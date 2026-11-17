//! The press: an inked plate under a sheet and the felts, through the
//! rollers (notes/plan-intaglio.md §6.2, phase P3b; the easel's
//! `press{}` under `--@ press`). Physics: notes/research/intaglio_physics.md
//! §6 (**IP §6.n**), with its tags.
//!
//! - **Pressure** (IP §6.1). The felts are a Winkler layer of stiffness
//!   E_f/T under a roller of radius R. Squeezed by d, a roller's length of
//!   them carries (4√2/3)·k·√R·d^(3/2) and presses k·d at the middle. Over the
//!   plate they are squeezed by its thickness more than beside it, so the
//!   press's load (its screws, up to its rating) is shared out between the
//!   plate and the felts beside it on the bed, and the plate takes more.
//! - **Press-in** (IP §6.4). Over an opening the damp sheet has nothing
//!   under it: the felt pushes it in, held by its through-thickness shear
//!   and its bending (a Timoshenko strip clamped at the rims):
//!   p·x(w − x)/(2·G·T) + p·x²(w − x)²/(24·D). A sheet many times thicker
//!   than the opening bridges it; a wide one is filled. Its fibrous surface
//!   reaches further: over the opening its fibre tops are not pressed flat,
//!   and the share of them reaching a depth z below the sheet's face is the
//!   bearing curve of its pores, e^(−z/µ) (paper.rs, µ its pore depth).
//! - **Transfer** (IP §6.4). Where the sheet touches the ink, the ink splits
//!   by s (0.45 to a non-absorbent sheet, 0.6 to an absorbent one [M]; a damp
//!   sheet 0.6 [E]); where it doesn't, nothing goes. So an opening gives s
//!   times its ink times the share of its width the sheet touches: more with
//!   its fill and with the press-in, never more than s. The film on the flat
//!   and the ink by the burr are always touched.
//! - **The print** is the plate mirrored, an oil film of the ink's paint on
//!   the sheet (`Canvas::lay_ink`), at its place. What isn't taken stays on
//!   the plate.
//! - **Embossing** (IP §6.3). A damp sheet (free water in its pores: its
//!   bonds open) keeps the shape it was pressed into as it dries: the lines
//!   stand up on the print by their press-in, the burr's shape is pressed
//!   in, and the plate's footprint lies below the margin by the plate mark,
//!   sloping at the bevel. A dry sheet springs back.

use super::*;
use super::inked::INKS;
use crate::canvas::Canvas;
use crate::surface::COAT_UM;
use crate::wet::Paint;

/// A hand etching press's rated line load (N/mm): 6 t over its 45 cm bed
/// [M, maker's rating: IP §6.1], and that bed's width (mm), which the felts
/// span.
pub const RATED_N_MM: f64 = 130.0;
pub const BED_MM: f64 = 450.0;
/// Its top roller's radius (mm): Ø 106 mm [M, the same press].
pub const ROLLER_MM: f64 = 53.0;
/// The felts (mm): a sizing catcher, a cushion and a pusher [M, Takach's
/// blanket specifications: IP §6.1].
pub const FELTS_MM: [f64; 3] = [1.6, 6.35, 3.2];
/// Wool felt's tangent modulus at its working strain (Pa): IP §6.1's 1–20
/// MPa, their geometric middle [E: felt is strongly nonlinear and no figure
/// for press blankets was found; Unknown 2].
pub const FELT_PA: f64 = 4.5e6;
/// A felt pack squeezed past this share of its thickness has bottomed out:
/// its fibres are packed solid [E: wool felt is about 80 % air].
pub const FELT_SOLID: f64 = 0.8;
/// The split share (IP §6.4): to a non-absorbent sheet 0.45 [M, Aoki &
/// Murakami 1962], to a highly absorbent one 0.6 [M, Pritchard & Finkle
/// 1964]; a dry sheet between them by how readily its pores take oil
/// (`Paper::absorbent`) [D]; a damp sheet 0.6 [E: IP §6.4's 0.55–0.7, the
/// plan's figure].
pub const SPLIT_SEALED: f64 = 0.45;
pub const SPLIT_ABSORBENT: f64 = 0.6;
pub const SPLIT_DAMP: f64 = 0.6;
/// A dry sheet's in-plane modulus (Pa): 4 GPa [M: machine-made printing and
/// drawing papers measure 2–8 GPa]. Damp, it falls with the through-thickness
/// ones (`damp::stiffness`). Its through-thickness shear modulus is sheet.rs's
/// (`PAPER_GXZ_MPA`).
pub const PAPER_E_PA: f64 = 4e9;
/// The plate mark's depth as a share of the plate's thickness in a damp
/// sheet: IP §6.3's "a fair share", 0.3–1 mm for 1–1.6 mm plates [E: the
/// sheet's spring-back over the bevel is unmeasured]. A dry sheet springs
/// back: none.
pub const MARK_SET: f64 = 0.5;
/// Laying the sheet, the tissue and the felts on the bed, and lifting them
/// off after (s) [E: plan §6.2's "about a minute per pass … with laying the
/// sheet and lifting it"].
pub const LAY_S: f64 = 45.0;

