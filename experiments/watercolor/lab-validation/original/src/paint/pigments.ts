// MIT License, Copyright (c) 2026 Ryan McCarty.
// Pinned Watercolor Lab source: a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../../../THIRD_PARTY_NOTICES.md.
/**
 * The pigment library.
 *
 * Every entry is a real pigment (or a traditional convenience mixture) with
 * its Colour Index code, and its behavior in the simulator comes from the
 * numbers a paint manufacturer would print on the tube:
 *
 * - `scattering` sets opacity. It is the absolute scale of S in Kubelka-Munk:
 *   high for cadmiums and earths that hide the paper, near zero for the
 *   quinacridones and phthalos that stay glazes forever.
 * - `strength` is tinting strength — how far a brush-load goes. The phthalos
 *   and alizarin overpower a wash at concentrations where terre verte barely
 *   tints; it scales K and S together so the masstone is unchanged.
 * - `granulation` is how eagerly particles settle into the paper's valleys.
 *   Ultramarine and cerulean granulate hard; the synthetic organics not at all.
 * - `staining` is the fraction of deposit that binds to the fibers and will
 *   not lift once it lands. Phthalo stains; cobalt violet wipes off.
 * - `density` is settling rate in suspension — heavy pigments drop out of
 *   moving water sooner, which is why an earth wash is calm where a dye wash
 *   keeps travelling.
 *
 * The masstone hex is the pigment at full strength; K and S are derived from
 * it once at module load (see kmFromMasstone), so a swatch, a wash, and a
 * glaze all agree because they all come from the same two curves. Masstones
 * are keyed deliberately dull — pigment on paper never reaches screen
 * saturation, and the deep transparents (alizarin, dioxazine, the phthalos)
 * are keyed near-black the way a real masstone reads in the pan.
 */

import { hexToLinear, kmFromMasstone, type Vec3 } from "./km";

export interface Pigment {
  id: string;
  name: string;
  /** Colour Index code, e.g. PB29 — or "mix" for a traditional convenience blend. */
  code: string;
  /** Masstone (full-strength) color as sRGB hex. */
  hex: string;
  /** Absolute Kubelka-Munk scattering strength: ~0.05 transparent → ~2.5 opaque. */
  scattering: number;
  /** Tinting strength: scales K and S together, so a phthalo at the same
   * concentration goes much further than a terre verte. */
  strength: number;
  /** 0..1, particle settling into paper texture. */
  granulation: number;
  /** 0..1, fraction of deposit that will not re-lift. */
  staining: number;
  /** 0..1, settling rate out of suspension. */
  density: number;
  /** Derived: absorption per unit thickness, linear RGB. */
  k: Vec3;
  /** Derived: scattering per unit thickness, linear RGB. */
  s: Vec3;
}

interface PigmentSpec {
  id: string;
  name: string;
  code: string;
  hex: string;
  scattering: number;
  strength: number;
  granulation: number;
  staining: number;
  density: number;
}

/** Ordered as a paint box is: yellows around to violets, then earths and
 * neutrals — so the palette grid reads like a color wheel with the dirt and
 * the darks on the bottom row. */
