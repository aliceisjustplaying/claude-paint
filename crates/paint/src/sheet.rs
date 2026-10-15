//! A paper mask: a sheet of paper laid over part of the picture to keep it
//! clean, as pastellists do. It is a surface of its own lying `caliper` µm
//! above the picture, with a paper's pores: a pastel stick rests on it as on
//! any surface (the contact solver sees it as raised ground), so near its
//! edge the stick bridges from sheet to picture and the edge it leaves is as
//! sharp as the stick's angle and force make it. What lands on the sheet
//! (strokes, crumbs) stays on the sheet and leaves with it; the finger, the
//! eraser and fixative don't reach under it. Wet-paint tools are not
//! modeled under a sheet (the easel refuses them while one is down).
//!
//! Also what a stick held up to the picture would touch (`stick_contact`),
//! for the easel's hold look.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::graphite::cover;
use crate::mask::Mask;
use crate::pastel::PARTICLE_UM;

/// A sheet of paper laid on the picture.
#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    /// Where it lies, per pixel of the window.
    pub(crate) over: Vec<bool>,
    /// Its thickness, µm.
    pub(crate) caliper_um: f32,
    /// Its pores' mean depth, µm.
    pub(crate) micro_um: f32,
    /// Its color (linear RGB).
    pub(crate) tone: Rgb,
    /// The pastel caught on it, for the look: coverage and color.
    pub(crate) a: Vec<f32>,
    pub(crate) r: Vec<Rgb>,
}

impl Canvas {
    /// Lay a sheet over `m` (where it is above 0.5): `caliper_um` thick, its
    /// pores `micro_um` deep, of color `tone`. One sheet at a time.
    pub fn lay_sheet(&mut self, m: &Mask, caliper_um: f32, micro_um: f32, tone: Rgb) -> Result<(), String> {
        if self.sheet.is_some() {
            return Err("a sheet already lies on the picture: lift it first (lift_sheet())".into());
        }
        self.check_mask(m);
        let f = self.f;
        let n = self.px.len();
        let over: Vec<bool> = (0..n).map(|i| m.data[f.whole_index(i)] > 0.5).collect();
        self.sheet = Some(Sheet { over, caliper_um: caliper_um.max(1.0), micro_um: micro_um.max(0.1), tone, a: vec![0.0; n], r: vec![[0.0; 3]; n] });
        Ok(())
    }

    /// Lift the sheet, with whatever it caught. False if none was laid.
    pub fn lift_sheet(&mut self) -> bool {
        self.sheet.take().is_some()
    }

    /// The mean pore depth of the support, µm (a paper's), if it has one.
    pub fn mean_micro_um(&self) -> Option<f32> {
        (!self.micro.is_empty()).then(|| self.micro.iter().sum::<f32>() / self.micro.len() as f32)
    }

    pub fn has_sheet(&self) -> bool {
        self.sheet.is_some()
    }

    /// Whether the sheet lies over window pixel `i`.
    #[inline]
    pub(crate) fn sheet_over(&self, i: usize) -> bool {
        self.sheet.as_ref().is_some_and(|s| s.over[i])
    }

    /// The surface a stick comes down on at window pixel `i`: its height (µm),
    /// its pores' mean depth (µm) and how stiffly it gives (MPa/µm). The
    /// sheet, where it lies, is that much higher, with its own pores.
    #[inline]
    pub(crate) fn surface_point(&self, i: usize) -> (f32, f32, f32) {
        match &self.sheet {
            Some(s) if s.over[i] => (self.height[i] + s.caliper_um, s.micro_um, self.give_mpa_per_um(i)),
            _ => (self.height[i], self.micro_um(i), self.give_mpa_per_um(i)),
        }
    }

    /// Pastel landed on the sheet: `(pixel, µm)` of `color`, kept there
    /// (coverage by volume, as on paper) until the sheet is lifted.
    pub(crate) fn sheet_catch(&mut self, caught: &[(usize, f32)], color: Rgb) {
        let Some(s) = self.sheet.as_mut() else { return };
        for &(i, dv) in caught {
            let q = 1.0 - (-dv / PARTICLE_UM).exp();
            let old = s.a[i] * (1.0 - q);
            let w = old + q;
            s.r[i] = [0, 1, 2].map(|k| ((old * s.r[i][k] + q * color[k]) / w.max(1e-6)).clamp(0.0, 1.0));
            s.a[i] = (s.a[i] + (1.0 - s.a[i]) * q).min(0.995);
        }
    }

    /// The picture as seen with the sheet on it (`seen`'s pixel `i`).
    #[inline]
    pub(crate) fn sheet_seen(&self, i: usize) -> Option<Rgb> {
        let s = self.sheet.as_ref()?;
        if !s.over[i] {
            return None;
        }
        // (paper is matte: its first-surface veil, as a matte surface's)
        Some(crate::canvas::haze(cover(s.tone, s.a[i], s.r[i]), 0.0))
    }

