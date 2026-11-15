//! A sheet's own water: damp printing paper (notes/plan-intaglio.md P3a,
//! the damp-paper switch). Source: notes/research/damp_paper.md (its
//! sections cited as §n; [M] measured, [D] derived here from measured
//! values or standard physics, [P] a practitioner's account, [E] an
//! estimate, each with the reason it is one).
//!
//! PR #322 (`crate::wet_pastel`) keeps the water a wash sinks into the sheet
//! under each pastel cell. A printer's sheet is wet all through: soaked in a
//! tray, drained, blotted, kept in a damp pack overnight, and drying on the
//! bench between plates. This module keeps that water for the whole sheet
//! (`Damp`: g of water per m², and how many faces are open to the air).
//!
//! - **Moisture content** is g of water per g of dry fiber (`Moisture::mc`).
//!   The fiber's mass is the sheet's grammage, which is measured conditioned
//!   at ISO 187's 50 % RH, less the water it held there (§1).
//! - **Where the water is** (§1). The fiber walls hold water as their
//!   sorption isotherm gives it (GAB, measured on paper), up to fiber
//!   saturation at activity 1. Past that, water fills the pores between the
//!   fibers, as far as #322's share of them a liquid reaches
//!   (`wet_pastel::sheet_capacity_um`). What is left stands on the faces:
//!   the sheet is shiny.
//! - **Soaking** (§2) draws water in through both faces at the sheet's
//!   measured sizing rate (#322's Cobb √t, `wet_pastel::uptake_rate`).
//!   **Draining** the lifted sheet leaves a film on each face that thins as
//!   gravity draws it down (Jeffreys).
//! - **Blotting and the damp pack** (§3) bring the sheets in contact to
//!   capillary equilibrium. The walls fill first (their suction is the
//!   hygroscopic one, MPa), then the free water fills the finest pores of
//!   all the sheets together: the pore heights measured for paper are
//!   exponential about t·ε/(1 − ε) (paper.rs's pores).
//! - **Drying** (§4): an open face evaporates as #322's wet surface does
//!   (`wet_pastel`'s psychrometrics in `climate::STUDIO`) while free water
//!   reaches it, then at the vapour pressure of the walls' water, which the
//!   isotherm gives, down toward the room's equilibrium moisture. A covered
//!   sheet (`faces` 0) keeps its water.
//! - **Stiffness** (§5) falls with the walls' water by Nissan's measured
//!   exponential law and stops falling at fiber saturation.
//! - **Size** (§6) grows with the walls' water by the measured
//!   hygroexpansion coefficients, more across the machine direction than
//!   along it, and stops growing at fiber saturation.

use crate::canvas::Canvas;
use crate::climate::STUDIO;
use crate::paper::Paper;

