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
layout(location=0) in vec2 vLocal;
layout(location=1) in vec2 vStampUV;
flat layout(location=2) in vec4 vStamp;
flat layout(location=3) in vec4 vPaint;
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
