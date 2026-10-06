//! Paper as a support (engine 6): a sheet made the way paper is made, not a
//! texture. Sources: notes/research/paper_surface.md.
//!
//! - **Fibres.** The sheet is a random fibre network (Kallmes–Corte; Dodson;
//!   Sampson): fibre centres land as a Poisson process, each fibre a
//!   segment of a length drawn from the pulp's distribution, a width ω and a
//!   coarseness δ (mass per length), at a uniform angle. Each fibre's mass
//!   is laid along its length into the pixels it crosses, so the local
//!   grammage (and its correlations along fibres and between them) comes
//!   from the fibres themselves. Mean coverage is c̄ = βω/δ (β the
//!   grammage): about 25 fibres over a point for a 160 g/m² sheet.
//! - **Flocs.** Long fibres flocculate in the suspension: a share of the
//!   fibres lands in clusters (a Neyman–Scott process: floc centres
//!   Poisson, each floc's fibres scattered round its centre), which gives
//!   the cloudy formation at a few mm.
//! - **The mould.** Over the wires of a laid mould the sheet drains faster
//!   and carries less fibre: fibre centres there are thinned (laid lines
//!   about 1 mm apart, chain lines about 25 mm apart, wandering).
//! - **Pressing.** Wet pressing turns most of the thickness variation into
//!   density variation: the surface follows the local fibre count to a
//!   power below one (T ∝ c^0.5–0.8).
//! - **Felt.** A sheet couched and pressed on wool felt takes the felt's
//!   cells (the "grain"): cells about 0.5–1 mm across with raised rims, tens
//!   of µm deep. The cells are the felt's (seeded Poisson, nearest-seed
//!   cells); the depth and cell size are estimates (no profilometry of
//!   artists' papers was found).
//! - **Calendering** presses the highest spots down.
//! - **Micro-roughness.** Below a pixel (0.1–0.3 mm), the surface between the
//!   fibre tops is a set of pores whose depth below the top envelope is
//!   exponentially distributed with a mean about the collapsed fibre
//!   thickness (Dodson; Sampson–Wang). It is kept as that mean, µ, per
//!   pixel: the bearing (Abbott) curve a stick or a finger meets is
//!   1 − e^(−d/µ) at depth d.
//!
//! Everything is laid in millimetres of the sheet, so the paper is the same
//! at any pixel width and in a crop render.

use crate::rng::{Rng, hash2};
use rayon::prelude::*;

/// The grain a sheet takes from the felt it was couched and pressed on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Felt {
    /// Cell size, mm (the distance between felt knuckles).
    pub cell_mm: f32,
    /// Depth of the imprint, µm (rims to cell floors).
    pub depth_um: f32,
}

/// The wires of a laid mould.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Laid {
    /// Laid lines per cm.
    pub per_cm: f32,
    /// Chain-line spacing, mm.
    pub chain_mm: f32,
    /// Share of the fibre missing over a wire (0.1–0.3 on handmade sheets).
    pub deficit: f32,
}

/// A sheet of paper.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Paper {
    /// Grammage, g/m².
    pub grammage: f32,
    /// Mean fibre length, mm (cotton rag after beating 1–3, softwood 2–3.5,
    /// hardwood 0.7–1.2).
    pub fibre_mm: f32,
    /// Fibre width, µm.
    pub fibre_um: f32,
    /// Collapsed fibre thickness, µm (4–10).
    pub thick_um: f32,
    /// Coarseness, mg per m of fibre (0.06–0.25).
    pub coarseness: f32,
    /// Sheet porosity (0.45–0.6).
    pub porosity: f32,
    /// Share of the fibres that land in flocs (0..1).
    pub floc: f32,
    /// Floc radius, mm.
    pub floc_mm: f32,
    /// How the surface follows the fibre count after pressing: T ∝ c^press
    /// (0.5 hard-pressed .. 0.8 soft).
    pub press: f32,
    /// Calendering, 0 (none) .. 1 (the highest spots pressed right down).
    pub calender: f32,
    pub felt: Option<Felt>,
    pub laid: Option<Laid>,
    /// How much of the sheet's pore volume takes oil (sizing slows water,
    /// barely oil): 0..1.
    pub absorbent: f32,
    /// Through-thickness (z) modulus, MPa: how the sheet gives under a
    /// stick (39 MPa measured on board, Mann, Baum & Habeger 1979; a soft
    /// drawing paper lower).
    pub z_mpa: f32,
    pub seed: u64,
}

