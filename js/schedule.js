// The 29-day schedule: carousel and the "don't skip" bubble.
//
// Phase 3 of the Rust migration: the day cards and the progress header above them
// are Leptos components (crates/tilawa-app/src/card.rs, header.rs). Rust renders
// them, handles taps, "−"/"+" and the two-tap reset, applies the rules
// (crates/tilawa-core), saves progress, and calls onEvent() here afterwards.
// This file moves the carousel and shows the bubble.
import { t, reduce, onLang, replay } from "./state.js";

let app;              // App (Rust/WASM)

const grid = document.getElementById("grid");

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

// ── "Undo from the end" ──
// Rust shows the hint text in the header; here we just point at that day.
function pointAt(day) {
  const card = document.getElementById("day-" + day);
  if (card) { replay(card, "flash"); reveal(day); }
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

// ── What Rust tells us after every tap / reset ──
// ev = { kind: "added"|"finished"|"removed"|"noop"|"locked"|"undofromend"|"reset", day, finished, firstDayPages, next?, last? }
export function onEvent(json) {
  const ev = JSON.parse(json);
  switch (ev.kind) {
    case "locked": return warnSkip(ev.next);
    case "undofromend": return pointAt(ev.last);
    case "reset": return goTo(1);
    case "finished": {
      const card = document.getElementById("day-" + ev.day);
      card.classList.add("just-done");
      setTimeout(() => card.classList.remove("just-done"), 1600);
      if (ev.day < 29) setTimeout(() => reveal(ev.day + 1), 900);   // after the moon fills, slide to the next day
    }
  }
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

/** `rustApp` is the App from the Rust crate (pkg/tilawa_app.js); the cards are already in #grid. */
export function initSchedule(rustApp) {
  app = rustApp;
  initCarousel();
  document.addEventListener("click", onJump);

  let first = true;
  onLang(() => {
    grid.setAttribute("aria-label", t().schedH);
    prevBtn.setAttribute("aria-label", t().carouselPrev);
    nextBtn.setAttribute("aria-label", t().carouselNext);
    if (first) { first = false; goTo(app.finished_count() + 1, false); }   // open on the day you're reading
    updateNav();
  });
}
