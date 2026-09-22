//! Evergreens written as brush gestures: the spruce as Friedrich places it,
//! a dark spire of drooping tiers (the evergreen against the dead oak).

use paint::{Canvas, Hand, Mark, Paint, Rgb, Tool};

/// A spruce: a trunk line, then tiers of drooping branch strokes from the top
/// down, each tier a little longer, the tip a single flick. `base` is where
/// the trunk meets the ground; `height` in canvas units.
pub fn spruce(c: &mut Canvas, base: (f32, f32), height: f32, color: Rgb, seed: u64) {
    let mut h = Hand::new(base, height, seed);
    h.tremor = 0.002;
    let p = Paint { color, hiding: 0.92, body: 0.6 };
    let mut t = h.take(Tool::round_sable, 0.014, p, 1.0);
    h.mark(c, &mut t, Mark { pts: &[(0.0, 0.0), (0.002, 0.5), (0.0, 1.0)], pressure: (0.9, 0.3), ramps: (0.0, 0.3) }, None);
    let mut b = h.take(Tool::round_sable, 0.028, p, 1.0);
    let tiers = 22 + (height / 12.0).min(16.0) as usize;
    let slim = h.rng.range(0.17, 0.24);
    for i in 0..tiers {
        let v = 0.96 - (i as f32 + h.rng.range(0.0, 0.6)) / tiers as f32 * 0.9;
        let reach = (0.015 + slim * (1.0 - v)) * h.rng.range(0.75, 1.1);
        if i % 3 == 0 {
            b.reload(p, 1.0);
        }
        for s in [-1.0f32, 1.0] {
            // out and down from the trunk, the tip turning a little up
            let droop = reach * h.rng.range(0.25, 0.45);
            let pts = [(s * 0.004, v), (s * reach * 0.55, v - droop), (s * reach, v - droop * 0.8)];
            h.mark(c, &mut b, Mark { pts: &pts, pressure: (0.9, 0.35), ramps: (0.0, 0.5) }, None);
        }
    }
    // the leader at the top
    h.mark(c, &mut t, Mark { pts: &[(0.0, 0.92), (0.001, 1.02)], pressure: (0.7, 0.2), ramps: (0.0, 0.5) }, None);
}
