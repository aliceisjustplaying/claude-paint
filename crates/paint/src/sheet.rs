//! Engine 7: a paper mask, and a blade scraping pastel.
//!
//! **The sheet.** A sheet of paper laid over part of the picture to keep it
//! clean, as pastellists do. It is a surface of its own lying `caliper` µm
//! above the picture, with a paper's pores: a pastel stick rests on it as on
//! any surface (the contact solver sees it as raised ground), so near its
//! edge the stick bridges from sheet to picture and the edge it leaves is as
//! sharp as the stick's angle and force make it. What lands on the sheet
//! (strokes, crumbs) stays on the sheet and leaves with it; the finger, the
//! eraser and fixative don't reach under it. Wet-paint tools are not
//! modelled under a sheet (the easel refuses them while one is down).
//!
//! **The blade on pastel** (notes/research/pastel_removal_air_blade.md §B).
//! A painting knife's unsharpened edge pressed with a hand's load sinks into
//! the sheet by its measured compression (15–40 µm, an upper bound: deeper
//! than the pores), so it takes the loose pastel down into the tooth, and
//! leaves the paper it pressed permanently flatter: burnished, the tooth
//! shallower. Fixed pastel stays (no wear law of steel on it is known).
//!
//! **A dry brush over pastel** (pastel_brushing.md). A bristle tip pushes
//! sideways with 10³–10⁶ times the force that rolls a pastel particle off,
//! so brushing is limited by reach: the tips (radius r, over gaps between
//! fibres of width w) sink h = r − √(r² − w²/4), and the pastel filling the
//! pores deeper than that, µ·e^(−h/µ), stays (the ghost), with the fixed.
//! The bristles keep what they lift until they are full (a fur brush's pile
//! holds 5–7 % of its volume in toner: a #8 hog some 2–10 mg); the rest is
//! pushed ahead into the pores downstream, met again, and left where the
//! brush lifts.
//!
//! **Blowing and tapping** (pastel_removal_air_blade.md §A). A puff's jet
//! shears the surface (measured impinging-jet law); loose crumbs heaped on
//! the tooth go past their measured threshold, the fine grains in the pores
//! only as deep as the flow reaches into them (it dies within a pore width).
//! A tap shakes off the crumbs heavy enough to beat their adhesion; grains
//! held in the pores never go.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::graphite::{cover, uncover};
use crate::mask::Mask;
use crate::pastel::PARTICLE_UM;

/// A sheet of paper laid on the picture (engine 7).
#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    /// Where it lies, per pixel of the window.
    pub(crate) over: Vec<bool>,
    /// Its thickness, µm.
    pub(crate) caliper_um: f32,
    /// Its pores' mean depth, µm.
    pub(crate) micro_um: f32,
    /// Its colour (linear RGB).
    pub(crate) tone: Rgb,
    /// The pastel caught on it, for the look: coverage and colour.
    pub(crate) a: Vec<f32>,
    pub(crate) r: Vec<Rgb>,
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
/// Adhesion of a loose pastel grain or crumb to what it lies on, N: 10–150
/// nN per contact, about the same for every size [M, D] (pastel_brushing.md §1).
const ADHESION_N: f32 = 5e-8;
/// A crumb's density, kg/m³ (pigment and chalk ~2500, crumbs about half
/// porous: pastel_removal_air_blade.md §A2).
const CRUMB_KG_M3: f32 = 1250.0;
/// The largest loose crumb lying on the tooth, µm (crumbs from soft pastel
/// run to 100–300 µm: pastel_stick_tribology.md §3).
const CRUMB_MAX_UM: f32 = 300.0;
/// Air: density kg/m³ and kinematic viscosity m²/s.
const AIR_KG_M3: f32 = 1.2;
const AIR_NU: f32 = 1.5e-5;

/// The steel flexes over broad relief and bridges fine hollows: the blade
/// rests on the highest point within this many mm along it (as `Canvas::knife`).
const FLEX_MM: f32 = 4.0;

/// Shao & Lu (2000): the friction velocity that sets loose grains of `d_um`
/// and density `rho` (kg/m³) moving on a bed, m/s (A_N = 0.0123, γ = 3e-4 N/m).
fn shao_lu(d_um: f32, rho: f32) -> f32 {
    let d = d_um * 1e-6;
    (0.0123 * (rho / AIR_KG_M3 * 9.81 * d + 3e-4 / (AIR_KG_M3 * d))).sqrt()
}

