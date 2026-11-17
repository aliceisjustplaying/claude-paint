//! Tools a hand moves over a plate (the easel's `--@ etch` and `--@ press`):
//! where a brush, a roller, a card, a pad, the palm or a muslin touches the
//! plate, from the points the painter moves it along, as a needle's path is
//! given (`Point`). Nothing here takes a region: a tool acts where it goes,
//! over the footprint its own size gives it.
//!
//! **An edge moved along a path** (`Plate::sweep`). A tool that slides or
//! rolls over the plate meets it along an edge across its way, `width` mm
//! long: a roller's line of contact, a card's edge, the front of a pad, of
//! the palm or of a loaded brush. As the hand moves it along the points the
//! edge sweeps the plate, and a cell is touched each time the edge passes
//! over it:
//! - between two points the edge moves straight and passes once over each
//!   cell of its band (the cells whose middles it crosses, the band's ends
//!   covering a share of a cell by how far into it they reach);
//! - where the path bends gently (`SMOOTH_DEG` or less between one piece and
//!   the next: points along a curve) the edge turns with it as the hand
//!   carries it round, square to the bend's middle way at the point, and
//!   turning evenly along each piece between;
//! - where it turns sharply the edge swings about the point it is at, the
//!   short way, and its two halves pass over the cells within half its
//!   length in the angle it swings through: the inside of the turn is rubbed
//!   back, then passed again as the edge goes on.
//!
//! So a straight stroke touches each cell of its band once, a stroke back
//! over itself touches them again, and a pad worked round in small circles
//! touches the middle of each circle every time round.
//!
//! **A pad slid along a path** (`Plate::slide`). A round pad pressed flat
//! and slid by the hand (a bunched tarlatan, the palm, a muslin, a dabber's
//! face rubbed along) doesn't turn with the path: it is carried along it, and
//! a point of the plate is rubbed for as far as the pad slides while the
//! point is under it. A straight pass rubs the point under its middle for the
//! pad's width (one pass, by the rag's own measure: `rag::LIFT` is a pass
//! through the pad's middle), a point nearer its rim for the shorter chord
//! there, and none beyond it. Back over the same way rubs it again; a pad
//! worked round in small circles rubs the middle of each circle all the time.
//! So each cell's share of a pass is the length of the path within the pad's
//! radius of it over the pad's width.
//!
//! **A face dabbed** (`Plate::dabs`): a dabber or a poupée set down at each
//! point touches the disc of its face there.

use super::*;

/// A stretch of a tool's path: a straight piece of it, no longer than the
/// step asked for, with the edge's swing at its start where the path turns.
#[derive(Clone, Debug, PartialEq)]
pub struct Stretch {
    /// The way the tool moves (a unit vector on the plate; (0, 0) for a dab).
    pub v: (f64, f64),
    /// The tool's pose at the stretch's middle (its force or pressure and its
    /// speed, as the painter gave them along the points).
    pub pose: Point,
    /// Its length (mm; 0 for a dab).
    pub len_mm: f64,
    /// The edge's length (mm) over the stretch, or the face's diameter.
    pub width_mm: f64,
    /// The cells it touches and the share of each the tool covers, in the
    /// order touched. A cell swung over at a turn and then passed is in it
    /// twice.
    pub cells: Vec<(usize, usize, f32)>,
}

impl Stretch {
    /// Its touches in layers, a cell at most once in each, in order: the
    /// first time each cell is touched, then the second, and so on.
    pub fn layers(&self) -> Vec<Vec<(usize, usize, f32)>> {
        let mut seen: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
        let mut out: Vec<Vec<(usize, usize, f32)>> = Vec::new();
        for &(x, y, m) in &self.cells {
            let k = seen.entry((x, y)).or_insert(0);
            if out.len() <= *k {
                out.push(Vec::new());
            }
            out[*k].push((x, y, m));
            *k += 1;
        }
        out
    }

    /// The area the tool covered (mm²: each touch's share of a cell of
    /// `pitch_um`).
    pub fn area_mm2(&self, pitch_um: f64) -> f64 {
        self.cells.iter().map(|c| c.2 as f64).sum::<f64>() * (pitch_um * 1e-3).powi(2)
    }
}

