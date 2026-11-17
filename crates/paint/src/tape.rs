//! Masking tape and a cut stencil on a painting.
//!
//! **What they are.** A strip of masking tape pressed down along a line, or
//! a sheet of stencil film with a shape cut out of it, laid on the painting
//! to keep paint off what it covers. Each is a surface of its own lying on
//! the picture, its thickness above it: a crepe-paper masking tape is about
//! 0.14 mm thick (`TAPE_UM`), a polyester stencil film about 0.19 mm
//! (`STENCIL_UM`; notes/research/masking_and_spray.md).
//!
//! **What happens on it.** Where it lies the surface the brushes, the knife
//! and the rag meet is the tape's: the relief is raised by its thickness
//! (pieces laid over each other add up), and a tool meets it as it meets any
//! step in the relief. Paint a tool lays there lies on the tape, and a
//! tool picks up paint from it as from any surface; paint pushed off it
//! lands on the picture beside it. The tape takes no oil (it is not a
//! ground) and shows its own color under whatever is laid on it.
//!
//! **Lifting.** `lift_tape` takes every piece away with all the paint on
//! it: the picture under it is as it was when the tape went down. Tape
//! holds only on a surface that is touch-dry, or bare; laid over paint that
//! is still wet or tacky its adhesive would bond into the film, and it is
//! refused there. Nothing dries on it: the easel lifts it within the chunk
//! that laid it and doesn't wait with it down, so the paint on it never
//! sets into the picture.
//!
//! Not modeled: paint creeping under the tape's edge (the tape is taken as
//! pressed down along its edge), the film torn at the edge as wet impasto
//! on the tape is pulled away with it, and the tape lifting a weak ground.

use crate::canvas::Canvas;
use crate::color::Rgb;

/// A crepe-paper masking tape's thickness, µm: 3M 2090 is 5.4 mil
/// (0.137 mm), general-purpose masking tapes 0.12–0.17 mm
/// (notes/research/masking_and_spray.md).
pub const TAPE_UM: f32 = 140.0;
/// A polyester stencil film's thickness, µm: stencil Mylar is sold at
/// 5–10 mil (0.125–0.25 mm), 7.5 mil (0.19 mm) the common weight.
pub const STENCIL_UM: f32 = 190.0;
/// The narrowest and widest tape (mm): fine-line masking tapes from 1 mm,
/// painter's masking tape to 48–50 mm, wide rolls to 100 mm.
pub const TAPE_WIDTH_MM: (f32, f32) = (1.0, 100.0);
/// The color of crepe-paper masking tape (an ESTIMATE, its look only: it
/// changes nothing once the tape is lifted).
const TAPE_TONE: Rgb = [0.62, 0.53, 0.36];
/// A frosted polyester film seen on the picture (an ESTIMATE, its look only).
const STENCIL_TONE: Rgb = [0.78, 0.79, 0.78];

/// What one buffer pixel was when the tape first covered it.
#[derive(Clone)]
struct Under {
    i: usize,
    px: Rgb,
    backing: Option<crate::mixer::Backing>,
    height: f32,
    film: f32,
    gloss: f32,
    absorb: f32,
    vol: f32,
    lat: crate::wet::Latent,
    hide: crate::wet::Prop,
    stroke: u32,
    touched: u32,
    floor: f32,
    cover: f32,
    solv: Option<f32>,
    dry: Option<crate::drying::Px>,
    skin: u32,
}

/// The tape and stencils on the picture.
#[derive(Clone)]
pub struct Tape {
    /// How thick the covering is over each buffer pixel, µm (0: bare).
    pub(crate) raised: Vec<f32>,
    /// The picture under it, as it was when it was first covered.
    under: Vec<Under>,
}

/// Which side of a cut a piece covers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cut {
    /// A strip of tape `mm` wide laid straight along each span of the points.
    Strip { width_mm: f32 },
    /// A sheet of stencil film over the whole picture with the polygon cut
    /// out of it: the paint reaches the picture only through the hole.
    Stencil,
}

