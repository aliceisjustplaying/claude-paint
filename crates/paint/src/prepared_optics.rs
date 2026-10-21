use crate::{Canvas, Frame, Rgb};
use rayon::prelude::*;

pub struct PreparedOptics {
    frame: Frame,
    color: Vec<Rgb>,
    wet: Vec<f32>,
    lighting: PreparedLighting,
}

impl PreparedOptics {
    pub(crate) fn capture(canvas: &Canvas) -> Self {
        Self::new(
            canvas.window(),
            canvas.px_mm() * 1000.0,
            canvas.seen(),
            &canvas.wet_surface(),
            canvas.wet.vol.clone(),
        )
    }

    pub(crate) fn new(
        frame: Frame,
        um_px: f32,
        color: Vec<Rgb>,
        surface: &[f32],
        wet: Vec<f32>,
    ) -> Self {
        Self {
            frame,
            color,
            wet,
            lighting: PreparedLighting::new(surface, frame.w, frame.h, um_px),
        }
    }

    pub fn frame(&self) -> Frame {
        self.frame
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.frame.w, self.frame.h)
    }

    pub fn color(&self) -> &[Rgb] {
        &self.color
    }

    pub fn seen_lit(&self, azimuth: f32, elevation: f32, strength: f32) -> Vec<Rgb> {
        self.lighting
            .lit(&self.color, &self.wet, azimuth, elevation, strength)
    }
}

pub(crate) struct PreparedLighting {
    w: usize,
    h: usize,
    um_px: f32,
    hi: f32,
    lo: f32,
    shade_hi: f32,
    shade_surface: Vec<f32>,
    normals: Vec<[f32; 3]>,
}

fn round_nonnegative(value: f32) -> isize {
    let whole = value as isize;
    whole.saturating_add(isize::from(value - whole as f32 >= 0.5))
}

