//! Paper as a support (`Canvas::with_paper`): a sheet made the way paper is
//! made, not a texture. Sources: notes/research/paper_surface.md.
//!
//! - **Fibers.** The sheet is a random fiber network (Kallmes–Corte; Dodson;
//!   Sampson): fiber centers land as a Poisson process, each fiber a
//!   segment of a length drawn from the pulp's distribution, a width ω and a
//!   coarseness δ (mass per length), at a uniform angle. Each fiber's mass
//!   is laid along its length into the pixels it crosses, so the local
//!   grammage (and its correlations along fibers and between them) comes
//!   from the fibers themselves. Mean coverage is c̄ = βω/δ (β the
//!   grammage): about 25 fibers over a point for a 160 g/m² sheet.
//! - **Flocs.** Long fibers flocculate in the suspension: a share of the
//!   fibers lands in clusters (a Neyman–Scott process: floc centers
//!   Poisson, each floc's fibers scattered round its center), which gives
//!   the cloudy formation at a few mm.
//! - **The mold.** Over the wires of a laid mold the sheet drains faster
//!   and carries less fiber: fiber centers there are thinned (laid lines
//!   about 1 mm apart, chain lines about 25 mm apart, wandering).
//! - **Pressing.** Wet pressing turns most of the thickness variation into
//!   density variation: the surface follows the local fiber count to a
//!   power below one (T ∝ c^0.5–0.8).
//! - **Felt.** A sheet couched and pressed on wool felt takes the felt's
//!   cells (the "grain"): cells about 0.5–1 mm across with raised rims, tens
//!   of µm deep. The cells are the felt's (seeded Poisson, nearest-seed
//!   cells); the depth and cell size are estimates (no profilometry of
//!   artists' papers was found).
//! - **Calendering** presses the highest spots down.
//! - **Micro-roughness.** Below a pixel (0.1–0.3 mm), the surface between the
//!   fiber tops is a set of pores whose depth below the top envelope is
//!   exponentially distributed with a mean about the collapsed fiber
//!   thickness (Dodson; Sampson–Wang). It is kept as that mean, µ, per
//!   pixel: the bearing (Abbott) curve a stick or a finger meets is
//!   1 − e^(−d/µ) at depth d.
//!
//! Everything is laid in millimeters of the sheet, so the paper is the same
//! at any pixel width and in a crop render.

use crate::rng::{Rng, hash2};
use rayon::prelude::*;

/// The grain a sheet takes from the felt it was couched and pressed on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Felt {
    /// Cell size, mm (the distance between felt knuckles).
    pub cell_mm: f32,
    /// Depth of the imprint, µm (rims to cell floors).
    pub depth_um: f32,
}

/// The wires of a laid mold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Laid {
    /// Laid lines per cm.
    pub per_cm: f32,
    /// Chain-line spacing, mm.
    pub chain_mm: f32,
    /// Share of the fiber missing over a wire (0.1–0.3 on handmade sheets).
    pub deficit: f32,
}

/// A sheet of paper.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Paper {
    /// Grammage, g/m².
    pub grammage: f32,
    /// Mean fiber length, mm (cotton rag after beating 1–3, softwood 2–3.5,
    /// hardwood 0.7–1.2).
    pub fiber_mm: f32,
    /// Fiber width, µm.
    pub fiber_um: f32,
    /// Collapsed fiber thickness, µm (4–10).
    pub thick_um: f32,
    /// Coarseness, mg per m of fiber (0.06–0.25).
    pub coarseness: f32,
    /// Sheet porosity (0.45–0.6).
    pub porosity: f32,
    /// Share of the fibers that land in flocs (0..1).
    pub floc: f32,
    /// Floc radius, mm.
    pub floc_mm: f32,
    /// How the surface follows the fiber count after pressing: T ∝ c^press
    /// (0.5 hard-pressed .. 0.8 soft).
    pub press: f32,
    /// Calendering, 0 (none) .. 1 (the highest spots pressed right down).
    pub calender: f32,
    pub felt: Option<Felt>,
    pub laid: Option<Laid>,
    /// How much of the sheet's pore volume takes oil (sizing slows water,
    /// barely oil): 0..1.
    pub absorbent: f32,
    /// Through-thickness (z) modulus, MPa: how the sheet gives under a
    /// stick (39 MPa measured on board, Mann, Baum & Habeger 1979; a soft
    /// drawing paper lower). A mount's backing (`Paper::mounted`) is folded
    /// into it: the give of the sheet and what it is laid on, together.
    pub z_mpa: f32,
    /// A sheet on canvas over a strainer: the bay between the bars drums
    /// under a stick (`Paper::give_at`). None on a firm bed.
    pub drum: Option<Drum>,
    pub seed: u64,
}

/// What a sheet is laid down on (notes/research/cassatt_papers.md §1).
/// Without one, the sheet lies on a hard drawing board, all but rigid under
/// a stick (the easel before mounts). Each mount's give is derived from
/// measured moduli under a stick's reference contact (`REF_FORCE_N`,
/// `ref_patch_mm`): what the contact's Winkler springs need is how far the
/// patch sinks relative to the surface round it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mount {
    /// Laid down on pasteboard or millboard (on a table): the board
    /// compresses through its thickness, its plies spreading the load
    /// (`board_mpa_per_um`).
    Board,
    /// Laid down on canvas on a firm bed (a lined canvas, or one taken off
    /// its strainer): the cloth compacts under the sheet
    /// (`cloth_mpa_per_um`).
    Canvas,
    /// Laid on canvas tacked to a wooden strainer: the cloth, and between
    /// the bars the bay drums (`Drum`). The bars are `bar_mm` wide under
    /// the picture, and `cross` crossbars as wide divide the longer side
    /// into equal bays; the canvas rests on all of them.
    Strainer { bar_mm: f32, cross: u32 },
}

/// The bay of a strainer under a sheet on canvas: the sheet and canvas are
/// a pre-tensioned plate (bending stiffness D, tension T) on the bars,
/// which deflects under the stick's patch. Firm over the bars, it softens
/// over a few bending lengths inward, then gives the same over the rest of
/// the bay (`Paper::give_at`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drum {
    /// The bars' width under the picture, mm (the outer bars from each
    /// edge, and each crossbar).
    pub bar_mm: f32,
    /// The bay's own give away from the bars, MPa per µm: the reference
    /// patch's pressure over how far its middle sinks below its rim,
    /// 2T / (a² h(a/ℓ)) (`drum_mpa_per_um`). In series with the sheet and
    /// canvas.
    pub bay_mpa_per_um: f32,
    /// The plate's bending length ℓ = √(D/T), mm: over shorter lengths
    /// the sheet bends, over longer ones the canvas stretches.
    pub bend_mm: f32,
    /// The reference patch's radius a, mm (`ref_patch_mm`).
    pub patch_mm: f32,
    /// Crossbars dividing the longer side into equal bays.
    pub cross: u32,
}

impl Mount {
    /// A mount by name (a strainer's bars as `STRAINER_BAR_MM`, no
    /// crossbar).
    pub fn named(name: &str) -> Option<Mount> {
        match name {
            "board" => Some(Mount::Board),
            "canvas" => Some(Mount::Canvas),
            "strainer" => Some(Mount::Strainer { bar_mm: STRAINER_BAR_MM, cross: 0 }),
            _ => None,
        }
    }

