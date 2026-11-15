//! Pastel with water: a stick dipped in water, a wet brush drawn through
//! laid pastel, ground pastel laid as a paste or wash with a brush. Source:
//! notes/research/wet_pastel.md (its sections cited as §n; [M] measured, [P]
//! a practitioner's or museum's account, [E] an estimate). Almost nothing
//! here is a measurement of pastel itself: none was found.
//!
//! - **Water in the pores** (`Cell::water`, µm standing in the pastel and
//!   the paper's surface) replaces the air between the particles: their
//!   index contrast falls and so does their scattering (§2). The pastel
//!   reads darker and more saturated while it is wet, by the share of its
//!   pores the water fills (`wet_scatter`). It is optical only: the dry
//!   colors are kept, so it lightens again as the water goes.
//! - **It dries on the painting's clock** (`Canvas::dry_pastel_water`, from
//!   `wait`): it evaporates at a studio's rate and sinks into an absorbent
//!   sheet (`Paper::absorbent`), faster the more absorbent (§4).
//! - **Dry again**, the gum the water took up has set (§1, §3): a share of
//!   the loose pastel there is bound (`GUM_BIND`; the finger and a dry tool
//!   no longer move it, and a stick files it as a crust), and what was wet
//!   keeps a darker color (`DRIED_S`): lighter than wet, darker than dry.
//! - **A wet stick** (`Stick::water`, `Canvas::stick_stroke`): the soaked
//!   tip is softer and wears faster, so it lays more for the same force; its
//!   paste sheds fewer crumbs and spreads in the tooth (it covers more for
//!   its volume), and it lays the water it carries with it.
//! - **A wet brush** (`Canvas::wash_pastel`) lays water along its path; the
//!   loose pastel it wets turns to a paste the bristles pick up and lay down
//!   further on, as the finger does dry pastel, with the paste it already
//!   held (ground pastel mixed with water: `WashLoad::paste_mm3`).

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::pastel::{PARTICLE_UM, Section, Stick};

