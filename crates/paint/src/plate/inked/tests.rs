//! Ink and wiping against their formulas (notes/research/intaglio_physics.md
//! §5, §1.5). Every expected number is computed here from the formula and the
//! inputs, not read from the code. Tiny plates only.

use super::*;

/// A bone black and umber ink, as an ink maker's: its optics are a dark
/// brown-black's (K large, S small), its pigment 45 % of its volume, in heavy
/// plate oil.
fn black() -> Ink {
    Ink { name: "black".into(), tubes: vec![("bone black".into(), 3.0)], oil: 0.1, stiff: 1.0, pvc: 0.45, k: [8.0, 8.5, 9.0], s: [0.5, 0.5, 0.5], drying: 0.4 }
}

/// A plate `mm` square at `pitch_um` a cell.
fn square(mm: f32, pitch_um: f64) -> Plate {
    let n = (mm as f64 * 1000.0 / pitch_um).round() as usize;
    Plate::new(Metal::Copper, Temper::HalfHard, mm, mm, 1.3, 45.0, n, n)
}

/// Openings `w` µm wide and `d` µm deep (a rectangular section: mean depth
/// d) over the cells (x0..x1, y0..y1), the share of each cell the opening
/// covers given (an opening wider than the cell covers all of it).
fn cut(p: &mut Plate, x0: usize, x1: usize, y0: usize, y1: usize, w: f64, d: f64, share: f64) {
    for y in y0..y1 {
        for x in x0..x1 {
            p.cell_mut(x, y).unwrap().add_open(w, share, share * d);
        }
    }
}

fn pt(x: f32, y: f32, force: f32) -> Point {
    Point { x, y, force, alt: 90.0, az: 0.0, speed: 20.0 }
}

/// The card (50 mm, 10 N) dragged across a 20 mm plate once: its edge spans it.
fn card() -> Vec<Point> {
    vec![pt(0.0, 10.0, 10.0), pt(20.0, 10.0, 10.0)]
}

/// A 40 mm pad slid across a 20 mm plate at pressure 0.5, along x and along y:
/// every cell lies within its radius of the path, the middle rows under its
/// middle.
fn along_x() -> Vec<Point> {
    vec![pt(-25.0, 10.0, 0.5), pt(45.0, 10.0, 0.5)]
}

fn along_y() -> Vec<Point> {
    vec![pt(10.0, -25.0, 0.5), pt(10.0, 45.0, 0.5)]
}

/// The palm (80 mm) drawn across, at pressure 0.5.
fn palm_x() -> Vec<Point> {
    vec![pt(-45.0, 10.0, 0.5), pt(65.0, 10.0, 0.5)]
}

fn line_ink(p: &Plate, x0: usize, x1: usize, y0: usize, y1: usize) -> f64 {
    let mut t = 0.0;
    for y in y0..y1 {
        for x in x0..x1 {
            t += p.ink_cell(x, y).lines() as f64;
        }
    }
    t / ((x1 - x0) * (y1 - y0)) as f64
}

/// IP §5.3's tables: the cloth's sag p·w⁴/(384B) at 10 kPa (2.6 µm over 1 mm
/// and 1.6 mm over 5 mm at B = 10⁻⁵ N·m), and the yarn's dip r − √(r² − w²/4)
/// for a 75 µm yarn (4 µm in a 50 µm line, 19 µm in a 100 µm one).
#[test]
fn the_cloths_reach_is_the_notes_tables() {
    let mid = |w: f64, b: f64| sag_um(w, 0.5 * w, 1e4, b);
    assert!((mid(1000.0, 1e-5) - 1e4 * 1e-12 / (384.0 * 1e-5) * 1e6).abs() < 1e-9);
    assert!((mid(1000.0, 1e-5) - 2.6).abs() < 0.05, "{}", mid(1000.0, 1e-5));
    assert!((mid(5000.0, 1e-5) - 1628.0).abs() < 5.0, "{}", mid(5000.0, 1e-5));
    assert!((mid(2000.0, 1e-4) - 4.2).abs() < 0.1);
    assert!((yarn_dip_um(75.0, 50.0) - (75.0 - (75.0f64 * 75.0 - 625.0).sqrt())).abs() < 1e-4);
    assert!((yarn_dip_um(75.0, 50.0) - 4.3).abs() < 0.1);
    assert!((yarn_dip_um(75.0, 100.0) - 19.1).abs() < 0.1);
    assert_eq!(yarn_dip_um(75.0, 400.0), 75.0);
    // the burr's lee: (B·h/p)^¼, 0.4–0.7 mm for a 25 µm burr at 10 kPa (IP §1.5)
    assert!((lee_um(25.0, 1e4, 1e-5) - 1e6 * (1e-5 * 25e-6 / 1e4f64).powf(0.25)).abs() < 1e-6);
    assert!((350.0..450.0).contains(&lee_um(25.0, 1e4, 1e-5)) && (650.0..750.0).contains(&lee_um(25.0, 1e4, 1e-4)));
}

/// Viscous filling: L = w·√(pt/(6η)); a yield stress holds a plug at
/// pw/(2τ); the share filled is L/d up to `PHI_AIR`. A fine deep line takes
/// stiff cold ink less fully than warm (IP §5.1: viscosity halves every
/// `HALVING_C`), and the card's film follows Reynolds' slider.
#[test]
fn fill_and_the_cards_film_follow_their_formulas() {
    let (w, d, p, t) = (2.0, 50.0, 1e5, 0.5);
    let eta = black().eta(20.0);
    let want = (w * (p * t / (6.0 * eta)).sqrt()).min(p * w / (2.0 * YIELD_PA)) / d;
    assert!((fill(w, d, p, t, eta, YIELD_PA) - want.min(PHI_AIR)).abs() < 1e-12);
    assert!(want < PHI_AIR, "the line is fine enough to be viscous-limited: {want}");
    let warm = black().eta(50.0);
    assert!((warm / eta - 0.5f64.powf(30.0 / HALVING_C)).abs() < 1e-12);
    assert!(fill(w, d, p, t, warm, YIELD_PA) > fill(w, d, p, t, eta, YIELD_PA));
    // the pigment's share raises the oil's viscosity by (1 − φ/φm)^(−2.5 φm)
    assert!((eta / (4.0 * (1.0 - 0.45f64 / 0.64).powf(-1.6))).abs() - 1.0 < 1e-9);
    let h = card_film_um(eta, CARD_MM_S, CARD_PA);
    assert!((h - (6.0 * 0.0267 * eta * 0.02 * 1e-3 / CARD_PA).sqrt() * 1e6).abs() < 1e-9);
    assert!((CARD_PA - 10.0 / (0.05 * 0.002)).abs() < 1e-6);
    // a wide line fills to the air's limit
    assert_eq!(fill(100.0, 30.0, p, t, eta, YIELD_PA), PHI_AIR);
}

/// A plate warmed to 40 °C cools in air with ρct/h: after one time constant
/// it is 20 + 20/e °C.
#[test]
fn a_warm_plate_cools_by_its_heat_capacity() {
    let mut p = square(20.0, 200.0);
    p.warm(40.0, 10.0);
    let tau = 8960.0 * 385.0 * 1.3e-3 / COOL_W_M2K;
    assert!((p.temp_c(10.0) - 40.0).abs() < 1e-9);
    assert!((p.temp_c(10.0 + tau / 60.0) - (20.0 + 20.0 / std::f64::consts::E)).abs() < 1e-5);
}

/// IP §5.3: a line under 0.5 mm keeps its ink under tarlatan: all that lies
/// below the yarn's dip (and the cloth's sag, negligible at these widths).
#[test]
fn a_fine_line_keeps_its_ink_under_tarlatan() {
    for (w, d) in [(120.0f64, 60.0f64), (400.0, 150.0)] {
        let pitch = 100.0;
        let mut p = square(20.0, pitch);
        let (share, cells) = if w <= pitch { (w / pitch, 1) } else { (1.0, (w / pitch).round() as usize) };
        cut(&mut p, 20, 180, 100, 100 + cells, w, d, share);
        p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        let filled = line_ink(&p, 20, 180, 100, 100 + cells);
        assert!((filled - share * PHI_AIR * d).abs() < 1e-3 * filled, "{w} µm: filled {filled}");
        let mut t = Rag::tarlatan(40.0, 7, 0.7);
        p.wipe(&mut t, &along_x(), 3, 1.0);
        let b = Tarlatan::new(0.7).bend_nm;
        let reach = yarn_dip_um(YARN_UM, w.min(2.0 * YARN_UM)) + if w > 2.0 * YARN_UM { sag_um(w, 0.5 * w, 0.5 * PAD_PA, b) } else { 0.0 };
        let below = share * (d - reach);
        let kept = line_ink(&p, 20, 180, 100, 100 + cells);
        assert!(kept >= below * 0.999 && kept <= filled + 1e-6, "{w} µm × {d} µm: kept {kept} µm, below the cloth's reach {below}");
        assert!(kept > 0.4 * filled);
    }
}

