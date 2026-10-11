/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Every simulation pass, as GLSL.
 *
 * The scheme is the Curtis et al. (SIGGRAPH 1997) three-layer model with the
 * shallow-water layer solved as virtual pipes (a flux field between cells)
 * rather than their explicit velocity relaxation — pipes are unconditionally
 * stable at the step sizes an interactive brush needs, and the velocity the
 * pigment advection wants falls out of the net flux for free.
 *
 * State textures and channel conventions (all RGBA16F):
 *   flow  = (vx, vy, water, wetness)         surface water and its motion
 *   flux  = (left, right, down, up)          pipe outflows per direction
 *   susK  = (K·c rgb, concentration)         pigment in suspension
 *   susS  = (S·c rgb, –)
 *   susP  = (density·c, granulation·c, staining·c, –)   handling properties,
 *            concentration-weighted so mixing pigments averages them
 *   depK  = (K rgb, thickness)               pigment settled on the sheet
 *   depS  = (S rgb, staining·thickness)
 *   cap   = (saturation, salt, –, –)         water inside the paper; salt
 *   dried = (R rgb linear, optical depth)    everything already dried, folded
 *            into a single Kubelka-Munk substrate
 *   paper = (height, grain, –, –)            static sheet
 *
 * All rates are per simulation substep; the tunables live in Params
 * (simulation.ts) and arrive as uniforms so the visual critic loop can turn
 * knobs without touching shader text.
 */

import { KM_GLSL } from "../paint/km";

const HEADER = `#version 300 es
precision highp float;
in vec2 vUV;
uniform vec2 uTexel;
`;

/** 9-tap blur of the wetness channel. One pass, used for edge detection:
 * where blurred wetness falls below local wetness, we are at a wash boundary. */
export const BLUR_WET_FRAG = `${HEADER}
uniform sampler2D uFlow;
out vec4 outColor;
// Source: src/engine/shaders.ts:42-50 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  float sum = 0.0;
  for (int dy = -1; dy <= 1; dy++)
  for (int dx = -1; dx <= 1; dx++) {
    float w = (dx == 0 && dy == 0) ? 4.0 : ((dx == 0 || dy == 0) ? 2.0 : 1.0);
    sum += w * texture(uFlow, vUV + vec2(float(dx), float(dy)) * uTexel * 2.0).a;
  }
  outColor = vec4(sum / 16.0, 0.0, 0.0, 0.0);
}
`;

/** Pipe flux update: water accelerates between cells down the gradient of
 * total head (standing water + a contribution from the paper's relief). */
export const FLUX_FRAG = `${HEADER}
uniform sampler2D uFlow;
uniform sampler2D uFlux;
uniform sampler2D uPaper;
uniform sampler2D uCap;
uniform float uGravity;      // gain on head difference
uniform float uDamp;         // flux persistence (inertia)
uniform float uSlope;        // how much paper relief tilts the water
uniform float uTension;      // water needed before invading dry paper
uniform float uSaltPull;     // osmotic head drop at a salt grain

// Source: src/engine/shaders.ts:66-71 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
float head(vec2 uv) {
  vec4 f = texture(uFlow, uv);
  float paper = texture(uPaper, uv).r;
  float salt = texture(uCap, uv).g;
  return f.b + paper * uSlope - salt * uSaltPull;
}

// Source: src/engine/shaders.ts:73-85 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
float gate(vec2 uv, float pHere) {
  // Flow into dry paper is resisted until there is enough water behind it:
  // this is what keeps the edge of a wash crisp instead of feathering out
  // like ink on tissue. The threshold rides the paper's tooth, so the
  // boundary advances unevenly along the fibers — but a well-fed edge beads
  // smooth, so the fiber modulation fades where water is plentiful. One
  // uniform fringe on every mark is a signature; wetness history is not.
  float wetThere = texture(uFlow, uv).a;
  float fiberMix = clamp(1.0 - pHere * 5.0, 0.25, 1.0);
  float fiber = mix(1.0, 0.55 + 1.1 * texture(uPaper, uv).r, fiberMix);
  float invade = smoothstep(uTension * fiber, uTension * fiber * 2.5, pHere);
  return mix(invade, 1.0, step(0.05, wetThere));
}

out vec4 outColor;
// Source: src/engine/shaders.ts:88-114 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flux = texture(uFlux, vUV);
  vec4 flow = texture(uFlow, vUV);
  float h = head(vUV);
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);

  vec4 dh = vec4(
    h - head(vUV - tx),   // toward left
    h - head(vUV + tx),   // toward right
    h - head(vUV - ty),   // toward down
    h - head(vUV + ty)    // toward up
  );
  vec4 gates = vec4(gate(vUV - tx, flow.b), gate(vUV + tx, flow.b), gate(vUV - ty, flow.b), gate(vUV + ty, flow.b));
  vec4 next = max(vec4(0.0), flux * uDamp + uGravity * dh) * gates;

  // A cell cannot export more than the water it holds.
  float total = next.x + next.y + next.z + next.w;
  float scale = total > 1e-6 ? min(1.0, flow.b * 0.9 / total) : 0.0;
  next *= scale;
  // No flow off the sheet: border cells keep their water.
  if (vUV.x < uTexel.x) next.x = 0.0;
  if (vUV.x > 1.0 - uTexel.x) next.y = 0.0;
  if (vUV.y < uTexel.y) next.z = 0.0;
  if (vUV.y > 1.0 - uTexel.y) next.w = 0.0;
  outColor = next;
}
`;