/// The share of a layer of loose pastel's volume that is pores (random
/// packing of fine powder, 0.4–0.6 [E], §2): what water fills.
pub const VOID: f32 = 0.45;
/// The pastel's scattering with its pores full of water, as a share of
/// its dry scattering: 0.3 [E], from the index contrast of chalk, kaolin
/// and pigment in water (n 1.33) against air and oil (§2): between dry
/// (1) and fixative's resin (`fix_pastel`'s fifth).
pub const WATER_S: f32 = 0.3;
/// Dried wet pastel's scattering, as a share of dry pastel's: packed
/// denser, its fines and gum drawn into the contacts (0.8 [E], §3).
pub const DRIED_S: f32 = 0.8;
/// The share of the loose pastel a drying wetting binds (its gum, 1–2 % by
/// weight, redissolved and set again at the contacts: 0.5 [E], §1, §3).
pub const GUM_BIND: f32 = 0.5;
/// Water evaporating from a damp sheet in a studio, µm a minute (still air
/// at 20 °C and 50 % RH: about 2.5; a generous wash is dry in about 20
/// minutes, Handprint [P]: §4) [E].
pub const EVAP_UM_MIN: f32 = 2.5;
/// How long a fully absorbent sheet (`absorbent` 1) takes to draw its
/// surface water into itself, minutes (an e-fold; a sized sheet slower in
/// proportion: 1.5 [E], §4).
pub const SINK_MIN: f32 = 1.5;
/// A stick's pores (soft pastel 0.3–0.5 [E], §1).
pub const TIP_POROSITY: f32 = 0.4;
/// How far up its end a dipped stick is soaked, mm (the tip dipped: 1 [E]).
pub const DIP_MM: f32 = 1.0;
/// The soaked tip's water spreads up the stick and dries off it: an
/// e-fold of this many minutes [E].
pub const TIP_DRY_MIN: f32 = 2.0;
/// Water a wet tip's paste carries per volume of pastel, the soaked
/// stick's φ/(1 − φ) (0.7 at `TIP_POROSITY`).
pub const PASTE_WATER: f32 = TIP_POROSITY / (1.0 - TIP_POROSITY);
/// A fully soaked tip's hardness, as a share of the dry stick's (wet chalk
/// and clay compacts lose most of their strength: 0.4 [E], §3).
pub const WET_SOFT: f32 = 0.4;
/// A fully soaked tip wears this many times faster (2.5 [E], §3).
pub const WET_WEAR: f32 = 2.5;
/// The share of the crumbs a fully soaked tip no longer sheds: its paste
/// holds together (0.6 [E], §3).
pub const WET_SHED: f32 = 0.6;
/// A paste's particles lie this share of a dry deposit's thickness
/// (`PARTICLE_UM`): it flows into the tooth and covers more for its volume
/// (0.5 [E], §3).
pub const PASTE_THIN: f32 = 0.5;
/// The water a brush's belly holds, mm³ per mm² of its width squared (a
/// 10 mm brush about 0.1 mL [E], §5).
pub const BRUSH_WATER_MM3: f32 = 0.9;
/// The most ground pastel a brush holds as a paste, a share of its water
/// (0.25 [E]).
pub const BRUSH_PASTE_SHARE: f32 = 0.25;
/// The water a fully loaded brush leaves in one pass at a middling
/// pressure, µm (a brushed wash 20–50 [E], §4).
pub const WASH_UM: f32 = 30.0;
/// The share of the wetted loose pastel under it a wet brush picks up in a
/// pass (0.5 [E]: more than a finger's 0.22, `pastel.rs`).
pub const PICK_WET: f32 = 0.5;
/// How far a brush carries the paste it holds before laying it, mm (an
/// e-fold: 12 [E]).
pub const CARRY_MM: f32 = 12.0;
/// A brush drawn through pastel, mm/s (40–80 [E]).
pub const WASH_SPEED: f32 = 60.0;
/// The walk's step along a path, pixels (`Canvas::walk`).
const STEP_PX: f32 = 0.7;

/// `r` (linear RGB, a reflectance at infinite thickness) with its
/// scattering times `s` (Kubelka–Munk: K/S over `s`).
pub fn km_scatter(r: Rgb, s: f32) -> Rgb {
    if s >= 1.0 {
        return r;
    }
    r.map(|r| {
        let r = r.clamp(1e-4, 0.9999);
        let ks = (1.0 - r) * (1.0 - r) / (2.0 * r) / s.max(1e-3);
        1.0 + ks - (ks * ks + 2.0 * ks).sqrt()
    })
}

/// The scattering of pastel (µm in the tooth) with `water` µm in its
/// pores, as a share of its dry scattering.
pub fn wet_scatter(water: f32, pastel: f32) -> f32 {
    if water <= 0.0 {
        return 1.0;
    }
    let filled = (water / (VOID * pastel).max(1e-6)).min(1.0);
    1.0 - filled * (1.0 - WATER_S)
}

/// A dipped tip's water (mm³) after `minutes` out of the water.
pub fn tip_water(water: f32, minutes: f64) -> f32 {
    if water <= 0.0 {
        return 0.0;
    }
    (water as f64 * (-minutes.max(0.0) / TIP_DRY_MIN as f64).exp()) as f32
}

/// Water (µm) standing on a surface after `dt` minutes of evaporating at
/// `e` µm a minute and sinking at `k` of itself a minute (0 once dry).
fn water_after(w: f32, dt: f32, e: f32, k: f32) -> f32 {
    if k > 0.0 {
        ((w + e / k) * (-k * dt).exp() - e / k).max(0.0)
    } else {
        (w - e * dt).max(0.0)
    }
}

/// Minutes until `w` µm of water is gone (as `water_after`).
fn time_to_dry(w: f32, e: f32, k: f32) -> f32 {
    if w <= 0.0 {
        0.0
    } else if k > 0.0 {
        (1.0 + k * w / e).ln() / k
    } else {
        w / e
    }
}

