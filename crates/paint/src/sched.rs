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
//!
//! `Canvas::paint_pass` runs a covering verb's tiles (`work`'s strokes,
//! `stipple`'s touches) through it: stroke ids, the crop, the dirty bounds
//! and, with hand time on, the slices of hand time with the paint ageing
//! between them.

use crate::bristle::{Bounds, Surf};
use crate::canvas::Canvas;
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

/// The union of two dirty bounds.
pub(crate) fn union(a: Bounds, b: Bounds) -> Bounds {
    match (a, b) {
        (None, r) | (r, None) => r,
        (Some(a), Some(b)) => Some((a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3))),
    }
}

/// A hand working down a passage: the tiles of a `tw` × `th` grid row by
/// row from the top, alternate tiles within a row (so they can run side by
/// side).
pub(crate) fn sweep_down((tw, th): (usize, usize)) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..tw * th).collect();
    idx.sort_by_key(|&i| (i / tw, (i % tw) % 2, i));
    idx
}

/// The tiles of a covering pass, planned and counted, ready to paint.
pub(crate) struct Pass {
    /// The grid of tiles (columns, rows).
    pub grid: (usize, usize),
    /// The order to paint them in: the one asked for (`asked`), or the
    /// verb's default (checkerboard phases that let tiles run in parallel).
    pub order: Vec<usize>,
    pub asked: bool,
    /// Each tile's pixel footprint (None: it paints nothing).
    pub rects: Vec<Option<Rect>>,
    /// Each tile's hand time (s), as counted in the ledger.
    pub secs: Vec<f64>,
    /// How many new stroke ids each tile takes.
    pub ids: Vec<u32>,
    /// The hand time (s) to cut it into slices of (hand time on), else None.
    /// Not for strokes whose ids were fixed in advance: a wait's watermark
    /// needs the strokes after it to take later ids.
    pub slice: Option<f64>,
}

impl Canvas {
    /// Paint a pass tile by tile: `paint(surf, tile, first_id)` paints one
    /// tile, its strokes taking ids from `first_id` on in order, and returns
    /// the pixels it dirtied.
    ///
    /// With a `slice` (hand time on), the tiles are painted in slices of
    /// about the canvas's hand slice (`batches`) with the paint ageing
    /// between them (`hand_pass`; the last slice is left owed, for the
    /// caller's clock), and the default order becomes a sweep down: a hand
    /// works down a passage, not in the phases that let tiles run in
    /// parallel. An order asked for is kept. Each slice takes its stroke ids
    /// together, in tile order (all at once for a single slice), so a wait
    /// between slices sees the next slice's strokes as fresh work; a crop
    /// render then skips the tiles that miss its window (they paint nothing
    /// it holds; the rest keep their relative order), after the ids are
    /// handed out, so it paints with the same ids as the whole canvas.
    pub(crate) fn paint_pass(&mut self, pass: Pass, paint: impl Fn(Surf, usize, u32) -> Bounds + Sync) {
        let Pass { grid, order, asked, rects, secs, ids, slice } = pass;
        let order = if slice.is_some() && !asked { sweep_down(grid) } else { order };
        let batches = batches(&order, &secs, slice);
        let n_batches = batches.len();
        let mut offsets = vec![0u32; ids.len()];
        for (bi, (order, bsecs)) in batches.into_iter().enumerate() {
            let mut in_batch = order.clone();
            in_batch.sort_unstable();
            let n: u32 = in_batch.iter().map(|&t| ids[t]).sum();
            let first = if n > 0 { self.next_stroke_ids(n) } else { 0 };
            let mut acc = 0u32;
            for &t in &in_batch {
                offsets[t] = acc;
                acc += ids[t];
            }
            let f = self.f;
            let order: Vec<usize> = order.into_iter().filter(|&t| rects[t].is_some_and(|r| f.clip(r).is_some())).collect();
            if std::env::var_os("PAINT_DEBUG").is_some() {
                eprintln!("  {} tiles", order.len());
            }
            let surf = self.surf();
            // tiles run in parallel wherever that can't change the result: a
            // tile starts once every earlier tile (in `order`) it overlaps is
            // done
            let dirty = run_ordered(&order, &rects, |t| paint(surf, t, first.wrapping_add(offsets[t]))).into_iter().fold(None, union);
            if let Some((x0, y0, x1, y1)) = dirty {
                self.wet.touch(x0, y0, x1, y1);
            }
            if bi + 1 < n_batches {
                self.hand_pass(bsecs);
            }
        }
    }
}

/// Tiles (indices in painting `order`) in consecutive batches of about
/// `slice` seconds of hand time each (`secs` per tile), with each batch's
/// seconds. No slice (hand time off): one batch of everything. The cut
/// follows the whole canvas's plan, so a crop ages its window on the same
/// timeline.
fn batches(order: &[usize], secs: &[f64], slice: Option<f64>) -> Vec<(Vec<usize>, f64)> {
    let Some(slice) = slice.filter(|s| *s > 0.0) else {
        return vec![(order.to_vec(), order.iter().map(|&t| secs[t]).sum())];
    };
    let mut out: Vec<(Vec<usize>, f64)> = vec![(Vec::new(), 0.0)];
    for &t in order {
        let last = out.last_mut().unwrap();
        if last.1 >= slice {
            out.push((vec![t], secs[t]));
        } else {
            last.0.push(t);
            last.1 += secs[t];
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batches_cut_the_order_by_hand_time() {
        let secs = [30.0, 50.0, 0.0, 70.0, 10.0, 90.0];
        let order = [5, 0, 1, 2, 3, 4];
        assert_eq!(batches(&order, &secs, None), vec![(order.to_vec(), 250.0)]);
        let b = batches(&order, &secs, Some(60.0));
        assert_eq!(b, vec![(vec![5], 90.0), (vec![0, 1], 80.0), (vec![2, 3], 70.0), (vec![4], 10.0)]);
    }

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
