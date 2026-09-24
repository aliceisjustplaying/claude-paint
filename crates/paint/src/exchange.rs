//! Paint exchange between one bristle and the wet film under it
//! (notes/wet.md): how a bristle meets the film (`contact`, once per
//! bristle and step), and what it lays, lifts, stirs and pushes aside
//! pixel by pixel along its track (`exchange`, through the stroke's
//! `film::Stroke`).

use crate::bristle::{Bristle, Tool};
use crate::film::{Stroke, WET_FILM, grow};
use crate::smoothstep;
use crate::wet::{LAT, Prop, mix_into};

/// How far into the tooth's range the paint a fully loaded hair carries
/// reaches ahead of the hair (see `exchange`).
const WET_REACH: f32 = 0.5;
/// Share of a pixel's surface film one bristle pass works into the film's
/// body, for a stiff bristle pressed fully into wet paint with no paint of
/// its own (see `contact`; notes/wet.md).
const STIR: f32 = 0.12;
/// The share of its paint a barely touching moving bristle drags into the
/// wet film (see `contact`).
const GLANCE: f32 = 0.8;
/// Distance (units, e-folding) over which what a bristle picked up works
/// from its surface into its reservoir (see `Bristle::tip`).
pub(crate) const TIP_RUN: f32 = 15.0;
/// How much a full load cushions a bristle from the wet film under it: it
/// lifts and stirs `1 − CUSHION` of what a spent one does.
const CUSHION: f32 = 0.8;
/// The wet film (coats) a stiff hog bristle laid lightly rides on and can't
/// push aside (15 µm, a small fraction of the bristle's 0.2–0.3 mm): finer,
/// softer hair and more pressure get closer to the canvas (see `contact`).
const PLOUGH_KEEP: f32 = 0.6;
/// Paint stiffness from which wet paint keeps a plough's bow wave (a yield
/// stress); more fluid paint mostly flows back (a glaze at medium 0.85 is
/// about 0.02 stiff, a thin sky at medium 0.3 about 0.35, tube paint 1).
const PLOUGH_STIFF: f32 = 0.6;

/// How a bristle meets the canvas: moving along a stroke, laying a share
/// of its load per distance, or pressed down in a touch, laying about
/// `dep` (units² × coats) on full contact (film splitting).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Mode {
    Drag,
    Touch { dep: f32 },
}

/// How one bristle meets the wet film this step (`contact`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Contact {
    /// How loaded it is, 0..1 (the paint it carries proud of itself).
    wet: f32,
    /// The lowest surface level it reaches (pressure, or the paint it
    /// carries), for the contact weights.
    th: f32,
    /// How far soft hair bends into the weave's valleys.
    give: f32,
    /// How readily it lifts wet paint (film splitting): a spent bristle
    /// and fluid paint on it drink more.
    hunger: f32,
    /// Share of the body under the surface film a pickup reaches: a loaded
    /// bristle rides on a cushion of its own paint; a pressed tip is pushed
    /// into the wet paint and splits off it as it lifts.
    through: f32,
    /// Share of what it lays that it drags into the wet film instead of
    /// laying it on top.
    drag_in: f32,
    /// Share of the surface film it works into the body.
    stir: f32,
    /// Share of the paint standing above `keep` it pushes aside.
    push_k: f32,
    /// The film (coats) it rides on and can't push aside.
    keep: f32,
}