/// IP §5.3: a 5 mm open bite is wiped clean but for its edges. The cloth sags
/// p·x²(w − x)²/(24B) at x from the rim: ink is left only where the yarn's
/// dip and that sag stay above the floor, a band at the rim.
#[test]
fn an_open_bite_is_wiped_clean_but_its_edges() {
    let (pitch, w, d) = (100.0, 5000.0, 120.0);
    let mut p = square(20.0, pitch);
    cut(&mut p, 75, 125, 75, 125, w, d, 1.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let mut t = Rag::tarlatan(40.0, 3, 0.7);
    p.wipe(&mut t, &along_x(), 3, 1.0);
    let (b, pr) = (Tarlatan::new(0.7).bend_nm, 0.5 * PAD_PA);
    let filled = PHI_AIR * d;
    // the edge band: x where the yarn (75 µm) and the sag reach the floor
    let edge = (1..2500).map(|x| x as f64).find(|&x| YARN_UM + sag_um(w, x, pr, b) >= d).unwrap();
    assert!(edge > pitch && edge < 1000.0, "the band is {edge} µm");
    let rim_cell = |c: usize| ((c as f64 - 75.0 + 0.5) * pitch).min((125.0 - c as f64 - 0.5) * pitch);
    let mut interior = (0.0, 0usize);
    for y in 90..110 {
        for x in 75..125 {
            let xr = rim_cell(x).min(rim_cell(y));
            let ink = p.ink_cell(x, y).lines() as f64;
            if xr > edge + pitch {
                interior = (interior.0 + ink, interior.1 + 1);
            }
        }
    }
    // (the middle keeps the plate tone the face smears back, as the flat does)
    let tone = (5..20).map(|y| p.ink_cell(10, y).film as f64).sum::<f64>() / 15.0;
    assert!(interior.0 / interior.1 as f64 <= (0.01 * filled).max(2.0 * tone), "the middle keeps {} µm of {filled}, the flat {tone}", interior.0 / interior.1 as f64);
    // the cells within the band keep ink: at the rim, all below the yarn's reach
    for y in 95..105 {
        let rim = p.ink_cell(75, y).lines() as f64;
        assert!(rim >= (filled - (filled - (d - YARN_UM - sag_um(w, 0.5 * pitch, pr, b)))).min(filled) * 0.99 - 1e-6, "rim keeps {rim}");
        assert!(rim > 0.25 * filled, "rim keeps {rim} of {filled}");
    }
}

/// IP §1.5: wiping across a burr keeps the ink in its lee; wiping along the
/// line takes it.
#[test]
fn wiping_across_a_burr_keeps_its_lee_and_along_it_does_not() {
    let make = || {
        let mut p = square(20.0, 100.0);
        // a needle pulled to the right at a low lean, its top ahead and well to one
        // side (past its cone's half-angle): it ploughs, its burr all on one side
        let pts = vec![Point { x: 2.0, y: 10.0, force: 3.0, alt: 30.0, az: 60.0, speed: 1.0 }, Point { x: 18.0, y: 10.0, force: 3.0, alt: 30.0, az: 60.0, speed: 1.0 }];
        p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
        p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        p
    };
    // the flat within the burr's lee, both sides of the line, away from its
    // ends; and the flat far from it
    let mean = |p: &Plate, rows: &[std::ops::Range<usize>]| {
        let (mut t, mut n) = (0.0, 0usize);
        for r in rows {
            for y in r.clone() {
                for x in 40..160 {
                    t += p.ink_cell(x, y).film as f64;
                    n += 1;
                }
            }
        }
        t / n as f64
    };
    let band = |p: &Plate| mean(p, &[93..98, 101..106]);
    let far = |p: &Plate| mean(p, &[20..30, 170..180]);
    let mut across = make();
    let burr = across.burr_um3().1;
    assert!(burr > 5.0, "the needle threw a burr: {burr} µm");
    let mut along = across.clone();
    // (wiped down to a thin tone: eight times back and forth)
    across.wipe(&mut Rag::tarlatan(40.0, 5, 0.7), &along_y(), 8, 1.0);
    along.wipe(&mut Rag::tarlatan(40.0, 5, 0.7), &along_x(), 8, 1.0);
    let (a, l) = (band(&across), band(&along));
    assert!(a > 3.0 * l, "across keeps {a} µm in the lee, along {l}");
    assert!(a > 3.0 * far(&across), "across keeps {a} µm in the lee and {} far from it", far(&across));
    // along, the lee is wiped as the plate's flat is
    assert!(l < 1.25 * far(&along), "along keeps {l} µm by the line and {} far from it", far(&along));
}

/// Ink is conserved: what leaves the plate is what the cloth or palm keeps.
#[test]
fn every_wipe_conserves_ink() {
    let mut p = square(20.0, 100.0);
    cut(&mut p, 20, 180, 50, 51, 80.0, 40.0, 0.8);
    cut(&mut p, 60, 120, 100, 140, 3000.0, 60.0, 1.0);
    let r = p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let (l, f) = p.ink_um3();
    assert!((r.added_um3 - (l + f)).abs() < 1e-6 * (l + f));
    let mut t = Rag::tarlatan(40.0, 9, 0.7);
    let w = p.wipe(&mut t, &along_x(), 3, 1.0);
    assert!(w.taken_um3 > 0.0 && w.laid_um3 > 0.0, "{w:?}");
    let rel = |a: f64, b: f64| (a - b).abs() <= 1e-5 * a.abs().max(b.abs()).max(1.0);
    assert!(rel(w.before_um3 - w.after_um3, w.kept_um3), "{w:?}");
    assert!(rel(t.cloth.as_ref().unwrap().held_um3, w.kept_um3));
    assert!(t.load > 0.0);
    let h = p.hand_wipe(&palm_x(), 2, true, 2.0);
    assert!(rel(h.before_um3 - h.after_um3, h.kept_um3), "{h:?}");
    let r = p.retrousse(&along_x(), 40.0, 1, 3.0);
    assert!(rel(r.before_um3 - r.after_um3, r.taken_um3 - r.laid_um3), "{r:?}");
    let left = p.ink_um3();
    // the solvent rag across the whole plate three times: all of it, into the rag
    let c = p.clean_ink(&along_x(), 40.0, 3, 4.0);
    assert!(rel(c.taken_um3, left.0 + left.1) && rel(c.kept_um3, c.taken_um3), "{c:?}");
    assert_eq!(p.ink_um3(), (0.0, 0.0));
    assert!(!p.inked());
}

/// Inks laid on stripes side by side and wiped with one tarlatan slid along
/// them and back: the cloth lays back what its face carries where it is. A point of
/// the face slides along its way, keeping (1 − SMEAR)(1 − SOAK) of its pool
/// over each quarter of the pad's width, so over a plate taking up ink alike
/// everywhere, at `d` from the edge of a stripe the ink it carries from beyond
/// it is that share to the power d / (pad/4) of all it carries, at most. The
/// middle of each stripe keeps its own ink but for that.
#[test]
fn a_tarlatan_lays_back_the_ink_where_it_wipes() {
    let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 150.0, 30.0, 1.3, 45.0, 600, 120);
    cut(&mut p, 0, 600, 0, 120, 30.0, 15.0, 0.4);
    let mk = |n: &str| Ink { name: n.into(), tubes: vec![(n.into(), 1.0)], ..black() };
    // each ink by a card 50 mm long dragged down its own stripe
    for (j, n) in ["vermilion", "Prussian blue", "chrome yellow"].iter().enumerate() {
        let x = 25.0 + 50.0 * j as f32;
        p.ink(&mk(n), InkTool::of(Inker::Card), &[pt(x, 0.0, 10.0), pt(x, 30.0, 10.0)], 0.0).unwrap();
    }
    let pad_mm = 40.0;
    let w = p.wipe(&mut Rag::tarlatan(pad_mm, 2, 0.7), &[pt(-25.0, 15.0, 0.5), pt(175.0, 15.0, 0.5), pt(-25.0, 15.0, 0.5)], 2, 1.0);
    assert!(w.laid_um3 > 0.2 * w.taken_um3, "the cloth lays a good share back: {w:?}");
    let d_mm = 25.0;
    let beyond = ((1.0 - crate::rag::SMEAR as f64) * (1.0 - crate::rag::SOAK as f64)).powf(d_mm / (pad_mm as f64 / 4.0));
    for j in 0..3 {
        let c = p.ink_cell(100 + 200 * j, 60);
        let own = c.shares()[j] as f64;
        assert!(c.lines() > 0.0 && 1.0 - own <= beyond, "stripe {j}: its own ink {own}, from beyond at most {beyond}");
    }
}

/// Plate tone (IP §5.4): tarlatan leaves a film the rag's smear lays back,
/// thinner with more passes; the palm with whiting polishes it to a few
/// hundredths of what tarlatan left, and reaches only the top of a fine line:
/// skin bulges p·w(1 − ν²)/E into it.
#[test]
fn the_palm_polishes_the_tone_and_spares_the_lines() {
    let mut p = square(20.0, 100.0);
    let (w, d) = (100.0, 50.0);
    cut(&mut p, 20, 180, 100, 101, w, d, 1.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let mut one = p.clone();
    one.wipe(&mut Rag::tarlatan(40.0, 2, 0.7), &along_x(), 1, 1.0);
    let t = p.wipe(&mut Rag::tarlatan(40.0, 2, 0.7), &along_x(), 4, 1.0);
    let film1 = one.ink_um3().1;
    assert!(t.film_um > 0.0 && p.ink_um3().1 < film1, "more passes, less tone");
    let lines = line_ink(&p, 20, 180, 100, 101);
    let h = p.hand_wipe(&palm_x(), 2, true, 2.0);
    assert!(h.film_um < 0.05 * t.film_um, "palm {} against tarlatan {}", h.film_um, t.film_um);
    let dip = 0.5 * PALM_PA * w * (1.0 - SKIN_NU * SKIN_NU) / SKIN_PA;
    assert!((palm_dip_um(w, 0.5 * w, 0.5 * PALM_PA) - dip).abs() < 1e-9 && dip < 5.0);
    let after = line_ink(&p, 20, 180, 100, 101);
    assert!(after >= lines - dip - 1e-6, "the palm took {} µm from a {w} µm line", lines - after);
}

/// Retroussage draws ink up out of the lines and lays it beside them, more
/// from a warm plate: Lucas–Washburn's √(γrt/(2η)).
#[test]
fn retroussage_draws_ink_beside_the_lines() {
    let mut p = square(20.0, 100.0);
    cut(&mut p, 20, 180, 100, 101, 200.0, 150.0, 1.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    p.wipe(&mut Rag::tarlatan(40.0, 4, 0.7), &along_x(), 3, 1.0);
    p.hand_wipe(&palm_x(), 2, true, 2.0);
    let beside = |p: &Plate| (20..180).map(|x| p.ink_cell(x, 102).film as f64).sum::<f64>();
    let mut warm = p.clone();
    let (b0, l0) = (beside(&p), line_ink(&p, 20, 180, 100, 101));
    p.retrousse(&along_x(), 40.0, 1, 3.0);
    assert!(beside(&p) > b0 && line_ink(&p, 20, 180, 100, 101) < l0);
    warm.warm(45.0, 3.0);
    warm.retrousse(&along_x(), 40.0, 1, 3.0);
    assert!(l0 - line_ink(&warm, 20, 180, 100, 101) > l0 - line_ink(&p, 20, 180, 100, 101));
    let eta = black().eta(20.0);
    assert!((washburn_um(eta, RETRO_TOUCH_S) - (0.033 * 10e-6 * 0.05 / (2.0 * eta)).sqrt() * 1e6).abs() < 1e-9);
}

/// Ink left on a plate dries as an oil film: a day or more later it has set
/// past its gel point and no wipe moves it; an hour later it still wipes.
#[test]
fn ink_left_on_a_plate_dries_in() {
    let mut p = square(20.0, 100.0);
    cut(&mut p, 20, 180, 100, 101, 100.0, 40.0, 1.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let mut hour = p.clone();
    let mut week = p.clone();
    let h = hour.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), &along_x(), 1, 60.0);
    assert!(h.taken_um3 > 0.5 * h.before_um3, "{h:?}");
    week.age_ink(7.0 * 24.0 * 60.0);
    assert!(week.ink_set(100, 100), "cure {}", week.ink_cell(100, 100).cure);
    let w = week.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), &along_x(), 1, 7.0 * 24.0 * 60.0);
    assert_eq!(w.taken_um3, 0.0);
}

