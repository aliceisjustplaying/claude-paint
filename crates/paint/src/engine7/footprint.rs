use crate::bristle::{Gesture, Held, Tool};
use super::Rect;

pub(crate) struct Footprint;

impl Footprint {
    pub(crate) fn exact(held: &Held, g: &Gesture, _id: u32, scale: f32, w: usize, h: usize) -> Option<Rect> {
        Self::conservative(&held.tool, g, scale, w, h)
    }

    pub(crate) fn conservative(tool: &Tool, g: &Gesture, scale: f32, w: usize, h: usize) -> Option<Rect> {
        crate::bristle::footprint(tool, &g.pts, g.shake, scale, w, h)
    }
}
