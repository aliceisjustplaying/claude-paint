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
use crate::surface::COAT_UM;
use crate::wet::{LAT, Latent, Paint, Prop, mix_into};

/// Relief (µm) that spans a bristle's contact range: a bristle pressed
/// lightly touches only peaks this much above their surroundings.
const TOOTH_UM: f32 = 60.0;
/// The level a brush rests on around each pixel: the median height within
/// about `r` px either side (a running median along the rows, then along the
/// columns, over ±1.5 r), smoothed a little (a box blur of radius r/3).
/// The weave and brush-mark relief, a tooth or so either side, sits about
/// its median. A step of thick paint doesn't lift the level of the thin
/// paint beside it (with a mean, the thin side would be a valley ~2r wide
/// that no bristle reaches, and paint laid up to the step would stop short
/// of it). Only the corner right at the foot of the step is missed.
pub(crate) fn contact_level(height: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
    use rayon::prelude::*;
    // a window of ±1.5 r
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

/// How far into the tooth's range the paint a fully loaded hair carries
/// reaches ahead of the hair (see `exchange`).
const WET_REACH: f32 = 0.5;

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
    /// How finely the hairs converge to a point: 0 = a blunt tuft, 1 = a
    /// fine point. 0 for every preset (round sable and rigger included, so
    /// a mark is as wide as the brush and pressure ask): the pointed tip is
    /// opt-in, `Tool { point: 1.0, ..Tool::round_sable(w) }`. A pointed
    /// tuft is a cone: pressed lightly only the point touches (a hairline),
    /// pressed harder the belly spreads (width grows with pressure), and on
    /// the lift the mark draws down to a point. Its loaded tip wets the
    /// weave's valleys (a continuous line, not dry-brush dots), paint runs
    /// down from the belly to the tip as the tip lays it, and a tip run dry
    /// loses its point and splits.
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

    /// Soft round; the pointed tip is opt-in (`point`). Smooth, precise,
    /// little ploughing.
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

    /// Filbert: oval hog/synthetic, soft-ended marks.
    pub fn filbert(width: f32) -> Self {
        Tool { bristles: 120, stiffness: 0.6, lay: 1.2, pickup: 0.18, push: 0.18, ragged: 0.3, ..Self::base(Kind::Filbert, width) }
    }

    /// Fan: sparse bristles spread in a fan.
    pub fn fan(width: f32) -> Self {
        Tool { bristles: 36, hair: 0.6, stiffness: 0.5, lay: 0.7, pickup: 0.1, push: 0.08, splay: 0.2, ragged: 0.5, ..Self::base(Kind::Fan, width) }
    }

    /// Rigger / liner: a few very long soft hairs; fine lines.
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

#[derive(Clone)]
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
    /// Cure of the paint in it (0 fresh from the palette; paint lifted off
    /// a drying film brings the film's).
    cure: f32,
    /// Solvent it carries beside its paint (`vol`), in the same units
    /// (`crate::thinner`; engine 3): from a thinned pile, or lifted with
    /// the paint off a solvent-wet film. 0 for unthinned paint.
    pub(crate) solvent: f32,
}

/// A brush in the hand, with paint in its bristles. (`Debug` is part of
/// `easel run --state-digest`: see its impl.)
#[derive(Clone)]
pub struct Held {
    pub tool: Tool,
    pub(crate) bristles: Vec<Bristle>,
    /// Painting with engine 3 (`with_engine`): its `Debug` names each
    /// bristle's `solvent`. Not printed itself.
    shows_solvent: bool,
    /// Painting with engine 4 or later: its `Debug` gives each bristle's
    /// paint its solvent and oil too. Not printed itself.
    shows_oil: bool,
    /// Engine 7: dry pastel held in the bristles, mg (`Canvas::dust_pastel`);
    /// a wipe takes its share off. Printed only when there is some.
    pub pastel_mg: f32,
}

/// A bristle's `Debug`, as `#[derive(Debug)]` printed it before the
/// thinner, field for field, plus `solvent` when `show` (engine 3); the
/// paint's properties without solvent and oil unless `oil` (engine 4).
struct BristleText<'a>(&'a Bristle, bool, bool);

impl std::fmt::Debug for BristleText<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0;
        let hide: &[f32] = if self.2 { &b.hide } else { &b.hide[..3] };
        let mut d = f.debug_struct("Bristle");
        d.field("rx", &b.rx)
            .field("ry", &b.ry)
            .field("len", &b.len)
            .field("thresh", &b.thresh)
            .field("bend", &b.bend)
            .field("seed", &b.seed)
            .field("prev", &b.prev)
            .field("vol", &b.vol)
            .field("lat", &b.lat)
            .field("hide", &hide)
            .field("cure", &b.cure);
        if self.1 {
            d.field("solvent", &b.solvent);
        }
        d.finish()
    }
}

impl std::fmt::Debug for Bristle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        BristleText(self, true, true).fmt(f)
    }
}

/// The text of a brush (`easel run --state-digest` hashes it, and the
/// state dump keeps it): exactly what `#[derive(Debug)]` printed before the
/// thinner, so a painting from an engine before 3 digests as it did; an
/// engine-3 brush (`with_engine`) names each bristle's `solvent` too.
impl std::fmt::Debug for Held {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct List<'a>(&'a [Bristle], bool, bool);
        impl std::fmt::Debug for List<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_list().entries(self.0.iter().map(|b| BristleText(b, self.1, self.2))).finish()
            }
        }
        let mut d = f.debug_struct("Held");
        d.field("tool", &self.tool).field("bristles", &List(&self.bristles, self.shows_solvent, self.shows_oil));
        if self.pastel_mg > 0.0 {
            d.field("pastel_mg", &self.pastel_mg);
        }
        d.finish()
    }
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
                    hide: [0.5, 0.5, 1.0, 0.0, 1.0],
                    cure: 0.0,
                    solvent: 0.0,
                }
            })
            .collect();
        Held { tool, bristles, shows_solvent: false, shows_oil: false, pastel_mg: 0.0 }
    }

    /// The brush of a painting with engine `v`: from engine 3 its `Debug`
    /// names the solvent in each bristle (`crate::thinner`); before, it is
    /// the text it always was.
    pub fn with_engine(mut self, v: u32) -> Self {
        self.shows_solvent = v >= 3;
        self.shows_oil = v >= 4;
        self
    }

    /// A full load's volume for one bristle.
    pub(crate) fn full(&self) -> f32 {
        // neighboring bristles overlap by ~hair², so each lays lay / hair²
        let track = 2.0 * self.tool.hair_radius();
        self.tool.lay.max(0.3) * track * self.tool.run / (self.tool.hair * self.tool.hair)
    }

    /// Whether any bristle holds solvent (a thinned load, or solvent
    /// picked up from a thinned film).
    pub(crate) fn holds_solvent(&self) -> bool {
        self.bristles.iter().any(|b| b.solvent > 0.0)
    }

    /// Dip the brush: mix `amount` (0..1 of a full load) of `paint` into
    /// every bristle's reservoir. Bristles hold a little more or less.
    /// Thinned paint (`Paint::with_thinner`) can be loaded here, but only a
    /// canvas of engine 3 takes it: `Canvas::drag` and `touch` panic before
    /// drawing otherwise.
    pub fn load(&mut self, paint: Paint, amount: f32) {
        let lat = paint.latent();
        let full = self.full();
        let scatter = paint.scatter();
        let t = paint.thinner;
        for (i, b) in self.bristles.iter_mut().enumerate() {
            let k = 0.75 + 0.5 * crate::rng::hash2(i as i64, 17, 3);
            if t > 0.0 {
                // a thinned load: the hairs take up `amount` of liquid, the
                // share `t` of it solvent (crate::thinner)
                let v = amount * full * k;
                let vp = v * (1.0 - t);
                b.cure = mix_cure(b.cure, b.vol, 0.0, vp);
                mix_into(&mut b.vol, &mut b.lat, &mut b.hide, vp, &lat, [scatter, paint.stiff, paint.drying, paint.solvent, paint.oil]);
                b.solvent += v * t;
                continue;
            }
            b.cure = mix_cure(b.cure, b.vol, 0.0, amount * full * k);
            mix_into(&mut b.vol, &mut b.lat, &mut b.hide, amount * full * k, &lat, [scatter, paint.stiff, paint.drying, paint.solvent, paint.oil]);
        }
    }

    /// Dip only part of the brush: `amount` of `paint` goes into the bristles
    /// `part` reaches, as much as each one takes. A painter double-loads a
    /// brush this way (one side or corner in a second pile), and a brush
    /// pulled through an unevenly knifed pile takes paint up in streaks; the
    /// colors then come off side by side within one stroke and mingle as they
    /// go. With `Part::ALL` this is `load`.
    pub fn load_part(&mut self, paint: Paint, amount: f32, part: &Part) {
        let lat = paint.latent();
        let full = self.full();
        let scatter = paint.scatter();
        for (i, b) in self.bristles.iter_mut().enumerate() {
            let w = part.weight(b.rx, i);
            if w <= 0.0 {
                continue;
            }
            let k = (0.75 + 0.5 * crate::rng::hash2(i as i64, 17, 3)) * w;
            if paint.thinner > 0.0 {
                // (a thinned load, as `load`: part paint, part solvent)
                let v = amount * full * k;
                let vp = v * (1.0 - paint.thinner);
                b.cure = mix_cure(b.cure, b.vol, 0.0, vp);
                mix_into(&mut b.vol, &mut b.lat, &mut b.hide, vp, &lat, [scatter, paint.stiff, paint.drying, paint.solvent, paint.oil]);
                b.solvent += v * paint.thinner;
                continue;
            }
            b.cure = mix_cure(b.cure, b.vol, 0.0, amount * full * k);
            mix_into(&mut b.vol, &mut b.lat, &mut b.hide, amount * full * k, &lat, [scatter, paint.stiff, paint.drying, paint.solvent, paint.oil]);
        }
    }

    /// Wipe the brush on a rag: remove `frac` of the paint in it.
    pub fn wipe(&mut self, frac: f32) {
        self.pastel_mg *= 1.0 - frac.clamp(0.0, 1.0);
        for b in &mut self.bristles {
            b.vol *= 1.0 - frac.clamp(0.0, 1.0);
            b.solvent *= 1.0 - frac.clamp(0.0, 1.0);
        }
    }

    /// Wipe thoroughly and load fresh paint.
    pub fn reload(&mut self, paint: Paint, amount: f32) {
        self.wipe(0.85);
        self.load(paint, amount);
    }

    /// Paint (with any solvent in it) left in the brush, relative to a full
    /// load.
    pub fn fullness(&self) -> f32 {
        let f = self.full();
        self.bristles.iter().map(|b| b.vol + b.solvent).sum::<f32>() / (f * self.bristles.len() as f32)
    }

    /// The paint and the solvent the brush carries, in the units of
    /// `Canvas::wet_total` (coats × square units).
    pub fn carried(&self) -> (f64, f64) {
        self.bristles.iter().fold((0.0, 0.0), |(p, s), b| (p + b.vol as f64, s + b.solvent as f64))
    }
}

/// Which of a brush's bristles a dip reaches, and how much each takes up
/// (see `Held::load_part`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    /// The edge of the brush's wide axis that goes into the pile: -1 one
    /// edge, 1 the other, 0 the whole width alike.
    pub side: f32,
    /// The share of the width, from that edge, that reaches the paint (1 all).
    pub share: f32,
    /// How unevenly the bristles take it up: 0 alike, 1 in streaks (bands a
    /// few bristles wide take much more, others little or none).
    pub streak: f32,
    /// The streaks' randomness (a new one for every dip).
    pub seed: u64,
}

impl Part {
    pub const ALL: Part = Part { side: 0.0, share: 1.0, streak: 0.0, seed: 0 };

    /// How much of the dip bristle `i` (root offset `rx` across the wide
    /// axis, about -1..1) takes, relative to an even dip (about 1 on
    /// average over the part reached).
    pub fn weight(&self, rx: f32, i: usize) -> f32 {
        let mut w = 1.0;
        if self.side != 0.0 && self.share < 1.0 {
            // 0 at the far edge .. 1 at the dipped edge; a soft margin, as hairs splay
            let t = (rx * self.side.signum() + 1.0) * 0.5;
            let lo = 1.0 - self.share.clamp(0.0, 1.0);
            w *= crate::smoothstep(lo - 0.08, lo + 0.08, t);
        }
        if self.streak > 0.0 {
            // bands: neighboring bristles share a streak (a clump of hair goes in together)
            let bands = 7.0;
            let u = (rx + 1.0) * 0.5 * bands;
            let (b0, f) = (u.floor(), u - u.floor());
            let s = self.seed;
            let a = crate::rng::hash2(b0 as i64, 91, s);
            let c = crate::rng::hash2(b0 as i64 + 1, 91, s);
            let band = a + (c - a) * crate::smoothstep(0.0, 1.0, f);
            let own = crate::rng::hash2(i as i64, 92, s);
            let n = 0.75 * band + 0.25 * own; // 0..1, mean 0.5
            // n² has a mean of about 0.3: 3.3 n² keeps the dip's total about the same
            let st = self.streak.clamp(0.0, 1.0);
            w *= (1.0 - st) + st * (3.3 * n * n);
        }
        w.max(0.0)
    }
}

