//! Fixative sprayed from a can along a path, or in a burst at a point.
//!
//! **The cone.** An aerosol can's actuator throws a full cone of droplets,
//! `cone` degrees across (aerosol full cones 20–30°), from the nozzle held
//! `distance` mm off the sheet (fixative cans: 12–16 in, 300–400 mm). Taken
//! as even in solid angle inside the cone and nothing outside it, the
//! spray lands on the sheet at a point at angle θ off the axis with a flux
//! I·cos³θ / h² (the inverse square of the slant distance, h / cos θ, and
//! the slant at which it meets the plane, cos θ): Q / Ω per unit solid
//! angle, Ω = 2π(1 − cos α) the cone's, α its half angle, Q the solution's
//! mass rate. Swept along a straight path at speed v, the mass laid at
//! distance d from the path is (Q / vΩ) · 2 S cos α / (h² + d²), with
//! S = √(h² tan²α − d²): highest under the nozzle, falling off to nothing
//! at h tan α. The deposit follows the nozzle's path sample by sample, so
//! a turn or an end of the path lays what the geometry gives there.
//!
//! **What it does.** The solution lands as droplets (Sauter mean about
//! 50 µm) that spread into spots wider than themselves; the share of the
//! surface the spots reach is 1 − exp(−N·A), N·A = 1.5 β² V / d for a
//! volume V per area of droplets of diameter d spreading β times their
//! width. That share of the loose drawing and pastel is bound (the eraser
//! no longer lifts it, the next stick files a crust); in the rest it stays
//! loose. The resin wets a share of the bound particles that grows with
//! the dose: 0.15 for a light coat (`LIGHT_COAT_G_M2`, as `fix()` gives the
//! whole surface), darkening them by index matching (pastel.rs
//! `fix_pastel`). A sheet of paper or tape on the picture keeps the spray
//! off what it covers, and paint over a drawing seals it from it.
//! Sources, and which numbers are estimates: notes/research/masking_and_spray.md.

use crate::canvas::Canvas;

/// A spray can's mass rate while spraying, g/s: 0.6 measured for two
/// paint cans (TNO, cited by RIVM's ConsExpo paint fact sheet).
pub const CAN_G_S: f32 = 0.6;
/// The share of what leaves the can that is propellant, flashing off
/// before it lands. ESTIMATE (aerosol paints are formulated with roughly a
/// third to a half propellant; no figure for fixatives was found).
pub const PROPELLANT: f32 = 0.4;
/// The cone's full angle, degrees: aerosol actuators throw a full cone of
/// 20–30°; its middle.
pub const CONE_DEG: f32 = 25.0;
/// The nozzle's distance from the sheet, mm: fixative cans say 12–16 in.
pub const DISTANCE_MM: f32 = 300.0;
/// The hand's sweep, mm/s: the spray painter's rule of about a foot a second.
pub const SWEEP_MM_S: f32 = 300.0;
/// A burst at a point, seconds (when no time is given).
pub const BURST_S: f32 = 0.5;
/// The solution's density, g/cm³ (alcohol and light hydrocarbon solvents,
/// 0.7–0.8, with a few % resin).
const SOLUTION_G_CM3: f32 = 0.8;
/// The droplets' Sauter mean diameter, µm: 20–70 µm for consumer
/// aerosol actuators (US 5,323,935).
const DROPLET_UM: f32 = 50.0;
/// How many times its own width a droplet spreads on the sheet. ESTIMATE
/// (a low-viscosity droplet landing slowly on a porous sheet wets out to
/// a spot a few times its width; no measurement for fixative was found).
const SPREAD: f32 = 2.0;
/// A light coat, g/m² of solution landed: conservation sprays lay 20–50
/// g/m² of a 1–2 % resin solution (notes/research/paper_surface.md §5).
pub const LIGHT_COAT_G_M2: f32 = 20.0;
/// The share of the particles a light coat's resin wets (pastel.rs
/// `fix_pastel`, `fix()` over the whole surface).
pub const LIGHT_COAT_WET: f32 = 0.15;

/// A spray: the nozzle's path (units), how far off the sheet and how wide
/// its cone, and how fast it is swept (or how long a burst lasts).
#[derive(Clone, Debug)]
pub struct Spray {
    pub path: Vec<(f32, f32)>,
    pub distance_mm: f32,
    pub cone_deg: f32,
    pub speed_mm_s: f32,
    pub burst_s: f32,
}

impl Spray {
    pub fn along(path: Vec<(f32, f32)>) -> Self {
        Spray { path, distance_mm: DISTANCE_MM, cone_deg: CONE_DEG, speed_mm_s: SWEEP_MM_S, burst_s: BURST_S }
    }

