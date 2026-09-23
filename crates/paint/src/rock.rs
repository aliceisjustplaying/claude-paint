//! Rocks grown from a drawn outline: the painter draws the silhouette (and,
//! if they like, a few crack or plane lines inside it) and this infers a
//! plausible solid behind the drawing, lights it with the picture's sun and
//! hands back what a painter needs to paint it: its planes, the light and
//! shadow families (lit, halftone, core shadow, reflected light), cracks and
//! their occlusion, the seam where it meets the ground, the shadow it casts
//! there, the faces that turn up to the sky (where snow lies) and stroke
//! directions. Geometry and light only; it paints nothing.
//!
//! How the solid is inferred (all in canvas units, z toward the viewer):
//!
//! - the outline is inflated into a **mass**: a pillow whose height grows
//!   with the distance from the drawn line, round for granite, boxy (steep
//!   walls, a flat top) for sandstone and chalk. It turns away at the
//!   silhouette;
//! - **planes** are cut into the mass: each is tangent to it at a site,
//!   sunk a little and turned a little, so it holds a patch around its site
//!   and the patches meet in arrises. Rim planes sit along the outline's
//!   spans (split at its corners, the drawn ones and the sharp turns found
//!   on the line), so arrises run in from the corners; face planes spread
//!   over the inside, upper ones turned to the sky, lower ones to the
//!   ground;
//! - each drawn **crack** or **plane line** seeds a pair of planes that
//!   meet on it (a fracture: the two sides face different ways) and, for a
//!   crack, a groove along it: a V joint with walls of different pitch and a
//!   narrow crevice at its bottom. The outline's concave corners grow joints
//!   of their own (a notch in a silhouette is where a crack comes out);
//! - the planes are joined by a soft minimum: a wide one gives the rounded,
//!   weathered arrises of a granite erratic, a narrow one the sharp edges of
//!   fresh sandstone;
//! - sandstone gets **bedding** (beds of uneven thickness, each face bulging
//!   and recessed at its joints), chalk vertical **flutes**, and all of them
//!   weathering lumps and grain.
//!
//! The height field is then lit with a `form::Light` (the world's sun):
//! n·L per point, cast shadows traced over the rock itself (so cracks and
//! overhangs shade what lies under them) and onto a ground plane behind and
//! beside it, reflected light from below, and ambient occlusion from the
//! cavities. Since the planes are many, the terminator runs along their
//! arrises in steps, not down one seam.
//!
//! Deterministic by seed; independent of the canvas resolution (the solid
//! lives on its own grid in canvas units).

use crate::form::{Light, Shade, V3, fall_angle, unit};
use crate::noise::Fbm;
use crate::{Frame, Mask, Rng, Shape, smoothstep};
use rayon::prelude::*;
use std::f32::consts::PI;

type P = (f32, f32);

/// What the stone is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RockKind {
    /// A glacial erratic: few big planes, rounded weathered arrises, lumps.
    Granite,
    /// Bedded and jointed: sharp arrises, beds with recessed joints, ledges.
    Sandstone,
    /// White, near-vertical faces, fluted by rain, sharp.
    Chalk,
}

impl RockKind {
    pub fn named(s: &str) -> Option<RockKind> {
        match s {
            "granite" | "erratic" => Some(RockKind::Granite),
            "sandstone" => Some(RockKind::Sandstone),
            "chalk" => Some(RockKind::Chalk),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            RockKind::Granite => "granite",
            RockKind::Sandstone => "sandstone",
            RockKind::Chalk => "chalk",
        }
    }
}

/// How a rock is inferred from its outline. Lengths given as fractions are
/// of the rock's inradius `R` (the largest circle inside the outline).
#[derive(Clone, Debug)]
pub struct RockSpec {
    pub kind: RockKind,
    /// Rounding of the arrises (fraction of R): 0.1 weathered, 0.02 fresh.
    pub round: f32,
    /// Face planes turned toward the viewer (more: a more broken surface).
    pub facets: usize,
    /// The mass's profile across its outline: 2 a round pillow (granite),
    /// 4 a box with a flat top and steep walls (sandstone, chalk).
    pub profile: f32,
    /// How deep each plane is sunk into the mass (fraction of its height),
    /// min..max: deeper cuts give bigger planes.
    pub sink: (f32, f32),
    /// How far each plane is turned from the mass's own slope (tangent), min..max.
    pub tilt: (f32, f32),
    /// Height of the crown over the outline's plane (fraction of R).
    pub bulge: f32,
    /// Bed thickness (units; 0: no bedding), tilt (radians, + falls to the
    /// right) and recess at the joints (fraction of R).
    pub bed: f32,
    pub bed_tilt: f32,
    pub bed_recess: f32,
    /// Crack groove depth (fraction of R) and crevice width (units).
    pub crack_depth: f32,
    pub crack_width: f32,
    /// Share of the outline's concave corners that grow a joint.
    pub joints: f32,
    /// Weathering: broad lumps and fine grain (fractions of R).
    pub lumps: f32,
    pub grain: f32,
    /// Vertical flutes (chalk), depth as a fraction of R.
    pub flutes: f32,
    /// Ground foreshortening: z units per canvas unit down (the ground the
    /// rock stands on, for its cast shadow).
    pub ground: f32,
}

impl RockSpec {
    pub fn granite() -> Self {
        RockSpec {
            kind: RockKind::Granite,
            round: 0.09,
            facets: 8,
            profile: 2.2,
            sink: (0.05, 0.14),
            tilt: (0.2, 0.7),
            bulge: 0.8,
            bed: 0.0,
            bed_tilt: 0.0,
            bed_recess: 0.0,
            crack_depth: 0.1,
            crack_width: 2.0,
            joints: 0.6,
            lumps: 0.045,
            grain: 0.01,
            flutes: 0.0,
            ground: 4.0,
        }
    }
    pub fn sandstone() -> Self {
        RockSpec {
            kind: RockKind::Sandstone,
            round: 0.025,
            facets: 6,
            profile: 4.0,
            sink: (0.03, 0.09),
            tilt: (0.1, 0.4),
            bulge: 0.6,
            bed: -1.0, // from the size
            bed_tilt: 0.03,
            bed_recess: 0.16,
            crack_depth: 0.16,
            crack_width: 2.4,
            joints: 0.9,
            lumps: 0.015,
            grain: 0.008,
            flutes: 0.0,
            ground: 4.0,
        }
    }
    pub fn chalk() -> Self {
        RockSpec {
            kind: RockKind::Chalk,
            round: 0.03,
            facets: 6,
            profile: 3.5,
            sink: (0.03, 0.09),
            tilt: (0.1, 0.4),
            bulge: 0.55,
            bed: 0.0,
            bed_tilt: 0.0,
            bed_recess: 0.0,
            crack_depth: 0.1,
            crack_width: 1.8,
            joints: 0.4,
            lumps: 0.02,
            grain: 0.006,
            flutes: 0.02,
            ground: 4.0,
        }
    }
    pub fn of(kind: RockKind) -> Self {
        match kind {
            RockKind::Granite => Self::granite(),
            RockKind::Sandstone => Self::sandstone(),
            RockKind::Chalk => Self::chalk(),
        }
    }
}

/// Where a plane came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneKind {
    /// Turns away at a span of the outline.
    Rim,
    /// Turned toward the viewer.
    Face,
    /// One side of a drawn crack or plane line.
    Fracture,
}

impl PlaneKind {
    pub fn name(self) -> &'static str {
        match self {
            PlaneKind::Rim => "rim",
            PlaneKind::Face => "face",
            PlaneKind::Fracture => "fracture",
        }
    }
}

/// One plane of the rock: `z = h - g·(p - at)` (it faces toward `g`), plus,
/// for a fracture plane, a lift beyond the ends of its line so it stays local.
#[derive(Clone, Debug)]
pub struct Plane {
    pub kind: PlaneKind,
    pub at: P,
    pub h: f32,
    pub g: P,
    /// Unit normal (x right, y down, z toward the viewer).
    pub n: V3,
    /// Where the plane holds: within `radius` of the segment a..b (a face:
    /// a point; a fracture: its line); beyond, it lifts away so it never
    /// cuts the far side of the rock.
    reach: Option<(P, P, f32)>,
}

