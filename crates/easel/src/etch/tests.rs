//! Tiny plates (40 × 30 mm beside a 64 px painting) for `--@ etch`.

use crate::session::{ETCH_MARK, PLATE_MARK, SURFACES_MARK, Session, box_for, parse_program};

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const PLATE: &str = r#"k = plate{name="key", metal="copper", temper="half-hard", size={40, 30}}
n = needle{tip=40, cone=30}"#;
/// A line plate: ground, two needle lines, a bite, the lower line stopped out, a second bite,
/// cleaned; then an aquatint, melted and bitten.
const ETCH: &str = r#"use(k)
k:ground("hard", {um=4})
r = k:needle(n, {{100, 200}, {900, 200}})
assert(r.touched)
k:needle(n, {{100, 500}, {900, 500}})
b = k:bite{acid="ferric", baume=36, minutes=10}
k:stop_out(rect(0, 400, 1000, 350))
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
    let e = s.run("k:ground(\"hard\")").unwrap_err();
    assert!(e.contains("clean it first"), "{e}");
    {
        let st = s.st.borrow();
        let (c, pl) = (st.canvas.as_ref().unwrap(), st.plate.as_ref().unwrap());
        let text = crate::plate::probe(pl, "at 500,200", (13, 5, 14, 6));
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
/// bytes (a plate no etching tool touched is PLAT version 1) and so is every surface; the logs
/// differ by the switch's line alone; a log without it refuses the etching tools.
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
    assert_eq!(u64::from_le_bytes(p[0].1[4..12].try_into().unwrap()), 1, "a drypoint plate is version 1");
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
    let e = r.run("use(k)\nk:ground(\"hard\")").unwrap_err();
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
