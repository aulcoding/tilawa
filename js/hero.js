// Hero: the moon you can tap toward full, "Breathe in / out" around it,
// and the "I'll start tomorrow" button that roasts you.
import { t, reduce, onLang, swipeExtras } from "./state.js";

// ── Tap the moon: each tap grows it one step toward full; after full it goes dark again ──
const MOON_STEPS = 8;   // taps from dark (0) to full (8)
let moonStep = 3;       // starts as a crescent
let moonP = moonStep / (MOON_STEPS * 2);   // 0 = dark, 0.5 = full
let moonAnim;
const lit = document.getElementById("hero-lit");
const moonBtn = document.getElementById("moon-tap");

/** SVG path of the lit part of a 200×200 moon at phase p (0 = new, 0.5 = full). */
function moonPath(p) {
  const r = 90, c = 100;
  if (p <= 0.001) return "";
  if (p >= 0.499) return `M${c} ${c - r}A${r} ${r} 0 1 1 ${c} ${c + r}A${r} ${r} 0 1 1 ${c} ${c - r}Z`;
  const rx = (r * Math.abs(Math.cos(2 * Math.PI * p))).toFixed(2);
  const inner = p < 0.25 ? 0 : 1;
  return `M${c} ${c - r}A${r} ${r} 0 0 1 ${c} ${c + r}A${rx} ${r} 0 0 ${inner} ${c} ${c - r}Z`;
}

function draw(p) { moonP = p; lit.setAttribute("d", moonPath(p)); }

function tweenTo(to) {
  cancelAnimationFrame(moonAnim);
  const from = moonP, dur = reduce ? 0 : 550;
  let t0 = null;
  const frame = (now) => {
    if (t0 === null) t0 = now;
    const k = dur ? Math.min(1, (now - t0) / dur) : 1;
    const e = 1 - Math.pow(1 - k, 3);   // ease-out
    draw(from + (to - from) * e);
    if (k < 1) moonAnim = requestAnimationFrame(frame);
  };
  moonAnim = requestAnimationFrame(frame);
}

function initMoon() {
  draw(moonP);
  moonBtn.addEventListener("click", () => {
    if (moonStep >= MOON_STEPS) {
      // Full moon: fade out, then back to dark
      moonStep = 0;
      cancelAnimationFrame(moonAnim);
      lit.classList.add("fading");
      setTimeout(() => { draw(0); lit.classList.remove("fading"); }, reduce ? 0 : 400);
      return;
    }
    moonStep++;
    tweenTo(moonStep / (MOON_STEPS * 2));
  });
  onLang(() => moonBtn.setAttribute("aria-label", t().moonLabel));
}

// ── Breathe in / out: every 4 s (in step with the moon's 8 s breathing) the label fades out,
//    jumps to a random point just outside the moon, swaps text and fades back in ──
function initBreath() {
  const breath = document.getElementById("breath");
  const box = breath.parentElement;
  const place = () => {
    const w = box.clientWidth, h = box.clientHeight;
    const a = Math.random() * Math.PI * 2;
    const R = 0.53 + Math.random() * 0.07;          // the disk edge is at 0.45 of the box
    let x = w / 2 + Math.cos(a) * w * R;
    const y = h / 2 + Math.sin(a) * h * R;
    const half = breath.offsetWidth / 2, slack = 24; // keep the label near the moon box
    x = Math.max(half - slack, Math.min(w - half + slack, x));
    breath.style.left = x + "px";
    breath.style.top = y + "px";
  };
  const cycle = (phase) => {
    breath.dataset.phase = phase;
    place();
    breath.classList.add("show");
    setTimeout(() => breath.classList.remove("show"), 3000);
    setTimeout(() => cycle(phase === "in" ? "out" : "in"), 4000);
  };
  if (!reduce) cycle("in");
}

// ── "I'll start tomorrow" → "Sure you will." ──
function initLater() {
  const later = document.getElementById("later");
  const live = document.getElementById("later-live");
  let timer;
  later.addEventListener("click", () => {
    later.classList.add("roasted");
    live.textContent = t().later;
    clearTimeout(timer);
    timer = setTimeout(() => { later.classList.remove("roasted"); live.textContent = ""; }, 2400);
  });
  swipeExtras.push(later);
}

export function initHero() {
  initMoon();
  initBreath();
  initLater();
}
