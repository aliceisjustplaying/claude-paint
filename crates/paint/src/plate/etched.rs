//! A plate's grounds, stopping-out, rosin and bites (notes/plan-intaglio.md
//! §4, phase P4; the physics in crate::etch, EP).
//!
//! **What is kept.** The plate's coats (`Coats`): a hard ground's thickness
//! and the cells its roller laid it on, the rosin dust laid on it (per size, per µm², with the seed
//! its grains are scattered from), and the tray of acid it bites in (what the
//! bath was made up as, and the metal it has taken in). Per cell (`Etch`,
//! sparse like the cells): whether it is stopped out, the bite of its bared
//! face between the dots, and up to `LINES` classes of needle lines bared
//! through the ground, each with its length per area, its window, its bite
//! and the needle's own groove and burr.
//!
//! **Joining the cells.** What the acid opens is written into the cell's
//! width bins (`Cell::open`, `vol`, `wsum`) and burr and level as any tool's
//! marks are, so the press (P3) reads a bitten line as it reads a drypoint
//! line. Each class remembers what it wrote (`Wrote`), so a further bite
//! takes it out and writes it again from the class's new depth. A clean
//! (`clean`) leaves what was written as plain openings.
//!
//! **The bath on the clock** (plan-intaglio §4.2). A plate goes into its
//! tray (`immerse`) at a time on its own clock and comes out (`lift`) at a
//! later one; between, whatever else happens, it lies in the acid. Its bite
//! is worked out when it comes out, from the minutes it lay there, in the
//! same steps as a bite of those minutes: nothing in the bath depends on
//! what the hand does meanwhile, so the bite of a plate put in, left while
//! other work is done and lifted out is the bite of the same minutes in one.
//! While it lies there the plate keeps the bath it is in and when it went
//! in (`InBath`), which a save, a checkpoint and a handoff carry.
//!
//! **Metal** taken by the acid is counted (`Coats::bitten_um3`): the plate's
//! metal above its flat plus its chips plus what the acid took is what it
//! started with.

use super::*;
use crate::etch::{self, Acid, Bath, Face};
use crate::spectral::{self, Spectrum};

/// Classes of needle lines a cell keeps apart (lines bared at different
/// times bite to different depths); a third is merged into the nearest.
pub const LINES: usize = 2;
/// The time steps of a bite: at most a minute each, so the bath's loading
/// and a still face-up plate's growing diffusion layer are followed.
const STEP_MIN: f64 = 1.0;
/// Past a day in the bath (a plate forgotten in it) a step lengthens to a
/// sixtieth of the time already passed, or the time the bath takes to use a
/// hundredth of its oxidant left at the last step's pace, whichever is
/// shorter (never under a minute): a week costs a few hundred steps more
/// than a day, not ten thousand, and a bath running out is followed.
const DAY_MIN: f64 = 24.0 * 60.0;
const LATE_STEP: f64 = 1.0 / 60.0;
const LATE_USE: f64 = 0.01;
/// A bath is spent when less than this share of the oxidant it was made up
/// with is left: all it could still dissolve is that share of what it held,
/// so nothing it could bite on would show.
const SPENT: f64 = 1e-3;
/// A bath's time is read to the millisecond: the minutes a plate lay in it
/// come from clocks that have added and subtracted many times, and a
/// rounding of a few parts in 10¹⁵ must not make one more step.
const CLOCK_GRAIN_MIN: f64 = 1.0 / 60_000.0;
/// A ground's resistance to the needle: the flow pressure of a beeswax and
/// asphaltum ground, 5 MPa [E: waxes and bitumens indent at 1–10 MPa at room
/// temperature, a hundredth of copper's hardness; unmeasured for a ground].
pub const GROUND_FLOW_PA: f64 = 5e6;

/// What a class wrote into its cell: one width bin's share, volume and width
/// sum, and the burr and level it holds.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Wrote {
    pub bin: u8,
    pub open: f32,
    pub vol: f32,
    pub wsum: f32,
    pub burr_vol: f32,
    pub burr_area: f32,
    pub level: f32,
}

/// Needle lines through the ground in a cell.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Line {
    /// Length per area of the cell (1/µm), the window in the ground (µm),
    /// and the bite at its floor (µm).
    pub lam: f32,
    pub w0: f32,
    pub z: f32,
    /// The needle's own mark in the metal: its section (µm²) and width (µm),
    /// and its burr's section (µm², both sides) and height (µm) as ploughed.
    pub groove: f32,
    pub gw: f32,
    pub burr: f32,
    pub burr_h: f32,
    pub wrote: Wrote,
}

/// A cell's etching state.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Etch {
    pub stopped: bool,
    /// The bite of the bared face between the dots (µm), what it wrote, and
    /// the burr it took over when it began (volume, share).
    pub z: f32,
    pub net: Wrote,
    pub burr0: (f32, f32),
    pub lines: [Line; LINES],
    /// What a solvent rag has dissolved of the cell's coats and not yet
    /// through (µm, from each coat's face; 0: whole, `Plate::wash`): of its
    /// stopping-out varnish, of its ground, and of its melted rosin's lenses.
    pub stop_taken: f32,
    pub ground_taken: f32,
    pub rosin_taken: f32,
}

impl Etch {
    /// Whether a rag has thinned any of its coats.
    pub fn thinned(&self) -> bool {
        self.stop_taken > 0.0 || self.ground_taken > 0.0 || self.rosin_taken > 0.0
    }
}

/// Rosin dust laid on the plate: grains per µm² by size bin
/// (`etch::dust_bin`), the seed they are scattered from, and whether they
/// were melted on, and the cells a solvent rag has taken it off (a bit a
/// cell, row by row; empty where it has taken none: `Plate::wash`).
#[derive(Clone, Debug, PartialEq)]
pub struct Dust {
    pub per_um2: Vec<f32>,
    pub seed: u64,
    pub melted: bool,
    pub gone: Vec<u8>,
}

impl Dust {
    /// Whether a solvent rag has taken this layer off cell `i` (row by row).
    pub fn gone_at(&self, i: usize) -> bool {
        self.gone.get(i / 8).is_some_and(|b| b >> (i % 8) & 1 == 1)
    }
}

/// The plate's tray of acid: as made up, its volume, and the metal it holds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tray {
    pub acid: Acid,
    pub molar0: f64,
    pub litres: f64,
    pub metal_mol: f64,
}

impl Tray {
    /// The oxidant left (mol/L): what the metal taken in has used.
    pub fn molar(&self, metal: Metal) -> f64 {
        (self.molar0 - self.acid.stoichiometry(metal).0 * self.metal_mol / self.litres).max(0.0)
    }
}

/// A plate lying in its tray: the bath it went into and when (minutes on
/// the plate's own clock; for a surface of a session, its canvas's clock,
/// which a rest, a save and a handoff keep).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InBath {
    pub bath: BathSpec,
    pub since: f64,
}

/// A plate's coats and baths (see the module's notes).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Coats {
    /// Hard ground (µm; 0: none), and the cells its roller laid it on (a bit
    /// a cell, row by row; empty with no ground): a cell the roller never
    /// crossed is bare.
    pub ground_um: f32,
    pub laid: Vec<u8>,
    pub dust: Vec<Dust>,
    pub tray: Option<Tray>,
    /// The plate in its tray now, if it is.
    pub bath: Option<InBath>,
    /// Metal the acid has taken (µm³).
    pub bitten_um3: f64,
    tiles: Vec<Option<Box<[Etch]>>>,
}

/// A bath as the plate is put into it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BathSpec {
    pub acid: Acid,
    /// The oxidant's concentration of a fresh bath (mol/L).
    pub molar: f64,
    pub temp_c: f64,
    pub face: Face,
    pub rock: Option<(f64, f64)>,
    pub feather: Option<f64>,
    /// Make a fresh bath even if the plate's tray holds this acid.
    pub fresh: bool,
}

/// A bite as it is asked for in one: into the bath, `minutes`, out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiteSpec {
    pub acid: Acid,
    /// The oxidant's concentration of a fresh bath (mol/L).
    pub molar: f64,
    pub temp_c: f64,
    pub minutes: f64,
    pub face: Face,
    pub rock: Option<(f64, f64)>,
    pub feather: Option<f64>,
    /// Make a fresh bath even if the plate's tray holds this acid.
    pub fresh: bool,
}

impl BiteSpec {
    pub fn bath(&self) -> BathSpec {
        BathSpec { acid: self.acid, molar: self.molar, temp_c: self.temp_c, face: self.face, rock: self.rock, feather: self.feather, fresh: self.fresh }
    }
}