impl Stick {
    /// The water a soaked tip holds, mm³: its pores over `DIP_MM` of it.
    pub fn tip_capacity_mm3(&self) -> f32 {
        let area = match self.section {
            Section::Round { d_mm } => std::f32::consts::FRAC_PI_4 * d_mm * d_mm,
            Section::Square { side_mm } => side_mm * side_mm,
        };
        TIP_POROSITY * area * DIP_MM
    }

    /// How wet the tip is, 0 (dry) to 1 (soaked).
    pub fn wetness(&self) -> f32 {
        if self.water <= 0.0 { 0.0 } else { (self.water / self.tip_capacity_mm3()).clamp(0.0, 1.0) }
    }
}

/// A wet brush's load (`Canvas::wash_pastel`): the water it holds and
/// what it can (mm³), and the paste of pastel in it (mm³ and its dry color).
#[derive(Clone, Copy, Debug, Default)]
pub struct WashLoad {
    pub water_mm3: f32,
    pub cap_mm3: f32,
    pub paste_mm3: f32,
    pub paste: Rgb,
}

/// The water a brush of `width_mm` holds, mm³ (`BRUSH_WATER_MM3`).
pub fn brush_water_mm3(width_mm: f32) -> f32 {
    BRUSH_WATER_MM3 * width_mm * width_mm
}

impl Canvas {
    /// How fast water standing on the picture sinks into it, a share a
    /// minute: an absorbent sheet's (`Paper::absorbent`); a ground's none.
    fn sink_rate(&self) -> f32 {
        self.paper.as_ref().map_or(0.0, |p| p.absorbent.clamp(0.0, 1.0)) / SINK_MIN
    }

    /// `dt` minutes pass for the water in the pastel (`wait`): it
    /// evaporates and sinks into the sheet; where it is gone the gum sets
    /// (`Cell::gum_set`). Pastel painted over since is sealed: its water
    /// is forgotten. Nothing without the switch or water.
    pub(crate) fn dry_pastel_water(&mut self, dt: f32) {
        if !self.wet_pastel || !(dt > 0.0) || !self.drawing.as_ref().is_some_and(|d| d.damp) {
            return;
        }
        let k = self.sink_rate();
        let d = self.drawing.as_deref_mut().expect("damp");
        let mut damp = false;
        for (i, c) in d.cells.iter_mut().enumerate() {
            if c.water <= 0.0 {
                continue;
            }
            if self.film[i] > c.film + 1e-4 {
                c.water = 0.0;
                continue;
            }
            let before = (c.a, c.reflectance());
            let w = water_after(c.water, dt, EVAP_UM_MIN, k);
            if w > 0.0 {
                c.water = w;
                damp = true;
            } else {
                c.gum_set(GUM_BIND, DRIED_S);
            }
            self.px[i] = c.repaint(self.px[i], before);
        }
        d.damp = damp;
    }

    /// Minutes until the last water in the pastel is gone (`dry`).
    pub(crate) fn pastel_water_minutes(&self) -> f32 {
        if !self.wet_pastel {
            return 0.0;
        }
        let Some(d) = self.drawing.as_ref().filter(|d| d.damp) else { return 0.0 };
        let k = self.sink_rate();
        d.cells.iter().map(|c| time_to_dry(c.water, EVAP_UM_MIN, k)).fold(0.0, f32::max)
    }

    /// Water standing in the pastel at canvas point (x, y), µm (0 off the
    /// canvas or where there is none).
    pub fn pastel_water_at(&self, x: f32, y: f32) -> f32 {
        let f = self.f;
        let (gx, gy) = ((x * f.scale).floor() as isize - f.x0 as isize, (y * f.scale).floor() as isize - f.y0 as isize);
        if gx < 0 || gy < 0 || gx >= f.w as isize || gy >= f.h as isize {
            return 0.0;
        }
        self.drawing.as_ref().map_or(0.0, |d| d.cells[gy as usize * f.w + gx as usize].water)
    }

