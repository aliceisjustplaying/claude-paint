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









    // 0 = reflected light, 1 = lightbox
    // render-res grain strength
layout(location=0) out vec4 outColor;

// Source: src/engine/shaders.ts:589-593 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
float hash12(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}

// Source: src/engine/shaders.ts:595-664 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vec4 flow = texture(sampler2D(t0, linearSampler), vUV);
  vec4 susK = texture(sampler2D(t1, linearSampler), vUV);
  vec4 susS = texture(sampler2D(t2, linearSampler), vUV);
  vec4 depK = texture(sampler2D(t3, linearSampler), vUV);
  vec4 depS = texture(sampler2D(t4, linearSampler), vUV);
  vec4 dried = texture(sampler2D(t5, linearSampler), vUV);
  vec2 paper = texture(sampler2D(t6, linearSampler), vUV).rg;

  // Paper ground: the sheet's own color shaded by its relief. Light falls
  // from the upper left; the tooth catches it on one flank and shadows the
  // other, and a fine sub-texel grain keeps the surface alive at full zoom.
  vec2 tx = vec2(uTexel.x, 0.0);
  vec2 ty = vec2(0.0, uTexel.y);
  float hL = texture(sampler2D(t6, linearSampler), vUV - tx).r;
  float hR = texture(sampler2D(t6, linearSampler), vUV + tx).r;
  float hD = texture(sampler2D(t6, linearSampler), vUV - ty).r;
  float hU = texture(sampler2D(t6, linearSampler), vUV + ty).r;
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
  float granMean = texture(sampler2D(t7, linearSampler), vUV).y / max(susK.a, 1e-5);
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
