//! The air: sky light, clouds and receding ranges, for one sun.
//!
//! `scene` gives one world and one sun for the solids on the ground. This
//! module gives the same sun's sky and the things far off in the air:
//!
//! - **`Sky`**: a physically based sky. Single scattering by air molecules
//!   (Rayleigh) and haze (Mie) in a spherical atmosphere over a spherical
//!   Earth, with ozone absorption, integrated along each line of sight
//!   (Nishita et al. 1993; coefficients from Bruneton 2017). It needs no
//!   special cases for the things painters see: a glow round the sun that
//!   isn't centered on anything and falls off faster upward than sideways,
//!   the sky paler and brighter at the horizon than overhead, a darker sky
//!   opposite the sun, reddening at low sun and, with the sun below the
//!   horizon, the Earth's shadow rising opposite the sun with the Belt of
//!   Venus above it and the afterglow over the sun's place. Haze can be
//!   uneven from place to place (`uneven`) and can lie in layers
//!   (`layer`). `overcast` blends toward the CIE standard overcast sky.
//! - **`SkyField`**: that sky seen through a `World`'s camera, sampled on a
//!   grid over the canvas and brought into paint's range (a painter's
//!   exposure: the brightest ordinary sky maps to a light paint, the sun's
//!   own neighborhood saturates): `at(x, y)` color, `value(x, y)`.
//! - **`Cloud`**: cloud volumes in world meters (cumulus heaps, banks,
//!   stratus decks with breaks), density fields made from the noise
//!   toolkit. **`Clouds::field`** marches them through the camera and lights
//!   them with the one sun (Beer–Lambert toward the sun, a two-lobe
//!   Henyey–Greenstein phase for the silver lining against the sun, sky
//!   light from above that the cloud itself shades, the Earth's shadow for a
//!   set sun) and returns fields: `alpha` (how much of the sky it hides),
//!   `lit` (0 in the shadowed belly, 1 on the lit edge), `glow` (light sent
//!   toward the eye, strong on thin edges against the sun), `soft` (how
//!   soft the edge is, in units), `dist` and a composed `color`.
//! - **`Ranges`**: receding mountain ranges in world meters that don't read
//!   as waves: layers at uneven distances, running obliquely (so each
//!   one's distance and haze change along it), each built from a few
//!   masses of different kinds (peaks, domes, plateaus, saddles, cliffs)
//!   that overlap and end, lowered by the Earth's curvature, and hazed by a
//!   stratified atmosphere (the foot of a range is hazier than its crest,
//!   mist lying in the valleys with an uneven top). A layer gives its crest
//!   on the canvas, its haze at any point and a `form::Ridge` to model its
//!   face.
//!
//! Like `form` and `scene`, none of this paints. Colors are linear RGB, as
//! everywhere in the engine. World coordinates are `scene`'s: meters, X
//! right, Y up, Z away, the eye at (0, eye, 0).

use crate::canvas::Frame;
use crate::color::Rgb;
use crate::form::{Ridge, V3};
use crate::mask::Mask;
use crate::noise::{Fbm, Octaves, Warp, Worley, rand01, uneven, vary};
use crate::scene::{Sun, World};
use crate::smoothstep;
use rayon::prelude::*;
use std::f32::consts::PI;

// ------------------------------------------------------------ constants

/// Earth's radius and the top of the atmosphere (m) (Bruneton 2017).
pub const EARTH_R: f32 = 6_360_000.0;
const TOP_R: f32 = 6_420_000.0;
/// Rayleigh scattering at sea level, per m, for R, G, B sampled at the
/// dominant wavelengths of the sRGB primaries (611, 549, 464 nm): Bruneton's
/// 680/550/440 nm values scaled by λ⁻⁴. (At 680/440 nm a low sun comes out
/// too red and twilight too purple: the red channel then misses both the
/// Rayleigh loss and the ozone Chappuis band, which peaks near 600 nm.)
const RAYLEIGH: Rgb = [8.90e-6, 13.66e-6, 26.76e-6];
const RAYLEIGH_H: f32 = 8000.0;
/// Mie (haze) scattering at sea level, per m; extinction is scattering / 0.9.
const MIE: f32 = 3.996e-6;
const MIE_H: f32 = 1200.0;
/// Ozone absorption (Chappuis bands) per m at the layer's peak, at the
/// same wavelengths: Bruneton's 550 nm value (1.881e-6) scaled by the
/// band's shape (≈1.5× at 611 nm near the peak, ≈0.07× at 464 nm).
const OZONE: Rgb = [2.8e-6, 1.881e-6, 0.14e-6];

#[inline]
fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline]
fn len(a: V3) -> f32 {
    dot(a, a).sqrt()
}
#[inline]
fn add(a: V3, b: V3, t: f32) -> V3 {
    [a[0] + b[0] * t, a[1] + b[1] * t, a[2] + b[2] * t]
}
#[inline]
fn lum(c: Rgb) -> f32 {
    0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]
}
#[inline]
fn scale(c: Rgb, k: f32) -> Rgb {
    [c[0] * k, c[1] * k, c[2] * k]
}
#[inline]
fn plus(a: Rgb, b: Rgb) -> Rgb {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
#[inline]
fn times(a: Rgb, b: Rgb) -> Rgb {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}
#[inline]
fn exp3(t: Rgb) -> Rgb {
    [(-t[0]).exp(), (-t[1]).exp(), (-t[2]).exp()]
}

/// Where a ray from `o` along unit `d` meets a sphere of radius `r` about
/// the origin: (near, far) distances, None if it misses.
fn sphere(o: V3, d: V3, r: f32) -> Option<(f32, f32)> {
    let b = dot(o, d);
    let c = dot(o, o) - r * r;
    let disc = b * b - c;
    if disc < 0.0 {
        return None;
    }
    let s = disc.sqrt();
    Some((-b - s, -b + s))
}

/// Rayleigh phase function.
#[inline]
fn phase_r(mu: f32) -> f32 {
    3.0 / (16.0 * PI) * (1.0 + mu * mu)
}
/// Cornette–Shanks phase function for haze (g ≈ 0.8).
#[inline]
fn phase_m(mu: f32, g: f32) -> f32 {
    let g2 = g * g;
    3.0 / (8.0 * PI) * ((1.0 - g2) * (1.0 + mu * mu)) / ((2.0 + g2) * (1.0 + g2 - 2.0 * g * mu).max(1e-4).powf(1.5))
}
/// Henyey–Greenstein.
#[inline]
fn hg(mu: f32, g: f32) -> f32 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * PI * (1.0 + g2 - 2.0 * g * mu).max(1e-4).powf(1.5))
}

// ------------------------------------------------------------------ sky

/// A layer of haze at some height (a smoke or dust layer, an inversion's
/// lid): extra haze between `alt ± thick/2` meters, varying across the
/// country by ±`uneven`.
#[derive(Clone, Copy)]
pub struct Layer {
    pub alt: f32,
    pub thick: f32,
    /// Haze density in units of the sea-level haze (`MIE`).
    pub density: f32,
    pub uneven: f32,
    noise: Fbm,
}

/// The physical sky for one sun. See the module docs.
#[derive(Clone)]
pub struct Sky {
    pub sun: Sun,
    /// Haze (aerosol) amount: 1 is Bruneton's clear reference atmosphere,
    /// 2–4 a hazy summer day, 6+ very hazy.
    pub haze: f32,
    /// Haze anisotropy (Cornette–Shanks g): 0.76 average, 0.85 big droplets.
    pub g: f32,
    /// How much the haze density wanders from place to place (0..1), and
    /// over what distance (m).
    pub uneven: f32,
    uneven_noise: Warp,
    uneven_fbm: Fbm,
    pub layers: Vec<Layer>,
    /// Multiple scattering, a fill (0..1): lifts the shadowed parts of the
    /// sky (the Earth's shadow, deep twilight) that single scattering
    /// leaves black.
    pub fill: f32,
    /// Overcast: 0 clear, 1 the CIE standard overcast sky.
    pub overcast: f32,
    /// Altitude of the eye above sea level (m).
    pub alt: f32,
    /// The dome's mean single-scattered radiance, cached with the
    /// parameters it was computed from (the fields are public: a painter
    /// may change `sun` or `haze` after sampling, and the cache follows).
    dome: DomeCache,
}

/// The dome radiance cache: the first value lock-free, a later one (after
/// the parameters changed) behind a lock. Keyed by `Sky::dome_key`.
#[derive(Default)]
struct DomeCache {
    first: std::sync::OnceLock<(u64, Rgb)>,
    later: std::sync::Mutex<Option<(u64, Rgb)>>,
}

impl Clone for DomeCache {
    fn clone(&self) -> Self {
        let later = *self.later.lock().unwrap_or_else(|e| e.into_inner());
        DomeCache { first: self.first.clone(), later: std::sync::Mutex::new(later) }
    }
}

