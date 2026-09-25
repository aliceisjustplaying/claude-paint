//! Cracks lab (test only, ignored): finish a painting's last checkpoint with
//! craquelure variants and measure what reaches the picture: coverage,
//! contrast and density per region (pale thin paint, darks, thick paint),
//! the local field `vary` builds, and the network alone.
//!
//! ```text
//! CRACK_LAB_CKPT=path/to/r10_winter_b_full.glaze.ckpt CRACK_LAB_OUT=dir \
//! CRACK_LAB_ONLY=base,hier0 cargo test --release -p paint crack_lab -- --ignored --nocapture
//! ```
//! The checkpoint is round 10's frozen pond (branch r10-arm2) after its
//! last stage (`glaze`); the lab replays the finish (`Run::finish`:
//! varnish, cracks, relief) with the painting's own settings.

use crate::canvas::Canvas;
use crate::color::{hex, linear_to_srgb};
use super::{Cracks, network, raster_window};
use crate::noise::Fbm;
use crate::pigment::Pigment;
use crate::surface::COAT_UM;
use std::io::Write;

fn relief() -> (f32, f32) {
    env("CRACK_LAB_RELIEF").map_or((0.06, 0.006), |s| (s.parse().unwrap(), 0.006))
}
/// The run's seed (the finish offsets the cracks' seed by it).
const SEED: u64 = 1;

fn env(k: &str) -> Option<String> {
    std::env::var(k).ok().filter(|s| !s.is_empty())
}

/// The canvas after the glaze stage (its checkpoint) and the varnish,
/// before the cracks.
fn prefinish(ckpt: &str, out: &str) -> Canvas {
    let cache = format!("{out}/prefinish.ckpt");
    if let Ok(f) = std::fs::File::open(&cache) {
        return Canvas::read_state(&mut std::io::BufReader::new(f)).unwrap().0;
    }
    let f = std::fs::File::open(ckpt).unwrap();
    let (mut c, _) = Canvas::read_state(&mut std::io::BufReader::new(f)).unwrap();
    // (a stage's checkpoint is saved at its end: this one holds the glaze)
    // Run::finish up to the cracks (Finish::aged)
    c.dry();
    let var = Fbm::new(SEED as u32 + 98, 3, 400.0);
    c.glaze(&Pigment::varnish(hex("#e6d3a4")), None, |x, y| 0.4 + 0.12 * var.get(x, y));
    c.dry();
    let mut w = std::io::BufWriter::new(std::fs::File::create(&cache).unwrap());
    c.write_state(&mut w, "crack_lab").unwrap();
    c
}

fn luma8(p: [f32; 3]) -> f32 {
    255.0 * (0.2126 * linear_to_srgb(p[0]) + 0.7152 * linear_to_srgb(p[1]) + 0.0722 * linear_to_srgb(p[2]))
}

fn save_rgb(path: &str, w: usize, h: usize, f: impl Fn(usize, usize) -> [u8; 3]) {
    let mut buf = vec![0u8; w * h * 3];
    for y in 0..h {
        for x in 0..w {
            buf[(y * w + x) * 3..(y * w + x) * 3 + 3].copy_from_slice(&f(x, y));
        }
    }
    image::save_buffer(path, &buf, w as u32, h as u32, image::ColorType::Rgb8).unwrap();
}

/// A pixel as `Canvas::save` writes it (dithered by whole-canvas pixel).
fn to8(p: [f32; 3], gx: usize, gy: usize) -> [u8; 3] {
    let (gx, gy) = (gx as i64, gy as i64);
    std::array::from_fn(|c| {
        let d = crate::rng::hash2(gx, gy, c as u64 * 7 + 1) - crate::rng::hash2(gx, gy, c as u64 * 7 + 2);
        (linear_to_srgb(p[c]) * 255.0 + d).round().clamp(0.0, 255.0) as u8
    })
}

