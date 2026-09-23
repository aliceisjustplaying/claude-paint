//! Noise in canvas units: fractal (fBm), ridged and billowed octaves, domain
//! warping, cellular (Worley), anisotropic stretching, and helpers that
//! break evenness (uneven spacing, clumping) for anything repeated.

use noise::{MultiFractal, NoiseFn, Perlin};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Fractal (fBm) Perlin noise. `Copy`: a field can go into any number of
/// `move` closures (a mask and a color field both using the same noise)
/// without clones or `let n = &n;`. The octave tables behind it are built
/// once per distinct (seed, octaves, persistence) and shared for the rest
/// of the program (about 1 KB per octave), so making the same noise twice
/// costs nothing, and making thousands of different ones costs memory.
#[derive(Clone, Copy)]
pub struct Fbm {
    f: &'static noise::Fbm<Perlin>,
    inv_period: f64,
}

type Key = (u32, usize, u64);

/// The shared octave tables for a key (built on first use).
fn tables(seed: u32, octaves: usize, persistence: f64) -> &'static noise::Fbm<Perlin> {
    static CACHE: OnceLock<Mutex<HashMap<Key, &'static noise::Fbm<Perlin>>>> = OnceLock::new();
    let mut m = CACHE.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner());
    m.entry((seed, octaves, persistence.to_bits())).or_insert_with(|| {
        let f = noise::Fbm::<Perlin>::new(seed).set_octaves(octaves).set_persistence(persistence).set_lacunarity(2.0);
        Box::leak(Box::new(f))
    })
}

impl Fbm {
    /// `period` is the size (in units) of the largest feature.
    pub fn new(seed: u32, octaves: usize, period: f32) -> Self {
        Fbm { f: tables(seed, octaves, 0.5), inv_period: 1.0 / period as f64 }
    }

    pub fn with_persistence(mut self, p: f64) -> Self {
        self.f = tables(noise::Seedable::seed(self.f), self.f.octaves, p);
        self
    }

    /// Roughly in [-1, 1].
    #[inline]
    pub fn get(&self, x: f32, y: f32) -> f32 {
        self.f.get([x as f64 * self.inv_period, y as f64 * self.inv_period]) as f32
    }

    /// Remapped to [0, 1].
    #[inline]
    pub fn get01(&self, x: f32, y: f32) -> f32 {
        (self.get(x, y) * 0.5 + 0.5).clamp(0.0, 1.0)
    }
}


// ------------------------------------------------------------ toolkit
//
// Fbm alone makes even, isotropic, "digital" noise: the same grain in every
// direction and at every place. The helpers below break that evenness the
// way nature does: domain warping (fields that flow and fold), ridged and
// billowed sums (sharp crests, heaped cloud tops), cellular noise (cells,
// cracks, clumps), anisotropy (streaks along a wind or a bedding) and
// irregular spacing for anything a painter repeats (layers, bands, posts).
// All are `Copy` and cheap to make, like `Fbm`.

/// Per-octave Perlin sources, shared like `Fbm`'s tables.
type OctaveCache = Mutex<HashMap<(u32, usize), &'static [Perlin]>>;

fn octave_tables(seed: u32, octaves: usize) -> &'static [Perlin] {
    static CACHE: OnceLock<OctaveCache> = OnceLock::new();
    let mut m = CACHE.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner());
    m.entry((seed, octaves)).or_insert_with(|| {
        let v: Vec<Perlin> = (0..octaves).map(|i| Perlin::new(seed.wrapping_mul(7919).wrapping_add(i as u32 * 1013))).collect();
        Box::leak(v.into_boxed_slice())
    })
}

/// What an octave contributes before weighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fold {
    /// Plain Perlin, signed (fBm).
    Plain,
    /// `1 − |n|`, squared: sharp crests, broad hollows (ridges, rock veins,
    /// the lit rims of cloud cells).
    Ridged,
    /// `|n|`: rounded heaps with creases between (cumulus tops, billowing
    /// smoke, soft hills).
    Billow,
}

/// Octave noise in 2-D or 3-D with a choice of fold (plain, ridged, billow)
/// and an optional level of detail (octaves finer than a footprint are
/// dropped, so far or coarse samples don't alias). `Copy`.
#[derive(Clone, Copy)]
pub struct Octaves {
    src: &'static [Perlin],
    inv_period: f64,
    persistence: f32,
    fold: Fold,
}

