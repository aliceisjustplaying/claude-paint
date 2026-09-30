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

// A look names its file one above the highest look-NNNN.png there, and never
// writes over a file that exists: a gap (look-0002 removed) or a stray
// look-prefixed file must not make the next look land on an older observation.
#[test]
fn a_look_never_overwrites_an_earlier_observation() {
    let s = ["-s", "gaps"];
    let with = |a: &[&'static str]| [&s[..], a].concat();
    ok(&["open", "gaps"]);
    // a failing assertion must not leave the server running
    struct Closing;
    impl Drop for Closing {
        fn drop(&mut self) {
            let _ = cmd(&["-s", "gaps", "close"]);
        }
    }
    let _closing = Closing;
    ok(&with(&["do", r#"canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}"#]));
    let dir = root().join("out/easel/gaps");
    for n in 1..=2 {
        let l = ok(&with(&["look", "--size", "40"]));
        assert!(l.starts_with(dir.join(format!("look-{n:04}.png")).to_str().unwrap()), "{l}");
    }
    // look-0001 pruned, look-0002 kept (marked), and an unrelated look-prefixed file
    std::fs::remove_file(dir.join("look-0001.png")).unwrap();
    std::fs::write(dir.join("look-0002.png"), b"kept observation").unwrap();
    let next = ok(&with(&["look", "--size", "40"]));
    assert!(next.starts_with(dir.join("look-0003.png").to_str().unwrap()), "{next}");
    std::fs::write(dir.join("look-notes.txt"), b"not an observation").unwrap();
    let after = ok(&with(&["look", "--size", "40"]));
    ok(&with(&["close"]));
    assert!(after.starts_with(dir.join("look-0004.png").to_str().unwrap()), "{after}");
    assert_eq!(std::fs::read(dir.join("look-0002.png")).unwrap(), b"kept observation", "a look overwrote an earlier one");
    assert_eq!(std::fs::read(dir.join("look-notes.txt")).unwrap(), b"not an observation");
}

// The servers running for a session (this test binary's, by its session name).
fn servers(name: &str) -> Vec<u32> {
    let o = Command::new("pgrep").args(["-f", &format!("^{} serve {name}$", env!("CARGO_BIN_EXE_easel"))]).output().unwrap();
    String::from_utf8_lossy(&o.stdout).lines().filter_map(|l| l.trim().parse().ok()).collect()
}

// Kills what a failed assertion would leave running.
struct Reaper(&'static str);
impl Drop for Reaper {
    fn drop(&mut self) {
        for pid in servers(self.0) {
            let _ = Command::new("kill").arg(pid.to_string()).status();
        }
    }
}

// One session has one server for good: opens racing to start it, while its log replays,
// all end up talking to the one that won, and no second server starts, even for a moment.
#[test]
fn racing_opens_start_one_server() {
    for round in 0..3 {
        let name: &'static str = Box::leak(format!("race{round}").into_boxed_str());
        let _reaper = Reaper(name);
        // a log that takes a while to replay
        ok(&["open", name]);
        ok(&["-s", name, "do", "local sum = 0; for i = 1, 100000000 do sum = sum + i end; assert(sum > 0)"]);
        ok(&["-s", name, "close"]);
        let racers: Vec<_> = (0..8)
            .map(|_| Command::new(env!("CARGO_BIN_EXE_easel")).args(["open", name]).env("EASEL_ROOT", root()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap())
            .collect();
        let mut most = 0;
        let watch = std::time::Instant::now();
        while watch.elapsed() < std::time::Duration::from_secs(3) {
            most = most.max(servers(name).len());
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        let outs: Vec<_> = racers.into_iter().map(|c| c.wait_with_output().unwrap()).collect();
        let failed: Vec<_> = outs.iter().filter(|o| !o.status.success()).map(|o| String::from_utf8_lossy(&o.stderr).to_string()).collect();
        assert!(failed.is_empty(), "round {round}: opens failed: {failed:?}");
        assert_eq!(most, 1, "round {round}: servers running at once for one session");
        assert_eq!(servers(name).len(), 1, "round {round}");
        assert!(ok(&["-s", name, "status"]).starts_with("1 chunks"), "round {round}");
        ok(&["-s", name, "close"]);
    }
}

// A second server for a session that has one is refused before it touches anything, and
// a server that died without closing (its socket left behind) is replaced by the next open.
#[test]
fn a_second_server_is_refused_and_a_dead_ones_socket_is_recovered() {
    use std::os::unix::fs::MetadataExt;
    let name = "owner";
    let _reaper = Reaper(name);
    ok(&["open", name]);
    ok(&["-s", name, "do", "x = 1"]);
    let sock = root().join("out/easel/owner/sock");
    let ino = std::fs::metadata(&sock).unwrap().ino();
    let o = Command::new(env!("CARGO_BIN_EXE_easel")).args(["serve", name]).env("EASEL_ROOT", root()).output().unwrap();
    assert!(!o.status.success() && String::from_utf8_lossy(&o.stderr).contains("another easel serves"), "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(std::fs::metadata(&sock).unwrap().ino(), ino, "the second server replaced the socket");
    assert!(ok(&["-s", name, "status"]).starts_with("1 chunks"));
    let pids = servers(name);
    assert_eq!(pids.len(), 1);
    Command::new("kill").args(["-9", &pids[0].to_string()]).status().unwrap();
    let t0 = std::time::Instant::now();
    while !servers(name).is_empty() {
        assert!(t0.elapsed() < std::time::Duration::from_secs(10));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(sock.exists(), "a killed server leaves its socket");
    assert!(ok(&["open", name]).contains("open: 1 chunks"));
    assert_eq!(servers(name).len(), 1);
    ok(&["-s", name, "do", "assert(x == 1)"]);
    ok(&["-s", name, "close"]);
}