/// The lab painting's finish (`r10_crackslab.rs`, variant "as delivered").
fn base() -> Cracks {
    Cracks { width_um: Some(16.0), dirt: 0.25, depth_um: 14.0, ..Cracks::aged(0) }
}

fn variants() -> Vec<(&'static str, Cracks)> {
    let b = base();
    vec![
        ("base", b),
        ("vary0", Cracks { vary: 0.0, ..b }),
        ("hier0", Cracks { hierarchy: 0.0, ..b }),
        ("patchy0", Cracks { patchy: 0.0, ..b }),
        ("grime0", Cracks { grime: 0.0, ..b }),
        ("dirt0", Cracks { dirt: 0.0, ..b }),
        ("dirt1", Cracks { dirt: 1.0, ..b }),
        ("depth0", Cracks { depth_um: 0.0, ..b }),
        ("depth40", Cracks { depth_um: 40.0, ..b }),
        ("cup0", Cracks { cupping_um: 0.0, ..b }),
        ("cup40", Cracks { cupping_um: 40.0, ..b }),
        ("corners0", Cracks { corners: false, ..b }),
        ("even", Cracks { width_um: Some(16.0), dirt: 0.25, depth_um: 14.0, vary: 0.0, ..Cracks::even(0) }),
        ("aged", Cracks::aged(0)),
        ("width30", Cracks { width_um: Some(30.0), ..b }),
        ("g30", Cracks { grain: 0.3, ..b }),
    ]
}

/// Region masks from the paint before the cracks.
fn regions(c: &Canvas) -> Vec<(&'static str, Vec<bool>)> {
    let g = c.ground_um;
    let n = c.px.len();
    let y: Vec<f32> = c.px.iter().map(|p| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]).collect();
    let paint: Vec<f32> = c.film.iter().map(|f| (f * COAT_UM - g).max(0.0)).collect();
    let pale = |i: usize| y[i] > 0.3;
    let dark = |i: usize| y[i] < 0.04;
    let thin = |i: usize| paint[i] < 150.0;
    let thick = |i: usize| paint[i] > 700.0;
    vec![
        ("all", (0..n).map(|_| true).collect()),
        ("pale", (0..n).map(pale).collect()),
        ("pale thin", (0..n).map(|i| pale(i) && thin(i)).collect()),
        ("pale thick", (0..n).map(|i| pale(i) && thick(i)).collect()),
        ("mid", (0..n).map(|i| !pale(i) && !dark(i)).collect()),
        ("dark", (0..n).map(dark).collect()),
        ("dark thin", (0..n).map(|i| dark(i) && thin(i)).collect()),
        ("dark thick", (0..n).map(|i| dark(i) && thick(i)).collect()),
    ]
}

fn pct(v: &mut [f32], q: f32) -> f32 {
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(f32::total_cmp);
    v[((v.len() - 1) as f32 * q) as usize]
}

