use crate::Frame;
use super::Rect;

#[derive(Clone)]
pub(crate) struct BaseCache {
    frame: Frame,
    px_mm: f32,
    dirty: bool,
    base: Vec<f32>,
}

impl BaseCache {
    pub(crate) fn new(frame: Frame, px_mm: f32) -> Self {
        Self { frame, px_mm, dirty: true, base: Vec::new() }
    }

    pub(crate) fn invalidate(&mut self, _rect: Rect) {
        self.invalidate_all();
    }

    pub(crate) fn invalidate_all(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn rebuild(&mut self, height: &[f32]) -> &[f32] {
        if self.dirty {
            let r = ((1.5 / self.px_mm).round() as usize).max(1);
            let low = crate::bristle::contact_level(height, self.frame.w, self.frame.h, r);
            self.base = height.iter().zip(low).map(|(&height, low)| (0.5 + (height - low) / 120.0).clamp(-0.2, 1.3)).collect();
            self.dirty = false;
        }
        &self.base
    }
}
