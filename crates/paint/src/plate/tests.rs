//! The plate's physics against its formulas (notes/research/intaglio_physics.md,
//! plate_physics.md). Every expected number is computed here from the formula
//! and the inputs, not read from the code. Tiny plates only.

use super::*;

const PI: f64 = std::f64::consts::PI;

/// Meyer's hardness of `hv`, as PP §2 converts it: HV × g × 2/1.8544.
fn meyer(hv: f64) -> f64 {
    hv * 9.80665e6 * 2.0 / 1.8544
}

fn pt(x: f32, y: f32, force: f32, alt: f32, az: f32, speed: f32) -> Point {
    Point { x, y, force, alt, az, speed }
}

/// A 20 × 10 mm plate at 0.1 mm a cell.
fn tiny(metal: Metal, temper: Temper) -> Plate {
    Plate::new(metal, temper, 20.0, 10.0, 1.3, 45.0, 200, 100)
}

/// A straight line from x 2 to 18 mm at y 5, pulled to the right at a lean
/// (its top ahead: azimuth 0), slowly, so the hand's landing and lift are a
/// few µm of it.
fn pulled(force: f32) -> Vec<Point> {
    vec![pt(2.0, 5.0, force, 55.0, 0.0, 1.0), pt(18.0, 5.0, force, 55.0, 0.0, 1.0)]
}

/// The width follows scratch hardness, w = √(8F/(πH)): four times the force
/// gives twice the width, zinc (45 HV) against half-hard copper (90 HV) √2
/// the width, and a sharp cone's depth is w/(2 tan θ) (IP §1.3).
#[test]
fn groove_size_follows_the_scratch_hardness_law() {
    let h = meyer(90.0);
    assert!((hardness_pa(90.0) / h - 1.0).abs() < 1e-12);
    for f in [0.5, 1.0, 2.0, 5.0] {
        let g = groove(h, 0.0, 45.0, f, 0.0, 0.0);
        let w = (8.0 * f / (PI * h)).sqrt() * 1e6;
        assert!((g.w / w - 1.0).abs() < 1e-9, "{f} N: {} against {w}", g.w);
        assert!((g.d / (w / 2.0) - 1.0).abs() < 1e-9, "a 90° cone: depth half the width");
        // its section: the triangle's, w·d/2
        assert!((g.area / (0.5 * w * g.d) - 1.0).abs() < 1e-9);
        let g4 = groove(h, 0.0, 45.0, 4.0 * f, 0.0, 0.0);
        assert!((g4.w / g.w - 2.0).abs() < 1e-9);
    }
    let cu = groove(meyer(90.0), 25.0, 30.0, 2.0, 30.0, 0.0);
    let zn = groove(meyer(45.0), 25.0, 30.0, 2.0, 30.0, 0.0);
    assert!((zn.w / cu.w - 2f64.sqrt()).abs() < 1e-9);
    // a rounded point: below its tangent the groove is the sphere's chord
    let r = 25.0;
    let g = groove(h, r, 30.0, 0.1, 0.0, 0.0);
    let a = g.w / 2.0;
    assert!(a < r * 30f64.to_radians().cos());
    assert!((g.d - (r - (r * r - a * a).sqrt())).abs() < 1e-9);
    // past it the cone's: depth R(1 − sin θ) + (a − R cos θ)/tan θ
    let g = groove(h, r, 30.0, 2.0, 0.0, 0.0);
    let (a, t) = (g.w / 2.0, 30f64.to_radians());
    assert!((g.d - (r * (1.0 - t.sin()) + (a - r * t.cos()) / t.tan())).abs() < 1e-9);
}

