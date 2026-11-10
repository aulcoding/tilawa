//! The day carousel (1 card per view on phones, 3 on tablet/desktop), what happens
//! around a card after a tap (the "don't skip" bubble, sliding to the next day),
//! and links to a specific day ("Day 9", "Start Day 1").

use leptos::prelude::*;
use tilawa_core::Outcome;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, ScrollLogicalPosition, ScrollToOptions};

use crate::State;
use crate::dom::{self, Timer, behavior, by_id, document, html, on, reduce_motion, replay, scroll_into_view, target, window};

const DAYS: i32 = 29;
/// Matches gap-3 on #grid
const GAP: f64 = 12.0;

fn grid() -> Option<HtmlElement> {
    html("grid")
}

fn card(day: u8) -> Option<Element> {
    by_id(&format!("day-{day}"))
}

fn slide_width(g: &HtmlElement) -> f64 {
    match g.query_selector(".day").ok().flatten().and_then(|c| c.dyn_into::<HtmlElement>().ok()) {
        Some(c) => c.offset_width() as f64 + GAP,
        None => g.client_width() as f64,
    }
}

fn per_view(g: &HtmlElement) -> i32 {
    (((g.client_width() as f64 + GAP) / slide_width(g)).round() as i32).max(1)
}

/// Day number of the first card in view (1-based).
fn first_visible(g: &HtmlElement) -> i32 {
    if g.first_element_child().is_none() {
        return 1;
    }
    ((g.scroll_left() as f64 / slide_width(g)).round() as i32 + 1).clamp(1, DAYS)
}

/// Scrolls so `day` is the first card in view (clamped so the last view is still full).
pub fn go_to(state: State, day: i32, smooth: bool) {
    let Some(g) = grid() else { return };
    let last = DAYS - per_view(&g) + 1;
    let d = day.min(last).max(1);
    let opts = ScrollToOptions::new();
    opts.set_left((d - 1) as f64 * slide_width(&g));
    opts.set_behavior(behavior(smooth));
    g.scroll_to_with_scroll_to_options(&opts);
    if !smooth {
        update_nav(state);
    }
}

/// Brings `day` into view only if it isn't already.
fn reveal(state: State, day: u8) {
    let Some(g) = grid() else { return };
    let (day, first, n) = (day as i32, first_visible(&g), per_view(&g));
    if day < first {
        go_to(state, day, true);
    } else if day > first + n - 1 {
        go_to(state, day - n + 1, true);
    }
}

/// "Days 4–6 of 29" and the arrows' disabled state.
fn update_nav(state: State) {
    let Some(g) = grid() else { return };
    let (first, n) = (first_visible(&g), per_view(&g));
    let last = (first + n - 1).min(DAYS);
    let text = state.texts.with_untracked(|t| {
        if n == 1 { t.carousel_one.clone() } else { t.carousel_many.clone() }
    });
    if let Some(pos) = by_id("carousel-pos") {
        pos.set_text_content(Some(&text.replace("{from}", &first.to_string()).replace("{to}", &last.to_string())));
    }
    for (id, off) in [("carousel-prev", first <= 1), ("carousel-next", last >= DAYS)] {
        if let Some(b) = by_id(id).and_then(|b| b.dyn_into::<web_sys::HtmlButtonElement>().ok()) {
            b.set_disabled(off);
        }
    }
}

// ── After a tap (called by State::tap) ──

/// What the page does around the card: the bubble, the flash, the slide to the next day.
pub fn after_tap(state: State, outcome: Outcome, day: u8, warn: Timer) {
    match outcome {
        Outcome::Locked { next } => warn_skip(state, next, warn),
        Outcome::UndoFromEnd { last } => point_at(state, last),
        Outcome::Finished => {
            if let Some(c) = card(day) {
                let _ = c.class_list().add_1("just-done");
                set_timeout(move || {
                    if let Some(c) = card(day) {
                        let _ = c.class_list().remove_1("just-done");
                    }
                }, std::time::Duration::from_millis(1600));
            }
            if day < DAYS as u8 {
                // after the moon fills, slide to the next day
                set_timeout(move || reveal(state, day + 1), std::time::Duration::from_millis(900));
            }
        }
        _ => {}
    }
}

/// "Undo from the end": the header shows the hint, here we point at that day.
fn point_at(state: State, day: u8) {
    if let Some(c) = card(day) {
        replay(&c, "flash");
        reveal(state, day);
    }
}

