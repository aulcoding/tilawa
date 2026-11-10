// Entry point: fetches the page text, loads the Rust/WebAssembly app, starts it.
// Everything else (cards, header, language, theme, hero, carousel, extras) is in
// crates/tilawa-app. js/boot.js runs earlier, in <head>, so the saved theme applies before paint.
import init, { start } from "../pkg/tilawa_app.js";

const text = (path) => fetch(path).then((r) => (r.ok ? r.text() : ""));

try {
  const [en, id] = await Promise.all([text("lang/en.jsonc"), text("lang/id.jsonc"), init()]);
  start(en, id);
} catch (err) {
  console.error("Tilawa: the Rust/WASM app failed to load. Serve the folder (node dev.js), don’t open the file directly.", err);
}
