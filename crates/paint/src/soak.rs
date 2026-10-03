//! Soak-stain: thinned paint poured onto raw, unprimed canvas soaks into the
//! cloth instead of lying on it as a film (Frankenthaler's stains from 1952,
//! Morris Louis's veils). The physics and its sources are in
//! notes/research/soak_stain.md; numbers marked "estimate" there are
//! estimates here too.
//!
//! **The cloth** holds liquid in its pores (`Fabric::cap_um`, µm of liquid
//! per area when saturated). Its threads are capillaries: liquid wicks
//! faster along the warp than across it, and faster along some threads than
//! others, so a stain is a little elongated along the warp and its edge
//! feathers along the threads.
//!
//! **A pour** (`Canvas::pour`) lands where its mask says and wicks outward.
//! The liquid reaches the cloth in order of capillary travel (a
//! shortest-path front over the cloth's permeability, Washburn's law giving
//! the times) and fills each pore volume it reaches until the poured volume
//! is used up; where more was poured, it pushes further. Cloth that is still
//! wet lets new liquid through easily (it mixes and runs on); cloth that
//! holds oil takes little and slows it.
//!
//! **Pigment** rides in the liquid and the fibres filter it as it passes
//! (deep-bed filtration: the denser near the pour, the paler far from it;
//! fine pigments travel further than coarse ones). What is still suspended
//! when the turpentine evaporates is drawn toward the drying edge and left
//! there (a ring: the more turpentine, the stronger).
//!
//! **Turpentine** darkens the cloth while it is there (liquid in the air
//! gaps between the fibres: "darker when wet") and evaporates over the next
//! hour, the edges first. **Oil** beyond what the fibres and the pigment
//! hold creeps on past the colour for a day or two until it gels, leaving a
//! darker, yellowing halo; paint thinned so far that too little oil is left
//! to wet the pigment dries lean, pale and matte.
//!
//! **Appearance**: the cloth is one Kubelka–Munk layer over a backing. Its
//! scattering is lowered where liquid fills the pores, and the deposited
//! pigments' absorption and scattering are added to it, so a stain is
//! colour *in* the cloth: the weave shows through it.

use crate::canvas::Canvas;
use crate::color::{Rgb, hex};
use crate::mask::Mask;
use crate::pigment::{Pigment, layer1};
use crate::surface::{COAT_UM, box_blur, vnoise};
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Scattering of the dry cloth through its thickness (estimate: raw cotton
/// duck is light and lets a little light through).
const S_CLOTH: f32 = 6.0;
/// Reflectance of what is behind the canvas (stretcher, wall).
const BACKING: f32 = 0.35;
/// How much the weave's relief changes the cloth's scattering (thread tops
/// light, the gaps between threads darker).
const WEAVE_AMP: f32 = 0.16;
/// Liquid in the pores lowers the cloth's scattering by up to this share
/// (refractive index of oil or turpentine near the fibres'), reaching most
/// of it at this fill of the pores.
const WET_DARK: f32 = 0.62;
const WET_FILL: f32 = 0.1;
/// The share of the pore volume the fibres themselves hold of oil.
const OIL_RETAIN: f32 = 0.03;
/// Oil a pigment holds, by volume of pigment.
const PIG_OIL: f32 = 0.8;
/// Unbound pigment (lean: too little oil to wet it) scatters this much more.
const LEAN_SCATTER: f32 = 0.7;
/// Oil fill of the cloth in a halo.
const HALO_FILL: f32 = 0.07;
/// How far past the colour oil creeps before it gels (estimate), mm.
const HALO_MM: f32 = 12.0;
/// How long it takes to get there (estimate), days.
const HALO_DAYS: f32 = 2.0;
/// Fine pigment carried into the halo with the oil (share of the edge's).
const HALO_TINT: f32 = 0.05;
/// Linseed oil yellowing in the cloth: absorption per unit oil fill, and
/// its time to develop (days).
const YELLOW: Rgb = [0.12, 0.3, 0.95];
const YELLOW_DAYS: f32 = 90.0;
/// Turpentine leaves a saturated cloth in about this long (estimate), min.
const EVAP_MIN: f32 = 45.0;
/// Capillary pore radius (m), surface tension of turpentine and oil (N/m),
/// viscosities (Pa·s) and the packing at which suspended pigment jams.
const R_PORE: f32 = 1.0e-5;
const GAMMA: f32 = 0.028;
const MU_TURPS: f32 = 1.4e-3;
const MU_OIL: f32 = 4.0e-2;
const PHI_MAX: f32 = 0.64;
/// Deep-bed filtration: the share of suspended pigment the fibres catch per
/// mm of travel, for the coarsest pigment, and the floor for the finest.
const FILTER_MM: f32 = 0.012;
const FILTER_MIN: f32 = 0.0012;
/// The ring: share of the still-suspended pigment drawn to the edge as the
/// turpentine evaporates (base, and more with turpentine), from a band of
/// this width (mm).
const RING_BASE: f32 = 0.1;
const RING_SOLV: f32 = 0.45;
const RING_MM: f32 = 10.0;
/// How unevenly the cloth wicks: lobes of looser and tighter weave, and
/// streaks along the threads (more in a thin, turpentine-rich liquid).
const PATCH: f32 = 1.0;
const FEATHER_BASE: f32 = 0.25;
const FEATHER_SOLV: f32 = 0.5;

/// The cloth a raw canvas is woven from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fabric {
    pub name: &'static str,
    /// The raw cloth's colour, linear RGB.
    pub color: Rgb,
    /// Pore volume: µm of liquid the cloth holds per area when saturated.
    pub cap_um: f32,
    /// How much faster liquid wicks along the warp (down the canvas) than
    /// along the weft.
    pub warp_bias: f32,
}

impl Fabric {
    /// Unbleached cotton duck, as Frankenthaler and Louis used: creamy,
    /// thick and absorbent (estimate: ~0.5 mm thick, porosity ~0.6).
    pub fn cotton_duck() -> Fabric {
        Fabric { name: "cotton duck", color: hex("#e3d9c4"), cap_um: 300.0, warp_bias: 1.3 }
    }
    /// Raw linen: browner, denser and thinner, it holds less.
    pub fn linen() -> Fabric {
        Fabric { name: "linen", color: hex("#a8966f"), cap_um: 220.0, warp_bias: 1.2 }
    }
    pub fn named(n: &str) -> Option<Fabric> {
        match n {
            "cotton duck" | "cotton" | "duck" => Some(Fabric::cotton_duck()),
            "linen" => Some(Fabric::linen()),
            _ => None,
        }
    }
    pub fn names() -> &'static str {
        "\"cotton duck\" or \"linen\""
    }
}