    /// A wet brush drawn along `pts` (units), touching a band `width` units
    /// wide across the path, pressed `pressure` (0..1). It lays water as it
    /// goes (`WASH_UM` at full load and a middling pressure, less as it
    /// empties); the loose pastel it wets turns to a paste the bristles pick
    /// up (`PICK_WET` of it a pass, as far as the water fills its pores) and,
    /// with the paste they already held, lay down along the path within
    /// about `CARRY_MM`, as a paste (`PASTE_THIN`). Bound pastel stays. Not
    /// under a sheet or on wet paint. Returns (the pastel it laid, mm³; the
    /// hand time, s); what it still holds stays in `load`. Nothing without
    /// the switch.
    pub fn wash_pastel(&mut self, width: f32, pressure: f32, pts: &[(f32, f32)], load: &mut WashLoad) -> (f32, f32) {
        if pts.is_empty() || !self.wet_pastel || !(width.is_finite() && width > 0.0) {
            return (0.0, 0.0);
        }
        let s = self.f.scale;
        let s_mm = self.px_mm();
        // (µm over a pixel to mm³)
        let um_mm3 = s_mm * s_mm * 1e-3;
        let half = width * s * 0.5;
        let press = 0.5 + pressure.clamp(0.0, 1.0);
        let steps = self.walk(pts);
        let lay = (STEP_PX * s_mm / CARRY_MM).min(1.0);
        let pick = (PICK_WET * STEP_PX).min(1.0);
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        let mut d = self.drawing.take().expect("drawing_mut made it");
        let (mut paste, mut col) = (load.paste_mm3.max(0.0) / um_mm3, load.paste);
        let mut laid = 0.0f32;
        for &(c, dir) in &steps {
            let e = (-dir.1, dir.0);
            let nb = ((2.0 * half / STEP_PX).ceil() as usize).max(1);
            let mut band: Vec<usize> = (0..=nb).filter_map(|k| {
                let u = -half + 2.0 * half * k as f32 / nb as f32;
                self.win_px(c.0 + e.0 * u, c.1 + e.1 * u)
            }).collect();
            band.sort_unstable();
            band.dedup();
            band.retain(|&i| !self.sheet_over(i) && self.wet.vol[i] <= 1e-5);
            if band.is_empty() {
                continue;
            }
            let n = band.len() as f32;
            // the water it lays, a visit's share of a pass, as it has it
            let full = if load.cap_mm3 > 0.0 { (load.water_mm3 / load.cap_mm3).clamp(0.0, 1.0) } else { 0.0 };
            let mut per = WASH_UM * full * press * STEP_PX;
            if per * n * um_mm3 > load.water_mm3 {
                per = load.water_mm3.max(0.0) / (n * um_mm3);
            }
            load.water_mm3 = (load.water_mm3 - per * n * um_mm3).max(0.0);
            for &i in &band {
                let cl = &mut d.cells[i];
                if cl.film < 0.0 || film[i] > cl.film + 1e-4 {
                    // bare, or painted over since: a new layer on top
                    cl.restart(film[i], pxs[i]);
                }
                let before = (cl.a, cl.reflectance());
                cl.water += per;
                if cl.loose > 0.0 {
                    // the wetted loose pastel, a paste the bristles take up
                    let filled = (cl.water / (VOID * (cl.loose + cl.bound)).max(1e-6)).min(1.0);
                    let take = cl.loose * pick * filled;
                    if take > 0.0 {
                        let total = paste + take;
                        col = [0, 1, 2].map(|q| (col[q] * paste + cl.r[q] * take) / total.max(1e-9));
                        paste = total;
                        let base = cl.floor + cl.non_pastel - cl.non_pastel_floor;
                        cl.a = (base + (cl.a - base).max(0.0) * (1.0 - take / cl.loose)).max(cl.non_pastel);
                        cl.loose -= take;
                    }
                }
                pxs[i] = cl.repaint(pxs[i], before);
            }
            // and lays a share of the paste it holds over the band
            let give = paste * lay;
            if give > 0.0 {
                let per = give / n;
                let q = 1.0 - (-per / (PARTICLE_UM * PASTE_THIN)).exp();
                for &i in &band {
                    let cl = &mut d.cells[i];
                    let before = (cl.a, cl.reflectance());
                    cl.add_pastel(q, col, per);
                    cl.loose += per;
                    pxs[i] = cl.repaint(pxs[i], before);
                }
                paste -= give;
                laid += give;
                d.sticks = true;
            }
            d.damp = true;
        }
        self.drawing = Some(d);
        self.px = pxs;
        self.film = film;
        load.paste_mm3 = paste * um_mm3;
        load.paste = col;
        let len_mm = (steps.len().max(1) - 1) as f32 * STEP_PX * s_mm;
        (laid * um_mm3, len_mm / WASH_SPEED + crate::pastel::LAND_S + crate::pastel::LIFT_S)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pastel::{Pose, StrokePoint};

    /// A 150 mm sheet of drawing paper, 160 px, with the drawing's optics
    /// and (or not) the switch.
    fn paper(wet: bool) -> Canvas {
        crate::tests::drawing_paper(160).with_drawing_optics(true).with_wet_pastel(wet)
    }

    fn stroke(c: &mut Canvas, st: &mut Stick, x0: f32, y0: f32, x1: f32, y1: f32, force: f32) -> f32 {
        let pose = Pose { force, alt: 60f32.to_radians(), az: 45f32.to_radians(), roll: 0.0 };
        c.stick_stroke(st, &[StrokePoint { x: x0, y: y0, pose, speed: 50.0 }, StrokePoint { x: x1, y: y1, pose, speed: 50.0 }]).volume_mm3
    }

    /// A band of dark pastel across the sheet at y 450–550.
    fn band(c: &mut Canvas) {
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        for k in 0..6 {
            let y = 455.0 + 18.0 * k as f32;
            stroke(c, &mut st, 100.0, y, 900.0, y, 3.0);
        }
    }

    /// How dark (below the paper's 0.55) the sheet is over x 200–800, y0–y1.
    fn darkening(c: &Canvas, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
        let f = c.window();
        c.pixels().iter().enumerate().filter(|(i, _)| {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            x >= x0 && x < x1 && y >= y0 && y < y1
        }).map(|(_, p)| 0.55 - (p[0] + p[1] + p[2]) / 3.0).sum()
    }

    /// Water poked into every cell holding pastel, as a brush that moved
    /// nothing would leave it.
    fn wet_all(c: &mut Canvas, um: f32) {
        let d = c.drawing.as_deref_mut().unwrap();
        for (i, cell) in d.cells.iter_mut().enumerate() {
            if cell.loose + cell.bound > 0.0 {
                let before = (cell.a, cell.reflectance());
                cell.water += um;
                c.px[i] = cell.repaint(c.px[i], before);
            }
        }
        d.damp = true;
    }

    fn water(c: &Canvas) -> f32 {
        c.drawing.as_ref().map_or(0.0, |d| d.cells.iter().map(|c| c.water).sum())
    }

    /// A stick dipped in water lays more than the dry stick at the same
    /// force, darker while it is wet, and leaves water; on the painting's
    /// clock the water dries and the stroke lightens, its tip drying too.
    #[test]
    fn a_wet_stick_lays_more_and_darker_and_dries_on_the_clock() {
        let (mut dry, mut wet) = (paper(true), paper(true));
        let mut a = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        let mut b = a.clone();
        b.water = b.tip_capacity_mm3();
        let (va, vb) = (stroke(&mut dry, &mut a, 100.0, 500.0, 900.0, 500.0, 2.0), stroke(&mut wet, &mut b, 100.0, 500.0, 900.0, 500.0, 2.0));
        assert!(vb > 1.3 * va, "lays more: {vb} vs {va}");
        assert!(b.water > 0.0 && b.water < b.tip_capacity_mm3(), "the tip gave water: {}", b.water);
        assert_eq!((b.hardness, b.wear), (a.hardness, a.wear), "the stick dries back to itself");
        assert!(water(&wet) > 0.0 && water(&dry) == 0.0);
        let f = wet.window();
        let i = wet.drawing.as_ref().unwrap().cells.iter().position(|c| c.water > 0.0).unwrap();
        let (x, y) = (f.ux(i % f.w) + 0.5 / f.scale, f.uy(i / f.w) + 0.5 / f.scale);
        assert!(wet.feel(x, y).ends_with(", wet"), "{}", wet.feel(x, y));
        let (da, db) = (darkening(&dry, 200.0, 800.0, 450.0, 550.0), darkening(&wet, 200.0, 800.0, 450.0, 550.0));
        assert!(db > 1.2 * da, "darker: {db} vs {da}");
        wet.wait(1.0);
        assert!(water(&wet) > 0.0, "still damp after a minute");
        wet.wait(30.0);
        assert_eq!(water(&wet), 0.0, "dry after half an hour");
        assert!(!wet.drawing.as_ref().unwrap().damp);
        let dried = darkening(&wet, 200.0, 800.0, 450.0, 550.0);
        assert!(dried < db, "lighter dry than wet: {dried} vs {db}");
        assert!(tip_water(1.0, 10.0) < 0.01 && tip_water(1.0, 0.0) == 1.0);
    }

    /// The same pastel wetted (without moving it) reads darker while wet,
    /// and once dry is lighter than wet but darker than it was dry.
    #[test]
    fn wetted_pastel_darkens_and_dries_lighter_than_wet_darker_than_dry() {
        let mut c = paper(true);
        band(&mut c);
        let dry = darkening(&c, 200.0, 800.0, 450.0, 550.0);
        wet_all(&mut c, 40.0);
        let wet = darkening(&c, 200.0, 800.0, 450.0, 550.0);
        c.wait(60.0);
        let dried = darkening(&c, 200.0, 800.0, 450.0, 550.0);
        assert!(wet > dried && dried > dry, "wet {wet}, dried {dried}, dry {dry}");
    }

    /// Dried wet pastel is partly bound by its gum: the finger moves less
    /// of it than of dry pastel, and more than of fixed pastel (none); and
    /// a stick takes to it more as to a fixed crust.
    #[test]
    fn dried_wet_pastel_resists_the_finger_between_dry_and_fixed() {
        let mut dry = paper(true);
        band(&mut dry);
        let mut gummed = dry.clone();
        wet_all(&mut gummed, 40.0);
        gummed.wait(60.0);
        assert_eq!(water(&gummed), 0.0);
        let mut fixed = dry.clone();
        fixed.fix_pastel(None, 0.15);
        let moved = |c: &mut Canvas| {
            let before = c.pixels().to_vec();
            c.rub(&[(500.0, 300.0), (500.0, 700.0)], 1.0, None, 40.0);
            c.pixels().iter().zip(&before).map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()).sum::<f32>()
        };
        let (md, mg, mf) = (moved(&mut dry), moved(&mut gummed), moved(&mut fixed));
        assert!(md > 1.3 * mg && mg > 0.0 && mf == 0.0, "dry {md}, dried wet {mg}, fixed {mf}");
        // over a full tooth, the dried wet passage takes more pastel than the dry one
        let (mut dry, mut gummed) = (paper(true), paper(true));
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        for _ in 0..8 {
            stroke(&mut dry, &mut st.clone(), 100.0, 500.0, 900.0, 500.0, 3.0);
            stroke(&mut gummed, &mut st.clone(), 100.0, 500.0, 900.0, 500.0, 3.0);
        }
        wet_all(&mut gummed, 40.0);
        gummed.wait(60.0);
        let (vd, vg) = (stroke(&mut dry, &mut st.clone(), 100.0, 500.0, 900.0, 500.0, 3.0), stroke(&mut gummed, &mut st, 100.0, 500.0, 900.0, 500.0, 3.0));
        assert!(vg > 1.1 * vd, "takes more: {vg} vs {vd}");
    }