/// An inked plate's section is version 3 and reads back whole; a plate
/// never inked is written as before (version 1).
#[test]
fn an_inked_plate_reads_back() {
    let mut p = square(20.0, 100.0);
    let mut b = Vec::new();
    p.write(&mut b).unwrap();
    assert_eq!(u64::from_le_bytes(b[4..12].try_into().unwrap()), 1);
    cut(&mut p, 20, 180, 100, 101, 100.0, 40.0, 1.0);
    p.warm(35.0, 0.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.5).unwrap();
    p.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), &along_x(), 2, 1.0);
    let mut b = Vec::new();
    p.write(&mut b).unwrap();
    assert_eq!(u64::from_le_bytes(b[4..12].try_into().unwrap()), 3);
    let q = Plate::read(&mut &b[..]).unwrap();
    assert_eq!(p, q);
    // damaged: refused
    let mut bad = b.clone();
    let n = bad.len();
    bad[n - 3] = 0xff;
    bad[n - 4] = 0xff;
    bad[n - 5] = 0xff;
    bad[n - 6] = 0x7f;
    assert!(Plate::read(&mut &bad[..]).is_err() || Plate::read(&mut &bad[..]).unwrap() != p);
    assert!(Plate::read(&mut &b[..b.len() - 1]).is_err());
}

/// A rag without a tarlatan's cloth prints as it always did (the state
/// digest of every rag log hashes it).
#[test]
fn a_rag_prints_as_before() {
    let r = Rag::new(40.0, 3);
    let s = format!("{r:?}");
    assert!(s.starts_with("Rag { width: 40.0, load: 0.0, soaked: 0.0, damp: 0.0, wet_at: 0.0, fold: 0, seed: 3, solvent_mm3: 0.0, mobile: Mobile {"), "{s}");
    assert!(!s.contains("cloth"));
    assert!(format!("{:?}", Rag::tarlatan(40.0, 3, 0.5)).contains("cloth"));
}

/// Ink looks dark on the metal: an inked line is darker than the bare metal
/// beside it, and a wiped flat with its thin tone nearly the metal's colour.
#[test]
fn ink_darkens_the_metal_it_lies_on() {
    let mut p = square(20.0, 100.0);
    cut(&mut p, 0, 200, 100, 101, 100.0, 50.0, 1.0);
    let bare = p.seen((100, 100, 101, 101), None)[0];
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    p.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), &along_x(), 3, 1.0);
    p.hand_wipe(&palm_x(), 2, true, 2.0);
    let line = p.seen((100, 100, 101, 101), None)[0];
    let flat = p.seen((100, 50, 101, 51), None)[0];
    let metal = Plate::bare_rgb(Metal::Copper);
    let lum = |c: Rgb| crate::color::luminance(c);
    assert!(lum(line) < 0.2 * lum(bare), "line {line:?} against bare {bare:?}");
    assert!(lum(flat) > 0.7 * lum(metal), "the polished flat {flat:?} against metal {metal:?}");
}

/// An etched plate, bitten on the clock (into its bath, out of it), cleaned
/// and inked, is version 3 with its coats, the bath slot among them, and
/// reads back whole; a plate still in its bath stays version 2 (`--@ etch`'s
/// layout, the bath in its coats).
#[test]
fn an_etched_and_inked_plate_reads_back_with_its_coats() {
    use super::etched::BathSpec;
    use crate::etch::{self, Acid, Face};
    let mut p = square(20.0, 100.0);
    let pt = |x: f32| Point { x, y: 10.0, force: 0.3, alt: 80.0, az: 0.0, speed: 20.0 };
    p.apply(Op::Ground { um: etch::GROUND_UM as f32, roller_mm: etch::ROLLER_MM as f32, pts: vec![pt(0.0), pt(20.0)] });
    p.apply(Op::Needle { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(4.0), pt(16.0)] });
    let bath = BathSpec { acid: Acid::Ferric, molar: 2.2, temp_c: 20.0, face: Face::Down, rock: None, feather: None, fresh: false };
    p.immerse(bath, 10.0).unwrap();
    let mut b = Vec::new();
    p.write(&mut b).unwrap();
    assert_eq!(u64::from_le_bytes(b[4..12].try_into().unwrap()), 2);
    let back = Plate::read(&mut &b[..]).unwrap();
    assert!(back == p && back.in_bath() == p.in_bath());
    p.lift(25.0).unwrap();
    p.apply(Op::Clean);
    assert!(p.coats().is_some() && p.in_bath().is_none());
    p.warm(35.0, 30.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 30.5).unwrap();
    let mut b = Vec::new();
    p.write(&mut b).unwrap();
    assert_eq!(u64::from_le_bytes(b[4..12].try_into().unwrap()), 3);
    let q = Plate::read(&mut &b[..]).unwrap();
    assert!(q == p, "an etched, inked plate comes back as it was");
    let mut again = Vec::new();
    q.write(&mut again).unwrap();
    assert!(again == b);
    assert!(Plate::read(&mut &b[..b.len() - 1]).is_err());
}

/// The palm's gap past a burr (`palm_span_um`, `palm_gap_um`): a 40 µm burr
/// whose top bears over 25 µm either side, under 5 kPa, by hand: a·F(a/c) =
/// h·E*/(2p) = 40 × 0.4 MPa / 10 kPa = 1600 µm with F(r) = acosh r − √(1 −
/// 1/r²), so a ≈ 568 µm (F(22.7) = 2.817). The gap is the burr's height on it
/// and closes smoothly at a, as (a − x)^(3/2). Uniform pressure's span,
/// h·E/(p(1 − ν²)) (`palm_lee_um`), is several times wider.
#[test]
fn the_palms_gap_past_a_burr_is_a_crack_opening() {
    let (h, c, p) = (40.0, 25.0, 5e3);
    let a = palm_span_um(h, c, p);
    assert!((555.0..580.0).contains(&a), "{a}");
    let es = SKIN_PA / (1.0 - SKIN_NU * SKIN_NU);
    assert!((2.0 * p * a / es * ((a / c).acosh() - (1.0 - (c / a).powi(2)).sqrt()) - h).abs() < 1e-6);
    assert_eq!(palm_gap_um(h, c, p, a, 10.0), h);
    assert_eq!(palm_gap_um(h, c, p, a, a + 1.0), 0.0);
    assert!((palm_gap_um(h, c, p, a, c + 1e-6) - h).abs() < 1e-3);
    let near = |e: f64| palm_gap_um(h, c, p, a, a - e);
    assert!((near(4.0) / near(2.0) - 2f64.powf(1.5)).abs() < 0.05, "{} {}", near(4.0), near(2.0));
    let mut last = h;
    for x in (30..560).step_by(10) {
        let g = palm_gap_um(h, c, p, a, x as f64);
        assert!(g <= last, "it falls away from the burr");
        last = g;
    }
    assert!(palm_span_um(h, c, 2.0 * p) < a, "pressed harder it is held off less");
    assert!(palm_lee_um(h, p) > 4.0 * a, "uniform pressure's span {} against {a}", palm_lee_um(h, p));
}

/// The palm worked round in circles 5 mm across the middle of a 20 mm plate,
/// `laps` times: every cell under it is rubbed from every way.
fn circles(laps: usize) -> Vec<Point> {
    let n = 48;
    (0..=laps * n).map(|i| {
        let a = i as f32 / n as f32 * std::f32::consts::TAU;
        pt(10.0 + 5.0 * a.cos(), 10.0 + 5.0 * a.sin(), 0.5)
    }).collect()
}

/// A slanting drypoint line with burr (a needle pulled the way it goes at 55°:
/// it ploughs), inked by card, then the palm along `path`; the plate under
/// the contact laws or not. Returns it and the ink in its lines before the palm.
fn palmed_slant(contact: bool, path: &[Point]) -> (Plate, f64) {
    let mut p = square(20.0, 100.0);
    p.contact = contact;
    let az = 8.0f32.atan2(14.0).to_degrees();
    let pts = vec![Point { x: 3.0, y: 6.0, force: 3.0, alt: 55.0, az, speed: 25.0 }, Point { x: 17.0, y: 14.0, force: 3.0, alt: 55.0, az, speed: 25.0 }];
    p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let lines = p.ink_um3().0;
    p.hand_wipe(path, 1, false, 2.0);
    (p, lines)
}

/// The films (µm) of the cells whose middles lie `d0..d1` mm from the slanting
/// line, on side `side` (±1), away from its ends.
fn band_films(p: &Plate, side: f64, d0: f64, d1: f64) -> Vec<f64> {
    let (ox, oy, tx, ty) = (3.0, 6.0, 14.0 / 260f64.sqrt(), 8.0 / 260f64.sqrt());
    let mut out = Vec::new();
    for y in 0..200 {
        for x in 0..200 {
            let (dx, dy) = ((x as f64 + 0.5) * 0.1 - ox, (y as f64 + 0.5) * 0.1 - oy);
            let (along, across) = (dx * tx + dy * ty, dy * tx - dx * ty);
            if (4.0..12.0).contains(&along) && across * side >= d0 && across * side < d1 {
                out.push(p.ink_cell(x, y).film as f64);
            }
        }
    }
    out
}

/// The mean and the coefficient of variation.
fn mean_cv(v: &[f64]) -> (f64, f64) {
    let m = v.iter().sum::<f64>() / v.len() as f64;
    let var = v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64;
    (m, var.sqrt() / m.max(1e-12))
}

