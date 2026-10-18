//! A paper mask, and a blade and a dry brush on pastel.
//!
//! **The sheet.** A sheet of paper laid over part of the picture to keep it
//! clean, as pastellists do. It is a surface of its own lying `caliper` µm
//! above the picture, with a paper's pores: a pastel stick rests on it as on
//! any surface (the contact solver sees it as raised ground), so near its
//! edge the stick bridges from sheet to picture and the edge it leaves is as
//! sharp as the stick's angle and force make it. What lands on the sheet
//! (strokes, crumbs) stays on the sheet and leaves with it; the finger, the
//! eraser, fixative, the blade and the dry brush don't reach under it.
//! Wet-paint tools are not modeled under a sheet (the easel refuses them
//! while one is down).
//!
//! **The blade on pastel** (notes/research/pastel_removal_air_blade.md §B,
//! paper_mechanics_and_transport.md §1, §3). A painting knife's unsharpened
//! edge, pressed with a hand's 1–5 N along its length, sinks into the sheet
//! by its measured compression with the load spread by the fibers'
//! through-thickness shear (`edge_sink_um`): deeper than most pores, so it
//! takes the loose pastel down into the tooth. Pressed hard enough it leaves
//! the paper permanently flatter (burnished, the tooth shallower); its drag
//! tears fiber joints (nap: more tooth there). A scalpel's edge cuts fiber
//! walls, shaving about a fiber layer and the pastel in it, fixed too;
//! otherwise fixed pastel stays (no wear law of steel on it is known).
//!
//! **A dry brush over pastel** (notes/research/pastel_brushing.md). A
//! bristle tip pushes sideways with 10³–10⁶ times the force that rolls a
//! pastel particle off, so brushing is limited by reach: the tips (radius r,
//! over gaps between fibers of width w) sink h = r − √(r² − w²/4), and the
//! pastel filling the pores deeper than that, µ·e^(−h/µ), stays (the ghost),
//! with the fixed. The bristles keep what they lift until they are full (a
//! fur brush's pile holds 5–7 % of its volume in toner: a #8 hog some
//! 2–10 mg); the rest is pushed ahead into the pores downstream, met again,
//! and left where the brush lifts.
//!
//! Also what a stick held up to the picture would touch (`stick_contact`),
//! for the easel's hold look.

use crate::canvas::{Canvas, Frame};
use crate::color::Rgb;
use crate::graphite::{cover, uncover};
use crate::mask::Mask;
use crate::pastel::PARTICLE_UM;

/// A sheet of paper laid on the picture.
#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    /// Where it lies, per pixel of the window.
    pub(crate) over: Vec<bool>,
    /// Its thickness, µm.
    pub(crate) caliper_um: f32,
    /// Its pores' mean depth, µm.
    pub(crate) micro_um: f32,
    /// Its color (linear RGB).
    pub(crate) tone: Rgb,
    /// The pastel caught on it, for the look: coverage and color.
    pub(crate) a: Vec<f32>,
    pub(crate) r: Vec<Rgb>,
}

impl Sheet {
    pub(crate) fn covers_whole_pixel(&self, f: Frame, i: usize) -> bool {
        let (x, y) = (i % f.full_w, i / f.full_w);
        x >= f.x0 && y >= f.y0 && x < f.x0 + f.w && y < f.y0 + f.h && self.over[(y - f.y0) * f.w + x - f.x0]
    }
}

/// A hog bristle's tip radius, µm (bristles about 160 µm thick, tapered and
/// split at the end: the research's 25–75 µm, its middle).
pub const HOG_TIP_UM: f32 = 50.0;
/// The largest crumb a dry brush drops, µm across: agglomerates of 50–150 µm
/// are partly broken up by a bristle (pastel_brushing.md §4).
const BRUSH_CRUMB_UM: f32 = 100.0;
/// What a brush's loaded tip holds, mg of pastel per mm³ of tip: toner fills
/// 5–7 % of a fur brush's pile at saturation, 0.053–0.074 g/cm³ [M, D]
/// (pastel_removal_air_blade.md §A3).
pub const BRUSH_HOLDS_MG_MM3: f32 = 0.06;
/// Pastel as it lies in the tooth, mg/mm³ (a saturated 20 µm layer is
/// ~2.6 mg/cm²: 1.3 g/cm³; pastel_stick_tribology.md §2).
const PASTEL_MG_MM3: f32 = 1.3;
/// The largest loose crumb lying on the tooth, µm (crumbs from soft pastel
/// run to 100–300 µm: pastel_stick_tribology.md §3).
const CRUMB_MAX_UM: f32 = 300.0;

/// The steel flexes over broad relief and bridges fine hollows: the blade
/// rests on the highest point within this many mm along it (as `Canvas::knife`).
const FLEX_MM: f32 = 4.0;

/// Through-thickness shear modulus of a drawing paper, MPa: 16–127 measured
/// in paperboard plies (Nygårds); for a drawing paper 15–50 [E], the middle
/// (paper_mechanics_and_transport.md §1).
const PAPER_GXZ_MPA: f32 = 30.0;
/// Steel on paper, friction (0.2–0.4 [E]), and the strength of a fiber–fiber
/// joint, N (1.1–6.5 mN measured: the middle).
const STEEL_PAPER_MU: f32 = 0.3;
const FIBER_JOINT_N: f32 = 3.0e-3;
/// Fiber wall hardness, MPa (0.27–0.42 GPa by nanoindentation [M]).
const FIBER_WALL_MPA: f32 = 350.0;
/// A scalpel cutting shaves at most about a fiber layer a pass, µm (10–15 [D]).
const SHAVE_UM: f32 = 12.0;
/// Torn fiber ends standing proud deepen the tooth locally 1.3–2.5× [E]: at
/// full nap, the middle.
const NAP_TOOTH: f32 = 1.9;

/// Chen et al.'s (2020) z-compression of paper: stress (MPa) at strain `e`.
fn chen(e: f32) -> f32 {
    0.636 * ((13.54 * e).exp() - 1.0)
}

