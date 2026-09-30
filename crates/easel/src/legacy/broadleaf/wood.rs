//! The painter's layer over a grown broadleaved tree's wood: which limbs
//! are drawn as lines at a detail (`Tree::drawn`), the strokes a pointed
//! brush lays along them band by band (`Tree::wood_strokes`), and the
//! undrawn twigs' mass as a tone (`Tree::twig_mass`). The growth model
//! (`broadleaf`) makes the skeleton; this only reads it.

#![allow(dead_code)]

use super::{Limb, Tree, edge_dist, inside};
use paint::path::{dist, length};
use paint::{Frame, Mask, Shape, lerp};
use std::collections::HashMap;

/// A stroke along the wood for a pointed brush pressed to the wood's width
/// (see `Tree::wood_strokes`).
#[derive(Clone, Debug)]
pub struct WoodStroke {
    pub pts: Vec<(f32, f32)>,
    /// Width of the wood at each point.
    pub w: Vec<f32>,
    pub limb: usize,
    /// It ends at a tip (the brush lifts off to a point there), rather than
    /// running on into thinner wood that another stroke paints.
    pub tip: bool,
    /// Fine wood: a twig, or a limb under about two twig widths at its base.
    pub fine: bool,
    /// How many of the points are the limb's own (the rest are its leading
    /// twig, drawn on from its tip).
    pub own: usize,
}

/// Points every `h` units or closer along each straight segment (the given
/// points kept), so a brush's spline through them keeps the corners; `own`
/// (a count of leading points) is carried over.
fn subdivide(p: &[(f32, f32)], w: &[f32], own: usize, h: f32) -> (Vec<(f32, f32)>, Vec<f32>, usize) {
    let (mut op, mut ow) = (vec![p[0]], vec![w[0]]);
    let mut own2 = 1;
    for k in 1..p.len() {
        let (a, b) = (p[k - 1], p[k]);
        let m = ((dist(a, b) / h).ceil() as usize).max(1);
        for j in 1..=m {
            let t = j as f32 / m as f32;
            op.push((lerp(a.0, b.0, t), lerp(a.1, b.1, t)));
            ow.push(lerp(w[k - 1], w[k], t));
        }
        if k < own {
            own2 = op.len();
        }
    }
    (op, ow, own2)
}

impl Tree {
    /// Fine wood: a twig, or a limb under about two twig widths at its base
    /// (the finest shoots of the model and the twigs past its resolution).
    pub fn is_fine(&self, l: &Limb) -> bool {
        l.twig || l.w[0] < 2.0 * self.twig_w
    }

