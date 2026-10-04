//! Thinner acceptance checks at the paint engine's own boundary (brush,
//! rag, canvas, clock, save): checks 4, 5, 6, 8 (a small thinner), 9, 10,
//! 13, 14, 15, 16, 17 and 19 of notes/thinner/ACCEPTANCE.md. The measuring
//! helpers, and the interface these tests require, are in
//! `thinner_support/mod.rs`.
//!
//!   cargo test -p paint --release --test thinner_physics

mod thinner_support;

use paint::color::luminance;
use paint::pigment::scatter_for;
use paint::rag::Rag;
use paint::thinner::{evaporation_tau_min, stroke_limit_um};
use paint::{Canvas, Stage, Tool, hex};
use thinner_support::*;

/// Check 4. Nothing vanishes: across a stroke (no time passes during one,
/// so nothing evaporates), the paint on the canvas and on the brush adds
/// up to what it was, and so does the solvent, including the paint and
/// solvent the brush picks up from a thinned underlayer. Across a rag wipe
/// what leaves the canvas is what the rag took. Rounding: `BALANCE_REL`.
/// The unthinned brush is the control on today's code path.
#[test]
fn c04_a_stroke_and_a_wipe_account_for_all_paint_and_solvent() {
    let mut c = canvas(240);
    thinned_patch(&mut c, raw_sienna(), 0.5, (150.0, 850.0), (280.0, 420.0), 1);
    let t0 = c.clock();
    assert!(c.solvent_total() > 0.0, "the underlayer holds solvent");
    let cases = [
        ("raw sienna thinned 0.5", raw_sienna().with_thinner(0.5)),
        ("lead white thinned 0.2", lead_white().with_thinner(0.2)),
        ("lead white thinned 0.9", lead_white().with_thinner(0.9)),
        ("raw sienna unthinned (today's path)", raw_sienna()),
    ];
    for (k, (name, p)) in cases.into_iter().enumerate() {
        let mut h = brush(Tool::filbert(20.0), 10 + k as u64, p, 0.6);
        let before = Totals::of(&c, &h);
        let y = 300.0 + 30.0 * k as f32;
        // across the thinned underlayer: the brush lays and picks up
        stroke(&mut c, &mut h, vec![(120.0, y - 60.0), (500.0, y), (880.0, y + 50.0)], 0.8);
        let after = Totals::of(&c, &h);
        assert_eq!(c.clock(), t0, "a stroke puts no time on the canvas's clock");
        assert!(after.brush_paint < before.brush_paint, "{name}: the brush laid paint");
        assert_balances(&format!("{name}: paint"), before.canvas_paint + before.brush_paint, after.canvas_paint + after.brush_paint);
        assert_balances(&format!("{name}: solvent"), before.canvas_solvent + before.brush_solvent, after.canvas_solvent + after.brush_solvent);
        if k == 3 {
            assert!(after.brush_solvent > 0.0, "{name}: the paint it picked up came with its solvent");
        }
    }
    for (name, damp) in [("dry rag", 0.0f32), ("rag dipped in spirits", 0.5)] {
        let mut r = Rag::new(60.0, 3);
        if damp > 0.0 {
            let now = c.now_min();
            r.dip(damp, now, c.tally_mut());
        }
        let (p0, s0) = (c.wet_total(), c.solvent_total());
        let lifted = c.rag_wipe(&mut r, &[(150.0, 350.0), (850.0, 350.0)], &[0.7], 5);
        let k = mm3_per_total(&c);
        assert_eq!(c.clock(), t0, "a wipe puts no time on the canvas's clock");
        assert!(lifted > 0.0 && r.solvent_mm3 > 0.0, "{name}: lifted {lifted} mm³ of paint and {} mm³ of solvent", r.solvent_mm3);
        assert_balances(&format!("{name}: paint"), p0, c.wet_total() + lifted / k);
        assert_balances(&format!("{name}: solvent"), s0, c.solvent_total() + r.solvent_mm3 / k);
    }
}

