//! Engine 7's paint exchange between one bristle and the canvas (lane
//! STIL-68).
//!
//! Contract (frozen, CONTRACTS-e7.md): `exchange` takes exactly
//! `bristle::exchange`'s arguments and keeps its physics: one ordered contact
//! along the capsule a–b, whose accepted pickup and deposit change the
//! bristle and the canvas before the next contact. Its stages:
//! `SegmentInput` (the segment with the live brush state) → `Contact` over a
//! `Window` → `Budget` → `AcceptedTransfer`. Requested transfer is never
//! reported as accepted.
//!
//! The law is engine 5's, hair after hair. What changes is the cost of a
//! contact:
//! - the capsule's contact list is gathered once into the thread's window,
//!   so the second pass visits only the pixels the hair touches, with their
//!   fluidity read once;
//! - pickup and plough sum paint as packed 21-float vectors (volume, latent
//!   color, properties, cure, cover, solvent), which the compiler lays out
//!   in vector registers;
//! - the plough gathers what it moves per destination pixel and lays it
//!   once per pixel when the contact ends (one add per destination instead
//!   of one per source and bilinear corner), with the clip's acceptance read
//!   once per destination and the source's loss taken once;
//! - per-contact constants (smoothstep spans, `1 / seg²`, the hunger curve)
//!   are hoisted out of the pixel loops, and floors avoid libm.
//!
//! The one change in what paint does: paint a hair ploughs within its own
//! capsule arrives when the contact ends, so the same contact doesn't lift
//! it or plough it on again. Engine 5 did that only for targets later in
//! raster order (its plough leaned down and to the right). Paint ploughed
//! off a pixel also carries the cure that pixel had before it was ploughed.

use super::Rect;
use super::add::{Film, Paint7};
use super::window::{self, COVER, CURE, HIDE0, Hit, LAT0, PACK, SOLV, VOL, Window};
use crate::bristle::{Bounds, Bristle, Clip, GHOST_TOUCH, Surf, Tool, WET_REACH, capsule_cover, fine_cover, grow, mix_cure};
use crate::smoothstep;
use crate::surface::COAT_UM;
use crate::wet::{LAT, Latent, Prop, mix_into};

/// One bristle's segment of a stroke, with the brush as it is now.
pub(crate) struct SegmentInput<'a> {
    pub sf: Surf,
    /// The live bristle: its load changes as the segment is painted.
    pub bristle: &'a mut Bristle,
    pub tool: &'a Tool,
    /// The capsule's ends and radius (whole-canvas pixels).
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub rb: f32,
    /// How far past touching the hair is pressed (0..1).
    pub reach: f32,
    /// A full load's volume for one bristle (`Held::full`).
    pub full: f32,
    /// A fixed deposit (a touch) instead of one by distance travelled.
    pub dep: Option<f32>,
    /// The share of the track this hair lays alone (pointed tools).
    pub excl: f32,
    pub clip: Option<Clip<'a>>,
    pub stroke_id: u32,
    /// The stroke's footprint: no pixel outside it is read or written.
    pub limit: Rect,
}

/// What the capsule touches, summed over its pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Contact {
    /// Geometric coverage (pixels).
    pub coverage: f32,
    /// Contact weight: coverage times how hard the hair presses there.
    pub weight: f32,
    /// Contact-weighted tack of the film.
    pub tack: f32,
    /// Contact weight a fence or mask held back.
    pub fence: f32,
}

/// What may move in this contact, in coats × units² (material volume, not
/// solvent µm on the canvas).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Budget {
    pub paint: f32,
    pub solvent: f32,
}

/// What did move, in coats × units²: picked up into the bristle, laid onto
/// the canvas, and ploughed outside the capsule.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AcceptedTransfer {
    pub picked: f32,
    pub laid: f32,
    pub outside: f32,
}

/// `x.floor()` for |x| < 2³¹ without a libm call.
#[inline(always)]
fn floor(x: f32) -> f32 {
    let t = x as i32 as f32;
    if t > x { t - 1.0 } else { t }
}

