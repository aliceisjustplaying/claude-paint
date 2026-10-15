//! Engine 7's per-pixel paint add (lane STIL-69).
//!
//! Contract (frozen, CONTRACTS-e7.md): the kernel takes `let film = Film::of(&sf)`
//! once per exchange call and `Paint7::new(sf.lean, &lat, hide, cure)` once per
//! paint source (per call for a deposit, per source pixel for the plough);
//! `film.add(i, v, &paint)` then lays `v` coats at buffer pixel `i` with the
//! meaning of `Surf::add` (mixed by volume into color, hiding and cure;
//! engine 4's lean film and absorbent ground).
//!
//! Stub: engine 5's `Surf::add`.

use crate::bristle::Surf;
use crate::wet::{Latent, Prop};

/// The canvas's film, as the add sees it.
#[derive(Clone, Copy)]
pub(crate) struct Film {
    sf: Surf,
}

/// Paint about to be laid: one source's color, hiding and cure.
pub(crate) struct Paint7 {
    lat: Latent,
    hide: Prop,
    cure: f32,
}

impl Paint7 {
    pub(crate) fn new(lean: bool, lat: &Latent, hide: Prop, cure: f32) -> Self {
        let _ = lean;
        Paint7 { lat: *lat, hide, cure }
    }
}

impl Film {
    pub(crate) fn of(sf: &Surf) -> Self {
        Film { sf: *sf }
    }

    /// SAFETY: as `Surf::add`: `i` is in the buffer and no other thread
    /// touches pixel `i`.
    #[inline]
    pub(crate) unsafe fn add(&self, i: usize, v: f32, paint: &Paint7) {
        unsafe { self.sf.add(i, v, &paint.lat, paint.hide, paint.cure) }
    }
}