/// How the press is set.
#[derive(Clone, Debug, PartialEq)]
pub struct PressSpec {
    /// The screws, 0..1 of its rated load (`RATED_N_MM` over `BED_MM`),
    /// and its rating (N/mm).
    pub pressure: f64,
    pub rated_n_mm: f64,
    /// The felts (mm), the roller's radius (mm) and the speed (mm/s).
    pub felts_mm: Vec<f64>,
    pub roller_mm: f64,
    pub speed_mm_s: f64,
}

impl Default for PressSpec {
    fn default() -> Self {
        PressSpec { pressure: 0.7, rated_n_mm: RATED_N_MM, felts_mm: FELTS_MM.to_vec(), roller_mm: ROLLER_MM, speed_mm_s: 20.0 }
    }
}

/// The felts' squeeze under the roller: over the plate and beside it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Squeeze {
    /// The pressure at the middle of the contact (Pa) over the plate and
    /// beside it, how far the felts are squeezed over the plate (mm) and the
    /// share of their thickness that is, and the contact's length (mm).
    pub on_plate_pa: f64,
    pub beside_pa: f64,
    pub squeeze_mm: f64,
    pub strain: f64,
    pub contact_mm: f64,
}

/// The line load (N per m of roller) a Winkler layer of stiffness `k` (Pa/m)
/// carries under a roller of radius `r` (m) squeezed by `d` (m).
fn winkler_load(k: f64, r: f64, d: f64) -> f64 {
    if d <= 0.0 { 0.0 } else { 4.0 * std::f64::consts::SQRT_2 / 3.0 * k * r.sqrt() * d.powf(1.5) }
}

/// The felts squeezed by a press set `spec` over a plate `plate_mm` wide
/// along the roller and `thick_mm` thick: the load is the screws' share of
/// the rating over the bed, carried by the felts over the plate (squeezed
/// by d) and beside it (by d less the plate's thickness).
pub fn squeeze(spec: &PressSpec, plate_mm: f64, thick_mm: f64) -> Squeeze {
    let t = spec.felts_mm.iter().sum::<f64>().max(1e-3) * 1e-3;
    let k = FELT_PA / t;
    let r = spec.roller_mm * 1e-3;
    let (wp, wb) = (plate_mm.min(BED_MM) * 1e-3, (BED_MM - plate_mm).max(0.0) * 1e-3);
    let tp = thick_mm * 1e-3;
    let load = spec.pressure.clamp(0.0, 1.0) * spec.rated_n_mm * 1e3 * BED_MM * 1e-3;
    let carried = |d: f64| winkler_load(k, r, d) * wp + winkler_load(k, r, d - tp) * wb;
    let (mut lo, mut hi) = (0.0, tp + t);
    while carried(hi) < load && hi < 10.0 {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let m = 0.5 * (lo + hi);
        if carried(m) < load { lo = m } else { hi = m }
    }
    let d = 0.5 * (lo + hi);
    Squeeze { on_plate_pa: k * d, beside_pa: k * (d - tp).max(0.0), squeeze_mm: d * 1e3, strain: d / t, contact_mm: 2.0 * (2.0 * r * d).sqrt() * 1e3 }
}

/// What the sheet brings to the press.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetFeel {
    /// Its thickness (µm), its moduli against the dry sheet's
    /// (`damp::stiffness`), whether it is damp (free water in its pores:
    /// its bonds open, `Moisture::plastic`) and how readily its pores take
    /// oil (`Paper::absorbent`).
    pub thick_um: f64,
    pub stiffness: f64,
    pub damp: bool,
    pub absorbent: f64,
    /// Its strain beside its size as made (fractions; across its width, along
    /// its height): the water in its fibre walls swells it (`damp::strain`,
    /// `Moisture::size_mm`). The press reads it under the contact laws.
    pub strain: (f64, f64),
}

