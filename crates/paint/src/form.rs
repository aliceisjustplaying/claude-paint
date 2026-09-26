//! Depth and lighting fields for caller-supplied solids: for every pixel,
//! which solid is nearest, which plane of it, how that plane is turned to a
//! light and how it is lit.
//!
//! - `Form` is a depth buffer over the canvas: for every pixel the nearest
//!   solid, its distance toward the viewer `z` (units), its surface normal, a
//!   part id (which solid) and a facet id (which plane of it), and its
//!   distance from the viewer for aerial perspective.
//! - Solids are either 3-D bodies (`Sdf`: ellipsoids, blocks, planes cut
//!   through them, noise displacement) seen straight on, or reliefs over the
//!   canvas (`Relief`: any height function).
//! - `Form::light` casts shadows over it; then `shade` says, per point, how far
//!   the plane turns toward the light, how much direct light it gets, whether
//!   it lies in a cast shadow, how much reflected light and sky it sees.
//! - Direction fields: `fall` (gravity projected onto the plane),
//!   `across` (perpendicular to it), `edge_angle`; masks for parts, facets,
//!   lit planes, shadows, silhouettes with a per-point edge width, and the
//!   hard edges where planes break or one solid overlaps another.
//!
//! None of this paints anything; it computes fields and masks. Coordinates:
//! x right, y down (canvas units), z toward the viewer. Normals are unit
//! vectors in that frame.

use crate::canvas::Frame;
use crate::mask::Mask;
use noise::{NoiseFn, Perlin};
use rayon::prelude::*;

pub type V3 = [f32; 3];

#[inline]
fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline]
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline]
fn len(a: V3) -> f32 {
    dot(a, a).sqrt()
}
/// Unit vector along `a`.
#[inline]
pub fn unit(a: V3) -> V3 {
    let l = len(a).max(1e-12);
    [a[0] / l, a[1] / l, a[2] / l]
}
#[inline]
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

// ------------------------------------------------------------------ light

/// One directional light plus what fills the shadows: ambient sky light and
/// light reflected from lit surroundings.
#[derive(Clone, Copy, Debug)]
pub struct Light {
    /// Unit vector toward the light (x right, y down, z toward the viewer).
    pub dir: V3,
    /// Light in full shadow from the open sky (0..1).
    pub ambient: f32,
    /// Strength of the reflected light (0..1) and where it comes from.
    pub bounce: f32,
    pub bounce_dir: V3,
    /// Shadow penumbra: how fast shadow edges soften with distance from the
    /// occluder (0.02 narrow .. 0.3 wide).
    pub penumbra: f32,
    /// How far (units) shadows are traced.
    pub reach: f32,
    /// Occluders more than this far (units of z) in front of a shadow ray
    /// are taken to be in front of it, not blocking it.
    pub thickness: f32,
    /// Whether one part casts shadows on another (false: each part is
    /// shadowed only by itself).
    pub across_parts: bool,
}

impl Light {
    /// Light coming from `from` in the picture ((-1, -0.5): from the left, a
    /// little above), with `front` its component toward the viewer: 0 rakes
    /// across the picture plane, 1 comes from behind the viewer, negative
    /// comes from behind the solids, toward the viewer. Reflected light
    /// comes from below and the far side.
    pub fn new(from: (f32, f32), front: f32) -> Self {
        let dir = unit([from.0, from.1, front]);
        Light {
            dir,
            ambient: 0.15,
            bounce: 0.25,
            bounce_dir: unit([-dir[0] * 0.7, 0.8, 0.3]),
            penumbra: 0.06,
            reach: 400.0,
            thickness: 1e9,
            across_parts: true,
        }
    }
    pub fn ambient(mut self, a: f32) -> Self {
        self.ambient = a;
        self
    }
    /// Reflected light: strength and the direction it comes from.
    pub fn bounce(mut self, amount: f32, from: V3) -> Self {
        self.bounce = amount;
        self.bounce_dir = unit(from);
        self
    }
    pub fn penumbra(mut self, p: f32) -> Self {
        self.penumbra = p;
        self
    }
    pub fn reach(mut self, r: f32) -> Self {
        self.reach = r;
        self
    }
    pub fn thickness(mut self, t: f32) -> Self {
        self.thickness = t;
        self
    }
    pub fn across_parts(mut self, on: bool) -> Self {
        self.across_parts = on;
        self
    }

    /// How a plane with normal `n` is lit, given how deep it lies in a cast
    /// shadow (`cast` 0..1).
    pub fn shade(&self, n: V3, cast: f32) -> Shade {
        let turn = dot(n, self.dir);
        let direct = turn.max(0.0) * (1.0 - cast);
        // reflected light shows where the direct light doesn't reach
        let bounce = dot(n, self.bounce_dir).max(0.0) * (1.0 - direct.min(1.0));
        let sky = (0.5 - 0.5 * n[1]).clamp(0.0, 1.0);
        let value = (self.ambient * sky + (1.0 - self.ambient) * direct + self.bounce * bounce).clamp(0.0, 1.0);
        Shade { turn, direct, cast, bounce, sky, value }
    }
}

/// How one point of a solid is lit.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Shade {
    /// n·L, -1..1: how far the plane turns toward the light (0 at the
    /// terminator; just past it, where neither light nor reflection reach,
    /// is the core shadow).
    pub turn: f32,
    /// Direct light received, 0..1 (turn, cut off by cast shadow).
    pub direct: f32,
    /// How deep the point lies in a cast shadow, 0..1.
    pub cast: f32,
    /// Reflected light received in shadow, 0..1.
    pub bounce: f32,
    /// How much of the sky the plane sees (1 facing up, 0 facing down).
    pub sky: f32,
    /// All of it as one value, 0..1: ambient·sky + direct + reflected.
    pub value: f32,
}

impl Shade {
    /// Membership of the light family (1) versus the shadow family (0), with
    /// a soft halftone of width `soft` around the terminator.
    pub fn lit(&self, soft: f32) -> f32 {
        crate::smoothstep(-soft * 0.5, soft * 0.5, self.direct - soft * 0.5)
    }
}

