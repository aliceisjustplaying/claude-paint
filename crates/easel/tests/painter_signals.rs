//! What the easel tells the painter that it used to leave silent: a full palette refuses a new
//! pile and names the piles on it, a running chunk's progress answers `status`, and so does a
//! reopen's replay of the log.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
use std::{
    path::PathBuf,
    process::{Command, Output},
    time::{Duration, Instant},
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("signals-{}", std::process::id()))
}
fn cmd(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").env("EASEL_ROOT", root()).env_remove("EASEL_SESSION").output().unwrap()
}
fn ok(args: &[&str]) -> String {
    let o = cmd(args);
    assert!(o.status.success(), "{args:?}: {}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
fn fails(args: &[&str]) -> String {
    let o = cmd(args);
    assert!(!o.status.success(), "{args:?} should fail: {}", String::from_utf8_lossy(&o.stdout));
    String::from_utf8(o.stderr).unwrap()
}

/// Closes a session however the test ends.
struct Closing(&'static str);
impl Drop for Closing {
    fn drop(&mut self) {
        let _ = cmd(&["-s", self.0, "close"]);
    }
}

const CANVAS: &str = r#"canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}"#;
const SIXTEEN: &str = r#"p = {} for i = 1, 16 do p[i] = pile{{"lead white", i}, {"vermilion", 1}, name="mix" .. i} end"#;
const SEVENTEENTH: &str = r#"q = pile{{"lead white", 1}, {"smalt", 1}}"#;

#[test]
fn a_full_palette_refuses_a_new_pile_and_names_the_piles_on_it() {
    let s = "full";
    let with = |a: &[&'static str]| [&["-s", s][..], a].concat();
    let _closing = Closing(s);
    ok(&["open", s]);
    ok(&with(&["do", CANVAS]));
    ok(&with(&["do", SIXTEEN]));
    let log = root().join("paintings/lua/full.lua");
    assert!(std::fs::read_to_string(&log).unwrap().lines().any(|l| l == "--@ full-palette-refuses"), "a new painting's log names the rule");
    let before = std::fs::read(&log).unwrap();
    let e = fails(&with(&["do", SEVENTEENTH]));
    assert!(e.contains("pile: the palette is full. It holds 16 piles"), "{e}");
    for i in 1..=16 {
        assert!(e.contains(&format!("\"mix{i}\" (")), "mix{i} is named: {e}");
    }
    assert!(e.contains("palette{scrape={p1, p2}}") && e.contains("the chunk failed and changed nothing"), "{e}");
    assert_eq!(std::fs::read(&log).unwrap(), before, "the refused chunk isn't in the log");
    // nothing was scraped off: every pile is still on the palette
    let board = ok(&with(&["do", "print(palette())"]));
    assert!(board.contains("16 heap(s)") && board.contains("1. mix1:"), "{board}");
    // scraped off, two piles make room for a new one; a scraped pile used again needs room too
    let scrape = format!("palette{{scrape={{p[1], p[2]}}}} {SEVENTEENTH} print(palette())");
    let r = ok(&["-s", s, "do", &scrape]);
    assert!(r.contains("15 heap(s)") && !r.contains("mix1:") && !r.contains("mix2:"), "{r}");
    ok(&with(&["do", r#"b = brush("filbert", 8) b:load(p[1], 0.9)"#]));
    let e = fails(&with(&["do", "b:load(p[2], 0.9)"]));
    assert!(e.contains("b:load: that pile was scraped off, and knifing it again needs room: the palette is full"), "{e}");
    // the log replays as painted
    let r = ok(&with(&["check"]));
    assert!(r.contains("replay matches the live canvas within tolerance"), "{r}");
}

#[test]
fn an_older_log_still_scrapes_the_oldest_pile_off() {
    let s = "older";
    let _closing = Closing(s);
    let dir = root().join("out/easel").join(s);
    std::fs::create_dir_all(&dir).unwrap();
    let logs = root().join("paintings/lua");
    std::fs::create_dir_all(&logs).unwrap();
    // engine 7 without the rule's head line: a 17th pile scrapes off the oldest, and using
    // the oldest again knifes it fresh, as when it was painted
    let text = format!("-- easel session \"{s}\": a painting replayed chunk by chunk.\n-- Each \"--@ chunk\" line starts one chunk as it was run at the easel.\n--@ engine 7\n--@ limbs-join-nearest\n\n--@ chunk 1\n{CANVAS}\n\n--@ chunk 2\n{SIXTEEN} {SEVENTEENTH} b = brush(\"filbert\", 8) b:load(p[1], 0.9)\n");
    std::fs::write(logs.join(format!("{s}.lua")), &text).unwrap();
    std::fs::write(dir.join("committed.lua"), &text).unwrap();
    let r = ok(&["open", s]);
    assert!(r.contains("resumed 2 chunks"), "{r}");
    let board = ok(&["-s", s, "do", "print(palette())"]);
    assert!(board.contains("16 heap(s)") && !board.contains("mix2:") && board.contains("16. mix1:"), "{board}");
    let r = ok(&["-s", s, "do", "x = pile{{\"smalt\", 1}}"]);
    assert!(r.contains("ok · chunk 4"), "{r}");
}

#[test]
fn status_answers_a_running_chunks_progress_and_other_requests_wait_their_turn() {
    let s = "progress";
    let with = |a: &[&'static str]| [&["-s", s][..], a].concat();
    let _closing = Closing(s);
    ok(&["open", s]);
    ok(&with(&["do", CANVAS]));
    let long = "a = pile{{\"lead white\", 3}, {\"smalt\", 1}}\nb = brush(\"round\", 4)\nfor i = 1, 12 do b:load(a, 0.8) b:stroke({{100, 20 * i}, {600, 20 * i}}) end\nlocal sum = 0\nfor i = 1, 400000000 do sum = sum + i end\nassert(sum > 0)";
    let painting = std::thread::spawn(move || ok(&["-s", "progress", "do", long]));
    let deadline = Instant::now() + Duration::from_secs(60);
    let seen = loop {
        assert!(Instant::now() < deadline, "status never answered with the chunk's progress");
        let st = ok(&with(&["status"]));
        if st.contains("at line 5 of 6") {
            break st;
        }
        assert!(st.starts_with("painting a chunk") || st.starts_with("1 chunks"), "{st}");
        std::thread::sleep(Duration::from_millis(20));
    };
    assert!(seen.starts_with("painting a chunk (at line 5 of 6): 12 strokes and touches so far, "), "{seen}");
    assert!(seen.contains(" on the painting's clock (now day 1, ") && seen.contains(", running for "), "{seen}");
    // a chunk sent while one runs waits, and runs after it
    let next = ok(&with(&["do", "x = 1"]));
    assert!(next.contains("ok · chunk 3"), "{next}");
    assert!(painting.join().unwrap().contains("ok · chunk 2"));
    assert!(ok(&with(&["status"])).starts_with("3 chunks"));
}

#[test]
fn a_reopen_answers_status_with_its_replay_and_requests_wait_for_it() {
    use std::process::{Child, Stdio};
    struct Server(Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let s = "reopen";
    let with = |a: &[&'static str]| [&["-s", s][..], a].concat();
    ok(&["open", s]);
    for _ in 0..3 {
        ok(&with(&["do", "local sum = 0; for i = 1, 150000000 do sum = sum + i end; assert(sum > 0)"]));
    }
    ok(&with(&["close"]));
    let _server = Server(Command::new(env!("CARGO_BIN_EXE_easel")).args(["serve", s]).env_remove("EASEL_WIDTH").env("EASEL_ROOT", root()).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap());
    let progress = |st: &str| st.strip_prefix("rebuilding from the log (").and_then(|t| t.strip_suffix(" of 3 chunks)\n")).map(|k| k.parse::<usize>().unwrap());
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut saw = None;
    while saw.is_none_or(|k| k == 0) {
        assert!(Instant::now() < deadline, "status never answered with the replay's progress (last {saw:?})");
        let o = cmd(&with(&["status"]));
        let st = String::from_utf8_lossy(&o.stdout).to_string();
        if let Some(k) = progress(&st) {
            assert!(k < 3);
            saw = Some(k);
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    // a chunk sent during the replay waits for it, then runs on the reopened painting
    let r = ok(&with(&["do", "x = 1"]));
    assert!(r.contains("ok · chunk 4"), "{r}");
    assert!(ok(&with(&["status"])).starts_with("4 chunks"));
    ok(&with(&["close"]));
}
