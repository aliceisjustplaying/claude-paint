use crate::bristle::{Bounds, Bristle, Clip, Surf, Tool};
use super::Rect;

pub(crate) struct SegmentInput<'a> {
    pub sf: Surf,
    pub br: &'a mut Bristle,
    pub tool: &'a Tool,
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub rb: f32,
    pub reach: f32,
    pub full: f32,
    pub dep: Option<f32>,
    pub excl: f32,
    pub clip: Option<Clip<'a>>,
    pub id: u32,
    pub lim: Rect,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Contact {
    pub coverage: f32,
    pub weight: f32,
    pub weighted_tack: f32,
    pub fence_weight: f32,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Budget {
    pub paint: f32,
    pub solvent: f32,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct AcceptedTransfer {
    pub picked: Budget,
    pub laid: Budget,
    pub outside: Budget,
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn exchange(
    sf: Surf,
    br: &mut Bristle,
    tool: &Tool,
    a: (f32, f32),
    b: (f32, f32),
    rb: f32,
    reach: f32,
    full: f32,
    dep: Option<f32>,
    excl: f32,
    clip: Option<Clip<'_>>,
    id: u32,
    wts: &mut Vec<f32>,
    bounds: &mut Bounds,
    lim: Rect,
) {
    unsafe { crate::bristle::exchange_legacy(sf, br, tool, a, b, rb, reach, full, dep, excl, clip, id, wts, bounds, lim) }
}
