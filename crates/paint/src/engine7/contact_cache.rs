//! Engine 7's contact level: the level a brush rests on (`Canvas::surf`'s
//! `base`), kept per block and rebuilt only where the height changed.
//!
//! Engine 5 rebuilds the whole canvas's level whenever any height changed
//! (a running median of ±1.5 r along rows then columns, then a box blur of
//! radius r/3, with r = 1.5 mm; `bristle::contact_level`). A pixel's level
//! depends only on the height within `halo` = 1.5 r + r/3 px of it, so a
//! height change in a rect stales only the blocks within that halo. Each
//! stale block is rebuilt from its own window of height grown by the halo,
//! with the same edge rule as the whole-canvas pass at the canvas's edges,
//! so a block's level is the whole-canvas level up to float rounding, and
//! the same bits however the cache got there (which blocks were stale with
//! it, or a fresh cache after a restore).

use rayon::prelude::*;

/// `bristle::TOOTH_UM`: relief (µm) that spans a bristle's contact range.
const TOOTH_UM: f32 = 60.0;
/// The smallest block side (px).
const MIN_BLOCK: usize = 64;

/// A rect of pixels: `(x0, y0, x1, y1)`, end exclusive.
pub(crate) type Rect = (usize, usize, usize, usize);

#[derive(Clone, Default)]
pub(crate) struct ContactCache {
    w: usize,
    h: usize,
    /// 1.5 mm in px, rounded (as `Canvas::surf`).
    r: usize,
    /// The canvas's height generation the level was last brought up to.
    surf_gen: Option<u64>,
    block: usize,
    /// Blocks across.
    bw: usize,
    stale: Vec<bool>,
    /// The height as of the last refresh: what changed since is diffed
    /// against it.
    seen: Vec<f32>,
    base: Vec<f32>,
    /// Pixels of level rebuilt by the last refresh.
    pub(crate) rebuilt: usize,
}

impl ContactCache {
    fn rm(&self) -> usize {
        (3 * self.r).div_ceil(2).max(1)
    }

    fn rb(&self) -> usize {
        (self.r / 3).max(1)
    }

    /// How far (px) a height change reaches into the level.
    fn halo(&self) -> usize {
        self.rm() + self.rb()
    }

    /// The height changed in `rect`: every block whose level reads it is stale.
    pub(crate) fn invalidate(&mut self, rect: Rect) {
        let (x0, y0, x1, y1) = rect;
        if self.stale.is_empty() || x0 >= x1 || y0 >= y1 {
            return;
        }
        let (hl, b) = (self.halo(), self.block);
        let bh = self.stale.len() / self.bw;
        for by in y0.saturating_sub(hl) / b..(y1 + hl).div_ceil(b).min(bh) {
            for bx in x0.saturating_sub(hl) / b..(x1 + hl).div_ceil(b).min(self.bw) {
                self.stale[by * self.bw + bx] = true;
            }
        }
    }

    /// The contact level of `height` (a `w`×`h` canvas of `px_mm` mm per
    /// pixel at height generation `surf_gen`), rebuilt where it changed since the
    /// last call. A new size or pixel pitch rebuilds it all.
    pub(crate) fn refresh(&mut self, height: &[f32], surf_gen: u64, w: usize, h: usize, px_mm: f32) -> &[f32] {
        assert_eq!(height.len(), w * h, "contact cache: height is not {w}×{h}");
        let r = ((1.5 / px_mm).round() as usize).max(1);
        self.rebuilt = 0;
        if (self.w, self.h, self.r) != (w, h, r) || self.seen.len() != w * h {
            *self = ContactCache { w, h, r, ..Default::default() };
            self.block = (2 * self.halo()).max(MIN_BLOCK);
            self.bw = w.div_ceil(self.block);
            self.stale = vec![true; self.bw * h.div_ceil(self.block)];
            self.seen = height.to_vec();
            self.base = vec![0.0; w * h];
        } else if self.surf_gen == Some(surf_gen) {
            return &self.base;
        } else {
            for rect in self.diff(height) {
                self.invalidate(rect);
            }
        }
        self.surf_gen = Some(surf_gen);
        self.rebuild(height);
        &self.base
    }

    /// Bring `seen` up to `height`; returns the bounds of what changed in
    /// each block.
    fn diff(&mut self, height: &[f32]) -> Vec<Rect> {
        let (w, b) = (self.w, self.block);
        self.seen
            .par_chunks_mut(b * w)
            .zip(height.par_chunks(b * w))
            .enumerate()
            .flat_map_iter(|(by, (seen, now))| {
                let mut found: Vec<Option<Rect>> = vec![None; w.div_ceil(b)];
                for (y, (s_row, n_row)) in seen.chunks_mut(w).zip(now.chunks(w)).enumerate() {
                    let y = by * b + y;
                    for (x, (s, &n)) in s_row.iter_mut().zip(n_row).enumerate() {
                        if s.to_bits() != n.to_bits() {
                            *s = n;
                            let f = &mut found[x / b];
                            *f = Some(match *f {
                                None => (x, y, x + 1, y + 1),
                                Some((a, c, d, e)) => (a.min(x), c.min(y), d.max(x + 1), e.max(y + 1)),
                            });
                        }
                    }
                }
                found.into_iter().flatten()
            })
            .collect()
    }

