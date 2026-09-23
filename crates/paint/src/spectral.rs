//! Spectral paint mixing and layering: Kubelka–Munk per wavelength.
//!
//! A port of spectral.js 3.0 (https://github.com/rvanwijnen/spectral.js,
//! commit bb2b05c9d1e65ae824d47e3b1cc17ea32c8ee68f, 2026-01-01), "a paint
//! like color mixing library utilizing the Kubelka-Munk theory", extended
//! to the engine's two-constant paint model and to layering (a film over a
//! substrate, per wavelength). See notes/spectral.md for the design, the
//! measurements against Mixbox and per-channel KM, and the verdict.
//!
//! What comes from spectral.js (the data tables and three conversions):
//! - `from_rgb`: a reflectance spectrum for a linear RGB color, 38 samples
//!   from 380 to 750 nm in 10 nm steps, built from seven basis spectra
//!   (white, cyan, magenta, yellow, red, green, blue);
//! - `to_xyz`/`to_rgb`: back through the CIE 1931 2° color matching
//!   functions weighted by D65;
//! - `ks`/`km`: K/S of a reflectance and the reflectance of a K/S;
//! - `mix_js`: spectral.js's `mix` (K/S averaged per wavelength, weighted
//!   by factor² × tinting strength² × luminance).
//!
//! What is new here: `SpectralPigment`, the engine's paint (a masstone and
//! KM scattering per coat, `Pigment::masstone`) with absorption per
//! wavelength, mixed as two-constant KM (K and S add by concentration) and
//! laid over a substrate with the same layer formulas the engine uses per
//! RGB channel (`pigment::layer1`), one wavelength at a time.
//!
//! ---------------------------------------------------------------------
//! spectral.js is used under the MIT License:
//!
//! MIT License
//!
//! Copyright (c) 2025 Ronald van Wijnen
//!
//! Permission is hereby granted, free of charge, to any person obtaining a
//! copy of this software and associated documentation files (the
//! "Software"), to deal in the Software without restriction, including
//! without limitation the rights to use, copy, modify, merge, publish,
//! distribute, sublicense, and/or sell copies of the Software, and to
//! permit persons to whom the Software is furnished to do so, subject to
//! the following conditions:
//!
//! The above copyright notice and this permission notice shall be included
//! in all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
//! OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
//! MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
//! IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//! CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
//! TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
//! SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//! ---------------------------------------------------------------------

use crate::color::Rgb;
use crate::pigment::{ks_from_appearance, layer1};

/// Number of spectral samples: 380–750 nm in 10 nm steps.
pub const N: usize = 38;
/// Reflectance (or transmittance, or K, or S) per wavelength sample.
pub type Spectrum = [f32; N];

/// Wavelength of sample `i`, nm.
pub fn wavelength(i: usize) -> f32 {
    380.0 + 10.0 * i as f32
}

/// Smallest reflectance a spectrum holds (spectral.js clamps at
/// `Number.EPSILON`; in f32 we keep K/S finite and well conditioned).
const R_MIN: f32 = 1e-6;

/// A reflectance spectrum for a linear RGB color (spectral.js `lRGB_to_R`):
/// the white part, then the secondary (cyan, magenta, yellow) and primary
/// parts, each a smooth basis spectrum. `to_rgb(&from_rgb(c))` returns `c`
/// for colors in 0..1 (to ~1e-6).
pub fn from_rgb(c: Rgb) -> Spectrum {
    let w = c[0].min(c[1]).min(c[2]);
    let l = [c[0] - w, c[1] - w, c[2] - w];
    let cy = l[1].min(l[2]);
    let ma = l[0].min(l[2]);
    let ye = l[0].min(l[1]);
    let r = (l[0] - l[2]).min(l[0] - l[1]).max(0.0);
    let g = (l[1] - l[2]).min(l[1] - l[0]).max(0.0);
    let b = (l[2] - l[1]).min(l[2] - l[0]).max(0.0);
    let wts = [w, cy, ma, ye, r, g, b].map(|v| v as f64);
    std::array::from_fn(|i| {
        let mut v = 0.0f64;
        for (k, &wk) in wts.iter().enumerate() {
            v += wk * BASE[k][i];
        }
        (v as f32).max(R_MIN)
    })
}

