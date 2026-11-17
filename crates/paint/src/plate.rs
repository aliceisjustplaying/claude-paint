//! An intaglio plate: copper or zinc, worked with a drypoint needle, a
//! burnisher, a scraper and a roulette (notes/plan-intaglio.md §2–§3, phase
//! P2; the easel's `--@ plate`). Physics: notes/research/intaglio_physics.md
//! (cited as **IP §n**) and notes/research/plate_physics.md (**PP §n**),
//! with their tags: [M] measured, [D] derived, [P] practitioners, [E]
//! estimate.
//!
//! **A statistical plate (plan-intaglio I1).** The plate is laid out in
//! cells at the pitch of the sheet it will print on (about 0.18 mm at the
//! easel's 2400 px over 430 mm), while a drypoint line is 40–170 µm wide
//! (IP §1.3). So each cell keeps what is below and above its surface as a
//! distribution, not as a picture of the lines:
//! - its openings binned by width (`Cell::open`, `vol`, `wsum`: the share
//!   of the cell open, the volume, and the width, per bin), because what the
//!   cloth of a wipe and the paper of the press meet depends on each
//!   opening's width (a tarlatan's sag goes as w⁴) and on its depth;
//! - its burr: volume, share of the cell, which sides of the line it stands
//!   on, and the line's direction (`burr_vol`, `burr_area`, `side`, `dir`);
//! - its surface's own height (`level`: a scraped dish is negative).
//!
//! Those are what the press and the wipe (phase P3) read. Beside them the
//! plate keeps every tool use as it was given (`ops`: the paths, forces and
//! angles), so a crop is rasterized again at its own scale (`rebuilt`) and
//! the line shapes come back from the vectors.
//!
//! **Metal is conserved.** A needle that ploughs pushes the groove's metal
//! into burr; a burnisher pushes burr and the rims of lines into the lines;
//! only a cutting needle, the scraper and (later) acid take metal away, and
//! what they take is counted (`chip_um3`). `metal_um3` is the plate's metal
//! above its original flat: a test of every tool.
//!
//! **Its look** is metal optics: Fresnel reflection from the metal's complex
//! refractive index (Johnson & Christy 1972 for copper, Werner et al. 2009
//! for zinc; plan-intaglio I4), at each facet's angle, with the light that a
//! V-groove traps by reflecting it from wall to wall (PP §4). The relief
//! (lines, burr, a scraped dish) is lit by the canvas's own light code
//! (`PreparedLighting`).

use crate::Rgb;
use crate::spectral::{self, Spectrum};
use std::io::{self, Read, Write};

// ---------------------------------------------------------------- metals

/// Standard gravity, for Vickers hardness in kgf/mm² (exact).
const G: f64 = 9.80665;
/// A Vickers number is the load over the indent's true (sloping) area; the
/// load over its projected area (Meyer's hardness), which is what carries a
/// scratch or an indent, is 2/1.8544 times it: the 136° pyramid's faces
/// make the true area 1/sin 68° of the projected one [D, geometry].
pub const MEYER_PER_VICKERS: f64 = 2.0 / 1.8544;

/// Elastic constants of a hardened steel tool (needle, burnisher, scraper):
/// E 210 GPa, ν 0.30 [M, handbook tables; PP §2].
pub const STEEL_E_PA: f64 = 210e9;
pub const STEEL_NU: f64 = 0.30;

/// The metal of a plate. The period box (plan-intaglio I5: an 1890s Paris
/// print box) holds copper and zinc; steel facing (`Facing`) comes later
/// (phase P7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metal {
    Copper,
    Zinc,
}

/// How hard a copper plate was rolled (zinc is sold one way: rolled).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temper {
    Annealed,
    EighthHard,
    HalfHard,
    /// Zinc's.
    Rolled,
}

impl Metal {
    pub fn word(self) -> u64 {
        match self {
            Metal::Copper => 0,
            Metal::Zinc => 1,
        }
    }

    pub fn of(w: u64) -> Option<Metal> {
        match w {
            0 => Some(Metal::Copper),
            1 => Some(Metal::Zinc),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Metal::Copper => "copper",
            Metal::Zinc => "zinc",
        }
    }

    /// Young's modulus (Pa) and Poisson's ratio: copper 125 GPa (IP §1.1:
    /// 120–130) and 0.34, zinc 100 GPa (IP §1.1) and 0.25 [M, handbook
    /// tables; PP §2].
    pub fn elastic(self) -> (f64, f64) {
        match self {
            Metal::Copper => (125e9, 0.34),
            Metal::Zinc => (100e9, 0.25),
        }
    }
}

impl Temper {
    pub fn word(self) -> u64 {
        match self {
            Temper::Annealed => 0,
            Temper::EighthHard => 1,
            Temper::HalfHard => 2,
            Temper::Rolled => 3,
        }
    }

    pub fn of(w: u64) -> Option<Temper> {
        match w {
            0 => Some(Temper::Annealed),
            1 => Some(Temper::EighthHard),
            2 => Some(Temper::HalfHard),
            3 => Some(Temper::Rolled),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Temper::Annealed => "annealed",
            Temper::EighthHard => "eighth-hard",
            Temper::HalfHard => "half-hard",
            Temper::Rolled => "rolled",
        }
    }
}

/// Vickers hardness (HV) of a metal in a temper (IP §1.1, plan §2.1):
/// copper annealed 45 HV and half-hard 90 HV [M, the middle of 40–50 and
/// 80–100]; eighth-hard 65 HV [E: between the two measured tempers]; rolled
/// zinc 45 HV [M]. None: the metal isn't sold so.
pub fn vickers(metal: Metal, temper: Temper) -> Option<f64> {
    match (metal, temper) {
        (Metal::Copper, Temper::Annealed) => Some(45.0),
        (Metal::Copper, Temper::EighthHard) => Some(65.0),
        (Metal::Copper, Temper::HalfHard) => Some(90.0),
        (Metal::Zinc, Temper::Rolled) => Some(45.0),
        _ => None,
    }
}

/// The load a contact carries per projected area while the metal flows
/// (Pa): Meyer's hardness from the Vickers number.
pub fn hardness_pa(hv: f64) -> f64 {
    hv * G * 1e6 * MEYER_PER_VICKERS
}

/// A plate's facing: an electrodeposited layer of iron (steel facing, from
/// 1857; IP §1.6). A hook for phase P7 (`steel_face`): nothing in P2 makes
/// one, but the plate keeps and saves it, and its hardness is the
/// surface's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Facing {
    pub hv: f32,
    pub um: f32,
}

// ---------------------------------------------------------------- the tools' physics

/// The attack angle above which a hard tip cuts a chip from copper rather
/// than ploughing it aside: copper's measured 45° < β_c < 85° [M, Sedriks &
/// Mulhearn 1963–64; IP §1.2], taken at its middle (plan §3.1).
pub const BETA_C_DEG: f64 = 60.0;
/// A drypoint burr's height over the groove's depth [E: IP §1.4's 0.5–1,
/// its middle; no profilometry of drypoint burr exists (IP §9, Unknown 1)].
pub const BURR_HEIGHT_PER_DEPTH: f64 = 0.75;
/// A scraper's edge: a three-sided blade's, 60° [D: an equilateral
/// section; IP §1.7 "a three-sided blade"].
pub const SCRAPER_EDGE_DEG: f64 = 60.0;

/// A groove a needle ploughs or cuts: its size from scratch hardness, its
/// regime from the attack angle, and its burr (IP §1.2–§1.4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Groove {
    /// Width at the surface and depth (µm), and cross-section (µm²).
    pub w: f64,
    pub d: f64,
    pub area: f64,
    /// The attack angle of the needle's leading face (degrees), and whether
    /// it cut (β ≥ β_c: the groove's metal leaves as a chip).
    pub beta: f64,
    pub cut: bool,
    /// The share of the groove's section that left as a chip (0..1): all or
    /// nothing by `groove`, the streams of metal the face meets steeper than
    /// β_c by `groove_streams`.
    pub chip: f64,
    /// The burr's cross-section on each side (µm²; left and right of the
    /// way the needle went) and its height (µm).
    pub burr_l: f64,
    pub burr_r: f64,
    pub burr_h: f64,
    tip: f64,
    cone: f64,
}

/// The height (µm) of a needle's point above its lowest point at `u` µm
/// off its axis: a sphere of radius `tip` (µm) blending into a cone of
/// half-angle `cone` (radians) where they are tangent (IP §1.3, "the tip").
fn tip_profile(tip: f64, cone: f64, u: f64) -> f64 {
    let u = u.abs();
    let rc = tip * cone.cos();
    if u <= rc { tip - (tip * tip - u * u).max(0.0).sqrt() } else { tip * (1.0 - cone.sin()) + (u - rc) / cone.tan() }
}

/// ∫₀ᵃ tip_profile du, in closed form.
fn tip_integral(tip: f64, cone: f64, a: f64) -> f64 {
    let rc = tip * cone.cos();
    let sphere = |b: f64| tip * b - (0.5 * b * (tip * tip - b * b).max(0.0).sqrt() + 0.5 * tip * tip * (b / tip).clamp(-1.0, 1.0).asin());
    if tip <= 0.0 {
        return a * a / (2.0 * cone.tan());
    }
    if a <= rc {
        sphere(a)
    } else {
        sphere(rc) + tip * (1.0 - cone.sin()) * (a - rc) + (a - rc) * (a - rc) / (2.0 * cone.tan())
    }
}

/// A needle of point radius `tip_um` and cone half-angle `cone_deg` drawn
/// with `force` N over a metal of hardness `h_pa`, leaning `along` degrees
/// toward the way it moves (pulled, its top ahead of the point: positive)
/// and `side` degrees to the right of it.
///
/// - **Size** (IP §1.3, [D]): the front half of the contact carries the load
///   at the scratch hardness, so w = √(8F/(πH)); the depth is the point's at
///   that half-width (the sphere's chord, then the cone's).
/// - **Regime** (IP §1.2, [M]): the leading face meets the plate at
///   β = 90° − θ − τ; below β_c it ploughs, above it cuts.
/// - **Burr** (IP §1.4, [D]): ploughing removes nothing, so the burr's
///   section is the groove's (the degree of wear is near 0 when ploughing,
///   and goes to 1 when cutting: Hokkirigawa & Kato 1988). Each flank pushes
///   up the metal of its side of the groove, whose half-width at the depth d
///   goes as the tangent of the flank's angle from the vertical, so a needle
///   leaning to one side throws its burr to the other; leaning past its
///   half-angle, all of it [D; P: drypoint manuals]. The rounded point is
///   the same leaned or not, so a groove within it parts its burr evenly.
pub fn groove(h_pa: f64, tip_um: f64, cone_deg: f64, force: f64, along_deg: f64, side_deg: f64) -> Groove {
    let cone = cone_deg.to_radians();
    let w = (8.0 * force.max(0.0) / (std::f64::consts::PI * h_pa)).sqrt() * 1e6;
    let a = 0.5 * w;
    let d = tip_profile(tip_um, cone, a);
    let area = (2.0 * (a * d - tip_integral(tip_um, cone, a))).max(0.0);
    let beta = 90.0 - cone_deg - along_deg;
    let cut = beta >= BETA_C_DEG;
    let burr = if cut { 0.0 } else { area };
    let (sl, sr) = burr_sides(tip_um, cone, a, side_deg);
    Groove { w, d, area, beta, cut, chip: if cut { 1.0 } else { 0.0 }, burr_l: burr * sl, burr_r: burr * sr, burr_h: BURR_HEIGHT_PER_DEPTH * d, tip: tip_um, cone }
}

