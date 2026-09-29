//! Working an area with a real (simulated) brush.
//!
//! `Handling` describes how a region is worked: which tool, how long
//! the strokes are and which way they run, how hard they press, how often
//! they go back to the palette and whether they wipe the brush first.
//! `Canvas::work` then drives a `Held` brush over the region stroke by
//! stroke, so all mixing, smearing, dry-brush and ridges come from the
//! bristle simulation, not from blend modes.

use crate::bristle::{Clip, Gesture, Held, Orient, Rect, Tool, footprint};
use crate::canvas::{Canvas, Frame};
use crate::color::{Rgb, from_oklab, to_oklab};
use crate::mask::Mask;
use crate::palette::Palette;
use crate::rng::Rng;
use crate::sched::TileOrder;
use crate::tally::Piles;
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
    /// Raw paint color for a stroke centered at a point.
    pub color: Field<'a, Rgb>,
    /// Raw paint color variation per dip: OKLab L and a/b sd.
    pub jitter: (f32, f32),
    /// Hiding and stiffness of raw paint (see `Paint`).
    pub hiding: f32,
    pub stiff: f32,
    /// How unevenly each pile is mixed: relative sd of the proportions.
    pub mix_jitter: f32,
    /// Dip into this pile (knifed from `palette`'s tubes in the given
    /// parts, thinned with this fraction of medium) for every stroke. When
    /// set, the color field is not
    /// used: the paint is the pile's (`piled`).
    pub pile: Option<(&'a Palette, crate::palette::Mixture, f32)>,
    /// Where the brush is loaded more or less (multiplies `load`,
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
    /// A hard limit no bristle paints outside of, whatever `clip` says (e.g.
    /// the part of a region not covered by another shape in front of it),
    /// while strokes still overshoot the region's own edges.
    pub limit: Option<std::sync::Arc<Mask>>,
    /// Instead of a stencil clip, a fence (`crate::fence`): each stroke
    /// overruns the region's edge by its own amount, found, soft or lost
    /// along the contour as the fence's quality says. Set by `fence()`.
    pub fence: Option<std::sync::Arc<crate::fence::Fence>>,
    /// Look and fill: after the strokes, dab paint into the bare spots the
    /// strokes left in the region (see `fill`). `None`: off.
    pub fill: Option<bool>,
    /// Hug the region's edges: strokes seeded just outside it (within half a
    /// brush) are moved onto its edge, so coverage doesn't thin there
    /// (default on; see `hug`).
    pub hug: bool,
    /// Minimum mask value for a stroke center.
    pub threshold: f32,
    /// Press-down and lift-off fractions of each stroke.
    pub ramps: (f32, f32),
    /// Hand unsteadiness (1 = normal).
    pub shake: f32,

    // ---- stroke geometry: how far strokes depart from ruler lines (see `ruler()`)
    /// Typical bow of a stroke: its sagitta as a fraction of its length (sd).
    /// 70% of simple arcs bulge up and to the left, away from a pivot below
    /// and to the right of the stroke. 0 = straight.
    pub curve: f32,
    /// Share of curved strokes that are S-shaped instead of simple arcs.
    pub wave: f32,
    /// Criss-cross: strokes fall into two families at ± this angle (radians)
    /// to the direction field. 0 = one family.
    pub cross: f32,
    /// The direction wanders across a passage: amplitude (radians) and the
    /// size (units) of the wandering.
    pub drift: (f32, f32),
    /// Share of strokes outside the length range: short dabs and long sweeps.
    pub tail: f32,
    /// Share of strokes lifted partway and restarted a little off the line.
    pub broken: f32,
    /// Pressure variation along a stroke (relative sd at a few knots).
    pub swell: f32,
    /// Uneven density: strokes crowd in some places and thin in others
    /// (relative amplitude of the stroke density, 0 = even).
    pub clump: f32,
    /// The order the area is worked in, if asked for (`order()`, `sweep()`,
    /// `ruler()`); None: the handling's default, `Order::Passages`, whose
    /// passages hand time paints in a sweep down (`Canvas::paint_pass`).
    pub order: Option<Order>,
}