impl Sky {
    /// A clear sky for `sun` over the sea, with a little haze.
    pub fn new(sun: Sun) -> Self {
        Sky {
            sun,
            haze: 1.5,
            g: 0.76,
            uneven: 0.0,
            uneven_noise: Warp::new(1, 40_000.0, 15_000.0),
            uneven_fbm: Fbm::new(1, 3, 60_000.0),
            layers: vec![],
            fill: 0.35,
            overcast: 0.0,
            alt: 2.0,
            dome: DomeCache::default(),
        }
    }
    /// Haze amount (1 clear, 3 hazy, 6 very hazy).
    pub fn haze(mut self, h: f32) -> Self {
        self.dome = DomeCache::default();
        self.haze = h.max(0.0);
        self
    }
    /// Haze that isn't the same everywhere: density varies by ±`amount`
    /// (0..1) over features about `period` meters across. The glow round
    /// the sun and the brightness along the horizon become uneven.
    pub fn uneven(mut self, amount: f32, period: f32, seed: u32) -> Self {
        self.dome = DomeCache::default();
        self.uneven = amount.clamp(0.0, 1.0);
        self.uneven_noise = Warp::new(seed, period * 0.7, period * 0.4);
        self.uneven_fbm = Fbm::new(seed + 1, 3, period);
        self
    }
    /// A layer of haze at `alt` m, `thick` m deep, `density` times the
    /// sea-level haze, varying by ±`uneven` across the country.
    pub fn layer(mut self, alt: f32, thick: f32, density: f32, uneven: f32, seed: u32) -> Self {
        self.dome = DomeCache::default();
        self.layers.push(Layer { alt, thick, density, uneven, noise: Fbm::new(seed + 77, 3, 25_000.0) });
        self
    }
    pub fn fill(mut self, f: f32) -> Self {
        self.fill = f;
        self
    }
    /// Blend toward the CIE standard overcast sky (0..1).
    pub fn overcast(mut self, o: f32) -> Self {
        self.overcast = o.clamp(0.0, 1.0);
        self
    }
    /// Eye altitude above sea level (m), for mountain views.
    pub fn altitude(mut self, m: f32) -> Self {
        self.dome = DomeCache::default();
        self.alt = m.max(0.5);
        self
    }

    /// Unit vector toward the sun (world).
    pub fn sun_dir(&self) -> V3 {
        self.sun.dir()
    }

    /// World position (meters, X right, Y up from sea level, Z away) to the
    /// planet frame (origin at the Earth's center).
    #[inline]
    fn planet(&self, w: V3) -> V3 {
        [w[0], EARTH_R + w[1], w[2]]
    }

    /// Haze density (in units of MIE) at a planet-frame point.
    fn mie_density(&self, p: V3, h: f32) -> f32 {
        let mut m = (-h / MIE_H).exp();
        if self.uneven > 0.0 {
            let (u, v) = self.uneven_noise.at(p[0], p[2]);
            m *= (1.0 + self.uneven * 1.4 * self.uneven_fbm.get(u, v)).max(0.05);
        }
        for l in &self.layers {
            let d = ((h - l.alt) / (0.5 * l.thick)).abs();
            if d < 1.0 {
                let bump = 1.0 - d * d;
                m += l.density * bump * (1.0 + l.uneven * 1.5 * l.noise.get(p[0], p[2])).max(0.0);
            }
        }
        m * self.haze
    }

    /// Extinction per meter (R, G, B) and the scattering densities at a
    /// planet-frame point: (extinction, rayleigh density, mie density).
    #[inline]
    fn medium(&self, p: V3) -> (Rgb, f32, f32) {
        let h = len(p) - EARTH_R;
        let r = (-h / RAYLEIGH_H).exp();
        let m = self.mie_density(p, h);
        let o = (1.0 - ((h - 25_000.0) / 15_000.0).abs()).max(0.0);
        let me = MIE / 0.9 * m;
        ([RAYLEIGH[0] * r + me + OZONE[0] * o, RAYLEIGH[1] * r + me + OZONE[1] * o, RAYLEIGH[2] * r + me + OZONE[2] * o], r, m)
    }

    /// Optical depth from a planet-frame point toward the sun (or any
    /// direction) to the top of the atmosphere; None if the Earth is in
    /// the way (the point is in the Earth's shadow).
    fn depth_to_top(&self, p: V3, d: V3, steps: usize) -> Option<Rgb> {
        if let Some((a, _)) = sphere(p, d, EARTH_R)
            && a > 0.0
        {
            return None;
        }
        let (_, far) = sphere(p, d, TOP_R)?;
        let far = far.max(0.0);
        let mut tau = [0.0f32; 3];
        let n = steps as f32;
        for i in 0..steps {
            // denser samples near the start, where the air is thick
            let (a, b) = ((i as f32 / n).powi(2), ((i + 1) as f32 / n).powi(2));
            let t = far * 0.5 * (a + b);
            let dt = far * (b - a);
            let (e, _, _) = self.medium(add(p, d, t));
            for c in 0..3 {
                tau[c] += e[c] * dt;
            }
        }
        Some(tau)
    }

    /// The color of sunlight arriving at a world point (m): white above
    /// the atmosphere, reddened by the air it crosses, black in the
    /// Earth's shadow.
    pub fn sunlight_at(&self, w: V3) -> Rgb {
        let p = self.planet(w);
        match self.depth_to_top(p, self.sun.dir(), 12) {
            Some(t) => exp3(t),
            None => [0.0; 3],
        }
    }

    /// The color of the sunlight where the painter stands.
    pub fn sunlight(&self) -> Rgb {
        self.sunlight_at([0.0, self.alt, 0.0])
    }

    /// Radiance of the sky seen along a world direction `d` (unit), for a
    /// sun of irradiance 1 in each channel. Below the horizon it is the air
    /// between the eye and the ground or sea (not the ground itself).
    pub fn radiance(&self, d: V3) -> Rgb {
        let clear = self.clear(d);
        if self.overcast <= 0.0 {
            return clear;
        }
        // CIE standard overcast (Moon & Spencer 1942): L/Lz = (1 + 2 sin e) / 3,
        // lit by the day's light (its color is the sunlight's)
        let e = d[1].clamp(0.0, 1.0);
        let day = self.clear([0.0, 1.0, 0.0]);
        let sl = self.sunlight();
        let z = 2.2 * lum(day).max(1e-6) + 0.02 * lum(sl) * self.sun.dir()[1].max(0.0);
        let k = z * (1.0 + 2.0 * e) / 3.0;
        let tint = scale(plus(scale(sl, 0.7), [0.3, 0.3, 0.3]), 1.0 / (0.7 * lum(sl) + 0.3).max(1e-4));
        let oc = scale(tint, k);
        let o = self.overcast;
        [clear[0] * (1.0 - o) + oc[0] * o, clear[1] * (1.0 - o) + oc[1] * o, clear[2] * (1.0 - o) + oc[2] * o]
    }

    /// The mean radiance of the sky dome by single scattering alone
    /// (cosine-weighted over the upper hemisphere): the light that the
    /// sky itself sheds on the air, which is what lights the Earth's
    /// shadow and the deep-twilight sky.
    pub fn dome(&self) -> Rgb {
        let key = self.dome_key();
        let (k0, v0) = *self.dome.first.get_or_init(|| (key, self.dome_now()));
        if k0 == key {
            return v0;
        }
        let mut later = self.dome.later.lock().unwrap_or_else(|e| e.into_inner());
        match *later {
            Some((k, v)) if k == key => v,
            _ => {
                let v = self.dome_now();
                *later = Some((key, v));
                v
            }
        }
    }

    /// What the dome radiance depends on, hashed: every public parameter
    /// single scattering reads (the noise fields are private and set only
    /// by builders, which reset the cache).
    fn dome_key(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |x: f32| {
            h ^= x.to_bits() as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        };
        for x in [self.sun.azimuth, self.sun.elevation, self.haze, self.g, self.uneven, self.alt] {
            eat(x);
        }
        eat(self.layers.len() as f32);
        for l in &self.layers {
            for x in [l.alt, l.thick, l.density, l.uneven] {
                eat(x);
            }
        }
        h
    }

    fn dome_now(&self) -> Rgb {
        let mut sum = [0.0f32; 3];
        let mut w = 0.0;
        for (el, n) in [(80.0f32, 1usize), (50.0, 8), (22.0, 12), (6.0, 12)] {
            let e = el.to_radians();
            for i in 0..n {
                let az = i as f32 / n as f32 * std::f32::consts::TAU + 0.3;
                let d = [az.sin() * e.cos(), e.sin(), az.cos() * e.cos()];
                let c = self.single(d, None);
                let k = e.sin() * e.cos().max(0.2);
                sum = plus(sum, scale(c, k));
                w += k;
            }
        }
        scale(sum, 1.0 / w)
    }

    /// Single scattering along one line of sight (plus the fill).
    fn clear(&self, d: V3) -> Rgb {
        let fill = if self.fill > 0.0 { Some(scale(self.dome(), self.fill)) } else { None };
        self.single(d, fill)
    }

