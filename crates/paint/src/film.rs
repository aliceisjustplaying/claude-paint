//! The wet film as brushes work it: a raw view of the canvas's wet layer
//! (`Surf`), and one stroke's hold on it (`Stroke`), through which every
//! change a brush makes to the film goes (notes/wet.md).
//!
//! A pixel's film is its body and the surface film over it (`wet::Wet`).
//! While a stroke is painted a third layer can lie between them: a surface
//! film an earlier stroke left, set aside when this stroke first lays paint
//! on it (`Stroke::lay`). The stroke holds that film (its volume, pigment
//! and properties) and settles it when it ends (`Drop`): nothing is set
//! aside between strokes, so the canvas stores none of it.

use crate::canvas::Canvas;
use crate::mask::Mask;
use crate::smoothstep;
use crate::wet::{Latent, Layer, Prop, mix_into};

/// Pixel bounds (x0, y0, x1, y1), end-exclusive, collected by a stroke.
pub(crate) type Bounds = Option<(usize, usize, usize, usize)>;
/// A pixel rectangle (x0, y0, x1, y1), end-exclusive.
pub(crate) type Rect = (usize, usize, usize, usize);

pub(crate) fn grow(b: &mut Bounds, x0: usize, y0: usize, x1: usize, y1: usize) {
    *b = Some(match *b {
        None => (x0, y0, x1, y1),
        Some((a, c, d, e)) => (a.min(x0), c.min(y0), d.max(x1), e.max(y1)),
    });
}

/// A surface film this thin (coats) is stirred twice as readily as a thick
/// one: sheared, it smears into the paint under it (see `Stroke::stir`).
const THIN_FILM: f32 = 0.15;

/// Thickness (coats) of a new stroke's paint that buries the surface film
/// under it in the body (about what hides it; see `Stroke::settle`).
const BURY: f32 = 0.6;

/// Wet film (coats) below which a pixel counts as dry for new paint: what
/// lands there becomes the film's body rather than lying on its surface.
pub(crate) const WET_FILM: f32 = 1e-3;

/// No film set aside at a pixel (`Slots`).
const NONE: u32 = u32::MAX;

/// Per pixel, where the stroke painting it keeps the film it set aside
/// there (an index into its `Stroke::aside`), or `NONE`. Scratch for the
/// strokes under way: every entry is `NONE` between strokes, and a copy of
/// the canvas (an undo snapshot) or a checkpoint carries none of it.
#[derive(Default)]
pub(crate) struct Slots(Vec<u32>);

impl Clone for Slots {
    fn clone(&self) -> Self {
        Slots::default()
    }
}

impl Slots {
    /// Nothing is set aside (debug builds check it between strokes: when
    /// time passes and when a checkpoint is written).
    pub(crate) fn settled(&self) -> bool {
        self.0.iter().all(|&s| s == NONE)
    }
}

/// Raw view of the wet layer and surface, so brushes working disjoint parts
/// of the canvas can run in parallel (see `Canvas::work`). The film itself
/// (`vol`, body, surface film, set-aside slots) changes only through a
/// `Stroke`.
#[derive(Clone, Copy)]
pub(crate) struct Surf {
    /// The buffers' pixels: a window (origin `ox`, `oy`, size `w` × `h`) of
    /// the whole `fw` × `fh` canvas. Brush geometry works in whole-canvas
    /// pixels; only buffer indices are translated.
    pub(crate) w: usize,
    pub(crate) h: usize,
    pub(crate) ox: usize,
    pub(crate) oy: usize,
    pub(crate) fw: usize,
    pub(crate) fh: usize,
    pub(crate) scale: f32,
    vol: *mut f32,
    lat: *mut Latent,
    hide: *mut Prop,
    top: *mut Layer,
    slot: *mut u32,
    stroke: *mut u32,
    touched: *mut u32,
    floor: *mut f32,
    cover: *mut f32,
    base: *const f32,
    /// Drying state (null until the canvas has waited): how open or tacky
    /// the paint under a bristle is (see `drying::feel`).
    dry: *const crate::drying::Px,
}
// SAFETY: callers only run brushes concurrently on pixel sets that cannot
// overlap (tiles separated by more than the largest stroke extent).
unsafe impl Send for Surf {}
unsafe impl Sync for Surf {}

