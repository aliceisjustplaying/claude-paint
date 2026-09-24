//! Piles: painting light the way a painter mixes it.
//!
//! A color field written as smooth math (a gradient, a Gaussian glow) gives
//! every stroke a mathematically perfect color. A painter doesn't work that
//! way: they knife a handful of piles on the palette along the passage's
//! range (the lightest light, the darkest dark and a few steps between),
//! reload from them, and make the transitions on the canvas, by laying
//! strokes of neighboring piles into each other wet and blending. No two
//! batches of a pile are quite the same mixture, and where two zones meet
//! the painter reaches for either pile, so the boundary wanders.
//!
//! A `PileSet` is that palette of piles for one field:
//! - `choose` picks the piles from the field's colors like a painter would:
//!   spread along the field's range (clusters of the distinct colors, not
//!   of the area, so a small bright glow still gets its own pile).
//! - `mix` knifes each pile from the palette's tubes once, aimed at how it
//!   looks over what is on the canvas where it will go (the median of the
//!   underlayer there), and misses a little (`vary`): a pile mixed by eye.
//! - `pick` chooses the pile for a stroke at its center (never per pixel):
//!   the nearest pile to the field there, with the boundary between two
//!   piles' zones displaced by a coherent noise and a little per-stroke
//!   chance, so the steps are ragged and interleaved, not contour lines.
//! - `batch` is the pile as it was knifed for the part of the picture a
//!   stroke is in: piles run out and are remixed, and each batch differs a
//!   little in its proportions and picks up a touch of the neighboring pile
//!   off the knife (cells of about `batch` units, irregular Voronoi cells so
//!   no batch seam is a straight line).
//!
//! Deterministic: everything follows from the seed and the position.
//! `Handling::piles` makes a pass dip into the set (see `handling.rs`).

use crate::canvas::Canvas;
use crate::color::{Rgb, from_oklab, to_oklab};
use crate::mask::Mask;
use crate::noise::{Fbm, Worley};
use crate::palette::{Marks, Mixture, Palette};
use crate::rng::Rng;
use crate::wet::Paint;

/// How a pile set is chosen and used (see the module docs).
#[derive(Clone, Copy, Debug)]
pub struct PileOpts {
    /// How far the boundary between two piles' zones wanders: 0 is a
    /// contour line of the field, 1 lets either pile reach well into the
    /// other's zone (share of the gap between the piles).
    pub overlap: f32,
    /// Size (units) of the patches the wandering boundary makes.
    pub patch: f32,
    /// Knife-mixing variation: relative sd of the proportions of a pile
    /// (its miss when first mixed) and of each batch.
    pub vary: f32,
    /// Size (units) of the part of the picture one batch of a pile covers.
    pub batch: f32,
    /// Most of the neighboring pile a batch picks up off the knife.
    pub dirty: f32,
    pub seed: u64,
}

impl Default for PileOpts {
    fn default() -> Self {
        PileOpts { overlap: 0.5, patch: 45.0, vary: 0.08, batch: 130.0, dirty: 0.12, seed: 1 }
    }
}

/// A pile knifed from tubes: its recipe and its neighbor (for batches).
#[derive(Clone, Debug)]
pub struct Pile {
    /// The look the painter meant the pile for (linear RGB).
    pub want: Rgb,
    /// Its recipe as mixed (None: no palette, the pile is `want` itself).
    pub recipe: Option<Mixture>,
    /// The nearest other pile (OKLab), which a batch picks up a touch of.
    pub near: usize,
}

pub struct PileSet {
    pub piles: Vec<Pile>,
    labs: Vec<Rgb>,
    /// The field the painter wrote (the light as they see it).
    field: Box<dyn Fn(f32, f32) -> Rgb + Sync>,
    pub opts: PileOpts,
    /// The palette the recipes are mixed from.
    pub palette: Option<Palette>,
    wander: Fbm,
    batches: Worley,
}

impl std::fmt::Debug for PileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PileSet").field("piles", &self.piles).field("opts", &self.opts).finish()
    }
}

fn dist(a: Rgb, b: Rgb) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// A stable 0..1 per position and salt.
fn unit_at(x: f32, y: f32, salt: u64) -> f32 {
    let mut h = (x.to_bits() as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y.to_bits() as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F) ^ salt.wrapping_mul(0x1656_67B1_9E37_79F9);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 29;
    (h >> 40) as f32 / (1u64 << 24) as f32
}

