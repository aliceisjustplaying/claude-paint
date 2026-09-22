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

use crate::canvas::Canvas;
use crate::color::{Rgb, hex, luminance, to_oklab};
use crate::pigment::{Pigment, hiding_of, scatter_for};
use crate::rng::Rng;
use crate::wet::Paint;
use std::collections::HashMap;
use std::sync::Mutex;

/// A tube (or hand-ground) paint.
#[derive(Clone, Debug)]
pub struct Tube {
    pub name: &'static str,
    /// Masstone color, linear RGB (the paint laid thick).
    pub color: Rgb,
    /// Hiding power of one coat of the tube paint (0 transparent .. 1
    /// opaque): contrast ratio, see `pigment::hiding_of`.
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
    /// Masstone of the mixture (Mixbox mix of the tubes' masstones, weighted
    /// by volume × tinting strength).
    pub color: Rgb,
    /// Hiding of one coat of the unthinned mixture (derived from `scatter`).
    pub hiding: f32,
    /// Kubelka–Munk scattering per coat: the tubes' scattering mixed by
    /// volume (two-constant KM mixing).
    pub scatter: f32,
    pub stiff: f32,
    /// OKLab distance from the color asked for (for `aim`: from the look
    /// asked for, at the expected thickness over the underlayer).
    pub error: f32,
}

/// A candidate on the coarse search grid (precomputed per palette).
#[derive(Clone, Debug)]
struct Cand {
    parts: Vec<(usize, f32)>,
    color: Rgb,
    lab: Rgb,
    scatter: f32,
}

type AimKey = [i32; 8];

pub struct Palette {
    pub name: &'static str,
    pub tubes: Vec<Tube>,
    lat: Vec<[f32; mixbox::LATENT_SIZE]>,
    /// Scattering per coat of each tube paint.
    scat: Vec<f32>,
    cands: Vec<Cand>,
    cache: Mutex<HashMap<[i32; 3], Mixture>>,
    aims: Mutex<HashMap<AimKey, Mixture>>,
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

/// Coarse grid: proportions in twelfths.
const GRID: usize = 12;
/// Quantization of cache keys: OKLab steps of the color asked for and of the
/// underlayer, thickness and medium steps.
const Q_WANT: f32 = 400.0;
const Q_UNDER: f32 = 100.0;
const Q_COATS: f32 = 16.0;
const Q_MEDIUM: f32 = 50.0;
/// Thicknesses (× the expected one) and weights at which an aimed pile is
/// judged: a stroke lays paint thinner at its edges and in dry-brush, thicker
/// where it starts; a pile that only looks right at exactly one thickness
/// would show every change of thickness (and every change of recipe).
const AIM_SPREAD: [(f32, f32); 3] = [(0.5, 0.25), (1.0, 0.5), (2.0, 0.25)];

impl Palette {
    pub fn new(name: &'static str, tubes: Vec<Tube>) -> Self {
        let lat = tubes.iter().map(|t| mixbox::linear_float_rgb_to_latent(&t.color)).collect();
        let scat = tubes.iter().map(|t| scatter_for(luminance(t.color), t.hiding)).collect();
        let mut p = Palette { name, tubes, lat, scat, cands: Vec::new(), cache: Mutex::new(HashMap::new()), aims: Mutex::new(HashMap::new()) };
        p.cands = p.grid();
        p
    }

    /// The same palette restricted to the named tubes: a painter setting out
    /// the few paints for one passage (lead white, smalt and a touch of
    /// ochre for a sky) keeps its mixtures in one family, so neighboring
    /// piles never jump between unrelated recipes. Panics on unknown names.
    pub fn only(&self, names: &[&str]) -> Palette {
        let tubes = names
            .iter()
            .map(|n| self.tubes.iter().find(|t| t.name == *n).unwrap_or_else(|| panic!("no tube {n:?} in palette {}", self.name)).clone())
            .collect();
        Palette::new(self.name, tubes)
    }

