use crate::io;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComboResponse {
    pub cardlist: Vec<CardCombo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CardCombo {
    pub name: String,
    pub sanitized: String,
    pub url: String,
}

fn sanitize_card_name(card_name: &str) -> String {
    card_name
        .replace(" ", "-")
        .replace("'", "")
        .replace(",", "")
        .replace(",", "")
        .to_lowercase()
}

pub fn fetch(card_name: &str) -> HashMap<String, Vec<CardCombo>> {
    let url = format!(
        "https://json.edhrec.com/pages/combos/{}.json",
        sanitize_card_name(card_name)
    );

    let response = io::fetch_url(&url);

    let json = serde_json::from_str::<ComboResponse>(&response).unwrap();
    let combo_card = CardCombo {
        name: card_name.to_string(),
        sanitized: sanitize_card_name(card_name),
        url: url.clone(),
    };

    let mut combolist: HashMap<String, Vec<CardCombo>> = HashMap::new();
    for card in json.cardlist.iter() {
        let url = card.url.clone();
        match combolist.get_mut(&url) {
            Some(cards) => {
                cards.push(card.clone());
                cards.dedup();
            }
            None => {
                let cards = vec![combo_card.clone(), card.clone()];
                combolist.insert(url, cards);
            }
        }
    }

    combolist
}
