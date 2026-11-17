//! Tiny scenes (160 px, a 150 mm sheet) for steam and sifted pastel. The
//! expected numbers are worked out here from the published laws the module
//! cites (the jet's decay, a semi-infinite slab's heat uptake, the crushing's
//! distribution, a grain's adhesion), never from a run of the code itself.

use super::*;
use crate::mask::Mask;
use crate::pastel::{Pose, StrokePoint};

fn paper() -> Canvas {
    crate::tests::drawing_paper(160).with_drawing_optics(true).with_wet_pastel(true)
}

fn stick() -> Stick {
    let mut s = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
    s.index = 1.5;
    s
}

/// A band of dark pastel across the sheet at y 450–550.
fn band(c: &mut Canvas) {
    let mut st = stick();
    let pose = Pose { force: 3.0, alt: 60f32.to_radians(), az: 45f32.to_radians(), roll: 0.0 };
    for k in 0..6 {
        let y = 455.0 + 18.0 * k as f32;
        c.stick_stroke(&mut st, &[StrokePoint { x: 100.0, y, pose, speed: 50.0 }, StrokePoint { x: 900.0, y, pose, speed: 50.0 }]);
    }
}

/// A pass of the kettle along the band's left end, 100 mm off: its jet
/// reaches 26 mm (175 units) about its track, so it steams the band to
/// x ≈ 470 and leaves x > 650 dry.
const LEFT_PASS: &[(f32, f32)] = &[(100.0, 500.0), (300.0, 500.0)];
const LEFT_SECS: f64 = 20.0;
const LEFT_MM: f64 = 100.0;
/// The middle of the 150 mm sheet: a kettle or a sieve held there.
const MIDDLE: &[(f32, f32)] = &[(500.0, 500.0)];
/// A tea strainer, mm across (the easel's default).
const SIEVE: f64 = 60.0;

/// Sums over the band's left (x < 350) and right (x > 650) parts of `f(cell, pixel)`.
fn halves(c: &Canvas, f: impl Fn(&Cell, crate::color::Rgb) -> f32) -> (f32, f32) {
    let w = c.window();
    let d = c.drawing.as_ref().unwrap();
    let (mut l, mut r) = (0.0, 0.0);
    for (i, cell) in d.cells.iter().enumerate() {
        let (x, y) = (w.ux(i % w.w), w.uy(i / w.w));
        if !(150.0..850.0).contains(&x) || !(450.0..560.0).contains(&y) {
            continue;
        }
        let v = f(cell, c.pixels()[i]);
        if x < 350.0 {
            l += v
        } else if x > 650.0 {
            r += v
        }
    }
    (l, r)
}

/// The jet at the sheet is the round jet's decay of the steam's share,
/// 5.4 (d/x) √(ρ_steam/ρ_air), and a fog: saturated, its dew point its
/// temperature; farther off it is weaker and cooler.
#[test]
fn the_kettle_jet_mixes_as_a_round_jet_into_a_warm_fog() {
    for x in [150.0, 200.0, 300.0] {
        let p = plume(x);
        let want = 5.4 * (10.0 / x) * (0.598f64 / 1.204).sqrt();
        assert!((p.steam - want).abs() < 1e-9, "{x}: {} vs {want}", p.steam);
        assert!(p.temp_c > 40.0 && p.temp_c < 100.0, "{x}: {}", p.temp_c);
        assert!((p.dew_c - p.temp_c).abs() < 0.5, "{x}: a fog, saturated: dew {} at {}", p.dew_c, p.temp_c);
    }
    assert!(plume(300.0).temp_c < plume(150.0).temp_c && plume(300.0).rho_v < plume(150.0).rho_v);
    // the steam share off the axis is Gaussian, half at 0.11 x; it condenses
    // out to where the room's air with that share of steam is saturated at
    // the room's 20 °C (Buck: 2339 Pa; at 50 % its vapour share is half)
    for x in [100.0, 200.0, 400.0] {
        let p = plume(x);
        assert!((p.scalar_half_m - 0.11 * x * 1e-3).abs() < 1e-12);
        assert!((p.share_at(p.scalar_half_m) - 0.5).abs() < 1e-12);
        let xs = 2339.3 / 101_325.0;
        let ysat = xs * 18.015 / (xs * 18.015 + (1.0 - xs) * 28.965);
        let ya = 0.5 * ysat;
        let s_dew = (ysat - ya) / (1.0 - ya);
        let want = p.scalar_half_m * ((p.steam / s_dew).ln() / std::f64::consts::LN_2).sqrt();
        assert!((p.reach_m - want).abs() < 2e-3 * want, "{x}: reach {} vs {want}", p.reach_m);
        assert!(p.share_at(p.reach_m * 1.001) == 0.0 && p.share_at(p.reach_m * 0.999) > 0.0);
    }
    // at 200 mm: about 2.2 half-widths, 47 mm
    assert!((plume(200.0).reach_m - 0.047).abs() < 0.002, "{}", plume(200.0).reach_m);
}

