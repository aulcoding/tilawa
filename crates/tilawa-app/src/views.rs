//! The whole page as Leptos components (index.html only has <head>).
//!
//! Text: `t("key")` is a closure over the current language, so every label
//! updates in place when the language changes. Values may contain a little HTML
//! (<span>, <em>, <a>, &amp;), so they go in with `inner_html`, like before.
//!
//! `data-swipe` marks what slides out/in on a language switch (page.rs).
//! The ids are what the plain web-sys modules look for (hero.rs, carousel.rs, extras.rs…).

use leptos::prelude::*;
use tilawa_core::build_days;

use crate::{State, card::DayCard, header::ProgressHead};

#[component]
pub fn App(state: State) -> impl IntoView {
    view! {
        <SkipLink state=state/>
        <Nav state=state/>
        <main id="top">
            <Hero state=state/>
            <More state=state/>
            <Schedule state=state/>
            <Excuses state=state/>
            <Closing state=state/>
            <Support state=state/>
        </main>
        <Footer state=state/>
    }
}

#[component]
fn SkipLink(state: State) -> impl IntoView {
    view! {
        <a id="skip" class="absolute -top-[60px] left-3 z-50 rounded-[10px] bg-btn px-3.5 py-2.5 text-btn-fg focus:top-3"
            href="#schedule" inner_html=state.t("skip")></a>
    }
}

/// The small moon + "Tilawa" (nav and footer).
#[component]
fn Logo(#[prop(into)] class: String, #[prop(default = false)] hide_name_on_phones: bool) -> impl IntoView {
    view! {
        <a class=class href="#top">
            <svg class="size-5" viewBox="0 0 40 40" aria-hidden="true">
                <circle class="fill-moon-dark stroke-line-strong" cx="20" cy="20" r="18"/>
                <path class="fill-moon-lit" d="M20 2A18 18 0 0 1 20 38A9.6 18 0 0 0 20 2Z"/>
            </svg>
            <span class=if hide_name_on_phones { "max-xs:hidden" } else { "" }>"Tilawa"</span>
        </a>
    }
}

// ───────────── Nav ─────────────

#[component]
fn NavLink(state: State, href: &'static str, key: &'static str) -> impl IntoView {
    view! {
        <a class="text-muted transition-colors duration-200 hover:text-fg max-md:hidden" href=href
            inner_html=state.t(key) data-swipe></a>
    }
}

#[component]
fn Nav(state: State) -> impl IntoView {
    view! {
        <header class="sticky top-0 z-20 border-b border-line bg-nav pt-[env(safe-area-inset-top,0px)] backdrop-blur-[14px] backdrop-saturate-[1.4]">
            <div class="wrap flex h-16 items-center justify-between gap-6 max-md:gap-3">
                <Logo class="inline-flex items-center gap-2.5 text-[18px] font-medium tracking-snug" hide_name_on_phones=true/>
                <nav class="flex items-center gap-7 text-[15px] max-md:gap-2.5" aria-label="Main">
                    <NavLink state=state href="#schedule" key="navSchedule"/>
                    <NavLink state=state href="#excuses" key="navExcuses"/>
                    <NavLink state=state href="#support" key="navSupport"/>

                    // EN/ID: the moon-fill slides with data-lang (page.rs sets it, styles/app.css .lang)
                    <button class="lang relative isolate inline-flex h-[38px] items-stretch gap-0.5 rounded-full border border-line-strong p-[3px] text-[13px] leading-none font-medium tracking-[.04em] text-muted transition-colors duration-200 hover:border-fg"
                        id="lang" type="button" aria-label=state.t("langLabel")>
                        <span class="l-en" aria-hidden="true">"EN"</span>
                        <span class="l-id" aria-hidden="true">"ID"</span>
                    </button>

                    <button class="theme relative grid size-[38px] flex-none place-items-center rounded-full border border-line-strong text-fg transition-colors duration-200 hover:border-fg" id="theme" type="button">
                        <svg class="ic-moon col-start-1 row-start-1 size-[18px] fill-current" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5Z"/>
                        </svg>
                        <svg class="ic-sun col-start-1 row-start-1 size-[18px] fill-none stroke-current stroke-[1.8] [stroke-linecap:round]" viewBox="0 0 24 24" aria-hidden="true">
                            <circle class="fill-current stroke-none" cx="12" cy="12" r="4"/>
                            <path d="M12 2.5v2M12 19.5v2M2.5 12h2M19.5 12h2M5.3 5.3l1.4 1.4M17.3 17.3l1.4 1.4M5.3 18.7l1.4-1.4M17.3 6.7l1.4-1.4"/>
                        </svg>
                    </button>

                    <a class="btn btn-primary btn-sm" id="nav-start" href="#schedule" data-jump="1"
                        inner_html=state.t("navStart") data-swipe></a>
                </nav>
            </div>
        </header>
    }
}

