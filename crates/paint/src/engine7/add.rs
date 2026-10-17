//! Engine 7's paint laying: engine 5's `Surf::add` (paint mixed by volume
//! into a pixel's latent color, properties and cure) with everything that
//! depends only on the paint being laid worked out once per source instead
//! of once per pixel, and the 7 latent channels and 5 properties each
//! blended as two overlapping 4-lane rows (`[0..4]` and `[3..7]`, `[0..4]`
//! and `[1..5]`). Every lane runs engine 5's arithmetic in engine 5's order,
//! so a pixel ends bit for bit where engine 5 leaves it.

use crate::bristle::{OIL_SHARE, Surf};
use crate::drying::Px;
use crate::wet::{Latent, Prop};

/// The wet film's buffers as `add` writes them: one pixel's volume, latent
/// color, properties, drying state and ground absorbency per index.
#[derive(Clone, Copy)]
pub(crate) struct Film {
    vol: *mut f32,
    lat: *mut Latent,
    hide: *mut Prop,
    /// Null when laid paint leaves the cure to the next `wait` (engine 1)
    /// or the canvas has not waited yet.
    dry: *mut Px,
    /// Null when no part of the ground is absorbent.
    absorb: *mut f32,
}

/// Paint ready to lay: the source's color and properties with the solvent
/// that evaporates as it is laid (engine 4's lean film) already taken out.
#[derive(Clone, Copy)]
pub(crate) struct Paint7 {
    lat: [[f32; 4]; 2],
    hide: [[f32; 4]; 2],
    /// Share of the laid volume left once the solvent is gone.
    keep: f32,
    cure: f32,
}

impl Paint7 {
    #[inline(always)]
    pub(crate) fn new(lean: bool, lat: &Latent, hide: Prop, cure: f32) -> Paint7 {
        let (keep, hide) = if lean && hide[3] > 0.0 {
            let mut h = hide;
            h[3] = 0.0;
            (1.0 - hide[3].clamp(0.0, 0.95), h)
        } else {
            (1.0, hide)
        };
        Paint7 {
            lat: [[lat[0], lat[1], lat[2], lat[3]], [lat[3], lat[4], lat[5], lat[6]]],
            hide: [[hide[0], hide[1], hide[2], hide[3]], [hide[1], hide[2], hide[3], hide[4]]],
            keep,
            cure,
        }
    }
}

#[inline(always)]
fn lerp4(p: [f32; 4], q: [f32; 4], a: f32) -> [f32; 4] {
    [p[0] + (q[0] - p[0]) * a, p[1] + (q[1] - p[1]) * a, p[2] + (q[2] - p[2]) * a, p[3] + (q[3] - p[3]) * a]
}

impl Film {
    /// The film of `sf`'s buffers, taken once per exchange call.
    pub(crate) fn of(sf: &Surf) -> Film {
        Film { vol: sf.vol, lat: sf.lat, hide: sf.hide, dry: if sf.cure_now { sf.dry } else { std::ptr::null_mut() }, absorb: sf.absorb }
    }

    /// `add`, for paint a brush lays from its load (`Surf::lay`): with
    /// `skim` > 0 it mixes with only the top `skim` coats of the wet paint
    /// at `i`, the rest lying buried under it.
    ///
    /// # Safety
    /// As `add`.
    #[inline(always)]
    pub(crate) unsafe fn lay(&self, i: usize, v: f32, p: &Paint7, skim: f32) {
        unsafe {
            if skim <= 0.0 {
                return self.add(i, v, p);
            }
            let buried = (*self.vol.add(i) - skim).max(0.0);
            *self.vol.add(i) -= buried;
            self.add(i, v, p);
            *self.vol.add(i) += buried;
        }
    }

    /// Lay `v` coats of `p` at pixel `i`, as engine 5's `Surf::add` does.
    ///
    /// # Safety
    /// `i` indexes every non-null buffer, and no other thread touches pixel
    /// `i` meanwhile.
    #[inline(always)]
    pub(crate) unsafe fn add(&self, i: usize, v: f32, p: &Paint7) {
        if v <= 0.0 {
            return;
        }
        let v = v * p.keep;
        unsafe {
            let vol = &mut *self.vol.add(i);
            let t = *vol + v;
            let a = v / t;
            // both rows are read before either is written: the shared lanes
            // (latent 3, properties 1–3) blend once from the old value and
            // are written twice with the same result
            let l = self.lat.add(i).cast::<f32>();
            let (l0, l1) = (l.cast::<[f32; 4]>().read_unaligned(), l.add(3).cast::<[f32; 4]>().read_unaligned());
            l.cast::<[f32; 4]>().write_unaligned(lerp4(l0, p.lat[0], a));
            l.add(3).cast::<[f32; 4]>().write_unaligned(lerp4(l1, p.lat[1], a));
            let h = self.hide.add(i).cast::<f32>();
            let (h0, h1) = (h.cast::<[f32; 4]>().read_unaligned(), h.add(1).cast::<[f32; 4]>().read_unaligned());
            h.cast::<[f32; 4]>().write_unaligned(lerp4(h0, p.hide[0], a));
            h.add(1).cast::<[f32; 4]>().write_unaligned(lerp4(h1, p.hide[1], a));
            if !self.dry.is_null() {
                // (a film too thin for `wait` to count is bare: no cure)
                let d = &mut *self.dry.add(i);
                d.cure = if t < 1e-5 { 0.0 } else { d.cure + (p.cure - d.cure) * a };
            }
            *vol = t;
            if self.absorb.is_null() {
                return;
            }
            let cap = &mut *self.absorb.add(i);
            if *cap <= 0.0 {
                return;
            }
            let hd = &mut *self.hide.add(i);
            let take = (*cap).min(0.8 * (v * OIL_SHARE * hd[4].max(0.0)));
            if take > 0.0 {
                *cap -= take;
                let film_oil = (t * OIL_SHARE * hd[4].max(0.0)).max(1e-9);
                let lean = (take / film_oil).min(0.9);
                hd[4] *= 1.0 - lean;
                hd[1] = (hd[1] * (1.0 + lean) * (1.0 + lean)).min(1.0);
                *vol = (t - take).max(0.0);
            }
        }
    }
}
