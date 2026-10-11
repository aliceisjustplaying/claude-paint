// Watercolor Lab brush contact and optics: Copyright (c) 2026 Ryan McCarty, MIT.
// Source pin: a19bfa2a026856a98ceb400a8eec41f091f04a11. See ../THIRD_PARTY_NOTICES.md.
use serde::{Deserialize, Serialize};
use watercolor_lab_core::brush::Stamp;
use watercolor_lab_core::{km, simulation::Simulation};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabSheet {
    pub simulation: Simulation,
    pub quarter_steps: u8,
    pub water_released_mm3: f64,
    pub pigment_released: f64,
}

impl LabSheet {
    pub fn new(simulation: Simulation) -> Self {
        Self {
            simulation,
            quarter_steps: 0,
            water_released_mm3: 0.0,
            pigment_released: 0.0,
        }
    }

    pub fn advance_tick(&mut self) {
        let steps = self.quarter_steps as usize + 45;
        self.simulation.step_many(steps / 4, 1.0);
        self.quarter_steps = (steps % 4) as u8;
    }

    pub fn validate(&self, width: usize, height: usize) -> Result<(), String> {
        let s = &self.simulation;
        if s.width != width
            || s.height != height
            || self.quarter_steps >= 4
            || s.raster_subpixel_bits > 16
            || s.params.substeps > 1000
            || !self.water_released_mm3.is_finite()
            || self.water_released_mm3 < 0.0
            || !self.pigment_released.is_finite()
            || self.pigment_released < 0.0
        {
            return Err("invalid Watercolor Lab adapter state".into());
        }
        let p = &s.params;
        if [
            p.gravity,
            p.damp,
            p.slope,
            p.tension,
            p.salt_pull,
            p.evap,
            p.evap_edge,
            p.absorb,
            p.cap_diff,
            p.cap_dry,
            p.vel_scale,
            p.advect,
            p.edge_drift,
            p.settle,
            p.gran_bias,
            p.lift,
            p.salt_lift,
            p.scrub_lift,
            p.edge_settle,
            p.zoom_grain,
        ]
        .iter()
        .any(|v| !v.is_finite())
        {
            return Err("invalid Watercolor Lab parameters".into());
        }
        for pair in [
            &s.flow, &s.flux, &s.sus_k, &s.sus_s, &s.sus_p, &s.dep_k, &s.dep_s, &s.cap, &s.dried,
        ] {
            for field in [&pair.read, &pair.write] {
                if field.len() != width * height || field.iter().flatten().any(|v| !v.is_finite()) {
                    return Err("invalid Watercolor Lab texture".into());
                }
            }
        }
        for field in [&s.paper, &s.blur_w] {
            if field.len() != width * height || field.iter().flatten().any(|v| !v.is_finite()) {
                return Err("invalid Watercolor Lab paper/blur texture".into());
            }
        }
        Ok(())
    }

    pub fn is_dry(&self) -> bool {
        self.simulation.flow.read.iter().all(|v| v[3] < 0.02)
            && self.simulation.sus_k.read.iter().all(|v| v[3] <= 1e-4)
    }

    pub fn surface_water(&self, cell_mm: f64) -> f64 {
        self.simulation
            .flow
            .read
            .iter()
            .map(|v| v[2] as f64)
            .sum::<f64>()
            * cell_mm.powi(2)
    }

    pub fn pore_water(&self, cell_mm: f64) -> f64 {
        self.simulation
            .cap
            .read
            .iter()
            .map(|v| v[0] as f64)
            .sum::<f64>()
            * cell_mm.powi(2)
    }

    /// Contact integrals from src/engine/shaders.ts:488–514 at the pinned commit.
    pub fn stamp_weights(&self, stamp: &Stamp) -> (f64, f64) {
        let s = &self.simulation;
        let mut water = 0.0;
        let mut pigment = 0.0;
        let quad = s.stamp_quad(
            stamp.x as f32,
            (s.height as f64 - stamp.y) as f32,
            stamp.radius as f32,
        );
        let (x0, x1, y0, y1) = s.stamp_bounds(quad);
        for yy in y0..y1 {
            for xx in x0..x1 {
                let lx = (xx as f32 + 0.5 - quad[0]) * (2.0 / (quad[2] - quad[0])) - 1.0;
                let ly = (yy as f32 + 0.5 - quad[1]) * (2.0 / (quad[3] - quad[1])) - 1.0;
                let r = (lx * lx + ly * ly).sqrt() as f64;
                if r > 1.0 {
                    continue;
                }
                let t = ((r - 1.0) / (0.62 - 1.0)).clamp(0.0, 1.0);
                let body = t * t * (3.0 - 2.0 * t);
                let p = s.paper[yy * s.width + xx];
                let level = 0.56 - 0.14 * (1.0 - stamp.dryness);
                let t = ((p[0] as f64 + p[1] as f64 * 0.3 - level) / 0.05).clamp(0.0, 1.0);
                let contact = 1.0 - stamp.dryness + stamp.dryness * t * t * (3.0 - 2.0 * t);
                water += body * contact;
                pigment += body
                    * contact
                    * if stamp.dryness > 0.0 {
                        0.7 + 0.6 * p[1] as f64
                    } else {
                        1.0
                    };
            }
        }
        (water, pigment)
    }

    pub fn project(&self, cell_mm: f64, light: Option<(f64, f64, f64)>) -> Vec<u8> {
        let s = &self.simulation;
        let mut rgb = Vec::with_capacity(s.width * s.height * 3);
        for y in (0..s.height).rev() {
            for x in 0..s.width {
                let i = y * s.width + x;
                let k = std::array::from_fn(|c| s.sus_k.read[i][c] + s.dep_k.read[i][c]);
                let scattering = std::array::from_fn(|c| s.sus_s.read[i][c] + s.dep_s.read[i][c]);
                let (r, t) = km::layer_rt(k, scattering, 1.0);
                let base = std::array::from_fn(|c| s.dried.read[i][c]);
                let color = km::composite(r, t, base);
                let shade = if let Some((azimuth, elevation, intensity)) = light {
                    let left = y * s.width + x.saturating_sub(1);
                    let right = y * s.width + (x + 1).min(s.width - 1);
                    let down = y.saturating_sub(1) * s.width + x;
                    let up = (y + 1).min(s.height - 1) * s.width + x;
                    let nx =
                        -0.025 * (s.paper[right][0] - s.paper[left][0]) as f64 / (2.0 * cell_mm);
                    let ny = 0.025 * (s.paper[up][0] - s.paper[down][0]) as f64 / (2.0 * cell_mm);
                    let dot = (nx * azimuth.cos() * elevation.cos()
                        + ny * azimuth.sin() * elevation.cos()
                        + elevation.sin())
                        / (nx * nx + ny * ny + 1.0).sqrt();
                    intensity * (0.45 + 0.55 * dot.max(0.0) / elevation.sin())
                } else {
                    1.0
                };
                rgb.extend(color.map(|v| km::linear_to_srgb_byte(v * shade as f32)));
            }
        }
        rgb
    }
}
