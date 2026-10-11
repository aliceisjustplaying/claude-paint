use super::*;

#[cfg(feature = "knife-2")]
#[derive(Clone, Debug)]
pub(super) struct Face {
    paint: f64,
    solvent: f64,
    lat: Latent,
    hide: Prop,
    cure: f32,
}

#[cfg(feature = "knife-2")]
impl Face {
    fn bind(&self, k: &mut Knife) -> (f64, f64) {
        k.vol = self.paint as f32;
        k.solvent = self.solvent as f32;
        k.lat = self.lat;
        k.hide = self.hide;
        k.cure = self.cure;
        (self.paint, self.solvent)
    }

    fn store(&mut self, k: &Knife, paint: f64, solvent: f64) {
        self.paint = paint;
        self.solvent = solvent;
        self.lat = k.lat;
        self.hide = k.hide;
        self.cure = k.cure;
    }
}

impl Knife {
    #[cfg(feature = "knife-3")]
    pub fn experimental_3(mut self, flex: f32) -> Self {
        self = self.experimental_2(flex);
        self.third = self.flex.is_some();
        self
    }
    #[cfg(feature = "knife-2")]
    pub fn experimental_2(mut self, flex: f32) -> Self {
        self = self.experimental(flex);
        self.second = self.flex.is_some();
        self
    }
    pub fn experimental(mut self, flex: f32) -> Self {
        if flex.is_finite() && (0.0..=1.0).contains(&flex) {
            self.flex = Some(flex);
            #[cfg(feature = "knife-3")]
            {
                self.third = false;
            }
            #[cfg(feature = "knife-2")]
            {
                self.second = false;
                self.faces = None;
            }
        }
        self
    }

    pub fn is_experimental(&self) -> bool {
        self.flex.is_some()
    }

    pub fn paint_volume(&self) -> f64 {
        #[cfg(feature = "knife-2")]
        if let Some(faces) = &self.faces {
            return faces.iter().map(|f| f.paint).sum();
        }
        self.vol as f64
    }

    pub fn solvent_volume(&self) -> f64 {
        #[cfg(feature = "knife-2")]
        if let Some(faces) = &self.faces {
            return faces.iter().map(|f| f.solvent).sum();
        }
        self.solvent as f64
    }

    #[cfg(feature = "knife-2")]
    fn make_faces(&self) -> Box<[Face; 8]> {
        Box::new(std::array::from_fn(|_| Face {
            paint: self.vol as f64 / 8.0,
            solvent: self.solvent as f64 / 8.0,
            lat: self.lat,
            hide: self.hide,
            cure: self.cure,
        }))
    }

    #[cfg(feature = "knife-2")]
    pub(super) fn load_faces(&mut self, paint: Paint, amount: f32) {
        let mut faces = self.faces.take().unwrap_or_else(|| self.make_faces());
        let liquid = amount * self.full();
        let volume = (liquid * (1.0 - paint.thinner)) as f64 / 8.0;
        let solvent = (liquid * paint.thinner) as f64 / 8.0;
        let lat = paint.latent();
        let hide = paint.prop();
        for face in faces.iter_mut() {
            if volume > 0.0 {
                let a = (volume / (face.paint + volume)) as f32;
                for j in 0..LAT {
                    face.lat[j] += (lat[j] - face.lat[j]) * a;
                }
                for j in 0..5 {
                    face.hide[j] += (hide[j] - face.hide[j]) * a;
                }
                face.cure *= 1.0 - a;
                face.paint += volume;
            }
            face.solvent += solvent;
        }
        self.faces = Some(faces);
        self.project_faces();
    }

