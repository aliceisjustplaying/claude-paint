use crate::bristle::contact_level;
use crate::canvas::Frame;
use rayon::prelude::*;

type Rect = (usize, usize, usize, usize);

const BLOCK: usize = 256;
const TOOTH_UM: f32 = 60.0;

#[derive(Clone)]
pub(crate) struct BaseCache {
    w: usize,
    h: usize,
    origin: (usize, usize),
    r: usize,
    halo: usize,
    cols: usize,
    dirty: Vec<bool>,
    base: Vec<f32>,
}

impl BaseCache {
    pub(crate) fn new(frame: Frame, px_mm: f32) -> Self {
        let (w, h) = (frame.w, frame.h);
        assert!(w > 0 && h > 0);
        assert!(px_mm.is_finite() && px_mm > 0.0);
        let r = ((1.5 / px_mm).round() as usize).max(1);
        let halo = (3 * r).div_ceil(2).max(1) + (r / 3).max(1);
        let cols = w.div_ceil(BLOCK);
        Self {
            w,
            h,
            origin: (frame.x0, frame.y0),
            r,
            halo,
            cols,
            dirty: vec![true; cols * h.div_ceil(BLOCK)],
            base: vec![0.0; w * h],
        }
    }

    pub(crate) fn invalidate(&mut self, rect: Rect) {
        let (ox, oy) = self.origin;
        let (x0, y0, x1, y1) = (
            rect.0.saturating_sub(ox).min(self.w),
            rect.1.saturating_sub(oy).min(self.h),
            rect.2.saturating_sub(ox).min(self.w),
            rect.3.saturating_sub(oy).min(self.h),
        );
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        let (x0, y0) = (x0.saturating_sub(self.halo), y0.saturating_sub(self.halo));
        let (x1, y1) = (
            x1.saturating_add(self.halo).min(self.w),
            y1.saturating_add(self.halo).min(self.h),
        );
        for by in y0 / BLOCK..y1.div_ceil(BLOCK) {
            self.dirty[by * self.cols + x0 / BLOCK..by * self.cols + x1.div_ceil(BLOCK)].fill(true);
        }
    }

    pub(crate) fn invalidate_all(&mut self) {
        self.dirty.fill(true);
    }

