//! Craquelure: age cracks through paint and ground, grown one at a time in a
//! stress field, then cut into the surface.
//!
//! Sources (notes/research/oil_paint_physics.md §5 and "Implications" E;
//! notes/research/friedrich_materials.md §8):
//! - **Sequential, not Voronoi.** Paquette, Poulin & Drettakis (GI 2002): a
//!   grid of stress and strength; a crack starts where σ/S_b is highest,
//!   runs perpendicular to the largest principal stress while σ > S_c
//!   (S_b ≈ 1.5 S_c) and relaxes the stress within a distance D_r of itself.
//!   A crack frees the stress *normal* to itself, so later cracks turn to
//!   meet earlier ones at T-junctions near 90°; the top bar of each T formed
//!   first, which gives primary and secondary generations (de Willigen 1999,
//!   Bucklow). Bury & Bratasz (npj Herit. Sci. 2024) fit the relief around a
//!   crack with a Lorentzian and add cracks one at a time; cracks stop
//!   multiplying once the stress between two of them falls below the
//!   fracture stress, so spacing saturates instead of being imposed.
//! - **Aging.** Paint and ground embrittle for decades (lead white's strain
//!   at break falls from 7.3% at 0.2 years to under 1% at 31 years, Janas et
//!   al. 2022). Here the film's strength steps down over a few generations:
//!   the first, strongest-film cracks are sparse and long (primaries); later
//!   ones subdivide the islands, and cracks arrested in tougher paint resume.
//!   An arrested tip still breaks through to a crack just ahead of it (the
//!   stress concentrates at a tip); a last "fatigue" pass (humidity cycling,
//!   subcritical growth) lets free tips creep a little farther and join a
//!   crack they reach, so most ends are T-junctions.
//! - **Spacing.** D_r ≈ S/2 for a target spacing S. Canvas craquelure islands
//!   are about 1–6 mm; Friedrich's centers ~2–6 mm (assumption), corners
//!   6 ± 3 mm (Bury & Bratasz mock-up).
//! - **Weave coupling.** "Jagged cracks with a rectangular pattern are
//!   associated with characteristically thin brittle grounds which allow
//!   cracks to faithfully follow the (plain) canvas weave. Smooth, curved
//!   cracks are associated with thick (possibly double) grounds" (de Willigen
//!   1999). Weight w = clamp(1 − t_ground / 100 µm): at w → 1 the paths snap
//!   to warp and weft in steps of the thread pitch; at w → 0 they curve.
//! - **Corners.** Stress concentrates at the stretcher corners (Mecklenburg
//!   1994); cracks there run perpendicular to the diagonal within ~5–10% of
//!   its length (Bury & Bratasz).
//! - **Scale.** Everything is in mm: the stress grid has cells of S/6, the
//!   crack path steps ≤ 0.25 mm (a whole number per thread pitch), and the
//!   rasterizer gives each pixel the fraction of it the crack covers (cracks
//!   of 50–100 µm are often narrower than a pixel).
//! - **Geometry.** Age cracks are sharp and narrow, 50–100 µm wide (OCT: 70 µm
//!   wide, 370 µm deep) and go through paint and ground; islands cup up at
//!   their edges (raking light shows it); grime collects in the cracks and
//!   makes them read darker (Getty *Conserving Canvas*; CAMEO).

use crate::canvas::Canvas;
use crate::noise::Fbm;
use crate::pigment::Pigment;
use crate::rng::{Rng, hash2};
use crate::surface::COAT_UM;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Craquelure recipe. All sizes are physical, so a painting cracks the same
/// at any pixel resolution. `None` fields are derived from the canvas when it
/// cracks (`Canvas::crack`): the ground it was primed with, the island size
/// that ground and paint give, the opening that size gives.
#[derive(Clone, Copy, Debug)]
pub struct Cracks {
    /// Target median island size (square root of island area), mm.
    /// `None`: from the layer thickness (`island_for`).
    pub island_mm: Option<f32>,
    /// Thickness of the ground under the paint, µm: thin brittle grounds let
    /// the cracks follow the weave, thick ones free them. `None`: the
    /// canvas's own ground (the sum of its `prime` layers).
    pub ground_um: Option<f32>,
    /// Opening of a primary crack at the surface, µm (secondaries are finer).
    /// `None`: the film's strain times the island size (`STRAIN`).
    pub width_um: Option<f32>,
    /// Visible depth of a primary crack after varnish, µm.
    pub depth_um: f32,
    /// How far island edges lift beside a crack, µm.
    pub cupping_um: f32,
    /// Grime in the cracks, 0 (clean) .. 1 (heavily soiled).
    pub dirt: f32,
    /// Corner cracks perpendicular to the diagonals.
    pub corners: bool,
    /// How unevenly the picture aged, 0 (one even network) .. 1. The film's
    /// stress varies over the canvas (patchy drying, the stretcher bars'
    /// inner edges); the local paint decides how far its network
    /// subdivided (lead-white-rich lights are brittle and crack finely,
    /// oily dark glazes stay tougher) and how wide cracks open (thicker
    /// film, wider).
    pub vary: f32,
    /// Microcracked varnish, 0 (none) .. 1: patches where the old varnish
    /// has crazed finely and its crack edges scatter light "like a milky
    /// veil" [SMB-blog], strongest over darks.
    pub veil: f32,
    pub seed: u64,
}

/// Island size per µm of layer (ground + paint): channel cracks in a film
/// on a compliant support saturate at a spacing of some ten to twenty film
/// thicknesses. 14 puts a 240 µm Friedrich ground at ~3.6 mm, inside the
/// 2–6 mm the research notes give (assumption, calibrated to that range).
pub const ISLAND_PER_UM: f32 = 14.0e-3;
/// Paint film over the ground assumed when sizing islands, µm.
const PAINT_UM: f32 = 20.0;
/// A crack opens by the film's strain times the island size: shrinkage and
/// the canvas's slack, ~1% after two centuries (assumption; a 3.5 mm island
/// opens 32 µm, under the 70 µm the OCT study measured on a wide crack).
pub const STRAIN: f32 = 0.009;

impl Cracks {
    /// A quietly aged canvas, fitted to the canvas it cracks: islands and
    /// weave coupling from its ground, openings from the island size (about
    /// 30 µm on a Friedrich ground), 20 µm deep after varnish, a little
    /// grime and cupping, uneven aging and a few patches of milky varnish.
    pub fn aged(seed: u64) -> Self {
        Cracks {
            island_mm: None,
            ground_um: None,
            width_um: None,
            depth_um: 20.0,
            cupping_um: 15.0,
            dirt: 0.4,
            corners: true,
            vary: 1.0,
            veil: 0.5,
            seed,
        }
    }

    /// The recipe fitted to a canvas primed with `ground_um` µm: every
    /// `None` field filled in.
    pub fn fit(&self, ground_um: f32) -> Cracks {
        let ground = self.ground_um.unwrap_or(ground_um);
        let island = self.island_mm.unwrap_or_else(|| island_for(ground));
        Cracks { ground_um: Some(ground), island_mm: Some(island), width_um: Some(self.width_um.unwrap_or(STRAIN * island * 1000.0)), ..*self }
    }

    /// Ground thickness, µm (0 if not given yet).
    pub fn ground(&self) -> f32 {
        self.ground_um.unwrap_or(0.0)
    }

    /// Median island size, mm.
    pub fn island(&self) -> f32 {
        self.island_mm.unwrap_or_else(|| island_for(self.ground()))
    }

    /// Opening of a primary crack, µm.
    pub fn width(&self) -> f32 {
        self.width_um.unwrap_or(STRAIN * self.island() * 1000.0)
    }

    /// Weave-coupling weight w = clamp(1 − t_ground / 100 µm).
    pub fn weave(&self) -> f32 {
        (1.0 - self.ground() / 100.0).clamp(0.0, 1.0)
    }
}

/// Median island size (mm) for a ground of `ground_um` under a thin paint
/// film: `ISLAND_PER_UM` × the layer thickness, 1.2–7 mm.
pub fn island_for(ground_um: f32) -> f32 {
    (ISLAND_PER_UM * (ground_um + PAINT_UM)).clamp(1.2, 7.0)
}

/// How a crack arm ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    /// Ran into crack number `.0` (a T-junction).
    Crack(u32),
    /// Left the canvas.
    Border,
    /// Arrested in the paint (stress ran out); a free end.
    Free,
}

/// One side of a crack, grown from its nucleation point outward.
#[derive(Clone, Debug)]
pub struct Arm {
    pub crack: u32,
    /// Polyline in mm (x right, y down), starting at the nucleation point.
    pub pts: Vec<[f32; 2]>,
    pub end: End,
    /// Opening relative to a primary crack (≈ the stress it released).
    pub opening: f32,
    /// Generation the crack nucleated in (0 = primary).
    pub generation: u8,
}

/// A crack network over a `size_mm` surface.
#[derive(Clone, Debug)]
pub struct Network {
    pub size_mm: [f32; 2],
    pub arms: Vec<Arm>,
}

// ---- tuning (dimensionless; lengths in units of the target spacing S) ----

