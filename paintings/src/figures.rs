//! Friedrich's small figures, written as brush gestures.
//!
//! Local coordinates: `u` across (+ = the figure's right, which is also the
//! viewer's right since they face away), `v` up from the feet, both in figure
//! heights. Sources for what they look like:
//! - Monk by the Sea: "a single figure, dressed in a long garment ... turned
//!   almost completely away from the viewer"; "the long blond hair and round
//!   skull" (https://en.wikipedia.org/wiki/The_Monk_by_the_Sea).
//! - Two Men Contemplating the Moon: "The man on the right is wearing a
//!   grey-green cape and the black beret of the altdeutsche Tracht and has a
//!   stick in his right hand. The man on the left is somewhat higher on the
//!   path and is leaning on his companion's shoulder; he is slimmer and is
//!   wearing a grey-green frock-coat, from which a white collar protrudes, and
//!   the black cap of an early Burschenschaft"
//!   (https://en.wikipedia.org/wiki/Two_Men_Contemplating_the_Moon).

use paint::{Canvas, Hand, Mark, Paint, Rgb, Tool};
use std::f32::consts::FRAC_PI_2;

fn body(color: Rgb) -> Paint {
    Paint { color, hiding: 0.97, stiff: 1.0 }
}

/// Thin paint for small touches (heads, caps, collars): no impasto knobs.
fn thin(color: Rgb) -> Paint {
    Paint { color, hiding: 0.95, stiff: 0.45 }
}

fn lean(color: Rgb) -> Paint {
    Paint { color, hiding: 0.55, stiff: 0.4 }
}

/// The monk: long dark habit widening a little to the hem, sloping shoulders,
/// right arm raised with the hand to the face (the elbow shows at his side),
/// round head of pale hair. `light` is a lean rim touched on the lit side.
pub fn monk(c: &mut Canvas, at: (f32, f32), size: f32, robe: Rgb, hair: Rgb, light: Option<Rgb>, seed: u64) {
    let mut h = Hand::new(at, size, seed);
    let p = body(robe);
    let mut b = h.take(Tool::round_sable, 0.075, p, 1.0);
    // the habit: four long strokes from the shoulders down, pressing harder
    // toward the hem so it widens; outer strokes lean out a little
    for (i, (u0, u1)) in [(-0.045f32, -0.075f32), (0.045, 0.08), (-0.014, -0.025), (0.016, 0.028)].iter().enumerate() {
        if i % 2 == 0 {
            b.reload(p, 1.0);
        }
        let mid = (u0 + u1) * 0.5 + h.rng.normal() * 0.004;
        h.mark(c, &mut b, Mark { pts: &[(*u0, 0.82), (mid, 0.45), (*u1, 0.02)], pressure: (0.65, 1.0), ramps: (0.05, 0.06) }, None);
    }
    // and over it once more, lighter, to close the grooves between the hairs
    for (u0, u1) in [(-0.03f32, -0.05f32), (0.03, 0.055), (0.0, 0.0)] {
        b.reload(p, 1.0);
        h.mark(c, &mut b, Mark { pts: &[(u0, 0.8), (u1, 0.03)], pressure: (0.6, 0.85), ramps: (0.1, 0.1) }, None);
    }
    b.reload(p, 1.0);
    // shoulders: one stroke across, dropping at both ends
    h.line(c, &mut b, &[(-0.085, 0.76), (-0.04, 0.825), (0.04, 0.825), (0.085, 0.76)], 0.7, 0.7);
    // raised right arm: out to the elbow and back in toward the face
    h.line(c, &mut b, &[(0.06, 0.79), (0.115, 0.69), (0.085, 0.61)], 0.75, 0.55);
    // hem: firm across the bottom
    h.line(c, &mut b, &[(-0.095, 0.03), (0.0, 0.015), (0.105, 0.03)], 0.8, 0.8);
    // the hood gathered at the nape
    let mut nb = h.take(Tool::round_sable, 0.04, p, 0.8);
    h.dab(c, &mut nb, 0.0, 0.845, 0.03, 0.0, 0.8);
    // let the habit set before the head goes on, so the hair stays clean
    c.dry();
    // the head: a dab of pale hair
    let mut hb = h.take(Tool::round_sable, 0.068, thin(hair), 1.0 * 0.45);
    h.dab(c, &mut hb, 0.004, 0.9, 0.065, FRAC_PI_2, 1.0);
    if let Some(l) = light {
        // light from the left: a lean touch dragged down the left edge,
        // barely pressing, and one along the left shoulder
        c.dry();
        let tone = paint::color::mix(robe, l, 0.4, paint::Mix::Pigment);
        let mut lb = h.take(Tool::round_sable, 0.025, lean(tone), 0.4 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(-0.07, 0.75), (-0.074, 0.6), (-0.078, 0.45)], pressure: (0.35, 0.15), ramps: (0.15, 0.6) }, None);
        h.mark(c, &mut lb, Mark { pts: &[(-0.02, 0.83), (-0.06, 0.8)], pressure: (0.4, 0.3), ramps: (0.1, 0.4) }, None);
        h.dab(c, &mut lb, -0.02, 0.91, 0.03, FRAC_PI_2, 0.4);
    }
}

