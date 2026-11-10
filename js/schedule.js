// The 29-day schedule: progress header, carousel, and feedback bubbles.
//
// Phase 2 of the Rust migration: the day cards themselves are a Leptos component
// (crates/tilawa-app/src/card.rs). Rust renders them, handles taps and "−"/"+",
// applies the rules (crates/tilawa-core), saves progress, and calls onEvent() here
// after every tap. This file draws everything around the cards.
import { t, reduce, onLang, swipeExtras, replay, pick } from "./state.js";

let app;              // ScheduleApp (Rust/WASM)
let topLine = "";     // the line next to the big progress number

const grid = document.getElementById("grid");
const countEl = document.getElementById("count");
const msgEl = document.getElementById("msg");
const barEl = document.getElementById("bar");
const resetBtn = document.getElementById("reset");

// ── Carousel (1 card per view on phones, 3 on tablet/desktop) ──
const posEl = document.getElementById("carousel-pos");
const prevBtn = document.getElementById("carousel-prev");
const nextBtn = document.getElementById("carousel-next");
const GAP = 12;   // matches gap-3 on #grid

const slideWidth = () => { const c = grid.querySelector(".day"); return c ? c.offsetWidth + GAP : grid.clientWidth; };
const perView = () => Math.max(1, Math.round((grid.clientWidth + GAP) / slideWidth()));
/** Day number of the first card in view (1-based). */
function firstVisible() {
  if (!grid.firstElementChild) return 1;
  return Math.min(29, Math.max(1, Math.round(grid.scrollLeft / slideWidth()) + 1));
}
/** Scrolls so `day` is the first card in view (clamped so the last view is still full). */
function goTo(day, smooth = true) {
  const last = 29 - perView() + 1;
  const d = Math.min(Math.max(1, day), last);
  grid.scrollTo({ left: (d - 1) * slideWidth(), behavior: smooth && !reduce ? "smooth" : "instant" });
  if (!smooth) updateNav();
}
/** Brings `day` into view only if it isn't already. */
function reveal(day, smooth = true) {
  const first = firstVisible(), n = perView();
  if (day < first) goTo(day, smooth);
  else if (day > first + n - 1) goTo(day - n + 1, smooth);
}
function updateNav() {
  const first = firstVisible(), n = perView(), last = Math.min(29, first + n - 1), tt = t();
  posEl.textContent = (n === 1 ? tt.carouselOne : tt.carouselMany)
    .replace("{from}", first).replace("{to}", last);
  prevBtn.disabled = first <= 1;
  nextBtn.disabled = last >= 29;
}

function initCarousel() {
  prevBtn.addEventListener("click", () => goTo(firstVisible() - perView()));
  nextBtn.addEventListener("click", () => goTo(firstVisible() + perView()));
  let raf;
  grid.addEventListener("scroll", () => { cancelAnimationFrame(raf); raf = requestAnimationFrame(updateNav); }, { passive: true });
  // Arrow keys while focus is inside the carousel
  grid.addEventListener("keydown", (e) => {
    if (e.key === "ArrowRight") { e.preventDefault(); goTo(firstVisible() + 1); }
    if (e.key === "ArrowLeft") { e.preventDefault(); goTo(firstVisible() - 1); }
  });
  // Keep the same first card when the layout changes (rotate phone, resize window)
  let keep = 1, rt;
  window.addEventListener("resize", () => {
    if (!rt) keep = firstVisible();
    clearTimeout(rt);
    rt = setTimeout(() => { rt = null; goTo(keep, false); }, 120);
  });
}

// ── Progress header ──
// The line changes only when a day is finished: warm lines while on Day 1, lightly sarcastic from Day 2 on.
function rollTop() {
  const n = app.finished_count();
  if (n >= 29 || (n === 0 && app.pages_on(1) === 0)) { topLine = ""; return; }
  topLine = pick(n === 0 ? t().progressDay1 : t().progress, topLine);
}
function message(n) {
  const m = t().msg;
  if (n === 0 && app.pages_on(1) === 0) return m[0];
  if (n >= 29) return m[3];
  return topLine || (n < 15 ? m[1] : m[2]);
}
function updateHeader() {
  const n = app.finished_count();
  countEl.textContent = n;
  barEl.style.width = (n / 29) * 100 + "%";
  msgEl.textContent = message(n);
  resetBtn.disabled = app.is_empty();
}