/// "Don't skip this surah" bubble on the day still waiting.
fn warn_skip(state: State, day: u8, timer: Timer) {
    let Some(c) = card(day) else { return };
    for w in dom::all("#grid .day-warn") {
        w.remove();
    }
    let Ok(w) = document().create_element("span") else { return };
    w.set_class_name("day-warn");
    let _ = w.set_attribute("role", "alert");
    w.set_text_content(Some(&state.texts.with_untracked(|t| t.skip_warn.clone())));
    let _ = c.append_child(&w);
    replay(&c, "flash");
    reveal(state, day);

    let r = c.get_bounding_client_rect();
    let inner_h = window().inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
    if r.top() < 90.0 || r.bottom() > inner_h {
        scroll_into_view(&c, true, ScrollLogicalPosition::Center);
    }
    let w = StoredValue::new_local(w);
    timer.start(2600, move || {
        w.with_value(|w| { let _ = w.class_list().add_1("leaving"); });
        let gone = move || w.with_value(|w| w.remove());
        if reduce_motion() { gone() } else { set_timeout(gone, std::time::Duration::from_millis(260)) }
    });
}

// ── Links to a specific day ──
fn on_jump(state: State, e: web_sys::Event) {
    let Some(a) = target(&e).and_then(|t| t.closest("[data-jump]").ok().flatten()) else { return };
    let Some(day) = a.get_attribute("data-jump").and_then(|d| d.parse::<u8>().ok()) else { return };
    let Some(c) = card(day) else { return };
    e.prevent_default();
    go_to(state, day as i32, false);
    scroll_into_view(&c, true, ScrollLogicalPosition::Center);
    replay(&c, "flash");
    set_timeout(move || {
        let hit = card(day).and_then(|c| c.query_selector(".day-hit").ok().flatten().or(Some(c)));
        if let Some(h) = hit.and_then(|h| h.dyn_into::<HtmlElement>().ok()) {
            let opts = web_sys::FocusOptions::new();
            opts.set_prevent_scroll(true);
            let _ = h.focus_with_options(&opts);
        }
    }, std::time::Duration::from_millis(450));
}

pub fn start(state: State) {
    let Some(g) = grid() else { return };
    if let Some(b) = by_id("carousel-prev") {
        on(&b, "click", move |_| {
            if let Some(g) = grid() {
                go_to(state, first_visible(&g) - per_view(&g), true);
            }
        });
    }
    if let Some(b) = by_id("carousel-next") {
        on(&b, "click", move |_| {
            if let Some(g) = grid() {
                go_to(state, first_visible(&g) + per_view(&g), true);
            }
        });
    }
    // One nav update per frame while scrolling
    let queued = StoredValue::new(false);
    on(&g, "scroll", move |_| {
        if !queued.get_value() {
            queued.set_value(true);
            request_animation_frame(move || {
                queued.set_value(false);
                update_nav(state);
            });
        }
    });
    // Arrow keys while focus is inside the carousel
    on(&g, "keydown", move |e| {
        let Some(k) = e.dyn_ref::<web_sys::KeyboardEvent>() else { return };
        let step = match k.key().as_str() {
            "ArrowRight" => 1,
            "ArrowLeft" => -1,
            _ => return,
        };
        e.prevent_default();
        if let Some(g) = grid() {
            go_to(state, first_visible(&g) + step, true);
        }
    });
    // Keep the same first card when the layout changes (rotate phone, resize window)
    let keep = StoredValue::new(None::<i32>);
    let settle = Timer::new();
    on(&window(), "resize", move |_| {
        if keep.get_value().is_none() {
            keep.set_value(grid().map(|g| first_visible(&g)));
        }
        settle.start(120, move || {
            let day = keep.get_value().unwrap_or(1);
            keep.set_value(None);
            go_to(state, day, false);
        });
    });

    on(&document(), "click", move |e| on_jump(state, e));
}

/// Labels that change with the language.
pub fn label(state: State) {
    let (sched, prev, next) = state.texts.with_untracked(|t| {
        (t.sched_h.clone(), t.carousel_prev.clone(), t.carousel_next.clone())
    });
    for (id, text) in [("grid", sched), ("carousel-prev", prev), ("carousel-next", next)] {
        if let Some(el) = by_id(id) {
            let _ = el.set_attribute("aria-label", &text);
        }
    }
    update_nav(state);
}