// ───────────── Hero ─────────────

#[component]
fn Hero(state: State) -> impl IntoView {
    let t = move |k| state.t(k);
    view! {
        <section class="relative flex min-h-[calc(100svh-64px)] items-center overflow-hidden pt-[clamp(56px,8vh,96px)] pb-[clamp(72px,10vh,112px)] max-2xl:block max-2xl:min-h-0 max-2xl:pt-8 max-2xl:pb-[72px]">
            <div class="wrap relative z-1 grid w-full grid-cols-[minmax(0,1fr)_minmax(220px,420px)] items-center gap-[clamp(40px,6vw,96px)] max-2xl:grid-cols-1 max-2xl:gap-[72px]">
                <div class="hero-copy min-w-0">
                    <p class="mb-[22px] text-[16px] text-muted" inner_html=t("kicker") data-swipe></p>
                    <h1 class="text-hero font-light tracking-display text-balance [&_span]:block" inner_html=t("h1") data-swipe></h1>
                    <p class="mt-8 mb-9 max-w-[52ch] text-sub text-muted" inner_html=t("heroSub") data-swipe></p>
                    <div class="flex flex-wrap gap-3">
                        <a class="btn btn-primary" href="#plan" inner_html=t("ctaPlan") data-swipe></a>
                        // "I'll start tomorrow" → "Sure. Like yesterday." (hero.rs adds .roasted)
                        <button class="btn btn-ghost btn-roast" id="later" type="button" data-swipe>
                            <span class="lbl lbl-a" inner_html=t("laterA")></span>
                            <span class="lbl lbl-b" aria-hidden="true" inner_html=t("laterB")></span>
                        </button>
                        <span class="sr-only" aria-live="polite" id="later-live"></span>
                    </div>
                </div>

                <div class="hero-moon pointer-events-none relative aspect-square w-full max-w-full justify-self-end max-2xl:order-first max-2xl:w-[min(58vw,300px)] max-2xl:justify-self-center">
                    <div class="moon-glide absolute inset-0">
                        <div class="glow absolute -inset-[16%] rounded-full bg-[radial-gradient(closest-side,var(--glow),transparent_72%)]" aria-hidden="true"></div>
                        <svg class="relative block size-full origin-center" viewBox="0 0 200 200" aria-hidden="true">
                            <circle class="fill-moon-dark" cx="100" cy="100" r="90"/>
                            <path class="lit fill-moon-lit transition-opacity duration-400" id="hero-lit" d=""/>
                        </svg>
                        <button class="moon-tap pointer-events-auto absolute inset-[5%] rounded-full [-webkit-tap-highlight-color:transparent] focus-visible:rounded-full focus-visible:outline-offset-4" id="moon-tap" type="button"></button>
                    </div>
                    <div class="breath pointer-events-none absolute top-1/2 left-1/2 -translate-1/2 text-[13px] tracking-label whitespace-nowrap text-muted uppercase opacity-0 transition-opacity duration-900 [&.show]:opacity-100"
                        id="breath" data-phase="in" aria-hidden="true">
                        <span inner_html=t("breathIn")></span>
                        <span inner_html=t("breathOut")></span>
                    </div>
                </div>
            </div>
        </section>
    }
}

// ───────────── Accordion: stats, screen time, plan ─────────────

/// One rule card in "the plan": number, icon, title (`h`), note (`p`), key number (`s`).
#[component]
fn RuleCard(state: State, n: u8, h: &'static str, p: &'static str, s: &'static str, icon: AnyView) -> impl IntoView {
    view! {
        <article class="flex flex-col gap-4 rounded-card border border-line bg-surface p-6 transition-colors duration-200 hover:border-line-strong">
            <div class="flex items-center justify-between">
                <span class="font-mono text-[13px] tracking-[.08em] text-muted">{format!("{n:02}")}</span>
                <span class="grid size-11 place-items-center rounded-full border border-line-strong text-fg">
                    <svg class="size-[22px] fill-none stroke-current stroke-[1.5] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 24 24" aria-hidden="true">
                        {icon}
                    </svg>
                </span>
            </div>
            <h3 class="text-[21px] leading-[1.25] font-normal tracking-snug" inner_html=state.t(h) data-swipe></h3>
            <p class="text-[15px] leading-[1.55] text-muted" inner_html=state.t(p) data-swipe></p>
            <span class="mt-auto self-start rounded-full border border-line-strong px-2.5 py-1 text-[12.5px] tabular-nums" inner_html=state.t(s) data-swipe></span>
        </article>
    }
}

