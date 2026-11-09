# Tilawa

A 29-day Qur’an reading plan, split by surah, with page-by-page progress.
EN/ID, light/dark, no account, progress saved in the browser.

## Run locally

```bash
node dev.js          # http://localhost:5500, reloads the browser on every save
```

Open it through the server, not as a file: the browser only loads the WebAssembly core over http.

## Project layout

| Path | What it is |
|---|---|
| `index.html` | The page: layout, styles, UI script |
| `lang/en.js`, `lang/id.js` | All text, one file per language |
| `quran.json` | Start page of every surah (Mushaf Madinah, 604 pages) |
| `crates/tilawa-core/` | Rust: the 29-day plan and progress rules |
| `pkg/` | `tilawa-core` compiled to WebAssembly (generated, committed) |
| `dev.js` | Zero-dependency dev server with auto-reload |

## Rust migration

Moving to Rust step by step.

- [x] **Phase 1** – plan + progress rules in Rust (`tilawa-core`), called from the existing JS through WebAssembly
- [ ] **Phase 2** – schedule grid rendered with Leptos
- [ ] **Phase 3** – progress header, language, theme in Leptos; text as JSON
- [ ] **Phase 4** – full Leptos CSR app built with `trunk`; old JS removed
- [ ] Phase 5 (optional) – Rust API on Vercel, only if a feature needs a server

### Working on the Rust core

```bash
cargo test                                   # run the rules' unit tests (no browser needed)
wasm-pack build crates/tilawa-core --target web --release --out-dir ../../pkg --no-pack
```

Rebuild `pkg/` after changing anything in `crates/`, then refresh the page.
Needs the `wasm32-unknown-unknown` target (Arch: `pacman -S rust-wasm wasm-bindgen wasm-pack`, same version as `rust`).
