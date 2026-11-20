//! Tiny plates (40 × 30 mm beside a 64 px painting) for `--@ press`: ink and wiping.

use crate::session::{DAMP_PAPER_MARK, PLATE_MARK, PRESS_MARK, SURFACES_MARK, Session, box_for, parse_program};

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const PLATE: &str = r#"k = plate{name="key", metal="copper", temper="half-hard", size={40, 30}}
n = needle{tip=25, cone=30}
use(k)
k:drypoint(n, {{100, 300}, {900, 300}}, {force=3, alt=30, azimuth=60})
k:drypoint(n, {{100, 500}, {900, 520}}, {force=2})"#;

fn live() -> Session {
    Session::with_box(W, box_for(None).unwrap()).unwrap().with_paper(true).unwrap()
}

fn replay(log: &str) -> std::result::Result<Session, String> {
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

fn plates(s: &Session) -> Vec<(String, Vec<u8>)> {
    crate::surfaces::plates(&s.st.borrow())
}

fn plate(s: &Session) -> paint::plate::Plate {
    let st = s.st.borrow();
    let p = st.plate.as_ref().map(|p| (**p).clone());
    p.or_else(|| st.surfaces.slots.iter().find_map(|x| x.rest.as_ref().and_then(|r| r.own.plate.as_ref().map(|p| (**p).clone())))).expect("a plate")
}

/// The box's darkest tube: the ink's pigment, whatever box the easel is set to.
fn darkest(s: &Session) -> String {
    let st = s.st.borrow();
    st.tubes.tubes.iter().min_by(|a, b| paint::color::luminance(a.color).total_cmp(&paint::color::luminance(b.color))).unwrap().name.to_string()
}

/// A plate inked, wiped, hand-wiped and retroussé; the ink is conserved between the plate and
/// the tarlatan and palm, as the verbs report it.
fn print_plate(s: &mut Session) -> String {
    let d = darkest(s);
    format!(
        r#"black = ink{{{{"{d}", 1}}, oil=0.1, stiff=1}}
k:warm(40)
r = k:ink(black, {{{{0, 375}}, {{1000, 375}}}})
assert(r.added > 0 and r.film > 0 and r.fill > 0.5, "inked")
t = tarlatan{{stiff=0.7}}
w = t:wipe(k, {{{{0, 375}}, {{1000, 375}}}}, {{passes=3}})
assert(math.abs((w.before - w.after) - w.kept) <= 1e-6 * w.before, "the tarlatan keeps what the plate lost")
assert(math.abs(t.ink - w.kept) <= 1e-6 * w.before, "t.ink")
h = k:hand_wipe({{{{0, 375}}, {{1000, 375}}}}, {{passes=2}})
assert(h.film < w.film, "the palm polishes the tone")
k:retrousse({{{{0, 300}}, {{1000, 300}}}}, {{width=10}})
print(k:feel(500, 300).words)"#
    )
}

/// Ink and wiping on the clock: the hand time passes, the words and probe find the ink, the
/// tarlatan's load is read back, and the plate looks.
#[test]
fn a_plate_is_inked_and_wiped_on_the_clock() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let t0 = s.st.borrow().clock;
    let src = print_plate(&mut s);
    let out = run(&mut s, &src);
    assert!(out.contains("ink, "), "{out}");
    run(&mut s, "use(\"painting\")");
    let t1 = s.st.borrow().clock;
    // a 40 × 30 mm plate: a card's, three tarlatan passes', two palm passes', a retroussage's
    // hand time and the hot plate's two minutes
    assert!(t1 - t0 >= 2.0, "{t0} → {t1}");
    let p = plate(&s);
    assert!(p.inked());
    let (l, f) = p.ink_um3();
    assert!(l > 0.0 && f > 0.0);
    // a look at the plate sees ink in its lines
    let png = s.st.borrow().surfaces.slots.iter().any(|x| x.name == "key");
    assert!(png);
}