/// The largest bend (degrees) between one piece of a path and the next that
/// the edge is carried round smoothly; a sharper one it swings about the
/// point [E: a hand following a curve given by points some degrees apart
/// carries the tool round it; at a corner it pivots].
pub const SMOOTH_DEG: f64 = 30.0;

/// The signed angle (radians) from way `a` to way `b`.
fn turn_of(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 * b.1 - a.1 * b.0).atan2(a.0 * b.0 + a.1 * b.1)
}

/// Way `a` turned by `ang` radians.
fn turned(a: (f64, f64), ang: f64) -> (f64, f64) {
    let (s, c) = ang.sin_cos();
    (a.0 * c - a.1 * s, a.0 * s + a.1 * c)
}

/// The share of a cell `pitch` mm across that lies within `half` mm of a line
/// (or a point) its middle is `d` mm from.
fn cover(half: f64, d: f64, pitch: f64) -> f32 {
    ((half - d) / pitch + 0.5).clamp(0.0, 1.0) as f32
}

/// `p` at `u` of the way from `a` to `b`.
fn pose_at(a: &Point, b: &Point, u: f64) -> Point {
    let l = |p: f32, q: f32| (p as f64 + (q as f64 - p as f64) * u) as f32;
    Point { x: l(a.x, b.x), y: l(a.y, b.y), force: l(a.force, b.force), alt: l(a.alt, b.alt), az: l(a.az, b.az), speed: l(a.speed, b.speed) }
}

impl Plate {
    /// The cells (x0, y0, x1, y1) within `r` mm of the box (mm) `x0..x1` ×
    /// `y0..y1`, on the plate and in its clip.
    fn cells_near(&self, (x0, y0, x1, y1): (f64, f64, f64, f64), r: f64) -> (usize, usize, usize, usize) {
        let p = self.pitch * 1e-3;
        let lo = |v: f64| ((v - r) / p).floor().max(0.0) as usize;
        let hi = |v: f64, n: usize| (((v + r) / p).ceil().max(0.0) as usize).min(n);
        let (mut a, mut b, mut c, mut d) = (lo(x0), lo(y0), hi(x1, self.w), hi(y1, self.h));
        if let Some((cx0, cy0, cx1, cy1)) = self.clip {
            (a, b, c, d) = (a.max(cx0), b.max(cy0), c.min(cx1), d.min(cy1));
        }
        (a, b, c.max(a), d.max(b))
    }

    /// The middle of cell (x, y) (mm).
    fn mid_mm(&self, x: usize, y: usize) -> (f64, f64) {
        let p = self.pitch * 1e-3;
        ((x as f64 + 0.5) * p, (y as f64 + 0.5) * p)
    }

