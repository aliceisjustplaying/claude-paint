//! Tiny scenes (64 px) for damp printing paper (`--@ damp-paper`). Expected numbers come from
//! paint's damp formulas (tested against measurements there) and the evaporation law, never
//! from a run of the easel itself.

use super::*;
use crate::session::{DAMP_PAPER_MARK, Session, box_for, parse_program};
use crate::surfaces::MAIN;
use paint::Canvas;
use std::path::PathBuf;

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const PROOF: &str = r#"pr = sheet{name="proof", size=60, aspect=1, paper={grammage=250}, seed=3}"#;

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

fn state(c: &Canvas) -> Vec<u8> {
    let mut b = Vec::new();
    c.write_state(&mut b, "").unwrap();
    b
}

/// A surface's canvas as it is (current) or rests.
fn canvas(s: &Session, name: &str) -> Canvas {
    let st = s.st.borrow();
    if st.surfaces.current_name() == name {
        return st.canvas.as_ref().unwrap().clone();
    }
    let i = st.surfaces.find(name).unwrap();
    st.surfaces.slots[i].rest.as_ref().unwrap().canvas.as_ref().unwrap().restore().unwrap()
}

fn clock(s: &Session) -> f64 {
    s.st.borrow().clock
}

/// A number a chunk printed as `name=<value>`.
fn printed(out: &str, name: &str) -> f64 {
    let at = out.find(&format!("{name}=")).unwrap_or_else(|| panic!("no {name}= in {out}"));
    out[at + name.len() + 1..].split_whitespace().next().unwrap().parse().unwrap()
}

struct Folder(PathBuf, String);

impl Folder {
    fn new(name: &str) -> Self {
        let rel = format!("out/easel/damp-test-{}-{name}", std::process::id());
        let abs = crate::session::root().join(&rel);
        let _ = std::fs::remove_dir_all(&abs);
        Folder(abs, rel)
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The switch comes from the log's head: a new painting has it, a log has it only if its head
/// names it, and the program writes it back; without paper it installs nothing.
#[test]
fn the_switch_comes_from_the_log_head() {
    assert!(box_for(None).unwrap().damp_paper);
    let head = "--@ engine 9\n--@ paper\n--@ surfaces\n";
    assert!(!box_for(Some(head)).unwrap().damp_paper);
    assert!(box_for(Some(&format!("{head}{DAMP_PAPER_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().damp_paper);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{DAMP_PAPER_MARK}\n"))).unwrap().damp_paper, "only before the first chunk");
    let mut s = live();
    run(&mut s, CANVAS);
    let log = s.program("x");
    assert!(log.lines().any(|l| l == DAMP_PAPER_MARK));
    let r = replay(&log).unwrap();
    assert!(r.st.borrow().tubes.damp_paper);
    assert_eq!(r.program("x"), log);
    let mut b = box_for(None).unwrap();
    b.damp_paper = true;
    let bare = Session::with_box(W, b).unwrap();
    assert!(bare.lua.globals().get::<Value>("soak").unwrap().is_nil());
}

/// Without the switch a session is byte-identical: a painting and a sheet beside it drawn the
/// same with and without it come out bit for bit, its log differs by the switch's line only, a
/// log without it has none of the verbs, and an older log with a verb of the switch fails.
#[test]
fn without_the_switch_it_is_byte_identical() {
    let mut on = live();
    let mut off_box = box_for(None).unwrap();
    off_box.damp_paper = false;
    let mut off = Session::with_box(W, off_box).unwrap().with_paper(true).unwrap();
    for s in [&mut on, &mut off] {
        run(s, CANVAS);
        run(s, &format!("{PROOF}\nuse(pr)\nh = pencil(\"2B\")\nh:line({{{{100,800}},{{800,800}}}})\nwait(30)\nuse(\"painting\")\nwait(5)"));
    }
    for name in [MAIN, "proof"] {
        assert!(state(&canvas(&on, name)) == state(&canvas(&off, name)), "{name}");
    }
    let (lon, loff) = (on.program("x"), off.program("x"));
    assert_eq!(lon.replace(&format!("{DAMP_PAPER_MARK}\n"), ""), loff);
    for verb in ["soak", "blot", "pack", "cover", "uncover", "damp_of"] {
        assert!(off.lua.globals().get::<Value>(verb).unwrap().is_nil(), "{verb}");
        assert!(!on.lua.globals().get::<Value>(verb).unwrap().is_nil(), "{verb}");
    }
    let e = replay(&format!("{loff}\n--@ chunk 3\nsoak(pr, {{minutes=5}})\n")).err().unwrap();
    assert!(e.contains("soak"), "{e}");
}

/// Soaking, draining and blotting a sheet put aside: it is taken up for each verb and put
/// aside again (the painting stays current), the soak and the drain pass on the clock for
/// every surface, and its water is what paint's damp formulas give for its paper. `damp_of` on
/// it changes nothing; its log replays to the same sheet.
#[test]
fn soak_drain_and_blot_a_sheet_beside_the_painting() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PROOF);
    let t0 = clock(&s);
    let out = run(&mut s, "mc = soak(pr, {minutes=60, drain=30})\nprint(\"soaked=\" .. mc)");
    assert_eq!(s.st.borrow().surfaces.current_name(), MAIN);
    assert!((clock(&s) - t0 - 60.5).abs() < 1e-6, "the soak and the drain: {}", clock(&s) - t0);
    let c = canvas(&s, "proof");
    let p = *c.paper().unwrap();
    let mut want = paint::damp::Damp::new(&p);
    let h = c.height() as f64 * 60.0 / 1000.0 * 1e-3;
    want.soak(&p, 60.0, 30.0, h);
    let got = c.damp().unwrap();
    assert!((got.water - want.water).abs() < 1e-6 * want.water, "{} vs {}", got.water, want.water);
    assert!((printed(&out, "soaked") - want.mc(&p)).abs() < 1e-9);