    /// The radius it reaches on the sheet, mm.
    pub fn reach_mm(&self) -> f32 {
        self.distance_mm * (0.5 * self.cone_deg.to_radians()).tan()
    }

    /// The solution laid at distance `d_mm` from the middle of a long
    /// straight sweep, g/m² (the module's closed form).
    pub fn across_g_m2(&self, d_mm: f32) -> f32 {
        let (h, a) = (self.distance_mm, 0.5 * self.cone_deg.to_radians());
        let s2 = (h * a.tan()).powi(2) - d_mm * d_mm;
        if s2 <= 0.0 {
            return 0.0;
        }
        let omega = 2.0 * std::f32::consts::PI * (1.0 - a.cos());
        let q = CAN_G_S * (1.0 - PROPELLANT);
        q / (self.speed_mm_s * omega) * 2.0 * s2.sqrt() * a.cos() / (h * h + d_mm * d_mm) * 1e6
    }

    /// Hand time, s.
    pub fn secs(&self, mm_per_unit: f32) -> f32 {
        if self.path.len() < 2 {
            return self.burst_s;
        }
        self.path.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt() * mm_per_unit).sum::<f32>() / self.speed_mm_s
    }
}

/// The share of the surface a dose (g/m² of solution) reaches with spots.
pub fn reached(g_m2: f32) -> f32 {
    let v_um = g_m2 / SOLUTION_G_CM3;
    1.0 - (-1.5 * SPREAD * SPREAD * v_um / DROPLET_UM).exp()
}

/// The share of the bound particles the resin wets at a dose (g/m²).
pub fn wetted(g_m2: f32) -> f32 {
    (LIGHT_COAT_WET * g_m2 / LIGHT_COAT_G_M2).min(1.0)
}

impl Canvas {
    /// The solution `sp` lays over the whole canvas, g/m² per whole-canvas
    /// pixel (row major). The nozzle steps along its path no more than a
    /// tenth of its reach and a pixel at a time, each step laying the
    /// cone's flux for the time it takes.
    pub fn spray_dose(&self, sp: &Spray) -> Vec<f32> {
        use rayon::prelude::*;
        let whole = self.f.whole();
        let (fw, fh) = (whole.w, whole.h);
        let mmu = self.mm_per_unit;
        let px_mm = self.px_mm();
        let (h, a) = (sp.distance_mm, 0.5 * sp.cone_deg.to_radians());
        let reach = h * a.tan();
        let omega = 2.0 * std::f32::consts::PI * (1.0 - a.cos());
        let q = CAN_G_S * (1.0 - PROPELLANT);
        // nozzle samples (mm) and the seconds each stands for
        let mut at: Vec<(f32, f32, f32)> = Vec::new();
        if sp.path.len() < 2 {
            let p = sp.path[0];
            at.push((p.0 * mmu, p.1 * mmu, sp.burst_s));
        } else {
            let step = (0.1 * reach).min(px_mm).max(1e-3);
            for w in sp.path.windows(2) {
                let (p0, p1) = ((w[0].0 * mmu, w[0].1 * mmu), (w[1].0 * mmu, w[1].1 * mmu));
                let len = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
                let n = ((len / step).ceil() as usize).max(1);
                let dt = len / n as f32 / sp.speed_mm_s;
                for k in 0..n {
                    let t = (k as f32 + 0.5) / n as f32;
                    at.push((p0.0 + (p1.0 - p0.0) * t, p0.1 + (p1.1 - p0.1) * t, dt));
                }
            }
        }
        let k = q / omega * h * 1e6;
        let r2 = reach * reach;
        let mut dose = vec![0.0f32; fw * fh];
        dose.par_chunks_mut(fw).enumerate().for_each(|(y, row)| {
            let ym = (y as f32 + 0.5) * px_mm;
            for &(sx, sy, dt) in &at {
                let dy = ym - sy;
                if dy * dy > r2 {
                    continue;
                }
                let half = (r2 - dy * dy).sqrt();
                let x0 = (((sx - half) / px_mm).floor().max(0.0)) as usize;
                let x1 = ((((sx + half) / px_mm).ceil()).max(0.0) as usize).min(fw);
                for (x, d) in row.iter_mut().enumerate().take(x1).skip(x0) {
                    let dx = (x as f32 + 0.5) * px_mm - sx;
                    let rho2 = dx * dx + dy * dy;
                    if rho2 <= r2 {
                        *d += k * dt / (h * h + rho2).powf(1.5);
                    }
                }
            }
        });
        dose
    }

