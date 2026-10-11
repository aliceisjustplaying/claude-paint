use serde::{Deserialize, Serialize};
mod fiber_paper;
mod rng;

pub const TICK_NS: u64 = 62_500_000;
pub const DT: f64 = 0.0625;
pub const PIGMENTS: usize = 3;

#[derive(Clone, Copy)]
pub struct Pigment {
    pub name: &'static str,
    pub granulation: f64,
    pub staining: f64,
    pub settling: f64,
    pub k: [f64; 3],
    pub s: [f64; 3],
}

pub const PALETTE: [Pigment; PIGMENTS] = [
    Pigment {
        name: "Ultramarine-like",
        granulation: 0.95,
        staining: 0.08,
        settling: 0.028,
        k: [5.8, 4.0, 0.35],
        s: [0.9, 0.9, 1.2],
    },
    Pigment {
        name: "Phthalo-like",
        granulation: 0.04,
        staining: 0.96,
        settling: 0.013,
        k: [9.5, 1.2, 0.65],
        s: [0.5, 0.7, 0.8],
    },
    Pigment {
        name: "Earth",
        granulation: 0.62,
        staining: 0.3,
        settling: 0.05,
        k: [0.45, 2.3, 5.3],
        s: [1.4, 1.1, 0.85],
    },
];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    pub height: Vec<f64>,
    pub sizing: Vec<f64>,
    pub capacity: Vec<f64>,
    pub clump: Vec<f64>,
}

fn noise(seed: u64, x: i64, y: i64) -> f64 {
    let mut z = seed
        ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
        ^ (y as u64).wrapping_mul(0xbf58476d1ce4e5b9);
    z ^= z >> 30;
    z = z.wrapping_mul(0xbf58476d1ce4e5b9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94d049bb133111eb);
    ((z ^ (z >> 31)) >> 11) as f64 / (1u64 << 53) as f64
}

fn smooth(seed: u64, x: f64, y: f64) -> f64 {
    let ix = x.floor() as i64;
    let iy = y.floor() as i64;
    let tx = x - x.floor();
    let ty = y - y.floor();
    let tx = tx * tx * (3.0 - 2.0 * tx);
    let ty = ty * ty * (3.0 - 2.0 * ty);
    let a = noise(seed, ix, iy) * (1.0 - tx) + noise(seed, ix + 1, iy) * tx;
    let b = noise(seed, ix, iy + 1) * (1.0 - tx) + noise(seed, ix + 1, iy + 1) * tx;
    a * (1.0 - ty) + b * ty
}

impl Paper {
    pub fn cold_press(width: usize, height: usize, seed: u64) -> Self {
        let mut paper = Self {
            height: Vec::new(),
            sizing: Vec::new(),
            capacity: Vec::new(),
            clump: Vec::new(),
        };
        let support = fiber_paper::Paper {
            grammage: 300.0,
            calender: 0.05,
            felt: Some(fiber_paper::Felt {
                cell_mm: 0.85,
                depth_um: 32.0,
            }),
            ..fiber_paper::Paper::drawing(seed)
        };
        let sheet = fiber_paper::lay(&support, 0, 0, width, height, 0.2);
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                let (x, y) = (x as f64, y as f64);
                let h = (0.5 + 0.48 * (sheet.height[i] as f64 / 23.0).tanh()).clamp(0.0, 1.0);
                paper.height.push(h);
                paper
                    .sizing
                    .push(0.3 + 0.58 * smooth(seed + 3, x / 9.0, y / 9.0));
                paper.capacity.push(
                    (support.caliper_um() as f64 * support.porosity as f64 / 1000.0
                        * (sheet.grammage[i] as f64 / 300.0))
                        .clamp(0.1, 0.4),
                );
                paper.clump.push(
                    (0.3 + 1.4 * smooth(seed + 4, x / 2.3, y / 2.3))
                        * (sheet.micro[i] as f64 / 6.0).clamp(0.5, 1.5),
                );
            }
        }
        paper
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Glaze {
    pub deposited: Vec<[f64; PIGMENTS]>,
    pub bound: Vec<[f64; PIGMENTS]>,
}

