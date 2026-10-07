//! Persistence contracts exercised with real locked files, no socket or global root.
use super::*;
use std::os::unix::fs::PermissionsExt;

struct Temp(PathBuf);
impl Temp {
    fn new(label: &str) -> Self {
        let p = std::env::temp_dir().join(format!("easel-persist-{label}-{}", std::process::id()));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn files(&self, name: &str) -> SessionFiles {
        SessionFiles::acquire(&self.0, name.into(), None).unwrap_or_else(|e| panic!("{e}"))
    }
    fn open(&self, name: &str) -> PersistedSession {
        let like = Session::with_box(
            32,
            paint::Palette::named_box(paint::Palette::fallback_box()).unwrap(),
        )
        .unwrap();
        self.files(name)
            .open(Some(&like))
            .unwrap_or_else(|(_, e)| panic!("{e}"))
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const CANVAS: &str = "canvas{size=300, aspect=4, linen=20, seed=5, raw='cotton duck'}";

// Existing socket tests cannot observe rollback at the filesystem failure point.
// Real permission/filename failures cover preparing, appending and publishing.
#[test]
fn failed_log_or_witness_writes_discard_the_chunk_and_allow_retry() {
    let temp = Temp::new("discard");
    let mut p = temp.open("demo");
    p.run(CANVAS).unwrap();
    p.run("x = 1; t = {a = 1}; b = brush('round', 3)").unwrap();
    let before = fs::read(p.log_path()).unwrap();
    let globals = p.session().globals();
    let status = p.session().status();
    let pixels = crate::bits(&p.session().canvas().unwrap().seen());
    let checkpoint = temp.0.join("before.ckpt");
    save::write(p.session(), &p.session().program(p.name()), &checkpoint).unwrap();
    let saved = fs::read(&checkpoint).unwrap();
    let pending = p.dir().join("committed.pending");
    for fail in ["prepare", "append", "publish"] {
        match fail {
            "prepare" => fs::create_dir(&pending).unwrap(),
            "append" => {
                fs::set_permissions(p.log_path(), fs::Permissions::from_mode(0o444)).unwrap()
            }
            "publish" => {
                fs::write(&pending, b"prepare here").unwrap();
                fs::set_permissions(p.dir(), fs::Permissions::from_mode(0o555)).unwrap();
            }
            _ => unreachable!(),
        }
        let e = p.run("x = 2; t.a = 2; t.b = 3; b:load(pile{{'bone black', 1}}, 0.9); b:stroke({0, 0, 30, 30}); wait(1)").unwrap_err();
        fs::set_permissions(p.dir(), fs::Permissions::from_mode(0o755)).unwrap();
        fs::set_permissions(p.log_path(), fs::Permissions::from_mode(0o644)).unwrap();
        if fail == "prepare" {
            fs::remove_dir(&pending).unwrap();
        }
        assert!(
            e.starts_with("session integrity: the chunk's log update couldn't be saved ("),
            "{fail}: {e}"
        );
        assert!(
            e.ends_with("; the chunk was not kept\n(the chunk failed and changed nothing)"),
            "{fail}: {e}"
        );
        assert_eq!(fs::read(p.log_path()).unwrap(), before, "{fail}");
        assert_eq!(
            fs::read(p.dir().join("committed.lua")).unwrap(),
            before,
            "{fail}"
        );
        p.ready().unwrap();
        assert_eq!(p.session().globals(), globals, "{fail}");
        assert_eq!(p.session().status(), status, "{fail}");
        assert_eq!(
            crate::bits(&p.session().canvas().unwrap().seen()),
            pixels,
            "{fail}"
        );
        save::write(p.session(), &p.session().program(p.name()), &checkpoint).unwrap();
        assert_eq!(fs::read(&checkpoint).unwrap(), saved, "{fail}");
    }
    p.run("assert(x == 1 and t.a == 1 and t.b == nil)").unwrap();
    drop(p);
    let mut reopened = temp.open("demo");
    reopened
        .run("assert(x == 1 and t.a == 1 and t.b == nil)")
        .unwrap();
}

#[test]
fn persisted_open_and_live_requests_reject_changed_logs_and_witnesses() {
    let temp = Temp::new("integrity");
    let mut p = temp.open("demo");
    p.run("x = 1").unwrap();
    let original = fs::read(p.log_path()).unwrap();
    for path in [p.log_path(), p.dir().join("committed.lua")] {
        for edit in [Some(b"-- edited\n".as_slice()), Some(b"".as_slice()), None] {
            match edit {
                Some(bytes) => fs::write(&path, bytes).unwrap(),
                None => fs::remove_file(&path).unwrap(),
            }
            let e = p.run("x = 2").unwrap_err();
            assert!(
                e.contains("The easel goes on only from the log it wrote"),
                "{e}"
            );
            assert!(p.save_live(0).is_err());
            assert!(p.rebuild_with_progress(|_, _| {}).is_err());
            let files = p.into_files();
            p = match files.open(None) {
                Err((files, e)) => {
                    assert!(e.contains("session integrity"), "{e}");
                    fs::write(&path, &original).unwrap();
                    files.open(None).unwrap_or_else(|(_, e)| panic!("{e}"))
                }
                Ok(_) => panic!("edited files were accepted"),
            };
        }
    }
    p.run("assert(x == 1)").unwrap();
}

#[test]
fn replay_failure_keeps_ownership_until_put_aside() {
    let temp = Temp::new("bad-replay");
    let p = temp.open("demo");
    let files = p.into_files();
    let text = "--@ chunk 1\nx = 1\n--@ chunk 2\nerror('cannot replay')\n--@ chunk 3\nx = 3\n";
    fs::write(files.log_path(), text).unwrap();
    fs::write(files.dir().join("committed.lua"), text).unwrap();
    let files = match files.open(None) {
        Err((files, e)) => {
            assert!(e.contains("replay failed at chunk 2"), "{e}");
            files
        }
        Ok(_) => panic!("partial replay accepted"),
    };
    assert!(SessionFiles::acquire(&temp.0, "demo".into(), None).is_err());
    let backup = files.put_aside().unwrap().unwrap();
    assert_eq!(fs::read_to_string(backup).unwrap(), text);
    let fresh = files.open(None).unwrap_or_else(|(_, e)| panic!("{e}"));
    assert_eq!(
        fresh.session().status(),
        "0 chunks · 2400px · no canvas yet"
    );
}

// Scratch and painting share the constructor. Holding either name blocks the
// other route, and resetting never releases the lock or overwrites a backup.
#[test]
fn scratch_owns_its_files_across_reset_and_reopen() {
    let temp = Temp::new("scratch");
    let mut painting = temp.open("demo");
    painting.run(CANVAS).unwrap();
    let painting_globals = painting.session().globals();
    let name = crate::scratch_name(painting.name());
    assert_eq!(crate::scratch_name("painting"), "scratch");
    let files = temp.files(&name);
    let mut scratch = files
        .open(Some(painting.session()))
        .unwrap_or_else(|(_, e)| panic!("{e}"));
    scratch.run(CANVAS).unwrap();
    scratch.run("x = 71; wait(1)").unwrap();
    let old = fs::read(scratch.log_path()).unwrap();
    assert!(
        SessionFiles::acquire(&temp.0, name.clone(), None)
            .err()
            .unwrap()
            .contains("another easel serves session \"demo-scratch\"")
    );
    assert_eq!(painting.session().globals(), painting_globals);
    assert_ne!(painting.session().status(), scratch.session().status());
    drop(scratch);
    let mut reopened = temp
        .files(&name)
        .open(Some(painting.session()))
        .unwrap_or_else(|(_, e)| panic!("{e}"));
    reopened.run("assert(x == 71)").unwrap();
    let files = reopened.into_files();
    let first = files.put_aside().unwrap().unwrap();
    assert!(first.ends_with("demo-scratch-1.lua"));
    assert!(fs::read(&first).unwrap().starts_with(&old));
    assert!(SessionFiles::acquire(&temp.0, name.clone(), None).is_err());
    let mut reset = files
        .open(Some(painting.session()))
        .unwrap_or_else(|(_, e)| panic!("{e}"));
    reset.run(CANVAS).unwrap();
    reset.run("assert(x == nil)").unwrap();
    let files = reset.into_files();
    assert!(
        files
            .put_aside()
            .unwrap()
            .unwrap()
            .ends_with("demo-scratch-2.lua")
    );
    drop(files);
    // A directly opened painting with scratch's name now owns the files.
    let owner = temp.open(&name);
    assert!(SessionFiles::acquire(&temp.0, name, None).is_err());
    assert!(
        !owner
            .log_path()
            .with_file_name("demo-scratch-3.lua")
            .exists()
    );
    assert!(first.exists());
}

// Moving bootstrap into this owner must not silently discard an incomplete
// stored session after a failed initial witness write (the base fails closed).
#[test]
fn initial_witness_failure_remains_an_incomplete_session() {
    let temp = Temp::new("bootstrap");
    let files = temp.files("demo");
    fs::create_dir(files.dir().join("committed.pending")).unwrap();
    let files = match files.open(None) {
        Err((files, e)) => {
            assert!(e.contains("directory"), "{e}");
            files
        }
        Ok(_) => panic!("failed initialization was accepted"),
    };
    assert!(files.log_path().is_file());
    assert!(!files.dir().join("committed.lua").exists());
    fs::remove_dir(files.dir().join("committed.pending")).unwrap();
    match files.open(None) {
        Err((_, e)) => assert!(
            e.contains("The easel goes on only from the log it wrote"),
            "{e}"
        ),
        Ok(_) => panic!("incomplete initialization was silently restarted"),
    }
}
