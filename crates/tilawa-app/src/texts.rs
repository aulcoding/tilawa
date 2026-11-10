//! The card text for the current language, sent from lang/*.js as JSON.

use std::collections::HashMap;

use serde::Deserialize;

/// Only the keys the cards use. Missing keys fall back to empty / defaults.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Texts {
    pub day: String,
    pub surah: String,
    pub surahs: String,
    pub pages_unit: String,
    pub pages_short: String,
    pub page_plus: String,
    pub page_minus: String,
    pub done: String,
    pub done_days: Vec<String>,
    pub cheer: Vec<String>,
    pub tags: HashMap<String, String>,
}

impl Texts {
    pub fn from_json(json: &str) -> Texts {
        serde_json::from_str(json).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_keys_the_cards_need() {
        let t = Texts::from_json(r#"{"day":"Hari","pagesUnit":"hal","doneDays":["a","b"],"tags":{"boss":"Big match"},"unused":1}"#);
        assert_eq!(t.day, "Hari");
        assert_eq!(t.pages_unit, "hal");
        assert_eq!(t.done_days.len(), 2);
        assert_eq!(t.tags["boss"], "Big match");
    }

    #[test]
    fn bad_json_gives_empty_texts() {
        assert!(Texts::from_json("nope").day.is_empty());
    }
}
