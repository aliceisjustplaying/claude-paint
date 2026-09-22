//! Simulated bristle brushes working in wet paint.
//!
//! A `Tool` is a physical brush (round sable, hog flat, filbert, fan, rigger,
//! badger blender). `Held` is that brush in the hand: every bristle has its
//! own little reservoir of paint (volume + Mixbox pigment mix + hiding).
//!
//! As the handle moves along a `Gesture`:
//! - bristles splay with pressure and their tips trail behind the motion,
//!   lagging on turns (a first-order bend model, after the verlet bristles in
//!   dli/paint: https://github.com/dli/paint);
//! - a bristle touches the canvas only where its reach clears the canvas tooth
//!   and the paint relief, so light pressure catches only the weave's peaks
//!   (dry-brush);
//! - each bristle swept capsule exchanges paint with the wet layer: it
//!   deposits a share of its load and picks up wet paint, so brushes get dirty,
//!   colors smear and mix on the canvas, and a clean brush blends;
//! - moving bristles plough wet paint aside and ahead, so ridges and ends of
//!   strokes build up by themselves.

use crate::path::densify;
use crate::canvas::Canvas;
use crate::mask::Mask;
use crate::rng::Rng;
use crate::smoothstep;
use crate::wet::{LAT, Latent, Paint, Prop, mix_into};

/// Relief (µm) that spans a bristle's contact range: a bristle pressed
/// lightly touches only peaks this much above their surroundings.
const TOOTH_UM: f32 = 60.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Round,
    Flat,
    Filbert,
    Fan,
    Rigger,
    Blender,
}

#[derive(Clone, Debug)]
pub struct Tool {
    pub kind: Kind,
    /// Footprint width at full pressure (units).
    pub width: f32,
    pub bristles: usize,
    /// How far tips trail behind the contact at full pressure (units).
    pub length: f32,
    /// 0 = soft hair (sable, badger) .. 1 = stiff hog bristle.
    pub stiffness: f32,
    /// Bristle thickness multiplier (overlap between bristles).
    pub hair: f32,
    /// Stroke length (units) over which a load runs down (e-folding).
    pub run: f32,
    /// Paint thickness laid at the start of a fully loaded stroke.
    pub lay: f32,
    /// Fraction of wet paint a bristle takes up per pass.
    pub pickup: f32,
    /// Fraction of wet paint a moving bristle ploughs aside per pass.
    pub push: f32,
    /// Footprint growth under pressure.
    pub splay: f32,
    /// Unevenness of bristle lengths (ragged edges, broken marks).
    pub ragged: f32,
}

impl Tool {
    fn base(kind: Kind, width: f32) -> Self {
        Tool {
            kind,
            width,
            bristles: 80,
            length: width * 0.7,
            stiffness: 0.4,
            hair: 1.3,
            run: 40.0 + 6.0 * width,
            lay: 1.0,
            pickup: 0.12,
            push: 0.1,
            splay: 0.3,
            ragged: 0.25,
        }
    }

    /// Soft pointed round: smooth, precise, little ploughing. Friedrich's detail brush.
    pub fn round_sable(width: f32) -> Self {
        Tool { stiffness: 0.2, pickup: 0.1, push: 0.05, splay: 0.45, ragged: 0.15, ..Self::base(Kind::Round, width) }
    }

    /// Stiff hog-bristle flat: square marks, strong ridges, broken edges.
    pub fn hog_flat(width: f32) -> Self {
        Tool {
            bristles: 140,
            length: width * 0.5,
            stiffness: 0.85,
            hair: 1.1,
            run: 25.0 + 4.0 * width,
            lay: 1.4,
            pickup: 0.2,
            push: 0.3,
            splay: 0.12,
            ragged: 0.45,
            ..Self::base(Kind::Flat, width)
        }
    }

