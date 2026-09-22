//! Working an area with a real (simulated) brush.
//!
//! `Handling` describes how a painter works a region: which tool, how long
//! the strokes are and which way they run, how hard they press, how often
//! they go back to the palette and whether they wipe the brush first.
//! `Canvas::work` then drives a `Held` brush over the region stroke by
//! stroke, so all mixing, smearing, dry-brush and ridges come from the
//! bristle simulation, not from blend modes.

use crate::bristle::{Gesture, Held, Orient, Rect, Tool, footprint};
use crate::canvas::Canvas;
use crate::color::{Rgb, from_oklab, to_oklab};
use crate::mask::Mask;
use crate::palette::Palette;
use crate::rng::Rng;
use crate::wet::Paint;
use rayon::prelude::*;

type Field<'a, T> = Box<dyn Fn(f32, f32) -> T + Sync + 'a>;

pub struct Handling<'a> {
    pub tool: Tool,
    /// Stroke length range in units.
    pub length: (f32, f32),
    /// How many layers of strokes cover each point on average.
    pub coverage: f32,
    /// Stroke direction field (radians, 0 = left→right).
    pub angle: Field<'a, f32>,
    pub angle_jitter: f32,
    /// Color mixed on the palette for a stroke centered at a point.
    pub color: Field<'a, Rgb>,
    /// Palette-mixing inconsistency per dip: OKLab L and a/b sd.
    pub jitter: (f32, f32),
    /// Hiding and stiffness of the paint when it isn't mixed from a
    /// palette (see `Paint`).
    pub hiding: f32,
    pub stiff: f32,
    /// Mix each pile from these tube paints, thinned with this fraction of
    /// oil medium (0..1). The color field is then what the painter aims for.
    pub palette: Option<(&'a Palette, f32)>,
    /// How unevenly each pile is mixed: relative sd of the proportions.
    pub mix_jitter: f32,
    /// Pressure range (a random value in it per stroke).
    pub pressure: (f32, f32),
    pub orient: Orient,
    /// Strokes between trips to the palette.
    pub dip_every: usize,
    /// How much of a full load a dip takes.
    pub load: f32,
    /// Fraction of old paint wiped off before each dip (1 = clean, 0 = dirty).
    pub wipe: f32,
    /// Clean blender: never loads paint, wiped every `dip_every` strokes.
    pub blender: bool,
    /// Scumble: short back-and-forth strokes instead of single pulls.
    pub scrub: usize,
    /// Clip bristle contact to the mask (crisp, cut-in edges).
    pub clip: bool,
    /// Minimum mask value for a stroke center.
    pub threshold: f32,
    /// Press-down and lift-off fractions of each stroke.
    pub ramps: (f32, f32),
    /// Hand unsteadiness (1 = normal).
    pub shake: f32,
}

