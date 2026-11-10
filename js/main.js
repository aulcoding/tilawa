// Entry point. Loads the Rust/WebAssembly app (Leptos), then starts every part of the page.
//
//   Rust (crates/tilawa-app)  day cards, progress header, EN/ID language, all text
//   js/state.js     shared: text copy for JS, language hooks, storage, small helpers
//   js/theme.js     light/dark switch
//   js/hero.js      tappable moon, breathe in/out, "I'll start tomorrow"
//   js/schedule.js  carousel around the day cards + "don't skip" bubble
//   js/extras.js    accordion, copy link, Saweria
import init, { App } from "../pkg/tilawa_app.js";
import { T, app as state, runLangHooks } from "./state.js";
import { initTheme } from "./theme.js";
import { initHero } from "./hero.js";
import { initSchedule, onEvent } from "./schedule.js";
import { initExtras, openFor } from "./extras.js";

const text = (path) => fetch(path).then((r) => (r.ok ? r.text() : ""));

try {
  const [en, id] = await Promise.all([text("lang/en.jsonc"), text("lang/id.jsonc"), init()]);
  const app = new App(document.getElementById("grid"), document.getElementById("progress-head"), en, id,
    onEvent, (lang) => { state.lang = lang; runLangHooks(); });
  T.en = JSON.parse(app.texts_json("en"));
  T.id = JSON.parse(app.texts_json("id"));
  state.lang = app.lang();

  initTheme();
  initHero();
  initExtras();
  initSchedule(app);
  app.start();            // last: filling the text runs every module's language hook
  openFor(location.hash); // a link like …/#plan opens the folded section
} catch (err) {
  console.error("Tilawa: the Rust/WASM core failed to load. Serve the folder (node dev.js), don’t open the file directly.", err);
}