/// Pixel accessors. SAFETY (all): `i` is a buffer index inside the view,
/// and no other thread works that pixel.
impl Surf {
    /// Wet film at pixel `i` (coats, all its layers).
    #[inline]
    pub(crate) unsafe fn vol(&self, i: usize) -> f32 {
        unsafe { *self.vol.add(i) }
    }
    /// The surface relief a bristle feels at pixel `i` (see `Canvas::surf`).
    #[inline]
    pub(crate) unsafe fn base(&self, i: usize) -> f32 {
        unsafe { *self.base.add(i) }
    }
    /// Drying state at pixel `i`, once the canvas has waited.
    #[inline]
    pub(crate) unsafe fn dry(&self, i: usize) -> Option<crate::drying::Px> {
        unsafe { if self.dry.is_null() { None } else { Some(*self.dry.add(i)) } }
    }
    /// The stroke that last laid paint at pixel `i`.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn stroke(&self, i: usize) -> &mut u32 {
        unsafe { &mut *self.stroke.add(i) }
    }
    /// The stroke that last touched pixel `i`, and the film floor it may
    /// not lift below there.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn touched(&self, i: usize) -> (&mut u32, &mut f32) {
        unsafe { (&mut *self.touched.add(i), &mut *self.floor.add(i)) }
    }
    /// The share of pixel `i` its wet paint covers.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn cover(&self, i: usize) -> &mut f32 {
        unsafe { &mut *self.cover.add(i) }
    }
}

impl Canvas {
    pub(crate) fn surf(&mut self) -> Surf {
        // the raw views below index 0..w*h: every buffer must be that long
        let n = self.f.w * self.f.h;
        let wt = &self.wet;
        assert!(
            [self.height.len(), self.px.len(), self.film.len(), wt.vol.len(), wt.lat.len(), wt.hide.len(), wt.top.len(), wt.stroke.len(), wt.touched.len(), wt.floor.len(), wt.cover.len()].iter().all(|&l| l == n),
            "canvas buffers out of sync with frame"
        );
        if self.aside.0.len() != n {
            self.aside.0 = vec![NONE; n];
        }
        if self.base.as_ref().map(|b| b.0) != Some(self.surf_gen) {
            let mut base = self.base.take().map(|b| b.1).unwrap_or_default();
            base.resize(self.height.len(), 0.0);
            use rayon::prelude::*;
            // a brush rests on local peaks: only relief relative to the
            // surroundings (within ~1.5 mm) matters. ±TOOTH_UM of relief spans
            // the whole contact range.
            let r = ((1.5 / self.px_mm()).round() as usize).max(1);
            let (w, h) = (self.f.w, self.f.h);
            let low = crate::bristle::contact_level(&self.height, w, h, r);
            base.par_iter_mut().zip(self.height.par_iter().zip(low.par_iter())).for_each(|(b, (&hgt, &lo))| {
                *b = (0.5 + (hgt - lo) / (2.0 * crate::bristle::TOOTH_UM)).clamp(-0.2, 1.3);
            });
            self.base = Some((self.surf_gen, base));
        }
        Surf {
            w: self.f.w,
            h: self.f.h,
            ox: self.f.x0,
            oy: self.f.y0,
            fw: self.f.full_w,
            fh: self.f.full_h,
            scale: self.f.scale,
            vol: self.wet.vol.as_mut_ptr(),
            lat: self.wet.lat.as_mut_ptr(),
            hide: self.wet.hide.as_mut_ptr(),
            top: self.wet.top.as_mut_ptr(),
            slot: self.aside.0.as_mut_ptr(),
            stroke: self.wet.stroke.as_mut_ptr(),
            touched: self.wet.touched.as_mut_ptr(),
            floor: self.wet.floor.as_mut_ptr(),
            cover: self.wet.cover.as_mut_ptr(),
            base: self.base.as_ref().unwrap().1.as_ptr(),
            dry: if self.wet.clock.px.len() == n { self.wet.clock.px.as_ptr() } else { std::ptr::null() },
        }
    }
}

/// An earlier stroke's surface film set aside at pixel `i` while a new
/// stroke lays paint over it (what is left of it: pickup and the plough
/// take from it).
struct Aside {
    i: usize,
    film: Layer,
}

/// Paint lifted or pushed off one pixel, by layer: from its surface film,
/// from a film set aside under the stroke and from its body.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Parcel {
    pub(crate) top: Layer,
    pub(crate) aside: Layer,
    pub(crate) body: Layer,
}

impl Parcel {
    const EMPTY: Parcel = Parcel { top: Layer::EMPTY, aside: Layer::EMPTY, body: Layer::EMPTY };

