use crate::{Canvas, surface::COAT_UM, wet::mix_into};

impl Canvas {
    /// Experimental fresh-film capillary relaxation, explicitly selected by the caller.
    /// Seconds are local to this study, not the drying clock. Each call starts a new
    /// shear-recovery interval; repeated calls are not equivalent to one longer call.
    /// Rejects solvent, aged films, partial coverage, drawings and crop windows.
    /// Returns the number of integration steps. No default engine path calls this.
    pub fn relax_fresh_body(&mut self, seconds: f32) -> Result<usize, &'static str> {
        if !seconds.is_finite() || !(0.0..=30.0).contains(&seconds) {
            return Err("body study requires 0..30 finite seconds");
        }
        if self.wet.vol.iter().any(|v| !v.is_finite() || *v < 0.0)
            || self.height.iter().any(|v| !v.is_finite())
            || self.wet.lat.iter().flatten().any(|v| !v.is_finite())
            || self.wet.hide.iter().flatten().any(|v| !v.is_finite())
            || self.wet.cover.iter().any(|v| !v.is_finite())
        {
            return Err("body study requires finite material fields and nonnegative volumes");
        }
        if !self.wet.clock.px.is_empty() || self.drawing.is_some() || self.soak.is_some()
            || self.wet.solv.iter().any(|&s| s != 0.0)
            || self.wet.hide.iter().zip(&self.wet.vol).any(|(p, &v)| v > 0.0 && p[3] != 0.0)
            || self.wet.cover.iter().zip(&self.wet.vol).any(|(&c, &v)| v > 0.0 && c != 1.0)
            || self.f.x0 != 0 || self.f.y0 != 0 || self.f.w != self.f.full_w || self.f.h != self.f.full_h
            || self.keep != (0, 0, self.f.w, self.f.h)
        {
            return Err("body study requires whole-canvas fresh, fully covered, solvent-free paint");
        }
        if seconds == 0.0 || self.wet.dirty.is_none() {
            return Ok(0);
        }
        let (w, h) = (self.f.w, self.f.h);
        let dx = self.px_mm() * 1e-3;
        if !dx.is_finite() || dx <= 0.0 {
            return Err("body study requires a positive physical pixel size");
        }
        let coat_m = COAT_UM * 1e-6;
        let mut wet = self.wet.clone();
        let n = wet.vol.len();
        let mut pressure = vec![0.0f32; n];
        let mut surface = vec![0.0f32; n];
        let mut viscosity = vec![0.0f32; n];
        let mut yield_stress = vec![0.0f32; n];
        let mut outgoing = vec![0.0f32; n];
        let mut flows = Vec::with_capacity(n * 2);
        let mut elapsed = 0.0f32;
        let mut steps = 0;
        while elapsed < seconds {
            if steps == 4096 {
                return Err("body study exceeded its integration budget; canvas unchanged");
            }
            let recovery = 1.0 - (-elapsed / 8.0).exp();
            for i in 0..n {
                let stiff = wet.hide[i][1].clamp(0.0, 1.0);
                viscosity[i] = 10f32.powf(0.3 + 3.0 * stiff) * (0.05 + 0.95 * recovery);
                yield_stress[i] = 5.0 * 600f32.powf(stiff) * (0.02 + 0.98 * recovery);
                let film = wet.vol[i] * coat_m;
                surface[i] = self.height[i] * 1e-6 + film;
            }
            for y in 0..h {
                for x in 0..w {
                    let i = y * w + x;
                    let left = y * w + x.saturating_sub(1);
                    let right = y * w + (x + 1).min(w - 1);
                    let up = y.saturating_sub(1) * w + x;
                    let down = (y + 1).min(h - 1) * w + x;
                    pressure[i] = -0.035 * (surface[left] + surface[right] + surface[up] + surface[down] - 4.0 * surface[i]) / dx.powi(2);
                    if !pressure[i].is_finite() {
                        return Err("body study pressure overflow; canvas unchanged");
                    }
                }
            }
            flows.clear();
            outgoing.fill(0.0);
            let mut mobility = 0.0f32;
            for y in 0..h {
                for x in 0..w {
                    let i = y * w + x;
                    for j in [if x + 1 < w { i + 1 } else { i }, if y + 1 < h { i + w } else { i }] {
                        if j == i { continue; }
                        let (from, to) = if pressure[i] > pressure[j] { (i, j) } else { (j, i) };
                        if wet.vol[from] <= 0.0 { continue; }
                        let thickness = 0.5 * (wet.vol[i] + wet.vol[j]) * coat_m;
                        let gradient = (pressure[from] - pressure[to]) / dx;
                        let stress = thickness * gradient;
                        let yielded = (1.0 - yield_stress[from] / stress.max(1e-20)).clamp(0.0, 1.0);
                        let face_mobility = thickness.powi(3) / (3.0 * viscosity[from]);
                        let amount = face_mobility * gradient * yielded / (dx * coat_m);
                        if !face_mobility.is_finite() || !amount.is_finite() || !stress.is_finite() {
                            return Err("body study flux overflow; canvas unchanged");
                        }
                        mobility = mobility.max(face_mobility);
                        if amount > 0.0 {
                            outgoing[from] += amount;
                            flows.push((from, to, amount));
                        }
                    }
                }
            }
            if flows.is_empty() { break; }
            let dt = (dx.powi(4) / (32.0 * 0.035 * mobility.max(1e-30))).min(0.1).min(seconds - elapsed);
            if !dt.is_finite() || dt <= 0.0 || elapsed + dt == elapsed {
                return Err("body study integration cannot advance; canvas unchanged");
            }
            let mut next = wet.clone();
            for i in 0..n {
                if !outgoing[i].is_finite() {
                    return Err("body study outflow overflow; canvas unchanged");
                }
                outgoing[i] *= dt;
                let scale = if outgoing[i] > 0.0 { (0.2 * wet.vol[i] / outgoing[i]).min(1.0) } else { 1.0 };
                next.vol[i] = (wet.vol[i] - outgoing[i] * scale).max(0.0);
                outgoing[i] = scale;
            }
            for &(from, to, amount) in &flows {
                let amount = amount * dt * outgoing[from];
                mix_into(&mut next.vol[to], &mut next.lat[to], &mut next.hide[to], amount, &wet.lat[from], wet.hide[from]);
            }
            if next.vol.iter().any(|v| !v.is_finite() || *v < 0.0)
                || next.lat.iter().flatten().any(|v| !v.is_finite())
                || next.hide.iter().flatten().any(|v| !v.is_finite())
            {
                return Err("body study output overflow; canvas unchanged");
            }
            wet = next;
            elapsed += dt;
            steps += 1;
        }
        if steps == 0 { return Ok(0); }
        wet.floor.fill(0.0);
        wet.touched.fill(0);
        wet.stroke.fill(0);
        wet.forget_laid();
        wet.touch(0, 0, w, h);
        self.wet = wet;
        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Canvas, Gesture, Held, Paint, Tool, color::hex};

    fn scene() -> Canvas {
        let mut c = Canvas::new_window(64, 2.0, hex("#c8bba5"), None).with_size_mm(16.0);
        let mut brush = Held::new(Tool::hog_flat(150.0), 19);
        brush.load(Paint { oil: 0.6, ..Paint::body(hex("#b7562d")).with_stiff(0.7).with_drying(0.4) }, 0.5);
        c.drag(&mut brush, &Gesture::line((150.0, 250.0), (850.0, 250.0)).shake(0.0), None);
        let mut blue = Held::new(Tool::hog_flat(120.0), 71);
        blue.load(Paint { oil: 1.4, ..Paint::body(hex("#325a8f")).with_stiff(0.65).with_drying(2.0) }, 0.3);
        c.drag(&mut blue, &Gesture::line((420.0, 100.0), (600.0, 400.0)).shake(0.0), None);
        c
    }

    fn snapshot(c: &Canvas) -> Vec<u8> {
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        b
    }

    #[test]
    fn body_study_conserves_inventory_and_checkpoint_continuation() {
        let mut c = scene();
        let original = c.clone();
        let before = c.wet_total();
        let moments = |c: &Canvas| -> Vec<f64> {
            (0..7).map(|k| c.wet.vol.iter().zip(&c.wet.lat).map(|(&v, l)| v as f64 * l[k] as f64).sum()).collect()
        };
        let initial = moments(&c);
        c.relax_fresh_body(2.0).unwrap();
        assert!((c.wet_total() / before - 1.0).abs() < 2e-6);
        for (a, b) in initial.iter().zip(moments(&c)) {
            assert!((a - b).abs() < 2e-5 * a.abs().max(1.0));
        }
        assert!(c.wet.vol.iter().all(|v| v.is_finite() && *v >= 0.0));
        assert_eq!(c.height, original.height);
        assert_ne!(c.wet.vol, original.wet.vol);
        let mut repeat = original.clone();
        repeat.relax_fresh_body(2.0).unwrap();
        assert_eq!(snapshot(&c), snapshot(&repeat));
        let bytes = snapshot(&c);
        let (mut restored, _) = Canvas::read_state(&mut bytes.as_slice()).unwrap();
        c.relax_fresh_body(0.25).unwrap();
        restored.relax_fresh_body(0.25).unwrap();
        assert_eq!(snapshot(&c), snapshot(&restored));
    }

    #[test]
    fn body_study_noop_and_rejections_leave_state_exact() {
        let mut c = scene();
        let before = snapshot(&c);
        assert_eq!(c.relax_fresh_body(0.0), Ok(0));
        assert!(c.relax_fresh_body(f32::NAN).is_err());
        assert_eq!(before, snapshot(&c));
        c.wait(1.0);
        let before = snapshot(&c);
        assert!(c.relax_fresh_body(1.0).is_err());
        assert_eq!(before, snapshot(&c));
    }

    #[test]
    fn body_study_conserves_five_material_property_moments() {
        let mut c = scene();
        let moments = |c: &Canvas| -> Vec<f64> {
            (0..5).map(|k| c.wet.vol.iter().zip(&c.wet.hide).map(|(&v, p)| v as f64 * p[k] as f64).sum()).collect()
        };
        let before = moments(&c);
        c.relax_fresh_body(2.0).unwrap();
        for (a, b) in before.iter().zip(moments(&c)) {
            assert!((a - b).abs() < 2e-5 * a.abs().max(1.0));
        }
    }

    #[test]
    fn body_study_heterogeneous_mobility_dissipates_surface_energy() {
        let mut c = Canvas::new_window(6, 1.0, hex("#ffffff"), None).with_size_mm(0.6);
        for i in 0..36 {
            let stiff = ((i % 6 + i / 6) % 2) as f32;
            c.wet.vol[i] = if stiff == 0.0 { 1.0 } else { 10.0 };
            c.wet.hide[i][1] = stiff;
            c.height[i] = 1000.0 - c.wet.vol[i] * crate::COAT_UM;
        }
        c.height[14] += 1.0;
        c.wet.touch(0, 0, 6, 6);
        let energy = |c: &Canvas| -> f64 {
            let h: Vec<f64> = c.height.iter().zip(&c.wet.vol).map(|(&h, &v)| h as f64 + v as f64 * crate::COAT_UM as f64).collect();
            (0..36).map(|i| {
                (if i % 6 < 5 { (h[i] - h[i + 1]).powi(2) } else { 0.0 })
                    + (if i / 6 < 5 { (h[i] - h[i + 6]).powi(2) } else { 0.0 })
            }).sum()
        };
        let before = energy(&c);
        c.relax_fresh_body(0.002).unwrap();
        assert!(energy(&c) < before);
    }

    #[test]
    fn body_study_rejects_invalid_or_unsupported_state_atomically() {
        for case in 0..5 {
            let mut c = scene();
            let i = c.wet.vol.iter().position(|&v| v > 0.0).unwrap();
            match case {
                0 => c.wet.vol[i] = f32::NAN,
                1 => c.height[i] = f32::INFINITY,
                2 => c.wet.cover[i] = 0.5,
                3 => { c.wet.solv = vec![0.0; c.wet.vol.len()]; c.wet.solv[i] = 1.0; }
                _ => c.wet.lat[i][0] = f32::NAN,
            }
            let before = snapshot(&c);
            assert!(c.relax_fresh_body(1.0).is_err());
            assert_eq!(snapshot(&c), before);
        }
        for margin in [0.0, 1000.0] {
            let mut c = Canvas::new_window(64, 2.0, hex("#ffffff"), Some(crate::Crop { units: [100.0, 100.0, 500.0, 300.0], margin }));
            let before = snapshot(&c);
            assert!(c.relax_fresh_body(1.0).is_err());
            assert_eq!(snapshot(&c), before);
        }
    }
}
