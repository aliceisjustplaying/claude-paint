//! The damp sheet's tests. Every expected number is computed here from the
//! research note's formulas and measured inputs (notes/research/
//! damp_paper.md), never read off the code's output.

use super::*;

/// A 150 mm square sheet of drawing paper (160 px), of grammage `g`.
fn sheet(g: f32) -> Canvas {
    let mut c = crate::tests::drawing_paper(160);
    let mut p = *c.paper.as_ref().unwrap();
    p.grammage = g;
    c.paper = Some(p);
    c
}

fn paper(c: &Canvas) -> Paper {
    *c.paper.as_ref().unwrap()
}

/// GAB, written out from Parker et al.'s fit (M₀ 5.04 %, C 13.9, K 0.83).
fn gab(a: f64) -> f64 {
    let x = 0.83 * a;
    0.0504 * 13.9 * x / ((1.0 - x) * (1.0 - x + 13.9 * x))
}

/// The isotherm gives a sized sheet's measured equilibrium moisture at
/// 50 % RH (6–8 %) and, at activity 1, the fiber saturation measured for
/// cellulose fibers (0.25–0.35); its inverse is its inverse.
#[test]
fn the_isotherm_is_paper_s() {
    assert!((isotherm(0.5) - gab(0.5)).abs() < 1e-15);
    assert!((0.06..=0.08).contains(&room_moisture()), "{}", room_moisture());
    assert!((0.25..=0.35).contains(&fiber_saturation()), "{}", fiber_saturation());
    for a in [0.2, 0.5, 0.75, 0.9, 0.99] {
        assert!((activity(gab(a)) - a).abs() < 1e-12, "{a}");
    }
    assert_eq!(activity(0.5), 1.0, "past fiber saturation the water is free");
}

/// Soaking, draining and blotting: the water a sheet takes in through both
/// faces at its Cobb rate, the film Jeffreys's law leaves on each face, then
/// capillary equilibrium with two fresh GB003 blotters (walls first, then the
/// finest pores of the three), all from the formulas. A long soak, drained
/// half a minute and blotted once, leaves a 250 g/m² sheet damp: past fiber
/// saturation, its free water in its finer pores. (The plan's 0.5–0.6 g/g is
/// the damp pack's, `a_damp_pack_equalises_to_its_recipe`.)
#[test]
fn soak_drain_and_blot_reach_printing_dampness() {
    let mut c = sheet(250.0);
    let p = paper(&c);
    let fiber = 250.0 / (1.0 + gab(0.5));
    let base = fiber * gab(0.5);
    // the sheet's pores: 0.8 × ε × caliper, caliper = c̄ t/(1 − ε), c̄ = β ω/δ
    let caliper = (250e-3 * 25e-3 / 0.16e-3) * 6.0 / 0.5;
    let pores = 0.8 * 0.5 * caliper;
    let cap = fiber * gab(1.0) + pores;

    // 5 minutes: Cobb 25 g/m² in 60 s from each face, √t
    c.damp_soak(5.0, 30.0).unwrap();
    let took = 2.0 * 25.0 * (300.0f64 / 60.0).sqrt();
    let film = (2.0f64 / 3.0) * (1.002e-3f64 * 0.150 / (998.2 * 9.81 * 30.0)).sqrt() * 1e6;
    let want = base + took.min(cap - base) + 2.0 * film;
    let got = c.damp().unwrap().water;
    // (the sheet's height is the canvas's, in f32)
    assert!((got - want).abs() < 1e-6 * want, "{got} vs {want}");
    assert!((c.clock() - (5.0 + 0.5)).abs() < 1e-4, "the soak and the drain pass on the clock");

    // a soak long enough to fill it: its capacity, and the drained film over it
    let mut c = sheet(250.0);
    c.damp_soak(60.0, 30.0).unwrap();
    let full = cap + 2.0 * film;
    assert!((c.damp().unwrap().water - full).abs() < 1e-6 * full);
    let m = c.moisture().unwrap();
    assert_eq!(m.word(), "shiny", "the film stands on it");

    // blotted once between two GB003 blotters (0.8 mm, 330 g/m²)
    c.damp_blot(1, GB003).unwrap();
    let rho_f = 0.16 / (25.0 * 6.0) * 1e3;
    let eps_b = 1.0 - (330.0 / 800.0) / rho_f;
    let (fiber_b, pores_b, h_b) = (330.0 / (1.0 + gab(0.5)), eps_b * 800.0, 6.0 * eps_b / (1.0 - eps_b));
    let h_s = 6.0 * 0.5 / 0.5;
    let total = full + 2.0 * fiber_b * gab(0.5);
    let free = total - (fiber + 2.0 * fiber_b) * gab(1.0);
    let share = |x: f64| 1.0 - (-x).exp() * (1.0 + x);
    let held = |h: f64| pores * share(h / h_s) + 2.0 * pores_b * share(h / h_b);
    let (mut lo, mut hi) = (0.0, 1000.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if held(mid) < free { lo = mid } else { hi = mid }
    }
    let want = fiber * gab(1.0) + pores * share(lo / h_s);
    let got = c.damp().unwrap().water;
    assert!((got - want).abs() < 1e-6 * want, "{got} vs {want}");
    let mc = c.moisture().unwrap().mc;
    // damp through (its walls saturated, free water in its finer pores) and
    // no longer shiny; the heavy blotter takes it below the damp pack's 0.56
    assert!(mc > fiber_saturation() && mc < full / fiber, "blotted: {mc} g/g");
    eprintln!("blotted once with GB003: {mc:.3} g/g");
    assert_eq!(c.moisture().unwrap().word(), "damp");
    let _ = p;
}