/// Trousers and boots, painted first so the coat can come down over them.
fn legs(c: &mut Canvas, h: &mut Hand, trousers: Rgb, dark: Rgb, gap: f32, w: f32) {
    let tr = body(trousers);
    let mut b = h.take(Tool::round_sable, w, tr, 1.0);
    for s in [-1.0f32, 1.0] {
        h.mark(c, &mut b, Mark { pts: &[(s * gap, 0.45), (s * gap, 0.2), (s * (gap + 0.002), 0.035)], pressure: (0.95, 0.65), ramps: (0.0, 0.05) }, None);
    }
    let mut bt = h.take(Tool::round_sable, w * 0.62, thin(dark), 1.0 * 0.45);
    for s in [-1.0f32, 1.0] {
        h.mark(c, &mut bt, Mark { pts: &[(s * (gap - 0.004), 0.02), (s * (gap + 0.024), 0.012)], pressure: (0.9, 0.6), ramps: (0.0, 0.3) }, None);
    }
    c.dry();
}

/// The older man in a cape and beret, stick in his right hand. Returns his
/// left shoulder in canvas units (for a companion's hand).
#[allow(clippy::too_many_arguments)]
pub fn man_in_cape(c: &mut Canvas, at: (f32, f32), size: f32, cape: Rgb, dark: Rgb, hair: Rgb, rim: Option<Rgb>, seed: u64) -> (f32, f32) {
    let mut h = Hand::new(at, size, seed);
    // back to front: legs, the stick, then the cape over them
    legs(c, &mut h, paint::color::mix(dark, cape, 0.3, paint::Mix::Pigment), dark, 0.03, 0.045);
    let mut st = h.take(Tool::rigger, 0.012, body(dark), 1.0);
    h.mark(c, &mut st, Mark { pts: &[(0.15, 0.52), (0.175, 0.26), (0.2, 0.0)], pressure: (0.9, 0.8), ramps: (0.0, 0.05) }, None);
    c.dry();
    // the cape: strokes fanning out from the neck, a bell to the knee
    let p = body(cape);
    let mut cb = h.take(Tool::round_sable, 0.07, p, 1.0);
    for (i, k) in [-3.0f32, 3.0, -2.0, 2.0, -1.0, 1.0, 0.0].iter().enumerate() {
        if i % 2 == 0 {
            cb.reload(p, 1.0);
        }
        let (u0, u1) = (k * 0.018, k * 0.045 + h.rng.normal() * 0.006);
        h.mark(c, &mut cb, Mark { pts: &[(u0, 0.83), ((u0 + u1) * 0.5, 0.6), (u1, 0.36)], pressure: (0.6, 1.0), ramps: (0.05, 0.1) }, None);
    }
    cb.reload(p, 1.0);
    h.line(c, &mut cb, &[(-0.105, 0.77), (-0.05, 0.835), (0.05, 0.835), (0.105, 0.77)], 0.75, 0.75);
    h.line(c, &mut cb, &[(-0.15, 0.37), (0.0, 0.355), (0.15, 0.37)], 0.7, 0.7);
    c.dry();
    // the hand on the stick
    let mut hb = h.take(Tool::round_sable, 0.035, thin(dark), 0.8 * 0.45);
    h.dab(c, &mut hb, 0.15, 0.5, 0.03, FRAC_PI_2, 0.9);
    // head, then the beret: a wide flat dab resting on it, tipped a little
    let mut hd = h.take(Tool::round_sable, 0.06, thin(hair), 1.0 * 0.45);
    h.dab(c, &mut hd, 0.0, 0.88, 0.05, FRAC_PI_2, 1.0);
    let mut be = h.take(Tool::round_sable, 0.04, thin(dark), 1.0 * 0.45);
    h.mark(c, &mut be, Mark { pts: &[(-0.068, 0.905), (0.0, 0.922), (0.064, 0.915)], pressure: (0.85, 0.9), ramps: (0.05, 0.2) }, None);
    if let Some(l) = rim {
        c.dry();
        let mut lb = h.take(Tool::round_sable, 0.014, lean(paint::color::mix(dark, l, 0.5, paint::Mix::Pigment)), 0.35 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(-0.1, 0.785), (-0.05, 0.842), (0.03, 0.845)], pressure: (0.4, 0.25), ramps: (0.2, 0.5) }, None);
    }
    h.p(-0.08, 0.8)
}

