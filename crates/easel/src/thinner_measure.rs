//! Measuring helpers for the easel-level thinner acceptance tests
//! (`thinner_tests.rs`, notes/thinner/ACCEPTANCE.md): sessions, the
//! black-and-white card, sums over the canvas, saves, and af49348's old
//! files (notes/thinner/baseline/old_files/).
//!
//! The paint interface they read is listed in
//! crates/paint/tests/thinner_support/mod.rs. The easel interface they
//! need is `pile{..., thinner = t}` (engine 3) and `p.thinner`.
#![allow(dead_code)]

use crate::session::{Session, box_for, parse_program};
use paint::color::{linear_to_srgb, luminance};
use paint::thinner::evaporation_tau_min;
use paint::{Canvas, Palette, Rgb};

/// (x0, y0, x1, y1) in canvas units.
pub type Rect = (f32, f32, f32, f32);

/// A live session at `width` px painting from the Inness box (it has raw
/// sienna), with the current engine.
pub fn inness(width: usize) -> Session {
    Session::with_box(width, Palette::named_box("inness").expect("the inness box (feature box-inness)")).expect("a session")
}

/// The same, painting with engine `engine`.
pub fn inness_engine(width: usize, engine: u32) -> Session {
    let mut t = Palette::named_box("inness").expect("the inness box (feature box-inness)");
    t.engine = engine;
    Session::with_box(width, t).expect("a session")
}

/// Run a chunk; panic with the chunk and the error if it fails.
pub fn run(s: &mut Session, src: &str) -> String {
    s.run(src).unwrap_or_else(|e| panic!("{src}\n-> {e}")).out
}

/// Replay a log (its own box and engine) at `width` px, as `easel run` does.
pub fn replay(log: &str, width: usize) -> Session {
    let mut s = Session::replay_with(width, box_for(Some(log)).expect("the log's box")).expect("a session");
    for (i, c) in parse_program(log).iter().enumerate() {
        s.run(c).unwrap_or_else(|e| panic!("chunk {}: {e}", i + 1));
    }
    s
}

/// The canvas's complete saved state (header "").
pub fn state(c: &Canvas) -> Vec<u8> {
    let mut v = Vec::new();
    c.write_state(&mut v, "").expect("write_state to memory");
    v
}

/// The pixel centers (units) inside `r`.
pub fn centers_in(c: &Canvas, r: Rect) -> Vec<(f32, f32)> {
    let f = c.frame();
    (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| x >= r.0 && x < r.2 && y >= r.1 && y < r.3).collect()
}

/// Sum of `f(canvas, x, y)` over the pixel centers in `r`.
pub fn sum_in(c: &Canvas, r: Rect, f: impl Fn(&Canvas, f32, f32) -> f64) -> f64 {
    centers_in(c, r).into_iter().map(|(x, y)| f(c, x, y)).sum()
}

/// Largest `f(canvas, x, y)` over the pixel centers in `r` (0 if none).
pub fn max_in(c: &Canvas, r: Rect, f: impl Fn(&Canvas, f32, f32) -> f32) -> f32 {
    centers_in(c, r).into_iter().map(|(x, y)| f(c, x, y)).fold(0.0, f32::max)
}

pub fn paint_um(c: &Canvas, x: f32, y: f32) -> f64 {
    c.wet_um(x, y) as f64
}

pub fn solvent_um(c: &Canvas, x: f32, y: f32) -> f64 {
    c.solvent_um(x, y) as f64
}

/// Mean color seen (dry picture and open paint), linear RGB, over `r`.
pub fn mean_rgb(c: &Canvas, r: Rect) -> Rgb {
    let p = centers_in(c, r);
    let mut a = [0.0f64; 3];
    for &(x, y) in &p {
        let q = c.under(x, y, 0.0);
        for k in 0..3 {
            a[k] += q[k] as f64;
        }
    }
    a.map(|v| (v / p.len().max(1) as f64) as f32)
}

// ---------------------------------------------------------------- the card

/// The card's canvas: the Inness box's toned ground on linen.
pub const CARD_CANVAS: &str = r#"canvas{size=400, aspect=1, linen=15, seed=5, ground={{pile={{"lead white", 3}, {"raw umber", 1}}, um=90, apply="knife"}}}"#;
/// The card's bands, each this tall (units): the ground, black, white.
pub const BAND: f32 = 100.0;
/// Pile parts, as the Lua `pile{}` takes them.
pub const RAW_SIENNA: &str = r#"{"raw sienna", 1}"#;
pub const LEAD_WHITE: &str = r#"{"lead white", 1}"#;

/// A black band (bone black) and a white band (lead white) laid thick
/// across the canvas under the ground band, then left 60 days to dry.
pub fn card_bands() -> String {
    format!(
        r#"local k, w = pile{{{{"bone black", 1}}}}, pile{{{{"lead white", 1}}}}
work(rect(0, {b}, 1000, {h}), {{hand="body", pile=k, load=1, coverage=5, clip=true, seed=1}})
work(rect(0, {w}, 1000, {h}), {{hand="body", pile=w, load=1, coverage=5, clip=true, seed=2}})
wait(60 * 24 * 60)
"#,
        b = BAND,
        w = 2.0 * BAND,
        h = BAND
    )
}

