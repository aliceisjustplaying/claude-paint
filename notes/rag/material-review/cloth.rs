//! A coarse, persistent mesh for the face of a bunched rag. Positions are
//! millimeters relative to the hand, in canvas axes; z is clearance above
//! the canvas. Distance constraints resist stretching and bending. A soft
//! grip pulls toward the bunched rest shape, pressure lowers that grip and
//! surface friction holds contacting nodes back as the hand moves.
//!
//! This is an overdamped position-based cloth approximation, not a fabric
//! material calibration or a self-colliding simulation of the whole rag.
//! See https://matthias-research.github.io/pages/publications/posBasedDyn.pdf.

use super::{smoothstep, vn};

const N: usize = 19;
pub(super) const CELLS: usize = N * N;
const MAP: usize = 97;
const ITERATIONS: usize = 8;
// All mechanical coefficients below are visual-model estimates.
const GRIP: f32 = 0.045;
const STATIC_FRICTION: f32 = 0.8;
const SLIDING_FRICTION: f32 = 0.35;
const FIBER_MM: f32 = 0.18;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Cloth {
    rest: Vec<[f32; 3]>,
    pos: Vec<[f32; 3]>,
    links: Vec<(usize, usize, f32, f32)>,
    width: f32,
    seed: u64,
    travel_mm: f32,
}

pub(super) struct Contact {
    field: Vec<f32>,
    material: Vec<[f32; 2]>,
    extent: f32,
}

impl Cloth {
    pub(super) fn new(width: f32, seed: u64) -> Self {
        let mut rest = Vec::with_capacity(N * N);
        for y in 0..N {
            for x in 0..N {
                let u = 2.0 * x as f32 / (N - 1) as f32 - 1.0;
                let v = 2.0 * y as f32 / (N - 1) as f32 - 1.0;
                // Square-to-disk mapping keeps the grid connected. Initial
                // bunching is seeded once per fold, never once per stroke.
                let px = u * (1.0 - 0.5 * v * v).sqrt();
                let py = v * (1.0 - 0.5 * u * u).sqrt();
                let edge = 0.85 + 0.3 * vn(px * 3.0, py * 3.0, seed ^ 0x12);
                let px = px * 0.5 * width * edge;
                let py = py * 0.5 * width * edge;
                let crease = 0.65 * vn(px / 3.0, py / 8.0, seed)
                    + 0.35 * vn(px / 9.0, py / 12.0, seed ^ 0x51ED);
                let z = 4.0 * crease + 0.7 * (u * u + v * v);
                rest.push([px, py, z]);
            }
        }
        let mut links = Vec::new();
        for y in 0..N {
            for x in 0..N {
                let a = y * N + x;
                // Neighbors resist stretch and shear. Second neighbors
                // resist bending weakly, preserving folds without rigidity.
                for (dx, dy, stiffness) in [(1, 0, 0.85), (0, 1, 0.85), (1, 1, 0.6), (-1, 1, 0.6), (2, 0, 0.12), (0, 2, 0.12)] {
                    let (xx, yy) = (x as i32 + dx, y as i32 + dy);
                    if xx < 0 || xx >= N as i32 || yy >= N as i32 { continue; }
                    let b = yy as usize * N + xx as usize;
                    let length = (0..3).map(|k| (rest[a][k] - rest[b][k]).powi(2)).sum::<f32>().sqrt();
                    links.push((a, b, length, stiffness));
                }
            }
        }
        Self { pos: rest.clone(), rest, links, width, seed, travel_mm: 0.0 }
    }

