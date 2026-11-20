//! A print's optics: the light its paper carries sideways under the ink
//! (the press, `--@ press`; notes/research/intaglio_physics.md §6.7, **IP
//! §6.7**, with its tags).
//!
//! **Lateral light (optical dot gain, the Yule–Nielsen effect).** Light
//! enters the sheet through the ink at one place, is scattered about in the
//! paper and comes out at another, through the ink there. So a print reflects
//! R(x) = r(x) + t(x)·[P ⊗ (t·g)](x) (Gustavson 1997/1999, after Yule &
//! Nielsen 1951; Arney et al. 1996): r the ink film's own reflectance, t its
//! diffuse transmittance one way, g the paper's reflectance under it with the
//! light the film sends back down (g = q/(1 − r·q), q the paper's), and P
//! the paper's point spread function, its integral one. With P a point (no
//! lateral light) it is Kubelka's film over its substrate, which the engine
//! already lays pixel by pixel; so only the difference is added:
//! t·([P ⊗ (t·g)] − t·g).
//!
//! **The paper's spread [D].** The two-flux (Kubelka–Munk) equations are the
//! diffusion equation across the sheet: the sum of the fluxes φ obeys
//! φ'' = K(K + 2S)·φ. Lit by a pattern of wavenumber q along the sheet, the
//! diffusion equation gains q²: φ'' = (K(K + 2S) + q²)·φ. That is a sheet
//! whose absorption is K(q) = √((K + S)² + q²) − S at the same S. So the
//! paper's MTF is its reflectance at K(q) over its reflectance at K, both
//! from Kubelka–Munk, with nothing fitted: M(q) = R∞(K(q)) / R∞(K), R∞ =
//! a − √(a² − 1), a = 1 + K/S.
//! - S is the sheet's specific scattering times its density, s·(grammage /
//!   caliper): s = 30 m²/kg (`PAPER_S_M2_KG`).
//! - K/S is the paper's own, from its colour: K/S = (1 − R)²/(2R), R its
//!   internal reflectance (the lightest of each channel over the sheet: a
//!   print only darkens its paper).
//! - The sheet is taken as optically thick, lying on more paper (R∞) [E].
//!
//! It is applied on the sheet's pixels by a discrete Fourier transform (the
//! MTF at each of the grid's frequencies), over the inked region and a margin
//! where the spread reaches (`margin_px`).

use crate::color::Rgb;

/// The specific scattering coefficient of a rag printmaking sheet, m²/kg:
/// bleached chemical pulps scatter 20–40 m²/kg (ISO 9416 practice; Hrčak
/// review of Kubelka–Munk for paper) [M], filled papers above 50; cotton,
/// a lignin-free bleached fibre, is taken in the middle of the chemical
/// pulps' range [E: no measurement of a cotton sheet's s was found].
pub const PAPER_S_M2_KG: f64 = 30.0;

/// The spread is counted out to this many of its decay lengths 1/µ₀ (µ₀ =
/// S·√(a² − 1), the diffusion's own) from the ink, at most `MARGIN_MM`:
/// a numerical reach, not a physical one (e⁻⁵ of the tail is left).
const MARGIN_DECAYS: f64 = 5.0;
const MARGIN_MM: f64 = 3.0;

/// The sheet's Kubelka–Munk scattering per µm: s × its density.
pub fn sheet_s_per_um(p: &crate::paper::Paper) -> f64 {
    // (grammage g/m² over caliper µm is 10³ kg/m³ a unit)
    let density = 1e3 * p.grammage as f64 / (p.caliper_um() as f64).max(1.0);
    PAPER_S_M2_KG * density * 1e-6
}

/// R∞ of a Kubelka–Munk layer with a = 1 + K/S.
fn r_inf(a: f64) -> f64 {
    a - (a * a - 1.0).max(0.0).sqrt()
}

/// The paper's MTF at wavenumber `q` (rad/µm): R∞ at K(q) over R∞ at K,
/// `kos` its K/S and `s` its S per µm.
pub fn mtf(q: f64, s: f64, kos: f64) -> f64 {
    let a0 = 1.0 + kos;
    let a = (a0 * a0 + (q / s).powi(2)).sqrt();
    r_inf(a) / r_inf(a0)
}

/// K/S of a layer of internal reflectance `r` (Kubelka–Munk's inversion).
pub fn kos_of(r: f64) -> f64 {
    let r = r.clamp(1e-4, 0.9999);
    (1.0 - r).powi(2) / (2.0 * r)
}

/// How far (pixels) the spread is counted beyond the ink.
pub fn margin_px(s: f64, kos: [f64; 3], px_um: f64) -> usize {
    let mu = kos.iter().map(|&k| s * ((1.0 + k).powi(2) - 1.0).sqrt()).fold(f64::MAX, f64::min);
    let um = (MARGIN_DECAYS / mu.max(1e-9)).min(MARGIN_MM * 1e3);
    (um / px_um).ceil() as usize
}