/** Water update: apply flux divergence, evaporation, capillary absorption;
 * derive the advection velocity from net flux; maintain the wet mask. */
export const WATER_FRAG = `${HEADER}
uniform sampler2D uFlow;
uniform sampler2D uFlux;
uniform sampler2D uBlurW;
uniform sampler2D uCap;
uniform float uEvap;         // base evaporation per step
uniform float uEvapEdge;     // extra evaporation at wash boundaries
uniform float uEvapMul;      // hairdryer / fast-forward multiplier
uniform float uAbsorb;       // surface -> paper interior
uniform float uVelScale;     // flux -> px/step velocity
out vec4 outColor;
// Source: src/engine/shaders.ts:130-161 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec4 f = texture(uFlux, vUV);
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  float inL = texture(uFlux, vUV - tx).y;
  float inR = texture(uFlux, vUV + tx).x;
  float inD = texture(uFlux, vUV - ty).w;
  float inU = texture(uFlux, vUV + ty).z;

  float p = flow.b + (inL + inR + inD + inU) - (f.x + f.y + f.z + f.w);

  // Evaporation, faster where the film is thin and at the boundary of the
  // wet region — the drying front eats inward, and the pigment it strands
  // there is the classic hard watercolor edge.
  float edge = clamp(flow.a - texture(uBlurW, vUV).r, 0.0, 1.0);
  p -= uEvap * uEvapMul + uEvapEdge * uEvapMul * edge;

  // The sheet drinks: absorption into the capillary layer until saturated.
  float sat = texture(uCap, vUV).r;
  p -= min(p, uAbsorb * (1.0 - sat));
  p = max(p, 0.0);

  vec2 vel = uVelScale * 0.5 * vec2((f.y - f.x) + (inL - inR), (f.w - f.z) + (inD - inU));

  // Wetness: saturates instantly with standing water, then decays as a damp
  // memory once the water is gone — a bloom pushed into damp paper still
  // creeps, which is exactly what a backrun is.
  float wet = clamp(flow.a, 0.0, 1.0);
  wet = max(wet * (p > 0.002 ? 1.0 : 0.985), min(1.0, p * 200.0));
  outColor = vec4(vel, p, wet);
}
`;

/** Move suspended pigment through the water: semi-Lagrangian advection along
 * the flow, plus a drift toward the drying edge (the coffee-ring current that
 * evaporation at the boundary induces in the real fluid). */
export const MOVE_PIGMENT_FRAG = `${HEADER}
uniform sampler2D uFlow;
uniform sampler2D uBlurW;
uniform sampler2D uSusK;
uniform sampler2D uSusS;
uniform sampler2D uSusP;
uniform float uAdvect;      // fraction of a cell moved per step at unit velocity
uniform float uEdgeDrift;   // strength of the outward drying-edge current
layout(location = 0) out vec4 outK;
layout(location = 1) out vec4 outS;
layout(location = 2) out vec4 outP;
// Source: src/engine/shaders.ts:178-195 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  // Outward = down the gradient of blurred wetness.
  vec2 gradW = vec2(
    texture(uBlurW, vUV + tx).r - texture(uBlurW, vUV - tx).r,
    texture(uBlurW, vUV + ty).r - texture(uBlurW, vUV - ty).r
  );
  float shallow = smoothstep(0.12, 0.0, flow.b); // the ring current lives in thin films
  vec2 vel = flow.xy - gradW * uEdgeDrift * shallow * flow.a;
  vec2 src = vUV - vel * uAdvect * uTexel;
  float moving = step(0.01, flow.a);
  vec2 at = mix(vUV, src, moving);
  outK = texture(uSusK, at);
  outS = texture(uSusS, at);
  outP = texture(uSusP, at);
}
`;