/// CIE XYZ (D65, Y = 1 for the perfect reflector) of a reflectance spectrum.
pub fn to_xyz(r: &Spectrum) -> [f32; 3] {
    std::array::from_fn(|k| {
        let mut v = 0.0f64;
        for i in 0..N {
            v += CMF[k][i] * r[i] as f64;
        }
        v as f32
    })
}

/// Linear sRGB of a reflectance spectrum (clamped at 0: a mixture can land
/// just outside the gamut).
pub fn to_rgb(r: &Spectrum) -> Rgb {
    let xyz = to_xyz(r).map(|v| v as f64);
    std::array::from_fn(|k| ((XYZ_RGB[k][0] * xyz[0] + XYZ_RGB[k][1] * xyz[1] + XYZ_RGB[k][2] * xyz[2]) as f32).max(0.0))
}

/// Luminance Y of a linear RGB color (spectral.js uses the XYZ Y of the
/// color's spectrum, which equals this for colors made by `from_rgb`).
fn luminance_y(c: Rgb) -> f32 {
    (RGB_XYZ[1][0] * c[0] as f64 + RGB_XYZ[1][1] * c[1] as f64 + RGB_XYZ[1][2] * c[2] as f64) as f32
}

/// K/S of a masstone reflectance: (1 − R)² / 2R.
#[inline]
pub fn ks(r: f32) -> f32 {
    let r = r.clamp(R_MIN, 1.0);
    (1.0 - r) * (1.0 - r) / (2.0 * r)
}

/// The masstone reflectance R∞ of a paint with this K/S: 1 + K/S −
/// √((K/S)² + 2 K/S), written as its reciprocal form
/// 1 / (1 + K/S + √(K/S (K/S + 2))), which is stable in f32 for deep
/// absorbers.
#[inline]
pub fn km(ks: f32) -> f32 {
    let q = ks.max(0.0);
    1.0 / (1.0 + q + (q * (q + 2.0)).sqrt())
}

/// spectral.js `mix`: colors (linear RGB) with a mixing factor and tinting
/// strength each. Per wavelength, K/S is averaged with weights
/// factor² × strength² × luminance, then turned back into a reflectance.
/// (The luminance and square weights are spectral.js's heuristics for how
/// much a color "counts" in a mix.)
pub fn mix_js(colors: &[(Rgb, f32, f32)]) -> Rgb {
    let parts: Vec<(Spectrum, f32)> = colors.iter().map(|&(c, f, ts)| (from_rgb(c), f * f * ts * ts * luminance_y(c).max(f32::EPSILON))).collect();
    let tot: f32 = parts.iter().map(|p| p.1).sum();
    let r: Spectrum = std::array::from_fn(|i| {
        let q: f32 = parts.iter().map(|(s, w)| ks(s[i]) * w).sum();
        km(q / tot.max(1e-12))
    });
    to_rgb(&r)
}

/// Single-constant spectral KM mixing with plain weights: K/S averaged per
/// wavelength with weight `w` (e.g. volume × tinting strength, the weights
/// the palette gives Mixbox). Returns the mixture's masstone.
pub fn mix_ks(colors: &[(Rgb, f32)]) -> Rgb {
    let tot: f32 = colors.iter().map(|p| p.1).sum();
    let sp: Vec<(Spectrum, f32)> = colors.iter().map(|&(c, w)| (from_rgb(c), w)).collect();
    let r: Spectrum = std::array::from_fn(|i| km(sp.iter().map(|(s, w)| ks(s[i]) * w).sum::<f32>() / tot.max(1e-12)));
    to_rgb(&r)
}

/// A paint layer with absorption and scattering per wavelength, per coat:
/// the engine's `Pigment` with a spectrum in place of three channels.
#[derive(Clone, Copy, Debug)]
pub struct SpectralPigment {
    pub k: Spectrum,
    pub s: Spectrum,
}

impl SpectralPigment {
    /// A paint whose masstone (laid thick) has reflectance spectrum `r`
    /// and which scatters `s` per coat at every wavelength
    /// (`Pigment::masstone`, per wavelength).
    pub fn from_masstone_spectrum(r: &Spectrum, s: f32) -> Self {
        let s = s.max(1e-6);
        SpectralPigment { k: std::array::from_fn(|i| s * ks(r[i])), s: [s; N] }
    }

