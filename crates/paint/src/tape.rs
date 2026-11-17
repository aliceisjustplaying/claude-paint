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
//! **It stays down.** Tape holds only on a surface that is touch-dry, or
//! bare; laid over paint that is still wet or tacky its adhesive would bond
//! into the film, and it is refused there. Once down it stays down through
//! later chunks and waits, and a save keeps it (a save from before this
//! keeps none). Paint on it dries where it lies, as paint anywhere does.
//! The picture under it is held as it was when it went down: it was
//! touch-dry then, and its through-drying under the tape isn't modeled
//! beyond its skin (`crate::skin` keeps ageing what baked before).
//!
//! **Lifting.** `lift_tape` takes every piece away with all the paint on
//! it: the picture under it is as it was when the tape went down. Where
//! paint laid since spans the tape's edge, the film must part there, and
//! how follows from its state (`Canvas::lift_tape`, notes/research/
//! masking_and_spray.md §4):
//!
//! - A wet (open) film is a liquid: it parts at the edge.
//! - A set film (past its gel point, `drying::GEL`) is a solid with a
//!   modulus and a work of fracture that grow as it cures to touch-dry,
//!   bonded to the surface beside the tape with an adhesion energy. A crack
//!   starting at the tape's edge turns into the interface beside it when
//!   the adhesion is under about a quarter of the film's work of fracture
//!   (He and Hutchinson's criterion for a crack meeting an interface at 90°
//!   between like materials); otherwise it runs straight down through the
//!   film and tears it at the edge. A film that delaminates is a flap
//!   peeled with the tape: its tears converge at the angle θ with
//!   sin θ = α√(2Bτ)/(2Γt) (Hamm et al. 2008, Kruglova et al. 2011: B the
//!   film's bending stiffness, τ the adhesion, Γ its work of fracture, t
//!   its thickness, α the peel angle). Where no such angle exists (sin θ ≥
//!   1) the flap tears off at once, at the edge; where it does, the film
//!   beside the tape lifts off with it over the width of the peel's ridge,
//!   α√(B/2τ), and what lies beneath it, as it was when the tape went down,
//!   shows again. The film's own thickness along the edge sets B and t
//!   there, so a stroke's thick and thin parts part differently.
//!
//! Not modeled: paint creeping under the tape's edge (the tape is taken as
//! pressed down along its edge), the tape lifting a weak ground, and the
//! tear's wander finer than a pixel.

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

/// How far beside the tape (mm) the picture is kept as it was when the
/// tape went down, so a set film lifted off it there can show it again.
/// A bound on the computation, not a physical constant: the peel's ridge
/// (`Canvas::lift_tape`) is a few mm for a film a few hundred µm thick.
pub const APRON_MM: f32 = 10.0;
/// A set oil film's Young's modulus once touch-dry, Pa: an unaged oil
/// paint film measured 33 ± 3 MPa in micro-tension, rising to 270 MPa after
/// 16 weeks (notes/research/masking_and_spray.md §4).
pub const FILM_E_PA: f32 = 33e6;
/// Its Poisson ratio (an ESTIMATE: glassy-to-rubbery polymer films 0.35–0.45).
pub const FILM_NU: f32 = 0.4;
/// Its work of fracture once touch-dry, J/m²: aged oil paint films measure
/// 10–40 J/m² at first crack (zinc white, umber and lead white about 15).
/// A young film's is taken at the low end, an ESTIMATE: no measurement of
/// a film days old was found.
pub const FILM_FRACTURE_J_M2: f32 = 15.0;
/// The adhesion energy, J/m², of a new oil film to the touch-dry surface it
/// was laid on: dry oil paint or a sealed ground (`ADHESION_PAINT`), or a
/// ground still absorbent, which the new film's oil keyed into
/// (`ADHESION_GROUND`). ESTIMATES within the 1–300 J/m² measured for
/// coatings' interfaces; no measurement for oil on oil was found.
pub const ADHESION_PAINT: f32 = 2.0;
pub const ADHESION_GROUND: f32 = 10.0;
/// He and Hutchinson's ratio for a crack meeting an interface at 90°
/// between elastically like materials: it turns into the interface when
/// the interface's toughness is under 0.26 of the film's.
pub const DEFLECT: f32 = 0.26;
/// The angle the tape is peeled at, radians (an ESTIMATE: pulled up and
/// back, about square to the picture).
pub const PEEL: f32 = std::f32::consts::FRAC_PI_2;