/// Held still, each point is steamed for the seconds times the steam share
/// on it; a long straight pass at v steams a point y beside it for the
/// Gaussian's integral, b √(π/ln 2) e^(−ln 2 y²/b²) / v (near the track, to
/// within the 2 % the reach cuts from its tails).
#[test]
fn the_jet_steams_each_point_for_the_time_its_share_is_on_it() {
    let c = paper();
    let pl = plume(200.0);
    let w = c.window();
    let mpu = c.mm_per_unit() as f64;
    let held = c.steam_exposure(MIDDLE, 10.0, &pl);
    for (i, &t) in held.iter().enumerate() {
        let (x, y) = ((w.ux(i % w.w) as f64 - 500.0) * mpu, (w.uy(i / w.w) as f64 - 500.0) * mpu);
        let r = (x * x + y * y).sqrt() * 1e-3;
        assert!((t - 10.0 * pl.share_at(r)).abs() < 1e-9, "{r}: {t}");
    }
    // a pass along y = 500 across the whole sheet and beyond, 300 mm in 10 s
    let pass = [(-500.0, 500.0), (1500.0, 500.0)];
    let along = c.steam_exposure(&pass, 10.0, &pl);
    let b = pl.scalar_half_m * 1e3;
    for (i, &t) in along.iter().enumerate() {
        let (x, y) = (w.ux(i % w.w), (w.uy(i / w.w) as f64 - 500.0) * mpu);
        if !(200.0..800.0).contains(&x) || y.abs() > 0.2 * pl.reach_m * 1e3 {
            continue;
        }
        let want = b * (std::f64::consts::PI / std::f64::consts::LN_2).sqrt() * (-std::f64::consts::LN_2 * y * y / (b * b)).exp() / 30.0;
        assert!(t <= want && t > 0.98 * want, "{y} mm off: {t} vs {want}");
    }
    // the same pass in two legs, or doubling back, adds up as time does
    let legs = c.steam_exposure(&[(-500.0, 500.0), (500.0, 500.0), (1500.0, 500.0)], 10.0, &pl);
    assert!(legs.iter().zip(&along).all(|(a, b)| (a - b).abs() < 1e-6 * b.max(1e-3)));
    let back = c.steam_exposure(&[(-500.0, 500.0), (1500.0, 500.0), (-500.0, 500.0)], 20.0, &pl);
    assert!(back.iter().zip(&along).all(|(a, b)| (a - 2.0 * b).abs() < 1e-6 * b.max(1e-3)));
}

