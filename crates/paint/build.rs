//! `cfg(tube_box)`: the default tube box (and the older boxes built from its
//! tubes) is in the build unless the build is a painter's build for one
//! named box (exactly one `box-*` feature; the easel's replay build turns on
//! all four). That build holds only its own box's tubes, so its binary names
//! no tube outside that box (palette.rs `catalog`).

fn main() {
    println!("cargo::rustc-check-cfg=cfg(tube_box)");
    let on = |f: &str| std::env::var_os(format!("CARGO_FEATURE_{f}")).is_some();
    let named = ["BOX_SARGENT", "BOX_INNESS", "BOX_ALMA_TADEMA", "BOX_TONN"].iter().filter(|f| on(f)).count();
    if named != 1 {
        println!("cargo::rustc-cfg=tube_box");
    }
}