/// Check 5. The brush runs out: along one long stroke the laid paint
/// falls as the brush empties, and a brush loaded 0.6 lays paint farther
/// than one loaded 0.3 at the same thinner (the same stroke limit). "Lays
/// paint" = the mean paint across the stroke's middle is at least a
/// quarter of the limit's paint (θ); "farther" = at least 30 units (a
/// third of the filbert's run, so the margin isn't one 10-unit bin).
#[test]
fn c05_an_emptying_brush_lays_less_and_a_fuller_load_lasts_farther() {
    let t = 0.5;
    let lim_paint = stroke_limit_um(t) * (1.0 - t);
    assert!(lim_paint.is_finite() && lim_paint > 0.0, "thinner {t} has a finite stroke limit ({lim_paint} µm of paint)");
    let theta = 0.25 * lim_paint;
    let mut reach = Vec::new();
    for load in [0.3f32, 0.6] {
        let mut c = canvas(600);
        let mut h = brush(Tool::filbert(8.0), 21, raw_sienna().with_thinner(t), load);
        stroke(&mut c, &mut h, vec![(20.0, 500.0), (980.0, 500.0)], 0.7);
        let p = profile(&c, 500.0, 2.0, 20.0, 980.0, 10.0);
        let early = p[1..6].iter().copied().fold(0.0, f32::max);
        let late = mean(&p[p.len() - 10..]) as f32;
        assert!(early > theta, "load {load}: the stroke starts laying paint ({early} µm, θ {theta} µm)");
        assert!(late < 0.5 * early, "load {load}: an emptying brush lays less ({early} µm early, {late} µm over the last 100 units)");
        let last = p.iter().rposition(|&v| v >= theta).expect("some paint above θ");
        assert!(last + 1 < p.len(), "load {load}: the brush ran out before the stroke's end");
        reach.push(20.0 + 10.0 * (last + 1) as f32);
        println!("load {load}: paint above θ to x = {} (profile {p:?})", reach[reach.len() - 1]);
    }
    assert!(reach[1] >= reach[0] + 30.0, "load 0.6 lays paint farther than 0.3: to {} vs {}", reach[1], reach[0]);
}

/// Check 6. Pressure: the same thinned load lays more paint at more
/// pressure (at least 5% more per step, so a tie fails), and never more
/// than the stroke limit on any pixel (rounding: 1e-3 relative + 1e-3 µm).
#[test]
fn c06_more_pressure_lays_more_paint_up_to_the_stroke_limit() {
    let t = 0.5;
    let lim = stroke_limit_um(t);
    let mut laid = Vec::new();
    for pr in [0.3f32, 0.6, 0.9] {
        let mut c = canvas(400);
        let w0 = wet_film(&c);
        let mut h = brush(Tool::filbert(20.0), 31, raw_sienna().with_thinner(t), 0.6);
        let p0 = h.carried().0;
        stroke(&mut c, &mut h, vec![(150.0, 500.0), (450.0, 510.0)], pr);
        laid.push(p0 - h.carried().0);
        let most = wet_film(&c).iter().zip(&w0).map(|(b, a)| b - a).fold(0.0, f32::max);
        assert!(most <= lim * 1.001 + 1e-3, "pressure {pr}: a pixel took {most} µm of wet film; the stroke limit is {lim} µm");
    }
    assert!(laid[1] > 1.05 * laid[0] && laid[2] > 1.05 * laid[1], "paint laid at pressure 0.3, 0.6, 0.9: {laid:?}");
}

/// Check 8, a small thinner: thinner 0.01 changes a pass of strokes only a
/// little from no thinner (paint within 3% overall and per pixel on
/// average; solvent under 2% of the paint), and thinner 0 is no thinner.
/// 3%: a hundredth of the load is solvent, and the brush's dips and
/// pickups may triple that, not more.
#[test]
fn c08_a_hundredth_of_thinner_is_a_small_change() {
    let pass = |p: paint::Paint| -> Canvas {
        let mut c = canvas(240);
        for (k, y) in [320.0f32, 380.0, 440.0, 500.0, 560.0].into_iter().enumerate() {
            let mut h = brush(Tool::filbert(30.0), 101 + k as u64, p, 0.6);
            stroke(&mut c, &mut h, vec![(150.0, y), (850.0, y + 20.0)], 0.8);
        }
        c
    };
    let none = pass(raw_sienna());
    let zero = pass(raw_sienna().with_thinner(0.0));
    let tiny = pass(raw_sienna().with_thinner(0.01));
    assert!(state(&none) == state(&zero), "thinner 0 paints exactly as no thinner");
    assert_eq!(zero.solvent_total(), 0.0);
    let (a, b) = (paint_um(&none), paint_um(&tiny));
    let (ta, tb) = (none.wet_total(), tiny.wet_total());
    assert!((tb - ta).abs() <= 0.03 * ta, "total paint: {ta} without thinner, {tb} at 0.01");
    let diff = a.iter().zip(&b).map(|(x, y)| (x - y).abs() as f64).sum::<f64>() / a.len() as f64;
    assert!(diff <= 0.03 * mean(&a), "mean difference per pixel {diff} µm against a mean film of {} µm", mean(&a));
    assert!(tiny.solvent_total() > 0.0 && tiny.solvent_total() <= 0.02 * tb, "solvent {} against paint {tb}", tiny.solvent_total());
}