/// A sieve held still lays its sift over its mesh's disc; moved, along the
/// band it sweeps, evenly; held over the sheet's corner, a quarter of it
/// lands on the sheet and the rest beside it.
#[test]
fn the_sieve_lays_its_sift_under_its_mesh() {
    let c = paper();
    let held = c.sift_share(MIDDLE, SIEVE);
    let sum: f64 = held.iter().sum();
    assert!((sum - 1.0).abs() < 0.01, "{sum}");
    let w = c.window();
    let mpu = c.mm_per_unit() as f64;
    for (i, &v) in held.iter().enumerate() {
        let r = (((w.ux(i % w.w) as f64 - 500.0).powi(2) + (w.uy(i / w.w) as f64 - 500.0).powi(2)).sqrt()) * mpu;
        assert!((r < 30.0) == (v > 0.0) || (r - 30.0).abs() < 1.0, "{r}: {v}");
    }
    let corner: f64 = c.sift_share(&[(0.0, 0.0)], SIEVE).iter().sum();
    assert!((corner - 0.25).abs() < 0.01, "{corner}");
    // a sweep 600 units (90 mm) across: a band 60 mm wide, the same share
    // all along a row of its middle (the ends' half-discs aside), the
    // sieve's chord across that row, 2√(R² − y²), over its path and area
    let sweep = c.sift_share(&[(200.0, 500.0), (800.0, 500.0)], SIEVE);
    assert!((sweep.iter().sum::<f64>() - 1.0).abs() < 0.01);
    let px_mm2 = (c.px_mm() as f64).powi(2);
    for i in 0..sweep.len() {
        let (x, y) = (w.ux(i % w.w), (w.uy(i / w.w) as f64 - 500.0) * mpu);
        if (400.0..600.0).contains(&x) && y.abs() < 25.0 {
            let want = 2.0 * (900.0 - y * y).sqrt() / 90.0 * px_mm2 / (std::f64::consts::PI * 900.0);
            assert!((sweep[i] - want).abs() < 1e-9, "{y}: {} vs {want}", sweep[i]);
        }
    }
}

/// Once the face is near the fog's dew point, the water condensed is the
/// heat the board behind takes over the latent heat: a semi-infinite slab
/// held ΔT warmer takes 2 e ΔT √(t/π) (e its effusivity, √(kρc)), so it
/// grows nearly as √t, and the sheet's own heat (ρc · caliper · ΔT) comes
/// first. Over a minute it is within a factor of 1.5 of that.
#[test]
fn steam_condenses_as_the_board_takes_its_heat() {
    let p = Paper::drawing(0);
    let c = condense(&p, 200.0, 60.0);
    let dew = plume(200.0).dew_c;
    let dt = dew - 20.0;
    let e = (0.072f64 * 480.0 * 1340.0).sqrt();
    let sheet = p.grammage as f64 * 1e-3 * 1340.0 * dt;
    let l = 2.501e6 - 2370.0 * dew;
    let want = |t: f64| (2.0 * e * dt * (t / std::f64::consts::PI).sqrt() + sheet) / l * 1e3;
    for t in [20.0, 60.0] {
        let got = c.at(t);
        assert!(got > want(t) / 1.5 && got < want(t) * 1.5, "{t} s: {got} µm vs {} µm", want(t));
    }
    let r = c.at(60.0) / c.at(15.0);
    assert!(r > 1.6 && r < 2.1, "60 s against 15 s: {r} (√4 = 2)");
    assert!(*c.face_c.last().unwrap() < dew && *c.face_c.last().unwrap() > dew - 5.0);
}

/// Steamed loose pastel, dried on the clock, is bound by the gum the water
/// dissolved: the finger moves less of it and the eraser lifts less than
/// from the unsteamed half.
#[test]
fn steamed_pastel_once_dry_lifts_and_smudges_less() {
    let mut c = paper();
    band(&mut c);
    let loose0 = halves(&c, |cl, _| cl.loose);
    let s = c.steam(LEFT_PASS, LEFT_SECS, LEFT_MM).unwrap();
    assert!(s.water_um > 5.0, "{s:?}");
    // nothing moves sideways: the pastel is where it was
    let after = halves(&c, |cl, _| cl.loose + cl.bound);
    assert!((after.0 - loose0.0).abs() < 1e-3 * loose0.0 && (after.1 - loose0.1).abs() < 1e-6 * loose0.1);
    c.wait(240.0);
    assert!(halves(&c, |cl, _| cl.water).0 == 0.0, "dry");
    let (bl, br) = halves(&c, |cl, _| cl.bound);
    assert!(bl > 0.9 * loose0.0 && br == 0.0, "the steamed half is bound: {bl} of {}, {br}", loose0.0);
    // the finger
    let mut f = c.clone();
    let before = halves(&f, |_, p| p[0] + p[1] + p[2]);
    for k in 0..4 {
        let y = 470.0 + 20.0 * k as f32;
        f.rub(&[(160.0, y), (840.0, y)], 2.0, None, 50.0);
    }
    let moved = halves(&f, |_, p| p[0] + p[1] + p[2]);
    let (ml, mr) = ((moved.0 - before.0).abs(), (moved.1 - before.1).abs());
    assert!(ml < 0.25 * mr, "the finger moves less of the steamed half: {ml} vs {mr}");
    // the eraser
    let mut e = c.clone();
    let full = Mask::full(e.frame());
    let a0 = halves(&e, |cl, _| cl.a);
    e.erase(&full, 1.0);
    let a1 = halves(&e, |cl, _| cl.a);
    assert!(a0.0 - a1.0 < 0.25 * (a0.1 - a1.1), "the eraser lifts less of the steamed half: {} vs {}", a0.0 - a1.0, a0.1 - a1.1);
}

