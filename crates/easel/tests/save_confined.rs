//! Live saves must stay in the studio in both developer and painter modes.

use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
mod support;

// A new painting here is 2400 px (no bin/width, EASEL_WIDTH unset); a thin, bare canvas keeps this path test cheap.
const CANVAS: &str = r#"canvas{size=100, aspect=5, linen=15, seed=7, raw="cotton duck"}"#;
const KEEP: &[u8] = b"outside file: preserve these bytes";

struct Studio {
    base: PathBuf,
    root: PathBuf,
    elsewhere: PathBuf,
    binary: PathBuf,
    session: &'static str,
    painter: bool,
}

impl Studio {
    fn new(name: &str, painter: bool) -> Self {
        let base = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("save-confined-{}-{name}-{painter}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("studio/bin")).unwrap();
        std::fs::create_dir_all(base.join("elsewhere")).unwrap();
        let base = base.canonicalize().unwrap();
        let root = base.join("studio");
        let elsewhere = base.join("elsewhere");
        let binary = if painter {
            let binary = root.join("bin/easel");
            std::fs::hard_link(env!("CARGO_BIN_EXE_easel"), &binary).unwrap();
            std::fs::write(root.join("bin/studio"), "painter none\n").unwrap();
            binary
        } else {
            PathBuf::from(env!("CARGO_BIN_EXE_easel"))
        };
        let session = if painter { "painting" } else { "sketch-save" };
        let studio = Self {
            base,
            root,
            elsewhere,
            binary,
            session,
            painter,
        };
        if painter {
            studio.ok(&["open"]);
        } else {
            studio.ok(&["open", session]);
        }
        studio.ok(&["do", CANVAS]);
        studio
    }

    fn command(&self, args: &[&str]) -> Output {
        let mut command = Command::new(&self.binary);
        command
            .args(args)
            .current_dir(&self.elsewhere)
            .env("RAYON_NUM_THREADS", "2")
            .env_remove("EASEL_BOX")
            .env_remove("EASEL_WIDTH")
            .env_remove("EASEL_STATE_SESSION")
            .env_remove("EASEL_STATE_DISK_MIB")
            .env_remove("EASEL_LOCK_ON_STDIN");
        command
            .env("EASEL_ROOT", if self.painter { &self.elsewhere } else { &self.root })
            .env("EASEL_SESSION", self.session);
        #[cfg(feature = "state-store")]
        command.env("EASEL_STATE_SESSION", self.session);
        command.output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let output = self.command(args);
        assert!(
            output.status.success(),
            "easel {args:?}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }

    fn refuses(&self, args: &[&str]) {
        let output = self.command(args);
        assert!(
            !output.status.success(),
            "easel {args:?} accepted an unsafe destination: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    fn default_png(&self) -> PathBuf {
        self.root.join(format!("out/easel/{0}/{0}.png", self.session))
    }

    fn save(&self, args: &[&str], destination: &Path) {
        assert_eq!(self.ok(args).trim_end(), destination.display().to_string());
        assert!(
            std::fs::read(destination)
                .unwrap()
                .starts_with(b"\x89PNG\r\n\x1a\n"),
            "{} is not a PNG",
            destination.display()
        );
    }
}

impl Drop for Studio {
    fn drop(&mut self) {
        support::stop_servers(&self.root);
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

#[test]
fn saves_absolute_relative_default_and_lit_pngs_in_the_studio() {
    for painter in [false, true] {
        let studio = Studio::new("accepted", painter);
        let absolute = studio.root.join("absolute/new/picture.png");
        studio.save(&["save", absolute.to_str().unwrap()], &absolute);
        studio.save(
            &["save", "relative/new/picture.png"],
            &studio.root.join("relative/new/picture.png"),
        );
        studio.save(&["save"], &studio.default_png());
        studio.save(
            &["save", "lit/gallery.png", "--gallery"],
            &studio.root.join("lit/gallery.png"),
        );
        studio.save(
            &["save", "lit/light.png", "--light", "135,25"],
            &studio.root.join("lit/light.png"),
        );
        assert_eq!(
            std::fs::read_dir(&studio.elsewhere).unwrap().count(),
            0,
            "relative saves used the caller's cwd"
        );
    }
}

#[test]
fn refuses_paths_and_symlinks_that_escape_before_creating_directories() {
    for painter in [false, true] {
        let studio = Studio::new("escapes", painter);
        let outside = studio.base.join("outside.png");
        std::fs::write(&outside, KEEP).unwrap();
        for path in [
            outside.to_str().unwrap(),
            "../outside.png",
            "missing/../../outside.png",
            "../new/nested/picture.png",
            "missing/../../new/nested/picture.png",
        ] {
            studio.refuses(&["save", path]);
            assert_eq!(std::fs::read(&outside).unwrap(), KEEP);
            assert!(!studio.root.join("missing").exists());
            assert!(!studio.base.join("new").exists());
        }

        symlink(&studio.elsewhere, studio.root.join("escape")).unwrap();
        studio.refuses(&["save", "escape/new/nested/picture.png", "--gallery"]);
        assert_eq!(std::fs::read_dir(&studio.elsewhere).unwrap().count(), 0);
        symlink(&outside, studio.root.join("linked.png")).unwrap();
        for path in ["linked.png", "linked.png/", "linked.png/."] {
            studio.refuses(&["save", path, "--light", "135,25"]);
            assert_eq!(std::fs::read(&outside).unwrap(), KEEP);
        }
        assert_eq!(std::fs::read(&outside).unwrap(), KEEP);
        symlink(
            studio.base.join("new/dangling.png"),
            studio.root.join("dangling.png"),
        )
        .unwrap();
        studio.refuses(&["save", "dangling.png"]);
        assert!(!studio.base.join("new").exists());

        for name in ["picture.jpg", "picture.lua", "no-extension"] {
            let destination = studio.root.join(name);
            std::fs::write(&destination, KEEP).unwrap();
            studio.refuses(&["save", name]);
            assert_eq!(std::fs::read(destination).unwrap(), KEEP);
        }
        studio.ok(&["status"]);
    }
}

#[test]
fn refuses_hardlinks_and_checks_the_default_destination_too() {
    for painter in [false, true] {
        let studio = Studio::new("aliases", painter);
        let outside = studio.base.join("outside.png");
        std::fs::write(&outside, KEEP).unwrap();
        let linked = studio.root.join("hardlink.png");
        std::fs::hard_link(&outside, &linked).unwrap();
        studio.refuses(&["save", "hardlink.png"]);
        assert_eq!(std::fs::read(&outside).unwrap(), KEEP);
        assert_eq!(std::fs::read(&linked).unwrap(), KEEP);

        std::fs::hard_link(&outside, studio.default_png()).unwrap();
        studio.refuses(&["save"]);
        assert_eq!(std::fs::read(&outside).unwrap(), KEEP);
        assert_eq!(std::fs::read(studio.default_png()).unwrap(), KEEP);
        std::fs::remove_file(studio.default_png()).unwrap();

        symlink(&outside, studio.default_png()).unwrap();
        studio.refuses(&["save"]);
        assert_eq!(std::fs::read(&outside).unwrap(), KEEP);
        assert!(
            std::fs::symlink_metadata(studio.default_png())
                .unwrap()
                .file_type()
                .is_symlink()
        );
        studio.ok(&["status"]);
    }
}
