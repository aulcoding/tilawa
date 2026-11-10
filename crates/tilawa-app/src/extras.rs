//! Smaller pieces: the accordion under the hero, "copy link", and the Saweria QR code.

use leptos::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::ScrollLogicalPosition;

use crate::State;
use crate::dom::{by_id, document, html, on, scroll_into_view, target, window};

/// Change the link here; the button and the QR code both use it.
pub const SAWERIA_URL: &str = "https://saweria.co/aulcoding";

// ── Accordion: stats, "screen time" and "plan" stay folded until the chevron is tapped ──

fn is_open() -> bool {
    by_id("more").is_some_and(|m| m.class_list().contains("open"))
}

fn set_more(state: State, open: bool) {
    let (Some(more), Some(btn)) = (by_id("more"), by_id("more-toggle")) else { return };
    let _ = more.class_list().toggle_with_force("open", open);
    let _ = btn.set_attribute("aria-expanded", if open { "true" } else { "false" });
    let label = state.texts.with_untracked(|t| if open { t.more_close.clone() } else { t.more_open.clone() });
    let _ = btn.set_attribute("aria-label", &label);
    let _ = btn.set_attribute("title", &label);
    if let Some(inner) = more.query_selector(".more-inner").ok().flatten() {
        let _ = if open { inner.remove_attribute("inert") } else { inner.set_attribute("inert", "") };
    }
}

/// If `hash` points inside the folded part (e.g. the hero's "show me the rules"),
/// opens it and scrolls there. Returns whether it did.
pub fn open_for(state: State, hash: &str) -> bool {
    let Some(id) = hash.strip_prefix('#').filter(|id| !id.is_empty()) else { return false };
    let (Some(target), Some(more)) = (by_id(id), html("more")) else { return false };
    if !more.contains(Some(&target)) {
        return false;
    }
    if !is_open() {
        // Open instantly (no height animation), so the scroll target is measured at its final spot
        let style = more.style();
        let _ = style.set_property("transition", "none");
        set_more(state, true);
        let _ = more.offset_height();
        let _ = style.set_property("transition", "");
    }
    scroll_into_view(&target, true, ScrollLogicalPosition::Start);
    true
}

fn start_accordion(state: State) {
    if let Some(btn) = by_id("more-toggle") {
        on(&btn, "click", move |_| set_more(state, !is_open()));
    }
    on(&document(), "click", move |e| {
        let Some(a) = target(&e).and_then(|t| t.closest(r##"a[href^="#"]"##).ok().flatten()) else { return };
        if open_for(state, &a.get_attribute("href").unwrap_or_default()) {
            e.prevent_default();
        }
    });
}

// ── "Invite a friend": copies the page link ──

/// For browsers without the Clipboard API (or when it refuses): a hidden textarea + execCommand.
fn fallback_copy(text: &str) -> bool {
    let doc = document();
    let Ok(ta) = doc.create_element("textarea") else { return false };
    let Ok(ta) = ta.dyn_into::<web_sys::HtmlTextAreaElement>() else { return false };
    ta.set_value(text);
    let _ = ta.set_attribute("readonly", "");
    let _ = ta.set_attribute("style", "position:fixed;opacity:0");
    let Some(body) = doc.body() else { return false };
    let _ = body.append_child(&ta);
    ta.select();
    let ok = doc
        .dyn_ref::<web_sys::HtmlDocument>()
        .and_then(|d| d.exec_command("copy").ok())
        .unwrap_or(false);
    ta.remove();
    ok
}

/// Copies the page link; the button label shows how it went for a moment (views.rs reads `shared`).
pub fn share(state: State) {
    let show = move |ok: bool| {
        state.shared.set(Some(ok));
        state.share_timer.start(2600, move || state.shared.set(None));
    };
    let href = window().location().href().unwrap_or_default();
    let url = href.split('#').next().unwrap_or_default().to_string();
    // navigator.clipboard is missing on plain http (except localhost)
    let clipboard = js_sys::Reflect::get(&window().navigator(), &"clipboard".into())
        .ok()
        .filter(|c| !c.is_undefined() && !c.is_null());
    let Some(clipboard) = clipboard.map(|c| c.unchecked_into::<web_sys::Clipboard>()) else {
        show(fallback_copy(&url));
        return;
    };
    let promise = clipboard.write_text(&url);
    let done = Closure::once(move |_: JsValue| show(true));
    let failed = Closure::once(move |_: JsValue| show(fallback_copy(&url)));
    let _ = promise.then2(&done, &failed);
    done.forget();
    failed.forget();
}

// ── Saweria: button + QR code ──

/// The QR code as an SVG: one path of dark squares, on white (the card adds the margin).
pub fn qr_svg(text: &str) -> Option<String> {
    let qr = QrCode::encode_text(text, QrCodeEcc::Medium).ok()?;
    let n = qr.size();
    let mut d = String::new();
    for y in 0..n {
        for x in 0..n {
            if qr.get_module(x, y) {
                d.push_str(&format!("M{x} {y}h1v1h-1z"));
            }
        }
    }
    Some(format!(
        r##"<svg viewBox="0 0 {n} {n}" shape-rendering="crispEdges" aria-hidden="true"><rect width="{n}" height="{n}" fill="#FFFFFF"/><path d="{d}" fill="#0B0C0E"/></svg>"##
    ))
}

pub fn start(state: State) {
    start_accordion(state);
}

/// Labels that change with the language.
pub fn label(state: State) {
    set_more(state, is_open());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saweria_qr_is_an_svg_with_modules() {
        let svg = qr_svg(SAWERIA_URL).unwrap();
        assert!(svg.starts_with("<svg viewBox=\"0 0 29 29\""), "{}", &svg[..40]);
        assert!(svg.matches("h1v1h-1z").count() > 100);
    }
}
