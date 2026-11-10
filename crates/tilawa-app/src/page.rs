//! The language switch for the whole page: fills every `[data-i18n]` element,
//! the tab title and `<html lang>`, slides the text out and back in, remembers
//! the choice, and keeps the "Start Day 1" button wide enough for both languages.
//!
//! Elements marked `data-swipe` (the cards, the header line…) slide with the text.
//!
//! `Page::start` also starts every other part of the page (theme, hero, carousel, extras),
//! and `apply` refreshes their labels when the language changes.

use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Animation, Element};

use crate::dom::{all, by_id, document, html, local_storage, on, reduce_motion, window};
use crate::{State, carousel, extras, hero, theme};
use crate::i18n::{Catalog, Lang};

const LANG_KEY: &str = "tilawa-lang";
const SWIPE_OUT_MS: u64 = 170;

#[derive(Clone, Copy)]
pub struct Page {
    pub state: State,
    pub catalog: StoredValue<Catalog>,
    pub lang: StoredValue<Lang>,
    /// The running "slide out" animations, and an id so a late timer knows it is stale
    swipe: StoredValue<(u32, Vec<Animation>), LocalStorage>,
}

/// Saved choice, else the browser language.
pub fn starting_lang() -> Lang {
    if let Some(saved) = local_storage().and_then(|s| s.get_item(LANG_KEY).ok().flatten()) {
        return Lang::from_code(&saved);
    }
    let browser = window().navigator().language().unwrap_or_default();
    Lang::from_browser(&browser)
}

/// `[{…}, {…}]` keyframes from two JSON objects, e.g. `{"opacity":1,"transform":"translateY(0)"}`.
fn keyframes(from: &str, to: &str) -> js_sys::Object {
    js_sys::JSON::parse(&format!("[{from},{to}]")).expect("valid keyframes").unchecked_into()
}

/// `el.animate(frames, {duration, delay, easing, fill})`. Called through Reflect because
/// web-sys still marks the options form of `animate()` as unstable.
fn slide(el: &Element, frames: &js_sys::Object, ms: f64, delay: f64, easing: &str, fill: &str) -> Option<Animation> {
    let opts = js_sys::JSON::parse(&format!(
        r#"{{"duration":{ms},"delay":{delay},"easing":"{easing}","fill":"{fill}"}}"#
    )).ok()?;
    let animate: js_sys::Function = js_sys::Reflect::get(el, &"animate".into()).ok()?.dyn_into().ok()?;
    Some(animate.call2(el, frames, &opts).ok()?.unchecked_into())
}

impl Page {
    pub fn new(state: State, catalog: Catalog, lang: Lang) -> Page {
        Page {
            state,
            catalog: StoredValue::new(catalog),
            lang: StoredValue::new(lang),
            swipe: StoredValue::new_local((0, Vec::new())),
        }
    }

    /// Starts every part of the page, then fills the text in the starting language.
    pub fn start(self) {
        let state = self.state;
        theme::start(state);
        hero::start(state);
        extras::start(state);
        carousel::start(state);
        self.set_lang(self.lang.get_value(), false);

        if let Some(btn) = by_id("lang") {
            on(&btn, "click", move |_| {
                let next = self.lang.get_value().other();
                self.set_lang(next, true);
                if let Some(s) = local_storage() {
                    let _ = s.set_item(LANG_KEY, next.code());
                }
            });
        }

        // Open on the day you're reading, and on the section a link like …/#plan points at
        let finished = state.progress.with_untracked(|p| p.finished_count());
        carousel::go_to(state, finished as i32 + 1, false);
        extras::open_for(state, &window().location().hash().unwrap_or_default());

        self.lock_nav_start();
        // Measure again once the web font is in (it changes the width)
        if let Ok(ready) = document().fonts().ready() {
            let again = Closure::once(move |_: JsValue| self.lock_nav_start());
            let _ = ready.then(&again);
            again.forget();
        }
        // Switch animation on only after the first paint, so it doesn't slide on load
        request_animation_frame(move || {
            request_animation_frame(move || {
                if let Some(b) = by_id("lang") {
                    let _ = b.class_list().add_1("ready");
                }
            })
        });
    }

