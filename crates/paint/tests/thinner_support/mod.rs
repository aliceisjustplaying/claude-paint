//! Measuring helpers for the thinner acceptance tests (notes/thinner/ACCEPTANCE.md).
//!
//! They read the canvas, the brush and the rag only through public API,
//! the interface the acceptance tests require. The interface already
//! existed at af49348, except for the items marked NEW:
//!
//! - `Paint::with_thinner(t)` (NEW): the paint thinned with solvent share `t`.
//! - `Canvas::wet_um(x, y)`: the open paint film at a point, µm, solvent-free.
//! - `Canvas::solvent_um(x, y)` (NEW): the solvent in the open film there, µm.
//! - `Canvas::wet_total()`: total open paint, coats × square units.
//! - `Canvas::solvent_total()` (NEW): total solvent, in the same units.
//! - `Canvas::cure_at(x, y)` (NEW): the open film's cure there (0 fresh).
//! - `Held::carried()` (NEW): (paint, solvent) on the brush, in the units
//!   of `Canvas::wet_total`.
//! - `Rag::solvent_mm3` (NEW, a public field): the solvent the cloth has
//!   taken off the canvas, mm³, cumulative. The paint is the mm³ that
//!   `rag_wipe` and `rag_blot` return, as at af49348.
//! - `paint::thinner::stroke_limit_um(t)` (NEW): how much wet film (paint +
//!   solvent, µm) one stroke may add to a pixel, for paint thinned `t`.
//!   `f32::INFINITY` at `t = 0`.
//! - `paint::thinner::evaporation_tau_min(paint_um)` (NEW): the
//!   evaporation time (minutes) of the solvent in a film that holds
//!   `paint_um` of paint (solvent-free).
//! - The engine-3 save (`Canvas::write_state`) starts with `PAINTCK9` and
//!   ends with the solvent: one little-endian f32 per buffer pixel (µm, row
//!   major), the last 4 × (pixel count) bytes of the file (NEW).
#![allow(dead_code)]

use paint::color::luminance;
use paint::rag::Rag;
use paint::{COAT_UM, Canvas, Gesture, Held, Linen, Paint, Palette, Rgb, Tool, hex};

/// A small engine-3 canvas `px` pixels wide (1000 units, square, 700 mm)
/// on fine linen, primed with a dry, toned ground.
pub fn canvas(px: usize) -> Canvas {
    let mut c = Canvas::new(px, 1.0, hex("#d8cdb8")).with_engine(3).with_linen(Linen::fine(3));
    c.prime(hex("#b9a98c"), 0.9, 60.0, 0.6, 0.2, 7);
    assert_eq!(c.engine(), 3);
    c
}

/// The same canvas without linen and with an untextured ground: a flat
/// surface, so the open paint's own ridges are the only relief.
pub fn smooth_canvas(px: usize) -> Canvas {
    let mut c = Canvas::new(px, 1.0, hex("#d8cdb8")).with_engine(3);
    c.prime(hex("#b9a98c"), 0.9, 60.0, 0.6, 0.0, 7);
    c
}

/// A tube of the Inness box as paint, straight from the tube.
pub fn tube_paint(name: &str) -> Paint {
    let pal = Palette::named_box("inness").expect("the inness box (paint's default features)");
    let i = pal.tubes.iter().position(|t| t.name == name).unwrap_or_else(|| panic!("no tube {name:?} in the inness box"));
    pal.pile(vec![(i, 1.0)]).laid(0.0)
}

pub fn raw_sienna() -> Paint {
    tube_paint("raw sienna")
}

pub fn lead_white() -> Paint {
    tube_paint("lead white")
}

/// A brush of `tool` (seed `seed`) loaded with `amount` of `paint`.
pub fn brush(tool: Tool, seed: u64, paint: Paint, amount: f32) -> Held {
    let mut h = Held::new(tool, seed);
    h.load(paint, amount);
    h
}

/// One stroke along `pts` (units) at a constant `pressure`.
pub fn stroke(c: &mut Canvas, h: &mut Held, pts: Vec<(f32, f32)>, pressure: f32) {
    c.drag(h, &Gesture::new(pts).pressure(pressure, pressure), None);
}

/// A patch of open paint: overlapping horizontal strokes of a broad flat
/// 25 units apart (the flat is 40 wide), each from a freshly loaded brush
/// (`load`, `pressure`), between `y.0` and `y.1`. `paint` as given: no
/// thinner call, so an unthinned patch is today's code path.
pub fn patch_with(c: &mut Canvas, paint: Paint, load: f32, pressure: f32, x: (f32, f32), y: (f32, f32), seed: u64) {
    let rows = ((y.1 - y.0) / 25.0).ceil().max(1.0) as usize;
    for k in 0..rows {
        let yk = y.0 + (y.1 - y.0) * (k as f32 + 0.5) / rows as f32;
        let mut h = brush(Tool::hog_flat(40.0), seed + k as u64, paint, load);
        stroke(c, &mut h, vec![(x.0, yk), (x.1, yk + 3.0)], pressure);
    }
}