/// `smoothstep(e0, e0 + 1 / inv, x)`.
#[inline(always)]
fn ramp(e0: f32, inv: f32, x: f32) -> f32 {
    let t = ((x - e0) * inv).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A pixel's paint, packed (`window::PACK` layout) with volume 1: its
/// color, the properties `hd` of the paint that leaves it (`Surf::take`'s)
/// and `cure`.
#[inline(always)]
unsafe fn packed(sf: &Surf, i: usize, hd: &Prop, cure: f32) -> [f32; PACK] {
    unsafe {
        let l = &*sf.lat.add(i);
        let mut p = [0.0f32; PACK];
        p[VOL] = 1.0;
        p[LAT0..LAT0 + LAT].copy_from_slice(&l.color);
        p[HIDE0..HIDE0 + hd.len()].copy_from_slice(hd);
        p[CURE] = cure;
        p
    }
}

#[inline(always)]
fn accumulate(acc: &mut [f32; PACK], p: &[f32; PACK], v: f32) {
    for k in 0..PACK {
        acc[k] += p[k] * v;
    }
}

/// Engine 7's exchange, with `bristle::exchange`'s arguments (`wts` is only
/// used by the engine-5 fallback for a contact larger than the window).
///
/// SAFETY: as `bristle::exchange`: no other thread touches pixels in `lim`.
#[allow(clippy::too_many_arguments)]
#[inline]
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
    // (the capsule's rectangle is at most |b − a| + 2·rb + 4 pixels a side)
    if ((b.0 - a.0).abs() + 2.0 * rb + 4.0) * ((b.1 - a.1).abs() + 2.0 * rb + 4.0) > window::CAPACITY as f32 {
        unsafe { crate::bristle::exchange(sf, br, tool, a, b, rb, reach, full, dep, excl, clip, id, wts, bounds, lim) };
        return;
    }
    let seg = SegmentInput { sf, bristle: &mut *br, tool, a, b, rb, reach, full, dep, excl, clip, stroke_id: id, limit: lim };
    if window::with(|w| unsafe { contact(seg, bounds, w) }).is_none() {
        unsafe { crate::bristle::exchange(sf, br, tool, a, b, rb, reach, full, dep, excl, clip, id, wts, bounds, lim) };
    }
}

