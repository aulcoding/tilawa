//! The text Rust uses directly, for the current language (from lang/*.jsonc).

use std::collections::HashMap;

use serde::Deserialize;

/// The keys the Rust code reads. Missing keys fall back to empty / defaults.
/// (Plain `data-i18n` text in the HTML is filled straight from the catalog, see page.rs.)
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
    // Progress header
    pub msg: Vec<String>,
    pub progress_day1: Vec<String>,
    pub progress: Vec<String>,
    pub reset: String,
    pub reset_armed: String,
    pub undo_last: String,
    // Carousel + "don't skip" bubble
    pub sched_h: String,
    pub carousel_one: String,
    pub carousel_many: String,
    pub carousel_prev: String,
    pub carousel_next: String,
    pub skip_warn: String,
    // Hero, accordion, share
    pub moon_label: String,
    pub later: String,
    pub more_open: String,
    pub more_close: String,
    pub share_cta: String,
    pub share_done: String,
    pub share_fail: String,
    // Light/dark button
    pub theme_to_dark: String,
    pub theme_to_light: String,
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
    fn reads_the_header_keys() {
        let t = Texts::from_json(r#"{"progressDay1":["x"],"resetArmed":"sure?","undoLast":"Day {n} first"}"#);
        assert_eq!(t.progress_day1, ["x"]);
        assert_eq!(t.reset_armed, "sure?");
        assert_eq!(t.undo_last, "Day {n} first");
    }

    #[test]
    fn bad_json_gives_empty_texts() {
        assert!(Texts::from_json("nope").day.is_empty());
    }
}
