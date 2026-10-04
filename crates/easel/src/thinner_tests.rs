//! Thinner acceptance checks at the easel (Lua, sessions, saves, logs):
//! checks 1, 2, 3, 7, 8 (the card at every thinner), 11, 12 and 18 of
//! notes/thinner/ACCEPTANCE.md, and the rag study. Measuring helpers:
//! `thinner_measure.rs`; tiny inputs: crates/easel/tests/thinner/.
//!
//!   cargo test -p easel --release --bin easel thinner_tests::
//!
//! Check 1 is `#[ignore = "slow"]` and runs only by its exact name
//! (scripts/test_thinner_acceptance --all).

use crate::session::Session;
use crate::thinner_measure::*;
use paint::color::luminance;
use paint::thinner::evaporation_tau_min;

const CANVAS: &str = include_str!("../tests/thinner/canvas.lua");
const THINNED: &str = include_str!("../tests/thinner/thinned.lua");
const NEXT: &str = include_str!("../tests/thinner/next.lua");
const FAILING: &str = include_str!("../tests/thinner/failing.lua");
const DETERMINISM: &str = include_str!("../tests/thinner/determinism.lua");

/// Check 1, the locked target: raw sienna thinned 0.5, one `body` pass at
/// load 0.3 and one at 0.6, over the black-and-white card at 2400 px.
/// First both evaporation checks: the wait is at least ten times the
/// model's evaporation time of the thickest paint on the card, and less
/// than a thousandth of the solvent is left (of the solvent there right
/// after the pass, which is no more than the pass added: stricter). Then
/// at least half the card's contrast shows, at both loads.
#[test]
#[ignore = "slow"]
fn c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px() {
    let cells = card(2400, RAW_SIENNA, &[(0.3, 0.5), (0.6, 0.5)]);
    for c in &cells {
        println!("{c:?}");
        assert!(c.solvent_after_pass > 0.0, "load {}: the pass laid solvent", c.load);
        assert!(c.waited >= 10.0 * c.tau_after_pass && c.waited >= 10.0 * c.tau_at_measure, "load {}: waited {} min; τ {} min after the pass, {} min at the measurement", c.load, c.waited, c.tau_after_pass, c.tau_at_measure);
        assert!(c.solvent_at_measure < 1e-3 * c.solvent_after_pass, "load {}: {} of {} solvent left", c.load, c.solvent_at_measure, c.solvent_after_pass);
    }
    for c in &cells {
        assert!(c.kept >= 0.5, "load {}: {:.1}% of the card's contrast shows (target 50%)", c.load, 100.0 * c.kept);
    }
}

/// Check 2: with no thinner, and with thinner=0 on every pile, each
/// before-change scene (notes/thinner/baseline/, from unchanged af49348)
/// replays to exactly its saved paint state; the solvent is all zero.
#[test]
fn c02_no_thinner_or_thinner_0_leaves_the_saved_paint_state_unchanged() {
    let scenes = baseline_scenes();
    assert!(!scenes.is_empty(), "no scenes in the baseline");
    for sc in &scenes {
        let zero = sc.log.replace("pile{", "pile{thinner=0, ");
        for (variant, log) in [("as logged", &sc.log), ("thinner=0", &zero)] {
            let s = replay(log, sc.width);
            let c = s.canvas().unwrap();
            assert_same_paint_state(&format!("{} ({variant})", sc.name), &sc.state, &state(&c), c.pixels().len());
            assert_eq!(c.solvent_total(), 0.0, "{} ({variant})", sc.name);
        }
    }
}