const SPECS: PigmentSpec[] = [
  // — Yellows —
  { id: "hansa-yellow", name: "Hansa Yellow Light", code: "PY3", hex: "#f0dd18", scattering: 0.28, strength: 1.2, granulation: 0.0, staining: 0.55, density: 0.15 },
  { id: "cadmium-yellow", name: "Cadmium Yellow", code: "PY35", hex: "#f6b90a", scattering: 1.9, strength: 1.3, granulation: 0.1, staining: 0.25, density: 0.65 },
  { id: "indian-yellow", name: "Indian Yellow", code: "PY110", hex: "#d98a10", scattering: 0.12, strength: 1.6, granulation: 0.0, staining: 0.7, density: 0.1 },
  { id: "naples-yellow", name: "Naples Yellow", code: "PBr24", hex: "#eec87e", scattering: 2.3, strength: 1.2, granulation: 0.15, staining: 0.1, density: 0.7 },
  { id: "yellow-ochre", name: "Yellow Ochre", code: "PY43", hex: "#b8823f", scattering: 1.1, strength: 1.2, granulation: 0.55, staining: 0.2, density: 0.75 },
  { id: "green-gold", name: "Green Gold", code: "PY129", hex: "#7f7a24", scattering: 0.15, strength: 1.4, granulation: 0.0, staining: 0.6, density: 0.15 },
  // — Oranges and reds —
  { id: "cadmium-orange", name: "Cadmium Orange", code: "PO20", hex: "#e0731f", scattering: 1.9, strength: 1.3, granulation: 0.1, staining: 0.3, density: 0.65 },
  { id: "pyrrol-scarlet", name: "Pyrrol Scarlet", code: "PR255", hex: "#c72f1a", scattering: 0.35, strength: 1.6, granulation: 0.0, staining: 0.75, density: 0.2 },
  { id: "cadmium-red", name: "Cadmium Red", code: "PR108", hex: "#b52a2c", scattering: 1.8, strength: 1.3, granulation: 0.15, staining: 0.3, density: 0.7 },
  { id: "quinacridone-rose", name: "Quinacridone Rose", code: "PV19", hex: "#97203f", scattering: 0.08, strength: 1.8, granulation: 0.0, staining: 0.8, density: 0.1 },
  { id: "alizarin-crimson", name: "Alizarin Crimson", code: "PR83", hex: "#6e1423", scattering: 0.1, strength: 2.2, granulation: 0.0, staining: 0.75, density: 0.12 },
  { id: "quinacridone-burnt-orange", name: "Quin. Burnt Orange", code: "PO48", hex: "#8f4522", scattering: 0.15, strength: 1.5, granulation: 0.15, staining: 0.7, density: 0.2 },
  // — Violets and blues —
  { id: "cobalt-violet", name: "Cobalt Violet", code: "PV14", hex: "#8d4f85", scattering: 0.7, strength: 1.1, granulation: 0.75, staining: 0.05, density: 0.8 },
  { id: "dioxazine-violet", name: "Dioxazine Violet", code: "PV23", hex: "#2a163f", scattering: 0.1, strength: 2.5, granulation: 0.0, staining: 0.85, density: 0.12 },
  { id: "ultramarine-violet", name: "Ultramarine Violet", code: "PV15", hex: "#4f4386", scattering: 0.4, strength: 1.3, granulation: 0.65, staining: 0.1, density: 0.6 },
  { id: "french-ultramarine", name: "French Ultramarine", code: "PB29", hex: "#363a92", scattering: 0.45, strength: 1.6, granulation: 0.85, staining: 0.15, density: 0.6 },
  { id: "cobalt-blue", name: "Cobalt Blue", code: "PB28", hex: "#2f4da8", scattering: 0.7, strength: 1.4, granulation: 0.6, staining: 0.15, density: 0.7 },
  { id: "cerulean-blue", name: "Cerulean Blue", code: "PB35", hex: "#34718f", scattering: 2.3, strength: 1.3, granulation: 0.85, staining: 0.1, density: 0.85 },
  // Green shade means green shade: the G channel must survive, or a phthalo
  // wash meeting a yellow one makes black instead of the green it must.
  { id: "phthalo-blue", name: "Phthalo Blue (GS)", code: "PB15:3", hex: "#032f56", scattering: 0.07, strength: 3.0, granulation: 0.0, staining: 0.9, density: 0.08 },
  { id: "prussian-blue", name: "Prussian Blue", code: "PB27", hex: "#122b3b", scattering: 0.12, strength: 2.2, granulation: 0.05, staining: 0.8, density: 0.2 },
  { id: "indanthrone-blue", name: "Indanthrone Blue", code: "PB60", hex: "#26315e", scattering: 0.15, strength: 1.8, granulation: 0.1, staining: 0.7, density: 0.25 },
  // — Greens —
  { id: "phthalo-green", name: "Phthalo Green (BS)", code: "PG7", hex: "#063f33", scattering: 0.07, strength: 3.0, granulation: 0.0, staining: 0.9, density: 0.08 },
  { id: "viridian", name: "Viridian", code: "PG18", hex: "#2a6b52", scattering: 0.35, strength: 1.2, granulation: 0.6, staining: 0.15, density: 0.55 },
  { id: "sap-green", name: "Sap Green", code: "mix", hex: "#47571f", scattering: 0.18, strength: 1.6, granulation: 0.05, staining: 0.5, density: 0.2 },
  { id: "hookers-green", name: "Hooker's Green", code: "mix", hex: "#2f4f30", scattering: 0.2, strength: 1.6, granulation: 0.1, staining: 0.5, density: 0.25 },
  { id: "terre-verte", name: "Terre Verte", code: "PG23", hex: "#71805e", scattering: 0.5, strength: 1.0, granulation: 0.6, staining: 0.05, density: 0.7 },
  // — Earths —
  { id: "raw-sienna", name: "Raw Sienna", code: "PBr7", hex: "#9c6428", scattering: 0.55, strength: 1.3, granulation: 0.6, staining: 0.2, density: 0.7 },
  { id: "burnt-sienna", name: "Burnt Sienna", code: "PBr7", hex: "#7a3b22", scattering: 0.5, strength: 1.4, granulation: 0.6, staining: 0.25, density: 0.7 },
  { id: "raw-umber", name: "Raw Umber", code: "PBr7", hex: "#655136", scattering: 0.55, strength: 1.3, granulation: 0.65, staining: 0.2, density: 0.75 },
  { id: "burnt-umber", name: "Burnt Umber", code: "PBr7", hex: "#5b3a25", scattering: 0.55, strength: 1.4, granulation: 0.6, staining: 0.25, density: 0.75 },
  { id: "indian-red", name: "Indian Red", code: "PR101", hex: "#7f3a33", scattering: 1.7, strength: 1.3, granulation: 0.7, staining: 0.3, density: 0.9 },
  { id: "buff-titanium", name: "Buff Titanium", code: "PW6:1", hex: "#e6d5b2", scattering: 2.4, strength: 1.1, granulation: 0.55, staining: 0.05, density: 0.8 },
  // — Neutrals and darks —
  { id: "sepia", name: "Sepia", code: "mix", hex: "#4a3423", scattering: 0.35, strength: 1.5, granulation: 0.35, staining: 0.4, density: 0.5 },
  { id: "paynes-gray", name: "Payne's Gray", code: "mix", hex: "#31404c", scattering: 0.25, strength: 1.5, granulation: 0.35, staining: 0.5, density: 0.4 },
  { id: "neutral-tint", name: "Neutral Tint", code: "mix", hex: "#37323e", scattering: 0.2, strength: 1.8, granulation: 0.15, staining: 0.6, density: 0.3 },
  { id: "lamp-black", name: "Lamp Black", code: "PBk6", hex: "#22211f", scattering: 0.5, strength: 1.6, granulation: 0.3, staining: 0.5, density: 0.45 },
];