/// Whether the point (x, y) lies inside the closed polygon `p` (even-odd).
pub(crate) fn inside(p: &[(f32, f32)], x: f32, y: f32) -> bool {
    let mut c = false;
    let n = p.len();
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (p[i], p[j]);
        if (a.1 > y) != (b.1 > y) && x < (b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0 {
            c = !c;
        }
        j = i;
    }
    c
}

/// Whether the point (x, y) lies on a straight strip `half` wide either side
/// of the span a→b, from a to b and no further (a piece of tape torn square).
fn on_strip(a: (f32, f32), b: (f32, f32), half: f32, x: f32, y: f32) -> bool {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    if l2 <= 1e-12 {
        return false;
    }
    let t = ((x - a.0) * dx + (y - a.1) * dy) / l2;
    if !(0.0..=1.0).contains(&t) {
        return false;
    }
    let d = ((x - a.0) * dy - (y - a.1) * dx).abs() / l2.sqrt();
    d <= half
}

impl Canvas {
    /// Lay a piece of tape or a stencil (`cut`) along `pts` (units): a strip
    /// needs two points or more, a stencil's cut three. What it covers is
    /// decided at each pixel's center. Refused, with nothing laid, on paper
    /// (a paper mask there is `lay_sheet`), on raw cloth, where it would
    /// cover paint that is wet or tacky, or where it covers nothing.
    /// Returns the area it covers, mm².
    pub fn lay_tape(&mut self, pts: &[(f32, f32)], cut: Cut) -> Result<f32, String> {
        if self.paper.is_some() {
            return Err("tape on paper is not modeled: a paper mask there is lay_sheet".into());
        }
        if self.soak.is_some() {
            return Err("tape on raw cloth is not modeled: size or prime the canvas first".into());
        }
        if pts.iter().any(|p| !(p.0.is_finite() && p.1.is_finite())) {
            return Err("the points are numbers (units)".into());
        }
        let (caliper, tone) = match cut {
            Cut::Strip { width_mm } => {
                if pts.len() < 2 {
                    return Err("a strip of tape is laid along two points or more".into());
                }
                if !(width_mm.is_finite() && (TAPE_WIDTH_MM.0..=TAPE_WIDTH_MM.1).contains(&width_mm)) {
                    return Err(format!("a tape's width is {} to {} mm (painter's masking tape: 6–50)", TAPE_WIDTH_MM.0, TAPE_WIDTH_MM.1));
                }
                (TAPE_UM, TAPE_TONE)
            }
            Cut::Stencil => {
                if pts.len() < 3 {
                    return Err("a stencil's cut is a closed shape of three points or more".into());
                }
                (STENCIL_UM, STENCIL_TONE)
            }
        };
        let f = self.f;
        let s = f.scale;
        let half = match cut {
            Cut::Strip { width_mm } => 0.5 * width_mm / self.mm_per_unit,
            Cut::Stencil => 0.0,
        };
        let covers = |i: usize| -> bool {
            let (x, y) = (((i % f.w + f.x0) as f32 + 0.5) / s, ((i / f.w + f.y0) as f32 + 0.5) / s);
            match cut {
                Cut::Strip { .. } => pts.windows(2).any(|w| on_strip(w[0], w[1], half, x, y)),
                Cut::Stencil => !inside(pts, x, y),
            }
        };
        let n = f.w * f.h;
        let hit: Vec<usize> = (0..n).filter(|&i| covers(i)).collect();
        if hit.is_empty() {
            return Err("it covers nothing of the canvas".into());
        }
        let raised_before = |i: usize| self.tape.as_ref().map_or(0.0, |t| t.raised[i]);
        if let Some(&i) = hit.iter().find(|&&i| raised_before(i) == 0.0 && !matches!(self.stage_at_index(i), crate::drying::Stage::Dry)) {
            let (x, y) = ((i % f.w + f.x0) as f32 / s, (i / f.w + f.y0) as f32 / s);
            return Err(format!("the paint under it at ({x:.0}, {y:.0}) is still wet or tacky: tape holds only on a touch-dry surface"));
        }
        let mut tape = self.tape.take().unwrap_or_else(|| Box::new(Tape { raised: vec![0.0; n], under: Vec::new() }));
        let spectral = self.backing.len() == n;
        let dry = self.wet.clock.px.len() == n;
        let solv = self.wet.solv.len() == n;
        for &i in &hit {
            if tape.raised[i] == 0.0 {
                let w = &self.wet;
                tape.under.push(Under {
                    i,
                    px: self.px[i],
                    backing: spectral.then(|| self.backing[i].clone()),
                    height: self.height[i],
                    film: self.film[i],
                    gloss: self.gloss[i],
                    absorb: self.absorb[i],
                    vol: w.vol[i],
                    lat: w.lat[i].clone(),
                    hide: w.hide[i],
                    stroke: w.stroke[i],
                    touched: w.touched[i],
                    floor: w.floor[i],
                    cover: w.cover[i],
                    solv: solv.then(|| w.solv[i]),
                    dry: dry.then(|| w.clock.px[i]),
                    skin: self.skin.top_of(i),
                });
                // (its own face: its color, no oil drawn into it)
                self.px[i] = tone;
                if spectral {
                    self.backing[i] = crate::mixer::Backing::default().under(tone);
                }
                self.absorb[i] = 0.0;
            }
            tape.raised[i] += caliper;
            self.height[i] += caliper;
        }
        self.tape = Some(tape);
        self.surf_gen += 1;
        let px_mm = self.px_mm();
        Ok(hit.len() as f32 * px_mm * px_mm)
    }

