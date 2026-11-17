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
    let h = card_film_um(eta);
    assert!((h - (6.0 * 0.0267 * eta * 0.02 * 1e-3 / CARD_PA).sqrt() * 1e6).abs() < 1e-9);
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
        p.ink(&black(), None, Inker::Card, 0.0).unwrap();
        let filled = line_ink(&p, 20, 180, 100, 100 + cells);
        assert!((filled - share * PHI_AIR * d).abs() < 1e-3 * filled, "{w} µm: filled {filled}");
        let mut t = Rag::tarlatan(40.0, 7, 0.7);
        p.wipe(&mut t, None, 0.5, 3, Pattern::Circles, 1.0);
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
    p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    let mut t = Rag::tarlatan(40.0, 3, 0.7);
    p.wipe(&mut t, None, 0.5, 3, Pattern::Circles, 1.0);
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
        p.ink(&black(), None, Inker::Card, 0.0).unwrap();
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
    across.wipe(&mut Rag::tarlatan(40.0, 5, 0.7), None, 0.5, 8, Pattern::Along(90.0), 1.0);
    along.wipe(&mut Rag::tarlatan(40.0, 5, 0.7), None, 0.5, 8, Pattern::Along(0.0), 1.0);
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
    let r = p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    let (l, f) = p.ink_um3();
    assert!((r.added_um3 - (l + f)).abs() < 1e-6 * (l + f));
    let mut t = Rag::tarlatan(40.0, 9, 0.7);
    let w = p.wipe(&mut t, None, 0.5, 3, Pattern::Circles, 1.0);
    assert!(w.taken_um3 > 0.0 && w.laid_um3 > 0.0, "{w:?}");
    let rel = |a: f64, b: f64| (a - b).abs() <= 1e-5 * a.abs().max(b.abs()).max(1.0);
    assert!(rel(w.before_um3 - w.after_um3, w.kept_um3), "{w:?}");
    assert!(rel(t.cloth.as_ref().unwrap().held_um3, w.kept_um3));
    assert!(t.load > 0.0);
    let h = p.hand_wipe(None, 0.5, 2, true, Pattern::Circles, 2.0);
    assert!(rel(h.before_um3 - h.after_um3, h.kept_um3), "{h:?}");
    let r = p.retrousse(None, 1, 3.0);
    assert!(rel(r.before_um3 - r.after_um3, r.taken_um3 - r.laid_um3), "{r:?}");
    let left = p.ink_um3();
    assert!(rel(p.clean_ink(), left.0 + left.1));
    assert_eq!(p.ink_um3(), (0.0, 0.0));
}