impl<'a> Handling<'a> {
    pub fn new(tool: Tool) -> Self {
        Handling {
            tool,
            length: (30.0, 90.0),
            coverage: 2.0,
            angle: Box::new(|_, _| 0.0),
            angle_jitter: 0.08,
            color: Box::new(|_, _| [0.5; 3]),
            jitter: (0.02, 0.006),
            hiding: 0.85,
            stiff: 0.8,
            pressure: (0.6, 0.9),
            orient: Orient::Across,
            dip_every: 1,
            load: 0.8 * 0.8,
            wipe: 0.6,
            blender: false,
            scrub: 0,
            clip: false,
            threshold: 0.3,
            ramps: (0.08, 0.15),
            shake: 1.0,
            palette: None,
            mix_jitter: 0.08,
        }
    }
    pub fn length(mut self, a: f32, b: f32) -> Self {
        self.length = (a, b);
        self
    }
    pub fn coverage(mut self, c: f32) -> Self {
        self.coverage = c;
        self
    }
    pub fn angle(mut self, f: impl Fn(f32, f32) -> f32 + Sync + 'a) -> Self {
        self.angle = Box::new(f);
        self
    }
    pub fn angle_jitter(mut self, a: f32) -> Self {
        self.angle_jitter = a;
        self
    }
    pub fn color(mut self, f: impl Fn(f32, f32) -> Rgb + Sync + 'a) -> Self {
        self.color = Box::new(f);
        self
    }
    pub fn jitter(mut self, l: f32, hue: f32) -> Self {
        self.jitter = (l, hue);
        self
    }
    /// A fixed paint (no palette mixing).
    pub fn paint(mut self, hiding: f32, stiff: f32) -> Self {
        self.hiding = hiding;
        self.stiff = stiff;
        self.palette = None;
        self
    }
    /// Mix every pile from `palette`'s tubes, thinned with `medium` (0..1).
    pub fn mixed(mut self, palette: &'a Palette, medium: f32) -> Self {
        self.palette = Some((palette, medium));
        self
    }
    /// Change how much medium goes into the palette mixtures.
    pub fn medium(mut self, medium: f32) -> Self {
        let (p, _) = self.palette.expect("medium() needs a palette: use mixed()");
        self.palette = Some((p, medium));
        self
    }
    pub fn mix_jitter(mut self, sd: f32) -> Self {
        self.mix_jitter = sd;
        self
    }
    /// How much paint each trip to the palette puts on the brush (0..1 of full).
    pub fn load(mut self, amount: f32) -> Self {
        self.load = amount;
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
    pub fn dips(mut self, every: usize, load: f32, wipe: f32) -> Self {
        self.dip_every = every.max(1);
        self.load = load;
        self.wipe = wipe;
        self
    }
    pub fn blender(mut self) -> Self {
        self.blender = true;
        self
    }
    pub fn scrub(mut self, n: usize) -> Self {
        self.scrub = n;
        self
    }
    pub fn clip(mut self, on: bool) -> Self {
        self.clip = on;
        self
    }
    pub fn threshold(mut self, t: f32) -> Self {
        self.threshold = t;
        self
    }
    pub fn ramps(mut self, attack: f32, release: f32) -> Self {
        self.ramps = (attack, release);
        self
    }
    pub fn shake(mut self, k: f32) -> Self {
        self.shake = k;
        self
    }
}

/// One planned stroke.
struct Plan {
    pts: Vec<(f32, f32)>,
    pressure: f32,
    fade: f32,
    /// Paint to dip into before this stroke (None = no trip to the palette).
    dip: Option<Paint>,
}

impl Canvas {
    /// Work the region `mask` with a simulated brush, stroke by stroke.
    ///
    /// Strokes are planned up front, then grouped into square passages
    /// (tiles) larger than twice any stroke's reach. Passages in a 2×2
    /// checkerboard phase can't share a pixel, so each gets its own brush and
    /// they are painted in parallel; phases run one after another.
    pub fn work(&mut self, mask: &Mask, hd: &Handling, seed: u64) {
        self.check_mask(mask);
        let mut rng = Rng::new(seed);
        let f = self.f;
        let mean_len = 0.5 * (hd.length.0 + hd.length.1);
        // one stroke per gap² of area gives coverage = width · length / gap²
        let gap = (hd.tool.width * mean_len.max(hd.tool.width) / hd.coverage.max(0.05)).sqrt().max(0.5);
        let (cols, rows) = ((f.width() / gap).ceil() as usize + 1, (f.height() / gap).ceil() as usize + 1);
        let mut centers = Vec::new();
        for j in 0..rows {
            for i in 0..cols {
                let x = (i as f32 + rng.f()) * gap;
                let y = (j as f32 + rng.f()) * gap;
                if x > f.width() || y > f.height() {
                    continue;
                }
                if mask.data[f.index(x, y)] >= hd.threshold {
                    centers.push((x, y));
                }
            }
        }
        for i in (1..centers.len()).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            centers.swap(i, j);
        }
        if centers.is_empty() {
            return;
        }

