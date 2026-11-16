//! A plate's grounds, bites and aquatint against the formulas (EP; IP §2–§4).
//! Tiny plates only; expected numbers come from the formulas and the inputs.

use super::*;
use crate::etch;

fn pt(x: f32, y: f32, force: f32) -> Point {
    Point { x, y, force, alt: 80.0, az: 0.0, speed: 20.0 }
}

/// A 20 × 10 mm plate at 0.1 mm a cell.
fn tiny(metal: Metal) -> Plate {
    let temper = if metal == Metal::Copper { Temper::HalfHard } else { Temper::Rolled };
    Plate::new(metal, temper, 20.0, 10.0, 1.3, 45.0, 200, 100)
}

fn ferric(minutes: f64, temp_c: f64) -> BiteSpec {
    BiteSpec { acid: Acid::Ferric, molar: 2.2, temp_c, minutes, face: Face::Down, rock: None, feather: None, fresh: false }
}

/// The plate's metal, plus its chips and what the acid took: what it began with (0).
fn balance(p: &Plate) -> f64 {
    p.metal_um3() + p.chip_um3 + p.bitten_um3()
}

/// A grounded copper plate with a needle line at y 5 mm, x 4–16, and a second at y 7.
fn lined() -> Plate {
    let mut p = tiny(Metal::Copper);
    p.apply(Op::Ground { um: etch::GROUND_UM as f32 });
    p.apply(Op::Needle { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(4.0, 5.0, 0.3), pt(16.0, 5.0, 0.3)] });
    p.apply(Op::Needle { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(4.0, 7.0, 0.3), pt(16.0, 7.0, 0.3)] });
    p
}

/// A bare face in still ferric face down bites down at the flat rate of
/// `etch::transport` all over (an open bite): depth ∝ time, 1.40× for 10 °C
/// (Ea 24.9 kJ/mol), and the metal the plate lost is the bath's.
#[test]
fn a_bare_face_bites_as_the_bath_transports() {
    let rate = |temp: f64| {
        let bath = Bath { acid: Acid::Ferric, molar: 2.2, molar0: 2.2, temp_c: temp, face: Face::Down, rock: None, feather: None };
        etch::transport(&bath, Metal::Copper, 0.02 * 0.01 / (2.0 * 0.03), 0.02, 60.0).rate_um_min
    };
    for (minutes, temp) in [(5.0, 20.0), (10.0, 20.0), (10.0, 30.0)] {
        let mut p = tiny(Metal::Copper);
        let r = p.bite(ferric(minutes, temp));
        let depth = -p.cell(50, 50).level as f64;
        // (the bath's loading over a 20 × 10 mm plate in its tray is a few parts in 10⁵)
        let want = rate(temp) * minutes;
        assert!((depth / want - 1.0).abs() < 1e-3, "{minutes} min at {temp} °C: {depth} µm against {want}");
        assert!((r.rate_um_min / rate(temp) - 1.0).abs() < 1e-9);
        assert!(balance(&p).abs() < 1e-4 * p.bitten_um3(), "metal: {}", balance(&p));
        let tray = p.coats().unwrap().tray.unwrap();
        let want_mol = p.bitten_um3() * 1e-18 / etch::molar(Metal::Copper).1;
        assert!((tray.metal_mol / want_mol - 1.0).abs() < 1e-9, "the bath holds what the plate lost");
    }
    let ten = |t: f64| {
        let mut p = tiny(Metal::Copper);
        p.bite(ferric(10.0, t));
        -p.cell(50, 50).level as f64
    };
    let five = {
        let mut p = tiny(Metal::Copper);
        p.bite(ferric(5.0, 20.0));
        -p.cell(50, 50).level as f64
    };
    assert!((ten(20.0) / five - 2.0).abs() < 1e-3, "depth ∝ time");
    let arrh = (24.9e3f64 / 8.314462618 * (1.0 / 293.15 - 1.0 / 303.15)).exp();
    assert!((ten(30.0) / ten(20.0) / arrh - 1.0).abs() < 1e-3);
}

