//! The relief of bare plain-weave linen under `--@ relief-prefilter`: the
//! top of the threads as they lie in the cloth, each following its crimped
//! path over and under the threads it crosses. The geometry is Peirce's
//! (F. T. Peirce, "The geometry of cloth structure", J. Textile Inst. 28,
//! T45–T96, 1937); the threads' sizes and crimp are measured ones. The
//! sources and the working are in notes/research/weave_geometry.md.
//!
//! **The threads.** Warp threads run down the canvas (one every
//! `10 / warp_per_cm` mm across it), weft threads across it. Each has a
//! lens-shaped cross-section, wider than tall, as cut and measured in
//! artists' linen: a lens as wide as it is tall is a circle, Peirce's round
//! thread.
//!
//! **The crimp.** Where a warp passes over a weft, its axis bends round the
//! weft's at the distance the two threads' heights make together, half of
//! `D = g_warp + g_weft`; it then runs straight, tangent to that arc and to
//! the one it makes under the next weft (Peirce's arcs and lines). The two
//! threads' crimp heights (how far each axis rises and falls) add up to `D`
//! (Peirce's h₁ + h₂ = D). Given the crimp of both, as measured on
//! nineteenth-century painting canvases, Peirce's equations give `D` and
//! how it splits between them; the threads' measured heights share `D`.
//! With threads of different thickness at each crossing (thick and thin
//! threads, slubs) every crossing has its own `D`; the arc a thread makes
//! round a crossing thread is centered on that thread's axis, the same arc
//! seen from either side, so each thread's path is smooth (its height and
//! slope continuous) everywhere.
//!
//! **The surface.** Each thread's top is its axis raised by its section's
//! half-height there (an offset of the path, exact for the arcs and the
//! straight runs alike), and the paint meets the highest thread at each
//! point. Where no thread lies (the small holes between threads) it meets
//! the cloth's underside. The threads' surfaces meet in creases, and the
//! relief drops at a thread's edge where it lies over a lower thread or a
//! hole, by no more than the thread's axis stands above the floor: nowhere
//! else does the height jump.

use crate::surface::{Linen, vnoise};
use rayon::prelude::*;

/// The crimp of warp and weft: how much longer a thread is than the cloth
/// it crosses. Means over eight linen painting canvases of 1845 to the
/// late 19th century (Iaccarino Idelson, Bergsma and Groves, "Quantitative
/// morphological analysis of warp and weft yarns in historical woven
/// structures", Studies in Conservation, 2025, Table 7: Postiglione,
/// Dehodencq, Gélibert, Auguin, Alleaume, Sauzeau, Müller, Furcy de
/// Lavault): warp 5.2–15.3%, weft 2.3–6.8%. The warp does most of the
/// bending, as in nearly every canvas.
pub(crate) const WARP_CRIMP: f32 = 0.1185;
pub(crate) const WEFT_CRIMP: f32 = 0.0438;
/// The warp's share of the two threads' heights together, on the same
/// canvases (yarn thickness, Table 4: 0.44–0.54, mean 0.486).
pub(crate) const WARP_SHARE: f32 = 0.486;
/// A thread's width in the cloth, times its spacing, on the same canvases
/// (yarn width as the cloth shows it, Table 4): warp 0.48–1.21, mean
/// 1.023, so the warp threads lie edge to edge and press into each other;
/// weft 0.56–0.87, mean 0.769.
pub(crate) const WARP_WIDTH: f32 = 1.023;
pub(crate) const WEFT_WIDTH: f32 = 0.769;
/// How much fibres lying loose on the threads roughen their tops, µm from
/// lowest to highest: about one elementary flax fibre across (10–25 µm).
/// An estimate: no measurement of a linen yarn's surface was found, and
/// the fibres' diameter only bounds it.
pub(crate) const FUZZ_UM: f32 = 15.0;

/// The thinnest and thickest a warp thread gets (`Weave::crossing`).
const WARP_THICK: (f32, f32) = (0.85, 1.15);

/// A plain weave's geometry for a linen, mm: the threads' spacings, their
/// sections (width and height, before each thread's own thickness), how
/// the crimp splits between warp and weft, and the cloth's underside.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Weave {
    pub(crate) linen: Linen,
    /// Warp spacing (across the canvas) and weft spacing (down it), mm.
    pub(crate) s_wa: f32,
    pub(crate) s_we: f32,
    /// Warp section's width and height, mm.
    pub(crate) e1: f32,
    pub(crate) g1: f32,
    /// Weft section's width and height, mm.
    pub(crate) e2: f32,
    pub(crate) g2: f32,
    /// The warp's share of the crimp heights at each crossing: h₁ = f₁·D,
    /// h₂ = (1 − f₁)·D.
    pub(crate) f1: f32,
    /// Where no thread lies, the height the paint meets, mm from the
    /// cloth's middle (below every thread's top).
    pub(crate) floor: f32,
}

