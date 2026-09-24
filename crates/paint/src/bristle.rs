//! Simulated bristle brushes working in wet paint.
//!
//! A `Tool` is a physical brush (round sable, hog flat, filbert, fan, rigger,
//! badger blender). `Held` is that brush in the hand: every bristle has its
//! own little reservoir of paint (volume + Mixbox pigment mix + KM scattering).
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
use crate::exchange::{FINE_RB, Mode, exchange};
pub(crate) use crate::film::{Bounds, Rect};
use crate::film::{Stroke, Surf};
use crate::wet::{LAT, Latent, Layer, Paint, Prop, mix_into};

/// Relief (µm) that spans a bristle's contact range: a bristle pressed
/// lightly touches only peaks this much above their surroundings.
pub(crate) const TOOTH_UM: f32 = 60.0;
/// The level a brush rests on around each pixel: the median height within
/// about `r` px either side (a running median along the rows, then along the
/// columns), smoothed a little. The weave and brush-mark relief, a tooth or
/// so either side, sits about its median as it did about the old mean. A
/// step of thick paint no longer lifts the level of the thin paint beside
/// it: a plain mean (two box blurs of radius r) made the thin side read as a
/// valley ~2r wide that no bristle reached, so a veil or glaze laid up to a
/// thick dark motif stopped short of it and left a pale halo (amnesia 3,
/// easel3_free: the stones, the figure and the trunk looked matted in).
/// Only the corner right at the foot of the step is missed.
pub(crate) fn contact_level(height: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
    use rayon::prelude::*;
    // a window about as wide as the old kernel's reach
    let rm = (3 * r).div_ceil(2).max(1);
    let rows = |src: &[f32], w: usize| -> Vec<f32> {
        let mut out = vec![0.0f32; src.len()];
        out.par_chunks_mut(w).zip(src.par_chunks(w)).for_each(|(o, s)| running_median(s, rm, o));
        out
    };
    let t = transpose(&rows(height, w), w, h);
    let m = transpose(&rows(&t, h), h, w);
    crate::surface::box_blur(&m, w, h, (r / 3).max(1))
}

/// Median of `s[i - r ..= i + r]` (edges repeated) into `out[i]`.
fn running_median(s: &[f32], r: usize, out: &mut [f32]) {
    let n = s.len();
    let at = |i: isize| s[i.clamp(0, n as isize - 1) as usize];
    let mut win: Vec<f32> = (-(r as isize)..=r as isize).map(at).collect();
    win.sort_by(f32::total_cmp);
    for (i, o) in out.iter_mut().enumerate() {
        *o = win[r];
        let (gone, come) = (at(i as isize - r as isize), at(i as isize + r as isize + 1));
        let k = win.partition_point(|v| v.total_cmp(&gone).is_lt());
        win.remove(k);
        let k = win.partition_point(|v| v.total_cmp(&come).is_lt());
        win.insert(k, come);
    }
}