/// Inks dabbed on stripes side by side and wiped with one tarlatan (à la
/// poupée): the cloth lays back what its face carries where it is. A point of
/// the face slides along its way, keeping (1 − SMEAR)(1 − SOAK) of its pool
/// over each quarter of the pad's width, so over a plate taking up ink alike
/// everywhere, at `d` from the edge of a stripe the ink it carries from beyond
/// it is that share to the power d / (pad/4) of all it carries, at most (the
/// slanting ways cross more of the stripe first). The middle of each stripe
/// keeps its own ink but for that.
#[test]
fn a_tarlatan_lays_back_the_ink_where_it_wipes() {
    let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 150.0, 30.0, 1.3, 45.0, 600, 120);
    cut(&mut p, 0, 600, 0, 120, 30.0, 15.0, 0.4);
    let mk = |n: &str| Ink { name: n.into(), tubes: vec![(n.into(), 1.0)], ..black() };
    for (j, n) in ["vermilion", "Prussian blue", "chrome yellow"].iter().enumerate() {
        let mask: Vec<f32> = (0..600 * 120).map(|i| if (i % 600) / 200 == j { 1.0 } else { 0.0 }).collect();
        p.ink(&mk(n), Some(&mask), Inker::Card, 0.0).unwrap();
    }
    let pad_mm = 40.0;
    let w = p.wipe(&mut Rag::tarlatan(pad_mm, 2, 0.7), None, 0.5, 3, Pattern::Circles, 1.0);
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
    p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    let mut one = p.clone();
    one.wipe(&mut Rag::tarlatan(40.0, 2, 0.7), None, 0.5, 1, Pattern::Circles, 1.0);
    let t = p.wipe(&mut Rag::tarlatan(40.0, 2, 0.7), None, 0.5, 4, Pattern::Circles, 1.0);
    let film1 = one.ink_um3().1;
    assert!(t.film_um > 0.0 && p.ink_um3().1 < film1, "more passes, less tone");
    let lines = line_ink(&p, 20, 180, 100, 101);
    let h = p.hand_wipe(None, 0.5, 2, true, Pattern::Circles, 2.0);
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
    p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    p.wipe(&mut Rag::tarlatan(40.0, 4, 0.7), None, 0.5, 3, Pattern::Circles, 1.0);
    p.hand_wipe(None, 0.5, 2, true, Pattern::Circles, 2.0);
    let beside = |p: &Plate| (20..180).map(|x| p.ink_cell(x, 102).film as f64).sum::<f64>();
    let mut warm = p.clone();
    let (b0, l0) = (beside(&p), line_ink(&p, 20, 180, 100, 101));
    p.retrousse(None, 1, 3.0);
    assert!(beside(&p) > b0 && line_ink(&p, 20, 180, 100, 101) < l0);
    warm.warm(45.0, 3.0);
    warm.retrousse(None, 1, 3.0);
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
    p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    let mut hour = p.clone();
    let mut week = p.clone();
    let h = hour.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), None, 0.5, 1, Pattern::Circles, 60.0);
    assert!(h.taken_um3 > 0.5 * h.before_um3, "{h:?}");
    week.age_ink(7.0 * 24.0 * 60.0);
    assert!(week.ink_set(100, 100), "cure {}", week.ink_cell(100, 100).cure);
    let w = week.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), None, 0.5, 1, Pattern::Circles, 7.0 * 24.0 * 60.0);
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
    p.ink(&black(), None, Inker::Card, 0.5).unwrap();
    p.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), None, 0.5, 2, Pattern::Circles, 1.0);
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
    p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    p.wipe(&mut Rag::tarlatan(40.0, 1, 0.7), None, 0.5, 3, Pattern::Circles, 1.0);
    p.hand_wipe(None, 0.5, 2, true, Pattern::Circles, 2.0);
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
    p.apply(Op::Ground { um: etch::GROUND_UM as f32 });
    let pt = |x: f32| Point { x, y: 10.0, force: 0.3, alt: 80.0, az: 0.0, speed: 20.0 };
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
    p.ink(&black(), None, Inker::Card, 30.5).unwrap();
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

