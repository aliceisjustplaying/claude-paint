//! Stippling: covering an area with many small touches of a brush tip.
//!
//! Friedrich stippled his skies, mist and distant hills, which "enhance[s]
//! the transparency and light scattering" [NG p.56]; his smooth gradations
//! come from stippling and from thin paint pooling in the ground texture,
//! not from thick blending [CATS p.127] (notes/research/friedrich_materials.md).
//!
//! A painter stippling holds a small soft round upright and touches the
//! canvas again and again with its tip, in a thin, lean paint, moving
//! around a passage and going back to the palette every so often. Tone is
//! built by how densely the touches fall and by their color; the marks have
//! no direction, so the brushwork disappears into a fine, even grain.
//! Passes are layered: a coarser, darker pass, then a finer, lighter one.
//!
//! Every touch is a `Touch` of a simulated `Held` brush (see `bristle`): it
//! deposits and lifts wet paint, so stippling into a wet lay-in fuses softly,
//! and it dries with the rest of the wet layer (Kubelka–Munk).
//!
//! ```ignore
//! // a thin sky already laid in; stipple it twice, coarse then fine
//! let coarse = Stipple::new(Tool::stippler(3.0)).mixed(&pal, 0.5)
//!     .color(|x, y| sky(x, y)).coverage(|_, y| 1.2 - y / 800.0).pressure(0.4, 0.8);
//! c.stipple(&sky_mask, &coarse, 1);
//! let fine = Stipple::new(Tool::stippler(1.6)).mixed(&pal, 0.55)
//!     .color(|x, y| lighter(sky(x, y))).coverage(|_, _| 1.0);
//! c.stipple(&sky_mask, &fine, 2);
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

/// How a painter stipples a region (see the module docs).
pub struct Stipple<'a> {
    /// The brush (default `Tool::stippler(2.0)`): its width sets the mark size.
    pub tool: Tool,
    /// Peak pressure range; a random value in it per touch. Harder touches
    /// make bigger, fuller marks, so this is the mark-size variation.
    pub pressure: (f32, f32),
    /// Coverage: how many touches fall on each point on average (a touch
    /// covers about π(0.4·width)²). Where it is higher the stipple is denser
    /// and its color stronger; this is how a painter grades tone.
    pub coverage: Field<'a, f32>,
    /// What the painter wants to see at a point, on the canvas (the paint is
    /// mixed so that a touch there dries to this; see `paint_for`).
    pub color: Field<'a, Rgb>,
    /// A color relative to what is on the canvas (see `color_over`); when
    /// set, it replaces `color`.
    pub color_over: Option<crate::handling::OverField<'a>>,
    /// Mix each pile from these tubes, thinned with this fraction of medium.
    pub palette: Option<(&'a Palette, f32)>,
    /// Hiding and stiffness of the paint without a palette.
    pub hiding: f32,
    pub stiff: f32,
    /// Palette-mixing inconsistency per dip: OKLab L and a/b sd (no palette).
    pub jitter: (f32, f32),
    /// Relative sd of the tube proportions per dip (palette).
    pub mix_jitter: f32,
    /// Touches between trips to the palette, how much of a full load a dip
    /// takes and how much old paint is wiped off first. A stippler's tip
    /// carries paint for many touches; each lays a little less.
    pub dip_every: usize,
    pub load: f32,
    pub wipe: f32,
    /// How far the hand drifts while the tip is down (units, mean); 0 = a
    /// clean vertical touch.
    pub drag: f32,
    /// Direction of the drift (radians); None = any direction (no stroke
    /// direction survives).
    pub drag_angle: Option<f32>,
    /// Roll of the handle while down (radians, sd).
    pub twist: f32,
    /// 0 = touches spread evenly (a practiced hand), 1 = in clumps.
    pub cluster: f32,
    /// Size of the clumps (units); default 5 marks.
    pub clump: Option<f32>,
    /// Where the coverage is thin (< 1), the hand also lightens: touches
    /// press less (smaller, fainter marks), so a veil feathers out instead
    /// of ending in isolated specks. 0 = off, 1 = pressure ∝ coverage.
    pub feather: f32,
    /// Contrast falls with density: where the coverage is thin (< 1) each
    /// touch is aimed that much nearer to what it sits on, `min(1, c)^fade`
    /// of the way from the underlayer to `color`. A lighter stipple thinning
    /// out over a field then fades into it instead of ending in salt. 0 = off
    /// (every touch aims at `color`, for deliberate specks); default 1.
    pub fade: f32,
    /// Clip the hairs' contact to the mask.
    pub clip: bool,
    /// A hard limit no touch paints outside of, whatever `clip` says.
    pub limit: Option<std::sync::Arc<Mask>>,
    /// Aim the paint at the result on the canvas (default). Off: the paint
    /// is simply mixed to `color`.
    pub aim: bool,
}