    /// Filbert: oval hog/synthetic, soft-ended marks, good for blending forms.
    pub fn filbert(width: f32) -> Self {
        Tool { bristles: 120, stiffness: 0.6, lay: 1.2, pickup: 0.18, push: 0.18, ragged: 0.3, ..Self::base(Kind::Filbert, width) }
    }

    /// Fan: sparse spread bristles, for foliage, grasses and feathering.
    pub fn fan(width: f32) -> Self {
        Tool { bristles: 36, hair: 0.6, stiffness: 0.5, lay: 0.7, pickup: 0.1, push: 0.08, splay: 0.2, ragged: 0.5, ..Self::base(Kind::Fan, width) }
    }

    /// Rigger / liner: a few very long soft hairs. Twigs, rigging, fine lines.
    pub fn rigger(width: f32) -> Self {
        Tool {
            bristles: 14,
            length: width * 5.0,
            stiffness: 0.08,
            hair: 1.4,
            run: 250.0,
            lay: 1.0,
            pickup: 0.04,
            push: 0.02,
            splay: 0.6,
            ragged: 0.1,
            ..Self::base(Kind::Rigger, width)
        }
    }

    /// Badger blender: big, soft, used clean to fuse wet paint.
    pub fn badger(width: f32) -> Self {
        Tool {
            bristles: 160,
            length: width * 0.6,
            stiffness: 0.15,
            lay: 0.0,
            run: 30.0,
            pickup: 0.3,
            push: 0.04,
            splay: 0.5,
            ragged: 0.2,
            ..Self::base(Kind::Blender, width)
        }
    }

