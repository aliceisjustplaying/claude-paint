/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Kubelka-Munk optics.
 *
 * Watercolor is not alpha blending. A wash is a turbid layer of pigment
 * particles in a dried gum film: light is absorbed (K) and scattered (S) on
 * the way down, reflects off the paper, and is absorbed and scattered again
 * on the way up. Kubelka-Munk solves that two-flux transport exactly for a
 * homogeneous layer, and it is why a glaze of ultramarine over burnt sienna
 * looks like a painting while `mix(blue, brown, 0.5)` looks like mud in a
 * paint program. This is the same model Curtis et al. used in
 * "Computer-Generated Watercolor" (SIGGRAPH 1997), applied per RGB channel.
 *
 * The formulas live here once, in TS for the palette swatches and the test
 * suite, and as a GLSL string (KM_GLSL below) for the render shader. The two
 * are written line-for-line against each other; the test suite pins the TS
 * side, and the GLSL side is a transcription kept adjacent so a change to one
 * is a diff touching both.
 */

export type Vec3 = [number, number, number];

/** Reflectance and transmittance of a single pigment layer.
 *
 * K and S are absorption and scattering per unit thickness, already summed
 * over the pigments in the layer (K and S are linear in concentration, which
 * is what makes physical mixing on wet paper come out right for free). x is
 * the layer thickness.
 */
/* Source: src/paint/km.ts:29-56 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function layerRT(k: Vec3, s: Vec3, x: number, outR: Vec3, outT: Vec3): void {
  for (let c = 0; c < 3; c++) {
    const K = Math.max(k[c], 0);
    const S = Math.max(s[c], 0);
    if (x <= 0 || (K <= 1e-9 && S <= 1e-9)) {
      // No layer: everything passes through.
      outR[c] = 0;
      outT[c] = 1;
      continue;
    }
    if (S <= 1e-6) {
      // Pure absorber (an ideal non-scattering glaze): Beer-Lambert limit.
      outR[c] = 0;
      outT[c] = Math.exp(-K * x);
      continue;
    }
    const a = 1 + K / S;
    const b = Math.sqrt(Math.max(a * a - 1, 1e-12));
    // y grows with optical depth; past ~40 the layer is opaque to double
    // precision and sinh/cosh overflow long before they disagree.
    const y = Math.min(b * S * x, 40);
    const sinhY = Math.sinh(y);
    const coshY = Math.cosh(y);
    const denom = a * sinhY + b * coshY;
    outR[c] = sinhY / denom;
    outT[c] = b / denom;
  }
}

/** Composites a layer (R1, T1) over a substrate of reflectance R2.
 *
 * The geometric series of light bouncing between the layer's underside and
 * the substrate sums to T1²R2 / (1 − R1R2).
 */
/* Source: src/paint/km.ts:63-67 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function composite(r1: Vec3, t1: Vec3, r2: Vec3, out: Vec3): void {
  for (let c = 0; c < 3; c++) {
    out[c] = r1[c] + (t1[c] * t1[c] * r2[c]) / (1 - Math.min(r1[c] * r2[c], 0.9999));
  }
}

/** The reflectance of an infinitely thick layer — a pigment's masstone.
 * R∞ = 1 / (a + b); used to derive K and S from a target color. */
/* Source: src/paint/km.ts:71-83 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function masstone(k: Vec3, s: Vec3): Vec3 {
  const out: Vec3 = [0, 0, 0];
  for (let c = 0; c < 3; c++) {
    if (s[c] <= 1e-9) {
      out[c] = 0;
      continue;
    }
    const a = 1 + k[c] / s[c];
    const b = Math.sqrt(Math.max(a * a - 1, 1e-12));
    out[c] = 1 / (a + b);
  }
  return out;
}

/**
 * Derives K and S from a pigment's masstone reflectance and a scattering
 * strength. The K/S ratio is fixed by the masstone alone — the classic
 * Kubelka-Munk relation K/S = (1−R∞)²/2R∞ — while the absolute scale of S
 * sets opacity: a cadmium hides the paper at S≈2, a quinacridone at S≈0.1
 * stays a glaze at any reasonable thickness.
 *
 * S is per-channel, weighted toward the channels the pigment reflects: real
 * pigments scatter selectively near their reflectance peak. With a flat S,
 * a thin wash is ruled by whichever channel is absorbed least and every blue
 * tints toward swimming-pool cyan while gaining chroma; weighting S by R∞
 * makes the backscatter carry the pigment's own hue, so tints walk toward
 * chalky paper-white along the pigment's hue line — which is what a real
 * cobalt or cerulean wash does. The per-channel K/S relation still holds
 * exactly, so the masstone round-trip in the test suite is untouched.
 *
 * `strength` is tinting strength: it scales K and S together, leaving the
 * masstone fixed while making a given concentration go further — how a
 * phthalo or an alizarin reaches a near-black masstone in heavy washes that
 * a weak pigment like terre verte never approaches.
 *
 * R∞ is clamped away from 0 and 1 because a literal 0 wants infinite
 * absorption and a literal 1 wants none, and real pigments are neither.
 */
