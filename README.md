# Tilawa

A 29-day Qur’an reading plan, split by surah, with page-by-page progress.
EN/ID, light/dark, no account, progress saved in the browser.

Written in Rust ([Leptos](https://leptos.dev) + web-sys), compiled to WebAssembly with
[trunk](https://trunkrs.dev). Live at https://aulcoding.github.io/tilawa/.

## Run locally

```bash
# once (Arch): Rust's WebAssembly target, trunk, wasm-opt, and Tailwind
sudo pacman -S rust-wasm trunk binaryen
npm install

trunk serve      # http://localhost:5500 (also on your Wi-Fi IP) · rebuilds + reloads on every save
cargo test       # the Rust unit tests, no browser needed
trunk build --release   # what the deploy runs → dist/
```

Text lives in `lang/*.jsonc`: edit, save, and `trunk serve` reloads the page.

## Deploy

Every push to `main` runs `.github/workflows/pages.yml`: `cargo test`, then
`trunk build --release` (Rust → WebAssembly, `wasm-opt -Oz`, Tailwind), and publishes
`dist/` to GitHub Pages. Nothing generated is committed.

## Project layout

| Path | What it is |
|---|---|
| `index.html` | Just `<head>` (meta, fonts, `boot.js`) + `data-trunk` links telling trunk what to build/copy |
| `Trunk.toml` | Build/serve settings and the Tailwind hook |
| `styles/app.css` | Tailwind source: colour tokens (light/dark), fluid type, breakpoints, animated bits |
| `js/boot.js` | The only JavaScript: applies the saved theme before the page paints (no dark-mode flash) |
| `lang/en.jsonc`, `lang/id.jsonc` | All text, one file per language (JSON + `//` comments) |
| `quran.json` | Start page of every surah (Mushaf Madinah, 604 pages), baked into the WebAssembly |
| `crates/tilawa-core/` | The 29-day plan and progress rules (plain Rust library) |
| `crates/tilawa-app/` | The page: markup as Leptos components (`src/views.rs`, Tailwind classes), plus everything that moves; module list at the top of `src/lib.rs` |

## Rust migration

Moved to Rust step by step.

- [x] **Phase 1** – plan + progress rules in Rust (`tilawa-core`), called from the existing JS through WebAssembly
- [x] **Phase 2** – schedule cards rendered with Leptos (`tilawa-app`); JS keeps the header, carousel and bubbles
- [x] **Phase 3** – progress header, language, theme in Leptos; text as JSON
  - [x] progress header (count, line, bar, two-tap reset) in `header.rs`
  - [x] language switch + `data-i18n` text (`i18n.rs`, `page.rs`), text in `lang/*.jsonc`
  - [x] theme switch (`theme.rs`; `js/boot.js` still applies the saved pick before paint)
- [x] **Phase 4** – full Leptos CSR app built with `trunk`; old JS removed
  - [x] all page behaviour in Rust; QR code made in Rust (no CDN)
  - [x] built with `trunk` + `wasm-opt`, deployed by GitHub Actions
- [x] **Markup in Leptos** – the whole page is components (`views.rs`); text is reactive (`state.t("key")`), no more `data-i18n`
- [ ] Phase 5 (optional) – Rust API on Vercel, only if a feature needs a server