/// Choose `n` piles (OKLab) from colors sampled over a passage (OKLab,
/// weight), as a painter would: along the field's range. The colors are
/// binned (OKLab steps of `BIN`) and each distinct color counts by the
/// square root of its area, so a small glow still gets a pile of its own
/// while a wide even passage doesn't take them all; then Lloyd's
/// k-means, started from quantiles of lightness. Sorted dark to light.
pub fn choose(samples: &[(Rgb, f32)], n: usize) -> Vec<Rgb> {
    const BIN: f32 = 0.006;
    let mut bins: std::collections::BTreeMap<[i32; 3], (Rgb, f32)> = std::collections::BTreeMap::new();
    for &(l, w) in samples {
        if w <= 0.0 {
            continue;
        }
        let k = [(l[0] / BIN).round() as i32, (l[1] / BIN).round() as i32, (l[2] / BIN).round() as i32];
        let e = bins.entry(k).or_insert(([0.0; 3], 0.0));
        for c in 0..3 {
            e.0[c] += l[c] * w;
        }
        e.1 += w;
    }
    let pts: Vec<(Rgb, f32)> = bins.values().map(|(s, w)| ([s[0] / w, s[1] / w, s[2] / w], w.sqrt())).collect();
    if pts.is_empty() || n == 0 {
        return Vec::new();
    }
    let n = n.min(pts.len());
    // start: quantiles of lightness (by weight)
    let mut by_l: Vec<usize> = (0..pts.len()).collect();
    by_l.sort_by(|&a, &b| pts[a].0[0].total_cmp(&pts[b].0[0]));
    let total: f32 = pts.iter().map(|p| p.1).sum();
    let mut centers: Vec<Rgb> = Vec::with_capacity(n);
    let (mut acc, mut k) = (0.0, 0);
    for &i in &by_l {
        acc += pts[i].1;
        while k < n && acc >= total * (k as f32 + 0.5) / n as f32 {
            centers.push(pts[i].0);
            k += 1;
        }
    }
    while centers.len() < n {
        centers.push(pts[*by_l.last().unwrap()].0);
    }
    for _ in 0..40 {
        let mut sum = vec![([0.0f32; 3], 0.0f32); n];
        for &(l, w) in &pts {
            let j = nearest(&centers, l).0;
            for c in 0..3 {
                sum[j].0[c] += l[c] * w;
            }
            sum[j].1 += w;
        }
        let mut moved = 0.0f32;
        for j in 0..n {
            if sum[j].1 > 0.0 {
                let m = [sum[j].0[0] / sum[j].1, sum[j].0[1] / sum[j].1, sum[j].0[2] / sum[j].1];
                moved = moved.max(dist(m, centers[j]));
                centers[j] = m;
            }
        }
        if moved < 1e-5 {
            break;
        }
    }
    // a pile nobody uses (a duplicate start) is dropped
    centers.sort_by(|a, b| a[0].total_cmp(&b[0]));
    centers.dedup_by(|a, b| dist(*a, *b) < 1e-4);
    centers
}

/// Index of the nearest and the second nearest center, with distances.
fn nearest(centers: &[Rgb], l: Rgb) -> (usize, f32, usize, f32) {
    let (mut i1, mut d1, mut i2, mut d2) = (0, f32::MAX, 0, f32::MAX);
    for (i, c) in centers.iter().enumerate() {
        let d = dist(*c, l);
        if d < d1 {
            (i2, d2) = (i1, d1);
            (i1, d1) = (i, d);
        } else if d < d2 {
            (i2, d2) = (i, d);
        }
    }
    (i1, d1, i2, d2)
}

/// Mix the parts of two recipes: `a` with a share `k` of `b`.
fn combine(a: &[(usize, f32)], b: &[(usize, f32)], k: f32) -> Vec<(usize, f32)> {
    let mut out: Vec<(usize, f32)> = a.iter().map(|&(i, f)| (i, f * (1.0 - k))).collect();
    for &(i, f) in b {
        match out.iter_mut().find(|p| p.0 == i) {
            Some(p) => p.1 += f * k,
            None => out.push((i, f * k)),
        }
    }
    out
}