/// One ordered contact: pass 1 gathers the capsule's contact into `win`,
/// pass 2 picks up, lays and ploughs pixel after pixel, and the ploughed
/// paint lands before it returns. None, having changed nothing, if the
/// pixels it may plough into don't fit the window.
///
/// SAFETY: as `exchange`.
unsafe fn contact(seg: SegmentInput<'_>, bounds: &mut Bounds, win: &mut Window) -> Option<AcceptedTransfer> {
    unsafe {
        let SegmentInput { sf, bristle: br, tool, a, b, rb, reach, full, dep, excl, clip, stroke_id: id, limit: lim } = seg;
        let sf = &sf;
        let (w, h) = (sf.fw, sf.fh);
        let (ox, oy, bw_buf) = (sf.ox, sf.oy, sf.w);
        let s = sf.scale;
        let px_area = 1.0 / (s * s);
        let cx0 = (floor(a.0.min(b.0) - rb - 1.0).max(0.0)) as usize;
        let cy0 = (floor(a.1.min(b.1) - rb - 1.0).max(0.0)) as usize;
        let cx1 = ((-floor(-(a.0.max(b.0) + rb + 1.0))).max(0.0) as usize).min(w);
        let cy1 = ((-floor(-(a.1.max(b.1) + rb + 1.0))).max(0.0) as usize).min(h);
        let none = AcceptedTransfer::default();
        if cx1 <= cx0 || cy1 <= cy0 {
            return Some(none);
        }
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        let fine = tool.point > 0.0 && dep.is_none();
        // the brush side of the plough, fixed for the contact
        let ps = (br.hide[1] * (1.0 - br.hide[3]).powi(2)).clamp(0.0, 1.0);
        let stiffen = ((ps - 0.4) / 0.6).clamp(0.0, 1.0);
        let hair = if fine || dep.is_some() { rb } else { (tool.hair_radius() * s).min(rb) };
        let push_k = tool.push * (seg / (2.0 * hair)).clamp(0.0, 1.0) * (hair / rb);
        let off = if dep.is_some() {
            rb + 1.0
        } else if sf.clump && stiffen > 0.0 && !fine && br.rx.abs() > 0.7 {
            (2.0 * tool.hair_radius() * s * (1.0 + 4.0 * stiffen * stiffen)).max(2.0 * hair).min(3.0)
        } else {
            2.0 * hair
        };
        // the pixels its plough may reach on the whole canvas must fit the
        // window (whatever a crop render holds of them, so a crop paints
        // with the same kernel as the whole canvas)
        let pad = off as usize + 3;
        let (ex0, ey0, ex1, ey1) = (cx0.max(lim.0), cy0.max(lim.1), cx1.min(lim.2), cy1.min(lim.3));
        if ex1 > ex0 && ey1 > ey0 && ((ex1 + pad).min(lim.2) - ex0.saturating_sub(pad).max(lim.0)) * ((ey1 + pad).min(lim.3) - ey0.saturating_sub(pad).max(lim.1)) > window::CAPACITY {
            return None;
        }
        let (x0, y0, x1, y1) = (cx0.max(ox), cy0.max(oy), cx1.min(ox + sf.w), cy1.min(oy + sf.h));
        let windowed = (x0, y0, x1, y1) != (cx0, cy0, cx1, cy1);
        if x1 <= x0 || y1 <= y0 {
            // outside a crop window: the bristle lays paint as if it touched
            let travel = (seg / s).max(rb / s * 0.5);
            let before = br.vol + br.solvent;
            if br.solvent > 0.0 {
                let l = br.vol + br.solvent;
                let travel = if crate::thinner::exchange() && !sf.laid.is_null() { travel * (1.0 - br.solvent / l).powf(crate::thinner::exchange_kj().0) } else { travel };
                let left = match dep {
                    None => l * (1.0 - (1.0 - (-travel / tool.run).exp()) * GHOST_TOUCH),
                    Some(v) => (l - v.min(l * 0.5) * GHOST_TOUCH).max(0.0),
                };
                let f = if l > 0.0 { left / l } else { 0.0 };
                br.vol *= f;
                br.solvent *= f;
            } else {
                br.vol = match dep {
                    None => br.vol * (1.0 - (1.0 - (-travel / tool.run).exp()) * GHOST_TOUCH),
                    Some(v) => (br.vol - v.min(br.vol * 0.5) * GHOST_TOUCH).max(0.0),
                };
            }
            return Some(AcceptedTransfer { laid: before - br.vol - br.solvent, ..none });
        }
        debug_assert!(
            x0 >= lim.0 && y0 >= lim.1 && x1 <= lim.2 && y1 <= lim.3,
            "bristle contact ({x0},{y0},{x1},{y1}) outside the stroke footprint {lim:?}"
        );
        let (x0, y0, x1, y1) = (x0.max(lim.0), y0.max(lim.1), x1.min(lim.2), y1.min(lim.3));
        if x1 <= x0 || y1 <= y0 {
            return Some(none);
        }
        let (mx, my) = if seg > 1e-4 { (dx / seg, dy / seg) } else { (0.0, 0.0) };
        let (nx, ny) = (-my, mx);
        let liquid = if br.solvent > 0.0 { br.vol + br.solvent } else { br.vol };
        let phi = if br.solvent > 0.0 { br.solvent / liquid } else { 0.0 };
        let exch = phi > 0.0 && !sf.laid.is_null() && crate::thinner::exchange();
        // a pass that lays a thin film (`Handling::film`): all its strokes
        // together add no more than its ceiling to a pixel, and what the hair
        // can't lay stays on it, as a thinned load's does (with both, the
        // lower ceiling, for the whole pass)
        let pass_film = sf.film.filter(|_| !sf.laid.is_null());
        let thinned = phi > 0.0 && !sf.laid.is_null() && !exch;
        let capped = thinned || pass_film.is_some();
        let cap = pass_film.map_or(f32::INFINITY, |f| f.0).min(if thinned { crate::thinner::stroke_limit_um(phi) / COAT_UM } else { f32::INFINITY });
        let wet = smoothstep(0.1, 0.8, liquid / full);
        let wick = if tool.point > 0.0 { tool.point * smoothstep(0.02, 0.25, liquid / full) } else { 0.0 }.max(WET_REACH * wet);
        let th = 1.0 - reach.max(wick) * 1.6;

        let spread = dep.is_none();
        // where ploughed paint may land: the crop window, the footprint, and
        // within reach of the capsule (a target is ≤ 0.875·off away, and a
        // bilinear share one pixel further)
        let (lx, ly) = (ox.max(lim.0), oy.max(lim.1));
        let (hx, hy) = (w.min(ox + sf.w).min(lim.2), h.min(oy + sf.h).min(lim.3));
        let interior = (x0.saturating_sub(pad).max(lx), y0.saturating_sub(pad).max(ly), (x1 + pad).min(hx), (y1 + pad).min(hy));
        win.begin((x0, y0, x1, y1), interior);
        let (lx, ly, hx, hy) = (interior.0 as isize, interior.1 as isize, interior.2 as isize, interior.3 as isize);
        // where a pixel's ploughed paint goes, on either side of the path:
        // a fixed offset from it (outward and ahead by `off`), so the same
        // whole-pixel steps and bilinear shares for every pixel on that side
        // (a fixed deposit's plough takes the one pixel under the target)
        let mut throw = [[(0isize, 0isize, 0.0f32); 4]; 2];
        for (t, sgn) in throw.iter_mut().zip([1.0f32, -1.0]) {
            let (ddx, ddy) = ((nx * sgn * 0.75 + mx * 0.45) * off, (ny * sgn * 0.75 + my * 0.45) * off);
            if spread {
                let (fgx, fgy) = (floor(ddx), floor(ddy));
                let (fx, fy) = (ddx - fgx, ddy - fgy);
                let (bx, by) = (fgx as isize, fgy as isize);
                *t = [(bx, by, (1.0 - fx) * (1.0 - fy)), (bx + 1, by, fx * (1.0 - fy)), (bx, by + 1, (1.0 - fx) * fy), (bx + 1, by + 1, fx * fy)];
            } else {
                t[0] = (floor(0.5 + ddx) as isize, floor(0.5 + ddy) as isize, 1.0);
            }
        }
        let n_to = if spread { 4 } else { 1 };

        // pass 1: contact weights, into the window's contact list
        let r_out = rb + 0.5;
        let (cov_e0, cov_inv) = if dep.is_some() { (0.0, 1.0 / rb) } else { (rb * 0.5, 1.0 / (r_out - rb * 0.5)) };
        let give = 0.15 + 0.45 * (1.0 - tool.stiffness).clamp(0.0, 1.0);
        let (s_e0, s_inv) = (th - give, 1.0 / (0.2 + give));
        let inv_seg2 = if seg2 > 1e-8 { 1.0 / seg2 } else { 0.0 };
        let dry = !sf.dry.is_null();
        let mut ct = Contact::default();
        // the contact weight on pixels that take what the bristle lays
        let mut sum_r = 0.0f32;
        for y in y0..y1 {
            let py = y as f32 + 0.5;
            let row = (y - oy) * bw_buf;
            for x in x0..x1 {
                let px = x as f32 + 0.5;
                let cov = if fine {
                    fine_cover(a, b, rb, px, py)
                } else {
                    let t = (((px - a.0) * dx + (py - a.1) * dy) * inv_seg2).max(0.0).min(1.0);
                    let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
                    let d2 = qx * qx + qy * qy;
                    if d2 > r_out * r_out {
                        continue;
                    }
                    1.0 - ramp(cov_e0, cov_inv, d2.sqrt())
                };
                if cov <= 0.0 {
                    continue;
                }
                ct.coverage += cov;
                let i = row + x - ox;
                let v = *sf.vol.add(i);
                let surf = (*sf.base.add(i) + 0.35 * v).min(1.5);
                let wt = match clip {
                    None => cov * ramp(s_e0, s_inv, surf),
                    Some(Clip::Mask(m)) => cov * ramp(s_e0, s_inv, surf) * m.data[y * w + x],
                    Some(c @ Clip::Fence { .. }) => {
                        let (k, lift) = c.at(y * w + x);
                        let wt = cov * ramp(s_e0 + lift, s_inv, surf) * k;
                        ct.fence += wt * k * k;
                        wt
                    }
                };
                ct.weight += wt;
                let mut fl = 1.0;
                if dry {
                    let p = *sf.dry.add(i);
                    let (f, tack) = crate::drying::feel(v, p.cure, p.sub);
                    fl = f;
                    ct.tack += wt * tack;
                }
                if wt > 0.0 {
                    win.hits.push(Hit { i, x: x as u32, y: y as u32, fl });
                    win.weights.push(wt);
                    sum_r += wt * sf.takes(i);
                }
            }
        }
        if ct.weight <= 1e-6 {
            return Some(none);
        }
        let sum_w = ct.weight;
        let tack = ct.tack / sum_w;

        let travel = (seg / s).max(rb / s * 0.5);
        let travel = if exch { travel * (1.0 - phi).powf(crate::thinner::exchange_kj().0) } else { travel };
        let touch = (sum_w / ct.coverage.max(1e-6)).min(1.0);
        let dep_total = match dep {
            None => liquid * (1.0 - (-travel / tool.run).exp()) * touch,
            Some(v) => v.min(liquid * 0.5) * touch,
        };
        let dep_total = if matches!(clip, Some(Clip::Fence { .. })) { dep_total * (ct.fence / sum_w).min(1.0) } else { dep_total };
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
        let dep_total = if sf.clump && dep.is_none() && !fine && stiffen > 0.0 {
            let clump_u = tool.hair_radius() * (1.0 + 4.0 * stiffen * stiffen);
            let bands = (tool.width / (2.0 * clump_u)).clamp(2.0, 48.0);
            let u = (br.rx.clamp(-1.0, 1.0) + 1.0) * 0.5 * bands;
            let b0 = floor(u);
            let f = u - b0;
            let h0 = crate::rng::hash2(b0 as i64, id as i64, 0x51C0);
            let h1 = crate::rng::hash2(b0 as i64 + 1, id as i64, 0x51C0);
            let band = h0 + (h1 - h0) * crate::smoothstep(0.0, 1.0, f);
            (dep_total * (1.0 + 0.6 * stiffen * (2.0 * band - 1.0))).min(liquid * 0.95)
        } else {
            dep_total
        };
        // (what the pixels it touched didn't take stays on the hair)
        let kept = if sf.receptive.is_null() { 1.0 } else { (sum_r / sum_w).min(1.0) };
        let budget = Budget { paint: dep_total * kept * (1.0 - phi), solvent: dep_total * phi };
        let share = if windowed { ct.coverage / capsule_cover(a, b, rb, fine, (cx0, cy0, cx1, cy1)).max(1e-6) } else { 1.0 };
        let dep_per_w = dep_total * share.min(1.0) / sum_w / px_area;
        let spent = (1.0 - liquid / full).clamp(0.0, 1.0);
        let pickup = tool.pickup * (0.35 + 0.65 * spent * spent.sqrt());

        let film = Film::of(sf);
        let laying = Paint7::new(sf.lean, sf.drain, &br.lat, br.hide, br.cure);
        let mut got = [0.0f32; PACK];
        let mut got_parts = Latent::default();
        let mut got_s = 0.0f32;
        let mut laid_liq = 0.0f32;
        let mut ploughed = 0.0f32;
        let hits = std::mem::take(&mut win.hits);
        let weights = std::mem::take(&mut win.weights);
        // pass 2: pickup, deposit and plough, pixel after pixel
        for (&Hit { i, x, y, fl }, &wt) in hits.iter().zip(&weights) {
            let takes = sf.takes(i);
            let v = *sf.vol.add(i);
            if *sf.touched.add(i) != id {
                *sf.touched.add(i) = id;
                *sf.floor.add(i) = v * (1.0 - tool.pickup * fl);
            }
            if v > 1e-6 {
                let tk = if exch {
                    (v * pickup * wt * fl).min(v)
                } else {
                    let own = if *sf.stroke.add(i) == id { 0.15 } else { 1.0 };
                    (v * pickup * wt * own * fl).min((v - *sf.floor.add(i)).max(0.0))
                };
                if tk > 0.0 {
                    let cure = if dry { (*sf.dry.add(i)).cure } else { 0.0 };
                    if !sf.solv.is_null() {
                        let sv = &mut *sf.solv.add(i);
                        if *sv > 0.0 {
                            let ts = (*sv * tk / v).min(*sv);
                            *sv -= ts;
                            sf.film_give(i, ts / COAT_UM);
                            got_s += ts / COAT_UM * px_area;
                        }
                    }
                    // (the paint lifted: with the drain switch from the top
                    // down, `bristle::take_from`; without, the film's)
                    let hp = sf.take(i, tk);
                    accumulate(&mut got, &packed(sf, i, &hp, cure), tk * px_area);
                    got_parts.add_parts(&*sf.lat.add(i), tk * px_area);
                }
            }
            if dep_per_w > 0.0 && !capped && phi <= 0.0 {
                let cv = &mut *sf.cover.add(i);
                *cv = if fine { ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0) } else { 1.0 };
                film.lay(i, dep_per_w * wt * takes, &laying, sf.skim);
                *sf.stroke.add(i) = id;
            } else if dep_per_w > 0.0 {
                let mut d = dep_per_w * wt * takes;
                if let Some((_, from)) = pass_film {
                    d *= crate::bristle::film_lay(&sf, i, from, cap * clip.map_or(1.0, |c| c.at(y as usize * w + x as usize).0), id, d);
                } else if capped {
                    let used = if *sf.laid_id.add(i) == id { *sf.laid.add(i) } else { 0.0 };
                    d = d.min((cap - used).max(0.0));
                    *sf.laid_id.add(i) = id;
                    *sf.laid.add(i) = used + d;
                }
                if d > 0.0 {
                    let cv = &mut *sf.cover.add(i);
                    *cv = if fine { ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0) } else { 1.0 };
                    film.lay(i, d * (1.0 - phi), &laying, sf.skim);
                    if !sf.solv.is_null() {
                        *sf.solv.add(i) += d * phi * COAT_UM;
                    }
                    *sf.stroke.add(i) = id;
                    laid_liq += d * px_area;
                }
            }
            // plough: gather what moves outward from the path and ahead
            if push_k <= 0.0 {
                continue;
            }
            let v = *sf.vol.add(i);
            let m = v * push_k * wt * fl;
            if m <= 1e-6 {
                continue;
            }
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let to = &throw[((px - a.0) * nx + (py - a.1) * ny < 0.0) as usize][..n_to];
            let sol_um = if sf.solv.is_null() { 0.0 } else { *sf.solv.add(i) };
            let cure = if dry { (*sf.dry.add(i)).cure } else { 0.0 };
            let (src_solv, src_cover) = (if sol_um > 0.0 && v > 0.0 { sol_um / v } else { 0.0 }, if fine { *sf.cover.add(i) / v.max(1e-9) } else { 0.0 });
            // where it goes and how much, before any of it leaves (`take`
            // says what paint leaves, so it lands after)
            let mut moves = [(0usize, 0usize, 0usize, 0.0f32); 4];
            let mut n_moves = 0;
            let mut out_m = 0.0f32;
            for &(ddx, ddy, sh) in to {
                let (tx, ty) = (x as isize + ddx, y as isize + ddy);
                if sh <= 0.0 || (ddx == 0 && ddy == 0) || tx < lx || ty < ly || tx >= hx || ty >= hy {
                    continue;
                }
                let (tx, ty) = (tx as usize, ty as usize);
                let j = (ty - oy) * bw_buf + tx - ox;
                let c = win.cell(tx, ty, j, || clip.map_or(1.0, |c| c.at(ty * w + tx).0) * sf.takes(j));
                let mut mm = m * sh * c.k;
                if let Some((_, from)) = pass_film.filter(|_| mm > 0.0) {
                    mm *= crate::bristle::film_lay(&sf, j, from, cap * c.k, id, mm * (1.0 + src_solv / COAT_UM));
                } else if capped && mm > 0.0 {
                    // a thinned stroke adds no more wet film to the pixel it
                    // ploughs into than its ceiling there allows
                    let ms = src_solv / COAT_UM * mm;
                    let used = if *sf.laid_id.add(j) == id { *sf.laid.add(j) } else { 0.0 };
                    let room = (cap - used).max(0.0);
                    let f = if mm + ms > room { room / (mm + ms) } else { 1.0 };
                    *sf.laid_id.add(j) = id;
                    *sf.laid.add(j) = used + (mm + ms) * f;
                    mm *= f;
                }
                if mm <= 0.0 {
                    continue;
                }
                moves[n_moves] = (tx, ty, j, mm);
                n_moves += 1;
                out_m += mm;
            }
            if out_m > 0.0 {
                if sol_um > 0.0 {
                    let left = (sol_um - src_solv * out_m).max(0.0);
                    *sf.solv.add(i) = left;
                    sf.film_give(i, (sol_um - left) / COAT_UM);
                }
                // the source's paint per coat moved: its color, the paint
                // that leaves (with the drain switch from the top down, all
                // of the contact's plough off it at once: as much oil as
                // engine 5's take per target, shared evenly among them), the
                // cure it had, the cover it brings (fine tools) and its
                // solvent (µm), in the film's proportions (m ≤ v, so these
                // never move more than is there)
                let hp = sf.take(i, out_m);
                let mut src = packed(sf, i, &hp, cure);
                src[COVER] = src_cover;
                src[SOLV] = src_solv;
                for &(tx, ty, j, mm) in &moves[..n_moves] {
                    let cell = win.cell(tx, ty, j, || unreachable!("a plough cell is started before its paint lands"));
                    accumulate(&mut cell.a, &src, mm);
                    cell.parts.add_parts(&*sf.lat.add(i), mm);
                }
                ploughed += out_m * px_area;
            }
        }
        win.hits = hits;
        win.weights = weights;
        // the ploughed paint lands, one add per destination pixel
        for c in win.hit_cells() {
            let m = c.a[VOL];
            if m <= 0.0 {
                continue;
            }
            let j = c.j;
            let inv = 1.0 / m;
            if !inv.is_finite() {
                continue;
            }
            let mut lat: Latent = Latent::default();
            lat.parts = c.parts.parts.clone();
            lat.scale_parts(inv);
            for k in 0..LAT {
                lat[k] = c.a[LAT0 + k] * inv;
            }
            let mut hide: Prop = [0.0; 10];
            for k in 0..hide.len() {
                hide[k] = c.a[HIDE0 + k] * inv;
            }
            if c.a[SOLV] > 0.0 {
                *sf.solv.add(j) += c.a[SOLV];
            }
            let cj = &mut *sf.cover.add(j);
            *cj = if fine { ((if *sf.vol.add(j) < 1e-6 { 0.0 } else { *cj }) + c.a[COVER]).min(1.0) } else { 1.0 };
            film.add(j, m, &Paint7::new(sf.lean, sf.drain, &lat, hide, c.a[CURE] * inv));
        }
        let had = br.vol + br.solvent;
        if phi > 0.0 || pass_film.is_some() {
            let gone = (laid_liq + dep_total * (1.0 - share.min(1.0))).min(liquid);
            br.vol = (br.vol - gone * (1.0 - phi)).max(0.0);
            br.solvent = (br.solvent - gone * phi).max(0.0);
        } else {
            br.vol = (br.vol - budget.paint).max(0.0);
        }
        let laid = had - br.vol - br.solvent;
        br.solvent += got_s;
        let got_v = got[VOL];
        if got_v > 0.0 {
            let mut got_l: Latent = Latent::default();
            got_l.parts = got_parts.parts;
            got_l.scale_parts(1.0 / got_v);
            for k in 0..LAT {
                got_l[k] = got[LAT0 + k] / got_v;
            }
            let mut got_h: Prop = [0.0; 10];
            for k in 0..got_h.len() {
                got_h[k] = got[HIDE0 + k] / got_v;
            }
            br.cure = mix_cure(br.cure, br.vol, got[CURE] / got_v, got_v);
            mix_into(&mut br.vol, &mut br.lat, &mut br.hide, got_v, &got_l, got_h);
        }
        let pad = (off + 2.0) as usize;
        grow(bounds, x0.saturating_sub(pad).max(ox), y0.saturating_sub(pad).max(oy), (x1 + pad).min(ox + sf.w), (y1 + pad).min(oy + sf.h));
        Some(AcceptedTransfer { picked: got_v + got_s, laid, outside: ploughed })
    }
}