/// Aerial perspective: the fraction of the color replaced by the air's
/// color at `dist` (in the same units as `visibility`, the distance at which
/// 63 % of it is gone).
pub fn aerial(dist: f32, visibility: f32) -> f32 {
    1.0 - (-dist.max(0.0) / visibility.max(1e-6)).exp()
}

// ----------------------------------------------------------------- solids

/// Where a solid meets a line of sight.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    /// Distance toward the viewer (units).
    pub z: f32,
    /// Unit surface normal.
    pub n: V3,
    /// Which plane of the solid (the solid's own numbering).
    pub facet: u16,
}

/// Something that can be seen: for a point of the canvas, where (if at all)
/// the line of sight meets it.
pub trait Solid: Sync {
    /// Canvas area it may cover: [x0, y0, x1, y1] in units.
    fn bounds(&self) -> [f32; 4];
    fn hit(&self, x: f32, y: f32) -> Option<Hit>;
}

/// A 3-D body as a signed distance function (negative inside), in canvas
/// units with z toward the viewer. Built from a primitive (ellipsoid or
/// block), turned, cut by planes and displaced by noise.
#[derive(Clone)]
pub enum Sdf {
    Ellipsoid { c: V3, r: V3 },
    Block { c: V3, half: V3, round: f32 },
    /// Half-space n·(p - at) <= 0.
    Plane { at: V3, n: V3 },
    Union(Vec<Sdf>, f32),
    Inter(Vec<Sdf>, f32),
    Subtract(Box<Sdf>, Box<Sdf>, f32),
    Turn { body: Box<Sdf>, c: V3, m: [V3; 3] },
    Rough { body: Box<Sdf>, amp: f32, period: f32, noise: Box<Perlin>, ridged: bool },
    Facet(Box<Sdf>, u16),
}

fn smin(a: f32, b: f32, k: f32) -> (f32, f32) {
    // polynomial smooth min; returns value and blend weight toward b
    if k <= 0.0 {
        return if a < b { (a, 0.0) } else { (b, 1.0) };
    }
    let h = (0.5 + 0.5 * (a - b) / k).clamp(0.0, 1.0);
    (b + (a - b) * (1.0 - h) - k * h * (1.0 - h), h)
}

impl Sdf {
    /// An ellipsoid centered at `c` (units; y down, z toward the viewer)
    /// with radii `r`: half extents, so it spans `c ± r` on each axis (unlike
    /// `block`, whose `size` is the whole extent).
    pub fn ellipsoid(c: V3, r: V3) -> Sdf {
        Sdf::Ellipsoid { c, r }
    }
    /// A box centered at `c` (units; y down, z toward the viewer). `size` is
    /// the **whole** extent, width, height and depth, not half of it: it
    /// spans `c ± size / 2`, so its top face is at `y = c[1] - size[1] / 2`
    /// and its foot at `c[1] + size[1] / 2` (to sit a block on ground at
    /// `y = g`, put `c[1] = g - size[1] / 2`, or lower to sink it in).
    /// `round` (units) rounds its edges and corners; at most half the
    /// smallest size, and it rounds inside the extent without growing it.
    /// Its faces are facets 1 right, 2 left, 3 bottom, 4 top, 5 front, 6 back
    /// (in the block's own frame, so they follow it when turned).
    ///
    /// ```ignore
    /// // a block 120 wide and 80 tall with its foot at y = 600
    /// let b = Sdf::block([400.0, 600.0 - 40.0, 0.0], [120.0, 80.0, 90.0], 12.0);
    /// ```
    pub fn block(c: V3, size: V3, round: f32) -> Sdf {
        Sdf::Block { c, half: [size[0] * 0.5, size[1] * 0.5, size[2] * 0.5], round }
    }
    /// Everything behind the plane through the point `at` (units) whose
    /// outward normal is `n` (any length; it is normalized).
    pub fn half_space(at: V3, n: V3) -> Sdf {
        Sdf::Plane { at, n: unit(n) }
    }
    /// Split off everything beyond the plane through the point `at` (units)
    /// facing `n`; the new face becomes facet `facet`. `round` (units)
    /// rounds the new edges (0 = sharp).
    pub fn cut(self, at: V3, n: V3, facet: u16, round: f32) -> Sdf {
        Sdf::Inter(vec![self, Sdf::Facet(Box::new(Sdf::half_space(at, n)), facet)], round)
    }
    /// Both bodies as one. `smooth` (units) fuses them with a fillet about
    /// that wide where they meet (0 = a sharp crease).
    pub fn union(self, o: Sdf, smooth: f32) -> Sdf {
        Sdf::Union(vec![self, o], smooth)
    }
    /// This body with `o` carved out of it; `smooth` (units) rounds the
    /// edge of the hollow (0 = sharp).
    pub fn subtract(self, o: Sdf, smooth: f32) -> Sdf {
        Sdf::Subtract(Box::new(self), Box::new(o), smooth)
    }
    /// Rotate about the point `c` (units; usually the body's center): `yaw` turns it about the vertical (positive turns
    /// its right side toward the viewer), `pitch` tips its top toward the
    /// viewer (so we look down on it), `roll` leans it in the picture plane
    /// (positive clockwise). Radians.
    pub fn turn(self, c: V3, yaw: f32, pitch: f32, roll: f32) -> Sdf {
        let (sy, cy) = yaw.sin_cos();
        let (sp, cp) = pitch.sin_cos();
        let (sr, cr) = roll.sin_cos();
        // body → view: roll · pitch · yaw (row vectors of the matrix)
        let ry = [[cy, 0.0, -sy], [0.0, 1.0, 0.0], [sy, 0.0, cy]];
        // the top (−y) comes toward the viewer (+z)
        let rp = [[1.0, 0.0, 0.0], [0.0, cp, sp], [0.0, -sp, cp]];
        let rr = [[cr, -sr, 0.0], [sr, cr, 0.0], [0.0, 0.0, 1.0]];
        let mul = |a: [V3; 3], b: [V3; 3]| {
            let mut m = [[0.0f32; 3]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    m[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum();
                }
            }
            m
        };
        let m = mul(rr, mul(rp, ry));
        Sdf::Turn { body: Box::new(self), c, m }
    }
    /// Displace the surface in and out by about `amp` units of 3-D fractal
    /// noise whose largest features are `period` units across. `ridged`
    /// sums `0.5 − |n|` per octave instead of `n`, which gives sharp creases
    /// along the noise's zero set instead of rounded lumps.
    pub fn rough(self, amp: f32, period: f32, seed: u32, ridged: bool) -> Sdf {
        Sdf::Rough { body: Box::new(self), amp, period, noise: Box::new(Perlin::new(seed)), ridged }
    }
    /// Tag the whole body as facet `id`.
    pub fn facet(self, id: u16) -> Sdf {
        Sdf::Facet(Box::new(self), id)
    }