/// What is soaked into a raw canvas, per pixel of its window.
#[derive(Clone)]
pub struct Soak {
    pub(crate) fabric: Fabric,
    /// The clock (minutes) when the canvas was set up; times below count
    /// from it.
    pub(crate) t0: f64,
    pub(crate) seed: u64,
    /// The dry cloth's absorption (per its thickness), so that it looks its
    /// colour.
    pub(crate) kf: Rgb,
    /// Scattering factor per pixel from the weave's relief.
    pub(crate) weave: Vec<f32>,
    /// Pore volume, µm of liquid.
    pub(crate) cap: Vec<f32>,
    /// Deposited pigment: absorption and scattering (in the cloth layer's
    /// units: coats of paint).
    pub(crate) kp: Vec<Rgb>,
    pub(crate) sp: Vec<Rgb>,
    /// Pigment volume, µm.
    pub(crate) pig: Vec<f32>,
    /// Oil in place, µm; oil still creeping here and when it arrives; when
    /// oil first came (for yellowing; infinite: never).
    pub(crate) oil: Vec<f32>,
    pub(crate) oil_pend: Vec<f32>,
    pub(crate) oil_t: Vec<f32>,
    pub(crate) oil_since: Vec<f32>,
    /// Turpentine, µm at `solv_t0`, gone at `solv_t1` (evaporating
    /// steadily between).
    pub(crate) solv: Vec<f32>,
    pub(crate) solv_t0: Vec<f32>,
    pub(crate) solv_t1: Vec<f32>,
    /// Buffer box where turpentine or oil is still moving.
    pub(crate) active: Option<(usize, usize, usize, usize)>,
    /// Buffer box of everything ever soaked.
    pub(crate) stained: Option<(usize, usize, usize, usize)>,
    pub(crate) pours: u64,
}

/// One pour: a pile thinned with turpentine.
#[derive(Clone, Copy, Debug)]
pub struct Pour {
    /// The pile's absorption and scattering per coat, as it comes (no
    /// turpentine).
    pub paint: Pigment,
    /// Pigment and oil, as shares of the pile's volume (they sum to 1).
    pub pigment: f32,
    pub oil: f32,
    /// How far the pigment's particles travel in the cloth: 0 coarse
    /// (smalt) .. 1 very fine (lakes, Prussian blue).
    pub mobility: f32,
    /// Parts of turpentine to one part of the pile.
    pub thinner: f32,
    /// Millilitres poured.
    pub ml: f32,
    /// The canvas tilted: direction it runs downhill (radians, canvas
    /// angles) and how steeply (0 flat .. 1).
    pub tilt: Option<(f32, f32)>,
    pub seed: u64,
}

/// What a pour did.
#[derive(Clone, Copy, Debug, Default)]
pub struct Poured {
    /// Millilitres that soaked in; that ran off the box (no room left).
    pub soaked_ml: f32,
    pub lost_ml: f32,
    /// Area the liquid reached, cm².
    pub area_cm2: f32,
    /// How long it took to spread, seconds.
    pub spread_s: f32,
    /// How far oil will creep past the colour (mm, the widest), and
    /// millilitres of it.
    pub halo_mm: f32,
    pub halo_ml: f32,
}

/// Reflectance of the cloth layer with absorption `k` and scattering `s`
/// over the backing.
#[inline]
fn cloth(k: f32, s: f32) -> f32 {
    let (r, t) = layer1(k.max(0.0), s.max(1e-4), 1.0);
    r + t * t * BACKING / (1.0 - r * BACKING).max(1e-6)
}