/// The printer's damp pack (intaglio_physics.md §6.2): 35 sheets of 250 g/m²
/// BFK, 10 × 15 in, damped with a pint (0.47 L), interleaved overnight.
/// Water is conserved and the sheets come to one moisture: the water added
/// over the conditioned sheet's mass is the plan's 0.56 g/g, and on a dry
/// basis the sheet holds that and its own conditioned water.
#[test]
fn a_damp_pack_equalises_to_its_recipe() {
    let c = sheet(250.0);
    let p = paper(&c);
    let area = (10.0 * 0.0254) * (15.0 * 0.0254);
    let per_sheet = 470.0 / 35.0 / area;
    // one sheet in ten wetted with all the water, the rest dry, as a printer sponges every tenth
    let mut pack: Vec<Medium> = (0..35).map(|i| Medium::sheet(&p, dry_fiber(&p) * room_moisture() + if i % 10 == 0 { per_sheet * 35.0 / 4.0 } else { 0.0 })).collect();
    let before: f64 = pack.iter().map(|m| m.water).sum();
    equalise(&mut pack);
    let after: f64 = pack.iter().map(|m| m.water).sum();
    assert!((after - before).abs() < 1e-9 * before, "water is conserved");
    let fiber = 250.0 / (1.0 + gab(0.5));
    let want = (fiber * gab(0.5) + per_sheet) / fiber;
    for m in &pack {
        assert!((m.water / fiber - want).abs() < 1e-9, "{} vs {want}", m.water / fiber);
    }
    let added = per_sheet / 250.0;
    assert!((added - 0.556).abs() < 0.001, "the plan's 0.56 g/g: {added}");
}

/// Drying on the clock (§4): free water goes from each open face at the wet
/// surface's psychrometric rate (0.71 µm/min in the studio, the research
/// note's derivation), then the walls' water by dM/dt = −faces e(a)/fiber,
/// integrated here by small steps; a wait split in two dries as one; a
/// covered sheet keeps its water.
#[test]
fn drying_follows_the_evaporation_law() {
    let e = evaporation_at(1.0);
    assert!((e - 0.71).abs() < 0.01, "{e}");
    let mut c = sheet(250.0);
    c.damp_soak(60.0, 30.0).unwrap();
    let w0 = c.damp().unwrap().water;
    // lying on the bench, one face: 30 minutes of free water
    c.wait(30.0);
    let w1 = c.damp().unwrap().water;
    assert!((w0 - w1 - 30.0 * e).abs() < 1e-3, "{} vs {}", w0 - w1, 30.0 * e);
    // hung up, two faces
    c.damp_cover(2).unwrap();
    c.wait(30.0);
    let w2 = c.damp().unwrap().water;
    assert!((w1 - w2 - 60.0 * e).abs() < 1e-3);
    // covered: nothing over half an hour (the plan's P3a gate)
    c.damp_cover(0).unwrap();
    c.wait(30.0);
    assert_eq!(c.damp().unwrap().water, w2);

    // the walls' water: from fiber saturation, 2 faces, 4 hours, by RK4 in M
    let p = paper(&c);
    let fiber = dry_fiber(&p);
    let mut d = Damp { water: fiber * gab(1.0), faces: 2 };
    let mut split = d;
    let rate = |m: f64| -2.0 * evaporation_at(activity(m)) / fiber;
    let (mut m, dt) = (gab(1.0) - 1e-9, 0.01);
    for _ in 0..(240.0 / dt) as usize {
        let k1 = rate(m);
        let k2 = rate(m + 0.5 * dt * k1);
        let k3 = rate(m + 0.5 * dt * k2);
        let k4 = rate(m + dt * k3);
        m += dt / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
    }
    d.dry(&p, 240.0);
    assert!((d.water / fiber - m).abs() < 2e-4 * m, "{} vs {m}", d.water / fiber);
    assert!(m > room_moisture() && m < gab(1.0));
    split.dry(&p, 70.0);
    split.dry(&p, 170.0);
    assert!((split.water - d.water).abs() < 1e-6 * d.water, "split {} vs one {}", split.water, d.water);
    // and in the end the room's moisture, never below it
    d.dry(&p, 60.0 * 24.0 * 30.0);
    assert!((d.water / fiber - room_moisture()).abs() < 1e-4);
}