/// How a bristle carrying `fill` of a full load (paint of stiffness
/// `stiff`) meets the wet film, pressed to `reach` and moving `seg` pixels
/// this step with a track of radius `rb`.
pub(crate) fn contact(
    tool: &Tool,
    fill: f32,
    stiff: f32,
    reach: f32,
    seg: f32,
    rb: f32,
    mode: Mode,
) -> Contact {
    // lowest surface height this bristle reaches down to. The loaded tip
    // of a pointed soft brush carries a bead of paint that wets the
    // weave's valleys as well as its peaks, however lightly it is
    // pressed; run dry, it skims the peaks. Any loaded hair carries
    // paint proud of itself: the paint touches before the hair does, so
    // a well-loaded blunt brush wets the shallow hollows too (up to half
    // the tooth's range); a nearly dry one drags over the peaks only
    // (dry brush, broken color)
    let wet = smoothstep(0.1, 0.8, fill);
    let wick = if tool.point > 0.0 {
        tool.point * smoothstep(0.02, 0.25, fill)
    } else {
        0.0
    }
    .max(WET_REACH * wet);
    let th = 1.0 - reach.max(wick) * 1.6;
    // soft hair bends down into the valleys of the weave; stiff hog
    // bristles ride on the peaks
    let give = 0.15 + 0.45 * (1.0 - tool.stiffness).clamp(0.0, 1.0);
    // film splitting: a bristle in wet paint always lifts some of it, even
    // when loaded; a spent bristle drinks more
    // (fluid, medium-rich paint on the bristle wets into the film and
    // takes it up more readily than stiff paint)
    let hunger = (0.35 + 0.65 * (1.0 - fill).clamp(0.0, 1.0).powf(1.5))
        * (1.3 - 0.6 * stiff.clamp(0.0, 1.0));
    let push_k = tool.push * (seg / (2.0 * rb)).clamp(0.0, 1.0);
    // how far this bristle reaches into the wet film: pressed, stiff and
    // lean it goes through to the body; a loaded one rides on a cushion
    // of its own paint (the painter's loaded brush and light touch)
    // (a blender's fine, splayed hair holds no charge to ride on: the
    // cushion goes with how much paint the tool is made to carry)
    let cushion = 1.0 - CUSHION * wet * tool.lay.clamp(0.0, 1.0);
    let through = match mode {
        Mode::Drag => cushion,
        // (a pressed tip is pushed into the wet paint and splits off it as
        // it lifts: its own load cushions it less)
        Mode::Touch { .. } => 1.0 - 0.3 * wet,
    };
    // the share of what it lays that a bristle drags into the wet film
    // instead of laying on it: all of it for a blender, and for a nearly
    // spent bristle, whose last paint is what it picked up; and for a
    // moving bristle that only glances the wet film (a stroke's lift-off
    // and its edges), whose thin film meets the wet surface and is dragged
    // into it: stroke ends feather into wet paint instead of stopping
    // blunt (a touch pressed down doesn't)
    let glance = if mode == Mode::Drag {
        GLANCE * (1.0 - reach.clamp(0.0, 1.0)).powi(2)
    } else {
        0.0
    };
    let drag_in = (1.0 - tool.lay.clamp(0.0, 1.0))
        .max(1.0 - smoothstep(0.02, 0.2, fill))
        .max(glance);
    // (mixing is shear: it goes with how far the bristle moves, as the
    // plough does, so a tip pressed straight down barely stirs)
    // (any hair sheared across a surface film mixes it; a stiff one
    // pressed hard digs deeper into the body too)
    let stir = STIR
        * (seg / (2.0 * rb)).clamp(0.0, 1.0)
        * reach.clamp(0.0, 1.0).sqrt()
        * (0.5 + 0.5 * tool.stiffness.clamp(0.0, 1.0))
        * cushion;
    // the film a bristle rides on, which it can't push aside (coats):
    // thicker under coarse stiff hog (0.2–0.3 mm) than fine soft hair
    // (0.06–0.12 mm), thinner the harder it is pressed
    let keep = PLOUGH_KEEP
        * (0.3 + 0.7 * tool.stiffness.clamp(0.0, 1.0))
        * (1.0 - 0.4 * reach.clamp(0.0, 1.0));
    Contact {
        wet,
        th,
        give,
        hunger,
        through,
        drag_in,
        stir,
        push_k,
        keep,
    }
}

impl Contact {
    /// Coats the bristle pushes aside from a film `v` coats thick of
    /// stiffness `stiff`, at contact weight `wt` where the paint is `fl`
    /// fluid (`drying::fluid`): only what stands above the film it rides
    /// on, and only paint with a yield stress keeps a bow wave; fluid,
    /// medium-rich paint flows round the bristle and back into its furrow
    /// (leveling in seconds).
    #[inline]
    fn plough(&self, v: f32, wt: f32, fl: f32, stiff: f32) -> f32 {
        (v - self.keep).max(0.0)
            * self.push_k
            * wt
            * fl
            * (0.1 + 0.9 * smoothstep(0.0, PLOUGH_STIFF, stiff))
    }
}