/// The palm's halos (a painter's report): wiping heavy ink by a burred
/// drypoint line, the palm without the contact laws is held off for h·E/(p(1 −
/// ν²)), 3 mm and more past this 39 µm burr, and the lee's edge is a comb on a
/// slanting line (each piece of burr shielded a strip one cell long, which a
/// slanting line's cells don't tile). Under the laws the palm bears on the
/// plate by its ridges (`Ridges`), and a burr lower than their furrows props
/// only a ridge riding it (`palm_gaps`): it is held off a share of its touches
/// within a ridge's contact and reach of the line, at most 2a/λ (a its
/// contact's half-width, λ the ridges' pitch), and nowhere past a + reach(h)
/// from the fin's crest (which stands some 0.08 mm off the line). So beside
/// the line, past the fin (whose faces only the ridge on its crest comes
/// near, `palm_on_fin_um`: from 0.2 mm, at these 0.1 mm cells), it leaves at
/// most (1 − q)^(−n·2a/λ) of the open plate's film after n passes of split
/// q; from 0.3 mm on, the open plate's own (no outline where a cell's points
/// are partly propped: the un-propped points touch whole); even along the
/// line; the line keeps its ink; and it is the same whichever way the palm
/// moves.
#[test]
fn the_palm_wipes_beside_a_burr_but_where_a_ridge_rides_it() {
    // (ten laps round 31 mm under an 80 mm palm: some four passes, every way)
    let (old, _) = palmed_slant(false, &circles(10));
    let (new, lines) = palmed_slant(true, &circles(10));
    let r = Ridges::at(PALM_PA * 0.5);
    let most = (1.0 / (1.0 - PALM_SPLIT)).powf(4.0 * 2.0 * r.a / r.lambda);
    for side in [-1.0, 1.0] {
        let far_old = mean_cv(&band_films(&old, side, 5.0, 6.0)).0;
        let far = mean_cv(&band_films(&new, side, 5.0, 6.0)).0;
        // without the laws: full of ink at 2 mm, and a comb at the lee's edge
        assert!(mean_cv(&band_films(&old, side, 1.9, 2.1)).0 > 3.0 * far_old);
        let comb = (0..90).map(|k| mean_cv(&band_films(&old, side, 0.05 * k as f64, 0.05 * (k + 1) as f64)).1).fold(0.0, f64::max);
        assert!(comb > 0.3, "the old lee's edge varies along the line by {comb}");
        // under them: the open plate's from 0.3 mm on
        for k in 0..8 {
            let d = 0.3 + 0.1 * k as f64;
            let m = mean_cv(&band_films(&new, side, d, d + 0.1)).0;
            assert!((m - far).abs() < 0.01 * far, "{d} mm: {m} against {far}");
        }
        // nearer, past the fin (its toe 0.13 mm out, and half a cell), held off only where a
        // ridge rides the burr, and even along the line
        for k in 2..3 {
            let (d0, d1) = (0.1 * k as f64, 0.1 * (k + 1) as f64);
            let (m, cv) = mean_cv(&band_films(&new, side, d0, d1));
            assert!(m >= far * 0.999 && m <= most * far, "{d0} mm: {m} µm, the open plate {far}, at most {}", most * far);
            assert!(cv < 0.12, "{d0}–{d1} mm: even along the line ({cv})");
        }
    }
    assert!(new.ink_um3().0 > 0.98 * lines, "the line keeps its ink: {} of {lines}", new.ink_um3().0);
    // along x and along y the palm slides over each cell within a few per cent alike (its
    // chord there over its 80 mm): the ridges' shield doesn't care which way it moves
    let (a, _) = palmed_slant(true, &[pt(-45.0, 10.0, 0.5), pt(65.0, 10.0, 0.5)]);
    let (b, _) = palmed_slant(true, &[pt(10.0, -45.0, 0.5), pt(10.0, 65.0, 0.5)]);
    let (ta, tb) = (a.ink_um3(), b.ink_um3());
    assert!(((ta.0 + ta.1) / (tb.0 + tb.1) - 1.0).abs() < 0.03, "{ta:?} {tb:?}");
}

/// The palm's ridges on the plate (`Ridges`, Westergaard's wavy contact): at
/// a hand's 2.5–10 kPa, far below p* = π·E*·Δ/λ, each ridge touches over
/// sin²(πa/λ) = p/p*; the furrow's gap is nothing on the contact, opens as
/// (x − a)^(3/2) past it (a smooth closure: the pressure's square-root edge),
/// rises to the furrow's middle, and stays below the ridges' height, pressed
/// less the harder the hand presses. The pressure over the contact is the
/// hand's over a pitch. A 2 N burr (29 µm) fits under the furrows; the share
/// of touches it holds off is at most 2a/λ and none past a + reach(h).
#[test]
fn the_palms_ridges_are_a_wavy_contact() {
    use std::f64::consts::PI;
    let es = SKIN_PA / (1.0 - SKIN_NU * SKIN_NU);
    let pstar = PI * es * 0.5 * RIDGE_UM / RIDGE_PITCH_UM;
    let mut last = f64::MAX;
    for p in [2.5e3, 5e3, 1e4] {
        let r = Ridges::at(p);
        assert!(((PI * r.a / r.lambda).sin().powi(2) - p / pstar).abs() < 1e-9);
        assert!(r.gap.iter().all(|&g| g >= 0.0));
        assert!(r.gap.iter().enumerate().all(|(i, &g)| i as f64 * r.dx > r.a || g == 0.0));
        assert!(r.gap.windows(2).all(|w| w[1] >= w[0]), "it rises to the furrow's middle");
        let cl = r.clearance();
        assert!(cl > 0.5 * RIDGE_UM && cl < RIDGE_UM && cl < last, "{p} Pa: clearance {cl}");
        last = cl;
        // (x − a)^(3/2) near the contact's edge
        let at = |x: f64| r.gap[(x / r.dx).round() as usize] as f64;
        let (e1, e2) = (r.a + 4.0 * r.dx, r.a + 8.0 * r.dx);
        let k = (at(e2) / at(e1)).ln() / ((e2 - r.a) / (e1 - r.a)).ln();
        assert!((1.2..1.8).contains(&k), "{p} Pa: it opens as (x − a)^{k}");
        // a 29 µm burr fits under; its shield
        assert!(r.reach(29.0) > r.a && r.reach(29.0) < 0.5 * r.lambda);
        assert!((r.shield(29.0, 0.0) - 2.0 * r.a / r.lambda).abs() < 1e-9);
        assert_eq!(r.shield(29.0, r.a + r.reach(29.0) + 1.0), 0.0);
    }
    // the hand's load over a pitch is what the contact carries: (2p/s²)∫cos(πx/λ)√(s² − sin²(πx/λ)) dx = pλ
    let p = 5e3;
    let r = Ridges::at(p);
    let s2 = (PI * r.a / r.lambda).sin().powi(2);
    let n = 20000;
    let w = 2.0 * r.a / n as f64;
    let total: f64 = (0..n).map(|i| {
        let t = -r.a + (i as f64 + 0.5) * w;
        2.0 * p / s2 * (PI * t / r.lambda).cos() * (s2 - (PI * t / r.lambda).sin().powi(2)).max(0.0).sqrt() * w
    }).sum();
    assert!((total / (p * r.lambda) - 1.0).abs() < 1e-3, "{total} against {}", p * r.lambda);
    assert_eq!(Ridges::at(2.0 * pstar).clearance(), 0.0, "pressed flat past p*");
}

/// The cloth's face (`Face`, the contact laws): a fibre mass by van Wyk's
/// law, p·(T − T_c)³ the same at every load, through the KES reference loads
/// with `FACE_EMC` of `FACE_T0_UM` squeezed off between them. At the pad's 10
/// kPa its give left is s = 75/(100^⅓ − 1)·(1/2)^⅓ = 16.3 µm, its stiffness
/// 3p/s = 1.8 × 10⁹ Pa/m and its squeeze from free 79 µm, so it bears
/// (s/(s + u))³ of p lifted u and opens a true gap only past 79 µm. The
/// softened cloth's rigidity is Peirce's W·c³.
#[test]
fn the_cloths_face_is_a_fibre_mass() {
    let (a, b) = (Face::at(KES_PA.0), Face::at(KES_PA.1));
    assert!(a.delta.abs() < 1e-9);
    assert!((b.delta - FACE_EMC * FACE_T0_UM).abs() < 1e-9, "T0 − Tm = EMC·T0");
    for p in [200.0, 1e4, 2e4] {
        let f = Face::at(p);
        assert!((p * f.s.powi(3) / (KES_PA.1 * b.s.powi(3)) - 1.0).abs() < 1e-9);
        assert!((f.k_pa_m() - 3.0 * p / (f.s * 1e-6)).abs() < 1e-6 * f.k_pa_m());
    }
    let f = Face::at(1e4);
    let sm = 75.0 / (100f64.cbrt() - 1.0);
    assert!((f.s - sm * 0.5f64.cbrt()).abs() < 1e-9 && (16.0..16.6).contains(&f.s), "{}", f.s);
    assert!((f.delta - (sm * 100f64.cbrt() - f.s)).abs() < 1e-9 && (78.0..80.0).contains(&f.delta), "{}", f.delta);
    assert!((1.8e9..1.9e9).contains(&f.k_pa_m()));
    assert_eq!(f.state(0.0), (1.0, 0.0));
    assert!((f.state(f.s).0 - 0.125).abs() < 1e-12);
    assert_eq!(f.state(f.delta + 3.0), (0.0, 3.0));
    // the fibres in a film at the plate go as 1/(T − T_c): r^⅓, denser over a burr's top,
    // no denser than fibres pack, none in a true gap
    for u in [0.0, 4.0, 16.0, 40.0] {
        assert!((f.fibres(u) - f.state(u).0.cbrt()).abs() < 1e-12);
    }
    assert!((f.fibres(-8.0) - f.s / (f.s - 8.0)).abs() < 1e-12 && f.fibres(-8.0) > 1.0);
    let packed = FIBRE_PACKING / (TARLATAN_GSM / COTTON_DENSITY / (FACE_T0_UM - sm * 100f64.cbrt() + f.s));
    assert!((f.packed() - packed).abs() < 1e-9 && (3.0..4.0).contains(&packed), "{packed}");
    assert_eq!(f.fibres(-f.s + 0.01), packed);
    assert_eq!(f.fibres(f.delta), 0.0);
    assert!((softened_bend_nm(0.0) - 0.045 * 9.81 * 0.02f64.powi(3)).abs() < 1e-15);
    assert!((softened_bend_nm(1.0) - 0.045 * 9.81 * 0.03f64.powi(3)).abs() < 1e-15);
}