/** The deposition/lift exchange, shared verbatim by the two transfer passes
 * so both sides of the ledger compute identical deltas from identical inputs. */
const TRANSFER_COMMON = `
uniform sampler2D uFlow;
uniform sampler2D uPaper;
uniform sampler2D uCap;
uniform sampler2D uBlurW;
uniform sampler2D uSusK;
uniform sampler2D uSusS;
uniform sampler2D uSusP;
uniform sampler2D uDepK;
uniform sampler2D uDepS;
uniform float uSettle;     // base settling rate
uniform float uGranBias;   // extra settling into paper valleys for granulators
uniform float uLift;       // base re-suspension rate
uniform float uSaltLift;   // extra lift at a salt grain
uniform float uScrubLift;  // extra lift under the lift tool's scrub
uniform float uEdgeSettle; // deposition boost at the boundary of the wet region

struct Exchange { float down; float up; };

Exchange exchange(vec4 flow, vec4 susK, vec4 susP, vec4 depK, vec4 depS, float paperH, float salt, float scrub, float edge) {
  float conc = max(susK.a, 1e-6);
  float density = susP.x / conc;
  float gran = susP.y / conc;
  float speed = length(flow.xy);
  // Settling: heavy pigment drops out of slow, shallow water, preferentially
  // into the valleys of the sheet. The valley bias is squared so granulating
  // pigment reads as particulate speckle, not as a gentle tint gradient.
  float calm = 1.0 / (1.0 + speed * 6.0);
  float shallow = smoothstep(0.35, 0.02, flow.b);
  float valley = max(1.0 + uGranBias * gran * (0.5 - paperH) * 2.0, 0.0);
  valley *= valley;
  float settleRate = uSettle * (0.25 + density) * calm * valley * (0.35 + 0.65 * shallow);
  settleRate *= 1.0 - 0.85 * clamp(salt * 4.0, 0.0, 1.0); // salt keeps pigment moving
  settleRate /= 1.0 + scrub * 6.0;                         // a scrubbed patch will not re-settle

  // The drying front. Where the wet region ends — a wash's perimeter, a tide
  // line mid-dry, the lobed rim of a backrun — suspended pigment strands
  // hard. This one term is the dried rim, the bead line, and the cauliflower
  // edge; without it every wash dries to an airbrush gradient.
  settleRate *= 1.0 + uEdgeSettle * edge;

  // The cap keeps deposition gradual: pigment has to survive in suspension
  // long enough to ride the drying currents, or no edge ever darkens.
  float down = susK.a * clamp(settleRate, 0.0, 0.2);

  // Lifting: moving water scrubs unstained deposit back into suspension —
  // and granulating deposit lets go of the peaks far more easily than the
  // pits, which is the second half of how granulation texture forms. A rim
  // that has settled at a drying edge stays put.
  float stained = depS.a / max(depK.a, 1e-6);
  float wetHere = step(0.002, flow.b);
  float peakBias = 1.0 + gran * (paperH - 0.5) * 1.6;
  float liftRate = uLift * wetHere * (0.3 + speed * 4.0) * (1.0 - clamp(stained, 0.0, 1.0)) * max(peakBias, 0.0);
  liftRate *= 1.0 - 0.75 * edge;
  liftRate += uSaltLift * salt * wetHere;
  // The lift tool scrubs through the staining resistance partially: even a
  // phthalo gives a little to a wet scrub, an earth gives nearly everything.
  liftRate += uScrubLift * scrub * wetHere * (1.0 - 0.7 * clamp(stained, 0.0, 1.0));
  float up = depK.a * clamp(liftRate, 0.0, 0.5);
  return Exchange(down, up);
}
`;