    /// Single scattering of sunlight along `d`, plus, if given, the
    /// in-scattering of an isotropic fill radiance (light scattered more
    /// than once: it keeps a little light and the sky's color where the
    /// Earth shades the direct beam).
    fn single(&self, d: V3, fill: Option<Rgb>) -> Rgb {
        let o = self.planet([0.0, self.alt, 0.0]);
        let Some((_, top)) = sphere(o, d, TOP_R) else { return [0.0; 3] };
        // stop at the ground (the sea) for rays below the horizon
        let end = match sphere(o, d, EARTH_R) {
            Some((a, _)) if a > 0.0 => a,
            _ => top,
        };
        let l = self.sun.dir();
        let mu = dot(d, l);
        let (pr, pm) = (phase_r(mu), phase_m(mu, self.g));
        let n = 32;
        let mut tau = [0.0f32; 3];
        let mut sum = [0.0f32; 3];
        for i in 0..n {
            let (a, b) = ((i as f32 / n as f32).powi(2), ((i + 1) as f32 / n as f32).powi(2));
            let t = end * 0.5 * (a + b);
            let dt = end * (b - a);
            let p = add(o, d, t);
            let (e, r, m) = self.medium(p);
            let half = [tau[0] + 0.5 * e[0] * dt, tau[1] + 0.5 * e[1] * dt, tau[2] + 0.5 * e[2] * dt];
            let direct = self.depth_to_top(p, l, 8);
            for c in 0..3 {
                let rs = RAYLEIGH[c] * r;
                let ms = MIE * m;
                let mut s = 0.0;
                if let Some(ts) = direct {
                    s += (rs * pr + ms * pm) * (-(half[c] + ts[c])).exp();
                }
                if let Some(f) = fill {
                    s += (rs + ms) * f[c] * (-half[c]).exp();
                }
                sum[c] += s * dt;
                tau[c] += e[c] * dt;
            }
        }
        sum
    }

    /// Optical transmittance of the air from the eye along `d` over `dist`
    /// meters (for things seen through it).
    pub fn transmittance(&self, d: V3, dist: f32) -> Rgb {
        let o = self.planet([0.0, self.alt, 0.0]);
        let n = 16;
        let mut tau = [0.0f32; 3];
        for i in 0..n {
            let (a, b) = ((i as f32 / n as f32).powi(2), ((i + 1) as f32 / n as f32).powi(2));
            let t = dist * 0.5 * (a + b);
            let (e, _, _) = self.medium(add(o, d, t));
            for c in 0..3 {
                tau[c] += e[c] * dist * (b - a);
            }
        }
        exp3(tau)
    }
}

// ------------------------------------------------------------ sky field

/// A tone map from radiance to paint: the painter's exposure. `white` is
/// the radiance that maps to the lightest paint (`top`); brighter light
/// rolls off (the sun's neighborhood saturates instead of clipping hard).
#[derive(Clone, Copy, Debug)]
pub struct Exposure {
    pub white: f32,
    pub top: f32,
    /// How much a saturated light desaturates toward white (0..1).
    pub bleach: f32,
    /// White balance: radiance of this color counts as neutral (the eye
    /// adapted to the day's light). `[1, 1, 1]` is sunlight above the air.
    pub balance: Rgb,
}

impl Exposure {
    pub fn map(&self, c: Rgb) -> Rgb {
        let b = self.balance;
        let bl = lum(b).max(1e-6);
        let c = [c[0] * bl / b[0].max(1e-6), c[1] * bl / b[1].max(1e-6), c[2] * bl / b[2].max(1e-6)];
        let l = lum(c).max(1e-12);
        let v = l / self.white;
        // linear below 0.6 of white, then a shoulder toward 1
        let m = if v < 0.6 { v } else { 0.6 + 0.4 * (1.0 - (-(v - 0.6) / 0.4).exp()) };
        let mut o = scale(c, m * self.top / l);
        // over-bright colors fade to white as they saturate
        let over = smoothstep(0.7, 1.6, v) * self.bleach;
        let t = lum(o);
        for ch in o.iter_mut() {
            *ch = (*ch + (t - *ch) * over).clamp(0.0, self.top);
        }
        o
    }
}

/// A sky sampled over a camera's view, in paint's range.
pub struct SkyField {
    /// The sky model (for other directions).
    pub sky: Sky,
    pub exposure: Exposure,
    cx: f32,
    horizon: f32,
    focal: f32,
    x0: f32,
    y0: f32,
    cell: f32,
    nx: usize,
    ny: usize,
    rad: Vec<Rgb>,
}

impl SkyField {
    /// Sample `sky` over `world`'s view (above the horizon and the
    /// mirrored sky below it, for water), every `cell` units (8 is plenty:
    /// the sky is smooth; its unevenness is broad). The exposure is set so
    /// the 97th percentile of the sky's luminance in view (the sun's own
    /// glare excluded) is a light paint; change it with `exposure`.
    pub fn new(sky: Sky, world: &World, cell: f32) -> Self {
        let v = world.view;
        let (x0, y0) = (v[0] - cell, v[1] - cell);
        let (nx, ny) = (((v[2] + 2.0 * cell) / cell).ceil() as usize + 1, ((v[3] + 2.0 * cell) / cell).ceil() as usize + 1);
        let (cx, horizon, focal) = (world.cx, world.horizon, world.focal);
        let rad: Vec<Rgb> = (0..nx * ny)
            .into_par_iter()
            .map(|i| {
                let (x, y) = (x0 + (i % nx) as f32 * cell, y0 + (i / nx) as f32 * cell);
                // below the horizon: the sky that calm water mirrors there
                let yy = if y > horizon { 2.0 * horizon - y } else { y };
                let d = crate::form::unit([(x - cx) / focal, -(yy - horizon) / focal + 1e-4, 1.0]);
                sky.radiance(d)
            })
            .collect();
        let mut ls: Vec<f32> = rad.iter().map(|&c| lum(c)).collect();
        ls.sort_by(|a, b| a.total_cmp(b));
        let white = ls[((ls.len() as f32 * 0.97) as usize).min(ls.len() - 1)].max(1e-9);
        SkyField { sky, exposure: Exposure { white, top: 0.86, bleach: 0.6, balance: [1.0; 3] }, cx, horizon, focal, x0, y0, cell, nx, ny, rad }
    }
    /// Scale the exposure: > 1 brighter, < 1 darker (a dusk sky painted
    /// darker than the day's).
    pub fn exposure(mut self, k: f32) -> Self {
        self.exposure.white /= k.max(1e-6);
        self
    }
    /// White balance: part of the way (0..1) toward treating `light` (e.g.
    /// `sky.sunlight()`, or a sunlight at some elevation) as neutral. A
    /// painter's eye adapts to the day's light; 0.3–0.5 keeps a sunset
    /// warm without turning everything orange.
    pub fn balance(mut self, light: Rgb, amount: f32) -> Self {
        let l = lum(light).max(1e-6);
        let n = scale(light, 1.0 / l);
        self.exposure.balance = [1.0 + (n[0] - 1.0) * amount, 1.0 + (n[1] - 1.0) * amount, 1.0 + (n[2] - 1.0) * amount];
        self
    }
    /// The radiance (unmapped) at a canvas point, bilinear.
    pub fn radiance(&self, x: f32, y: f32) -> Rgb {
        let u = ((x - self.x0) / self.cell).clamp(0.0, (self.nx - 1) as f32 - 1e-3);
        let v = ((y - self.y0) / self.cell).clamp(0.0, (self.ny - 1) as f32 - 1e-3);
        let (i, j) = (u as usize, v as usize);
        let (fu, fv) = (u - i as f32, v - j as f32);
        let at = |i: usize, j: usize| self.rad[j * self.nx + i];
        let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
        let mut o = [0.0; 3];
        for k in 0..3 {
            o[k] = (a[k] * (1.0 - fu) + b[k] * fu) * (1.0 - fv) + (c[k] * (1.0 - fu) + d[k] * fu) * fv;
        }
        o
    }
    /// The sky's color at a canvas point, as paint (linear RGB, 0..0.86).
    /// Below the horizon: the sky that still water mirrors there.
    pub fn at(&self, x: f32, y: f32) -> Rgb {
        self.exposure.map(self.radiance(x, y))
    }
    /// The sky's value (luminance of `at`), 0..~0.86.
    pub fn value(&self, x: f32, y: f32) -> f32 {
        lum(self.at(x, y))
    }
    /// Radiance mapped to paint with this sky's exposure.
    pub fn paint(&self, rad: Rgb) -> Rgb {
        self.exposure.map(rad)
    }
    /// The world direction through a canvas point.
    pub fn dir(&self, x: f32, y: f32) -> V3 {
        crate::form::unit([(x - self.cx) / self.focal, -(y - self.horizon) / self.focal, 1.0])
    }
    /// The airlight: what a thick enough layer of air in front of anything
    /// at canvas x looks like (the sky just above the horizon there). Use
    /// it as the haze color for distant things.
    pub fn airlight(&self, x: f32) -> Rgb {
        self.at(x, self.horizon - 0.004 * self.focal)
    }
}

// --------------------------------------------------------------- clouds

/// What kind of cloud a `Cloud` is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CloudKind {
    /// A heaped cloud: a flat base, a domed top built of rounded towers.
    Cumulus,
    /// A long heaped mass lying along the horizon.
    Bank,
    /// A layer (a deck or a sheet) with breaks.
    Stratus,
}

/// One cloud volume in world meters. Build with `cumulus`, `bank` or
/// `stratus`; tune with `density`, `soft`, `wind`, `breaks`.
#[derive(Clone, Copy, Debug)]
pub struct Cloud {
    pub kind: CloudKind,
    /// Bounds (m): X, Y (base, top), Z.
    pub lo: V3,
    pub hi: V3,
    /// Extinction per meter at full density (0.03–0.1 for real clouds).
    pub density: f32,
    /// Edge softness: 0.05 a hard cauliflower edge, 0.4 a fibrous, soft one.
    pub soft: f32,
    /// Stratus cover (0..1).
    pub cover: f32,
    /// Wind direction (radians on the ground: 0 along X) and stretch
    /// (how many times longer features are along the wind).
    pub wind: (f32, f32),
    /// Breaks in a stratus deck: (period m, how much 0..1).
    pub breaks: Option<(f32, f32)>,
    seed: u32,
    /// Heap scale (m): the size of the rounded towers.
    pub heap: f32,
}

