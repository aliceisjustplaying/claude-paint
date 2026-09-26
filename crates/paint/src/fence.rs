//! Edges of a pass at a region's boundary: found, soft and lost.
//!
//! A clipped pass (`Handling::clip`) stops every bristle exactly on its
//! mask's 0.5 level, the same line for every stroke. A fenced pass instead
//! lets each stroke stop a little past the line or short of it, by its own
//! amount; where the brush runs out past the edge it lifts and its hairs
//! skim the tops of the weave, so the edge breaks up. A quality field sets,
//! point by point along the contour, whether the edge is **found** (the
//! stroke ends close to the line), **soft** (the stroke runs over and thins
//! out, picking up the wet neighbor) or **lost** (the stroke carries well
//! past the line and thins out over several brush widths).
//!
//! A `Fence` is that edge for one pass: the region's signed distance to its
//! edge in units, offset by a slow noise (the waver), and a quality field
//! `q` (0 found, 0.5 soft, 1 lost) over the canvas. Each stroke draws its own overrun `u` (0..1, from where it
//! starts, so a crop paints the same strokes). A bristle's contact at a
//! pixel is then scaled by
//!
//! ```text
//!   o = W (a(q) + b(q) u)        how far past the edge this stroke reaches (W: brush width)
//!   s = px + W c(q)               over how far it lifts off
//!   f = smoothstep(-o - s, -o, d)
//! ```
//!
//! and in the lift-off zone (`f < 1`) the hairs are held up off the weave's
//! valleys (`lift = LIFT (1 - f)`): a dry, broken fringe rather than a
//! smooth fade. Paint a clipped bristle can't lay outside the fence stays
//! on it, as with a mask clip.
//!
//! The numbers (units of brush width): found `a 0 b 0.12 c 0.06`, soft
//! `0.02 / 0.25 / 0.8`, lost `0.1 / 0.4 / 2.2`, interpolated in between.
//! The overrun `a + b u` is at most 0.5 brush widths; the lift-off
//! distance `c` reaches 2.2. Past the fence the deposit falls with `f²`
//! (`bristle::exchange`), so soft and lost edges are a film thinning to
//! nothing over that distance.
//! `reach` scales `a` and `b`, `waver` the line's wander.

use crate::mask::Mask;
use crate::noise::Fbm;
use crate::smoothstep;
use rayon::prelude::*;

/// How far the hairs lift off the weave where a stroke runs out past a fence
/// (in the contact threshold's units: the surface height is 0..1.5).
pub(crate) const LIFT: f32 = 0.7;

/// One pass's edge (see the module docs).
pub struct Fence {
    /// Signed distance to the wavered line, units, + inside (whole canvas).
    d: Vec<f32>,
    /// Edge quality 0 found .. 1 lost (whole canvas).
    q: Vec<f32>,
    /// Brush width, units.
    width: f32,
    /// One pixel, units.
    px: f32,
    /// Scales the overrun.
    reach: f32,
}

/// The quality numbers at `q`: (a, b, c) of the module docs.
fn shape(q: f32) -> (f32, f32, f32) {
    let q = q.clamp(0.0, 1.0);
    if q <= 0.5 {
        let t = q / 0.5;
        (0.02 * t, 0.12 + 0.13 * t, 0.06 + 0.74 * t)
    } else {
        let t = (q - 0.5) / 0.5;
        (0.02 + 0.08 * t, 0.25 + 0.15 * t, 0.8 + 1.4 * t)
    }
}

impl Fence {
    /// The fence of `region` (its 0.5 level) for a brush `width` units wide,
    /// with edge quality `quality` (0 found .. 1 lost, a whole-canvas mask
    /// used as a field). `waver` scales the line's offset (at 1: an
    /// amplitude of 0.3 units plus a tenth of the brush width where found,
    /// three times that where lost, with a noise period of four brush widths
    /// clamped to 6-40 units); `reach`
    /// scales every stroke's overrun (1 default); `seed` fixes the waver.
    pub fn new(region: &Mask, quality: &Mask, width: f32, waver: f32, reach: f32, seed: u64) -> Fence {
        let f = region.f;
        assert!(quality.f.w == f.w && quality.f.h == f.h, "Fence: the quality field must be a whole-canvas mask");
        let dist = region.distance();
        let period = (4.0 * width).clamp(6.0, 40.0);
        let slow = Fbm::new((seed as u32) ^ 0xFE4C, 3, period);
        let fine = Fbm::new((seed as u32) ^ 0x0E0F, 2, (period * 0.2).max(1.5));
        let w = f.w;
        let d: Vec<f32> = dist
            .data
            .par_iter()
            .enumerate()
            .map(|(i, &dv)| {
                // only near the edge does the waver matter
                if dv.abs() > 6.0 * width + 12.0 {
                    return dv;
                }
                let q = quality.data[i].clamp(0.0, 1.0);
                let (x, y) = (f.ux(i % w), f.uy(i / w));
                let amp = waver * (0.3 + 0.1 * width) * (1.0 + 2.0 * q);
                dv + amp * (0.8 * slow.get(x, y) + 0.35 * fine.get(x, y))
            })
            .collect();
        Fence { d, q: quality.data.clone(), width, px: 1.0 / f.scale, reach }
    }

