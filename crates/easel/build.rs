//! `cfg(tube_box)`: this build's paint has the default tube box (paint's
//! build.rs: no `box-*` feature, or `all-boxes`). The easel's own tests that
//! paint from the default box are compiled only then, so a painter's build
//! for one box (`--no-default-features --features box-<name>`) runs the rest.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(tube_box)");
    let on = |f: &str| std::env::var_os(format!("CARGO_FEATURE_{f}")).is_some();
    let named = ["BOX_SARGENT", "BOX_INNESS", "BOX_ALMA_TADEMA", "BOX_TONN", "BOX_HOPPER"].iter().filter(|f| on(f)).count();
    if named == 0 || on("ALL_BOXES") {
        println!("cargo::rustc-cfg=tube_box");
    }
}