impl<'a> Stipple<'a> {
    pub fn new(tool: Tool) -> Self {
        Stipple {
            tool,
            pressure: (0.4, 0.75),
            coverage: Box::new(|_, _| 1.0),
            color: Box::new(|_, _| [0.5; 3]),
            color_over: None,
            palette: None,
            hiding: 0.4,
            stiff: 0.3,
            jitter: (0.015, 0.004),
            mix_jitter: 0.05,
            dip_every: 24,
            load: 0.5,
            wipe: 0.5,
            drag: 0.0,
            drag_angle: None,
            twist: 0.3,
            cluster: 0.15,
            clump: None,
            feather: 0.6,
            fade: 1.0,
            clip: false,
            limit: None,
            aim: true,
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
        self.color_over = None;
        self
    }
    /// A color that sees the canvas: `f(x, y, under)` gets what is on the
    /// canvas where a load of touches will land (judged before the pass, see
    /// `Canvas::judge_under`) and returns the look wanted there, e.g. a cast
    /// shadow on snow as "the snow here, darker and bluer":
    /// `color_over(|_, _, u| shift(u, -0.06, 0.0, -0.03))`.
    pub fn color_over(mut self, f: impl Fn(f32, f32, Rgb) -> Rgb + Sync + 'a) -> Self {
        self.color_over = Some(Box::new(f));
        self
    }
    /// Mix every pile from `palette`'s tubes, thinned with `medium` (0..1).
    pub fn mixed(mut self, palette: &'a Palette, medium: f32) -> Self {
        self.palette = Some((palette, medium));
        self
    }
    /// A fixed paint (no palette mixing).
    pub fn paint(mut self, hiding: f32, stiff: f32) -> Self {
        self.hiding = hiding;
        self.stiff = stiff;
        self.palette = None;
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
    /// Hand drift while down: mean length (units) and direction (None = any).
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
    /// Contrast falls where the coverage thins (see `fade`; 0 = off).
    pub fn fade(mut self, k: f32) -> Self {
        self.fade = k.max(0.0);
        self
    }
    /// The look one load of touches aims at, where the canvas looks `seen`,
    /// the passage should look `want` and the coverage is `c`.
    fn touch_target(&self, want: Rgb, seen: Rgb, c: f32) -> Rgb {
        let t = if self.fade > 0.0 { c.clamp(0.0, 1.0).powf(self.fade) } else { 1.0 };
        if t >= 1.0 {
            return want;
        }
        crate::color::from_oklab(crate::color::lerp3(to_oklab(seen), to_oklab(want), t))
    }
    pub fn aim(mut self, on: bool) -> Self {
        self.aim = on;
        self
    }

    /// Area (units²) one touch covers at mid pressure.
    fn mark_area(&self) -> f32 {
        let r = 0.4 * self.tool.width;
        std::f32::consts::PI * r * r
    }

    /// Thickness (coats) a touch lays on average: the tip's film at the
    /// load, run down a little over the touches of a dip, less where a
    /// light touch only catches the tooth.
    fn touch_coats(&self) -> f32 {
        self.tool.lay * self.load.min(1.0) * 0.8
    }

    /// The paint for one trip to the palette: what the painter mixes so that
    /// the stippled passage around a point where the canvas looks like
    /// `seen` dries to `want`. The painter judges the passage, not one dot:
    /// with `coverage` touches falling on each point, the film there is
    /// about `coverage` touches thick (at least one).
    ///
    /// INTEGRATION POINT: this is the only place stippling chooses paint.
    /// When `Palette` gains its substrate-aware "aim at the result on the
    /// canvas" API (color stream), call that here (with `coats`) instead of
    /// the `aim_km` workaround.
    fn paint_for(&self, want: Rgb, seen: Rgb, coverage: f32, memo: &mut Memo, rng: &mut Rng) -> Paint {
        let coats = self.touch_coats() * coverage.max(1.0);
        let aim = self.aim;
        match self.palette {
            Some((pal, medium0)) => {
                if !aim {
                    return pal.remix(&pal.mix(want), self.mix_jitter, rng).paint(medium0);
                }
                // aim at the look over what's there (Palette::aim); if the
                // thinned paint can't get there over this substrate, the
                // painter thins it less (more body, more hiding). The painter
                // remembers a recipe for a tone over a tone.
                let q = |c: Rgb, k: f32| {
                    let l = to_oklab(c);
                    [(l[0] * k).round() as i32, (l[1] * k).round() as i32, (l[2] * k).round() as i32]
                };
                let key = (q(want, 200.0), q(seen, 120.0), (coats * 20.0).round() as i32);
                let (m, medium) = if let Some(v) = memo.get(&key) {
                    v.clone()
                } else {
                    // the recipe for the key's center, so it doesn't depend on
                    // which touch asked first (a crop or a reordered run must
                    // mix what a whole one does)
                    let c = |k: [i32; 3], s: f32| crate::color::from_oklab([k[0] as f32 / s, k[1] as f32 / s, k[2] as f32 / s]);
                    let (want, seen, coats) = (c(key.0, 200.0), c(key.1, 120.0), key.2 as f32 / 20.0);
                    let wl = to_oklab(want);
                    let mut best: Option<(f32, crate::palette::Mixture, f32)> = None;
                    for medium in [medium0, medium0 * 0.5, 0.0] {
                        let m = pal.aim(want, seen, medium, coats);
                        let got = to_oklab(m.paint(medium).over(seen, coats));
                        let err = ((got[0] - wl[0]).powi(2) + (got[1] - wl[1]).powi(2) + (got[2] - wl[2]).powi(2)).sqrt();
                        if best.as_ref().is_none_or(|b| err < b.0 - 0.005) {
                            best = Some((err, m, medium));
                        }
                        if err < 0.02 {
                            break;
                        }
                    }
                    let (_, m, medium) = best.unwrap();
                    memo.insert(key, (m.clone(), medium));
                    (m, medium)
                };
                pal.remix(&m, self.mix_jitter, rng).paint(medium)
            }
            None => {
                let lab = to_oklab(want);
                let col = from_oklab([lab[0] + rng.normal() * self.jitter.0, lab[1] + rng.normal() * self.jitter.1, lab[2] + rng.normal() * self.jitter.1]);
                if aim { Paint::aimed(col, seen, coats, self.hiding, self.stiff) } else { Paint::new(col, self.hiding, self.stiff) }
            }
        }
    }
}

type Memo = std::collections::HashMap<([i32; 3], [i32; 3], i32), (crate::palette::Mixture, f32)>;

/// One planned touch.
struct Plan {
    touch: Touch,
    /// Paint to dip into first (None = keep going with what's on the brush).
    dip: Option<Paint>,
    /// Where the touches this dip serves are centered, and their coverage:
    /// the painter mixes for that spot.
    aim_at: (f32, f32, f32),
    rect: Rect,
    /// The color asked for there (before aiming it over what's there):
    /// which pile on the palette the dip comes from (`tally::Piles`).
    want: Rgb,
}

impl Canvas {
    /// Stipple the region `mask` (its value scales the coverage) touch by
    /// touch with a simulated brush (see `Stipple`).
    ///
    /// Touches are planned up front on a jittered grid thinned by the
    /// coverage, then grouped into square passages larger than twice any
    /// touch's reach, and passages in a 2×2 checkerboard phase are painted in
    /// parallel, each with its own brush.
    pub fn stipple(&mut self, mask: &Mask, sp: &Stipple, seed: u64) {
        sp.tool.assert_valid();
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
                    // clumps: the hand lingers here and hurries there
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
        // plan each passage: the order the hand visits the touches, each
        // touch's pressure, drift and roll, and the trips to the palette
        let plans: Vec<Vec<Plan>> = tiles
            .par_iter()
            .enumerate()
            .map(|(ti, pts)| {
                let mut rng = Rng::new(seed ^ 0x7111E ^ (ti as u64).wrapping_mul(0xD1B5_4A32_D192_ED03));
                // the hand works through the passage in small patches, row
                // by row, back and forth, dabbing about at random within a
                // patch; a load serves about one patch, so each pile of paint
                // lands where it was mixed for
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
                        let aim_at = if dip.is_some() {
                            let grp = &pts[k..(k + sp.dip_every).min(pts.len())];
                            let (sx, sy) = grp.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
                            let (mx, my) = (sx / grp.len() as f32, sy / grp.len() as f32);
                            (mx, my, cov(mx, my).max(cv * 0.5))
                        } else {
                            (x, y, cv)
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
                        Plan { touch, dip, aim_at, rect, want: [0.0; 3] }
                    })
                    .collect()
            })
            .collect();
        // trips to the palette, one passage after another (the palette's
        // mixing cache makes the result depend on the order of requests)
        let mut plans = plans;
        let t_geom = t0.elapsed().as_secs_f32();
        let mut prng = Rng::new(seed ^ 0xD1B);
        let mut memo = std::collections::HashMap::new();
        for t in plans.iter_mut() {
            for p in t.iter_mut() {
                if let Some(d) = p.dip.as_mut() {
                    let (x, y, cv) = p.aim_at;
                    // (a fresh generator per dip, so a recipe's draws can't shift the
                    // rest; see `finish_plan`)
                    let seen = self.judge_under(x, y, sp.tool.width * 0.6);
                    let want = match &sp.color_over {
                        Some(g) => g(x, y, seen),
                        None => (sp.color)(x, y),
                    };
                    p.want = want;
                    let want = sp.touch_target(want, seen, cv);
                    *d = sp.paint_for(want, seen, cv, &mut memo, &mut Rng::new(prng.next_u64()));
                }
            }
        }
        let n_memo = memo.len();
        // the hand's ledger: every planned touch and trip to the palette (on
        // the whole canvas, before a crop drops passages; see `tally`)
        let (mpu, mut piles) = (self.mm_per_unit, crate::tally::Piles::default());
        let mut tile_secs = Vec::with_capacity(plans.len());
        for t in plans.iter() {
            let secs0 = self.tally.secs;
            for p in t {
                self.tally.touch(&sp.tool, mpu);
                if p.dip.is_some() {
                    piles.trip(&mut self.tally, p.want);
                }
            }
            tile_secs.push(self.tally.secs - secs0);
        }
        let n: usize = plans.iter().map(|t| t.len()).sum();
        let first_id = self.next_stroke_ids(n as u32);
        let mut offsets = Vec::with_capacity(plans.len());
        let mut acc = 0u32;
        for t in &plans {
            offsets.push(acc);
            acc += t.len() as u32;
        }
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
        // (as the phases did) and a crop render skips those off its window
        let win = self.f;
        let tile_rect: Vec<Option<Rect>> = plans
            .iter()
            .map(|t| t.iter().filter(|p| p.rect.2 > p.rect.0).map(|p| p.rect).reduce(|a, r| (a.0.min(r.0), a.1.min(r.1), a.2.max(r.2), a.3.max(r.3))))
            .collect();
        let order: Vec<usize> = phases.iter().flat_map(|&(px, py)| (0..plans.len()).filter(move |&i| (i % tw) % 2 == px && (i / tw) % 2 == py)).collect();
        // with hand time on, the passages are painted in slices of hand time
        // and the paint ages between them (`tally::batches`); else one batch
        let batches = crate::tally::batches(&order, &tile_secs, self.hand_slice_secs());
        let n_batches = batches.len();
        for (bi, (order, bsecs)) in batches.into_iter().enumerate() {
        let surf = self.surf();
        let order: Vec<usize> = order.into_iter().filter(|&i| !plans[i].is_empty() && tile_rect[i].is_some_and(|r| win.clip(r).is_some())).collect();
        let paint_tile = |ti: usize| {
            let mut held = Held::new(sp.tool.clone(), seed ^ 0x5717 ^ (ti as u64).wrapping_mul(0x9E37_79B9));
            let mut scratch = Vec::new();
            let mut b: Bounds = None;
            for (k, p) in plans[ti].iter().enumerate() {
                if let Some(paint) = p.dip {
                    held.wipe(sp.wipe);
                    held.load(paint, sp.load);
                }
                if p.rect.2 <= p.rect.0 {
                    continue;
                }
                let id = first_id.wrapping_add(offsets[ti] + k as u32);
                // SAFETY: every pixel a touch reaches lies in its footprint,
                // inside its passage's rect; run_ordered never runs passages
                // with overlapping rects at once; `surf()` checked the buffers.
                let r = unsafe { touch_on(surf, &mut held, &p.touch, clip, id, &mut scratch) };
                if let Some((a, c, d, e)) = r {
                    b = Some(match b {
                        None => (a, c, d, e),
                        Some((a0, c0, d0, e0)) => (a0.min(a), c0.min(c), d0.max(d), e0.max(e)),
                    });
                }
            }
            b
        };
        let mut dirty: Bounds = None;
        for r in crate::sched::run_ordered(&order, &tile_rect, paint_tile).into_iter().flatten() {
            dirty = Some(match dirty {
                None => r,
                Some((a0, c0, d0, e0)) => (a0.min(r.0), c0.min(r.1), d0.max(r.2), e0.max(r.3)),
            });
        }
        if let Some((x0, y0, x1, y1)) = dirty {
            self.wet.touch(x0, y0, x1, y1);
        }
        if bi + 1 < n_batches {
            self.hand_pass(bsecs);
        }
        }
        if std::env::var_os("PAINT_DEBUG").is_some() {
            eprintln!("stipple: {n} touches, reach {reach:.1}, tiles {tw}x{th} ({tile:.0} units), plan {t_geom:.2}s + paint {:.2}s ({} recipes), total {:.2}s", t_plan - t_geom, n_memo, t0.elapsed().as_secs_f32());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bristle::{Gesture, Kind};
    use crate::color::hex;

    fn brush_volume(h: &Held) -> f64 {
        h.bristles.iter().map(|b| b.vol as f64).sum()
    }

    fn tools() -> Vec<Tool> {
        vec![Tool::stippler(2.0), Tool::round_sable(8.0), Tool::hog_flat(12.0), Tool::filbert(9.0), Tool::fan(14.0), Tool::rigger(0.8), Tool::badger(20.0)]
    }

    /// Wet paint to pick up, over the middle of the canvas.
    fn wet_canvas(w: usize) -> Canvas {
        let mut c = Canvas::new(w, 1.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(3));
        let mut under = Held::new(Tool::filbert(60.0), 2);
        for k in 0..12 {
            under.reload(Paint::body(hex("#304060")), 1.0);
            let y = 40.0 + k as f32 * 80.0;
            c.drag(&mut under, &Gesture::new(vec![(0.0, y), (1000.0, y)]).pressure(1.0, 1.0), None);
        }
        c
    }

    /// A touch neither creates nor destroys paint.
    #[test]
    fn touch_conserves_paint() {
        let mut c = wet_canvas(400);
        for (k, tool) in tools().into_iter().enumerate() {
            let mut h = Held::new(tool.clone(), 5);
            h.load(Paint::scumble(hex("#d0c060")), 0.8);
            let before = c.wet_total() + brush_volume(&h);
            let t = Touch::at(200.0 + 90.0 * k as f32, 500.0).pressure(0.9).drag(3.0, -2.0).twist(0.6);
            c.touch(&mut h, &t, None);
            let after = c.wet_total() + brush_volume(&h);
            assert!((after - before).abs() <= before * 2e-3, "{:?}: {before} -> {after}", tool.kind);
        }
    }

    /// Every pixel a touch changes lies inside its footprint.
    #[test]
    fn touch_footprint_bounds_every_touched_pixel() {
        for scale in [0.1f32, 0.4, 1.0] {
            let w = (1000.0 * scale) as usize;
            let mut c = wet_canvas(w);
            for tool in tools() {
                let before = c.wet.vol.clone();
                let mut h = Held::new(tool.clone(), 9);
                h.load(Paint::body(hex("#d0c060")), 1.0);
                let t = Touch::at(503.0, 497.0).pressure(1.0).drag(-4.0, 6.0).twist(1.5).angle(0.7);
                c.touch(&mut h, &t, None);
                let r = touch_footprint(&tool, &t, c.f.scale, c.f.w, c.f.h).unwrap();
                for (i, (a, b)) in before.iter().zip(&c.wet.vol).enumerate() {
                    if a != b {
                        let (x, y) = (i % c.f.w, i / c.f.w);
                        assert!(x >= r.0 && x < r.2 && y >= r.1 && y < r.3, "{:?} scale {scale}: ({x},{y}) outside {r:?}", tool.kind);
                    }
                }
            }
        }
    }

    /// Thickness laid by one touch on a smooth canvas, per pixel.
    fn one_touch(tool: &Tool, width_px: usize, p: f32) -> (Canvas, f32) {
        let mut c = Canvas::new(width_px, 1.0, hex("#c8b89a"));
        let mut h = Held::new(tool.clone(), 3);
        h.load(Paint::scumble(hex("#304060")), 0.8);
        c.touch(&mut h, &Touch::at(500.0, 500.0).pressure(p), None);
        let s = c.f.scale;
        (c, s)
    }

    /// A pressed round tip lays one continuous patch, fullest in the middle:
    /// no dot per hair (frogspawn) and no hollow ring.
    #[test]
    fn touch_is_one_solid_patch() {
        for tool in [Tool::round_sable(8.0), Tool::stippler(3.0)] {
            let (c, s) = one_touch(&tool, 3200, 0.8);
            let (cx, cy) = (500.0 * s, 500.0 * s);
            let at = |x: f32, y: f32| c.wet.vol[(y as usize) * c.f.w + x as usize];
            let peak = c.wet.vol.iter().cloned().fold(0.0f32, f32::max);
            // radius of the patch: farthest pixel with a tenth of the peak
            let mut rmax = 0.0f32;
            for (i, &v) in c.wet.vol.iter().enumerate() {
                if v > 0.1 * peak {
                    let (x, y) = ((i % c.f.w) as f32 + 0.5 - cx, (i / c.f.w) as f32 + 0.5 - cy);
                    rmax = rmax.max((x * x + y * y).sqrt());
                }
            }
            assert!(rmax > 2.0, "{:?}: no patch", tool.kind);
            // the inner half is filled everywhere
            let n = 64;
            let mut min_in = f32::MAX;
            for k in 0..n {
                let a = k as f32 / n as f32 * std::f32::consts::TAU;
                for rr in [0.0, 0.25, 0.5] {
                    min_in = min_in.min(at(cx + a.cos() * rr * rmax, cy + a.sin() * rr * rmax));
                }
            }
            assert!(min_in > 0.3 * peak, "{:?}: holes inside the patch ({min_in} vs peak {peak})", tool.kind);
            // no hollow: the middle is at least as full as the rim
            let rim: f32 = (0..n).map(|k| { let a = k as f32 / n as f32 * std::f32::consts::TAU; at(cx + a.cos() * 0.7 * rmax, cy + a.sin() * 0.7 * rmax) }).sum::<f32>() / n as f32;
            let mid: f32 = (0..n).map(|k| { let a = k as f32 / n as f32 * std::f32::consts::TAU; at(cx + a.cos() * 0.15 * rmax, cy + a.sin() * 0.15 * rmax) }).sum::<f32>() / n as f32;
            assert!(mid >= rim, "{:?}: hollow ring (middle {mid} < rim {rim})", tool.kind);
        }
    }

    /// The same touch at 1000 and 3200 px lays the same paint over the same
    /// area (in canvas units).
    #[test]
    fn touch_is_resolution_independent() {
        for tool in [Tool::stippler(2.0), Tool::stippler(4.0), Tool::round_sable(8.0)] {
            let stats = |w: usize| {
                let (c, s) = one_touch(&tool, w, 0.7);
                let px = 1.0 / (s * s);
                let vol: f32 = c.wet.vol.iter().sum::<f32>() * px;
                let mean_t = vol / c.wet.vol.iter().filter(|&&v| v > 1e-4).count() as f32 / px;
                // area holding half the paint (robust to the soft rim)
                let mut v: Vec<f32> = c.wet.vol.iter().cloned().filter(|&v| v > 0.0).collect();
                v.sort_by(|a, b| b.partial_cmp(a).unwrap());
                let (mut acc, mut n) = (0.0, 0);
                for x in &v {
                    acc += x * px;
                    n += 1;
                    if acc >= vol * 0.5 {
                        break;
                    }
                }
                (vol, n as f32 * px, mean_t)
            };
            let (v1, a1, _) = stats(1000);
            let (v3, a3, _) = stats(3200);
            assert!((v1 / v3 - 1.0).abs() < 0.15, "{:?} w{}: volume {v1} vs {v3}", tool.kind, tool.width);
            // (a coarse pixel grid can only resolve an area to about a pixel)
            assert!((a1 - a3).abs() < 0.35 * a3 + 1.0, "{:?} w{}: area {a1} vs {a3}", tool.kind, tool.width);
        }
    }

    #[test]
    fn aimed_touch_paint_reaches_reachable_targets() {
        let seen = hex("#5a6878");
        // reachable: near the underlayer's tone (a half-hiding paint at 0.6
        // coats can't lift a dark blue to a pale sky; see the next assert)
        for want in [hex("#6a7488"), hex("#5f6d80"), hex("#55606e"), hex("#66707a")] {
            let got = Paint::aimed(want, seen, 0.6, 0.5, 0.5).over(seen, 0.6);
            for k in 0..3 {
                assert!((got[k] - want[k]).abs() < 0.01, "{want:?}: {got:?}");
            }
        }
    }

    #[test]
    fn aimed_touch_paint_gets_as_near_as_it_can() {
        let seen = hex("#5a6878");
        let want = hex("#7d8fae");
        let got = Paint::aimed(want, seen, 0.6, 0.5, 0.5).over(seen, 0.6);
        let lightest = Paint::new([0.995; 3], 0.5, 0.5).over(seen, 0.6);
        for k in 0..3 {
            assert!(got[k] <= want[k] + 0.01 && got[k] >= seen[k].min(want[k]) - 0.01, "{got:?}");
            assert!(got[k] <= lightest[k] + 1e-3);
        }
    }

    /// Speckle of a stipple over a flat field: (std of L, mean L lift) over
    /// x in `xs`. Coverage runs 0 → `cmax` across the canvas.
    fn speckle(field: Rgb, want: Rgb, cmax: f32, xs: std::ops::Range<f32>, set: impl Fn(Stipple) -> Stipple) -> (f32, f32) {
        let st = crate::style::Style::friedrich();
        let mut c = Canvas::new(400, 1.0, field);
        let f = c.frame();
        let before = to_oklab(field)[0];
        let sp = set(Stipple::new(Tool::stippler(3.0)).mixed(&st.palette, 0.45).color(move |_, _| want).coverage(move |x, _| cmax * x / 1000.0));
        c.stipple(&Mask::full(f), &sp, 4);
        c.dry();
        let ls: Vec<f32> = c.pixels().iter().enumerate().filter(|(i, _)| xs.contains(&((i % f.w) as f32 / f.scale))).map(|(_, p)| to_oklab(*p)[0]).collect();
        let n = ls.len() as f32;
        let m = ls.iter().sum::<f32>() / n;
        ((ls.iter().map(|l| (l - m).powi(2)).sum::<f32>() / n).sqrt(), m - before)
    }

    /// A lighter stipple thinning out over a field fades into it instead of
    /// ending in salt, aimed or not, over a mid tone or a dark (amnesia 2,
    /// coast #9, mountains #10).
    #[test]
    fn thin_stipple_fades_instead_of_salt() {
        // a pale stipple thinning out over a mid-blue sky
        let (sky, pale) = (hex("#7d8fae"), hex("#b8c2d2"));
        let (salt, lift0) = speckle(sky, pale, 1.2, 150.0..400.0, |s| s.fade(0.0));
        let (faded, lift1) = speckle(sky, pale, 1.2, 150.0..400.0, |s| s);
        println!("thin pale stipple (coverage 0.18-0.48): L sd {salt:.4} -> {faded:.4}, lift {lift0:+.4} -> {lift1:+.4}");
        assert!(faded < 0.6 * salt, "thin stipple still salty: {faded} vs {salt}");
        assert!(lift1 > 0.0, "it still lifts the tone: {lift1}");
        // where it is dense, it reaches the same tone
        let (_, dense0) = speckle(sky, pale, 1.2, 900.0..1000.0, |s| s.fade(0.0));
        let (_, dense1) = speckle(sky, pale, 1.2, 900.0..1000.0, |s| s);
        assert!((dense0 - dense1).abs() < 0.3 * dense0.abs(), "dense tone {dense0} vs {dense1}");
        // the thin fringe of a pale mist over a dark, masstone-mixed (aim
        // off, as mountains #10 did): pale dots on the dark fade out instead
        let (dark, mist) = (hex("#2c3038"), hex("#9aa0a8"));
        let (dots, l0) = speckle(dark, mist, 1.2, 150.0..400.0, |s| s.aim(false).fade(0.0));
        let (fringe, l1) = speckle(dark, mist, 1.2, 150.0..400.0, |s| s.aim(false));
        println!("mist fringe over dark (coverage 0.18-0.48): L sd {dots:.4} -> {fringe:.4}, lift {l0:+.4} -> {l1:+.4}");
        assert!(fringe < 0.6 * dots, "mist fringe still static: {fringe} vs {dots}");
        assert!(l1 > 0.0, "the fringe still lifts the dark: {l1}");
    }

    fn stipple_scene() -> Canvas {
        let st = crate::style::Style::friedrich();
        let mut c = st.prepare(300, 1.5, 7);
        let m = Mask::from_fn(c.f, |x, y| crate::smoothstep(100.0, 300.0, x) * (1.0 - crate::smoothstep(400.0, 450.0, y)));
        let sp = Stipple::new(Tool::stippler(4.0)).mixed(&st.palette, 0.5).color(|_, y| if y < 200.0 { hex("#7d8fae") } else { hex("#e0d4b0") }).coverage(|x, _| x / 500.0).drag(1.0, None);
        c.stipple(&m, &sp, 3);
        c.dry();
        c
    }

    fn fp(c: &Canvas) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for v in c.px.iter().flatten().chain(c.height.iter()) {
            for b in v.to_bits().to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100_0000_01b3);
            }
        }
        h
    }

