// Copyright (c) 2026 Ryan McCarty. MIT License.
// Mechanical port of uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See experiments/watercolor/THIRD_PARTY_NOTICES.md for the full license.

pub type Vec3 = [f32; 3];

pub const PAPER_REFLECTANCE: Vec3 = [0.9559733, 0.8962694, 0.7758222];

/// Source: src/paint/km.ts:149–170 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn layer_rt(k: Vec3, s: Vec3, x: f32) -> (Vec3, Vec3) {
    let mut r = [0.0; 3];
    let mut t = [0.0; 3];
    for c in 0..3 {
        let k = k[c].max(0.0);
        let s = s[c].max(0.0);
        let a = 1.0 + k / s.max(1e-6);
        let b = (a * a - 1.0).max(1e-12).sqrt();
        let y = (b * s * x).min(40.0);
        let ey = y.exp();
        let einy = 1.0 / ey;
        let sinh_y = 0.5 * (ey - einy);
        let cosh_y = 0.5 * (ey + einy);
        let denom = a * sinh_y + b * cosh_y;
        r[c] = sinh_y / denom;
        t[c] = b / denom;
        if s <= 1e-6 {
            r[c] = 0.0;
            t[c] = (-k * x).exp();
        }
        if x <= 0.0 {
            r[c] = 0.0;
            t[c] = 1.0;
        }
    }
    (r, t)
}

/// Source: src/paint/km.ts:174–176 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn composite(r1: Vec3, t1: Vec3, r2: Vec3) -> Vec3 {
    let mut out = [0.0; 3];
    for c in 0..3 {
        out[c] = r1[c] + t1[c] * t1[c] * r2[c] / (1.0 - (r1[c] * r2[c]).min(0.9999));
    }
    out
}

/// Source: src/paint/km.ts:71–83 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn masstone(k: Vec3, s: Vec3) -> Vec3 {
    let mut out = [0.0; 3];
    for c in 0..3 {
        if s[c] <= 1e-9 {
            continue;
        }
        let a = 1.0 + k[c] / s[c];
        let b = (a * a - 1.0).max(1e-12).sqrt();
        out[c] = 1.0 / (a + b);
    }
    out
}

/// Source: src/paint/km.ts:109–118 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn km_from_masstone(r_inf: [f64; 3], scattering: f64, strength: f64) -> (Vec3, Vec3) {
    let mut k = [0.0; 3];
    let mut s = [0.0; 3];
    for c in 0..3 {
        let r = r_inf[c].clamp(0.012, 0.985);
        let sc = scattering * (0.35 + 0.65 * r) * strength;
        k[c] = (sc * (1.0 - r) * (1.0 - r) / (2.0 * r)) as f32;
        s[c] = sc as f32;
    }
    (k, s)
}

/// Source: src/paint/km.ts:121–125 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn hex_to_linear(hex: &str) -> [f64; 3] {
    let n = u32::from_str_radix(hex.trim_start_matches('#'), 16).expect("sRGB hex");
    [((n >> 16) & 255), ((n >> 8) & 255), (n & 255)].map(|v| {
        let v = v as f64 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    })
}

/// Source: src/paint/km.ts:127–131 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn linear_to_srgb_byte(v: f32) -> u8 {
    let clamped = (v as f64).clamp(0.0, 1.0);
    let srgb = if clamped <= 0.0031308 {
        clamped * 12.92
    } else {
        1.055 * clamped.powf(1.0 / 2.4) - 0.055
    };
    (srgb * 255.0).round() as u8
}

/// Source: src/paint/km.ts:139 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn paper_reflectance() -> Vec3 {
    PAPER_REFLECTANCE
}
