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
layout(location=0) in vec2 vUV;



  // Kubelka-Munk: reflectance and transmittance of a pigment layer.
  // Mirrors layerRT() in src/paint/km.ts — keep the two in lockstep.
  void kmLayer(vec3 K, vec3 S, float x, out vec3 R, out vec3 T) {
    K = max(K, vec3(0.0));
    S = max(S, vec3(0.0));
    vec3 a = 1.0 + K / max(S, vec3(1e-6));
    vec3 b = sqrt(max(a * a - 1.0, 1e-12));
    vec3 y = min(b * S * x, vec3(40.0));
    vec3 ey = exp(y);
    vec3 einy = 1.0 / ey;
    vec3 sinhY = 0.5 * (ey - einy);
    vec3 coshY = 0.5 * (ey + einy);
    vec3 denom = a * sinhY + b * coshY;
    R = sinhY / denom;
    T = b / denom;
    // Pure-absorber limit for near-zero scattering (Beer-Lambert).
    vec3 absorber = step(S, vec3(1e-6));
    R = mix(R, vec3(0.0), absorber);
    T = mix(T, exp(-K * x), absorber);
    // No layer at all.
    float none = step(x, 0.0);
    R = mix(R, vec3(0.0), none);
    T = mix(T, vec3(1.0), none);
  }

  // Layer over substrate: R1 + T1^2 R2 / (1 - R1 R2).
  // Mirrors composite() in src/paint/km.ts.
  vec3 kmComposite(vec3 R1, vec3 T1, vec3 R2) {
    return R1 + T1 * T1 * R2 / (1.0 - min(R1 * R2, vec3(0.9999)));
  }









layout(location=0) out vec4 outColor;
// Source: src/engine/shaders.ts:378-408 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 dried = texture(sampler2D(t6, linearSampler), vUV);
  vec4 flow = texture(sampler2D(t0, linearSampler), vUV);
  vec4 depK = texture(sampler2D(t4, linearSampler), vUV);
  vec4 susK = texture(sampler2D(t1, linearSampler), vUV);
  float pigment = depK.a + susK.a;
  bool justDried = flow.a < 0.02 && pigment > 1e-4;
  if (!justDried) {
    outColor = dried;
    return;
  }
  vec3 K = depK.rgb + susK.rgb;
  vec3 S = texture(sampler2D(t5, linearSampler), vUV).rgb + texture(sampler2D(t2, linearSampler), vUV).rgb;

  // Granulation is baked at drying time, at the finest scale the paper
  // carries: a granulating pigment's layer is genuinely thicker in the
  // tooth's pits and thinner on its peaks. Because it lands in the dried
  // reflectance itself, the sediment speckle ghosts through every later
  // glaze — a real sheet never forgets its texture.
  float gran = clamp(texture(sampler2D(t3, linearSampler), vUV).y / max(susK.a, 1e-5), 0.0, 1.0);
  gran *= clamp(pigment * 2.2, 0.15, 1.0); // speckle pools with the paint
  vec2 paper = texture(sampler2D(t7, linearSampler), vUV).rg;
  float pit = smoothstep(0.62, 0.32, paper.r * 0.55 + paper.g * 0.45);
  float thickness = 1.0 + gran * (0.9 * pit - 0.35);

  vec3 R, T;
  kmLayer(K, S, thickness, R, T);
  vec3 folded = kmComposite(R, T, dried.rgb);
  float depth = dried.a + thickness * (K.r + K.g + K.b) / 3.0;
  outColor = vec4(folded, depth);
}
