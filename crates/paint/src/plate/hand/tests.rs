//! Where a hand's tool touches a plate: the edge's band and swing, and a dab.
//! Expected counts come from the geometry of the path and the edge.

use super::*;

fn pt(x: f32, y: f32) -> Point {
    Point { x, y, force: 0.5, alt: 90.0, az: 0.0, speed: 20.0 }
}

/// A 20 × 10 mm plate at 0.1 mm a cell.
fn tiny() -> Plate {
    Plate::new(Metal::Copper, Temper::HalfHard, 20.0, 10.0, 1.3, 45.0, 200, 100)
}

fn touched(p: &Plate, s: &[Stretch]) -> Vec<f32> {
    p.touches(s)
}

/// A straight stroke touches each cell of its band once (whatever its
/// stretches), and nothing ahead of its start or past its end; the band's
/// sides are the edge's half-width either side of the path.
#[test]
fn a_straight_stroke_touches_its_band_once() {
    let p = tiny();
    let s = p.sweep(&[pt(2.0, 5.0), pt(18.0, 5.0)], &|_| 4.0, 1.0);
    assert_eq!(s.len(), 16, "1 mm stretches");
    let t = touched(&p, &s);
    for y in 0..100 {
        for x in 0..200 {
            let (cx, cy) = ((x as f64 + 0.5) * 0.1, (y as f64 + 0.5) * 0.1);
            let v = t[y * 200 + x];
            let inside = cx > 2.0 && cx <= 18.0 && (cy - 5.0).abs() < 1.95;
            let outside = cx < 2.0 || cx > 18.0 || (cy - 5.0).abs() > 2.05;
            if inside {
                assert_eq!(v, 1.0, "({x}, {y}) once");
            }
            if outside {
                assert_eq!(v, 0.0, "({x}, {y}) untouched");
            }
        }
    }
    // the same stroke in one stretch is the same touches
    let one = p.sweep(&[pt(2.0, 5.0), pt(18.0, 5.0)], &|_| 4.0, 100.0);
    assert_eq!(touched(&p, &one), t);
}

/// Back over itself, a stroke touches its band twice; the edge swinging
/// round at the far end passes once over the half-disc beyond it.
#[test]
fn back_over_itself_touches_twice() {
    let p = tiny();
    let s = p.sweep(&[pt(4.0, 5.0), pt(16.0, 5.0), pt(4.0, 5.0)], &|_| 4.0, 2.0);
    let t = touched(&p, &s);
    // in the band, between the ends
    assert_eq!(t[50 * 200 + 100], 2.0);
    assert_eq!(t[40 * 200 + 60], 2.0);
    // 1 mm past the turn, on the path: the swing's half-disc, once
    assert_eq!(t[50 * 200 + 170], 1.0);
    // and nothing past the edge's reach
    assert_eq!(t[50 * 200 + 185], 0.0);
}

/// Round a right-angled corner the edge leaves no gap outside the turn: its
/// swing passes over the quarter-disc between the two bands; inside the turn
/// it rubs back over the cells it then passes again.
#[test]
fn round_a_corner_no_gap_outside() {
    let p = tiny();
    let s = p.sweep(&[pt(2.0, 2.0), pt(10.0, 2.0), pt(10.0, 9.0)], &|_| 2.0, 100.0);
    assert_eq!(s.len(), 2);
    let t = touched(&p, &s);
    // outside the corner (beyond x 10, above y 2), within the edge's half-width of it
    let (x, y) = (105, 15);
    assert_eq!(t[y * 200 + x], 1.0, "the swing covers the outer corner");
    // inside the corner (x < 10, y > 2): passed by the first band, swung back over, passed again
    let (x, y) = (95, 25);
    assert_eq!(t[y * 200 + x], 3.0);
    // well along each leg, once
    assert_eq!(t[20 * 200 + 50], 1.0);
    assert_eq!(t[70 * 200 + 100], 1.0);
}

/// A pad worked in a closed circle round a point within its half-width
/// passes over that point twice a turn (each half of its edge once).
#[test]
fn a_circle_passes_its_middle_twice_a_turn() {
    let p = tiny();
    let n = 64;
    let pts: Vec<Point> = (0..=n).map(|i| {
        let a = i as f32 / n as f32 * std::f32::consts::TAU;
        pt(10.0 + 1.0 * a.cos(), 5.0 + 1.0 * a.sin())
    }).collect();
    let s = p.sweep(&pts, &|_| 6.0, 100.0);
    let t = touched(&p, &s);
    // (the centre cell's middle is 0.05 mm off the circle's centre)
    let v = t[50 * 200 + 100];
    assert!((v - 2.0).abs() < 0.05, "{v}");
}

