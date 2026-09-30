//! Transport/lifecycle contracts without allocating a live canvas.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]
use std::{
    path::PathBuf,
    process::{Command, Output},
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("il-{}", std::process::id()))
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
        assert!(String::from_utf8_lossy(&o.stderr).contains("The easel goes on only from the log it wrote"));
    }
    std::fs::write(&log, original).unwrap();
    let reopened = ok(&["open", "reopen"]);
    assert!(reopened.contains("resumed chunk 1/1"), "{reopened}");
    assert!(reopened.contains("resumed 1 chunks"), "{reopened}");
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

// A chunk that ran is in the log even when the look asked for after it fails: the reply
// says both, so the chunk isn't sent again.
#[test]
fn a_failed_look_after_a_chunk_still_reports_the_chunk() {
    ok(&["open", "looked"]);
    let out = ok(&["-s", "looked", "do", "x = 1", "--look"]);
    ok(&["-s", "looked", "close"]);
    assert!(out.contains("ok · chunk 1") && out.contains("no canvas yet"), "{out}");
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

// Every request to a running session checks the log first: an edited,
// truncated or deleted log stops them all before anything runs, and the
// genuine bytes put back let the session go on unchanged.
#[test]
fn a_running_session_refuses_every_request_while_its_log_is_edited() {
    let s = ["-s", "gate"];
    let with = |a: &[&'static str]| [&s[..], a].concat();
    ok(&["open", "gate"]);
    ok(&with(&["do", "x = 1"]));
    let log = root().join("paintings/lua/gate.lua");
    let original = std::fs::read(&log).unwrap();
    for edit in [Some(b"-- edited\n".to_vec()), Some(Vec::new()), None] {
        match edit {
            Some(bytes) => std::fs::write(&log, bytes).unwrap(),
            None => std::fs::remove_file(&log).unwrap(),
        }
        for req in [&["do", "x = 2"][..], &["status"], &["log"], &["look"], &["save"], &["check"], &["frames", "on"], &["note", "edited"], &["close"]] {
            let o = cmd(&with(req));
            assert!(!o.status.success(), "{req:?} ran over an edited log");
            assert!(String::from_utf8_lossy(&o.stderr).contains("The easel goes on only from the log it wrote"), "{req:?}: {}", String::from_utf8_lossy(&o.stderr));
        }
        std::fs::write(&log, &original).unwrap();
    }
    assert!(ok(&with(&["status"])).starts_with("1 chunks"));
    ok(&with(&["do", "assert(x == 1)"]));
    assert!(!root().join("notes/journal.md").exists() || !std::fs::read_to_string(root().join("notes/journal.md")).unwrap().contains("edited"));
    ok(&with(&["close"]));
}

// Unlike the Session rollback test, this exercises concurrent clients of the real
// server: replay must not occupy the only listener or silently queue mutations.
#[test]
fn rebuilding_serves_progress_and_refuses_nonstatus_requests() {
    use std::{
        io::{Read, Write},
        os::unix::net::UnixStream,
        process::{Child, Stdio},
        time::{Duration, Instant},
    };
    struct Server(Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let name = "rebuild-boundary";
    let dir = root().join("out/easel").join(name);
    std::fs::create_dir_all(&dir).unwrap();
    let socket = dir.join("sock");
    let mut server = Server(Command::new(env!("CARGO_BIN_EXE_easel"))
        .args(["serve", name])
        .env("EASEL_ROOT", root())
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().unwrap());
    let start = Instant::now();
    while !socket.exists() {
        assert!(server.0.try_wait().unwrap().is_none(), "server exited at startup");
        assert!(start.elapsed() < Duration::from_secs(10), "server did not bind");
        std::thread::sleep(Duration::from_millis(10));
    }
    let request = |head: &str, payload: &str, timeout: Duration| {
        let mut conn = UnixStream::connect(&socket).unwrap();
        conn.set_read_timeout(Some(timeout)).unwrap();
        conn.set_write_timeout(Some(timeout)).unwrap();
        let body = format!("{head}\n{payload}");
        write!(conn, "{}\n{body}", body.len()).unwrap();
        let mut reply = String::new();
        conn.read_to_string(&mut reply).unwrap_or_else(|e| panic!("{head} did not answer within {timeout:?}: {e}; partial reply: {reply:?}"));
        reply
    };
    let normal = Duration::from_secs(60);
    let prompt = Duration::from_secs(1);
    assert!(request("do", "t = {}; for i = 1, 30 do t['k' .. i] = i end", normal).starts_with("ok\n"));
    // CPU only, fixed bounds and constant memory. Three slow committed chunks
    // leave time to observe both zero and intermediate replay progress.
    for _ in 0..3 {
        assert!(request("do", "local sum = 0; for i = 1, 150000000 do sum = sum + i end; assert(sum > 0)", normal).starts_with("ok\n"));
    }
    let log = root().join("paintings/lua").join(format!("{name}.lua"));
    let original = std::fs::read(&log).unwrap();
    let failed = request("do", "for i = 1, 1000 do t['x' .. i] = i end; for i = 1, 1000 do t['x' .. i] = nil end; error('force rebuild')", normal);
    assert!(failed.starts_with("err\n") && failed.contains("force rebuild") && failed.contains("rebuild"), "{failed}");
    let progress = |reply: &str| -> Option<usize> {
        reply.strip_prefix("ok\nrebuilding from the log (")
            .and_then(|s| s.strip_suffix(" of 4 chunks)\n"))
            .map(|k| k.parse::<usize>().expect("numeric completed-chunk count"))
    };
    let first = request("status", "", prompt);
    let mut previous = progress(&first).unwrap_or_else(|| panic!("expected rebuilding progress, got {first:?}"));
    assert!(previous < 4, "rebuild finished before concurrent requests");
    // These requests have observable side effects if queued instead of refused;
    // include check, whose server dispatch bypasses ordinary handle().
    for (head, payload) in [("do", "t.unexpected = true"), ("note", "must not be journaled"), ("frames\ton", ""), ("check", ""), ("close", "")] {
        assert_eq!(request(head, payload, prompt), "err\nthe easel is rebuilding from the log; retry after it finishes\n", "{head}");
    }
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut saw_intermediate = false;
    loop {
        assert!(Instant::now() < deadline, "rebuild did not finish");
        let reply = request("status", "", prompt);
        if let Some(k) = progress(&reply) {
            assert!(k >= previous && k <= 4, "progress regressed or exceeded total: {reply:?}");
            saw_intermediate |= k > 1 && k < 4;
            previous = k;
        } else {
            assert_eq!(reply, "ok\n4 chunks · 2400px · no canvas yet\n");
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(saw_intermediate, "never observed a completed slow chunk during rebuild");
    assert_eq!(std::fs::read(&log).unwrap(), original, "refused requests or failed chunk changed the log");
    assert_eq!(std::fs::read(dir.join("committed.lua")).unwrap(), original);
    assert!(!root().join("notes/journal.md").exists() || !std::fs::read_to_string(root().join("notes/journal.md")).unwrap().contains("must not be journaled"));
    assert!(request("do", "assert(t.unexpected == nil); local n = 0; for _ in pairs(t) do n = n + 1 end; assert(n == 30)", normal).starts_with("ok\n"));
    assert!(request("close", "", normal).starts_with("ok\n"));
}