/// The absorption that makes the dry cloth look `target` (one channel).
fn solve_k(target: f32) -> f32 {
    if target >= cloth(0.0, S_CLOTH) {
        return 0.0;
    }
    let (mut lo, mut hi) = (0.0f32, 60.0f32);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if cloth(mid, S_CLOTH) > target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

fn grow(b: Option<(usize, usize, usize, usize)>, c: (usize, usize, usize, usize)) -> Option<(usize, usize, usize, usize)> {
    Some(match b {
        None => c,
        Some(b) => (b.0.min(c.0), b.1.min(c.1), b.2.max(c.2), b.3.max(c.3)),
    })
}

impl Soak {
    /// Turpentine at pixel `i` at time `now` (minutes after `t0`), µm.
    #[inline]
    pub(crate) fn solv_at(&self, i: usize, now: f32) -> f32 {
        let s = self.solv[i];
        if s <= 0.0 {
            return 0.0;
        }
        let (a, b) = (self.solv_t0[i], self.solv_t1[i]);
        if now <= a {
            s
        } else if now >= b {
            0.0
        } else {
            s * (b - now) / (b - a).max(1e-6)
        }
    }

    /// Oil at pixel `i` at time `now`, µm, and since when.
    #[inline]
    fn oil_at(&self, i: usize, now: f32) -> (f32, f32) {
        let (mut o, mut since) = (self.oil[i], self.oil_since[i]);
        if self.oil_pend[i] > 0.0 && now >= self.oil_t[i] {
            o += self.oil_pend[i];
            since = since.min(self.oil_t[i]);
        }
        (o, since)
    }

    /// The colour of pixel `i` at time `now`.
    pub(crate) fn shade(&self, i: usize, now: f32) -> Rgb {
        let cap = self.cap[i].max(1.0);
        let solv = self.solv_at(i, now);
        let (oil, since) = self.oil_at(i, now);
        let fill = ((oil + solv) / cap).clamp(0.0, 1.0);
        let dark = WET_DARK * (1.0 - (-fill / WET_FILL).exp());
        let sf = S_CLOTH * self.weave[i] * (1.0 - dark);
        let pig = self.pig[i];
        let bind = if pig > 1e-4 { (((oil - OIL_RETAIN * cap).max(0.0) + solv) / (PIG_OIL * pig)).clamp(0.0, 1.0) } else { 1.0 };
        let lean = 1.0 + LEAN_SCATTER * (1.0 - bind);
        let yel = if oil > 0.0 {
            let age = (now - since).max(0.0) / 1440.0;
            (oil / cap).min(1.0) * (0.4 + 0.6 * (1.0 - (-age / YELLOW_DAYS).exp()))
        } else {
            0.0
        };
        let (kp, sp) = (self.kp[i], self.sp[i]);
        std::array::from_fn(|c| cloth(self.kf[c] + kp[c] + yel * YELLOW[c], sf + sp[c] * lean))
    }

    /// Words for what is in the cloth at pixel `i` (for `soaked`).
    pub fn describe(&self, i: usize, now: f32) -> String {
        let cap = self.cap[i].max(1.0);
        let solv = self.solv_at(i, now);
        let (oil, _) = self.oil_at(i, now);
        let creeping = self.oil_pend[i] > 0.0 && now < self.oil_t[i];
        let mut s = if solv > 0.02 * cap {
            format!("wet with turpentine ({:.0}% of the pores)", 100.0 * solv / cap)
        } else if self.pig[i] > 0.0 || oil > 0.0 {
            "dry".to_string()
        } else {
            "raw".to_string()
        };
        if self.pig[i] > 0.0 {
            s.push_str(&format!(", pigment {:.1} µm", self.pig[i]));
        }
        if oil > 0.0 {
            s.push_str(&format!(", oil {:.0}% of the pores", 100.0 * oil / cap));
        }
        if creeping {
            s.push_str(&format!(", oil creeping in (in {:.0} min)", self.oil_t[i] - now));
        }
        s
    }
}

const NEIGH: [(i32, i32, f32); 8] = [(1, 0, 1.0), (-1, 0, 1.0), (0, 1, 1.0), (0, -1, 1.0), (1, 1, std::f32::consts::SQRT_2), (-1, 1, std::f32::consts::SQRT_2), (1, -1, std::f32::consts::SQRT_2), (-1, -1, std::f32::consts::SQRT_2)];

/// The fronts' stencil: 16 neighbours (with the knight's moves), so a
/// front in even cloth is round, not an octagon.
const NEIGH16: [(i32, i32, f32); 16] = {
    const R5: f32 = 2.236_068;
    [
        (1, 0, 1.0), (-1, 0, 1.0), (0, 1, 1.0), (0, -1, 1.0),
        (1, 1, std::f32::consts::SQRT_2), (-1, 1, std::f32::consts::SQRT_2), (1, -1, std::f32::consts::SQRT_2), (-1, -1, std::f32::consts::SQRT_2),
        (2, 1, R5), (-2, 1, R5), (2, -1, R5), (-2, -1, R5), (1, 2, R5), (-1, 2, R5), (1, -2, R5), (-1, -2, R5),
    ]
};

/// A Dijkstra heap entry: key (non-negative f32, as bits, which order as
/// the floats do) and local index; ties go to the lower index.
type Heap = BinaryHeap<Reverse<(u32, u32)>>;

#[inline]
fn kbits(k: f32) -> u32 {
    k.max(0.0).to_bits()
}

impl Canvas {
    /// Leave the canvas raw: no ground, the bare `fabric`, which soaks up
    /// what is poured on it (`pour`). Call it on a canvas with no ground.
    pub fn raw_canvas(&mut self, fabric: Fabric, seed: u64) {
        let (w, h) = (self.f.w, self.f.h);
        let n = w * h;
        // the weave's relief: thread tops scatter more than the gaps
        let (mut m, mut v) = (0.0f64, 0.0f64);
        for &z in &self.height {
            m += z as f64;
        }
        m /= n.max(1) as f64;
        for &z in &self.height {
            v += (z as f64 - m).powi(2);
        }
        let sd = (v / n.max(1) as f64).sqrt().max(1e-6) as f32;
        let m = m as f32;
        let weave: Vec<f32> = self.height.par_iter().map(|&z| 1.0 + WEAVE_AMP * ((z - m) / (2.0 * sd)).clamp(-1.0, 1.0)).collect();
        let cap: Vec<f32> = weave.par_iter().map(|&q| fabric.cap_um * (1.0 + 1.5 * (q - 1.0))).collect();
        let kf = std::array::from_fn(|c| solve_k(fabric.color[c]));
        self.soak = Some(Box::new(Soak {
            fabric,
            t0: self.clock(),
            seed,
            kf,
            weave,
            cap,
            kp: vec![[0.0; 3]; n],
            sp: vec![[0.0; 3]; n],
            pig: vec![0.0; n],
            oil: vec![0.0; n],
            oil_pend: vec![0.0; n],
            oil_t: vec![0.0; n],
            oil_since: vec![f32::INFINITY; n],
            solv: vec![0.0; n],
            solv_t0: vec![0.0; n],
            solv_t1: vec![0.0; n],
            active: None,
            stained: None,
            pours: 0,
        }));
        self.soak_render((0, 0, w, h));
    }

    /// Is this a raw canvas (`raw_canvas`)?
    pub fn is_raw(&self) -> bool {
        self.soak.is_some()
    }

    /// The raw fabric, if this is a raw canvas.
    pub fn fabric(&self) -> Option<Fabric> {
        self.soak.as_ref().map(|s| s.fabric)
    }

    /// Words for what is soaked into the cloth at (`x`, `y`) (units).
    pub fn soaked_at(&self, x: f32, y: f32) -> Option<String> {
        let s = self.soak.as_ref()?;
        let f = self.f;
        if !f.holds(x, y) {
            return None;
        }
        let (bx, by) = ((x * f.scale).floor() as usize - f.x0, (y * f.scale).floor() as usize - f.y0);
        let i = by * f.w + bx;
        let now = (self.clock() - s.t0) as f32;
        let mut d = s.describe(i, now);
        if self.film[i] > 1e-3 {
            d.push_str(", under a paint film");
        }
        Some(d)
    }

    /// Recolor the soaked cloth in buffer box `b` for the time now (only
    /// where no paint or ground film lies on it).
    pub(crate) fn soak_render(&mut self, b: (usize, usize, usize, usize)) {
        let Some(s) = self.soak.as_ref() else { return };
        let now = (self.wet.clock.now - s.t0) as f32;
        let w = self.f.w;
        let (x0, y0, x1, y1) = (b.0, b.1, b.2.min(w), b.3.min(self.f.h));
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let film = &self.film;
        let drawing = self.drawing.as_deref();
        self.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let i = (y0 + j) * w + x;
                if film[i] < 1e-3 {
                    // charcoal on the cloth stays on top of what soaks in
                    let c = s.shade(i, now);
                    row[x] = match drawing {
                        Some(d) => d.over(i, c),
                        None => c,
                    };
                }
            }
        });
    }

    /// Time passed (`dt` minutes, the clock already moved): oil that has
    /// crept in lands, turpentine that has gone is cleared, and the cloth is
    /// recolored where anything changed (all of it after a long wait: the
    /// oil yellows).
    pub(crate) fn soak_tick(&mut self, dt: f32) {
        let w = self.f.w;
        let Some(s) = self.soak.as_mut() else { return };
        let now = (self.wet.clock.now - s.t0) as f32;
        let mut redraw = s.active;
        if dt >= 600.0 {
            if let Some(b) = s.stained {
                redraw = grow(redraw, b);
            }
        }
        if let Some((x0, y0, x1, y1)) = s.active {
            let mut still = false;
            for y in y0..y1 {
                for x in x0..x1 {
                    let i = y * w + x;
                    if s.oil_pend[i] > 0.0 {
                        if now >= s.oil_t[i] {
                            s.oil[i] += s.oil_pend[i];
                            s.oil_since[i] = s.oil_since[i].min(s.oil_t[i]);
                            s.oil_pend[i] = 0.0;
                        } else {
                            still = true;
                        }
                    }
                    if s.solv[i] > 0.0 {
                        if now >= s.solv_t1[i] {
                            s.solv[i] = 0.0;
                        } else {
                            still = true;
                        }
                    }
                }
            }
            if !still {
                s.active = None;
            }
        }
        if let Some(b) = redraw {
            self.soak_render(b);
        }
    }

    /// Pour paint thinned with turpentine onto the raw canvas where `m`
    /// says (its coverage: how much lands where): it soaks in and spreads
    /// (see the module docs). The hand time is the pouring and the minute or
    /// two the painter watches it spread.
    pub fn pour(&mut self, m: &Mask, p: &Pour) -> Result<Poured, String> {
        if self.soak.is_none() {
            return Err("the canvas has a ground: pouring soaks only into a raw canvas (canvas{raw=...})".into());
        }
        if !(p.ml > 0.0 && p.ml.is_finite()) {
            return Err("pour: ml must be positive".into());
        }
        let (w, h) = (self.f.w, self.f.h);
        let f = self.f;
        let dx = self.px_mm();
        let area = dx * dx;
        let s = self.soak.as_ref().unwrap();
        let now = (self.wet.clock.now - s.t0) as f32;
        // where it lands
        let mut land: Vec<(usize, f32)> = Vec::new();
        let (mut bx0, mut by0, mut bx1, mut by1) = (w, h, 0, 0);
        for i in 0..w * h {
            let v = m.data[f.whole_index(i)];
            if v >= 0.02 {
                land.push((i, v));
                let (x, y) = (i % w, i / w);
                bx0 = bx0.min(x);
                by0 = by0.min(y);
                bx1 = bx1.max(x);
                by1 = by1.max(y);
            }
        }
        if land.is_empty() {
            return Err("pour: the mask is empty (nothing lands)".into());
        }
        let msum: f32 = land.iter().map(|l| l.1).sum();
        let vol = p.ml * 1000.0; // mm³
        let t = p.thinner.max(0.0);
        let (s0, oil0, pig0) = (t / (1.0 + t), p.oil / (1.0 + t), p.pigment / (1.0 + t));
        // how fast it wicks: Washburn, h² = 2Kt with K = rγ/(4μ); the
        // liquid's viscosity by log-mixing turpentine and oil, raised by the
        // pigment in suspension (Krieger–Dougherty)
        let liq = (s0 + oil0).max(1e-6);
        let sf = s0 / liq;
        let mu_l = (sf * MU_TURPS.ln() + (1.0 - sf) * MU_OIL.ln()).exp();
        let phi = (pig0 / (pig0 + liq)).min(0.6);
        let mu = mu_l * (1.0 - phi / PHI_MAX).max(0.05).powf(-2.5 * PHI_MAX);
        let kw = R_PORE * GAMMA / (4.0 * mu) * 1.0e6; // mm²/s
        // the box the liquid can reach
        let cap_mm = s.fabric.cap_um * 1.0e-3;
        let reach = (vol / (cap_mm * std::f32::consts::PI)).sqrt();
        let pad = ((2.2 * reach + 2.5 * HALO_MM) / dx).ceil() as usize + 4;
        let (x0, y0, x1, y1) = (bx0.saturating_sub(pad), by0.saturating_sub(pad), (bx1 + 1 + pad).min(w), (by1 + 1 + pad).min(h));
        let (rw, rh) = (x1 - x0, y1 - y0);
        let nl = rw * rh;
        let gi = |li: usize| (y0 + li / rw) * w + x0 + li % rw;
        // room in the pores (mm³) and how readily the cloth passes liquid
        // along x (the weft) and y (the warp)
        let sig = FEATHER_BASE + FEATHER_SOLV * s0;
        let cseed = s.seed;
        let pseed = p.seed;
        let film = &self.film;
        let cells: Vec<(f32, f32, f32)> = (0..nl)
            .into_par_iter()
            .map(|li| {
                let i = gi(li);
                if film[i] > 1e-3 {
                    return (0.0, 0.0, 0.0);
                }
                let cap = s.cap[i];
                let sn = s.solv_at(i, now);
                let oil = s.oil[i] + s.oil_pend[i];
                let free = (cap - oil - sn).max(0.0) * 1.0e-3 * area;
                let wet = (sn / cap).min(1.0);
                let oily = (oil / (0.3 * cap)).min(1.0);
                let mult = (1.0 + 1.5 * wet) * (1.0 - 0.85 * oily);
                let (xm, ym) = (((li % rw + x0 + f.x0) as f32 + 0.5) * dx, ((li / rw + y0 + f.y0) as f32 + 0.5) * dx);
                // lobes of looser and tighter weave (isotropic, 4-30 mm),
                // fine streaks along the threads (feathering, stronger in a
                // thin liquid), and the pour's own unevenness
                let patch = 0.55 * vnoise(xm / 30.0, ym / 30.0, cseed ^ 0x55) + 0.3 * vnoise(xm / 11.0, ym / 11.0, cseed ^ 0x57) + 0.15 * vnoise(xm / 4.0, ym / 4.0, cseed ^ 0x58) - 0.5;
                let nw = vnoise(xm / 0.9, ym / 9.0, cseed ^ 0x51) - 0.5;
                let nf = vnoise(xm / 9.0, ym / 0.9, cseed ^ 0x53) - 0.5;
                let own = 0.25 * (vnoise(xm / 7.0, ym / 7.0, pseed ^ 0x56) - 0.5);
                let ky = s.fabric.warp_bias * (PATCH * patch + sig * nw + own).exp() * mult;
                let kx = (PATCH * patch + sig * nf + own).exp() * mult;
                (free, kx, ky)
            })
            .collect();
        // more poured here: it pushes further (a head start from the
        // liquid's local excess over what the cloth there holds)
        let mut landv = vec![0.0f32; nl];
        for &(i, mv) in &land {
            let (x, y) = (i % w, i / w);
            landv[(y - y0) * rw + x - x0] = vol * mv / msum;
        }
        let rb = ((8.0 / dx).round() as usize).max(2);
        let bl = box_blur(&landv, rw, rh, rb);
        let hs: Vec<f32> = (0..nl).map(|li| if landv[li] > 0.0 { 0.5 * rb as f32 * dx * ((bl[li] / (cap_mm * area)).max(1.0).sqrt() - 1.0) } else { 0.0 }).collect();
        let off = hs.iter().cloned().fold(0.0f32, f32::max);
        // the front: Dijkstra over capillary travel, filling as it goes
        let mut key = vec![f32::INFINITY; nl];
        let mut parent = vec![u32::MAX; nl];
        let mut done = vec![false; nl];
        let mut take = vec![0.0f32; nl];
        let mut order: Vec<u32> = Vec::new();
        let mut heap = Heap::new();
        for li in 0..nl {
            if landv[li] > 0.0 && cells[li].1 > 0.0 {
                key[li] = off - hs[li];
                parent[li] = li as u32;
                heap.push(Reverse((kbits(key[li]), li as u32)));
            }
        }
        let speed = |a: usize, b: usize, ox: i32, oy: i32, tilt: Option<(f32, f32)>| -> f32 {
            let (ca, cb) = (cells[a], cells[b]);
            let (ax, ay) = (ox.unsigned_abs() as f32, oy.unsigned_abs() as f32);
            let v = (ax * (ca.1 + cb.1) + ay * (ca.2 + cb.2)) / (2.0 * (ax + ay));
            match tilt {
                None => v,
                Some((ang, g)) => {
                    let l = (ax * ax + ay * ay).sqrt();
                    let d = (ox as f32 * ang.cos() + oy as f32 * ang.sin()) / l;
                    v * (1.0 + 0.9 * g * d).max(0.15)
                }
            }
        };
        let mut left = vol;
        while let Some(Reverse((kb, li))) = heap.pop() {
            let li = li as usize;
            if done[li] || f32::from_bits(kb) > key[li] {
                continue;
            }
            done[li] = true;
            order.push(li as u32);
            let tk = cells[li].0.min(left);
            take[li] = tk;
            left -= tk;
            if left <= 1e-6 {
                break;
            }
            let (x, y) = ((li % rw) as i32, (li / rw) as i32);
            for &(ox, oy, len) in NEIGH16.iter() {
                let (nx, ny) = (x + ox, y + oy);
                if nx < 0 || ny < 0 || nx >= rw as i32 || ny >= rh as i32 {
                    continue;
                }
                let nli = ny as usize * rw + nx as usize;
                if done[nli] {
                    continue;
                }
                let v = speed(li, nli, ox, oy, p.tilt);
                if v <= 1e-6 {
                    continue;
                }
                let nk = key[li] + len * dx / v;
                if nk < key[nli] {
                    key[nli] = nk;
                    parent[nli] = li as u32;
                    heap.push(Reverse((kbits(nk), nli as u32)));
                }
            }
        }
        let lost = left.max(0.0);
        // the liquid through each cell: what still had to pass outward when
        // the front was there, shared by the cells of the front then (the
        // flow in the cloth is spread out between its pores, not channelled:
        // its pores are far finer than a pixel)
        let kmin0 = order.iter().map(|&li| key[li as usize]).fold(f32::INFINITY, f32::min);
        let band = |li: usize| ((key[li] - kmin0) / dx).max(0.0) as usize;
        let nb = order.iter().map(|&li| band(li as usize)).max().unwrap_or(0) + 1;
        let (mut cnt, mut vb) = (vec![0.0f32; nb], vec![0.0f64; nb]);
        for &li in &order {
            let li = li as usize;
            cnt[band(li)] += 1.0;
            vb[band(li)] += take[li] as f64;
        }
        // volume beyond each band, and the front's width there (smoothed
        // over a few bands)
        let mut beyond = vec![0.0f32; nb + 1];
        for b in (0..nb).rev() {
            beyond[b] = beyond[b + 1] + vb[b] as f32;
        }
        let width: Vec<f32> = (0..nb)
            .map(|b| {
                let (lo, hi) = (b.saturating_sub(2), (b + 3).min(nb));
                cnt[lo..hi].iter().sum::<f32>() / (hi - lo) as f32
            })
            .collect();
        // its pigment, filtered by the fibres along the way: the further
        // past where it landed, the less is left in the liquid
        let lam = FILTER_MM * (1.0 - p.mobility.clamp(0.0, 1.0)) + FILTER_MIN;
        let fr = 1.0 - (-lam * dx).exp();
        let mut dep = vec![0.0f32; nl]; // pigment, mm³
        let mut resident = vec![0.0f32; nl];
        let mean_k = {
            let (mut a, mut n) = (0.0f64, 0usize);
            for &li in &order {
                let c = cells[li as usize];
                if c.1 > 0.0 {
                    a += (0.5 * (c.1 + c.2)).ln() as f64;
                    n += 1;
                }
            }
            (a / n.max(1) as f64).exp() as f32
        };
        // where it landed, the puddle feeds everything beyond it: the flow
        // there is the flow out of the puddle's rim, more where more landed
        let (mut bl_max, mut lsum, mut ln) = (0usize, 0.0f32, 0usize);
        for &li in &order {
            let li = li as usize;
            if landv[li] > 0.0 {
                bl_max = bl_max.max(band(li));
                lsum += landv[li];
                ln += 1;
            }
        }
        let rim = (bl_max + 1).min(nb - 1);
        let through_rim = beyond[rim] / width[rim].max(1.0);
        let lmean = lsum / ln.max(1) as f32;
        for &li in &order {
            let li = li as usize;
            let b = band(li);
            let out = beyond[b] / width[b].max(1.0);
            let through = if landv[li] > 0.0 { (through_rim * (landv[li] / lmean.max(1e-9)).sqrt()).max(out) } else { out };
            let past = if landv[li] > 0.0 { 0.0 } else { (key[li] - off).max(0.0) };
            let cin = pig0 * (-lam * past).exp();
            // looser yarns take up a little more (the stain's mottle)
            let c = cells[li];
            let mott = if c.1 > 0.0 { (0.5 * (c.1 + c.2) / mean_k).powf(0.35) } else { 1.0 };
            dep[li] = through * cin * fr * mott;
            resident[li] = take[li] * cin * (1.0 - fr) * mott;
        }
        // (the routing is approximate: make the pigment add up to what was
        // poured)
        let sum: f64 = order.iter().map(|&li| (dep[li as usize] + resident[li as usize]) as f64).sum();
        let want = ((vol - lost) * pig0) as f64;
        if sum > 0.0 {
            let k = (want / sum) as f32;
            for &li in &order {
                dep[li as usize] *= k;
                resident[li as usize] *= k;
            }
        }
        // the edge of what it reached, and each cell's way to it
        let inside = |li: usize| done[li];
        let mut dedge = vec![f32::INFINITY; nl];
        let mut origin = vec![u32::MAX; nl];
        let mut heap = Heap::new();
        let mut edge_cells: Vec<usize> = Vec::new();
        for &li in &order {
            let li = li as usize;
            let (x, y) = ((li % rw) as i32, (li / rw) as i32);
            let rim = NEIGH[..4].iter().any(|&(ox, oy, _)| {
                let (nx, ny) = (x + ox, y + oy);
                nx < 0 || ny < 0 || nx >= rw as i32 || ny >= rh as i32 || !inside(ny as usize * rw + nx as usize)
            });
            if rim {
                dedge[li] = 0.0;
                origin[li] = li as u32;
                heap.push(Reverse((kbits(0.0), li as u32)));
                edge_cells.push(li);
            }
        }
        while let Some(Reverse((kb, li))) = heap.pop() {
            let li = li as usize;
            if f32::from_bits(kb) > dedge[li] {
                continue;
            }
            let (x, y) = ((li % rw) as i32, (li / rw) as i32);
            for &(ox, oy, len) in &NEIGH {
                let (nx, ny) = (x + ox, y + oy);
                if nx < 0 || ny < 0 || nx >= rw as i32 || ny >= rh as i32 {
                    continue;
                }
                let nli = ny as usize * rw + nx as usize;
                if !inside(nli) {
                    continue;
                }
                let nk = dedge[li] + len;
                if nk < dedge[nli] {
                    dedge[nli] = nk;
                    origin[nli] = origin[li];
                    heap.push(Reverse((kbits(nk), nli as u32)));
                }
            }
        }
        // the ring: suspended pigment drawn to the drying edge
        let eps0 = RING_BASE + RING_SOLV * s0;
        let mut ring = vec![0.0f32; nl];
        for &li in &order {
            let li = li as usize;
            let mv = resident[li];
            if mv > 0.0 && origin[li] != u32::MAX {
                let mvv = mv * eps0 * (-dedge[li] * dx / RING_MM).exp();
                resident[li] -= mvv;
                ring[origin[li] as usize] += mvv;
            }
        }
        let ring_total: f64 = ring.iter().map(|&v| v as f64).sum();
        let ringb = box_blur(&ring, rw, rh, ((0.6 / dx).round() as usize).max(1));
        let ringb_in: f64 = order.iter().map(|&li| ringb[li as usize] as f64).sum();
        let rscale = if ringb_in > 0.0 { (ring_total / ringb_in) as f32 } else { 0.0 };
        let pigdep: Vec<f32> = (0..nl).map(|li| if done[li] { dep[li] + resident[li] + ringb[li] * rscale } else { 0.0 }).collect();
        // oil: what the fibres and the pigment hold stays; the rest creeps on
        let um = |v_mm3: f32| v_mm3 / area * 1.0e3;
        let s = self.soak.as_ref().unwrap();
        let mut excess = vec![0.0f32; nl];
        let mut e_total = 0.0f32;
        for &li in &order {
            let li = li as usize;
            let i = gi(li);
            let oil_new = um(take[li] * oil0);
            let hold = OIL_RETAIN * s.cap[i] + PIG_OIL * (s.pig[i] + um(pigdep[li])) - (s.oil[i] + s.oil_pend[i]);
            let e = (oil_new - hold.max(0.0)).clamp(0.0, oil_new);
            excess[li] = e;
            e_total += e;
        }
        // the halo: from the edge outward, into cloth this pour didn't reach
        let mut hkey = vec![f32::INFINITY; nl];
        let mut horig = vec![u32::MAX; nl];
        let mut hdone = vec![false; nl];
        let mut halo: Vec<(usize, f32, f32, usize)> = Vec::new(); // (li, oil µm, mm, origin)
        let mut e_left = e_total;
        if e_total > 1e-4 {
            let mut heap = Heap::new();
            for &li in &edge_cells {
                hkey[li] = 0.0;
                horig[li] = li as u32;
                heap.push(Reverse((kbits(0.0), li as u32)));
            }
            while let Some(Reverse((kb, li))) = heap.pop() {
                let li = li as usize;
                if hdone[li] || f32::from_bits(kb) > hkey[li] {
                    continue;
                }
                hdone[li] = true;
                if !done[li] {
                    let i = gi(li);
                    let room = (HALO_FILL * s.cap[i] - (s.oil[i] + s.oil_pend[i] + s.solv_at(i, now))).max(0.0);
                    if cells[li].1 > 0.0 && room > 0.0 {
                        let put = room.min(e_left);
                        e_left -= put;
                        halo.push((li, put, hkey[li], horig[li] as usize));
                        if e_left <= 1e-4 {
                            break;
                        }
                    }
                }
                let (x, y) = ((li % rw) as i32, (li / rw) as i32);
                for &(ox, oy, len) in NEIGH16.iter() {
                    let (nx, ny) = (x + ox, y + oy);
                    if nx < 0 || ny < 0 || nx >= rw as i32 || ny >= rh as i32 {
                        continue;
                    }
                    let nli = ny as usize * rw + nx as usize;
                    if hdone[nli] || done[nli] {
                        continue;
                    }
                    let v = speed(li, nli, ox, oy, None);
                    if v <= 1e-6 {
                        continue;
                    }
                    // distance in mm, weighted by how readily the cloth
                    // wicks there (relative to its mean)
                    let nk = hkey[li] + len * dx / v.max(0.05);
                    if nk <= HALO_MM && nk < hkey[nli] {
                        hkey[nli] = nk;
                        horig[nli] = horig[li];
                        heap.push(Reverse((kbits(nk), nli as u32)));
                    }
                }
            }
        }
        let placed = (e_total - e_left).max(0.0);
        let keep = if e_total > 1e-6 { 1.0 - placed / e_total } else { 1.0 };
        // when things happen
        let mut spread_s = 0.0f32;
        let kmin = order.iter().map(|&li| key[li as usize]).fold(f32::INFINITY, f32::min).min(off);
        for &li in &order {
            let d = (key[li as usize] - kmin).max(0.0);
            spread_s = spread_s.max(d * d / (2.0 * kw));
        }
        let t_set = now + spread_s / 60.0;
        let coats_per_um = 1.0 / (COAT_UM * p.pigment.max(1e-3));
        let mut evap_end = vec![0.0f32; nl];
        let mut area_px = 0usize;
        let mut box_ = (w, h, 0usize, 0usize);
        let s = self.soak.as_mut().unwrap();
        for &li in &order {
            let li = li as usize;
            let i = gi(li);
            let (x, y) = (i % w, i / w);
            box_ = (box_.0.min(x), box_.1.min(y), box_.2.max(x + 1), box_.3.max(y + 1));
            if take[li] > 0.0 {
                area_px += 1;
            }
            let pig_um = um(pigdep[li]);
            if pig_um > 0.0 {
                let c = pig_um * coats_per_um;
                for ch in 0..3 {
                    s.kp[i][ch] += c * p.paint.k[ch];
                    s.sp[i][ch] += c * p.paint.s[ch];
                }
                s.pig[i] += pig_um;
            }
            let oil_new = um(take[li] * oil0);
            let stay = oil_new - excess[li] * (1.0 - keep);
            if stay > 0.0 {
                s.oil[i] += stay;
                s.oil_since[i] = s.oil_since[i].min(t_set);
            }
            // turpentine: evaporates over the next hour, the edges first
            let sn = s.solv_at(i, now);
            let add = um(take[li] * s0);
            let cap = s.cap[i].max(1.0);
            let fill = ((sn + add + s.oil[i]) / cap).min(1.0);
            let edge = (dedge[li] * dx / 15.0).min(1.0);
            let mut end = t_set + EVAP_MIN * fill * (0.45 + 0.55 * edge) * (cap / s.fabric.cap_um);
            if sn > 0.0 {
                end = end.max(s.solv_t1[i]);
            }
            if sn + add > 0.0 {
                s.solv[i] = sn + add;
                s.solv_t0[i] = t_set;
                s.solv_t1[i] = end;
            }
            evap_end[li] = end;
        }
        // the halo's oil arrives over the next days, carrying a trace of
        // the finest pigment
        let mut halo_mm = 0.0f32;
        let pc = p.paint;
        for &(li, put, d, o) in &halo {
            let i = gi(li);
            let (x, y) = (i % w, i / w);
            box_ = (box_.0.min(x), box_.1.min(y), box_.2.max(x + 1), box_.3.max(y + 1));
            halo_mm = halo_mm.max(d);
            let at = evap_end[o] + HALO_DAYS * 1440.0 * (d / HALO_MM).powi(2);
            if s.oil_pend[i] > 0.0 {
                s.oil_t[i] = s.oil_t[i].min(at);
            } else {
                s.oil_t[i] = at;
            }
            s.oil_pend[i] += put;
            let tint = p.mobility.powi(2) * HALO_TINT * um(pigdep[o]) * (-d / (0.4 * HALO_MM)).exp();
            if tint > 0.0 {
                let c = tint * coats_per_um;
                for ch in 0..3 {
                    s.kp[i][ch] += c * pc.k[ch];
                    s.sp[i][ch] += c * pc.s[ch];
                }
                s.pig[i] += tint;
            }
        }
        let b = (box_.0, box_.1, box_.2, box_.3);
        if b.2 > b.0 {
            s.active = grow(s.active, b);
            s.stained = grow(s.stained, b);
        }
        s.pours += 1;
        // the hand: pouring, then watching it spread
        let secs = 4.0 + p.ml as f64 / 25.0 + spread_s as f64;
        self.tally.pour(secs);
        if b.2 > b.0 {
            self.soak_render(b);
        }
        Ok(Poured {
            soaked_ml: (vol - lost) / 1000.0,
            lost_ml: lost / 1000.0,
            area_cm2: area_px as f32 * area / 100.0,
            spread_s,
            halo_mm,
            halo_ml: placed * area * 1.0e-6,
        })
    }

    /// Blot the wet stain under `m` with a rag or sponge (`strength` 0..1,
    /// times the mask's coverage): it lifts turpentine, some oil and the
    /// pigment still loose in it. Dry cloth gives nothing back. Returns the
    /// millilitres lifted.
    pub fn blot(&mut self, m: &Mask, strength: f32) -> Result<f32, String> {
        let Some(s) = self.soak.as_mut() else {
            return Err("blot: the canvas has a ground (blotting lifts what soaked into a raw canvas)".into());
        };
        let (w, h) = (self.f.w, self.f.h);
        let f = self.f;
        let area = self.mm_per_unit / f.scale;
        let area = area * area;
        let now = (self.wet.clock.now - s.t0) as f32;
        let mut lifted = 0.0f32;
        let (mut bx0, mut by0, mut bx1, mut by1) = (w, h, 0, 0);
        let mut n_px = 0usize;
        for i in 0..w * h {
            let mv = m.data[f.whole_index(i)] * strength.clamp(0.0, 1.0);
            if mv < 0.01 {
                continue;
            }
            n_px += 1;
            let sn = s.solv_at(i, now);
            if sn <= 0.0 {
                continue;
            }
            let wet = (sn / s.cap[i].max(1.0)).min(1.0);
            let lift = (mv * wet * 0.8).min(0.9);
            let pk = lift * 0.7;
            for ch in 0..3 {
                s.kp[i][ch] *= 1.0 - pk;
                s.sp[i][ch] *= 1.0 - pk;
            }
            s.pig[i] *= 1.0 - pk;
            lifted += (sn * lift + s.oil[i] * lift * 0.5) * 1.0e-3 * area;
            s.oil[i] *= 1.0 - 0.5 * lift;
            s.oil_pend[i] *= 1.0 - lift;
            s.solv[i] *= 1.0 - lift;
            let (x, y) = (i % w, i / w);
            bx0 = bx0.min(x);
            by0 = by0.min(y);
            bx1 = bx1.max(x + 1);
            by1 = by1.max(y + 1);
        }
        // a blot takes about a second per palm's width (60 mm square)
        self.tally.pour(2.0 + n_px as f64 * area as f64 / 3600.0);
        if bx1 > bx0 {
            self.soak_render((bx0, by0, bx1, by1));
        }
        Ok(lifted * 1.0e-3)
    }
}

