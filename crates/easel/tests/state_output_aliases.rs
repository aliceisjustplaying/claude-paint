#![cfg(all(feature = "replay", feature = "state-store"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CANVAS: &str = r#"canvas{size=100, aspect=1.25, linen=15, seed=7, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Self {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("store-output-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
    fn command(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", &self.0).env("RAYON_NUM_THREADS", "2").env("EASEL_STATE_SESSION", "sketch-output").env_remove("EASEL_SESSION").env_remove("EASEL_STATE_DISK_MIB").output().unwrap()
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let lock = self.0.join("out/easel/sketch-output/lock");
        if let Ok(pid) = std::fs::read_to_string(&lock) {
            let _ = Command::new("kill").args(["-KILL", pid.trim()]).status();
        }
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn success(output: Output) { assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr)); }

fn store_files(dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for directory in [dir.to_path_buf(), dir.join("objects"), dir.join("revisions")] {
        paths.extend(std::fs::read_dir(directory).unwrap().map(|e| e.unwrap().path()).filter(|p| p.is_file()));
    }
    paths
}

#[test]
fn render_state_refuses_every_store_inode_alias_in_native_and_bounded_modes() {
    let temp = Temp::new("render");
    let source = temp.0.join("painting.lua");
    std::fs::write(&source, format!("--@ engine 5\n--@ chunk 1\n{CANVAS}\n")).unwrap();
    let dir = temp.0.join("state");
    let png = temp.0.join("initial.png");
    success(temp.command(&["run", source.to_str().unwrap(), "--width", "32", "--state-store", dir.to_str().unwrap(), "--out", png.to_str().unwrap()]));
    let pointer = std::fs::read(dir.join("CURRENT")).unwrap();
    for path in store_files(&dir) {
        let before = std::fs::read(&path).unwrap();
        let alias = temp.0.join("alias.png");
        std::fs::hard_link(&path, &alias).unwrap();
        for mode in [vec![], vec!["--look"]] {
            let mut args = vec!["render-state", dir.to_str().unwrap(), alias.to_str().unwrap()];
            args.extend(mode);
            let result = temp.command(&args);
            assert!(!result.status.success(), "accepted alias of {}", path.display());
            assert!(String::from_utf8_lossy(&result.stderr).contains("aliases a painting store file"));
            assert_eq!(std::fs::read(&path).unwrap(), before);
            assert_eq!(std::fs::read(&alias).unwrap(), before);
        }
        std::fs::remove_file(alias).unwrap();
    }
    assert_eq!(std::fs::read(dir.join("CURRENT")).unwrap(), pointer);
    success(temp.command(&["render-state", dir.to_str().unwrap(), png.to_str().unwrap()]));
}

#[test]
fn live_save_refuses_store_inode_aliases_and_reopen_keeps_the_accepted_root() {
    let temp = Temp::new("live");
    success(temp.command(&["open", "sketch-output"]));
    success(temp.command(&["-s", "sketch-output", "do", CANVAS]));
    let dir = temp.0.join("out/easel/sketch-output/state");
    let pointer = std::fs::read(dir.join("CURRENT")).unwrap();
    for path in store_files(&dir) {
        let before = std::fs::read(&path).unwrap();
        let alias = temp.0.join("alias.png");
        std::fs::hard_link(&path, &alias).unwrap();
        let result = temp.command(&["-s", "sketch-output", "save", alias.to_str().unwrap()]);
        assert!(!result.status.success(), "accepted live save alias of {}", path.display());
        assert!(String::from_utf8_lossy(&result.stderr).contains("aliases a painting store file"));
        assert_eq!(std::fs::read(&path).unwrap(), before);
        std::fs::remove_file(alias).unwrap();
    }
    let png = temp.0.join("accepted.png");
    success(temp.command(&["-s", "sketch-output", "save", png.to_str().unwrap()]));
    assert!(std::fs::read(png).unwrap().starts_with(b"\x89PNG"));
    success(temp.command(&["-s", "sketch-output", "close"]));
    success(temp.command(&["open", "sketch-output"]));
    assert_eq!(std::fs::read(dir.join("CURRENT")).unwrap(), pointer);
    success(temp.command(&["-s", "sketch-output", "close"]));
}

#[test]
fn final_destinations_preserve_log_alias_checks_journal_exports_and_png_format() {
    let temp = Temp::new("final-destination");
    success(temp.command(&["open", "sketch-output"]));
    success(temp.command(&["-s", "sketch-output", "do", CANVAS]));
    let state = temp.0.join("out/easel/sketch-output/state");
    let log = temp.0.join("paintings/lua/sketch-output.lua");
    let witness = temp.0.join("out/easel/sketch-output/committed.lua");
    let accepted = std::fs::read(&log).unwrap();
    let pointer = std::fs::read(state.join("CURRENT")).unwrap();
    for export in [&log, &witness] {
        assert_eq!(std::fs::read(export).unwrap(), accepted);
        for path in [export.to_path_buf(), temp.0.join("journal-alias.png")] {
            if path != *export { std::fs::hard_link(export, &path).unwrap(); }
            for bounded in [false, true] {
                let mut args = vec!["render-state", state.to_str().unwrap(), path.to_str().unwrap(), "--log", export.to_str().unwrap()];
                if bounded { args.push("--look"); }
                let result = temp.command(&args);
                assert!(!result.status.success());
                assert!(String::from_utf8_lossy(&result.stderr).contains("output must differ from the source log"));
                assert_eq!(std::fs::read(export).unwrap(), accepted);
            }
            let result = temp.command(&["-s", "sketch-output", "save", path.to_str().unwrap()]);
            assert!(!result.status.success());
            assert!(String::from_utf8_lossy(&result.stderr).contains("journal exports"));
            assert_eq!(std::fs::read(export).unwrap(), accepted);
            if path != *export { std::fs::remove_file(path).unwrap(); }
        }
    }
    for filename in ["unrelated.lua", "unsupported.jpg", "no-extension"] {
        let path = temp.0.join(filename);
        std::fs::write(&path, b"keep this file").unwrap();
        assert!(!temp.command(&["-s", "sketch-output", "save", path.to_str().unwrap()]).status.success());
        assert!(!temp.command(&["render-state", state.to_str().unwrap(), path.to_str().unwrap()]).status.success());
        assert_eq!(std::fs::read(path).unwrap(), b"keep this file");
    }
    success(temp.command(&["-s", "sketch-output", "status"]));
    success(temp.command(&["-s", "sketch-output", "close"]));
    success(temp.command(&["open", "sketch-output"]));
    assert_eq!(std::fs::read(&log).unwrap(), accepted);
    assert_eq!(std::fs::read(state.join("CURRENT")).unwrap(), pointer);
    success(temp.command(&["-s", "sketch-output", "close"]));
}
