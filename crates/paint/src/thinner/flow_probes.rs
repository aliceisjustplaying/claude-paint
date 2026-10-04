//! Lane B probes for the thin-film flow (notes/engine3/HANDOVER-3.md row B;
//! AGENT_BRIEF_V2 §4a-b; REVIEW_RESPONSE §3-4). All `#[ignore]`: they print
//! tables, they assert only material balance and nonnegativity.
//!
//! `cargo test --release -p paint --lib thinner::flow_probes -- --ignored --nocapture --test-threads 1`
//!
//! The direct films sit on a smooth diagnostic ground: `Canvas::new`
//! without priming or linen, relief 0 everywhere, nothing retained by the
//! ground. Pixel spacing 0.1833 mm (2400 px across 440 mm), as at the
//! review's mixed-field case.

use crate::bristle::{Gesture, Held, Tool};
use crate::canvas::{Canvas, Crop};
use crate::color::hex;
use crate::surface::COAT_UM;
use crate::wet::Paint;

fn paint() -> Paint {
    Paint::new(hex("#9a5a2a"), 0.85, 0.8)
}

/// A smooth zero-relief canvas, `px` square pixels at 0.1833 mm.
fn smooth(px: usize) -> Canvas {
    let mm = 440.0 * px as f32 / 2400.0;
    let mut c = Canvas::new(px, 1.0, hex("#d8cdb8")).with_engine(3).with_size_mm(mm);
    c.wet.ensure_solvent();
    c
}

/// Lay a fresh film directly over the pixel rectangle `r`: `liquid_um` of
/// paint + solvent holding the share `phi` of solvent, times
/// `1 + ripple · sin(2π x / 11 px)` (a 2 mm ripple at this spacing).
fn lay(c: &mut Canvas, r: (usize, usize, usize, usize), liquid_um: f32, phi: f32, ripple: f32, id: u32) {
    let p = paint();
    let w = c.f.w;
    for y in r.1..r.3 {
        for x in r.0..r.2 {
            let i = y * w + x;
            let l = liquid_um * (1.0 + ripple * (std::f32::consts::TAU * x as f32 / 11.0).sin());
            c.wet.vol[i] = l * (1.0 - phi) / COAT_UM;
            c.wet.solv[i] = l * phi;
            c.wet.lat[i] = p.latent();
            c.wet.hide[i] = [p.scatter, p.stiff, p.drying];
            c.wet.stroke[i] = id;
        }
    }
    c.wet.current = id;
    c.wet.touch(r.0, r.1, r.2, r.3);
}

fn sums(c: &Canvas) -> (f64, f64) {
    (c.wet.vol.iter().map(|&v| v as f64).sum(), c.wet.solv.iter().map(|&v| v as f64).sum())
}

/// Half the summed |change| of paint over the total: the share of the
/// paint that moved.
fn moved(a: &[f32], b: &[f32]) -> f64 {
    let all: f64 = a.iter().map(|&v| v as f64).sum();
    a.iter().zip(b).map(|(p, q)| (p - q).abs() as f64).sum::<f64>() / 2.0 / all.max(1e-30)
}

fn in_rect(w: usize, i: usize, r: (usize, usize, usize, usize)) -> bool {
    let (x, y) = (i % w, i / w);
    x >= r.0 && x < r.2 && y >= r.1 && y < r.3
}

fn nonneg(c: &Canvas) {
    assert!(c.wet.vol.iter().all(|&v| v >= 0.0 && v.is_finite()), "negative or non-finite paint");
    assert!(c.wet.solv.iter().all(|&v| v >= 0.0 && v.is_finite()), "negative or non-finite solvent");
}