#[test]
#[ignore]
fn crack_lab() {
    let (Some(ckpt), Some(out)) = (env("CRACK_LAB_CKPT"), env("CRACK_LAB_OUT")) else {
        eprintln!("set CRACK_LAB_CKPT and CRACK_LAB_OUT");
        return;
    };
    std::fs::create_dir_all(&out).unwrap();
    let only: Option<Vec<String>> = env("CRACK_LAB_ONLY").map(|s| s.split(',').map(str::to_string).collect());
    let t = std::time::Instant::now();
    let pre = prefinish(&ckpt, &out);
    let (w, h) = (pre.f.w, pre.f.h);
    let px = pre.px_mm();
    eprintln!("prefinish {w}x{h}, {px:.4} mm/px, ground {} µm, {:.1}s", pre.ground_um, t.elapsed().as_secs_f32());
    let regs = regions(&pre);
    // what the painting is made of
    {
        let g = pre.ground_um;
        for (name, m) in &regs {
            let mut ys: Vec<f32> = (0..w * h).filter(|&i| m[i]).map(|i| luma8(pre.px[i])).collect();
            let mut fs: Vec<f32> = (0..w * h).filter(|&i| m[i]).map(|i| pre.film[i] * COAT_UM - g).collect();
            let n = ys.len();
            eprintln!("  region {name:<11} {:>5.1}% of px, luma8 p50 {:.0}, paint µm p10/50/90 {:.0}/{:.0}/{:.0}", 100.0 * n as f32 / (w * h) as f32, pct(&mut ys, 0.5), pct(&mut fs, 0.1), pct(&mut fs, 0.5), pct(&mut fs, 0.9));
        }
    }
    let mut base_l: Option<Vec<f32>> = None;
    let crops = [("sky", 944usize, 108usize, 800usize, 500usize), ("tree", 304, 870, 800, 500), ("corner", w - 500, h - 500, 500, 500), ("cornerTL", 0, 0, 500, 500)];
    let mut report = std::fs::OpenOptions::new().create(true).append(true).open(format!("{out}/report.txt")).unwrap();
    for (name, k) in variants() {
        if only.as_ref().is_some_and(|o| !o.iter().any(|x| x == name)) {
            continue;
        }
        let t = std::time::Instant::now();
        let k = Cracks { seed: k.seed.wrapping_add(SEED), ..k };
        let mut c = pre.clone();
        c.crack(&k);
        c.relief(relief().0, relief().1);
        let tc = t.elapsed().as_secs_f32();
        // the reference: the same finish without the cracks (the veil kept)
        let mut refc = pre.clone();
        if k.veil > 0.0 {
            let px = refc.px_mm();
            refc.varnish_veil(&k, px);
        }
        refc.relief(relief().0, relief().1);
        let ref_l: Vec<f32> = refc.px.iter().map(|&p| luma8(p)).collect();
        drop(refc);
        let l: Vec<f32> = c.px.iter().map(|&p| luma8(p)).collect();
        let d: Vec<f32> = l.iter().zip(&ref_l).map(|(a, r)| a - r).collect();
        let mut lines = vec![format!("== {name} ({tc:.1}s) {k:?}")];
        if name == "base" {
            base_l = Some(l.clone());
        } else if let Some(b) = &base_l {
            let dv: Vec<f32> = l.iter().zip(b).map(|(a, b)| (a - b).abs()).collect();
            let n2 = dv.iter().filter(|&&v| v >= 2.0).count();
            let mean = dv.iter().sum::<f32>() / dv.len() as f32;
            let max = dv.iter().copied().fold(0.0f32, f32::max);
            lines.push(format!("  vs base picture: px differing >=2 levels {:.3}%, mean |diff| {mean:.4}, max {max:.1}", 100.0 * n2 as f32 / dv.len() as f32));
        }
        for (rn, m) in &regs {
            let n = m.iter().filter(|&&b| b).count();
            if n == 0 {
                continue;
            }
            let mut hit: Vec<f32> = (0..w * h).filter(|&i| m[i] && d[i].abs() >= 2.0).map(|i| d[i]).collect();
            let cov = hit.len() as f32 / n as f32;
            let cov6 = (0..w * h).filter(|&i| m[i] && d[i].abs() > 6.0).count() as f32 / n as f32;
            let light = hit.iter().filter(|&&v| v > 0.0).count() as f32 / hit.len().max(1) as f32;
            let mut mag: Vec<f32> = hit.iter().map(|v| v.abs()).collect();
            let sum: f32 = (0..w * h).filter(|&i| m[i]).map(|i| d[i].abs()).sum::<f32>() / n as f32;
            lines.push(format!(
                "  {rn:<11} cover(|d|>=2) {:>5.2}% (>6 {:>5.2}%)  |d| p10/50/90/99 {:>4.1}/{:>4.1}/{:>4.1}/{:>4.1}  lighter {:.2}  mean|d| per px {:.3}",
                100.0 * cov,
                100.0 * cov6,
                pct(&mut mag, 0.1),
                pct(&mut mag, 0.5),
                pct(&mut mag, 0.9),
                pct(&mut mag, 0.99),
                light,
                sum
            ));
            hit.clear();
        }
        // density in ~5 mm tiles over pale thin paint: spread
        let tile = (5.0 / px).round() as usize;
        let (tx, ty) = (w / tile, h / tile);
        let pale = &regs[2].1;
        let mut dens = Vec::new();
        for j in 0..ty {
            for i in 0..tx {
                let (mut n, mut m, mut hitn) = (0, 0, 0);
                for y in j * tile..(j + 1) * tile {
                    for x in i * tile..(i + 1) * tile {
                        let q = y * w + x;
                        n += 1;
                        if pale[q] {
                            m += 1;
                            if d[q].abs() >= 2.0 {
                                hitn += 1;
                            }
                        }
                    }
                }
                if m * 10 >= n * 9 {
                    dens.push(hitn as f32 / m as f32);
                }
            }
        }
        let mean = dens.iter().sum::<f32>() / dens.len().max(1) as f32;
        let sd = (dens.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / dens.len().max(1) as f32).sqrt();
        let med = pct(&mut dens.clone(), 0.5);
        let quiet = dens.iter().filter(|&&v| v < 0.5 * med).count() as f32 / dens.len().max(1) as f32;
        let busy = dens.iter().filter(|&&v| v > 2.0 * med).count() as f32 / dens.len().max(1) as f32;
        lines.push(format!("  pale-thin 5mm tiles: n {} mean {:.2}% cv {:.2} quiet(<med/2) {:.2} busy(>2med) {:.2}", dens.len(), 100.0 * mean, sd / mean.max(1e-9), quiet, busy));
        // the local field vary builds, and the raster's own widths
        let kf = k.fit(pre.ground_um);
        let local = pre.crack_local(&kf);
        let (mut reach, mut open) = (Vec::new(), Vec::new());
        for (rn, m) in regs.iter().filter(|r| ["pale thin", "dark", "pale thick", "dark thick"].contains(&r.0)) {
            let (mut r, mut o) = (Vec::new(), Vec::new());
            for i in (0..w * h).step_by(97).filter(|&i| m[i]) {
                let p = [((i % w) as f32 + 0.5) * px, ((i / w) as f32 + 0.5) * px];
                let (a, b) = local.at(p);
                r.push(a);
                o.push(b);
            }
            reach.push(format!("{rn} {:.2}/{:.2}/{:.2}", pct(&mut r, 0.1), pct(&mut r, 0.5), pct(&mut r, 0.9)));
            open.push(format!("{rn} {:.2}/{:.2}/{:.2}", pct(&mut o, 0.1), pct(&mut o, 0.5), pct(&mut o, 0.9)));
        }
        lines.push(format!("  local reach p10/50/90: {}", reach.join(", ")));
        lines.push(format!("  local open  p10/50/90: {}", open.join(", ")));
        let pitch = pre.linen.map_or([10.0 / 14.0, 10.0 / 12.0], |l| [10.0 / l.warp_per_cm, 10.0 / l.weft_per_cm]);
        let net = network(&kf, [w as f32 * px, h as f32 * px], pitch);
        let r = raster_window(&net, &kf, &local, (0, 0, w, h), px);
        let mut gen_len = [0.0f32; 6];
        for a in &net.arms {
            let l: f32 = a.pts.windows(2).map(|s| ((s[1][0] - s[0][0]).powi(2) + (s[1][1] - s[0][1]).powi(2)).sqrt()).sum();
            gen_len[(a.generation as usize).min(5)] += l;
        }
        let mut cv: Vec<f32> = r.cover.iter().copied().filter(|&v| v > 0.01).collect();
        lines.push(format!(
            "  network: {} arms, length by generation (m) {:.2?}; raster cover>0.01 {:.2}% of px, cover p10/50/90/99 {:.3}/{:.3}/{:.3}/{:.3}; dz p1/p99 {:.1}/{:.1}",
            net.arms.len(),
            gen_len.map(|l| l / 1000.0),
            100.0 * cv.len() as f32 / (w * h) as f32,
            pct(&mut cv, 0.1),
            pct(&mut cv, 0.5),
            pct(&mut cv, 0.9),
            pct(&mut cv, 0.99),
            pct(&mut r.dz.clone(), 0.001),
            pct(&mut r.dz.clone(), 0.999)
        ));
        // the network's own density per region (cover > 0.01: drawn at all)
        let dens: Vec<String> = regs
            .iter()
            .filter(|r| ["pale thin", "pale thick", "mid", "dark", "dark thin", "dark thick"].contains(&r.0))
            .map(|(rn, m)| {
                let n = m.iter().filter(|&&b| b).count().max(1);
                let on = (0..w * h).filter(|&i| m[i] && r.cover[i] > 0.01).count();
                let sum: f32 = (0..w * h).filter(|&i| m[i]).map(|i| r.cover[i]).sum();
                format!("{rn} {:.2}% (open area {:.3}%)", 100.0 * on as f32 / n as f32, 100.0 * sum / n as f32)
            })
            .collect();
        lines.push(format!("  network drawn per region: {}", dens.join(", ")));
        // corners: crack length within 5% of the diagonal from a corner
        // running within 30° of perpendicular to the diagonal
        let (cw, chh) = (w as f32 * px, h as f32 * px);
        let diag = (cw * cw + chh * chh).sqrt();
        let (mut perp, mut all) = (0.0f32, 0.0f32);
        for a in &net.arms {
            for s in a.pts.windows(2) {
                let m = [(s[0][0] + s[1][0]) / 2.0, (s[0][1] + s[1][1]) / 2.0];
                let (cx, cy) = (if m[0] < cw / 2.0 { 0.0 } else { cw }, if m[1] < chh / 2.0 { 0.0 } else { chh });
                let dd = ((m[0] - cx).powi(2) + (m[1] - cy).powi(2)).sqrt();
                if dd > 0.05 * diag {
                    continue;
                }
                let u = [(cw / 2.0 - cx) / (diag / 2.0), (chh / 2.0 - cy) / (diag / 2.0)];
                let v = [s[1][0] - s[0][0], s[1][1] - s[0][1]];
                let l = (v[0] * v[0] + v[1] * v[1]).sqrt();
                if l > 0.0 && ((v[0] * u[0] + v[1] * u[1]) / l).abs() < 0.5 {
                    perp += l;
                }
                all += l;
            }
        }
        lines.push(format!("  corners: {:.2} of crack length within 30° of perpendicular to the diagonal", perp / all.max(1e-6)));
        for l in &lines {
            eprintln!("{l}");
            writeln!(report, "{l}").unwrap();
        }
        // images: crops of the picture, the crack cover alone (whole, halved)
        let dir = format!("{out}/{name}");
        std::fs::create_dir_all(&dir).unwrap();
        for &(cn, x0, y0, cw, ch) in &crops {
            save_rgb(&format!("{dir}/{cn}.png"), cw, ch, |x, y| to8(c.px[(y0 + y) * w + x0 + x], x0 + x, y0 + y));
            // the cracks' effect on the picture, ×8 (dark: darker, red: lighter)
            save_rgb(&format!("{dir}/{cn}_diff.png"), cw, ch, |x, y| {
                let v = d[(y0 + y) * w + x0 + x] * 8.0;
                let a = (255.0 - v.abs()).clamp(0.0, 255.0) as u8;
                if v < 0.0 { [a, a, a] } else { [255, a, a] }
            });
            save_rgb(&format!("{dir}/{cn}_net.png"), cw, ch, |x, y| {
                let v = (255.0 * (1.0 - (r.cover[(y0 + y) * w + x0 + x] * 4.0).min(1.0))) as u8;
                [v, v, v]
            });
        }
        // the whole difference map (f32, row major) and the region codes
        // (0 mid, 1 pale thin, 2 dark, 3 other pale) for offline analysis
        if env("CRACK_LAB_DUMP").is_some() {
            let bytes: Vec<u8> = d.iter().flat_map(|v| v.to_le_bytes()).collect();
            std::fs::write(format!("{dir}/diff_{w}x{h}.f32"), bytes).unwrap();
            let code: Vec<u8> = (0..w * h).map(|i| if regs[2].1[i] { 1 } else if regs[5].1[i] { 2 } else if regs[1].1[i] { 3 } else { 0 }).collect();
            std::fs::write(format!("{out}/regions_{w}x{h}.u8"), code).unwrap();
        }
        let (hw, hh) = (w / 2, h / 2);
        save_rgb(&format!("{dir}/net_half.png"), hw, hh, |x, y| {
            let m = (0..2).flat_map(|j| (0..2).map(move |i| (i, j))).map(|(i, j)| r.cover[(2 * y + j) * w + 2 * x + i]).fold(0.0f32, f32::max);
            let v = (255.0 * (1.0 - (m * 4.0).min(1.0))) as u8;
            [v, v, v]
        });
    }
}

