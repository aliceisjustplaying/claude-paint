//! Wet paint sitting on top of the dry picture.
//!
//! Every pixel holds a volume of wet paint (thickness, in "layer units": 1.0
//! is one normal coat), its pigment mixture as a Mixbox latent vector (mixing
//! is linear in latent space, weighted by volume) and its hiding power
//! (0 = pure glaze, 1 = fully opaque body color). Brushes exchange paint with
//! this layer. `Canvas::dry` bakes it into the dry picture with Kubelka–Munk.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::pigment::Pigment;
use rayon::prelude::*;

pub const LAT: usize = mixbox::LATENT_SIZE;
pub type Latent = [f32; LAT];

/// A paint as squeezed from the tube and thinned with medium.
#[derive(Clone, Copy, Debug)]
pub struct Paint {
    pub color: Rgb,
    /// Hiding power at unit thickness: 0.05 = glaze, 0.5 = scumble, 0.92 = body.
    pub hiding: f32,
    /// Body: how much paint a dip puts on the brush (1 = stiff tube paint,
    /// 0.3 = a glaze thinned with lots of medium).
    pub body: f32,
}

impl Paint {
    pub fn body(color: Rgb) -> Self {
        Paint { color, hiding: 0.92, body: 1.0 }
    }
    pub fn scumble(color: Rgb) -> Self {
        Paint { color, hiding: 0.5, body: 0.6 }
    }
    pub fn glaze(color: Rgb) -> Self {
        Paint { color, hiding: 0.07, body: 0.3 }
    }
    pub fn latent(&self) -> Latent {
        mixbox::linear_float_rgb_to_latent(&self.color)
    }
}

pub struct Wet {
    pub vol: Vec<f32>,
    pub lat: Vec<Latent>,
    pub hide: Vec<f32>,
    /// Which stroke last laid paint here (a stroke barely re-picks its own paint).
    pub stroke: Vec<u32>,
    /// Stroke that last touched a pixel, and the film floor that stroke may
    /// not lift below (one pass lifts only part of the film).
    pub touched: Vec<u32>,
    pub floor: Vec<f32>,
    /// Id of the stroke being painted.
    pub current: u32,
    /// Dirty bounding box in pixels (x0, y0, x1, y1), if any paint is wet.
    pub dirty: Option<(usize, usize, usize, usize)>,
}

impl Wet {
    pub fn new(n: usize) -> Self {
        Wet { vol: vec![0.0; n], lat: vec![[0.0; LAT]; n], hide: vec![0.0; n], stroke: vec![0; n], touched: vec![0; n], floor: vec![0.0; n], current: 0, dirty: None }
    }

    pub fn touch(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        self.dirty = Some(match self.dirty {
            None => (x0, y0, x1, y1),
            Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
        });
    }

    /// Add paint to pixel `i`, mixing by volume.
    #[inline]
    pub fn add(&mut self, i: usize, v: f32, lat: &Latent, hide: f32) {
        if v <= 0.0 {
            return;
        }
        let t = self.vol[i] + v;
        let a = v / t;
        let l = &mut self.lat[i];
        for k in 0..LAT {
            l[k] += (lat[k] - l[k]) * a;
        }
        self.hide[i] += (hide - self.hide[i]) * a;
        self.vol[i] = t;
    }
}

/// Mix `v` of (`lat`, `hide`) into a reservoir (`rv`, `rl`, `rh`).
#[inline]
pub fn mix_into(rv: &mut f32, rl: &mut Latent, rh: &mut f32, v: f32, lat: &Latent, hide: f32) {
    if v <= 0.0 {
        return;
    }
    let t = *rv + v;
    let a = v / t;
    for k in 0..LAT {
        rl[k] += (lat[k] - rl[k]) * a;
    }
    *rh += (hide - *rh) * a;
    *rv = t;
}

/// Relief added per unit of dried paint thickness.
pub const RELIEF_PER_VOL: f32 = 0.12;

impl Canvas {
    /// Let the wet paint dry: composite it over the dry picture (Kubelka–Munk,
    /// thickness = volume), add its thickness to the relief, clear the wet layer.
    pub fn dry(&mut self) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty.take() else { return };
        let w = self.f.w;
        let (x1, y1) = (x1.min(w), y1.min(self.f.h));
        let wet = &mut self.wet;
        let rows = self.px[y0 * w..y1 * w]
            .par_chunks_mut(w)
            .zip(self.height[y0 * w..y1 * w].par_chunks_mut(w))
            .zip(self.film[y0 * w..y1 * w].par_chunks_mut(w))
            .zip(wet.vol[y0 * w..y1 * w].par_chunks_mut(w))
            .zip(wet.lat[y0 * w..y1 * w].par_chunks(w))
            .zip(wet.hide[y0 * w..y1 * w].par_chunks(w));
        rows.for_each(|(((((px, hh), ff), vv), ll), hd)| {
            for x in x0..x1 {
                let v = vv[x];
                if v < 1e-5 {
                    vv[x] = 0.0;
                    continue;
                }
                let c = mixbox::latent_to_linear_float_rgb(&ll[x]);
                let pig = Pigment::with_hiding(c, hd[x].clamp(0.01, 0.99));
                px[x] = pig.over(px[x], v);
                hh[x] += v * RELIEF_PER_VOL;
                ff[x] += v;
                vv[x] = 0.0;
            }
        });
    }

    /// Total wet paint on the canvas (for tests / debugging).
    pub fn wet_total(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum::<f64>() / (self.f.scale as f64 * self.f.scale as f64)
    }
}