/// Size against moisture (§6): β (min(M, M_fsp) − M_iso), with the measured
/// coefficients: a machine-made wove sheet expands across its machine
/// direction 4 times what it does along it (3–5 measured), about the
/// isotropic value for its fiber length; across, the measured mill range
/// 0.1–0.15 %/%; soaked through, the measured cross-direction wet expansion
/// of machine papers, 2.4–2.75 % (Lindner, after Figueiredo). A
/// laid sheet (a mold's) expands the same both ways. Past fiber saturation
/// it grows no more.
#[test]
fn hygroexpansion_matches_the_measured_coefficients() {
    let mut c = sheet(250.0);
    let p = paper(&c);
    // 2 mm fibers: log-linear between 0.085 at 0.8 mm and 0.05 at 3 mm
    let t = (2.0f64 / 0.8).ln() / (3.0f64 / 0.8).ln();
    let iso = 0.085 * (0.05f64 / 0.085).powf(t);
    let (md, cd) = hygroexpansion(&p);
    assert!((md - iso / 2.0).abs() < 1e-12 && (cd - iso * 2.0).abs() < 1e-12, "{md} {cd}");
    assert!((0.10..=0.15).contains(&cd), "CD {cd} %/%");
    let m0 = c.moisture().unwrap();
    assert!((m0.size_mm.0 - m0.made_mm.0).abs() < 1e-9, "a dry sheet is its size as made");
    c.damp_soak(60.0, 30.0).unwrap();
    let m = c.moisture().unwrap();
    let dm = gab(1.0) - gab(0.5);
    let (sx, sy) = (m.size_mm.0 / m.made_mm.0 - 1.0, m.size_mm.1 / m.made_mm.1 - 1.0);
    assert!((sx - cd * dm).abs() < 1e-12 && (sy - md * dm).abs() < 1e-12, "{sx} {sy}");
    assert!((0.024..=0.0275).contains(&sx), "wet expansion across {sx}");
    // drier but still past fiber saturation: the same size
    c.damp_blot(1, GB003).unwrap();
    assert_eq!(c.moisture().unwrap().size_mm, m.size_mm);
    // a sheet below saturation is between
    let fiber = dry_fiber(&p);
    c.damp_set(fiber * 0.15, 1).unwrap();
    let m = c.moisture().unwrap();
    assert!((m.size_mm.0 / m.made_mm.0 - 1.0 - cd * (0.15 - gab(0.5))).abs() < 1e-12);
    // a laid sheet: isotropic
    let mut laid = p;
    laid.laid = Some(crate::paper::Laid { per_cm: 9.0, chain_mm: 25.0, deficit: 0.15 });
    let (a, b) = hygroexpansion(&laid);
    assert_eq!(a, b);
}