    /// A stroke's overrun draw (0..1) from where it starts and the pass's seed:
    /// the same stroke draws the same in a crop.
    pub fn stroke_draw(first: (f32, f32), seed: u64) -> f32 {
        let mut h = seed ^ 0x0FE7_CE00 ^ ((first.0.to_bits() as u64) << 21) ^ first.1.to_bits() as u64;
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        h ^= h >> 33;
        (h >> 40) as f32 / (1u64 << 24) as f32
    }

    /// (contact factor, lift) at whole-canvas pixel `i` for a stroke with draw `u`.
    #[inline]
    pub(crate) fn at(&self, i: usize, u: f32) -> (f32, f32) {
        let d = self.d[i];
        if d > 0.0 {
            return (1.0, 0.0);
        }
        let (a, b, c) = shape(self.q[i]);
        let o = self.width * self.reach * (a + b * u);
        let s = self.px + self.width * c;
        let f = smoothstep(-o - s, -o, d);
        (f, LIFT * (1.0 - f))
    }

    /// The share of the region's edge zone (within a brush of its line) at
    /// each quality: (found q < 0.25, soft, lost q ≥ 0.75). For reporting.
    pub fn shares(&self) -> (f32, f32, f32) {
        let band = self.width.max(2.0);
        let (mut n, mut fo, mut lo) = (0usize, 0usize, 0usize);
        for (d, q) in self.d.iter().zip(&self.q) {
            if d.abs() < band {
                n += 1;
                if *q < 0.25 {
                    fo += 1;
                } else if *q >= 0.75 {
                    lo += 1;
                }
            }
        }
        let n = n.max(1) as f32;
        (fo as f32 / n, (n - fo as f32 - lo as f32) / n, lo as f32 / n)
    }
}

