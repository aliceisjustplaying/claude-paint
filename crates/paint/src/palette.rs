//! The painter's palette: a few tube paints, and mixing on the palette.
//!
//! A painting asks for a color; the painter can only mix it from the tubes
//! they own. `Palette::mix` searches mixtures of up to three tube paints (in
//! proportions a painter would knife together) for the one that looks
//! closest (OKLab distance), mixing the way pigments mix (Mixbox latent
//! space, weighted by each paint's tinting strength). Colors the palette
//! cannot reach come out as the nearest color the painter could actually
//! make: the palette limits the gamut, as it did for the painter.
//!
//! Masstone colors, hiding and tinting strength are approximations from
//! pigment knowledge (not measurements); which pigments a painter owned is
//! sourced per palette.

use crate::color::{Rgb, hex, to_oklab};
use crate::rng::Rng;
use crate::wet::Paint;
use std::collections::HashMap;
use std::sync::Mutex;

/// A tube (or hand-ground) paint.
#[derive(Clone, Debug)]
pub struct Tube {
    pub name: &'static str,
    /// Masstone color, linear RGB.
    pub color: Rgb,
    /// Hiding power (0 transparent .. 1 opaque).
    pub hiding: f32,
    /// Stiffness straight from the tube (0 fluid .. 1 stiff).
    pub stiff: f32,
    /// Tinting strength relative to an average pigment (smalt is weak,
    /// Prussian blue very strong).
    pub strength: f32,
}

fn tube(name: &'static str, color: &str, hiding: f32, stiff: f32, strength: f32) -> Tube {
    Tube { name, color: hex(color), hiding, stiff, strength }
}

/// A mixture on the palette: parts of tubes.
#[derive(Clone, Debug)]
pub struct Mixture {
    /// (tube index, fraction by volume), fractions sum to 1.
    pub parts: Vec<(usize, f32)>,
    pub color: Rgb,
    pub hiding: f32,
    pub stiff: f32,
    /// OKLab distance from the color asked for.
    pub error: f32,
}

pub struct Palette {
    pub name: &'static str,
    pub tubes: Vec<Tube>,
    lat: Vec<[f32; mixbox::LATENT_SIZE]>,
    cache: Mutex<HashMap<[i32; 3], Mixture>>,
}

impl Clone for Palette {
    fn clone(&self) -> Self {
        Palette::new(self.name, self.tubes.clone())
    }
}

impl std::fmt::Debug for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Palette").field("name", &self.name).field("tubes", &self.tubes.iter().map(|t| t.name).collect::<Vec<_>>()).finish()
    }
}

impl Palette {
    pub fn new(name: &'static str, tubes: Vec<Tube>) -> Self {
        let lat = tubes.iter().map(|t| mixbox::linear_float_rgb_to_latent(&t.color)).collect();
        Palette { name, tubes, lat, cache: Mutex::new(HashMap::new()) }
    }

    /// Friedrich before ~1820: lead white, smalt (semi-transparent cobalt
    /// glass, weak, in several grades), ochres and earths, vermilion, umber,
    /// bone black (notes/research/friedrich_materials.md §4: CATS p.127,
    /// NG pp.51–56, ALF p.348). Naples yellow and Prussian blue are uncertain
    /// and left out.
    pub fn friedrich_early() -> Self {
        Palette::new(
            "Friedrich, early",
            vec![
                tube("lead white", "#efe9dc", 0.82, 0.8, 1.0),
                tube("smalt", "#5a6e9e", 0.3, 0.55, 0.45),
                tube("pale smalt", "#8d9bb8", 0.35, 0.55, 0.35),
                tube("yellow ochre", "#b98a36", 0.8, 0.7, 0.8),
                tube("red earth", "#9c4a30", 0.85, 0.7, 0.9),
                tube("vermilion", "#cf3a24", 0.9, 0.75, 1.0),
                tube("raw umber", "#5c4c3a", 0.8, 0.65, 0.9),
                tube("bone black", "#1e1b19", 0.9, 0.7, 1.1),
            ],
        )
    }

    /// Friedrich from ~1820: cobalt blue and chrome yellow join the palette
    /// and largely replace smalt (ALF pp.341, 348–349; NG p.56).
    pub fn friedrich_1820() -> Self {
        let mut t = Palette::friedrich_early().tubes;
        t.retain(|t| t.name != "smalt");
        t.push(tube("cobalt blue", "#2f55a8", 0.55, 0.6, 0.8));
        t.push(tube("chrome yellow", "#e8b21c", 0.9, 0.7, 1.0));
        Palette::new("Friedrich, after 1820", t)
    }

    fn eval(&self, parts: &[(usize, f32)]) -> (Rgb, f32, f32) {
        let mut lat = [0.0f32; mixbox::LATENT_SIZE];
        let (mut wsum, mut hid, mut stf) = (0.0, 0.0, 0.0);
        for &(i, f) in parts {
            let w = f * self.tubes[i].strength;
            for k in 0..mixbox::LATENT_SIZE {
                lat[k] += self.lat[i][k] * w;
            }
            wsum += w;
            hid += self.tubes[i].hiding * f;
            stf += self.tubes[i].stiff * f;
        }
        for v in lat.iter_mut() {
            *v /= wsum.max(1e-9);
        }
        (mixbox::latent_to_linear_float_rgb(&lat), hid, stf)
    }