    let out = run(&mut s, "mc = blot(pr)\nprint(\"blotted=\" .. mc)\nlocal m, w, t = damp_of(pr)\nprint(\"word=\" .. w .. \" mc=\" .. m .. \" width=\" .. t.width)");
    want.blot(&p, 1, paint::damp::GB003);
    // (the blot's hand time dries it a little on the bench, at the wet surface's rate)
    want.dry(&p, 20.0 / 60.0);
    assert!((printed(&out, "mc") - want.mc(&p)).abs() < 1e-9, "{out}");
    assert!(out.contains("word=damp"), "{out}");
    // its size: the walls' saturation across the machine direction (its width)
    let (_, cd) = paint::damp::hygroexpansion(&p);
    let grown = 60.0 * (1.0 + cd * (paint::damp::fiber_saturation() - paint::damp::isotherm(paint::damp::ISO_RH)));
    assert!((printed(&out, "width") - grown).abs() < 1e-3, "{out}");

    let before = state(&canvas(&s, "proof"));
    run(&mut s, "damp_of(\"proof\")");
    assert!(state(&canvas(&s, "proof")) == before, "a look changes nothing");
    let r = replay(&s.program("x")).unwrap();
    assert!(state(&canvas(&r, "proof")) == state(&canvas(&s, "proof")));
    assert!(state(&canvas(&r, MAIN)) == state(&canvas(&s, MAIN)));
}

/// Two sheets packed with two dry ones come to one moisture content, the water conserved, and
/// stay covered: they keep it while the hours pass.
#[test]
fn a_pack_equalises_its_sheets() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, &format!("{PROOF}\npr2 = sheet{{name=\"proof2\", size=60, aspect=1, paper={{grammage=250}}, seed=4}}\nsoak(pr, {{minutes=30}})"));
    let w = |s: &Session, n: &str| canvas(s, n).damp().map(|d| d.water);
    let p = *canvas(&s, "proof").paper().unwrap();
    let total = w(&s, "proof").unwrap() + paint::damp::dry_fiber(&p) * paint::damp::room_moisture() * 3.0;
    let out = run(&mut s, "local m = pack({pr, \"proof2\"}, {hours=10, dry=2})\nprint(\"a=\" .. m[1] .. \" b=\" .. m[2])");
    let want = total / 4.0 / paint::damp::dry_fiber(&p);
    assert!((printed(&out, "a") - want).abs() < 1e-9 && (printed(&out, "b") - want).abs() < 1e-9, "{out} vs {want}");
    let (a, b) = (canvas(&s, "proof").damp().unwrap(), canvas(&s, "proof2").damp().unwrap());
    assert_eq!((a.faces, b.faces), (0, 0), "covered in the pack");
    assert!((a.water / paint::damp::dry_fiber(&p) - want).abs() < 1e-9);
    assert!(clock(&s) >= 600.0, "the night passed");
    // the pack's own sheets only, each once
    assert!(s.run("pack({pr, pr}, {hours=1})").unwrap_err().contains("twice"));
}

