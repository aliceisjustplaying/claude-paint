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






      // gain on head difference
         // flux persistence (inertia)
        // how much paper relief tilts the water
      // water needed before invading dry paper
     // osmotic head drop at a salt grain

// Source: src/engine/shaders.ts:66-71 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
float head(vec2 uv) {
  vec4 f = texture(sampler2D(t0, linearSampler), uv);
  float paper = texture(sampler2D(t2, linearSampler), uv).r;
  float salt = texture(sampler2D(t3, linearSampler), uv).g;
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
  float wetThere = texture(sampler2D(t0, linearSampler), uv).a;
  float fiberMix = clamp(1.0 - pHere * 5.0, 0.25, 1.0);
  float fiber = mix(1.0, 0.55 + 1.1 * texture(sampler2D(t2, linearSampler), uv).r, fiberMix);
  float invade = smoothstep(uTension * fiber, uTension * fiber * 2.5, pHere);
  return mix(invade, 1.0, step(0.05, wetThere));
}

layout(location=0) out vec4 outColor;
// Source: src/engine/shaders.ts:88-114 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flux = texture(sampler2D(t1, linearSampler), vUV);
  vec4 flow = texture(sampler2D(t0, linearSampler), vUV);
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