// ── Feedback when a tap isn't allowed ──
let hintTimer;
/** "Undo from the end: Day N first." in the header, and flashes that day. */
function hint(text, day) {
  clearTimeout(hintTimer);
  msgEl.textContent = text;
  const card = document.getElementById("day-" + day);
  if (card) { replay(card, "flash"); reveal(day); }
  hintTimer = setTimeout(() => { msgEl.textContent = message(app.finished_count()); }, 2800);
}

let warnTimer;
/** "Don't skip this surah" bubble on the day still waiting. */
function warnSkip(day) {
  const card = document.getElementById("day-" + day);
  if (!card) return;
  clearTimeout(warnTimer);
  grid.querySelectorAll(".day-warn").forEach((w) => w.remove());
  const w = document.createElement("span");
  w.className = "day-warn";
  w.setAttribute("role", "alert");
  w.textContent = t().skipWarn;
  card.appendChild(w);
  replay(card, "flash");
  reveal(day);
  const r = card.getBoundingClientRect();
  if (r.top < 90 || r.bottom > innerHeight) card.scrollIntoView({ behavior: reduce ? "auto" : "smooth", block: "center", inline: "nearest" });
  warnTimer = setTimeout(() => {
    w.classList.add("leaving");
    setTimeout(() => w.remove(), reduce ? 0 : 260);
  }, 2600);
}

// ── What Rust tells us after every tap ──
// ev = { kind: "added"|"finished"|"removed"|"noop"|"locked"|"undofromend", day, finished, firstDayPages, next?, last? }
let lastFinished = 0;
function onEvent(json) {
  const ev = JSON.parse(json);
  switch (ev.kind) {
    case "locked": return warnSkip(ev.next);
    case "undofromend": return hint(t().undoLast.replace("{n}", ev.last), ev.last);
    case "noop": return;
    case "finished": {
      const card = document.getElementById("day-" + ev.day);
      card.classList.add("just-done");
      setTimeout(() => card.classList.remove("just-done"), 1600);
      if (ev.day < 29) setTimeout(() => reveal(ev.day + 1), 900);   // after the moon fills, slide to the next day
    }
  }
  clearTimeout(hintTimer);
  if (ev.finished !== lastFinished || !topLine) rollTop();   // the header line changes only when a day is (un)finished
  lastFinished = ev.finished;
  updateHeader();
}

// ── Two-tap reset, no dialog needed ──
let armTimer;
function disarm() {
  clearTimeout(armTimer);
  resetBtn.classList.remove("armed");
  resetBtn.textContent = t().reset;
}
function onReset() {
  if (resetBtn.classList.contains("armed")) {
    disarm();
    app.reset();
    lastFinished = 0;
    rollTop();
    updateHeader();
    goTo(1);
    return;
  }
  resetBtn.classList.add("armed");
  resetBtn.textContent = t().resetArmed;
  armTimer = setTimeout(disarm, 3000);
}

// ── Links to a specific day ("Day 9", "Start Day 1") ──
function onJump(e) {
  const a = e.target.closest("[data-jump]");
  if (!a) return;
  const card = document.getElementById("day-" + a.dataset.jump);
  if (!card) return;
  e.preventDefault();
  goTo(+a.dataset.jump, false);
  card.scrollIntoView({ behavior: reduce ? "auto" : "smooth", block: "center", inline: "nearest" });
  replay(card, "flash");
  setTimeout(() => (card.querySelector(".day-hit") || card).focus({ preventScroll: true }), 450);
}

/** `ScheduleApp` is the class exported by the Rust crate (pkg/tilawa_app.js). */
export function initSchedule(ScheduleApp) {
  // Rust renders the 29 cards into #grid right away (in the default language;
  // the language hook below sends the real one).
  app = new ScheduleApp(grid, JSON.stringify(t()), onEvent);
  lastFinished = app.finished_count();

  initCarousel();
  resetBtn.addEventListener("click", onReset);
  document.addEventListener("click", onJump);
  swipeExtras.push(grid, msgEl, resetBtn);

  let first = true;
  onLang(() => {
    app.set_texts(JSON.stringify(t()));   // cards update their labels in place
    disarm();
    grid.setAttribute("aria-label", t().schedH);
    prevBtn.setAttribute("aria-label", t().carouselPrev);
    nextBtn.setAttribute("aria-label", t().carouselNext);
    if (first) { first = false; goTo(app.finished_count() + 1, false); }   // open on the day you're reading
    rollTop();
    updateHeader();
    updateNav();
  });
}