/// The GAB isotherm of paper, M = M₀ C K a / ((1 − K a)(1 − K a + C K a))
/// (g water per g fiber at water activity a): a bleached kraft pulp at
/// 25 °C, fitted over a = 0.12–0.93 (Parker, Bronlund & Mawson 2006,
/// *Packag. Technol. Sci.* 19:193, their table of published fits) [M]. It
/// gives 0.078 at 50 % RH (sized papers measure 6–8 %) and, at a = 1, 0.29:
/// the fiber saturation point measured for cellulose fibers (0.25–0.35) [D].
const GAB_M0: f64 = 0.0504;
const GAB_C: f64 = 13.9;
const GAB_K: f64 = 0.83;
/// ISO 187's conditioning: a sheet's grammage and size are measured at
/// 50 % RH (and 23 °C).
pub const ISO_RH: f64 = 0.5;
/// Nissan's hydrogen-bond law for paper's modulus above 5 % moisture:
/// ln E falls linearly with moisture content, about 7 % for each 1 % (the
/// slope the same for every paper: Caulfield 1990, after Nissan 1976;
/// Zauscher, Caulfield & Nissan 1996) [M]. Per g/g.
pub const NISSAN_B: f64 = 7.0;
/// Hygroexpansion of an isotropic sheet of short fibers, % strain per %
/// moisture: freely dried hardwood handsheets (eucalyptus, birch, acacia,
/// 0.7–1 mm) measure 0.064–0.107, mean 0.085 (Pulkkinen et al. 2009,
/// *BioResources* 4:126, OPTIDIM) [M].
const BETA_SHORT: f64 = 0.085;
const SHORT_MM: f64 = 0.8;
/// Longer fibers expand less: 0.05 at the long end of Uesaka & Moss's
/// 0.05–0.12 (in Lindner 2018, *J. Mater. Sci.* 53:1) [M]. The fiber
/// length that end belongs to, 3 mm (softwood and rag), is an estimate
/// [E]: the review gives the trend, not the lengths.
const BETA_LONG: f64 = 0.05;
const LONG_MM: f64 = 3.0;
/// A machine-made sheet expands this many times more across the machine
/// direction than along it: mill papers measure 3–5 (0.140 against 0.036
/// %/%RH, Uesaka; 0.210 against 0.048 %/%MC, "Bridging network
/// properties...", 2015) [M]; 4 is the middle.
const CD_OVER_MD: f64 = 4.0;
/// Water: viscosity (Pa·s), density (kg/m³) at 20 °C, and g.
const ETA_WATER: f64 = 1.002e-3;
const RHO_WATER: f64 = 998.2;
const G: f64 = 9.81;
/// The fiber's own density in the engine's sheets, g/cm³: a sheet's
/// grammage over its solid volume, coarseness / (fiber width × collapsed
/// thickness) for `Paper::drawing`'s fiber (0.16 mg/m over 25 × 6 µm) [D].
/// A blotter's porosity is read against it (`Blotter`).
fn fiber_density(p: &Paper) -> f64 {
    p.coarseness as f64 / (p.fiber_um as f64 * p.thick_um as f64) * 1e3
}

/// The isotherm: g of water per g of fiber at activity `a` (0..1).
pub fn isotherm(a: f64) -> f64 {
    let x = GAB_K * a.clamp(0.0, 1.0);
    GAB_M0 * GAB_C * x / ((1.0 - x) * (1.0 - x + GAB_C * x))
}

/// dM/da of the isotherm.
fn isotherm_slope(a: f64) -> f64 {
    // M = M0 C x / D, D = (1 − x)(1 + (C − 1)x), x = K a
    let x = GAB_K * a;
    let d = (1.0 - x) * (1.0 + (GAB_C - 1.0) * x);
    let dd = (GAB_C - 2.0) - 2.0 * (GAB_C - 1.0) * x;
    GAB_K * GAB_M0 * GAB_C * (d - x * dd) / (d * d)
}

/// The activity of the walls' water at moisture `m` (the isotherm's
/// inverse); 1 at fiber saturation and above, where free water is there.
pub fn activity(m: f64) -> f64 {
    if m >= fiber_saturation() {
        return 1.0;
    }
    if m <= 0.0 {
        return 0.0;
    }
    // m (C − 1) x² − (m (C − 2) − M0 C) x − m = 0, x = K a
    let (a2, b) = (m * (GAB_C - 1.0), -(m * (GAB_C - 2.0) - GAB_M0 * GAB_C));
    let x = (-b + (b * b + 4.0 * a2 * m).sqrt()) / (2.0 * a2);
    (x / GAB_K).clamp(0.0, 1.0)
}

/// Fiber saturation, g/g: the isotherm at activity 1 (0.29).
pub fn fiber_saturation() -> f64 {
    isotherm(1.0)
}

/// The room's equilibrium moisture content (`climate::STUDIO`), g/g.
pub fn room_moisture() -> f64 {
    isotherm(STUDIO.rh)
}

/// The dry fiber of a sheet, g/m²: its grammage as measured (ISO 187
/// conditioning), less the water it held there.
pub fn dry_fiber(p: &Paper) -> f64 {
    p.grammage as f64 / (1.0 + isotherm(ISO_RH))
}

/// The free water a sheet's pores take, g/m² (µm): #322's capacity, the
/// share of the pore volume a liquid reaches times porosity and caliper.
pub fn pore_capacity(p: &Paper) -> f64 {
    crate::wet_pastel::sheet_capacity_um(p) as f64
}

/// The pores' mean height, µm: t·ε/(1 − ε) (paper.rs, after Dodson 2001),
/// about which the heights measured in sheets are exponential (Niskanen &
/// Rajatora 2002) [M].
fn pore_height(thick_um: f64, porosity: f64) -> f64 {
    thick_um * porosity / (1.0 - porosity).max(0.05)
}