    pub(crate) fn rebuild(&mut self, height: &[f32]) -> &[f32] {
        assert_eq!(height.len(), self.w * self.h);
        let source_pixels: usize = self
            .dirty
            .iter()
            .enumerate()
            .filter(|(_, dirty)| **dirty)
            .map(|(block, _)| {
                let (x0, y0) = (block % self.cols * BLOCK, block / self.cols * BLOCK);
                let (x1, y1) = ((x0 + BLOCK).min(self.w), (y0 + BLOCK).min(self.h));
                let width = x1.saturating_add(self.halo).min(self.w) - x0.saturating_sub(self.halo);
                let height =
                    y1.saturating_add(self.halo).min(self.h) - y0.saturating_sub(self.halo);
                width * height
            })
            .sum();
        if source_pixels >= height.len() {
            let low = contact_level(height, self.w, self.h, self.r);
            self.base
                .par_iter_mut()
                .zip(height.par_iter().zip(low.par_iter()))
                .for_each(|(base, (&height, &low))| {
                    *base = (0.5 + (height - low) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3);
                });
            self.dirty.fill(false);
            return &self.base;
        }
        for block in 0..self.dirty.len() {
            if !self.dirty[block] {
                continue;
            }
            let (x0, y0) = (block % self.cols * BLOCK, block / self.cols * BLOCK);
            let (x1, y1) = ((x0 + BLOCK).min(self.w), (y0 + BLOCK).min(self.h));
            let (sx, sy) = (x0.saturating_sub(self.halo), y0.saturating_sub(self.halo));
            let (ex, ey) = ((x1 + self.halo).min(self.w), (y1 + self.halo).min(self.h));
            let (w, h) = (ex - sx, ey - sy);
            let mut source = Vec::with_capacity(w * h);
            for y in sy..ey {
                source.extend_from_slice(&height[y * self.w + sx..y * self.w + ex]);
            }
            let low = contact_level(&source, w, h, self.r);
            for y in y0..y1 {
                for x in x0..x1 {
                    let i = y * self.w + x;
                    let lo = low[(y - sy) * w + x - sx];
                    self.base[i] = (0.5 + (height[i] - lo) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3);
                }
            }
            self.dirty[block] = false;
        }
        &self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cache(w: usize, h: usize, px_mm: f32) -> BaseCache {
        BaseCache::new(Frame::new(w, h, w as f32 / 1000.0), px_mm)
    }

    fn reference(height: &[f32], w: usize, h: usize, px_mm: f32) -> Vec<f32> {
        let r = ((1.5 / px_mm).round() as usize).max(1);
        contact_level(height, w, h, r)
            .iter()
            .zip(height)
            .map(|(&lo, &height)| (0.5 + (height - lo) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3))
            .collect()
    }

    fn assert_close(actual: &[f32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        assert!(actual.iter().chain(expected).all(|value| value.is_finite()));
        let max_error = actual
            .iter()
            .zip(expected)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(max_error < 1e-4, "maximum contact error: {max_error}");
    }

    fn height(w: usize, h: usize) -> Vec<f32> {
        (0..w * h)
            .map(|i| {
                let (x, y) = (i % w, i / w);
                80.0 + ((x * 17 + y * 31) % 101) as f32 * 0.7 + if x > w / 2 { 130.0 } else { 0.0 }
            })
            .collect()
    }

    #[test]
    fn tiny_and_awkward_frames_match_whole_canvas() {
        for (w, h) in [(1, 1), (1, 273), (271, 1), (3, 5), (517, 531)] {
            for px_mm in [
                50.0 / 2400.0,
                200.0 / 2400.0,
                900.0 / 2400.0,
                5000.0 / 2400.0,
            ] {
                let height = height(w, h);
                let mut cache = cache(w, h, px_mm);
                assert_close(cache.rebuild(&height), &reference(&height, w, h, px_mm));
            }
        }
    }

    #[test]
    fn changed_height_invalidates_physical_halo_across_blocks() {
        let (w, h) = (803, 785);
        for px_mm in [
            50.0 / 2400.0,
            200.0 / 2400.0,
            730.0 / 4800.0,
            900.0 / 2400.0,
        ] {
            let mut height = height(w, h);
            let mut cache = cache(w, h, px_mm);
            cache.rebuild(&height);
            for rect in [
                (250, 247, 260, 263),
                (0, 0, 5, 3),
                (797, 778, 803, 785),
                (507, 510, 516, 518),
                (220, 240, 590, 590),
            ] {
                for y in rect.1..rect.3 {
                    for x in rect.0..rect.2 {
                        height[y * w + x] += 75.0;
                    }
                }
                cache.invalidate(rect);
                assert_close(cache.rebuild(&height), &reference(&height, w, h, px_mm));
            }
        }
    }

    #[test]
    fn shifting_mark_across_block_edges_does_not_change_contact() {
        let (w, h) = (1541, 1283);
        for mm in [50.0, 200.0, 730.0, 900.0, 5000.0] {
            let px_mm = mm / 2400.0;
            let mut contacts = Vec::new();
            for (cx, cy) in [(128, 128), (256, 256), (511, 512)] {
                let mut height = vec![80.0; w * h];
                let mut cache = cache(w, h, px_mm);
                cache.rebuild(&height);
                let rect = (cx - 45, cy - 37, cx + 46, cy + 38);
                for y in rect.1..rect.3 {
                    for x in rect.0..rect.2 {
                        let dx = (x as f32 - cx as f32) / 45.0;
                        let dy = (y as f32 - cy as f32) / 37.0;
                        height[y * w + x] += (1.0 - dx * dx - dy * dy).max(0.0) * 150.0;
                    }
                }
                cache.invalidate(rect);
                let base = cache.rebuild(&height);
                assert_close(base, &reference(&height, w, h, px_mm));
                let mut crop = Vec::new();
                for y in cy - 50..=cy + 50 {
                    crop.extend_from_slice(&base[y * w + cx - 55..y * w + cx + 56]);
                }
                contacts.push(crop);
            }
            assert_close(&contacts[0], &contacts[1]);
            assert_close(&contacts[0], &contacts[2]);
        }
    }

    #[test]
    fn invalidation_is_local_and_empty_rectangles_do_nothing() {
        let mut cache = cache(1024, 1024, 900.0 / 2400.0);
        let height = vec![80.0; 1024 * 1024];
        cache.rebuild(&height);
        for rect in [(3, 4, 3, 5), (10, 20, 3, 5), (1024, 1024, 1100, 1200)] {
            cache.invalidate(rect);
            assert!(cache.dirty.iter().all(|dirty| !dirty));
        }
        cache.invalidate((255, 255, 256, 256));
        assert_eq!(cache.dirty.iter().filter(|&&dirty| dirty).count(), 4);
        assert_close(cache.rebuild(&height), &vec![0.5; height.len()]);
        cache.invalidate_all();
        assert!(cache.dirty.iter().all(|&dirty| dirty));
    }

    #[test]
    fn cropped_frames_invalidate_in_global_coordinates() {
        let frame = Frame::new(2400, 1600, 2.4).window((701, 403, 1504, 1188));
        let (w, h) = (frame.w, frame.h);
        let px_mm = 200.0 / 2400.0;
        let mut height = height(w, h);
        let mut cache = BaseCache::new(frame, px_mm);
        cache.rebuild(&height);
        cache.invalidate((0, 0, frame.x0, frame.y0));
        assert!(cache.dirty.iter().all(|dirty| !dirty));
        for y in 250..264 {
            for x in 250..264 {
                height[y * w + x] += 50.0;
            }
        }
        cache.invalidate((
            frame.x0 + 250,
            frame.y0 + 250,
            frame.x0 + 264,
            frame.y0 + 264,
        ));
        assert_close(cache.rebuild(&height), &reference(&height, w, h, px_mm));
    }
}
