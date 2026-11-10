// Local dev server with auto-reload.
// Run:  node dev.js        (or: node dev.js 4000 for another port)
// Stop: Ctrl+C
// Every time you save a file in this folder, the browser refreshes by itself.
// It also runs Tailwind in watch mode (after `npm install`), so styles/build.css
// is rebuilt whenever you change a class in index.html, js/ or styles/app.css.

const http = require("http");
const fs = require("fs");
const path = require("path");
const { spawn } = require("child_process");

const ROOT = __dirname;
const PORT = Number(process.argv[2]) || 5500;
const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".jsonc": "text/plain; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".ico": "image/x-icon",
  ".woff2": "font/woff2",
  ".wasm": "application/wasm",   // Rust core (pkg/tilawa_core_bg.wasm)
};

// Snippet added to every HTML page: listens for "reload" and refreshes.
const CLIENT = `<script>(function(){var es=new EventSource("/__reload");
es.onmessage=function(){location.reload()};})();</script>`;

const clients = new Set();

const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split("?")[0]);

  if (url === "/__reload") {
    res.writeHead(200, {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
    });
    res.write(": connected\n\n");
    clients.add(res);
    req.on("close", () => clients.delete(res));
    return;
  }

  let file = path.normalize(path.join(ROOT, url));
  if (!file.startsWith(ROOT)) {
    res.writeHead(403);
    return res.end("Forbidden");
  }
  if (fs.existsSync(file) && fs.statSync(file).isDirectory())
    file = path.join(file, "index.html");

  fs.readFile(file, (err, data) => {
    if (err) {
      res.writeHead(404);
      return res.end("Not found: " + url);
    }
    const ext = path.extname(file).toLowerCase();
    res.writeHead(200, {
      "Content-Type": TYPES[ext] || "application/octet-stream",
      "Cache-Control": "no-store",
    });
    if (ext === ".html") {
      const html = data.toString("utf8");
      return res.end(
        html.includes("</body>")
          ? html.replace("</body>", CLIENT + "</body>")
          : html + CLIENT,
      );
    }
    res.end(data);
  });
});

// Watch the folder; wait a moment so one save triggers one reload.
let timer;
fs.watch(ROOT, { recursive: true }, (event, name) => {
  const base = path.basename(name || "");
  // skip hidden files, editor swap/backup files and temp files
  if (
    !base ||
    base.startsWith(".") ||
    name.includes("node_modules") ||
    name.startsWith("target") ||          // cargo build output
    name.startsWith("crates") ||          // Rust source: needs `npm run build:wasm` first
    name === path.join("styles", "app.css") || // wait for Tailwind to write build.css instead
    /(~|\.swp|\.swx|\.tmp|^4913)$/.test(base)
  )
    return;
  clearTimeout(timer);
  timer = setTimeout(() => {
    console.log("changed: " + name + " → reloading");
    for (const c of clients) c.write("data: reload\n\n");
  }, 120);
});

// Tailwind watch: rebuilds styles/build.css from styles/app.css + the classes used in the markup
const tailwindBin = path.join(ROOT, "node_modules", ".bin", "tailwindcss");
if (fs.existsSync(tailwindBin)) {
  const tw = spawn(tailwindBin, ["-i", "styles/app.css", "-o", "styles/build.css", "--watch", "--minify"], {
    cwd: ROOT,
    stdio: ["pipe", "inherit", "inherit"],   // keep stdin open: Tailwind stops watching when it closes
  });
  process.on("exit", () => tw.kill());
  process.on("SIGINT", () => process.exit());
} else {
  console.log("Tailwind not installed: run `npm install` to rebuild styles while you edit.");
}

server.listen(PORT, () => {
  console.log("Tilawa dev server: http://localhost:" + PORT);
  console.log(
    "Edit and save any file, the browser reloads by itself. Ctrl+C to stop.",
  );
});
