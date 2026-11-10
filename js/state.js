// Shared bits every module uses: page text, current language, motion setting.
// (localStorage is all Rust now: progress, language and theme.)

/** All page text: T.en and T.id (from lang/*.jsonc, parsed by Rust; Indonesian already falls back to English). */
export const T = { en: {}, id: {} };

/** Current language ("en" | "id"). Rust owns it; main.js keeps this copy in sync. */
export const app = { lang: "en" };

/** Text for the current language. */
export const t = () => T[app.lang];

/** True when the viewer asked their device for less motion. */
export const reduce = !!(window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches);

/** Replays a CSS animation class on an element (e.g. the "flash" outline). */
export function replay(el, cls) {
  el.classList.remove(cls);
  void el.offsetWidth;
  el.classList.add(cls);
}

// ── Language change hooks ──
// Modules register a function here; Rust calls them all (through main.js) whenever the language changes.
const langHooks = [];
export function onLang(fn) { langHooks.push(fn); }
export function runLangHooks() { langHooks.forEach((fn) => fn()); }
