/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Paper.
 *
 * Everything distinctive about watercolor happens because the surface is not
 * flat: pigment settles into valleys (granulation), a starved brush touches
 * only the peaks (dry brush), and water runs downhill across the sheet's
 * texture. So the paper is a real height field, generated once per sheet and
 * read by almost every simulation pass.
 *
 * The field is fractal value noise for the felt texture, plus a faint
 * directional fiber component — cold-press paper is pressed against a felt,
 * and the felt has a grain. Hot press is the same recipe with the amplitude
 * turned down, rough with it turned up; the difference between papers really
 * is mostly amplitude.
 */

export type PaperKind = "hot-press" | "cold-press" | "rough";

export interface PaperSpec {
  kind: PaperKind;
  name: string;
  /** Texture amplitude, 0..1 of the working range. */
  tooth: number;
  /** Fiber streak strength. */
  fiber: number;
}

export const PAPERS: Record<PaperKind, PaperSpec> = {
  "hot-press": { kind: "hot-press", name: "Hot press", tooth: 0.3, fiber: 0.5 },
  "cold-press": { kind: "cold-press", name: "Cold press", tooth: 0.72, fiber: 1.0 },
  rough: { kind: "rough", name: "Rough", tooth: 1.0, fiber: 0.85 },
};

/** Deterministic hash → [0,1) for lattice noise. */
/* Source: src/engine/paper.ts:35-40 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function hash2(ix: number, iy: number, seed: number): number {
  let h = (ix * 374761393 + iy * 668265263 + seed * 2246822519) | 0;
  h = Math.imul(h ^ (h >>> 13), 1274126177);
  h ^= h >>> 16;
  return (h >>> 0) / 4294967296;
}

/* Source: src/engine/paper.ts:42-44 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function smooth(t: number): number {
  return t * t * (3 - 2 * t);
}

/** Value noise at (x, y) for one octave of cell size `scale`. */
/* Source: src/engine/paper.ts:47-59 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function valueNoise(x: number, y: number, scale: number, seed: number): number {
  const gx = x / scale;
  const gy = y / scale;
  const ix = Math.floor(gx);
  const iy = Math.floor(gy);
  const fx = smooth(gx - ix);
  const fy = smooth(gy - iy);
  const v00 = hash2(ix, iy, seed);
  const v10 = hash2(ix + 1, iy, seed);
  const v01 = hash2(ix, iy + 1, seed);
  const v11 = hash2(ix + 1, iy + 1, seed);
  return (v00 * (1 - fx) + v10 * fx) * (1 - fy) + (v01 * (1 - fx) + v11 * fx) * fy;
}

export interface PaperField {
  width: number;
  height: number;
  /** Interleaved RG float pairs: R = height 0..1 (0.5 = mean surface),
   * G = fine grain used for the visible speckle at display resolution. */
  data: Float32Array;
}

/**
 * Generates the sheet. The height channel carries the octaves that matter to
 * the physics (the scale a pigment particle or a brush hair cares about);
 * the grain channel carries the sub-pixel speckle that only the renderer
 * needs. Splitting them keeps the physics resolution-independent: simulating
 * at half size does not smooth the sheet's tooth away.
 */
/* Source: src/engine/paper.ts:76-112 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function generatePaper(width: number, height: number, spec: PaperSpec, seed = 7): PaperField {
  const data = new Float32Array(width * height * 2);
  const octaves = [
    // [cell size in px, amplitude]
    [34, 0.30],
    [16, 0.42],
    [7, 0.55],
    [3.2, 0.35],
  ] as const;
  let ampSum = 0;
  for (const [, amp] of octaves) ampSum += amp;

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      let h = 0;
      for (let o = 0; o < octaves.length; o++) {
        const [size, amp] = octaves[o];
        h += (valueNoise(x, y, size, seed + o * 101) - 0.5) * amp;
      }
      h /= ampSum;

      // Fiber grain: two crossed, slightly wavy directional streak fields,
      // like the felt marks on a real sheet. Stretched noise, not lines.
      const wave = valueNoise(x, y, 90, seed + 900) * 8;
      const f1 = valueNoise(x * 0.22, y * 1.6 + wave, 6, seed + 501) - 0.5;
      const f2 = valueNoise(x * 1.6 + wave, y * 0.22, 6, seed + 502) - 0.5;
      h += (f1 * 0.75 + f2 * 0.45) * 0.22 * spec.fiber;

      const idx = (y * width + x) * 2;
      data[idx] = Math.min(1, Math.max(0, 0.5 + h * spec.tooth));
      // Fine grain: single high-frequency octave, independent of tooth so
      // hot press still shows a whisper of surface under raking light.
      data[idx + 1] = valueNoise(x, y, 1.7, seed + 77);
    }
  }
  return { width, height, data };
}
