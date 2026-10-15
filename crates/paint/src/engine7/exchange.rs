use std::cell::RefCell;

use crate::bristle::{
    Bounds, Bristle, Clip, Rect, Surf, Tool, WET_REACH, capsule_cover, grow, mix_cure,
};
use crate::smoothstep;
use crate::surface::COAT_UM;
use crate::wet::{LAT, Prop, mix_into};

use super::window::{Capsule, SmoothRange, Window};

thread_local! {
    static SCRATCH: RefCell<Window> = const { RefCell::new(Window::new()) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Canvas, Frame, Held, Mask, Paint};

    fn fixture() -> Canvas {
        let mut c = Canvas::new_window(96, 1.5, [0.5; 3], None).with_engine(7);
        c.f = Frame::new(96, 64, 2.0);
        c.base = Some((c.surf_gen, vec![0.6; 96 * 64]));
        c.wet.ensure_solvent();
        c
    }

    fn hair(tool: &Tool, volume: f32, solvent: f32) -> Bristle {
        let mut b = Held::new(tool.clone(), 219).bristles.remove(0);
        b.vol = volume;
        b.solvent = solvent;
        b.lat = Paint::body([0.8, 0.05, 0.03]).latent();
        b.hide = [1.2, 0.65, 1.0, 0.0, 1.0];
        b
    }

    fn wet_field(c: &mut Canvas) {
        let red = Paint::body([0.8, 0.05, 0.03]).latent();
        let blue = Paint::body([0.02, 0.1, 0.8]).latent();
        for i in 0..c.wet.vol.len() {
            c.wet.vol[i] = 0.2 + (i % 13) as f32 * 0.03;
            c.wet.lat[i] = if i % c.f.w < c.f.w / 2 { red } else { blue };
            c.wet.hide[i] = [0.8, 0.5, 1.0, 0.0, 1.0];
            c.wet.solv[i] = (i % 5) as f32 * 0.2;
        }
    }

    fn total(c: &Canvas, b: &Bristle) -> (f64, f64, [f64; LAT]) {
        let area = (1.0 / (c.f.scale * c.f.scale)) as f64;
        let mut paint = b.vol as f64;
        let mut solvent = b.solvent as f64;
        let mut pigment = b.lat.map(|l| l as f64 * b.vol as f64);
        for i in 0..c.wet.vol.len() {
            let v = c.wet.vol[i] as f64 * area;
            paint += v;
            solvent += c.wet.solv[i] as f64 / COAT_UM as f64 * area;
            for (p, l) in pigment.iter_mut().zip(c.wet.lat[i]) {
                *p += l as f64 * v;
            }
        }
        (paint, solvent, pigment)
    }

    fn near(a: f64, b: f64, tolerance: f64) {
        assert!(
            (a - b).abs() <= tolerance * a.abs().max(b.abs()).max(1.0),
            "{a} != {b}"
        );
    }

    fn nonnegative(c: &Canvas, b: &Bristle) {
        assert!(b.vol.is_finite() && b.vol >= 0.0);
        assert!(b.solvent.is_finite() && b.solvent >= 0.0);
        assert!(
            c.wet
                .vol
                .iter()
                .chain(&c.wet.solv)
                .all(|v| v.is_finite() && *v >= -1e-7)
        );
        assert!(c.wet.lat.iter().flatten().all(|v| v.is_finite()));
    }

    #[allow(clippy::too_many_arguments)]
    fn step(
        c: &mut Canvas,
        b: &mut Bristle,
        tool: &Tool,
        a: (f32, f32),
        end: (f32, f32),
        rb: f32,
        mask: Option<&Mask>,
        id: u32,
        window: &mut Window,
    ) -> AcceptedTransfer {
        let sf = c.surf();
        let mut bounds = None;
        let mut fallback = Vec::new();
        unsafe {
            run(
                SegmentInput {
                    surface: sf,
                    bristle: b,
                    tool,
                    start: a,
                    end,
                    radius: rb,
                    reach: 0.6,
                    full: 10.0,
                    deposit: None,
                    exclusion: 1.0,
                    clip: mask.map(Clip::Mask),
                    stroke_id: id,
                    limit: (0, 0, 96, 64),
                },
                &mut fallback,
                &mut bounds,
                window,
            )
            .expect("fixture must exercise optimized scalar path")
        }
    }

    #[test]
    fn overlapping_contacts_conserve_paint_pigment_and_solvent() {
        for (tool, solvent, tack) in [
            (Tool::filbert(8.0), 0.0, false),
            (Tool::filbert(8.0), 3.0, false),
            (Tool::badger(8.0), 0.0, true),
        ] {
            let mut c = fixture();
            wet_field(&mut c);
            if tack {
                c.wet.clock.px = vec![
                    crate::drying::Px {
                        cure: 0.06,
                        sub: 0.4,
                        ..crate::drying::Px::FRESH
                    };
                    c.wet.vol.len()
                ];
            }
            let mut b = hair(&tool, 6.0, solvent);
            let before = total(&c, &b);
            let mut window = Window::new();
            for id in 1..=160 {
                let x = 20.0 + (id % 35) as f32;
                let y = 20.0 + (id % 11) as f32 * 0.2;
                let old = (b.vol, b.solvent);
                let accepted = step(
                    &mut c,
                    &mut b,
                    &tool,
                    (x, y),
                    (x + 1.7, y + 0.3),
                    1.1,
                    None,
                    id,
                    &mut window,
                );
                near(
                    b.vol as f64,
                    (old.0 - accepted.laid.paint + accepted.picked.paint) as f64,
                    2e-6,
                );
                near(
                    b.solvent as f64,
                    (old.1 - accepted.laid.solvent + accepted.picked.solvent) as f64,
                    2e-6,
                );
                assert_eq!(accepted.outside.paint, 0.0);
                assert_eq!(accepted.outside.solvent, 0.0);
                nonnegative(&c, &b);
            }
            let after = total(&c, &b);
            near(before.0, after.0, 2e-6);
            near(before.1, after.1, 2e-6);
            for (a, b) in before.2.into_iter().zip(after.2) {
                near(a, b, 2e-6);
            }
        }
    }

    #[test]
    fn clean_blender_picks_up_then_pulls_paint_into_bare_canvas() {
        let mut c = fixture();
        let red = Paint::body([0.8, 0.05, 0.03]).latent();
        for y in 20..30 {
            for x in 20..27 {
                let i = y * 96 + x;
                c.wet.vol[i] = 1.0;
                c.wet.lat[i] = red;
            }
        }
        let tool = Tool::badger(8.0);
        let mut b = hair(&tool, 0.0, 0.0);
        let mut window = Window::new();
        let first = step(
            &mut c,
            &mut b,
            &tool,
            (23.0, 25.0),
            (24.5, 25.0),
            1.1,
            None,
            1,
            &mut window,
        );
        assert!(first.picked.paint > 0.0);
        assert_eq!(first.laid.paint, 0.0);
        for x in 25..42 {
            step(
                &mut c,
                &mut b,
                &tool,
                (x as f32 - 0.5, 25.0),
                (x as f32 + 1.0, 25.0),
                1.1,
                None,
                1,
                &mut window,
            );
        }
        let pulled: f32 = (20..30)
            .flat_map(|y| (30..40).map(move |x| y * 96 + x))
            .map(|i| c.wet.vol[i])
            .sum();
        assert!(
            pulled > 0.0,
            "a clean blender must carry wet paint beyond its original patch"
        );
        nonnegative(&c, &b);
    }

    #[test]
    fn two_color_wet_drag_changes_the_carried_pigment_in_order() {
        let mut c = fixture();
        wet_field(&mut c);
        let tool = Tool::filbert(8.0);
        let mut b = hair(&tool, 0.0, 0.0);
        let mut window = Window::new();
        for x in 35..47 {
            step(
                &mut c,
                &mut b,
                &tool,
                (x as f32, 25.0),
                (x as f32 + 1.0, 25.0),
                1.1,
                None,
                1,
                &mut window,
            );
        }
        let red_carried = b.lat;
        for x in 48..64 {
            step(
                &mut c,
                &mut b,
                &tool,
                (x as f32, 25.0),
                (x as f32 + 1.0, 25.0),
                1.1,
                None,
                1,
                &mut window,
            );
        }
        assert!(
            b.lat
                .iter()
                .zip(red_carried)
                .any(|(a, b)| (*a - b).abs() > 0.01)
        );
        nonnegative(&c, &b);
    }

    #[test]
    fn dry_scumble_skims_high_tooth_and_masks_stop_transfer() {
        let mut c = fixture();
        for (i, base) in c.base.as_mut().unwrap().1.iter_mut().enumerate() {
            *base = if i % 2 == 0 { 1.0 } else { 0.0 };
        }
        let mask = Mask::from_fn(c.f, |x, _| if x < 15.0 { 0.0 } else { 0.5 });
        let tool = Tool::hog_flat(8.0);
        let mut b = hair(&tool, 0.03, 0.0);
        let sf = c.surf();
        let mut window = Window::new();
        let mut bounds = None;
        unsafe {
            run(
                SegmentInput {
                    surface: sf,
                    bristle: &mut b,
                    tool: &tool,
                    start: (24.0, 25.0),
                    end: (38.0, 25.0),
                    radius: 1.1,
                    reach: 0.1,
                    full: 10.0,
                    deposit: None,
                    exclusion: 1.0,
                    clip: Some(Clip::Mask(&mask)),
                    stroke_id: 1,
                    limit: (0, 0, 96, 64),
                },
                &mut Vec::new(),
                &mut bounds,
                &mut window,
            )
            .unwrap();
        }
        assert!(c.wet.vol.iter().any(|v| *v > 0.0));
        for (i, v) in c.wet.vol.iter().enumerate() {
            if i % 2 == 1 || mask.data[i] == 0.0 {
                assert_eq!(*v, 0.0);
            }
        }
        nonnegative(&c, &b);
    }

    #[test]
    fn fine_hairline_and_zero_load_are_finite_and_conservative() {
        let mut c = fixture();
        let tool = Tool {
            point: 1.0,
            ..Tool::rigger(1.0)
        };
        let mut b = hair(&tool, 0.3, 0.0);
        let before = total(&c, &b);
        let mut window = Window::new();
        step(
            &mut c,
            &mut b,
            &tool,
            (20.1, 20.2),
            (26.4, 25.7),
            0.04,
            None,
            1,
            &mut window,
        );
        let after = total(&c, &b);
        near(before.0, after.0, 2e-6);
        assert!(c.wet.cover.iter().any(|v| *v > 0.0 && *v < 0.1));
        let mut empty = fixture();
        let mut clean = hair(&tool, 0.0, 0.0);
        let transfer = step(
            &mut empty,
            &mut clean,
            &tool,
            (20.1, 20.2),
            (26.4, 25.7),
            0.04,
            None,
            1,
            &mut window,
        );
        assert_eq!(transfer.laid.paint, 0.0);
        assert_eq!(transfer.picked.paint, 0.0);
        assert!(empty.wet.vol.iter().all(|v| *v == 0.0));
        nonnegative(&c, &b);
        nonnegative(&empty, &clean);
    }
}

