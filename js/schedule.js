// The 29-day schedule: progress header, the day cards, and taps on them.
// The rules (what a tap may do) are decided by the Rust core (crates/tilawa-core);
// this file draws the result. Phase 2 of the Rust migration replaces it with Leptos.
import { t, reduce, KEYS, store, onLang, swipeExtras, replay, pick } from "./state.js";

let core;          // Tilawa (Rust/WASM)
let DAYS = [];     // [{ s: surah names, r: [first, last] surah, tag?, pg: [first, last] page }]
let pages = [];    // pages read per day, a read-only mirror of the core
let done = new Set();
let cheerText = {};   // the line currently shown on each unfinished card
let topLine = "";     // the line next to the big progress number

const grid = document.getElementById("grid");
const countEl = document.getElementById("count");
const msgEl = document.getElementById("msg");
const barEl = document.getElementById("bar");
const resetBtn = document.getElementById("reset");

const total = (i) => DAYS[i].pg[1] - DAYS[i].pg[0] + 1;
const sync = () => { pages = Array.from(core.pages()); };

function save() {
  store.set(KEYS.pages, core.to_json());
  store.set(KEYS.done, core.finished_days_json());   // kept for older versions
}

// ── Card moon ──
// Phase per day: a visible crescent on Day 1, full on Day 15, a last sliver on Day 29.
// The lit part has two layers: a dim one, and a bright one behind a clip that rises
// from the bottom to the top when the day is finished (styles: .day-moon in styles/app.css).
function moonSVG(n) {
  const p = n <= 15 ? 0.08 + (n - 1) * 0.03 : 0.5 + (n - 15) * 0.03;
  const r = 18, c = 20;
  let shape;
  if (Math.abs(p - 0.5) < 0.001) {
    shape = (cls) => `<circle class="${cls}" cx="${c}" cy="${c}" r="${r}"/>`;
  } else {
    const rx = (r * Math.abs(Math.cos(2 * Math.PI * p))).toFixed(2);
    const wax = p < 0.5;
    const outer = wax ? 1 : 0;
    const inner = wax ? (p < 0.25 ? 0 : 1) : (p > 0.75 ? 1 : 0);
    const d = `M${c} ${c - r}A${r} ${r} 0 0 ${outer} ${c} ${c + r}A${rx} ${r} 0 0 ${inner} ${c} ${c - r}Z`;
    shape = (cls) => `<path class="${cls}" d="${d}"/>`;
  }
  return `<svg class="day-moon size-[38px] flex-none" viewBox="0 0 40 40" aria-hidden="true">`
    + `<defs><clipPath id="rise-${n}"><rect class="rise" x="0" y="0" width="40" height="40"/></clipPath></defs>`
    + `<circle class="disk" cx="${c}" cy="${c}" r="${r}"/>`
    + shape("lit-base")
    + `<g clip-path="url(#rise-${n})">${shape("lit-on")}</g>`
    + `</svg>`;
}

// ── Card markup ──
// State classes set by update(): is-done, is-next, locked, has-status, flash, just-done.
const CARD = [
  "day group/day relative flex min-h-[204px] flex-col gap-3 rounded-card border border-line bg-surface px-[18px] pt-[18px] pb-3.5 text-left",
  "transition-[border-color,background-color] duration-250 [-webkit-tap-highlight-color:transparent]",
  "hover:border-line-strong [&.is-next]:border-line-strong",
  "[&.is-done]:border-line-strong [&.is-done]:bg-surface-done",
  "[&.locked]:opacity-50 [&.locked]:hover:border-line [&.flash]:opacity-100!",
  "max-sm:min-w-0",
].join(" ");
const STEP = "day-step grid size-8 place-items-center rounded-full border border-line-strong bg-bg text-[18px] leading-none text-fg "
  + "transition-[border-color,opacity,transform] duration-200 hover:border-fg active:scale-90 "
  + "aria-disabled:opacity-35 aria-disabled:hover:border-line-strong group-[.locked]/day:opacity-35";

