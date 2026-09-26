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

type AimKey = [i32; 9];
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
/// where it starts (a stroke's film runs from about 0.6× its median to 2.5×
/// at the 90th percentile, `handling::tests::probe_laid_by_coverage`); a
/// pile that only looks right at exactly one thickness would show every
/// change of thickness (and every change of recipe). The 4× point stands
/// for the pile's own color where it lands thick: a light mark over a dark
/// mixed from lead white and red earth hits its target at the expected
/// thickness and dries salmon wherever it lays thicker.
const AIM_SPREAD: [(f32, f32); 4] = [(0.5, 0.2), (1.0, 0.45), (2.0, 0.25), (4.0, 0.1)];
/// Thin edges (this × the expected thickness) should lie on the way from
/// the underlayer to the look wanted: a pile whose thin film swings off
/// that line (a red rim, a milky blue veil) is penalized by its distance
/// from it, with this weight.
const AIM_THIN: (f32, f32) = (0.25, 0.3);
/// A painter keeps a pile in the family of the color wanted: its own color
/// (masstone) costs this much per unit of OKLab a/b distance from the look
/// wanted. Aim may still push a pile's hue against the underlayer, but only
/// where that buys a real improvement in the look.
const AIM_FAMILY: f32 = 0.28;
/// How a mark's area is spread over thicknesses (× the expected one,
/// share of the area), as the eye averages it at viewing distance: the
/// aim makes this mean look (in linear light) the look wanted. Measured by
/// `handling::tests::probe_mark_thickness` (sparse marks, Friedrich ground,
/// 1000px): a blunt brush (filbert, flat) lays most of its mark near 1–2×,
/// a pointed one (round sable, rigger) a thick core with thin, semi-
/// transparent edges and tails (half its area at ½× or less), which over a
/// dark dry darker than the core.
const MARKS_BLUNT: [(f32, f32); 6] = [(0.125, 0.04), (0.25, 0.045), (0.5, 0.1), (1.0, 0.29), (2.0, 0.46), (4.0, 0.065)];
const MARKS_POINTED: [(f32, f32); 5] = [(0.125, 0.13), (0.25, 0.14), (0.5, 0.21), (1.0, 0.37), (2.0, 0.15)];
/// Weight of the per-thickness spread (`AIM_SPREAD`) next to the mean look:
/// it keeps a pile from looking right on average only by being far off
/// at every thickness (and neighboring targets on one recipe).
const AIM_ROBUST: f32 = 0.5;

/// The shape of the marks an aimed pile will make (see `Palette::aim_for`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Marks {
    /// A blunt brush: most of the mark near the expected thickness.
    Blunt,
    /// A pointed brush: a thick core, thin edges and tails.
    Pointed,
}

impl Marks {
    /// The marks a tool makes.
    pub fn of(tool: &crate::bristle::Tool) -> Self {
        if tool.point > 0.0 { Marks::Pointed } else { Marks::Blunt }
    }
    fn spread(self) -> &'static [(f32, f32)] {
        match self {
            Marks::Blunt => &MARKS_BLUNT,
            Marks::Pointed => &MARKS_POINTED,
        }
    }
}

