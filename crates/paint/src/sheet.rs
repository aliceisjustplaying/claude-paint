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
//! **The blade on pastel.** A knife drawn over pastel rests, as on paint, on
//! the highest points of the surface within the steel's flex and stands off
//! them by a gap that closes as it is pressed. Pastel lies in the paper's
//! pores from the bottom up (`pastel.rs`): what fills the pores lies below
//! the top envelope the blade rides on, and only what is heaped above it, a
//! built-up layer or a fixed crust, stands where the blade can take it. So a
//! blade lowers a heavy, packed passage and leaves the tooth's filling.
//!
//! **A dry brush over pastel** (notes/research/pastel_brushing.md). A bristle
//! tip pushes sideways with 10³–10⁶ times the force that rolls a pastel
//! particle off paper, so brushing is limited by reach, not force: loose
//! pastel the tips touch comes away, and what lies deeper in the pores than
//! a tip can go stays (the ghost a brushed-out correction leaves). A tip of
//! radius r over gaps between fibres of width w sinks h = r − √(r² − w²/4);
//! the pastel filling the pores from the bottom up (exponential depths of
//! mean µ) that lies deeper than h is µ·e^(−h/µ), and that stays. Fixed
//! pastel is bonded and stays. What comes away is pushed ahead of the tips
//! and carried on the bristles, dropping into the pores downstream, where
//! the brush meets it again: so it is swept along to where the brush lifts,
//! and left there as a darker lip, with the ghost behind:
//! the shares are estimates (no measurement exists; the note gives a bench
//! test that would set them).

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
/// Of the pastel a dry brush lifts, the share carried on the bristles and let
/// go over `CARRY_MM` downstream; the rest is pushed ahead and drops within
/// `DROP_MM` of the tips
/// (estimates: 30–70 %, 1–5 mm, 0.1–1 mm; pastel_brushing.md §6).
const CARRY: f32 = 0.5;
const CARRY_MM: f32 = 3.0;
const DROP_MM: f32 = 0.5;

