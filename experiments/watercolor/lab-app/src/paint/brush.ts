/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Stroke dynamics: pointer events in, splat stamps out.
 *
 * The simulator only understands one thing — a stamp of water and pigment at
 * a point — so everything a brush *is* lives here: how stamps are spaced
 * along the path, how pressure and speed shape the footprint, and how the
 * load runs out. This file is pure math over coordinates, which is what
 * makes brushes testable without a GPU.
 */

export type ToolId = "round" | "mop" | "rigger" | "drybrush" | "spatter" | "water" | "lift";

export interface ToolSpec {
  id: ToolId;
  name: string;
  /** Multiplier on the user's size slider. */
  sizeScale: number;
  /** Water per stamp, before the water slider. */
  water: number;
  /** Pigment per stamp, before the load slider. */
  pigment: number;
  /** Stamp spacing as a fraction of radius: dense for wet brushes, sparse
   * and jittered for spatter. */
  spacing: number;
  /** 1 = only paints the tops of the paper grain (a starved brush dragged
   * fast); 0 = full contact. */
  dryness: number;
  /** How much of the load is spent per stamp — a mop holds a flood, a rigger
   * runs out in one gesture. */
  depletion: number;
  /** Random offset of each stamp, as a fraction of radius. */
  scatter: number;
}

export const TOOLS: Record<ToolId, ToolSpec> = {
  round:    { id: "round",    name: "Round",     sizeScale: 1.0,  water: 1.0,  pigment: 1.0,  spacing: 0.22, dryness: 0,   depletion: 0.004,  scatter: 0 },
  mop:      { id: "mop",      name: "Mop",       sizeScale: 2.4,  water: 1.8,  pigment: 0.55, spacing: 0.25, dryness: 0,   depletion: 0.002,  scatter: 0 },
  rigger:   { id: "rigger",   name: "Rigger",    sizeScale: 0.32, water: 0.55, pigment: 1.5,  spacing: 0.18, dryness: 0,   depletion: 0.009,  scatter: 0 },
  drybrush: { id: "drybrush", name: "Dry brush", sizeScale: 1.15, water: 0.12, pigment: 1.2,  spacing: 0.16, dryness: 1,   depletion: 0.012,  scatter: 0.08 },
  spatter:  { id: "spatter",  name: "Spatter",   sizeScale: 0.38, water: 0.9,  pigment: 1.2,  spacing: 2.6,  dryness: 0,   depletion: 0.006,  scatter: 3.2 },
  water:    { id: "water",    name: "Water",     sizeScale: 1.3,  water: 1.6,  pigment: 0,    spacing: 0.22, dryness: 0,   depletion: 0.0,    scatter: 0 },
  // A damp scrub: a little clean water plus a scrub field the transfer pass
  // reads, so unstained deposit re-suspends and can be dabbed away.
  lift:     { id: "lift",     name: "Lift",      sizeScale: 1.2,  water: 0.8,  pigment: 0,    spacing: 0.2,  dryness: 0,   depletion: 0.0,    scatter: 0 },
};

export interface Stamp {
  x: number;
  y: number;
  radius: number;
  /** Water volume added at this stamp. */
  water: number;
  /** Pigment concentration added, already scaled by the remaining load. */
  pigment: number;
  /** 0..1: gate deposition to the paper's peaks. */
  dryness: number;
}

export interface StrokeState {
  tool: ToolSpec;
  /** User sliders, all 0..1. */
  size: number;
  waterAmount: number;
  pigmentLoad: number;
  /** Remaining charge of paint on the brush, 1 at the dip. */
  reservoir: number;
  /** Leftover distance to the next stamp, carried between move events so
   * spacing is uniform across event boundaries. */
  carry: number;
  lastX: number;
  lastY: number;
  /** Deterministic scatter for spatter — a stroke is replayable. */
  seed: number;
  /** Radius multiplier, 1 at reference sim resolution — lets a scripted
   * stroke keep its composition on a smaller grid. */
  radiusScale: number;
}