    /// Signed distance at `p` and the facet id of the nearest surface.
    pub fn eval(&self, p: V3) -> (f32, u16) {
        match self {
            Sdf::Ellipsoid { c, r } => {
                let q = sub(p, *c);
                let k0 = len([q[0] / r[0], q[1] / r[1], q[2] / r[2]]);
                let k1 = len([q[0] / (r[0] * r[0]), q[1] / (r[1] * r[1]), q[2] / (r[2] * r[2])]);
                (if k1 > 1e-9 { k0 * (k0 - 1.0) / k1 } else { -r[0].min(r[1]).min(r[2]) }, 0)
            }
            Sdf::Block { c, half, round } => {
                let q = sub(p, *c);
                let r = round.min(half[0]).min(half[1]).min(half[2]);
                let d = [q[0].abs() - half[0] + r, q[1].abs() - half[1] + r, q[2].abs() - half[2] + r];
                let o = len([d[0].max(0.0), d[1].max(0.0), d[2].max(0.0)]);
                // which face: the axis the point lies furthest out along
                let ax = if d[0] >= d[1] && d[0] >= d[2] { 0 } else if d[1] >= d[2] { 1 } else { 2 };
                let facet = 1 + 2 * ax as u16 + (q[ax] < 0.0) as u16;
                (o + d[0].max(d[1]).max(d[2]).min(0.0) - r, facet)
            }
            Sdf::Plane { at, n } => (dot(sub(p, *at), *n), 0),
            Sdf::Union(v, k) => {
                let mut acc = v[0].eval(p);
                for s in &v[1..] {
                    let e = s.eval(p);
                    let (d, h) = smin(acc.0, e.0, *k);
                    acc = (d, if h > 0.5 { e.1 } else { acc.1 });
                }
                acc
            }
            Sdf::Inter(v, k) => {
                let mut acc = v[0].eval(p);
                for s in &v[1..] {
                    let e = s.eval(p);
                    let (d, h) = smin(-acc.0, -e.0, *k);
                    acc = (-d, if h > 0.5 { e.1 } else { acc.1 });
                }
                acc
            }
            Sdf::Subtract(a, b, k) => {
                let ea = a.eval(p);
                let eb = b.eval(p);
                let (d, h) = smin(-ea.0, eb.0, *k);
                (-d, if h > 0.5 { eb.1 } else { ea.1 })
            }
            Sdf::Turn { body, c, m } => {
                // view → body is the transpose
                let q = sub(p, *c);
                let b = [m[0][0] * q[0] + m[1][0] * q[1] + m[2][0] * q[2], m[0][1] * q[0] + m[1][1] * q[1] + m[2][1] * q[2], m[0][2] * q[0] + m[1][2] * q[1] + m[2][2] * q[2]];
                body.eval([b[0] + c[0], b[1] + c[1], b[2] + c[2]])
            }
            Sdf::Rough { body, amp, period, noise, ridged } => {
                let (d, id) = body.eval(p);
                // far from the surface the displacement can't matter
                if d > amp * 2.0 {
                    return (d - amp, id);
                }
                let s = 1.0 / *period as f64;
                let (mut n, mut a, mut f) = (0.0f64, 1.0f64, 1.0f64);
                for o in 0..4 {
                    let v = noise.get([p[0] as f64 * s * f, p[1] as f64 * s * f + 31.7 * o as f64, p[2] as f64 * s * f]);
                    n += a * if *ridged { 0.5 - v.abs() } else { v };
                    a *= 0.5;
                    f *= 2.03;
                }
                (d - *amp * n as f32, id)
            }
            Sdf::Facet(b, id) => (b.eval(p).0, *id),
        }
    }

    /// Axis-aligned bounds [min, max] (generous).
    pub fn aabb(&self) -> (V3, V3) {
        match self {
            Sdf::Ellipsoid { c, r } => (sub(*c, *r), [c[0] + r[0], c[1] + r[1], c[2] + r[2]]),
            Sdf::Block { c, half, .. } => (sub(*c, *half), [c[0] + half[0], c[1] + half[1], c[2] + half[2]]),
            Sdf::Plane { .. } => ([-1e6; 3], [1e6; 3]),
            Sdf::Union(v, k) => {
                let mut b = v[0].aabb();
                for s in &v[1..] {
                    let o = s.aabb();
                    for i in 0..3 {
                        b.0[i] = b.0[i].min(o.0[i]) - k;
                        b.1[i] = b.1[i].max(o.1[i]) + k;
                    }
                }
                b
            }
            Sdf::Inter(v, _) => {
                let mut b = v[0].aabb();
                for s in &v[1..] {
                    let o = s.aabb();
                    for i in 0..3 {
                        b.0[i] = b.0[i].max(o.0[i]);
                        b.1[i] = b.1[i].min(o.1[i]);
                    }
                }
                b
            }
            Sdf::Subtract(a, _, _) => a.aabb(),
            Sdf::Turn { body, c, m } => {
                let (lo, hi) = body.aabb();
                if lo.iter().chain(hi.iter()).any(|v| v.abs() >= 1e6) {
                    // unbounded (a bare half-space): stays unbounded
                    return ([-1e6; 3], [1e6; 3]);
                }
                // the body's box turned: bound all eight of its corners
                let (mut a, mut b) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
                for k in 0..8 {
                    let q = sub([if k & 1 == 0 { lo[0] } else { hi[0] }, if k & 2 == 0 { lo[1] } else { hi[1] }, if k & 4 == 0 { lo[2] } else { hi[2] }], *c);
                    for i in 0..3 {
                        let v = c[i] + dot(m[i], q);
                        a[i] = a[i].min(v);
                        b[i] = b[i].max(v);
                    }
                }
                (a, b)
            }
            Sdf::Rough { body, amp, .. } => {
                let (lo, hi) = body.aabb();
                let a = amp * 1.5;
                ([lo[0] - a, lo[1] - a, lo[2] - a], [hi[0] + a, hi[1] + a, hi[2] + a])
            }
            Sdf::Facet(b, _) => b.aabb(),
        }
    }

