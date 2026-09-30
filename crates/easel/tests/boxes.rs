//! Boxes: a new painting takes its box from a file `box` next to the easel
//! (a studio's `bin/box`), else EASEL_BOX, else the default tube box; its log
//! names the box, and every replay uses the log's box whatever easel runs it.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn base(tag: &str) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("boxes-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// A copy of the easel in `<dir>/bin`, with `bin/box` holding `name` (if any).
fn easel_in(dir: &Path, name: Option<&str>) -> PathBuf {
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    let e = dir.join("bin/easel");
    std::fs::copy(env!("CARGO_BIN_EXE_easel"), &e).unwrap();
    if let Some(n) = name {
        std::fs::write(dir.join("bin/box"), format!("{n}\n")).unwrap();
    }
    e
}

/// Run `exe` with EASEL_ROOT `root` and EASEL_BOX `env_box` (unset if None).
fn run(exe: &Path, root: &Path, env_box: Option<&str>, args: &[&str]) -> Output {
    let mut c = Command::new(exe);
    c.args(args).env("EASEL_ROOT", root).env("EASEL_SESSION", "p").env_remove("EASEL_BOX");
    if let Some(b) = env_box {
        c.env("EASEL_BOX", b);
    }
    c.output().unwrap()
}

fn ok(exe: &Path, root: &Path, env_box: Option<&str>, args: &[&str]) -> String {
    let o = run(exe, root, env_box, args);
    assert!(o.status.success(), "easel {args:?}: {}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

fn fails(exe: &Path, root: &Path, env_box: Option<&str>, args: &[&str]) -> String {
    let o = run(exe, root, env_box, args);
    assert!(!o.status.success(), "easel {args:?} should fail: {}", String::from_utf8_lossy(&o.stdout));
    String::from_utf8(o.stderr).unwrap()
}

/// The plain replay build: no box file next to it (the tests unset EASEL_BOX).
fn plain() -> PathBuf {
    let e = PathBuf::from(env!("CARGO_BIN_EXE_easel"));
    assert!(!e.with_file_name("box").exists(), "a box file next to the test's easel");
    e
}

struct Closing(PathBuf, PathBuf);
impl Drop for Closing {
    fn drop(&mut self) {
        let _ = run(&self.0, &self.1, None, &["close"]);
    }
}

const CANVAS: &str = r#"canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}"#;
const MADDER: &str = r#"b = brush("round", 4); b:load(pile{{"rose madder", 1}, {"ultramarine blue", 1}, {"zinc white", 2}}, 0.9); for i = 1, 6 do b:stroke({{100 + i * 110, 200}, {150 + i * 110, 60}}) end"#;

/// In a studio whose box file says sargent, a new painting's log names the
/// box; the plain replay build (no box file, no EASEL_BOX) replays it with
/// that box to the painter's picture, byte for byte, reopens it and checks
/// it. A different box configured where it is replayed is refused.
#[test]
fn a_new_painting_takes_the_studio_box_and_every_replay_uses_it() {
    let dir = base("studio");
    let e = easel_in(&dir, Some("sargent"));
    let _closing = Closing(e.clone(), dir.clone());
    assert!(ok(&e, &dir, None, &["tubes"]).lines().any(|l| l == "rose madder"));
    ok(&e, &dir, None, &["open", "p"]);
    ok(&e, &dir, None, &["do", CANVAS]);
    ok(&e, &dir, None, &["do", MADDER]);
    let e2 = fails(&e, &dir, None, &["do", r#"pile{{"smalt", 1}}"#]);
    assert!(e2.contains("no tube \"smalt\""), "{e2}");
    ok(&e, &dir, None, &["save", dir.join("saved.png").to_str().unwrap()]);
    ok(&e, &dir, None, &["close"]);
    let log = dir.join("paintings/lua/p.lua");
    let text = std::fs::read_to_string(&log).unwrap();
    assert_eq!(text.lines().nth(2), Some("--@ box sargent"), "{text}");

    // replayed elsewhere, by an easel with no box of its own
    let other = base("elsewhere");
    let out = other.join("replayed.png");
    ok(&plain(), &other, None, &["run", log.to_str().unwrap(), "--out", out.to_str().unwrap()]);
    assert!(std::fs::read(&out).unwrap() == std::fs::read(dir.join("saved.png")).unwrap(), "the replay differs from the painter's save");
    // reopened by it too (as scripts/check_painting does), and checked
    let _closing2 = Closing(plain(), dir.clone());
    assert!(ok(&plain(), &dir, None, &["open", "p"]).contains("resumed 2 chunks"));
    assert!(ok(&plain(), &dir, None, &["check"]).contains("replay matches the live canvas exactly"));
    ok(&plain(), &dir, None, &["close"]);

    // a conflicting box where it is replayed: an error, not a silent pick
    for args in [&["run", log.to_str().unwrap(), "--out", out.to_str().unwrap()][..], &["open", "p"]] {
        let err = fails(&plain(), &dir, Some("inness"), args);
        assert!(err.contains("painted from the box \"sargent\"") && err.contains("EASEL_BOX says \"inness\""), "{args:?}: {err}");
    }
    let other_studio = base("other-studio");
    let e3 = easel_in(&other_studio, Some("alma-tadema"));
    let err = fails(&e3, &dir, None, &["run", log.to_str().unwrap()]);
    assert!(err.contains("painted from the box \"sargent\"") && err.contains("bin/box says \"alma-tadema\""), "{err}");
    assert_eq!(std::fs::read_to_string(&log).unwrap(), text, "a refused reopen changed the log");
}

/// A log with no box line was painted from the default box (every log
/// before round 20): it replays with it, and a studio set to another box
/// refuses it. A new default painting's log names no box.
#[test]
fn a_log_without_a_box_line_is_the_default_box() {
    let dir = base("default");
    let e = easel_in(&dir, None);
    let _closing = Closing(e.clone(), dir.clone());
    ok(&e, &dir, None, &["open", "p"]);
    ok(&e, &dir, None, &["do", CANVAS]);
    ok(&e, &dir, None, &["close"]);
    let log = dir.join("paintings/lua/p.lua");
    let text = std::fs::read_to_string(&log).unwrap();
    assert!(!text.contains("--@ box"), "{text}");
    assert!(text.starts_with("-- easel session \"p\": a painting replayed chunk by chunk.\n-- Each \"--@ chunk\" line starts one chunk as it was run at the easel.\n--@ engine 2\n\n--@ chunk 1\n"), "{text}");
    let out = dir.join("x.png");
    ok(&plain(), &dir, None, &["run", log.to_str().unwrap(), "--out", out.to_str().unwrap()]);
    ok(&plain(), &dir, Some("tube box"), &["run", log.to_str().unwrap(), "--out", out.to_str().unwrap()]);
    let err = fails(&plain(), &dir, Some("sargent"), &["run", log.to_str().unwrap(), "--out", out.to_str().unwrap()]);
    assert!(err.contains("the log names no box, so it was painted from the default box \"tube box\"") && err.contains("EASEL_BOX says \"sargent\""), "{err}");
    let err = fails(&plain(), &dir, Some("sargent"), &["open", "p"]);
    assert!(err.contains("names no box"), "{err}");
}

/// A log from round 19 (no box line, the fourteen tubes) replays to the
/// picture and surface round 19's easel made of it (recorded with r19-base,
/// 5069814).
#[test]
fn a_round_19_log_replays_as_before() {
    let dir = base("r19");
    let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.lua");
    let (png, surf) = (dir.join("r19.png"), dir.join("r19.surf"));
    let out = ok(&plain(), &dir, None, &["run", fixture, "--out", png.to_str().unwrap(), "--dump-surface", surf.to_str().unwrap()]);
    assert!(out.starts_with("lead white, smalt, pale smalt, yellow ochre, red earth, vermilion, raw umber, bone black, cobalt blue, chrome yellow, Prussian blue, green earth, Rinmann's green, copper green\n"), "{out}");
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in std::fs::read(&png).unwrap().into_iter().chain(std::fs::read(&surf).unwrap()) {
        h ^= b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    let want = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.golden")).unwrap();
    assert_eq!(format!("{h:016x}"), want.trim(), "the round 19 log no longer replays as it did");
}

/// Where the box is set: the file next to the easel, else EASEL_BOX, else the
/// default; the two disagreeing, a name no box has, or a file that isn't one
/// line are errors. `tubes --markdown` prints the box as the guide's table.
#[test]
fn the_box_file_and_easel_box_must_agree_on_a_box_there_is() {
    let dir = base("config");
    let root = dir.join("root");
    let default = ok(&plain(), &root, None, &["tubes", "--markdown"]);
    assert!(default.starts_with("| tube | pigment | hiding | stiffness | tinting strength | drying |\n|---|---|---|---|---|---|\n| lead white | basic lead carbonate | 0.82 | 0.8 | 1.0 | 2.0 |\n"), "{default}");
    assert_eq!(default.lines().count(), 16);
    let inness = ok(&plain(), &root, Some("inness"), &["tubes", "--markdown"]);
    assert!(inness.contains("| Antwerp blue |") && !inness.contains("| smalt |"), "{inness}");
    let e = easel_in(&dir, Some("inness"));
    assert_eq!(ok(&e, &root, None, &["tubes", "--markdown"]), inness);
    assert_eq!(ok(&e, &root, Some("inness"), &["tubes", "--markdown"]), inness);
    let err = fails(&e, &root, Some("sargent"), &["tubes"]);
    assert!(err.contains("bin/box says box \"inness\" but EASEL_BOX says \"sargent\""), "{err}");
    let err = fails(&e, &root, Some("sargent"), &["open", "p"]);
    assert!(err.contains("EASEL_BOX says \"sargent\""), "{err}");
    std::fs::write(dir.join("bin/box"), "no such box\n").unwrap();
    let err = fails(&e, &root, None, &["tubes"]);
    assert!(err.contains("no box \"no such box\"") && err.contains("\"alma-tadema\""), "{err}");
    std::fs::write(dir.join("bin/box"), "inness\nsargent\n").unwrap();
    assert!(fails(&e, &root, None, &["tubes"]).contains("holds one line"));
    std::fs::write(dir.join("bin/box"), "\n").unwrap();
    assert!(fails(&e, &root, None, &["tubes"]).contains("holds one line"));
    assert!(fails(&plain(), &root, Some("x"), &["tubes"]).contains("no box \"x\""));
}

/// The replay build holds every box, the default tube box first (its
/// `all-boxes` turns on paint's, which keeps the default box in), and every
/// tube of the catalog is in one of them.
#[test]
fn the_replay_build_holds_every_box() {
    use paint::palette::{Palette, catalog};
    assert_eq!(Palette::box_names(), ["tube box", "sargent", "inness", "alma-tadema", "tonn", "hopper"]);
    let mut want: Vec<&str> = Palette::box_names().into_iter().flat_map(|b| Palette::named_box(b).unwrap().tubes.into_iter().map(|t| t.name)).collect();
    want.sort();
    want.dedup();
    let mut have: Vec<&str> = catalog().iter().map(|t| t.name).collect();
    have.sort();
    assert_eq!(have, want);
}
