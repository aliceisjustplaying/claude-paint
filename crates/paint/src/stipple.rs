//! Stippling: covering an area with many small touches of a brush tip.
//!
//! A stipple pass is many separate touches of a tip held upright: each
//! touch presses the tip into the canvas and lifts it, with no path between
//! touches. Touches fall on a jittered grid thinned by `coverage` (the mean
//! number of touches per point), optionally in clumps, and the tip is
//! reloaded from the palette every `dip_every` touches. The tone of the
//! passage is set by how densely the touches fall and by the paint's color,
//! hiding and thickness.
//!
//! Every touch is a `Touch` of a simulated `Held` brush (see `bristle`): it
//! deposits and lifts wet paint, so touches into a wet layer pick some of it
//! up and mix with it, and it dries with the rest of the wet layer
//! (Kubelka–Munk).
//!
//! ```ignore
//! // a region `m`, coverage falling from 1.2 at y = 0 to 0.2 at y = 800
//! let sp = Stipple::new(Tool::stippler(3.0)).piled(&pal, pile, 0.5)
//!     .coverage(|_, y| 1.2 - y / 800.0).pressure(0.4, 0.8);
//! c.stipple(&m, &sp, 1);
//! c.dry();
//! ```

use crate::bristle::{Bounds, Held, Rect, Tool, Touch, touch_footprint, touch_on};
use crate::canvas::Canvas;
use crate::color::{Rgb, from_oklab, to_oklab};
use crate::mask::Mask;
use crate::palette::Palette;
use crate::rng::Rng;
use crate::wet::Paint;
use rayon::prelude::*;

type Field<'a, T> = Box<dyn Fn(f32, f32) -> T + Sync + 'a>;

/// The parameters of a stipple pass over a region (see the module docs).
pub struct Stipple<'a> {
    pub dip_with: Option<crate::handling::Dip<'a>>,
    /// The brush (default `Tool::stippler(2.0)`): its width sets the mark size.
    pub tool: Tool,
    /// Peak pressure range; a random value in it per touch. Harder touches
    /// make bigger, fuller marks, so this is the mark-size variation.
    pub pressure: (f32, f32),
    /// Coverage: how many touches fall on each point on average (a touch
    /// covers about π(0.4·width)²). Where it is higher the stipple is denser
    /// and its color stronger.
    pub coverage: Field<'a, f32>,
    /// Raw paint color at a point, without matching the canvas.
    pub color: Field<'a, Rgb>,
    /// Hiding and stiffness of the paint without a palette.
    pub hiding: f32,
    pub stiff: f32,
    /// Raw paint color variation per dip: OKLab L and a/b sd.
    pub jitter: (f32, f32),
    /// Relative sd of the pile's tube proportions per dip.
    pub mix_jitter: f32,
    /// Dip into this pile (parts of the palette's tubes, thinned with this
    /// medium) on every trip; the raw color field is then not used.
    pub pile: Option<(&'a Palette, crate::palette::Mixture, f32)>,
    /// Share of solvent knifed into that pile (`crate::thinner`; engine 3).
    pub thinner: f32,
    /// Touches between trips to the palette, how much of a full load a dip
    /// takes and how much old paint is wiped off first. A stippler's tip
    /// carries paint for many touches; each lays a little less.
    pub dip_every: usize,
    pub load: f32,
    pub wipe: f32,
    /// How far the tip drifts while it is down (units, mean); 0 = a straight
    /// down-and-up touch.
    pub drag: f32,
    /// Direction of the drift (radians); None = a random direction per
    /// touch.
    pub drag_angle: Option<f32>,
    /// Roll of the handle while down (radians, sd).
    pub twist: f32,
    /// 0 = touches spread evenly, 1 = in clumps.
    pub cluster: f32,
    /// Size of the clumps (units); default 5 marks.
    pub clump: Option<f32>,
    /// Where the coverage `c` is below 1, each touch's pressure is scaled by
    /// `1 - feather·(1 - c)`: smaller, fainter marks where the touches are
    /// sparse. 0 = off, 1 = pressure ∝ coverage; default 0.6.
    pub feather: f32,
    /// Clip the hairs' contact to the mask.
    pub clip: bool,
    /// A hard limit no touch paints outside of, whatever `clip` says.
    pub limit: Option<std::sync::Arc<Mask>>,
}