impl Paper {
    /// A 160 g/m² cotton-rich drawing paper with a felt grain: about the
    /// pastel papers of the colourmen (estimates where no measurement was
    /// found; notes/research/paper_surface.md).
    pub fn drawing(seed: u64) -> Paper {
        Paper {
            grammage: 160.0,
            fibre_mm: 2.0,
            fibre_um: 25.0,
            thick_um: 6.0,
            coarseness: 0.16,
            porosity: 0.5,
            floc: 0.5,
            floc_mm: 1.5,
            press: 0.65,
            calender: 0.2,
            felt: Some(Felt { cell_mm: 0.8, depth_um: 20.0 }),
            laid: None,
            absorbent: 0.8,
            z_mpa: 39.0,
            seed,
        }
    }

    /// Mean number of fibres over a point: c̄ = βω/δ.
    pub fn coverage(&self) -> f32 {
        // β g/m² = β·1e-3 mg/mm²; δ mg/m = δ·1e-3 mg/mm; ω µm = ω·1e-3 mm
        (self.grammage * 1e-3) * (self.fibre_um * 1e-3) / (self.coarseness * 1e-3)
    }

    /// Mean caliper, µm: c̄·t_f / (1 − ε).
    pub fn caliper_um(&self) -> f32 {
        self.coverage() * self.thick_um / (1.0 - self.porosity).max(0.05)
    }

    /// Fibres per mm² of sheet: β / (δ·λ̄).
    fn fibres_per_mm2(&self) -> f32 {
        (self.grammage * 1e-3) / (self.coarseness * 1e-3 * self.fibre_mm)
    }

    /// How much of a laid mould's wires lie under a point (mm): 0 between
    /// wires, 1 on one.
    fn wire(&self, xm: f32, ym: f32) -> f32 {
        let Some(l) = self.laid else { return 0.0 };
        // laid wires run across the sheet (horizontal), about 0.4 mm thick
        let pitch = 10.0 / l.per_cm.max(1.0);
        // (a non-negative phase: fibres of the halo above the sheet's top
        // edge see the same wires)
        let fy = (ym / pitch).rem_euclid(1.0);
        let d = fy.min(1.0 - fy) * pitch;
        let laid = (1.0 - d / 0.2).max(0.0);
        // chain wires run down it, about 0.5 mm thick, wandering a little
        // (each chain line is a wire of its own: its offset is the mould's)
        let k = (xm / l.chain_mm).floor();
        let off = 0.12 * l.chain_mm * (hash2(k as i64, 7, self.seed ^ 0xC4A1) - 0.5);
        let xc = (k + 0.5) * l.chain_mm + off;
        let chain = (1.0 - (xm - xc).abs() / 0.25).max(0.0);
        laid.max(chain)
    }
}

/// Fibre centres are generated per square cell of the sheet this big (mm),
/// each cell from its own seed, so any part of the sheet can be laid
/// without the rest.
const CELL_MM: f32 = 4.0;

/// The sheet over a window of pixels: surface height (µm, about 0 mean) and
/// the micro-roughness µ (µm) per pixel, and the grammage (g/m²).
pub struct Sheet {
    pub height: Vec<f32>,
    pub micro: Vec<f32>,
    pub grammage: Vec<f32>,
}

/// Poisson count with mean `m` (Knuth for small, normal approximation for
/// large means).
fn poisson(rng: &mut Rng, m: f32) -> u32 {
    if m <= 0.0 {
        return 0;
    }
    if m > 40.0 {
        return (m + m.sqrt() * rng.normal()).round().max(0.0) as u32;
    }
    let l = (-m).exp();
    let (mut k, mut p) = (0u32, 1.0f32);
    loop {
        p *= rng.f();
        if p <= l {
            return k;
        }
        k += 1;
    }
}

