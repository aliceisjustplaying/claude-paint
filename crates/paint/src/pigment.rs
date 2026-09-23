//! Kubelka–Munk pigments, following Curtis et al. 1997, "Computer-Generated
//! Watercolor", section 5.1:
//! https://grail.cs.washington.edu/wp-content/uploads/2015/08/curtis-1997-cgw.pdf
//!
//! A pigment layer is absorption K and scattering S per channel (per coat of
//! thickness). Two ways to name one:
//!
//! - by **masstone** and scattering (`Pigment::masstone`): the color of the
//!   paint laid thick enough to hide anything, R∞, plus how strongly it
//!   scatters. This is how wet paint is described (`Paint`, `Tube`): a coat
//!   of paint over paint of its own masstone looks the same at any thickness,
//!   so a mark matched to the field it sits in disappears into it.
//! - by **appearance** (`from_appearance`, `with_hiding`, `transparent`, …):
//!   how a unit layer looks over white and over black. Natural for glazes
//!   and varnish films named by the tint they give a white ground.
//!
//! Either way we composite layers of any thickness over whatever is already
//! painted.

use crate::color::{Rgb, luminance};

/// K/S of a paint whose masstone (infinitely thick reflectance) is `r`.
#[inline]
pub fn ks_of(r: f32) -> f32 {
    let r = r.clamp(0.002, 0.995);
    (1.0 - r) * (1.0 - r) / (2.0 * r)
}

/// Hiding of a unit coat of a paint with masstone reflectance `r` and
/// scattering `s`: its reflectance over black divided by over white (the
/// paint industry's contrast ratio). 0 = clear glaze, 1 = hides completely.
pub fn hiding_of(r: f32, s: f32) -> f32 {
    let p = Pigment { k: [s * ks_of(r); 3], s: [s; 3] };
    let b = p.over([0.0; 3], 1.0)[0];
    let w = p.over([1.0; 3], 1.0)[0];
    (b / w.max(1e-6)).clamp(0.0, 1.0)
}

