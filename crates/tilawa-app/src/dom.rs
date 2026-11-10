//! Small web-sys helpers for the parts of the page that are plain HTML
//! (index.html) rather than Leptos views: find elements, listen, scroll, timers.

use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Element, HtmlElement, ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("a browser window")
}

pub fn document() -> web_sys::Document {
    window().document().expect("a page")
}

pub fn by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

/// `by_id` as an HtmlElement (for style, offsetWidth, focus…).
pub fn html(id: &str) -> Option<HtmlElement> {
    by_id(id)?.dyn_into().ok()
}

pub fn all(selector: &str) -> Vec<Element> {
    let Ok(list) = document().query_selector_all(selector) else { return Vec::new() };
    (0..list.length())
        .filter_map(|i| list.item(i)?.dyn_into::<Element>().ok())
        .collect()
}

pub fn reduce_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

pub fn local_storage() -> Option<web_sys::Storage> {
    window().local_storage().ok()?
}

/// `el.addEventListener(event, f)` for the life of the page.
pub fn on(el: &web_sys::EventTarget, event: &str, f: impl Fn(web_sys::Event) + 'static) {
    let cb = Closure::<dyn Fn(web_sys::Event)>::new(f);
    let _ = el.add_event_listener_with_callback(event, cb.as_ref().unchecked_ref());
    cb.forget();
}

/// The element an event happened on, if it is one.
pub fn target(e: &web_sys::Event) -> Option<Element> {
    e.target()?.dyn_into().ok()
}

/// Restarts a CSS animation class (e.g. the "flash" outline on a card).
pub fn replay(el: &Element, class: &str) {
    let _ = el.class_list().remove_1(class);
    if let Some(h) = el.dyn_ref::<HtmlElement>() {
        let _ = h.offset_width(); // forces a reflow, so re-adding the class starts over
    }
    let _ = el.class_list().add_1(class);
}

pub fn behavior(smooth: bool) -> ScrollBehavior {
    if smooth && !reduce_motion() { ScrollBehavior::Smooth } else { ScrollBehavior::Instant }
}

/// `el.scrollIntoView({behavior, block, inline: "nearest"})`
pub fn scroll_into_view(el: &Element, smooth: bool, block: ScrollLogicalPosition) {
    let opts = ScrollIntoViewOptions::new();
    opts.set_behavior(behavior(smooth));
    opts.set_block(block);
    opts.set_inline(ScrollLogicalPosition::Nearest);
    el.scroll_into_view_with_scroll_into_view_options(&opts);
}

/// A timer that can be restarted: starting it again drops the earlier callback.
#[derive(Clone, Copy)]
pub struct Timer(StoredValue<u32>);

impl Timer {
    pub fn new() -> Timer {
        Timer(StoredValue::new(0))
    }

    pub fn start(self, ms: u64, f: impl FnOnce() + 'static) {
        self.0.update_value(|n| *n += 1);
        let id = self.0.get_value();
        set_timeout(move || {
            if self.0.get_value() == id {
                f();
            }
        }, Duration::from_millis(ms));
    }

    pub fn cancel(self) {
        self.0.update_value(|n| *n += 1);
    }
}
