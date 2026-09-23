//! Painting passages (tiles of strokes) in parallel without changing what
//! gets painted.
//!
//! `Canvas::work` groups strokes into tiles and used to paint them in four
//! checkerboard phases, each phase waiting for its slowest tile. But tiles
//! whose pixel footprints are disjoint commute exactly (each has its own
//! brush; they share no pixel), so only tiles that overlap need to keep
//! their order. `run_ordered` starts a tile as soon as every earlier tile it
//! overlaps has finished: the result is bit-for-bit that of running them one
//! by one in order, and later phases no longer wait for unrelated tiles.

use rayon::Scope;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A pixel rectangle (x0, y0, x1, y1), end-exclusive.
pub(crate) type Rect = (usize, usize, usize, usize);

struct Ctx<'a, T, W> {
    order: &'a [usize],
    succ: Vec<Vec<usize>>,
    pending: Vec<AtomicUsize>,
    out: Vec<Mutex<Option<T>>>,
    work: &'a W,
}

fn run<'a, 's, T: Send, W: Fn(usize) -> T + Sync>(c: &'a Ctx<'a, T, W>, s: &Scope<'s>, k: usize)
where
    'a: 's,
{
    let v = (c.work)(c.order[k]);
    *c.out[k].lock().unwrap() = Some(v);
    for &j in &c.succ[k] {
        // the last predecessor to finish starts it (AcqRel: j sees every
        // predecessor's writes)
        if c.pending[j].fetch_sub(1, Ordering::AcqRel) == 1 {
            s.spawn(move |s| run(c, s, j));
        }
    }
}

/// Run `work(i)` for every `i` in `order`, as if one by one in that order,
/// where `rects[i]` bounds every pixel `work(i)` touches. Returns the
/// results in `order`'s order.
pub(crate) fn run_ordered<T: Send, W: Fn(usize) -> T + Sync>(order: &[usize], rects: &[Option<Rect>], work: W) -> Vec<T> {
    let n = order.len();
    let overlaps = |a: Rect, b: Rect| a.0 < b.2 && b.0 < a.2 && a.1 < b.3 && b.1 < a.3;
    let r: Vec<Rect> = order.iter().map(|&i| rects[i].expect("tile without footprint")).collect();
    let mut succ = vec![Vec::new(); n];
    let mut count = vec![0usize; n];
    // rects no wider (taller) than cw (ch) can only overlap if their corners
    // lie in neighboring cells of that size: look only there
    let cw = r.iter().map(|q| q.2 - q.0).max().unwrap_or(1).max(1);
    let ch = r.iter().map(|q| q.3 - q.1).max().unwrap_or(1).max(1);
    let mut seen: std::collections::HashMap<(usize, usize), Vec<usize>> = std::collections::HashMap::new();
    for j in 0..n {
        let (cx, cy) = (r[j].0 / cw, r[j].1 / ch);
        for gy in cy.saturating_sub(1)..=cy + 1 {
            for gx in cx.saturating_sub(1)..=cx + 1 {
                for &i in seen.get(&(gx, gy)).map_or(&[][..], |v| v.as_slice()) {
                    if overlaps(r[i], r[j]) {
                        succ[i].push(j);
                        count[j] += 1;
                    }
                }
            }
        }
        seen.entry((cx, cy)).or_default().push(j);
    }
    let ctx = Ctx {
        order,
        succ,
        pending: count.iter().map(|&c| AtomicUsize::new(c)).collect(),
        out: (0..n).map(|_| Mutex::new(None)).collect(),
        work: &work,
    };
    let ready: Vec<usize> = (0..n).filter(|&k| count[k] == 0).collect();
    rayon::scope(|s| {
        for k in ready {
            let c = &ctx;
            s.spawn(move |s| run(c, s, k));
        }
    });
    ctx.out.into_iter().map(|m| m.into_inner().unwrap().expect("tile never ran (dependency cycle?)")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Overlapping items keep their order; the rest may interleave.
    #[test]
    fn keeps_order_of_overlapping_items() {
        let rects: Vec<Option<Rect>> = (0..40).map(|i| Some(((i % 8) * 10, (i / 8) * 10, (i % 8) * 10 + 15, (i / 8) * 10 + 15))).collect();
        let order: Vec<usize> = (0..40).rev().collect();
        let log = Mutex::new(Vec::new());
        let out = run_ordered(&order, &rects, |i| {
            log.lock().unwrap().push(i);
            i * 2
        });
        assert_eq!(out, order.iter().map(|i| i * 2).collect::<Vec<_>>());
        let log = log.into_inner().unwrap();
        let pos = |i: usize| log.iter().position(|&v| v == i).unwrap();
        for (a, &i) in order.iter().enumerate() {
            for &j in &order[a + 1..] {
                let (p, q) = (rects[i].unwrap(), rects[j].unwrap());
                if p.0 < q.2 && q.0 < p.2 && p.1 < q.3 && q.1 < p.3 {
                    assert!(pos(i) < pos(j), "{i} must run before {j}");
                }
            }
        }
    }
}
