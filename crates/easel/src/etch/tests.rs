//! Tiny plates (40 × 30 mm beside a 64 px painting) for `--@ etch`.

use crate::session::{ETCH_MARK, PLATE_MARK, SURFACES_MARK, Session, box_for, parse_program};

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const PLATE: &str = r#"k = plate{name="key", metal="copper", temper="half-hard", size={40, 30}}
n = needle{tip=40, cone=30}"#;
/// A line plate: ground, two needle lines, a bite, the lower line stopped out, a second bite,
/// cleaned; then an aquatint, melted and bitten.
const ETCH: &str = r#"use(k)
k:ground("hard", {{0, 375}, {1000, 375}}, {um=4})
r = k:needle(n, {{100, 200}, {900, 200}})
assert(r.touched)
k:needle(n, {{100, 500}, {900, 500}})
b = k:bite{acid="ferric", baume=36, minutes=10}
k:stop_out({{60, 500}, {940, 500}}, {pressure=0.85})
k:bite{acid="ferric", baume=36, minutes=20, rock=true}
k:clean()
a = k:aquatint{box=0.6, wait=30, minutes=1}
k:melt()
k:bite{acid="ferric", baume=36, minutes=3}"#;

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

fn run(s: &mut Session, src: &str) {
    if let Err(e) = s.run(src) {
        panic!("{src}\n→ {e}");
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

struct Folder(std::path::PathBuf, String);

impl Folder {
    fn new(name: &str) -> Self {
        let rel = format!("out/easel/etch-test-{}-{name}", std::process::id());
        let abs = crate::session::root().join(&rel);
        let _ = std::fs::remove_dir_all(&abs);
        std::fs::create_dir_all(&abs).unwrap();
        Folder(abs, rel)
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The plate etched: the bites' minutes pass on the shared clock; the lines are bitten, the
/// stopped-out one less; the metal balances; the fingernail and probe find the coats; it looks.
#[test]
fn a_plate_is_etched_on_the_clock() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let t0 = s.st.borrow().clock;
    run(&mut s, ETCH);
    let t1 = s.st.borrow().clock;
    assert!(t1 - t0 >= 10.0 + 20.0 + 0.5 + 1.0 + 3.0, "the bath's and the box's minutes: {t0} → {t1}");
    let p = plate(&s);
    assert!(p.bitten_um3() > 0.0);
    assert!((p.metal_um3() + p.chip_um3 + p.bitten_um3()).abs() < 1e-3 * p.bitten_um3(), "the metal balances");
    let depth = |p: &paint::plate::Plate, y_mm: f64| p.cell_at_mm(20.0, y_mm).unwrap().mean_depth_width().0;
    // (the upper line at 8 mm bit 30 minutes, the lower at 20 mm 10, then both 3 more under the aquatint)
    assert!(depth(&p, 8.0) > 1.5 * depth(&p, 20.0), "{} {}", depth(&p, 8.0), depth(&p, 20.0));
    let out = s.run("print(k:feel(500, 750).words)").unwrap();
    assert!(out.out.contains("melted rosin"), "{}", out.out);
    let e = s.run("k:ground(\"hard\", {{0, 375}, {1000, 375}})").unwrap_err();
    assert!(e.contains("clean it first"), "{e}");
    {
        let st = s.st.borrow();
        let (c, pl) = (st.canvas.as_ref().unwrap(), st.plate.as_ref().unwrap());
        let text = crate::plate::probe(pl, c.clock(), "at 500,200", (13, 5, 14, 6));
        assert!(text.contains("rosin over") && text.contains("bitten up to") && text.contains("its tray"), "{text}");
        for v in [crate::look::View::default(), crate::look::View { crop: Some([400.0, 150.0, 600.0, 250.0]), light: Some((135.0, 25.0)), ..Default::default() }] {
            let (w, h, png) = crate::plate::render(c, pl, &v).unwrap();
            assert!(w > 0 && h > 0 && !png.is_empty());
        }
    }
    // the drypoint tools want a clean plate; the needle a ground; a feather nitric
    let e = s.run("k:drypoint(n, {{100,100},{900,100}})").unwrap_err();
    assert!(e.contains("clean it first"), "{e}");
    run(&mut s, "k:clean()");
    let e = s.run("k:needle(n, {{100,100},{900,100}})").unwrap_err();
    assert!(e.contains("lay one first"), "{e}");
    let e = s.run("k:bite{acid=\"ferric\", minutes=1, feather=true}").unwrap_err();
    assert!(e.contains("bubbles"), "{e}");
    let e = s.run("k:bite{acid=\"aqua regia\", minutes=1}").unwrap_err();
    assert!(e.contains("ferric"), "{e}");
    run(&mut s, "k:bite{acid=\"nitric\", parts=8, minutes=1, feather=true}\nk:bite{acid=\"dutch\", minutes=1}");
}

/// A replay gives the etched plate again bit for bit; a failed chunk puts it back; a save and a
/// replay checkpoint keep it; a handoff carries it, and a session that doesn't etch refuses it.
#[test]
fn an_etched_plate_survives_replay_save_checkpoint_and_handoff() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    run(&mut s, ETCH);
    run(&mut s, "k:bite{acid=\"ferric\", minutes=2}\nwait(5)");
    let log = s.program("etch");
    assert!(log.lines().any(|l| l == ETCH_MARK));
    let r = replay(&log).unwrap();
    assert!(plates(&r) == plates(&s), "the replay's plate is the painting's");

    let before = plates(&s);
    let e = s.run("k:bite{acid=\"ferric\", minutes=5}\nerror(\"no\")").unwrap_err();
    assert!(e.contains("no"));
    assert!(plates(&s) == before, "a failed chunk puts the plate back");

    let dir = Folder::new("save");
    let path = dir.0.join("live.ckpt");
    assert!(crate::save::write(&s, &log, &path).unwrap());
    let back = crate::save::read(&path).unwrap();
    assert!(plates(&back.session) == plates(&s), "a save round trip keeps the plate");
    assert!(back.session.st.borrow().tubes.etch);

    // a replay checkpoint after the third chunk (the plate etched), resumed, then the rest
    let ck = dir.0.join("replay.ckpt");
    let chunks = parse_program(&log);
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string(), "--until-chunk".into(), "3".into()];
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
    assert_eq!(batch.done, 3);
    assert!(plates(&resumed) == plates(&first));
    for c in &chunks[3..] {
        resumed.run(c).unwrap();
    }
    drop(batch);
    assert!(plates(&resumed) == plates(&r), "and on, as the painting went");

    // handed off and received, then bitten on in the other session
    let sent = plate(&s);
    let out = s.run(&format!("handoff(k, \"{}/key.surface\")", dir.1)).unwrap();
    let sha = out.out.split("sha256=").nth(1).unwrap()[..64].to_string();
    let mut b = live();
    run(&mut b, CANVAS);
    run(&mut b, &format!("k = receive(\"{}/key.surface\", {{after=60, sha256=\"{sha}\"}})\nuse(k)", dir.1));
    assert!(plate(&b) == sent, "the plate arrives as it left, with its coats and tray");
    run(&mut b, "k:bite{acid=\"ferric\", baume=36, minutes=1}");
    let mut nb = box_for(None).unwrap();
    nb.etch = false;
    let mut c = Session::with_box(W, nb).unwrap().with_paper(true).unwrap();
    run(&mut c, CANVAS);
    let e = c.run(&format!("receive(\"{}/key.surface\", {{after=0, sha256=\"{sha}\"}})", dir.1)).unwrap_err();
    assert!(e.contains("doesn't etch"), "{e}");
}

/// Without the switch nothing changes: a drypoint plate made with and without it is the same
/// bytes (a plate no etching tool touched keeps version 1's layout, inside version 4's contact
/// word when the painting has the contact laws) and so is every surface; the logs differ by
/// the switch's line alone; a log without it refuses the etching tools.
#[test]
fn without_the_switch_byte_identical() {
    let mut on = live();
    let mut off_box = box_for(None).unwrap();
    off_box.etch = false;
    let mut off = Session::with_box(W, off_box).unwrap().with_paper(true).unwrap();
    for s in [&mut on, &mut off] {
        run(s, CANVAS);
        run(s, PLATE);
        run(s, "use(k)\nk:drypoint(n, {{100, 300}, {900, 300}}, {force=2})\nk:burnish({{300, 300}, {600, 300}})\nwait(10)\nuse(\"painting\")");
    }
    let state = |s: &Session| {
        let mut b = Vec::new();
        crate::surfaces::write_all(&s.st.borrow(), "", &mut b).unwrap();
        b
    };
    assert!(state(&on) == state(&off), "every surface as before");
    let p = plates(&on);
    let word = |at: usize| u64::from_le_bytes(p[0].1[at..at + 8].try_into().unwrap());
    // a plate under the contact laws writes version 4, its laws, then the layout it would have
    let layout = if word(4) == paint::plate::PLAT_V { word(20) } else { word(4) };
    assert_eq!(layout, 1, "a drypoint plate no etching tool touched is laid out as version 1");
    let (lon, loff) = (on.program("x"), off.program("x"));
    assert!(lon.contains(ETCH_MARK) && !loff.contains(ETCH_MARK));
    assert_eq!(lon.replace(&format!("{ETCH_MARK}\n"), ""), loff);
    let dir = Folder::new("alone");
    let (pon, poff) = (dir.0.join("on.ckpt"), dir.0.join("off.ckpt"));
    crate::save::write(&on, &lon, &pon).unwrap();
    crate::save::write(&off, &lon, &poff).unwrap();
    let read = |p: &std::path::Path| crate::surfaces::read_all(&mut std::fs::File::open(p).unwrap()).unwrap();
    let (ron, roff) = (read(&pon), read(&poff));
    assert_eq!(ron.header.replace("etch=1\n", ""), roff.header, "the save's header but its switch line");
    let rest = |p: &std::path::Path, h: &str| std::fs::read(p).unwrap().len() - h.len();
    assert_eq!(rest(&pon, &ron.header), rest(&poff, &roff.header));
    // the old log refuses the etching tools; it replays as it was
    let r = replay(&loff).unwrap();
    assert!(plates(&r) == plates(&off));
    let mut r = r;
    let e = r.run("use(k)\nk:ground(\"hard\", {{0, 375}, {1000, 375}})").unwrap_err();
    assert!(e.contains("doesn't etch"), "{e}");
}

/// The switch comes from the log's head, before the first chunk, and builds on plates; the
/// program writes it back.
#[test]
fn the_switch_comes_from_the_log_head() {
    assert!(box_for(None).unwrap().etch);
    let head = format!("--@ engine 9\n--@ paper\n{SURFACES_MARK}\n{PLATE_MARK}\n");
    assert!(!box_for(Some(&head)).unwrap().etch);
    assert!(box_for(Some(&format!("{head}{ETCH_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().etch);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{ETCH_MARK}\n"))).unwrap().etch, "only before the first chunk");
    let mut s = live();
    run(&mut s, CANVAS);
    let log = s.program("x");
    let r = replay(&log).unwrap();
    assert!(r.st.borrow().tubes.etch);
    assert_eq!(r.program("x"), log);
    // without plates it is accepted and installs nothing: there is no plate to etch
    let mut b = box_for(None).unwrap();
    b.plate = false;
    let mut bare = Session::with_box(W, b).unwrap().with_paper(true).unwrap();
    run(&mut bare, CANVAS);
    assert!(bare.lua.globals().get::<mlua::Value>("plate").unwrap().is_nil());
    let e = bare.run("st = sheet{name=\"s\", size=60, aspect=1, paper={grammage=200}, seed=3}\nuse(st)\nst:ground(\"hard\")").unwrap_err();
    assert!(e.contains("plate"), "{e}");
}

/// A grounded plate with two needle lines, in hand.
const LINES: &str = r#"use(k)
k:ground("hard", {{0, 375}, {1000, 375}}, {um=4})
k:needle(n, {{100, 200}, {900, 200}})
k:needle(n, {{100, 500}, {900, 500}})"#;
const STUDY: &str = r#"st = sheet{name="study", size=60, aspect=1, paper={grammage=200}, seed=3}"#;

fn lined() -> Session {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    run(&mut s, STUDY);
    run(&mut s, LINES);
    s
}

/// Into the bath, other work for 15 minutes (a pencil drawing on the study, a wait at the
/// painting, a wait at the plate), out: the plate of a 15-minute bite in one, bit for bit; and
/// with hand work that leaves odd minutes, the bite in one of the minutes the lift reports.
#[test]
fn in_other_work_out_is_the_bite_in_one() {
    let mut one = lined();
    run(&mut one, "k:bite{acid=\"ferric\", baume=36, minutes=15}");
    let mut s = lined();
    let t0 = s.st.borrow().clock;
    run(&mut s, "k:immerse{acid=\"ferric\", baume=36}\nuse(st)\nh = pencil(\"2B\")\nh:line({{100,100},{500,500}})");
    let drawn = s.st.borrow().clock - t0;
    assert!(drawn > 0.0, "the drawing took hand time");
    run(&mut s, &format!("use(\"painting\")\nwait({})\nuse(k)\nwait(5)\nr = k:lift()", 10.0 - drawn));
    let got: f64 = s.lua.globals().get::<mlua::Table>("r").unwrap().get("minutes").unwrap();
    assert!((got - 15.0).abs() < 1e-4, "{got} min in the bath");
    // (the drawing's odd seconds: the minutes are read to the millisecond)
    let mut odd = lined();
    run(&mut odd, &format!("k:bite{{acid=\"ferric\", baume=36, minutes={got}}}"));
    assert!(plates(&s) == plates(&odd), "in, a drawing and waits, out: the bite in one of {got} min");

    let mut w = lined();
    run(&mut w, "k:immerse{acid=\"ferric\", baume=36}\nuse(st)\nwait(6)\nuse(\"painting\")\nwait(4)\nuse(k)\nwait(5)\nk:lift()");
    assert!(plates(&w) == plates(&one), "in, waits at three surfaces, out: the 15-minute bite");
    let (ct, cw) = (one.st.borrow().clock, w.st.borrow().clock);
    assert!((ct - cw).abs() < 1e-9, "the same time on the clock: {ct} {cw}");
}

/// A plate still in its bath through a save and its reading, a replay of the log, a replay
/// checkpoint, and a handoff (it travels in its tray, biting on the way) comes out with the bite
/// it would have had without them.
#[test]
fn a_bite_across_a_save_reload_and_handoff_is_the_same() {
    let mut one = lined();
    run(&mut one, "k:bite{acid=\"ferric\", baume=36, minutes=15}");
    let mut s = lined();
    run(&mut s, "k:immerse{acid=\"ferric\", baume=36}\nuse(\"painting\")\nwait(4)");
    let status = s.status();
    assert!(status.contains("in the bath: key (ferric, 4 min)"), "{status}");
    let dir = Folder::new("bath");
    let log = s.program("bath");
    let path = dir.0.join("live.ckpt");
    assert!(crate::save::write(&s, &log, &path).unwrap());
    let mut back = crate::save::read(&path).unwrap().session;
    assert!(plates(&back) == plates(&s), "saved in its bath");
    run(&mut s, "wait(6)\nuse(k)\nwait(5)\nk:lift()");
    // (a save keeps no Lua heap, so no `k`: the plate is taken up by name and lifted at its clock)
    run(&mut back, "wait(6)\nuse(\"key\")\nwait(5)");
    {
        let mut st = back.st.borrow_mut();
        let now = st.canvas.as_ref().unwrap().clock();
        std::rc::Rc::make_mut(st.plate.as_mut().unwrap()).lift(now).unwrap();
    }
    assert!(plates(&back) == plates(&one), "lifted after the save was read: the 15-minute bite");
    assert!(plates(&s) == plates(&one));
    // the log replayed, and resumed from a replay checkpoint taken while the plate was in
    let log = s.program("bath");
    let r = replay(&log).unwrap();
    assert!(plates(&r) == plates(&one));
    let chunks = parse_program(&log);
    let n = chunks.len();
    let ck = dir.0.join("replay.ckpt");
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string(), "--until-chunk".into(), (n - 1).to_string()];
    let mut first = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    let mut batch = crate::replay_checkpoint::Batch::open(&args, &log, W, &mut first, &chunks, &dir.0.join("out.png")).unwrap();
    for (i, c) in chunks.iter().enumerate() {
        first.run(c).unwrap();
        if batch.completed(&first, i + 1).unwrap() {
            break;
        }
    }
    drop(batch);
    assert!(plate(&first).in_bath().is_some(), "the checkpoint is taken with the plate in its bath");
    let args = vec!["--replay-checkpoint".to_string(), ck.display().to_string()];
    let mut resumed = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    let batch = crate::replay_checkpoint::Batch::open(&args, &log, W, &mut resumed, &chunks, &dir.0.join("out.png")).unwrap();
    assert_eq!(batch.done, n - 1);
    resumed.run(&chunks[n - 1]).unwrap();
    drop(batch);
    assert!(plates(&resumed) == plates(&one), "resumed in the bath, lifted: the same bite");

    // handed off in the bath 4 minutes in, 5 on the way, lifted 6 minutes after it came
    let mut a = lined();
    run(&mut a, "k:immerse{acid=\"ferric\", baume=36}\nuse(\"painting\")\nwait(4)");
    let out = a.run(&format!("handoff(k, \"{}/key.surface\")", dir.1)).unwrap();
    let sha = out.out.split("sha256=").nth(1).unwrap()[..64].to_string();
    let mut b = live();
    run(&mut b, CANVAS);
    let out = b.run(&format!("k = receive(\"{}/key.surface\", {{after=5, sha256=\"{sha}\"}})", dir.1)).unwrap();
    assert!(out.out.contains("came in its ferric bath"), "{}", out.out);
    run(&mut b, "wait(2)\nuse(k)\nwait(4)\nk:lift()");
    assert!(plates(&b) == plates(&one), "a plate handed off in its bath bit all the way");
}

