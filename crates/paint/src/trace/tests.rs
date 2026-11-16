//! Tracing: the ball's contact, the guide's paths, the hand and the coat's
//! bookkeeping. Every expected number is computed here from the inputs.

use super::*;
use crate::graphite::{Lead, Mark};

fn stack() -> Vec<Layer> {
    vec![Layer::of(&tracing_paper(1)), Layer::of(&Paper::drawing(2))]
}

/// A sheet of drawing paper 50 mm wide at 0.125 mm a pixel, with the drawing's optics.
fn sheet() -> Canvas {
    Canvas::new_window(400, 1.0, [0.8; 3], None).with_size_mm(50.0).with_paper(Paper::drawing(4)).with_drawing_optics(true)
}

/// The same paper at 0.0625 mm a pixel: fine enough to see a line widen by a tenth of a mm.
fn fine() -> Canvas {
    Canvas::new_window(400, 1.0, [0.8; 3], None).with_size_mm(25.0).with_paper(Paper::drawing(4)).with_drawing_optics(true)
}

#[test]
fn tracing_paper_is_the_measured_sheet() {
    let p = tracing_paper(1);
    assert!((p.caliper_um() - 76.0).abs() < 0.01, "{}", p.caliper_um());
    // 90 g/m² over 76 µm is 1.18 g/cm³: 21 % pores against the wall's 1.5
    assert!(((1.0 - 90.0 / 76.0 / 1.5) - p.porosity).abs() < 0.005);
}

#[test]
fn more_force_widens_the_line_up_to_the_stack_s_density() {
    let s = stack();
    let r = STYLUS_MM / 2.0;
    let limit = 2.0 * (2.0 * r * s.iter().map(|l| l.t_um * l.porosity).sum::<f32>() * 1e-3).sqrt();
    assert!((width_limit_mm(r, &s) - limit).abs() < 1e-6);
    let mut last = 0.0;
    for f in [0.2, 0.5, 1.0, 2.0, 5.0, 20.0] {
        let c = contact(r, f, &s);
        assert!(c.width_mm() > last, "{f} N: {} after {last}", c.width_mm());
        assert!(c.width_mm() < limit && !c.limited, "{f} N: {} against {limit}", c.width_mm());
        last = c.width_mm();
    }
    // past the force that closes every pore, the line is as wide as it gets
    let c = contact(r, 1e6, &s);
    assert!(c.limited);
    assert!((c.width_mm() - limit).abs() < 1e-5, "{} against {limit}", c.width_mm());
}

#[test]
fn the_contact_carries_the_force() {
    // the ball's pressure summed over its contact, by quadrature of Chen's
    // curve under the sink, against the force: the closed form's check
    let s = stack();
    let r = 0.5f64;
    for f in [0.5f32, 2.0, 5.0] {
        let c = contact(r as f32, f, &s);
        let n = 4000;
        let a = c.a_mm as f64;
        let mut total = 0.0f64;
        for k in 0..n {
            let rr = a * (k as f64 + 0.5) / n as f64;
            let d = c.sink_um as f64 - rr * rr / (2.0 * r) * 1000.0;
            // the pressure at which the layers' compressions add up to d
            let (mut lo, mut hi) = (0.0f64, 1e5f64);
            for _ in 0..80 {
                let m = 0.5 * (lo + hi);
                let sink: f64 = s.iter().map(|l| l.t_um as f64 * ((1.0 + 13.54 * m / l.e_mpa as f64).ln() / 13.54).min(l.porosity as f64)).sum();
                if sink < d { lo = m } else { hi = m }
            }
            total += 0.5 * (lo + hi) * 2.0 * std::f64::consts::PI * rr * (a / n as f64);
        }
        assert!((total - f as f64).abs() / (f as f64) < 0.01, "{f} N carried as {total}");
    }
}