    /// Lift every piece of tape and stencil, with the paint on it: the
    /// picture under it as it was when it was covered. Returns the wet
    /// paint taken away with it, mm³ (None if none lies on the picture).
    pub fn lift_tape(&mut self) -> Option<f32> {
        let tape = self.tape.take()?;
        let n = self.f.w * self.f.h;
        let (dry, solv) = (self.wet.clock.px.len() == n, self.wet.solv.len() == n);
        let mut taken = 0.0f64;
        for u in tape.under.into_iter() {
            let i = u.i;
            taken += (self.wet.vol[i] - u.vol).max(0.0) as f64;
            self.px[i] = u.px;
            if self.backing.len() == n {
                self.backing[i] = u.backing.unwrap_or_default();
            }
            self.height[i] = u.height;
            self.film[i] = u.film;
            self.gloss[i] = u.gloss;
            self.absorb[i] = u.absorb;
            let w = &mut self.wet;
            w.vol[i] = u.vol;
            w.lat[i] = u.lat;
            w.hide[i] = u.hide;
            w.stroke[i] = u.stroke;
            w.touched[i] = u.touched;
            w.floor[i] = u.floor;
            w.cover[i] = u.cover;
            if solv {
                w.solv[i] = u.solv.unwrap_or(0.0);
            }
            if dry {
                w.clock.px[i] = u.dry.unwrap_or(crate::drying::Px::FRESH);
            }
            self.skin.peel_to(i, u.skin);
        }
        self.surf_gen += 1;
        let px_mm = self.px_mm() as f64;
        Some((taken * crate::surface::COAT_UM as f64 * 1e-3 * px_mm * px_mm) as f32)
    }

    pub fn has_tape(&self) -> bool {
        self.tape.is_some()
    }

