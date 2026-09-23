//! Wet paint sitting on top of the dry picture.
//!
//! Every pixel holds a volume of wet paint (thickness, in "layer units": 1.0
//! is one normal coat), its pigment mixture as a Mixbox latent vector of the
//! masstone (mixing is linear in latent space, weighted by volume) and its
//! Kubelka–Munk scattering per coat (mixed linearly by volume, as K and S mix
//! in the two-constant KM model; absorption follows from masstone and S).
//! Brushes exchange paint with this layer. `Canvas::dry` bakes it into the
//! dry picture with Kubelka–Munk.
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
use crate::surface::COAT_UM;
use rayon::prelude::*;

pub const LAT: usize = mixbox::LATENT_SIZE;
pub type Latent = [f32; LAT];
/// Paint properties mixed by volume alongside the pigment: [KM scattering per
/// coat, stiffness].
/// Stiffness 0 = fluid, medium-rich glaze; 1 = stiff tube paint.
pub type Prop = [f32; 2];

#[inline]
fn lerp_prop(p: &mut Prop, q: Prop, a: f32) {
    p[0] += (q[0] - p[0]) * a;
    p[1] += (q[1] - p[1]) * a;
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
}

impl Paint {
    /// A paint of masstone `color` whose one coat hides `hiding` (contrast
    /// ratio: over black ÷ over white; 0.05 = glaze, 0.5 = scumble,
    /// 0.92 = body).
    pub fn new(color: Rgb, hiding: f32, stiff: f32) -> Self {
        Paint { color, scatter: scatter_for(luminance(color), hiding), stiff }
    }
    /// A paint of masstone `color` that scatters `scatter` per coat.
    pub fn km(color: Rgb, scatter: f32, stiff: f32) -> Self {
        Paint { color, scatter, stiff }
    }
    pub fn body(color: Rgb) -> Self {
        Paint::new(color, 0.92, 1.0)
    }
    pub fn scumble(color: Rgb) -> Self {
        Paint::new(color, 0.5, 0.6)
    }
    /// This paint with its scattering set so one coat hides `hiding`
    /// (the masstone stays).
    pub fn with_hiding(self, hiding: f32) -> Self {
        Paint::new(self.color, hiding, self.stiff)
    }
    /// This paint with stiffness `stiff`.
    pub fn with_stiff(self, stiff: f32) -> Self {
        Paint { stiff, ..self }
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
        Paint { color, scatter, stiff }
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
    /// [hiding, stiffness] of the wet paint.
    pub(crate) hide: Vec<Prop>,
    /// Which stroke last laid paint here (a stroke barely re-picks its own paint).
    pub(crate) stroke: Vec<u32>,
    /// Stroke that last touched a pixel, and the film floor that stroke may
    /// not lift below (one pass lifts only part of the film).
    pub(crate) touched: Vec<u32>,
    pub(crate) floor: Vec<f32>,
    /// Id of the stroke being painted.
    pub(crate) current: u32,
    /// Dirty bounding box in pixels (x0, y0, x1, y1), if any paint is wet.
    pub(crate) dirty: Option<(usize, usize, usize, usize)>,
}

impl Wet {
    pub fn new(n: usize) -> Self {
        Wet { vol: vec![0.0; n], lat: vec![[0.0; LAT]; n], hide: vec![[0.0, 0.5]; n], stroke: vec![0; n], touched: vec![0; n], floor: vec![0.0; n], current: 0, dirty: None }
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


impl Canvas {
    /// Let the wet paint dry: the film levels over the surface (thin fluid
    /// paint pools in the hollows, stiff paint keeps its marks), then it is
    /// composited over the dry picture with Kubelka–Munk using the settled
    /// thickness, and the wet layer is cleared.
    pub fn dry(&mut self) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty.take() else { return };
        let (w, h) = (self.f.w, self.f.h);
        let (x1, y1) = (x1.min(w), y1.min(h));
        let pad = ((2.0 / self.px_mm()).ceil() as usize).max(2);
        let ex = (x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
        let (ew, eh) = (ex.2 - ex.0, ex.3 - ex.1);
        let mut add = vec![0.0f32; ew * eh];
        let mut stiff = vec![0.5f32; ew * eh];
        for y in 0..eh {
            for x in 0..ew {
                let i = (ex.1 + y) * w + ex.0 + x;
                let v = self.wet.vol[i];
                if v >= 1e-5 {
                    add[y * ew + x] = v * COAT_UM;
                    stiff[y * ew + x] = self.wet.hide[i][1];
                }
            }
        }
        let t = self.settle(ex, &add, &stiff);
        let wet = &mut self.wet;
        let (lat, hide) = (&wet.lat, &wet.hide);
        self.px[ex.1 * w..ex.3 * w]
            .par_chunks_mut(w)
            .zip(wet.vol[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(self.film[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .enumerate()
            .for_each(|(j, ((px, vv), ff))| {
                let y = ex.1 + j;
                for x in ex.0..ex.2 {
                    if vv[x] < 1e-5 {
                        vv[x] = 0.0;
                        continue;
                    }
                    let ti = t[j * ew + x - ex.0] / COAT_UM;
                    let i = y * w + x;
                    let c = mixbox::latent_to_linear_float_rgb(&lat[i]);
                    px[x] = Pigment::masstone(c, hide[i][0]).over(px[x], ti);
                    ff[x] += ti;
                    vv[x] = 0.0;
                }
            });
    }

    /// What the painter sees at pixel `i`: the dry picture with any wet paint
    /// on it (at its laid thickness, before it levels).
    pub(crate) fn look_px(&self, i: usize) -> Rgb {
        let v = self.wet.vol[i];
        if v < 1e-5 {
            return self.px[i];
        }
        let c = mixbox::latent_to_linear_float_rgb(&self.wet.lat[i]);
        Pigment::masstone(c, self.wet.hide[i][0]).over(self.px[i], v)
    }

    /// What the painter sees, pixel by pixel over the window: the dry
    /// picture with any wet paint on it (at its laid thickness).
    pub fn seen(&self) -> Vec<Rgb> {
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

    /// Total wet paint on the canvas (for tests / debugging).
    pub fn wet_total(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum::<f64>() / (self.f.scale as f64 * self.f.scale as f64)
    }
}

#[cfg(test)]
mod tests {
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
