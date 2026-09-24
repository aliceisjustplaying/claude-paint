//! Edges a brush makes: found, soft and lost.
//!
//! A clipped pass (`Handling::clip`) stops every bristle exactly on its
//! mask's edge: one even, crisp line that no brush makes, and the same line
//! for every stroke (a "filled selection"). A painter carries a passage up to
//! a neighbor instead. Each stroke stops a little past the line or short of
//! it, by its own amount; where the brush runs out past the edge it lifts
//! and its hairs skim the tops of the weave, so the edge breaks up; and the
//! painter decides, stretch by stretch along one contour, where the edge is
//! **found** (crisp: a careful stroke cut to the line, where a form turns
//! against the light), **soft** (the stroke runs over and thins out,
//! picking up the wet neighbor) or **lost** (the passage carries well into
//! its neighbor and the shape dissolves there).
//!
//! A `Fence` is that edge for one pass: the region's signed distance to its
//! edge in units, moved by a slow waver (the painter's line isn't the mask's
//! line), and a quality field `q` (0 found, 0.5 soft, 1 lost) over the
//! canvas. Each stroke draws its own overrun `u` (0..1, from where it
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
//! The overrun stays short: a longer one only moves the edge out as a
//! staircase of opaque stroke ends (tried: `notes/edges.md`). What makes an
//! edge soft or lost is the film thinning to nothing over a distance: the
//! deposit past the fence falls with `f²` (`bristle::exchange`).
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
    /// Signed distance to the painter's line, units, + inside (whole canvas).
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
    /// used as a field). `waver` scales the line's wander (1 = a hand
    /// following a drawn line: about 0.3 units plus a tenth of the brush
    /// where found, three times that where lost, over 6-40 units); `reach`
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

/// A quality field in stretches along a contour: shares of found, soft and
/// lost (normalized), laid out by a noise of `period` units, so each quality
/// comes in runs of about that length, with short transitions. The shares
/// are met over the band within `band` units of `region`'s edge.
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
}