    /// Bristle radius in units.
    pub(crate) fn hair_radius(&self) -> f32 {
        let across = match self.kind {
            Kind::Flat => 0.36,
            Kind::Filbert => 0.6,
            Kind::Fan => 0.12,
            _ => 1.0,
        };
        // area the bristles share, divided among them
        let area = self.width * 0.5 * self.width * 0.5 * across * std::f32::consts::PI;
        (area / (self.bristles as f32 * std::f32::consts::PI)).sqrt() * self.hair
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Bristle {
    /// Root offset in the brush frame (x along the wide axis), roughly −1..1.
    rx: f32,
    ry: f32,
    len: f32,
    /// Pressure needed before this bristle touches.
    thresh: f32,
    bend: (f32, f32),
    seed: u64,
    prev: [Option<(f32, f32)>; 2],
    pub(crate) vol: f32,
    lat: Latent,
    hide: Prop,
}

/// A brush in the hand, with paint in its bristles.
pub struct Held {
    pub tool: Tool,
    pub(crate) bristles: Vec<Bristle>,
}

impl Held {
    pub fn new(tool: Tool, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let n = tool.bristles.max(1);
        let golden = std::f32::consts::PI * (3.0 - 5f32.sqrt());
        let bristles = (0..n)
            .map(|i| {
                let t = (i as f32 + 0.5) / n as f32;
                let (rx, ry) = match tool.kind {
                    Kind::Round | Kind::Rigger | Kind::Blender => {
                        let r = t.sqrt();
                        let a = i as f32 * golden;
                        (r * a.cos(), r * a.sin())
                    }
                    Kind::Filbert => {
                        let r = t.sqrt();
                        let a = i as f32 * golden;
                        (r * a.cos(), r * a.sin() * 0.3)
                    }
                    Kind::Flat => (rng.range(-1.0, 1.0), rng.range(-0.18, 0.18)),
                    Kind::Fan => {
                        let a = rng.range(-0.95, 0.95);
                        (a.sin(), (a.cos() - 1.0) * 0.35 + rng.range(-0.04, 0.04))
                    }
                };
                // jitter roots a little so bristles don't sit on a perfect lattice
                let (rx, ry) = ((rx + rng.normal() * 0.02).clamp(-ROOT_MAX, ROOT_MAX), (ry + rng.normal() * 0.02).clamp(-ROOT_MAX, ROOT_MAX));
                Bristle {
                    rx,
                    ry,
                    len: (1.0 + rng.normal() * 0.15 * tool.ragged).clamp(0.4, LEN_MAX),
                    // a round brush is shaped to a point: the outer hairs are
                    // shorter and touch only under pressure, so a light touch
                    // or a lift-off gives just the tip
                    thresh: tool.ragged * rng.f().powf(1.5) * 0.55
                        + match tool.kind {
                            Kind::Round | Kind::Rigger => 0.6 * (rx * rx + ry * ry),
                            Kind::Filbert => 0.3 * (rx * rx + ry * ry / 0.09),
                            _ => 0.0,
                        },
                    bend: (0.0, 0.0),
                    seed: i as u64 * 7919 + seed,
                    prev: [None, None],
                    vol: 0.0,
                    lat: [0.0; LAT],
                    hide: [0.5, 0.5],
                }
            })
            .collect();
        Held { tool, bristles }
    }

    /// A full load's volume for one bristle.
    pub(crate) fn full(&self) -> f32 {
        // neighbouring bristles overlap by ~hair², so each lays lay / hair²
        let track = 2.0 * self.tool.hair_radius();
        self.tool.lay.max(0.3) * track * self.tool.run / (self.tool.hair * self.tool.hair)
    }

    /// Dip the brush: mix `amount` (0..1 of a full load) of `paint` into
    /// every bristle's reservoir. Bristles hold a little more or less.
    pub fn load(&mut self, paint: Paint, amount: f32) {
        let lat = paint.latent();
        let full = self.full();
        for (i, b) in self.bristles.iter_mut().enumerate() {
            let k = 0.75 + 0.5 * crate::rng::hash2(i as i64, 17, 3);
            mix_into(&mut b.vol, &mut b.lat, &mut b.hide, amount * paint.body * full * k, &lat, [paint.hiding, paint.body]);
        }
    }

    /// Wipe the brush on a rag: remove `frac` of the paint in it.
    pub fn wipe(&mut self, frac: f32) {
        for b in &mut self.bristles {
            b.vol *= 1.0 - frac.clamp(0.0, 1.0);
        }
    }

    /// Wipe thoroughly and load fresh paint.
    pub fn reload(&mut self, paint: Paint, amount: f32) {
        self.wipe(0.85);
        self.load(paint, amount);
    }

    /// Paint left in the brush, relative to a full load.
    pub fn fullness(&self) -> f32 {
        let f = self.full();
        self.bristles.iter().map(|b| b.vol).sum::<f32>() / (f * self.bristles.len() as f32)
    }
}

/// How the brush's wide axis is held.
#[derive(Clone, Copy, Debug)]
pub enum Orient {
    /// Wide axis across the direction of travel (full-width marks).
    Across,
    /// Wide axis along the travel (thin edge marks).
    Along,
    /// Fixed angle in radians, regardless of travel (e.g. Cézanne hatching).
    Fixed(f32),
}

/// One movement of the hand.
#[derive(Clone, Debug)]
pub struct Gesture {
    pub pts: Vec<(f32, f32)>,
    /// Pressure at the start and end (0..1).
    pub pressure: (f32, f32),
    pub orient: Orient,
    /// Fraction of the stroke spent pressing down / lifting off.
    pub attack: f32,
    pub release: f32,
    /// Hand unsteadiness: 1 = a normal hand, 0 = mechanically exact.
    pub shake: f32,
}

impl Gesture {
    pub fn new(pts: Vec<(f32, f32)>) -> Self {
        Gesture { pts, pressure: (0.8, 0.8), orient: Orient::Across, attack: 0.08, release: 0.15, shake: 1.0 }
    }
    pub fn line(a: (f32, f32), b: (f32, f32)) -> Self {
        Self::new(vec![a, b])
    }
    /// Scale hand unsteadiness (0 = exact, 1 = normal).
    pub fn shake(mut self, k: f32) -> Self {
        self.shake = k;
        self
    }
    pub fn pressure(mut self, a: f32, b: f32) -> Self {
        self.pressure = (a, b);
        self
    }
    pub fn orient(mut self, o: Orient) -> Self {
        self.orient = o;
        self
    }
    pub fn ramps(mut self, attack: f32, release: f32) -> Self {
        self.attack = attack;
        self.release = release;
        self
    }
    fn pressure_at(&self, u: f32) -> f32 {
        let base = self.pressure.0 + (self.pressure.1 - self.pressure.0) * u;
        let a = if self.attack > 0.0 { 0.08 + 0.92 * smoothstep(0.0, self.attack, u) } else { 1.0 };
        let r = if self.release > 0.0 { 0.05 + 0.95 * smoothstep(0.0, self.release, 1.0 - u) } else { 1.0 };
        base * a * r
    }
}

/// Raw view of the wet layer and surface, so brushes working disjoint parts
/// of the canvas can run in parallel (see `Canvas::work`).
#[derive(Clone, Copy)]
pub(crate) struct Surf {
    w: usize,
    h: usize,
    scale: f32,
    vol: *mut f32,
    lat: *mut Latent,
    hide: *mut Prop,
    stroke: *mut u32,
    touched: *mut u32,
    floor: *mut f32,
    base: *const f32,
}
// SAFETY: callers only run brushes concurrently on pixel sets that cannot
// overlap (tiles separated by more than the largest stroke extent).
unsafe impl Send for Surf {}
unsafe impl Sync for Surf {}

/// Dirty pixel bounds collected by a drag.
pub(crate) type Bounds = Option<(usize, usize, usize, usize)>;

fn grow(b: &mut Bounds, x0: usize, y0: usize, x1: usize, y1: usize) {
    *b = Some(match *b {
        None => (x0, y0, x1, y1),
        Some((a, c, d, e)) => (a.min(x0), c.min(y0), d.max(x1), e.max(y1)),
    });
}

impl Surf {
    #[inline]
    unsafe fn add(&self, i: usize, v: f32, lat: &Latent, hide: Prop) {
        unsafe {
            if v <= 0.0 {
                return;
            }
            let vol = &mut *self.vol.add(i);
            let l = &mut *self.lat.add(i);
            let hd = &mut *self.hide.add(i);
            let t = *vol + v;
            let a = v / t;
            for k in 0..LAT {
                l[k] += (lat[k] - l[k]) * a;
            }
            hd[0] += (hide[0] - hd[0]) * a;
            hd[1] += (hide[1] - hd[1]) * a;
            *vol = t;
        }
    }
}

impl Canvas {
    pub(crate) fn surf(&mut self) -> Surf {
        // the raw views below index 0..w*h: every buffer must be that long
        let n = self.f.w * self.f.h;
        let wt = &self.wet;
        assert!(
            [self.height.len(), self.px.len(), self.film.len(), wt.vol.len(), wt.lat.len(), wt.hide.len(), wt.stroke.len(), wt.touched.len(), wt.floor.len()].iter().all(|&l| l == n),
            "canvas buffers out of sync with frame"
        );
        if self.base.as_ref().map(|b| b.0) != Some(self.surf_gen) {
            let mut base = self.base.take().map(|b| b.1).unwrap_or_default();
            base.resize(self.height.len(), 0.0);
            use rayon::prelude::*;
            // a brush rests on local peaks: only relief relative to the
            // surroundings (within ~1.5 mm) matters. ±TOOTH_UM of relief spans
            // the whole contact range.
            let r = ((1.5 / self.px_mm()).round() as usize).max(1);
            let (w, h) = (self.f.w, self.f.h);
            let low = crate::surface::box_blur(&crate::surface::box_blur(&self.height, w, h, r), w, h, r);
            base.par_iter_mut().zip(self.height.par_iter().zip(low.par_iter())).for_each(|(b, (&hgt, &lo))| {
                *b = (0.5 + (hgt - lo) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3);
            });
            self.base = Some((self.surf_gen, base));
        }
        Surf {
            w: self.f.w,
            h: self.f.h,
            scale: self.f.scale,
            vol: self.wet.vol.as_mut_ptr(),
            lat: self.wet.lat.as_mut_ptr(),
            hide: self.wet.hide.as_mut_ptr(),
            stroke: self.wet.stroke.as_mut_ptr(),
            touched: self.wet.touched.as_mut_ptr(),
            floor: self.wet.floor.as_mut_ptr(),
            base: self.base.as_ref().unwrap().1.as_ptr(),
        }
    }

    pub(crate) fn next_stroke_ids(&mut self, n: u32) -> u32 {
        let first = self.wet.current.wrapping_add(1).max(1);
        self.wet.current = first.wrapping_add(n);
        first
    }

    /// Drag a held brush through a gesture, working the wet paint.
    pub fn drag(&mut self, held: &mut Held, g: &Gesture, clip: Option<&Mask>) {
        if let Some(m) = clip {
            self.check_mask(m);
        }
        let id = self.next_stroke_ids(1);
        let surf = self.surf();
        let mut scratch = Vec::new();
        // SAFETY: exclusive &mut self, single brush.
        let b = unsafe { drag_on(surf, held, g, clip, id, &mut scratch) };
        if let Some((x0, y0, x1, y1)) = b {
            self.wet.touch(x0, y0, x1, y1);
        }
    }
}

/// Bristle roots never sit farther than this (in half-widths) from the axis.
const ROOT_MAX: f32 = 1.2;
/// Longest bristle relative to the tool's length.
const LEN_MAX: f32 = 1.6;

/// A pixel rectangle (x0, y0, x1, y1), end-exclusive.
pub(crate) type Rect = (usize, usize, usize, usize);

/// Every pixel `drag_on` may read or write for a gesture with these points
/// (units), as a conservative end-exclusive rectangle clamped to the canvas.
/// Mirrors the geometry in `drag_on`/`exchange`: the resampled path (a spline
/// can overshoot its control points), hand shake, root offsets with wander
/// and splay, bristle bend (it relaxes from zero toward targets bounded by
/// the trail and spread), the capsule radius, and the plough destination.
pub(crate) fn footprint(tool: &Tool, pts: &[(f32, f32)], shake: f32, scale: f32, w: usize, h: usize) -> Option<Rect> {
    if pts.is_empty() {
        return None;
    }
    let s = scale;
    let px: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
    let path = if px.len() >= 2 { densify(&px) } else { px };
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for &(x, y) in &path {
        x0 = x0.min(x);
        y0 = y0.min(y);
        x1 = x1.max(x);
        y1 = y1.max(y);
    }
    let shake_px = shake.abs() * (0.05 * tool.width + 0.12) * s * 1.5;
    let half = tool.width * 0.5 * s * (1.0 + tool.splay.abs() * 0.5);
    let root = (ROOT_MAX + 0.12 * tool.ragged.abs()) * std::f32::consts::SQRT_2 * half;
    let bend = tool.length * s * LEN_MAX + root * tool.splay.abs() * 0.3;
    let rb = (tool.hair_radius() * s).max(0.55);
    // exchange rect: capsule ± (rb + 1); plough target ≤ off from a pixel in
    // it, off = rb + 1; bounds padding off + 2; plus rounding
    let pad = shake_px + root + bend * 0.6 + (rb + 1.0) + 2.0 * (rb + 1.0) + 4.0;
    if !pad.is_finite() || !x0.is_finite() || !x1.is_finite() || !y0.is_finite() || !y1.is_finite() {
        panic!("non-finite brush footprint (gesture or tool parameters)");
    }
    let c = |v: f32, n: usize| (v.max(0.0) as usize).min(n);
    let r = (c((x0 - pad).floor(), w), c((y0 - pad).floor(), h), c((x1 + pad).ceil() + 1.0, w), c((y1 + pad).ceil() + 1.0, h));
    if r.2 <= r.0 || r.3 <= r.1 { None } else { Some(r) }
}

/// SAFETY: no other thread may touch pixels in `footprint(..)` of `g`.
pub(crate) unsafe fn drag_on(
    sf: Surf,
    held: &mut Held,
    g: &Gesture,
    clip: Option<&Mask>,
    id: u32,
    scratch: &mut Vec<f32>,
) -> Bounds {
    let mut bounds: Bounds = None;
    if g.pts.is_empty() {
        return bounds;
    }
    let s = sf.scale;
    let pts: Vec<(f32, f32)> = g.pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
    let path = if pts.len() >= 2 { densify(&pts) } else { vec![pts[0], pts[0]] };
    let mut arc = vec![0.0f32; path.len()];
    for i in 1..path.len() {
        let (dx, dy) = (path[i].0 - path[i - 1].0, path[i].1 - path[i - 1].1);
        arc[i] = arc[i - 1] + (dx * dx + dy * dy).sqrt();
    }
    let total = arc[arc.len() - 1];
    let tool = held.tool.clone();
    let rb = (tool.hair_radius() * s).max(0.55);
    let full = held.full();
    let step = rb.max(1.25);
    let nsteps = ((total / step).ceil() as usize).max(1);
    let bend_len = tool.length * (0.25 + 0.75 * (1.0 - tool.stiffness));

    for b in &mut held.bristles {
        b.prev = [None, None];
        b.bend = (0.0, 0.0);
    }
    let mut seg = 0usize;
    let mut last_dir = (1.0f32, 0.0f32);
    for k in 0..=nsteps {
        let d = (k as f32 * step).min(total);
        while seg + 1 < path.len() - 1 && arc[seg + 1] < d {
            seg += 1;
        }
        let seg_len = (arc[seg + 1] - arc[seg]).max(1e-6);
        let f = ((d - arc[seg]) / seg_len).clamp(0.0, 1.0);
        let (ax, ay) = path[seg];
        let (bx, by) = path[seg + 1];
        let hx = ax + (bx - ax) * f;
        let hy = ay + (by - ay) * f;
        let dir = if seg_len > 1e-3 { ((bx - ax) / seg_len, (by - ay) / seg_len) } else { last_dir };
        last_dir = dir;
        // the hand is never exact: the path drifts sideways a little and the
        // pressure breathes, both slowly along the stroke
        let lw = (tool.width * 2.5 + 6.0) * s;
        let hs = id as u64 * 7919;
        let off = g.shake * (0.05 * tool.width + 0.12) * s * (wander(d / lw, hs + 1) + 0.5 * wander(d / (lw * 0.37), hs + 2));
        let (hx, hy) = (hx - dir.1 * off, hy + dir.0 * off);
        let u = if total > 0.0 { d / total } else { 0.5 };
        let p = (g.pressure_at(u) * (1.0 + g.shake * 0.14 * wander(d / (lw * 0.8), hs + 3))).clamp(0.0, 1.0);
        let theta = match g.orient {
            Orient::Across => dir.1.atan2(dir.0) + std::f32::consts::FRAC_PI_2,
            Orient::Along => dir.1.atan2(dir.0),
            Orient::Fixed(a) => a,
        };
        let (st, ct) = theta.sin_cos();
        let half = tool.width * 0.5 * s * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5));
        let rate = 1.0 - (-(step / s) / (bend_len + 1e-3)).exp();

        for b in held.bristles.iter_mut() {
            let reach = (p - b.thresh) / (1.0 - b.thresh).max(1e-3);
            if reach <= 0.0 {
                b.prev = [None, None];
                continue;
            }
            // bristles wander a little across the stroke
            let wv = wander(d / s / tool.width.max(2.0) * 1.3, b.seed) * tool.ragged;
            let (ox, oy) = ((b.rx + wv * 0.12) * half, (b.ry + wv * 0.05) * half);
            let root = (hx + ox * ct - oy * st, hy + ox * st + oy * ct);
            // tips trail behind the motion and splay outward under pressure
            let trail = tool.length * s * b.len * p;
            let spread = tool.splay * p * 0.3;
            let target = (-dir.0 * trail + (ox * ct - oy * st) * spread, -dir.1 * trail + (ox * st + oy * ct) * spread);
            b.bend.0 += (target.0 - b.bend.0) * rate;
            b.bend.1 += (target.1 - b.bend.1) * rate;
            // one contact point: the belly-to-tip region of the bent bristle
            let cur = (root.0 + b.bend.0 * 0.6, root.1 + b.bend.1 * 0.6);
            let prev = b.prev[0].unwrap_or(cur);
            unsafe { exchange(sf, b, &tool, prev, cur, rb, reach, full, clip, id, scratch, &mut bounds) };
            b.prev[0] = Some(cur);
        }
    }
    bounds
}

