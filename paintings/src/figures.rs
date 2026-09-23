//! Friedrich's small figures, written as brush gestures.
//!
//! Local coordinates: `u` across (+ = the figure's right, which is also the
//! viewer's right since they face away), `v` up from the feet, both in figure
//! heights. Sources for what they look like:
//! - Two Men Contemplating the Moon: "The man on the right is wearing a
//!   grey-green cape and the black beret of the altdeutsche Tracht and has a
//!   stick in his right hand. The man on the left is somewhat higher on the
//!   path and is leaning on his companion's shoulder; he is slimmer and is
//!   wearing a grey-green frock-coat, from which a white collar protrudes, and
//!   the black cap of an early Burschenschaft"
//!   (https://en.wikipedia.org/wiki/Two_Men_Contemplating_the_Moon).

//!
//! Every motif takes `Paint`s the painter mixed on their palette (e.g.
//! `palette.paint(hex("#1d201b"), 0.05)`), never raw colors: the figure is
//! painted in what is on the palette.

use paint::{Canvas, Hand, Mark, Mix, Paint, Tool};
use std::f32::consts::FRAC_PI_2;

/// The paint as mixed, for the body of a garment.
fn body(p: Paint) -> Paint {
    p
}

/// The same paint thinned a little for small touches (heads, caps,
/// collars): no impasto knobs.
fn thin(p: Paint) -> Paint {
    Paint { stiff: p.stiff.min(0.45), ..p }
}

/// Lean: thinned with medium for a rim of light.
fn lean(p: Paint) -> Paint {
    Paint { hiding: p.hiding * 0.6, stiff: p.stiff.min(0.4), ..p }
}

/// Two paints knifed together on the palette, `t` of `b` into `a`.
pub fn mix(a: Paint, b: Paint, t: f32) -> Paint {
    Paint {
        color: paint::color::mix(a.color, b.color, t, Mix::Pigment),
        hiding: a.hiding + (b.hiding - a.hiding) * t,
        stiff: a.stiff + (b.stiff - a.stiff) * t,
    }
}

/// Trousers and boots, painted first so the coat can come down over them.
/// Each boot is pulled out of the wet trouser leg (from the shin down to
/// the heel, then forward to the toe), so leg and boot are one shape with
/// no gap between them.
fn legs(c: &mut Canvas, h: &mut Hand, trousers: Paint, dark: Paint, gap: f32, w: f32) {
    let tr = body(trousers);
    let mut b = h.take(Tool::round_sable, w, tr, 1.0);
    for s in [-1.0f32, 1.0] {
        h.mark(c, &mut b, Mark { pts: &[(s * gap, 0.45), (s * gap, 0.2), (s * (gap + 0.002), 0.03)], pressure: (0.95, 0.75), ramps: (0.0, 0.0) }, None);
    }
    let mut bt = h.take(Tool::round_sable, w * 0.8, thin(dark), 0.6);
    for s in [-1.0f32, 1.0] {
        let x = s * (gap + 0.002);
        h.mark(c, &mut bt, Mark { pts: &[(x, 0.085), (x, 0.03), (x + s * 0.004, 0.012), (x + s * 0.026, 0.008)], pressure: (0.8, 0.7), ramps: (0.0, 0.25) }, None);
    }
    c.dry();
}

/// The older man in a cape and beret, stick in his right hand. Returns his
/// left shoulder in canvas units (for a companion's hand).
#[allow(clippy::too_many_arguments)]
pub fn man_in_cape(c: &mut Canvas, at: (f32, f32), size: f32, cape: Paint, dark: Paint, hair: Paint, rim: Option<Paint>, seed: u64) -> (f32, f32) {
    let mut h = Hand::new(at, size, seed);
    // back to front: legs, the stick, then the cape over them
    legs(c, &mut h, mix(dark, cape, 0.3), dark, 0.03, 0.045);
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
        let mut lb = h.take(Tool::round_sable, 0.014, lean(mix(dark, l, 0.5)), 0.35 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(-0.1, 0.785), (-0.05, 0.842), (0.03, 0.845)], pressure: (0.4, 0.25), ramps: (0.2, 0.5) }, None);
    }
    h.p(-0.08, 0.8)
}

