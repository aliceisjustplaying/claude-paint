/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Fitting a saved dried layer onto a sheet of different dimensions. Sheets are
 * landscape on a desk and portrait on a phone, so a painting saved on one
 * device must be resampled to restore on the other. The painting is scaled to
 * fit (aspect preserved, never stretched, never rotated), centered, and the
 * margins are bare paper — the same reflectance a fresh sheet starts with.
 */

import { PAPER_REFLECTANCE } from "../paint/km";

/** Where a source sheet lands inside a destination sheet, in destination
 * pixels. Pure arithmetic, exposed for tests. */
/* Source: src/data/resample.ts:13-18 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function fitRect(sw: number, sh: number, dw: number, dh: number): { x: number; y: number; w: number; h: number; scale: number } {
  const scale = Math.min(dw / sw, dh / sh);
  const w = Math.max(1, Math.round(sw * scale));
  const h = Math.max(1, Math.round(sh * scale));
  return { x: Math.floor((dw - w) / 2), y: Math.floor((dh - h) / 2), w, h, scale };
}

/**
 * Resamples an RGBA float32 dried layer of `sw`×`sh` onto a `dw`×`dh` sheet.
 * Bilinear, aspect-preserving contain fit, paper-filled margins. Returns the
 * input untouched when the dimensions already match.
 */
/* Source: src/data/resample.ts:25-61 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function fitDried(src: Float32Array, sw: number, sh: number, dw: number, dh: number): Float32Array {
  if (sw === dw && sh === dh) return src;
  const [pr, pg, pb] = PAPER_REFLECTANCE;
  const out = new Float32Array(dw * dh * 4);
  for (let i = 0; i < dw * dh; i++) {
    out[i * 4] = pr;
    out[i * 4 + 1] = pg;
    out[i * 4 + 2] = pb;
    // alpha (optical depth) stays 0: bare paper.
  }
  const rect = fitRect(sw, sh, dw, dh);
  const inv = 1 / rect.scale;
  for (let y = 0; y < rect.h; y++) {
    // Sample at destination pixel centres mapped back into the source.
    const sy = Math.min(sh - 1, Math.max(0, (y + 0.5) * inv - 0.5));
    const y0 = Math.floor(sy);
    const y1 = Math.min(sh - 1, y0 + 1);
    const fy = sy - y0;
    for (let x = 0; x < rect.w; x++) {
      const sx = Math.min(sw - 1, Math.max(0, (x + 0.5) * inv - 0.5));
      const x0 = Math.floor(sx);
      const x1 = Math.min(sw - 1, x0 + 1);
      const fx = sx - x0;
      const i00 = (y0 * sw + x0) * 4;
      const i10 = (y0 * sw + x1) * 4;
      const i01 = (y1 * sw + x0) * 4;
      const i11 = (y1 * sw + x1) * 4;
      const o = ((rect.y + y) * dw + (rect.x + x)) * 4;
      for (let c = 0; c < 4; c++) {
        const top = src[i00 + c] * (1 - fx) + src[i10 + c] * fx;
        const bot = src[i01 + c] * (1 - fx) + src[i11 + c] * fx;
        out[o + c] = top * (1 - fy) + bot * fy;
      }
    }
  }
  return out;
}
