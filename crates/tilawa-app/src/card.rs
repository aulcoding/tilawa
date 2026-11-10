//! One day card. Every part that can change (classes, counts, status line, labels)
//! is a closure over signals, so Leptos updates just that bit of the DOM.

use leptos::prelude::*;
use tilawa_core::{Day, Tag};

use crate::State;

// Tailwind classes (styles/app.css scans this file). State classes such as
// `is-done` are toggled below with class:… so Tailwind's [&.is-done]: variants apply.
const CARD: &str = "day group/day relative flex min-h-[204px] flex-col gap-3 rounded-card border border-line bg-surface px-[18px] pt-[18px] pb-3.5 text-left \
    transition-[border-color,background-color] duration-250 [-webkit-tap-highlight-color:transparent] \
    hover:border-line-strong [&.is-next]:border-line-strong \
    [&.is-done]:border-line-strong [&.is-done]:bg-surface-done \
    [&.locked]:opacity-50 [&.locked]:hover:border-line [&.flash]:opacity-100! \
    w-[calc((100%-24px)/3)] flex-none snap-start max-md:w-full";
const STEP: &str = "day-step grid size-8 place-items-center rounded-full border border-line-strong bg-bg text-[18px] leading-none text-fg \
    transition-[border-color,opacity,transform] duration-200 hover:border-fg active:scale-90 \
    aria-disabled:opacity-35 aria-disabled:hover:border-line-strong group-[.locked]/day:opacity-35";

fn tag_key(tag: Tag) -> &'static str {
    match tag {
        Tag::Boss => "boss",
        Tag::Pack => "pack",
        Tag::Half => "half",
        Tag::End => "end",
    }
}