/// Check 9. Evaporation: the time constant is positive, finite and no
/// shorter for a thicker film; the solvent falls every minute of painting
/// time and is under a thousandth of what was laid after ten times the
/// slowest film's time constant; the paint stays (only solvent leaves the
/// wet film, so the film loses exactly the solvent's volume; rounding
/// `BALANCE_REL`). That the solvent adds no color is check 19.
#[test]
fn c09_the_solvent_evaporates_and_the_film_loses_its_volume() {
    let mut prev = 0.0f64;
    for um in [0.5f32, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0] {
        let tau = evaporation_tau_min(um);
        assert!(tau.is_finite() && tau > 0.0 && tau >= prev, "τ({um} µm) = {tau} min after {prev}");
        prev = tau;
    }
    let mut c = canvas(200);
    thinned_patch(&mut c, raw_sienna(), 0.5, (200.0, 800.0), (300.0, 700.0), 41);
    let (s0, p0) = (c.solvent_total(), c.wet_total());
    assert!(s0 > 0.0, "solvent laid");
    let tau = evaporation_tau_min(max_paint_um(&c));
    let minutes = (10.0 * tau).ceil() as usize + 1;
    let mut last = s0;
    for m in 1..=minutes {
        c.wait(1.0);
        let s = c.solvent_total();
        assert!(s < last || (s == 0.0 && last == 0.0), "minute {m}: the solvent went from {last} to {s}");
        assert_balances(&format!("minute {m}: the paint"), p0, c.wet_total());
        last = s;
    }
    assert!(last < 1e-3 * s0, "after {minutes} min (10 τ = {} min) {last} of {s0} solvent is left", 10.0 * tau);
}

/// Check 10, wet handling: a solvent-wet film levels more than the same
/// film once its solvent has gone. Over the same span (one time constant
/// of the median film), the paint surface's roughness falls by more than
/// 2% while solvent is present and by less than half as much after it has
/// gone (the film still open). A flat ground, so the paint's own ridges
/// are the only relief.
#[test]
fn c10_a_solvent_wet_film_spreads_more_than_after_the_solvent_has_gone() {
    let mut c = smooth_canvas(300);
    thinned_patch(&mut c, raw_sienna(), 0.5, (150.0, 850.0), (300.0, 700.0), 51);
    let r = (250.0, 380.0, 750.0, 620.0);
    let s0 = c.solvent_total();
    let film: Vec<f32> = in_rect(&c, r).iter().map(|&(_, x, y)| c.wet_um(x, y)).collect();
    let span = evaporation_tau_min(median(&film)).ceil().max(1.0) as f32;
    let r0 = std_dev(&paint_surface(&c, r));
    c.wait(span);
    let r1 = std_dev(&paint_surface(&c, r));
    c.wait((12.0 * evaporation_tau_min(max_paint_um(&c))).ceil() as f32);
    assert!(c.solvent_total() < 1e-3 * s0, "the solvent has gone");
    assert_eq!(c.drying_at(500.0, 500.0), Stage::Open, "the film is still open");
    let r2 = std_dev(&paint_surface(&c, r));
    c.wait(span);
    let r3 = std_dev(&paint_surface(&c, r));
    let (wet, gone) = (r0 - r1, (r2 - r3).max(0.0));
    println!("roughness {r0} -> {r1} µm with solvent, {r2} -> {r3} µm after ({span} min each)");
    assert!(wet > 0.02 * r0, "with solvent the film levels: roughness {r0} -> {r1} µm over {span} min");
    assert!(wet > 2.0 * gone, "it levels more with solvent ({wet} µm) than after ({gone} µm)");
}

