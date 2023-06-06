use easy_repl::{command, CommandStatus, Repl, anyhow::{self, Context}};
use std::collections::HashMap;
use types::Card;

mod carddb;
mod combo;
mod db;
mod io;
mod types;

fn matryoshka(name: String) -> anyhow::Result<Repl<'static>> {
    let prompt = format!("{}> ", name);

    let cloned_prompt = prompt.clone(); // need to move it into closure
    let new = command! {
        "Enter new repl",
        (name:String) => |name: String| {
            let name = cloned_prompt.clone() + &name;
            let mut repl = matryoshka(name)?;
            repl.run()?;
            Ok(CommandStatus::Done)
        }
    };

    let repl = Repl::builder().prompt(prompt).add("new", new).build()?;

    Ok(repl)
}

fn adder(num: i32) -> anyhow::Result<Repl<'static>> {
    let prompt = format!("> {:?} + ", num);

    let add = command! {
        "Add X to Y",
        (X:i32, Y:i32) => |x, y| {
            println!("{} + {} = {}", x, y, x + y);

            
            Ok(CommandStatus::Quit)
        }
    };

    let repl = Repl::builder().prompt(prompt).add("add", add).build()?;

    Ok(repl)
}

fn main() -> anyhow::Result<()> {
    io::init_dirs();
    let mut db = db::Db::new();

    let mut map: HashMap<String, Vec<Card>> = std::collections::HashMap::new();

    for (set_code, set) in db.printings().data.iter() {
        for card in set.cards.iter() {
            if let Some(face_name) = &card.face_name {
                match map.get_mut(face_name) {
                    Some(cards) => cards.push(card.clone()),
                    None => {
                        map.insert(face_name.clone(), vec![card.clone()]);
                    }
                }
            }

            match map.get_mut(&card.name) {
                Some(cards) => cards.push(card.clone()),
                None => {
                    map.insert(card.name.clone(), vec![card.clone()]);
                }
            }
        }
    }

    let mut builder = Repl::builder();        
    let mut repl = builder 
        .add(
            "hello",
            command! {
                "Say hello",
                (name: String) => |name| {
                    println!("Hello {}!", name);
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "add",
            command! {
                "Add X to Y",
                (X:i32, Y:i32) => |x, y| {
                    println!("{} + {} = {}", x, y, x + y);
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "search",
            command! {
                "Search for combos for a given mtg card",
                (name: String,MaxCards: usize, MaxCMC: usize) => |name: String, max_cards: usize,  max_cmc: usize| {                    
                    let combos = fetch_combo(&map, name.as_str(), max_cards, max_cmc as f64, Some(vec!["W".to_string(), "U".to_string(), "B".to_string(), "R".to_string(), "G".to_string(), "C".to_string()]));
                    println!("------------------------------");
                    println!("------------------------------");
                    println!("------------------------------");
                    for combo in combos.combos.iter() {
                        println!("----");
                        for card in combo.cards.iter() {
                            println!("{} ({}): {}", card.name, card.converted_mana_cost, card.clone().text.unwrap().replace("\n", " "));
                        }
                    }
                    println!("------------------------------");

                    for card in combos.top_cards().iter() {
                        println!("{}: {}", card.0, card.1);
                    }
                    println!("------------------------------");
                    println!("------------------------------");

                    Ok(CommandStatus::Done)
                }
            },
        )
        .build()
        .expect("Failed to create repl");

    repl.run().expect("Critical REPL error");

    Ok(())
}

#[derive(Debug)]
pub struct ComboResult {
    pub key_card: Card,
    pub combos: Vec<Combo>,
    pub card_counts: HashMap<String, usize>,
    pub color_ratios: HashMap<String, usize>,
}
impl ComboResult {
    /// Returns the most used colors in the combos
    pub fn top_colors(&self) -> Vec<(String, usize)> {
        let mut colors: Vec<(String, usize)> = self
            .color_ratios
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));
        colors
    }

    /// Returns the most used cards in the combos
    pub fn top_cards(&self) -> Vec<(String, usize)> {
        let mut cards: Vec<(String, usize)> = self
            .card_counts
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        cards.sort_by(|a, b| b.1.cmp(&a.1));
        cards.remove(0);
        cards
    }
}

#[derive(Debug, Clone)]
pub struct Combo {
    pub identity: Vec<String>,
    pub total_mana_cost: f64,
    pub cards: Vec<Card>,
    pub url: String,
}

fn fetch_combo(
    map: &HashMap<String, Vec<Card>>,
    name: &str,
    max_cards_per_combo: usize,
    max_cmc: f64,
    valid_identities: Option<Vec<String>>,
) -> ComboResult {
    let resp = combo::fetch(name);
    let mut card_counts = HashMap::new();
    let mut color_ratios = HashMap::new();

    let valid_identities = match valid_identities {
        Some(identities) => identities,
        None => vec!["W", "U", "B", "R", "G", "C"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    };
    let mut combos = vec![];

    for (url, cards) in resp.iter() {
        if cards.len() > max_cards_per_combo {
            continue;
        }

        let mut combo = Combo {
            identity: vec![],
            total_mana_cost: 0.0,
            cards: vec![],
            url: url.clone(),
        };
        let mut valid_combo = true;
        let mut combo_counts: HashMap<String, usize> = HashMap::new();
        let mut combo_colors: HashMap<String, usize> = HashMap::new();

        for card in cards.iter() {
            let card = map.get(&card.name).unwrap();
            if card.len() > 1 {
                let card = &card[0];
                if card.converted_mana_cost > max_cmc {
                    valid_combo = false;
                }
                if !card
                    .color_identity
                    .iter()
                    .all(|c| valid_identities.contains(c))
                {
                    valid_combo = false;
                }

                combo.cards.push(card.clone());
                combo.total_mana_cost += card.converted_mana_cost;
                for identity in card.color_identity.iter() {
                    combo.identity.push(identity.clone());
                }

                if !card.is_commander_legal() {
                    valid_combo = false;
                }

                if card.text.is_none() {
                    valid_combo = false;
                }

                match combo_counts.get_mut(&card.name) {
                    Some(count) => *count += 1,
                    None => {
                        combo_counts.insert(card.name.clone(), 1);
                    }
                }

                for identity in card.color_identity.iter() {
                    match combo_colors.get_mut(identity) {
                        Some(count) => *count += 1,
                        None => {
                            combo_colors.insert(identity.clone(), 1);
                        }
                    }
                }
            }
        }

        if valid_combo {
            for (key, value) in combo_counts.iter() {
                match card_counts.get_mut(key) {
                    Some(count) => *count += *value,
                    None => {
                        card_counts.insert(key.clone(), *value);
                    }
                }
            }
            for (key, value) in combo_colors.iter() {
                match color_ratios.get_mut(key) {
                    Some(count) => *count += *value,
                    None => {
                        color_ratios.insert(key.clone(), *value);
                    }
                }
            }
            combo.identity.dedup();
            combo.cards.sort_by(|a, b| a.name.cmp(&b.name));
            combos.push(combo);
        }
    }

    ComboResult {
        key_card: map.get(name).unwrap()[0].clone(),
        combos,
        card_counts,
        color_ratios,
    }
}