    /// What the sheet is laid on gives under the reference contact, MPa per
    /// µm, in series with the sheet's own: board about 0.015, canvas cloth
    /// about 0.0005 (also over a strainer's bars and, under the drum, in
    /// its bay).
    pub fn backing_mpa_per_um(self) -> f32 {
        match self {
            Mount::Board => board_mpa_per_um(),
            Mount::Canvas | Mount::Strainer { .. } => cloth_mpa_per_um(),
        }
    }
}

/// A strainer's bars, mm, by default (measured: the original rectangular
/// strainers of Jens Juel's portraits had bars 4–6 cm wide, Slotsgaard,
/// Arts 11 (2022) 55; `mount={"strainer", bar=}` sets it).
pub const STRAINER_BAR_MM: f32 = 50.0;
/// The most crossbars a strainer is given here.
pub const MAX_CROSS: u32 = 8;

// The reference contact: a round soft stick's full facet under a drawing
// hand (notes/research/pastel_stick_tribology.md §4–5).
/// The hand's force, N (measured: 2–3 N drawing on a tablet, Xin, Bi & Ren
/// 2012).
pub const REF_FORCE_N: f32 = 2.5;
/// The stick's radius, mm (a round soft pastel, Ø 11–13 mm).
const REF_STICK_MM: f32 = 6.0;
/// The stick's altitude, degrees (measured: a pen held at about 50°,
/// Schomaker & Plamondon 1990).
const REF_ALTITUDE_DEG: f32 = 50.0;

/// The reference patch's radius, mm: the full facet of a stick of radius R
/// worn flat at altitude θ is an ellipse of semi-axes R and R / sin θ, of
/// area πR²/sin θ, so a = R / √sin θ ≈ 6.9 mm.
pub fn ref_patch_mm() -> f32 {
    REF_STICK_MM / REF_ALTITUDE_DEG.to_radians().sin().sqrt()
}

/// The reference contact's mean pressure, MPa (N/mm²): F / πa² ≈ 0.017.
fn ref_pressure_mpa() -> f32 {
    let a = ref_patch_mm();
    REF_FORCE_N / (std::f32::consts::PI * a * a)
}

/// Paper's in-plane modulus per density, MPa per kg/m³ (MN·m/kg): the
/// geometric mean of the measured E₁₁ ≈ 190 × 39 MPa and E₂₂ = 3.47 GPa
/// (Mann, Baum & Habeger 1979, ultrasonic, board), 5.07 GPa, over a
/// board's density of about 700 kg/m³ (estimate: linerboard), brought to a
/// stroke's time scale: paper's ultrasonic modulus is 25–40% above its
/// mechanical one (viscoelastic; Waterhouse, IPST TPS 495, 1993), and a
/// stroke loads the sheet over tens of ms, so ÷1.3: about 5.6.
const PAPER_SPECIFIC_MODULUS: f32 = 5070.0 / 700.0 / 1.3;
/// A canvas tacked days before it is worked, N/mm: hand-stretched at about
/// 200 N/m, it loses 30–50% of that in the first 48 h (measured, Berger &
/// Russell, JAIC 29 (1990)), so 0.2 × 0.65 (inside paintings' measured
/// 0.12–0.24 kN/m, Heritage Science 2022).
pub const CANVAS_TENSION_N_PER_MM: f32 = 0.13;
/// A painting canvas's thickness, mm (measured range 0.3–0.6).
const CLOTH_MM: f32 = 0.4;
/// A woven fabric's compaction: its thickness goes as p^(−B) (measured: a
/// single woven flax ply, 1.345 mm at 1 kPa and 0.946 mm at 100 kPa, B =
/// ln(1.345/0.946)/ln 100; Rayyaan et al., arXiv 2110.06110; the other three
/// flax structures lost the same ≈30%). This is the soft bound: a weave
/// filled with dried paste and size compacts at most like board (E_z ≥ 17
/// MPa over 0.4 mm, k ≤ 0.04 MPa/µm), and a thin paste layer leaves most
/// of the interstices open, so the true value is near this end; it needs
/// a bench test (notes/research/cassatt_papers.md §1).
const CLOTH_COMPACTION_EXP: f32 = 0.0764;
/// Her board mounts ("illustration board", "cardboard": Met 10424, 10404):
/// thickness, mm (a heavyweight illustration board, Crescent No. 100,
/// 0.09 in); through-thickness modulus and shear modulus, MPa (measured,
/// quasi-static, on paperboard: E_initial 34 and G_initial 32 MPa,
/// Stenberg, Int. J. Solids Struct. 2003; G_xz 30–85 MPa, Nygårds 2008).
const BOARD_MM: f32 = 2.3;
const BOARD_EZ_MPA: f32 = 34.0;
const BOARD_GXZ_MPA: f32 = 32.0;

/// The cloth's give, MPa per µm: a thin layer (0.4 mm against a patch of
/// 7 mm, so a Winkler spring E/t), whose tangent modulus at pressure p is
/// E = −t dp/dt = p/B for t ∝ p^(−B): at the reference pressure about
/// 0.22 MPa, so 0.00055 MPa/µm. (The sheet's bending over it changes the
/// patch's sink by under 10%: left out.)
pub fn cloth_mpa_per_um() -> f32 {
    ref_pressure_mpa() / CLOTH_COMPACTION_EXP / (CLOTH_MM * 1000.0)
}

/// A board's give, MPa per µm: its plies are about 190 times stiffer in
/// plane than through the thickness, so they barely stretch and the board
/// is a shear-coupled (Pasternak) layer on the table: k = E_z/t, shear
/// G_xz·t/3, spreading length L = t√(G_xz/3E_z). Under a uniform patch of
/// radius a its middle sinks (p/k)(1 − (a/L)K₁(a/L)): about 0.015 MPa/µm.
/// (The sheet's bending on it changes that under 1%: left out.)
pub fn board_mpa_per_um() -> f32 {
    let k = BOARD_EZ_MPA / BOARD_MM; // N/mm³
    let l = BOARD_MM * (BOARD_GXZ_MPA / (3.0 * BOARD_EZ_MPA)).sqrt();
    let x = (ref_patch_mm() / l) as f64;
    let sink = 1.0 - x * bessel_k1(x);
    // N/mm³ = MPa/mm = 10⁻³ MPa/µm
    (k as f64 / sink * 1e-3) as f32
}

/// The bay's sink under a patch of radius a, in units of F/(2πT), its middle
/// below its rim, for a pre-tensioned plate (D∇⁴w − T∇²w = p) far from its
/// edges: w = −(F/2πT)[ln r + K₀(r/ℓ)] for a point load, so for the patch
/// h(β) = ½ − (2/β) K₁(β) (I₀(β) − 1), β = a/ℓ: ½ for a membrane (β → ∞),
/// (β²/4)(ln(2/β) − γ + 3/8) for a plate (β → 0).
fn drum_sink(beta: f64) -> f64 {
    0.5 - 2.0 / beta * bessel_k1(beta) * (bessel_i0(beta) - 1.0)
}

/// The bay's give away from the bars, MPa per µm, for a sheet of bending
/// stiffness `d_nmm` N·mm on canvas at tension `t_n_per_mm` under a patch
/// of radius `a_mm`: p over the sink, 2T / (a² h(a/ℓ)) N/mm³.
pub fn drum_mpa_per_um(d_nmm: f32, t_n_per_mm: f32, a_mm: f32) -> f32 {
    let l = (d_nmm / t_n_per_mm).sqrt() as f64;
    let a = a_mm as f64;
    (2.0 * t_n_per_mm as f64 / (a * a * drum_sink(a / l)) * 1e-3) as f32
}

