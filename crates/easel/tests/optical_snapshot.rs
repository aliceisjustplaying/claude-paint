#![cfg(all(feature = "replay", feature = "render-snapshot"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Self {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("snapshot-cli-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
    fn command(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", &self.0).env("RAYON_NUM_THREADS", "2").env_remove("EASEL_SESSION").env_remove("EASEL_STATE_SESSION").env_remove("EASEL_STATE_DISK_MIB").output().unwrap()
    }
    fn path(&self, name: &str) -> String { self.0.join(name).to_string_lossy().into_owned() }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let path = self.0.join("out/easel/sketch-snapshot/lock");
        if let (Ok(file), Ok(pid)) = (std::fs::OpenOptions::new().read(true).write(true).open(&path), std::fs::read_to_string(&path)) {
            if matches!(file.try_lock(), Err(std::fs::TryLockError::WouldBlock)) && pid.trim().parse::<u32>().is_ok() { let _ = Command::new("kill").args(["-KILL", pid.trim()]).status(); }
        }
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn success(output: Output) -> String {
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn committed_snapshot_opens_and_makes_all_companions_without_material_or_journal() {
    let temp = Temp::new("standalone");
    let source = temp.path("painting.lua");
    std::fs::write(&source, "--@ engine 5\n--@ chunk 1\ncanvas{size=50, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\",3},{\"yellow ochre\",1}}, um=120, apply=\"knife\", texture=0.4}}}\n--@ chunk 2\nwait(1)\n").unwrap();
    let store = temp.path("state");
    let out = temp.path("direct.png");
    success(temp.command(&["run", &source, "--width", "32", "--state-store", &store, "--out", &out]));
    let view = temp.path("saved.view");
    success(temp.command(&["snapshot-state", &store, &view]));
    let mut native = Vec::new();
    for mode in ["color", "gallery", "relief"] {
        let path = temp.path(&format!("native-{mode}.png"));
        success(temp.command(&["render-state", &store, &path, "--mode", mode]));
        native.push(std::fs::read(path).unwrap());
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_dir_all(store).unwrap();
    for (i, mode) in ["color", "gallery", "relief"].into_iter().enumerate() {
        success(temp.command(&["render-snapshot", &view, &out, "--mode", mode]));
        assert_eq!(std::fs::read(&out).unwrap(), native[i]);
    }
    let cache = temp.path("companions");
    let outputs: serde_json::Value = serde_json::from_str(&success(temp.command(&["snapshot-companions", &view, &cache]))).unwrap();
    for (i, mode) in ["color", "gallery", "relief"].into_iter().enumerate() { assert_eq!(std::fs::read(outputs[mode].as_str().unwrap()).unwrap(), native[i]); }
    let cached = success(temp.command(&["snapshot-companions", &view, &cache]));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&cached).unwrap(), outputs);
    let changed: serde_json::Value = serde_json::from_str(&success(temp.command(&["snapshot-companions", &view, &cache, "--gallery-light", "20,35,1"]))).unwrap();
    assert_ne!(changed["generation"], outputs["generation"]);
    assert_eq!(changed["color"], outputs["color"]);
    assert_ne!(changed["gallery"], outputs["gallery"]);
}

#[test]
fn snapshot_export_rejects_store_inode_aliases_and_failure_preserves_existing_output() {
    let temp = Temp::new("aliases");
    let source = temp.path("painting.lua");
    std::fs::write(&source, "--@ engine 5\n--@ chunk 1\ncanvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\",3}}, um=120, apply=\"knife\"}}}\n").unwrap();
    let store = temp.path("state");
    let png = temp.path("out.png");
    success(temp.command(&["run", &source, "--width", "32", "--state-store", &store, "--out", &png]));
    let pointer = Path::new(&store).join("CURRENT");
    let before = std::fs::read(&pointer).unwrap();
    let alias = temp.path("alias.view");
    std::fs::hard_link(&pointer, &alias).unwrap();
    assert!(!temp.command(&["snapshot-state", &store, &alias]).status.success());
    assert_eq!(std::fs::read(&pointer).unwrap(), before);
    let output = temp.path("old.view");
    std::fs::write(&output, b"existing output").unwrap();
    assert!(!temp.command(&["snapshot-state", &temp.path("missing"), &output]).status.success());
    assert_eq!(std::fs::read(output).unwrap(), b"existing output");
}

#[test]
fn snapshot_final_destinations_preserve_journals_aliases_formats_and_live_reopen() {
    let temp = Temp::new("final-destination");
    let live = |args: &[&str]| Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", &temp.0).env("RAYON_NUM_THREADS", "2").env("EASEL_STATE_SESSION", "sketch-snapshot").env_remove("EASEL_SESSION").env_remove("EASEL_STATE_DISK_MIB").output().unwrap();
    success(live(&["open", "sketch-snapshot"]));
    success(live(&["-s", "sketch-snapshot", "do", "canvas{size=100, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\",3}}, um=120, apply=\"knife\"}}}"]));
    let store = temp.path("out/easel/sketch-snapshot/state");
    let log = temp.path("paintings/lua/sketch-snapshot.lua");
    let witness = temp.path("out/easel/sketch-snapshot/committed.lua");
    let unrelated = temp.path("unrelated.lua");
    std::fs::write(&unrelated, b"print('unrelated source')\n").unwrap();
    let pointer = std::fs::read(Path::new(&store).join("CURRENT")).unwrap();
    let accepted = std::fs::read(&log).unwrap();
    for source in [&log, &witness, &unrelated] {
        let before = std::fs::read(source).unwrap();
        let result = temp.command(&["snapshot-state", &store, source]);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains(".view extension"));
        for symlink in [false, true] {
            let alias = temp.path("source-alias.view");
            if symlink { std::os::unix::fs::symlink(source, &alias).unwrap(); }
            else { std::fs::hard_link(source, &alias).unwrap(); }
            let result = temp.command(&["snapshot-state", &store, &alias]);
            assert!(!result.status.success());
            assert!(String::from_utf8_lossy(&result.stderr).contains("refusing to replace journal exports or unrelated files"));
            assert_eq!(std::fs::read(&alias).unwrap(), before);
            assert_eq!(std::fs::read(source).unwrap(), before);
            std::fs::remove_file(alias).unwrap();
        }
        assert_eq!(std::fs::read(source).unwrap(), before);
    }
    for name in ["unsupported.png", "unsupported.jpg", "no-extension", "unrelated.view"] {
        let output = temp.path(name);
        std::fs::write(&output, b"preserve this file").unwrap();
        assert!(!temp.command(&["snapshot-state", &store, &output]).status.success());
        assert_eq!(std::fs::read(output).unwrap(), b"preserve this file");
    }
    let view = temp.path("accepted.view");
    success(temp.command(&["snapshot-state", &store, &view]));
    let original = std::fs::read(&view).unwrap();
    success(temp.command(&["snapshot-state", &store, &view]));
    assert_eq!(std::fs::read(&view).unwrap(), original);
    success(live(&["-s", "sketch-snapshot", "status"]));
    success(live(&["-s", "sketch-snapshot", "close"]));
    success(live(&["open", "sketch-snapshot"]));
    assert_eq!(std::fs::read(log).unwrap(), accepted);
    assert_eq!(std::fs::read(Path::new(&store).join("CURRENT")).unwrap(), pointer);
    success(live(&["-s", "sketch-snapshot", "close"]));
    assert!(!std::fs::read_dir(&temp.0).unwrap().any(|e| e.unwrap().file_name().to_string_lossy().ends_with(".part")));
}
