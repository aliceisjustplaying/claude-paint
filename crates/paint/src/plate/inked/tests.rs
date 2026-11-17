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
