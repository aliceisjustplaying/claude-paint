//! Vector shapes in canvas units (rasterized by tiny-skia into masks).

use tiny_skia::{Path, PathBuilder};

#[derive(Default)]
pub struct Shape {
    pb: PathBuilder,
}

impl Shape {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.pb.move_to(x, y);
        self
    }
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.pb.line_to(x, y);
        self
    }
    pub fn quad_to(mut self, cx: f32, cy: f32, x: f32, y: f32) -> Self {
        self.pb.quad_to(cx, cy, x, y);
        self
    }
    pub fn cubic_to(mut self, c1: (f32, f32), c2: (f32, f32), p: (f32, f32)) -> Self {
        self.pb.cubic_to(c1.0, c1.1, c2.0, c2.1, p.0, p.1);
        self
    }
    pub fn close(mut self) -> Self {
        self.pb.close();
        self
    }

    pub fn rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        if let Some(r) = tiny_skia::Rect::from_xywh(x, y, w, h) {
            self.pb.push_rect(r);
        }
        self
    }

    pub fn ellipse(mut self, cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        if let Some(r) = tiny_skia::Rect::from_xywh(cx - rx, cy - ry, 2.0 * rx, 2.0 * ry) {
            self.pb.push_oval(r);
        }
        self
    }

    pub fn circle(self, cx: f32, cy: f32, r: f32) -> Self {
        self.ellipse(cx, cy, r, r)
    }

    /// Straight-edged closed polygon.
    pub fn poly(mut self, pts: &[(f32, f32)]) -> Self {
        if let Some((&(x, y), rest)) = pts.split_first() {
            self.pb.move_to(x, y);
            for &(x, y) in rest {
                self.pb.line_to(x, y);
            }
            self.pb.close();
        }
        self
    }

    /// Closed smooth curve through the points (Catmull-Rom → cubic Béziers).
    pub fn smooth_poly(mut self, pts: &[(f32, f32)]) -> Self {
        let n = pts.len();
        if n < 3 {
            return self.poly(pts);
        }
        self.pb.move_to(pts[0].0, pts[0].1);
        for i in 0..n {
            let p0 = pts[(i + n - 1) % n];
            let p1 = pts[i];
            let p2 = pts[(i + 1) % n];
            let p3 = pts[(i + 2) % n];
            let c1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);
            let c2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);
            self.pb.cubic_to(c1.0, c1.1, c2.0, c2.1, p2.0, p2.1);
        }
        self.pb.close();
        self
    }

    /// Region below an open curve (e.g. a hill line or horizon) down to `bottom`.
    pub fn below(mut self, curve: &[(f32, f32)], bottom: f32) -> Self {
        if curve.len() < 2 {
            return self;
        }
        self.pb.move_to(curve[0].0, bottom);
        for &(x, y) in curve {
            self.pb.line_to(x, y);
        }
        self.pb.line_to(curve[curve.len() - 1].0, bottom);
        self.pb.close();
        self
    }

    pub fn path(self) -> Option<Path> {
        self.pb.finish()
    }
}