export const PIGMENTS: Pigment[] = SPECS.map(/* Source: src/paint/pigments.ts:118–121 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */
(spec) => {
  const { k, s } = kmFromMasstone(hexToLinear(spec.hex), spec.scattering, spec.strength);
  return { ...spec, k, s };
});

export const PIGMENT_BY_ID = new Map(PIGMENTS.map(/* Source: src/paint/pigments.ts:123–123 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */
(p) => [p.id, p]));

export const DEFAULT_PIGMENT_ID = "french-ultramarine";

/** A one-word handling note for the palette tooltip, in the vocabulary a
 * tube label uses. */
/* Source: src/paint/pigments.ts:129–135 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */
export function handlingNote(p: Pigment): string {
  const notes: string[] = [];
  notes.push(p.scattering >= 1.2 ? "opaque" : p.scattering >= 0.4 ? "semi-transparent" : "transparent");
  if (p.granulation >= 0.5) notes.push("granulating");
  if (p.staining >= 0.65) notes.push("staining");
  return notes.join(" · ");
}

/** The declared numbers, in the order a tube label lists them, each with the
 * word a painter would use for it. `value` is the raw number; `level` is
 * where it sits on its own scale, 0..1, for a meter. Tinting strength runs
 * roughly 0.8 (terre verte) to 2 (the phthalos), scattering 0.05 to 2.5. */