/// Lay the sheet over the window `(x0, y0, w, h)` of a canvas whose pixels
/// are `px_mm` mm.
pub fn lay(p: &Paper, x0: usize, y0: usize, w: usize, h: usize, px_mm: f32) -> Sheet {
    let n = w * h;
    let mut gram = vec![0.0f32; n];
    let per_mm2 = p.fibres_per_mm2();
    // the fibres' reach beyond their cell: half the longest fibre (lengths
    // are held to 4 means) and a floc's widest spread (`Rng::normal` is at
    // most 3.46), so a crop lays exactly the fibres the whole sheet does
    let reach_mm = 2.0 * p.fibre_mm + 3.5 * p.floc_mm;
    let band = 48usize;
    let bands: Vec<(usize, usize)> = (0..h).step_by(band).map(|b| (b, (b + band).min(h))).collect();
    let mass_per_mm = p.coarseness * 1e-3; // mg per mm of fibre
    let px_area = px_mm * px_mm;
    let parts: Vec<(usize, Vec<f32>)> = bands
        .par_iter()
        .map(|&(b0, b1)| {
            let rows = b1 - b0;
            let mut g = vec![0.0f32; rows * w];
            // the band in sheet mm, grown by the reach
            let (bx0, bx1) = (x0 as f32 * px_mm - reach_mm, (x0 + w) as f32 * px_mm + reach_mm);
            let (by0, by1) = ((y0 + b0) as f32 * px_mm - reach_mm, (y0 + b1) as f32 * px_mm + reach_mm);
            let (cx0, cx1) = ((bx0 / CELL_MM).floor() as i64, (bx1 / CELL_MM).floor() as i64);
            let (cy0, cy1) = ((by0 / CELL_MM).floor() as i64, (by1 / CELL_MM).floor() as i64);
            let mut lay_fibre = |cx: f32, cy: f32, ang: f32, len: f32| {
                // the fibre's mass laid along it, half a pixel at a time
                let steps = ((len / (0.5 * px_mm)).ceil() as usize).max(1);
                let (dx, dy) = (ang.cos() * len, ang.sin() * len);
                let m = mass_per_mm * len / steps as f32 / px_area; // mg/mm² per step
                for s in 0..steps {
                    let t = (s as f32 + 0.5) / steps as f32 - 0.5;
                    let (xm, ym) = (cx + dx * t, cy + dy * t);
                    // into the four pixels round the point (area weights)
                    let (fx, fy) = (xm / px_mm - 0.5 - x0 as f32, ym / px_mm - 0.5 - (y0 + b0) as f32);
                    let (ix, iy) = (fx.floor(), fy.floor());
                    let (ax, ay) = (fx - ix, fy - iy);
                    for (oy, wy) in [(0i64, 1.0 - ay), (1, ay)] {
                        let yy = iy as i64 + oy;
                        if yy < 0 || yy >= rows as i64 {
                            continue;
                        }
                        for (ox, wx) in [(0i64, 1.0 - ax), (1, ax)] {
                            let xx = ix as i64 + ox;
                            if xx < 0 || xx >= w as i64 {
                                continue;
                            }
                            g[yy as usize * w + xx as usize] += m * wx * wy;
                        }
                    }
                }
            };
            for cy in cy0..=cy1 {
                for cx in cx0..=cx1 {
                    let mut rng = Rng::new(p.seed ^ (cx as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (cy as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
                    let area = CELL_MM * CELL_MM;
                    let (ox, oy) = (cx as f32 * CELL_MM, cy as f32 * CELL_MM);
                    // a fibre's length: lognormal about the mean (sd 0.4 in ln)
                    let len = |rng: &mut Rng| (p.fibre_mm * (0.4 * rng.normal() - 0.08).exp()).clamp(0.1 * p.fibre_mm, 4.0 * p.fibre_mm);
                    // the mould thins fibre over its wires
                    let keep = |rng: &mut Rng, x: f32, y: f32| p.laid.is_none_or(|l| rng.f() >= l.deficit * p.wire(x, y));
                    // free fibres
                    let nf = poisson(&mut rng, per_mm2 * area * (1.0 - p.floc));
                    for _ in 0..nf {
                        let (x, y) = (ox + rng.f() * CELL_MM, oy + rng.f() * CELL_MM);
                        let (a, l) = (rng.f() * std::f32::consts::PI, len(&mut rng));
                        if keep(&mut rng, x, y) {
                            lay_fibre(x, y, a, l);
                        }
                    }
                    // flocs: about 200 fibres each, round their centre
                    if p.floc > 0.0 {
                        let per_floc = 200.0;
                        let nflocs = poisson(&mut rng, per_mm2 * area * p.floc / per_floc);
                        for _ in 0..nflocs {
                            let (fx, fy) = (ox + rng.f() * CELL_MM, oy + rng.f() * CELL_MM);
                            let k = poisson(&mut rng, per_floc);
                            for _ in 0..k {
                                let (x, y) = (fx + p.floc_mm * rng.normal(), fy + p.floc_mm * rng.normal());
                                let (a, l) = (rng.f() * std::f32::consts::PI, len(&mut rng));
                                if keep(&mut rng, x, y) {
                                    lay_fibre(x, y, a, l);
                                }
                            }
                        }
                    }
                }
            }
            (b0, g)
        })
        .collect();
    for (b0, g) in parts {
        gram[b0 * w..b0 * w + g.len()].copy_from_slice(&g);
    }
    // mg/mm² → g/m²
    gram.iter_mut().for_each(|v| *v *= 1000.0);
    // the surface: pressed, so it follows the fibre count to a power below one
    let (beta, cal) = (p.grammage, p.caliper_um());
    let mut height: Vec<f32> = gram.par_iter().map(|&g| cal * ((g / beta).max(0.0).powf(p.press) - 1.0)).collect();
    // the felt's cells
    if let Some(f) = p.felt {
        height.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                let (xm, ym) = ((x0 + x) as f32 * px_mm + 0.5 * px_mm, (y0 + y) as f32 * px_mm + 0.5 * px_mm);
                *v += felt_um(xm, ym, &f, p.seed);
            }
        });
    }
    // calendering: the spots standing above the sheet's mean surface (0)
    // pressed toward it (in sheet terms, not the window's: a crop is the
    // same paper)
    if p.calender > 0.0 {
        height.par_iter_mut().for_each(|v| {
            if *v > 0.0 {
                *v -= p.calender * *v;
            }
        });
    }
    // the pores between the fibre tops: mean depth about the fibre thickness
    let micro = vec![p.thick_um; n];
    Sheet { height, micro, grammage: gram }
}

