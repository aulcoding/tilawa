//! Tilawa UI in Rust: Leptos for the parts that change a lot, web-sys for the rest.
//!
//! Phase 4 of the Rust migration: everything that moves on the page is Rust, built with
//! `trunk` (see Trunk.toml). The page itself is Leptos components (views.rs); index.html
//! only has <head>. The only JavaScript left is js/boot.js, which applies the saved theme
//! before the first paint. [`boot`] (called by main.rs) fetches lang/*.jsonc and starts the page.
//!
//! | module        | what                                                        |
//! |---------------|-------------------------------------------------------------|
//! | `views`       | the whole page markup (Leptos)                              |
//! | `card`        | the 29 day cards (Leptos)                                   |
//! | `header`      | progress count, line, bar, two-tap reset (Leptos)           |
//! | `carousel`    | carousel, "don't skip" bubble, links to a day               |
//! | `page`        | EN/ID switch, all [data-i18n] text, swipe animation         |
//! | `i18n`        | lang/*.jsonc parser + catalog                               |
//! | `theme`       | light/dark switch                                           |
//! | `hero`        | tappable moon, breathe in/out, "I'll start tomorrow"        |
//! | `extras`      | accordion, copy link, Saweria button + QR                   |
//! | `pwa`         | registers sw.js (offline + installable)                     |
//! | `dom`         | small web-sys helpers                                       |
//! | `storage`     | saved progress                                              |

mod card;
mod carousel;
mod dom;
mod extras;
mod header;
mod hero;
pub mod i18n;
mod page;
mod pwa;
mod storage;
mod texts;
mod theme;
mod views;

use leptos::prelude::*;
use tilawa_core::{build_days, Outcome, Progress};
use wasm_bindgen::{JsCast, prelude::*};

use dom::Timer;

pub use texts::Texts;

/// How long "Undo from the end: Day N first." stays in the header (ms)
const HINT_MS: u64 = 2800;
/// How long the reset button waits for the second tap (ms)
const ARM_MS: u64 = 3000;

/// Everything the UI reacts to. Signals are `Copy`, so this is passed around by value.
#[derive(Clone, Copy)]
pub struct State {
    /// Pages read per day + the rules (from tilawa-core)
    pub progress: RwSignal<Progress>,
    /// Current language; every text in views.rs follows it
    pub lang: RwSignal<i18n::Lang>,
    /// All text, both languages (lang/*.jsonc)
    pub catalog: StoredValue<i18n::Catalog>,
    /// The keys the Rust code reads, for the current language
    pub texts: RwSignal<Texts>,
    /// "Invite a friend": Some(copied?) for a moment after a tap
    pub shared: RwSignal<Option<bool>>,
    /// The encouraging line shown on each unfinished card: (day, line)
    pub cheer: RwSignal<Vec<(u8, String)>>,
    /// Bumped on every tap so the status line can replay its little rise animation
    pub tap_count: RwSignal<u32>,
    /// The header line; changes only when a day is (un)finished or the language changes
    pub top_line: RwSignal<String>,
    /// Finished days when `top_line` was last rolled
    pub top_for: StoredValue<u8>,
    /// A short-lived header message that wins over `top_line`
    pub hint: RwSignal<Option<String>>,
    /// Reset button waiting for its second tap
    pub armed: RwSignal<bool>,
    hint_timer: Timer,
    arm_timer: Timer,
    /// Hides the "don't skip" bubble
    warn_timer: Timer,
    /// Puts the "Invite a friend" label back
    pub share_timer: Timer,
}

/// A random line from `lines`, never `avoid` (unless it is the only one).
fn pick(lines: &[String], avoid: Option<&str>) -> String {
    if lines.is_empty() {
        return String::new();
    }
    loop {
        let i = ((js_sys::Math::random() * lines.len() as f64) as usize).min(lines.len() - 1);
        if lines.len() == 1 || Some(lines[i].as_str()) != avoid {
            return lines[i].clone();
        }
    }
}