/// A damp sheet handed off arrives drier by its time in transit (plan-intaglio I6): lying
/// uncovered it loses one face's evaporation, `evaporation_at(1)` µm a minute, for the
/// minutes on the way; covered it arrives as it left. Its damp state goes in the handed-off
/// file and in a save.
#[test]
fn a_damp_sheet_handed_off_arrives_drier_by_its_transit_time() {
    let dir = Folder::new("handoff");
    for (covered, file) in [(false, "open.surface"), (true, "covered.surface")] {
        let mut s = live();
        run(&mut s, CANVAS);
        run(&mut s, &format!("{PROOF}\nsoak(pr, {{minutes=60}})\nblot(pr)\n{}", if covered { "cover(pr)" } else { "" }));
        let sent = canvas(&s, "proof").damp().unwrap();
        let path = format!("{}/{file}", dir.1);
        let out = run(&mut s, &format!("handoff(pr, {path:?})"));
        let sha = out.split("sha256=").nth(1).unwrap()[..64].to_string();
        let mut r = live();
        run(&mut r, CANVAS);
        let after = 20.0;
        let out = run(&mut r, &format!("k = receive({path:?}, {{after={after}, sha256={sha:?}}})\nlocal m = damp_of(k)\nprint(\"mc=\" .. m)"));
        let got = canvas(&r, "proof");
        let p = *got.paper().unwrap();
        let fiber = paint::damp::dry_fiber(&p);
        let want = if covered { sent.water } else { sent.water - after * paint::damp::evaporation_at(1.0) };
        assert!(want > fiber * paint::damp::fiber_saturation(), "still past saturation: the free water's constant rate");
        assert!((printed(&out, "mc") * fiber - want).abs() < 1e-6, "covered={covered}: {} vs {want}", printed(&out, "mc") * fiber);

        // a save of the receiver keeps the damp sheet
        let save = dir.0.join(format!("{file}.ckpt"));
        let log = r.program("printer");
        assert!(crate::save::write(&r, &log, &save).unwrap());
        let back = crate::save::read(&save).unwrap();
        assert!(back.session.st.borrow().tubes.damp_paper);
        assert_eq!(canvas(&back.session, "proof").damp(), got.damp());
    }
}

/// Only a blank sheet of paper is damped: the linen painting isn't paper, and a drawn sheet
/// would lose its drawing in the tray.
#[test]
fn only_a_blank_sheet_is_damped() {
    let mut s = live();
    run(&mut s, CANVAS);
    let e = s.run("soak(\"painting\", {minutes=5})").unwrap_err();
    assert!(e.contains("not a sheet of paper"), "{e}");
    run(&mut s, &format!("{PROOF}\nuse(pr)\nh = pencil(\"2B\")\nh:line({{{{100,800}},{{800,800}}}})\nuse(\"painting\")"));
    let e = s.run("soak(pr, {minutes=5})").unwrap_err();
    assert!(e.contains("drawn on"), "{e}");
    let e = s.run("soak(\"nothing\", {minutes=5})").unwrap_err();
    assert!(e.contains("no surface"), "{e}");
}

/// A replay checkpoint taken after a soak resumes to the same surfaces, and the log goes on
/// through a blot, a pack and a look at a resting sheet as the painting went.
#[test]
fn a_replay_checkpoint_across_damp_verbs() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, &format!("{PROOF}\nsoak(pr, {{minutes=20}})"));
    run(&mut s, "pr2 = sheet{name=\"proof2\", size=60, aspect=1, paper={grammage=250}, seed=4}\nblot(pr)\nprint(damp_of(pr))");
    run(&mut s, "pack({pr, pr2}, {hours=8})\nuncover(pr2, {faces=2})\nwait(30)\nprint(damp_of(\"proof2\"))");
    let log = s.program("x");
    let dir = Folder::new("replay");
    std::fs::create_dir_all(&dir.0).unwrap();
    let ck = dir.0.join("replay.ckpt");
    let chunks = parse_program(&log);
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string(), "--until-chunk".into(), "2".into()];
    let mut first = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    let mut batch = crate::replay_checkpoint::Batch::open(&args, &log, W, &mut first, &chunks, &dir.0.join("out.png")).unwrap();
    for (i, c) in chunks.iter().enumerate() {
        first.run(c).unwrap();
        if batch.completed(&first, i + 1).unwrap() {
            break;
        }
    }
    drop(batch);
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string()];
    let mut resumed = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    let batch = crate::replay_checkpoint::Batch::open(&args, &log, W, &mut resumed, &chunks, &dir.0.join("out.png")).unwrap();
    assert_eq!(batch.done, 2);
    assert_eq!(canvas(&resumed, "proof").damp(), canvas(&first, "proof").damp());
    for c in &chunks[2..] {
        resumed.run(c).unwrap();
    }
    drop(batch);
    for name in [MAIN, "proof", "proof2"] {
        assert!(state(&canvas(&resumed, name)) == state(&canvas(&s, name)), "{name}");
    }
}
