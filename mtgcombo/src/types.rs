use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllPrintings {
    pub data: HashMap<String, Set>,
}
impl AllPrintings {
    pub fn get_card(&self, card_name: &str) -> Option<&Card> {
        for set in self.data.values() {
            for card in set.cards.iter() {
                if card.name == card_name {
                    return Some(card);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Set {
    pub code: String,
    #[serde(rename = "isForeignOnly", default)]
    pub foreign_only: bool,

    #[serde(rename = "isFoilOnly")]
    pub foil_only: bool,
    pub name: String,

    #[serde(rename = "codeV3")]
    pub code_v3: Option<String>,
    pub cards: Vec<Card>,
    #[serde(rename = "isOnlineOnly", default)]
    pub online_only: bool,
    #[serde(rename = "isPaperOnly", default)]
    pub paper_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "artist")]
    pub artist: Option<String>,
    #[serde(rename = "asciiName")]
    pub ascii_name: Option<String>,
    #[serde(rename = "attractionLights")]
    pub attraction_lights: Option<Vec<serde_json::Value>>,
    ////
    #[serde(rename = "availability")]
    pub availability: Vec<String>,
    #[serde(rename = "boosterTypes")]
    pub booster_types: Option<Vec<String>>,
    #[serde(rename = "borderColor")]
    pub border_color: String,
    #[serde(rename = "cardParts")]
    pub card_parts: Option<Vec<String>>,
    #[serde(rename = "colorIdentity")]
    pub color_identity: Vec<String>,
    #[serde(rename = "colorIndicator")]
    pub color_indicator: Option<Vec<String>>,
    #[serde(rename = "colors")]
    pub colors: Vec<String>,
    #[serde(rename = "convertedManaCost")]
    pub converted_mana_cost: f64,
    #[serde(rename = "defense")]
    pub defense: Option<String>,
    #[serde(rename = "edhrecRank")]
    pub edhrec_rank: Option<f64>,
    #[serde(rename = "edhrecSaltiness")]
    pub edhrec_saltiness: Option<f64>,
    #[serde(rename = "faceConvertedManaCost")]
    pub face_converted_mana_cost: Option<f64>,
    #[serde(rename = "faceFlavorName")]
    pub face_flavor_name: Option<String>,
    #[serde(rename = "faceManaValue")]
    pub face_mana_value: Option<f64>,
    #[serde(rename = "faceName")]
    pub face_name: Option<String>,
    #[serde(rename = "finishes")]
    pub finishes: Vec<String>,
    #[serde(rename = "flavorName")]
    pub flavor_name: Option<String>,
    #[serde(rename = "flavorText")]
    pub flavor_text: Option<String>,
    #[serde(rename = "foreignData")]
    pub foreign_data: Option<Vec<ForeignData>>,
    #[serde(rename = "frameEffects")]
    pub frame_effects: Option<Vec<String>>,
    #[serde(rename = "frameVersion")]
    pub frame_version: String,
    #[serde(rename = "hand")]
    pub hand: Option<String>,
    #[serde(rename = "hasAlternativeDeckLimit")]
    pub has_alternative_deck_limit: Option<bool>,
    #[serde(rename = "hasContentWarning")]
    pub has_content_warning: Option<bool>,
    #[serde(rename = "hasFoil")]
    pub has_foil: bool,
    #[serde(rename = "hasNonFoil")]
    pub has_non_foil: bool,
    #[serde(rename = "identifiers")]
    pub identifiers: Identifiers,
    #[serde(rename = "isAlternative")]
    pub is_alternative: Option<bool>,
    #[serde(rename = "isFullArt")]
    pub is_full_art: Option<bool>,
    #[serde(rename = "isFunny")]
    pub is_funny: Option<bool>,
    #[serde(rename = "isOnlineOnly")]
    pub is_online_only: Option<bool>,
    #[serde(rename = "isOversized")]
    pub is_oversized: Option<bool>,
    #[serde(rename = "isPromo")]
    pub is_promo: Option<bool>,
    #[serde(rename = "isRebalanced")]
    pub is_rebalanced: Option<bool>,
    #[serde(rename = "isReprint")]
    pub is_reprint: Option<bool>,
    #[serde(rename = "isReserved")]
    pub is_reserved: Option<bool>,
    #[serde(rename = "isStarter")]
    pub is_starter: Option<bool>,
    #[serde(rename = "isStorySpotlight")]
    pub is_story_spotlight: Option<bool>,
    #[serde(rename = "isTextless")]
    pub is_textless: Option<bool>,
    #[serde(rename = "isTimeshifted")]
    pub is_timeshifted: Option<bool>,
    #[serde(rename = "keywords")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "layout")]
    pub layout: String,
    #[serde(rename = "leadershipSkills")]
    pub leadership_skills: Option<LeadershipSkills>,
    #[serde(rename = "legalities")]
    pub legalities: Legalities,
    #[serde(rename = "life")]
    pub life: Option<String>,
    #[serde(rename = "loyalty")]
    pub loyalty: Option<String>,
    #[serde(rename = "manaCost")]
    pub mana_cost: Option<String>,
    #[serde(rename = "manaValue")]
    pub mana_value: f64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "originalPrintings")]
    pub original_printings: Option<Vec<String>>,
    #[serde(rename = "originalReleaseDate")]
    pub original_release_date: Option<String>,
    #[serde(rename = "originalText")]
    pub original_text: Option<String>,
    #[serde(rename = "originalType")]
    pub original_type: Option<String>,
    #[serde(rename = "otherFaceIds")]
    pub other_face_ids: Option<Vec<String>>,
    #[serde(rename = "power")]
    pub power: Option<String>,
    #[serde(rename = "printings")]
    pub printings: Option<Vec<String>>,
    #[serde(rename = "promoTypes")]
    pub promo_types: Option<Vec<String>>,
    #[serde(rename = "purchaseUrls")]
    pub purchase_urls: PurchaseUrls,
    #[serde(rename = "rarity")]
    pub rarity: String,
    #[serde(rename = "relatedCards")]
    pub related_cards: Option<RelatedCards>,
    #[serde(rename = "rebalancedPrintings")]
    pub rebalanced_printings: Option<Vec<String>>,
    #[serde(rename = "rulings", default)]
    pub rulings: Vec<Rulings>,
    #[serde(rename = "securityStamp")]
    pub security_stamp: Option<String>,
    #[serde(rename = "setCode")]
    pub set_code: String,
    #[serde(rename = "side")]
    pub side: Option<String>,
    #[serde(rename = "signature")]
    pub signature: Option<String>,
    #[serde(rename = "subsets")]
    pub subsets: Option<Vec<String>>,
    #[serde(rename = "subtypes")]
    pub subtypes: Vec<String>,
    #[serde(rename = "supertypes")]
    pub supertypes: Vec<String>,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "toughness")]
    pub toughness: Option<String>,
    #[serde(rename = "type")]
    pub card_type: String,
    #[serde(rename = "types")]
    pub types: Vec<String>,
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "variations")]
    pub variations: Option<Vec<String>>,
    #[serde(rename = "watermark")]
    pub watermark: Option<String>,
}
impl Card {
    pub fn is_commander_legal(&self) -> bool {
        self.legalities.is_commander_legal()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedCards {
    #[serde(rename = "reverseRelated")]
    pub reverse_related: Option<Vec<String>>,
    #[serde(rename = "spellbook")]
    pub spellbook: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rulings {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseUrls {
    #[serde(rename = "cardKingdom")]
    pub card_kingdom: Option<String>,
    #[serde(rename = "cardKingdomEtched")]
    pub card_kingdom_etched: Option<String>,
    #[serde(rename = "cardKingdomFoil")]
    pub card_kingdom_foil: Option<String>,
    #[serde(rename = "cardmarket")]
    pub cardmarket: Option<String>,
    #[serde(rename = "tcgplayer")]
    pub tcgplayer: Option<String>,
    #[serde(rename = "tcgplayerEtched")]
    pub tcgplayer_etched: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legalities {
    #[serde(rename = "alchemy")]
    pub alchemy: Option<String>,
    #[serde(rename = "brawl")]
    pub brawl: Option<String>,
    #[serde(rename = "commander")]
    pub commander: Option<String>,
    #[serde(rename = "duel")]
    pub duel: Option<String>,
    #[serde(rename = "explorer")]
    pub explorer: Option<String>,
    #[serde(rename = "future")]
    pub future: Option<String>,
    #[serde(rename = "gladiator")]
    pub gladiator: Option<String>,
    #[serde(rename = "historic")]
    pub historic: Option<String>,
    #[serde(rename = "historicbrawl")]
    pub historicbrawl: Option<String>,
    #[serde(rename = "legacy")]
    pub legacy: Option<String>,
    #[serde(rename = "modern")]
    pub modern: Option<String>,
    #[serde(rename = "oldschool")]
    pub oldschool: Option<String>,
    #[serde(rename = "pauper")]
    pub pauper: Option<String>,
    #[serde(rename = "penny")]
    pub penny: Option<String>,
    #[serde(rename = "pioneer")]
    pub pioneer: Option<String>,
    #[serde(rename = "predh")]
    pub predh: Option<String>,
    #[serde(rename = "premodern")]
    pub premodern: Option<String>,
    #[serde(rename = "standard")]
    pub standard: Option<String>,
    #[serde(rename = "vintage")]
    pub vintage: Option<String>,
}
impl Legalities {
    pub fn is_commander_legal(&self) -> bool {
        self.commander == Some("Legal".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignData {
    #[serde(rename = "faceName")]
    pub face_name: Option<String>,
    #[serde(rename = "flavorText")]
    pub flavor_text: Option<String>,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "multiverseId")]
    pub multiverse_id: Option<f64>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeadershipSkills {
    #[serde(rename = "brawl")]
    pub brawl: bool,
    #[serde(rename = "commander")]
    pub commander: bool,
    #[serde(rename = "oathbreaker")]
    pub oathbreaker: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(rename = "cardKingdomEtchedId")]
    pub card_kingdom_etched_id: Option<String>,
    #[serde(rename = "cardKingdomFoilId")]
    pub card_kingdom_foil_id: Option<String>,
    #[serde(rename = "cardKingdomId")]
    pub card_kingdom_id: Option<String>,
    #[serde(rename = "cardsphereId")]
    pub cardsphere_id: Option<String>,
    #[serde(rename = "mcmId")]
    pub mcm_id: Option<String>,
    #[serde(rename = "mcmMetaId")]
    pub mcm_meta_id: Option<String>,
    #[serde(rename = "mtgArenaId")]
    pub mtg_arena_id: Option<String>,
    #[serde(rename = "mtgjsonFoilVersionId")]
    pub mtgjson_foil_version_id: Option<String>,
    #[serde(rename = "mtgjsonNonFoilVersionId")]
    pub mtgjson_non_foil_version_id: Option<String>,
    #[serde(rename = "mtgjsonV4Id")]
    pub mtgjson_v4_id: Option<String>,
    #[serde(rename = "mtgoFoilId")]
    pub mtgo_foil_id: Option<String>,
    #[serde(rename = "mtgoId")]
    pub mtgo_id: Option<String>,
    #[serde(rename = "multiverseId")]
    pub multiverse_id: Option<String>,
    #[serde(rename = "scryfallId")]
    pub scryfall_id: Option<String>,
    #[serde(rename = "scryfallOracleId")]
    pub scryfall_oracle_id: Option<String>,
    #[serde(rename = "scryfallIllustrationId")]
    pub scryfall_illustration_id: Option<String>,
    #[serde(rename = "tcgplayerProductId")]
    pub tcgplayer_product_id: Option<String>,
    #[serde(rename = "tcgplayerEtchedProductId")]
    pub tcgplayer_etched_product_id: Option<String>,
}