        // plan every stroke deterministically
        let mut plans = Vec::with_capacity(centers.len());
        // how far (units) any stroke's pixel footprint reaches from its center
        let (mut ex, mut ey) = (0.0f32, 0.0f32);
        for &(cx, cy) in &centers {
            let len = rng.range(hd.length.0, hd.length.1);
            let bend = rng.normal() * hd.angle_jitter;
            let pts: Vec<(f32, f32)> = if hd.scrub > 0 {
                let a = (hd.angle)(cx, cy) + bend;
                let (ca, sa) = (a.cos(), a.sin());
                let (nx, ny) = (-sa, ca);
                let amp = len * 0.5;
                let adv = hd.tool.width * 0.35;
                (0..=hd.scrub * 2)
                    .map(|k| {
                        let s = if k % 2 == 0 { -amp } else { amp };
                        let t = (k as f32 - hd.scrub as f32) * adv * 0.5;
                        (cx + ca * s + nx * t + rng.normal() * 1.5, cy + sa * s + ny * t + rng.normal() * 1.5)
                    })
                    .collect()
            } else {
                trace(&*hd.angle, cx, cy, len, bend, &mut rng)
            };
            let rect = footprint(&hd.tool, &pts, hd.shake, f.scale, f.w, f.h);
            if let Some(r) = rect {
                let (px, py) = (cx * f.scale, cy * f.scale);
                ex = ex.max((px - r.0 as f32).max(r.2 as f32 - px) / f.scale);
                ey = ey.max((py - r.1 as f32).max(r.3 as f32 - py) / f.scale);
            }
            let pressure = rng.range(hd.pressure.0, hd.pressure.1);
            let fade = rng.range(0.75, 1.05);
            let target = (hd.color)(cx, cy);
            let paint = match hd.palette {
                // on the palette: mix the pile from tubes, never twice alike
                Some((pal, medium)) => pal.remix(&pal.mix(target), hd.mix_jitter, &mut rng).paint(medium),
                None => {
                    let lab = to_oklab(target);
                    let col = from_oklab([
                        lab[0] + rng.normal() * hd.jitter.0,
                        lab[1] + rng.normal() * hd.jitter.1,
                        lab[2] + rng.normal() * hd.jitter.1,
                    ]);
                    Paint { color: col, hiding: hd.hiding, stiff: hd.stiff }
                }
            };
            plans.push((cx, cy, rect, Plan { pts, pressure, fade, dip: Some(paint) }));
        }
        // tiles are sized per axis from the footprints: tiles painted at the
        // same time are one tile apart, so a tile at least twice the largest
        // reach keeps their pixels disjoint (long horizontal strokes get wide,
        // short tiles)
        let margin = 2.0 / f.scale;
        let (tile_x, tile_y) = ((2.0 * ex + margin).max(gap * 2.0), (2.0 * ey + margin).max(gap * 2.0));
        let (tw, th) = ((f.width() / tile_x).ceil().max(1.0) as usize, (f.height() / tile_y).ceil().max(1.0) as usize);
        if std::env::var_os("PAINT_DEBUG").is_some() {
            eprintln!("work: {} strokes, reach {ex:.0}x{ey:.0}, tiles {tw}x{th}", centers.len());
        }
        let mut tiles: Vec<Vec<Plan>> = (0..tw * th).map(|_| Vec::new()).collect();
        // pixel footprint of each tile: the union of its strokes'
        let mut tile_rect: Vec<Option<Rect>> = vec![None; tw * th];
        for (cx, cy, rect, p) in plans {
            let tx = ((cx / tile_x) as usize).min(tw - 1);
            let ty = ((cy / tile_y) as usize).min(th - 1);
            let t = ty * tw + tx;
            if let Some(r) = rect {
                tile_rect[t] = Some(match tile_rect[t] {
                    None => r,
                    Some(a) => (a.0.min(r.0), a.1.min(r.1), a.2.max(r.2), a.3.max(r.3)),
                });
                tiles[t].push(p);
            }
        }
        // trips to the palette: every `dip_every` strokes within a passage
        for t in tiles.iter_mut() {
            for (k, p) in t.iter_mut().enumerate() {
                if k % hd.dip_every != 0 {
                    p.dip = None;
                }
            }
        }
        let n_strokes: usize = tiles.iter().map(|t| t.len()).sum();
        let first_id = self.next_stroke_ids(n_strokes as u32);
        let mut offsets = Vec::with_capacity(tiles.len());
        let mut acc = 0u32;
        for t in &tiles {
            offsets.push(acc);
            acc += t.len() as u32;
        }

