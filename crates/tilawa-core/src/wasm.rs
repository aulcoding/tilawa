//! The bridge between Rust and the page's JavaScript.
//!
//! `wasm-bindgen` turns this `Tilawa` struct into a JavaScript class:
//!
//! ```js
//! import init, { Tilawa } from "./pkg/tilawa_core.js";
//! await init();
//! const core = Tilawa.restore(localStorage.getItem("tilawa-pages-v1"), null);
//! core.add_page(1); // → '{"kind":"added"}'
//! ```
//!
//! Lists and outcomes cross the bridge as JSON strings to keep it simple.

use wasm_bindgen::prelude::*;

use crate::{build_days, Progress};

#[wasm_bindgen]
pub struct Tilawa {
    progress: Progress,
}

#[wasm_bindgen]
impl Tilawa {
    /// Starts from saved data (either argument may be null).
    pub fn restore(saved_pages: Option<String>, old_done: Option<String>) -> Tilawa {
        Tilawa {
            progress: Progress::restore(build_days(), saved_pages.as_deref(), old_done.as_deref()),
        }
    }

    /// The 29 days as JSON: [{ s, r, tag?, pg }, ...]
    pub fn days_json(&self) -> String {
        serde_json::to_string(self.progress.days()).expect("days serialise")
    }

    /// Pages read per day (index 0 = Day 1). Arrives in JS as a Uint16Array.
    pub fn pages(&self) -> Vec<u16> {
        self.progress.pages().to_vec()
    }

    pub fn finished_count(&self) -> u8 {
        self.progress.finished_count()
    }

    pub fn can_add(&self, day: u8) -> bool {
        self.progress.can_add(day)
    }

    pub fn can_remove(&self, day: u8) -> bool {
        self.progress.can_remove(day)
    }

    /// Returns an outcome as JSON, e.g. {"kind":"finished"} or {"kind":"locked","next":1}.
    pub fn add_page(&mut self, day: u8) -> String {
        serde_json::to_string(&self.progress.add_page(day)).expect("outcome serialises")
    }

    /// Same shape as add_page, e.g. {"kind":"undofromend","last":2}.
    pub fn remove_page(&mut self, day: u8) -> String {
        serde_json::to_string(&self.progress.remove_page(day)).expect("outcome serialises")
    }

    pub fn reset(&mut self) {
        self.progress.reset();
    }

    /// For localStorage "tilawa-pages-v1".
    pub fn to_json(&self) -> String {
        self.progress.to_json()
    }

    /// For the older localStorage "tilawa-progress-v1".
    pub fn finished_days_json(&self) -> String {
        self.progress.finished_days_json()
    }
}