/// How the brush's wide axis is held.
#[derive(Clone, Copy, Debug)]
pub enum Orient {
    /// Wide axis across the direction of travel (full-width marks).
    Across,
    /// Wide axis along the travel (thin edge marks).
    Along,
    /// Fixed angle in radians, regardless of travel.
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
    /// from start to end, interpolated smoothly (empty = none).
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

/// Raw view of the wet layer and surface, so brushes working disjoint parts
/// of the canvas can run in parallel (see `Canvas::work`).
#[derive(Clone, Copy)]
pub(crate) struct Surf {
    /// The buffers' pixels: a window (origin `ox`, `oy`, size `w` × `h`) of
    /// the whole `fw` × `fh` canvas. Brush geometry works in whole-canvas
    /// pixels; only buffer indices are translated.
    w: usize,
    h: usize,
    ox: usize,
    oy: usize,
    fw: usize,
    fh: usize,
    scale: f32,
    vol: *mut f32,
    lat: *mut Latent,
    hide: *mut Prop,
    stroke: *mut u32,
    touched: *mut u32,
    floor: *mut f32,
    cover: *mut f32,
    base: *const f32,
    /// Drying state (null until the canvas has waited): how open or tacky
    /// the paint under a bristle is (see `drying::feel`).
    dry: *mut crate::drying::Px,
    /// Paint laid or moved updates the film's cure at once (see `add`;
    /// engine 2). Engine 1 left it to the next `wait`.
    cure_now: bool,
    /// Engine 4: bristles in stiff paint gather into clumps that lay it in
    /// ridges and furrows and plough it aside a clump's width (`exchange`).
    clump: bool,
    /// Engine 4: paint laid loses its solvent at once (it evaporates in
    /// minutes), and an absorbent ground draws oil out of it (`add`).
    lean: bool,
    /// The ground's remaining absorbency per pixel (coats of oil; null when
    /// none of it is absorbent or before engine 4).
    absorb: *mut f32,
    /// Solvent in the open film, and each pixel's wet film laid by the
    /// stroke `laid_id` against its ceiling (`Wet::solv`, `Wet::laid`):
    /// null before engine 3.
    solv: *mut f32,
    laid: *mut f32,
    laid_id: *mut u32,
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
    /// Lay `v` coats of paint of cure `cure` (0: fresh) at pixel `i`: it
    /// mixes by volume into the color and into the film's cure, so the next
    /// bristle feels the film as it now is, not as it was at the last wait.
    #[inline]
    unsafe fn add(&self, i: usize, v: f32, lat: &Latent, hide: Prop, cure: f32) {
        unsafe {
            if v <= 0.0 {
                return;
            }
            // engine 4: the solvent in the paint evaporates as it is laid
            // (in minutes: before anything else can work it), leaving a film
            // of the paint's own body that much thinner
            let (v, hide) = if self.lean && hide[3] > 0.0 {
                let mut h = hide;
                h[3] = 0.0;
                (v * (1.0 - hide[3].clamp(0.0, 0.95)), h)
            } else {
                (v, hide)
            };
            let vol = &mut *self.vol.add(i);
            let l = &mut *self.lat.add(i);
            let hd = &mut *self.hide.add(i);
            let t = *vol + v;
            let a = v / t;
            for k in 0..LAT {
                l[k] += (lat[k] - l[k]) * a;
            }
            for k in 0..hd.len() {
                hd[k] += (hide[k] - hd[k]) * a;
            }
            if self.cure_now && !self.dry.is_null() {
                // (a film too thin for `wait` to count is bare: no cure)
                let p = &mut *self.dry.add(i);
                p.cure = if t < 1e-5 { 0.0 } else { p.cure + (cure - p.cure) * a };
            }
            *vol = t;
            // engine 4: an absorbent ground under the film draws oil out of
            // the paint just laid, until its pores are full: a thin wash goes
            // lean (stiff, matte, quick to set), thick paint barely notices
            if !self.absorb.is_null() {
                let cap = &mut *self.absorb.add(i);
                if *cap > 0.0 {
                    let oil_in = v * OIL_SHARE * hd[4].max(0.0);
                    let take = (*cap).min(0.8 * oil_in);
                    if take > 0.0 {
                        *cap -= take;
                        let film_oil = (t * OIL_SHARE * hd[4].max(0.0)).max(1e-9);
                        let lean = (take / film_oil).min(0.9);
                        hd[4] *= 1.0 - lean;
                        hd[1] = (hd[1] * (1.0 + lean) * (1.0 + lean)).min(1.0);
                        *vol = (t - take).max(0.0);
                    }
                }
            }
        }
    }

    /// Lift `v` coats off pixel `i`. The paint left keeps its cure; a pixel
    /// left bare (as `wait` judges it) holds no film to have one.
    #[inline]
    unsafe fn take(&self, i: usize, v: f32) {
        unsafe {
            let vol = &mut *self.vol.add(i);
            *vol -= v;
            if self.cure_now && *vol < 1e-5 && !self.dry.is_null() {
                (*self.dry.add(i)).cure = 0.0;
            }
        }
    }
}

impl Canvas {
    pub(crate) fn surf(&mut self) -> Surf {
        // the raw views below index 0..w*h: every buffer must be that long
        let n = self.f.w * self.f.h;
        if self.engine >= 3 {
            self.wet.ensure_solvent();
        }
        let wt = &self.wet;
        assert!(
            [self.height.len(), self.px.len(), self.film.len(), wt.vol.len(), wt.lat.len(), wt.hide.len(), wt.stroke.len(), wt.touched.len(), wt.floor.len(), wt.cover.len()].iter().all(|&l| l == n),
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
            let low = contact_level(&self.height, w, h, r);
            base.par_iter_mut().zip(self.height.par_iter().zip(low.par_iter())).for_each(|(b, (&hgt, &lo))| {
                *b = (0.5 + (hgt - lo) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3);
            });
            self.base = Some((self.surf_gen, base));
        }
        Surf {
            w: self.f.w,
            h: self.f.h,
            ox: self.f.x0,
            oy: self.f.y0,
            fw: self.f.full_w,
            fh: self.f.full_h,
            scale: self.f.scale,
            vol: self.wet.vol.as_mut_ptr(),
            lat: self.wet.lat.as_mut_ptr(),
            hide: self.wet.hide.as_mut_ptr(),
            stroke: self.wet.stroke.as_mut_ptr(),
            touched: self.wet.touched.as_mut_ptr(),
            floor: self.wet.floor.as_mut_ptr(),
            cover: self.wet.cover.as_mut_ptr(),
            base: self.base.as_ref().unwrap().1.as_ptr(),
            dry: if self.wet.clock.px.len() == n { self.wet.clock.px.as_mut_ptr() } else { std::ptr::null_mut() },
            cure_now: self.engine >= 2,
            clump: self.engine >= 4,
            lean: self.engine >= 4,
            absorb: if self.engine >= 4 && self.absorb_any { self.absorb.as_mut_ptr() } else { std::ptr::null_mut() },
            solv: if self.wet.solv.len() == n { self.wet.solv.as_mut_ptr() } else { std::ptr::null_mut() },
            laid: if self.wet.laid.len() == n { self.wet.laid.as_mut_ptr() } else { std::ptr::null_mut() },
            laid_id: if self.wet.laid_id.len() == n { self.wet.laid_id.as_mut_ptr() } else { std::ptr::null_mut() },
        }
    }

    pub(crate) fn next_stroke_ids(&mut self, n: u32) -> u32 {
        let old = self.wet.current;
        let first = old.wrapping_add(1).max(1);
        self.wet.current = first.wrapping_add(n);
        if first <= old || self.wet.current < first {
            // the ids wrapped: a stroke's ceiling (`Wet::laid`) must not
            // find an old stroke's entry under a reused id
            self.wet.forget_laid();
        }
        first
    }

    /// Panics if solvent would go onto a canvas that can't hold it:
    /// thinner is engine 3 only (`crate::thinner`), and before engine 3 the
    /// film has no solvent buffer, so the brush's solvent would vanish.
    pub(crate) fn assert_thinner_supported(&self, thinned: bool, what: &str) {
        assert!(!thinned || self.engine >= 3, "{what}: thinned paint needs engine 3 (this canvas is engine {}, which has no solvent in its film)", self.engine);
    }

