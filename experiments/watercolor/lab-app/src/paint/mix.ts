/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * The mixing well. A brush charge is one or more dips of real pigments, and
 * because Kubelka-Munk K and S are linear in concentration, the mix is just
 * the parts-weighted average of the component spectra — the same arithmetic
 * the suspension textures do when two strokes meet in the water. The
 * handling properties average the same way, which is approximately what a
 * physical mixed wash does: ultramarine cut with phthalo granulates less
 * than ultramarine alone.
 */

import { linearToSrgbByte, masstone, type Vec3 } from "./km";
import type { Pigment } from "./pigments";

export interface WellEntry {
  pigment: Pigment;
  parts: number;
}

/** Collapses the well into one virtual pigment the splat pass can use. */
/* Source: src/paint/mix.ts:20-63 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function mixWell(entries: WellEntry[]): Pigment | null {
  if (entries.length === 0) return null;
  if (entries.length === 1) return entries[0].pigment;

  const total = entries.reduce(/* Source: src/paint/mix.ts:24-24 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (sum, e) => sum + e.parts, 0);
  const k: Vec3 = [0, 0, 0];
  const s: Vec3 = [0, 0, 0];
  let granulation = 0;
  let staining = 0;
  let density = 0;
  let scattering = 0;
  let strength = 0;
  for (const { pigment, parts } of entries) {
    const w = parts / total;
    for (let c = 0; c < 3; c++) {
      k[c] += pigment.k[c] * w;
      s[c] += pigment.s[c] * w;
    }
    granulation += pigment.granulation * w;
    staining += pigment.staining * w;
    density += pigment.density * w;
    scattering += pigment.scattering * w;
    strength += pigment.strength * w;
  }

  const inf = masstone(k, s);
  const hex = `#${[0, 1, 2]
    .map(/* Source: src/paint/mix.ts:47-47 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (c) => linearToSrgbByte(inf[c]).toString(16).padStart(2, "0"))
    .join("")}`;

  return {
    id: "mix",
    name: wellName(entries),
    code: "mix",
    hex,
    scattering,
    strength,
    granulation,
    staining,
    density,
    k,
    s,
  };
}

/** "Ultramarine + Burnt Sienna" — parts shown only when uneven. */
/* Source: src/paint/mix.ts:66-72 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function wellName(entries: WellEntry[]): string {
  if (entries.length === 0) return "Rinsed";
  const uneven = entries.some(/* Source: src/paint/mix.ts:68-68 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => e.parts !== entries[0].parts);
  return entries
    .map(/* Source: src/paint/mix.ts:70-70 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (e) => (uneven ? `${e.pigment.name} ×${e.parts}` : e.pigment.name))
    .join(" + ");
}
