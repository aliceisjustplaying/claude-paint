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
use crate::pigment::Pigment;
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
    /// Clip the hairs' contact to the mask.
    pub clip: bool,
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
            clip: false,
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
    fn paint_for(&self, want: Rgb, seen: Rgb, coverage: f32, rng: &mut Rng) -> Paint {
        let coats = self.touch_coats() * coverage.max(1.0);
        match self.palette {
            Some((pal, medium0)) => {
                let (target, medium) = if self.aim {
                    // WORKAROUND (until the color stream lands): invert KM for
                    // the paint's own hiding, mix, and correct once for the
                    // hiding of the mixture the palette actually gives. If the
                    // thinned paint can't get there over this substrate, the
                    // painter thins it less (more body, more hiding).
                    let wl = to_oklab(want);
                    let mut best = (f32::MAX, want, medium0);
                    for medium in [medium0, medium0 * 0.5, 0.0] {
                        let mut hide = pal.mix(want).paint(medium).hiding;
                        let mut t = want;
                        for _ in 0..2 {
                            t = aim_km(want, seen, hide, coats);
                            hide = pal.mix(t).paint(medium).hiding;
                        }
                        let pt = pal.mix(t).paint(medium);
                        let got = to_oklab(Pigment::with_hiding(pt.color, pt.hiding).over(seen, coats));
                        let err = ((got[0] - wl[0]).powi(2) + (got[1] - wl[1]).powi(2) + (got[2] - wl[2]).powi(2)).sqrt();
                        if err < best.0 - 0.005 {
                            best = (err, t, medium);
                        }
                        if err < 0.02 {
                            break;
                        }
                    }
                    (best.1, best.2)
                } else {
                    (want, medium0)
                };
                let pt = pal.remix(&pal.mix(target), self.mix_jitter, rng).paint(medium);
                if std::env::var_os("PAINT_DEBUG_AIM").is_some() && rng.f() < 0.002 {
                    let hx = |c: Rgb| format!("#{:02x}{:02x}{:02x}", (crate::color::linear_to_srgb(c[0]) * 255.0) as u8, (crate::color::linear_to_srgb(c[1]) * 255.0) as u8, (crate::color::linear_to_srgb(c[2]) * 255.0) as u8);
                    let got = Pigment::with_hiding(pt.color, pt.hiding).over(seen, coats);
                    eprintln!("aim want {} seen {} coats {coats:.2} -> target {} medium {medium:.2} paint {} hide {:.2} ({}) => {}", hx(want), hx(seen), hx(target), hx(pt.color), pt.hiding, pal.recipe(&pal.mix(target)), hx(got));
                }
                pt
            }
            None => {
                let target = if self.aim { aim_km(want, seen, self.hiding, coats) } else { want };
                let lab = to_oklab(target);
                let col = from_oklab([lab[0] + rng.normal() * self.jitter.0, lab[1] + rng.normal() * self.jitter.1, lab[2] + rng.normal() * self.jitter.1]);
                Paint { color: col, hiding: self.hiding, stiff: self.stiff }
            }
        }
    }
}