impl Cloud {
    /// A cumulus at (x, z) m, its base `base` m up, `width` m across and
    /// `height` m tall.
    pub fn cumulus(x: f32, z: f32, base: f32, width: f32, height: f32, seed: u32) -> Self {
        let r = width * 0.5;
        Cloud {
            kind: CloudKind::Cumulus,
            lo: [x - r * 1.1, base, z - r * 0.9],
            hi: [x + r * 1.1, base + height * 1.1, z + r * 0.9],
            density: 0.05,
            soft: 0.12,
            cover: 1.0,
            wind: (0.0, 1.0),
            breaks: None,
            seed,
            heap: height * 0.35,
        }
    }
    /// A bank of heaped cloud from `x0` to `x1` m, at `z` to `z + depth` m
    /// away, from `base` to `top` m up (its top rises and falls along it).
    #[allow(clippy::too_many_arguments)]
    pub fn bank(x0: f32, x1: f32, z: f32, depth: f32, base: f32, top: f32, seed: u32) -> Self {
        Cloud {
            kind: CloudKind::Bank,
            lo: [x0, base, z],
            hi: [x1, top, z + depth],
            density: 0.04,
            soft: 0.15,
            cover: 1.0,
            wind: (0.0, 1.0),
            breaks: None,
            seed,
            heap: (top - base) * 0.4,
        }
    }
    /// A layer from `base` to `base + thick` m up covering `cover` (0..1) of
    /// the sky, out to 60 km; wisps and holes from warped, wind-stretched
    /// noise.
    pub fn stratus(base: f32, thick: f32, cover: f32, seed: u32) -> Self {
        Cloud {
            kind: CloudKind::Stratus,
            lo: [-80_000.0, base, 300.0],
            hi: [80_000.0, base + thick, 60_000.0],
            density: 0.02,
            soft: 0.25,
            cover,
            wind: (0.3, 3.0),
            breaks: None,
            seed,
            heap: 2500.0,
        }
    }
    pub fn density(mut self, d: f32) -> Self {
        self.density = d;
        self
    }
    pub fn soft(mut self, s: f32) -> Self {
        self.soft = s.max(0.01);
        self
    }
    pub fn wind(mut self, angle: f32, stretch: f32) -> Self {
        self.wind = (angle, stretch.max(1.0));
        self
    }
    /// Breaks in a deck: a fraction `amount` (0..1) of the Worley cells
    /// `period` m across opens a hole of its own size (0.35–0.75 of a cell).
    pub fn breaks(mut self, period: f32, amount: f32) -> Self {
        self.breaks = Some((period, amount));
        self
    }
    /// The size (m) of the rounded heaps.
    pub fn heap(mut self, m: f32) -> Self {
        self.heap = m;
        self
    }
    /// Only out to `z` m (a stratus deck that ends).
    pub fn reach(mut self, near: f32, far: f32) -> Self {
        self.lo[2] = near;
        self.hi[2] = far;
        self
    }

    /// Where the ray from `o` along `d` is inside the bounds.
    fn span(&self, o: V3, d: V3) -> Option<(f32, f32)> {
        let (mut t0, mut t1) = (0.0f32, f32::MAX);
        for a in 0..3 {
            if d[a].abs() < 1e-9 {
                if o[a] < self.lo[a] || o[a] > self.hi[a] {
                    return None;
                }
                continue;
            }
            let (mut a0, mut a1) = ((self.lo[a] - o[a]) / d[a], (self.hi[a] - o[a]) / d[a]);
            if a0 > a1 {
                std::mem::swap(&mut a0, &mut a1);
            }
            t0 = t0.max(a0);
            t1 = t1.min(a1);
        }
        (t1 > t0).then_some((t0, t1))
    }

    /// Density (extinction per meter) at a world point; `lod` is the size
    /// (m) of the smallest detail worth resolving there.
    pub fn at(&self, p: V3, lod: f32) -> f32 {
        if p[0] < self.lo[0] || p[1] < self.lo[1] || p[2] < self.lo[2] || p[0] > self.hi[0] || p[1] > self.hi[1] || p[2] > self.hi[2] {
            return 0.0;
        }
        let s = self.seed;
        let heap = self.heap.max(1.0);
        let billow = Octaves::billow(s + 1, 5, heap);
        let h = self.hi[1] - self.lo[1];
        let y = (p[1] - self.lo[1]) / h; // 0 base .. 1 top of the bounds
        let inside = match self.kind {
            CloudKind::Cumulus => {
                let c = [(self.lo[0] + self.hi[0]) * 0.5, (self.lo[2] + self.hi[2]) * 0.5];
                let (rx, rz) = ((self.hi[0] - self.lo[0]) * 0.5 / 1.1, (self.hi[2] - self.lo[2]) * 0.5 / 0.9);
                let (u, w) = ((p[0] - c[0]) / rx, (p[2] - c[1]) / rz);
                let r2 = u * u + w * w;
                // towers: the top is lumpy at the heap scale; a dome overall
                let towers = Octaves::billow(s + 3, 3, heap * 1.6).get(p[0], p[2]);
                let top = (1.0 - r2).max(0.0).powf(0.6) * (0.55 + 0.45 * towers) / 1.1;
                let base = 0.03 * r2;
                (y - base).min(top - y) * 3.0
            }
            CloudKind::Bank => {
                let len = self.hi[0] - self.lo[0];
                let u = (p[0] - self.lo[0]) / len;
                // ragged ends and a rising and falling top of heaped towers
                let ends = smoothstep(0.0, 0.12, u) * smoothstep(1.0, 0.85, u);
                let towers = Octaves::billow(s + 3, 4, heap * 2.5).get(p[0], p[2] * 0.5);
                let swell = Fbm::new(s + 5, 2, len * 0.4).get01(p[0], 0.3);
                let top = ends * (0.3 + 0.45 * towers + 0.35 * swell);
                let dz = (p[2] - self.lo[2]) / (self.hi[2] - self.lo[2]);
                let front = (dz.min(1.0 - dz) * 3.0).min(1.0);
                (y - 0.02).min(top - y).min(front * 0.3) * 3.0
            }
            CloudKind::Stratus => {
                let (sa, ca) = self.wind.0.sin_cos();
                let along = (p[0] * ca + p[2] * sa) / self.wind.1;
                let across = -p[0] * sa + p[2] * ca;
                let wp = Warp::new(s + 7, heap * 2.0, heap * 0.8);
                let (u, v) = wp.at(along, across);
                let n = Octaves::new(s + 9, 5, heap * 1.6, crate::noise::Fold::Plain).get_lod(u, v, lod) * 0.5 + 0.5;
                let mut c = n - (1.0 - self.cover);
                if let Some((per, amt)) = self.breaks {
                    // a fraction `amt` of the cells opens a hole about its
                    // feature point, each its own size
                    let cell = Worley::new(s + 11, per).get(u * 0.8 + 0.2 * along, v);
                    if cell.rand() < amt {
                        let size = 0.35 + 0.4 * ((cell.rand() * 7.3).fract());
                        c -= 1.5 * (1.0 - cell.f1 / size).max(0.0).powf(0.7);
                    }
                }
                // thickest in the middle of the layer, thinner where cover is thin
                let prof = 1.0 - (2.0 * y - 1.0).abs();
                c.min(prof * (0.3 + c.max(0.0))) * 2.0
            }
        };
        // heaped surfaces: billows at the heap scale erode the edge
        let erode = billow.get3([p[0], p[1] * 1.3, p[2]], lod) - 0.45;
        let e = inside + 0.35 * erode;
        if e <= 0.0 {
            return 0.0;
        }
        self.density * smoothstep(0.0, self.soft, e)
    }
}

/// Clouds lit by one sun under one sky.
pub struct Clouds {
    pub clouds: Vec<Cloud>,
    /// Ground albedo (light bounced up into the bellies).
    pub ground: f32,
    /// Silver lining: the forward lobe's anisotropy (0.5–0.85).
    pub forward: f32,
}

/// The clouds' fields over a camera's view (see `Clouds::field`).
pub struct CloudField {
    x0: f32,
    y0: f32,
    cell: f32,
    nx: usize,
    ny: usize,
    alpha: Vec<f32>,
    lit: Vec<f32>,
    glow: Vec<f32>,
    amb: Vec<f32>,
    dist: Vec<f32>,
    rad: Vec<Rgb>,
    exposure: Exposure,
}

impl Clouds {
    pub fn new(clouds: Vec<Cloud>) -> Self {
        Clouds { clouds, ground: 0.15, forward: 0.65 }
    }

    /// Density of all clouds at a world point.
    pub fn density(&self, p: V3, lod: f32) -> f32 {
        self.clouds.iter().map(|c| c.at(p, lod)).sum()
    }