/// The slimmer young man in a frock coat, white collar and student cap, his
/// right hand resting on `hand_on` (canvas units) if given.
#[allow(clippy::too_many_arguments)]
pub fn youth_in_frock(c: &mut Canvas, at: (f32, f32), size: f32, coat: Paint, dark: Paint, hair: Paint, collar: Paint, hand_on: Option<(f32, f32)>, rim: Option<Paint>, seed: u64) {
    let mut h = Hand::new(at, size, seed);
    legs(c, &mut h, mix(dark, coat, 0.3), dark, 0.025, 0.04);
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
        let mut lb = h.take(Tool::round_sable, 0.012, lean(mix(dark, l, 0.5)), 0.35 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(-0.08, 0.775), (-0.04, 0.832)], pressure: (0.4, 0.25), ramps: (0.2, 0.5) }, None);
    }
}

/// How a woman stands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WomanPose {
    /// Still, arms close at her sides.
    Standing,
    /// Arms held a little out and down, palms open to the light, as in
    /// *Woman before the Setting Sun* (c. 1818): "a woman ... with her back
    /// to the viewer ... her arms slightly raised"
    /// (https://en.wikipedia.org/wiki/Woman_before_the_Rising_Sun).
    ArmsOpen,
    /// Leaning a little toward a window, both forearms on the sill at
    /// height `v` (figure heights), as in *Woman at a Window* (1822): "his
    /// wife Caroline ... seen from behind ... looking out of the window"
    /// (https://en.wikipedia.org/wiki/Woman_at_a_Window).
    AtSill(f32),
}

/// What a woman wears, mixed on the palette.
#[derive(Clone, Copy, Debug)]
pub struct Gown {
    /// The long high-waisted dress.
    pub dress: Paint,
    /// A shawl over the shoulders, its point down her back.
    pub shawl: Option<Paint>,
    pub hair: Paint,
    /// Hands, where they show.
    pub skin: Paint,
    /// A lean rim of light on the side toward the light.
    pub rim: Option<Paint>,
}