/// `patch_with` at load 0.9, pressure 0.85.
pub fn patch(c: &mut Canvas, paint: Paint, x: (f32, f32), y: (f32, f32), seed: u64) {
    patch_with(c, paint, 0.9, 0.85, x, y, seed);
}

/// A patch of `paint` thinned `t`.
pub fn thinned_patch(c: &mut Canvas, paint: Paint, t: f32, x: (f32, f32), y: (f32, f32), seed: u64) {
    patch(c, paint.with_thinner(t), x, y, seed);
}

/// Every pixel of the canvas: (buffer index, center x, center y) in units.
pub fn centers(c: &Canvas) -> Vec<(usize, f32, f32)> {
    let f = c.frame();
    (0..f.w * f.h).map(|i| (i, f.ux(i % f.w), f.uy(i / f.w))).collect()
}

/// The pixels whose centers lie in the rectangle (x0, y0, x1, y1), units.
pub fn in_rect(c: &Canvas, r: (f32, f32, f32, f32)) -> Vec<(usize, f32, f32)> {
    centers(c).into_iter().filter(|&(_, x, y)| x >= r.0 && x < r.2 && y >= r.1 && y < r.3).collect()
}

/// Open paint (solvent-free), µm, per pixel.
pub fn paint_um(c: &Canvas) -> Vec<f32> {
    centers(c).iter().map(|&(_, x, y)| c.wet_um(x, y)).collect()
}

/// Solvent in the open film, µm, per pixel.
pub fn solvent_um(c: &Canvas) -> Vec<f32> {
    centers(c).iter().map(|&(_, x, y)| c.solvent_um(x, y)).collect()
}

/// The wet film (paint + solvent), µm, per pixel.
pub fn wet_film(c: &Canvas) -> Vec<f32> {
    centers(c).iter().map(|&(_, x, y)| c.wet_um(x, y) + c.solvent_um(x, y)).collect()
}

/// Below this a film is no film (µm): the engine's own threshold for bare
/// canvas, 1e-5 coats (`Canvas::look_px`, `drying::Canvas::wait`).
pub const ZERO_UM: f64 = 1e-5 * COAT_UM as f64;
/// Below this a cure is no cure: a millionth of the way to the gel point
/// (`drying::GEL` = 0.15).
pub const ZERO_CURE: f64 = 1.5e-7;

/// The open film's cure, per pixel.
pub fn cure(c: &Canvas) -> Vec<f32> {
    centers(c).iter().map(|&(_, x, y)| c.cure_at(x, y)).collect()
}

/// mm³ per unit of `Canvas::wet_total` (coats × square units).
pub fn mm3_per_total(c: &Canvas) -> f64 {
    let mmu = c.mm_per_unit() as f64;
    COAT_UM as f64 / 1000.0 * mmu * mmu
}

/// Paint and solvent on the canvas and on a brush, in `wet_total`'s units.
#[derive(Clone, Copy, Debug)]
pub struct Totals {
    pub canvas_paint: f64,
    pub canvas_solvent: f64,
    pub brush_paint: f64,
    pub brush_solvent: f64,
}

impl Totals {
    pub fn of(c: &Canvas, h: &Held) -> Self {
        let (bp, bs) = h.carried();
        Totals { canvas_paint: c.wet_total(), canvas_solvent: c.solvent_total(), brush_paint: bp, brush_solvent: bs }
    }
}

/// The rounding allowed in a balance of paint or solvent (check 4): the
/// books may be off by this share of everything being counted.
pub const BALANCE_REL: f64 = 1e-4;

/// Assert that `before` and `after` (sums of the same quantity) balance
/// within `BALANCE_REL` of the larger.
#[track_caller]
pub fn assert_balances(what: &str, before: f64, after: f64) {
    let scale = before.abs().max(after.abs());
    assert!((after - before).abs() <= BALANCE_REL * scale, "{what}: {before} before, {after} after (off by {:.3e} of the total; allowed {BALANCE_REL:e})", (after - before).abs() / scale.max(f64::MIN_POSITIVE));
}

/// Two values agree: equal, or both no further from zero than `floor`, or
/// within `rel` of the larger in magnitude (the zero rule of check 17).
pub fn close(a: f64, b: f64, rel: f64, floor: f64) -> bool {
    if a == b {
        return true;
    }
    if a.abs() <= floor && b.abs() <= floor {
        return true;
    }
    (a - b).abs() <= rel * a.abs().max(b.abs())
}

/// The canvas's complete saved state (header "").
pub fn state(c: &Canvas) -> Vec<u8> {
    let mut v = Vec::new();
    c.write_state(&mut v, "").expect("write_state to memory");
    v
}