impl Plane {
    fn new(kind: PlaneKind, at: P, h: f32, g: P) -> Plane {
        Plane { kind, at, h, g, n: unit([g.0, g.1, 1.0]), reach: None }
    }
    #[inline]
    fn z(&self, x: f32, y: f32) -> f32 {
        let mut z = self.h - self.g.0 * (x - self.at.0) - self.g.1 * (y - self.at.1);
        if let Some((a, b, rad)) = self.reach {
            let e = seg((x, y), a, b).0 - rad;
            if e > 0.0 {
                z += 3.0 * e * e / (e + 0.25 * rad);
            }
        }
        z
    }
}

/// A drawn or grown line inside the rock.
#[derive(Clone, Debug)]
pub struct Seam {
    pub pts: Vec<P>,
    /// A crack (a groove) or a plane line (an arris, no groove).
    pub crack: bool,
    /// Grown from a corner of the outline (not drawn).
    pub grown: bool,
}

/// The rock: its outline, planes and lines, and the lit solid on a grid.
pub struct Rock {
    pub kind: RockKind,
    pub outline: Vec<P>,
    pub seams: Vec<Seam>,
    pub planes: Vec<Plane>,
    pub light: Light,
    /// Inradius (units).
    pub r: f32,
    pub bounds: (f32, f32, f32, f32),
    /// The ground line under it (y at its lowest point) and its foot: the
    /// stretch of outline that meets the ground.
    pub base: f32,
    pub foot: Vec<P>,
    /// Upper edges (outline points whose outward normal points up).
    pub tops: Vec<P>,
    bed: f32,
    bed_tilt: f32,
    bed_y0: f32,
    beds: Vec<f32>,
    // the grid over the rock's bounds
    x0: f32,
    y0: f32,
    step: f32,
    nx: usize,
    ny: usize,
    inside: Vec<bool>,
    h: Vec<f32>,
    n: Vec<V3>,
    facet: Vec<u16>,
    crack: Vec<f32>,
    ao: Vec<f32>,
    cast: Vec<f32>,
    // the ground's cast shadow on a coarser grid around the rock
    gx0: f32,
    gy0: f32,
    gstep: f32,
    gnx: usize,
    gny: usize,
    gcast: Vec<f32>,
    seed: u64,
}

/// Everything known at one point of a rock.
#[derive(Clone, Copy, Debug)]
pub struct RockSample {
    pub n: V3,
    pub z: f32,
    /// Index into `planes`.
    pub plane: usize,
    pub shade: Shade,
    /// Occlusion 0..1 (cavities, cracks, the foot).
    pub ao: f32,
    /// On a crack or bed joint, 0..1.
    pub crack: f32,
}

impl RockSample {
    /// The light as one value (shade value less occlusion).
    pub fn value(&self) -> f32 {
        (self.shade.value * (1.0 - 0.65 * self.ao)).clamp(0.0, 1.0)
    }
    /// How much the point faces up, 0..1 (1: a level top).
    pub fn up(&self) -> f32 {
        (-self.n[1]).clamp(0.0, 1.0)
    }
    /// Core shadow: just past the terminator, where neither the sun nor the
    /// reflected light reach.
    pub fn core(&self) -> f32 {
        let past = smoothstep(0.06, -0.04, self.shade.turn) * smoothstep(-0.55, -0.12, self.shade.turn);
        let cast = smoothstep(0.3, 0.8, self.shade.cast) * smoothstep(0.02, -0.1, -self.shade.turn) * 0.6;
        (past.max(cast) * (1.0 - smoothstep(0.08, 0.4, self.shade.bounce))).clamp(0.0, 1.0)
    }
    /// Reflected light in the shadow family, 0..1.
    pub fn reflected(&self) -> f32 {
        (1.0 - self.shade.lit(0.12)) * smoothstep(0.25, 0.65, self.shade.bounce) * (1.0 - 0.7 * self.ao)
    }
}

// ------------------------------------------------------------- geometry

fn area2(p: &[P]) -> f32 {
    let n = p.len();
    (0..n).map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1).sum()
}

fn inside_poly(p: &[P], x: f32, y: f32) -> bool {
    let mut c = false;
    let n = p.len();
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (p[i], p[j]);
        if (a.1 > y) != (b.1 > y) && x < (b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0 {
            c = !c;
        }
        j = i;
    }
    c
}

/// Distance from a point to a segment, and the parameter along it.
#[inline]
fn seg(p: P, a: P, b: P) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 1e-9 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    let (qx, qy) = (a.0 + dx * t, a.1 + dy * t);
    (((p.0 - qx).powi(2) + (p.1 - qy).powi(2)).sqrt(), t)
}

/// Distance to a closed polygon's edge.
fn dist_poly(p: &[P], x: f32, y: f32) -> f32 {
    let n = p.len();
    let mut best = f32::MAX;
    for i in 0..n {
        best = best.min(seg((x, y), p[i], p[(i + 1) % n]).0);
    }
    best
}

/// Nearest point on an open polyline: distance, arc fraction (0..1),
/// signed side (+ left of the direction of travel in y-down coordinates)
/// and the local direction.
fn near_line(l: &[P], cum: &[f32], x: f32, y: f32) -> (f32, f32, f32, P) {
    let mut best = (f32::MAX, 0.0, 0.0, (1.0, 0.0));
    let total = cum.last().copied().unwrap_or(1.0).max(1e-6);
    for i in 0..l.len().saturating_sub(1) {
        let (d, t) = seg((x, y), l[i], l[i + 1]);
        if d < best.0 {
            let (dx, dy) = (l[i + 1].0 - l[i].0, l[i + 1].1 - l[i].1);
            let len = (dx * dx + dy * dy).sqrt().max(1e-6);
            let side = (dx * (y - l[i].1) - dy * (x - l[i].0)).signum();
            best = (d, (cum[i] + t * len) / total, side, (dx / len, dy / len));
        }
    }
    best
}

fn cumlen(l: &[P]) -> Vec<f32> {
    let mut c = vec![0.0];
    for w in l.windows(2) {
        let d = ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt();
        c.push(c.last().unwrap() + d);
    }
    c
}

/// Resample a closed line to about `n` points.
fn resample_closed(p: &[P], n: usize) -> Vec<P> {
    let mut l = p.to_vec();
    l.push(p[0]);
    let c = cumlen(&l);
    let total = *c.last().unwrap();
    let mut out = Vec::with_capacity(n);
    let mut k = 0;
    for i in 0..n {
        let s = total * i as f32 / n as f32;
        while k + 1 < c.len() - 1 && c[k + 1] < s {
            k += 1;
        }
        let seg_len = (c[k + 1] - c[k]).max(1e-6);
        let t = ((s - c[k]) / seg_len).clamp(0.0, 1.0);
        out.push((l[k].0 + (l[k + 1].0 - l[k].0) * t, l[k].1 + (l[k + 1].1 - l[k].1) * t));
    }
    out
}

/// Corners of a closed line: indices where it turns sharply over a window
/// of `w` points (local maxima of turning over `min_turn` radians), merged
/// with the given ones.
fn find_corners(p: &[P], w: usize, min_turn: f32, given: &[P]) -> Vec<usize> {
    let n = p.len();
    let dir = |a: P, b: P| (b.1 - a.1).atan2(b.0 - a.0);
    let turn: Vec<f32> = (0..n)
        .map(|i| {
            let a = dir(p[(i + n - w) % n], p[i]);
            let b = dir(p[i], p[(i + w) % n]);
            let mut d = b - a;
            while d > PI {
                d -= 2.0 * PI;
            }
            while d < -PI {
                d += 2.0 * PI;
            }
            d.abs()
        })
        .collect();
    let mut out: Vec<usize> = (0..n)
        .filter(|&i| turn[i] > min_turn && (1..=w / 2 + 1).all(|k| turn[i] >= turn[(i + k) % n] && turn[i] >= turn[(i + n - k) % n]))
        .collect();
    for g in given {
        let i = (0..n).min_by(|&a, &b| {
            let da = (p[a].0 - g.0).powi(2) + (p[a].1 - g.1).powi(2);
            let db = (p[b].0 - g.0).powi(2) + (p[b].1 - g.1).powi(2);
            da.partial_cmp(&db).unwrap()
        });
        if let Some(i) = i {
            out.push(i);
        }
    }
    out.sort_unstable();
    out.dedup();
    // merge corners closer than the window
    let mut merged: Vec<usize> = Vec::new();
    for &i in &out {
        if let Some(&l) = merged.last()
            && i - l < w
        {
            if turn[i] > turn[l] {
                *merged.last_mut().unwrap() = i;
            }
            continue;
        }
        merged.push(i);
    }
    if merged.len() > 1 && merged[0] + n - merged[merged.len() - 1] < w {
        merged.pop();
    }
    merged
}