export const TRANSFER_SUS_FRAG = `${HEADER}
${TRANSFER_COMMON}
layout(location = 0) out vec4 outK;
layout(location = 1) out vec4 outS;
layout(location = 2) out vec4 outP;
// Source: src/engine/shaders.ts:268-298 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec4 susK = texture(uSusK, vUV);
  vec4 susS = texture(uSusS, vUV);
  vec4 susP = texture(uSusP, vUV);
  vec4 depK = texture(uDepK, vUV);
  vec4 depS = texture(uDepS, vUV);
  vec2 paper = texture(uPaper, vUV).rg;
  float salt = texture(uCap, vUV).g;
  float scrub = texture(uCap, vUV).b;
  float edge = clamp((flow.a - texture(uBlurW, vUV).r) * 3.0, 0.0, 1.0);
  Exchange ex = exchange(flow, susK, susP, depK, depS, paper.r, salt, scrub, edge);

  float conc = max(susK.a, 1e-6);
  float downFrac = ex.down / conc;
  float depth = max(depK.a, 1e-6);
  float upFrac = ex.up / depth;
  // Deposit carries its K and S proportionally; lifted deposit returns with
  // the deposit's spectral mix and (approximately) mid handling properties.
  outK = susK * (1.0 - downFrac) + depK * upFrac;
  outS = susS * (1.0 - downFrac) + depS * upFrac;
  vec3 liftedProps = vec3(0.5, 0.3, depS.a / depth) * ex.up;
  outP = vec4(susP.xyz * (1.0 - downFrac) + liftedProps, 0.0);

  // The lift tool's brush is thirsty: suspension under the scrub leaves the
  // sheet entirely, the way a damp brush or tissue carries pigment away.
  float drink = clamp(scrub * 0.35, 0.0, 0.2);
  outK *= 1.0 - drink;
  outS *= 1.0 - drink;
  outP *= 1.0 - drink;
}
`;

export const TRANSFER_DEP_FRAG = `${HEADER}
${TRANSFER_COMMON}
layout(location = 0) out vec4 outDepK;
layout(location = 1) out vec4 outDepS;
// Source: src/engine/shaders.ts:305-328 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec4 susK = texture(uSusK, vUV);
  vec4 susS = texture(uSusS, vUV);
  vec4 susP = texture(uSusP, vUV);
  vec4 depK = texture(uDepK, vUV);
  vec4 depS = texture(uDepS, vUV);
  vec2 paper = texture(uPaper, vUV).rg;
  float salt = texture(uCap, vUV).g;
  float scrub = texture(uCap, vUV).b;
  float edge = clamp((flow.a - texture(uBlurW, vUV).r) * 3.0, 0.0, 1.0);
  Exchange ex = exchange(flow, susK, susP, depK, depS, paper.r, salt, scrub, edge);

  float conc = max(susK.a, 1e-6);
  float downFrac = ex.down / conc;
  float depth = max(depK.a, 1e-6);
  float upFrac = ex.up / depth;
  float stainingMean = susP.z / conc;
  outDepK = depK * (1.0 - upFrac) + susK * downFrac;
  outDepS = vec4(
    depS.rgb * (1.0 - upFrac) + susS.rgb * downFrac,
    depS.a * (1.0 - upFrac) + ex.down * clamp(stainingMean, 0.0, 1.0)
  );
}
`;

/** Capillary layer: the sheet's interior water. It diffuses sideways through
 * the fibers and seeps back out into barely-damp cells, which is where
 * backruns and soft bloom fringes come from. Salt decays slowly as it
 * dissolves. */
export const CAPILLARY_FRAG = `${HEADER}
uniform sampler2D uCap;
uniform sampler2D uFlow;
uniform float uCapDiff;
uniform float uCapDry;
uniform float uEvapMul;
uniform float uAbsorb;
out vec4 outColor;
// Source: src/engine/shaders.ts:343-362 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 cap = texture(uCap, vUV);
  vec4 flow = texture(uFlow, vUV);
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  float avg = 0.25 * (
    texture(uCap, vUV - tx).r + texture(uCap, vUV + tx).r +
    texture(uCap, vUV - ty).r + texture(uCap, vUV + ty).r
  );
  float s = cap.r + uCapDiff * (avg - cap.r);
  // What the surface lost to absorption (same formula as the water pass)
  // arrives here, so the sheet's interior wetness is a real quantity.
  s += min(flow.b, uAbsorb * (1.0 - s));
  s = max(0.0, s - uCapDry * uEvapMul);
  float salt = cap.g * (1.0 - 0.0015 * uEvapMul);
  // The scrub field left by the lift tool fades as the disturbed sizing
  // resettles.
  float scrub = cap.b * 0.94;
  outColor = vec4(s, salt, scrub, 0.0);
}
`;

