//! Engine 7's paint laying: engine 5's `Surf::add` (paint mixed by volume
//! into a pixel's latent color, properties and cure) with everything that
//! depends only on the paint being laid worked out once per source instead
//! of once per pixel, and the 7 latent channels and 10 properties blended
//! as overlapping 4-lane rows (`[0..4]` and `[3..7]`; `[0..4]`, `[3..7]`
//! and `[6..10]`). Every lane runs engine 5's arithmetic in engine 5's
//! order, so a pixel ends bit for bit where engine 5 leaves it.

use crate::bristle::{Surf, fluid_of, ground_draw, ground_drain, restore_buried, skim_top};
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
    /// The drain switch (`Surf::drain`): the ground drains the film as its
    /// pigment packs (`bristle::ground_drain`) instead of drawing oil from
    /// the paint just laid (`bristle::ground_draw`).
    drain: bool,
}

/// Paint ready to lay: the source's color and properties with the solvent
/// that evaporates as it is laid (engine 4's lean film) already taken out,
/// and with the drain switch broken up and fluid (`bristle::fluid_of`), as
/// `Surf::add` lays it.
#[derive(Clone, Copy)]
pub(crate) struct Paint7 {
    lat: [[f32; 4]; 2],
    hide: [[f32; 4]; 3],
    /// Share of the laid volume left once the solvent is gone.
    keep: f32,
    cure: f32,
}

impl Paint7 {
    #[inline(always)]
    pub(crate) fn new(lean: bool, drain: bool, lat: &Latent, hide: Prop, cure: f32) -> Paint7 {
        let (keep, hide) = if lean && hide[3] > 0.0 {
            let mut h = hide;
            h[3] = 0.0;
            (1.0 - hide[3].clamp(0.0, 0.95), h)
        } else {
            (1.0, hide)
        };
        let hide = if drain { fluid_of(hide) } else { hide };
        Paint7 {
            lat: [[lat[0], lat[1], lat[2], lat[3]], [lat[3], lat[4], lat[5], lat[6]]],
            hide: [[hide[0], hide[1], hide[2], hide[3]], [hide[3], hide[4], hide[5], hide[6]], [hide[6], hide[7], hide[8], hide[9]]],
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
        Film { vol: sf.vol, lat: sf.lat, hide: sf.hide, dry: if sf.cure_now { sf.dry } else { std::ptr::null_mut() }, absorb: sf.absorb, drain: sf.drain }
    }

    /// `add`, for paint a brush lays from its load (`Surf::lay`): with
    /// `skim` > 0 it mixes with only the top `skim` coats of the wet paint
    /// at `i`, the rest lying buried under it.
    ///
    /// # Safety
    /// As `add`.
    #[inline(always)]
    pub(crate) unsafe fn lay(&self, i: usize, v: f32, p: &Paint7, skim: f32) {
        if self.drain && v <= 0.0 {
            return;
        }
        unsafe {
            if skim <= 0.0 {
                return self.add(i, v, p);
            }
            let buried = (*self.vol.add(i) - skim).max(0.0);
            let below = (self.drain && buried > 0.0).then(|| skim_top(&mut *self.hide.add(i), *self.vol.add(i), buried));
            *self.vol.add(i) -= buried;
            self.add(i, v, p);
            *self.vol.add(i) += buried;
            if let Some(below) = below {
                restore_buried(&mut *self.hide.add(i), &below, buried / *self.vol.add(i));
            }
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
            // every row is read before any is written: the shared lanes
            // (latent 3, properties 3 and 6) blend once from the old value
            // and are written twice with the same result
            let l = self.lat.add(i).cast::<f32>();
            let (l0, l1) = (l.cast::<[f32; 4]>().read_unaligned(), l.add(3).cast::<[f32; 4]>().read_unaligned());
            l.cast::<[f32; 4]>().write_unaligned(lerp4(l0, p.lat[0], a));
            l.add(3).cast::<[f32; 4]>().write_unaligned(lerp4(l1, p.lat[1], a));
            let h = self.hide.add(i).cast::<f32>();
            let (h0, h1, h2) = (h.cast::<[f32; 4]>().read_unaligned(), h.add(3).cast::<[f32; 4]>().read_unaligned(), h.add(6).cast::<[f32; 4]>().read_unaligned());
            h.cast::<[f32; 4]>().write_unaligned(lerp4(h0, p.hide[0], a));
            h.add(3).cast::<[f32; 4]>().write_unaligned(lerp4(h1, p.hide[1], a));
            h.add(6).cast::<[f32; 4]>().write_unaligned(lerp4(h2, p.hide[2], a));
            if !self.dry.is_null() {
                // (a film too thin for `wait` to count is bare: no cure)
                let d = &mut *self.dry.add(i);
                d.cure = if t < 1e-5 { 0.0 } else { d.cure + (p.cure - d.cure) * a };
            }
            *vol = t;
            if self.absorb.is_null() {
                return;
            }
            // (the ground's draw, or with the drain switch its drain, as
            // `Surf::add` has them)
            let (cap, hd) = (&mut *self.absorb.add(i), &mut *self.hide.add(i));
            *vol = if self.drain { ground_drain(cap, t, hd) } else { ground_draw(cap, v, t, hd) };
        }
    }
}