/// Crushed pastel sifted on a dry sheet lies loose on the tooth: a tap at
/// 100 g sheds every crumb heavier than a grain's 50 nN hold, d_c =
/// (6F/πρa)^⅓ ≈ 43 µm, which by the crushing's distribution (mass finer
/// than d: (d/D)^0.3, D the sieve's 0.5 mm) is most of what was sifted,
/// the fines below 5 µm going into the pores.
#[test]
fn sifted_pastel_on_a_dry_sheet_mostly_taps_off() {
    let mut c = paper();
    let s = c.sift(MIDDLE, 50.0, 0.5, SIEVE, &stick(), 7).unwrap();
    assert!((s.laid_mm3 - 50.0).abs() < 0.01 * 50.0 && s.stuck_mm3 == 0.0 && s.cluster_um == 0.0, "{s:?}");
    let fell = c.tap_pastel(100.0) as f64;
    let dc = (6.0 * 5e-8 / (std::f64::consts::PI * 1250.0 * 9.81 * 100.0)).cbrt() * 1e6;
    let (m, d, lo) = (0.3f64, 500.0f64, 5.0f64);
    let want = s.laid_mm3 * (d.powf(m) - dc.powf(m)) / d.powf(m);
    assert!(fell > 25.0, "most of it taps off: {fell} of 50 mm³");
    assert!((fell - want).abs() < 0.03 * want, "{fell} vs {want}");
    assert!((s.fines_mm3 - s.laid_mm3 * (lo / d).powf(m)).abs() < 0.01 * s.fines_mm3);
    // a second tap at the same g sheds nothing more; harder, more
    assert!(c.tap_pastel(100.0) < 1e-3);
    assert!(c.tap_pastel(400.0) > 0.5);
}

/// On a steamed sheet the crumbs drink the water standing there, and the
/// rest pulls them into clusters as it dries: more water, larger
/// clusters. While wet a liquid bridge holds them through a tap (a crumb
/// to d² = 12γ/(ρa), 0.59 mm at 100 g, past the sieve's 0.5); dried, the gum
/// binds them. (The kettle and the sieve are held over the middle; 6 mm³
/// over the sieve's 60 mm is as dense as 50 over the whole sheet.)
#[test]
fn sifted_pastel_on_a_damp_sheet_stays_in_clusters_that_grow_with_the_water() {
    let mut sizes = Vec::new();
    for secs in [2.0, 20.0] {
        let mut c = paper();
        let st = c.steam(MIDDLE, secs, 200.0).unwrap();
        let s = c.sift(MIDDLE, 6.0, 0.5, SIEVE, &stick(), 7).unwrap();
        assert!(s.stuck_mm3 > 0.7 * 6.0 && s.loose_mm3 == 0.0, "{secs} s: {s:?}");
        assert!(s.film_um > 0.0 && s.film_um < st.water_um, "{s:?} {st:?}");
        // the drops the film breaks into, R = 3h/(0.5 f(105°)), take crumbs to 2R
        let th = 105f64.to_radians();
        let f = (2.0 - 3.0 * th.cos() + th.cos().powi(3)) / th.sin().powi(3);
        let r = 3.0 * s.film_um / (0.5 * f);
        assert!(s.cluster_um > 0.5 * r && s.cluster_um < 4.0 * r, "{secs} s: clusters {} µm, drops {r} µm", s.cluster_um);
        sizes.push(s.cluster_um);
        let fell = c.tap_pastel(100.0) as f64;
        assert!(fell < 0.05 * 6.0, "{secs} s: wet, it stays: {fell}");
        c.wait(600.0);
        let (lo, held) = c.perched_mm3();
        assert!(held > 0.9 * (lo + held), "{secs} s: dried, the gum binds it: loose {lo}, held {held}");
        assert!((c.tap_pastel(100.0) as f64) < 0.05 * 6.0);
    }
    assert!(sizes[1] > 1.5 * sizes[0], "clusters grow with the water: {sizes:?}");
}