/// Paper under a cylindrical edge of radius `r_um` pressed with line load
/// `q` N/mm, the sheet `t_um` thick on a rigid board, as a bed of springs
/// with Chen et al.'s (2020) compression curve σ = 0.636(e^(13.54 ε) − 1)
/// MPa: how deep the edge sinks below the surface it rests on, µm.
/// (Ignoring the fibres' in-plane stiffness: an upper bound.)
pub fn edge_sink_um(q: f32, r_um: f32, t_um: f32) -> f32 {
    let line = |d0: f32| -> f32 {
        // q = ∫ σ(δ(x)/T) dx, δ(x) = δ0 − x²/2R, over the contact (µm → mm: MPa·µm = 1e-3 N/mm)
        let half = (2.0 * r_um * d0).sqrt();
        let n = 64;
        let mut sum = 0.0;
        for k in 0..n {
            let x = -half + 2.0 * half * (k as f32 + 0.5) / n as f32;
            let d = (d0 - x * x / (2.0 * r_um)).max(0.0);
            sum += 0.636 * ((13.54 * d / t_um).exp() - 1.0) * (2.0 * half / n as f32);
        }
        sum * 1e-3
    };
    let (mut lo, mut hi) = (0.0f32, t_um * 0.6);
    for _ in 0..40 {
        let m = 0.5 * (lo + hi);
        if line(m) < q { lo = m } else { hi = m }
    }
    0.5 * (lo + hi)
}

