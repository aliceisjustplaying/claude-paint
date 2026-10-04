//! Thinner acceptance checks at the paint engine's own boundary (brush,
//! rag, canvas, clock, save): checks 4, 5, 6, 8 (a small thinner), 9, 10,
//! 14, 15, 16, 17 and 19 of notes/thinner/ACCEPTANCE.md. The gel and
//! touch-dry comparison of check 10 is `#[ignore = "slow"]` (run by exact
//! name: scripts/test_thinner_acceptance --all). The measuring
//! helpers, and the interface these tests require, are in
//! `thinner_support/mod.rs`.
//!
//!   cargo test -p paint --release --test thinner_physics

mod thinner_support;

use paint::rag::Rag;
use paint::thinner::{evaporation_tau_min, stroke_limit_um};
use paint::{Canvas, Stage, Tool};
use thinner_support::*;

/// Check 4. Nothing vanishes: across a stroke (no time passes during one,
/// so nothing evaporates), the paint on the canvas and on the brush adds
/// up to what it was, and so does the solvent, including the paint and
/// solvent the brush picks up from a thinned underlayer (an unthinned
/// brush comes away carrying solvent). Across a rag wipe what leaves the
/// canvas is what the rag took. Rounding: `BALANCE_REL`; its control is
/// `c04_control_a_wholly_unthinned_scene_balances`.
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
        ("raw sienna unthinned, over thinned paint", raw_sienna()),
    ];
    for (k, (name, p)) in cases.into_iter().enumerate() {
        let mut h = brush(Tool::filbert(20.0), 10 + k as u64, p, 0.6);
        let before = Totals::of(&c, &h);
        let film0 = paint_um(&c);
        let y = 300.0 + 30.0 * k as f32;
        // across the thinned underlayer: the brush lays and picks up
        stroke(&mut c, &mut h, vec![(120.0, y - 60.0), (500.0, y), (880.0, y + 50.0)], 0.8);
        let after = Totals::of(&c, &h);
        assert_eq!(c.clock(), t0, "a stroke puts no time on the canvas's clock");
        // the stroke moved paint (laid it, picked it up or both): the canvas's
        // paint changed by more than a thousandth of all of it, pixel by
        // pixel, either way. (A thin, solvent-heavy load can pick up more
        // than its ceiling lets it lay, so the brush may end fuller.)
        let moved: f64 = paint_um(&c).iter().zip(&film0).map(|(a, b)| (a - b).abs() as f64).sum();
        let all: f64 = film0.iter().map(|&v| v as f64).sum();
        assert!(moved > 1e-3 * all, "{name}: the stroke moved paint ({moved} of {all} µm·pixels)");
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

/// Check 4's control: a scene with no thinner anywhere (today's code path:
/// `with_thinner` is never called), the same kinds of stroke over wet
/// paint and the same wipe, balances paint within the same `BALANCE_REL`,
/// and holds no solvent at all.
#[test]
fn c04_control_a_wholly_unthinned_scene_balances() {
    let mut c = canvas(240);
    patch(&mut c, raw_sienna(), (150.0, 850.0), (280.0, 420.0), 1);
    for (k, p) in [raw_sienna(), lead_white(), lead_white(), raw_sienna()].into_iter().enumerate() {
        let mut h = brush(Tool::filbert(20.0), 10 + k as u64, p, 0.6);
        let before = Totals::of(&c, &h);
        let y = 300.0 + 30.0 * k as f32;
        stroke(&mut c, &mut h, vec![(120.0, y - 60.0), (500.0, y), (880.0, y + 50.0)], 0.8);
        let after = Totals::of(&c, &h);
        assert!(after.brush_paint < before.brush_paint, "stroke {k}: the brush laid paint");
        assert_balances(&format!("unthinned stroke {k}: paint"), before.canvas_paint + before.brush_paint, after.canvas_paint + after.brush_paint);
        assert_eq!((after.canvas_solvent, after.brush_solvent), (0.0, 0.0), "stroke {k}: no solvent anywhere");
    }
    let mut r = Rag::new(60.0, 3);
    let p0 = c.wet_total();
    let lifted = c.rag_wipe(&mut r, &[(150.0, 350.0), (850.0, 350.0)], &[0.7], 5);
    assert!(lifted > 0.0);
    assert_balances("unthinned wipe: paint", p0, c.wet_total() + lifted / mm3_per_total(&c));
    assert_eq!((c.solvent_total(), r.solvent_mm3), (0.0, 0.0), "no solvent anywhere");
}

/// Check 5. The brush runs out: along one long stroke the laid paint
/// falls as the brush empties, and a brush loaded 0.6 lays paint farther
/// than one loaded 0.3 at the same thinner (the same stroke limit). "Lays
/// paint" = the mean paint across the stroke's middle is at least a
/// quarter of the limit's paint (θ); "farther" = at least 30 units (a
/// third of the filbert's run, so the margin isn't one 10-unit bin). The
/// stroke is one continuous zigzag over fresh ground, eight 960-unit rows
/// 80 units apart (7680 units, no reload): long enough for a thinned
/// brush, which keeps what it can't lay, to empty. Its pressure ramps up
/// and lifts over as many units as the one 960-unit stroke's did (attack
/// 0.08, release 0.15 of 960 units). Distances are along the rows, in
/// order.
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
        let rows: Vec<f32> = (0..8).map(|k| 200.0 + 80.0 * k as f32).collect();
        let path: Vec<(f32, f32)> = rows.iter().enumerate().flat_map(|(k, &y)| if k % 2 == 0 { [(20.0, y), (980.0, y)] } else { [(980.0, y), (20.0, y)] }).collect();
        let len = 960.0 * rows.len() as f32 + 80.0 * (rows.len() - 1) as f32;
        c.drag(&mut h, &paint::Gesture::new(path).pressure(0.7, 0.7).ramps(0.08 * 960.0 / len, 0.15 * 960.0 / len), None);
        let p: Vec<f32> = rows
            .iter()
            .enumerate()
            .flat_map(|(k, &y)| {
                let mut r = profile(&c, y, 2.0, 20.0, 980.0, 10.0);
                if k % 2 == 1 {
                    r.reverse();
                }
                r
            })
            .collect();
        let early = p[1..6].iter().copied().fold(0.0, f32::max);
        let late = mean(&p[p.len() - 10..]) as f32;
        assert!(early > theta, "load {load}: the stroke starts laying paint ({early} µm, θ {theta} µm)");
        assert!(late < 0.5 * early, "load {load}: an emptying brush lays less ({early} µm early, {late} µm over the last 100 units)");
        let last = p.iter().rposition(|&v| v >= theta).expect("some paint above θ");
        assert!(last + 1 < p.len(), "load {load}: the brush ran out before the stroke's end");
        reach.push(10.0 * (last + 1) as f32);
        println!("load {load}: paint above θ for {} units of the rows (profile {p:?})", reach[reach.len() - 1]);
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

/// Check 9. Evaporation follows `solvent = s0 × exp(-t / τ(h))` at a fixed
/// paint thickness h, τ from `evaporation_tau_min` (the model's own,
/// labeled estimate; the equation is the plan's and is computed here).
/// Two films on a flat ground, one thinned pass and five passes (thicker).
/// Every minute, at every pixel whose paint changed by at most 1e-3 of
/// itself (spreading barely touched it) and still holds ≥ 0.01 µm of
/// solvent: the share of its solvent left equals exp(-1 min / τ(h)) within
/// 1e-4 + 2 × the paint's relative change (a pixel whose paint moved by δ
/// can have its solvent moved by about that much again). At least 200 such
/// pixel-minutes in each film; the thick film loses a smaller share per
/// minute. Immediate loss, linear loss or a wrong rate fail. Also: τ is
/// positive, finite and never shorter for a thicker film; total solvent
/// falls every minute; paint stays (`BALANCE_REL`: only solvent leaves the
/// wet film); after ten times the slowest τ, under a thousandth is left.
/// That the solvent adds no color is check 19.
#[test]
fn c09_the_solvent_evaporates_and_the_film_loses_its_volume() {
    let mut prev_tau = 0.0f64;
    for um in [0.5f32, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0] {
        let tau = evaporation_tau_min(um);
        assert!(tau.is_finite() && tau > 0.0 && tau >= prev_tau, "τ({um} µm) = {tau} min after {prev_tau}");
        prev_tau = tau;
    }
    let mut c = smooth_canvas(300);
    thinned_patch(&mut c, raw_sienna(), 0.5, (100.0, 450.0), (300.0, 700.0), 41);
    for k in 0..5 {
        thinned_patch(&mut c, raw_sienna(), 0.5, (550.0, 900.0), (300.0, 700.0), 141 + 20 * k);
    }
    let groups = [in_rect(&c, (150.0, 350.0, 400.0, 650.0)), in_rect(&c, (600.0, 350.0, 850.0, 650.0))];
    let h0 = paint_um(&c);
    let med: Vec<f32> = groups.iter().map(|g| median(&g.iter().map(|&(i, _, _)| h0[i]).collect::<Vec<_>>())).collect();
    assert!(med[1] > 1.5 * med[0], "the thick film is thicker: median {} vs {} µm", med[1], med[0]);
    let (s0, p0) = (c.solvent_total(), c.wet_total());
    assert!(s0 > 0.0, "solvent laid");
    let tau_max = evaporation_tau_min(max_paint_um(&c));
    let minutes = (10.0f64 * tau_max).ceil() as usize + 1;
    let (mut ph, mut ps) = (h0, solvent_um(&c));
    let mut last = s0;
    let mut loss: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    for m in 1..=minutes {
        c.wait(1.0);
        let (h1, s1) = (paint_um(&c), solvent_um(&c));
        for (gi, g) in groups.iter().enumerate() {
            for &(i, _, _) in g {
                let (a, b) = (ps[i] as f64, s1[i] as f64);
                let d = ((h1[i] - ph[i]) / ph[i].max(1e-6)).abs() as f64;
                if ph[i] < 1.0 || d > 1e-3 || a < 0.01 {
                    continue;
                }
                let want = (-1.0f64 / evaporation_tau_min(ph[i])).exp();
                let got = b / a;
                assert!((got - want).abs() <= 1e-4 + 2.0 * d, "minute {m}, pixel {i} ({} µm of paint): {got} of its solvent left, the law gives {want}", ph[i]);
                loss[gi].push(1.0 - got);
            }
        }
        let s = c.solvent_total();
        assert!(s < last || (s == 0.0 && last == 0.0), "minute {m}: the solvent went from {last} to {s}");
        assert_balances(&format!("minute {m}: the paint"), p0, c.wet_total());
        last = s;
        (ph, ps) = (h1, s1);
    }
    assert!(loss[0].len() >= 200 && loss[1].len() >= 200, "pixel-minutes at a fixed thickness: thin film {}, thick film {}", loss[0].len(), loss[1].len());
    let (thin, thick) = (loss[0].iter().sum::<f64>() / loss[0].len() as f64, loss[1].iter().sum::<f64>() / loss[1].len() as f64);
    assert!(thick < thin, "the thicker film loses a smaller share a minute: {thick} vs {thin}");
    assert!(last < 1e-3 * s0, "after {minutes} min (10 τ = {} min) {last} of {s0} solvent is left", 10.0 * tau_max);
}

/// Check 10, wet handling: solvent makes a film level more. Two copies of
/// one thinned patch on a flat ground, the same paint in the same places
/// with the same cure, one with its solvent and one without (taken out
/// through the save), wait the same span (one τ of the median film).
/// Expected: the copy with solvent levels: the paint surface's roughness
/// falls by more than 2% (a labeled model estimate); by more than twice
/// what the copy without solvent loses; paint balances in both.
#[test]
fn c10_a_solvent_wet_film_spreads_more_than_after_the_solvent_has_gone() {
    let mut c = smooth_canvas(300);
    thinned_patch(&mut c, raw_sienna(), 0.5, (150.0, 850.0), (300.0, 700.0), 51);
    let r = (250.0, 380.0, 750.0, 620.0);
    let film: Vec<f32> = in_rect(&c, r).iter().map(|&(_, x, y)| c.wet_um(x, y)).collect();
    let span = evaporation_tau_min(median(&film)).ceil().max(1.0) as f32;
    let mut wet = with_solvent_scaled(&c, 1.0);
    let mut dry = with_solvent_scaled(&c, 0.0);
    assert!(wet.solvent_total() > 0.0 && dry.solvent_total() == 0.0);
    let r0 = std_dev(&paint_surface(&wet, r));
    assert_eq!(r0, std_dev(&paint_surface(&dry, r)), "the copies start alike");
    let (pw, pd) = (wet.wet_total(), dry.wet_total());
    wet.wait(span);
    dry.wait(span);
    assert_balances("with solvent: the paint", pw, wet.wet_total());
    assert_balances("without solvent: the paint", pd, dry.wet_total());
    let (a, b) = (r0 - std_dev(&paint_surface(&wet, r)), (r0 - std_dev(&paint_surface(&dry, r))).max(0.0));
    println!("roughness {r0} µm: falls {a} with solvent, {b} without, over {span} min");
    assert!(a > 0.02 * r0, "with solvent the film levels: roughness falls {a} of {r0} µm over {span} min");
    assert!(a > 2.0 * b, "it levels more with solvent ({a} µm) than without ({b} µm)");
}

/// Check 10, drying: thinned paint and unthinned paint of equal remaining
/// thickness gel and touch-dry at the same painting time. The unthinned
/// film is the thinned one with its solvent taken out through the save
/// (the same paint in the same places). Checked every 30 min; the median
/// times agree within 3% or one check (30 min), whichever is longer.
#[test]
#[ignore = "slow"]
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

/// Check 14. The same straight path given with 2× and 10× the points lays
/// the same paint, and the same solvent, at every pixel within 2% of the
/// stroke limit (thinned) or of the thickest film (the unthinned control),
/// and the same totals of each within 0.5%; the stroke laid paint (and,
/// thinned, solvent). One stroke shares one limit: a brush of 400
/// overlapping hairs, and one stroke that goes over its path four times,
/// add no more than the limit to any pixel (rounding 1e-3 relative +
/// 1e-3 µm).
#[test]
fn c14_more_points_on_the_same_path_lay_the_same_paint_and_one_stroke_shares_one_limit() {
    let t = 0.5;
    let lim = stroke_limit_um(t);
    for (name, p, thinned) in [("thinned 0.5", raw_sienna().with_thinner(t), true), ("unthinned (control)", raw_sienna(), false)] {
        let lay = |n: usize| -> (Vec<f32>, Vec<f32>, f64, f64) {
            let mut c = canvas(400);
            let mut h = brush(Tool::filbert(20.0), 71, p, 0.8);
            let pts = (0..n).map(|i| {
                let u = i as f32 / (n - 1) as f32;
                (150.0 + 700.0 * u, 480.0 + 40.0 * u)
            });
            stroke(&mut c, &mut h, pts.collect(), 0.8);
            (paint_um(&c), solvent_um(&c), c.wet_total(), c.solvent_total())
        };
        let (bp, bs, tp, ts) = lay(5);
        assert!(tp > 0.0, "{name}: the stroke laid paint");
        assert_eq!(ts > 0.0, thinned, "{name}: solvent laid only when thinned ({ts})");
        let top = bp.iter().zip(&bs).map(|(a, b)| a + b).fold(0.0, f32::max);
        let allowed = if thinned { 0.02 * lim } else { 0.02 * top };
        for n in [10, 50] {
            let (vp, vs, np, ns) = lay(n);
            for (what, v, base) in [("paint", &vp, &bp), ("solvent", &vs, &bs)] {
                let worst = v.iter().zip(base.iter()).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
                assert!(worst <= allowed, "{name}, {n} points vs 5: {what} differs by {worst} µm at a pixel (allowed {allowed})");
            }
            let worst = vp.iter().zip(&vs).zip(bp.iter().zip(&bs)).map(|((a, b), (c, d))| ((a + b) - (c + d)).abs()).fold(0.0, f32::max);
            assert!(worst <= allowed, "{name}, {n} points vs 5: the wet film differs by {worst} µm at a pixel (allowed {allowed})");
            assert!((np - tp).abs() <= 0.005 * tp, "{name}, {n} points vs 5: paint {np} vs {tp}");
            assert!((ns - ts).abs() <= 0.005 * ts.max(f64::MIN_POSITIVE) || (ns == 0.0 && ts == 0.0), "{name}, {n} points vs 5: solvent {ns} vs {ts}");
        }
    }
    let dense = Tool { bristles: 400, hair: 2.5, ..Tool::filbert(20.0) };
    let mut c = canvas(400);
    let mut h = brush(dense, 72, raw_sienna().with_thinner(t), 1.0);
    stroke(&mut c, &mut h, vec![(150.0, 300.0), (850.0, 300.0)], 0.9);
    assert!(c.wet_total() > 0.0);
    let most = wet_film(&c).into_iter().fold(0.0, f32::max);
    assert!(most <= lim * 1.001 + 1e-3, "400 overlapping hairs: a pixel took {most} µm (limit {lim})");
    let mut h = brush(Tool::filbert(20.0), 73, raw_sienna().with_thinner(t), 1.0);
    stroke(&mut c, &mut h, vec![(300.0, 700.0), (700.0, 700.0), (300.0, 700.0), (700.0, 700.0), (300.0, 700.0)], 0.9);
    let most = in_rect(&c, (250.0, 600.0, 750.0, 800.0)).iter().map(|&(_, x, y)| c.wet_um(x, y) + c.solvent_um(x, y)).fold(0.0, f32::max);
    assert!(most > 0.0 && most <= lim * 1.001 + 1e-3, "one stroke four times over its path: a pixel took {most} µm (limit {lim})");
}

/// Check 15. Oil drying takes the paint's own stiffness and solvent-free
/// thickness. A film of five thinned passes, measured where it holds at
/// least 12 µm of paint and solvent: there `drying::rate`'s thickness
/// factor isn't clamped (it is below 0.37 coats, 9.3 µm, drying.rs:251),
/// so a thickness with the solvent counted in would dry ≥ 14% slower (see
/// ACCEPTANCE.md). The same film with and without its solvent (through the
/// save) waits one τ, while the solvent is mostly still there. Expected:
/// the 10th, 50th and 90th percentiles of cure with solvent ÷ without are
/// all within 2% of 1.
#[test]
fn c15_solvent_does_not_slow_the_oil_cure() {
    let mut c = canvas(160);
    for k in 0..5 {
        thinned_patch(&mut c, raw_sienna(), 0.5, (200.0, 800.0), (300.0, 700.0), 62 + 20 * k);
    }
    let mut a = with_solvent_scaled(&c, 1.0);
    let mut b = with_solvent_scaled(&c, 0.0);
    let (h, s) = (paint_um(&a), solvent_um(&a));
    let film: Vec<usize> = (0..h.len()).filter(|&i| h[i] >= 12.0 && s[i] > 0.0).collect();
    assert!(film.len() >= 100, "{} pixels with ≥ 12 µm of paint and some solvent", film.len());
    let m = evaporation_tau_min(median(&film.iter().map(|&i| h[i]).collect::<Vec<_>>())).ceil().max(1.0) as f32;
    a.wait(m);
    b.wait(m);
    let (ca, cb) = (cure(&a), cure(&b));
    assert!(film.iter().all(|&i| cb[i] > 0.0), "the film cured over {m} min");
    let ratios: Vec<f32> = film.iter().map(|&i| ca[i] / cb[i]).collect();
    for q in [0.1f32, 0.5, 0.9] {
        let r = quantile(&ratios, q);
        assert!((r - 1.0).abs() <= 0.02, "cure with solvent ÷ without, {}th percentile over the film: {r}", (q * 100.0) as u32);
    }
}

/// Check 16. Transport. (1) A clean brush through a thinned film carries
/// the film's own ratio of solvent to paint (within 1e-3). (2) A rag wiping
/// either of two films of different ratios takes each film's ratio (1e-3)
/// and leaves every pixel's ratio as it was (1e-4). (3) Spreading, one
/// minute over a thick film of ratio 1 (thinner 0.5, four passes) beside a
/// thin film of ratio 1/9 (thinner 0.1, one light pass), on a flat ground:
/// - the solvent balances: the total after equals what the law
///   (exp(-1 min / τ(h)) per pixel) leaves of the solvent before, at the
///   thickness before (evaporation first) or after (spreading first),
///   within 1e-4;
/// - every pixel's ratio after lies within the ratios its neighborhood
///   (4 px) had before, times the evaporation factors that neighborhood's
///   thicknesses give (1e-4): spreading mixes liquids, it doesn't separate
///   paint from solvent;
/// - the low-ratio film's pixels at the boundary (chosen before the
///   minute: ratio ≤ 0.2, ≥ 0.5 µm of paint, a pixel of ratio ≥ 0.5 within
///   2 px) that gained ≥ 1% paint (at least 20) end with a higher ratio,
///   after evaporation is taken back, than they had: they took in the
///   thick film's liquid, solvent and all.
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
    // (1) a clean brush through the left film only
    let mut h = paint::Held::new(Tool::hog_flat(30.0), 83);
    stroke(&mut c, &mut h, vec![(180.0, 500.0), (380.0, 520.0)], 0.9);
    let (bp, bs) = h.carried();
    assert!(bp > 0.0, "the clean brush picked up paint");
    assert!((bs / bp - 1.0).abs() <= 1e-3, "the brush carries the film's ratio: {}", bs / bp);
    // (2) a rag over each film
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
    // (3) one minute of spreading across two ratios. The ratio-1/9 film is
    // one light pass (thinner 0.1, load 0.05, pressure 0.3) from x 400 to
    // 850; then the ratio-1 film, eight thinned-0.5 passes (each stroke's
    // ceiling keeps each pass thin) from 150 to 500, over the low film's
    // start, so the ratio-1 film ends in an edge with untouched low-ratio
    // paint beside it. The setup's precondition, asserted: the ratio-1 film
    // is the thicker (its mean wet film left of the edge at least 1.5× the
    // other's right of it), so its liquid runs into the other
    let mut c = smooth_canvas(300);
    patch_with(&mut c, raw_sienna().with_thinner(0.1), 0.05, 0.3, (400.0, 850.0), (300.0, 700.0), 300);
    for k in 0..8 {
        thinned_patch(&mut c, raw_sienna(), 0.5, (150.0, 500.0), (300.0, 700.0), 200 + 20 * k);
    }
    let wet_mean = |c: &Canvas, r: (f32, f32, f32, f32)| -> f64 { mean(&in_rect(c, r).iter().map(|&(_, x, y)| c.wet_um(x, y) + c.solvent_um(x, y)).collect::<Vec<_>>()) };
    let (thick_mean, thin_mean) = (wet_mean(&c, (400.0, 350.0, 500.0, 650.0)), wet_mean(&c, (540.0, 350.0, 640.0, 650.0)));
    println!("spreading setup: ratio-1 film {thick_mean} µm, ratio-1/9 film {thin_mean} µm");
    assert!(thick_mean >= 1.5 * thin_mean, "setup: the ratio-1 film ({thick_mean} µm wet) is at least 1.5× the ratio-1/9 film ({thin_mean} µm)");
    let (h0, s0) = (paint_um(&c), solvent_um(&c));
    let (tot0, ptot0) = (c.solvent_total(), c.wet_total());
    c.wait(1.0);
    let (h1, s1) = (paint_um(&c), solvent_um(&c));
    let tot1 = c.solvent_total();
    assert_balances("spreading: the paint", ptot0, c.wet_total());
    let e = |h: f32| -> f64 { (-1.0f64 / evaporation_tau_min(h)).exp() };
    let (mut evap_first, mut back) = (0.0f64, 0.0f64);
    for i in 0..h0.len() {
        if s0[i] > 0.0 {
            evap_first += s0[i] as f64 * e(h0[i]);
        }
        if s1[i] > 0.0 {
            back += s1[i] as f64 / e(h1[i]);
        }
    }
    let sum1: f64 = s1.iter().map(|&v| v as f64).sum();
    let sum0: f64 = s0.iter().map(|&v| v as f64).sum();
    assert!(tot1 < tot0 && sum1 > 0.0);
    let (a, b) = ((evap_first / sum1 - 1.0).abs(), (back / sum0 - 1.0).abs());
    // 5e-4, not 1e-4 (2026-10-04, owner's decision; ACCEPTANCE.md "Decisions"):
    // the solvent's loss and flow now step on a 1/64-minute grid, so neither
    // one-minute prediction is exact. May be revisited.
    assert!(a.min(b) <= 5e-4, "the solvent left ({sum1}) is what the law leaves of what was there ({sum0}): evaporating first predicts {evap_first}, spreading first needs {back} before");
    let q = |h: &[f32], s: &[f32], i: usize| -> Option<f64> { if h[i] >= 0.1 && s[i] > 0.0 { Some(s[i] as f64 / h[i] as f64) } else { None } };
    let mut checked = 0;
    for i in 0..h1.len() {
        let Some(qi) = q(&h1, &s1, i) else { continue };
        if h1[i] < 1.0 {
            continue;
        }
        let nb = around(&c, i, 4);
        let qs: Vec<f64> = nb.iter().filter_map(|&j| q(&h0, &s0, j)).collect();
        if qs.is_empty() {
            continue;
        }
        let es: Vec<f64> = nb.iter().flat_map(|&j| [h0[j], h1[j]]).filter(|&h| h >= 0.1).map(e).collect();
        let lo = qs.iter().copied().fold(f64::MAX, f64::min) * es.iter().copied().fold(f64::MAX, f64::min);
        let hi = qs.iter().copied().fold(0.0, f64::max) * es.iter().copied().fold(0.0, f64::max);
        assert!(qi >= lo * (1.0 - 1e-4) && qi <= hi * (1.0 + 1e-4), "pixel {i}: ratio {qi} after one minute, outside its neighborhood's {lo}..{hi}");
        checked += 1;
    }
    assert!(checked >= 1000, "{checked} pixels checked");
    // the low-ratio pixels at the boundary, chosen from the state before
    // the minute: ratio ≤ 0.2 with ≥ 0.5 µm of paint, and a pixel of ratio
    // ≥ 0.5 within 2 px; of those, the ones that gained ≥ 1% paint
    let ratio0 = |j: usize| -> Option<f32> { if h0[j] >= 0.1 && s0[j] > 0.0 { Some(s0[j] / h0[j]) } else { None } };
    let mut rise = Vec::new();
    for i in 0..h1.len() {
        if h0[i] < 0.5 || !ratio0(i).is_some_and(|q| q <= 0.2) {
            continue;
        }
        if !around(&c, i, 2).into_iter().any(|j| ratio0(j).is_some_and(|q| q >= 0.5)) {
            continue;
        }
        if h1[i] < 1.01 * h0[i] || s1[i] <= 0.0 {
            continue;
        }
        rise.push((s1[i] as f64 / h1[i] as f64 / e(h1[i]), s0[i] as f64 / h0[i] as f64));
    }
    assert!(rise.len() >= 20, "the thick film spread into the thin one: {} low-ratio pixels at the boundary gained ≥ 1% paint", rise.len());
    let (after, before) = (rise.iter().map(|r| r.0).sum::<f64>() / rise.len() as f64, rise.iter().map(|r| r.1).sum::<f64>() / rise.len() as f64);
    assert!(after > 1.01 * before, "the thin film's ratio where the thick film's liquid came in: {before} before, {after} after (evaporation taken back)");
}