/// Strength steps (strength / mean stress) from the first generation to the
/// last; the last one sets where cracking saturates, calibrated so the median
/// island matches `island_mm`.
const R_FIRST: f32 = 1.6;
const R_LAST: f32 = 0.42;
const GENERATIONS: usize = 5;
/// Paquette et al.: breaking strength S_b ≈ 1.5 × the strength that stops a
/// running crack, S_c.
const SB_OVER_SC: f32 = 1.5;
/// Relaxation half-distance D_r (the Lorentzian's half width), × S.
const RELAX: f32 = 0.5;
/// The Lorentzian is cut off at this many D_r.
const RELAX_CUT: f32 = 3.0;
/// Stress grid cell, × S.
const GRID: f32 = 1.0 / 6.0;
/// Log-normal spread of the film's strength (patchy paint and ground).
const STRENGTH_SD: f32 = 0.22;
/// A running crack turns toward the principal direction this fast (per step).
const STEER: f32 = 0.6;
/// An arrested tip still breaks through to a crack this close ahead (× S).
const LINK: f32 = 0.45;
/// How far a free tip creeps in the fatigue pass (× S).
const FATIGUE: f32 = 0.6;
/// Extra stress at the corners along the diagonal (× mean stress), and the
/// size of the corner zone (fraction of the diagonal).
const CORNER_STRESS: f32 = 0.7;
const CORNER_ZONE: f32 = 0.08;
/// Fraction of the isotropic stress turned along the diagonal at a corner.
const CORNER_UNIAXIAL: f32 = 0.75;

type V2 = [f32; 2];

#[inline]
fn sub(a: V2, b: V2) -> V2 {
    [a[0] - b[0], a[1] - b[1]]
}
#[inline]
fn addv(a: V2, b: V2) -> V2 {
    [a[0] + b[0], a[1] + b[1]]
}
#[inline]
fn mul(a: V2, k: f32) -> V2 {
    [a[0] * k, a[1] * k]
}
#[inline]
fn dot(a: V2, b: V2) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}
#[inline]
fn norm(a: V2) -> V2 {
    let l = dot(a, a).sqrt().max(1e-12);
    [a[0] / l, a[1] / l]
}
#[inline]
fn perp(a: V2) -> V2 {
    [-a[1], a[0]]
}
#[inline]
fn rot(a: V2, t: f32) -> V2 {
    let (s, c) = t.sin_cos();
    [a[0] * c - a[1] * s, a[0] * s + a[1] * c]
}