    /// An edge `width(pose)` mm long moved along `pts` (mm on the plate), in
    /// stretches of at most `step_mm`: the cells it touches (see the module's
    /// notes). Points closer than a thousandth of a millimetre are one.
    pub fn sweep(&self, pts: &[Point], width: &dyn Fn(&Point) -> f64, step_mm: f64) -> Vec<Stretch> {
        let p = self.pitch * 1e-3;
        let step = step_mm.max(p);
        // the pieces between distinct points, and their ways
        let mut pcs: Vec<(Point, Point, (f64, f64), f64)> = Vec::new();
        for seg in pts.windows(2) {
            let (a, b) = (seg[0], seg[1]);
            let (dx, dy) = ((b.x - a.x) as f64, (b.y - a.y) as f64);
            let len = dx.hypot(dy);
            if len >= 1e-3 {
                pcs.push((a, b, (dx / len, dy / len), len));
            }
        }
        let smooth = SMOOTH_DEG.to_radians();
        // the turn at the start of each piece (0 for the first)
        let turns: Vec<f64> = (0..pcs.len()).map(|k| if k == 0 { 0.0 } else { turn_of(pcs[k - 1].2, pcs[k].2) }).collect();
        let mut out = Vec::new();
        for (k, &(a, b, t, len)) in pcs.iter().enumerate() {
            // the edge's way at the piece's ends: half a gentle bend's turn either side of the point
            let start = if turns[k].abs() <= smooth { turned(t, -0.5 * turns[k]) } else { t };
            let next = turns.get(k + 1).copied().unwrap_or(0.0);
            let end = if next.abs() <= smooth { turned(t, 0.5 * next) } else { t };
            let span = turn_of(start, end);
            let n = ((len / step).ceil() as usize).max(1);
            let l = len / n as f64;
            for i in 0..n {
                let (u0, u1) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
                let q0 = (a.x as f64 + (b.x - a.x) as f64 * u0, a.y as f64 + (b.y - a.y) as f64 * u0);
                let q1 = (a.x as f64 + (b.x - a.x) as f64 * u1, a.y as f64 + (b.y - a.y) as f64 * u1);
                let pose = pose_at(&a, &b, 0.5 * (u0 + u1));
                let half = 0.5 * width(&pose).max(0.0);
                let mut cells = Vec::new();
                if i == 0 && turns[k].abs() > smooth {
                    self.edge_swing(q0, pcs[k - 1].2, t, half, &mut cells);
                }
                self.edge_move((q0, turned(start, span * u0)), (q1, turned(start, span * u1)), half, &mut cells);
                out.push(Stretch { v: t, pose, len_mm: l, width_mm: 2.0 * half, cells });
            }
        }
        out
    }

    /// The cells an edge `2·half` mm long passes over moving from `(q0, t0)`
    /// to `(q1, t1)` (its middle, and the way it faces: it lies square to
    /// it): those whose middles go from ahead of it to behind it, or back,
    /// within `half` of its middle as it crosses them (by share). Ahead is
    /// strictly ahead: a cell on the edge where it starts was the last
    /// stretch's.
    fn edge_move(&self, (q0, t0): ((f64, f64), (f64, f64)), (q1, t1): ((f64, f64), (f64, f64)), half: f64, out: &mut Vec<(usize, usize, f32)>) {
        if half <= 0.0 {
            return;
        }
        let p = self.pitch * 1e-3;
        let (x0, y0, x1, y1) = self.cells_near((q0.0.min(q1.0), q0.1.min(q1.1), q0.0.max(q1.0), q0.1.max(q1.1)), half + p);
        for y in y0..y1 {
            for x in x0..x1 {
                let (cx, cy) = self.mid_mm(x, y);
                let f = |q: (f64, f64), t: (f64, f64)| ((cx - q.0) * t.0 + (cy - q.1) * t.1, (cy - q.1) * t.0 - (cx - q.0) * t.1);
                let ((f0, u0), (f1, u1)) = (f(q0, t0), f(q1, t1));
                if (f0 > 0.0) == (f1 > 0.0) {
                    continue;
                }
                let at = f0 / (f0 - f1);
                let m = cover(half, (u0 + (u1 - u0) * at).abs(), p);
                if m > 0.0 {
                    out.push((x, y, m));
                }
            }
        }
    }

    /// The cells an edge `2·half` mm long swings over about `q`, turning from
    /// square to `ta` to square to `tb` the short way: those within `half` of
    /// `q` in the angle either half of it sweeps (by share, at its ends).
    fn edge_swing(&self, q: (f64, f64), ta: (f64, f64), tb: (f64, f64), half: f64, out: &mut Vec<(usize, usize, f32)>) {
        let turn = turn_of(ta, tb);
        if half <= 0.0 || turn.abs() < 1e-9 {
            return;
        }
        let p = self.pitch * 1e-3;
        // the edge lies along the normal, which turns with the way
        let na = (-ta.1, ta.0);
        let (x0, y0, x1, y1) = self.cells_near((q.0, q.1, q.0, q.1), half + p);
        let pi = std::f64::consts::PI;
        for y in y0..y1 {
            for x in x0..x1 {
                let (cx, cy) = self.mid_mm(x, y);
                let (dx, dy) = (cx - q.0, cy - q.1);
                let r = dx.hypot(dy);
                let m = cover(half, r, p);
                if m <= 0.0 || r < 1e-12 {
                    continue;
                }
                // its angle from the edge as it was, in the way the edge turns, either half (a
                // cell on the edge as it was was the last stretch's; on it as it ends, this one's)
                let b = ((na.0 * dy - na.1 * dx).atan2(na.0 * dx + na.1 * dy) * turn.signum()).rem_euclid(pi);
                if b > 0.0 && b <= turn.abs() {
                    out.push((x, y, m));
                }
            }
        }
    }