/// The burr's shares (left, right) for a groove `a` µm half-wide under a
/// point of radius `tip` and half-angle `cone` (radians) leaned `side_deg`
/// to the right (`groove`).
fn burr_sides(tip: f64, cone: f64, a: f64, side_deg: f64) -> (f64, f64) {
    if a <= tip * cone.cos() {
        return (0.5, 0.5);
    }
    let lim = 89.0f64.to_radians();
    let s = side_deg.to_radians();
    let l = (cone + s).clamp(0.0, lim).tan();
    let r = (cone - s).clamp(0.0, lim).tan();
    if l + r <= 0.0 { (0.5, 0.5) } else { (l / (l + r), r / (l + r)) }
}

/// `groove` with its regime decided stream by stream (`Plate::contact`, the
/// easel's `--@ intaglio-contact`; notes/research/intaglio_physics.md §1.2).
///
/// β_c is measured on a plane face meeting the metal square on (Sedriks &
/// Mulhearn's pyramids, a face leading). A needle's face is a cone: each
/// stream of metal it meets, across the groove, meets the face at the face's
/// own slope along the way the needle goes, tan α = −n·x̂ / n·ẑ for the
/// face's outward normal n where that stream reaches the plate's surface
/// [D, geometry]. On an upright cone that is cot θ·cos φ: steepest at the
/// front (90° − θ), shallower toward the flanks, so an upright needle ploughs
/// unless its point is finer than 90° − β_c (drypoint manuals: held upright
/// it throws a burr on both sides [P]). The streams the face meets steeper
/// than β_c leave as a chip, the rest ploughs [D; the mixed mode of a
/// conical tool, Wang & Subhash 2002]; the shares are the groove's section
/// across those streams. Within the rounded point the face is the sphere's,
/// whose slope at the surface's edge is a·cos φ/(R − d). Leaning the needle
/// tilts the cone: pulled (`along` > 0) its front lies back toward the
/// plate, pushed or leaned to the side it stands up. `beta` is the steepest
/// angle the face meets the metal at.
pub fn groove_streams(h_pa: f64, tip_um: f64, cone_deg: f64, force: f64, along_deg: f64, side_deg: f64) -> Groove {
    groove_streams_at(h_pa, tip_um, cone_deg, force, along_deg, side_deg, cone_front(cone_deg, along_deg, side_deg))
}

/// The cone's face about its axis, the needle's top leaning `along` degrees
/// ahead and `side` to the right: its axis and two unit vectors square to it.
fn cone_frame(along_deg: f64, side_deg: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (ta, ts) = (along_deg.to_radians().tan(), side_deg.to_radians().tan());
    let m = (ta * ta + ts * ts + 1.0).sqrt();
    let ax = [ta / m, ts / m, 1.0 / m];
    let e1 = {
        let k = [ax[2], 0.0, -ax[0]];
        let l = (k[0] * k[0] + k[2] * k[2]).sqrt();
        [k[0] / l, 0.0, k[2] / l]
    };
    let e2 = [ax[1] * e1[2] - ax[2] * e1[1], ax[2] * e1[0] - ax[0] * e1[2], ax[0] * e1[1] - ax[1] * e1[0]];
    (ax, e1, e2)
}

/// The generatrix at φ round a cone of half-angle `cone` (radians) in its
/// frame (`cone_frame`), and its face's outward normal.
fn cone_face(frame: &([f64; 3], [f64; 3], [f64; 3]), cone: f64, phi: f64) -> ([f64; 3], [f64; 3]) {
    let (ax, e1, e2) = frame;
    let (c, s) = (phi.cos(), phi.sin());
    let gn: [f64; 3] = std::array::from_fn(|j| ax[j] * cone.cos() + cone.sin() * (e1[j] * c + e2[j] * s));
    let nrm: [f64; 3] = std::array::from_fn(|j| (gn[j] * cone.cos() - ax[j]) / cone.sin());
    (gn, nrm)
}

/// The steepest a cone's face meets the metal ahead (degrees): its attack
/// angle at its front, which a groove's width doesn't change.
fn cone_front(cone_deg: f64, along_deg: f64, side_deg: f64) -> f64 {
    let cone = cone_deg.to_radians();
    let frame = cone_frame(along_deg, side_deg);
    let alpha = |phi: f64| {
        let (gn, n) = cone_face(&frame, cone, phi);
        if gn[2] <= 1e-9 || n[0] <= 0.0 { f64::NEG_INFINITY } else { n[0].atan2(-n[2]).to_degrees() }
    };
    // coarsely round the point, then refined
    let coarse = 24;
    let step = std::f64::consts::TAU / coarse as f64;
    let best = (0..coarse).map(|i| i as f64 * step).max_by(|p, q| alpha(*p).total_cmp(&alpha(*q))).unwrap();
    let (mut lo, mut hi) = (best - step, best + step);
    for _ in 0..30 {
        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
        if alpha(m1) < alpha(m2) { lo = m1 } else { hi = m2 }
    }
    alpha(best).max(alpha(0.5 * (lo + hi)))
}

/// `groove_streams` given its cone's `cone_front` (a drypoint reuses it while
/// the needle's lean holds).
fn groove_streams_at(h_pa: f64, tip_um: f64, cone_deg: f64, force: f64, along_deg: f64, side_deg: f64, front: f64) -> Groove {
    let mut g = groove(h_pa, tip_um, cone_deg, force, along_deg, side_deg);
    let a = 0.5 * g.w;
    if a <= 0.0 || g.area <= 0.0 {
        return g;
    }
    let cone = cone_deg.to_radians();
    let sphere = a <= tip_um * cone.cos();
    let r_d = (tip_um - g.d).max(1e-9);
    let frame = cone_frame(along_deg, side_deg);
    // the virtual apex lies tip(1/sin θ − 1) under the rounded point
    let depth = g.d + tip_um * (1.0 / cone.sin() - 1.0);
    // the face where a stream at angle φ round the point meets the surface: its place
    // across the groove and its attack angle (degrees), if it meets the metal ahead
    let at = |phi: f64| -> Option<(f64, f64)> {
        if sphere {
            // the sphere: its edge at the surface, a across, R − d over its centre
            let c = phi.cos();
            return (c > 0.0).then(|| (a * phi.sin(), (a * c / r_d).atan().to_degrees()));
        }
        let (gn, nrm) = cone_face(&frame, cone, phi);
        // the face meeting the metal ahead: its normal points forward
        (gn[2] > 1e-9 && nrm[0] > 0.0).then(|| (gn[1] * depth / gn[2], nrm[0].atan2(-nrm[2]).to_degrees()))
    };
    // the steepest the face meets the metal: the sphere's front, or the cone's
    let beta = if sphere { (a / r_d).atan().to_degrees() } else { front };
    let (sl, sr) = burr_sides(tip_um, cone, a, side_deg);
    if !beta.is_finite() || beta <= BETA_C_DEG {
        // (no stream meets it steeper than β_c: it all ploughs)
        g.beta = if beta.is_finite() { beta } else { g.beta };
        (g.chip, g.cut) = (0.0, false);
        (g.burr_l, g.burr_r) = (g.area * sl, g.area * sr);
        return g;
    }
    // the streams, finely: each sample's place across and its attack angle
    let n = 360;
    let mut fr: Vec<(f64, f64)> = (0..n).filter_map(|i| at((i as f64 + 0.5) * std::f64::consts::TAU / n as f64)).collect();
    if fr.len() < 2 {
        return g;
    }
    fr.sort_by(|p, q| p.0.total_cmp(&q.0));
    let (lo, hi) = (fr[0].0, fr[fr.len() - 1].0);
    let span = (hi - lo).max(1e-9);
    let (mut all, mut cut) = (0.0f64, 0.0f64);
    for k in 0..fr.len() {
        // (this stream's width across the front, mapped onto the groove's width)
        let y0 = if k == 0 { lo } else { 0.5 * (fr[k - 1].0 + fr[k].0) };
        let y1 = if k + 1 == fr.len() { hi } else { 0.5 * (fr[k].0 + fr[k + 1].0) };
        let u = -a + ((0.5 * (y0 + y1)) - lo) / span * 2.0 * a;
        let wt = (y1 - y0).max(0.0) * g.depth_at(u);
        all += wt;
        if fr[k].1 > BETA_C_DEG {
            cut += wt;
        }
    }
    let chip = if all > 0.0 { (cut / all).clamp(0.0, 1.0) } else { 0.0 };
    // (the ploughed share parts to the sides as `groove` parts it)
    let burr = g.area * (1.0 - chip);
    g.beta = beta;
    g.chip = chip;
    g.cut = chip >= 0.5;
    (g.burr_l, g.burr_r) = (burr * sl, burr * sr);
    g
}

/// A path's azimuths (degrees) made to turn the short way between points, so
/// that a needle following its line round past due left (±180°) doesn't
/// swing back through 0° between two points (`Plate::contact`).
fn azimuths_unwound(pts: &[Point]) -> Vec<Point> {
    let mut out = pts.to_vec();
    for i in 1..out.len() {
        let prev = out[i - 1].az as f64;
        let d = (out[i].az as f64 - prev + 180.0).rem_euclid(360.0) - 180.0;
        out[i].az = (prev + d) as f32;
    }
    out
}

impl Groove {
    /// The depth (µm) of the groove `u` µm off its middle.
    pub fn depth_at(&self, u: f64) -> f64 {
        if u.abs() >= 0.5 * self.w { 0.0 } else { (self.d - tip_profile(self.tip, self.cone, u)).max(0.0) }
    }
}

/// The reduced modulus of a steel tool on a metal (Pa): 1/E* = (1 − ν₁²)/E₁
/// + (1 − ν₂²)/E₂ (Hertz).
pub fn reduced_modulus(metal: Metal) -> f64 {
    let (e, nu) = metal.elastic();
    1.0 / ((1.0 - STEEL_NU * STEEL_NU) / STEEL_E_PA + (1.0 - nu * nu) / e)
}

/// A burnisher's contact on the flat plate (Hertz, a sphere of radius
/// `r_m` pressed with `f` N; PP §3): its radius a (m), its peak pressure
/// p₀ = 3F/(2πa²) (Pa) and its approach δ = a²/R (m).
pub fn hertz(f: f64, r_m: f64, e_star: f64) -> (f64, f64, f64) {
    let a = (3.0 * f * r_m / (4.0 * e_star)).cbrt();
    (a, 3.0 * f / (2.0 * std::f64::consts::PI * a * a), a * a / r_m)
}