#[test]
fn a_ball_sinks_no_further_than_its_equator() {
    // a needle-fine ball in a thick soft stack: its contact is its own width at most
    let s = vec![Layer { t_um: 2000.0, e_mpa: 1.0, porosity: 0.8 }];
    let c = contact(0.05, 50.0, &s);
    assert!(c.limited);
    assert!((c.a_mm - 0.05).abs() < 1e-6, "{}", c.a_mm);
}

/// The guide of one pencil line from `a` to `b` (units).
fn drawn(a: (f32, f32), b: (f32, f32)) -> Canvas {
    let mut c = sheet();
    let n = 200;
    let pts: Vec<(f32, f32)> = (0..=n).map(|k| {
        let t = k as f32 / n as f32;
        (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
    }).collect();
    let pressure = vec![0.6; pts.len()];
    c.draw(&Lead::pencil("2B").unwrap(), &Mark { pts, pressure }, 0.0, 7);
    c
}

#[test]
fn the_tracer_s_paths_run_down_the_middle_of_the_lines() {
    let (a, b) = ((100.0, 300.0), (900.0, 700.0));
    let c = drawn(a, b);
    let g = c.drawing_guide();
    let (paths, w) = guide_paths(&g, LEVEL);
    assert!(!paths.is_empty());
    let s = g.f.scale;
    let (ax, ay, bx, by) = (a.0 * s, a.1 * s, b.0 * s, b.1 * s);
    let (dx, dy) = (bx - ax, by - ay);
    let len = (dx * dx + dy * dy).sqrt();
    let mut walked = 0.0;
    for p in &paths {
        for &(x, y) in p {
            let off = ((x - ax) * dy - (y - ay) * dx).abs() / len;
            assert!(off < 1.5, "a path point {off:.2} px off the line");
        }
        walked += crate::path::arclen(p).last().copied().unwrap_or(0.0);
    }
    // the whole line, one path (or a few that meet), less the ends' half widths
    assert!(walked > 0.9 * len && walked < 1.1 * len, "{walked} px walked of {len}");
    // its width: the 2B point's, in px (0.32 + 0.04 mm at pressure 0.6, as draws)
    let want = Lead::pencil("2B").unwrap().width_mm(0.0) * (0.8 + 0.4 * 0.6) / c.px_mm();
    assert!(w > 0.5 * want && w < 2.0 * want.max(2.0) + 2.0, "{w} px against {want}");
    // a level above any line: nothing to follow
    assert!(guide_paths(&g, 1.01).0.is_empty());
}

#[test]
fn the_hand_strays_by_the_line_s_effective_width() {
    let path: Vec<(f32, f32)> = (0..=4000).map(|k| (k as f32, 100.0)).collect();
    for generations in [1, 2] {
        let h = Hand { w_px: 4.0, generations };
        let p = h.follow(&path, 3);
        let ys: Vec<f32> = p.iter().map(|q| q.1 - 100.0).collect();
        let mean = ys.iter().sum::<f32>() / ys.len() as f32;
        let sd = (ys.iter().map(|y| (y - mean).powi(2)).sum::<f32>() / ys.len() as f32).sqrt();
        let want = 4.0 / EFFECTIVE_WIDTH * (generations as f32).sqrt();
        assert!((sd - want).abs() < 0.1 * want, "{generations}: {sd} against {want}");
    }
    // and takes the steering law's time: L·b/W, one set-down a pass
    let h = Hand { w_px: 4.0, generations: 2 };
    assert!((h.secs(400.0) - 2.0 * (SET_DOWN_S + 400.0 * STEER_S / 4.0)).abs() < 1e-4);
}

fn line_paths(c: &Canvas, y_units: f32) -> Vec<Vec<(f32, f32)>> {
    let s = c.window().scale;
    vec![(0..=60).map(|k| (s * (100.0 + 800.0 * k as f32 / 60.0), s * y_units)).collect()]
}

fn whole(c: &Canvas) -> Back {
    let f = c.window();
    (0.0, 0.0, f.full_w as f32, f.full_h as f32)
}

fn red() -> Coat {
    Coat { color: [0.45, 0.12, 0.08], lift: crate::pastel::PASTEL_LIFT, carbon: 0.0 }
}

#[test]
fn what_leaves_the_back_lands_on_the_sheet() {
    let mut c = sheet();
    let before = c.loose_mm3();
    let s = c.window().scale;
    // two lines crossing: the second takes what the first left on the back
    let mut paths = line_paths(&c, 500.0);
    paths.push((0..=60).map(|k| (s * 500.0, s * (100.0 + 800.0 * k as f32 / 60.0))).collect());
    let ct = contact(0.5, 2.5, &stack());
    let mu = tracing_paper(1).pore_um();
    let laid = c.trace_transfer(&paths, whole(&c), ct, Some((red(), mu)));
    assert!(laid.moved_mm3 > 0.0);
    let gained = c.loose_mm3() - before;
    assert!((gained - laid.moved_mm3).abs() <= 1e-4 * laid.moved_mm3, "{gained} gained, {} left the back", laid.moved_mm3);
    assert!(laid.moved_mm3 < laid.coat_mm3);
    // where the lines cross, the back gave no more than it held
    let f = c.window();
    let (x, y) = ((500.0 * s) as usize, (500.0 * s) as usize);
    let cell = &c.drawing.as_ref().unwrap().cells[y * f.w + x];
    assert!(cell.loose <= mu * 1.0001, "{} µm laid from a {mu} µm coat", cell.loose);
}

/// Pixels across the traced line (a column's count where it took any coat), averaged.
fn traced_width_px(force: f32) -> f32 {
    let mut c = fine();
    let paths = line_paths(&c, 500.0);
    let ct = contact(0.5, force, &stack());
    c.trace_transfer(&paths, whole(&c), ct, Some((red(), tracing_paper(1).pore_um())));
    let f = c.window();
    let d = c.drawing.as_ref().unwrap();
    let cols: Vec<usize> = (150..250).collect();
    cols.iter().map(|&x| (0..f.h).filter(|&y| d.cells[y * f.w + x].loose > 0.0).count() as f32).sum::<f32>() / cols.len() as f32
}

#[test]
fn a_heavier_hand_lays_a_wider_line() {
    let (light, heavy) = (traced_width_px(0.5), traced_width_px(5.0));
    assert!(heavy > light, "{heavy} px at 5 N, {light} px at 0.5 N");
    // as wide as the contact: 2a over the pixel, within a pixel either side
    let px = fine().px_mm();
    for (f, w) in [(0.5, light), (5.0, heavy)] {
        let want = contact(0.5, f, &stack()).width_mm() / px;
        assert!((w - want).abs() <= 2.0, "{f} N: {w} px against {want}");
    }
}

#[test]
fn a_blind_stylus_lays_only_the_guide() {
    let mut c = sheet();
    let paths = line_paths(&c, 500.0);
    let laid = c.trace_transfer(&paths, whole(&c), contact(0.5, 2.5, &stack()), None);
    assert_eq!(laid.moved_mm3, 0.0);
    assert_eq!(c.loose_mm3(), 0.0);
    let g = c.drawing_guide();
    let s = g.f.scale;
    let k = (500.0 * s) as usize * g.f.w + (500.0 * s) as usize;
    assert!(g.data[k] > 0.9, "the guide reads {} on the line", g.data[k]);
}

#[test]
fn off_the_back_nothing_is_laid() {
    let mut c = sheet();
    let paths = line_paths(&c, 500.0);
    let f = c.window();
    // the back covers the left half only
    let back = (0.0, 0.0, f.full_w as f32 / 2.0, f.full_h as f32);
    c.trace_transfer(&paths, back, contact(0.5, 2.5, &stack()), Some((red(), 2.0)));
    let d = c.drawing.as_ref().unwrap();
    for y in 0..f.h {
        for x in f.w / 2 + 1..f.w {
            assert_eq!(d.cells[y * f.w + x].loose, 0.0);
        }
    }
    assert!((0..f.h).any(|y| d.cells[y * f.w + f.w / 4].loose > 0.0));
}