/// An in-place radix-2 FFT of `re`/`im` (length a power of two); `inv`
/// for the inverse (unscaled).
fn fft(re: &mut [f64], im: &mut [f64], inv: bool) {
    let n = re.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = if inv { 2.0 } else { -2.0 } * std::f64::consts::PI / len as f64;
        let (wr, wi) = (ang.cos(), ang.sin());
        for start in (0..n).step_by(len) {
            let (mut cr, mut ci) = (1.0, 0.0);
            for k in 0..len / 2 {
                let (a, b) = (start + k, start + k + len / 2);
                let (tr, ti) = (re[b] * cr - im[b] * ci, re[b] * ci + im[b] * cr);
                re[b] = re[a] - tr;
                im[b] = im[a] - ti;
                re[a] += tr;
                im[a] += ti;
                let nr = cr * wr - ci * wi;
                ci = cr * wi + ci * wr;
                cr = nr;
            }
        }
        len <<= 1;
    }
}

/// `field` (w × h, one channel) seen through the paper's spread: P ⊗ field,
/// by the MTF `m` of the wavenumber (rad/µm) at pixels `px_um` apart. The
/// field is extended past its edges by its edge values (the sheet goes on).
fn spread(field: &[f64], w: usize, h: usize, pad: usize, px_um: f64, m: &(dyn Fn(f64) -> f64 + Sync)) -> Vec<f64> {
    use rayon::prelude::*;
    // (the field in the middle, its edges carried on `pad` either side, so
    // the transform's wrap joins only those copies, `pad` from the field)
    let (nw, nh) = ((w + 2 * pad).next_power_of_two(), (h + 2 * pad).next_power_of_two());
    let at = |i: usize, n: usize| i.saturating_sub(pad).min(n - 1);
    // rows: the field, its edges carried on
    let mut re = vec![0.0f64; nw * nh];
    let mut im = vec![0.0f64; nw * nh];
    re.par_chunks_mut(nw).zip(im.par_chunks_mut(nw)).enumerate().for_each(|(y, (r, i))| {
        let sy = at(y, h);
        for x in 0..nw {
            r[x] = field[sy * w + at(x, w)];
        }
        fft(r, i, false);
    });
    // columns, filtered, back
    let cols: Vec<(Vec<f64>, Vec<f64>)> = (0..nw).into_par_iter().map(|x| {
        let mut cr: Vec<f64> = (0..nh).map(|y| re[y * nw + x]).collect();
        let mut ci: Vec<f64> = (0..nh).map(|y| im[y * nw + x]).collect();
        fft(&mut cr, &mut ci, false);
        let fx = if x <= nw / 2 { x as f64 } else { x as f64 - nw as f64 } / (nw as f64 * px_um);
        for y in 0..nh {
            let fy = if y <= nh / 2 { y as f64 } else { y as f64 - nh as f64 } / (nh as f64 * px_um);
            let g = m(std::f64::consts::TAU * (fx * fx + fy * fy).sqrt());
            cr[y] *= g;
            ci[y] *= g;
        }
        fft(&mut cr, &mut ci, true);
        (cr, ci)
    }).collect();
    for (x, (cr, ci)) in cols.into_iter().enumerate() {
        for y in 0..nh {
            re[y * nw + x] = cr[y];
            im[y * nw + x] = ci[y];
        }
    }
    let scale = 1.0 / (nw * nh) as f64;
    re.par_chunks_mut(nw).zip(im.par_chunks_mut(nw)).skip(pad).take(h).for_each(|(r, i)| fft(r, i, true));
    (0..w * h).map(|k| re[(k / w + pad) * nw + k % w + pad] * scale).collect()
}

/// The lateral light under a print's films, added to `body` (the engine's
/// pixel-by-pixel colour of film over paper, linear RGB, specular excluded)
/// over the window `w` × `h`: `films` each pixel's film (its own internal
/// reflectance r and its transmittance t, RGB; (0, 1) where there is none),
/// `under` the colour of what lies under the films (the dry picture).
/// `s` is the sheet's S per µm (`sheet_s_per_um`) and `px_um` the pixel.
pub fn add_lateral(body: &mut [Rgb], films: &[(Rgb, Rgb)], under: &[Rgb], w: usize, h: usize, s: f64, px_um: f64) {
    use crate::mixer::{inside, outside};
    // the inked pixels' extent
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for (i, (_, t)) in films.iter().enumerate() {
        if t.iter().any(|&v| v < 1.0 - 1e-6) {
            let (x, y) = (i % w, i / w);
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x + 1);
            y1 = y1.max(y + 1);
        }
    }
    if x0 >= x1 || s <= 0.0 {
        return;
    }
    // the paper's K/S: the lightest of each channel (a print only darkens it)
    let kos: [f64; 3] = std::array::from_fn(|c| kos_of(under.iter().map(|p| inside(p[c].clamp(0.0, 1.0)) as f64).fold(0.0, f64::max)));
    let mg = margin_px(s, kos, px_um);
    let (x0, y0, x1, y1) = (x0.saturating_sub(mg), y0.saturating_sub(mg), (x1 + mg).min(w), (y1 + mg).min(h));
    let (rw, rh) = (x1 - x0, y1 - y0);
    for c in 0..3 {
        // t·g at each pixel of the region, g the paper's reflectance with the film's light sent back down
        let mut tg = vec![0.0f64; rw * rh];
        let mut tt = vec![0.0f64; rw * rh];
        for y in 0..rh {
            for x in 0..rw {
                let i = (y + y0) * w + x + x0;
                let (r, t) = (films[i].0[c] as f64, films[i].1[c] as f64);
                let q = inside(under[i][c].clamp(0.0, 1.0)) as f64;
                let g = q / (1.0 - r * q).max(1e-6);
                tg[y * rw + x] = t * g;
                tt[y * rw + x] = t;
            }
        }
        let k = kos[c];
        let sp = spread(&tg, rw, rh, mg, px_um, &move |q| mtf(q, s, k));
        for y in 0..rh {
            for x in 0..rw {
                let (j, i) = (y * rw + x, (y + y0) * w + x + x0);
                let d = tt[j] * (sp[j] - tg[j]);
                if d.abs() > 1e-7 {
                    let v = inside(body[i][c].clamp(0.0, 1.0)) as f64 + d;
                    body[i][c] = outside(v.clamp(0.0, 1.0) as f32);
                }
            }
        }
    }
}

