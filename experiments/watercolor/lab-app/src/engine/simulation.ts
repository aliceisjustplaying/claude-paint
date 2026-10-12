/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/** Every tunable in the physics, in one bag, so the whole feel of the medium
 * can be adjusted (or bisected, when a behavior goes wrong) without touching
 * a shader. These defaults are the result of the visual tuning passes logged
 * in docs/session-log.md. */
export interface Params {
  gravity: number;
  damp: number;
  slope: number;
  tension: number;
  saltPull: number;
  evap: number;
  evapEdge: number;
  absorb: number;
  capDiff: number;
  capDry: number;
  velScale: number;
  advect: number;
  edgeDrift: number;
  settle: number;
  granBias: number;
  lift: number;
  saltLift: number;
  /** Extra lift under the lift tool's scrub field. */
  scrubLift: number;
  /** Deposition multiplier at the drying front — the dried rim, the tide
   * line, the backrun's cauliflower edge. */
  edgeSettle: number;
  substeps: number;
  zoomGrain: number;
}

export const DEFAULT_PARAMS: Params = {
  gravity: 0.14,
  damp: 0.982,
  slope: 0.12,
  tension: 0.02,
  saltPull: 0.8,
  evap: 0.00016,
  evapEdge: 0.0016,
  absorb: 0.00045,
  capDiff: 0.04,
  capDry: 0.00025,
  velScale: 20,
  advect: 1.0,
  edgeDrift: 4.5,
  settle: 0.0018,
  granBias: 1.1,
  lift: 0.005,
  saltLift: 0.5,
  scrubLift: 0.9,
  edgeSettle: 14,
  substeps: 3,
  zoomGrain: 0.006,
}

/** The X-ray views: which state texture, and the value at which its ramp
 * reaches 63% (1 - 1/e) of full ink. Scales were set once against measured
 * fields of a heavy mop wash on cold press (readFloat over the whole sheet at
 * 0, 1, 3 and 8 s) so a typical wash sits mid-ramp; they are constants, never
 * fitted to the frame being drawn. */
export type XrayField = 0 | 1 | 2 | 3 | 4 | 5 | 6;
export const XRAY_FIELDS: Record<XrayField, { id: string; name: string; what: string; scale: number; unit: string }> = {
  0: { id: "paint", name: "Paint", what: "The painting as it looks.", scale: 1, unit: "" },
  1: { id: "water", name: "Water", what: "Standing water on the sheet: the film that flows and carries pigment.", scale: 0.12, unit: "depth" },
  2: { id: "wet", name: "Wet", what: "How wet the surface is, 0 to 1. The wash boundary is where it drops.", scale: 0.6, unit: "wetness" },
  3: { id: "suspended", name: "Suspended", what: "Pigment riding in the water, not yet settled.", scale: 0.15, unit: "concentration" },
  4: { id: "settled", name: "Settled", what: "Pigment settled on the sheet but not yet dried: what the lift tool can reach.", scale: 0.06, unit: "thickness" },
  5: { id: "paper", name: "Paper", what: "Water inside the paper's fibres, where backruns travel. Salt grains in vermillion.", scale: 0.03, unit: "saturation" },
  6: { id: "dried", name: "Dried", what: "Everything cured into the sheet, as optical depth. Permanent.", scale: 1.0, unit: "depth" },
};