    /// Unit normal (gradient of the distance) at `p`.
    pub fn normal(&self, p: V3, h: f32) -> V3 {
        // tetrahedron of samples
        let k = [[1.0, -1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, 1.0, -1.0], [1.0, 1.0, 1.0f32]];
        let mut g = [0.0f32; 3];
        for e in k {
            let d = self.eval([p[0] + e[0] * h, p[1] + e[1] * h, p[2] + e[2] * h]).0;
            for i in 0..3 {
                g[i] += e[i] * d;
            }
        }
        unit(g)
    }
}

impl Solid for Sdf {
    fn bounds(&self) -> [f32; 4] {
        let (lo, hi) = self.aabb();
        [lo[0], lo[1], hi[0], hi[1]]
    }
    /// Along the line of sight (−z): exact for a bare ellipsoid, otherwise
    /// sphere tracing. Where tracing crawls (a thin body seen near its edge,
    /// where the distance estimate is poor), it goes on in steps of at
    /// least `MIN_STEP` units and bisects the crossing, so running out of
    /// steps is never taken for a miss; only passing the back of the bounds
    /// is.
    fn hit(&self, x: f32, y: f32) -> Option<Hit> {
        if let Sdf::Ellipsoid { c, r } = self {
            return ellipsoid_hit(*c, *r, x, y);
        }
        const MIN_STEP: f32 = 0.25;
        let (lo, hi) = self.aabb();
        let (z_front, z_back) = (hi[2] + 1.0, lo[2] - 1.0);
        let eps = 0.02;
        let found = |z: f32| {
            let p = [x, y, z];
            let (_, facet) = self.eval(p);
            Some(Hit { z, n: self.normal(p, 0.08), facet })
        };
        let (mut z, mut prev) = (z_front, z_front);
        let mut steps = 0u32;
        loop {
            let (d, _) = self.eval([x, y, z]);
            if d < eps && d >= 0.0 {
                // refine along the ray (Newton on the distance): a distance
                // bound far from Euclidean (a thin ellipsoid) stops short
                let (mut zr, mut dr) = (z, d);
                for _ in 0..4 {
                    if dr.abs() < 1e-4 {
                        break;
                    }
                    let h = 0.05;
                    let g = (self.eval([x, y, zr + h]).0 - self.eval([x, y, zr - h]).0) / (2.0 * h);
                    if g < 1e-4 {
                        break;
                    }
                    let zn = zr - (dr / g).clamp(-4.0, 4.0);
                    let dn = self.eval([x, y, zn]).0;
                    if dn.abs() >= dr.abs() {
                        break;
                    }
                    (zr, dr) = (zn, dn);
                }
                return found(zr);
            }
            if d < 0.0 {
                // stepped through the surface: bisect back to it
                let (mut a, mut b) = (prev, z);
                for _ in 0..24 {
                    let m = 0.5 * (a + b);
                    if self.eval([x, y, m]).0 < 0.0 { b = m } else { a = m }
                }
                return found(0.5 * (a + b));
            }
            steps += 1;
            // the distance bound first; after 160 steps it is crawling, so
            // guard the step
            let min = if steps < 160 { eps * 0.5 } else { MIN_STEP };
            prev = z;
            z -= (d * 0.8).max(min);
            if z < z_back {
                return None;
            }
        }
    }
}

/// The front of an axis-aligned ellipsoid on the line of sight through
/// (x, y), in closed form.
fn ellipsoid_hit(c: V3, r: V3, x: f32, y: f32) -> Option<Hit> {
    let (u, v) = ((x - c[0]) as f64 / r[0] as f64, (y - c[1]) as f64 / r[1] as f64);
    let e = 1.0 - u * u - v * v;
    if e < 0.0 {
        return None;
    }
    let z = c[2] + (r[2] as f64 * e.sqrt()) as f32;
    let q = [x - c[0], y - c[1], z - c[2]];
    Some(Hit { z, n: unit([q[0] / (r[0] * r[0]), q[1] / (r[1] * r[1]), q[2] / (r[2] * r[2])]), facet: 0 })
}

/// Any relief over the canvas: `f(x, y)` (canvas units) gives the height
/// toward the viewer (z, units) and a facet id, or None where there is
/// nothing. `area` is [x0, y0, x1, y1] in units: the only part of the
/// canvas where `f` is asked. Normals come from its
/// slopes (central differences, 0.3 units).
pub struct Relief<F: Fn(f32, f32) -> Option<(f32, u16)> + Sync> {
    pub area: [f32; 4],
    pub f: F,
}

impl<F: Fn(f32, f32) -> Option<(f32, u16)> + Sync> Relief<F> {
    pub fn new(area: [f32; 4], f: F) -> Self {
        Relief { area, f }
    }
}

fn relief_hit(f: &dyn Fn(f32, f32) -> Option<(f32, u16)>, x: f32, y: f32) -> Option<Hit> {
    let (z, facet) = f(x, y)?;
    let e = 0.3;
    let zs = |xx: f32, yy: f32| f(xx, yy).map(|v| v.0);
    let dx = match (zs(x + e, y), zs(x - e, y)) {
        (Some(a), Some(b)) => (a - b) / (2.0 * e),
        (Some(a), None) => (a - z) / e,
        (None, Some(b)) => (z - b) / e,
        _ => 0.0,
    };
    let dy = match (zs(x, y + e), zs(x, y - e)) {
        (Some(a), Some(b)) => (a - b) / (2.0 * e),
        (Some(a), None) => (a - z) / e,
        (None, Some(b)) => (z - b) / e,
        _ => 0.0,
    };
    Some(Hit { z, n: unit([-dx, -dy, 1.0]), facet })
}