    #[test]
    fn stipple_is_deterministic_across_thread_counts() {
        let a = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap().install(|| fp(&stipple_scene()));
        let b = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap().install(|| fp(&stipple_scene()));
        assert_eq!(a, b);
    }

    /// Denser coverage covers more of the canvas, and nothing is painted
    /// outside the region.
    #[test]
    fn stipple_follows_coverage_and_mask() {
        let mut c = Canvas::new(400, 1.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(3));
        let m = Mask::from_fn(c.f, |x, _| if (100.0..900.0).contains(&x) { 1.0 } else { 0.0 });
        let sp = Stipple::new(Tool::stippler(5.0)).color(|_, _| hex("#304060")).coverage(|x, _| if x < 500.0 { 0.4 } else { 2.0 }).aim(false).feather(0.0);
        c.stipple(&m, &sp, 5);
        let frac = |x0: f32, x1: f32| {
            let (a, b) = ((x0 * c.f.scale) as usize, (x1 * c.f.scale) as usize);
            let mut n = 0;
            let mut hit = 0;
            for y in 0..c.f.h {
                for x in a..b {
                    n += 1;
                    if c.wet.vol[y * c.f.w + x] > 0.02 {
                        hit += 1;
                    }
                }
            }
            hit as f32 / n as f32
        };
        let (thin, dense, outside) = (frac(150.0, 450.0), frac(550.0, 850.0), frac(0.0, 80.0) + frac(920.0, 1000.0));
        assert!(dense > thin * 1.5 && dense > 0.6, "coverage: thin {thin}, dense {dense}");
        assert!(outside == 0.0, "painted outside the region: {outside}");
        let _ = Kind::Round;
    }