/// The pressure under which the metal at a groove's rim flows (Pa): a
/// wedge of included angle φ loaded on one face beside a free face yields at
/// p = 2k(1 + φ − π/2), the flat surface (φ = π) at Prandtl's 2k(1 + π/2),
/// which is the hardness (Hill 1950; PP §3). `wall` is the groove wall's
/// slope from the horizontal (radians).
pub fn rim_yield_pa(h_pa: f64, wall: f64) -> f64 {
    let half_pi = std::f64::consts::FRAC_PI_2;
    let phi = std::f64::consts::PI - wall;
    h_pa * (1.0 + phi - half_pi).max(0.0) / (1.0 + half_pi)
}

/// How deep (µm) a scraper's edge, pressed with `f` N along `width_mm`,
/// cuts in one stroke: the front face of a wedge edge of half-angle ψ
/// carries the load at the hardness, F = H·b·t·tan ψ (IP §1.3's scratch
/// law for a straight edge; PP §3).
pub fn scrape_depth_um(f: f64, width_mm: f64, h_pa: f64) -> f64 {
    let psi = (0.5 * SCRAPER_EDGE_DEG).to_radians();
    f / (h_pa * width_mm * 1e-3 * psi.tan()) * 1e6
}

/// A roulette tooth's pit (IP §1.7, [D]): a point of radius `tip_um` and
/// half-angle `cone_deg` pressed with `f` N indents at the hardness over its
/// whole projected area, πa² = F/H. Returns (a, depth, volume) in µm, µm³.
pub fn pit(h_pa: f64, tip_um: f64, cone_deg: f64, f: f64) -> (f64, f64, f64) {
    let cone = cone_deg.to_radians();
    let a = (f.max(0.0) / (std::f64::consts::PI * h_pa)).sqrt() * 1e6;
    let d = tip_profile(tip_um, cone, a);
    // V = 2π ∫₀ᵃ (d − profile(u)) u du (Simpson, 128 steps)
    let n = 128;
    let hstep = a / n as f64;
    let g = |u: f64| (d - tip_profile(tip_um, cone, u)).max(0.0) * u;
    let mut s = g(0.0) + g(a);
    for i in 1..n {
        s += if i % 2 == 1 { 4.0 } else { 2.0 } * g(i as f64 * hstep);
    }
    (a, d, 2.0 * std::f64::consts::PI * s * hstep / 3.0)
}

// ---------------------------------------------------------------- metal optics

/// Copper's optical constants (λ µm, n, k): Johnson & Christy, Phys. Rev. B
/// 6, 4370 (1972) [M], via refractiveindex.info (PP §4).
const CU_JC: [(f64, f64, f64); 21] = [
    (0.3315, 1.34, 1.821),
    (0.3425, 1.36, 1.864),
    (0.3542, 1.37, 1.916),
    (0.3679, 1.36, 1.975),
    (0.3815, 1.33, 2.045),
    (0.3974, 1.32, 2.116),
    (0.4133, 1.28, 2.207),
    (0.4305, 1.25, 2.305),
    (0.4509, 1.24, 2.397),
    (0.4714, 1.25, 2.483),
    (0.4959, 1.22, 2.564),
    (0.5209, 1.18, 2.608),
    (0.5486, 1.02, 2.577),
    (0.5821, 0.70, 2.704),
    (0.6168, 0.30, 3.205),
    (0.6595, 0.22, 3.747),
    (0.7045, 0.21, 4.205),
    (0.7560, 0.24, 4.665),
    (0.8211, 0.26, 5.180),
    (0.8920, 0.30, 5.768),
    (0.9840, 0.32, 6.400),
];

/// Zinc's (λ µm, n, k): Werner, Glantschnig & Ambrosch-Draxl, J. Phys.
/// Chem. Ref. Data 38, 1013 (2009), from reflection electron energy-loss
/// spectra [M], via refractiveindex.info (PP §4).
const ZN_WERNER: [(f64, f64, f64); 10] = [
    (0.330625, 0.5362, 2.7034),
    (0.354241, 0.5240, 2.9907),
    (0.381490, 0.5508, 3.2986),
    (0.413281, 0.6029, 3.6376),
    (0.450852, 0.6794, 4.0209),
    (0.495937, 0.7858, 4.4636),
    (0.551041, 0.9322, 4.9850),
    (0.619921, 1.1356, 5.6116),
    (0.708481, 1.4247, 6.3811),
    (0.826561, 1.8489, 7.3486),
];

/// A metal's (n, k) at `lambda_um`, linear between the tabulated points.
pub fn nk(metal: Metal, lambda_um: f64) -> (f64, f64) {
    let t: &[(f64, f64, f64)] = match metal {
        Metal::Copper => &CU_JC,
        Metal::Zinc => &ZN_WERNER,
    };
    let i = t.iter().position(|p| p.0 >= lambda_um).unwrap_or(t.len() - 1).max(1);
    let (a, b) = (t[i - 1], t[i]);
    let u = ((lambda_um - a.0) / (b.0 - a.0)).clamp(0.0, 1.0);
    (a.1 + (b.1 - a.1) * u, a.2 + (b.2 - a.2) * u)
}

/// Fresnel reflectance of unpolarized light from air onto a medium of
/// complex index n + ik at incidence `theta` (radians):
/// R = (|r_s|² + |r_p|²)/2, with cos θ_t = √(1 − sin²θ/N²).
pub fn fresnel(n: f64, k: f64, theta: f64) -> f64 {
    #[derive(Clone, Copy)]
    struct C(f64, f64);
    let add = |a: C, b: C| C(a.0 + b.0, a.1 + b.1);
    let sub = |a: C, b: C| C(a.0 - b.0, a.1 - b.1);
    let mul = |a: C, b: C| C(a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0);
    let div = |a: C, b: C| {
        let m = b.0 * b.0 + b.1 * b.1;
        C((a.0 * b.0 + a.1 * b.1) / m, (a.1 * b.0 - a.0 * b.1) / m)
    };
    let sqrt = |a: C| {
        let m = (a.0 * a.0 + a.1 * a.1).sqrt();
        let re = ((m + a.0) / 2.0).max(0.0).sqrt();
        let im = ((m - a.0) / 2.0).max(0.0).sqrt();
        C(re, if a.1 < 0.0 { -im } else { im })
    };
    let abs2 = |a: C| a.0 * a.0 + a.1 * a.1;
    let nn = C(n, k);
    let (ci, si) = (theta.cos(), theta.sin());
    let ct = sqrt(sub(C(1.0, 0.0), div(C(si * si, 0.0), mul(nn, nn))));
    let ci_c = C(ci, 0.0);
    let rs = div(sub(ci_c, mul(nn, ct)), add(ci_c, mul(nn, ct)));
    let rp = div(sub(mul(nn, ci_c), ct), add(mul(nn, ci_c), ct));
    0.5 * (abs2(rs) + abs2(rp))
}

/// A metal's reflectance spectrum at incidence `theta` (radians).
pub fn reflectance(metal: Metal, theta: f64) -> Spectrum {
    std::array::from_fn(|i| {
        let (n, k) = nk(metal, spectral::wavelength(i) as f64 / 1000.0);
        fresnel(n, k, theta) as f32
    })
}

/// What comes back out of a V-groove whose walls slope `wall` (radians)
/// from the horizontal, for light that goes in straight down: traced from
/// wall to wall in its section, each reflection taking the Fresnel share at
/// its incidence (PP §4). Averaged over where across the groove it enters.
pub fn groove_return(metal: Metal, wall: f64) -> Spectrum {
    let depth = wall.tan();
    let n = 32;
    let mut sum = [0.0f64; spectral::N];
    for j in 0..n {
        let x0 = -1.0 + (j as f64 + 0.5) * 2.0 / n as f64;
        let mut t = [1.0f64; spectral::N];
        // the ray at (x, y), going (dx, dy); the walls from (−1, 0) and (1, 0) to (0, −depth)
        let (mut x, mut y, mut dx, mut dy) = (x0, 0.0f64, 0.0f64, -1.0f64);
        let mut out = false;
        for _ in 0..64 {
            // each wall as a line through its rim, with its inward normal
            let mut best: Option<(f64, f64, f64)> = None;
            for sgn in [-1.0f64, 1.0] {
                let (px, py) = (sgn, 0.0);
                let (ex, ey) = (-sgn, -depth);
                let (nx, ny) = {
                    let (a, b) = (-ey, ex);
                    let m = (a * a + b * b).sqrt();
                    // pointing up out of the metal
                    if b / m > 0.0 { (a / m, b / m) } else { (-a / m, -b / m) }
                };
                let denom = dx * nx + dy * ny;
                if denom >= -1e-12 {
                    continue;
                }
                let s = ((px - x) * nx + (py - y) * ny) / denom;
                if s > 1e-9 && best.is_none_or(|b| s < b.0) {
                    best = Some((s, nx, ny));
                }
            }
            let Some((s, nx, ny)) = best else {
                out = dy > 0.0;
                break;
            };
            let (hx, hy) = (x + s * dx, y + s * dy);
            if hy > 0.0 {
                out = dy > 0.0;
                break;
            }
            let cos_i = -(dx * nx + dy * ny);
            let theta = cos_i.clamp(-1.0, 1.0).acos();
            for (i, v) in t.iter_mut().enumerate() {
                let (nn, kk) = nk(metal, spectral::wavelength(i) as f64 / 1000.0);
                *v *= fresnel(nn, kk, theta);
            }
            let dot = dx * nx + dy * ny;
            dx -= 2.0 * dot * nx;
            dy -= 2.0 * dot * ny;
            x = hx;
            y = hy;
            if dy > 0.0 && y >= -1e-12 {
                out = true;
                break;
            }
        }
        if out {
            for (s, v) in sum.iter_mut().zip(t) {
                *s += v;
            }
        }
    }
    std::array::from_fn(|i| (sum[i] / n as f64) as f32)
}

// ---------------------------------------------------------------- the plate

/// Openings are binned by width: bin 0 below 32 µm, bin k from 16·2ᵏ µm,
/// the last from 2048 µm up (open bite, later).
pub const BINS: usize = 8;
const TILE: usize = 64;

fn bin_of(w: f64) -> usize {
    if w < 32.0 { 0 } else { ((w / 16.0).log2().floor() as usize).min(BINS - 1) }
}

/// What one cell of the plate holds (plan §2.2). Shares are of the cell's
/// area; volumes are µm over the cell (µm³ per µm² of plate).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    /// Per width bin: the share of the cell open, the volume of the
    /// openings, and the sum of share × width (µm).
    pub open: [f32; BINS],
    pub vol: [f32; BINS],
    pub wsum: [f32; BINS],
    /// The burr's volume and its share of the cell.
    pub burr_vol: f32,
    pub burr_area: f32,
    /// The surface's own height (µm; a scraped dish is negative).
    pub level: f32,
    /// The line's direction, degrees over 0..180 (255: none), and which of
    /// its sides the burr stands on (bit 0 left, bit 1 right, looking along
    /// `dir`; both for a roulette's ring).
    pub dir: u8,
    pub side: u8,
}

impl Cell {
    const EMPTY: Cell = Cell { open: [0.0; BINS], vol: [0.0; BINS], wsum: [0.0; BINS], burr_vol: 0.0, burr_area: 0.0, level: 0.0, dir: 255, side: 0 };