/** Folds everything that has finished drying into the single dried substrate.
 * Runs every step; only cells whose wetness has collapsed change. */
export const DRY_FOLD_FRAG = `${HEADER}
${KM_GLSL}
uniform sampler2D uFlow;
uniform sampler2D uSusK;
uniform sampler2D uSusS;
uniform sampler2D uSusP;
uniform sampler2D uDepK;
uniform sampler2D uDepS;
uniform sampler2D uDried;
uniform sampler2D uPaper;
out vec4 outColor;
// Source: src/engine/shaders.ts:378-408 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 dried = texture(uDried, vUV);
  vec4 flow = texture(uFlow, vUV);
  vec4 depK = texture(uDepK, vUV);
  vec4 susK = texture(uSusK, vUV);
  float pigment = depK.a + susK.a;
  bool justDried = flow.a < 0.02 && pigment > 1e-4;
  if (!justDried) {
    outColor = dried;
    return;
  }
  vec3 K = depK.rgb + susK.rgb;
  vec3 S = texture(uDepS, vUV).rgb + texture(uSusS, vUV).rgb;

  // Granulation is baked at drying time, at the finest scale the paper
  // carries: a granulating pigment's layer is genuinely thicker in the
  // tooth's pits and thinner on its peaks. Because it lands in the dried
  // reflectance itself, the sediment speckle ghosts through every later
  // glaze — a real sheet never forgets its texture.
  float gran = clamp(texture(uSusP, vUV).y / max(susK.a, 1e-5), 0.0, 1.0);
  gran *= clamp(pigment * 2.2, 0.15, 1.0); // speckle pools with the paint
  vec2 paper = texture(uPaper, vUV).rg;
  float pit = smoothstep(0.62, 0.32, paper.r * 0.55 + paper.g * 0.45);
  float thickness = 1.0 + gran * (0.9 * pit - 0.35);

  vec3 R, T;
  kmLayer(K, S, thickness, R, T);
  vec3 folded = kmComposite(R, T, dried.rgb);
  float depth = dried.a + thickness * (K.r + K.g + K.b) / 3.0;
  outColor = vec4(folded, depth);
}
`;

/** Companion to DRY_FOLD: zero the wet-layer pigment in cells that just
 * folded. Same condition, evaluated from the same inputs. */
export const DRY_CLEAR_SUS_FRAG = `${HEADER}
uniform sampler2D uFlow;
uniform sampler2D uSusK;
uniform sampler2D uSusS;
uniform sampler2D uSusP;
uniform sampler2D uDepK;
layout(location = 0) out vec4 outK;
layout(location = 1) out vec4 outS;
layout(location = 2) out vec4 outP;
// Source: src/engine/shaders.ts:422-431 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec4 susK = texture(uSusK, vUV);
  vec4 depK = texture(uDepK, vUV);
  float pigment = depK.a + susK.a;
  float keep = (flow.a < 0.02 && pigment > 1e-4) ? 0.0 : 1.0;
  outK = texture(uSusK, vUV) * keep;
  outS = texture(uSusS, vUV) * keep;
  outP = texture(uSusP, vUV) * keep;
}
`;

export const DRY_CLEAR_DEP_FRAG = `${HEADER}
uniform sampler2D uFlow;
uniform sampler2D uSusK;
uniform sampler2D uDepK;
uniform sampler2D uDepS;
layout(location = 0) out vec4 outK;
layout(location = 1) out vec4 outS;
// Source: src/engine/shaders.ts:441-449 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec4 susK = texture(uSusK, vUV);
  vec4 depK = texture(uDepK, vUV);
  float pigment = depK.a + susK.a;
  float keep = (flow.a < 0.02 && pigment > 1e-4) ? 0.0 : 1.0;
  outK = depK * keep;
  outS = texture(uDepS, vUV) * keep;
}
`;

/** Brush splat: instanced soft stamps, blended additively into the water and
 * suspension textures. The vertex shader positions a quad per stamp; the
 * fragment gates contact by the paper's tooth when the brush is dry. */
