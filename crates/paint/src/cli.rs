//! Shared command-line handling for painting binaries.
//!
//!   cargo paint <name>                 1000px preview → out/<name>.png
//!   cargo paint <name> -- --full       3200px final   → out/<name>_full.png
//!   cargo paint <name> -- --width 1600 --seed 7

pub struct Opts {
    pub width: usize,
    pub seed: u64,
    pub out: std::path::PathBuf,
}

pub fn opts(name: &str) -> Opts {
    let args: Vec<String> = std::env::args().collect();
    let get = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1));
    let full = args.iter().any(|a| a == "--full");
    let width = get("--width").and_then(|s| s.parse().ok()).unwrap_or(if full { 3200 } else { 1000 });
    let seed = get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../out");
    let out = match get("--out") {
        Some(p) => p.into(),
        None if full => root.join(format!("{name}_full.png")),
        None => root.join(format!("{name}.png")),
    };
    Opts { width, seed, out }
}

pub fn done(o: &Opts, t: std::time::Instant) {
    let p = o.out.canonicalize().unwrap_or(o.out.clone());
    eprintln!("wrote {} ({}px) in {:.1}s", p.display(), o.width, t.elapsed().as_secs_f32());
}