    /// Every single tube, pair and triple on a grid of twelfths.
    fn grid(&self) -> Vec<Cand> {
        let n = self.tubes.len();
        let g = GRID;
        let mut parts: Vec<Vec<(usize, f32)>> = Vec::new();
        for i in 0..n {
            parts.push(vec![(i, 1.0)]);
            for j in i + 1..n {
                for a in 1..g {
                    parts.push(vec![(i, a as f32 / g as f32), (j, 1.0 - a as f32 / g as f32)]);
                }
                for k in j + 1..n {
                    for a in 1..g {
                        for b in 1..g - a {
                            let (fa, fb) = (a as f32 / g as f32, b as f32 / g as f32);
                            parts.push(vec![(i, fa), (j, fb), (k, 1.0 - fa - fb)]);
                        }
                    }
                }
            }
        }
        parts
            .into_iter()
            .map(|p| {
                let (color, scatter, _) = self.eval(&p);
                Cand { lab: to_oklab(color), color, scatter, parts: p }
            })
            .collect()
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

    /// Masstone, scattering per coat and stiffness of a mixture.
    fn eval(&self, parts: &[(usize, f32)]) -> (Rgb, f32, f32) {
        let mut lat = [0.0f32; mixbox::LATENT_SIZE];
        let (mut wsum, mut sct, mut stf) = (0.0, 0.0, 0.0);
        for &(i, f) in parts {
            let w = f * self.tubes[i].strength;
            for k in 0..mixbox::LATENT_SIZE {
                lat[k] += self.lat[i][k] * w;
            }
            wsum += w;
            sct += self.scat[i] * f;
            stf += self.tubes[i].stiff * f;
        }
        for v in lat.iter_mut() {
            *v /= wsum.max(1e-9);
        }
        (mixbox::latent_to_linear_float_rgb(&lat), sct, stf)
    }

    fn mixture(&self, parts: Vec<(usize, f32)>, error: f32) -> Mixture {
        let (color, scatter, stiff) = self.eval(&parts);
        Mixture { hiding: hiding_of(luminance(color), scatter), parts, color, scatter, stiff, error }
    }

    /// The mixture of up to three tubes whose masstone is closest to
    /// `target` (linear RGB). This is the pile that, laid thick or over paint
    /// of its own color, looks `target`; to judge a pile by how it will look
    /// over what is already on the canvas, use `aim`.
    pub fn mix(&self, target: Rgb) -> Mixture {
        let lab = to_oklab(target);
        let key = [(lab[0] * Q_WANT).round() as i32, (lab[1] * Q_WANT).round() as i32, (lab[2] * Q_WANT).round() as i32];
        if let Some(m) = self.cache.lock().unwrap().get(&key) {
            return m.clone();
        }
        // search from the key's center, so the result doesn't depend on which
        // nearby color happened to be asked for first
        let lab = [key[0] as f32 / Q_WANT, key[1] as f32 / Q_WANT, key[2] as f32 / Q_WANT];
        let score = |c: Rgb, _s: f32| dist(to_oklab(c), lab);
        let best = self.cands.iter().min_by(|a, b| dist(a.lab, lab).total_cmp(&dist(b.lab, lab))).expect("empty palette");
        let (parts, e) = self.refine(best.parts.clone(), &score);
        let m = self.mixture(parts, e);
        self.cache.lock().unwrap().insert(key, m.clone());
        m
    }

    /// Aim at the result: the mixture that, thinned with `medium` and laid
    /// `coats` thick over `under` (what is on the canvas there, see
    /// `Canvas::under`), looks most like `want`. A painter judges a pile by
    /// eye on the canvas, not by its masstone: a thin scumble meant to read
    /// as a pale sky over a darker underpainting must be mixed paler than
    /// the sky, a glaze deeper. Piles are judged over a spread of
    /// thicknesses around `coats` (see `AIM_SPREAD`), so the choice favors
    /// piles that look right however thick the brush lays them; this also
    /// keeps neighboring targets from flipping between recipes. Targets no
    /// pile can reach (a light glaze over a dark ground) come out as the
    /// nearest the painter could get. `error` is the OKLab miss at `coats`.
    pub fn aim(&self, want: Rgb, under: Rgb, medium: f32, coats: f32) -> Mixture {
        let (wl, ul) = (to_oklab(want), to_oklab(under));
        let q = |v: f32, s: f32| (v * s).round() as i32;
        let key: AimKey = [q(wl[0], Q_WANT), q(wl[1], Q_WANT), q(wl[2], Q_WANT), q(ul[0], Q_UNDER), q(ul[1], Q_UNDER), q(ul[2], Q_UNDER), q(coats.max(0.0), Q_COATS), q(medium.clamp(0.0, 1.0), Q_MEDIUM)];
        if let Some(m) = self.aims.lock().unwrap().get(&key) {
            return m.clone();
        }
        let wl = [key[0] as f32 / Q_WANT, key[1] as f32 / Q_WANT, key[2] as f32 / Q_WANT];
        let under = crate::color::from_oklab([key[3] as f32 / Q_UNDER, key[4] as f32 / Q_UNDER, key[5] as f32 / Q_UNDER]);
        let coats = (key[6] as f32 / Q_COATS).max(1.0 / Q_COATS);
        let dil = 1.0 - key[7] as f32 / Q_MEDIUM;
        let look = |c: Rgb, s: f32, x: f32| to_oklab(Pigment::masstone(c, s * dil).over(under, x));
        let robust = |c: Rgb, s: f32| AIM_SPREAD.iter().map(|&(t, w)| w * dist(look(c, s, coats * t), wl)).sum::<f32>();
        // coarse: every candidate at the expected thickness; the best few
        // judged over the spread of thicknesses; then refine the proportions
        let mut first: Vec<(f32, usize)> = self.cands.iter().enumerate().map(|(i, c)| (dist(look(c.color, c.scatter, coats), wl), i)).collect();
        let n = first.len().min(24);
        first.select_nth_unstable_by(n - 1, |a, b| a.0.total_cmp(&b.0));
        let best = first[..n].iter().map(|&(_, i)| (robust(self.cands[i].color, self.cands[i].scatter), i)).min_by(|a, b| a.0.total_cmp(&b.0)).unwrap();
        let (parts, _) = self.refine(self.cands[best.1].parts.clone(), &robust);
        let (c, s, _) = self.eval(&parts);
        let m = self.mixture(parts, dist(look(c, s, coats), wl));
        self.aims.lock().unwrap().insert(key, m.clone());
        m
    }

    /// Refine proportions by coordinate descent with shrinking steps.
    fn refine(&self, mut parts: Vec<(usize, f32)>, score: &dyn Fn(Rgb, f32) -> f32) -> (Vec<(usize, f32)>, f32) {
        let sc = |p: &[(usize, f32)]| {
            let (c, s, _) = self.eval(p);
            score(c, s)
        };
        let mut s0 = sc(&parts);
        let mut step = 1.0 / GRID as f32;
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
                    let s = sc(&q);
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
        (parts, s0)
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
        Mixture { error: m.error, ..self.mixture(parts, 0.0) }
    }

    /// A paint mixed to `target` (masstone), thinned: `medium` 0..1 is the
    /// fraction of oil medium added (it lowers hiding and stiffness).
    pub fn paint(&self, target: Rgb, medium: f32) -> Paint {
        let m = self.mix(target);
        m.paint(medium)
    }

    /// Aimed paint: `aim` then thin it (see `aim`).
    pub fn paint_for(&self, want: Rgb, under: Rgb, medium: f32, coats: f32) -> Paint {
        self.aim(want, under, medium, coats).paint(medium)
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
        // medium dilutes the pigment: K and S per coat fall with the pigment
        // concentration, the masstone stays; the paint flows (stiffness
        // falls faster than hiding)
        Paint { color: self.color, hiding: hiding_of(luminance(self.color), self.scatter * k.max(1e-3)), stiff: self.stiff * k * k }
    }
}

impl Canvas {
    /// Aim at the result, for a hand-made mark at (`x`, `y`) of radius `r`
    /// (units): the pile from `pal` that, thinned with `medium` and laid
    /// `coats` thick over what is on the canvas there, looks `want`. See
    /// `Palette::aim`. Cheap enough to call per mark (results are cached).
    #[allow(clippy::too_many_arguments)]
    pub fn aim(&self, pal: &Palette, want: Rgb, (x, y): (f32, f32), r: f32, medium: f32, coats: f32) -> Paint {
        pal.paint_for(want, self.under(x, y, r), medium, coats)
    }
}

fn dist(a: Rgb, b: Rgb) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}