/// The tarlatan over a burr on its face (`cloth_lift`):
/// - a burr far lower than the face's give barely lifts the cloth, which
///   settles as a plate on a Winkler skin under a line load does, u(x) =
///   u₀·e^(−βx)(cos βx + sin βx), β = (k/4B)^¼ (Hetényi);
/// - it is in balance: what the burr's top bears beyond p is what the face
///   lacks beside it;
/// - a drypoint burr (15–48 µm, 0.5–4 N) squeezes the face over its top
///   nearly to its core and lifts the cloth by the rest: h − u₀ < s; the face
///   beside it bears little; no true gap opens at a hand's pressures (2–20
///   kPa), the lift short of the face's squeeze δ; and it settles within a
///   few bending lengths.
#[test]
fn the_cloth_over_a_burr_lies_on_its_face() {
    let b = softened_bend_nm(0.7);
    let p = 1e4;
    let f = Face::at(p);
    let beta = (f.k_pa_m() / (4.0 * b)).powf(0.25) * 1e-6;
    let small = cloth_lift(0.3, 10.0, p, b);
    let u0 = small.at(0.0);
    assert!(u0 > 0.0 && u0 < 0.1, "{u0}");
    for x in [100.0, 200.0, 400.0, 600.0] {
        let bx = beta * x;
        let want = (-bx).exp() * (bx.cos() + bx.sin());
        assert!((small.at(x) / u0 - want).abs() < 0.03, "{x} µm: {} against Hetényi's {want}", small.at(x) / u0);
    }
    for (h, c) in [(15.0, 10.0), (29.0, 15.0), (48.0, 25.0)] {
        let l = cloth_lift(h, c, p, b);
        // balance over the half line (the burr's top at its raised base)
        let mut sum = 0.0;
        let mut top = 0.0;
        let bears = |u: f64, hb: f64| if u - hb >= f.delta { 0.0 } else { p * (f.s / (f.s + u - hb)).powi(3) };
        for i in 0..l.u.len() {
            // each node's span (node 0's half), the burr's top over its share of it
            let (lo, hi) = (if i == 0 { 0.0 } else { (i as f64 - 0.5) * l.dx }, (i as f64 + 0.5) * l.dx);
            let on = ((c - lo) / (hi - lo)).clamp(0.0, 1.0);
            let u = l.u[i] as f64;
            let (qb, qf) = (bears(u, h) - p, bears(u, 0.0) - p);
            sum += (on * qb + (1.0 - on) * qf) * (hi - lo);
            top += on * qb * (hi - lo);
        }
        assert!(sum.abs() < 0.05 * top, "{h} µm: unbalanced by {sum} of {top}");
        let u0 = l.at(0.0);
        assert!(h - u0 < f.s && h - u0 > 0.0, "{h} µm: lifted {u0}, the face's give {}", f.s);
        assert!(f.state(l.at(c + 20.0)).0 < 0.5, "beside it the face bears little");
        let lam = 2f64.sqrt() / beta;
        assert!(l.at(4.0 * lam) < 0.05 * u0, "{h} µm: settled within four bending lengths");
        for q in [2e3, 2e4] {
            let fq = Face::at(q);
            let lq = cloth_lift(h, c, q, b);
            assert!(lq.u.iter().all(|&u| (u as f64) < fq.delta), "{h} µm at {q} Pa: no true gap");
        }
    }
}

/// A burred drypoint line (2 N, a needle ploughing at 55°) across a 20 mm
/// plate at `deg`, its burr kept, inked by `inked`, under the contact laws.
fn burred(deg: f64, inked: &dyn Fn(&mut Plate)) -> Plate {
    let mut p = square(20.0, 100.0);
    p.contact = true;
    let (c, s) = (deg.to_radians().cos() as f32, deg.to_radians().sin() as f32);
    let az = deg as f32;
    let pts = vec![Point { x: 10.0 - 7.0 * c, y: 10.0 - 7.0 * s, force: 2.0, alt: 55.0, az, speed: 25.0 }, Point { x: 10.0 + 7.0 * c, y: 10.0 + 7.0 * s, force: 2.0, alt: 55.0, az, speed: 25.0 }];
    p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
    inked(&mut p);
    p
}

/// The films (µm) of the cells `d0..d1` mm from the line at `deg` through
/// the plate's middle, on side `side` (±1, or 0 for both), away from its ends.
fn beside(p: &Plate, deg: f64, side: f64, d0: f64, d1: f64) -> Vec<f64> {
    let a = deg.to_radians();
    let mut v = Vec::new();
    for y in 0..200 {
        for x in 0..200 {
            let (dx, dy) = ((x as f64 + 0.5) * 0.1 - 10.0, (y as f64 + 0.5) * 0.1 - 10.0);
            let (along, across) = (dx * a.cos() + dy * a.sin(), dy * a.cos() - dx * a.sin());
            let d = if side == 0.0 { across.abs() } else { across * side };
            if along.abs() < 5.0 && (d0..d1).contains(&d) {
                v.push(p.ink_cell(x, y).film as f64);
            }
        }
    }
    v
}

/// The cloth's lee under the contact laws (`Face::fibres`, `yarn_shade_um`).
/// Beside a burr the face meets the film by the fibres that dip into it,
/// whose density goes as r^⅓ (r the share of the pad's pressure it bears),
/// not by r: the film is a liquid about a fibre deep. So the band where the
/// lifted cloth bears little keeps only a little more film than the open
/// plate. Within a yarn's shade of the fin, √(2Rh) ≈ 65 µm past its top, the
/// yarn can't come down: there it keeps the film it can't reach, a velvet.
/// Measured on a 2 N line wiped across twice (0.1 mm cells):
/// - the line under the fin keeps its ink: a yarn resting on the fin stands
///   h over it, more than its chord dips into the line (a burred line's ink
///   holds, as printers say), and the cloth lays none back into it;
/// - the velvet: the cells by the line keep more than twice the open plate's
///   film;
/// - no plateau: from 0.15 mm on, less than twice the open plate's film (by
///   the old law, r, it kept three to four times it out to 0.5 mm);
///   by 1.5–2.5 mm, the open plate's;
/// (Each burr is as tall wherever its line crosses its cells, and its lee is
/// as even along a slanting line as the open plate: the two tests below.)
#[test]
fn the_cloths_lee_is_a_yarns_shade() {
    for deg in [0.0f64, 30.0, 45.0] {
        let mut p = burred(deg, &|p| {
            p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        });
        let (c, s) = (deg.to_radians().cos() as f32, deg.to_radians().sin() as f32);
        let lines = p.ink_um3().0;
        let mut t = Rag::tarlatan(40.0, 3, 0.7);
        p.wipe(&mut t, &[pt(10.0 - 50.0 * c, 10.0 - 50.0 * s, 0.5), pt(10.0 + 50.0 * c, 10.0 + 50.0 * s, 0.5)], 2, 1.0);
        // the yarn is held off the line by the fin, higher than its chord dips: the line keeps
        // its ink, and the cloth lays none into it (at these 0.1 mm cells a groove's cell reaches
        // past the fin's shade on one side, and gives a little; at the easel's 42 µm, below,
        // none)
        let kept = p.ink_um3().0 / lines;
        assert!((kept - 1.0).abs() < 0.1, "{deg}°: the line keeps {kept} of its ink");
        let far = mean_cv(&beside(&p, deg, 0.0, 3.5, 5.0)).0;
        let near = beside(&p, deg, 0.0, 0.0, 0.1).into_iter().fold(0.0, f64::max);
        assert!(near > 2.0 * far, "{deg}°: the velvet {near} µm by the line against {far} on the open plate");
        for k in 0..4 {
            let (d0, d1) = (0.15 + 0.1 * k as f64, 0.25 + 0.1 * k as f64);
            let m = mean_cv(&beside(&p, deg, 0.0, d0, d1)).0;
            assert!(m < 2.0 * far, "{deg}°, {d0}–{d1} mm: {m} µm against {far}");
        }
        let off = mean_cv(&beside(&p, deg, 0.0, 1.5, 2.5)).0;
        assert!((off / far - 1.0).abs() < 0.2, "{deg}°: {off} µm 1.5–2.5 mm off against {far}");
    }
}

/// The burr's top under the cloth (`cloth_gaps`): the face over the fin is
/// squeezed by h − u₀ more than on the flat, so its fibres there are denser
/// than on the open plate (`Face::fibres` above 1), and it wipes the film on
/// the fin's top faster than the open plate's, not at the share it bears
/// beside it (as it did: the top kept 1.1 µm after a first pass against the
/// open plate's 0.3). Every point on a piece of a 2 N line's fin (within its
/// faces' reach from its crest, `Fence::ei` toward its groove and `eo` away)
/// reads so; points beside it read the lifted face's fewer.
#[test]
fn the_cloth_presses_hardest_on_the_burrs_top() {
    let p = burred(0.0, &|_| {});
    let face = Face::at(1e4);
    let g = p.cloth_gaps(1e4, softened_bend_nm(0.7));
    let fences = p.fences();
    let (mut tops, mut beside) = (0, 0);
    for y in 0..200 {
        for x in 0..200 {
            let Some(slot) = g.get(y * 200 + x) else { continue };
            for (j, pr) in slot.iter().enumerate() {
                let (sx, sy) = ((x as f64 + ((j % SUB) as f64 + 0.5) / SUB as f64) * 100.0, (y as f64 + ((j / SUB) as f64 + 0.5) / SUB as f64) * 100.0);
                let on = |f: &Fence, k: f64| -k * f.ei < f.signed(sx, sy) && f.signed(sx, sy) < k * f.eo;
                if fences.iter().any(|f| on(f, 0.9)) {
                    assert!(pr.top.is_finite() && face.fibres(pr.top as f64) > 1.0, "on the top: {pr:?}");
                    tops += 1;
                } else if fences.iter().all(|f| !on(f, 1.1)) && pr.v > 0.0 {
                    assert!(!pr.top.is_finite() && face.fibres(pr.v as f64) < 1.0, "beside it: {pr:?}");
                    beside += 1;
                }
            }
        }
    }
    assert!(tops > 100 && beside > 100, "{tops} {beside}");
}