    /// Which limbs a painter draws as lines at `detail` (0..1): all the
    /// stout wood, and of the fine wood a share `detail` of its length,
    /// picked for character (long pieces, a limb's leading twig, pieces
    /// toward the crown's edge, where they show against the sky), always
    /// with the wood each leaves from, so nothing drawn floats. `detail`
    /// 1 draws everything, 0 only the stout wood; the rest is left to a
    /// tone (`twig_mass`).
    pub fn drawn(&self, detail: f32) -> Vec<bool> {
        let n = self.limbs.len();
        if detail >= 1.0 {
            return vec![true; n];
        }
        let fine: Vec<bool> = self.limbs.iter().map(|l| self.is_fine(l)).collect();
        let len: Vec<f32> = self.limbs.iter().map(|l| length(&l.pts)).collect();
        let mut on: Vec<bool> = fine.iter().map(|f| !f).collect();
        let (x0, y0, x1, y1) = self.crown.iter().fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)));
        let reach = (0.2 * (x1 - x0).min(y1 - y0)).max(1.0);
        let mut cand: Vec<(f32, usize)> = (0..n)
            .filter(|&i| fine[i] && self.limbs[i].pts.len() >= 2)
            .map(|i| {
                let l = &self.limbs[i];
                let tip = *l.pts.last().unwrap();
                let outer = if inside(&self.crown, tip.0, tip.1) { 1.0 - (edge_dist(&self.crown, tip.0, tip.1) / reach).min(1.0) } else { 1.0 };
                let jit = paint::rng::hash2(i as i64, 41, self.seed);
                let s = len[i] * if l.lead { 1.6 } else { 1.0 } * (0.6 + 0.8 * outer) * (0.5 + jit);
                (s, i)
            })
            .collect();
        cand.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
        let total: f32 = cand.iter().map(|c| len[c.1]).sum();
        let budget = detail.max(0.0) * total;
        let mut acc = 0.0;
        // where drawn fine pieces leave their parents: one side twig per
        // spot, not a starburst of them from one point
        let gap = 0.5 * self.step;
        let mut starts: HashMap<usize, Vec<(f32, f32)>> = HashMap::new();
        for (_, i) in cand {
            if acc >= budget {
                break;
            }
            let l = &self.limbs[i];
            if let Some(p) = l.parent {
                let s0 = l.pts[0];
                let near = |q: &(f32, f32)| (q.0 - s0.0).powi(2) + (q.1 - s0.1).powi(2) < gap * gap;
                if !l.lead && starts.get(&p).is_some_and(|v| v.iter().any(near)) {
                    continue;
                }
                starts.entry(p).or_default().push(s0);
            }
            // the piece and the wood it leaves from, down to drawn wood
            let mut j = i;
            while !on[j] {
                on[j] = true;
                if fine[j] {
                    acc += len[j];
                }
                match self.limbs[j].parent {
                    Some(p) => j = p,
                    None => break,
                }
            }
        }
        on
    }

    /// Strokes that paint the wood between `lo` and `hi` wide (by the
    /// local width, so the thin ends of stout limbs are in the thin band),
    /// drawing the fine wood at `detail` (see `drawn`). Every stroke starts
    /// inside the wood it leaves from: a limb's run starts one point back,
    /// in the thicker wood before it, and a limb's first point is on its
    /// parent. A run that goes on into thinner wood overlaps it by a point
    /// and does not lift off (`tip` false); a drawn leading twig is painted
    /// as the end of its limb's stroke, so a limb runs out into its twig
    /// in one movement and lifts off there. Parents come before children.
    pub fn wood_strokes(&self, lo: f32, hi: f32, detail: f32) -> Vec<WoodStroke> {
        let on = self.drawn(detail);
        let n = self.limbs.len();
        let band = |w: f32| w >= lo && w < hi;
        // a leading twig is drawn on in its limb's stroke when both are in
        // this band (a brush for stouter wood can't lay a twig's hairline)
        let mut lead_of = vec![None; n];
        for (i, l) in self.limbs.iter().enumerate() {
            if l.lead
                && on[i]
                && l.pts.len() >= 2
                && l.w.iter().all(|w| band(*w))
                && let Some(p) = l.parent
                && on[p]
                && self.limbs[p].w.last().is_some_and(|w| band(*w))
            {
                lead_of[p] = Some(i);
            }
        }
        let mut out = vec![];
        for (i, l) in self.limbs.iter().enumerate() {
            let m = l.pts.len();
            if !on[i] || m < 2 || (l.lead && l.parent.is_some_and(|p| lead_of[p] == Some(i))) {
                continue;
            }
            let mut k = 0;
            while k < m {
                if !band(l.w[k]) {
                    k += 1;
                    continue;
                }
                let s = k;
                while k < m && band(l.w[k]) {
                    k += 1;
                }
                let a = s.saturating_sub(1);
                let b = if k < m { k + 1 } else { m };
                let mut pts = l.pts[a..b].to_vec();
                let mut w = l.w[a..b].to_vec();
                // a limb is pulled out of its parent: its stroke starts half a
                // segment back along the parent, at its own width, so the
                // brush is down on painted wood before the fork
                if a == 0
                    && let Some(p) = l.parent
                    && let Some(j) = self.limbs[p].pts.iter().position(|q| *q == l.pts[0]).filter(|&j| j > 0)
                {
                    let (q, r) = (self.limbs[p].pts[j - 1], self.limbs[p].pts[j]);
                    pts.insert(0, (0.5 * (q.0 + r.0), 0.5 * (q.1 + r.1)));
                    w.insert(0, l.w[0]);
                }
                let tip = k >= m;
                let own = pts.len();
                if tip && let Some(t) = lead_of[i] {
                    pts.extend_from_slice(&self.limbs[t].pts[1..]);
                    w.extend_from_slice(&self.limbs[t].w[1..]);
                }
                if pts.len() >= 2 {
                    let (pts, w, own) = if self.angular { subdivide(&pts, &w, own, 0.5) } else { (pts, w, own) };
                    out.push(WoodStroke { pts, w, limb: i, tip, fine: self.is_fine(l), own });
                }
            }
        }
        out
    }

    /// The fine wood's mass as a soft tone (0..1): where the twigs are
    /// thick on the ground, strongest where fewest of them are drawn at
    /// `detail`, spread over about half a twig's length. A painter
    /// indicates a winter crown's twig mass this way, with a dry brush or a
    /// scumble, and draws a few twigs over it.
    pub fn twig_mass(&self, f: Frame, detail: f32) -> Mask {
        let on = self.drawn(detail);
        let mut shape = Shape::new();
        let mut shape_on = Shape::new();
        let (mut tl, mut nt) = (0.0, 0);
        for (i, l) in self.limbs.iter().enumerate() {
            if !self.is_fine(l) || l.pts.len() < 2 {
                continue;
            }
            if l.twig {
                tl += length(&l.pts);
                nt += 1;
            }
            // a fixed width, so the density is the same at any resolution
            let w = vec![0.5; l.pts.len()];
            if on[i] {
                shape_on = shape_on.ribbon(&l.pts, &w);
            } else {
                shape = shape.ribbon(&l.pts, &w);
            }
        }
        let reach = if nt > 0 { 0.5 * tl / nt as f32 } else { self.step };
        let undrawn = Mask::from_shape(f, shape);
        let drawn = Mask::from_shape(f, shape_on);
        let mut m = undrawn.union(&drawn.map(|v| 0.35 * v)).blur(reach);
        // scale by the densest parts (the 98th percentile of the tone), so
        // the thick of the twigs is about 1 and thinner parts grade off
        let mut v: Vec<f32> = m.data.iter().copied().filter(|v| *v > 1e-3).collect();
        if !v.is_empty() {
            let k = ((v.len() - 1) as f32 * 0.98) as usize;
            let q = *v.select_nth_unstable_by(k, |a, b| a.total_cmp(b)).1;
            let q = q.max(1e-4);
            m = m.map(|x| (x / q).clamp(0.0, 1.0));
        }
        m
    }
}

