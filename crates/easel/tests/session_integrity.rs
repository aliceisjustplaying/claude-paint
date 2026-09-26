//! Transport/lifecycle contracts without allocating a live canvas.
use std::{
    path::PathBuf,
    process::{Command, Output},
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("integrity-live-{}", std::process::id()))
}
fn cmd(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", root()).env("EASEL_SESSION", "s").output().unwrap()
}
fn ok(args: &[&str]) -> String {
    let o = cmd(args);
    assert!(o.status.success(), "{args:?}: {}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
#[test]
fn live_resolution_is_fixed() {
    let o = cmd(&["open", "s", "--width", "80"]);
    assert!(!o.status.success(), "live width override accepted");
    assert!(ok(&["open", "s"]).contains("2400px"));
    ok(&["close"]);
}

#[test]
fn edited_or_truncated_logs_cannot_reopen() {
    ok(&["open", "reopen"]);
    ok(&["-s", "reopen", "do", "x = 1"]);
    ok(&["-s", "reopen", "close"]);
    let log = root().join("paintings/lua/reopen.lua");
    let original = std::fs::read(&log).unwrap();
    for bytes in [String::from_utf8(original.clone()).unwrap().replace("x = 1", "x = 2").into_bytes(), Vec::new()] {
        std::fs::write(&log, bytes).unwrap();
        let o = cmd(&["open", "reopen"]);
        assert!(!o.status.success());
        assert!(String::from_utf8_lossy(&o.stderr).contains("integrity"));
    }
    std::fs::write(&log, original).unwrap();
    assert!(ok(&["open", "reopen"]).contains("resumed 1 chunks"));
    ok(&["-s", "reopen", "close"]);
}

// A corrupt/obsolete chunk must never leave a partially resumed server running.
#[test]
fn replay_failure_is_fatal() {
    let name = "bad-replay";
    let dir = root().join("out/easel").join(name);
    std::fs::create_dir_all(&dir).unwrap();
    let logs = root().join("paintings/lua");
    std::fs::create_dir_all(&logs).unwrap();
    let text = "--@ chunk 1\nx = 1\n--@ chunk 2\nerror('cannot replay')\n--@ chunk 3\nx = 3\n";
    std::fs::write(logs.join("bad-replay.lua"), text).unwrap();
    // Matching local witness isolates replay failure from edit detection.
    std::fs::write(dir.join("committed.lua"), text).unwrap();
    let o = cmd(&["open", name]);
    assert!(!o.status.success(), "partial replay was accepted");
    assert!(String::from_utf8_lossy(&o.stderr).contains("replay failed at chunk 2"));
    assert_eq!(std::fs::read_to_string(logs.join("bad-replay.lua")).unwrap(), text);
    assert!(!cmd(&["-s", name, "status"]).status.success());
}

#[test]
fn journal_uses_the_selected_painting_clock() {
    ok(&["open", "journal"]);
    let result = cmd(&["-s", "journal", "note", "painting clock"]);
    ok(&["-s", "journal", "close"]);
    assert!(result.status.success());
    let journal = std::fs::read_to_string(root().join("notes/journal.md")).unwrap();
    assert!(journal.lines().any(|l| l == "- day 1, 09:00: painting clock"), "{journal}");
}