/// The card's moon as SVG markup. Phase per day: a crescent on Day 1, full on Day 15,
/// a last sliver on Day 29. The bright layer sits behind a clip that rises from the
/// bottom when the day is done (CSS: .day-moon in styles/app.css).
fn moon_svg(n: u8) -> String {
    let n = n as f64;
    let p = if n <= 15.0 { 0.08 + (n - 1.0) * 0.03 } else { 0.5 + (n - 15.0) * 0.03 };
    let (r, c) = (18.0_f64, 20.0_f64);
    let shape = |cls: &str| -> String {
        if (p - 0.5).abs() < 0.001 {
            return format!(r#"<circle class="{cls}" cx="{c}" cy="{c}" r="{r}"/>"#);
        }
        let rx = r * (2.0 * std::f64::consts::PI * p).cos().abs();
        let wax = p < 0.5;
        let outer = if wax { 1 } else { 0 };
        let inner = if wax { (p >= 0.25) as u8 } else { (p > 0.75) as u8 };
        format!(
            r#"<path class="{cls}" d="M{c} {t}A{r} {r} 0 0 {outer} {c} {b}A{rx:.2} {r} 0 0 {inner} {c} {t}Z"/>"#,
            t = c - r,
            b = c + r,
        )
    };
    let id = n as u8;
    format!(
        r#"<svg class="day-moon size-[38px] flex-none" viewBox="0 0 40 40" aria-hidden="true"><defs><clipPath id="rise-{id}"><rect class="rise" x="0" y="0" width="40" height="40"/></clipPath></defs><circle class="disk" cx="{c}" cy="{c}" r="{r}"/>{base}<g clip-path="url(#rise-{id})">{on}</g></svg>"#,
        base = shape("lit-base"),
        on = shape("lit-on"),
    )
}

#[component]
pub fn DayCard(n: u8, day: Day, state: State) -> impl IntoView {
    let i = n as usize - 1;
    let total = day.total();
    let State { progress, texts, cheer, tap_count, .. } = state;

    // Derived state. Memos only notify when their value really changes.
    let pages = Memo::new(move |_| progress.with(|p| p.pages_on(n)));
    let current = Memo::new(move |_| progress.with(|p| p.current_day()));
    let can_add = Memo::new(move |_| progress.with(|p| p.can_add(n)));
    let can_remove = Memo::new(move |_| progress.with(|p| p.can_remove(n)));
    let is_done = move || pages.get() >= total;
    let locked = move || current.get().is_some_and(|c| n > c);
    let is_next = move || current.get() == Some(n);

    // The line under the surah names: the "done" line once finished, else the latest cheer.
    let status = move || {
        if is_done() {
            texts.with(|t| t.done_days.get(i).cloned().unwrap_or_else(|| t.done.clone()))
        } else if pages.get() > 0 {
            cheer.with(|c| c.iter().find(|(d, _)| *d == n).map(|(_, l)| l.clone()).unwrap_or_default())
        } else {
            String::new()
        }
    };

    let range = {
        let [a, b] = day.surah_range;
        move || texts.with(|t| if a == b { format!("{}{a}", t.surah) } else { format!("{}{a}–{b}", t.surahs) })
    };
    let [first_page, last_page] = day.pages;
    let tag = day.tag;
    let surahs = day.surahs.clone();

    view! {
        <article
            id=format!("day-{n}")
            data-day=n
            class=CARD
            class:is-done=is_done
            class:locked=locked
            class:is-next=is_next
            class:has-status=move || !status().is_empty()
        >
            // The whole card is a "+1 page" button; the text above lets taps through
            <button
                class="day-hit absolute inset-0 z-0 rounded-[inherit] focus-visible:outline-offset-2"
                type="button"
                aria-label=move || {
                    let t = texts.read();
                    let more = if can_add.get() { format!(". {}", t.page_plus) } else { String::new() };
                    format!("{} {n}: {}/{total} {}{more}", t.day, pages.get(), t.pages_unit)
                }
                on:click=move |_| state.tap(n, 1)
            ></button>

            <div class="flex items-start justify-between gap-2">
                <span class="text-[14px] leading-[1.2] text-muted">
                    {move || texts.read().day.clone()}
                    <b class="mt-1 block text-[34px] leading-none font-light tracking-stats text-fg tabular-nums">{n}</b>
                </span>
                <span class="contents" inner_html=moon_svg(n)></span>
            </div>

            {tag.map(|tag| view! {
                <span class="self-start rounded-xl border border-line-strong px-2.5 py-[5px] text-[12.5px] leading-[1.3]">
                    {move || texts.with(|t| t.tags.get(tag_key(tag)).cloned().unwrap_or_default())}
                </span>
            })}

            <div class="mt-auto text-[17px] leading-[1.3] tracking-[-.01em] [&>span]:block">
                {surahs.into_iter().map(|name| {
                    // "Al-Hujurat – An-Najm": keep each name on one line
                    let parts: Vec<String> = name.split(" – ").map(str::to_string).collect();
                    let last = parts.len() - 1;
                    view! {
                        <span>
                            {parts.into_iter().enumerate().map(|(k, part)| view! {
                                <span class="whitespace-nowrap">{part}</span>
                                {(k < last).then_some(" – ")}
                            }).collect_view()}
                        </span>
                    }
                }).collect_view()}
            </div>

            // Re-created on every tap, so the CSS "rise" animation plays for each new line
            {move || {
                tap_count.track();
                view! {
                    <p class="day-status line-clamp-2 h-[2.7em] text-[13px] leading-[1.35] text-muted group-[.is-done]/day:text-fg" aria-live="polite">
                        {status()}
                    </p>
                }
            }}

            <div class="flex flex-col gap-2">
                <div class="day-bar h-[3px] overflow-hidden rounded-[3px] bg-line">
                    <i
                        class="block h-full rounded-[inherit] bg-fg transition-[width] duration-350 ease-soft motion-reduce:transition-none"
                        style:width=move || format!("{}%", pages.get() as f64 / total as f64 * 100.0)
                    ></i>
                </div>
                <div class="flex items-center justify-between gap-2">
                    <span class="day-count text-[13px] whitespace-nowrap text-muted tabular-nums">
                        <b class="font-medium text-fg">{move || pages.get()}</b>
                        {move || format!("/{total} {}", texts.read().pages_unit)}
                    </span>
                    <span class="day-ctrl inline-flex gap-1.5">
                        <button class=STEP type="button" data-step="-1"
                            aria-label=move || texts.read().page_minus.clone()
                            aria-disabled=move || (!can_remove.get()).to_string()
                            on:click=move |_| state.tap(n, -1)
                        >"−"</button>
                        <button class=STEP type="button" data-step="1"
                            aria-label=move || texts.read().page_plus.clone()
                            aria-disabled=move || (!can_add.get()).to_string()
                            on:click=move |_| state.tap(n, 1)
                        >"+"</button>
                    </span>
                </div>
            </div>

            <div class="flex justify-between gap-2 text-[12.5px] text-muted [&>span]:whitespace-nowrap">
                <span>{range}</span>
                <span>{move || format!("{} {first_page}–{last_page}", texts.read().pages_short)}</span>
            </div>
        </article>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_fifteen_moon_is_a_full_circle() {
        assert!(moon_svg(15).contains(r#"<circle class="lit-on""#));
    }

    #[test]
    fn each_moon_has_its_own_clip_id() {
        assert!(moon_svg(3).contains(r#"id="rise-3""#));
        assert!(moon_svg(3).contains("url(#rise-3)"));
    }
}