/// The share of a sheet's pore volume in pores below `x` mean heights:
/// pores exponential in height, each holding volume in proportion to its
/// height, so the volume is Gamma(2) in height.
fn filled_share(x: f64) -> f64 {
    if x <= 0.0 { 0.0 } else { 1.0 - (-x).exp() * (1.0 + x) }
}

/// One sheet of a stack in contact (a sheet and its blotters, a damp pack):
/// g/m² of dry fiber, its pores' volume (µm) and mean height (µm), and the
/// water it holds (g/m²). All are per square meter of the stack: the sheets
/// lie over one another.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Medium {
    pub fiber: f64,
    pub pores: f64,
    pub height: f64,
    pub water: f64,
}

impl Medium {
    /// A sheet of paper `p` holding `water` g/m².
    pub fn sheet(p: &Paper, water: f64) -> Medium {
        Medium { fiber: dry_fiber(p), pores: pore_capacity(p), height: pore_height(p.thick_um as f64, p.porosity as f64), water }
    }
}

/// A blotter: an unsized cotton-linters sheet, whose pores all take water.
/// GB003 blotting paper is 0.8 mm thick (Whatman/Cytiva) [M]; its grammage
/// is not published: a pack of 25 at 300 × 600 mm ships at 1.64 kg, so at
/// most 364 g/m², and 330 is that less the packing [E]. Its fiber is the
/// engine's (`fiber_density`) [E: no linters' dimensions to hand].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Blotter {
    pub mm: f64,
    pub grammage: f64,
}

pub const GB003: Blotter = Blotter { mm: 0.8, grammage: 330.0 };

impl Blotter {
    /// A fresh blotter, at the room's moisture.
    pub fn medium(&self) -> Medium {
        let p = Paper::drawing(0);
        let rho = self.grammage / (self.mm * 1e3);
        let eps = (1.0 - rho / fiber_density(&p)).clamp(0.05, 0.95);
        let fiber = self.grammage / (1.0 + isotherm(ISO_RH));
        Medium { fiber, pores: eps * self.mm * 1e3, height: pore_height(p.thick_um as f64, eps), water: fiber * room_moisture() }
    }

    pub fn valid(&self) -> bool {
        self.mm.is_finite() && self.grammage.is_finite() && (0.05..=5.0).contains(&self.mm) && self.grammage > 0.0 && self.grammage / (self.mm * 1e3) < 0.95 * fiber_density(&Paper::drawing(0))
    }
}

/// Sheets in contact come to equilibrium (blotting, a damp pack; §3), their
/// water conserved. Below every wall's saturation the walls share it at one
/// activity, so at one moisture content (one isotherm). Past it, the walls
/// are full and the free water fills the finest pores of all of them first,
/// to one capillary suction: the pores below one height h*, the same in
/// every sheet (a slit's suction is 2γ/h). What all their pores can't hold
/// stands between them, shared equally.
pub fn equalise(media: &mut [Medium]) {
    let total: f64 = media.iter().map(|m| m.water).sum();
    let fiber: f64 = media.iter().map(|m| m.fiber).sum();
    let fsp = fiber_saturation();
    if total <= fiber * fsp {
        let m = total / fiber.max(1e-12);
        for x in media.iter_mut() {
            x.water = x.fiber * m;
        }
        return;
    }
    let free = total - fiber * fsp;
    let room: f64 = media.iter().map(|m| m.pores).sum();
    if free >= room {
        let stand = (free - room) / media.len() as f64;
        for x in media.iter_mut() {
            x.water = x.fiber * fsp + x.pores + stand;
        }
        return;
    }
    let held = |h: f64| media.iter().map(|m| m.pores * filled_share(h / m.height.max(1e-9))).sum::<f64>();
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    while held(hi) < free {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if held(mid) < free { lo = mid } else { hi = mid }
    }
    let h = 0.5 * (lo + hi);
    for x in media.iter_mut() {
        x.water = x.fiber * fsp + x.pores * filled_share(h / x.height.max(1e-9));
    }
    // (rounding: the water put back is what was there)
    let put: f64 = media.iter().map(|m| m.water).sum();
    if let Some(x) = media.iter_mut().max_by(|a, b| a.pores.total_cmp(&b.pores)) {
        x.water += total - put;
    }
}

