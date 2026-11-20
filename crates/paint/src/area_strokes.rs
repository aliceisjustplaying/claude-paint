//! Strokes over an area: hatching bowed by the hand's swing, zigzags,
//! jitter and open gaps to the paper, for a pastel stick's `p:hatch` with
//! the easel's `--@ pastel-area-strokes` switch.
//!
//! This only places marks; each mark is then drawn as one stroke of the
//! stick, as `graphite::hatch_marks_graded`'s marks are, so the stroke model
//! (contact, wear, crumbs) is the stick's own.
//!
//! - **A fixed angle** places marks exactly as `hatch_marks_graded` does.
//! - **A pivot** bows each mark as the hand swings it, a fixed arc about
//!   the wrist (or the elbow, for a sweep longer than the wrist turns)
//!   instead of the pencil's random bow (`graphite::Pivot`).
//! - **Open**, **jitter** and **zigzag** then act on each mark in turn,
//!   each with its own seeded draws per mark, so turning one on doesn't
//!   move what the others do.
//!
//! The marks run at one angle, and the share left open is one number: the
//! painting program gives no function, field or mask of where or how much.

pub use crate::graphite::Pivot;
use crate::graphite::{self, Mark};
use crate::mask::Mask;
use crate::path::arclen;
use crate::rng::Rng;

/// Each mark turned into one back-and-forth stroke along it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zigzag {
    /// How far the stroke swings to either side of the mark's line (units).
    pub amplitude: f32,
    /// The length along the line of one swing there and back (units).
    pub period: f32,
    /// 0: sharp corners (a triangle wave); 1: rounded right through (a sine).
    pub round: f32,
}

/// Each mark's length and angle varied.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Jitter {
    /// The length scaled by up to 1 ± this (a share, 0..1).
    pub length: f32,
    /// The mark turned by up to ± this about its middle (radians).
    pub angle: f32,
}

/// Hatching over an area.
pub struct AreaHatch {
    /// The marks' angle, radians.
    pub angle: f32,
    pub spacing: f32,
    pub length: f32,
    pub pressure: f32,
    pub graded: bool,
    pub zigzag: Option<Zigzag>,
    pub jitter: Option<Jitter>,
    /// The share of the marks left out, open to the paper (0..1).
    pub open: Option<f32>,
    /// The hand's pivot, bowing each mark; without it, a random bow as the
    /// pencil's hatching has.
    pub pivot: Option<Pivot>,
}

/// Per-mark draws: one generator per (option, mark), so options draw
/// independently of each other and of the placement.
fn mark_rng(seed: u64, salt: u64, k: usize) -> Rng {
    Rng::new(seed ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (k as u64).wrapping_mul(0xD1B5_4A32_D192_ED03))
}
const OPEN_SALT: u64 = 0x0FE7;
const JITTER_SALT: u64 = 0x1177;
const ZIGZAG_SALT: u64 = 0x2162;

/// The marks for hatching `m` as `h` says.
pub fn area_marks(m: &Mask, h: &AreaHatch, seed: u64) -> Vec<Mark> {
    let level = if h.graded { 0.04 } else { 0.5 };
    let base = graphite::hatch_marks_swung(m, h.angle, h.spacing, h.length, h.pressure, h.graded, seed, h.pivot.as_ref());
    let mut out = Vec::with_capacity(base.len());
    for (k, mark) in base.into_iter().enumerate() {
        if mark.pts.len() < 2 {
            continue;
        }
        if let Some(share) = h.open {
            if mark_rng(seed, OPEN_SALT, k).f() < share.clamp(0.0, 1.0) {
                continue;
            }
        }
        let mark = match h.jitter {
            Some(j) => jittered(m, level, mark, j, &mut mark_rng(seed, JITTER_SALT, k)),
            None => mark,
        };
        let mark = match h.zigzag {
            Some(z) => zigzagged(m, level, &mark, z, &mut mark_rng(seed, ZIGZAG_SALT, k)),
            None => mark,
        };
        out.push(mark);
    }
    out
}