/// The plate's ink survives a replay, a failed chunk, a save round trip, a replay checkpoint
/// and a handoff, where it arrives aged by its time on the way: three days later its ink has
/// set and no wipe moves it.
#[test]
fn an_inked_plate_survives_replay_save_checkpoint_and_handoff() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let src = print_plate(&mut s);
    run(&mut s, &src);
    run(&mut s, "wait(5)");
    let log = s.program("press");
    assert!(log.lines().any(|l| l == PRESS_MARK));
    let r = replay(&log).unwrap();
    assert!(plates(&r) == plates(&s), "the replay's plate is the painting's");

    let before = plates(&s);
    let e = s.run("t:wipe(k, {{0, 375}, {1000, 375}}, {passes=2})\nerror(\"no\")").unwrap_err();
    assert!(e.contains("no"));
    assert!(plates(&s) == before, "a failed chunk puts the plate back");

    let dir = std::path::PathBuf::from(format!("out/easel/ink-test-{}", std::process::id()));
    let abs = crate::session::root().join(&dir);
    let _ = std::fs::remove_dir_all(&abs);
    std::fs::create_dir_all(&abs).unwrap();
    let path = abs.join("live.ckpt");
    assert!(crate::save::write(&s, &log, &path).unwrap());
    let back = crate::save::read(&path).unwrap();
    assert!(plates(&back.session) == plates(&s), "a save round trip keeps the plate and its ink");
    assert!(back.session.st.borrow().tubes.press);

    // a replay checkpoint after the third chunk (the plate inked and wiped), resumed
    let ck = abs.join("replay.ckpt");
    let chunks = parse_program(&log);
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string(), "--until-chunk".into(), "3".into()];
    let mut first = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    let mut batch = crate::replay_checkpoint::Batch::open(&args, &log, W, &mut first, &chunks, &abs.join("out.png")).unwrap();
    for (i, c) in chunks.iter().enumerate() {
        first.run(c).unwrap();
        if batch.completed(&first, i + 1).unwrap() {
            break;
        }
    }
    drop(batch);
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string()];
    let mut resumed = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    let batch = crate::replay_checkpoint::Batch::open(&args, &log, W, &mut resumed, &chunks, &abs.join("out.png")).unwrap();
    assert_eq!(batch.done, 3);
    assert!(plates(&resumed) == plates(&first));
    for c in &chunks[3..] {
        resumed.run(c).unwrap();
    }
    drop(batch);
    assert!(plates(&resumed) == plates(&r), "and on, as the painting went");

    // handed off; received three days later: the ink has dried in on the way
    let sent = plate(&s);
    let out = s.run(&format!("handoff(k, \"{}/key.surface\")", dir.display())).unwrap();
    let sha = out.out.split("sha256=").nth(1).unwrap()[..64].to_string();
    let mut b = live();
    run(&mut b, CANVAS);
    run(&mut b, &format!("k = receive(\"{}/key.surface\", {{after={}, sha256=\"{sha}\"}})\nuse(k)", dir.display(), 3 * 24 * 60));
    assert!(plate(&b) == sent, "the plate arrives as it left (its ink aged when it is next worked)");
    let w = run(&mut b, "t = tarlatan{}\nw = t:wipe(k, {{0, 375}, {1000, 375}}, {passes=1})\nprint(w.taken)");
    assert_eq!(w.trim(), "0.0", "three days on, the ink has set: {w}");
    let got = plate(&b);
    assert!(got.ink_set(100, 100) || got.ink_cell(100, 100).total() == 0.0);
    // the same plate a minute later still wipes
    let mut c = live();
    run(&mut c, CANVAS);
    run(&mut c, &format!("k = receive(\"{}/key.surface\", {{after=1, sha256=\"{sha}\"}})\nuse(k)", dir.display()));
    let w = run(&mut c, "t = tarlatan{}\nw = t:wipe(k, {{0, 375}, {1000, 375}}, {passes=1})\nprint(w.taken > 0)");
    assert_eq!(w.trim(), "true");
    // a session without the switch refuses an inked plate
    let mut nb = box_for(None).unwrap();
    nb.press = false;
    let mut d = Session::with_box(W, nb).unwrap().with_paper(true).unwrap();
    run(&mut d, CANVAS);
    let e = d.run(&format!("receive(\"{}/key.surface\", {{after=0, sha256=\"{sha}\"}})", dir.display())).unwrap_err();
    assert!(e.contains("no ink"), "{e}");
    let _ = std::fs::remove_dir_all(&abs);
}

