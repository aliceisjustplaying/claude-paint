//! Painting passages (tiles of strokes) in parallel without changing what
//! gets painted.
//!
//! `Canvas::work` groups strokes into tiles. Tiles whose pixel footprints
//! are disjoint commute exactly (each has its own brush; they share no
//! pixel), so only tiles that overlap need to keep their order.
//! `run_ordered` starts a tile as soon as every earlier tile it overlaps has
//! finished: the result is bit-for-bit that of running them one by one in
//! order, and no tile waits for tiles it does not overlap.
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

/// Shrink dense work tiles toward 32 strokes per tile, never below half
/// their legacy dimensions. The plan, not the worker count, sets the scale.
pub(crate) fn work_tile_scale(strokes: &[usize]) -> f32 {
    let longest = strokes.iter().copied().max().unwrap_or(0).max(32);
    (32.0 / longest as f32).sqrt().max(0.5)
}

pub(crate) fn quarter_work_tile_scale(strokes: &[usize]) -> f32 {
    if strokes.iter().any(|&n| n > 32) { 0.25 } else { 1.0 }
}

fn disjoint_groups(order: &[usize], rects: &[Option<Rect>]) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for &t in order {
        let Some(a) = rects[t] else { continue };
        if let Some(group) = groups.iter_mut().find(|group| group.iter().all(|&u| {
            let b = rects[u].unwrap();
            !(a.0 < b.2 && b.0 < a.2 && a.1 < b.3 && b.1 < a.3)
        })) {
            group.push(t);
        } else {
            groups.push(vec![t]);
        }
    }
    groups
}

struct Ctx<'a, T, W> {
    order: &'a [usize],
    succ: Vec<Vec<usize>>,
    pending: Vec<AtomicUsize>,
    out: Vec<Mutex<Option<T>>>,
    work: &'a W,
    stats: Option<&'a crate::engine7::BatchStats>,
}

fn run<'a, 's, T: Send, W: Fn(usize) -> T + Sync>(c: &'a Ctx<'a, T, W>, s: &Scope<'s>, k: usize)
where
    'a: 's,
{
    let v = match c.stats {
        Some(st) => st.tile(|| (c.work)(c.order[k])),
        None => (c.work)(c.order[k]),
    };
    *c.out[k].lock().unwrap() = Some(v);
    for &j in &c.succ[k] {
        // the last predecessor to finish starts it (AcqRel: j sees every
        // predecessor's writes)
        if c.pending[j].fetch_sub(1, Ordering::AcqRel) == 1 {
            if let Some(st) = c.stats {
                st.ready(1);
            }
            s.spawn(move |s| run(c, s, j));
        }
    }
}

/// Run `work(i)` for every `i` in `order`, as if one by one in that order,
/// where `rects[i]` bounds every pixel `work(i)` touches. Returns the
/// results in `order`'s order. Counts ready and running tiles into
/// `stats`, if given (`E7_STATS`).
pub(crate) fn run_ordered<T: Send, W: Fn(usize) -> T + Sync>(order: &[usize], rects: &[Option<Rect>], stats: Option<&crate::engine7::BatchStats>, work: W) -> Vec<T> {
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
        stats,
    };
    let ready: Vec<usize> = (0..n).filter(|&k| count[k] == 0).collect();
    if let Some(st) = stats {
        st.ready(ready.len());
    }
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

/// A sweep down: the tiles of a `tw` × `th` grid row by row from the top, alternate tiles within a row (so they can run side by
/// side).
pub(crate) fn sweep_down((tw, th): (usize, usize)) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..tw * th).collect();
    idx.sort_by_key(|&i| (i / tw, (i % tw) % 2, i));
    idx
}

/// The order a pass's tiles are painted in.
pub(crate) enum TileOrder {
    /// An order the caller asked for: kept.
    Asked(Vec<usize>),
    /// The verb's default (checkerboard phases that let tiles run in
    /// parallel): with hand time on, a sweep down instead.
    Default(Vec<usize>),
    /// Dense quarter-sized work tiles: keep the default hand-time batches,
    /// then regroup each whole-canvas batch into disjoint footprint groups.
    Quarter(Vec<usize>),
}

/// The tiles of a covering pass, planned and counted, ready to paint.
pub(crate) struct Pass {
    /// The grid of tiles (columns, rows).
    pub grid: (usize, usize),
    /// The order to paint them in.
    pub order: TileOrder,
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
    /// about that much hand time (`batches`) with the paint ageing
    /// between them (`hand_pass`; the last slice is left owed, for the
    /// caller's clock), and the default order becomes a sweep down
    /// (`sweep_down`) instead of the checkerboard phases. An order asked for
    /// is kept. Each slice takes its stroke ids
    /// together, in tile order (all at once for a single slice), so a wait
    /// between slices sees the next slice's strokes as fresh work; a crop
    /// render then skips the tiles that miss its window (they paint nothing
    /// it holds; the rest keep their relative order), after the ids are
    /// handed out, so it paints with the same ids as the whole canvas.
    pub(crate) fn paint_pass(&mut self, pass: Pass, serial: bool, paint: impl Fn(Surf, usize, u32) -> Bounds + Sync) {
        let Pass { grid, order, rects, secs, ids, slice } = pass;
        let regroup = matches!(order, TileOrder::Quarter(_));
        let order = match order {
            TileOrder::Default(_) | TileOrder::Quarter(_) if slice.is_some() => sweep_down(grid),
            TileOrder::Asked(o) | TileOrder::Default(o) | TileOrder::Quarter(o) => o,
        };
        let batches = match (slice, self.hand_slice_secs()) {
            (Some(limit), Some(base)) if limit > base => bounded_batches(&order, &secs, limit),
            _ => batches(&order, &secs, slice),
        };
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
            let order = if regroup { disjoint_groups(&order, &rects).into_iter().flatten().collect() } else { order };
            let f = self.f;
            let order: Vec<usize> = order.into_iter().filter(|&t| rects[t].is_some_and(|r| f.clip(r).is_some())).collect();
            if std::env::var_os("PAINT_DEBUG").is_some() {
                eprintln!("  {} tiles", order.len());
            }
            let surf = self.surf();
            // tiles run in parallel wherever that can't change the result: a
            // tile starts once every earlier tile (in `order`) it overlaps is
            // done
            let stats = crate::engine7::stats().then(crate::engine7::BatchStats::default);
            let t0 = std::time::Instant::now();
            let dirty = if serial {
                order.iter().map(|&t| paint(surf, t, first.wrapping_add(offsets[t]))).fold(None, union)
            } else {
                run_ordered(&order, &rects, stats.as_ref(), |t| paint(surf, t, first.wrapping_add(offsets[t]))).into_iter().fold(None, union)
            };
            if let Some(st) = &stats {
                st.print(order.len(), t0.elapsed().as_secs_f64());
            }
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

fn bounded_batches(order: &[usize], secs: &[f64], limit: f64) -> Vec<(Vec<usize>, f64)> {
    let mut out: Vec<(Vec<usize>, f64)> = vec![(Vec::new(), 0.0)];
    for &t in order {
        let last = out.last_mut().unwrap();
        if !last.0.is_empty() && last.1 + secs[t] > limit {
            out.push((vec![t], secs[t]));
        } else {
            last.0.push(t);
            last.1 += secs[t];
        }
    }
    out
}