/// The mean film a sheet held up by an edge keeps on each face after
/// draining `secs`: a film on a vertical wall thins as h(x, t) = √(η x /
/// (ρ g t)) (Jeffreys 1930), x down from the top, so over a sheet `height_m`
/// tall its mean is ⅔ √(η L/(ρ g t)), µm [D].
pub fn drained_film_um(height_m: f64, secs: f64) -> f64 {
    (2.0 / 3.0) * (ETA_WATER * height_m / (RHO_WATER * G * secs.max(1.0))).sqrt() * 1e6
}

/// The hygroexpansion coefficients of a sheet, % strain per % moisture,
/// (along the machine direction, across it) (§6). A sheet made by hand or
/// on a mold, laid or of long kozo fibers (`Paper::laid`, fibers ≥ 5 mm), is
/// isotropic; a machine-made wove sheet has its fibers along the machine
/// direction, so it expands `CD_OVER_MD` times more across it, about the
/// same geometric mean. The isotropic value falls with fiber length between
/// the measured ends, log-linearly in length between them [E].
pub fn hygroexpansion(p: &Paper) -> (f64, f64) {
    let l = (p.fiber_mm as f64).clamp(SHORT_MM, LONG_MM);
    let t = (l / SHORT_MM).ln() / (LONG_MM / SHORT_MM).ln();
    let iso = BETA_SHORT * (BETA_LONG / BETA_SHORT).powf(t);
    if handmade(p) { (iso, iso) } else { (iso / CD_OVER_MD.sqrt(), iso * CD_OVER_MD.sqrt()) }
}

/// Whether a sheet is made on a hand mold (isotropic) rather than on a
/// paper machine: laid, or of long bast fibers (Japan paper).
pub fn handmade(p: &Paper) -> bool {
    p.laid.is_some() || p.fiber_mm >= 5.0
}

/// The strain (fraction) the walls' water stretches a sheet by beside its
/// size as made (measured at ISO conditioning), along and across the
/// machine direction: β (min(M, M_fsp) − M_iso) for each (§6). Free water
/// past fiber saturation fills the pores and swells nothing.
pub fn strain(p: &Paper, mc: f64) -> (f64, f64) {
    let dm = mc.min(fiber_saturation()) - isotherm(ISO_RH);
    let (md, cd) = hygroexpansion(p);
    (md * dm, cd * dm)
}

/// The sheet's moduli (in-plane, bending, and through its thickness) at
/// moisture `mc`, against the same sheet conditioned (ISO): Nissan's
/// exp(−B (M − M_iso)), held at fiber saturation above it, where the water
/// is free and breaks no more bonds (§5). 0.22 for a damp sheet.
pub fn stiffness(mc: f64) -> f64 {
    (-NISSAN_B * (mc.min(fiber_saturation()) - isotherm(ISO_RH))).exp()
}

/// Water evaporating from an open face whose water is at activity `a`, µm
/// (g/m²) a minute: `wet_pastel`'s wet surface (the still air's boundary
/// layer, the face cooled to its energy balance), its vapour at a times
/// saturation.
pub fn evaporation_at(a: f64) -> f64 {
    crate::wet_pastel::evaporation_at(a)
}

/// The walls' drying clock (§4): τ(u), minutes × (faces / g fiber per m²),
/// from activity 1 down to RH + e^u, on a fixed grid in u = ln(a − RH)
/// (dM = M′(a) da = M′(a)(a − RH) du, and the flux goes to 0 as a − RH, so
/// the integrand in u is smooth). A wall's water dries as dM/dt = −faces·e(a)
/// / fiber, so τ = ∫ M′(a)(a − RH)/e(a) du. Built once.
struct Clock {
    u_hi: f64,
    du: f64,
    tau: Vec<f64>,
}

const CLOCK_N: usize = 4096;
/// The grid's lowest point: a − RH = 10⁻⁷ of (1 − RH), where the sheet is
/// within 10⁻⁷ of the room's moisture.
const CLOCK_SPAN: f64 = 16.118_095_650_958_32; // ln(1e7)

