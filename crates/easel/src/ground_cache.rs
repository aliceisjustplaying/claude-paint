use paint::{Canvas, Style};
use std::path::PathBuf;
use std::sync::OnceLock;

pub fn prepare(style: &Style, width: usize, aspect: f32, seed: u64) -> Canvas {
    if !style.ground.iter().any(|g| g.um > 0.0 && matches!(g.apply, paint::Apply::Brush)) {
        return style.prepare(width, aspect, seed);
    }
    static BUILD: OnceLock<Option<String>> = OnceLock::new();
    let directory = match std::env::var_os("EASEL_GROUND_CACHE") {
        Some(v) if v == "off" => return style.prepare(width, aspect, seed),
        Some(v) => Some(PathBuf::from(v)),
        None => std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
            .map(|p| p.join("claude-paint/grounds")),
    };
    let build = BUILD.get_or_init(|| {
        let bytes = std::env::current_exe().and_then(std::fs::read).ok()?;
        Some(format!("{:016x}", crate::save::fnv1a(&bytes)))
    });
    match (directory, build) {
        (Some(dir), Some(build)) => style.prepare_cached(width, aspect, seed, &dir, build),
        _ => style.prepare(width, aspect, seed),
    }
}