/* Source: src/paint/km.ts:109-118 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function kmFromMasstone(rInf: Vec3, scattering: number, strength = 1): { k: Vec3; s: Vec3 } {
  const k: Vec3 = [0, 0, 0];
  const s: Vec3 = [0, 0, 0];
  for (let c = 0; c < 3; c++) {
    const r = Math.min(Math.max(rInf[c], 0.012), 0.985);
    s[c] = scattering * (0.35 + 0.65 * r) * strength;
    k[c] = (s[c] * (1 - r) * (1 - r)) / (2 * r);
  }
  return { k, s };
}

/** sRGB byte-hex to linear-light RGB, the space all KM math runs in. */
/* Source: src/paint/km.ts:121-125 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function hexToLinear(hex: string): Vec3 {
  const n = parseInt(hex.replace("#", ""), 16);
  const srgb = [(n >> 16) & 255, (n >> 8) & 255, n & 255].map(/* Source: src/paint/km.ts:123-123 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => v / 255);
  return srgb.map(/* Source: src/paint/km.ts:124-124 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (v) => (v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4))) as Vec3;
}

/* Source: src/paint/km.ts:127-131 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function linearToSrgbByte(v: number): number {
  const clamped = Math.min(Math.max(v, 0), 1);
  const srgb = clamped <= 0.0031308 ? clamped * 12.92 : 1.055 * Math.pow(clamped, 1 / 2.4) - 0.055;
  return Math.round(srgb * 255);
}

/**
 * The paper's own reflectance in linear light: the site's cream, slightly
 * duller than a gallery white, so an unpainted sheet sits on the page instead
 * of glowing against it. The render shader modulates this with the paper
 * texture; this constant is the flat-field value the swatches use.
 */
export const PAPER_REFLECTANCE: Vec3 = hexToLinear("#faf3e4");

/**
 * The same maths for the render shader, transcribed line for line from the
 * functions above. Vectorized per RGB channel; the branches become mix() on
 * step() masks because this runs per pixel per frame.
 */
export const KM_GLSL = /* glsl */ `
  // Kubelka-Munk: reflectance and transmittance of a pigment layer.
  // Mirrors layerRT() in src/paint/km.ts — keep the two in lockstep.
  void kmLayer(vec3 K, vec3 S, float x, out vec3 R, out vec3 T) {
    K = max(K, vec3(0.0));
    S = max(S, vec3(0.0));
    vec3 a = 1.0 + K / max(S, vec3(1e-6));
    vec3 b = sqrt(max(a * a - 1.0, 1e-12));
    vec3 y = min(b * S * x, vec3(40.0));
    vec3 ey = exp(y);
    vec3 einy = 1.0 / ey;
    vec3 sinhY = 0.5 * (ey - einy);
    vec3 coshY = 0.5 * (ey + einy);
    vec3 denom = a * sinhY + b * coshY;
    R = sinhY / denom;
    T = b / denom;
    // Pure-absorber limit for near-zero scattering (Beer-Lambert).
    vec3 absorber = step(S, vec3(1e-6));
    R = mix(R, vec3(0.0), absorber);
    T = mix(T, exp(-K * x), absorber);
    // No layer at all.
    float none = step(x, 0.0);
    R = mix(R, vec3(0.0), none);
    T = mix(T, vec3(1.0), none);
  }

  // Layer over substrate: R1 + T1^2 R2 / (1 - R1 R2).
  // Mirrors composite() in src/paint/km.ts.
  vec3 kmComposite(vec3 R1, vec3 T1, vec3 R2) {
    return R1 + T1 * T1 * R2 / (1.0 - min(R1 * R2, vec3(0.9999)));
  }
`;