impl PileSet {
    /// Piles for `field` at the looks `wants` (linear RGB), not yet mixed
    /// (see `mix`; unmixed piles are laid as `want` with the handling's own
    /// paint).
    pub fn new(wants: Vec<Rgb>, field: Box<dyn Fn(f32, f32) -> Rgb + Sync>, opts: PileOpts) -> Self {
        assert!(!wants.is_empty(), "a pile set needs at least one pile");
        let labs: Vec<Rgb> = wants.iter().map(|&c| to_oklab(c)).collect();
        let piles = (0..wants.len())
            .map(|i| {
                let near = (0..labs.len()).filter(|&j| j != i).min_by(|&a, &b| dist(labs[a], labs[i]).total_cmp(&dist(labs[b], labs[i]))).unwrap_or(i);
                Pile { want: wants[i], recipe: None, near }
            })
            .collect();
        let s = opts.seed;
        PileSet {
            piles,
            labs,
            field,
            opts,
            palette: None,
            wander: Fbm::new((s as u32) ^ 0x9113, 3, opts.patch.max(1.0)),
            batches: Worley::new((s as u32) ^ 0xBA7C, opts.batch.max(1.0)),
        }
    }

    /// Piles chosen from the field itself: `n` of them along the colors it
    /// takes over `over` (see `choose`), sampled every `step` units.
    pub fn from_field(field: Box<dyn Fn(f32, f32) -> Rgb + Sync>, over: &Mask, n: usize, step: f32, opts: PileOpts) -> Self {
        let f = over.f;
        let step = step.max(0.5);
        let mut samples = Vec::new();
        let mut y = 0.5 * step;
        while y < f.height() {
            let mut x = 0.5 * step;
            while x < f.width() {
                let w = over.data[f.index(x, y)];
                if w > 0.25 {
                    samples.push((to_oklab(field(x, y)), w));
                }
                x += step;
            }
            y += step;
        }
        let labs = choose(&samples, n.max(1));
        let wants = if labs.is_empty() { vec![field(0.5 * f.width(), 0.5 * f.height())] } else { labs.iter().map(|&l| from_oklab(l)).collect() };
        Self::new(wants, field, opts)
    }

    /// The field the painter wrote, at (x, y).
    pub fn field_at(&self, x: f32, y: f32) -> Rgb {
        (self.field)(x, y)
    }

    /// The pile a stroke centered at (x, y) is loaded from.
    pub fn pick(&self, x: f32, y: f32) -> usize {
        if self.piles.len() == 1 {
            return 0;
        }
        let l = to_oklab((self.field)(x, y));
        let (i1, d1, i2, d2) = nearest(&self.labs, l);
        // where between the two piles the field is (0 at the first, 0.5
        // halfway), the boundary moved by a coherent wander and a little
        // chance per stroke
        // (oriented from the lower index to the higher, so the displacement
        // moves the boundary instead of swapping the piles on both sides)
        let r = d1 / (d1 + d2).max(1e-9);
        let (a, b, pos) = if i1 < i2 { (i1, i2, r) } else { (i2, i1, 1.0 - r) };
        let n = (1.6 * self.wander.get(x, y)).clamp(-1.0, 1.0) * 0.8 + 0.4 * (unit_at(x, y, self.opts.seed) - 0.5);
        if pos + 0.5 * self.opts.overlap * n > 0.5 { b } else { a }
    }

    /// The look pile `i` was meant for.
    pub fn want(&self, i: usize) -> Rgb {
        self.piles[i].want
    }

    /// The field as the piles give it: each point the look of the pile a
    /// stroke there is loaded from (stepped, ragged at the steps).
    pub fn stepped(&self, x: f32, y: f32) -> Rgb {
        self.piles[self.pick(x, y)].want
    }

