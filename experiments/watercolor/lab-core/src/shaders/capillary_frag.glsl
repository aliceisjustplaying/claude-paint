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
layout(location=0) in vec2 vUV;








layout(location=0) out vec4 outColor;
// Source: src/engine/shaders.ts:343-362 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 cap = texture(sampler2D(t0, linearSampler), vUV);
  vec4 flow = texture(sampler2D(t1, linearSampler), vUV);
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  float avg = 0.25 * (
    texture(sampler2D(t0, linearSampler), vUV - tx).r + texture(sampler2D(t0, linearSampler), vUV + tx).r +
    texture(sampler2D(t0, linearSampler), vUV - ty).r + texture(sampler2D(t0, linearSampler), vUV + ty).r
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