    /// Drag a held brush through a gesture, working the wet paint.
    ///
    /// Panics if the brush holds solvent (a thinned load) and the canvas's
    /// engine is before 3.
    pub fn drag(&mut self, held: &mut Held, g: &Gesture, clip: Option<&Mask>) {
        held.tool.assert_valid();
        if self.engine < 3 {
            self.assert_thinner_supported(held.holds_solvent(), "Canvas::drag");
        }
        if let Some(m) = clip {
            self.check_mask(m);
        }
        self.tally.stroke(&held.tool, &g.pts, self.mm_per_unit);
        let id = self.next_stroke_ids(1);
        let surf = self.surf();
        let mut scratch = Vec::new();
        // SAFETY: exclusive &mut self, single brush.
        let b = unsafe { drag_on(surf, held, g, clip.map(Clip::Mask), id, &mut scratch) };
        if let Some((x0, y0, x1, y1)) = b {
            self.wet.touch(x0, y0, x1, y1);
        }
    }
}

/// Where a stroke may lay paint: a mask multiplying the bristles' contact
/// (a stencil: the region's edge exactly, the same for every stroke), or a
/// fence (`crate::fence`): the region's edge, overrun by this stroke's own
/// amount `u` (0..1), the hairs lifting off the weave past it; `limit` is a
/// hard mask on top (what is in front).
#[derive(Clone, Copy)]
pub(crate) enum Clip<'a> {
    Mask(&'a Mask),
    Fence { fence: &'a crate::fence::Fence, u: f32, limit: Option<&'a Mask> },
}

impl Clip<'_> {
    /// (contact factor, lift) at whole-canvas pixel `i`. (Always inlined:
    /// the plough calls it per destination pixel; as a call it was a sixth
    /// of a replay's time.)
    #[inline(always)]
    pub(crate) fn at(&self, i: usize) -> (f32, f32) {
        match self {
            Clip::Mask(m) => (m.data[i], 0.0),
            Clip::Fence { fence, u, limit } => {
                let (k, lift) = fence.at(i, *u);
                (k * limit.map_or(1.0, |l| l.data[i]), lift)
            }
        }
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
    let fill = held.bristles.iter().map(|b| b.vol + b.solvent).sum::<f32>() / (full * n);
    smoothstep(0.02, 0.2, fill)
}

/// Cure of `v` coats of paint of cure `c` mixed into `vol` coats of cure
/// `cure`.
#[inline]
fn mix_cure(cure: f32, vol: f32, c: f32, v: f32) -> f32 {
    if v <= 0.0 { cure } else { cure + (c - cure) * v / (vol + v) }
}

/// Capillary feed: paint in a soft tuft runs from full hairs to spent ones
/// (the belly feeds the tip). Moves the share `k` of every hair's paint into
/// a common pool and shares it out evenly; volume is conserved exactly and
/// colors mix through the tuft.
fn feed(bristles: &mut [Bristle], k: f32) {
    if k <= 0.0 || bristles.is_empty() {
        return;
    }
    let (mut tv, mut lat, mut hide, mut cure, mut ts) = (0.0f32, [0.0f32; LAT], [0.0f32; 5], 0.0f32, 0.0f32);
    for b in bristles.iter() {
        tv += b.vol;
        ts += b.solvent;
        cure += b.cure * b.vol;
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
    hide = [hide[0] / tv, hide[1] / tv, hide[2] / tv, hide[3] / tv, hide[4] / tv];
    cure /= tv;
    let share = k * tv / bristles.len() as f32;
    let share_s = k * ts / bristles.len() as f32;
    for b in bristles.iter_mut() {
        if ts > 0.0 {
            b.solvent = b.solvent * (1.0 - k) + share_s;
        }
        b.vol *= 1.0 - k;
        b.cure = mix_cure(b.cure, b.vol, cure, share);
        mix_into(&mut b.vol, &mut b.lat, &mut b.hide, share, &lat, hide);
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
    clip: Option<Clip<'_>>,
    id: u32,
    scratch: &mut Vec<f32>,
) -> Bounds {
    g.assert_valid();
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
    // a pointed tool's hairs are drawn at their true size, however far
    // below a pixel (see `strip_cover`)
    let rb = (tool.hair_radius() * s).max(if tool.point > 0.0 { FINE_RB } else { 0.55 });
    let full = held.full();
    let step = rb.max(1.25);
    let nsteps = ((total / step).ceil() as usize).max(1);
    let bend_len = tool.length * (0.25 + 0.75 * (1.0 - tool.stiffness));
    let lim = limit(footprint_checked(&tool, &g.pts, g.shake, s, sf.fw, sf.fh));

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
    // the hair that comes down first: the tip. However lightly the brush is
    // pressed or lifted, the tip is on the canvas until the hand leaves it
    // at the path's end (a lift-off draws down to the tip, it doesn't stop
    // short of the end where the pressure falls below the tip's own length)
    let tip = held.bristles.iter().enumerate().min_by(|a, b| a.1.thresh.total_cmp(&b.1.thresh)).map_or(0, |(i, _)| i);
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
        let coh = cohesion(held, full);
        let half = tool.width * 0.5 * s * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5)) * cone(&tool, p, coh);
        // bend relaxes toward its target: a rate in 0..1 keeps it a blend of
        // targets, within the reach `footprint` allows for
        let rate = (1.0 - (-(step / s) / (bend_len.max(0.0) + 1e-3)).exp()).clamp(0.0, 1.0);

        // where each touching hair meets the canvas this step
        for (bi, b) in held.bristles.iter_mut().enumerate() {
            let reach = (p - b.thresh) / (1.0 - b.thresh).max(1e-3);
            if reach <= 0.0 && !(bi == tip && p > 0.0) {
                b.prev = [None, None];
                contact[bi] = None;
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
            contact[bi] = Some(((root.0 + b.bend.0 * 0.6, root.1 + b.bend.1 * 0.6), reach.max(0.0)));
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
            unsafe { exchange(sf, b, &tool, (prev.0 + sx, prev.1 + sy), (cur.0 + sx, cur.1 + sy), rk, reach, full, None, 1.0, clip, id, scratch, &mut bounds, lim) };
            b.prev[0] = Some(cur);
        }
        feed(&mut held.bristles, feed_k);
    }
    // whole-canvas pixels → buffer pixels
    bounds.map(|(x0, y0, x1, y1)| (x0 - sf.ox, y0 - sf.oy, x1 - sf.ox, y1 - sf.oy))
}

/// How much of a capsule's contact a bristle is assumed to make outside a
/// crop window (where there is no canvas to feel).
const GHOST_TOUCH: f32 = 0.8;

/// Summed coverage of the capsule a–b (radius rb, pixels) over rect `r`:
/// its geometric footprint, whatever the canvas under it.
fn capsule_cover(a: (f32, f32), b: (f32, f32), rb: f32, fine: bool, r: (usize, usize, usize, usize)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let seg2 = dx * dx + dy * dy;
    let mut sum = 0.0f32;
    for y in r.1..r.3 {
        for x in r.0..r.2 {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let t = if seg2 > 1e-8 { (((px - a.0) * dx + (py - a.1) * dy) / seg2).clamp(0.0, 1.0) } else { 0.0 };
            let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
            let dist = (qx * qx + qy * qy).sqrt();
            if fine {
                sum += fine_cover(a, b, rb, px, py);
            } else if dist <= rb + 0.5 {
                sum += 1.0 - smoothstep(rb * 0.5, rb + 0.5, dist);
            }
        }
    }
    sum
}

/// Smallest hair radius (pixels) a pointed tool is drawn with.
const FINE_RB: f32 = 0.02;

/// Share of a pixel covered by a hair's track of radius `rb` (pixels) whose
/// center line passes `dist` from the pixel center: the overlap of the
/// pixel's span with the track's, box filtered. Summed across the track it
/// is its width, wherever the track falls between pixel centers, so a track
/// finer than a pixel lays the same paint per length on any grid (no dark
/// dots where it happens to hit a pixel center, no beads at full size).
fn strip_cover(dist: f32, rb: f32) -> f32 {
    ((dist + 0.5).min(rb) - (dist - 0.5).max(-rb)).clamp(0.0, 1.0)
}

/// Share of the pixel centered at (`px`, `py`) covered by a pointed tool's
/// hair moving from `a` to `b` (pixels): the area of its track, a rectangle
/// 2·`rb` wide with square ends, inside the (axis-aligned) pixel. Exact, so
/// the shares of all pixels sum to the track's area however it lies across
/// the lattice (a diagonal track shifted half a pixel covers what it did),
/// and a hair's successive steps tile its path without overlapping: the
/// paint a step lays and the area it darkens don't depend on where the
/// pixel centers fall. A hair that doesn't move covers a square 2·`rb` wide.
fn fine_cover(a: (f32, f32), b: (f32, f32), rb: f32, px: f32, py: f32) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let seg = (dx * dx + dy * dy).sqrt();
    let (ux, uy, lo, hi) = if seg > 1e-4 { (dx / seg, dy / seg, 0.0, seg) } else { (1.0, 0.0, -rb, rb) };
    // the pixel center in track coordinates: along `t`, across `q`
    let (rx, ry) = (px - a.0, py - a.1);
    let t = rx * ux + ry * uy;
    let q = ry * ux - rx * uy;
    // (the pixel reaches √½ from its center in any direction)
    const R: f32 = std::f32::consts::FRAC_1_SQRT_2;
    if q.abs() > rb + R || t < lo - R || t > hi + R {
        return 0.0;
    }
    // axis-aligned track: a product of spans
    if ux.abs() < 1e-6 || uy.abs() < 1e-6 {
        return strip_cover(q.abs(), rb) * ((t + 0.5).min(hi) - (t - 0.5).max(lo)).clamp(0.0, 1.0);
    }
    // the track's corners relative to the pixel center, clipped to the
    // pixel (Sutherland–Hodgman against its four sides), shoelace area
    let (nx, ny) = (-uy, ux);
    let corner = |s: f32, r: f32| (a.0 - px + ux * s + nx * r, a.1 - py + uy * s + ny * r);
    let mut poly = [(0.0f32, 0.0f32); 8];
    let mut n = 4;
    poly[..4].copy_from_slice(&[corner(lo, -rb), corner(hi, -rb), corner(hi, rb), corner(lo, rb)]);
    for side in 0..4 {
        // inside: sign·coordinate ≤ ½ along x (sides 0, 1) or y (2, 3)
        let (axis, sign) = (side / 2, if side % 2 == 0 { 1.0f32 } else { -1.0 });
        let d = |p: (f32, f32)| sign * if axis == 0 { p.0 } else { p.1 } - 0.5;
        let mut out = [(0.0f32, 0.0f32); 8];
        let mut m = 0;
        for k in 0..n {
            let (p, q) = (poly[k], poly[(k + 1) % n]);
            let (dp, dq) = (d(p), d(q));
            if dp <= 0.0 {
                out[m] = p;
                m += 1;
            }
            if (dp <= 0.0) != (dq <= 0.0) && m < 8 {
                let s = dp / (dp - dq);
                out[m] = (p.0 + (q.0 - p.0) * s, p.1 + (q.1 - p.1) * s);
                m += 1;
            }
        }
        n = m;
        if n < 3 {
            return 0.0;
        }
        poly = out;
    }
    let mut area = 0.0f32;
    for k in 0..n {
        let (p, q) = (poly[k], poly[(k + 1) % n]);
        area += p.0 * q.1 - q.0 * p.1;
    }
    (0.5 * area.abs()).clamp(0.0, 1.0)
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
/// swept from `a` to `b` (pixels). A moving bristle deposits in proportion
/// to the distance it travels; `dep` instead fixes the volume (units² ×
/// coats) it would lay on full contact (a touch: film splitting).
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
    dep: Option<f32>,
    excl: f32,
    clip: Option<Clip<'_>>,
    id: u32,
    wts: &mut Vec<f32>,
    bounds: &mut Bounds,
    lim: Rect,
) {
    unsafe {
        // pixel coordinates are whole-canvas ones; buffer index of (x, y) is
        // (y - oy) * bw + x - ox
        let (w, h) = (sf.fw, sf.fh);
        let (ox, oy, bw_buf) = (sf.ox, sf.oy, sf.w);
        let s = sf.scale;
        let px_area = 1.0 / (s * s);
        let cx0 = ((a.0.min(b.0) - rb - 1.0).floor().max(0.0)) as usize;
        let cy0 = ((a.1.min(b.1) - rb - 1.0).floor().max(0.0)) as usize;
        let cx1 = ((a.0.max(b.0) + rb + 1.0).ceil().max(0.0) as usize).min(w);
        let cy1 = ((a.1.max(b.1) + rb + 1.0).ceil().max(0.0) as usize).min(h);
        if cx1 <= cx0 || cy1 <= cy0 {
            return;
        }
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        // a crop render holds only a window of the canvas: clip to it
        let (x0, y0, x1, y1) = (cx0.max(ox), cy0.max(oy), cx1.min(ox + sf.w), cy1.min(oy + sf.h));
        let windowed = (x0, y0, x1, y1) != (cx0, cy0, cx1, cy1);
        if x1 <= x0 || y1 <= y0 {
            // outside the window the canvas isn't there to feel: assume the
            // bristle touched and laid paint as usual (so it arrives in the
            // window about as spent as in a whole render), lifting none
            let travel = (seg / s).max(rb / s * 0.5);
            if br.solvent > 0.0 {
                // (a thinned load: its liquid runs down the same way, paint
                // and solvent alike)
                let l = br.vol + br.solvent;
                let left = match dep {
                    None => l * (1.0 - (1.0 - (-travel / tool.run).exp()) * GHOST_TOUCH),
                    Some(v) => (l - v.min(l * 0.5) * GHOST_TOUCH).max(0.0),
                };
                let f = if l > 0.0 { left / l } else { 0.0 };
                br.vol *= f;
                br.solvent *= f;
                return;
            }
            br.vol = match dep {
                None => br.vol * (1.0 - (1.0 - (-travel / tool.run).exp()) * GHOST_TOUCH),
                Some(v) => (br.vol - v.min(br.vol * 0.5) * GHOST_TOUCH).max(0.0),
            };
            return;
        }
        // never outside the stroke's footprint: the scheduler runs strokes
        // whose footprints don't overlap at once
        debug_assert!(
            x0 >= lim.0 && y0 >= lim.1 && x1 <= lim.2 && y1 <= lim.3,
            "bristle contact ({x0},{y0},{x1},{y1}) outside the stroke footprint {lim:?}"
        );
        let (x0, y0, x1, y1) = (x0.max(lim.0), y0.max(lim.1), x1.min(lim.2), y1.min(lim.3));
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let (mx, my) = if seg > 1e-4 { (dx / seg, dy / seg) } else { (0.0, 0.0) };
        let (nx, ny) = (-my, mx);
        // a pointed tool's moving hair covers pixels by the exact share of
        // its track in them (see `strip_cover`)
        let fine = tool.point > 0.0 && dep.is_none();
        // lowest surface height this bristle reaches down to. The loaded tip
        // of a pointed soft brush carries a bead of paint that wets the
        // weave's valleys as well as its peaks, however lightly it is
        // pressed; run dry, it skims the peaks
        // pressed; run dry, it skims the peaks. Any loaded hair carries
        // paint proud of itself: the paint touches before the hair does, so
        // a well-loaded blunt brush wets the shallow hollows too (up to half
        // the tooth's range); a nearly dry one drags over the peaks only
        // (dry brush, broken color)
        // what the hair carries: its paint, and with a thinned load the
        // solvent in it too (crate::thinner): a thinned brush is a wet one.
        // Unthinned (no solvent), exactly the paint, as before
        let liquid = if br.solvent > 0.0 { br.vol + br.solvent } else { br.vol };
        // the share of solvent in what it lays, and the most wet film this
        // stroke may add to a pixel (coats; engine 3)
        let phi = if br.solvent > 0.0 { br.solvent / liquid } else { 0.0 };
        let capped = phi > 0.0 && !sf.laid.is_null();
        let cap = if capped { crate::thinner::stroke_limit_um(phi) / COAT_UM } else { f32::INFINITY };
        let wet = smoothstep(0.1, 0.8, liquid / full);
        let wick = if tool.point > 0.0 { tool.point * smoothstep(0.02, 0.25, liquid / full) } else { 0.0 }.max(WET_REACH * wet);
        let th = 1.0 - reach.max(wick) * 1.6;

        // pass 1: contact weights
        let bw = x1 - x0;
        wts.clear();
        wts.resize(bw * (y1 - y0), 0.0);
        let mut sum_w = 0.0f32;
        let mut sum_cov = 0.0f32;
        let mut sum_tack = 0.0f32;
        // past a fence a lifting brush lays a thinner film too (see `Clip`)
        let mut sum_k = 0.0f32;
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let t = if seg2 > 1e-8 { (((px - a.0) * dx + (py - a.1) * dy) / seg2).clamp(0.0, 1.0) } else { 0.0 };
                let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
                let dist = (qx * qx + qy * qy).sqrt();
                // a moving bristle has a crisp track; in a pressed tip (fixed
                // deposit) paint wicks between the hairs, so each hair's
                // contact fades out and neighbors sum to one smooth patch
                let cov = if fine {
                    fine_cover(a, b, rb, px, py)
                } else if dist > rb + 0.5 {
                    continue;
                } else if dep.is_some() {
                    1.0 - smoothstep(0.0, rb, dist)
                } else {
                    1.0 - smoothstep(rb * 0.5, rb + 0.5, dist)
                };
                if cov <= 0.0 {
                    continue;
                }
                sum_cov += cov;
                let i = (y - oy) * bw_buf + x - ox;
                let surf = (*sf.base.add(i) + 0.35 * *sf.vol.add(i)).min(1.5);
                // soft hair bends down into the valleys of the weave; stiff hog
                // bristles ride on the peaks
                let give = 0.15 + 0.45 * (1.0 - tool.stiffness).clamp(0.0, 1.0);
                let wt = match clip {
                    None => cov * smoothstep(th - give, th + 0.2, surf),
                    Some(Clip::Mask(m)) => cov * smoothstep(th - give, th + 0.2, surf) * m.data[y * w + x],
                    // past a fence the hairs lift off the weave's hollows
                    Some(c @ Clip::Fence { .. }) => {
                        let (k, lift) = c.at(y * w + x);
                        let th = th + lift;
                        let wt = cov * smoothstep(th - give, th + 0.2, surf) * k;
                        sum_k += wt * k * k;
                        wt
                    }
                };
                wts[(y - y0) * bw + (x - x0)] = wt;
                sum_w += wt;
                if !sf.dry.is_null() {
                    let p = *sf.dry.add(i);
                    sum_tack += wt * crate::drying::feel(*sf.vol.add(i), p.cure, p.sub).1;
                }
            }
        }
        if sum_w <= 1e-6 {
            return;
        }
        // a tacky surface grabs: it pulls paint off the bristle faster, in
        // patches as the bristle sticks and slips
        let tack = sum_tack / sum_w;

        // deposit: a share of the load, proportional to distance traveled
        let travel = (seg / s).max(rb / s * 0.5);
        // only the part of the footprint actually in contact takes paint: a
        // bristle skimming the weave peaks keeps most of its load
        let touch = (sum_w / sum_cov.max(1e-6)).min(1.0);
        let dep_total = match dep {
            None => liquid * (1.0 - (-travel / tool.run).exp()) * touch,
            Some(v) => v.min(liquid * 0.5) * touch,
        };
        let dep_total = if matches!(clip, Some(Clip::Fence { .. })) { dep_total * (sum_k / sum_w).min(1.0) } else { dep_total };
        let dep_total = if tack > 0.0 {
            let g = crate::drying::grab(tack) * crate::drying::stick(b.0, b.1, rb, br.seed, tack);
            let d = match dep {
                None => liquid * (1.0 - (-travel * g / tool.run).exp()) * touch,
                Some(_) => dep_total * g,
            };
            d.min(liquid * 0.9)
        } else {
            dep_total
        };
        // engine 4: in stiff paint the hairs gather into clumps, a few hairs
        // to a clump, more the stiffer the paint; a clump lays more and the
        // gaps between clumps less, so the stroke lies in ridges and furrows
        // along its length (the variation averages out across the brush).
        // A pointed tip's few hairs lie together already.
        // (the paint as it is on the brush: solvent makes it flow)
        let ps = (br.hide[1] * (1.0 - br.hide[3]).powi(2)).clamp(0.0, 1.0);
        // how far the paint is stiff enough to hold hairs together: none in
        // fluid paint, rising steeply in stiff
        let stiffen = ((ps - 0.4) / 0.6).clamp(0.0, 1.0);
        let dep_total = if sf.clump && dep.is_none() && !fine && stiffen > 0.0 {
            let clump_u = tool.hair_radius() * (1.0 + 4.0 * stiffen * stiffen);
            let bands = (tool.width / (2.0 * clump_u)).clamp(2.0, 48.0);
            let u = (br.rx.clamp(-1.0, 1.0) + 1.0) * 0.5 * bands;
            let (b0, f) = (u.floor(), u - u.floor());
            let h0 = crate::rng::hash2(b0 as i64, id as i64, 0x51C0);
            let h1 = crate::rng::hash2(b0 as i64 + 1, id as i64, 0x51C0);
            let band = h0 + (h1 - h0) * crate::smoothstep(0.0, 1.0, f);
            (dep_total * (1.0 + 0.6 * stiffen * (2.0 * band - 1.0))).min(br.vol * 0.95)
        } else {
            dep_total
        };
        // a capsule cut by the window edge lays only the window's share there
        let share = if windowed { sum_cov / capsule_cover(a, b, rb, fine, (cx0, cy0, cx1, cy1)).max(1e-6) } else { 1.0 };
        let dep_per_w = dep_total * share.min(1.0) / sum_w / px_area;
        // film splitting: a bristle in wet paint always lifts some of it, even
        // when loaded; a spent bristle drinks more
        let hunger = 0.35 + 0.65 * (1.0 - liquid / full).clamp(0.0, 1.0).powf(1.5);
        // a moving hair ploughs aside the share `push` of the paint in its
        // own track, 2·`hair` wide, and lays it a hair's width away (shared
        // bilinearly, below): the same paint moved the same distance at any
        // resolution. A hair finer than a pixel is drawn wider than it is
        // (`rb`, see `drag_on`), so each pixel of its drawn track gives up
        // only its share of it, hair / rb. (Ploughing by the whole drawn
        // track and throwing it to the next pixel would move many times the
        // paint a hair's width does, e.g. some 18× for a filbert 5 at 1000px
        // whose hairs are a quarter of a pixel, compounding over the ~50
        // hairs over each pixel into rims along the stroke's edges.) A
        // pressed tip's hairs lie together
        // (see `touch_rb`): its contact is its track.
        let hair = if fine || dep.is_some() { rb } else { (tool.hair_radius() * s).min(rb) };
        let push_k = tool.push * (seg / (2.0 * hair)).clamp(0.0, 1.0) * (hair / rb);
        // (engine 4: each hair still moves only its own share of paint, but a
        // clump of hairs in stiff paint throws it aside the clump's width,
        // up to a few pixels: walls along the stroke's edges, a bead ahead)
        // (only the hairs along the brush's sides: inside it, a clump's
        // neighbors hem it in, and its paint stays in the stroke)
        let off = if dep.is_some() {
            rb + 1.0
        } else if sf.clump && stiffen > 0.0 && !fine && br.rx.abs() > 0.7 {
            (2.0 * tool.hair_radius() * s * (1.0 + 4.0 * stiffen * stiffen)).max(2.0 * hair).min(3.0)
        } else {
            2.0 * hair
        };
        let spread = dep.is_none();

        let mut got_v = 0.0f32;
        let mut got_l = [0.0f32; LAT];
        let mut got_h: Prop = [0.0; 5];
        let mut got_c = 0.0f32;
        // solvent lifted with the paint, and wet film (paint + solvent) laid
        let mut got_s = 0.0f32;
        let mut laid_liq = 0.0f32;
        let (blat, bhide, bcure) = (br.lat, br.hide, br.cure);
        for y in y0..y1 {
            for x in x0..x1 {
                let wt = wts[(y - y0) * bw + (x - x0)];
                if wt <= 0.0 {
                    continue;
                }
                let i = (y - oy) * bw_buf + x - ox;
                let v = *sf.vol.add(i);
                // paint that is setting is stiff: it comes up and moves less
                let fl = if sf.dry.is_null() { 1.0 } else { crate::drying::fluid((*sf.dry.add(i)).cure) };
                // one stroke lifts only part of the film
                if *sf.touched.add(i) != id {
                    *sf.touched.add(i) = id;
                    *sf.floor.add(i) = v * (1.0 - tool.pickup * fl);
                }
                if v > 1e-6 {
                    let own = if *sf.stroke.add(i) == id { 0.15 } else { 1.0 };
                    let take = (v * tool.pickup * wt * hunger * own * fl).min((v - *sf.floor.add(i)).max(0.0));
                    if take > 0.0 {
                        let tv = take * px_area;
                        got_v += tv;
                        let l = &*sf.lat.add(i);
                        for k in 0..LAT {
                            got_l[k] += l[k] * tv;
                        }
                        let hp = *sf.hide.add(i);
                        for k in 0..hp.len() {
                            got_h[k] += hp[k] * tv;
                        }
                        if !sf.dry.is_null() {
                            got_c += (*sf.dry.add(i)).cure * tv;
                        }
                        // the solvent comes up with the paint, in the
                        // film's own proportions
                        if !sf.solv.is_null() {
                            // (the canvas holds its solvent in µm)
                            let sv = &mut *sf.solv.add(i);
                            if *sv > 0.0 {
                                let ts = (*sv * take / v).min(*sv);
                                *sv -= ts;
                                got_s += ts / COAT_UM * px_area;
                            }
                        }
                        sf.take(i, take);
                    }
                }
                if dep_per_w > 0.0 && !capped && phi <= 0.0 {
                    // the share of the pixel this paint covers: its contact
                    // (a fine hair's own share of the tuft's width, where
                    // the hairs of a gathered point lie over each other)
                    let cv = &mut *sf.cover.add(i);
                    *cv = if fine { ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0) } else { 1.0 };
                    sf.add(i, dep_per_w * wt, &blat, bhide, bcure);
                    *sf.stroke.add(i) = id;
                } else if dep_per_w > 0.0 {
                    // a thinned load lays liquid: no more than what is left
                    // of this pixel's ceiling for this stroke, shared by all
                    // its hairs (crate::thinner); the rest stays on the hair
                    let mut d = dep_per_w * wt;
                    if capped {
                        let used = if *sf.laid_id.add(i) == id { *sf.laid.add(i) } else { 0.0 };
                        d = d.min((cap - used).max(0.0));
                        *sf.laid_id.add(i) = id;
                        *sf.laid.add(i) = used + d;
                    }
                    if d > 0.0 {
                        let cv = &mut *sf.cover.add(i);
                        *cv = if fine { ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0) } else { 1.0 };
                        sf.add(i, d * (1.0 - phi), &blat, bhide, bcure);
                        if !sf.solv.is_null() {
                            *sf.solv.add(i) += d * phi * COAT_UM;
                        }
                        *sf.stroke.add(i) = id;
                        laid_liq += d * px_area;
                    }
                }
                // plough: move paint outward from the bristle's path, and ahead
                if push_k > 0.0 {
                    let v = *sf.vol.add(i);
                    let m = v * push_k * wt * fl;
                    if m > 1e-6 {
                        let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                        let side = if (px - a.0) * nx + (py - a.1) * ny >= 0.0 { 1.0 } else { -1.0 };
                        let tx = px + (nx * side * 0.75 + mx * 0.45) * off;
                        let ty = py + (ny * side * 0.75 + my * 0.45) * off;
                        // where the paint goes: the pixel under the target,
                        // or for a moving hair, shared
                        // bilinearly by the four pixels around it (a hair
                        // finer than a pixel would otherwise leave a ridge
                        // of dots along its track where rounding lands it)
                        let mut to = [(0.0f32, 0.0f32, 0.0f32); 4];
                        let n_to = if spread {
                            let (gx, gy) = (tx - 0.5, ty - 0.5);
                            let (fx, fy) = (gx - gx.floor(), gy - gy.floor());
                            let (bx, by) = (gx.floor() + 0.5, gy.floor() + 0.5);
                            to = [(bx, by, (1.0 - fx) * (1.0 - fy)), (bx + 1.0, by, fx * (1.0 - fy)), (bx, by + 1.0, (1.0 - fx) * fy), (bx + 1.0, by + 1.0, fx * fy)];
                            4
                        } else {
                            to[0] = (tx, ty, 1.0);
                            1
                        };
                        // the source's paint, read once: nothing below writes
                        // lat or hide at `i` (`take` lowers its volume, `add`
                        // writes `j != i`). Its cure is read per destination:
                        // `take` zeroes it when the film goes bare (see the
                        // kernel_traps tests)
                        let (l, hd) = (*sf.lat.add(i), *sf.hide.add(i));
                        for &(tx, ty, share) in &to[..n_to] {
                            if share <= 0.0 {
                                continue;
                            }
                            // (paint pushed out of a crop window stays put)
                            let inside = tx >= ox as f32 && ty >= oy as f32 && (tx as usize) < w.min(ox + sf.w) && (ty as usize) < h.min(oy + sf.h);
                            debug_assert!(!inside || (tx >= lim.0 as f32 && ty >= lim.1 as f32 && (tx as usize) < lim.2 && (ty as usize) < lim.3), "plough target outside the stroke footprint {lim:?}");
                            if inside && tx >= lim.0 as f32 && ty >= lim.1 as f32 && (tx as usize) < lim.2 && (ty as usize) < lim.3 {
                                let (tx, ty) = (tx as usize, ty as usize);
                                let j = (ty - oy) * bw_buf + tx - ox;
                                // a clipped stroke can't push paint past its mask:
                                // only the accepted share moves, the rest stays
                                let m = m * share * clip.map_or(1.0, |c| c.at(ty * w + tx).0);
                                // a thinned stroke adds no more wet film to the
                                // pixel it ploughs into than its ceiling there
                                // allows; the solvent goes with the paint
                                // (solvent in µm, as the canvas holds it)
                                let sol = if sf.solv.is_null() { 0.0 } else { *sf.solv.add(i) / COAT_UM };
                                let vi = *sf.vol.add(i);
                                let ms = if sol > 0.0 && vi > 0.0 { (sol * m / vi).min(sol) } else { 0.0 };
                                let (m, ms) = if capped && j != i && m > 0.0 {
                                    let used = if *sf.laid_id.add(j) == id { *sf.laid.add(j) } else { 0.0 };
                                    let room = (cap - used).max(0.0);
                                    let f = if m + ms > room { room / (m + ms) } else { 1.0 };
                                    *sf.laid_id.add(j) = id;
                                    *sf.laid.add(j) = used + (m + ms) * f;
                                    (m * f, ms * f)
                                } else {
                                    (m, ms)
                                };
                                if j != i && m > 0.0 {
                                    if ms > 0.0 {
                                        let si = &mut *sf.solv.add(i);
                                        let mu = (ms * COAT_UM).min(*si);
                                        *si -= mu;
                                        *sf.solv.add(j) += mu;
                                    }
                                    let cure = if sf.dry.is_null() { 0.0 } else { (*sf.dry.add(i)).cure };
                                    // the paint moved covers its share of
                                    // the pixel it came from
                                    // (its film there thins but still covers it)
                                    let cj = &mut *sf.cover.add(j);
                                    *cj = if fine { ((if *sf.vol.add(j) < 1e-6 { 0.0 } else { *cj }) + (m / v.max(1e-9)).min(1.0) * *sf.cover.add(i)).min(1.0) } else { 1.0 };
                                    sf.take(i, m);
                                    sf.add(j, m, &l, hd, cure);
                                }
                            }
                        }
                    }
                }
            }
        }
        if phi > 0.0 {
            // the hair loses what it laid (and in a crop, the share laid
            // outside the window), paint and solvent in its proportions
            let gone = (laid_liq + dep_total * (1.0 - share.min(1.0))).min(liquid);
            br.vol = (br.vol - gone * (1.0 - phi)).max(0.0);
            br.solvent = (br.solvent - gone * phi).max(0.0);
        } else {
            br.vol = (br.vol - dep_total).max(0.0);
        }
        br.solvent += got_s;
        if got_v > 0.0 {
            for k in 0..LAT {
                got_l[k] /= got_v;
            }
            for g in &mut got_h {
                *g /= got_v;
            }
            br.cure = mix_cure(br.cure, br.vol, got_c / got_v, got_v);
            mix_into(&mut br.vol, &mut br.lat, &mut br.hide, got_v, &got_l, got_h);
        }
        let pad = (off + 2.0) as usize;
        grow(bounds, x0.saturating_sub(pad).max(ox), y0.saturating_sub(pad).max(oy), (x1 + pad).min(ox + sf.w), (y1 + pad).min(oy + sf.h));
    }
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
    ///
    /// Panics if the brush holds solvent and the canvas's engine is before
    /// 3 (as `drag`).
    pub fn touch(&mut self, held: &mut Held, t: &Touch, clip: Option<&Mask>) {
        held.tool.assert_valid();
        if self.engine < 3 {
            self.assert_thinner_supported(held.holds_solvent(), "Canvas::touch");
        }
        if let Some(m) = clip {
            self.check_mask(m);
        }
        self.tally.touch();
        let id = self.next_stroke_ids(1);
        let surf = self.surf();
        let mut scratch = Vec::new();
        // SAFETY: exclusive &mut self, single brush.
        let b = unsafe { touch_on(surf, held, t, clip.map(Clip::Mask), id, &mut scratch) };
        if let Some((x0, y0, x1, y1)) = b {
            self.wet.touch(x0, y0, x1, y1);
        }
    }
}