/// The attack angle is 90° − θ − τ: pulled at a lean the needle ploughs (all
/// its groove into burr), pushed it cuts (a chip, no burr); a needle leaning
/// to one side throws its burr to the other (IP §1.2).
#[test]
fn plough_or_cut_by_the_attack_angle() {
    let h = meyer(90.0);
    let pull = groove(h, 0.0, 30.0, 2.0, 35.0, 0.0);
    assert!((pull.beta - 25.0).abs() < 1e-9 && !pull.cut);
    assert!(((pull.burr_l + pull.burr_r) / pull.area - 1.0).abs() < 1e-12);
    assert!((pull.burr_l - pull.burr_r).abs() < 1e-9, "upright sideways: both sides alike");
    let push = groove(h, 0.0, 30.0, 2.0, -35.0, 0.0);
    assert!(push.beta >= BETA_C_DEG && push.cut && push.burr_l + push.burr_r == 0.0);
    // leaning right by its half-angle: the right flank is vertical, all the burr goes left
    let lean = groove(h, 0.0, 30.0, 2.0, 35.0, 30.0);
    assert!(lean.burr_r < 1e-9 * lean.area && (lean.burr_l / lean.area - 1.0).abs() < 1e-9);
    let some = groove(h, 0.0, 30.0, 2.0, 35.0, 10.0);
    let (l, r) = (40f64.to_radians().tan(), 20f64.to_radians().tan());
    assert!((some.burr_l / some.area - l / (l + r)).abs() < 1e-9);
}

/// A pulled line on the plate: its groove is the scratch law's section along
/// its length, its burr has the groove's volume (nothing removed), and the
/// plate's metal is what it was. Pushed, the groove's metal leaves as a chip.
#[test]
fn a_ploughed_burr_holds_the_grooves_metal() {
    let mut p = tiny(Metal::Copper, Temper::HalfHard);
    let rep = p.apply(Op::Drypoint { tip_um: 0.0, cone_deg: 30.0, pts: pulled(2.0) });
    assert!(rep.ploughed && !rep.cut);
    // the lean along the line: alt 55 leans 35° toward the right, the way it goes
    let g = groove(meyer(90.0), 0.0, 30.0, 2.0, 35.0, 0.0);
    let expected = g.area * 16_000.0;
    let made = p.groove_um3();
    assert!((made / expected - 1.0).abs() < 0.01, "groove {made} µm³ against {expected}");
    let (burr, tallest) = p.burr_um3();
    assert!((burr / made - 1.0).abs() < 1e-3, "burr {burr} against groove {made}");
    assert!(p.metal_um3().abs() < 1e-3 * made, "metal conserved: {}", p.metal_um3());
    assert!(tallest > 0.0);
    // the cells keep the opening's width and depth
    let c = p.cell_at_mm(10.0, 5.0).unwrap();
    let (s, w, _) = c.openings().next().unwrap();
    assert!(s > 0.0 && (w as f64 / g.w - 1.0).abs() < 1e-3, "width {w} against {}", g.w);

    let mut q = tiny(Metal::Copper, Temper::HalfHard);
    let pushed: Vec<Point> = pulled(2.0).into_iter().map(|mut x| { x.az = 180.0; x }).collect();
    let rep = q.apply(Op::Drypoint { tip_um: 0.0, cone_deg: 30.0, pts: pushed });
    assert!(rep.cut && !rep.ploughed);
    assert!(q.burr_um3().0 < 0.2 * q.groove_um3(), "under a fifth of the burr");
    assert!((q.chip_um3 / q.groove_um3() - 1.0).abs() < 0.01);
    assert!((q.metal_um3() + q.chip_um3).abs() < 1e-3 * q.chip_um3, "what is left and the chip");
}

/// A needle leaning sideways (its top to the right of the way it goes) puts
/// its burr on the left side of the line only.
#[test]
fn a_tilted_needles_burr_is_on_one_side() {
    let mut p = tiny(Metal::Copper, Temper::HalfHard);
    // going right (+x), the top leaning ahead and to the right (+y, down the plate): az 60, alt 30
    p.apply(Op::Drypoint { tip_um: 0.0, cone_deg: 30.0, pts: vec![pt(2.0, 5.0, 2.0, 30.0, 60.0, 1.0), pt(18.0, 5.0, 2.0, 30.0, 60.0, 1.0)] });
    let (mut above, mut below) = (0.0, 0.0);
    for y in 0..100 {
        for x in 0..200 {
            let c = p.cell(x, y);
            if y < 50 { above += c.burr_vol } else { below += c.burr_vol }
        }
    }
    // the left of a line going right is up the plate (−y)
    assert!(above > 0.0 && below < 1e-6 * above, "{above} {below}");
}

