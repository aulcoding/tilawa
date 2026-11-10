//! Tilawa UI in Leptos.
//!
//! Phase 2 of the Rust migration: the 29 schedule cards are a Leptos component.
//! The page's JavaScript still owns everything around them (progress header,
//! carousel arrows, "don't skip" bubble). It creates a [`ScheduleApp`] and gets
//! told what happened after every tap through a callback.
//!
//! ```js
//! import init, { ScheduleApp } from "./pkg/tilawa_app.js";
//! await init();
//! const app = new ScheduleApp(gridElement, JSON.stringify(texts), (json) => {
//!   const ev = JSON.parse(json);   // {"kind":"finished","day":4,"finished":4,"firstDayPages":49}
//! });
//! ```

mod card;
mod storage;
mod texts;

use leptos::prelude::*;
use serde_json::json;
use tilawa_core::{build_days, Outcome, Progress};
use wasm_bindgen::prelude::*;

pub use texts::Texts;

/// Everything the cards react to. Signals are `Copy`, so this is passed around by value.
#[derive(Clone, Copy)]
pub struct State {
    /// Pages read per day + the rules (from tilawa-core)
    pub progress: RwSignal<Progress>,
    /// Text for the current language (sent from lang/*.js)
    pub texts: RwSignal<Texts>,
    /// The encouraging line shown on each unfinished card: (day, line)
    pub cheer: RwSignal<Vec<(u8, String)>>,
    /// Bumped on every tap so the status line can replay its little rise animation
    pub tap_count: RwSignal<u32>,
    /// Tells the page's JavaScript what happened
    pub notify: StoredValue<js_sys::Function, LocalStorage>,
}

impl State {
    /// Applies a tap (`step` = +1 or −1) on `day`, saves, picks a new cheer line, tells JS.
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
            _ => {}
        }
        if matches!(outcome, Outcome::Added | Outcome::Removed | Outcome::Finished) {
            self.tap_count.update(|n| *n += 1);
            self.progress.with_untracked(storage::save);
        }
        self.emit(outcome, day);
    }

    /// A random line from `cheer`, never the same as the one already on the card.
    /// `{left}` becomes the pages left that day.
    fn pick_cheer(self, day: u8) -> String {
        let left = self.progress.with_untracked(|p| {
            p.days()[day as usize - 1].total() - p.pages_on(day)
        });
        let current = self.cheer.with_untracked(|c| {
            c.iter().find(|(d, _)| *d == day).map(|(_, l)| l.clone())
        });
        self.texts.with_untracked(|t| {
            let lines: Vec<String> = t.cheer.iter().map(|l| l.replace("{left}", &left.to_string())).collect();
            if lines.is_empty() {
                return String::new();
            }
            loop {
                let i = (js_sys::Math::random() * lines.len() as f64) as usize;
                let pick = &lines[i.min(lines.len() - 1)];
                if lines.len() == 1 || Some(pick) != current.as_ref() {
                    return pick.clone();
                }
            }
        })
    }

    /// Sends {"kind": …, "day": …, "finished": …, "firstDayPages": …, plus next/last} to JS.
    fn emit(self, outcome: Outcome, day: u8) {
        let (finished, first) = self.progress.with_untracked(|p| (p.finished_count(), p.pages_on(1)));
        let mut event = serde_json::to_value(outcome).expect("outcome serialises");
        event["day"] = json!(day);
        event["finished"] = json!(finished);
        event["firstDayPages"] = json!(first);
        let text = JsValue::from_str(&event.to_string());
        self.notify.with_value(|f| {
            let _ = f.call1(&JsValue::NULL, &text);
        });
    }
}

/// The handle the page's JavaScript holds.
#[wasm_bindgen]
pub struct ScheduleApp {
    state: State,
}

#[wasm_bindgen]
impl ScheduleApp {
    /// Renders the 29 cards into `grid`, restoring saved progress from localStorage.
    #[wasm_bindgen(constructor)]
    pub fn new(grid: web_sys::HtmlElement, texts_json: &str, on_event: js_sys::Function) -> ScheduleApp {
        let (saved_pages, old_done) = storage::load();
        let progress = Progress::restore(build_days(), saved_pages.as_deref(), old_done.as_deref());

        let state = State {
            progress: RwSignal::new(progress),
            texts: RwSignal::new(Texts::from_json(texts_json)),
            cheer: RwSignal::new(Vec::new()),
            tap_count: RwSignal::new(0),
            notify: StoredValue::new_local(on_event),
        };

        let days = build_days();
        let handle = leptos::mount::mount_to(grid, move || {
            days.into_iter()
                .enumerate()
                .map(|(i, day)| view! { <card::DayCard n={i as u8 + 1} day=day state=state/> })
                .collect_view()
        });
        handle.forget(); // the cards live as long as the page

        ScheduleApp { state }
    }

    /// New language: every card label updates in place (no re-render, so moons keep their state).
    pub fn set_texts(&self, texts_json: &str) {
        self.state.texts.set(Texts::from_json(texts_json));
    }

    pub fn reset(&self) {
        self.state.progress.update(|p| p.reset());
        self.state.cheer.set(Vec::new());
        self.state.progress.with_untracked(storage::save);
    }

    pub fn finished_count(&self) -> u8 {
        self.state.progress.with_untracked(|p| p.finished_count())
    }

    pub fn pages_on(&self, day: u8) -> u16 {
        self.state.progress.with_untracked(|p| p.pages_on(day))
    }

    /// True when no page is recorded on any day (the reset button is disabled then).
    pub fn is_empty(&self) -> bool {
        self.state.progress.with_untracked(|p| p.pages().iter().all(|&v| v == 0))
    }
}