/// SAFETY: no other thread may touch pixels in `touch_footprint(..)` of `t`,
/// and `held.tool` must pass `Tool::validate` (accesses are clamped to that
/// footprint too, as in `drag_on`).
pub(crate) unsafe fn touch_on(sf: Surf, held: &mut Held, t: &Touch, clip: Option<Clip<'_>>, id: u32, scratch: &mut Vec<f32>) -> Bounds {
    t.assert_valid();
    let mut bounds: Bounds = None;
    let s = sf.scale;
    let tool = held.tool.clone();
    let full = held.full();
    let p = t.pressure.clamp(0.0, 1.0);
    let (cx, cy) = (t.at.0 * s, t.at.1 * s);
    let (dx, dy) = (t.drag.0 * s, t.drag.1 * s);
    let rb = touch_rb(&tool, p, s);
    let lim = limit(touch_footprint_checked(&tool, t, s, sf.fw, sf.fh));
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
            let fill = ((b.vol + b.solvent) / full).min(1.0);
            let v = film * fill * reach / sums[bi].max(1e-6);
            unsafe { exchange(sf, b, &tool, prev, cur, rb, reach, full, Some(v), 1.0, clip, id, scratch, &mut bounds, lim) };
            b.prev[0] = Some(cur);
        }
    }
    // whole-canvas pixels → buffer pixels
    bounds.map(|(x0, y0, x1, y1)| (x0 - sf.ox, y0 - sf.oy, x1 - sf.ox, y1 - sf.oy))
}

