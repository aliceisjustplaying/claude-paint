//! Tiny scenes (64 px) for several named surfaces in one session (`--@ surfaces`).

use super::*;
use crate::session::{SURFACES_MARK, Session, box_for, parse_program};

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const OIL: &str = r#"p = pile{{"red earth",1}}
b = brush{kind="round",width=20}
b:load(p,0.8)
b:stroke({{100,100},{600,500}}, {pressure=0.6})"#;
const STUDY: &str = r#"st = sheet{name="study", size=60, aspect=1, paper={grammage=200}, seed=3}"#;

/// A new painting's session (the switch on), live (a failed chunk rolls back).
fn live() -> Session {
    Session::with_box(W, box_for(None).unwrap()).unwrap().with_paper(true).unwrap()
}

/// A replay of `log`, from the switches its head names.
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

fn state(c: &Canvas) -> Vec<u8> {
    let mut b = Vec::new();
    c.write_state(&mut b, "").unwrap();
    b
}

/// Every surface's checkpoint bytes, by name: the current one's from memory, the others'
/// as they rest.
fn states(s: &Session) -> Vec<(String, Vec<u8>)> {
    let st = s.st.borrow();
    if !st.surfaces.named() {
        return vec![(MAIN.into(), state(st.canvas.as_ref().unwrap()))];
    }
    st.surfaces.slots.iter().map(|slot| {
        let bytes = match &slot.rest {
            None => state(st.canvas.as_ref().unwrap()),
            Some(r) => {
                let mut b = Vec::new();
                r.canvas.as_ref().unwrap().copy_state(&mut b).unwrap();
                b
            }
        };
        (slot.name.clone(), bytes)
    }).collect()
}

fn surface(s: &Session, name: &str) -> Vec<u8> {
    states(s).into_iter().find(|(n, _)| n == name).unwrap_or_else(|| panic!("no surface {name}")).1
}

fn clock(s: &Session) -> f64 {
    s.st.borrow().clock
}

/// A folder of the studio for one test's handed-off files (out/easel is the easel's own and
/// not committed), and its path relative to the studio.
struct Folder(PathBuf, String);