/// Check 10, drying: thinned paint and unthinned paint of equal remaining
/// thickness gel and touch-dry at the same painting time. The unthinned
/// film is the thinned one with its solvent taken out through the save
/// (the same paint in the same places). Checked every 30 min; the median
/// times agree within 3% or one check (30 min), whichever is longer.
#[test]
fn c10_thinned_and_unthinned_paint_of_equal_thickness_gel_and_dry_together() {
    let mut c = canvas(160);
    thinned_patch(&mut c, raw_sienna(), 0.5, (200.0, 800.0), (300.0, 700.0), 61);
    let a = with_solvent_scaled(&c, 1.0);
    let b = with_solvent_scaled(&c, 0.0);
    assert!(a.solvent_total() > 0.0 && b.solvent_total() == 0.0);
    let pts: Vec<(f32, f32)> = in_rect(&a, (300.0, 400.0, 700.0, 600.0)).into_iter().filter(|&(_, x, y)| a.wet_um(x, y) >= 2.0).map(|(_, x, y)| (x, y)).collect();
    assert!(pts.len() >= 100, "{} pixels of film", pts.len());
    let step = 30.0f32;
    let times = |mut c: Canvas| -> (Vec<f32>, Vec<f32>) {
        let (mut gel, mut dry) = (vec![f32::NAN; pts.len()], vec![f32::NAN; pts.len()]);
        for k in 1..=(20 * 48) {
            c.wait(step);
            let t = k as f32 * step;
            let mut left = false;
            for (j, &(x, y)) in pts.iter().enumerate() {
                let st = c.drying_at(x, y);
                if gel[j].is_nan() && matches!(st, Stage::Tacky | Stage::Dry) {
                    gel[j] = t;
                }
                if dry[j].is_nan() && st == Stage::Dry {
                    dry[j] = t;
                }
                left |= dry[j].is_nan();
            }
            if !left {
                break;
            }
        }
        assert!(dry.iter().all(|v| !v.is_nan()), "every pixel touch-dry within 20 days");
        (gel, dry)
    };
    let (ga, da) = times(a);
    let (gb, db) = times(b);
    for (what, x, y) in [("gel", median(&ga), median(&gb)), ("touch-dry", median(&da), median(&db))] {
        let allowed = (0.03 * y).max(step);
        println!("{what}: thinned {x} min, unthinned {y} min");
        assert!((x - y).abs() <= allowed, "{what}: thinned {x} min, unthinned {y} min (allowed ±{allowed})");
    }
}

/// Check 13. Pigment values stay as they were at af49348 (masstone,
/// hiding, stiffness, tinting strength of both siennas), and burnt sienna
/// stays the more transparent (less scattering per coat), Field/Salter
/// 1869 §155.
#[test]
fn c13_pigment_values_are_unchanged_and_burnt_sienna_is_the_more_transparent() {
    let cat = paint::palette::catalog();
    let mut s = Vec::new();
    for (name, color, hiding, stiff, strength) in [("raw sienna", "#9a6a2b", 0.4f32, 0.5f32, 0.7f32), ("burnt sienna", "#7c3f24", 0.45, 0.55, 0.9)] {
        let t = cat.iter().find(|t| t.name == name).unwrap_or_else(|| panic!("{name} in the catalog"));
        assert_eq!(t.color.map(f32::to_bits), hex(color).map(f32::to_bits), "{name}: masstone");
        assert_eq!([t.hiding, t.stiff, t.strength].map(f32::to_bits), [hiding, stiff, strength].map(f32::to_bits), "{name}: hiding, stiffness, strength");
        s.push(scatter_for(luminance(t.color), t.hiding));
    }
    assert!(s[1] < s[0], "burnt sienna scatters less per coat than raw sienna: {} vs {}", s[1], s[0]);
}