/// A woman seen from behind in the dress of Friedrich's time: a long
/// high-waisted gown (the waist just under the shoulder blades) falling
/// straight to the ground and flaring a little at the hem, close sleeves,
/// hair gathered up in a knot, a shawl if given. Local gestures in figure
/// heights (u across, v up from the hem), head about an eighth of her
/// height. Back to front: skirt, bodice, sleeves, shawl, head, hair, light.
pub fn woman(c: &mut Canvas, at: (f32, f32), size: f32, g: &Gown, pose: WomanPose, seed: u64) {
    let mut h = Hand::new(at, size, seed);
    h.tremor = 0.002;
    let dress = body(g.dress);
    // leaning toward a sill: the body tips forward, so from behind the
    // shoulders and head drop a little
    let drop = if let WomanPose::AtSill(_) = pose { 0.03 } else { 0.0 };
    // the skirt: from the high waist to the hem, flaring, pressing harder
    // toward the hem; outer strokes first, then the middle over them
    let mut b = h.take(Tool::round_sable, 0.07, dress, 1.0);
    for (i, (u0, u1)) in [(-0.05f32, -0.115f32), (0.05, 0.115), (-0.02, -0.05), (0.02, 0.05), (0.0, 0.0)].iter().enumerate() {
        if i % 2 == 0 {
            b.reload(dress, 1.0);
        }
        let mid = (u0 + u1) * 0.5 + h.rng.normal() * 0.004;
        h.mark(c, &mut b, Mark { pts: &[(*u0, 0.68 - drop), (mid, 0.34), (*u1, 0.012)], pressure: (0.65, 1.0), ramps: (0.04, 0.03) }, None);
    }
    // the hem, just touching the ground
    b.reload(dress, 1.0);
    h.line(c, &mut b, &[(-0.12, 0.016), (0.0, 0.008), (0.12, 0.016)], 0.8, 0.8);
    // the bodice: short, from the shoulders to the high waist
    for u in [-0.036f32, 0.036, 0.0] {
        b.reload(dress, 0.9);
        h.mark(c, &mut b, Mark { pts: &[(u * 1.15, 0.84 - drop), (u, 0.76 - drop), (u * 1.15, 0.66 - drop)], pressure: (0.75, 0.8), ramps: (0.05, 0.1) }, None);
    }
    // sleeves, by pose; hands where they show
    let mut sl = h.take(Tool::round_sable, 0.034, dress, 0.9);
    let mut hands = vec![];
    for s in [-1.0f32, 1.0] {
        sl.reload(dress, 0.9);
        let sh = (s * 0.074, 0.83 - drop);
        let pts: Vec<(f32, f32)> = match pose {
            WomanPose::Standing => vec![sh, (s * 0.092, 0.72 - drop), (s * 0.088, 0.58)],
            WomanPose::ArmsOpen => {
                // out and down from the shoulder, the forearm a little lifted
                let hand = (s * 0.2, 0.64);
                hands.push(hand);
                vec![sh, (s * 0.125, 0.72), (s * 0.165, 0.66), (s * 0.19, 0.645)]
            }
            WomanPose::AtSill(v) => {
                // elbows out, forearms forward onto the sill (hidden by her)
                vec![sh, (s * 0.115, (0.83 + v) * 0.5 - drop), (s * 0.075, v)]
            }
        };
        h.mark(c, &mut sl, Mark { pts: &pts, pressure: (0.8, 0.65), ramps: (0.05, 0.15) }, None);
        if pose == WomanPose::Standing {
            hands.push((s * 0.088, 0.565));
        }
    }
    c.dry();
    // hands: small touches of flesh, the open palms as short dabs
    let mut hb = h.take(Tool::round_sable, 0.024, thin(g.skin), 0.5);
    for (u, v) in hands {
        let dir = if pose == WomanPose::ArmsOpen { -FRAC_PI_2 + u.signum() * 0.5 } else { -FRAC_PI_2 };
        h.dab(c, &mut hb, u, v, 0.03, dir, 0.8);
    }
    // the shawl: across the shoulders, drawn to a point at the small of the
    // back, its ends over the upper arms
    if let Some(sp) = g.shawl {
        let sp = body(sp);
        let mut sh = h.take(Tool::round_sable, 0.045, sp, 1.0);
        h.mark(c, &mut sh, Mark { pts: &[(-0.09, 0.82 - drop), (-0.045, 0.852 - drop), (0.045, 0.852 - drop), (0.09, 0.82 - drop)], pressure: (0.85, 0.85), ramps: (0.05, 0.1) }, None);
        for s in [-1.0f32, 1.0] {
            sh.reload(sp, 1.0);
            h.mark(c, &mut sh, Mark { pts: &[(s * 0.07, 0.84 - drop), (s * 0.035, 0.76 - drop), (s * 0.006, 0.64 - drop)], pressure: (0.9, 0.3), ramps: (0.05, 0.35) }, None);
            sh.reload(sp, 0.8);
            h.mark(c, &mut sh, Mark { pts: &[(s * 0.085, 0.83 - drop), (s * 0.1, 0.75 - drop)], pressure: (0.7, 0.4), ramps: (0.05, 0.4) }, None);
        }
        sh.reload(sp, 0.8);
        h.mark(c, &mut sh, Mark { pts: &[(0.0, 0.84 - drop), (0.0, 0.7 - drop)], pressure: (0.8, 0.4), ramps: (0.05, 0.3) }, None);
        c.dry();
    }
    // the neck, the head, and the hair gathered in a knot at the crown
    let hair = thin(g.hair);
    let mut hd = h.take(Tool::round_sable, 0.075, hair, 0.8);
    let lean_u = if let WomanPose::AtSill(_) = pose { 0.004 } else { 0.0 };
    h.mark(c, &mut hd, Mark { pts: &[(lean_u, 0.87 - drop), (lean_u, 0.93 - drop)], pressure: (0.9, 0.85), ramps: (0.1, 0.3) }, None);
    let mut kn = h.take(Tool::round_sable, 0.042, hair, 0.7);
    h.dab(c, &mut kn, lean_u + 0.004, 0.952 - drop, 0.024, 0.0, 0.8);
    if let Some(l) = g.rim {
        c.dry();
        let mut lb = h.take(Tool::round_sable, 0.014, lean(mix(g.shawl.unwrap_or(g.dress), l, 0.5)), 0.35 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(-0.085, 0.83 - drop), (-0.045, 0.857 - drop), (0.0, 0.862 - drop)], pressure: (0.4, 0.2), ramps: (0.2, 0.5) }, None);
        let mut ld = h.take(Tool::round_sable, 0.012, lean(mix(g.dress, l, 0.4)), 0.3 * 0.4);
        h.mark(c, &mut ld, Mark { pts: &[(-0.06, 0.6), (-0.09, 0.35), (-0.11, 0.08)], pressure: (0.3, 0.15), ramps: (0.3, 0.5) }, None);
    }
}