/// Without the switch nothing changes: a plate drawn and a rag wiped on the painting give the
/// same bytes and the same state digest with the switch and without it; the logs differ by its
/// line alone; an old log has no `ink` or `tarlatan` and refuses the plate's printer's verbs.
#[test]
fn without_the_switch_byte_identical() {
    let mut on = live();
    let mut off_box = box_for(None).unwrap();
    off_box.press = false;
    let mut off = Session::with_box(W, off_box).unwrap().with_paper(true).unwrap();
    for s in [&mut on, &mut off] {
        run(s, CANVAS);
        run(s, "r = rag()\nr:wipe({{100, 100}, {900, 120}}, {pressure=0.6})");
        run(s, PLATE);
        run(s, "wait(10)\nuse(\"painting\")");
    }
    let state = |s: &Session| {
        let mut b = Vec::new();
        crate::surfaces::write_all(&s.st.borrow(), "", &mut b).unwrap();
        b
    };
    assert!(state(&on) == state(&off), "every surface as before");
    assert_eq!(crate::state_digest_line(&on, 1, 0.0), crate::state_digest_line(&off, 1, 0.0), "the rag and the studio as before");
    let (lon, loff) = (on.program("x"), off.program("x"));
    assert!(lon.contains(PRESS_MARK) && !loff.contains(PRESS_MARK));
    assert_eq!(lon.replace(&format!("{PRESS_MARK}\n"), ""), loff);
    let r = replay(&loff).unwrap();
    assert!(plates(&r) == plates(&off));
    assert!(r.lua.globals().get::<mlua::Value>("ink").unwrap().is_nil());
    assert!(r.lua.globals().get::<mlua::Value>("tarlatan").unwrap().is_nil());
    let mut r = r;
    let e = r.run("use(k)\nk:warm(40)").unwrap_err();
    assert!(e.contains("no ink or press"), "{e}");
}

/// The switch comes from the log's head, before the first chunk, and builds on plates and damp
/// paper; the program writes it back.
#[test]
fn the_switch_comes_from_the_log_head() {
    assert!(box_for(None).unwrap().press);
    let head = format!("--@ engine 9\n--@ paper\n{SURFACES_MARK}\n{PLATE_MARK}\n{DAMP_PAPER_MARK}\n");
    assert!(!box_for(Some(&head)).unwrap().press);
    assert!(box_for(Some(&format!("{head}{PRESS_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().press);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{PRESS_MARK}\n"))).unwrap().press, "only before the first chunk");
    let mut s = live();
    run(&mut s, CANVAS);
    let log = s.program("x");
    let r = replay(&log).unwrap();
    assert!(r.st.borrow().tubes.press);
    assert_eq!(r.program("x"), log);
    // without damp paper it is accepted and installs nothing
    let mut b = box_for(None).unwrap();
    b.damp_paper = false;
    let mut bare = Session::with_box(W, b).unwrap().with_paper(true).unwrap();
    run(&mut bare, CANVAS);
    assert!(bare.lua.globals().get::<mlua::Value>("ink").unwrap().is_nil());
    assert!(!bare.lua.globals().get::<mlua::Value>("plate").unwrap().is_nil());
}

/// An inked plate refuses the plate's other tools until it is cleaned.
#[test]
fn an_inked_plate_is_cleaned_before_it_is_worked() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let d = darkest(&s);
    run(&mut s, &format!("black = ink{{{{\"{d}\", 1}}}}\nk:ink(black, {{{{0, 375}}, {{1000, 375}}}})"));
    let e = s.run("k:drypoint(n, {{100, 700}, {900, 700}})").unwrap_err();
    assert!(e.contains("inked"), "{e}");
    let e = s.run("k:ground(\"hard\", {{0, 375}, {1000, 375}})").unwrap_err();
    assert!(e.contains("inked"), "{e}");
    run(&mut s, "c = k:clean_ink()\nassert(c.taken > 0)\nk:drypoint(n, {{100, 700}, {900, 700}})");
}

