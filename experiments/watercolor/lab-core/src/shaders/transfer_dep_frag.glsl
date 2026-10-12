// Copyright (c) 2026 Ryan McCarty. MIT License.
// Adapted from uxpreview/watercolor-lab a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../../../THIRD_PARTY_NOTICES.md.
#version 450
layout(set=0,binding=0,std140) uniform Settings { vec4 q[32]; };
#define uTexel q[0].xy
#define uSimSize q[0].zw
#define uGravity q[1].x
#define uDamp q[1].y
#define uSlope q[1].z
#define uTension q[1].w
#define uSaltPull q[2].x
#define uEvap q[2].y
#define uEvapEdge q[2].z
#define uEvapMul q[2].w
#define uAbsorb q[3].x
#define uVelScale q[3].y
#define uAdvect q[3].z
#define uEdgeDrift q[3].w
#define uSettle q[4].x
#define uGranBias q[4].y
#define uLift q[4].z
#define uSaltLift q[4].w
#define uScrubLift q[5].x
#define uEdgeSettle q[5].y
#define uCapDiff q[5].z
#define uCapDry q[5].w
#define uPigK q[6].xyz
#define uPigS q[7].xyz
#define uPigProps q[8].xyz
#define uBacklight q[8].w
#define uZoomGrain q[9].x
#define uField int(q[9].y)
#define uScale q[9].z
layout(set=0,binding=1) uniform sampler linearSampler;
layout(set=0,binding=2) uniform texture2D t0;
layout(set=0,binding=3) uniform texture2D t1;
layout(set=0,binding=4) uniform texture2D t2;
layout(set=0,binding=5) uniform texture2D t3;
layout(set=0,binding=6) uniform texture2D t4;
layout(set=0,binding=7) uniform texture2D t5;
layout(set=0,binding=8) uniform texture2D t6;
layout(set=0,binding=9) uniform texture2D t7;
layout(set=0,binding=10) uniform texture2D t8;
layout(location=0) in vec2 vUV;












     // base settling rate
   // extra settling into paper valleys for granulators
       // base re-suspension rate
   // extra lift at a salt grain
  // extra lift under the lift tool's scrub
 // deposition boost at the boundary of the wet region

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

layout(location = 0) out vec4 outDepK;
layout(location = 1) out vec4 outDepS;
// Source: src/engine/shaders.ts:305-328 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(sampler2D(t0, linearSampler), vUV);
  vec4 susK = texture(sampler2D(t4, linearSampler), vUV);
  vec4 susS = texture(sampler2D(t5, linearSampler), vUV);
  vec4 susP = texture(sampler2D(t6, linearSampler), vUV);
  vec4 depK = texture(sampler2D(t7, linearSampler), vUV);
  vec4 depS = texture(sampler2D(t8, linearSampler), vUV);
  vec2 paper = texture(sampler2D(t1, linearSampler), vUV).rg;
  float salt = texture(sampler2D(t2, linearSampler), vUV).g;
  float scrub = texture(sampler2D(t2, linearSampler), vUV).b;
  float edge = clamp((flow.a - texture(sampler2D(t3, linearSampler), vUV).r) * 3.0, 0.0, 1.0);
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