/** Radius in sim-space pixels for the current sliders. The curve is eased so
 * the low end of the slider has room: a size-0.1 rigger line and a size-0.9
 * mop differ by ~40×, which is the working span of a real brush roll. */
/* Source: src/paint/brush.ts:82-85 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function brushRadius(tool: ToolSpec, size: number): number {
  const eased = 0.012 + 0.988 * size * size;
  return Math.max(1.1, eased * 68 * tool.sizeScale);
}

/* Source: src/paint/brush.ts:87-98 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function beginStroke(
  tool: ToolSpec,
  size: number,
  waterAmount: number,
  pigmentLoad: number,
  x: number,
  y: number,
  seed = 1,
  radiusScale = 1
): StrokeState {
  return { tool, size, waterAmount, pigmentLoad, reservoir: 1, carry: 0, lastX: x, lastY: y, seed, radiusScale };
}

/** Mulberry32 — tiny deterministic PRNG for scatter. */
/* Source: src/paint/brush.ts:101-107 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function rand(state: StrokeState): number {
  state.seed = (state.seed + 0x6d2b79f5) | 0;
  let t = state.seed;
  t = Math.imul(t ^ (t >>> 15), t | 1);
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
}

/* Source: src/paint/brush.ts:109-129 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function makeStamp(state: StrokeState, x: number, y: number, pressure: number): Stamp {
  const { tool } = state;
  const radius = brushRadius(tool, state.size) * state.radiusScale * (0.65 + 0.5 * pressure);
  // Water follows the slider linearly; pigment follows the slider *and* the
  // reservoir, so a long stroke exhausts to clean water the way a real brush
  // does — the tail of a stroke is always paler than the head.
  const water = tool.water * (0.25 + 0.95 * state.waterAmount) * (0.55 + 0.45 * state.reservoir);
  const pigment = tool.pigment * state.pigmentLoad * state.reservoir;
  state.reservoir = Math.max(0.15, state.reservoir - tool.depletion * (0.5 + state.pigmentLoad));

  let sx = x;
  let sy = y;
  if (tool.scatter > 0) {
    const angle = rand(state) * Math.PI * 2;
    const dist = Math.sqrt(rand(state)) * tool.scatter * radius;
    sx += Math.cos(angle) * dist;
    sy += Math.sin(angle) * dist;
  }
  const sizeJitter = tool.scatter > 0 ? 0.4 + 0.85 * rand(state) : 1;
  return { x: sx, y: sy, radius: radius * sizeJitter, water, pigment, dryness: tool.dryness };
}

/**
 * Advances the stroke to (x, y) and returns the stamps crossed on the way.
 * Spacing is measured along the true path with the remainder carried, so
 * stamp density is independent of how the browser batches pointer events.
 */
/* Source: src/paint/brush.ts:136-157 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function strokeTo(state: StrokeState, x: number, y: number, pressure = 0.5): Stamp[] {
  const stamps: Stamp[] = [];
  const dx = x - state.lastX;
  const dy = y - state.lastY;
  const dist = Math.hypot(dx, dy);
  const radius = brushRadius(state.tool, state.size) * state.radiusScale;
  const spacing = Math.max(0.9, radius * state.tool.spacing);

  if (dist <= 1e-6) return stamps;
  // `carry` is the distance already travelled since the previous stamp, so
  // the next stamp falls at (spacing − carry) along this segment.
  let along = spacing - state.carry;
  while (along <= dist) {
    const t = along / dist;
    stamps.push(makeStamp(state, state.lastX + dx * t, state.lastY + dy * t, pressure));
    along += spacing;
  }
  state.carry = dist - (along - spacing);
  state.lastX = x;
  state.lastY = y;
  return stamps;
}

/** The dot a click makes — a stroke of zero length still leaves a mark. */
/* Source: src/paint/brush.ts:160-162 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function tapStamp(state: StrokeState, pressure = 0.5): Stamp {
  return makeStamp(state, state.lastX, state.lastY, pressure);
}