/// A needle through the ground bares the metal only along its path: the
/// acid opens the lines and leaves every other cell as it was; the line's
/// width at the face is its window plus 2z/K, growing with its depth; it
/// has no burr once bitten; the metal balances.
#[test]
fn a_needle_bares_only_its_lines_and_the_undercut_grows() {
    let mut p = lined();
    let e = p.etch(100, 50);
    let l = e.lines.iter().find(|l| l.lam > 0.0).copied().expect("a line through the ground");
    assert!(l.w0 > l.gw && l.gw > 0.0, "the window is wider than the needle's mark in the metal: {l:?}");
    let before: Vec<(usize, usize, Cell)> = (0..100).flat_map(|y| (0..200).map(move |x| (x, y))).map(|(x, y)| (x, y, p.cell(x, y))).collect();
    let mut widths = Vec::new();
    for _ in 0..3 {
        p.bite(ferric(10.0, 20.0));
        let e = p.etch(100, 50);
        let l = e.lines.iter().find(|l| l.lam > 0.0).unwrap();
        let (_, w) = etch::line_section(l.w0 as f64, l.z as f64, Acid::Ferric.etch_factor());
        assert!((w - (l.w0 as f64 + 2.0 * l.z as f64 / 1.2)).abs() < 1e-3);
        // the cell's openings hold the line: its width bin's mean width is the line's
        let c = p.cell(100, 50);
        let (_, mean_w) = c.openings().map(|(s, w, _)| (s, w)).fold((0.0, 0.0), |a, b| if b.0 > a.0 { b } else { a });
        assert!((mean_w as f64 / w - 1.0).abs() < 1e-3, "{mean_w} against {w}");
        assert_eq!(c.burr_vol, 0.0, "a bitten line has no burr");
        widths.push((l.z, w));
        assert!(balance(&p).abs() < 1e-3 * p.bitten_um3());
    }
    assert!(widths.windows(2).all(|w| w[1].0 > w[0].0 && w[1].1 > w[0].1), "{widths:?}");
    for (x, y, c) in before {
        if !(45..=55).contains(&y) && !(65..=75).contains(&y) {
            assert!(p.cell(x, y) == c, "({x}, {y}) under the ground was bitten");
        }
    }
    assert!(p.cell(100, 30).open_share() == 0.0);
}

/// Stopping-out: a line under the varnish keeps the depth it had; the other
/// bites on (stepped bites: 10 min, stop out, 20 more give 10 and 30 min of
/// depth, IP §4.2).
#[test]
fn stop_out_protects() {
    let mut p = lined();
    p.bite(ferric(10.0, 20.0));
    let first = p.etch(100, 70);
    // varnish over the upper half of the plate (y < 6 mm: the line at 5)
    let (bw, bh) = (200usize, 100usize);
    let mut bits = vec![0u8; (bw * bh).div_ceil(8)];
    for y in 0..60 {
        for x in 0..bw {
            let i = y * bw + x;
            bits[i / 8] |= 1 << (i % 8);
        }
    }
    p.apply(Op::StopOut { w: bw as u32, h: bh as u32, bits });
    let kept = p.cell(100, 50);
    let z_kept = p.etch(100, 50).lines.iter().map(|l| l.z).fold(0.0, f32::max);
    p.bite(ferric(20.0, 20.0));
    assert!(p.cell(100, 50) == kept, "the stopped-out line is as it was");
    assert_eq!(p.etch(100, 50).lines.iter().map(|l| l.z).fold(0.0, f32::max), z_kept);
    let z_on = p.etch(100, 70).lines.iter().map(|l| l.z).fold(0.0, f32::max);
    assert!(z_on > first.lines.iter().map(|l| l.z).fold(0.0, f32::max) * 2.0, "30 min against 10: {z_on} {z_kept}");
}

