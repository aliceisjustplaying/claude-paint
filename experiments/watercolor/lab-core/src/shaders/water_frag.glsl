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
layout(location=0) in vec2 vUV;






         // base evaporation per step
     // extra evaporation at wash boundaries
      // hairdryer / fast-forward multiplier
       // surface -> paper interior
     // flux -> px/step velocity
layout(location=0) out vec4 outColor;
// Source: src/engine/shaders.ts:130-161 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(sampler2D(t0, linearSampler), vUV);
  vec4 f = texture(sampler2D(t1, linearSampler), vUV);
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  float inL = texture(sampler2D(t1, linearSampler), vUV - tx).y;
  float inR = texture(sampler2D(t1, linearSampler), vUV + tx).x;
  float inD = texture(sampler2D(t1, linearSampler), vUV - ty).w;
  float inU = texture(sampler2D(t1, linearSampler), vUV + ty).z;

  float p = flow.b + (inL + inR + inD + inU) - (f.x + f.y + f.z + f.w);

  // Evaporation, faster where the film is thin and at the boundary of the
  // wet region — the drying front eats inward, and the pigment it strands
  // there is the classic hard watercolor edge.
  float edge = clamp(flow.a - texture(sampler2D(t2, linearSampler), vUV).r, 0.0, 1.0);
  p -= uEvap * uEvapMul + uEvapEdge * uEvapMul * edge;

  // The sheet drinks: absorption into the capillary layer until saturated.
  float sat = texture(sampler2D(t3, linearSampler), vUV).r;
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
