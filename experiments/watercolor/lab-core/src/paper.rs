// Copyright (c) 2026 Ryan McCarty. MIT License.
// Mechanical port of uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See experiments/watercolor/THIRD_PARTY_NOTICES.md for the full license.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaperKind {
    HotPress,
    ColdPress,
    Rough,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct PaperSpec {
    pub kind: PaperKind,
    pub name: &'static str,
    pub tooth: f64,
    pub fiber: f64,
}

pub const PAPERS: [PaperSpec; 3] = [
    PaperSpec {
        kind: PaperKind::HotPress,
        name: "Hot press",
        tooth: 0.3,
        fiber: 0.5,
    },
    PaperSpec {
        kind: PaperKind::ColdPress,
        name: "Cold press",
        tooth: 0.72,
        fiber: 1.0,
    },
    PaperSpec {
        kind: PaperKind::Rough,
        name: "Rough",
        tooth: 1.0,
        fiber: 0.85,
    },
];

/// Source: src/engine/paper.ts:28–32 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn paper_spec(kind: PaperKind) -> PaperSpec {
    match kind {
        PaperKind::HotPress => PAPERS[0],
        PaperKind::ColdPress => PAPERS[1],
        PaperKind::Rough => PAPERS[2],
    }
}

/// Source: src/engine/paper.ts:35–40 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn hash2(ix: f64, iy: f64, seed: f64) -> f64 {
    let h = (ix * 374761393.0 + iy * 668265263.0 + seed * 2246822519.0)
        .trunc()
        .rem_euclid(4294967296.0) as u32;
    let h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    let h = h ^ (h >> 16);
    h as f64 / 4294967296.0
}

/// Source: src/engine/paper.ts:42–44 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

/// Source: src/engine/paper.ts:47–59 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn value_noise(x: f64, y: f64, scale: f64, seed: f64) -> f64 {
    let gx = x / scale;
    let gy = y / scale;
    let ix = gx.floor();
    let iy = gy.floor();
    let fx = smooth(gx - ix);
    let fy = smooth(gy - iy);
    let v00 = hash2(ix, iy, seed);
    let v10 = hash2(ix + 1.0, iy, seed);
    let v01 = hash2(ix, iy + 1.0, seed);
    let v11 = hash2(ix + 1.0, iy + 1.0, seed);
    (v00 * (1.0 - fx) + v10 * fx) * (1.0 - fy) + (v01 * (1.0 - fx) + v11 * fx) * fy
}

/// Source: src/engine/paper.ts:76–112 and src/engine/simulation.ts:243–250
/// at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn generate_paper(width: usize, height: usize, kind: PaperKind, seed: u32) -> Vec<[f32; 4]> {
    generate_paper_with_spec(width, height, &paper_spec(kind), seed)
}

/// Source: src/engine/paper.ts:76–112 and src/engine/simulation.ts:243–250
/// at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn generate_paper_with_spec(
    width: usize,
    height: usize,
    spec: &PaperSpec,
    seed: u32,
) -> Vec<[f32; 4]> {
    let mut data = vec![[0.0; 4]; width * height];
    let octaves = [(34.0, 0.30), (16.0, 0.42), (7.0, 0.55), (3.2, 0.35)];
    let mut amp_sum = 0.0;
    for (_, amp) in octaves {
        amp_sum += amp;
    }
    let seed = seed as f64;
    for y in 0..height {
        for x in 0..width {
            let xf = x as f64;
            let yf = y as f64;
            let mut h = 0.0;
            for (o, (size, amp)) in octaves.iter().enumerate() {
                h += (value_noise(xf, yf, *size, seed + o as f64 * 101.0) - 0.5) * amp;
            }
            h /= amp_sum;
            let wave = value_noise(xf, yf, 90.0, seed + 900.0) * 8.0;
            let f1 = value_noise(xf * 0.22, yf * 1.6 + wave, 6.0, seed + 501.0) - 0.5;
            let f2 = value_noise(xf * 1.6 + wave, yf * 0.22, 6.0, seed + 502.0) - 0.5;
            h += (f1 * 0.75 + f2 * 0.45) * 0.22 * spec.fiber;
            data[y * width + x][0] = (0.5 + h * spec.tooth).clamp(0.0, 1.0) as f32;
            data[y * width + x][1] = value_noise(xf, yf, 1.7, seed + 77.0) as f32;
        }
    }
    data
}