/// A flick of a loaded brush (spatter): the paint the hairs can't hold by
/// capillarity flies off in droplets and lands in a cone toward the flick.
///
/// What flies is physical: only the loose paint beyond what each hair holds,
/// more of it the harder the flick and the more fluid the paint (medium and
/// turpentine thin it; blotted or stiff tube paint barely leaves the brush).
/// A harder flick breaks it into more, smaller droplets. Each droplet comes
/// off one hair and carries that hair's paint, so a double-loaded brush
/// spatters both colors. Heavier droplets carry farther; those arriving at a
/// slant stretch into ovals along their flight. They land in the wet layer
/// like any paint: they mix into wet paint under them and dry with it.
#[derive(Clone, Debug)]
pub struct Spatter {
    /// Where the brush is when it is flicked (canvas units).
    pub at: (f32, f32),
    /// The flick: its direction, and its length how far the paint carries
    /// (canvas units).
    pub toward: (f32, f32),
    /// Half the angle of the cone the droplets fly in (radians).
    pub spread: f32,
    /// How hard the flick is, 0..1.
    pub force: f32,
    pub seed: u64,
}

/// A full load of a brush, in mm³ of paint, for its width in mm (a 10 mm
/// brush holds about 0.08 ml).
fn load_mm3(width_mm: f32) -> f32 {
    0.08 * width_mm.max(0.5).powi(3)
}

impl Canvas {
    /// Flick a held brush (see `Spatter`); returns how many droplets landed.
    pub fn spatter(&mut self, held: &mut Held, sp: &Spatter, clip: Option<&Mask>) -> usize {
        held.tool.assert_valid();
        if let Some(m) = clip {
            self.check_mask(m);
        }
        self.tally.touch();
        let _id = self.next_stroke_ids(1);
        let mm_per_px = self.px_mm();
        let sf = self.surf();
        let s = sf.scale;
        let mut rng = Rng::new(sp.seed);
        let force = sp.force.clamp(0.0, 1.0);
        let full = held.full();
        let n = held.bristles.len().max(1) as f32;
        let width_mm = held.tool.width * s * mm_per_px;
        // mm³ of paint per unit of bristle volume
        let per_vol = load_mm3(width_mm) / (full * n);
        // each hair's loose paint, and the share of it the flick throws
        let mut budget: Vec<f32> = held
            .bristles
            .iter()
            .map(|b| {
                let stiff = b.hide[1].clamp(0.0, 1.0);
                let fluid = ((1.0 - stiff) * (1.0 + b.hide[3].clamp(0.0, 0.9))).clamp(0.0, 1.5);
                let hold = full * (0.2 + 0.6 * stiff);
                let loose = (b.vol - hold).max(0.0);
                loose * (force.powf(0.8) * (0.15 + 0.6 * fluid)).clamp(0.0, 0.95)
            })
            .collect();
        let total: f32 = budget.iter().sum();
        if total <= 0.0 {
            return 0;
        }
        let stiff_mean = held.bristles.iter().map(|b| b.hide[1]).sum::<f32>() / n;
        // droplet sizes (radius in flight, mm): log-normal, smaller the harder the flick
        let r_med = (0.32 * (1.25 - force) * (0.6 + 0.8 * stiff_mean)).clamp(0.04, 1.2);
        let reach = (sp.toward.0 * sp.toward.0 + sp.toward.1 * sp.toward.1).sqrt().max(1e-3);
        let dir = sp.toward.1.atan2(sp.toward.0);
        let spread = sp.spread.clamp(0.0, 1.5);
        let px_mm2 = mm_per_px * mm_per_px;
        let (cx0, cy0) = (sp.at.0 * s, sp.at.1 * s);
        let mut bounds: Bounds = None;
        let mut count = 0usize;
        let mut left = total;
        let mut cum: Vec<f32> = Vec::with_capacity(budget.len());
        let mut guard = 0;
        while left > total * 0.002 && guard < 40000 {
            guard += 1;
            // the hair it comes off: by how much loose paint each still has
            cum.clear();
            let mut acc = 0.0;
            for &v in &budget {
                acc += v;
                cum.push(acc);
            }
            if acc <= 0.0 {
                break;
            }
            let pick = rng.f() * acc;
            let bi = cum.iter().position(|&c| c >= pick).unwrap_or(budget.len() - 1);
            let r = (r_med * (0.55 * rng.normal()).exp()).clamp(0.03, 2.5);
            let want = (4.0 / 3.0) * std::f32::consts::PI * r * r * r / per_vol;
            let take = want.min(budget[bi]);
            budget[bi] -= take;
            left -= take;
            let vol_mm3 = take * per_vol;
            let b = &mut held.bristles[bi];
            b.vol = (b.vol - take).max(0.0);
            // flight: off the flick's line by the cone, farther for heavier drops
            let th = dir + (0.5 * spread * rng.normal()).clamp(-1.2 * spread, 1.2 * spread);
            let d = reach * (0.35 + 0.65 * rng.f()) * (r / r_med).powf(0.25).clamp(0.5, 1.6);
            let (px, py) = (cx0 + th.cos() * d * s, cy0 + th.sin() * d * s);
            // on impact it spreads, less for stiff paint; a slanting arrival stretches it
            let stiff = b.hide[1].clamp(0.0, 1.0);
            let r_land = r * (2.2 - 0.9 * stiff) / mm_per_px;
            let stretch = 1.0 + 1.2 * force * (d / reach) * rng.f();
            let (ra, rb) = ((r_land * stretch.sqrt()).max(0.35), (r_land / stretch.sqrt()).max(0.35));
            let area_px = std::f32::consts::PI * ra * rb;
            let coats_mean = vol_mm3 / (area_px * px_mm2).max(1e-9) * 1000.0 / crate::surface::COAT_UM;
            let (ct, st) = (th.cos(), th.sin());
            let ext = ra.max(rb) + 1.0;
            let (x0, x1) = ((px - ext).floor() as i64, (px + ext).ceil() as i64);
            let (y0, y1) = ((py - ext).floor() as i64, (py + ext).ceil() as i64);
            let lat = b.lat;
            let hide = b.hide;
            let cure = b.cure;
            // dome-shaped: thickest in the middle; weights normalised so the droplet's volume lands
            let mut cells: Vec<(usize, usize, f32)> = Vec::new();
            let mut wsum = 0.0;
            for y in y0..=y1 {
                for x in x0..=x1 {
                    if x < 0 || y < 0 || x as usize >= sf.fw || y as usize >= sf.fh {
                        continue;
                    }
                    let (dx, dy) = (x as f32 + 0.5 - px, y as f32 + 0.5 - py);
                    let (u, v) = ((dx * ct + dy * st) / ra, (-dx * st + dy * ct) / rb);
                    let q = u * u + v * v;
                    let w = if q < 1.0 { (1.0 - q).sqrt() } else if ra < 1.0 && q < 2.0 { 0.15 * (2.0 - q) } else { 0.0 };
                    if w <= 0.0 {
                        continue;
                    }
                    wsum += w;
                    cells.push((x as usize, y as usize, w));
                }
            }
            if wsum <= 0.0 {
                continue;
            }
            let mut landed = false;
            for (x, y, w) in cells {
                if x < sf.ox || y < sf.oy || x >= sf.ox + sf.w || y >= sf.oy + sf.h {
                    continue;
                }
                let m = clip.map(|m| m.data[y * sf.fw + x]).unwrap_or(1.0);
                if m <= 0.0 {
                    continue;
                }
                let i = (y - sf.oy) * sf.w + (x - sf.ox);
                let v = coats_mean * area_px * w / wsum * m;
                // SAFETY: exclusive &mut self; i is inside the buffer window.
                unsafe { sf.add(i, v, &lat, hide, cure) };
                grow(&mut bounds, x - sf.ox, y - sf.oy, x - sf.ox + 1, y - sf.oy + 1);
                landed = true;
            }
            if landed {
                count += 1;
            }
        }
        if let Some((x0, y0, x1, y1)) = bounds {
            self.wet.touch(x0, y0, x1, y1);
        }
        count
    }
}