/// Check 14. The same straight path given with 2× and 10× the points lays
/// the same wet film at every pixel within 2% of the stroke limit
/// (thinned) or of the thickest film (the unthinned control), and the same
/// total within 0.5%. One stroke shares one limit: a brush of 400
/// overlapping hairs, and one stroke that goes over its path four times,
/// add no more than the limit to any pixel (rounding 1e-3 relative +
/// 1e-3 µm).
#[test]
fn c14_more_points_on_the_same_path_lay_the_same_paint_and_one_stroke_shares_one_limit() {
    let t = 0.5;
    let lim = stroke_limit_um(t);
    for (name, p) in [("thinned 0.5", raw_sienna().with_thinner(t)), ("unthinned (control)", raw_sienna())] {
        let lay = |n: usize| -> (Vec<f32>, f64) {
            let mut c = canvas(400);
            let mut h = brush(Tool::filbert(20.0), 71, p, 0.8);
            let pts = (0..n).map(|i| {
                let u = i as f32 / (n - 1) as f32;
                (150.0 + 700.0 * u, 480.0 + 40.0 * u)
            });
            stroke(&mut c, &mut h, pts.collect(), 0.8);
            (wet_film(&c), c.wet_total() + c.solvent_total())
        };
        let (base, total) = lay(5);
        let top = base.iter().copied().fold(0.0, f32::max);
        let allowed = if lim.is_finite() && name.starts_with("thinned") { 0.02 * lim } else { 0.02 * top };
        for n in [10, 50] {
            let (v, tot) = lay(n);
            let worst = v.iter().zip(&base).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
            assert!(worst <= allowed, "{name}, {n} points vs 5: a pixel differs by {worst} µm (allowed {allowed})");
            assert!((tot - total).abs() <= 0.005 * total, "{name}, {n} points vs 5: total {tot} vs {total}");
        }
    }
    let dense = Tool { bristles: 400, hair: 2.5, ..Tool::filbert(20.0) };
    let mut c = canvas(400);
    let mut h = brush(dense, 72, raw_sienna().with_thinner(t), 1.0);
    stroke(&mut c, &mut h, vec![(150.0, 300.0), (850.0, 300.0)], 0.9);
    let most = wet_film(&c).into_iter().fold(0.0, f32::max);
    assert!(most <= lim * 1.001 + 1e-3, "400 overlapping hairs: a pixel took {most} µm (limit {lim})");
    let mut h = brush(Tool::filbert(20.0), 73, raw_sienna().with_thinner(t), 1.0);
    stroke(&mut c, &mut h, vec![(300.0, 700.0), (700.0, 700.0), (300.0, 700.0), (700.0, 700.0), (300.0, 700.0)], 0.9);
    let most = in_rect(&c, (250.0, 600.0, 750.0, 800.0)).iter().map(|&(_, x, y)| c.wet_um(x, y) + c.solvent_um(x, y)).fold(0.0, f32::max);
    assert!(most <= lim * 1.001 + 1e-3, "one stroke four times over its path: a pixel took {most} µm (limit {lim})");
}

/// Check 15. Oil drying takes the paint's own stiffness and solvent-free
/// thickness: over three time constants, while the solvent leaves, a film
/// holding solvent cures as the same film without it (median ratio of
/// cure within 2%; an input softened or thickened by the solvent would
/// slow it by 14% or more, see ACCEPTANCE.md).
#[test]
fn c15_solvent_does_not_slow_the_oil_cure() {
    let mut c = canvas(160);
    thinned_patch(&mut c, raw_sienna(), 0.5, (200.0, 800.0), (300.0, 700.0), 62);
    let mut a = with_solvent_scaled(&c, 1.0);
    let mut b = with_solvent_scaled(&c, 0.0);
    let m = (3.0 * evaporation_tau_min(max_paint_um(&a))).ceil().max(3.0) as f32;
    a.wait(m);
    b.wait(m);
    let (ca, cb) = (cure(&a), cure(&b));
    let ratios: Vec<f32> = paint_um(&b).iter().enumerate().filter(|&(i, &p)| p >= 2.0 && cb[i] > 0.0).map(|(i, _)| ca[i] / cb[i]).collect();
    assert!(ratios.len() >= 100, "{} pixels of film", ratios.len());
    let r = median(&ratios);
    assert!((r - 1.0).abs() <= 0.02, "cure with solvent / without, median over the film: {r}");
}

