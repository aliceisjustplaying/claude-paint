//! Wet paint sitting on top of the dry picture.
//!
//! Every pixel holds a volume of wet paint (thickness, in "layer units": 1.0
//! is one normal coat), its pigment mixture as a Mixbox latent vector of the
//! masstone (mixing is linear in latent space, weighted by volume) and its
//! Kubelka–Munk scattering per coat (mixed linearly by volume, as K and S mix
//! in the two-constant KM model; absorption follows from masstone and S).
//! Brushes exchange paint with this layer. It ages as the painting's clock
//! runs (`Canvas::wait`, see `drying`); `Canvas::dry` waits until it is
//! touch-dry, baking it into the dry picture with Kubelka–Munk.
//!
//! **What a paint's color means.** `Paint::color` is its *masstone*: the
//! color the paint has laid thick, which is also how it looks laid over
//! paint of that same color, at any thickness. Hiding decides how much of a
//! different underlayer shows through a thin coat. So a mark mixed to the
//! color of the field it sits in disappears into it, whether it is body
//! color or a thin scumble; a thin coat over a different color lands between
//! the two, as real paint does. To ask "what will this look like *here*",
//! use `Paint::over`, `Canvas::under` and the aiming calls in `palette`
//! (`Palette::aim`, `Canvas::aim`). Glazes named by the tint they give a
//! white ground use `Paint::tint`/`Paint::glaze`.

use crate::canvas::Canvas;
use crate::color::{Rgb, luminance};
use crate::pigment::{Pigment, hiding_of, scatter_for};

pub const LAT: usize = mixbox::LATENT_SIZE;
pub type Latent = [f32; LAT];
/// Paint properties mixed by volume alongside the pigment: [KM scattering per
/// coat, stiffness, drying rate].
/// Stiffness 0 = fluid, medium-rich glaze; 1 = stiff tube paint. Drying rate
/// relative to average paint (see `drying::drier`).
pub type Prop = [f32; 3];

#[inline]
fn lerp_prop(p: &mut Prop, q: Prop, a: f32) {
    for k in 0..3 {
        p[k] += (q[k] - p[k]) * a;
    }
}

/// A paint as squeezed from the tube and thinned with medium.
///
/// It carries its Kubelka–Munk scattering `scatter` directly, so a mixture
/// handed to the brush keeps the S it was scored with, however opaque it is
/// (hiding saturates near 1 and can't carry a large S). Name paints by
/// hiding with `Paint::new`/`with_hiding`; `hiding()` reports it back.
#[derive(Clone, Copy, Debug)]
pub struct Paint {
    /// Masstone, linear RGB: the paint's color laid thick (and over itself).
    pub color: Rgb,
    /// Kubelka–Munk scattering per coat (flat across channels; absorption
    /// follows from it and the masstone). See `pigment::scatter_for`.
    pub scatter: f32,
    /// Stiffness: 0 = fluid, rich in medium (a glaze), 1 = stiff tube paint.
    /// Sets how the paint levels as it dries (see `surface::settle`). How
    /// much paint goes on the brush is the separate `amount` of `Held::load`.
    pub stiff: f32,
    /// How fast it dries, relative to average paint (1): lead white and
    /// umber about 2, bone black and lakes 0.3–0.4 (`drying::drier`). With
    /// film thickness and fat (low `stiff`) it sets how long the paint stays
    /// open (see `Canvas::wait`).
    pub drying: f32,
}