/// The order an area's strokes are painted in.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Order {
    /// Passage by passage: the area is worked patch by patch, the strokes in a
    /// patch laid side by side, row after row across it.
    #[default]
    Passages,
    /// One sweep across the whole area in the given direction (radians;
    /// `FRAC_PI_2` = top to bottom), band by band, so later strokes don't
    /// drag paint back across bands already worked.
    Sweep(f32),
    /// Anywhere, in random order.
    Scatter,
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
            limit: None,
            fence: None,
            fill: None,
            hug: true,
            threshold: 0.3,
            ramps: (0.08, 0.15),
            shake: 1.0,
            mix_jitter: 0.08,
            pile: None,
            load_at: None,
            cut_in: None,
            curve: 0.05,
            wave: 0.25,
            cross: 0.0,
            drift: (0.12, 300.0),
            tail: 0.12,
            broken: 0.06,
            swell: 0.15,
            clump: 0.3,
            order: None,
        }
    }
    /// Ruler strokes: straight, even, evenly spread, in random order
    /// (`curve`, `cross`, the `drift` amount, `tail`, `broken`, `swell` and
    /// `clump` set to 0; `Order::Scatter`).
    pub fn ruler(mut self) -> Self {
        self.curve = 0.0;
        self.cross = 0.0;
        self.drift = (0.0, self.drift.1);
        self.tail = 0.0;
        self.broken = 0.0;
        self.swell = 0.0;
        self.clump = 0.0;
        self.order(Order::Scatter)
    }
    /// Bow strokes into arcs (sagitta / length, sd); `wave` of them are S-curves.
    pub fn curve(mut self, bow: f32, wave: f32) -> Self {
        self.curve = bow;
        self.wave = wave;
        self
    }
    /// Criss-cross: two stroke families at ± `angle` to the direction field.
    pub fn cross(mut self, angle: f32) -> Self {
        self.cross = angle;
        self
    }
    /// Let the direction wander by up to about `amount` radians over `scale` units.
    pub fn drift(mut self, amount: f32, scale: f32) -> Self {
        self.drift = (amount, scale.max(1.0));
        self
    }
    /// Share of strokes outside the length range (dabs and long sweeps).
    pub fn tail(mut self, share: f32) -> Self {
        self.tail = share.clamp(0.0, 1.0);
        self
    }
    /// Share of strokes lifted and restarted partway.
    pub fn broken(mut self, share: f32) -> Self {
        self.broken = share.clamp(0.0, 1.0);
        self
    }
    /// Pressure variation along each stroke.
    pub fn swell(mut self, sd: f32) -> Self {
        self.swell = sd;
        self
    }
    /// Uneven stroke density (0 = even).
    pub fn clump(mut self, amount: f32) -> Self {
        self.clump = amount.clamp(0.0, 1.0);
        self
    }
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }
    /// Work the area in one sweep in direction `angle` (see `Order::Sweep`).
    pub fn sweep(self, angle: f32) -> Self {
        self.order(Order::Sweep(angle))
    }
    /// Mean stroke length including the tails of the distribution.
    fn mean_length(&self) -> f32 {
        let (a, b) = self.length;
        let t = self.tail;
        (1.0 - t) * 0.5 * (a + b) + t * DAB_SHARE * 0.45 * a + t * (1.0 - DAB_SHARE) * 1.35 * b
    }
    pub fn length(mut self, a: f32, b: f32) -> Self {
        self.length = (a, b);
        self
    }
    /// How many times over the strokes cover the region (width × length ×
    /// count ÷ area). 0 disables the pass; negative or non-finite panics.
    pub fn coverage(mut self, c: f32) -> Self {
        assert!(c.is_finite() && c >= 0.0, "Handling::coverage must be finite and ≥ 0, got {c}");
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
        self
    }
    /// Dip every stroke into `pile` (parts of `palette`'s tubes), thinned
    /// with `medium` (0..1): the pass lays that pile as knifed, remixed a
    /// little per dip by `mix_jitter` (a pile mixed by hand is uneven),
    /// drying at its tubes' rate. Nothing is aimed or matched.
    pub fn piled(mut self, palette: &'a Palette, pile: crate::palette::Mixture, medium: f32) -> Self {
        self.pile = Some((palette, pile, medium.clamp(0.0, 1.0)));
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
    /// Carry the passage up to the region's edge the way a brush does (see
    /// `crate::fence`) instead of stopping every bristle on it: strokes are
    /// clipped as with `clip(true)`, but each by its own overrun.
    pub fn fence(mut self, fence: std::sync::Arc<crate::fence::Fence>) -> Self {
        self.fence = Some(fence);
        self.clip = true;
        self
    }
    pub fn clip(mut self, on: bool) -> Self {
        self.clip = on;
        self
    }
    /// Look and fill: after the strokes, lay a short dab into each bare spot
    /// they left in the region (see `fill_gaps`). Off by default;
    /// `true` explicitly fills them at any nonzero coverage.
    pub fn fill(mut self, on: bool) -> Self {
        self.fill = Some(on);
        self
    }
    /// Whether this pass fills the gaps its strokes leave (see `fill`).
    pub fn fills(&self) -> bool {
        self.fill.unwrap_or(false)
    }
    /// Never paint outside `m` (see `limit`).
    pub fn limit(mut self, m: std::sync::Arc<Mask>) -> Self {
        self.limit = Some(m);
        self
    }
    /// Hug the region's edges (default on): stroke centers seeded just
    /// outside the region (within half a brush) move onto its edge, so the
    /// edge gets as many strokes as the middle. Off: stroke centers fall
    /// only inside the region, and coverage halves along its edges.
    pub fn hug(mut self, on: bool) -> Self {
        self.hug = on;
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
    /// Pressure swell knots along the stroke (see `Gesture::swell`).
    swell: Vec<f32>,
    /// The passage this stroke belongs to: moving on to another passage is
    /// always a trip to the palette.
    passage: u32,
    /// Take a fresh brush for this stroke (its own, seeded by where the
    /// stroke starts).
    fresh: bool,
    /// Its stroke id, if fixed in advance (see `fill_gaps`); else the next.
    id: Option<u32>,
    /// The paint color: which pile
    /// on the palette the dip comes from (`tally::Piles`).
    want: Rgb,
}

impl Canvas {
    /// Work the region `mask` with a simulated brush, stroke by stroke.
    ///
    /// Strokes are planned up front, then grouped into passages (tiles) at
    /// least twice any stroke's reach on each axis. The tiles are painted in
    /// an order set by `Order` (2×2 checkerboard phases in shuffled order,
    /// or bands along a sweep), each with its own brush; tiles whose
    /// footprints don't overlap run in parallel (`sched::run_ordered`), with
    /// the same result as painting them one by one.
    pub fn work(&mut self, mask: &Mask, hd: &Handling, seed: u64) {
        self.work_with(&mut Piles::default(), mask, hd, seed);
    }

    /// `work`, dipping into the piles already mixed on the palette (a
    /// sitting's, shared by its passes and held brushes) and leaving the
    /// new ones there: which trips to the palette are reloads and which are
    /// new mixes, in the hand's ledger (`tally`). `work` starts from a
    /// clean palette.
    pub fn work_with(&mut self, piles: &mut Piles, mask: &Mask, hd: &Handling, seed: u64) {
        hd.tool.assert_valid();
        if let Some(t) = &hd.cut_in {
            t.assert_valid();
        }
        self.check_mask(mask);
        // coverage 0 disables the pass
        assert!(hd.coverage.is_finite() && hd.coverage >= 0.0, "Handling::coverage must be finite and ≥ 0, got {}", hd.coverage);
        if hd.coverage == 0.0 {
            return;
        }
        let mut rng = Rng::new(seed);
        // plan on the whole canvas (also in a crop render, so the strokes
        // are the same ones); run_plans paints only what reaches the window
        let f = mask.f;
        let mean_len = hd.mean_length();
        // one stroke per gap² of area gives coverage = width · length / gap²
        let gap = (hd.tool.width * mean_len.max(hd.tool.width) / hd.coverage.max(0.05)).sqrt().max(0.5);
        let centers = place(hd, f, gap, mean_len, seed, &mut rng);
        let drift = crate::noise::Fbm::new((seed as u32) ^ 0xD21F, 3, hd.drift.1);
        // clipped strokes may start outside the region and brush into it
        let reach_in = hd.clip && hd.cut_in.is_none();
        // unclipped strokes seeded outside that brush into the region are
        // carried in from its edge (see `carry_in`)
        let carry = !hd.clip && hd.cut_in.is_none() && hd.hug && !hd.blender;

        // plan every stroke deterministically
        let mut plans = Vec::with_capacity(centers.len());
        // how far (units) any stroke's pixel footprint reaches from its center
        let (mut ex, mut ey) = (0.0f32, 0.0f32);
        for &(cx, cy, passage) in &centers {
            let mut inside = mask_at(mask, cx, cy) >= hd.threshold;
            // hug the edges: a center just outside the region (within half a
            // brush across the stroke) moves onto its edge, so the edge gets
            // as many strokes as the inside instead of half as many. The
            // stroke then overhangs the edge by half a brush at most, as one
            // centered on the edge does (clipped, it lays a full edge).
            let (cx, cy) = if inside || hd.cut_in.is_some() || !hd.hug {
                (cx, cy)
            } else {
                match hug_edge(hd, mask, (cx, cy)) {
                    Some(p) => {
                        inside = true;
                        p
                    }
                    None => (cx, cy),
                }
            };
            if !inside && !reach_in && !carry {
                continue;
            }
            let len = stroke_length(hd, &mut rng);
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
                hand_trace(hd, &drift, cx, cy, len, bend, &mut rng)
            };
            // the stroke's anchor: its center, or where a stroke seeded outside
            // the region first enters it (its color and passage are taken there)
            let (cx, cy) = if inside {
                (cx.clamp(0.0, f.width()), cy.clamp(0.0, f.height()))
            } else {
                match entry(mask, f, &pts, (cx, cy), hd.threshold) {
                    Some(p) => p,
                    None => continue,
                }
            };
            // cutting in: the body strokes stop short of the edge
            let pts = if hd.cut_in.is_some() { trim_inside(mask, &pts, (cx, cy), hd.tool.width) } else { pts };
            // carried in: from half a brush outside the edge, inward
            let pts = if carry && !inside { carry_in(mask, &pts, (cx, cy), hd.tool.width, hd.threshold) } else { pts };
            if pts.is_empty() {
                continue;
            }
            let pieces = if hd.scrub > 0 { vec![pts] } else { break_stroke(hd, pts, &mut rng) };
            for (k, piece) in pieces.into_iter().enumerate() {
                let n_knots = 2 + (len / (4.0 * hd.tool.width.max(1.0))).clamp(1.0, 4.0) as usize;
                let (rect, mut plan) = finish_plan(self, hd, &hd.tool, (cx, cy), piece, &mut rng);
                if k > 0 {
                    // a restart carries on with the paint left on the brush
                    plan.dip = None;
                }
                plan.passage = passage as u32;
                // a dab takes only a touch of paint, not a full stroke's load
                plan.load *= (len / hd.length.0.max(1e-3)).clamp(0.25, 1.0);
                if hd.swell > 0.0 {
                    plan.swell = (0..n_knots).map(|_| (1.0 + rng.normal() * hd.swell).clamp(0.35, 1.6)).collect();
                }
                if let Some(r) = rect {
                    let (px, py) = (cx * f.scale, cy * f.scale);
                    ex = ex.max((px - r.0 as f32).max(r.2 as f32 - px) / f.scale);
                    ey = ey.max((py - r.1 as f32).max(r.3 as f32 - py) / f.scale);
                }
                plans.push((cx, cy, rect, plan));
            }
        }
        if plans.is_empty() {
            return;
        }
        let limited;
        let clip = match (&hd.limit, hd.clip && hd.cut_in.is_none()) {
            (Some(l), true) => {
                limited = mask.clone().mul(l);
                Some(&limited)
            }
            (Some(l), false) => Some(&**l),
            (None, true) => Some(mask),
            (None, false) => None,
        };
        // a fence instead of a stencil (the stroke's own overrun is drawn in run_plans)
        let clip = match &hd.fence {
            Some(fe) if hd.clip && hd.cut_in.is_none() => Some(Clip::Fence { fence: fe, u: 0.0, limit: hd.limit.as_deref() }),
            _ => clip.map(Clip::Mask),
        };
        let before = self.wet.current;
        let slice = self.hand_slice_secs();
        // with hand time on, the pass's first slices can set before the look
        // (baked into the dry film, no longer wet): the film before the pass
        // tells that paint from a gap
        let film0 = (hd.fills() && slice.is_some()).then(|| self.film.clone());
        self.run_plans(plans, (ex, ey), gap, &hd.tool, hd, hd.ramps, clip, seed, &mut rng, piles, slice);
        if hd.fills() {
            self.fill_gaps(mask, hd, before, film0.as_deref(), clip, seed);
        }
        if let Some(edge) = &hd.cut_in {
            self.cut_in_edges(mask, edge, hd, seed ^ 0xED6E, &mut rng, piles);
        }
    }

    /// Look and fill: find the bare spots the pass (strokes with ids above
    /// `before`) left inside the region: pixels it didn't reach, or where
    /// it laid less than `FILL_BARE`, wet or (with `film0`, the dry film
    /// before a hand-timed pass) set since. Lay a short stroke through each.
    /// The spots are gathered on a grid
    /// of cells half a brush wide (units, so any resolution fills the same
    /// spots); a cell is filled when its bare area is at least 0.5% of a
    /// brush width squared, and it is filled once: this is one look, not a
    /// loop. Deterministic (its own random stream), and it sees only the
    /// pixels the canvas holds (a crop's margin is wider than a fill stroke
    /// reaches).
    fn fill_gaps(&mut self, mask: &Mask, hd: &Handling, before: u32, film0: Option<&[f32]>, clip: Option<Clip<'_>>, seed: u64) {
        let f = self.f;
        let w = hd.tool.width.max(0.3);
        let cell = (0.5 * w).max(1.5 / f.scale);
        let (cw, ch) = ((f.width() / cell).ceil() as usize, (f.height() / cell).ceil() as usize);
        // per cell: bare pixels and their summed position
        let mut acc: std::collections::BTreeMap<usize, (u32, f32, f32)> = std::collections::BTreeMap::new();
        let thr = hd.threshold.max(0.5);
        for y in 0..f.h {
            let uy = f.uy(y);
            for x in 0..f.w {
                let i = y * f.w + x;
                let laid = match film0 {
                    None => self.wet.vol[i],
                    Some(f0) => self.wet.vol[i] + (self.film[i] - f0[i]),
                };
                let bare = self.wet.stroke[i] <= before || laid < FILL_BARE;
                if !bare {
                    continue;
                }
                let ux = f.ux(x);
                if mask_at(mask, ux, uy) < thr {
                    continue;
                }
                let k = ((uy / cell) as usize).min(ch - 1) * cw + ((ux / cell) as usize).min(cw - 1);
                let e = acc.entry(k).or_insert((0, 0.0, 0.0));
                e.0 += 1;
                e.1 += ux;
                e.2 += uy;
            }
        }
        let px_area = 1.0 / (f.scale * f.scale);
        let least = (0.005 * w * w).max(1.5 * px_area);
        let drift = crate::noise::Fbm::new((seed as u32) ^ 0xD21F, 3, hd.drift.1);
        let len = (0.5 * hd.length.0).clamp(w, 2.0 * w);
        // Everything about a dab follows from its cell alone, so a crop
        // (which sees only some cells) paints the same dabs where it looks:
        // its randomness, its brush (`fresh`), its stroke id (one reserved
        // per cell of the region's bounding box, whether or not it is
        // filled) and the tiles (sized for any dab, not for these).
        let (bx0, by0, bx1, by1) = mask_cells(mask, thr, cell, cw, ch);
        let span = (bx1 + 1 - bx0) * (by1 + 1 - by0);
        let first = if bx1 >= bx0 && by1 >= by0 { self.next_stroke_ids(span as u32) } else { 0 };
        let reach = {
            // a straight dab's footprint (units), plus room for its bow
            const X: f32 = 1.0e4;
            let r = footprint(&hd.tool, &[(X - 0.5 * len, X), (X + 0.5 * len, X)], hd.shake, 1.0, 1 << 16, 1 << 16);
            r.map_or(len, |r| (X - r.0 as f32).max(r.2 as f32 - X).max(X - r.1 as f32).max(r.3 as f32 - X)) + 0.15 * len
        };
        let (mut plans, ex, ey) = (Vec::new(), reach, reach);
        for (key, (n, sx, sy)) in acc {
            if n as f32 * px_area < least {
                continue;
            }
            let (kx, ky) = (key % cw, key / cw);
            if kx < bx0 || kx > bx1 || ky < by0 || ky > by1 {
                continue;
            }
            let mut rng = Rng::new(seed ^ 0xF111_0F11 ^ (key as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let c = (sx / n as f32, sy / n as f32);
            let bend = rng.normal() * hd.angle_jitter;
            let pts = hand_trace(hd, &drift, c.0, c.1, len, bend, &mut rng);
            let (rect, mut plan) = finish_plan(self, hd, &hd.tool, c, pts, &mut rng);
            // a dab takes a touch of paint (as in `work`), on a brush of its
            // own: what one fill dab leaves on the brush can't change
            // another's (a crop sees only some of them)
            plan.load *= (len / hd.length.0.max(1e-3)).clamp(0.25, 1.0);
            plan.fresh = true;
            // (its own passage: a trip to the palette for every dab)
            plan.passage = 0x8000_0000 | key as u32;
            plan.id = Some(first.wrapping_add(((ky - by0) * (bx1 + 1 - bx0) + kx - bx0) as u32));
            plans.push((c.0, c.1, rect, plan));
        }
        if std::env::var_os("PAINT_DEBUG").is_some() {
            let n = f.w * f.h;
            let a = (0..n).filter(|&i| self.wet.stroke[i] <= before).count();
            let b = (0..n).filter(|&i| self.wet.vol[i] < FILL_BARE).count();
            eprintln!("fill: {} strokes; of {n} px, {a} untouched by the pass, {b} thin", plans.len());
        }
        if !plans.is_empty() {
            let mut rng = Rng::new(seed ^ 0xF111);
            // (no palette: every dab is `fresh`, a fraction of a reload of
            // the pass's paint)
            self.run_plans(plans, (ex, ey), w * 2.0, &hd.tool, hd, hd.ramps, clip, seed ^ 0xF111, &mut rng, &mut Piles::default(), None);
        }
    }

    /// Cut in the edges of `mask`: short strokes of the `tool` laid along the
    /// region's outline, just inside it.
    fn cut_in_edges(&mut self, mask: &Mask, tool: &Tool, hd: &Handling, seed: u64, rng: &mut Rng, piles: &mut Piles) {
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
                        let (rect, plan) = finish_plan(self, hd, tool, c, pts, rng);
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
            self.run_plans(plans, (ex, ey), tool.width * 4.0, tool, hd, (0.03, 0.08), hd.limit.as_deref().map(Clip::Mask), seed, rng, piles, None);
        }
    }

    /// Paint planned strokes as one pass (`paint_pass`): group them into
    /// tiles, count them in the hand's ledger, then paint the tiles in
    /// order, in parallel wherever that can't change the result. With hand
    /// time on, `slice` (s) cuts the pass into slices with the paint aging
    /// between them (None for the fill and cut-in sub-passes: their time
    /// goes on the clock when the verb ends).
    #[allow(clippy::too_many_arguments)]
    fn run_plans(&mut self, plans: Vec<(f32, f32, Option<Rect>, Plan)>, (ex, ey): (f32, f32), gap: f32, tool: &Tool, hd: &Handling, ramps: (f32, f32), clip: Option<Clip<'_>>, seed: u64, rng: &mut Rng, piles: &mut Piles, slice: Option<f64>) {
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
        let mut rects: Vec<Option<Rect>> = vec![None; tw * th];
        for (cx, cy, rect, p) in plans {
            let tx = ((cx / tile_x) as usize).min(tw - 1);
            let ty = ((cy / tile_y) as usize).min(th - 1);
            let t = ty * tw + tx;
            if let Some(r) = rect {
                rects[t] = crate::sched::union(rects[t], Some(r));
                tiles[t].push(p);
            }
        }
        // trips to the palette: every `dip_every` strokes within a tile, and
        // whenever the next stroke is in a new passage
        // (and the hand's ledger: every planned stroke and trip, counted on
        // the whole canvas before a crop drops any tiles; see `tally`)
        let mpu = self.mm_per_unit;
        let mut secs = Vec::with_capacity(tiles.len());
        for t in tiles.iter_mut() {
            let secs0 = self.tally.secs;
            let (mut since, mut last) = (0, None);
            for p in t.iter_mut() {
                let moved = last != Some(p.passage);
                last = Some(p.passage);
                if since % hd.dip_every != 0 && !moved {
                    p.dip = None;
                    since += 1;
                } else {
                    since = 1;
                }
                self.tally.stroke(tool, &p.pts, mpu);
                if p.dip.is_some() {
                    if hd.blender {
                        self.tally.wipe();
                    } else if p.fresh {
                        self.tally.reload(1.0 / crate::tally::pace::DABS_PER_RELOAD);
                    } else {
                        piles.trip(&mut self.tally, p.want);
                    }
                }
            }
            secs.push(self.tally.secs - secs0);
        }
        // (the checkerboard's shuffle is drawn whatever order the pass is
        // painted in, so the strokes planned after it don't depend on it)
        let order = tile_order(hd.order.unwrap_or_default(), (tw, th), (tile_x, tile_y), rng);
        // (strokes with ids fixed in advance don't take new ones)
        let ids = tiles.iter().map(|t| t.iter().filter(|p| p.id.is_none()).count() as u32).collect();
        let order = if hd.order.is_some() { TileOrder::Asked(order) } else { TileOrder::Default(order) };
        let pass = crate::sched::Pass { grid: (tw, th), order, rects, secs, ids, slice };
        self.paint_pass(pass, |surf, ti, first_id| {
            let mut held = Held::new(tool.clone(), seed ^ 0x5EED ^ (ti as u64).wrapping_mul(0x9E37_79B9));
            let mut scratch = Vec::new();
            let mut b: crate::bristle::Bounds = None;
            let mut k = 0u32;
            for p in tiles[ti].iter() {
                let id = p.id.unwrap_or_else(|| {
                    k += 1;
                    first_id.wrapping_add(k - 1)
                });
                if p.fresh {
                    held = Held::new(tool.clone(), seed ^ 0xF4E5 ^ (p.pts[0].0.to_bits() as u64) << 20 ^ p.pts[0].1.to_bits() as u64);
                }
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
                    .shake(hd.shake)
                    .swell(p.swell.clone());
                // SAFETY: every pixel this drag touches lies in its stroke
                // footprint, inside this tile's rect; run_ordered never runs
                // tiles with overlapping rects at once; `surf()` checked the
                // buffers match the frame.
                // a fence: this stroke's own overrun, drawn from where it starts
                let clip = match clip {
                    Some(Clip::Fence { fence, limit, .. }) => Some(Clip::Fence { fence, u: crate::fence::Fence::stroke_draw(p.pts[0], seed), limit }),
                    c => c,
                };
                let r = unsafe { crate::bristle::drag_on(surf, &mut held, &g, clip, id, &mut scratch) };
                b = crate::sched::union(b, r);
            }
            b
        });
    }
}

/// Footprint and the per-stroke choices (pressure, fade, the pile of paint,
/// the load) for a stroke along `pts` centered at `c`.
fn finish_plan(cv: &Canvas, hd: &Handling, tool: &Tool, c: (f32, f32), pts: Vec<(f32, f32)>, rng: &mut Rng) -> (Option<Rect>, Plan) {
    // footprints in whole-canvas pixels (a crop render plans the same tiles)
    let f = cv.f;
    let rect = footprint(tool, &pts, hd.shake, f.scale, f.full_w, f.full_h);
    let pressure = rng.range(hd.pressure.0, hd.pressure.1);
    let fade = rng.range(0.75, 1.05);
    let load_k = hd.load_at.as_ref().map_or(1.0, |f| f(c.0, c.1).max(0.0));
    if let Some((pal, pile, medium)) = &hd.pile {
        // the pile on the palette, as knifed (its own mixing generator)
        let mut prng = Rng::new(rng.next_u64());
        let paint = pal.remix(pile, hd.mix_jitter, &mut prng).laid(*medium);
        let load = hd.load * load_k;
        return (rect, Plan { pts, pressure, fade, dip: Some(paint), load, swell: Vec::new(), passage: 0, fresh: false, id: None, want: pile.color });
    }
    let target = (hd.color)(c.0, c.1);
    let lab = to_oklab(target);
    let col = from_oklab([
        lab[0] + rng.normal() * hd.jitter.0,
        lab[1] + rng.normal() * hd.jitter.1,
        lab[2] + rng.normal() * hd.jitter.1,
    ]);
    let paint = Paint::new(col, hd.hiding, hd.stiff);
    let load = hd.load * load_k;
    (rect, Plan { pts, pressure, fade, dip: Some(paint), load, swell: Vec::new(), passage: 0, fresh: false, id: None, want: target })
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

/// A point on the region's edge within half a brush of `c` (across the
/// stroke direction there), nearest first; None if the region isn't there.
fn hug_edge(hd: &Handling, mask: &Mask, c: (f32, f32)) -> Option<(f32, f32)> {
    let f = mask.f;
    let a = (hd.angle)(c.0.clamp(0.0, f.width()), c.1.clamp(0.0, f.height()));
    let (nx, ny) = (-a.sin(), a.cos());
    let w = hd.tool.width;
    let at = |p: (f32, f32)| p.0 >= 0.0 && p.1 >= 0.0 && p.0 < f.width() && p.1 < f.height() && mask_at(mask, p.0, p.1) >= hd.threshold;
    // the first offset that reaches the region; unclipped, a quarter brush
    // further in (a stroke on the very edge line bows and wanders out of it;
    // clipped, the clip cuts that off and the stroke should cover the edge)
    let k = [0.15f32, -0.15, 0.3, -0.3, 0.5, -0.5].into_iter().find(|k| at((c.0 + nx * k * w, c.1 + ny * k * w)))?;
    let deeper = if hd.clip { k } else { k + k.signum() * 0.25 };
    let p = (c.0 + nx * deeper * w, c.1 + ny * deeper * w);
    Some(if at(p) { p } else { (c.0 + nx * k * w, c.1 + ny * k * w) })
}

/// An unclipped stroke seeded outside the region that brushes into it,
/// entering at `c`: the part of it in the region plus a quarter brush past
/// the edge, pulled from the edge inward (the brush goes down at the edge, fully
/// loaded, rather than lifting off there). Stroke ends that would otherwise be
/// missing along edges across the stroke direction fill in, so coverage
/// doesn't thin there, and the overhang is no more than a stroke centered on
/// the edge makes.
fn carry_in(mask: &Mask, pts: &[(f32, f32)], c: (f32, f32), width: f32, threshold: f32) -> Vec<(f32, f32)> {
    let f = mask.f;
    let mut dense = vec![pts[0]];
    for w in pts.windows(2) {
        let d = ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt();
        let n = d.ceil().max(1.0) as usize;
        for k in 1..=n {
            let t = k as f32 / n as f32;
            dense.push((w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t));
        }
    }
    let at = |p: (f32, f32)| p.0 >= 0.0 && p.1 >= 0.0 && p.0 < f.width() && p.1 < f.height() && mask_at(mask, p.0, p.1) >= threshold;
    let ci = (0..dense.len())
        .min_by(|&a, &b| {
            let da = (dense[a].0 - c.0).powi(2) + (dense[a].1 - c.1).powi(2);
            let db = (dense[b].0 - c.0).powi(2) + (dense[b].1 - c.1).powi(2);
            da.total_cmp(&db)
        })
        .unwrap();
    // grow both ways while inside, then half a brush more
    let over = (0.25 * width).ceil() as usize;
    let grow = |step: isize| -> usize {
        let mut i = ci as isize;
        let mut out = 0usize;
        while i + step >= 0 && ((i + step) as usize) < dense.len() {
            let q = dense[(i + step) as usize];
            if at(q) {
                out = 0;
            } else {
                out += 1;
                if out > over {
                    break;
                }
            }
            i += step;
        }
        i as usize
    };
    let (a, b) = (grow(-1), grow(1));
    if b <= a {
        return Vec::new();
    }
    // only strokes that cross the edge (> 30°): one grazing along it is the
    // edge-hugging strokes' job, and carried in it would run along outside
    let (p0, p1) = (dense[ci.saturating_sub(2)], dense[(ci + 2).min(dense.len() - 1)]);
    let (dx, dy) = (p1.0 - p0.0, p1.1 - p0.1);
    let m = |x: f32, y: f32| mask.sample(x.clamp(0.0, f.width() - 1e-3), y.clamp(0.0, f.height() - 1e-3));
    let h = 1.5 / f.scale.min(1.0);
    let (gx, gy) = (m(c.0 + h, c.1) - m(c.0 - h, c.1), m(c.0, c.1 + h) - m(c.0, c.1 - h));
    let (dn, gn) = ((dx * dx + dy * dy).sqrt(), (gx * gx + gy * gy).sqrt());
    // (no edge at the entry: the stroke dips deep into the region well
    // away from its seed, running along the edge)
    if dn <= 1e-6 || gn <= 1e-6 || ((dx * gx + dy * gy) / (dn * gn)).abs() < 0.5 {
        return Vec::new();
    }
    let mut run: Vec<(f32, f32)> = dense[a..=b].iter().step_by(4).copied().chain(std::iter::once(dense[b])).collect();
    // start at the end that lies outside
    if at(dense[a]) && !at(dense[b]) {
        run.reverse();
    }
    run
}

/// Share of the length tail that is short dabs (the rest are long sweeps).
const DAB_SHARE: f32 = 0.6;
/// Film (coats) under which a pixel of the region counts as bare.
const FILL_BARE: f32 = 0.04;

/// Mask value at a point; the region continues past the canvas edges.
/// The cells (of size `cell` units, `cw` × `ch` over the whole canvas)
/// holding the region's pixels at or above `thr`, as an inclusive box
/// (x0, y0, x1, y1); empty (x1 < x0) if there are none.
fn mask_cells(mask: &Mask, thr: f32, cell: f32, cw: usize, ch: usize) -> (usize, usize, usize, usize) {
    use rayon::prelude::*;
    let mf = mask.f;
    let rows: Vec<Option<(usize, usize)>> = mask
        .data
        .par_chunks(mf.w)
        .map(|row| {
            let a = row.iter().position(|&v| v >= thr)?;
            let b = row.iter().rposition(|&v| v >= thr)?;
            Some((a, b))
        })
        .collect();
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for (y, r) in rows.iter().enumerate() {
        if let Some((a, b)) = r {
            x0 = x0.min(*a);
            x1 = x1.max(*b);
            y0 = y0.min(y);
            y1 = y1.max(y);
        }
    }
    if x0 == usize::MAX {
        return (1, 1, 0, 0);
    }
    let c = |px: usize, n: usize| (((px as f32 + 0.5) / mf.scale / cell) as usize).min(n - 1);
    (c(x0, cw), c(y0, ch), c(x1, cw), c(y1, ch))
}

fn mask_at(mask: &Mask, x: f32, y: f32) -> f32 {
    mask.data[mask.f.index(x, y)]
}

/// Stroke centers in working order.
///
/// The area, plus a margin past the canvas edges so strokes brush in from
/// outside, is divided into passages. Each passage gets its own lattice of
/// centers aligned with its stroke direction (rows of strokes laid side by
/// side), jittered cell by cell so coverage stays even, and thinned or
/// crowded by a slow density field (`clump`). Lattices of neighboring
/// passages don't line up, so passages meet raggedly.
fn place(hd: &Handling, f: Frame, gap: f32, mean_len: f32, seed: u64, rng: &mut Rng) -> Vec<(f32, f32, usize)> {
    let w = hd.tool.width.max(0.3);
    let len = mean_len.max(w);
    // spacing along a row and between rows: the same overlap both ways
    let along = gap * (len / w).sqrt();
    let across = gap * (w / len).sqrt();
    let m = 0.5 * len + 0.5 * w;
    let (x0, y0) = (-m, -m);
    let (x1, y1) = (f.width() + m, f.height() + m);
    let side = (1.6 * len).max(6.0 * gap);
    let (nx, ny) = (((x1 - x0) / side).ceil().max(1.0) as usize, ((y1 - y0) / side).ceil().max(1.0) as usize);
    let dens = crate::noise::Fbm::new((seed as u32) ^ 0xC1C1, 2, side * 1.3);
    let mut rank: Vec<usize> = (0..nx * ny).collect();
    for i in (1..rank.len()).rev() {
        let j = (rng.next_u64() % (i as u64 + 1)) as usize;
        rank.swap(i, j);
    }
    // (x, y, passage, passage rank, key within the passage)
    let mut out: Vec<(f32, f32, usize, usize, f32)> = Vec::new();
    for pj in 0..ny {
        for pi in 0..nx {
            let (sx, sy) = (x0 + pi as f32 * side, y0 + pj as f32 * side);
            let (pcx, pcy) = (sx + 0.5 * side, sy + 0.5 * side);
            let a = (hd.angle)(pcx.clamp(0.0, f.width()), pcy.clamp(0.0, f.height()));
            let (ca, sa) = (a.cos(), a.sin());
            let half = side * std::f32::consts::FRAC_1_SQRT_2;
            let (ou, ov) = (rng.f() * along, rng.f() * across);
            let (nu, nv) = ((half / along).ceil() as i32 + 1, (half / across).ceil() as i32 + 1);
            // rows are worked across the passage in one direction or the other
            let dir = if rng.chance(0.5) { 1.0 } else { -1.0 };
            for j in -nv..nv {
                for i in -nu..nu {
                    let (u, v) = ((i as f32 + 0.5) * along + ou, (j as f32 + 0.5) * across + ov);
                    let (qx, qy) = (pcx + u * ca - v * sa, pcy + u * sa + v * ca);
                    // cells wholly outside the passage
                    if (qx - pcx).abs() > 0.5 * side + along + across || (qy - pcy).abs() > 0.5 * side + along + across {
                        continue;
                    }
                    // crowding only adds strokes: thinning would open gaps
                    let d = if hd.clump > 0.0 { 1.0 + hd.clump * 2.0 * dens.get(qx, qy).max(0.0) } else { 1.0 };
                    let n = d.floor() as usize + usize::from(rng.chance(d.fract()));
                    for _ in 0..n {
                        // rows stay near their line (so neighbors overlap and
                        // no ground shows between them); anywhere along a row
                        let (u, v) = ((i as f32 + 0.5 + 0.7 * (rng.f() - 0.5)) * along + ou, (j as f32 + 0.5 + 0.7 * (rng.f() - 0.5)) * across + ov);
                        let (x, y) = (pcx + u * ca - v * sa, pcy + u * sa + v * ca);
                        if x < sx || x >= sx + side || y < sy || y >= sy + side {
                            continue;
                        }
                        let key = match hd.order.unwrap_or_default() {
                            Order::Passages => dir * v + rng.normal() * across * 0.7,
                            Order::Sweep(s) => x * s.cos() + y * s.sin() + rng.normal() * gap * 0.3,
                            Order::Scatter => rng.f(),
                        };
                        out.push((x, y, pj * nx + pi, rank[pj * nx + pi], key));
                    }
                }
            }
        }
    }
    match hd.order.unwrap_or_default() {
        Order::Passages => out.sort_by(|a, b| a.3.cmp(&b.3).then(a.4.total_cmp(&b.4))),
        _ => out.sort_by(|a, b| a.4.total_cmp(&b.4)),
    }
    out.into_iter().map(|(x, y, p, _, _)| (x, y, p)).collect()
}

/// A stroke length: uniform in the range, with a tail of short dabs and
/// long sweeps.
fn stroke_length(hd: &Handling, rng: &mut Rng) -> f32 {
    let (a, b) = hd.length;
    if hd.tail > 0.0 && rng.chance(hd.tail) {
        if rng.chance(DAB_SHARE) { a * rng.range(0.2, 0.7) } else { b * rng.range(1.0, 1.7) }
    } else {
        rng.range(a, b)
    }
}

/// A stroke path through (cx, cy): along the direction field (wandering
/// with `drift`, in one of two families when criss-crossing, turned by the
/// per-stroke `bend`), bowed into an arc or an S (`curve`, `wave`), its
/// direction reversed half the time.
fn hand_trace(hd: &Handling, drift: &crate::noise::Fbm, cx: f32, cy: f32, len: f32, bend: f32, rng: &mut Rng) -> Vec<(f32, f32)> {
    let fam = if hd.cross != 0.0 && rng.chance(0.5) { -hd.cross } else { hd.cross };
    let bow = hd.curve * rng.normal();
    let s_curve = rng.chance(hd.wave);
    let flip = rng.chance(0.7);
    let reverse = rng.chance(0.5);
    let len = len.max(0.1);
    let steps = (3 + (len / 30.0) as usize).min(9);
    let h = len / (2 * steps) as f32;
    let dr = hd.drift.0;
    let run = |bow: f32| {
        let (k0, k1) = if s_curve { (0.0, 48.0 * bow / (len * len)) } else { (8.0 * bow / len, 0.0) };
        let heading = |x: f32, y: f32, s: f32| {
            let d = if dr != 0.0 { dr * drift.get(x, y) } else { 0.0 };
            (hd.angle)(x, y) + d + fam + bend + k0 * s + 0.5 * k1 * s * s
        };
        let mut fwd = vec![(cx, cy)];
        let (mut x, mut y) = (cx, cy);
        for k in 0..steps {
            let a = heading(x, y, (k as f32 + 0.5) * h);
            x += a.cos() * h;
            y += a.sin() * h;
            fwd.push((x, y));
        }
        let mut back = vec![];
        let (mut x, mut y) = (cx, cy);
        for k in 0..steps {
            let a = heading(x, y, -(k as f32 + 0.5) * h);
            x -= a.cos() * h;
            y -= a.sin() * h;
            back.push((x, y));
        }
        back.reverse();
        back.extend(fwd);
        back
    };
    let mut pts = run(bow);
    if bow != 0.0 && !s_curve && flip {
        // 70% of arcs bulge away from a pivot below and to the right of the
        // stroke
        let (a, b) = (pts[0], pts[pts.len() - 1]);
        let m = pts[pts.len() / 2];
        let bulge = (m.0 - 0.5 * (a.0 + b.0), m.1 - 0.5 * (a.1 + b.1));
        if bulge.0 * 0.45 + bulge.1 * 0.9 > 0.0 {
            pts = run(-bow);
        }
    }
    if reverse {
        pts.reverse();
    }
    pts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(tube_box)]
    fn filling_requires_explicit_opt_in() {
        let pal = Palette::tube_box();
        let pile = pal.pile(vec![(0, 1.0)]);
        let run = |fill: Option<bool>| {
            let mut c = Canvas::new(160, 1.0, [0.8; 3]);
            let mask = Mask::full(c.frame());
            let mut hd = Handling::new(Tool::filbert(6.0)).piled(&pal, pile.clone(), 0.2).load(0.3).coverage(1.5);
            if let Some(on) = fill {
                hd = hd.fill(on);
            }
            c.work(&mask, &hd, 3);
            (c.tally().strokes, c.wet_total())
        };
        let off = run(Some(false));
        assert_eq!(run(None), off, "default must leave the strokes' gaps alone");
        let on = run(Some(true));
        assert!(on.0 > off.0 && on.1 != off.1, "explicit fill must add paint dabs");
    }

    /// A pass from a pile lays that pile: its paint and drying rate,
    /// whatever the color field says, every dip from the same pile
    /// (remixed a little: a pile knifed by hand is uneven), never a recipe
    /// the engine chose.
    #[test]
    #[cfg(tube_box)]
    fn a_pass_from_a_pile_lays_the_pile() {
        use crate::bristle::Tool;
        let pal = Palette::tube_box();
        let at = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
        let blue = pal.pile(vec![(at("lead white"), 0.5), (at("Prussian blue"), 0.5)]);
        let run = |mix_jitter: f32| {
            let mut c = Canvas::new(200, 1.0, [0.8; 3]);
            let m = Mask::from_fn(c.frame(), |x, y| if (300.0..700.0).contains(&x) && (300.0..700.0).contains(&y) { 1.0 } else { 0.0 });
            let hd = Handling::new(Tool::filbert(20.0)).color(|_, _| [0.9, 0.05, 0.05]).coverage(3.0).mix_jitter(mix_jitter).piled(&pal, blue.clone(), 0.2);
            c.work(&m, &hd, 3);
            c
        };
        let c = run(0.0);
        let f = c.frame();
        let i = f.index(500.0, 500.0);
        let px = c.seen()[i];
        assert!(px[2] > px[0] + 0.05, "the pile's blue, not the field's red: {px:?}");
        assert_eq!(c.wet.hide[i][2], blue.drying, "the pile's drying rate");
        // with mixing jitter the dips vary around the pile, not toward the field
        let c = run(0.08);
        let d = c.wet.hide[i][2];
        assert!((d - blue.drying).abs() < 0.2 * blue.drying && d != blue.drying, "{d} vs {}", blue.drying);
        let px = c.seen()[i];
        assert!(px[2] > px[0] + 0.05, "{px:?}");
    }

    /// Strokes bow by default and are straight with `ruler()`.
    #[test]
    fn strokes_bow_unless_ruled() {
        use crate::bristle::Tool;
        let bows = |h: &Handling| {
            let drift = crate::noise::Fbm::new(1, 3, 300.0);
            let mut rng = crate::rng::Rng::new(9);
            let mut v: Vec<f32> = (0..200)
                .map(|_| {
                    let p = hand_trace(h, &drift, 500.0, 500.0, 150.0, 0.0, &mut rng);
                    let (a, b, m) = (p[0], p[p.len() - 1], p[p.len() / 2]);
                    let chord = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                    // distance of the middle from the chord, relative to its length
                    ((b.0 - a.0) * (a.1 - m.1) - (a.0 - m.0) * (b.1 - a.1)).abs() / chord / chord
                })
                .collect();
            v.sort_by(f32::total_cmp);
            v[v.len() / 2]
        };
        let hand = bows(&Handling::new(Tool::filbert(10.0)).curve(0.08, 0.0).drift(0.0, 100.0));
        let ruler = bows(&Handling::new(Tool::filbert(10.0)).ruler());
        assert!(hand > 0.03 && ruler < 1e-3, "median bow: hand {hand}, ruler {ruler}");
    }

    /// Every pair of tiles whose footprints overlap is painted in the
    /// requested order (so a parallel run equals a serial one in that order).
    #[test]
    fn tiles_keep_their_order_where_they_overlap() {
        let mut rng = Rng::new(4);
        let (tw, th, tp) = (9usize, 7usize, 40.0f32);
        let rects: Vec<Option<Rect>> = (0..tw * th)
            .map(|t| {
                if rng.chance(0.15) {
                    return None;
                }
                let (x, y) = ((t % tw) as f32 * tp, (t / tw) as f32 * tp);
                let r = rng.range(0.0, 0.5) * tp;
                Some(((x - r).max(0.0) as usize, (y - r).max(0.0) as usize, (x + tp + r) as usize, (y + tp + r) as usize))
            })
            .collect();
        for order in [Order::Passages, Order::Sweep(1.0), Order::Sweep(std::f32::consts::FRAC_PI_2)] {
            let ord = tile_order(order, (tw, th), (tp, tp), &mut rng);
            // the scheduler all tile work runs through (sched::run_ordered)
            let ord: Vec<usize> = ord.into_iter().filter(|&t| rects[t].is_some()).collect();
            let log = std::sync::Mutex::new(Vec::new());
            let ran = crate::sched::run_ordered(&ord, &rects, |t| log.lock().unwrap().push(t));
            let log = log.into_inner().unwrap();
            let pos: Vec<usize> = (0..tw * th).map(|t| ord.iter().position(|&u| u == t).unwrap_or(usize::MAX)).collect();
            let ran_at: Vec<usize> = (0..tw * th).map(|t| log.iter().position(|&u| u == t).unwrap_or(usize::MAX)).collect();
            let ov = |a: Rect, b: Rect| a.0 < b.2 && b.0 < a.2 && a.1 < b.3 && b.1 < a.3;
            for a in 0..tw * th {
                for b in 0..tw * th {
                    if let (Some(ra), Some(rb)) = (rects[a], rects[b])
                        && a != b
                        && ov(ra, rb)
                        && pos[a] < pos[b]
                    {
                        assert!(ran_at[a] < ran_at[b], "{order:?}: tile {a} must run before {b}");
                    }
                }
            }
            assert_eq!(ran.len(), rects.iter().flatten().count());
        }
    }

    /// `.coverage(0.0)` disables a pass: nothing is painted or blended.
    #[test]
    fn zero_coverage_is_a_no_op() {
        use crate::color::{Mix, gradient};
        for blender in [false, true] {
            let mut c = Canvas::new(100, 1.0, [0.9; 3]);
            let m = Mask::full(c.frame());
            // something wet to blend
            c.work(&m, &Handling::new(Tool::filbert(22.0)).color(|x, _| gradient(&[(0.0, [0.1, 0.2, 0.6]), (1.0, [0.9, 0.7, 0.2])], x / 100.0, Mix::Pigment)), 3);
            let before = (c.wet_total(), c.pixels().to_vec());
            let mut hd = Handling::new(Tool::filbert(22.0)).coverage(0.0).color(|_, _| [0.05; 3]);
            if blender {
                hd = hd.blender();
            }
            c.work(&m, &hd, 1);
            assert_eq!(c.wet_total(), before.0, "blender {blender}: wet paint changed");
            c.dry();
            let mut d = Canvas::new(100, 1.0, [0.9; 3]);
            d.work(&m, &Handling::new(Tool::filbert(22.0)).color(|x, _| gradient(&[(0.0, [0.1, 0.2, 0.6]), (1.0, [0.9, 0.7, 0.2])], x / 100.0, Mix::Pigment)), 3);
            d.dry();
            let changed = c.pixels().iter().zip(d.pixels()).filter(|(a, b)| a != b).count();
            assert_eq!(changed, 0, "blender {blender}: {changed} pixels changed");
            let _ = before.1;
        }
    }

    /// `Style::blend()` fuses a masked passage without dragging its wet
    /// paint across the mask's edge;
    /// `.clip(false)` fuses across it.
    #[test]
    #[cfg(tube_box)]
    fn blender_stays_in_its_region() {
        use crate::color::hex;
        let st = crate::style::Style::oil();
        let run = |clip: Option<bool>| {
            let mut c = Canvas::new(200, 1.0, hex("#d8d0c0"));
            let f = c.frame();
            // a wet dark passage below y = 500, a dry light one above
            let below = Mask::from_fn(f, |_, y| if y >= 500.0 { 1.0 } else { 0.0 });
            c.work(&below, &Handling::new(Tool::filbert(30.0)).color(|_, _| hex("#2a2a30")).coverage(3.0).clip(true), 1);
            let before = c.pixels().to_vec();
            let mut b = st.blend().unwrap();
            if let Some(on) = clip {
                b = b.clip(on);
            }
            c.work(&below, &b, 2);
            c.dry();
            // how much darker the light passage got, just above the edge
            let mut worst = 0.0f32;
            for (i, p) in c.pixels().iter().enumerate() {
                let y = (i / f.w) as f32 / f.scale;
                if y < 495.0 && y > 440.0 {
                    worst = worst.max(to_oklab(before[i])[0] - to_oklab(*p)[0]);
                }
            }
            worst
        };
        let (inside, across) = (run(None), run(Some(false)));
        println!("blend: light passage darkened by {inside:.4} (default), {across:.4} (clip(false))");
        assert!(inside < 0.01, "the default blender dragged dark paint out of its region: {inside}");
        assert!(across > 0.05, "clip(false) should still fuse across the edge: {across}");
    }

    /// A dark body passage through a mask over the light ground: (share of
    /// pixels still reading as ground (L within 0.12 of it) in a 5-unit band
    /// inside the mask's edges, the band's L above the interior's, the share
    /// of a 12–30-unit band outside the edges, past the brush that got darker by 0.1 L), for a
    /// band region parallel to the strokes or a disk (edges at every angle).
    #[cfg(tube_box)]
    fn edge_stats(clip: bool, disk: bool, hug: bool, seed: u64) -> (f32, f32, f32) {
        edge_stats_in(clip, disk, false, hug, seed)
    }

    #[cfg(tube_box)]
    fn edge_stats_in(clip: bool, disk: bool, thin: bool, hug: bool, seed: u64) -> (f32, f32, f32) {
        use crate::color::hex;
        let st = crate::style::Style::oil();
        // a knifed light ground on linen (the brushed one costs a pass)
        let mut c = Canvas::new(300, 1.0, st.raw).with_size_mm(st.width_mm).with_linen(crate::surface::Linen { seed, ..st.linen });
        c.prime(hex("#b08457"), 0.8, 120.0, 0.3, 0.3, seed);
        let before = c.pixels().to_vec();
        let g = to_oklab(c.pixels()[c.f.w * c.f.h / 2])[0];
        let f = c.frame();
        let m = if disk {
            Mask::from_fn(f, |x, y| if (x - 500.0).powi(2) + (y - 500.0).powi(2) < 250.0f32.powi(2) { 1.0 } else { 0.0 })
        } else if thin {
            // a band narrower than the broad brush
            Mask::from_fn(f, |_, y| if (400.0..415.0).contains(&y) { 1.0 } else { 0.0 })
        } else {
            Mask::from_fn(f, |_, y| if (400.0..460.0).contains(&y) { 1.0 } else { 0.0 })
        };
        let hd = if thin { st.broad() } else { st.body() };
        let dark = st.palette.pile(vec![(st.palette.tubes.iter().position(|t| t.name == "bone black").unwrap(), 1.0)]);
        c.work(&m, &hd.piled(&st.palette, dark, 0.2).coverage(2.5).fill(true).clip(clip).hug(hug), seed + 4);
        c.dry();
        let sd = m.distance();
        let (mut e, mut en, mut inn, mut sp, mut spn) = (0, 0, 0, 0, 0);
        let (mut le, mut li) = (0.0f32, 0.0f32);
        for (k, p) in c.pixels().iter().enumerate() {
            let d = sd.data[k];
            let l = to_oklab(*p)[0];
            if d < -12.0 && d > -30.0 {
                spn += 1;
                sp += (to_oklab(before[k])[0] - l > 0.1) as usize;
            } else if d > 0.0 && d < 5.0 {
                en += 1;
                e += (l > g - 0.12) as usize;
                le += l;
            } else if d > 8.0 {
                inn += 1;
                li += l;
            }
        }
        (e as f32 / en.max(1) as f32, le / en.max(1) as f32 - li / inn.max(1) as f32, sp as f32 / spn.max(1) as f32)
    }

    /// Coverage doesn't thin at a mask's edges: stroke centers just outside a region hug its edge, and
    /// unclipped strokes that brush into it are carried in from the edge.
    #[test]
    #[ignore]
    #[cfg(tube_box)]
    fn probe_edges_over_seeds() {
        for disk in [false, true] {
            for clip in [false, true] {
                let (mut o, mut n) = ([0.0f32; 3], [0.0f32; 3]);
                for seed in 1..7u64 {
                    let a = edge_stats(clip, disk, false, seed);
                    let b = edge_stats(clip, disk, true, seed);
                    for (acc, v) in [(&mut o, a), (&mut n, b)] {
                        acc[0] += v.0 / 6.0;
                        acc[1] += v.1 / 6.0;
                        acc[2] += v.2 / 6.0;
                    }
                }
                println!("disk {disk} clip {clip}, hug off / on: bare edge {:.4} / {:.4}, edge L excess {:+.4} / {:+.4}, spill {:.3} / {:.3}", o[0], n[0], o[1], n[1], o[2], n[2]);
            }
        }
    }

    #[test]
    #[cfg(tube_box)]
    fn edges_are_covered_like_the_inside() {
        for disk in [false, true] {
            for clip in [false, true] {
                // (set EDGE_UNHUGGED to also print the `hug(false)` stats)
                let unhugged = std::env::var_os("EDGE_UNHUGGED").map(|_| edge_stats(clip, disk, false, 3));
                let hugged = edge_stats(clip, disk, true, 3);
                println!("disk {disk} clip {clip}: bare edge {:.4}, edge L excess {:+.4}, spill {:.3} (hug off {unhugged:?})", hugged.0, hugged.1, hugged.2);
                assert!(hugged.0 < 0.012, "disk {disk} clip {clip}: {:.4} of the edge band bare", hugged.0);
                assert!(hugged.1 < 0.009, "disk {disk} clip {clip}: edge lighter than the inside by {:.4}", hugged.1);
                // strokes reach no farther past the edge than a brush width
                assert!(hugged.2 < 0.03 || disk && !clip && hugged.2 < 0.2, "disk {disk} clip {clip}: spill {:.3}", hugged.2);
            }
        }
        // a clipped band narrower than the broad brush, strokes along it
        let (bare, _, _) = edge_stats_in(true, false, true, true, 3);
        let unhugged = std::env::var_os("EDGE_UNHUGGED").map(|_| edge_stats_in(true, false, true, false, 3).0);
        println!("thin clipped band: bare edge {bare:.4} (hug off {unhugged:?})");
        assert!(bare < 0.05, "thin band: {bare:.4} of its edges bare");
    }

    /// A long hand-timed pass ages as it goes: its first slices can set (and
    /// bake into the dry film) before the look-and-fill. The look must see
    /// that paint as laid, not as a gap, so the pass lays at most about 1.5×
    /// the fill dabs it does with hand time off.
    #[test]
    fn a_long_timed_pass_fills_only_its_gaps() {
        let run = |hand: Option<f32>, fill: bool| {
            let mut c = Canvas::new(160, 1.4, crate::color::hex("#c8b89a")).with_size_mm(440.0);
            c.set_hand_time(hand);
            let all = Mask::from_fn(c.frame(), |_, _| 1.0);
            // thin, lean paint, one reload a stroke: hours of hand time
            let hd = Handling::new(Tool::filbert(6.0)).color(|_, _| crate::color::hex("#6f84a8")).paint(0.85, 1.0).load(0.3).coverage(3.0).fill(fill);
            c.work(&all, &hd, 3);
            let set = c.wet.clock.px.iter().filter(|p| p.sub > 0.0 && p.sub < 1.0).count() as f32 / c.wet.vol.len() as f32;
            (c.tally().strokes, c.clock(), set)
        };
        let (pass, _, _) = run(None, false);
        let (off, _, _) = run(None, true);
        let (on, clock, set) = run(Some(15.0), true);
        assert!(clock > 120.0 && set > 0.01, "the pass took hours ({clock} min) and its start set ({set})");
        let (off, on) = (off - pass, on - pass);
        assert!(on <= off + off / 2 + 10, "fill dabs: {on} with hand time, {off} without");
    }

    #[test]
    #[should_panic(expected = "coverage")]
    fn negative_coverage_is_rejected() {
        let _ = Handling::new(Tool::filbert(22.0)).coverage(-1.0);
    }

    #[test]
    #[should_panic(expected = "coverage")]
    fn nonfinite_coverage_is_rejected() {
        let _ = Handling::new(Tool::filbert(22.0)).coverage(f32::NAN);
    }
}

/// Maybe lift the brush partway and put it down again a little off the line.
fn break_stroke(hd: &Handling, pts: Vec<(f32, f32)>, rng: &mut Rng) -> Vec<Vec<(f32, f32)>> {
    let n = pts.len();
    if n < 5 || hd.broken <= 0.0 || !rng.chance(hd.broken) {
        return vec![pts];
    }
    let cut = ((n as f32 * rng.range(0.3, 0.7)) as usize).clamp(2, n - 3);
    let (dx, dy) = (pts[cut + 1].0 - pts[cut].0, pts[cut + 1].1 - pts[cut].1);
    let d = (dx * dx + dy * dy).sqrt().max(1e-6);
    let (tx, ty) = (dx / d, dy / d);
    let w = hd.tool.width;
    // off the line by a fraction of the brush, a small gap or overlap, and
    // turned a little around the restart
    let off = rng.normal() * 0.3 * w;
    let shift = rng.range(-0.8, 0.6) * w;
    let turn = rng.normal() * 0.06;
    let (ct, st) = (turn.cos(), turn.sin());
    let o = pts[cut];
    let second = pts[cut..]
        .iter()
        .map(|&(x, y)| {
            let (rx, ry) = (x - o.0, y - o.1);
            let (rx, ry) = (rx * ct - ry * st, rx * st + ry * ct);
            (o.0 + rx - ty * off + tx * shift, o.1 + ry + tx * off + ty * shift)
        })
        .collect();
    let first = pts[..=cut].to_vec();
    vec![first, second]
}

/// Where a stroke seeded outside the region comes within it: the point of
/// its path (on the canvas, mask at or above `threshold`) nearest `c`.
fn entry(mask: &Mask, f: Frame, pts: &[(f32, f32)], c: (f32, f32), threshold: f32) -> Option<(f32, f32)> {
    let mut best: Option<((f32, f32), f32)> = None;
    let mut consider = |p: (f32, f32)| {
        if p.0 < 0.0 || p.1 < 0.0 || p.0 >= f.width() || p.1 >= f.height() || mask_at(mask, p.0, p.1) < threshold {
            return;
        }
        let d = (p.0 - c.0).powi(2) + (p.1 - c.1).powi(2);
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some((p, d));
        }
    };
    for (k, &p) in pts.iter().enumerate() {
        consider(p);
        if let Some(&q) = pts.get(k + 1) {
            for t in [0.25, 0.5, 0.75] {
                consider((p.0 + (q.0 - p.0) * t, p.1 + (q.1 - p.1) * t));
            }
        }
    }
    best.map(|(p, _)| p)
}