    /// March the clouds through `world`'s camera every `cell` units (2 at
    /// 1000px is plenty to paint from) and light them with `sky`'s sun.
    /// Parts of the view below the horizon are skipped.
    pub fn field(&self, sky: &SkyField, world: &World, cell: f32) -> CloudField {
        let v = world.view;
        let (x0, y0) = (v[0] - cell, v[1] - cell);
        let nx = ((v[2] + 2.0 * cell) / cell).ceil() as usize + 1;
        let ny = (((world.horizon - y0) / cell).ceil() as usize + 2).min(((v[3] + 2.0 * cell) / cell).ceil() as usize + 1);
        let eye = [0.0, sky.sky.alt.max(world.eye), 0.0];
        let l = sky.sky.sun_dir();
        let skyr = &sky.sky;
        // light from the sky above onto a cloud's top, and bounced up from
        // the ground into its belly
        // the dome's mean light (single scattered plus the fill), a little
        // brighter than the sky overhead
        let upper = scale(skyr.dome(), 1.0 + skyr.fill);
        let sun_here = skyr.sunlight();
        let ground = scale(plus(scale(sun_here, l[1].max(0.0) / PI), scale(upper, 0.5)), self.ground);
        let fw = self.forward;
        // the sun's flux onto a level cloud top (from below after sunset)
        let slab = l[1].abs().max(0.08).sqrt();
        type Px = (f32, f32, f32, f32, f32, Rgb);
        let px: Vec<Px> = (0..nx * ny)
            .into_par_iter()
            .map(|i| {
                let (x, y) = (x0 + (i % nx) as f32 * cell, y0 + (i / nx) as f32 * cell);
                let d = crate::form::unit([(x - world.cx) / world.focal, -(y - world.horizon) / world.focal, 1.0]);
                if d[1] <= 0.0 {
                    return (0.0, 0.0, 0.0, 0.0, 0.0, [0.0; 3]);
                }
                // each cloud's span along the ray, with the step that
                // resolves it (64 samples across it); gaps between clouds
                // are skipped, so a distant bank cannot coarsen the march
                // through a near heap
                let spans: Vec<(f32, f32, f32)> = self.clouds.iter().filter_map(|c| c.span(eye, d)).filter(|(a, b)| b > a).map(|(a, b)| (a, b, (b - a) / 64.0)).collect();
                if spans.is_empty() {
                    return (0.0, 0.0, 0.0, 0.0, 0.0, [0.0; 3]);
                }
                let mut cuts: Vec<f32> = spans.iter().flat_map(|&(a, b, _)| [a, b]).collect();
                cuts.sort_by(f32::total_cmp);
                cuts.dedup();
                // the occupied pieces between cuts and the finest step over each
                let pieces: Vec<(f32, f32, f32)> = cuts
                    .windows(2)
                    .filter_map(|w| {
                        let m = 0.5 * (w[0] + w[1]);
                        let step = spans.iter().filter(|&&(a, b, _)| a <= m && m <= b).map(|s| s.2).fold(f32::MAX, f32::min);
                        (step < f32::MAX).then_some((w[0], w[1], step))
                    })
                    .collect();
                let mu = dot(d, l);
                // multiple scattering by octaves (Wrenninge et al. 2013):
                // each octave reaches deeper (optical depth × b^i), carries
                // less (a^i) and scatters less forward (g × c^i)
                let oct: [(f32, f32, f32); 2] = [(1.0, 1.0, 1.0), (0.5, 0.5, 0.5)];
                let phases: Vec<f32> = oct.iter().map(|&(_, _, c)| (0.75 * hg(mu, fw * c) + 0.25 * hg(mu, -0.25 * c)) * 4.0 * PI).collect();
                let (mut tr, mut sun_iso, mut sun_ph, mut amb, mut dsum, mut wsum) = (1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
                let mut sun_col = [0.0f32; 3];
                let mut got_col = false;
                'march: for &(t0, t1, step) in &pieces {
                    let n = ((t1 - t0) / step - 1e-3).ceil().max(1.0) as usize;
                    let dt = (t1 - t0) / n as f32;
                    for k in 0..n {
                        let t = t0 + (k as f32 + 0.5) * dt;
                        let p = add(eye, d, t);
                        let lod = t * cell / world.focal;
                        let sigma = self.density(p, lod);
                        if sigma <= 0.0 {
                            continue;
                        }
                        if !got_col {
                            sun_col = skyr.sunlight_at(p);
                            got_col = true;
                        }
                        // toward the sun through the clouds: 6 steps, longer and longer
                        let mut tau = 0.0;
                        let mut s = 30.0f32;
                        let mut q = p;
                        for _ in 0..6 {
                            q = add(q, l, s);
                            tau += self.density(q, lod.max(s * 0.3)) * s;
                            s *= 1.8;
                        }
                        // the sky light from above, shaded by the cloud over it
                        let mut up = 0.0;
                        let mut s = 60.0f32;
                        let mut q = p;
                        for _ in 0..3 {
                            q = add(q, [0.0, 1.0, 0.0], s);
                            up += self.density(q, lod.max(s * 0.3)) * s;
                            s *= 2.0;
                        }
                        let direct = (-tau).exp();
                        let a = tr * (1.0 - (-sigma * dt).exp());
                        sun_iso += a * direct;
                        for (o, &(ka, kb, _)) in oct.iter().enumerate() {
                            sun_ph += a * ka * (-tau * kb).exp() * phases[o];
                        }
                        // light diffused through the cloud (two-stream transmission,
                        // 1 / (1 + 0.75 (1 − g) τ), g ≈ 0.85): what lights a deck's
                        // underside and a heap's belly, neutral in color
                        sun_ph += a * 1.2 * slab / (1.0 + 0.75 * 0.15 * tau);
                        amb += a * (-up * 0.5).exp();
                        dsum += a * t;
                        wsum += a;
                        tr *= (-sigma * dt).exp();
                        if tr < 0.005 {
                            break 'march;
                        }
                    }
                }
                let alpha = 1.0 - tr;
                if alpha <= 1e-4 {
                    return (0.0, 0.0, 0.0, 0.0, 0.0, [0.0; 3]);
                }
                let lit = sun_iso / alpha;
                let glow = sun_ph;
                let ambient = amb / alpha;
                let dist = dsum / wsum.max(1e-6);
                // what the eye gets from the cloud (before the air in front)
                let ir = 1.0 / (4.0 * PI);
                let mut rad = plus(scale(sun_col, glow * ir * 1.0), scale(upper, amb * 1.2));
                rad = plus(rad, scale(ground, alpha - amb.min(alpha)));
                // the air between the eye and the cloud veils it
                let t_air = skyr.transmittance(d, dist.min(80_000.0));
                let behind = skyr.radiance(d);
                let air = [behind[0] * (1.0 - t_air[0]), behind[1] * (1.0 - t_air[1]), behind[2] * (1.0 - t_air[2])];
                let seen = plus(times(rad, t_air), scale(air, alpha));
                (alpha, lit, glow, ambient, dist, seen)
            })
            .collect();
        let mut f = CloudField {
            x0,
            y0,
            cell,
            nx,
            ny,
            alpha: vec![0.0; nx * ny],
            lit: vec![0.0; nx * ny],
            glow: vec![0.0; nx * ny],
            amb: vec![0.0; nx * ny],
            dist: vec![0.0; nx * ny],
            rad: vec![[0.0; 3]; nx * ny],
            exposure: sky.exposure,
        };
        for (i, p) in px.into_iter().enumerate() {
            f.alpha[i] = p.0;
            f.lit[i] = p.1;
            f.glow[i] = p.2;
            f.amb[i] = p.3;
            f.dist[i] = p.4;
            f.rad[i] = p.5;
        }
        f
    }
}

impl CloudField {
    fn bilinear<T: Copy>(&self, v: &[T], x: f32, y: f32, lerp: impl Fn(T, T, f32) -> T) -> T {
        let u = ((x - self.x0) / self.cell).clamp(0.0, (self.nx - 1) as f32 - 1e-3);
        let w = ((y - self.y0) / self.cell).clamp(0.0, (self.ny - 1) as f32 - 1e-3);
        let (i, j) = (u as usize, w as usize);
        let (fu, fw) = (u - i as f32, w - j as f32);
        let at = |i: usize, j: usize| v[j * self.nx + i];
        lerp(lerp(at(i, j), at(i + 1, j), fu), lerp(at(i, j + 1), at(i + 1, j + 1), fu), fw)
    }
    fn get(&self, v: &[f32], x: f32, y: f32) -> f32 {
        self.bilinear(v, x, y, |a, b, t| a + (b - a) * t)
    }
    /// How much of the sky the clouds hide (0..1).
    pub fn alpha(&self, x: f32, y: f32) -> f32 {
        self.get(&self.alpha, x, y)
    }
    /// How sunlit the cloud seen here is: 0 the shadowed belly, 1 the lit
    /// edge facing the sun (no phase function: pure geometry of light).
    pub fn lit(&self, x: f32, y: f32) -> f32 {
        self.get(&self.lit, x, y)
    }
    /// Sunlight sent toward the eye, with the phase function: strong on thin
    /// edges between the eye and the sun (the silver lining), where `lit`
    /// alone would be modest. ~0..several.
    pub fn glow(&self, x: f32, y: f32) -> f32 {
        self.get(&self.glow, x, y)
    }
    /// How much open sky the cloud seen here gets from above (0..1).
    pub fn ambient(&self, x: f32, y: f32) -> f32 {
        self.get(&self.amb, x, y)
    }
    /// Distance (m) to the cloud seen here.
    pub fn dist(&self, x: f32, y: f32) -> f32 {
        self.get(&self.dist, x, y)
    }
    /// The edge's softness at a point: how many units the cloud takes to go
    /// from clear to solid here (large inside and outside, small at a hard
    /// edge).
    pub fn soft(&self, x: f32, y: f32) -> f32 {
        let e = self.cell;
        let gx = (self.alpha(x + e, y) - self.alpha(x - e, y)) / (2.0 * e);
        let gy = (self.alpha(x, y + e) - self.alpha(x, y - e)) / (2.0 * e);
        (1.0 / (gx * gx + gy * gy).sqrt().max(1e-3)).min(1000.0)
    }
    /// The cloud's own color at a point over the sky behind it, as paint
    /// (with the sky field's exposure): `sky` composited under the cloud.
    pub fn color(&self, sky: &SkyField, x: f32, y: f32) -> Rgb {
        let a = self.alpha(x, y);
        let r = self.bilinear(&self.rad, x, y, |p, q, t| [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t]);
        let s = sky.radiance(x, y);
        self.exposure.map(plus(scale(s, 1.0 - a), r))
    }
    /// The cloud alone, as paint (for a pass that paints only the cloud).
    pub fn cloud_color(&self, x: f32, y: f32) -> Rgb {
        let a = self.alpha(x, y).max(1e-3);
        let r = self.bilinear(&self.rad, x, y, |p, q, t| [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t]);
        self.exposure.map(scale(r, 1.0 / a))
    }
    /// A whole-canvas mask of `g(alpha, lit, glow, soft)` (masks must be
    /// whole-canvas frames).
    pub fn mask(&self, f: Frame, g: impl Fn(&CloudPoint) -> f32 + Sync) -> Mask {
        Mask::from_fn(f, |x, y| {
            let a = self.alpha(x, y);
            if a <= 0.0 {
                return g(&CloudPoint::default());
            }
            g(&CloudPoint { alpha: a, lit: self.lit(x, y), glow: self.glow(x, y), ambient: self.ambient(x, y), dist: self.dist(x, y) })
        })
    }
}

