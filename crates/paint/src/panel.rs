use crate::surface::{COAT_UM, vnoise};
use crate::{Canvas, Rgb};
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topography {
    Smooth,
    QuarterSawnOak,
    QuarterSawnOakRays,
    QuarterSawnOakGallery,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ground {
    ChalkGlue,
    OilPrimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Panel {
    pub topography: Topography,
    pub ground: Ground,
    pub seed: u64,
}

impl Panel {
    pub fn chalk(seed: u64) -> Self {
        Self {
            topography: Topography::Smooth,
            ground: Ground::ChalkGlue,
            seed,
        }
    }

    pub fn oil_capacity_coats(self) -> f32 {
        match self.ground {
            Ground::ChalkGlue => crate::canvas::ABSORB_COATS,
            Ground::OilPrimed => 0.0,
        }
    }

    pub fn color(self) -> Rgb {
        crate::hex("#f4f2ea")
    }

    pub fn ground_um(self) -> f32 {
        180.0
    }

    pub(crate) fn height_um(self, x: f32, y: f32) -> f32 {
        let s = self.seed;
        let tooth = 0.65 * (vnoise(x / 0.11, y / 0.11, s ^ 0x391) - 0.5);
        let grain = match self.topography {
            Topography::Smooth => 0.0,
            Topography::QuarterSawnOak => {
                let wander = 1.8 * (vnoise(x / 35.0, y / 18.0, s ^ 0x392) - 0.5);
                let along = x + wander;
                let oak = 4.0 * (vnoise(along / 0.85, y / 45.0, s ^ 0x393) - 0.5);
                let ridge = 1.3 * (vnoise((along + 0.3 * y) / 0.24, y / 12.0, s ^ 0x394) - 0.5);
                let uneven = 0.6 + 0.8 * vnoise(x / 12.0, y / 16.0, s ^ 0x395);
                uneven * (oak + ridge)
            }
            Topography::QuarterSawnOakRays | Topography::QuarterSawnOakGallery => {
                let gallery = self.topography == Topography::QuarterSawnOakGallery;
                let wander = 0.9 * (vnoise(x / 24.0, y / 15.0, s ^ 0x396) - 0.5)
                    + 0.25 * (vnoise(x / 9.0, y / 6.0, s ^ 0x397) - 0.5);
                let along = x + wander;
                let growth = (if gallery { 10.0 } else { 8.0 })
                    * (crate::smoothstep(
                        if gallery { 0.46 } else { 0.42 },
                        if gallery { 0.61 } else { 0.67 },
                        vnoise(along / 1.4, y / 95.0, s ^ 0x398),
                    ) - 0.5);
                let fine = (if gallery { 8.0 } else { 3.0 })
                    * (vnoise(along / 0.21, y / 60.0, s ^ 0x399) - 0.5);
                let ray = crate::smoothstep(
                    0.57,
                    0.76,
                    vnoise((x + 0.12 * y) / 4.5, y / 0.27, s ^ 0x39a),
                );
                let broken = crate::smoothstep(0.32, 0.7, vnoise(x / 22.0, y / 19.0, s ^ 0x39b));
                let uneven = 0.65 + 0.7 * vnoise(x / 14.0, y / 23.0, s ^ 0x39c);
                uneven * (growth + fine) + 2.8 * ray * broken
            }
        };
        self.ground_um() + tooth + grain
    }
}

#[derive(Clone)]
pub(crate) struct State {
    pub descriptor: Panel,
    pub oil_absorbed: Vec<f32>,
}

impl Canvas {
    pub fn with_panel(mut self, descriptor: Panel) -> Self {
        assert!(
            self.engine >= 4,
            "experimental panel requires the engine-4 oil channels"
        );
        assert!(self.soak.is_none(), "a panel cannot also be raw fabric");
        assert!(
            self.film.iter().all(|&v| v == 0.0)
                && self.wet.vol.iter().all(|&v| v == 0.0)
                && self.drawing.is_none(),
            "set the experimental panel before priming, painting or drawing"
        );
        self.linen = None;
        self.panel = Some(Box::new(State {
            descriptor,
            oil_absorbed: vec![0.0; self.height.len()],
        }));
        self.px.fill(descriptor.color());
        self.film.fill(descriptor.ground_um() / COAT_UM);
        self.ground_um = descriptor.ground_um();
        self.ground_finish(match descriptor.ground {
            Ground::ChalkGlue => 1.0,
            Ground::OilPrimed => 0.0,
        });
        self.build_panel();
        self
    }

    pub fn panel(&self) -> Option<Panel> {
        self.panel.as_ref().map(|s| s.descriptor)
    }

    pub fn panel_oil_absorbed(&self) -> Option<&[f32]> {
        self.panel.as_ref().map(|s| s.oil_absorbed.as_slice())
    }

    pub fn panel_oil_available(&self) -> Option<&[f32]> {
        self.panel.as_ref().map(|_| self.absorb.as_slice())
    }

    pub fn panel_gloss(&self) -> Option<&[f32]> {
        self.panel.as_ref().map(|_| self.gloss.as_slice())
    }

    pub(crate) fn build_panel(&mut self) {
        let p = self.panel.as_ref().unwrap().descriptor;
        let f = self.f;
        let px = self.px_mm();
        let ss = ((px / 0.055).ceil() as usize).clamp(2, 8);
        self.height
            .par_chunks_mut(f.w)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, h) in row.iter_mut().enumerate() {
                    let mut sum = 0.0;
                    for j in 0..ss {
                        for i in 0..ss {
                            let xm = ((x + f.x0) as f32 + (i as f32 + 0.5) / ss as f32) * px;
                            let ym = ((y + f.y0) as f32 + (j as f32 + 0.5) / ss as f32) * px;
                            sum += p.height_um(xm, ym);
                        }
                    }
                    *h = sum / (ss * ss) as f32;
                }
            });
        self.surf_gen += 1;
        self.base = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cracks, Crop, Gesture, Held, Lead, Linen, Paint, Tool, Touch};

    fn sample(p: Panel) -> Canvas {
        Canvas::new_window(160, 1.0, p.color(), None)
            .with_size_mm(40.0)
            .with_panel(p)
    }

    fn stroke(c: &mut Canvas) {
        let mut h = Held::new(Tool::hog_flat(80.0), 19).with_engine(c.engine());
        h.load(Paint::new(crate::hex("#773d25"), 0.2, 0.3), 0.7);
        c.drag(
            &mut h,
            &Gesture::line((100.0, 400.0), (900.0, 500.0)).shake(0.2),
            None,
        );
    }

    fn bytes(c: &Canvas) -> Vec<u8> {
        let mut v = Vec::new();
        c.write_state(&mut v, "panel probe").unwrap();
        v
    }

    #[test]
    fn panel_ground_is_bright_and_support_has_no_linen() {
        let smooth = sample(Panel::chalk(7));
        let oak = sample(Panel {
            topography: Topography::QuarterSawnOak,
            ..Panel::chalk(7)
        });
        for c in [&smooth, &oak] {
            assert!(c.linen.is_none() && c.soak.is_none());
            assert_eq!(c.ground_um(), 180.0);
            assert!(
                c.pixels()
                    .iter()
                    .all(|p| p[0] > 0.8 && p[1] > 0.8 && p[2] > 0.8)
            );
        }
        let span = |c: &Canvas| {
            c.height.iter().copied().fold(f32::MIN, f32::max)
                - c.height.iter().copied().fold(f32::MAX, f32::min)
        };
        assert!(span(&smooth) < 0.65);
        assert!(span(&oak) > 1.5 && span(&oak) < 6.0);
        assert_ne!(smooth.height, oak.height);
    }

    #[test]
    fn panel_geometry_is_physical_and_crop_anchored() {
        let p = Panel {
            topography: Topography::QuarterSawnOak,
            ..Panel::chalk(23)
        };
        let full = sample(p);
        let crop = Crop {
            units: [200.0, 200.0, 750.0, 750.0],
            margin: 0.0,
        };
        let window = Canvas::new_window(160, 1.0, p.color(), Some(crop))
            .with_size_mm(40.0)
            .with_panel(p);
        for i in 0..window.height.len() {
            assert_eq!(
                window.height[i].to_bits(),
                full.height[window.f.whole_index(i)].to_bits()
            );
        }
    }

    #[test]
    fn panel_checkpoint_and_continuation_preserve_every_byte() {
        for ground in [Ground::ChalkGlue, Ground::OilPrimed] {
            for topography in [Topography::Smooth, Topography::QuarterSawnOak] {
                let mut c = sample(Panel {
                    ground,
                    topography,
                    seed: 7,
                });
                let mark = crate::graphite::hand_line(
                    &[(80.0, 180.0), (860.0, 240.0)],
                    &[0.6],
                    false,
                    true,
                    0.0,
                    3,
                );
                c.draw(&Lead::pencil("2B").unwrap(), &mark, 0.0, 9);
                stroke(&mut c);
                c.wait(30.0);
                let state = bytes(&c);
                assert_eq!(&state[..8], b"PAINTP01");
                let (mut restored, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
                assert_eq!(state, bytes(&restored));
                for (a, b) in [
                    (c.seen(), restored.seen()),
                    (
                        c.seen_lit(115.0, 55.0, 1.0),
                        restored.seen_lit(115.0, 55.0, 1.0),
                    ),
                    (
                        c.seen_lit(135.0, 25.0, 1.0),
                        restored.seen_lit(135.0, 25.0, 1.0),
                    ),
                ] {
                    assert_eq!(a.len(), b.len());
                    assert!(
                        a.iter()
                            .flatten()
                            .zip(b.iter().flatten())
                            .all(|(a, b)| a.to_bits() == b.to_bits())
                    );
                }
                stroke(&mut c);
                stroke(&mut restored);
                c.wait(120.0);
                restored.wait(120.0);
                assert_eq!(bytes(&c), bytes(&restored));
            }
        }
    }

    #[test]
    fn panel_checkpoint_refuses_bad_inventory_tags_and_tail() {
        let c = sample(Panel::chalk(7));
        let data = bytes(&c);
        let start = data.len() - c.height.len() * 4 - 24;
        for (offset, value) in [(start, 8u64), (start + 8, 8u64)] {
            let mut bad = data.clone();
            bad[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
            assert!(Canvas::read_state(&mut bad.as_slice()).is_err());
        }
        for v in [f32::NAN, -1.0, 2.0] {
            let mut bad = data.clone();
            bad[start + 24..start + 28].copy_from_slice(&v.to_le_bytes());
            assert!(Canvas::read_state(&mut bad.as_slice()).is_err());
        }
        let mut bad = data.clone();
        bad.push(0);
        assert!(Canvas::read_state(&mut bad.as_slice()).is_err());
        assert!(Canvas::read_state(&mut &data[..data.len() - 1]).is_err());
    }

    #[test]
    fn linen_checkpoints_keep_their_historical_magic() {
        for engine in 1..=5 {
            let c = Canvas::new_window(16, 1.0, [0.5; 3], None)
                .with_engine(engine)
                .with_linen(Linen::fine(7));
            let state = bytes(&c);
            let expected: &[u8; 8] = match engine {
                1 | 2 => b"PAINTCK8",
                3 => b"PAINTCK9",
                _ => b"PAINTC11",
            };
            assert_eq!(&state[..8], expected);
            let (d, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
            assert!(d.panel.is_none());
            assert_eq!(state, bytes(&d));
        }
    }

    #[test]
    fn thin_translucent_oil_preserves_bright_ground_and_oak_relief() {
        let p = Panel {
            topography: Topography::QuarterSawnOak,
            ..Panel::chalk(7)
        };
        let mut c = sample(p);
        let before = c.height.clone();
        c.glaze(
            &crate::Pigment::masstone_hiding(crate::hex("#643d23"), 0.07),
            None,
            |_, _| 0.18,
        );
        assert!(c.seen().iter().all(|p| crate::color::luminance(*p) > 0.65));
        let before_range = before.iter().copied().fold(f32::MIN, f32::max)
            - before.iter().copied().fold(f32::MAX, f32::min);
        let after_range = c.height.iter().copied().fold(f32::MIN, f32::max)
            - c.height.iter().copied().fold(f32::MAX, f32::min);
        assert!(after_range > 0.7 * before_range);
    }

    #[test]
    fn chalk_uptake_saturates_and_oil_ground_takes_nothing() {
        let mut chalk = sample(Panel::chalk(7));
        let mut oil = sample(Panel {
            ground: Ground::OilPrimed,
            ..Panel::chalk(7)
        });
        let center = chalk.f.index(500.0, 500.0);
        let initial = chalk.absorb[center];
        for _ in 0..32 {
            for c in [&mut chalk, &mut oil] {
                let mut h = Held::new(Tool::hog_flat(180.0), 31).with_engine(5);
                h.load(
                    Paint {
                        oil: 2.0,
                        ..Paint::scumble(crate::hex("#974729"))
                    },
                    1.0,
                );
                c.touch(&mut h, &Touch::at(500.0, 500.0).pressure(1.0), None);
            }
        }
        let taken = chalk.panel_oil_absorbed().unwrap()[center];
        assert!(
            taken > 0.99 * initial,
            "center took {taken} of {initial} coats"
        );
        assert!(chalk.absorb[center] <= 0.01 * initial);
        assert!((taken + chalk.absorb[center] - initial).abs() < 1e-6);
        assert!(oil.panel_oil_absorbed().unwrap().iter().all(|&v| v == 0.0));
        let before = chalk.panel_oil_absorbed().unwrap().to_vec();
        chalk.dry();
        assert_eq!(chalk.panel_oil_absorbed().unwrap(), before);
        for (&a, &r) in before.iter().zip(&chalk.absorb) {
            assert!(a + r <= initial + 1e-5);
        }
    }

    #[test]
    fn panel_cracks_ignore_fabric_corner_and_directional_stress() {
        let mut a = sample(Panel::chalk(7));
        let mut b = a.clone();
        let k = Cracks {
            ground_um: Some(5.0),
            island_mm: Some(3.0),
            grain: 1.0,
            corners: true,
            ..Cracks::aged(11)
        };
        a.crack(&k);
        b.crack(&Cracks {
            corners: false,
            grain: 0.0,
            ..k
        });
        assert_eq!(bytes(&a), bytes(&b));
        assert_ne!(a.height, sample(Panel::chalk(7)).height);
    }

    #[test]
    fn oak_rays_are_resolved_and_keep_their_checkpoint_tag() {
        let p = Panel {
            topography: Topography::QuarterSawnOakRays,
            ..Panel::chalk(23)
        };
        let c = sample(p);
        let span = c.height.iter().copied().fold(f32::MIN, f32::max)
            - c.height.iter().copied().fold(f32::MAX, f32::min);
        assert!(span > 6.0 && span < 20.0, "ray ground span {span} µm");
        let state = bytes(&c);
        let (d, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
        assert_eq!(d.panel(), Some(p));
        assert_eq!(state, bytes(&d));
        let crop = Crop {
            units: [200.0, 200.0, 750.0, 750.0],
            margin: 0.0,
        };
        let window = Canvas::new_window(160, 1.0, p.color(), Some(crop))
            .with_size_mm(40.0)
            .with_panel(p);
        for i in 0..window.height.len() {
            assert_eq!(
                window.height[i].to_bits(),
                c.height[window.f.whole_index(i)].to_bits()
            );
        }
    }

    #[test]
    fn impasto_bridges_ray_ground_more_than_a_thin_film() {
        let mut thin = sample(Panel {
            topography: Topography::QuarterSawnOakRays,
            ..Panel::chalk(23)
        });
        let mut thick = thin.clone();
        let paint = Paint::body(crate::hex("#ebe4ce"));
        for (c, coats) in [(&mut thin, 0.15), (&mut thick, 5.0)] {
            c.wet.vol.fill(coats);
            c.wet.lat.fill(paint.latent());
            c.wet.hide.fill(paint.prop());
            c.wet.touch(0, 0, c.f.w, c.f.h);
        }
        let fine_rms = |c: &Canvas| {
            let h = c.wet_surface();
            let low = crate::surface::box_blur(
                &crate::surface::box_blur(&h, c.f.w, c.f.h, 1),
                c.f.w,
                c.f.h,
                1,
            );
            (h.iter()
                .zip(low)
                .map(|(&v, l)| ((v - l) as f64).powi(2))
                .sum::<f64>()
                / h.len() as f64)
                .sqrt()
        };
        assert!(fine_rms(&thick) < 0.6 * fine_rms(&thin));
    }

    #[test]
    fn panel_gallery_sheen_uses_gloss_without_changing_color() {
        let mut matte = sample(Panel::chalk(7));
        matte.px.fill(crate::hex("#3b3028"));
        matte.gloss.fill(0.05);
        let mut oily = matte.clone();
        oily.gloss.fill(0.8);
        let m = matte.seen_lit(115.0, 55.0, 1.0);
        let o = oily.seen_lit(115.0, 55.0, 1.0);
        let sheen = |c: &Canvas, lit: &[Rgb]| {
            let color = c.seen();
            lit.iter()
                .zip(color)
                .map(|(a, b)| crate::color::luminance(*a) - crate::color::luminance(b))
                .sum::<f32>()
                / lit.len() as f32
        };
        assert!(sheen(&oily, &o) > sheen(&matte, &m) + 0.005);
        assert_eq!(matte.px, oily.px);
    }

    #[test]
    fn gallery_oak_resolves_more_fine_relief_without_weave() {
        let build = |topography| {
            Canvas::new_window(640, 1.0, Panel::chalk(23).color(), None)
                .with_size_mm(40.0)
                .with_panel(Panel {
                    topography,
                    ..Panel::chalk(23)
                })
        };
        let old = build(Topography::QuarterSawnOakRays);
        let new = build(Topography::QuarterSawnOakGallery);
        let rms = |c: &Canvas| {
            let lit = c.seen_lit(115.0, 55.0, 1.0);
            let mean = lit
                .iter()
                .map(|p| crate::color::luminance(*p) as f64)
                .sum::<f64>()
                / lit.len() as f64;
            (lit.iter()
                .map(|p| (crate::color::luminance(*p) as f64 - mean).powi(2))
                .sum::<f64>()
                / lit.len() as f64)
                .sqrt()
        };
        assert!(rms(&new) > 1.5 * rms(&old));
        assert!(new.linen.is_none());
        let state = bytes(&new);
        let (restored, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
        assert_eq!(new.panel(), restored.panel());
        assert_eq!(state, bytes(&restored));
    }

    #[cfg(feature = "brush-sim-2")]
    #[test]
    fn combined_panel_and_body2_checkpoint_resumes_exactly() {
        let mut c = sample(Panel {
            topography: Topography::QuarterSawnOakGallery,
            ..Panel::chalk(23)
        })
        .with_engine(10);
        stroke(&mut c);
        c.wait(30.0);
        let state = bytes(&c);
        let (mut d, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
        assert_eq!(state, bytes(&d));
        stroke(&mut c);
        stroke(&mut d);
        c.wait(90.0);
        d.wait(90.0);
        assert_eq!(bytes(&c), bytes(&d));
        assert!(c.panel_oil_absorbed().unwrap().iter().any(|&v| v > 0.0));
    }
}