    /// A round pad `width(pose)` mm across slid along `pts` (mm), in
    /// stretches of at most `step_mm`: each cell's share of a pass over each
    /// stretch, the length of the stretch within the pad's radius of the
    /// cell's middle over the pad's width (see the module's notes).
    pub fn slide(&self, pts: &[Point], width: &dyn Fn(&Point) -> f64, step_mm: f64) -> Vec<Stretch> {
        let p = self.pitch * 1e-3;
        let step = step_mm.max(p);
        let mut out = Vec::new();
        for seg in pts.windows(2) {
            let (a, b) = (seg[0], seg[1]);
            let (dx, dy) = ((b.x - a.x) as f64, (b.y - a.y) as f64);
            let len = dx.hypot(dy);
            if len < 1e-3 {
                continue;
            }
            let t = (dx / len, dy / len);
            let n = ((len / step).ceil() as usize).max(1);
            let l = len / n as f64;
            for i in 0..n {
                let u0 = i as f64 / n as f64;
                let q = (a.x as f64 + dx * u0, a.y as f64 + dy * u0);
                let e = (q.0 + t.0 * l, q.1 + t.1 * l);
                let pose = pose_at(&a, &b, (i as f64 + 0.5) / n as f64);
                let w = width(&pose).max(0.0);
                let r = 0.5 * w;
                let mut cells = Vec::new();
                if r > 0.0 {
                    let (x0, y0, x1, y1) = self.cells_near((q.0.min(e.0), q.1.min(e.1), q.0.max(e.0), q.1.max(e.1)), r + p);
                    for y in y0..y1 {
                        for x in x0..x1 {
                            let (cx, cy) = self.mid_mm(x, y);
                            let (ox, oy) = (cx - q.0, cy - q.1);
                            let (s, u) = (ox * t.0 + oy * t.1, oy * t.0 - ox * t.1);
                            if u.abs() >= r {
                                continue;
                            }
                            let h = (r * r - u * u).sqrt();
                            let over = (s + h).min(l) - (s - h).max(0.0);
                            if over > 0.0 {
                                cells.push((x, y, (over / w) as f32));
                            }
                        }
                    }
                }
                out.push(Stretch { v: t, pose, len_mm: l, width_mm: w, cells });
            }
        }
        out
    }

    /// A face `width(pose)` mm across set down at each of `pts`: the disc of
    /// cells it touches there (by share at its rim), a stretch each.
    pub fn dabs(&self, pts: &[Point], width: &dyn Fn(&Point) -> f64) -> Vec<Stretch> {
        let p = self.pitch * 1e-3;
        pts.iter().map(|pt| {
            let half = 0.5 * width(pt).max(0.0);
            let (qx, qy) = (pt.x as f64, pt.y as f64);
            let (x0, y0, x1, y1) = self.cells_near((qx, qy, qx, qy), half + p);
            let mut cells = Vec::new();
            for y in y0..y1 {
                for x in x0..x1 {
                    let (cx, cy) = self.mid_mm(x, y);
                    let m = cover(half, (cx - qx).hypot(cy - qy), p);
                    if m > 0.0 {
                        cells.push((x, y, m));
                    }
                }
            }
            Stretch { v: (0.0, 0.0), pose: *pt, len_mm: 0.0, width_mm: 2.0 * half, cells }
        }).collect()
    }

    /// Each cell's touches over `stretches`, summed by the share covered, as
    /// a raster over the plate (row by row).
    pub fn touches(&self, stretches: &[Stretch]) -> Vec<f32> {
        let mut out = vec![0.0f32; self.w * self.h];
        for s in stretches {
            for &(x, y, m) in &s.cells {
                out[y * self.w + x] += m;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests;
