//! A raw canvas: the bare cloth, unprimed, as Frankenthaler and Morris Louis
//! left it for their soak-stains. The physics and its sources are in
//! notes/research/soak_stain.md; numbers marked "estimate" there are
//! estimates here too.
//!
//! **The cloth** is woven from a `Fabric`: its colour, the liquid its pores
//! hold when saturated (`Fabric::cap_um`), and how much faster liquid wicks
//! along the warp than across it. The weave's relief (the canvas's surface
//! as the linen makes it) varies the cloth from pixel to pixel: thread tops
//! hold more fibre than the gaps between them.
//!
//! **Appearance**: the cloth is one Kubelka–Munk layer over a backing,
//! scattering more on the thread tops, with the absorption that makes the
//! dry cloth look its colour. Paint lies on it as on any canvas, and a film
//! hides the cloth under it. With no ground to break, age cracks
//! (`Canvas::crack`) run only through the paint.
//!
//! **Limits.** A raw canvas is painted whole, never as a crop render: the
//! weave is measured over the whole cloth.

use crate::canvas::Canvas;
use crate::color::{Rgb, hex};
use crate::pigment::layer1;
use rayon::prelude::*;
use std::borrow::Cow;

/// Scattering of the dry cloth through its thickness (estimate: raw cotton
/// duck lets some light through when held up, so not much over 5).
const S_CLOTH: f32 = 6.0;
/// Reflectance of what is behind the cloth (the stretcher bars, the wall).
const BACKING: f32 = 0.35;
/// How much more the thread tops scatter than the cloth's mean (and the
/// gaps less).
pub(crate) const WEAVE_AMP: f32 = 0.16;

/// The longest name a fabric can have, in bytes (a checkpoint keeps it).
pub const NAME_MAX: usize = 64;

/// The cloth a raw canvas is woven from.
#[derive(Clone, Debug, PartialEq)]
pub struct Fabric {
    /// At most `NAME_MAX` bytes.
    pub name: Cow<'static, str>,
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
        Fabric { name: "cotton duck".into(), color: hex("#e3d9c4"), cap_um: 300.0, warp_bias: 1.3 }
    }
    /// Raw linen: browner, denser and thinner, it holds less.
    pub fn linen() -> Fabric {
        Fabric { name: "linen".into(), color: hex("#a8966f"), cap_um: 220.0, warp_bias: 1.2 }
    }
    /// The fabrics with names.
    pub fn all() -> [Fabric; 2] {
        [Fabric::cotton_duck(), Fabric::linen()]
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
    /// A cloth a raw canvas can be woven from (and a checkpoint can keep):
    /// a name of at most `NAME_MAX` bytes, a colour of finite channels not
    /// below 0, a positive finite pore volume and warp bias.
    pub fn is_valid(&self) -> bool {
        let finite = |v: f32| v.is_finite();
        self.name.len() <= NAME_MAX && self.color.iter().all(|&c| finite(c) && c >= 0.0) && finite(self.cap_um) && self.cap_um > 0.0 && finite(self.warp_bias) && self.warp_bias > 0.0
    }
}

/// The cloth of a raw canvas, per pixel of its window.
#[derive(Clone)]
pub struct Soak {
    pub(crate) fabric: Fabric,
    /// The clock (minutes) when the canvas was set up.
    pub(crate) t0: f64,
    pub(crate) seed: u64,
    /// The dry cloth's absorption (per its thickness), so that it looks its
    /// colour (from `fabric.color`: `Soak::new` works it out).
    pub(crate) kf: Rgb,
    /// Scattering factor per pixel from the weave's relief, within
    /// 1 ± `WEAVE_AMP`.
    pub(crate) weave: Vec<f32>,
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

impl Soak {
    /// The cloth of `fabric` with this weave, set up at clock `t0`.
    pub(crate) fn new(fabric: Fabric, t0: f64, seed: u64, weave: Vec<f32>) -> Soak {
        let kf = std::array::from_fn(|c| solve_k(fabric.color[c]));
        Soak { fabric, t0, seed, kf, weave }
    }

    /// The colour of pixel `i`.
    pub(crate) fn shade(&self, i: usize) -> Rgb {
        let sf = S_CLOTH * self.weave[i];
        std::array::from_fn(|c| cloth(self.kf[c], sf))
    }

    /// Words for what is in the cloth at pixel `i` (for `soaked`).
    pub fn describe(&self, _i: usize) -> String {
        "raw".to_string()
    }
}

impl Canvas {
    /// Leave the canvas raw: no ground, the bare `fabric`. Call it on a bare
    /// engine-3 canvas (no ground, no paint yet) with a valid fabric
    /// (`Fabric::is_valid`); panics otherwise, and on a crop render
    /// (`set_crop`): the weave is measured over the whole cloth.
    pub fn raw_canvas(&mut self, fabric: Fabric, seed: u64) {
        assert!(self.f.is_whole(), "a raw canvas is painted whole, not as a crop render: the weave is measured over the whole cloth");
        assert!(self.engine >= 3, "a raw canvas is engine 3's");
        assert!(fabric.is_valid(), "an invalid fabric: {fabric:?}");
        assert!(self.ground_um == 0.0 && self.film.iter().chain(&self.wet.vol).all(|&v| v == 0.0), "a raw canvas is set up bare: no ground, no paint");
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
        self.soak = Some(Box::new(Soak::new(fabric, self.clock(), seed, weave)));
        self.soak_render((0, 0, w, h));
    }

    /// Is this a raw canvas (`raw_canvas`)?
    pub fn is_raw(&self) -> bool {
        self.soak.is_some()
    }

    /// The raw fabric, if this is a raw canvas.
    pub fn fabric(&self) -> Option<Fabric> {
        self.soak.as_ref().map(|s| s.fabric.clone())
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
        let mut d = s.describe(i);
        if self.film[i] > 1e-3 {
            d.push_str(", under a paint film");
        } else if self.wet.vol[i] > 1e-3 {
            d.push_str(", under wet paint");
        }
        Some(d)
    }

    /// Colour the cloth in buffer box `b` (only where no paint or ground
    /// film lies on it). It replaces those pixels' colour: lighting that
    /// `relief` added there is lost, and a film thinner than 0.001 coats
    /// (0.025 µm, below anything visible) counts as bare cloth.
    pub(crate) fn soak_render(&mut self, b: (usize, usize, usize, usize)) {
        let Some(s) = self.soak.as_ref() else { return };
        let w = self.f.w;
        let (x0, y0, x1, y1) = (b.0, b.1, b.2.min(w), b.3.min(self.f.h));
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let film = &self.film;
        self.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let i = (y0 + j) * w + x;
                if film[i] < 1e-3 {
                    row[x] = s.shade(i);
                }
            }
        });
    }
}
