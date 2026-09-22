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
use crate::surface::COAT_UM;
use rayon::prelude::*;

pub const LAT: usize = mixbox::LATENT_SIZE;
pub type Latent = [f32; LAT];
/// Paint properties mixed by volume alongside the pigment: [hiding, stiffness].
/// Stiffness 0 = fluid, medium-rich glaze; 1 = stiff tube paint.
pub type Prop = [f32; 2];

#[inline]
fn lerp_prop(p: &mut Prop, q: Prop, a: f32) {
    p[0] += (q[0] - p[0]) * a;
    p[1] += (q[1] - p[1]) * a;
}

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
    /// [hiding, stiffness] of the wet paint.
    pub hide: Vec<Prop>,
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
        Wet { vol: vec![0.0; n], lat: vec![[0.0; LAT]; n], hide: vec![[0.0, 0.5]; n], stroke: vec![0; n], touched: vec![0; n], floor: vec![0.0; n], current: 0, dirty: None }
    }

    pub fn touch(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        self.dirty = Some(match self.dirty {
            None => (x0, y0, x1, y1),
            Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
        });
    }

    /// Add paint to pixel `i`, mixing by volume.
    #[inline]
    pub fn add(&mut self, i: usize, v: f32, lat: &Latent, hide: Prop) {
        if v <= 0.0 {
            return;
        }
        let t = self.vol[i] + v;
        let a = v / t;
        let l = &mut self.lat[i];
        for k in 0..LAT {
            l[k] += (lat[k] - l[k]) * a;
        }
        lerp_prop(&mut self.hide[i], hide, a);
        self.vol[i] = t;
    }
}

/// Mix `v` of (`lat`, `hide`) into a reservoir (`rv`, `rl`, `rh`).
#[inline]
pub fn mix_into(rv: &mut f32, rl: &mut Latent, rh: &mut Prop, v: f32, lat: &Latent, hide: Prop) {
    if v <= 0.0 {
        return;
    }
    let t = *rv + v;
    let a = v / t;
    for k in 0..LAT {
        rl[k] += (lat[k] - rl[k]) * a;
    }
    lerp_prop(rh, hide, a);
    *rv = t;
}


impl Canvas {
    /// Let the wet paint dry: the film levels over the surface (thin fluid
    /// paint pools in the hollows, stiff paint keeps its marks), then it is
    /// composited over the dry picture with Kubelka–Munk using the settled
    /// thickness, and the wet layer is cleared.
    pub fn dry(&mut self) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty.take() else { return };
        let (w, h) = (self.f.w, self.f.h);
        let (x1, y1) = (x1.min(w), y1.min(h));
        let pad = ((2.0 / self.px_mm()).ceil() as usize).max(2);
        let ex = (x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
        let (ew, eh) = (ex.2 - ex.0, ex.3 - ex.1);
        let mut add = vec![0.0f32; ew * eh];
        let mut stiff = vec![0.5f32; ew * eh];
        for y in 0..eh {
            for x in 0..ew {
                let i = (ex.1 + y) * w + ex.0 + x;
                let v = self.wet.vol[i];
                if v >= 1e-5 {
                    add[y * ew + x] = v * COAT_UM;
                    stiff[y * ew + x] = self.wet.hide[i][1];
                }
            }
        }
        let t = self.settle(ex, &add, &stiff);
        let wet = &mut self.wet;
        let (lat, hide) = (&wet.lat, &wet.hide);
        self.px[ex.1 * w..ex.3 * w]
            .par_chunks_mut(w)
            .zip(wet.vol[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(self.film[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .enumerate()
            .for_each(|(j, ((px, vv), ff))| {
                let y = ex.1 + j;
                for x in ex.0..ex.2 {
                    if vv[x] < 1e-5 {
                        vv[x] = 0.0;
                        continue;
                    }
                    let ti = t[j * ew + x - ex.0] / COAT_UM;
                    let i = y * w + x;
                    let c = mixbox::latent_to_linear_float_rgb(&lat[i]);
                    let pig = Pigment::with_hiding(c, hide[i][0].clamp(0.01, 0.99));
                    px[x] = pig.over(px[x], ti);
                    ff[x] += ti;
                    vv[x] = 0.0;
                }
            });
    }

    /// Total wet paint on the canvas (for tests / debugging).
    pub fn wet_total(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum::<f64>() / (self.f.scale as f64 * self.f.scale as f64)
    }
}
