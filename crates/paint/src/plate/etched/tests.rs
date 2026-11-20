//! A plate's grounds, bites and aquatint against the formulas (EP; IP §2–§4).
//! Tiny plates only; expected numbers come from the formulas and the inputs.

use super::*;
use crate::etch;

fn pt(x: f32, y: f32, force: f32) -> Point {
    Point { x, y, force, alt: 80.0, az: 0.0, speed: 20.0 }
}

/// A hard ground `um` thick rolled once across the whole of a `tiny` plate
/// (a 150 mm roller's line of contact spans its height).
fn rolled(um: f32) -> Op {
    Op::Ground { um, roller_mm: etch::ROLLER_MM as f32, pts: vec![pt(0.0, 5.0, 0.5), pt(20.0, 5.0, 0.5)] }
}

/// Stopping-out varnish brushed along y `y` mm from x 2 to 18 with a size 6
/// round at pressure 0.5: a strip some 2.3 mm wide.
fn varnish(y: f32) -> Op {
    Op::StopOut { brush_mm: etch::STOP_BRUSH_MM as f32, load: 1.0, pts: vec![pt(2.0, y, 0.5), pt(18.0, y, 0.5)] }
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
    p.apply(rolled(etch::GROUND_UM as f32));
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
    // varnish brushed along the line at 5 mm (not the one at 7)
    p.apply(varnish(5.0));
    assert!(p.etch(100, 50).stopped && !p.etch(100, 70).stopped);
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
    p.apply(rolled(4.0));
    p.apply(Op::Needle { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(4.0, 7.0, 0.3), pt(16.0, 7.0, 0.3)] });
    p.apply(varnish(2.0));
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
    p.apply(rolled(4.0));
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

/// The bath on the clock: a plate put in at one time on its clock and lifted
/// out 15 minutes later is the plate of a 15-minute bite in one, bit for bit,
/// whenever it went in; a plate still in its bath goes through its section
/// and comes out the same; a look at it in the bath sees what a lift then
/// would give and leaves it in; it can't go in twice, nor come out unless in.
#[test]
fn a_bath_on_the_clock_is_the_bite_in_one() {
    let mut one = lined();
    one.bite(ferric(15.0, 20.0));
    // (a clock that has had many odd minutes added: the 15 minutes come back a hair off)
    let t0 = 0.1 + 0.2 + 1e3 / 7.0;
    let t1 = (t0 + 0.11) + 14.89;
    assert!(t1 - t0 != 15.0, "the clock's sums aren't exact here: {}", t1 - t0);
    let mut p = lined();
    let im = p.immerse(ferric(0.0, 20.0).bath(), t0).unwrap();
    assert!(im.fresh && !im.washed);
    assert!(p.in_bath().is_some());
    assert!(p.immerse(ferric(0.0, 20.0).bath(), t0 + 1.0).unwrap_err().contains("already"));
    // in its section, still in the bath
    let mut b = Vec::new();
    p.write(&mut b).unwrap();
    let mut back = Plate::read(&mut b.as_slice()).unwrap();
    assert!(back == p, "a plate in its bath comes back in it");
    // a look half way: a copy bitten to then; the plate is still in, unbitten
    let half = p.as_lifted(t0 + 7.5);
    assert!(half.in_bath().is_none() && p.in_bath().is_some());
    let z = |p: &Plate| p.etch(100, 50).lines.iter().map(|l| l.z).fold(0.0, f32::max);
    assert!(z(&half) > 0.0 && z(&p) == 0.0);
    let r = p.lift(t1).unwrap();
    assert_eq!(r.minutes, 15.0);
    assert!(p == one, "in at {t0}, out at {t1}: the 15-minute bite");
    back.lift(t1).unwrap();
    assert!(back == one, "and through a save while in the bath");
    assert!(p.lift(t1 + 1.0).unwrap_err().contains("isn't in"));
    assert!(z(&half) < z(&one));
}