/// What one buffer pixel was when the tape first covered it, or (the
/// apron) when tape first went down beside it.
#[derive(Clone)]
pub(crate) struct Under {
    pub(crate) i: usize,
    pub(crate) px: Rgb,
    pub(crate) backing: Option<crate::mixer::Backing>,
    pub(crate) height: f32,
    pub(crate) film: f32,
    pub(crate) gloss: f32,
    pub(crate) absorb: f32,
    pub(crate) vol: f32,
    pub(crate) lat: crate::wet::Latent,
    pub(crate) hide: crate::wet::Prop,
    pub(crate) stroke: u32,
    pub(crate) touched: u32,
    pub(crate) floor: f32,
    pub(crate) cover: f32,
    pub(crate) solv: Option<f32>,
    pub(crate) dry: Option<crate::drying::Px>,
    pub(crate) skin: u32,
}

/// The tape and stencils on the picture.
#[derive(Clone)]
pub struct Tape {
    /// How thick the covering is over each buffer pixel, µm (0: bare).
    pub(crate) raised: Vec<f32>,
    /// The picture under it, as it was when it was first covered.
    pub(crate) under: Vec<Under>,
    /// The touch-dry picture within `APRON_MM` beside it, as it was when
    /// tape first went down there.
    pub(crate) apron: Vec<Under>,
}

/// What lifting the tape did (`Canvas::lift_tape`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Lift {
    /// The wet paint taken away on it, mm³.
    pub taken_mm3: f32,
    /// The length of edge (mm) where a wet film parted.
    pub parted_mm: f32,
    /// The length of edge (mm) where a set film broke: tore through at the
    /// edge, or lifted off beside it in flaps whose tears run back to it.
    pub torn_mm: f32,
    /// The area beside the tape (mm²) where a set film lifted off with it.
    pub lifted_mm2: f32,
}

/// How a film spanning the tape's edge parts there.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    /// No set film spans it: wet paint parts (or there is none).
    Parts,
    /// The set film tears through at the edge.
    Tears,
    /// The set film lifts off the surface beside the tape over this width, m.
    Lifts(f32),
}

/// How a set film `t_um` thick and `maturity` of the way from its gel point
/// to touch-dry (0..1), bonded with adhesion `tau` (J/m²) beside the tape,
/// parts at the tape's edge (module notes).
pub fn part(t_um: f32, maturity: f32, tau: f32) -> Part {
    let m = maturity.clamp(0.0, 1.0);
    let t = t_um * 1e-6;
    if t <= 0.0 || m <= 0.0 {
        return Part::Tears;
    }
    let e = FILM_E_PA * m;
    let gamma = FILM_FRACTURE_J_M2 * m;
    if tau >= DEFLECT * gamma {
        return Part::Tears;
    }
    let b = e * t.powi(3) / (12.0 * (1.0 - FILM_NU * FILM_NU));
    let sin = PEEL * (2.0 * b * tau).sqrt() / (2.0 * gamma * t);
    if sin >= 1.0 {
        return Part::Tears;
    }
    Part::Lifts(PEEL * (b / (2.0 * tau)).sqrt())
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

/// For every pixel of a `w`×`h` buffer, the distance (pixels) to the
/// nearest pixel that is `on`, and that pixel (two-pass chamfer transform,
/// steps 1 and √2: within 8% of the true distance). `u32::MAX` where none is.
fn nearest(on: &[bool], w: usize, h: usize) -> Vec<(f32, u32)> {
    let mut d: Vec<(f32, u32)> = on.iter().enumerate().map(|(i, &o)| if o { (0.0, i as u32) } else { (f32::INFINITY, u32::MAX) }).collect();
    const D: f32 = std::f32::consts::SQRT_2;
    let relax = |d: &mut Vec<(f32, u32)>, i: usize, j: usize, step: f32| {
        let c = d[j].0 + step;
        if c < d[i].0 {
            d[i] = (c, d[j].1);
        }
    };
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if x > 0 { relax(&mut d, i, i - 1, 1.0); }
            if y > 0 {
                relax(&mut d, i, i - w, 1.0);
                if x > 0 { relax(&mut d, i, i - w - 1, D); }
                if x + 1 < w { relax(&mut d, i, i - w + 1, D); }
            }
        }
    }
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let i = y * w + x;
            if x + 1 < w { relax(&mut d, i, i + 1, 1.0); }
            if y + 1 < h {
                relax(&mut d, i, i + w, 1.0);
                if x + 1 < w { relax(&mut d, i, i + w + 1, D); }
                if x > 0 { relax(&mut d, i, i + w - 1, D); }
            }
        }
    }
    d
}