#[component]
fn More(state: State) -> impl IntoView {
    let t = move |k| state.t(k);
    // Icons: swords, handshake (Lucide), a full moon, trending-down (Lucide)
    let swords = view! {
        <polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5"/><line x1="13" x2="19" y1="19" y2="13"/>
        <line x1="16" x2="20" y1="16" y2="20"/><line x1="19" x2="21" y1="21" y2="19"/>
        <polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5"/><line x1="5" x2="9" y1="14" y2="18"/>
        <line x1="7" x2="4" y1="17" y2="20"/><line x1="3" x2="5" y1="19" y2="21"/>
    }.into_any();
    let handshake = view! {
        <path d="m11 17 2 2a1 1 0 1 0 3-3"/>
        <path d="m14 14 2.5 2.5a1 1 0 1 0 3-3l-3.88-3.88a3 3 0 0 0-4.24 0l-.88.88a1 1 0 1 1-3-3l2.81-2.81a5.79 5.79 0 0 1 7.06-.87l.47.28a2 2 0 0 0 1.42.25L21 4"/>
        <path d="m21 3 1 11h-2"/><path d="M3 3 2 14l6.5 6.5a1 1 0 1 0 3-3"/><path d="M3 4h8"/>
    }.into_any();
    let full_moon = view! { <circle class="fill-current stroke-none" cx="12" cy="12" r="9"/> }.into_any();
    let trending_down = view! { <path d="M16 17h6v-6"/><path d="m22 17-8.5-8.5-5 5L2 7"/> }.into_any();

    view! {
        // The chevron that opens/closes the folded part (extras.rs)
        <div class="relative z-2 flex h-0 justify-center border-t border-line">
            <button class="group absolute top-0 grid size-10 -translate-y-1/2 place-items-center rounded-full border border-line-strong bg-bg text-muted transition-colors duration-200 hover:border-fg hover:text-fg"
                id="more-toggle" type="button" aria-expanded="false" aria-controls="more">
                <svg class="size-[18px] fill-none stroke-current stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round] transition-transform duration-350 group-aria-expanded:rotate-180 motion-reduce:transition-none" viewBox="0 0 24 24" aria-hidden="true">
                    <path d="M6 9l6 6 6-6"/>
                </svg>
            </button>
        </div>

        <div class="more peer grid grid-rows-[0fr] transition-[grid-template-rows] duration-500 motion-reduce:transition-none [&.open]:grid-rows-[1fr]" id="more">
            <div class="more-inner min-h-0 overflow-hidden" inert="">
                <div class="border-b border-line">
                    <div class="wrap">
                        <p class="py-[clamp(48px,8vw,96px)] text-stats font-light tracking-stats [&_span]:inline-block max-md:[&_span]:block" inner_html=t("stats") data-swipe></p>
                    </div>
                </div>

                <section class="section-y" id="reality">
                    <div class="wrap">
                        <div class="grid grid-cols-[minmax(0,5fr)_minmax(0,7fr)] items-start gap-[clamp(24px,5vw,80px)] max-xl:grid-cols-1">
                            <h2 class="max-w-[16ch] text-h2 font-light tracking-title" inner_html=t("realityH") data-swipe></h2>
                            <p class="max-w-[46ch] text-big tracking-[-.01em]" inner_html=t("realityP") data-swipe></p>
                        </div>
                        <figure class="mt-[clamp(64px,9vw,120px)] border-t border-line pt-[clamp(40px,6vw,64px)]">
                            <blockquote>
                                <p class="max-w-[22ch] font-serif text-ayah font-light tracking-[-.015em] italic" inner_html=t("ayah") data-swipe></p>
                            </blockquote>
                            <figcaption class="mt-6 flex flex-wrap gap-x-6 gap-y-1.5 text-[15px] text-muted">
                                <span>"QS Ar-Ra’d 13:28"</span>
                                <span class="[&_a]:text-fg [&_a]:underline [&_a]:underline-offset-3" inner_html=t("ayahNote") data-swipe></span>
                            </figcaption>
                        </figure>
                    </div>
                </section>

                <section class="section-y border-t border-line" id="plan">
                    <div class="wrap">
                        <h2 class="max-w-[16ch] text-h2 font-light tracking-title" inner_html=t("planH") data-swipe></h2>
                        <p class="mt-6 max-w-[56ch] text-lede text-muted" inner_html=t("planLede") data-swipe></p>
                        <div class="mt-[clamp(40px,6vw,72px)] grid grid-cols-4 gap-3 max-xl:grid-cols-2 max-md:grid-cols-1">
                            <RuleCard state=state n=1 h="r1h" p="r1p" s="r1s" icon=swords/>
                            <RuleCard state=state n=2 h="r2h" p="r2p" s="r2s" icon=handshake/>
                            <RuleCard state=state n=3 h="r3h" p="r3p" s="r3s" icon=full_moon/>
                            <RuleCard state=state n=4 h="r4h" p="r4p" s="r4s" icon=trending_down/>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}

// ───────────── Schedule ─────────────

const ARROW: &str = "grid size-10 place-items-center rounded-full border border-line-strong text-fg transition-[border-color,opacity] duration-200 hover:border-fg disabled:cursor-default disabled:opacity-35 disabled:hover:border-line-strong";

#[component]
fn Schedule(state: State) -> impl IntoView {
    let t = move |k| state.t(k);
    let days = build_days();
    view! {
        <section class="section-y peer-[.open]:border-t peer-[.open]:border-line" id="schedule">
            <div class="wrap">
                <h2 class="max-w-[16ch] text-h2 font-light tracking-title" inner_html=t("schedH") data-swipe></h2>
                <p class="mt-6 max-w-[56ch] text-lede text-muted" inner_html=t("schedLede") data-swipe></p>

                <div class="mt-10 grid grid-cols-[auto_minmax(0,1fr)_auto] items-center gap-x-7 gap-y-3 border-t border-line py-6 max-md:grid-cols-[auto_minmax(0,1fr)]" id="progress-head">
                    <ProgressHead state=state/>
                </div>

                // Day carousel: 1 card on phones, 3 on tablet/desktop. Swipe, or use the arrows (carousel.rs).
                <div class="mt-4 flex items-center justify-between gap-4">
                    <p class="text-[14px] text-muted tabular-nums" id="carousel-pos" aria-live="polite"></p>
                    <div class="flex gap-2">
                        <button class=ARROW id="carousel-prev" type="button">
                            <svg class="size-[18px] fill-none stroke-current stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 24 24" aria-hidden="true"><path d="M15 6l-6 6 6 6"/></svg>
                        </button>
                        <button class=ARROW id="carousel-next" type="button">
                            <svg class="size-[18px] fill-none stroke-current stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 24 24" aria-hidden="true"><path d="M9 6l6 6-6 6"/></svg>
                        </button>
                    </div>
                </div>
                // pt-5 leaves room for the "don't skip" bubble; px-1/-mx-1 for the flash outline
                <div class="relative -mx-1 flex snap-x snap-mandatory scroll-px-1 gap-3 overflow-x-auto overscroll-x-contain px-1 pt-5 pb-2 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
                    id="grid" data-swipe role="region" aria-roledescription="carousel" tabindex="-1">
                    {days.into_iter().enumerate().map(|(i, day)| view! {
                        <DayCard n={i as u8 + 1} day=day state=state/>
                    }).collect_view()}
                </div>
                <p class="mt-6 text-[14px] text-muted" inner_html=t("note") data-swipe></p>
            </div>
        </section>
    }
}

// ───────────── Excuses ─────────────

#[component]
fn Excuse(state: State, q: &'static str, a: &'static str) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 items-baseline gap-x-[clamp(24px,5vw,80px)] gap-y-2 border-b border-line py-[30px] max-lg:grid-cols-1">
            <q class="text-quote tracking-snug text-muted" inner_html=state.t(q) data-swipe></q>
            <p class="font-serif text-answer italic" inner_html=state.t(a) data-swipe></p>
        </div>
    }
}