/// The steel flexes over broad relief and bridges fine hollows: the blade
/// rests on the highest point within this many mm along it (as `Canvas::knife`).
const FLEX_MM: f32 = 4.0;

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

    /// A scrape with a blade `width` units long along `pts` (units), held
    /// across the path or at `angle` (radians), pressed `pressure` (start,
    /// end; 0..1): the pastel heaped above the paper's top envelope, where it
    /// stands above the blade, is taken off (the newest, loose, first; then
    /// the fixed crust). The pastel in the pores stays. Not under a sheet,
    /// nor under paint. Returns the volume taken, mm³. Engine 7.
    pub fn scrape_pastel(&mut self, width: f32, pts: &[(f32, f32)], pressure: (f32, f32), angle: Option<f32>) -> f32 {
        if self.engine < 7 || pts.is_empty() || self.drawing.is_none() || !(width.is_finite() && width >= 1.0) {
            return 0.0;
        }
        let f = self.f;
        let s = f.scale;
        let s_mm = self.px_mm();
        let a_px_mm2 = s_mm * s_mm;
        let p: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
        let path = if p.len() >= 2 { crate::path::densify(&p) } else { vec![p[0], p[0]] };
        let mut arc = vec![0.0f32; path.len()];
        for i in 1..path.len() {
            arc[i] = arc[i - 1] + ((path[i].0 - path[i - 1].0).powi(2) + (path[i].1 - path[i - 1].1).powi(2)).sqrt();
        }
        let total = arc[arc.len() - 1].max(1e-6);
        let half = width * s * 0.5;
        let pix = |x: f32, y: f32| -> Option<usize> {
            let (xi, yi) = (x.floor() as isize - f.x0 as isize, y.floor() as isize - f.y0 as isize);
            if xi < 0 || yi < 0 || xi >= f.w as isize || yi >= f.h as isize {
                return None;
            }
            Some(yi as usize * f.w + xi as usize)
        };
        let reach = ((FLEX_MM / (s_mm * 0.7)).ceil() as usize).max(1);
        let n = ((total / 0.7).ceil() as usize).max(1);
        let mut j = 0usize;
        let mut last_dir = (1.0f32, 0.0f32);
        let mut taken_um = 0.0f32;
        let sheet = self.sheet.take();
        let mut pxs = std::mem::take(&mut self.px);
        let mut dr = self.drawing.take().unwrap();
        {
            let height = &self.height;
            let film = &self.film;
            let d = &mut dr;
            for t in 0..=n {
                let dd = total * t as f32 / n as f32;
                while j + 1 < path.len() - 1 && arc[j + 1] < dd {
                    j += 1;
                }
                let (a, b) = (path[j], path[(j + 1).min(path.len() - 1)]);
                let seg = (arc[(j + 1).min(path.len() - 1)] - arc[j]).max(1e-6);
                let fr = ((dd - arc[j]) / seg).clamp(0.0, 1.0);
                let c = (a.0 + (b.0 - a.0) * fr, a.1 + (b.1 - a.1) * fr);
                let dir = {
                    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                    let m = (dx * dx + dy * dy).sqrt();
                    if m > 1e-6 { (dx / m, dy / m) } else { last_dir }
                };
                last_dir = dir;
                let e = match angle {
                    Some(a) => (a.cos(), a.sin()),
                    None => (-dir.1, dir.0),
                };
                let pr = pressure.0 + (pressure.1 - pressure.0) * (dd / total);
                // the gap between blade and the paper's peaks, µm (as `knife`)
                let gap = 300.0 * (1.0 - pr.clamp(0.0, 1.0)).powf(1.5);
                let nb = ((2.0 * half / 0.7).ceil() as usize).max(2);
                let mut blade: Vec<usize> = Vec::with_capacity(nb + 1);
                for q in 0..=nb {
                    let u = -half + 2.0 * half * q as f32 / nb as f32;
                    if let Some(i) = pix(c.0 + e.0 * u, c.1 + e.1 * u) {
                        if blade.last() != Some(&i) {
                            blade.push(i);
                        }
                    }
                }
                if blade.is_empty() {
                    continue;
                }
                // what the blade rests on: the paper's top envelope, and the
                // pastel heaped above it, and the sheet where it lies
                let mu: Vec<f32> = blade.iter().map(|&i| self.micro_um(i)).collect();
                let top = |bi: usize, i: usize| -> f32 {
                    let cl = &d.cells[i];
                    // the pores hold a layer as deep as their mean depth; the rest is heaped above
                    let heap = if cl.film >= 0.0 && film[i] <= cl.film + 1e-4 { (cl.loose + cl.bound - mu[bi]).max(0.0) } else { 0.0 };
                    let sh = sheet.as_ref().filter(|s| s.over[i]).map_or(0.0, |s| s.caliper_um);
                    height[i] + sh + if sh > 0.0 { 0.0 } else { heap }
                };
                let tops: Vec<f32> = blade.iter().enumerate().map(|(bi, &i)| top(bi, i)).collect();
                let hs: Vec<f32> = blade.iter().map(|&i| height[i] + sheet.as_ref().filter(|s| s.over[i]).map_or(0.0, |s| s.caliper_um)).collect();
                let rest: Vec<f32> = (0..hs.len()).map(|q| hs[q.saturating_sub(reach)..(q + reach + 1).min(hs.len())].iter().cloned().fold(f32::MIN, f32::max)).collect();
                for (bi, &i) in blade.iter().enumerate() {
                    if sheet.as_ref().is_some_and(|s| s.over[i]) {
                        continue;
                    }
                    let plane = rest[bi] + gap;
                    let tp = tops[bi];
                    if tp <= plane {
                        continue;
                    }
                    let cl = &mut d.cells[i];
                    if cl.film < 0.0 || film[i] > cl.film + 1e-4 {
                        continue;
                    }
                    let v = cl.loose + cl.bound;
                    let ex = (tp - plane).min(v);
                    if ex <= 0.0 {
                        continue;
                    }
                    // the newest, loose, comes off first; then the crust
                    let from_loose = ex.min(cl.loose);
                    cl.loose -= from_loose;
                    cl.bound = (cl.bound - (ex - from_loose)).max(0.0);
                    let v1 = cl.loose + cl.bound;
                    // coverage follows the volume as a deposit's does
                    let cov = |v: f32| 1.0 - (-v / PARTICLE_UM).exp();
                    let a1 = (cl.a * cov(v1) / cov(v).max(1e-6)).clamp(0.0, cl.a);
                    let under = uncover(pxs[i], cl.a, cl.r);
                    cl.a = a1;
                    cl.floor = cl.floor.min(a1);
                    pxs[i] = cover(under, cl.a, cl.r);
                    taken_um += ex;
                }
            }
        }
        self.drawing = Some(dr);
        self.px = pxs;
        self.sheet = sheet;
        taken_um * a_px_mm2 * 1e-3
    }

    /// A dry brush drawn over pastel along `pts` (units), its tips touching a
    /// band `width` units wide across the path, each tip `tip_um` µm in
    /// radius. Loose pastel within the tips' reach comes away; what lies
    /// deeper in the pores, and fixed pastel, stays. What it lifts it pushes
    /// ahead and drops downstream (see the module's notes). Not under a sheet, nor under paint
    /// or wet paint. Returns (volume lifted mm³, hand time s). Engine 7.
    pub fn dust_pastel(&mut self, width: f32, tip_um: f32, pts: &[(f32, f32)]) -> (f32, f32) {
        if self.engine < 7 || pts.is_empty() || self.drawing.is_none() || !(width.is_finite() && width > 0.0) {
            return (0.0, 0.0);
        }
        let f = self.f;
        let s = f.scale;
        let s_mm = self.px_mm();
        let a_px_mm2 = s_mm * s_mm;
        // the gaps between fibres: about a fibre's width
        let w_um = self.paper.as_ref().map_or(30.0, |p| p.fibre_um);
        let r = tip_um.max(1.0);
        let reach_um = if w_um / 2.0 >= r { f32::INFINITY } else { r - (r * r - w_um * w_um / 4.0).sqrt() };
        let p: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
        let path = if p.len() >= 2 { crate::path::densify(&p) } else { vec![p[0], p[0]] };
        let mut arc = vec![0.0f32; path.len()];
        for i in 1..path.len() {
            arc[i] = arc[i - 1] + ((path[i].0 - path[i - 1].0).powi(2) + (path[i].1 - path[i - 1].1).powi(2)).sqrt();
        }
        let total = arc[arc.len() - 1].max(1e-6);
        let half = width * s * 0.5;
        // the tips' band along the path: a bristle tuft pressed on paper, about 2 mm deep
        let depth_px = (2.0 / s_mm).max(1.0);
        let pix = |x: f32, y: f32| -> Option<usize> {
            let (xi, yi) = (x.floor() as isize - f.x0 as isize, y.floor() as isize - f.y0 as isize);
            if xi < 0 || yi < 0 || xi >= f.w as isize || yi >= f.h as isize {
                return None;
            }
            Some(yi as usize * f.w + xi as usize)
        };
        let n = ((total / 0.7).ceil() as usize).max(1);
        let ds_mm = total / n as f32 * s_mm;
        let release = 1.0 - (-ds_mm / CARRY_MM).exp();
        let mut j = 0usize;
        let mut last_dir = (1.0f32, 0.0f32);
        let mut lifted_um = 0.0f32;
        // carried and dropping: µm over a pixel, and its colour
        let (mut carry, mut drop, mut col) = (0.0f32, 0.0f32, [0.0f32; 3]);
        let mut pxs = std::mem::take(&mut self.px);
        let mut dr = self.drawing.take().unwrap();
        let mut last_trail: Vec<usize> = Vec::new();
        {
            let sheet = self.sheet.as_ref();
            let film = &self.film;
            let wet = &self.wet.vol;
            let cov = |v: f32| 1.0 - (-v / PARTICLE_UM).exp();
            let lay = |cells: &mut [crate::graphite::Cell], pxs: &mut [Rgb], trail: &[usize], vol: f32, col: Rgb| {
                if trail.is_empty() || vol <= 0.0 {
                    return;
                }
                let dv = vol / trail.len() as f32;
                let q = cov(dv);
                for &i in trail {
                    let c = &mut cells[i];
                    if c.film < 0.0 || film[i] > c.film + 1e-4 {
                        *c = crate::graphite::Cell { film: film[i], ..crate::graphite::Cell::default() };
                    }
                    let under = uncover(pxs[i], c.a, c.r);
                    let old = c.a * (1.0 - q);
                    let w = old + q;
                    c.r = [0, 1, 2].map(|k| ((old * c.r[k] + q * col[k]) / w.max(1e-6)).clamp(0.0, 1.0));
                    c.lift = (old * c.lift + q * 0.55) / w.max(1e-6);
                    c.a = (c.a + (1.0 - c.a) * q).min(0.995);
                    c.loose += dv;
                    pxs[i] = cover(under, c.a, c.r);
                }
            };
            for t in 0..=n {
                let dd = total * t as f32 / n as f32;
                while j + 1 < path.len() - 1 && arc[j + 1] < dd {
                    j += 1;
                }
                let (a, b) = (path[j], path[(j + 1).min(path.len() - 1)]);
                let seg = (arc[(j + 1).min(path.len() - 1)] - arc[j]).max(1e-6);
                let fr = ((dd - arc[j]) / seg).clamp(0.0, 1.0);
                let c = (a.0 + (b.0 - a.0) * fr, a.1 + (b.1 - a.1) * fr);
                let dir = {
                    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                    let m = (dx * dx + dy * dy).sqrt();
                    if m > 1e-6 { (dx / m, dy / m) } else { last_dir }
                };
                last_dir = dir;
                let e = (-dir.1, dir.0);
                // the band under the tips, and just ahead of it, downstream
                let mut band: Vec<usize> = Vec::new();
                let mut trail: Vec<usize> = Vec::new();
                let nb = ((2.0 * half / 0.7).ceil() as usize).max(1);
                for q in 0..=nb {
                    let u = -half + 2.0 * half * q as f32 / nb as f32;
                    for back in 0..(depth_px.ceil() as usize) {
                        if let Some(i) = pix(c.0 + e.0 * u - dir.0 * back as f32, c.1 + e.1 * u - dir.1 * back as f32) {
                            band.push(i);
                        }
                    }
                    let tb = DROP_MM / s_mm;
                    if let Some(i) = pix(c.0 + e.0 * u + dir.0 * tb, c.1 + e.1 * u + dir.1 * tb) {
                        trail.push(i);
                    }
                }
                band.sort_unstable();
                band.dedup();
                trail.sort_unstable();
                trail.dedup();
                trail.retain(|&i| !sheet.is_some_and(|s| s.over[i]) && wet[i] <= 1e-5);
                for &i in &band {
                    if sheet.is_some_and(|s| s.over[i]) || wet[i] > 1e-5 {
                        continue;
                    }
                    let cl = &mut dr.cells[i];
                    if cl.film < 0.0 || film[i] > cl.film + 1e-4 || cl.loose <= 0.0 {
                        continue;
                    }
                    let v = cl.loose + cl.bound;
                    let mu = self.micro_um(i);
                    // what lies deeper than the tips reach stays, and the fixed
                    let keep = cl.bound.max(v.min(mu * (-reach_um / mu).exp()));
                    let ex = v - keep;
                    if ex <= 1e-6 {
                        continue;
                    }
                    cl.loose -= ex;
                    let a1 = (cl.a * cov(keep) / cov(v).max(1e-6)).clamp(0.0, cl.a);
                    let under = uncover(pxs[i], cl.a, cl.r);
                    let tot = carry + drop + ex;
                    col = [0, 1, 2].map(|k| (col[k] * (carry + drop) + cl.r[k] * ex) / tot.max(1e-9));
                    cl.a = a1;
                    pxs[i] = cover(under, cl.a, cl.r);
                    carry += ex * CARRY;
                    drop += ex * (1.0 - CARRY);
                    lifted_um += ex;
                }
                // what drops just ahead, and a share of what is carried
                let give = drop + carry * release;
                lay(&mut dr.cells, &mut pxs, &trail, give, col);
                carry -= carry * release;
                drop = 0.0;
                if !trail.is_empty() {
                    last_trail = trail;
                }
            }
            // lifted off: what is left on the bristles drops where they leave
            lay(&mut dr.cells, &mut pxs, &last_trail, carry, col);
        }
        self.drawing = Some(dr);
        self.px = pxs;
        // hand time: brushing at about 100 mm/s
        (lifted_um * a_px_mm2 * 1e-3, total / s * self.mm_per_unit / 100.0 + 0.3)
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
        let bare_before = dark(&c, 965.0, 995.0, 230.0, 370.0);
        for y in (200..=400).step_by(10) {
            // (out past the passage's end onto bare paper, where the brush lifts)
            c.dust_pastel(40.0, 50.0, &[(120.0, y as f32), (975.0, y as f32)]);
            fine.dust_pastel(40.0, 10.0, &[(120.0, y as f32), (975.0, y as f32)]);
        }
        for y in (600..=800).step_by(10) {
            c.dust_pastel(40.0, 50.0, &[(120.0, y as f32), (880.0, y as f32)]);
        }
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
        for y in (200..=400).step_by(10) {
            c.dust_pastel(40.0, 50.0, &[(120.0, y as f32), (880.0, y as f32)]);
        }
        let after = dark(&c, 300.0, 700.0, 230.0, 370.0);
        assert!(after > 0.8 * before, "a speckled passage isn't cleaned by brushing: {before} -> {after}");
    }

    /// The blade takes what is heaped above the paper and leaves the pores'
    /// filling: a heavy passage gets lighter, but not bare.
    #[test]
    fn a_blade_takes_the_heap_and_leaves_the_tooth() {
        let mut c = sheet7();
        let mut st = Stick::round([0.02; 3], 0.9, 12.0);
        for _ in 0..4 {
            band(&mut c, &mut st, 300.0, 700.0, 4.0);
            c.fix_pastel(None, 0.15);
        }
        let d0 = dark(&c, 200.0, 800.0, 320.0, 680.0);
        let took = c.scrape_pastel(200.0, &[(500.0, 250.0), (500.0, 750.0)], (1.0, 1.0), Some(0.0));
        let d1 = dark(&c, 400.0, 600.0, 320.0, 680.0);
        assert!(took > 0.0, "the blade took some pastel");
        assert!(d1 > 0.3 * d0, "the pores' filling stays: {d0} -> {d1}");
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
