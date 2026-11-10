//! localStorage, through web-sys. Never fails: private windows or blocked storage just mean nothing is kept.

use tilawa_core::Progress;

/// Pages read per day, e.g. [49, 5, 0, ...]
const PAGES_KEY: &str = "tilawa-pages-v1";
/// Older format (finished day numbers), still written so older versions can read it
const DONE_KEY: &str = "tilawa-progress-v1";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

/// (saved pages, old finished-days list)
pub fn load() -> (Option<String>, Option<String>) {
    match storage() {
        Some(s) => (s.get_item(PAGES_KEY).ok().flatten(), s.get_item(DONE_KEY).ok().flatten()),
        None => (None, None),
    }
}

pub fn save(progress: &Progress) {
    if let Some(s) = storage() {
        let _ = s.set_item(PAGES_KEY, &progress.to_json());
        let _ = s.set_item(DONE_KEY, &progress.finished_days_json());
    }
}