/// Soak properties of a tube: pigment share of the tube paint's volume (the
/// rest oil), and how far its particles travel in cloth (0 coarse .. 1 very
/// fine). Estimates from particle size and oil absorption by kind.
pub fn tube_soak(name: &str) -> (f32, f32) {
    let n = name.to_ascii_lowercase();
    let has = |k: &str| n.contains(k);
    if has("smalt") {
        (0.35, 0.1)
    } else if has("prussian") || has("antwerp") {
        (0.25, 0.85)
    } else if has("madder") || has("alizarin") || has("magenta") || has("indian yellow") || has("lake") {
        (0.25, 0.9)
    } else if has("bitumen") {
        (0.3, 0.95)
    } else if has("black") || has("bone brown") {
        (0.3, 0.6)
    } else if has("lead white") || has("naples") || has("lead-tin") || has("red lead") {
        (0.5, 0.3)
    } else if has("zinc white") {
        (0.45, 0.4)
    } else if has("vermilion") {
        (0.45, 0.25)
    } else if has("sienna") || has("transparent oxide") {
        (0.35, 0.55)
    } else if has("ochre") || has("earth") || has("umber") || has("mars") || has("indian red") {
        (0.35, 0.4)
    } else if has("emerald") || has("cerulean") || has("cobalt violet") {
        (0.4, 0.2)
    } else if has("cobalt") || has("ultramarine") || has("rinmann") {
        (0.35, 0.3)
    } else if has("viridian") || has("copper") {
        (0.3, 0.45)
    } else if has("chrome") || has("cadmium") {
        (0.4, 0.35)
    } else {
        (0.35, 0.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Canvas;
    use crate::color::luminance;
    use crate::surface::Linen;

    fn raw(width: usize, mm: f32) -> Canvas {
        let mut c = Canvas::new_window(width, 1.0, Fabric::cotton_duck().color, None).with_size_mm(mm).with_linen(Linen { seed: 3, ..Linen::fine(3) });
        c.raw_canvas(Fabric::cotton_duck(), 3);
        c
    }

    fn blue_pour(ml: f32, thinner: f32) -> Pour {
        Pour { paint: Pigment::masstone(hex("#232a8c"), 4.0), pigment: 0.35, oil: 0.65, mobility: 0.3, thinner, ml, tilt: None, seed: 1 }
    }

    fn disc(c: &Canvas, x: f32, y: f32, r: f32) -> Mask {
        Mask::from_fn(c.frame(), move |u, v| if (u - x).powi(2) + (v - y).powi(2) < r * r { 1.0 } else { 0.0 })
    }

    fn mean_lum(c: &Canvas, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
        let f = c.frame();
        let (w, h) = (f.w, f.h);
        let (mut s, mut n) = (0.0f32, 0usize);
        for y in 0..h {
            for x in 0..w {
                let (u, v) = (x as f32 / f.scale, y as f32 / f.scale);
                if u >= x0 && u < x1 && v >= y0 && v < y1 {
                    s += luminance(c.pixels()[y * w + x]);
                    n += 1;
                }
            }
        }
        s / n.max(1) as f32
    }

    #[test]
    fn the_raw_cloth_looks_its_colour() {
        let c = raw(200, 300.0);
        let mut m = [0.0f32; 3];
        for p in c.pixels() {
            for ch in 0..3 {
                m[ch] += p[ch] / c.pixels().len() as f32;
            }
        }
        let want = Fabric::cotton_duck().color;
        for ch in 0..3 {
            assert!((m[ch] - want[ch]).abs() < 0.03, "{m:?} vs {want:?}");
        }
    }

    #[test]
    fn a_pour_soaks_conserves_and_spreads_with_volume() {
        let mut c = raw(300, 400.0);
        let small = c.pour(&disc(&c, 300.0, 500.0, 15.0), &blue_pour(5.0, 4.0)).unwrap();
        let big = c.pour(&disc(&c, 700.0, 500.0, 15.0), &blue_pour(40.0, 4.0)).unwrap();
        assert!(small.lost_ml < 1e-3 && big.lost_ml < 1e-3, "{small:?} {big:?}");
        assert!((small.soaked_ml - 5.0).abs() < 0.01);
        // the area holds the volume: 300 µm of pores per mm²
        let a = big.area_cm2 * 100.0 * 0.3 / 1000.0;
        assert!(a > 35.0 && a < 48.0, "{big:?}");
        assert!(big.area_cm2 > 5.0 * small.area_cm2, "{small:?} {big:?}");
        // it stains: the middle of the stain is darker than raw cloth
        let raw_l = luminance(Fabric::cotton_duck().color);
        assert!(mean_lum(&c, 690.0, 490.0, 710.0, 510.0) < 0.6 * raw_l);
    }

    #[test]
    fn wet_cloth_is_darker_and_dries_lighter() {
        let mut c = raw(200, 300.0);
        c.pour(&disc(&c, 500.0, 500.0, 40.0), &blue_pour(20.0, 6.0)).unwrap();
        let wet = mean_lum(&c, 480.0, 480.0, 520.0, 520.0);
        c.wait(240.0);
        let dry = mean_lum(&c, 480.0, 480.0, 520.0, 520.0);
        assert!(dry > wet * 1.1, "wet {wet} dry {dry}");
    }

    #[test]
    fn oily_paint_leaves_a_halo_past_the_colour() {
        let mut c = raw(300, 600.0);
        // a pile rich in oil medium (half medium): oil beyond what the
        // pigment holds
        let p = Pour { pigment: 0.35 * 0.4, oil: 1.0 - 0.35 * 0.4, ..blue_pour(8.0, 3.0) };
        let r = c.pour(&disc(&c, 300.0, 500.0, 30.0), &p).unwrap();
        assert!(r.halo_ml > 0.05 && r.halo_mm > 3.0, "{r:?}");
        // lean paint (much turpentine, little oil) on clean cloth: none
        let lean = c.pour(&disc(&c, 800.0, 200.0, 20.0), &blue_pour(3.0, 20.0)).unwrap();
        assert!(lean.halo_ml < 1e-3, "{lean:?}");
        // the halo creeps in over days
        c.wait(10.0 * 1440.0);
        let s = c.soak.as_ref().unwrap();
        assert!(s.oil_pend.iter().all(|&v| v == 0.0));
        assert!(s.active.is_none());
    }

    #[test]
    fn a_soaked_canvas_checkpoints_exactly() {
        let mut c = raw(160, 300.0);
        c.pour(&disc(&c, 450.0, 450.0, 30.0), &Pour { pigment: 0.14, oil: 0.86, ..blue_pour(8.0, 3.0) }).unwrap();
        c.wait(30.0);
        let mut b = Vec::new();
        c.write_state(&mut b, "t").unwrap();
        let (mut d, _) = Canvas::read_state(&mut std::io::Cursor::new(b)).unwrap();
        c.wait(3.0 * 1440.0);
        d.wait(3.0 * 1440.0);
        assert_eq!(c.pixels(), d.pixels());
        // a canvas with no soak writes what it always wrote
        let p = Canvas::new_window(8, 1.0, [0.5; 3], None);
        let mut b = Vec::new();
        p.write_state(&mut b, "").unwrap();
        let (q, _) = Canvas::read_state(&mut std::io::Cursor::new(b.clone())).unwrap();
        assert!(q.soak.is_none());
        b.extend_from_slice(&7u64.to_le_bytes());
        assert!(Canvas::read_state(&mut std::io::Cursor::new(b)).is_err());
    }

    #[test]
    fn charcoal_stays_on_top_of_a_stain() {
        let mut c = raw(200, 300.0);
        let ch = crate::graphite::Lead::chalk();
        let pts: Vec<(f32, f32)> = (0..=200).map(|k| (300.0 + 2.0 * k as f32, 500.0)).collect();
        let mark = crate::graphite::Mark { pressure: vec![0.8; pts.len()], pts };
        c.draw(&ch, &mark, 0.0, 5);
        let line = mean_lum(&c, 480.0, 497.0, 520.0, 503.0);
        c.pour(&disc(&c, 500.0, 500.0, 40.0), &blue_pour(20.0, 6.0)).unwrap();
        c.wait(240.0);
        let on_line = mean_lum(&c, 480.0, 497.0, 520.0, 503.0);
        let beside = mean_lum(&c, 480.0, 520.0, 520.0, 526.0);
        assert!(on_line < beside * 0.9, "line {line} on {on_line} beside {beside}");
    }

    #[test]
    fn pours_are_deterministic() {
        let run = || {
            let mut c = raw(200, 300.0);
            c.pour(&disc(&c, 400.0, 400.0, 30.0), &blue_pour(12.0, 5.0)).unwrap();
            c.pour(&disc(&c, 550.0, 500.0, 30.0), &Pour { tilt: Some((1.0, 0.5)), ..blue_pour(12.0, 8.0) }).unwrap();
            c.wait(60.0);
            c.pixels().to_vec()
        };
        assert_eq!(run(), run());
    }
}