    /// Small, narrow and scattered regions get stippled however they sit on
    /// any probe lattice (reviews: a radius-10 disk with a 20-unit stippler,
    /// and a 2-unit band at y 100 vs y 102, came out empty).
    #[test]
    fn small_and_narrow_regions_are_not_skipped() {
        // compact disk narrower than the old probe spacing
        let mut c = Canvas::new_window(1000, 1.0, [0.5; 3], None);
        let mask = Mask::from_fn(c.frame(), |x, y| if (x - 500.0).powi(2) + (y - 500.0).powi(2) < 100.0 { 1.0 } else { 0.0 });
        let sp = Stipple::new(Tool::stippler(20.0)).coverage(|_, _| 10.0).aim(false).color(|_, _| [0.1; 3]);
        c.stipple(&mask, &sp, 123);
        assert!(c.wet_total() > 0.0, "radius-10 disk, 20-unit stippler: wet_total {}", c.wet_total());

        // a narrow coverage band, wherever it falls
        for lo in [100.0f32, 101.0, 102.0, 103.5] {
            let mut c = Canvas::new_window(400, 2.0, [0.1; 3], None);
            let m = Mask::full(c.frame());
            let sp = Stipple::new(Tool::stippler(2.0))
                .coverage(move |_, y| if y >= lo && y < lo + 2.0 { 10.0 } else { 0.0 })
                .color(|_, _| [0.8; 3])
                .aim(false);
            c.stipple(&m, &sp, 7);
            c.dry();
            let changed = c.pixels().iter().filter(|p| **p != [0.1; 3]).count();
            assert!(changed > 1000, "band [{lo}, {}): {changed} pixels changed", lo + 2.0);
        }

        // separated islands, each smaller than the old probe spacing
        let mut c = Canvas::new_window(1000, 1.0, [0.5; 3], None);
        let isl = [(101.0f32, 97.0f32), (333.0, 251.0), (470.0, 520.0)];
        let mask = Mask::from_fn(c.frame(), |x, y| if isl.iter().any(|&(a, b)| (x - a).powi(2) + (y - b).powi(2) < 16.0) { 1.0 } else { 0.0 });
        let sp = Stipple::new(Tool::stippler(6.0)).coverage(|_, _| 3.0).aim(false).color(|_, _| [0.1; 3]);
        c.stipple(&mask, &sp, 5);
        c.dry();
        for &(a, b) in &isl {
            let hit = (-6..=6).flat_map(|dy| (-6..=6).map(move |dx| (dx, dy))).filter(|&(dx, dy)| c.pixels()[((b as i32 + dy) * 1000 + a as i32 + dx) as usize] != [0.5; 3]).count();
            assert!(hit > 0, "island at ({a}, {b}) was skipped");
        }
    }