/// Direct films: uniform squares of total liquid L at solvent share phi on
/// the smooth ground. (a) the flow alone, `spread` on the 1/64-minute grid
/// for one minute, no evaporation: paint and solvent balance, and the share
/// of the paint that left the square; (b) `wait(5)` (evaporation too): the
/// share moved and left.
#[test]
#[ignore]
fn direct_films() {
    let sq = (80, 80, 160, 160);
    println!("L_um phi paint_um | flow-only 1 min: left%  moved%  dpaint_rel  dsolv_rel  substeps | wait(5): left%  moved%");
    for phi in [0.5f32, 0.75, 0.9] {
        for l in [0.25f32, 0.5, 1.0, 2.0, 3.0, 10.0, 69.0] {
            let mut c = smooth(240);
            lay(&mut c, sq, l, phi, 0.0, 1);
            let w = c.f.w;
            let v0 = c.wet.vol.clone();
            let (p0, s0) = sums(&c);
            let mut a = c.clone();
            let mut used = 0;
            for _ in 0..super::FLOW_TICKS {
                used += a.spread(1.0 / super::FLOW_TICKS as f32).used;
            }
            nonneg(&a);
            let (p1, s1) = sums(&a);
            let left = |c: &Canvas| -> f64 { (0..c.wet.vol.len()).filter(|&i| !in_rect(w, i, sq)).map(|i| c.wet.vol[i] as f64).sum::<f64>() / p0 };
            let (la, ma) = (left(&a), moved(&v0, &a.wet.vol));
            let mut b = c.clone();
            b.wait(5.0);
            nonneg(&b);
            let pb = sums(&b).0;
            assert!(((pb - p0) / p0).abs() < 1e-5, "wait(5) paint balance {p0} → {pb}");
            assert!(((p1 - p0) / p0).abs() < 1e-5 && ((s1 - s0) / s0).abs() < 1e-5, "flow balance: paint {p0} → {p1}, solvent {s0} → {s1}");
            println!(
                "{l:5.2} {phi:.2} {:7.4} | {:9.5} {:7.4} {:+.1e} {:+.1e} {used:4} | {:9.5} {:7.4}",
                l * (1.0 - phi),
                100.0 * la,
                100.0 * ma,
                (p1 - p0) / p0,
                (s1 - s0) / s0,
                100.0 * left(&b),
                100.0 * moved(&v0, &b.wet.vol)
            );
        }
    }
}

/// The mixed-field case (REVIEW_RESPONSE §4): an active leveling patch
/// (phi 0.5, 20 µm liquid with a ±50% 2 mm ripple) alone, and with a
/// separate high-solvent below-floor patch (phi 0.95, 0.667 µm liquid) in the
/// same dirty region; then the same thin patch touching the active one, so
/// it receives liquid and can become active during the step. Each run as
/// one direct `spread(1)` (the review's frozen-coefficient test) and as one
/// minute on the 1/64-minute grid. Logged: the largest mobility, the largest
/// donor-eligible mobility, requested/used substeps, scheduled minutes, the
/// active patch's transport, and how far the active patch's end state lies
/// from the active-only run.
#[test]
#[ignore]
fn mixed_field() {
    let act = (40, 60, 100, 120);
    let far = (160, 60, 220, 120);
    let near = (100, 60, 160, 120);
    let cases: [(&str, Option<(usize, usize, usize, usize)>); 3] = [("active only", None), ("+ separate thin phi .95", Some(far)), ("+ adjacent thin phi .95", Some(near))];
    let mut ends: Vec<Vec<f32>> = Vec::new();
    for (how, ticks) in [("spread(1)", 1u32), ("64 x spread(1/64)", super::FLOW_TICKS)] {
        println!("-- {how}");
        println!("case                      | m_max   m_donor | requested used | scheduled_min | active moved%  | max |Δ active| vs active-only (µm) | dpaint_rel dsolv_rel");
        ends.clear();
        for (name, thin) in cases {
            let mut c = smooth(260);
            lay(&mut c, act, 20.0, 0.5, 0.5, 1);
            if let Some(r) = thin {
                lay(&mut c, r, 2.0 / 3.0, 0.95, 0.0, 2);
            }
            c.wet.touch(0, 0, 260, 180);
            let w = c.f.w;
            let v0 = c.wet.vol.clone();
            let (p0, s0) = sums(&c);
            let (mut m_max, mut m_don, mut req, mut used, mut sched) = (0.0f32, 0.0f32, 0usize, 0usize, 0.0f64);
            for _ in 0..ticks {
                let f = c.spread(1.0 / ticks as f32);
                m_max = m_max.max(f.m_max);
                m_don = m_don.max(f.m_donor);
                req = req.max(f.requested);
                used += f.used;
                sched += f.scheduled_min as f64;
            }
            nonneg(&c);
            let (p1, s1) = sums(&c);
            let idx: Vec<usize> = (0..v0.len()).filter(|&i| in_rect(w, i, act)).collect();
            let a0: Vec<f32> = idx.iter().map(|&i| v0[i]).collect();
            let a1: Vec<f32> = idx.iter().map(|&i| c.wet.vol[i]).collect();
            let dev = ends.first().map_or(0.0, |e: &Vec<f32>| e.iter().zip(&a1).map(|(p, q)| (p - q).abs() * COAT_UM).fold(0.0f32, f32::max));
            println!(
                "{name:25} | {m_max:.4} {m_don:.4} | {req:9} {used:4} | {sched:13.4} | {:13.4} | {dev:.3e} | {:+.1e} {:+.1e}",
                100.0 * moved(&a0, &a1),
                (p1 - p0) / p0,
                (s1 - s0) / s0
            );
            ends.push(a1);
        }
    }
}