impl Octaves {
    /// `period` (units or meters, whatever you sample in) of the largest feature.
    pub fn new(seed: u32, octaves: usize, period: f32, fold: Fold) -> Self {
        Octaves { src: octave_tables(seed, octaves.max(1)), inv_period: 1.0 / period as f64, persistence: 0.5, fold }
    }
    pub fn ridged(seed: u32, octaves: usize, period: f32) -> Self {
        Self::new(seed, octaves, period, Fold::Ridged)
    }
    pub fn billow(seed: u32, octaves: usize, period: f32) -> Self {
        Self::new(seed, octaves, period, Fold::Billow)
    }
    pub fn persistence(mut self, p: f32) -> Self {
        self.persistence = p;
        self
    }
    #[inline]
    fn fold(&self, n: f32) -> f32 {
        match self.fold {
            Fold::Plain => n,
            Fold::Ridged => {
                let r = 1.0 - n.abs().min(1.0);
                r * r
            }
            Fold::Billow => n.abs().min(1.0),
        }
    }
    /// How many octaves resolve features larger than `footprint`.
    #[inline]
    fn count(&self, footprint: f32) -> (usize, f32) {
        let n = self.src.len();
        if footprint <= 0.0 {
            return (n, 1.0);
        }
        // octave i has period P / 2^i; keep it while that is > 2 footprints
        let per = 1.0 / self.inv_period as f32;
        let k = ((per / (2.0 * footprint)).max(1.0)).log2();
        // whole octaves, plus the next one faded in by the fraction
        let whole = (k.floor() as usize + 1).min(n);
        if whole < n { (whole + 1, k.fract()) } else { (n, 1.0) }
    }
    /// 2-D, normalized: `Plain` roughly in [-1, 1], `Ridged`/`Billow` in [0, 1].
    #[inline]
    pub fn get(&self, x: f32, y: f32) -> f32 {
        self.get_lod(x, y, 0.0)
    }
    /// 2-D with octaves finer than `footprint` faded out.
    pub fn get_lod(&self, x: f32, y: f32, footprint: f32) -> f32 {
        let (n, last) = self.count(footprint);
        let (mut f, mut a, mut sum, mut norm) = (self.inv_period, 1.0f32, 0.0f32, 0.0f32);
        for (i, p) in self.src.iter().enumerate().take(n) {
            let w = if i + 1 == n { a * last } else { a };
            sum += w * self.fold(p.get([x as f64 * f, y as f64 * f]) as f32);
            norm += w;
            f *= 2.0;
            a *= self.persistence;
        }
        if norm > 0.0 { sum / norm } else { 0.0 }
    }
    /// 3-D (cloud densities, volumes), normalized like `get`.
    pub fn get3(&self, p: [f32; 3], footprint: f32) -> f32 {
        let (n, last) = self.count(footprint);
        let (mut f, mut a, mut sum, mut norm) = (self.inv_period, 1.0f32, 0.0f32, 0.0f32);
        for (i, s) in self.src.iter().enumerate().take(n) {
            let w = if i + 1 == n { a * last } else { a };
            sum += w * self.fold(s.get([p[0] as f64 * f, p[1] as f64 * f, p[2] as f64 * f]) as f32);
            norm += w;
            f *= 2.0;
            a *= self.persistence;
        }
        if norm > 0.0 { sum / norm } else { 0.0 }
    }
}

/// Domain warping: a displacement field made of two noises. Sampling any
/// field at `warp.at(x, y)` instead of `(x, y)` makes it flow, fold and
/// thin out unevenly (Quilez, "Domain warping",
/// <https://iquilezles.org/articles/warp/>). `twice` warps the warp. `Copy`.
#[derive(Clone, Copy)]
pub struct Warp {
    wx: Fbm,
    wy: Fbm,
    /// Displacement (same units as x, y) at full noise.
    pub amount: f32,
    twice: bool,
}

