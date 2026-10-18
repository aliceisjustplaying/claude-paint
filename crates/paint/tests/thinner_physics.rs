//! Thinner acceptance checks at the paint engine's own boundary (brush,
//! rag, canvas, clock): checks 4 and 9 of notes/thinner/ACCEPTANCE.md. The
//! measuring helpers, and the interface these tests require, are in
//! `thinner_support/mod.rs`.
//!
//!   cargo test -p paint --release --test thinner_physics

mod thinner_support;

use paint::rag::Rag;
use paint::Tool;
use paint::thinner::evaporation_tau_min;
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

/// Check 9. Evaporation follows `solvent = s0 × exp(-t / τ(h))` at a fixed
/// paint thickness h, τ from `evaporation_tau_min` (the model's own,
/// labeled estimate; the equation is the plan's and is computed here).
/// Two films on a flat ground, one thinned pass and five passes (thicker).
/// Every minute, at every pixel whose paint changed by at most 1e-3 of
/// itself (spreading barely touched it) and still holds ≥ 0.01 µm of
/// solvent: the share of its solvent left equals exp(-1 min / τ(h)) within
/// 2e-4 + 2 × the paint's relative change (a pixel whose paint moved by δ
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
                // 2e-4, was 1e-4: the owner's decision of 2026-10-04 (ACCEPTANCE.md,
                // "Decisions"), when the flow lost its FLOW_MIN cutoff
                assert!((got - want).abs() <= 2e-4 + 2.0 * d, "minute {m}, pixel {i} ({} µm of paint): {got} of its solvent left, the law gives {want}", ph[i]);
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