    /// Spray fixative (`Spray`): over each pixel the share its spots reach
    /// of the loose drawing and pastel is bound, the resin wetting the
    /// share its dose gives. Refused on a canvas without the drawing's
    /// separate optics (an older painting's). Returns the solution laid on
    /// the canvas, g.
    pub fn spray_fix(&mut self, sp: &Spray) -> Result<f32, String> {
        if !self.drawing_optics {
            return Err("fix along a path needs the drawing's own optics (a painting begun with them)".into());
        }
        if sp.path.is_empty() || sp.path.iter().any(|p| !(p.0.is_finite() && p.1.is_finite())) {
            return Err("the nozzle's path is points (units)".into());
        }
        let dose = self.spray_dose(sp);
        let f = self.f;
        let px_mm = self.px_mm();
        let total = dose.iter().map(|&g| g as f64).sum::<f64>() * (px_mm as f64 * px_mm as f64) * 1e-6;
        let Some(d) = self.drawing.as_mut() else { return Ok(total as f32) };
        let sheet = self.sheet.as_ref();
        let tape = self.tape.as_ref();
        for (i, c) in d.cells.iter_mut().enumerate() {
            let g = dose[f.whole_index(i)];
            if g <= 0.0 || self.film[i] > c.film + 1e-4 || sheet.is_some_and(|s| s.over[i]) || tape.is_some_and(|t| t.raised[i] > 0.0) {
                continue;
            }
            let before = (c.a, c.reflectance());
            c.bind_share(wetted(g), reached(g));
            if self.wet.vol[i] <= 1e-5 {
                self.px[i] = c.repaint(self.px[i], before);
            }
        }
        d.fix_guide_share(|k| {
            let covered = sheet.is_some_and(|s| s.covers_whole_pixel(f, k));
            if covered { 0.0 } else { reached(dose[k]) }
        });
        Ok(total as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The deposit the cone lays across a sweep follows the closed form: a
    /// sweep along a long straight line, read across it at its middle.
    #[test]
    fn the_deposit_falls_off_across_the_path_as_the_cone_predicts() {
        let c = Canvas::new(400, 1.0, [0.8; 3]).with_size_mm(400.0);
        let sp = Spray::along(vec![(-500.0, 500.0), (1500.0, 500.0)]);
        let dose = c.spray_dose(&sp);
        let w = 400;
        let reach = sp.reach_mm();
        assert!((reach - 300.0 * (12.5f32).to_radians().tan()).abs() < 1e-3);
        for d_mm in [0.5, 10.5, 30.5, 50.5, 60.5] {
            let (x, y) = (200usize, (200.0 + d_mm) as usize);
            let got = dose[y * w + x];
            let want = sp.across_g_m2(d_mm);
            assert!((got - want).abs() <= 0.02 * sp.across_g_m2(0.0), "{d_mm} mm: {got} vs {want}");
        }
        // past the cone's reach, nothing
        assert_eq!(dose[(200 + 70) * w + 200], 0.0);
        // and its peak is a light coat's order: tens of g/m²
        let peak = sp.across_g_m2(0.0);
        assert!((5.0..40.0).contains(&peak), "{peak}");
        // farther off, a wider, fainter band
        let far = Spray { distance_mm: 500.0, ..sp.clone() };
        assert!(far.reach_mm() > reach && far.across_g_m2(0.0) < peak);
    }

    /// A burst at a point lays its flux in a disc, highest under the nozzle.
    #[test]
    fn a_burst_lays_a_disc() {
        let c = Canvas::new(200, 1.0, [0.8; 3]).with_size_mm(400.0);
        let sp = Spray { burst_s: 1.0, ..Spray::along(vec![(500.0, 500.0)]) };
        let dose = c.spray_dose(&sp);
        let at = |mm: f32| dose[100 * 200 + 100 + (mm / 2.0) as usize];
        assert!(at(0.0) > at(30.0) && at(30.0) > at(60.0) && at(60.0) > 0.0 && at(80.0) == 0.0);
        let total: f32 = dose.iter().sum::<f32>() * 4.0 * 1e-6;
        let want = CAN_G_S * (1.0 - PROPELLANT);
        assert!((total - want).abs() < 0.03 * want, "{total} g of {want}");
    }

    #[test]
    fn spots_reach_more_of_the_surface_with_more_spray() {
        assert!(reached(0.0) == 0.0 && reached(2.0) < reached(10.0) && reached(10.0) < reached(40.0) && reached(200.0) > 0.999);
        assert_eq!(wetted(LIGHT_COAT_G_M2), LIGHT_COAT_WET);
    }
}