impl PreparedLighting {
    pub(crate) fn new(surface: &[f32], w: usize, h: usize, um_px: f32) -> Self {
        let (hi, lo) = surface
            .par_iter()
            .fold(|| (f32::MIN, f32::MAX), |(a, b), &v| (a.max(v), b.min(v)))
            .reduce(
                || (f32::MIN, f32::MAX),
                |(a, b), (c, d)| (a.max(c), b.min(d)),
            );
        let shade_surface = crate::surface::box_blur(surface, w, h, 1);
        let shade_hi = shade_surface
            .par_iter()
            .copied()
            .reduce(|| f32::MIN, f32::max);
        let k = 0.5 / um_px;
        let mut normals = vec![[0.0; 3]; w * h];
        normals.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let center = &surface[y * w..(y + 1) * w];
            let above = &surface[y.saturating_sub(1) * w..(y.saturating_sub(1) + 1) * w];
            let below = &surface[(y + 1).min(h - 1) * w..((y + 1).min(h - 1) + 1) * w];
            for (x, normal) in row.iter_mut().enumerate() {
                let dx = (center[(x + 1).min(w - 1)] - center[x.saturating_sub(1)]) * k;
                let dy = (below[x] - above[x]) * k;
                let g = (dx * dx + dy * dy).sqrt();
                let soft = 1.0 / (1.0 + g / 1.2);
                let (dx, dy) = (dx * soft, dy * soft);
                let m = (dx * dx + dy * dy + 1.0).sqrt();
                *normal = [-dx / m, -dy / m, 1.0 / m];
            }
        });
        Self {
            w,
            h,
            um_px,
            hi,
            lo,
            shade_hi,
            shade_surface,
            normals,
        }
    }

    pub(crate) fn lit(
        &self,
        color: &[Rgb],
        wet: &[f32],
        azimuth: f32,
        elevation: f32,
        strength: f32,
    ) -> Vec<Rgb> {
        let ambient = crate::lerp(0.3, 0.55, crate::smoothstep(10.0, 55.0, elevation));
        let (w, h) = (self.w, self.h);
        // elevation as the easel takes it, 0 to 90 degrees: overhead (90) has
        // no horizontal part and casts no shadow; grazing (0) is a lamp in the
        // canvas's plane, which lights no flat paint, only slopes turned to it
        let elevation = elevation.clamp(0.0, 90.0);
        let (overhead, grazing) = (elevation >= 90.0, elevation <= 1e-3);
        let (az, el) = (azimuth.to_radians(), elevation.to_radians());
        let (lx, ly, lz) = if overhead {
            (0.0, 0.0, 1.0)
        } else if grazing {
            (az.cos(), -az.sin(), 0.0)
        } else {
            (el.cos() * az.cos(), -el.cos() * az.sin(), el.sin())
        };
        // µm the light ray climbs per pixel toward the light (overhead and grazing, none)
        let rise = if overhead || grazing { 0.0 } else { el.tan() * self.um_px };
        let m = (lx * lx + ly * ly).sqrt();
        let march = m > 1e-6 && (rise > 0.0 || grazing);
        let steps = if !march {
            0
        } else if grazing {
            w.saturating_add(h)
        } else {
            (((self.hi - self.lo) / rise).ceil() as usize)
                .max(1)
                .min(w.saturating_add(h))
        };
        let (sx, sy) = if march { (lx / m, ly / m) } else { (0.0, 0.0) };
        let rays: Vec<_> = (1..=steps)
            .map(|s| {
                let s = s as f32;
                (rise * s, sx * s, sy * s)
            })
            .collect();
        let sat = |x: isize, y: isize| {
            self.shade_surface
                [y.clamp(0, h as isize - 1) as usize * w + x.clamp(0, w as isize - 1) as usize]
        };
        let hv = {
            let v = [lx, ly, lz + 1.0];
            let hm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / hm, v[1] / hm, v[2] / hm]
        };
        (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                let n = self.normals[i];
                let ndl = (n[0] * lx + n[1] * ly + n[2] * lz).max(0.0);
                let h0 = self.shade_surface[i];
                let mut lit = 1.0f32;
                for &(height, dx, dy) in &rays {
                    let ray_height = h0 + height;
                    // (at or above the highest paint nothing further shades it:
                    // a grazing ray, which doesn't climb, stops there at once)
                    if ray_height >= self.shade_hi || lit == 0.0 {
                        break;
                    }
                    let (px, py) = (x as f32 + dx, y as f32 + dy);
                    if px < 0.0 || py < 0.0 || px >= w as f32 || py >= h as f32 {
                        break;
                    }
                    let over = sat(round_nonnegative(px), round_nonnegative(py)) - ray_height;
                    if over > 0.0 {
                        // (grazing, anything higher hides the lamp)
                        lit = if grazing {
                            0.0
                        } else {
                            lit.min(1.0 - (over / (0.5 * rise)).min(1.0))
                        };
                    }
                }
                // (a slope facing a low lamp is lit more than the flat canvas,
                // 1; capped, so the lowest lights don't burn ridges out to
                // white; grazing, only slopes toward the lamp are lit, by n·l
                // itself: lz is 0)
                let direct = if grazing { ndl * lit } else { ndl * lit / lz };
                let diffuse = (ambient + (1.0 - ambient) * direct).min(1.6);
                let shade = (1.0 + strength * (diffuse - 1.0)).max(0.0);
                let wet = (wet[i] * 4.0).min(1.0);
                let ndh = (n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]).max(0.0);
                let spec = (0.02 + 0.10 * wet) * ndh.powi(40) * lit;
                let p = color[i];
                [
                    (p[0] * shade + spec).max(0.0),
                    (p[1] * shade + spec).max(0.0),
                    (p[2] * shade + spec).max(0.0),
                ]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn original_normals(surface: &[f32], w: usize, h: usize, um_px: f32) -> Vec<[f32; 3]> {
        let at = |x: isize, y: isize| {
            surface[y.clamp(0, h as isize - 1) as usize * w + x.clamp(0, w as isize - 1) as usize]
        };
        let k = 0.5 / um_px;
        (0..w * h)
            .map(|i| {
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                let dx = (at(x + 1, y) - at(x - 1, y)) * k;
                let dy = (at(x, y + 1) - at(x, y - 1)) * k;
                let g = (dx * dx + dy * dy).sqrt();
                let soft = 1.0 / (1.0 + g / 1.2);
                let (dx, dy) = (dx * soft, dy * soft);
                let m = (dx * dx + dy * dy + 1.0).sqrt();
                [-dx / m, -dy / m, 1.0 / m]
            })
            .collect()
    }

    fn bits(pixels: &[[f32; 3]]) -> Vec<[u32; 3]> {
        pixels.iter().map(|p| p.map(f32::to_bits)).collect()
    }

    #[test]
    fn row_normals_match_original_bits_at_borders_and_extreme_slopes() {
        for (w, h) in [(1, 1), (1, 7), (7, 1), (3, 5), (64, 37), (321, 63)] {
            for pattern in 0..3 {
                let surface: Vec<_> = (0..w * h)
                    .map(|i| match pattern {
                        0 => {
                            if i % 2 == 0 {
                                0.0
                            } else {
                                -0.0
                            }
                        }
                        1 => (i % 17) as f32 * 0.125,
                        _ => ((i * 7919 % 104729) as f32 - 50000.0) * 100.0,
                    })
                    .collect();
                for um_px in [0.001, 1.0, 25000.0] {
                    let prepared = PreparedLighting::new(&surface, w, h, um_px);
                    assert_eq!(
                        bits(&prepared.normals),
                        bits(&original_normals(&surface, w, h, um_px)),
                        "{w}x{h}, pattern {pattern}, {um_px}"
                    );
                }
            }
        }
    }

    #[test]
    fn row_preparation_preserves_material_lighting_within_tolerance_in_whole_and_cropped_frames() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        use crate::tests::image_tolerance::{assert_close, assert_fields_close};
        for value in [0.0, -0.0, f32::from_bits(1), f32::MIN_POSITIVE, f32::MAX] {
            assert_eq!(round_nonnegative(value), value.round() as isize);
        }
        for whole in (0..4096)
            .map(|n| n as f32)
            .chain((12..=63).flat_map(|n| {
                let whole = 2.0f32.powi(n);
                [whole - 1.0, whole]
            }))
        {
            for center in [whole, whole + 0.5] {
                for value in [center.next_down(), center, center.next_up()] {
                    if value >= 0.0 {
                        assert_eq!(
                            round_nonnegative(value),
                            value.round() as isize,
                            "rounding {value:?}, bits {:08x}",
                            value.to_bits()
                        );
                    }
                }
            }
        }
        for crop in [
            None,
            Some(crate::Crop {
                units: [200.0, 100.0, 800.0, 650.0],
                margin: 30.0,
            }),
        ] {
            for material in 0..4 {
                let mut canvas = Canvas::new_window(96, 1.25, [0.3, 0.5, 0.2], crop)
                    .with_engine(8)
                    .with_size_mm(if material < 2 { 150.0 } else { 8.0 });
                if material < 2 {
                    canvas = canvas.with_linen(crate::Linen::fine(3));
                    canvas.prime([0.6, 0.5, 0.4], 0.8, 50.0, 0.6, 0.7, 7);
                    if material == 1 {
                        canvas.wet.solv.resize(canvas.px.len(), 0.0);
                        for i in 0..canvas.px.len() {
                            canvas.wet.vol[i] = (i % 7) as f32 * 0.07;
                            canvas.wet.solv[i] = (i % 11) as f32 * 3.0;
                        }
                    }
                } else {
                    canvas = canvas.with_paper(crate::paper::Paper::drawing(4));
                    if material == 2 {
                        let mark = crate::graphite::hand_line(
                            &[(250.0, 300.0), (750.0, 400.0)],
                            &[0.8],
                            false,
                            true,
                            0.0,
                            5,
                        );
                        canvas.draw(&crate::Lead::graphite(2.0), &mark, 0.0, 9);
                    } else {
                        let pose = Pose {
                            force: 3.0,
                            alt: 60f32.to_radians(),
                            az: 45f32.to_radians(),
                            roll: 0.0,
                        };
                        let points = [
                            StrokePoint {
                                x: 250.0,
                                y: 300.0,
                                pose,
                                speed: 50.0,
                            },
                            StrokePoint {
                                x: 750.0,
                                y: 400.0,
                                pose,
                                speed: 50.0,
                            },
                        ];
                        canvas
                            .stick_stroke(&mut Stick::round([0.02, 0.1, 0.4], 0.8, 12.0), &points);
                    }
                    assert!(canvas.has_drawing());
                }
                let f = canvas.window();
                let surface = canvas.wet_surface();
                let um_px = canvas.px_mm() * 1000.0;
                let prepared = PreparedLighting::new(&surface, f.w, f.h, um_px);
                let original = PreparedLighting {
                    normals: original_normals(&surface, f.w, f.h, um_px),
                    ..PreparedLighting::new(&surface, f.w, f.h, um_px)
                };
                assert_eq!(bits(&prepared.normals), bits(&original.normals));
                let color = canvas.seen();
                for (az, el, strength) in [
                    (115.0, 55.0, 1.0),
                    (135.0, 25.0, 1.0),
                    (45.0, 3.0, 0.4),
                    (270.0, 89.0, 0.0),
                    (0.0, 0.0, 1.0),
                    (90.0, 0.001, 1.0),
                    (-30.0, 90.0, 1.5),
                ] {
                    let actual = prepared.lit(&color, &canvas.wet.vol, az, el, strength);
                    let expected = original.original_lit(&color, &canvas.wet.vol, az, el, strength);
                    assert_fields_close(
                        actual.iter().flatten().copied(),
                        expected.iter().flatten().copied(),
                    );
                    let image = |pixels: &[Rgb]| {
                        image::RgbaImage::from_fn(f.w as u32, f.h as u32, |x, y| {
                            let [r, g, b] = pixels[y as usize * f.w + x as usize].map(|v| {
                                (crate::color::linear_to_srgb(v) * 255.0).round() as u8
                            });
                            image::Rgba([r, g, b, 255])
                        })
                    };
                    assert_close(&image(&actual), &image(&expected));
                }
            }
        }
    }

    impl PreparedLighting {
        fn original_lit(
            &self,
            color: &[Rgb],
            wet: &[f32],
            azimuth: f32,
            elevation: f32,
            strength: f32,
        ) -> Vec<Rgb> {
            let ambient = crate::lerp(0.3, 0.55, crate::smoothstep(10.0, 55.0, elevation));
            let (w, h) = (self.w, self.h);
            let elevation = elevation.clamp(0.0, 90.0);
            let (overhead, grazing) = (elevation >= 90.0, elevation <= 1e-3);
            let (az, el) = (azimuth.to_radians(), elevation.to_radians());
            let (lx, ly, lz) = if overhead {
                (0.0, 0.0, 1.0)
            } else if grazing {
                (az.cos(), -az.sin(), 0.0)
            } else {
                (el.cos() * az.cos(), -el.cos() * az.sin(), el.sin())
            };
            let rise = if overhead || grazing {
                0.0
            } else {
                el.tan() * self.um_px
            };
            let m = (lx * lx + ly * ly).sqrt();
            let march = m > 1e-6 && (rise > 0.0 || grazing);
            let steps = if !march {
                0
            } else if grazing {
                w.saturating_add(h)
            } else {
                (((self.hi - self.lo) / rise).ceil() as usize)
                    .max(1)
                    .min(w.saturating_add(h))
            };
            let (sx, sy) = if march { (lx / m, ly / m) } else { (0.0, 0.0) };
            let sat = |x: isize, y: isize| {
                self.shade_surface
                    [y.clamp(0, h as isize - 1) as usize * w + x.clamp(0, w as isize - 1) as usize]
            };
            let hv = {
                let v = [lx, ly, lz + 1.0];
                let hm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                [v[0] / hm, v[1] / hm, v[2] / hm]
            };
            (0..w * h)
                .into_par_iter()
                .map(|i| {
                    let (x, y) = ((i % w) as isize, (i / w) as isize);
                    let n = self.normals[i];
                    let ndl = (n[0] * lx + n[1] * ly + n[2] * lz).max(0.0);
                    let h0 = self.shade_surface[i];
                    let mut lit = 1.0f32;
                    for s in 1..=steps {
                        let ray_height = h0 + rise * s as f32;
                        if ray_height >= self.shade_hi || lit == 0.0 {
                            break;
                        }
                        let (px, py) = (x as f32 + sx * s as f32, y as f32 + sy * s as f32);
                        if px < 0.0 || py < 0.0 || px >= w as f32 || py >= h as f32 {
                            break;
                        }
                        let over = sat(px.round() as isize, py.round() as isize) - ray_height;
                        if over > 0.0 {
                            lit = if grazing {
                                0.0
                            } else {
                                lit.min(1.0 - (over / (0.5 * rise)).min(1.0))
                            };
                        }
                    }
                    let direct = if grazing { ndl * lit } else { ndl * lit / lz };
                    let diffuse = (ambient + (1.0 - ambient) * direct).min(1.6);
                    let shade = (1.0 + strength * (diffuse - 1.0)).max(0.0);
                    let wet = (wet[i] * 4.0).min(1.0);
                    let ndh = (n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]).max(0.0);
                    let spec = (0.02 + 0.10 * wet) * ndh.powf(40.0) * lit;
                    let p = color[i];
                    [
                        (p[0] * shade + spec).max(0.0),
                        (p[1] * shade + spec).max(0.0),
                        (p[2] * shade + spec).max(0.0),
                    ]
                })
                .collect()
        }
    }
}
