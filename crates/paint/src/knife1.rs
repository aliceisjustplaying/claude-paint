use super::*;

impl Knife {
    pub fn experimental(mut self, flex: f32) -> Self {
        if flex.is_finite() && (0.0..=1.0).contains(&flex) {
            self.flex = Some(flex);
        }
        self
    }

    pub fn is_experimental(&self) -> bool {
        self.flex.is_some()
    }

    pub fn paint_volume(&self) -> f64 {
        self.vol as f64
    }

    pub fn solvent_volume(&self) -> f64 {
        self.solvent as f64
    }
}

impl Canvas {
    pub fn knife_1(
        &mut self,
        k: &mut Knife,
        pts: &[(f32, f32)],
        pressure: (f32, f32),
        angle: Option<f32>,
        lay: bool,
        lift: f32,
        tilt: f32,
    ) {
        let Some(flex) = k.flex else { return };
        if Knife::check_path(pts, self.f.scale).is_err()
            || !pressure.0.is_finite()
            || !pressure.1.is_finite()
            || angle.is_some_and(|a| !a.is_finite())
            || !lift.is_finite()
            || !(0.0..=90.0).contains(&tilt)
            || !(k.width.is_finite() && (1.0..=1000.0).contains(&k.width))
        {
            return;
        }
        self.assert_thinner_supported(k.solvent > 0.0, "Canvas::knife_1");
        self.tally
            .stroke(&Tool::hog_flat(k.width), pts, self.mm_per_unit);
        let id = self.next_stroke_ids(1);
        let seed = (id as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xB1ADE;
        let height = self.height.as_ptr();
        let sf = self.surf();
        let s = sf.scale;
        let area = 1.0 / (s * s);
        let mm_px = self.px_mm();
        let p: Vec<_> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
        let path = if p.len() > 1 {
            densify(&p)
        } else {
            vec![p[0], p[0]]
        };
        let mut arc = vec![0.0; path.len()];
        for j in 1..path.len() {
            arc[j] = arc[j - 1] + (path[j].0 - path[j - 1].0).hypot(path[j].1 - path[j - 1].1);
        }
        let total = arc[arc.len() - 1];
        let n = (total / 0.65).ceil().max(1.0) as usize;
        let advance = if total > 0.0 { total / n as f32 } else { 0.65 };
        let half = k.width * s * 0.5;
        let upright = tilt.to_radians().sin();
        let depth = (k.width * s * 0.18 * (1.0 - upright)).max(0.65);
        let nx = (2.0 * half / 0.65).ceil().max(2.0) as usize;
        let ny = (depth / 0.65).ceil().max(1.0) as usize;
        let pix = |x: f32, y: f32| -> Option<usize> {
            if x < sf.ox as f32
                || y < sf.oy as f32
                || x >= (sf.ox + sf.w) as f32
                || y >= (sf.oy + sf.h) as f32
            {
                return None;
            }
            Some((y.floor() as usize - sf.oy) * sf.w + x.floor() as usize - sf.ox)
        };
        let mut bounds: Bounds = None;
        let mut blade_paint = k.vol as f64;
        let mut blade_solvent = k.solvent as f64;
        let mut seg = 0;
        let mut last = (path[0], (1.0, 0.0), (0.0, 1.0));
        for step in 0..=n {
            let d = total * step as f32 / n as f32;
            while seg + 1 < path.len() - 1 && arc[seg + 1] < d {
                seg += 1;
            }
            let (a, b) = (path[seg], path[seg + 1]);
            let len = (arc[seg + 1] - arc[seg]).max(1e-6);
            let f = ((d - arc[seg]) / len).clamp(0.0, 1.0);
            let c = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            let dir = if len > 1e-6 {
                ((b.0 - a.0) / len, (b.1 - a.1) / len)
            } else {
                last.1
            };
            let e = angle.map_or((-dir.1, dir.0), |a| (a.cos(), a.sin()));
            let face = if -e.1 * dir.0 + e.0 * dir.1 > 0.0 {
                (e.1, -e.0)
            } else {
                (-e.1, e.0)
            };
            let span = 2.0 * half * (e.0 * dir.0 + e.1 * dir.1).abs()
                + depth * (face.0 * dir.0 + face.1 * dir.1).abs();
            let duty = (advance / span.max(0.65)).min(1.0);
            last = (c, dir, e);
            let pr =
                (pressure.0 + (pressure.1 - pressure.0) * step as f32 / n as f32).clamp(0.0, 1.0);
            let gap = 280.0 * (1.0 - pr).powf(1.6) * (1.0 - 0.7 * upright);
            let mut cells = Vec::new();
            for q in 0..=nx {
                let u = -1.0 + 2.0 * q as f32 / nx as f32;
                let drift = 0.7 * crate::surface::vnoise(u * 3.0, d * mm_px * 0.6, seed) - 0.35;
                let parting = depth
                    * 0.65
                    * (crate::surface::vnoise(u * 6.0 + 2.0, d * mm_px * 0.2, seed ^ 0xED6E) - 0.5);
                for r in 0..=ny {
                    let back = depth * r as f32 / ny as f32 + parting;
                    let x = c.0 + e.0 * (u * half + drift) + face.0 * back;
                    let y = c.1 + e.1 * (u * half + drift) + face.1 * back;
                    if let Some(i) = pix(x, y) {
                        cells.push((i, u, x, y, q));
                    }
                }
            }
            cells.sort_by_key(|v| v.0);
            cells.dedup_by_key(|v| v.0);
            if cells.is_empty() {
                continue;
            }
            let rigid = cells
                .iter()
                .map(|v| unsafe { *height.add(v.0) })
                .fold(f32::MIN, f32::max);
            let mut support = vec![f32::MIN; nx + 1];
            for cell in &cells {
                support[cell.4] = support[cell.4].max(unsafe { *height.add(cell.0) });
            }
            let radius = (((1.0 + 5.0 * (1.0 - flex)) / (k.width * self.mm_per_unit)) * nx as f32)
                .ceil()
                .max(1.0) as usize;
            let rest: Vec<_> = (0..=nx)
                .map(|q| {
                    let local = support[q.saturating_sub(radius)..(q + radius + 1).min(nx + 1)]
                        .iter()
                        .copied()
                        .fold(f32::MIN, f32::max);
                    rigid + (local - rigid) * flex * pr.sqrt()
                })
                .collect();
            for &(i, u, x, y, q) in &cells {
                let rest = rest[q];
                let noise = crate::surface::vnoise(u * 4.0 + 9.0, d * mm_px * 0.8, seed) * 0.75
                    + crate::surface::vnoise(u * 13.0, d * mm_px * 2.1, seed ^ 41) * 0.25;
                let penetration = 85.0 * pr * pr * (0.25 + 0.75 * upright) * (0.3 + 0.7 * flex);
                let plane = rest + gap * (0.7 + 0.6 * noise) - penetration;
                let reach = (2.0 * gap + 45.0 * pr) * (0.45 + noise);
                unsafe {
                    let v = *sf.vol.add(i);
                    let sol = if sf.solv.is_null() {
                        0.0
                    } else {
                        *sf.solv.add(i) / COAT_UM
                    };
                    let top = *height.add(i) + (v + sol) * COAT_UM;
                    let fluid = if sf.dry.is_null() {
                        1.0
                    } else {
                        crate::drying::fluid((*sf.dry.add(i)).cure)
                    };
                    let cut = ((top - plane).max(0.0) / COAT_UM).min(v + sol) * fluid;
                    let touches = top >= plane - 5.0 || k.vol > 1e-9 && plane - top <= reach;
                    let stir = if lay && touches {
                        (v + sol) * fluid * duty * (0.025 + 0.18 * pr)
                    } else {
                        0.0
                    };
                    let take = cut.max(stir).min(v + sol);
                    if take > 1e-7 && v > 1e-9 {
                        let phi = sol / (v + sol);
                        let (l, hd) = (*sf.lat.add(i), *sf.hide.add(i));
                        let cure = if sf.dry.is_null() {
                            0.0
                        } else {
                            (*sf.dry.add(i)).cure
                        };
                        sf.take(i, take * (1.0 - phi));
                        if !sf.solv.is_null() {
                            *sf.solv.add(i) -= take * phi * COAT_UM;
                        }
                        let ridge = if u.abs() > 0.72 {
                            pix(x + e.0 * u.signum() * 1.2, y + e.1 * u.signum() * 1.2)
                        } else {
                            None
                        };
                        let spill = if ridge.is_some() {
                            take * 0.22 * (0.5 + noise)
                        } else {
                            0.0
                        };
                        if let Some(j) = ridge {
                            sf.add(j, spill * (1.0 - phi), &l, hd, cure);
                            if !sf.solv.is_null() {
                                *sf.solv.add(j) += spill * phi * COAT_UM;
                            }
                            *sf.cover.add(j) = 1.0;
                            let rx = sf.ox + j % sf.w;
                            let ry = sf.oy + j / sf.w;
                            grow(&mut bounds, rx, ry, rx + 1, ry + 1);
                        }
                        let paint = (take - spill) * (1.0 - phi) * area;
                        k.cure = mix_cure(k.cure, k.vol, cure, paint);
                        mix_into(&mut k.vol, &mut k.lat, &mut k.hide, paint, &l, hd);
                        blade_paint += paint as f64;
                        blade_solvent += ((take - spill) * phi * area) as f64;
                        k.vol = blade_paint as f32;
                        k.solvent = blade_solvent as f32;
                    }
                    let now = *height.add(i)
                        + (*sf.vol.add(i)
                            + if sf.solv.is_null() {
                                0.0
                            } else {
                                *sf.solv.add(i) / COAT_UM
                            })
                            * COAT_UM;
                    let edge = u.abs() > 0.8 && noise < (u.abs() - 0.8) * 3.5;
                    let spent = (k.fullness() / 0.12).min(1.0);
                    let skip = noise < (1.0 - spent) * 0.33;
                    let flow = 1.0 - smoothstep(0.15, 0.35, k.hide[1]);
                    let wet_reach = reach + 220.0 * flow * pr;
                    let fluid_contact = flow > 0.0 && now - plane <= 45.0 * pr;
                    if lay
                        && !edge
                        && !skip
                        && (plane > now || fluid_contact)
                        && plane - now <= wet_reach
                        && k.vol > 1e-9
                    {
                        let liquid = blade_paint + blade_solvent;
                        let film = (4.0 + 60.0 * (1.0 - pr) - (now - *height.add(i))).max(0.0);
                        let thickness = (plane - now).max(0.0) * (1.0 - flow) + film * flow;
                        let give = (thickness / COAT_UM * duty * (0.65 + noise))
                            .min((liquid / area as f64 * 0.12) as f32);
                        let phi = (blade_solvent / liquid) as f32;
                        let paint = give * (1.0 - phi);
                        let solvent = give * phi;
                        sf.add(i, paint, &k.lat, k.hide, k.cure);
                        if !sf.solv.is_null() {
                            *sf.solv.add(i) += solvent * COAT_UM;
                        }
                        *sf.cover.add(i) = 1.0;
                        blade_paint = (blade_paint - paint as f64 * area as f64).max(0.0);
                        blade_solvent = (blade_solvent - solvent as f64 * area as f64).max(0.0);
                        k.vol = blade_paint as f32;
                        k.solvent = blade_solvent as f32;
                    }
                    *sf.stroke.add(i) = id;
                }
                grow(
                    &mut bounds,
                    x as usize,
                    y as usize,
                    x as usize + 1,
                    y as usize + 1,
                );
            }
        }
        if lay && lift > 0.0 && k.vol > 1e-9 {
            let (c, dir, e) = last;
            let mut cells = Vec::new();
            for q in 0..=nx {
                let u = -1.0 + 2.0 * q as f32 / nx as f32;
                let noise = crate::surface::vnoise(u * 5.0, total * mm_px, seed ^ 0x11F7);
                for back in 0..3 {
                    let x = c.0 + e.0 * u * half + dir.0 * back as f32;
                    let y = c.1 + e.1 * u * half + dir.1 * back as f32;
                    if let Some(i) = pix(x, y) {
                        cells.push((i, noise * (1.0 - 0.22 * back as f32), x, y));
                    }
                }
            }
            cells.sort_by_key(|v| v.0);
            cells.dedup_by_key(|v| v.0);
            let sum: f32 = cells.iter().map(|v| v.1).sum();
            if sum > 0.0 {
                let liquid = blade_paint + blade_solvent;
                let leave = (liquid * lift.clamp(0.0, 1.0) as f64).min((sum * area * 18.0) as f64);
                let phi = (blade_solvent / liquid) as f32;
                for (i, weight, x, y) in cells {
                    let give = (leave * weight as f64 / sum as f64 / area as f64) as f32;
                    let paint = give * (1.0 - phi);
                    let solvent = give * phi;
                    unsafe {
                        sf.add(i, paint, &k.lat, k.hide, k.cure);
                        if !sf.solv.is_null() {
                            *sf.solv.add(i) += solvent * COAT_UM;
                        }
                        *sf.cover.add(i) = 1.0;
                        *sf.stroke.add(i) = id;
                    }
                    blade_paint = (blade_paint - paint as f64 * area as f64).max(0.0);
                    blade_solvent = (blade_solvent - solvent as f64 * area as f64).max(0.0);
                    grow(
                        &mut bounds,
                        x as usize,
                        y as usize,
                        x as usize + 1,
                        y as usize + 1,
                    );
                }
                k.vol = blade_paint as f32;
                k.solvent = blade_solvent as f32;
            }
        }
        if let Some((x0, y0, x1, y1)) = bounds {
            self.wet
                .touch(x0 - sf.ox, y0 - sf.oy, x1 - sf.ox, y1 - sf.oy);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canvas() -> Canvas {
        Canvas::new(240, 1.0, crate::hex("#ddd3bc"))
            .with_size_mm(300.0)
            .with_engine(5)
            .with_linen(crate::Linen::fine(18))
    }

    fn inventory(c: &Canvas, k: &Knife) -> (f64, f64) {
        (
            c.wet_total() + k.paint_volume(),
            c.solvent_total() + k.solvent_volume(),
        )
    }

    fn close(a: (f64, f64), b: (f64, f64)) {
        for (a, b) in [(a.0, b.0), (a.1, b.1)] {
            assert!((a - b).abs() < a.abs().max(1.0) * 3e-6, "{a} != {b}");
        }
    }

    #[test]
    fn knife_1_conserves_paint_and_solvent_across_lay_scrape_and_edge_clipping() {
        let mut c = canvas();
        let mut k = Knife::new(75.0).experimental(0.65);
        k.load(Paint::body(crate::hex("#bc4724")).with_thinner(0.4), 0.3);
        let before = inventory(&c, &k);
        for (pts, lay, tilt) in [
            (vec![(150.0, 200.0), (780.0, 300.0)], true, 15.0),
            (vec![(780.0, 300.0), (150.0, 200.0)], false, 80.0),
            (vec![(-20.0, 600.0), (1050.0, 600.0)], true, 40.0),
        ] {
            c.knife_1(&mut k, &pts, (0.7, 0.9), None, lay, 0.25, tilt);
            close(before, inventory(&c, &k));
            assert!(k.vol >= 0.0 && k.solvent >= 0.0);
            assert!(
                c.wet
                    .vol
                    .iter()
                    .chain(c.wet.solv.iter())
                    .all(|v| v.is_finite() && *v >= -1e-5)
            );
        }
    }

    #[test]
    fn knife_1_scrapes_open_paint_back_to_bare_support_and_clean_high_passes_miss_it() {
        let mut c = Canvas::new(200, 1.0, crate::hex("#d6cab5"))
            .with_size_mm(300.0)
            .with_engine(5);
        let mut k = Knife::new(70.0).experimental(0.65);
        k.load(Paint::body(crate::hex("#9a4724")), 0.7);
        let path = [(200.0, 300.0), (700.0, 300.0)];
        c.knife(&mut k, &path, (0.4, 0.4), None, true, 0.0);
        let paint = c.wet_total();
        k.wipe();
        let fields: Vec<_> = c
            .state_fields()
            .into_iter()
            .find(|f| f.name == "wet.vol")
            .unwrap()
            .values
            .le_bytes();
        c.knife_1(&mut k, &path, (0.0, 0.0), None, true, 0.0, 0.0);
        assert_eq!(
            fields,
            c.state_fields()
                .into_iter()
                .find(|f| f.name == "wet.vol")
                .unwrap()
                .values
                .le_bytes()
        );
        let before = inventory(&c, &k);
        c.knife_1(&mut k, &path, (1.0, 1.0), None, false, 0.0, 80.0);
        close(before, inventory(&c, &k));
        assert!(
            c.wet_total() < paint * 0.3,
            "a firm edge reaches bare support"
        );
    }

    #[test]
    fn knife_1_scrape_protects_set_color_height_and_film() {
        let mut c = canvas();
        let mut k = Knife::new(80.0).experimental(0.65);
        k.load(Paint::body(crate::hex("#214366")), 0.3);
        let path = [(150.0, 300.0), (800.0, 300.0)];
        c.knife(&mut k, &path, (0.3, 0.3), None, true, 0.1);
        c.dry();
        let dry = (c.px.clone(), c.height.clone(), c.film.clone());
        k.wipe();
        c.knife(&mut k, &path, (1.0, 1.0), None, false, 0.0);
        assert_eq!(dry, (c.px.clone(), c.height.clone(), c.film.clone()));
        assert_eq!(k.vol, 0.0);
        k.load(Paint::body(crate::hex("#ecbb41")), 0.3);
        c.knife(&mut k, &path, (0.65, 0.65), None, true, 0.0);
        assert!(c.wet_total() > 0.0);
        k.wipe();
        let before = inventory(&c, &k);
        c.knife(&mut k, &path, (1.0, 1.0), None, false, 0.0);
        close(before, inventory(&c, &k));
        assert_eq!(dry, (c.px.clone(), c.height.clone(), c.film.clone()));
        assert!(k.vol > 0.0);
    }

    #[test]
    fn knife_1_wet_pickup_dirties_blade_and_carries_color_into_next_mark() {
        let mut c = canvas();
        let mut base = Knife::new(100.0).experimental(0.65);
        base.load(Paint::body(crate::hex("#173e94")), 0.5);
        c.knife(
            &mut base,
            &[(180.0, 300.0), (480.0, 300.0)],
            (0.2, 0.2),
            None,
            true,
            0.1,
        );
        let mut k = Knife::new(65.0).experimental(0.65);
        k.load(Paint::body(crate::hex("#ecc54a")), 0.04);
        let color = k.paint().unwrap().color;
        let before = inventory(&c, &k);
        c.knife_1(
            &mut k,
            &[(200.0, 300.0), (800.0, 300.0)],
            (0.85, 0.65),
            None,
            true,
            0.05,
            30.0,
        );
        close(before, inventory(&c, &k));
        assert_ne!(k.paint().unwrap().color, color);
        assert!(c.wet_um(650.0, 300.0) > 0.0);
    }

    #[test]
    fn knife_1_fluid_veils_wet_tooth_without_a_body_slab() {
        let mark = |stiff| {
            let mut c = Canvas::new(640, 1.0, crate::hex("#ddd3bc"))
                .with_size_mm(300.0)
                .with_engine(5)
                .with_linen(crate::Linen::fine(18));
            let mut k = Knife::new(80.0).experimental(0.65);
            k.load(
                Paint::new(crate::hex("#ac8f32"), 0.3, stiff).with_thinner(0.35),
                0.5,
            );
            let before = inventory(&c, &k);
            c.knife_1(
                &mut k,
                &[(200.0, 300.0), (700.0, 300.0)],
                (0.75, 0.75),
                None,
                true,
                0.0,
                10.0,
            );
            close(before, inventory(&c, &k));
            c.wet.vol.iter().filter(|v| **v > 0.0).count()
        };
        let fluid = mark(0.1);
        let body = mark(0.9);
        assert!(
            fluid > body,
            "fluid paint wets more of the tooth: fluid={fluid}, body={body}"
        );
    }

    #[test]
    fn knife_1_pressure_tilt_and_flex_change_contact_deterministically() {
        let mark = |pressure, angle, tilt, flex| {
            let mut c = canvas();
            let mut k = Knife::new(80.0).experimental(flex);
            k.load(Paint::body(crate::hex("#cc5331")), 0.4);
            c.knife_1(
                &mut k,
                &[(180.0, 400.0), (700.0, 400.0)],
                (pressure, pressure),
                angle,
                true,
                0.05,
                tilt,
            );
            let fields: Vec<_> = c
                .state_fields()
                .into_iter()
                .map(|f| f.values.le_bytes())
                .collect();
            (c.wet_total(), c.wet.vol.clone(), fields, format!("{k:?}"))
        };
        let flat = mark(0.3, None, 15.0, 0.65);
        assert_eq!(flat, mark(0.3, None, 15.0, 0.65));
        let veil = mark(0.9, None, 15.0, 0.65);
        assert!(veil.0 < flat.0 * 0.5, "firm pressure thins the slab");
        let edge = mark(0.3, Some(0.0), 80.0, 0.65);
        assert!(
            edge.1.iter().filter(|v| **v > 0.0).count()
                < flat.1.iter().filter(|v| **v > 0.0).count() / 2
        );
        assert_ne!(flat.2, mark(0.3, None, 15.0, 0.0).2);
        assert_ne!(flat.2, mark(0.3, None, 80.0, 0.65).2);
    }

    #[test]
    fn knife_1_invalid_calls_are_atomic_and_spent_blades_remain_finite() {
        let mut c = canvas();
        let mut k = Knife::new(70.0).experimental(0.65);
        k.load(Paint::body(crate::hex("#a44424")), 0.001);
        let initial = c.wet_total() + k.paint_volume();
        c.knife_1(
            &mut k,
            &[(100.0, 300.0), (900.0, 300.0)],
            (0.2, 0.2),
            None,
            true,
            0.0,
            10.0,
        );
        assert!((k.vol as f64) < 0.01 * initial);
        assert!(c.wet.vol.iter().all(|v| v.is_finite()));
        let fields: Vec<_> = c
            .state_fields()
            .into_iter()
            .map(|f| f.values.le_bytes())
            .collect();
        let blade = format!("{k:?}");
        for (pts, tilt) in [
            (vec![(f32::NAN, 0.0)], 20.0),
            (vec![(100.0, 300.0)], f32::NAN),
            (vec![(100.0, 300.0)], 91.0),
        ] {
            c.knife_1(&mut k, &pts, (0.5, 0.5), None, true, 0.1, tilt);
            assert_eq!(
                fields,
                c.state_fields()
                    .into_iter()
                    .map(|f| f.values.le_bytes())
                    .collect::<Vec<_>>()
            );
            assert_eq!(blade, format!("{k:?}"));
        }
    }
}