    /// Switches language. With `animate`, the old text slides up and out, the new comes up from below.
    pub fn set_lang(self, lang: Lang, animate: bool) {
        self.lang.set_value(lang);
        if let Some(b) = by_id("lang") {
            let _ = b.set_attribute("data-lang", lang.code()); // the moon-fill on the switch starts right away
        }

        // A switch while the last one is still sliding: drop the old one
        let mut id = 0;
        self.swipe.update_value(|(n, anims)| {
            *n += 1;
            id = *n;
            anims.drain(..).for_each(|a| a.cancel());
        });

        if !animate || reduce_motion() {
            self.apply(lang);
            return;
        }

        let targets: Vec<Element> = all("[data-i18n], [data-swipe]")
            .into_iter()
            .filter(|el| {
                el.id() != "skip" && el.closest(".btn-roast, .breath").ok().flatten().is_none()
            })
            .collect();
        let out = keyframes(r#"{"opacity":1,"transform":"translateY(0)"}"#, r#"{"opacity":0,"transform":"translateY(-0.5em)"}"#);
        let anims: Vec<Animation> = targets
            .iter()
            .filter_map(|el| slide(el, &out, SWIPE_OUT_MS as f64, 0.0, "cubic-bezier(.5,0,.75,0)", "forwards"))
            .collect();
        self.swipe.update_value(|(_, a)| *a = anims);

        let targets = StoredValue::new_local(targets);
        set_timeout(move || {
            if self.swipe.with_value(|(n, _)| *n != id) {
                return;
            }
            self.apply(lang);
            self.swipe.update_value(|(_, a)| a.drain(..).for_each(|a| a.cancel()));
            let back = keyframes(r#"{"opacity":0,"transform":"translateY(0.6em)"}"#, r#"{"opacity":1,"transform":"translateY(0)"}"#);
            targets.with_value(|els| {
                for (i, el) in els.iter().enumerate() {
                    let delay = (i as f64 * 6.0).min(120.0);
                    slide(el, &back, 380.0, delay, "cubic-bezier(.2,.8,.2,1)", "backwards");
                }
            });
        }, Duration::from_millis(SWIPE_OUT_MS));
    }

    /// Puts `lang`'s text everywhere and tells JS.
    fn apply(self, lang: Lang) {
        self.catalog.with_value(|c| {
            for el in all("[data-i18n]") {
                let key = el.get_attribute("data-i18n").unwrap_or_default();
                if let Some(v) = c.text(lang, &key) {
                    el.set_inner_html(v);
                }
            }
            let doc = document();
            if let Some(root) = doc.document_element() {
                let _ = root.set_attribute("lang", lang.code());
            }
            if let Some(title) = c.text(lang, "title") {
                doc.set_title(title);
            }
            if let (Some(b), Some(label)) = (by_id("lang"), c.text(lang, "langLabel")) {
                let _ = b.set_attribute("aria-label", label);
            }
            // Cards + header update their labels in place
            self.state.texts.set(c.texts(lang));
        });
        self.state.armed.set(false);
        self.state.roll_top();
        // Labels the other parts set themselves (aria-labels, "Days 1–3 of 29"…)
        theme::label(self.state);
        hero::label(self.state);
        extras::label(self.state);
        carousel::label(self.state);
    }

    /// Keeps the EN/ID switch perfectly still: the "Start Day 1" button next to it gets a
    /// fixed width that fits both languages, so its label change can't push the switch.
    fn lock_nav_start(self) {
        let Some(b) = html("nav-start") else { return };
        let keep = b.inner_html();
        let style = b.style();
        let _ = style.set_property("min-width", "");
        let mut width: f64 = 0.0;
        self.catalog.with_value(|c| {
            for lang in [Lang::En, Lang::Id] {
                if let Some(label) = c.text(lang, "navStart") {
                    b.set_inner_html(label);
                    width = width.max(b.get_bounding_client_rect().width());
                }
            }
        });
        b.set_inner_html(&keep);
        let _ = style.set_property("min-width", &format!("{}px", width.ceil()));
    }
}