/// A quality field for `Fence::new`: the values 0 (found), 0.5 (soft) and 1
/// (lost) in the given shares (normalized), assigned by thresholding an Fbm
/// noise of `period` units, so each value occupies runs of about that
/// length, with transitions over 4% of the noise's range. The shares are
/// met over the band within `band` units of `region`'s edge.
pub fn stretches(region: &Mask, shares: (f32, f32, f32), period: f32, band: f32, seed: u64) -> Mask {
    let f = region.f;
    let total = (shares.0 + shares.1 + shares.2).max(1e-6);
    let (fo, so) = (shares.0 / total, shares.1 / total);
    let n = Fbm::new((seed as u32) ^ 0x57E7, 3, period.max(2.0));
    let dist = region.distance();
    // the noise's own distribution over the edge band: thresholds at the shares
    let w = f.w;
    let mut vals: Vec<f32> = dist
        .data
        .iter()
        .enumerate()
        .step_by(7)
        .filter(|(_, d)| d.abs() < band)
        .map(|(i, _)| n.get(f.ux(i % w), f.uy(i / w)))
        .collect();
    if vals.is_empty() {
        return Mask::from_fn(f, |_, _| 0.5);
    }
    vals.sort_by(|a, b| a.total_cmp(b));
    let at = |p: f32| vals[((p * (vals.len() - 1) as f32).round() as usize).min(vals.len() - 1)];
    let (t1, t2) = (at(fo), at(fo + so));
    let tw = 0.04 * (vals[vals.len() - 1] - vals[0]).max(1e-3);
    Mask::from_fn(f, move |x, y| {
        let v = n.get(x, y);
        // found below t1, soft between, lost above t2 (smooth over tw)
        0.5 * smoothstep(t1 - tw, t1 + tw, v) + 0.5 * smoothstep(t2 - tw, t2 + tw, v)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_is_near_the_line_and_lost_runs_far() {
        let (a0, b0, c0) = shape(0.0);
        let (a1, b1, c1) = shape(1.0);
        assert!(a0 + b0 < 0.25 && c0 < 0.1);
        assert!(a1 + b1 < 0.6 && c1 > 2.0 && c1 > 10.0 * c0);
        // monotone in q
        let mut last = (0.0, 0.0, 0.0);
        for k in 0..=10 {
            let s = shape(k as f32 / 10.0);
            assert!(s.0 >= last.0 && s.1 >= last.1 && s.2 >= last.2);
            last = s;
        }
    }

    #[test]
    fn stroke_draws_spread_over_zero_one() {
        let us: Vec<f32> = (0..2000).map(|k| Fence::stroke_draw((k as f32 * 0.37, 100.0 + k as f32 * 0.11), 7)).collect();
        let mean = us.iter().sum::<f32>() / us.len() as f32;
        assert!(us.iter().all(|u| (0.0..1.0).contains(u)));
        assert!((mean - 0.5).abs() < 0.03, "{mean}");
    }

    /// A dark disk over a light ground, painted with a stencil clip, a found
    /// fence and a lost fence: mean darkening (OKLab L) in rings just outside
    /// the edge (0-3 units, 8-16 units: a body brush is 8 wide) and the share of a 2-unit rim
    /// inside that is covered.
    fn rings(fence: Option<f32>) -> (f32, f32, f32) {
        use crate::canvas::Canvas;
        use crate::color::{hex, to_oklab};
        let st = crate::style::Style::friedrich();
        let mut c = Canvas::new(1000, 1.0, st.raw).with_size_mm(st.width_mm).with_linen(crate::surface::Linen { seed: 3, ..st.linen });
        c.prime(hex("#b08457"), 0.8, 120.0, 0.3, 0.3, 3);
        let before = c.pixels().to_vec();
        let f = c.frame();
        let m = Mask::from_fn(f, |x, y| if (x - 500.0).powi(2) + (y - 330.0).powi(2) < 200.0f32.powi(2) { 1.0 } else { 0.0 });
        let mut hd = st.body().color(|_, _| hex("#2e2a28")).coverage(2.5).clip(true);
        if let Some(q) = fence {
            let qm = Mask::from_fn(f, move |_, _| q);
            let w = hd.tool.width;
            hd = hd.fence(std::sync::Arc::new(Fence::new(&m, &qm, w, 1.0, 1.0, 9)));
        }
        c.work(&m, &hd, 4);
        c.dry();
        let sd = m.distance();
        let (mut a, mut an, mut b, mut bn, mut r, mut rn) = (0.0f32, 0, 0.0f32, 0, 0, 0);
        for (k, p) in c.pixels().iter().enumerate() {
            let d = sd.data[k];
            let dl = to_oklab(before[k])[0] - to_oklab(*p)[0];
            if (-3.0..0.0).contains(&d) {
                a += dl;
                an += 1;
            } else if (-16.0..-8.0).contains(&d) {
                b += dl;
                bn += 1;
            } else if (0.0..2.0).contains(&d) {
                rn += 1;
                r += (dl > 0.15) as usize;
            }
        }
        (a / an as f32, b / bn as f32, r as f32 / rn as f32)
    }

    /// The stencil stops dead on the line; a found fence nearly so; a lost
    /// fence carries a thinning film well past it; all cover the rim inside.
    #[test]
    fn fences_carry_paint_past_the_edge_by_quality() {
        let (s1, s2, sr) = rings(None);
        let (f1, f2, fr) = rings(Some(0.0));
        let (l1, l2, lr) = rings(Some(1.0));
        println!("stencil {s1:.3} {s2:.3} rim {sr:.2} | found {f1:.3} {f2:.3} rim {fr:.2} | lost {l1:.3} {l2:.3} rim {lr:.2}");
        assert!(s1 < 0.02 && s2 < 0.01, "a stencil lays nothing past its edge: {s1} {s2}");
        assert!(f1 > s1 + 0.05, "a found edge's strokes run a little over the line: {f1}");
        assert!(f2 < 0.02, "but stay near it: {f2}");
        assert!(l2 > 0.08, "a lost edge carries paint a brush and more past it: {l2}");
        assert!(l1 > l2 + 0.05, "thinning as it goes: {l1} then {l2}");
        for rim in [sr, fr, lr] {
            assert!(rim > 0.9, "the rim inside is covered: {rim}");
        }
    }
}