/// The thread on top at a point (`Weave::top_detail`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Top {
    /// Its top, mm from the cloth's middle.
    pub(crate) z: f32,
    /// A warp thread (or a weft thread), and which.
    pub(crate) warp: bool,
    pub(crate) index: i64,
    /// How far its edge is, mm across it.
    pub(crate) edge: f32,
}

/// The angle a thread's straight run makes with the cloth, between an arc
/// of radius `r0` round a center `a` high and one of radius `r1` round a
/// center `b` high, `p` further along (Peirce's tangent to both arcs:
/// p sin θ + (b − a) cos θ = r0 + r1).
fn tangent_angle(p: f32, a: f32, r0: f32, b: f32, r1: f32) -> f32 {
    let hb = b - a;
    let l = (p * p + hb * hb).sqrt();
    (((r0 + r1) / l).min(1.0).asin() - hb.atan2(p)).clamp(0.0, 1.5)
}

/// A thread's path from a crossing where it lies over a thread to the next
/// where it lies under one (`p` further on), offset `q` up (`up`) or down:
/// it bends round the first thread's axis (`a` high) at radius `r0`, runs
/// straight, and bends under the next's (`b` high) at radius `r1`. The
/// offset of an arc is an arc about the same center, of a straight run a
/// straight run `q / cos θ` higher, so the offset path is as smooth as the
/// path. Height at `y` along it, mm.
#[allow(clippy::too_many_arguments)]
fn crimped(y: f32, p: f32, a: f32, r0: f32, b: f32, r1: f32, q: f32, up: bool) -> f32 {
    let th = tangent_angle(p, a, r0, b, r1);
    let (sn, cs) = th.sin_cos();
    let (ra, rb, off) = if up { (r0 + q, (r1 - q).max(0.0), q) } else { ((r0 - q).max(0.0), r1 + q, -q) };
    if y <= ra * sn {
        a + (ra * ra - y * y).max(0.0).sqrt()
    } else if y >= p - rb * sn {
        let d = p - y;
        b - (rb * rb - d * d).max(0.0).sqrt()
    } else {
        a + r0 * cs - (y - r0 * sn) * (sn / cs) + off / cs
    }
}

/// The half-height of a lens-shaped section `e` wide and `g` tall, `d`
/// from its middle (0 past its edge): two circular arcs meeting at its
/// edges. As wide as it is tall it is a circle.
pub(crate) fn lens(d: f32, e: f32, g: f32) -> f32 {
    let h = 0.5 * e;
    if g <= 0.0 || d.abs() >= h {
        return 0.0;
    }
    let r = (h * h + 0.25 * g * g) / g;
    ((r * r - d * d).max(0.0).sqrt() - (r - 0.5 * g)).max(0.0)
}

/// Peirce's crimp of a thread whose crimp height is `h`, crossing threads
/// spaced `p` apart, with `D` the two threads' heights together: its length
/// over the cloth's, less one.
pub(crate) fn peirce_crimp(p: f64, d: f64, h: f64) -> f64 {
    let (pf, df, hf) = (p as f32, d as f32, h as f32);
    let th = tangent_angle(pf, -0.5 * (df - hf), 0.5 * df, 0.5 * (df - hf), 0.5 * df) as f64;
    let l = (p - d * th.sin()) / th.cos() + d * th;
    l / p - 1.0
}

/// The crimp height Peirce gives a thread of crimp `c` (`peirce_crimp`
/// inverted, by bisection).
pub(crate) fn peirce_height(p: f64, d: f64, c: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, d);
    for _ in 0..60 {
        let m = 0.5 * (lo + hi);
        if peirce_crimp(p, d, m) < c {
            lo = m;
        } else {
            hi = m;
        }
    }
    0.5 * (lo + hi)
}

/// The two threads' heights together `D` and the warp's crimp height h₁
/// that Peirce's geometry gives a plain weave of warp spacing `s_wa` and
/// weft spacing `s_we` whose warp and weft have crimps `c1` and `c2`: the
/// warp's crimp height for c₁ past wefts s_we apart and the weft's for c₂
/// past warps s_wa apart add up to D (h₁ + h₂ = D), solved for D. Where no
/// D does it (thread counts far apart), the threads' heights are 0.78 of
/// their mean spacing, as on every canvas measured, the crimp split as the
/// warp's crimp gives it.
pub(crate) fn peirce_weave(s_wa: f64, s_we: f64, c1: f64, c2: f64) -> (f64, f64) {
    let gap = |d: f64| peirce_height(s_we, d, c1) + peirce_height(s_wa, d, c2) - d;
    let top = 2.0 * s_wa.max(s_we);
    let steps = 400;
    let (mut lo, mut glo) = (top / steps as f64, gap(top / steps as f64));
    for k in 2..=steps {
        let hi = top * k as f64 / steps as f64;
        let ghi = gap(hi);
        if (glo > 0.0) != (ghi > 0.0) {
            let (mut a, mut b) = (lo, hi);
            for _ in 0..60 {
                let m = 0.5 * (a + b);
                if (gap(m) > 0.0) == (glo > 0.0) {
                    a = m;
                } else {
                    b = m;
                }
            }
            let d = 0.5 * (a + b);
            return (d, peirce_height(s_we, d, c1));
        }
        (lo, glo) = (hi, ghi);
    }
    let d = 0.78 * 0.5 * (s_wa + s_we);
    (d, peirce_height(s_we, d, c1).min(d))
}