/// What is sifted is on the sheet, and what a tap sheds is what left it:
/// the stick's worn volume, the sheet's and what fell add up, dry or damp.
#[test]
fn sifting_and_tapping_conserve_the_pastel() {
    for steam in [0.0, 5.0] {
        let mut c = paper();
        if steam > 0.0 {
            c.steam(MIDDLE, steam, 200.0).unwrap();
        }
        let before = c.pastel_mm3();
        let s = c.sift(MIDDLE, 30.0, 0.5, SIEVE, &stick(), 3).unwrap();
        assert!((s.laid_mm3 + s.off_mm3 - 30.0).abs() < 1e-9 && s.off_mm3 < 0.01 * 30.0, "{s:?}");
        let on = c.pastel_mm3() - before + s.caught_mm3;
        assert!((on - s.laid_mm3).abs() < 1e-3 * 30.0, "steam {steam}: {on} on the sheet of {}", s.laid_mm3);
        let fell = c.tap_pastel(300.0) as f64;
        let left = c.pastel_mm3() - before;
        assert!((left + fell - s.laid_mm3).abs() < 2e-3 * 30.0, "steam {steam}: {left} left + {fell} fell");
    }
}

/// A puff takes dry sifted crumbs as it takes the heap; wet ones it leaves.
#[test]
fn a_puff_blows_dry_crumbs_off_and_leaves_wet_ones() {
    let mut dry = paper();
    dry.sift(MIDDLE, 30.0, 0.5, SIEVE, &stick(), 3).unwrap();
    let (l0, _) = dry.perched_mm3();
    dry.blow_pastel(500.0, 500.0, 50.0, 12.0, 8.0);
    let (l1, _) = dry.perched_mm3();
    assert!(l1 < 0.95 * l0, "{l1} of {l0}");
    let mut wet = paper();
    wet.steam(MIDDLE, 5.0, 200.0).unwrap();
    // (3.5 mm³ over the sieve's 60 mm: as thin as 30 over the whole sheet, no heap
    // over the pores)
    wet.sift(MIDDLE, 3.5, 0.5, SIEVE, &stick(), 3).unwrap();
    let w0 = wet.perched_mm3();
    wet.blow_pastel(500.0, 500.0, 50.0, 12.0, 8.0);
    assert_eq!(wet.perched_mm3(), w0);
}

/// The crumbs' record goes through a checkpoint (engine bit 54); a canvas
/// that never sifted writes no such section.
#[test]
fn the_checkpoint_keeps_the_sifted_crumbs() {
    let mut c = paper();
    band(&mut c);
    let mut plain = Vec::new();
    c.write_state(&mut plain, "").unwrap();
    let (r, _) = Canvas::read_state(&mut plain.as_slice()).unwrap();
    assert!(r.drawing.as_ref().unwrap().perched.is_empty());
    c.steam(LEFT_PASS, 6.0, LEFT_MM).unwrap();
    c.sift(MIDDLE, 20.0, 0.5, SIEVE, &stick(), 5).unwrap();
    let mut b = Vec::new();
    c.write_state(&mut b, "").unwrap();
    let (r, _) = Canvas::read_state(&mut b.as_slice()).unwrap();
    assert_eq!(r.drawing.as_ref().unwrap().perched, c.drawing.as_ref().unwrap().perched);
    assert_eq!(r.pixels(), c.pixels());
    assert_eq!(r.perched_mm3(), c.perched_mm3());
}

