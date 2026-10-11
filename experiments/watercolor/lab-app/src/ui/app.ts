/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * The studio screen: sheet on the left (or on top, on a phone), paint box
 * beside it, and the wiring between pointer events, the brush model, the
 * mixing well, and the simulator.
 */

import { beginStroke, brushRadius, strokeTo, tapStamp, TOOLS, type StrokeState, type ToolId } from "../paint/brush";
import { DEFAULT_PIGMENT_ID, handlingNote, PIGMENT_BY_ID, PIGMENTS, type Pigment } from "../paint/pigments";
import { mixWell, wellName, type WellEntry } from "../paint/mix";
import { PAPERS, type PaperKind } from "../engine/paper";
import { composite, layerRT, linearToSrgbByte, PAPER_REFLECTANCE, type Vec3 } from "../paint/km";
import { XRAY_FIELDS, type XrayField } from "../engine/simulation";
import { createBlindSimulation } from "../engine/blind-simulation";
import type { SheetSimulation } from "../engine/cpu-simulation";
import { loadSheet, saveSheet } from "../data/store";
import { fitDried } from "../data/resample";
import { confirmButton, el } from "./dom";
import { createPalette, paintChip } from "./palette";

export const SIM_W = 1056;
export const SIM_H = 704;

/** The simulation grid. Landscape 3:2 on a desk, portrait 2:3 on a phone —
 * a sheet is held the way the hand holding it is oriented. `?sim=WxH`
 * overrides both (the scripted figure harness uses it). */
/* Source: src/ui/app.ts:24-29 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function simDims(): { w: number; h: number } {
  const m = /[?&]sim=(\d+)x(\d+)/.exec(location.search);
  if (m) return { w: Math.min(2048, Number(m[1])), h: Math.min(2048, Number(m[2])) };
  const portrait = window.matchMedia("(max-width: 720px)").matches;
  return portrait ? { w: SIM_H / 8, h: SIM_W / 8 } : { w: SIM_W / 8, h: SIM_H / 8 };
}

export interface AppApi {
  sim: SheetSimulation;
  setTool(id: ToolId | "salt"): void;
  setPigment(id: string): void;
  setSize(v: number): void;
  setWater(v: number): void;
  setLoad(v: number): void;
  setPaper(kind: PaperKind): void;
  setBacklight(on: boolean): void;
  /** The X-ray view: 0 is the painting, 1–6 one state field each. */
  setXray(field: XrayField): void;
  strokePath(points: Array<{ x: number; y: number }>, stepsPerSegment?: number): void;
  salt(x: number, y: number): void;
  dry(): void;
  undo(): void;
  clear(): void;
  step(n: number, evapMul?: number): void;
}