impl Weave {
    /// The weave of linen `l`: the crimp heights and `D` Peirce gives the
    /// measured crimps for its spacings (`peirce_weave`), `D` shared by the
    /// threads' measured heights, their widths in proportion to their
    /// spacing as measured. A cloth can't be woven tighter than its threads
    /// allow (Peirce's jammed limit): where the thread counts asked for leave
    /// no room for the thickest crossing (far outside the canvases
    /// measured), the threads are made thinner until it fits.
    pub(crate) fn new(l: &Linen) -> Weave {
        let (s_wa, s_we) = (10.0 / l.warp_per_cm, 10.0 / l.weft_per_cm);
        let (d, h1) = peirce_weave(s_wa as f64, s_we as f64, WARP_CRIMP as f64, WEFT_CRIMP as f64);
        let f1 = (h1 / d) as f32;
        let (e1, e2) = (WARP_WIDTH * s_wa, WEFT_WIDTH * s_we);
        // the thickest crossing must fit both threads' paths: the warp's
        // straight run between wefts s_we apart whose axes its crimp lifts
        // (1 − f1)·D apart, and the weft's between warps s_wa apart, f1·D
        // apart (Peirce's tangent needs D ≤ √(p² + (h_other)²))
        let grow = WARP_SHARE * WARP_THICK.1 + (1.0 - WARP_SHARE) * Self::weft_thick(l).1;
        let dmax = d as f32 * grow;
        let k = (0.98 * s_we / (dmax * (1.0 - (1.0 - f1).powi(2)).max(1e-6).sqrt())).min(0.98 * s_wa / (dmax * (1.0 - f1 * f1).max(1e-6).sqrt())).min(1.0);
        let d = d as f32 * k;
        let (g1, g2) = (WARP_SHARE * d, (1.0 - WARP_SHARE) * d);
        let (h1, h2) = (f1 * d, (1.0 - f1) * d);
        let lowest = 0.5 * f1.max(1.0 - f1) * d * grow;
        let floor = -(0.5 * (h1 + g1).max(h2 + g2)).max(lowest);
        Weave { linen: *l, s_wa, s_we, e1, g1, e2, g2, f1, floor }
    }

    /// The thinnest and thickest a weft thread gets, its slubs with it.
    fn weft_thick(l: &Linen) -> (f32, f32) {
        (0.8, 1.1 + l.slubs.max(0.0) * 1.2 * 0.4)
    }

    /// Weft thread `j`'s thickness `u` threads across (relative), with its
    /// slubs (thick spots a few mm long), as `surface::linen_um` makes it.
    fn weft_thickness(&self, u: f32, j: i64) -> f32 {
        let (seed, slubs) = (self.linen.seed, self.linen.slubs);
        let slub = vnoise(u * 0.25, j as f32 * 0.93, seed + 13);
        0.8 + 0.3 * vnoise(u * 0.4, j as f32 * 0.93, seed + 17) + slubs * 1.2 * (slub - 0.6).max(0.0)
    }

    /// How thick warp `i` and weft `j` are where they cross (relative), as
    /// `surface::linen_um` makes them: the warp even, varying slowly along
    /// it, the weft with its slubs. Between crossings a thread's thickness
    /// runs straight from one crossing's to the next's (they vary over
    /// several threads).
    pub(crate) fn crossing(&self, i: i64, j: i64) -> (f32, f32) {
        let tw = WARP_THICK.0 + (WARP_THICK.1 - WARP_THICK.0) * vnoise(i as f32 * 0.93, (j as f32 + 0.5) * 0.3, self.linen.seed + 11);
        (tw, self.weft_thickness(i as f32 + 0.5, j))
    }