/// A burr is as tall wherever its line crosses its cells (`Plate::deposit`
/// under the contact laws): a 2 N line level, at 1.4° and at 5.7° across a
/// plate of the easel's 42 µm cells, away from its ends (where the hand eases
/// off), stands at the groove's burr height in every cell it fills. Each
/// sample of the fin's base carries h/2 of its width and is laid over the cells
/// it covers by area, so a cell reads 2·vol/area = h however its edges cut the
/// base. (Weighted by the fin's own height a
/// cell holding its inner part read up to 1.5 h: 44 µm level, 48–55 µm
/// slanting, by where the line lay in its cells.)
#[test]
fn a_burr_is_as_tall_wherever_its_line_lies_in_its_cells() {
    for deg in [0.0f64, 1.4, 5.7] {
        let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 10.0, 10.0, 1.3, 45.0, 240, 240);
        p.contact = true;
        let (c, s) = (deg.to_radians().cos() as f32, deg.to_radians().sin() as f32);
        let pts = vec![Point { x: 5.0 - 4.5 * c, y: 5.0 - 4.5 * s, force: 2.0, alt: 90.0, az: 0.0, speed: 25.0 }, Point { x: 5.0 + 4.5 * c, y: 5.0 + 4.5 * s, force: 2.0, alt: 90.0, az: 0.0, speed: 25.0 }];
        let rep = p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
        let h = rep.burr_um;
        let mut n = 0;
        for y in 0..240 {
            for x in 60..180 {
                let cell = p.cell(x, y);
                // (a cell the fin's base covers a tenth of, or more; one only grazed holds rounding)
                if cell.burr_area > 0.1 {
                    let b = cell.burr_height() as f64;
                    assert!((b / h - 1.0).abs() < 0.02, "{deg}°: cell ({x}, {y}) {b} µm against the fin's {h}");
                    n += 1;
                }
            }
        }
        assert!(n > 100, "{deg}°: {n} cells");
    }
}

/// The lee along a slanting line (`Plate::fences`, `Plate::across`): a piece
/// of burr stands where its fin does, measured from its line's centreline,
/// not from its cell's middle, so a cell's film beside a line at 1.4° or 5.7°
/// depends on its distance from the line, not on where the line crosses its
/// row. Its crest lies w/2 + the reach of its inner face (the groove's wall
/// carried on up) off the centreline, and its base is its side's section over
/// half its height, read from the line's cells to a tenth or so (their
/// lattice) and on average to 5 %. The needle is held as the easel holds it
/// by default, pulled at 55°, which throws burr up to four cells off the
/// groove: those pieces too are placed from the centreline (placed at their
/// cells' middles they printed a dash of velvet by each row step). At the
/// easel's 42 µm cells, wiped twice along the line, the cells in each 20 µm
/// band from past the yarn's shade (0.16 mm, whose steep edge a band 20 µm
/// wide can't hold even) to 0.4 mm off vary along it by no more than the open
/// plate's creases make them vary.
#[test]
fn the_lee_is_even_along_a_slanting_line() {
    for deg in [1.4f64, 5.7] {
        let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 10.0, 10.0, 1.3, 45.0, 240, 240);
        p.contact = true;
        let a = deg.to_radians();
        let (c, s) = (a.cos() as f32, a.sin() as f32);
        let az = deg as f32;
        let pts = vec![Point { x: 5.0 - 4.5 * c, y: 5.0 - 4.5 * s, force: 2.0, alt: 55.0, az, speed: 25.0 }, Point { x: 5.0 + 4.5 * c, y: 5.0 + 4.5 * s, force: 2.0, alt: 55.0, az, speed: 25.0 }];
        let rep = p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
        let pitch = 10.0 / 240.0;
        let g = crate::plate::groove_streams(p.hardness_pa(), 25.0, 30.0, 2.0, 35.0, 0.0);
        assert!(g.chip == 0.0 && (g.w - rep.width_um).abs() < 1e-6 && (g.burr_l - g.burr_r).abs() < 1e-9);
        let half = g.burr_l / g.burr_h;
        let crest = (0.5 * g.w + half) * 1e-3;
        let mut halves = Vec::new();
        for f in p.fences() {
            let (fx, fy) = (f.x * 1e-3 - 5.0, f.y * 1e-3 - 5.0);
            if (fx * a.cos() + fy * a.sin()).abs() > 3.0 {
                continue;
            }
            let across = (fy * a.cos() - fx * a.sin()).abs();
            // (to the quarter cell the groove is laid at, and the fin's base to the share the
            // cells its toe grazes make of it: a tenth of a cell here)
            assert!((across - crest).abs() < 0.25 * pitch + 0.15 * half * 1e-3, "{deg}°: a piece of burr {across} mm off the line at ({}, {}) mm, its fin's crest {crest}", f.x * 1e-3, f.y * 1e-3);
            assert!((f.c / half - 1.0).abs() < 0.15, "{deg}°: the fin's half base {} µm against {half}", f.c);
            halves.push(f.c);
        }
        let mean = halves.iter().sum::<f64>() / halves.len() as f64;
        assert!(halves.len() > 200 && (mean / half - 1.0).abs() < 0.05, "{deg}°: the fins' half base {mean} µm on average against {half}");
        p.ink(&black(), InkTool::of(Inker::Card), &[pt(-25.0, 5.0, 10.0), pt(35.0, 5.0, 10.0)], 0.0).unwrap();
        let lines = p.ink_um3().0;
        let mut t = Rag::tarlatan(40.0, 3, 0.7);
        p.wipe(&mut t, &[pt(5.0 - 50.0 * c, 5.0 - 50.0 * s, 0.5), pt(5.0 + 50.0 * c, 5.0 + 50.0 * s, 0.5)], 2, 1.0);
        // (the burr keeps its line's ink, `the_cloths_lee_is_a_yarns_shade`, but for what the
        // cloth's fibre ends hanging into it draw up: at most their share of it a pass, the pad
        // pressing its middle 3π/8 of a pass, `fibre_draw`)
        let kept = p.ink_um3().0 / lines;
        let most = 2.0 * 3.0 * std::f64::consts::PI / 8.0 * FIBRE_END_SHARE;
        assert!(kept < 1.0 && kept > 1.0 - most, "{deg}°: the line keeps {kept} of its ink, against at least {}", 1.0 - most);
        let band = |d0: f64, d1: f64| {
            let mut v = Vec::new();
            for y in 0..240 {
                for x in 0..240 {
                    let (dx, dy) = ((x as f64 + 0.5) * pitch - 5.0, (y as f64 + 0.5) * pitch - 5.0);
                    let (along, across) = (dx * a.cos() + dy * a.sin(), dy * a.cos() - dx * a.sin());
                    if along.abs() < 3.0 && (d0..d1).contains(&across.abs()) {
                        v.push(p.ink_cell(x, y).film as f64);
                    }
                }
            }
            mean_cv(&v)
        };
        let (_, open) = band(2.0, 3.0);
        // (past the yarn's shade: its crest's place, w/2 + its inner face's reach, the
        // groove's wall carried on up, w²/(4S) of its height from upright, and √(2Rh) beyond,
        // and half a cell)
        let area = 2.0 * g.burr_l;
        let edge = (crest * 1e3 - half + g.burr_h * g.w * g.w / (4.0 * area) + (2.0 * YARN_UM * g.burr_h).sqrt()) * 1e-3 + 0.5 * pitch;
        let first = (edge / 0.02).ceil() as usize;
        assert!((7..=8).contains(&first), "{deg}°: the shade's edge {edge} mm");
        for k in first..20 {
            let d0 = 0.02 * k as f64;
            let (m, cv) = band(d0, d0 + 0.02);
            assert!(cv < open + 0.03, "{deg}°, {d0:.2} mm off: {m} µm varies by {cv} along the line, the open plate by {open}");
        }
    }
}

/// The wipe's way matters (Ross: "if they wipe perpendicularly to the line,
/// they can actually increase the pile of ink on the other side"): each
/// point of the cloth's face carries what it has picked up along its track,
/// and lays it back on past where it took it. A line inked by a narrow card
/// on a clean plate, wiped across once: the plate past the inked band on the
/// side the pad goes to takes much more of the line's ink than the side it
/// comes from, and wiped the other way, the other side does.
#[test]
fn the_cloth_carries_ink_the_way_it_wipes() {
    let inked = |p: &mut Plate| {
        p.ink(&black(), InkTool { width_mm: 2.0, ..InkTool::of(Inker::Card) }, &[pt(-5.0, 10.0, 2.0), pt(25.0, 10.0, 2.0)], 0.0).unwrap();
    };
    for (way, ahead) in [(1.0f32, 1.0), (-1.0, -1.0)] {
        let mut p = burred(0.0, &inked);
        let mut t = Rag::tarlatan(40.0, 3, 0.7);
        p.wipe(&mut t, &[pt(10.0, 10.0 - 50.0 * way, 0.5), pt(10.0, 10.0 + 50.0 * way, 0.5)], 1, 1.0);
        let (to, from) = (mean_cv(&beside(&p, 0.0, ahead, 1.5, 5.0)).0, mean_cv(&beside(&p, 0.0, -ahead, 1.5, 5.0)).0);
        assert!(to > 1.6 * from, "wiped {way}: {to} µm on the side it goes to, {from} on the side it comes from");
    }
}

/// The ink goes only where the card's edge goes: a 10 mm card dragged along
/// the middle inks the band it covers, and nothing beside it.
#[test]
fn the_card_inks_where_it_goes() {
    let mut p = square(20.0, 100.0);
    cut(&mut p, 0, 200, 0, 200, 80.0, 40.0, 0.5);
    let tool = InkTool { width_mm: 10.0, ..InkTool::of(Inker::Card) };
    let r = p.ink(&black(), tool, &card(), 0.0).unwrap();
    for y in 0..200 {
        let d = ((y as f64 + 0.5) * 0.1 - 10.0).abs();
        let ic = p.ink_cell(100, y);
        if d < 4.95 {
            assert!(ic.total() > 0.0, "row {y} under the card");
        }
        if d > 5.05 {
            assert_eq!(ic.total(), 0.0, "row {y} beside it");
        }
    }
    assert!((r.area_mm2 - 20.0 * 10.0).abs() < 0.5, "{}", r.area_mm2);
    // the bead's pressure is the hand's 10 N over the shorter card's bead
    assert!((tool.pressure(10.0) - 10.0 / (0.01 * 0.002)).abs() < 1e-6);
}