/// A bath 10 °C warmer bites a bare face 1.40 times as fast (Ea 24.9 kJ/mol); a line stopped out
/// between two baths keeps the depth of the first.
#[test]
fn a_warmer_bath_bites_faster_and_stop_out_keeps_depth() {
    let open = |temp: u32| {
        let mut s = live();
        run(&mut s, CANVAS);
        run(&mut s, PLATE);
        run(&mut s, &format!("use(k)\nk:immerse{{acid=\"ferric\", baume=36, temp={temp}}}\nwait(10)\nk:lift()"));
        -plate(&s).cell(10, 10).level as f64
    };
    let arrh = (24.9e3f64 / 8.314462618 * (1.0 / 293.15 - 1.0 / 303.15)).exp();
    let ratio = open(30) / open(20);
    assert!((ratio / arrh - 1.0).abs() < 1e-3 && (ratio - 1.40).abs() < 0.01, "{ratio} against {arrh}");

    let mut s = lined();
    run(&mut s, "k:immerse{acid=\"ferric\", baume=36}\nwait(10)\nk:lift()\nk:stop_out({{60, 500}, {940, 500}}, {pressure=0.85})");
    // (the lower line at y 500 units, 20 mm down the 40 mm plate, under the varnish; the upper at 8 mm)
    let at = |s: &Session| {
        let p = plate(s);
        let c = |mm: f64| (mm * 1000.0 / p.pitch_um()) as usize;
        (p.cell(c(20.0), c(20.0)), p.etch(c(20.0), c(20.0)), p.etch(c(20.0), c(8.0)))
    };
    let (cell, etched, upper) = at(&s);
    assert!(etched.stopped && etched.lines.iter().any(|l| l.z > 0.0));
    run(&mut s, "k:immerse{acid=\"ferric\", baume=36}\nuse(st)\nwait(20)\nuse(k)\nk:lift()");
    let (cell2, etched2, upper2) = at(&s);
    assert!(cell2 == cell && etched2 == etched, "the stopped-out line keeps its first depth");
    let z = |e: &paint::plate::etched::Etch| e.lines.iter().map(|l| l.z).fold(0.0, f32::max);
    assert!(z(&upper2) > 2.0 * z(&upper), "the open line bit on: {} → {}", z(&upper), z(&upper2));
}