/// Taken out, stopped out and put back into the same (loaded) tray: the lines
/// under the varnish keep the depth of the first bath; a +10 °C bath bites
/// 1.40 times as fast in the open (Ea 24.9 kJ/mol); straight in and out bites
/// nothing but washes loose rosin off.
#[test]
fn out_stopped_out_and_back_in_keeps_the_first_depths() {
    let mut p = lined();
    p.immerse(ferric(0.0, 20.0).bath(), 100.0).unwrap();
    p.lift(110.0).unwrap();
    p.apply(varnish(5.0));
    let kept = (p.cell(100, 50), p.etch(100, 50));
    let tray = p.coats().unwrap().tray.unwrap();
    let im = p.immerse(ferric(0.0, 20.0).bath(), 130.0).unwrap();
    assert!(!im.fresh && im.tray == tray, "back into the tray it came out of");
    p.lift(150.0).unwrap();
    assert!((p.cell(100, 50), p.etch(100, 50)) == kept, "the stopped-out line keeps its first depth");
    let z = |p: &Plate, x, y| p.etch(x, y).lines.iter().map(|l| l.z).fold(0.0, f32::max);
    assert!(z(&p, 100, 70) > 2.0 * z(&p, 100, 50));

    let open = |temp: f64| {
        let mut q = tiny(Metal::Copper);
        q.immerse(ferric(0.0, temp).bath(), 5.0).unwrap();
        q.lift(15.0).unwrap();
        -q.cell(50, 50).level as f64
    };
    let arrh = (24.9e3f64 / 8.314462618 * (1.0 / 293.15 - 1.0 / 303.15)).exp();
    assert!((arrh - 1.40).abs() < 0.01);
    assert!((open(30.0) / open(20.0) / arrh - 1.0).abs() < 1e-3, "{} {}", open(30.0), open(20.0));

    let mut q = tiny(Metal::Copper);
    q.apply(Op::Aquatint { per_um2: etch::dust_box(0.6, 15.0, 1.0).iter().map(|&v| v as f32).collect(), seed: 5 });
    let before = q.cell(50, 50);
    assert!(q.immerse(ferric(0.0, 20.0).bath(), 1.0).unwrap().washed);
    let r = q.lift(1.0).unwrap();
    assert!(r.minutes == 0.0 && r.metal_um3 == 0.0 && q.cell(50, 50) == before);
    assert!(q.coats().unwrap().dust.is_empty());
    assert!(q.rebuilt(1, (0, 0, 200, 100)).coats().is_none_or(|c| c.dust.is_empty()), "the vectors wash it off too");
}

/// A plate forgotten in its bath bites on past a day in longer steps; a big
/// open plate spends its tray (Dutch mordant, 0.16 M chlorate, takes 0.03 M
/// of copper per litre) and bites no deeper than the tray's oxidant could
/// take off it; the metal balances.
#[test]
fn a_forgotten_plate_bites_on_until_the_bath_is_spent() {
    assert_eq!(steps(15.0).len(), 15);
    assert_eq!(steps(0.5), vec![(0.0, 30.0)]);
    assert_eq!(steps(3.0 * DAY_MIN).len(), 1440);
    let dutch = BathSpec { acid: Acid::Dutch, molar: etch::DUTCH_CHLORATE_M, temp_c: 20.0, face: Face::Up, rock: Some((5.0, 4.0)), feather: None, fresh: true };
    // (a 400 mm plate at 10 mm a cell: a 7.2 L tray)
    let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 400.0, 400.0, 1.3, 45.0, 40, 40);
    p.immerse(dutch, 0.0).unwrap();
    let t0 = std::time::Instant::now();
    let r = p.lift(365.0 * DAY_MIN).unwrap();
    assert!(t0.elapsed().as_secs_f64() < 20.0, "a year in the bath: {:?}", t0.elapsed());
    let t = p.coats().unwrap().tray.unwrap();
    assert!(r.spent, "a year: the bath is spent ({} M left)", t.molar(Metal::Copper));
    let (_, v) = etch::molar(Metal::Copper);
    let most_um = etch::DUTCH_CHLORATE_M * t.litres / Acid::Dutch.stoichiometry(Metal::Copper).0 * v * 1e18 / (400e3 * 400e3);
    let depth = -p.cell(20, 20).level as f64;
    assert!(depth <= most_um * 1.001 && depth > most_um * (1.0 - 2.0 * SPENT) - 0.01, "{depth} µm against the bath's {most_um}");
    assert!(balance(&p).abs() < 1e-3 * p.bitten_um3());
}