/// What putting a plate into its bath found: the tray (fresh, or the one it
/// had, with the metal it holds), and whether loose rosin floated off.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Immersion {
    pub tray: Tray,
    pub fresh: bool,
    pub washed: bool,
}

/// What a bite did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BiteReport {
    /// The flat face's rate at the start (µm/min), the diffusion layer (µm),
    /// the share under bubbles, the deepest bite (µm), the metal taken (µm³),
    /// the bath's dissolved metal after (g/L) and its oxidant (mol/L).
    pub rate_um_min: f64,
    pub delta_um: f64,
    pub bubbles: f64,
    pub deepest_um: f64,
    pub metal_um3: f64,
    pub loaded_g_l: f64,
    pub molar: f64,
    /// Loose (unmelted) rosin floated off.
    pub washed: bool,
    /// The minutes the plate lay in the bath, and whether the bath was spent
    /// (its oxidant used up) before it came out.
    pub minutes: f64,
    pub spent: bool,
}

/// What a stroke of stopping-out varnish lays (`Plate::stopping`): the
/// length it laid of the path's (mm), whether the brush ran dry, the
/// varnish used and left in it (mm³), the strip's mean width and how far
/// its edge crept (mm), and the path it laid along.
#[derive(Clone, Debug, PartialEq)]
pub struct Stopping {
    pub laid_mm: f64,
    pub total_mm: f64,
    pub dry: bool,
    pub used_mm3: f64,
    pub left_mm3: f64,
    pub width_mm: f64,
    pub spread_mm: f64,
    pub pts: Vec<Point>,
}

/// A melted rosin grain's lens over the plate (µm thick) under a dot of
/// radius `r` µm: its grain's volume spread to `etch::MELT_SPREAD` times its
/// diameter, (2/3)·d/MELT_SPREAD² (the look's).
fn lens_um(r: f64) -> f64 {
    2.0 / 3.0 * (2.0 * r / etch::MELT_SPREAD) / (etch::MELT_SPREAD * etch::MELT_SPREAD)
}

/// The radius (µm) left of a dot of radius `r` whose lens a solvent has
/// dissolved `taken` µm into from its face, None once it is gone: a
/// solvent takes a lens as a spherical cap whose face recedes evenly, the
/// cap's sphere shrinking about its centre, so its foot draws in to
/// √(2ρ′h′ − h′²) with ρ′ and h′ the sphere's radius and the cap's height
/// less what was taken [D, geometry].
fn thinned_lens(r: f64, taken: f64) -> Option<f64> {
    let h = lens_um(r);
    if taken >= h {
        return None;
    }
    let rho = (r * r + h * h) / (2.0 * h);
    let (rho1, h1) = (rho - taken, h - taken);
    Some((2.0 * rho1 * h1 - h1 * h1).max(0.0).sqrt())
}