/// A thinned patch made by the brush (three strokes at thinner 0.5 on a
/// primed ground, 2400 px across 440 mm, crop window), after a quarter
/// minute's wait: the incoming state of the timing probes.
fn brush_patch() -> Canvas {
    let mut c = Canvas::new_window(2400, 1.5, hex("#d8cdb8"), Some(Crop { units: [380.0, 280.0, 620.0, 420.0], margin: 10.0 })).with_engine(3).with_size_mm(440.0);
    c.prime(hex("#e4dcc8"), 0.9, 40.0, 0.6, 0.0, 7);
    for k in 0..3 {
        let y = 320.0 + 25.0 * k as f32;
        let mut h = Held::new(Tool::hog_flat(40.0), 10 + k);
        h.load(paint().with_thinner(0.5), 0.9);
        c.drag(&mut h, &Gesture::new(vec![(400.0, y), (600.0, y + 3.0)]).pressure(0.85, 0.85), None);
    }
    c.wait(0.25);
    c
}

fn maxdiff(a: &Canvas, b: &Canvas) -> (f32, f32) {
    let d = |x: &[f32], y: &[f32], k: f32| x.iter().zip(y).map(|(p, q)| (p - q).abs() * k).fold(0.0f32, f32::max);
    (d(&a.wet.vol, &b.wet.vol, COAT_UM), d(&a.wet.solv, &b.wet.solv, 1.0))
}

fn at_phase(c0: &Canvas, phase: f64) -> Canvas {
    let mut c = c0.clone();
    c.wet.clock.now = c0.wet.clock.now.floor() + 3.0 + phase;
    c
}

