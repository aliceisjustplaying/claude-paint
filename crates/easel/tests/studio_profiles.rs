use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

mod support;

struct Studio {
    root: PathBuf,
}

impl Studio {
    fn new(tag: &str, setting: &str) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("studio-profile-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("bin")).unwrap();
        let root = root.canonicalize().unwrap();
        std::fs::hard_link(env!("CARGO_BIN_EXE_easel"), root.join("bin/easel")).unwrap();
        std::fs::write(root.join("bin/studio"), setting).unwrap();
        std::fs::write(root.join("bin/box"), "every\n").unwrap();
        assert_eq!(std::fs::metadata(env!("CARGO_BIN_EXE_easel")).unwrap().ino(), std::fs::metadata(root.join("bin/easel")).unwrap().ino());
        Self { root }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(self.root.join("bin/easel"))
            .args(args)
            .current_dir("/")
            .env("EASEL_ROOT", self.root.join("ignored-root"))
            .env("EASEL_SESSION", "ignored-session")
            .env("EASEL_BOX", "sargent")
            .env("EASEL_MODE", "developer")
            .env("RAYON_NUM_THREADS", "2")
            .env_remove("EASEL_WIDTH")
            .env_remove("EASEL_STATE_SESSION")
            .env_remove("EASEL_LOCK_ON_STDIN")
            .output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(out.status.success(), "{args:?}: {}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    }

    fn refuses(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(!out.status.success(), "accepted {args:?}");
        String::from_utf8(out.stderr).unwrap()
    }
}

impl Drop for Studio {
    fn drop(&mut self) {
        let _ = self.run(&["close"]);
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn one_binary_serves_two_boxes_and_the_default_without_developer_commands() {
    for name in ["friedrich", "hopper", "none"] {
        let studio = Studio::new(name, &format!("painter {name}\n"));
        let own = if name == "none" { paint::palette::DEFAULT_BOX } else { name };
        let tubes = paint::Palette::named_box(own).unwrap();
        assert_eq!(studio.ok(&["tubes", "--markdown"]), tubes.table());
        let help = studio.ok(&["help"]);
        for hidden in ["easel run", "easel check", "easel finish", "hash-probe", "render-checkpoint", "EASEL_ROOT", "EASEL_SESSION", "-s <name>"] {
            assert!(!help.contains(hidden), "{name}: {help}");
        }
        for other in paint::Palette::box_names().into_iter().filter(|b| *b != own && *b != "every") {
            assert!(!help.contains(other), "{name}: {help}");
        }
        for args in [
            vec!["run", "absent.lua"], vec!["check"], vec!["finish", "--print-chunk"],
            vec!["hash-probe"], vec!["render-checkpoint"], vec!["render-state"],
            vec!["snapshot-state"], vec!["render-snapshot"], vec!["snapshot-companions"],
            vec!["open", "other"], vec!["serve", "other"], vec!["-s", "other", "status"],
            vec!["status", "--session=other"], vec!["--box", "every", "tubes"],
        ] {
            studio.refuses(&args);
        }
        studio.ok(&["open"]);
        let expected = tubes.tubes.iter().map(|t| t.name).collect::<Vec<_>>().join(", ");
        let query = studio.ok(&["do", "print(table.concat(tubes(), ', ')); assert(io == nil and os == nil and require == nil and dofile == nil and loadfile == nil and debug == nil)"]);
        assert!(query.starts_with(&format!("{expected}\n")), "{name}: {query}");
        for verb in ["varnish", "cracks", "relief"] {
            let chunk = format!("{verb}()");
            let error = studio.refuses(&["do", &chunk]);
            assert!(error.contains(&format!("global '{verb}'")) && error.contains("the chunk failed and changed nothing"), "{chunk}: {error}");
        }
        let error = studio.refuses(&["do", "canvas{style='friedrich', palette='friedrich_early'}"]);
        assert!(error.contains("unknown option"), "{error}");
        let error = studio.refuses(&["do", "palette('friedrich_early')"]);
        assert!(!error.contains("friedrich_1820"), "{error}");
        let log = studio.ok(&["log"]);
        if name == "none" {
            assert!(!log.contains("--@ box"), "{log}");
        } else {
            assert!(log.contains(&format!("--@ box {own}\n")), "{log}");
        }
        assert!(studio.root.join("paintings/lua/painting.lua").is_file());
        assert!(!studio.root.join("ignored-root").exists());
        assert!(!studio.root.join("out/easel/current").exists());
        studio.ok(&["close"]);
        studio.ok(&["open"]);
        assert!(studio.ok(&["do", "print(table.concat(tubes(), ', '))"]).starts_with(&format!("{expected}\n")));
        studio.ok(&["close"]);
        for file in ["paintings/lua/painting.lua", "out/easel/painting/committed.lua"] {
            std::fs::write(studio.root.join(file), "--@ box sargent\n").unwrap();
        }
        let error = studio.refuses(&["open"]);
        assert!(error.contains("the log wasn't painted from this studio's box") && !error.contains("sargent"), "{error}");
        println!("{name}: same binary, only {own}, painter commands only");
    }
}

#[test]
fn painter_server_refuses_check_even_without_the_client() {
    let studio = Studio::new("socket-with-a-studio-path-long-enough-to-require-a-short-unix-socket", "painter none\n");
    studio.ok(&["open"]);
    let socket = support::sock_of(&studio.root.join("out/easel/painting"));
    let mut connection = UnixStream::connect(socket).unwrap();
    connection.write_all(b"6\ncheck\n").unwrap();
    let mut reply = String::new();
    connection.read_to_string(&mut reply).unwrap();
    assert!(reply.starts_with("err\nunknown command"), "{reply}");
}

#[test]
fn invalid_studio_settings_fail_closed_and_developer_mode_remains_available() {
    let studio = Studio::new("invalid", "painter missing\n");
    for setting in ["painter missing\n", "painter\n", "", "developer every\n"] {
        std::fs::write(studio.root.join("bin/studio"), setting).unwrap();
        let error = studio.refuses(&["help"]);
        assert!(error.contains("invalid studio"), "{error}");
        assert!(!error.contains("sargent") && !error.contains("friedrich"), "{error}");
    }
    std::fs::write(studio.root.join("bin/studio"), "developer\n").unwrap();
    let help = studio.ok(&["help"]);
    assert!(help.contains("easel run") && help.contains("easel finish"), "{help}");
    assert!(!studio.ok(&["hash-probe"]).trim().is_empty());
    std::fs::remove_file(studio.root.join("bin/studio")).unwrap();
    assert_eq!(studio.ok(&["help"]), help);
}
