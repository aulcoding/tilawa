// Smaller pieces: the accordion under the hero, "copy link", and the Saweria support box.
import { t, reduce, onLang } from "./state.js";

// ── Accordion: stats, "screen time" and "plan" stay folded until the chevron is tapped ──
const more = document.getElementById("more");
const moreInner = more.querySelector(".more-inner");
const moreBtn = document.getElementById("more-toggle");

function setMore(open) {
  more.classList.toggle("open", open);
  moreBtn.setAttribute("aria-expanded", String(open));
  moreBtn.setAttribute("aria-label", open ? t().moreClose : t().moreOpen);
  moreBtn.title = moreBtn.getAttribute("aria-label");
  if (open) moreInner.removeAttribute("inert"); else moreInner.setAttribute("inert", "");
}

/** If `hash` points inside the folded part (e.g. the hero's "show me the rules"), open it and scroll there. */
export function openFor(hash) {
  const target = hash && hash.length > 1 && document.getElementById(hash.slice(1));
  if (!target || !more.contains(target)) return false;
  if (!more.classList.contains("open")) {
    // Open instantly (no height animation), so the scroll target is measured at its final spot
    more.style.transition = "none";
    setMore(true);
    void more.offsetHeight;
    more.style.transition = "";
  }
  target.scrollIntoView({ behavior: reduce ? "auto" : "smooth", block: "start" });
  return true;
}

function initAccordion() {
  moreBtn.addEventListener("click", () => setMore(!more.classList.contains("open")));
  document.addEventListener("click", (e) => {
    const a = e.target.closest('a[href^="#"]');
    if (a && openFor(a.getAttribute("href"))) e.preventDefault();
  });
  onLang(() => setMore(more.classList.contains("open")));
}

// ── "Invite a friend": copies the page link ──
function initShare() {
  const btn = document.getElementById("share");
  const label = document.getElementById("share-label");
  const live = document.getElementById("share-live");
  let timer;

  const fallbackCopy = (text) => {
    const ta = document.createElement("textarea");
    ta.value = text;
    ta.setAttribute("readonly", "");
    ta.style.position = "fixed";
    ta.style.opacity = "0";
    document.body.appendChild(ta);
    ta.select();
    let ok = false;
    try { ok = document.execCommand("copy"); } catch (e) {}
    ta.remove();
    return ok;
  };
  const show = (ok) => {
    const text = ok ? t().shareDone : t().shareFail;
    label.textContent = text;
    live.textContent = text;
    btn.classList.toggle("copied", ok);
    clearTimeout(timer);
    timer = setTimeout(() => {
      label.innerHTML = t().shareCta;
      live.textContent = "";
      btn.classList.remove("copied");
    }, 2600);
  };

  btn.addEventListener("click", () => {
    const url = location.href.split("#")[0];
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(url).then(() => show(true), () => show(fallbackCopy(url)));
    } else {
      show(fallbackCopy(url));
    }
  });
}

// ── Saweria: button + QR code ──
// Change the link here; the button and the QR code both use it.
const SAWERIA_URL = "https://saweria.co/aulcoding";

function initSaweria() {
  document.getElementById("saweria-link").href = SAWERIA_URL;
  const qr = document.getElementById("saweria-qr");
  if (window.QRCode) {
    new window.QRCode(qr, {
      text: SAWERIA_URL, width: 512, height: 512,
      colorDark: "#0B0C0E", colorLight: "#FFFFFF", correctLevel: window.QRCode.CorrectLevel.M,
    });
    qr.removeAttribute("title");   // the library adds a tooltip with the raw URL
  } else {
    qr.closest(".qr-card").hidden = true;   // QR library didn't load (offline): keep just the button
  }
}

export function initExtras() {
  initAccordion();
  initShare();
  initSaweria();
}
