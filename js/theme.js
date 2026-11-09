// Light / dark: follows the device until the viewer picks one; the pick is remembered.
// (js/boot.js applies the saved pick before the page paints.)
import { t, reduce, KEYS, store, onLang } from "./state.js";

const btn = document.getElementById("theme");
const root = document.documentElement;
const sysDark = window.matchMedia ? window.matchMedia("(prefers-color-scheme: dark)") : null;

function currentTheme() {
  const th = root.dataset.theme;
  return th === "dark" || th === "light" ? th : (sysDark && sysDark.matches ? "dark" : "light");
}

function label() {
  const text = currentTheme() === "dark" ? t().themeToLight : t().themeToDark;
  btn.setAttribute("aria-label", text);
  btn.title = text;
}

export function initTheme() {
  let animTimer;
  btn.addEventListener("click", () => {
    const next = currentTheme() === "dark" ? "light" : "dark";
    if (!reduce) {
      root.classList.add("theme-anim");   // brief colour cross-fade (styles/app.css)
      clearTimeout(animTimer);
      animTimer = setTimeout(() => root.classList.remove("theme-anim"), 450);
    }
    root.dataset.theme = next;
    store.set(KEYS.theme, next);
    document.querySelectorAll('meta[name="theme-color"]').forEach((m) =>
      m.setAttribute("content", next === "dark" ? "#000000" : "#F3F4F6"));
    label();
  });
  if (sysDark && sysDark.addEventListener) sysDark.addEventListener("change", label);
  onLang(label);
}