impl<F: Fn(f32, f32) -> Option<(f32, u16)> + Sync> Solid for Relief<F> {
    fn bounds(&self) -> [f32; 4] {
        self.area
    }
    fn hit(&self, x: f32, y: f32) -> Option<Hit> {
        relief_hit(&self.f, x, y)
    }
}

// ------------------------------------------------------------------- form

/// A part of the form, as added.
pub type PartId = u16;

/// Everything known at one point of the form.
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub part: PartId,
    pub facet: u16,
    pub z: f32,
    pub n: V3,
    /// Distance from the viewer (for aerial perspective).
    pub dist: f32,
    /// Lighting (all zero until `Form::light` is called).
    pub shade: Shade,
}

impl Sample {
    /// The fall line through this point, as a canvas angle (radians,
    /// 0 = left→right, y down): gravity projected onto the plane.
    pub fn fall(&self) -> f32 {
        fall_angle(self.n)
    }
    /// Perpendicular to the fall line.
    pub fn across(&self) -> f32 {
        fall_angle(self.n) - std::f32::consts::FRAC_PI_2
    }
}

/// Canvas angle of gravity projected onto a plane with normal `n`.
pub fn fall_angle(n: V3) -> f32 {
    let (dx, dy) = (-n[1] * n[0], 1.0 - n[1] * n[1]);
    if dx.abs() + dy.abs() < 1e-5 { std::f32::consts::FRAC_PI_2 } else { dy.atan2(dx) }
}

/// The depth buffer of solids over the canvas (see the module docs).
pub struct Form {
    pub f: Frame,
    z: Vec<f32>,
    n: Vec<V3>,
    part: Vec<PartId>,
    facet: Vec<u16>,
    dist: Vec<f32>,
    cast: Vec<f32>,
    light: Option<Light>,
    parts: PartId,
}

impl Form {
    /// An empty form over the whole canvas: pass `c.frame()` (not
    /// `c.window()`, even in a crop render). All positions and sizes given to
    /// solids are canvas units, like masks.
    pub fn new(f: Frame) -> Self {
        let n = f.w * f.h;
        Form {
            f,
            z: vec![f32::NEG_INFINITY; n],
            n: vec![[0.0, 0.0, 1.0]; n],
            part: vec![0; n],
            facet: vec![0; n],
            dist: vec![0.0; n],
            cast: vec![0.0; n],
            light: None,
            parts: 0,
        }
    }

    /// Add a solid at distance `dist` from the viewer (any unit, used only
    /// for aerial perspective). Where it is nearer than what
    /// is already there, it hides it. Returns its part id (1, 2, …).
    pub fn add(&mut self, s: &dyn Solid, dist: f32) -> PartId {
        self.add_at(s, &|_, _, _| dist)
    }

    /// Like `add`, with the distance varying over the solid:
    /// `dist(x, y, z)` (e.g. a plane receding from the viewer).
    pub fn add_at(&mut self, s: &dyn Solid, dist: &(dyn Fn(f32, f32, f32) -> f32 + Sync)) -> PartId {
        self.add_by(s, dist, false)
    }

    /// Like `add_at`, but what hides what is decided by `dist` (nearer
    /// wins), not by the solid's own `z`: for solids each drawn in its own
    /// projection (a scene's bodies), whose `z` values do not compare.
    pub fn add_nearest(&mut self, s: &dyn Solid, dist: &(dyn Fn(f32, f32, f32) -> f32 + Sync)) -> PartId {
        self.add_by(s, dist, true)
    }

    fn add_by(&mut self, s: &dyn Solid, dist: &(dyn Fn(f32, f32, f32) -> f32 + Sync), by_dist: bool) -> PartId {
        self.parts += 1;
        let id = self.parts;
        let f = self.f;
        let b = s.bounds();
        let px0 = ((b[0] * f.scale).floor().max(0.0) as usize).min(f.w);
        let py0 = ((b[1] * f.scale).floor().max(0.0) as usize).min(f.h);
        let px1 = ((b[2] * f.scale).ceil().max(0.0) as usize + 1).min(f.w);
        let py1 = ((b[3] * f.scale).ceil().max(0.0) as usize + 1).min(f.h);
        if px1 <= px0 || py1 <= py0 {
            return id;
        }
        let inv = 1.0 / f.scale;
        // row by row, straight into the buffers: no frame-sized scratch
        let rows = py0 * f.w..py1 * f.w;
        let w = f.w;
        self.z[rows.clone()]
            .par_chunks_mut(w)
            .zip(self.n[rows.clone()].par_chunks_mut(w))
            .zip(self.part[rows.clone()].par_chunks_mut(w))
            .zip(self.facet[rows.clone()].par_chunks_mut(w))
            .zip(self.dist[rows].par_chunks_mut(w))
            .enumerate()
            .for_each(|(r, ((((zr, nr), pr), fr), dr))| {
                let y = ((py0 + r) as f32 + 0.5) * inv;
                for px in px0..px1 {
                    let x = (px as f32 + 0.5) * inv;
                    if let Some(h) = s.hit(x, y) {
                        let d = if by_dist { Some(dist(x, y, h.z)) } else { None };
                        let nearer = match d {
                            Some(d) => pr[px] == 0 || d < dr[px],
                            None => h.z > zr[px],
                        };
                        if nearer {
                            zr[px] = h.z;
                            nr[px] = h.n;
                            pr[px] = id;
                            fr[px] = h.facet;
                            dr[px] = d.unwrap_or_else(|| dist(x, y, h.z));
                        }
                    }
                }
            });
        self.light = None;
        id
    }