impl Warp {
    /// A displacement of up to about `amount` with features `period` wide.
    pub fn new(seed: u32, period: f32, amount: f32) -> Self {
        Warp { wx: Fbm::new(seed.wrapping_add(4001), 3, period), wy: Fbm::new(seed.wrapping_add(4002), 3, period), amount, twice: false }
    }
    /// Warp the warp (folds within folds).
    pub fn twice(mut self) -> Self {
        self.twice = true;
        self
    }
    /// The warped point.
    #[inline]
    pub fn at(&self, x: f32, y: f32) -> (f32, f32) {
        let (mut u, mut v) = (x, y);
        if self.twice {
            let (a, b) = (self.wx.get(x + 5.2, y + 1.3), self.wy.get(x - 8.3, y + 2.8));
            u = x + self.amount * 0.6 * a;
            v = y + self.amount * 0.6 * b;
        }
        (x + self.amount * self.wx.get(u, v), y + self.amount * self.wy.get(u + 3.7, v - 9.1))
    }
    /// An fBm seen through the warp.
    #[inline]
    pub fn fbm(&self, n: &Fbm, x: f32, y: f32) -> f32 {
        let (u, v) = self.at(x, y);
        n.get(u, v)
    }
}

/// Stretch a field along a direction: streaks along a wind, a bedding plane,
/// a current. `angle` (radians, canvas y down) is the long axis, `stretch`
/// how many times longer features are along it than across. `Copy`.
#[derive(Clone, Copy, Debug)]
pub struct Aniso {
    c: f32,
    s: f32,
    inv: f32,
}

impl Aniso {
    pub fn new(angle: f32, stretch: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Aniso { c, s, inv: 1.0 / stretch.max(1e-3) }
    }
    /// The point in the stretched frame (sample any field there).
    #[inline]
    pub fn at(&self, x: f32, y: f32) -> (f32, f32) {
        let along = x * self.c + y * self.s;
        let across = -x * self.s + y * self.c;
        (along * self.inv, across)
    }
    #[inline]
    pub fn fbm(&self, n: &Fbm, x: f32, y: f32) -> f32 {
        let (u, v) = self.at(x, y);
        n.get(u, v)
    }
}

#[inline]
fn hash3(a: i32, b: i32, c: i32, seed: u32) -> u32 {
    let mut h = seed.wrapping_mul(0x9E37_79B9) ^ (a as u32).wrapping_mul(0x85EB_CA6B) ^ (b as u32).wrapping_mul(0xC2B2_AE35) ^ (c as u32).wrapping_mul(0x27D4_EB2F);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^ (h >> 15)
}
#[inline]
fn unit_of(h: u32) -> f32 {
    (h >> 8) as f32 / (1u32 << 24) as f32
}

/// What `Worley` returns at a point.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    /// Distance to the nearest feature point, in cells (0 at the point).
    pub f1: f32,
    /// Distance to the second nearest.
    pub f2: f32,
    /// A stable id of the nearest cell (its color, its size, its kind).
    pub id: u32,
    /// The nearest feature point (same units as the input).
    pub at: (f32, f32),
}

impl Cell {
    /// `f2 − f1`: 0 on the borders between cells (cracks, cell walls).
    pub fn edge(&self) -> f32 {
        self.f2 - self.f1
    }
    /// The cell id as a number in [0, 1).
    pub fn rand(&self) -> f32 {
        unit_of(self.id)
    }
}

/// Cellular (Worley) noise: jittered feature points, one per cell of size
/// `period`. Cells, cracked mud, clumps of foliage, the heaps of a cloud
/// bank. `jitter` 1 is fully random, 0 a square grid. `Copy`.
#[derive(Clone, Copy, Debug)]
pub struct Worley {
    seed: u32,
    period: f32,
    jitter: f32,
}