/// Smooth 1-D value noise, −1..1.
fn wander(t: f32, seed: u64) -> f32 {
    let i = t.floor();
    let f = t - i;
    let f = f * f * (3.0 - 2.0 * f);
    let a = crate::rng::hash2(i as i64, 0, seed);
    let b = crate::rng::hash2(i as i64 + 1, 0, seed);
    (a + (b - a) * f) * 2.0 - 1.0
}

/// Paint exchange between one bristle and the canvas along the capsule
/// swept from `a` to `b` (pixels).
#[allow(clippy::too_many_arguments)]
unsafe fn exchange(
    sf: Surf,
    br: &mut Bristle,
    tool: &Tool,
    a: (f32, f32),
    b: (f32, f32),
    rb: f32,
    reach: f32,
    full: f32,
    clip: Option<&Mask>,
    id: u32,
    wts: &mut Vec<f32>,
    bounds: &mut Bounds,
) {
    unsafe {
        let (w, h) = (sf.w, sf.h);
        let s = sf.scale;
        let px_area = 1.0 / (s * s);
        let x0 = ((a.0.min(b.0) - rb - 1.0).floor().max(0.0)) as usize;
        let y0 = ((a.1.min(b.1) - rb - 1.0).floor().max(0.0)) as usize;
        let x1 = ((a.0.max(b.0) + rb + 1.0).ceil().max(0.0) as usize).min(w);
        let y1 = ((a.1.max(b.1) + rb + 1.0).ceil().max(0.0) as usize).min(h);
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        let (mx, my) = if seg > 1e-4 { (dx / seg, dy / seg) } else { (0.0, 0.0) };
        let (nx, ny) = (-my, mx);
        // lowest surface height this bristle reaches down to
        let th = 1.0 - reach * 1.6;

        // pass 1: contact weights
        let bw = x1 - x0;
        wts.clear();
        wts.resize(bw * (y1 - y0), 0.0);
        let mut sum_w = 0.0f32;
        let mut sum_cov = 0.0f32;
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let t = if seg2 > 1e-8 { (((px - a.0) * dx + (py - a.1) * dy) / seg2).clamp(0.0, 1.0) } else { 0.0 };
                let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
                let dist = (qx * qx + qy * qy).sqrt();
                if dist > rb + 0.5 {
                    continue;
                }
                let cov = 1.0 - smoothstep(rb * 0.5, rb + 0.5, dist);
                sum_cov += cov;
                let i = y * w + x;
                let surf = (*sf.base.add(i) + 0.35 * *sf.vol.add(i)).min(1.5);
                // soft hair bends down into the valleys of the weave; stiff hog
                // bristles ride on the peaks
                let give = 0.15 + 0.45 * (1.0 - tool.stiffness).clamp(0.0, 1.0);
                let contact = smoothstep(th - give, th + 0.2, surf);
                let mut wt = cov * contact;
                if let Some(m) = clip {
                    wt *= m.data[i];
                }
                wts[(y - y0) * bw + (x - x0)] = wt;
                sum_w += wt;
            }
        }
        if sum_w <= 1e-6 {
            return;
        }

        // deposit: a share of the load, proportional to distance traveled
        let travel = (seg / s).max(rb / s * 0.5);
        // only the part of the footprint actually in contact takes paint: a
        // bristle skimming the weave peaks keeps most of its load
        let touch = (sum_w / sum_cov.max(1e-6)).min(1.0);
        let dep_total = br.vol * (1.0 - (-travel / tool.run).exp()) * touch;
        let dep_per_w = dep_total / sum_w / px_area;
        // film splitting: a bristle in wet paint always lifts some of it, even
        // when loaded; a spent bristle drinks more
        let hunger = 0.35 + 0.65 * (1.0 - br.vol / full).clamp(0.0, 1.0).powf(1.5);
        let push_k = tool.push * (seg / (2.0 * rb)).clamp(0.0, 1.0);
        let off = rb + 1.0;

        let mut got_v = 0.0f32;
        let mut got_l = [0.0f32; LAT];
        let mut got_h: Prop = [0.0, 0.0];
        let (blat, bhide) = (br.lat, br.hide);
        for y in y0..y1 {
            for x in x0..x1 {
                let wt = wts[(y - y0) * bw + (x - x0)];
                if wt <= 0.0 {
                    continue;
                }
                let i = y * w + x;
                let vol = &mut *sf.vol.add(i);
                // one stroke lifts only part of the film
                if *sf.touched.add(i) != id {
                    *sf.touched.add(i) = id;
                    *sf.floor.add(i) = *vol * (1.0 - tool.pickup);
                }
                let v = *vol;
                if v > 1e-6 {
                    let own = if *sf.stroke.add(i) == id { 0.15 } else { 1.0 };
                    let take = (v * tool.pickup * wt * hunger * own).min((v - *sf.floor.add(i)).max(0.0));
                    if take > 0.0 {
                        *vol -= take;
                        let tv = take * px_area;
                        got_v += tv;
                        let l = &*sf.lat.add(i);
                        for k in 0..LAT {
                            got_l[k] += l[k] * tv;
                        }
                        let hp = *sf.hide.add(i);
                        got_h[0] += hp[0] * tv;
                        got_h[1] += hp[1] * tv;
                    }
                }
                if dep_per_w > 0.0 {
                    sf.add(i, dep_per_w * wt, &blat, bhide);
                    *sf.stroke.add(i) = id;
                }
                // plough: move paint outward from the bristle's path, and ahead
                if push_k > 0.0 {
                    let v = *sf.vol.add(i);
                    let m = v * push_k * wt;
                    if m > 1e-6 {
                        let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                        let side = if (px - a.0) * nx + (py - a.1) * ny >= 0.0 { 1.0 } else { -1.0 };
                        let tx = px + (nx * side * 0.75 + mx * 0.45) * off;
                        let ty = py + (ny * side * 0.75 + my * 0.45) * off;
                        if tx >= 0.0 && ty >= 0.0 && (tx as usize) < w && (ty as usize) < h {
                            let j = ty as usize * w + tx as usize;
                            if j != i {
                                let l = *sf.lat.add(i);
                                let hd = *sf.hide.add(i);
                                *sf.vol.add(i) -= m;
                                sf.add(j, m, &l, hd);
                            }
                        }
                    }
                }
            }
        }
        br.vol = (br.vol - dep_total).max(0.0);
        if got_v > 0.0 {
            for k in 0..LAT {
                got_l[k] /= got_v;
            }
            got_h[0] /= got_v;
            got_h[1] /= got_v;
            mix_into(&mut br.vol, &mut br.lat, &mut br.hide, got_v, &got_l, got_h);
        }
        let pad = (off + 2.0) as usize;
        grow(bounds, x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
    }
}

