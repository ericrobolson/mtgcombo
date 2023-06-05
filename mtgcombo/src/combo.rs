use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::io;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct ComboResponse {
    pub cardlist: Vec<CardCombo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct CardCombo {
    pub name: String,
    pub sanitized: String,
    pub url: String,
}

pub fn fetch(card_name: &str) {
    let url = "https://json.edhrec.com/pages/combos/phyrexian-altar.json";

    let response = io::fetch_url(url);

    let json = serde_json::from_str::<ComboResponse>(&response).unwrap();

    let mut combolist: HashMap<String, Vec<CardCombo>> = HashMap::new();
    for card in json.cardlist.iter() {
        let url = card.url.clone();
        match combolist.get_mut(&url) {
            Some(cards) => {
                cards.push(card.clone());
                cards.dedup();
            }
            None => {
                let cards = vec![card.clone()];
                combolist.insert(url, cards);
            }
        }
    }
}