    /// The highest thread's top at `(u, v)` (threads across, threads down:
    /// warp `i` lies along u = i + ½, weft `j` along v = j + ½, the warp on
    /// top where i + j is even), mm from the cloth's middle, for threads as
    /// thick as `cross(i, j)` makes them where they cross (`crossing`), with
    /// which thread it is and how far its edge is; `None` where no thread
    /// lies. Only the four crossings round the point matter: the threads
    /// through them are the only ones that reach it.
    pub(crate) fn top_detail(&self, u: f32, v: f32, cross: &dyn Fn(i64, i64) -> (f32, f32)) -> Option<Top> {
        let f1 = self.f1;
        let (i0, j0) = ((u - 0.5).floor() as i64, (v - 0.5).floor() as i64);
        let (fu, fv) = (u - 0.5 - i0 as f32, v - 0.5 - j0 as f32);
        // c[dj][di]: crossing (i0 + di, j0 + dj)
        let c = [[cross(i0, j0), cross(i0 + 1, j0)], [cross(i0, j0 + 1), cross(i0 + 1, j0 + 1)]];
        // a crossing's two threads' heights together
        let dd = |di: usize, dj: usize| self.g1 * c[dj][di].0 + self.g2 * c[dj][di].1;
        let mut best: Option<Top> = None;
        let mut take = |t: Top| {
            if best.is_none_or(|b| t.z > b.z) {
                best = Some(t);
            }
        };
        let (x, y) = (fu * self.s_wa, fv * self.s_we);
        for di in 0..2 {
            let t = c[0][di].0 + (c[1][di].0 - c[0][di].0) * fv;
            let dx = (fu - di as f32) * self.s_wa;
            if dx.abs() >= 0.5 * self.e1 * t {
                continue;
            }
            let q = lens(dx, self.e1 * t, self.g1 * t);
            let (d0, d1) = (dd(di, 0), dd(di, 1));
            let (a, b) = (-0.5 * (1.0 - f1) * d0, 0.5 * (1.0 - f1) * d1);
            let i = i0 + di as i64;
            let z = if (i + j0).rem_euclid(2) == 0 { crimped(y, self.s_we, a, 0.5 * d0, b, 0.5 * d1, q, true) } else { -crimped(y, self.s_we, a, 0.5 * d0, b, 0.5 * d1, q, false) };
            take(Top { z, warp: true, index: i, edge: 0.5 * self.e1 * t - dx.abs() });
        }
        for dj in 0..2 {
            let t = c[dj][0].1 + (c[dj][1].1 - c[dj][0].1) * fu;
            let dy = (fv - dj as f32) * self.s_we;
            if dy.abs() >= 0.5 * self.e2 * t {
                continue;
            }
            let q = lens(dy, self.e2 * t, self.g2 * t);
            let (d0, d1) = (dd(0, dj), dd(1, dj));
            let (a, b) = (-0.5 * f1 * d0, 0.5 * f1 * d1);
            let j = j0 + dj as i64;
            let z = if (i0 + j).rem_euclid(2) == 1 { crimped(x, self.s_wa, a, 0.5 * d0, b, 0.5 * d1, q, true) } else { -crimped(x, self.s_wa, a, 0.5 * d0, b, 0.5 * d1, q, false) };
            take(Top { z, warp: false, index: j, edge: 0.5 * self.e2 * t - dy.abs() });
        }
        best
    }

    /// Where the point `(xm, ym)` (mm) lies among the threads (`top_detail`'s
    /// u, v): they wander a little, the weft more than the warp, as
    /// `surface::linen_um` makes them.
    fn threads_at(&self, xm: f32, ym: f32) -> (f32, f32) {
        let l = &self.linen;
        let seed = l.seed;
        let (x0, y0) = (xm * l.warp_per_cm / 10.0, ym * l.weft_per_cm / 10.0);
        let u = x0 + 0.25 * (vnoise(x0 * 0.45, y0 * 0.06, seed) - 0.5);
        let v = y0 + 0.5 * l.slubs * (vnoise(x0 * 0.06, y0 * 0.45, seed + 3) - 0.5) + 0.25 * (vnoise(x0 * 0.2, y0 * 0.9, seed + 9) - 0.5);
        (u, v)
    }

    /// Bare linen's height at a point (mm), µm above the cloth's underside:
    /// the highest thread's top, the threads wandering a little and thick
    /// and thin as `crossing` makes them, with the fibres' fuzz on them.
    /// The crossings' thicknesses come from `table` where it holds them
    /// (the same values, read once).
    pub(crate) fn height_um(&self, xm: f32, ym: f32, table: Option<&CrossTable>) -> f32 {
        self.height_detail(xm, ym, table).0
    }

    /// `height_um`, with the thread on top there (`top_detail`).
    pub(crate) fn height_detail(&self, xm: f32, ym: f32, table: Option<&CrossTable>) -> (f32, Option<Top>) {
        let (u, v) = self.threads_at(xm, ym);
        let top = match table {
            Some(t) => self.top_detail(u, v, &|i, j| t.get(i, j).unwrap_or_else(|| self.crossing(i, j))),
            None => self.top_detail(u, v, &|i, j| self.crossing(i, j)),
        };
        let l = &self.linen;
        let (x0, y0) = (xm * l.warp_per_cm / 10.0, ym * l.weft_per_cm / 10.0);
        let z = match top {
            Some(t) => (1000.0 * (t.z - self.floor) + FUZZ_UM * (vnoise(x0 * 3.1, y0 * 3.1, l.seed + 19) - 0.5)).max(0.0),
            None => 0.0,
        };
        (z, top)
    }

