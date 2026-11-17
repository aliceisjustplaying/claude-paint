//! Tiny plates (40 × 30 mm beside a 64 px painting) for `--@ plate`.

use crate::session::{INTAGLIO_CONTACT_MARK, PLATE_MARK, SURFACES_MARK, Session, box_for, parse_program};
use crate::surfaces::MAIN;

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const PLATE: &str = r#"k = plate{name="key", metal="copper", temper="half-hard", size={40, 30}}
n = needle{tip=25, cone=30}"#;
/// Lines at 0.5, 1, 2 and 5 N pulled at a lean, one pushed, a burnished patch and a scrape.
const WORK: &str = r#"use(k)
for i, f in ipairs({0.5, 1, 2, 5}) do
  k:drypoint(n, {{100, 100 + 100 * i}, {900, 100 + 100 * i}}, {force=f, alt=55, azimuth=0})
end
r = k:drypoint(n, {{100, 650}, {900, 650}}, {force=2, alt=55, azimuth=180})
assert(r.cut)
k:burnish({{300, 300}, {600, 300}}, {force=30, radius=10, passes=3})
k:scrape({{650, 400}, {800, 400}}, {force=5, width=4})"#;

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

/// A folder of the studio for one test's files, and its path relative to the studio.
struct Folder(std::path::PathBuf, String);