#[component]
fn Excuses(state: State) -> impl IntoView {
    view! {
        <section class="section-y border-t border-line" id="excuses">
            <div class="wrap">
                <h2 class="max-w-[16ch] text-h2 font-light tracking-title" inner_html=state.t("excH") data-swipe></h2>
                <div class="mt-[clamp(40px,6vw,72px)] border-t border-line">
                    <Excuse state=state q="e1q" a="e1a"/>
                    <Excuse state=state q="e2q" a="e2a"/>
                    <Excuse state=state q="e3q" a="e3a"/>
                    <Excuse state=state q="e4q" a="e4a"/>
                    <Excuse state=state q="e5q" a="e5a"/>
                    <Excuse state=state q="e6q" a="e6a"/>
                </div>
            </div>
        </section>
    }
}

// ───────────── Closing ─────────────

#[component]
fn Closing(state: State) -> impl IntoView {
    let t = move |k| state.t(k);
    // "Invite a friend": the label shows "Copied" / "Couldn't copy" for a moment (extras.rs sets `shared`)
    let share_label = move || match state.shared.get() {
        None => t("shareCta")(),
        Some(true) => t("shareDone")(),
        Some(false) => t("shareFail")(),
    };
    view! {
        <section class="border-t border-line py-[clamp(96px,14vw,184px)] text-center" id="start">
            <div class="wrap">
                <svg class="mx-auto mb-8 block size-14" viewBox="0 0 40 40" aria-hidden="true"><circle class="fill-moon-lit" cx="20" cy="20" r="18"/></svg>
                <h2 class="mx-auto text-final font-light tracking-display" inner_html=t("finalH") data-swipe></h2>
                <p class="mx-auto mt-6 mb-10 max-w-[56ch] text-lede text-muted" inner_html=t("finalLede") data-swipe></p>
                <div class="flex flex-wrap justify-center gap-3">
                    <a class="btn btn-primary" href="#schedule" data-jump="1" inner_html=t("finalCta") data-swipe></a>
                    <button class="btn btn-ghost [&.copied]:border-fg" class:copied=move || state.shared.get() == Some(true)
                        id="share" type="button" data-swipe on:click=move |_| crate::extras::share(state)>
                        <svg class="mr-2 size-[18px] flex-none fill-none stroke-current stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round]" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M10 14a4.5 4.5 0 0 0 6.4 0l3.2-3.2a4.5 4.5 0 0 0-6.4-6.4L12 5.6"/>
                            <path d="M14 10a4.5 4.5 0 0 0-6.4 0l-3.2 3.2a4.5 4.5 0 0 0 6.4 6.4l1.2-1.2"/>
                        </svg>
                        <span id="share-label" inner_html=share_label></span>
                    </button>
                </div>
                <p class="sr-only" id="share-live" aria-live="polite">
                    {move || state.shared.get().map(|_| share_label()).unwrap_or_default()}
                </p>
                <p class="mt-[18px] text-[14px] text-muted" inner_html=t("finalMicro") data-swipe></p>
            </div>
        </section>
    }
}

