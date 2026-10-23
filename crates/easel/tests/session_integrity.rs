//! Transport/lifecycle contracts without allocating a live canvas.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};
mod support;
use support::sock_of;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("il-{}", std::process::id()))
}
/// Stops the session's server however the test ends.
fn closing(name: &str) -> support::Closing {
    support::Closing(root().join("out/easel").join(name))
}
fn cmd(args: &[&str]) -> Output {
    cmd_in(&root(), args)
}
fn ok(args: &[&str]) -> String {
    ok_in(&root(), args)
}
fn cmd_in(studio: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env_remove("EASEL_WIDTH").env("EASEL_ROOT", studio).env("EASEL_SESSION", "s").output().unwrap()
}
fn ok_in(studio: &Path, args: &[&str]) -> String {
    let o = cmd_in(studio, args);
    assert!(o.status.success(), "{args:?}: {}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
fn status_with_timeout(name: &str) -> std::io::Result<String> {
    use std::{io::{Read, Write}, os::unix::net::UnixStream, time::Duration};
    let mut conn = UnixStream::connect(sock_of(&root().join("out/easel").join(name)))?;
    conn.set_read_timeout(Some(Duration::from_secs(1)))?;
    conn.set_write_timeout(Some(Duration::from_secs(1)))?;
    conn.write_all(b"7\nstatus\n")?;
    let mut reply = String::new();
    conn.read_to_string(&mut reply)?;
    Ok(reply)
}
#[test]
fn live_resolution_is_fixed() {
    let _closing = closing("s");
    let o = cmd(&["open", "s", "--width", "80"]);
    assert!(!o.status.success(), "live width override accepted");
    assert!(ok(&["open", "s"]).contains("2400px"));
    ok(&["close"]);
}

#[test]
fn edited_or_truncated_logs_cannot_reopen() {
    let _closing = closing("reopen");
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
    assert!(reopened.contains("reconstructed 1 chunks"), "{reopened}");
    assert!(reopened.contains("resumed 1 chunks"), "{reopened}");
    ok(&["-s", "reopen", "close"]);
}

#[test]
fn open_waits_for_previous_lock_holder_despite_stale_fatal() {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;

    let name = "stale-fatal";
    let _reaper = Reaper(name);
    let dir = root().join("out/easel").join(name);
    std::fs::create_dir_all(&dir).unwrap();
    let lock = std::fs::File::create(dir.join("lock")).unwrap();
    lock.lock().unwrap();
    std::fs::write(dir.join("server.log"), "resuming previous server\neasel: fatal: previous server failed\n").unwrap();
    let mut opener = Command::new(env!("CARGO_BIN_EXE_easel"))
        .args(["open", name])
        .env_remove("EASEL_WIDTH")
        .env("EASEL_ROOT", root())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut progress = String::new();
    BufReader::new(opener.stdout.as_mut().unwrap()).read_line(&mut progress).unwrap();
    drop(lock);
    let output = opener.wait_with_output().unwrap();
    assert_eq!(progress, "resuming previous server\n");
    assert!(output.status.success(), "open: {output:?}");
    assert!(ok(&["-s", name, "status"]).starts_with("0 chunks"));
    ok(&["-s", name, "close"]);
}

// A corrupt/obsolete chunk must never leave a partially resumed server running.
#[test]
fn replay_failure_is_fatal() {
    let name = "bad-replay";
    let _closing = closing(name);
    let dir = root().join("out/easel").join(name);
    std::fs::create_dir_all(&dir).unwrap();
    let logs = root().join("paintings/lua");
    std::fs::create_dir_all(&logs).unwrap();
    ok(&["open", name]);
    ok(&["-s", name, "do", "x = 1"]);
    ok(&["-s", name, "close"]);
    let text = std::fs::read_to_string(logs.join("bad-replay.lua")).unwrap()
        + "\n--@ chunk 2\nerror('cannot replay')\n\n--@ chunk 3\nx = 3\n";
    std::fs::write(logs.join("bad-replay.lua"), &text).unwrap();
    // Matching local witness isolates replay failure from edit detection.
    std::fs::write(dir.join("committed.lua"), &text).unwrap();
    let o = cmd(&["open", name]);
    assert!(!o.status.success(), "partial replay was accepted");
    assert!(String::from_utf8_lossy(&o.stderr).contains("replay failed at chunk 2"), "open: {o:?}");
    assert_eq!(std::fs::read_to_string(logs.join("bad-replay.lua")).unwrap(), text);
    assert!(!cmd(&["-s", name, "status"]).status.success());
}

// A chunk that ran is in the log even when the look asked for after it fails: the reply
// says both, so the chunk isn't sent again.
#[test]
fn a_failed_look_after_a_chunk_still_reports_the_chunk() {
    let _closing = closing("looked");
    ok(&["open", "looked"]);
    let out = ok(&["-s", "looked", "do", "x = 1", "--look"]);
    ok(&["-s", "looked", "close"]);
    assert!(out.contains("ok · chunk 1") && out.contains("no canvas yet"), "{out}");
}

#[test]
fn an_implicit_scratch_reset_survives_a_failed_chunk() {
    // its own studio: the log directory is made read-only below
    let studio = root().join("scratch-notice");
    let (cmd, ok) = (|args: &[&str]| cmd_in(&studio, args), |args: &[&str]| ok_in(&studio, args));
    let _closing = support::Closing(studio.join("out/easel/scratch-notice"));
    ok(&["open", "scratch-notice"]);
    ok(&["-s", "scratch-notice", "do", "canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{\"lead white\",5},{\"yellow ochre\",1}}, um=80, apply=\"knife\"}}}"]);
    ok(&["-s", "scratch-notice", "do", "local old_swatch=1", "--scratch"]);
    ok(&["-s", "scratch-notice", "close"]);
    let log = studio.join("paintings/lua/scratch-notice-scratch.lua");
    let mut old = std::fs::read_to_string(&log).unwrap();
    old.push_str("\nbad(\n");
    std::fs::write(&log, old).unwrap();
    ok(&["open", "scratch-notice"]);
    let o = cmd(&["-s", "scratch-notice", "do", "error('new scratch chunk fails')", "--scratch"]);
    assert!(!o.status.success());
    let error = String::from_utf8(o.stderr).unwrap();
    assert!(error.contains("a scratch canvas beside the painting"), "{error}");
    assert!(error.contains("new scratch chunk fails"), "{error}");
    assert!(error.contains("the chunk failed and changed nothing"), "{error}");
    assert!(!std::fs::read_to_string(log).unwrap().contains("old_swatch"));
    assert!(studio.join("paintings/lua/scratch-notice-scratch-1.lua").exists());
    // a fresh scratch canvas whose old log can't be put aside leaves that log and its
    // record together
    let (lua, record) = (studio.join("paintings/lua"), studio.join("out/easel/scratch-notice-scratch/committed.lua"));
    ok(&["-s", "scratch-notice", "do", "local kept=1", "--scratch"]);
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&lua, std::fs::Permissions::from_mode(0o555)).unwrap();
    let o = cmd(&["-s", "scratch-notice", "do", "local fresh=1", "--scratch", "--new"]);
    std::fs::set_permissions(&lua, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(!o.status.success());
    assert_eq!(std::fs::read_to_string(&record).unwrap(), std::fs::read_to_string(lua.join("scratch-notice-scratch.lua")).unwrap());
    // a look at a scratch canvas that can't be reopened says it was put aside
    ok(&["-s", "scratch-notice", "close"]);
    let mut old = std::fs::read_to_string(lua.join("scratch-notice-scratch.lua")).unwrap();
    old.push_str("\nbad(\n");
    std::fs::write(lua.join("scratch-notice-scratch.lua"), old).unwrap();
    ok(&["open", "scratch-notice"]);
    let o = cmd(&["-s", "scratch-notice", "look", "--scratch"]);
    assert!(!o.status.success());
    let error = String::from_utf8(o.stderr).unwrap();
    assert!(error.contains("couldn't be reopened, so it is put aside") && error.contains("no scratch canvas yet"), "{error}");
    assert!(lua.join("scratch-notice-scratch-2.lua").exists());
}

// A chunk whose committed record can't be written isn't kept: the log, its record and the
// state stay at the chunk before. A publication the easel stopped in is resolved on
// reopen: kept when the log holds all of the pending record, else taken back out.
#[test]
fn an_interrupted_publication_goes_on_from_an_accepted_revision() {
    let name = "publication";
    let _reaper = Reaper(name);
    let with = |a: &[&'static str]| [&["-s", name][..], a].concat();
    let (dir, log) = (root().join("out/easel").join(name), root().join("paintings/lua/publication.lua"));
    ok(&["open", name]);
    ok(&with(&["do", "x = 1"]));
    std::fs::create_dir(dir.join("committed.pending")).unwrap();
    let o = cmd(&with(&["do", "y = 2"]));
    std::fs::remove_dir(dir.join("committed.pending")).unwrap();
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("so it isn't kept"), "{o:?}");
    assert!(!std::fs::read_to_string(&log).unwrap().contains("y = 2"));
    while !ok(&with(&["status"])).starts_with("1 chunks") {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    ok(&with(&["do", "assert(x == 1 and y == nil)"]));
    ok(&with(&["close"]));
    // the easel stopped just before the record's rename: the chunk is kept
    let committed = std::fs::read_to_string(&log).unwrap();
    let grown = format!("{committed}\n--@ chunk 3\nz = 3\n");
    std::fs::write(&log, &grown).unwrap();
    std::fs::write(dir.join("committed.pending"), &grown).unwrap();
    assert!(ok(&["open", name]).contains("the chunk is kept"));
    ok(&with(&["do", "assert(z == 3)"]));
    ok(&with(&["close"]));
    // stopped while the chunk was appended: it is taken back out
    let committed = std::fs::read_to_string(&log).unwrap();
    std::fs::write(dir.join("committed.pending"), format!("{committed}\n--@ chunk 5\nw = 5\n")).unwrap();
    std::fs::write(&log, format!("{committed}\n--@ chunk 5\nw = ")).unwrap();
    assert!(ok(&["open", name]).contains("goes on from the chunk before it"));
    assert_eq!(std::fs::read_to_string(&log).unwrap(), committed);
    ok(&with(&["do", "assert(z == 3 and w == nil)"]));
    ok(&with(&["close"]));
}

// A note the easel is stopped in the middle of writing (here by a file size limit) leaves
// the journal as it was, never a partial entry.
#[test]
fn an_interrupted_note_leaves_the_journal_whole() {
    let studio = root().join("note-limit");
    let journal = studio.join("notes/journal.md");
    std::fs::create_dir_all(journal.parent().unwrap()).unwrap();
    let before = format!("{}\n", "x".repeat(200 * 512 - 8));
    std::fs::write(&journal, &before).unwrap();
    let easel = |args: &[&str]| Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env_remove("EASEL_WIDTH").env("EASEL_ROOT", &studio).env("EASEL_SESSION", "s").output().unwrap();
    let _closing = support::Closing(studio.join("out/easel/s"));
    let mut server = Command::new("sh").args(["-c", "ulimit -f 200 && exec \"$0\" serve s", env!("CARGO_BIN_EXE_easel")]).env("EASEL_ROOT", &studio).env_remove("EASEL_WIDTH").stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().unwrap();
    while !easel(&["status"]).status.success() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(!easel(&["note", "the sky is finished"]).status.success());
    let _ = server.kill();
    let _ = server.wait();
    assert_eq!(std::fs::read_to_string(&journal).unwrap(), before);
}

// Named sessions share the studio's journal: notes written from two of them at once all
// stay in it.
#[test]
fn notes_from_two_sessions_at_once_are_all_kept() {
    let studio = root().join("note-race");
    let easel = |args: &[&str]| ok_in(&studio, args);
    let _closing = ["left", "right"].map(|name| support::Closing(studio.join("out/easel").join(name)));
    for name in ["left", "right"] {
        easel(&["open", name]);
    }
    std::thread::scope(|s| {
        for name in ["left", "right"] {
            s.spawn(move || {
                for i in 0..20 {
                    easel(&["-s", name, "note", &format!("{name} {i}")]);
                }
            });
        }
    });
    for name in ["left", "right"] {
        easel(&["-s", name, "close"]);
    }
    let journal = std::fs::read_to_string(studio.join("notes/journal.md")).unwrap();
    for name in ["left", "right"] {
        for i in 0..20 {
            assert!(journal.contains(&format!(": {name} {i}\n")), "{name} {i} lost:\n{journal}");
        }
    }
    assert_eq!(std::fs::read_dir(studio.join("notes")).unwrap().filter(|e| e.as_ref().unwrap().file_name().to_string_lossy().starts_with(".journal.pending")).count(), 0);
}

#[test]
fn journal_uses_the_selected_painting_clock() {
    let _closing = closing("journal");
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
    let _closing = closing("gate");
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
    let socket = sock_of(&dir);
    let _ = std::fs::remove_file(&socket); // a previous run's, if the socket lives in the temp dir
    let mut server = Server(Command::new(env!("CARGO_BIN_EXE_easel"))
        .args(["serve", name])
        .env_remove("EASEL_WIDTH").env("EASEL_ROOT", root())
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().unwrap());
    let start = Instant::now();
    // ready when it accepts a connection, not when the socket file appears: the
    // file exists from bind(), and a connect before listen() is refused (under
    // load the gap is long enough: the full parallel suite hit it)
    while UnixStream::connect(&socket).is_err() {
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
    // t has a hole before its last integer key: a failed chunk that changes it leaves `#t`
    // to its layout, which forces a rebuild under every engine (heap.lua `holey`)
    assert!(request("do", "t = {}; for i = 1, 30 do t['k' .. i] = i end; t[2] = 2", normal).starts_with("ok\n"));
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
    assert!(request("do", "assert(t.unexpected == nil); local n = 0; for _ in pairs(t) do n = n + 1 end; assert(n == 31)", normal).starts_with("ok\n"));
    assert!(request("close", "", normal).starts_with("ok\n"));
}

// A look names its file one above the highest look-NNNN.png there, and never
// writes over a file that exists: a gap (look-0002 removed) or a stray
// look-prefixed file must not make the next look land on an older observation.
#[test]
fn a_look_never_overwrites_an_earlier_observation() {
    let s = ["-s", "gaps"];
    let with = |a: &[&'static str]| [&s[..], a].concat();
    // a failing assertion must not leave the server running
    let _closing = closing("gaps");
    ok(&["open", "gaps"]);
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
            .map(|_| Command::new(env!("CARGO_BIN_EXE_easel")).args(["open", name]).env_remove("EASEL_WIDTH").env("EASEL_ROOT", root()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap())
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

#[test]
fn serve_waits_for_a_closing_servers_lock() {
    use std::{
        io::Read,
        os::unix::net::UnixListener,
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
    let name = "serve-handoff";
    let _closing = closing(name);
    ok(&["open", name]);
    ok(&["-s", name, "do", "x = 1"]);
    ok(&["-s", name, "close"]);
    let dir = root().join("out/easel").join(name);
    let lock = std::fs::File::open(dir.join("lock")).unwrap();
    lock.lock().unwrap();
    let sock = sock_of(&dir);
    let listener = UnixListener::bind(&sock).unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut server = Server(Command::new(env!("CARGO_BIN_EXE_easel"))
        .args(["serve", name])
        .env_remove("EASEL_WIDTH").env("EASEL_ROOT", root())
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped())
        .spawn().unwrap());
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut conn = loop {
        if let Ok((conn, _)) = listener.accept() {
            break conn;
        }
        if let Some(status) = server.0.try_wait().unwrap() {
            let mut error = String::new();
            server.0.stderr.as_mut().unwrap().read_to_string(&mut error).unwrap();
            panic!("serve exited instead of waiting for the closing server: {status}: {error}");
        }
        assert!(Instant::now() < deadline, "serve never checked the closing server");
        std::thread::sleep(Duration::from_millis(1));
    };
    conn.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    let mut request = [0; 9];
    conn.read_exact(&mut request).unwrap();
    assert_eq!(&request, b"7\nstatus\n");
    std::fs::remove_file(&sock).unwrap();
    drop(listener);
    drop(lock);
    loop {
        if let Some(status) = server.0.try_wait().unwrap() {
            let mut error = String::new();
            server.0.stderr.as_mut().unwrap().read_to_string(&mut error).unwrap();
            panic!("serve exited at startup: {status}: {error}");
        }
        if let Ok(reply) = status_with_timeout(name)
            && reply.starts_with("ok\n1 chunks")
        {
            break;
        }
        assert!(Instant::now() < deadline, "serve never started after the lock was released");
        std::thread::sleep(Duration::from_millis(1));
    }
    drop(conn);
    assert!(status_with_timeout(name).unwrap().starts_with("ok\n1 chunks"));
    ok(&["-s", name, "do", "assert(x == 1)"]);
    ok(&["-s", name, "close"]);
    assert!(server.0.wait().unwrap().success());
}

#[test]
fn serve_refuses_a_stopped_owner() {
    serve_refuses_unresponsive_owner("stopped-owner", true);
}

#[test]
fn serve_refuses_an_owner_reading_an_incomplete_request() {
    serve_refuses_unresponsive_owner("incomplete-owner", false);
}

fn serve_refuses_unresponsive_owner(name: &str, stopped: bool) {
    use std::{
        io::{Read, Write},
        os::unix::{fs::MetadataExt, net::UnixStream},
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
    let spawn = || Server(Command::new(env!("CARGO_BIN_EXE_easel"))
        .args(["serve", name])
        .env_remove("EASEL_WIDTH").env("EASEL_ROOT", root())
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped())
        .spawn().unwrap());
    let mut owner = spawn();
    let sock = sock_of(&root().join("out/easel").join(name));
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Ok(reply) = status_with_timeout(name)
            && reply.starts_with("ok\n0 chunks")
        {
            break;
        }
        assert!(owner.0.try_wait().unwrap().is_none(), "{name}: owner exited at startup");
        assert!(Instant::now() < deadline, "{name}: owner never started");
        std::thread::sleep(Duration::from_millis(1));
    }
    let ino = std::fs::metadata(&sock).unwrap().ino();
    let incomplete = if stopped {
        assert!(Command::new("kill").args(["-STOP", &owner.0.id().to_string()]).status().unwrap().success());
        let deadline = Instant::now() + Duration::from_secs(1);
        loop {
            let state = Command::new("ps").args(["-o", "stat=", "-p", &owner.0.id().to_string()]).output().unwrap();
            if String::from_utf8_lossy(&state.stdout).contains('T') {
                break;
            }
            assert!(Instant::now() < deadline, "{name}: owner did not stop");
            std::thread::sleep(Duration::from_millis(1));
        }
        None
    } else {
        let mut conn = UnixStream::connect(&sock).unwrap();
        conn.set_write_timeout(Some(Duration::from_secs(1))).unwrap();
        conn.write_all(b"7\nsta").unwrap();
        Some(conn)
    };
    let mut contender = spawn();
    let deadline = Instant::now() + Duration::from_secs(2);
    let status = loop {
        if let Some(status) = contender.0.try_wait().unwrap() {
            break status;
        }
        assert!(Instant::now() < deadline, "{name}: second serve blocked on its owner");
        std::thread::sleep(Duration::from_millis(5));
    };
    let mut error = String::new();
    contender.0.stderr.as_mut().unwrap().read_to_string(&mut error).unwrap();
    assert!(!status.success() && error.contains("another easel serves"), "{name}: {status}: {error}");
    assert_eq!(std::fs::metadata(&sock).unwrap().ino(), ino, "{name}: second serve replaced the socket");
    drop(incomplete);
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
    let sock = sock_of(&root().join("out/easel/owner"));
    let ino = std::fs::metadata(&sock).unwrap().ino();
    let o = Command::new(env!("CARGO_BIN_EXE_easel")).args(["serve", name]).env_remove("EASEL_WIDTH").env("EASEL_ROOT", root()).output().unwrap();
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