/// An aquatint: the dust box's grains, melted, bitten in steps. The pits'
/// tone (the ink the yarns leave, IP §4.2) rises with the bite while the dots
/// stand, then falls when the undercut has eaten them (z > K·R for the
/// largest): the pits merge into an open bite. Loose rosin washes off.
#[test]
fn an_aquatint_darkens_then_overbites() {
    let mut p = tiny(Metal::Copper);
    p.apply(Op::Aquatint { per_um2: etch::dust_box(0.6, 30.0, 1.0).iter().map(|&v| v as f32).collect(), seed: 7 });
    let cover: f64 = (0..100).map(|i| p.rosin_cover(i * 2, 50)).sum::<f64>() / 100.0;
    assert!((cover - etch::cover(&etch::dust_box(0.6, 30.0, 1.0), 1.0)).abs() < 0.05, "the cells' cover is the box's: {cover}");
    // speckle: cells differ
    let covers: Vec<f64> = (0..50).map(|i| p.rosin_cover(i * 4, 20)).collect();
    assert!(covers.iter().any(|&c| (c - covers[0]).abs() > 0.01));
    p.apply(Op::Melt);
    let largest = etch::largest_grain(0.6, 30.0) * etch::MELT_SPREAD / 2.0;
    let tone = |p: &Plate| -> f64 {
        (0..100).map(|i| p.cell(i * 2, 50).openings().map(|(s, w, d)| etch::held_um(s as f64, w as f64, d as f64)).sum::<f64>()).sum::<f64>() / 100.0
    };
    let mut tones = Vec::new();
    let mut z = 0.0f64;
    while z < 1.2 * 1.6 * largest {
        p.bite(ferric(1.0, 20.0));
        z = p.etch(100, 50).z as f64;
        tones.push((z, tone(&p)));
        assert!(balance(&p).abs() < 1e-3 * p.bitten_um3());
    }
    let peak = tones.iter().cloned().fold((0.0, 0.0), |a, b| if b.1 > a.1 { b } else { a });
    assert!(peak.1 > 0.1, "a tone: {tones:?}");
    assert!(tones[0].1 < peak.1, "it darkens: {tones:?}");
    let last = tones.last().unwrap();
    assert!(last.0 > 1.2 * largest && last.1 < 0.5 * peak.1, "then lightens, over-bitten: {tones:?}");
    // merged: the face's pits are one open bite, the face lowered to the pillars' tips
    let c = p.cell(100, 50);
    assert!(c.level < 0.0 && c.open_share() > 0.99, "{c:?}");
    // loose rosin floats off in the bath
    let mut q = tiny(Metal::Copper);
    q.apply(Op::Aquatint { per_um2: etch::dust_box(0.6, 30.0, 1.0).iter().map(|&v| v as f32).collect(), seed: 7 });
    assert!(q.bite(ferric(2.0, 20.0)).washed);
    assert!(q.coats().unwrap().dust.is_empty() && q.cell(50, 50).open_share() == 0.0, "an open bite, no pits");
}