    /// Light the form: cast shadows are traced over the depth buffer toward
    /// the light (soft with distance from the occluder).
    pub fn light(&mut self, l: Light) {
        let f = self.f;
        let (lx, ly, lz) = (l.dir[0], l.dir[1], l.dir[2]);
        let lxy = (lx * lx + ly * ly).sqrt();
        if lxy < 1e-3 {
            self.cast.iter_mut().for_each(|c| *c = 0.0);
            self.light = Some(l);
            return;
        }
        let (sx, sy) = (lx / lxy, ly / lxy);
        let rise = lz / lxy;
        let zmax = self.z.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let step = 1.0 / f.scale;
        let (z, part) = (&self.z, &self.part);
        let inv = 1.0 / f.scale;
        // in place: no second frame-sized buffer
        self.cast.par_iter_mut().enumerate().for_each(|(i, out)| {
            *out = {
                if part[i] == 0 {
                    0.0
                } else {
                let (x, y) = (((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv);
                let z0 = z[i] + 0.3;
                let mut t = step * 1.5;
                let mut occl = 0.0f32;
                while t < l.reach {
                    let ray = z0 + rise * t;
                    if rise >= 0.0 && ray > zmax {
                        break;
                    }
                    let (qx, qy) = (x + sx * t, y + sy * t);
                    if qx < 0.0 || qy < 0.0 || qx >= f.width() || qy >= f.height() {
                        break;
                    }
                    let j = f.index(qx, qy);
                    if part[j] != 0 && (l.across_parts || part[j] == part[i]) {
                        let hgt = z[j] - ray;
                        if hgt < l.thickness {
                            let o = 0.5 + 0.5 * hgt / (l.penumbra * t + 0.3);
                            occl = occl.max(o);
                            if occl >= 1.0 {
                                break;
                            }
                        }
                    }
                    t += step;
                }
                crate::smoothstep(0.0, 1.0, occl)
                }
            };
        });
        self.light = Some(l);
    }

    /// Light the form with cast shadows the caller has traced instead of
    /// tracing them over the depth buffer: `cast(x, y, &sample)` (0..1) for
    /// every point of a solid. A `scene::World` lights its solids this way,
    /// so the shadows on the solids and on the ground come from one sun.
    pub fn light_given(&mut self, l: Light, cast: impl Fn(f32, f32, &Sample) -> f32 + Sync) {
        let f = self.f;
        let inv = 1.0 / f.scale;
        self.light = None;
        let vals: Vec<f32> = (0..f.w * f.h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = (((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv);
                self.sample_i(i).map_or(0.0, |s| cast(x, y, &s).clamp(0.0, 1.0))
            })
            .collect();
        self.cast = vals;
        self.light = Some(l);
    }

    /// The light the form was last lit with.
    pub fn lighting(&self) -> Option<Light> {
        self.light
    }

    fn sample_i(&self, i: usize) -> Option<Sample> {
        let part = self.part[i];
        if part == 0 {
            return None;
        }
        let n = self.n[i];
        let shade = self.light.map_or(Shade::default(), |l| l.shade(n, self.cast[i]));
        Some(Sample { part, facet: self.facet[i], z: self.z[i], n, dist: self.dist[i], shade })
    }

    /// What is at a point (None where no solid is).
    pub fn sample(&self, x: f32, y: f32) -> Option<Sample> {
        if x < 0.0 || y < 0.0 || x >= self.f.width() || y >= self.f.height() {
            return None;
        }
        self.sample_i(self.f.index(x, y))
    }

    /// Lighting at a point (zero where no solid is).
    pub fn shade(&self, x: f32, y: f32) -> Shade {
        self.sample(x, y).map_or(Shade::default(), |s| s.shade)
    }

    /// Fall-line angle at a point (straight down where no solid is).
    pub fn fall(&self, x: f32, y: f32) -> f32 {
        self.sample(x, y).map_or(std::f32::consts::FRAC_PI_2, |s| s.fall())
    }

    /// Perpendicular to the fall line.
    pub fn across(&self, x: f32, y: f32) -> f32 {
        self.fall(x, y) - std::f32::consts::FRAC_PI_2
    }

    /// Part id at a point (0 = none).
    pub fn part(&self, x: f32, y: f32) -> PartId {
        self.sample(x, y).map_or(0, |s| s.part)
    }

    /// Distance from the viewer at a point (`far` where no solid is).
    pub fn dist(&self, x: f32, y: f32, far: f32) -> f32 {
        self.sample(x, y).map_or(far, |s| s.dist)
    }

    /// A mask from any per-point rule over the form (0 where no solid is):
    /// `form.mask(|s| if s.part == id { s.shade.lit(0.2) } else { 0.0 })`.
    pub fn mask(&self, g: impl Fn(&Sample) -> f32 + Sync) -> Mask {
        let data = (0..self.f.w * self.f.h).into_par_iter().map(|i| self.sample_i(i).map_or(0.0, |s| g(&s))).collect();
        Mask { f: self.f, data }
    }

    /// The silhouette of the given parts, its edge `soft(&sample)` units wide
    /// (evaluated at the nearest point inside), so the width can vary with
    /// the sample's distance, normal or facet.
    pub fn silhouette(&self, parts: &[PartId], soft: impl Fn(&Sample) -> f32 + Sync) -> Mask {
        let inside: Vec<bool> = self.part.par_iter().map(|p| *p != 0 && parts.contains(p)).collect();
        let f = self.f;
        crate::mask::soften_region(f, inside, |x, y| self.sample_i(f.index(x, y)).map_or(0.0, |s| soft(&s)))
    }

    /// Hard edges inside the form: plane breaks (normals turning by more
    /// than `turn` radians within `span` units) and overlaps (depth jumping
    /// by more than `step` units, where one solid or part of one stands in
    /// front of another). 0..1, strongest at the sharpest breaks.
    pub fn edges(&self, turn: f32, step: f32, span: f32) -> Mask {
        let f = self.f;
        let k = (span * f.scale).round().max(1.0) as isize;
        let data = (0..f.w * f.h)
            .into_par_iter()
            .map(|i| {
                if self.part[i] == 0 {
                    return 0.0;
                }
                let (x, y) = ((i % f.w) as isize, (i / f.w) as isize);
                let mut e = 0.0f32;
                for (dx, dy) in [(k, 0), (0, k), (k, k), (k, -k)] {
                    let a = (x - dx, y - dy);
                    let b = (x + dx, y + dy);
                    let ok = |p: (isize, isize)| p.0 >= 0 && p.1 >= 0 && p.0 < f.w as isize && p.1 < f.h as isize;
                    if !ok(a) || !ok(b) {
                        continue;
                    }
                    let (ia, ib) = (a.1 as usize * f.w + a.0 as usize, b.1 as usize * f.w + b.0 as usize);
                    if self.part[ia] == 0 || self.part[ib] == 0 {
                        continue;
                    }
                    if self.part[ia] != self.part[ib] {
                        e = 1.0;
                        continue;
                    }
                    // turning per distance along the surface, so a form that
                    // turns away smoothly at its limb isn't taken for a break
                    let (na, nb) = (self.n[ia], self.n[ib]);
                    let ang = dot(na, nb).clamp(-1.0, 1.0).acos();
                    let (ddx, ddy) = ((2 * dx) as f32 / f.scale, (2 * dy) as f32 / f.scale);
                    let dz = self.z[ib] - self.z[ia];
                    let run = (ddx * ddx + ddy * ddy).sqrt();
                    let surf = (run * run + dz * dz).sqrt();
                    e = e.max(crate::smoothstep(turn * 0.7, turn * 1.3, ang * run / surf.max(1e-6)));
                    // a jump in depth the slopes on either side don't explain:
                    // one surface standing in front of another
                    let slope = |n: V3| -(n[0] * ddx + n[1] * ddy) / n[2].max(0.2);
                    let pred = 0.5 * (slope(na) + slope(nb));
                    e = e.max(crate::smoothstep(step * 0.7, step * 1.3, (dz - pred).abs()));
                }
                e
            })
            .collect();
        Mask { f, data }
    }

    /// How the surface bends at a point, over `span` units: positive where it
    /// is convex (normals diverge), negative where concave (normals
    /// converge). Roughly the turn in radians across the span, whichever of
    /// the two axes turns more.
    pub fn bend(&self, x: f32, y: f32, span: f32) -> f32 {
        let s = |dx: f32, dy: f32| self.sample(x + dx, y + dy);
        let mut best = 0.0f32;
        for (dx, dy) in [(span, 0.0), (0.0, span)] {
            if let (Some(a), Some(b)) = (s(-dx, -dy), s(dx, dy)) {
                if a.part != b.part {
                    continue;
                }
                // normals diverge along a convex surface
                let p = [2.0 * dx, 2.0 * dy, b.z - a.z];
                let k = dot(p, sub(b.n, a.n)) / len(p).max(1e-6);
                if k.abs() > best.abs() {
                    best = k;
                }
            }
        }
        best
    }

    /// Direction along the edge through a point (canvas angle): the line
    /// where the planes on either side meet, or along an overlap. Sample a
    /// span of `span` units around it.
    pub fn edge_angle(&self, x: f32, y: f32, span: f32) -> f32 {
        let s = |dx: f32, dy: f32| self.sample(x + dx, y + dy);
        let (Some(l), Some(r), Some(u), Some(d)) = (s(-span, 0.0), s(span, 0.0), s(0.0, -span), s(0.0, span)) else {
            // at the silhouette: along the outline of the solid
            let has = |dx: f32, dy: f32| -> f32 { if s(dx, dy).is_some() { 1.0 } else { 0.0 } };
            let gx = has(span, 0.0) - has(-span, 0.0);
            let gy = has(0.0, span) - has(0.0, -span);
            return gy.atan2(gx) + std::f32::consts::FRAC_PI_2;
        };
        // the pair across which the normal turns most
        let (a, b) = if dot(l.n, r.n) < dot(u.n, d.n) { (l, r) } else { (u, d) };
        if a.part != b.part || (a.z - b.z).abs() > span * 2.0 {
            // an overlap: along the contour of depth
            let gx = r.z - l.z;
            let gy = d.z - u.z;
            return gy.atan2(gx) + std::f32::consts::FRAC_PI_2;
        }
        let c = cross(a.n, b.n);
        if c[0].abs() + c[1].abs() < 1e-5 {
            return 0.0;
        }
        c[1].atan2(c[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> Frame {
        Frame::new(500, 350, 0.5)
    }

    #[test]
    fn a_ball_lit_from_the_left() {
        let mut form = Form::new(frame());
        let ball = form.add(&Sdf::ellipsoid([500.0, 350.0, 0.0], [150.0, 150.0, 150.0]), 1.0);
        form.light(Light::new((-1.0, -0.4), 0.5));
        let left = form.sample(400.0, 300.0).unwrap();
        let right = form.sample(620.0, 350.0).unwrap();
        assert_eq!(left.part, ball);
        assert!(left.n[0] < -0.5 && right.n[0] > 0.6);
        assert!(left.shade.direct > 0.5 && right.shade.direct == 0.0);
        // the core shadow, just past the terminator, is darker than the
        // reflected light further round
        let core = (500..640).map(|x| form.shade(x as f32, 350.0)).find(|s| s.turn < 0.0).unwrap();
        let far = form.shade(645.0, 380.0);
        assert!(far.value > core.value, "{} vs {}", far.value, core.value);
        assert!(form.sample(500.0, 150.0).is_none());
        // the fall line on the left flank runs down and to the left
        let a = left.fall();
        assert!(a > std::f32::consts::FRAC_PI_2 && a < std::f32::consts::PI, "{a}");
    }

    #[test]
    fn fracture_planes_are_facets_with_hard_edges() {
        let mut form = Form::new(frame());
        let body = Sdf::ellipsoid([500.0, 350.0, 0.0], [200.0, 150.0, 120.0]).cut([500.0, 300.0, 60.0], [-0.3, -0.5, 1.0], 1, 0.0);
        let id = form.add(&body, 1.0);
        let s = form.sample(500.0, 300.0).unwrap();
        assert_eq!((s.part, s.facet), (id, 1));
        let n = unit([-0.3, -0.5, 1.0]);
        assert!(dot(s.n, n) > 0.99);
        let edges = form.edges(0.4, 5.0, 2.0);
        let (mut on, mut total) = (0.0, 0.0);
        for (i, e) in edges.data.iter().enumerate() {
            if form.part[i] != 0 {
                total += 1.0;
                on += e;
            }
        }
        // a thin line, not everywhere
        assert!(on > 20.0 && on < total * 0.2, "{on} of {total}");
    }

    #[test]
    fn a_block_casts_a_shadow_on_the_ground() {
        let mut form = Form::new(frame());
        let ground = Relief::new([0.0, 0.0, 1000.0, 700.0], |_, y| Some((y * 0.05, 0)));
        form.add(&ground, 5.0);
        let block = Sdf::block([400.0, 350.0, 40.0], [100.0, 200.0, 80.0], 4.0);
        form.add(&block, 4.0);
        form.light(Light::new((-1.0, -0.3), 0.3).penumbra(0.02));
        // right of the block: in its shadow; left: in the sun
        assert!(form.shade(480.0, 380.0).cast > 0.9, "{:?}", form.shade(480.0, 380.0));
        assert!(form.shade(320.0, 380.0).cast < 0.1);
        assert!(form.shade(900.0, 380.0).cast < 0.1);
        let sil = form.silhouette(&[2], |_| 0.0);
        assert!(sil.sample(400.0, 350.0) > 0.99 && sil.sample(300.0, 350.0) < 0.01);
    }

    /// Every interior line of sight through a thin ellipsoid (10:1, 20:1)
    /// hits it, at the analytic front surface; turned, it is still seen.
    #[test]
    fn thin_ellipsoids_keep_their_surface() {
        for rx in [10.0f32, 5.0] {
            let c = [500.0, 350.0, 0.0];
            let r = [rx, 100.0, 100.0];
            let bare = Sdf::ellipsoid(c, r);
            // the same body turned by nothing goes through the traced path
            let traced = Sdf::ellipsoid(c, r).turn(c, 0.0, 0.0, 0.0);
            let mut misses = (0, 0);
            for ix in -95..96 {
                for iy in -95..96 {
                    let (x, y) = (ix as f32 * rx / 100.0, iy as f32);
                    let e = (x / rx).powi(2) + (y / 100.0).powi(2);
                    if e >= 0.95 {
                        continue;
                    }
                    let front = 100.0 * (1.0 - e).sqrt();
                    match bare.hit(500.0 + x, 350.0 + y) {
                        Some(h) => assert!((h.z - front).abs() < 0.05, "rx {rx} ({x},{y}): z {} vs {front}", h.z),
                        None => misses.0 += 1,
                    }
                    match traced.hit(500.0 + x, 350.0 + y) {
                        Some(h) => assert!((h.z - front).abs() < 0.1, "rx {rx} ({x},{y}) traced: z {} vs {front}", h.z),
                        None => misses.1 += 1,
                    }
                }
            }
            assert_eq!(misses, (0, 0), "rx {rx}: misses (analytic, traced)");
        }
        // a pixel near the thin edge: the analytic front is z = 23.108446
        let h = Sdf::ellipsoid([500.0, 350.0, 0.0], [5.0, 100.0, 100.0]).hit(495.25, 329.0).unwrap();
        assert!((h.z - 23.108446).abs() < 1e-3, "{}", h.z);
        assert!(h.n[0] < -0.9, "{:?}", h.n);
    }

    /// Turned about a pivot outside the body, the solid stays inside its
    /// bounds, so `Form::add` doesn't clip it.
    #[test]
    fn turning_about_an_outside_pivot_keeps_the_body_in_bounds() {
        let s = Sdf::block([500.0, 350.0, 0.0], [20.0; 3], 0.0).turn([600.0, 350.0, -100.0], std::f32::consts::FRAC_PI_4, 0.0, 0.0);
        let b = s.bounds();
        let direct = s.hit(450.5, 350.5).expect("the turned block is there");
        assert!(b[0] <= 450.0 && b[2] >= 451.0, "bounds {b:?}");
        let mut form = Form::new(Frame::new(1000, 700, 1.0));
        let id = form.add(&s, 1.0);
        let got = form.sample(450.5, 350.5).expect("clipped by the bounds");
        assert_eq!(got.part, id);
        assert!((got.z - direct.z).abs() < 1e-3);
        // every pixel the solid covers is in the form
        for y in 330..370 {
            for x in 420..500 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                assert_eq!(s.hit(px, py).is_some(), form.sample(px, py).is_some(), "({px},{py})");
            }
        }
    }

    /// The documented turn contract, on the basis vectors: positive yaw
    /// brings the right side toward the viewer, positive pitch the top,
    /// positive roll turns the right side down (clockwise, y down).
    #[test]
    fn turn_follows_its_contract() {
        let a = 0.5f32;
        let body_to_view = |s: Sdf, b: V3| -> V3 {
            let Sdf::Turn { m, .. } = s else { unreachable!() };
            [dot(m[0], b), dot(m[1], b), dot(m[2], b)]
        };
        let unit_ball = || Sdf::ellipsoid([0.0; 3], [1.0; 3]);
        let right = body_to_view(unit_ball().turn([0.0; 3], a, 0.0, 0.0), [1.0, 0.0, 0.0]);
        assert!(right[2] > 0.4, "yaw: {right:?}");
        let top = body_to_view(unit_ball().turn([0.0; 3], 0.0, a, 0.0), [0.0, -1.0, 0.0]);
        assert!(top[2] > 0.4 && top[1] < 0.0, "pitch: {top:?}");
        let right = body_to_view(unit_ball().turn([0.0; 3], 0.0, 0.0, a), [1.0, 0.0, 0.0]);
        assert!(right[1] > 0.4 && right[2].abs() < 1e-6, "roll: {right:?}");
        // a ball above the pivot, pitched a quarter turn, comes round to the
        // front
        let s = Sdf::ellipsoid([0.0, -10.0, 0.0], [1.0; 3]).turn([0.0; 3], 0.0, std::f32::consts::FRAC_PI_2, 0.0);
        let h = s.hit(0.0, 0.0).expect("in front of the pivot");
        assert!((h.z - 11.0).abs() < 0.1, "{}", h.z);
        // and a pitched block shows its top face (facet 4) above its front
        let b = Sdf::block([0.0; 3], [40.0, 40.0, 40.0], 0.0).turn([0.0; 3], 0.0, 0.4, 0.0);
        assert_eq!(b.hit(0.0, -22.0).map(|h| h.facet), Some(4));
        assert_eq!(b.hit(0.0, 0.0).map(|h| h.facet), Some(5));
    }
}
