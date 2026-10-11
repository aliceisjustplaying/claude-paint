#![cfg(feature = "replay")]

use std::{fs, process::Command};

fn run(name: &str, canvas: &str) -> std::process::Output {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("panel-{name}"));
    fs::create_dir_all(&dir).unwrap();
    let src = dir.join("probe.lua");
    fs::write(&src, format!("--@ engine 5\n--@ chunk 1\n{canvas}\n--@ chunk 2\nb=brush(\"flat\", 50); b:load(pile{{{{\"raw umber\",1}}}},0.4); b:stroke({{{{100,100}},{{800,200}}}})\n")).unwrap();
    Command::new(env!("CARGO_BIN_EXE_easel"))
        .args([
            "run",
            src.to_str().unwrap(),
            "--width",
            "100",
            "--out",
            dir.join("probe.png").to_str().unwrap(),
        ])
        .output()
        .unwrap()
}

#[test]
fn panel_is_only_available_in_the_explicit_experimental_build() {
    let out = run(
        "availability",
        "canvas{size=120,aspect=1,support=\"panel\",ground=\"chalk\",topography=\"oak\"}",
    );
    #[cfg(feature = "experimental-panel")]
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    #[cfg(not(feature = "experimental-panel"))]
    assert!(!out.status.success() && String::from_utf8_lossy(&out.stderr).contains("support"));
}

#[cfg(feature = "experimental-panel")]
#[test]
fn experimental_panel_rejects_incompatible_or_unknown_descriptors() {
    for (name, fields) in [
        ("linen", "support=\"panel\",linen=15"),
        ("raw", "support=\"panel\",raw=\"linen\""),
        ("ground", "support=\"panel\",ground=\"unknown\""),
        ("topography", "support=\"panel\",topography=\"unknown\""),
        ("support", "support=\"stone\""),
    ] {
        let out = run(name, &format!("canvas{{size=120,aspect=1,{fields}}}"));
        assert!(!out.status.success(), "{name} was accepted");
        assert!(String::from_utf8_lossy(&out.stderr).contains("experimental canvas"));
    }
}