/// Where (`x` mm along a side `len` mm) a strainer of `bar` mm bars and
/// `cross` crossbars across it puts the canvas: on a bar (None), or in a
/// bay, how far in from the bars either side (mm).
fn bay_at(x: f32, len: f32, bar: f32, cross: u32) -> Option<(f32, f32)> {
    let (mut lo, mut hi) = (x - bar, len - bar - x);
    // crossbars centered at len·i/(n+1), each `bar` wide
    for i in 1..=cross {
        let c = len * i as f32 / (cross + 1) as f32;
        if x >= c {
            lo = lo.min(x - c - 0.5 * bar);
        } else {
            hi = hi.min(c - 0.5 * bar - x);
        }
    }
    (lo > 0.0 && hi > 0.0).then_some((lo, hi))
}

// Modified Bessel functions (Abramowitz & Stegun 9.8.1–9.8.8; |error| < 2e-7
// relative).
fn bessel_i0(x: f64) -> f64 {
    let ax = x.abs();
    if ax < 3.75 {
        let y = (x / 3.75).powi(2);
        1.0 + y * (3.5156229 + y * (3.0899424 + y * (1.2067492 + y * (0.2659732 + y * (0.0360768 + y * 0.0045813)))))
    } else {
        let y = 3.75 / ax;
        (ax.exp() / ax.sqrt()) * (0.39894228 + y * (0.01328592 + y * (0.00225319 + y * (-0.00157565 + y * (0.00916281 + y * (-0.02057706 + y * (0.02635537 + y * (-0.01647633 + y * 0.00392377))))))))
    }
}

fn bessel_i1(x: f64) -> f64 {
    let ax = x.abs();
    let v = if ax < 3.75 {
        let y = (x / 3.75).powi(2);
        ax * (0.5 + y * (0.87890594 + y * (0.51498869 + y * (0.15084934 + y * (0.02658733 + y * (0.00301532 + y * 0.00032411))))))
    } else {
        let y = 3.75 / ax;
        let p = 0.02282967 + y * (-0.02895312 + y * (0.01787654 - y * 0.00420059));
        let p = 0.39894228 + y * (-0.03988024 + y * (-0.00362018 + y * (0.00163801 + y * (-0.01031555 + y * p))));
        p * (ax.exp() / ax.sqrt())
    };
    if x < 0.0 { -v } else { v }
}

fn bessel_k0(x: f64) -> f64 {
    if x <= 2.0 {
        let y = x * x / 4.0;
        -(x / 2.0).ln() * bessel_i0(x) + (-0.57721566 + y * (0.42278420 + y * (0.23069756 + y * (0.03488590 + y * (0.00262698 + y * (0.00010750 + y * 0.0000074))))))
    } else {
        let y = 2.0 / x;
        ((-x).exp() / x.sqrt()) * (1.25331414 + y * (-0.07832358 + y * (0.02189568 + y * (-0.01062446 + y * (0.00587872 + y * (-0.00251540 + y * 0.00053208))))))
    }
}

fn bessel_k1(x: f64) -> f64 {
    if x <= 2.0 {
        let y = x * x / 4.0;
        (x / 2.0).ln() * bessel_i1(x) + (1.0 / x) * (1.0 + y * (0.15443144 + y * (-0.67278579 + y * (-0.18156897 + y * (-0.01919402 + y * (-0.00110404 + y * -0.00004686))))))
    } else {
        let y = 2.0 / x;
        ((-x).exp() / x.sqrt()) * (1.25331414 + y * (0.23498619 + y * (-0.03655620 + y * (0.01504268 + y * (-0.00780353 + y * (0.00325614 + y * -0.00068245))))))
    }
}

/// A named sheet (`Paper::preset`): the dials of a paper of Cassatt's and
/// her circle's, and its tone as parts of tubes of the catalog. Sources
/// and estimates: notes/research/cassatt_papers.md.
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    pub name: &'static str,
    pub grammage: f32,
    pub fiber_mm: f32,
    pub fiber_um: f32,
    pub thick_um: f32,
    pub coarseness: f32,
    pub porosity: f32,
    pub floc: f32,
    pub floc_mm: f32,
    pub press: f32,
    pub calender: f32,
    pub felt: Option<Felt>,
    pub laid: Option<Laid>,
    /// The sheet's tone: parts of the catalog's tubes, seen dry.
    pub tone: &'static [(&'static str, f32)],
}

/// A colored machine-made wove pastel sheet of the 1880s–1900s: wood pulp
/// with some rag, about 130 g/m², a light felt mark (estimates; the
/// research note).
const WOVE: Preset = Preset {
    name: "",
    grammage: 130.0,
    fiber_mm: 1.6,
    fiber_um: 25.0,
    thick_um: 6.0,
    coarseness: 0.16,
    porosity: 0.5,
    floc: 0.5,
    floc_mm: 1.5,
    press: 0.65,
    calender: 0.25,
    felt: Some(Felt { cell_mm: 0.7, depth_um: 14.0 }),
    laid: None,
    tone: &[],
};

/// The named sheets (the research note's table).
pub const PRESETS: &[Preset] = &[
    Preset { name: "blue wove", tone: &[("lead white", 1.0), ("ultramarine blue", 1.0), ("bone black", 0.45)], ..WOVE },
    Preset { name: "gray-blue wove", tone: &[("lead white", 1.0), ("ultramarine blue", 1.0), ("viridian", 1.5), ("red earth", 1.5)], ..WOVE },
    Preset { name: "tan wove", tone: &[("yellow ochre", 0.2), ("bitumen", 1.0), ("vine black", 0.23)], ..WOVE },
    Preset { name: "buff wove", tone: &[("lead white", 1.0), ("yellow ochre", 2.2), ("raw umber", 3.0), ("Prussian blue", 0.02)], ..WOVE },
    Preset { name: "green wove", tone: &[("lead white", 1.0), ("yellow ochre", 0.65), ("viridian", 0.45), ("bone black", 0.45)], ..WOVE },
    // brown: an unbleached wood-pulp sheet, heavier and coarser
    Preset { name: "brown wove", grammage: 150.0, fiber_mm: 1.4, coarseness: 0.2, floc: 0.6, felt: Some(Felt { cell_mm: 0.8, depth_um: 18.0 }), tone: &[("raw sienna", 1.0), ("bitumen", 3.0)], ..WOVE },
    // a handmade or mold-made rag laid sheet (her print and drawing papers)
    Preset { name: "cream laid", grammage: 110.0, fiber_mm: 2.2, calender: 0.2, felt: Some(Felt { cell_mm: 0.8, depth_um: 15.0 }), laid: Some(Laid { per_cm: 9.0, chain_mm: 25.0, deficit: 0.15 }), tone: &[("lead white", 1.0), ("yellow ochre", 0.15), ("raw sienna", 0.02), ("green earth", 3.0)], ..WOVE },
    // Japan paper: long kozo fibers, thin, open, well formed, smooth
    Preset { name: "japan paper", grammage: 45.0, fiber_mm: 10.0, fiber_um: 30.0, thick_um: 5.0, coarseness: 0.1, porosity: 0.6, floc: 0.15, floc_mm: 2.0, press: 0.7, calender: 0.6, felt: None, laid: None, tone: &[("lead white", 1.0), ("yellow ochre", 0.1), ("raw sienna", 0.1), ("viridian", 0.02)] },
];

