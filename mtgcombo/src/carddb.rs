use crate::io::fetch_url;

pub struct CardDb {}
impl CardDb {
    pub fn load() {
        let json_url = "https://mtgjson.com/api/v5/AllPrintings.json.zip";
        let _ = fetch_url(json_url);

        let price_url = "https://www.mtgjson.com/api/v5/AllPrices.json.zip";
        let _ = fetch_url(price_url);
    }
}