fn clock() -> &'static Clock {
    static C: std::sync::OnceLock<Clock> = std::sync::OnceLock::new();
    C.get_or_init(|| {
        let rh = STUDIO.rh;
        let u_hi = (1.0 - rh).ln();
        let du = CLOCK_SPAN / (CLOCK_N - 1) as f64;
        let f = |u: f64| {
            let a = rh + u.exp();
            isotherm_slope(a) * (a - rh) / evaporation_at(a).max(1e-300)
        };
        let mut tau = Vec::with_capacity(CLOCK_N);
        tau.push(0.0);
        let mut prev = f(u_hi);
        for i in 1..CLOCK_N {
            let u = u_hi - i as f64 * du;
            let cur = f(u);
            tau.push(tau[i - 1] + 0.5 * (prev + cur) * du);
            prev = cur;
        }
        Clock { u_hi, du, tau }
    })
}

impl Clock {
    /// τ at activity `a` (piecewise linear in u).
    fn at(&self, a: f64) -> f64 {
        let d = a - STUDIO.rh;
        if d <= 0.0 {
            return f64::INFINITY;
        }
        let s = ((self.u_hi - d.ln()) / self.du).max(0.0);
        let i = s.floor() as usize;
        if i + 1 >= self.tau.len() {
            return f64::INFINITY;
        }
        let f = s - i as f64;
        self.tau[i] + f * (self.tau[i + 1] - self.tau[i])
    }

    /// The activity at τ (the inverse of `at`).
    fn activity(&self, tau: f64) -> f64 {
        if tau <= 0.0 {
            return 1.0;
        }
        let i = self.tau.partition_point(|&t| t <= tau);
        if i >= self.tau.len() {
            return STUDIO.rh;
        }
        let (t0, t1) = (self.tau[i - 1], self.tau[i]);
        let s = (i - 1) as f64 + (tau - t0) / (t1 - t0).max(1e-300);
        STUDIO.rh + (self.u_hi - s * self.du).exp()
    }
}

/// A sheet's own water, for the whole sheet (`Canvas::damp`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damp {
    /// Water in and on the sheet, g/m² (µm): the walls', the pores', and
    /// what stands on its faces.
    pub water: f64,
    /// Faces open to the room's air: 0 covered (under blotters, plastic, a
    /// board, in a damp pack), 1 lying on the bench, 2 hung up.
    pub faces: u8,
}

impl Damp {
    /// A sheet as it comes, at the room's moisture, lying on the bench.
    pub fn new(p: &Paper) -> Damp {
        Damp { water: dry_fiber(p) * room_moisture(), faces: 1 }
    }

    /// `minutes` of drying (§4): free water evaporates from each open face
    /// at the wet surface's rate, then the walls' water at its activity's,
    /// toward the room's moisture. The walls' part follows its clock
    /// (`Clock`), so a wait split in two dries as one.
    pub fn dry(&mut self, p: &Paper, minutes: f64) {
        if self.faces == 0 || !(minutes > 0.0) {
            return;
        }
        let f = self.faces as f64;
        let fiber = dry_fiber(p);
        let wet = fiber * fiber_saturation();
        let mut t = minutes;
        if self.water > wet {
            let e = f * evaporation_at(1.0);
            let lose = (self.water - wet).min(e * t);
            self.water -= lose;
            t -= lose / e;
            if self.water > wet || t <= 0.0 {
                return;
            }
            self.water = wet;
        }
        let m = self.water / fiber;
        let a = activity(m);
        if a <= STUDIO.rh {
            return;
        }
        let c = clock();
        let tau = c.at(a) + t * f / fiber;
        self.water = fiber * isotherm(c.activity(tau)).min(m);
    }