/// Check 3: a save an older easel wrote of an engine-3 canvas (PAINTCK8)
/// is refused from its header alone, before anything is painted, and the
/// refusal names the version that reads it (af49348).
#[test]
fn c03_a_paintck8_save_of_an_engine_3_canvas_is_refused_by_its_header_naming_the_old_version() {
    let head = "easel save 1\nlog_fnv=0000000000000000\nchunks=1\nbox=inness\nengine=3\nwidth=64\nseed=1\nchunk=1\ncalls=0\nclock=0000000000000000\nclock0=0000000000000000\npiles=\n";
    let mut bytes = b"PAINTCK8".to_vec();
    bytes.extend_from_slice(&(head.len() as u64).to_le_bytes());
    bytes.extend_from_slice(head.as_bytes());
    // no canvas after the header: a tiny file is enough to be refused
    let path = std::env::temp_dir().join(format!("thinner-c03-{}.ckpt", std::process::id()));
    std::fs::write(&path, &bytes).unwrap();
    let r = crate::save::read(&path);
    let _ = std::fs::remove_file(&path);
    let e = match r {
        Ok(_) => panic!("a PAINTCK8 save of an engine-3 canvas was read"),
        Err(e) => e,
    };
    assert!(e.contains("af49348"), "the refusal names the version to use: {e}");
    assert!(!e.contains("fill whole buffer"), "refused from the header, not by running out of file: {e}");
}

/// Check 3: the PAINTCK8 checkpoints unchanged af49348 wrote (the
/// baseline's) are refused by the canvas reader, naming af49348.
#[test]
fn c03_a_baseline_paintck8_checkpoint_is_refused_naming_the_old_version() {
    let scenes = baseline_scenes();
    assert!(!scenes.is_empty(), "no scenes in the baseline");
    for sc in &scenes {
        let mut r: &[u8] = &sc.state;
        match paint::Canvas::read_state(&mut r) {
            Ok(_) => panic!("{}: an af49348 checkpoint of an engine-3 canvas was read", sc.name),
            Err(e) => assert!(e.to_string().contains("af49348"), "{}: the refusal names the version to use: {e}", sc.name),
        }
    }
}