/// The cloud at one canvas point (for `CloudField::mask`).
#[derive(Clone, Copy, Debug, Default)]
pub struct CloudPoint {
    pub alpha: f32,
    pub lit: f32,
    pub glow: f32,
    pub ambient: f32,
    pub dist: f32,
}

// --------------------------------------------------------------- ranges

/// The shape of one mass in a range's skyline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Silhouette {
    /// A pointed summit with concave flanks.
    Peak,
    /// A rounded, worn summit.
    Dome,
    /// A flat or tilted top with steep shoulders (a table mountain).
    Plateau,
    /// Two summits with a col between.
    Saddle,
    /// A long slope ending in a sheer drop on one side.
    Cliff,
}

/// One mass: centered at `x` m, `half` m to either side, `h` m high, of a
/// kind, leaning (−1..1: which flank is steeper).
#[derive(Clone, Copy, Debug)]
pub struct Mass {
    pub x: f32,
    pub half: f32,
    pub h: f32,
    pub kind: Silhouette,
    pub lean: f32,
    /// Where the drop is (cliffs) or the col (saddles), −1..1.
    pub at: f32,
}

impl Mass {
    /// Height (m) at X.
    pub fn height(&self, x: f32) -> f32 {
        let u = (x - self.x) / self.half;
        if u.abs() >= 1.0 {
            return 0.0;
        }
        // lean: push the summit toward one side
        let u = u - self.lean * 0.5 * (1.0 - u * u);
        let a = u.abs().min(1.0);
        let f = match self.kind {
            Silhouette::Peak => (1.0 - a).powf(1.6),
            Silhouette::Dome => (1.0 - a * a).max(0.0).powf(0.9),
            Silhouette::Plateau => {
                let top = 0.92 + 0.08 * u * self.at.signum();
                top * (1.0 - smoothstep(0.55, 1.0, a).powf(0.7))
            }
            Silhouette::Saddle => {
                let col = self.at * 0.4;
                (1.0 - a).powf(0.9) * (1.0 - 0.38 * (-((u - col) / 0.22).powi(2)).exp())
            }
            Silhouette::Cliff => {
                // a long slope up toward the drop at `at`, then sheer
                let c = self.at.clamp(-0.6, 0.6);
                let dir = if self.lean >= 0.0 { 1.0 } else { -1.0 };
                let s = u * dir;
                let cc = c * dir;
                if s < cc { ((s + 1.0) / (cc + 1.0)).powf(1.3) } else { (1.0 - smoothstep(cc, cc + 0.12, s)) * 0.85 + 0.15 * (1.0 - (s - cc) / (1.0 - cc)).max(0.0) }
            }
        };
        self.h * f.max(0.0)
    }
}

/// Haze in the low air: aerosols thinning with height, and optionally
/// mist lying in the valleys up to an uneven top.
#[derive(Clone, Copy, Debug)]
pub struct Haze {
    /// Meteorological visibility at sea level (m): where 98 % of a
    /// contrast is lost (Koschmieder: β = 3.912 / V).
    pub visibility: f32,
    /// Scale height of the haze (m): 1200 typical, 400 a low inversion.
    pub height: f32,
    /// Valley mist: its top (m), how much denser than the haze it is, and
    /// how uneven its top is (m).
    pub mist: Option<(f32, f32, f32)>,
    seed: u32,
}

impl Haze {
    pub fn new(visibility: f32) -> Self {
        Haze { visibility, height: 1200.0, mist: None, seed: 1 }
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = h.max(50.0);
        self
    }
    /// Mist in the valleys up to about `top` m (± `uneven` m), `density`
    /// times as thick as the haze at the ground.
    pub fn mist(mut self, top: f32, density: f32, uneven: f32, seed: u32) -> Self {
        self.mist = Some((top, density, uneven));
        self.seed = seed;
        self
    }
    /// The fraction of a thing's own color lost to the air between an eye
    /// at `eye` m and a point `dist` m away, `h` m up, at world X `x`.
    /// Integrates the haze's exponential profile along the line of sight
    /// (and the mist below its top), so the foot of a far range is hazier
    /// than its crest.
    pub fn loss(&self, eye: f32, dist: f32, h: f32, x: f32) -> f32 {
        let b0 = 3.912 / self.visibility;
        let hh = self.height;
        // ∫ exp(-y/H) ds along the straight segment from eye to h over dist
        let dy = h - eye;
        let mean = if dy.abs() < 1.0 { (-eye / hh).exp() } else { hh * ((-eye / hh).exp() - (-h / hh).exp()) / dy };
        let mut tau = b0 * dist * mean.max(0.0);
        // molecules: a little blue loss, 1.2e-5 per m at sea level
        tau += 1.2e-5 * dist * (-(0.5 * (eye + h)) / 8000.0).exp();
        if let Some((top, dens, uneven)) = self.mist {
            let n = Fbm::new(self.seed + 3, 3, 3000.0);
            let top = top + uneven * n.get(x, dist * 0.7);
            // the part of the path below the mist's top, in the far half
            let lo = eye.min(h);
            let below = if h >= top && eye >= top {
                0.0
            } else if dy.abs() < 1.0 {
                1.0
            } else {
                ((top - lo) / dy.abs()).clamp(0.0, 1.0)
            };
            // mist lies where the ground is: weight toward the far end
            let deep = smoothstep(top, top - 0.6 * (top - lo).abs().max(1.0), h);
            tau += b0 * dens * dist * below * (0.25 + 0.75 * deep);
        }
        1.0 - (-tau).exp()
    }
}

/// One range: masses along a line running (maybe obliquely) at distance
/// `z` m, with fine crest detail.
#[derive(Clone)]
pub struct RangeLayer {
    /// Its index, nearest 0.
    pub k: usize,
    /// Distance (m) where it crosses the line of sight straight ahead.
    pub z: f32,
    /// How obliquely it runs: extra distance per meter to the right.
    pub skew: f32,
    pub masses: Vec<Mass>,
    /// The height of the low country between the masses (m).
    pub floor: f32,
    detail: Octaves,
    rough: f32,
    seed: u32,
}