    /// A stick held at `pose` with its tip at (x, y) (units), over the window's
    /// pixels: 2 where it touches, 1 where its face lies within a crumb's size
    /// of the surface (where its crumbs settle), 0 elsewhere under its low
    /// part. Reads only (the hold look).
    pub fn stick_contact(&self, stick: &crate::pastel::Stick, x: f32, y: f32, pose: crate::pastel::Pose) -> Vec<(usize, u8)> {
        let sp = crate::pastel::StrokePoint { x, y, pose, speed: 50.0 };
        let Some(st) = self.seat_stick(stick, &sp) else { return Vec::new() };
        let mut out = Vec::new();
        for (j, &(i, b)) in st.under.iter().enumerate() {
            let (hs, mu, k) = st.surf[j];
            let o = crate::pastel::seat_overlap(hs - b * 1000.0 + st.delta, mu, k, stick.hardness);
            if o > 0.0 {
                out.push((i, 2));
            } else if b * 1000.0 - st.delta - hs < crate::pastel::CRUMB_UM {
                out.push((i, 1));
            } else {
                out.push((i, 0));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::mask::Mask;
    use crate::paper::Paper;
    use crate::pastel::{Pose, Stick, StrokePoint};

    fn drawing_paper() -> Canvas {
        Canvas::new_window(600, 1.0, [0.55; 3], None).with_size_mm(150.0).with_paper(Paper::drawing(4))
    }

    fn pose(force: f32, alt_deg: f32) -> Pose {
        Pose { force, alt: alt_deg.to_radians(), az: 45f32.to_radians(), roll: 0.0 }
    }

    fn band(c: &mut Canvas, st: &mut Stick, y0: f32, y1: f32, force: f32) {
        let mut y = y0;
        while y < y1 {
            let pts = [StrokePoint { x: 100.0, y, pose: pose(force, 50.0), speed: 50.0 }, StrokePoint { x: 900.0, y, pose: pose(force, 50.0), speed: 50.0 }];
            c.stick_stroke(st, &pts);
            y += 15.0;
        }
    }

    fn dark(c: &Canvas, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
        let f = c.window();
        let (mut s, mut n) = (0.0, 0);
        for (i, p) in c.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x >= x0 && x < x1 && y >= y0 && y < y1 {
                s += 0.55 - p[0];
                n += 1;
            }
        }
        s / n.max(1) as f32
    }

    fn left_half(c: &Canvas) -> Mask {
        let f = c.window();
        let mut m = Mask::empty(f);
        for y in 0..f.full_h {
            for x in 0..f.full_w / 2 {
                m.data[y * f.full_w + x] = 1.0;
            }
        }
        m
    }

    /// A sheet over the left half: strokes across it leave the picture under
    /// it as it was, and the sheet keeps what fell on it; lifted, it is gone.
    #[test]
    fn a_sheet_keeps_what_it_covers_clean() {
        let mut c = drawing_paper();
        let before = c.pixels().to_vec();
        let m = left_half(&c);
        c.lay_sheet(&m, 150.0, 8.0, [0.85; 3]).unwrap();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        band(&mut c, &mut st, 300.0, 700.0, 2.0);
        let f = c.window();
        let under_clean = c.pixels().iter().enumerate().filter(|(i, _)| f.ux(i % f.w) < 450.0).all(|(i, p)| *p == before[i]);
        assert!(under_clean, "nothing reached the picture under the sheet");
        assert!(dark(&c, 600.0, 850.0, 320.0, 680.0) > 0.05, "the open half took pastel");
        let caught = c.sheet.as_ref().unwrap().a.iter().cloned().fold(0.0, f32::max);
        assert!(caught > 0.3, "the sheet caught pastel: {caught}");
        assert!(c.lift_sheet() && !c.has_sheet());
    }

    /// The hold's contact is where a stroke from there lays.
    #[test]
    fn the_hold_shows_where_a_stroke_lays() {
        let c = drawing_paper();
        let st = Stick::round([0.02; 3], 0.8, 12.0);
        let cells = c.stick_contact(&st, 500.0, 500.0, pose(2.0, 60.0));
        let touch: Vec<usize> = cells.iter().filter(|c| c.1 == 2).map(|c| c.0).collect();
        assert!(!touch.is_empty(), "it touches somewhere");
        let mut d = c.clone();
        let mut s2 = st.clone();
        let p = pose(2.0, 60.0);
        // a stroke through there, slow, so the hand is down at full force at
        // its middle (it lands over 50 ms and lifts over 40)
        d.stick_stroke(&mut s2, &[StrokePoint { x: 460.0, y: 500.0, pose: p, speed: 20.0 }, StrokePoint { x: 540.0, y: 500.0, pose: p, speed: 20.0 }]);
        let laid: Vec<usize> = (0..d.pixels().len()).filter(|&i| d.pixels()[i] != c.pixels()[i]).collect();
        let hit = touch.iter().filter(|i| laid.contains(i)).count();
        assert!(hit * 2 >= touch.len(), "most touching pixels took pastel: {hit} of {}", touch.len());
    }
}
