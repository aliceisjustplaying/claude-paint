//! A short session at the easel through its commands: open, knife two
//! piles, lay strokes and a passage, wait, look (whole and cropped), check
//! that the log replays exactly, save and replay it, keep a journal. What
//! the easel doesn't have errors clearly and changes nothing.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn root() -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-smoke")
}

fn easel(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", root()).env_remove("EASEL_SESSION").output().unwrap()
}

fn ok(args: &[&str]) -> String {
    let o = easel(args);
    assert!(o.status.success(), "easel {args:?} failed:\n{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

fn fails(args: &[&str]) -> String {
    let o = easel(args);
    assert!(!o.status.success(), "easel {args:?} should fail:\n{}", String::from_utf8_lossy(&o.stdout));
    String::from_utf8(o.stderr).unwrap()
}

/// Closes the session however the test ends.
struct Closing;
impl Drop for Closing {
    fn drop(&mut self) {
        let _ = easel(&["-s", "smoke", "close"]);
    }
}

const CANVAS: &str = r#"canvas{size=300, aspect=5, linen={16, 14}, seed=3, ground={{pile={{"lead white", 2}, {"red earth", 1}}, um=100, apply="knife"}, {pile={{"lead white", 6}, {"raw umber", 1}}, um=50, apply="brush"}}}"#;

const PAINT: &str = r#"
a = pile{{"lead white", 6}, {"yellow ochre", 2}, {"vermilion", 0.3}, medium=0.2}
c = pile{{"lead white", 4}, {"cobalt blue", 1}, medium=0.35}
print(a, c)
work(rect(100, 100, 300, 250), {hand="body", pile=c, coverage=3})
b = brush("filbert", 8)
b:load(a, 0.9)
for i = 0, 5 do b:stroke({{520, 120 + 40 * i}, {880, 140 + 40 * i}}, {pressure={0.8, 0.3}}) end
print(wait(24 * 60))
print(drying(300, 200))
"#;

#[test]
fn a_short_session_at_the_easel() {
    let _ = std::fs::remove_dir_all(root());
    std::fs::create_dir_all(root().join("notes")).unwrap();
    let _closing = Closing;
    ok(&["open", "smoke"]);
    let r = ok(&["-s", "smoke", "do", CANVAS]);
    assert!(r.contains("ok · chunk 1"), "{r}");
    let r = ok(&["-s", "smoke", "do", PAINT]);
    assert!(r.contains("pile(lead white 6, yellow ochre 2, vermilion 0.3; medium 0.2)"), "{r}");
    assert!(r.contains("day 2, "), "wait returns the time of day: {r}");
    assert!(r.contains("ok · chunk 2"), "{r}");

    // looking: the whole canvas, and a window with a grid
    let look = ok(&["-s", "smoke", "look"]);
    let path = look.split_whitespace().next().unwrap();
    assert!(Path::new(path).exists(), "{look}");
    let look = ok(&["-s", "smoke", "look", "--crop", "450,20,950,180", "--grid", "50", "--mode", "value,squint"]);
    assert!(Path::new(look.split_whitespace().next().unwrap()).exists(), "{look}");

    // not in the easel: every one errors, and the log keeps its two chunks
    for cmd in ["undo", "try", "show", "edit", "undone", "redo"] {
        let e = fails(&["-s", "smoke", cmd, "1"]);
        assert!(e.contains(&format!("no command \"{cmd}\"")), "{cmd}: {e}");
    }
    for arg in ["--dried", "--probe", "--show", "--scale"] {
        let e = fails(&["-s", "smoke", "look", arg, "1"]);
        assert!(e.contains("unknown argument"), "{arg}: {e}");
    }
    for chunk in [
        "dry()",
        r##"x = mix("#334455", "#887766", 0.5)"##,
        "x = sample(500, 400)",
        "x = probe(500, 400)",
        "show(ellipse(500, 400, 50))",
        r##"b:load("#8090a0", 0.8)"##,
        r##"work(rect(0, 0, 100, 100), {hand="body", color="#8090a0"})"##,
        r##"work(rect(0, 0, 100, 100), {hand="body", pile=a, color=function(x, y) return "#ffffff" end})"##,
    ] {
        let e = fails(&["-s", "smoke", "do", chunk]);
        assert!(e.contains("the chunk failed and changed nothing"), "{chunk}: {e}");
    }
    // removed from the painter's API: a poured glaze (glazes are brushed:
    // work(m, {hand="glaze", ...})), the drawing mask, the false-color
    // drying look, and waits past 10 years
    for (chunk, want) in [
        ("glaze(rect(0, 0, 10, 10), {pile=a})", "global 'glaze'"),
        ("m = drawing_mask()", "global 'drawing_mask'"),
        ("wait(5259601)", "10 years"),
        ("wait(1e100)", "10 years"),
    ] {
        let e = fails(&["-s", "smoke", "do", chunk]);
        assert!(e.contains(want) && e.contains("the chunk failed and changed nothing"), "{chunk}: {e}");
    }
    // finishing is applied after the session (scripts/finish_painting): the
    // painter build (`--no-default-features`; test it with `--features replay`)
    // has no finishing verbs
    #[cfg(not(feature = "finish"))]
    for verb in ["varnish", "cracks", "relief"] {
        let e = fails(&["-s", "smoke", "do", &format!("{verb}()")]);
        assert!(e.contains(&format!("global '{verb}'")) && e.contains("the chunk failed and changed nothing"), "{verb}: {e}");
    }
    for mode in ["wet", "drying", "stages"] {
        let e = fails(&["-s", "smoke", "look", "--mode", mode]);
        assert!(e.contains(&format!("--mode {mode}:")), "--mode {mode}: {e}");
    }
    assert!(ok(&["-s", "smoke", "status"]).starts_with("2 chunks"));

    // the log replays to the live canvas exactly, in the session and from the file
    let r = ok(&["-s", "smoke", "check"]);
    assert!(r.contains("replay matches the live canvas exactly (2 chunks"), "{r}");
    let saved = root().join("saved.png");
    ok(&["-s", "smoke", "save", saved.to_str().unwrap()]);
    ok(&["-s", "smoke", "close"]);
    let log = root().join("paintings/lua/smoke.lua");
    let replayed = root().join("replayed.png");
    ok(&["run", log.to_str().unwrap(), "--out", replayed.to_str().unwrap()]);
    assert!(std::fs::read(&saved).unwrap() == std::fs::read(&replayed).unwrap(), "the replay's PNG differs from the session's");

    // reopening goes on from the log: a chunk that ran is there for good
    let r = ok(&["open", "smoke"]);
    assert!(r.contains("resumed 2 chunks"), "{r}");
    // the journal only grows
    ok(&["note", "first entry"]);
    ok(&["note", "second entry", "in two words"]);
    let j = std::fs::read_to_string(root().join("notes/journal.md")).unwrap();
    let lines: Vec<&str> = j.lines().collect();
    assert_eq!(lines.len(), 2, "{j}");
    assert!(lines[0].starts_with("- day 2, ") && lines[0].ends_with(": first entry"), "{j}");
    assert!(lines[1].ends_with(": second entry in two words"), "{j}");
}