/// AGENT_BRIEF_V2 §4a: equal 0.02-minute waits from phases 0.10 and 0.99
/// with identical incoming state (the clock set directly, nothing else
/// changed); a sweep of phases across one tick; split and unsplit waits;
/// save/reload mid-wait.
#[test]
#[ignore]
fn short_waits_b() {
    let c0 = brush_patch();
    println!("incoming: clock {:.4}, paint {:.6}, solvent {:.6}", c0.clock(), c0.wet_total(), c0.solvent_total());
    let run = |phase: f64, waits: &[f32]| -> Canvas {
        let mut c = at_phase(&c0, phase);
        for &t in waits {
            c.wait(t);
        }
        c
    };
    let v0 = c0.wet.vol.clone();
    for len in [0.02f32, 0.005, 0.5] {
        let (a, b) = (run(0.10, &[len]), run(0.99, &[len]));
        let (dp, ds) = maxdiff(&a, &b);
        println!("wait({len}) from 0.10: moved {:.5}%; from 0.99: moved {:.5}%; max |Δ| between them: paint {dp:.3e} µm, solvent {ds:.3e} µm", 100.0 * moved(&v0, &a.wet.vol), 100.0 * moved(&v0, &b.wet.vol));
    }
    let mut lo = f64::MAX;
    let mut hi = 0.0f64;
    for j in 0..32 {
        let ph = 0.10 + j as f64 / 64.0 / 32.0;
        let m = moved(&v0, &run(ph, &[0.02]).wet.vol);
        lo = lo.min(m);
        hi = hi.max(m);
    }
    println!("wait(0.02) from 32 phases across one tick after 0.10: moved {:.5}%..{:.5}% (max/min {:.3})", 100.0 * lo, 100.0 * hi, hi / lo.max(1e-30));
    let one = run(0.10, &[0.02]);
    for (what, w) in [("0.01 + 0.01", vec![0.01f32, 0.01]), ("0.007 + 0.013", vec![0.007, 0.013]), ("20 x 0.001", vec![0.001; 20])] {
        let (dp, ds) = maxdiff(&one, &run(0.10, &w));
        println!("from 0.10, 0.02 against {what}: max |Δ| paint {dp:.3e} µm, solvent {ds:.3e} µm");
    }
    let mut r = at_phase(&c0, 0.10);
    r.wait(0.01);
    let mut buf = Vec::new();
    r.write_state(&mut buf, "b").unwrap();
    let (mut r2, _) = Canvas::read_state(&mut buf.as_slice()).unwrap();
    r2.wait(0.01);
    let split = run(0.10, &[0.01, 0.01]);
    let (dp, ds) = maxdiff(&split, &r2);
    let (dp1, ds1) = maxdiff(&one, &r2);
    println!("save/reload after 0.01 then 0.01: against the same split without reload max |Δ| paint {dp:.3e}, solvent {ds:.3e}; against unsplit 0.02 paint {dp1:.3e}, solvent {ds1:.3e}");
    for c in [&one, &split, &r2] {
        nonneg(c);
    }
}

/// Brush strokes at thinner 0.5, 0.75 and 0.9 (one pass, and three passes
/// along the same path with a reload each), 2400 px across 440 mm, crop
/// window: the film they lay (nonvolatile and total liquid) and the share
/// of the paint 5 minutes move.
#[test]
#[ignore]
fn brush_strokes() {
    println!("thinner passes | paint_um p50 p90 | liquid_um p50 p90 | liquid<=2um share | 5 min moved% | paint balance");
    for t in [0.5f32, 0.75, 0.9] {
        for passes in [1u64, 3] {
            let mut c = Canvas::new_window(2400, 1.5, hex("#d8cdb8"), Some(Crop { units: [380.0, 280.0, 620.0, 360.0], margin: 10.0 })).with_engine(3).with_size_mm(440.0);
            c.prime(hex("#e4dcc8"), 0.9, 40.0, 0.6, 0.0, 7);
            for k in 0..passes {
                let mut h = Held::new(Tool::hog_flat(40.0), 10 + k);
                h.load(paint().with_thinner(t), 0.9);
                c.drag(&mut h, &Gesture::new(vec![(400.0, 320.0), (600.0, 323.0)]).pressure(0.85, 0.85), None);
            }
            let v0 = c.wet.vol.clone();
            let mut pv: Vec<f32> = v0.iter().map(|&v| v * COAT_UM).filter(|&v| v > 0.01).collect();
            let mut lv: Vec<f32> = (0..v0.len()).map(|i| v0[i] * COAT_UM + c.wet.solv[i]).filter(|&l| l > 0.01).collect();
            pv.sort_by(f32::total_cmp);
            lv.sort_by(f32::total_cmp);
            let thin = lv.iter().filter(|&&l| l <= 2.0).count() as f64 / lv.len() as f64;
            let p0 = c.wet_total();
            c.wait(5.0);
            nonneg(&c);
            println!(
                "{t:.2} {passes} | {:.3} {:.3} | {:.3} {:.3} | {:.3} | {:.5} | {:+.1e}",
                pv[pv.len() / 2],
                pv[pv.len() * 9 / 10],
                lv[lv.len() / 2],
                lv[lv.len() * 9 / 10],
                thin,
                100.0 * moved(&v0, &c.wet.vol),
                (c.wet_total() - p0) / p0
            );
        }
    }
}