/// A plate lying in its acid (`--@ etch`'s `k:immerse{}`) takes no ink, warmth or tarlatan until
/// it is lifted out; an inked plate doesn't go into the bath; a plate bitten on the clock,
/// cleaned and inked comes back whole from a save.
#[test]
fn a_plate_in_its_bath_is_lifted_before_it_is_inked() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let d = darkest(&s);
    run(&mut s, &format!("black = ink{{{{\"{d}\", 1}}}}\nt = tarlatan{{}}\nk:ground(\"hard\", {{{{0, 375}}, {{1000, 375}}}})\nk:needle(needle{{tip=40, cone=30}}, {{{{100, 700}}, {{900, 700}}}})\nk:immerse{{acid=\"ferric\", baume=36}}\nwait(5)"));
    for src in ["k:warm(40)", "k:ink(black)", "t:wipe(k)", "k:hand_wipe()", "k:retrousse()", "k:clean_ink()"] {
        let e = s.run(src).unwrap_err();
        assert!(e.contains("in its ferric bath") && e.contains("k:lift()"), "{src}: {e}");
    }
    run(&mut s, "k:lift()\nk:clean()\nk:warm(40)\nk:ink(black, {{0, 375}, {1000, 375}})\nt:wipe(k, {{0, 375}, {1000, 375}}, {passes=2})");
    let e = s.run("k:immerse{acid=\"ferric\", baume=36}").unwrap_err();
    assert!(e.contains("inked"), "{e}");
    let p = plate(&s);
    assert!(p.inked() && p.coats().is_some() && p.in_bath().is_none());
    let log = s.program("press");
    let dir = std::path::PathBuf::from(format!("out/easel/ink-bath-test-{}", std::process::id()));
    let abs = crate::session::root().join(&dir);
    let _ = std::fs::remove_dir_all(&abs);
    std::fs::create_dir_all(&abs).unwrap();
    let path = abs.join("live.ckpt");
    assert!(crate::save::write(&s, &log, &path).unwrap());
    let back = crate::save::read(&path).unwrap();
    assert!(plates(&back.session) == plates(&s), "an etched, inked plate through a save");
    assert!(plates(&replay(&log).unwrap()) == plates(&s), "and through a replay");
    let _ = std::fs::remove_dir_all(&abs);
}

/// Inking and wiping take no region: a mask, an angle, bare options or nothing is refused with
/// words saying to give the points the tool is moved along; the card inks only the band it is
/// dragged over; a poupée inks the disc it is dabbed on.
#[test]
fn ink_and_wipes_go_where_the_points_go() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let d = darkest(&s);
    run(&mut s, &format!("black = ink{{{{\"{d}\", 1}}}}\nt = tarlatan{{}}"));
    for (src, words) in [
        ("k:ink(black)", "no points"),
        ("k:ink(black, {by=\"dabber\"})", "options but no points"),
        ("k:ink(black, {mask=rect(0, 0, 500, 750)})", "takes no mask"),
        ("k:ink(black, rect(0, 0, 500, 750))", "no mask or region"),
        ("k:ink(black, function(x, y) return 1 end)", "no function"),
    ] {
        let e = s.run(src).unwrap_err();
        assert!(e.contains(words) && e.contains("points it is moved along") || e.contains(words) && e.contains("give the points"), "{src}: {e}");
    }
    // a card 10 mm long dragged along the top: only that band takes ink
    run(&mut s, "r = k:ink(black, {{0, 125}, {1000, 125}}, {width=10})\nassert(r.area > 0)");
    let p = plate(&s);
    let c = |mm: f64| (mm * 1000.0 / p.pitch_um()) as usize;
    assert!(p.ink_cell(c(20.0), c(5.0)).total() > 0.0 && p.ink_cell(c(20.0), c(20.0)).total() == 0.0);
    for (src, words) in [
        ("t:wipe(k)", "no points"),
        ("t:wipe(k, nil, {passes=3})", "no points"),
        ("t:wipe(k, rect(0, 0, 1000, 750))", "no mask or region"),
        ("t:wipe(k, {{0, 125}, {1000, 125}}, {angle=90})", "no angle"),
        ("k:hand_wipe(nil, {passes=2})", "no points"),
        ("k:hand_wipe({mask=rect(0, 0, 1000, 750)})", "options but no points"),
        ("k:retrousse(rect(0, 250, 1000, 100))", "no mask or region"),
    ] {
        let e = s.run(src).unwrap_err();
        assert!(e.contains(words), "{src}: {e}");
    }
    run(&mut s, "k:clean_ink()");
    // a poupée dabbed at the middle inks its face's disc (20 mm) and not the corner
    run(&mut s, "k:ink(black, {{500, 375}}, {by=\"poupee\"})");
    let p = plate(&s);
    assert!(p.ink_cell(c(20.0), c(15.0)).total() > 0.0 && p.ink_cell(c(2.0), c(2.0)).total() == 0.0);
}
