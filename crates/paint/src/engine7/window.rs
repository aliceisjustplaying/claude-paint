use crate::bristle::Rect;

pub(crate) const MAX_CONTACTS: usize = 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct WindowDescriptor {
    pub interior: Rect,
    pub read_halo: Rect,
    pub generation: u64,
}

pub(crate) struct Window {
    pub descriptor: WindowDescriptor,
    pub len: usize,
    pub indices: [usize; MAX_CONTACTS],
    pub xs: [usize; MAX_CONTACTS],
    pub ys: [usize; MAX_CONTACTS],
    pub weights: [f32; MAX_CONTACTS],
}

impl Window {
    pub const fn new() -> Self {
        Self {
            descriptor: WindowDescriptor {
                interior: (0, 0, 0, 0),
                read_halo: (0, 0, 0, 0),
                generation: 0,
            },
            len: 0,
            indices: [0; MAX_CONTACTS],
            xs: [0; MAX_CONTACTS],
            ys: [0; MAX_CONTACTS],
            weights: [0.0; MAX_CONTACTS],
        }
    }

    pub fn reset(&mut self, interior: Rect, read_halo: Rect) {
        self.len = 0;
        self.descriptor = WindowDescriptor {
            interior,
            read_halo,
            generation: self.descriptor.generation.wrapping_add(1),
        };
    }

    #[inline]
    pub fn push(&mut self, i: usize, x: usize, y: usize, weight: f32) -> bool {
        if self.len == MAX_CONTACTS {
            return false;
        }
        self.indices[self.len] = i;
        self.xs[self.len] = x;
        self.ys[self.len] = y;
        self.weights[self.len] = weight;
        self.len += 1;
        true
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SmoothRange {
    lo: f32,
    inverse: f32,
}

impl SmoothRange {
    pub fn new(lo: f32, hi: f32) -> Self {
        Self {
            lo,
            inverse: 1.0 / (hi - lo),
        }
    }

    #[inline]
    pub fn at(self, value: f32) -> f32 {
        let t = ((value - self.lo) * self.inverse).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

pub(crate) struct Capsule {
    a: (f32, f32),
    dx: f32,
    dy: f32,
    inverse_seg2: f32,
    rb: f32,
    radial: SmoothRange,
    fine: bool,
    ux: f32,
    uy: f32,
    lo: f32,
    hi: f32,
}

impl Capsule {
    pub fn new(a: (f32, f32), b: (f32, f32), rb: f32, fine: bool, touch: bool) -> Self {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        let (ux, uy, lo, hi) = if seg > 1e-4 {
            (dx / seg, dy / seg, 0.0, seg)
        } else {
            (1.0, 0.0, -rb, rb)
        };
        Self {
            a,
            dx,
            dy,
            inverse_seg2: if seg2 > 1e-8 { 1.0 / seg2 } else { 0.0 },
            rb,
            radial: if touch {
                SmoothRange::new(0.0, rb)
            } else {
                SmoothRange::new(rb * 0.5, rb + 0.5)
            },
            fine,
            ux,
            uy,
            lo,
            hi,
        }
    }

    #[inline]
    pub fn coverage(&self, px: f32, py: f32) -> f32 {
        if self.fine {
            return self.fine_coverage(px, py);
        }
        let t = (((px - self.a.0) * self.dx + (py - self.a.1) * self.dy) * self.inverse_seg2)
            .clamp(0.0, 1.0);
        let (qx, qy) = (self.a.0 + self.dx * t - px, self.a.1 + self.dy * t - py);
        let dist = (qx * qx + qy * qy).sqrt();
        if dist > self.rb + 0.5 {
            0.0
        } else {
            1.0 - self.radial.at(dist)
        }
    }

    fn fine_coverage(&self, px: f32, py: f32) -> f32 {
        let (rx, ry) = (px - self.a.0, py - self.a.1);
        let t = rx * self.ux + ry * self.uy;
        let q = ry * self.ux - rx * self.uy;
        const R: f32 = std::f32::consts::FRAC_1_SQRT_2;
        if q.abs() > self.rb + R || t < self.lo - R || t > self.hi + R {
            return 0.0;
        }
        if self.ux.abs() < 1e-6 || self.uy.abs() < 1e-6 {
            let across =
                ((q.abs() + 0.5).min(self.rb) - (q.abs() - 0.5).max(-self.rb)).clamp(0.0, 1.0);
            return across * ((t + 0.5).min(self.hi) - (t - 0.5).max(self.lo)).clamp(0.0, 1.0);
        }
        let (nx, ny) = (-self.uy, self.ux);
        let corner = |s: f32, r: f32| {
            (
                self.a.0 - px + self.ux * s + nx * r,
                self.a.1 - py + self.uy * s + ny * r,
            )
        };
        let mut poly = [(0.0f32, 0.0f32); 8];
        let mut n = 4;
        poly[..4].copy_from_slice(&[
            corner(self.lo, -self.rb),
            corner(self.hi, -self.rb),
            corner(self.hi, self.rb),
            corner(self.lo, self.rb),
        ]);
        for side in 0..4 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_scratch_reuses_storage_and_renews_generation() {
        let mut window = Window::new();
        let pointer = window.weights.as_ptr();
        window.reset((2, 3, 4, 5), (1, 2, 5, 6));
        for i in 0..MAX_CONTACTS {
            assert!(window.push(i, i, 0, 0.5));
        }
        assert!(!window.push(0, 0, 0, 0.5));
        window.reset((0, 0, 1, 1), (0, 0, 2, 2));
        assert_eq!(window.len, 0);
        assert_eq!(window.descriptor.generation, 2);
        assert_eq!(window.weights.as_ptr(), pointer);
    }

    #[test]
    fn fine_capsule_preserves_legacy_coverage() {
        for (a, b, rb) in [
            ((3.2, 4.5), (7.1, 6.4), 0.04),
            ((3.0, 4.0), (8.0, 4.0), 0.2),
            ((4.1, 4.6), (4.1, 4.6), 0.1),
        ] {
            let capsule = Capsule::new(a, b, rb, true, false);
            for y in 0..12 {
                for x in 0..12 {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    assert_eq!(
                        capsule.coverage(px, py),
                        crate::bristle::fine_cover(a, b, rb, px, py)
                    );
                }
            }
        }
    }
}