/// Burnishing lowers the burr and keeps the metal; the burr goes before the
/// line: the rims flow into the groove only where no burr holds the
/// burnisher off, and only where its Hertz pressure exceeds what a rim bears.
#[test]
fn burnishing_lowers_the_burr_and_conserves_metal() {
    let mut p = tiny(Metal::Copper, Temper::HalfHard);
    p.apply(Op::Drypoint { tip_um: 0.0, cone_deg: 30.0, pts: pulled(2.0) });
    // the tallest burr where the burnisher goes, well inside its path
    let tallest = |p: &Plate| (60..140).flat_map(|x| (40..60).map(move |y| (x, y))).map(|(x, y)| p.cell(x, y).burr_height()).fold(0.0f32, f32::max);
    let (m0, (b0, _), g0, h0) = (p.metal_um3(), p.burr_um3(), p.groove_um3(), tallest(&p));
    let before = p.clone();
    let line = vec![pt(4.0, 5.0, 30.0, 60.0, 0.0, 30.0), pt(16.0, 5.0, 30.0, 60.0, 0.0, 30.0)];
    p.apply(Op::Burnish { radius_mm: 10.0, passes: 1, pts: line.clone() });
    let (b1, h1) = (p.burr_um3().0, tallest(&p));
    assert!(b1 < b0 && h1 < h0, "the burr is lower: {h0} → {h1} µm");
    assert!((p.metal_um3() - m0).abs() < 1e-3 * g0, "metal conserved: {m0} → {}", p.metal_um3());
    // the flattened fin folded onto the land: the line is as it was while burr holds the burnisher off
    assert!((g0 - p.groove_um3()).abs() < 0.05 * g0, "{g0} → {}", p.groove_um3());
    for _ in 0..12 {
        p.apply(Op::Burnish { radius_mm: 10.0, passes: 1, pts: line.clone() });
    }
    assert!((p.metal_um3() - m0).abs() < 1e-3 * g0);
    // the line's rims: Hertz's peak pressure against the rim's slip-line limit for this groove's walls
    let e: f64 = 1.0 / ((1.0 - 0.09) / 210e9 + (1.0 - 0.34f64.powi(2)) / 125e9);
    let a = (3.0 * 30.0 * 0.01 / (4.0 * e)).cbrt();
    let p0 = 3.0 * 30.0 / (2.0 * PI * a * a);
    let wall = (90f64 - 30.0).to_radians();
    let rim = meyer(90.0) * (1.0 + (PI - wall) - PI / 2.0) / (1.0 + PI / 2.0);
    assert!(p0 > rim, "this burnisher flows the rims: {p0} > {rim}");
    let c = p.cell_at_mm(10.0, 5.0).unwrap();
    assert!(c.burr_vol == 0.0, "the burr is flat after the passes");
    // then the line itself closes, where the burnisher went
    let inside = |p: &Plate| (60..140).flat_map(|x| (40..60).map(move |y| (x, y))).map(|(x, y)| p.cell(x, y).groove_vol() as f64).sum::<f64>();
    assert!(inside(&p) < 0.9 * inside(&before), "{} → {}", inside(&before), inside(&p));
}

/// The scraper takes the burr off and shaves the plate by t = F/(H·b·tan 30°)
/// a stroke; the metal it takes is counted, and is what the plate lost.
#[test]
fn the_scraper_removes_metal() {
    let mut p = tiny(Metal::Copper, Temper::HalfHard);
    p.apply(Op::Drypoint { tip_um: 0.0, cone_deg: 30.0, pts: pulled(2.0) });
    let m0 = p.metal_um3();
    let rep = p.apply(Op::Scrape { width_mm: 4.0, passes: 2, pts: vec![pt(8.0, 5.0, 5.0, 30.0, 0.0, 20.0), pt(12.0, 5.0, 5.0, 30.0, 0.0, 20.0)] });
    let t = 5.0 / (meyer(90.0) * 4e-3 * 30f64.to_radians().tan()) * 1e6;
    assert!((rep.scrape_um / t - 1.0).abs() < 1e-9, "{} µm a stroke against {t}", rep.scrape_um);
    let c = p.cell_at_mm(10.0, 5.0).unwrap();
    assert!(((-c.level) as f64 / (2.0 * t) - 1.0).abs() < 1e-5 && c.burr_vol == 0.0);
    assert!(p.chip_um3 > 0.0 && (m0 - p.metal_um3() - p.chip_um3).abs() < 1e-3 * p.chip_um3, "lost what it took");
    // an untouched cell keeps its level
    assert_eq!(p.cell_at_mm(1.0, 1.0).unwrap().level, 0.0);
}