const ICONS: Record<string, string> = {
  round: `<svg viewBox="0 0 24 24"><path d="M12 3c1.8 0 3.4 2.6 3.4 6.2 0 2.9-1 5.3-2.1 7.4-.5 1-.8 2.4-1.3 2.4s-.8-1.4-1.3-2.4c-1.1-2.1-2.1-4.5-2.1-7.4C8.6 5.6 10.2 3 12 3z" fill="currentColor"/></svg>`,
  mop: `<svg viewBox="0 0 24 24"><ellipse cx="12" cy="10" rx="6.5" ry="7" fill="currentColor" opacity="0.55"/><ellipse cx="12" cy="9" rx="4" ry="4.6" fill="currentColor"/></svg>`,
  rigger: `<svg viewBox="0 0 24 24"><path d="M11.4 3h1.2l.6 12.5c0 2.5-.5 5.5-1.2 5.5s-1.2-3-1.2-5.5L11.4 3z" fill="currentColor"/></svg>`,
  drybrush: `<svg viewBox="0 0 24 24"><g fill="currentColor"><rect x="4" y="5" width="2.1" height="14" rx="1"/><rect x="8.2" y="4" width="2.1" height="12" rx="1"/><rect x="12.4" y="6" width="2.1" height="14" rx="1"/><rect x="16.6" y="4.5" width="2.1" height="11" rx="1"/></g></svg>`,
  spatter: `<svg viewBox="0 0 24 24"><g fill="currentColor"><circle cx="7" cy="8" r="2.6"/><circle cx="15.5" cy="5.5" r="1.6"/><circle cx="18" cy="12" r="2.2"/><circle cx="10" cy="15.5" r="1.3"/><circle cx="15" cy="18.5" r="1.8"/><circle cx="5.5" cy="18" r="1"/></g></svg>`,
  water: `<svg viewBox="0 0 24 24"><path d="M12 3.5c2.8 3.9 6 7.6 6 11a6 6 0 1 1-12 0c0-3.4 3.2-7.1 6-11z" fill="none" stroke="currentColor" stroke-width="1.8"/></svg>`,
  lift: `<svg viewBox="0 0 24 24"><rect x="5" y="5" width="14" height="9" rx="3" fill="currentColor" opacity="0.8"/><path d="M8 17c0 1.2-.9 2.5-.9 2.5M12 17c0 1.2-.9 2.5-.9 2.5M16 17c0 1.2-.9 2.5-.9 2.5" stroke="currentColor" stroke-width="1.6" fill="none" stroke-linecap="round"/></svg>`,
  salt: `<svg viewBox="0 0 24 24"><g fill="currentColor"><path d="M12 4l1 2.2L15.2 7 13 8l-1 2.2L11 8l-2.2-1L11 6.2z"/><circle cx="6.5" cy="13.5" r="1.2"/><circle cx="12" cy="16" r="1.2"/><circle cx="17.5" cy="13.5" r="1.2"/><circle cx="9" cy="19.5" r="1"/><circle cx="15" cy="19.5" r="1"/></g></svg>`,
  undo: `<svg viewBox="0 0 24 24"><path d="M8.5 4.5L4 9l4.5 4.5M4 9h9.5a5.5 5.5 0 0 1 0 11H9" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  dry: `<svg viewBox="0 0 24 24"><path d="M3 8h10a3 3 0 1 0-3-3M3 13h14a3 3 0 1 1-3 3M3 18h7" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>`,
};

/* Source: src/ui/app.ts:63-68 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function iconButton(name: string, label: string, onClick: () => void): HTMLButtonElement {
  const btn = el("button", { class: "tool", type: "button", title: label, "aria-label": label, onclick: onClick });
  btn.innerHTML = ICONS[name] ?? "";
  btn.append(el("span", { class: "tool-label" }, label));
  return btn;
}

/* Source: src/ui/app.ts:70-92 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function slider(label: string, initial: number, onInput: (v: number) => void): { root: HTMLElement; set(v: number): void } {
  const readout = el("span", { class: "slider-value" }, String(Math.round(initial * 100)));
  const apply = /* Source: src/ui/app.ts:72-75 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v: number) => {
    readout.textContent = String(Math.round(v * 100));
    onInput(v);
  };
  const input = el("input", {
    type: "range",
    min: "0",
    max: "1",
    step: "0.01",
    value: String(initial),
    oninput: /* Source: src/ui/app.ts:82-82 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => apply(Number(input.value)),
  }) as HTMLInputElement;
  const root = el("label", { class: "slider" }, el("span", { class: "slider-label" }, label), input, readout);
  return {
    root,
    /* Source: src/ui/app.ts:87-90 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ set(v: number) {
      input.value = String(v);
      apply(v);
    },
  };
}

/* Source: src/ui/app.ts:94-1046 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function mountStudio(host: HTMLElement): AppApi {
  const dims = simDims();
  const portrait = dims.h > dims.w;
  // The phone shell: sheet first, a paint strip in view beneath it, the bench
  // in the page flow, a fixed thumb bar. Decided once, at mount, like the
  // simulation grid's orientation.
  const phone = window.matchMedia("(max-width: 720px)").matches;
  const canvas = el("canvas", {
    class: `sheet${portrait ? " is-portrait" : ""}`,
    "aria-label": "Watercolor paper. Draw here to paint.",
  });

  // ---- state -------------------------------------------------------------
  let tool: ToolId | "salt" = "round";
  let well: WellEntry[] = [{ pigment: PIGMENT_BY_ID.get(DEFAULT_PIGMENT_ID)!, parts: 1 }];
  let brushPigment: Pigment | null = mixWell(well);
  let mixMode = false;
  let size = 0.42;
  let water = 0.55;
  let load = 0.65;
  let backlight = false;
  let xray: XrayField = 0;
  let rain = false;
  let stroke: StrokeState | null = null;
  let strokeMoved = false;

  const comparison = createBlindSimulation(canvas, dims.w, dims.h);
  const sim = comparison.sim;
  const scripted = /[?&](sim|seed)=/.test(location.search);
  const seedMatch = /[?&]seed=(\d+)/.exec(location.search);
  if (seedMatch) {
    sim.paperSeed = Number(seedMatch[1]);
    sim.setPaper("cold-press", true, sim.paperSeed);
  }
  const refScale = dims.w / (portrait ? SIM_H : SIM_W);

  // ---- canvas sizing -----------------------------------------------------
  const fitCanvas = /* Source: src/ui/app.ts:130-139 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    const rect = canvas.getBoundingClientRect();
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const w = Math.round(rect.width * dpr);
    const h = Math.round(rect.height * dpr);
    if (w > 0 && h > 0 && (canvas.width !== w || canvas.height !== h)) {
      canvas.width = w;
      canvas.height = h;
    }
  };
  new ResizeObserver(fitCanvas).observe(canvas);

  const toSim = /* Source: src/ui/app.ts:142-148 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e: PointerEvent): { x: number; y: number } => {
    const rect = canvas.getBoundingClientRect();
    return {
      x: ((e.clientX - rect.left) / rect.width) * dims.w,
      y: ((e.clientY - rect.top) / rect.height) * dims.h,
    };
  };

  // ---- painting ----------------------------------------------------------
  // ---- lift hint -----------------------------------------------------------
  // The lift tool re-suspends paint that has settled but not cured. Dried
  // paint is permanent, as on a real sheet, so scrubbing it does nothing —
  // say so once, instead of letting the silence read as a bug.
  const hint = el("p", { class: "sheet-hint", role: "status", "aria-live": "polite" });
  let hintTimer = 0;
  const showHint = /* Source: src/ui/app.ts:157-162 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (text: string) => {
    hint.textContent = text;
    hint.classList.add("is-visible");
    window.clearTimeout(hintTimer);
    hintTimer = window.setTimeout(/* Source: src/ui/app.ts:161-161 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => hint.classList.remove("is-visible"), 3200);
  };
  const checkLiftTarget = /* Source: src/ui/app.ts:163-167 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (x: number, y: number) => {
    const probe = sim.probe(x, y, Math.min(12, brushRadius(TOOLS.lift, size) * 0.5));
    const cured = probe.dried > 0.02 && probe.workable < 0.004 && probe.wet < 0.02;
    if (cured) showHint("That paint has dried and is cured. Lift works on damp washes. Wet it first, or paint over it.");
  };

  let lastSalt = { x: -1e9, y: -1e9 };
  // The undo snapshot is taken just before the stroke's first mark, not on
  // pointerdown: a touch that turns out to be a gesture must not spend an
  // undo slot on a state identical to the one before it.
  let strokeSnapshotted = false;
  const ensureSnapshot = /* Source: src/ui/app.ts:174-178 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    if (strokeSnapshotted) return;
    sim.snapshot();
    strokeSnapshotted = true;
  };
  // On touch, salt waits for the finger to move or lift, so a two-finger
  // gesture that starts on the salt tool never drops a grain.
  let pendingSalt: { x: number; y: number } | null = null;
  // Two fingers on the sheet are a gesture, not two brushes: painting stops,
  // and a quick two-finger tap is undo — the shortcut every touch painting
  // app shares (Procreate, Fresco, Sketchbook). If the first finger already
  // left paint, the gesture cancels that partial stroke instead.
  const touches = new Set<number>();
  let gestureStart = 0;
  let gestureConsumed = false;

  canvas.addEventListener("pointerdown", /* Source: src/ui/app.ts:190-228 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => {
    e.preventDefault();
    canvas.setPointerCapture(e.pointerId);
    if (e.pointerType === "touch") {
      touches.add(e.pointerId);
      if (touches.size === 2) {
        // The event's own timestamp, not performance.now(): under a heavy
        // wash the frame loop can lag the handlers by more than the tap.
        gestureStart = e.timeStamp;
        gestureConsumed = false;
        if (strokeSnapshotted) {
          // The first finger painted before the second landed: take that
          // partial stroke back off the sheet, and the tap is spent.
          sim.undo();
          gestureConsumed = true;
        }
        stroke = null;
        pendingSalt = null;
        strokeSnapshotted = false;
        return;
      }
      if (touches.size > 2) return;
    }
    const at = toSim(e);
    strokeSnapshotted = false;
    if (tool === "salt") {
      if (e.pointerType === "touch") {
        pendingSalt = at;
      } else {
        ensureSnapshot();
        sim.addSalt(at.x, at.y);
        lastSalt = at;
      }
      return;
    }
    stroke = beginStroke(TOOLS[tool], size, water, load, at.x, at.y, (e.pointerId + 1) * 7919, refScale);
    strokeMoved = false;
    if (tool === "lift") checkLiftTarget(at.x, at.y);
  });

  const strokePigment = /* Source: src/ui/app.ts:230-230 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => (stroke && stroke.tool.pigment > 0 ? brushPigment : null);

  canvas.addEventListener("pointermove", /* Source: src/ui/app.ts:232-262 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => {
    if (e.pointerType === "touch" && touches.size >= 2) return;
    const events = typeof e.getCoalescedEvents === "function" ? e.getCoalescedEvents() : [e];
    if (tool === "salt") {
      if (e.buttons === 0) return;
      const at = toSim(e);
      if (pendingSalt) {
        ensureSnapshot();
        sim.addSalt(pendingSalt.x, pendingSalt.y);
        lastSalt = pendingSalt;
        pendingSalt = null;
      }
      if (Math.hypot(at.x - lastSalt.x, at.y - lastSalt.y) > 60) {
        sim.addSalt(at.x, at.y);
        lastSalt = at;
      }
      return;
    }
    if (!stroke) return;
    for (const ev of events) {
      const at = toSim(ev as PointerEvent);
      const pressure = (ev as PointerEvent).pressure > 0 ? (ev as PointerEvent).pressure : 0.5;
      const stamps = strokeTo(stroke, at.x, at.y, pressure);
      if (stamps.length > 0) {
        ensureSnapshot();
        strokeMoved = true;
        sim.splat(stamps, strokePigment());
        if (stroke.tool.id === "lift") sim.addScrub(stamps);
      }
    }
  });

  const endStroke = /* Source: src/ui/app.ts:264-294 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e: PointerEvent) => {
    if (e.pointerType === "touch") {
      touches.delete(e.pointerId);
      if (gestureStart > 0) {
        if (touches.size === 0) {
          const quick = e.timeStamp - gestureStart < 350;
          gestureStart = 0;
          if (quick && !gestureConsumed) sim.undo();
          gestureConsumed = false;
        }
        stroke = null;
        pendingSalt = null;
        return;
      }
    }
    if (pendingSalt) {
      ensureSnapshot();
      sim.addSalt(pendingSalt.x, pendingSalt.y);
      lastSalt = pendingSalt;
      pendingSalt = null;
    }
    if (stroke && !strokeMoved) {
      // A tap still leaves a mark — a loaded brush touched the sheet.
      const pressure = e.pressure > 0 ? e.pressure : 0.5;
      const stamp = tapStamp(stroke, pressure);
      ensureSnapshot();
      sim.splat([stamp], strokePigment());
      if (stroke.tool.id === "lift") sim.addScrub([stamp]);
    }
    stroke = null;
  };
  canvas.addEventListener("pointerup", endStroke);
  canvas.addEventListener("pointercancel", endStroke);

  window.addEventListener("keydown", /* Source: src/ui/app.ts:298-305 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "z") {
      e.preventDefault();
      sim.undo();
    }
    if (e.key === "[") sizeCtl.set(Math.max(0, size - 0.06));
    if (e.key === "]") sizeCtl.set(Math.min(1, size + 0.06));
  });

  // The brush preview lives further down; the well marks it stale and the
  // frame repaints it, so pigment changes and slider moves share one path.
  let previewDirty = false;

  // ---- the mixing well ---------------------------------------------------
  const WELL_CHIP = 44;
  const wellCanvas = el("canvas", { width: WELL_CHIP, height: WELL_CHIP, class: "well-swatch" });
  // The thumb bar's copy of the well: same physics chip, painted in the same
  // pass, so the bar always shows the paint the brush is actually carrying.
  const barWell = el("canvas", { width: 28, height: 28, class: "bar-swatch", "aria-hidden": "true" });
  const wellName_ = el("span", { class: "pigment-name" });
  const wellMeta = el("span", { class: "pigment-meta" });
  const recentRow = el("div", { class: "recent-row", "aria-label": "Recent pigments" });
  let recents: Pigment[] = [];
  // The phone's paint strip mirrors the well; assigned where the strip is
  // built, called wherever the well changes, a no-op on a desk.
  let syncStrip: () => void = /* Source: src/ui/app.ts:323-323 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {};

  const renderWell = /* Source: src/ui/app.ts:325-343 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    if (brushPigment) {
      paintChip(wellCanvas, brushPigment);
      paintChip(barWell, brushPigment);
      wellName_.textContent = wellName(well);
      wellMeta.textContent =
        well.length > 1 ? `mix · ${handlingNote(brushPigment)}` : `${brushPigment.code} · ${handlingNote(brushPigment)}`;
    } else {
      for (const c of [wellCanvas, barWell]) {
        const ctx2d = c.getContext("2d")!;
        ctx2d.fillStyle = "#f2ead9";
        ctx2d.fillRect(0, 0, c.width, c.height);
      }
      wellName_.textContent = "Rinsed";
      wellMeta.textContent = "clean water. Dip a pigment";
    }
    syncStrip();
    previewDirty = true;
  };

  const renderRecents = /* Source: src/ui/app.ts:345-357 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    recentRow.replaceChildren(
      ...recents.map(/* Source: src/ui/app.ts:347-355 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (p) => {
        const mini = el("canvas", { width: 28, height: 28, class: "chip-swatch" });
        paintChip(mini, p);
        return el(
          "button",
          { class: "chip chip-mini", type: "button", title: p.name, "aria-label": p.name, onclick: /* Source: src/ui/app.ts:352-352 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => dip(p) },
          mini
        );
      })
    );
  };

  const dip = /* Source: src/ui/app.ts:359-372 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (p: Pigment) => {
    if (mixMode && well.length > 0) {
      const existing = well.find(/* Source: src/ui/app.ts:361-361 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => e.pigment.id === p.id);
      if (existing) existing.parts = Math.min(existing.parts + 1, 6);
      else if (well.length < 4) well.push({ pigment: p, parts: 1 });
    } else {
      well = [{ pigment: p, parts: 1 }];
    }
    brushPigment = mixWell(well);
    recents = [p, ...recents.filter(/* Source: src/ui/app.ts:368-368 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (r) => r.id !== p.id)].slice(0, 6);
    if (tool === "water" || tool === "lift" || tool === "salt") selectTool("round");
    renderWell();
    renderRecents();
  };

  const rinse = /* Source: src/ui/app.ts:374-378 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    well = [];
    brushPigment = null;
    renderWell();
  };

  const mixBtn = el(
    "button",
    {
      class: "btn btn-small",
      type: "button",
      "aria-pressed": "false",
      title: "When on, dipping a pigment adds it to the mix instead of replacing it",
      onclick: /* Source: src/ui/app.ts:387-391 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
        mixMode = !mixMode;
        mixBtn.setAttribute("aria-pressed", String(mixMode));
        mixBtn.classList.toggle("is-active", mixMode);
      },
    },
    "Mix"
  );

  const wellRow = el(
    "div",
    { class: "well-row" },
    wellCanvas,
    el("div", { class: "pigment-caption well-caption" }, wellName_, wellMeta),
    el("div", { class: "well-actions" }, mixBtn, el("button", { class: "btn btn-small", type: "button", onclick: rinse }, "Rinse"))
  );

  const palette = createPalette(DEFAULT_PIGMENT_ID);
  palette.onChange(dip);

  // ---- the paint strip (phone) -------------------------------------------
  // Color in view without opening anything: the first thing under the sheet
  // is the current charge by name, Rinse, and all 36 pans in one
  // thumb-scrollable row, the same physics chips as the palette. A tap dips;
  // mixing, recents and the pigment cards stay in the Pigments box below.
  let paintStrip: HTMLElement | null = null;
  if (phone) {
    const stripChips = new Map<string, HTMLButtonElement>();
    const stripRow = el("div", { class: "strip-row", role: "listbox", "aria-label": "Pigments, quick row" });
    for (const p of PIGMENTS) {
      const mini = el("canvas", { width: 40, height: 40, class: "chip-swatch" });
      paintChip(mini, p);
      const chipBtn = el(
        "button",
        { class: "chip strip-chip", type: "button", role: "option", title: p.name, "aria-label": p.name, "aria-selected": "false", onclick: /* Source: src/ui/app.ts:421-421 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => dip(p) },
        mini
      );
      stripChips.set(p.id, chipBtn);
      stripRow.append(chipBtn);
    }
    const stripName = el("span", { class: "strip-name" });
    syncStrip = /* Source: src/ui/app.ts:428-444 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
      stripName.textContent = brushPigment ? wellName(well) : "Rinsed, clean water";
      for (const [id, b] of stripChips) b.setAttribute("aria-selected", String(well.some(/* Source: src/ui/app.ts:430-430 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => e.pigment.id === id)));
      // If the charge changed from somewhere other than the strip (the
      // palette, recents, restore) the ringed chip may be off-screen: bring
      // it in. Only the row scrolls, and only when the chip is out of view.
      const sel = well[0] && stripChips.get(well[0].pigment.id);
      if (sel) {
        const rowRect = stripRow.getBoundingClientRect();
        if (rowRect.width > 0) {
          const chipRect = sel.getBoundingClientRect();
          if (chipRect.right < rowRect.left + 8 || chipRect.left > rowRect.right - 8) {
            stripRow.scrollLeft += chipRect.left - rowRect.left - (rowRect.width - chipRect.width) / 2;
          }
        }
      }
    };
    paintStrip = el(
      "div",
      { class: "paint-strip" },
      el(
        "div",
        { class: "strip-head" },
        stripName,
        el("button", { class: "btn btn-small", type: "button", onclick: rinse }, "Rinse")
      ),
      stripRow
    );
  }

  renderWell();

  // ---- the brush preview ---------------------------------------------------
  // What the sliders mean without a mark on the sheet: one dab of the current
  // brush, painted by the same Kubelka-Munk maths as the pans. Radius is the
  // brush's real radius against a fixed scale where the biggest brush fills
  // the box; water widens and softens the edge and thins the film; pigment
  // load sets the film's concentration. The line under it gives the true
  // size as a share of the sheet's width, which is the number that transfers.
  const PREVIEW = 96;
  const previewCanvas = el("canvas", { width: PREVIEW * 2, height: PREVIEW * 2, class: "brush-preview-canvas", "aria-hidden": "true" });
  const previewLine = el("p", { class: "brush-preview-line" });
  const renderPreview = /* Source: src/ui/app.ts:470-542 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    const ctx2d = previewCanvas.getContext("2d")!;
    const size2 = PREVIEW * 2;
    const image = ctx2d.createImageData(size2, size2);
    const [pr, pg, pb] = PAPER_REFLECTANCE;
    const isSalt = tool === "salt";
    const spec = isSalt ? null : TOOLS[tool as ToolId];
    const radiusSim = isSalt ? 46 : brushRadius(spec!, size);
    // The box is this brush at full size: the dab shows where the slider is
    // and how the film reads; the line beneath gives the true footprint.
    const fullRadius = isSalt ? 46 : brushRadius(spec!, 1);
    const radiusPx = (radiusSim / fullRadius) * (size2 / 2) * 0.86;
    const waterAmt = spec ? water * spec.water : 0;
    const pigmentAmt = spec ? load * spec.pigment : 0;
    // A stroke lays ~1/spacing overlapping stamps; the film's concentration
    // is that many splats' worth, thinned by the water it rides in.
    const conc = spec ? (pigmentAmt * 0.13 * (1 / spec.spacing)) / (0.55 + waterAmt) : 0;
    const feather = spec ? 0.08 + 0.42 * Math.min(1, waterAmt / 1.6) : 0.1;
    const wetDarken = spec ? 1 - 0.075 * Math.min(1, waterAmt) : 1;
    const r: Vec3 = [0, 0, 0];
    const t: Vec3 = [0, 0, 0];
    const out: Vec3 = [0, 0, 0];
    const cx = size2 / 2;
    const cy = size2 / 2;
    for (let y = 0; y < size2; y++) {
      for (let x = 0; x < size2; x++) {
        const d = Math.hypot(x - cx, y - cy) / Math.max(radiusPx, 1);
        // Solid core, feathered rim: the wet edge of a real dab.
        const inside = d <= 1 - feather ? 1 : d >= 1 ? 0 : (1 - d) / feather;
        let R = pr;
        let G = pg;
        let B = pb;
        if (inside > 0) {
          const dark = 1 - (1 - wetDarken) * inside;
          const paper: Vec3 = [pr * dark, pg * dark, pb * dark];
          if (brushPigment && conc > 0) {
            layerRT(brushPigment.k, brushPigment.s, conc * inside, r, t);
            composite(r, t, paper, out);
            R = out[0];
            G = out[1];
            B = out[2];
          } else {
            R = paper[0];
            G = paper[1];
            B = paper[2];
          }
        }
        const idx = (y * size2 + x) * 4;
        image.data[idx] = linearToSrgbByte(R);
        image.data[idx + 1] = linearToSrgbByte(G);
        image.data[idx + 2] = linearToSrgbByte(B);
        image.data[idx + 3] = 255;
      }
    }
    ctx2d.putImageData(image, 0, 0);
    if (isSalt) {
      // Salt is grains, not a film: a scatter of dots at the true throw radius.
      ctx2d.fillStyle = "#c9c2b0";
      let seed = 7;
      for (let i = 0; i < 24; i++) {
        seed = (seed * 16807) % 2147483647;
        const a = (seed / 2147483647) * Math.PI * 2;
        seed = (seed * 16807) % 2147483647;
        const dd = Math.sqrt(seed / 2147483647) * radiusPx;
        ctx2d.beginPath();
        ctx2d.arc(cx + Math.cos(a) * dd, cy + Math.sin(a) * dd, 2.2, 0, Math.PI * 2);
        ctx2d.fill();
      }
    }
    const share = Math.round(((radiusSim * refScale * 2) / dims.w) * 100);
    const what = isSalt ? "salt throw" : brushPigment && pigmentAmt > 0 ? "one dab" : "clean water";
    previewLine.textContent = `${what} · ${share}% of the sheet's width`;
  };

  // ---- the brush ring ------------------------------------------------------
  // On a desk the pointer wears the brush's true footprint over the sheet, so
  // size reads before a mark is made. Fine pointers only: a finger covers it.
  const ring = el("div", { class: "brush-ring", "aria-hidden": "true" });
  const finePointer = window.matchMedia("(hover: hover) and (pointer: fine)").matches;
  const moveRing = /* Source: src/ui/app.ts:549-560 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e: PointerEvent) => {
    if (!finePointer) return;
    const rect = canvas.getBoundingClientRect();
    const host = easel.getBoundingClientRect();
    const scale = rect.width / dims.w;
    const radiusSim = tool === "salt" ? 46 : brushRadius(TOOLS[tool as ToolId], size);
    const px = radiusSim * refScale * scale;
    ring.style.width = `${px * 2}px`;
    ring.style.height = `${px * 2}px`;
    ring.style.transform = `translate(${e.clientX - host.left - px}px, ${e.clientY - host.top - px}px)`;
    ring.classList.add("is-visible");
  };
  canvas.addEventListener("pointermove", moveRing);
  canvas.addEventListener("pointerenter", moveRing);
  canvas.addEventListener("pointerleave", /* Source: src/ui/app.ts:563-563 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => ring.classList.remove("is-visible"));

  // ---- focus ------------------------------------------------------------------
  // The sheet takes the whole window and the bench becomes a drawer. Desks
  // only: on a phone the sheet already leads the page at full width, so a
  // focus state would be a second copy of the screen the phone starts on.
  let focus = false;
  let drawerOpen = false;
  // The button lives on the desk itself, top right, where a fullscreen
  // control is expected: icon, word, and the key. It is the one control that
  // is not in the bench, because it is about the sheet, not the paint.
  const FOCUS_ICON = `<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M4 9V4h5M20 9V4h-5M4 15v5h5M20 15v5h-5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></svg>`;
  const focusBtn = el(
    "button",
    {
      class: "btn btn-primary focus-toggle",
      type: "button",
      "aria-pressed": "false",
      title: "Focus: the sheet fills the window (F)",
      onclick: /* Source: src/ui/app.ts:582-582 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => setFocus(!focus),
    }
  );
  focusBtn.innerHTML = `${FOCUS_ICON}<span>Focus</span><kbd>F</kbd>`;
  const drawerBtn = el(
    "button",
    { class: "btn focus-tools", type: "button", "aria-pressed": "false", "aria-expanded": "false", onclick: /* Source: src/ui/app.ts:588-588 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => setDrawer(!drawerOpen) },
    "Tools"
  );
  const exitBtn = el("button", { class: "btn focus-exit", type: "button", onclick: /* Source: src/ui/app.ts:591-591 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => setFocus(false) }, "Exit focus");
  const focusBar = el("div", { class: "focus-bar" }, drawerBtn, exitBtn);
  const setDrawer = /* Source: src/ui/app.ts:593-599 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (open: boolean) => {
    drawerOpen = open;
    document.body.classList.toggle("is-drawer-open", open);
    drawerBtn.setAttribute("aria-pressed", String(open));
    drawerBtn.setAttribute("aria-expanded", String(open));
    drawerBtn.classList.toggle("is-active", open);
  };
  const setFocus = /* Source: src/ui/app.ts:600-611 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (on: boolean) => {
    focus = on;
    document.body.classList.toggle("is-focus", on);
    focusBtn.setAttribute("aria-pressed", String(on));
    if (!on) setDrawer(false);
    // The canvas' box changes size in the same frame; the observer catches it,
    // but the ring's cached rect is stale until the next move.
    ring.classList.remove("is-visible");
    layoutSheet();
    if (on) exitBtn.focus();
    else focusBtn.focus();
  };
  if (!phone) {
    window.addEventListener("keydown", /* Source: src/ui/app.ts:613-623 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => {
      if (e.key === "Escape") {
        if (drawerOpen) setDrawer(false);
        else if (focus) setFocus(false);
      }
      const typing = (e.target as HTMLElement | null)?.tagName === "INPUT";
      if ((e.key === "f" || e.key === "F") && !e.metaKey && !e.ctrlKey && !e.altKey && !typing) {
        e.preventDefault();
        setFocus(!focus);
      }
    });
  }

  // ---- toolbox -----------------------------------------------------------
  // The thumb bar's brush button wears the current tool's icon and name, so
  // the bar reads what the finger is about to do without opening anything.
  const brushBarIcon = el("span", { class: "bar-icon", "aria-hidden": "true" });
  const brushBarLabel = el("span", { class: "bar-label" }, "Brush");
  const toolButtons = new Map<string, HTMLButtonElement>();
  const selectTool = /* Source: src/ui/app.ts:632-638 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (id: ToolId | "salt") => {
    tool = id;
    for (const [key, btn] of toolButtons) btn.classList.toggle("is-active", key === id);
    brushBarIcon.innerHTML = ICONS[id] ?? "";
    brushBarLabel.textContent = id === "salt" ? "Salt" : TOOLS[id as ToolId].name;
    renderPreview();
  };
  const toolRow = el("div", { class: "tool-row", role: "toolbar", "aria-label": "Brushes" });
  const toolIds: Array<ToolId | "salt"> = ["round", "mop", "rigger", "drybrush", "spatter", "water", "lift"];
  for (const id of toolIds) {
    const label = id === "salt" ? "Salt" : TOOLS[id as ToolId].name;
    const btn = iconButton(id, label, /* Source: src/ui/app.ts:643-643 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => selectTool(id));
    toolButtons.set(id, btn);
    toolRow.append(btn);
  }
  selectTool("round");

  const sizeCtl = slider("Size", size, /* Source: src/ui/app.ts:649-652 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => {
    size = v;
    renderPreview();
  });
  const waterCtl = slider("Water", water, /* Source: src/ui/app.ts:653-656 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => {
    water = v;
    renderPreview();
  });
  const loadCtl = slider("Pigment", load, /* Source: src/ui/app.ts:657-660 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => {
    load = v;
    renderPreview();
  });
  renderPreview();

  const paperRow = el("div", { class: "paper-row", role: "radiogroup", "aria-label": "Paper" });
  const paperButtons = new Map<PaperKind, HTMLButtonElement>();
  const selectPaper = /* Source: src/ui/app.ts:665-668 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (kind: PaperKind) => {
    sim.setPaper(kind);
    for (const [key, btn] of paperButtons) btn.setAttribute("aria-checked", String(key === kind));
  };
  for (const spec of Object.values(PAPERS)) {
    const btn = el(
      "button",
      { class: "btn btn-small", type: "button", role: "radio", "aria-checked": spec.kind === "cold-press", onclick: /* Source: src/ui/app.ts:672-672 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => selectPaper(spec.kind) },
      spec.name
    );
    paperButtons.set(spec.kind, btn);
    paperRow.append(btn);
  }

  // ---- alive modes -------------------------------------------------------
  const foreverWetBtn = el(
    "button",
    {
      class: "btn btn-small",
      type: "button",
      "aria-pressed": "false",
      title: "Evaporation off: the sheet stays workable forever",
      onclick: /* Source: src/ui/app.ts:687-691 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
        sim.foreverWet = !sim.foreverWet;
        foreverWetBtn.setAttribute("aria-pressed", String(sim.foreverWet));
        foreverWetBtn.classList.toggle("is-active", sim.foreverWet);
      },
    },
    "Forever wet"
  );
  const rainBtn = el(
    "button",
    {
      class: "btn btn-small",
      type: "button",
      "aria-pressed": "false",
      title: "Clean water falls on the sheet and works your paint over",
      onclick: /* Source: src/ui/app.ts:702-706 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
        rain = !rain;
        rainBtn.setAttribute("aria-pressed", String(rain));
        rainBtn.classList.toggle("is-active", rain);
      },
    },
    "Rain"
  );

  // ---- x-ray ---------------------------------------------------------------
  // The state textures themselves, one field at a time. An inspection mode,
  // not an effect: every pixel is one number from the simulation, on a fixed
  // scale, so a wash reads the same shade from one moment to the next.
  const xrayRow = el("div", { class: "paper-row", role: "radiogroup", "aria-label": "X-ray view" });
  const xrayButtons = new Map<XrayField, HTMLButtonElement>();
  const xrayWhat = el("p", { class: "xray-what" });
  const xrayRamp = el(
    "div",
    { class: "xray-ramp", "aria-hidden": "true" },
    el("span", { class: "xray-ramp-bar" }),
    el("span", { class: "xray-ramp-ends" }, el("span", {}, "0"), el("span", { class: "xray-ramp-unit" }))
  );
  const setXray = /* Source: src/ui/app.ts:724-732 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (field: XrayField) => {
    xray = field;
    const spec = XRAY_FIELDS[field];
    for (const [key, btn] of xrayButtons) btn.setAttribute("aria-checked", String(key === field));
    xrayWhat.textContent = spec.what;
    xrayRamp.hidden = field === 0;
    (xrayRamp.querySelector(".xray-ramp-unit") as HTMLElement).textContent = `${spec.unit} · full ink past ${Number((spec.scale * 3).toPrecision(2))}`;
    document.body.classList.toggle("is-xray", field !== 0);
  };
  for (const key of [0, 1, 2, 3, 4, 5, 6] as XrayField[]) {
    const spec = XRAY_FIELDS[key];
    const btn = el(
      "button",
      { class: "btn btn-small", type: "button", role: "radio", "aria-checked": "false", onclick: /* Source: src/ui/app.ts:737-737 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => setXray(key) },
      spec.name
    );
    xrayButtons.set(key, btn);
    xrayRow.append(btn);
  }
  setXray(0);

  // ---- actions -----------------------------------------------------------
  const backlightBtn = el(
    "button",
    { class: "btn", type: "button", "aria-pressed": "false", onclick: /* Source: src/ui/app.ts:748-748 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => setBacklight(!backlight) },
    "Backlight"
  );
  const setBacklight = /* Source: src/ui/app.ts:751-756 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (on: boolean) => {
    backlight = on;
    backlightBtn.setAttribute("aria-pressed", String(on));
    backlightBtn.classList.toggle("is-active", on);
    document.body.classList.toggle("is-backlit", on);
  };

  const exportPNG = /* Source: src/ui/app.ts:758-767 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    sim.update();
    sim.render(backlight ? 1 : 0, xray);
    canvas.toBlob(/* Source: src/ui/app.ts:761-766 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (blob) => {
      if (!blob) return;
      const a = el("a", { href: URL.createObjectURL(blob), download: "watercolor.png" });
      a.click();
      setTimeout(/* Source: src/ui/app.ts:765-765 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => URL.revokeObjectURL(a.href), 5000);
    }, "image/png");
  };

  // Save dries the sheet first — the wet layer is not part of the document,
  // same as walking away from a real painting overnight.
  const saveBtn = el(
    "button",
    {
      class: "btn",
      type: "button",
      onclick: /* Source: src/ui/app.ts:776-802 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ async () => {
        saveBtn.textContent = "Drying…";
        (saveBtn as HTMLButtonElement).disabled = true;
        sim.stepMany(500, 28);
        const dried = sim.serializeDried();
        if (dried) {
          try {
            await saveSheet({
              width: sim.simWidth,
              height: sim.simHeight,
              paperKind: sim.paperKind,
              paperSeed: sim.paperSeed,
              dried,
              savedAt: Date.now(),
            });
            saveBtn.textContent = "Saved";
          } catch {
            saveBtn.textContent = "Save failed";
          }
        } else {
          saveBtn.textContent = "Save failed";
        }
        setTimeout(/* Source: src/ui/app.ts:798-801 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
          saveBtn.textContent = "Save sheet";
          (saveBtn as HTMLButtonElement).disabled = false;
        }, 1600);
      },
    },
    "Save sheet"
  );

  if (!scripted) {
    loadSheet().then(/* Source: src/ui/app.ts:808-816 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (rec) => {
      if (!rec || rec.width <= 0 || rec.height <= 0) return;
      if (rec.dried.length !== rec.width * rec.height * 4) return;
      // A sheet saved on a desk (landscape) restores on a phone (portrait)
      // and back: scaled to fit, centred, margins bare paper.
      const dried = fitDried(rec.dried, rec.width, rec.height, sim.simWidth, sim.simHeight);
      sim.setPaper(rec.paperKind, true, rec.paperSeed);
      sim.restoreDried(dried);
    });
  }

  // The actions are one strip under the sheet, in reach without a scroll:
  // the committing action filled, the rest outlined, Focus on the far end.
  const actions = el(
    "div",
    { class: "action-bar" },
    el("button", { class: "btn btn-primary", type: "button", onclick: /* Source: src/ui/app.ts:824-824 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.dryFast() }, "Dry the sheet"),
    backlightBtn,
    el("button", { class: "btn", type: "button", onclick: /* Source: src/ui/app.ts:826-826 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.undo() }, "Undo"),
    saveBtn,
    el("button", { class: "btn", type: "button", onclick: exportPNG }, "Export PNG"),
    confirmButton("Clear sheet", "Really clear?", /* Source: src/ui/app.ts:829-829 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.clearSheet())
  );

  // A box is a section on a desk and a collapsible on a phone, where the
  // bench is a column under the sheet and every closed box is one less
  // screen to scroll past. Brushes starts open; the rest closed, since the
  // paint strip above the bench already keeps color in reach.
  const box = /* Source: src/ui/app.ts:836-844 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (title: string, open: boolean, cls: string, ...children: (HTMLElement | null)[]): HTMLElement => {
    if (!phone) return el("section", { class: `box ${cls}` }, el("h2", { class: "box-title" }, title), ...children);
    return el(
      "details",
      { class: `box ${cls}`, open: open ? true : null },
      el("summary", { class: "box-title" }, title),
      ...children
    );
  };
  const subRow = /* Source: src/ui/app.ts:845-845 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (label: string, row: HTMLElement) => el("div", { class: "sub-row" }, el("h3", { class: "sub-title" }, label), row);

  // On a phone the paint strip above the bench holds the pans, so the
  // Pigments box (mixing, recents, the cards) starts closed and Brushes,
  // the box the strip cannot stand in for, leads the column.
  const pigmentsBox = box("Pigments", false, "box-pigments", wellRow, recentRow, palette.root);
  const brushesBox = box(
    "Brushes",
    true,
    "box-brushes",
    toolRow,
    el(
      "div",
      { class: "brush-controls" },
      el("div", { class: "slider-col" }, sizeCtl.root, waterCtl.root, loadCtl.root),
      el("div", { class: "brush-preview" }, previewCanvas, previewLine)
    )
  );
  const paperBox = box("Paper", false, "box-paper", paperRow, subRow("Alive", el("div", { class: "paper-row" }, foreverWetBtn, rainBtn)));
  const xrayBox = box("X-ray", false, "box-xray", xrayRow, xrayWhat, xrayRamp);
  const bench = el(
    "aside",
    { class: "bench" },
    ...(phone
      ? // The two actions a painting needs mid-stroke (undo, dry) ride the
        // thumb bar; the rest wait in Sheet actions.
        [brushesBox, pigmentsBox, paperBox, xrayBox, box("Sheet actions", false, "box-actions", actions)]
      : [pigmentsBox, brushesBox, paperBox, xrayBox])
  );

  const engineButtons = (["A", "B"] as const).map((label) => {
    const button = el("button", { class: `btn${label === "A" ? " is-active" : ""}`, type: "button", "aria-pressed": label === "A" ? "true" : "false", onclick: () => {
      comparison.select(label);
      stroke = null;
      for (const other of engineButtons) {
        const selected = other === button;
        other.classList.toggle("is-active", selected);
        other.setAttribute("aria-pressed", String(selected));
      }
    } }, `Engine ${label}`);
    return button;
  });
  const comparisonBar = el("div", { class: "comparison-bar" },
    el("div", { class: "engine-buttons", role: "group", "aria-label": "Blind engine comparison" }, ...engineButtons,
      el("button", { class: "btn", type: "button", onclick: () => comparison.reveal() }, "Reveal")),
    el("p", {}, "Switching engines clears the sheet. Paper, brush, paint and display settings stay the same."),
    el("p", {}, "Salt is unavailable in both options because its grain patterns cannot be matched reliably."),
    el("p", {}, `Both engines use a ${dims.w} × ${dims.h} simulation grid on the same size sheet. The grid is reduced for interactive CPU speed; paper grain and fine edges may look coarser. Each frame runs the original three substeps, so the simulation clock slows when frame rate drops. Speed and stroke lag may still identify an engine. Reveal prints the mapping only in the console.`));
  const easel = el("div", { class: "easel" }, canvas, ring, hint, phone ? null : focusBtn);

  // ---- the thumb bar (phone) ------------------------------------------------
  // The canvas gets the screen and the four most-reached controls sit fixed
  // in the thumb zone. Paints and Brush go to their box in the page below,
  // opened if it was closed: the bench is ordinary page flow, one scroll
  // away, not a drawer over the painting.
  const openBox = /* Source: src/ui/app.ts:882-886 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (target: HTMLElement) => {
    if (target instanceof HTMLDetailsElement) target.open = true;
    const top = target.getBoundingClientRect().top + window.scrollY - 8;
    window.scrollTo({ top, behavior: "smooth" });
  };
  const barButton = /* Source: src/ui/app.ts:887-888 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (label: string, cls: string, onclick: () => void, ...lead: HTMLElement[]) =>
    el("button", { class: `bar-btn ${cls}`, type: "button", onclick }, ...lead, el("span", { class: "bar-label" }, label));
  const barIcon = /* Source: src/ui/app.ts:889-893 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (name: string) => {
    const span = el("span", { class: "bar-icon", "aria-hidden": "true" });
    span.innerHTML = ICONS[name] ?? "";
    return span;
  };
  const brushBarBtn = el(
    "button",
    { class: "bar-btn", type: "button", onclick: /* Source: src/ui/app.ts:896-896 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => openBox(brushesBox) },
    brushBarIcon,
    brushBarLabel
  );
  const sheetBar = phone
    ? el(
        "div",
        { class: "sheet-bar", role: "toolbar", "aria-label": "Painting controls" },
        barButton("Paints", "", /* Source: src/ui/app.ts:904-904 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => openBox(pigmentsBox), barWell),
        brushBarBtn,
        barButton("Undo", "", /* Source: src/ui/app.ts:906-906 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.undo(), barIcon("undo")),
        barButton("Dry", "bar-btn-primary", /* Source: src/ui/app.ts:907-907 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.dryFast(), barIcon("dry"))
      )
    : null;

  if (phone) {
    document.body.classList.add("is-phone");
    host.append(el("div", { class: "studio container" }, comparisonBar, easel, paintStrip, bench));
    document.body.append(sheetBar!);
    // The strip is only now in the DOM: run the sync once more so the
    // starting pigment's ringed chip lands in view.
    requestAnimationFrame(/* Source: src/ui/app.ts:917-917 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => syncStrip());
  } else {
    host.append(el("div", { class: "studio container" }, comparisonBar, easel, actions, bench, focusBar));
  }

  // ---- sheet sizing -------------------------------------------------------
  // The biggest sheet of the right aspect that fits the desk's width and,
  // on a desk, leaves the action strip and the bench in view beneath it: the
  // whole studio in one screen, no scrolling to reach a control. In focus
  // the window is the desk. A canvas cannot be left to CSS for this: a
  // replaced element with an explicit width does not give height back to
  // width when a max-height bites, it stretches.
  const wideDesk = window.matchMedia("(min-width: 1180px)");
  wideDesk.addEventListener("change", /* Source: src/ui/app.ts:930-930 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => layoutSheet());
  const studioPadX = /* Source: src/ui/app.ts:931-934 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    const st = getComputedStyle(easel.parentElement as HTMLElement);
    return parseFloat(st.paddingLeft) + parseFloat(st.paddingRight);
  };
  const layoutSheet = /* Source: src/ui/app.ts:935-975 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    const aspect = dims.w / dims.h;
    const style = getComputedStyle(easel);
    const padX = parseFloat(style.paddingLeft) + parseFloat(style.paddingRight);
    const padY = parseFloat(style.paddingTop) + parseFloat(style.paddingBottom);
    // The desk wraps the sheet, so the width on offer is the studio's, not
    // the desk's own (which would be circular); in focus it is the window's.
    const room = focus
      ? window.innerWidth
      : portrait
        ? easel.clientWidth // the desk is stretched edge to edge on a phone
        : (easel.parentElement as HTMLElement).clientWidth - studioPadX();
    const availW = Math.max(120, room - padX - 2);
    let capH: number;
    if (focus) {
      // The strip and the focus buttons sit under the sheet, not over it.
      // On a phone only the focus buttons do; the strip lives in the drawer.
      capH = window.innerHeight - padY - (portrait ? 76 : 64);
    } else if (portrait) {
      // A phone: the sheet leads the page, and the first screen holds the
      // sheet, the paint strip beneath it, and the fixed thumb bar, so a
      // stroke, a color and undo are never a scroll apart.
      const barH = sheetBar ? Math.max(64, sheetBar.offsetHeight) : 64;
      const stripH = paintStrip ? paintStrip.offsetHeight + 14 : 0;
      capH = Math.max(320, window.innerHeight - barH - stripH - padY - 16);
    } else if (wideDesk.matches) {
      // A desk wide enough for the four-column bench: the sheet is as wide
      // as the bench beneath it. That is Ryan's call over the one-screen
      // rule (a full-width 3:2 sheet is taller than a laptop's screen; the
      // bench is one scroll below, and Focus is there for the sheet alone).
      capH = Infinity;
    } else {
      // A narrow desk: room for the strip and the bench under the sheet, so
      // the whole studio holds in one screen.
      const below = actions.offsetHeight + bench.offsetHeight + 16 * 2 + padY + 4;
      capH = Math.max(320, window.innerHeight - below);
    }
    const w = Math.min(availW, capH * aspect);
    canvas.style.width = `${Math.floor(w)}px`;
    canvas.style.height = `${Math.floor(w / aspect)}px`;
  };
  new ResizeObserver(layoutSheet).observe(easel.parentElement as HTMLElement);
  new ResizeObserver(layoutSheet).observe(bench);
  window.addEventListener("resize", layoutSheet);
  layoutSheet();
  fitCanvas();

  // ---- loop --------------------------------------------------------------
  let frameCount = 0;
  const frame = /* Source: src/ui/app.ts:984-1007 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    fitCanvas();
    if (previewDirty) {
      previewDirty = false;
      renderPreview();
    }
    if (rain && frameCount++ % 10 === 0) {
      const drops = [];
      for (let i = 0; i < 2; i++) {
        drops.push({
          x: Math.random() * dims.w,
          y: Math.random() * dims.h,
          radius: (3 + Math.random() * 7) * refScale,
          water: 0.5 + Math.random(),
          pigment: 0,
          dryness: 0,
        });
      }
      sim.splat(drops, null);
    }
    sim.update();
    sim.render(backlight ? 1 : 0, xray);
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);

  // ---- scripted API ------------------------------------------------------
  const api: AppApi = {
    sim,
    setTool: selectTool,
    setPigment: /* Source: src/ui/app.ts:1014-1021 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (id) => {
      const p = PIGMENT_BY_ID.get(id);
      if (p) {
        well = [{ pigment: p, parts: 1 }];
        brushPigment = mixWell(well);
        renderWell();
      }
    },
    setSize: /* Source: src/ui/app.ts:1022-1022 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => sizeCtl.set(v),
    setWater: /* Source: src/ui/app.ts:1023-1023 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => waterCtl.set(v),
    setLoad: /* Source: src/ui/app.ts:1024-1024 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => loadCtl.set(v),
    setPaper: selectPaper,
    setBacklight,
    setXray,
    /* Source: src/ui/app.ts:1028-1038 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ strokePath(points, stepsPerSegment = 2) {
      if (points.length === 0 || tool === "salt") return;
      sim.snapshot();
      const pts = points.map(/* Source: src/ui/app.ts:1031-1031 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (p) => ({ x: p.x * refScale, y: p.y * refScale }));
      const s = beginStroke(TOOLS[tool as ToolId], size, water, load, pts[0].x, pts[0].y, 12345, refScale);
      for (let i = 1; i < pts.length; i++) {
        const stamps = strokeTo(s, pts[i].x, pts[i].y, 0.55);
        if (stamps.length > 0) sim.splat(stamps, s.tool.pigment > 0 ? brushPigment : null);
        sim.stepMany(stepsPerSegment);
      }
    },
    salt: /* Source: src/ui/app.ts:1039-1039 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (x, y) => sim.addSalt(x * refScale, y * refScale, 46 * refScale, 424242),
    dry: /* Source: src/ui/app.ts:1040-1040 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.dryFast(),
    undo: /* Source: src/ui/app.ts:1041-1041 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.undo(),
    clear: /* Source: src/ui/app.ts:1042-1042 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => sim.clearSheet(),
    step: /* Source: src/ui/app.ts:1043-1043 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (n, evapMul = 1) => sim.stepMany(n, evapMul),
  };
  return api;
}