/// A copy of `c`, through its save, with the solvent at every pixel
/// multiplied by `k`: the same paint in the same places, with the same
/// pigment, stiffness, cure, clock and everything else, and a different
/// amount of solvent. It relies on the PAINTCK9 format contract
/// (ACCEPTANCE.md, "The interface"): the solvent is the save's last
/// 4 × pixels bytes, one f32 µm per pixel. Checks the copy saves back to
/// exactly the same bytes outside that block, and the scaled solvent in it.
pub fn with_solvent_scaled(c: &Canvas, k: f32) -> Canvas {
    let orig = state(c);
    assert_eq!(&orig[..8], b"PAINTCK9", "an engine-3 canvas saves as PAINTCK9");
    let n = c.pixels().len();
    assert!(orig.len() >= 8 + 4 * n, "the save is shorter than its solvent section");
    let start = orig.len() - 4 * n;
    let mut b = orig.clone();
    for i in 0..n {
        let o = start + 4 * i;
        let v = f32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        b[o..o + 4].copy_from_slice(&(v * k).to_le_bytes());
    }
    let mut r: &[u8] = &b;
    let (copy, _) = Canvas::read_state(&mut r).expect("read the edited save back");
    let back = state(&copy);
    assert_eq!(back.len(), b.len(), "the copy saves to the same length");
    assert!(back[..start] == orig[..start], "the copy differs from the original outside the solvent block (first at byte {:?})", back[..start].iter().zip(&orig[..start]).position(|(x, y)| x != y));
    assert!(back[start..] == b[start..], "the copy's solvent block is not the scaled solvent");
    let (s0, s1) = (solvent_um(c), solvent_um(&copy));
    for i in 0..n {
        assert_eq!(s1[i].to_bits(), (s0[i] * k).to_bits(), "the save's last block is the solvent the canvas reports (pixel {i})");
    }
    copy
}

/// The pixels within `r` pixels (a square) of buffer pixel `i`.
pub fn around(c: &Canvas, i: usize, r: usize) -> Vec<usize> {
    let f = c.frame();
    let (x, y) = (i % f.w, i / f.w);
    let (x0, x1, y0, y1) = (x.saturating_sub(r), (x + r + 1).min(f.w), y.saturating_sub(r), (y + r + 1).min(f.h));
    (y0..y1).flat_map(|yy| (x0..x1).map(move |xx| yy * f.w + xx)).collect()
}

/// The `q`-quantile (0..1) of `v`.
pub fn quantile(v: &[f32], q: f32) -> f32 {
    let mut s: Vec<f32> = v.to_vec();
    s.sort_by(f32::total_cmp);
    if s.is_empty() { f32::NAN } else { s[((s.len() - 1) as f32 * q).round() as usize] }
}

pub fn mean(v: &[f32]) -> f64 {
    v.iter().map(|&x| x as f64).sum::<f64>() / v.len().max(1) as f64
}

pub fn median(v: &[f32]) -> f32 {
    let mut s: Vec<f32> = v.to_vec();
    s.sort_by(f32::total_cmp);
    if s.is_empty() { f32::NAN } else { s[s.len() / 2] }
}

pub fn std_dev(v: &[f32]) -> f64 {
    let m = mean(v);
    (v.iter().map(|&x| (x as f64 - m).powi(2)).sum::<f64>() / v.len().max(1) as f64).sqrt()
}

/// Least-squares slope of `y` on `x`.
pub fn slope(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    sxy / sxx
}

/// Mean open paint (µm) along a horizontal stroke at `y`, within `half`
/// units of its center line, in bins `bin` units long from `x0` to `x1`.
pub fn profile(c: &Canvas, y: f32, half: f32, x0: f32, x1: f32, bin: f32) -> Vec<f32> {
    let n = ((x1 - x0) / bin).floor() as usize;
    let (mut sum, mut cnt) = (vec![0.0f64; n], vec![0usize; n]);
    for (_, x, yy) in in_rect(c, (x0, y - half, x0 + bin * n as f32, y + half)) {
        let k = (((x - x0) / bin) as usize).min(n - 1);
        sum[k] += c.wet_um(x, yy) as f64;
        cnt[k] += 1;
    }
    sum.iter().zip(&cnt).map(|(s, &k)| (s / k.max(1) as f64) as f32).collect()
}

/// The paint surface (relief + open paint, solvent-free), µm, over a rect.
pub fn paint_surface(c: &Canvas, r: (f32, f32, f32, f32)) -> Vec<f32> {
    let h = c.surface_um();
    in_rect(c, r).iter().map(|&(i, x, y)| h[i] + c.wet_um(x, y)).collect()
}

/// Luminance of the picture as seen (dry picture and open paint).
pub fn seen_luminance(c: &Canvas) -> Vec<f32> {
    c.seen().iter().map(|&p: &Rgb| luminance(p)).collect()
}

/// The thickest open paint (µm) anywhere on the canvas.
pub fn max_paint_um(c: &Canvas) -> f32 {
    paint_um(c).into_iter().fold(0.0, f32::max)
}

/// Rag on the canvas: a fresh one `width` units across.
pub fn rag(width: f32, seed: u64) -> Rag {
    Rag::new(width, seed)
}