/// A roulette's teeth each indent a pit at the hardness, πa² = F/H, one a
/// pitch, and throw up a ring of their volume.
#[test]
fn a_roulette_pits_at_its_pitch() {
    let mut p = tiny(Metal::Copper, Temper::Annealed);
    let rep = p.apply(Op::Roulette { pitch_mm: 0.5, tooth_um: 30.0, cone_deg: 45.0, pts: vec![pt(5.0, 5.0, 3.0, 90.0, 0.0, 1.0), pt(15.0, 5.0, 3.0, 90.0, 0.0, 1.0)] });
    let a = (3.0 / (PI * meyer(45.0))).sqrt() * 1e6;
    assert!((rep.width_um / (2.0 * a) - 1.0).abs() < 1e-6, "{} against {}", rep.width_um, 2.0 * a);
    let (_, _, v) = pit(meyer(45.0), 30.0, 45.0, 3.0);
    // 20 teeth over 10 mm, all but the ends at full force
    assert!(p.groove_um3() < 20.0 * v * 1.001 && p.groove_um3() > 18.0 * v);
    assert!((p.burr_um3().0 / p.groove_um3() - 1.0).abs() < 1e-3);
}

/// Copper's reflectance at normal incidence is ((n − 1)² + k²)/((n + 1)² + k²)
/// from Johnson & Christy's constants: at 520 nm, between their 495.9 and
/// 520.9 nm points. A metal reflects more at grazing incidence; a dielectric
/// of n = 1.5 reflects 4 % straight on.
#[test]
fn coppers_reflectance_from_its_optical_constants() {
    let u: f64 = (0.520 - 0.4959) / (0.5209 - 0.4959);
    let (n, k): (f64, f64) = (1.22 + (1.18 - 1.22) * u, 2.564 + (2.608 - 2.564) * u);
    let r = ((n - 1.0).powi(2) + k * k) / ((n + 1.0).powi(2) + k * k);
    let spec = reflectance(Metal::Copper, 0.0);
    assert_eq!(spectral::wavelength(14), 520.0);
    assert!((spec[14] as f64 - r).abs() < 1e-6, "{} against {r}", spec[14]);
    // red above green: copper's colour
    let rgb = Plate::bare_rgb(Metal::Copper);
    assert!(rgb[0] > rgb[1] && rgb[1] > rgb[2], "{rgb:?}");
    assert!((fresnel(1.5, 0.0, 0.0) - 0.04).abs() < 1e-12);
    assert!(fresnel(n, k, 89.9f64.to_radians()) > 0.99);
    // zinc is a bluish grey: flatter across the spectrum than copper
    let zn = reflectance(Metal::Zinc, 0.0);
    let (z0, z1) = (zn[4] as f64, zn[30] as f64);
    let (w0, w1) = nk(Metal::Zinc, 0.42);
    assert!((z0 - ((w0 - 1.0).powi(2) + w1 * w1) / ((w0 + 1.0).powi(2) + w1 * w1)).abs() < 1e-6);
    assert!((z1 / z0) < (spec[30] / spec[4]) as f64);
    // a V-groove traps light: what it returns is less than one reflection at its walls' angle
    let back = groove_return(Metal::Copper, 60f64.to_radians());
    assert!(back[20] < reflectance(Metal::Copper, 60f64.to_radians())[20]);
}