/// Check 16. Transport: a clean brush through a thinned film carries the
/// film's own ratio of solvent to paint; a rag wiping either of two films
/// of different ratios takes each film's ratio and leaves it at every
/// pixel; one minute of spreading moves solvent with the paint (a pixel's
/// ratio doesn't fall as it gains paint). Ratios within 1e-3 (brush,
/// rag's load) and 1e-4 per pixel (rag); spreading: see ACCEPTANCE.md.
#[test]
fn c16_brush_rag_and_spreading_carry_solvent_in_the_local_ratio() {
    let ratios = |c: &Canvas, r: (f32, f32, f32, f32)| -> Vec<(usize, f32)> {
        in_rect(c, r).iter().filter(|&&(_, x, y)| c.wet_um(x, y) >= 1.0).map(|&(i, x, y)| (i, c.solvent_um(x, y) / c.wet_um(x, y))).collect()
    };
    let mut c = canvas(300);
    let (left, right) = ((150.0, 350.0, 400.0, 650.0), (600.0, 350.0, 850.0, 650.0));
    thinned_patch(&mut c, raw_sienna(), 0.5, (100.0, 450.0), (300.0, 700.0), 81);
    thinned_patch(&mut c, lead_white(), 0.2, (550.0, 900.0), (300.0, 700.0), 82);
    for (r, want) in [(left, 1.0f32), (right, 0.25)] {
        let q = ratios(&c, r);
        assert!(q.len() >= 100);
        for &(i, v) in &q {
            assert!((v - want).abs() <= 1e-3 * want, "fresh film at pixel {i}: ratio {v}, want {want}");
        }
    }
    // a clean brush through the left film only
    let mut h = paint::Held::new(Tool::hog_flat(30.0), 83);
    stroke(&mut c, &mut h, vec![(180.0, 500.0), (380.0, 520.0)], 0.9);
    let (bp, bs) = h.carried();
    assert!(bp > 0.0, "the clean brush picked up paint");
    assert!((bs / bp - 1.0).abs() <= 1e-3, "the brush carries the film's ratio: {}", bs / bp);
    // a rag over each film
    for (r, want, y) in [(left, 1.0f64, 450.0f32), (right, 0.25, 550.0)] {
        let before: std::collections::HashMap<usize, f32> = ratios(&c, r).into_iter().collect();
        let mut g = Rag::new(50.0, 84);
        let lifted = c.rag_wipe(&mut g, &[(r.0 + 20.0, y), (r.2 - 20.0, y)], &[0.7], 9);
        assert!(lifted > 0.0);
        assert!((g.solvent_mm3 / lifted - want).abs() <= 1e-3 * want, "the rag took the film's ratio: {}", g.solvent_mm3 / lifted);
        let mut n = 0;
        for (i, q) in ratios(&c, r) {
            if let Some(&q0) = before.get(&i) {
                assert!((q - q0).abs() <= 1e-4 * q0, "pixel {i}: ratio {q0} before the wipe, {q} after");
                n += 1;
            }
        }
        assert!(n >= 100);
    }
    // one minute of spreading over a fresh film of one ratio
    let mut c = canvas(300);
    thinned_patch(&mut c, raw_sienna(), 0.5, (150.0, 850.0), (300.0, 700.0), 85);
    let (h0, s0) = (paint_um(&c), solvent_um(&c));
    c.wait(1.0);
    let (h1, s1) = (paint_um(&c), solvent_um(&c));
    let (mut gain, mut resid) = (Vec::new(), Vec::new());
    for i in 0..h0.len() {
        if h0[i] < 1.0 || h1[i] < 1.0 || s1[i] <= 0.0 {
            continue;
        }
        let dq = ((s1[i] / h1[i]) as f64).ln() - ((s0[i] / h0[i]) as f64).ln();
        // what evaporation alone does to the ratio, at the thickness before or after
        let e = [h0[i], h1[i]].map(|h| dq + 1.0 / evaporation_tau_min(h));
        let r = if e[0].abs() <= e[1].abs() { e[0] } else { e[1] };
        gain.push(((h1[i] / h0[i]) as f64).ln());
        resid.push(r);
    }
    let moved = gain.iter().filter(|g| g.abs() > 0.01).count();
    assert!(moved >= 50, "the film spread in one minute: {moved} pixels changed thickness by more than 1%");
    let k = slope(&gain, &resid);
    assert!(k > -0.5 && k < 0.5, "the ratio's change, beyond evaporation, against the paint gained: slope {k} (solvent left behind gives about -1)");
}

