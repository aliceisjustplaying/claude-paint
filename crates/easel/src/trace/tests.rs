//! Tiny scenes for tracing a study onto another surface (`--@ trace`): a 100 mm sheet
//! of paper at 256 px (0.39 mm a pixel), a study beside it of the same size.

use crate::session::{SURFACES_MARK, Session, TRACE_MARK, box_for, parse_program};
use mlua::Value;
use paint::Canvas;

const W: usize = 256;
const PAPER: &str = r#"canvas{size=100, aspect=1, seed=3, paper={grammage=160}}"#;
const STUDY: &str = r#"st = sheet{name="study", size=100, aspect=1, paper={grammage=160}, seed=5}
use(st)
h = pencil("2B")
h:line({{150,200},{400,200},{400,600}}, {pressure=0.7, smooth=false})
use("painting")"#;

fn live() -> Session {
    Session::with_box(W, box_for(None).unwrap()).unwrap().with_paper(true).unwrap()
}

fn replay(log: &str) -> Result<Session, String> {
    let mut s = Session::replay_with(W, box_for(Some(log))?).unwrap().with_paper(crate::session::logged_paper(log)).unwrap();
    for (i, c) in parse_program(log).iter().enumerate() {
        s.run(c).map_err(|e| format!("chunk {}: {e}", i + 1))?;
    }
    Ok(s)
}

fn run(s: &mut Session, src: &str) -> String {
    match s.run(src) {
        Ok(r) => r.out,
        Err(e) => panic!("{src}\n→ {e}"),
    }
}

fn state(c: &Canvas) -> Vec<u8> {
    let mut b = Vec::new();
    c.write_state(&mut b, "").unwrap();
    b
}

/// A resting surface's canvas, as it rests.
fn resting(s: &Session, name: &str) -> Canvas {
    let st = s.st.borrow();
    let i = st.surfaces.find(name).unwrap();
    st.surfaces.slots[i].rest.as_ref().unwrap().canvas.as_ref().unwrap().restore().unwrap()
}

fn with_study() -> Session {
    let mut s = live();
    run(&mut s, PAPER);
    run(&mut s, STUDY);
    s
}

fn number(s: &Session, expr: &str) -> f64 {
    s.lua.load(expr).eval::<f64>().unwrap()
}

#[test]
fn a_new_painting_has_the_switch_and_an_old_log_has_no_trace() {
    assert!(box_for(None).unwrap().trace);
    let mut s = with_study();
    let log = s.program("x");
    assert!(log.lines().any(|l| l == TRACE_MARK) && log.lines().any(|l| l == SURFACES_MARK));
    assert!(!s.lua.globals().get::<Value>("trace").unwrap().is_nil());
    // a log painted before the switch: no trace, and it replays as it was painted
    let old = log.replace(&format!("{TRACE_MARK}\n"), "");
    let r = replay(&old).unwrap();
    assert!(!r.st.borrow().tubes.trace);
    assert!(r.lua.globals().get::<Value>("trace").unwrap().is_nil());
    assert_eq!(r.program("x"), old);
    assert!(state(&r.canvas().unwrap()) == state(&s.canvas().unwrap()));
    assert!(state(&resting(&r, "study")) == state(&resting(&s, "study")));
    let e = replay(&format!("{old}\n--@ chunk 3\ntrace(st, {{coat=chalk()}})\n")).err().unwrap();
    assert!(e.contains("trace"), "{e}");
    // without the named surfaces it is accepted and installs nothing
    let mut b = box_for(None).unwrap();
    b.surfaces = false;
    let bare = Session::with_box(W, b).unwrap().with_paper(true).unwrap();
    assert!(bare.lua.globals().get::<Value>("trace").unwrap().is_nil());
    run(&mut s, "trace(st, {coat=chalk()})");
}

/// The study's guide, mirrored if asked, against the painting's after the trace: every line
/// of the trace lies near the study's, and nearly all of the study's line is traced.
fn follows(mirror: bool) {
    let mut s = with_study();
    run(&mut s, &format!("r = trace(st, {{coat=chalk(), pressure=0.6, mirror={mirror}}})"));
    let study = resting(&s, "study").drawing_guide();
    let c = s.canvas().unwrap();
    let g = c.drawing_guide();
    assert_eq!((g.f.w, g.f.h), (study.f.w, study.f.h), "the same size at the same pitch");
    let (w, h) = (g.f.w, g.f.h);
    let src = |x: usize, y: usize| study.data[y * w + if mirror { w - 1 - x } else { x }];
    let near = |f: &dyn Fn(usize, usize) -> bool, x: usize, y: usize, r: usize| {
        (y.saturating_sub(r)..(y + r + 1).min(h)).any(|yy| (x.saturating_sub(r)..(x + r + 1).min(w)).any(|xx| f(xx, yy)))
    };
    // the hand strays a fraction of the line's width: within 2 px of it
    let (mut traced, mut off) = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if g.data[y * w + x] > 0.5 {
                traced += 1;
                if !near(&|xx, yy| src(xx, yy) >= paint::trace::LEVEL, x, y, 2) {
                    off += 1;
                }
            }
        }
    }
    assert!(traced > 50 && off == 0, "{off} of {traced} traced pixels off the study's line");
    let (mut line, mut missed) = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if src(x, y) >= 0.9 {
                line += 1;
                if !near(&|xx, yy| g.data[yy * w + xx] > 0.5, x, y, 2) {
                    missed += 1;
                }
            }
        }
    }
    assert!(missed * 20 <= line, "{missed} of {line} line pixels not traced");
    // the coat lies on the side the study's line is, or its mirror
    let d = (0..h).flat_map(|y| (0..w).map(move |x| (x, y)));
    let (left, right): (Vec<_>, Vec<_>) = d.filter(|&(x, y)| g.data[y * w + x] > 0.5).partition(|&(x, _)| x < w / 2);
    assert!(if mirror { right.len() > 10 * left.len().max(1) } else { left.len() > 10 * right.len().max(1) }, "{} left, {} right", left.len(), right.len());
}