impl Worley {
    pub fn new(seed: u32, period: f32) -> Self {
        Worley { seed, period, jitter: 1.0 }
    }
    pub fn jitter(mut self, j: f32) -> Self {
        self.jitter = j.clamp(0.0, 1.0);
        self
    }
    fn point(&self, i: i32, j: i32, k: i32) -> [f32; 3] {
        let h = hash3(i, j, k, self.seed);
        let r = |s: u32| 0.5 + self.jitter * (unit_of(hash3(i, j, k, self.seed ^ s ^ h)) - 0.5);
        [i as f32 + r(0x1111), j as f32 + r(0x2222), k as f32 + r(0x3333)]
    }
    /// 2-D cells.
    pub fn get(&self, x: f32, y: f32) -> Cell {
        let (u, v) = (x / self.period, y / self.period);
        let (ci, cj) = (u.floor() as i32, v.floor() as i32);
        let (fu, fv) = (u - ci as f32, v - cj as f32);
        let (mut f1, mut f2, mut id, mut at) = (f32::MAX, f32::MAX, 0u32, (0.0, 0.0));
        // a feature lies anywhere in its cell, so one two or three cells
        // away can be nearer than the 3×3 block's: search out to 7×7,
        // skipping cells that cannot come nearer than the second-nearest
        for ring in 0..=3i32 {
            for dj in -ring..=ring {
                for di in -ring..=ring {
                    if di.abs().max(dj.abs()) != ring {
                        continue;
                    }
                    if ring > 1 {
                        let (gx, gy) = (gap(fu, di), gap(fv, dj));
                        if gx * gx + gy * gy >= f2 * f2 {
                            continue;
                        }
                    }
                    let p = self.point(ci + di, cj + dj, 0);
                    let d = ((p[0] - u).powi(2) + (p[1] - v).powi(2)).sqrt();
                    if d < f1 {
                        f2 = f1;
                        f1 = d;
                        id = hash3(ci + di, cj + dj, 0, self.seed ^ 0xABCD);
                        at = (p[0] * self.period, p[1] * self.period);
                    } else if d < f2 {
                        f2 = d;
                    }
                }
            }
        }
        Cell { f1, f2, id, at }
    }
    /// 3-D: distance to the nearest feature point, in cells.
    pub fn f1_3(&self, p: [f32; 3]) -> f32 {
        let q = [p[0] / self.period, p[1] / self.period, p[2] / self.period];
        let c = [q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32];
        let fr = [q[0] - c[0] as f32, q[1] - c[1] as f32, q[2] - c[2] as f32];
        let mut f1 = f32::MAX;
        // the nearest feature is within √3 of the point: out to 5×5×5,
        // the 3×3×3 block first, then the outer shell where it could be nearer
        for outer in [false, true] {
            for dk in -2..=2i32 {
                for dj in -2..=2i32 {
                    for di in -2..=2i32 {
                        if (di.abs().max(dj.abs()).max(dk.abs()) > 1) != outer {
                            continue;
                        }
                        if outer {
                            let (gx, gy, gz) = (gap(fr[0], di), gap(fr[1], dj), gap(fr[2], dk));
                            if gx * gx + gy * gy + gz * gz >= f1 {
                                continue;
                            }
                        }
                        let f = self.point(c[0] + di, c[1] + dj, c[2] + dk);
                        let d = (f[0] - q[0]).powi(2) + (f[1] - q[1]).powi(2) + (f[2] - q[2]).powi(2);
                        f1 = f1.min(d);
                    }
                }
            }
        }
        f1.sqrt()
    }
}

/// Least distance along one axis from a point at fraction `f` (0..1) of its
/// cell to the cell `d` cells over.
#[inline]
fn gap(f: f32, d: i32) -> f32 {
    match d {
        0 => 0.0,
        d if d > 0 => d as f32 - f,
        d => f - (d + 1) as f32,
    }
}

/// Uneven placement: `n` positions from `lo` to `hi` (sorted, the first at
/// `lo` and the last at `hi` when `n > 1`) whose gaps are not equal.
/// `irregular` 0 spaces them evenly; 0.5 is a hand placing things by eye
/// (gaps vary about ±50 %); 1 and more gives big gaps and near-touches.
/// `clump` (0..1) pulls neighbors together into groups, leaving wider gaps
/// between groups (ranges that crowd in one part of the view, posts that
/// come in twos and threes). Parallel repeated bands with even gaps read as
/// waves; this is the cure. Deterministic in `seed`.
pub fn uneven(n: usize, lo: f32, hi: f32, irregular: f32, clump: f32, seed: u32) -> Vec<f32> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![0.5 * (lo + hi)];
    }
    let r = |i: usize, s: u32| unit_of(hash3(i as i32, s as i32, 17, seed));
    // gaps drawn from a lognormal around 1 (Box–Muller), then grouped
    let mut gaps: Vec<f32> = (0..n - 1)
        .map(|i| {
            let (u1, u2) = (r(i, 1).max(1e-6), r(i, 2));
            let z = (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos();
            (irregular * 0.8 * z).exp()
        })
        .collect();
    if clump > 0.0 {
        // every gap is inside a group (shrunk) or between groups (grown)
        let group = 2 + (r(0, 9) * 2.0) as usize;
        let mut k = (r(0, 8) * group as f32) as usize;
        for (i, g) in gaps.iter_mut().enumerate() {
            k += 1;
            let between = k.is_multiple_of(group) || r(i, 7) < 0.15;
            *g *= if between { 1.0 + 2.5 * clump } else { 1.0 - 0.7 * clump };
        }
    }
    let total: f32 = gaps.iter().sum();
    let mut out = Vec::with_capacity(n);
    let mut t = lo;
    out.push(t);
    for g in gaps {
        t += g / total * (hi - lo);
        out.push(t);
    }
    out
}