impl Preset {
    /// The sheet of this make (a drawing paper's give and absorbency: the
    /// sources give none for these sheets; the absorbency, 0.8, lies between
    /// a hard-sized sheet's and a waterleaf's 1, and wants a bench test).
    pub fn paper(&self, seed: u64) -> Paper {
        Paper {
            grammage: self.grammage,
            fiber_mm: self.fiber_mm,
            fiber_um: self.fiber_um,
            thick_um: self.thick_um,
            coarseness: self.coarseness,
            porosity: self.porosity,
            floc: self.floc,
            floc_mm: self.floc_mm,
            press: self.press,
            calender: self.calender,
            felt: self.felt,
            laid: self.laid,
            ..Paper::drawing(seed)
        }
    }
}

impl Paper {
    /// A 160 g/m² cotton-rich drawing paper with a felt grain: about the
    /// pastel papers of the colormen (estimates where no measurement was
    /// found; notes/research/paper_surface.md).
    pub fn drawing(seed: u64) -> Paper {
        Paper {
            grammage: 160.0,
            fiber_mm: 2.0,
            fiber_um: 25.0,
            thick_um: 6.0,
            coarseness: 0.16,
            porosity: 0.5,
            floc: 0.5,
            floc_mm: 1.5,
            press: 0.65,
            calender: 0.2,
            felt: Some(Felt { cell_mm: 0.8, depth_um: 20.0 }),
            laid: None,
            absorbent: 0.8,
            z_mpa: 39.0,
            drum: None,
            seed,
        }
    }

    /// The named sheet `name` (`PRESETS`), if there is one.
    pub fn preset(name: &str) -> Option<&'static Preset> {
        PRESETS.iter().find(|p| p.name == name)
    }

    /// This sheet laid down on `m`: the backing's give in series with the
    /// sheet's own, folded into `z_mpa` (so the stiffness a stick meets,
    /// `z_mpa / caliper`, is the two springs together), and on a strainer
    /// the bay's drum, from the sheet's own bending stiffness.
    pub fn mounted(mut self, m: Mount) -> Paper {
        let cal = self.caliper_um().max(10.0);
        let k = 1.0 / (cal / self.z_mpa + 1.0 / m.backing_mpa_per_um());
        if let Mount::Strainer { bar_mm, cross } = m {
            let (d, t, a) = (self.bending_nmm(), CANVAS_TENSION_N_PER_MM, ref_patch_mm());
            self.drum = Some(Drum { bar_mm, bay_mpa_per_um: drum_mpa_per_um(d, t, a), bend_mm: (d / t).sqrt(), patch_mm: a, cross });
        }
        self.z_mpa = k * cal;
        self
    }

    /// The sheet's bending stiffness, N·mm: E t³/12, its in-plane modulus
    /// from its density (`PAPER_SPECIFIC_MODULUS`) and t its caliper. A
    /// canvas carries little bending (it is soft in plane and slackens in
    /// compression), so the laminate's is the sheet's: about 6.7 N·mm for
    /// the drawing paper (300 µm, 530 kg/m³).
    pub fn bending_nmm(&self) -> f32 {
        let t_mm = self.caliper_um().max(10.0) * 1e-3;
        // g/m² over µm is g/cm³, ×1000 kg/m³
        let rho = self.grammage / (t_mm * 1000.0) * 1000.0;
        PAPER_SPECIFIC_MODULUS * rho * t_mm.powi(3) / 12.0
    }

    /// How stiffly the sheet gives under a stick at (`xm`, `ym`) mm of a
    /// sheet `w_mm` × `h_mm`, MPa per µm (a Winkler spring): its z modulus
    /// over its caliper, and on a strainer, in a bay, the drum in series.
    ///
    /// The drum is the bay's sink under the reference patch centered here,
    /// its middle below its rim. The bars hold the canvas simply supported
    /// (w = ∇²w = 0 on a straight edge), which a mirror patch of opposite
    /// sign across each bar's edge gives exactly, and one of the same sign
    /// across each corner. A mirror's membrane part (ln r) is harmonic over
    /// the patch, so it sinks the patch's middle and its rim alike and
    /// leaves the sink unchanged; its bending part, (F/2πT)(2/β)I₁(β)K₀(r/ℓ)
    /// at a distance r from it, takes (I₀(β) − 1) times that off it.
    /// So the sink is h(β) less those, and the bay stiffens within a few
    /// bending lengths of a bar (to the canvas's give at the bar) and gives
    /// the same everywhere further in, however large the bay: the bay's
    /// size sets only the whole sheet's sink under the hand (millimeters,
    /// the felt "drum"), which carries the stick down with the surface
    /// round it. Exact for a patch clear of the bars (more than a in from
    /// one). Nearer, where the facet rests partly on the bar, the mirror's
    /// continuation: it firms the sheet up to the bar's own give at the
    /// bar's edge, without a step (a rigid facet held up by the bar can't
    /// sink into the bay beside it).
    pub fn give_at(&self, xm: f32, ym: f32, w_mm: f32, h_mm: f32) -> f32 {
        let k = self.z_mpa / self.caliper_um().max(10.0);
        let Some(d) = self.drum else { return k };
        // the crossbars divide the longer side
        let (cx, cy) = if w_mm >= h_mm { (d.cross, 0) } else { (0, d.cross) };
        let (Some((x0, x1)), Some((y0, y1))) = (bay_at(xm, w_mm, d.bar_mm, cx), bay_at(ym, h_mm, d.bar_mm, cy)) else { return k };
        let (l, b) = (d.bend_mm as f64, (d.patch_mm / d.bend_mm) as f64);
        let mirror = |r: f64| if r > 40.0 * l { 0.0 } else { bessel_k0(r / l) };
        let (x0, x1, y0, y1) = (x0 as f64, x1 as f64, y0 as f64, y1 as f64);
        let edges = mirror(2.0 * x0) + mirror(2.0 * x1) + mirror(2.0 * y0) + mirror(2.0 * y1);
        let corners = [(x0, y0), (x0, y1), (x1, y0), (x1, y1)].iter().map(|&(p, q)| mirror(2.0 * (p * p + q * q).sqrt())).sum::<f64>();
        let free = drum_sink(b);
        let sink = free - 2.0 / b * bessel_i1(b) * (bessel_i0(b) - 1.0) * (edges - corners);
        if sink <= 0.0 {
            return k;
        }
        let drum = d.bay_mpa_per_um as f64 * free / sink;
        (1.0 / (1.0 / k as f64 + 1.0 / drum)) as f32
    }

    /// Mean number of fibers over a point: c̄ = βω/δ.
    pub fn coverage(&self) -> f32 {
        // β g/m² = β·1e-3 mg/mm²; δ mg/m = δ·1e-3 mg/mm; ω µm = ω·1e-3 mm
        (self.grammage * 1e-3) * (self.fiber_um * 1e-3) / (self.coarseness * 1e-3)
    }

    /// Mean caliper, µm: c̄·t_f / (1 − ε).
    pub fn caliper_um(&self) -> f32 {
        self.coverage() * self.thick_um / (1.0 - self.porosity).max(0.05)
    }

    /// Fibers per mm² of sheet: β / (δ·λ̄).
    fn fibers_per_mm2(&self) -> f32 {
        (self.grammage * 1e-3) / (self.coarseness * 1e-3 * self.fiber_mm)
    }

    /// How much of a laid mold's wires lie under a point (mm): 0 between
    /// wires, 1 on one.
    fn wire(&self, xm: f32, ym: f32) -> f32 {
        let Some(l) = self.laid else { return 0.0 };
        // laid wires run across the sheet (horizontal), about 0.4 mm thick
        let pitch = 10.0 / l.per_cm.max(1.0);
        // (a non-negative phase: fibers of the halo above the sheet's top
        // edge see the same wires)
        let fy = (ym / pitch).rem_euclid(1.0);
        let d = fy.min(1.0 - fy) * pitch;
        let laid = (1.0 - d / 0.2).max(0.0);
        // chain wires run down it, about 0.5 mm thick, wandering a little
        // (each chain line is a wire of its own: its offset is the mold's)
        let k = (xm / l.chain_mm).floor();
        let off = 0.12 * l.chain_mm * (hash2(k as i64, 7, self.seed ^ 0xC4A1) - 0.5);
        let xc = (k + 0.5) * l.chain_mm + off;
        let chain = (1.0 - (xm - xc).abs() / 0.25).max(0.0);
        laid.max(chain)
    }
}