    fn brush(water: f32) -> WashLoad {
        WashLoad { water_mm3: water, cap_mm3: brush_water_mm3(10.0), ..WashLoad::default() }
    }

    /// A wet brush drawn out of a band of loose pastel onto bare paper
    /// carries it along its path as a paste: the paper past the band
    /// darkens along the path (and not beside it), the band gives up some of
    /// its loose pastel, and all of it is wet. Water alone on bare paper
    /// shows nothing.
    #[test]
    fn a_wet_brush_moves_loose_pastel_along_its_path() {
        let mut c = paper(true);
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        for k in 0..6 {
            let x = 300.0 + 8.0 * k as f32;
            stroke(&mut c, &mut st, x, 200.0, x, 800.0, 3.0);
        }
        let clean = c.pixels().to_vec();
        let mut load = brush(brush_water_mm3(10.0));
        let f = c.window();
        let row = |c: &Canvas, x0: f32, x1: f32, y: f32| (0..f.w).filter(|&x| f.ux(x) > x0 && f.ux(x) < x1).map(|x| 0.55 - c.pixels()[(y * f.scale) as usize * f.w + x][0]).sum::<f32>();
        let (past, beside) = (row(&c, 400.0, 650.0, 500.0), row(&c, 400.0, 650.0, 300.0));
        assert!(past.abs() < 1e-4 && beside.abs() < 1e-4, "{past} {beside}");
        let (laid, secs) = c.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut load);
        assert!(laid > 0.0 && secs > 0.0);
        assert!(load.water_mm3 < brush_water_mm3(10.0));
        assert!(row(&c, 400.0, 650.0, 500.0) > 0.05, "carried past the band: {}", row(&c, 400.0, 650.0, 500.0));
        assert!(row(&c, 400.0, 650.0, 300.0).abs() < 1e-4, "not beside the path");
        assert!(c.pastel_water_at(500.0, 500.0) > 0.0 && c.pastel_water_at(500.0, 300.0) == 0.0);
        // water alone over bare paper
        let mut bare = paper(true);
        let before = bare.pixels().to_vec();
        bare.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut brush(50.0));
        assert_eq!(bare.pixels(), &before[..]);
        assert!(bare.pastel_water_at(500.0, 500.0) > 0.0);
        bare.wait(60.0);
        assert_eq!(bare.pixels(), &before[..]);
        let _ = clean;
    }

    /// Ground pastel mixed with water on the brush (a paste) lays its color
    /// over bare paper, wet, and partly bound once dry.
    #[test]
    fn a_paste_on_the_brush_lays_its_color_wet() {
        let mut c = paper(true);
        let mut load = WashLoad { paste_mm3: 4.0, paste: [0.6, 0.05, 0.03], ..brush(brush_water_mm3(10.0)) };
        let (laid, _) = c.wash_pastel(40.0, 0.5, &[(200.0, 500.0), (800.0, 500.0)], &mut load);
        // (what it lays it partly picks up again further on: laid counts it each time)
        assert!(laid > 1.0 && load.paste_mm3 < 2.0, "{laid} {}", load.paste_mm3);
        let i = {
            let f = c.window();
            (500.0 * f.scale) as usize * f.w + (400.0 * f.scale) as usize
        };
        let p = c.pixels()[i];
        assert!(p[0] > p[1] + 0.05, "red: {p:?}");
        assert!(c.drawing.as_ref().unwrap().cells[i].water > 0.0);
        c.wait(60.0);
        let cell = c.drawing.as_ref().unwrap().cells[i];
        assert!(cell.bound > 0.0 && cell.loose > 0.0 && cell.water == 0.0);
    }

    /// Without the switch a wet tip draws as the dry stick, bit for bit, a
    /// wet brush does nothing and `wait` leaves the drawing as it was.
    #[test]
    fn without_the_switch_nothing_takes_water() {
        let (mut a, mut b) = (paper(false), paper(false));
        let mut dry = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        let mut wet = dry.clone();
        wet.water = wet.tip_capacity_mm3();
        assert_eq!(stroke(&mut a, &mut dry, 100.0, 500.0, 900.0, 500.0, 2.0), stroke(&mut b, &mut wet, 100.0, 500.0, 900.0, 500.0, 2.0));
        assert_eq!(a.pixels(), b.pixels());
        assert_eq!(wet.water, wet.tip_capacity_mm3());
        let before = b.pixels().to_vec();
        assert_eq!(b.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut brush(50.0)), (0.0, 0.0));
        assert_eq!(b.pixels(), &before[..]);
        assert!(!crate::tests::drawing_paper(160).with_wet_pastel(true).wet_pastel(), "needs the drawing's optics");
    }

    /// The water in the pastel is saved and read back, and dries the same
    /// after; a file whose section is damaged is refused.
    #[test]
    fn a_checkpoint_keeps_the_water() {
        let mut c = paper(true);
        band(&mut c);
        wet_all(&mut c, 20.0);
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut std::io::Cursor::new(&bytes)).unwrap();
        assert!(r.wet_pastel() && r.drawing.as_ref().unwrap().damp);
        assert_eq!(water(&r), water(&c));
        assert_eq!(r.pixels(), c.pixels());
        c.wait(3.0);
        r.wait(3.0);
        assert_eq!(r.pixels(), c.pixels());
        let mut again = Vec::new();
        r.write_state(&mut again, "").unwrap();
        let mut c2 = Vec::new();
        c.write_state(&mut c2, "").unwrap();
        assert_eq!(again, c2);
        // the section's mark, damaged
        let at = bytes.windows(8).position(|w| w == WET_MARK.to_le_bytes()).expect("the section");
        let mut bad = bytes.clone();
        bad[at] ^= 1;
        assert!(Canvas::read_state(&mut std::io::Cursor::new(&bad)).is_err());
        // a canvas without the switch writes no section
        let mut plain = paper(false);
        band(&mut plain);
        let mut b = Vec::new();
        plain.write_state(&mut b, "").unwrap();
        assert!(!b.windows(8).any(|w| w == WET_MARK.to_le_bytes()));
        assert!(!Canvas::read_state(&mut std::io::Cursor::new(&b)).unwrap().0.wet_pastel());
    }

    /// "WETP", as checkpoint.rs marks the section.
    const WET_MARK: u64 = 0x50544557;

    #[test]
    fn water_dries_by_evaporation_and_sinking() {
        // sized: by evaporation alone
        assert_eq!(water_after(10.0, 2.0, 2.5, 0.0), 5.0);
        assert_eq!(water_after(10.0, 4.0, 2.5, 0.0), 0.0);
        assert!((time_to_dry(10.0, 2.5, 0.0) - 4.0).abs() < 1e-6);
        // an absorbent sheet takes it sooner, and the two agree on when
        let t = time_to_dry(30.0, 2.5, 0.5);
        assert!(t < 12.0, "{t}");
        assert!(water_after(30.0, t * 0.999, 2.5, 0.5) > 0.0);
        assert_eq!(water_after(30.0, t * 1.001, 2.5, 0.5), 0.0);
    }

    #[test]
    fn water_darkens_by_the_share_of_the_pores_it_fills() {
        let r = [0.6, 0.4, 0.2];
        assert_eq!(km_scatter(r, wet_scatter(0.0, 10.0)), r);
        let half = km_scatter(r, wet_scatter(0.5 * VOID * 10.0, 10.0));
        let full = km_scatter(r, wet_scatter(VOID * 10.0, 10.0));
        let more = km_scatter(r, wet_scatter(10.0 * VOID * 10.0, 10.0));
        for k in 0..3 {
            assert!(full[k] < half[k] && half[k] < r[k], "{k}");
            assert_eq!(full[k], more[k]);
        }
    }
}