/// WORKAROUND for the missing substrate-aware palette API: the paint color
/// (its appearance at one coat over white, as `Paint::color` means) whose
/// Kubelka–Munk layer of `coats` with `hiding` over `seen` looks like `want`.
/// Per channel by bisection (reflectance rises with the paint's color);
/// clamped to what paint can be, so an unreachable target gets as close as
/// the paint allows.
pub(crate) fn aim_km(want: Rgb, seen: Rgb, hiding: f32, coats: f32) -> Rgb {
    let (mut lo, mut hi) = ([0.002f32; 3], [0.998f32; 3]);
    for _ in 0..18 {
        let mid = [(lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5, (lo[2] + hi[2]) * 0.5];
        let got = Pigment::with_hiding(mid, hiding.clamp(0.01, 0.99)).over(seen, coats);
        for c in 0..3 {
            if got[c] < want[c] {
                lo[c] = mid[c];
            } else {
                hi[c] = mid[c];
            }
        }
    }
    [(lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5, (lo[2] + hi[2]) * 0.5]
}

/// One planned touch.
struct Plan {
    touch: Touch,
    /// Paint to dip into first (None = keep going with what's on the brush).
    dip: Option<Paint>,
    rect: Rect,
}

impl Canvas {
    /// What the canvas looks like at a pixel: wet paint over the dry picture.
    fn seen_px(&self, i: usize) -> Rgb {
        let v = self.wet.vol[i];
        if v < 1e-5 {
            return self.px[i];
        }
        let c = mixbox::latent_to_linear_float_rgb(&self.wet.lat[i]);
        Pigment::with_hiding(c, self.wet.hide[i][0].clamp(0.01, 0.99)).over(self.px[i], v)
    }

    /// What the painter sees around (x, y), within `r` units.
    fn seen_around(&self, x: f32, y: f32, r: f32) -> Rgb {
        let mut acc = [0.0f32; 3];
        for (dx, dy) in [(0.0, 0.0), (-r, 0.0), (r, 0.0), (0.0, -r), (0.0, r)] {
            let p = self.seen_px(self.f.index(x + dx, y + dy));
            for c in 0..3 {
                acc[c] += p[c] / 5.0;
            }
        }
        acc
    }

    /// Stipple the region `mask` (its value scales the coverage) touch by
    /// touch with a simulated brush (see `Stipple`).
    ///
    /// Touches are planned up front on a jittered grid thinned by the
    /// coverage, then grouped into square passages larger than twice any
    /// touch's reach, and passages in a 2×2 checkerboard phase are painted in
    /// parallel, each with its own brush.
    pub fn stipple(&mut self, mask: &Mask, sp: &Stipple, seed: u64) {
        self.check_mask(mask);
        let f = self.f;
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
        // the densest coverage asked for sets the grid
        let probe = (sp.tool.width * 3.0).max(4.0);
        let (pw, ph) = (((ux1 - ux0) / probe).ceil() as usize + 1, ((uy1 - uy0) / probe).ceil() as usize + 1);
        let dmax = (0..pw * ph)
            .into_par_iter()
            .map(|k| {
                let (x, y) = (ux0 + (k % pw) as f32 * probe, uy0 + (k / pw) as f32 * probe);
                let mut m = 0.0f32;
                for (dx, dy) in [(0.0, 0.0), (0.5, 0.0), (0.0, 0.5), (0.5, 0.5)] {
                    m = m.max(cov(x + dx * probe, y + dy * probe));
                }
                m
            })
            .reduce(|| 0.0, f32::max)
            * 1.15;
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
                let mut pts = pts.clone();
                for i in (1..pts.len()).rev() {
                    let j = (rng.next_u64() % (i as u64 + 1)) as usize;
                    pts.swap(i, j);
                }
                pts.iter()
                    .enumerate()
                    .map(|(k, &(x, y))| {
                        let cv = cov(x, y);
                        let dip = (k % sp.dip_every == 0).then(|| sp.paint_for((sp.color)(x, y), self.seen_around(x, y, sp.tool.width * 0.6), cv, &mut rng));
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
                        Plan { touch, dip, rect }
                    })
                    .collect()
            })
            .collect();
        let n: usize = plans.iter().map(|t| t.len()).sum();
        let first_id = self.next_stroke_ids(n as u32);
        let mut offsets = Vec::with_capacity(plans.len());
        let mut acc = 0u32;
        for t in &plans {
            offsets.push(acc);
            acc += t.len() as u32;
        }
        let t_plan = t0.elapsed().as_secs_f32();

        let clip = if sp.clip { Some(mask) } else { None };
        let surf = self.surf();
        let mut rng = Rng::new(seed ^ 0xFA5E);
        let mut phases = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)];
        for i in (1..4).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            phases.swap(i, j);
        }
        let mut dirty: Bounds = None;
        for (px, py) in phases {
            let idx: Vec<usize> = (0..plans.len()).filter(|&i| (i % tw) % 2 == px && (i / tw) % 2 == py && !plans[i].is_empty()).collect();
            let results: Vec<Bounds> = idx
                .par_iter()
                .map(|&ti| {
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
                        // SAFETY: every pixel a touch reaches lies in its
                        // footprint, within `reach` of its tile; tiles in one
                        // phase are a tile (> 2·reach) apart, so no two
                        // threads share a pixel; `surf()` checked the buffers.
                        let r = unsafe { touch_on(surf, &mut held, &p.touch, clip, id, &mut scratch) };
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
        if let Some((x0, y0, x1, y1)) = dirty {
            self.wet.touch(x0, y0, x1, y1);
        }
        if std::env::var_os("PAINT_DEBUG").is_some() {
            eprintln!("stipple: {n} touches, reach {reach:.1}, tiles {tw}x{th} ({tile:.0} units), plan {t_plan:.2}s, total {:.2}s", t0.elapsed().as_secs_f32());
        }
    }
}
