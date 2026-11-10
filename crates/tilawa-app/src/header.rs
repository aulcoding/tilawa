//! The progress header above the cards: "12/29", the line next to it, the bar,
//! and the two-tap reset button. Everything here is derived from the same
//! signals as the cards, so it updates by itself after every tap.

use leptos::prelude::*;

use crate::State;

const RESET: &str = "py-1.5 text-[14px] text-muted underline underline-offset-3 hover:text-fg \
    disabled:cursor-default disabled:no-underline disabled:opacity-40 \
    max-md:col-span-full max-md:justify-self-start [&.armed]:text-fg";

/// The line next to the big number. A hint ("undo from the end") wins for a moment;
/// otherwise: welcome before any page, the rolled top line, or a plain fallback.
pub fn message(state: State, finished: u8, first_day_empty: bool) -> String {
    if let Some(h) = state.hint.get() {
        return h;
    }
    let top = state.top_line.get();
    state.texts.with(|t| {
        let m = |i: usize| t.msg.get(i).cloned().unwrap_or_default();
        if finished == 0 && first_day_empty {
            m(0)
        } else if finished >= 29 {
            m(3)
        } else if !top.is_empty() {
            top
        } else if finished < 15 {
            m(1)
        } else {
            m(2)
        }
    })
}

#[component]
pub fn ProgressHead(state: State) -> impl IntoView {
    let finished = Memo::new(move |_| state.progress.with(|p| p.finished_count()));
    let first_day_empty = Memo::new(move |_| state.progress.with(|p| p.pages_on(1) == 0));
    let empty = Memo::new(move |_| state.progress.with(|p| p.pages().iter().all(|&v| v == 0)));
    let armed = state.armed;

    view! {
        <div class="text-count font-light tracking-stats tabular-nums">
            <span id="count">{move || finished.get()}</span>
            <small class="ml-0.5 text-[.42em] tracking-[-.01em] text-muted">"/29"</small>
        </div>
        <div>
            <p class="mb-2.5 min-h-[1.4em] text-[17px] leading-[1.4] max-md:flex max-md:min-h-[2.8em] max-md:items-end"
                id="msg" aria-live="polite">
                {move || message(state, finished.get(), first_day_empty.get())}
            </p>
            <div class="h-0.5 overflow-hidden rounded-[2px] bg-line">
                <i class="block h-full bg-fg transition-[width] duration-500 motion-reduce:transition-none" id="bar"
                    style:width=move || format!("{}%", finished.get() as f64 / 29.0 * 100.0)></i>
            </div>
        </div>
        <button class=RESET class:armed=move || armed.get() id="reset" type="button"
            disabled=move || empty.get()
            on:click=move |_| state.press_reset()
        >
            {move || state.texts.with(|t| if armed.get() { t.reset_armed.clone() } else { t.reset.clone() })}
        </button>
    }
}