impl Canvas {
    /// Pixel `i` as it is now (`Under`).
    fn under_at(&self, i: usize) -> Under {
        let n = self.f.w * self.f.h;
        let w = &self.wet;
        Under {
            i,
            px: self.px[i],
            backing: (self.backing.len() == n).then(|| self.backing[i].clone()),
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
            solv: (w.solv.len() == n).then(|| w.solv[i]),
            dry: (w.clock.px.len() == n).then(|| w.clock.px[i]),
            skin: self.skin.top_of(i),
        }
    }

    /// Put pixel `u.i` back as `u` was (the skin baked on it since peeled).
    fn restore(&mut self, u: Under) {
        let n = self.f.w * self.f.h;
        let i = u.i;
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
        if w.solv.len() == n {
            w.solv[i] = u.solv.unwrap_or(0.0);
        }
        if w.clock.px.len() == n {
            w.clock.px[i] = u.dry.unwrap_or(crate::drying::Px::FRESH);
        }
        self.skin.peel_to(i, u.skin);
    }

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
        let mut tape = self.tape.take().unwrap_or_else(|| Box::new(Tape { raised: vec![0.0; n], under: Vec::new(), apron: Vec::new() }));
        let spectral = self.backing.len() == n;
        for &i in &hit {
            if tape.raised[i] == 0.0 {
                let u = self.under_at(i);
                tape.under.push(u);
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
        // the touch-dry picture beside it, as it is now, where nothing has
        // kept it yet
        let reach = APRON_MM / self.px_mm();
        let near = nearest(&tape.raised.iter().map(|&r| r > 0.0).collect::<Vec<_>>(), f.w, f.h);
        let mut kept = vec![false; n];
        for u in tape.apron.iter() {
            kept[u.i] = true;
        }
        for i in 0..n {
            if !kept[i] && tape.raised[i] == 0.0 && near[i].0 <= reach && matches!(self.stage_at_index(i), crate::drying::Stage::Dry) {
                tape.apron.push(self.under_at(i));
            }
        }
        self.tape = Some(tape);
        self.surf_gen += 1;
        let px_mm = self.px_mm();
        Ok(hit.len() as f32 * px_mm * px_mm)
    }

    /// Lift every piece of tape and stencil, with the paint on it: the
    /// picture under it as it was when it was covered. Where a film laid
    /// since spans an edge, it parts there as its state says (`part`, the
    /// module notes): wet paint parts, a set film tears, or lifts off the
    /// picture beside the tape, which shows as it was when the tape went
    /// down. Returns what it did (None if no tape lies on the picture).
    pub fn lift_tape(&mut self) -> Option<Lift> {
        let mut tape = self.tape.take()?;
        let (w, h) = (self.f.w, self.f.h);
        let n = w * h;
        let px_mm = self.px_mm();
        let near = nearest(&tape.raised.iter().map(|&r| r > 0.0).collect::<Vec<_>>(), w, h);
        let mut at = vec![u32::MAX; n];
        for (k, u) in tape.under.iter().enumerate() {
            at[u.i] = k as u32;
        }
        let dry = self.wet.clock.px.len() == n;
        // each edge pixel's parting, judged from the film on the tape there
        let mut parts: std::collections::HashMap<u32, Part> = std::collections::HashMap::new();
        let mut lift = Lift::default();
        let mut part_at = |c: &Canvas, e: u32| -> Part {
            *parts.entry(e).or_insert_with(|| {
                let e = e as usize;
                let Some(u) = tape.under.get(at[e] as usize) else { return Part::Parts };
                // the film baked on it since it went down, and its cure
                let t_um = (c.film[e] - u.film).max(0.0) * crate::surface::COAT_UM;
                if t_um < 1.0 {
                    return Part::Parts;
                }
                // (a canvas that never waited keeps no drying state: what
                // baked on it, `dry` baked touch-dry)
                let sub = if dry { c.wet.clock.px[e].sub } else { 1.0 };
                let maturity = (sub - crate::drying::GEL) / (1.0 - crate::drying::GEL);
                part(t_um, maturity, ADHESION_PAINT)
            })
        };
        // the edge: the tape's pixels next to bare picture
        let mut edge = vec![false; n];
        for i in 0..n {
            if near[i].0 > 0.0 && near[i].0 <= 1.5 && near[i].1 != u32::MAX {
                edge[near[i].1 as usize] = true;
            }
        }
        for e in (0..n).filter(|&e| edge[e]) {
            match part_at(self, e as u32) {
                Part::Parts => {
                    if self.wet.vol[e] > 1e-3 || (self.film[e] - tape.under.get(at[e] as usize).map_or(self.film[e], |u| u.film)) > 1e-3 {
                        lift.parted_mm += px_mm;
                    }
                }
                // (a film that lifts beside the tape broke there too: its
                // flaps' tears run back to the edge, `lifted_mm2` says how
                // far it lifted where that is a pixel or more)
                Part::Tears | Part::Lifts(_) => lift.torn_mm += px_mm,
            }
        }
        // the film beside the tape that lifts with it
        let mut lifted = Vec::new();
        for (k, u) in tape.apron.iter().enumerate() {
            let i = u.i;
            let (d, e) = near[i];
            if tape.raised[i] > 0.0 || e == u32::MAX || (self.film[i] - u.film) * crate::surface::COAT_UM < 1.0 {
                continue;
            }
            // (an absorbent ground beside it holds the film keyed in)
            let tau = if u.absorb > 0.0 { ADHESION_GROUND } else { ADHESION_PAINT };
            let width = match part_at(self, e) {
                Part::Lifts(_) if tau != ADHESION_PAINT => {
                    let ue = &tape.under[at[e as usize] as usize];
                    let t_um = (self.film[e as usize] - ue.film).max(0.0) * crate::surface::COAT_UM;
                    let sub = if dry { self.wet.clock.px[e as usize].sub } else { 1.0 };
                    match part(t_um, (sub - crate::drying::GEL) / (1.0 - crate::drying::GEL), tau) {
                        Part::Lifts(w) => w,
                        _ => 0.0,
                    }
                }
                Part::Lifts(w) => w,
                _ => 0.0,
            };
            if (d - 0.5).max(0.0) * px_mm * 1e-3 < width {
                lifted.push(k);
            }
        }
        let mut taken = 0.0f64;
        for u in std::mem::take(&mut tape.under) {
            taken += (self.wet.vol[u.i] - u.vol).max(0.0) as f64;
            self.restore(u);
        }
        for k in lifted.into_iter().rev() {
            let u = tape.apron.swap_remove(k);
            taken += (self.wet.vol[u.i] - u.vol).max(0.0) as f64;
            lift.lifted_mm2 += px_mm * px_mm;
            self.restore(u);
        }
        self.surf_gen += 1;
        lift.taken_mm3 = (taken * crate::surface::COAT_UM as f64 * 1e-3 * px_mm as f64 * px_mm as f64) as f32;
        Some(lift)
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
        let taken = c.lift_tape().unwrap().taken_mm3;
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

    fn state(c: &Canvas) -> Vec<u8> {
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        b
    }

    /// Tape stays down through a wait and a save: the reopened canvas is
    /// the same bits, tape and all, and lifting it from either gives the
    /// same picture. A save written without tape holds none.
    #[test]
    fn tape_survives_a_wait_and_a_save() {
        let mut c = canvas();
        c.dry();
        let bare = state(&c);
        assert!(!bare.starts_with(b"PAINTC23"));
        c.lay_tape(&[(500.0, 0.0), (500.0, 1000.0)], Cut::Strip { width_mm: 24.0 }).unwrap();
        c.drag(&mut brush(), &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.8, 0.8), None);
        c.wait(60.0);
        assert!(c.has_tape());
        let saved = state(&c);
        assert!(saved.starts_with(b"PAINTC23"));
        let (mut back, _) = Canvas::read_state(&mut std::io::Cursor::new(saved.clone())).unwrap();
        assert!(back.has_tape() && state(&back) == saved, "the reopened canvas differs");
        back.wait(24.0 * 60.0);
        c.wait(24.0 * 60.0);
        let (a, b) = (c.lift_tape().unwrap(), back.lift_tape().unwrap());
        assert_eq!(a, b);
        assert!(state(&c) == state(&back));
        let (old, _) = Canvas::read_state(&mut std::io::Cursor::new(bare)).unwrap();
        assert!(!old.has_tape());
    }

    /// Lifted at once, the wet paint across the tape's edge parts there;
    /// lifted once that paint has set, the film tears along the edge.
    #[test]
    fn wet_paint_parts_and_set_paint_tears_at_the_edge() {
        let stroke = || Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.8, 0.8);
        let mut wet = canvas();
        wet.dry();
        let mut set = wet.clone();
        for c in [&mut wet, &mut set] {
            c.lay_tape(&[(500.0, 0.0), (500.0, 1000.0)], Cut::Strip { width_mm: 24.0 }).unwrap();
            c.drag(&mut brush(), &stroke(), None);
        }
        let w = wet.lift_tape().unwrap();
        assert!(w.parted_mm > 0.0 && w.torn_mm == 0.0 && w.lifted_mm2 == 0.0, "{w:?}");
        set.dry();
        let s = set.lift_tape().unwrap();
        // (where the stroke's fringe laid under a µm, that parts as a liquid would)
        assert!(s.torn_mm + s.lifted_mm2 > 0.0 && s.parted_mm < 0.2 * s.torn_mm, "{s:?}");
        assert_eq!(s.taken_mm3, 0.0, "nothing wet was left to take: {s:?}");
    }