/// A slanting drypoint line with burr (a needle pulled the way it goes at 55°:
/// it ploughs), inked by card, then one palm pass in circles; the plate under
/// the contact laws or not. Returns it and the ink in its lines before the palm.
fn palmed_slant(contact: bool, pattern: Pattern) -> (Plate, f64) {
    let mut p = square(20.0, 100.0);
    p.contact = contact;
    let az = 8.0f32.atan2(14.0).to_degrees();
    let pts = vec![Point { x: 3.0, y: 6.0, force: 3.0, alt: 55.0, az, speed: 25.0 }, Point { x: 17.0, y: 14.0, force: 3.0, alt: 55.0, az, speed: 25.0 }];
    p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
    p.ink(&black(), None, Inker::Card, 0.0).unwrap();
    let lines = p.ink_um3().0;
    p.hand_wipe(None, 0.5, 1, false, pattern, 2.0);
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
/// slanting line's cells don't tile). Under the laws it is held off only by
/// the gap the burr leaves (`palm_gaps`): the tone it leaves falls smoothly
/// away from the line, as even along the line as the cells allow, and is gone
/// within a millimetre; the line keeps its ink; and it is the same whichever way
/// the palm moves.
#[test]
fn the_palm_leaves_a_narrow_smooth_tone_by_a_burr() {
    let (old, _) = palmed_slant(false, Pattern::Circles);
    let (new, lines) = palmed_slant(true, Pattern::Circles);
    for side in [-1.0, 1.0] {
        let far_old = mean_cv(&band_films(&old, side, 5.0, 6.0)).0;
        let far = mean_cv(&band_films(&new, side, 5.0, 6.0)).0;
        // without the laws: full of ink at 2 mm, and a comb at the lee's edge
        assert!(mean_cv(&band_films(&old, side, 1.9, 2.1)).0 > 3.0 * far_old);
        let comb = (0..90).map(|k| mean_cv(&band_films(&old, side, 0.05 * k as f64, 0.05 * (k + 1) as f64)).1).fold(0.0, f64::max);
        assert!(comb > 0.3, "the old lee's edge varies along the line by {comb}");
        // under them: polished as the open plate from 0.8 mm on, with a tone near the line
        for d in [0.8, 1.0, 1.5, 2.0] {
            let m = mean_cv(&band_films(&new, side, d, d + 0.1)).0;
            assert!((m - far).abs() < 0.02 * far, "{d} mm: {m} against {far}");
        }
        let mut last = f64::MAX;
        for k in 3..13 {
            let (d0, d1) = (0.05 * k as f64, 0.05 * (k + 1) as f64);
            let (m, cv) = mean_cv(&band_films(&new, side, d0, d1));
            assert!(m > far && m <= last * 1.001, "{d0} mm: {m} µm, falling away from the line");
            assert!(cv < 0.12, "{d0}–{d1} mm: even along the line ({cv})");
            last = m;
        }
        assert!(last < 0.5 * mean_cv(&band_films(&new, side, 0.15, 0.2)).0);
    }
    assert!(new.ink_um3().0 > 0.98 * lines, "the line keeps its ink: {} of {lines}", new.ink_um3().0);
    let (a, _) = palmed_slant(true, Pattern::Along(0.0));
    let (b, _) = palmed_slant(true, Pattern::Along(90.0));
    assert_eq!(a.ink_um3(), b.ink_um3(), "the skin's gap is the same whichever way the palm moves");
}

/// The cloth under the contact laws (`shields_tiled`): across a slanting
/// burred line its lee is as even along the line as across a straight one,
/// and wiping along the line still takes it (IP §1.5).
#[test]
fn the_cloths_lee_is_even_along_a_slanting_line() {
    let make = |deg: f64| {
        let mut p = square(20.0, 100.0);
        p.contact = true;
        let (c, s) = (deg.to_radians().cos() as f32, deg.to_radians().sin() as f32);
        let az = deg as f32;
        let pts = vec![Point { x: 10.0 - 7.0 * c, y: 10.0 - 7.0 * s, force: 3.0, alt: 55.0, az, speed: 25.0 }, Point { x: 10.0 + 7.0 * c, y: 10.0 + 7.0 * s, force: 3.0, alt: 55.0, az, speed: 25.0 }];
        p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts });
        p.ink(&black(), None, Inker::Card, 0.0).unwrap();
        p
    };
    // the lee's share of each cell near the line, wiping square across it
    for deg in [0.0f64, 30.0, 45.0] {
        let p = make(deg);
        let a = deg.to_radians();
        let v = (-a.sin(), a.cos());
        let b = Tarlatan::new(0.7).bend_nm;
        let (sh, _) = p.shields_tiled(v, &|h| lee_um(h, 0.5 * PAD_PA, b)).unwrap();
        // cells 0.1–0.3 mm from the line, away from its ends: all in the lee
        let mut v = Vec::new();
        for y in 0..200 {
            for x in 0..200 {
                let (dx, dy) = ((x as f64 + 0.5) * 0.1 - 10.0, (y as f64 + 0.5) * 0.1 - 10.0);
                let (along, across) = (dx * a.cos() + dy * a.sin(), (dy * a.cos() - dx * a.sin()).abs());
                if along.abs() < 5.0 && (0.15..0.3).contains(&across) {
                    v.push(sh[y * 200 + x] as f64);
                }
            }
        }
        let min = v.iter().cloned().fold(f64::MAX, f64::min);
        assert!(min > 0.99, "{deg}°: every cell beside the line is in the lee (least {min})");
    }
}