    /// A veil stippled around dark motifs (the region cut out of it with a
    /// small gap) reaches its tone right up to the gap, not only out in the
    /// open: no pale halo matting the motifs in (amnesia 3, easel3_free: the
    /// sea veil left a 3–6 unit rim of the old, paler sea around the stones,
    /// the figure and the trunk).
    #[test]
    fn veil_reaches_its_tone_up_to_dark_motifs() {
        let st = crate::style::Style::friedrich();
        let mut c = Canvas::new_window(2000, 4.0, hex("#8a8894"), None).with_size_mm(440.0);
        let f = c.frame();
        // dark bars and blocks, like trunks, a figure and standing stones
        let bars: Vec<(f32, f32, f32, f32)> = (0..9)
            .map(|k| {
                let x = 60.0 + k as f32 * 105.0;
                let w = [6.0, 14.0, 30.0][k % 3];
                (x, 40.0, x + w, 60.0 + 40.0 * (k % 4) as f32 + 110.0)
            })
            .collect();
        // distance (units) outside the nearest bar, 0 inside
        let dist = |x: f32, y: f32| {
            bars.iter()
                .map(|&(x0, y0, x1, y1)| {
                    let (dx, dy) = ((x0 - x).max(x - x1).max(0.0), (y0 - y).max(y - y1).max(0.0));
                    (dx * dx + dy * dy).sqrt()
                })
                .fold(f32::MAX, f32::min)
        };
        let dark = Mask::from_fn(f, |x, y| if dist(x, y) <= 0.0 { 1.0 } else { 0.0 });
        let ink = Stipple::new(Tool::stippler(3.0)).paint(1.0, 0.4).color(|_, _| hex("#24221e")).coverage(|_, _| 8.0).aim(false).fade(0.0).clip(true);
        c.stipple(&dark, &ink, 1);
        c.dry();
        // laid in thick body paint: a plateau ~0.6 mm proud of the thin
        // field (the stones in easel3_free stand ~650 µm above the sea)
        for (hg, &m) in c.height.iter_mut().zip(&dark.data) {
            *hg += 600.0 * m;
        }
        c.surf_gen += 1;
        // the veil: the field minus the motifs grown by 1.5 units, clipped
        let veil = Mask::from_fn(f, |x, y| if dist(x, y) > 1.5 && (20.0..230.0).contains(&y) { 1.0 } else { 0.0 });
        let before: Vec<f32> = c.pixels().iter().map(|p| to_oklab(*p)[0]).collect();
        let sp = Stipple::new(Tool::stippler(2.2)).mixed(&st.palette, 0.5).color(|_, _| hex("#4a5068")).coverage(|_, _| 2.6).pressure(0.4, 0.8).dips(20, 0.35, 0.6).clip(true);
        c.stipple(&veil, &sp, 9);
        c.dry();
        // mean darkening at a distance band from the motifs, inside the veil
        let drop = |d0: f32, d1: f32| {
            let (mut s, mut n) = (0.0, 0);
            for (i, p) in c.pixels().iter().enumerate() {
                let (x, y) = (((i % f.w) as f32 + 0.5) / f.scale, ((i / f.w) as f32 + 0.5) / f.scale);
                let d = dist(x, y);
                if d >= d0 && d < d1 && (30.0..220.0).contains(&y) && veil.data[i] >= 1.0 {
                    s += before[i] - to_oklab(*p)[0];
                    n += 1;
                }
            }
            s / n as f32
        };
        let (near, far) = (drop(1.5, 4.0), drop(12.0, 40.0));
        println!("veil darkening: {near:.4} near the motifs, {far:.4} in the open");
        assert!(far > 0.05, "the veil didn't darken the field: {far}");
        assert!(near > 0.8 * far, "pale halo: the veil darkens {near:.4} next to the motifs vs {far:.4} in the open");
    }
}
