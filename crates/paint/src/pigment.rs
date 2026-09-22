//! Kubelka–Munk pigments, following Curtis et al. 1997, "Computer-Generated
//! Watercolor", section 5.1:
//! https://grail.cs.washington.edu/wp-content/uploads/2015/08/curtis-1997-cgw.pdf
//!
//! A pigment is specified by how a unit-thickness layer looks over white and
//! over black. From that we derive absorption K and scattering S per channel,
//! then composite layers of any thickness over whatever is already painted.

use crate::color::Rgb;

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
}