/// 64-bit mixing (splitmix64).
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn unit(h: u64) -> f64 {
    ((h >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

/// A Poisson count of mean `mean`, from the hash `h` (exact by inversion to
/// a mean of 30; beyond, the normal approximation).
fn poisson(mean: f64, h: u64) -> f64 {
    if mean <= 0.0 {
        return 0.0;
    }
    if mean > 30.0 {
        let (u1, u2) = (unit(mix(h)), unit(mix(h ^ 0x5555)));
        let n = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        return (mean + mean.sqrt() * n).round().max(0.0);
    }
    let u = unit(mix(h));
    let (mut k, mut p, mut cdf) = (0.0f64, (-mean).exp(), 0.0f64);
    loop {
        cdf += p;
        if u <= cdf || k > 200.0 {
            return k;
        }
        k += 1.0;
        p *= mean / k;
    }
}

impl Coats {
    fn new(plate: &Plate) -> Coats {
        Coats { tiles: vec![None; plate.tiles.len()], ..Coats::default() }
    }

    pub fn tiles_used(&self) -> usize {
        self.tiles.iter().filter(|t| t.is_some()).count()
    }

    /// Whether a solvent rag has left any coat thinned (`Etch::thinned`).
    pub fn thinned(&self) -> bool {
        self.tiles.iter().flatten().any(|t| t.iter().any(Etch::thinned))
    }
}

impl Plate {
    /// The plate's coats, if any etching tool has touched it.
    pub fn coats(&self) -> Option<&Coats> {
        self.coats.as_deref()
    }

    fn coats_mut(&mut self) -> &mut Coats {
        if self.coats.is_none() {
            self.coats = Some(Box::new(Coats::new(self)));
        }
        self.coats.as_mut().unwrap()
    }

    /// The etching state of a cell.
    pub fn etch(&self, x: usize, y: usize) -> Etch {
        let Some(c) = self.coats.as_ref() else { return Etch::default() };
        if x >= self.w || y >= self.h {
            return Etch::default();
        }
        let tw = self.w.div_ceil(TILE);
        match &c.tiles[(y / TILE) * tw + x / TILE] {
            Some(t) => t[(y % TILE) * TILE + x % TILE],
            None => Etch::default(),
        }
    }

    fn in_clip(&self, x: usize, y: usize) -> bool {
        x < self.w && y < self.h && self.clip.is_none_or(|(x0, y0, x1, y1)| (x0..x1).contains(&x) && (y0..y1).contains(&y))
    }

    fn set_etch(&mut self, x: usize, y: usize, e: Etch) {
        if !self.in_clip(x, y) {
            return;
        }
        let tw = self.w.div_ceil(TILE);
        let c = self.coats_mut();
        let t = c.tiles[(y / TILE) * tw + x / TILE].get_or_insert_with(|| vec![Etch::default(); TILE * TILE].into_boxed_slice());
        t[(y % TILE) * TILE + x % TILE] = e;
    }

    fn set_cell(&mut self, x: usize, y: usize, c: Cell) {
        if let Some(m) = self.cell_mut(x, y) {
            *m = c;
        }
    }

    /// The hard ground's thickness where it is laid (µm; 0: none).
    pub fn ground_um(&self) -> f32 {
        self.coats.as_ref().map_or(0.0, |c| c.ground_um)
    }

    /// Whether the ground lies on cell (x, y).
    pub fn grounded(&self, x: usize, y: usize) -> bool {
        let i = y * self.w + x;
        x < self.w && y < self.h && self.coats.as_ref().is_some_and(|c| c.ground_um > 0.0 && c.laid.get(i / 8).is_some_and(|b| b >> (i % 8) & 1 == 1))
    }

    /// The ground's thickness on cell (x, y) (µm; 0 where it isn't laid): as
    /// laid, less what a solvent rag has dissolved of it there.
    pub fn ground_at(&self, x: usize, y: usize) -> f64 {
        if self.grounded(x, y) { (self.ground_um() as f64 - self.etch(x, y).ground_taken as f64).max(0.0) } else { 0.0 }
    }

    /// The coat over cell (x, y)'s flat (µm): what is left of its stopping-out
    /// varnish and of its ground.
    pub fn coat_um(&self, x: usize, y: usize) -> f64 {
        let e = self.etch(x, y);
        let v = if e.stopped { (etch::STOP_UM - e.stop_taken as f64).max(0.0) } else { 0.0 };
        v + self.ground_at(x, y)
    }

    /// The pinholes a thinned coat lets the acid through on cell (x, y), as
    /// their number per µm² (each `etch::PINHOLE_UM` across at the face), where
    /// they bare at least `etch::PINHOLE_LEAST` of it (`etch::bared_share`);
    /// None where the coat holds, or there is none.
    pub fn pinholes_at(&self, x: usize, y: usize) -> Option<f64> {
        if !self.etch(x, y).stopped && !self.grounded(x, y) {
            return None;
        }
        let p = etch::bared_share(self.coat_um(x, y));
        (p >= etch::PINHOLE_LEAST).then(|| -(1.0 - p).max(1e-300).ln() / (std::f64::consts::PI * etch::PINHOLE_UM * etch::PINHOLE_UM))
    }

    /// The share of the plate's face the ground lies on.
    pub fn ground_share(&self) -> f64 {
        let Some(c) = self.coats.as_ref().filter(|c| c.ground_um > 0.0) else { return 0.0 };
        c.laid.iter().map(|b| b.count_ones() as f64).sum::<f64>() / (self.w * self.h) as f64
    }

    /// The metal the acid has taken (µm³).
    pub fn bitten_um3(&self) -> f64 {
        self.coats.as_ref().map_or(0.0, |c| c.bitten_um3)
    }

    /// The rosin dots over a cell, from its share of each dust layer's grains
    /// (a Poisson count of the layer's number per area over the cell's): (number
    /// per µm², radius µm) of every size present, melted or not.
    pub fn dots(&self, x: usize, y: usize, melted_only: bool) -> Vec<(f64, f64)> {
        let Some(c) = self.coats.as_ref() else { return Vec::new() };
        let area = self.pitch * self.pitch;
        let mut out = Vec::new();
        for d in &c.dust {
            if melted_only && !d.melted {
                continue;
            }
            let dots = Self::layer_dots(d, x, y, self.w, area);
            let taken = if d.melted { self.etch(x, y).rosin_taken as f64 } else { 0.0 };
            if taken > 0.0 {
                out.extend(dots.into_iter().filter_map(|(n, r)| thinned_lens(r, taken).map(|r| (n, r))));
            } else {
                out.extend(dots);
            }
        }
        out
    }

    /// One dust layer's dots over cell (x, y) (`dots`), none where a solvent
    /// rag has taken it off; `w` the plate's width in cells, `area` a cell's
    /// (µm²).
    fn layer_dots(d: &Dust, x: usize, y: usize, w: usize, area: f64) -> Vec<(f64, f64)> {
        if d.gone_at(y * w + x) {
            return Vec::new();
        }
        let spread = if d.melted { etch::MELT_SPREAD } else { 1.0 };
        let mut out = Vec::new();
        for (b, n) in d.per_um2.iter().enumerate() {
            if *n <= 0.0 {
                continue;
            }
            let h = mix(d.seed ^ mix((x as u64) << 32 | y as u64) ^ mix(b as u64 + 1));
            let count = poisson(*n as f64 * area, h);
            if count > 0.0 {
                out.push((count / area, 0.5 * spread * etch::dust_bin(b).2));
            }
        }
        out
    }

    /// The share of a cell its rosin covers.
    pub fn rosin_cover(&self, x: usize, y: usize) -> f64 {
        let dots = self.dots(x, y, false);
        1.0 - (-std::f64::consts::PI * dots.iter().map(|(n, r)| n * r * r).sum::<f64>()).exp()
    }

    // ------------------------------------------------------------ the tools

    pub(super) fn run_etch(&mut self, op: &Op) -> Report {
        match op {
            Op::Ground { um, roller_mm, pts } => self.ground(*um, *roller_mm as f64, pts),
            Op::Needle { tip_um, cone_deg, pts } => self.needle(*tip_um as f64, *cone_deg as f64, pts),
            Op::StopOut { brush_mm, load, pts } => self.stop_out(*brush_mm as f64, *load as f64, pts),
            Op::Aquatint { per_um2, seed } => {
                self.coats_mut().dust.push(Dust { per_um2: per_um2.clone(), seed: *seed, melted: false, gone: Vec::new() });
                Report::default()
            }
            Op::Melt => {
                for d in &mut self.coats_mut().dust {
                    d.melted = true;
                }
                Report::default()
            }
            Op::Bite { acid, doses } => {
                self.bite_run(*acid, doses);
                Report::default()
            }
            Op::Clean => {
                self.clean();
                Report::default()
            }
            Op::Wash { pad_mm, passes, pts } => self.wash(*pad_mm as f64, *passes, pts),
            _ => unreachable!("not an etching tool"),
        }
    }

    /// An etching needle through the ground (IP §2.1; EP §1). Along the path
    /// it parts the ground where its point is wider than the ground is thick
    /// (the window: the point's width at the ground's top), and carries the
    /// rest of its load on the metal, which it marks by the scratch law
    /// (`groove`): light, a needle barely marks copper.
    fn needle(&mut self, tip_um: f64, cone_deg: f64, pts: &[Point]) -> Report {
        let h_pa = self.hardness_pa();
        let p = self.pitch;
        let cone = cone_deg.to_radians();
        let (samples, total) = Self::samples(pts, p / 4000.0);
        let mut rep = Report { length_mm: total, secs: Self::secs(&samples), ..Report::default() };
        // the point's half-width at a height above its lowest point (tip_profile inverted)
        let half_at = |height: f64| {
            let (mut lo, mut hi) = (0.0f64, 1e5f64);
            for _ in 0..80 {
                let mid = 0.5 * (lo + hi);
                if tip_profile(tip_um, cone, mid) < height { lo = mid } else { hi = mid }
            }
            lo
        };
        // the force the ground bears before the point reaches the metal, where it is laid
        let t_laid = self.ground_um() as f64;
        let f_laid = GROUND_FLOW_PA * std::f64::consts::PI * (half_at(t_laid) * 1e-6).powi(2);
        for &(x, y, (tx, ty), s, pose, ds) in &samples {
            let f = Self::hand(pose.force as f64, s, total, pose.speed as f64);
            let laid = self.at_mm(x, y).is_some_and(|(cx, cy)| self.grounded(cx, cy));
            let (t, f_ground) = if laid { (t_laid, f_laid) } else { (0.0, 0.0) };
            if f <= f_ground {
                continue;
            }
            let lean = (90.0 - (pose.alt as f64).clamp(0.0, 90.0)).to_radians().tan();
            let (ax, ay) = ((pose.az as f64).to_radians().cos(), (pose.az as f64).to_radians().sin());
            let (rx, ry) = (-ty, tx);
            let along = (lean * (ax * tx + ay * ty)).atan().to_degrees();
            let side = (lean * (ax * rx + ay * ry)).atan().to_degrees();
            let g = groove(h_pa, tip_um, cone_deg, f - f_ground, along, side);
            let w0 = 2.0 * half_at(t + g.d);
            if w0 > rep.width_um {
                (rep.width_um, rep.depth_um, rep.beta_deg) = (w0, g.d, g.beta);
            }
            rep.ploughed |= !g.cut && g.area > 0.0;
            rep.cut |= g.cut;
            let len_um = ds * 1000.0;
            rep.groove_um3 += g.area * len_um;
            if g.cut {
                rep.chip_um3 += g.area * len_um;
                self.chip_um3 += g.area * len_um;
            } else {
                rep.burr_um3 += g.area * len_um;
            }
            // across the window: each part of it in its cell, with the line's whole window and marks
            let n = ((w0 / (p / 4.0)).ceil() as usize).max(2);
            let du = w0 / n as f64;
            for i in 0..n {
                let u = -0.5 * w0 + (i as f64 + 0.5) * du;
                let (px, py) = (x + u * rx / 1000.0, y + u * ry / 1000.0);
                let Some((cx, cy)) = self.at_mm(px, py) else { continue };
                if !self.in_clip(cx, cy) {
                    continue;
                }
                let lam = (len_um * du / w0 / (p * p)) as f32;
                let burr = if g.cut { 0.0 } else { (g.burr_l + g.burr_r) as f32 };
                let fresh = Line { lam, w0: w0 as f32, z: 0.0, groove: g.area as f32, gw: g.w as f32, burr, burr_h: g.burr_h as f32, wrote: Wrote::default() };
                let mut e = self.etch(cx, cy);
                add_line(&mut e, fresh);
                self.rewrite(cx, cy, e);
            }
        }
        rep
    }

    /// A hard ground rolled along `pts` (mm) with a roller `roller_mm` long
    /// (IP §2.1): laid `um` thick on every cell its line of contact crosses
    /// over half of (`Plate::sweep`). Rolled again, it lays the same ground
    /// on more cells.
    fn ground(&mut self, um: f32, roller_mm: f64, pts: &[Point]) -> Report {
        let (samples, total) = Self::samples(pts, roller_mm.max(1.0));
        let rep = Report { length_mm: total, secs: Self::secs(&samples), width_um: roller_mm * 1000.0, ..Report::default() };
        let touched = self.touches(&self.sweep(pts, &|_| roller_mm, roller_mm));
        let n = self.w * self.h;
        let c = self.coats_mut();
        c.ground_um = um;
        c.laid.resize(n.div_ceil(8), 0);
        for (i, t) in touched.iter().enumerate() {
            if *t >= 0.5 {
                c.laid[i / 8] |= 1 << (i % 8);
            }
        }
        rep
    }

    /// What stopping-out varnish brushed along `pts` (mm; each point's force
    /// the hand's pressure, 0..1) with a round brush `brush_mm` across loaded
    /// to `load` (0..1 of its belly) lays (EP §3):
    /// - the brush lays the width a wet tuft presses out at its pressure
    ///   (`etch::brush_width_mm`), a film `etch::STOP_UM` thick, until the
    ///   varnish its belly held (`etch::round_belly_mm3`) is spent: then it
    ///   runs dry and lays no more;
    /// - each strip's edge creeps out as the liquid spreads before it sets
    ///   (`etch::stop_out_spread`): some tens of µm, so the edge is the
    ///   brush's;
    /// - where the brush comes down and lifts it leaves the disc of its tuft.
    pub fn stopping(brush_mm: f64, load: f64, pts: &[Point]) -> Stopping {
        let held0 = load.clamp(0.0, 1.0) * etch::round_belly_mm3(brush_mm);
        let h_mm = etch::STOP_UM * 1e-3;
        let w = |p: &Point| etch::brush_width_mm(brush_mm, p.force as f64);
        let total: f64 = pts.windows(2).map(|p| ((p[1].x - p[0].x) as f64).hypot((p[1].y - p[0].y) as f64)).sum();
        let mut held = held0;
        let mut laid = pts.first().copied().into_iter().collect::<Vec<_>>();
        let (mut len, mut dry, mut wsum) = (0.0f64, false, 0.0f64);
        for seg in pts.windows(2) {
            let (a, b) = (seg[0], seg[1]);
            let l = ((b.x - a.x) as f64).hypot((b.y - a.y) as f64);
            let wm = 0.5 * (w(&a) + w(&b));
            let need = l * h_mm * wm;
            if need <= held {
                held -= need;
                len += l;
                wsum += l * wm;
                laid.push(b);
                continue;
            }
            let u = if need > 0.0 { held / need } else { 0.0 };
            let l_ = |p: f32, q: f32| (p as f64 + (q as f64 - p as f64) * u) as f32;
            laid.push(Point { x: l_(a.x, b.x), y: l_(a.y, b.y), force: l_(a.force, b.force), alt: l_(a.alt, b.alt), az: l_(a.az, b.az), speed: l_(a.speed, b.speed) });
            len += l * u;
            wsum += l * u * wm;
            held = 0.0;
            dry = true;
            break;
        }
        let width = if len > 0.0 { wsum / len } else { pts.first().map_or(0.0, w) };
        Stopping { laid_mm: len, total_mm: total, dry, used_mm3: held0 - held, left_mm3: held, width_mm: width, spread_mm: etch::stop_out_spread(width, etch::STOP_UM), pts: laid }
    }

    /// Stopping-out varnish brushed along `pts` (`stopping`): no acid reaches
    /// a cell its strips cover over half of until a clean.
    fn stop_out(&mut self, brush_mm: f64, load: f64, pts: &[Point]) -> Report {
        let s = Self::stopping(brush_mm, load, pts);
        let (samples, _) = Self::samples(pts, 1.0);
        let rep = Report { length_mm: s.laid_mm, secs: Self::secs(&samples), width_um: s.width_mm * 1000.0, ..Report::default() };
        if s.pts.is_empty() || load <= 0.0 {
            return rep;
        }
        let strip = |p: &Point| {
            let w = etch::brush_width_mm(brush_mm, p.force as f64);
            w + 2.0 * etch::stop_out_spread(w, etch::STOP_UM)
        };
        let mut st = self.sweep(&s.pts, &strip, 1.0);
        let ends: Vec<Point> = if s.pts.len() > 1 { vec![s.pts[0], *s.pts.last().unwrap()] } else { vec![s.pts[0]] };
        st.extend(self.dabs(&ends, &strip));
        let touched = self.touches(&st);
        for (i, t) in touched.iter().enumerate() {
            if *t >= 0.5 {
                let (x, y) = (i % self.w, i / self.w);
                let mut e = self.etch(x, y);
                e.stopped = true;
                self.set_etch(x, y, e);
            }
        }
        rep
    }

    /// Solvent: ground, stopping-out and rosin off. What the acid opened stays
    /// as the cells' openings.
    fn clean(&mut self) {
        let c = self.coats_mut();
        c.ground_um = 0.0;
        c.laid.clear();
        c.dust.clear();
        for t in &mut c.tiles {
            *t = None;
        }
    }

    /// A rag dampened with solvent, a pad `pad_mm` across, rubbed along `pts`
    /// (mm; slid, `Plate::slide`, or pressed soft under the contact laws,
    /// `slide_pressed`) `passes` times (EP §8): each cell gets its share of a
    /// pass each time, and the solvent dissolves `etch::solvent_dissolves_um`
    /// of coat a whole pass, from the top down: stopping-out varnish
    /// (`etch::STOP_UM`), then the ground or the rosin under it. The rag
    /// carries off what it dissolves, so a coat it doesn't dissolve through is
    /// left thinner by that much (`Etch::stop_taken`, `ground_taken`,
    /// `rosin_taken`), and the next pass starts on what is left; a thinned
    /// coat still resists the acid until pinholes open in it
    /// (`pinholes_at`). Loose rosin is swept off by any touch; a melted
    /// grain's lens (`lens_um`) is dissolved from its face, its foot drawing in
    /// (`thinned_lens`), and goes once the solvent has taken its height. A
    /// cell left with no coat loses the acid's memory there, as a whole clean
    /// does: what the acid opened stays as its openings. Nothing happens
    /// where the rag doesn't go.
    pub(super) fn wash(&mut self, pad_mm: f64, passes: u32, pts: &[Point]) -> Report {
        let pad = pad_mm.max(0.1);
        let st = if self.contact { self.slide_pressed(pts, &|_| pad, 0.25 * pad) } else { self.slide(pts, &|_| pad, 0.25 * pad) };
        let unit = self.pitch * self.pitch;
        let swept: f64 = st.iter().flat_map(|s| s.cells.iter()).map(|c| c.2 as f64).sum::<f64>() * unit * 1e-6 * passes as f64;
        let length_mm: f64 = pts.windows(2).map(|p| ((p[1].x - p[0].x) as f64).hypot((p[1].y - p[0].y) as f64)).sum();
        let rep = Report { length_mm, secs: swept / super::inked::pace::TARLATAN_MM2_S, width_um: pad * 1000.0, ..Report::default() };
        if self.coats.is_none() {
            return rep;
        }
        let touched = self.touches(&st);
        let d = etch::solvent_dissolves_um();
        let (w, h, area) = (self.w, self.h, unit);
        let ground = self.ground_um() as f64;
        for (i, t) in touched.iter().enumerate() {
            if *t <= 0.0 {
                continue;
            }
            let (x, y) = (i % w, i / w);
            let mut b = *t as f64 * passes as f64 * d;
            let mut e = self.etch(x, y);
            let was = e;
            if e.stopped {
                let left = etch::STOP_UM - e.stop_taken as f64;
                if b < left {
                    e.stop_taken += b as f32;
                    self.set_etch(x, y, e);
                    continue;
                }
                e.stopped = false;
                e.stop_taken = 0.0;
                b -= left;
            }
            let mut grounded = self.grounded(x, y);
            if grounded {
                let left = ground - e.ground_taken as f64;
                if b >= left {
                    let c = self.coats.as_mut().unwrap();
                    c.laid[i / 8] &= !(1 << (i % 8));
                    grounded = false;
                    e.ground_taken = 0.0;
                    b -= left;
                } else {
                    e.ground_taken += b as f32;
                    b = 0.0;
                }
            }
            let mut rosin = false;
            if !grounded {
                let taken = e.rosin_taken as f64 + b;
                let c = self.coats.as_mut().unwrap();
                for dl in c.dust.iter_mut() {
                    let dots = Self::layer_dots(dl, x, y, w, area);
                    if dots.is_empty() {
                        continue;
                    }
                    // (the tallest grain's lens: a melted grain's radius is its spread's)
                    let lens = dots.iter().map(|&(_, r)| lens_um(r)).fold(0.0, f64::max);
                    if !dl.melted || taken >= lens {
                        if dl.gone.is_empty() {
                            dl.gone = vec![0u8; (w * h).div_ceil(8)];
                        }
                        dl.gone[i / 8] |= 1 << (i % 8);
                    } else {
                        rosin = true;
                    }
                }
                e.rosin_taken = if rosin { taken as f32 } else { 0.0 };
            }
            if !grounded && !e.stopped && !rosin {
                e = Etch::default();
            }
            if e != was {
                self.set_etch(x, y, e);
            }
        }
        self.tidy_coats();
        rep
    }

    /// The coats with what a rag has taken off them gone: no ground left
    /// anywhere is none, a rosin layer gone from every cell that had grains
    /// is none, and a tile of cells with no etching state is none (so a plate
    /// washed all over is as `clean` leaves it).
    fn tidy_coats(&mut self) {
        let (w, h, area) = (self.w, self.h, self.pitch * self.pitch);
        let Some(c) = self.coats.as_mut() else { return };
        if c.ground_um > 0.0 && c.laid.iter().all(|&b| b == 0) {
            c.ground_um = 0.0;
            c.laid.clear();
        }
        c.dust.retain(|d| d.gone.is_empty() || (0..w * h).any(|i| !d.gone_at(i) && !Self::layer_dots(d, i % w, i / w, w, area).is_empty()));
        for t in c.tiles.iter_mut() {
            if t.as_ref().is_some_and(|t| t.iter().all(|e| *e == Etch::default())) {
                *t = None;
            }
        }
    }

    /// The cells (row by row) that hold a coat: ground, stopping-out varnish
    /// or rosin.
    pub fn coat_cells(&self) -> usize {
        if self.coats.is_none() {
            return 0;
        }
        (0..self.w * self.h).filter(|&i| {
            let (x, y) = (i % self.w, i / self.w);
            self.grounded(x, y) || self.etch(x, y).stopped || !self.dots(x, y, false).is_empty()
        }).count()
    }

    /// Whether the plate holds a ground and nothing else of the etching
    /// tools' (more ground can be rolled onto it).
    pub fn only_ground(&self) -> bool {
        self.coats.as_ref().is_some_and(|c| c.ground_um > 0.0 && c.dust.is_empty() && c.tiles.iter().all(|t| t.is_none()))
    }

    /// Whether the plate holds a coat or a bite's state (ground, varnish,
    /// rosin, lines or a face the acid is working): the other tools want it
    /// cleaned first.
    pub fn coated(&self) -> bool {
        self.coats.as_ref().is_some_and(|c| c.ground_um > 0.0 || !c.dust.is_empty() || c.tiles.iter().any(|t| t.is_some()))
    }

    /// A cell's classes written again from their state: what each wrote is
    /// taken out, and its present section put in (a bitten line: the floor
    /// and the undercut from its window, `etch::line_section`, with the
    /// needle's groove; its burr as bitten from both faces, `etch::fin_left`;
    /// the bared face: its pits or its lowered level, `etch::network`).
    fn rewrite(&mut self, x: usize, y: usize, mut e: Etch) {
        let mut c = self.cell(x, y);
        let k = self.coats.as_ref().and_then(|c| c.tray.map(|t| t.acid.etch_factor())).unwrap_or(Acid::Ferric.etch_factor());
        // out with the old
        let take = |c: &mut Cell, w: &Wrote| {
            let b = w.bin as usize;
            let zap = |v: &mut f32, d: f32| {
                *v -= d;
                if v.abs() < 1e-7 {
                    *v = 0.0;
                }
                *v = v.max(0.0);
            };
            zap(&mut c.open[b], w.open);
            zap(&mut c.vol[b], w.vol);
            zap(&mut c.wsum[b], w.wsum);
            zap(&mut c.burr_vol, w.burr_vol);
            zap(&mut c.burr_area, w.burr_area);
            c.level -= w.level;
        };
        for l in &e.lines {
            take(&mut c, &l.wrote);
        }
        take(&mut c, &e.net);
        // in with the new
        let put = |c: &mut Cell, share: f64, w: f64, vol: f64, burr_vol: f64, burr_area: f64, level: f64| {
            let mut out = Wrote { level: level as f32, ..Wrote::default() };
            if vol > 0.0 && share > 0.0 {
                let b = bin_of(w);
                let room = (1.0 - c.open_share()).max(0.0);
                let s = (share as f32).min(room);
                out.bin = b as u8;
                out.open = s;
                out.vol = vol as f32;
                out.wsum = s * w as f32;
                c.open[b] += out.open;
                c.vol[b] += out.vol;
                c.wsum[b] += out.wsum;
            }
            if burr_vol > 0.0 {
                out.burr_vol = burr_vol as f32;
                out.burr_area = (burr_area as f32).min(1.0 - c.burr_area).max(0.0);
                c.burr_vol += out.burr_vol;
                c.burr_area += out.burr_area;
            }
            c.level += out.level;
            out
        };
        let dir = c.dir;
        for l in e.lines.iter_mut() {
            if l.lam <= 0.0 {
                continue;
            }
            let (lam, z) = (l.lam as f64, l.z as f64);
            let (area, w) = if z > 0.0 {
                let (a, w) = etch::line_section(l.w0 as f64, z, k);
                (a + l.groove as f64, w.max(l.gw as f64))
            } else {
                (l.groove as f64, l.gw as f64)
            };
            let f = if z > 0.0 { etch::fin_left(0.5 * l.burr as f64, l.burr_h as f64, z) } else { 1.0 };
            let (bv, ba) = if l.burr > 0.0 && l.burr_h > 0.0 && f > 0.0 { (lam * l.burr as f64 * f * f, lam * 2.0 * l.burr as f64 / l.burr_h as f64 * f) } else { (0.0, 0.0) };
            l.wrote = put(&mut c, lam * w, w, lam * area, bv, ba, 0.0);
        }
        if e.z > 0.0 {
            let n = match self.pinholes_at(x, y) {
                Some(nh) => etch::pinholes(nh, etch::PINHOLE_UM, e.z as f64, k),
                None => etch::network(&self.dots(x, y, true), e.z as f64, k),
            };
            let (b0v, b0a) = e.burr0;
            let (bv, ba) = if b0v > 0.0 && b0a > 0.0 {
                let h = 2.0 * b0v as f64 / b0a as f64;
                let base = b0a as f64 * self.pitch;
                let f = etch::fin_left(0.5 * h * base, h, e.z as f64);
                (b0v as f64 * f * f, b0a as f64 * f)
            } else {
                (0.0, 0.0)
            };
            e.net = put(&mut c, n.share, n.width, n.vol, bv, ba, -n.level);
        } else {
            e.net = Wrote::default();
        }
        if c.burr_vol <= 0.0 {
            c.burr_area = 0.0;
            c.side = 0;
        }
        c.dir = dir;
        self.set_cell(x, y, c);
        self.set_etch(x, y, e);
    }

    /// The cells the bath reaches: where the ground lies, those with needle
    /// lines; where it doesn't (no ground, or cells its roller missed), every
    /// cell of the face (between its dots, if it has any); never a stopped one;
    /// and any whose thinned coat has pinholes (`pinholes_at`).
    fn exposed(&self) -> Vec<(usize, usize)> {
        let (x0, y0, x1, y1) = self.clip.unwrap_or((0, 0, self.w, self.h));
        let mut out = Vec::new();
        for y in y0..y1.min(self.h) {
            for x in x0..x1.min(self.w) {
                let e = self.etch(x, y);
                if !e.stopped && (!self.grounded(x, y) || e.lines.iter().any(|l| l.lam > 0.0)) || self.pinholes_at(x, y).is_some() {
                    out.push((x, y));
                }
            }
        }
        out
    }

    /// The bite's steps (`doses`: each step's bite of a flat face, µm, and
    /// its diffusion layer, µm) applied to every exposed cell: each front
    /// advances by `etch::advance` (a slot slows as it deepens), and the cell
    /// is written again. Returns the metal taken (µm³) and the deepest bite.
    fn bite_run(&mut self, acid: Acid, doses: &[(f32, f32)]) -> (f64, f64) {
        // loose rosin floats off in the bath
        if let Some(c) = self.coats.as_mut() {
            c.dust.retain(|d| d.melted);
        }
        let k = acid.etch_factor();
        // (a crop rebuilt from the vectors has no tray: its lines are written with this acid's undercut)
        if self.coats.as_ref().and_then(|c| c.tray).is_none_or(|t| t.acid != acid) {
            self.coats_mut().tray = Some(Tray { acid, molar0: 0.0, litres: 1.0, metal_mol: 0.0 });
        }
        if doses.is_empty() {
            return (0.0, 0.0);
        }
        let a = self.pitch * self.pitch;
        let cells = self.exposed();
        let (mut taken, mut deepest) = (0.0f64, 0.0f64);
        for (x, y) in cells {
            let t = self.ground_at(x, y);
            let holes = self.pinholes_at(x, y);
            let mut e = self.etch(x, y);
            let before = self.cell(x, y).height() as f64;
            let grounded = self.grounded(x, y);
            if grounded && !e.stopped {
                for l in e.lines.iter_mut().filter(|l| l.lam > 0.0) {
                    let mut z = l.z as f64;
                    for &(dose, delta) in doses {
                        z = etch::advance(z, dose as f64, delta as f64, etch::slot_share(l.w0 as f64, delta as f64), t);
                    }
                    l.z = z as f32;
                    deepest = deepest.max(z);
                }
            }
            if holes.is_some() {
                // (through a thinned coat's pinholes: each a pit under its mouth in the coat)
                if e.z <= 0.0 {
                    let c = self.cell(x, y);
                    e.burr0 = (c.burr_vol, c.burr_area);
                    e.net = Wrote { burr_vol: c.burr_vol, burr_area: c.burr_area, ..Wrote::default() };
                }
                let mouth = self.coat_um(x, y);
                let mut z = e.z as f64;
                for &(dose, delta) in doses {
                    let g = etch::slot_share(2.0 * (etch::PINHOLE_UM + z / k), delta as f64);
                    z = etch::advance(z, dose as f64, delta as f64, g, mouth);
                }
                e.z = z as f32;
                deepest = deepest.max(z);
            } else if !grounded && !e.stopped {
                if e.z <= 0.0 {
                    let c = self.cell(x, y);
                    e.burr0 = (c.burr_vol, c.burr_area);
                    e.net = Wrote { burr_vol: c.burr_vol, burr_area: c.burr_area, ..Wrote::default() };
                }
                // the pits' mouth: the free path between the dots at the face, step by step
                let dots = self.dots(x, y, true);
                let mut z = e.z as f64;
                for &(dose, delta) in doses {
                    let edge: f64 = dots.iter().map(|&(n, r)| n * (r - z / k).max(0.0)).sum();
                    let g = if edge > 0.0 { etch::slot_share(1.0 / (2.0 * edge), delta as f64) } else { 0.0 };
                    z = etch::advance(z, dose as f64, delta as f64, g, 0.0);
                }
                e.z = z as f32;
                deepest = deepest.max(z);
            }
            self.rewrite(x, y, e);
            taken += (before - self.cell(x, y).height() as f64) * a;
        }
        self.coats_mut().bitten_um3 += taken;
        (taken, deepest)
    }

    /// The plate in its tray, if it is.
    pub fn in_bath(&self) -> Option<InBath> {
        self.coats.as_ref().and_then(|c| c.bath)
    }

    /// The plate's tray, as a bath of `bath` finds it (EP §2): the tray it
    /// has if it holds this acid made up to this strength and no fresh bath
    /// is asked for, else a fresh one, as deep as `etch::TRAY_DEPTH_M` over
    /// the plate and its margins.
    fn tray_for(&self, bath: &BathSpec) -> (Tray, bool) {
        let (wm, hm) = (self.width_mm as f64 * 1e-3, self.height_mm as f64 * 1e-3);
        let litres = (wm + 2.0 * etch::TRAY_MARGIN_M) * (hm + 2.0 * etch::TRAY_MARGIN_M) * etch::TRAY_DEPTH_M * 1000.0;
        match self.coats.as_ref().and_then(|c| c.tray) {
            Some(t) if !bath.fresh && t.acid == bath.acid && (t.molar0 - bath.molar).abs() < 1e-9 => (t, false),
            _ => (Tray { acid: bath.acid, molar0: bath.molar, litres, metal_mol: 0.0 }, true),
        }
    }

    /// Put the plate into its tray at `now` (minutes on its clock): it lies
    /// in the acid until `lift`. Loose rosin floats off as it goes in. A
    /// plate already in a bath is refused (it comes out first).
    pub fn immerse(&mut self, bath: BathSpec, now: f64) -> Result<Immersion, String> {
        if let Some(b) = self.in_bath() {
            return Err(format!("the plate is in its {} bath already (since {} min on its clock)", b.bath.acid.name(), b.since));
        }
        if !now.is_finite() {
            return Err("the bath's clock isn't a time".into());
        }
        let (tray, fresh) = self.tray_for(&bath);
        let c = self.coats_mut();
        let washed = c.dust.iter().any(|d| !d.melted);
        c.dust.retain(|d| d.melted);
        c.tray = Some(tray);
        c.bath = Some(InBath { bath: BathSpec { fresh: false, ..bath }, since: now });
        Ok(Immersion { tray, fresh, washed })
    }

    /// Take the plate out of its tray at `now` (minutes on its clock): it
    /// has bitten for the minutes since it went in (`bite_for`).
    pub fn lift(&mut self, now: f64) -> Result<BiteReport, String> {
        let Some(b) = self.in_bath() else { return Err("the plate isn't in a bath".into()) };
        let minutes = now - b.since;
        if !(minutes >= -CLOCK_GRAIN_MIN) {
            return Err(format!("the plate's clock reads {now} min, before it went into the bath at {} min", b.since));
        }
        let rep = self.bite_for(&b.bath, (minutes / CLOCK_GRAIN_MIN).round().max(0.0) * CLOCK_GRAIN_MIN);
        self.coats_mut().bath = None;
        Ok(rep)
    }

    /// A copy of the plate as it would come out of its bath at `now` (a look
    /// at a plate still in it); the plate itself as it is if it isn't in one.
    pub fn as_lifted(&self, now: f64) -> Plate {
        let mut p = self.clone();
        if p.in_bath().is_some() {
            let since = p.in_bath().unwrap().since;
            let _ = p.lift(now.max(since));
        }
        p
    }

    /// A bite in one: into the bath at 0, out at `spec.minutes` (EP §2).
    pub fn bite(&mut self, spec: BiteSpec) -> BiteReport {
        let washed = self.immerse(spec.bath(), 0.0).expect("bite: the plate is in a bath already").washed;
        BiteReport { washed, ..self.lift(spec.minutes).expect("bite: the plate went in") }
    }

    /// The bath's work on the plate over `minutes` in it, in steps (`steps`).
    /// Each step's rate is the bath's (`etch::transport`) at the oxidant left,
    /// the metal taken in the earlier steps having used some; after the bite
    /// the tray holds exactly the metal the plate lost. A spent bath (no
    /// oxidant left) bites no more.
    fn bite_for(&mut self, spec: &BathSpec, minutes: f64) -> BiteReport {
        let metal = self.metal;
        let (wm, hm) = (self.width_mm as f64 * 1e-3, self.height_mm as f64 * 1e-3);
        let tray = self.coats.as_ref().and_then(|c| c.tray).filter(|t| t.acid == spec.acid).unwrap_or_else(|| self.tray_for(spec).0);
        self.coats_mut().tray = Some(tray);
        let l_nat = match spec.face {
            Face::Down => wm * hm / (2.0 * (wm + hm)),
            _ => hm,
        };
        let still_up = spec.face == Face::Up && spec.rock.is_none();
        let (_, v_m) = etch::molar(metal);
        let mut doses = Vec::new();
        let mut rep = BiteReport { minutes, ..BiteReport::default() };
        let mut metal_mol = tray.metal_mol;
        let before = self.bitten_um3();
        let mut deepest = 0.0f64;
        let bath_at = |metal_mol: f64| Bath { acid: spec.acid, molar: Tray { metal_mol, ..tray }.molar(metal), molar0: tray.molar0, temp_c: spec.temp_c, face: spec.face, rock: spec.rock, feather: spec.feather };
        // (the rate the plate starts at, reported if it comes straight out: over its first minute)
        let tr0 = etch::transport(&bath_at(metal_mol), metal, l_nat, wm, STEP_MIN * 60.0);
        (rep.rate_um_min, rep.delta_um, rep.bubbles) = (tr0.rate_um_min, tr0.delta_um, tr0.bubbles);
        let early = steps(minutes);
        let end = minutes * 60.0;
        let (mut i, mut t1, mut pace) = (0usize, 0.0f64, 0.0f64);
        while t1 < end {
            // (the first day in its steps; past it, steps as `LATE_STEP` and `LATE_USE` allow)
            let t2 = match early.get(i) {
                Some(&(_, t2)) => t2,
                None => {
                    let left = Tray { metal_mol, ..tray }.molar(metal) * tray.litres / spec.acid.stoichiometry(metal).0;
                    let by_use = if pace > 0.0 { LATE_USE * left / pace } else { f64::INFINITY };
                    (t1 + (t1 * LATE_STEP).min(by_use).max(STEP_MIN * 60.0)).min(end)
                }
            };
            let bath = bath_at(metal_mol);
            if bath.molar < SPENT * tray.molar0 {
                rep.spent = true;
                break;
            }
            let tr = etch::transport(&bath, metal, l_nat, wm, t2);
            let dose = if still_up {
                // the diffusion layer's growth integrated: ∫k dt = 2t·k(t), from t1 to t2
                let tr1 = etch::transport(&bath, metal, l_nat, wm, t1.max(1e-9));
                (tr.rate_um_min / 60.0) * 2.0 * t2 - if t1 > 0.0 { (tr1.rate_um_min / 60.0) * 2.0 * t1 } else { 0.0 }
            } else {
                tr.rate_um_min / 60.0 * (t2 - t1)
            };
            if i == 0 {
                (rep.rate_um_min, rep.delta_um, rep.bubbles) = (tr.rate_um_min, tr.delta_um, tr.bubbles);
            }
            let step = (dose as f32, tr.delta_um.min(1e9) as f32);
            doses.push(step);
            // (each step bites the plate and loads the bath before the next: the plate's vectors keep
            // the steps, which bite a rebuilt crop to the same depths, `bite_run`)
            let (taken, z) = self.bite_run(spec.acid, &[step]);
            metal_mol += taken * 1e-18 / v_m;
            pace = taken * 1e-18 / v_m / (t2 - t1);
            deepest = deepest.max(z);
            (i, t1) = (i + 1, t2);
        }
        // (no step: a plate in and straight out; its vectors still wash its loose rosin off)
        if doses.is_empty() {
            self.bite_run(spec.acid, &[]);
        }
        self.ops.push(Op::Bite { acid: spec.acid, doses });
        let taken = self.bitten_um3() - before;
        let tray = Tray { metal_mol: tray.metal_mol + taken * 1e-18 / v_m, ..tray };
        self.coats_mut().tray = Some(tray);
        rep.deepest_um = deepest;
        rep.metal_um3 = taken;
        rep.loaded_g_l = tray.metal_mol * etch::molar(metal).0 * 1000.0 / tray.litres;
        rep.molar = tray.molar(metal);
        rep
    }
}

/// A bite's time steps (s from the plate going in) over its first day:
/// `minutes` cut into equal steps of at most `STEP_MIN`, or minute steps for
/// the day if it is longer (`bite_for` steps on past it).
fn steps(minutes: f64) -> Vec<(f64, f64)> {
    if minutes <= 0.0 {
        return Vec::new();
    }
    let span = minutes.min(DAY_MIN);
    let n = (span / STEP_MIN).ceil().max(1.0) as usize;
    let dt = span * 60.0 / n as f64;
    (0..n).map(|i| (i as f64 * dt, (i + 1) as f64 * dt)).collect()
}

/// A coated plate's look (EP §8): a ground or a varnish as a film of bitumen
/// over the metal (`etch::film_over_metal`), the needle's windows in it bright
/// metal, or a bitten line's walls once the acid has opened it; rosin as its
/// dots over the face, each cell's share of them its own (the speckle of a
/// grain).
pub(super) struct Looks {
    ground: Option<Rgb>,
    stop: Rgb,
    rosin: Option<Rgb>,
}

impl Looks {
    pub(super) fn new(p: &Plate, c: &Coats) -> Looks {
        let film = |um: f64| spectral::to_rgb(&etch::film_over_metal(p.metal, etch::GROUND_N, etch::ground_alpha, um));
        let ground = (c.ground_um > 0.0).then(|| film(c.ground_um as f64));
        let stop = film(etch::STOP_UM);
        // rosin's grains by area: melted, a flat lens 0.39 of the grain's diameter thick (its volume
        // spread to 1.3 times its diameter); loose, a powder [E: diffuse, 0.6 of the light through a grain]
        let (mut a, mut ad, mut melted) = (0.0f64, 0.0f64, true);
        for d in &c.dust {
            melted &= d.melted;
            for (b, n) in d.per_um2.iter().enumerate() {
                let dm = etch::dust_bin(b).2;
                a += *n as f64 * dm * dm;
                ad += *n as f64 * dm * dm * dm;
            }
        }
        let rosin = (a > 0.0).then(|| {
            let d = ad / a;
            if melted {
                let t = 2.0 / 3.0 * d / (etch::MELT_SPREAD * etch::MELT_SPREAD);
                spectral::to_rgb(&etch::film_over_metal(p.metal, etch::ROSIN_N, etch::rosin_alpha, t))
            } else {
                let s: Spectrum = std::array::from_fn(|i| (0.6 * (-etch::rosin_alpha(spectral::wavelength(i) as f64) * d).exp()) as f32);
                spectral::to_rgb(&s)
            }
        });
        Looks { ground, stop, rosin }
    }

    /// A stopped-out or grounded cell's colour (None: the bare face's, then `rosin`).
    pub(super) fn cell(&self, p: &Plate, x: usize, y: usize, _c: &Cell, flat: &Rgb, walls: &[Rgb]) -> Option<Rgb> {
        let e = p.etch(x, y);
        if e.stopped {
            return Some(self.stop);
        }
        let g = self.ground.filter(|_| p.grounded(x, y))?;
        let mut px = g;
        let mut bare = 0.0f32;
        for l in e.lines.iter().filter(|l| l.lam > 0.0) {
            let s = (l.lam * l.w0).min(1.0 - bare);
            if s <= 0.0 {
                continue;
            }
            bare += s;
            // through the window: the floor and walls the acid opened (a V of mean depth: walls 4d/w), or
            // the metal the needle bared
            let t = if l.z > 0.0 {
                let wall = ((4.0 * 0.5 * l.z as f64 / l.w0.max(1e-3) as f64).atan().to_degrees().round() as usize).min(90);
                walls[wall]
            } else {
                *flat
            };
            for q in 0..3 {
                px[q] += s * (t[q] - g[q]);
            }
        }
        Some(px)
    }

    /// The rosin over a bare cell: its share of the cell in rosin's colour.
    pub(super) fn rosin(&self, p: &Plate, x: usize, y: usize, px: Rgb) -> Rgb {
        let Some(r) = self.rosin else { return px };
        let c = p.rosin_cover(x, y) as f32;
        std::array::from_fn(|q| px[q] * (1.0 - c) + r[q] * c)
    }
}

/// A needle line joins the cell's classes: lines not yet bitten join the
/// unbitten class; a new class while there is room; else the nearest in
/// depth, length-weighted.
fn add_line(e: &mut Etch, l: Line) {
    let join = |a: &mut Line, b: Line| {
        let (la, lb) = (a.lam as f64, b.lam as f64);
        let t = la + lb;
        let avg = |x: f32, y: f32| ((x as f64 * la + y as f64 * lb) / t) as f32;
        let burr_h = if a.burr + b.burr > 0.0 { ((a.burr_h as f64 * a.burr as f64 * la + b.burr_h as f64 * b.burr as f64 * lb) / (a.burr as f64 * la + b.burr as f64 * lb)) as f32 } else { 0.0 };
        *a = Line { lam: t as f32, w0: avg(a.w0, b.w0), z: avg(a.z, b.z), groove: avg(a.groove, b.groove), gw: avg(a.gw, b.gw), burr: avg(a.burr, b.burr), burr_h, wrote: a.wrote };
    };
    if let Some(a) = e.lines.iter_mut().find(|a| a.lam > 0.0 && a.z == l.z) {
        return join(a, l);
    }
    if let Some(a) = e.lines.iter_mut().find(|a| a.lam <= 0.0) {
        *a = Line { wrote: a.wrote, ..l };
        return;
    }
    let i = (0..LINES).min_by(|&i, &j| (e.lines[i].z - l.z).abs().total_cmp(&(e.lines[j].z - l.z).abs())).unwrap();
    join(&mut e.lines[i], l);
}

// ---------------------------------------------------------------- persistence (PLAT v2)

/// The coats (`thin`: each etched cell then also carries what a rag took of
/// its varnish, ground and rosin, `Etch::stop_taken` and the rest).
pub(super) fn write_etch(c: &Coats, w: &mut impl Write, thin: bool) -> io::Result<()> {
    put32(w, c.ground_um.to_bits())?;
    put64(w, c.bitten_um3.to_bits())?;
    put64(w, c.dust.len() as u64)?;
    for d in &c.dust {
        put64(w, d.seed)?;
        // (bit 1: the cells a solvent rag took it off follow its grading)
        put64(w, d.melted as u64 | (!d.gone.is_empty() as u64) << 1)?;
        put64(w, d.per_um2.len() as u64)?;
        for v in &d.per_um2 {
            put32(w, v.to_bits())?;
        }
        if !d.gone.is_empty() {
            put64(w, d.gone.len() as u64)?;
            w.write_all(&d.gone)?;
        }
    }
    match c.tray {
        None => put64(w, 0)?,
        Some(t) => {
            put64(w, 1)?;
            put64(w, t.acid.word())?;
            for v in [t.molar0, t.litres, t.metal_mol] {
                put64(w, v.to_bits())?;
            }
        }
    }
    // the plate in its bath: the bath (acid, strength, °C, face, rock, feather) and when it went in
    match c.bath {
        None => put64(w, 0)?,
        Some(b) => {
            put64(w, 1)?;
            put64(w, b.bath.acid.word())?;
            put64(w, b.bath.face.word())?;
            let (rocks, (tilt, period)) = (b.bath.rock.is_some(), b.bath.rock.unwrap_or((0.0, 0.0)));
            put64(w, rocks as u64 | (b.bath.feather.is_some() as u64) << 1)?;
            for v in [b.bath.molar, b.bath.temp_c, tilt, period, b.bath.feather.unwrap_or(0.0), b.since] {
                put64(w, v.to_bits())?;
            }
        }
    }
    let used: Vec<usize> = (0..c.tiles.len()).filter(|&i| c.tiles[i].is_some()).collect();
    put64(w, used.len() as u64)?;
    let mut buf = Vec::new();
    for i in used {
        put64(w, i as u64)?;
        buf.clear();
        for e in c.tiles[i].as_ref().unwrap().iter() {
            buf.push(e.stopped as u8);
            buf.extend_from_slice(&e.z.to_bits().to_le_bytes());
            wrote_bytes(&e.net, &mut buf);
            for v in [e.burr0.0, e.burr0.1] {
                buf.extend_from_slice(&v.to_bits().to_le_bytes());
            }
            for l in &e.lines {
                for v in [l.lam, l.w0, l.z, l.groove, l.gw, l.burr, l.burr_h] {
                    buf.extend_from_slice(&v.to_bits().to_le_bytes());
                }
                wrote_bytes(&l.wrote, &mut buf);
            }
            if thin {
                for v in [e.stop_taken, e.ground_taken, e.rosin_taken] {
                    buf.extend_from_slice(&v.to_bits().to_le_bytes());
                }
            }
        }
        w.write_all(&buf)?;
    }
    // the cells the ground is laid on
    put64(w, c.laid.len() as u64)?;
    w.write_all(&c.laid)?;
    Ok(())
}

fn wrote_bytes(o: &Wrote, buf: &mut Vec<u8>) {
    buf.push(o.bin);
    for v in [o.open, o.vol, o.wsum, o.burr_vol, o.burr_area, o.level] {
        buf.extend_from_slice(&v.to_bits().to_le_bytes());
    }
}

/// The bytes of one cell's etching state.
const ETCH_BYTES: usize = 1 + 4 + 25 + 8 + LINES * (28 + 25);
/// And what a rag took of its coats, on a thinned plate's.
const THIN_BYTES: usize = 3 * 4;

/// `write_etch` read back; without `thin` every coat is whole.
pub(super) fn read_etch(p: &Plate, r: &mut impl Read, thin: bool) -> io::Result<Coats> {
    let bad = |m: &str| io::Error::new(io::ErrorKind::InvalidData, format!("plate: {m}"));
    let f32_ = |r: &mut dyn Read| -> io::Result<f32> { Ok(f32::from_bits(get32(r)?)) };
    let mut c = Coats::new(p);
    c.ground_um = f32_(r)?;
    c.bitten_um3 = f64::from_bits(get64(r)?);
    if !c.ground_um.is_finite() || c.ground_um < 0.0 || !c.bitten_um3.is_finite() {
        return Err(bad("its coats are damaged"));
    }
    let nd = get64(r)?;
    if nd > 1 << 16 {
        return Err(bad("too many rosin layers"));
    }
    for _ in 0..nd {
        let seed = get64(r)?;
        let flags = get64(r)?;
        if flags > 3 {
            return Err(bad("a rosin layer is damaged"));
        }
        let melted = flags & 1 == 1;
        let n = get64(r)?;
        if n != etch::DUST_BINS as u64 {
            return Err(bad("a rosin layer of another grading"));
        }
        let mut per = Vec::with_capacity(n as usize);
        for _ in 0..n {
            per.push(f32_(r)?);
        }
        let mut gone = Vec::new();
        if flags & 2 == 2 {
            let m = get64(r)?;
            if m != (p.w * p.h).div_ceil(8) as u64 {
                return Err(bad("a rosin layer's cleaned cells are damaged"));
            }
            gone = vec![0u8; m as usize];
            r.read_exact(&mut gone)?;
        }
        c.dust.push(Dust { per_um2: per, seed, melted, gone });
    }
    c.tray = match get64(r)? {
        0 => None,
        1 => {
            let acid = Acid::of(get64(r)?).ok_or_else(|| bad("an acid this build doesn't know"))?;
            let (molar0, litres, metal_mol) = (f64::from_bits(get64(r)?), f64::from_bits(get64(r)?), f64::from_bits(get64(r)?));
            if ![molar0, litres, metal_mol].iter().all(|v| v.is_finite() && *v >= 0.0) || litres <= 0.0 {
                return Err(bad("its tray is damaged"));
            }
            Some(Tray { acid, molar0, litres, metal_mol })
        }
        _ => return Err(bad("its tray is damaged")),
    };
    c.bath = match get64(r)? {
        0 => None,
        1 => {
            let acid = Acid::of(get64(r)?).ok_or_else(|| bad("an acid this build doesn't know"))?;
            let face = Face::of(get64(r)?).ok_or_else(|| bad("its bath is damaged"))?;
            let flags = get64(r)?;
            let mut v = [0.0f64; 6];
            for x in v.iter_mut() {
                *x = f64::from_bits(get64(r)?);
            }
            let [molar, temp_c, tilt, period, feather, since] = v;
            if flags > 3 || !v.iter().all(|x| x.is_finite()) || molar < 0.0 {
                return Err(bad("its bath is damaged"));
            }
            let bath = BathSpec { acid, molar, temp_c, face, rock: (flags & 1 == 1).then_some((tilt, period)), feather: (flags & 2 == 2).then_some(feather), fresh: false };
            Some(InBath { bath, since })
        }
        _ => return Err(bad("its bath is damaged")),
    };
    let used = get64(r)?;
    if used > c.tiles.len() as u64 {
        return Err(bad("more etched tiles than it has"));
    }
    let mut buf = vec![0u8; TILE * TILE * (ETCH_BYTES + if thin { THIN_BYTES } else { 0 })];
    let mut last = None;
    for _ in 0..used {
        let i = get64(r)? as usize;
        if i >= c.tiles.len() || last.is_some_and(|l| i <= l) {
            return Err(bad("an etched tile out of place"));
        }
        last = Some(i);
        r.read_exact(&mut buf)?;
        let mut cells = vec![Etch::default(); TILE * TILE];
        let mut at = 0usize;
        let rd = |at: &mut usize| {
            let v = f32::from_bits(u32::from_le_bytes(buf[*at..*at + 4].try_into().unwrap()));
            *at += 4;
            v
        };
        let wr = |at: &mut usize| {
            let bin = buf[*at];
            *at += 1;
            Wrote { bin, open: rd(at), vol: rd(at), wsum: rd(at), burr_vol: rd(at), burr_area: rd(at), level: rd(at) }
        };
        for e in cells.iter_mut() {
            e.stopped = match buf[at] {
                0 => false,
                1 => true,
                _ => return Err(bad("an etched cell is damaged")),
            };
            at += 1;
            e.z = rd(&mut at);
            e.net = wr(&mut at);
            e.burr0 = (rd(&mut at), rd(&mut at));
            for l in e.lines.iter_mut() {
                *l = Line { lam: rd(&mut at), w0: rd(&mut at), z: rd(&mut at), groove: rd(&mut at), gw: rd(&mut at), burr: rd(&mut at), burr_h: rd(&mut at), wrote: wr(&mut at) };
            }
            if thin {
                (e.stop_taken, e.ground_taken, e.rosin_taken) = (rd(&mut at), rd(&mut at), rd(&mut at));
            }
            if e.net.bin as usize >= BINS || e.lines.iter().any(|l| l.wrote.bin as usize >= BINS) || [e.stop_taken, e.ground_taken, e.rosin_taken].iter().any(|v| !v.is_finite() || *v < 0.0) {
                return Err(bad("an etched cell is damaged"));
            }
        }
        c.tiles[i] = Some(cells.into_boxed_slice());
    }
    let n = get64(r)?;
    let want = if c.ground_um > 0.0 { (p.w * p.h).div_ceil(8) as u64 } else { 0 };
    if n != want {
        return Err(bad("its ground's cells are damaged"));
    }
    c.laid = vec![0u8; n as usize];
    r.read_exact(&mut c.laid)?;
    Ok(c)
}

#[cfg(test)]
mod tests;