export const SPLAT_VERT = `#version 300 es
layout(location = 0) in vec2 aCorner;           // unit quad, -1..1
layout(location = 1) in vec4 aStamp;            // x, y (sim px), radius, water
layout(location = 2) in vec4 aPaint;            // pigment amount, dryness, seed, –
uniform vec2 uSimSize;
out vec2 vLocal;
out vec2 vStampUV;
flat out vec4 vStamp;
flat out vec4 vPaint;
// Source: src/engine/shaders.ts:464-471 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vLocal = aCorner;
  vStamp = aStamp;
  vPaint = aPaint;
  vec2 px = aStamp.xy + aCorner * aStamp.z;
  vStampUV = px / uSimSize;
  gl_Position = vec4(vStampUV * 2.0 - 1.0, 0.0, 1.0);
}
`;

export const SPLAT_FRAG = `#version 300 es
precision highp float;
in vec2 vLocal;
in vec2 vStampUV;
flat in vec4 vStamp;
flat in vec4 vPaint;
uniform sampler2D uPaper;
uniform vec3 uPigK;
uniform vec3 uPigS;
uniform vec3 uPigProps;   // density, granulation, staining
layout(location = 0) out vec4 outFlow;   // adds (0, 0, water, wetness)
layout(location = 1) out vec4 outK;
layout(location = 2) out vec4 outS;
layout(location = 3) out vec4 outP;
// Source: src/engine/shaders.ts:488-515 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  float r = length(vLocal);
  if (r > 1.0) discard;
  // Soft-bodied stamp with a definite rim: a loaded brush wets fully inside
  // its footprint and falls off fast at the edge.
  float body = smoothstep(1.0, 0.62, r);
  float water = vStamp.w * body;
  float pigment = vPaint.x * body;

  // Dry contact: only the tops of the tooth receive paint, broken further by
  // the fine grain — the sparkle of a starved brush on rough paper.
  float dryness = vPaint.y;
  if (dryness > 0.0) {
    // Hard-edged flecks: the paper is dry, so contact is a near-binary
    // threshold on the tooth — crisp, not sponge-soft.
    vec2 paper = texture(uPaper, vStampUV).rg;
    float level = 0.56 - 0.14 * (1.0 - dryness);
    float tooth = smoothstep(level, level + 0.05, paper.r + paper.g * 0.3);
    float contact = mix(1.0, tooth, dryness);
    water *= contact;
    pigment *= contact * (0.7 + 0.6 * paper.g);
  }

  outFlow = vec4(0.0, 0.0, water, min(1.0, water * 60.0));
  outK = vec4(uPigK * pigment, pigment);
  outS = vec4(uPigS * pigment, 0.0);
  outP = vec4(uPigProps * pigment, 0.0);
}
`;

/** Salt: a handful of grains as tiny splats into the capillary texture's
 * salt channel, plus a bite taken out of the standing water. */
export const SALT_FRAG = `#version 300 es
precision highp float;
in vec2 vLocal;
in vec2 vStampUV;
flat in vec4 vStamp;
flat in vec4 vPaint;
layout(location = 0) out vec4 outCap;    // adds (0, salt, 0, 0)

// Source: src/engine/shaders.ts:528-530 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
float saltHash(float x, float seed) {
  return fract(sin(x * 127.1 + seed * 311.7) * 43758.5453);
}

// Source: src/engine/shaders.ts:532-551 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  // An irregular amoeba, not a glyph: the grain's reach is value noise over
  // angle (different per grain), and the whole footprint is stretched along
  // a random wicking direction — the way brine actually creeps.
  float seed = vPaint.z;
  float wickAngle = seed * 2.39996;
  vec2 wick = vec2(cos(wickAngle), sin(wickAngle));
  vec2 lp = vLocal - wick * dot(vLocal, wick) * (0.3 + 0.35 * saltHash(7.0, seed));
  float r = length(lp);
  if (r > 1.4) discard;
  float bins = 5.0 + floor(mod(seed, 3.0));
  float b = (atan(lp.y, lp.x) / 6.2831853 + 0.5) * bins;
  float i0 = mod(floor(b), bins);
  float i1 = mod(i0 + 1.0, bins);
  float f = fract(b);
  f = f * f * (3.0 - 2.0 * f);
  float reach = mix(saltHash(i0, seed), saltHash(i1, seed), f) * 0.45 + 0.35;
  float grain = smoothstep(1.0, 0.15, r / max(reach, 0.2));
  outCap = vec4(0.0, vPaint.x * grain * 5.0, 0.0, 0.0);
}
`;