/// Paint exchange between one bristle and the canvas along the capsule
/// swept from `a` to `b` (pixels, radius `rb`), for the stroke `st`: the
/// bristle lays paint (a moving one in proportion to the distance it
/// travels, a touch by `Mode::Touch`), lifts some of the film, stirs the
/// surface film into the body and pushes paint aside. `excl`: the share of
/// the width a pointed tool's hair covers (see `bristle::drag_on`).
///
/// SAFETY: as `Stroke::begin`: no other thread works the stroke's footprint.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn exchange(
    st: &mut Stroke,
    br: &mut Bristle,
    tool: &Tool,
    a: (f32, f32),
    b: (f32, f32),
    rb: f32,
    reach: f32,
    full: f32,
    mode: Mode,
    excl: f32,
) {
    unsafe {
        let sf = st.sf;
        let lim = st.lim;
        // pixel coordinates are whole-canvas ones; buffer index of (x, y) is
        // (y - oy) * bw + x - ox
        let (w, h) = (sf.fw, sf.fh);
        let (ox, oy, bw_buf) = (sf.ox, sf.oy, sf.w);
        let s = sf.scale;
        let px_area = 1.0 / (s * s);
        let cx0 = ((a.0.min(b.0) - rb - 1.0).floor().max(0.0)) as usize;
        let cy0 = ((a.1.min(b.1) - rb - 1.0).floor().max(0.0)) as usize;
        let cx1 = ((a.0.max(b.0) + rb + 1.0).ceil().max(0.0) as usize).min(w);
        let cy1 = ((a.1.max(b.1) + rb + 1.0).ceil().max(0.0) as usize).min(h);
        if cx1 <= cx0 || cy1 <= cy0 {
            return;
        }
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        // a crop render holds only a window of the canvas: clip to it
        let (x0, y0, x1, y1) = (
            cx0.max(ox),
            cy0.max(oy),
            cx1.min(ox + sf.w),
            cy1.min(oy + sf.h),
        );
        let windowed = (x0, y0, x1, y1) != (cx0, cy0, cx1, cy1);
        if x1 <= x0 || y1 <= y0 {
            // outside the window the canvas isn't there to feel: assume the
            // bristle touched and laid paint as usual (so it arrives in the
            // window about as spent as in a whole render), lifting none
            let travel = (seg / s).max(rb / s * 0.5);
            br.vol = match mode {
                Mode::Drag => br.vol * (1.0 - (1.0 - (-travel / tool.run).exp()) * GHOST_TOUCH),
                Mode::Touch { dep } => (br.vol - dep.min(br.vol * 0.5) * GHOST_TOUCH).max(0.0),
            };
            br.tip.v = br.tip.v.min(br.vol);
            return;
        }
        // never outside the stroke's footprint: the scheduler runs strokes
        // whose footprints don't overlap at once
        debug_assert!(
            x0 >= lim.0 && y0 >= lim.1 && x1 <= lim.2 && y1 <= lim.3,
            "bristle contact ({x0},{y0},{x1},{y1}) outside the stroke footprint {lim:?}"
        );
        let (x0, y0, x1, y1) = (x0.max(lim.0), y0.max(lim.1), x1.min(lim.2), y1.min(lim.3));
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let (mx, my) = if seg > 1e-4 {
            (dx / seg, dy / seg)
        } else {
            (0.0, 0.0)
        };
        let (nx, ny) = (-my, mx);
        // a pointed tool's moving hair covers pixels by the exact share of
        // its track in them (see `strip_cover`)
        let fine = tool.point > 0.0 && mode == Mode::Drag;
        let ct = contact(tool, br.vol / full, br.hide[1], reach, seg, rb, mode);
        let clip = st.clip;

        // pass 1: contact weights
        let bw = x1 - x0;
        let wts = &mut *st.wts;
        wts.clear();
        wts.resize(bw * (y1 - y0), 0.0);
        let mut sum_w = 0.0f32;
        let mut sum_cov = 0.0f32;
        let mut sum_tack = 0.0f32;
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let t = if seg2 > 1e-8 {
                    (((px - a.0) * dx + (py - a.1) * dy) / seg2).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
                let dist = (qx * qx + qy * qy).sqrt();
                // a moving bristle has a crisp track; in a pressed tip (fixed
                // deposit) paint wicks between the hairs, so each hair's
                // contact fades out and neighbors sum to one smooth patch
                let cov = if fine {
                    fine_cover(a, b, rb, px, py)
                } else if dist > rb + 0.5 {
                    continue;
                } else if mode != Mode::Drag {
                    1.0 - smoothstep(0.0, rb, dist)
                } else {
                    1.0 - smoothstep(rb * 0.5, rb + 0.5, dist)
                };
                if cov <= 0.0 {
                    continue;
                }
                sum_cov += cov;
                let i = (y - oy) * bw_buf + x - ox;
                let surf = (sf.base(i) + 0.35 * sf.vol(i)).min(1.5);
                let contact = smoothstep(ct.th - ct.give, ct.th + 0.2, surf);
                let mut wt = cov * contact;
                if let Some(m) = clip {
                    wt *= m.data[y * w + x];
                }
                wts[(y - y0) * bw + (x - x0)] = wt;
                sum_w += wt;
                if let Some(p) = sf.dry(i) {
                    sum_tack += wt * crate::drying::feel(sf.vol(i), p.cure, p.sub).1;
                }
            }
        }
        if sum_w <= 1e-6 {
            return;
        }
        // a tacky surface grabs: it pulls paint off the bristle faster, in
        // patches as the bristle sticks and slips
        let tack = sum_tack / sum_w;

        // deposit: a share of the load, proportional to distance traveled
        let travel = (seg / s).max(rb / s * 0.5);
        // only the part of the footprint actually in contact takes paint: a
        // bristle skimming the weave peaks keeps most of its load
        let touch = (sum_w / sum_cov.max(1e-6)).min(1.0);
        let dep_total = match mode {
            Mode::Drag => br.vol * (1.0 - (-travel / tool.run).exp()) * touch,
            Mode::Touch { dep } => dep.min(br.vol * 0.5) * touch,
        };
        let dep_total = if tack > 0.0 {
            let g = crate::drying::grab(tack) * crate::drying::stick(b.0, b.1, rb, br.seed, tack);
            let d = match mode {
                Mode::Drag => br.vol * (1.0 - (-travel * g / tool.run).exp()) * touch,
                Mode::Touch { .. } => dep_total * g,
            };
            d.min(br.vol * 0.9)
        } else {
            dep_total
        };
        // a capsule cut by the window edge lays only the window's share there
        let share = if windowed {
            sum_cov / capsule_cover(a, b, rb, fine, (cx0, cy0, cx1, cy1)).max(1e-6)
        } else {
            1.0
        };
        let dep_per_w = dep_total * share.min(1.0) / sum_w / px_area;
        // ploughed paint lands just outside the track: the next pixel, or for
        // a pointed tool (shared bilinearly, below) a hair's width away, the
        // same distance at any resolution
        let off = if fine { 2.0 * rb } else { rb + 1.0 };

        let mut got_v = 0.0f32;
        let mut got_l = [0.0f32; LAT];
        let mut got_h: Prop = [0.0; 3];
        // the tip's paint (what the bristle picked up) goes down first
        let from_tip = dep_total.min(br.tip.v);
        let (blat, bhide) = if from_tip > 0.0 {
            let (mut v, mut l, mut h) = ((dep_total - from_tip).max(0.0), br.lat, br.hide);
            mix_into(&mut v, &mut l, &mut h, from_tip, &br.tip.lat, br.tip.hide);
            (l, h)
        } else {
            (br.lat, br.hide)
        };
        for y in y0..y1 {
            for x in x0..x1 {
                let wt = st.wts[(y - y0) * bw + (x - x0)];
                if wt <= 0.0 {
                    continue;
                }
                let i = (y - oy) * bw_buf + x - ox;
                // paint that is setting is stiff: it comes up and moves less
                let fl = sf.dry(i).map_or(1.0, |p| crate::drying::fluid(p.cure));
                // one stroke lifts only part of the film
                let (touched, floor) = sf.touched(i);
                if *touched != st.id {
                    *touched = st.id;
                    *floor = sf.vol(i) * (1.0 - tool.pickup * fl);
                }
                let v = sf.vol(i);
                if v > 1e-6 {
                    let own = if *sf.stroke(i) == st.id { 0.15 } else { 1.0 };
                    let take =
                        (v * tool.pickup * wt * ct.hunger * own * fl).min((v - *floor).max(0.0));
                    // the surface film comes up first (with an earlier one
                    // this stroke has set aside under its own paint); the
                    // body under it only as far as the bristles reach
                    // through their own paint
                    let a = take.min(st.over_body(i));
                    let p = st.take(i, a + (take - a) * ct.through);
                    for part in p.parts() {
                        if part.v > 0.0 {
                            let tv = part.v * px_area;
                            got_v += tv;
                            for (g, l) in got_l.iter_mut().zip(&part.lat) {
                                *g += l * tv;
                            }
                            for (g, h) in got_h.iter_mut().zip(&part.hide) {
                                *g += h * tv;
                            }
                        }
                    }
                }
                if dep_per_w > 0.0 {
                    // the share of the pixel this paint covers: its contact
                    // (a fine hair's own share of the tuft's width, where
                    // the hairs of a gathered point lie over each other)
                    let cv = sf.cover(i);
                    *cv = if fine {
                        ((if sf.vol(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0)
                    } else {
                        1.0
                    };
                    // a loaded bristle lays its paint on the wet film (it
                    // rides on it); a lean one or a blender drags what it
                    // carries into the film
                    let d = dep_per_w * wt;
                    if sf.vol(i) >= WET_FILM && ct.drag_in > 0.0 {
                        st.add_body(i, d * ct.drag_in, &blat, bhide);
                    }
                    st.lay(
                        i,
                        if sf.vol(i) >= WET_FILM {
                            d * (1.0 - ct.drag_in)
                        } else {
                            d
                        },
                        &blat,
                        bhide,
                    );
                    *sf.stroke(i) = st.id;
                }
                // the bristle works the surface film (with what it just
                // laid) into the body as far as it reaches into it
                st.stir(i, ct.stir * wt * fl);
                // plough: move paint outward from the bristle's path, and ahead
                if ct.push_k > 0.0 {
                    // a bristle parts and smears a film thinner than the
                    // layer it rides on; it pushes only what stands above
                    // it (`Contact::plough`)
                    let v = sf.vol(i);
                    let m = ct.plough(v, wt, fl, st.stiffness(i));
                    if m > 1e-6 {
                        let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                        let side = if (px - a.0) * nx + (py - a.1) * ny >= 0.0 {
                            1.0
                        } else {
                            -1.0
                        };
                        let tx = px + (nx * side * 0.75 + mx * 0.45) * off;
                        let ty = py + (ny * side * 0.75 + my * 0.45) * off;
                        // where the paint goes: the pixel under the target,
                        // or for a pointed tool's fine hairs, shared
                        // bilinearly by the four pixels around it (a hair
                        // finer than a pixel would otherwise leave a ridge
                        // of dots along its track where rounding lands it)
                        let mut to = [(0.0f32, 0.0f32, 0.0f32); 4];
                        let n_to = if fine {
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
                        for &(tx, ty, share) in &to[..n_to] {
                            if share <= 0.0 {
                                continue;
                            }
                            // (paint pushed out of a crop window stays put)
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
                                // a clipped stroke can't push paint past its mask:
                                // only the accepted share moves, the rest stays
                                let m = m * share * clip.map_or(1.0, |c| c.data[ty * w + tx]);
                                if j != i && m > 0.0 {
                                    // the paint moved covers its share of
                                    // the pixel it came from
                                    // (its film there thins but still covers it)
                                    let ci = *sf.cover(i);
                                    let cj = sf.cover(j);
                                    *cj = if fine {
                                        ((if sf.vol(j) < 1e-6 { 0.0 } else { *cj })
                                            + (m / v.max(1e-9)).min(1.0) * ci)
                                            .min(1.0)
                                    } else {
                                        1.0
                                    };
                                    // a bristle pushes the whole column of
                                    // paint in its way; it keeps its
                                    // layering where it lands (the
                                    // set-aside film on the neighbor's
                                    // surface, as surface paint)
                                    let p = st.take_column(i, m);
                                    st.land(j, &p);
                                }
                            }
                        }
                    }
                }
            }
        }
        br.vol = (br.vol - dep_total).max(0.0);
        br.tip.v = (br.tip.v - from_tip).max(0.0).min(br.vol);
        if got_v > 0.0 {
            for k in 0..LAT {
                got_l[k] /= got_v;
            }
            for g in &mut got_h {
                *g /= got_v;
            }
            // what it picks up stays on the bristle's surface
            br.tip.mix(got_v, &got_l, got_h);
            br.vol += got_v;
        }
        // and works into the reservoir as the bristle travels
        br.fold_tip(1.0 - (-travel / TIP_RUN).exp());
        let pad = (off + 2.0) as usize;
        grow(
            &mut st.bounds,
            x0.saturating_sub(pad).max(ox),
            y0.saturating_sub(pad).max(oy),
            (x1 + pad).min(ox + sf.w),
            (y1 + pad).min(oy + sf.h),
        );
    }
}

/// How much of a capsule's contact a bristle is assumed to make outside a
/// crop window (where there is no canvas to feel).
const GHOST_TOUCH: f32 = 0.8;

/// Summed coverage of the capsule a–b (radius rb, pixels) over rect `r`:
/// its geometric footprint, whatever the canvas under it.
fn capsule_cover(
    a: (f32, f32),
    b: (f32, f32),
    rb: f32,
    fine: bool,
    r: (usize, usize, usize, usize),
) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let seg2 = dx * dx + dy * dy;
    let mut sum = 0.0f32;
    for y in r.1..r.3 {
        for x in r.0..r.2 {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let t = if seg2 > 1e-8 {
                (((px - a.0) * dx + (py - a.1) * dy) / seg2).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
            let dist = (qx * qx + qy * qy).sqrt();
            if fine {
                sum += fine_cover(a, b, rb, px, py);
            } else if dist <= rb + 0.5 {
                sum += 1.0 - smoothstep(rb * 0.5, rb + 0.5, dist);
            }
        }
    }
    sum
}

/// Smallest hair radius (pixels) a pointed tool is drawn with.
pub(crate) const FINE_RB: f32 = 0.02;

/// Share of a pixel covered by a hair's track of radius `rb` (pixels) whose
/// center line passes `dist` from the pixel center: the overlap of the
/// pixel's span with the track's, box filtered. Summed across the track it
/// is its width, wherever the track falls between pixel centers, so a track
/// finer than a pixel lays the same paint per length on any grid (no dark
/// dots where it happens to hit a pixel center, no beads at full size).
fn strip_cover(dist: f32, rb: f32) -> f32 {
    ((dist + 0.5).min(rb) - (dist - 0.5).max(-rb)).clamp(0.0, 1.0)
}

/// Share of the pixel centered at (`px`, `py`) covered by a pointed tool's
/// hair moving from `a` to `b` (pixels): the area of its track, a rectangle
/// 2·`rb` wide with square ends, inside the (axis-aligned) pixel. Exact, so
/// the shares of all pixels sum to the track's area however it lies across
/// the lattice (a diagonal track shifted half a pixel covers what it did),
/// and a hair's successive steps tile its path without overlapping: the
/// paint a step lays and the area it darkens don't depend on where the
/// pixel centers fall. A hair that doesn't move covers a square 2·`rb` wide.
pub(crate) fn fine_cover(a: (f32, f32), b: (f32, f32), rb: f32, px: f32, py: f32) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let seg = (dx * dx + dy * dy).sqrt();
    let (ux, uy, lo, hi) = if seg > 1e-4 {
        (dx / seg, dy / seg, 0.0, seg)
    } else {
        (1.0, 0.0, -rb, rb)
    };
    // the pixel center in track coordinates: along `t`, across `q`
    let (rx, ry) = (px - a.0, py - a.1);
    let t = rx * ux + ry * uy;
    let q = ry * ux - rx * uy;
    // (the pixel reaches √½ from its center in any direction)
    const R: f32 = std::f32::consts::FRAC_1_SQRT_2;
    if q.abs() > rb + R || t < lo - R || t > hi + R {
        return 0.0;
    }
    // axis-aligned track: a product of spans
    if ux.abs() < 1e-6 || uy.abs() < 1e-6 {
        return strip_cover(q.abs(), rb) * ((t + 0.5).min(hi) - (t - 0.5).max(lo)).clamp(0.0, 1.0);
    }
    // the track's corners relative to the pixel center, clipped to the
    // pixel (Sutherland–Hodgman against its four sides), shoelace area
    let (nx, ny) = (-uy, ux);
    let corner = |s: f32, r: f32| (a.0 - px + ux * s + nx * r, a.1 - py + uy * s + ny * r);
    let mut poly = [(0.0f32, 0.0f32); 8];
    let mut n = 4;
    poly[..4].copy_from_slice(&[
        corner(lo, -rb),
        corner(hi, -rb),
        corner(hi, rb),
        corner(lo, rb),
    ]);
    for side in 0..4 {
        // inside: sign·coordinate ≤ ½ along x (sides 0, 1) or y (2, 3)
        let (axis, sign) = (side / 2, if side % 2 == 0 { 1.0f32 } else { -1.0 });
        let d = |p: (f32, f32)| sign * if axis == 0 { p.0 } else { p.1 } - 0.5;
        let mut out = [(0.0f32, 0.0f32); 8];
        let mut m = 0;
        for k in 0..n {
            let (p, q) = (poly[k], poly[(k + 1) % n]);
            let (dp, dq) = (d(p), d(q));
            if dp <= 0.0 {
                out[m] = p;
                m += 1;
            }
            if (dp <= 0.0) != (dq <= 0.0) && m < 8 {
                let s = dp / (dp - dq);
                out[m] = (p.0 + (q.0 - p.0) * s, p.1 + (q.1 - p.1) * s);
                m += 1;
            }
        }
        n = m;
        if n < 3 {
            return 0.0;
        }
        poly = out;
    }
    let mut area = 0.0f32;
    for k in 0..n {
        let (p, q) = (poly[k], poly[(k + 1) % n]);
        area += p.0 * q.1 - q.0 * p.1;
    }
    (0.5 * area.abs()).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bristle::Held;
    use crate::canvas::Canvas;
    use crate::color::hex;
    use crate::wet::Layer;

    /// The plough's stiffness gate reads the whole column it pushes: paint
    /// set aside under the stroke counts with its own stiffness (review B3,
    /// Astra's reproduction). Ten coats whose volume-weighted stiffness is
    /// 0.9 move alike whether they lie as one body or as one fluid coat
    /// under nine stiff ones set aside (before: 2.64 and 0.26 coats).
    #[test]
    fn the_plough_feels_the_set_aside_films_stiffness() {
        let run = |aside: bool| {
            let mut c = Canvas::new(100, 1.0, hex("#ffffff"));
            let i = 50 * c.f.w + 50;
            c.wet.vol[i] = 10.0;
            c.wet.hide[i] = [1.0, if aside { 0.0 } else { 0.9 }, 1.0];
            let mut tool = Tool::hog_flat(10.0);
            tool.pickup = 0.0;
            tool.push = 0.3;
            let mut held = Held::new(tool.clone(), 1);
            let full = held.full();
            let sf = c.surf();
            let mut wts = Vec::new();
            unsafe {
                let mut st = Stroke::begin(sf, 1, None, (0, 0, 100, 100), &mut wts);
                if aside {
                    st.set_aside(i, Layer::new(9.0, [0.0; LAT], [1.0, 1.0, 1.0]));
                }
                exchange(&mut st, &mut held.bristles[0], &tool, (49.5, 50.5), (50.5, 50.5), 0.55, 1.0, full, Mode::Touch { dep: 0.0 }, 1.0);
                10.0 - sf.vol(i)
            }
        };
        let (plain, aside) = (run(false), run(true));
        assert!(plain > 1.0, "the plough moves stiff paint: {plain}");
        assert!((plain - aside).abs() < 1e-5, "same whole-column stiffness: plain moves {plain}, with a film set aside {aside}");
    }
}
