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
    /// Where the painter loads the brush more or less (multiplies `load`,
    /// evaluated at each stroke's center): a glaze goes on deeper where the
    /// brush carries more.
    pub load_at: Option<Field<'a, f32>>,
    /// Cut in the region's edges with this brush instead of clipping: body
    /// strokes stop short of the edge, then short strokes follow the outline.
    pub cut_in: Option<Tool>,
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
            load_at: None,
            cut_in: None,
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
    /// Cut the region's edges in with `tool` (see `cut_in`).
    pub fn cut_in(mut self, tool: Tool) -> Self {
        self.cut_in = Some(tool);
        self
    }
    /// Vary the load across the canvas (see `load_at`).
    pub fn load_at(mut self, f: impl Fn(f32, f32) -> f32 + Sync + 'a) -> Self {
        self.load_at = Some(Box::new(f));
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
    /// How much of a full load that dip takes.
    load: f32,
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
        // plan on the whole canvas (also in a crop render, so the strokes
        // are the same ones); run_plans paints only what reaches the window
        let f = mask.f;
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
            // cutting in: the body strokes stop short of the edge
            let pts = if hd.cut_in.is_some() { trim_inside(mask, &pts, (cx, cy), hd.tool.width) } else { pts };
            if pts.is_empty() {
                continue;
            }
            let (rect, plan) = finish_plan(hd, &hd.tool, (cx, cy), pts, f.scale, (f.w, f.h), &mut rng);
            if let Some(r) = rect {
                let (px, py) = (cx * f.scale, cy * f.scale);
                ex = ex.max((px - r.0 as f32).max(r.2 as f32 - px) / f.scale);
                ey = ey.max((py - r.1 as f32).max(r.3 as f32 - py) / f.scale);
            }
            plans.push((cx, cy, rect, plan));
        }
        let clip = if hd.clip && hd.cut_in.is_none() { Some(mask) } else { None };
        self.run_plans(plans, (ex, ey), gap, &hd.tool, hd, hd.ramps, clip, seed, &mut rng);
        if let Some(edge) = &hd.cut_in {
            self.cut_in_edges(mask, edge, hd, seed ^ 0xED6E, &mut rng);
        }
    }

    /// Cut in the edges of `mask`: short strokes of the `tool` laid along the
    /// region's outline, just inside it, the way a painter sharpens a form.
    fn cut_in_edges(&mut self, mask: &Mask, tool: &Tool, hd: &Handling, seed: u64, rng: &mut Rng) {
        let f = mask.f;
        let step = (tool.width * 0.5).max(1.0 / f.scale);
        let inside = |p: (f32, f32)| p.0 >= 0.0 && p.1 >= 0.0 && p.0 < f.width() && p.1 < f.height() && mask.data[f.index(p.0, p.1)] >= 0.5;
        // rings stepping inward from the edge until they meet the body
        // strokes (which keep half their brush clear of the edge)
        let reach = hd.tool.width * 0.5 + tool.width * 0.5;
        let (mut plans, mut ex, mut ey) = (Vec::new(), 0.0f32, 0.0f32);
        let lines = crate::edge::contours(mask, step);
        let mut ring = 0;
        loop {
            let inset = tool.width * (0.45 + 0.8 * ring as f32);
            if inset > reach.max(tool.width * 0.5) {
                break;
            }
            for line in &lines {
                // offset inward, split where the offset leaves the region
                let mut runs: Vec<Vec<(f32, f32)>> = vec![vec![]];
                for &((x, y), (nx, ny)) in line {
                    let q = (x + nx * inset, y + ny * inset);
                    if inside(q) {
                        runs.last_mut().unwrap().push(q);
                    } else if !runs.last().unwrap().is_empty() {
                        runs.push(vec![]);
                    }
                }
                for run in runs.into_iter().filter(|r| r.len() >= 2) {
                    // overlapping strokes along the run
                    let mut k = 0;
                    while k + 1 < run.len() {
                        let len = rng.range(6.0, 16.0) * tool.width.max(1.0);
                        let (mut acc, mut j) = (0.0, k);
                        while j + 1 < run.len() && acc < len {
                            acc += ((run[j + 1].0 - run[j].0).powi(2) + (run[j + 1].1 - run[j].1).powi(2)).sqrt();
                            j += 1;
                        }
                        let pts: Vec<(f32, f32)> = run[k..=j].to_vec();
                        let c = pts[pts.len() / 2];
                        let (rect, plan) = finish_plan(hd, tool, c, pts, f.scale, (f.w, f.h), rng);
                        if let Some(r) = rect {
                            let (px, py) = (c.0 * f.scale, c.1 * f.scale);
                            ex = ex.max((px - r.0 as f32).max(r.2 as f32 - px) / f.scale);
                            ey = ey.max((py - r.1 as f32).max(r.3 as f32 - py) / f.scale);
                        }
                        plans.push((c.0, c.1, rect, plan));
                        if j + 1 >= run.len() {
                            break;
                        }
                        // next stroke starts a third of the way back
                        k = (k + (j - k) * 2 / 3).max(k + 1);
                    }
                }
            }
            ring += 1;
        }
        if !plans.is_empty() {
            self.run_plans(plans, (ex, ey), tool.width * 4.0, tool, hd, (0.03, 0.08), None, seed, rng);
        }
    }

    /// Paint planned strokes: group them into tiles, then paint the tiles in
    /// four checkerboard phases, tiles within a phase in parallel.
    #[allow(clippy::too_many_arguments)]
    fn run_plans(&mut self, plans: Vec<(f32, f32, Option<Rect>, Plan)>, (ex, ey): (f32, f32), gap: f32, tool: &Tool, hd: &Handling, ramps: (f32, f32), clip: Option<&Mask>, seed: u64, rng: &mut Rng) {
        let f = self.f;
        // tiles are sized per axis from the footprints: tiles painted at the
        // same time are one tile apart, so a tile at least twice the largest
        // reach keeps their pixels disjoint (long horizontal strokes get wide,
        // short tiles)
        let margin = 2.0 / f.scale;
        let (tile_x, tile_y) = ((2.0 * ex + margin).max(gap * 2.0), (2.0 * ey + margin).max(gap * 2.0));
        let (tw, th) = ((f.width() / tile_x).ceil().max(1.0) as usize, (f.height() / tile_y).ceil().max(1.0) as usize);
        if std::env::var_os("PAINT_DEBUG").is_some() {
            eprintln!("work: {} strokes, reach {ex:.0}x{ey:.0}, tiles {tw}x{th}", plans.len());
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
        let mut phases = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)];
        for i in (1..4).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            phases.swap(i, j);
        }
        // the painting order: phases, then batches of disjoint tiles
        let mut order = Vec::new();
        for (px, py) in phases {
            let idx: Vec<usize> = (0..tiles.len()).filter(|&i| (i % tw) % 2 == px && (i / tw) % 2 == py && tile_rect[i].is_some()).collect();
            let batches = disjoint_batches(&idx, &tile_rect);
            if std::env::var_os("PAINT_DEBUG").is_some() {
                eprintln!("  phase: {} tiles in {} batches", idx.len(), batches.len());
            }
            // a crop render skips passages that miss its window
            order.extend(batches.into_iter().flatten().filter(|&i| tile_rect[i].is_some_and(|r| f.clip(r).is_some())));
        }
        let paint_tile = |ti: usize| {
            let mut held = Held::new(tool.clone(), seed ^ 0x5EED ^ (ti as u64).wrapping_mul(0x9E37_79B9));
            let mut scratch = Vec::new();
            let mut b: crate::bristle::Bounds = None;
            for (k, p) in tiles[ti].iter().enumerate() {
                if let Some(paint) = p.dip {
                    if hd.blender {
                        held.wipe(0.9);
                    } else {
                        held.wipe(hd.wipe);
                        held.load(paint, p.load);
                    }
                }
                let g = Gesture::new(p.pts.clone())
                    .pressure(p.pressure, p.pressure * p.fade)
                    .orient(hd.orient)
                    .ramps(ramps.0, ramps.1)
                    .shake(hd.shake);
                let id = first_id.wrapping_add(offsets[ti] + k as u32);
                // SAFETY: every pixel this drag touches lies in its stroke
                // footprint, inside this tile's rect; run_ordered never runs
                // tiles with overlapping rects at once; `surf()` checked the
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
        };
        // tiles run in parallel wherever that can't change the result
        let mut dirty: crate::bristle::Bounds = None;
        for r in crate::sched::run_ordered(&order, &tile_rect, paint_tile).into_iter().flatten() {
            dirty = Some(match dirty {
                None => r,
                Some((a0, c0, d0, e0)) => (a0.min(r.0), c0.min(r.1), d0.max(r.2), e0.max(r.3)),
            });
        }
        if let Some((x0, y0, x1, y1)) = dirty {
            self.wet.touch(x0, y0, x1, y1);
        }
    }
}