#[cfg(test)]
mod tests {
    use crate::bristle::{Gesture, Held, Tool};
    use crate::canvas::Canvas;
    use crate::color::hex;
    use crate::mask::Mask;
    use crate::surface::COAT_UM;
    use crate::wet::{LAT, Paint};

    /// 400 px across 150 mm: r40's pitch (900 mm at 2400 px).
    fn canvas(engine: u32) -> Canvas {
        Canvas::new(400, 1.25, hex("#e8e0d0")).with_size_mm(150.0).with_engine(engine)
    }

    /// A wet film of `coats` of `paint` over the units rectangle `r`.
    fn film(c: &mut Canvas, r: (f32, f32, f32, f32), paint: Paint, coats: f32) {
        let (w, s) = (c.f.w, c.f.scale);
        let (lat, prop) = (paint.latent(), paint.prop());
        for y in (r.1 * s) as usize..((r.3 * s) as usize).min(c.f.h) {
            for x in (r.0 * s) as usize..((r.2 * s) as usize).min(w) {
                let i = y * w + x;
                c.wet.vol[i] = coats;
                c.wet.lat[i] = lat.clone();
                c.wet.hide[i] = prop;
            }
        }
        c.wet.dirty = Some((0, 0, c.f.w, c.f.h));
    }

