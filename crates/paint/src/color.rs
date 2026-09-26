//! Color helpers. All `Rgb` values are linear-light.

pub type Rgb = [f32; 3];

#[inline]
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

#[inline]
pub fn linear_to_srgb(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}

/// Parse "#rrggbb" (sRGB) into linear RGB.
pub fn hex(s: &str) -> Rgb {
    let s = s.trim_start_matches('#');
    let v = u32::from_str_radix(s, 16).expect("bad hex color");
    let f = |shift: u32| srgb_to_linear(((v >> shift) & 0xff) as f32 / 255.0);
    [f(16), f(8), f(0)]
}

/// How two colors combine.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mix {
    /// Physical pigment mixing via Mixbox (blue + yellow = green).
    Pigment,
    /// Perceptual interpolation in OKLab.
    Light,
    /// Plain linear-light interpolation.
    Linear,
}

pub fn mix(a: Rgb, b: Rgb, t: f32, mode: Mix) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.0 {
        return a;
    }
    if t >= 1.0 {
        return b;
    }
    match mode {
        Mix::Pigment => mixbox::lerp_linear_float(&a, &b, t),
        Mix::Linear => lerp3(a, b, t),
        Mix::Light => from_oklab(lerp3(to_oklab(a), to_oklab(b), t)),
    }
}

#[inline]
pub fn lerp3(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

#[inline]
pub fn scale(a: Rgb, k: f32) -> Rgb {
    [a[0] * k, a[1] * k, a[2] * k]
}

#[inline]
pub fn luminance(c: Rgb) -> f32 {
    0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]
}

/// Multi-stop gradient; `stops` are (position, color) sorted by position.
pub fn gradient(stops: &[(f32, Rgb)], t: f32, mode: Mix) -> Rgb {
    assert!(!stops.is_empty());
    if t <= stops[0].0 {
        return stops[0].1;
    }
    for w in stops.windows(2) {
        let (p0, c0) = w[0];
        let (p1, c1) = w[1];
        if t <= p1 {
            let u = if p1 > p0 { (t - p0) / (p1 - p0) } else { 1.0 };
            // smooth the transition so stops don't show as creases
            let u = u * u * (3.0 - 2.0 * u);
            return mix(c0, c1, u, mode);
        }
    }
    stops[stops.len() - 1].1
}

// OKLab (Björn Ottosson, https://bottosson.github.io/posts/oklab/)
/// `c` moved in OKLab: `dl` lighter (negative: darker), `da` toward red
/// (negative: green), `db` toward yellow (negative: blue). For colors
/// relative to another, e.g. slightly darker and bluer than `under`:
/// `shift(under, -0.06, 0.0, -0.03)`.
pub fn shift(c: Rgb, dl: f32, da: f32, db: f32) -> Rgb {
    let l = to_oklab(c);
    from_oklab([(l[0] + dl).clamp(0.0, 1.0), l[1] + da, l[2] + db])
}

pub fn to_oklab(c: Rgb) -> Rgb {
    let l = 0.4122214708 * c[0] + 0.5363325363 * c[1] + 0.0514459929 * c[2];
    let m = 0.2119034982 * c[0] + 0.6806995451 * c[1] + 0.1073969566 * c[2];
    let s = 0.0883024619 * c[0] + 0.2817188376 * c[1] + 0.6299787005 * c[2];
    let (l, m, s) = (l.max(0.0).cbrt(), m.max(0.0).cbrt(), s.max(0.0).cbrt());
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}

pub fn from_oklab(c: Rgb) -> Rgb {
    let l = c[0] + 0.3963377774 * c[1] + 0.2158037573 * c[2];
    let m = c[0] - 0.1055613458 * c[1] - 0.0638541728 * c[2];
    let s = c[0] - 0.0894841775 * c[1] - 1.2914855480 * c[2];
    let (l, m, s) = (l * l * l, m * m * m, s * s * s);
    [
        (4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s).max(0.0),
        (-1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s).max(0.0),
        (-0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s).max(0.0),
    ]
}