impl crate::canvas::Canvas {
    /// Whether this canvas shows a print's optics: printed on by the press,
    /// engine 9's films.
    pub(crate) fn printed(&self) -> bool {
        self.print_optics && self.engine >= crate::PIGMENT_MIXER
    }

    /// The sheet's S per µm, if it is paper.
    fn print_s(&self) -> Option<f64> {
        self.paper.as_ref().map(sheet_s_per_um)
    }

    /// A printed sheet's colours with its wet ink on it, pixel by pixel
    /// (`look_px`), with the lateral light under the ink added; None where
    /// the canvas isn't a print or has no wet ink.
    pub(crate) fn print_bodies(&self) -> Option<Vec<Rgb>> {
        use rayon::prelude::*;
        if !self.printed() || self.wet.dirty.is_none() {
            return None;
        }
        let s = self.print_s()?;
        let px_um = self.px_mm() as f64 * 1000.0;
        let n = self.px.len();
        let mut body: Vec<Rgb> = (0..n).into_par_iter().map(|i| self.look_px(i)).collect();
        let films: Vec<(Rgb, Rgb)> = (0..n).into_par_iter().map(|i| {
            let v = self.wet.vol[i];
            if v < 1e-5 {
                return ([0.0; 3], [1.0; 3]);
            }
            let cover = crate::wet::bead_cover(self.wet.cover[i], v, px_um as f32).clamp(0.0, 1.0);
            film_share(crate::mixer::film_rt(&self.wet.lat[i], self.wet.hide[i][0], (v / cover.max(1e-6)).min(1e4)), cover)
        }).collect();
        add_lateral(&mut body, &films, &self.px, self.f.w, self.f.h, s, px_um);
        Some(body)
    }

    /// The films baking now on a printed sheet (`add` > 0 over the region
    /// `ex`, their settled thickness `t` µm and cover), alone, and the dry
    /// picture they are laid on: for `print_bake` after they are composited.
    pub(crate) fn print_films(&self, ex: (usize, usize, usize, usize), add: &[f32], t: &[f32], cover: &[f32], px_um: f32) -> Option<(Vec<(Rgb, Rgb)>, Vec<Rgb>)> {
        if !self.printed() || self.paper.is_none() {
            return None;
        }
        let (w, ew) = (self.f.w, ex.2 - ex.0);
        let mut films = vec![([0.0; 3], [1.0; 3]); self.px.len()];
        for y in ex.1..ex.3 {
            for x in ex.0..ex.2 {
                let k = (y - ex.1) * ew + x - ex.0;
                if add[k] <= 0.0 || self.wet.vol[y * w + x] < 1e-5 {
                    continue;
                }
                let i = y * w + x;
                let ti = t[k] / crate::surface::COAT_UM;
                let c = crate::wet::bead_cover(cover[k], ti, px_um).clamp(0.0, 1.0);
                films[i] = film_share(crate::mixer::film_rt(&self.wet.lat[i], self.wet.hide[i][0], (ti / c.max(1e-6)).min(1e4)), c);
            }
        }
        Some((films, self.px.clone()))
    }

    /// The lateral light under films just baked into the dry picture
    /// (`print_films`): the same as was shown while they were wet.
    pub(crate) fn print_bake(&mut self, films: &[(Rgb, Rgb)], under: &[Rgb]) {
        let Some(s) = self.print_s() else { return };
        let px_um = self.px_mm() as f64 * 1000.0;
        let (w, h) = (self.f.w, self.f.h);
        add_lateral(&mut self.px, films, under, w, h, s, px_um);
    }
}

/// A film (r, t) over `cover` of a pixel, the rest bare.
fn film_share((r, t): (Rgb, Rgb), cover: f32) -> (Rgb, Rgb) {
    (r.map(|v| v * cover), t.map(|v| 1.0 - cover * (1.0 - v)))
}

#[cfg(test)]
mod tests;