/// The slimmer young man in a frock coat, white collar and student cap, his
/// right hand resting on `hand_on` (canvas units) if given.
#[allow(clippy::too_many_arguments)]
pub fn youth_in_frock(c: &mut Canvas, at: (f32, f32), size: f32, coat: Rgb, dark: Rgb, hair: Rgb, collar: Rgb, hand_on: Option<(f32, f32)>, rim: Option<Rgb>, seed: u64) {
    let mut h = Hand::new(at, size, seed);
    legs(c, &mut h, paint::color::mix(dark, coat, 0.3, paint::Mix::Pigment), dark, 0.025, 0.04);
    // frock coat: narrow torso to the waist, then tails flaring to the knee
    let p = body(coat);
    let mut cb = h.take(Tool::round_sable, 0.055, p, 1.0);
    for (i, u) in [-0.04f32, 0.04, 0.0].iter().enumerate() {
        if i % 2 == 0 {
            cb.reload(p, 1.0);
        }
        h.mark(c, &mut cb, Mark { pts: &[(*u, 0.82), (u * 0.85, 0.68), (u * 0.8, 0.56)], pressure: (0.8, 0.75), ramps: (0.05, 0.1) }, None);
    }
    cb.reload(p, 1.0);
    for u in [-0.035f32, 0.035, 0.0] {
        h.mark(c, &mut cb, Mark { pts: &[(u * 0.9, 0.58), (u * 1.2, 0.45), (u * 1.6, 0.33)], pressure: (0.7, 0.9), ramps: (0.05, 0.15) }, None);
    }
    h.line(c, &mut cb, &[(-0.085, 0.765), (-0.04, 0.825), (0.04, 0.825), (0.085, 0.765)], 0.7, 0.7);
    // left arm hangs at his side
    h.line(c, &mut cb, &[(-0.07, 0.78), (-0.085, 0.64), (-0.08, 0.5)], 0.6, 0.5);
    // right arm to the companion's shoulder
    if let Some(t) = hand_on {
        let tu = (t.0 - at.0) / size;
        let tv = (at.1 - t.1) / size;
        let e = (0.5 * (0.07 + tu) + 0.02, 0.5 * (0.78 + tv) - 0.06);
        h.mark(c, &mut cb, Mark { pts: &[(0.065, 0.78), e, (tu, tv)], pressure: (0.65, 0.5), ramps: (0.05, 0.2) }, None);
    } else {
        h.line(c, &mut cb, &[(0.07, 0.78), (0.085, 0.64), (0.08, 0.5)], 0.6, 0.5);
    }
    c.dry();
    // white collar standing up at the neck
    let mut wc = h.take(Tool::round_sable, 0.022, thin(collar), 0.8 * 0.45);
    h.mark(c, &mut wc, Mark { pts: &[(-0.03, 0.832), (0.0, 0.842), (0.03, 0.832)], pressure: (0.7, 0.7), ramps: (0.1, 0.3) }, None);
    // head, then the cap: a small flat dab resting on it
    let mut hd = h.take(Tool::round_sable, 0.055, thin(hair), 1.0 * 0.45);
    h.dab(c, &mut hd, 0.0, 0.882, 0.045, FRAC_PI_2, 1.0);
    let mut cp = h.take(Tool::round_sable, 0.036, thin(dark), 1.0 * 0.45);
    h.mark(c, &mut cp, Mark { pts: &[(-0.042, 0.905), (0.0, 0.918), (0.045, 0.91)], pressure: (0.85, 0.85), ramps: (0.05, 0.2) }, None);
    if let Some(l) = rim {
        c.dry();
        let mut lb = h.take(Tool::round_sable, 0.012, lean(paint::color::mix(dark, l, 0.5, paint::Mix::Pigment)), 0.35 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(-0.08, 0.775), (-0.04, 0.832)], pressure: (0.4, 0.25), ramps: (0.2, 0.5) }, None);
    }
}
