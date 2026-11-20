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

/// A puff takes dry sifted crumbs as it takes the heap; wet ones it leaves
/// (the thin sift: no heap over the pores), on a soaked sheet whose water
/// feeds their bridges from beneath (`Wet::Film`).
#[test]
fn a_puff_blows_dry_crumbs_off_and_leaves_wet_ones() {
    let mut dry = paper();
    dry.sift(MIDDLE, 30.0, 0.5, SIEVE, &stick(), 3).unwrap();
    let (l0, _) = dry.perched_mm3();
    dry.blow_pastel_with(500.0, 500.0, 50.0, 12.0, 8.0, true);
    let (l1, _) = dry.perched_mm3();
    assert!(l1 < 0.95 * l0, "{l1} of {l0}");
    let mut wet = soaked();
    // (3.5 mm³ over the sieve's 60 mm: as thin as 30 over the whole sheet, no heap
    // over the pores)
    wet.sift(MIDDLE, 3.5, 0.5, SIEVE, &stick(), 3).unwrap();
    let w0 = wet.perched_mm3();
    wet.blow_pastel_with(500.0, 500.0, 50.0, 12.0, 8.0, true);
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


/// A heap: 30 mm³ through a 20 mm sieve, about 95 µm of stick over the mesh's
/// disc, far more than the tooth's pores take.
const HEAP_SIEVE: f64 = 20.0;

fn heap(steamed: bool) -> Canvas {
    let mut c = paper();
    c.sift(MIDDLE, 30.0, 0.5, HEAP_SIEVE, &stick(), 3).unwrap();
    if steamed {
        // (200 mm off for 5 s: water on every cell the sieve reached, 9.5 µm
        // at the middle, all of it in the heap's pores, 40 µm there)
        c.steam(MIDDLE, 5.0, 200.0).unwrap();
    }
    c
}

/// A sheet soaked 5 minutes in a tray and drained 30 s: its pores hold free
/// water (`crate::damp`), which the crumbs sifted on it drink and which
/// feeds their bridges from beneath.
fn soaked() -> Canvas {
    let mut c = paper();
    c.damp_soak(5.0, 30.0).unwrap();
    c
}

/// The heap, sifted on the soaked sheet.
fn soaked_heap() -> Canvas {
    let mut c = soaked();
    c.sift(MIDDLE, 30.0, 0.5, HEAP_SIEVE, &stick(), 3).unwrap();
    c
}

/// How the water lies at every cell holding perched crumbs, by the
/// definitions of §5: none; past the pores (the deposit's and the
/// crumbs' as crushed) by more than l_R²/πR (0.85 µm grains, the
/// smallest crumb's 2.5 µm radius: 0.092 µm), or over a wet sheet; else
/// only in the pores.
fn water_lies(c: &Canvas) -> (usize, usize, usize) {
    let d = c.drawing.as_ref().unwrap();
    let free = c.damp_free();
    let (mut dry, mut pores, mut film) = (0, 0, 0);
    for (i, cl) in d.cells.iter().enumerate() {
        let Some(p) = d.perched.get(i * PERCH..(i + 1) * PERCH) else { break };
        if p[P_V] <= 0.0 {
            continue;
        }
        let under = d.sheet.get(i).copied().unwrap_or(0.0) + free;
        let held = 0.55 * (cl.loose + cl.bound + p[P_V] - p[P_L]);
        if cl.water <= 0.0 {
            dry += 1
        } else if cl.water - held > 0.85 * 0.85 / (std::f32::consts::PI * 2.5) || under > 0.0 {
            film += 1
        } else {
            pores += 1
        }
    }
    (dry, pores, film)
}

/// A hard blow (12 m/s from the lips, 8 mm, 50 mm off) clears a dry heap and
/// leaves a wet one whole (sifted on a soaked sheet: a film's bridges). Its wall shear peaks at τ = 44.6 ρU² Re^(−½)
/// (h/d)^(−2) = 2.5 Pa, u* 1.43 m/s, past a dry 100 µm crumb's 0.2 m/s
/// (Shao & Lu) sevenfold. A wet crumb's bridge, 2πγd with γ 50 mN/m (31 µN
/// at 100 µm, 630 times a dry grain's 50 nN), makes Shao & Lu's cohesion
/// 3e-4 × 630 = 0.19 N/m: the threshold √(0.0123 (ρ g d/ρ_a + 0.19/(ρ_a d)))
/// (ρ 1750) is least for the sieve's largest, 0.5 mm, 2.0 m/s, and the ramp
/// starts at 0.8 of it, 1.6 m/s: none moves.
#[test]
fn a_hard_blow_clears_a_dry_heap_and_leaves_a_wet_one() {
    let mut dry = heap(false);
    let (l0, _) = dry.perched_mm3();
    dry.blow_pastel_with(500.0, 500.0, 50.0, 12.0, 8.0, true);
    let (l1, _) = dry.perched_mm3();
    assert!(l0 > 20.0 && l1 < 0.05 * l0, "the dry heap: {l1} of {l0} left");
    let mut wet = soaked_heap();
    assert_eq!(water_lies(&wet).1, 0, "every crumb over the soaked sheet");
    let (w0, p0, px0) = (wet.perched_mm3(), wet.pastel_mm3(), wet.pixels().to_vec());
    let off = wet.blow_pastel_with(500.0, 500.0, 50.0, 12.0, 8.0, true);
    assert_eq!((wet.perched_mm3(), wet.pastel_mm3(), off), (w0, p0, 0.0));
    assert_eq!(wet.pixels(), px0);
}

/// A bulb's hard puff close in (60 m/s through 4 mm, 20 mm off: u* up to
/// 5.9 m/s on a ring 1.8 mm out, falling as r^(−1.15) past it) moves the
/// wet heap's larger crumbs, near the ring only. A wet crumb's threshold
/// falls with its size (the bridge grows as d, the drag as d²): 0.5 mm ones
/// go from 1.6 m/s, 0.1 mm ones from 3.5, and under about 35 µm none can;
/// so less leaves the wet heap than the dry one, and some does.
#[test]
fn a_bulb_close_in_moves_only_the_largest_wet_crumbs() {
    let mut dry = heap(false);
    let mut wet = soaked_heap();
    let (d0, _) = dry.perched_mm3();
    let (w0, _) = wet.perched_mm3();
    dry.blow_pastel_with(500.0, 500.0, 20.0, 60.0, 4.0, true);
    wet.blow_pastel_with(500.0, 500.0, 20.0, 60.0, 4.0, true);
    let moved = |a: f64, b: f64| a - b;
    let (dm, wm) = (moved(d0, dry.perched_mm3().0), moved(w0, wet.perched_mm3().0));
    assert!(wm > 0.0 && wm < 0.5 * dm, "wet {wm} mm³ moved, dry {dm}");
}

/// Off the switch (`blow_pastel`, as every log without `--@ steam-sift`
/// replays) a puff still moves wet pastel as it did; on it, dry pastel
/// moves exactly as before.
#[test]
fn bridges_change_only_wet_pastel_on_the_switch() {
    let (mut a, mut b) = (heap(false), heap(false));
    let va = a.blow_pastel(500.0, 500.0, 50.0, 12.0, 8.0);
    let vb = b.blow_pastel_with(500.0, 500.0, 50.0, 12.0, 8.0, true);
    assert_eq!(va, vb);
    assert_eq!(a.pixels(), b.pixels());
    let mut old = heap(true);
    let (w0, _) = old.perched_mm3();
    old.blow_pastel(500.0, 500.0, 50.0, 12.0, 8.0);
    assert!(old.perched_mm3().0 < 0.5 * w0, "off the switch the wet heap blows as before");
}

/// The bridge pore water alone makes (§5): full pores give water up only at
/// their entry suction 2γ/r_p (r_p = 2εd/6(1 − ε) = 0.69 µm at ε 0.55,
/// d 1.7 µm), so a bridge they feed spans gaps to r_p, under the 0.85 µm a
/// crumb's grains stand proud: it holds at the grain it touches, 4πγa =
/// 4π × 0.05 × 0.85 µm = 0.534 µN (van Zwol et al. 2007's rough limit; a
/// film's 4πγR at 100 µm is 31 µN). With the dry 50 nN, against a wet
/// crumb (1250 + 0.5 × 998.2 kg/m³) at 100 g: d = (6F/πρa)^⅓ = 86.6 µm.
#[test]
fn pore_water_bridges_at_the_touching_grain() {
    let rp = 2.0 * 0.55 * 1.7 / (6.0 * 0.45);
    assert!((pore_radius_um() - rp).abs() < 1e-6 && (rp - 0.6926).abs() < 1e-3);
    assert!(pore_bridge_reach_um() < GRAIN_RELIEF_UM, "{} µm under {} µm", pore_bridge_reach_um(), GRAIN_RELIEF_UM);
    let f = 4.0 * std::f64::consts::PI * 0.05 * 0.85e-6;
    assert!((pore_bridge_n() - f).abs() < 1e-12 && (f - 0.534e-6).abs() < 1e-9);
    let d = (6.0 * (5e-8 + f) / (std::f64::consts::PI * 1749.1 * 9.81 * 100.0)).cbrt() * 1e6;
    assert!((pore_fall_um(100.0) - d).abs() < 0.01 && (d - 86.6).abs() < 0.1, "{d}");
    assert!(dry_fall_um(100.0) < pore_fall_um(100.0) && pore_fall_um(100.0) < wet_fall_um(100.0));
    assert!((film_saturates_um() - 0.85 * 0.85 / (std::f64::consts::PI * 2.5)).abs() < 1e-6);
}

/// A tap at 100 g on a band of stroke pastel half steamed (1 s, 100 mm
/// off: some cells' water past their pores, some only in them). A
/// stroke's crumbs are even in mass by area from 5 µm to 0.3 mm, so a
/// tap sheds (A_max − d_c²)/(A_max − A_min) of each heap: dry, d_c = 43.3
/// µm (a grain's 50 nN), 0.979; water only in the pores, d_c = 86.6 µm
/// (`pore_water_bridges_at_the_touching_grain`), 0.916; a film, d_c =
/// √(12γ/ρga) = 585 µm, past the largest: none. Off the switch (every
/// log without `--@ steam-sift`) it sheds every heap as dry, as before.
#[test]
fn a_tap_holds_wet_stroke_pastel_as_its_water_lies() {
    let mut c = paper();
    band(&mut c);
    c.steam(LEFT_PASS, 1.0, LEFT_MM).unwrap();
    let share = |d: f64| ((90000.0 - (d * d).max(25.0)) / (90000.0 - 25.0)).clamp(0.0, 1.0);
    let dc_dry = (6.0 * 5e-8 / (std::f64::consts::PI * 1250.0 * 9.81 * 100.0)).cbrt() * 1e6;
    let dc_pores = (6.0 * (5e-8 + 4.0 * std::f64::consts::PI * 0.05 * 0.85e-6) / (std::f64::consts::PI * 1749.1 * 9.81 * 100.0)).cbrt() * 1e6;
    let dc_film = (12.0f64 * 0.05 / (1749.1 * 9.81 * 100.0)).sqrt() * 1e6;
    assert!(dc_film > 300.0);
    let px_um2 = (c.px_mm() as f64 * 1e3).powi(2);
    let d = c.drawing.as_ref().unwrap();
    let (mut want_on, mut want_off, mut n) = (0.0f64, 0.0f64, [0usize; 3]);
    for (i, cl) in d.cells.iter().enumerate() {
        let heap = ((cl.loose + cl.bound) - c.micro_um(i)).max(0.0).min(cl.loose) as f64;
        if heap <= 0.0 {
            continue;
        }
        let pores = 0.55 * (cl.loose + cl.bound);
        let k = if cl.water <= 0.0 { 0 } else if (cl.water - pores) as f64 > film_saturates_um() { 2 } else { 1 };
        n[k] += 1;
        want_on += heap * share([dc_dry, dc_pores, dc_film][k]);
        want_off += heap * share(dc_dry);
    }
    assert!(n.iter().all(|&k| k > 50), "dry, pore-wet and film-wet cells: {n:?}");
    let mut off = c.clone();
    let on = c.tap_pastel_with(100.0, true) as f64;
    let (want_on, want_off) = (want_on * px_um2 * 1e-9, want_off * px_um2 * 1e-9);
    assert!((on - want_on).abs() < 1e-3 * want_on, "{on} vs {want_on} mm³");
    let shed_off = off.tap_pastel(100.0) as f64;
    assert!((shed_off - want_off).abs() < 1e-3 * want_off, "{shed_off} vs {want_off} mm³");
    assert!(on < shed_off, "{on} of {shed_off} mm³");
}

/// Dry pastel taps and blows exactly as before on the switch: the same
/// volume, pixels and state, a band and a sift alike.
#[test]
fn bridges_leave_dry_pastel_as_it_was() {
    let mut a = paper();
    band(&mut a);
    a.sift(MIDDLE, 20.0, 0.5, SIEVE, &stick(), 5).unwrap();
    let mut b = a.clone();
    let state = |c: &Canvas| {
        let mut v = Vec::new();
        c.write_state(&mut v, "").unwrap();
        v
    };
    assert_eq!(a.blow_pastel(500.0, 500.0, 20.0, 60.0, 4.0).to_bits(), b.blow_pastel_with(500.0, 500.0, 20.0, 60.0, 4.0, true).to_bits());
    assert_eq!(a.tap_pastel(100.0).to_bits(), b.tap_pastel_with(100.0, true).to_bits());
    assert!(state(&a) == state(&b));
}

/// A heap wet only in its pores (steamed after it was sifted) holds by the
/// bridges at its touching grains: Shao & Lu's cohesion 3e-4 (1 + 0.534
/// µN/50 nN) = 3.5e-3 N/m, so a wet crumb's threshold is
/// √(0.0123 (ρgd/ρ_a + 3.5e-3/(ρ_a d))): 0.40 m/s at 0.5 mm, 0.61 at
/// 0.1 mm, 1.0 at 35 µm. A hard blow (u* up to 1.43 m/s) moves every crumb
/// over about 27 µm near its ring, some 80 % of the sift by its crushing,
/// less further out: less than the dry heap, where every crumb goes, and
/// far more than a film-held heap, where none does.
#[test]
fn a_hard_blow_moves_a_heap_wet_only_in_its_pores() {
    let moved = |mut c: Canvas| {
        let l0 = c.perched_mm3().0;
        c.blow_pastel_with(500.0, 500.0, 50.0, 12.0, 8.0, true);
        l0 - c.perched_mm3().0
    };
    let pores = heap(true);
    let (dry_cells, pore_cells, film_cells) = water_lies(&pores);
    assert!(dry_cells == 0 && film_cells == 0 && pore_cells > 100, "{:?}", water_lies(&pores));
    let (d, p, f) = (moved(heap(false)), moved(pores), moved(soaked_heap()));
    assert!(f == 0.0 && p > 0.3 * d && p < 0.95 * d, "dry {d}, pores {p}, film {f} mm³");
}

/// A puff that moved a wet heap's larger crumbs leaves its record's
/// largest at what it left: every crumb whose bridged threshold (a film's,
/// 0.19 N/m) is under u*/1.25, the ramp's top, went. A bulb's puff (60 m/s
/// through 4 mm, 20 mm off: τ = 44.6 ρU² Re^(−½) 6^(−2) = 42 Pa at its
/// ring 1.8 mm out, u* 5.9 m/s, rising linearly to it and falling as
/// r^(−2.3) past it) leaves 86 µm crumbs at the ring, solved here from
/// Shao & Lu. A second puff the same then weighs only what is left, so it
/// moves far less than the first.
#[test]
fn a_wet_puff_lowers_the_largest_crumb_left() {
    let mut c = soaked_heap();
    let l0 = c.perched_mm3().0;
    c.blow_pastel_with(500.0, 500.0, 20.0, 60.0, 4.0, true);
    let l1 = c.perched_mm3().0;
    // the jet's wall shear at a radius, mm (Phares et al. 2000)
    let (u, dn, h) = (60.0f64, 4.0f64, 20.0f64);
    let tau_max = 44.6 * 1.2 * u * u / (u * dn * 1e-3 / 1.5e-5).sqrt() / 36.0;
    let rp = 0.09 * h;
    let ustar = |r: f64| (if r <= rp { tau_max * r / rp } else { tau_max * (rp / r).powf(2.3) } / 1.2).sqrt();
    let gamma = 3e-4 * (1.0 + std::f64::consts::TAU * 0.05 * 100e-6 / 5e-8);
    let ut = |d_um: f64| (0.0123 * (1749.1 * 9.81 * d_um * 1e-6 / 1.2 + gamma / (1.2 * d_um * 1e-6))).sqrt();
    let left = |us: f64| {
        let x = us / 1.25;
        if ut(500.0) > x {
            return 500.0;
        }
        let (mut a, mut b) = (5.0f64, 500.0f64);
        for _ in 0..60 {
            let m = 0.5 * (a + b);
            if ut(m) > x { a = m } else { b = m }
        }
        b
    };
    assert!((left(ustar(rp)) - 86.0).abs() < 1.0, "{}", left(ustar(rp)));
    let (w, mpu) = (c.window(), c.mm_per_unit() as f64);
    let d = c.drawing.as_ref().unwrap();
    let mut lowered = 0;
    for (i, _) in d.cells.iter().enumerate() {
        let p = &d.perched[i * PERCH..(i + 1) * PERCH];
        if p[P_V] <= 0.0 {
            continue;
        }
        let (x, y) = (w.ux(i % w.w) as f64 * mpu - 75.0, w.uy(i / w.w) as f64 * mpu - 75.0);
        let want = left(ustar((x * x + y * y).sqrt()).max(1e-9));
        assert!((p[P_TOP] as f64 - want).abs() < 0.01 * want, "at {x:.2}, {y:.2} mm: {} vs {want}", p[P_TOP]);
        lowered += (want < 500.0) as usize;
    }
    assert!(lowered >= 4, "{lowered} cells");
    c.blow_pastel_with(500.0, 500.0, 20.0, 60.0, 4.0, true);
    let l2 = c.perched_mm3().0;
    assert!(l1 < l0 && l1 - l2 < 0.35 * (l0 - l1), "first {} mm³, second {}", l0 - l1, l1 - l2);
}

