//! Installable + offline: registers sw.js (the service worker; see the notes at its top).
//!
//! Only in release builds (what GitHub Pages serves), so `trunk serve` never hands you a
//! cached page while you develop. Browsers only allow service workers on https and localhost.

use wasm_bindgen::prelude::*;

use crate::dom::window;

pub fn register() {
    if cfg!(debug_assertions) {
        return;
    }
    let navigator = window().navigator();
    // Missing on plain http (e.g. http://192.168.x.x): then the site just works online, as before
    let has_sw = js_sys::Reflect::get(&navigator, &"serviceWorker".into()).is_ok_and(|v| !v.is_undefined());
    if !has_sw {
        return;
    }
    let promise = navigator.service_worker().register("sw.js");
    let failed = Closure::once(move |e: JsValue| {
        web_sys::console::warn_2(&"Tilawa: offline mode not available".into(), &e);
    });
    let _ = promise.catch(&failed);
    failed.forget();
}
