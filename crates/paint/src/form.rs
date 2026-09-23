//! Form and light for solids: the painter's model of a rock or a mountain.
//!
//! A painter doesn't copy a rock's colors; they think of it as a solid made
//! of planes, some turned toward the light, some away, and paint that. This
//! module gives a painting program that solid:
//!
//! - `Form` is a depth buffer over the canvas: for every pixel the nearest
//!   solid, its distance toward the viewer `z` (units), its surface normal, a
//!   part id (which solid) and a facet id (which plane of it), and its
//!   distance from the viewer for aerial perspective.
//! - Solids are either 3-D bodies (`Sdf`: ellipsoids, blocks, fracture planes
//!   cut through them, weathering) seen straight on, or reliefs over the
//!   canvas (`Ridge`: an eroded mountain face with spurs and gullies running
//!   down from its crest; `Relief`: any height function).
//! - `Form::light` casts shadows over it; then `shade` says, per point, how far
//!   the plane turns toward the light, how much direct light it gets, whether
//!   it lies in a cast shadow, how much reflected light and sky it sees.
//! - Fields for brush handling: `fall` (the way water runs down the plane),
//!   `across` (around the form), `edge_angle`; masks for parts, facets, lit
//!   planes, shadows, silhouettes whose edges soften with distance, and the
//!   hard edges where planes break or one solid overlaps another.
//!
//! None of this paints anything: colors, brushes and the order of passes are
//! the painter's. Coordinates: x right, y down (canvas units), z toward the
//! viewer. Normals are unit vectors in that frame.

use crate::canvas::Frame;
use crate::mask::Mask;
use crate::noise::Fbm;
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

/// One light (the sun or the sky's brightest quarter) plus what fills the
/// shadows: ambient sky light and light reflected from lit surroundings.
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
    /// occluder (0.02 crisp sun .. 0.3 hazy).
    pub penumbra: f32,
    /// How far (units) shadows are traced.
    pub reach: f32,
    /// Occluders more than this far (units of z) in front of a shadow ray
    /// are taken to be in front of it, not blocking it.
    pub thickness: f32,
    /// Whether one part casts shadows on another (a boulder on the ground:
    /// yes; mountain ranges miles apart: no).
    pub across_parts: bool,
}