/// The plate's section round trip, and refusals of another version or a
/// cut-short or lengthened section.
#[test]
fn a_plate_survives_its_section() {
    let mut p = tiny(Metal::Zinc, Temper::Rolled);
    p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts: pulled(1.0) });
    p.apply(Op::Roulette { pitch_mm: 0.3, tooth_um: 20.0, cone_deg: 45.0, pts: vec![pt(5.0, 2.0, 1.0, 90.0, 0.0, 20.0), pt(9.0, 2.0, 1.0, 90.0, 0.0, 20.0)] });
    p.apply(Op::Burnish { radius_mm: 8.0, passes: 2, pts: vec![pt(4.0, 5.0, 20.0, 60.0, 0.0, 30.0), pt(8.0, 5.0, 20.0, 60.0, 0.0, 30.0)] });
    p.apply(Op::Scrape { width_mm: 3.0, passes: 1, pts: vec![pt(12.0, 5.0, 4.0, 30.0, 0.0, 20.0), pt(14.0, 5.0, 4.0, 30.0, 0.0, 20.0)] });
    let mut b = Vec::new();
    p.write(&mut b).unwrap();
    let q = Plate::read(&mut std::io::Cursor::new(&b)).unwrap();
    assert!(q == p);
    let mut again = Vec::new();
    q.write(&mut again).unwrap();
    assert!(again == b);
    assert!(p.tiles_used() < p.tiles.len() || p.tiles.len() <= 8);
    // (version 2 is a plate etched, `--@ etch`: an unknown version is 9)
    let mut v9 = b.clone();
    v9[4] = 9;
    assert!(Plate::read(&mut std::io::Cursor::new(&v9)).unwrap_err().to_string().contains("version 9"));
    assert!(Plate::read(&mut std::io::Cursor::new(&b[..b.len() - 3])).is_err());
    let mut longer = b.clone();
    longer.push(0);
    assert!(Plate::read(&mut std::io::Cursor::new(&longer)).is_err());
}

/// A crop rebuilt from the vectors at twice the pitch holds the same groove
/// and burr, drawn at its own scale.
#[test]
fn a_crop_is_rebuilt_from_the_vectors() {
    let mut p = tiny(Metal::Copper, Temper::HalfHard);
    p.apply(Op::Drypoint { tip_um: 0.0, cone_deg: 30.0, pts: pulled(2.0) });
    let fine = p.rebuilt(2, (0, 0, 400, 200));
    assert!((fine.groove_um3() / p.groove_um3() - 1.0).abs() < 0.01);
    assert!((fine.burr_um3().0 / p.burr_um3().0 - 1.0).abs() < 0.01);
    // the window: only its cells
    let half = p.rebuilt(2, (0, 0, 200, 200));
    assert!((half.groove_um3() / p.groove_um3() - 0.5).abs() < 0.05);
    // seen: the line darker than the bare plate, lit or not
    let seen = fine.seen((0, 90, 400, 110), None);
    let bare = Plate::bare_rgb(Metal::Copper);
    assert!(seen.iter().any(|c| c[1] < 0.9 * bare[1]));
    let lit = fine.seen((0, 90, 400, 110), Some((135.0, 25.0)));
    assert!(lit.iter().all(|c| c.iter().all(|v| v.is_finite())));
}