/// Fiber centers are generated per square cell of the sheet this big (mm),
/// each cell from its own seed, so any part of the sheet can be laid
/// without the rest.
const CELL_MM: f32 = 4.0;

/// The sheet over a window of pixels: surface height (µm, about 0 mean) and
/// the micro-roughness µ (µm) per pixel, and the grammage (g/m²).
pub struct Sheet {
    pub height: Vec<f32>,
    pub micro: Vec<f32>,
    pub grammage: Vec<f32>,
}

/// Poisson count with mean `m` (Knuth for small, normal approximation for
/// large means).
fn poisson(rng: &mut Rng, m: f32) -> u32 {
    if m <= 0.0 {
        return 0;
    }
    if m > 40.0 {
        return (m + m.sqrt() * rng.normal()).round().max(0.0) as u32;
    }
    let l = (-m).exp();
    let (mut k, mut p) = (0u32, 1.0f32);
    loop {
        p *= rng.f();
        if p <= l {
            return k;
        }
        k += 1;
    }
}

#[inline]
fn fiber_misses_window(cx: f32, cy: f32, len: f32, (x0, y0, w, h): (usize, usize, usize, usize), px_mm: f32) -> bool {
    if !px_mm.is_finite() || px_mm <= 0.0 {
        return false;
    }
    let radius = 0.5 * len.abs();
    let fy0 = (cy - radius) / px_mm - 0.5 - y0 as f32;
    let fy1 = (cy + radius) / px_mm - 0.5 - y0 as f32;
    if fy1 < -1.0 || fy0 >= h as f32 {
        return true;
    }
    let fx0 = (cx - radius) / px_mm - 0.5 - x0 as f32;
    let fx1 = (cx + radius) / px_mm - 0.5 - x0 as f32;
    fx1 < -1.0 || fx0.floor() as i64 >= w as i64
}

/// Lay the sheet over the window `(x0, y0, w, h)` of a canvas whose pixels
/// are `px_mm` mm.
pub fn lay(p: &Paper, x0: usize, y0: usize, w: usize, h: usize, px_mm: f32) -> Sheet {
    let n = w * h;
    let mut gram = vec![0.0f32; n];
    let per_mm2 = p.fibers_per_mm2();
    // the fibers' reach beyond their cell: half the longest fiber (lengths
    // are held to 4 means) and a floc's widest spread (`Rng::normal` is at
    // most 3.46), so a crop lays exactly the fibers the whole sheet does
    let reach_mm = 2.0 * p.fiber_mm + 3.5 * p.floc_mm;
    let band = 48usize;
    let bands: Vec<(usize, usize)> = (0..h).step_by(band).map(|b| (b, (b + band).min(h))).collect();
    let mass_per_mm = p.coarseness * 1e-3; // mg per mm of fiber
    let px_area = px_mm * px_mm;
    let parts: Vec<(usize, Vec<f32>)> = bands
        .par_iter()
        .map(|&(b0, b1)| {
            let rows = b1 - b0;
            let mut g = vec![0.0f32; rows * w];
            // the band in sheet mm, grown by the reach
            let (bx0, bx1) = (x0 as f32 * px_mm - reach_mm, (x0 + w) as f32 * px_mm + reach_mm);
            let (by0, by1) = ((y0 + b0) as f32 * px_mm - reach_mm, (y0 + b1) as f32 * px_mm + reach_mm);
            let (cx0, cx1) = ((bx0 / CELL_MM).floor() as i64, (bx1 / CELL_MM).floor() as i64);
            let (cy0, cy1) = ((by0 / CELL_MM).floor() as i64, (by1 / CELL_MM).floor() as i64);
            let mut lay_fiber = |cx: f32, cy: f32, ang: f32, len: f32| {
                if fiber_misses_window(cx, cy, len, (x0, y0 + b0, w, rows), px_mm) {
                    return;
                }
                // the fiber's mass laid along it, half a pixel at a time
                let steps = ((len / (0.5 * px_mm)).ceil() as usize).max(1);
                let (dx, dy) = (ang.cos() * len, ang.sin() * len);
                let m = mass_per_mm * len / steps as f32 / px_area; // mg/mm² per step
                for s in 0..steps {
                    let t = (s as f32 + 0.5) / steps as f32 - 0.5;
                    let (xm, ym) = (cx + dx * t, cy + dy * t);
                    // into the four pixels round the point (area weights)
                    let (fx, fy) = (xm / px_mm - 0.5 - x0 as f32, ym / px_mm - 0.5 - (y0 + b0) as f32);
                    let (ix, iy) = (fx.floor(), fy.floor());
                    let (ax, ay) = (fx - ix, fy - iy);
                    for (oy, wy) in [(0i64, 1.0 - ay), (1, ay)] {
                        let yy = iy as i64 + oy;
                        if yy < 0 || yy >= rows as i64 {
                            continue;
                        }
                        for (ox, wx) in [(0i64, 1.0 - ax), (1, ax)] {
                            let xx = ix as i64 + ox;
                            if xx < 0 || xx >= w as i64 {
                                continue;
                            }
                            g[yy as usize * w + xx as usize] += m * wx * wy;
                        }
                    }
                }
            };
            for cy in cy0..=cy1 {
                for cx in cx0..=cx1 {
                    let mut rng = Rng::new(p.seed ^ (cx as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (cy as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
                    let area = CELL_MM * CELL_MM;
                    let (ox, oy) = (cx as f32 * CELL_MM, cy as f32 * CELL_MM);
                    // a fiber's length: lognormal about the mean (sd 0.4 in ln)
                    let len = |rng: &mut Rng| (p.fiber_mm * (0.4 * rng.normal() - 0.08).exp()).clamp(0.1 * p.fiber_mm, 4.0 * p.fiber_mm);
                    // the mold thins fiber over its wires
                    let keep = |rng: &mut Rng, x: f32, y: f32| p.laid.is_none_or(|l| rng.f() >= l.deficit * p.wire(x, y));
                    // free fibers
                    let nf = poisson(&mut rng, per_mm2 * area * (1.0 - p.floc));
                    for _ in 0..nf {
                        let (x, y) = (ox + rng.f() * CELL_MM, oy + rng.f() * CELL_MM);
                        let (a, l) = (rng.f() * std::f32::consts::PI, len(&mut rng));
                        if keep(&mut rng, x, y) {
                            lay_fiber(x, y, a, l);
                        }
                    }
                    // flocs: about 200 fibers each, round their center
                    if p.floc > 0.0 {
                        let per_floc = 200.0;
                        let nflocs = poisson(&mut rng, per_mm2 * area * p.floc / per_floc);
                        for _ in 0..nflocs {
                            let (fx, fy) = (ox + rng.f() * CELL_MM, oy + rng.f() * CELL_MM);
                            let k = poisson(&mut rng, per_floc);
                            for _ in 0..k {
                                let (x, y) = (fx + p.floc_mm * rng.normal(), fy + p.floc_mm * rng.normal());
                                let (a, l) = (rng.f() * std::f32::consts::PI, len(&mut rng));
                                if keep(&mut rng, x, y) {
                                    lay_fiber(x, y, a, l);
                                }
                            }
                        }
                    }
                }
            }
            (b0, g)
        })
        .collect();
    for (b0, g) in parts {
        gram[b0 * w..b0 * w + g.len()].copy_from_slice(&g);
    }
    // mg/mm² → g/m²
    gram.iter_mut().for_each(|v| *v *= 1000.0);
    // the surface: pressed, so it follows the fiber count to a power below one
    let (beta, cal) = (p.grammage, p.caliper_um());
    let mut height: Vec<f32> = gram.par_iter().map(|&g| cal * ((g / beta).max(0.0).powf(p.press) - 1.0)).collect();
    // the felt's cells
    if let Some(f) = p.felt {
        height.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                let (xm, ym) = ((x0 + x) as f32 * px_mm + 0.5 * px_mm, (y0 + y) as f32 * px_mm + 0.5 * px_mm);
                *v += felt_um(xm, ym, &f, p.seed);
            }
        });
    }
    // calendering: the spots standing above the sheet's mean surface (0)
    // pressed toward it (in sheet terms, not the window's: a crop is the
    // same paper)
    if p.calender > 0.0 {
        height.par_iter_mut().for_each(|v| {
            if *v > 0.0 {
                *v -= p.calender * *v;
            }
        });
    }
    // the pores between the fiber tops (Dodson 2001; notes/research/
    // paper_mechanics_and_transport.md §2): many layers deep, a surface
    // pore's mean height is t·ε/(1−ε) at the local porosity, its spread about
    // its mean (near exponential, as measured). Pressing makes the denser
    // spots denser: the solid share follows the fiber count to the power
    // 1 − press. A pixel holds only a few dozen pores, so its mean scatters:
    // Gamma(k)/k, k = n/4.93 for the n = x²·τ²/π surface pores of a pixel x
    // mm across, τ = ln(1/ε)/ω at the local porosity ε and fiber width ω
    // (the research's §4: k ≈ 2–12 at 0.25–0.3 mm), so a finer render's
    // pixels scatter more, as they hold fewer pores.
    let micro: Vec<f32> = (0..n)
        .map(|i| {
            let solid = ((1.0 - p.porosity) * (gram[i] / beta).max(0.05).powf(1.0 - p.press)).clamp(0.05, 0.95);
            let mean = p.thick_um * (1.0 - solid) / solid;
            let (x, y) = ((x0 + i % w) as u64, (y0 + i / w) as u64);
            let mut r = Rng::new(p.seed ^ 0x5EED_9031 ^ x.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ y.wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
            let tau = (1.0 / (1.0 - solid)).ln() / (p.fiber_um * 1e-3);
            let pores = px_mm * px_mm * tau * tau / std::f32::consts::PI;
            // (at least one pore's worth)
            let k = (pores / 4.93).max(1.0);
            let g = gamma(&mut r, k) / k;
            mean * g
        })
        .collect();
    Sheet { height, micro, grammage: gram }
}

/// A draw of Gamma(k, 1), k ≥ 1, in constant time: Marsaglia and Tsang's
/// squeeze (ACM TOMS 26, 2000), its normal from Box–Muller.
fn gamma(r: &mut Rng, k: f32) -> f32 {
    let d = k - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let (u1, u2) = (r.f().max(1e-7), r.f());
        let x = (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos();
        let v = 1.0 + c * x;
        if v <= 0.0 {
            continue;
        }
        let v = v * v * v;
        let u = r.f().max(1e-7);
        if u < 1.0 - 0.0331 * x * x * x * x || u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln()) {
            return d * v;
        }
    }
}