    /// The openings: (share of the cell, width µm, depth µm) of each bin
    /// that holds any (what the wipe and the press read, phase P3).
    pub fn openings(&self) -> impl Iterator<Item = (f32, f32, f32)> + '_ {
        (0..BINS).filter(|&k| self.open[k] > 0.0 && self.vol[k] > 0.0).map(|k| (self.open[k], self.wsum[k] / self.open[k], self.vol[k] / self.open[k]))
    }

    pub fn open_share(&self) -> f32 {
        self.open.iter().sum()
    }

    pub fn groove_vol(&self) -> f32 {
        self.vol.iter().sum()
    }

    /// The burr's height (µm): a fin of triangular section over its share.
    pub fn burr_height(&self) -> f32 {
        if self.burr_area > 0.0 { 2.0 * self.burr_vol / self.burr_area } else { 0.0 }
    }

    /// The openings' depth and width, averaged by volume (µm).
    pub fn mean_depth_width(&self) -> (f32, f32) {
        let v = self.groove_vol();
        if v <= 0.0 {
            return (0.0, 0.0);
        }
        let (mut d, mut w) = (0.0, 0.0);
        for (s, wi, di) in self.openings() {
            d += di * s * di;
            w += wi * s * di;
        }
        (d / v, w / v)
    }

    /// The mean height of the cell's surface over the plate's flat (µm).
    pub fn height(&self) -> f32 {
        self.level + self.burr_vol - self.groove_vol()
    }

    fn add_open(&mut self, w: f64, share: f64, vol: f64) {
        let k = bin_of(w);
        self.open[k] += share as f32;
        self.vol[k] += vol as f32;
        self.wsum[k] += (share * w) as f32;
        // (crossing lines count the crossing twice: the shares add up to at
        // most the cell, keeping their volume)
        let total = self.open_share();
        if total > 1.0 {
            for k in 0..BINS {
                self.open[k] /= total;
                self.wsum[k] /= total;
            }
        }
    }

    /// Bin `k` to volume `v`, its section keeping its shape.
    fn shrink(&mut self, k: usize, v: f32) {
        if v <= 1e-6 || self.vol[k] <= 0.0 {
            self.open[k] = 0.0;
            self.vol[k] = 0.0;
            self.wsum[k] = 0.0;
            return;
        }
        let r = (v / self.vol[k]).min(1.0);
        let s = r.sqrt();
        self.open[k] *= s;
        self.wsum[k] *= r;
        self.vol[k] = v;
    }
}

/// A point of a tool's path: where (mm on the plate), with what force (N),
/// at what angles (degrees: alt to the plate, azimuth where the tool's upper
/// end points, 0 to the right and 90 down the plate, as for pastel) and how
/// fast (mm/s).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub force: f32,
    pub alt: f32,
    pub az: f32,
    pub speed: f32,
}

/// A tool's use, as it was given: the plate's vectors.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Drypoint { tip_um: f32, cone_deg: f32, pts: Vec<Point> },
    Burnish { radius_mm: f32, passes: u32, pts: Vec<Point> },
    Scrape { width_mm: f32, passes: u32, pts: Vec<Point> },
    Roulette { pitch_mm: f32, tooth_um: f32, cone_deg: f32, pts: Vec<Point> },
    // the etching tools (`etched`, the easel's `--@ etch`)
    /// A hard ground `um` thick rolled along `pts` with a roller
    /// `roller_mm` long.
    Ground { um: f32, roller_mm: f32, pts: Vec<Point> },
    /// An etching needle drawn through the ground.
    Needle { tip_um: f32, cone_deg: f32, pts: Vec<Point> },
    /// Stopping-out varnish brushed along `pts` (each point's force the
    /// hand's pressure, 0..1) with a round brush `brush_mm` across, loaded to
    /// `load` of its belly.
    StopOut { brush_mm: f32, load: f32, pts: Vec<Point> },
    /// Rosin dust from the box: grains per µm² by size bin, and the seed they
    /// are scattered from.
    Aquatint { per_um2: Vec<f32>, seed: u64 },
    /// The plate heated until its rosin melts on.
    Melt,
    /// A bite: each step's bite of a flat face (µm) and diffusion layer (µm).
    Bite { acid: crate::etch::Acid, doses: Vec<(f32, f32)> },
    /// Solvent: grounds, varnish and rosin off, all over (the plates of
    /// paintings begun before the solvent rag, and their saves).
    Clean,
    /// A rag dampened with solvent, a pad `pad_mm` across, rubbed along
    /// `pts` (each point's speed its own) `passes` times: coats off where it
    /// dissolves through them (`Plate::wash`).
    Wash { pad_mm: f32, passes: u32, pts: Vec<Point> },
}

impl Op {
    /// An etching tool's (`--@ etch`): written only in a plate's version 2.
    pub fn etching(&self) -> bool {
        !matches!(self, Op::Drypoint { .. } | Op::Burnish { .. } | Op::Scrape { .. } | Op::Roulette { .. })
    }
}

/// What a tool's use did (for the verb's report and its tests).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Report {
    /// The widest mark's width and depth and the tallest burr (µm), and the
    /// attack angle there (degrees).
    pub width_um: f64,
    pub depth_um: f64,
    pub burr_um: f64,
    pub beta_deg: f64,
    /// Some of it cut a chip; some of it ploughed.
    pub cut: bool,
    pub ploughed: bool,
    /// Volumes (µm³): of the openings made, of the burr thrown up, of the
    /// metal taken off (chips, scrapings), and of the burr flattened.
    pub groove_um3: f64,
    pub burr_um3: f64,
    pub chip_um3: f64,
    pub flattened_um3: f64,
    /// Path length (mm) and hand time (s).
    pub length_mm: f64,
    pub secs: f64,
    /// The burnisher's peak pressure on the flat and the scraper's depth a
    /// stroke (MPa, µm).
    pub pressure_mpa: f64,
    pub scrape_um: f64,
}

/// A point along a tool's path: (x, y) mm, its unit direction, its arc
/// length from the start (mm), the pose there, and the length it stands
/// for (mm).
type Sample = (f64, f64, (f64, f64), f64, Point, f64);

/// An intaglio plate (see the module's notes).
#[derive(Clone, Debug, PartialEq)]
pub struct Plate {
    pub metal: Metal,
    pub temper: Temper,
    pub facing: Option<Facing>,
    pub width_mm: f32,
    pub height_mm: f32,
    pub thickness_mm: f32,
    pub bevel_deg: f32,
    w: usize,
    h: usize,
    /// The cell pitch (µm).
    pitch: f64,
    tiles: Vec<Option<Box<[Cell]>>>,
    /// Every tool use, in order.
    pub ops: Vec<Op>,
    /// Metal taken off as chips and scrapings (µm³).
    pub chip_um3: f64,
    /// While rebuilt for a crop: the cells written (x0, y0, x1, y1).
    clip: Option<(usize, usize, usize, usize)>,
    /// Grounds, varnish, rosin and the acid's bites (`etched`): None until an
    /// etching tool touches the plate.
    coats: Option<Box<etched::Coats>>,
    /// Its ink (`inked`, the easel's `--@ press`): None until it is inked.
    ink: Option<Box<inked::Inking>>,
    /// The contact laws of the easel's `--@ intaglio-contact` (set when a plate is
    /// made under it, kept with the plate): a drypoint needle's regime stream
    /// by stream (`groove_streams`) along azimuths that turn the short way,
    /// and a wipe held off by a burr over the gap it leaves (`inked`). False:
    /// as the plate was first released.
    pub contact: bool,
}

impl Plate {
    /// A bare plate of `metal` in `temper`, `width_mm` × `height_mm`, laid out
    /// in `w` × `h` cells.
    pub fn new(metal: Metal, temper: Temper, width_mm: f32, height_mm: f32, thickness_mm: f32, bevel_deg: f32, w: usize, h: usize) -> Plate {
        let pitch = width_mm as f64 * 1000.0 / w as f64;
        let (tw, th) = (w.div_ceil(TILE), h.div_ceil(TILE));
        Plate { metal, temper, facing: None, width_mm, height_mm, thickness_mm, bevel_deg, w, h, pitch, tiles: vec![None; tw * th], ops: Vec::new(), chip_um3: 0.0, clip: None, coats: None, ink: None, contact: false }
    }