export interface PigmentFact {
  key: "transparency" | "strength" | "granulation" | "staining" | "density";
  label: string;
  value: number;
  level: number;
  word: string;
}

/* Source: src/paint/pigments.ts:149–188 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */
export function pigmentFacts(p: Pigment): PigmentFact[] {
  const clamp01 = /* Source: src/paint/pigments.ts:150–150 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */
(v: number) => Math.min(1, Math.max(0, v));
  return [
    {
      key: "transparency",
      label: "Opacity",
      value: p.scattering,
      level: clamp01(p.scattering / 2.5),
      word: p.scattering >= 1.2 ? "opaque" : p.scattering >= 0.4 ? "semi-transparent" : "transparent",
    },
    {
      key: "strength",
      label: "Tinting strength",
      value: p.strength,
      level: clamp01((p.strength - 0.8) / 1.2),
      word: p.strength >= 1.7 ? "very strong" : p.strength >= 1.3 ? "strong" : p.strength >= 1.0 ? "moderate" : "weak",
    },
    {
      key: "granulation",
      label: "Granulation",
      value: p.granulation,
      level: clamp01(p.granulation),
      word: p.granulation >= 0.7 ? "heavy" : p.granulation >= 0.5 ? "granulating" : p.granulation >= 0.2 ? "slight" : "smooth",
    },
    {
      key: "staining",
      label: "Staining",
      value: p.staining,
      level: clamp01(p.staining),
      word: p.staining >= 0.65 ? "staining" : p.staining >= 0.35 ? "moderate" : "lifts clean",
    },
    {
      key: "density",
      label: "Settling",
      value: p.density,
      level: clamp01(p.density),
      word: p.density >= 0.7 ? "sinks fast" : p.density >= 0.4 ? "settles" : "stays afloat",
    },
  ];
}

/** One sentence on how the pigment handles, composed from the same numbers
 * that drive the physics — so the card can never promise a behaviour the
 * wash does not have. Three clauses, always in the same order: how it
 * covers, how it moves, how it lifts. */
/* Source: src/paint/pigments.ts:194–217 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */
export function handlingSentence(p: Pigment): string {
  const cover =
    p.scattering >= 1.2
      ? "Opaque: it hides the paper and lightens what it glazes over"
      : p.scattering >= 0.4
        ? "Semi-transparent: a wash lets the sheet through, a heavy load does not"
        : "Transparent: every glaze shows what is under it";
  const move =
    p.granulation >= 0.5 && p.density >= 0.7
      ? "settles fast and hard into the tooth, so a wash calms and granulates"
      : p.granulation >= 0.5
        ? "granulates into the tooth as it dries"
        : p.density <= 0.3
          ? "stays in the water and keeps travelling, so blooms and backruns run far"
          : "flows evenly and dries smooth";
  const lift =
    p.staining >= 0.65
      ? "and it stains: once settled it will not lift"
      : p.staining >= 0.35
        ? "and it lifts partly from damp paper"
        : "and it lifts clean from damp paper";
  const strength = p.strength >= 1.7 ? " A little goes a long way." : p.strength < 1.0 ? " It is weak in the well; load the brush." : "";
  return `${cover}; ${move}, ${lift}.${strength}`;
}
