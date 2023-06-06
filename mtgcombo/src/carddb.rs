use std::{collections::HashMap, io::Cursor};

use serde::{Deserialize, Serialize};

use crate::io::fetch_url;

pub struct CardDb {}
impl CardDb {
    pub fn all_printings() -> String {
        let json_url = "https://mtgjson.com/api/v5/AllPrintings.json.gz";
        fetch_url(json_url)
    }

    pub fn all_prices() -> String {
        let json_url = "https://mtgjson.com/api/v5/AllPrices.json.gz";
        fetch_url(json_url)
    }

    pub fn load() {
        Self::all_printings();
        Self::all_prices();
    }

    pub fn get_price(card_name: &str) -> Option<f64> {
        let prices = Self::all_prices();
        let prices = serde_json::from_str::<serde_json::Value>(&prices).unwrap();

        for key in prices.as_object().unwrap()["data"]
            .as_object()
            .unwrap()
            .keys()
        {
            println!("{}", key);
        }

        Some(0.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Price {}