    pub fn size_px(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    /// The cell pitch (µm).
    pub fn pitch_um(&self) -> f64 {
        self.pitch
    }

    /// The surface's Vickers hardness: its facing's if it has one.
    pub fn vickers(&self) -> f64 {
        match self.facing {
            Some(f) => f.hv as f64,
            None => vickers(self.metal, self.temper).unwrap_or(45.0),
        }
    }

    pub fn hardness_pa(&self) -> f64 {
        hardness_pa(self.vickers())
    }

    pub fn cell(&self, x: usize, y: usize) -> Cell {
        if x >= self.w || y >= self.h {
            return Cell::EMPTY;
        }
        let tw = self.w.div_ceil(TILE);
        match &self.tiles[(y / TILE) * tw + x / TILE] {
            Some(t) => t[(y % TILE) * TILE + x % TILE],
            None => Cell::EMPTY,
        }
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.w || y >= self.h {
            return None;
        }
        if let Some((x0, y0, x1, y1)) = self.clip
            && !(x0..x1).contains(&x) | !(y0..y1).contains(&y)
        {
            return None;
        }
        let tw = self.w.div_ceil(TILE);
        let t = self.tiles[(y / TILE) * tw + x / TILE].get_or_insert_with(|| vec![Cell::EMPTY; TILE * TILE].into_boxed_slice());
        Some(&mut t[(y % TILE) * TILE + x % TILE])
    }

    /// The cell under a point (mm).
    fn at_mm(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let (cx, cy) = (x * 1000.0 / self.pitch, y * 1000.0 / self.pitch);
        (cx >= 0.0 && cy >= 0.0 && (cx as usize) < self.w && (cy as usize) < self.h).then_some((cx as usize, cy as usize))
    }

    /// The cell under a point (mm), to read.
    pub fn cell_at_mm(&self, x: f64, y: f64) -> Option<Cell> {
        self.at_mm(x, y).map(|(cx, cy)| self.cell(cx, cy))
    }

    /// Tiles in use (an untouched plate holds none).
    pub fn tiles_used(&self) -> usize {
        self.tiles.iter().filter(|t| t.is_some()).count()
    }

    /// The metal above the plate's original flat (µm³): burr and raised
    /// level less the openings and the lowered level.
    pub fn metal_um3(&self) -> f64 {
        let a = self.pitch * self.pitch;
        self.tiles.iter().flatten().flat_map(|t| t.iter()).map(|c| c.height() as f64 * a).sum()
    }

    /// The burr's volume on the plate (µm³) and its tallest (µm).
    pub fn burr_um3(&self) -> (f64, f64) {
        let a = self.pitch * self.pitch;
        let cells = self.tiles.iter().flatten().flat_map(|t| t.iter());
        cells.fold((0.0, 0.0), |(v, h), c| (v + c.burr_vol as f64 * a, f64::max(h, c.burr_height() as f64)))
    }

    /// The openings' volume on the plate (µm³).
    pub fn groove_um3(&self) -> f64 {
        let a = self.pitch * self.pitch;
        self.tiles.iter().flatten().flat_map(|t| t.iter()).map(|c| c.groove_vol() as f64 * a).sum()
    }

    // ------------------------------------------------------------ the tools

    /// Apply a tool's use and keep it among the plate's vectors.
    pub fn apply(&mut self, op: Op) -> Report {
        let r = self.run(&op);
        self.ops.push(op);
        r
    }

    fn run(&mut self, op: &Op) -> Report {
        match op {
            Op::Drypoint { tip_um, cone_deg, pts } => self.drypoint(*tip_um as f64, *cone_deg as f64, pts),
            Op::Burnish { radius_mm, passes, pts } => self.burnish(*radius_mm as f64, *passes, pts),
            Op::Scrape { width_mm, passes, pts } => self.scrape(*width_mm as f64, *passes, pts),
            Op::Roulette { pitch_mm, tooth_um, cone_deg, pts } => self.roulette(*pitch_mm as f64, *tooth_um as f64, *cone_deg as f64, pts),
            op => self.run_etch(op),
        }
    }

    /// The path sampled every `step` mm or less (`Sample`), and its length.
    fn samples(pts: &[Point], step: f64) -> (Vec<Sample>, f64) {
        let total: f64 = pts.windows(2).map(|p| ((p[1].x - p[0].x) as f64).hypot((p[1].y - p[0].y) as f64)).sum();
        if total <= 0.0 || step <= 0.0 {
            return (Vec::new(), total);
        }
        let n = (total / step).ceil().max(1.0) as usize;
        let ds = total / n as f64;
        Self::samples_at(pts, (0..n).map(|j| (j as f64 + 0.5) * ds), ds)
    }

    /// The path at arc lengths `at` (mm, rising), each sample `ds` long.
    fn samples_at(pts: &[Point], at: impl Iterator<Item = f64>, ds: f64) -> (Vec<Sample>, f64) {
        let seg: Vec<f64> = pts.windows(2).map(|p| ((p[1].x - p[0].x) as f64).hypot((p[1].y - p[0].y) as f64)).collect();
        let total: f64 = seg.iter().sum();
        let mut out = Vec::new();
        if seg.is_empty() {
            return (out, total);
        }
        let (mut k, mut start) = (0usize, 0.0f64);
        for s in at {
            while k + 1 < seg.len() && start + seg[k] < s {
                start += seg[k];
                k += 1;
            }
            let (a, b) = (pts[k], pts[k + 1]);
            let u = if seg[k] > 0.0 { ((s - start) / seg[k]).clamp(0.0, 1.0) } else { 0.0 };
            let lerp = |p: f32, q: f32| p as f64 + (q as f64 - p as f64) * u;
            let dir = if seg[k] > 0.0 { (((b.x - a.x) as f64) / seg[k], ((b.y - a.y) as f64) / seg[k]) } else { (1.0, 0.0) };
            let pose = Point { x: lerp(a.x, b.x) as f32, y: lerp(a.y, b.y) as f32, force: lerp(a.force, b.force) as f32, alt: lerp(a.alt, b.alt) as f32, az: lerp(a.az, b.az) as f32, speed: lerp(a.speed, b.speed) as f32 };
            out.push((lerp(a.x, b.x), lerp(a.y, b.y), dir, s, pose, ds));
        }
        (out, total)
    }

    /// The hand's force at arc length `s` of a stroke `total` mm long at
    /// `speed` mm/s: it rises over the landing and falls over the lift, as
    /// pastel's does (pastel.rs `LAND_S`, `LIFT_S`).
    fn hand(force: f64, s: f64, total: f64, speed: f64) -> f64 {
        let land = (crate::pastel::LAND_S as f64 * speed).max(1e-3) as f32;
        let lift = (crate::pastel::LIFT_S as f64 * speed).max(1e-3) as f32;
        force * (crate::smoothstep(0.0, land, s as f32) * crate::smoothstep(0.0, lift, (total - s) as f32)) as f64
    }

    /// Hand time of a path (s): its length at its speed, with the landing
    /// and the lift.
    fn secs(samples: &[Sample]) -> f64 {
        samples.iter().map(|s| s.5 / (s.4.speed as f64).max(1.0)).sum::<f64>() + (crate::pastel::LAND_S + crate::pastel::LIFT_S) as f64
    }

    /// Drypoint (IP §1.2–§1.4): a groove along the path, its burr beside it.
    fn drypoint(&mut self, tip_um: f64, cone_deg: f64, pts: &[Point]) -> Report {
        let h_pa = self.hardness_pa();
        let p = self.pitch;
        let unwound;
        let pts = if self.contact {
            unwound = azimuths_unwound(pts);
            &unwound[..]
        } else {
            pts
        };
        let (samples, total) = Self::samples(pts, p / 4000.0);
        let mut rep = Report { length_mm: total, secs: Self::secs(&samples), ..Report::default() };
        // (the cone's front for the last lean, to a twentieth of a degree: it holds
        // along a stretch, and moves β by less than that)
        let mut front: Option<((i64, i64), f64)> = None;
        for &(x, y, (tx, ty), s, pose, ds) in &samples {
            let f = Self::hand(pose.force as f64, s, total, pose.speed as f64);
            // the needle's lean: along the way it goes (pulled: its top ahead) and to its right
            let lean = (90.0 - (pose.alt as f64).clamp(0.0, 90.0)).to_radians().tan();
            let (ax, ay) = ((pose.az as f64).to_radians().cos(), (pose.az as f64).to_radians().sin());
            let (rx, ry) = (-ty, tx);
            let along = (lean * (ax * tx + ay * ty)).atan().to_degrees();
            let side = (lean * (ax * rx + ay * ry)).atan().to_degrees();
            let g = if self.contact {
                let key = ((along * 20.0).round() as i64, (side * 20.0).round() as i64);
                let fr = match front {
                    Some((k, v)) if k == key => v,
                    _ => {
                        let v = cone_front(cone_deg, along, side);
                        front = Some((key, v));
                        v
                    }
                };
                groove_streams_at(h_pa, tip_um, cone_deg, f, along, side, fr)
            } else {
                groove(h_pa, tip_um, cone_deg, f, along, side)
            };
            if g.w <= 0.0 || g.area <= 0.0 {
                continue;
            }
            if g.w > rep.width_um {
                (rep.width_um, rep.depth_um, rep.burr_um, rep.beta_deg) = (g.w, g.d, if g.chip >= 1.0 { 0.0 } else { g.burr_h }, g.beta);
            }
            rep.cut |= g.chip > 0.0;
            rep.ploughed |= g.chip < 1.0;
            let len_um = ds * 1000.0;
            rep.groove_um3 += g.area * len_um;
            rep.chip_um3 += g.chip * g.area * len_um;
            self.chip_um3 += g.chip * g.area * len_um;
            rep.burr_um3 += (g.burr_l + g.burr_r) * len_um;
            // the line's direction over 0..180, and its sides as seen along it
            let ang = ty.atan2(tx).to_degrees().rem_euclid(360.0);
            let flip = ang >= 180.0;
            let dir = (ang % 180.0).round() as u8 % 180;
            self.deposit(x, y, (rx, ry), len_um, &g, dir, flip);
        }
        rep
    }

    /// One sample of a groove `len_um` long at (x, y) mm, across `r` (its
    /// right): the opening by its section, the burr on its sides, each
    /// sampled across and scaled to its exact section, so the burr's volume
    /// is the groove's to rounding.
    #[allow(clippy::too_many_arguments)]
    fn deposit(&mut self, x: f64, y: f64, r: (f64, f64), len_um: f64, g: &Groove, dir: u8, flip: bool) {
        let p = self.pitch;
        let cell_area = p * p;
        let a = 0.5 * g.w;
        let n = ((2.0 * a / (p / 4.0)).ceil() as usize).max(4);
        let du = 2.0 * a / n as f64;
        let us: Vec<f64> = (0..n).map(|i| -a + (i as f64 + 0.5) * du).collect();
        let wts: Vec<f64> = us.iter().map(|&u| g.depth_at(u)).collect();
        let sum: f64 = wts.iter().sum();
        if sum <= 0.0 {
            return;
        }
        for (u, wt) in us.iter().zip(&wts) {
            let (px, py) = (x + u * r.0 / 1000.0, y + u * r.1 / 1000.0);
            if let Some((cx, cy)) = self.at_mm(px, py)
                && let Some(c) = self.cell_mut(cx, cy)
            {
                let share = du * len_um / cell_area;
                c.add_open(g.w, share, g.area * wt / sum * len_um / cell_area);
                c.dir = dir;
            }
        }
        for (sgn, section) in [(-1.0f64, g.burr_l), (1.0, g.burr_r)] {
            if section <= 0.0 || g.burr_h <= 0.0 {
                continue;
            }
            let base = 2.0 * section / g.burr_h;
            let nb = ((base / (p / 4.0)).ceil() as usize).max(3);
            let dv = base / nb as f64;
            let vs: Vec<f64> = (0..nb).map(|i| (i as f64 + 0.5) * dv).collect();
            // Under the contact laws each sample of the fin's base carries the same metal, h/2 of
            // its width: a cell's piece, read back as a ridge of height 2·vol/area
            // (`Cell::burr_height`), is then the fin's own height wherever the cell's edges cut
            // its base. (Weighted by the fin's height at each sample, a cell holding only its
            // inner part read up to 1.5 h and one holding its toe less, so the height of a
            // slanting line's burr stepped with the line's place in its cells.) The metal is
            // the same; its centre lies base/6 further out, a few µm.
            let hs: Vec<f64> = vs.iter().map(|v| if self.contact { 1.0 } else { 1.0 - v / base }).collect();
            let hsum: f64 = hs.iter().sum();
            // bit 0: the left of the line as `dir` runs (the needle went the other way: its right)
            let bit = if (sgn < 0.0) != flip { 1u8 } else { 2u8 };
            for (v, hv) in vs.iter().zip(&hs) {
                let off = sgn * (a + v);
                let (px, py) = (x + off * r.0 / 1000.0, y + off * r.1 / 1000.0);
                if self.contact {
                    // the sample's piece of base (dv across, len along) laid over the cells it
                    // covers by area, a square of its area: a cell's share is then the base's
                    // own, not every piece whose middle falls in it (which read a cell up to a
                    // quarter over full, and its burr that much too tall); a full cell takes no
                    // more, and none of the metal of what it can't take
                    let q = (dv * len_um).sqrt();
                    let (ux, uy) = (px * 1000.0 / p, py * 1000.0 / p);
                    let hw = 0.5 * q / p;
                    let (x0, x1, y0, y1) = (ux - hw, ux + hw, uy - hw, uy + hw);
                    let whole = dv * len_um / cell_area;
                    let vol = section * hv / hsum * len_um / cell_area;
                    for cy in (y0.floor() as i64)..=(y1.floor() as i64) {
                        for cx in (x0.floor() as i64)..=(x1.floor() as i64) {
                            let f = ((x1.min(cx as f64 + 1.0) - x0.max(cx as f64)).max(0.0) * (y1.min(cy as f64 + 1.0) - y0.max(cy as f64)).max(0.0)) / (4.0 * hw * hw);
                            if f <= 0.0 || cx < 0 || cy < 0 {
                                continue;
                            }
                            if let Some(c) = self.cell_mut(cx as usize, cy as usize) {
                                let want = whole * f;
                                let took = want.min((1.0 - c.burr_area as f64).max(0.0));
                                if took <= 0.0 {
                                    continue;
                                }
                                c.burr_area += took as f32;
                                c.burr_vol += (vol * f * took / want) as f32;
                                c.dir = dir;
                                c.side |= bit;
                            }
                        }
                    }
                    continue;
                }
                if let Some((cx, cy)) = self.at_mm(px, py)
                    && let Some(c) = self.cell_mut(cx, cy)
                {
                    c.burr_area = (c.burr_area + (dv * len_um / cell_area) as f32).min(1.0);
                    c.burr_vol += (section * hv / hsum * len_um / cell_area) as f32;
                    c.dir = dir;
                    c.side |= bit;
                }
            }
        }
    }

    /// The cells within `reach` µm of the sampled path, each with its
    /// distance to the path (µm) and the sample nearest it.
    fn band(&self, samples: &[Sample], reach: impl Fn(usize, usize) -> f64, max_reach: f64) -> Vec<(usize, usize, f64, usize)> {
        let p = self.pitch;
        let mut near: std::collections::BTreeMap<(usize, usize), (f64, usize)> = std::collections::BTreeMap::new();
        let span = (max_reach / p).ceil() as i64 + 1;
        for (i, &(x, y, ..)) in samples.iter().enumerate() {
            let (cx, cy) = ((x * 1000.0 / p).floor() as i64, (y * 1000.0 / p).floor() as i64);
            for yy in (cy - span).max(0)..=(cy + span).min(self.h as i64 - 1) {
                for xx in (cx - span).max(0)..=(cx + span).min(self.w as i64 - 1) {
                    let (mx, my) = ((xx as f64 + 0.5) * p, (yy as f64 + 0.5) * p);
                    let d = (mx - x * 1000.0).hypot(my - y * 1000.0);
                    let e = near.entry((xx as usize, yy as usize)).or_insert((f64::INFINITY, i));
                    if d < e.0 {
                        *e = (d, i);
                    }
                }
            }
        }
        near.into_iter().filter(|((x, y), (d, _))| *d <= reach(*x, *y)).map(|((x, y), (d, i))| (x, y, d, i)).collect()
    }

    /// The burnisher (IP §1.7; PP §3). Each pass, at each cell within its
    /// reach:
    /// - **Burr first.** A sphere of radius R on ridges of triangular section
    ///   that cover a share s of the plate, flowing at the hardness, sinks to
    ///   z₀ = h − √(F·h/(π·H·s·R)) under F (the bearing of the ridges within
    ///   its sag). The burr above z₀ (and above its sag r²/2R off the path)
    ///   is pressed flat. The fin leans away from its line (IP §1.4), so it
    ///   folds over onto the land beside it, raising it by its metal.
    ///   A burr left lower than the contact's elastic approach (`hertz`) no
    ///   longer holds the burnisher off the land, and goes flat.
    /// - **Then the rims.** Where no burr holds it off, the contact is
    ///   Hertz's on the flat: where its pressure exceeds what a groove's rim
    ///   bears (`rim_yield_pa`), the rim flows into the groove by the
    ///   contact's elastic approach there, from the groove's depth on each
    ///   side, and the land beside it comes down by the metal it gave.
    fn burnish(&mut self, radius_mm: f64, passes: u32, pts: &[Point]) -> Report {
        let h_pa = self.hardness_pa();
        let e_star = reduced_modulus(self.metal);
        let p = self.pitch;
        let r_um = radius_mm * 1000.0;
        let (samples, total) = Self::samples(pts, p / 2000.0);
        let mut rep = Report { length_mm: total * passes as f64, secs: Self::secs(&samples) * passes as f64, ..Report::default() };
        let before = self.metal_um3();
        let (b0, _) = self.burr_um3();
        for _ in 0..passes {
            let fmax = samples.iter().map(|s| s.4.force as f64).fold(0.0, f64::max);
            let (amax, _, _) = hertz(fmax.max(1e-9), r_um * 1e-6, e_star);
            let hmax = self.burr_um3().1;
            let reach_of = |c: &Cell, a_um: f64| a_um.max((2.0 * r_um * c.burr_height() as f64).sqrt());
            let max_reach = (amax * 1e6).max((2.0 * r_um * hmax).sqrt());
            let snap = self.clone();
            let band = self.band(&samples, |x, y| reach_of(&snap.cell(x, y), amax * 1e6), max_reach);
            for (x, y, r, i) in band {
                let f = samples[i].4.force as f64;
                if f <= 0.0 {
                    continue;
                }
                let (a, p0, delta) = hertz(f, r_um * 1e-6, e_star);
                let (a_um, delta_um) = (a * 1e6, delta * 1e6);
                rep.pressure_mpa = rep.pressure_mpa.max(p0 * 1e-6);
                let c0 = snap.cell(x, y);
                let mut c = c0;
                let mut rides = false;
                if c.burr_area > 0.0 && c.burr_vol > 0.0 {
                    let h = c.burr_height() as f64;
                    // the ridges' share and height over the burnisher's reach into them
                    let rho = (2.0 * r_um * h).sqrt();
                    let k = (rho / p).ceil() as i64;
                    let (mut area, mut vol, mut n) = (0.0f64, 0.0f64, 0usize);
                    for yy in (y as i64 - k).max(0)..=(y as i64 + k).min(self.h as i64 - 1) {
                        for xx in (x as i64 - k).max(0)..=(x as i64 + k).min(self.w as i64 - 1) {
                            if (((xx - x as i64) as f64) * p).hypot(((yy - y as i64) as f64) * p) <= rho.max(0.5 * p) {
                                let o = snap.cell(xx as usize, yy as usize);
                                area += o.burr_area as f64;
                                vol += o.burr_vol as f64;
                                n += 1;
                            }
                        }
                    }
                    let s = area / n.max(1) as f64;
                    let hb = if area > 0.0 { 2.0 * vol / area } else { h };
                    let sink = (f * hb * 1e-6 / (std::f64::consts::PI * h_pa * s.max(1e-12) * r_um * 1e-6)).sqrt() * 1e6;
                    let z0 = (hb - sink).max(0.0) + r * r / (2.0 * r_um);
                    // a burr lower than the contact's elastic approach there can't hold it
                    // off the flat: it is pressed flat with the land under it
                    let reach = if r < a_um { delta_um - r * r / (2.0 * r_um) } else { 0.0 };
                    if z0 < h {
                        let keep = if z0 <= reach { 0.0 } else { (z0 / h).clamp(0.0, 1.0) as f32 };
                        let flat = c.burr_vol * (1.0 - keep);
                        c.burr_vol -= flat;
                        if keep <= 0.0 {
                            c.burr_vol = 0.0;
                            c.burr_area = 0.0;
                            c.side = 0;
                        }
                        // (the fin leans away from its line, IP §1.4: pressed flat, it folds
                        // over onto the land beside the line, which it raises by its metal)
                        c.level += flat;
                        rep.flattened_um3 += flat as f64 * p * p;
                    }
                    rides = c.burr_height() > 0.0;
                }
                if !rides && r < a_um {
                    let pr = p0 * (1.0 - (r / a_um).powi(2)).max(0.0).sqrt();
                    let dz = delta_um - r * r / (2.0 * r_um);
                    if dz > 0.0 {
                        for k in 0..BINS {
                            if c.vol[k] <= 0.0 || c.open[k] <= 0.0 {
                                continue;
                            }
                            // (a V of section w·d/2: its full depth twice its mean)
                            let (w, d) = ((c.wsum[k] / c.open[k]) as f64, 2.0 * (c.vol[k] / c.open[k]) as f64);
                            let wall = (2.0 * d / w.max(1e-9)).atan();
                            if pr > rim_yield_pa(h_pa, wall) {
                                let dv = (2.0 * dz * d * c.open[k] as f64 / w.max(1e-9)).min(c.vol[k] as f64) as f32;
                                c.shrink(k, c.vol[k] - dv);
                                c.level -= dv;
                            }
                        }
                    }
                }
                if c != c0
                    && let Some(m) = self.cell_mut(x, y)
                {
                    *m = c;
                }
            }
        }
        rep.burr_um3 = self.burr_um3().0 - b0;
        rep.groove_um3 = 0.0;
        debug_assert!((self.metal_um3() - before).abs() <= 1e-3 * before.abs().max(self.groove_um3()).max(1.0));
        rep
    }

    /// The scraper (IP §1.7; PP §3): a blade `width_mm` wide drawn across
    /// the plate, held to cut. It takes the burr off, then shaves the plate
    /// down by `scrape_depth_um` a stroke, the lines with it; what it takes
    /// is counted as scrapings.
    fn scrape(&mut self, width_mm: f64, passes: u32, pts: &[Point]) -> Report {
        let h_pa = self.hardness_pa();
        let p = self.pitch;
        let (samples, total) = Self::samples(pts, p / 2000.0);
        let mut rep = Report { length_mm: total * passes as f64, secs: Self::secs(&samples) * passes as f64, ..Report::default() };
        let half = width_mm * 500.0;
        for _ in 0..passes {
            let band = self.band(&samples, |_, _| half, half);
            for (x, y, _, i) in band {
                let t = scrape_depth_um(samples[i].4.force as f64, width_mm, h_pa);
                rep.scrape_um = rep.scrape_um.max(t);
                let Some(c) = self.cell_mut(x, y) else { continue };
                let mut chip = c.burr_vol as f64;
                c.burr_vol = 0.0;
                c.burr_area = 0.0;
                c.side = 0;
                let mut air = 0.0f64;
                for k in 0..BINS {
                    if c.vol[k] <= 0.0 || c.open[k] <= 0.0 {
                        continue;
                    }
                    let d = (c.vol[k] / c.open[k]) as f64;
                    let v0 = c.vol[k];
                    if d <= t {
                        c.shrink(k, 0.0);
                    } else {
                        let s = ((d - t) / d) as f32;
                        c.shrink(k, v0 * s * s);
                    }
                    air += (v0 - c.vol[k]) as f64;
                }
                chip += t - air;
                c.level -= t as f32;
                let um3 = chip * p * p;
                rep.chip_um3 += um3;
                self.chip_um3 += um3;
            }
        }
        rep
    }

    /// The roulette (IP §1.7, [D]): a wheel of teeth `pitch_mm` apart rolled
    /// along the path, one tooth bearing at a time with the force. Each
    /// indents a pit at the hardness and throws up a ring of burr of the
    /// pit's volume, as tall over its depth as a drypoint burr.
    fn roulette(&mut self, pitch_mm: f64, tooth_um: f64, cone_deg: f64, pts: &[Point]) -> Report {
        let h_pa = self.hardness_pa();
        let p = self.pitch;
        // (a tooth every `pitch_mm` along the path, the first half a pitch in)
        let total: f64 = pts.windows(2).map(|q| ((q[1].x - q[0].x) as f64).hypot((q[1].y - q[0].y) as f64)).sum();
        let teeth = if pitch_mm > 0.0 { (total / pitch_mm + 0.5).floor() as usize } else { 0 };
        let (samples, _) = Self::samples_at(pts, (0..teeth).map(|j| (j as f64 + 0.5) * pitch_mm), pitch_mm);
        let mut rep = Report { length_mm: total, secs: Self::secs(&samples), ploughed: true, ..Report::default() };
        for &(x, y, _, s, pose, _) in &samples {
            let f = Self::hand(pose.force as f64, s, total, pose.speed as f64);
            let (a, d, v) = pit(h_pa, tooth_um, cone_deg, f);
            if v <= 0.0 {
                continue;
            }
            if 2.0 * a > rep.width_um {
                (rep.width_um, rep.depth_um) = (2.0 * a, d);
            }
            let hb = BURR_HEIGHT_PER_DEPTH * d;
            rep.burr_um = rep.burr_um.max(hb);
            rep.groove_um3 += v;
            rep.burr_um3 += v;
            // the ring's base b: V = 2π(a + b/3)·(h·b/2), a triangle section around the pit
            let (qa, qb, qc) = (std::f64::consts::PI * hb / 3.0, std::f64::consts::PI * a * hb, -v);
            let b = (-qb + (qb * qb - 4.0 * qa * qc).sqrt()) / (2.0 * qa);
            let step = (p / 4.0).min(a / 3.0).max(1e-3);
            let reach = a + b;
            let n = (reach / step).ceil() as i64;
            let (mut pits, mut ring) = (Vec::new(), Vec::new());
            for j in -n..=n {
                for i in -n..=n {
                    let (u, w) = ((i as f64) * step, (j as f64) * step);
                    let rr = u.hypot(w);
                    if rr < a {
                        pits.push((u, w, (d - tip_profile(tooth_um, cone_deg.to_radians(), rr)).max(0.0)));
                    } else if rr < reach {
                        ring.push((u, w, 1.0 - (rr - a) / b));
                    }
                }
            }
            let ca = p * p;
            let cell_share = step * step / ca;
            for (list, is_pit) in [(pits, true), (ring, false)] {
                let sum: f64 = list.iter().map(|q| q.2).sum();
                if sum <= 0.0 {
                    continue;
                }
                for (u, w, wt) in list {
                    if let Some((cx, cy)) = self.at_mm(x + u / 1000.0, y + w / 1000.0)
                        && let Some(c) = self.cell_mut(cx, cy)
                    {
                        if is_pit {
                            c.add_open(2.0 * a, cell_share, v * wt / sum / ca);
                        } else {
                            c.burr_area = (c.burr_area + cell_share as f32).min(1.0);
                            c.burr_vol += (v * wt / sum / ca) as f32;
                            c.side = 3;
                        }
                    }
                }
            }
        }
        rep
    }

    // ------------------------------------------------------------ looking

    /// The plate again at `factor` times its pitch, its tool uses rasterized
    /// at that scale, only the cells in `window` (cells of the finer
    /// plate) written: a crop's line shapes, from the vectors.
    pub fn rebuilt(&self, factor: usize, window: (usize, usize, usize, usize)) -> Plate {
        let mut p = Plate::new(self.metal, self.temper, self.width_mm, self.height_mm, self.thickness_mm, self.bevel_deg, self.w * factor, self.h * factor);
        p.facing = self.facing;
        p.contact = self.contact;
        p.clip = Some(window);
        for op in &self.ops {
            p.run(op);
        }
        p.ops = self.ops.clone();
        p.clip = None;
        self.ink_onto(&mut p, factor, window);
        p
    }

    /// The plate's colour in a uniform room light (linear RGB) over cells
    /// `rect` (x0, y0, x1, y1), and its surface (µm), row by row: flat metal
    /// at normal incidence, the openings by what their walls send back, the
    /// burr by its faces' reflection.
    pub fn colour_and_height(&self, rect: (usize, usize, usize, usize)) -> (Vec<Rgb>, Vec<f32>) {
        let flat = spectral::to_rgb(&reflectance(self.metal, 0.0));
        let deg: Vec<Rgb> = (0..=90).map(|d| spectral::to_rgb(&reflectance(self.metal, (d as f64).to_radians()))).collect();
        let walls: Vec<Rgb> = (0..=90).map(|d| spectral::to_rgb(&groove_return(self.metal, (d.clamp(1, 89) as f64).to_radians()))).collect();
        let (x0, y0, x1, y1) = rect;
        let mut col = Vec::with_capacity((x1 - x0) * (y1 - y0));
        let mut hgt = Vec::with_capacity(col.capacity());
        let coats = self.coats.as_deref().map(|c| etched::Looks::new(self, c));
        for y in y0..y1 {
            for x in x0..x1 {
                let c = self.cell(x, y);
                // ink on it (`inked`, `--@ press`): the metal under its films
                if self.ink.is_some() {
                    let tilt = if c.burr_area > 0.0 { (2.0 * c.burr_height() / (c.burr_area * self.pitch as f32).max(1e-6)).atan().to_degrees().round() as usize } else { 0 };
                    if let Some(px) = self.inked_rgb(x, y, &c, flat, &walls, deg[tilt.min(90)]) {
                        col.push(px);
                        hgt.push(c.height());
                        continue;
                    }
                }
                if let Some(l) = &coats
                    && let Some(px) = l.cell(self, x, y, &c, &flat, &walls)
                {
                    col.push(px);
                    hgt.push(c.height());
                    continue;
                }
                let open = c.open_share().min(1.0);
                let burr = c.burr_area.min(1.0 - open);
                let mut px = flat.map(|v| v * (1.0 - open - burr));
                for (s, w, d) in c.openings() {
                    // (a V of mean depth d: its walls slope 4d/w)
                    let wall = ((4.0 * d / w.max(1e-6)) as f64).atan().to_degrees().round() as usize;
                    let t = walls[wall.min(90)];
                    for q in 0..3 {
                        px[q] += s * t[q];
                    }
                }
                if burr > 0.0 {
                    // a fin of the burr's height on its share: its faces' slope
                    // (its base: its share of a cell a line crosses, the share times the pitch)
                    let tilt = (2.0 * c.burr_height() / (c.burr_area * self.pitch as f32).max(1e-6)).atan().to_degrees().round() as usize;
                    let t = deg[tilt.min(90)];
                    for q in 0..3 {
                        px[q] += burr * t[q];
                    }
                }
                if let Some(l) = &coats {
                    px = l.rosin(self, x, y, px);
                }
                col.push(px);
                hgt.push(c.height());
            }
        }
        (col, hgt)
    }

    /// The plate as it is seen over cells `rect`: its colour in a uniform
    /// room light, or lit by a lamp at (azimuth, elevation) degrees on its
    /// relief by the canvas's own light (`Canvas::seen_lit`).
    pub fn seen(&self, rect: (usize, usize, usize, usize), light: Option<(f32, f32)>) -> Vec<Rgb> {
        let (col, hgt) = self.colour_and_height(rect);
        match light {
            None => col,
            Some((az, el)) => {
                let (w, h) = (rect.2 - rect.0, rect.3 - rect.1);
                crate::prepared_optics::PreparedLighting::new(&hgt, w, h, self.pitch as f32).lit(&col, &vec![0.0; w * h], az, el, 1.0)
            }
        }
    }

    /// The colour of the bare metal at normal incidence (linear RGB).
    pub fn bare_rgb(metal: Metal) -> Rgb {
        spectral::to_rgb(&reflectance(metal, 0.0))
    }

    // ------------------------------------------------------------ persistence

    /// The plate's state as its surface entry's payload (`PLAT`): the metal,
    /// its size and cells, its facing, its tool uses and what they took off,
    /// then the tiles in use with their cells. Version 1 is that; version 2
    /// (`--@ etch`) is a plate an etching tool has touched: its etching tools
    /// among its uses, each with its own data after its points (a length and
    /// the bytes), and after the tiles its coats (`etched`: the ground, the
    /// rosin, the tray, the bath the plate lies in if it does, the metal the
    /// acid took, and the etched cells). A
    /// plate no etching tool has touched is written as version 1, byte for
    /// byte as before. Version 3 (`--@ press`) is a plate that has been
    /// inked: as version 2, but its coats (version 2's, the bath slot among
    /// them) only if it has any (a word, 0 or 1, before them), then its ink
    /// (`inked`). A plate never inked is written
    /// as version 1 or 2 as before. Version 4 (`--@ intaglio-contact`) is a plate
    /// made under the contact laws (`contact`): a word of its laws (bit 0,
    /// `contact`) and the version it would otherwise be written as, then that
    /// version's layout. A plate made without them is written as before.
    /// Bit 1 of the laws word marks a plate a solvent rag has left a coat
    /// thinned on or a stripper has worked on (`thin`): its etched cells carry
    /// what the rag took of each coat and its ink cells what the stripper has
    /// loosened and where it lies; a plate with neither is written as before,
    /// and one read without the bit has whole coats and no stripper.
    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        let v3 = self.ink.is_some();
        let v2 = v3 || self.coats.is_some() || self.ops.iter().any(Op::etching);
        let base = if v3 { 3 } else if v2 { 2 } else { 1 };
        let thin = self.thin();
        w.write_all(PLAT_MAGIC)?;
        if self.contact || thin {
            put64(w, PLAT_V)?;
            put64(w, self.contact as u64 | (thin as u64) << 1)?;
        }
        put64(w, base)?;
        put64(w, self.metal.word())?;
        put64(w, self.temper.word())?;
        for v in [self.width_mm, self.height_mm, self.thickness_mm, self.bevel_deg] {
            put32(w, v.to_bits())?;
        }
        put64(w, self.w as u64)?;
        put64(w, self.h as u64)?;
        put64(w, self.pitch.to_bits())?;
        put64(w, self.chip_um3.to_bits())?;
        match self.facing {
            None => put64(w, 0)?,
            Some(f) => {
                put64(w, 1)?;
                put32(w, f.hv.to_bits())?;
                put32(w, f.um.to_bits())?;
            }
        }
        put64(w, self.ops.len() as u64)?;
        for op in &self.ops {
            let none: &[Point] = &[];
            let mut blob = Vec::new();
            let (tag, a, b, c, pts) = match op {
                Op::Drypoint { tip_um, cone_deg, pts } => (0u64, *tip_um, *cone_deg, 0.0, pts.as_slice()),
                Op::Burnish { radius_mm, passes, pts } => (1, *radius_mm, *passes as f32, 0.0, pts.as_slice()),
                Op::Scrape { width_mm, passes, pts } => (2, *width_mm, *passes as f32, 0.0, pts.as_slice()),
                Op::Roulette { pitch_mm, tooth_um, cone_deg, pts } => (3, *pitch_mm, *tooth_um, *cone_deg, pts.as_slice()),
                Op::Ground { um, roller_mm, pts } => (4, *um, *roller_mm, 0.0, pts.as_slice()),
                Op::Needle { tip_um, cone_deg, pts } => (5, *tip_um, *cone_deg, 0.0, pts.as_slice()),
                Op::StopOut { brush_mm, load, pts } => (6, *brush_mm, *load, 0.0, pts.as_slice()),
                Op::Aquatint { per_um2, seed } => {
                    blob.extend_from_slice(&seed.to_le_bytes());
                    for v in per_um2 {
                        blob.extend_from_slice(&v.to_bits().to_le_bytes());
                    }
                    (7, 0.0, 0.0, 0.0, none)
                }
                Op::Melt => (8, 0.0, 0.0, 0.0, none),
                Op::Bite { acid, doses } => {
                    blob.extend_from_slice(&acid.word().to_le_bytes());
                    for (d, delta) in doses {
                        blob.extend_from_slice(&d.to_bits().to_le_bytes());
                        blob.extend_from_slice(&delta.to_bits().to_le_bytes());
                    }
                    (9, 0.0, 0.0, 0.0, none)
                }
                Op::Clean => (10, 0.0, 0.0, 0.0, none),
                Op::Wash { pad_mm, passes, pts } => (11, *pad_mm, *passes as f32, 0.0, pts.as_slice()),
            };
            put64(w, tag)?;
            for v in [a, b, c] {
                put32(w, v.to_bits())?;
            }
            put64(w, pts.len() as u64)?;
            for q in pts {
                for v in [q.x, q.y, q.force, q.alt, q.az, q.speed] {
                    put32(w, v.to_bits())?;
                }
            }
            if op.etching() {
                put64(w, blob.len() as u64)?;
                w.write_all(&blob)?;
            }
        }
        let used: Vec<usize> = (0..self.tiles.len()).filter(|&i| self.tiles[i].is_some()).collect();
        put64(w, used.len() as u64)?;
        let mut buf = Vec::with_capacity(TILE * TILE * 4 * (3 * BINS + 4));
        for i in used {
            put64(w, i as u64)?;
            buf.clear();
            for c in self.tiles[i].as_ref().unwrap().iter() {
                for v in c.open.iter().chain(&c.vol).chain(&c.wsum).chain([&c.burr_vol, &c.burr_area, &c.level]) {
                    buf.extend_from_slice(&v.to_bits().to_le_bytes());
                }
                buf.extend_from_slice(&[c.dir, c.side]);
            }
            w.write_all(&buf)?;
        }
        if v3 {
            // (version 3: whether it has coats, then them; then its ink)
            put64(w, self.coats.is_some() as u64)?;
            if let Some(c) = self.coats.as_deref() {
                etched::write_etch(c, w, thin)?;
            }
            inked::write_ink(self.ink.as_deref().unwrap(), w, thin)?;
        } else if v2 {
            etched::write_etch(self.coats.as_deref().unwrap_or(&etched::Coats::default()), w, thin)?;
        }
        Ok(())
    }

