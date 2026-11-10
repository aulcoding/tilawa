//! Tilawa UI in Leptos.
//!
//! Phase 3 (step 1) of the Rust migration: the 29 schedule cards and the progress
//! header above them (count, line, bar, two-tap reset) are Leptos components.
//! The page's JavaScript still owns the carousel, the "don't skip" bubble and the
//! language/theme switches. It creates a [`ScheduleApp`] and is told what happened
//! after every tap through a callback.
//!
//! ```js
//! import init, { ScheduleApp } from "./pkg/tilawa_app.js";
//! await init();
//! const app = new ScheduleApp(gridEl, headEl, JSON.stringify(texts), (json) => {
//!   const ev = JSON.parse(json);   // {"kind":"finished","day":4,"finished":4,"firstDayPages":49}
//! });
//! ```

mod card;
mod header;
mod storage;
mod texts;

use std::time::Duration;

use leptos::prelude::*;
use serde_json::json;
use tilawa_core::{build_days, Outcome, Progress};
use wasm_bindgen::prelude::*;

pub use texts::Texts;

/// How long "Undo from the end: Day N first." stays in the header
const HINT_MS: u64 = 2800;
/// How long the reset button waits for the second tap
const ARM_MS: u64 = 3000;

/// Everything the UI reacts to. Signals are `Copy`, so this is passed around by value.
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
    /// The header line; changes only when a day is (un)finished or the language changes
    pub top_line: RwSignal<String>,
    /// Finished days when `top_line` was last rolled
    pub top_for: StoredValue<u8>,
    /// A short-lived header message that wins over `top_line`
    pub hint: RwSignal<Option<String>>,
    /// Reset button waiting for its second tap
    pub armed: RwSignal<bool>,
    /// Bumped on every new hint / arm, so an older timer knows it is stale
    pub timers: StoredValue<(u32, u32)>,
    /// Tells the page's JavaScript what happened
    pub notify: StoredValue<js_sys::Function, LocalStorage>,
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
    /// Applies a tap (`step` = +1 or −1) on `day`, saves, updates the lines, tells JS.
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
        self.emit(outcome, day);
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
        let id = self.bump_timer(true);
        set_timeout(move || {
            if self.timers.get_value().0 == id {
                self.hint.set(None);
            }
        }, Duration::from_millis(HINT_MS));
    }

    /// First tap arms the button, the second (within 3 s) clears all 29 days.
    pub fn press_reset(self) {
        if !self.armed.get_untracked() {
            self.armed.set(true);
            let id = self.bump_timer(false);
            set_timeout(move || {
                if self.timers.get_value().1 == id {
                    self.armed.set(false);
                }
            }, Duration::from_millis(ARM_MS));
            return;
        }
        self.armed.set(false);
        self.hint.set(None);
        self.progress.update(|p| p.reset());
        self.cheer.set(Vec::new());
        self.progress.with_untracked(storage::save);
        self.roll_top();
        self.send(json!({ "kind": "reset", "finished": 0, "firstDayPages": 0 }));
    }

    /// Bumps the hint (`true`) or arm timer id and returns the new id.
    fn bump_timer(self, hint: bool) -> u32 {
        let mut id = 0;
        self.timers.update_value(|(h, a)| {
            let t = if hint { h } else { a };
            *t += 1;
            id = *t;
        });
        id
    }

    /// Sends {"kind": …, "day": …, "finished": …, "firstDayPages": …, plus next/last} to JS.
    fn emit(self, outcome: Outcome, day: u8) {
        let (finished, first) = self.progress.with_untracked(|p| (p.finished_count(), p.pages_on(1)));
        let mut event = serde_json::to_value(outcome).expect("outcome serialises");
        event["day"] = json!(day);
        event["finished"] = json!(finished);
        event["firstDayPages"] = json!(first);
        self.send(event);
    }

    fn send(self, event: serde_json::Value) {
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
    /// Renders the 29 cards into `grid` and the progress header into `head`,
    /// restoring saved progress from localStorage.
    #[wasm_bindgen(constructor)]
    pub fn new(grid: web_sys::HtmlElement, head: web_sys::HtmlElement, texts_json: &str, on_event: js_sys::Function) -> ScheduleApp {
        let (saved_pages, old_done) = storage::load();
        let progress = Progress::restore(build_days(), saved_pages.as_deref(), old_done.as_deref());

        let state = State {
            progress: RwSignal::new(progress),
            texts: RwSignal::new(Texts::from_json(texts_json)),
            cheer: RwSignal::new(Vec::new()),
            tap_count: RwSignal::new(0),
            top_line: RwSignal::new(String::new()),
            top_for: StoredValue::new(0),
            hint: RwSignal::new(None),
            armed: RwSignal::new(false),
            timers: StoredValue::new((0, 0)),
            notify: StoredValue::new_local(on_event),
        };
        state.roll_top();

        leptos::mount::mount_to(head, move || view! { <header::ProgressHead state=state/> }).forget();

        let days = build_days();
        let cards = leptos::mount::mount_to(grid, move || {
            days.into_iter()
                .enumerate()
                .map(|(i, day)| view! { <card::DayCard n={i as u8 + 1} day=day state=state/> })
                .collect_view()
        });
        cards.forget(); // both live as long as the page

        ScheduleApp { state }
    }

    /// New language: every label updates in place (no re-render, so moons keep their state).
    /// The header gets a fresh line in the new language and the reset button disarms.
    pub fn set_texts(&self, texts_json: &str) {
        self.state.texts.set(Texts::from_json(texts_json));
        self.state.armed.set(false);
        self.state.roll_top();
    }

    pub fn finished_count(&self) -> u8 {
        self.state.progress.with_untracked(|p| p.finished_count())
    }
}