/// The felt's imprint at a point (mm), µm: cells pressed in, their rims
/// standing (nearest-seed cells round the felt's knuckles, one seed per
/// cell-sized square of the felt).
fn felt_um(xm: f32, ym: f32, f: &Felt, seed: u64) -> f32 {
    let (u, v) = (xm / f.cell_mm, ym / f.cell_mm);
    let (iu, iv) = (u.floor() as i64, v.floor() as i64);
    let (mut d1, mut d2) = (f32::MAX, f32::MAX);
    for j in -2..=2 {
        for i in -2..=2 {
            let (cu, cv) = (iu + i, iv + j);
            let sx = cu as f32 + hash2(cu, cv, seed ^ 0xFE17);
            let sy = cv as f32 + hash2(cu, cv, seed ^ 0x0F31);
            let d = ((u - sx).powi(2) + (v - sy).powi(2)).sqrt();
            if d < d1 {
                d2 = d1;
                d1 = d;
            } else if d < d2 {
                d2 = d;
            }
        }
    }
    // distance to the cell wall, in cells; the rim is about a fifth of a cell
    let e = 0.5 * (d2 - d1);
    let rim = (1.0 - e / 0.2).max(0.0);
    f.depth_um * (rim * rim - 0.5)
}

#[cfg(test)]
mod tests {
    /// The pore-depth scatter's Gamma draw has the right mean and spread
    /// (Gamma(k, 1): mean k, variance k), whatever k, in constant time.
    #[test]
    fn the_gamma_draw_has_its_mean_and_spread() {
        let mut r = Rng::new(7);
        for k in [1.0f32, 3.0, 50.0] {
            let n = 40_000;
            let xs: Vec<f32> = (0..n).map(|_| gamma(&mut r, k)).collect();
            let mean = xs.iter().sum::<f32>() / n as f32;
            let var = xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f32>() / n as f32;
            assert!((mean / k - 1.0).abs() < 0.03 && (var / k - 1.0).abs() < 0.06, "k {k}: mean {mean}, variance {var}");
        }
    }

    use super::*;