impl RangeLayer {
    /// The distance (m) of this range at world X.
    pub fn z_at(&self, x: f32) -> f32 {
        (self.z + self.skew * x).max(50.0)
    }
    /// Crest height (m above the datum) at world X, before the Earth's curvature.
    pub fn height(&self, x: f32, lod: f32) -> f32 {
        let mut h = self.floor * (0.6 + 0.4 * Fbm::new(self.seed + 21, 2, 4.0 * self.z.max(1000.0) * 0.4).get01(x, 1.3));
        // the flanks wander: masses are sampled through a warp at their own scale
        let hm0 = self.masses.iter().map(|m| m.h).fold(0.0, f32::max).max(1.0);
        let x = x + self.rough * 3.0 * hm0 * Fbm::new(self.seed + 23, 3, hm0 * 2.5).get(x, 0.7);
        // masses overlap by a soft maximum: notches where two meet
        let k = 0.04 * self.masses.iter().map(|m| m.h).fold(1.0, f32::max);
        for m in &self.masses {
            let v = m.height(x);
            let d = (h - v).abs();
            h = h.max(v) + if d < k { (k - d).powi(2) / (4.0 * k) * 0.3 } else { 0.0 };
        }
        // crest detail at two scales, ridged (sharp crests, broad notches):
        // sub-summits and shoulders the size of a mass's flank, then
        // crags; finer ones only where the range is near enough (lod)
        let hm = self.masses.iter().map(|m| m.h).fold(0.0, f32::max).max(self.floor);
        let big = self.detail.get_lod(x * 2.5, 0.5, lod * 2.5) - 0.5;
        let fine = self.detail.get_lod(x * 12.0, 3.5, lod * 12.0) - 0.5;
        h + self.rough * (1.2 * h.max(self.floor) * big + 0.12 * hm * fine * (0.3 + big.abs() * 2.0)) * smoothstep(0.0, hm * 0.3, h)
    }
    /// World X seen at canvas x on this range (solving for its skew).
    pub fn world_x(&self, w: &World, x: f32) -> f32 {
        let a = (x - w.cx) / w.focal;
        a * self.z / (1.0 - a * self.skew).max(0.05)
    }
    /// The drop (m) of a point `dist` m away below the eye's level line,
    /// from the Earth's curvature less refraction (k = 0.13).
    pub fn curvature(dist: f32) -> f32 {
        dist * dist / (2.0 * EARTH_R) * (1.0 - 0.13)
    }
    /// The crest on the canvas: its y at canvas x (units, y down).
    pub fn crest(&self, w: &World, x: f32) -> f32 {
        let wx = self.world_x(w, x);
        let z = self.z_at(wx);
        let lod = z / w.focal;
        let h = self.height(wx, lod) - Self::curvature(z);
        w.horizon + w.focal * (w.eye - h) / z
    }
    /// The height (m) of the point of this range's face seen at canvas
    /// (x, y), treating the face as standing at the crest's distance.
    pub fn height_at(&self, w: &World, x: f32, y: f32) -> f32 {
        let z = self.z_at(self.world_x(w, x));
        w.eye - (y - w.horizon) * z / w.focal + Self::curvature(z)
    }
    /// The fraction of the range's own color lost to the air at canvas
    /// (x, y) on its face: more at the foot than the crest, more where the
    /// range runs away from us.
    pub fn haze(&self, w: &World, air: &Haze, x: f32, y: f32) -> f32 {
        let wx = self.world_x(w, x);
        let z = self.z_at(wx);
        let h = self.height_at(w, x, y).max(0.0);
        air.loss(w.eye, z, h, wx)
    }
    /// A `form::Ridge` for this range's face, from canvas x0 to x1: its
    /// crest is `crest`, its gullies sized to the range's distance
    /// (`gully_m` meters apart), reaching `depth` units down.
    pub fn ridge(&self, w: &World, x0: f32, x1: f32, depth: f32, gully_m: f32) -> Ridge {
        let s = w.focal / self.z;
        Ridge::new(x0, x1, |x| self.crest(w, x), depth, self.seed).lean(0.9, 0.7).gullies((gully_m * s).max(2.0), 0.45)
    }
}

/// A set of receding ranges (nearest first). Build with `Ranges::new`,
/// set the knobs, then `build`.
#[derive(Clone, Debug)]
pub struct Ranges {
    pub near: f32,
    pub far: f32,
    pub count: usize,
    /// Heights (m) of the tallest masses: nearest and farthest range (each
    /// range varies around a value between them).
    pub heights: (f32, f32),
    /// 0 regular (the old waves), 1 a hand spacing and sizing by eye.
    pub irregular: f32,
    /// How much the ranges run obliquely (0 all parallel to the picture).
    pub oblique: f32,
    /// World X span (m) to fill at the nearest distance (the view's width
    /// there and more).
    pub span: f32,
    pub seed: u32,
    /// Which kinds may appear (chosen by chance per mass).
    pub kinds: Vec<Silhouette>,
}

impl Ranges {
    /// `count` ranges from `near` to `far` meters.
    pub fn new(near: f32, far: f32, count: usize, seed: u32) -> Self {
        Ranges {
            near,
            far,
            count,
            heights: (300.0, 1400.0),
            irregular: 0.8,
            oblique: 0.5,
            span: 0.0,
            seed,
            kinds: vec![Silhouette::Peak, Silhouette::Dome, Silhouette::Plateau, Silhouette::Saddle, Silhouette::Cliff],
        }
    }
    pub fn heights(mut self, near: f32, far: f32) -> Self {
        self.heights = (near, far);
        self
    }
    pub fn irregular(mut self, k: f32) -> Self {
        self.irregular = k;
        self
    }
    pub fn oblique(mut self, k: f32) -> Self {
        self.oblique = k;
        self
    }
    pub fn kinds(mut self, k: &[Silhouette]) -> Self {
        self.kinds = k.to_vec();
        self
    }
    /// The layers for a camera (nearest first). Distances are spaced
    /// unevenly in log distance, each range spans only part of the view
    /// (some enter from a side and end), masses are placed unevenly and
    /// vary in kind, width and height, and each range runs at its own
    /// slant.
    pub fn build(&self, w: &World) -> Vec<RangeLayer> {
        let s = self.seed;
        let irr = self.irregular;
        let n = self.count.max(1);
        let (ln, lf) = (self.near.ln(), self.far.ln());
        let ts = uneven(n, 0.0, 1.0, irr, 0.5 * irr, s);
        let half_view = |z: f32| 0.5 * w.view[2] / w.focal * z;
        let mut out = Vec::with_capacity(n);
        for (k, t) in ts.iter().enumerate() {
            let z = (ln + (lf - ln) * t).exp();
            let r = |j: i32| rand01(k as i32, j, s);
            let hw = half_view(z) * 1.3;
            // the tallest mass: between the near and far heights, varied a lot
            let h_top = vary(self.heights.0 + (self.heights.1 - self.heights.0) * t, 0.55 * irr, k, s + 1);
            // extent: some ranges fill the view, others come in from one side
            let (a, b) = if r(1) < 0.35 * irr + 0.1 && k > 0 {
                if r(2) < 0.5 { (-hw, hw * (-0.2 + 0.8 * r(3))) } else { (-hw * (-0.2 + 0.8 * r(3)), hw) }
            } else {
                (-hw, hw)
            };
            // masses sized from their height (flanks of ~10–35°): a peak's
            // half-width is 1.4–2.5 heights, a plateau's 3–5; as many as
            // fill the span, placed unevenly (some overlap, some leave gaps)
            let width_of = |kind: Silhouette, q: f32| match kind {
                Silhouette::Peak => 1.4 + 1.1 * q,
                Silhouette::Dome => 2.5 + 1.5 * q,
                Silhouette::Plateau => 3.0 + 2.0 * q,
                Silhouette::Saddle => 3.0 + 1.5 * q,
                Silhouette::Cliff => 2.5 + 1.5 * q,
            };
            let typical = if irr <= 0.0 { 2.0 } else { 3.0 };
            // masses overlap: about one per 1.6 heights of span (fewer on a
            // range that is all plateau, more on a jagged one)
            let fill = (b - a) / (2.0 * h_top * 1.6);
            let nm = ((fill * (0.7 + 0.6 * r(4) * irr)).round() as usize).clamp(1, 12);
            let xs = uneven(nm + 2, a, b, irr, 0.5 * irr, s + 10 + k as u32);
            let xs = &xs[1..=nm];
            let mut masses = vec![];
            let main = ((r(6) * nm as f32) as usize).min(nm - 1);
            for (j, &x) in xs.iter().enumerate() {
                let q = |i: i32| rand01(k as i32 * 31 + j as i32, i, s + 5);
                let kind = if irr <= 0.0 { Silhouette::Peak } else { self.kinds[(q(1) * self.kinds.len() as f32) as usize % self.kinds.len()] };
                let big = if j == main { 1.0 } else { 0.3 + 0.6 * q(2) };
                let h = h_top * if irr <= 0.0 { 0.9 + 0.1 * q(2) } else { big };
                let half = h * if irr <= 0.0 { typical } else { width_of(kind, q(3)) };
                masses.push(Mass { x, half, h, kind, lean: (q(4) * 2.0 - 1.0) * irr, at: q(5) * 2.0 - 1.0 });
            }
            let skew = (r(7) * 2.0 - 1.0) * 0.25 * self.oblique * irr;
            out.push(RangeLayer {
                k,
                z,
                skew,
                masses,
                floor: h_top * if irr <= 0.0 { 0.3 } else { 0.15 + 0.35 * r(8) },
                detail: Octaves::ridged(s + 40 + k as u32, 6, (h_top * 3.0).max(200.0)),
                rough: 0.08 + 0.14 * irr,
                seed: s + 100 + k as u32,
            });
        }
        out
    }