    /// Its parts, surface first.
    pub(crate) fn parts(&self) -> [&Layer; 3] {
        [&self.top, &self.aside, &self.body]
    }
}

/// One stroke (a drag or a touch) working the wet film: its id, mask,
/// footprint and scratch, the bounds it dirties, and the films it has set
/// aside. Every change a brush makes to the film goes through it. Dropping
/// it ends the stroke: it settles what it set aside (`settle`), so no
/// stroke path can leave paint in flight.
pub(crate) struct Stroke<'a> {
    pub(crate) sf: Surf,
    pub(crate) id: u32,
    pub(crate) clip: Option<&'a Mask>,
    /// The stroke's footprint (whole-canvas pixels): it never works outside.
    pub(crate) lim: Rect,
    /// Contact weights, reused by every bristle.
    pub(crate) wts: &'a mut Vec<f32>,
    /// Pixels dirtied (whole-canvas).
    pub(crate) bounds: Bounds,
    aside: Vec<Aside>,
    /// How often pickup and the plough took paint set aside under this
    /// stroke (tests check that the conservation scene exercises it).
    pub(crate) through: [u32; 2],
}

impl<'a> Stroke<'a> {
    /// SAFETY: no other thread may work pixels in `lim` until the stroke
    /// is dropped.
    pub(crate) unsafe fn begin(sf: Surf, id: u32, clip: Option<&'a Mask>, lim: Rect, wts: &'a mut Vec<f32>) -> Self {
        Stroke { sf, id, clip, lim, wts, bounds: None, aside: Vec::new(), through: [0; 2] }
    }

    /// End the stroke: settle it and return the pixels it dirtied (buffer
    /// pixels) and how often it took set-aside paint.
    pub(crate) fn finish(mut self) -> (Bounds, [u32; 2]) {
        // SAFETY: `begin`'s contract holds until the stroke is dropped.
        unsafe { self.settle() };
        let sf = self.sf;
        (self.bounds.map(|(x0, y0, x1, y1)| (x0 - sf.ox, y0 - sf.oy, x1 - sf.ox, y1 - sf.oy)), self.through)
    }

    #[inline]
    unsafe fn aside_at(&mut self, i: usize) -> Option<&mut Layer> {
        unsafe {
            let s = *self.sf.slot.add(i);
            if s == NONE { None } else { Some(&mut self.aside[s as usize].film) }
        }
    }

    /// Set `film` aside at pixel `i` as `lay` does (tests; its coats must
    /// already be part of the pixel's `vol`).
    #[cfg(test)]
    pub(crate) unsafe fn set_aside(&mut self, i: usize, film: Layer) {
        unsafe {
            *self.sf.slot.add(i) = self.aside.len() as u32;
        }
        self.aside.push(Aside { i, film });
    }

    /// Coats set aside at pixel `i` (0 where none is).
    #[inline]
    unsafe fn aside_v(&self, i: usize) -> f32 {
        unsafe {
            let s = *self.sf.slot.add(i);
            if s == NONE { 0.0 } else { self.aside[s as usize].film.v }
        }
    }

    /// Coats of pixel `i`'s surface film and of the film set aside under
    /// this stroke there: the paint over the body.
    #[inline]
    pub(crate) unsafe fn over_body(&self, i: usize) -> f32 {
        unsafe { (*self.sf.top.add(i)).v + self.aside_v(i) }
    }

    /// Volume of pixel `i`'s body: its film less the surface film and any
    /// earlier surface film set aside by this stroke (`lay`).
    #[inline]
    unsafe fn body(&self, i: usize) -> f32 {
        unsafe { (*self.sf.vol.add(i) - (*self.sf.top.add(i)).v - self.aside_v(i)).max(0.0) }
    }

    /// Stiffness of pixel `i`'s whole film: its body, its surface film and
    /// what this stroke set aside there, mixed by volume.
    #[inline]
    pub(crate) unsafe fn stiffness(&self, i: usize) -> f32 {
        unsafe {
            let (hide, top) = (*self.sf.hide.add(i), &*self.sf.top.add(i));
            let s = *self.sf.slot.add(i);
            let m = if s == NONE { None } else { Some(&self.aside[s as usize].film).filter(|m| m.v > 0.0) };
            let Some(m) = m else {
                // (without a film set aside: the body and surface film)
                return crate::wet::whole(*self.sf.vol.add(i), hide, top)[1];
            };
            let (b, t) = (self.body(i), top.v);
            (b * hide[1] + t * top.hide[1] + m.v * m.hide[1]) / (b + t + m.v)
        }
    }