fn transpose(src: &[f32], w: usize, h: usize) -> Vec<f32> {
    use rayon::prelude::*;
    let mut dst = vec![0.0; w * h];
    dst.par_chunks_mut(h).enumerate().for_each(|(x, col)| {
        for (y, c) in col.iter_mut().enumerate() {
            *c = src[y * w + x];
        }
    });
    dst
}


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
    /// How finely the hairs converge to a point: 0 = a blunt tuft (hog,
    /// flat, stippler), 1 = a fine point (round sable, rigger). A pointed
    /// tuft is a cone: pressed lightly only the point touches (a hairline),
    /// pressed harder the belly spreads (width grows with pressure), and on
    /// the lift the mark draws down to a point. Its loaded tip wets the
    /// weave's valleys (a continuous line, not dry-brush dots), paint runs
    /// down from the belly to the tip as the tip lays it, and a tip run dry
    /// loses its point and splits. See `notes/tip.md`.
    pub point: f32,
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
            point: 0.0,
        }
    }

    /// Soft pointed round: smooth, precise, little ploughing. Friedrich's detail brush.
    pub fn round_sable(width: f32) -> Self {
        Tool { stiffness: 0.2, pickup: 0.1, push: 0.05, splay: 0.45, ragged: 0.15, point: 1.0, ..Self::base(Kind::Round, width) }
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
            point: 1.0,
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

    /// Small soft round for stippling: a short, full, blunt-pointed tuft of
    /// soft hair (sable or fitch) used with the tip, touch after touch. Few
    /// modeled hairs (a pressed tip is one patch; see `Touch`), little
    /// ploughing, moderate pickup so touches into wet paint fuse.
    pub fn stippler(width: f32) -> Self {
        Tool {
            bristles: 36,
            length: width * 0.8,
            stiffness: 0.3,
            hair: 1.2,
            lay: 0.9,
            pickup: 0.12,
            push: 0.02,
            splay: 0.4,
            ragged: 0.2,
            ..Self::base(Kind::Round, width)
        }
    }

    /// Check that the tool describes a physically possible brush: finite
    /// parameters, a positive width, hair size and load run, at least one
    /// bristle, no negative length, lay, splay or raggedness, and stiffness,
    /// pickup and push within 0..1. The fields are public, so every painting
    /// entry point checks this (see `assert_valid`): the stroke footprints
    /// the parallel scheduler relies on are only bounds for such tools.
    pub fn validate(&self) -> Result<(), String> {
        let finite = [
            ("width", self.width),
            ("length", self.length),
            ("stiffness", self.stiffness),
            ("hair", self.hair),
            ("run", self.run),
            ("lay", self.lay),
            ("pickup", self.pickup),
            ("push", self.push),
            ("splay", self.splay),
            ("ragged", self.ragged),
            ("point", self.point),
        ];
        if let Some((k, v)) = finite.iter().find(|(_, v)| !v.is_finite()) {
            return Err(format!("tool {k} = {v} is not finite"));
        }
        let checks = [
            ("width > 0", self.width > 0.0),
            ("bristles >= 1", self.bristles >= 1),
            ("length >= 0", self.length >= 0.0),
            ("stiffness in 0..=1", (0.0..=1.0).contains(&self.stiffness)),
            ("hair > 0", self.hair > 0.0),
            ("run > 0", self.run > 0.0),
            ("lay >= 0", self.lay >= 0.0),
            ("pickup in 0..=1", (0.0..=1.0).contains(&self.pickup)),
            ("push in 0..=1", (0.0..=1.0).contains(&self.push)),
            ("splay >= 0", self.splay >= 0.0),
            ("ragged >= 0", self.ragged >= 0.0),
            ("point in 0..=1", (0.0..=1.0).contains(&self.point)),
        ];
        match checks.iter().find(|(_, ok)| !ok) {
            Some((rule, _)) => Err(format!("invalid tool: want {rule} ({self:?})")),
            None => Ok(()),
        }
    }

    /// Panic unless `validate` accepts the tool (painting entry points).
    #[track_caller]
    pub(crate) fn assert_valid(&self) {
        if let Err(e) = self.validate() {
            panic!("{e}");
        }
    }

    /// Width (units) of the mark a loaded brush makes at pressure `p`, from
    /// the same geometry the bristles use (a lower bound of about two
    /// hairs: the finest line the point draws). For a pointed tool it grows
    /// roughly in proportion to the pressure; a blunt one starts wide.
    pub fn mark_width(&self, p: f32) -> f32 {
        let p = p.clamp(0.0, 1.0);
        let half = self.width * 0.5 * (0.45 + 0.55 * p) * (1.0 + self.splay * (p - 0.5)) * cone(self, p, 1.0);
        // the outermost hair that touches, at its root radius
        let rho = if self.point > 0.0 { lerp_f(1.0, (p / P_FULL).min(1.0).sqrt(), self.point) } else { 1.0 };
        (2.0 * half * rho).max(2.0 * self.hair_radius())
    }

    /// The pressure at which the brush makes a mark `width` units wide
    /// (see `mark_width`), clamped to 0..1.
    pub fn pressure_for(&self, width: f32) -> f32 {
        let (mut lo, mut hi) = (0.0f32, 1.0f32);
        for _ in 0..30 {
            let m = 0.5 * (lo + hi);
            if self.mark_width(m) < width {
                lo = m;
            } else {
                hi = m;
            }
        }
        0.5 * (lo + hi)
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
    pub(crate) seed: u64,
    prev: [Option<(f32, f32)>; 2],
    /// All the paint the bristle holds (coats × units²): its reservoir
    /// (`lat`, `hide`) and its tip.
    pub(crate) vol: f32,
    pub(crate) lat: Latent,
    pub(crate) hide: Prop,
    /// The paint on the bristle's surface, part of `vol`: what it picked up
    /// from the wet film it passed through. It is laid first, and works
    /// into the reservoir (`lat`, `hide`: the rest of `vol`) as the brush
    /// travels (`exchange::TIP_RUN`). A dip in the pile coats it afresh.
    pub(crate) tip: Layer,
}

impl Bristle {
    /// Work the tip's paint into the reservoir: all of it, or the share `k`.
    pub(crate) fn fold_tip(&mut self, k: f32) {
        let t = &mut self.tip;
        if t.v <= 0.0 {
            return;
        }
        let m = t.v * k.clamp(0.0, 1.0);
        let mut res = (self.vol - t.v).max(0.0);
        mix_into(&mut res, &mut self.lat, &mut self.hide, m, &t.lat, t.hide);
        t.v -= m;
    }
}

/// A brush in the hand, with paint in its bristles.
#[derive(Clone)]
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
                    thresh: {
                        let noise = tool.ragged * rng.f().powf(1.5) * 0.55;
                        let blunt = noise
                            + match tool.kind {
                                Kind::Round | Kind::Rigger => 0.6 * (rx * rx + ry * ry),
                                Kind::Filbert => 0.3 * (rx * rx + ry * ry / 0.09),
                                _ => 0.0,
                            };
                        if tool.point > 0.0 {
                            // a pointed tuft is graded: the hairs at root
                            // radius ρ end on the cone, P_FULL·ρ² up from the
                            // point, so the touching share grows with the
                            // pressure; unevenness grows outward (a clean point)
                            let r2 = rx * rx + ry * ry;
                            let pointed = P_FULL * r2 + noise * r2.sqrt() * 0.5;
                            lerp_f(blunt, pointed, tool.point)
                        } else {
                            blunt
                        }
                    },
                    bend: (0.0, 0.0),
                    seed: i as u64 * 7919 + seed,
                    prev: [None, None],
                    vol: 0.0,
                    lat: [0.0; LAT],
                    hide: [0.5, 0.5, 1.0],
                    tip: Layer::new(0.0, [0.0; LAT], [0.5, 0.5, 1.0]),
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
        let scatter = paint.scatter();
        for (i, b) in self.bristles.iter_mut().enumerate() {
            let k = 0.75 + 0.5 * crate::rng::hash2(i as i64, 17, 3);
            // (the pile coats the hairs afresh: what the tip held goes in)
            b.fold_tip(1.0);
            mix_into(&mut b.vol, &mut b.lat, &mut b.hide, amount * full * k, &lat, [scatter, paint.stiff, paint.drying]);
        }
    }

    /// Wipe the brush on a rag: remove `frac` of the paint in it.
    pub fn wipe(&mut self, frac: f32) {
        for b in &mut self.bristles {
            b.vol *= 1.0 - frac.clamp(0.0, 1.0);
            b.tip.v *= 1.0 - frac.clamp(0.0, 1.0);
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
    /// Pressure swell along the stroke: multipliers at evenly spaced knots
    /// from start to end, interpolated smoothly (empty = none). A hand
    /// presses harder and lighter as it travels.
    pub swell: Vec<f32>,
}

impl Gesture {
    pub fn new(pts: Vec<(f32, f32)>) -> Self {
        Gesture { pts, pressure: (0.8, 0.8), orient: Orient::Across, attack: 0.08, release: 0.15, shake: 1.0, swell: Vec::new() }
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
    /// Vary the pressure along the stroke (see `swell`).
    pub fn swell(mut self, knots: Vec<f32>) -> Self {
        self.swell = knots;
        self
    }
    fn swell_at(&self, u: f32) -> f32 {
        let k = &self.swell;
        match k.len() {
            0 => 1.0,
            1 => k[0],
            n => {
                let t = u.clamp(0.0, 1.0) * (n - 1) as f32;
                let i = (t as usize).min(n - 2);
                let s = smoothstep(0.0, 1.0, t - i as f32);
                k[i] + (k[i + 1] - k[i]) * s
            }
        }
    }
    fn pressure_at(&self, u: f32) -> f32 {
        let base = (self.pressure.0 + (self.pressure.1 - self.pressure.0) * u) * self.swell_at(u);
        let a = if self.attack > 0.0 { 0.08 + 0.92 * smoothstep(0.0, self.attack, u) } else { 1.0 };
        let r = if self.release > 0.0 { 0.05 + 0.95 * smoothstep(0.0, self.release, 1.0 - u) } else { 1.0 };
        base * a * r
    }
}

impl Canvas {
    pub(crate) fn next_stroke_ids(&mut self, n: u32) -> u32 {
        let first = self.wet.current.wrapping_add(1).max(1);
        self.wet.current = first.wrapping_add(n);
        first
    }

    /// Drag a held brush through a gesture, working the wet paint.
    pub fn drag(&mut self, held: &mut Held, g: &Gesture, clip: Option<&Mask>) {
        self.drag_counted(held, g, clip);
    }

    /// `drag`, reporting how often the stroke's pickup and plough took
    /// paint it had set aside (`film::Stroke::through`).
    pub(crate) fn drag_counted(&mut self, held: &mut Held, g: &Gesture, clip: Option<&Mask>) -> [u32; 2] {
        held.tool.assert_valid();
        if let Some(m) = clip {
            self.check_mask(m);
        }
        self.tally.stroke(&held.tool, &g.pts, self.mm_per_unit);
        let id = self.next_stroke_ids(1);
        let surf = self.surf();
        let mut scratch = Vec::new();
        // SAFETY: exclusive &mut self, single brush.
        let (b, through) = unsafe { drag_stroke(surf, held, g, clip, id, &mut scratch) };
        if let Some((x0, y0, x1, y1)) = b {
            self.wet.touch(x0, y0, x1, y1);
        }
        through
    }
}

/// Pressure at which a pointed tuft's whole belly is down.
const P_FULL: f32 = 0.85;
/// Distance (in tool widths) over which paint runs down from the belly of a
/// pointed tuft to its tip (e-folding): capillary feed.
const FEED_WIDTHS: f32 = 2.0;

fn lerp_f(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// How the hairs of a pointed tuft gather toward the point at pressure `p`:
/// a factor on their spread (1 = at their belly positions). A cohesive
/// (wet) tuft is a cone, so the spread goes with the square root of the
/// pressure and, with the graded hair lengths, the mark's width with the
/// pressure itself; a dry tuft (`coh` → 0) no longer holds its point.
fn cone(tool: &Tool, p: f32, coh: f32) -> f32 {
    if tool.point <= 0.0 {
        return 1.0;
    }
    lerp_f(1.0, (p.max(0.0) / P_FULL).min(1.0).sqrt(), tool.point * coh)
}

/// Cohesion of a pointed tuft from its load: wet hairs cling into a point,
/// hairs run dry spring apart (the point splits).
fn cohesion(held: &Held, full: f32) -> f32 {
    if held.tool.point <= 0.0 {
        return 1.0;
    }
    let n = held.bristles.len().max(1) as f32;
    let fill = held.bristles.iter().map(|b| b.vol).sum::<f32>() / (full * n);
    smoothstep(0.02, 0.2, fill)
}

/// Capillary feed: paint in a soft tuft runs from full hairs to spent ones
/// (the belly feeds the tip). Moves the share `k` of every hair's paint into
/// a common pool and shares it out evenly; volume is conserved exactly and
/// colors mix through the tuft.
fn feed(bristles: &mut [Bristle], k: f32) {
    if k <= 0.0 || bristles.is_empty() {
        return;
    }
    let (mut tv, mut lat, mut hide) = (0.0f32, [0.0f32; LAT], [0.0f32; 3]);
    // (the feed runs through the whole tuft, tips and all)
    for b in bristles.iter_mut() {
        b.fold_tip(1.0);
    }
    for b in bristles.iter() {
        tv += b.vol;
        for (l, bl) in lat.iter_mut().zip(&b.lat) {
            *l += bl * b.vol;
        }
        for (h, bh) in hide.iter_mut().zip(&b.hide) {
            *h += bh * b.vol;
        }
    }
    if tv <= 1e-12 {
        return;
    }
    for l in &mut lat {
        *l /= tv;
    }
    hide = [hide[0] / tv, hide[1] / tv, hide[2] / tv];
    let share = k * tv / bristles.len() as f32;
    for b in bristles.iter_mut() {
        b.vol *= 1.0 - k;
        mix_into(&mut b.vol, &mut b.lat, &mut b.hide, share, &lat, hide);
    }
}

/// How far the hairs of a lifting brush trail (as a share of the full trail
/// they have pressed down) as the handle rises (see `lift_off`).
const LIFT_DRAG: f32 = 0.6;

/// How far along its lift-off a stroke is at `u` (0 before the release
/// ramp, 1 at the end), and how the tool lifts: how far its wide axis rolls
/// toward the travel (a flat onto its chisel edge: 1; a filbert half way; a
/// round has no wide axis) and how far its footprint draws in (a round to
/// its point; a flat's edge is its narrowing). A stroke without a release
/// ramp stops square: that's the painter pressing to the end and lifting
/// straight off.
fn lift_off(tool: &Tool, g: &Gesture, u: f32) -> (f32, f32, f32) {
    if g.release <= 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let lift = 1.0 - smoothstep(0.0, g.release, 1.0 - u);
    let (roll, taper) = match tool.kind {
        Kind::Flat => (1.0, 0.0),
        Kind::Filbert => (0.5, 0.3),
        Kind::Fan => (0.3, 0.0),
        Kind::Round | Kind::Rigger => (0.0, if tool.point > 0.0 { 0.0 } else { 0.5 }),
        Kind::Blender => (0.0, 0.3),
    };
    (lift, roll, taper)
}

/// Bristle roots never sit farther than this (in half-widths) from the axis.
const ROOT_MAX: f32 = 1.2;
/// Longest bristle relative to the tool's length.
const LEN_MAX: f32 = 1.6;


/// Every pixel `drag_on` may read or write for a gesture with these points
/// (units), as a conservative end-exclusive rectangle clamped to the canvas.
/// Mirrors the geometry in `drag_on`/`exchange`: the resampled path (a spline
/// can overshoot its control points), hand shake, root offsets with wander
/// and splay, bristle bend (it relaxes from zero toward targets bounded by
/// the trail and spread), the capsule radius, and the plough destination.
pub(crate) fn footprint(tool: &Tool, pts: &[(f32, f32)], shake: f32, scale: f32, w: usize, h: usize) -> Option<Rect> {
    footprint_checked(tool, pts, shake, scale, w, h).unwrap_or_else(|| panic!("non-finite brush footprint (gesture or tool parameters)"))
}

/// `footprint`, or None if it isn't finite.
fn footprint_checked(tool: &Tool, pts: &[(f32, f32)], shake: f32, scale: f32, w: usize, h: usize) -> Option<Option<Rect>> {
    if pts.is_empty() {
        return Some(None);
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
    // a pointed tool's hairs lay tracks up to the tool's half-width wide
    // (a wet tuft bridging between its hairs, see `drag_on`), ploughing up
    // to twice that away
    let bridge = if tool.point > 0.0 { 3.0 * half } else { 0.0 };
    let pad = shake_px + root + bend * 0.6 + (rb + 1.0) + 2.0 * (rb + 1.0) + bridge + 4.0;
    if !pad.is_finite() || !x0.is_finite() || !x1.is_finite() || !y0.is_finite() || !y1.is_finite() {
        return None;
    }
    let c = |v: f32, n: usize| (v.max(0.0) as usize).min(n);
    let r = (c((x0 - pad).floor(), w), c((y0 - pad).floor(), h), c((x1 + pad).ceil() + 1.0, w), c((y1 + pad).ceil() + 1.0, h));
    Some(if r.2 <= r.0 || r.3 <= r.1 { None } else { Some(r) })
}

/// The rectangle a stroke must stay in (whole-canvas pixels), for
/// `exchange`: its planned footprint, or nothing at all.
fn limit(r: Option<Option<Rect>>) -> Rect {
    r.flatten().unwrap_or((0, 0, 0, 0))
}

/// SAFETY: no other thread may touch pixels in `footprint(..)` of `g`, and
/// `held.tool` must pass `Tool::validate`. (Defense in depth: every pixel
/// access is clamped to that footprint, and debug builds assert that the
/// clamp never cuts anything.)
pub(crate) unsafe fn drag_on(
    sf: Surf,
    held: &mut Held,
    g: &Gesture,
    clip: Option<&Mask>,
    id: u32,
    scratch: &mut Vec<f32>,
) -> Bounds {
    unsafe { drag_stroke(sf, held, g, clip, id, scratch).0 }
}

/// `drag_on`, also reporting how often it took set-aside paint.
unsafe fn drag_stroke(sf: Surf, held: &mut Held, g: &Gesture, clip: Option<&Mask>, id: u32, scratch: &mut Vec<f32>) -> (Bounds, [u32; 2]) {
    g.assert_valid();
    if g.pts.is_empty() {
        return (None, [0; 2]);
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
    // a pointed tool's hairs are drawn at their true size, however far
    // below a pixel (see `strip_cover`)
    let rb = (tool.hair_radius() * s).max(if tool.point > 0.0 { FINE_RB } else { 0.55 });
    let full = held.full();
    let step = rb.max(1.25);
    let nsteps = ((total / step).ceil() as usize).max(1);
    let bend_len = tool.length * (0.25 + 0.75 * (1.0 - tool.stiffness));
    let lim = limit(footprint_checked(&tool, &g.pts, g.shake, s, sf.fw, sf.fh));
    // SAFETY: the caller keeps other brushes off this footprint
    let mut stroke = unsafe { Stroke::begin(sf, id, clip, lim, scratch) };

    for b in &mut held.bristles {
        b.prev = [None, None];
        b.bend = (0.0, 0.0);
    }
    let mut seg = 0usize;
    let mut last_dir = (1.0f32, 0.0f32);
    // capillary feed per step (a share per distance, so any resolution
    // feeds the tip alike)
    // (the widest a track may be, bounded in `footprint`)
    let half_max = tool.width * 0.5 * s * (1.0 + tool.splay * 0.5);
    // per hair: the radius (pixels) of the track it lays this step
    let mut excl = vec![rb; held.bristles.len()];
    let mut shift = vec![0.0f32; held.bristles.len()];
    let mut contact: Vec<Option<((f32, f32), f32)>> = vec![None; held.bristles.len()];
    let mut order: Vec<(f32, usize)> = Vec::with_capacity(held.bristles.len());
    let feed_k = tool.point * (1.0 - (-(step / s) / (FEED_WIDTHS * tool.width).max(0.3)).exp());
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
        // lifting off (the release ramp): a flat rolls onto its chisel
        // edge (its wide axis turns toward the travel), a round draws up to
        // its point, a filbert between; the trailing hairs drag longer as
        // the handle rises (see `lift_off`)
        let (lift, roll, taper) = lift_off(&tool, g, u);
        let theta = match g.orient {
            Orient::Across => dir.1.atan2(dir.0) + std::f32::consts::FRAC_PI_2,
            Orient::Along => dir.1.atan2(dir.0),
            Orient::Fixed(a) => a,
        };
        let theta = if lift * roll > 0.0 {
            // turn toward the travel by the shortest way (the axis is a line)
            let along = dir.1.atan2(dir.0);
            let d = (along - theta + std::f32::consts::FRAC_PI_2).rem_euclid(std::f32::consts::PI) - std::f32::consts::FRAC_PI_2;
            theta + d * lift * roll
        } else {
            theta
        };
        let (st, ct) = theta.sin_cos();
        let coh = cohesion(held, full);
        let half = tool.width * 0.5 * s * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5)) * cone(&tool, p, coh) * (1.0 - taper * lift);
        // bend relaxes toward its target: a rate in 0..1 keeps it a blend of
        // targets, within the reach `footprint` allows for
        let rate = (1.0 - (-(step / s) / (bend_len.max(0.0) + 1e-3)).exp()).clamp(0.0, 1.0);

        // where each touching hair meets the canvas this step
        for (bi, b) in held.bristles.iter_mut().enumerate() {
            let reach = (p - b.thresh) / (1.0 - b.thresh).max(1e-3);
            if reach <= 0.0 {
                b.prev = [None, None];
                contact[bi] = None;
                continue;
            }
            // bristles wander a little across the stroke
            let wv = wander(d / s / tool.width.max(2.0) * 1.3, b.seed) * tool.ragged;
            let (ox, oy) = ((b.rx + wv * 0.12) * half, (b.ry + wv * 0.05) * half);
            let root = (hx + ox * ct - oy * st, hy + ox * st + oy * ct);
            // tips trail behind the motion and splay outward under pressure
            let trail = tool.length * s * b.len * (p + LIFT_DRAG * lift * (1.0 - p));
            let spread = tool.splay * p * 0.3;
            let target = (-dir.0 * trail + (ox * ct - oy * st) * spread, -dir.1 * trail + (ox * st + oy * ct) * spread);
            b.bend.0 += (target.0 - b.bend.0) * rate;
            b.bend.1 += (target.1 - b.bend.1) * rate;
            // one contact point: the belly-to-tip region of the bent bristle
            contact[bi] = Some(((root.0 + b.bend.0 * 0.6, root.1 + b.bend.1 * 0.6), reach));
        }
        // the track each hair of a pointed tuft lays (see `exchange`): the
        // touching hairs lie over and beside each other, so each covers its
        // own share of the contact's width, from halfway to its neighbor on
        // one side to halfway to the one on the other, across the direction
        // of travel (a round tuft has more hairs over its middle than its
        // edges). In a wet tuft the paint between the hairs bridges wider
        // gaps too, so the shares tile the whole mark; a dry one leaves the
        // gaps open (a split, streaky mark)
        if tool.point > 0.0 {
            order.clear();
            for (bi, ct) in contact.iter().enumerate() {
                excl[bi] = rb;
                shift[bi] = 0.0;
                if let Some(((cx, cy), _)) = ct {
                    order.push(((cx - hx) * -dir.1 + (cy - hy) * dir.0, bi));
                }
            }
            order.sort_by(|a, b| a.0.total_cmp(&b.0));
            let n = order.len();
            let bridge = |g: f32| g.min(rb) + (g - g.min(rb)) * coh;
            for k in 0..n {
                let lo = bridge(if k > 0 { 0.5 * (order[k].0 - order[k - 1].0) } else { rb });
                let hi = bridge(if k + 1 < n { 0.5 * (order[k + 1].0 - order[k].0) } else { rb });
                // (the floor is far below a hair, and physical, so no
                // resolution adds to it however many hairs lie stacked)
                excl[order[k].1] = (0.5 * (lo + hi)).clamp((0.01 * rb).max(1e-4), half_max.max(rb));
                shift[order[k].1] = (0.5 * (hi - lo)).clamp(-half_max, half_max);
            }
        }
        for (bi, b) in held.bristles.iter_mut().enumerate() {
            let Some((cur, reach)) = contact[bi] else { continue };
            // a pointed tool's hair that has just come down lays its share
            // of the step it came down in (its neighbors' tracks tile with it)
            let prev = b.prev[0].unwrap_or(if tool.point > 0.0 && k > 0 { (cur.0 - dir.0 * step, cur.1 - dir.1 * step) } else { cur });
            let (rk, sh) = if tool.point > 0.0 { (excl[bi], shift[bi]) } else { (rb, 0.0) };
            let (sx, sy) = (-dir.1 * sh, dir.0 * sh);
            unsafe { exchange(&mut stroke, b, &tool, (prev.0 + sx, prev.1 + sy), (cur.0 + sx, cur.1 + sy), rk, reach, full, Mode::Drag, 1.0) };
            b.prev[0] = Some(cur);
        }
        feed(&mut held.bristles, feed_k);
    }
    stroke.finish()
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

/// One touch of the brush: the tip pressed straight down onto the canvas
/// and lifted again, the hand drifting by `drag` and rolling the handle by
/// `twist` while the hairs are down. The mark of a stippling brush, a
/// dabbing sable, the point of a round.
///
/// Pressed vertically, the hairs don't trail: the belly flattens and the
/// tips slide outward, so the contact patch grows with pressure (the
/// outer hairs of a pointed round only land under pressure) and shrinks
/// again on the lift. The hairs of a pressed tip lie against each other and
/// the paint between them bridges the gaps, so the patch is continuous, not
/// one dot per hair. Each hair deposits by film splitting: about half of the
/// paint film on its contact face stays on the canvas, a set thickness per
/// load (not a share per distance, as in a stroke), and it lifts some of
/// the wet paint under it, so touches into wet paint blend and dirty the brush.
#[derive(Clone, Copy, Debug)]
pub struct Touch {
    /// Where the tip lands (units).
    pub at: (f32, f32),
    /// Peak pressure 0..1: harder presses give bigger, fuller marks.
    pub pressure: f32,
    /// Movement of the hand while the hairs are down (units).
    pub drag: (f32, f32),
    /// Roll of the handle while down (radians).
    pub twist: f32,
    /// Direction of the brush's wide axis (radians; flats and filberts).
    pub angle: f32,
}

impl Touch {
    pub fn at(x: f32, y: f32) -> Self {
        Touch { at: (x, y), pressure: 0.6, drag: (0.0, 0.0), twist: 0.0, angle: 0.0 }
    }
    pub fn pressure(mut self, p: f32) -> Self {
        self.pressure = p;
        self
    }
    pub fn drag(mut self, dx: f32, dy: f32) -> Self {
        self.drag = (dx, dy);
        self
    }
    pub fn twist(mut self, a: f32) -> Self {
        self.twist = a;
        self
    }
    pub fn angle(mut self, a: f32) -> Self {
        self.angle = a;
        self
    }
}

/// Thickness (coats, relative to the tool's `lay`) a fully loaded tip
/// leaves where it is pressed flat.
const TOUCH_FILM: f32 = 1.0;
/// Steps through press, hold and lift.
const TOUCH_STEPS: usize = 4;

/// Area of the bristle roots' layout in the brush frame (half-width units²).
fn root_area(kind: Kind) -> f32 {
    use std::f32::consts::PI;
    match kind {
        Kind::Round | Kind::Rigger | Kind::Blender => PI,
        Kind::Filbert => PI * 0.3,
        Kind::Flat => 2.0 * 0.36,
        Kind::Fan => 0.4,
    }
}

/// Half-width of the pressed tip (pixels) at pressure `p`.
/// A pointed tip gathers toward its point (see `cone`).
fn touch_half(tool: &Tool, p: f32, s: f32) -> f32 {
    tool.width * 0.5 * s * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5)) * cone(tool, p, 1.0)
}

/// Contact radius of one hair in a pressed tip (pixels): at least the hair,
/// and wide enough that the fading contacts of neighbors overlap (paint
/// bridges the gaps). The floor is only as wide as a pixel needs to be
/// sampled: a coarse render must not make small marks bigger (and fainter).
fn touch_rb(tool: &Tool, p: f32, s: f32) -> f32 {
    let spacing = touch_half(tool, p, s) * (root_area(tool.kind) / tool.bristles.max(1) as f32).sqrt();
    (tool.hair_radius() * s).max(1.4 * spacing).max(0.75)
}

/// Every pixel `touch_on` may read or write for `t`, as a conservative
/// end-exclusive rectangle clamped to the canvas (see `footprint`).
pub(crate) fn touch_footprint(tool: &Tool, t: &Touch, scale: f32, w: usize, h: usize) -> Option<Rect> {
    touch_footprint_checked(tool, t, scale, w, h).unwrap_or_else(|| panic!("non-finite touch footprint (touch or tool parameters)"))
}

/// `touch_footprint`, or None if it isn't finite.
fn touch_footprint_checked(tool: &Tool, t: &Touch, scale: f32, w: usize, h: usize) -> Option<Option<Rect>> {
    let s = scale;
    let p = t.pressure.clamp(0.0, 1.0);
    let half = tool.width * 0.5 * s * (1.0 + tool.splay.abs() * 0.5);
    let reach = half * ROOT_MAX * std::f32::consts::SQRT_2 * (1.0 + 0.3 * tool.splay.abs()) * (1.0 + 0.5 * (LEN_MAX - 1.0));
    let rb = touch_rb(tool, p, s);
    let pad = reach + 3.0 * (rb + 1.0) + 4.0;
    let (x0, y0) = (t.at.0 * s + t.drag.0.min(0.0) * s, t.at.1 * s + t.drag.1.min(0.0) * s);
    let (x1, y1) = (t.at.0 * s + t.drag.0.max(0.0) * s, t.at.1 * s + t.drag.1.max(0.0) * s);
    if !pad.is_finite() || !x0.is_finite() || !x1.is_finite() || !y0.is_finite() || !y1.is_finite() {
        return None;
    }
    let c = |v: f32, n: usize| (v.max(0.0) as usize).min(n);
    let r = (c((x0 - pad).floor(), w), c((y0 - pad).floor(), h), c((x1 + pad).ceil() + 1.0, w), c((y1 + pad).ceil() + 1.0, h));
    Some(if r.2 <= r.0 || r.3 <= r.1 { None } else { Some(r) })
}

impl Canvas {
    /// Touch the canvas with the tip of a held brush (see `Touch`).
    pub fn touch(&mut self, held: &mut Held, t: &Touch, clip: Option<&Mask>) {
        self.touch_counted(held, t, clip);
    }

    /// `touch`, reporting how often it took set-aside paint (as
    /// `drag_counted`).
    pub(crate) fn touch_counted(&mut self, held: &mut Held, t: &Touch, clip: Option<&Mask>) -> [u32; 2] {
        held.tool.assert_valid();
        if let Some(m) = clip {
            self.check_mask(m);
        }
        self.tally.touch(&held.tool, self.mm_per_unit);
        let id = self.next_stroke_ids(1);
        let surf = self.surf();
        let mut scratch = Vec::new();
        // SAFETY: exclusive &mut self, single brush.
        let (b, through) = unsafe { touch_stroke(surf, held, t, clip, id, &mut scratch) };
        if let Some((x0, y0, x1, y1)) = b {
            self.wet.touch(x0, y0, x1, y1);
        }
        through
    }
}

/// SAFETY: no other thread may touch pixels in `touch_footprint(..)` of `t`,
/// and `held.tool` must pass `Tool::validate` (accesses are clamped to that
/// footprint too, as in `drag_on`).
pub(crate) unsafe fn touch_on(sf: Surf, held: &mut Held, t: &Touch, clip: Option<&Mask>, id: u32, scratch: &mut Vec<f32>) -> Bounds {
    unsafe { touch_stroke(sf, held, t, clip, id, scratch).0 }
}

/// `touch_on`, also reporting how often it took set-aside paint.
unsafe fn touch_stroke(sf: Surf, held: &mut Held, t: &Touch, clip: Option<&Mask>, id: u32, scratch: &mut Vec<f32>) -> (Bounds, [u32; 2]) {
    t.assert_valid();
    let s = sf.scale;
    let tool = held.tool.clone();
    let full = held.full();
    let p = t.pressure.clamp(0.0, 1.0);
    let (cx, cy) = (t.at.0 * s, t.at.1 * s);
    let (dx, dy) = (t.drag.0 * s, t.drag.1 * s);
    let rb = touch_rb(&tool, p, s);
    let lim = limit(touch_footprint_checked(&tool, t, s, sf.fw, sf.fh));
    // SAFETY: the caller keeps other brushes off this footprint
    let mut stroke = unsafe { Stroke::begin(sf, id, clip, lim, scratch) };
    let dlen = (dx * dx + dy * dy).sqrt();
    let steps = (TOUCH_STEPS + (dlen / rb.max(1.0)).ceil() as usize).min(64);
    // press, hold, lift
    let at = |k: usize| {
        let u = (k as f32 + 0.5) / steps as f32;
        (u, p * (std::f32::consts::PI * u).sin().max(0.0).sqrt())
    };
    let reach_of = |b: &Bristle, pk: f32| (pk - b.thresh) / (1.0 - b.thresh).max(1e-3);
    // each hair stands for spacing² of the patch; over the touch it lays
    // TOUCH_FILM·lay coats there at full load, spread over the steps it is down
    let spacing_u = touch_half(&tool, p, s) / s * (root_area(tool.kind) / held.bristles.len().max(1) as f32).sqrt();
    // a lighter press squeezes less paint out of the tip
    let film = TOUCH_FILM * tool.lay * spacing_u * spacing_u * (0.4 + 0.6 * p);
    let sums: Vec<f32> = held.bristles.iter().map(|b| (0..steps).map(|k| reach_of(b, at(k).1).max(0.0)).sum()).collect();
    for b in &mut held.bristles {
        b.prev = [None, None];
    }
    for k in 0..steps {
        let (u, pk) = at(k);
        let half = touch_half(&tool, pk, s);
        let theta = t.angle + t.twist * u;
        let (st, ct) = theta.sin_cos();
        let (hx, hy) = (cx + dx * u, cy + dy * u);
        for (bi, b) in held.bristles.iter_mut().enumerate() {
            let reach = reach_of(b, pk);
            if reach <= 0.0 {
                b.prev[0] = None;
                continue;
            }
            // tips slide outward as the belly flattens; longer hairs farther
            let spread = (1.0 + tool.splay * pk * 0.3) * (1.0 + 0.5 * (b.len - 1.0));
            let (ox, oy) = (b.rx * half * spread, b.ry * half * spread);
            let cur = (hx + ox * ct - oy * st, hy + ox * st + oy * ct);
            let prev = b.prev[0].unwrap_or(cur);
            let fill = (b.vol / full).min(1.0);
            let v = film * fill * reach / sums[bi].max(1e-6);
            unsafe { exchange(&mut stroke, b, &tool, prev, cur, rb, reach, full, Mode::Touch { dep: v }, 1.0) };
            b.prev[0] = Some(cur);
        }
    }
    stroke.finish()
}

#[cfg(test)]
mod tip_tests {
    use super::*;
    use crate::exchange::fine_cover;
    use crate::color::hex;

    const BG: &str = "#e8e0d0";
    const INK: &str = "#1a1612";

    /// A narrow strip of a Friedrich-sized canvas (440 mm wide) with one
    /// mark of dark body paint on it, dried.
    fn canvas(px: usize, linen: bool, tool: Tool, g: &Gesture) -> Canvas {
        let mut c = Canvas::new(px, 4.0, hex(BG)).with_size_mm(440.0);
        if linen {
            c = c.with_linen(crate::surface::Linen::fine(3));
        }
        let mut h = Held::new(tool, 3);
        h.load(Paint::body(hex(INK)), 1.0);
        c.drag(&mut h, g, None);
        c.dry();
        c
    }

    /// Darkness of pixel (x, y): 0 = ground, 1 = the paint's masstone.
    fn dark(c: &Canvas, x: usize, y: usize) -> f32 {
        let lum = |p: [f32; 3]| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
        let (lb, ld) = (lum(hex(BG)), lum(hex(INK)));
        (lb - lum(c.px[y * c.f.w + x])) / (lb - ld)
    }

    /// Ink across the mark (units: the width of a fully dark track) between
    /// x0 and x1 (units), averaged along it.
    fn ink(c: &Canvas, x0: f32, x1: f32) -> f32 {
        let s = c.f.scale;
        let (a, b) = ((x0 * s) as usize, (x1 * s) as usize);
        let mut sum = 0.0;
        for y in 0..c.f.h {
            for x in a..b {
                sum += dark(c, x, y);
            }
        }
        sum / s / (b - a) as f32
    }

    #[test]
    fn pointed_marks_are_resolution_independent() {
        let g = Gesture::line((50.0, 120.0), (450.0, 122.0)).pressure(0.4, 0.4).ramps(0.05, 0.1).shake(0.0);
        for tool in [Tool::rigger(0.5), Tool::round_sable(1.6)] {
            let lo = ink(&canvas(500, false, tool.clone(), &g), 100.0, 400.0);
            let hi = ink(&canvas(1600, false, tool.clone(), &g), 100.0, 400.0);
            assert!((lo / hi - 1.0).abs() < 0.2, "{:?}: ink width {lo} at 500px, {hi} at 1600px", tool.kind);
            // and about as wide as the brush says
            let w = tool.mark_width(0.4);
            assert!(hi > 0.6 * w && hi < 2.0 * w, "{:?}: ink width {hi}, mark_width {w}", tool.kind);
        }
    }

    #[test]
    fn pointed_width_follows_pressure_and_tapers() {
        let t = Tool::round_sable(3.0);
        assert!(t.mark_width(0.1) < 0.25 * t.mark_width(0.9), "{} vs {}", t.mark_width(0.1), t.mark_width(0.9));
        assert!((t.mark_width(t.pressure_for(1.5)) - 1.5).abs() < 0.01);
        let at = |p: f32| ink(&canvas(800, false, t.clone(), &Gesture::line((50.0, 120.0), (450.0, 120.0)).pressure(p, p).shake(0.0)), 150.0, 350.0);
        let (a, b, c) = (at(0.1), at(0.4), at(0.8));
        assert!(a < 0.5 * b && b < 0.7 * c, "ink width at pressure .1/.4/.8: {a} {b} {c}");
        // a flick lifted off draws down to a point
        let f = canvas(800, false, t.clone(), &Gesture::line((50.0, 120.0), (450.0, 120.0)).pressure(0.8, 0.0).ramps(0.05, 0.8).shake(0.0));
        let (root, mid, tip) = (ink(&f, 80.0, 120.0), ink(&f, 230.0, 270.0), ink(&f, 400.0, 430.0));
        assert!(root > mid && mid > tip && tip < 0.3 * root, "flick ink root {root}, middle {mid}, tip {tip}");
        // a blunt stippler of the same kind keeps its old footprint
        assert_eq!(Tool::stippler(2.0).point, 0.0);
    }

    /// A hair's track covers its own area of the pixel lattice, however it
    /// lies across the pixels: diagonal and oblique tracks keep their area
    /// under sub-pixel translation (a track 45° across used to count twice
    /// its area when shifted half a pixel, and nothing like it at others).
    #[test]
    fn fine_cover_conserves_area_on_the_lattice() {
        let total = |a: (f32, f32), b: (f32, f32), rb: f32| {
            let mut s = 0.0f64;
            for y in 0..240 {
                for x in 0..240 {
                    s += fine_cover(a, b, rb, x as f32 + 0.5, y as f32 + 0.5) as f64;
                }
            }
            s as f32
        };
        for rb in [0.02f32, 0.15, 0.6] {
            for deg in [0.0f32, 17.0, 30.0, 45.0, 71.0, 90.0, 135.0] {
                let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
                for len in [0.3f32, 3.7, 100.0] {
                    let want = 2.0 * rb * len;
                    for off in [(0.0f32, 0.0f32), (0.0, 0.5), (0.25, 0.1), (0.5, 0.5), (0.37, 0.81)] {
                        let a = (100.0 + off.0, 20.0 + off.1);
                        let b = (a.0 + c * len, a.1 + s * len);
                        let got = total(a, b, rb);
                        assert!((got / want - 1.0).abs() < 2e-3, "rb {rb}, {deg}°, length {len}, offset {off:?}: covers {got}, track area {want}");
                    }
                }
            }
            // a hair that doesn't move covers its square
            let got = total((30.3, 30.6), (30.3, 30.6), rb);
            assert!((got / (4.0 * rb * rb) - 1.0).abs() < 2e-3, "still hair rb {rb}: {got}");
        }
    }

    /// The same diagonal or oblique pointed mark, shifted by part of a pixel,
    /// darkens the canvas by the same amount (its look follows its paint,
    /// not where it falls between pixel centers).
    #[test]
    fn translated_pointed_marks_look_alike() {
        for (tool, p) in [(Tool::rigger(0.5), 0.3), (Tool::round_sable(1.6), 0.15)] {
            for (dx, dy) in [(200.0f32, 200.0f32), (240.0, 90.0)] {
                let total = |off: (f32, f32)| {
                    let (a, b) = ((100.0 + off.0, 10.0 + off.1), (100.0 + dx + off.0, 10.0 + dy + off.1));
                    let c = canvas(500, false, tool.clone(), &Gesture::line(a, b).pressure(p, p).ramps(0.05, 0.1).shake(0.0));
                    (0..c.f.h).flat_map(|y| (0..c.f.w).map(move |x| (x, y))).map(|(x, y)| dark(&c, x, y)).sum::<f32>()
                };
                let base = total((0.0, 0.0));
                // (a pixel is 2 units here)
                for off in [(0.0, 1.0), (0.5, 0.2), (1.0, 1.0), (0.74, 1.62)] {
                    let got = total(off);
                    assert!((got / base - 1.0).abs() < 0.05, "{:?} along ({dx}, {dy}) shifted {off:?}: darkness {got} vs {base}", tool.kind);
                }
            }
        }
    }

    /// A light hairline on linen at full size is a line, not a row of beads
    /// on the weave's peaks.
    #[test]
    fn hairline_on_linen_is_continuous() {
        let c = canvas(1600, true, Tool::rigger(0.5), &Gesture::line((50.0, 120.0), (450.0, 120.0)).pressure(0.25, 0.25).ramps(0.05, 0.1).shake(0.0));
        let s = c.f.scale;
        let (y0, y1) = (((120.0 - 2.0) * s) as usize, ((120.0 + 2.0) * s) as usize);
        let cols: Vec<f32> = ((150.0 * s) as usize..(350.0 * s) as usize).map(|x| (y0..y1).map(|y| dark(&c, x, y)).sum::<f32>()).collect();
        let mean = cols.iter().sum::<f32>() / cols.len() as f32;
        let gaps = cols.iter().filter(|&&v| v < 0.3 * mean).count();
        assert!(mean > 0.1 && gaps == 0, "mean column ink {mean}, {gaps} gaps of {}", cols.len());
    }

    /// Ink width (units) of straight marks at two resolutions, for tuning.
    #[test]
    #[ignore]
    fn probe_ink_width() {
        for (name, tool) in [("rigger .5", Tool::rigger(0.5)), ("sable 1.6", Tool::round_sable(1.6)), ("sable 3", Tool::round_sable(3.0))] {
            for p in [0.1, 0.4, 0.7, 0.9] {
                let g = Gesture::line((50.0, 120.0), (450.0, 122.0)).pressure(p, p).ramps(0.05, 0.1).shake(0.0);
                let lo = ink(&canvas(1000, false, tool.clone(), &g), 100.0, 400.0);
                let hi = ink(&canvas(3200, false, tool.clone(), &g), 100.0, 400.0);
                println!("{name:10} p {p:.1}: ink width {lo:.3} at 1000px, {hi:.3} at 3200px; mark_width {:.3}", tool.mark_width(p));
            }
        }
    }

    #[test]
    #[ignore]
    fn probe_patch() {
        // TIP_OUT=path.png TIP_P=0.8 cargo test --release -p paint probe_patch -- --ignored
        let out = std::env::var("TIP_OUT").unwrap_or_else(|_| "patch.png".into());
        let mut c = Canvas::new(3200, 4.0, hex(BG)).with_size_mm(440.0).with_linen(crate::surface::Linen::fine(3));
        let mut h = Held::new(Tool::round_sable(5.6), 3);
        let mut rng = crate::rng::Rng::new(4);
        for k in 0..14 {
            h.reload(Paint::body(hex(INK)), 1.0);
            let y = 60.0 + k as f32 * 3.0;
            let pr: f32 = std::env::var("TIP_P").ok().and_then(|v| v.parse().ok()).unwrap_or(0.7);
            c.drag(&mut h, &Gesture::new(vec![(60.0, y + rng.range(-1.0, 1.0)), (120.0, y + 2.0), (180.0, y + rng.range(-1.0, 1.0))]).pressure(pr * 0.6, pr).ramps(0.05, 0.1), None);
        }
        c.dry();
        c.save(std::path::Path::new(&out)).unwrap();
    }

    #[test]
    fn feed_conserves_paint() {
        let mut h = Held::new(Tool::round_sable(2.0), 1);
        h.load(Paint::body(hex(INK)), 1.0);
        for (i, b) in h.bristles.iter_mut().enumerate() {
            b.vol *= (i % 5) as f32 / 4.0;
        }
        let before: f64 = h.bristles.iter().map(|b| b.vol as f64).sum();
        feed(&mut h.bristles, 0.3);
        let after: f64 = h.bristles.iter().map(|b| b.vol as f64).sum();
        assert!((after - before).abs() < before * 1e-5);
    }
}


impl Gesture {
    /// Check that every number in the gesture is finite: a NaN point would
    /// otherwise make the brush take one step and lift, silently (a NaN arc
    /// length casts to zero steps). E.g. `sin(PI).powf(0.8)` is NaN in f32.
    pub fn validate(&self) -> Result<(), String> {
        if let Some((i, p)) = self.pts.iter().enumerate().find(|(_, p)| !(p.0.is_finite() && p.1.is_finite())) {
            return Err(format!("Gesture point {i} is not finite: ({}, {})", p.0, p.1));
        }
        let nums = [("pressure.0", self.pressure.0), ("pressure.1", self.pressure.1), ("attack", self.attack), ("release", self.release), ("shake", self.shake)];
        if let Some((name, v)) = nums.iter().find(|(_, v)| !v.is_finite()) {
            return Err(format!("Gesture {name} is not finite: {v}"));
        }
        if let Some((i, v)) = self.swell.iter().enumerate().find(|(_, v)| !v.is_finite()) {
            return Err(format!("Gesture swell knot {i} is not finite: {v}"));
        }
        Ok(())
    }

    #[track_caller]
    pub(crate) fn assert_valid(&self) {
        if let Err(e) = self.validate() {
            panic!("{e}");
        }
    }
}

impl Touch {
    /// Check that every number in the touch is finite (see `Gesture::validate`).
    pub fn validate(&self) -> Result<(), String> {
        let nums = [("at.x", self.at.0), ("at.y", self.at.1), ("pressure", self.pressure), ("drag.x", self.drag.0), ("drag.y", self.drag.1), ("twist", self.twist), ("angle", self.angle)];
        match nums.iter().find(|(_, v)| !v.is_finite()) {
            Some((name, v)) => Err(format!("Touch {name} is not finite: {v}")),
            None => Ok(()),
        }
    }

    #[track_caller]
    pub(crate) fn assert_valid(&self) {
        if let Err(e) = self.validate() {
            panic!("{e}");
        }
    }
}

#[cfg(test)]
mod cover_tests {
    use crate::canvas::Canvas;
    use crate::color::hex;
    use crate::handling::Handling;
    use crate::mask::Mask;
    use crate::style::Style;

    fn lum(p: [f32; 3]) -> f32 {
        0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]
    }

    /// Paint a dark square (units 350..650) with `hd`, dry it, and return
    /// (share of interior pixels whose wet film is under 0.15 coat before
    /// drying, share that shows the ground after: closer to the ground's
    /// value than to the paint's).
    pub(crate) fn bare_share(w: usize, hd: &Handling, seed: u64) -> (f32, f32) {
        let st = Style::friedrich_early();
        let mut c: Canvas = st.prepare(w, 1.0, seed);
        let m = Mask::from_fn(c.frame(), |x, y| if (350.0..650.0).contains(&x) && (350.0..650.0).contains(&y) { 1.0 } else { 0.0 });
        let ground = c.px.clone();
        c.work(&m, hd, seed);
        let f = c.f;
        let inner: Vec<usize> = (0..f.w * f.h).filter(|&i| {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            (380.0..620.0).contains(&x) && (380.0..620.0).contains(&y)
        }).collect();
        let thin = inner.iter().filter(|&&i| c.wet.vol[i] < 0.15).count() as f32 / inner.len() as f32;
        let untouched = inner.iter().filter(|&&i| c.wet.vol[i] < 0.15 && c.wet.touched[i] == 0).count() as f32 / inner.len() as f32;

        if std::env::var_os("PROBE_VERBOSE").is_some() {
            println!("    thin {:.2}%, of it never touched by a bristle {:.2}%", 100.0 * thin, 100.0 * untouched);
        }
        c.dry();
        let mut ls: Vec<f32> = inner.iter().map(|&i| lum(c.px[i])).collect();
        ls.sort_by(f32::total_cmp);
        let paint = ls[ls.len() / 2];
        if let Ok(out) = std::env::var("PROBE_OUT") {
            c.save(std::path::Path::new(&format!("{out}_{}.png", hd.tool.width))).unwrap();
        }
        let bare = inner.iter().filter(|&&i| lum(c.px[i]) - paint > 0.5 * (lum(ground[i]) - paint)).count() as f32 / inner.len() as f32;
        (thin, bare)
    }

    /// A loaded brush covers a passage it means to cover: no flecks of the
    /// ground in a dark body passage at coverage 2.5 (they were the gaps
    /// the strokes' hand placement left, 1.5% of the area; broad 11%); a
    /// nearly dry brush at light pressure still breaks up (dry brush).
    #[test]
    fn loaded_passage_covers() {
        let st = Style::friedrich_early();
        let dark = |_: f32, _: f32| hex("#2c2925");
        let body = || st.body().color(dark).clip(true).threshold(0.5).coverage(2.5);
        let broad = || st.broad().color(dark).clip(true).threshold(0.5).coverage(2.5);
        let (_, bare) = bare_share(500, &body(), 5);
        let (_, bare_broad) = bare_share(500, &broad(), 5);
        let (_, gaps) = bare_share(500, &broad().fill(false), 5);
        let h = body().pressure(0.15, 0.3);
        let (_, dry) = bare_share(500, &Handling { load: 0.1, ..h }, 5);
        println!("bare: body {:.2}%, broad {:.2}% ({:.2}% without looking), dry brush {:.1}%", 100.0 * bare, 100.0 * bare_broad, 100.0 * gaps, 100.0 * dry);
        assert!(bare < 0.002 && bare_broad < 0.002, "a loaded passage at coverage 2.5 shows bare ground: body {:.2}%, broad {:.2}%", 100.0 * bare, 100.0 * bare_broad);
        assert!(gaps > 0.005, "without looking the strokes leave gaps: {gaps}");
        assert!(dry > 0.1, "dry brush at light pressure should break up: {dry}");
    }

    /// Where do bare flecks in a dark body passage come from?
    /// `cargo test --release -p paint probe_bare -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn probe_bare() {
        let st = Style::friedrich_early();
        let w: usize = std::env::var("PROBE_W").ok().and_then(|s| s.parse().ok()).unwrap_or(1000);
        let dark = |_: f32, _: f32| hex("#2c2925");
        let names: Vec<String> = std::env::var("PROBE").map(|s| s.split(',').map(String::from).collect()).unwrap_or(vec!["body".into(), "broad".into(), "detail".into()]);
        let covs: Vec<f32> = std::env::var("PROBE_COV").map(|s| s.split(',').map(|v| v.parse().unwrap()).collect()).unwrap_or(vec![1.0, 2.5, 4.0]);
        let vars = std::env::var("PROBE_VARS").unwrap_or("base,nofill".into());
        for name in names.iter().map(|s| s.as_str()) {
            for &cov in &covs {
                let base = || match name {
                    "body" => st.body(),
                    "broad" => st.broad(),
                    _ => st.detail(),
                }
                .color(dark)
                .clip(true)
                .threshold(0.5)
                .coverage(cov);
                let mut line = format!("{name:6} cov {cov:.1}:");
                for (var, hd) in [
                    ("base", base()),
                    ("push0", { let h = base(); let t = crate::bristle::Tool { push: 0.0, ..h.tool.clone() }; Handling { tool: t, ..h } }),
                    ("pick0", { let h = base(); let t = crate::bristle::Tool { pickup: 0.0, ..h.tool.clone() }; Handling { tool: t, ..h } }),
                    ("unbroken", { let h = base(); Handling { broken: 0.0, ..h } }),
                    ("full", { let h = base(); Handling { load: 1.0, dip_every: 1, ..h } }),
                    ("nofill", base().fill(false)),
                    ("load.1", { let h = base(); Handling { load: 0.1, ..h } }),
                    ("load.2", { let h = base(); Handling { load: 0.2, ..h } }),
                    ("load.4", { let h = base(); Handling { load: 0.4, ..h } }),
                    ("light", base().pressure(0.15, 0.3)),
                    ("light.1", { let h = base().pressure(0.15, 0.3); Handling { load: 0.1, ..h } }),
                ] {
                    if !vars.split(',').any(|v| v == var) {
                        continue;
                    }
                    let seeds: Vec<u64> = std::env::var("PROBE_SEEDS").unwrap_or("5,6,7".into()).split(',').map(|v| v.parse().unwrap()).collect();
                    let (mut thin, mut bare) = (0.0, 0.0);
                    for &sd in &seeds {
                        let (t, b) = bare_share(w, &hd, sd);
                        thin += t / seeds.len() as f32;
                        bare += b / seeds.len() as f32;
                    }
                    line += &format!("  {var} thin {:.2}% bare {:.2}%", 100.0 * thin, 100.0 * bare);
                }
                println!("{line}");
            }
        }
    }

    /// Area a single stroke covers (film ≥ 0.15 coat) against its nominal
    /// width × length.
    #[test]
    #[ignore]
    fn probe_mark_area() {
        use crate::bristle::{Gesture, Held};
        use crate::wet::Paint;
        let st = Style::friedrich_early();
        for (name, tool, len, p, ramps, load) in [
            ("broad", st.broad.clone(), 150.0f32, 0.675f32, (0.12f32, 0.4f32), 0.4f32),
            ("body", st.body.clone(), 40.0, 0.75, (0.08, 0.15), 0.56),
            ("detail", st.detail.clone(), 9.0, 0.82, (0.08, 0.15), 0.72),
        ] {
            let mut tot = (0.0, 0.0);
            for sd in 0..6u64 {
                let mut c: Canvas = st.prepare(1000, 1.0, sd);
                let mut h = Held::new(tool.clone(), sd);
                h.load(Paint::body(hex("#2c2925")), load);
                let y = 500.0;
                c.drag(&mut h, &Gesture::new(vec![(500.0 - len / 2.0, y), (500.0 + len / 2.0, y)]).pressure(p, p * 0.9).ramps(ramps.0, ramps.1), None);
                let a = |t: f32| c.wet.vol.iter().filter(|&&v| v >= t).count() as f32 / (c.f.scale * c.f.scale);
                tot.0 += a(0.15) / 6.0;
                tot.1 += a(1e-4) / 6.0;
            }
            println!("{name:6}: area {:.0} (any paint {:.0}) of nominal {:.0}: {:.2}; mark_width {:.2} of {:.2}", tot.0, tot.1, tool.width * len, tot.0 / (tool.width * len), tool.mark_width(p), tool.width);
        }
    }
}