fn inside(m: &Mask, level: f32, x: f32, y: f32) -> bool {
    let f = m.f;
    x >= 0.0 && y >= 0.0 && x < f.width() && y < f.height() && m.sample(x, y) > level
}

/// A mark turned about its middle and scaled in length (growing only as far
/// as the area: a mark that would leave it keeps its length).
fn jittered(m: &Mask, level: f32, mark: Mark, j: Jitter, rng: &mut Rng) -> Mark {
    let turn = j.angle * rng.range(-1.0, 1.0);
    let mut k = 1.0 + j.length.clamp(0.0, 1.0) * rng.range(-1.0, 1.0);
    let c = mark.pts[mark.pts.len() / 2];
    let (ct, st) = (turn.cos(), turn.sin());
    let moved = |k: f32, p: (f32, f32)| {
        let (dx, dy) = ((p.0 - c.0) * k, (p.1 - c.1) * k);
        (c.0 + dx * ct - dy * st, c.1 + dx * st + dy * ct)
    };
    if k > 1.0 {
        let (a, b) = (moved(k, mark.pts[0]), moved(k, *mark.pts.last().unwrap()));
        if !inside(m, level, a.0, a.1) || !inside(m, level, b.0, b.1) {
            k = 1.0;
        }
    }
    Mark { pts: mark.pts.iter().map(|&p| moved(k, p)).collect(), pressure: mark.pressure }
}

/// The wave a zigzag swings by, -1..1 at `t` swings along (1 at t = 0).
fn wave(t: f32, round: f32) -> f32 {
    let u = t - t.floor();
    let tri = 4.0 * (u - 0.5).abs() - 1.0;
    let sine = (std::f32::consts::TAU * u).cos();
    tri + (sine - tri) * round.clamp(0.0, 1.0)
}

