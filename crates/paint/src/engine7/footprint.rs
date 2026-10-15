use crate::bristle::{Gesture, Held, Orient, Rect, Tool, footprint, wander};
use crate::path::densify;

pub(crate) struct Footprint;

impl Footprint {
    pub(crate) fn conservative(tool: &Tool, g: &Gesture, scale: f32, w: usize, h: usize) -> Option<Rect> {
        footprint(tool, &g.pts, g.shake, scale, w, h)
    }

    pub(crate) fn exact(held: &Held, g: &Gesture, id: u32, scale: f32, w: usize, h: usize) -> Option<Rect> {
        held.tool.assert_valid();
        g.assert_valid();
        assert!(scale.is_finite() && scale > 0.0, "invalid footprint scale");
        if g.pts.is_empty() || w == 0 || h == 0 {
            return None;
        }
        let tool = &held.tool;
        if tool.point > 0.0 {
            return Self::conservative(tool, g, scale, w, h);
        }
        let pts: Vec<_> = g.pts.iter().map(|&(x, y)| (x * scale, y * scale)).collect();
        let path = if pts.len() >= 2 { densify(&pts) } else { vec![pts[0], pts[0]] };
        let mut arc = vec![0.0f32; path.len()];
        for i in 1..path.len() {
            let (dx, dy) = (path[i].0 - path[i - 1].0, path[i].1 - path[i - 1].1);
            arc[i] = arc[i - 1] + (dx * dx + dy * dy).sqrt();
        }
        let total = arc[arc.len() - 1];
        let rb = (tool.hair_radius() * scale).max(0.55);
        let step = rb.max(1.25);
        let nsteps = ((total / step).ceil() as usize).max(1);
        let bend_len = tool.length * (0.25 + 0.75 * (1.0 - tool.stiffness));
        let rate = (1.0 - (-(step / scale) / (bend_len.max(0.0) + 1e-3)).exp()).clamp(0.0, 1.0);
        let tip = held.bristles.iter().enumerate().min_by(|a, b| a.1.thresh.total_cmp(&b.1.thresh)).map_or(0, |(i, _)| i);
        let mut bends = vec![(0.0f32, 0.0f32); held.bristles.len()];
        let mut prev = vec![None; held.bristles.len()];
        let mut seg = 0usize;
        let mut last_dir = (1.0f32, 0.0f32);
        let mut bounds = None;
        for k in 0..=nsteps {
            let d = (k as f32 * step).min(total);
            while seg + 1 < path.len() - 1 && arc[seg + 1] < d {
                seg += 1;
            }
            let seg_len = (arc[seg + 1] - arc[seg]).max(1e-6);
            let f = ((d - arc[seg]) / seg_len).clamp(0.0, 1.0);
            let (ax, ay) = path[seg];
            let (bx, by) = path[seg + 1];
            let hx = ax + (bx - ax) * f;
            let hy = ay + (by - ay) * f;
            let dir = if seg_len > 1e-3 { ((bx - ax) / seg_len, (by - ay) / seg_len) } else { last_dir };
            last_dir = dir;
            let lw = (tool.width * 2.5 + 6.0) * scale;
            let hs = id as u64 * 7919;
            let off = g.shake * (0.05 * tool.width + 0.12) * scale * (wander(d / lw, hs + 1) + 0.5 * wander(d / (lw * 0.37), hs + 2));
            let (hx, hy) = (hx - dir.1 * off, hy + dir.0 * off);
            let u = if total > 0.0 { d / total } else { 0.5 };
            let p = (g.pressure_at(u) * (1.0 + g.shake * 0.14 * wander(d / (lw * 0.8), hs + 3))).clamp(0.0, 1.0);
            let theta = match g.orient {
                Orient::Across => dir.1.atan2(dir.0) + std::f32::consts::FRAC_PI_2,
                Orient::Along => dir.1.atan2(dir.0),
                Orient::Fixed(a) => a,
            };
            let (st, ct) = theta.sin_cos();
            let half = tool.width * 0.5 * scale * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5));
            for (bi, b) in held.bristles.iter().enumerate() {
                let reach = (p - b.thresh) / (1.0 - b.thresh).max(1e-3);
                if reach <= 0.0 && !(bi == tip && p > 0.0) {
                    prev[bi] = None;
                    continue;
                }
                let wv = wander(d / scale / tool.width.max(2.0) * 1.3, b.seed) * tool.ragged;
                let (ox, oy) = ((b.rx + wv * 0.12) * half, (b.ry + wv * 0.05) * half);
                let root = (hx + ox * ct - oy * st, hy + ox * st + oy * ct);
                let trail = tool.length * scale * b.len * p;
                let spread = tool.splay * p * 0.3;
                let target = (-dir.0 * trail + (ox * ct - oy * st) * spread, -dir.1 * trail + (ox * st + oy * ct) * spread);
                bends[bi].0 += (target.0 - bends[bi].0) * rate;
                bends[bi].1 += (target.1 - bends[bi].1) * rate;
                let cur = (root.0 + bends[bi].0 * 0.6, root.1 + bends[bi].1 * 0.6);
                let a: (f32, f32) = prev[bi].unwrap_or(cur);
                let x0 = ((a.0.min(cur.0) - rb - 1.0).floor().max(0.0)) as usize;
                let y0 = ((a.1.min(cur.1) - rb - 1.0).floor().max(0.0)) as usize;
                let x1 = ((a.0.max(cur.0) + rb + 1.0).ceil().max(0.0) as usize).min(w);
                let y1 = ((a.1.max(cur.1) + rb + 1.0).ceil().max(0.0) as usize).min(h);
                if x1 > x0 && y1 > y0 {
                    let hair = (tool.hair_radius() * scale).min(rb);
                    let off = if b.rx.abs() > 0.7 { (2.0 * hair).max(3.0) } else { 2.0 * hair };
                    let pad = (off + 2.0) as usize;
                    bounds = crate::sched::union(bounds, Some((x0.saturating_sub(pad), y0.saturating_sub(pad), x1.saturating_add(pad).min(w), y1.saturating_add(pad).min(h))));
                }
                prev[bi] = Some(cur);
            }
        }
        bounds
    }

    pub(crate) fn reach(rect: Option<Rect>, center: (f32, f32), scale: f32) -> (f32, f32) {
        match rect {
            Some((x0, y0, x1, y1)) => (
                (center.0 * scale - x0 as f32).max(x1 as f32 - center.0 * scale) / scale,
                (center.1 * scale - y0 as f32).max(y1 as f32 - center.1 * scale) / scale,
            ),
            None => (0.0, 0.0),
        }
    }

    pub(crate) fn tile_size((ex, ey): (f32, f32), gap: f32, scale: f32) -> (f32, f32) {
        let margin = 2.0 / scale;
        ((2.0 * ex + margin).max(gap * 2.0), (2.0 * ey + margin).max(gap * 2.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn contains(outer: Option<Rect>, inner: Option<Rect>) -> bool {
        match (outer, inner) {
            (_, None) => true,
            (Some(a), Some(b)) => a.0 <= b.0 && a.1 <= b.1 && a.2 >= b.2 && a.3 >= b.3,
            _ => false,
        }
    }

    fn tool(i: usize, width: f32) -> Tool {
        match i % 6 {
            0 => Tool::round_sable(width),
            1 => Tool::hog_flat(width),
            2 => Tool::filbert(width),
            3 => Tool::fan(width),
            4 => Tool::rigger(width),
            _ => Tool::badger(width),
        }
    }

    #[test]
    fn exact_is_inside_conservative_for_1000_gestures() {
        let mut rng = Rng::new(716023);
        for i in 0..1000 {
            let mut tool = tool(i, rng.range(0.1, 35.0));
            tool.length *= rng.range(0.0, 2.0);
            tool.splay *= rng.range(0.0, 3.0);
            tool.ragged *= rng.range(0.0, 3.0);
            if i % 7 == 0 { tool.point = rng.range(0.1, 1.0); }
            let held = Held::new(tool, i as u64 * 711);
            let n = i % 7;
            let pts = (0..n).map(|_| (rng.range(-20.0, 230.0), rng.range(-20.0, 170.0))).collect();
            let mut g = Gesture::new(pts)
                .pressure(rng.range(0.0, 1.0), rng.range(0.0, 1.0))
                .ramps(rng.range(0.0, 0.6), rng.range(0.0, 0.6))
                .shake(rng.range(0.0, 3.0))
                .swell(vec![rng.range(0.0, 1.5), rng.range(0.0, 1.5), rng.range(0.0, 1.5)]);
            g.orient = match i % 3 { 0 => Orient::Across, 1 => Orient::Along, _ => Orient::Fixed(rng.range(-6.0, 6.0)) };
            let scale = rng.range(0.1, 5.0);
            let (w, h) = ((210.0 * scale) as usize, (150.0 * scale) as usize);
            let exact = Footprint::exact(&held, &g, i as u32 + 1, scale, w, h);
            let conservative = Footprint::conservative(&held.tool, &g, scale, w, h);
            assert!(contains(conservative, exact), "gesture {i}: {exact:?} outside {conservative:?}");
        }
    }

    #[test]
    fn dry_pass_does_not_change_the_held_brush() {
        let held = Held::new(Tool::filbert(12.0), 71);
        let before = format!("{held:?}");
        let g = Gesture::line((30.0, 50.0), (60.0, 55.0));
        let a = Footprint::exact(&held, &g, 4, 2.0, 200, 200);
        assert_eq!(a, Footprint::exact(&held, &g, 4, 2.0, 200, 200));
        assert_eq!(before, format!("{held:?}"));
    }

    #[test]
    fn pointed_load_uses_the_legacy_envelope() {
        let held = Held::new(Tool { point: 1.0, ..Tool::rigger(2.0) }, 19);
        let g = Gesture::line((20.0, 20.0), (40.0, 25.0)).pressure(0.1, 0.7);
        assert_eq!(Footprint::exact(&held, &g, 3, 2.0, 160, 160), Footprint::conservative(&held.tool, &g, 2.0, 160, 160));
    }

    #[test]
    fn exact_contains_drag_damage_on_wet_paint_across_tools() {
        use crate::color::hex;
        use crate::mask::Mask;
        use crate::style::Style;
        use crate::wet::Paint;

        for i in 0..36 {
            let mut canvas = Style::oil_red_ground().prepare(160, 1.0, 31);
            let frame = canvas.frame();
            let mut held = Held::new(tool(i, 35.0), 71 + i as u64);
            held.load(Paint::body(hex("#dfcf99")), 0.8);
            canvas.wet.vol.fill(1.5);
            canvas.wet.lat.fill(Paint::body(hex("#273899")).latent());
            canvas.wet.hide.fill(Paint::body(hex("#273899")).prop());
            let edge = if i % 2 == 0 { 10.0 } else { 500.0 };
            let g = Gesture::new(vec![(edge, 350.0), (edge + 50.0, 450.0), (edge - 60.0, 520.0), (edge + 100.0, 570.0)])
                .pressure(0.9, 0.3)
                .shake(2.0)
                .orient(match i % 3 { 0 => Orient::Across, 1 => Orient::Along, _ => Orient::Fixed(0.7) });
            let id = canvas.wet.current.wrapping_add(1).max(1);
            let envelope = Footprint::exact(&held, &g, id, frame.scale, frame.full_w, frame.full_h);
            let clip = Mask::from_fn(frame, |x, _| if x < edge + 25.0 { 1.0 } else { 0.0 });
            canvas.drag(&mut held, &g, if i % 2 == 0 { Some(&clip) } else { None });
            assert!(contains(envelope, canvas.wet.dirty), "tool {i}: damage {:?} outside {envelope:?}", canvas.wet.dirty);
            for (pixel, &touched) in canvas.wet.touched.iter().enumerate() {
                if touched == id {
                    let (x, y) = (pixel % frame.w, pixel / frame.w);
                    assert!(contains(envelope, Some((x, y, x + 1, y + 1))));
                }
            }
        }
    }

    #[test]
    fn zero_pressure_and_off_canvas_have_no_envelope() {
        let held = Held::new(Tool::hog_flat(4.0), 8);
        let g = Gesture::line((30.0, 40.0), (60.0, 40.0)).pressure(0.0, 0.0);
        assert_eq!(Footprint::exact(&held, &g, 3, 1.0, 100, 100), None);
        let g = Gesture::line((-300.0, -300.0), (-200.0, -200.0));
        assert_eq!(Footprint::exact(&held, &g, 3, 1.0, 100, 100), None);
    }

    #[test]
    fn filbert_envelope_is_smaller_than_blanket_padding() {
        let held = Held::new(Tool::filbert(24.0), 107);
        let g = Gesture::line((300.0, 250.0), (330.0, 250.0)).pressure(0.7, 0.7);
        let scale = 2400.0 / 900.0;
        let a = Footprint::exact(&held, &g, 8, scale, 2400, 1500).unwrap();
        let b = Footprint::conservative(&held.tool, &g, scale, 2400, 1500).unwrap();
        let area = |r: Rect| (r.2 - r.0) * (r.3 - r.1);
        assert!(area(a) * 2 < area(b), "exact area {} versus conservative {}", area(a), area(b));
    }

    #[test]
    fn narrower_reach_shrinks_tiles_without_removing_the_gap_floor() {
        assert_eq!(Footprint::reach(Some((10, 30, 50, 70)), (30.0, 50.0), 1.0), (20.0, 20.0));
        assert_eq!(Footprint::tile_size((20.0, 10.0), 5.0, 2.0), (41.0, 21.0));
        assert_eq!(Footprint::tile_size((1.0, 1.0), 5.0, 2.0), (10.0, 10.0));
    }
}