/// A value varied by a hand: `v × (1 + amount × z)` with z a stable random
/// number in [-1, 1] for key `i` (heights, widths, loads of repeated things).
pub fn vary(v: f32, amount: f32, i: usize, seed: u32) -> f32 {
    v * (1.0 + amount * (2.0 * unit_of(hash3(i as i32, 3, 5, seed)) - 1.0))
}

/// A stable random number in [0, 1) for key `(i, j)`.
pub fn rand01(i: i32, j: i32, seed: u32) -> f32 {
    unit_of(hash3(i, j, 11, seed))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Copy and shared, with the same values as the noise crate's own Fbm
    /// built the way `Fbm::new` always built it (output unchanged).
    #[test]
    fn fbm_is_copy_and_unchanged() {
        let n = Fbm::new(7, 4, 120.0).with_persistence(0.6);
        let a = move |x: f32| n.get(x, 3.0);
        let b = move |x: f32| n.get01(x, 3.0);
        let _ = (a(1.0), b(1.0));
        let own = noise::Fbm::<Perlin>::new(7).set_octaves(4).set_persistence(0.5).set_lacunarity(2.0).set_persistence(0.6);
        for i in 0..50 {
            let (x, y) = (i as f32 * 13.7, i as f32 * 5.3 - 40.0);
            assert_eq!(n.get(x, y), own.get([x as f64 * (1.0 / 120.0), y as f64 * (1.0 / 120.0)]) as f32);
        }
        // the same key shares its tables
        assert!(std::ptr::eq(Fbm::new(7, 4, 1.0).f, Fbm::new(7, 4, 99.0).f));
        assert!(!std::ptr::eq(Fbm::new(7, 4, 1.0).f, Fbm::new(8, 4, 1.0).f));
    }

    /// Field objects a painter shares between closures are `Copy`
    /// (winter #13, coast #7, mountains #6): a noise, a `per_column` profile
    /// and a `move` closure built from them.
    #[test]
    fn fields_are_copy() {
        fn copy<T: Copy + Sync>(_: &T) {}
        let f = crate::canvas::Frame::new(200, 100, 0.2);
        let n = Fbm::new(3, 3, 200.0);
        let ridge = f.per_column(move |x| 300.0 + 40.0 * n.get(x, 0.0));
        let mask = move |x: f32, y: f32| if y > ridge(x) { 1.0 } else { 0.0 };
        let color = move |x: f32, y: f32| (y - ridge(x)) * 0.01 + n.get(x, y);
        copy(&n);
        copy(&ridge);
        copy(&mask);
        copy(&color);
        let m = crate::Mask::from_fn(f, mask);
        assert!(m.data.contains(&1.0) && m.data.contains(&0.0));
        // exact at column centers and off them
        assert_eq!(ridge(2.5), 300.0 + 40.0 * n.get(2.5, 0.0));
        assert_eq!(ridge(3.1), 300.0 + 40.0 * n.get(3.1, 0.0));
        let _ = color(1.0, 1.0);
    }

    #[test]
    fn toolkit_is_copy_and_ranged() {
        fn copy<T: Copy + Sync>(_: &T) {}
        let r = Octaves::ridged(3, 5, 100.0);
        let b = Octaves::billow(4, 5, 100.0);
        let p = Octaves::new(5, 4, 100.0, Fold::Plain);
        let w = Warp::new(1, 200.0, 40.0).twice();
        let a = Aniso::new(0.3, 6.0);
        let c = Worley::new(9, 50.0);
        copy(&r);
        copy(&b);
        copy(&p);
        copy(&w);
        copy(&a);
        copy(&c);
        let n = Fbm::new(2, 3, 80.0);
        for i in 0..200 {
            let (x, y) = (i as f32 * 7.3, i as f32 * 3.1 - 50.0);
            let (vr, vb) = (r.get(x, y), b.get(x, y));
            assert!((0.0..=1.0).contains(&vr) && (0.0..=1.0).contains(&vb));
            assert!(p.get(x, y).abs() <= 1.2);
            let _ = w.fbm(&n, x, y) + a.fbm(&n, x, y);
            let cl = c.get(x, y);
            assert!(cl.f1 <= cl.f2 && cl.edge() >= 0.0);
            let _ = r.get3([x, y, 3.0], 20.0) + c.f1_3([x, y, 1.0]);
        }
        // level of detail: a huge footprint leaves only the first octave
        assert_eq!(p.get_lod(3.0, 4.0, 1e6), octave_tables(5, 4)[0].get([0.03, 0.04]) as f32);
    }

    #[test]
    fn uneven_is_uneven_and_deterministic() {
        let e = uneven(8, 0.0, 70.0, 0.0, 0.0, 1);
        for (i, v) in e.iter().enumerate() {
            assert!((v - 10.0 * i as f32).abs() < 1e-3);
        }
        let u = uneven(8, 0.0, 70.0, 0.7, 0.5, 1);
        assert_eq!(u, uneven(8, 0.0, 70.0, 0.7, 0.5, 1));
        assert!((u[0] - 0.0).abs() < 1e-4 && (u[7] - 70.0).abs() < 1e-3);
        let gaps: Vec<f32> = u.windows(2).map(|w| w[1] - w[0]).collect();
        let (mn, mx) = gaps.iter().fold((f32::MAX, 0.0f32), |a, &g| (a.0.min(g), a.1.max(g)));
        assert!(gaps.iter().all(|&g| g > 0.0));
        assert!(mx > 2.0 * mn, "gaps {gaps:?}");
    }

    /// Worley finds the nearest and second-nearest features even when they
    /// lie two cells away (features sit anywhere in their cell): checked
    /// against a brute-force 7×7 (2-D) and 5×5×5 (3-D) search.
    #[test]
    fn worley_finds_the_true_nearest_features() {
        fn wide(w: &Worley, x: f32, y: f32) -> Cell {
            let (u, v) = (x / w.period, y / w.period);
            let (ci, cj) = (u.floor() as i32, v.floor() as i32);
            let mut all: Vec<(f32, u32, (f32, f32))> = vec![];
            for dj in -3..=3 {
                for di in -3..=3 {
                    let p = w.point(ci + di, cj + dj, 0);
                    let d = ((p[0] - u).powi(2) + (p[1] - v).powi(2)).sqrt();
                    all.push((d, hash3(ci + di, cj + dj, 0, w.seed ^ 0xABCD), (p[0] * w.period, p[1] * w.period)));
                }
            }
            all.sort_by(|a, b| a.0.total_cmp(&b.0));
            Cell { f1: all[0].0, f2: all[1].0, id: all[0].1, at: all[0].2 }
        }
        // the review's counterexample: a feature two cells away is nearest
        let w = Worley::new(15, 1.0);
        let c = w.get(-0.029999733, 0.7669997);
        assert_eq!(c.id, 409115953, "{c:?}");
        assert!((c.f1 - 1.1241378).abs() < 1e-5, "{c:?}");
        for seed in 0..40u32 {
            let w = Worley::new(seed, 1.0);
            for i in 0..400 {
                let (x, y) = (rand01(i, 1, seed) * 20.0 - 10.0, rand01(i, 2, seed) * 20.0 - 10.0);
                let (a, b) = (w.get(x, y), wide(&w, x, y));
                assert_eq!(a.id, b.id, "seed {seed} ({x},{y})");
                assert!((a.f1 - b.f1).abs() < 1e-6 && (a.f2 - b.f2).abs() < 1e-6, "seed {seed} ({x},{y}) {a:?} {b:?}");
                let p = [x, y, rand01(i, 3, seed) * 20.0 - 10.0];
                let c = [p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32];
                let mut f1 = f32::MAX;
                for dk in -2..=2 {
                    for dj in -2..=2 {
                        for di in -2..=2 {
                            let f = w.point(c[0] + di, c[1] + dj, c[2] + dk);
                            f1 = f1.min((f[0] - p[0]).powi(2) + (f[1] - p[1]).powi(2) + (f[2] - p[2]).powi(2));
                        }
                    }
                }
                assert!((w.f1_3(p) - f1.sqrt()).abs() < 1e-6, "3-D seed {seed} {p:?}");
            }
        }
    }
}
