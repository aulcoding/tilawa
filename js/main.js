// Entry point. Loads the Rust/WebAssembly app (Leptos), then starts every part of the page.
//
//   js/state.js     shared: text, language, storage, small helpers
//   js/i18n.js      EN/ID switch + text swipe
//   js/theme.js     light/dark switch
//   js/hero.js      tappable moon, breathe in/out, "I'll start tomorrow"
//   js/schedule.js  progress header + carousel around the day cards (the cards are Leptos/Rust)
//   js/extras.js    accordion, copy link, Saweria
import init, { ScheduleApp } from "../pkg/tilawa_app.js";
import { initI18n } from "./i18n.js";
import { initTheme } from "./theme.js";
import { initHero } from "./hero.js";
import { initSchedule } from "./schedule.js";
import { initExtras, openFor } from "./extras.js";

try {
  await init();
  initTheme();
  initHero();
  initExtras();
  initSchedule(ScheduleApp);   // Leptos renders the day cards (crates/tilawa-app)
  initI18n();             // last: filling the text runs every module's language hook
  openFor(location.hash); // a link like …/#plan opens the folded section
} catch (err) {
  console.error("Tilawa: the Rust/WASM core failed to load. Serve the folder (node dev.js), don’t open the file directly.", err);
}