    /// Advance by physical travel, independent of canvas pixel resolution.
    /// The plane is the coarse support surface; the paint engine separately
    /// resolves linen peaks and the solvent's reach into its hollows.
    pub(super) fn press(&mut self, pressure: f32, travel: [f32; 2]) -> Contact {
        let distance = travel[0].hypot(travel[1]);
        // Hold mechanical travel per substep constant when refining paint
        // sampling; otherwise half-sized stamps double the relaxation rate.
        let steps = if distance > 0.0 { (distance / 0.25).ceil().max(1.0) as usize } else { 4 };
        let motion = [travel[0] / steps as f32, travel[1] / steps as f32];
        let depression = 0.8 + 3.0 * pressure.clamp(0.0, 1.0);
        for _ in 0..steps {
            self.travel_mm += distance / steps as f32;
            // Hand motion changes the grip, not the rendered contact mask.
            // Persist its phase across wipes; refolding creates a new grip.
            // These amplitudes are estimates, not measured hand mechanics.
            let phase = self.travel_mm / self.width;
            let drift = |channel| {
                0.65 * (2.0 * vn(phase / 1.4, channel, self.seed ^ 0x4719) - 1.0)
                    + 0.35 * (2.0 * vn(phase / 0.45, channel, self.seed ^ 0x82AB) - 1.0)
            };
            let angle = 0.55 * drift(1.3);
            let (sin, cos) = angle.sin_cos();
            let shift = [0.09 * self.width * drift(3.7), 0.09 * self.width * drift(7.1)];
            let tilt = [1.6 * drift(11.3), 1.6 * drift(17.9)];
            let targets: Vec<[f32; 3]> = self.rest.iter().map(|rest| {
                let u = rest[0] / (0.5 * self.width);
                let v = rest[1] / (0.5 * self.width);
                [cos * rest[0] - sin * rest[1] + shift[0],
                 sin * rest[0] + cos * rest[1] + shift[1],
                 rest[2] - depression + tilt[0] * u + tilt[1] * v]
            }).collect();
            // Express the previous world-space positions in the moving hand
            // frame. Contacting points stick there until tangential pull
            // exceeds their normal-contact friction budget.
            let previous: Vec<[f32; 3]> = self.pos.iter().map(|p| [p[0] - motion[0], p[1] - motion[1], p[2]]).collect();
            self.pos.clone_from(&previous);
            for iteration in 0..ITERATIONS {
                for (p, target) in self.pos.iter_mut().zip(&targets) {
                    for k in 0..3 {
                        p[k] += GRIP * (target[k] - p[k]);
                    }
                }
                // Alternate order to avoid a persistent solver-direction bias.
                for j in 0..self.links.len() {
                    let i = if iteration % 2 == 0 { j } else { self.links.len() - 1 - j };
                    let (a, b, rest, stiffness) = self.links[i];
                    let d: [f32; 3] = std::array::from_fn(|k| self.pos[b][k] - self.pos[a][k]);
                    let length = d.iter().map(|x| x * x).sum::<f32>().sqrt();
                    if length < 1e-6 { continue; }
                    let amount = 0.5 * stiffness * (length - rest) / length;
                    for (k, delta) in d.into_iter().enumerate() {
                        self.pos[a][k] += amount * delta;
                        self.pos[b][k] -= amount * delta;
                    }
                }
                // Position-level Coulomb friction, Macklin et al. 2014,
                // section 6.1, eq. 24: https://mmacklin.com/uppfrta_preprint.pdf
                // Coefficients above remain uncalibrated estimates.
                for (p, old) in self.pos.iter_mut().zip(&previous) {
                    if p[2] >= 0.0 { continue; }
                    let normal = -p[2];
                    p[2] = 0.0;
                    let delta = [p[0] - old[0], p[1] - old[1]];
                    let tangent = delta[0].hypot(delta[1]);
                    let fraction = if tangent <= STATIC_FRICTION * normal { 1.0 }
                        else { (SLIDING_FRICTION * normal / tangent).min(1.0) };
                    p[0] -= fraction * delta[0];
                    p[1] -= fraction * delta[1];
                }
            }
        }
        self.contact()
    }

    fn contact(&self) -> Contact {
        let extent = self.pos.iter().flat_map(|p| [p[0].abs(), p[1].abs()]).fold(self.width * 0.5, f32::max) + 0.5;
        let mut contact = Contact { field: vec![0.0; MAP * MAP], material: vec![[0.0; 2]; MAP * MAP], extent };
        for y in 0..N - 1 {
            for x in 0..N - 1 {
                let a = y * N + x;
                for ids in [[a, a + 1, a + N], [a + 1, a + N + 1, a + N]] {
                    let p = ids.map(|i| self.pos[i]);
                    contact.triangle(p, ids.map(|i| [(i % N) as f32, (i / N) as f32]));
                }
            }
        }
        contact
    }
}