    /// Knife each pile from `pal`'s tubes (thinned with `medium`), aimed
    /// at its look over what is on `cv` where it will go (the per-channel
    /// OKLab median of `Canvas::judge_under` at the points of `over` that
    /// pick it, every `step` units, and the canvas holds), laid `coats`
    /// thick (None: by masstone). Each pile misses its aim a little
    /// (`vary`), as a pile mixed by eye does.
    pub fn mix(&mut self, cv: &Canvas, over: &Mask, pal: &Palette, medium: f32, coats: Option<f32>, marks: Marks, step: f32) {
        let f = over.f;
        let win = cv.window();
        let step = step.max(1.0);
        let mut unders: Vec<Vec<Rgb>> = vec![Vec::new(); self.piles.len()];
        let mut y = 0.5 * step;
        while y < f.height() {
            let mut x = 0.5 * step;
            while x < f.width() {
                if over.data[f.index(x, y)] > 0.5 && win.holds(x, y) {
                    unders[self.pick(x, y)].push(to_oklab(cv.judge_under(x, y, step.min(8.0))));
                }
                x += step;
            }
            y += step;
        }
        for (i, us) in unders.iter().enumerate() {
            let want = self.piles[i].want;
            let m = match (coats, us.is_empty()) {
                (Some(c), false) => {
                    let med = |k: usize| {
                        let mut v: Vec<f32> = us.iter().map(|u| u[k]).collect();
                        v.sort_by(f32::total_cmp);
                        v[v.len() / 2]
                    };
                    let under = from_oklab([med(0), med(1), med(2)]);
                    pal.aim_for(want, under, medium, c, marks)
                }
                _ => pal.mix(want),
            };
            let mut rng = Rng::new(self.opts.seed ^ 0x9111_E5ED ^ (i as u64).wrapping_mul(0x9E37_79B9));
            self.piles[i].recipe = Some(pal.remix(&m, self.opts.vary, &mut rng));
        }
        self.palette = Some(pal.clone());
    }

    /// The batch of pile `i` a stroke at (x, y) is loaded from: its recipe
    /// re-knifed for this part of the picture (`vary`) with a touch of the
    /// neighboring pile (`dirty`). None when the piles aren't mixed.
    pub fn batch(&self, i: usize, x: f32, y: f32) -> Option<Mixture> {
        let pal = self.palette.as_ref()?;
        let p = &self.piles[i];
        let m = p.recipe.as_ref()?;
        let cell = self.batches.get(x, y).id as u64;
        let mut rng = Rng::new(self.opts.seed ^ 0xBA7C_4000 ^ cell.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ (i as u64).wrapping_mul(0x9E37_79B9));
        let k = self.opts.dirty * rng.f();
        let parts = match &self.piles[p.near].recipe {
            Some(n) if p.near != i => combine(&m.parts, &n.parts, k),
            _ => m.parts.clone(),
        };
        let b = pal.pile(parts);
        Some(pal.remix(&b, self.opts.vary, &mut rng))
    }

    /// Paint for a stroke at (x, y) from pile `i`, thinned with `medium`,
    /// with the handling's per-dip mixing jitter drawn from `rng`. None
    /// when the piles aren't mixed.
    pub fn paint(&self, i: usize, x: f32, y: f32, medium: f32, mix_jitter: f32, rng: &mut Rng) -> Option<Paint> {
        let pal = self.palette.as_ref()?;
        let b = self.batch(i, x, y)?;
        Some(pal.remix(&b, mix_jitter, rng).paint(medium))
    }

