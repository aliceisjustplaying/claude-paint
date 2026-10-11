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
layout(location=0) in vec2 vUV;









layout(location=0) out vec4 outColor;

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
  if (uField == 1) v = texture(sampler2D(t0, linearSampler), vUV).b;
  else if (uField == 2) v = texture(sampler2D(t0, linearSampler), vUV).a;
  else if (uField == 3) v = texture(sampler2D(t1, linearSampler), vUV).a;
  else if (uField == 4) v = texture(sampler2D(t2, linearSampler), vUV).a;
  else if (uField == 5) v = texture(sampler2D(t3, linearSampler), vUV).r;
  else if (uField == 6) v = texture(sampler2D(t4, linearSampler), vUV).a;
  float t = 1.0 - exp(-max(v, 0.0) / uScale);
  vec3 color = ramp(t);
  if (uField == 5) {
    float salt = clamp(texture(sampler2D(t3, linearSampler), vUV).g * 0.6, 0.0, 1.0);
    color = mix(color, SALT, salt);
  }
  vec3 srgb = mix(color * 12.92, 1.055 * pow(color, vec3(1.0 / 2.4)) - 0.055, step(0.0031308, color));
  outColor = vec4(srgb, 1.0);
}