impl Contact {
    pub(super) fn extent(&self) -> f32 { self.extent }

    fn triangle(&mut self, p: [[f32; 3]; 3], uv: [[f32; 2]; 3]) {
        let to_grid = |v: f32| (v / self.extent + 1.0) * 0.5 * (MAP - 1) as f32;
        let q = p.map(|p| [to_grid(p[0]), to_grid(p[1])]);
        let area = (q[1][0] - q[0][0]) * (q[2][1] - q[0][1]) - (q[1][1] - q[0][1]) * (q[2][0] - q[0][0]);
        if area.abs() < 1e-6 { return; }
        let lo = |k| q.iter().map(|p| p[k]).fold(f32::INFINITY, f32::min).floor().max(0.0) as usize;
        let hi = |k| q.iter().map(|p| p[k]).fold(f32::NEG_INFINITY, f32::max).ceil().min((MAP - 1) as f32) as usize;
        for y in lo(1)..=hi(1) {
            for x in lo(0)..=hi(0) {
                let dx = x as f32 - q[0][0];
                let dy = y as f32 - q[0][1];
                let b = (dx * (q[2][1] - q[0][1]) - dy * (q[2][0] - q[0][0])) / area;
                let c = ((q[1][0] - q[0][0]) * dy - (q[1][1] - q[0][1]) * dx) / area;
                let a = 1.0 - b - c;
                if a < 0.0 || b < 0.0 || c < 0.0 { continue; }
                let gap = a * p[0][2] + b * p[1][2] + c * p[2][2];
                let touch = 1.0 - smoothstep(0.0, FIBER_MM, gap);
                let i = y * MAP + x;
                if touch > self.field[i] {
                    self.field[i] = touch;
                    self.material[i] = std::array::from_fn(|k| a * uv[0][k] + b * uv[1][k] + c * uv[2][k]);
                }
            }
        }
    }

    pub(super) fn at(&self, x: f32, y: f32) -> f32 {
        self.sample(x, y).0
    }

    /// Bilinear weights in the cloth's material coordinates. Paint stays on
    /// those material locations when their projected positions deform.
    pub(super) fn weights(&self, x: f32, y: f32) -> [(usize, f32); 4] {
        let (contact, uv) = self.sample(x, y);
        if contact <= 0.0 { return [(0, 0.0); 4]; }
        let u = uv[0].clamp(0.0, (N - 1) as f32);
        let v = uv[1].clamp(0.0, (N - 1) as f32);
        let ix = (u as usize).min(N - 2);
        let iy = (v as usize).min(N - 2);
        let (fx, fy) = (u - ix as f32, v - iy as f32);
        let i = iy * N + ix;
        [(i, (1.0 - fx) * (1.0 - fy)), (i + 1, fx * (1.0 - fy)),
         (i + N, (1.0 - fx) * fy), (i + N + 1, fx * fy)]
    }

    fn sample(&self, x: f32, y: f32) -> (f32, [f32; 2]) {
        let gx = (x / self.extent + 1.0) * 0.5 * (MAP - 1) as f32;
        let gy = (y / self.extent + 1.0) * 0.5 * (MAP - 1) as f32;
        if gx < 0.0 || gy < 0.0 || gx >= (MAP - 1) as f32 || gy >= (MAP - 1) as f32 { return (0.0, [0.0; 2]); }
        let (ix, iy) = (gx as usize, gy as usize);
        let (fx, fy) = (gx - ix as f32, gy - iy as f32);
        let i = iy * MAP + ix;
        let mut c = 0.0;
        let mut uv = [0.0; 2];
        for (j, w) in [(i, (1.0-fx)*(1.0-fy)), (i+1, fx*(1.0-fy)), (i+MAP, (1.0-fx)*fy), (i+MAP+1, fx*fy)] {
            let a = w * self.field[j];
            c += a;
            for k in 0..2 { uv[k] += a * self.material[j][k]; }
        }
        if c > 0.0 { for v in &mut uv { *v /= c; } }
        (c, uv)
    }
}