/// A wanderer on rough ground, one foot up on a rock ahead of him (the
/// stance of *Wanderer above the Sea of Fog*): long coat, stick in the right
/// hand, bareheaded, hair blown. `step` lifts the left foot (figure heights).
#[allow(clippy::too_many_arguments)]
pub fn wanderer(c: &mut Canvas, at: (f32, f32), size: f32, coat: Paint, dark: Paint, hair: Paint, step: f32, rim: Option<Paint>, seed: u64) {
    let mut h = Hand::new(at, size, seed);
    h.tremor = 0.002;
    // legs: the right straight down, the left bent up onto the rock
    let tr = body(mix(dark, coat, 0.25));
    let mut lg = h.take(Tool::round_sable, 0.045, tr, 1.0);
    h.mark(c, &mut lg, Mark { pts: &[(0.03, 0.45), (0.035, 0.2), (0.038, 0.03)], pressure: (0.95, 0.75), ramps: (0.0, 0.0) }, None);
    lg.reload(tr, 1.0);
    h.mark(c, &mut lg, Mark { pts: &[(-0.03, 0.45), (-0.06, 0.3 + step * 0.6), (-0.055, 0.03 + step)], pressure: (0.95, 0.75), ramps: (0.0, 0.0) }, None);
    let mut bt = h.take(Tool::round_sable, 0.036, thin(dark), 0.6);
    h.mark(c, &mut bt, Mark { pts: &[(0.038, 0.085), (0.038, 0.03), (0.042, 0.012), (0.066, 0.008)], pressure: (0.8, 0.7), ramps: (0.0, 0.25) }, None);
    h.mark(c, &mut bt, Mark { pts: &[(-0.055, 0.085 + step), (-0.055, 0.03 + step), (-0.06, 0.012 + step), (-0.085, 0.008 + step)], pressure: (0.8, 0.7), ramps: (0.0, 0.25) }, None);
    // the stick, planted beside the right foot
    let mut st = h.take(Tool::rigger, 0.012, body(dark), 1.0);
    h.mark(c, &mut st, Mark { pts: &[(0.12, 0.5), (0.135, 0.25), (0.15, 0.0)], pressure: (0.9, 0.8), ramps: (0.0, 0.05) }, None);
    c.dry();
    // the coat: to the knee, open at the back so the stepping leg shows
    let cp = body(coat);
    let mut cb = h.take(Tool::round_sable, 0.065, cp, 1.0);
    for (i, (u0, u1)) in [(-0.05f32, -0.085f32), (0.05, 0.085), (-0.018, -0.03), (0.02, 0.035)].iter().enumerate() {
        if i % 2 == 0 {
            cb.reload(cp, 1.0);
        }
        h.mark(c, &mut cb, Mark { pts: &[(*u0, 0.82), ((u0 + u1) * 0.5, 0.6), (*u1, 0.38)], pressure: (0.7, 0.95), ramps: (0.05, 0.08) }, None);
    }
    cb.reload(cp, 1.0);
    h.line(c, &mut cb, &[(-0.09, 0.765), (-0.045, 0.825), (0.045, 0.825), (0.09, 0.765)], 0.7, 0.7);
    // arms: the left hangs, the right down to the stick
    h.line(c, &mut cb, &[(-0.075, 0.78), (-0.095, 0.64), (-0.09, 0.5)], 0.6, 0.5);
    h.line(c, &mut cb, &[(0.075, 0.78), (0.105, 0.64), (0.12, 0.52)], 0.6, 0.5);
    c.dry();
    // bare head, the hair blown to one side
    let mut hd = h.take(Tool::round_sable, 0.06, thin(hair), 0.45);
    h.dab(c, &mut hd, 0.0, 0.88, 0.05, FRAC_PI_2, 1.0);
    let mut hb = h.take(Tool::round_sable, 0.02, thin(hair), 0.4);
    for k in 0..4 {
        let v = 0.9 + k as f32 * 0.012;
        h.mark(c, &mut hb, Mark { pts: &[(-0.01, v), (0.03, v + 0.005), (0.06, v - 0.004)], pressure: (0.7, 0.1), ramps: (0.0, 0.6) }, None);
    }
    if let Some(l) = rim {
        c.dry();
        let mut lb = h.take(Tool::round_sable, 0.013, lean(mix(coat, l, 0.5)), 0.35 * 0.4);
        h.mark(c, &mut lb, Mark { pts: &[(0.09, 0.77), (0.05, 0.83), (-0.02, 0.84)], pressure: (0.4, 0.2), ramps: (0.2, 0.5) }, None);
    }
}