#[cfg(test)]
mod tip_tests {
    use super::*;
    use crate::color::hex;

    /// The pointed tip is opt-in: these tests use it explicitly.
    fn sable(w: f32) -> Tool {
        Tool { point: 1.0, ..Tool::round_sable(w) }
    }
    fn rigger(w: f32) -> Tool {
        Tool { point: 1.0, ..Tool::rigger(w) }
    }

    const BG: &str = "#e8e0d0";
    const INK: &str = "#1a1612";

    /// A narrow strip of a 440 mm wide canvas with one
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

    /// Paint (coats) one stroke of `tool` lays across its mark over a dry
    /// layer on a strip of a 440 mm wide canvas, `px` wide: the mean film at each
    /// pixel row from 2 widths above the stroke's line to 2 below, over the
    /// middle of its length, as (offset from the line in units, coats).
    #[cfg(tube_box)]
    fn film_across_over_dry(px: usize, tool: Tool, pressure: f32) -> Vec<(f32, f32)> {
        let mut c = Canvas::new(px, 4.0, hex(BG)).with_size_mm(440.0).with_linen(crate::surface::Linen::fine(3));
        // a light layer, laid in overlapping bands and let dry
        for (k, y) in [95.0, 110.0, 125.0, 140.0, 155.0].into_iter().enumerate() {
            let mut h = Held::new(Tool::filbert(40.0), 10 + k as u64);
            h.load(Paint::body(hex("#b8c4d0")), 1.0);
            c.drag(&mut h, &Gesture::line((100.0, y), (900.0, y)).pressure(0.9, 0.9), None);
        }
        c.dry();
        let mut h = Held::new(tool.clone(), 5);
        h.load(Paint::body(hex("#50586a")), 0.8);
        c.drag(&mut h, &Gesture::line((300.0, 125.0), (700.0, 125.0)).pressure(pressure, pressure), None);
        let f = c.f;
        let (x0, x1) = ((400.0 * f.scale) as usize, (600.0 * f.scale) as usize);
        let (y0, y1) = (((125.0 - 2.0 * tool.width) * f.scale) as usize, ((125.0 + 2.0 * tool.width) * f.scale) as usize);
        (y0..=y1).map(|y| (f.uy(y) - 125.0, (x0..x1).map(|x| c.wet.vol[y * f.w + x]).sum::<f32>() / (x1 - x0) as f32)).collect()
    }

    /// A body stroke over dry paint lays a covering film, thickest about
    /// where the brush pressed, not a ring: across the middle half of the
    /// mark the film is at least 0.6 of the thickest paint at its edges,
    /// also for brushes whose hairs are finer than a pixel (the filbert 5
    /// and filbert 2 at 1000px).
    #[test]
    #[cfg(tube_box)]
    fn a_stroke_over_dry_paint_covers_its_middle() {
        let st = crate::style::Style::oil();
        for (name, tool) in [("filbert 5", Tool::filbert(5.0)), ("filbert 2", Tool { lay: 0.5, stiffness: 0.3, ..Tool::filbert(2.0) }), ("body", st.body.clone())] {
            let prof = film_across_over_dry(1000, tool, 0.8);
            let peak = prof.iter().map(|p| p.1).fold(0.0f32, f32::max);
            // the mark: where it laid a tenth of its peak or more
            let on: Vec<f32> = prof.iter().filter(|p| p.1 >= 0.1 * peak).map(|p| p.0).collect();
            let (a, b) = (on[0], on[on.len() - 1]);
            let mid: Vec<f32> = prof.iter().filter(|p| p.0 >= a + 0.25 * (b - a) && p.0 <= b - 0.25 * (b - a)).map(|p| p.1).collect();
            let middle = mid.iter().sum::<f32>() / mid.len() as f32;
            assert!(middle >= 0.6 * peak, "{name}: {middle:.2} coats in the middle of the mark, {peak:.2} at its thickest: {prof:.2?}");
        }
    }

    #[test]
    fn pointed_marks_are_resolution_independent() {
        let g = Gesture::line((50.0, 120.0), (450.0, 122.0)).pressure(0.4, 0.4).ramps(0.05, 0.1).shake(0.0);
        for tool in [rigger(0.5), sable(1.6)] {
            let lo = ink(&canvas(500, false, tool.clone(), &g), 100.0, 400.0);
            let hi = ink(&canvas(1600, false, tool.clone(), &g), 100.0, 400.0);
            assert!((lo / hi - 1.0).abs() < 0.2, "{:?}: ink width {lo} at 500px, {hi} at 1600px", tool.kind);
            // and about as wide as the brush says
            let w = tool.mark_width(0.4);
            assert!(hi > 0.6 * w && hi < 2.0 * w, "{:?}: ink width {hi}, mark_width {w}", tool.kind);
        }
    }

    /// The presets are blunt, so a round, a rigger, a line or a detail brush
    /// lays the width asked for; the pointed tip (a hairline at light
    /// pressure) is opt-in.
    #[test]
    #[cfg(tube_box)]
    fn presets_are_blunt_and_lay_their_width() {
        let st = crate::style::Style::oil();
        for t in [Tool::round_sable(1.6), Tool::rigger(0.5), st.detail.clone(), st.line_tool(0.8)] {
            assert_eq!(t.point, 0.0, "{:?} is pointed by default", t.kind);
        }
        let g = Gesture::line((50.0, 120.0), (450.0, 122.0)).pressure(0.4, 0.4).ramps(0.05, 0.1).shake(0.0);
        let blunt = ink(&canvas(1600, false, Tool::round_sable(1.6), &g), 100.0, 400.0);
        let pointed = ink(&canvas(1600, false, sable(1.6), &g), 100.0, 400.0);
        // a blunt sable 1.6 at a light pressure lays most of its width; the
        // pointed one only its point
        assert!(blunt > 0.5 * 1.6 && blunt > 1.5 * pointed, "ink width at p 0.4: blunt {blunt}, pointed {pointed}");
    }

    #[test]
    fn pointed_width_follows_pressure_and_tapers() {
        let t = sable(3.0);
        assert!(t.mark_width(0.1) < 0.25 * t.mark_width(0.9), "{} vs {}", t.mark_width(0.1), t.mark_width(0.9));
        assert!((t.mark_width(t.pressure_for(1.5)) - 1.5).abs() < 0.01);
        let at = |p: f32| ink(&canvas(800, false, t.clone(), &Gesture::line((50.0, 120.0), (450.0, 120.0)).pressure(p, p).shake(0.0)), 150.0, 350.0);
        let (a, b, c) = (at(0.1), at(0.4), at(0.8));
        assert!(a < 0.5 * b && b < 0.7 * c, "ink width at pressure .1/.4/.8: {a} {b} {c}");
        // a flick lifted off draws down to a point
        let f = canvas(800, false, t.clone(), &Gesture::line((50.0, 120.0), (450.0, 120.0)).pressure(0.8, 0.0).ramps(0.05, 0.8).shake(0.0));
        let (root, mid, tip) = (ink(&f, 80.0, 120.0), ink(&f, 230.0, 270.0), ink(&f, 400.0, 430.0));
        assert!(root > mid && mid > tip && tip < 0.3 * root, "flick ink root {root}, middle {mid}, tip {tip}");
        // the stippler is blunt too
        assert_eq!(Tool::stippler(2.0).point, 0.0);
    }

    /// A lifting brush deposits paint through the end of its path, even
    /// where the pressure falls below its first hair's threshold: every
    /// column up to the last pixel before the end has paint. Blunt and
    /// pointed brushes, a flick lifted to zero and a stroke lifted to 0.36.
    #[test]
    fn a_lifted_stroke_paints_to_the_end_of_its_path() {
        let tools = [("blunt sable 3", Tool::round_sable(3.0)), ("blunt rigger 1", Tool::rigger(1.0)), ("pointed rigger 1.4", Tool { point: 0.9, ..Tool::rigger(1.4) }), ("pointed sable 3", sable(3.0))];
        let strokes = [("flick", (0.75, 0.0), (0.1, 0.75)), ("lifted to 0.36", (0.8, 0.36), (0.03, 0.35))];
        for (tn, tool) in &tools {
            for (gn, p, r) in strokes {
                // 80 units, within one load of every brush here
                let g = Gesture::line((100.0, 120.0), (180.0, 120.0)).pressure(p.0, p.1).ramps(r.0, r.1).shake(0.0);
                let c = canvas(1000, false, tool.clone(), &g);
                let s = c.f.scale;
                let col = |x: usize| (0..c.f.h).map(|y| dark(&c, x, y)).sum::<f32>() / s;
                // every column from just past the start to the last pixel
                // before the end has paint on it
                let bare: Vec<f32> = ((101.0 * s) as usize..(179.0 * s) as usize).filter(|&x| col(x) < 0.02).map(|x| x as f32 / s).collect();
                assert!(bare.is_empty(), "{tn}, {gn}: path ends at 180, mark stops at {:?} ({} bare columns)", bare.first(), bare.len());
            }
        }
    }

    /// A hair's track covers its own area of the pixel lattice, however it
    /// lies across the pixels: diagonal and oblique tracks keep their area
    /// under sub-pixel translation.
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
        for (tool, p) in [(rigger(0.5), 0.3), (sable(1.6), 0.15)] {
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
        let c = canvas(1600, true, rigger(0.5), &Gesture::line((50.0, 120.0), (450.0, 120.0)).pressure(0.25, 0.25).ramps(0.05, 0.1).shake(0.0));
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
        for (name, tool) in [("rigger .5", rigger(0.5)), ("sable 1.6", sable(1.6)), ("sable 3", sable(3.0))] {
            for p in [0.1, 0.4, 0.7, 0.9] {
                let g = Gesture::line((50.0, 120.0), (450.0, 122.0)).pressure(p, p).ramps(0.05, 0.1).shake(0.0);
                let lo = ink(&canvas(1000, false, tool.clone(), &g), 100.0, 400.0);
                let hi = ink(&canvas(3200, false, tool.clone(), &g), 100.0, 400.0);
                println!("{name:10} p {p:.1}: ink width {lo:.3} at 1000px, {hi:.3} at 3200px; mark_width {:.3}", tool.mark_width(p));
            }
        }
    }