/// Check 17. Waits on the one-minute clock: from a whole minute, one
/// wait of 15 minutes and fifteen of one leave exactly the same canvas
/// (the whole save, byte for byte), solvent present throughout; 7.3 then
/// 7.7 minutes agree with 15 within 1e-4 relative in paint, solvent and
/// cure at every pixel. Zero rule: values both no further from zero than
/// `ZERO_UM` (paint, solvent) or `ZERO_CURE` (cure) agree.
#[test]
fn c17_a_wait_split_on_the_minute_grid_is_exact_and_off_it_within_1e4() {
    let mut base = canvas(160);
    thinned_patch(&mut base, raw_sienna(), 0.5, (200.0, 800.0), (300.0, 700.0), 91);
    thinned_patch(&mut base, lead_white(), 0.3, (200.0, 800.0), (500.0, 900.0), 92);
    assert_eq!(base.clock().fract(), 0.0, "the waits start on a whole minute");
    let (mut a, mut b, mut d) = (with_solvent_scaled(&base, 1.0), with_solvent_scaled(&base, 1.0), with_solvent_scaled(&base, 1.0));
    a.wait(15.0);
    for _ in 0..15 {
        b.wait(1.0);
    }
    d.wait(7.3);
    d.wait(7.7);
    assert!(a.solvent_total() > 0.0, "solvent is present throughout the 15 minutes");
    assert_eq!(a.clock(), b.clock());
    assert_eq!(a.clock(), d.clock());
    let (sa, sb) = (state(&a), state(&b));
    if sa != sb {
        let first = sa.iter().zip(&sb).position(|(x, y)| x != y);
        panic!("wait(15) and 15 × wait(1) differ (sizes {} and {}, first differing byte {first:?}); paint equal: {}, solvent equal: {}, cure equal: {}", sa.len(), sb.len(), paint_um(&a) == paint_um(&b), solvent_um(&a) == solvent_um(&b), cure(&a) == cure(&b));
    }
    for (what, x, y, floor) in [("paint µm", paint_um(&a), paint_um(&d), ZERO_UM), ("solvent µm", solvent_um(&a), solvent_um(&d), ZERO_UM), ("cure", cure(&a), cure(&d), ZERO_CURE)] {
        let bad: Vec<(usize, f32, f32)> = x.iter().zip(&y).enumerate().filter(|&(_, (p, q))| !close(*p as f64, *q as f64, 1e-4, floor)).map(|(i, (p, q))| (i, *p, *q)).collect();
        assert!(bad.is_empty(), "{what}: 7.3 + 7.7 min differs from 15 min at {} pixels, e.g. {:?}", bad.len(), &bad[..bad.len().min(5)]);
    }
}

/// Check 19. Solvent has no color: the same paint in the same places with
/// its solvent, without it and with half of it renders to the same bits.
#[test]
fn c19_the_same_paint_with_more_or_less_solvent_looks_the_same() {
    let mut c = canvas(200);
    thinned_patch(&mut c, raw_sienna(), 0.5, (150.0, 650.0), (200.0, 600.0), 95);
    thinned_patch(&mut c, lead_white(), 0.3, (400.0, 900.0), (400.0, 800.0), 96);
    let a = with_solvent_scaled(&c, 1.0);
    let none = with_solvent_scaled(&c, 0.0);
    let half = with_solvent_scaled(&c, 0.5);
    assert!(a.solvent_total() > 0.0 && none.solvent_total() == 0.0 && half.solvent_total() < a.solvent_total());
    let bits = |c: &Canvas| -> Vec<u32> { c.seen().iter().flat_map(|p| p.map(f32::to_bits)).collect() };
    assert!(bits(&a) == bits(&none), "the picture changed when the solvent was taken out");
    assert!(bits(&a) == bits(&half), "the picture changed when half the solvent was taken out");
}