    #[test]
    fn fiber_bounds_reject_only_empty_cropped_bands() {
        let mut rejected = 0;
        for (x0, y0, w, h) in [(0, 0, 17, 48), (7, 47, 11, 48), (20, 48, 13, 1), (31, 49, 1, 47), (16_777_215, 16_777_216, 5, 48)] {
            for px_mm in [0.1, 0.125, 0.25, 1.0] {
                assert!(!fiber_misses_window((x0 as f32 + 0.5) * px_mm, (y0 as f32 + 0.5) * px_mm, 2.0, (x0, y0, w, h), px_mm));
                let xs = [(-1.0f32).next_down(), -1.0, (-1.0f32).next_up(), 0.0, w as f32 - 1.0, w as f32, (w as f32).next_up()];
                let ys = [(-1.0f32).next_down(), -1.0, (-1.0f32).next_up(), 0.0, h as f32 - 1.0, h as f32, (h as f32).next_up()];
                for x in xs {
                    for y in ys {
                        let (cx, cy) = ((x0 as f32 + x + 0.5) * px_mm, (y0 as f32 + y + 0.5) * px_mm);
                        for len in [0.0, 0.001, 0.125, 2.0, 8.0] {
                            if !fiber_misses_window(cx, cy, len, (x0, y0, w, h), px_mm) {
                                continue;
                            }
                            rejected += 1;
                            let steps = ((len / (0.5 * px_mm)).ceil() as usize).max(1);
                            for ang in [0.0, std::f32::consts::FRAC_PI_6, std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_2, 3.0 * std::f32::consts::FRAC_PI_4, std::f32::consts::PI] {
                                let (dx, dy) = (ang.cos() * len, ang.sin() * len);
                                for s in 0..steps {
                                    let t = (s as f32 + 0.5) / steps as f32 - 0.5;
                                    let (xm, ym) = (cx + dx * t, cy + dy * t);
                                    let (ix, iy) = ((xm / px_mm - 0.5 - x0 as f32).floor() as i64, (ym / px_mm - 0.5 - y0 as f32).floor() as i64);
                                    for oy in 0..=1 {
                                        for ox in 0..=1 {
                                            let (xx, yy) = (ix + ox, iy + oy);
                                            assert!(xx < 0 || xx >= w as i64 || yy < 0 || yy >= h as i64, "rejected fiber touches {xx},{yy} in {x0},{y0},{w},{h} at {px_mm} mm: center {cx},{cy}, length {len}, angle {ang}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(rejected > 0);
        for px_mm in [0.0, -0.25, f32::INFINITY, f32::NEG_INFINITY, f32::NAN] {
            assert!(!fiber_misses_window(-10.0, -10.0, 2.0, (0, 0, 17, 48), px_mm));
        }
    }

    fn stats(v: &[f32]) -> (f64, f64) {
        let n = v.len() as f64;
        let m = v.iter().map(|&x| x as f64).sum::<f64>() / n;
        let var = v.iter().map(|&x| (x as f64 - m).powi(2)).sum::<f64>() / n;
        (m, var.sqrt())
    }

    /// The fibers laid add up to the sheet's grammage, and the point CV
    /// at a scale below a fiber length is of the order the network theory
    /// gives (a few tens of percent at 0.1 mm, less at 0.5 mm).
    #[test]
    fn the_fibers_make_the_grammage() {
        let p = Paper { floc: 0.0, felt: None, calender: 0.0, ..Paper::drawing(3) };
        let s = lay(&p, 40, 40, 200, 200, 0.1);
        let (m, sd) = stats(&s.grammage);
        assert!((m - 160.0).abs() < 8.0, "mean grammage {m}");
        let cv = sd / m;
        assert!(cv > 0.05 && cv < 0.6, "cv at 0.1 mm {cv}");
        let s5 = lay(&p, 8, 8, 40, 40, 0.5);
        let (m5, sd5) = stats(&s5.grammage);
        assert!(sd5 / m5 < cv, "cv falls with zone size: {} vs {}", sd5 / m5, cv);
    }

    /// The same sheet at two pixel sizes has the same grammage at the
    /// coarser scale (it is laid in mm, not in pixels), and a crop is the
    /// same paper as the whole.
    #[test]
    fn the_sheet_is_laid_in_millimeters() {
        let p = Paper::drawing(5);
        let whole = lay(&p, 0, 0, 60, 60, 0.25);
        let part = lay(&p, 20, 10, 20, 30, 0.25);
        for y in 0..30 {
            for x in 0..20 {
                let (a, b) = (whole.height[(y + 10) * 60 + x + 20], part.height[y * 20 + x]);
                let (ga, gb) = (whole.grammage[(y + 10) * 60 + x + 20], part.grammage[y * 20 + x]);
                assert!((ga - gb).abs() < 1e-3, "grammage at {x},{y}: {ga} vs {gb}");
                assert!((a - b).abs() < 1e-3, "height at {x},{y}: {a} vs {b}");
            }
        }
    }

    /// Flocculated and felted sheets are rougher at the mm scale than a
    /// random, calendered one.
    #[test]
    fn flocs_and_felt_roughen_the_sheet() {
        let smooth = Paper { floc: 0.0, felt: None, calender: 0.8, ..Paper::drawing(9) };
        let rough = Paper { floc: 0.7, calender: 0.0, ..Paper::drawing(9) };
        let (_, a) = stats(&lay(&smooth, 0, 0, 120, 120, 0.25).height);
        let (_, b) = stats(&lay(&rough, 0, 0, 120, 120, 0.25).height);
        assert!(b > 1.5 * a, "rough {b} smooth {a}");
    }

    /// Each named sheet is the make its research note documents
    /// (notes/research/cassatt_papers.md; the guide's table), with a drawing
    /// paper's absorbency and give, and no drum.
    #[test]
    fn the_presets_are_their_documented_dials() {
        let felt = |cell_mm, depth_um| Some(Felt { cell_mm, depth_um });
        let wove = felt(0.7, 14.0);
        // name: grammage, fiber, width, thickness, coarseness, porosity, floc, floc size, press, calender, felt, laid, tone
        type Row = (&'static str, [f32; 10], Option<Felt>, Option<Laid>, &'static [(&'static str, f32)]);
        let w = [130.0, 1.6, 25.0, 6.0, 0.16, 0.5, 0.5, 1.5, 0.65, 0.25];
        let rows: [Row; 8] = [
            ("blue wove", w, wove, None, &[("lead white", 1.0), ("ultramarine blue", 1.0), ("bone black", 0.45)]),
            ("gray-blue wove", w, wove, None, &[("lead white", 1.0), ("ultramarine blue", 1.0), ("viridian", 1.5), ("red earth", 1.5)]),
            ("tan wove", w, wove, None, &[("yellow ochre", 0.2), ("bitumen", 1.0), ("vine black", 0.23)]),
            ("buff wove", w, wove, None, &[("lead white", 1.0), ("yellow ochre", 2.2), ("raw umber", 3.0), ("Prussian blue", 0.02)]),
            ("green wove", w, wove, None, &[("lead white", 1.0), ("yellow ochre", 0.65), ("viridian", 0.45), ("bone black", 0.45)]),
            ("brown wove", [150.0, 1.4, 25.0, 6.0, 0.2, 0.5, 0.6, 1.5, 0.65, 0.25], felt(0.8, 18.0), None, &[("raw sienna", 1.0), ("bitumen", 3.0)]),
            ("cream laid", [110.0, 2.2, 25.0, 6.0, 0.16, 0.5, 0.5, 1.5, 0.65, 0.2], felt(0.8, 15.0), Some(Laid { per_cm: 9.0, chain_mm: 25.0, deficit: 0.15 }), &[("lead white", 1.0), ("yellow ochre", 0.15), ("raw sienna", 0.02), ("green earth", 3.0)]),
            ("japan paper", [45.0, 10.0, 30.0, 5.0, 0.1, 0.6, 0.15, 2.0, 0.7, 0.6], None, None, &[("lead white", 1.0), ("yellow ochre", 0.1), ("raw sienna", 0.1), ("viridian", 0.02)]),
        ];
        assert_eq!(PRESETS.len(), rows.len());
        for (name, v, felt, laid, tone) in rows {
            let pr = Paper::preset(name).unwrap_or_else(|| panic!("{name}"));
            assert_eq!(pr.tone, tone, "{name}");
            let p = pr.paper(7);
            let got = [p.grammage, p.fiber_mm, p.fiber_um, p.thick_um, p.coarseness, p.porosity, p.floc, p.floc_mm, p.press, p.calender];
            assert_eq!(got, v, "{name}");
            let d = Paper::drawing(7);
            assert!(p.felt == felt && p.laid == laid && p.absorbent == d.absorbent && p.z_mpa == d.z_mpa && p.drum.is_none() && p.seed == 7, "{name}");
            // every tone's tube is in the catalog
            let every = crate::palette::Palette::named_box("every").unwrap();
            assert!(tone.iter().all(|(t, _)| every.tubes.iter().any(|e| e.name == *t)), "{name}");
        }
        assert!(Paper::preset("Blue wove").is_none() && Paper::preset("").is_none());
    }

    /// A mount's backing gives in series with the sheet: board least,
    /// canvas more, a strainer as canvas over its bars and much more in the
    /// bay. The bay firms up toward a bar over a few bending lengths and
    /// gives the same further in, whatever the bay's size; a crossbar is a
    /// bar.
    #[test]
    fn mounts_order_the_give() {
        let (w, h) = (600.0, 500.0);
        let mut sheets: Vec<Paper> = PRESETS.iter().map(|p| p.paper(3)).collect();
        sheets.push(Paper::drawing(3));
        sheets.push(Paper { z_mpa: 2.0, ..Paper::drawing(3) });
        sheets.push(Paper { z_mpa: 500.0, ..Paper::drawing(3) });
        let strainer = Mount::named("strainer").unwrap();
        let bar = STRAINER_BAR_MM;
        for p in sheets {
            let k = |q: Paper, x: f32, y: f32| q.give_at(x, y, w, h);
            let mid = (0.5 * w, 0.5 * h);
            let (free, board, canvas) = (k(p, mid.0, mid.1), k(p.mounted(Mount::Board), mid.0, mid.1), k(p.mounted(Mount::Canvas), mid.0, mid.1));
            let s = p.mounted(strainer);
            assert!(free > board && board > canvas && canvas > k(s, mid.0, mid.1), "{p:?}");
            assert_eq!(p.mounted(Mount::Board).drum, None);
            assert_eq!(s.z_mpa, p.mounted(Mount::Canvas).z_mpa);
            // over the bars, the canvas's give
            for (x, y) in [(1.0, 250.0), (bar - 1.0, 100.0), (300.0, 499.0), (599.0, 400.0)] {
                assert_eq!(k(s, x, y), k(p.mounted(Mount::Canvas), x, y));
            }
            // inward from the bar, never firmer; as firm as over it just
            // inside its edge, then softer at every step over the next few
            // bending lengths past a patch's radius
            let (l, a) = (s.drum.unwrap().bend_mm, s.drum.unwrap().patch_mm);
            assert_eq!(k(s, bar + 0.1, mid.1), k(p.mounted(Mount::Canvas), mid.0, mid.1));
            let mut last = k(s, bar, mid.1);
            for f in [0.0, 0.5, 1.0, 1.5, 2.0, 3.0] {
                let now = k(s, bar + a + f * l, mid.1);
                assert!(now <= last && (f < 0.5 || now < last), "a + {f}ℓ: {now} vs {last}");
                last = now;
            }
            // far in, the bay's own give, the same in a small bay
            let far = k(s, bar + a + 12.0 * l, mid.1);
            assert!((far / k(s, mid.0, mid.1) - 1.0).abs() < 1e-4, "{far}");
            assert_eq!(k(s, mid.0, mid.1), s.give_at(150.0, 150.0, 300.0, 300.0));
            // the drum is the bay's: no unmounted sheet has one
            assert_eq!(k(p, bar, mid.1), free);
            // a crossbar across the longer side is a bar
            let c = p.mounted(Mount::Strainer { bar_mm: bar, cross: 1 });
            assert_eq!(k(c, mid.0, mid.1), k(p.mounted(Mount::Canvas), mid.0, mid.1));
            assert!(k(c, mid.0 + 0.5 * bar + a + 0.5 * l, mid.1) > k(c, mid.0 + 0.5 * bar + a + 3.0 * l, mid.1));
            assert_eq!(k(c, 150.0, mid.1), k(s, 150.0, mid.1));
        }
        assert_eq!(Mount::named("strainer"), Some(Mount::Strainer { bar_mm: STRAINER_BAR_MM, cross: 0 }));
        assert_eq!(Mount::named("stretcher"), None);
    }

    /// The mounts' numbers, from the measured inputs (notes/research/
    /// cassatt_papers.md §1; checked there against a numerical Hankel
    /// transform): the reference patch, the cloth's and the board's give,
    /// the drawing paper's bending stiffness and its bay's give; the
    /// Bessel functions against tables; the drum's sink at its limits.
    #[test]
    fn the_mounts_are_their_derived_numbers() {
        let near = |a: f64, b: f64, tol: f64| (a / b - 1.0).abs() < tol;
        for (got, want) in [
            (bessel_i0(1.0), 1.266065878), (bessel_i1(1.0), 0.565159104), (bessel_k0(1.0), 0.421024438), (bessel_k1(1.0), 0.601907230),
            (bessel_i0(5.0), 27.23987182), (bessel_i1(5.0), 24.33564214), (bessel_k0(3.0), 0.034739504), (bessel_k1(3.0), 0.040156431),
            (bessel_k0(0.1), 2.427069025), (bessel_k1(0.1), 9.853844780),
        ] {
            assert!(near(got, want, 1e-6), "{got} vs {want}");
        }
        assert!(near(ref_patch_mm() as f64, 6.855, 1e-3));
        assert!(near(cloth_mpa_per_um() as f64, 5.541e-4, 2e-3), "{}", cloth_mpa_per_um());
        assert!(near(board_mpa_per_um() as f64, 0.015009, 2e-3), "{}", board_mpa_per_um());
        let p = Paper::drawing(1);
        assert!(near(p.bending_nmm() as f64, 6.686, 1e-3), "{}", p.bending_nmm());
        let d = p.mounted(Mount::named("strainer").unwrap()).drum.unwrap();
        assert!(near(d.bend_mm as f64, 7.171, 1e-3) && near(d.bay_mpa_per_um as f64, 3.226e-5, 2e-3), "{d:?}");
        assert_eq!(d.bar_mm, 50.0);
        // a membrane's ½; a plate's (β²/4)(ln(2/β) − γ + 3/8)
        assert!(near(drum_sink(40.0), 0.5, 0.03));
        let b: f64 = 0.05;
        assert!(near(drum_sink(b), b * b / 4.0 * ((2.0 / b).ln() - 0.5772157 + 0.375), 0.01), "{}", drum_sink(b));
    }

    /// Pores: a pixel's mean depth scatters (a few dozen pores in it), the
    /// denser spots shallower, the sheet's mean about the fiber thickness.
    #[test]
    fn pores_vary_with_the_sheet() {
        let c = crate::Canvas::new_window(600, 1.0, [0.55; 3], None).with_size_mm(150.0).with_paper(Paper::drawing(4));
        let m = &c.micro;
        let mean = m.iter().sum::<f32>() / m.len() as f32;
        let sd = (m.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / m.len() as f32).sqrt();
        assert!(mean > 4.0 && mean < 9.0, "mean pore depth {mean}");
        assert!(sd / mean > 0.2, "they vary: cv {}", sd / mean);
    }
}

#[cfg(test)]
mod measure {
    use super::*;
    #[test]
    #[ignore]
    fn print_roughness() {
        for (name, p) in [("drawing", Paper::drawing(3)), ("no felt", Paper { felt: None, ..Paper::drawing(3) })] {
            for px in [0.125f32, 0.26] {
                let n = (40.0 / px) as usize;
                let s = lay(&p, 0, 0, n, n, px);
                let m = s.height.iter().sum::<f32>() / s.height.len() as f32;
                let ra = s.height.iter().map(|v| (v - m).abs()).sum::<f32>() / s.height.len() as f32;
                let (lo, hi) = s.height.iter().fold((f32::MAX, f32::MIN), |(a, b), &v| (a.min(v), b.max(v)));
                let gm = s.grammage.iter().sum::<f32>() / s.grammage.len() as f32;
                let gcv = (s.grammage.iter().map(|g| (g - gm).powi(2)).sum::<f32>() / s.grammage.len() as f32).sqrt() / gm;
                println!("{name} px {px}: Ra {ra:.1} µm, range {lo:.0}..{hi:.0}, grammage cv {gcv:.3}");
            }
        }
    }
}