/// How many of the best candidates at the expected thickness are judged in
/// full (spread, thin edge and family).
const AIM_SHORTLIST: usize = 48;

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
    /// NG pp.51–56, ALF p.348). Naples yellow is left out. For greens,
    /// see `friedrich_early_greens`.
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
    /// and largely replace smalt (ALF pp.341, 348–349; NG p.56). For
    /// greens, see `friedrich_1820_greens`.
    pub fn friedrich_1820() -> Self {
        let mut t = Palette::friedrich_early().tubes;
        t.retain(|t| t.name != "smalt");
        t.push(tube("cobalt blue", "#2f55a8", 0.55, 0.6, 0.8));
        t.push(tube("chrome yellow", "#e8b21c", 0.9, 0.7, 1.0));
        Palette::new("Friedrich, after 1820", t)
    }

    /// The tubes Friedrich added for his greens
    /// (notes/research/friedrich_materials.md §9): Prussian blue, which he
    /// used "very often" to mix greens with ochre, Naples yellow or chrome
    /// yellow, and green earth, one of his few true green pigments
    /// [MÄD p.102]. Masstones and numbers are documented approximations:
    /// Prussian blue transparent and very strong [AP3 pp.196–197] (tinting
    /// strength 3, below the sourced "very high", because Mixbox's latent
    /// already carries some of a dark pigment's strength); green earth
    /// translucent, weak, short of body [AP1 p.146; FIELD p.129], its
    /// masstone from Munsell 7.5G/2.9/1.5 [AP1 Table 1].
    pub fn green_tubes() -> Vec<Tube> {
        vec![tube("Prussian blue", "#172440", 0.35, 0.45, 3.0), tube("green earth", "#3a4843", 0.2, 0.35, 0.3)]
    }

    /// `friedrich_early` with his green tubes (`green_tubes`), for green
    /// passages: meadows, foliage, summer. Kept apart from the base palette
    /// because the aimed search would otherwise pick very strong Prussian
    /// blue for skies (the sky ramp's miss halves but its recipes seam; and
    /// his skies are smalt or cobalt [ALF; NPJ25]). Set out a sky family
    /// with `only` when painting a sky from this palette.
    pub fn friedrich_early_greens() -> Self {
        Palette::friedrich_early().with(Palette::green_tubes()).named("Friedrich, early, greens")
    }

    /// `friedrich_1820` with his green tubes and Rinmann's green
    /// (cobalt-zinc oxide: semi-transparent, weak, permanent [WEB-co]),
    /// found, rarely, in paintings of c.1819–23 [MÄD p.102 n.3].
    pub fn friedrich_1820_greens() -> Self {
        let mut t = Palette::green_tubes();
        t.push(tube("Rinmann's green", "#5f8f76", 0.35, 0.5, 0.4));
        Palette::friedrich_1820().with(t).named("Friedrich, after 1820, greens")
    }

    /// The same tubes under another name.
    pub fn named(self, name: &'static str) -> Palette {
        Palette { name, ..self }
    }

    /// A copper green (verdigris ground in oil): one of the "copper-containing"
    /// true greens found in Friedrich's Dresden paintings [MÄD p.102], used
    /// sparingly; not in the standard palettes (add it with `with`).
    /// Masstone and numbers are assumptions; "poor hiding power in oil"
    /// [AP2 p.132] (notes/research/friedrich_materials.md §9).
    pub fn copper_green() -> Tube {
        tube("copper green", "#3f7f6a", 0.25, 0.4, 1.0)
    }

    /// This palette with more tubes (a rare paint for one passage).
    pub fn with(&self, extra: Vec<Tube>) -> Palette {
        let mut t = self.tubes.clone();
        t.extend(extra);
        Palette::new(self.name, t)
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

    /// The pile knifed from these parts (tube index, fraction by volume;
    /// fractions sum to 1), mixed the way the palette mixes: its masstone,
    /// scattering and stiffness (`error` is 0).
    pub fn pile(&self, parts: Vec<(usize, f32)>) -> Mixture {
        self.mixture(parts, 0.0)
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
    /// nearest the painter could get. `error` is the OKLab miss of the
    /// mark's mean look (see `aim_for`).
    pub fn aim(&self, want: Rgb, under: Rgb, medium: f32, coats: f32) -> Mixture {
        self.aim_for(want, under, medium, coats, Marks::Blunt)
    }

    /// `aim` for the marks of a particular brush: the mean look over the
    /// mark (its thick core and thin edges, `Marks`) is what should come out
    /// as `want`. A pointed brush's light marks over a dark get a lighter
    /// pile than a filbert's, since their thin edges dry darker.
    pub fn aim_for(&self, want: Rgb, under: Rgb, medium: f32, coats: f32, marks: Marks) -> Mixture {
        let (wl, ul) = (to_oklab(want), to_oklab(under));
        let q = |v: f32, s: f32| (v * s).round() as i32;
        let key: AimKey = [q(wl[0], Q_WANT), q(wl[1], Q_WANT), q(wl[2], Q_WANT), q(ul[0], Q_UNDER), q(ul[1], Q_UNDER), q(ul[2], Q_UNDER), q(coats.max(0.0), Q_COATS), q(medium.clamp(0.0, 1.0), Q_MEDIUM), marks as i32];
        if let Some(m) = self.aims.lock().unwrap().get(&key) {
            return m.clone();
        }
        let wl = [key[0] as f32 / Q_WANT, key[1] as f32 / Q_WANT, key[2] as f32 / Q_WANT];
        let under = crate::color::from_oklab([key[3] as f32 / Q_UNDER, key[4] as f32 / Q_UNDER, key[5] as f32 / Q_UNDER]);
        let coats = (key[6] as f32 / Q_COATS).max(1.0 / Q_COATS);
        let dil = 1.0 - key[7] as f32 / Q_MEDIUM;
        let look = |c: Rgb, s: f32, x: f32| to_oklab(Pigment::masstone(c, s * dil).over(under, x));
        // the mark's mean look: its area at each thickness, averaged in
        // linear light (as the eye does at viewing distance)
        let spread_of = marks.spread();
        let mean = |c: Rgb, s: f32| {
            let pg = Pigment::masstone(c, s * dil);
            let mut acc = [0.0f32; 3];
            for &(t, w) in spread_of {
                let l = pg.over(under, coats * t);
                for k in 0..3 {
                    acc[k] += w * l[k];
                }
            }
            to_oklab(acc)
        };
        let ul = to_oklab(under);
        let robust = |p: &[(usize, f32)], c: Rgb, s: f32| {
            let spread = dist(mean(c, s), wl) + AIM_ROBUST * AIM_SPREAD.iter().map(|&(t, w)| w * dist(look(c, s, coats * t), wl)).sum::<f32>();
            let thin = AIM_THIN.1 * seg_dist(look(c, s, coats * AIM_THIN.0), ul, wl);
            let m = to_oklab(c);
            let family = AIM_FAMILY * ((m[1] - wl[1]).powi(2) + (m[2] - wl[2]).powi(2)).sqrt();
            spread + thin + family + parsimony(p)
        };
        // coarse: every candidate at the expected thickness; the best few
        // judged over the spread of thicknesses; then refine the proportions
        let mut first: Vec<(f32, usize)> = self.cands.iter().enumerate().map(|(i, c)| (dist(mean(c.color, c.scatter), wl), i)).collect();
        let n = first.len().min(AIM_SHORTLIST);
        first.select_nth_unstable_by(n - 1, |a, b| a.0.total_cmp(&b.0));
        let best = first[..n].iter().map(|&(_, i)| (robust(&self.cands[i].parts, self.cands[i].color, self.cands[i].scatter), i)).min_by(|a, b| a.0.total_cmp(&b.0)).unwrap();
        let (parts, _) = self.refine(self.cands[best.1].parts.clone(), &robust);
        let (c, s, _) = self.eval(&parts);
        let m = self.mixture(parts, dist(mean(c, s), wl));
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
    /// The underlayer is judged robustly (`Canvas::judge_under`): a fleck of
    /// bare ground inside the mark doesn't skew the pile.
    #[allow(clippy::too_many_arguments)]
    pub fn aim(&self, pal: &Palette, want: Rgb, (x, y): (f32, f32), r: f32, medium: f32, coats: f32) -> Paint {
        pal.paint_for(want, self.judge_under(x, y, r), medium, coats)
    }

    /// What a mark of radius `r` at (`x`, `y`) sits on, as a painter judges
    /// it: the typical color there, not the average. Nine sub-discs across
    /// the mark, combined by a median per OKLab channel, so a fleck of bare
    /// ground or a stray speck (which a mean in linear light lets dominate a
    /// dark passage) doesn't decide the pile. `Canvas::under` is the plain
    /// mean.
    pub fn judge_under(&self, x: f32, y: f32, r: f32) -> Rgb {
        let q = 0.5 * r;
        let labs: Vec<(Rgb, f32)> = [(0.0, 0.0), (-q, 0.0), (q, 0.0), (0.0, -q), (0.0, q), (-q, -q), (q, -q), (-q, q), (q, q)]
            .iter()
            .map(|&(dx, dy)| (to_oklab(self.under(x + dx, y + dy, 0.5 * r)), if dx == 0.0 && dy == 0.0 { 1.5 } else { 1.0 }))
            .collect();
        crate::color::from_oklab(std::array::from_fn(|c| weighted_median(labs.iter().map(|(l, w)| (l[c], *w)).collect())))
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

/// The weighted median of (value, weight) pairs (the lower one at a tie).
pub(crate) fn weighted_median(mut v: Vec<(f32, f32)>) -> f32 {
    v.sort_by(|a, b| a.0.total_cmp(&b.0));
    let half = 0.5 * v.iter().map(|p| p.1).sum::<f32>();
    let mut acc = 0.0;
    for &(x, w) in &v {
        acc += w;
        if acc >= half {
            return x;
        }
    }
    v.last().map_or(0.0, |p| p.0)
}

/// Distance from `p` to the segment `a`–`b` (OKLab).
fn seg_dist(p: Rgb, a: Rgb, b: Rgb) -> f32 {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    let t = if dd > 1e-12 { (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1] + (p[2] - a[2]) * d[2]) / dd).clamp(0.0, 1.0) } else { 0.0 };
    dist(p, [a[0] + d[0] * t, a[1] + d[1] * t, a[2] + d[2] * t])
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

    /// What aim picks for contrasting marks: the
    /// recipe and its look at a spread of thicknesses, in OKLab.
    /// `cargo test --release -p paint probe_contrast_aims -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn probe_contrast_aims() {
        let full = Palette::friedrich_1820();
        let sea = full.only(&["lead white", "pale smalt", "cobalt blue", "raw umber", "bone black", "yellow ochre"]);
        let lift = |c: Rgb, dl: f32| { let mut l = to_oklab(c); l[0] += dl; from_oklab(l) };
        let cases = [
            ("glint on sea", hex("#3d4a5c"), lerp3(to_oklab(lift(hex("#3d4a5c"), 0.12)), to_oklab(hex("#d8cdb4")), 0.3)),
            ("pebble top", hex("#4a3f33"), to_oklab(hex("#7a6d5c"))),
            ("stone patch", hex("#6d6a70"), to_oklab(hex("#8a8590"))),
            ("shadow on sand", hex("#b8a888"), to_oklab(hex("#8a8680"))),
            ("bright glint", hex("#3d4a5c"), to_oklab(hex("#d8cdb4"))),
            ("glint, fleck", { let (a, b) = (hex("#3d4a5c"), hex("#a9785a")); std::array::from_fn(|i| 0.8 * a[i] + 0.2 * b[i]) }, lerp3(to_oklab(lift(hex("#3d4a5c"), 0.12)), to_oklab(hex("#d8cdb4")), 0.3)),
            ("light on dark sand", hex("#3a3128"), to_oklab(hex("#9a8f80"))),
        ];
        for (name, under, want) in cases {
            let want = from_oklab(want);
            for (pn, pal) in [("full", &full), ("sea", &sea)] {
                for coats in [0.3, 1.0, 2.0] {
                    let m = pal.aim(want, under, 0.3, coats);
                    let p = m.paint(0.3);
                    let f = |c: Rgb| { let l = to_oklab(c); format!("({:.2} {:+.3} {:+.3})", l[0], l[1], l[2]) };
                    let looks: Vec<String> = [0.25, 0.5, 1.0, 2.0, 4.0].iter().map(|&t| f(p.over(under, coats * t))).collect();
                    println!("{name:15} {pn:4} coats {coats:.1}: err {:.3} [{}] masstone {} looks {} | want {} under {}", m.error, pal.recipe(&m), f(m.color), looks.join(" "), f(want), f(under));
                }
            }
        }
    }

    /// A light touch over a dark, aimed thin, stays in the family of the
    /// color asked for: its pile is not a salmon (lead white + red earth,
    /// masstone a +0.053 before) that only matches at exactly the expected
    /// thickness and dries pink wherever the brush lays more.
    #[test]
    fn contrasting_aims_stay_in_family() {
        let pal = Palette::friedrich_1820();
        for (under, want) in [(hex("#3a3128"), hex("#9a8f80")), (hex("#3d4a5c"), hex("#b8b4a8")), (hex("#4a3f33"), hex("#7a6d5c"))] {
            let wl = to_oklab(want);
            for coats in [0.3, 0.6, 1.0] {
                let m = pal.aim(want, under, 0.3, coats);
                let ml = to_oklab(m.color);
                let fam = ((ml[1] - wl[1]).powi(2) + (ml[2] - wl[2]).powi(2)).sqrt();
                let thick = to_oklab(m.paint(0.3).over(under, 4.0 * coats));
                let thick_ab = ((thick[1] - wl[1]).powi(2) + (thick[2] - wl[2]).powi(2)).sqrt();
                assert!(fam < 0.025 && thick_ab < 0.025, "{want:?} over {under:?} at {coats}: {} masstone ab off {fam:.3}, 4x ab off {thick_ab:.3}", pal.recipe(&m));
                assert!(m.error < 0.03, "{}: error {}", pal.recipe(&m), m.error);
            }
        }
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
        c.work(&all, &st.glaze(0.9).color(|_, _| hex("#4a2f1c")).angle(|_, _| std::f32::consts::FRAC_PI_2).coverage(3.0), 3);
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

#[cfg(test)]
mod green_tests {
    use super::*;
    use crate::color::hex;

    /// The green palettes reach summer greens the base palettes can only
    /// approach, by mixing (Prussian blue with the yellows), as he did.
    #[test]
    fn greens_are_mixed_closer() {
        for want in [hex("#4f6331"), hex("#2e3d2a"), hex("#93a14a"), hex("#6d7e3e")] {
            let (base, green) = (Palette::friedrich_1820(), Palette::friedrich_1820_greens());
            let (a, b) = (base.mix(want), green.mix(want));
            println!("{:?}: base {:.3} ({}) greens {:.3} ({})", want, a.error, base.recipe(&a), b.error, green.recipe(&b));
            assert!(b.error <= a.error + 1e-4);
        }
        let early = Palette::friedrich_early_greens();
        assert_eq!(early.tubes.len(), Palette::friedrich_early().tubes.len() + 2);
        assert!(Palette::friedrich_1820().with(vec![Palette::copper_green()]).tubes.iter().any(|t| t.name == "copper green"));
    }
}