    /// Paint, each pigment and solvent on the canvas and in the brush,
    /// in the brush's units (units² × coats).
    fn totals(c: &Canvas, held: &Held) -> (f64, [f64; LAT], f64) {
        let pa = 1.0 / (c.f.scale as f64 * c.f.scale as f64);
        let (mut v, mut p, mut s) = (0.0f64, [0.0f64; LAT], 0.0f64);
        for i in 0..c.wet.vol.len() {
            let vi = c.wet.vol[i] as f64 * pa;
            v += vi;
            for k in 0..LAT {
                p[k] += c.wet.lat[i][k] as f64 * vi;
            }
            s += c.wet.solv.get(i).copied().unwrap_or(0.0) as f64 / COAT_UM as f64 * pa;
        }
        for b in &held.bristles {
            v += b.vol as f64;
            for k in 0..LAT {
                p[k] += b.lat[k] as f64 * b.vol as f64;
            }
            s += b.solvent as f64;
        }
        (v, p, s)
    }

    fn assert_conserved(before: (f64, [f64; LAT], f64), after: (f64, [f64; LAT], f64), what: &str) {
        let rel = |a: f64, b: f64| (a - b).abs() / a.abs().max(b.abs()).max(1e-9);
        assert!(rel(before.0, after.0) < 2e-4, "{what}: paint {} -> {}", before.0, after.0);
        for k in 0..LAT {
            assert!((before.1[k] - after.1[k]).abs() < 2e-4 * before.0.max(1e-9), "{what}: pigment {k} {} -> {}", before.1[k], after.1[k]);
        }
        assert!(rel(before.2, after.2) < 2e-4 || (before.2 - after.2).abs() < 1e-9, "{what}: solvent {} -> {}", before.2, after.2);
    }