impl SheetFeel {
    /// The split share for this sheet.
    pub fn split(&self) -> f64 {
        if self.damp { SPLIT_DAMP } else { SPLIT_SEALED + (SPLIT_ABSORBENT - SPLIT_SEALED) * self.absorbent.clamp(0.0, 1.0) }
    }

    /// How far (µm) the sheet pressed at `p` Pa sinks into an opening `w` µm
    /// wide, `x` µm in from its rim: through-thickness shear and bending,
    /// clamped at the rims.
    pub fn press_in_um(&self, w: f64, x: f64, p: f64) -> f64 {
        let (w, x) = (w * 1e-6, x.clamp(0.0, w) * 1e-6);
        let t = self.thick_um * 1e-6;
        let g = crate::sheet::PAPER_GXZ_MPA as f64 * 1e6 * self.stiffness;
        let d = PAPER_E_PA * self.stiffness * t * t * t / 12.0;
        (p * x * (w - x) / (2.0 * g * t) + p * x * x * (w - x) * (w - x) / (24.0 * d)) * 1e6
    }
}

/// The share of an opening `w` µm wide and `d` µm deep, its ink's top `g`
/// µm below the plate's face, that a sheet pressed at `p` Pa touches: across
/// its width, its press-in and the bearing curve of its fibre tops (pore
/// depth `mu` µm).
pub fn contact(sheet: &SheetFeel, w: f64, d: f64, g: f64, p: f64, mu: f64) -> f64 {
    let n = 16;
    (0..n)
        .map(|i| {
            let x = (i as f64 + 0.5) * w / n as f64;
            let v = sheet.press_in_um(w, x, p).min(d);
            (-(g - v).max(0.0) / mu.max(1e-3)).exp()
        })
        .sum::<f64>()
        / n as f64
}

/// What a pass through the press did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PressReport {
    pub squeeze: Squeeze,
    /// The split share, the plate's ink before and after (µm³), what the
    /// sheet took (µm³), the plate mark's depth (µm) and hand time (s).
    pub split: f64,
    pub before_um3: f64,
    pub after_um3: f64,
    pub taken_um3: f64,
    pub mark_um: f64,
    pub secs: f64,
    /// Whether the sheet was damp (it keeps the embossing).
    pub damp: bool,
}