    /// `write` read back (version 1, 2, 3 or 4): refuses another version, a
    /// damaged size, a tool it doesn't know and trailing data.
    pub fn read(r: &mut impl Read) -> io::Result<Plate> {
        let bad = |m: &str| io::Error::new(io::ErrorKind::InvalidData, format!("plate: {m}"));
        let mut m = [0u8; 4];
        r.read_exact(&mut m)?;
        if &m != PLAT_MAGIC {
            return Err(bad("not a plate's section"));
        }
        let mut v = get64(r)?;
        if !(1..=PLAT_V).contains(&v) {
            return Err(bad(&format!("a plate of version {v}: this build reads versions 1 to {PLAT_V}")));
        }
        let (mut contact, mut thin) = (false, false);
        if v == 4 {
            // (version 4: its laws, then the version its layout is)
            let laws = get64(r)?;
            if !(1..=3).contains(&laws) {
                return Err(bad(&format!("laws {laws:#x} this build doesn't know")));
            }
            contact = laws & 1 == 1;
            thin = laws & 2 == 2;
            v = get64(r)?;
            if !(1..=3).contains(&v) {
                return Err(bad(&format!("a version-4 plate laid out as version {v}")));
            }
        }
        let metal = Metal::of(get64(r)?).ok_or_else(|| bad("a metal this build doesn't know"))?;
        let temper = Temper::of(get64(r)?).ok_or_else(|| bad("a temper this build doesn't know"))?;
        if vickers(metal, temper).is_none() {
            return Err(bad("a metal in a temper it isn't sold in"));
        }
        let f = |r: &mut dyn Read| -> io::Result<f32> { Ok(f32::from_bits(get32(r)?)) };
        let (width_mm, height_mm, thickness_mm, bevel_deg) = (f(r)?, f(r)?, f(r)?, f(r)?);
        let (w, h) = (get64(r)? as usize, get64(r)? as usize);
        if w == 0 || h == 0 || w > 1 << 16 || h > 1 << 16 || ![width_mm, height_mm, thickness_mm, bevel_deg].iter().all(|v| v.is_finite() && *v > 0.0) {
            return Err(bad("its size is damaged"));
        }
        let pitch = f64::from_bits(get64(r)?);
        let chip = f64::from_bits(get64(r)?);
        let mut p = Plate::new(metal, temper, width_mm, height_mm, thickness_mm, bevel_deg, w, h);
        if pitch.to_bits() != p.pitch.to_bits() || !chip.is_finite() {
            return Err(bad("its pitch is damaged"));
        }
        p.chip_um3 = chip;
        p.contact = contact;
        p.facing = match get64(r)? {
            0 => None,
            1 => Some(Facing { hv: f(r)?, um: f(r)? }),
            _ => return Err(bad("its facing is damaged")),
        };
        let nops = get64(r)?;
        if nops > 1 << 24 {
            return Err(bad("too many tool uses"));
        }
        for _ in 0..nops {
            let tag = get64(r)?;
            let (a, b, c) = (f(r)?, f(r)?, f(r)?);
            let n = get64(r)?;
            if n > 1 << 24 {
                return Err(bad("a tool's path is too long"));
            }
            let mut pts = Vec::with_capacity(n as usize);
            for _ in 0..n {
                pts.push(Point { x: f(r)?, y: f(r)?, force: f(r)?, alt: f(r)?, az: f(r)?, speed: f(r)? });
            }
            if tag <= 3 {
                p.ops.push(match tag {
                    0 => Op::Drypoint { tip_um: a, cone_deg: b, pts },
                    1 => Op::Burnish { radius_mm: a, passes: b as u32, pts },
                    2 => Op::Scrape { width_mm: a, passes: b as u32, pts },
                    _ => Op::Roulette { pitch_mm: a, tooth_um: b, cone_deg: c, pts },
                });
                continue;
            }
            if v < 2 || tag > 11 {
                return Err(bad(&format!("a tool of kind {tag}, which this build doesn't know")));
            }
            let n = get64(r)?;
            if n > 1 << 30 {
                return Err(bad("a tool's data is too long"));
            }
            let mut blob = vec![0u8; n as usize];
            r.read_exact(&mut blob)?;
            let w32 = |b: &[u8], i: usize| u32::from_le_bytes(b[i..i + 4].try_into().unwrap());
            let w64 = |b: &[u8], i: usize| u64::from_le_bytes(b[i..i + 8].try_into().unwrap());
            p.ops.push(match tag {
                4 => Op::Ground { um: a, roller_mm: b, pts },
                5 => Op::Needle { tip_um: a, cone_deg: b, pts },
                6 => Op::StopOut { brush_mm: a, load: b, pts },
                7 if blob.len() == 8 + 4 * crate::etch::DUST_BINS => Op::Aquatint { seed: w64(&blob, 0), per_um2: (0..crate::etch::DUST_BINS).map(|i| f32::from_bits(w32(&blob, 8 + 4 * i))).collect() },
                8 => Op::Melt,
                9 if blob.len() >= 8 && (blob.len() - 8) % 8 == 0 => {
                    let acid = crate::etch::Acid::of(w64(&blob, 0)).ok_or_else(|| bad("an acid this build doesn't know"))?;
                    Op::Bite { acid, doses: (8..blob.len()).step_by(8).map(|i| (f32::from_bits(w32(&blob, i)), f32::from_bits(w32(&blob, i + 4)))).collect() }
                }
                10 => Op::Clean,
                11 if a.is_finite() && a > 0.0 && b >= 1.0 && b.fract() == 0.0 => Op::Wash { pad_mm: a, passes: b as u32, pts },
                _ => return Err(bad("an etching tool's data is damaged")),
            });
        }
        let used = get64(r)?;
        if used > p.tiles.len() as u64 {
            return Err(bad("more tiles than it has"));
        }
        let mut buf = vec![0u8; TILE * TILE * (4 * (3 * BINS + 3) + 2)];
        let mut last = None;
        for _ in 0..used {
            let i = get64(r)? as usize;
            if i >= p.tiles.len() || last.is_some_and(|l| i <= l) {
                return Err(bad("a tile out of place"));
            }
            last = Some(i);
            r.read_exact(&mut buf)?;
            let mut at = 0;
            let word = |at: &mut usize| {
                let v = f32::from_bits(u32::from_le_bytes(buf[*at..*at + 4].try_into().unwrap()));
                *at += 4;
                v
            };
            let mut cells = vec![Cell::EMPTY; TILE * TILE];
            for c in cells.iter_mut() {
                for k in 0..BINS {
                    c.open[k] = word(&mut at);
                }
                for k in 0..BINS {
                    c.vol[k] = word(&mut at);
                }
                for k in 0..BINS {
                    c.wsum[k] = word(&mut at);
                }
                c.burr_vol = word(&mut at);
                c.burr_area = word(&mut at);
                c.level = word(&mut at);
                c.dir = buf[at];
                c.side = buf[at + 1];
                at += 2;
            }
            p.tiles[i] = Some(cells.into_boxed_slice());
        }
        if v == 2 {
            p.coats = Some(Box::new(etched::read_etch(&p, r, thin)?));
        } else if v == 3 {
            match get64(r)? {
                0 => {}
                1 => p.coats = Some(Box::new(etched::read_etch(&p, r, thin)?)),
                _ => return Err(bad("its coats' mark is damaged")),
            }
            p.ink = Some(Box::new(inked::read_ink(&p, r, thin)?));
        }
        if r.read(&mut [0u8; 1])? != 0 {
            return Err(bad("trailing data"));
        }
        Ok(p)
    }
}

impl Plate {
    /// Whether a solvent rag has left a coat thinned on the plate, or a
    /// stripper lies on its ink or has loosened any (`write`'s laws bit 1).
    fn thin(&self) -> bool {
        self.coats.as_deref().is_some_and(etched::Coats::thinned) || self.ink.as_deref().is_some_and(inked::Inking::stripped)
    }
}

const PLAT_MAGIC: &[u8; 4] = b"PLAT";
/// The newest version of a plate's section (`write`): 4, a plate made under
/// the contact laws (`--@ intaglio-contact`), around one of the others' layouts;
/// 3, a plate that has been inked (`--@ press`); 2, a plate an etching tool
/// has touched, and 1 are still written for plates without them, and read.
pub const PLAT_V: u64 = 4;

fn put64(w: &mut impl Write, v: u64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn put32(w: &mut impl Write, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn get64(r: &mut (impl Read + ?Sized)) -> io::Result<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}

fn get32(r: &mut (impl Read + ?Sized)) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

pub mod etched;
pub mod hand;
pub mod inked;
pub mod press;

#[cfg(test)]
mod tests;