    /// The old way, for comparison: evenly spaced ranges parallel to the
    /// picture, each with a few similar peaks of similar height (they
    /// read as waves).
    pub fn regular(&self, w: &World) -> Vec<RangeLayer> {
        Ranges { irregular: 0.0, oblique: 0.0, ..self.clone() }.build(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> World {
        World::new([0.0, 0.0, 1000.0, 600.0], 380.0, 3.0).sun(Sun::deg(-40.0, 12.0))
    }

    /// The day sky: brighter and paler at the horizon than at the zenith,
    /// brightest toward the sun, darker opposite it; blue overhead.
    #[test]
    fn day_sky_structure() {
        let sky = Sky::new(Sun::deg(0.0, 20.0));
        let zen = sky.radiance([0.0, 1.0, 0.0]);
        let hor_side = sky.radiance(crate::form::unit([1.0, 0.03, 0.0]));
        let toward = sky.radiance(crate::form::unit([0.0, 0.2, 1.0]));
        let away = sky.radiance(crate::form::unit([0.0, 0.2, -1.0]));
        assert!(zen[2] > zen[0] * 1.5, "zenith blue {zen:?}");
        assert!(lum(hor_side) > lum(zen), "horizon {hor_side:?} zenith {zen:?}");
        // paler at the horizon: less blue relative to red
        assert!(hor_side[2] / hor_side[0] < zen[2] / zen[0]);
        assert!(lum(toward) > 2.0 * lum(away), "toward {toward:?} away {away:?}");
        // a low sun's light is reddened
        let low = Sky::new(Sun::deg(0.0, 2.0)).sunlight();
        assert!(low[0] > low[2] * 1.5, "{low:?}");
    }

    /// Twilight: the Earth's shadow is low and dark opposite the sun, the
    /// Belt of Venus above it is pinker, and the sky over the sun glows warm.
    #[test]
    fn twilight_structure() {
        let sky = Sky::new(Sun::deg(0.0, -3.0));
        let e = |deg: f32| deg.to_radians().tan();
        let shadow = sky.radiance(crate::form::unit([0.0, e(1.0), -1.0]));
        let belt = sky.radiance(crate::form::unit([0.0, e(10.0), -1.0]));
        let glow = sky.radiance(crate::form::unit([0.0, e(2.0), 1.0]));
        assert!(lum(belt) > lum(shadow) * 1.2, "belt {belt:?} shadow {shadow:?}");
        assert!(belt[0] / belt[2] > shadow[0] / shadow[2], "belt warmer than the shadow");
        assert!(glow[0] > glow[2], "afterglow warm {glow:?}");
        assert!(lum(glow) > lum(belt));
        // the shadow overhead reaches about R d²/2 ≈ 8.7 km; at 15 km the sun still shines; at the ground it is gone
        assert!(lum(sky.sunlight_at([0.0, 15_000.0, 0.0])) > 1e-3, "{:?}", sky.sunlight_at([0.0, 15_000.0, 0.0]));
        assert_eq!(sky.sunlight(), [0.0; 3]);
    }

    #[test]
    fn sky_field_maps_to_paint() {
        let w = world();
        let f = SkyField::new(Sky::new(w.sun).uneven(0.5, 30_000.0, 3), &w, 10.0);
        for (x, y) in [(10.0, 10.0), (500.0, 300.0), (990.0, 370.0), (500.0, 450.0)] {
            let c = f.at(x, y);
            assert!(c.iter().all(|v| v.is_finite() && *v >= 0.0 && *v <= 0.861), "{c:?}");
        }
        // not a vertical ramp: along a row the sky changes
        let row: Vec<f32> = (0..10).map(|i| f.value(50.0 + 100.0 * i as f32, 300.0)).collect();
        let (mn, mx) = row.iter().fold((1.0f32, 0.0f32), |a, &v| (a.0.min(v), a.1.max(v)));
        assert!(mx > mn * 1.15, "{row:?}");
    }

    #[test]
    fn clouds_have_lit_edges_and_shadowed_bellies() {
        let w = World::new([0.0, 0.0, 400.0, 300.0], 280.0, 2.0).fov(400.0, 50.0).sun(Sun::deg(-70.0, 25.0));
        let sf = SkyField::new(Sky::new(w.sun), &w, 10.0);
        let cu = Cloud::cumulus(0.0, 6000.0, 900.0, 1800.0, 1400.0, 5);
        let cf = Clouds::new(vec![cu]).field(&sf, &w, 3.0);
        // find the cloud in view and compare its left (sunward) and right sides
        let mut pts = vec![];
        for j in 0..100 {
            for i in 0..130 {
                let (x, y) = (i as f32 * 3.0, j as f32 * 2.8);
                if cf.alpha(x, y) > 0.9 {
                    pts.push((x, y, cf.lit(x, y)));
                }
            }
        }
        assert!(pts.len() > 20, "cloud not in view");
        let xs: Vec<f32> = pts.iter().map(|p| p.0).collect();
        let (x0, x1) = (xs.iter().cloned().fold(f32::MAX, f32::min), xs.iter().cloned().fold(0.0, f32::max));
        let side = |a: f32, b: f32| {
            let v: Vec<f32> = pts.iter().filter(|p| p.0 >= a && p.0 <= b).map(|p| p.2).collect();
            v.iter().sum::<f32>() / v.len().max(1) as f32
        };
        let (l, r) = (side(x0, x0 + (x1 - x0) * 0.25), side(x1 - (x1 - x0) * 0.25, x1));
        assert!(l > r * 1.3, "lit side {l}, shadow side {r}");
        let ys: Vec<f32> = pts.iter().map(|p| p.1).collect();
        let yb = ys.iter().cloned().fold(0.0, f32::max);
        let belly = pts.iter().filter(|p| p.1 > yb - 6.0).map(|p| p.2).sum::<f32>() / pts.iter().filter(|p| p.1 > yb - 6.0).count().max(1) as f32;
        assert!(belly < l, "belly {belly} lit {l}");
    }

    #[test]
    fn ranges_are_uneven() {
        let w = world();
        let rs = Ranges::new(1500.0, 40_000.0, 5, 7).build(&w);
        assert_eq!(rs.len(), 5);
        let gaps: Vec<f32> = rs.windows(2).map(|p| (p[1].z / p[0].z).ln()).collect();
        let (mn, mx) = gaps.iter().fold((f32::MAX, 0.0f32), |a, &g| (a.0.min(g), a.1.max(g)));
        assert!(mx > 1.5 * mn, "{gaps:?}");
        let old = Ranges::new(1500.0, 40_000.0, 5, 7).regular(&w);
        let og: Vec<f32> = old.windows(2).map(|p| (p[1].z / p[0].z).ln()).collect();
        assert!(og.iter().all(|g| (g - og[0]).abs() < 1e-3));
        // crests are finite, above the horizon somewhere, and the foot is hazier
        let air = Haze::new(30_000.0);
        for r in &rs {
            let ys: Vec<f32> = (0..50).map(|i| r.crest(&w, i as f32 * 20.0)).collect();
            assert!(ys.iter().all(|y| y.is_finite()));
            let x = 500.0;
            let c = r.crest(&w, x);
            assert!(r.haze(&w, &air, x, c + 30.0) >= r.haze(&w, &air, x, c) - 1e-4);
        }
    }

    #[test]
    #[ignore]
    fn probe_twilight() {
        for el in [5.0f32, 0.5, -2.0, -4.0, -7.0] {
            let sky = Sky::new(Sun::deg(0.0, el));
            let e = |deg: f32| deg.to_radians().tan();
            println!("sun {el}: sunlight {:?}", sky.sunlight());
            for side in [-1.0f32, 1.0] {
                for a in [0.5f32, 2.0, 5.0, 10.0, 20.0, 40.0, 89.0] {
                    let c = sky.radiance(crate::form::unit([0.0, e(a), side]));
                    let l = lum(c);
                    println!("  {} {a:>4}: lum {:.2e} rgb/l {:.2} {:.2} {:.2}", if side < 0.0 { "anti" } else { "sun " }, l, c[0] / l, c[1] / l, c[2] / l);
                }
            }
        }
    }

    /// The dome radiance follows the public fields even after it was
    /// sampled: moving the sun below the horizon gives the fresh twilight.
    #[test]
    fn sky_dome_follows_field_changes() {
        let mut sky = Sky::new(Sun::deg(0.0, 30.0));
        let _ = sky.dome();
        sky.sun = Sun::deg(0.0, -3.0);
        let fresh = Sky::new(sky.sun);
        assert_eq!(sky.dome(), fresh.dome());
        assert_eq!(sky.radiance([0.0, 1.0, 0.0]), fresh.radiance([0.0, 1.0, 0.0]));
        // and back again (the first value is still cached) and on a clone
        sky.sun = Sun::deg(0.0, 30.0);
        assert_eq!(sky.clone().dome(), Sky::new(sky.sun).dome());
        sky.haze = 4.0;
        assert_eq!(sky.dome(), Sky::new(sky.sun).haze(4.0).dome());
    }

    /// A distant cloud bank (here even an empty one) must not coarsen the
    /// march so much that a near heap drops out between samples.
    #[test]
    fn a_distant_cloud_does_not_hide_a_near_one() {
        let w = World::new([0.0, 0.0, 100.0, 100.0], 100.0, 2.0).fov(100.0, 50.0).sun(Sun::deg(0.0, 30.0));
        let sf = SkyField::new(Sky::new(w.sun), &w, 10.0);
        let near = Cloud::cumulus(0.0, 1000.0, 450.0, 100.0, 100.0, 5);
        let empty = Cloud::bank(-10000.0, 10000.0, 50000.0, 1000.0, 20000.0, 30000.0, 3).density(0.0);
        let far = Cloud::bank(-10000.0, 10000.0, 50000.0, 1000.0, 20000.0, 30000.0, 3);
        let a = Clouds::new(vec![near]).field(&sf, &w, 5.0);
        let b = Clouds::new(vec![near, empty]).field(&sf, &w, 5.0);
        let c = Clouds::new(vec![near, far]).field(&sf, &w, 5.0);
        assert!(a.alpha(50.0, 50.0) > 0.5, "{}", a.alpha(50.0, 50.0));
        let mut worst = 0.0f32;
        for y in 0..100 {
            for x in 0..100 {
                let (x, y) = (x as f32, y as f32);
                worst = worst.max((a.alpha(x, y) - b.alpha(x, y)).abs());
                // a real bank behind only adds cover
                assert!(c.alpha(x, y) >= a.alpha(x, y) - 1e-3, "({x},{y}) {} < {}", c.alpha(x, y), a.alpha(x, y));
            }
        }
        assert!(worst < 1e-4, "{worst}");
    }
}
