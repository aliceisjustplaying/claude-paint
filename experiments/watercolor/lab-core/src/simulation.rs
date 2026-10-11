// Mechanical port of Watercolor Lab, copyright (c) 2026 Ryan McCarty, MIT.
// Source: uxpreview/watercolor-lab a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See experiments/watercolor/THIRD_PARTY_NOTICES.md for the full MIT license.
use crate::km::{composite as km_composite, layer_rt as km_layer};
use crate::{
    brush::Stamp,
    paper::{generate_paper, PaperKind},
    pigments::Pigment,
};
use half::f16;
use serde::{Deserialize, Serialize};

pub type Pixel = [f32; 4];

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub gravity: f32,
    pub damp: f32,
    pub slope: f32,
    pub tension: f32,
    pub salt_pull: f32,
    pub evap: f32,
    pub evap_edge: f32,
    pub absorb: f32,
    pub cap_diff: f32,
    pub cap_dry: f32,
    pub vel_scale: f32,
    pub advect: f32,
    pub edge_drift: f32,
    pub settle: f32,
    pub gran_bias: f32,
    pub lift: f32,
    pub salt_lift: f32,
    pub scrub_lift: f32,
    pub edge_settle: f32,
    pub substeps: usize,
    pub zoom_grain: f32,
}

pub const DEFAULT_PARAMS: Params = Params {
    gravity: 0.14,
    damp: 0.982,
    slope: 0.12,
    tension: 0.02,
    salt_pull: 0.8,
    evap: 0.00016,
    evap_edge: 0.0016,
    absorb: 0.00045,
    cap_diff: 0.04,
    cap_dry: 0.00025,
    vel_scale: 20.0,
    advect: 1.0,
    edge_drift: 4.5,
    settle: 0.0018,
    gran_bias: 1.1,
    lift: 0.005,
    salt_lift: 0.5,
    scrub_lift: 0.9,
    edge_settle: 14.0,
    substeps: 3,
    zoom_grain: 0.006,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PingPong {
    pub read: Vec<Pixel>,
    pub write: Vec<Pixel>,
}
impl PingPong {
    // src/engine/gl.ts:119-130 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn new(n: usize) -> Self {
        Self {
            read: vec![[0.0; 4]; n],
            write: vec![[0.0; 4]; n],
        }
    }
    // src/engine/gl.ts:123-127 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Simulation {
    pub width: usize,
    pub height: usize,
    pub params: Params,
    pub flow: PingPong,
    pub flux: PingPong,
    pub sus_k: PingPong,
    pub sus_s: PingPong,
    pub sus_p: PingPong,
    pub dep_k: PingPong,
    pub dep_s: PingPong,
    pub cap: PingPong,
    pub dried: PingPong,
    pub blur_w: Vec<Pixel>,
    pub paper: Vec<Pixel>,
    pub paper_kind: PaperKind,
    pub paper_seed: u32,
    pub fast_dry_steps: i32,
    pub forever_wet: bool,
    pub frame_accumulator: f64,
    pub undo_ring: Vec<Vec<Vec<Pixel>>>,
    pub undo_live: Vec<usize>,
    pub undo_cursor: usize,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Probe {
    pub dried: f64,
    pub workable: f64,
    pub wet: f64,
}

// src/engine/gl.ts:97-109,168-173: RGBA16F upload/write conversion at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn quantize(v: Pixel) -> Pixel {
    v.map(|x| f16::from_f32(x).to_f32())
}
// src/engine/shaders.ts:81-84,188-191 at a19bfa2a026856a98ceb400a8eec41f091f04a11: GLSL mix.
fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
// GLSL built-in smoothstep, including reversed edges in src/engine/shaders.ts:187-191,227-231 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
// src/engine/gl.ts:97-104: LINEAR and CLAMP_TO_EDGE, texel centers at (i+0.5)/size.
pub fn sample(data: &[Pixel], w: usize, h: usize, u: f32, v: f32) -> Pixel {
    let x = (u * w as f32 - 0.5).clamp(0.0, (w - 1) as f32);
    let y = (v * h as f32 - 0.5).clamp(0.0, (h - 1) as f32);
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    std::array::from_fn(|c| {
        mix(
            mix(data[y0 * w + x0][c], data[y0 * w + x1][c], x - x0 as f32),
            mix(data[y1 * w + x0][c], data[y1 * w + x1][c], x - x0 as f32),
            y - y0 as f32,
        )
    })
}
// src/engine/shaders.ts:47,95-101: cell-offset samples with clamp-to-edge.
fn cell(data: &[Pixel], w: usize, h: usize, x: usize, y: usize, dx: isize, dy: isize) -> Pixel {
    data[(y as isize + dy).clamp(0, h as isize - 1) as usize * w
        + (x as isize + dx).clamp(0, w as isize - 1) as usize]
}

