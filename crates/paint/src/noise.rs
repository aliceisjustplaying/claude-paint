//! Fractal noise in canvas units.

use noise::{MultiFractal, NoiseFn, Perlin};

pub struct Fbm {
    f: noise::Fbm<Perlin>,
    inv_period: f64,
}

impl Fbm {
    /// `period` is the size (in units) of the largest feature.
    pub fn new(seed: u32, octaves: usize, period: f32) -> Self {
        let f = noise::Fbm::<Perlin>::new(seed)
            .set_octaves(octaves)
            .set_persistence(0.5)
            .set_lacunarity(2.0);
        Fbm { f, inv_period: 1.0 / period as f64 }
    }

    pub fn with_persistence(mut self, p: f64) -> Self {
        self.f = self.f.set_persistence(p);
        self
    }

    /// Roughly in [-1, 1].
    #[inline]
    pub fn get(&self, x: f32, y: f32) -> f32 {
        self.f.get([x as f64 * self.inv_period, y as f64 * self.inv_period]) as f32
    }

    /// Remapped to [0, 1].
    #[inline]
    pub fn get01(&self, x: f32, y: f32) -> f32 {
        (self.get(x, y) * 0.5 + 0.5).clamp(0.0, 1.0)
    }
}
