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
/// Score of a pile: (parts, masstone, scattering) → OKLab miss.
type Score<'a> = &'a dyn Fn(&[(usize, f32)], Rgb, f32) -> f32;

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
        let score = |_: &[(usize, f32)], c: Rgb, _s: f32| dist(to_oklab(c), lab);
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
        let robust = |p: &[(usize, f32)], c: Rgb, s: f32| AIM_SPREAD.iter().map(|&(t, w)| w * dist(look(c, s, coats * t), wl)).sum::<f32>() + parsimony(p);
        // coarse: every candidate at the expected thickness; the best few
        // judged over the spread of thicknesses; then refine the proportions
        let mut first: Vec<(f32, usize)> = self.cands.iter().enumerate().map(|(i, c)| (dist(look(c.color, c.scatter, coats), wl), i)).collect();
        let n = first.len().min(24);
        first.select_nth_unstable_by(n - 1, |a, b| a.0.total_cmp(&b.0));
        let best = first[..n].iter().map(|&(_, i)| (robust(&self.cands[i].parts, self.cands[i].color, self.cands[i].scatter), i)).min_by(|a, b| a.0.total_cmp(&b.0)).unwrap();
        let (parts, _) = self.refine(self.cands[best.1].parts.clone(), &robust);
        let (c, s, _) = self.eval(&parts);
        let m = self.mixture(parts, dist(look(c, s, coats), wl));
        self.aims.lock().unwrap().insert(key, m.clone());
        m
    }

    /// Refine proportions by coordinate descent with shrinking steps.
    fn refine(&self, mut parts: Vec<(usize, f32)>, score: Score) -> (Vec<(usize, f32)>, f32) {
        let sc = |p: &[(usize, f32)]| {
            let (c, s, _) = self.eval(p);
            score(p, c, s)
        };
        let mut s0 = sc(&parts);
        let mut step = 1.0 / GRID as f32;
        while step > 1e-3 {
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
            // a touch of a paint not yet in the pile (up to three tubes)
            if parts.len() < 3 {
                let big = (0..parts.len()).max_by(|&a, &b| parts[a].1.total_cmp(&parts[b].1)).unwrap();
                for t in 0..self.tubes.len() {
                    if parts.iter().any(|p| p.0 == t) || parts[big].1 < step {
                        continue;
                    }
                    let mut q = parts.clone();
                    q[big].1 -= step;
                    q.push((t, step));
                    let s = sc(&q);
                    if s < s0 {
                        s0 = s;
                        parts = q;
                        improved = true;
                        break;
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

    /// Number of cached (masstone mixes, aimed mixes): each is one search
    /// (~0.1 ms for an aim); for measuring cost.
    pub fn cache_sizes(&self) -> (usize, usize) {
        (self.cache.lock().unwrap().len(), self.aims.lock().unwrap().len())
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
        // falls faster than hiding). The paint carries S itself: hiding
        // rounds to 1 for strong scatterers and would lose it.
        Paint::km(self.color, self.scatter * k.max(1e-3), self.stiff * k * k)
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

/// A painter doesn't flick a touch of a third paint in and out of neighboring
/// piles: each tube in a pile costs a little (OKLab units), ramping in over
/// its first `PARSIMONY_RAMP` of the pile so the cost is continuous in the
/// proportions. Keeps aimed recipes in one family along a smooth passage.
const PARSIMONY: f32 = 0.004;
const PARSIMONY_RAMP: f32 = 0.1;

fn parsimony(parts: &[(usize, f32)]) -> f32 {
    PARSIMONY * parts.iter().map(|p| (p.1 / PARSIMONY_RAMP).min(1.0)).sum::<f32>()
}

fn dist(a: Rgb, b: Rgb) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{from_oklab, hex, lerp3};

    fn de(a: Rgb, b: Rgb) -> f32 {
        dist(to_oklab(a), to_oklab(b))
    }

    /// A smalt → lead-white sky over a smooth warm underlayer, `n` steps.
    fn sky_ramp(n: usize) -> Vec<(Rgb, Rgb)> {
        let (top, bottom) = (to_oklab(hex("#5f7398")), to_oklab(hex("#e2ddd0")));
        let (u0, u1) = (to_oklab(hex("#a9785a")), to_oklab(hex("#c4a482")));
        (0..=n).map(|k| {
            let t = k as f32 / n as f32;
            (from_oklab(lerp3(top, bottom, t)), from_oklab(lerp3(u0, u1, t)))
        }).collect()
    }

    /// Worst step (OKLab) between neighbors along the ramp in the look laid
    /// 0.3, 1 and 2.5 coats over the underlayer, and the worst miss at 1 coat.
    fn ramp_steps(pal: &Palette, ramp: &[(Rgb, Rgb)], aimed: bool) -> ([f32; 3], f32) {
        let (mut worst, mut miss) = ([0.0f32; 3], 0.0f32);
        let mut prev: Option<[Rgb; 3]> = None;
        for &(want, under) in ramp {
            let m = if aimed { pal.aim(want, under, 0.45, 1.0) } else { pal.mix(want) };
            let p = m.paint(0.45);
            let looks = [p.over(under, 0.3), p.over(under, 1.0), p.over(under, 2.5)];
            miss = miss.max(de(looks[1], want));
            if let Some(q) = prev {
                for i in 0..3 {
                    worst[i] = worst[i].max(de(looks[i], q[i]));
                }
            }
            prev = Some(looks);
        }
        (worst, miss)
    }

    /// A smooth smalt → lead-white sky over a smooth warm underlayer: aimed
    /// piles change no faster than the target does (no recipe seams), at
    /// any thickness the brush lays.
    #[test]
    fn smooth_targets_make_no_seams() {
        let full = Palette::friedrich_early();
        let fam = full.only(&["lead white", "smalt", "yellow ochre"]);
        let n = 80;
        let ramp = sky_ramp(n);
        let target_step = (0..n).map(|k| de(ramp[k].0, ramp[k + 1].0)).fold(0.0f32, f32::max);
        let (mix, _) = ramp_steps(&full, &ramp, false);
        let (aim, aim_miss) = ramp_steps(&full, &ramp, true);
        let (fam, _) = ramp_steps(&fam, &ramp, true);
        println!("target step {target_step:.4}; worst steps at 0.3/1/2.5 coats: masstone mix {mix:?}, aimed {aim:?} (miss {aim_miss:.3}), aimed family {fam:?}");
        for i in 0..3 {
            assert!(fam[i] < 2.75 * target_step, "family seam at thickness {i}: {fam:?}");
            assert!(aim[i] < 4.0 * target_step, "aimed seam at thickness {i}: {aim:?}");
        }
        // one-coat look: the aimed choice is at least twice as smooth as masstone mixing
        assert!(aim[1] < 0.5 * mix[1], "{aim:?} vs {mix:?}");
    }

    #[test]
    fn aim_hits_reachable_targets() {
        let pal = Palette::friedrich_1820();
        // a mid sky blue over a pale ground, thin paint: reachable
        let (want, under) = (hex("#8898b0"), hex("#d8cdb8"));
        let m = pal.aim(want, under, 0.45, 1.0);
        assert!(m.error < 0.02, "{} {}", m.error, pal.recipe(&m));
        // judged on the canvas, the pile is deeper than the look wanted (the pale
        // ground shows through a thin coat)
        assert!(to_oklab(m.color)[0] < to_oklab(want)[0]);
        // cached: same answer
        let m2 = pal.aim(want, under, 0.45, 1.0);
        assert_eq!(m.parts, m2.parts);
    }

    #[test]
    fn tint_round_trips_over_white() {
        let p = Paint::glaze(hex("#8a5a3a"));
        let o = p.over([1.0; 3], 1.0);
        assert!(de(o, hex("#8a5a3a")) < 0.005, "{o:?}");
        let a = Paint::aimed(hex("#7a8aa0"), hex("#c0a080"), 0.8, 0.5, 0.5);
        assert!(de(a.over(hex("#c0a080"), 0.8), hex("#7a8aa0")) < 0.005);
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;
    use crate::bristle::{Gesture, Held, Tool};
    use crate::color::{Mix, gradient, hex};
    use crate::mask::Mask;
    use crate::style::Style;

    fn de(a: Rgb, b: Rgb) -> f32 {
        dist(to_oklab(a), to_oklab(b))
    }

    fn mean(px: &[Rgb], idx: &[usize]) -> Rgb {
        let mut m = [0.0f32; 3];
        for &i in idx {
            for q in 0..3 {
                m[q] += px[i][q];
            }
        }
        let n = idx.len().max(1) as f32;
        [m[0] / n, m[1] / n, m[2] / n]
    }

    /// A painted sky field, dry: a pale-to-blue gradient laid with a broad
    /// brush over a warm ground.
    pub(super) fn sky_field(w: usize) -> Canvas {
        let st = Style::friedrich();
        let mut c = Canvas::new(w, 1.0, hex("#a9785a"));
        let all = Mask::from_fn(c.frame(), |_, _| 1.0);
        let stops = [(0.0, hex("#6f84a8")), (1.0, hex("#dcd6c4"))];
        c.work(&all, &st.broad().color(move |_, y| gradient(&stops, y / 1000.0, Mix::Pigment)).coverage(3.0).load(0.5), 11);
        c.dry();
        c
    }

    /// Dab a mark of `paint` at (x, y); return (ΔE of the region's mean,
    /// mean per-pixel ΔE) between before and after, over the mark.
    fn dab(c: &mut Canvas, paint: Paint, (x, y): (f32, f32), seed: u64) -> (f32, f32) {
        let before = c.pixels().to_vec();
        let f0 = c.film.clone();
        let mut b = Held::new(Tool::round_sable(14.0), seed);
        b.load(paint, 0.7);
        c.drag(&mut b, &Gesture::new(vec![(x, y), (x + 3.0, y + 2.0)]).pressure(0.8, 0.6), None);
        c.dry();
        let idx: Vec<usize> = (0..f0.len()).filter(|&i| c.film[i] - f0[i] > 0.05).collect();
        assert!(idx.len() > 10, "the dab laid paint");
        let px = c.pixels();
        let per = idx.iter().map(|&i| de(px[i], before[i])).sum::<f32>() / idx.len() as f32;
        (de(mean(px, &idx), mean(&before, &idx)), per)
    }

    #[test]
    fn matched_marks_disappear() {
        let st = Style::friedrich();
        let pal = &st.palette;
        let mut c = sky_field(500);
        let mut k = 0;
        for (label, medium) in [("body", 0.15), ("semi", 0.55), ("thin", 0.8)] {
            let (mut new, mut old) = ((0.0f32, 0.0f32), (0.0f32, 0.0f32));
            for j in 0..4 {
                let (x, y) = (150.0 + 230.0 * j as f32, 120.0 + 250.0 * j as f32);
                let want = c.under(x, y, 7.0);
                let p = c.aim(pal, want, (x, y), 7.0, medium, 0.7);
                let (a, b) = dab(&mut c, p, (x, y), 40 + k);
                new = (new.0.max(a), new.1.max(b));
                // the old meaning: the same pile, its masstone read as its look over white
                let m = pal.mix(want).paint(medium);
                let q = Paint::tint(m.color, m.hiding(), m.stiff);
                let (a, b) = dab(&mut c, q, (x + 60.0, y), 80 + k);
                old = (old.0.max(a), old.1.max(b));
                k += 1;
            }
            println!("{label}: aimed ΔE mean {:.4} per-px {:.4} | old ΔE mean {:.4} per-px {:.4}", new.0, new.1, old.0, old.1);
            // a JND in OKLab is about 0.02; aimed marks stay well under it
            assert!(new.0 < 0.008 && new.1 < 0.012, "{label}: aimed mark shows: {new:?}");
            if medium > 0.5 {
                assert!(old.0 > 3.0 * new.0, "{label}: the old reading should have shown: {old:?} vs {new:?}");
            }
        }
    }

    /// A transparent dark glaze deepens a light passage, barely shows on a
    /// dark one, deepens with thickness; aiming a light glaze at a dark
    /// ground can't lighten it (physics is not faked).
    #[test]
    fn glazes_stay_glazes() {
        let st = Style::friedrich();
        let pal = &st.palette;
        let (light, dark) = (hex("#d8d0bc"), hex("#2a2622"));
        let mut c = Canvas::new(300, 1.0, light);
        c.prime(dark, 0.95, 0.0, 0.5, 0.0, 1); // no-op thickness: keep the light ground
        let f = c.frame();
        let left = Mask::from_fn(f, |x, _| if x < 500.0 { 1.0 } else { 0.0 });
        c.apply_masked(&left, |_, _, _, _| dark);
        let all = Mask::from_fn(f, |_, _| 1.0);
        c.work(&all, &st.glaze(0.9).color(|_, _| hex("#4a2f1c")).angle(|_, _| 1.5708).coverage(3.0), 3);
        c.dry();
        let region = |x0: f32, x1: f32| -> Vec<usize> { (0..f.w * f.h).filter(|&i| { let x = (i % f.w) as f32 / f.scale; x > x0 && x < x1 }).collect() };
        let (on_dark, on_light) = (mean(c.pixels(), &region(100.0, 400.0)), mean(c.pixels(), &region(600.0, 900.0)));
        let (dl, ll) = (to_oklab(on_dark)[0] - to_oklab(dark)[0], to_oklab(on_light)[0] - to_oklab(light)[0]);
        println!("glaze: over light ΔL {ll:.3}, over dark ΔL {dl:.3}");
        assert!(ll < -0.05, "a dark glaze deepens a light passage: {ll}");
        assert!(dl.abs() < 0.3 * ll.abs(), "and barely shows on a dark one: {dl} vs {ll}");
        // deeper with thickness
        let g = pal.paint(hex("#4a2f1c"), 0.9);
        let (l1, l2) = (to_oklab(g.over(light, 1.0))[0], to_oklab(g.over(light, 2.0))[0]);
        assert!(l2 < l1 && l1 < to_oklab(light)[0]);
        // aim a pale transparent glaze (hiding 0.07) at a dark ground: as
        // pale as such a glaze can get, no paler (one coat of it reflects 7%
        // over black: a faint veil, far from the target). (A lead-white or ochre veil
        // is another matter: they scatter, and a thin milky scumble does lift
        // a dark; the palette's aim finds those when asked.)
        let want = hex("#c8c0b0");
        let g = Paint::aimed(want, dark, 1.0, 0.07, 0.3);
        let got = g.over(dark, 1.0);
        println!("pale glaze over dark: got L {:.3} want {:.3} dark {:.3}", to_oklab(got)[0], to_oklab(want)[0], to_oklab(dark)[0]);
        let (gl, wl, dl) = (to_oklab(got)[0], to_oklab(want)[0], to_oklab(dark)[0]);
        assert!(gl - dl < 0.5 * (wl - dl), "a transparent glaze gets nowhere near covering a dark ground: {gl} {wl} {dl}");
        let veil = pal.aim(want, dark, 0.9, 1.0);
        assert!(veil.error > 0.1, "a thin veil can't reach it either: {}", veil.error);
        // the full palette thick, as body color, can
        let body = pal.aim(want, dark, 0.1, 2.0);
        assert!(body.error < 0.05, "body color covers: {}", body.error);
    }

    /// Brush paint keeps a mixture's scattering, even when it rounds to
    /// hiding 1 (the review's repro: opaque neutral tubes mixed to 0.1).
    #[test]
    fn mixture_to_paint_preserves_scattering() {
        let pal = Palette::new("opaque neutral tubes", vec![
            Tube { name: "white", color: [0.99; 3], hiding: 0.99, stiff: 0.5, strength: 1.0 },
            Tube { name: "black", color: [0.01; 3], hiding: 0.99, stiff: 0.5, strength: 1.0 },
        ]);
        for (target, medium) in [([0.1; 3], 0.0), ([0.1; 3], 0.5), ([0.6; 3], 0.0), ([0.6; 3], 0.9)] {
            let m = pal.mix(target);
            let p = m.paint(medium);
            let s = m.scatter * (1.0 - medium);
            assert!((p.scatter() - s).abs() <= 1e-4 * s, "S {} thinned {s} → paint S {}", m.scatter, p.scatter());
            let expected = Pigment::masstone(m.color, s).over([1.0; 3], 0.1);
            let got = p.over([1.0; 3], 0.1);
            assert!((expected[0] - got[0]).abs() < 1e-3, "{target:?} medium {medium}: expected {expected:?} got {got:?}");
            assert!((p.hiding() - hiding_of(luminance(m.color), s)).abs() < 1e-4, "hiding is reported from S");
        }
        // and the brush lays that scattering into the wet layer
        let p = pal.mix([0.1; 3]).paint(0.0);
        let mut c = Canvas::new(100, 1.0, [1.0; 3]);
        let mut b = crate::bristle::Held::new(crate::bristle::Tool::round_sable(14.0), 1);
        b.load(p, 0.7);
        c.drag(&mut b, &crate::bristle::Gesture::new(vec![(50.0, 50.0), (53.0, 52.0)]).pressure(0.8, 0.6), None);
        let i = (0..c.wet.vol.len()).max_by(|&a, &b| c.wet.vol[a].total_cmp(&c.wet.vol[b])).unwrap();
        assert!(c.wet.vol[i] > 0.0);
        let laid = c.wet.hide[i][0];
        assert!((laid - p.scatter()).abs() <= 1e-3 * p.scatter(), "wet S {laid} vs paint S {}", p.scatter());
    }
}