impl Paint {
    /// A paint of masstone `color` whose one coat hides `hiding` (contrast
    /// ratio: over black ÷ over white; 0.05 = glaze, 0.5 = scumble,
    /// 0.92 = body).
    pub fn new(color: Rgb, hiding: f32, stiff: f32) -> Self {
        Paint { color, scatter: scatter_for(luminance(color), hiding), stiff, drying: 1.0 }
    }
    /// A paint of masstone `color` that scatters `scatter` per coat.
    pub fn km(color: Rgb, scatter: f32, stiff: f32) -> Self {
        Paint { color, scatter, stiff, drying: 1.0 }
    }
    pub fn body(color: Rgb) -> Self {
        Paint::new(color, 0.92, 1.0)
    }
    pub fn scumble(color: Rgb) -> Self {
        Paint::new(color, 0.5, 0.6)
    }
    /// This paint with its scattering set so one coat hides `hiding`
    /// (the masstone, stiffness and drying rate stay).
    pub fn with_hiding(self, hiding: f32) -> Self {
        Paint { scatter: scatter_for(luminance(self.color), hiding), ..self }
    }
    /// This paint with stiffness `stiff`.
    pub fn with_stiff(self, stiff: f32) -> Self {
        Paint { stiff, ..self }
    }
    /// This paint drying at `rate` relative to average paint (see
    /// `drying::drier`: lead white 2, bone black 0.4).
    pub fn with_drying(self, rate: f32) -> Self {
        Paint { drying: rate, ..self }
    }
    /// Hiding power of one coat (contrast ratio), derived from the
    /// scattering; for reporting (it rounds to 1 for strong scatterers).
    pub fn hiding(&self) -> f32 {
        hiding_of(luminance(self.color), self.scatter)
    }
    /// A transparent glaze that tints a white ground to `tint` at one coat.
    pub fn glaze(tint: Rgb) -> Self {
        Paint::tint(tint, 0.07, 0.3)
    }
    /// A paint named by its tint: one coat of it over white looks `tint`
    /// (the old "appearance over white" meaning). Its masstone is deeper.
    pub fn tint(tint: Rgb, hiding: f32, stiff: f32) -> Self {
        Paint::solve(tint, [1.0; 3], 1.0, hiding, stiff)
    }
    /// The paint of this hiding that, laid `coats` thick over `under`,
    /// looks `want` (or as near as a paint can: a light glaze cannot
    /// lighten a dark underlayer, so it comes out as light as it can).
    pub fn aimed(want: Rgb, under: Rgb, coats: f32, hiding: f32, stiff: f32) -> Self {
        Paint::solve(want, under, coats, hiding, stiff)
    }
    fn solve(want: Rgb, under: Rgb, coats: f32, hiding: f32, stiff: f32) -> Self {
        // For a trial masstone luminance `l` the scattering is fixed
        // (`scatter_for(l, hiding)`) and each channel's masstone follows by
        // bisection. The answer is a root of `luminance(m(l)) − l`, which is
        // ≥ 0 at the darkest masstone and ≤ 0 at the lightest, so bisecting
        // on `l` always converges (a plain fixed-point iteration can
        // oscillate and stop far from it). Whatever `l` it lands on, the
        // paint carries the scattering its channels were solved with, so
        // its look over `under` is exact wherever the target is reachable.
        let fit = |l: f32| -> (Rgb, f32) {
            let s = scatter_for(l, hiding);
            let m = std::array::from_fn(|c| {
                let (mut lo, mut hi) = (0.002f32, 0.995f32);
                for _ in 0..30 {
                    let mid = 0.5 * (lo + hi);
                    if Pigment::masstone([mid; 3], s).over([under[c]; 3], coats)[0] < want[c] {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                0.5 * (lo + hi)
            });
            (m, s)
        };
        let (mut lo, mut hi) = (0.002f32, 0.995f32);
        for _ in 0..24 {
            let mid = 0.5 * (lo + hi);
            if luminance(fit(mid).0) > mid {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let (color, scatter) = fit(0.5 * (lo + hi));
        Paint { color, scatter, stiff, drying: 1.0 }
    }
    pub fn latent(&self) -> Latent {
        mixbox::linear_float_rgb_to_latent(&self.color)
    }
    /// Kubelka–Munk scattering per coat (the `scatter` field).
    pub fn scatter(&self) -> f32 {
        self.scatter
    }
    /// The paint as a Kubelka–Munk layer (per coat).
    pub fn pigment(&self) -> Pigment {
        Pigment::masstone(self.color, self.scatter)
    }
    /// What `coats` of this paint look like laid over `under`.
    pub fn over(&self, under: Rgb, coats: f32) -> Rgb {
        self.pigment().over(under, coats)
    }
}

#[derive(Clone)]
pub(crate) struct Wet {
    pub(crate) vol: Vec<f32>,
    pub(crate) lat: Vec<Latent>,
    /// [scattering, stiffness, drying rate] of the wet paint.
    pub(crate) hide: Vec<Prop>,
    /// Which stroke last laid paint here (a stroke barely re-picks its own paint).
    pub(crate) stroke: Vec<u32>,
    /// Stroke that last touched a pixel, and the film floor that stroke may
    /// not lift below (one pass lifts only part of the film).
    pub(crate) touched: Vec<u32>,
    pub(crate) floor: Vec<f32>,
    /// Share of the pixel the wet paint covers (1 = all of it). Only the
    /// hairs of a pointed tool, finer than a pixel, lay paint over part of
    /// one; `dry` composites that paint at its real thickness over that
    /// share, so a hairline looks alike at any resolution.
    pub(crate) cover: Vec<f32>,
    /// Id of the stroke being painted.
    pub(crate) current: u32,
    /// Dirty bounding box in pixels (x0, y0, x1, y1), if any paint is wet.
    pub(crate) dirty: Option<(usize, usize, usize, usize)>,
    /// The painting's clock and how far each film has dried (`drying`).
    pub(crate) clock: crate::drying::Clock,
}

impl Wet {
    pub fn new(n: usize) -> Self {
        Wet { vol: vec![0.0; n], lat: vec![[0.0; LAT]; n], hide: vec![[0.0, 0.5, 1.0]; n], stroke: vec![0; n], touched: vec![0; n], floor: vec![0.0; n], cover: vec![1.0; n], current: 0, dirty: None, clock: Default::default() }
    }

    pub fn touch(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        self.dirty = Some(match self.dirty {
            None => (x0, y0, x1, y1),
            Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
        });
    }

}

/// Mix `v` of (`lat`, `hide`) into a reservoir (`rv`, `rl`, `rh`).
#[inline]
pub fn mix_into(rv: &mut f32, rl: &mut Latent, rh: &mut Prop, v: f32, lat: &Latent, hide: Prop) {
    if v <= 0.0 {
        return;
    }
    let t = *rv + v;
    let a = v / t;
    for k in 0..LAT {
        rl[k] += (lat[k] - rl[k]) * a;
    }
    lerp_prop(rh, hide, a);
    *rv = t;
}


/// `coats` of `pig` over `under`, laid over the share `cover` of a pixel:
/// the paint sits there `coats / cover` thick and the rest of the pixel
/// shows `under` (area-weighted in linear light, as the eye averages it).
/// However small the share, it weighs what it covers: a hair's edge over a
/// thousandth of a pixel darkens it by a thousandth. (The paint's thickness
/// there is capped far beyond hiding, so a vanishing share stays finite.)
pub(crate) fn over_share(pig: Pigment, under: Rgb, coats: f32, cover: f32) -> Rgb {
    if cover >= 1.0 {
        return pig.over(under, coats);
    }
    if cover.is_nan() || cover <= 0.0 {
        return under;
    }
    let c = cover;
    let o = pig.over(under, (coats / c).min(1e4));
    [under[0] + (o[0] - under[0]) * c, under[1] + (o[1] - under[1]) * c, under[2] + (o[2] - under[2]) * c]
}

/// Paste stands at most this tall for its width (a bead of oil paint: stroke
/// ridges of 0.1-0.3 mm on strokes 1-2 mm wide are 0.1-0.3; 0.5 is stiff
/// impasto).
const BEAD_ASPECT: f32 = 0.5;

/// The share of a pixel (`px_um` wide) that `coats` of wet paint in it
/// cover: at least `cover` (what the hairs touched), and at least enough
/// that the paint on it stands no taller than `BEAD_ASPECT` times the
/// pixel's width. A sliver of a pixel can't hold a bead taller than that:
/// the paint has slumped across it (c ≥ coats·COAT_UM / (BEAD_ASPECT·px_um)).
/// Linear in the paint, so a hairline's fringe keeps its share wherever the
/// line falls between pixel centers; at 1000px (pixels of 0.2-0.9 mm) it
/// takes a film over 100 µm on the sliver to act. At 3200 (0.1 mm pixels)
/// the tens of µm that leveling pours into a pixel a hair only grazed no
/// longer sit on a sliver of it and leave the rest bare (the lab 2 lime's
/// pale pinholes, notes/glitch.md P1).
pub(crate) fn bead_cover(cover: f32, coats: f32, px_um: f32) -> f32 {
    if cover >= 1.0 || coats.is_nan() || coats <= 0.0 || px_um.is_nan() || px_um <= 0.0 {
        return cover;
    }
    let need = coats * crate::surface::COAT_UM / (BEAD_ASPECT * px_um);
    cover.max(need).min(1.0)
}

impl Canvas {
    /// What the painter sees at pixel `i`: the dry picture with any wet paint
    /// on it (at its laid thickness, before it levels).
    pub(crate) fn look_px(&self, i: usize) -> Rgb {
        let v = self.wet.vol[i];
        if v < 1e-5 {
            return self.px[i];
        }
        let c = mixbox::latent_to_linear_float_rgb(&self.wet.lat[i]);
        over_share(Pigment::masstone(c, self.wet.hide[i][0]), self.px[i], v, bead_cover(self.wet.cover[i], v, self.px_mm() * 1000.0))
    }

    /// What the painter sees, pixel by pixel over the window: the dry
    /// picture with any wet paint on it (at its laid thickness).
    pub fn seen(&self) -> Vec<Rgb> {
        use rayon::prelude::*;
        (0..self.px.len()).into_par_iter().map(|i| self.look_px(i)).collect()
    }

    /// What is on the canvas around (`x`, `y`) within radius `r` (units),
    /// dry picture plus wet paint, averaged in linear light: the underlayer a
    /// new mark there will sit on.
    pub fn under(&self, x: f32, y: f32, r: f32) -> Rgb {
        let f = self.f;
        let n = if r * f.scale < 1.5 { 0 } else { 2 };
        let (mut acc, mut k) = ([0.0f32; 3], 0.0f32);
        for j in -n..=n {
            for i in -n..=n {
                let (dx, dy) = (i as f32 / 2.0 * r, j as f32 / 2.0 * r);
                if n > 0 && dx * dx + dy * dy > r * r * 1.01 {
                    continue;
                }
                let (px, py) = (x + dx, y + dy);
                // (a crop render holds only its window: look where it can)
                if px < 0.0 || py < 0.0 || px >= f.width() || py >= f.height() || !f.holds(px, py) {
                    continue;
                }
                let c = self.look_px(f.index(px, py));
                for q in 0..3 {
                    acc[q] += c[q];
                }
                k += 1.0;
            }
        }
        if k == 0.0 {
            return self.look_px(f.index(x, y));
        }
        [acc[0] / k, acc[1] / k, acc[2] / k]
    }

    /// Wet paint film at a point (units), µm: 0 where the paint is dry.
    pub fn wet_um(&self, x: f32, y: f32) -> f32 {
        self.wet.vol[self.f.index(x, y)] * crate::surface::COAT_UM
    }

    /// Total wet paint on the canvas (for tests / debugging).
    pub fn wet_total(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum::<f64>() / (self.f.scale as f64 * self.f.scale as f64)
    }
}

#[cfg(test)]
mod tests {

    /// Setting a paint's hiding changes its scattering and nothing else: a
    /// slow-drying paint stays slow whichever builder comes first.
    #[test]
    fn with_hiding_keeps_the_other_fields() {
        let a = super::Paint::body([0.3; 3]).with_stiff(0.4).with_drying(0.3).with_hiding(0.5);
        let b = super::Paint::body([0.3; 3]).with_stiff(0.4).with_hiding(0.5).with_drying(0.3);
        assert_eq!((a.drying, a.stiff, a.color), (0.3, 0.4, [0.3; 3]));
        assert_eq!((a.drying, a.stiff, a.color, a.scatter), (b.drying, b.stiff, b.color, b.scatter));
        assert!((a.hiding() - 0.5).abs() < 0.01);
    }

    /// A hairline's fringe keeps its share (the floor is linear in the paint
    /// and far below a hair's film at 1000px); a sliver holding a bead many
    /// times taller than the pixel is wide spreads over the pixel.
    #[test]
    fn bead_cover_floors_only_impossible_beads() {
        // 0.02 coats on 5% of a 0.3 mm pixel: a 10 µm film, left alone
        assert_eq!(super::bead_cover(0.05, 0.02, 300.0), 0.05);
        // linear: twice the paint on twice the share is the same film
        assert_eq!(super::bead_cover(0.10, 0.04, 300.0), 0.10);
        // 4 coats (100 µm) on 10% of a 0.1 mm pixel would stand 1 mm tall
        assert_eq!(super::bead_cover(0.10, 4.0, 100.0), 1.0);
        // 1 coat on 10% of a 0.1 mm pixel: at least half of it
        assert!((super::bead_cover(0.10, 1.0, 100.0) - 0.5).abs() < 1e-6);
        assert_eq!(super::bead_cover(1.0, 9.0, 100.0), 1.0);
        assert_eq!(super::bead_cover(0.3, 0.0, 100.0), 0.3);
    }

    /// Paint over a sliver of a pixel darkens it by that sliver's share, not
    /// by a 1% floor; no share shows nothing, and a vanishing share of thick
    /// paint stays finite.
    #[test]
    fn sub_percent_cover_keeps_its_area() {
        let pig = super::Paint::body([0.01; 3]).pigment();
        let under = [1.0; 3];
        for cover in [0.001f32, 0.005, 0.02, 0.5] {
            let thick = pig.over(under, 0.1 / cover);
            let want: [f32; 3] = std::array::from_fn(|i| under[i] + (thick[i] - under[i]) * cover);
            let got = super::over_share(pig, under, 0.1, cover);
            assert!((0..3).all(|i| (got[i] - want[i]).abs() < 1e-6), "cover {cover}: {got:?} vs {want:?}");
        }
        assert_eq!(super::over_share(pig, under, 0.1, 0.0), under);
        let tiny = super::over_share(pig, under, 5.0, 1e-30);
        assert!(tiny.iter().all(|v| v.is_finite() && (v - 1.0).abs() < 1e-6), "{tiny:?}");
    }
    use crate::color::hex;
    use crate::mask::Mask;
    use crate::style::Style;

    /// How thick the stock handlings lay paint (coats), for choosing `aim`.
    #[test]
    #[ignore]
    fn probe_laid_thickness() {
        let st = Style::friedrich();
        for (name, k) in [("broad", 0), ("body", 1), ("detail", 2), ("glaze0.9", 3), ("broad load .3", 4)] {
            let mut c = st.prepare(500, 1.0, 1);
            let f0 = c.film.clone();
            let m = Mask::from_fn(c.frame(), |x, y| if (x - 500.0).abs() < 300.0 && (y - 500.0).abs() < 300.0 { 1.0 } else { 0.0 });
            let col = move |_: f32, _: f32| hex("#8a9ab0");
            let h = match k {
                0 => st.broad().color(col),
                1 => st.body().color(col),
                2 => st.detail().color(col),
                3 => st.glaze(0.9).color(col),
                _ => st.broad().color(col).load(0.3),
            };
            c.work(&m, &h, 3);
            c.dry();
            let mut d: Vec<f32> = (0..f0.len()).filter(|&i| m.data[i] > 0.5).map(|i| c.film[i] - f0[i]).collect();
            d.sort_by(|a, b| a.total_cmp(b));
            let p = |q: f32| d[((d.len() - 1) as f32 * q) as usize];
            println!("{name:14} coats p10 {:.2} p50 {:.2} p90 {:.2} mean {:.2}", p(0.1), p(0.5), p(0.9), d.iter().sum::<f32>() / d.len() as f32);
        }
    }

    /// `Paint::aimed` reaches any target made by a paint of the same hiding
    /// (the review's repro: masstone 0.8, hiding 0.92, 0.1 coats over black),
    /// across lightening targets, thin films and substrates.
    #[test]
    fn aimed_reaches_targets_made_by_the_same_model() {
        use crate::wet::Paint;
        let mut worst = (0.0f32, String::new());
        for hiding in [0.07, 0.3, 0.5, 0.92] {
            for under in [[0.0; 3], [1.0; 3], [0.2, 0.3, 0.45], [0.7, 0.5, 0.3]] {
                for coats in [0.1, 0.3, 0.6, 1.0, 2.5] {
                    for m in [[0.8; 3], [0.95, 0.9, 0.8], [0.05, 0.1, 0.3], [0.5, 0.2, 0.1], [0.3; 3]] {
                        let want = Paint::new(m, hiding, 0.5).over(under, coats);
                        let got = Paint::aimed(want, under, coats, hiding, 0.5).over(under, coats);
                        let e = (0..3).map(|c| (want[c] - got[c]).abs()).fold(0.0, f32::max);
                        if e > worst.0 {
                            worst = (e, format!("m {m:?} hiding {hiding} under {under:?} coats {coats}: want {want:?} got {got:?}"));
                        }
                    }
                }
            }
        }
        assert!(worst.0 < 2e-3, "worst miss {}: {}", worst.0, worst.1);
    }
}