impl<'a> Stipple<'a> {
    pub fn new(tool: Tool) -> Self {
        Stipple {
            dip_with: None,            tool,
            pressure: (0.4, 0.75),
            coverage: Box::new(|_, _| 1.0),
            color: Box::new(|_, _| [0.5; 3]),
            hiding: 0.4,
            stiff: 0.3,
            jitter: (0.015, 0.004),
            mix_jitter: 0.05,
            pile: None,
            thinner: 0.0,
            dip_every: 24,
            load: 0.5,
            wipe: 0.5,
            drag: 0.0,
            drag_angle: None,
            twist: 0.3,
            cluster: 0.15,
            clump: None,
            feather: 0.6,
            clip: false,
            limit: None,
        }
    }
    pub fn pressure(mut self, a: f32, b: f32) -> Self {
        self.pressure = (a, b);
        self
    }
    pub fn coverage(mut self, f: impl Fn(f32, f32) -> f32 + Sync + 'a) -> Self {
        self.coverage = Box::new(f);
        self
    }
    pub fn color(mut self, f: impl Fn(f32, f32) -> Rgb + Sync + 'a) -> Self {
        self.color = Box::new(f);
        self
    }
    /// Load every trip from `pile` (parts of `palette`'s tubes), thinned
    /// with `medium` (0..1; below 0, down to -0.5, that share of its oil
    /// blotted out), remixed a little per dip by `mix_jitter`,
    /// drying at its tubes' rate.
    pub fn piled(mut self, palette: &'a Palette, pile: crate::palette::Mixture, medium: f32) -> Self {
        self.pile = Some((palette, pile, medium.clamp(-0.5, 1.0)));
        self
    }
    /// The pile holds the share `t` of solvent (`crate::thinner`; engine 3).
    pub fn thinner(mut self, t: f32) -> Self {
        self.thinner = t;
        self
    }
    /// A fixed paint (no palette mixing).
    pub fn paint(mut self, hiding: f32, stiff: f32) -> Self {
        self.hiding = hiding;
        self.stiff = stiff;
        self
    }
    pub fn jitter(mut self, l: f32, hue: f32) -> Self {
        self.jitter = (l, hue);
        self
    }
    pub fn mix_jitter(mut self, sd: f32) -> Self {
        self.mix_jitter = sd;
        self
    }
    pub fn dips(mut self, every: usize, load: f32, wipe: f32) -> Self {
        self.dip_every = every.max(1);
        self.load = load;
        self.wipe = wipe;
        self
    }
    /// Tip drift while down: mean length (units) and direction (None = any).
    pub fn drag(mut self, len: f32, angle: Option<f32>) -> Self {
        self.drag = len;
        self.drag_angle = angle;
        self
    }
    pub fn twist(mut self, sd: f32) -> Self {
        self.twist = sd;
        self
    }
    /// Clumping 0..1 and the clump size in units (None = 5 marks).
    pub fn cluster(mut self, amount: f32, size: Option<f32>) -> Self {
        self.cluster = amount;
        self.clump = size;
        self
    }
    pub fn feather(mut self, k: f32) -> Self {
        self.feather = k;
        self
    }
    pub fn clip(mut self, on: bool) -> Self {
        self.clip = on;
        self
    }
    /// Never paint outside `m` (see `limit`).
    pub fn limit(mut self, m: std::sync::Arc<Mask>) -> Self {
        self.limit = Some(m);
        self
    }
    /// Area (units²) one touch covers at mid pressure.
    fn mark_area(&self) -> f32 {
        let r = 0.4 * self.tool.width;
        std::f32::consts::PI * r * r
    }

    /// Raw paint for one dip, with color variation but no canvas compensation.
    fn paint_for(&self, color: Rgb, rng: &mut Rng) -> Paint {
        let lab = to_oklab(color);
        let col = from_oklab([lab[0] + rng.normal() * self.jitter.0, lab[1] + rng.normal() * self.jitter.1, lab[2] + rng.normal() * self.jitter.1]);
        Paint::new(col, self.hiding, self.stiff)
    }
}

/// One planned touch.
struct Plan {
    dip_seed: u64,
    touch: Touch,
    /// Paint to dip into first (None = keep going with what's on the brush).
    dip: Option<Paint>,
    /// Center of the touches served by this dip, for sampling raw color.
    color_at: (f32, f32),
    rect: Rect,
    /// The raw color or explicit pile color:
    /// which pile on the palette the dip comes from (`tally::Piles`).
    want: Rgb,
}