impl State {
    /// The text for `key` in the current language, as a closure Leptos can track.
    pub fn t(self, key: &'static str) -> impl Fn() -> String + Copy + Send + Sync + 'static {
        move || {
            let lang = self.lang.get();
            self.catalog.with_value(|c| c.text(lang, key).unwrap_or_default().to_string())
        }
    }

    /// Applies a tap (`step` = +1 or −1) on `day`, saves, updates the lines and the carousel.
    pub fn tap(self, day: u8, step: i8) {
        let mut outcome = Outcome::Noop;
        self.progress.update(|p| {
            outcome = if step > 0 { p.add_page(day) } else { p.remove_page(day) };
        });

        match outcome {
            Outcome::Added | Outcome::Removed => {
                let line = self.pick_cheer(day);
                self.cheer.update(|c| {
                    c.retain(|(d, _)| *d != day);
                    c.push((day, line));
                });
            }
            Outcome::Finished => self.cheer.update(|c| c.retain(|(d, _)| *d != day)),
            Outcome::UndoFromEnd { last } => {
                let text = self.texts.with_untracked(|t| t.undo_last.replace("{n}", &last.to_string()));
                self.show_hint(text);
            }
            _ => {}
        }
        if matches!(outcome, Outcome::Added | Outcome::Removed | Outcome::Finished) {
            self.tap_count.update(|n| *n += 1);
            self.progress.with_untracked(storage::save);
            self.hint.set(None);
            let finished = self.progress.with_untracked(|p| p.finished_count());
            if finished != self.top_for.get_value() || self.top_line.with_untracked(String::is_empty) {
                self.roll_top();
            }
        }
        carousel::after_tap(self, outcome, day, self.warn_timer);
    }

    /// A cheer line for `day` (`{left}` = pages left), never the one already on the card.
    fn pick_cheer(self, day: u8) -> String {
        let left = self.progress.with_untracked(|p| {
            p.days()[day as usize - 1].total() - p.pages_on(day)
        });
        let current = self.cheer.with_untracked(|c| {
            c.iter().find(|(d, _)| *d == day).map(|(_, l)| l.clone())
        });
        self.texts.with_untracked(|t| {
            let lines: Vec<String> = t.cheer.iter().map(|l| l.replace("{left}", &left.to_string())).collect();
            pick(&lines, current.as_deref())
        })
    }

    /// New header line: warm while on Day 1, lightly sarcastic from Day 2 on,
    /// nothing before the first page or after the last day.
    fn roll_top(self) {
        let (n, first) = self.progress.with_untracked(|p| (p.finished_count(), p.pages_on(1)));
        self.top_for.set_value(n);
        if n >= 29 || (n == 0 && first == 0) {
            self.top_line.set(String::new());
            return;
        }
        let line = self.top_line.with_untracked(|cur| {
            self.texts.with_untracked(|t| {
                pick(if n == 0 { &t.progress_day1 } else { &t.progress }, Some(cur))
            })
        });
        self.top_line.set(line);
    }

    fn show_hint(self, text: String) {
        self.hint.set(Some(text));
        self.hint_timer.start(HINT_MS, move || self.hint.set(None));
    }

    /// First tap arms the button, the second (within 3 s) clears all 29 days.
    pub fn press_reset(self) {
        if !self.armed.get_untracked() {
            self.armed.set(true);
            self.arm_timer.start(ARM_MS, move || self.armed.set(false));
            return;
        }
        self.arm_timer.cancel();
        self.armed.set(false);
        self.hint.set(None);
        self.progress.update(|p| p.reset());
        self.cheer.set(Vec::new());
        self.progress.with_untracked(storage::save);
        self.roll_top();
        carousel::go_to(self, 1, true);
    }
}

/// Fetches the page text, then starts the page.
pub fn boot() {
    // (wasm-bindgen-futures, not leptos::task: the Leptos executor only starts with the first mount)
    wasm_bindgen_futures::spawn_local(async {
        // Both requests start right away; then wait for each
        let (en, id) = (fetch_text("lang/en.jsonc"), fetch_text("lang/id.jsonc"));
        let (en, id) = (en.await, id.await);
        start(&en, &id);
    });
}

/// The body of `url` as text, or "" if it can't be loaded (the catalog then logs why).
async fn fetch_text(url: &str) -> String {
    let request = dom::window().fetch_with_str(url);
    let load = async {
        let response: web_sys::Response = wasm_bindgen_futures::JsFuture::from(request).await?.dyn_into()?;
        if !response.ok() {
            return Err(JsValue::from_str(&format!("{url}: HTTP {}", response.status())));
        }
        wasm_bindgen_futures::JsFuture::from(response.text()?).await
    };
    match load.await {
        Ok(text) => text.as_string().unwrap_or_default(),
        Err(e) => {
            web_sys::console::error_2(&JsValue::from_str("Tilawa: couldn't load"), &e);
            String::new()
        }
    }
}

/// Starts the page. `en` / `id` are the contents of lang/en.jsonc and lang/id.jsonc.
pub fn start(en: &str, id: &str) {
    let catalog = i18n::Catalog::parse(en, id);
    for e in &catalog.errors {
        web_sys::console::error_1(&JsValue::from_str(&format!("Tilawa text: {e}")));
    }
    let lang = page::starting_lang();

    let (saved_pages, old_done) = storage::load();
    let progress = Progress::restore(build_days(), saved_pages.as_deref(), old_done.as_deref());

    let state = State {
        progress: RwSignal::new(progress),
        lang: RwSignal::new(lang),
        texts: RwSignal::new(catalog.texts(lang)),
        catalog: StoredValue::new(catalog),
        shared: RwSignal::new(None),
        cheer: RwSignal::new(Vec::new()),
        tap_count: RwSignal::new(0),
        top_line: RwSignal::new(String::new()),
        top_for: StoredValue::new(0),
        hint: RwSignal::new(None),
        armed: RwSignal::new(false),
        hint_timer: Timer::new(),
        arm_timer: Timer::new(),
        warn_timer: Timer::new(),
        share_timer: Timer::new(),
    };

    // The whole page; it lives as long as the tab
    leptos::mount::mount_to_body(move || view! { <views::App state=state/> });

    page::Page::new(state).start();
    pwa::register();
}