    /// `minutes` in a tray of water (§2): water goes in through both faces at
    /// the sheet's sizing rate (#322's Cobb √t, or an unsized sheet's
    /// Washburn), from what it holds above its conditioned moisture, up to
    /// the walls' saturation and the pores' capacity. Lifted out and held up
    /// by an edge for `drain_s` seconds, a sheet `height_m` tall keeps a
    /// draining film on each face (`drained_film_um`).
    pub fn soak(&mut self, p: &Paper, minutes: f64, drain_s: f64, height_m: f64) {
        let fiber = dry_fiber(p);
        let base = fiber * isotherm(ISO_RH);
        let cap = fiber * fiber_saturation() + pore_capacity(p);
        let held = (self.water - base).max(0.0);
        let rate = 2.0 * crate::wet_pastel::uptake_rate(p) as f64;
        let took = ((held * held + rate * rate * 60.0 * minutes.max(0.0)).sqrt()).min((cap - base).max(0.0)).max(held);
        // (in the tray it is under water: whatever it held over its capacity stays)
        self.water = self.water.max(base + took);
        self.water += 2.0 * drained_film_um(height_m, drain_s);
        self.faces = 1;
    }

    /// `passes` between fresh blotters, one on each face (§3): each pass
    /// brings the sheet and its two blotters to equilibrium.
    pub fn blot(&mut self, p: &Paper, passes: u32, b: Blotter) {
        for _ in 0..passes {
            let mut m = [Medium::sheet(p, self.water), b.medium(), b.medium()];
            equalise(&mut m);
            self.water = m[0].water;
        }
        self.faces = 1;
    }

    /// The moisture content, g/g.
    pub fn mc(&self, p: &Paper) -> f64 {
        self.water / dry_fiber(p)
    }

    /// Water standing on the faces past what the walls and pores hold, g/m².
    pub fn standing(&self, p: &Paper) -> f64 {
        (self.water - dry_fiber(p) * fiber_saturation() - pore_capacity(p)).max(0.0)
    }

    /// Free water in the pores (and on the faces), g/m².
    pub fn free(&self, p: &Paper) -> f64 {
        (self.water - dry_fiber(p) * fiber_saturation()).max(0.0)
    }
}

/// What a damp sheet is like now (`Canvas::moisture`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Moisture {
    /// g of water per g of dry fiber.
    pub mc: f64,
    /// g of water per m².
    pub water: f64,
    /// Water standing on the faces, g/m² (the sheet is shiny).
    pub standing: f64,
    /// The walls' water activity (1 while free water is in the pores).
    pub activity: f64,
    /// The sheet's size now, mm (width, height), and as made (ISO).
    pub size_mm: (f64, f64),
    pub made_mm: (f64, f64),
    /// Its moduli against the conditioned sheet's (`stiffness`).
    pub stiffness: f64,
    /// Its through-thickness modulus now, MPa (`Paper::z_mpa` × stiffness).
    pub z_mpa: f64,
    /// Open faces (0: covered).
    pub faces: u8,
    /// Whether its fiber bonds are opened by water (free water in its pores,
    /// past fiber saturation): a shape pressed into it now sets as it dries
    /// below saturation, the bonds re-forming in the shape (the plate mark,
    /// intaglio_physics.md §6.3) (§5).
    pub plastic: bool,
}

impl Moisture {
    /// A word for it, as a printer would say it: "shiny" (water standing),
    /// "damp" (free water in the pores: soft, it takes the plate), "drying"
    /// (below fiber saturation, stiffening and shrinking), "dry" (at the
    /// room's moisture).
    pub fn word(&self) -> &'static str {
        if self.standing > 1e-9 {
            "shiny"
        } else if self.mc >= fiber_saturation() - 1e-12 {
            "damp"
        } else if self.mc > room_moisture() + 0.005 {
            "drying"
        } else {
            "dry"
        }
    }
}

impl Canvas {
    /// The sheet's own water, if it has been wetted (`soak` and the rest).
    pub fn damp(&self) -> Option<Damp> {
        self.damp
    }

    /// Whether this canvas is a sheet of paper with nothing on it yet:
    /// what the damp-paper verbs may wet (a drawn or painted sheet would
    /// lose its drawing in the tray, which isn't modelled).
    pub fn blank_sheet(&self) -> Result<&Paper, String> {
        let p = self.paper.as_ref().ok_or("it is not a sheet of paper")?;
        if self.drawing.as_ref().is_some_and(|d| d.cells.iter().any(|c| c.a > 0.0 || c.loose + c.bound > 0.0)) {
            return Err("it is drawn on: soaking a drawing isn't modelled".into());
        }
        let t = self.tally();
        if t.strokes + t.touches > 0 || self.film.iter().any(|&f| f > 0.0) {
            return Err("it is painted on: soaking a painting isn't modelled".into());
        }
        Ok(p)
    }