/// The order the tiles of a passage are painted in. `Sweep` goes band by band
/// in its direction (alternate tiles within a band, so they can run side by
/// side); otherwise the four checkerboard phases in random order.
fn tile_order(order: Order, (tw, th): (usize, usize), (tile_x, tile_y): (f32, f32), rng: &mut Rng) -> Vec<usize> {
    let parity = |t: usize| (t % tw) % 2 + 2 * ((t / tw) % 2);
    match order {
        Order::Sweep(a) => {
            let (ca, sa) = (a.cos(), a.sin());
            let bw = tile_x.min(tile_y);
            let band = |t: usize| {
                let (x, y) = (((t % tw) as f32 + 0.5) * tile_x, ((t / tw) as f32 + 0.5) * tile_y);
                ((x * ca + y * sa) / bw).round() as i64
            };
            let mut idx: Vec<usize> = (0..tw * th).collect();
            idx.sort_by_key(|&t| (band(t), parity(t), t));
            idx
        }
        _ => {
            let mut phases = [0usize, 1, 2, 3];
            for i in (1..4).rev() {
                let j = (rng.next_u64() % (i as u64 + 1)) as usize;
                phases.swap(i, j);
            }
            phases.iter().flat_map(|&p| (0..tw * th).filter(move |&t| parity(t) == p)).collect()
        }
    }
}