    /// The recipes as text (one line per pile), for the painter.
    pub fn describe(&self) -> Vec<String> {
        self.piles
            .iter()
            .map(|p| {
                let b = |v: f32| (crate::color::linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8;
                let hexs = format!("#{:02x}{:02x}{:02x}", b(p.want[0]), b(p.want[1]), b(p.want[2]));
                match (&p.recipe, &self.palette) {
                    (Some(m), Some(pal)) => format!("{hexs}: {}", pal.recipe(m)),
                    _ => format!("{hexs}: (not mixed)"),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Frame;

    fn sky(_: f32, y: f32) -> Rgb {
        // a dark-to-light vertical ramp
        let t = (y / 1000.0).clamp(0.0, 1.0);
        let a = to_oklab(crate::color::hex("#47536c"));
        let b = to_oklab(crate::color::hex("#d2bd98"));
        from_oklab([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t])
    }

    #[test]
    fn choose_spreads_piles_along_the_range() {
        let s: Vec<(Rgb, f32)> = (0..1000).map(|y| (to_oklab(sky(0.0, y as f32)), 1.0)).collect();
        let p = choose(&s, 5);
        assert_eq!(p.len(), 5);
        for w in p.windows(2) {
            assert!(w[1][0] > w[0][0] + 0.03, "piles step in lightness: {p:?}");
        }
        // the extremes are near the ends of the range
        let (lo, hi) = (to_oklab(sky(0.0, 0.0))[0], to_oklab(sky(0.0, 1000.0))[0]);
        assert!(p[0][0] - lo < 0.15 * (hi - lo) && hi - p[4][0] < 0.15 * (hi - lo), "{p:?} in {lo}..{hi}");
    }

    #[test]
    fn a_small_glow_gets_its_own_pile() {
        // a wide even gray and a small warm glow (1% of the area)
        let mut s: Vec<(Rgb, f32)> = (0..9900).map(|k| (to_oklab(crate::color::hex(if k % 2 == 0 { "#6f727a" } else { "#737680" })), 1.0)).collect();
        s.extend((0..100).map(|_| (to_oklab(crate::color::hex("#ecd49e")), 1.0)));
        let p = choose(&s, 2);
        let glow = to_oklab(crate::color::hex("#ecd49e"));
        assert!(p.iter().any(|c| dist(*c, glow) < 0.02), "{p:?}");
    }

    #[test]
    fn picks_are_stepped_ragged_and_deterministic() {
        let f = Frame::new(400, 400, 0.4);
        let m = Mask::full(f);
        let set = PileSet::from_field(Box::new(sky), &m, 5, 4.0, PileOpts::default());
        let again = PileSet::from_field(Box::new(sky), &m, 5, 4.0, PileOpts::default());
        let mut used = std::collections::BTreeSet::new();
        // along one row the pile changes with x near a zone boundary (the
        // boundary wanders), and each pick is the same every time
        let mut mixed_rows = 0;
        for y in (0..1000).step_by(12) {
            let row: std::collections::BTreeSet<usize> = (0..1000).step_by(7).map(|x| set.pick(x as f32, y as f32)).collect();
            for x in (0..1000).step_by(7) {
                assert_eq!(set.pick(x as f32, y as f32), again.pick(x as f32, y as f32));
            }
            if row.len() > 1 {
                mixed_rows += 1;
            }
            used.extend(row);
        }
        assert_eq!(used.len(), 5, "every pile is used");
        assert!(mixed_rows >= 8, "the steps are ragged, not level contour lines: {mixed_rows} rows use two piles");
        // but a pile is never far from its zone: never two piles apart
        for y in (0..1000).step_by(17) {
            let lo = (0..1000).step_by(7).map(|x| set.pick(x as f32, y as f32)).min().unwrap();
            let hi = (0..1000).step_by(7).map(|x| set.pick(x as f32, y as f32)).max().unwrap();
            assert!(hi - lo <= 1, "row {y}: piles {lo}..{hi}");
        }
    }

    #[test]
    fn batches_vary_but_stay_near_the_pile() {
        let f = Frame::new(400, 400, 0.4);
        let m = Mask::full(f);
        let cv = Canvas::new(400, 1.0, crate::color::hex("#d8cfbf"));
        let pal = Palette::friedrich_1820();
        let mut set = PileSet::from_field(Box::new(sky), &m, 4, 4.0, PileOpts::default());
        set.mix(&cv, &m, &pal, 0.3, None, Marks::Blunt, 8.0);
        let i = 2;
        let want = to_oklab(set.want(i));
        let looks: Vec<Rgb> = (0..12).map(|k| to_oklab(set.batch(i, 40.0 + 80.0 * k as f32, 500.0 + 30.0 * k as f32).unwrap().color)).collect();
        let spread = looks.iter().map(|l| dist(*l, looks[0])).fold(0.0f32, f32::max);
        assert!(spread > 0.004, "batches differ: {spread}");
        for l in &looks {
            assert!(dist(*l, want) < 0.08, "a batch stays near its pile: {l:?} vs {want:?}");
        }
        // the same place, the same batch
        assert_eq!(set.batch(i, 100.0, 100.0).unwrap().parts, set.batch(i, 100.0, 100.0).unwrap().parts);
        assert!(set.describe()[i].contains("white"), "{:?}", set.describe());
    }
}