    fn assert_non_negative(c: &Canvas, held: &Held, what: &str) {
        assert!(c.wet.vol.iter().all(|&v| v >= -1e-7 && v.is_finite()), "{what}: canvas paint below zero");
        assert!(c.wet.solv.iter().all(|&v| v >= -1e-5 && v.is_finite()), "{what}: canvas solvent below zero");
        assert!(held.bristles.iter().all(|b| b.vol >= 0.0 && b.solvent >= 0.0 && b.vol.is_finite()), "{what}: brush below zero");
    }

    /// One fixture: a canvas set up by `setup`, a brush, and its strokes,
    /// painted with `engine`. Checks conservation and non-negativity of
    /// every stroke in engine 7.
    fn paint(engine: u32, setup: &dyn Fn(&mut Canvas), mut held: Held, strokes: &[Gesture], clip: Option<&dyn Fn(&Canvas) -> Mask>) -> (Canvas, Held) {
        let mut c = canvas(engine);
        setup(&mut c);
        held = held.with_engine(engine);
        let mask = clip.map(|f| f(&c));
        for g in strokes {
            let before = totals(&c, &held);
            c.drag(&mut held, g, mask.as_ref());
            if engine == 7 {
                // (an absorbent ground takes oil out of the paint, in engine 5
                // as in 7: nothing to conserve there)
                if !c.absorb_any {
                    assert_conserved(before, totals(&c, &held), "engine 7 stroke");
                }
                assert_non_negative(&c, &held, "engine 7 stroke");
            }
        }
        (c, held)
    }

