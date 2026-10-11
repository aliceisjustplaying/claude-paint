/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
import "./tokens.css";
import "./styles.css";
import { appBar, LAB_URL } from "./ui/chrome";
import { mountStudio, type AppApi } from "./ui/app";
import { el } from "./ui/dom";
import { initializeCpu } from "./engine/cpu-simulation";

declare global {
  interface Window {
    /** The scripted driver: everything the pointer can do, callable. Used by
     * the figures script and the visual checks; left on in production
     * because a console-driven brush is a feature, not a leak. */
    __wash?: AppApi;
  }
}

// --- The experiment page shell -------------------------------------------
// This tool is EXP-040 in the Lab on ryankm.com and it is hosted here rather
// than there, so the page has to do the work the site's /lab/<slug> route
// normally does: the trail back up, the record voice (number, kind, state,
// then the stack), the claim in display type, and the lede. Same order, same
// tokens, same measure as the other off-site experiments (see Attention Lab).

/* Source: src/main.ts:23-57 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function experimentHead(): HTMLElement {
  const meta = el(
    "div",
    { class: "exp-meta" },
    el("span", { class: "label label-strong" }, "EXP-040"),
    el("span", { class: "label" }, "tool"),
    el(
      "span",
      { class: "label label-strong exp-state" },
      el("span", { class: "status-dot", "aria-hidden": "true" }),
      "Live"
    ),
    el("span", { class: "exp-rule", "aria-hidden": "true" }),
    el("span", { class: "label exp-stack" }, "Curtis et al. 1997 · Kubelka-Munk · WebGL2 · Client-side only")
  );

  return el(
    "header",
    { class: "app-header" },
    appBar(),
    el(
      "div",
      { class: "container exp-head" },
      meta,
      el("h1", { class: "t-h1" }, "Watercolor Lab", el("span", { class: "dot" }, ".")),
      // Two sentences. The physics gets its say in the bench notes below the
      // sheet, not above it.
      el(
        "p",
        { class: "t-lede" },
        "Real watercolor, simulated: water flows across the sheet, pigment rides it and settles, and the paper decides the rest. Thirty-six real pigments, wet-on-wet blooms, granulation and backruns, all running locally in your browser."
      )
    )
  );
}

// Bench notes, in the site's ledger voice: a label rail on the left and the
// note beside it. What a painter needs to know that the studio cannot say.
// Focus is a desk feature (on a phone the sheet already leads the page at
// full width), so its sentence only appears where the button does.
const BENCH_NOTES: { k: string; v: (string | HTMLElement)[] }[] = [
  {
    k: "Wet the sheet first",
    v: [
      "Water before paint gives soft blooms; paint onto dry paper for hard edges. Backlight shows what the light survives." +
        (window.matchMedia("(max-width: 720px)").matches
          ? ""
          : " Focus (the button on the desk, or F) gives you the sheet alone, as big as the window."),
    ],
  },
  {
    k: "Dried paint is cured",
    v: [
      "Once a wash dries it is part of the sheet: glaze over it, but you cannot lift it. The lift tool works on damp washes, and Forever wet keeps the sheet workable for as long as you like.",
    ],
  },
  {
    k: "The color is physics",
    v: [
      "Every swatch and every wash is Kubelka-Munk optics from the absorption and scattering of a real pigment, not alpha blending. Mixing on the sheet and mixing in the well are the same arithmetic.",
    ],
  },
  {
    k: "It stays on this machine",
    v: [
      "There is no server and no analytics. A saved sheet lives in this browser's own storage, which is also why clearing site data deletes it.",
    ],
  },
  {
    k: "Where it comes from",
    v: [
      "The fluid model is ",
      el(
        "a",
        { href: "https://grail.cs.washington.edu/projects/watercolor/" },
        "Curtis, Anderson, Seims, Fleischer & Salesin, Computer-Generated Watercolor (SIGGRAPH 1997)"
      ),
      ", as WebGL2 fragment shaders. The ",
      el("a", { href: "https://github.com/uxpreview/watercolor-lab" }, "source"),
      " is on GitHub.",
    ],
  },
];

/* Source: src/main.ts:107-127 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function footer(): HTMLElement {
  const notes = el("dl", { class: "notes" });
  for (const note of BENCH_NOTES) {
    notes.append(el("div", { class: "note-row" }, el("dt", { class: "label" }, note.k), el("dd", {}, ...note.v)));
  }
  return el(
    "footer",
    { class: "app-footer" },
    el(
      "div",
      { class: "container" },
      el("h2", { class: "t-display notes-title" }, "Bench notes"),
      notes,
      el(
        "p",
        { class: "footer-credit" },
        el("a", { class: "arrow-link", href: LAB_URL }, "EXP-040 in the Lab at ryankm.com", el("span", { "aria-hidden": "true" }, "→"))
      )
    )
  );
}

const root = document.getElementById("app")!;

// On a phone the sheet leads: the first thing on the page is the paper, the
// way every phone painting tool opens, and the masthead becomes the colophon
// between the studio and the bench notes. A desk keeps the site's order,
// where the record voice above the fold is the point of the Lab shell.
const phoneShell = window.matchMedia("(max-width: 720px)").matches;

if (!phoneShell) root.append(experimentHead());

try {
  await initializeCpu();
  window.__wash = mountStudio(root);
} catch (err) {
  root.append(
    el(
      "div",
      { class: "container gl-error" },
      el("h2", {}, "This browser can't run the simulation"),
      el("p", {}, `The simulator needs WebGL2 with float render targets. (${err instanceof Error ? err.message : String(err)})`)
    )
  );
}

if (phoneShell) root.append(experimentHead());
root.append(footer());