    /// The weave's mean height (µm above the underside) over a period of
    /// its over and under (two threads each way) for every thread `tw`
    /// (warp) and `th` (weft) thick, by the midpoint rule on `n` × `n`
    /// points a thread.
    pub(crate) fn mean_um(&self, tw: f32, th: f32, n: usize) -> f32 {
        let mut acc = 0.0f64;
        for jv in 0..2 * n {
            let v = (jv as f32 + 0.5) / n as f32;
            for iu in 0..2 * n {
                let u = (iu as f32 + 0.5) / n as f32;
                let z = self.top_detail(u, v, &|_, _| (tw, th)).map_or(0.0, |t| t.z - self.floor);
                acc += z as f64;
            }
        }
        (1000.0 * acc / (4 * n * n) as f64) as f32
    }
}

/// The crossings' thicknesses (`Weave::crossing`) over a window of the
/// canvas, read once: every point of the weave needs the four crossings
/// round it, and reading them is most of its cost.
pub(crate) struct CrossTable {
    i0: i64,
    j0: i64,
    ni: usize,
    nj: usize,
    v: Vec<(f32, f32)>,
}

impl CrossTable {
    /// The crossings any point from `(x0, y0)` to `(x1, y1)` (mm) can need,
    /// the threads' wandering included.
    pub(crate) fn new(weave: &Weave, (x0, y0): (f32, f32), (x1, y1): (f32, f32)) -> CrossTable {
        let l = &weave.linen;
        // the threads wander under a thread across, the weft by up to
        // ½ slubs + ⅛ of a thread down
        let reach = 2.0 + 0.5 * l.slubs.abs();
        let i0 = (x0 * l.warp_per_cm / 10.0 - reach).floor() as i64;
        let i1 = (x1 * l.warp_per_cm / 10.0 + reach).ceil() as i64;
        let j0 = (y0 * l.weft_per_cm / 10.0 - reach).floor() as i64;
        let j1 = (y1 * l.weft_per_cm / 10.0 + reach).ceil() as i64;
        let (ni, nj) = ((i1 - i0 + 1).max(0) as usize, (j1 - j0 + 1).max(0) as usize);
        let v = (0..ni * nj).into_par_iter().map(|k| weave.crossing(i0 + (k % ni) as i64, j0 + (k / ni) as i64)).collect();
        CrossTable { i0, j0, ni, nj, v }
    }

    /// The thicknesses at crossing `(i, j)`, if the table holds it.
    pub(crate) fn get(&self, i: i64, j: i64) -> Option<(f32, f32)> {
        let (a, b) = (i - self.i0, j - self.j0);
        if a < 0 || b < 0 || a as usize >= self.ni || b as usize >= self.nj {
            return None;
        }
        Some(self.v[b as usize * self.ni + a as usize])
    }
}

/// The weave's mean height against its threads' thicknesses
/// (`Weave::mean_um`), tabulated for the thicknesses `linen_um` gives them
/// and read bilinearly, for pixels at least a thread wide.
pub(crate) struct WeaveMean {
    weave: Weave,
    tw: (f32, f32),
    th: (f32, f32),
    table: Vec<f32>,
}

/// Steps of the mean's table in the warp's and the weft's thickness.
const MEAN_TW: usize = 6;
const MEAN_TH: usize = 12;
/// Points a thread each way the table's means are read at.
const MEAN_POINTS: usize = 48;

impl WeaveMean {
    pub(crate) fn new(weave: &Weave) -> WeaveMean {
        let (tw, th) = (WARP_THICK, Weave::weft_thick(&weave.linen));
        let table = (0..(MEAN_TW + 1) * (MEAN_TH + 1))
            .into_par_iter()
            .map(|k| {
                let (a, b) = (k % (MEAN_TW + 1), k / (MEAN_TW + 1));
                weave.mean_um(tw.0 + (tw.1 - tw.0) * a as f32 / MEAN_TW as f32, th.0 + (th.1 - th.0) * b as f32 / MEAN_TH as f32, MEAN_POINTS)
            })
            .collect();
        WeaveMean { weave: *weave, tw, th, table }
    }