    #[test]
    fn feed_conserves_paint() {
        let mut h = Held::new(sable(2.0), 1);
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

// every test here paints on the default box's style
#[cfg(all(test, tube_box))]
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
        let st = Style::oil_red_ground();
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

    /// With look and fill, a loaded brush at coverage 2.5 leaves under 0.2%
    /// of a dark body or broad passage showing the ground; with fill off
    /// the strokes leave more than 0.5% as gaps; a nearly dry brush at
    /// light pressure breaks up (more than 10% bare).
    #[test]
    fn loaded_passage_covers() {
        let st = Style::oil_red_ground();
        let dark = |_: f32, _: f32| hex("#2c2925");
        let body = || st.body().color(dark).clip(true).threshold(0.5).coverage(2.5);
        let broad = || st.broad().color(dark).clip(true).threshold(0.5).coverage(2.5);
        let (_, bare) = bare_share(500, &body().fill(true), 5);
        let (_, bare_broad) = bare_share(500, &broad().fill(true), 5);
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
        let st = Style::oil_red_ground();
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
        let st = Style::oil_red_ground();
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

/// Two arithmetic traps for exact (bit-identical) speedups of the brush
/// kernel and its contact surface. Each pins what the code on main does, so
/// a faster version that changes a bit fails here before it reaches a replay.
#[cfg(test)]
mod kernel_traps {
    use super::*;
    use crate::color::hex;

    fn fnv(h: &mut u64, bits: u32) {
        for b in bits.to_le_bytes() {
            *h ^= b as u64;
            *h = h.wrapping_mul(0x100_0000_01b3);
        }
    }

    /// An engine-2 canvas that has waited (drying state allocated), wet all
    /// over: every other pixel (at random) a film just over the 1e-5 coats
    /// `wait` counts as a film (up to 1.05e-5), with cure 0.1, the rest
    /// thick wet paint with cure 0.05.
    fn thin_and_thick() -> Canvas {
        // (the recorded hashes are engine 2's)
        let mut c = Canvas::new(600, 2.0, hex("#e8e0d0")).with_size_mm(440.0).with_engine(2);
        c.wait(1.0);
        let n = c.f.w * c.f.h;
        assert_eq!(c.wet.clock.px.len(), n, "drying state allocated");
        let lat = Paint::body(hex("#50586a")).latent();
        let mut rng = Rng::new(9);
        for i in 0..n {
            let thin = rng.f() < 0.5;
            c.wet.vol[i] = if thin { 1.0e-5 * (1.0 + 0.05 * rng.f()) } else { 2e-3 };
            c.wet.lat[i] = lat;
            c.wet.clock.px[i].cure = if thin { 0.1 } else { 0.05 };
        }
        c.wet.dirty = Some((0, 0, c.f.w, c.f.h));
        c
    }

    /// Trap A, the transfer: a plough transfer lifts paint off the source
    /// (`Surf::take`), which zeroes the source's cure once its film falls
    /// under 1e-5 coats (engine 2, drying state allocated), then lays it on
    /// the destination (`Surf::add`) with the cure it read. The plough reads
    /// the source's cure again for every destination it shares paint with,
    /// so the destinations after the crossing get cure 0, not the cure the
    /// source had before the first transfer.
    #[test]
    fn a_plough_transfer_rereads_the_source_cure_after_each_take() {
        let mut c = thin_and_thick();
        let w = c.f.w;
        let i = 50 * w + 50;
        let js = [49 * w + 50, 50 * w + 49, 50 * w + 51, 51 * w + 50];
        c.wet.vol[i] = 1.03e-5;
        c.wet.clock.px[i].cure = 0.1;
        for &j in &js {
            c.wet.vol[j] = 1e-3;
            c.wet.clock.px[j].cure = 0.0;
        }
        let sf = c.surf();
        let m = 1.44e-6f32 * 0.25;
        let mut carried = Vec::new();
        for &j in &js {
            // as the plough block in `exchange` does, per accepted destination
            unsafe {
                let l = *sf.lat.add(i);
                let hd = *sf.hide.add(i);
                let cure = (*sf.dry.add(i)).cure;
                carried.push(cure);
                sf.take(i, m);
                sf.add(j, m, &l, hd, cure);
            }
        }
        // the first take crossed the threshold: only the first destination
        // got the source's cure
        assert_eq!(carried, [0.1, 0.0, 0.0, 0.0]);
        assert!(c.wet.vol[i] < 1e-5 && c.wet.clock.px[i].cure == 0.0);
        let got: Vec<u32> = js.iter().map(|&j| c.wet.clock.px[j].cure.to_bits()).collect();
        let t = 1e-3f32 + m;
        let first = (0.0 + (0.1f32 - 0.0) * (m / t)).to_bits();
        assert_eq!(got, [first, 0, 0, 0], "destination cures {:?}", js.iter().map(|&j| c.wet.clock.px[j].cure).collect::<Vec<_>>());
        // (a cure read once before the loop would have given every
        // destination the first one's cure: 3.6e-5, not 0)
        assert!(f32::from_bits(first) > 3e-5);
    }

    /// Trap A, the kernel: an empty filbert that doesn't pick up (so it only
    /// ploughs, sharing each pixel's paint bilinearly among four
    /// destinations) dragged through films at the threshold leaves exactly
    /// the wet volume and cure recorded from main (4ec3169). A plough that
    /// hoisted the source's cure out of its destination loop fails this.
    #[test]
    fn ploughing_films_at_the_threshold_leaves_the_recorded_cure() {
        let mut c = thin_and_thick();
        let before: Vec<f32> = c.wet.clock.px.iter().map(|p| p.cure).collect();
        let tool = Tool { pickup: 0.0, push: 1.0, ..Tool::filbert(12.0) };
        let mut h = Held::new(tool, 4);
        for (k, y) in [150.0f32, 250.0, 350.0].into_iter().enumerate() {
            let g = Gesture::line((80.0, y), (920.0, y + 30.0 * k as f32)).pressure(0.8, 0.8);
            c.drag(&mut h, &g, None);
        }
        let n = c.f.w * c.f.h;
        let zeroed = (0..n).filter(|&i| before[i] > 0.0 && c.wet.clock.px[i].cure == 0.0).count();
        let (mut hv, mut hc) = (0xcbf2_9ce4_8422_2325u64, 0xcbf2_9ce4_8422_2325u64);
        for i in 0..n {
            fnv(&mut hv, c.wet.vol[i].to_bits());
            fnv(&mut hc, c.wet.clock.px[i].cure.to_bits());
        }
        // the strokes did plough thin films under the threshold
        assert!(zeroed > 1000, "{zeroed} films ploughed bare");
        assert_eq!(format!("vol={hv:016x} cure={hc:016x} zeroed={zeroed}"), "vol=216de608ff085476 cure=322c7a06e7dcb3f4 zeroed=7536", "the plough no longer moves paint and cure as main did");
    }

    /// Trap B, the medians: the contact level's running median of a pixel
    /// is the median of the pixels within its window, whatever else is in
    /// the row, so recomputing it over a crop that holds each output's
    /// whole window (or reaches the row's own ends) gives the full row's
    /// result bit for bit.
    #[test]
    fn a_running_median_over_a_crop_with_its_window_is_bit_identical() {
        let mut rng = Rng::new(20260930);
        let row: Vec<f32> = (0..400).map(|_| rng.range(-80.0, 200.0)).collect();
        let r = 23;
        let mut full = vec![0.0; row.len()];
        running_median(&row, r, &mut full);
        for (a, b) in [(100usize, 180usize), (0, 60), (350, 400), (0, 400)] {
            let (ca, cb) = (a.saturating_sub(r), (b + r).min(row.len()));
            let mut local = vec![0.0; cb - ca];
            running_median(&row[ca..cb], r, &mut local);
            let same = (a..b).all(|x| full[x].to_bits() == local[x - ca].to_bits());
            assert!(same, "median over [{ca}, {cb}) differs from the full row in [{a}, {b})");
        }
    }

    /// Trap B, the blur: `surface::box_blur` keeps a running sum from the
    /// start of each row and column. The same blur over a crop, even one
    /// holding every output's whole window, restarts that sum and rounds
    /// differently: it is NOT bit-identical to the full frame. Recomputing
    /// only part of the contact level must keep the full rows' and columns'
    /// sums (or recompute the blur in full).
    #[test]
    fn a_box_blur_over_a_crop_is_not_bit_identical() {
        let (w, h, r) = (160, 120, 3);
        let mut rng = Rng::new(20260930);
        let src: Vec<f32> = (0..w * h).map(|_| rng.range(0.0, 200.0)).collect();
        let full = crate::surface::box_blur(&src, w, h, r);
        // the interior [60, 100) × [40, 80), cropped with a halo of r
        let (x0, y0, x1, y1) = (60 - r, 40 - r, 100 + r, 80 + r);
        let crop: Vec<f32> = (y0..y1).flat_map(|y| src[y * w + x0..y * w + x1].to_vec()).collect();
        let local = crate::surface::box_blur(&crop, x1 - x0, y1 - y0, r);
        let (mut differ, mut max) = (0, 0.0f32);
        for y in 40..80 {
            for x in 60..100 {
                let (a, b) = (full[y * w + x], local[(y - y0) * (x1 - x0) + x - x0]);
                if a.to_bits() != b.to_bits() {
                    differ += 1;
                    max = max.max((a - b).abs());
                }
                // the same value, but for rounding
                assert!((a - b).abs() <= 1e-3, "{a} vs {b} at ({x}, {y})");
            }
        }
        assert!(differ > 0, "the cropped blur is bit-identical here: box_blur no longer keeps a running sum?");
        eprintln!("cropped box blur: {differ} of 1600 pixels differ, by up to {max:e}");
    }
}

#[cfg(test)]
mod part_tests {
    use super::*;
    use crate::color::hex;

    fn vols(h: &Held) -> Vec<(f32, f32)> {
        h.bristles.iter().map(|b| (b.rx, b.vol)).collect()
    }

    #[test]
    fn whole_part_is_an_ordinary_load() {
        let mut a = Held::new(Tool::filbert(12.0), 7);
        let mut b = Held::new(Tool::filbert(12.0), 7);
        a.load(Paint::body(hex("#445566")), 0.8);
        b.load_part(Paint::body(hex("#445566")), 0.8, &Part::ALL);
        assert_eq!(vols(&a), vols(&b));
    }

    #[test]
    fn a_side_dip_reaches_only_that_side() {
        let mut h = Held::new(Tool::filbert(12.0), 7);
        h.load_part(Paint::body(hex("#c04040")), 0.8, &Part { side: 1.0, share: 0.4, streak: 0.0, seed: 1 });
        for (rx, v) in vols(&h) {
            if rx < -0.1 {
                assert_eq!(v, 0.0, "bristle at {rx} took paint");
            }
            if rx > 0.6 {
                assert!(v > 0.0, "bristle at {rx} took none");
            }
        }
    }

    #[test]
    fn streaks_keep_about_the_same_paint() {
        let mut even = Held::new(Tool::filbert(12.0), 7);
        even.load(Paint::body(hex("#445566")), 0.8);
        let mut sum = 0.0;
        for seed in 0..40 {
            let mut h = Held::new(Tool::filbert(12.0), 7);
            h.load_part(Paint::body(hex("#445566")), 0.8, &Part { streak: 1.0, seed, ..Part::ALL });
            sum += h.fullness() / even.fullness();
            // and the bristles take it unevenly
            let v: Vec<f32> = h.bristles.iter().map(|b| b.vol).collect();
            let (lo, hi) = v.iter().fold((f32::MAX, 0f32), |(l, m), &x| (l.min(x), m.max(x)));
            assert!(hi > 3.0 * lo.max(1e-9), "seed {seed}: {lo}..{hi}");
        }
        let mean = sum / 40.0;
        assert!((0.8..1.2).contains(&mean), "mean {mean}");
    }
}

/// The share of a tube paint's volume that is oil (about 30–45%).
const OIL_SHARE: f32 = 0.4;

/// How far (µm) below the paint under a knife's blade it is pressed into the
/// hollows of the surface (`Canvas::knife`).
const PRESS_IN_UM: f32 = 60.0;

/// The span (mm) over which a knife's flexible blade follows the relief under
/// it; finer hollows than that it bridges (`Canvas::knife`).
const FLEX_MM: f32 = 4.0;

/// A painting knife: a flat, rigid steel blade, `width` units long, and the
/// bead of paint it carries (coats × units², as a bristle's load). It lays
/// paint in flat slabs, leaving the paint it cuts off at the blade's ends in
/// ridges and a bead where it lifts, and scrapes wet paint off.
#[derive(Clone, Debug)]
pub struct Knife {
    pub width: f32,
    vol: f32,
    lat: Latent,
    hide: Prop,
    cure: f32,
}

impl Knife {
    pub fn new(width: f32) -> Self {
        Knife { width: if width.is_finite() { width.clamp(1.0, 1000.0) } else { 1.0 }, vol: 0.0, lat: [0.0; LAT], hide: [0.0; 5], cure: 0.0 }
    }
    /// A full load: a bead along the blade twice its length deep and 12
    /// coats (300 µm) thick (about a millilitre on a 4 cm blade).
    pub fn full(&self) -> f32 {
        self.width * self.width * 2.0 * 12.0
    }
    /// Pick up `amount` (0..1 of a full load) of `paint` onto the blade.
    pub fn load(&mut self, paint: Paint, amount: f32) {
        // (no amount that is not a number: it would leave the blade's paint NaN)
        if !amount.is_finite() {
            return;
        }
        let v = amount.max(0.0) * self.full();
        let lat = paint.latent();
        self.cure = mix_cure(self.cure, self.vol, 0.0, v);
        crate::wet::mix_into(&mut self.vol, &mut self.lat, &mut self.hide, v, &lat, paint.prop());
    }
    /// The paint on the blade as one paint (what it has picked up, mixed by
    /// volume), or None on a clean blade. For looking at it: nothing changes.
    pub fn paint(&self) -> Option<Paint> {
        if self.vol <= 1e-9 {
            return None;
        }
        let color = mixbox::latent_to_linear_float_rgb(&self.lat);
        let p = Paint::km(color, self.hide[0], self.hide[1].clamp(0.0, 1.0)).with_drying(self.hide[2]);
        Some(Paint { solvent: self.hide[3], oil: self.hide[4], ..p })
    }
    /// Wipe the blade clean on the rag.
    pub fn wipe(&mut self) {
        self.vol = 0.0;
    }
    /// Paint on the blade, relative to a full load.
    pub fn fullness(&self) -> f32 {
        self.vol / self.full()
    }
}

impl Canvas {
    /// Drag a knife along `pts` (units), its blade held across the path or at
    /// a fixed `angle` (radians), pressed `pressure` (start, end; 0..1). The
    /// blade rests on the highest points of the dry surface under it and
    /// stands off them by a gap that closes as it is pressed: a light touch
    /// rides some 0.3 mm up, a full press scrapes down to the dry paint. Wet
    /// paint standing above the blade is cut off and carried in its bead,
    /// mixing with it, or at the blade's ends pressed out sideways into
    /// ridges. Where the surface lies below the blade, within the gap of it,
    /// `lay` fills it from the bead to the blade's level: a slab with a flat
    /// top, or over dry impasto paint on its peaks only. When the blade
    /// lifts, the share `lift` of the bead stays where it last was.
    pub fn knife(&mut self, k: &mut Knife, pts: &[(f32, f32)], pressure: (f32, f32), angle: Option<f32>, lay: bool, lift: f32) {
        // (the blade's length is a public field: nothing to pull with one that is no length)
        if pts.is_empty() || !(k.width.is_finite() && (1.0..=1000.0).contains(&k.width)) {
            return;
        }
        self.tally.stroke(&Tool::hog_flat(k.width), pts, self.mm_per_unit);
        let id = self.next_stroke_ids(1);
        // engine 5: knife-laid paint tears where it parts from the blade:
        // at the slab's ends, its leading edge, and where the blade's reach
        // into the hollows runs out (`tn`, a noise along and across the blade)
        let tears = self.engine >= 5;
        // (the paint the tears held back under the blade: back on it at the end)
        let mut held = 0.0f32;
        let tseed = (id as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x7EA2;
        let height: *const f32 = self.height.as_ptr();
        let sf = self.surf();
        let s = sf.scale;
        let px_area = 1.0 / (s * s);
        let p: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
        let path = if p.len() >= 2 { crate::path::densify(&p) } else { vec![p[0], p[0]] };
        let mut arc = vec![0.0f32; path.len()];
        for i in 1..path.len() {
            arc[i] = arc[i - 1] + ((path[i].0 - path[i - 1].0).powi(2) + (path[i].1 - path[i - 1].1).powi(2)).sqrt();
        }
        let total = arc[arc.len() - 1].max(1e-6);
        let half = k.width * s * 0.5;
        // mm per pixel
        let s_mm = self.px_mm();
        // the tears' noise: cells per pixel along the path (0.9 and 2.7 to
        // the mm, at most a cell to 3 and to 2 pixels) and cells across half
        // the blade (4 and 11, at least 2 pixels each)
        let tear_d = ((0.9 * s_mm).min(1.0 / 3.0), (2.7 * s_mm).min(0.5));
        let tear_u = (4.0f32.min(half / 2.0).max(1.0), 11.0f32.min(half / 2.0).max(1.0));
        let (fw, fh) = (sf.fw as isize, sf.fh as isize);
        let pix = |x: f32, y: f32| -> Option<usize> {
            let (xi, yi) = (x.floor() as isize, y.floor() as isize);
            if xi < sf.ox as isize || yi < sf.oy as isize || xi >= (sf.ox + sf.w) as isize || yi >= (sf.oy + sf.h) as isize || xi >= fw || yi >= fh {
                return None;
            }
            Some((yi as usize - sf.oy) * sf.w + xi as usize - sf.ox)
        };
        let mut bounds: Bounds = None;
        let step = 0.7f32;
        let n = ((total / step).ceil() as usize).max(1);
        let mut j = 0usize;
        let mut last = (path[0], (1.0f32, 0.0f32));
        for t in 0..=n {
            let d = total * t as f32 / n as f32;
            while j + 1 < path.len() - 1 && arc[j + 1] < d {
                j += 1;
            }
            let (a, b) = (path[j], path[(j + 1).min(path.len() - 1)]);
            let seg = (arc[(j + 1).min(path.len() - 1)] - arc[j]).max(1e-6);
            let f = ((d - arc[j]) / seg).clamp(0.0, 1.0);
            let c = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            let dir = {
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                let m = (dx * dx + dy * dy).sqrt();
                if m > 1e-6 { (dx / m, dy / m) } else { last.1 }
            };
            last = (c, dir);
            let e = match angle {
                Some(a) => (a.cos(), a.sin()),
                None => (-dir.1, dir.0),
            };
            let pr = pressure.0 + (pressure.1 - pressure.0) * (d / total);
            // the gap between blade and the dry surface's peaks, µm
            let gap = 300.0 * (1.0 - pr.clamp(0.0, 1.0)).powf(1.5);
            // the blade's pixels, and the highest dry point under it
            let nb = ((2.0 * half / 0.7).ceil() as usize).max(2);
            let mut blade: Vec<(usize, f32, (f32, f32))> = Vec::with_capacity(nb + 1);
            for q in 0..=nb {
                let u = -half + 2.0 * half * q as f32 / nb as f32;
                let (x, y) = (c.0 + e.0 * u, c.1 + e.1 * u);
                if let Some(i) = pix(x, y) {
                    if blade.last().map(|b| b.0) != Some(i) {
                        blade.push((i, u / half, (x, y)));
                    }
                }
            }
            if blade.is_empty() {
                continue;
            }
            // the steel flexes over broad relief and bridges fine hollows: the
            // blade rests on the highest point within FLEX_MM along it
            let hs: Vec<f32> = blade.iter().map(|b| unsafe { *height.add(b.0) }).collect();
            let reach = ((FLEX_MM / (s_mm * 0.7)).ceil() as usize).max(1);
            let rest: Vec<f32> = (0..hs.len()).map(|q| hs[q.saturating_sub(reach)..(q + reach + 1).min(hs.len())].iter().cloned().fold(f32::MIN, f32::max)).collect();
            for (bi, &(i, u, (x, y))) in blade.iter().enumerate() {
                let plane = rest[bi] + gap;
                // the tear here: 0..1, coarse along the blade, finer along the
                // path: about a millimetre across, but never finer than a few
                // pixels, so a small canvas (a sketch, a preview) tears too,
                // more coarsely, where a pixel is wider than a tear
                let tn = if tears {
                    crate::surface::vnoise(u * tear_u.0 + 7.0, d * tear_d.0, tseed) * 0.7 + crate::surface::vnoise(u * tear_u.1, d * tear_d.1, tseed ^ 0x51) * 0.3
                } else {
                    0.5
                };
                // SAFETY: exclusive &mut self; the knife is one tool on its own
                unsafe {
                    let v = *sf.vol.add(i);
                    let top = *height.add(i) + v * crate::surface::COAT_UM;
                    let fl = if sf.dry.is_null() { 1.0 } else { crate::drying::fluid((*sf.dry.add(i)).cure) };
                    if top > plane && v > 1e-6 {
                        // cut off what stands above the blade (setting paint resists)
                        let ex = ((top - plane) / crate::surface::COAT_UM).min(v) * fl;
                        if ex <= 1e-7 {
                            continue;
                        }
                        let (l, hd) = (*sf.lat.add(i), *sf.hide.add(i));
                        let cure = if sf.dry.is_null() { 0.0 } else { (*sf.dry.add(i)).cure };
                        sf.take(i, ex);
                        // pressed out past the blade's end into a ridge (off
                        // the canvas's edge there is nowhere to press it: it
                        // stays on the blade)
                        let o = half * u.signum() * 0.2 + 1.5 * u.signum();
                        let ridge = if u.abs() > 0.85 { pix(x + e.0 * o, y + e.1 * o) } else { None };
                        if let Some(jx) = ridge {
                            sf.add(jx, ex, &l, hd, cure);
                            *sf.cover.add(jx) = 1.0;
                            grow(&mut bounds, (x + e.0 * o) as usize, (y + e.1 * o) as usize, (x + e.0 * o) as usize + 1, (y + e.1 * o) as usize + 1);
                        } else {
                            let tv = ex * px_area;
                            k.cure = mix_cure(k.cure, k.vol, cure, tv);
                            crate::wet::mix_into(&mut k.vol, &mut k.lat, &mut k.hide, tv, &l, hd);
                        }
                    } else if lay
                        && top < plane
                        && tears
                        && plane - top <= 2.0 * gap + PRESS_IN_UM
                        && (plane - top > (2.0 * gap + PRESS_IN_UM) * (0.45 + 1.1 * tn) || u.abs() > 0.72 && tn < (u.abs() - 0.72) / 0.28 * 1.1 || d < (1.0 + 7.0 * tn) / s_mm)
                    {
                        // a tear: the paint a whole slab would have left here
                        // parts with the blade and stays under it, out of the
                        // bead, so a torn pull runs out where a whole one
                        // does and covers less (once for each pixel)
                        if k.vol > 1e-9 && *sf.stroke.add(i) != id {
                            let hold = ((plane - top) / crate::surface::COAT_UM).min(k.vol / px_area * 0.25) * px_area;
                            k.vol -= hold;
                            held += hold;
                        }
                    } else if lay && top < plane && plane - top <= (2.0 * gap + PRESS_IN_UM) * if tears { 0.45 + 1.1 * tn } else { 1.0 } && k.vol > 1e-9 {
                        // the paint under the blade is pressed into the
                        // surface's hollows as deep as the gap and some tens
                        // of µm more (the weave, a ground's marks); deeper
                        // hollows stay bare, so a light pull over dry
                        // impasto catches its ridges and skips its valleys
                        let want = (plane - top) / crate::surface::COAT_UM;
                        let give = want.min(k.vol / px_area * 0.25);
                        if give > 1e-7 {
                            sf.add(i, give, &k.lat, k.hide, k.cure);
                            *sf.cover.add(i) = 1.0;
                            k.vol -= give * px_area;
                        }
                    }
                    *sf.stroke.add(i) = id;
                    grow(&mut bounds, x as usize, y as usize, x as usize + 1, y as usize + 1);
                }
            }
        }
        // lifting off: part of the bead stays as a ridge along the blade
        if lift > 0.0 && k.vol > 1e-9 {
            let (c, dir) = last;
            let e = match angle {
                Some(a) => (a.cos(), a.sin()),
                None => (-dir.1, dir.0),
            };
            let leave = k.vol * lift.clamp(0.0, 1.0);
            let nb = ((2.0 * half / 0.7).ceil() as usize).max(2);
            let mut cells = Vec::new();
            for q in 0..=nb {
                let u = -half + 2.0 * half * q as f32 / nb as f32;
                for back in [0.0f32, 1.0, 2.0] {
                    let (x, y) = (c.0 + e.0 * u + dir.0 * back, c.1 + e.1 * u + dir.1 * back);
                    if let Some(i) = pix(x, y) {
                        // (where it tears, the ridge left at the lift breaks up)
                        let t = if tears { smoothstep(0.25, 0.65, crate::surface::vnoise(u / half * 6.0 + 3.0, back, tseed ^ 0x11F7)) } else { 1.0 };
                        cells.push((i, (1.0 - (u / half).powi(2)).max(0.2) * (1.0 - 0.3 * back) * t, (x, y)));
                    }
                }
            }
            let tw: f32 = cells.iter().map(|c| c.1).sum();
            if tw > 0.0 {
                for (i, wgt, (x, y)) in cells {
                    let v = leave * wgt / tw / px_area;
                    // SAFETY: as above
                    unsafe {
                        sf.add(i, v, &k.lat, k.hide, k.cure);
                        *sf.cover.add(i) = 1.0;
                        *sf.stroke.add(i) = id;
                    }
                    grow(&mut bounds, x as usize, y as usize, x as usize + 1, y as usize + 1);
                }
                k.vol -= leave;
            }
        }
        k.vol += held;
        // (the window's own pixels, as a brush's bounds: a crop's start at its corner)
        if let Some((x0, y0, x1, y1)) = bounds.map(|(x0, y0, x1, y1)| (x0 - sf.ox, y0 - sf.oy, x1 - sf.ox, y1 - sf.oy)) {
            self.wet.touch(x0, y0, x1, y1);
        }
    }
}

#[cfg(test)]
mod spatter_tests {
    use super::*;
    use crate::color::hex;

    fn flick(stiff: f32, force: f32) -> (usize, f32, f32) {
        let mut c = Canvas::new(400, 2.0, hex("#e8e0d0")).with_size_mm(300.0);
        let mut h = Held::new(Tool::round_sable(8.0), 3);
        h.load(Paint::new(hex("#b03020"), 0.9, stiff), 1.0);
        let before = h.fullness();
        let n = c.spatter(&mut h, &Spatter { at: (100.0, 150.0), toward: (150.0, 0.0), spread: 0.4, force, seed: 9 }, None);
        let laid: f32 = c.wet.vol.iter().sum();
        (n, before - h.fullness(), laid)
    }

    /// Fluid paint flies; stiff paint mostly stays on the brush; a harder
    /// flick throws more; what leaves the brush lands on the canvas.
    #[test]
    fn fluid_paint_flies_and_stiff_paint_stays() {
        let (n_fluid, out_fluid, laid_fluid) = flick(0.1, 0.9);
        let (_, out_stiff, _) = flick(0.95, 0.9);
        let (_, out_soft, _) = flick(0.1, 0.2);
        assert!(n_fluid > 20, "a hard flick of fluid paint throws many droplets ({n_fluid})");
        assert!(out_fluid > 2.0 * out_stiff, "stiff paint stays on the brush ({out_fluid} vs {out_stiff})");
        assert!(out_fluid > out_soft, "a harder flick throws more ({out_fluid} vs {out_soft})");
        assert!(laid_fluid > 0.0, "the droplets land in the wet layer");
    }

    /// Same seed, same spatter.
    #[test]
    fn spatter_is_deterministic() {
        assert_eq!(flick(0.2, 0.7).0, flick(0.2, 0.7).0);
        assert_eq!(flick(0.2, 0.7).2.to_bits(), flick(0.2, 0.7).2.to_bits());
    }
}
