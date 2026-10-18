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
        let at = |x: isize, y: isize| {
            surface[y.clamp(0, h as isize - 1) as usize * w + x.clamp(0, w as isize - 1) as usize]
        };
        let k = 0.5 / um_px;
        let normals = (0..w * h)
            .into_par_iter()
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
            .collect();
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
                    // (at or above the highest paint nothing further shades it:
                    // a grazing ray, which doesn't climb, stops there at once)
                    if ray_height >= self.shade_hi || lit == 0.0 {
                        break;
                    }
                    let (px, py) = (x as f32 + sx * s as f32, y as f32 + sy * s as f32);
                    if px < 0.0 || py < 0.0 || px >= w as f32 || py >= h as f32 {
                        break;
                    }
                    let over = sat(px.round() as isize, py.round() as isize) - ray_height;
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
