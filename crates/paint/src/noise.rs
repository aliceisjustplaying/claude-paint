//! Fractal noise in canvas units.

use noise::{MultiFractal, NoiseFn, Perlin};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Fractal (fBm) Perlin noise. `Copy`: a field can go into any number of
/// `move` closures (a mask and a color field both using the same noise)
/// without clones or `let n = &n;`. The octave tables behind it are built
/// once per distinct (seed, octaves, persistence) and shared for the rest
/// of the program (about 1 KB per octave), so making the same noise twice
/// costs nothing, and making thousands of different ones costs memory.
#[derive(Clone, Copy)]
pub struct Fbm {
    f: &'static noise::Fbm<Perlin>,
    inv_period: f64,
}

type Key = (u32, usize, u64);

/// The shared octave tables for a key (built on first use).
fn tables(seed: u32, octaves: usize, persistence: f64) -> &'static noise::Fbm<Perlin> {
    static CACHE: OnceLock<Mutex<HashMap<Key, &'static noise::Fbm<Perlin>>>> = OnceLock::new();
    let mut m = CACHE.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner());
    m.entry((seed, octaves, persistence.to_bits())).or_insert_with(|| {
        let f = noise::Fbm::<Perlin>::new(seed).set_octaves(octaves).set_persistence(persistence).set_lacunarity(2.0);
        Box::leak(Box::new(f))
    })
}

impl Fbm {
    /// `period` is the size (in units) of the largest feature.
    pub fn new(seed: u32, octaves: usize, period: f32) -> Self {
        Fbm { f: tables(seed, octaves, 0.5), inv_period: 1.0 / period as f64 }
    }

    pub fn with_persistence(mut self, p: f64) -> Self {
        self.f = tables(noise::Seedable::seed(self.f), self.f.octaves, p);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Copy and shared, with the same values as the noise crate's own Fbm
    /// built the way `Fbm::new` always built it (output unchanged).
    #[test]
    fn fbm_is_copy_and_unchanged() {
        let n = Fbm::new(7, 4, 120.0).with_persistence(0.6);
        let a = move |x: f32| n.get(x, 3.0);
        let b = move |x: f32| n.get01(x, 3.0);
        let _ = (a(1.0), b(1.0));
        let own = noise::Fbm::<Perlin>::new(7).set_octaves(4).set_persistence(0.5).set_lacunarity(2.0).set_persistence(0.6);
        for i in 0..50 {
            let (x, y) = (i as f32 * 13.7, i as f32 * 5.3 - 40.0);
            assert_eq!(n.get(x, y), own.get([x as f64 * (1.0 / 120.0), y as f64 * (1.0 / 120.0)]) as f32);
        }
        // the same key shares its tables
        assert!(std::ptr::eq(Fbm::new(7, 4, 1.0).f, Fbm::new(7, 4, 99.0).f));
        assert!(!std::ptr::eq(Fbm::new(7, 4, 1.0).f, Fbm::new(8, 4, 1.0).f));
    }

    /// Field objects a painter shares between closures are `Copy`
    /// (winter #13, coast #7, mountains #6): a noise, a `per_column` profile
    /// and a `move` closure built from them.
    #[test]
    fn fields_are_copy() {
        fn copy<T: Copy + Sync>(_: &T) {}
        let f = crate::canvas::Frame::new(200, 100, 0.2);
        let n = Fbm::new(3, 3, 200.0);
        let ridge = f.per_column(move |x| 300.0 + 40.0 * n.get(x, 0.0));
        let mask = move |x: f32, y: f32| if y > ridge(x) { 1.0 } else { 0.0 };
        let color = move |x: f32, y: f32| (y - ridge(x)) * 0.01 + n.get(x, y);
        copy(&n);
        copy(&ridge);
        copy(&mask);
        copy(&color);
        let m = crate::Mask::from_fn(f, mask);
        assert!(m.data.iter().any(|&v| v == 1.0) && m.data.iter().any(|&v| v == 0.0));
        // exact at column centers and off them
        assert_eq!(ridge(2.5), 300.0 + 40.0 * n.get(2.5, 0.0));
        assert_eq!(ridge(3.1), 300.0 + 40.0 * n.get(3.1, 0.0));
        let _ = color(1.0, 1.0);
    }
}