/// Per generation: length, released stress (`Arm::opening`) and where the
/// cracks sit (near a corner or a stretcher bar edge), on the lab canvas.
#[test]
#[ignore]
fn crack_lab_generations() {
    let size = [440.0f32, 308.0];
    for (name, k) in [("base", base()), ("patchy0", Cracks { patchy: 0.0, ..base() }), ("nocorner_novary", Cracks { corners: false, vary: 0.0, ..base() })] {
        let k = Cracks { seed: 1, ..k }.fit(240.0);
        let net = network(&k, size, [10.0 / 14.0, 10.0 / 12.0]);
        let diag = (size[0] * size[0] + size[1] * size[1]).sqrt();
        let bar = (0.1 * size[0].min(size[1])).clamp(30.0, 70.0);
        eprintln!("== {name}: island {:.2} mm, width {:.1} µm", k.island(), k.width());
        for g in 0..5u8 {
            let (mut len, mut near_corner, mut near_bar) = (0.0f32, 0.0f32, 0.0f32);
            let mut rel = Vec::new();
            let mut n = 0;
            for a in net.arms.iter().filter(|a| a.generation == g) {
                n += 1;
                rel.push(a.opening);
                for s in a.pts.windows(2) {
                    let l = ((s[1][0] - s[0][0]).powi(2) + (s[1][1] - s[0][1]).powi(2)).sqrt();
                    let m = [(s[0][0] + s[1][0]) / 2.0, (s[0][1] + s[1][1]) / 2.0];
                    len += l;
                    let dc = [m[0].min(size[0] - m[0]), m[1].min(size[1] - m[1])];
                    if (dc[0] * dc[0] + dc[1] * dc[1]).sqrt() < 0.08 * diag {
                        near_corner += l;
                    } else if (dc[0] - bar).abs() < 5.0 || (dc[1] - bar).abs() < 5.0 {
                        near_bar += l;
                    }
                }
            }
            eprintln!(
                "  gen {g}: {n:>5} arms, {:>6.2} m, near corner {:.2}, near bar edge {:.2}; released stress p10/50/90 {:.2}/{:.2}/{:.2}",
                len / 1000.0,
                near_corner / len.max(1e-6),
                near_bar / len.max(1e-6),
                pct(&mut rel.clone(), 0.1),
                pct(&mut rel.clone(), 0.5),
                pct(&mut rel, 0.9)
            );
        }
    }
}