function cardHTML(d, i) {
  const n = i + 1, tt = t();
  const range = d.r[0] === d.r[1] ? tt.surah + d.r[0] : tt.surahs + d.r[0] + "–" + d.r[1];
  const surahs = d.s.map((x) => `<span>${x.split(" – ").map((y) => `<span class="whitespace-nowrap">${y}</span>`).join(" – ")}</span>`).join("");
  return `<article class="${CARD}" id="day-${n}" data-day="${n}">
    <button class="day-hit absolute inset-0 z-0 rounded-[inherit] focus-visible:outline-offset-2" type="button"></button>
    <div class="flex items-start justify-between gap-2">
      <span class="text-[14px] leading-[1.2] text-muted">${tt.day}<b class="mt-1 block text-[34px] leading-none font-light tracking-stats text-fg tabular-nums">${n}</b></span>
      ${moonSVG(n)}
    </div>
    ${d.tag ? `<span class="self-start rounded-xl border border-line-strong px-2.5 py-[5px] text-[12.5px] leading-[1.3]">${(tt.tags || {})[d.tag] || ""}</span>` : ""}
    <div class="mt-auto text-[17px] leading-[1.3] tracking-[-.01em] [&>span]:block">${surahs}</div>
    <p class="day-status line-clamp-2 h-[2.7em] text-[13px] leading-[1.35] text-muted group-[.is-done]/day:text-fg" aria-live="polite"></p>
    <div class="flex flex-col gap-2">
      <div class="day-bar h-[3px] overflow-hidden rounded-[3px] bg-line"><i class="block h-full w-(--fill,0%) rounded-[inherit] bg-fg transition-[width] duration-350 ease-soft motion-reduce:transition-none"></i></div>
      <div class="flex items-center justify-between gap-2 max-sm:flex-wrap">
        <span class="day-count text-[13px] whitespace-nowrap text-muted tabular-nums"><b class="font-medium text-fg">0</b>/${total(i)} ${tt.pagesUnit}</span>
        <span class="day-ctrl inline-flex gap-1.5">
          <button class="${STEP}" type="button" data-step="-1" aria-label="${tt.pageMinus}">−</button>
          <button class="${STEP}" type="button" data-step="1" aria-label="${tt.pagePlus}">+</button>
        </span>
      </div>
    </div>
    <div class="flex justify-between gap-2 text-[12.5px] text-muted max-sm:flex-col [&>span]:whitespace-nowrap">
      <span>${range}</span><span>${tt.pagesShort} ${d.pg[0]}–${d.pg[1]}</span>
    </div>
  </article>`;
}

function renderGrid() {
  grid.innerHTML = DAYS.map(cardHTML).join("");
}

// ── Progress header line ──
// Changes only when a day is finished: warm lines while on Day 1, lightly sarcastic from Day 2 on.
function rollTop() {
  const n = done.size;
  if (n >= 29 || (n === 0 && pages[0] === 0)) { topLine = ""; return; }
  topLine = pick(n === 0 ? t().progressDay1 : t().progress, topLine);
}
function message(n) {
  const m = t().msg;
  if (n === 0 && pages[0] === 0) return m[0];
  if (n >= 29) return m[3];
  return topLine || (n < 15 ? m[1] : m[2]);
}

/** Calming line on a card, picked on every tap. {left} = pages left that day. */
function cheer(d) {
  const left = total(d - 1) - pages[d - 1];
  const list = (t().cheer || []).map((x) => x.replace("{left}", left));
  return pick(list, cheerText[d]);
}