/// How deep an edge of radius `r_um` sinks into a sheet `t_um` thick on a
/// rigid board under line load `q` N/mm, µm. The sheet compresses by Chen's
/// curve, and its fibrous surface, ~190 times stiffer in plane than through
/// it, carries load sideways by its through-thickness shear: a Pasternak
/// layer, p = σ(w/T) − (G_xz·T/3)·w″ (the spreading length T√(G/3E_z),
/// some 0.2–0.6 mm). Solved on a grid across the edge with the contact
/// found by an active set; the load matched by bisection on the depth.
/// (Without the shear it is the bed of springs: 1.25–6× deeper at a knife's
/// light loads, the same for a pen's hard ones; §1.)
pub fn edge_sink_um(q: f32, r_um: f32, t_um: f32) -> f32 {
    let gp = PAPER_GXZ_MPA * t_um / 3.0; // MPa·µm
    let ell = t_um * (PAPER_GXZ_MPA / (3.0 * 8.6)).sqrt();
    let half = (6.0 * ell).max(4.0 * (2.0 * r_um * t_um * 0.5).sqrt());
    let n = 401usize;
    let h = 2.0 * half / (n - 1) as f32;
    let xs: Vec<f32> = (0..n).map(|i| -half + h * i as f32).collect();
    // the load the sheet carries at depth `d0` under the edge's center, N/mm
    let load = |d0: f32| -> f32 {
        let edge: Vec<f32> = xs.iter().map(|&x| d0 - x * x / (2.0 * r_um)).collect();
        let mut w: Vec<f32> = edge.iter().map(|&e| e.max(0.0)).collect();
        let mut contact: Vec<bool> = edge.iter().map(|&e| e > 0.0).collect();
        for _ in 0..12 {
            // Newton on the free nodes: gp·(w[i-1] − 2w[i] + w[i+1])/h² − σ(w[i]/T) = 0
            for _ in 0..8 {
                let (mut a, mut b, mut c, mut rhs) = (vec![0.0f32; n], vec![1.0f32; n], vec![0.0f32; n], vec![0.0f32; n]);
                for i in 1..n - 1 {
                    if contact[i] {
                        continue;
                    }
                    let e = w[i] / t_um;
                    let ds = 0.636 * 13.54 * (13.54 * e).exp() / t_um;
                    a[i] = gp / (h * h);
                    c[i] = gp / (h * h);
                    b[i] = -2.0 * gp / (h * h) - ds;
                    rhs[i] = -(gp * (w[i - 1] - 2.0 * w[i] + w[i + 1]) / (h * h) - chen(e));
                }
                // Thomas: the increments (fixed nodes: 0)
                for i in 1..n {
                    if b[i - 1].abs() < 1e-12 {
                        continue;
                    }
                    let m = a[i] / b[i - 1];
                    b[i] -= m * c[i - 1];
                    rhs[i] -= m * rhs[i - 1];
                }
                let mut dw = vec![0.0f32; n];
                for i in (0..n).rev() {
                    let nxt = if i + 1 < n { dw[i + 1] } else { 0.0 };
                    dw[i] = (rhs[i] - c[i] * nxt) / b[i];
                }
                for i in 1..n - 1 {
                    if !contact[i] {
                        w[i] = (w[i] + dw[i]).max(0.0);
                    }
                }
            }
            // the active set: free nodes the edge would pass join it; contact
            // nodes pulling instead of pushing leave it
            let mut changed = false;
            for i in 1..n - 1 {
                let p = chen(w[i] / t_um) - gp * (w[i - 1] - 2.0 * w[i] + w[i + 1]) / (h * h);
                if !contact[i] && w[i] < edge[i] {
                    contact[i] = true;
                    w[i] = edge[i];
                    changed = true;
                } else if contact[i] && p < 0.0 {
                    contact[i] = false;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        // the edge's load: the contact pressure over the contact (MPa·µm = 1e-3 N/mm)
        (1..n - 1).filter(|&i| contact[i]).map(|i| (chen(w[i] / t_um) - gp * (w[i - 1] - 2.0 * w[i] + w[i + 1]) / (h * h)) * h).sum::<f32>() * 1e-3
    };
    let (mut lo, mut hi) = (0.0f32, t_um * 0.6);
    for _ in 0..30 {
        let m = 0.5 * (lo + hi);
        if load(m) < q { lo = m } else { hi = m }
    }
    0.5 * (lo + hi)
}

impl Canvas {
    /// Lay a sheet over `m` (where it is above 0.5): `caliper_um` thick, its
    /// pores `micro_um` deep, of color `tone`. One sheet at a time.
    pub fn lay_sheet(&mut self, m: &Mask, caliper_um: f32, micro_um: f32, tone: Rgb) -> Result<(), String> {
        if self.sheet.is_some() {
            return Err("a sheet already lies on the picture: lift it first (lift_sheet())".into());
        }
        self.check_mask(m);
        let f = self.f;
        let n = self.px.len();
        let over: Vec<bool> = (0..n).map(|i| m.data[f.whole_index(i)] > 0.5).collect();
        self.sheet = Some(Sheet { over, caliper_um: caliper_um.max(1.0), micro_um: micro_um.max(0.1), tone, a: vec![0.0; n], r: vec![[0.0; 3]; n] });
        Ok(())
    }

    /// Lift the sheet, with whatever it caught. False if none was laid.
    pub fn lift_sheet(&mut self) -> bool {
        self.sheet.take().is_some()
    }

    /// The mean pore depth of the support, µm (a paper's), if it has one.
    pub fn mean_micro_um(&self) -> Option<f32> {
        (!self.micro.is_empty()).then(|| self.micro.iter().sum::<f32>() / self.micro.len() as f32)
    }

    pub fn has_sheet(&self) -> bool {
        self.sheet.is_some()
    }

    /// Whether the sheet lies over window pixel `i`.
    #[inline]
    pub(crate) fn sheet_over(&self, i: usize) -> bool {
        self.sheet.as_ref().is_some_and(|s| s.over[i])
    }

    /// The surface a stick comes down on at window pixel `i`: its height (µm),
    /// its pores' mean depth (µm) and how stiffly it gives (MPa/µm). The
    /// sheet, where it lies, is that much higher, with its own pores.
    #[inline]
    pub(crate) fn surface_point(&self, i: usize) -> (f32, f32, f32) {
        match &self.sheet {
            Some(s) if s.over[i] => (self.height[i] + s.caliper_um, s.micro_um, self.give_mpa_per_um(i)),
            _ => (self.height[i], self.micro_um(i), self.give_mpa_per_um(i)),
        }
    }

    /// Pastel landed on the sheet: `(pixel, µm)` of `color`, kept there
    /// (coverage by volume, as on paper) until the sheet is lifted.
    pub(crate) fn sheet_catch(&mut self, caught: &[(usize, f32)], color: Rgb) {
        let Some(s) = self.sheet.as_mut() else { return };
        for &(i, dv) in caught {
            let q = 1.0 - (-dv / PARTICLE_UM).exp();
            let old = s.a[i] * (1.0 - q);
            let w = old + q;
            s.r[i] = [0, 1, 2].map(|k| ((old * s.r[i][k] + q * color[k]) / w.max(1e-6)).clamp(0.0, 1.0));
            s.a[i] = (s.a[i] + (1.0 - s.a[i]) * q).min(0.995);
        }
    }

    /// The picture as seen with the sheet on it (`seen`'s pixel `i`).
    #[inline]
    pub(crate) fn sheet_seen(&self, i: usize) -> Option<Rgb> {
        let s = self.sheet.as_ref()?;
        if !s.over[i] {
            return None;
        }
        // (paper is matte: its first-surface veil, as a matte surface's)
        Some(crate::canvas::haze(cover(s.tone, s.a[i], s.r[i]), 0.0))
    }

    /// (pixels of the whole canvas) and direction.
    fn walk(&self, pts: &[(f32, f32)]) -> Vec<((f32, f32), (f32, f32))> {
        let s = self.f.scale;
        let p: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
        let path = if p.len() >= 2 { crate::path::densify(&p) } else { vec![p[0], p[0]] };
        let mut arc = vec![0.0f32; path.len()];
        for i in 1..path.len() {
            arc[i] = arc[i - 1] + ((path[i].0 - path[i - 1].0).powi(2) + (path[i].1 - path[i - 1].1).powi(2)).sqrt();
        }
        let total = arc[arc.len() - 1].max(1e-6);
        let n = ((total / 0.7).ceil() as usize).max(1);
        let (mut j, mut last) = (0usize, (1.0f32, 0.0f32));
        let mut out = Vec::with_capacity(n + 1);
        for t in 0..=n {
            let dd = total * t as f32 / n as f32;
            while j + 1 < path.len() - 1 && arc[j + 1] < dd {
                j += 1;
            }
            let (a, b) = (path[j], path[(j + 1).min(path.len() - 1)]);
            let seg = (arc[(j + 1).min(path.len() - 1)] - arc[j]).max(1e-6);
            let fr = ((dd - arc[j]) / seg).clamp(0.0, 1.0);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let m = (dx * dx + dy * dy).sqrt();
            if m > 1e-6 {
                last = (dx / m, dy / m);
            }
            out.push(((a.0 + dx * fr, a.1 + dy * fr), last));
        }
        out
    }

    /// Window pixel at whole-canvas pixel (x, y).
    fn win_px(&self, x: f32, y: f32) -> Option<usize> {
        let f = self.f;
        let (xi, yi) = (x.floor() as isize - f.x0 as isize, y.floor() as isize - f.y0 as isize);
        if xi < 0 || yi < 0 || xi >= f.w as isize || yi >= f.h as isize {
            return None;
        }
        Some(yi as usize * f.w + xi as usize)
    }

    /// Whether the pastel at window pixel `i` is open to a dry tool: drawn,
    /// not painted over, no wet paint, no sheet over it.
    fn open_pastel(&self, d: &crate::graphite::Drawing, i: usize) -> bool {
        let c = &d.cells[i];
        c.film >= 0.0 && self.film[i] <= c.film + 1e-4 && self.wet.vol[i] <= 1e-5 && !self.sheet_over(i)
    }

    /// Take loose pastel off cell `i` down to `keep` µm of it (the fixed stays):
    /// its coverage falls as a deposit's builds (Poisson, PARTICLE_UM).
    /// Returns what came off, µm.
    fn take_loose(c: &mut crate::graphite::Cell, px: &mut Rgb, keep: f32) -> f32 {
        let v = c.loose + c.bound;
        let keep = keep.max(c.bound).min(v);
        let ex = v - keep;
        if ex <= 1e-6 {
            return 0.0;
        }
        let cov = |v: f32| 1.0 - (-v / PARTICLE_UM).exp();
        c.loose -= ex;
        let a1 = (c.a * cov(keep) / cov(v).max(1e-6)).clamp(0.0, c.a);
        let under = uncover(*px, c.a, c.r);
        c.a = a1;
        c.floor = c.floor.min(a1);
        *px = cover(under, c.a, c.r);
        ex
    }

    /// The pastel in the pores of pixel `i` deeper than `z_um` below the paper's
    /// top envelope, µm, of `v` filling the pores (mean depth µ) from the bottom
    /// up: µ·e^(−z/µ), and none heaped above (the pores hold µ).
    fn below(v: f32, mu: f32, z_um: f32) -> f32 {
        v.min(mu).min(mu * (-z_um.max(0.0) / mu).exp())
    }

    /// A blade `width` units long scraped along `pts` (units), held across the
    /// path or at `angle` (radians), pressed `pressure` (start, end; 0..1): a
    /// hand's 1–5 N over the edge's length (pastel_removal_air_blade.md §B4). Its
    /// edge, `edge_um` in radius (a painting knife's: unsharpened, 20–500 µm),
    /// rests on the highest point within the steel's flex and sinks into the
    /// paper by the sheet's measured compression (`edge_sink_um`): it takes
    /// the loose pastel above that depth, heaped or in the pores, pushes part
    /// of it ahead (20–50 %: 35 %, an estimate) to drop where it lifts and
    /// carries the rest off, and leaves the paper it pressed permanently
    /// flatter, its pores shallower (Chen's residual strain, 0.49 ε − 0.027:
    /// burnished). Its drag tears the joints of the fibers it crosses past
    /// their strength, and the torn ends deepen the tooth (nap). An edge fine
    /// enough to cut fiber walls (a scalpel's) shaves about a fiber layer a
    /// pass and the pastel in it, fixed too; otherwise fixed pastel stays (no
    /// wear law of steel on bound pastel is known). Not under a sheet, paint
    /// or wet paint. On paper. Returns the volume taken, mm³.
    pub fn scrape_pastel(&mut self, width: f32, edge_um: f32, pts: &[(f32, f32)], pressure: (f32, f32), angle: Option<f32>) -> f32 {
        let Some(paper) = self.paper else { return 0.0 };
        if pts.is_empty() || self.drawing.is_none() || !(width.is_finite() && width >= 1.0) {
            return 0.0;
        }
        let s_mm = self.px_mm();
        let px_um2 = (s_mm * 1000.0).powi(2);
        let t_um = paper.caliper_um();
        let half = width * self.f.scale * 0.5;
        let reach = ((FLEX_MM / (s_mm * 0.7)).ceil() as usize).max(1);
        let steps = self.walk(pts);
        let n = steps.len().max(2) - 1;
        let mut dr = self.drawing.take().unwrap();
        let mut pxs = std::mem::take(&mut self.px);
        let mut heights = std::mem::take(&mut self.height);
        let (mut taken, mut pushed) = (0.0f32, 0.0f32);
        let mut col = [0.0f32; 3];
        let mut last_band: Vec<usize> = Vec::new();
        let mut dent: Vec<(usize, f32)> = Vec::new();
        let mut napped: Vec<(usize, f32)> = Vec::new();
        let mut sinks: Vec<(i32, f32)> = Vec::new();
        for (t, &(c, dir)) in steps.iter().enumerate() {
            let e = match angle {
                Some(a) => (a.cos(), a.sin()),
                None => (-dir.1, dir.0),
            };
            let pr = pressure.0 + (pressure.1 - pressure.0) * t as f32 / n as f32;
            // a hand's 1–5 N over the length of edge bearing [E]: a palette
            // knife's 3 cm light, a scalpel's few mm heavy
            let q = (1.0 + 4.0 * pr.clamp(0.0, 1.0)) / (width * self.mm_per_unit).max(0.5);
            let qk = (q * 50.0).round() as i32;
            let sink = match sinks.iter().find(|s| s.0 == qk) {
                Some(s) => s.1,
                None => {
                    let v = edge_sink_um(qk as f32 / 50.0, edge_um.max(1.0), t_um);
                    sinks.push((qk, v));
                    v
                }
            };
            // the edge cuts fiber walls past q = H·2R (a scalpel's 1 µm edge
            // at 0.15–0.85 N/mm; a knife's never at a hand's load), shaving
            // about a fiber layer
            let shave = if q * 1000.0 > FIBER_WALL_MPA * 2.0 * edge_um { SHAVE_UM } else { 0.0 };
            // its drag on each fiber it crosses tears the fiber's joints past
            // their strength: the torn ends stand proud (nap)
            let drag = STEEL_PAPER_MU * q * paper.fiber_um * 1e-3;
            let nap = ((drag - FIBER_JOINT_N) / FIBER_JOINT_N).clamp(0.0, 1.0);
            let nb = ((2.0 * half / 0.7).ceil() as usize).max(2);
            let mut blade: Vec<usize> = Vec::with_capacity(nb + 1);
            for k in 0..=nb {
                let u = -half + 2.0 * half * k as f32 / nb as f32;
                if let Some(i) = self.win_px(c.0 + e.0 * u, c.1 + e.1 * u) {
                    if blade.last() != Some(&i) {
                        blade.push(i);
                    }
                }
            }
            if blade.is_empty() {
                continue;
            }
            let hs: Vec<f32> = blade.iter().map(|&i| heights[i]).collect();
            let rest: Vec<f32> = (0..hs.len()).map(|k| hs[k.saturating_sub(reach)..(k + reach + 1).min(hs.len())].iter().cloned().fold(f32::MIN, f32::max)).collect();
            for (k, &i) in blade.iter().enumerate() {
                if self.sheet_over(i) || self.wet.vol[i] > 1e-5 {
                    continue;
                }
                // how far below this point's top the edge reaches
                let z = heights[i] - (rest[k] - sink);
                if z <= 0.0 {
                    continue;
                }
                let cl = &mut dr.cells[i];
                if cl.film >= 0.0 && self.film[i] <= cl.film + 1e-4 {
                    let mu = self.micro_um(i);
                    let v = cl.loose + cl.bound;
                    let keep = Self::below(v, mu, z + shave);
                    let r = cl.r;
                    if shave > 0.0 {
                        // cut fibers take what is bound to them too: freed,
                        // it comes off with the loose (its coverage with it)
                        let cut = (cl.bound - keep).max(0.0);
                        cl.bound -= cut;
                        cl.loose += cut;
                    }
                    let ex = Self::take_loose(cl, &mut pxs[i], keep);
                    if ex > 0.0 {
                        let tot = pushed + ex * 0.35;
                        col = [0, 1, 2].map(|j| (col[j] * pushed + r[j] * ex * 0.35) / tot.max(1e-9));
                        pushed = tot;
                        taken += ex;
                    }
                }
                // pressed in by z: what stays of it after the edge has passed
                let eps = z / t_um;
                let resid = ((0.49 * eps - 0.027).max(0.0) * t_um).min(z) + shave;
                if resid > 0.0 {
                    dent.push((i, resid));
                }
                if nap > 0.0 {
                    napped.push((i, nap));
                }
            }
            last_band = blade;
        }
        // burnished: the peaks pressed down, their pores shallower (the voids
        // closed by what the dent took), once per pixel, by the deepest press
        dent.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.partial_cmp(&a.1).unwrap()));
        dent.dedup_by_key(|d| d.0);
        for &(i, r) in &dent {
            heights[i] -= r;
            if let Some(m) = self.micro.get_mut(i) {
                *m = (*m - r).max(0.5);
            }
        }
        // nap: the torn ends deepen the tooth there (once per pixel, by the most)
        napped.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.partial_cmp(&a.1).unwrap()));
        napped.dedup_by_key(|d| d.0);
        for &(i, f) in &napped {
            if let Some(m) = self.micro.get_mut(i) {
                *m *= 1.0 + (NAP_TOOTH - 1.0) * f;
            }
        }
        self.height = heights;
        if !dent.is_empty() {
            self.surf_gen += 1;
        }
        // what was pushed ahead drops where the blade lifts
        // (on open paper, not the sheet; crumbs on wet paint are lost in it)
        let bed: Vec<(usize, f32, bool)> = last_band.iter().filter(|&&i| !self.sheet_over(i)).map(|&i| (i, 1.0, self.wet.vol[i] > 1e-5)).collect();
        if pushed > 0.0 && !bed.is_empty() {
            let film = std::mem::take(&mut self.film);
            let mut rng = crate::rng::Rng::new((pts[0].0.to_bits() as u64) << 32 ^ pts[0].1.to_bits() as u64 ^ 0x5C2A);
            crate::pastel::lay_crumbs(&mut dr, &film, &mut pxs, &bed, pushed, col, &mut rng, CRUMB_MAX_UM, px_um2);
            self.film = film;
        }
        self.drawing = Some(dr);
        self.px = pxs;
        taken * px_um2 * 1e-9
    }

    /// A dry brush drawn over pastel along `pts` (units), its tips touching a
    /// band `width` units wide across the path, each tip `tip_um` µm in
    /// radius, its bristles already holding `held_mg` of pastel of `cap_mg`
    /// they can. Loose pastel within the tips' reach comes away; what lies
    /// deeper in the pores, and fixed pastel, stays. The bristles keep what
    /// they lift as long as they have room (the less room, the less they
    /// take: the brush fills); the rest is pushed ahead into the pores
    /// downstream, met again, and left at the lift. Not under a sheet, paint
    /// or wet paint. Returns (the volume taken off the picture into the
    /// bristles, mm³, hand time s): what is pushed ahead and lifted again is
    /// counted once, where it ends.
    pub fn dust_pastel(&mut self, width: f32, tip_um: f32, pts: &[(f32, f32)], held_mg: &mut f32, cap_mg: f32) -> (f32, f32) {
        if pts.is_empty() || self.drawing.is_none() || !(width.is_finite() && width > 0.0) {
            return (0.0, 0.0);
        }
        let f = self.f;
        let s = f.scale;
        let s_mm = self.px_mm();
        let px_um2 = (s_mm * 1000.0).powi(2);
        // the gaps between fibers: about a fiber's width
        let w_um = self.paper.as_ref().map_or(30.0, |p| p.fiber_um);
        let r = tip_um.max(1.0);
        let reach_um = if w_um / 2.0 >= r { f32::INFINITY } else { r - (r * r - w_um * w_um / 4.0).sqrt() };
        let half = width * s * 0.5;
        // the tips' band along the path: a bristle tuft pressed on paper, about 2 mm deep
        let depth_px = (2.0 / s_mm).max(1.0);
        let steps = self.walk(pts);
        let total_px = (steps.len().max(1) - 1) as f32 * 0.7;
        let mut kept_um = 0.0f32;
        let (mut ahead, mut col) = (0.0f32, [0.0f32; 3]);
        let mut dr = self.drawing.take().unwrap();
        let mut pxs = std::mem::take(&mut self.px);
        let mut rng = crate::rng::Rng::new((pts[0].0.to_bits() as u64) << 32 ^ pts[0].1.to_bits() as u64 ^ 0xD057);
        let mut last_band: Vec<usize> = Vec::new();
        let um_to_mg = px_um2 * 1e-9 * PASTEL_MG_MM3;
        for &(c, dir) in &steps {
            let e = (-dir.1, dir.0);
            let (mut band, mut front) = (Vec::new(), Vec::new());
            let nb = ((2.0 * half / 0.7).ceil() as usize).max(1);
            for k in 0..=nb {
                let u = -half + 2.0 * half * k as f32 / nb as f32;
                for back in 0..(depth_px.ceil() as usize) {
                    if let Some(i) = self.win_px(c.0 + e.0 * u - dir.0 * back as f32, c.1 + e.1 * u - dir.1 * back as f32) {
                        band.push(i);
                    }
                }
                // pushed ahead: into the pores within 0.1–1 mm of the tips (0.5 mm)
                if let Some(i) = self.win_px(c.0 + e.0 * u + dir.0 * 0.5 / s_mm, c.1 + e.1 * u + dir.1 * 0.5 / s_mm) {
                    front.push(i);
                }
            }
            band.sort_unstable();
            band.dedup();
            front.sort_unstable();
            front.dedup();
            band.retain(|&i| self.open_pastel(&dr, i) || (dr.cells[i].film < 0.0 && !self.sheet_over(i) && self.wet.vol[i] <= 1e-5));
            front.retain(|&i| !self.sheet_over(i) && self.wet.vol[i] <= 1e-5);
            for &i in &band {
                if !self.open_pastel(&dr, i) {
                    continue;
                }
                let mu = self.micro_um(i);
                let cl = &mut dr.cells[i];
                let r = cl.r;
                let keep = Self::below(cl.loose + cl.bound, mu, reach_um);
                let ex = Self::take_loose(cl, &mut pxs[i], keep);
                if ex <= 0.0 {
                    continue;
                }
                // the bristles keep what they have room for
                let room = (1.0 - *held_mg / cap_mg.max(1e-9)).clamp(0.0, 1.0);
                let kept = ex * room;
                *held_mg += kept * um_to_mg;
                kept_um += kept;
                let tot = ahead + (ex - kept);
                col = [0, 1, 2].map(|j| (col[j] * ahead + r[j] * (ex - kept)) / tot.max(1e-9));
                ahead = tot;
            }
            if !front.is_empty() && ahead > 0.0 {
                let hmax = front.iter().map(|&i| self.height[i]).fold(f32::MIN, f32::max);
                let bed: Vec<(usize, f32, bool)> = front.iter().map(|&i| (i, 1.0 + (hmax - self.height[i]), false)).collect();
                let film = std::mem::take(&mut self.film);
                crate::pastel::lay_crumbs(&mut dr, &film, &mut pxs, &bed, ahead, col, &mut rng, BRUSH_CRUMB_UM, px_um2);
                self.film = film;
                ahead = 0.0;
            }
            if !band.is_empty() {
                last_band = band;
            }
        }
        // what is still pushed ahead drops where the brush lifts (on open
        // paper, not the sheet; crumbs on wet paint are lost in it)
        let bed: Vec<(usize, f32, bool)> = last_band.iter().filter(|&&i| !self.sheet_over(i)).map(|&i| (i, 1.0, self.wet.vol[i] > 1e-5)).collect();
        if ahead > 0.0 && !bed.is_empty() {
            let film = std::mem::take(&mut self.film);
            crate::pastel::lay_crumbs(&mut dr, &film, &mut pxs, &bed, ahead, col, &mut rng, BRUSH_CRUMB_UM, px_um2);
            self.film = film;
        }
        self.drawing = Some(dr);
        self.px = pxs;
        // hand time: brushing at about 100 mm/s
        (kept_um * px_um2 * 1e-9, total_px / s * self.mm_per_unit / 100.0 + 0.3)
    }

    /// A puff of air from `h_mm` above the picture at (x, y) (units), leaving
    /// the mouth (or a bulb's nozzle, `d_mm` across) at `u` m/s (blowing hard:
    /// 12 m/s on average, 6–64 measured). The jet's wall shear peaks on a ring
    /// at 0.09 h (Phares et al. 2000: τ = 44.6 ρU²Re^(−½)(h/d)^(−2), h/d at
    /// least 6) and falls off outside it as a radial wall jet's, ∝ r^(−2.3)
    /// (Poreh et al. 1967), rising from nothing at the center. Loose pastel
    /// heaped on the tooth goes where the shear passes the threshold of its
    /// crumbs (Shao & Lu 2000, ~0.2 m/s); in the pores the flow dies with
    /// depth as exp(−4.21 z/w) (Moffatt's eddies, w a fiber's width), so only
    /// the top few µm of the pores' filling, its fine grains (~0.8 m/s), can
    /// go. Where it goes (paper_mechanics_and_transport.md §4): most crumbs
    /// roll out to where the shear no longer keeps each moving, a broad ring;
    /// some hop, landing further out or rolling on; the fines ride the wall
    /// jet and settle from it by their size, most leaving as a cloud. Fixed
    /// pastel stays. Returns the volume blown off the picture, mm³ (what lands
    /// on the paper mask goes with it; what falls on wet paint is lost in it,

    /// A stick held at `pose` with its tip at (x, y) (units), over the window's
    /// pixels: 2 where it touches, 1 where its face lies within a crumb's size
    /// of the surface (where its crumbs settle), 0 elsewhere under its low
    /// part. Reads only (the hold look).
    pub fn stick_contact(&self, stick: &crate::pastel::Stick, x: f32, y: f32, pose: crate::pastel::Pose) -> Vec<(usize, u8)> {
        let sp = crate::pastel::StrokePoint { x, y, pose, speed: 50.0 };
        let Some(st) = self.seat_stick(stick, &sp) else { return Vec::new() };
        let mut out = Vec::new();
        for (j, &(i, b)) in st.under.iter().enumerate() {
            let (hs, mu, k) = st.surf[j];
            let o = crate::pastel::seat_overlap(hs - b * 1000.0 + st.delta, mu, k, stick.hardness);
            if o > 0.0 {
                out.push((i, 2));
            } else if b * 1000.0 - st.delta - hs < crate::pastel::CRUMB_UM {
                out.push((i, 1));
            } else {
                out.push((i, 0));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::BRUSH_HOLDS_MG_MM3;
    use crate::canvas::{Canvas, Crop};
    use crate::graphite::{Lead, hand_line};
    use crate::mask::Mask;
    use crate::paper::Paper;
    use crate::pastel::{Pose, Stick, StrokePoint};

    fn drawing_paper() -> Canvas {
        Canvas::new_window(600, 1.0, [0.55; 3], None).with_size_mm(150.0).with_paper(Paper::drawing(4))
    }

    fn pose(force: f32, alt_deg: f32) -> Pose {
        Pose { force, alt: alt_deg.to_radians(), az: 45f32.to_radians(), roll: 0.0 }
    }

    fn band(c: &mut Canvas, st: &mut Stick, y0: f32, y1: f32, force: f32) {
        let mut y = y0;
        while y < y1 {
            let pts = [StrokePoint { x: 100.0, y, pose: pose(force, 50.0), speed: 50.0 }, StrokePoint { x: 900.0, y, pose: pose(force, 50.0), speed: 50.0 }];
            c.stick_stroke(st, &pts);
            y += 15.0;
        }
    }

    fn dark(c: &Canvas, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
        let f = c.window();
        let (mut s, mut n) = (0.0, 0);
        for (i, p) in c.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x >= x0 && x < x1 && y >= y0 && y < y1 {
                s += 0.55 - p[0];
                n += 1;
            }
        }
        s / n.max(1) as f32
    }

    fn left_half(c: &Canvas) -> Mask {
        let f = c.window();
        let mut m = Mask::empty(f);
        for y in 0..f.full_h {
            for x in 0..f.full_w / 2 {
                m.data[y * f.full_w + x] = 1.0;
            }
        }
        m
    }

    fn graphite_and_sheet_mask(crop: Option<Crop>) -> (Canvas, Mask) {
        let mut c = Canvas::new_window(160, 1.0, [0.8; 3], crop).with_size_mm(150.0);
        let line = hand_line(&[(100.0, 500.0), (900.0, 500.0)], &[0.6], false, true, 0.0, 5);
        c.draw(&Lead::pencil("2B").unwrap(), &line, 0.0, 9);
        let window = c.window();
        let covered = Mask::from_fn(c.frame(), |x, y| if x < 500.0 && window.holds(x, y) { 1.0 } else { 0.0 });
        let guide = c.drawing_guide();
        for covered_value in [0.0, 1.0] {
            assert!(guide.data.iter().zip(&covered.data).any(|(&g, &m)| g > 0.0 && m == covered_value));
        }
        (c, covered)
    }

    #[test]
    fn a_sheet_protects_the_drawing_guide_from_erasing() {
        for crop in [None, Some(Crop { units: [200.0, 200.0, 800.0, 800.0], margin: 0.0 })] {
            let (mut c, covered) = graphite_and_sheet_mask(crop);
            let all = Mask::full(c.frame());
            let mut expected = c.clone();
            expected.erase(&covered.clone().invert(), 1.0);
            c.lay_sheet(&covered, 150.0, 8.0, [0.85; 3]).unwrap();
            c.erase(&all, 1.0);
            assert_eq!(c.drawing_guide().data, expected.drawing_guide().data);
            assert_eq!(c.pixels(), expected.pixels());
            assert!(c.lift_sheet());
            c.erase(&all, 1.0);
            expected.erase(&all, 1.0);
            assert_eq!(c.drawing_guide().data, expected.drawing_guide().data);
            assert_eq!(c.pixels(), expected.pixels());
        }
    }

    #[test]
    fn a_sheet_keeps_fixative_off_the_drawing_guide() {
        for crop in [None, Some(Crop { units: [200.0, 200.0, 800.0, 800.0], margin: 0.0 })] {
            let (base, covered) = graphite_and_sheet_mask(crop);
            let all = Mask::full(base.frame());
            for mask in [None, Some(Mask::from_fn(base.frame(), |x, _| if x < 650.0 { 1.0 } else { 0.0 }))] {
                let mut expected = base.clone();
                let open = mask.clone().unwrap_or_else(|| all.clone()).mul(&covered.clone().invert());
                expected.fix_drawing(Some(&open));
                let mut c = base.clone();
                c.lay_sheet(&covered, 150.0, 8.0, [0.85; 3]).unwrap();
                c.fix_drawing(mask.as_ref());
                assert_eq!(
                    c.drawing.as_ref().unwrap().cells.iter().map(|c| c.floor).collect::<Vec<_>>(),
                    expected.drawing.as_ref().unwrap().cells.iter().map(|c| c.floor).collect::<Vec<_>>()
                );
                assert!(c.lift_sheet());
                c.erase(&all, 1.0);
                expected.erase(&all, 1.0);
                assert_eq!(c.drawing_guide().data, expected.drawing_guide().data);
                assert_eq!(c.pixels(), expected.pixels());
            }
        }
    }

    /// A sheet over the left half: strokes across it leave the picture under
    /// it as it was, and the sheet keeps what fell on it; lifted, it is gone.
    #[test]
    fn a_sheet_keeps_what_it_covers_clean() {
        let mut c = drawing_paper();
        let before = c.pixels().to_vec();
        let m = left_half(&c);
        c.lay_sheet(&m, 150.0, 8.0, [0.85; 3]).unwrap();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        band(&mut c, &mut st, 300.0, 700.0, 2.0);
        let f = c.window();
        let under_clean = c.pixels().iter().enumerate().filter(|(i, _)| f.ux(i % f.w) < 450.0).all(|(i, p)| *p == before[i]);
        assert!(under_clean, "nothing reached the picture under the sheet");
        assert!(dark(&c, 600.0, 850.0, 320.0, 680.0) > 0.05, "the open half took pastel");
        let caught = c.sheet.as_ref().unwrap().a.iter().cloned().fold(0.0, f32::max);
        assert!(caught > 0.3, "the sheet caught pastel: {caught}");
        assert!(c.lift_sheet() && !c.has_sheet());
    }

    /// Dense crossing passes: the pores full, the tooth packed.
    fn dense(c: &mut Canvas, st: &mut Stick, y0: f32, y1: f32) {
        for pass in 0..3 {
            let az = (60.0 + 25.0 * pass as f32).to_radians();
            let mut x = 100.0;
            while x < 900.0 {
                let p = Pose { force: 3.0, alt: 50f32.to_radians(), az, roll: 0.0 };
                c.stick_stroke(st, &[StrokePoint { x, y: y0, pose: p, speed: 50.0 }, StrokePoint { x: x + 30.0, y: y1, pose: p, speed: 50.0 }]);
                x += 8.0;
            }
        }
    }

    /// A dry brush over a dense passage lifts what its tips reach and leaves
    /// a ghost; over fixed pastel it lifts nothing; a finer tip reaches deeper.
    #[test]
    #[ignore = "slow (about 2 minutes: hundreds of stick strokes first); run with cargo test -p paint --lib -- --ignored sheet::tests::a_dry_brush_lifts_what_its_tips_reach"]
    fn a_dry_brush_lifts_what_its_tips_reach() {
        let mut c = drawing_paper();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        dense(&mut c, &mut st, 200.0, 400.0);
        dense(&mut c, &mut st, 600.0, 800.0);
        let f = c.window();
        let mut low = Mask::empty(f);
        for y in (f.full_h * 55 / 100)..f.full_h {
            for x in 0..f.full_w {
                low.data[y * f.full_w + x] = 1.0;
            }
        }
        c.fix_pastel(Some(&low), 0.15);
        let (loose0, fixed0) = (dark(&c, 300.0, 700.0, 230.0, 370.0), dark(&c, 300.0, 700.0, 630.0, 770.0));
        let mut fine = c.clone();
        // a 40-unit brush on this 150 mm sheet: 6 mm wide, its loaded tip 6 × 2 × 1.8 mm
        let cap = BRUSH_HOLDS_MG_MM3 * 6.0 * 2.0 * 1.8;
        let (mut held, mut held_fine, mut held_fixed) = (0.0f32, 0.0f32, 0.0f32);
        let bare_before = dark(&c, 965.0, 995.0, 230.0, 370.0);
        for y in (200..=400).step_by(10) {
            // (out past the passage's end onto bare paper, where the brush lifts)
            c.dust_pastel(40.0, 50.0, &[(120.0, y as f32), (975.0, y as f32)], &mut held, cap);
            fine.dust_pastel(40.0, 10.0, &[(120.0, y as f32), (975.0, y as f32)], &mut held_fine, cap);
        }
        for y in (600..=800).step_by(10) {
            c.dust_pastel(40.0, 50.0, &[(120.0, y as f32), (880.0, y as f32)], &mut held_fixed, cap);
        }
        assert!(held > 0.9 * cap, "the brush filled: {held} of {cap} mg");
        assert!(held_fixed == 0.0, "fixed pastel gave the brush nothing");
        let (loose1, fixed1, fine1) = (dark(&c, 300.0, 700.0, 230.0, 370.0), dark(&c, 300.0, 700.0, 630.0, 770.0), dark(&fine, 300.0, 700.0, 230.0, 370.0));
        assert!(loose1 < loose0 * 0.97, "the brush lifted some: {loose0} -> {loose1}");
        assert!(loose1 > loose0 * 0.2, "and left a ghost: {loose1}");
        assert!((fixed1 - fixed0).abs() < 0.02 * fixed0.max(1e-3), "fixed pastel stays: {fixed0} -> {fixed1}");
        assert!(fine1 < loose1, "a finer tip reaches deeper: {fine1} vs {loose1}");
        // what it lifted went along with it, to where it lifted
        let bare_after = dark(&c, 965.0, 995.0, 230.0, 370.0);
        assert!(bare_after > bare_before + 0.01, "a lip where the brush lifts, on bare paper past the passage: {bare_before} -> {bare_after}");
    }

    /// Over a speckled passage (bare tooth between the marks) the brush pushes
    /// what it lifts into the empty pores, below its reach: it smears, the
    /// passage evens out rather than cleans (why pastels are not brushed).
    #[test]
    fn a_dry_brush_smears_a_speckled_passage() {
        let mut c = drawing_paper();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        band(&mut c, &mut st, 200.0, 400.0, 1.0);
        let before = dark(&c, 300.0, 700.0, 230.0, 370.0);
        let mut held = 0.0;
        for y in (200..=400).step_by(10) {
            c.dust_pastel(40.0, 50.0, &[(120.0, y as f32), (880.0, y as f32)], &mut held, BRUSH_HOLDS_MG_MM3 * 21.6);
        }
        let after = dark(&c, 300.0, 700.0, 230.0, 370.0);
        assert!(after > 0.8 * before, "a speckled passage isn't cleaned by brushing: {before} -> {after}");
    }

    /// A knife pressed into a dense, loose passage takes pastel down into the
    /// tooth (the edge sinks deeper than the pores) and burnishes the paper:
    /// the peaks it pressed stay lower, their pores shallower. Fixed pastel stays.
    #[test]
    #[ignore = "slow (about 2 minutes: hundreds of stick strokes first); run with cargo test -p paint --lib -- --ignored sheet::tests::a_knife_takes_the_loose_pastel_and_burnishes_the_paper"]
    fn a_knife_takes_the_loose_pastel_and_burnishes_the_paper() {
        let mut c = drawing_paper();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        dense(&mut c, &mut st, 200.0, 400.0);
        dense(&mut c, &mut st, 600.0, 800.0);
        let f = c.window();
        let mut low = Mask::empty(f);
        for y in (f.full_h * 55 / 100)..f.full_h {
            for x in 0..f.full_w {
                low.data[y * f.full_w + x] = 1.0;
            }
        }
        c.fix_pastel(Some(&low), 0.15);
        let (d0, f0) = (dark(&c, 450.0, 550.0, 230.0, 370.0), dark(&c, 450.0, 550.0, 630.0, 770.0));
        let (h0, m0) = (c.height.clone(), c.micro.clone());
        let took = c.scrape_pastel(100.0, 100.0, &[(500.0, 180.0), (500.0, 820.0)], (1.0, 1.0), Some(0.0));
        let (d1, f1) = (dark(&c, 470.0, 530.0, 230.0, 370.0), dark(&c, 470.0, 530.0, 630.0, 770.0));
        // (a stiff blade rides the paper's high spots and bridges its low ones:
        // the scrape is mottled, so the passage lightens by part)
        assert!(took > 0.0 && d1 < 0.95 * d0, "the knife took loose pastel: {d0} -> {d1}");
        assert!((f1 - f0).abs() < 0.05 * f0.max(1e-3), "the fixed stays: {f0} -> {f1}");
        // a wide knife at a hand's force presses too lightly to set the paper
        // (Chen's residual strain is 0 below ε ≈ 0.055); the same hand on a 3 mm
        // edge burnishes it: the peaks stay lower, their pores shallower
        let pressed = |c: &Canvas| (0..h0.len()).filter(|&i| c.height[i] < h0[i] - 0.5).count();
        assert_eq!(pressed(&c), 0, "the light knife left no dent");
        c.scrape_pastel(20.0, 100.0, &[(300.0, 180.0), (300.0, 420.0)], (1.0, 1.0), Some(0.0));
        let shallower = (0..m0.len()).filter(|&i| c.micro[i] < m0[i] - 0.5).count();
        assert!(pressed(&c) > 0 && shallower > 0, "pressed hard on a short edge, it burnished: {} pixels pressed, {shallower} shallower", pressed(&c));
    }

    /// The edge's dent with the surface's shear: shallower than the bed of
    /// springs at a knife's light load (research: 10.8–15.6 µm at 0.2 N/mm,
    /// R 100 µm, T 250 µm, against Winkler's 29), deeper with load.
    #[test]
    fn an_edge_dents_paper_less_than_a_bed_of_springs() {
        let d = super::edge_sink_um(0.2, 100.0, 250.0);
        assert!(d > 9.0 && d < 18.0, "dent at 0.2 N/mm: {d} µm");
        assert!(super::edge_sink_um(0.5, 100.0, 250.0) > d, "deeper with load");
    }

    /// A scalpel (a 1 µm edge on a short length) cuts: it shaves the surface
    /// and takes even fixed pastel; a palette knife at the same hand doesn't.
    #[test]
    #[ignore = "slow (about 2 minutes: hundreds of stick strokes first); run with cargo test -p paint --lib -- --ignored sheet::tests::a_scalpel_cuts_a_knife_does_not"]
    fn a_scalpel_cuts_a_knife_does_not() {
        let mut c = drawing_paper();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        dense(&mut c, &mut st, 300.0, 700.0);
        c.fix_pastel(None, 0.15);
        let mut k = c.clone();
        let (h0, b0) = (c.height.clone(), c.drawing.as_ref().unwrap().cells.iter().map(|c| c.bound).sum::<f32>());
        c.scrape_pastel(10.0, 1.0, &[(500.0, 280.0), (500.0, 720.0)], (0.5, 0.5), Some(0.0));
        k.scrape_pastel(200.0, 100.0, &[(500.0, 280.0), (500.0, 720.0)], (0.5, 0.5), Some(0.0));
        let shaved = (0..h0.len()).filter(|&i| c.height[i] < h0[i] - 10.0).count();
        assert!(shaved > 0, "the scalpel shaved the surface");
        assert!(c.drawing.as_ref().unwrap().cells.iter().map(|c| c.bound).sum::<f32>() < b0, "and took fixed pastel");
        assert!((k.drawing.as_ref().unwrap().cells.iter().map(|c| c.bound).sum::<f32>() - b0).abs() < 1e-3 * b0, "the knife left the fixed");
    }

    /// The hold's contact is where a stroke from there lays.
    #[test]
    fn the_hold_shows_where_a_stroke_lays() {
        let c = drawing_paper();
        let st = Stick::round([0.02; 3], 0.8, 12.0);
        let cells = c.stick_contact(&st, 500.0, 500.0, pose(2.0, 60.0));
        let touch: Vec<usize> = cells.iter().filter(|c| c.1 == 2).map(|c| c.0).collect();
        assert!(!touch.is_empty(), "it touches somewhere");
        let mut d = c.clone();
        let mut s2 = st.clone();
        let p = pose(2.0, 60.0);
        // a stroke through there, slow, so the hand is down at full force at
        // its middle (it lands over 50 ms and lifts over 40)
        d.stick_stroke(&mut s2, &[StrokePoint { x: 460.0, y: 500.0, pose: p, speed: 20.0 }, StrokePoint { x: 540.0, y: 500.0, pose: p, speed: 20.0 }]);
        let laid: Vec<usize> = (0..d.pixels().len()).filter(|&i| d.pixels()[i] != c.pixels()[i]).collect();
        let hit = touch.iter().filter(|i| laid.contains(i)).count();
        assert!(hit * 2 >= touch.len(), "most touching pixels took pastel: {hit} of {}", touch.len());
    }
}
