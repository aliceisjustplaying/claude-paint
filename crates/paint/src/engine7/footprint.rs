//! Brush footprints for engine 7 (lane STIL-67).
//!
//! Contract (frozen, CONTRACTS-e7.md):
//! - `conservative` is the legacy safety bound, `bristle::footprint`.
//! - `exact(held, g, id, ..)` is every pixel `drag_on` reads or writes for
//!   that brush, gesture and stroke id (capsules, plough destinations and the
//!   exchange's ±(rb + 1) reads), never clipped by a mask. It is a subset of
//!   `conservative`. Pointed tools may fall back to the conservative bound.
//! - `planned(tool, pts, shake, ..)` bounds `exact` for every brush of that
//!   tool and every stroke id, before the brush that paints the stroke is
//!   known: `work` schedules tiles by it, so it must be a true bound, never
//!   an estimate.
//!
//! Stubs: all three return the legacy bound.

use super::Rect;
use crate::bristle::{Gesture, Held, Tool, footprint};

pub struct Footprint;

impl Footprint {
    /// The pixels a drag of `held` through `g` as stroke `id` reads or writes,
    /// on a `w` × `h` px canvas at `scale` px per unit.
    pub fn exact(held: &Held, g: &Gesture, id: u32, scale: f32, w: usize, h: usize) -> Option<Rect> {
        let _ = id;
        Self::conservative(&held.tool, g, scale, w, h)
    }

    /// The legacy safety bound (`bristle::footprint`).
    pub fn conservative(tool: &Tool, g: &Gesture, scale: f32, w: usize, h: usize) -> Option<Rect> {
        footprint(tool, &g.pts, g.shake, scale, w, h)
    }

    /// A bound on `exact` over every brush of `tool` and every stroke id, for
    /// a stroke along `pts` (units) with hand shake `shake`: what `work`
    /// schedules by. Same arguments as `bristle::footprint`.
    pub fn planned(tool: &Tool, pts: &[(f32, f32)], shake: f32, scale: f32, w: usize, h: usize) -> Option<Rect> {
        footprint(tool, pts, shake, scale, w, h)
    }
}