impl Plate {
    /// Print the plate on `sheet` (a canvas of paper at the plate's pitch):
    /// the plate's left edge at `at_mm` from the sheet's (x, y) as printed,
    /// mirrored; `paints` are the plate's inks as the sheet's oil paint, in
    /// the plate's order; `now` the plate's clock (minutes).
    pub fn press(&mut self, sheet: &mut Canvas, at_mm: (f64, f64), spec: &PressSpec, feel: &SheetFeel, paints: &[Paint], now: f64) -> Result<PressReport, String> {
        self.age_ink(now);
        let ps = sheet.px_mm() as f64 * 1000.0;
        if (ps / self.pitch - 1.0).abs() > 0.1 {
            return Err(format!("the sheet's pixels ({ps:.1} µm) aren't near the plate's cells ({:.1} µm): make them at one pitch", self.pitch));
        }
        let inks = self.ink.as_ref().map(|k| k.inks.len()).unwrap_or(0);
        if paints.len() < inks {
            return Err("a paint for each of the plate's inks".into());
        }
        let f = sheet.window();
        let (pw, ph) = (self.width_mm as f64, self.height_mm as f64);
        // the sheet as it is now: its pixels are its fibres as made, so a mm
        // of it now is 1/(1 + strain) of a mm as made (under the contact laws;
        // without them, as made). `at_mm` is measured on the sheet now from its
        // top left edge, where it is held.
        let (gx, gy) = if self.contact { (1.0 / (1.0 + feel.strain.0), 1.0 / (1.0 + feel.strain.1)) } else { (1.0, 1.0) };
        let (sw, sh_mm) = (f.full_w as f64 * ps / 1000.0 / gx, f.full_h as f64 * ps / 1000.0 / gy);
        if at_mm.0 < -1e-9 || at_mm.1 < -1e-9 || at_mm.0 + pw > sw + 1e-6 || at_mm.1 + ph > sh_mm + 1e-6 {
            return Err("the plate doesn't lie within the sheet there".into());
        }
        let sq = squeeze(spec, pw, self.thickness_mm as f64);
        if sq.strain > FELT_SOLID {
            return Err(format!("the felts are squeezed solid (by {:.0} % of their thickness): ease the screws or add a felt", 100.0 * sq.strain));
        }
        let p = sq.on_plate_pa;
        let s = feel.split();
        let before = self.ink_um3();
        let mut taken = 0.0f64;
        let a = self.pitch * self.pitch;
        // the sheet's pixel (in its window) under a point of the plate (mm from its left and top), mirrored
        let under = |x_mm: f64, y_mm: f64| -> Option<usize> {
            let (sx, sy) = (((at_mm.0 + pw - x_mm) * gx * 1000.0 / ps).floor(), ((at_mm.1 + y_mm) * gy * 1000.0 / ps).floor());
            if sx < 0.0 || sy < 0.0 {
                return None;
            }
            let (sx, sy) = (sx as usize, sy as usize);
            if sx < f.x0 || sy < f.y0 || sx >= f.x0 + f.w || sy >= f.y0 + f.h {
                return None;
            }
            Some((sy - f.y0) * f.w + (sx - f.x0))
        };
        // the embossing and the plate mark (a damp sheet keeps them): each sheet pixel over
        // the plate, from the cell under it
        let mark = if feel.damp { MARK_SET * (self.thickness_mm as f64 * 1000.0).min(sq.squeeze_mm * 1000.0) } else { 0.0 };
        if feel.damp {
            let ramp = (self.thickness_mm as f64 / (self.bevel_deg as f64).to_radians().tan()).max(1e-3);
            for wy in 0..f.h {
                for wx in 0..f.w {
                    let (cx, cy) = ((f.x0 + wx) as f64 + 0.5, (f.y0 + wy) as f64 + 0.5);
                    let (x_mm, y_mm) = (at_mm.0 + pw - cx * ps / 1000.0 / gx, cy * ps / 1000.0 / gy - at_mm.1);
                    if x_mm < 0.0 || y_mm < 0.0 || x_mm >= pw || y_mm >= ph {
                        continue;
                    }
                    let edge = x_mm.min(pw - x_mm).min(y_mm).min(ph - y_mm);
                    let mut dz = -mark * (edge / ramp).min(1.0);
                    let c = self.cell((x_mm * 1000.0 / self.pitch) as usize, (y_mm * 1000.0 / self.pitch) as usize);
                    for (sh, w, d) in c.openings() {
                        let v = (0..8).map(|j| feel.press_in_um(w as f64, (j as f64 + 0.5) * w as f64 / 8.0, p).min(d as f64)).sum::<f64>() / 8.0;
                        dz += sh as f64 * v;
                    }
                    dz -= c.burr_vol as f64;
                    sheet.height[wy * f.w + wx] += dz as f32;
                }
            }
        }
        // the ink: each cell's share, laid over the sheet pixels its footprint
        // covers, by the area it covers of each (the plate's cells and the
        // sheet's pixels are laid out at nearly the same pitch, but not
        // exactly: a cell dropped whole on the pixel under its middle leaves
        // a row or column of pixels with none where the two drift apart by a
        // pixel, and lays two cells' ink on one where they drift back)
        let area = (self.pitch / ps).powi(2);
        let (kx, ky) = (self.pitch * gx / ps, self.pitch * gy / ps);
        let pitch = self.pitch;
        let footprint = move |x: usize, y: usize| -> Vec<(usize, f64)> {
            // the cell's corners on the sheet, in pixels (mirrored in x)
            let u0 = (at_mm.0 + pw - (x as f64 + 1.0) * pitch / 1000.0) * gx * 1000.0 / ps;
            let v0 = (at_mm.1 + y as f64 * pitch / 1000.0) * gy * 1000.0 / ps;
            let (u1, v1) = (u0 + kx, v0 + ky);
            let mut out = Vec::with_capacity(4);
            let mut sy = v0.floor();
            while sy < v1 {
                let oy = (v1.min(sy + 1.0) - v0.max(sy)).max(0.0);
                let mut sx = u0.floor();
                while sx < u1 {
                    let ox = (u1.min(sx + 1.0) - u0.max(sx)).max(0.0);
                    let w = ox * oy / (kx * ky);
                    if w > 0.0 && sx >= 0.0 && sy >= 0.0 {
                        let (px, py) = (sx as usize, sy as usize);
                        if px >= f.x0 && py >= f.y0 && px < f.x0 + f.w && py < f.y0 + f.h {
                            out.push(((py - f.y0) * f.w + (px - f.x0), w));
                        }
                    }
                    sx += 1.0;
                }
                sy += 1.0;
            }
            out
        };
        let mut lays: Vec<(usize, [f32; INKS], f32, f32)> = Vec::new();
        let mut dirty: Option<(usize, usize, usize, usize)> = None;
        for y in 0..self.h {
            for x in 0..self.w {
                let ic = self.ink_cell(x, y);
                if ic.total() <= 0.0 {
                    continue;
                }
                let Some(i) = under((x as f64 + 0.5) * self.pitch / 1000.0, (y as f64 + 0.5) * self.pitch / 1000.0) else { continue };
                let c = self.cell(x, y);
                let fl = crate::drying::fluid(ic.cure) as f64;
                let mu = sheet.micro.get(i).copied().unwrap_or(5.0) as f64;
                let mut t = ic.film as f64 * s * fl;
                let mut cell = ic;
                cell.film = (ic.film as f64 - t).max(0.0) as f32;
                for k in 0..BINS {
                    if c.open[k] <= 0.0 || ic.line[k] <= 0.0 {
                        continue;
                    }
                    let (w, d) = ((c.wsum[k] / c.open[k]) as f64, (c.vol[k] / c.open[k]) as f64);
                    let fill = (ic.line[k] / c.open[k]) as f64;
                    let share = contact(feel, w, d, (d - fill).max(0.0), p, mu);
                    let b = (ic.line[k] as f64 * s * share * fl).min(ic.line[k] as f64);
                    cell.line[k] = (ic.line[k] as f64 - b).max(0.0) as f32;
                    t += b;
                }
                if t <= 0.0 {
                    continue;
                }
                taken += t * a;
                if cell.total() <= 0.0 {
                    cell = super::inked::InkCell::default();
                }
                *self.ink_cell_mut(x, y) = cell;
                let coats = t * area / COAT_UM as f64;
                for (j, w) in footprint(x, y) {
                    lays.push((j, ic.shares(), (coats * w) as f32, ic.cure));
                    let (ux, uy) = (j % f.w, j / f.w);
                    dirty = Some(match dirty {
                        None => (ux, uy, ux + 1, uy + 1),
                        Some(d) => (d.0.min(ux), d.1.min(uy), d.2.max(ux + 1), d.3.max(uy + 1)),
                    });
                }
            }
        }
        for (i, sh, coats, cure) in lays {
            for (k, paint) in paints.iter().enumerate().take(inks) {
                if sh[k] > 0.0 {
                    sheet.lay_ink(i, coats * sh[k], paint, cure);
                }
            }
        }
        if let Some((x0, y0, x1, y1)) = dirty {
            sheet.wet.touch(x0, y0, x1, y1);
        }
        let after = self.ink_um3();
        // a pass: the bed's travel over the plate and the contact, at its speed, with the laying
        let travel = self.height_mm as f64 + sq.contact_mm;
        let secs = travel / spec.speed_mm_s.max(1.0) + LAY_S;
        Ok(PressReport { squeeze: sq, split: s, before_um3: before.0 + before.1, after_um3: after.0 + after.1, taken_um3: taken, mark_um: mark, secs, damp: feel.damp })
    }
}