    #[cfg(feature = "knife-2")]
    fn project_faces(&mut self) {
        let Some(faces) = &self.faces else { return };
        let total: f64 = faces.iter().map(|f| f.paint).sum();
        self.vol = total as f32;
        self.solvent = faces.iter().map(|f| f.solvent).sum::<f64>() as f32;
        if total > 0.0 {
            for j in 0..LAT {
                self.lat[j] =
                    (faces.iter().map(|f| f.paint * f.lat[j] as f64).sum::<f64>() / total) as f32;
            }
            for j in 0..5 {
                self.hide[j] = (faces
                    .iter()
                    .map(|f| f.paint * f.hide[j] as f64)
                    .sum::<f64>()
                    / total) as f32;
            }
            self.cure = (faces.iter().map(|f| f.paint * f.cure as f64).sum::<f64>() / total) as f32;
        }
        if total == 0.0 && faces.iter().all(|f| f.solvent == 0.0) {
            self.faces = None;
        }
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
        #[cfg(not(feature = "knife-2"))]
        let second = false;
        #[cfg(feature = "knife-2")]
        let second = k.second;
        #[cfg(not(feature = "knife-3"))]
        let third = false;
        #[cfg(feature = "knife-3")]
        let third = k.third;
        let pressure = if second {
            (pressure.0.clamp(0.0, 1.0), pressure.1.clamp(0.0, 1.0))
        } else {
            pressure
        };
        #[cfg(feature = "knife-2")]
        if second && k.faces.is_none() && (!lay || k.hide[1] > 0.35) {
            k.faces = Some(k.make_faces());
        }
        let body_pass = second && lay && k.hide[1] > 0.35 && tilt < 65.0;
        let bed = if body_pass || third && !lay {
            Some(crate::surface::box_blur(
                &self.height,
                self.f.w,
                self.f.h,
                (1.2 / self.px_mm()).ceil().max(1.0) as usize,
            ))
        } else {
            None
        };
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
        #[cfg(feature = "knife-2")]
        let mut faces = k.faces.take();
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
            let pr = if third && (body_pass || !lay) {
                (pr + 0.25
                    * flex
                    * (crate::surface::vnoise(d * mm_px * 0.065, 7.0, seed ^ 0xF1E7) - 0.5))
                    .clamp(0.0, 1.0)
            } else {
                pr
            };
            let gap = 280.0 * (1.0 - pr).powf(1.6) * (1.0 - 0.7 * upright);
            let mut cells = Vec::new();
            let peel_span = (12.0 / mm_px).min(total * 0.25).max(1.0);
            let peeling = if body_pass && total > 0.0 {
                smoothstep(total - peel_span, total, d)
            } else {
                0.0
            };
            for q in 0..=nx {
                let u = -1.0 + 2.0 * q as f32 / nx as f32;
                let drift = 0.7 * crate::surface::vnoise(u * 3.0, d * mm_px * 0.6, seed) - 0.35;
                let parting = depth
                    * 0.65
                    * (crate::surface::vnoise(u * 6.0 + 2.0, d * mm_px * 0.2, seed ^ 0xED6E) - 0.5);
                let retreat = (6.0 + 6.0 * lift.clamp(0.0, 1.0)) / mm_px
                    * (0.25
                        + 0.75
                            * crate::surface::vnoise(
                                u * 4.0 + 4.0,
                                total * mm_px * 0.15,
                                seed ^ 0x1AF7,
                            ))
                    * peeling;
                for r in 0..=ny {
                    let back = depth * r as f32 / ny as f32 + parting;
                    let x = c.0 + e.0 * (u * half + drift) + face.0 * back - dir.0 * retreat;
                    let y = c.1 + e.1 * (u * half + drift) + face.1 * back - dir.1 * retreat;
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
            let support_height = bed.as_ref().map_or(height, |v| v.as_ptr());
            let rigid = cells
                .iter()
                .map(|v| unsafe { *support_height.add(v.0) })
                .fold(f32::MIN, f32::max);
            let mut support = vec![f32::MIN; nx + 1];
            for cell in &cells {
                support[cell.4] = support[cell.4].max(unsafe { *support_height.add(cell.0) });
            }
            let radius = (((1.0 + 5.0 * (1.0 - flex)) / (k.width * self.mm_per_unit)) * nx as f32)
                .ceil()
                .max(1.0) as usize;
            let local_pressure: Vec<_> = (0..=nx)
                .map(|q| {
                    if third && (body_pass || !lay) {
                        let transverse = (-1.0 + 2.0 * q as f32 / nx as f32) * half * mm_px;
                        let bend = crate::surface::vnoise(
                            transverse * 0.13,
                            d * mm_px * 0.025,
                            seed ^ 0xB1ADE3,
                        ) - 0.5;
                        let drag = crate::surface::vnoise(
                            transverse * 0.65,
                            d * mm_px * 0.035,
                            seed ^ 0xD2A6,
                        ) - 0.5;
                        (pr + flex * pr.sqrt() * (0.75 * bend + 0.5 * smoothstep(0.05, 0.25, drag)))
                            .clamp(0.0, 1.0)
                    } else {
                        pr
                    }
                })
                .collect();
            let rest: Vec<_> = (0..=nx)
                .map(|q| {
                    let local = support[q.saturating_sub(radius)..(q + radius + 1).min(nx + 1)]
                        .iter()
                        .copied()
                        .fold(f32::MIN, f32::max);
                    rigid + (local - rigid) * flex * local_pressure[q].sqrt()
                })
                .collect();
            for &(i, u, x, y, q) in &cells {
                #[cfg(feature = "knife-2")]
                let face_id = (((u + 1.0) * 4.0).floor() as usize).min(7);
                #[cfg(feature = "knife-2")]
                if let Some(faces) = &faces {
                    (blade_paint, blade_solvent) = faces[face_id].bind(k);
                }
                let rest = rest[q];
                let pr = local_pressure[q];
                let gap = if third && (body_pass || !lay) {
                    280.0 * (1.0 - pr).powf(1.6) * (1.0 - 0.7 * upright)
                } else {
                    gap
                };
                let noise = crate::surface::vnoise(u * 4.0 + 9.0, d * mm_px * 0.8, seed) * 0.75
                    + crate::surface::vnoise(u * 13.0, d * mm_px * 2.1, seed ^ 41) * 0.25;
                let scrape_edge = if third && !lay {
                    let transverse = u * half * mm_px;
                    let wander =
                        1.2 * (crate::surface::vnoise(d * mm_px * 0.045, 5.0, seed ^ 0x5C2A) - 0.5);
                    let warp = transverse
                        + wander
                        + 2.0
                            * (crate::surface::vnoise(
                                transverse * 0.09,
                                d * mm_px * 0.018,
                                seed ^ 0xB022,
                            ) - 0.5);
                    crate::surface::vnoise(warp * 0.43, d * mm_px * 0.012, seed ^ 0xED6E3)
                } else {
                    0.0
                };
                let penetration = 85.0 * pr * pr * (0.25 + 0.75 * upright) * (0.3 + 0.7 * flex);
                let plane = rest + gap * (0.7 + 0.6 * noise) - penetration;
                let plane = if third && !lay {
                    plane + (45.0 + 105.0 * flex) * smoothstep(0.35, 0.85, scrape_edge)
                } else {
                    plane
                };
                let reach = (2.0 * gap + 45.0 * pr) * (0.45 + noise);
                unsafe {
                    let v = *sf.vol.add(i);
                    let sol = if sf.solv.is_null() {
                        0.0
                    } else {
                        *sf.solv.add(i) / COAT_UM
                    };
                    let top = *support_height.add(i) + (v + sol) * COAT_UM;
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
                    let now = *support_height.add(i)
                        + (*sf.vol.add(i)
                            + if sf.solv.is_null() {
                                0.0
                            } else {
                                *sf.solv.add(i) / COAT_UM
                            })
                            * COAT_UM;
                    let edge = u.abs() > 0.8 && noise < (u.abs() - 0.8) * 3.5;
                    let fill = k.fullness();
                    #[cfg(feature = "knife-2")]
                    let fill = if faces.is_some() { fill * 8.0 } else { fill };
                    let spent = (fill / 0.12).min(1.0);
                    let skip = if body_pass {
                        let island =
                            crate::surface::vnoise(u * 3.5 + 5.0, d * mm_px * 0.18, seed ^ 0xC0A7);
                        let contact = 0.75 * island + 0.25 * noise;
                        contact < 0.54 * (1.0 - smoothstep(0.04, 0.5, fill))
                    } else {
                        noise < (1.0 - spent) * 0.33
                    };
                    let peel_tear = body_pass
                        && crate::surface::vnoise(u * 8.0 + 2.0, d * mm_px * 0.36, seed ^ 0x7EAD)
                            < 0.48 * peeling;
                    let flow = 1.0 - smoothstep(0.15, 0.35, k.hide[1]);
                    let wet_reach = reach + 220.0 * flow * pr;
                    let fluid_contact = flow > 0.0 && now - plane <= 45.0 * pr;
                    let adhesion = if body_pass {
                        (6.0 + 35.0 * (1.0 - pr)) * smoothstep(0.0, 0.25, fill)
                    } else {
                        0.0
                    };
                    let body_target = if body_pass {
                        (plane - *support_height.add(i)).max(adhesion)
                    } else {
                        0.0
                    };
                    let body_coating = (body_target - (now - *support_height.add(i))).max(0.0);
                    if lay
                        && !edge
                        && !skip
                        && !peel_tear
                        && (plane > now || fluid_contact || body_coating > 0.0)
                        && plane - now <= wet_reach + adhesion
                        && k.vol > 1e-9
                    {
                        let liquid = blade_paint + blade_solvent;
                        let film = (4.0 + 60.0 * (1.0 - pr) - (now - *height.add(i))).max(0.0);
                        let thickness = if body_pass {
                            body_coating
                        } else {
                            (plane - now).max(0.0) * (1.0 - flow) + film * flow
                        };
                        let give = (thickness / COAT_UM * duty * (0.65 + noise))
                            .min((liquid / area as f64 * 0.12) as f32);
                        let phi = (blade_solvent / liquid) as f32;
                        let paint = give * (1.0 - phi);
                        let solvent = give * phi;
                        let lip_wander = if third {
                            0.6 * crate::surface::vnoise(
                                d * mm_px * 0.12,
                                u.signum() + 4.0,
                                seed ^ 0x11F3,
                            )
                        } else {
                            0.0
                        };
                        let side_start = if third { 0.6 } else { 0.7 };
                        let side = if body_pass && u.abs() > side_start {
                            let offset = if third {
                                u.signum() * (half + (0.5 + lip_wander) / mm_px) - u * half
                            } else {
                                u.signum() * (half * 1.03 + 0.3 * noise / mm_px) - u * half
                            };
                            pix(x + e.0 * offset, y + e.1 * offset)
                        } else {
                            None
                        };
                        let side_noise = if body_pass {
                            crate::surface::vnoise(
                                d * mm_px * 0.22,
                                u.signum() + 4.0,
                                seed ^ 0x51DE,
                            )
                        } else {
                            1.0
                        };
                        let side_inner = if side.is_some() {
                            let offset = if third {
                                u.signum() * (half + (-0.9 + lip_wander) / mm_px) - u * half
                            } else {
                                u.signum() * (half * 1.03 + (0.3 * noise - 0.55) / mm_px) - u * half
                            };
                            pix(x + e.0 * offset, y + e.1 * offset)
                        } else {
                            None
                        };
                        let side_middle = if third && side.is_some() {
                            let offset =
                                u.signum() * (half + (0.05 + lip_wander) / mm_px) - u * half;
                            pix(x + e.0 * offset, y + e.1 * offset)
                        } else {
                            None
                        };
                        let share = if side.is_some() {
                            ((u.abs() - side_start) / (1.0 - side_start)).clamp(0.0, 1.0)
                                * if third { 0.9 } else { 0.7 }
                                * (0.35 + 0.65 * side_noise)
                        } else {
                            0.0
                        };
                        let side_paint = paint * share;
                        let side_solvent = solvent * share;
                        let face_paint = paint - side_paint;
                        let face_solvent = solvent - side_solvent;
                        sf.add(i, face_paint, &k.lat, k.hide, k.cure);
                        if !sf.solv.is_null() {
                            *sf.solv.add(i) += face_solvent * COAT_UM;
                        }
                        *sf.cover.add(i) = 1.0;
                        if let Some(j) = side {
                            let inner = side_inner.unwrap_or(j);
                            let lip = if third {
                                [(inner, 0.5), (side_middle.unwrap_or(j), 0.35), (j, 0.15)]
                            } else {
                                [(inner, 0.65), (j, 0.35), (j, 0.0)]
                            };
                            for (j, fraction) in lip {
                                if fraction == 0.0 {
                                    continue;
                                }
                                sf.add(j, side_paint * fraction, &k.lat, k.hide, k.cure);
                                if !sf.solv.is_null() {
                                    *sf.solv.add(j) += side_solvent * fraction * COAT_UM;
                                }
                                *sf.cover.add(j) = 1.0;
                                *sf.stroke.add(j) = id;
                                let rx = sf.ox + j % sf.w;
                                let ry = sf.oy + j / sf.w;
                                grow(&mut bounds, rx, ry, rx + 1, ry + 1);
                            }
                        }
                        blade_paint = (blade_paint
                            - (face_paint as f64 + side_paint as f64) * area as f64)
                            .max(0.0);
                        blade_solvent = (blade_solvent
                            - (face_solvent as f64 + side_solvent as f64) * area as f64)
                            .max(0.0);
                        k.vol = blade_paint as f32;
                        k.solvent = blade_solvent as f32;
                    }
                    *sf.stroke.add(i) = id;
                }
                #[cfg(feature = "knife-2")]
                if let Some(faces) = &mut faces {
                    faces[face_id].store(k, blade_paint, blade_solvent);
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
        #[cfg(feature = "knife-2")]
        if let Some(owned) = faces.take() {
            k.faces = Some(owned);
            k.project_faces();
            faces = k.faces.take();
        }
        if lay && lift > 0.0 && k.vol > 1e-9 {
            let (c, dir, e) = last;
            let mut cells = Vec::new();
            for q in 0..=nx {
                let u = -1.0 + 2.0 * q as f32 / nx as f32;
                let noise = crate::surface::vnoise(u * 5.0, total * mm_px, seed ^ 0x11F7);
                let retreat = if body_pass {
                    (6.0 + 6.0 * lift.clamp(0.0, 1.0)) / mm_px
                        * (0.25
                            + 0.75
                                * crate::surface::vnoise(
                                    u * 4.0 + 4.0,
                                    total * mm_px * 0.15,
                                    seed ^ 0x1AF7,
                                ))
                } else {
                    0.0
                };
                let rows = if body_pass {
                    ((if third { 3.8 } else { 2.5 }) / mm_px).ceil().max(3.0) as usize
                } else {
                    3
                };
                for row in 0..rows {
                    let t = row as f32 / (rows - 1) as f32;
                    let back = if body_pass {
                        (if third {
                            1.8 + 2.0 * noise
                        } else {
                            1.0 + 1.7 * noise
                        }) / mm_px
                            * (t - 0.45)
                    } else {
                        row as f32
                    };
                    let weight = if body_pass {
                        (0.15 + 0.85 * noise)
                            * (std::f32::consts::PI * t).sin().max(0.0)
                            * smoothstep(
                                0.25,
                                0.6,
                                crate::surface::vnoise(
                                    u * 8.0 + 2.0,
                                    total * mm_px * 0.36,
                                    seed ^ 0x7EAD,
                                ),
                            )
                    } else {
                        noise * (1.0 - 0.22 * row as f32)
                    };
                    let x = c.0 + e.0 * u * half + dir.0 * (back - retreat);
                    let y = c.1 + e.1 * u * half + dir.1 * (back - retreat);
                    if let Some(i) = pix(x, y) {
                        cells.push((i, weight, x, y, (((u + 1.0) * 4.0).floor() as usize).min(7)));
                    }
                }
            }
            cells.sort_by_key(|v| v.0);
            cells.dedup_by_key(|v| v.0);
            let groups = 1;
            #[cfg(feature = "knife-2")]
            let groups = if faces.is_some() { 8 } else { groups };
            for group in 0..groups {
                #[cfg(feature = "knife-2")]
                if let Some(faces) = &faces {
                    (blade_paint, blade_solvent) = faces[group].bind(k);
                }
                let portion: Vec<_> = cells
                    .iter()
                    .copied()
                    .filter(|v| groups == 1 || v.4 == group)
                    .collect();
                let sum: f32 = portion.iter().map(|v| v.1).sum();
                if sum > 0.0 && blade_paint + blade_solvent > 1e-12 {
                    let liquid = blade_paint + blade_solvent;
                    let peel_coats = if body_pass { 44.0 } else { 18.0 };
                    let leave = (liquid * lift.clamp(0.0, 1.0) as f64)
                        .min((sum * area * peel_coats) as f64);
                    let phi = (blade_solvent / liquid) as f32;
                    for (i, weight, x, y, _) in portion {
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
                #[cfg(feature = "knife-2")]
                if let Some(faces) = &mut faces {
                    faces[group].store(k, blade_paint, blade_solvent);
                }
            }
        }
        #[cfg(feature = "knife-2")]
        if faces.is_some() {
            k.faces = faces;
            k.project_faces();
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

#[cfg(all(test, feature = "knife-2"))]
mod round_two_tests {
    use super::*;

    fn fixture() -> Canvas {
        let mut c = Canvas::new(640, 2.0, crate::hex("#d6cebb"))
            .with_size_mm(300.0)
            .with_engine(5)
            .with_linen(crate::Linen::fine(23));
        c.prime(crate::hex("#e2d7bf"), 0.9, 65.0, 0.6, 0.35, 17);
        c
    }

    fn close(c: &Canvas, k: &Knife, before: (f64, f64)) {
        for (a, b) in [
            (c.wet_total() + k.paint_volume(), before.0),
            (c.solvent_total() + k.solvent_volume(), before.1),
        ] {
            assert!((a - b).abs() < b.abs().max(1.0) * 3e-6, "{a} != {b}");
        }
    }

    #[test]
    fn knife_2_spent_body_loses_thread_scale_grid_without_losing_paint() {
        let mark = |second| {
            let mut c = fixture();
            let mut k = Knife::new(110.0);
            k = if second {
                k.experimental_2(0.65)
            } else {
                k.experimental(0.65)
            };
            k.load(Paint::body(crate::hex("#d8b757")), 0.15);
            let before = (
                c.wet_total() + k.paint_volume(),
                c.solvent_total() + k.solvent_volume(),
            );
            c.knife_1(
                &mut k,
                &[(190.0, 250.0), (790.0, 250.0)],
                (0.65, 0.8),
                None,
                true,
                0.0,
                25.0,
            );
            close(&c, &k, before);
            let mut variation = 0.0;
            let mut volume = 0.0;
            let s = c.f.scale;
            for y in (225.0 * s) as usize..(275.0 * s) as usize {
                for x in (400.0 * s) as usize..(760.0 * s) as usize {
                    let i = y * c.f.w + x;
                    volume += c.wet.vol[i] as f64;
                    variation += (c.wet.vol[i] - c.wet.vol[i + 1]).abs() as f64
                        + (c.wet.vol[i] - c.wet.vol[i + c.f.w]).abs() as f64;
                }
            }
            assert!(volume > 0.0);
            variation / volume
        };
        let previous = mark(false);
        let current = mark(true);
        assert!(
            current < previous * 0.7,
            "thread-scale jumps: knife-1={previous}, knife-2={current}"
        );
    }

    #[test]
    fn knife_2_preserves_fluid_veil_exactly() {
        let mut initial = fixture();
        initial.prime(crate::hex("#294867"), 0.9, 100.0, 0.6, 0.08, 29);
        let mark = |second| {
            let mut c = initial.clone();
            let mut k = Knife::new(110.0);
            k = if second {
                k.experimental_2(0.65)
            } else {
                k.experimental(0.65)
            };
            k.load(
                Paint::new(crate::hex("#c8a647"), 0.25, 0.1).with_thinner(0.35),
                0.1,
            );
            c.knife_1(
                &mut k,
                &[(190.0, 250.0), (450.0, 248.0), (790.0, 250.0)],
                (0.75, 0.8),
                None,
                true,
                0.0,
                10.0,
            );
            (
                c.state_fields()
                    .into_iter()
                    .map(|f| f.values.le_bytes())
                    .collect::<Vec<_>>(),
                k.vol,
                k.solvent,
            )
        };
        assert_eq!(mark(false), mark(true));
    }

    #[test]
    fn knife_2_torn_end_and_side_piles_use_the_finite_bead() {
        let mut c = fixture();
        let mut k = Knife::new(110.0).experimental_2(0.65);
        k.load(Paint::body(crate::hex("#a89a7e")), 1.0);
        let before = (
            c.wet_total() + k.paint_volume(),
            c.solvent_total() + k.solvent_volume(),
        );
        c.knife_1(
            &mut k,
            &[(240.0, 250.0), (430.0, 250.0)],
            (0.15, 0.3),
            None,
            true,
            0.35,
            10.0,
        );
        close(&c, &k, before);
        let s = c.f.scale;
        let mut ends = Vec::new();
        for y in (225.0 * s) as usize..(275.0 * s) as usize {
            let end = (200..c.f.w)
                .rev()
                .find(|&x| c.wet.vol[y * c.f.w + x] > 0.1)
                .unwrap();
            ends.push(end);
        }
        let spread = ends.iter().max().unwrap() - ends.iter().min().unwrap();
        assert!(
            spread >= 4,
            "the terminal silhouette varies across the blade: {spread}px"
        );
        let mut face = 0.0;
        let mut count = 0.0;
        let mut side = 0.0f32;
        let mut end = 0.0f32;
        for y in (230.0 * s) as usize..(270.0 * s) as usize {
            for x in (330.0 * s) as usize..(380.0 * s) as usize {
                face += c.wet.vol[y * c.f.w + x];
                count += 1.0;
            }
        }
        for y in (305.0 * s) as usize..(315.0 * s) as usize {
            for x in (280.0 * s) as usize..(380.0 * s) as usize {
                side = side.max(c.wet.vol[y * c.f.w + x]);
            }
        }
        for y in (210.0 * s) as usize..(290.0 * s) as usize {
            for x in (390.0 * s) as usize..(440.0 * s) as usize {
                end = end.max(c.wet.vol[y * c.f.w + x]);
            }
        }
        let face = face / count;
        assert!(side > face * 1.5, "side={side}, face={face}");
        assert!(end > face * 2.0, "end={end}, face={face}");
        assert!(c.wet.vol.iter().copied().fold(0.0, f32::max) * COAT_UM < 2000.0);
        assert!(k.vol > 0.0, "unreleased bead stays on the blade");
    }

    #[test]
    fn knife_2_still_bridges_deep_set_valleys() {
        let mut c = fixture();
        let s = c.f.scale;
        for y in (240.0 * s) as usize..(260.0 * s) as usize {
            for x in 0..c.f.w {
                c.height[y * c.f.w + x] += 3000.0;
            }
        }
        let support = c.height.clone();
        let mut k = Knife::new(110.0).experimental_2(0.65);
        k.load(Paint::body(crate::hex("#bd7045")), 0.5);
        c.knife_1(
            &mut k,
            &[(190.0, 250.0), (790.0, 250.0)],
            (0.2, 0.2),
            None,
            true,
            0.0,
            20.0,
        );
        assert_eq!(support, c.height);
        assert!(c.wet_um(400.0, 250.0) > 0.0);
        assert_eq!(c.wet_um(400.0, 285.0), 0.0, "deep valleys stay untouched");
    }

    #[test]
    fn knife_2_transport_stays_finite_and_protects_set_layers() {
        let mut c = fixture();
        let dry = (c.px.clone(), c.height.clone(), c.film.clone());
        let mut k = Knife::new(110.0).experimental_2(0.65);
        k.load(Paint::body(crate::hex("#bd7045")).with_thinner(0.4), 0.4);
        let before = (
            c.wet_total() + k.paint_volume(),
            c.solvent_total() + k.solvent_volume(),
        );
        for (path, lay, tilt) in [
            (vec![(-20.0, 250.0), (1020.0, 250.0)], true, 20.0),
            (vec![(200.0, 250.0), (790.0, 250.0)], false, 75.0),
        ] {
            c.knife_1(&mut k, &path, (0.7, 0.9), None, lay, 0.2, tilt);
            close(&c, &k, before);
            assert_eq!(dry, (c.px.clone(), c.height.clone(), c.film.clone()));
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
    fn knife_2_dirty_faces_keep_pickup_local_across_passes_and_reload() {
        let mut c = Canvas::new(320, 1.0, crate::hex("#d6cebb"))
            .with_size_mm(300.0)
            .with_engine(5);
        let mut base = Knife::new(40.0);
        base.load(Paint::body(crate::hex("#164f9c")), 0.7);
        c.knife(
            &mut base,
            &[(150.0, 210.0), (500.0, 210.0)],
            (0.3, 0.3),
            None,
            true,
            0.0,
        );
        let yellow = Paint::body(crate::hex("#d8b757"));
        let original = yellow.latent();
        let mut k = Knife::new(110.0).experimental_2(0.65);
        k.load(yellow, 0.25);
        let before = (
            c.wet_total() + k.paint_volume(),
            c.solvent_total() + k.solvent_volume(),
        );
        c.knife_1(
            &mut k,
            &[(180.0, 250.0), (650.0, 250.0)],
            (0.75, 0.75),
            None,
            true,
            0.0,
            25.0,
        );
        close(&c, &k, before);
        let faces = k.faces.as_ref().unwrap();
        assert_ne!(faces[0].lat, original, "the contacted end picks up blue");
        assert_eq!(faces[7].lat, original, "the untouched end retains yellow");
        c.knife_1(
            &mut k,
            &[(180.0, 650.0), (650.0, 650.0)],
            (0.65, 0.65),
            None,
            true,
            0.15,
            20.0,
        );
        close(&c, &k, before);
        assert_ne!(
            k.faces.as_ref().unwrap()[0].lat,
            k.faces.as_ref().unwrap()[7].lat
        );
        let amount = 0.1;
        let paint = Paint::body(crate::hex("#bd7045")).with_thinner(0.4);
        let liquid = amount * k.full();
        k.load(paint, amount);
        close(
            &c,
            &k,
            (
                before.0 + (liquid * (1.0 - paint.thinner)) as f64,
                before.1 + (liquid * paint.thinner) as f64,
            ),
        );
        k.wipe();
        assert_eq!(k.paint_volume(), 0.0);
        assert_eq!(k.solvent_volume(), 0.0);
        assert!(k.faces.is_none());
    }

    #[test]
    fn knife_2_zero_load_is_material_identity_and_clean_scrapes_are_local() {
        let mut k = Knife::new(110.0).experimental_2(0.65);
        let before = format!("{k:?}");
        k.load(Paint::body(crate::hex("#d8b757")), 0.0);
        assert_eq!(before, format!("{k:?}"));
        let mut c = Canvas::new(240, 1.0, crate::hex("#d6cebb"))
            .with_size_mm(300.0)
            .with_engine(5);
        c.knife_1(
            &mut k,
            &[(180.0, 250.0), (650.0, 250.0)],
            (1.0, 1.0),
            None,
            false,
            0.0,
            75.0,
        );
        assert_eq!(
            before,
            format!("{k:?}"),
            "an empty scrape retains no phantom reservoirs"
        );
        let mut base = Knife::new(40.0);
        base.load(Paint::body(crate::hex("#164f9c")), 0.7);
        c.knife(
            &mut base,
            &[(150.0, 210.0), (500.0, 210.0)],
            (0.3, 0.3),
            None,
            true,
            0.0,
        );
        let inventory = (c.wet_total(), c.solvent_total());
        c.knife_1(
            &mut k,
            &[(180.0, 250.0), (650.0, 250.0)],
            (1.0, 1.0),
            None,
            false,
            0.0,
            75.0,
        );
        close(&c, &k, inventory);
        let faces = k.faces.as_ref().unwrap();
        assert!(faces[0].paint > 0.0);
        assert_eq!(faces[7].paint, 0.0);
    }

    #[test]
    fn knife_2_finite_extreme_pressure_is_clamped_before_interpolation() {
        let mut c = fixture();
        let mut k = Knife::new(110.0).experimental_2(0.65);
        k.load(Paint::body(crate::hex("#bd7045")), 0.3);
        let before = (
            c.wet_total() + k.paint_volume(),
            c.solvent_total() + k.solvent_volume(),
        );
        c.knife_1(
            &mut k,
            &[(190.0, 250.0), (790.0, 250.0)],
            (f32::MAX, -f32::MAX),
            None,
            true,
            0.2,
            20.0,
        );
        close(&c, &k, before);
        assert!(c.state_fields().into_iter().all(|f| match f.values {
            crate::state_dump::Values::F32(v) => v.iter().all(|v| v.is_finite()),
            _ => true,
        }));
    }
}

#[cfg(all(test, feature = "knife-3"))]
mod round_three_tests {
    use super::*;

    #[test]
    fn knife_3_compliant_pressure_breaks_a_flat_slab_into_coherent_thickness_changes() {
        let mark = |flex| {
            let mut c = Canvas::new(640, 2.0, crate::hex("#d6cebb"))
                .with_size_mm(300.0)
                .with_engine(5);
            let mut k = Knife::new(110.0).experimental_3(flex);
            k.load(Paint::body(crate::hex("#b3673e")), 0.9);
            c.knife_1(
                &mut k,
                &[(190.0, 250.0), (650.0, 250.0)],
                (0.45, 0.45),
                None,
                true,
                0.0,
                15.0,
            );
            let s = c.f.scale;
            let mut samples = Vec::new();
            for y in (220.0 * s) as usize..(280.0 * s) as usize {
                for x in (300.0 * s) as usize..(550.0 * s) as usize {
                    samples.push(c.wet.vol[y * c.f.w + x] as f64);
                }
            }
            let mean = samples.iter().sum::<f64>() / samples.len() as f64;
            let deviation = (samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
                / samples.len() as f64)
                .sqrt();
            deviation / mean
        };
        let rigid = mark(0.0);
        let compliant = mark(0.65);
        assert!(
            compliant > rigid * 1.3,
            "rigid={rigid}, compliant={compliant}"
        );
        assert!(
            compliant < 0.6,
            "the body retains coherent flats: {compliant}"
        );
    }

    #[test]
    fn knife_3_fluid_veil_is_exactly_the_round_two_path() {
        let initial = Canvas::new(320, 2.0, crate::hex("#35556a"))
            .with_size_mm(300.0)
            .with_engine(5)
            .with_linen(crate::Linen::fine(41));
        let mark = |third| {
            let mut c = initial.clone();
            let mut k = Knife::new(110.0);
            k = if third {
                k.experimental_3(0.65)
            } else {
                k.experimental_2(0.65)
            };
            k.load(
                Paint::new(crate::hex("#baa452"), 0.25, 0.1).with_thinner(0.35),
                0.15,
            );
            c.knife_1(
                &mut k,
                &[(170.0, 250.0), (650.0, 250.0)],
                (0.75, 0.82),
                None,
                true,
                0.0,
                10.0,
            );
            (
                c.state_fields()
                    .into_iter()
                    .map(|f| f.values.le_bytes())
                    .collect::<Vec<_>>(),
                k.paint_volume(),
                k.solvent_volume(),
            )
        };
        assert_eq!(mark(false), mark(true));
    }

    #[test]
    fn knife_3_contact_variation_conserves_faces_and_protects_set_layers_across_scales() {
        for (seed, width, pressure, tilt, flex) in [
            (7, 35.0, (0.0, 0.3), 10.0, 0.0),
            (23, 110.0, (0.35, 0.6), 20.0, 0.65),
            (41, 190.0, (0.8, 1.0), 45.0, 1.0),
            (59, 75.0, (f32::MAX, -f32::MAX), 80.0, 0.65),
        ] {
            let mut c = Canvas::new(320, 2.0, crate::hex("#d6cebb"))
                .with_size_mm(300.0)
                .with_engine(5)
                .with_linen(crate::Linen::fine(seed));
            let dry = (c.px.clone(), c.height.clone(), c.film.clone());
            let mut k = Knife::new(width).experimental_3(flex);
            k.load(Paint::body(crate::hex("#b3673e")).with_thinner(0.2), 0.8);
            let before = (
                c.wet_total() + k.paint_volume(),
                c.solvent_total() + k.solvent_volume(),
            );
            for (points, lay, tilt) in [
                (vec![(-30.0, 250.0), (1030.0, 250.0)], true, tilt),
                (vec![(190.0, 250.0), (790.0, 250.0)], false, 75.0),
                (vec![(190.0, 380.0), (790.0, 380.0)], true, 20.0),
            ] {
                c.knife_1(&mut k, &points, pressure, None, lay, 0.25, tilt);
                for (after, before) in [
                    (c.wet_total() + k.paint_volume(), before.0),
                    (c.solvent_total() + k.solvent_volume(), before.1),
                ] {
                    assert!(
                        (after - before).abs() < before.abs().max(1.0) * 3e-6,
                        "{after} != {before}"
                    );
                }
                assert_eq!(dry, (c.px.clone(), c.height.clone(), c.film.clone()));
                assert!(
                    c.wet
                        .vol
                        .iter()
                        .chain(c.wet.solv.iter())
                        .all(|v| v.is_finite() && *v >= -1e-5)
                );
            }
        }
    }

    #[test]
    fn knife_3_scrape_contacts_a_footprint_over_set_tooth_without_a_pixel_grid() {
        let mark = |third| {
            let mut c = Canvas::new(640, 2.0, crate::hex("#35556a"))
                .with_size_mm(300.0)
                .with_engine(5);
            let s = c.f.scale;
            let orange = Paint::body(crate::hex("#b3673e"));
            for y in 0..c.f.h {
                for x in 0..c.f.w {
                    c.height[y * c.f.w + x] = if (x / 2 + y / 2) % 2 == 0 {
                        220.0
                    } else {
                        340.0
                    };
                }
            }
            let sf = c.surf();
            for y in (200.0 * s) as usize..(300.0 * s) as usize {
                for x in (150.0 * s) as usize..(800.0 * s) as usize {
                    unsafe {
                        sf.add(y * sf.w + x, 4.0, &orange.latent(), orange.prop(), 0.0);
                    }
                }
            }
            let dry = (c.px.clone(), c.height.clone(), c.film.clone());
            let before = c.wet_total();
            let mut k = Knife::new(110.0);
            k = if third {
                k.experimental_3(0.65)
            } else {
                k.experimental_2(0.65)
            };
            c.knife_1(
                &mut k,
                &[(190.0, 250.0), (790.0, 250.0)],
                (1.0, 1.0),
                None,
                false,
                0.0,
                75.0,
            );
            assert_eq!(dry, (c.px.clone(), c.height.clone(), c.film.clone()));
            assert!((c.wet_total() + k.paint_volume() - before).abs() < before * 3e-6);
            let mut jumps = 0.0;
            for y in (230.0 * s) as usize..(270.0 * s) as usize {
                for x in (300.0 * s) as usize..(700.0 * s) as usize {
                    let i = y * c.f.w + x;
                    jumps += (c.wet.vol[i] - c.wet.vol[i + 1]).abs() as f64
                        + (c.wet.vol[i] - c.wet.vol[i + c.f.w]).abs() as f64;
                }
            }
            jumps
        };
        let previous = mark(false);
        let current = mark(true);
        assert!(
            previous > 100.0,
            "the old pixel cutoff must be exercised: {previous}"
        );
        assert!(
            current < previous * 0.35,
            "knife-2={previous}, knife-3={current}"
        );
    }
}
