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
layout(location=0) in vec2 vLocal;
layout(location=1) in vec2 vStampUV;
flat layout(location=2) in vec4 vStamp;
flat layout(location=3) in vec4 vPaint;



   // density, granulation, staining
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
    vec2 paper = texture(sampler2D(t0, linearSampler), vStampUV).rg;
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