impl Canvas {
    /// Lay `coats` of `paint` (cure `cure`) at pixel `i` as wet paint,
    /// mixing by volume with what is wet there (as a rag's smear lays paint,
    /// `rag_lay`); nothing drains into the paper: a damp sheet's pores hold
    /// water, which oil doesn't enter (IP §6.4), and the ink sits on its
    /// fibres. The caller marks the wet layer touched.
    pub fn lay_ink(&mut self, i: usize, coats: f32, paint: &Paint, cure: f32) {
        if coats <= 0.0 {
            return;
        }
        let t = self.wet.vol[i] + coats;
        let a = coats / t;
        let lat = paint.latent();
        let l = &mut self.wet.lat[i];
        for k in 0..l.len() {
            l[k] += (lat[k] - l[k]) * a;
        }
        l.blend_parts(&lat, a);
        let ph = paint.prop();
        let hd = &mut self.wet.hide[i];
        for k in 0..hd.len() {
            hd[k] += (ph[k] - hd[k]) * a;
        }
        if self.wet.clock.px.len() == self.wet.vol.len() {
            let p = &mut self.wet.clock.px[i];
            p.cure += (cure - p.cure) * a;
        }
        self.wet.cover[i] = 1.0;
        self.wet.vol[i] = t;
    }

    /// The wet paint on the canvas (coats × pixels).
    pub fn wet_coats(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum()
    }
}

#[cfg(test)]
mod tests;
