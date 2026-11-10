//! Light / dark: follows the device until the viewer picks one; the pick is remembered.
//! (js/boot.js applies the saved pick before the page paints, long before this WASM loads.)

use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};

use crate::page::{Page, by_id, document, local_storage, reduce_motion};

const THEME_KEY: &str = "tilawa-theme";
/// Length of the colour cross-fade (`html.theme-anim` in styles/app.css)
const FADE_MS: u64 = 450;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn code(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    fn other(self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    /// The browser bar colour (`<meta name="theme-color">`), same as the page background.
    fn bar_color(self) -> &'static str {
        match self {
            Theme::Light => "#F3F4F6",
            Theme::Dark => "#000000",
        }
    }
}

fn system_dark() -> Option<web_sys::MediaQueryList> {
    web_sys::window()?.match_media("(prefers-color-scheme: dark)").ok().flatten()
}

/// The picked theme (`<html data-theme>`), else the device's.
fn current() -> Theme {
    let picked = document().document_element().and_then(|r| r.get_attribute("data-theme"));
    match picked.as_deref() {
        Some("dark") => Theme::Dark,
        Some("light") => Theme::Light,
        _ if system_dark().is_some_and(|m| m.matches()) => Theme::Dark,
        _ => Theme::Light,
    }
}

/// "Switch to dark mode" / "Switch to light mode", in the current language.
pub fn label(page: Page) {
    let Some(btn) = by_id("theme") else { return };
    let key = if current() == Theme::Dark { "themeToLight" } else { "themeToDark" };
    let lang = page.lang.get_value();
    if let Some(text) = page.catalog.with_value(|c| c.text(lang, key).map(str::to_string)) {
        let _ = btn.set_attribute("aria-label", &text);
        let _ = btn.set_attribute("title", &text);
    }
}

pub fn start(page: Page) {
    let Some(btn) = by_id("theme") else { return };
    let fade = StoredValue::new(0u32); // bumped per click, so an older timer knows it is stale

    let click = Closure::<dyn Fn()>::new(move || {
        let Some(root) = document().document_element() else { return };
        let next = current().other();
        if !reduce_motion() {
            let _ = root.class_list().add_1("theme-anim"); // brief colour cross-fade
            fade.update_value(|n| *n += 1);
            let id = fade.get_value();
            set_timeout(move || {
                if fade.get_value() == id
                    && let Some(root) = document().document_element()
                {
                    let _ = root.class_list().remove_1("theme-anim");
                }
            }, Duration::from_millis(FADE_MS));
        }
        let _ = root.set_attribute("data-theme", next.code());
        if let Some(s) = local_storage() {
            let _ = s.set_item(THEME_KEY, next.code());
        }
        if let Ok(metas) = document().query_selector_all(r#"meta[name="theme-color"]"#) {
            for i in 0..metas.length() {
                if let Some(m) = metas.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                    let _ = m.set_attribute("content", next.bar_color());
                }
            }
        }
        label(page);
    });
    let _ = btn.add_event_listener_with_callback("click", click.as_ref().unchecked_ref());
    click.forget(); // lives as long as the page

    // The device switched light/dark (and the viewer never picked): the label follows
    if let Some(m) = system_dark() {
        let changed = Closure::<dyn Fn()>::new(move || label(page));
        let _ = m.add_event_listener_with_callback("change", changed.as_ref().unchecked_ref());
        changed.forget();
    }
}