impl Glaze {
    fn empty(n: usize) -> Self {
        Self {
            deposited: vec![[0.0; PIGMENTS]; n],
            bound: vec![[0.0; PIGMENTS]; n],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ledger {
    pub water_added: f64,
    pub water_evaporated: f64,
    pub water_lifted: f64,
    pub pigment_added: [f64; PIGMENTS],
    pub pigment_lifted: [f64; PIGMENTS],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Watercolor {
    pub schema: u32,
    pub width: usize,
    pub height: usize,
    pub paper: Paper,
    pub water: Vec<f64>,
    pub capillary: Vec<f64>,
    pub suspended: Vec<[f64; PIGMENTS]>,
    pub active: Glaze,
    pub glazes: Vec<Glaze>,
    pub wet_front: Vec<bool>,
    pub flux_x: Vec<f64>,
    pub flux_y: Vec<f64>,
    pub ledger: Ledger,
    pub tick: u64,
    pub residual_ns: u64,
}

#[derive(Debug)]
pub struct Balance {
    pub surface_water: f64,
    pub capillary_water: f64,
    pub suspended: [f64; PIGMENTS],
    pub deposited: [f64; PIGMENTS],
    pub bound: [f64; PIGMENTS],
    pub water_error: f64,
    pub pigment_error: [f64; PIGMENTS],
}

impl Watercolor {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        assert!(width > 1 && height > 1);
        let n = width * height;
        Self {
            schema: 1,
            width,
            height,
            paper: Paper::cold_press(width, height, seed),
            water: vec![0.0; n],
            capillary: vec![0.0; n],
            suspended: vec![[0.0; PIGMENTS]; n],
            active: Glaze::empty(n),
            glazes: Vec::new(),
            wet_front: vec![false; n],
            flux_x: vec![0.0; n],
            flux_y: vec![0.0; n],
            ledger: Ledger {
                water_added: 0.0,
                water_evaporated: 0.0,
                water_lifted: 0.0,
                pigment_added: [0.0; PIGMENTS],
                pigment_lifted: [0.0; PIGMENTS],
            },
            tick: 0,
            residual_ns: 0,
        }
    }

    pub fn add(&mut self, cell: usize, water: f64, pigment: [f64; PIGMENTS]) {
        assert!(water.is_finite() && water >= 0.0);
        assert!(pigment.iter().all(|v| v.is_finite() && *v >= 0.0));
        self.water[cell] += water;
        self.wet_front[cell] |= water > 0.0;
        self.ledger.water_added += water;
        for (p, amount) in pigment.iter().enumerate() {
            self.suspended[cell][p] += amount;
            self.ledger.pigment_added[p] += amount;
        }
    }

    pub fn dry_contact(&mut self, cell: usize, load: [f64; PIGMENTS], pressure: f64) {
        let contact = ((self.paper.height[cell] - (0.67 - pressure * 0.25)) * 12.0).clamp(0.0, 1.0);
        assert!(load.iter().all(|v| v.is_finite() && *v >= 0.0));
        for (p, amount) in load.iter().enumerate() {
            let m = amount * contact;
            self.active.deposited[cell][p] += m;
            self.ledger.pigment_added[p] += m;
        }
    }

    pub fn wait_seconds(&mut self, seconds: f64) {
        assert!(seconds.is_finite() && (0.0..=1e9).contains(&seconds));
        self.wait_ns((seconds * 1e9).round() as u64);
    }

    pub fn wait_ns(&mut self, ns: u64) {
        self.advance(ns, 0, |_| {});
    }

    pub fn wait_ns_observed(&mut self, ns: u64, every_ticks: u64, observer: impl FnMut(&Self)) {
        assert!(every_ticks > 0);
        self.advance(ns, every_ticks, observer);
    }

    fn advance(&mut self, ns: u64, every_ticks: u64, mut observer: impl FnMut(&Self)) {
        let total = self.residual_ns.checked_add(ns).expect("time overflow");
        let steps = total / TICK_NS;
        let residual = total % TICK_NS;
        self.residual_ns = 0;
        for _ in 0..steps {
            self.step();
            if every_ticks > 0 && self.tick % every_ticks == 0 {
                observer(self);
            }
        }
        self.residual_ns = residual;
    }

    pub fn snapshot(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn restore(bytes: &[u8]) -> Result<Self, String> {
        let state: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        let n = state
            .width
            .checked_mul(state.height)
            .ok_or("grid overflow")?;
        if state.schema != 1
            || state.width < 2
            || state.height < 2
            || state.residual_ns >= TICK_NS
            || [
                &state.water,
                &state.capillary,
                &state.flux_x,
                &state.flux_y,
                &state.paper.height,
                &state.paper.sizing,
                &state.paper.capacity,
                &state.paper.clump,
            ]
            .iter()
            .any(|v| v.len() != n || v.iter().any(|x| !x.is_finite()))
            || state.wet_front.len() != n
            || state.suspended.len() != n
            || std::iter::once(&state.active)
                .chain(state.glazes.iter())
                .any(|g| {
                    g.deposited.len() != n
                        || g.bound.len() != n
                        || g.deposited
                            .iter()
                            .chain(&g.bound)
                            .flatten()
                            .any(|v| !v.is_finite() || *v < 0.0)
                })
            || state
                .suspended
                .iter()
                .flatten()
                .any(|v| !v.is_finite() || *v < 0.0)
            || state.water.iter().chain(&state.capillary).any(|v| *v < 0.0)
            || state.paper.capacity.iter().any(|v| *v <= 0.0)
            || state
                .paper
                .height
                .iter()
                .chain(&state.paper.sizing)
                .any(|v| !(0.0..=1.0).contains(v))
            || state.paper.clump.iter().any(|v| *v < 0.0)
            || state
                .capillary
                .iter()
                .zip(&state.paper.capacity)
                .any(|(v, c)| *v > c + 1e-12)
            || [
                state.ledger.water_added,
                state.ledger.water_evaporated,
                state.ledger.water_lifted,
            ]
            .iter()
            .chain(&state.ledger.pigment_added)
            .chain(&state.ledger.pigment_lifted)
            .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err("invalid watercolor state".into());
        }
        Ok(state)
    }

    fn faces(&self) -> Vec<(usize, usize, bool)> {
        let mut faces = Vec::with_capacity(self.width * self.height * 2);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                if x + 1 < self.width {
                    faces.push((i, i + 1, true));
                }
                if y + 1 < self.height {
                    faces.push((i, i + self.width, false));
                }
            }
        }
        faces
    }

    fn step(&mut self) {
        let n = self.water.len();
        let faces = self.faces();
        let old_front = self.wet_front.clone();
        let mut transfers = Vec::with_capacity(faces.len());
        for &(i, j, horizontal) in &faces {
            if self.capillary[j] / self.paper.capacity[j] > 0.001
                && old_front[i]
                && self.water[i] > 0.22
            {
                self.wet_front[j] = true;
            }
            if self.capillary[i] / self.paper.capacity[i] > 0.001
                && old_front[j]
                && self.water[j] > 0.22
            {
                self.wet_front[i] = true;
            }
            let previous = if horizontal {
                self.flux_x[i]
            } else {
                self.flux_y[i]
            };
            let head_i = self.water[i] + 0.012 * self.paper.height[i];
            let head_j = self.water[j] + 0.012 * self.paper.height[j];
            let depth = (self.water[i] + self.water[j]) * 0.5;
            let flux = if old_front[i] && old_front[j] {
                0.55 * previous + DT * 65.0 * depth * (head_i - head_j)
            } else {
                0.0
            };
            if horizontal {
                self.flux_x[i] = flux;
            } else {
                self.flux_y[i] = flux;
            }
            if flux >= 0.0 {
                transfers.push((i, j, flux * DT));
            } else {
                transfers.push((j, i, -flux * DT));
            }
        }
        let mut outgoing = vec![0.0; n];
        for &(i, _, amount) in &transfers {
            outgoing[i] += amount;
        }
        let mut delta = vec![0.0; n];
        let mut pigment_delta = vec![[0.0; PIGMENTS]; n];
        for &(i, j, requested) in &transfers {
            let amount = requested * (self.water[i] / outgoing[i].max(1e-30)).min(1.0);
            delta[i] -= amount;
            delta[j] += amount;
            for p in 0..PIGMENTS {
                let moved = self.suspended[i][p] * amount / self.water[i].max(1e-30);
                pigment_delta[i][p] -= moved;
                pigment_delta[j][p] += moved;
            }
        }
        for i in 0..n {
            self.water[i] = (self.water[i] + delta[i]).max(0.0);
            for p in 0..PIGMENTS {
                self.suspended[i][p] = (self.suspended[i][p] + pigment_delta[i][p]).max(0.0);
            }
        }
        for p in 0..PIGMENTS {
            let mut moves = Vec::new();
            let mut out = vec![0.0; n];
            for &(i, j, _) in &faces {
                if self.water[i] < 1e-7 || self.water[j] < 1e-7 {
                    continue;
                }
                let diff = DT
                    * 0.045
                    * self.water[i].min(self.water[j])
                    * (self.suspended[i][p] / self.water[i] - self.suspended[j][p] / self.water[j]);
                let (from, to, m) = if diff >= 0.0 {
                    (i, j, diff)
                } else {
                    (j, i, -diff)
                };
                out[from] += m;
                moves.push((from, to, m));
            }
            let mut change = vec![0.0; n];
            for (i, j, requested) in moves {
                let moved = requested * (self.suspended[i][p] / out[i].max(1e-30)).min(1.0);
                change[i] -= moved;
                change[j] += moved;
            }
            for (i, d) in change.iter().enumerate() {
                self.suspended[i][p] = (self.suspended[i][p] + d).max(0.0);
            }
        }
        let mut cap_change = vec![0.0; n];
        let mut cap_moves = Vec::new();
        let mut cap_out = vec![0.0; n];
        let mut cap_in = vec![0.0; n];
        for &(i, j, _) in &faces {
            let d = DT
                * 0.035
                * (self.capillary[i] / self.paper.capacity[i]
                    - self.capillary[j] / self.paper.capacity[j]);
            let (a, b, m) = if d >= 0.0 { (i, j, d) } else { (j, i, -d) };
            cap_out[a] += m;
            cap_in[b] += m;
            cap_moves.push((a, b, m));
        }
        for (i, j, m) in cap_moves {
            let limit = (self.capillary[i] / cap_out[i].max(1e-30))
                .min(1.0)
                .min((self.paper.capacity[j] - self.capillary[j]) / cap_in[j].max(1e-30));
            let moved = m * limit.max(0.0);
            cap_change[i] -= moved;
            cap_change[j] += moved;
        }
        for (i, change) in cap_change.iter().enumerate() {
            self.capillary[i] += change;
        }
        let edge_water = self.water.clone();
        for i in 0..n {
            let saturation = self.capillary[i] / self.paper.capacity[i];
            let uptake = (DT * 0.016 * (1.0 - self.paper.sizing[i]) * (1.0 - saturation))
                .min(self.water[i])
                .min((self.paper.capacity[i] - self.capillary[i]).max(0.0));
            self.water[i] -= uptake;
            self.capillary[i] += uptake;
            let mut edge = 0.0;
            let x = i % self.width;
            let y = i / self.width;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                    edge += 0.25;
                } else {
                    let j = ny as usize * self.width + nx as usize;
                    if !self.wet_front[j] || edge_water[j] < edge_water[i] * 0.3 {
                        edge += 0.25;
                    }
                }
            }
            let evap = (DT * 0.0035 * (0.75 + 0.5 * self.paper.sizing[i]) * (1.0 + edge * 8.0))
                .min(self.water[i]);
            self.water[i] -= evap;
            let cap_evap = (DT
                * 0.0018
                * (0.7 + self.paper.sizing[i])
                * (self.capillary[i] / self.paper.capacity[i]).sqrt())
            .min(self.capillary[i]);
            self.capillary[i] -= cap_evap;
            self.ledger.water_evaporated += evap + cap_evap;
            let wet = (self.water[i] / 0.035).clamp(0.0, 1.0);
            for (p, pigment) in PALETTE.iter().enumerate() {
                let valley = (1.0 - self.paper.height[i]).powi(3) * 9.0 * self.paper.clump[i];
                let rate =
                    pigment.settling * (0.22 + pigment.granulation * valley + 1.4 * (1.0 - wet));
                let adsorbed = if self.water[i] < 1e-10 {
                    self.suspended[i][p]
                } else {
                    self.suspended[i][p] * (1.0 - (-rate * DT).exp())
                };
                self.suspended[i][p] -= adsorbed;
                self.active.deposited[i][p] += adsorbed;
                let binding_room = (pigment.staining
                    * (self.active.deposited[i][p] + self.active.bound[i][p])
                    - self.active.bound[i][p])
                    .max(0.0);
                let bind = (self.active.deposited[i][p]
                    * (1.0
                        - (-DT
                            * pigment.staining
                            * 0.8
                            * (0.3 + 1.0 - wet)
                            * (self.capillary[i] / 0.02 + wet).clamp(0.0, 1.0))
                        .exp()))
                .min(binding_room);
                self.active.deposited[i][p] -= bind;
                self.active.bound[i][p] += bind;
                let released = self.active.deposited[i][p]
                    * (1.0 - (-DT * 0.035 * wet * (1.0 - pigment.staining)).exp());
                self.active.deposited[i][p] -= released;
                self.suspended[i][p] += released;
                for glaze in &mut self.glazes {
                    let rewet = glaze.deposited[i][p]
                        * (1.0 - (-DT * 0.11 * wet * (1.0 - pigment.staining)).exp());
                    glaze.deposited[i][p] -= rewet;
                    self.suspended[i][p] += rewet;
                }
            }
            if self.water[i] < 1e-10 && self.capillary[i] < 1e-10 {
                self.wet_front[i] = false;
            }
        }
        self.tick += 1;
    }

    pub fn finish_glaze(&mut self) -> Result<(), &'static str> {
        if self.water.iter().chain(&self.capillary).any(|v| *v > 1e-10)
            || self.suspended.iter().flatten().any(|v| *v > 1e-10)
        {
            return Err("glaze is still wet");
        }
        self.glazes.push(std::mem::replace(
            &mut self.active,
            Glaze::empty(self.water.len()),
        ));
        Ok(())
    }

    pub fn lift(&mut self, cell: usize, contact: f64) {
        assert!(contact.is_finite() && (0.0..=1.0).contains(&contact));
        let wet = (self.water[cell] / 0.025).clamp(0.0, 1.0);
        let water = self.water[cell] * contact * 0.75;
        self.water[cell] -= water;
        self.ledger.water_lifted += water;
        for (p, pigment) in PALETTE.iter().enumerate() {
            let suspension = self.suspended[cell][p] * contact * 0.75;
            self.suspended[cell][p] -= suspension;
            let mut removed = suspension;
            for glaze in std::iter::once(&mut self.active).chain(self.glazes.iter_mut()) {
                let loose = glaze.deposited[cell][p] * wet * contact * 0.8;
                let bound = glaze.bound[cell][p] * wet * contact * (1.0 - pigment.staining) * 0.12;
                glaze.deposited[cell][p] -= loose;
                glaze.bound[cell][p] -= bound;
                removed += loose + bound;
            }
            self.ledger.pigment_lifted[p] += removed;
        }
    }

    pub fn balance(&self) -> Balance {
        let mut balance = Balance {
            surface_water: self.water.iter().sum(),
            capillary_water: self.capillary.iter().sum(),
            suspended: [0.0; PIGMENTS],
            deposited: [0.0; PIGMENTS],
            bound: [0.0; PIGMENTS],
            water_error: 0.0,
            pigment_error: [0.0; PIGMENTS],
        };
        for i in 0..self.water.len() {
            for p in 0..PIGMENTS {
                balance.suspended[p] += self.suspended[i][p];
                for glaze in std::iter::once(&self.active).chain(self.glazes.iter()) {
                    balance.deposited[p] += glaze.deposited[i][p];
                    balance.bound[p] += glaze.bound[i][p];
                }
            }
        }
        balance.water_error = balance.surface_water
            + balance.capillary_water
            + self.ledger.water_evaporated
            + self.ledger.water_lifted
            - self.ledger.water_added;
        for p in 0..PIGMENTS {
            balance.pigment_error[p] = balance.suspended[p]
                + balance.deposited[p]
                + balance.bound[p]
                + self.ledger.pigment_lifted[p]
                - self.ledger.pigment_added[p];
        }
        balance
    }

    pub fn rgb(&self, raking: bool) -> Vec<u8> {
        let mut image = Vec::with_capacity(self.water.len() * 3);
        for i in 0..self.water.len() {
            let mut reflectance = [0.92, 0.90, 0.85];
            for glaze in &self.glazes {
                let mass = std::array::from_fn(|p| glaze.deposited[i][p] + glaze.bound[i][p]);
                reflectance = compose_km(reflectance, mass);
            }
            let mass = std::array::from_fn(|p| {
                self.active.deposited[i][p] + self.active.bound[i][p] + self.suspended[i][p]
            });
            reflectance = compose_km(reflectance, mass);
            let x = i % self.width;
            let y = i / self.width;
            let left = y * self.width + x.saturating_sub(1);
            let right = y * self.width + (x + 1).min(self.width - 1);
            let up = y.saturating_sub(1) * self.width + x;
            let down = (y + 1).min(self.height - 1) * self.width + x;
            let dx = self.paper.height[right] - self.paper.height[left];
            let dy = self.paper.height[down] - self.paper.height[up];
            let shade = if raking {
                (0.91 + 0.6 * (dx - dy)).clamp(0.62, 1.15)
            } else {
                (0.99 + 0.08 * (dx - dy)).clamp(0.94, 1.04)
            };
            for c in reflectance {
                let linear = (c * shade).clamp(0.0, 1.0);
                let srgb = if linear <= 0.0031308 {
                    12.92 * linear
                } else {
                    1.055 * linear.powf(1.0 / 2.4) - 0.055
                };
                image.push((srgb * 255.0).round() as u8);
            }
        }
        image
    }
}

pub fn compose_km(base: [f64; 3], mass: [f64; PIGMENTS]) -> [f64; 3] {
    std::array::from_fn(|c| {
        let k: f64 = (0..PIGMENTS)
            .map(|p| PALETTE[p].k[c] * mass[p] * 30.0)
            .sum();
        let s: f64 = (0..PIGMENTS)
            .map(|p| PALETTE[p].s[c] * mass[p] * 30.0)
            .sum();
        if s < 1e-14 {
            return base[c] * (-2.0 * k).exp();
        }
        let a = 1.0 + k / s;
        let b = (a * a - 1.0).sqrt();
        let z = b * s;
        let e = (-2.0 * z).exp();
        let denom = a * (1.0 - e) + b * (1.0 + e);
        let r = (1.0 - e) / denom;
        let t = 2.0 * b * (-z).exp() / denom;
        r + t * t * base[c] / (1.0 - r * base[c]).max(1e-14)
    })
}