/// À la poupée: a poupée dabbed at a point inks the disc of its face there,
/// and dabbed twice it pushes ink in for twice the time (the pushes add: a
/// fine line fills as √(p·2t)).
#[test]
fn a_poupee_inks_the_disc_it_is_dabbed_on() {
    let (w, d) = (2.0, 60.0);
    let mut p = square(20.0, 100.0);
    cut(&mut p, 0, 200, 0, 200, w, d, 0.01);
    let tool = InkTool { width_mm: 6.0, ..InkTool::of(Inker::Poupee) };
    let f = tool.force();
    assert!((tool.pressure(f) - DABBER_PA).abs() < 1e-6 * DABBER_PA, "the dabber's pressure when no force is given");
    let mut twice = p.clone();
    p.ink(&black(), tool, &[pt(10.0, 10.0, f as f32)], 0.0).unwrap();
    twice.ink(&black(), tool, &[pt(10.0, 10.0, f as f32), pt(10.0, 10.0, f as f32)], 0.0).unwrap();
    assert!(p.ink_cell(100, 100).film > 0.0 && p.ink_cell(100, 128).film > 0.0);
    assert_eq!(p.ink_cell(100, 135).total(), 0.0, "beyond the face's 3 mm");
    assert_eq!(p.ink_cell(10, 10).total(), 0.0);
    let eta = black().eta(20.0);
    let line = |q: &Plate| q.ink_cell(100, 100).lines() as f64 / (0.01 * d);
    assert!((line(&p) - fill(w, d, DABBER_PA, DAB_S, eta, black().yield_pa())).abs() < 1e-5, "{}", line(&p));
    assert!((line(&twice) - fill(w, d, DABBER_PA, 2.0 * DAB_S, eta, black().yield_pa())).abs() < 1e-5, "{}", line(&twice));
    assert!(line(&twice) > line(&p));
}

/// A poupée under the contact laws is a rounded compliant wad: pressed with
/// the force that gives its size its working pressure, it touches a disc as
/// wide, at Hertz's pressure (3/2)·p̄·√(1 − ρ²/a²), nothing at the rim. Each
/// cell's line fills by that pressure for the dab's time, and the film it
/// lays is the full one where the pressure passes the dabber's working
/// pressure and falls with it toward the rim: a soft edge, no hard disc.
/// Pressed eight times as hard its contact is twice as wide (a ∝ F^⅓).
#[test]
fn a_poupee_dabs_a_soft_edged_disc() {
    let (w, d) = (2.0, 60.0);
    let mut p = square(20.0, 100.0);
    p.contact = true;
    cut(&mut p, 0, 200, 0, 200, w, d, 0.01);
    let tool = InkTool { width_mm: 6.0, ..InkTool::of(Inker::Poupee) };
    let f = tool.force();
    let mut hard = p.clone();
    p.ink(&black(), tool, &[pt(10.0, 10.0, f as f32)], 0.0).unwrap();
    let eta = black().eta(20.0);
    let flat = 0.99;
    for x in 100..140 {
        let rho = ((x as f64 + 0.5) * 0.1 - 10.0).hypot(0.05);
        let pr = DABBER_PA * Plate::hertz(rho, 3.0);
        let ic = p.ink_cell(x, 100);
        let film = DABBER_FILM_UM * (pr / DABBER_PA).min(1.0) * flat;
        assert!((ic.film as f64 - film).abs() < 1e-4, "{rho} mm: film {} against {film}", ic.film);
        if pr > 0.0 {
            let line = ic.lines() as f64 / (0.01 * d);
            assert!((line - fill(w, d, pr, DAB_S, eta, black().yield_pa())).abs() < 1e-5, "{rho} mm: fill {line}");
        } else {
            assert_eq!(ic.total(), 0.0, "{rho} mm: past the rim");
        }
    }
    assert!(p.ink_cell(100, 100).film as f64 > 0.99 * DABBER_FILM_UM * flat && p.ink_cell(128, 100).film < 0.5 * p.ink_cell(100, 100).film, "full in the middle, thin by the rim");
    hard.ink(&black(), tool, &[pt(10.0, 10.0, 8.0 * f as f32)], 0.0).unwrap();
    assert!((tool.contact_mm(8.0 * f) - 12.0).abs() < 1e-9);
    assert!(hard.ink_cell(155, 100).total() > 0.0 && hard.ink_cell(161, 100).total() == 0.0, "twice as wide pressed eight times as hard");
}

/// A pad wipes only where it slides: a 10 mm pad drawn along y 5 mm leaves the
/// film at y 15 mm as the card laid it.
#[test]
fn the_pad_wipes_where_it_goes() {
    let mut p = square(20.0, 100.0);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let laid = p.ink_cell(100, 150).film;
    let w = p.wipe(&mut Rag::tarlatan(10.0, 3, 0.7), &[pt(-10.0, 5.0, 0.5), pt(30.0, 5.0, 0.5)], 2, 1.0);
    assert!(p.ink_cell(100, 50).film < 0.5 * laid, "wiped under the pad");
    assert_eq!(p.ink_cell(100, 150).film, laid, "untouched beyond it");
    assert!((w.area_mm2 - 20.0 * 10.0).abs() < 1.0, "{}", w.area_mm2);
    let h = p.hand_wipe(&[pt(10.0, -45.0, 0.5), pt(10.0, -41.0, 0.5)], 1, true, 2.0);
    assert_eq!(h.taken_um3, 0.0, "the palm drawn off the plate takes nothing");
}

/// The plate's polish (`POLISH_HOLD_UM`): under the contact laws no wipe
/// takes the film below the void of the plate's roughness, Ra/2 over the
/// flat, however long the tarlatan and the palm work; without them the palm
/// takes it to nothing.
#[test]
fn the_polish_keeps_a_film_no_wipe_reaches() {
    let wiped = |contact: bool| {
        let mut p = square(10.0, 100.0);
        p.contact = contact;
        p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        // the pad round in circles over the 10 mm plate, two laps a time, refolded between
        let ring: Vec<Point> = (0..=96).map(|i| {
            let a = i as f32 / 48.0 * std::f32::consts::TAU;
            pt(5.0 + 3.0 * a.cos(), 5.0 + 3.0 * a.sin(), 0.5)
        }).collect();
        let mut t = Rag::tarlatan(40.0, 3, 0.7);
        for _ in 0..4 {
            p.wipe(&mut t, &ring, 2, 1.0);
            t.refold(&mut crate::tally::Tally::default());
        }
        // the palm drawn hard straight across it, three times
        p.hand_wipe(&[pt(-45.0, 5.0, 1.0), pt(55.0, 5.0, 1.0)], 3, true, 2.0);
        p.ink_um3().1 / (10_000.0 * 10_000.0)
    };
    let (old, new) = (wiped(false), wiped(true));
    assert!((new - POLISH_HOLD_UM).abs() < 1e-3 * POLISH_HOLD_UM, "under the laws the film comes to the polish's hold: {new} µm");
    assert!(old < 1e-3 * POLISH_HOLD_UM, "without them the palm takes it all: {old} µm");
    assert!((POLISH_HOLD_UM - POLISH_RA_UM / 2.0).abs() < 1e-12);
}

/// Where the cloth stands clear of the plate (in a yarn's shade by every fin,
/// `yarn_shade_um`, and past its face's squeeze by a burr taller than that),
/// the face doesn't touch the film below the gap, and the loose fibre ends
/// that hang in touch only `FIBRE_END_SHARE` of it (0.0225), drawing what
/// capillarity lifts. So beside the tallest burr, two 4 N lines crossing at
/// the easel's 42 µm cells, the plate keeps its velvet: no cell within a
/// millimetre of the crossing but the burrs' own tops is wiped bare (to the
/// polish's hold, half the open plate's film here); none is thinner than the
/// open plate by more than the cloth's creases vary it (some 20 %).
#[test]
fn a_gap_under_the_cloth_keeps_its_film() {
    assert!((FIBRE_END_SHARE - 5.0 * 3000.0 * 15e-6 * 0.1).abs() < 1e-12);
    let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 10.0, 10.0, 1.3, 45.0, 240, 240);
    p.contact = true;
    for (a, b) in [((1.0, 5.0), (9.0, 5.2)), ((1.5, 1.5), (8.5, 8.5))] {
        let pts = vec![Point { x: a.0, y: a.1, force: 4.0, alt: 90.0, az: 0.0, speed: 25.0 }, Point { x: b.0, y: b.1, force: 4.0, alt: 90.0, az: 0.0, speed: 25.0 }];
        p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
    }
    // the yarn stands clear of the plate beside the tallest piece of burr
    let tall = p.fences().iter().map(|q| q.h).fold(0.0, f64::max);
    assert!(yarn_shade_um(tall, 60.0) > 0.0, "{tall} µm shades the plate beside it");
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let ring: Vec<Point> = (0..=72).map(|i| {
        let a = i as f32 / 24.0 * std::f32::consts::TAU;
        pt(5.0 + 3.0 * a.cos(), 5.0 + 3.0 * a.sin(), 0.5)
    }).collect();
    let mut t = Rag::tarlatan(40.0, 3, 0.7);
    p.wipe(&mut t, &ring, 2, 1.0);
    let at = |x: f64, y: f64| p.ink_cell((x / 0.04167) as usize, (y / 0.04167) as usize).film as f64;
    let open = (0..10).map(|k| at(0.5 + 0.2 * k as f64, 8.5)).sum::<f64>() / 10.0;
    let mut least = f64::MAX;
    for y in 96..144 {
        for x in 96..144 {
            // (beside the burrs: their own tops, pressed hardest, are wiped hardest)
            if p.cell(x, y).open_share() < 0.5 && p.cell(x, y).burr_area == 0.0 {
                least = least.min(p.ink_cell(x, y).film as f64 / (1.0 - p.cell(x, y).open_share() as f64));
            }
        }
    }
    assert!(least > 0.7 * open && least > 1.4 * POLISH_HOLD_UM, "the thinnest film by the crossing {least} µm, the open plate {open}");
}