impl Simulation {
    // src/engine/simulation.ts:173-229 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn new(width: usize, height: usize, paper_kind: PaperKind, seed: u32) -> Self {
        assert!(width > 0 && height > 0);
        let n = width * height;
        let mut sim = Self {
            width,
            height,
            params: DEFAULT_PARAMS,
            flow: PingPong::new(n),
            flux: PingPong::new(n),
            sus_k: PingPong::new(n),
            sus_s: PingPong::new(n),
            sus_p: PingPong::new(n),
            dep_k: PingPong::new(n),
            dep_s: PingPong::new(n),
            cap: PingPong::new(n),
            dried: PingPong::new(n),
            blur_w: vec![[0.0; 4]; n],
            paper: vec![[0.0; 4]; n],
            paper_kind,
            paper_seed: seed,
            fast_dry_steps: 0,
            forever_wet: false,
            frame_accumulator: 0.0,
            undo_ring: Vec::new(),
            undo_live: Vec::new(),
            undo_cursor: 0,
        };
        sim.set_paper(paper_kind, false, seed);
        sim.reset();
        sim
    }
    // src/engine/simulation.ts:239-252 at a19bfa2a026856a98ceb400a8eec41f091f04a11; paper upload is RGBA16F.
    pub fn set_paper(&mut self, kind: PaperKind, clear: bool, seed: u32) {
        self.paper_kind = kind;
        self.paper_seed = seed;
        self.paper = generate_paper(self.width, self.height, kind, seed)
            .into_iter()
            .map(quantize)
            .collect();
        if clear {
            self.reset();
        }
    }
    // src/engine/simulation.ts:254-264 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn reset(&mut self) {
        for p in [
            &mut self.flow,
            &mut self.flux,
            &mut self.sus_k,
            &mut self.sus_s,
            &mut self.sus_p,
            &mut self.dep_k,
            &mut self.dep_s,
            &mut self.cap,
        ] {
            p.read.fill([0.0; 4]);
            p.write.fill([0.0; 4]);
        }
        let paper = crate::km::PAPER_REFLECTANCE;
        let clear = quantize([paper[0] as f32, paper[1] as f32, paper[2] as f32, 0.0]);
        self.dried.read.fill(clear);
        self.dried.write.fill(clear);
        self.undo_live.clear();
        self.fast_dry_steps = 0;
    }
    // src/engine/simulation.ts:400-407 at a19bfa2a026856a98ceb400a8eec41f091f04a11: no elapsed-time shader uniform.
    pub fn update(&mut self) {
        let steps = self.params.substeps + if self.fast_dry_steps > 0 { 13 } else { 0 };
        let evap = if self.fast_dry_steps > 0 {
            28.0
        } else if self.forever_wet {
            0.0
        } else {
            1.0
        };
        for _ in 0..steps {
            self.substep(evap);
        }
        if self.fast_dry_steps > 0 {
            self.fast_dry_steps -= steps as i32;
        }
    }
    // Host clock adapter only: src/engine/simulation.ts:400-407 is frame-based at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn step(&mut self, dt_seconds: f64) {
        self.frame_accumulator += dt_seconds * 60.0;
        while self.frame_accumulator >= 1.0 {
            self.update();
            self.frame_accumulator -= 1.0;
        }
    }
    // src/engine/simulation.ts:408-410 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn step_many(&mut self, steps: usize, evap_mul: f32) {
        for _ in 0..steps {
            self.substep(evap_mul);
        }
    }
    // src/engine/simulation.ts:393-395 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn dry_fast(&mut self) {
        self.fast_dry_steps = 480;
    }
    // src/engine/simulation.ts:446-575 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn substep(&mut self, evap_mul: f32) {
        self.substep_with_callback(evap_mul, |_, _| {});
    }
    // src/engine/simulation.ts:446-575 at a19bfa2a026856a98ceb400a8eec41f091f04a11; callback observes attachments before swap.
    pub fn substep_with_callback(&mut self, evap_mul: f32, mut callback: impl FnMut(&str, &Self)) {
        self.blur_wet();
        callback("blur_wet", self);
        self.flux_pass();
        callback("flux", self);
        self.flux.swap();
        self.water(evap_mul);
        callback("water", self);
        self.flow.swap();
        self.move_pigment();
        callback("move_pigment", self);
        self.sus_k.swap();
        self.sus_s.swap();
        self.sus_p.swap();
        self.transfer_dep();
        callback("transfer_dep", self);
        self.transfer_sus();
        callback("transfer_sus", self);
        self.dep_k.swap();
        self.dep_s.swap();
        self.sus_k.swap();
        self.sus_s.swap();
        self.sus_p.swap();
        self.capillary(evap_mul);
        callback("capillary", self);
        self.cap.swap();
        self.dry_fold();
        callback("dry_fold", self);
        self.dried.swap();
        self.dry_clear_sus();
        callback("dry_clear_sus", self);
        self.dry_clear_dep();
        callback("dry_clear_dep", self);
        self.sus_k.swap();
        self.sus_s.swap();
        self.sus_p.swap();
        self.dep_k.swap();
        self.dep_s.swap();
    }
    // src/engine/shaders.ts:39-51 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn blur_wet(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let weight = if dx == 0 && dy == 0 {
                            4.0
                        } else if dx == 0 || dy == 0 {
                            2.0
                        } else {
                            1.0
                        };
                        sum += weight
                            * cell(
                                &self.flow.read,
                                self.width,
                                self.height,
                                x,
                                y,
                                dx * 2,
                                dy * 2,
                            )[3];
                    }
                }
                self.blur_w[y * self.width + x] = quantize([sum / 16.0, 0.0, 0.0, 0.0]);
            }
        }
    }
    // src/engine/shaders.ts:55-115 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn flux_pass(&mut self) {
        let p = self.params;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let flow = self.flow.read[i];
                let flux = self.flux.read[i];
                let head = flow[2] + self.paper[i][0] * p.slope - self.cap.read[i][1] * p.salt_pull;
                let mut next = [0.0; 4];
                for (c, (dx, dy)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate() {
                    let there = cell(&self.flow.read, self.width, self.height, x, y, dx, dy);
                    let paper = cell(&self.paper, self.width, self.height, x, y, dx, dy);
                    let cap = cell(&self.cap.read, self.width, self.height, x, y, dx, dy);
                    let dh = head - (there[2] + paper[0] * p.slope - cap[1] * p.salt_pull);
                    let fiber_mix = (1.0 - flow[2] * 5.0).clamp(0.25, 1.0);
                    let fiber = mix(1.0, 0.55 + 1.1 * paper[0], fiber_mix);
                    let invade = smoothstep(p.tension * fiber, p.tension * fiber * 2.5, flow[2]);
                    let gate = mix(invade, 1.0, if there[3] >= 0.05 { 1.0 } else { 0.0 });
                    next[c] = (flux[c] * p.damp + p.gravity * dh).max(0.0) * gate;
                }
                let total = next[0] + next[1] + next[2] + next[3];
                let scale = if total > 1e-6 {
                    (flow[2] * 0.9 / total).min(1.0)
                } else {
                    0.0
                };
                for value in &mut next {
                    *value *= scale;
                }
                if x == 0 {
                    next[0] = 0.0;
                }
                if x == self.width - 1 {
                    next[1] = 0.0;
                }
                if y == 0 {
                    next[2] = 0.0;
                }
                if y == self.height - 1 {
                    next[3] = 0.0;
                }
                self.flux.write[i] = quantize(next);
            }
        }
    }
    // src/engine/shaders.ts:119-162 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn water(&mut self, evap_mul: f32) {
        let p = self.params;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let flow = self.flow.read[i];
                let f = self.flux.read[i];
                let il = cell(&self.flux.read, self.width, self.height, x, y, -1, 0)[1];
                let ir = cell(&self.flux.read, self.width, self.height, x, y, 1, 0)[0];
                let id = cell(&self.flux.read, self.width, self.height, x, y, 0, -1)[3];
                let iu = cell(&self.flux.read, self.width, self.height, x, y, 0, 1)[2];
                let mut water = flow[2] + (il + ir + id + iu) - (f[0] + f[1] + f[2] + f[3]);
                let edge = (flow[3] - self.blur_w[i][0]).clamp(0.0, 1.0);
                water -= p.evap * evap_mul + p.evap_edge * evap_mul * edge;
                water -= water.min(p.absorb * (1.0 - self.cap.read[i][0]));
                water = water.max(0.0);
                let vx = p.vel_scale * 0.5 * ((f[1] - f[0]) + (il - ir));
                let vy = p.vel_scale * 0.5 * ((f[3] - f[2]) + (id - iu));
                let wet = (flow[3].clamp(0.0, 1.0) * if water > 0.002 { 1.0 } else { 0.985 })
                    .max((water * 200.0).min(1.0));
                self.flow.write[i] = quantize([vx, vy, water, wet]);
            }
        }
    }
    // src/engine/shaders.ts:167-196 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn move_pigment(&mut self) {
        let p = self.params;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let f = self.flow.read[i];
                let gx = cell(&self.blur_w, self.width, self.height, x, y, 1, 0)[0]
                    - cell(&self.blur_w, self.width, self.height, x, y, -1, 0)[0];
                let gy = cell(&self.blur_w, self.width, self.height, x, y, 0, 1)[0]
                    - cell(&self.blur_w, self.width, self.height, x, y, 0, -1)[0];
                let shallow = smoothstep(0.12, 0.0, f[2]);
                let vx = f[0] - gx * p.edge_drift * shallow * f[3];
                let vy = f[1] - gy * p.edge_drift * shallow * f[3];
                let u = (x as f32 + 0.5) / self.width as f32;
                let v = (y as f32 + 0.5) / self.height as f32;
                let moving = if f[3] >= 0.01 { 1.0 } else { 0.0 };
                let at_u = mix(u, u - vx * p.advect * (1.0 / self.width as f32), moving);
                let at_v = mix(v, v - vy * p.advect * (1.0 / self.height as f32), moving);
                self.sus_k.write[i] = quantize(sample(
                    &self.sus_k.read,
                    self.width,
                    self.height,
                    at_u,
                    at_v,
                ));
                self.sus_s.write[i] = quantize(sample(
                    &self.sus_s.read,
                    self.width,
                    self.height,
                    at_u,
                    at_v,
                ));
                self.sus_p.write[i] = quantize(sample(
                    &self.sus_p.read,
                    self.width,
                    self.height,
                    at_u,
                    at_v,
                ));
            }
        }
    }
    // src/engine/shaders.ts:218-262 at a19bfa2a026856a98ceb400a8eec41f091f04a11, shared by both transfer passes.
    fn exchange(&self, i: usize) -> (f32, f32) {
        let p = self.params;
        let f = self.flow.read[i];
        let sk = self.sus_k.read[i];
        let sp = self.sus_p.read[i];
        let dk = self.dep_k.read[i];
        let ds = self.dep_s.read[i];
        let ph = self.paper[i][0];
        let salt = self.cap.read[i][1];
        let scrub = self.cap.read[i][2];
        let edge = ((f[3] - self.blur_w[i][0]) * 3.0).clamp(0.0, 1.0);
        let conc = sk[3].max(1e-6);
        let density = sp[0] / conc;
        let gran = sp[1] / conc;
        let speed = (f[0] * f[0] + f[1] * f[1]).sqrt();
        let calm = 1.0 / (1.0 + speed * 6.0);
        let shallow = smoothstep(0.35, 0.02, f[2]);
        let mut valley = (1.0 + p.gran_bias * gran * (0.5 - ph) * 2.0).max(0.0);
        valley *= valley;
        let mut settle = p.settle * (0.25 + density) * calm * valley * (0.35 + 0.65 * shallow);
        settle *= 1.0 - 0.85 * (salt * 4.0).clamp(0.0, 1.0);
        settle /= 1.0 + scrub * 6.0;
        settle *= 1.0 + p.edge_settle * edge;
        let down = sk[3] * settle.clamp(0.0, 0.2);
        let stained = ds[3] / dk[3].max(1e-6);
        let wet = if f[2] >= 0.002 { 1.0 } else { 0.0 };
        let peak = 1.0 + gran * (ph - 0.5) * 1.6;
        let mut lift =
            p.lift * wet * (0.3 + speed * 4.0) * (1.0 - stained.clamp(0.0, 1.0)) * peak.max(0.0);
        lift *= 1.0 - 0.75 * edge;
        lift += p.salt_lift * salt * wet;
        lift += p.scrub_lift * scrub * wet * (1.0 - 0.7 * stained.clamp(0.0, 1.0));
        (down, dk[3] * lift.clamp(0.0, 0.5))
    }
    // src/engine/shaders.ts:301-329 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn transfer_dep(&mut self) {
        for i in 0..self.width * self.height {
            let (down, up) = self.exchange(i);
            let sk = self.sus_k.read[i];
            let ss = self.sus_s.read[i];
            let sp = self.sus_p.read[i];
            let dk = self.dep_k.read[i];
            let ds = self.dep_s.read[i];
            let conc = sk[3].max(1e-6);
            let down_frac = down / conc;
            let up_frac = up / dk[3].max(1e-6);
            self.dep_k.write[i] = quantize(std::array::from_fn(|c| {
                dk[c] * (1.0 - up_frac) + sk[c] * down_frac
            }));
            self.dep_s.write[i] = quantize(std::array::from_fn(|c| {
                if c == 3 {
                    ds[3] * (1.0 - up_frac) + down * (sp[2] / conc).clamp(0.0, 1.0)
                } else {
                    ds[c] * (1.0 - up_frac) + ss[c] * down_frac
                }
            }));
        }
    }
    // src/engine/shaders.ts:263-299 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn transfer_sus(&mut self) {
        for i in 0..self.width * self.height {
            let (down, up) = self.exchange(i);
            let sk = self.sus_k.read[i];
            let ss = self.sus_s.read[i];
            let sp = self.sus_p.read[i];
            let dk = self.dep_k.read[i];
            let ds = self.dep_s.read[i];
            let down_frac = down / sk[3].max(1e-6);
            let depth = dk[3].max(1e-6);
            let up_frac = up / depth;
            let lifted = [0.5 * up, 0.3 * up, ds[3] / depth * up];
            let drink = (self.cap.read[i][2] * 0.35).clamp(0.0, 0.2);
            self.sus_k.write[i] = quantize(std::array::from_fn(|c| {
                (sk[c] * (1.0 - down_frac) + dk[c] * up_frac) * (1.0 - drink)
            }));
            self.sus_s.write[i] = quantize(std::array::from_fn(|c| {
                (ss[c] * (1.0 - down_frac) + ds[c] * up_frac) * (1.0 - drink)
            }));
            self.sus_p.write[i] = quantize(std::array::from_fn(|c| {
                if c == 3 {
                    0.0
                } else {
                    (sp[c] * (1.0 - down_frac) + lifted[c]) * (1.0 - drink)
                }
            }));
        }
    }
    // src/engine/shaders.ts:335-363 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn capillary(&mut self, evap_mul: f32) {
        let p = self.params;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let cap = self.cap.read[i];
                let flow = self.flow.read[i];
                let avg = 0.25
                    * (cell(&self.cap.read, self.width, self.height, x, y, -1, 0)[0]
                        + cell(&self.cap.read, self.width, self.height, x, y, 1, 0)[0]
                        + cell(&self.cap.read, self.width, self.height, x, y, 0, -1)[0]
                        + cell(&self.cap.read, self.width, self.height, x, y, 0, 1)[0]);
                let mut s = cap[0] + p.cap_diff * (avg - cap[0]);
                s += flow[2].min(p.absorb * (1.0 - s));
                s = (s - p.cap_dry * evap_mul).max(0.0);
                self.cap.write[i] =
                    quantize([s, cap[1] * (1.0 - 0.0015 * evap_mul), cap[2] * 0.94, 0.0]);
            }
        }
    }
    // src/engine/shaders.ts:367-409 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn dry_fold(&mut self) {
        for i in 0..self.width * self.height {
            let dried = self.dried.read[i];
            let f = self.flow.read[i];
            let dk = self.dep_k.read[i];
            let sk = self.sus_k.read[i];
            let pigment = dk[3] + sk[3];
            if !(f[3] < 0.02 && pigment > 1e-4) {
                self.dried.write[i] = dried;
                continue;
            }
            let k = std::array::from_fn(|c| dk[c] + sk[c]);
            let s = std::array::from_fn(|c| self.dep_s.read[i][c] + self.sus_s.read[i][c]);
            let mut gran = (self.sus_p.read[i][1] / sk[3].max(1e-5)).clamp(0.0, 1.0);
            gran *= (pigment * 2.2).clamp(0.15, 1.0);
            let paper = self.paper[i];
            let pit = smoothstep(0.62, 0.32, paper[0] * 0.55 + paper[1] * 0.45);
            let thickness = 1.0 + gran * (0.9 * pit - 0.35);
            let (r, t) = km_layer(k, s, thickness);
            let folded = km_composite(r, t, [dried[0], dried[1], dried[2]]);
            self.dried.write[i] = quantize([
                folded[0],
                folded[1],
                folded[2],
                dried[3] + thickness * (k[0] + k[1] + k[2]) / 3.0,
            ]);
        }
    }
    // src/engine/shaders.ts:413-432 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn dry_clear_sus(&mut self) {
        for i in 0..self.width * self.height {
            let keep = if self.flow.read[i][3] < 0.02
                && self.dep_k.read[i][3] + self.sus_k.read[i][3] > 1e-4
            {
                0.0
            } else {
                1.0
            };
            self.sus_k.write[i] = quantize(self.sus_k.read[i].map(|v| v * keep));
            self.sus_s.write[i] = quantize(self.sus_s.read[i].map(|v| v * keep));
            self.sus_p.write[i] = quantize(self.sus_p.read[i].map(|v| v * keep));
        }
    }
    // src/engine/shaders.ts:434-450 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn dry_clear_dep(&mut self) {
        for i in 0..self.width * self.height {
            let keep = if self.flow.read[i][3] < 0.02
                && self.dep_k.read[i][3] + self.sus_k.read[i][3] > 1e-4
            {
                0.0
            } else {
                1.0
            };
            self.dep_k.write[i] = quantize(self.dep_k.read[i].map(|v| v * keep));
            self.dep_s.write[i] = quantize(self.dep_s.read[i].map(|v| v * keep));
        }
    }
}