/// Check 3: engines 1 and 2 keep their language, so their logs replay as
/// they were painted: `thinner` is refused and a pile prints as before;
/// engine 3 has it (0 to 0.9).
#[test]
fn c03_engines_1_and_2_have_no_thinner_and_engine_3_has() {
    for e in [1u32, 2] {
        let mut s = inness_engine(120, e);
        run(&mut s, CANVAS);
        let err = s.run(r#"p = pile{{"raw sienna", 1}, thinner=0.5}"#).unwrap_err();
        assert!(err.contains("thinner"), "engine {e}: {err}");
        assert_eq!(run(&mut s, r#"q = pile{{"raw sienna", 1}}; print(q.thinner, q)"#).trim_end(), "nil\tpile(raw sienna 1; medium 0)", "engine {e}");
    }
    let mut s = inness(120);
    run(&mut s, CANVAS);
    assert_eq!(run(&mut s, r#"q = pile{{"raw sienna", 1}}; print(q.thinner, q)"#).trim_end(), "0.0\tpile(raw sienna 1; medium 0)");
    assert_eq!(run(&mut s, r#"p = pile{{"raw sienna", 1}, thinner=0.5}; print(p.thinner, p)"#).trim_end(), "0.5\tpile(raw sienna 1; medium 0, thinner 0.5)");
    for bad in ["0.95", "-0.1", "\"a lot\""] {
        let err = s.run(&format!(r#"p = pile{{{{"raw sienna", 1}}, thinner={bad}}}"#)).unwrap_err();
        assert!(err.contains("thinner"), "thinner={bad}: {err}");
    }
}

/// Check 7: a second thinned pass over the first leaves more paint than
/// the first alone (at least 20% more: each stroke has its own limit).
#[test]
fn c07_two_overlapping_thinned_passes_leave_more_paint_than_one() {
    let mut s = inness(240);
    run(&mut s, CANVAS);
    let inner = (220.0, 220.0, 680.0, 480.0);
    run(&mut s, r#"p = pile{{"raw sienna", 1}, thinner=0.5}; work(rect(200, 200, 500, 300), {hand="body", pile=p, load=0.3, clip=true, seed=3})"#);
    let one = sum_in(&s.canvas().unwrap(), inner, paint_um);
    run(&mut s, r#"work(rect(200, 200, 500, 300), {hand="body", pile=p, load=0.3, clip=true, seed=4})"#);
    let two = sum_in(&s.canvas().unwrap(), inner, paint_um);
    assert!(one > 0.0 && two > 1.2 * one, "paint after one pass {one}, after two {two}");
}

/// Check 8: more thinner never hides the card more. Raw sienna, loads 0.3
/// and 0.6, thinner 0, 0.1, ..., 0.9, each measured after its solvent has
/// gone: the contrast showing never falls from one step to the next by
/// more than half a percentage point (measurement noise), and thinner 0.9
/// shows more than none.
#[test]
fn c08_more_thinner_never_hides_the_card_more() {
    for load in [0.3f32, 0.6] {
        let cells: Vec<(f32, f32)> = (0..10).map(|k| (load, k as f32 / 10.0)).collect();
        let r = card(960, RAW_SIENNA, &cells);
        let kept: Vec<f32> = r.iter().map(|c| c.kept).collect();
        println!("load {load}: kept {kept:?}");
        for c in &r {
            assert!(c.solvent_at_measure <= 1e-3 * c.solvent_after_pass, "load {load}, thinner {}: solvent left", c.thinner);
        }
        for k in 1..kept.len() {
            assert!(kept[k] >= kept[k - 1] - 0.005, "load {load}: thinner {} shows {} of the card, thinner {} {}", r[k - 1].thinner, kept[k - 1], r[k].thinner, kept[k]);
        }
        assert!(kept[9] > kept[0], "load {load}: thinner 0.9 shows more of the card than none");
    }
}

/// Check 11: a session saved halfway through the solvent's evaporation,
/// between whole minutes, reopens to exactly the same canvas; the same
/// next chunk then gives exactly the same canvas in both; and a replay of
/// the whole log from scratch gives that canvas too.
#[test]
fn c11_a_save_mid_evaporation_reopens_to_the_same_state_and_goes_on_the_same() {
    const W: usize = 200;
    let mut a = inness(W);
    run(&mut a, CANVAS);
    run(&mut a, THINNED);
    let (s0, m) = {
        let c = a.canvas().unwrap();
        let mut film: Vec<f32> = centers_in(&c, (100.0, 130.0, 900.0, 400.0)).into_iter().map(|(x, y)| c.wet_um(x, y)).filter(|&v| v >= 1.0).collect();
        film.sort_by(f32::total_cmp);
        assert!(!film.is_empty());
        let half = evaporation_tau_min(film[film.len() / 2]) * std::f64::consts::LN_2;
        (c.solvent_total(), (half * 10.0).round() / 10.0 + 0.37)
    };
    assert!(s0 > 0.0, "the strokes laid solvent");
    run(&mut a, &format!("wait({m})"));
    {
        let c = a.canvas().unwrap();
        let s = c.solvent_total();
        assert!(s > 0.1 * s0 && s < 0.9 * s0, "mid-evaporation: {s} of {s0} left");
        let f = c.clock().fract();
        assert!(f > 1e-6 && f < 1.0 - 1e-6, "between whole minutes: clock {}", c.clock());
    }
    let path = std::env::temp_dir().join(format!("thinner-c11-{}.ckpt", std::process::id()));
    crate::save::write(&a, &a.program("t"), &path).unwrap();
    let restored = crate::save::read(&path);
    let _ = std::fs::remove_file(&path);
    let mut b = match restored {
        Ok(r) => r.session,
        Err(e) => panic!("reopening the save: {e}"),
    };
    assert!(state(&a.canvas().unwrap()) == state(&b.canvas().unwrap()), "the reopened canvas differs from the saved one");
    run(&mut a, NEXT);
    run(&mut b, NEXT);
    assert!(state(&a.canvas().unwrap()) == state(&b.canvas().unwrap()), "going on from the save differs from going on without closing");
    let r = replay(&a.program("t"), W);
    assert!(state(&r.canvas().unwrap()) == state(&a.canvas().unwrap()), "the replayed log differs from the live session");
}

/// Check 12: the same tiny log (thinned strokes and passes, waits, a damp
/// rag) saves exactly the same bytes painted on one thread and on four.
#[test]
fn c12_the_same_log_saves_the_same_bytes_on_one_thread_and_four() {
    let on = |n: usize| -> (Vec<u8>, f64) {
        rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap().install(|| {
            let s = replay(DETERMINISM, 240);
            let c = s.canvas().unwrap();
            (state(&c), c.solvent_total())
        })
    };
    let (a, solvent) = on(1);
    let (b, _) = on(4);
    assert!(solvent > 0.0, "the log ends with solvent on the canvas");
    assert!(a == b, "one thread and four save different bytes ({} and {})", a.len(), b.len());
}

/// Check 18: a chunk that fails after thinned strokes, a thinned pass, a
/// wait and a damp rag is taken back whole (canvas with its solvent,
/// clock, brushes, rags), and the next chunk paints exactly as in a
/// session where the failed chunk never ran.
#[test]
fn c18_a_failed_chunk_after_thinned_paint_takes_everything_back() {
    let (mut s, mut t) = (inness(160), inness(160));
    for x in [&mut s, &mut t] {
        run(x, CANVAS);
        run(x, THINNED);
    }
    let held = |s: &Session| -> (Vec<String>, Vec<String>) {
        let mut st = s.st.borrow_mut();
        (st.live_brushes().iter().map(|b| format!("{:?}", b.borrow())).collect(), st.live_rags().iter().map(|r| format!("{:?}", r.borrow())).collect())
    };
    let (bytes0, clock0, hand0) = (state(&s.canvas().unwrap()), s.st.borrow().clock, held(&s));
    assert!(s.canvas().unwrap().solvent_total() > 0.0);
    let e = s.run(FAILING).unwrap_err();
    assert!(e.contains("stop"), "{e}");
    assert!(state(&s.canvas().unwrap()) == bytes0, "the canvas (paint, solvent, clock) is as before the failed chunk");
    assert_eq!(s.st.borrow().clock, clock0);
    assert_eq!(held(&s), hand0, "the brushes and rags are as before");
    run(&mut s, NEXT);
    run(&mut t, NEXT);
    assert!(state(&s.canvas().unwrap()) == state(&t.canvas().unwrap()), "the next chunk saw a different state than if the failed chunk had never run");
    assert_eq!(held(&s), held(&t));
}

// ---------------------------------------------------------------- the rag study

const RAG_CANVAS: &str = r#"canvas{size=300, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=40, apply="brush"}}}"#;
const WIPE: &str = r#"r = rag(); r:wipe({{150, 275}, {750, 275}}, {pressure=0.6, seed=2})"#;
const BLOT: &str = r#"r = rag(); r:blot(450, 275, {pressure=0.6, seed=2})"#;
const DAMP_WIPE: &str = r#"r = rag(); r:dip(0.5); r:wipe({{150, 275}, {750, 275}}, {pressure=0.6, seed=2})"#;
const BRUSH_OVER: &str = r#"b = brush("filbert", 30); b:load(pile{{"cobalt blue", 1}, {"lead white", 2}}, 0.8); b:stroke({{450, 140}, {450, 410}}, {pressure={0.8, 0.8}})"#;

/// One panel: the tone (`thinner` 0 = none), `setup` after it, a picture,
/// `action`, a picture; and what the action did.
#[derive(Debug)]
struct Panel {
    name: &'static str,
    /// Share of the open paint in the band along the wipe that the action
    /// removed (negative: it added paint).
    removed: f64,
    /// How much of the tone's darkening of the ground still shows in the
    /// band (1 = all of it, 0 = back to the ground).
    tone_left: f32,
    /// Relative change of the open paint well outside the cloth's reach.
    outside: f64,
    /// Elongation of the lifted patch (sqrt of its second moments' ratio).
    elongation: f32,
    /// Mean open paint in the band, µm, before and after.
    paint: (f64, f64),
    /// The same in the column a vertical stroke at x = 450 crosses the band.
    column: (f64, f64),
    before: (u32, u32, Vec<u8>),
    after: (u32, u32, Vec<u8>),
    /// Luminance of the band before and after, and of the ground.
    lum: (f32, f32, f32),
}

fn rag_panel(name: &'static str, thinner: f32, setup: &str, action: &str) -> Panel {
    let mut s = inness(256);
    run(&mut s, RAG_CANVAS);
    let th = if thinner > 0.0 { format!(", thinner={thinner}") } else { String::new() };
    run(&mut s, &format!(r#"tone = pile{{{{"raw umber", 2}}, {{"bone black", 1}}{th}}}; work(rect(100, 120, 800, 310), {{hand="broad", pile=tone, coverage=2.5, angle=0, seed=1}})"#));
    if !setup.is_empty() {
        run(&mut s, setup);
    }
    let (band, outside, ground, tone) = ((250.0, 255.0, 650.0, 295.0), (250.0, 135.0, 650.0, 175.0), (20.0, 20.0, 80.0, 80.0), (100.0, 120.0, 900.0, 430.0));
    let column = (440.0, 255.0, 460.0, 295.0);
    let snap = |s: &Session| {
        let c = s.canvas().unwrap();
        let film: Vec<(f32, f32, f32)> = centers_in(&c, tone).into_iter().map(|(x, y)| (x, y, c.wet_um(x, y))).collect();
        let col = sum_in(&c, column, paint_um) / centers_in(&c, column).len().max(1) as f64;
        (picture(&c), sum_in(&c, band, paint_um), sum_in(&c, outside, paint_um), luminance(mean_rgb(&c, band)), luminance(mean_rgb(&c, ground)), film, centers_in(&c, band).len(), col)
    };
    let (before, pb, ob, lb, lg, film0, nb, cb) = snap(&s);
    run(&mut s, action);
    let (after, pa, oa, la, _, film1, _, ca) = snap(&s);
    let lifted: Vec<(f32, f32)> = film0.iter().zip(&film1).filter(|(a, b)| a.2 >= 1.0 && b.2 < 0.7 * a.2).map(|(a, _)| (a.0, a.1)).collect();
    let elongation = {
        let n = lifted.len().max(1) as f32;
        let (mx, my) = (lifted.iter().map(|p| p.0).sum::<f32>() / n, lifted.iter().map(|p| p.1).sum::<f32>() / n);
        let (a, b, c) = lifted.iter().fold((0.0f32, 0.0f32, 0.0f32), |(a, b, c), p| (a + (p.0 - mx).powi(2), b + (p.0 - mx) * (p.1 - my), c + (p.1 - my).powi(2)));
        let (a, b, c) = (a / n, b / n, c / n);
        let d = (((a - c) / 2.0).powi(2) + b * b).sqrt();
        (((a + c) / 2.0 + d) / ((a + c) / 2.0 - d).max(1e-6)).sqrt()
    };
    Panel {
        name,
        removed: if pb > 0.0 { 1.0 - pa / pb } else { 0.0 },
        tone_left: (lg - la) / (lg - lb),
        outside: if ob > 0.0 { (oa - ob).abs() / ob } else { 0.0 },
        elongation,
        paint: (pb / nb.max(1) as f64, pa / nb.max(1) as f64),
        column: (cb, ca),
        before,
        after,
        lum: (lb, la, lg),
    }
}

/// The rag study: a dry cloth wiping and blotting wet paint, a cloth
/// dipped in spirits wiping it, the same on thinned paint, a dry-paint
/// control and a brush stroke over a wiped area, each a 256 px panel
/// before and after. The picture is written to $THINNER_RAG_STUDY (PNG)
/// when that is set; the numbers are printed. Expected, from what a rag
/// does (limits in ACCEPTANCE.md): the wipe lifts where it went (at least
/// a quarter of the paint along its middle, under 1% change well outside
/// it) and the ground shows (under 90% of the tone left); spirits lift
/// more than a dry cloth; a wipe's lift is long (elongation at least 2.5)
/// and a blot's round (at most 1.6); dry paint doesn't come up; a stroke
/// over a wiped area lays paint there (at least 5 µm more on average where
/// it crosses the wipe).
#[test]
fn rag_study() {
    let wet = rag_panel("dry cloth wipe, wet paint", 0.0, "", WIPE);
    let blot = rag_panel("blot, wet paint", 0.0, "", BLOT);
    let damp = rag_panel("damp cloth wipe, wet paint", 0.0, "", DAMP_WIPE);
    let t_wet = rag_panel("dry cloth wipe, thinned paint", 0.5, "", WIPE);
    let t_blot = rag_panel("blot, thinned paint", 0.5, "", BLOT);
    let t_damp = rag_panel("damp cloth wipe, thinned paint", 0.5, "", DAMP_WIPE);
    let dry = rag_panel("dry cloth wipe, dry paint (control)", 0.0, "wait(60 * 24 * 60)", WIPE);
    let over = rag_panel("brush stroke over a wiped area", 0.0, WIPE, BRUSH_OVER);
    let panels = [&wet, &blot, &damp, &t_wet, &t_blot, &t_damp, &dry, &over];
    for p in panels {
        println!("{:<38} removed {:>6.1}%  tone left {:>5.1}%  outside {:>5.2}%  elongation {:>4.1}  paint {:.2} -> {:.2} µm  L {:.3} -> {:.3} (ground {:.3})", p.name, 100.0 * p.removed, 100.0 * p.tone_left, 100.0 * p.outside, p.elongation, p.paint.0, p.paint.1, p.lum.0, p.lum.1, p.lum.2);
    }
    if let Ok(out) = std::env::var("THINNER_RAG_STUDY") {
        let (w, h) = (wet.before.0, wet.before.1);
        let mut sheet = image::RgbImage::new(2 * w, h * panels.len() as u32);
        for (k, p) in panels.iter().enumerate() {
            for (col, pic) in [&p.before, &p.after].into_iter().enumerate() {
                for y in 0..pic.1 {
                    for x in 0..pic.0 {
                        let i = ((y * pic.0 + x) * 3) as usize;
                        sheet.put_pixel(col as u32 * w + x, k as u32 * h + y, image::Rgb([pic.2[i], pic.2[i + 1], pic.2[i + 2]]));
                    }
                }
            }
        }
        sheet.save(&out).unwrap_or_else(|e| panic!("{out}: {e}"));
        println!("rag study written to {out}");
    }
    for p in [&wet, &damp, &t_wet, &t_damp] {
        assert!(p.removed >= 0.25, "{}: lifted {:.1}% of the paint along the wipe", p.name, 100.0 * p.removed);
        assert!(p.outside <= 0.01, "{}: the paint well outside the cloth changed by {:.2}%", p.name, 100.0 * p.outside);
        assert!(p.tone_left < 0.9, "{}: the ground shows through ({:.1}% of the tone left)", p.name, 100.0 * p.tone_left);
    }
    assert!(damp.removed > wet.removed && t_damp.removed > t_wet.removed, "spirits lift more: {} vs {}, thinned {} vs {}", damp.removed, wet.removed, t_damp.removed, t_wet.removed);
    for (w, b) in [(&wet, &blot), (&t_wet, &t_blot)] {
        assert!(w.elongation >= 2.5 && b.elongation <= 1.6, "a wipe lifts a long streak ({:.2}), a blot a round patch ({:.2})", w.elongation, b.elongation);
        assert!(b.removed > 0.0, "{}: the blot lifted paint", b.name);
    }
    assert!(dry.paint.0 == 0.0 && (dry.lum.1 - dry.lum.0).abs() <= 1e-6, "dry paint doesn't come up: paint {:?}, luminance {} -> {}", dry.paint, dry.lum.0, dry.lum.1);
    assert!(over.column.1 > over.column.0 + 5.0, "the stroke laid paint over the wiped area: {:?} µm where it crossed the wipe", over.column);
}