    /// Lay `v` coats of paint (`lat`, `hide`) on pixel `i`. On a dry
    /// surface it becomes the film's body; laid into wet paint it goes on
    /// the surface film, on top of what is there (a brush works it in by
    /// stirring, see `stir`).
    #[inline]
    pub(crate) unsafe fn add(&mut self, i: usize, v: f32, lat: &Latent, hide: Prop) {
        unsafe {
            if v <= 0.0 {
                return;
            }
            let sf = self.sf;
            if *sf.vol.add(i) < WET_FILM {
                // (a trace of paint on a dry pixel is part of the body)
                let mut b = self.body(i);
                let (bl, bh) = (&mut *sf.lat.add(i), &mut *sf.hide.add(i));
                let t = &mut *sf.top.add(i);
                if t.v > 0.0 {
                    mix_into(&mut b, bl, bh, t.v, &t.lat, t.hide);
                    t.v = 0.0;
                }
                if let Some(m) = self.aside_at(i)
                    && m.v > 0.0
                {
                    mix_into(&mut b, bl, bh, m.v, &m.lat, m.hide);
                    m.v = 0.0;
                }
                mix_into(&mut b, bl, bh, v, lat, hide);
            } else {
                (*sf.top.add(i)).mix(v, lat, hide);
            }
            *sf.vol.add(i) += v;
        }
    }

    /// Lay `v` coats from this stroke's brush on pixel `i` (`add`). The
    /// surface film is the newest stroke's paint: a surface film left by an
    /// earlier stroke is set aside when this stroke first lays paint here;
    /// until the stroke ends it lies between the body and this stroke's
    /// paint, and `settle` then decides whether it is buried under this
    /// stroke's paint or shows through it.
    #[inline]
    pub(crate) unsafe fn lay(&mut self, i: usize, v: f32, lat: &Latent, hide: Prop) {
        unsafe {
            let sf = self.sf;
            let t = &mut *sf.top.add(i);
            if v > 0.0 && *sf.stroke.add(i) != self.id && t.v > 0.0 && *sf.vol.add(i) >= WET_FILM && self.aside_v(i) <= 0.0 {
                // (a pixel's paint is set aside once a stroke: it lays here
                // and is the pixel's last stroke from then on)
                debug_assert_eq!(*sf.slot.add(i), NONE, "a film set aside twice by one stroke");
                *sf.slot.add(i) = self.aside.len() as u32;
                self.aside.push(Aside { i, film: *t });
                t.v = 0.0;
            }
            self.add(i, v, lat, hide);
        }
    }

    /// At the stroke's end: each surface film it set aside (`lay`), as much
    /// of it as is left after the stroke's pickup and plough, is buried in
    /// the body as far as the stroke's own paint over it hides it (`BURY`
    /// coats hide it all), and the rest stays in the surface film, mixed
    /// with this stroke's paint: a thin edge over an earlier light stroke
    /// doesn't bring the dark body up through it. A film like the body it
    /// lies on is always buried (that changes nothing but frees the
    /// surface for the new paint).
    unsafe fn settle(&mut self) {
        unsafe {
            let sf = self.sf;
            for a in std::mem::take(&mut self.aside) {
                let i = a.i;
                let mut b = (*sf.vol.add(i) - (*sf.top.add(i)).v - a.film.v).max(0.0);
                *sf.slot.add(i) = NONE;
                let (mv, m) = (a.film.v, &a.film);
                if mv <= 0.0 {
                    continue;
                }
                let t = &mut *sf.top.add(i);
                let l = &mut *sf.lat.add(i);
                let like = 1.0 - smoothstep(0.05, 0.3, l.iter().zip(&m.lat).map(|(a, c)| (a - c).abs()).sum::<f32>());
                let k = if b > 1e-6 { smoothstep(0.0, BURY, t.v).max(like) } else { smoothstep(0.0, BURY, t.v) };
                mix_into(&mut b, l, &mut *sf.hide.add(i), mv * k, &m.lat, m.hide);
                t.mix(mv * (1.0 - k), &m.lat, m.hide);
            }
        }
    }