// src/engine/shaders.ts:530-532 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn salt_hash(x: f32, seed: f32) -> f32 {
    fract((x * 127.1 + seed * 311.7).sin() * 43758.5453)
}
// GLSL fract built-in used in src/engine/shaders.ts:530-550,589-594 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn fract(x: f32) -> f32 {
    x - x.floor()
}
// src/engine/shaders.ts:589-594 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn hash12(p: [f32; 2]) -> f32 {
    let mut p3 = [
        fract(p[0] * 0.1031),
        fract(p[1] * 0.1031),
        fract(p[0] * 0.1031),
    ];
    let dot = p3[0] * (p3[1] + 33.33) + p3[1] * (p3[2] + 33.33) + p3[2] * (p3[0] + 33.33);
    for v in &mut p3 {
        *v += dot;
    }
    fract((p3[0] + p3[1]) * p3[2])
}
// GLSL normalize built-in used in src/engine/shaders.ts:613-614,651-652 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    v.map(|x| x / len)
}
// src/engine/shaders.ts:662,712 at a19bfa2a026856a98ceb400a8eec41f091f04a11, default RGBA8 framebuffer conversion.
fn srgb_byte(v: f32) -> u8 {
    let s = if v >= 0.0031308 {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    } else {
        v * 12.92
    };
    (s.clamp(0.0, 1.0) * 255.0).round() as u8
}

