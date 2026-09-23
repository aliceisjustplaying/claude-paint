//! Coast FRICTION #17 (notes/amnesia2/fresh2_coast.md): "a curved
//! multi-point `drag` with a fine brush laid nothing". The coil of rope in
//! paintings/fresh2/fresh2_coast.rs is an 11-point U,
//! `y = py + 1.3 + drop * sin(a).powf(0.8)` for `a = PI * k / 10`. In f32,
//! `sin(PI)` is -8.7e-8, so the last point's y is NaN, and a gesture with a
//! NaN point lays nothing anywhere along it (and says nothing). The
//! diagnosis and the proposed fix are in notes/workflow.md, "Open bugs".

use paint::{Canvas, Gesture, Held, Orient, Paint, Tool, hex};

/// The rope coil's points, exactly as the coast painting builds them.
fn coil(px: f32, py: f32, drop: f32, half: f32) -> Vec<(f32, f32)> {
    (0..=10)
        .map(|k| {
            let a = std::f32::consts::PI * k as f32 / 10.0;
            (px + 5.5 - half * a.cos(), py + 1.3 + drop * a.sin().powf(0.8))
        })
        .collect()
}

/// How many pixels a drag through `pts` changed on a fresh canvas.
fn laid(pts: Vec<(f32, f32)>) -> usize {
    let mut c = Canvas::new(1000, 1.0, hex("#c8c0b0"));
    c.dry();
    let before = c.pixels().to_vec();
    let mut b = Held::new(Tool::rigger(0.5), 211);
    b.load(Paint::body(hex("#3a3129")), 0.55);
    let g = Gesture::new(pts).pressure(0.7, 0.7).ramps(0.0, 0.0).orient(Orient::Across);
    c.drag(&mut b, &g, None);
    c.dry();
    c.pixels().iter().zip(&before).filter(|(a, b)| a != b).count()
}

#[test]
fn the_coil_has_a_nan_point() {
    assert!(std::f32::consts::PI.sin() < 0.0);
    let pts = coil(500.0, 372.0, 12.0, 2.3);
    let bad: Vec<usize> = (0..pts.len()).filter(|&i| !(pts[i].0.is_finite() && pts[i].1.is_finite())).collect();
    assert_eq!(bad, vec![10], "only the last point, at a = PI, is NaN");
}

/// The same curved drag with the last point made finite paints normally:
/// curvature, the fine brush and the direction are not the cause.
#[test]
fn the_same_curve_with_finite_points_lays_paint() {
    let mut pts = coil(500.0, 372.0, 12.0, 2.3);
    pts[10].1 = 372.0 + 1.3;
    assert!(laid(pts) > 50);
    // the half that painted in the coast session: bottom to left end, no NaN
    let pts = coil(500.0, 372.0, 12.0, 2.3);
    assert!(laid(pts[..=5].iter().rev().copied().collect()) > 20);
}

/// What the painter saw: next to nothing. `drag_on` sums the arc length
/// of the resampled path, which is NaN; `nsteps = (total / step).ceil() as
/// usize` is then 0 (NaN casts to 0), `.max(1)` makes it 1, and
/// `(k * step).min(total)` ignores the NaN: the brush takes a single step
/// at the start and lifts. One dab of a fine rigger (12 pixels at 1000px,
/// against 106 for the whole U).
#[test]
fn a_nan_point_lays_only_the_first_step() {
    let nan = laid(coil(500.0, 372.0, 12.0, 2.3));
    let mut pts = coil(500.0, 372.0, 12.0, 2.3);
    pts[10].1 = 373.3;
    let whole = laid(pts);
    assert!(nan * 5 < whole, "NaN coil {nan} px, finite coil {whole} px");
}

/// The fix to apply (in bristle.rs, after the `tip` stream lands): a drag
/// through a non-finite point should fail loudly, like an invalid `Tool`
/// does, and name the point.
#[test]
#[ignore = "fix belongs in bristle.rs (Canvas::drag / footprint); apply after merging `tip`"]
#[should_panic(expected = "point 10")]
fn a_nan_point_is_an_error() {
    laid(coil(500.0, 372.0, 12.0, 2.3));
}
