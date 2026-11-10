// Shared bits every module uses: page text, current language, storage, motion setting.

/** All page text: T.en and T.id (from lang/*.jsonc, parsed by Rust; Indonesian already falls back to English). */
export const T = { en: {}, id: {} };

/** Current language ("en" | "id"). Rust owns it; main.js keeps this copy in sync. */
export const app = { lang: "en" };

/** Text for the current language. */
export const t = () => T[app.lang];

/** True when the viewer asked their device for less motion. */
export const reduce = !!(window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches);

/** localStorage keys. */
export const KEYS = {
  pages: "tilawa-pages-v1",     // pages read per day: [49, 5, 0, ...]
  done: "tilawa-progress-v1",   // older format: finished day numbers, still written for compatibility
  theme: "tilawa-theme",
};

/** localStorage that never throws (private windows, blocked storage...). */
export const store = {
  get(k) { try { return localStorage.getItem(k); } catch (e) { return null; } },
  set(k, v) { try { localStorage.setItem(k, v); } catch (e) {} },
};

/** Replays a CSS animation class on an element (e.g. the "flash" outline). */
export function replay(el, cls) {
  el.classList.remove(cls);
  void el.offsetWidth;
  el.classList.add(cls);
}

/** Picks a random item, avoiding `avoid` when there is a choice. */
export function pick(list, avoid) {
  if (!list || !list.length) return "";
  let p;
  do { p = list[Math.floor(Math.random() * list.length)]; } while (list.length > 1 && p === avoid);
  return p;
}

// ── Language change hooks ──
// Modules register a function here; Rust calls them all (through main.js) whenever the language changes.
const langHooks = [];
export function onLang(fn) { langHooks.push(fn); }
export function runLangHooks() { langHooks.forEach((fn) => fn()); }
