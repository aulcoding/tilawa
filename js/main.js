// Entry point. Loads the Rust/WebAssembly core, then starts every part of the page.
//
//   js/state.js     shared: text, language, storage, small helpers
//   js/i18n.js      EN/ID switch + text swipe
//   js/theme.js     light/dark switch
//   js/hero.js      tappable moon, breathe in/out, "I'll start tomorrow"
//   js/schedule.js  progress header + the 29 day cards (Rust decides the rules)
//   js/extras.js    accordion, copy link, Saweria
import init, { Tilawa } from "../pkg/tilawa_core.js";
import { KEYS, store } from "./state.js";
import { initI18n } from "./i18n.js";
import { initTheme } from "./theme.js";
import { initHero } from "./hero.js";
import { initSchedule } from "./schedule.js";
import { initExtras, openFor } from "./extras.js";

try {
  await init();
  const core = Tilawa.restore(store.get(KEYS.pages), store.get(KEYS.done));

  initTheme();
  initHero();
  initExtras();
  initSchedule(core);
  initI18n();             // last: filling the text runs every module's language hook
  openFor(location.hash); // a link like …/#plan opens the folded section
} catch (err) {
  console.error("Tilawa: the Rust/WASM core failed to load. Serve the folder (node dev.js), don’t open the file directly.", err);
}