pub(crate) struct SegmentInput<'a> {
    pub surface: Surf,
    pub bristle: &'a mut Bristle,
    pub tool: &'a Tool,
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub radius: f32,
    pub reach: f32,
    pub full: f32,
    pub deposit: Option<f32>,
    pub exclusion: f32,
    pub clip: Option<Clip<'a>>,
    pub stroke_id: u32,
    pub limit: Rect,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Contact {
    pub coverage: f32,
    pub weight: f32,
    pub tack: f32,
    pub fence_weight: f32,
}

/// Material amounts in coats times squared canvas units; solvent is not in micrometers.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Budget {
    pub paint: f32,
    pub solvent: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct AcceptedTransfer {
    pub picked: Budget,
    pub laid: Budget,
    pub outside: Budget,
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn exchange(
    sf: Surf,
    br: &mut Bristle,
    tool: &Tool,
    a: (f32, f32),
    b: (f32, f32),
    rb: f32,
    reach: f32,
    full: f32,
    dep: Option<f32>,
    excl: f32,
    clip: Option<Clip<'_>>,
    id: u32,
    wts: &mut Vec<f32>,
    bounds: &mut Bounds,
    lim: Rect,
) {
    SCRATCH.with_borrow_mut(|window| unsafe {
        run(
            SegmentInput {
                surface: sf,
                bristle: br,
                tool,
                start: a,
                end: b,
                radius: rb,
                reach,
                full,
                deposit: dep,
                exclusion: excl,
                clip,
                stroke_id: id,
                limit: lim,
            },
            wts,
            bounds,
            window,
        );
    });
}

pub(crate) unsafe fn run(
    input: SegmentInput<'_>,
    wts: &mut Vec<f32>,
    bounds: &mut Bounds,
    window: &mut Window,
) -> Option<AcceptedTransfer> {
    let SegmentInput {
        surface: sf,
        bristle: br,
        tool,
        start: a,
        end: b,
        radius: rb,
        reach,
        full,
        deposit: dep,
        exclusion: excl,
        clip,
        stroke_id: id,
        limit: lim,
    } = input;
    unsafe {
        let (w, h) = (sf.fw, sf.fh);
        let (ox, oy, bw_buf) = (sf.ox, sf.oy, sf.w);
        let s = sf.scale;
        let px_area = 1.0 / (s * s);
        let cx0 = ((a.0.min(b.0) - rb - 1.0).floor().max(0.0)) as usize;
        let cy0 = ((a.1.min(b.1) - rb - 1.0).floor().max(0.0)) as usize;
        let cx1 = ((a.0.max(b.0) + rb + 1.0).ceil().max(0.0) as usize).min(w);
        let cy1 = ((a.1.max(b.1) + rb + 1.0).ceil().max(0.0) as usize).min(h);
        if cx1 <= cx0 || cy1 <= cy0 {
            return Some(AcceptedTransfer::default());
        }
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        let (x0, y0, x1, y1) = (
            cx0.max(ox),
            cy0.max(oy),
            cx1.min(ox + sf.w),
            cy1.min(oy + sf.h),
        );
        let windowed = (x0, y0, x1, y1) != (cx0, cy0, cx1, cy1);
        if x1 <= x0 || y1 <= y0 {
            crate::bristle::exchange_legacy(
                sf, br, tool, a, b, rb, reach, full, dep, excl, clip, id, wts, bounds, lim,
            );
            return None;
        }
        debug_assert!(
            x0 >= lim.0 && y0 >= lim.1 && x1 <= lim.2 && y1 <= lim.3,
            "bristle contact ({x0},{y0},{x1},{y1}) outside the stroke footprint {lim:?}"
        );
        let (x0, y0, x1, y1) = (x0.max(lim.0), y0.max(lim.1), x1.min(lim.2), y1.min(lim.3));
        if x1 <= x0 || y1 <= y0 {
            return Some(AcceptedTransfer::default());
        }
        let (mx, my) = if seg > 1e-4 {
            (dx / seg, dy / seg)
        } else {
            (0.0, 0.0)
        };
        let (nx, ny) = (-my, mx);
        let fine = tool.point > 0.0 && dep.is_none();
        let liquid = if br.solvent > 0.0 {
            br.vol + br.solvent
        } else {
            br.vol
        };
        let phi = if br.solvent > 0.0 {
            br.solvent / liquid
        } else {
            0.0
        };
        let exch = phi > 0.0 && !sf.laid.is_null() && crate::thinner::exchange();
        let capped = phi > 0.0 && !sf.laid.is_null() && !exch;
        let cap = if capped {
            crate::thinner::stroke_limit_um(phi) / COAT_UM
        } else {
            f32::INFINITY
        };
        let wet = smoothstep(0.1, 0.8, liquid / full);
        let wick = if tool.point > 0.0 {
            tool.point * smoothstep(0.02, 0.25, liquid / full)
        } else {
            0.0
        }
        .max(WET_REACH * wet);
        let th = 1.0 - reach.max(wick) * 1.6;

        window.reset((x0, y0, x1, y1), lim);
        let capsule = Capsule::new(a, b, rb, fine, dep.is_some());
        let give = 0.15 + 0.45 * (1.0 - tool.stiffness).clamp(0.0, 1.0);
        let contact_range = SmoothRange::new(th - give, th + 0.2);
        let mut contact = Contact::default();
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let cov = capsule.coverage(px, py);
                if cov <= 0.0 {
                    continue;
                }
                contact.coverage += cov;
                let i = (y - oy) * bw_buf + x - ox;
                let surf = (*sf.base.add(i) + 0.35 * *sf.vol.add(i)).min(1.5);
                let wt = match clip {
                    None => cov * contact_range.at(surf),
                    Some(Clip::Mask(m)) => cov * contact_range.at(surf) * m.data[y * w + x],
                    Some(c @ Clip::Fence { .. }) => {
                        let (k, lift) = c.at(y * w + x);
                        let wt = cov * contact_range.at(surf - lift) * k;
                        contact.fence_weight += wt * k * k;
                        wt
                    }
                };
                if wt > 0.0 && !window.push(i, x, y, wt) {
                    crate::bristle::exchange_legacy(
                        sf, br, tool, a, b, rb, reach, full, dep, excl, clip, id, wts, bounds, lim,
                    );
                    return None;
                }
                contact.weight += wt;
                if !sf.dry.is_null() {
                    let p = *sf.dry.add(i);
                    contact.tack += wt * crate::drying::feel(*sf.vol.add(i), p.cure, p.sub).1;
                }
            }
        }
        if contact.weight <= 1e-6 {
            return Some(AcceptedTransfer::default());
        }
        let tack = contact.tack / contact.weight;

        let travel = (seg / s).max(rb / s * 0.5);
        let travel = if exch {
            travel * (1.0 - phi).powf(crate::thinner::exchange_kj().0)
        } else {
            travel
        };
        let touch = (contact.weight / contact.coverage.max(1e-6)).min(1.0);
        let dep_total = match dep {
            None => liquid * (1.0 - (-travel / tool.run).exp()) * touch,
            Some(v) => v.min(liquid * 0.5) * touch,
        };
        let dep_total = if matches!(clip, Some(Clip::Fence { .. })) {
            dep_total * (contact.fence_weight / contact.weight).min(1.0)
        } else {
            dep_total
        };
        let dep_total = if tack > 0.0 {
            let g = crate::drying::grab(tack) * crate::drying::stick(b.0, b.1, rb, br.seed, tack);
            let d = match dep {
                None => liquid * (1.0 - (-travel * g / tool.run).exp()) * touch,
                Some(_) => dep_total * g,
            };
            d.min(liquid * 0.9)
        } else {
            dep_total
        };
        let hair_radius = tool.hair_radius();
        let ps = (br.hide[1] * (1.0 - br.hide[3]).powi(2)).clamp(0.0, 1.0);
        let stiffen = ((ps - 0.4) / 0.6).clamp(0.0, 1.0);
        let dep_total = if sf.clump && dep.is_none() && !fine && stiffen > 0.0 {
            let clump_u = hair_radius * (1.0 + 4.0 * stiffen * stiffen);
            let bands = (tool.width / (2.0 * clump_u)).clamp(2.0, 48.0);
            let u = (br.rx.clamp(-1.0, 1.0) + 1.0) * 0.5 * bands;
            let (b0, f) = (u.floor(), u - u.floor());
            let h0 = crate::rng::hash2(b0 as i64, id as i64, 0x51C0);
            let h1 = crate::rng::hash2(b0 as i64 + 1, id as i64, 0x51C0);
            let band = h0 + (h1 - h0) * crate::smoothstep(0.0, 1.0, f);
            (dep_total * (1.0 + 0.6 * stiffen * (2.0 * band - 1.0))).min(liquid * 0.95)
        } else {
            dep_total
        };
        let share = if windowed {
            contact.coverage / capsule_cover(a, b, rb, fine, (cx0, cy0, cx1, cy1)).max(1e-6)
        } else {
            1.0
        };
        let dep_per_w = dep_total * share.min(1.0) / contact.weight / px_area;
        let empty = (1.0 - liquid / full).clamp(0.0, 1.0);
        let hunger = 0.35 + 0.65 * empty * empty.sqrt();
        let hair = if fine || dep.is_some() {
            rb
        } else {
            (hair_radius * s).min(rb)
        };
        let push_k = tool.push * (seg / (2.0 * hair)).clamp(0.0, 1.0) * (hair / rb);
        let off = if dep.is_some() {
            rb + 1.0
        } else if sf.clump && stiffen > 0.0 && !fine && br.rx.abs() > 0.7 {
            (2.0 * hair_radius * s * (1.0 + 4.0 * stiffen * stiffen))
                .max(2.0 * hair)
                .min(3.0)
        } else {
            2.0 * hair
        };
        let spread = dep.is_none();

        let mut got_v = 0.0f32;
        let mut got_l = [0.0f32; LAT];
        let mut got_h: Prop = [0.0; 5];
        let mut got_c = 0.0f32;
        let mut got_s = 0.0f32;
        let mut laid_liq = 0.0f32;
        let (blat, bhide, bcure) = (br.lat, br.hide, br.cure);
        for at in 0..window.len {
            let wt = window.weights[at];
            let (x, y, i) = (window.xs[at], window.ys[at], window.indices[at]);
            let v = *sf.vol.add(i);
            let fl = if sf.dry.is_null() {
                1.0
            } else {
                crate::drying::fluid((*sf.dry.add(i)).cure)
            };
            if *sf.touched.add(i) != id {
                *sf.touched.add(i) = id;
                *sf.floor.add(i) = v * (1.0 - tool.pickup * fl);
            }
            if v > 1e-6 {
                let take = if exch {
                    (v * tool.pickup * wt * hunger * fl).min(v)
                } else {
                    let own = if *sf.stroke.add(i) == id { 0.15 } else { 1.0 };
                    (v * tool.pickup * wt * hunger * own * fl).min((v - *sf.floor.add(i)).max(0.0))
                };
                if take > 0.0 {
                    let tv = take * px_area;
                    got_v += tv;
                    let l = &*sf.lat.add(i);
                    for k in 0..LAT {
                        got_l[k] += l[k] * tv;
                    }
                    let hp = *sf.hide.add(i);
                    for k in 0..hp.len() {
                        got_h[k] += hp[k] * tv;
                    }
                    if !sf.dry.is_null() {
                        got_c += (*sf.dry.add(i)).cure * tv;
                    }
                    if !sf.solv.is_null() {
                        let sv = &mut *sf.solv.add(i);
                        if *sv > 0.0 {
                            let ts = (*sv * take / v).min(*sv);
                            *sv -= ts;
                            got_s += ts / COAT_UM * px_area;
                        }
                    }
                    sf.take(i, take);
                }
            }
            if dep_per_w > 0.0 && !capped && phi <= 0.0 {
                let cv = &mut *sf.cover.add(i);
                *cv = if fine {
                    ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0)
                } else {
                    1.0
                };
                let d = dep_per_w * wt;
                sf.add(i, d, &blat, bhide, bcure);
                laid_liq += d * px_area;
                *sf.stroke.add(i) = id;
            } else if dep_per_w > 0.0 {
                let mut d = dep_per_w * wt;
                if capped {
                    let used = if *sf.laid_id.add(i) == id {
                        *sf.laid.add(i)
                    } else {
                        0.0
                    };
                    d = d.min((cap - used).max(0.0));
                    *sf.laid_id.add(i) = id;
                    *sf.laid.add(i) = used + d;
                }
                if d > 0.0 {
                    let cv = &mut *sf.cover.add(i);
                    *cv = if fine {
                        ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0)
                    } else {
                        1.0
                    };
                    sf.add(i, d * (1.0 - phi), &blat, bhide, bcure);
                    if !sf.solv.is_null() {
                        *sf.solv.add(i) += d * phi * COAT_UM;
                    }
                    *sf.stroke.add(i) = id;
                    laid_liq += d * px_area;
                }
            }
            if push_k > 0.0 {
                let v = *sf.vol.add(i);
                let m = v * push_k * wt * fl;
                if m > 1e-6 {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    let side = if (px - a.0) * nx + (py - a.1) * ny >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    };
                    let tx = px + (nx * side * 0.75 + mx * 0.45) * off;
                    let ty = py + (ny * side * 0.75 + my * 0.45) * off;
                    let mut to = [(0.0f32, 0.0f32, 0.0f32); 4];
                    let n_to = if spread {
                        let (gx, gy) = (tx - 0.5, ty - 0.5);
                        let (fx, fy) = (gx - gx.floor(), gy - gy.floor());
                        let (bx, by) = (gx.floor() + 0.5, gy.floor() + 0.5);
                        to = [
                            (bx, by, (1.0 - fx) * (1.0 - fy)),
                            (bx + 1.0, by, fx * (1.0 - fy)),
                            (bx, by + 1.0, (1.0 - fx) * fy),
                            (bx + 1.0, by + 1.0, fx * fy),
                        ];
                        4
                    } else {
                        to[0] = (tx, ty, 1.0);
                        1
                    };
                    let (l, hd) = (*sf.lat.add(i), *sf.hide.add(i));
                    for &(tx, ty, share) in &to[..n_to] {
                        if share <= 0.0 {
                            continue;
                        }
                        let inside = tx >= ox as f32
                            && ty >= oy as f32
                            && (tx as usize) < w.min(ox + sf.w)
                            && (ty as usize) < h.min(oy + sf.h);
                        debug_assert!(
                            !inside
                                || (tx >= lim.0 as f32
                                    && ty >= lim.1 as f32
                                    && (tx as usize) < lim.2
                                    && (ty as usize) < lim.3),
                            "plough target outside the stroke footprint {lim:?}"
                        );
                        if inside
                            && tx >= lim.0 as f32
                            && ty >= lim.1 as f32
                            && (tx as usize) < lim.2
                            && (ty as usize) < lim.3
                        {
                            let (tx, ty) = (tx as usize, ty as usize);
                            let j = (ty - oy) * bw_buf + tx - ox;
                            let m = m * share * clip.map_or(1.0, |c| c.at(ty * w + tx).0);
                            let sol = if sf.solv.is_null() {
                                0.0
                            } else {
                                *sf.solv.add(i) / COAT_UM
                            };
                            let vi = *sf.vol.add(i);
                            let ms = if sol > 0.0 && vi > 0.0 {
                                (sol * m / vi).min(sol)
                            } else {
                                0.0
                            };
                            let (m, ms) = if capped && j != i && m > 0.0 {
                                let used = if *sf.laid_id.add(j) == id {
                                    *sf.laid.add(j)
                                } else {
                                    0.0
                                };
                                let room = (cap - used).max(0.0);
                                let f = if m + ms > room { room / (m + ms) } else { 1.0 };
                                *sf.laid_id.add(j) = id;
                                *sf.laid.add(j) = used + (m + ms) * f;
                                (m * f, ms * f)
                            } else {
                                (m, ms)
                            };
                            if j != i && m > 0.0 {
                                if ms > 0.0 {
                                    let si = &mut *sf.solv.add(i);
                                    let mu = (ms * COAT_UM).min(*si);
                                    *si -= mu;
                                    *sf.solv.add(j) += mu;
                                }
                                let cure = if sf.dry.is_null() {
                                    0.0
                                } else {
                                    (*sf.dry.add(i)).cure
                                };
                                let cj = &mut *sf.cover.add(j);
                                *cj = if fine {
                                    ((if *sf.vol.add(j) < 1e-6 { 0.0 } else { *cj })
                                        + (m / v.max(1e-9)).min(1.0) * *sf.cover.add(i))
                                    .min(1.0)
                                } else {
                                    1.0
                                };
                                sf.take(i, m);
                                sf.add(j, m, &l, hd, cure);
                            }
                        }
                    }
                }
            }
        }
        let outside_liq = dep_total * (1.0 - share.min(1.0));
        let gone = (laid_liq + outside_liq).min(liquid);
        br.vol = (br.vol - gone * (1.0 - phi)).max(0.0);
        br.solvent = (br.solvent - gone * phi).max(0.0);
        br.solvent += got_s;
        if got_v > 0.0 {
            for k in 0..LAT {
                got_l[k] /= got_v;
            }
            for g in &mut got_h {
                *g /= got_v;
            }
            br.cure = mix_cure(br.cure, br.vol, got_c / got_v, got_v);
            mix_into(&mut br.vol, &mut br.lat, &mut br.hide, got_v, &got_l, got_h);
        }
        let pad = (off + 2.0) as usize;
        grow(
            bounds,
            x0.saturating_sub(pad).max(ox),
            y0.saturating_sub(pad).max(oy),
            (x1 + pad).min(ox + sf.w),
            (y1 + pad).min(oy + sf.h),
        );
        Some(AcceptedTransfer {
            picked: Budget {
                paint: got_v,
                solvent: got_s,
            },
            laid: Budget {
                paint: laid_liq * (1.0 - phi),
                solvent: laid_liq * phi,
            },
            outside: Budget {
                paint: outside_liq * (1.0 - phi),
                solvent: outside_liq * phi,
            },
        })
    }
}