    /// A paint of masstone `r` (linear RGB, its spectrum by `from_rgb`)
    /// that scatters `s` per coat.
    pub fn masstone(r: Rgb, s: f32) -> Self {
        Self::from_masstone_spectrum(&from_rgb(r), s)
    }

    /// A film named by its look over white and over black, per wavelength
    /// (`Pigment::from_appearance`): glazes and varnish.
    pub fn from_appearance(on_white: Rgb, on_black: Rgb) -> Self {
        let (w, b) = (from_rgb(on_white), from_rgb(on_black));
        let mut p = SpectralPigment { k: [0.0; N], s: [0.0; N] };
        for i in 0..N {
            (p.k[i], p.s[i]) = ks_from_appearance(w[i], b[i]);
        }
        p
    }

    /// `Pigment::with_hiding` per wavelength: one coat over white looks
    /// `color`, over black `color × hiding`.
    pub fn with_hiding(color: Rgb, hiding: f32) -> Self {
        Self::from_appearance(color, color.map(|v| v * hiding))
    }

    /// `Pigment::varnish` per wavelength: a clear, yellowing film.
    pub fn varnish(color: Rgb) -> Self {
        Self::with_hiding(color, 0.004)
    }

    /// Two-constant KM mixing: K and S of the parts add, weighted by
    /// concentration (`w`, e.g. volume fraction).
    pub fn mix(parts: &[(SpectralPigment, f32)]) -> Self {
        let tot: f32 = parts.iter().map(|p| p.1).sum::<f32>().max(1e-12);
        let mut m = SpectralPigment { k: [0.0; N], s: [0.0; N] };
        for (p, w) in parts {
            let a = w / tot;
            for i in 0..N {
                m.k[i] += a * p.k[i];
                m.s[i] += a * p.s[i];
            }
        }
        m
    }

    /// K and S scaled by `a` (medium dilutes the pigment: `1 − medium`).
    pub fn scaled(&self, a: f32) -> Self {
        SpectralPigment { k: self.k.map(|v| v * a), s: self.s.map(|v| v * a) }
    }

    /// Masstone spectrum R∞.
    pub fn masstone_spectrum(&self) -> Spectrum {
        std::array::from_fn(|i| if self.s[i] < 1e-9 { R_MIN } else { km(self.k[i] / self.s[i]) })
    }

    /// Masstone, linear RGB.
    pub fn masstone_color(&self) -> Rgb {
        to_rgb(&self.masstone_spectrum())
    }

    /// A film `x` coats thick over an opaque substrate of spectrum `sub`.
    pub fn over_spectrum(&self, sub: &Spectrum, x: f32) -> Spectrum {
        if x <= 0.0 {
            return *sub;
        }
        std::array::from_fn(|i| {
            let (r, t) = layer1(self.k[i], self.s[i], x);
            r + t * t * sub[i] / (1.0 - r * sub[i]).max(1e-6)
        })
    }

    /// A film `x` coats thick over a substrate of linear RGB `sub`.
    pub fn over(&self, sub: Rgb, x: f32) -> Rgb {
        to_rgb(&self.over_spectrum(&from_rgb(sub), x))
    }
}

// ---- data (spectral.js 3.0: BASE_SPECTRA, CIE.CMF, CONVERSION)