    fn mixture(&self, parts: Vec<(usize, f32)>, target_lab: Rgb) -> Mixture {
        let (color, hiding, stiff) = self.eval(&parts);
        Mixture { error: dist(to_oklab(color), target_lab), parts, color, hiding, stiff }
    }

    /// The closest mixture of up to three tubes to `target` (linear RGB).
    pub fn mix(&self, target: Rgb) -> Mixture {
        let lab = to_oklab(target);
        let key = [(lab[0] * 400.0) as i32, (lab[1] * 400.0) as i32, (lab[2] * 400.0) as i32];
        if let Some(m) = self.cache.lock().unwrap().get(&key) {
            return m.clone();
        }
        let m = self.search(lab);
        self.cache.lock().unwrap().insert(key, m.clone());
        m
    }

    fn search(&self, lab: Rgb) -> Mixture {
        let n = self.tubes.len();
        let score = |parts: &[(usize, f32)]| dist(to_oklab(self.eval(parts).0), lab);
        // coarse: every pair and triple on a grid of twelfths
        let g = 12;
        let mut best: (f32, Vec<(usize, f32)>) = (f32::MAX, vec![]);
        for i in 0..n {
            let p = vec![(i, 1.0)];
            let s = score(&p);
            if s < best.0 {
                best = (s, p);
            }
            for j in i + 1..n {
                for a in 1..g {
                    let p = vec![(i, a as f32 / g as f32), (j, 1.0 - a as f32 / g as f32)];
                    let s = score(&p);
                    if s < best.0 {
                        best = (s, p);
                    }
                }
                for k in j + 1..n {
                    for a in 1..g {
                        for b in 1..g - a {
                            let (fa, fb) = (a as f32 / g as f32, b as f32 / g as f32);
                            let p = vec![(i, fa), (j, fb), (k, 1.0 - fa - fb)];
                            let s = score(&p);
                            if s < best.0 {
                                best = (s, p);
                            }
                        }
                    }
                }
            }
        }
        // refine the proportions by coordinate descent with shrinking steps
        let mut parts = best.1;
        let mut s0 = best.0;
        let mut step = 1.0 / g as f32;
        while step > 1e-3 && parts.len() > 1 {
            let mut improved = false;
            for a in 0..parts.len() {
                for b in 0..parts.len() {
                    if a == b || parts[b].1 < step {
                        continue;
                    }
                    let mut q = parts.clone();
                    q[a].1 += step;
                    q[b].1 -= step;
                    let s = score(&q);
                    if s < s0 {
                        s0 = s;
                        parts = q;
                        improved = true;
                    }
                }
            }
            if !improved {
                step *= 0.5;
            }
        }
        parts.retain(|p| p.1 > 1e-4);
        self.mixture(parts, lab)
    }

    /// The painter never mixes the same pile twice: jitter the proportions
    /// (relative sd `amount`) and remix.
    pub fn remix(&self, m: &Mixture, amount: f32, rng: &mut Rng) -> Mixture {
        if amount <= 0.0 || m.parts.len() < 2 {
            return m.clone();
        }
        let mut parts: Vec<(usize, f32)> = m.parts.iter().map(|&(i, f)| (i, (f * (1.0 + rng.normal() * amount)).max(0.0))).collect();
        let s: f32 = parts.iter().map(|p| p.1).sum();
        parts.iter_mut().for_each(|p| p.1 /= s.max(1e-9));
        let (color, hiding, stiff) = self.eval(&parts);
        Mixture { parts, color, hiding, stiff, error: m.error }
    }

    /// A paint mixed to `target`, thinned: `medium` 0..1 is the fraction of
    /// oil medium added (it lowers hiding and stiffness).
    pub fn paint(&self, target: Rgb, medium: f32) -> Paint {
        let m = self.mix(target);
        m.paint(medium)
    }

    /// Human-readable recipe, e.g. "lead white 0.72 + yellow ochre 0.20 + raw umber 0.08".
    pub fn recipe(&self, m: &Mixture) -> String {
        m.parts.iter().map(|&(i, f)| format!("{} {:.2}", self.tubes[i].name, f)).collect::<Vec<_>>().join(" + ")
    }
}

impl Mixture {
    /// This mixture as paint on the brush, thinned with `medium` (0..1).
    pub fn paint(&self, medium: f32) -> Paint {
        let k = (1.0 - medium).clamp(0.0, 1.0);
        // medium dilutes the pigment (less hiding per unit thickness) and makes
        // the paint flow (stiffness falls faster than hiding)
        Paint { color: self.color, hiding: (self.hiding * (0.5 + 0.5 * k)).clamp(0.02, 0.99), stiff: self.stiff * k * k }
    }
}

fn dist(a: Rgb, b: Rgb) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}
