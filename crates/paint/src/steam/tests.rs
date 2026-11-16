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

fn left(c: &Canvas) -> Mask {
    Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 })
}

/// Sums over the band's left (x < 500) and right halves of `f(cell, pixel)`.
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
        if x < 500.0 { l += v } else { r += v }
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
    let s = c.steam(Some(&left(&c)), 5.0, 200.0).unwrap();
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
    let s = c.sift(None, 50.0, 0.5, &stick(), 7).unwrap();
    assert!((s.laid_mm3 - 50.0).abs() < 1e-9 && s.stuck_mm3 == 0.0 && s.cluster_um == 0.0, "{s:?}");
    let fell = c.tap_pastel(100.0) as f64;
    let dc = (6.0 * 5e-8 / (std::f64::consts::PI * 1250.0 * 9.81 * 100.0)).cbrt() * 1e6;
    let (m, d, lo) = (0.3f64, 500.0f64, 5.0f64);
    let want = 50.0 * (d.powf(m) - dc.powf(m)) / d.powf(m);
    assert!(fell > 25.0, "most of it taps off: {fell} of 50 mm³");
    assert!((fell - want).abs() < 0.03 * want, "{fell} vs {want}");
    assert!((s.fines_mm3 - 50.0 * (lo / d).powf(m)).abs() < 0.01 * s.fines_mm3);
    // a second tap at the same g sheds nothing more; harder, more
    assert!(c.tap_pastel(100.0) < 1e-3);
    assert!(c.tap_pastel(400.0) > 0.5);
}

/// On a steamed sheet the crumbs drink the water standing there, and the
/// rest pulls them into clusters as it dries: more water, larger
/// clusters. While wet a liquid bridge holds them through a tap (a crumb
/// to d² = 12γ/(ρa), 0.59 mm at 100 g, past the sieve's 0.5); dried, the gum
/// binds them.
#[test]
fn sifted_pastel_on_a_damp_sheet_stays_in_clusters_that_grow_with_the_water() {
    let mut sizes = Vec::new();
    for secs in [2.0, 20.0] {
        let mut c = paper();
        let st = c.steam(None, secs, 200.0).unwrap();
        let s = c.sift(None, 50.0, 0.5, &stick(), 7).unwrap();
        assert!(s.stuck_mm3 > 0.7 * 50.0 && s.loose_mm3 == 0.0, "{secs} s: {s:?}");
        assert!(s.film_um > 0.0 && s.film_um < st.water_um, "{s:?} {st:?}");
        // the drops the film breaks into, R = 3h/(0.5 f(105°)), take crumbs to 2R
        let th = 105f64.to_radians();
        let f = (2.0 - 3.0 * th.cos() + th.cos().powi(3)) / th.sin().powi(3);
        let r = 3.0 * s.film_um / (0.5 * f);
        assert!(s.cluster_um > 0.5 * r && s.cluster_um < 4.0 * r, "{secs} s: clusters {} µm, drops {r} µm", s.cluster_um);
        sizes.push(s.cluster_um);
        let fell = c.tap_pastel(100.0) as f64;
        assert!(fell < 0.05 * 50.0, "{secs} s: wet, it stays: {fell}");
        c.wait(600.0);
        let (lo, held) = c.perched_mm3();
        assert!(held > 0.9 * (lo + held), "{secs} s: dried, the gum binds it: loose {lo}, held {held}");
        assert!((c.tap_pastel(100.0) as f64) < 0.05 * 50.0);
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
            c.steam(None, steam, 200.0).unwrap();
        }
        let before = c.pastel_mm3();
        let s = c.sift(None, 30.0, 0.5, &stick(), 3).unwrap();
        let on = c.pastel_mm3() - before + s.caught_mm3;
        assert!((on - 30.0).abs() < 1e-3 * 30.0, "steam {steam}: {on} on the sheet of 30");
        let fell = c.tap_pastel(300.0) as f64;
        let left = c.pastel_mm3() - before;
        assert!((left + fell - 30.0).abs() < 2e-3 * 30.0, "steam {steam}: {left} left + {fell} fell");
    }
}

/// A puff takes dry sifted crumbs as it takes the heap; wet ones it leaves.
#[test]
fn a_puff_blows_dry_crumbs_off_and_leaves_wet_ones() {
    let mut dry = paper();
    dry.sift(None, 30.0, 0.5, &stick(), 3).unwrap();
    let (l0, _) = dry.perched_mm3();
    dry.blow_pastel(500.0, 500.0, 50.0, 12.0, 8.0);
    let (l1, _) = dry.perched_mm3();
    assert!(l1 < 0.95 * l0, "{l1} of {l0}");
    let mut wet = paper();
    wet.steam(None, 5.0, 200.0).unwrap();
    wet.sift(None, 30.0, 0.5, &stick(), 3).unwrap();
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
    c.steam(Some(&left(&c)), 3.0, 200.0).unwrap();
    c.sift(None, 20.0, 0.5, &stick(), 5).unwrap();
    let mut b = Vec::new();
    c.write_state(&mut b, "").unwrap();
    let (r, _) = Canvas::read_state(&mut b.as_slice()).unwrap();
    assert_eq!(r.drawing.as_ref().unwrap().perched, c.drawing.as_ref().unwrap().perched);
    assert_eq!(r.pixels(), c.pixels());
    assert_eq!(r.perched_mm3(), c.perched_mm3());
}