    fn damp_mut(&mut self) -> Result<(Paper, &mut Damp), String> {
        let p = *self.blank_sheet()?;
        Ok((p, self.damp.get_or_insert_with(|| Damp::new(&p))))
    }

    /// The sheet's height, m.
    fn height_m(&self) -> f64 {
        self.height() as f64 * self.mm_per_unit() as f64 * 1e-3
    }

    /// Soak the sheet `minutes` in a tray, lift it out and drain it
    /// `drain_s` seconds held by its top edge (`Damp::soak`). The clock
    /// passes the soak and the drain; under water the sheet doesn't dry.
    pub fn damp_soak(&mut self, minutes: f64, drain_s: f64) -> Result<(), String> {
        let h = self.height_m();
        let (p, d) = self.damp_mut()?;
        d.faces = 0;
        let wait = (minutes + drain_s / 60.0) as f32;
        self.wait(wait);
        let d = self.damp.as_mut().expect("damp");
        d.soak(&p, minutes, drain_s, h);
        Ok(())
    }

    /// Mist the sheet: `gsm` g of water a square meter sprayed or sponged on
    /// (unsized sheets and Japan paper are only misted).
    pub fn damp_mist(&mut self, gsm: f64) -> Result<(), String> {
        let (_, d) = self.damp_mut()?;
        d.water += gsm.max(0.0);
        Ok(())
    }

    /// Blot the sheet `passes` times between fresh blotters (`Damp::blot`).
    pub fn damp_blot(&mut self, passes: u32, b: Blotter) -> Result<(), String> {
        let (p, d) = self.damp_mut()?;
        d.blot(&p, passes, b);
        Ok(())
    }

    /// Open `faces` of the sheet to the air (0 covers it).
    pub fn damp_cover(&mut self, faces: u8) -> Result<(), String> {
        let (_, d) = self.damp_mut()?;
        d.faces = faces.min(2);
        Ok(())
    }

    /// The sheet as one of a stack in contact (`equalise`).
    pub fn damp_medium(&self) -> Result<Medium, String> {
        let p = self.blank_sheet()?;
        Ok(Medium::sheet(p, self.damp.map_or_else(|| Damp::new(p).water, |d| d.water)))
    }

    /// The sheet's water, as a stack it lay in left it (`pack`), covered.
    pub fn damp_set(&mut self, water: f64, faces: u8) -> Result<(), String> {
        let (_, d) = self.damp_mut()?;
        d.water = water.max(0.0);
        d.faces = faces.min(2);
        Ok(())
    }

    /// The sheet's moisture now: its water, size and stiffness. None for a
    /// canvas that isn't paper.
    pub fn moisture(&self) -> Option<Moisture> {
        let p = self.paper.as_ref()?;
        let d = self.damp.unwrap_or_else(|| Damp::new(p));
        let mc = d.mc(p);
        let made = (self.width() as f64 * self.mm_per_unit() as f64, self.height() as f64 * self.mm_per_unit() as f64);
        // the machine direction runs down the sheet (its height)
        let (md, cd) = strain(p, mc);
        let k = stiffness(mc);
        Some(Moisture {
            mc,
            water: d.water,
            standing: d.standing(p),
            activity: activity(mc),
            size_mm: (made.0 * (1.0 + cd), made.1 * (1.0 + md)),
            made_mm: made,
            stiffness: k,
            z_mpa: p.z_mpa as f64 * k,
            faces: d.faces,
            plastic: mc >= fiber_saturation(),
        })
    }

    /// `dt` minutes of drying for the sheet's own water (`wait`, `dry`).
    pub(crate) fn dry_damp(&mut self, dt: f32) {
        if let (Some(d), Some(p)) = (self.damp.as_mut(), self.paper.as_ref()) {
            d.dry(p, dt as f64);
        }
    }

    /// The free water in the sheet's pores (g/m², µm), which #322's sinking
    /// (`dry_pastel_water`) finds already there: 0 without the switch.
    pub(crate) fn damp_free(&self) -> f32 {
        match (self.damp.as_ref(), self.paper.as_ref()) {
            (Some(d), Some(p)) => d.free(p) as f32,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests;