    /// Work the share `k` (0..1) of pixel `i`'s surface film into its body,
    /// for surface paint of middling stiffness: stiff paint holds its place
    /// (it has a yield stress), fluid paint gives way (up to 1.5× apart).
    #[inline]
    pub(crate) unsafe fn stir(&mut self, i: usize, k: f32) {
        unsafe {
            let sf = self.sf;
            let t = *sf.top.add(i);
            if t.v <= 0.0 || k <= 0.0 {
                return;
            }
            // (a film much thinner than a coat is no layer of its own once
            // it is sheared: it smears into the wet paint it lies on)
            let k = k * (1.25 - 0.5 * t.hide[1].clamp(0.0, 1.0)) * (1.0 + THIN_FILM / t.v.max(1e-3));
            let m = t.v * k.min(1.0);
            let mut b = self.body(i);
            mix_into(&mut b, &mut *sf.lat.add(i), &mut *sf.hide.add(i), m, &t.lat, t.hide);
            (*sf.top.add(i)).v -= m;
        }
    }

    /// Mix `v` coats of paint into the body of pixel `i`'s film (under its
    /// surface film).
    #[inline]
    pub(crate) unsafe fn add_body(&mut self, i: usize, v: f32, lat: &Latent, hide: Prop) {
        unsafe {
            if v <= 0.0 {
                return;
            }
            let sf = self.sf;
            let mut b = self.body(i);
            mix_into(&mut b, &mut *sf.lat.add(i), &mut *sf.hide.add(i), v, lat, hide);
            *sf.vol.add(i) += v;
        }
    }

    /// Land a parcel pushed off another pixel on pixel `i`, each part where
    /// it belongs: body to body, the surface film and a set-aside film on
    /// the surface.
    #[inline]
    pub(crate) unsafe fn land(&mut self, i: usize, p: &Parcel) {
        unsafe {
            self.add_body(i, p.body.v, &p.body.lat, p.body.hide);
            self.add(i, p.top.v, &p.top.lat, p.top.hide);
            self.add(i, p.aside.v, &p.aside.lat, p.aside.hide);
        }
    }

    /// The parcel of paint at pixel `i` whose layers hold `v` (surface),
    /// `m` (set aside) and `b` (body) coats.
    #[inline]
    unsafe fn parcel(&mut self, i: usize, v: [f32; 3]) -> Parcel {
        unsafe {
            let sf = self.sf;
            let t = &*sf.top.add(i);
            let aside = match self.aside_at(i) {
                Some(m) if v[1] > 0.0 => m.of(v[1]),
                _ => Layer::EMPTY.of(v[1]),
            };
            Parcel { top: t.of(v[0]), aside, body: Layer::new(v[2], *sf.lat.add(i), *sf.hide.add(i)) }
        }
    }

    /// Take `v` coats of pixel `i`'s whole film, its layers alike (what a
    /// bristle pushes aside).
    #[inline]
    pub(crate) unsafe fn take_column(&mut self, i: usize, v: f32) -> Parcel {
        unsafe {
            let sf = self.sf;
            let vol = *sf.vol.add(i);
            if vol <= 0.0 {
                return Parcel::EMPTY;
            }
            let f = (v / vol).clamp(0.0, 1.0);
            let parts = [(*sf.top.add(i)).v * f, self.aside_v(i) * f, self.body(i) * f];
            (*sf.top.add(i)).v -= parts[0];
            if let Some(m) = self.aside_at(i) {
                m.v -= parts[1];
            }
            *sf.vol.add(i) = vol - v.min(vol).max(0.0);
            if parts[1] > 0.0 {
                self.through[1] += 1;
            }
            self.parcel(i, parts)
        }
    }

    /// Take `v` coats off the top of pixel `i`: the surface film first, then
    /// the film set aside under it, then the body (what a bristle lifts).
    #[inline]
    pub(crate) unsafe fn take(&mut self, i: usize, v: f32) -> Parcel {
        unsafe {
            let sf = self.sf;
            let vol = *sf.vol.add(i);
            let v = v.min(vol).max(0.0);
            let t = &mut *sf.top.add(i);
            let a = v.min(t.v);
            t.v -= a;
            let am = match self.aside_at(i) {
                Some(m) => {
                    let am = (v - a).min(m.v);
                    m.v -= am;
                    am
                }
                None => (v - a).min(0.0),
            };
            *sf.vol.add(i) = vol - v;
            if am > 0.0 {
                self.through[0] += 1;
            }
            self.parcel(i, [a, am, v - a - am])
        }
    }
}

impl Drop for Stroke<'_> {
    fn drop(&mut self) {
        // SAFETY: `begin`'s contract holds until the stroke is dropped.
        unsafe { self.settle() };
    }
}