    /// How a set film parts: a thick one tears (the flap's tears can't
    /// converge), a thin one on dry paint lifts off over the peel's ridge,
    /// one still tacky tears (its adhesion beats its weak cohesion), and on
    /// an absorbent ground it tears (keyed in, it holds).
    #[test]
    fn a_set_film_parts_as_its_mechanics_say() {
        assert_eq!(part(100.0, 1.0, ADHESION_PAINT), Part::Tears);
        let Part::Lifts(w) = part(20.0, 1.0, ADHESION_PAINT) else { panic!("{:?}", part(20.0, 1.0, ADHESION_PAINT)) };
        let b = FILM_E_PA * 20e-6f32.powi(3) / (12.0 * (1.0 - FILM_NU * FILM_NU));
        assert!((w - PEEL * (b / (2.0 * ADHESION_PAINT)).sqrt()).abs() < 1e-9 && w > 50e-6 && w < 0.5e-3, "{w}");
        assert_eq!(part(20.0, 0.05, ADHESION_PAINT), Part::Tears);
        assert_eq!(part(20.0, 1.0, ADHESION_GROUND), Part::Tears);
    }

    /// On a fine canvas a thin set film laid across the tape and the dry
    /// paint beside it lifts off with the tape over the peel's ridge: the
    /// paint beside it shows as it was, within the width `part` gives.
    #[test]
    fn a_thin_set_film_lifts_off_beside_the_tape() {
        // 20 mm across 400 px: 0.05 mm pixels
        let mut c = Canvas::new(400, 1.0, hex("#e8e0d0")).with_engine(crate::PIGMENT_MIXER).with_size_mm(20.0);
        c.dry();
        c.lay_tape(&[(500.0, 0.0), (500.0, 1000.0)], Cut::Strip { width_mm: 4.0 }).unwrap();
        let before = c.clone();
        // a set film 20 µm thick, touch-dry, laid over everything since
        for i in 0..c.film.len() {
            c.film[i] += 20.0 / crate::surface::COAT_UM;
            c.px[i] = [0.1, 0.2, 0.6];
        }
        let lift = c.lift_tape().unwrap();
        let Part::Lifts(w) = part(20.0, 1.0, ADHESION_PAINT) else { panic!() };
        let px_mm = c.px_mm();
        let band = (w * 1e3 / px_mm).floor() as usize;
        assert!(band >= 1 && lift.lifted_mm2 > 0.0 && lift.parted_mm == 0.0, "{lift:?}, band {band} px");
        // along the row: the tape's 4 mm, then the lifted band, then the film
        let y = 500.0;
        let edge = (500.0 + 2.0 / c.mm_per_unit) * c.f.scale;
        let at = |x: usize| c.f.index(x as f32 / c.f.scale, y);
        let first = edge.ceil() as usize;
        assert!(c.px[at(first)] == before.px[at(first)], "the film beside the edge stayed");
        assert!(c.px[at(first + band + 2)] == [0.1, 0.2, 0.6], "the film lifted too far");
    }
}
