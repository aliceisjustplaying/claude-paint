//! `cfg(tube_box)`: the default tube box (and the older boxes built from its
//! tubes) is in the build when the build names no box (no `box-*` feature)
//! or all of them (`all-boxes`: paint's own default, and the easel's replay
//! build, whose `all-boxes` turns on paint's). A painter's build for one
//! named box (exactly one `box-*` feature) holds only its own box's tubes,
//! so its binary names no tube outside that box (palette.rs `catalog`).
//!
//! Two or more `box-*` features without `all-boxes` are no build: a painter's
//! easel has one box, and a union of some boxes is no build anything uses
//! (`cfg(box_conflict)` makes lib.rs a compile error).

fn main() {
    println!("cargo::rustc-check-cfg=cfg(tube_box)");
    println!("cargo::rustc-check-cfg=cfg(box_conflict)");
    let on = |f: &str| std::env::var_os(format!("CARGO_FEATURE_{f}")).is_some();
    let named = ["BOX_SARGENT", "BOX_INNESS", "BOX_ALMA_TADEMA", "BOX_TONN", "BOX_HOPPER"].iter().filter(|f| on(f)).count();
    let all = on("ALL_BOXES");
    if named == 0 || all {
        println!("cargo::rustc-cfg=tube_box");
    }
    if named > 1 && !all {
        println!("cargo::rustc-cfg=box_conflict");
    }
}
