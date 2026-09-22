//! Working an area with a real (simulated) brush.
//!
//! `Handling` describes how a painter works a region: which tool, how long
//! the strokes are and which way they run, how hard they press, how often
//! they go back to the palette and whether they wipe the brush first.
//! `Canvas::work` then drives a `Held` brush over the region stroke by
//! stroke, so all mixing, smearing, dry-brush and ridges come from the
//! bristle simulation, not from blend modes.

use crate::bristle::{Gesture, Held, Orient, Tool};
use crate::canvas::Canvas;
use crate::color::{Rgb, from_oklab, to_oklab};
use crate::mask::Mask;
use crate::rng::Rng;
use crate::wet::Paint;

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
    /// Hiding and body of the mixed paint (see `Paint`).
    pub hiding: f32,
    pub body: f32,
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
            body: 0.8,
            pressure: (0.6, 0.9),
            orient: Orient::Across,
            dip_every: 1,
            load: 0.8,
            wipe: 0.6,
            blender: false,
            scrub: 0,
            clip: false,
            threshold: 0.3,
            ramps: (0.08, 0.15),
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
    pub fn paint(mut self, hiding: f32, body: f32) -> Self {
        self.hiding = hiding;
        self.body = body;
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
}

impl Canvas {
    /// Work the region `mask` with a simulated brush, stroke by stroke.
    pub fn work(&mut self, mask: &Mask, hd: &Handling, seed: u64) {
        let mut rng = Rng::new(seed);
        let f = self.f;
        // one stroke per gap² of area gives coverage = width · length / gap²
        let mean_len = 0.5 * (hd.length.0 + hd.length.1);
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
        // painters work in passages, not scanlines or pure noise: sort centers
        // into coarse patches, shuffle patches, shuffle within
        let patch = gap * 8.0;
        let mut keyed: Vec<(u64, (f32, f32))> = centers
            .into_iter()
            .map(|(x, y)| {
                let key = crate::rng::hash2((x / patch) as i64, (y / patch) as i64, seed) * 1e6;
                ((key as u64) << 20 | (rng.next_u64() & 0xFFFFF), (x, y))
            })
            .collect();
        keyed.sort_by_key(|k| k.0);

        let mut held = Held::new(hd.tool.clone(), seed ^ 0x5EED);
        let clip = if hd.clip { Some(mask) } else { None };
        for (n, &(_, (cx, cy))) in keyed.iter().enumerate() {
            if n % hd.dip_every == 0 {
                if hd.blender {
                    held.wipe(0.9);
                } else {
                    let lab = to_oklab((hd.color)(cx, cy));
                    let col = from_oklab([
                        lab[0] + rng.normal() * hd.jitter.0,
                        lab[1] + rng.normal() * hd.jitter.1,
                        lab[2] + rng.normal() * hd.jitter.1,
                    ]);
                    held.wipe(hd.wipe);
                    held.load(Paint { color: col, hiding: hd.hiding, body: hd.body }, hd.load);
                }
            }
            let len = rng.range(hd.length.0, hd.length.1);
            let bend = rng.normal() * hd.angle_jitter;
            let pts = if hd.scrub > 0 {
                // short zigzag across the center along the field
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
            let p = rng.range(hd.pressure.0, hd.pressure.1);
            let g = Gesture::new(pts).pressure(p, p * rng.range(0.75, 1.05)).orient(hd.orient).ramps(hd.ramps.0, hd.ramps.1);
            self.drag(&mut held, &g, clip);
        }
    }
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