/** The lift tool's scrub marks: soft discs into the capillary texture's
 * scrub channel. The physics does the actual lifting. */
export const LIFT_FRAG = `#version 300 es
precision highp float;
in vec2 vLocal;
in vec2 vStampUV;
flat in vec4 vStamp;
flat in vec4 vPaint;
layout(location = 0) out vec4 outCap;    // adds (0, 0, scrub, 0)
// Source: src/engine/shaders.ts:564-569 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  float r = length(vLocal);
  if (r > 1.0) discard;
  float body = smoothstep(1.0, 0.45, r);
  outCap = vec4(0.0, 0.0, vPaint.x * body, 0.0);
}
`;

/** The render: Kubelka-Munk composite of the live wet layer over the dried
 * substrate over the sheet, lit so the paper's relief and the standing water
 * read as material rather than as a flat picture. */
export const RENDER_FRAG = `${HEADER}
${KM_GLSL}
uniform sampler2D uFlow;
uniform sampler2D uSusK;
uniform sampler2D uSusS;
uniform sampler2D uDepK;
uniform sampler2D uDepS;
uniform sampler2D uDried;
uniform sampler2D uPaper;
uniform sampler2D uSusP;
uniform float uBacklight;    // 0 = reflected light, 1 = lightbox
uniform float uZoomGrain;    // render-res grain strength
out vec4 outColor;

// Source: src/engine/shaders.ts:589-593 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
float hash12(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}

// Source: src/engine/shaders.ts:595-664 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(uFlow, vUV);
  vec4 susK = texture(uSusK, vUV);
  vec4 susS = texture(uSusS, vUV);
  vec4 depK = texture(uDepK, vUV);
  vec4 depS = texture(uDepS, vUV);
  vec4 dried = texture(uDried, vUV);
  vec2 paper = texture(uPaper, vUV).rg;

  // Paper ground: the sheet's own color shaded by its relief. Light falls
  // from the upper left; the tooth catches it on one flank and shadows the
  // other, and a fine sub-texel grain keeps the surface alive at full zoom.
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  float hL = texture(uPaper, vUV - tx).r;
  float hR = texture(uPaper, vUV + tx).r;
  float hD = texture(uPaper, vUV - ty).r;
  float hU = texture(uPaper, vUV + ty).r;
  vec3 normal = normalize(vec3((hL - hR) * 2.2, (hD - hU) * 2.2, 1.0));
  vec3 lightDir = normalize(vec3(-0.45, 0.55, 0.9));
  float relief = 0.85 + 0.15 * dot(normal, lightDir);
  float microGrain = 0.97 + 0.03 * paper.g + uZoomGrain * (hash12(gl_FragCoord.xy) - 0.5);
  // The room's light is not perfectly even across a sheet this size: a
  // gentle falloff away from the light corner keeps the field from reading
  // as a synthetic flat.
  float illum = 1.035 - 0.085 * length(vUV - vec2(0.3, 0.78));

  // Wet paper is darker paper: water fills the tooth and reduces scattering.
  float wet = clamp(flow.a, 0.0, 1.0);
  float damp = clamp(flow.b * 8.0, 0.0, 1.0);
  float wetDarken = 1.0 - 0.075 * wet - 0.05 * damp;

  // The dried texture is initialized to the bare sheet's reflectance and
  // accumulates every dried layer of paint, so it IS the ground — the render
  // lights it with the paper's relief, and where paint has dried, the tooth
  // shows through it: every real wash reads the sheet's texture, granulating
  // or not, because deposit is physically thicker in the pits.
  // Weighted toward the structured height octaves rather than the 1px fine
  // grain: pigment texture clumps at fiber scale, it is not white noise.
  float driedPaint = clamp(dried.a * 1.6, 0.0, 1.0);
  float tooth = 1.0 + ((paper.g - 0.5) * 0.07 + (0.5 - paper.r) * 0.22) * driedPaint;
  vec3 base = dried.rgb * tooth * relief * microGrain * illum * wetDarken;

  // Live wet layer: suspension plus fresh deposit as one KM film. The
  // granulation speckle scales pigment thickness by the fine grain, which is
  // how settled ultramarine actually reads — thicker in every pit.
  float granMean = texture(uSusP, vUV).y / max(susK.a, 1e-5);
  float depGran = clamp(granMean, 0.0, 1.0) * 0.95 + 0.06;
  float granMod = 1.0 + depGran * (paper.g - 0.5) * 0.8 * min(depK.a * 3.0, 1.0);
  vec3 K = (susK.rgb + depK.rgb) * granMod;
  vec3 S = (susS.rgb + depS.rgb) * granMod;
  vec3 R, T;
  kmLayer(K, S, 1.0, R, T);
  vec3 color = kmComposite(R, T, base);

  // A live sheen on standing water: a whisper of the sky in the film.
  float sheen = smoothstep(0.05, 0.5, flow.b) * 0.05 * pow(max(dot(normal, normalize(lightDir + vec3(0.0, 0.0, 1.0))), 0.0), 3.0);
  color += vec3(sheen);

  if (uBacklight > 0.0) {
    // Lightbox: what survives the trip through every layer and the sheet.
    vec3 through = T * exp(-vec3(dried.a) * vec3(1.15, 1.0, 0.9)) ;
    vec3 lit = vec3(1.0, 0.985, 0.94) * through * (0.55 + 0.45 * paper.r);
    color = mix(color, lit, uBacklight);
  }

  // Linear -> sRGB.
  vec3 srgb = mix(color * 12.92, 1.055 * pow(color, vec3(1.0 / 2.4)) - 0.055, step(0.0031308, color));
  outColor = vec4(srgb, 1.0);
}
`;