/// In its bath the plate can't be worked, felt or put in again, nor lifted out when it isn't in;
/// a look and a probe see it as it would come out, and leave it in; a one-call bite is refused
/// too.
#[test]
fn a_plate_in_its_bath_refuses_work_and_is_seen_as_it_would_come_out() {
    let mut s = lined();
    let e = s.run("k:lift()").unwrap_err();
    assert!(e.contains("isn't in a bath"), "{e}");
    let e = s.run("k:immerse{acid=\"ferric\", minutes=3}").unwrap_err();
    assert!(e.contains("minutes"), "{e}");
    run(&mut s, "k:immerse{acid=\"ferric\", baume=36}\nwait(3)");
    for (src, verb) in [
        ("k:immerse{acid=\"ferric\"}", "immerse"),
        ("k:bite{acid=\"ferric\", minutes=1}", "bite"),
        ("k:needle(n, {{100,700},{900,700}})", "needle"),
        ("k:stop_out({{100, 100}, {200, 100}})", "stop_out"),
        ("k:clean()", "clean"),
        ("k:feel(500, 200)", "feel"),
        ("k:drypoint(n, {{100,700},{900,700}})", "drypoint"),
    ] {
        let e = s.run(src).unwrap_err();
        assert!(e.contains(&format!("{verb}: the plate \"key\" is in its ferric bath (3 min so far): lift it out first")), "{src}: {e}");
    }
    {
        let st = s.st.borrow();
        let (c, pl) = (st.canvas.as_ref().unwrap(), st.plate.as_ref().unwrap());
        let text = crate::plate::probe(pl, c.clock(), "at 500,200", (13, 5, 14, 6));
        assert!(text.contains("in its bath: ferric") && text.contains("3 min so far"), "{text}");
        let deepest = text.split("bitten up to ").nth(1).unwrap().split(' ').next().unwrap().parse::<f64>().unwrap();
        assert!(deepest > 0.0, "seen bitten so far: {text}");
        let (w, h, png) = crate::plate::render(c, pl, &crate::look::View::default()).unwrap();
        assert!(w > 0 && h > 0 && !png.is_empty());
    }
    assert!(plate(&s).in_bath().is_some(), "a look leaves it in");
    let r = s.run("r = k:lift()\nprint(r.minutes)").unwrap();
    assert_eq!(r.out.trim(), "3.0");
}