/// Closest point on segment ab to p: (parameter t in 0..1, point).
#[inline]
fn closest(p: V2, a: V2, b: V2) -> (f32, V2) {
    let ab = sub(b, a);
    let l2 = dot(ab, ab);
    let t = if l2 > 0.0 { (dot(sub(p, a), ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (t, addv(a, mul(ab, t)))
}

/// Parameter along pq where it crosses segment ab, if it does.
fn cross(p: V2, q: V2, a: V2, b: V2) -> Option<f32> {
    let r = sub(q, p);
    let s = sub(b, a);
    let den = r[0] * s[1] - r[1] * s[0];
    if den.abs() < 1e-12 {
        return None;
    }
    let ap = sub(a, p);
    let t = (ap[0] * s[1] - ap[1] * s[0]) / den;
    let u = (ap[0] * r[1] - ap[1] * r[0]) / den;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then_some(t)
}

/// Principal stresses of a plane stress tensor (σxx, σyy, σxy):
/// (σ1 ≥ σ2, direction of σ1).
#[inline]
fn principal(s: [f32; 3]) -> (f32, f32, V2) {
    let m = 0.5 * (s[0] + s[1]);
    let r = (0.25 * (s[0] - s[1]).powi(2) + s[2] * s[2]).sqrt();
    let th = 0.5 * (2.0 * s[2]).atan2(s[0] - s[1]);
    (m + r, m - r, [th.cos(), th.sin()])
}

/// Normal stress across a plane with unit normal n.
#[inline]
fn normal_stress(s: [f32; 3], n: V2) -> f32 {
    s[0] * n[0] * n[0] + s[1] * n[1] * n[1] + 2.0 * s[2] * n[0] * n[1]
}

/// Fraction of the normal stress a crack frees at distance d (Lorentzian
/// with half width `l`, cut off smoothly at RELAX_CUT·l).
#[inline]
fn relief(d: f32, l: f32) -> f32 {
    let x = d / l;
    let lc = 1.0 / (1.0 + RELAX_CUT * RELAX_CUT);
    ((1.0 / (1.0 + x * x) - lc) / (1.0 - lc)).max(0.0)
}

struct Seg {
    a: V2,
    b: V2,
    crack: u32,
    /// Arm and position along it (a crack may not cross itself).
    arm: u32,
    k: u32,
}

/// Who is asking about collisions: a crack's arm, `k` segments along.
#[derive(Clone, Copy)]
struct Who {
    crack: u32,
    arm: u32,
    k: u32,
}

impl Who {
    /// Segments too close along the crack to count as a collision: the
    /// arm's own last few, and the sibling arm's first ones.
    fn skips(&self, s: &Seg) -> bool {
        s.crack == self.crack && ((s.arm == self.arm && s.k + 4 > self.k) || (s.arm != self.arm && s.k < 3 && self.k < 3))
    }
}

#[derive(Clone)]
struct ArmState {
    arm: Arm,
    h: V2,
    /// Curvature (rad/mm) and, on the weave, a slow wobble of the heading
    /// that makes the staircase drift one way for a while.
    omega: f32,
    wob: f32,
    /// Weave staircase error (desired minus stepped displacement), the
    /// current thread step (per sub-step) and sub-steps left in it.
    acc: V2,
    stair: (V2, u32),
    /// The tip without its local jaggedness.
    ideal: V2,
    rng: Rng,
    /// Stress released along the arm (sum, count), for its opening.
    rel: (f32, f32),
}

struct Builder {
    size: V2,
    /// Target spacing S, step length (a thread pitch is `sub` steps),
    /// thread pitch (x, y), weave w.
    s: f32,
    step: f32,
    sub: u32,
    pitch: V2,
    weave: f32,
    // stress grid
    gc: f32,
    gx: usize,
    gy: usize,
    sig: Vec<[f32; 3]>,
    strength: Vec<f32>,
    // segment index
    ic: f32,
    ix: usize,
    iy: usize,
    cells: Vec<Vec<u32>>,
    segs: Vec<Seg>,
    arms: Vec<ArmState>,
    ncracks: u32,
    // relaxation scratch
    mark: Vec<u32>,
    best: Vec<(f32, V2)>,
    touched: Vec<u32>,
    epoch: u32,
    /// Current strength factor (strength / mean stress).
    r: f32,
    /// Step limit per growth (fatigue pass).
    limit: Option<usize>,
    seed: u64,
}

impl Builder {
    fn new(k: &Cracks, size: V2, pitch: V2) -> Self {
        let s = k.island().max(0.2);
        let gc = s * GRID;
        let gx = (size[0] / gc).ceil() as usize + 1;
        let gy = (size[1] / gc).ceil() as usize + 1;
        let seed = k.seed;
        // mean stress: isotropic shrinkage of the film, varying slowly
        let broad = Fbm::new(seed as u32 ^ 0x51, 3, 12.0 * s);
        let patchy = Fbm::new(seed as u32 ^ 0x52, 3, 2.5 * s);
        let diag = (size[0] * size[0] + size[1] * size[1]).sqrt();
        let zone = CORNER_ZONE * diag;
        let corners = k.corners;
        let vary = k.vary.clamp(0.0, 1.0);
        // uneven aging: the film shrank more here than there (drying,
        // humidity, old restorations), over some centimeters
        let uneven = Fbm::new(seed as u32 ^ 0x54, 3, 60.0);
        let bars = stretcher(size);
        let wob = Fbm::new(seed as u32 ^ 0x55, 2, 40.0);
        let cells: Vec<([f32; 3], f32)> = (0..gx * gy)
            .into_par_iter()
            .map(|i| {
                let p = [(i % gx) as f32 * gc + 0.5 * gc, (i / gx) as f32 * gc + 0.5 * gc];
                let mut m = 1.0 + 0.12 * broad.get(p[0], p[1]) + 0.3 * vary * uneven.get(p[0], p[1]);
                let mut t = [m, m, 0.0];
                if vary > 0.0 {
                    // the stretcher bars: the canvas over a bar is held and
                    // cracks less; along a bar's inner edge it flexes, and
                    // cracks run parallel to the edge (a stress across it)
                    let (over, edge) = bars.at(p, s, &wob);
                    m *= 1.0 - 0.15 * vary * over;
                    t = [m, m, 0.0];
                    let e = 0.35 * vary;
                    t[0] += e * edge[0];
                    t[1] += e * edge[1];
                }
                if corners {
                    for (cx, cy) in [(0.0, 0.0), (size[0], 0.0), (0.0, size[1]), (size[0], size[1])] {
                        let d = sub(p, [cx, cy]);
                        let r = dot(d, d).sqrt();
                        if r < zone {
                            // tension along the diagonal
                            let u = norm([size[0] * if cx > 0.0 { -1.0 } else { 1.0 }, size[1] * if cy > 0.0 { -1.0 } else { 1.0 }]);
                            // the stretcher corner pulls the canvas along its
                            // diagonal: the stress there becomes nearly uniaxial
                            let f = 1.0 - crate::smoothstep(0.5 * zone, zone, r);
                            let along = m * CORNER_UNIAXIAL * f + CORNER_STRESS * f;
                            t = [t[0] * (1.0 - CORNER_UNIAXIAL * f), t[1] * (1.0 - CORNER_UNIAXIAL * f), t[2] * (1.0 - CORNER_UNIAXIAL * f)];
                            t[0] += along * u[0] * u[0];
                            t[1] += along * u[1] * u[1];
                            t[2] += along * u[0] * u[1];
                        }
                    }
                }
                let white = hash2((i % gx) as i64, (i / gx) as i64, seed ^ 0x53) - 0.5;
                let st = (STRENGTH_SD * (1.4 * patchy.get(p[0], p[1]) + 0.9 * white)).exp();
                (t, st)
            })
            .collect();
        let (sig, strength) = cells.into_iter().unzip();
        // cracks are jagged below the thread scale (paint and ground grain):
        // steps of ≤ 0.25 mm, a whole number per thread pitch
        let pitch = [pitch[0].min(s / 3.0), pitch[1].min(s / 3.0)];
        let pm = pitch[0].min(pitch[1]);
        let sub = (pm / 0.25f32.min(s / 10.0)).ceil().max(1.0) as u32;
        let step = pm / sub as f32;
        let ic = (0.25 * s).max(step);
        let ix = (size[0] / ic).ceil() as usize + 1;
        let iy = (size[1] / ic).ceil() as usize + 1;
        Builder {
            size,
            s,
            step,
            sub,
            pitch,
            weave: k.weave(),
            gc,
            gx,
            gy,
            sig,
            strength,
            ic,
            ix,
            iy,
            cells: (0..ix * iy).map(|_| Vec::new()).collect(),
            segs: Vec::new(),
            arms: Vec::new(),
            ncracks: 0,
            mark: vec![0; gx * gy],
            best: vec![(0.0, [0.0; 2]); gx * gy],
            touched: Vec::new(),
            epoch: 0,
            r: R_FIRST,
            limit: None,
            seed,
        }
    }

    /// Bilinear stress and strength at a point.
    fn sample(&self, p: V2) -> ([f32; 3], f32) {
        let fx = (p[0] / self.gc - 0.5).clamp(0.0, (self.gx - 1) as f32);
        let fy = (p[1] / self.gc - 0.5).clamp(0.0, (self.gy - 1) as f32);
        let (x0, y0) = (fx as usize, fy as usize);
        let (x1, y1) = ((x0 + 1).min(self.gx - 1), (y0 + 1).min(self.gy - 1));
        let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
        let mut s = [0.0; 3];
        let mut st = 0.0;
        for (i, wgt) in [
            (y0 * self.gx + x0, (1.0 - tx) * (1.0 - ty)),
            (y0 * self.gx + x1, tx * (1.0 - ty)),
            (y1 * self.gx + x0, (1.0 - tx) * ty),
            (y1 * self.gx + x1, tx * ty),
        ] {
            for (sc, v) in s.iter_mut().zip(self.sig[i]) {
                *sc += v * wgt;
            }
            st += self.strength[i] * wgt;
        }
        (s, st)
    }

    fn index_cells(&self, lo: V2, hi: V2) -> impl Iterator<Item = usize> + '_ {
        let cx0 = ((lo[0] / self.ic).floor().max(0.0) as usize).min(self.ix - 1);
        let cy0 = ((lo[1] / self.ic).floor().max(0.0) as usize).min(self.iy - 1);
        let cx1 = ((hi[0] / self.ic).floor().max(0.0) as usize).min(self.ix - 1);
        let cy1 = ((hi[1] / self.ic).floor().max(0.0) as usize).min(self.iy - 1);
        (cy0..=cy1).flat_map(move |y| (cx0..=cx1).map(move |x| y * self.ix + x))
    }

    fn add_seg(&mut self, a: V2, b: V2, who: Who) {
        let id = self.segs.len() as u32;
        self.segs.push(Seg { a, b, crack: who.crack, arm: who.arm, k: who.k });
        let lo = [a[0].min(b[0]), a[1].min(b[1])];
        let hi = [a[0].max(b[0]), a[1].max(b[1])];
        let cs: Vec<usize> = self.index_cells(lo, hi).collect();
        for c in cs {
            self.cells[c].push(id);
        }
    }

    /// First place segment pq meets another crack: (point, crack).
    fn hit(&self, p: V2, q: V2, who: Who) -> Option<(V2, u32)> {
        let eps = 0.01 * self.s;
        let lo = [p[0].min(q[0]) - eps, p[1].min(q[1]) - eps];
        let hi = [p[0].max(q[0]) + eps, p[1].max(q[1]) + eps];
        let mut best: Option<(f32, V2, u32)> = None;
        for c in self.index_cells(lo, hi) {
            for &id in &self.cells[c] {
                let sg = &self.segs[id as usize];
                if who.skips(sg) {
                    continue;
                }
                if let Some(t) = cross(p, q, sg.a, sg.b) {
                    if best.is_none_or(|b| t < b.0) {
                        best = Some((t, addv(p, mul(sub(q, p), t)), sg.crack));
                    }
                } else {
                    // the tip grazes a crack: stop on it
                    let (_, x) = closest(q, sg.a, sg.b);
                    let d = sub(q, x);
                    if dot(d, d) < eps * eps && best.is_none_or(|b| b.0 > 1.0) {
                        best = Some((1.0 + dot(d, d).sqrt(), x, sg.crack));
                    }
                }
            }
        }
        best.map(|b| (b.1, b.2))
    }

    /// Clip pq to the canvas; Some(point) if q left it.
    fn border(&self, p: V2, q: V2) -> Option<V2> {
        let (w, h) = (self.size[0], self.size[1]);
        if q[0] >= 0.0 && q[0] <= w && q[1] >= 0.0 && q[1] <= h {
            return None;
        }
        let d = sub(q, p);
        let mut t = 1.0f32;
        for (pc, dc, lim) in [(p[0], d[0], w), (p[1], d[1], h)] {
            if dc < 0.0 {
                t = t.min(-pc / dc);
            } else if dc > 0.0 {
                t = t.min((lim - pc) / dc);
            }
        }
        Some(addv(p, mul(d, t.clamp(0.0, 1.0))))
    }

    /// Direction prior of the weave: pull h toward the nearest thread axis.
    fn snap(&self, h: V2, k: f32) -> V2 {
        let a = if h[0].abs() >= h[1].abs() { [h[0].signum(), 0.0] } else { [0.0, h[1].signum()] };
        norm(addv(h, mul(sub(a, h), k)))
    }

    /// Grow arm `ai` until it arrests, hits a crack or leaves the canvas.
    /// Returns the ids of the segments it laid.
    fn grow(&mut self, ai: usize) -> std::ops::Range<usize> {
        let first = self.segs.len();
        let mut a = std::mem::replace(
            &mut self.arms[ai],
            ArmState {
                arm: Arm { crack: 0, pts: vec![], end: End::Free, opening: 0.0, generation: 0 },
                h: [1.0, 0.0],
                omega: 0.0,
                wob: 0.0,
                acc: [0.0; 2],
                stair: ([0.0; 2], 0),
                ideal: [0.0; 2],
                rng: Rng::new(0),
                rel: (0.0, 0.0),
            },
        );
        let w = self.weave;
        let max_steps = self.limit.unwrap_or((4.0 * (self.size[0] + self.size[1]) / self.step) as usize + 16);
        let crack = a.arm.crack;
        a.arm.end = End::Free;
        for _ in 0..max_steps {
            let p = *a.arm.pts.last().unwrap();
            let who = Who { crack, arm: ai as u32, k: a.arm.pts.len() as u32 - 1 };
            let (sig, st) = self.sample(p);
            let n = perp(a.h);
            let snn = normal_stress(sig, n);
            if snn < self.r * st / SB_OVER_SC {
                // arrested; the stress concentrated at the tip still breaks
                // through to a crack just ahead
                let q = addv(p, mul(a.h, LINK * self.s));
                if let Some((x, c)) = self.hit(p, q, who) {
                    self.add_seg(p, x, who);
                    a.arm.pts.push(x);
                    a.arm.end = End::Crack(c);
                }
                break;
            }
            a.rel.0 += snn;
            a.rel.1 += 1.0;
            // turn toward the direction perpendicular to the largest
            // principal stress, as far as the stress is anisotropic
            let (s1, s2, e1) = principal(sig);
            let conf = ((s1 - s2) / (s1.abs() + s2.abs() + 1e-6)).clamp(0.0, 1.0);
            let mut t = perp(e1);
            if dot(t, a.h) < 0.0 {
                t = mul(t, -1.0);
            }
            let mut h = norm(addv(a.h, mul(t, STEER * conf)));
            // gentle global curvature (the free, thick-ground look) and
            // local jaggedness (which the weave turns into jogs)
            // curvature κ (rad/mm): correlated over ~S, radius ~8 S
            let c = (-self.step / self.s).exp();
            a.omega = c * a.omega + (1.0 - c * c).sqrt() * (1.0 - w) / (8.0 * self.s) * a.rng.normal();
            h = rot(h, a.omega * self.step + 0.03 * a.rng.normal());
            // weave prior
            h = self.snap(h, 0.3 * w);
            a.h = h;
            a.wob = c * a.wob + (1.0 - c * c).sqrt() * 0.2 * a.rng.normal();
            // the step: smooth, or a staircase along warp and weft in whole
            // thread pitches, or a blend
            let smooth = mul(h, self.step);
            let d = if w > 0.0 {
                if a.stair.1 == 0 {
                    let (px, py) = (self.pitch[0], self.pitch[1]);
                    let hw = rot(h, a.wob);
                    let run = 1.0 / (hw[0].abs() / px + hw[1].abs() / py).max(1e-6);
                    a.acc = addv(a.acc, mul(hw, run));
                    // never step back against the heading (old error from
                    // before a turn would make notches)
                    for (e, hc) in a.acc.iter_mut().zip(hw) {
                        if *e * hc < 0.0 {
                            *e = 0.0;
                        }
                    }
                    let st = if (a.acc[0] / px).abs() >= (a.acc[1] / py).abs() {
                        [px * a.acc[0].signum(), 0.0]
                    } else {
                        [0.0, py * a.acc[1].signum()]
                    };
                    a.acc = sub(a.acc, st);
                    a.stair = (mul(st, 1.0 / self.sub as f32), self.sub);
                }
                a.stair.1 -= 1;
                addv(mul(smooth, 1.0 - w), mul(a.stair.0, w))
            } else {
                smooth
            };
            a.ideal = addv(a.ideal, d);
            // fine lateral jitter of the crack line (grain of the paint)
            let dn = norm(perp(d));
            let jit = self.step * (0.12 - 0.06 * w) * a.rng.normal();
            let q = addv(a.ideal, mul(dn, jit));
            if let Some((x, c)) = self.hit(p, q, who) {
                self.add_seg(p, x, who);
                a.arm.pts.push(x);
                a.arm.end = End::Crack(c);
                break;
            }
            if let Some(x) = self.border(p, q) {
                self.add_seg(p, x, who);
                a.arm.pts.push(x);
                a.arm.end = End::Border;
                break;
            }
            self.add_seg(p, q, who);
            a.arm.pts.push(q);
        }
        let last = self.segs.len();
        self.arms[ai] = a;
        first..last
    }

    /// Relax the stress around chords of crack just laid (`(a, b, tip)`;
    /// `tip`: b is a free tip, and the stress ahead of it is not relaxed).
    fn relax(&mut self, chords: &[(V2, V2, bool)]) {
        self.epoch += 1;
        let ep = self.epoch;
        self.touched.clear();
        let l = RELAX * self.s;
        let reach = RELAX_CUT * l;
        for &(a, b, free_tip) in chords {
            let ab = sub(b, a);
            let l2 = dot(ab, ab);
            if l2 <= 0.0 {
                continue;
            }
            let nrm = norm(perp(ab));
            let x0 = (((a[0].min(b[0]) - reach) / self.gc - 0.5).floor().max(0.0)) as usize;
            let y0 = (((a[1].min(b[1]) - reach) / self.gc - 0.5).floor().max(0.0)) as usize;
            let x1 = ((((a[0].max(b[0]) + reach) / self.gc - 0.5).ceil().max(0.0)) as usize).min(self.gx - 1);
            let y1 = ((((a[1].max(b[1]) + reach) / self.gc - 0.5).ceil().max(0.0)) as usize).min(self.gy - 1);
            for y in y0..=y1 {
                let cy = (y as f32 + 0.5) * self.gc;
                for x in x0..=x1 {
                    let c = [(x as f32 + 0.5) * self.gc, cy];
                    let tr = dot(sub(c, a), ab) / l2;
                    if free_tip && tr > 1.0 {
                        continue;
                    }
                    let p = addv(a, mul(ab, tr.clamp(0.0, 1.0)));
                    let d = sub(c, p);
                    let dd = dot(d, d);
                    if dd >= reach * reach {
                        continue;
                    }
                    let i = y * self.gx + x;
                    if self.mark[i] != ep {
                        self.mark[i] = ep;
                        self.best[i] = (dd, nrm);
                        self.touched.push(i as u32);
                    } else if dd < self.best[i].0 {
                        self.best[i] = (dd, nrm);
                    }
                }
            }
        }
        for &i in &self.touched {
            let i = i as usize;
            let (d2, n) = self.best[i];
            let r = relief(d2.sqrt(), l);
            if r <= 0.0 {
                continue;
            }
            // σ' = Q σ Q, Q = I − a n nᵀ: the normal stress falls by (1 − r),
            // shear across the crack by √(1 − r), the stress along it stays
            let a = 1.0 - (1.0 - r).max(0.0).sqrt();
            let s = self.sig[i];
            let v = [s[0] * n[0] + s[2] * n[1], s[2] * n[0] + s[1] * n[1]];
            let nv = dot(n, v);
            self.sig[i] = [
                s[0] - 2.0 * a * n[0] * v[0] + a * a * nv * n[0] * n[0],
                s[1] - 2.0 * a * n[1] * v[1] + a * a * nv * n[1] * n[1],
                s[2] - a * (n[0] * v[1] + v[0] * n[1]) + a * a * nv * n[0] * n[1],
            ];
        }
    }

    /// Relax around arms that just grew (their new segment ranges). The
    /// relief reaches ~1.5 S, so the jagged path is relaxed along chords of
    /// about half that: far cheaper, and the jags don't matter at that range.
    fn relax_arms(&mut self, grown: &[(usize, std::ops::Range<usize>)]) {
        let lc = 0.5 * RELAX_CUT * RELAX * self.s;
        let mut chords = Vec::new();
        for (ai, r) in grown {
            if r.is_empty() {
                continue;
            }
            let free = self.arms[*ai].arm.end == End::Free;
            let mut a = self.segs[r.start].a;
            let mut run = 0.0;
            for si in r.clone() {
                let (sa, sb) = (self.segs[si].a, self.segs[si].b);
                let d = sub(sb, sa);
                run += dot(d, d).sqrt();
                let last = si + 1 == r.end;
                if run >= lc || last {
                    chords.push((a, sb, free && last));
                    a = sb;
                    run = 0.0;
                }
            }
        }
        self.relax(&chords);
    }

    fn ratio(&self, i: usize) -> f32 {
        principal(self.sig[i]).0 / (self.r * self.strength[i])
    }

    /// Start a crack at grid cell i: two arms back to back.
    fn nucleate(&mut self, i: usize, generation: u8) {
        let (cx, cy) = (i % self.gx, i / self.gx);
        let j = |k: u64| hash2(cx as i64, cy as i64, self.seed ^ k) - 0.5;
        let p = [((cx as f32 + 0.5 + j(0x61)) * self.gc).clamp(0.0, self.size[0]), ((cy as f32 + 0.5 + j(0x62)) * self.gc).clamp(0.0, self.size[1])];
        let (s1, s2, e1) = principal(self.sig[i]);
        let conf = (s1 - s2) / (s1.abs() + s2.abs() + 1e-6);
        let h = if conf > 0.05 { perp(e1) } else { rot([1.0, 0.0], std::f32::consts::PI * (j(0x63) + 0.5)) };
        let h = self.snap(h, self.weave);
        let crack = self.ncracks;
        self.ncracks += 1;
        let mut grown = Vec::new();
        for side in [1.0f32, -1.0] {
            let ai = self.arms.len();
            self.arms.push(ArmState {
                arm: Arm { crack, pts: vec![p], end: End::Free, opening: 0.0, generation },
                h: mul(h, side),
                omega: 0.0,
                wob: 0.0,
                acc: [0.0; 2],
                stair: ([0.0; 2], 0),
                ideal: p,
                rng: Rng::new(self.seed ^ ((crack as u64) << 1 | (side > 0.0) as u64).wrapping_mul(0x9E37_79B9)),
                rel: (0.0, 0.0),
            });
            let r = self.grow(ai);
            grown.push((ai, r));
        }
        // opening ≈ the stress the crack released, relative to the mean
        let (a, b) = (self.arms.len() - 2, self.arms.len() - 1);
        let rel = (self.arms[a].rel.0 + self.arms[b].rel.0) / (self.arms[a].rel.1 + self.arms[b].rel.1).max(1.0);
        self.arms[a].arm.opening = rel;
        self.arms[b].arm.opening = rel;
        self.relax_arms(&grown);
    }

    fn run(mut self) -> Network {
        for g in 0..GENERATIONS {
            let f = g as f32 / (GENERATIONS - 1) as f32;
            self.r = R_FIRST * (R_LAST / R_FIRST).powf(f);
            // the film is weaker now: arrested cracks run on
            for ai in 0..self.arms.len() {
                if self.arms[ai].arm.end == End::Free {
                    let r = self.grow(ai);
                    if !r.is_empty() {
                        self.relax_arms(&[(ai, r)]);
                    }
                }
            }
            // then new cracks, strongest stress / strength first
            let mut heap: BinaryHeap<(u32, Reverse<u32>)> = (0..self.gx * self.gy)
                .filter_map(|i| {
                    let q = self.ratio(i);
                    (q > 1.0).then_some((q.to_bits(), Reverse(i as u32)))
                })
                .collect();
            while let Some((qb, Reverse(i))) = heap.pop() {
                let i = i as usize;
                let q = self.ratio(i);
                if q <= 1.0 {
                    continue;
                }
                if q.to_bits() < qb {
                    // relaxed since it was queued: requeue at its new rank
                    heap.push((q.to_bits(), Reverse(i as u32)));
                    continue;
                }
                self.nucleate(i, g as u8);
            }
        }
        self.fatigue();
        Network { size_mm: self.size, arms: self.arms.into_iter().map(|a| a.arm).collect() }
    }

    /// Decades of humidity cycling: a free tip creeps on at any stress
    /// (subcritical growth) and joins a crack it reaches within ~0.6 S; tips
    /// that reach nothing stay free.
    fn fatigue(&mut self) {
        self.r = 0.0;
        self.limit = Some((FATIGUE * self.s / self.step).ceil() as usize);
        for ai in 0..self.arms.len() {
            if self.arms[ai].arm.end != End::Free {
                continue;
            }
            let keep = self.arms[ai].clone();
            let r = self.grow(ai);
            if !matches!(self.arms[ai].arm.end, End::Crack(_)) {
                // undo: drop the new segments from the index, newest first
                for si in r.rev() {
                    let (a, b) = (self.segs[si].a, self.segs[si].b);
                    let lo = [a[0].min(b[0]), a[1].min(b[1])];
                    let hi = [a[0].max(b[0]), a[1].max(b[1])];
                    let cs: Vec<usize> = self.index_cells(lo, hi).collect();
                    for c in cs {
                        if self.cells[c].last() == Some(&(si as u32)) {
                            self.cells[c].pop();
                        }
                    }
                    self.segs.pop();
                }
                self.arms[ai] = keep;
            }
        }
    }
}

/// The stretcher behind the canvas: a bar along each edge, `bar` mm wide, and
/// on a large canvas a cross bar or two.
struct Stretcher {
    size: V2,
    bar: f32,
    /// Cross bars: vertical at these x, horizontal at these y (mm).
    cx: Vec<f32>,
    cy: Vec<f32>,
}

fn stretcher(size: V2) -> Stretcher {
    let bar = (0.1 * size[0].min(size[1])).clamp(30.0, 70.0);
    let cross = |l: f32| if l > 900.0 { vec![0.5 * l] } else { vec![] };
    Stretcher { size, bar, cx: cross(size[0]), cy: cross(size[1]) }
}

impl Stretcher {
    /// At p: (how far p lies over a bar 0..1, stress across the nearest
    /// inner bar edge as (σxx, σyy) extra, relative to the mean stress).
    /// The line where the canvas flexes wanders (the canvas was restretched,
    /// the bar edge is worn and rounded) and is a band some islands wide.
    fn at(&self, p: V2, s: f32, wob: &Fbm) -> (f32, [f32; 2]) {
        let (w, h, b) = (self.size[0], self.size[1], self.bar);
        let band = 0.8 * s;
        let ridge = |d: f32| (-(d / band).powi(2)).exp();
        // the flex line: a little inside the bar edge, wandering ±0.6 S
        let bx = b + 0.6 * s * wob.get(0.0, p[1]);
        let by = b + 0.6 * s * wob.get(p[0], 500.0);
        let bx2 = b + 0.6 * s * wob.get(250.0, p[1]);
        let by2 = b + 0.6 * s * wob.get(p[0], 750.0);
        // x-bars (left, right, crosses) give a stress along x
        let mut dx = (p[0] - bx).abs().min((w - bx2 - p[0]).abs());
        let mut dy = (p[1] - by).abs().min((h - by2 - p[1]).abs());
        let inside = crate::smoothstep(0.0, 0.5 * s, p[0] - b).min(crate::smoothstep(0.0, 0.5 * s, w - b - p[0])).min(crate::smoothstep(0.0, 0.5 * s, p[1] - b)).min(crate::smoothstep(0.0, 0.5 * s, h - b - p[1]));
        let mut over = 1.0 - inside;
        for &c in &self.cx {
            let o = wob.get(c, p[1]) * 0.6 * s;
            dx = dx.min((p[0] - (c - 0.5 * b) - o).abs()).min((p[0] - (c + 0.5 * b) - o).abs());
            over = over.max(1.0 - crate::smoothstep(0.5 * b - 0.5 * s, 0.5 * b, (p[0] - c).abs()));
        }
        for &c in &self.cy {
            let o = wob.get(p[0], c) * 0.6 * s;
            dy = dy.min((p[1] - (c - 0.5 * b) - o).abs()).min((p[1] - (c + 0.5 * b) - o).abs());
            over = over.max(1.0 - crate::smoothstep(0.5 * b - 0.5 * s, 0.5 * b, (p[1] - c).abs()));
        }
        (over, [ridge(dx), ridge(dy)])
    }
}

/// Grow a crack network over a `size_mm` surface on linen of thread pitch
/// `pitch_mm` (x: across the warp threads, y: across the weft).
pub fn network(k: &Cracks, size_mm: [f32; 2], pitch_mm: [f32; 2]) -> Network {
    Builder::new(k, size_mm, pitch_mm).run()
}

/// A crack segment for rasterizing: ends in mm and relative opening at each.
struct RSeg {
    a: V2,
    b: V2,
    wa: f32,
    wb: f32,
    /// Generation the crack formed in.
    generation: f32,
}

/// What the paint under a crack does to it, on a coarse grid (mm) over the
/// rasterized window: how many generations of the network the film there
/// went through (`reach`, up to `GENERATIONS`), and how wide cracks open
/// relative to the recipe.
pub(crate) struct Local {
    cell: f32,
    o: V2,
    nx: usize,
    ny: usize,
    reach: Vec<f32>,
    open: Vec<f32>,
}

impl Local {
    /// The whole network, openings as given.
    pub(crate) fn uniform() -> Self {
        Local { cell: 1e9, o: [0.0; 2], nx: 1, ny: 1, reach: vec![GENERATIONS as f32], open: vec![1.0] }
    }

    /// Bilinear (reach, opening) at p (mm).
    fn at(&self, p: V2) -> (f32, f32) {
        let fx = ((p[0] - self.o[0]) / self.cell - 0.5).clamp(0.0, (self.nx - 1) as f32);
        let fy = ((p[1] - self.o[1]) / self.cell - 0.5).clamp(0.0, (self.ny - 1) as f32);
        let (x0, y0) = (fx as usize, fy as usize);
        let (x1, y1) = ((x0 + 1).min(self.nx - 1), (y0 + 1).min(self.ny - 1));
        let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
        let mut r = (0.0, 0.0);
        for (i, w) in [(y0 * self.nx + x0, (1.0 - tx) * (1.0 - ty)), (y0 * self.nx + x1, tx * (1.0 - ty)), (y1 * self.nx + x0, (1.0 - tx) * ty), (y1 * self.nx + x1, tx * ty)] {
            r.0 += self.reach[i] * w;
            r.1 += self.open[i] * w;
        }
        r
    }
}

/// Per-pixel crack effects.
pub(crate) struct Raster {
    /// Fraction of the pixel that is open crack.
    pub cover: Vec<f32>,
    /// Fraction that is crack or its worn, grimy shoulders.
    pub shoulder: Vec<f32>,
    /// Height change, µm.
    pub dz: Vec<f32>,
}

/// Coverage of a pixel (tent filter of radius r around its center) by a band
/// of width w whose center line is d away.
#[inline]
fn band_cover(d: f32, w: f32, r: f32) -> f32 {
    let cdf = |u: f32| {
        if u <= -r {
            0.0
        } else if u < 0.0 {
            (u + r) * (u + r) / (2.0 * r * r)
        } else if u < r {
            1.0 - (r - u) * (r - u) / (2.0 * r * r)
        } else {
            1.0
        }
    };
    (cdf(0.5 * w - d) - cdf(-0.5 * w - d)).clamp(0.0, 1.0)
}

fn rsegs(net: &Network, k: &Cracks) -> Vec<RSeg> {
    let taper = 0.35 * k.island();
    let mut out = Vec::new();
    for a in &net.arms {
        let n = a.pts.len();
        if n < 2 {
            continue;
        }
        // each crack opens by the stress it released, and unevenly along
        // its length (grain, varying film thickness)
        let lognormal = |x: f32| (0.3 * 1.7 * (x - 0.5)).exp();
        let open = (a.opening * lognormal(hash2(a.crack as i64, 0, k.seed ^ 0x71))).clamp(0.2, 1.4);
        let mut arc = vec![0.0f32; n];
        for i in 1..n {
            let d = sub(a.pts[i], a.pts[i - 1]);
            arc[i] = arc[i - 1] + dot(d, d).sqrt();
        }
        let total = arc[n - 1];
        let along = |s: f32| {
            let x = s / 0.7;
            let (i, f) = (x.floor(), x - x.floor());
            let f = f * f * (3.0 - 2.0 * f);
            let v = |j: f32| hash2(a.crack as i64, j as i64 + 1, k.seed ^ 0x72);
            0.7 + 0.6 * (v(i) + (v(i + 1.0) - v(i)) * f)
        };
        // the opening closes to a point at a free end
        let wf = |i: usize| {
            let t = if a.end == End::Free { 0.15 + 0.85 * ((total - arc[i]) / taper).min(1.0) } else { 1.0 };
            open * along(arc[i]) * t
        };
        for i in 0..n - 1 {
            out.push(RSeg { a: a.pts[i], b: a.pts[i + 1], wa: wf(i), wb: wf(i + 1), generation: a.generation as f32 });
        }
    }
    out
}

/// Rasterize the network onto a w × h pixel grid of `px` mm per pixel.
#[cfg(test)]
pub(crate) fn raster(net: &Network, k: &Cracks, w: usize, h: usize, px: f32) -> Raster {
    raster_window(net, k, &Local::uniform(), (0, 0, w, h), px)
}

/// Rasterize the network onto the window `win` = (x0, y0, w, h) of the
/// pixel grid (a crop render); pixel centers stay whole-canvas ones.
pub(crate) fn raster_window(net: &Network, k: &Cracks, local: &Local, win: (usize, usize, usize, usize), px: f32) -> Raster {
    let (ox, oy, w, h) = win;
    let segs = rsegs(net, k);
    let width = k.width() * 1e-3; // mm
    let sh = 0.4 * width; // worn shoulder each side
    let lc = 0.18 * k.island(); // cupping falls off over ~10–20% of an island
    // lift ∝ 1 / distance near the crack (research notes, E.5), reaching
    // zero at lc; ℓ0 keeps it finite at the edge
    let l0 = 0.06 * k.island();
    let tail = l0 / (l0 + lc);
    // averaged over the pixel (a box of half width px/2 across the crack):
    // the integral of the profile from 0 to u, odd in u
    let prim = |u: f32| {
        let a = u.abs().min(lc);
        (l0 * (1.0 + a / l0).ln() - tail * a) / (1.0 - tail) * u.signum()
    };
    let hp = 0.5 * px;
    let cup_profile = |d: f32| ((prim(d + hp) - prim(d - hp)) / px).max(0.0);
    let reach = lc.max(0.5 * width * 1.8 + sh) + 1.5 * px;
    let fr = 0.75 * px; // tent filter radius
    // bin segments into bands of rows
    const BAND: usize = 16;
    let nb = h.div_ceil(BAND);
    let mut bins: Vec<Vec<u32>> = vec![Vec::new(); nb];
    for (i, s) in segs.iter().enumerate() {
        let (ga, gb) = (((s.a[1].min(s.b[1]) - reach) / px).floor().max(0.0) as usize, ((s.a[1].max(s.b[1]) + reach) / px).floor().max(0.0) as usize);
        if gb < oy || ga >= oy + h {
            continue;
        }
        let y0 = ga.saturating_sub(oy) / BAND;
        let y1 = ((gb - oy) / BAND).min(nb - 1);
        for b in bins.iter_mut().take(y1 + 1).skip(y0) {
            b.push(i as u32);
        }
    }
    let mut cover = vec![0.0f32; w * h];
    let mut shoulder = vec![0.0f32; w * h];
    let mut dz = vec![0.0f32; w * h];
    cover
        .par_chunks_mut(w * BAND)
        .zip(shoulder.par_chunks_mut(w * BAND))
        .zip(dz.par_chunks_mut(w * BAND))
        .enumerate()
        .for_each(|(bi, ((cv, shv), dzv))| {
            let rows = cv.len() / w;
            let y0 = bi * BAND;
            let mut deep = vec![0.0f32; w * rows];
            let mut cup = vec![0.0f32; w * rows];
            for &si in &bins[bi] {
                let s = &segs[si as usize];
                // whole-canvas pixels, clipped to this band of the window
                let (gy0, gy1) = (oy + y0, oy + y0 + rows - 1);
                let xa = (((s.a[0].min(s.b[0]) - reach) / px).floor().max(0.0) as usize).max(ox);
                let xb = (((s.a[0].max(s.b[0]) + reach) / px).ceil().max(0.0) as usize).min(ox + w - 1);
                let ya = (((s.a[1].min(s.b[1]) - reach) / px).floor().max(gy0 as f32) as usize).max(gy0);
                let yb = (((s.a[1].max(s.b[1]) + reach) / px).ceil().max(0.0) as usize).min(gy1);
                if ya > yb || xa > xb {
                    continue;
                }
                for y in ya..=yb {
                    let cy = (y as f32 + 0.5) * px;
                    for x in xa..=xb {
                        let c = [(x as f32 + 0.5) * px, cy];
                        let (t, p) = closest(c, s.a, s.b);
                        let dv = sub(c, p);
                        let d = dot(dv, dv).sqrt();
                        if d > reach {
                            continue;
                        }
                        // the film here went through this crack's
                        // generation or not (fading over one generation)
                        let (rch, of) = local.at(c);
                        let vis = (rch - s.generation).clamp(0.0, 1.0);
                        if vis <= 0.0 {
                            continue;
                        }
                        let wf = (s.wa + (s.wb - s.wa) * t) * of * vis;
                        let wd = width * wf;
                        let core = band_cover(d, wd, fr);
                        let shd = band_cover(d, wd + 2.0 * sh * wf, fr);
                        let i = (y - gy0) * w + x - ox;
                        cv[i] = cv[i].max(core);
                        shv[i] = shv[i].max(shd);
                        // groove: open crack at full depth, rounded shoulders shallow
                        let g = wf * (core + 0.3 * (shd - core));
                        deep[i] = deep[i].max(g);
                        cup[i] = cup[i].max(vis * cup_profile(d));
                    }
                }
            }
            for i in 0..w * rows {
                dzv[i] = k.cupping_um * cup[i] - k.depth_um * deep[i];
            }
        });
    Raster { cover, shoulder, dz }
}

impl Canvas {
    /// Craquelure (see `crack` module docs): grow a sequential crack network
    /// sized in mm, cut it into the surface relief (grooves, cupped island
    /// edges) and let grime settle in it. `None` fields of the recipe come
    /// from this canvas (`Cracks::fit` with the ground it was primed with);
    /// with `vary` the paint under each crack decides whether it formed and
    /// how wide it opened; with `veil` patches of microcracked varnish
    /// scatter a milky light.
    pub fn crack(&mut self, k: &Cracks) {
        self.dry();
        let k = &k.fit(self.ground_um);
        let f = self.f;
        let px = self.px_mm();
        let pitch = self.linen.map_or([10.0 / 14.0, 10.0 / 12.0], |l| [10.0 / l.warp_per_cm, 10.0 / l.weft_per_cm]);
        // the network grows over the whole canvas (a crop render too, so its
        // cracks are the same ones); only the window is rasterized
        let net = network(k, [f.full_w as f32 * px, f.full_h as f32 * px], pitch);
        let local = self.crack_local(k);
        let r = raster_window(&net, k, &local, (f.x0, f.y0, f.w, f.h), px);
        self.surf_gen += 1;
        self.height.par_iter_mut().zip(&r.dz).for_each(|(z, d)| *z += d);
        // an open crack is a deep narrow slot: it traps light (its walls and
        // floor are in shadow), and soot and dust in a little oil settle in it
        let dirt = Pigment::from_appearance([0.065, 0.054, 0.042], [0.014, 0.012, 0.01]);
        let th = 2.5 * k.dirt;
        self.px.par_iter_mut().enumerate().for_each(|(i, p)| {
            let c = (r.cover[i] + 0.25 * (r.shoulder[i] - r.cover[i])).min(1.0);
            if c > 0.0 {
                let slot = [p[0] * SLOT, p[1] * SLOT, p[2] * SLOT];
                let d = dirt.over(slot, th);
                for ch in 0..3 {
                    p[ch] += (d[ch] - p[ch]) * c;
                }
            }
        });
        if k.veil > 0.0 {
            self.varnish_veil(k, px);
        }
    }

    /// The local field for `raster_window`: the paint here (its total layer
    /// thickness and how light it is) on a ~1 mm grid over the window. The
    /// grid is the whole canvas's (cell edges at multiples of `n` canvas
    /// pixels), so a crop render averages the same pixels into the same
    /// cells as the whole render, wherever its corner falls. Cells cut by the
    /// window's edge average the part inside; that and the interpolation
    /// between cells reach two cells (~2 mm) in, well within a crop's
    /// margin (`run::DEFAULT_MARGIN`, 40 units), which isn't saved.
    fn crack_local(&self, k: &Cracks) -> Local {
        if k.vary <= 0.0 {
            return Local::uniform();
        }
        let f = self.f;
        let px = self.px_mm();
        let n = ((1.0 / px).round() as usize).max(2);
        // the canvas cells the window touches
        let (gx0, gy0) = (f.x0 / n, f.y0 / n);
        let (nx, ny) = ((f.x0 + f.w).div_ceil(n) - gx0, (f.y0 + f.h).div_ceil(n) - gy0);
        let ground = k.ground();
        let cells: Vec<(f32, f32)> = (0..nx * ny)
            .into_par_iter()
            .map(|ci| {
                // the cell's pixels in the buffer
                let (cx, cy) = (gx0 + ci % nx, gy0 + ci / nx);
                let (xa, xb) = ((cx * n).max(f.x0) - f.x0, ((cx + 1) * n).min(f.x0 + f.w) - f.x0);
                let (ya, yb) = ((cy * n).max(f.y0) - f.y0, ((cy + 1) * n).min(f.y0 + f.h) - f.y0);
                let (mut t, mut y, mut m) = (0.0, 0.0, 0.0);
                for yy in ya..yb {
                    for xx in xa..xb {
                        let i = yy * f.w + xx;
                        let p = self.px[i];
                        t += self.film[i] * COAT_UM;
                        y += 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
                        m += 1.0;
                    }
                }
                let (t, y) = (t / m, y / m);
                let paint = (t - ground).max(0.0);
                // lead-white-rich lights are brittle and went through every
                // generation; dark, oily earth and black glazes stayed
                // tougher and stopped up to two generations earlier
                let tough = 2.0 * (1.0 - crate::smoothstep(0.02, 0.25, y));
                // thick paint cracks more coarsely (spacing grows with the
                // layer), and each crack opens wider
                let thick = crate::smoothstep(30.0, 150.0, paint);
                let reach = GENERATIONS as f32 - k.vary * (tough + thick);
                let open = (1.0 + k.vary * (((ground + paint) / (ground + PAINT_UM).max(1.0)).sqrt() - 1.0)).clamp(0.7, 1.8);
                (reach, open)
            })
            .collect();
        let (reach, open) = cells.into_iter().unzip();
        Local { cell: n as f32 * px, o: [(gx0 * n) as f32 * px, (gy0 * n) as f32 * px], nx, ny, reach, open }
    }

    /// Patches of old varnish crazed into microcracks a few tenths of a mm
    /// apart; the crack edges scatter light, a milky veil over the picture
    /// (strongest over darks). Resolved as fine light lines where a pixel is
    /// finer than the crazing, as their mean haze where it is coarser.
    fn varnish_veil(&mut self, k: &Cracks, px: f32) {
        let f = self.f;
        let patches = Fbm::new(k.seed as u32 ^ 0x81, 5, 120.0).with_persistence(0.6);
        let cell = VEIL_CELL;
        let scatter_w = 0.04; // mm: the lit edge zone of a microcrack
        let mean = (2.0 * scatter_w / cell).min(1.0);
        let resolve = 1.0 - crate::smoothstep(0.5 * cell, 1.5 * cell, px);
        let seed = k.seed ^ 0x82;
        let milk = [0.52f32, 0.53, 0.55];
        self.px.par_chunks_mut(f.w).enumerate().for_each(|(y, row)| {
            let gy = (y + f.y0) as f32 * px + 0.5 * px;
            for (x, p) in row.iter_mut().enumerate() {
                let gx = (x + f.x0) as f32 * px + 0.5 * px;
                let patch = crate::smoothstep(0.0, 0.45, patches.get(gx, gy)) * k.veil;
                if patch <= 0.0 {
                    continue;
                }
                let lines = if resolve > 0.0 {
                    let d = voronoi_edge(gx / cell, gy / cell, seed) * cell;
                    mean + resolve * (band_cover(d, scatter_w, 0.75 * px) - mean)
                } else {
                    mean
                };
                let a = VEIL * patch * lines;
                for ch in 0..3 {
                    p[ch] += (milk[ch] - p[ch]) * a;
                }
            }
        });
    }
}

/// Reflectance left in an open crack (its shadowed slot) before grime.
const SLOT: f32 = 0.35;
/// Spacing of varnish microcracks, mm.
const VEIL_CELL: f32 = 0.3;
/// How far a lit microcrack edge veils the paint toward milky gray; a fully
/// crazed patch averages ~1.3% (a surface bloom: a percent or two of
/// diffuse light, visible over darks, lost over lights).
const VEIL: f32 = 0.05;

/// Distance (in cells) from (x, y) to the nearest edge of a jittered
/// Voronoi tessellation (half the gap between the nearest two sites).
fn voronoi_edge(x: f32, y: f32, seed: u64) -> f32 {
    let (cx, cy) = (x.floor() as i64, y.floor() as i64);
    let (mut d1, mut d2) = (f32::MAX, f32::MAX);
    let mut s1 = [0.0f32; 2];
    let mut sites = [[0.0f32; 2]; 9];
    let mut k = 0;
    for j in -1..=1 {
        for i in -1..=1 {
            let (gx, gy) = (cx + i, cy + j);
            let s = [gx as f32 + hash2(gx, gy, seed), gy as f32 + hash2(gx, gy, seed ^ 0x5)];
            sites[k] = s;
            k += 1;
            let dd = (s[0] - x).powi(2) + (s[1] - y).powi(2);
            if dd < d1 {
                d1 = dd;
                s1 = s;
            }
        }
    }
    // distance to the bisector with each other site; the nearest is the edge
    for s in sites {
        if s == s1 {
            continue;
        }
        let m = [(s[0] + s1[0]) * 0.5, (s[1] + s1[1]) * 0.5];
        let n = norm(sub(s, s1));
        let d = dot(sub(m, [x, y]), n);
        d2 = d2.min(d);
    }
    d2.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn net(k: &Cracks, size: [f32; 2]) -> Network {
        network(k, size, [10.0 / 15.0, 10.0 / 13.0])
    }

    /// Median island size (√area, mm) by flood-filling the crack mask.
    fn islands(k: &Cracks, size: [f32; 2], px: f32) -> (f32, usize) {
        let n = net(k, size);
        let (w, h) = ((size[0] / px) as usize, (size[1] / px) as usize);
        let r = raster(&n, &Cracks { width_um: Some(1.2 * px * 1000.0), ..*k }, w, h, px);
        let crack: Vec<bool> = r.cover.iter().map(|&c| c > 0.25).collect();
        let mut seen = vec![false; w * h];
        let mut areas = Vec::new();
        let mut stack = Vec::new();
        for s in 0..w * h {
            if crack[s] || seen[s] {
                continue;
            }
            let (mut area, mut edge) = (0usize, false);
            seen[s] = true;
            stack.push(s);
            while let Some(i) = stack.pop() {
                area += 1;
                let (x, y) = (i % w, i / w);
                if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
                    edge = true;
                }
                let mut go = |j: usize| {
                    if !crack[j] && !seen[j] {
                        seen[j] = true;
                        stack.push(j);
                    }
                };
                if x > 0 {
                    go(i - 1);
                }
                if x + 1 < w {
                    go(i + 1);
                }
                if y > 0 {
                    go(i - w);
                }
                if y + 1 < h {
                    go(i + w);
                }
            }
            // whole islands only, and not specks between close cracks
            if !edge && area as f32 * px * px > 0.04 * k.island() * k.island() {
                areas.push(area as f32 * px * px);
            }
        }
        areas.sort_by(f32::total_cmp);
        (areas[areas.len() / 2].sqrt(), areas.len())
    }

    #[test]
    fn deterministic_for_a_seed() {
        let k = Cracks::aged(5).fit(240.0);
        let (a, b) = (net(&k, [60.0, 40.0]), net(&k, [60.0, 40.0]));
        assert_eq!(a.arms.len(), b.arms.len());
        for (x, y) in a.arms.iter().zip(&b.arms) {
            assert_eq!(x.pts, y.pts);
            assert_eq!(x.end, y.end);
        }
        let c = net(&Cracks::aged(6).fit(240.0), [60.0, 40.0]);
        assert!(c.arms.len() != a.arms.len() || c.arms[3].pts != a.arms[3].pts);
    }

    #[test]
    fn median_island_matches_target() {
        for (island, ground) in [(2.0, 150.0), (4.0, 150.0), (3.0, 0.0), (6.0, 60.0)] {
            let k = Cracks { island_mm: Some(island), ground_um: Some(ground), corners: false, vary: 0.0, ..Cracks::aged(3) };
            let size = [island * 22.0, island * 16.0];
            let (m, n) = islands(&k, size, island / 40.0);
            eprintln!("island {island} mm, ground {ground} µm: median {m:.2} mm over {n} islands");
            assert!(n > 50, "too few islands: {n}");
            assert!(m > island / 1.5 && m < island * 1.5, "median {m} vs target {island}");
        }
    }

    #[test]
    fn junctions_are_mostly_t_shaped() {
        let k = Cracks { corners: false, vary: 0.0, ..Cracks::aged(8).fit(240.0) };
        let n = net(&k, [80.0, 60.0]);
        let (mut t, mut free) = (0, 0);
        for a in &n.arms {
            match a.end {
                End::Crack(_) => t += 1,
                End::Free => free += 1,
                End::Border => {}
            }
        }
        let frac = t as f32 / (t + free) as f32;
        eprintln!("T-junction ends {t}, free ends {free} ({frac:.2})");
        assert!(frac > 0.75, "only {frac} of crack ends meet another crack");
        // and they meet near 90°: compare the last step with the crack it hits
        let mut near_square = 0;
        let mut total = 0;
        for a in &n.arms {
            let (End::Crack(c), true) = (a.end, a.pts.len() >= 3) else { continue };
            let m = a.pts.len();
            let dir = norm(sub(a.pts[m - 1], a.pts[m.saturating_sub(4).max(0)]));
            let tip = a.pts[m - 1];
            // direction of the hit crack at the junction
            let mut best = (f32::MAX, [1.0, 0.0]);
            for b in n.arms.iter().filter(|b| b.crack == c) {
                for s in b.pts.windows(2) {
                    let (_, x) = closest(tip, s[0], s[1]);
                    let d = sub(tip, x);
                    let dd = dot(d, d);
                    if dd < best.0 && dot(sub(s[1], s[0]), sub(s[1], s[0])) > 0.0 {
                        best = (dd, norm(sub(s[1], s[0])));
                    }
                }
            }
            total += 1;
            if dot(dir, best.1).abs() < 0.5 {
                near_square += 1; // within 30° of perpendicular
            }
        }
        let sq = near_square as f32 / total as f32;
        eprintln!("junctions within 30° of square: {sq:.2}");
        assert!(sq > 0.6, "{sq}");
    }

    /// Length-weighted fraction of crack segments within 10° of warp or weft.
    fn axis_fraction(k: &Cracks) -> f32 {
        let n = net(k, [60.0, 45.0]);
        let (mut on, mut all) = (0.0, 0.0);
        for a in &n.arms {
            for s in a.pts.windows(2) {
                let d = sub(s[1], s[0]);
                let l = dot(d, d).sqrt();
                if l <= 0.0 {
                    continue;
                }
                let ang = d[1].atan2(d[0]).abs().to_degrees() % 90.0;
                if !(10.0..=80.0).contains(&ang) {
                    on += l;
                }
                all += l;
            }
        }
        on / all
    }

    #[test]
    fn thin_grounds_follow_the_weave() {
        let thin = axis_fraction(&Cracks { ground_um: Some(5.0), island_mm: Some(3.5), corners: false, ..Cracks::aged(4) });
        let thick = axis_fraction(&Cracks { ground_um: Some(200.0), island_mm: Some(3.5), corners: false, ..Cracks::aged(4) });
        eprintln!("near warp/weft: thin ground {thin:.2}, thick ground {thick:.2} (uniform 0.22)");
        assert!(thin > 0.8, "thin {thin}");
        assert!(thick < 0.45, "thick {thick}");
    }

    #[test]
    fn corner_cracks_cross_the_diagonal() {
        let k = Cracks { corners: true, island_mm: Some(3.0), vary: 0.0, ..Cracks::aged(2).fit(240.0) };
        let size = [120.0, 90.0];
        let n = net(&k, size);
        let diag = norm(size);
        let zone = 0.05 * (size[0] * size[0] + size[1] * size[1]).sqrt();
        let (mut perp_len, mut all) = (0.0, 0.0);
        for a in &n.arms {
            for s in a.pts.windows(2) {
                let m = mul(addv(s[0], s[1]), 0.5);
                if dot(m, m).sqrt() > zone {
                    continue;
                }
                let d = sub(s[1], s[0]);
                let l = dot(d, d).sqrt();
                if l > 0.0 && (dot(d, diag) / l).abs() < 0.5 {
                    perp_len += l;
                }
                all += l;
            }
        }
        eprintln!("corner: {:.2} of crack length within 30° of perpendicular to the diagonal", perp_len / all);
        assert!(perp_len / all > 0.6);
    }

    /// A crop render judges the paint under its cracks on the same cells as
    /// the whole canvas: the averaging grid is anchored to the canvas, not
    /// to the crop's corner, so a crop that doesn't start on a cell boundary
    /// sees the same reach and opening away from its edge (it used to see
    /// light and dark strips averaged differently and grow a generation-4
    /// crack the whole render didn't have).
    #[test]
    fn crop_origin_doesnt_move_the_local_field() {
        let k = Cracks { vary: 1.0, ..Cracks::aged(4).fit(240.0) };
        let make = |crop: Option<crate::canvas::Crop>| {
            let mut c = Canvas::new_window(400, 1.0, [0.5; 3], crop).with_size_mm(100.0);
            let f = c.f;
            for y in 0..f.h {
                for x in 0..f.w {
                    // light and dark strips 4 px wide (a cell is 4 px), and
                    // a film that thickens in steps down the canvas
                    let (gx, gy) = (x + f.x0, y + f.y0);
                    c.px[y * f.w + x] = if (gx / 4) % 2 == 0 { [0.01; 3] } else { [0.4; 3] };
                    c.film[y * f.w + x] = 9.6 + 2.0 * ((gy / 3) % 5) as f32;
                }
            }
            c
        };
        let whole = make(None);
        let crop = make(Some(crate::canvas::Crop { units: [102.5, 104.0, 300.0, 250.0], margin: 0.0 }));
        let (f, px) = (crop.f, crop.px_mm());
        assert!(f.x0 % 4 != 0 && f.y0 % 4 != 0, "crop at ({}, {}) should cut the cells", f.x0, f.y0);
        let (a, b) = (whole.crack_local(&k), crop.crack_local(&k));
        // (two cells from the crop's edge: its margin, in a real crop render)
        for y in f.y0 + 8..f.y0 + f.h - 8 {
            for x in f.x0 + 8..f.x0 + f.w - 8 {
                let p = [(x as f32 + 0.5) * px, (y as f32 + 0.5) * px];
                let (ra, oa) = a.at(p);
                let (rb, ob) = b.at(p);
                assert!((ra - rb).abs() < 1e-4 && (oa - ob).abs() < 1e-4, "at pixel ({x}, {y}): whole ({ra}, {oa}), crop ({rb}, {ob})");
            }
        }
    }

    #[test]
    fn height_changes_only_near_cracks() {
        let mut c = Canvas::new(300, 1.25, [0.6, 0.55, 0.45]).with_size_mm(40.0);
        c.prime([0.7, 0.6, 0.5], 0.8, 50.0, 0.3, 0.3, 1);
        let before = c.height.clone();
        let px_before = c.px.clone();
        let g0 = c.surf_gen;
        let k = Cracks { vary: 0.0, veil: 0.0, width_um: Some(70.0), depth_um: 35.0, cupping_um: 30.0, ..Cracks::aged(9).fit(50.0) };
        c.crack(&k);
        assert!(c.surf_gen > g0);
        let px = c.px_mm();
        let (w, h) = (c.f.w, c.f.h);
        let n = network(&k, [w as f32 * px, h as f32 * px], [10.0 / 14.0, 10.0 / 12.0]);
        let segs: Vec<(V2, V2)> = n.arms.iter().flat_map(|a| a.pts.windows(2).map(|s| (s[0], s[1])).collect::<Vec<_>>()).collect();
        let reach = 0.18 * k.island() + 2.0 * px;
        let mut changed = 0;
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                let dz = c.height[i] - before[i];
                let dc = (c.px[i][0] - px_before[i][0]).abs();
                if dz.abs() < 1e-4 && dc < 1e-6 {
                    continue;
                }
                changed += 1;
                let p = [(x as f32 + 0.5) * px, (y as f32 + 0.5) * px];
                let d = segs.iter().map(|&(a, b)| {
                    let (_, q) = closest(p, a, b);
                    let v = sub(p, q);
                    dot(v, v).sqrt()
                }).fold(f32::MAX, f32::min);
                assert!(d <= reach, "pixel ({x},{y}) changed {dz} µm but is {d} mm from a crack");
            }
        }
        assert!(changed > w * h / 20, "cracks changed only {changed} pixels");
        // grooves go down, cupped edges up
        let (lo, hi) = c.height.iter().zip(&before).fold((0.0f32, 0.0f32), |(lo, hi), (a, b)| (lo.min(a - b), hi.max(a - b)));
        assert!(lo < -5.0 && hi > 5.0, "dz range {lo}..{hi}");
    }

    /// `cargo test --release -p paint full_canvas_speed -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn full_canvas_speed() {
        for (mm, island, ground) in [(440.0, 3.5, 240.0), (440.0, 3.5, 0.0), (1714.0, 5.0, 210.0)] {
            let mut c = Canvas::new(3200, 1.4, [0.6, 0.55, 0.45]).with_size_mm(mm);
            let k = Cracks { island_mm: Some(island), ground_um: Some(ground), ..Cracks::aged(1) };
            let t = std::time::Instant::now();
            let px = c.px_mm();
            let n = network(&k, [3200.0 * px, c.f.h as f32 * px], [10.0 / 14.0, 10.0 / 12.0]);
            let tn = t.elapsed().as_secs_f32();
            c.crack(&k);
            let tc = t.elapsed().as_secs_f32() - tn;
            eprintln!("{}x{} px, {mm} mm, island {island} mm, ground {ground} µm: {} arms; network {tn:.2}s, crack() {tc:.2}s", c.f.w, c.f.h, n.arms.len());
        }
    }
}