impl Folder {
    fn new(name: &str) -> Self {
        let rel = format!("out/easel/surfaces-test-{}-{name}", std::process::id());
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

/// Two surfaces drawn independently, on one clock: the painting's oil, left while the study is
/// worked for half an hour, is what it would be had the painter waited that long beside it
/// (bit for bit: one wait of the time it rested), and the study is what a canvas of that
/// paper drawn the same way is.
#[test]
fn two_surfaces_are_drawn_independently_and_age_together() {
    // (the session's randomness is shared by its surfaces: the painting's seed is the study's)
    let canvas = CANVAS.replace("seed=37", "seed=3");
    let mut a = live();
    run(&mut a, &format!("{canvas}\n{OIL}"));
    let c1 = clock(&a);
    run(&mut a, STUDY);
    assert_eq!(clock(&a), c1, "a sheet taken from the drawer takes no hand time");
    run(&mut a, r#"assert(use(st) == "painting")
h = pencil("2B")
h:line({{100,100},{800,700}}, {pressure=0.5})
wait(30)
assert(use("painting") == "study")"#);
    let rested = clock(&a) - c1;
    assert!(rested >= 30.0, "{rested}");
    // the painting: the stroke, then the time it rested
    let mut b = live();
    run(&mut b, &format!("{canvas}\n{OIL}"));
    run(&mut b, &format!("wait({rested:?})"));
    assert!(surface(&a, MAIN) == state(&b.canvas().unwrap()), "the painting aged while the study was worked, as if waited beside");
    // the study: a sheet of the same paper and pitch with the same line, waited the same
    let mut c = Session::with_box(38, box_for(None).unwrap()).unwrap().with_paper(true).unwrap();
    run(&mut c, r#"canvas{size=60, aspect=1, paper={grammage=200}, seed=3}"#);
    // (the line in the third chunk, as on the study)
    run(&mut c, "x = 1");
    run(&mut c, r#"h = pencil("2B")
h:line({{100,100},{800,700}}, {pressure=0.5})
wait(30)"#);
    assert!(surface(&a, "study") == state(&c.canvas().unwrap()), "the study holds its own line, nothing of the painting's");
    // and the line is only on the study: the painting's drawing is untouched
    assert!(a.canvas().unwrap().drawing_guide().data.iter().all(|v| *v == 0.0));
}

/// A failed chunk that drew on both surfaces and changed which is current leaves both as they
/// were, the painting current and the clock where it was.
#[test]
fn a_failed_chunk_puts_back_every_surface() {
    let mut s = live();
    run(&mut s, &format!("{CANVAS}\n{OIL}"));
    run(&mut s, &format!("{STUDY}\nuse(st)\nh = pencil(\"2B\")\nh:line({{{{100,100}},{{500,500}}}})\nuse(\"painting\")"));
    let (before, t) = (states(&s), clock(&s));
    let e = s.run(r#"b:load(p,0.8)
b:stroke({{200,800},{800,800}}, {pressure=0.6})
use(st)
h:line({{100,900},{900,100}})
wait(20)
error("the chunk fails")"#).unwrap_err();
    assert!(e.contains("the chunk fails"), "{e}");
    assert!(states(&s) == before, "both surfaces as they were");
    assert_eq!(s.st.borrow().surfaces.current_name(), MAIN);
    assert_eq!(clock(&s), t);
    assert!(!s.stale);
}

/// The log of a session with three surfaces replays to the same surfaces, bit for bit, and its
/// save and its replay checkpoint carry all three (the current one a sheet).
#[test]
fn three_surfaces_replay_save_and_checkpoint() {
    let mut s = live();
    run(&mut s, &format!("{CANVAS}\n{OIL}"));
    run(&mut s, &format!("{STUDY}\njp = sheet{{name=\"japan\", size=40, aspect=0.8, paper={{grammage=40}}, seed=5}}"));
    run(&mut s, "use(jp)\nh = pencil(\"HB\")\nh:line({{100,100},{900,900}})\nwait(12)");
    run(&mut s, "use(st)\nh:line({{900,100},{100,900}})");
    let log = s.program("three");
    assert!(log.lines().any(|l| l == SURFACES_MARK));
    let r = replay(&log).unwrap();
    assert!(states(&r) == states(&s), "the replay's surfaces are the painting's");
    assert_eq!(r.st.borrow().surfaces.current_name(), "study");
    assert_eq!(r.st.borrow().surfaces.identity(), s.st.borrow().surfaces.identity());

    // the save: every surface, and the current one's own fields
    let dir = Folder::new("save");
    std::fs::create_dir_all(&dir.0).unwrap();
    let path = dir.0.join("live.ckpt");
    assert!(crate::save::write(&s, &log, &path).unwrap());
    let back = crate::save::read(&path).unwrap();
    assert!(states(&back.session) == states(&s), "a save round trip keeps every surface");
    {
        let (a, b) = (back.session.st.borrow(), s.st.borrow());
        assert_eq!(a.surfaces.identity(), b.surfaces.identity());
        assert_eq!((a.setup.clone(), a.clock0.to_bits(), a.clock.to_bits()), (b.setup.clone(), b.clock0.to_bits(), b.clock.to_bits()));
        assert!(a.tubes.surfaces);
    }
    // a damaged section is refused: the section's version, then a byte of its first canvas
    let bytes = std::fs::read(&path).unwrap();
    assert!(paint::checkpoint::is_surfaces(&bytes));
    // (version 2 holds plates: `--@ plate`; a version this build doesn't know is refused)
    let mut v9 = bytes.clone();
    v9[8] = 9;
    std::fs::write(&path, &v9).unwrap();
    let e = crate::save::read(&path).err().unwrap();
    assert!(e.contains("version 9"), "{e}");
    let mut cut = bytes.clone();
    cut.truncate(bytes.len() - 100);
    std::fs::write(&path, &cut).unwrap();
    assert!(crate::save::read(&path).is_err());

    // the replay checkpoint: written after the third chunk, resumed, then the fourth (a `use`)
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
    assert!(states(&resumed) == states(&first), "reconstructed with every surface the checkpoint kept");
    for c in &chunks[3..] {
        resumed.run(c).unwrap();
    }
    drop(batch);
    assert!(states(&resumed) == states(&s), "and on across a use, as the painting went");
}

/// A session with the switch that never names a surface is byte-identical to one without it:
/// its canvas, its log but the switch's line and its save but the switch's header line. A log
/// without the switch has none of its verbs.
#[test]
fn the_painting_alone_is_byte_identical() {
    let mut on = live();
    let mut off_box = box_for(None).unwrap();
    off_box.surfaces = false;
    let mut off = Session::with_box(W, off_box).unwrap().with_paper(true).unwrap();
    for s in [&mut on, &mut off] {
        run(s, &format!("{CANVAS}\n{OIL}"));
        run(s, "wait(15)\nh = pencil(\"2B\")\nh:line({{100,800},{800,800}})");
    }
    assert!(state(&on.canvas().unwrap()) == state(&off.canvas().unwrap()));
    let (lon, loff) = (on.program("x"), off.program("x"));
    assert!(lon.contains(SURFACES_MARK) && !loff.contains(SURFACES_MARK));
    assert_eq!(lon.replace(&format!("{SURFACES_MARK}\n"), ""), loff);
    let dir = Folder::new("alone");
    std::fs::create_dir_all(&dir.0).unwrap();
    let (pon, poff) = (dir.0.join("on.ckpt"), dir.0.join("off.ckpt"));
    crate::save::write(&on, &lon, &pon).unwrap();
    crate::save::write(&off, &lon, &poff).unwrap();
    let (bon, boff) = (std::fs::read(&pon).unwrap(), std::fs::read(&poff).unwrap());
    assert!(!paint::checkpoint::is_surfaces(&bon), "one canvas, as before");
    let read = |b: &[u8]| Canvas::read_state(&mut std::io::Cursor::new(b)).unwrap();
    let ((con, hon), (coff, hoff)) = (read(&bon), read(&boff));
    assert!(state(&con) == state(&coff), "the save's canvas as before");
    assert_eq!(hon.replace("surfaces=1\n", ""), hoff, "the save's header but its switch line");
    // the old log: none of the verbs
    let e = replay(&format!("{loff}\n--@ chunk 3\nsheet{{name=\"s\", size=50, aspect=1, paper={{}}}}\n")).err().unwrap();
    assert!(e.contains("sheet"), "{e}");
    for verb in ["sheet", "use", "surfaces", "handoff", "receive"] {
        assert!(off.lua.globals().get::<Value>(verb).unwrap().is_nil(), "{verb}");
        assert!(!on.lua.globals().get::<Value>(verb).unwrap().is_nil(), "{verb}");
    }
}

/// The switch comes from the log's head: a new painting has it, a log has it only if its head
/// names it, and the program writes it back.
#[test]
fn the_switch_comes_from_the_log_head() {
    assert!(box_for(None).unwrap().surfaces);
    let head = "--@ engine 9\n--@ paper\n";
    assert!(!box_for(Some(head)).unwrap().surfaces);
    assert!(box_for(Some(&format!("{head}{SURFACES_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().surfaces);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{SURFACES_MARK}\n"))).unwrap().surfaces, "only before the first chunk");
    let mut s = live();
    run(&mut s, CANVAS);
    let log = s.program("x");
    let r = replay(&log).unwrap();
    assert!(r.st.borrow().tubes.surfaces);
    assert_eq!(r.program("x"), log);
    // without paper the switch is accepted and installs nothing (a sheet is paper)
    let mut b = box_for(None).unwrap();
    b.surfaces = true;
    let bare = Session::with_box(W, b).unwrap();
    assert!(bare.lua.globals().get::<Value>("sheet").unwrap().is_nil());
}

/// What the handoff tests share: a sender whose sheet "plate" has fresh oil on it.
fn sender() -> Session {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, r#"pl = sheet{name="plate", size=60, aspect=1, paper={grammage=300}, seed=4}
use(pl)
p = pile{{"bone black",1}}
b = brush{kind="round",width=20}
b:load(p,0.8)
b:stroke({{100,300},{900,600}}, {pressure=0.6})"#);
    s
}

fn receiver() -> Session {
    let mut s = live();
    run(&mut s, CANVAS);
    s
}

fn sha_of(out: &str) -> String {
    out.split("sha256=").nth(1).unwrap()[..64].to_string()
}

/// A surface handed off and received after an interval arrives aged by exactly it: as if the
/// sender had waited that long with it, bit for bit, and as it left with no interval. It
/// leaves the sender, and the receiving log names it by its content, so a replay with the file
/// changed is refused.
#[test]
fn a_handed_off_surface_arrives_aged_by_its_time_on_the_way() {
    let dir = Folder::new("handoff");
    let mut s = sender();
    let left = surface(&s, "plate");
    let file = format!("{}/plate.surface", dir.1);
    let out = s.run(&format!("assert(handoff(pl, {file:?}) ~= nil)")).unwrap().out;
    let sha = sha_of(&out);
    assert!(dir.0.join("plate.surface").is_file());
    assert_eq!(s.st.borrow().surfaces.names(), vec![MAIN.to_string()], "it left the sender");
    assert_eq!(s.st.borrow().surfaces.current_name(), MAIN);
    // a file there already is never replaced
    run(&mut s, r#"q = sheet{name="q", size=50, aspect=1, paper={grammage=100}}"#);
    let e = s.run(&format!("handoff(q, {file:?})")).unwrap_err();
    assert!(e.contains("handed off there already"), "{e}");

    for after in [0.0, 45.0] {
        let mut r = receiver();
        // sha256= is asked for, with the file's
        let e = r.run(&format!("receive({file:?}, {{after={after}}})")).unwrap_err();
        assert!(e.contains(&sha), "{e}");
        run(&mut r, &format!("k = receive({file:?}, {{after={after}, sha256={sha:?}}})\nassert(k.name == \"plate\")\nuse(k)"));
        let got = surface(&r, "plate");
        if after == 0.0 {
            assert!(got == left, "bit-identical with no time on the way");
        } else {
            let mut w = sender();
            run(&mut w, &format!("wait({after})"));
            assert!(got == surface(&w, "plate"), "aged as if the sender had waited {after} min");
            assert!(got != left);
        }
        // the receiving log replays to the same; with the file changed it is refused
        let log = r.program("printer");
        assert!(log.contains(&sha));
        assert!(states(&replay(&log).unwrap()) == states(&r));
        if after > 0.0 {
            let path = dir.0.join("plate.surface");
            let good = std::fs::read(&path).unwrap();
            let mut bad = good.clone();
            let n = bad.len() / 2;
            bad[n] ^= 1;
            std::fs::write(&path, &bad).unwrap();
            let e = replay(&log).err().unwrap();
            assert!(e.contains("not the surface the log received"), "{e}");
            std::fs::write(&path, &good).unwrap();
        }
    }
}

/// A pastel sheet with everything the paper switches put on it: a named sheet (its tone and
/// sizing) on a gritted ground over a strainer, pastel and vine charcoal, and a wet brush
/// through both. Made by `sheet{}` with a sheet's name alone, too. It rests (spilled to disk),
/// wakes and rests again as it was, and handed off and received with no time on the way it
/// arrives bit for bit: the grit and the drum in its paper section, charcoal's carbon in the
/// drawing's layout and the water in the wet pastel section.
#[test]
fn a_handed_off_sheet_keeps_its_grit_drum_water_and_charcoal() {
    let dir = Folder::new("cassatt");
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, r#"pa = sheet{name="pastel", size=60, aspect=1, paper={preset="tan wove", grit=true, mount="strainer"}, seed=5}
use(pa)
p = pastel(pile{{"cobalt blue",1}})
p:stroke({{100,300},{900,300}}, {force=2})
v = charcoal()
v:stroke({{100,600},{900,600}}, {force=2})
b = brush{kind="round", width=20}
b:dip(1)
b:wash({{100,300},{900,600}})"#);
    {
        let st = s.st.borrow();
        let c = st.canvas.as_ref().unwrap();
        let p = c.paper().unwrap();
        assert!(p.grit.is_some() && p.drum.is_some() && p.cobb60 == Some(25.0));
        assert!(c.pastel_water_at(500.0, 450.0) > 0.0, "wet along the wash");
    }
    let woken = surface(&s, "pastel");
    run(&mut s, r#"use("painting")"#);
    let resting = surface(&s, "pastel");
    assert!(resting == woken, "put down as it was");
    // the sections it carries
    let has = |b: &[u8], mark: &[u8; 4]| b.windows(4).any(|w| w == mark);
    assert!(has(&resting, b"WETP") && has(&resting, b"PAPR"));
    let papr = resting.windows(4).rposition(|w| w == b"PAPR").unwrap();
    assert_eq!(u64::from_le_bytes(resting[papr + 8..papr + 16].try_into().unwrap()), 2, "paper section version 2: grit and drum");
    let file = format!("{}/pastel.surface", dir.1);
    let out = s.run(&format!("assert(handoff(pa, {file:?}) ~= nil)")).unwrap().out;
    let sha = sha_of(&out);
    let mut r = receiver();
    run(&mut r, &format!("k = receive({file:?}, {{after=0, sha256={sha:?}}})\nuse(k)"));
    assert!(surface(&r, "pastel") == resting, "arrives bit for bit");
    {
        let st = r.st.borrow();
        let c = st.canvas.as_ref().unwrap();
        let p = c.paper().unwrap();
        assert!(p.grit.is_some() && p.drum.is_some());
        assert!(c.pastel_water_at(500.0, 450.0) > 0.0);
    }
    // the receiving log replays to the same
    assert!(states(&replay(&r.program("printer")).unwrap()) == states(&r));
    // a sheet's name alone, as canvas{} takes it
    run(&mut r, r#"t = sheet{name="japan", size=40, aspect=1, paper="japan paper"} use(t)"#);
    assert_eq!(r.st.borrow().canvas.as_ref().unwrap().paper().unwrap().cobb60, None, "Japan paper is unsized");
}

/// A surface handed off by a chunk that fails never leaves: no file, still in the session.
/// Files stay in the studio, as `.surface` files.
#[test]
fn a_failed_handoff_leaves_nothing_and_files_stay_in_the_studio() {
    let dir = Folder::new("failed");
    let mut s = sender();
    let file = format!("{}/plate.surface", dir.1);
    let e = s.run(&format!("handoff(pl, {file:?})\nerror(\"no\")")).unwrap_err();
    assert!(e.contains("no"), "{e}");
    assert!(!dir.0.join("plate.surface").exists());
    assert!(std::fs::read_dir(&dir.0).map(|d| d.count()).unwrap_or(0) == 0, "no part file left");
    assert!(s.st.borrow().surfaces.find("plate").is_some());
    for bad in ["/tmp/plate.surface".to_string(), format!("{}/plate.png", dir.1), format!("{}/../../../../plate.surface", dir.1)] {
        let e = s.run(&format!("handoff(pl, {bad:?})")).unwrap_err();
        assert!(e.contains("handoff"), "{bad}: {e}");
    }
    let e = s.run(&format!("handoff(\"painting\", {file:?})")).unwrap_err();
    assert!(e.contains("stays in its session"), "{e}");
}

/// A look at a resting surface sees it aged up to now and changes nothing.
#[test]
fn a_look_at_a_resting_surface_changes_nothing() {
    let mut s = sender();
    run(&mut s, "use(\"painting\")\nwait(60)");
    let before = states(&s);
    let shown = s.show("plate").unwrap();
    {
        let c = s.canvas().unwrap();
        let mut w = sender();
        run(&mut w, "wait(60)");
        let waited = state(&w.canvas().unwrap());
        // the copy waited once for the time it rested; the session's surfaces as they were
        assert_eq!(c.clock().to_bits(), w.canvas().unwrap().clock().to_bits());
        assert!(state(&c) == waited);
    }
    s.unshow(shown).unwrap();
    assert!(states(&s) == before);
    assert!(s.show("nowhere").is_err());
}

/// What a resting sheet costs at a typical size (the painting's 2400 px over 430 mm, a
/// 300 × 225 mm sheet, 1674 px across): run by name for the numbers (it builds a full-size
/// sheet).
#[test]
#[ignore]
fn resting_sheet_cost() {
    let tubes = box_for(None).unwrap();
    for (what, paper) in [("bare", true), ("drawn", false)] {
        let sty = paint::Style { name: "oil", width_mm: 300.0, ..paint::Style::oil_with(tubes.clone()) };
        let mut c = sty.prepare_paper(1674, 300.0 / 225.0, 3, paint::paper::Paper::drawing(3));
        if !paper {
            let mut st = paint::pastel::Stick::round([0.2, 0.3, 0.6], 0.8, 12.0);
            let pose = paint::pastel::Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
            c.stick_stroke(&mut st, &[paint::pastel::StrokePoint { x: 100.0, y: 300.0, pose, speed: 60.0 }, paint::pastel::StrokePoint { x: 800.0, y: 400.0, pose, speed: 60.0 }]);
        }
        let p = Packed::rest(&c).unwrap();
        let (disk, mem) = p.cost();
        let (_, kept) = Packed::take(&c).unwrap().cost();
        eprintln!("SURFACE_COST {what}: a rollback snapshot keeps {kept} bytes in memory");
        eprintln!("SURFACE_COST {what}: {}x{} px, packed file {disk} bytes, aside {mem} bytes", c.frame().w, c.frame().h);
    }
}