// ---------------------------------------------------------------- grow

impl Rock {
    /// Grow a rock from a closed outline (`outline`, dense points, e.g. an
    /// `Outline`'s line), its corners (points; more are found on the line),
    /// drawn lines inside it (`cracks`: grooves; `planes`: arrises), a spec
    /// and the light.
    pub fn grow(outline: &[P], corners: &[P], cracks: &[Vec<P>], plane_lines: &[Vec<P>], spec: &RockSpec, light: Light, seed: u64) -> Rock {
        assert!(outline.len() >= 3, "a rock's outline needs at least three points");
        let mut rng = Rng::new(seed.wrapping_mul(0x9E37_79B9).wrapping_add(17));
        let bounds = outline.iter().fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)));
        let (bw, bh) = (bounds.2 - bounds.0, bounds.3 - bounds.1);
        let size = bw.max(bh).max(1.0);
        // a working copy of the outline, evenly spaced
        let per = (cumlen(&[outline, &outline[..1]].concat()).last().copied().unwrap_or(1.0)).max(1.0);
        let m = ((per / (size / 160.0)).round() as usize).clamp(48, 480);
        let poly = resample_closed(outline, m);
        let orient = area2(&poly).signum();
        let outward = |i: usize, k: usize| -> P {
            let (a, b) = (poly[(i + m - k) % m], poly[(i + k) % m]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-6);
            (dy / l * orient, -dx / l * orient)
        };

        // grid over the bounds
        let step = (size / 520.0).clamp(0.25, 1.2);
        let (x0, y0) = (bounds.0 - 2.0 * step, bounds.1 - 2.0 * step);
        let nx = ((bw / step).ceil() as usize) + 5;
        let ny = ((bh / step).ceil() as usize) + 5;
        let at = |i: usize, j: usize| (x0 + i as f32 * step, y0 + j as f32 * step);
        let inside: Vec<bool> = (0..nx * ny).into_par_iter().map(|k| {
            let (x, y) = at(k % nx, k / nx);
            inside_poly(&poly, x, y)
        }).collect();
        let dist: Vec<f32> = (0..nx * ny).into_par_iter().map(|k| {
            if !inside[k] {
                return 0.0;
            }
            let (x, y) = at(k % nx, k / nx);
            dist_poly(&poly, x, y)
        }).collect();
        let (mut r, mut center) = (1.0f32, ((bounds.0 + bounds.2) * 0.5, (bounds.1 + bounds.3) * 0.5));
        for (k, d) in dist.iter().enumerate() {
            if *d > r {
                r = *d;
                center = at(k % nx, k / nx);
            }
        }
        let d_at = |x: f32, y: f32| -> f32 {
            let (i, j) = (((x - x0) / step).round() as isize, ((y - y0) / step).round() as isize);
            if i < 0 || j < 0 || i >= nx as isize || j >= ny as isize {
                return 0.0;
            }
            dist[j as usize * nx + i as usize]
        };

        // spans and rim planes
        let w = (m / 40).max(2);
        let mut cs = find_corners(&poly, w, 0.45, corners);
        if cs.len() < 3 {
            cs = (0..5).map(|k| k * m / 5).collect();
        }
        // split long spans
        let max_span = (m as f32 / 7.0) as usize;
        let mut spans: Vec<(usize, usize)> = Vec::new();
        for k in 0..cs.len() {
            let (a, b) = (cs[k], cs[(k + 1) % cs.len()]);
            let len = (b + m - a) % m;
            let len = if len == 0 { m } else { len };
            let parts = len.div_ceil(max_span).max(1);
            let mut s = a;
            for q in 1..=parts {
                let e = if q == parts { (a + len) % m } else { (a + len * q / parts + (rng.normal() * len as f32 / parts as f32 * 0.12) as usize) % m };
                spans.push((s, e));
                s = e;
            }
        }
        // the mass: the outline inflated into a rounded (granite) or boxy
        // (sandstone, chalk) pillow; every plane is cut tangent to it
        let bulge = spec.bulge * r;
        let pw = spec.profile.max(1.2);
        let prof = |d: f32| -> f32 {
            let t = (d / r).clamp(0.0, 1.0);
            bulge * (1.0 - (1.0 - t).powf(pw)).max(0.0).powf(1.0 / pw)
        };
        let mass: Vec<f32> = dist.iter().zip(&inside).map(|(d, i)| if *i { prof(*d) } else { 0.0 }).collect();
        let mass_at = |x: f32, y: f32| -> f32 {
            let (i, j) = (((x - x0) / step).round() as isize, ((y - y0) / step).round() as isize);
            if i < 0 || j < 0 || i >= nx as isize || j >= ny as isize {
                return 0.0;
            }
            mass[j as usize * nx + i as usize]
        };
        let slope_at = |x: f32, y: f32| -> P {
            let e = (r * 0.06).max(step * 2.0);
            ((mass_at(x + e, y) - mass_at(x - e, y)) / (2.0 * e), (mass_at(x, y + e) - mass_at(x, y - e)) / (2.0 * e))
        };
        let (t_lo, t_hi) = spec.tilt;
        let (k_lo, k_hi) = spec.sink;
        let half_h = (bh * 0.5).max(1.0);
        // a plane tangent to the mass at `site`, sunk `sink`·bulge, turned by a jitter and a bias
        let cut = |rng: &mut Rng, kind: PlaneKind, site: P, sink: f32, jitter: f32, bias: P| -> Plane {
            let gb = slope_at(site.0, site.1);
            let a = rng.range(0.0, 2.0 * PI);
            let mut g = (-gb.0 + jitter * a.cos() + bias.0, -gb.1 + jitter * a.sin() + bias.1);
            if spec.kind != RockKind::Granite {
                // walls: faces square to the view in x, tops turned up
                g.0 *= 0.8;
            }
            Plane::new(kind, site, mass_at(site.0, site.1) - sink * bulge, g)
        };
        let mut planes: Vec<Plane> = Vec::new();
        for &(a, b) in &spans {
            let len = (b + m - a) % m;
            let mid = (a + len / 2) % m;
            let o = outward(mid, (len / 2).max(1));
            let inset = r * rng.range(0.1, 0.2);
            let site = (poly[mid].0 - o.0 * inset, poly[mid].1 - o.1 * inset);
            if d_at(site.0, site.1) <= 0.0 {
                continue;
            }
            // a face turned down to the ground; one turned up to the sky
            let bias = if o.1 > 0.5 { (0.0, 0.4 * t_hi) } else if o.1 < -0.5 { (0.0, -0.3 * t_hi) } else { (0.0, 0.0) };
            let sink = rng.range(k_lo, k_hi) * 0.5;
            let jit = rng.range(t_lo, t_hi) * 0.6;
            planes.push(cut(&mut rng, PlaneKind::Rim, site, sink, jit, bias));
        }
        // face planes: sites spread over the inside, far from each other
        let mut sites: Vec<P> = Vec::new();
        let mut tries = 0;
        while sites.len() < spec.facets && tries < 600 {
            tries += 1;
            let p = (rng.range(bounds.0, bounds.2), rng.range(bounds.1, bounds.3));
            let d = d_at(p.0, p.1);
            if d < r * 0.3 {
                continue;
            }
            let near = sites.iter().map(|s| ((s.0 - p.0).powi(2) + (s.1 - p.1).powi(2)).sqrt()).fold(f32::MAX, f32::min);
            if near < r * 0.7 * (1.0 - tries as f32 / 700.0) {
                continue;
            }
            sites.push(p);
        }
        // the top: the sky-facing plane every boulder has
        if spec.kind != RockKind::Chalk {
            let top = (center.0 + rng.normal() * r * 0.3, bounds.1 + (center.1 - bounds.1) * 0.5);
            if d_at(top.0, top.1) > r * 0.2 {
                let lift = -rng.range(0.5, 1.0) * t_hi;
                let sink = rng.range(k_lo, k_hi);
                planes.push(cut(&mut rng, PlaneKind::Face, top, sink, t_lo, (0.0, lift)));
            }
        }
        for st in &sites {
            let v = ((st.1 - center.1) / half_h).clamp(-1.0, 1.0);
            // upper faces turn to the sky, lower ones to the ground
            let bias = (0.0, if v < 0.0 { 0.6 * v * t_hi } else { 0.3 * v * t_hi });
            let sink = rng.range(k_lo, k_hi);
            let jit = rng.range(t_lo, t_hi);
            planes.push(cut(&mut rng, PlaneKind::Face, *st, sink, jit, bias));
        }

        // the drawn lines, and joints from the outline's concave corners
        let mut seams: Vec<Seam> = Vec::new();
        for c in cracks.iter().filter(|c| c.len() >= 2) {
            seams.push(Seam { pts: c.clone(), crack: true, grown: false });
        }
        for c in plane_lines.iter().filter(|c| c.len() >= 2) {
            seams.push(Seam { pts: c.clone(), crack: false, grown: false });
        }
        for &c in &cs {
            if !rng.chance(spec.joints) {
                continue;
            }
            // concave: the line turns inward here
            let (a, b) = (poly[(c + m - w) % m], poly[(c + w) % m]);
            let turn = ((poly[c].0 - a.0) * (b.1 - poly[c].1) - (poly[c].1 - a.1) * (b.0 - poly[c].0)) * orient;
            if turn >= 0.0 {
                continue;
            }
            let o = outward(c, w);
            let mut dir = (-o.0, -o.1);
            if spec.kind == RockKind::Sandstone {
                // joints run across the beds: mostly vertical
                let down = if poly[c].1 < center.1 { 1.0 } else { -1.0 };
                dir = (dir.0 * 0.35, dir.1 * 0.35 + down * 0.65);
            }
            let l = (dir.0 * dir.0 + dir.1 * dir.1).sqrt().max(1e-6);
            dir = (dir.0 / l, dir.1 / l);
            let len = r * rng.range(0.6, 1.3);
            let mut pts = vec![(poly[c].0 + o.0 * step, poly[c].1 + o.1 * step)];
            let mut p = poly[c];
            for k in 1..=6 {
                let wob = rng.normal() * 0.18;
                let d = (dir.0 - dir.1 * wob, dir.1 + dir.0 * wob);
                p = (p.0 + d.0 * len / 6.0, p.1 + d.1 * len / 6.0);
                if d_at(p.0, p.1) <= 0.0 && k > 1 {
                    break;
                }
                pts.push(p);
            }
            if pts.len() >= 3 {
                seams.push(Seam { pts, crack: true, grown: true });
            }
        }
        // sandstone: vertical joints through the beds even without notches
        if spec.kind == RockKind::Sandstone {
            let k = ((bw / r) * 0.6).round() as usize;
            for q in 0..k.clamp(1, 4) {
                let x = bounds.0 + bw * (q as f32 + 0.5 + rng.normal() * 0.15) / k.clamp(1, 4) as f32;
                let (ya, yb) = (bounds.1 + rng.range(0.0, 0.4) * bh, bounds.1 + rng.range(0.6, 1.1) * bh);
                let mut pts = Vec::new();
                for i in 0..=6 {
                    let y = ya + (yb - ya) * i as f32 / 6.0;
                    let px = x + rng.normal() * r * 0.04 + (y - ya) * rng.range(-0.08, 0.08);
                    if d_at(px, y) > 0.0 {
                        pts.push((px, y));
                    }
                }
                if pts.len() >= 3 {
                    seams.push(Seam { pts, crack: true, grown: true });
                }
            }
        }

        // the base height before the lines (min of planes, soft)
        let k_round = (spec.round * r).max(step * 0.5);
        let soft_min = |vals: &mut dyn Iterator<Item = (usize, f32)>| -> (f32, usize) {
            let mut best = (f32::MAX, 0usize);
            let mut all: [(usize, f32); 96] = [(0, 0.0); 96];
            let mut n = 0;
            for (i, v) in vals {
                if v < best.0 {
                    best = (v, i);
                }
                if n < 96 {
                    all[n] = (i, v);
                    n += 1;
                }
            }
            let s: f32 = all[..n].iter().map(|(_, v)| (-(v - best.0) / k_round).exp()).sum();
            (best.0 - k_round * s.ln(), best.1)
        };

        // each line seeds a pair of planes meeting on it: the two sides of
        // a fracture face different ways
        for sm in &seams {
            let (a, b) = (sm.pts[0], *sm.pts.last().unwrap());
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let len = (dx * dx + dy * dy).sqrt().max(1e-3);
            let nrm = (-dy / len, dx / len);
            let mid = sm.pts[sm.pts.len() / 2];
            for (sgn, k) in [(1.0, rng.range(0.35, 1.0)), (-1.0, rng.range(0.35, 1.0))] {
                let site = (mid.0 + sgn * nrm.0 * r * 0.2, mid.1 + sgn * nrm.1 * r * 0.2);
                if d_at(site.0, site.1) <= r * 0.05 {
                    continue;
                }
                let bias = (sgn * nrm.0 * k * t_hi, sgn * nrm.1 * k * t_hi);
                let sink = rng.range(k_lo, k_hi) * if sm.crack { 1.0 } else { 0.7 };
                planes.push(cut(&mut rng, PlaneKind::Fracture, site, sink, t_lo * 0.5, bias));
            }
        }

        // lines as (points, cumulative length, groove parameters)
        struct Groove {
            pts: Vec<P>,
            cum: Vec<f32>,
            bb: (f32, f32, f32, f32),
            depth: f32,
            wide: (f32, f32),
            crevice: f32,
            ends: (bool, bool),
        }
        let grooves: Vec<Groove> = seams
            .iter()
            .filter(|s| s.crack)
            .map(|s| {
                let wide = (r * rng.range(0.12, 0.3), r * rng.range(0.04, 0.12));
                let wide = if rng.chance(0.5) { wide } else { (wide.1, wide.0) };
                let pad = wide.0.max(wide.1) + spec.crack_width * 2.0;
                let bb = s.pts.iter().fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| (b.0.min(p.0 - pad), b.1.min(p.1 - pad), b.2.max(p.0 + pad), b.3.max(p.1 + pad)));
                let e0 = s.pts[0];
                let e1 = *s.pts.last().unwrap();
                Groove {
                    cum: cumlen(&s.pts),
                    pts: s.pts.clone(),
                    bb,
                    depth: spec.crack_depth * r * rng.range(0.7, 1.3) * if s.grown { 0.8 } else { 1.0 },
                    wide,
                    crevice: spec.crack_width * rng.range(0.8, 1.2),
                    ends: (d_at(e0.0, e0.1) > r * 0.08, d_at(e1.0, e1.1) > r * 0.08),
                }
            })
            .collect();

        // bedding
        let bed = if spec.bed < 0.0 { (bh / 5.5).max(6.0) } else { spec.bed };
        let bed_y0 = bounds.1 - rng.range(0.0, 1.0) * bed;
        let mut beds = vec![0.0f32];
        if bed > 0.0 {
            let span = bh + bw * spec.bed_tilt.abs() + 4.0 * bed;
            while *beds.last().unwrap() < span {
                let t = bed * rng.range(0.55, 1.5);
                beds.push(beds.last().unwrap() + t);
            }
        }
        let bed_recess: Vec<f32> = (0..beds.len()).map(|_| rng.range(0.0, 1.0)).collect();
        let lumps = Fbm::new(seed as u32 ^ 0x51, 3, r * 0.9);
        let grain = Fbm::new(seed as u32 ^ 0x77, 3, (r * 0.08).max(3.0));
        let wav = Fbm::new(seed as u32 ^ 0x93, 2, r * 1.2);
        let flute = Fbm::new(seed as u32 ^ 0xa1, 3, 1.0);

        let bed_of = |x: f32, y: f32| -> (f32, usize) {
            let v = y - bed_y0 + (x - bounds.0) * spec.bed_tilt.tan() + wav.get(x, y) * bed * 0.18;
            let i = beds.partition_point(|b| *b <= v).max(1) - 1;
            let (b0, b1) = (beds[i], beds[(i + 1).min(beds.len() - 1)].max(beds[i] + 1e-3));
            ((v - b0) / (b1 - b0), i)
        };

        // heights
        let hs: Vec<(f32, u16, f32)> = (0..nx * ny)
            .into_par_iter()
            .map(|k| {
                if !inside[k] {
                    return (f32::NAN, 0, 0.0);
                }
                let (x, y) = at(k % nx, k / nx);
                let d = dist[k];
                let (mut h, id) = soft_min(&mut planes.iter().enumerate().map(|(i, p)| (i, p.z(x, y))));
                // where no plane cuts it, the mass itself: rounded, turning away at the silhouette
                {
                    let e = mass[k];
                    let lo = h.min(e);
                    let sum = (-(h - lo) / k_round).exp() + (-(e - lo) / k_round).exp();
                    h = lo - k_round * sum.ln();
                }
                let mut crack = 0.0f32;
                for g in &grooves {
                    if x < g.bb.0 || x > g.bb.2 || y < g.bb.1 || y > g.bb.3 {
                        continue;
                    }
                    let (dc, t, side, _) = near_line(&g.pts, &g.cum, x, y);
                    let taper = (if g.ends.0 { smoothstep(0.0, 0.25, t) } else { 1.0 }) * (if g.ends.1 { smoothstep(1.0, 0.75, t) } else { 1.0 });
                    let taper = taper.max(0.08);
                    let wv = if side > 0.0 { g.wide.0 } else { g.wide.1 };
                    let v = (1.0 - dc / wv).max(0.0);
                    let c = (1.0 - dc / g.crevice).max(0.0);
                    h -= taper * (g.depth * v + g.depth * 0.6 * c.powf(1.5));
                    crack = crack.max(c.powf(0.7) * taper.sqrt());
                }
                if bed > 0.0 {
                    let (u, i) = bed_of(x, y);
                    // each bed weathers back toward its base (an overhang
                    // over the joint below) and rounds at its top edge
                    let rec = spec.bed_recess * r * (0.2 * (1.0 - u).powi(8) + 0.6 * u.powf(2.5) + 0.7 * bed_recess[i]);
                    h -= rec * smoothstep(0.0, r * 0.2, d);
                    let joint = (1.0 - (u.min(1.0 - u) * (beds[(i + 1).min(beds.len() - 1)] - beds[i])) / (spec.crack_width * 0.8)).max(0.0);
                    crack = crack.max(joint * 0.8 * smoothstep(0.0, r * 0.1, d));
                }
                if spec.flutes > 0.0 {
                    // vertical rain flutes: stretched ridged noise
                    let per = (r * 0.16).max(6.0);
                    let f = (1.0 - flute.get(x / per, y / (per * 14.0)).abs()).powi(3);
                    h -= spec.flutes * r * f * smoothstep(0.0, r * 0.15, d);
                }
                h += spec.lumps * r * lumps.get(x, y) * smoothstep(0.0, r * 0.2, d);
                h += spec.grain * r * grain.get(x, y);
                (h.max(0.0), id as u16, crack)
            })
            .collect();
        let mut h: Vec<f32> = hs.iter().map(|v| v.0).collect();
        let facet: Vec<u16> = hs.iter().map(|v| v.1).collect();
        let crack: Vec<f32> = hs.iter().map(|v| v.2).collect();
        for (k, v) in h.iter_mut().enumerate() {
            if !inside[k] {
                *v = 0.0;
            }
        }

        // normals: central differences, one-sided at the edge (outside: z 0)
        let hz = |i: isize, j: isize| -> f32 {
            if i < 0 || j < 0 || i >= nx as isize || j >= ny as isize {
                return 0.0;
            }
            h[j as usize * nx + i as usize]
        };
        let n: Vec<V3> = (0..nx * ny)
            .into_par_iter()
            .map(|k| {
                let (i, j) = ((k % nx) as isize, (k / nx) as isize);
                let dx = (hz(i + 1, j) - hz(i - 1, j)) / (2.0 * step);
                let dy = (hz(i, j + 1) - hz(i, j - 1)) / (2.0 * step);
                unit([-dx, -dy, 1.0])
            })
            .collect();

        // occlusion: how far each point lies below its surroundings
        let blur = |src: &[f32], rad: usize| -> Vec<f32> {
            let mut a = src.to_vec();
            for _ in 0..2 {
                let mut b = vec![0.0; nx * ny];
                b.par_chunks_mut(nx).enumerate().for_each(|(j, row)| {
                    let mut acc = 0.0;
                    let mut cnt = 0.0;
                    for i in 0..(rad + 1).min(nx) {
                        acc += a[j * nx + i];
                        cnt += 1.0;
                    }
                    for (i, o) in row.iter_mut().enumerate() {
                        *o = acc / cnt;
                        if i + rad + 1 < nx {
                            acc += a[j * nx + i + rad + 1];
                            cnt += 1.0;
                        }
                        if i >= rad {
                            acc -= a[j * nx + i - rad];
                            cnt -= 1.0;
                        }
                    }
                });
                let mut c = vec![0.0; nx * ny];
                c.par_chunks_mut(nx).enumerate().for_each(|(j, row)| {
                    for (i, o) in row.iter_mut().enumerate() {
                        let (lo, hi) = (j.saturating_sub(rad), (j + rad).min(ny - 1));
                        let mut s = 0.0;
                        for jj in lo..=hi {
                            s += b[jj * nx + i];
                        }
                        *o = s / (hi - lo + 1) as f32;
                    }
                });
                a = c;
            }
            a
        };
        let r1 = ((r * 0.05 / step).round() as usize).max(2);
        let r2 = ((r * 0.18 / step).round() as usize).max(3);
        let (b1, b2) = (blur(&h, r1), blur(&h, r2));
        let ao: Vec<f32> = (0..nx * ny)
            .into_par_iter()
            .map(|k| {
                if !inside[k] {
                    return 0.0;
                }
                let c1 = (b1[k] - h[k]).max(0.0) / (r * 0.02);
                let c2 = (b2[k] - h[k] - r * 0.04).max(0.0) / (r * 0.08);
                (1.0 - (-(c1 + c2)).exp()).max(crack[k] * 0.7).min(1.0)
            })
            .collect();

        // the foot: the lower stretch of the outline, facing down
        let base = bounds.3;
        let foot: Vec<P> = (0..m)
            .filter(|&i| {
                let o = outward(i, w);
                o.1 > 0.3 && poly[i].1 > bounds.1 + bh * 0.55
            })
            .map(|i| poly[i])
            .collect();
        let tops: Vec<P> = (0..m).filter(|&i| outward(i, w).1 < -0.45).map(|i| poly[i]).collect();

        let mut rock = Rock {
            kind: spec.kind,
            outline: poly,
            seams,
            planes,
            light,
            r,
            bounds,
            base,
            foot,
            tops,
            bed,
            bed_tilt: spec.bed_tilt,
            bed_y0,
            beds,
            x0,
            y0,
            step,
            nx,
            ny,
            inside,
            h,
            n,
            facet,
            crack,
            ao,
            cast: Vec::new(),
            gx0: 0.0,
            gy0: 0.0,
            gstep: 1.0,
            gnx: 0,
            gny: 0,
            gcast: Vec::new(),
            seed,
        };
        rock.relight(light, spec.ground);
        rock
    }

    /// Light (or relight) the rock: cast shadows over itself and onto the
    /// ground (`ground`: z units per canvas unit down, the foreshortening of
    /// the plane it stands on).
    pub fn relight(&mut self, l: Light, ground: f32) {
        self.light = l;
        let (lx, ly, lz) = (l.dir[0], l.dir[1], l.dir[2]);
        let lxy = (lx * lx + ly * ly).sqrt();
        if lxy < 1e-3 {
            self.cast = vec![0.0; self.nx * self.ny];
            self.gcast.clear();
            return;
        }
        let (sx, sy) = (lx / lxy, ly / lxy);
        let rise = lz / lxy;
        let hmax = self.h.iter().cloned().fold(0.0f32, f32::max);
        let step = self.step;
        let pen = l.penumbra;
        // over the rock
        let cast: Vec<f32> = (0..self.nx * self.ny)
            .into_par_iter()
            .map(|k| {
                if !self.inside[k] {
                    return 0.0;
                }
                let (x, y) = (self.x0 + (k % self.nx) as f32 * step, self.y0 + (k / self.nx) as f32 * step);
                let z0 = self.h[k] + step * 0.5;
                self.trace(x, y, z0, sx, sy, rise, hmax, pen, false)
            })
            .collect();
        self.cast = cast;
        // over the ground: a coarser grid reaching away from the sun
        let b = self.bounds;
        let size = (b.2 - b.0).max(b.3 - b.1);
        let reach = if rise > 0.02 { (hmax / rise).min(size * 2.5) } else { size * 2.5 };
        let gstep = (self.step * 2.0).max(0.5);
        let gx0 = b.0.min(b.0 - sx * reach) - size * 0.05;
        let gx1 = b.2.max(b.2 - sx * reach) + size * 0.05;
        let gy0 = b.1;
        let gy1 = (b.3 - sy * reach).max(b.3) + size * 0.2;
        let gnx = ((gx1 - gx0) / gstep).ceil() as usize + 1;
        let gny = ((gy1 - gy0) / gstep).ceil() as usize + 1;
        let base = self.base;
        let gcast: Vec<f32> = (0..gnx * gny)
            .into_par_iter()
            .map(|k| {
                let (x, y) = (gx0 + (k % gnx) as f32 * gstep, gy0 + (k / gnx) as f32 * gstep);
                if self.inside_at(x, y) {
                    return 0.0;
                }
                let zg = (y - base) * ground;
                self.trace(x, y, zg + 0.3, sx, sy, rise, hmax, pen, true)
            })
            .collect();
        (self.gx0, self.gy0, self.gstep, self.gnx, self.gny, self.gcast) = (gx0, gy0, gstep, gnx, gny, gcast);
    }

    /// March toward the light from (x, y) at height z0; how deep in shadow.
    /// `ground`: the start is on the ground, and the rock is a slab from
    /// -h to +h (its back hides nothing in front of it).
    #[allow(clippy::too_many_arguments)]
    fn trace(&self, x: f32, y: f32, z0: f32, sx: f32, sy: f32, rise: f32, hmax: f32, pen: f32, ground: bool) -> f32 {
        let mut t = self.step * 1.5;
        let mut occl = 0.0f32;
        let far = (self.bounds.2 - self.bounds.0) + (self.bounds.3 - self.bounds.1) + 4.0 * hmax.max(1.0);
        while t < far * 2.0 {
            let ray = z0 + rise * t;
            if rise >= 0.0 && ray > hmax {
                break;
            }
            let (qx, qy) = (x + sx * t, y + sy * t);
            let (i, j) = ((qx - self.x0) / self.step, (qy - self.y0) / self.step);
            if (i < -1.0 && sx <= 0.0) || (j < -1.0 && sy <= 0.0) || (i > self.nx as f32 && sx >= 0.0) || (j > self.ny as f32 && sy >= 0.0) {
                break;
            }
            if i >= 0.0 && j >= 0.0 && (i as usize) < self.nx && (j as usize) < self.ny {
                let k = j as usize * self.nx + i as usize;
                if self.inside[k] {
                    let hq = self.h[k];
                    let hgt = hq - ray;
                    if !ground || ray > -hq {
                        let o = 0.5 + 0.5 * hgt / (pen * t + 0.3);
                        occl = occl.max(o);
                        if occl >= 1.0 {
                            break;
                        }
                    }
                }
            }
            t += self.step;
        }
        smoothstep(0.0, 1.0, occl)
    }

    fn inside_at(&self, x: f32, y: f32) -> bool {
        let (i, j) = (((x - self.x0) / self.step).round() as isize, ((y - self.y0) / self.step).round() as isize);
        i >= 0 && j >= 0 && (i as usize) < self.nx && (j as usize) < self.ny && self.inside[j as usize * self.nx + i as usize]
    }

    fn in_grid(&self, x: f32, y: f32) -> bool {
        x >= self.x0 && y >= self.y0 && x <= self.x0 + (self.nx - 1) as f32 * self.step && y <= self.y0 + (self.ny - 1) as f32 * self.step
    }

    /// What is at a point (None off the rock). Normals are interpolated
    /// between the grid's nodes (inside ones only, so the rim stays turned).
    pub fn sample(&self, x: f32, y: f32) -> Option<RockSample> {
        if !self.in_grid(x, y) {
            return None;
        }
        let fx = ((x - self.x0) / self.step).clamp(0.0, (self.nx - 1) as f32 - 1e-3);
        let fy = ((y - self.y0) / self.step).clamp(0.0, (self.ny - 1) as f32 - 1e-3);
        let (i, j) = (fx as usize, fy as usize);
        let (tx, ty) = (fx - i as f32, fy - j as f32);
        let mut acc = [0.0f32; 7];
        let mut wsum = 0.0;
        let mut best = (0.0f32, 0usize);
        for (a, b, wgt) in [(i, j, (1.0 - tx) * (1.0 - ty)), (i + 1, j, tx * (1.0 - ty)), (i, j + 1, (1.0 - tx) * ty), (i + 1, j + 1, tx * ty)] {
            let k = b * self.nx + a;
            if !self.inside[k] {
                continue;
            }
            let n = self.n[k];
            for (q, v) in [n[0], n[1], n[2], self.h[k], self.ao[k], self.crack[k], self.cast[k]].iter().enumerate() {
                acc[q] += v * wgt;
            }
            wsum += wgt;
            if wgt > best.0 {
                best = (wgt, k);
            }
        }
        if wsum <= 1e-6 {
            return None;
        }
        for v in acc.iter_mut() {
            *v /= wsum;
        }
        let n = unit([acc[0], acc[1], acc[2]]);
        let shade = self.light.shade(n, acc[6].clamp(0.0, 1.0));
        Some(RockSample { n, z: acc[3], plane: self.facet[best.1] as usize, shade, ao: acc[4], crack: acc[5] })
    }

    // ------------------------------------------------------------ masks

    fn over(&self, f: Frame, g: impl Fn(&RockSample) -> f32 + Sync) -> Mask {
        let sil = self.mask(f);
        let b = self.bounds;
        let mut m = Mask::from_fn(f, |x, y| {
            if x < b.0 - 1.0 || x > b.2 + 1.0 || y < b.1 - 1.0 || y > b.3 + 1.0 {
                return 0.0;
            }
            self.sample(x, y).map_or(0.0, |s| g(&s))
        });
        for (v, s) in m.data.iter_mut().zip(&sil.data) {
            *v *= s;
        }
        m
    }

    /// The silhouette (antialiased; exactly the drawn outline).
    pub fn mask(&self, f: Frame) -> Mask {
        Mask::from_shape(f, Shape::new().poly(&self.outline))
    }
    /// The light family (direct light over `soft`'s halftone), less the cracks.
    pub fn lit(&self, f: Frame, soft: f32) -> Mask {
        self.over(f, |s| s.shade.lit(soft) * (1.0 - 0.8 * s.crack))
    }
    pub fn shadow(&self, f: Frame, soft: f32) -> Mask {
        self.over(f, |s| 1.0 - s.shade.lit(soft) * (1.0 - 0.8 * s.crack))
    }
    /// The halftone: planes turned half toward the light.
    pub fn halftone(&self, f: Frame) -> Mask {
        self.over(f, |s| {
            let t = s.shade.direct;
            smoothstep(0.0, 0.18, t) * smoothstep(0.62, 0.3, t)
        })
    }
    pub fn core(&self, f: Frame) -> Mask {
        self.over(f, |s| s.core())
    }
    pub fn reflected(&self, f: Frame) -> Mask {
        self.over(f, |s| s.reflected())
    }
    /// The light as one value, 0..1 (sun, sky, reflected light, occlusion).
    pub fn value(&self, f: Frame) -> Mask {
        self.over(f, |s| s.value())
    }
    pub fn occlusion(&self, f: Frame) -> Mask {
        self.over(f, |s| s.ao)
    }
    pub fn cracks(&self, f: Frame) -> Mask {
        self.over(f, |s| s.crack)
    }
    /// Faces turned up to the sky: 0 below `lo` of "up", 1 above `hi`.
    pub fn up(&self, f: Frame, lo: f32, hi: f32) -> Mask {
        self.over(f, |s| smoothstep(lo, hi, s.up()))
    }
    /// One plane (index into `planes`).
    pub fn plane(&self, f: Frame, i: usize) -> Mask {
        self.over(f, |s| if s.plane == i { 1.0 } else { 0.0 })
    }
    /// Arrises: where the planes break convexly (catch a light edge).
    pub fn arrises(&self, f: Frame, span: f32) -> Mask {
        self.over(f, |_| 1.0).mul(&Mask::from_fn(f, |x, y| {
            let b = self.bounds;
            if x < b.0 || x > b.2 || y < b.1 || y > b.3 {
                return 0.0;
            }
            smoothstep(0.25, 0.7, self.bend(x, y, span))
        }))
    }

    /// How the surface bends over `span` units: + convex, - concave.
    pub fn bend(&self, x: f32, y: f32, span: f32) -> f32 {
        let mut best = 0.0f32;
        for (dx, dy) in [(span, 0.0), (0.0, span)] {
            if let (Some(a), Some(b)) = (self.sample(x - dx, y - dy), self.sample(x + dx, y + dy)) {
                let p = [2.0 * dx, 2.0 * dy, b.z - a.z];
                let lp = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt().max(1e-6);
                let k = (p[0] * (b.n[0] - a.n[0]) + p[1] * (b.n[1] - a.n[1]) + p[2] * (b.n[2] - a.n[2])) / lp;
                if k.abs() > best.abs() {
                    best = k;
                }
            }
        }
        best
    }

    /// Where the rock meets the ground: inside, the dark seam along its foot
    /// (`inside` units deep); outside, the contact shadow on the ground
    /// (`outside` units).
    pub fn contact(&self, f: Frame, inside: f32, outside: f32) -> Mask {
        let b = self.bounds;
        let foot: Vec<P> = self.foot.iter().step_by((self.foot.len() / 120).max(1)).copied().collect();
        if foot.is_empty() {
            return Mask::empty(f);
        }
        let reach = inside.max(outside) * 3.0;
        let sil = self.mask(f);
        let mut m = Mask::from_fn(f, |x, y| {
            if x < b.0 - reach || x > b.2 + reach || y < b.1 || y > b.3 + reach {
                return 0.0;
            }
            let d = foot.iter().map(|p| ((p.0 - x).powi(2) + (p.1 - y).powi(2)).sqrt()).fold(f32::MAX, f32::min);
            if d > reach {
                return 0.0;
            }
            (-(d / inside.max(0.1))).exp()
        });
        let mo = Mask::from_fn(f, |x, y| {
            if x < b.0 - reach || x > b.2 + reach || y < b.1 || y > b.3 + reach {
                return 0.0;
            }
            // below the foot: the ground close under the rock
            let d = foot.iter().filter(|p| y >= p.1 - 1.0).map(|p| ((p.0 - x).powi(2) + ((p.1 - y) * 1.6).powi(2)).sqrt()).fold(f32::MAX, f32::min);
            if d > reach { 0.0 } else { (-(d / outside.max(0.1))).exp() }
        });
        for ((v, s), o) in m.data.iter_mut().zip(&sil.data).zip(&mo.data) {
            *v = *v * s + o * (1.0 - s);
        }
        m
    }

    /// The rock's shadow on the ground around it (0 on the rock itself).
    pub fn cast(&self, f: Frame) -> Mask {
        if self.gcast.is_empty() {
            return Mask::empty(f);
        }
        let sil = self.mask(f);
        let (gx1, gy1) = (self.gx0 + (self.gnx - 1) as f32 * self.gstep, self.gy0 + (self.gny - 1) as f32 * self.gstep);
        let mut m = Mask::from_fn(f, |x, y| {
            if x < self.gx0 || y < self.gy0 || x > gx1 || y > gy1 {
                return 0.0;
            }
            let fx = ((x - self.gx0) / self.gstep).min((self.gnx - 1) as f32 - 1e-3);
            let fy = ((y - self.gy0) / self.gstep).min((self.gny - 1) as f32 - 1e-3);
            let (i, j) = (fx as usize, fy as usize);
            let (tx, ty) = (fx - i as f32, fy - j as f32);
            let g = |a: usize, b: usize| self.gcast[b * self.gnx + a];
            (g(i, j) * (1.0 - tx) + g(i + 1, j) * tx) * (1.0 - ty) + (g(i, j + 1) * (1.0 - tx) + g(i + 1, j + 1) * tx) * ty
        });
        for (v, s) in m.data.iter_mut().zip(&sil.data) {
            *v *= 1.0 - s;
        }
        m
    }

    /// Snow lying on the faces turned up (`amount` 0..1: how much of them),
    /// broken by a noise, plus a cap `depth` units thick standing above the
    /// upper edges of the silhouette (snow breaks the outline).
    pub fn snow(&self, f: Frame, amount: f32, depth: f32, seed: u32) -> Mask {
        let nz = Fbm::new(seed ^ (self.seed as u32), 4, (self.r * 0.35).max(6.0));
        let fine = Fbm::new(seed ^ 0x3c ^ (self.seed as u32), 3, (self.r * 0.07).max(2.5));
        let lo = 0.62 - 0.5 * amount;
        let b = self.bounds;
        let sil = self.mask(f);
        let tops: Vec<P> = self.tops.iter().step_by((self.tops.len() / 150).max(1)).copied().collect();
        let on = Mask::from_fn(f, |x, y| {
            if x < b.0 - depth * 2.0 || x > b.2 + depth * 2.0 || y < b.1 - depth * 2.0 || y > b.3 {
                return 0.0;
            }
            let brk = 0.25 * nz.get(x, y) + 0.12 * fine.get(x, y);
            match self.sample(x, y) {
                Some(s) if self.inside_at(x, y) => {
                    let up = s.up() + brk - 0.4 * s.crack;
                    smoothstep(lo, lo + 0.12, up)
                }
                _ => {
                    // above an upper edge: the cap
                    let mut best = (f32::MAX, 0.0);
                    for p in &tops {
                        let d = ((p.0 - x).powi(2) + (p.1 - y).powi(2)).sqrt();
                        if d < best.0 {
                            best = (d, p.1 - y);
                        }
                    }
                    if best.0 > depth * 2.0 || best.1 < -0.5 {
                        return 0.0;
                    }
                    let th = depth * (0.55 + 1.2 * brk + 0.5 * amount).max(0.0);
                    smoothstep(th, th * 0.6, best.0)
                }
            }
        });
        let mut m = on;
        for (v, s) in m.data.iter_mut().zip(&sil.data) {
            // antialias the faces' snow at the silhouette; the cap is outside it
            *v = v.clamp(0.0, 1.0) * if *s > 0.0 && *s < 1.0 { s.max(*v) } else { 1.0 };
        }
        m
    }

    // ------------------------------------------------------------ fields

    /// Down the plane at a point (the way water runs), a canvas angle.
    pub fn fall(&self, x: f32, y: f32) -> f32 {
        self.sample(x, y).map_or(PI * 0.5, |s| fall_angle(s.n))
    }
    /// Around the form.
    pub fn across(&self, x: f32, y: f32) -> f32 {
        self.fall(x, y) - PI * 0.5
    }
    /// Down the plane the point belongs to: one direction per plane, so
    /// strokes change direction where the planes break.
    pub fn plane_fall(&self, x: f32, y: f32) -> f32 {
        self.sample(x, y).map_or(PI * 0.5, |s| {
            let n = self.planes[s.plane].n;
            if n[0].abs() + n[1].abs() < 0.08 { PI * 0.5 } else { fall_angle(n) }
        })
    }
    /// Along the nearest drawn or grown line (or the bedding, if nearer).
    pub fn along_crack(&self, x: f32, y: f32) -> f32 {
        let mut best = (f32::MAX, 0.0f32);
        for s in &self.seams {
            let c = cumlen(&s.pts);
            let (d, _, _, dir) = near_line(&s.pts, &c, x, y);
            if d < best.0 {
                best = (d, dir.1.atan2(dir.0));
            }
        }
        if self.bed > 0.0 {
            let v = y - self.bed_y0 + (x - self.bounds.0) * self.bed_tilt.tan();
            let i = self.beds.partition_point(|b| *b <= v).max(1) - 1;
            let d = (v - self.beds[i]).min(self.beds[(i + 1).min(self.beds.len() - 1)] - v).abs();
            if d < best.0 {
                best = (d, self.bed_tilt);
            }
        }
        best.1
    }
    /// Along the bedding.
    pub fn bedding(&self) -> f32 {
        self.bed_tilt
    }

    /// Cells of the grid in each plane: (plane index, cells), largest first.
    pub fn plane_areas(&self) -> Vec<(usize, usize)> {
        let mut c = vec![0usize; self.planes.len()];
        for (k, f) in self.facet.iter().enumerate() {
            if self.inside[k] {
                c[*f as usize] += 1;
            }
        }
        let mut v: Vec<(usize, usize)> = c.into_iter().enumerate().filter(|(_, n)| *n > 0).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v
    }

    /// The share of the rock (by grid cells) in the light family.
    pub fn lit_share(&self) -> f32 {
        let (mut lit, mut all) = (0.0f32, 0.0f32);
        for k in 0..self.nx * self.ny {
            if self.inside[k] {
                all += 1.0;
                lit += self.light.shade(self.n[k], self.cast[k]).lit(0.12);
            }
        }
        lit / all.max(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boulder() -> Vec<P> {
        // a lumpy boulder about 300 x 200, drawn clockwise on screen
        let mut v = Vec::new();
        let key = [(100.0, 400.0), (120.0, 300.0), (190.0, 230.0), (280.0, 215.0), (360.0, 250.0), (400.0, 330.0), (395.0, 400.0)];
        for w in key.windows(2) {
            for k in 0..10 {
                let t = k as f32 / 10.0;
                v.push((w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t));
            }
        }
        for k in 0..20 {
            let t = k as f32 / 20.0;
            v.push((395.0 + (100.0 - 395.0) * t, 400.0 + 6.0 * (t * PI).sin()));
        }
        v
    }

    fn sun() -> Light {
        Light::new((-1.0, -0.6), 0.45)
    }

    #[test]
    fn many_planes_and_no_single_seam() {
        let r = Rock::grow(&boulder(), &[], &[], &[], &RockSpec::granite(), sun(), 3);
        let big = r.plane_areas().iter().filter(|(_, n)| *n > 400).count();
        assert!(big >= 6, "only {big} sizable planes");
        // along a horizontal line through the middle, the light family
        // changes more than once (the terminator steps along plane breaks),
        // and neither family has the whole rock
        let share = r.lit_share();
        assert!(share > 0.25 && share < 0.9, "lit share {share}");
        let y = 320.0;
        let mut flips = 0;
        let mut last: Option<bool> = None;
        let mut x = 105.0;
        while x < 395.0 {
            if let Some(s) = r.sample(x, y) {
                let l = s.shade.lit(0.12) > 0.5;
                if last.is_some_and(|p| p != l) {
                    flips += 1;
                }
                last = Some(l);
            }
            x += 1.0;
        }
        assert!(flips >= 1, "no terminator across the middle");
        // the upper faces look up; the lower ones don't
        let top = r.sample(280.0, 235.0).unwrap();
        let bottom = r.sample(250.0, 395.0).unwrap();
        assert!(top.up() > 0.3 && bottom.up() < 0.1, "{} {}", top.up(), bottom.up());
    }

    #[test]
    fn a_crack_is_a_dark_groove() {
        let crack = vec![(230.0, 225.0), (240.0, 280.0), (232.0, 340.0)];
        let r = Rock::grow(&boulder(), &[], &[crack], &[], &RockSpec::granite(), sun(), 5);
        let on = r.sample(240.0, 280.0).unwrap();
        let off = r.sample(275.0, 280.0).unwrap();
        assert!(on.crack > 0.5 && off.crack < 0.2, "{} {}", on.crack, off.crack);
        assert!(on.ao > off.ao + 0.2, "{} {}", on.ao, off.ao);
        assert!(on.value() < off.value(), "{} {}", on.value(), off.value());
    }

    #[test]
    fn deterministic_and_kinds_differ() {
        let a = Rock::grow(&boulder(), &[], &[], &[], &RockSpec::sandstone(), sun(), 7);
        let b = Rock::grow(&boulder(), &[], &[], &[], &RockSpec::sandstone(), sun(), 7);
        assert_eq!(a.h, b.h);
        let g = Rock::grow(&boulder(), &[], &[], &[], &RockSpec::granite(), sun(), 7);
        assert_ne!(a.h, g.h);
        // sandstone is bedded: many crack cells in horizontal runs
        let beds = a.crack.iter().filter(|c| **c > 0.5).count();
        assert!(beds > 200, "{beds}");
    }

    #[test]
    fn it_casts_a_shadow_on_the_ground() {
        // a low sun from the left: the shadow lies to the right of the foot
        let r = Rock::grow(&boulder(), &[], &[], &[], &RockSpec::granite(), Light::new((-1.0, -0.25), 0.1), 3);
        let f = Frame { w: 600, h: 500, scale: 1.0, x0: 0, y0: 0, full_w: 600, full_h: 500 };
        let c = r.cast(f);
        let right = c.data[395 * 600 + 430];
        let left = c.data[395 * 600 + 70];
        assert!(right > 0.5 && left < 0.1, "{right} {left}");
        let s = r.snow(f, 0.6, 4.0, 1);
        assert!(s.data[240 * 600 + 280] > 0.3 || s.data[222 * 600 + 280] > 0.3);
    }
}

#[cfg(test)]
mod look {
    use super::*;

    /// A grisaille of a rock for looking (writes ROCK_LOOK/rock_look.ppm):
    /// `ROCK_LOOK=dir cargo test -p paint --lib rock::look -- --ignored`.
    #[test]
    #[ignore]
    fn grisaille() {
        let Ok(dir) = std::env::var("ROCK_LOOK") else { return };
        let (w, h) = (1200usize, 500usize);
        let f = Frame { w, h, scale: 1.0, x0: 0, y0: 0, full_w: w, full_h: h };
        // an erratic, a sandstone ledge, a chalk stack
        let blob = |cx: f32, cy: f32, rx: f32, ry: f32, seed: u64| -> Vec<P> {
            let mut rng = Rng::new(seed);
            let k = 11;
            let mut key: Vec<P> = (0..k).map(|i| {
                let a = PI + i as f32 / k as f32 * 2.0 * PI;
                let rr = rng.range(0.8, 1.1);
                (cx + rx * rr * a.cos(), (cy + ry * rr * a.sin()).min(cy + ry * 0.55))
            }).collect();
            key.push(key[0]);
            let mut v = Vec::new();
            for wn in key.windows(2) {
                for q in 0..12 {
                    let t = q as f32 / 12.0;
                    v.push((wn[0].0 + (wn[1].0 - wn[0].0) * t, wn[0].1 + (wn[1].1 - wn[0].1) * t));
                }
            }
            v
        };
        let sun = Light::new((-1.0, -0.55), 0.4).ambient(0.2).bounce(0.3, [0.45, 0.8, 0.3]);
        let rocks = [
            Rock::grow(&blob(200.0, 280.0, 170.0, 150.0, 1), &[], &[vec![(210.0, 150.0), (225.0, 250.0), (205.0, 340.0)]], &[], &RockSpec::granite(), sun, 3),
            Rock::grow(&blob(600.0, 280.0, 190.0, 130.0, 2), &[], &[], &[], &RockSpec::sandstone(), sun, 4),
            Rock::grow(&blob(1000.0, 260.0, 150.0, 170.0, 3), &[], &[], &[], &RockSpec::chalk(), sun, 5),
        ];
        let mut img = vec![[90u8, 100, 110]; w * h];
        for r in &rocks {
            let cast = r.cast(f);
            for (i, c) in cast.data.iter().enumerate() {
                if *c > 0.0 {
                    let p = &mut img[i];
                    for q in 0..3 {
                        p[q] = (p[q] as f32 * (1.0 - 0.5 * c)) as u8;
                    }
                }
            }
            let sil = r.mask(f);
            let v = r.value(f);
            let core = r.core(f);
            let refl = r.reflected(f);
            for i in 0..w * h {
                if sil.data[i] > 0.0 {
                    let g = (v.data[i] * 235.0 + 10.0) * sil.data[i] + img[i][0] as f32 * (1.0 - sil.data[i]);
                    // reflected light warm, core shadow cool (for looking only)
                    img[i] = [(g + 25.0 * refl.data[i]).min(255.0) as u8, g as u8, (g + 20.0 * core.data[i]).min(255.0) as u8];
                }
            }
            println!("{:?} planes {} lit {:.2} r {:.0}", r.kind, r.planes.len(), r.lit_share(), r.r);
        }
        let mut out = format!("P6 {w} {h} 255\n").into_bytes();
        for p in &img {
            out.extend_from_slice(p);
        }
        std::fs::write(format!("{dir}/rock_look.ppm"), out).unwrap();
    }
}
