// Runs in <head>, before the page paints (a plain script, not a module, so it can't be deferred).
(function () {
  try {
    // The site used to be called Chillawah: carry saved progress, language and theme over once
    ["progress-v1", "pages-v1", "lang", "theme"].forEach(function (k) {
      var v = localStorage.getItem("chillawah-" + k);
      if (v !== null && localStorage.getItem("tilawa-" + k) === null) localStorage.setItem("tilawa-" + k, v);
    });
    // Apply the saved theme now, so a dark-mode pick doesn't flash light first
    var th = localStorage.getItem("tilawa-theme");
    if (th === "dark" || th === "light") document.documentElement.dataset.theme = th;
  } catch (e) {}
})();