/// The solvent rag over ink (EP §8): a 4 mm pad drawn once along y 10 at 20
/// mm/s. On the flat it keeps RESIDUE/(f + SOLVENT) of the film a whole pass
/// (the middle row, share 1 on a straight stroke) and leaves the film 3 mm
/// off untouched; in the lines it draws the ink up by Lucas–Washburn at the
/// ink's viscosity for the 0.2 s the pad lies over the point, so a stiff ink
/// stays deep in an 80 µm line where a loose one comes right out, and a warm
/// plate gives up more of the stiff one.
#[test]
fn a_solvent_rag_takes_ink_where_it_goes_and_a_stiff_ink_holds_in_the_lines() {
    let loose = Ink { name: "loose".into(), stiff: 0.0, oil: 0.3, pvc: 0.45 * 0.7, ..black() };
    let run = |ink: &Ink, warm: Option<f64>| {
        let mut p = square(20.0, 100.0);
        cut(&mut p, 0, 200, 95, 105, 100.0, 80.0, 0.5);
        if let Some(t) = warm {
            p.warm(t, 0.0);
        }
        p.ink(ink, InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        let before = (p.ink_cell(100, 100), p.ink_cell(100, 40), p.temp_c(0.0));
        let r = p.clean_ink(&[pt(0.0, 10.0, 0.5), pt(20.0, 10.0, 0.5)], 4.0, 1, 0.0);
        (p, before, r)
    };
    for (ink, warm) in [(black(), None), (loose.clone(), None), (black(), Some(45.0))] {
        let (p, (mid, far, temp), r) = run(&ink, warm);
        let (hs, hr) = (crate::etch::SOLVENT_UM, crate::etch::RESIDUE_UM);
        let f = mid.film as f64;
        let want = f * hr / (f + hs);
        let got = p.ink_cell(100, 100);
        assert!((got.film as f64 - if want < crate::etch::MONOLAYER_UM { 0.0 } else { want }).abs() < 1e-4 * f.max(1.0), "film {} against {want}", got.film);
        assert_eq!(p.ink_cell(100, 40), far, "3 mm off the path's pad (2 mm radius): untouched");
        let draw = capillary_um(ink.eta(temp), ink.yield_pa(), 4.0 / 20.0);
        for k in 0..BINS {
            let open = p.cell(100, 100).open[k] as f64;
            if open > 0.0 {
                let want = open * (mid.line[k] as f64 / open - draw).max(0.0);
                // (the middle row lies 0.05 mm off the path: its share is √(1 − 0.025²), not 1)
                assert!((got.line[k] as f64 - want).abs() < 0.01, "bin {k}: {} against {want}", got.line[k]);
            }
        }
        assert!((r.taken_um3 - (r.before_um3 - r.after_um3)).abs() < 1e-6 * r.before_um3 && r.laid_um3 == 0.0);
    }
    let left = |ink: &Ink, warm| {
        let (p, (mid, _, _), _) = run(ink, warm);
        p.ink_cell(100, 100).lines() as f64 / mid.lines() as f64
    };
    let (stiff, thin, warm) = (left(&black(), None), left(&loose, None), left(&black(), Some(45.0)));
    assert!(stiff > 0.3 && thin == 0.0 && warm < stiff, "stiff keeps {stiff}, loose {thin}, warm {warm}");
}

/// A copperplate ink (IP §5.8) of the impressionist box's bone black: its
/// pigment at `COPPERPLATE_CPVC` of bone black's critical share 1/(1 +
/// OA·ρ/(100·0.93)), more to the volume than the tube's paste, so its
/// absorption per coat is the tube ink's times the ratio of the shares, its
/// viscosity the paste's Krieger–Dougherty factor over the tube's, and its
/// yield stress (φ/φ_tube)^5.5 times the tube ink's; it prints that much
/// denser paint (a pile with oil drawn out: `Mixture::paint`'s negative
/// medium). An ink of the tube's paste is as before, bit for bit.
#[test]
fn a_copperplate_ink_is_its_pigment_ground_stiff() {
    let pal = crate::palette::Palette::named_box("impressionist").unwrap();
    let parts = vec![("bone black".to_string(), 1.0f32)];
    let tube = Ink::of_tubes(&pal, &parts, 0.0, 1.0, "t".into()).unwrap();
    assert_eq!(Ink::made(&pal, &parts, 0.0, 1.0, Paste::Tube, "t".into()).unwrap(), tube);
    let cp = Ink::made(&pal, &parts, 0.0, 1.0, Paste::Copperplate, "c".into()).unwrap();
    let t = pal.tubes.iter().find(|t| t.name == "bone black").unwrap();
    let (_, _, oil) = crate::palette::tube_packing(t, pal.name);
    let cpvc = 1.0 / (1.0 + t.oa as f64 * t.density as f64 / (100.0 * 0.93));
    assert!((tube.pvc as f64 - (1.0 - oil as f64)).abs() < 1e-6);
    let want = (0.75 * cpvc).min(DENSE_MAX * tube.pvc as f64);
    assert!((cp.pvc as f64 - want).abs() < 1e-5, "{} against {want} (0.75 of {cpvc}, at most 1.5 of the tube's)", cp.pvc);
    assert!(cpvc > 0.4 && cpvc < 0.5 && cp.pvc > tube.pvc, "bone black's critical share {cpvc}; the tube's paste {}", tube.pvc);
    let r = cp.pvc / tube.pvc;
    assert!((cp.k[1] / tube.k[1] - r).abs() < 1e-4 && (cp.s[1] / tube.s[1] - r).abs() < 1e-4);
    assert!((cp.dense(&pal) - r).abs() < 1e-5 && (tube.dense(&pal) - 1.0).abs() < 1e-6);
    assert!((cp.eta(20.0) / tube.eta(20.0) - krieger(cp.pvc as f64) / krieger(tube.pvc as f64)).abs() < 1e-9);
    assert!((cp.yield_pa() / tube.yield_pa() - (r as f64).powf(YIELD_EXP)).abs() < 1e-6);
    assert!(cp.eta(20.0) > tube.eta(20.0) && cp.yield_pa() > 2.0 * tube.yield_pa());
    let (pt, pc) = (tube.paint(&pal).unwrap(), cp.paint(&pal).unwrap());
    assert!((pc.scatter / pt.scatter - r).abs() < 1e-3 * r, "printed {} against the tube's {} times {r}", pc.scatter, pt.scatter);
    assert_eq!(pt.scatter, pal.pile(vec![(pal.tubes.iter().position(|t| t.name == "bone black").unwrap(), 1.0)]).laid(0.0).scatter);
}

/// The same lines and flat inked and wiped alike with a loose ink (light
/// plate oil, oil added) and a stiff copperplate one: the card drags a film
/// as √η (Reynolds' slider), so the stiff ink leaves more plate tone; and the
/// tarlatan's fibre ends hanging into the lines draw ink up by its capillary
/// rise, Lucas–Washburn capped at γ/τ_y (`capillary_um`), several times
/// further for the loose ink, so it wipes out of the lines and the stiff one
/// holds. No region: the pad goes along the points.
#[test]
fn a_loose_ink_wipes_out_of_the_lines_and_a_stiff_one_holds() {
    let stiff = Ink { name: "copperplate".into(), stiff: 1.0, oil: 0.0, pvc: 0.34, ..black() };
    let loose = Ink { name: "loose".into(), stiff: 0.0, oil: 0.3, pvc: 0.26 * 0.7, ..black() };
    let rise = |i: &Ink| capillary_um(i.eta(20.0), i.yield_pa(), RETRO_TOUCH_S);
    assert!(rise(&loose) > 4.0 * rise(&stiff), "{} against {}", rise(&loose), rise(&stiff));
    let run = |ink: &Ink| {
        let mut p = square(20.0, 100.0);
        p.contact = true;
        cut(&mut p, 0, 200, 60, 61, 60.0, 40.0, 0.6);
        cut(&mut p, 0, 200, 140, 141, 60.0, 40.0, 0.6);
        let r = p.ink(ink, InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        let lines = line_ink(&p, 0, 200, 0, 200);
        let mut t = Rag::tarlatan(40.0, 4, 0.7);
        p.wipe(&mut t, &along_x(), 4, 0.5);
        (r.film_um, line_ink(&p, 0, 200, 0, 200) / lines, p.ink_cell(100, 100).film as f64)
    };
    let (sf, sl, st) = run(&stiff);
    let (lf, ll, lt) = run(&loose);
    assert!((sf / lf - (stiff.eta(20.0) / loose.eta(20.0)).sqrt()).abs() < 1e-6 * sf / lf, "the card's films {sf} and {lf}");
    assert!(st > lt, "plate tone: the stiff ink's {st} µm against the loose one's {lt}");
    assert!(1.0 - ll > 1.5 * (1.0 - sl), "the lines keep {sl} of the stiff ink, {ll} of the loose");
}

/// Vine black's masstone (`Palette::vine_black`): with it, a coat of an ink of
/// its tube's paste absorbs its tinting strength's share (0.6) of a coat of
/// bone black's, K = S·(1 − R∞)²/(2R∞) at its hiding; without it (the
/// estimate #323538) under half. Per volume of pigment it absorbs 0.3 of bone
/// black's, the low end of carbon's size argument (0.3–1.0), where it was
/// 0.25. Bone black and the other tubes are untouched, and a palette without
/// the flag is as it was.
#[test]
fn vine_black_prints_as_its_strength_says() {
    let old = crate::palette::Palette::named_box("impressionist").unwrap();
    let new = old.with_vine_black();
    assert!(!old.vine_black && new.vine_black);
    let tube = |p: &crate::palette::Palette, n: &str| p.tubes.iter().find(|t| t.name == n).unwrap().clone();
    assert_eq!(tube(&old, "vine black").color, crate::color::hex("#323538"));
    assert_eq!(tube(&new, "vine black").color, crate::color::hex(crate::palette::VINE_BLACK_OIL));
    for t in &old.tubes {
        if t.name != "vine black" {
            assert_eq!(tube(&new, t.name).color, t.color);
        }
    }
    let ink = |p: &crate::palette::Palette, n: &str| Ink::of_tubes(p, &[(n.to_string(), 1.0)], 0.0, 1.0, n.into()).unwrap();
    assert_eq!(ink(&new, "bone black"), ink(&old, "bone black"));
    let (bone, vine0, vine) = (ink(&old, "bone black"), ink(&old, "vine black"), ink(&new, "vine black"));
    let strength = tube(&old, "vine black").strength as f64;
    let per_coat = |v: &Ink| v.k[1] as f64 / bone.k[1] as f64;
    assert!(per_coat(&vine0) < 0.5, "{}", per_coat(&vine0));
    assert!((per_coat(&vine) - strength).abs() < 0.03, "a coat of vine black's paste: {} of bone black's", per_coat(&vine));
    let per_pigment = per_coat(&vine) * bone.pvc as f64 / vine.pvc as f64;
    assert!((0.29..0.4).contains(&per_pigment), "{per_pigment}");
    // its K from its masstone and hiding, as every tube's
    let t = tube(&new, "vine black");
    let r = crate::color::luminance(t.color);
    let sc = crate::pigment::scatter_for(r, t.hiding);
    assert!((vine.s[1] - sc).abs() < 1e-6 && (crate::color::luminance(t.color) - 0.0158).abs() < 0.001);
}