/// The scattering (per coat) that gives a unit coat of masstone `r`
/// (luminance) the contrast ratio `hiding`. Inverse of `hiding_of`.
pub fn scatter_for(r: f32, hiding: f32) -> f32 {
    let h = hiding.clamp(1e-4, 0.9995);
    let (mut lo, mut hi) = (-9.0f32, 9.0f32); // ln s
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if hiding_of(r, mid.exp()) < h {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

#[derive(Clone, Copy, Debug)]
pub struct Pigment {
    pub k: Rgb,
    pub s: Rgb,
}

impl Pigment {
    /// Derive K and S from a layer's appearance over white and over black.
    pub fn from_appearance(on_white: Rgb, on_black: Rgb) -> Self {
        let mut k = [0.0; 3];
        let mut s = [0.0; 3];
        for i in 0..3 {
            let rw = on_white[i].clamp(0.004, 0.996);
            let rb = on_black[i].clamp(0.001, rw - 0.002);
            let a = 0.5 * (rw + (rb - rw + 1.0) / rb);
            let b = (a * a - 1.0).max(1e-8).sqrt();
            let z = (b * b - (a - rw) * (a - 1.0)) / (b * (1.0 - rw));
            let arcoth = 0.5 * ((z + 1.0) / (z - 1.0).max(1e-8)).ln();
            s[i] = (arcoth / b).max(0.0);
            k[i] = s[i] * (a - 1.0);
        }
        Pigment { k, s }
    }

    /// A paint of masstone `r` (the color it has laid thick, or over itself)
    /// that scatters `s` per coat, the same in every channel: scattering
    /// (by white and by particle edges) is nearly flat across the spectrum,
    /// absorption carries the hue.
    pub fn masstone(r: Rgb, s: f32) -> Self {
        let s = s.max(1e-6);
        Pigment { k: [s * ks_of(r[0]), s * ks_of(r[1]), s * ks_of(r[2])], s: [s; 3] }
    }

    /// A paint of masstone `r` with `hiding` (contrast ratio of a unit coat,
    /// measured on the luminance of the masstone).
    pub fn masstone_hiding(r: Rgb, hiding: f32) -> Self {
        Self::masstone(r, scatter_for(luminance(r), hiding))
    }

    /// The masstone R∞ of this pigment: what an infinitely thick layer looks like.
    pub fn masstone_color(&self) -> Rgb {
        std::array::from_fn(|i| {
            if self.s[i] < 1e-9 {
                return 0.0;
            }
            let a = 1.0 + self.k[i] / self.s[i];
            a - (a * a - 1.0).max(0.0).sqrt()
        })
    }

    /// A transparent glazing pigment (hides very little).
    pub fn transparent(color: Rgb) -> Self {
        Self::with_hiding(color, 0.06)
    }

    /// Aged varnish: a clear film that only absorbs (yellows), with almost
    /// no scattering, so it warms the darks without veiling them.
    pub fn varnish(color: Rgb) -> Self {
        Self::with_hiding(color, 0.004)
    }

    /// Semi-opaque (scumbles, thin body color).
    pub fn semi(color: Rgb) -> Self {
        Self::with_hiding(color, 0.45)
    }

    /// Nearly opaque body color.
    pub fn opaque(color: Rgb) -> Self {
        Self::with_hiding(color, 0.92)
    }

    /// `hiding` = fraction of the over-white appearance that survives over black.
    pub fn with_hiding(color: Rgb, hiding: f32) -> Self {
        let on_black = [color[0] * hiding, color[1] * hiding, color[2] * hiding];
        Self::from_appearance(color, on_black)
    }

    /// Reflectance and transmittance of a layer of thickness `x`.
    #[inline]
    pub fn layer(&self, x: f32) -> (Rgb, Rgb) {
        let mut r = [0.0; 3];
        let mut t = [1.0; 3];
        if x <= 0.0 {
            return ([0.0; 3], [1.0; 3]);
        }
        for i in 0..3 {
            let (k, s) = (self.k[i], self.s[i]);
            if s < 1e-6 {
                // pure absorber
                r[i] = 0.0;
                t[i] = (-k * x).exp();
                continue;
            }
            let a = 1.0 + k / s;
            let b = (a * a - 1.0).max(0.0).sqrt();
            if b < 1e-3 {
                // (nearly) non-absorbing: the b → 0 limit of the formulas below
                let sx = s * x;
                r[i] = sx / (1.0 + a * sx);
                t[i] = 1.0 / (1.0 + a * sx);
                continue;
            }
            let bsx = (b * s * x).min(40.0);
            let (sh, ch) = (bsx.sinh(), bsx.cosh());
            let c = a * sh + b * ch;
            r[i] = sh / c;
            t[i] = b / c;
        }
        (r, t)
    }

    /// Composite a layer of thickness `x` over an opaque substrate.
    #[inline]
    pub fn over(&self, sub: Rgb, x: f32) -> Rgb {
        if x <= 0.0 {
            return sub;
        }
        let (r, t) = self.layer(x);
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = r[i] + t[i] * t[i] * sub[i] / (1.0 - r[i] * sub[i]).max(1e-6);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_unit_layer() {
        let w = [0.6, 0.4, 0.2];
        let b = [0.1, 0.05, 0.02];
        let p = Pigment::from_appearance(w, b);
        let ow = p.over([1.0; 3], 1.0);
        let ob = p.over([0.0; 3], 1.0);
        for i in 0..3 {
            assert!((ow[i] - w[i]).abs() < 0.01, "white {i}: {:?} vs {:?}", ow, w);
            assert!((ob[i] - b[i]).abs() < 0.01, "black {i}: {:?} vs {:?}", ob, b);
        }
    }

    #[test]
    fn masstone_is_a_fixed_point() {
        // paint over paint of its own masstone looks the same at any thickness
        let m = [0.3, 0.2, 0.08];
        for h in [0.05, 0.3, 0.9] {
            let p = Pigment::masstone_hiding(m, h);
            for x in [0.1, 0.5, 1.0, 3.0] {
                let o = p.over(m, x);
                for i in 0..3 {
                    assert!((o[i] - m[i]).abs() < 1e-4, "h {h} x {x}: {o:?}");
                }
            }
            let r = p.masstone_color();
            for i in 0..3 {
                assert!((r[i] - m[i]).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn scatter_inverts_hiding() {
        for r in [0.03, 0.2, 0.7] {
            for h in [0.02, 0.1, 0.5, 0.92, 0.99] {
                let s = scatter_for(r, h);
                assert!((hiding_of(r, s) - h).abs() < 1e-3, "r {r} h {h}");
            }
        }
    }
}