/// Paint is conserved: no brush operation creates or destroys pigment or
/// changes paint properties in total. Every moment (volume, volume × each
/// latent pigment component, volume × scattering, stiffness and drying
/// rate), summed over the canvas's wet paint and the brush's bristles, is
/// the same before and after a stroke.
#[cfg(test)]
mod conservation {
    use super::*;
    use crate::color::hex;
    use crate::surface::Linen;

    const N: usize = LAT + 4;

    fn add(m: &mut [f64; N], v: f32, lat: &Latent, hide: &Prop) {
        let v = v as f64;
        m[0] += v;
        for k in 0..LAT {
            m[1 + k] += v * lat[k] as f64;
        }
        for k in 0..3 {
            m[1 + LAT + k] += v * hide[k] as f64;
        }
    }

    /// Moments of the canvas's wet paint (per unit², like a brush's
    /// volumes) and of the brushes' paint.
    fn moments(c: &Canvas, brushes: &[&Held]) -> [f64; N] {
        let mut m = [0.0f64; N];
        let px_area = 1.0 / (c.f.scale * c.f.scale);
        let w = &c.wet;
        for i in 0..w.vol.len() {
            let t = &w.top[i];
            assert!(t.v <= w.vol[i], "pixel {i}: surface film {} of {}", t.v, w.vol[i]);
            add(&mut m, (w.vol[i] - t.v) * px_area, &w.lat[i], &w.hide[i]);
            add(&mut m, t.v * px_area, &t.lat, &t.hide);
        }
        for h in brushes {
            for b in &h.bristles {
                let t = &b.tip;
                assert!(t.v <= b.vol, "tip {} of {}", t.v, b.vol);
                add(&mut m, b.vol - t.v, &b.lat, &b.hide);
                add(&mut m, t.v, &t.lat, &t.hide);
            }
        }
        m
    }

