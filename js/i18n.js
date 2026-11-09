// Language: fills every [data-i18n] element from lang/*.js, the EN/ID switch,
// and the swipe-up text animation when switching.
import { T, app, t, reduce, KEYS, store, runLangHooks, swipeExtras } from "./state.js";

const nodes = Array.from(document.querySelectorAll("[data-i18n]"));
const langBtn = document.getElementById("lang");

function applyLang() {
  nodes.forEach((el) => {
    const v = t()[el.dataset.i18n];
    if (v != null) el.innerHTML = v;
  });
  document.documentElement.lang = app.lang;
  document.title = t().title;
  langBtn.setAttribute("aria-label", t().langLabel);
  runLangHooks();   // every other module refreshes its own labels / cards
}

// Old text slides up and fades out, new text comes up from below.
// Web Animations, so it also wins over the hero's load-in CSS animation.
const SWIPE_OUT = 170;
let swapTimer = null;
let outAnims = [];

function swipeTargets() {
  const list = nodes.filter((el) => !el.closest(".btn-roast") && !el.closest(".breath") && el.id !== "skip");
  return list.concat(swipeExtras);
}

export function setLang(l, animate) {
  app.lang = l === "id" ? "id" : "en";
  langBtn.dataset.lang = app.lang;   // the moon-fill on the switch starts right away
  if (swapTimer) {
    clearTimeout(swapTimer);
    swapTimer = null;
    outAnims.forEach((a) => a.cancel());
    outAnims = [];
  }
  if (!animate || reduce || !document.body.animate) { applyLang(); return; }

  const els = swipeTargets();
  outAnims = els.map((el) => el.animate(
    [{ opacity: 1, transform: "translateY(0)" }, { opacity: 0, transform: "translateY(-0.5em)" }],
    { duration: SWIPE_OUT, easing: "cubic-bezier(.5,0,.75,0)", fill: "forwards" }));

  swapTimer = setTimeout(() => {
    swapTimer = null;
    applyLang();
    outAnims.forEach((a) => a.cancel());
    outAnims = [];
    els.forEach((el, i) => el.animate(
      [{ opacity: 0, transform: "translateY(0.6em)" }, { opacity: 1, transform: "translateY(0)" }],
      { duration: 380, delay: Math.min(i * 6, 120), easing: "cubic-bezier(.2,.8,.2,1)", fill: "backwards" }));
  }, SWIPE_OUT);
}

// Keep the EN/ID switch perfectly still: the "Start Day 1" button to its right gets a
// fixed width that fits both languages, so its label change can't push the switch.
function lockNavStart() {
  const b = document.getElementById("nav-start");
  const keep = b.innerHTML;
  let w = 0;
  b.style.minWidth = "";
  ["en", "id"].forEach((l) => {
    if (T[l].navStart == null) return;
    b.innerHTML = T[l].navStart;
    w = Math.max(w, b.getBoundingClientRect().width);
  });
  b.innerHTML = keep;
  b.style.minWidth = Math.ceil(w) + "px";
}

export function initI18n() {
  langBtn.addEventListener("click", () => {
    const next = app.lang === "en" ? "id" : "en";
    setLang(next, true);
    store.set(KEYS.lang, next);
  });

  // Saved choice, else the browser language (Indonesian or Malay → ID)
  let saved = store.get(KEYS.lang);
  if (!saved) {
    const nav = (navigator.language || "").toLowerCase();
    saved = nav.startsWith("id") || nav.startsWith("ms") ? "id" : "en";
  }
  setLang(saved);

  lockNavStart();
  if (document.fonts && document.fonts.ready) document.fonts.ready.then(lockNavStart);   // re-measure once the web font is in
  // Turn on the switch animation only after the first paint, so it doesn't slide on load
  requestAnimationFrame(() => requestAnimationFrame(() => langBtn.classList.add("ready")));
}