    /// Rebuild every stale block.
    fn rebuild(&mut self, height: &[f32]) {
        let (w, h, b, bw) = (self.w, self.h, self.block, self.bw);
        let blocks: Vec<Rect> = (0..self.stale.len()).filter(|&k| self.stale[k]).map(|k| ((k % bw) * b, (k / bw) * b, ((k % bw + 1) * b).min(w), ((k / bw + 1) * b).min(h))).collect();
        let (rm, rb, hl) = (self.rm(), self.rb(), self.halo());
        let levels: Vec<(Rect, Vec<f32>)> = blocks.into_par_iter().map(|block| (block, level(height, w, h, block, rm, rb, hl))).collect();
        for ((x0, y0, x1, y1), lv) in levels {
            let rw = x1 - x0;
            for y in y0..y1 {
                self.base[y * w + x0..y * w + x1].copy_from_slice(&lv[(y - y0) * rw..(y - y0 + 1) * rw]);
            }
            self.rebuilt += rw * (y1 - y0);
        }
        self.stale.fill(false);
    }
}

/// The level over `run` of a `w`×`h` canvas's `height`, from the window of
/// `run` grown by the halo `hl` (clipped to the canvas, where the median and
/// blur repeat the edge as the whole-canvas pass does). `rm` is the median's
/// reach and `rb` the blur's radius.
fn level(height: &[f32], w: usize, h: usize, run: Rect, rm: usize, rb: usize, hl: usize) -> Vec<f32> {
    let (x0, y0, x1, y1) = run;
    let (wx0, wy0, wx1, wy1) = (x0.saturating_sub(hl), y0.saturating_sub(hl), (x1 + hl).min(w), (y1 + hl).min(h));
    let (ww, wh) = (wx1 - wx0, wy1 - wy0);
    let mut med = vec![0.0f32; ww * wh];
    for y in 0..wh {
        let row = &height[(wy0 + y) * w + wx0..(wy0 + y) * w + wx1];
        running_median(row, rm, &mut med[y * ww..(y + 1) * ww]);
    }
    let (mut col, mut out) = (vec![0.0f32; wh], vec![0.0f32; wh]);
    for x in 0..ww {
        for y in 0..wh {
            col[y] = med[y * ww + x];
        }
        running_median(&col, rm, &mut out);
        for y in 0..wh {
            med[y * ww + x] = out[y];
        }
    }
    let n = (2 * rb + 1) as f32;
    let mut tmp = vec![0.0f32; ww * wh];
    for (s, o) in med.chunks(ww).zip(tmp.chunks_mut(ww)) {
        let mut acc = 0.0f32;
        for k in 0..=2 * rb {
            acc += s[k.saturating_sub(rb).min(ww - 1)];
        }
        for (x, o) in o.iter_mut().enumerate() {
            *o = acc / n;
            acc += s[(x + rb + 1).min(ww - 1)] - s[x.saturating_sub(rb)];
        }
    }
    let (rw, rh) = (x1 - x0, y1 - y0);
    let mut lv = vec![0.0f32; rw * rh];
    for x in x0 - wx0..x1 - wx0 {
        let mut acc = 0.0f32;
        for k in 0..=2 * rb {
            acc += tmp[k.saturating_sub(rb).min(wh - 1) * ww + x];
        }
        for y in 0..wh {
            if (y0 - wy0..y1 - wy0).contains(&y) {
                let i = (wy0 + y) * w + wx0 + x;
                lv[(y + wy0 - y0) * rw + x + wx0 - x0] = (0.5 + (height[i] - acc / n) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3);
            }
            acc += tmp[(y + rb + 1).min(wh - 1) * ww + x] - tmp[y.saturating_sub(rb) * ww + x];
        }
    }
    lv
}

/// `bristle::running_median`: the median of `s[i - r ..= i + r]` (edges
/// repeated) into `out[i]`.
fn running_median(s: &[f32], r: usize, out: &mut [f32]) {
    let n = s.len();
    let at = |i: isize| s[i.clamp(0, n as isize - 1) as usize];
    let mut win: Vec<f32> = (-(r as isize)..=r as isize).map(at).collect();
    win.sort_by(f32::total_cmp);
    for (i, o) in out.iter_mut().enumerate() {
        *o = win[r];
        let (gone, come) = (at(i as isize - r as isize), at(i as isize + r as isize + 1));
        if gone.to_bits() == come.to_bits() {
            continue;
        }
        let from = win.partition_point(|v| v.total_cmp(&gone).is_lt());
        let to = win.partition_point(|v| v.total_cmp(&come).is_lt());
        let to = if to > from { to - 1 } else { to };
        if from < to {
            win.copy_within(from + 1..=to, from);
        } else if to < from {
            win.copy_within(to..from, to + 1);
        }
        win[to] = come;
    }
}