    fn check(before: &[f64; N], after: &[f64; N], what: &str) {
        // (each moment relative to its own total, or to the volume where
        // that total is small: f32 mixing rounds at about 1e-6 per step)
        let vol = before[0].max(after[0]).max(1e-9);
        let names = |k: usize| if k == 0 { "volume".to_string() } else if k <= LAT { format!("pigment {}", k - 1) } else { ["scattering", "stiffness", "drying"][k - 1 - LAT].to_string() };
        for k in 0..N {
            let e = (after[k] - before[k]).abs() / before[k].abs().max(after[k].abs()).max(0.01 * vol);
            assert!(e < 2e-4, "{what}: {} changed by {e:.2e} of its total ({} → {})", names(k), before[k], after[k]);
        }
    }

    /// A dark field, a light field over half of it (so its pixels carry a
    /// surface film), then a third pigment stroked across both while all is
    /// wet: its bristles lay paint on the earlier surface film (setting it
    /// aside until the stroke ends), then pick up and plough through the
    /// pixels where it lies. Stirring, pickup, the plough and the stroke's
    /// settlement must all keep every moment.
    #[test]
    fn a_third_stroke_over_two_wet_colors_conserves_paint() {
        let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_linen(Linen::fine(3));
        let paints = [
            Paint::body(hex("#23302a")).with_drying(0.4).with_stiff(0.9),
            Paint::new(hex("#e2d6b8"), 0.8, 0.5).with_drying(2.0),
            Paint::new(hex("#a0342a"), 0.7, 0.3).with_drying(1.2),
        ];
        let mut h = Held::new(Tool::hog_flat(30.0), 5);
        // the dark, then the light over its right half
        for (k, (p, x0)) in [(paints[0], 100.0), (paints[1], 480.0)].into_iter().enumerate() {
            let mut y = 300.0;
            while y < 700.0 {
                h.reload(p, 0.9);
                let b = moments(&c, &[&h]);
                c.drag(&mut h, &Gesture::new(vec![(x0, y), (900.0, y + 3.0)]).pressure(0.85, 0.8).orient(Orient::Across), None);
                check(&b, &moments(&c, &[&h]), &format!("field {k} at y {y}"));
                y += 22.0;
            }
        }
        // the third pigment, pressed and dragged back and forth across both
        // (and how often its strokes took paint they had set aside)
        let mut moves = [0u32; 2];
        let mut count = |m: [u32; 2]| (0..2).for_each(|k| moves[k] += m[k]);
        let mut r = Held::new(Tool::filbert(24.0), 9);
        for (k, y) in [360.0f32, 420.0, 480.0, 540.0, 600.0].into_iter().enumerate() {
            r.reload(paints[2], if k % 2 == 0 { 0.8 } else { 0.25 });
            let b = moments(&c, &[&r]);
            count(c.drag_counted(&mut r, &Gesture::new(vec![(120.0, y), (500.0, y - 20.0), (880.0, y + 10.0), (500.0, y + 30.0)]).pressure(0.95, 0.7).orient(Orient::Across), None));
            check(&b, &moments(&c, &[&r]), &format!("third stroke {k}"));
            // and a clean hog through it at once: pickup and plough
            let mut clean = Held::new(Tool::hog_flat(18.0), 11 + k as u64);
            let b = moments(&c, &[&clean]);
            count(c.drag_counted(&mut clean, &Gesture::new(vec![(860.0, y + 5.0), (140.0, y - 5.0)]).pressure(0.95, 0.9).orient(Orient::Across), None));
            check(&b, &moments(&c, &[&clean]), &format!("clean hog after third stroke {k}"));
            // and a touch into it
            let b = moments(&c, &[&r]);
            count(c.touch_counted(&mut r, &Touch::at(500.0, y).pressure(0.9).drag(3.0, 1.0), None));
            check(&b, &moments(&c, &[&r]), &format!("touch after third stroke {k}"));
        }
        // (the strokes did pick up and plough paint set aside under them)
        assert!(moves[0] > 100 && moves[1] > 100, "pickups and plough moves through set-aside paint: {moves:?}");
        assert!(c.aside.settled(), "every set-aside film is settled when its stroke ends");
    }
}
