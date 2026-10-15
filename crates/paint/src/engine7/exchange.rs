//! Engine 7's paint exchange between one bristle and the canvas (lane
//! STIL-68).
//!
//! Contract (frozen, CONTRACTS-e7.md): `exchange` takes exactly
//! `bristle::exchange`'s arguments and keeps its physics: one ordered contact
//! along the capsule a–b, whose accepted pickup and deposit change the
//! bristle and the canvas before the next contact. Its stages:
//! `SegmentInput` (the segment with the live brush state) → `Contact` over a
//! `Window` → `Budget` → `AcceptedTransfer`. Requested transfer is never
//! reported as accepted.
//!
//! Stub: engine 5's `bristle::exchange`.

use super::Rect;
use crate::bristle::{Bounds, Bristle, Clip, Surf, Tool};

/// One bristle's segment of a stroke, with the brush as it is now.
#[allow(dead_code)]
pub(crate) struct SegmentInput<'a> {
    pub sf: Surf,
    /// The live bristle: its load changes as the segment is painted.
    pub bristle: &'a mut Bristle,
    pub tool: &'a Tool,
    /// The capsule's ends and radius (whole-canvas pixels).
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub rb: f32,
    /// How far past touching the hair is pressed (0..1).
    pub reach: f32,
    /// A full load's volume for one bristle (`Held::full`).
    pub full: f32,
    /// A fixed deposit (a touch) instead of one by distance travelled.
    pub dep: Option<f32>,
    /// The share of the track this hair lays alone (pointed tools).
    pub excl: f32,
    pub clip: Option<Clip<'a>>,
    pub stroke_id: u32,
    /// The stroke's footprint: no pixel outside it is read or written.
    pub limit: Rect,
}

/// What the capsule touches, summed over its pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Contact {
    /// Geometric coverage (pixels).
    pub coverage: f32,
    /// Contact weight: coverage times how hard the hair presses there.
    pub weight: f32,
    /// Contact-weighted tack of the film.
    pub tack: f32,
    /// Contact weight a fence or mask held back.
    pub fence: f32,
}

/// What may move in this contact, in coats × units² (material volume, not
/// solvent µm on the canvas).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Budget {
    pub paint: f32,
    pub solvent: f32,
}

/// What did move, in coats × units²: picked up into the bristle, laid onto
/// the canvas, and ploughed outside the capsule.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AcceptedTransfer {
    pub picked: f32,
    pub laid: f32,
    pub outside: f32,
}

/// Engine 7's exchange, with `bristle::exchange`'s arguments.
///
/// SAFETY: as `bristle::exchange`: no other thread touches pixels in `lim`.
#[allow(clippy::too_many_arguments)]
#[inline]
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
    unsafe { crate::bristle::exchange(sf, br, tool, a, b, rb, reach, full, dep, excl, clip, id, wts, bounds, lim) }
}