/** X-ray: the state textures themselves, one field at a time, on the bare
 * sheet. Nothing is stylized: each pixel is one number from one texture,
 * tone-mapped through 1 - exp(-v / scale) so a saturating ramp never clips
 * and small values stay legible, then painted from paper cream toward the
 * site's deep teal. The scale per field is fixed, never fitted to the frame,
 * so the same wash reads the same shade from one moment to the next.
 *   1 water   flow.b   standing water depth
 *   2 wet     flow.a   surface wetness (0..1)
 *   3 sus     susK.a   pigment concentration in suspension
 *   4 dep     depK.a   deposit thickness (settled, not yet dried)
 *   5 paper   cap.r    water inside the paper; salt grains (cap.g) in vermillion
 *   6 dried   dried.a  optical depth of everything cured into the sheet */
export const XRAY_FRAG = `${HEADER}
uniform sampler2D uFlow;
uniform sampler2D uSusK;
uniform sampler2D uDepK;
uniform sampler2D uCap;
uniform sampler2D uDried;
uniform int uField;
uniform float uScale;
out vec4 outColor;

const vec3 PAPER = vec3(0.957, 0.898, 0.776); // #faf3e4 in linear light
const vec3 MID   = vec3(0.071, 0.254, 0.291); // #4b8a93 — keeps the ramp teal through its middle
const vec3 INK   = vec3(0.0, 0.050, 0.065);   // #003f48 in linear light
const vec3 SALT  = vec3(0.791, 0.045, 0.0);   // #e73d00 in linear light

// Source: src/engine/shaders.ts:694-696 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
vec3 ramp(float t) {
  return t < 0.5 ? mix(PAPER, MID, t * 2.0) : mix(MID, INK, (t - 0.5) * 2.0);
}

// Source: src/engine/shaders.ts:698-714 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  float v = 0.0;
  if (uField == 1) v = texture(uFlow, vUV).b;
  else if (uField == 2) v = texture(uFlow, vUV).a;
  else if (uField == 3) v = texture(uSusK, vUV).a;
  else if (uField == 4) v = texture(uDepK, vUV).a;
  else if (uField == 5) v = texture(uCap, vUV).r;
  else if (uField == 6) v = texture(uDried, vUV).a;
  float t = 1.0 - exp(-max(v, 0.0) / uScale);
  vec3 color = ramp(t);
  if (uField == 5) {
    float salt = clamp(texture(uCap, vUV).g * 0.6, 0.0, 1.0);
    color = mix(color, SALT, salt);
  }
  vec3 srgb = mix(color * 12.92, 1.055 * pow(color, vec3(1.0 / 2.4)) - 0.055, step(0.0031308, color));
  outColor = vec4(srgb, 1.0);
}
`;