/// Footprint and the per-stroke choices (pressure, fade, the pile of paint,
/// the load) for a stroke along `pts` centered at `c`.
fn finish_plan(hd: &Handling, tool: &Tool, c: (f32, f32), pts: Vec<(f32, f32)>, scale: f32, (w, h): (usize, usize), rng: &mut Rng) -> (Option<Rect>, Plan) {
    let rect = footprint(tool, &pts, hd.shake, scale, w, h);
    let pressure = rng.range(hd.pressure.0, hd.pressure.1);
    let fade = rng.range(0.75, 1.05);
    let target = (hd.color)(c.0, c.1);
    let paint = match hd.palette {
        // on the palette: mix the pile from tubes, never twice alike
        Some((pal, medium)) => pal.remix(&pal.mix(target), hd.mix_jitter, rng).paint(medium),
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
    let load = hd.load * hd.load_at.as_ref().map_or(1.0, |f| f(c.0, c.1).max(0.0));
    (rect, Plan { pts, pressure, fade, dip: Some(paint), load })
}

/// The part of a stroke through `c` that stays inside `mask` (≥ 0.5), pulled
/// back by about half the brush from any edge it would cross.
fn trim_inside(mask: &Mask, pts: &[(f32, f32)], c: (f32, f32), width: f32) -> Vec<(f32, f32)> {
    let f = mask.f;
    // resample the stroke at ~1 unit
    let mut dense = vec![pts[0]];
    for w in pts.windows(2) {
        let d = ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt();
        let n = d.ceil().max(1.0) as usize;
        for k in 1..=n {
            let t = k as f32 / n as f32;
            dense.push((w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t));
        }
    }
    let at = |p: (f32, f32)| p.0 >= 0.0 && p.1 >= 0.0 && p.0 < f.width() && p.1 < f.height() && mask.data[f.index(p.0, p.1)] >= 0.5;
    // inside with half a brush of room on both sides of the path
    let clear = width * 0.45;
    let inside_at = |i: usize| {
        let (a, b) = (dense[i.saturating_sub(1)], dense[(i + 1).min(dense.len() - 1)]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let n = (dx * dx + dy * dy).sqrt().max(1e-6);
        let (nx, ny) = (-dy / n * clear, dx / n * clear);
        let p = dense[i];
        at(p) && at((p.0 + nx, p.1 + ny)) && at((p.0 - nx, p.1 - ny))
    };
    let ci = (0..dense.len()).min_by(|&a, &b| {
        let da = (dense[a].0 - c.0).powi(2) + (dense[a].1 - c.1).powi(2);
        let db = (dense[b].0 - c.0).powi(2) + (dense[b].1 - c.1).powi(2);
        da.partial_cmp(&db).unwrap()
    }).unwrap();
    let (mut a, mut b) = (ci, ci);
    if !inside_at(ci) {
        // no room for this brush here: the cutting-in brush fills it
        return Vec::new();
    }
    while a > 0 && inside_at(a - 1) {
        a -= 1;
    }
    while b + 1 < dense.len() && inside_at(b + 1) {
        b += 1;
    }
    let back = (width * 0.5).round() as usize;
    if a > 0 {
        a = (a + back).min(ci);
    }
    if b + 1 < dense.len() {
        b = b.saturating_sub(back).max(ci);
    }
    if b <= a {
        // too close to the edge for a stroke: a short touch at the center
        return vec![dense[ci], dense[(ci + 1).min(dense.len() - 1)]];
    }
    dense[a..=b].iter().step_by(4).copied().chain(std::iter::once(dense[b])).collect()
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