impl Canvas {
    /// Lay a sheet over `m` (where it is above 0.5): `caliper_um` thick, its
    /// pores `micro_um` deep, of colour `tone`. Engine 7; one sheet at a time.
    pub fn lay_sheet(&mut self, m: &Mask, caliper_um: f32, micro_um: f32, tone: Rgb) -> Result<(), String> {
        if self.engine < 7 {
            return Err(format!("a paper mask needs engine 7; this painting is painted with engine {}", self.engine));
        }
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

    /// The path `pts` (units) walked in steps of 0.7 pixel: each step's centre
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
    /// hand's 0.05–0.5 N/mm along it (pastel_removal_air_blade.md §B4). Its
    /// edge, `edge_um` in radius (a painting knife's: unsharpened, 20–500 µm),
    /// rests on the highest point within the steel's flex and sinks into the
    /// paper by the sheet's measured compression (`edge_sink_um`): it takes
    /// the loose pastel above that depth, heaped or in the pores, pushes part
    /// of it ahead (20–50 %: 35 %, an estimate) to drop where it lifts and
    /// carries the rest off, and leaves the paper it pressed permanently
    /// flatter, its pores shallower (Chen's residual strain, 0.49 ε − 0.027:
    /// burnished). Fixed pastel stays (no wear law of steel on bound pastel
    /// is known). Not under a sheet, paint or wet paint. Engine 7; on paper.
    /// Returns the volume taken, mm³.
    pub fn scrape_pastel(&mut self, width: f32, edge_um: f32, pts: &[(f32, f32)], pressure: (f32, f32), angle: Option<f32>) -> f32 {
        let Some(paper) = self.paper else { return 0.0 };
        if self.engine < 7 || pts.is_empty() || self.drawing.is_none() || !(width.is_finite() && width >= 1.0) {
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
        for (t, &(c, dir)) in steps.iter().enumerate() {
            let e = match angle {
                Some(a) => (a.cos(), a.sin()),
                None => (-dir.1, dir.0),
            };
            let pr = pressure.0 + (pressure.1 - pressure.0) * t as f32 / n as f32;
            let q = 0.05 + 0.45 * pr.clamp(0.0, 1.0);
            let sink = edge_sink_um(q, edge_um.max(1.0), t_um);
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
                    let keep = Self::below(v, mu, z);
                    let r = cl.r;
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
                let resid = ((0.49 * eps - 0.027).max(0.0) * t_um).min(z);
                if resid > 0.0 {
                    dent.push((i, resid));
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
        self.height = heights;
        if !dent.is_empty() {
            self.surf_gen += 1;
        }
        // what was pushed ahead drops where the blade lifts
        if pushed > 0.0 && !last_band.is_empty() {
            let film = std::mem::take(&mut self.film);
            let bed: Vec<(usize, f32, bool)> = last_band.iter().map(|&i| (i, 1.0, false)).collect();
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
    /// or wet paint. Returns (volume lifted mm³, hand time s). Engine 7.
    pub fn dust_pastel(&mut self, width: f32, tip_um: f32, pts: &[(f32, f32)], held_mg: &mut f32, cap_mg: f32) -> (f32, f32) {
        if self.engine < 7 || pts.is_empty() || self.drawing.is_none() || !(width.is_finite() && width > 0.0) {
            return (0.0, 0.0);
        }
        let f = self.f;
        let s = f.scale;
        let s_mm = self.px_mm();
        let px_um2 = (s_mm * 1000.0).powi(2);
        // the gaps between fibres: about a fibre's width
        let w_um = self.paper.as_ref().map_or(30.0, |p| p.fibre_um);
        let r = tip_um.max(1.0);
        let reach_um = if w_um / 2.0 >= r { f32::INFINITY } else { r - (r * r - w_um * w_um / 4.0).sqrt() };
        let half = width * s * 0.5;
        // the tips' band along the path: a bristle tuft pressed on paper, about 2 mm deep
        let depth_px = (2.0 / s_mm).max(1.0);
        let steps = self.walk(pts);
        let total_px = (steps.len().max(1) - 1) as f32 * 0.7;
        let mut lifted = 0.0f32;
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
                lifted += ex;
                // the bristles keep what they have room for
                let room = (1.0 - *held_mg / cap_mg.max(1e-9)).clamp(0.0, 1.0);
                let kept = ex * room;
                *held_mg += kept * um_to_mg;
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
        let _ = last_band;
        self.drawing = Some(dr);
        self.px = pxs;
        // hand time: brushing at about 100 mm/s
        (lifted * px_um2 * 1e-9, total_px / s * self.mm_per_unit / 100.0 + 0.3)
    }

    /// A puff of air from `h_mm` above the picture at (x, y) (units), leaving
    /// the mouth (or a bulb's nozzle, `d_mm` across) at `u` m/s (blowing hard:
    /// 12 m/s on average, 6–64 measured). The jet's wall shear peaks on a ring
    /// at 0.09 h (Phares et al. 2000: τ = 44.6 ρU²Re^(−½)(h/d)^(−2), h/d at
    /// least 6) and falls off outside it as a radial wall jet's, ∝ r^(−2.3)
    /// (Poreh et al. 1967), rising from nothing at the centre. Loose pastel
    /// heaped on the tooth goes where the shear passes the threshold of its
    /// crumbs (Shao & Lu 2000, ~0.2 m/s); in the pores the flow dies with
    /// depth as exp(−4.21 z/w) (Moffatt's eddies, w a fibre's width), so only
    /// the top few µm of the pores' filling, its fine grains (~0.8 m/s), can
    /// go. What goes is airborne (redeposition, 0–20 % nearby, is left out).
    /// Fixed pastel stays. Returns the volume blown off, mm³. Engine 7.
    pub fn blow_pastel(&mut self, x: f32, y: f32, h_mm: f32, u: f32, d_mm: f32) -> f32 {
        if self.engine < 7 || self.drawing.is_none() || !(h_mm > 0.0 && u > 0.0 && d_mm > 0.0) {
            return 0.0;
        }
        let f = self.f;
        let s_mm = self.px_mm();
        let px_um2 = (s_mm * 1000.0).powi(2);
        let re = u * d_mm * 1e-3 / AIR_NU;
        let hd = (h_mm / d_mm).max(6.0);
        let tau_max = 44.6 * AIR_KG_M3 * u * u / re.sqrt() / (hd * hd);
        let r_peak = 0.09 * h_mm;
        let w_um = self.paper.as_ref().map_or(30.0, |p| p.fibre_um);
        let (ut_heap, ut_fine) = (shao_lu(100.0, CRUMB_KG_M3), shao_lu(5.0, 2500.0));
        let tau_at = |r: f32| -> f32 {
            if r <= r_peak { tau_max * r / r_peak.max(1e-6) } else { tau_max * (r_peak / r).powf(2.3) }
        };
        // out to where even the heaps stay
        let r_out = r_peak * (tau_max / (AIR_KG_M3 * ut_heap * ut_heap)).max(1.0).powf(1.0 / 2.3);
        let (cx, cy) = (x * self.mm_per_unit, y * self.mm_per_unit);
        let mut dr = self.drawing.take().unwrap();
        let mut pxs = std::mem::take(&mut self.px);
        let mut gone = 0.0f32;
        let to = |v: f32| ((v / s_mm).floor() as isize).max(0) as usize;
        for py in to(cy - r_out).max(f.y0)..(to(cy + r_out) + 1).min(f.y0 + f.h) {
            for pxx in to(cx - r_out).max(f.x0)..(to(cx + r_out) + 1).min(f.x0 + f.w) {
                let i = (py - f.y0) * f.w + (pxx - f.x0);
                if !self.open_pastel(&dr, i) {
                    continue;
                }
                let (wx, wy) = ((pxx as f32 + 0.5) * s_mm - cx, (py as f32 + 0.5) * s_mm - cy);
                let ustar = (tau_at((wx * wx + wy * wy).sqrt()) / AIR_KG_M3).sqrt();
                let mu = self.micro_um(i);
                let cl = &mut dr.cells[i];
                let v = cl.loose + cl.bound;
                // the heap goes past its threshold (shear fluctuates: a ramp, not a cut)
                let heap_goes = crate::smoothstep(0.8, 1.25, ustar / ut_heap);
                let heap = (v - mu).max(0.0).min(cl.loose);
                // and the pores' fine grains as deep as the flow keeps the shear
                let z = if ustar > ut_fine { w_um / 2.105 * (ustar / ut_fine).ln() } else { 0.0 };
                let in_pores = v.min(mu);
                let keep_pores = if z > 0.0 { Self::below(in_pores, mu, z) } else { in_pores };
                let keep = keep_pores + (v - in_pores) - heap * heap_goes;
                gone += Self::take_loose(cl, &mut pxs[i], keep);
            }
        }
        self.drawing = Some(dr);
        self.px = pxs;
        gone * px_um2 * 1e-9
    }

    /// The board tapped (its edge struck on the table, or the sheet held
    /// upright): every loose crumb lying on the tooth whose weight at `g`
    /// times gravity passes its adhesion falls off (m·a > F_adh: a crumb of
    /// d_c = (6F/πρa)^⅓ and larger). The crumbs' sizes, as a stroke leaves
    /// them, follow P(A) ∝ A^(−3/2), so their mass is even in area: the share
    /// of the heap that falls is (A_max − A_c)/(A_max − A_min). Grains held
    /// in the pores never go. Tapping a board edge gives some 50–400 g; an
    /// upright sheet, 1 g. Returns the volume shed, mm³. Engine 7.
    pub fn tap_pastel(&mut self, g: f32) -> f32 {
        if self.engine < 7 || self.drawing.is_none() || !(g > 0.0) {
            return 0.0;
        }
        let s_mm = self.px_mm();
        let px_um2 = (s_mm * 1000.0).powi(2);
        let d_c = (6.0 * ADHESION_N / (std::f32::consts::PI * CRUMB_KG_M3 * 9.81 * g)).cbrt() * 1e6;
        let (amin, amax, ac) = (25.0f32, CRUMB_MAX_UM * CRUMB_MAX_UM, (d_c * d_c).max(25.0));
        let share = ((amax - ac) / (amax - amin)).clamp(0.0, 1.0);
        if share <= 0.0 {
            return 0.0;
        }
        let mut dr = self.drawing.take().unwrap();
        let mut pxs = std::mem::take(&mut self.px);
        let mut gone = 0.0f32;
        for i in 0..pxs.len() {
            if !self.open_pastel(&dr, i) {
                continue;
            }
            let mu = self.micro_um(i);
            let cl = &mut dr.cells[i];
            let v = cl.loose + cl.bound;
            let heap = (v - mu).max(0.0).min(cl.loose);
            if heap > 0.0 {
                gone += Self::take_loose(cl, &mut pxs[i], v - heap * share);
            }
        }
        self.drawing = Some(dr);
        self.px = pxs;
        gone * px_um2 * 1e-9
    }

    /// A stick held at `pose` with its tip at (x, y) (units), over the window's
    /// pixels: 2 where it touches, 1 where its face lies within a crumb's size
    /// of the surface (where its crumbs settle, engine 7), 0 elsewhere under
    /// its low part. Reads only (the hold look). Engine 6 and on.
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
    use crate::canvas::Canvas;
    use crate::mask::Mask;
    use crate::paper::Paper;
    use crate::pastel::{Pose, Stick, StrokePoint};

    fn sheet7() -> Canvas {
        Canvas::new_window(600, 1.0, [0.55; 3], None).with_size_mm(150.0).with_engine(7).with_paper(Paper::drawing(4))
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

    /// A sheet over the left half: strokes across it leave the picture under
    /// it as it was, and the sheet keeps what fell on it; lifted, it is gone.
    #[test]
    fn a_sheet_keeps_what_it_covers_clean() {
        let mut c = sheet7();
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
    fn a_dry_brush_lifts_what_its_tips_reach() {
        let mut c = sheet7();
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
        let mut c = sheet7();
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
    fn a_knife_takes_the_loose_pastel_and_burnishes_the_paper() {
        let mut c = sheet7();
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
        let took = c.scrape_pastel(100.0, 100.0, &[(500.0, 180.0), (500.0, 820.0)], (0.6, 0.6), Some(0.0));
        let (d1, f1) = (dark(&c, 470.0, 530.0, 230.0, 370.0), dark(&c, 470.0, 530.0, 630.0, 770.0));
        assert!(took > 0.0 && d1 < 0.7 * d0, "the knife took the loose pastel: {d0} -> {d1}");
        assert!((f1 - f0).abs() < 0.05 * f0.max(1e-3), "the fixed stays: {f0} -> {f1}");
        let pressed = (0..h0.len()).filter(|&i| c.height[i] < h0[i] - 0.5).count();
        let shallower = (0..m0.len()).filter(|&i| c.micro[i] < m0[i] - 0.5).count();
        assert!(pressed > 0 && shallower > 0, "the paper burnished: {pressed} pixels pressed, {shallower} shallower");
    }

    /// An ordinary puff from 10 cm (12 m/s: u* ≈ 0.7 m/s at the ring) takes
    /// the loose heap and leaves the pores' filling (their fine grains need
    /// ~0.8 m/s); a hard blow from 5 cm (21 m/s: u* ≈ 2.2) also clears the top
    /// of the tooth under its ring (why blowers aren't used on friable pastel);
    /// beyond the jet's reach nothing moves.
    #[test]
    fn a_puff_takes_the_heap_and_a_hard_one_the_top_of_the_tooth() {
        let mut c = sheet7();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        dense(&mut c, &mut st, 200.0, 800.0);
        let mut hard = c.clone();
        let heap = |c: &Canvas| -> f32 { (0..c.px.len()).map(|i| { let cl = &c.drawing.as_ref().unwrap().cells[i]; (cl.loose + cl.bound - c.micro_um(i)).max(0.0) }).sum() };
        let pores = |c: &Canvas| -> f32 { (0..c.px.len()).map(|i| { let cl = &c.drawing.as_ref().unwrap().cells[i]; (cl.loose + cl.bound).min(c.micro_um(i)) }).sum() };
        let (h0, p0, far0) = (heap(&c), pores(&c), dark(&c, 150.0, 200.0, 250.0, 300.0));
        // (150 mm sheet: 1 unit = 0.15 mm)
        let soft = c.blow_pastel(500.0, 500.0, 100.0, 12.0, 8.0);
        let strong = hard.blow_pastel(500.0, 500.0, 50.0, 21.0, 8.0);
        assert!(soft > 0.0 && heap(&c) < h0, "the puff took heaped pastel");
        assert!((pores(&c) - p0).abs() < 1e-3 * p0, "an ordinary puff leaves the tooth's filling");
        assert!(strong > soft && pores(&hard) < p0, "a hard blow close in takes more, into the tooth");
        assert!((dark(&c, 150.0, 200.0, 250.0, 300.0) - far0).abs() < 1e-6, "out of its reach nothing moved");
    }

    /// A tap sheds the heaped crumbs (a hard tap more than gravity) and never
    /// the grains in the pores.
    #[test]
    fn a_tap_sheds_crumbs_not_the_tooth() {
        let mut c = sheet7();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        dense(&mut c, &mut st, 200.0, 800.0);
        let mut upright = c.clone();
        let heap = |c: &Canvas| -> f32 { (0..c.px.len()).map(|i| { let cl = &c.drawing.as_ref().unwrap().cells[i]; (cl.loose + cl.bound - c.micro_um(i)).max(0.0) }).sum() };
        let pores = |c: &Canvas| -> f32 { (0..c.px.len()).map(|i| { let cl = &c.drawing.as_ref().unwrap().cells[i]; (cl.loose + cl.bound).min(c.micro_um(i)) }).sum() };
        let (h0, p0) = (heap(&c), pores(&c));
        assert!(h0 > 0.0, "the passage is heaped");
        c.tap_pastel(100.0);
        upright.tap_pastel(1.0);
        assert!(heap(&c) < heap(&upright), "a hard tap sheds more than gravity: {} vs {}", heap(&c), heap(&upright));
        assert!(heap(&c) < 0.2 * h0, "most of the heap fell");
        assert!((pores(&c) - p0).abs() < 1e-3 * p0, "the pores keep theirs");
    }

    /// The hold's contact is where a stroke from there lays.
    #[test]
    fn the_hold_shows_where_a_stroke_lays() {
        let c = sheet7();
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