impl Canvas {
    /// Stipple the region `mask` (its value scales the coverage) touch by
    /// touch with a simulated brush (see `Stipple`).
    ///
    /// Touches are planned up front on a jittered grid thinned by the
    /// coverage, then grouped into square passages larger than twice any
    /// touch's reach. Passages are ordered by 2×2 checkerboard phase and
    /// painted with `run_ordered` (see `sched`), each with its own brush:
    /// passages whose footprints don't overlap run in parallel.
    pub fn stipple(&mut self, mask: &Mask, sp: &Stipple, seed: u64) {
        self.stipple_with(&mut crate::tally::Piles::default(), mask, sp, seed);
    }

    /// `stipple`, dipping into the piles already on the palette (see
    /// `work_with`).
    ///
    /// Panics if `sp` is thinned (`Stipple::thinner`, which applies to a
    /// pile, `Stipple::piled`) and the canvas's engine is before 3.
    pub fn stipple_with(&mut self, piles: &mut crate::tally::Piles, mask: &Mask, sp: &Stipple, seed: u64) {
        sp.tool.assert_valid();
        self.assert_thinner_supported(sp.thinner > 0.0 && sp.pile.is_some(), "Canvas::stipple");
        self.check_mask(mask);
        // plan on the whole canvas (a crop render plans the same touches)
        let f = mask.f;
        let t0 = std::time::Instant::now();
        // bounds of the region, in units
        let (mut bx0, mut by0, mut bx1, mut by1) = (usize::MAX, usize::MAX, 0usize, 0usize);
        for y in 0..f.h {
            let row = &mask.data[y * f.w..(y + 1) * f.w];
            if let (Some(a), Some(b)) = (row.iter().position(|&v| v > 0.0), row.iter().rposition(|&v| v > 0.0)) {
                bx0 = bx0.min(a);
                bx1 = bx1.max(b + 1);
                by0 = by0.min(y);
                by1 = by1.max(y + 1);
            }
        }
        if bx1 <= bx0 {
            return;
        }
        let inv = 1.0 / f.scale;
        let (ux0, uy0, ux1, uy1) = (bx0 as f32 * inv, by0 as f32 * inv, bx1 as f32 * inv, by1 as f32 * inv);
        let cov = |x: f32, y: f32| ((sp.coverage)(x, y) * mask.data[f.index(x, y)]).max(0.0);
        // the densest coverage asked for sets the grid. Sample it at mask
        // pixels no more than a unit (or half a tool width) apart, so a
        // small region or a narrow band of coverage can't fall between the
        // samples; if they still find nothing, look at every mask pixel
        // before calling the region empty.
        let peak = |step: usize| {
            (by0..by1)
                .into_par_iter()
                .filter(|y| (y - by0) % step == 0)
                .map(|y| {
                    let row = &mask.data[y * f.w..(y + 1) * f.w];
                    let mut m = 0.0f32;
                    for x in (bx0..bx1).step_by(step) {
                        if row[x] > 0.0 {
                            m = m.max(cov((x as f32 + 0.5) * inv, (y as f32 + 0.5) * inv));
                        }
                    }
                    m
                })
                .reduce(|| 0.0, f32::max)
        };
        let step = ((sp.tool.width * 0.5).min(1.0) * f.scale).floor().max(1.0) as usize;
        let mut dmax = peak(step);
        if dmax <= 1e-4 && step > 1 {
            dmax = peak(1);
        }
        let dmax = dmax * 1.15;
        if dmax <= 1e-4 {
            return;
        }
        let area = sp.mark_area();
        // one candidate per cell; at the densest, each one is kept
        let g = (area / dmax).sqrt().max(0.3 * inv);
        let clump = sp.clump.unwrap_or(sp.tool.width * 4.0).max(inv);
        let cl = crate::noise::Fbm::new((seed as u32).wrapping_mul(2654435761) ^ 0x5717, 3, clump);
        let (cols, rows) = (((ux1 - ux0) / g).ceil() as usize + 1, ((uy1 - uy0) / g).ceil() as usize + 1);
        let pts: Vec<(f32, f32)> = (0..rows)
            .into_par_iter()
            .flat_map_iter(|j| {
                let mut rng = Rng::new(seed ^ (j as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x57_1BB1E);
                let cov = &cov;
                let cl = &cl;
                (0..cols).filter_map(move |i| {
                    let x = ux0 + (i as f32 + rng.f()) * g;
                    let y = uy0 + (j as f32 + rng.f()) * g;
                    let keep = rng.f();
                    if x >= ux1 || y >= uy1 {
                        return None;
                    }
                    let c = cov(x, y);
                    if c <= 0.0 {
                        return None;
                    }
                    // clumps: a noise field raises the keep rate in places and lowers it in others
                    let k = ((1.0 - sp.cluster) + sp.cluster * 2.0 * cl.get01(x, y).powf(1.5) * 1.6).max(0.0);
                    (keep < c / dmax * k).then_some((x, y))
                })
            })
            .collect();
        if pts.is_empty() {
            return;
        }

        // the reach of the biggest touch sets the passage size
        let big = Touch::at(f.width() * 0.5, f.height() * 0.5).pressure(sp.pressure.1.max(sp.pressure.0)).drag(sp.drag * 2.0, sp.drag * 2.0);
        let r = touch_footprint(&sp.tool, &big, f.scale, usize::MAX / 4, usize::MAX / 4).expect("touch footprint");
        let (cxp, cyp) = (big.at.0 * f.scale, big.at.1 * f.scale);
        let reach = ((cxp - r.0 as f32).max(r.2 as f32 - cxp).max(cyp - r.1 as f32).max(r.3 as f32 - cyp)) * inv;
        let tile = (2.0 * reach + 4.0 * inv).max(sp.tool.width * 12.0).max(16.0);
        let (tw, th) = ((f.width() / tile).ceil().max(1.0) as usize, (f.height() / tile).ceil().max(1.0) as usize);
        let mut tiles: Vec<Vec<(f32, f32)>> = vec![Vec::new(); tw * th];
        for &(x, y) in &pts {
            let (tx, ty) = (((x / tile) as usize).min(tw - 1), ((y / tile) as usize).min(th - 1));
            tiles[ty * tw + tx].push((x, y));
        }
        // plan each passage: the order of the touches, each touch's
        // pressure, drift and roll, and the trips to the palette
        let plans: Vec<Vec<Plan>> = tiles
            .par_iter()
            .enumerate()
            .map(|(ti, pts)| {
                let mut rng = Rng::new(seed ^ 0x7111E ^ (ti as u64).wrapping_mul(0xD1B5_4A32_D192_ED03));
                // touches are ordered in small patches, row by row,
                // alternating direction, in random order within a patch; a
                // load serves about one patch, so each dip's paint lands
                // near the spot where its color was sampled
                let cell = (g * (sp.dip_every as f32).sqrt()).max(sp.tool.width * 2.0);
                let (tx0, ty0) = ((ti % tw) as f32 * tile, (ti / tw) as f32 * tile);
                let mut keyed: Vec<(u64, (f32, f32))> = pts
                    .iter()
                    .map(|&(x, y)| {
                        let row = ((y - ty0) / cell).max(0.0) as u64;
                        let col = ((x - tx0) / cell).max(0.0) as u64;
                        let col = if row.is_multiple_of(2) { col } else { 1_000 - col.min(1_000) };
                        ((row << 40) | (col << 24) | (rng.next_u64() & 0xFF_FFFF), (x, y))
                    })
                    .collect();
                keyed.sort_by_key(|k| k.0);
                let pts: Vec<(f32, f32)> = keyed.into_iter().map(|k| k.1).collect();
                pts.iter()
                    .enumerate()
                    .map(|(k, &(x, y))| {
                        let cv = cov(x, y);
                        // the paint is chosen below, in order
                        let dip = (k % sp.dip_every == 0).then_some(Paint::km([0.0; 3], 0.0, 0.0));
                        let color_at = if dip.is_some() {
                            let grp = &pts[k..(k + sp.dip_every).min(pts.len())];
                            let (sx, sy) = grp.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
                            let (mx, my) = (sx / grp.len() as f32, sy / grp.len() as f32);
                            (mx, my)
                        } else {
                            (x, y)
                        };
                        let a = sp.drag_angle.map_or(rng.range(0.0, std::f32::consts::TAU), |a| a + rng.normal() * 0.2);
                        let len = sp.drag * rng.range(0.4, 1.6);
                        let touch = Touch {
                            at: (x, y),
                            pressure: rng.range(sp.pressure.0, sp.pressure.1) * (1.0 - sp.feather * (1.0 - cv.min(1.0))),
                            drag: (a.cos() * len, a.sin() * len),
                            twist: rng.normal() * sp.twist,
                            angle: rng.range(0.0, std::f32::consts::TAU),
                        };
                        let rect = touch_footprint(&sp.tool, &touch, f.scale, f.w, f.h).unwrap_or((0, 0, 0, 0));
                        Plan { dip_seed: 0, touch, dip, color_at, rect, want: [0.0; 3] }
                    })
                    .collect()
            })
            .collect();
        // Trips to the palette in passage order keep per-dip variation stable.
        let mut plans = plans;
        let t_geom = t0.elapsed().as_secs_f32();
        let mut prng = Rng::new(seed ^ 0xD1B);
        for t in plans.iter_mut() {
            for p in t.iter_mut() {
                if let Some(d) = p.dip.as_mut() {
                    if let Some((pal, pile, medium)) = &sp.pile {
                        p.want = pile.color;
                        p.dip_seed = prng.next_u64();
                        *d = pal.remix(pile, sp.mix_jitter, &mut Rng::new(p.dip_seed)).laid(*medium);
                        if sp.thinner > 0.0 {
                            *d = d.clone().with_thinner(sp.thinner);
                        }
                        continue;
                    }
                    let (x, y) = p.color_at;
                    p.want = (sp.color)(x, y);
                    *d = sp.paint_for(p.want, &mut Rng::new(prng.next_u64()));
                }
            }
        }
        // the hand's ledger: every planned touch and trip to the palette (on
        // the whole canvas, before a crop drops passages; see `tally`)
        let mut tile_secs = Vec::with_capacity(plans.len());
        for t in plans.iter() {
            let secs0 = self.tally.secs;
            for p in t {
                self.tally.touch();
                if p.dip.is_some() {
                    piles.trip(&mut self.tally, p.want);
                }
            }
            tile_secs.push(self.tally.secs - secs0);
        }
        let n: usize = plans.iter().map(|t| t.len()).sum();
        let t_plan = t0.elapsed().as_secs_f32();

        let limited;
        let clip = match (&sp.limit, sp.clip) {
            (Some(l), true) => {
                limited = mask.clone().mul(l);
                Some(&limited)
            }
            (Some(l), false) => Some(&**l),
            (None, true) => Some(mask),
            (None, false) => None,
        };
        let mut rng = Rng::new(seed ^ 0xFA5E);
        let mut phases = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)];
        for i in (1..4).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            phases.swap(i, j);
        }
        // passages in phase order; each passage's footprint is the union of
        // its touches'. run_ordered keeps overlapping passages in this order
        let rects: Vec<Option<Rect>> = plans
            .iter()
            .map(|t| t.iter().filter(|p| p.rect.2 > p.rect.0).map(|p| p.rect).reduce(|a, r| (a.0.min(r.0), a.1.min(r.1), a.2.max(r.2), a.3.max(r.3))))
            .collect();
        let order: Vec<usize> = phases.iter().flat_map(|&(px, py)| (0..plans.len()).filter(move |&i| (i % tw) % 2 == px && (i / tw) % 2 == py)).collect();
        let ids = plans.iter().map(|t| t.len() as u32).collect();
        let slice = self.covering_slice_secs(self.hand_slice_secs(), plans.iter().flatten().filter_map(|p| p.dip.clone()), &tile_secs);
        let pass = crate::sched::Pass { grid: (tw, th), order: crate::sched::TileOrder::Default(order), rects, secs: tile_secs, ids, slice };
        self.paint_pass(pass, sp.dip_with.is_some(), |surf, ti, first_id| {
            let mut held = Held::new(sp.tool.clone(), seed ^ 0x5717 ^ (ti as u64).wrapping_mul(0x9E37_79B9));
            let mut scratch = Vec::new();
            let mut b: Bounds = None;
            for (k, p) in plans[ti].iter().enumerate() {
                if let Some(paint) = p.dip.clone() {
                    held.wipe(sp.wipe);
                    let paint = sp.dip_with.as_ref().map_or(paint, |dip| dip(&held, 0, p.dip_seed));
                    held.load(paint, sp.load);
                }
                if p.rect.2 <= p.rect.0 {
                    continue;
                }
                let id = first_id.wrapping_add(k as u32);
                // SAFETY: every pixel a touch reaches lies in its footprint,
                // inside its passage's rect; run_ordered never runs passages
                // with overlapping rects at once; `surf()` checked the buffers.
                let r = unsafe { touch_on(surf, &mut held, &p.touch, clip.map(crate::bristle::Clip::Mask), id, &mut scratch) };
                b = crate::sched::union(b, r);
            }
            b
        });
        if std::env::var_os("PAINT_DEBUG").is_some() {
            eprintln!("stipple: {n} touches, reach {reach:.1}, tiles {tw}x{th} ({tile:.0} units), plan {t_geom:.2}s + paint {:.2}s, total {:.2}s", t_plan - t_geom, t0.elapsed().as_secs_f32());
        }
    }
}
