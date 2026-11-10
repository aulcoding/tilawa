//! The hero: the moon you can tap toward full, "Breathe in / out" around it,
//! and the "I'll start tomorrow" button that roasts you.

use std::f64::consts::PI;

use leptos::prelude::*;

use crate::State;
use crate::dom::{Timer, by_id, html, on, reduce_motion};

/// Taps from dark (0) to full
const MOON_STEPS: u8 = 8;
/// Starts as a crescent
const MOON_START: u8 = 3;
const TWEEN_MS: f64 = 550.0;

/// SVG path of the lit part of a 200×200 moon at phase `p` (0 = new, 0.5 = full).
pub fn moon_path(p: f64) -> String {
    let (r, c) = (90.0_f64, 100.0_f64);
    if p <= 0.001 {
        return String::new();
    }
    if p >= 0.499 {
        return format!("M{c} {t}A{r} {r} 0 1 1 {c} {b}A{r} {r} 0 1 1 {c} {t}Z", t = c - r, b = c + r);
    }
    let rx = r * (2.0 * PI * p).cos().abs();
    let inner = if p < 0.25 { 0 } else { 1 };
    format!("M{c} {t}A{r} {r} 0 0 1 {c} {b}A{rx:.2} {r} 0 0 {inner} {c} {t}Z", t = c - r, b = c + r)
}

fn phase(step: u8) -> f64 {
    step as f64 / (MOON_STEPS as f64 * 2.0)
}

fn now() -> f64 {
    js_sys::Date::now()
}

// ── Tap the moon: each tap grows it one step toward full; after full it goes dark again ──
fn start_moon() {
    let (Some(lit), Some(btn)) = (by_id("hero-lit"), by_id("moon-tap")) else { return };
    // (step, drawn phase, animation id)
    let moon = StoredValue::new((MOON_START, phase(MOON_START), 0u32));
    let lit = StoredValue::new_local(lit);

    let draw = move |p: f64| {
        moon.update_value(|m| m.1 = p);
        lit.with_value(|l| { let _ = l.set_attribute("d", &moon_path(p)); });
    };
    draw(phase(MOON_START));

    on(&btn, "click", move |_| {
        let (step, from, _) = moon.get_value();
        if step >= MOON_STEPS {
            // Full moon: fade out, then back to dark
            moon.update_value(|m| { m.0 = 0; m.2 += 1; });
            lit.with_value(|l| { let _ = l.class_list().add_1("fading"); });
            let back = move || {
                draw(0.0);
                lit.with_value(|l| { let _ = l.class_list().remove_1("fading"); });
            };
            if reduce_motion() { back() } else { set_timeout(back, std::time::Duration::from_millis(400)) }
            return;
        }
        let to = phase(step + 1);
        moon.update_value(|m| { m.0 = step + 1; m.2 += 1; });
        let id = moon.get_value().2;
        if reduce_motion() {
            draw(to);
            return;
        }
        // Ease-out tween, one frame at a time; a newer tap (new id) stops this one
        let t0 = now();
        fn frame(id: u32, t0: f64, from: f64, to: f64, moon: StoredValue<(u8, f64, u32)>, draw: impl Fn(f64) + Copy + 'static) {
            request_animation_frame(move || {
                if moon.get_value().2 != id {
                    return;
                }
                let k = ((now() - t0) / TWEEN_MS).min(1.0);
                let e = 1.0 - (1.0 - k).powi(3);
                draw(from + (to - from) * e);
                if k < 1.0 {
                    frame(id, t0, from, to, moon, draw);
                }
            });
        }
        frame(id, t0, from, to, moon, draw);
    });
}

// ── Breathe in / out: every 4 s (in step with the moon's 8 s breathing) the label fades out,
//    jumps to a random point just outside the moon, swaps text and fades back in ──
fn start_breath() {
    if reduce_motion() {
        return;
    }
    let Some(breath) = html("breath") else { return };
    let breath = StoredValue::new_local(breath);

    fn place(b: &web_sys::HtmlElement) {
        let Some(bx) = b.parent_element() else { return };
        let (w, h) = (bx.client_width() as f64, bx.client_height() as f64);
        let a = js_sys::Math::random() * PI * 2.0;
        let r = 0.53 + js_sys::Math::random() * 0.07; // the disk edge is at 0.45 of the box
        let half = b.offset_width() as f64 / 2.0;
        let slack = 24.0; // keep the label near the moon box
        let x = (w / 2.0 + a.cos() * w * r).clamp(half - slack, (w - half + slack).max(half - slack));
        let y = h / 2.0 + a.sin() * h * r;
        let s = b.style();
        let _ = s.set_property("left", &format!("{x}px"));
        let _ = s.set_property("top", &format!("{y}px"));
    }

    fn cycle(breath: StoredValue<web_sys::HtmlElement, LocalStorage>, inhale: bool) {
        breath.with_value(|b| {
            let _ = b.set_attribute("data-phase", if inhale { "in" } else { "out" });
            place(b);
            let _ = b.class_list().add_1("show");
        });
        set_timeout(move || breath.with_value(|b| { let _ = b.class_list().remove_1("show"); }),
            std::time::Duration::from_millis(3000));
        set_timeout(move || cycle(breath, !inhale), std::time::Duration::from_millis(4000));
    }
    cycle(breath, true);
}

// ── "I'll start tomorrow" → "Sure you will." ──
fn start_later(state: State) {
    let (Some(later), Some(live)) = (by_id("later"), by_id("later-live")) else { return };
    let timer = Timer::new();
    let (later, live) = (StoredValue::new_local(later), StoredValue::new_local(live));
    later.with_value(|el| on(el, "click", move |_| {
        later.with_value(|l| { let _ = l.class_list().add_1("roasted"); });
        live.with_value(|l| l.set_text_content(Some(&state.texts.with_untracked(|t| t.later.clone()))));
        timer.start(2400, move || {
            later.with_value(|l| { let _ = l.class_list().remove_1("roasted"); });
            live.with_value(|l| l.set_text_content(Some("")));
        });
    }));
}

pub fn start(state: State) {
    start_moon();
    start_breath();
    start_later(state);
}

/// Labels that change with the language.
pub fn label(state: State) {
    if let Some(b) = by_id("moon-tap") {
        let _ = b.set_attribute("aria-label", &state.texts.with_untracked(|t| t.moon_label.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_moon_draws_nothing_and_full_is_two_arcs() {
        assert_eq!(moon_path(0.0), "");
        assert_eq!(moon_path(0.5), "M100 10A90 90 0 1 1 100 190A90 90 0 1 1 100 10Z");
    }

    #[test]
    fn crescent_matches_the_old_javascript() {
        // moonPath(3/16) in js/hero.js
        assert_eq!(moon_path(phase(3)), "M100 10A90 90 0 0 1 100 190A34.44 90 0 0 0 100 10Z");
    }
}