/// Stiffness against moisture (§5): Nissan's ln E falls 0.07 for each 1 %
/// of moisture, held at fiber saturation above it; a damp sheet's
/// z-compression modulus (Kananen et al.'s E* 4–5 MPa when dry) comes to
/// about 1 MPa, the order the research note estimated.
#[test]
fn stiffness_falls_with_moisture_as_measured() {
    let slope = (stiffness(0.20).ln() - stiffness(0.10).ln()) / 10.0;
    assert!((slope + 0.07).abs() < 1e-12, "{slope} per %");
    assert_eq!(stiffness(gab(0.5)), 1.0);
    assert_eq!(stiffness(0.6), stiffness(gab(1.0)), "free water breaks no bonds");
    // Kananen's measured E* of 4–5 MPa, damp: 0.9–1.1 MPa, the order of the
    // research note's estimate (0.5–1 MPa), which this replaces
    let (lo, hi) = (4.0 * (-7.0 * (gab(1.0) - gab(0.5))).exp(), 5.0 * (-7.0 * (gab(1.0) - gab(0.5))).exp());
    assert!(lo > 0.5 && hi < 1.2, "{lo}–{hi} MPa");
    let mut c = sheet(250.0);
    c.damp_soak(60.0, 30.0).unwrap();
    let m = c.moisture().unwrap();
    assert!((m.stiffness - stiffness(gab(1.0))).abs() < 1e-15);
    assert!((m.z_mpa - paper(&c).z_mpa as f64 * m.stiffness).abs() < 1e-9);
}

/// The damp sheet goes in its checkpoint and comes back; a sheet never
/// wetted writes no damp section; another version of the section, or a
/// damp section on a canvas that isn't paper, is refused.
#[test]
fn checkpoint_round_trip() {
    let dry = sheet(250.0);
    let mut a = Vec::new();
    dry.write_state(&mut a, "").unwrap();
    let mut c = dry.clone();
    c.damp_soak(10.0, 30.0).unwrap();
    c.damp_cover(2).unwrap();
    let mut b = Vec::new();
    c.write_state(&mut b, "").unwrap();
    let (r, _) = Canvas::read_state(&mut std::io::Cursor::new(&b)).unwrap();
    assert_eq!(r.damp(), c.damp());
    // and it dries the same after
    let (mut x, mut y) = (c.clone(), r);
    x.wait(45.0);
    y.wait(45.0);
    assert_eq!(x.damp(), y.damp());
    // never wetted: read back without damp state, and written the same
    let (r, _) = Canvas::read_state(&mut std::io::Cursor::new(&a)).unwrap();
    assert!(r.damp().is_none());
    let mut a2 = Vec::new();
    r.write_state(&mut a2, "").unwrap();
    assert_eq!(a, a2);
    // the section's version, changed: refused
    let at = b.windows(8).position(|w| w == DAMP_MARK_BYTES).expect("the damp section");
    let mut v2 = b.clone();
    v2[at + 8..at + 16].copy_from_slice(&2u64.to_le_bytes());
    let e = Canvas::read_state(&mut std::io::Cursor::new(&v2)).err().expect("refused").to_string();
    assert!(e.contains("version 2"), "{e}");
    // its water made invalid: refused
    let mut bad = b.clone();
    bad[at + 16..at + 24].copy_from_slice(&f64::NAN.to_bits().to_le_bytes());
    assert!(Canvas::read_state(&mut std::io::Cursor::new(&bad)).is_err());
}

const DAMP_MARK_BYTES: &[u8] = &0x504d4144u64.to_le_bytes();

/// Only a blank sheet of paper is wetted; a canvas without the switch's
/// verbs never has damp state, and waits as it did.
#[test]
fn only_a_blank_sheet_is_wetted() {
    let mut c = sheet(160.0);
    let mut st = crate::pastel::Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
    let pose = crate::pastel::Pose { force: 2.0, alt: 60f32.to_radians(), az: 0.0, roll: 0.0 };
    c.stick_stroke(&mut st, &[crate::pastel::StrokePoint { x: 300.0, y: 500.0, pose, speed: 50.0 }, crate::pastel::StrokePoint { x: 700.0, y: 500.0, pose, speed: 50.0 }]);
    assert!(c.damp_soak(5.0, 30.0).unwrap_err().contains("drawn on"));
    let mut linen = Canvas::new_window(160, 1.0, [0.55; 3], None);
    assert!(linen.damp_soak(5.0, 30.0).is_err());
    assert!(linen.moisture().is_none());
}

/// #322's sinking finds a damp sheet's pores already holding its water: a
/// full sheet takes none of a wash's.
#[test]
fn a_damp_sheet_takes_less_of_a_wash() {
    let c = sheet(160.0);
    let p = paper(&c);
    let mut c = c;
    c.damp_soak(60.0, 30.0).unwrap();
    let free = c.damp_free() as f64;
    assert!(free >= pore_capacity(&p) - 1e-6, "{free} of {}", pore_capacity(&p));
}