impl Folder {
    fn new(name: &str) -> Self {
        let rel = format!("out/easel/plate-test-{}-{name}", std::process::id());
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

/// The plan's test sheet, small: the plate's lines widen as √F, the pushed line cut, the
/// burnished patch keeps the metal, the scrape takes some; its hand time is on the clock; the
/// fingernail finds the lines; paint is refused on it.
#[test]
fn a_plate_is_worked_with_its_tools() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let t0 = s.st.borrow().clock;
    run(&mut s, WORK);
    let p = plate(&s);
    assert_eq!((p.metal, p.temper), (paint::plate::Metal::Copper, paint::plate::Temper::HalfHard));
    // (the needle's hand time: four lines and one of 32 mm at 30 mm/s, the scrape and burnish after)
    let t1 = s.st.borrow().clock;
    assert!(t1 - t0 >= 5.0 * 32.0 / 30.0 / 60.0, "{t0} → {t1}");
    assert!(p.chip_um3 > 0.0, "the pushed line and the scrape took metal");
    assert!((p.metal_um3() + p.chip_um3).abs() < 1e-3 * p.groove_um3(), "the rest is conserved: {} {}", p.metal_um3(), p.chip_um3);
    let out = s.run(r#"local a = k:feel(500, 300)
local b = k:feel(500, 50)
print(a.words, b.words)
assert(a.depth > 0 and b.words == "smooth")"#).unwrap();
    assert!(out.out.contains("line") && out.out.contains("smooth"), "{}", out.out);
    let e = s.run(r#"h = pencil("2B")
h:line({{100,100},{800,700}})"#).unwrap_err();
    assert!(e.contains("is a plate"), "{e}");
    // its look and probe (the server's, through a session's show of it)
    let shown = s.show("key").unwrap();
    {
        let st = s.st.borrow();
        let (c, pl) = (st.canvas.as_ref().unwrap(), st.plate.as_ref().unwrap());
        for v in [crate::look::View::default(), crate::look::View { light: Some((135.0, 25.0)), ..Default::default() }, crate::look::View { crop: Some([400.0, 250.0, 600.0, 350.0]), light: Some((135.0, 25.0)), ..Default::default() }] {
            let (w, h, png) = super::render(c, pl, &v).unwrap();
            assert!(w > 0 && h > 0 && !png.is_empty());
        }
        let text = super::probe(pl, c.clock(), "at 500,300", (13, 7, 14, 8));
        assert!(text.contains("copper") && text.contains("µm"), "{text}");
    }
    s.unshow(shown).unwrap();
}

/// A replay of the log gives the plate again bit for bit; a save, and a replay checkpoint
/// resumed after the plate was worked, keep it; a failed chunk puts it back.
#[test]
fn a_plate_survives_replay_save_and_checkpoint() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    run(&mut s, WORK);
    run(&mut s, "use(\"painting\")");
    run(&mut s, "use(k)\nk:drypoint(n, {{200, 700}, {800, 100}}, {force=1})\nwait(5)");
    let log = s.program("plate");
    assert!(log.lines().any(|l| l == PLATE_MARK));
    let r = replay(&log).unwrap();
    assert!(plates(&r) == plates(&s), "the replay's plate is the painting's");
    assert_eq!(r.st.borrow().surfaces.identity(), s.st.borrow().surfaces.identity());

    // a failed chunk: the plate as it was
    let before = plates(&s);
    let e = s.run("k:drypoint(n, {{100, 50}, {900, 50}}, {force=5})\nerror(\"no\")").unwrap_err();
    assert!(e.contains("no"));
    assert!(plates(&s) == before);

    // the save
    let dir = Folder::new("save");
    let path = dir.0.join("live.ckpt");
    assert!(crate::save::write(&s, &log, &path).unwrap());
    let back = crate::save::read(&path).unwrap();
    assert!(plates(&back.session) == plates(&s), "a save round trip keeps the plate");
    assert!(back.session.st.borrow().tubes.plate);

    // a replay checkpoint after the third chunk (the plate worked), resumed, then the rest
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
    assert!(plates(&resumed) == plates(&first), "reconstructed with the plate the checkpoint kept");
    for c in &chunks[3..] {
        resumed.run(c).unwrap();
    }
    drop(batch);
    assert!(plates(&resumed) == plates(&r), "and on, as the painting went");
}

/// A plate handed off arrives as it left (its state bit for bit), a plate in the receiving
/// session; one without the switch can't receive it.
#[test]
fn a_plate_is_handed_off_whole() {
    let dir = Folder::new("handoff");
    let mut a = live();
    run(&mut a, CANVAS);
    run(&mut a, PLATE);
    run(&mut a, WORK);
    let sent = plate(&a);
    let out = a.run(&format!("handoff(k, \"{}/key.surface\")", dir.1)).unwrap();
    let sha = out.out.split("sha256=").nth(1).unwrap()[..64].to_string();
    assert!(a.st.borrow().surfaces.find("key").is_none());
    let mut b = live();
    run(&mut b, CANVAS);
    run(&mut b, &format!("k = receive(\"{}/key.surface\", {{after=60, sha256=\"{sha}\"}})\nassert(k.kind == \"plate\")\nuse(k)\nprint(k:feel(500, 300).words)", dir.1));
    assert!(plate(&b) == sent, "the plate arrives as it left");
    // a session without plates refuses it
    let mut nb = box_for(None).unwrap();
    nb.plate = false;
    let mut c = Session::with_box(W, nb).unwrap().with_paper(true).unwrap();
    run(&mut c, CANVAS);
    let e = c.run(&format!("receive(\"{}/key.surface\", {{after=0, sha256=\"{sha}\"}})", dir.1)).unwrap_err();
    assert!(e.contains("no plates"), "{e}");
}

/// Without the switch nothing changes: a painting with named surfaces and the switch, that
/// makes no plate, is byte-identical to one without it but for the switch's lines; a log
/// without it has no `plate` or `needle`, and a sheet takes no plate tools.
#[test]
fn without_the_switch_byte_identical() {
    let mut on = live();
    let mut off_box = box_for(None).unwrap();
    off_box.plate = false;
    let mut off = Session::with_box(W, off_box).unwrap().with_paper(true).unwrap();
    for s in [&mut on, &mut off] {
        run(s, CANVAS);
        run(s, "st = sheet{name=\"study\", size=60, aspect=1, paper={grammage=200}, seed=3}\nuse(st)\nh = pencil(\"2B\")\nh:line({{100,100},{800,700}})\nwait(10)\nuse(\"painting\")");
    }
    let state = |s: &Session| {
        let mut b = Vec::new();
        crate::surfaces::write_all(&s.st.borrow(), "", &mut b).unwrap();
        b
    };
    assert!(state(&on) == state(&off), "every surface as before");
    assert!(state(&on)[8] == 1, "a multi-surface checkpoint without plates is version 1, as before");
    let (lon, loff) = (on.program("x"), off.program("x"));
    assert!(lon.contains(PLATE_MARK) && !loff.contains(PLATE_MARK));
    assert_eq!(lon.replace(&format!("{PLATE_MARK}\n"), ""), loff);
    let dir = Folder::new("alone");
    let (pon, poff) = (dir.0.join("on.ckpt"), dir.0.join("off.ckpt"));
    crate::save::write(&on, &lon, &pon).unwrap();
    crate::save::write(&off, &lon, &poff).unwrap();
    let read = |p: &std::path::Path| crate::surfaces::read_all(&mut std::fs::File::open(p).unwrap()).unwrap();
    let (ron, roff) = (read(&pon), read(&poff));
    assert_eq!(ron.header.replace("plate=1\n", ""), roff.header, "the save's header but its switch line");
    let rest = |p: &std::path::Path, h: &str| std::fs::read(p).unwrap().len() - h.len();
    assert_eq!(rest(&pon, &ron.header), rest(&poff, &roff.header), "and the rest as before");
    for verb in ["plate", "needle"] {
        assert!(off.lua.globals().get::<mlua::Value>(verb).unwrap().is_nil(), "{verb}");
        assert!(!on.lua.globals().get::<mlua::Value>(verb).unwrap().is_nil(), "{verb}");
    }
    let e = replay(&format!("{loff}\n--@ chunk 3\nk = plate{{name=\"k\", metal=\"zinc\", size={{30, 30}}}}\n")).err().unwrap();
    assert!(e.contains("plate"), "{e}");
    let e = on.run("st:drypoint(needle{}, {{0,0},{10,10}})").unwrap_err();
    assert!(e.contains("plate's tool"), "{e}");
}

/// The switch comes from the log's head, needs the named surfaces, and the program writes it
/// back. The period box has copper and zinc only.
#[test]
fn the_switch_comes_from_the_log_head() {
    assert!(box_for(None).unwrap().plate);
    let head = format!("--@ engine 9\n--@ paper\n{SURFACES_MARK}\n");
    assert!(!box_for(Some(&head)).unwrap().plate);
    assert!(box_for(Some(&format!("{head}{PLATE_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().plate);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{PLATE_MARK}\n"))).unwrap().plate, "only before the first chunk");
    let mut s = live();
    run(&mut s, CANVAS);
    let log = s.program("x");
    let r = replay(&log).unwrap();
    assert!(r.st.borrow().tubes.plate);
    assert_eq!(r.program("x"), log);
    // without the named surfaces it is accepted and installs nothing
    let mut b = box_for(None).unwrap();
    b.surfaces = false;
    let bare = Session::with_box(W, b).unwrap().with_paper(true).unwrap();
    assert!(bare.lua.globals().get::<mlua::Value>("plate").unwrap().is_nil());
    let mut s = live();
    run(&mut s, CANVAS);
    let e = s.run(r#"plate{name="b", metal="brass", size={30, 30}}"#).unwrap_err();
    assert!(e.contains("1890s"), "{e}");
    let e = s.run(r#"plate{name="z", metal="zinc", temper="half-hard", size={30, 30}}"#).unwrap_err();
    assert!(e.contains("rolled"), "{e}");
    assert_eq!(s.st.borrow().surfaces.current_name(), MAIN);
}

/// A line drawn leftward, the needle pulled the way it goes at 55° as a painter
/// sets it, each point's azimuth the line's way there: atan2 gives it as 180° or
/// −180° by the sign of a hair's rise or fall.
const LEFTWARD: &str = r#"r = k:drypoint(n, {{900, 300}, {700, 302}, {500, 299}, {300, 301}, {100, 300}}, {force=2, alt=55, azimuth={179.4, -179.7, 179.6, -179.5, 180}})
print("chip=" .. r.chip .. " groove=" .. r.groove .. " beta=" .. r.beta)"#;

fn contact_box(on: bool) -> paint::palette::Palette {
    let mut b = box_for(None).unwrap();
    b.plate_contact = on;
    b
}

fn val(out: &str, key: &str) -> f64 {
    out.split_whitespace().find_map(|w| w.strip_prefix(&format!("{key}="))).unwrap_or_else(|| panic!("{key} in {out}")).parse().unwrap()
}

/// The painter's fault: without the contact laws the azimuth is interpolated
/// straight from 179.4° to −179.7°, swinging the needle round through 0° between
/// two points, so it is pushed there and cuts a chip, while the line ploughs at 25°
/// elsewhere. The message no longer quotes the ploughing 25° as the angle that cut.
/// With the laws the azimuth turns the short way and the whole line ploughs.
#[test]
fn a_pulled_line_drawn_leftward_ploughs_under_the_contact_laws() {
    let mut out = Vec::new();
    for on in [false, true] {
        let mut s = Session::with_box(W, contact_box(on)).unwrap().with_paper(true).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, PLATE);
        run(&mut s, "use(k)");
        assert_eq!(plate(&s).contact, on);
        out.push(s.run(LEFTWARD).unwrap().out);
    }
    let (old, new) = (&out[0], &out[1]);
    assert!(val(old, "chip") > 0.0, "without the laws part of it cut: {old}");
    assert!(old.contains("cut a chip from") && old.contains("% of the groove") && !old.contains("25°"), "{old}");
    assert_eq!(val(new, "chip"), 0.0, "{new}");
    assert!(!new.contains("cut a chip"), "{new}");
    assert!((val(new, "beta") - 25.0).abs() < 0.01, "pulled at 55° with a 30° cone: 90 − 30 − 35 = 25°: {new}");
    assert!((val(new, "groove") - val(old, "groove")).abs() < 1e-6 * val(old, "groove"), "the same groove");
}

/// The contact switch comes from the log's head and goes with the plate: a new
/// painting has it and writes it; a log without it makes plates as before, and
/// its plates save and read back as before; a plate made under it keeps it
/// through a save, a replay and a handoff.
#[test]
fn the_contact_laws_come_from_the_log_head_and_stay_with_the_plate() {
    assert!(box_for(None).unwrap().plate_contact);
    let head = format!("--@ engine 9\n--@ paper\n{SURFACES_MARK}\n{PLATE_MARK}\n");
    assert!(!box_for(Some(&head)).unwrap().plate_contact);
    assert!(box_for(Some(&format!("{head}{INTAGLIO_CONTACT_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().plate_contact);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{INTAGLIO_CONTACT_MARK}\n"))).unwrap().plate_contact, "only before the first chunk");
    let dir = Folder::new("contact");
    let mut logs = Vec::new();
    for on in [false, true] {
        let mut s = Session::with_box(W, contact_box(on)).unwrap().with_paper(true).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, PLATE);
        run(&mut s, WORK);
        run(&mut s, "use(k)\nk:drypoint(n, {{200, 700}, {800, 100}}, {force=1})");
        let log = s.program("contact");
        assert_eq!(log.lines().any(|l| l == INTAGLIO_CONTACT_MARK), on);
        let r = replay(&log).unwrap();
        assert!(plates(&r) == plates(&s) && plate(&r).contact == on, "the replay's plate is the painting's");
        let path = dir.0.join(format!("{on}.ckpt"));
        assert!(crate::save::write(&s, &log, &path).unwrap());
        let back = crate::save::read(&path).unwrap();
        assert!(plates(&back.session) == plates(&s) && back.session.st.borrow().tubes.plate_contact == on);
        // the plate's own section: version 1 as before without the laws, 4 with them
        let mut b = Vec::new();
        plate(&s).write(&mut b).unwrap();
        assert_eq!(u64::from_le_bytes(b[4..12].try_into().unwrap()), if on { 4 } else { 1 });
        logs.push(log);
    }
    assert_eq!(logs[1].replace(&format!("{INTAGLIO_CONTACT_MARK}\n"), ""), logs[0]);
    // a plate made under the laws keeps them in a session without them
    let mut a = Session::with_box(W, contact_box(true)).unwrap().with_paper(true).unwrap();
    run(&mut a, CANVAS);
    run(&mut a, PLATE);
    run(&mut a, WORK);
    let sent = plate(&a);
    let out = a.run(&format!("handoff(k, \"{}/key.surface\")", dir.1)).unwrap();
    let sha = out.out.split("sha256=").nth(1).unwrap()[..64].to_string();
    let mut b = Session::with_box(W, contact_box(false)).unwrap().with_paper(true).unwrap();
    run(&mut b, CANVAS);
    run(&mut b, &format!("k = receive(\"{}/key.surface\", {{after=60, sha256=\"{sha}\"}})\nuse(k)", dir.1));
    assert!(plate(&b) == sent && plate(&b).contact);
}