        let surf = self.surf();
        let clip = if hd.clip { Some(mask) } else { None };
        let mut phases = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)];
        for i in (1..4).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            phases.swap(i, j);
        }
        let mut dirty: crate::bristle::Bounds = None;
        for (px, py) in phases {
            let idx: Vec<usize> = (0..tiles.len()).filter(|&i| (i % tw) % 2 == px && (i / tw) % 2 == py && tile_rect[i].is_some()).collect();
            let batches = disjoint_batches(&idx, &tile_rect);
            if std::env::var_os("PAINT_DEBUG").is_some() {
                eprintln!("  phase: {} tiles in {} batches", idx.len(), batches.len());
            }
            for batch in batches {
                let results: Vec<crate::bristle::Bounds> = batch
                    .par_iter()
                    .map(|&ti| {
                        let mut held = Held::new(hd.tool.clone(), seed ^ 0x5EED ^ (ti as u64).wrapping_mul(0x9E37_79B9));
                        let mut scratch = Vec::new();
                        let mut b: crate::bristle::Bounds = None;
                        for (k, p) in tiles[ti].iter().enumerate() {
                            if let Some(paint) = p.dip {
                                if hd.blender {
                                    held.wipe(0.9);
                                } else {
                                    held.wipe(hd.wipe);
                                    held.load(paint, hd.load);
                                }
                            }
                            let g = Gesture::new(p.pts.clone())
                                .pressure(p.pressure, p.pressure * p.fade)
                                .orient(hd.orient)
                                .ramps(hd.ramps.0, hd.ramps.1)
                                .shake(hd.shake);
                            let id = first_id.wrapping_add(offsets[ti] + k as u32);
                            // SAFETY: every pixel this drag touches lies in its
                            // stroke footprint, inside this tile's rect; tiles in
                            // one batch have disjoint rects; `surf()` checked the
                            // buffers match the frame.
                            let r = unsafe { crate::bristle::drag_on(surf, &mut held, &g, clip, id, &mut scratch) };
                            if let Some((a, c, d, e)) = r {
                                b = Some(match b {
                                    None => (a, c, d, e),
                                    Some((a0, c0, d0, e0)) => (a0.min(a), c0.min(c), d0.max(d), e0.max(e)),
                                });
                            }
                        }
                        b
                    })
                    .collect();
                for r in results.into_iter().flatten() {
                    dirty = Some(match dirty {
                        None => r,
                        Some((a0, c0, d0, e0)) => (a0.min(r.0), c0.min(r.1), d0.max(r.2), e0.max(r.3)),
                    });
                }
            }
        }
        if let Some((x0, y0, x1, y1)) = dirty {
            self.wet.touch(x0, y0, x1, y1);
        }
    }
}

/// Group tiles (in the given, deterministic order) into batches whose
/// footprints are pairwise disjoint; tiles in a batch may run concurrently.
fn disjoint_batches(idx: &[usize], rects: &[Option<Rect>]) -> Vec<Vec<usize>> {
    let overlaps = |a: Rect, b: Rect| a.0 < b.2 && b.0 < a.2 && a.1 < b.3 && b.1 < a.3;
    let mut batches: Vec<(Vec<usize>, Vec<Rect>)> = Vec::new();
    for &ti in idx {
        let r = rects[ti].expect("tile without footprint");
        match batches.iter_mut().find(|(_, rs)| rs.iter().all(|&q| !overlaps(q, r))) {
            Some((ts, rs)) => {
                ts.push(ti);
                rs.push(r);
            }
            None => batches.push((vec![ti], vec![r])),
        }
    }
    batches.into_iter().map(|(ts, _)| ts).collect()
}

/// A streamline through (cx, cy) along the angle field, random direction.
fn trace(angle: &(dyn Fn(f32, f32) -> f32 + Sync), cx: f32, cy: f32, len: f32, bend: f32, rng: &mut Rng) -> Vec<(f32, f32)> {
    let steps = 3;
    let h = len / (2 * steps) as f32;
    let mut fwd = vec![(cx, cy)];
    let mut back = vec![];
    let (mut x, mut y) = (cx, cy);
    for k in 0..steps {
        let a = angle(x, y) + bend * (1.0 + k as f32 * 0.3);
        x += a.cos() * h;
        y += a.sin() * h;
        fwd.push((x, y));
    }
    let (mut x, mut y) = (cx, cy);
    for k in 0..steps {
        let a = angle(x, y) + bend * (1.0 + k as f32 * 0.3);
        x -= a.cos() * h;
        y -= a.sin() * h;
        back.push((x, y));
    }
    back.reverse();
    back.extend(fwd);
    if rng.chance(0.5) {
        back.reverse();
    }
    back
}