pub(crate) const BASE: [[f64; N]; 7] = [
    // W
    [1.00116072718764, 1.00116065159728, 1.00116031922747, 1.00115867270789, 1.00115259844552, 1.00113252528998, 1.00108500663327, 1.00099687889453, 1.00086525152274, 1.0006962900094, 1.00050496114888, 1.00030808187992, 1.00011966602013, 0.999952765968407, 0.999821836899297, 0.999738609557593, 0.999709551639612, 0.999731930210627, 0.999799436346195, 0.999900330316671, 1.00002040652611, 1.00014478793658, 1.00025997903412, 1.00035579697089, 1.00042753780269, 1.00047623344888, 1.00050720967508, 1.00052519156373, 1.00053509606896, 1.00054022097482, 1.00054272816784, 1.00054389569087, 1.00054448212151, 1.00054476959992, 1.00054489887762, 1.00054496254689, 1.00054498927058, 1.000544996993],
    // C
    [0.970585001322962, 0.970592498143425, 0.970625348729891, 0.970786806119017, 0.971368673228248, 0.973163230621252, 0.976740223158765, 0.981587605491377, 0.986280265652949, 0.989949147689134, 0.99249270153842, 0.994145680405256, 0.995183975033212, 0.995756750110818, 0.99591281828671, 0.995606157834528, 0.994597600961854, 0.99221571549237, 0.986236452783249, 0.967943337264541, 0.891285004244943, 0.536202477862053, 0.154108119001878, 0.0574575093228929, 0.0315349873107007, 0.0222633920086335, 0.0182022841492439, 0.016299055973264, 0.0153656239334613, 0.0149111568733976, 0.0146954339898235, 0.0145964146717719, 0.0145470156699655, 0.0145228771899495, 0.0145120341118965, 0.0145066940939832, 0.0145044507314479, 0.0145038009464639],
    // M
    [0.990673557319988, 0.990671524961979, 0.990662582353421, 0.990618107644795, 0.99045148087871, 0.989871081400204, 0.98828660875964, 0.984290692797504, 0.973934905625306, 0.941817838460145, 0.817390326195156, 0.432472805065729, 0.13845397825887, 0.0537347216940033, 0.0292174996673231, 0.021313651750859, 0.0201349530181136, 0.0241323096280662, 0.0372236145223627, 0.0760506552706601, 0.205375471942399, 0.541268903460439, 0.815841685086486, 0.912817704123976, 0.946339830166962, 0.959927696331991, 0.966260595230312, 0.969325970058424, 0.970854536721399, 0.971605066528128, 0.971962769757392, 0.972127272274509, 0.972209417745812, 0.972249577678424, 0.972267621998742, 0.97227650946215, 0.972280243306874, 0.97228132482656],
    // Y
    [0.0210523371789306, 0.0210564627517414, 0.0210746178695038, 0.0211649058448753, 0.0215027957272504, 0.0226738799041561, 0.0258235649693629, 0.0334879385639851, 0.0519069663740307, 0.100749014833473, 0.239129899706847, 0.534804312272748, 0.79780757864303, 0.911449894067384, 0.953797963004507, 0.971241615465429, 0.979303123807588, 0.983380119507575, 0.985461246567755, 0.986435046976605, 0.986738250670141, 0.986617882445032, 0.986277776758643, 0.985860592444056, 0.98547492767621, 0.985176934765558, 0.984971574014181, 0.984846303415712, 0.984775351811199, 0.984738066625265, 0.984719648311765, 0.984711023391939, 0.984706683300676, 0.984704554393091, 0.98470359630937, 0.984703124077552, 0.98470292561509, 0.984702868122795],
    // R
    [0.0315605737777207, 0.0315520718330149, 0.0315148215513658, 0.0313318044982702, 0.0306729857725527, 0.0286480476989607, 0.0246450407045709, 0.0192960753663651, 0.0142066612220556, 0.0102942608878609, 0.0076191460521811, 0.005898041083542, 0.0048233247781713, 0.0042298748350633, 0.0040599171299341, 0.0043533695594676, 0.0053434425970201, 0.0076917201010463, 0.0135969795736536, 0.0316975442661115, 0.107861196355249, 0.463812603168704, 0.847055405272011, 0.943185409393918, 0.968862150696558, 0.978030667473603, 0.982043643854306, 0.983923623718707, 0.984845484154382, 0.985294275814596, 0.985507295219825, 0.985605071539837, 0.985653849933578, 0.985677685033883, 0.985688391806122, 0.985693664690031, 0.985695879848205, 0.985696521463762],
    // G
    [0.0095560747554212, 0.0095581580120851, 0.0095673245444588, 0.0096129126297349, 0.0097837090401843, 0.010378622705871, 0.0120026452378567, 0.0160977721473922, 0.026706190223168, 0.0595555440185881, 0.186039826532826, 0.570579820116159, 0.861467768400292, 0.945879089767658, 0.970465486474305, 0.97841363028445, 0.979589031411224, 0.975533536908632, 0.962288755397813, 0.92312157451312, 0.793434018943111, 0.459270135902429, 0.185574103666303, 0.0881774959955372, 0.05436302287667, 0.0406288447060719, 0.034221520431697, 0.0311185790956966, 0.0295708898336134, 0.0288108739348928, 0.0284486271324597, 0.0282820301724731, 0.0281988376490237, 0.0281581655342037, 0.0281398910216386, 0.0281308901665811, 0.0281271086805816, 0.0281260133612096],
    // B
    [0.979404752502014, 0.97940070684313, 0.979382903470261, 0.979294364945594, 0.97896301460857, 0.977814466694043, 0.974724321133836, 0.967198482343973, 0.949079657530575, 0.900850128940977, 0.76315044546224, 0.465922171649319, 0.201263280451005, 0.0877524413419623, 0.0457176793291679, 0.0284706050521843, 0.020527176756985, 0.0165302792310211, 0.0145135107212858, 0.0136003508637687, 0.0133604258769571, 0.013548894314568, 0.0139594356366992, 0.014443425575357, 0.0148854440621406, 0.0152254296999746, 0.0154592848180209, 0.0156018026485961, 0.0156824871281936, 0.0157248764360615, 0.0157458108784121, 0.0157556123350225, 0.0157605443964911, 0.0157629637515278, 0.0157640525629106, 0.015764589232951, 0.0157648147772649, 0.0157648801149616],
];