#[test]
fn a_traced_line_follows_the_study() {
    follows(false);
}

#[test]
fn a_mirrored_tracing_follows_the_study_s_mirror() {
    follows(true);
}

#[test]
fn what_leaves_the_coat_lands_on_the_sheet_and_the_study_is_unchanged() {
    let mut s = with_study();
    let study = state(&resting(&s, "study"));
    let before = s.canvas().unwrap().loose_mm3();
    let out = run(&mut s, "r = trace(st, {coat=chalk(), pressure=0.6})");
    assert!(out.contains("traced"), "{out}");
    let moved = number(&s, "r.moved");
    assert!(moved > 0.0 && moved < number(&s, "r.coat"));
    let gained = s.canvas().unwrap().loose_mm3() - before;
    assert!((gained - moved).abs() <= 1e-4 * moved, "{gained} mm³ gained, {moved} left the coat");
    assert!(state(&resting(&s, "study")) == study, "a study read is not changed");
    assert_eq!(s.st.borrow().surfaces.current_name(), "painting");
}

#[test]
fn more_pressure_widens_the_line_up_to_a_limit() {
    let mut s = with_study();
    run(&mut s, "a = trace(st, {pressure=0.1}); b = trace(st, {pressure=0.9}); c = trace(st, {force=50})");
    let (a, b, c) = (number(&s, "a.width"), number(&s, "b.width"), number(&s, "c.width"));
    assert!(b > a, "{b} mm at 0.9 against {a} at 0.1");
    // the limit: a ball 1 mm across in the stack fully dense, 2√(2R·Σtφ)
    let st = s.st.borrow();
    let painting = st.canvas.as_ref().unwrap().paper().copied().unwrap();
    let tracing = paint::trace::tracing_paper(1);
    let dense: f32 = [tracing, painting].iter().map(|p| p.caliper_um() * p.porosity).sum();
    let limit = 2.0 * (2.0 * 0.5 * dense * 1e-3).sqrt() as f64;
    assert!(c > b && c < limit, "{c} mm at 50 N against the limit {limit}");
}

#[test]
fn a_window_trace_is_drawn_with_the_pencil() {
    let mut s = with_study();
    let out = run(&mut s, "p = pencil(\"HB\"); r = trace(st, {by=\"window\", tool=p, pressure=0.5}); print(p.worn)");
    assert!(out.contains("drawn at the window"), "{out}");
    assert!(number(&s, "p.worn") > 50.0, "the point wore by the lines drawn: {} mm; {out}", number(&s, "p.worn"));
    assert!(number(&s, "r.moved") == 0.0);
    // (one HB pass at 0.5 reads about 0.8 in the guide: graphite.rs guide_line)
    assert!(s.canvas().unwrap().drawing_guide().data.iter().any(|&v| v > 0.5));
    assert!(s.run("trace(st, {by=\"window\"})").is_err(), "a window wants a pencil");
}

#[test]
fn a_trace_onto_a_resting_sheet_comes_back_to_the_current_one_and_replays() {
    let mut s = with_study();
    run(&mut s, r#"pr = sheet{name="proof", size=100, aspect=1, paper={grammage=160}, seed=6}
r = trace("study", {onto=pr, coat=chalk(), through="study", mirror=true, at={500, 450}})"#);
    assert_eq!(s.st.borrow().surfaces.current_name(), "painting");
    let proof = resting(&s, "proof");
    assert!(proof.loose_mm3() > 0.0);
    assert_eq!(s.canvas().unwrap().loose_mm3(), 0.0, "the painting took nothing");
    let log = s.program("x");
    let r = replay(&log).unwrap();
    assert!(state(&resting(&r, "proof")) == state(&proof), "a replay traces the same");
    assert!(s.run("trace(st, {onto=st})").is_err());
    assert!(s.run("trace(st, {by=\"press\"})").is_err());
}