/// The felt's imprint at a point (mm), µm: cells pressed in, their rims
/// standing (nearest-seed cells round the felt's knuckles, one seed per
/// cell-sized square of the felt).
fn felt_um(xm: f32, ym: f32, f: &Felt, seed: u64) -> f32 {
    let (u, v) = (xm / f.cell_mm, ym / f.cell_mm);
    let (iu, iv) = (u.floor() as i64, v.floor() as i64);
    let (mut d1, mut d2) = (f32::MAX, f32::MAX);
    for j in -2..=2 {
        for i in -2..=2 {
            let (cu, cv) = (iu + i, iv + j);
            let sx = cu as f32 + hash2(cu, cv, seed ^ 0xFE17);
            let sy = cv as f32 + hash2(cu, cv, seed ^ 0x0F31);
            let d = ((u - sx).powi(2) + (v - sy).powi(2)).sqrt();
            if d < d1 {
                d2 = d1;
                d1 = d;
            } else if d < d2 {
                d2 = d;
            }
        }
    }
    // distance to the cell wall, in cells; the rim is about a fifth of a cell
    let e = 0.5 * (d2 - d1);
    let rim = (1.0 - e / 0.2).max(0.0);
    f.depth_um * (rim * rim - 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stats(v: &[f32]) -> (f64, f64) {
        let n = v.len() as f64;
        let m = v.iter().map(|&x| x as f64).sum::<f64>() / n;
        let var = v.iter().map(|&x| (x as f64 - m).powi(2)).sum::<f64>() / n;
        (m, var.sqrt())
    }

    /// The fibres laid add up to the sheet's grammage, and the point CV
    /// at a scale below a fibre length is of the order the network theory
    /// gives (a few tens of percent at 0.1 mm, less at 0.5 mm).
    #[test]
    fn the_fibres_make_the_grammage() {
        let p = Paper { floc: 0.0, felt: None, calender: 0.0, ..Paper::drawing(3) };
        let s = lay(&p, 40, 40, 200, 200, 0.1);
        let (m, sd) = stats(&s.grammage);
        assert!((m - 160.0).abs() < 8.0, "mean grammage {m}");
        let cv = sd / m;
        assert!(cv > 0.05 && cv < 0.6, "cv at 0.1 mm {cv}");
        let s5 = lay(&p, 8, 8, 40, 40, 0.5);
        let (m5, sd5) = stats(&s5.grammage);
        assert!(sd5 / m5 < cv, "cv falls with zone size: {} vs {}", sd5 / m5, cv);
    }

    /// The same sheet at two pixel sizes has the same grammage at the
    /// coarser scale (it is laid in mm, not in pixels), and a crop is the
    /// same paper as the whole.
    #[test]
    fn the_sheet_is_laid_in_millimetres() {
        let p = Paper::drawing(5);
        let whole = lay(&p, 0, 0, 60, 60, 0.25);
        let part = lay(&p, 20, 10, 20, 30, 0.25);
        for y in 0..30 {
            for x in 0..20 {
                let (a, b) = (whole.height[(y + 10) * 60 + x + 20], part.height[y * 20 + x]);
                let (ga, gb) = (whole.grammage[(y + 10) * 60 + x + 20], part.grammage[y * 20 + x]);
                assert!((ga - gb).abs() < 1e-3, "grammage at {x},{y}: {ga} vs {gb}");
                assert!((a - b).abs() < 1e-3, "height at {x},{y}: {a} vs {b}");
            }
        }
    }

    /// Flocculated and felted sheets are rougher at the mm scale than a
    /// random, calendered one.
    #[test]
    fn flocs_and_felt_roughen_the_sheet() {
        let smooth = Paper { floc: 0.0, felt: None, calender: 0.8, ..Paper::drawing(9) };
        let rough = Paper { floc: 0.7, calender: 0.0, ..Paper::drawing(9) };
        let (_, a) = stats(&lay(&smooth, 0, 0, 120, 120, 0.25).height);
        let (_, b) = stats(&lay(&rough, 0, 0, 120, 120, 0.25).height);
        assert!(b > 1.5 * a, "rough {b} smooth {a}");
    }
}

#[cfg(test)]
mod measure {
    use super::*;
    #[test]
    #[ignore]
    fn print_roughness() {
        for (name, p) in [("drawing", Paper::drawing(3)), ("no felt", Paper { felt: None, ..Paper::drawing(3) })] {
            for px in [0.125f32, 0.26] {
                let n = (40.0 / px) as usize;
                let s = lay(&p, 0, 0, n, n, px);
                let m = s.height.iter().sum::<f32>() / s.height.len() as f32;
                let ra = s.height.iter().map(|v| (v - m).abs()).sum::<f32>() / s.height.len() as f32;
                let (lo, hi) = s.height.iter().fold((f32::MAX, f32::MIN), |(a, b), &v| (a.min(v), b.max(v)));
                let gm = s.grammage.iter().sum::<f32>() / s.grammage.len() as f32;
                let gcv = (s.grammage.iter().map(|g| (g - gm).powi(2)).sum::<f32>() / s.grammage.len() as f32).sqrt() / gm;
                println!("{name} px {px}: Ra {ra:.1} µm, range {lo:.0}..{hi:.0}, grammage cv {gcv:.3}");
            }
        }
    }
}
