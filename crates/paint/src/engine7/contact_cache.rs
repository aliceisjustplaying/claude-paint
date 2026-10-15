//! A local cache of the brush contact surface for engine 7 (lane STIL-70).
//!
//! Contract (frozen, CONTRACTS-e7.md): `rebuild` returns the surface
//! `Canvas::surf` computes for engine 5,
//! `clamp(0.5 + (height - level) / (2 TOOTH_UM), -0.2, 1.3)` with
//! `level = contact_level(height, w, h, r)` and `r = max(round(1.5 / px_mm), 1)`,
//! buffer-local and row-major, recomputing only what `invalidate` marked,
//! widened by the level's support. A clone owns its own cache; a restored
//! canvas starts with it invalid.
//!
//! Stub: any invalidation rebuilds everything.

use super::Rect;
use crate::canvas::Frame;

#[derive(Clone)]
pub struct BaseCache {
    frame: Frame,
    px_mm: f32,
    base: Vec<f32>,
    valid: bool,
}

impl BaseCache {
    pub fn new(frame: Frame, px_mm: f32) -> Self {
        BaseCache { frame, px_mm, base: Vec::new(), valid: false }
    }

    /// The height changed in `global_rect` (whole-canvas pixels).
    pub fn invalidate(&mut self, global_rect: Rect) {
        let _ = global_rect;
        self.valid = false;
    }

    /// The height may have changed anywhere.
    pub fn invalidate_all(&mut self) {
        self.valid = false;
    }

    /// The contact surface over `height` (the frame's buffer).
    pub fn rebuild(&mut self, height: &[f32]) -> &[f32] {
        let (w, h) = (self.frame.w, self.frame.h);
        assert_eq!(height.len(), w * h, "BaseCache: height buffer doesn't match its frame");
        if !self.valid {
            let r = ((1.5 / self.px_mm).round() as usize).max(1);
            let low = crate::bristle::contact_level(height, w, h, r);
            self.base.clear();
            self.base.extend(height.iter().zip(&low).map(|(&hgt, &lo)| (0.5 + (hgt - lo) / (2.0 * crate::bristle::TOOTH_UM)).clamp(-0.2, 1.3)));
            self.valid = true;
        }
        &self.base
    }
}
