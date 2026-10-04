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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface::Linen;

    fn on(width: usize, fabric: Fabric) -> Canvas {
        let mut c = Canvas::new_window(width, 1.0, fabric.color, None).with_size_mm(300.0).with_linen(Linen { seed: 3, ..Linen::fine(3) });
        c.raw_canvas(fabric, 3);
        c
    }

    fn mean(c: &Canvas) -> Rgb {
        let mut m = [0.0f64; 3];
        for p in c.pixels() {
            for k in 0..3 {
                m[k] += p[k] as f64;
            }
        }
        m.map(|v| (v / c.pixels().len() as f64) as f32)
    }

    #[test]
    fn the_raw_cloth_looks_its_colour() {
        for fabric in [Fabric::cotton_duck(), Fabric::linen()] {
            let c = on(200, fabric.clone());
            let m = mean(&c);
            for k in 0..3 {
                assert!((m[k] - fabric.color[k]).abs() < 0.03, "{}: {m:?} vs {:?}", fabric.name, fabric.color);
            }
        }
    }

    #[test]
    fn the_weave_shows_in_the_cloth() {
        let c = on(200, Fabric::cotton_duck());
        let s = c.soak.as_ref().unwrap();
        let (lo, hi) = s.weave.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &q| (lo.min(q), hi.max(q)));
        assert!(lo >= 1.0 - WEAVE_AMP && hi <= 1.0 + WEAVE_AMP && hi - lo > WEAVE_AMP, "weave {lo}..{hi}");
        // thread tops scatter more, so they are lighter than the gaps
        let lum = |i: usize| c.pixels()[i].iter().sum::<f32>();
        let top = (0..s.weave.len()).max_by(|&a, &b| s.weave[a].total_cmp(&s.weave[b])).unwrap();
        let gap = (0..s.weave.len()).min_by(|&a, &b| s.weave[a].total_cmp(&s.weave[b])).unwrap();
        assert!(lum(top) > lum(gap), "{} vs {}", lum(top), lum(gap));
    }

    #[test]
    fn the_cloth_says_what_lies_on_it() {
        let mut c = on(100, Fabric::cotton_duck());
        assert_eq!(c.soaked_at(500.0, 500.0).as_deref(), Some("raw"));
        assert_eq!(c.soaked_at(-1.0, 500.0), None);
        let i = 50 * c.f.w + 50;
        c.wet.vol[i] = 1.0;
        assert_eq!(c.soaked_at(500.0, 500.0).as_deref(), Some("raw, under wet paint"));
        c.film[i] = 1.0;
        assert_eq!(c.soaked_at(500.0, 500.0).as_deref(), Some("raw, under a paint film"));
        // a primed canvas has no cloth to soak
        let p = Canvas::new_window(100, 1.0, [0.8; 3], None);
        assert!(!p.is_raw() && p.fabric().is_none() && p.soaked_at(500.0, 500.0).is_none());
    }

    fn saved(c: &Canvas) -> Vec<u8> {
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        b
    }

    fn load(b: Vec<u8>) -> std::io::Result<Canvas> {
        Canvas::read_state(&mut std::io::Cursor::new(b)).map(|(c, _)| c)
    }

    fn err(b: Vec<u8>) -> String {
        load(b).err().map(|e| e.to_string()).unwrap_or_default()
    }

    #[test]
    fn a_raw_canvas_checkpoints_exactly() {
        use crate::bristle::Tool;
        use crate::handling::Handling;
        use crate::mask::Mask;
        let mut c = on(80, Fabric::cotton_duck());
        let b = saved(&c);
        assert_eq!(&b[..8], b"PAINTC10");
        let mut d = load(b.clone()).unwrap();
        // all of it came back: written again, it is the same file, and the
        // cloth's absorption, which isn't stored, is worked out alike
        assert!(saved(&d) == b);
        assert_eq!(d.soak.as_ref().unwrap().kf, c.soak.as_ref().unwrap().kf);
        assert_eq!(d.pixels(), c.pixels());
        // and painting goes on alike
        let base = c.pixels().to_vec();
        let f = c.frame();
        let hd = Handling::new(Tool::round_sable(4.0)).length(10.0, 20.0).coverage(2.0).color(|_, _| [0.08, 0.12, 0.18]).hug(false).clip(false).fill(false);
        for k in [&mut c, &mut d] {
            k.work(&Mask::from_fn(f, |x, y| if (x - 500.0).abs() < 200.0 && (y - 500.0).abs() < 60.0 { 1.0 } else { 0.0 }), &hd, 77);
            k.dry();
        }
        assert!(c.pixels() != base, "the stroke painted nothing");
        assert!(c.pixels() == d.pixels(), "resumed painting differs");
        // a primed canvas writes version 9 and has no cloth
        let p = load(saved(&Canvas::new_window(8, 1.0, [0.5; 3], None).with_engine(3))).unwrap();
        assert!(&saved(&p)[..8] == b"PAINTCK9" && p.soak.is_none());
        // New engines retain both cloth state and the extended material arrays.
        let mut material = on(40, Fabric::cotton_duck()).with_engine(4);
        material.wet.vol[0] = 0.5;
        material.wet.hide[0] = [0.8, 0.6, 1.1, 0.2, 0.7];
        material.gloss[0] = 0.4;
        material.absorb[0] = 0.3;
        let bytes = saved(&material);
        assert_eq!(&bytes[..8], b"PAINTC12");
        let resumed = load(bytes.clone()).unwrap();
        assert!(saved(&resumed) == bytes, "raw material state must round-trip without loss");
    }

    #[test]
    fn a_cloth_of_ones_own_checkpoints() {
        // its numbers and its name, spelt as it was (even as a named
        // cloth's other spelling)
        for name in ["raw silk", "duck"] {
            let silk = Fabric { name: name.into(), color: hex("#efe6d2"), cap_um: 140.0, warp_bias: 1.1 };
            let d = load(saved(&on(40, silk.clone()))).unwrap();
            assert_eq!(d.fabric(), Some(silk));
        }
        // a fabric the reader would refuse can't be written either: a name
        // too long, a number out of range
        let mut c = on(40, Fabric::cotton_duck());
        c.soak.as_mut().unwrap().fabric.name = "a cloth with a name much longer than any checkpoint will keep for it".into();
        assert!(c.write_state(&mut Vec::new(), "").is_err());
        let mut c = on(40, Fabric::cotton_duck());
        c.soak.as_mut().unwrap().fabric.cap_um = 0.0;
        assert!(c.write_state(&mut Vec::new(), "").is_err());
    }

    // a raw canvas is engine 3's: set to an older engine, it won't write a
    // checkpoint it couldn't read back
    #[test]
    fn a_raw_canvas_on_an_older_engine_is_not_written() {
        let c = on(40, Fabric::cotton_duck()).with_engine(2);
        assert!(c.write_state(&mut Vec::new(), "").is_err());
    }

    #[test]
    #[should_panic(expected = "invalid fabric")]
    fn an_invalid_fabric_is_refused() {
        on(40, Fabric { cap_um: f32::NAN, ..Fabric::cotton_duck() });
    }

    #[test]
    #[should_panic(expected = "set up bare")]
    fn a_primed_canvas_cannot_be_made_raw() {
        let mut c = Canvas::new_window(40, 1.0, [0.8; 3], None).with_size_mm(300.0);
        c.film[7] = 2.0;
        c.raw_canvas(Fabric::cotton_duck(), 3);
    }

    #[test]
    fn a_corrupt_soak_checkpoint_is_an_error() {
        let base = on(40, Fabric::cotton_duck());
        let refused = |edit: &dyn Fn(&mut Soak)| {
            let mut c = base.clone();
            edit(c.soak.as_mut().unwrap());
            load(saved(&c)).is_err()
        };
        assert!(!refused(&|_| {}));
        assert!(refused(&|s| s.t0 = f64::NAN));
        assert!(refused(&|s| s.weave[5] = f32::NAN));
        assert!(refused(&|s| s.weave[5] = 0.0));
        assert!(refused(&|s| s.weave[5] = 1.0 + 2.0 * WEAVE_AMP));
        // the layout: version 9's bytes, the engine word before its
        // solvent, then the section (mark, version, name, five numbers, the
        // clock, the seed, the weave), which ends the file
        let b = saved(&base);
        let n = base.f.w * base.f.h;
        let section = 8 + 8 + 8 + "cotton duck".len() + 5 * 4 + 8 + 8 + 4 * n;
        let at = b.len() - section;
        assert_eq!(&b[at..at + 8], &0x4b414f53u64.to_le_bytes());
        // the fabric's numbers (colour, pore volume, warp bias), which the
        // writer won't write wrong, so here they are spoilt in the file
        let nums = at + 24 + "cotton duck".len();
        for (k, v) in [(1, f32::NAN), (2, -0.5), (3, 0.0), (4, -1.0), (4, f32::INFINITY)] {
            let mut bad = b.clone();
            bad[nums + 4 * k..nums + 4 * k + 4].copy_from_slice(&v.to_le_bytes());
            assert!(err(bad).contains("soak is invalid"), "number {k} = {v}");
        }
        let mut e2 = b.clone();
        e2[at - 4 * n - 8..at - 4 * n].copy_from_slice(&2u64.to_le_bytes());
        assert!(err(e2).contains("engine-1 or engine-2"));
        let mut v2 = b.clone();
        v2[at + 8..at + 16].copy_from_slice(&2u64.to_le_bytes());
        assert!(err(v2).contains("soak section is version 2"));
        let mut mark = b.clone();
        mark[at] ^= 1;
        assert!(err(mark).contains("soak mark is wrong"));
        let mut tail = b.clone();
        tail.push(0);
        assert!(err(tail).contains("trailing data"));
        assert!(load(b[..b.len() - 1].to_vec()).is_err());
        // a primed canvas's file read as a raw one's has no section
        let mut p = saved(&Canvas::new_window(8, 1.0, [0.5; 3], None));
        p[..8].copy_from_slice(b"PAINTC10");
        assert!(load(p).is_err());
        // nor can the cloth be a crop render's (the frame's full width is
        // at byte 48 of a file with an empty header)
        let mut crop = b.clone();
        crop[48..56].copy_from_slice(&(base.f.w as u64 + 1).to_le_bytes());
        assert!(err(crop).contains("crop render"));
    }

    #[test]
    #[should_panic(expected = "painted whole")]
    fn a_crop_render_cannot_be_raw() {
        let crop = crate::canvas::Crop { units: [100.0, 100.0, 300.0, 300.0], margin: 10.0 };
        let mut c = Canvas::new_window(200, 1.0, Fabric::cotton_duck().color, Some(crop)).with_size_mm(300.0);
        c.raw_canvas(Fabric::cotton_duck(), 3);
    }
}