/// The varnish lies where the brush laid it: a strip as wide as the size 6
/// round pressed out at its pressure (`etch::brush_width_mm`) and the few tens
/// of µm its edge crept (`etch::stop_out_spread`), its ends the brush's disc;
/// cells it covers over half are stopped, the rest are not.
#[test]
fn stop_out_lies_where_the_brush_went() {
    let mut p = tiny(Metal::Copper);
    p.apply(varnish(5.0));
    let w = etch::brush_width_mm(etch::STOP_BRUSH_MM, 0.5);
    assert!((w - 3.9 * 0.5 / 0.85).abs() < 1e-6);
    let spread = etch::stop_out_spread(w, etch::STOP_UM);
    assert!(spread > 0.0 && spread < 0.05, "the edge creeps {spread} mm: the brush's edge");
    let half = 0.5 * w + spread;
    for y in 0..100 {
        let d = ((y as f64 + 0.5) * 0.1 - 5.0).abs();
        let stopped = p.etch(100, y).stopped;
        if d < half - 0.05 {
            assert!(stopped, "row {y} under the strip");
        }
        if d > half + 0.05 {
            assert!(!stopped, "row {y} beside it");
        }
    }
    // along the stroke: from the brush's landing to its lift, and its tuft's disc past each end
    assert!(p.etch(19, 50).stopped && p.etch(181, 50).stopped);
    assert!(!p.etch(5, 50).stopped && !p.etch(195, 50).stopped);
}

/// A brush runs dry: the varnish its belly held (load of `round_belly_mm3`)
/// is laid as a film `STOP_UM` thick across its width, and no further.
#[test]
fn a_brush_runs_dry() {
    let pts = vec![pt(1.0, 5.0, 0.85), pt(19.0, 5.0, 0.85)];
    let held = 0.1 * etch::round_belly_mm3(1.0);
    let s = Plate::stopping(1.0, 0.1, &pts);
    let want = held / (etch::STOP_UM * 1e-3 * 1.0);
    assert!(s.dry && (s.laid_mm / want - 1.0).abs() < 1e-9, "{} mm against {want}", s.laid_mm);
    assert!(s.left_mm3.abs() < 1e-12 && (s.used_mm3 / held - 1.0).abs() < 1e-9);
    let mut p = tiny(Metal::Copper);
    p.apply(Op::StopOut { brush_mm: 1.0, load: 0.1, pts });
    let end = 1.0 + want;
    let x_in = ((end - 0.8) / 0.1) as usize;
    let x_out = ((end + 0.8) / 0.1) as usize;
    assert!(p.etch(x_in, 50).stopped && !p.etch(x_out, 50).stopped, "stopped up to {end} mm");
    // a full brush lays the whole stroke
    assert!(!Plate::stopping(1.0, 1.0, &[pt(1.0, 5.0, 0.85), pt(19.0, 5.0, 0.85)]).dry);
}

/// The ground lies where the roller went: a cell its line of contact never
/// crossed is bare and bites as an open face, while the grounded cells bite
/// only through their needle lines; a needle on the bare part needs no force
/// to part a ground.
#[test]
fn a_ground_lies_where_the_roller_went() {
    let mut p = tiny(Metal::Copper);
    // rolled over the left half only (x 0 to 10 mm)
    p.apply(Op::Ground { um: 4.0, roller_mm: 150.0, pts: vec![pt(0.0, 5.0, 0.5), pt(10.0, 5.0, 0.5)] });
    assert!(p.grounded(50, 50) && !p.grounded(150, 50));
    assert!((p.ground_share() - 0.5).abs() < 1e-9);
    p.apply(Op::Needle { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(2.0, 5.0, 0.3), pt(18.0, 5.0, 0.3)] });
    p.bite(ferric(10.0, 20.0));
    assert_eq!(p.cell(50, 20).level, 0.0, "the ground keeps the acid off the left half's flat");
    assert!(p.cell(150, 20).level < 0.0, "the bare right half bites open");
    assert!(p.etch(50, 50).lines.iter().any(|l| l.z > 0.0), "the needle line bites through the ground");
    // and it comes back from its section as it was
    let mut v = Vec::new();
    p.write(&mut v).unwrap();
    assert!(Plate::read(&mut v.as_slice()).unwrap() == p);
}