/// The hand's tools take no region: stopping-out and the ground refuse a mask, a function, bare
/// options or nothing, and say to give the points the tool is moved along; the varnish lies where
/// the brush went, a brush runs dry, and the ground lies where the roller rolled.
#[test]
fn hand_tools_go_where_the_points_go() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    run(&mut s, "use(k)");
    for (src, words) in [
        ("k:ground(\"hard\", {um=4})", "options but no points"),
        ("k:ground(\"hard\")", "no points"),
        ("k:ground(\"hard\", rect(0, 0, 1000, 750))", "no mask or region"),
        ("k:stop_out(rect(0, 400, 1000, 350))", "no mask or region"),
        ("k:stop_out(function(x, y) return 1 end)", "no function"),
        ("k:stop_out()", "no points"),
    ] {
        let e = s.run(src).unwrap_err();
        assert!(e.contains(words) && e.contains("points it is moved along"), "{src}: {e}");
    }
    // the roller over the top half only: the bottom is bare
    let r = s.run("r = k:ground(\"hard\", {{0, 150}, {1000, 150}}, {roller=100})\nprint(r.covered)").unwrap();
    let covered: f64 = r.out.trim().parse().unwrap();
    // (a 100 mm roller rolled 6 mm down the 30 mm plate reaches its whole height)
    assert!(covered > 99.0, "{covered}");
    run(&mut s, "k:clean()");
    let r = s.run("r = k:ground(\"hard\", {{0, 100}, {1000, 100}}, {roller=20})\nprint(r.covered)").unwrap();
    let covered: f64 = r.out.trim().parse().unwrap();
    // (a 20 mm roller along 4 mm down: 0 to 14 mm of the 30, to a cell of 1.5 mm)
    assert!((covered - 100.0 * 14.0 / 30.0).abs() < 6.0, "{covered}");
    let p = plate(&s);
    let c = |mm: f64| (mm * 1000.0 / p.pitch_um()) as usize;
    assert!(p.grounded(c(20.0), c(10.0)) && !p.grounded(c(20.0), c(20.0)));
    // more of the same ground covers the rest; another thickness is refused
    let e = s.run("k:ground(\"hard\", {{0, 600}, {1000, 600}}, {um=6})").unwrap_err();
    assert!(e.contains("roll more of the same"), "{e}");
    run(&mut s, "k:ground(\"hard\", {{0, 500}, {1000, 500}}, {roller=150})");
    assert!((plate(&s).ground_share() - 1.0).abs() < 1e-9);
    // a small brush lightly loaded runs dry and says so
    let r = s.run("r = k:stop_out({{50, 300}, {950, 300}}, {brush=1, load=0.1, pressure=0.85})\nprint(r.dry, r.length, r.width)").unwrap();
    assert!(r.out.contains("ran dry") && r.out.contains("true"), "{}", r.out);
    let p = plate(&s);
    let c = |mm: f64| (mm * 1000.0 / p.pitch_um()) as usize;
    assert!(p.etch(c(3.0), c(12.0)).stopped && !p.etch(c(30.0), c(12.0)).stopped);
}