/// A dab touches the disc of its face: the share of a rim cell is how far the
/// rim reaches into it.
#[test]
fn a_dab_touches_its_face() {
    let p = tiny();
    let s = p.dabs(&[pt(10.0, 5.0)], &|_| 3.0);
    assert_eq!(s.len(), 1);
    let t = touched(&p, &s);
    let mut area = 0.0f64;
    for v in &t {
        area += *v as f64 * 0.01;
    }
    let want = std::f64::consts::PI * 1.5 * 1.5;
    assert!((area / want - 1.0).abs() < 0.01, "{area} mm² against {want}");
    assert_eq!(t[50 * 200 + 100], 1.0);
    assert_eq!(t[50 * 200 + 120], 0.0);
}

/// A stretch's layers keep a cell at most once each.
#[test]
fn layers_split_repeats() {
    let s = Stretch { v: (1.0, 0.0), pose: pt(0.0, 0.0), len_mm: 1.0, width_mm: 1.0, cells: vec![(1, 1, 1.0), (2, 1, 1.0), (1, 1, 0.5)] };
    let l = s.layers();
    assert_eq!(l, vec![vec![(1, 1, 1.0), (2, 1, 1.0)], vec![(1, 1, 0.5)]]);
}

/// A slid pad rubs the cells under its middle one pass for a straight
/// stroke, nearer its rim the chord over its width, and beyond it nothing;
/// back over itself twice; round a corner it rubs the corner as it is
/// carried (no gap, no pivot); worked in circles it rubs the middle by the
/// circles' length over its width.
#[test]
fn a_slid_pad_rubs_by_the_length_it_slides() {
    let p = tiny();
    let t = |s: &[Stretch]| p.touches(s);
    let one = t(&p.slide(&[pt(-5.0, 5.0), pt(25.0, 5.0)], &|_| 4.0, 1.0));
    // (cell middles are 0.05 mm off the path's line: the chord there)
    let chord = |u: f64| 2.0 * (4.0 - u * u).sqrt() / 4.0;
    assert!((one[49 * 200 + 100] as f64 - chord(0.05)).abs() < 1e-5);
    assert!((one[34 * 200 + 100] as f64 - chord(1.55)).abs() < 1e-5);
    assert_eq!(one[25 * 200 + 100], 0.0);
    let back = t(&p.slide(&[pt(-5.0, 5.0), pt(25.0, 5.0), pt(-5.0, 5.0)], &|_| 4.0, 1.0));
    assert!((back[49 * 200 + 100] - 2.0 * one[49 * 200 + 100]).abs() < 1e-5);
    // a corner: the cell at the corner's outside is rubbed as the pad passes it, once
    let c = t(&p.slide(&[pt(2.0, 2.0), pt(10.0, 2.0), pt(10.0, 9.0)], &|_| 2.0, 1.0));
    // (the cell at (10.45, 1.55) lies 0.45 mm off each leg: under the pad for its chord's part past
    // the corner on the first leg and before it on the second, 2·(h − 0.45) over the 2 mm pad)
    let h = (1.0f64 - 0.45 * 0.45).sqrt();
    let v = c[15 * 200 + 104] as f64;
    assert!((v - 2.0 * (h - 0.45) / 2.0).abs() < 1e-5, "{v}");
    // a pad of 6 mm worked twice round a 1 mm circle over its middle: 2 × 2π mm over 6
    let n = 64;
    let pts: Vec<Point> = (0..=2 * n).map(|i| {
        let a = i as f32 / n as f32 * std::f32::consts::TAU;
        pt(10.0 + a.cos(), 5.0 + a.sin())
    }).collect();
    let r = t(&p.slide(&pts, &|_| 6.0, 100.0));
    let len: f64 = pts.windows(2).map(|w| ((w[1].x - w[0].x) as f64).hypot((w[1].y - w[0].y) as f64)).sum();
    assert!((r[50 * 200 + 100] as f64 - len / 6.0).abs() < 1e-3, "{} against {}", r[50 * 200 + 100], len / 6.0);
}