/// A mark made one continuous stroke swinging across its line: `amplitude`
/// to either side, one swing there and back every `period` along it, from
/// a seeded phase. Where a swing would leave the area it is cut short at
/// its edge, so the band stays inside the mask.
fn zigzagged(m: &Mask, level: f32, mark: &Mark, z: Zigzag, rng: &mut Rng) -> Mark {
    let (amp, period) = (z.amplitude.max(0.0), z.period.max(0.5));
    let s0 = arclen(&mark.pts);
    let total = s0.last().copied().unwrap_or(0.0);
    if total <= 0.0 || amp <= 0.0 {
        return mark.clone();
    }
    // the line resampled finely enough that the swung path's points stay
    // about 0.25 units apart
    let slope = 4.0 * amp / period;
    let step = 0.25 / (1.0 + slope);
    let line = graphite::resample(&mark.pts, false, step);
    let s = arclen(&line);
    let phase = rng.f();
    let n = line.len();
    let mut pts = Vec::with_capacity(n);
    let mut pressure = Vec::with_capacity(n);
    let mut j = 0;
    for i in 0..n {
        let (a, b) = (line[i.saturating_sub(1)], line[(i + 1).min(n - 1)]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l = (dx * dx + dy * dy).sqrt().max(1e-6);
        let nm = (-dy / l, dx / l);
        let d = amp * wave(s[i] / period + phase, z.round);
        let mut q = (line[i].0 + nm.0 * d, line[i].1 + nm.1 * d);
        if !inside(m, level, q.0, q.1) {
            // back toward the line until inside (or on it)
            let mut f = 1.0;
            for _ in 0..6 {
                f *= 0.5;
                q = (line[i].0 + nm.0 * d * f, line[i].1 + nm.1 * d * f);
                if inside(m, level, q.0, q.1) {
                    break;
                }
            }
            if !inside(m, level, q.0, q.1) {
                q = line[i];
            }
        }
        pts.push(q);
        // the mark's pressure at this far along it
        while j + 1 < s0.len() && s0[j + 1] < s[i] {
            j += 1;
        }
        let p = if j + 1 < s0.len() {
            let t = ((s[i] - s0[j]) / (s0[j + 1] - s0[j]).max(1e-6)).clamp(0.0, 1.0);
            mark.pressure[j] + (mark.pressure[j + 1] - mark.pressure[j]) * t
        } else {
            *mark.pressure.last().unwrap()
        };
        pressure.push(p);
    }
    Mark { pts, pressure }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Frame;
    use crate::shape::Shape;

    fn square() -> Mask {
        let f = Frame::new(200, 200, 0.2);
        Mask::from_shape(f, Shape::new().rect(200.0, 200.0, 600.0, 600.0))
    }

    fn direction(m: &Mark) -> f32 {
        let (a, b) = (m.pts[0], *m.pts.last().unwrap());
        let d = (b.1 - a.1).atan2(b.0 - a.0);
        // as a direction, in (-π/2, π/2]
        if d > std::f32::consts::FRAC_PI_2 { d - std::f32::consts::PI } else if d <= -std::f32::consts::FRAC_PI_2 { d + std::f32::consts::PI } else { d }
    }

    fn plain(angle: f32) -> AreaHatch {
        AreaHatch { angle, spacing: 10.0, length: 60.0, pressure: 0.5, graded: false, zigzag: None, jitter: None, open: None, pivot: None }
    }

    #[test]
    fn a_fixed_angle_places_the_graded_hatchings_marks() {
        let m = square();
        let a = area_marks(&m, &plain(-0.9), 7);
        let b = graphite::hatch_marks_graded(&m, -0.9, 10.0, 60.0, 0.5, false, 7);
        assert_eq!(a.len(), b.len());
        assert!(a.iter().zip(&b).all(|(a, b)| a.pts == b.pts && a.pressure == b.pressure));
    }

    #[test]
    fn a_zigzag_mark_is_one_path_over_a_band() {
        let m = square();
        let mut h = plain(0.0);
        h.spacing = 50.0;
        h.zigzag = Some(Zigzag { amplitude: 20.0, period: 15.0, round: 0.3 });
        let plainm = area_marks(&m, &plain(0.0), 5);
        let zig = area_marks(&m, &h, 5);
        let mk = &zig[zig.len() / 2];
        let ys: Vec<f32> = mk.pts.iter().map(|p| p.1).collect();
        let band = ys.iter().cloned().fold(f32::MIN, f32::max) - ys.iter().cloned().fold(f32::MAX, f32::min);
        assert!(band > 35.0, "band {band}");
        // back and forth: the direction across the line changes sign many times
        let turns = ys.windows(3).filter(|w| (w[1] - w[0]) * (w[2] - w[1]) < 0.0).count();
        assert!(turns >= 4, "{turns}");
        // consecutive points stay close: one continuous stroke
        assert!(mk.pts.windows(2).all(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt() < 1.0));
        // the swings stay inside the area across the marks
        assert!(zig.iter().all(|mk| mk.pts.iter().all(|p| p.1 > 199.0 && p.1 < 801.0)));
        assert!(plainm.len() > zig.len());
    }

    /// A right hand's pivot in mm (units here): ANSUR II's hand 189.3 mm,
    /// 40° of wrist turn, forearm and hand 467 mm, the hand to the lower right.
    fn hand() -> Pivot {
        let r = std::f32::consts::FRAC_1_SQRT_2;
        Pivot { wrist: 189.3, reach: 189.3 * 40f32.to_radians(), elbow: 467.0, toward: (r, r) }
    }

    /// A mark's chord length and its middle's offset from the chord along
    /// the normal (-dy, dx) / L (the farthest point from the chord, signed).
    fn sag(mk: &Mark) -> (f32, f32, (f32, f32)) {
        let (a, b) = (mk.pts[0], *mk.pts.last().unwrap());
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l = (dx * dx + dy * dy).sqrt();
        let n = (-dy / l, dx / l);
        let off = mk.pts.iter().map(|p| (p.0 - a.0) * n.0 + (p.1 - a.1) * n.1).fold(0.0f32, |m, d| if d.abs() > m.abs() { d } else { m });
        (l, off, n)
    }

    /// An arc about the wrist sags L²/(8R) along the radius, its cosine c
    /// to the chord's normal of it across the chord (the swept share L·|c|),
    /// away from the hand: -c³ L²/(8R), R the elbow's past the wrist's reach.
    fn expected_sag(l: f32, n: (f32, f32), pv: &Pivot) -> f32 {
        let c = n.0 * pv.toward.0 + n.1 * pv.toward.1;
        let r = if l * c.abs() <= 189.3 * 40f32.to_radians() { 189.3 } else { 467.0 };
        -c.powi(3) * l * l / (8.0 * r)
    }

    #[test]
    fn a_pivot_bows_each_mark_away_from_the_hand() {
        let m = square();
        let pv = hand();
        let mut h = plain(-0.9);
        h.pivot = Some(pv);
        let swung = area_marks(&m, &h, 7);
        let drawn = area_marks(&m, &plain(-0.9), 7);
        // the same marks and ends: only the bow differs
        assert_eq!(swung.len(), drawn.len());
        for (s, d) in swung.iter().zip(&drawn) {
            assert!(s.pts[0] == d.pts[0] && s.pts.last() == d.pts.last() && s.pts.len() == d.pts.len());
            let (l, off, n) = sag(s);
            let want = expected_sag(l, n, &pv);
            assert!((off - want).abs() < 0.02 * want.abs() + 0.01, "{l}: {off} against {want}");
            // away from the hand, to the upper left
            let mid = s.pts[s.pts.len() / 2];
            let (a, b) = (s.pts[0], *s.pts.last().unwrap());
            assert!((mid.0 - (a.0 + b.0) / 2.0) * pv.toward.0 + (mid.1 - (a.1 + b.1) / 2.0) * pv.toward.1 < 0.0 || l < 5.0);
        }
        // a 60 mm mark at -0.9: 60²/(8·189.3) = 2.38 mm, nearly all of it across
        let long = swung.iter().map(sag).filter(|(l, _, _)| (*l - 60.0).abs() < 3.0).count();
        assert!(long > 10, "{long}");
        // along the radius (toward the hand) a mark is straight
        let mut h = plain(std::f32::consts::FRAC_PI_4);
        h.pivot = Some(pv);
        assert!(area_marks(&m, &h, 7).iter().all(|mk| sag(mk).1.abs() < 1e-3));
        // past the wrist's reach (132 mm) the elbow swings it
        let mut h = plain(-std::f32::consts::FRAC_PI_4);
        h.length = 300.0;
        h.pivot = Some(pv);
        let marks = area_marks(&m, &h, 7);
        let elbow = marks.iter().map(sag).filter(|(l, _, _)| *l > 140.0).collect::<Vec<_>>();
        assert!(!elbow.is_empty());
        for (l, off, n) in elbow {
            assert!((off - expected_sag(l, n, &pv)).abs() < 0.02 * off.abs() + 0.01, "{l}: {off}");
        }
    }

    #[test]
    fn round_one_is_a_sine() {
        for i in 0..40 {
            let t = i as f32 / 17.0;
            assert!((wave(t, 1.0) - (std::f32::consts::TAU * t).cos()).abs() < 1e-5);
        }
    }

    #[test]
    fn jitter_and_open_are_seeded_and_independent() {
        let m = square();
        let mut h = plain(0.3);
        h.jitter = Some(Jitter { length: 0.3, angle: 0.3 });
        let a = area_marks(&m, &h, 11);
        let b = area_marks(&m, &h, 11);
        let c = area_marks(&m, &h, 12);
        assert!(a.iter().zip(&b).all(|(a, b)| a.pts == b.pts));
        assert!(a.len() != c.len() || a.iter().zip(&c).any(|(a, c)| a.pts != c.pts));
        let base = area_marks(&m, &plain(0.3), 11);
        let turned = a.iter().zip(&base).filter(|(a, b)| (direction(a) - direction(b)).abs() > 0.02).count();
        assert!(turned > base.len() / 2, "{turned} of {}", base.len());
        // open drops about its share of the marks, and the same ones whatever the jitter
        h.open = Some(0.5);
        let opened = area_marks(&m, &h, 11);
        let share = opened.len() as f32 / a.len() as f32;
        assert!((0.35..0.65).contains(&share), "{share}");
        h.jitter = None;
        assert_eq!(area_marks(&m, &h, 11).len(), opened.len());
    }
}