pub(crate) const CMF: [[f64; N]; 3] = [
    [0.0000646919989576, 0.0002194098998132, 0.0011205743509343, 0.0037666134117111, 0.011880553603799, 0.0232864424191771, 0.0345594181969747, 0.0372237901162006, 0.0324183761091486, 0.021233205609381, 0.0104909907685421, 0.0032958375797931, 0.0005070351633801, 0.0009486742057141, 0.0062737180998318, 0.0168646241897775, 0.028689649025981, 0.0426748124691731, 0.0562547481311377, 0.0694703972677158, 0.0830531516998291, 0.0861260963002257, 0.0904661376847769, 0.0850038650591277, 0.0709066691074488, 0.0506288916373645, 0.035473961885264, 0.0214682102597065, 0.0125164567619117, 0.0068045816390165, 0.0034645657946526, 0.0014976097506959, 0.000769700480928, 0.0004073680581315, 0.0001690104031614, 0.0000952245150365, 0.0000490309872958, 0.0000199961492222],
    [0.000001844289444, 0.0000062053235865, 0.0000310096046799, 0.0001047483849269, 0.0003536405299538, 0.0009514714056444, 0.0022822631748318, 0.004207329043473, 0.0066887983719014, 0.0098883960193565, 0.0152494514496311, 0.0214183109449723, 0.0334229301575068, 0.0513100134918512, 0.070402083939949, 0.0878387072603517, 0.0942490536184085, 0.0979566702718931, 0.0941521856862608, 0.0867810237486753, 0.0788565338632013, 0.0635267026203555, 0.05374141675682, 0.042646064357412, 0.0316173492792708, 0.020885205921391, 0.0138601101360152, 0.0081026402038399, 0.004630102258803, 0.0024913800051319, 0.0012593033677378, 0.000541646522168, 0.0002779528920067, 0.0001471080673854, 0.0000610327472927, 0.0000343873229523, 0.0000177059860053, 0.000007220974913],
    [0.000305017147638, 0.0010368066663574, 0.0053131363323992, 0.0179543925899536, 0.0570775815345485, 0.113651618936287, 0.17335872618355, 0.196206575558657, 0.186082370706296, 0.139950475383207, 0.0891745294268649, 0.0478962113517075, 0.0281456253957952, 0.0161376622950514, 0.0077591019215214, 0.0042961483736618, 0.0020055092122156, 0.0008614711098802, 0.0003690387177652, 0.0001914287288574, 0.0001495555858975, 0.0000923109285104, 0.0000681349182337, 0.0000288263655696, 0.0000157671820553, 0.0000039406041027, 0.000001584012587, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

pub(crate) const XYZ_RGB: [[f64; 3]; 3] = [
    [3.2409699419045226, -1.537383177570094, -0.4986107602930034],
    [-0.9692436362808796, 1.8759675015077202, 0.04155505740717559],
    [0.05563007969699366, -0.20397695888897652, 1.0569715142428786],
];
pub(crate) const RGB_XYZ: [[f64; 3]; 3] = [
    [0.41239079926595934, 0.357584339383878, 0.1804807884018343],
    [0.21263900587151027, 0.715168678767756, 0.07219231536073371],
    [0.01933081871559182, 0.11919477979462598, 0.9505321522496607],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::hex;
    use crate::pigment::Pigment;

    fn close(a: Rgb, b: Rgb, tol: f32) -> bool {
        (0..3).all(|k| (a[k] - b[k]).abs() <= tol)
    }

    /// RGB → spectrum → RGB is the identity inside the gamut.
    #[test]
    fn rgb_round_trips() {
        for c in ["#5a6e9e", "#efe9dc", "#172440", "#b98a36", "#e8b21c", "#1e1b19", "#cf3a24", "#ffffff", "#000000"] {
            let c = hex(c);
            let r = to_rgb(&from_rgb(c));
            assert!(close(r, c, 2e-5), "{c:?} → {r:?}");
        }
    }

    /// `mix_js` matches spectral.js 3.0 (`spectral.mix([a, fa], [b, fb]).lRGB`,
    /// run under node at commit bb2b05c).
    #[test]
    fn matches_spectral_js() {
        let cases = [
            ("#002185", "#FCD200", 1.0, 1.0, [0.046226, 0.293265, 0.048296]),
            ("#172440", "#b98a36", 1.0, 1.0, [0.115485, 0.148143, 0.050301]),
            ("#5a6e9e", "#efe9dc", 1.0, 3.0, [0.654373, 0.696696, 0.697626]),
            ("#cf3a24", "#2f55a8", 2.0, 1.0, [0.217714, 0.055656, 0.028859]),
        ];
        for (a, b, fa, fb, want) in cases {
            let got = mix_js(&[(hex(a), fa, 1.0), (hex(b), fb, 1.0)]);
            assert!(close(got, want, 2e-4), "{a} + {b}: {got:?} vs {want:?}");
        }
    }

    /// The masstone is the fixed point of layering, as in `Pigment`, and a
    /// neutral (flat-spectrum) paint layers exactly as per-channel KM does.
    #[test]
    fn layering_agrees_with_the_engine_where_it_must() {
        let m = hex("#6a5a44");
        for s in [0.05, 0.8, 6.0] {
            let p = SpectralPigment::masstone(m, s);
            assert!(close(p.masstone_color(), m, 1e-4));
            for x in [0.1, 1.0, 4.0] {
                assert!(close(p.over(m, x), m, 1e-3), "s {s} x {x}: {:?}", p.over(m, x));
            }
        }
        for g in [0.05, 0.4, 0.8] {
            let (sp, rg) = (SpectralPigment::masstone([g; 3], 1.3), Pigment::masstone([g; 3], 1.3));
            for sub in [[0.9; 3], [0.02; 3], [0.3; 3]] {
                for x in [0.2, 1.0, 3.0] {
                    assert!(close(sp.over(sub, x), rg.over(sub, x), 2e-3), "gray {g} over {sub:?} × {x}");
                }
            }
        }
    }

    /// Mixing a paint with itself changes nothing; K/S mixing is monotone
    /// in the proportions.
    #[test]
    fn mixing_is_sane() {
        let a = SpectralPigment::masstone(hex("#5a6e9e"), 0.7);
        let m = SpectralPigment::mix(&[(a, 0.3), (a, 0.7)]);
        assert!(close(m.masstone_color(), a.masstone_color(), 1e-5));
        let w = SpectralPigment::masstone(hex("#efe9dc"), 12.0);
        let mut last = 0.0;
        for k in 0..=10 {
            let f = k as f32 / 10.0;
            let l = crate::color::luminance(SpectralPigment::mix(&[(a, 1.0 - f), (w, f)]).masstone_color());
            assert!(l >= last - 1e-5, "{f}: {l} < {last}");
            last = l;
        }
    }
}
