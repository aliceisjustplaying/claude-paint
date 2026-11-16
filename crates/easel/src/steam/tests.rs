//! Tiny scenes (64 px) for steam and sifted pastel (`--@ steam-sift`). Expected numbers come
//! from paint's steam laws (tested against the literature there), never from a run of the easel.

use crate::session::{STEAM_SIFT_MARK, Session, box_for, parse_program};
use mlua::Value;

const W: usize = 64;
const PAPER: &str = r#"canvas{size=60, aspect=1, paper={grammage=200}, seed=3}"#;

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

fn state(s: &Session) -> Vec<u8> {
    let mut b = Vec::new();
    s.st.borrow().canvas.as_ref().unwrap().write_state(&mut b, "").unwrap();
    b
}

/// A number a chunk printed as `name=<value>`.
fn printed(out: &str, name: &str) -> f64 {
    let at = out.find(&format!("{name}=")).unwrap_or_else(|| panic!("no {name}= in {out}"));
    out[at + name.len() + 1..].split_whitespace().next().unwrap().parse().unwrap()
}

/// The switch comes from the log's head: a new painting has it, a log has it only if its head
/// names it, and the program writes it back; without paper it installs nothing.
#[test]
fn the_switch_comes_from_the_log_head() {
    assert!(box_for(None).unwrap().steam_sift);
    let head = "--@ engine 9\n--@ paper\n--@ wet-pastel\n";
    assert!(!box_for(Some(head)).unwrap().steam_sift);
    assert!(box_for(Some(&format!("{head}{STEAM_SIFT_MARK}\n--@ chunk 1\nx=1\n"))).unwrap().steam_sift);
    assert!(!box_for(Some(&format!("{head}--@ chunk 1\n{STEAM_SIFT_MARK}\n"))).unwrap().steam_sift, "only before the first chunk");
    let mut s = live();
    run(&mut s, PAPER);
    let log = s.program("x");
    assert!(log.lines().any(|l| l == STEAM_SIFT_MARK));
    let r = replay(&log).unwrap();
    assert!(r.st.borrow().tubes.steam_sift);
    assert_eq!(r.program("x"), log);
    let bare = Session::with_box(W, box_for(None).unwrap()).unwrap();
    assert!(bare.lua.globals().get::<Value>("steam").unwrap().is_nil());
}

/// Without the switch a session is byte-identical: a pastel drawn and tapped the same with and
/// without it comes out bit for bit, its log differs by the switch's line only, a log without
/// it has neither verb, and an older log calling one fails.
#[test]
fn without_the_switch_it_is_byte_identical() {
    let mut on = live();
    let mut off_box = box_for(None).unwrap();
    off_box.steam_sift = false;
    let mut off = Session::with_box(W, off_box).unwrap().with_paper(true).unwrap();
    for s in [&mut on, &mut off] {
        run(s, PAPER);
        run(s, "p = pastel(pile{{\"cobalt blue\", 1}, {\"lead white\", 2}})\np:stroke({{100,500},{900,500}}, {force=3})\ntap({g=200})\nwait(30)");
    }
    assert!(state(&on) == state(&off));
    let (lon, loff) = (on.program("x"), off.program("x"));
    assert_eq!(lon.replace(&format!("{STEAM_SIFT_MARK}\n"), ""), loff);
    for verb in ["steam", "sift"] {
        assert!(off.lua.globals().get::<Value>(verb).unwrap().is_nil(), "{verb}");
        assert!(!on.lua.globals().get::<Value>(verb).unwrap().is_nil(), "{verb}");
    }
    let e = replay(&format!("{loff}\n--@ chunk 3\nsteam(nil, {{seconds=5}})\n")).err().unwrap();
    assert!(e.contains("steam"), "{e}");
}

/// A stick crushed and sifted shortens by the volume over its section; on the dry sheet a tap
/// sheds most of it. A patch steamed and sifted at once, while the condensed drops stand on the
/// sized sheet, holds its crumbs in clusters through a tap. The log replays to the same sheet.
#[test]
fn sift_a_stick_dry_and_steamed() {
    let mut s = live();
    run(&mut s, PAPER);
    let out = run(&mut s, "p = pastel(pile{{\"vermilion\", 1}, {\"lead white\", 1}})\nlocal l0 = p.length\nlocal r = sift(p, nil, {amount=20})\nprint(\"cut=\" .. (l0 - p.length) .. \" laid=\" .. r.laid .. \" loose=\" .. r.loose .. \" cluster=\" .. r.cluster)\nprint(\"fell=\" .. tap())");
    let area = std::f64::consts::FRAC_PI_4 * 12.0 * 12.0;
    assert!((printed(&out, "cut") - 20.0 / area).abs() < 1e-4, "{out}");
    assert!((printed(&out, "laid") - 20.0).abs() < 1e-9 && printed(&out, "cluster") == 0.0);
    assert!(printed(&out, "fell") > 10.0, "most of the dry crumbs tap off: {out}");

    // a patch steamed (30 mm square, the jet's footprint) and sifted at once, while its drops stand
    let out = run(&mut s, "m = rect(250, 250, 500, 500)\nlocal w, t = steam(m, {seconds=20})\nprint(\"water=\" .. w .. \" face=\" .. t)\nlocal r = sift(pile{{\"green earth\", 1}}, m, {amount=2, grain=0.3})\nprint(\"stuck=\" .. r.stuck .. \" cluster=\" .. r.cluster .. \" film=\" .. r.film)\nprint(\"fell=\" .. tap())");
    let pl = paint::steam::plume(200.0);
    assert!(printed(&out, "water") > 5.0 && printed(&out, "face") < pl.dew_c, "{out}");
    assert!(printed(&out, "stuck") > 1.5 && printed(&out, "cluster") > 0.0 && printed(&out, "film") > 0.0, "{out}");
    assert!(printed(&out, "fell") < 0.1, "wet, the crumbs stay: {out}");
    let log = s.program("x");
    let r = replay(&log).unwrap();
    assert!(state(&r) == state(&s), "the log replays to the same sheet");
}

/// `steam` asks for its seconds and keeps the kettle where the jet has mixed; `sift` asks for
/// its amount and won't crush a stick shorter than the fingers hold.
#[test]
fn the_verbs_refuse_what_they_cannot_do() {
    let mut s = live();
    run(&mut s, PAPER);
    for (src, word) in [
        ("steam(nil, {})", "seconds"),
        ("steam(nil, {seconds=5, distance=20})", "distance"),
        ("sift(pastel(pile{{\"vermilion\", 1}}), nil, {})", "amount"),
        ("sift(pastel(pile{{\"vermilion\", 1}}, {length=6}), nil, {amount=500})", "hold"),
        ("sift(pastel(pile{{\"vermilion\", 1}}), nil, {amount=5, grain=9})", "grain"),
    ] {
        let e = s.run(src).err().unwrap_or_else(|| panic!("{src} ran")).to_string();
        assert!(e.contains(word), "{src}: {e}");
    }
}