/// The section's versions: a plate no etching tool touched is version 1,
/// byte for byte as before, and read; a grounded, bitten, aquatinted plate is
/// version 2 and comes back equal; a damaged one is refused, and an etching
/// tool in a version 1 section too. A crop rebuilt from the vectors keeps the
/// bite.
#[test]
fn plat_v1_still_reads_and_v2_round_trips() {
    let mut p = tiny(Metal::Zinc);
    p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts: vec![Point { x: 2.0, y: 5.0, force: 2.0, alt: 55.0, az: 0.0, speed: 1.0 }, Point { x: 18.0, y: 5.0, force: 2.0, alt: 55.0, az: 0.0, speed: 1.0 }] });
    let mut v1 = Vec::new();
    p.write(&mut v1).unwrap();
    assert_eq!(u64::from_le_bytes(v1[4..12].try_into().unwrap()), 1);
    assert!(Plate::read(&mut v1.as_slice()).unwrap() == p);

    p.apply(Op::Clean);
    p.apply(Op::Ground { um: 4.0 });
    p.apply(Op::Needle { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(4.0, 7.0, 0.3), pt(16.0, 7.0, 0.3)] });
    let mut bits = vec![0u8; 200usize.div_ceil(8) * 100];
    bits[0] = 0xff;
    p.apply(Op::StopOut { w: 200, h: 100, bits: bits[..(200 * 100usize).div_ceil(8)].to_vec() });
    p.bite(BiteSpec { acid: Acid::Nitric, molar: etch::NITRIC_CONC_M / 9.0, temp_c: 18.0, minutes: 3.0, face: Face::Up, rock: Some((5.0, 4.0)), feather: Some(30.0), fresh: false });
    p.apply(Op::Clean);
    p.apply(Op::Aquatint { per_um2: etch::dust_box(0.6, 60.0, 2.0).iter().map(|&v| v as f32).collect(), seed: 3 });
    p.apply(Op::Melt);
    p.bite(BiteSpec { acid: Acid::Dutch, molar: etch::DUTCH_CHLORATE_M, temp_c: 20.0, minutes: 2.0, face: Face::Up, rock: None, feather: None, fresh: true });
    let mut v2 = Vec::new();
    p.write(&mut v2).unwrap();
    assert_eq!(u64::from_le_bytes(v2[4..12].try_into().unwrap()), 2);
    let back = Plate::read(&mut v2.as_slice()).unwrap();
    assert!(back == p, "a version 2 plate comes back as it was");
    let mut again = Vec::new();
    back.write(&mut again).unwrap();
    assert!(again == v2);
    for cut in [v2.len() - 3, v2.len() / 2] {
        assert!(Plate::read(&mut &v2[..cut]).is_err(), "cut at {cut}");
    }
    let mut longer = v2.clone();
    longer.push(0);
    assert!(Plate::read(&mut longer.as_slice()).is_err(), "trailing data");
    // an etching tool in a version 1 section
    let mut forged = v2.clone();
    forged[4..12].copy_from_slice(&1u64.to_le_bytes());
    assert!(Plate::read(&mut forged.as_slice()).is_err());

    // a crop rebuilt at twice the pitch from the vectors: the lines bitten as deep
    let mut q = lined();
    q.bite(ferric(15.0, 20.0));
    let fine = q.rebuilt(2, (150, 80, 250, 160));
    let z = |p: &Plate, x, y| p.etch(x, y).lines.iter().map(|l| l.z).fold(0.0, f32::max);
    assert!((z(&fine, 200, 100) / z(&q, 100, 50) - 1.0).abs() < 1e-5, "{} {}", z(&fine, 200, 100), z(&q, 100, 50));
    let open = |p: &Plate, x0: usize, x1: usize, y0: usize, y1: usize| -> f64 { (y0..y1).flat_map(|y| (x0..x1).map(move |x| (x, y))).map(|(x, y)| p.cell(x, y).groove_vol() as f64).sum::<f64>() * p.pitch_um().powi(2) };
    let (a, b) = (open(&q, 80, 120, 45, 55), open(&fine, 160, 240, 90, 110));
    assert!((a / b - 1.0).abs() < 0.05, "the openings' volume: {a} {b}");
}

/// A grounded plate looks dark, its needle lines bright metal; stop-out
/// darker still; rosin speckles the face.
#[test]
fn a_grounded_plate_looks_dark_with_bright_lines() {
    let lum = |c: crate::Rgb| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    let bare = lum(tiny(Metal::Copper).colour_and_height((0, 0, 1, 1)).0[0]);
    let mut p = tiny(Metal::Copper);
    p.apply(Op::Ground { um: 4.0 });
    p.apply(Op::Needle { tip_um: 60.0, cone_deg: 30.0, pts: vec![pt(4.0, 5.05, 0.3), pt(16.0, 5.05, 0.3)] });
    let (col, _) = p.colour_and_height((0, 0, 200, 100));
    let (ground, line) = (lum(col[30 * 200 + 100]), lum(col[50 * 200 + 100]));
    assert!(ground < 0.75 * bare, "the ground {ground} against bare {bare}");
    assert!(line > ground, "the line {line} shows metal through the ground {ground}");
    let mut q = tiny(Metal::Copper);
    q.apply(Op::Aquatint { per_um2: etch::dust_box(0.6, 15.0, 1.0).iter().map(|&v| v as f32).collect(), seed: 5 });
    q.apply(Op::Melt);
    let (col, _) = q.colour_and_height((0, 0, 200, 1));
    let l: Vec<f32> = col.iter().map(|&c| lum(c)).collect();
    let mean = l.iter().sum::<f32>() / l.len() as f32;
    let var = l.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / l.len() as f32;
    assert!(var.sqrt() > 0.002 * mean, "a speckle: {} of {mean}", var.sqrt());
}