/// One strip of the card, measured.
#[derive(Clone, Debug)]
pub struct Cell {
    pub load: f32,
    pub thinner: f32,
    /// The share of the card's black/white contrast (luminance of the
    /// white band's mean minus the black band's) still showing.
    pub kept: f32,
    /// Solvent in the strip (sum of µm over its pixels) right after the
    /// pass: no more than the pass added (some evaporated during it).
    pub solvent_after_pass: f64,
    /// The same when `kept` is measured.
    pub solvent_at_measure: f64,
    /// The evaporation time (min) of the thickest paint on the card, right
    /// after the passes and when measured.
    pub tau_after_pass: f64,
    pub tau_at_measure: f64,
    /// Minutes waited after the passes before measuring.
    pub waited: f64,
    /// Paint in the strip (sum of µm) when measured.
    pub paint: f64,
}

/// Lay one `body` pass of `parts` per cell (load, thinner; 0 = no thinner
/// key) in strips side by side over the card at `width` px, wait ten times
/// the evaporation time of the thickest paint on the card (whole minutes,
/// plus one), and measure each strip.
pub fn card(width: usize, parts: &str, cells: &[(f32, f32)]) -> Vec<Cell> {
    let mut s = inness(width);
    run(&mut s, CARD_CANVAS);
    run(&mut s, &card_bands());
    let n = cells.len();
    let col = 1000.0 / n as f32;
    let sw = col * 0.78;
    let inset = 10.0;
    let strip = |c: usize| -> Rect {
        let x0 = c as f32 * col + (col - sw) / 2.0;
        (x0, 0.0, x0 + sw, 3.0 * BAND)
    };
    let band = |c: usize, b: usize| -> Rect {
        let (x0, _, x1, _) = strip(c);
        (x0 + inset, b as f32 * BAND + inset, x1 - inset, (b + 1) as f32 * BAND - inset)
    };
    let contrast = |s: &Session, c: usize| -> f32 {
        let cv = s.canvas().unwrap();
        luminance(mean_rgb(&cv, band(c, 2))) - luminance(mean_rgb(&cv, band(c, 1)))
    };
    let before: Vec<f32> = (0..n).map(|c| contrast(&s, c)).collect();
    let mut chunk = String::new();
    for (c, &(load, t)) in cells.iter().enumerate() {
        let (x0, _, x1, _) = strip(c);
        let th = if t > 0.0 { format!(", thinner={t}") } else { String::new() };
        chunk += &format!("work(rect({x0}, 0, {}, {}), {{hand=\"body\", pile=pile{{{parts}{th}}}, load={load}, clip=true, seed={}}})\n", x1 - x0, 3.0 * BAND, 11 + c);
    }
    run(&mut s, &chunk);
    let tau_now = |s: &Session| -> f64 {
        let cv = s.canvas().unwrap();
        let top = (0..n).map(|c| max_in(&cv, strip(c), |cv, x, y| cv.wet_um(x, y))).fold(0.0, f32::max);
        evaporation_tau_min(top)
    };
    let solvent_after: Vec<f64> = (0..n).map(|c| sum_in(&s.canvas().unwrap(), strip(c), solvent_um)).collect();
    let tau_after = tau_now(&s);
    let waited = (10.0 * tau_after).ceil() + 1.0;
    run(&mut s, &format!("wait({waited})"));
    let tau_at = tau_now(&s);
    (0..n)
        .map(|c| {
            let cv = s.canvas().unwrap();
            Cell {
                load: cells[c].0,
                thinner: cells[c].1,
                kept: contrast(&s, c) / before[c],
                solvent_after_pass: solvent_after[c],
                solvent_at_measure: sum_in(&cv, strip(c), solvent_um),
                tau_after_pass: tau_after,
                tau_at_measure: tau_at,
                waited,
                paint: sum_in(&cv, strip(c), paint_um),
            }
        })
        .collect()
}

// ---------------------------------------------------------------- old files

/// A file of notes/thinner/baseline/old_files/ (made by unchanged af49348).
pub fn old_file(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../notes/thinner/baseline/old_files").join(name)
}

/// old_files/paintck8_save_stroke.ckpt.xz, decompressed (`xz -dc`) to a
/// temporary file: a genuine PAINTCK8 save of an engine-3 canvas.
pub fn old_save() -> std::path::PathBuf {
    let out = std::process::Command::new("xz").arg("-dc").arg(old_file("paintck8_save_stroke.ckpt.xz")).output().expect("xz (to read old_files/paintck8_save_stroke.ckpt.xz)");
    assert!(out.status.success(), "xz -dc: {}", String::from_utf8_lossy(&out.stderr));
    let path = std::env::temp_dir().join(format!("thinner-paintck8-{}.ckpt", std::process::id()));
    std::fs::write(&path, &out.stdout).unwrap();
    path
}

// ---------------------------------------------------------------- pictures

/// The canvas as seen, 8-bit sRGB, with its width and height.
pub fn picture(c: &Canvas) -> (u32, u32, Vec<u8>) {
    let f = c.frame();
    let px: Vec<u8> = c.seen().iter().flat_map(|p| p.map(|v| (linear_to_srgb(v).clamp(0.0, 1.0) * 255.0).round() as u8)).collect();
    (f.w as u32, f.h as u32, px)
}
