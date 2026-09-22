//! Edges of a region, as a painter follows them when cutting in: the 0.5
//! iso-line of a mask, traced with marching squares into polylines (units).

use crate::mask::Mask;
use std::collections::HashMap;

/// Contour polylines of `m` at level 0.5, sampled on a grid of `step` units.
/// Each polyline also returns, per point, the unit normal pointing into the
/// region (mask increasing).
pub fn contours(m: &Mask, step: f32) -> Vec<Vec<((f32, f32), (f32, f32))>> {
    let f = m.f;
    let (w, h) = (f.width(), f.height());
    let nx = (w / step).ceil() as usize + 1;
    let ny = (h / step).ceil() as usize + 1;
    let at = |i: usize, j: usize| m.data[f.index((i as f32 * step).min(w - 1e-3), (j as f32 * step).min(h - 1e-3))];
    let v: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| (i, j))).map(|(i, j)| at(i, j)).collect();
    let val = |i: usize, j: usize| v[j * nx + i];
    // segments between edge crossings, keyed by edge id so they chain exactly
    // edge id: (i, j, dir) with dir 0 = horizontal edge (i,j)-(i+1,j), 1 = vertical (i,j)-(i,j+1)
    type E = (usize, usize, u8);
    let cross = |e: E| -> (f32, f32) {
        let (i, j, d) = e;
        let (a, b, (x0, y0), (x1, y1)) = if d == 0 {
            (val(i, j), val(i + 1, j), (i, j), (i + 1, j))
        } else {
            (val(i, j), val(i, j + 1), (i, j), (i, j + 1))
        };
        let t = ((0.5 - a) / (b - a)).clamp(0.0, 1.0);
        ((x0 as f32 + (x1 as f32 - x0 as f32) * t) * step, (y0 as f32 + (y1 as f32 - y0 as f32) * t) * step)
    };
    let mut adj: HashMap<E, Vec<E>> = HashMap::new();
    let mut link = |a: E, b: E| {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    };
    for j in 0..ny - 1 {
        for i in 0..nx - 1 {
            let c = [val(i, j) >= 0.5, val(i + 1, j) >= 0.5, val(i + 1, j + 1) >= 0.5, val(i, j + 1) >= 0.5];
            let (top, right, bottom, left) = ((i, j, 0u8), (i + 1, j, 1u8), (i, j + 1, 0u8), (i, j, 1u8));
            let idx = c[0] as u8 | (c[1] as u8) << 1 | (c[2] as u8) << 2 | (c[3] as u8) << 3;
            match idx {
                0 | 15 => {}
                1 | 14 => link(left, top),
                2 | 13 => link(top, right),
                3 | 12 => link(left, right),
                4 | 11 => link(right, bottom),
                6 | 9 => link(top, bottom),
                7 | 8 => link(left, bottom),
                5 => {
                    link(left, top);
                    link(right, bottom);
                }
                10 => {
                    link(top, right);
                    link(left, bottom);
                }
                _ => unreachable!(),
            }
        }
    }
    // chain into polylines: start from open ends first, then closed loops
    let mut keys: Vec<E> = adj.keys().copied().collect();
    keys.sort();
    let mut used: HashMap<E, bool> = HashMap::new();
    let mut lines = Vec::new();
    let walk = |start: E, used: &mut HashMap<E, bool>| {
        let mut line = vec![start];
        used.insert(start, true);
        let mut cur = start;
        loop {
            let next = adj[&cur].iter().copied().find(|n| !used.get(n).copied().unwrap_or(false));
            match next {
                Some(n) => {
                    used.insert(n, true);
                    line.push(n);
                    cur = n;
                }
                None => break,
            }
        }
        line
    };
    for &k in keys.iter().filter(|k| adj[*k].len() == 1) {
        if !used.get(&k).copied().unwrap_or(false) {
            lines.push(walk(k, &mut used));
        }
    }
    for &k in &keys {
        if !used.get(&k).copied().unwrap_or(false) {
            lines.push(walk(k, &mut used));
        }
    }
    // points and inward normals (mask gradient)
    let grad = |x: f32, y: f32| {
        let e = step;
        let s = |x: f32, y: f32| m.data[f.index(x.clamp(0.0, w - 1e-3), y.clamp(0.0, h - 1e-3))];
        let (gx, gy) = (s(x + e, y) - s(x - e, y), s(x, y + e) - s(x, y - e));
        let n = (gx * gx + gy * gy).sqrt().max(1e-6);
        (gx / n, gy / n)
    };
    lines
        .into_iter()
        .filter(|l| l.len() >= 2)
        .map(|l| l.into_iter().map(|e| {
            let p = cross(e);
            (p, grad(p.0, p.1))
        }).collect())
        .collect()
}
