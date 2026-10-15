use crate::bristle::Surf;
use crate::drying::Px;
use crate::wet::{Latent, Prop};

pub(crate) struct AddPaint {
    volume: *mut f32,
    latent: *mut Latent,
    properties: *mut Prop,
    dry: *mut Px,
    absorb: *mut f32,
    incoming: [f32; 12],
    retained: f32,
    cure: f32,
}

impl AddPaint {
    #[inline]
    pub(crate) fn new(sf: Surf, latent: &Latent, mut properties: Prop, cure: f32) -> Self {
        let retained = if sf.lean && properties[3] > 0.0 {
            let retained = 1.0 - properties[3].clamp(0.0, 0.95);
            properties[3] = 0.0;
            retained
        } else {
            1.0
        };
        let mut incoming = [0.0; 12];
        incoming[..7].copy_from_slice(latent);
        incoming[7..].copy_from_slice(&properties);
        Self {
            volume: sf.vol,
            latent: sf.lat,
            properties: sf.hide,
            dry: if sf.cure_now { sf.dry } else { std::ptr::null_mut() },
            absorb: sf.absorb,
            incoming,
            retained,
            cure,
        }
    }

    #[inline]
    pub(crate) unsafe fn add(&self, i: usize, amount: f32) {
        if amount <= 0.0 {
            return;
        }
        unsafe {
            let amount = amount * self.retained;
            let volume = &mut *self.volume.add(i);
            let latent = &mut *self.latent.add(i);
            let properties = &mut *self.properties.add(i);
            let total = *volume + amount;
            let share = amount / total;
            let mut components = [0.0; 12];
            components[..7].copy_from_slice(latent);
            components[7..].copy_from_slice(properties);
            for k in 0..12 {
                components[k] += (self.incoming[k] - components[k]) * share;
            }
            latent.copy_from_slice(&components[..7]);
            properties.copy_from_slice(&components[7..]);
            if !self.dry.is_null() {
                let dry = &mut *self.dry.add(i);
                dry.cure = if total < 1e-5 { 0.0 } else { dry.cure + (self.cure - dry.cure) * share };
            }
            *volume = total;
            if !self.absorb.is_null() {
                let absorb = &mut *self.absorb.add(i);
                if *absorb > 0.0 {
                    let oil_in = amount * 0.4 * properties[4].max(0.0);
                    let take = (*absorb).min(0.8 * oil_in);
                    if take > 0.0 {
                        *absorb -= take;
                        let film_oil = (total * 0.4 * properties[4].max(0.0)).max(1e-9);
                        let lean = (take / film_oil).min(0.9);
                        properties[4] *= 1.0 - lean;
                        properties[1] = (properties[1] * (1.0 + lean) * (1.0 + lean)).min(1.0);
                        *volume = (total - take).max(0.0);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use std::time::Instant;

    struct Pixel {
        volume: f32,
        latent: Latent,
        properties: Prop,
        dry: Px,
        absorb: f32,
    }

    impl Pixel {
        fn new() -> Self {
            Self {
                volume: 0.7,
                latent: [0.2, 0.1, 0.3, -0.1, 0.3, 0.2, 0.1],
                properties: [0.4, 0.6, 1.3, 0.0, 0.8],
                dry: Px { cure: 0.3, lev: 0.0, seen: 0.7, sub: 1.0, srate: 1.0, thick: 0.7 },
                absorb: 0.0,
            }
        }

        fn surf(&mut self, lean: bool, dry: bool, absorb: bool) -> Surf {
            Surf {
                engine: 7, w: 1, h: 1, ox: 0, oy: 0, fw: 1, fh: 1, scale: 1.0,
                vol: &mut self.volume, lat: &mut self.latent, hide: &mut self.properties,
                stroke: std::ptr::null_mut(), touched: std::ptr::null_mut(),
                floor: std::ptr::null_mut(), cover: std::ptr::null_mut(),
                base: std::ptr::null(), dry: if dry { &mut self.dry } else { std::ptr::null_mut() },
                cure_now: true, clump: true, lean,
                absorb: if absorb { &mut self.absorb } else { std::ptr::null_mut() },
                solv: std::ptr::null_mut(), laid: std::ptr::null_mut(), laid_id: std::ptr::null_mut(),
            }
        }
    }

    #[test]
    fn prepared_add_matches_legacy_material_effects() {
        let source = [0.7, -0.2, 0.4, 0.1, 0.3, 0.2, -0.1];
        for amount in [0.0, -0.1, 1e-10, 0.01, 0.7, 2.0] {
            for thinner in [0.0, 0.3, 0.99] {
                for lean in [false, true] {
                    for dry in [false, true] {
                        for absorb in [false, true] {
                            let mut legacy = Pixel::new();
                            let mut prepared = Pixel::new();
                            legacy.absorb = 0.02;
                            prepared.absorb = legacy.absorb;
                            let properties = [0.2, 0.8, 1.7, thinner, 0.9];
                            let reference = legacy.surf(lean, dry, absorb);
                            let next = AddPaint::new(prepared.surf(lean, dry, absorb), &source, properties, 0.1);
                            unsafe {
                                reference.add(0, amount, &source, properties, 0.1);
                                next.add(0, amount);
                            }
                            assert_eq!(legacy.volume, prepared.volume);
                            assert_eq!(legacy.latent, prepared.latent);
                            assert_eq!(legacy.properties, prepared.properties);
                            assert_eq!(legacy.dry, prepared.dry);
                            assert_eq!(legacy.absorb, prepared.absorb);
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn bench_add() {
        let source = black_box([0.7, -0.2, 0.4, 0.1, 0.3, 0.2, -0.1]);
        let properties = black_box([0.2, 0.8, 1.7, 0.0, 0.9]);
        let n = 10_000_000;
        for prepared in [false, true] {
            let mut pixel = Pixel::new();
            let sf = pixel.surf(true, true, true);
            let paint = AddPaint::new(sf, &source, properties, 0.1);
            let start = Instant::now();
            for _ in 0..n {
                unsafe {
                    if prepared {
                        paint.add(black_box(0), black_box(0.001));
                    } else {
                        sf.add(black_box(0), black_box(0.001), black_box(&source), black_box(properties), black_box(0.1));
                    }
                }
            }
            black_box(pixel);
            println!("prepared={prepared} ns/call={:.3}", start.elapsed().as_nanos() as f64 / n as f64);
        }
    }
}