    /// The mean for threads `tw` and `th` thick.
    pub(crate) fn at(&self, tw: f32, th: f32) -> f32 {
        let s = ((tw - self.tw.0) / (self.tw.1 - self.tw.0) * MEAN_TW as f32).clamp(0.0, MEAN_TW as f32);
        let t = ((th - self.th.0) / (self.th.1 - self.th.0).max(1e-6) * MEAN_TH as f32).clamp(0.0, MEAN_TH as f32);
        let (a, b) = ((s as usize).min(MEAN_TW - 1), (t as usize).min(MEAN_TH - 1));
        let (fs, ft) = (s - a as f32, t - b as f32);
        let at = |a: usize, b: usize| self.table[b * (MEAN_TW + 1) + a];
        let lo = at(a, b) + (at(a + 1, b) - at(a, b)) * fs;
        let hi = at(a, b + 1) + (at(a + 1, b + 1) - at(a, b + 1)) * fs;
        lo + (hi - lo) * ft
    }

    /// Bare linen's height at a point (mm), µm, where the pixels are at
    /// least a thread wide: the weave's mean over its over and under for the
    /// warp and weft threads there, as thick as `Weave::height_um` makes
    /// them, the weft's slubs with them. A pixel this wide resolves neither
    /// the threads nor their over and under (twice their pitch, past the
    /// footprint's band), only the weave's mean and how it varies from
    /// thread to thread and slub to slub. The thicknesses each whole thread
    /// has (by its index) vary smoothly here, matching each thread at its
    /// middle; the threads' wandering (a fraction of a thread) is left out.
    pub(crate) fn height_um(&self, xm: f32, ym: f32) -> f32 {
        let l = &self.weave.linen;
        let (x0, y0) = (xm * l.warp_per_cm / 10.0, ym * l.weft_per_cm / 10.0);
        let tw = WARP_THICK.0 + (WARP_THICK.1 - WARP_THICK.0) * vnoise((x0 - 0.5) * 0.93, y0 * 0.3, l.seed + 11);
        // (each weft's thickness, by its index, blended to the next's
        // between their middles)
        let y = y0 - 0.5;
        let j = y.floor();
        let (t0, t1) = (self.weave.weft_thickness(x0, j as i64), self.weave.weft_thickness(x0, j as i64 + 1));
        let th = t0 + (t1 - t0) * (y - j);
        self.at(tw, th)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linen(warp: f32, weft: f32) -> Linen {
        Linen { warp_per_cm: warp, weft_per_cm: weft, crown_um: 160.0, slubs: 0.7, seed: 1880 }
    }

    /// Even threads (every crossing's thicknesses 1).
    fn even(_: i64, _: i64) -> (f32, f32) {
        (1.0, 1.0)
    }

    /// The crimp heights are Peirce's for the threads' sizes and spacing:
    /// they add up to the two threads' heights (h₁ + h₂ = D) and give warp
    /// and weft their measured crimps, and the threads' tops where they
    /// cross lie at their axes' crimp height plus their half-height. Each
    /// path, measured point by point, is as long as Peirce's arcs and
    /// straight runs make it (l = (p − D sin θ)/cos θ + D θ, the crimp it
    /// gives), runs from +h/2 to −h/2 between two crossings, and nowhere
    /// comes nearer the axis of a thread it crosses than the two threads'
    /// half-heights: the threads touch and don't cut into each other.
    #[test]
    fn crimp_heights_follow_peirce() {
        for (warp, weft) in [(14.0, 12.0), (16.0, 15.0), (17.0, 14.5), (23.0, 23.0), (12.0, 11.0), (20.0, 16.0)] {
            let w = Weave::new(&linen(warp, weft));
            let d = w.g1 + w.g2;
            let (h1, h2) = (w.f1 * d, (1.0 - w.f1) * d);
            assert!(((h1 + h2) / d - 1.0).abs() < 1e-6);
            let c1 = peirce_crimp(w.s_we as f64, d as f64, h1 as f64);
            let c2 = peirce_crimp(w.s_wa as f64, d as f64, h2 as f64);
            assert!((c1 / WARP_CRIMP as f64 - 1.0).abs() < 2e-3 && (c2 / WEFT_CRIMP as f64 - 1.0).abs() < 2e-3, "{warp}x{weft}: crimps {c1}, {c2}");
            // warp 0 over weft 0 at (½, ½); weft 0 over warp 1 at (1½, ½)
            let crown = w.top_detail(0.5, 0.5, &even).unwrap();
            assert!(crown.warp && (crown.z - 0.5 * (h1 + w.g1)).abs() < 1e-5, "{crown:?}");
            let weft_top = w.top_detail(1.5, 0.5, &even).unwrap();
            assert!(!weft_top.warp && (weft_top.z - 0.5 * (h2 + w.g2)).abs() < 1e-5, "{weft_top:?}");
            // each thread's axis, past the threads it crosses
            for (name, p, h, other) in [("warp", w.s_we, h1, h2), ("weft", w.s_wa, h2, h1)] {
                let (a, b, r) = (-0.5 * other, 0.5 * other, 0.5 * d);
                let axis = |y: f32| crimped(y, p, a, r, b, r, 0.0, true);
                assert!((axis(0.0) - 0.5 * h).abs() < 1e-6 && (axis(p) + 0.5 * h).abs() < 1e-6, "{name}: {} to {}", axis(0.0), axis(p));
                let n = 20000;
                let (mut len, mut prev, mut nearest) = (0.0f64, axis(0.0), f32::MAX);
                for k in 1..=n {
                    let y = p * k as f32 / n as f32;
                    let z = axis(y);
                    len += ((p / n as f32) as f64).hypot((z - prev) as f64);
                    nearest = nearest.min(y.hypot(z - a)).min((p - y).hypot(z - b));
                    prev = z;
                }
                let c = len / p as f64 - 1.0;
                let peirce = peirce_crimp(p as f64, d as f64, h as f64);
                eprintln!("{warp} x {weft}/cm, {name}: crimp height {:.0} µm, crimp {:.2}% (Peirce {:.2}%), nearest a crossing axis {:.4} of {:.4} mm", 1000.0 * h, 100.0 * c, 100.0 * peirce, nearest, r);
                assert!((c / peirce - 1.0).abs() < 2e-3, "{name}: crimp {c} vs Peirce {peirce}");
                assert!(nearest > r - 1e-5, "{name}: comes within {nearest} of an axis (threads touch at {r})");
            }
        }
    }

    /// Peirce's geometry, from a canvas's thread counts and its measured
    /// crimps alone, gives the threads' heights together (D) and the cloth's
    /// thickness as measured on the same canvases (the threads' heights
    /// sharing D as theirs do), within 25% and 35%:
    /// eight 19th-century linen painting canvases and three modern artists'
    /// linens. The eight (Iaccarino Idelson et al. 2025, Tables 2–4, 7):
    /// yarn heights measured on yarns drawn out of the cloth, thickness by
    /// gauge; two more, whose drawn yarns stand over twice as thick as their
    /// cloth (Dehodencq 0.70 mm in 0.32, Auguin 0.53 in 0.22: yarns that
    /// sprang back once drawn), can't be compared and are left out. The
    /// three modern linens (Chiriboga 2013, Tables 3.1–3.3, 4.1): sections
    /// cut through the cloth, thickness by caliper. The weave laid takes the
    /// eight canvases' mean crimps (`WARP_CRIMP`, `WEFT_CRIMP`).
    #[test]
    fn peirce_matches_the_measured_canvases() {
        // name, warp and weft /cm, crimp warp and weft %, warp and weft yarn heights and cloth thickness, mm
        let canvases = [
            ("Postiglione 1845", 8.0, 8.0, 5.16, 3.27, 0.38, 0.37, 0.66),
            ("Gélibert 1881", 24.0, 18.7, 11.92, 3.56, 0.20, 0.23, 0.45),
            ("Alleaume 1887", 24.0, 21.7, 11.36, 6.76, 0.16, 0.17, 0.44),
            ("Sauzeau 1893", 22.0, 20.0, 13.65, 5.73, 0.23, 0.23, 0.37),
            ("Müller, late 19th c.", 31.3, 29.0, 10.26, 5.70, 0.14, 0.14, 0.26),
            ("Furcy de Lavault, late 19th c.", 23.5, 22.3, 14.21, 3.03, 0.17, 0.22, 0.38),
            ("Rijn", 17.0, 14.5, 19.60, 0.92, 0.196, 0.250, 0.562),
            ("Waal", 23.0, 23.0, 18.88, 1.68, 0.115, 0.167, 0.396),
            ("Amazone", 20.0, 16.0, 15.04, 0.96, 0.167, 0.262, 0.369),
        ];
        for (name, warp, weft, c1, c2, g1, g2, thick) in canvases {
            let (s_wa, s_we) = (10.0 / warp, 10.0 / weft);
            let (d, h1) = peirce_weave(s_wa, s_we, c1 / 100.0, c2 / 100.0);
            let h2 = d - h1;
            let g = g1 + g2;
            // the canvas's own share of the heights
            let gw = d * g1 / g;
            let t = (h1 + gw).max(h2 + d - gw);
            eprintln!("{name} ({warp} x {weft}/cm, crimp {c1}% / {c2}%): D {d:.3} mm (yarns {g}), thickness {t:.3} mm (measured {thick})");
            assert!((d / g - 1.0).abs() < 0.25, "{name}: D {d} vs {g}");
            assert!((t / thick - 1.0).abs() < 0.35, "{name}: thickness {t} vs {thick}");
        }
    }

    /// The largest jumps between points `delta` mm apart along lines across,
    /// down and slanting over the weave `w`: where the same thread is on top
    /// at both, where the top passes from one thread to another away from
    /// the higher one's edge (a crease), and at the higher thread's edge
    /// (and how many of those jump more than a slope of `steep` allows):
    /// (same, crease, edge, edges), µm.
    fn jumps(w: &Weave, delta: f32, steep: f32) -> (f32, f32, f32, usize) {
        let (mut same, mut crease, mut cliff, mut cliffs) = (0.0f32, 0.0f32, 0.0f32, 0);
        let steps = (5.0 * w.s_wa.max(w.s_we) / delta) as usize;
        for line in 0..60 {
            let (x0, y0) = (1.3 + 0.37 * line as f32, 2.1 + 0.29 * line as f32);
            let (dx, dy) = [(1.0, 0.0), (0.0, 1.0), (0.8, 0.6)][line % 3];
            let mut prev = w.height_detail(x0, y0, None);
            for k in 1..steps {
                let s = k as f32 * delta;
                let cur = w.height_detail(x0 + s * dx, y0 + s * dy, None);
                let jump = (cur.0 - prev.0).abs();
                match (prev.1, cur.1) {
                    (Some(a), Some(b)) if a.warp == b.warp && a.index == b.index => same = same.max(jump),
                    (a, b) => {
                        // the higher side's thread, if it is at its edge
                        let hi = if cur.0 > prev.0 { b } else { a };
                        if hi.is_some_and(|t| t.edge < 3.0 * delta) {
                            cliff = cliff.max(jump);
                            cliffs += (jump > steep * 1000.0 * delta) as usize;
                        } else {
                            crease = crease.max(jump);
                        }
                    }
                }
                prev = cur;
            }
        }
        (same, crease, cliff, cliffs)
    }

    /// The relief has no steps but where geometry puts them. Read along
    /// lines over the weave (every thread wandering, thick and thin, with
    /// slubs) at a two-thousandth of a thread and eight times finer: where
    /// the same thread is on top at both points, and where the top passes
    /// from one thread to another in contact with it (a crease), the height
    /// moves less than a slope of 4 allows, and eight times less at eight
    /// times finer: the surface is continuous. It jumps only at a thread's
    /// edge, where the thread lies over a lower one or a hole, and then by
    /// no more than the thread's axis can stand above the floor: at most
    /// its crimp height over the cloth's middle. (The old weave stepped by
    /// 0.55 of the weft's height where a weft passed from under one warp to
    /// over the next, between two points of the same thread.)
    #[test]
    fn crimped_weave_is_continuous() {
        let w = Weave::new(&linen(16.0, 15.0));
        let steep = 4.0;
        let edge_max = 1000.0 * (0.5 * w.f1.max(1.0 - w.f1) * (w.g1 * WARP_THICK.1 + w.g2 * Weave::weft_thick(&w.linen).1) - w.floor);
        let delta = w.s_wa.min(w.s_we) / 2000.0;
        let (same, crease, cliff, cliffs) = jumps(&w, delta, steep);
        let (same8, crease8, _, _) = jumps(&w, delta / 8.0, steep);
        let bound = steep * 1000.0 * delta;
        eprintln!("at {:.3} µm a step: within a thread {same:.3} µm, across a crease {crease:.3} µm (bound {bound:.3}); at an eighth of it {same8:.4} and {crease8:.4} µm; {cliffs} edges over a lower thread, the highest {cliff:.0} µm (at most {edge_max:.0})", 1000.0 * delta);
        assert!(same < bound && crease < bound, "the relief jumps {same} µm (a thread), {crease} µm (a crease) in {delta} mm");
        assert!(same8 <= 1.5 * same / 8.0 && crease8 <= 1.5 * crease / 8.0, "an eighth of the step: {same8}, {crease8} µm");
        assert!(cliffs > 0 && cliff <= edge_max, "{cliffs} edges, highest {cliff} µm");
    }

    /// Reading the crossings from a table gives the same weave, bit for bit.
    #[test]
    fn a_crossing_table_reads_the_same_weave() {
        let w = Weave::new(&linen(14.0, 12.0));
        let t = CrossTable::new(&w, (0.0, 0.0), (8.0, 8.0));
        for k in 0..20000 {
            let (x, y) = ((k % 200) as f32 * 0.041, (k / 200) as f32 * 0.039);
            assert_eq!(w.height_um(x, y, Some(&t)).to_bits(), w.height_um(x, y, None).to_bits(), "({x}, {y})");
        }
        // and past its edge, where it reads them afresh
        assert_eq!(w.height_um(30.0, 1.0, Some(&t)).to_bits(), w.height_um(30.0, 1.0, None).to_bits());
    }

    /// A lens as wide as it is tall is a circle (Peirce's round thread).
    #[test]
    fn a_square_lens_is_a_circle() {
        for k in 0..100 {
            let d = k as f32 / 100.0 * 0.5;
            assert!((lens(d, 1.0, 1.0) - (0.25 - d * d).max(0.0).sqrt()).abs() < 1e-6);
        }
        assert!((lens(0.0, 0.6, 0.2) - 0.1).abs() < 1e-7);
        assert_eq!(lens(0.3, 0.6, 0.2), 0.0);
    }
}