    /// How far apart two paintings' wet films are: the paint moved between
    /// them over the paint there (L1).
    fn film_gap(a: &Canvas, b: &Canvas) -> f32 {
        let d: f64 = a.wet.vol.iter().zip(&b.wet.vol).map(|(x, y)| (x - y).abs() as f64).sum();
        let t: f64 = a.wet.vol.iter().map(|x| x.abs() as f64).sum();
        (d / t.max(1e-12)) as f32
    }

    fn brush_vol(h: &Held) -> f32 {
        h.bristles.iter().map(|b| b.vol).sum()
    }

    const RED: &str = "#b8321e";

    const BLUE: &str = "#203c8c";

    /// Engine 5 and 7 on the same fixture: both paintings, and how far
    /// apart their films and brushes are.
    fn both(setup: &dyn Fn(&mut Canvas), held: impl Fn() -> Held, strokes: &[Gesture], clip: Option<&dyn Fn(&Canvas) -> Mask>) -> ((Canvas, Held), (Canvas, Held)) {
        let five = paint(5, setup, held(), strokes, clip);
        let seven = paint(7, setup, held(), strokes, clip);
        (five, seven)
    }

    /// With the drain switch, on an absorbent ground (paint laid fluid, the
    /// ground draining the film as its pigment packs, paint lifted and
    /// ploughed from the top down): engine 7 lays, lifts, ploughs and drains
    /// as engine 5 does, and the switch is on in both (the ground fills, a
    /// packed layer grows).
    #[test]
    fn the_drain_switch_drains_as_in_engine_5() {
        let setup = |c: &mut Canvas| {
            *c = canvas(c.engine()).with_drain(true);
            c.ground_finish(0.7);
            film(c, (0.0, 0.0, 500.0, 800.0), Paint { oil: 1.3, ..Paint::body(hex(RED)) }, 0.8);
        };
        let strokes: Vec<Gesture> = (0..4).map(|k| Gesture::new(vec![(150.0, 200.0 + 90.0 * k as f32), (450.0, 230.0 + 90.0 * k as f32), (850.0, 190.0 + 90.0 * k as f32)])).collect();
        let held = || {
            let mut h = Held::new(Tool::filbert(60.0), 12);
            h.load(Paint::body(hex(BLUE)), 1.0);
            h
        };
        let ((c5, h5), (c7, h7)) = both(&setup, held, &strokes, None);
        // the film's oil and packed paint (coats·px), and what the ground took
        let sums = |c: &Canvas| {
            let (mut oil, mut packed) = (0.0f64, 0.0f64);
            for (v, hd) in c.wet.vol.iter().zip(&c.wet.hide) {
                oil += (*v * hd[4]) as f64;
                packed += (*v * hd[7]) as f64;
            }
            (oil, packed, c.absorb.iter().map(|&a| a as f64).sum::<f64>())
        };
        let (s5, s7) = (sums(&c5), sums(&c7));
        let mut fresh = canvas(7).with_drain(true);
        fresh.ground_finish(0.7);
        let full: f64 = fresh.absorb.iter().map(|&a| a as f64).sum();
        eprintln!("drain: oil, packed, ground left: engine 5 {s5:?}, engine 7 {s7:?} of {full}; film gap {}, brush {} vs {}", film_gap(&c5, &c7), brush_vol(&h5), brush_vol(&h7));
        assert!(c7.drain() && s7.1 > 0.0 && s7.2 < 0.99 * full, "engine 7 drained nothing");
        let rel = |a: f64, b: f64| (a - b).abs() / a.abs().max(1e-9);
        assert!(rel(s5.0, s7.0) < 0.02 && rel(s5.1, s7.1) < 0.05 && rel(full - s5.2, full - s7.2) < 0.05, "{s5:?} vs {s7:?}");
        assert!(film_gap(&c5, &c7) < 0.05);
        assert!((brush_vol(&h5) - brush_vol(&h7)).abs() < 0.05 * brush_vol(&h5));
        let pk = |h: &Held| h.bristles.iter().map(|b| b.hide[7]).fold(0.0f32, f32::max);
        assert_eq!((pk(&h5), pk(&h7)), (0.0, 0.0), "paint taken up is broken up and fluid");
    }
}