impl Light {
    /// Light coming from `from` in the picture ((-1, -0.5): from the left, a
    /// little above), with `front` its component toward the viewer: 0 rakes
    /// across the picture plane, 1 comes from behind the painter's shoulder,
    /// negative comes from behind the motif (contre-jour: dark masses with
    /// lit rims). Reflected light comes from below and the far side.
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
/// units with z toward the viewer. Build it the way a stone is described:
/// a mass (ellipsoid or block), turned, cut by fracture planes, weathered.
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
    /// An ellipsoid centered at `c` with radii `r`.
    pub fn ellipsoid(c: V3, r: V3) -> Sdf {
        Sdf::Ellipsoid { c, r }
    }
    /// A box centered at `c`, `size` wide/tall/deep, edges rounded by `round`.
    /// Its faces are facets 1 right, 2 left, 3 bottom, 4 top, 5 front, 6 back
    /// (in the block's own frame, so they follow it when turned).
    pub fn block(c: V3, size: V3, round: f32) -> Sdf {
        Sdf::Block { c, half: [size[0] * 0.5, size[1] * 0.5, size[2] * 0.5], round }
    }
    /// Everything behind the plane through `at` whose outward normal is `n`.
    pub fn half_space(at: V3, n: V3) -> Sdf {
        Sdf::Plane { at, n: unit(n) }
    }
    /// Split off everything beyond the plane through `at` facing `n` (a
    /// fracture plane); the new face becomes facet `facet`. `round` rounds
    /// the new edges (weathered arrises; 0 = fresh break).
    pub fn cut(self, at: V3, n: V3, facet: u16, round: f32) -> Sdf {
        Sdf::Inter(vec![self, Sdf::Facet(Box::new(Sdf::half_space(at, n)), facet)], round)
    }
    pub fn union(self, o: Sdf, smooth: f32) -> Sdf {
        Sdf::Union(vec![self, o], smooth)
    }
    pub fn subtract(self, o: Sdf, smooth: f32) -> Sdf {
        Sdf::Subtract(Box::new(self), Box::new(o), smooth)
    }
    /// Rotate about `c`: `yaw` turns it about the vertical (positive turns
    /// its right side toward the viewer), `pitch` tips its top toward the
    /// viewer (so we look down on it), `roll` leans it in the picture plane
    /// (positive clockwise). Radians.
    pub fn turn(self, c: V3, yaw: f32, pitch: f32, roll: f32) -> Sdf {
        let (sy, cy) = yaw.sin_cos();
        let (sp, cp) = pitch.sin_cos();
        let (sr, cr) = roll.sin_cos();
        // body → view: roll · pitch · yaw (row vectors of the matrix)
        let ry = [[cy, 0.0, -sy], [0.0, 1.0, 0.0], [sy, 0.0, cy]];
        let rp = [[1.0, 0.0, 0.0], [0.0, cp, -sp], [0.0, sp, cp]];
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
    /// Weather the surface: displace it by 3-D fractal noise of `amp` units
    /// at `period`. `ridged` makes sharp-lipped pits and crests (granite
    /// grain, eroded sandstone) instead of soft lumps.
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
            Sdf::Turn { body, c, .. } => {
                let (lo, hi) = body.aabb();
                // a sphere around the body's box, about the pivot
                let r = [lo, hi, [lo[0], hi[1], lo[2]], [hi[0], lo[1], hi[2]]].iter().map(|q| len(sub(*q, *c))).fold(0.0f32, f32::max)
                    .max(len(sub(hi, *c)))
                    .max(len(sub(lo, *c)));
                ([c[0] - r, c[1] - r, c[2] - r], [c[0] + r, c[1] + r, c[2] + r])
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
    /// Orthographic sphere tracing along the line of sight (−z).
    fn hit(&self, x: f32, y: f32) -> Option<Hit> {
        let (lo, hi) = self.aabb();
        let (z_front, z_back) = (hi[2] + 1.0, lo[2] - 1.0);
        let eps = 0.02;
        let mut z = z_front;
        for _ in 0..160 {
            let (d, _) = self.eval([x, y, z]);
            if d < eps {
                // refine: step back and forth once
                let z1 = z + d;
                let p = [x, y, z1];
                let (_, facet) = self.eval(p);
                return Some(Hit { z: z1, n: self.normal(p, 0.08), facet });
            }
            z -= (d * 0.8).max(eps * 0.5);
            if z < z_back {
                return None;
            }
        }
        None
    }
}

/// Any relief over the canvas: `f(x, y)` gives the height toward the viewer
/// and a facet id, or None where there is nothing. Normals come from its
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

/// A mountain face or cliff seen from in front: a skyline (the crest), and
/// below it a face that leans back from the viewer (`lean`), cut by gullies
/// that start just under the crest and run down the fall lines, fanning out
/// and widening as they descend, with rounded spurs between them. Optional
/// strata make ledges across it.
///
/// Facets: 0 the face; 1 the top of a ledge, 2 its riser (with strata).
pub struct Ridge {
    x0: f32,
    /// Crest y sampled every unit from x0, and its prefix sums.
    crest: Vec<f32>,
    sums: Vec<f64>,
    /// How far below the crest the face reaches (units).
    pub depth: f32,
    /// How the face leans back: dz/dy (0 a sheer wall facing us, 1 a slope
    /// tilted 45° toward the sky). Mountains ~0.6–1.2, cliffs ~0.1–0.3.
    pub lean: f32,
    /// Extra lean at the foot (a concave slope flattening out).
    pub foot: f32,
    /// Gully spacing just under the crest (units); they widen downhill.
    pub gully: f32,
    /// Gully depth, as a fraction of their spacing.
    pub carve: f32,
    /// How much the gullies follow the fall line (1) or run straight down (0).
    pub fan: f32,
    /// Ledges: (spacing, step height in z, tilt as dy/dx), or None.
    pub strata: Option<(f32, f32, f32)>,
    /// Added to every z (to place it in front of or behind other solids).
    pub z0: f32,
    /// Where the face meets level ground or water (y), if above its depth.
    pub base: Option<f32>,
    spurs: Fbm,
    rills: Fbm,
    wander: Fbm,
}

impl Ridge {
    /// A ridge from x0 to x1 whose crest is `crest(x)`, reaching `depth`
    /// units below it.
    pub fn new(x0: f32, x1: f32, crest: impl Fn(f32) -> f32, depth: f32, seed: u32) -> Self {
        let n = (x1 - x0).ceil().max(1.0) as usize + 1;
        let crest: Vec<f32> = (0..n).map(|i| crest(x0 + i as f32)).collect();
        let mut sums = vec![0.0f64; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + crest[i] as f64;
        }
        Ridge {
            x0,
            crest,
            sums,
            depth,
            lean: 0.8,
            foot: 0.6,
            gully: 40.0,
            carve: 0.35,
            fan: 1.0,
            strata: None,
            z0: 0.0,
            base: None,
            spurs: Fbm::new(seed, 3, 1.0),
            rills: Fbm::new(seed + 7, 3, 1.0),
            wander: Fbm::new(seed + 13, 2, 1.0),
        }
    }
    pub fn lean(mut self, lean: f32, foot: f32) -> Self {
        self.lean = lean;
        self.foot = foot;
        self
    }
    pub fn gullies(mut self, spacing: f32, carve: f32) -> Self {
        self.gully = spacing;
        self.carve = carve;
        self
    }
    pub fn fan(mut self, f: f32) -> Self {
        self.fan = f;
        self
    }
    pub fn strata(mut self, spacing: f32, step: f32, tilt: f32) -> Self {
        self.strata = Some((spacing, step, tilt));
        self
    }
    pub fn z0(mut self, z: f32) -> Self {
        self.z0 = z;
        self
    }
    /// The face stops at this level line (a beach, a lake, a valley floor).
    pub fn base(mut self, y: f32) -> Self {
        self.base = Some(y);
        self
    }

    /// The crest's y at x (linear between samples, flat past the ends).
    pub fn crest(&self, x: f32) -> f32 {
        let t = (x - self.x0).clamp(0.0, (self.crest.len() - 1) as f32);
        let i = (t as usize).min(self.crest.len() - 2);
        let u = t - i as f32;
        self.crest[i] + (self.crest[i + 1] - self.crest[i]) * u
    }

    /// The crest averaged over ±r units (the shape of the mass seen from
    /// further down its face, where the small notches no longer matter).
    fn crest_smooth(&self, x: f32, r: f32) -> f32 {
        if r < 0.5 {
            return self.crest(x);
        }
        // the integral of the crest (piecewise constant per sample, from
        // sample centers), continuous in x and r so the face has no steps
        let n = self.crest.len();
        let integral = |t: f32| -> f64 {
            let t = (t - self.x0 + 0.5).clamp(0.0, n as f32);
            let i = (t as usize).min(n - 1);
            self.sums[i] + (t - i as f32) as f64 * self.crest[i] as f64
        };
        let (a, b) = (x - r, x + r);
        let (ca, cb) = ((a - self.x0 + 0.5).clamp(0.0, n as f32), (b - self.x0 + 0.5).clamp(0.0, n as f32));
        if cb - ca < 1e-3 {
            return self.crest(x);
        }
        ((integral(b) - integral(a)) / (cb - ca) as f64) as f32
    }

    fn height(&self, x: f32, y: f32) -> Option<(f32, u16)> {
        let d = y - self.crest(x);
        if d < 0.0 || d > self.depth || self.base.is_some_and(|b| y > b) {
            return None;
        }
        // the mass: a face leaning back, steepest near the crest, flattening
        // toward the foot; deeper down, only the large shape of the crest counts
        let r = (d * 0.8).min(200.0);
        let cs = self.crest_smooth(x, r);
        let h = (r * 0.5).max(3.0);
        let slope = (self.crest_smooth(x + h, r) - self.crest_smooth(x - h, r)) / (2.0 * h);
        let dd = (y - cs).max(0.0);
        let s = self.lean + self.foot * 2.0 * dd / self.depth;
        let mut z = self.lean * dd + self.foot * dd * dd / self.depth;
        // follow the fall line back up to where it leaves the crest
        let k = s * s * slope / (s * s * slope * slope + 1.0);
        let u = x - self.fan * k * dd;
        // gullies: they wander a little, and downhill the fine ones merge
        // into fewer, wider ones (a fine set fading into a coarse one; the
        // noise is sampled at fixed scales so nothing shears)
        let g = self.gully;
        let uw = u + 0.35 * g * self.wander.get(u / (g * 2.0), dd / (g * 6.0));
        let merge = crate::smoothstep(0.0, self.depth * 0.7, dd);
        let a = self.carve * g * (1.0 + 1.5 * dd / self.depth) * crate::smoothstep(0.0, g * 0.8, d);
        let v = |n: f32, p: f32| n.abs().min(0.6).powf(p);
        let fine = v(self.spurs.get(uw / g, dd / (g * 3.5)), 0.75);
        let coarse = v(self.spurs.get(uw / (g * 2.2) + 17.3, dd / (g * 7.0)), 0.75);
        let rills = v(self.rills.get(uw / (g * 0.3), dd / (g * 1.2)), 0.8);
        z += a * (fine + (coarse - fine) * merge + 0.2 * rills);
        let mut facet = 0;
        if let Some((sp, step, tilt)) = self.strata {
            // ledges: each bed leans back a little, then a riser
            let q = (d + tilt * (x - self.x0) + 0.2 * sp * self.wander.get(x / (sp * 6.0), 3.0)) / sp;
            let fr = q - q.floor();
            let riser = crate::smoothstep(0.78, 1.0, fr);
            z += step * (q.floor() + riser);
            facet = if fr < 0.78 { 1 } else { 2 };
        }
        Some((z + self.z0, facet))
    }
}

impl Solid for Ridge {
    fn bounds(&self) -> [f32; 4] {
        let top = self.crest.iter().cloned().fold(f32::INFINITY, f32::min);
        let bot = self.crest.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let low = self.base.map_or(bot + self.depth, |b| b.min(bot + self.depth));
        [self.x0, top, self.x0 + (self.crest.len() - 1) as f32, low]
    }
    fn hit(&self, x: f32, y: f32) -> Option<Hit> {
        relief_hit(&|x, y| self.height(x, y), x, y)
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
    /// 0 = left→right, y down): the way water runs down the plane.
    pub fn fall(&self) -> f32 {
        fall_angle(self.n)
    }
    /// Around the form: perpendicular to the fall line.
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

    #[inline]
    fn px(&self, i: usize) -> (f32, f32) {
        let inv = 1.0 / self.f.scale;
        (((i % self.f.w) as f32 + 0.5) * inv, ((i / self.f.w) as f32 + 0.5) * inv)
    }

    /// Add a solid at distance `dist` from the viewer (any unit the painter
    /// likes, used only for aerial perspective). Where it is nearer than what
    /// is already there, it hides it. Returns its part id (1, 2, …).
    pub fn add(&mut self, s: &dyn Solid, dist: f32) -> PartId {
        self.add_at(s, &|_, _, _| dist)
    }

    /// Like `add`, with the distance varying over the solid:
    /// `dist(x, y, z)` (a ground plane running back, a ridge whose foot is
    /// nearer than its crest).
    pub fn add_at(&mut self, s: &dyn Solid, dist: &(dyn Fn(f32, f32, f32) -> f32 + Sync)) -> PartId {
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
        let bw = px1 - px0;
        let hits: Vec<Option<(Hit, f32)>> = (0..bw * (py1 - py0))
            .into_par_iter()
            .map(|k| {
                let (x, y) = (((px0 + k % bw) as f32 + 0.5) * inv, ((py0 + k / bw) as f32 + 0.5) * inv);
                s.hit(x, y).map(|h| (h, dist(x, y, h.z)))
            })
            .collect();
        for (k, h) in hits.into_iter().enumerate() {
            if let Some((h, d)) = h {
                let i = (py0 + k / bw) * f.w + px0 + k % bw;
                if h.z > self.z[i] {
                    self.z[i] = h.z;
                    self.n[i] = h.n;
                    self.part[i] = id;
                    self.facet[i] = h.facet;
                    self.dist[i] = d;
                }
            }
        }
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
        let cast: Vec<f32> = (0..f.w * f.h)
            .into_par_iter()
            .map(|i| {
                if part[i] == 0 {
                    return 0.0;
                }
                let (x, y) = self.px(i);
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
            })
            .collect();
        self.cast = cast;
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

    /// Fall-line angle at a point (straight down where no solid is): brush
    /// strokes down the planes.
    pub fn fall(&self, x: f32, y: f32) -> f32 {
        self.sample(x, y).map_or(std::f32::consts::FRAC_PI_2, |s| s.fall())
    }

    /// Perpendicular to the fall line: strokes around the form.
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
    /// `form.mask(|s| if s.part == rock { s.shade.lit(0.2) } else { 0.0 })`.
    pub fn mask(&self, g: impl Fn(&Sample) -> f32 + Sync) -> Mask {
        let data = (0..self.f.w * self.f.h).into_par_iter().map(|i| self.sample_i(i).map_or(0.0, |s| g(&s))).collect();
        Mask { f: self.f, data }
    }

    /// The silhouette of the given parts, its edge `soft(&sample)` units wide
    /// (evaluated at the nearest point inside): crisp near, lost in haze far
    /// off, softer where the form turns away than where a plane breaks.
    pub fn silhouette(&self, parts: &[PartId], soft: impl Fn(&Sample) -> f32 + Sync) -> Mask {
        let inside = self.mask(|s| if parts.contains(&s.part) { 1.0 } else { 0.0 });
        let f = self.f;
        inside.soften(|x, y| self.sample_i(f.index(x, y)).map_or(0.0, |s| soft(&s)))
    }

    /// Hard edges inside the form: plane breaks (normals turning by more
    /// than `turn` radians within `span` units) and overlaps (depth jumping
    /// by more than `step` units, where one solid or ledge stands in front of
    /// another). 0..1, strongest at the sharpest breaks.
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
                    // one ledge standing in front of another
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
    /// is convex (an arris, a spur: catches light, often a light edge),
    /// negative where concave (a joint, a gully, a crevice: a dark accent).
    /// Roughly the turn in radians across the span, strongest of the two axes.
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
        let rock = Sdf::ellipsoid([500.0, 350.0, 0.0], [200.0, 150.0, 120.0]).cut([500.0, 300.0, 60.0], [-0.3, -0.5, 1.0], 1, 0.0);
        let id = form.add(&rock, 1.0);
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

    #[test]
    fn a_ridge_has_flanks_and_gullies() {
        let mut form = Form::new(frame());
        // a peak at x = 500
        let r = Ridge::new(0.0, 1000.0, |x| 200.0 + (x - 500.0).abs() * 0.5, 400.0, 3).gullies(40.0, 0.4);
        form.add(&r, 1.0);
        form.light(Light::new((-1.0, -0.5), 0.4));
        assert!(form.sample(500.0, 190.0).is_none());
        // averaged over the gullies, the left flank faces left, the right right
        let mean_nx = |x0: f32| (0..40).map(|k| form.sample(x0 + k as f32 * 3.0, 380.0).unwrap().n[0]).sum::<f32>() / 40.0;
        assert!(mean_nx(250.0) < -0.05 && mean_nx(650.0) > 0.05);
        // the gullies make light and dark alternate across the face
        let vals: Vec<f32> = (0..200).map(|k| form.shade(300.0 + k as f32 * 2.0, 450.0).direct).collect();
        let (lo, hi) = vals.iter().fold((1.0f32, 0.0f32), |a, &v| (a.0.min(v), a.1.max(v)));
        assert!(hi - lo > 0.3, "{lo}..{hi}");
    }
}