/** Redraws the header and every card from the current progress (no re-render, so the moons can animate). */
function update() {
  done = new Set();
  for (let i = 0; i < 29 && pages[i] >= total(i); i++) done.add(i + 1);
  const n = done.size, cur = n + 1, tt = t();
  countEl.textContent = n;
  barEl.style.width = (n / 29) * 100 + "%";
  msgEl.textContent = message(n);
  resetBtn.disabled = pages.every((v) => v === 0);

  grid.querySelectorAll(".day").forEach((card) => {
    const d = +card.dataset.day, i = d - 1, tot = total(i), v = pages[i];
    const isDone = v >= tot;
    card.classList.toggle("is-done", isDone);
    card.classList.toggle("locked", d > cur);
    card.classList.toggle("is-next", d === cur);
    card.style.setProperty("--fill", (v / tot) * 100 + "%");
    card.querySelector(".day-count b").textContent = v;

    const status = isDone ? ((tt.doneDays && tt.doneDays[i]) || tt.done) : (v > 0 ? cheerText[d] || "" : "");
    const st = card.querySelector(".day-status");
    if (st.textContent !== status) st.textContent = status;
    card.classList.toggle("has-status", !!status);

    const canMinus = core.can_remove(d), canPlus = core.can_add(d);   // decided in Rust
    card.querySelector('[data-step="-1"]').setAttribute("aria-disabled", String(!canMinus));
    card.querySelector('[data-step="1"]').setAttribute("aria-disabled", String(!canPlus));
    card.querySelector(".day-hit").setAttribute("aria-label",
      `${tt.day} ${d}: ${v}/${tot} ${tt.pagesUnit}${canPlus ? ". " + tt.pagePlus : ""}`);
  });
}

// ── Feedback when a tap isn't allowed ──
let hintTimer;
/** "Undo from the end: Day N first." in the header, and flashes that day. */
function hint(text, day) {
  clearTimeout(hintTimer);
  msgEl.textContent = text;
  const card = document.getElementById("day-" + day);
  if (card) replay(card, "flash");
  hintTimer = setTimeout(() => { msgEl.textContent = message(done.size); }, 2800);
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
  const r = card.getBoundingClientRect();
  if (r.top < 90 || r.bottom > innerHeight) card.scrollIntoView({ behavior: reduce ? "auto" : "smooth", block: "center" });
  warnTimer = setTimeout(() => {
    w.classList.add("leaving");
    setTimeout(() => w.remove(), reduce ? 0 : 260);
  }, 2600);
}

// ── Taps: the card or "+" = one page read, "−" = one page back ──
function onTap(e) {
  const card = e.target.closest(".day");
  if (!card) return;
  const stepBtn = e.target.closest(".day-step");
  const step = stepBtn ? +stepBtn.dataset.step : 1;
  const d = +card.dataset.day;
  const before = done.size;

  const out = JSON.parse(step > 0 ? core.add_page(d) : core.remove_page(d));   // Rust decides
  if (out.kind === "locked") return warnSkip(out.next);
  if (out.kind === "undofromend") return hint(t().undoLast.replace("{n}", out.last), out.last);
  if (out.kind === "noop") return;

  sync();
  if (out.kind === "finished") {
    delete cheerText[d];
    card.classList.add("just-done");
    setTimeout(() => card.classList.remove("just-done"), 1600);
  } else {
    cheerText[d] = cheer(d);
  }
  clearTimeout(hintTimer);
  save();
  update();
  if (done.size !== before || !topLine) rollTop();   // the header line changes only when a day is (un)finished
  msgEl.textContent = message(done.size);
  const st = card.querySelector(".day-status");      // replay the little rise on every new line
  st.style.animation = "none"; void st.offsetWidth; st.style.animation = "";
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
    core.reset(); sync(); cheerText = {}; save(); update();
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
  card.scrollIntoView({ behavior: reduce ? "auto" : "smooth", block: "center" });
  replay(card, "flash");
  setTimeout(() => (card.querySelector(".day-hit") || card).focus({ preventScroll: true }), 450);
}

export function initSchedule(tilawaCore) {
  core = tilawaCore;
  DAYS = JSON.parse(core.days_json());   // built in Rust from quran.json
  sync();
  grid.addEventListener("click", onTap);
  resetBtn.addEventListener("click", onReset);
  document.addEventListener("click", onJump);
  swipeExtras.push(grid, msgEl, resetBtn);
  onLang(() => {
    disarm();
    renderGrid();
    update();
    rollTop();
    msgEl.textContent = message(done.size);
  });
}