impl Simulation {
    // src/engine/simulation.ts:270-311 and src/engine/shaders.ts:455-515 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn splat(&mut self, stamps: &[Stamp], pigment: Option<&Pigment>) {
        let k = pigment.map_or([0.0; 3], |p| p.k);
        let s = pigment.map_or([0.0; 3], |p| p.s);
        let props = pigment.map_or([0.4, 0.0, 0.3], |p| [p.density, p.granulation, p.staining]);
        for stamp in stamps.iter().take(4096) {
            let cx = stamp.x as f32;
            let cy = (self.height as f64 - stamp.y) as f32;
            let radius = stamp.radius as f32;
            let water = (stamp.water * 0.055) as f32;
            let amount = if pigment.is_some() {
                (stamp.pigment * 0.13) as f32
            } else {
                0.0
            };
            let dryness = stamp.dryness as f32;
            let (x0, x1, y0, y1) = self.stamp_bounds(cx, cy, radius);
            for y in y0..y1 {
                for x in x0..x1 {
                    let lx = (x as f32 + 0.5 - cx) / radius;
                    let ly = (y as f32 + 0.5 - cy) / radius;
                    let r = (lx * lx + ly * ly).sqrt();
                    if r > 1.0 {
                        continue;
                    }
                    let body = smoothstep(1.0, 0.62, r);
                    let mut water = water * body;
                    let mut pigment = amount * body;
                    if dryness > 0.0 {
                        let paper = sample(
                            &self.paper,
                            self.width,
                            self.height,
                            (x as f32 + 0.5) / self.width as f32,
                            (y as f32 + 0.5) / self.height as f32,
                        );
                        let level = 0.56 - 0.14 * (1.0 - dryness);
                        let tooth = smoothstep(level, level + 0.05, paper[0] + paper[1] * 0.3);
                        let contact = mix(1.0, tooth, dryness);
                        water *= contact;
                        pigment *= contact * (0.7 + 0.6 * paper[1]);
                    }
                    let i = y * self.width + x;
                    let outputs = [
                        [0.0, 0.0, water, (water * 60.0).min(1.0)],
                        [k[0] * pigment, k[1] * pigment, k[2] * pigment, pigment],
                        [s[0] * pigment, s[1] * pigment, s[2] * pigment, 0.0],
                        [
                            props[0] * pigment,
                            props[1] * pigment,
                            props[2] * pigment,
                            0.0,
                        ],
                    ];
                    for (target, out) in [
                        &mut self.flow.read,
                        &mut self.sus_k.read,
                        &mut self.sus_s.read,
                        &mut self.sus_p.read,
                    ]
                    .into_iter()
                    .zip(outputs)
                    {
                        target[i] = quantize(std::array::from_fn(|c| target[i][c] + out[c]));
                    }
                }
            }
        }
    }
    // src/engine/shaders.ts:455-471 at a19bfa2a026856a98ceb400a8eec41f091f04a11: rasterized quad footprint.
    fn stamp_bounds(&self, cx: f32, cy: f32, r: f32) -> (usize, usize, usize, usize) {
        let x0 = (cx - r - 0.5).ceil().clamp(0.0, self.width as f32) as usize;
        let x1 = (cx + r - 0.5).ceil().clamp(0.0, self.width as f32) as usize;
        let y0 = (cy - r - 0.5).ceil().clamp(0.0, self.height as f32) as usize;
        let y1 = (cy + r - 0.5).ceil().clamp(0.0, self.height as f32) as usize;
        (x0, x1, y0, y1)
    }
    // src/engine/simulation.ts:316-354 at a19bfa2a026856a98ceb400a8eec41f091f04a11, JS RNG and stamp upload boundary.
    pub fn add_salt(&mut self, x: f64, y: f64, radius: f64, seed: u32) {
        let mut state = seed;
        let mut rand = || {
            state = state.wrapping_add(0x6d2b79f5);
            let mut t = state;
            t = (t ^ (t >> 15)).wrapping_mul(t | 1);
            t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
            (t ^ (t >> 14)) as f64 / 4294967296.0
        };
        let grains = 18 + (rand() * 12.0).floor() as usize;
        for i in 0..grains {
            let angle = rand() * std::f64::consts::TAU;
            let dist = rand().sqrt() * radius;
            let cx = (x + angle.cos() * dist) as f32;
            let cy = (self.height as f64 - (y + angle.sin() * dist)) as f32;
            let r = (4.0 + rand() * 4.5) as f32;
            let amount = (0.35 + rand() * 0.5) as f32;
            self.salt(cx, cy, r, amount, i as f32);
        }
    }
    // src/engine/shaders.ts:520-551 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn salt(&mut self, cx: f32, cy: f32, radius: f32, amount: f32, seed: f32) {
        let (x0, x1, y0, y1) = self.stamp_bounds(cx, cy, radius);
        let angle = seed * 2.39996;
        let wick = [angle.cos(), angle.sin()];
        for y in y0..y1 {
            for x in x0..x1 {
                let local = [
                    (x as f32 + 0.5 - cx) / radius,
                    (y as f32 + 0.5 - cy) / radius,
                ];
                let dot = local[0] * wick[0] + local[1] * wick[1];
                let stretch = 0.3 + 0.35 * salt_hash(7.0, seed);
                let lp = [
                    local[0] - wick[0] * dot * stretch,
                    local[1] - wick[1] * dot * stretch,
                ];
                let r = (lp[0] * lp[0] + lp[1] * lp[1]).sqrt();
                if r > 1.4 {
                    continue;
                }
                let bins = 5.0 + seed.rem_euclid(3.0).floor();
                let b = (lp[1].atan2(lp[0]) / 6.2831853 + 0.5) * bins;
                let i0 = b.floor().rem_euclid(bins);
                let i1 = (i0 + 1.0).rem_euclid(bins);
                let f = fract(b);
                let f = f * f * (3.0 - 2.0 * f);
                let reach = mix(salt_hash(i0, seed), salt_hash(i1, seed), f) * 0.45 + 0.35;
                let grain = smoothstep(1.0, 0.15, r / reach.max(0.2));
                let i = y * self.width + x;
                let old = self.cap.read[i];
                self.cap.read[i] =
                    quantize([old[0], old[1] + amount * grain * 5.0, old[2], old[3]]);
            }
        }
    }
    // src/engine/simulation.ts:359-388 and src/engine/shaders.ts:557-569 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn add_scrub(&mut self, stamps: &[Stamp]) {
        for stamp in stamps.iter().take(4096) {
            self.lift_pass(
                stamp.x as f32,
                (self.height as f64 - stamp.y) as f32,
                stamp.radius as f32,
                0.35,
            );
        }
    }
    // src/engine/shaders.ts:557-569 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn lift_pass(&mut self, cx: f32, cy: f32, radius: f32, amount: f32) {
        let (x0, x1, y0, y1) = self.stamp_bounds(cx, cy, radius);
        for y in y0..y1 {
            for x in x0..x1 {
                let lx = (x as f32 + 0.5 - cx) / radius;
                let ly = (y as f32 + 0.5 - cy) / radius;
                let r = (lx * lx + ly * ly).sqrt();
                if r > 1.0 {
                    continue;
                }
                let body = smoothstep(1.0, 0.45, r);
                let i = y * self.width + x;
                let old = self.cap.read[i];
                self.cap.read[i] = quantize([old[0], old[1], old[2] + amount * body, old[3]]);
            }
        }
    }
    // src/engine/simulation.ts:254-264 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn reset_state(&mut self) {
        self.reset();
    }
    // src/engine/simulation.ts:621-632 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    fn state_targets(&self) -> Vec<Vec<Pixel>> {
        vec![
            self.flow.read.clone(),
            self.sus_k.read.clone(),
            self.sus_s.read.clone(),
            self.sus_p.read.clone(),
            self.dep_k.read.clone(),
            self.dep_s.read.clone(),
            self.cap.read.clone(),
            self.dried.read.clone(),
        ]
    }
    // src/engine/simulation.ts:637-653 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn snapshot(&mut self) {
        if self.undo_ring.len() < 3 {
            self.undo_ring.resize_with(3, Vec::new);
        }
        let slot = self.undo_cursor % self.undo_ring.len();
        self.undo_ring[slot] = self.state_targets();
        self.undo_live.push(slot);
        if self.undo_live.len() > self.undo_ring.len() {
            self.undo_live.remove(0);
        }
        self.undo_cursor += 1;
    }
    // src/engine/simulation.ts:655-664 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn undo(&mut self) -> bool {
        let Some(slot) = self.undo_live.pop() else {
            return false;
        };
        for (target, saved) in [
            &mut self.flow.read,
            &mut self.sus_k.read,
            &mut self.sus_s.read,
            &mut self.sus_p.read,
            &mut self.dep_k.read,
            &mut self.dep_s.read,
            &mut self.cap.read,
            &mut self.dried.read,
        ]
        .into_iter()
        .zip(&self.undo_ring[slot])
        {
            *target = saved.clone();
        }
        self.flux.read.fill([0.0; 4]);
        self.flux.write.fill([0.0; 4]);
        self.fast_dry_steps = 0;
        true
    }
    // src/engine/simulation.ts:666-677 at a19bfa2a026856a98ceb400a8eec41f091f04a11; clear retains the undo ring.
    pub fn clear_sheet(&mut self) {
        self.snapshot();
        let live = std::mem::take(&mut self.undo_live);
        self.reset();
        self.undo_live = live;
    }
    // src/engine/simulation.ts:682-704 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn probe(&self, x: f64, y: f64, radius: f64) -> Probe {
        let r = radius.round().max(1.0) as isize;
        let cx = (x + 0.5).floor() as isize;
        let cy = (self.height as f64 - y + 0.5).floor() as isize;
        let x0 = (cx - r).max(0);
        let y0 = (cy - r).max(0);
        let x1 = (cx + r + 1).min(self.width as isize);
        let y1 = (cy + r + 1).min(self.height as isize);
        let mut out = Probe {
            dried: 0.0,
            workable: 0.0,
            wet: 0.0,
        };
        if x1 <= x0 || y1 <= y0 {
            return out;
        }
        for yy in y0..y1 {
            for xx in x0..x1 {
                let i = yy as usize * self.width + xx as usize;
                out.dried += self.dried.read[i][3] as f64;
                out.workable += self.dep_k.read[i][3] as f64 + self.sus_k.read[i][3] as f64;
                out.wet += self.flow.read[i][3] as f64;
            }
        }
        let n = ((x1 - x0) * (y1 - y0)) as f64;
        out.dried /= n;
        out.workable /= n;
        out.wet /= n;
        out
    }
    // src/engine/simulation.ts:708-715 at a19bfa2a026856a98ceb400a8eec41f091f04a11; readFloat preserves GL row orientation.
    pub fn serialize_dried(&self) -> Vec<f32> {
        self.dried.read.iter().flatten().copied().collect()
    }
    // src/engine/simulation.ts:731-739 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn restore_dried(&mut self, data: &[f32]) {
        if data.len() != self.width * self.height * 4 {
            return;
        }
        self.reset();
        for (pixel, input) in self.dried.read.iter_mut().zip(data.chunks_exact(4)) {
            *pixel = quantize([input[0], input[1], input[2], input[3]]);
        }
        self.dried.write.clone_from(&self.dried.read);
    }
    // src/engine/simulation.ts:720-728 at a19bfa2a026856a98ceb400a8eec41f091f04a11, CPU readback of a selected region.
    pub fn read_float(&self, target: &[Pixel], x: usize, y: usize, w: usize, h: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(w * h * 4);
        for yy in y..y + h {
            for xx in x..x + w {
                out.extend_from_slice(&target[yy * self.width + xx]);
            }
        }
        out
    }
    // src/engine/simulation.ts:579-617 at a19bfa2a026856a98ceb400a8eec41f091f04a11; top-down RGBA bytes for host display.
    pub fn render_rgba(&self, backlight: f32, xray: u8) -> Vec<u8> {
        self.render_rgba_sized(self.width, self.height, backlight, xray)
    }
    // src/engine/simulation.ts:579-617 at a19bfa2a026856a98ceb400a8eec41f091f04a11: canvas viewport independent of state size.
    pub fn render_rgba_sized(
        &self,
        width: usize,
        height: usize,
        backlight: f32,
        xray: u8,
    ) -> Vec<u8> {
        let mut out = vec![0; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let uv = [
                    (x as f32 + 0.5) / width as f32,
                    (y as f32 + 0.5) / height as f32,
                ];
                let color = if xray == 0 {
                    self.render_pass(uv, [x as f32 + 0.5, y as f32 + 0.5], backlight)
                } else {
                    self.xray_pass(uv, xray)
                };
                let i = ((height - 1 - y) * width + x) * 4;
                for c in 0..3 {
                    out[i + c] = srgb_byte(color[c]);
                }
                out[i + 3] = 255;
            }
        }
        out
    }
    // src/engine/shaders.ts:575-665 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn render_pass(&self, uv: [f32; 2], frag_coord: [f32; 2], backlight: f32) -> [f32; 3] {
        let at = |data: &[Pixel], u: f32, v: f32| sample(data, self.width, self.height, u, v);
        let f = at(&self.flow.read, uv[0], uv[1]);
        let sk = at(&self.sus_k.read, uv[0], uv[1]);
        let ss = at(&self.sus_s.read, uv[0], uv[1]);
        let dk = at(&self.dep_k.read, uv[0], uv[1]);
        let ds = at(&self.dep_s.read, uv[0], uv[1]);
        let dried = at(&self.dried.read, uv[0], uv[1]);
        let paper = at(&self.paper, uv[0], uv[1]);
        let tx = 1.0 / self.width as f32;
        let ty = 1.0 / self.height as f32;
        let hl = at(&self.paper, uv[0] - tx, uv[1])[0];
        let hr = at(&self.paper, uv[0] + tx, uv[1])[0];
        let hd = at(&self.paper, uv[0], uv[1] - ty)[0];
        let hu = at(&self.paper, uv[0], uv[1] + ty)[0];
        let normal = normalize([(hl - hr) * 2.2, (hd - hu) * 2.2, 1.0]);
        let light = normalize([-0.45, 0.55, 0.9]);
        let dot = normal[0] * light[0] + normal[1] * light[1] + normal[2] * light[2];
        let relief = 0.85 + 0.15 * dot;
        let micro = 0.97 + 0.03 * paper[1] + self.params.zoom_grain * (hash12(frag_coord) - 0.5);
        let illum = 1.035 - 0.085 * ((uv[0] - 0.3).powi(2) + (uv[1] - 0.78).powi(2)).sqrt();
        let wet = f[3].clamp(0.0, 1.0);
        let damp = (f[2] * 8.0).clamp(0.0, 1.0);
        let darken = 1.0 - 0.075 * wet - 0.05 * damp;
        let paint = (dried[3] * 1.6).clamp(0.0, 1.0);
        let tooth = 1.0 + ((paper[1] - 0.5) * 0.07 + (0.5 - paper[0]) * 0.22) * paint;
        let base = std::array::from_fn(|c| dried[c] * tooth * relief * micro * illum * darken);
        let gran = at(&self.sus_p.read, uv[0], uv[1])[1] / sk[3].max(1e-5);
        let dep_gran = gran.clamp(0.0, 1.0) * 0.95 + 0.06;
        let gran_mod = 1.0 + dep_gran * (paper[1] - 0.5) * 0.8 * (dk[3] * 3.0).min(1.0);
        let k = std::array::from_fn(|c| (sk[c] + dk[c]) * gran_mod);
        let s = std::array::from_fn(|c| (ss[c] + ds[c]) * gran_mod);
        let (r, t) = km_layer(k, s, 1.0);
        let mut color = km_composite(r, t, base);
        let half = normalize([light[0], light[1], light[2] + 1.0]);
        let dot = normal[0] * half[0] + normal[1] * half[1] + normal[2] * half[2];
        let sheen = smoothstep(0.05, 0.5, f[2]) * 0.05 * dot.max(0.0).powf(3.0);
        for c in 0..3 {
            color[c] += sheen;
        }
        if backlight > 0.0 {
            for c in 0..3 {
                let through = t[c] * (-dried[3] * [1.15, 1.0, 0.9][c]).exp();
                let lit = [1.0, 0.985, 0.94][c] * through * (0.55 + 0.45 * paper[0]);
                color[c] = mix(color[c], lit, backlight);
            }
        }
        color
    }
    // src/engine/shaders.ts:679-715 at a19bfa2a026856a98ceb400a8eec41f091f04a11, linear output before RGBA8 conversion.
    pub fn xray_pass(&self, uv: [f32; 2], field: u8) -> [f32; 3] {
        let data = match field {
            1 | 2 => &self.flow.read,
            3 => &self.sus_k.read,
            4 => &self.dep_k.read,
            5 => &self.cap.read,
            _ => &self.dried.read,
        };
        let pixel = sample(data, self.width, self.height, uv[0], uv[1]);
        let value = match field {
            1 => pixel[2],
            2 | 3 | 4 | 6 => pixel[3],
            5 => pixel[0],
            _ => 0.0,
        };
        let scale = match field {
            1 => 0.12,
            2 => 0.6,
            3 => 0.15,
            4 => 0.06,
            5 => 0.03,
            _ => 1.0,
        };
        let t = 1.0 - (-value.max(0.0) / scale).exp();
        let paper = [0.957, 0.898, 0.776];
        let mid = [0.071, 0.254, 0.291];
        let ink = [0.0, 0.050, 0.065];
        let mut color = std::array::from_fn(|c| {
            if t < 0.5 {
                mix(paper[c], mid[c], t * 2.0)
            } else {
                mix(mid[c], ink[c], (t - 0.5) * 2.0)
            }
        });
        if field == 5 {
            let salt = (pixel[1] * 0.6).clamp(0.0, 1.0);
            for c in 0..3 {
                color[c] = mix(color[c], [0.791, 0.045, 0.0][c], salt);
            }
        }
        color
    }
}
