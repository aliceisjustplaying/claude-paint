//! The painter build (`--no-default-features`): the easel shipped in a
//! studio as `<studio>/bin/easel`. Run from anywhere, it works in that
//! studio on its one painting; what the replay build adds (named sessions,
//! EASEL_ROOT, `run`, `check`) is not there.
//!
//!   cargo test -p easel --no-default-features --test painter
#![cfg(not(feature = "replay"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn base() -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("painter-{}", std::process::id()))
}
fn studio() -> PathBuf {
    base().join("studio")
}
/// An unrelated working directory that looks like a checkout, and where
/// EASEL_ROOT points: the painter build must use neither.
fn elsewhere() -> PathBuf {
    base().join("elsewhere")
}

fn easel(args: &[&str]) -> Output {
    Command::new(studio().join("bin/easel"))
        .args(args)
        .current_dir(elsewhere())
        .env("EASEL_ROOT", elsewhere())
        .env("EASEL_SESSION", "other")
        .output()
        .unwrap()
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

struct Closing;
impl Drop for Closing {
    fn drop(&mut self) {
        let _ = easel(&["close"]);
    }
}

const CANVAS: &str = r#"canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}"#;
const STROKE: &str = r#"b = brush("round", 4); b:load(pile{{"bone black", 1}}, 0.9); b:stroke({{100, 120}, {800, 140}})"#;

#[test]
fn the_shipped_easel_paints_its_one_painting_in_its_own_studio() {
    let _ = std::fs::remove_dir_all(base());
    for d in ["bin", "notes", "paintings/lua"] {
        std::fs::create_dir_all(studio().join(d)).unwrap();
    }
    std::fs::copy(env!("CARGO_BIN_EXE_easel"), studio().join("bin/easel")).unwrap();
    std::fs::create_dir_all(elsewhere().join("crates/easel")).unwrap();
    std::fs::write(elsewhere().join("crates/easel/Cargo.toml"), "").unwrap();
    let studio = studio().canonicalize().unwrap();
    let _closing = Closing;

    // the usage lists only what exists
    let usage = ok(&["help"]);
    for cmd in ["open", "do", "look", "log", "status", "save", "frames", "close", "note", "tubes"] {
        assert!(usage.contains(&format!("easel {cmd}")), "{cmd}: {usage}");
    }
    for gone in ["easel run", "easel check", "-s ", "EASEL_SESSION", "<name>", "wet"] {
        assert!(!usage.contains(gone), "{gone}: {usage}");
    }

    let r = ok(&["open"]);
    assert!(r.contains("\"painting\" open"), "{r}");
    assert!(ok(&["do", CANVAS]).contains("ok · chunk 1"));
    assert!(ok(&["do", STROKE]).contains("ok · chunk 2"));
    let look = ok(&["look"]);
    let look = PathBuf::from(look.split_whitespace().next().unwrap());
    assert!(look.starts_with(studio.join("out/easel/painting")) && look.exists(), "{}", look.display());
    ok(&["note", "a first line"]);
    let saved = ok(&["save"]);
    assert_eq!(saved.trim_end(), studio.join("out/easel/painting/painting.png").display().to_string());
    assert!(studio.join("out/easel/painting/painting.png").exists());

    // what the painter build doesn't have
    let log = studio.join("paintings/lua/painting.lua");
    let e = fails(&["run", log.to_str().unwrap()]);
    assert!(e.contains("no command \"run\""), "{e}");
    let e = fails(&["run", log.to_str().unwrap(), "--dump-surface", "x.bin"]);
    assert!(e.contains("no command \"run\""), "{e}");
    let e = fails(&["hash-probe"]);
    assert!(e.contains("no command \"hash-probe\""), "{e}");
    // replaying the log against the canvas is the runner's (scripts/check_painting), not the painter's
    let e = fails(&["check"]);
    assert!(e.contains("no command \"check\"") && !e.contains("easel check"), "{e}");
    for args in [&["-s", "painting", "status"][..], &["--session", "painting", "status"], &["status", "-s", "painting"]] {
        let e = fails(args);
        assert!(e.contains("one painting"), "{args:?}: {e}");
    }
    let e = fails(&["open", "othername"]);
    assert!(e.contains("takes no name"), "{e}");
    let e = fails(&["look", "--mode", "wet"]);
    assert!(e.contains("--mode wet:"), "{e}");
    assert!(ok(&["status"]).starts_with("2 chunks"));
    let r = ok(&["close"]);
    assert!(r.contains(&log.display().to_string()), "{r}");

    // everything landed in the studio; nothing where EASEL_ROOT and the cwd point
    assert!(std::fs::read_to_string(&log).unwrap().contains(STROKE));
    let journal = std::fs::read_to_string(studio.join("notes/journal.md")).unwrap();
    assert!(journal.lines().any(|l| l.ends_with(": a first line")), "{journal}");
    let mut stray: Vec<_> = std::fs::read_dir(elsewhere()).unwrap().map(|e| e.unwrap().file_name()).collect();
    stray.sort();
    assert_eq!(stray, ["crates"], "the easel wrote outside its studio");
    assert!(!studio.join("out/easel/other").exists() && !studio.join("paintings/lua/other.lua").exists());
}

/// A studio's `bin/box` sets the box its painting is painted from. A painter
/// build knows only the boxes it was built with (`box-*` features): the
/// export builds each studio's easel with its own box and no other.
#[test]
fn the_studio_box_file_sets_the_box() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("painter-box-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    let e = dir.join("bin/easel");
    std::fs::copy(env!("CARGO_BIN_EXE_easel"), &e).unwrap();
    std::fs::write(dir.join("bin/box"), "sargent\n").unwrap();
    let run = |args: &[&str]| Command::new(&e).args(args).current_dir(&dir).env_remove("EASEL_BOX").output().unwrap();
    let usage = String::from_utf8(run(&["help"]).stdout).unwrap();
    assert!(usage.contains("easel tubes"), "{usage}");
    #[cfg(feature = "box-sargent")]
    {
        struct Close<'a>(&'a dyn Fn(&[&str]) -> Output);
        impl Drop for Close<'_> {
            fn drop(&mut self) {
                let _ = (self.0)(&["close"]);
            }
        }
        let _close = Close(&run);
        let tubes = String::from_utf8(run(&["tubes"]).stdout).unwrap();
        assert!(tubes.lines().any(|l| l == "rose madder") && !tubes.lines().any(|l| l == "smalt"), "{tubes}");
        assert!(run(&["open"]).status.success());
        let o = run(&["do", r#"canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{"lead white", 5}, {"rose madder", 1}}, um=80, apply="knife"}}}"#]);
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        assert!(run(&["close"]).status.success());
        let log = std::fs::read_to_string(dir.join("paintings/lua/painting.lua")).unwrap();
        assert_eq!(log.lines().nth(2), Some("--@ box sargent"), "{log}");
    }
    #[cfg(not(feature = "box-sargent"))]
    {
        let o = run(&["open"]);
        assert!(!o.status.success());
        let err = String::from_utf8_lossy(&o.stdout).to_string() + &String::from_utf8_lossy(&o.stderr);
        assert!(err.contains("no box \"sargent\" (its boxes: \"tube box\")"), "{err}");
        assert!(!dir.join("paintings/lua/painting.lua").exists());
    }
}
