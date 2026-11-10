// Tilawa service worker: lets the site open offline and be installed as an app.
// (Service workers have to be JavaScript; Rust only registers this file, see crates/tilawa-app/src/pwa.rs.)
//
//   page + lang/*.jsonc     network first, so a new deploy shows up right away; cache when offline
//   hashed build files      cache first: trunk gives every build new names (tilawa-app-<hash>.wasm…),
//   (wasm, js, css), fonts  so a cached copy never goes stale. Old builds are dropped once a new
//                           page has been loaded.
//   anything else           (boot.js, icons, manifest) cached copy right away, refreshed in the background
//
// Saved progress is in localStorage, not here, so clearing this cache never loses it.

const CACHE = "tilawa-v1";
const PAGE = new URL("./", self.location).href;
/** The one page: …/tilawa/ or …/tilawa/index.html (any ?query or #hash). Other URLs opened
 *  directly in the tab (say lang/id.jsonc) must never be stored as the page. */
const isPage = (url) => url.origin === self.location.origin &&
  (url.pathname === new URL(PAGE).pathname || url.pathname === new URL("index.html", PAGE).pathname);
const SHELL = ["./", "lang/en.jsonc", "lang/id.jsonc", "js/boot.js", "manifest.webmanifest", "icons/icon-192.png"];
const HASHED = /-[0-9a-f]{16}(_bg)?\.(wasm|js|css)$/;

/** The hashed files a page links to (trunk adds them as <link rel="preload|modulepreload|stylesheet">). */
function assetsIn(html) {
  return [...html.matchAll(/<link[^>]+href="([^"]+)"/g)]
    .map((m) => new URL(m[1], PAGE).href)
    .filter((u) => HASHED.test(u));
}

/** Caches the page and its build files, then forgets build files from older deploys. */
async function storePage(cache, response) {
  const html = await response.clone().text();
  const assets = assetsIn(html);
  await cache.put(PAGE, response);
  await Promise.all(assets.map(async (u) => { if (!(await cache.match(u))) await cache.add(u); }));
  const keep = new Set(assets);
  for (const req of await cache.keys()) {
    if (HASHED.test(req.url) && !keep.has(req.url)) await cache.delete(req);
  }
}

self.addEventListener("install", (event) => {
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    await cache.addAll(SHELL.filter((p) => p !== "./"));
    await storePage(cache, await fetch(PAGE, { cache: "no-cache" }));
    await self.skipWaiting();
  })());
});

self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    for (const name of await caches.keys()) if (name !== CACHE) await caches.delete(name);
    await self.clients.claim();
  })());
});

async function networkFirst(request, key) {
  const cache = await caches.open(CACHE);
  try {
    const response = await fetch(request);
    if (response.ok) {
      if (key === PAGE) await storePage(cache, response.clone());
      else await cache.put(key, response.clone());
    }
    return response;
  } catch (err) {
    const cached = await cache.match(key);
    if (cached) return cached;
    throw err;
  }
}

async function cacheFirst(request) {
  const cache = await caches.open(CACHE);
  const cached = await cache.match(request);
  if (cached) return cached;
  const response = await fetch(request);
  if (response.ok || response.type === "opaque") await cache.put(request, response.clone());
  return response;
}

async function staleWhileRevalidate(request) {
  const cache = await caches.open(CACHE);
  const cached = await cache.match(request);
  const fresh = fetch(request)
    .then(async (response) => { if (response.ok) await cache.put(request, response.clone()); return response; })
    .catch(() => cached);
  return cached || fresh;
}

self.addEventListener("fetch", (event) => {
  const req = event.request;
  if (req.method !== "GET") return;
  const url = new URL(req.url);

  if (req.mode === "navigate") {
    if (isPage(url)) event.respondWith(networkFirst(req, PAGE));   // …/#plan, …/?utm=… are the same page
    return;   // anything else opened in the tab: straight to the network
  }
  if (url.origin === self.location.origin && url.pathname.endsWith(".jsonc")) {
    event.respondWith(networkFirst(req, url.href));
  } else if (HASHED.test(url.pathname) || url.hostname === "fonts.googleapis.com" || url.hostname === "fonts.gstatic.com") {
    event.respondWith(cacheFirst(req));
  } else if (url.origin === self.location.origin) {
    event.respondWith(staleWhileRevalidate(req));
  }
});