// ───────────── Support (Saweria) ─────────────

#[component]
fn Support(state: State) -> impl IntoView {
    let t = move |k| state.t(k);
    let qr = crate::extras::qr_svg(crate::extras::SAWERIA_URL);
    let no_qr = qr.is_none(); // (only if the link were too long for a QR code)
    view! {
        <section class="border-t border-line py-[clamp(80px,11vw,140px)]" id="support">
            <div class="wrap grid grid-cols-[minmax(0,1fr)_minmax(0,300px)] items-center gap-[clamp(32px,6vw,96px)] max-lg:grid-cols-1">
                <div>
                    <h2 class="max-w-[16ch] text-h2 font-light tracking-title" inner_html=t("supportH") data-swipe></h2>
                    <p class="mt-6 mb-8 max-w-[56ch] text-lede text-muted" inner_html=t("supportP") data-swipe></p>
                    <a class="btn btn-primary" id="saweria-link" href=crate::extras::SAWERIA_URL target="_blank" rel="noopener"
                        inner_html=t("supportCta") data-swipe></a>
                    <p class="mt-4 text-[14px] text-muted" inner_html=t("supportNote") data-swipe></p>
                </div>
                <div class="qr-card grid aspect-square w-[min(300px,100%)] max-w-full place-items-center justify-self-end rounded-3xl border border-line-strong bg-qr p-3.5 max-lg:justify-self-center"
                    hidden=no_qr>
                    <div id="saweria-qr" role="img" aria-label="QR code Saweria" inner_html=qr.unwrap_or_default()></div>
                </div>
            </div>
        </section>
    }
}

// ───────────── Footer ─────────────

#[component]
fn Footer(state: State) -> impl IntoView {
    view! {
        <footer class="border-t border-line pt-10 pb-[calc(40px+env(safe-area-inset-bottom,0px))] text-[14px] text-muted">
            <div class="wrap flex flex-wrap items-center justify-between gap-x-10 gap-y-4">
                <Logo class="inline-flex items-center gap-2.5 text-[18px] font-medium tracking-snug text-fg"/>
                <p class="ml-auto max-w-[60ch] text-right max-md:ml-0 max-md:text-left" inner_html=state.t("footer") data-swipe></p>
            </div>
        </footer>
    }
}