    /// How thick the tape is over the point (units), µm (0: none there).
    pub fn tape_at(&self, x: f32, y: f32) -> f32 {
        self.tape.as_ref().map_or(0.0, |t| t.raised[self.f.index(x, y)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bristle::{Gesture, Held, Tool};
    use crate::wet::Paint;
    use crate::hex;

    fn canvas() -> Canvas {
        Canvas::new(200, 1.0, hex("#e8e0d0")).with_engine(crate::PIGMENT_MIXER).with_size_mm(200.0).with_glaze_film(true).with_blend_feather(true).with_brush_skim(true)
    }

    fn brush() -> Held {
        let mut h = Held::new(Tool::filbert(60.0), 7).with_engine(crate::PIGMENT_MIXER);
        h.load(Paint::body(hex("#203080")), 1.0);
        h
    }

    fn wet_at(c: &Canvas, x: f32, y: f32) -> f32 {
        c.wet.vol[c.f.index(x, y)]
    }

    /// A stroke across a strip of tape lays paint on both sides of it and
    /// on it; lifting the tape takes what is on it and leaves the picture
    /// under it as it was, bit for bit, and the paint beside it where it lay.
    #[test]
    fn paint_over_tape_never_reaches_the_canvas_and_goes_with_it() {
        let mut c = canvas();
        let before = c.clone();
        let area = c.lay_tape(&[(500.0, 0.0), (500.0, 1000.0)], Cut::Strip { width_mm: 24.0 }).unwrap();
        assert!((area - 24.0 * 200.0).abs() < 0.08 * area, "{area}");
        assert!((c.tape_at(500.0, 500.0) - TAPE_UM).abs() < 1e-3 && c.tape_at(300.0, 500.0) == 0.0);
        c.drag(&mut brush(), &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.8, 0.8), None);
        let (on, left, right) = (wet_at(&c, 500.0, 500.0), wet_at(&c, 300.0, 500.0), wet_at(&c, 700.0, 500.0));
        assert!(on > 0.05 && left > 0.05 && right > 0.05, "on {on}, left {left}, right {right}");
        let taken = c.lift_tape().unwrap();
        assert!(taken > 0.0 && !c.has_tape());
        assert_eq!(wet_at(&c, 500.0, 500.0), 0.0, "paint on the tape reached the canvas");
        assert!(wet_at(&c, 300.0, 500.0) == left && wet_at(&c, 700.0, 500.0) == right);
        for i in 0..c.px.len() {
            let x = (i % c.f.w) as f32 / c.f.scale;
            if (x - 500.0).abs() < 50.0 {
                assert!(c.px[i] == before.px[i] && c.height[i] == before.height[i] && c.wet.vol[i] == before.wet.vol[i], "pixel {i} under the tape changed");
            }
        }
        assert!(c.lift_tape().is_none());
    }

    /// A stencil lets paint through its hole only.
    #[test]
    fn a_stencil_keeps_paint_off_all_but_its_hole() {
        let mut c = canvas();
        c.lay_tape(&[(400.0, 400.0), (600.0, 400.0), (600.0, 600.0), (400.0, 600.0)], Cut::Stencil).unwrap();
        c.drag(&mut brush(), &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.8, 0.8), None);
        c.lift_tape().unwrap();
        assert!(wet_at(&c, 500.0, 500.0) > 0.05 && wet_at(&c, 300.0, 500.0) == 0.0 && wet_at(&c, 700.0, 500.0) == 0.0);
    }

    /// Tape won't go down on wet paint, on paper or over nothing.
    #[test]
    fn tape_is_refused_on_wet_paint_and_where_it_covers_nothing() {
        let mut c = canvas();
        c.drag(&mut brush(), &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.8, 0.8), None);
        let e = c.lay_tape(&[(500.0, 0.0), (500.0, 1000.0)], Cut::Strip { width_mm: 24.0 }).unwrap_err();
        assert!(e.contains("wet or tacky") && !c.has_tape(), "{e}");
        assert!(c.lay_tape(&[(500.0, 0.0), (500.0, 300.0)], Cut::Strip { width_mm: 0.5 }).is_err());
        assert!(c.lay_tape(&[(5000.0, 0.0), (5000.0, 300.0)], Cut::Strip { width_mm: 10.0 }).unwrap_err().contains("covers nothing"));
        c.dry();
        c.lay_tape(&[(500.0, 0.0), (500.0, 1000.0)], Cut::Strip { width_mm: 24.0 }).unwrap();
    }
}