/// Check 17. Waits on the one-minute clock, which counts from the
/// canvas's start. Expected:
/// - from a whole minute, one wait of 15 minutes and fifteen of one leave
///   exactly the same canvas (the whole save, byte for byte), solvent
///   present throughout;
/// - with a stroke between waits, on whole minutes (wait 3, stroke, wait
///   12 against 3 × 1, stroke, 12 × 1): exactly the same save;
/// - from a fractional minute, the grid still counts from the canvas's
///   start, not from each wait: `wait(0.25); wait(0.75); wait(14)` and
///   `wait(0.25); wait(14.75)` leave exactly the same save (both: a quarter
///   minute, the rest of minute 1, then 14 whole minutes). The same when
///   hand time, not a wait, put the clock on the quarter minute (15 s of
///   hand time clocked by `clock_hand_min`, the brushwork path). A clock
///   that restarted at each wait would step 0.25, 0.75, 14 × 1 against
///   0.25, 14 × 1, 0.75, and differ (ACCEPTANCE.md, check 17). All these
///   durations are exact in binary;
/// - 7.3 then 7.7 minutes agree with 15 within 1e-4 relative in paint,
///   solvent and cure at every pixel; and from a fractional start (0.4
///   min), 15 against 15 × 1 within the same.
/// Zero rule: values both no further from zero than `ZERO_UM` (paint,
/// solvent) or `ZERO_CURE` (cure) agree.
#[test]
fn c17_a_wait_split_on_the_minute_grid_is_exact_and_off_it_within_1e4() {
    let mut base = canvas(160);
    thinned_patch(&mut base, raw_sienna(), 0.5, (200.0, 800.0), (300.0, 700.0), 91);
    thinned_patch(&mut base, lead_white(), 0.3, (200.0, 800.0), (500.0, 900.0), 92);
    assert_eq!(base.clock().fract(), 0.0, "the waits start on a whole minute");
    let copy = || with_solvent_scaled(&base, 1.0);
    let exact = |what: &str, a: &Canvas, b: &Canvas| {
        assert_eq!(a.clock(), b.clock(), "{what}: clocks");
        let (sa, sb) = (state(a), state(b));
        if sa != sb {
            let first = sa.iter().zip(&sb).position(|(x, y)| x != y);
            panic!("{what}: the saves differ (sizes {} and {}, first differing byte {first:?}); paint equal: {}, solvent equal: {}, cure equal: {}", sa.len(), sb.len(), paint_um(a) == paint_um(b), solvent_um(a) == solvent_um(b), cure(a) == cure(b));
        }
    };
    let near = |what: &str, a: &Canvas, b: &Canvas| {
        assert_eq!(a.clock(), b.clock(), "{what}: clocks");
        for (q, x, y, floor) in [("paint µm", paint_um(a), paint_um(b), ZERO_UM), ("solvent µm", solvent_um(a), solvent_um(b), ZERO_UM), ("cure", cure(a), cure(b), ZERO_CURE)] {
            let bad: Vec<(usize, f32, f32)> = x.iter().zip(&y).enumerate().filter(|&(_, (p, r))| !close(*p as f64, *r as f64, 1e-4, floor)).map(|(i, (p, r))| (i, *p, *r)).collect();
            assert!(bad.is_empty(), "{what}: {q} differs at {} pixels, e.g. {:?}", bad.len(), &bad[..bad.len().min(5)]);
        }
    };
    let (mut a, mut b, mut d) = (copy(), copy(), copy());
    a.wait(15.0);
    for _ in 0..15 {
        b.wait(1.0);
    }
    d.wait(7.3);
    d.wait(7.7);
    assert!(a.solvent_total() > 0.0, "solvent is present throughout the 15 minutes");
    exact("wait(15) and 15 × wait(1)", &a, &b);
    near("7.3 + 7.7 min and 15 min", &a, &d);
    let stroke_over = |c: &mut Canvas| {
        let mut h = brush(Tool::filbert(30.0), 93, lead_white().with_thinner(0.3), 0.7);
        stroke(c, &mut h, vec![(150.0, 450.0), (850.0, 600.0)], 0.8);
    };
    let (mut g, mut k) = (copy(), copy());
    g.wait(3.0);
    stroke_over(&mut g);
    g.wait(12.0);
    for _ in 0..3 {
        k.wait(1.0);
    }
    stroke_over(&mut k);
    for _ in 0..12 {
        k.wait(1.0);
    }
    assert!(g.solvent_total() > 0.0);
    exact("wait 3, stroke, wait 12 against the same in one-minute waits", &g, &k);
    let (mut p, mut q) = (copy(), copy());
    p.wait(0.25);
    p.wait(0.75);
    p.wait(14.0);
    q.wait(0.25);
    q.wait(14.75);
    assert!(p.solvent_total() > 0.0);
    exact("from 0.25 min: 0.75 + 14 min against 14.75 min", &p, &q);
    let by_hand = || {
        let mut c = copy();
        c.set_hand_time(Some(15.0));
        // 15 s of hand time (marks made off the canvas, counted in its
        // ledger), put on the clock as brushwork's is
        c.tally_mut().secs += 15.0;
        assert_eq!(c.clock_hand_min(), 0.25);
        assert_eq!(c.clock(), base.clock() + 0.25, "hand time put the clock on the quarter minute");
        c
    };
    let (mut hp, mut hq) = (by_hand(), by_hand());
    hp.wait(0.75);
    hp.wait(14.0);
    hq.wait(14.75);
    exact("after 15 s of hand time: 0.75 + 14 min against 14.75 min", &hp, &hq);
    let (mut u, mut v) = (copy(), copy());
    u.wait(0.4);
    v.wait(0.4);
    u.wait(15.0);
    for _ in 0..15 {
        v.wait(1.0);
    }
    near("from 0.4 min: 15 min and 15 × 1 min", &u, &v);
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