/// Under the contact laws (`groove_streams`) each stream of metal across the
/// groove meets the needle's face at its own slope, cot θ·cos φ on an upright
/// cone: the streams within |u| < a·√(1 − (tan β_c·tan θ)²) of the middle cut
/// a chip, the rest ploughs. An upright 30° cone meets the metal at 60° only
/// at its very front, so it ploughs and throws its burr both sides (held
/// upright, a drypoint needle throws a burr on both sides [P]), where the old
/// law cut; pulled at 55° it ploughs at 25°; within the rounded point, at
/// the sphere's slope a/(R − d), it ploughs even pushed; pushed with a cone,
/// its middle cuts and its flanks plough.
#[test]
fn a_needle_ploughs_or_cuts_stream_by_stream() {
    let h = hardness_pa(90.0);
    let g = groove_streams(h, 25.0, 30.0, 2.0, 0.0, 0.0);
    assert!(groove(h, 25.0, 30.0, 2.0, 0.0, 0.0).cut, "the old law: upright, it cut");
    assert_eq!(g.chip, 0.0);
    assert!(g.burr_l == g.burr_r && (g.burr_l + g.burr_r - g.area).abs() < 1e-9 * g.area);
    assert!((g.beta - 60.0).abs() < 0.01, "{}", g.beta);
    let g = groove_streams(h, 25.0, 30.0, 2.0, 35.0, 0.0);
    assert!(g.chip == 0.0 && (g.beta - 25.0).abs() < 0.01, "{g:?}");
    // fine points held upright: the middle cuts, by the slope of each stream
    for theta in [10.0f64, 15.0, 20.0] {
        let g = groove_streams(h, 25.0, theta, 2.0, 0.0, 0.0);
        let a = 0.5 * g.w;
        assert!(a > 25.0 * theta.to_radians().cos(), "on the cone");
        let uc = a * (1.0 - (BETA_C_DEG.to_radians().tan() * theta.to_radians().tan()).powi(2)).sqrt();
        let n = 20000;
        let (mut all, mut cut) = (0.0, 0.0);
        for i in 0..n {
            let u = -a + (i as f64 + 0.5) * 2.0 * a / n as f64;
            all += g.depth_at(u);
            if u.abs() < uc {
                cut += g.depth_at(u);
            }
        }
        assert!((g.chip - cut / all).abs() < 0.01, "{theta}°: chip {} against {}", g.chip, cut / all);
        assert!(((g.burr_l + g.burr_r) - g.area * (1.0 - g.chip)).abs() < 1e-9 * g.area);
        assert!((g.beta - (90.0 - theta)).abs() < 0.01);
    }
    // a light line, within the point's sphere, pushed: the sphere's slope
    let g = groove_streams(h, 25.0, 30.0, 0.3, -35.0, 0.0);
    let a = 0.5 * g.w;
    assert!(a < 25.0 * 30f64.to_radians().cos());
    assert_eq!(g.chip, 0.0);
    assert!((g.beta - (a / (25.0 - g.d)).atan().to_degrees()).abs() < 0.01);
    // pushed: its front stands at 95°; the flanks, meeting the metal aslant, plough
    let g = groove_streams(h, 25.0, 30.0, 2.0, -35.0, 0.0);
    assert!((g.beta - 95.0).abs() < 0.01 && g.chip > 0.8 && g.chip < 1.0, "{g:?}");
}

/// Under the contact laws a needle following a leftward line turns the short
/// way between azimuths of 179.4° and −179.7°; without them it swings round
/// through 0° (pushed) and cuts there. A plate under them saves as version 4
/// and reads back with them; one without them as before.
#[test]
fn azimuths_turn_the_short_way_and_the_laws_are_kept() {
    let line = |contact: bool| {
        let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 30.0, 20.0, 1.3, 45.0, 300, 200);
        p.contact = contact;
        let pt = |x: f32, y: f32, az: f32| Point { x, y, force: 2.0, alt: 55.0, az, speed: 30.0 };
        let r = p.apply(Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts: vec![pt(25.0, 10.0, 179.4), pt(15.0, 10.1, -179.7), pt(5.0, 10.0, 180.0)] });
        (p, r)
    };
    let (old, ro) = line(false);
    let (new, rn) = line(true);
    assert!(ro.chip_um3 > 0.0 && ro.cut && ro.ploughed, "{ro:?}");
    assert!(rn.chip_um3 == 0.0 && !rn.cut && rn.ploughed, "{rn:?}");
    assert!((rn.groove_um3 - ro.groove_um3).abs() < 1e-9 * ro.groove_um3);
    let bytes = |p: &Plate| {
        let mut b = Vec::new();
        p.write(&mut b).unwrap();
        b
    };
    let (bo, bn) = (bytes(&old), bytes(&new));
    assert_eq!(u64::from_le_bytes(bo[4..12].try_into().unwrap()), 1);
    assert_eq!(u64::from_le_bytes(bn[4..12].try_into().unwrap()), 4);
    assert_eq!(bn.len(), bo.len() + 16);
    let back = Plate::read(&mut &bn[..]).unwrap();
    assert!(back == new && back.contact);
    assert!(!Plate::read(&mut &bo[..]).unwrap().contact);
    let mut bad = bn.clone();
    bad[12] = 2;
    assert!(Plate::read(&mut &bad[..]).is_err(), "laws it doesn't know");
}
