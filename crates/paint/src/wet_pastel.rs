//! Pastel with water: a stick dipped in water, a wet brush drawn through
//! laid pastel, ground pastel laid as a paste or wash with a brush. Source:
//! notes/research/wet_pastel.md (its sections cited as §n; [M] measured,
//! [D] derived here from measured values, [P] a practitioner's or museum's
//! account, [E] an estimate, each with the reason it is one), and the
//! estimates audit (notes/estimates-audit-2.md, W1–W23).
//!
//! - **Water in the pores** (`Cell::water`, µm in the pastel's pores and
//!   standing on the paper) replaces the air between the particles. Their
//!   scattering falls with the index contrast against water (Mie,
//!   `dry::scatter_in`, at each cell's particle index `Cell::index`, the
//!   pigments' and their whiting's, weighted by scattering). The darkening
//!   rises with the share of the pores filled as measured for soils,
//!   exponentially, near its full value by half full (§2). The dry colors
//!   are kept, so it lightens again as the water goes.
//! - **Where the water goes** (§4). The pastel's pores draw harder than the
//!   paper's, so they fill first and empty last. Water standing over them
//!   sinks into the sheet as its sizing lets it (a sized sheet's measured
//!   Cobb uptake, √t; an unsized one at its pores' measured Washburn rate),
//!   until the share of the sheet's pores water reaches is full
//!   (`Paper::absorbent`). Evaporation is a still-air boundary layer's mass
//!   transfer of the vapour-pressure deficit in the studio
//!   (`crate::climate::STUDIO`), the wet surface cooled to its energy
//!   balance (`evaporation_um_min`): first what stands, then what the sheet
//!   took in (drawn back up through the pastel's pores, which stay full),
//!   then the pores themselves at a falling rate.
//! - **The gum** (§1, §3): the stick's gum dissolves in the water and sets
//!   again at the contacts as it dries. Water that sank took its share of
//!   the gum with it (`Cell::gum`). The bridges the rest makes hold as their
//!   volume^(2/3): the pastel is bound where they hold against a finger
//!   (`binds`), and left loose where too little gum is left.
//! - **A wet stick** (`Stick::water`). Its tip is a chalk soil: below its
//!   plastic limit it is as hard as dry; in its plastic range it is as soft
//!   as a remoulded soil's strength at its liquidity index
//!   (`wet_hardness`) and smears instead of crumbling; past its liquid
//!   limit it is a slurry, pressed into the tooth as a paste rather than
//!   worn off (`paste_share`). Its water spreads up the dry stick by
//!   capillarity (`tip_water`).
//! - **A wet brush** (`Canvas::wash_pastel`) holds water in its belly
//!   (`Held::capacity_mm3`), lays it as it lays paint, picks up the wetted
//!   loose pastel and lays it further on as it lays and picks up wet paint
//!   (the tool's `lay`, `pickup` and `run`).

use crate::canvas::Canvas;
use crate::climate::STUDIO;
use crate::color::Rgb;
use crate::graphite::Cell;
use crate::pastel::{PARTICLE_UM, Section, Stick};

/// A pastel stick's porosity, and so its deposit's (a deposit is counted in
/// the stick's volume it wore off: `Cell::loose`): 0.55 [D, bounds
/// 0.50–0.61]. A Sennelier stick listed at 9.07 g, at its listed 2.65 ×
/// 0.45 in, is ρ 1.32 g/cm³: 0.50 against solids of 2.65 (calcite 2.71,
/// kaolinite 2.6), 0.54 against 2.9; at the nominal ½ × 2½ in, 0.57–0.61
/// (wet_pastel.md §1). The 9.07 g is a listing weight shared by several
/// colors, so it may include the wrapper.
pub const POROSITY: f32 = 0.55;
/// The particles' size, µm: the Sauter (surface-weighted) mean d₃₂ of the
/// historic filler, Champagne/Meudon whiting (d50 2.4–2.7 µm, D97 13–15
/// µm, Art-Chimie; d50 3, d98 25, Omya): a volume lognormal through d50 and
/// D97 has σ_ln 0.87–0.94 and d₃₂ 1.64–1.75 µm [D]. Capillary strength,
/// the pores' hydraulic radius and coverage per volume all go with d₃₂.
pub const PARTICLE_D_UM: f32 = 1.7;
/// The stick's dry gum, a share of its mass: recipes dough 1 g of powder
/// with 0.3–0.5 mL of a 1:30 tragacanth solution, 1–2 % [D]
/// (pastel_stick_tribology.md §1; wet_pastel.md §1). Bounds 0.3–3 %: no
/// assay of a stick was found (a bench measurement: ash or TGA loss).
pub const GUM_SHARE: f32 = 0.015;
/// Densities, g/cm³: the solids (calcite 2.71, kaolinite 2.6 [M]) and dry
/// gum arabic (1.35–1.49 [M], wet_pastel.md §1).
const SOLID_DENSITY: f32 = 2.65;
const GUM_DENSITY: f32 = 1.4;
/// The refractive index of dry gum (gum arabic films 1.51–1.54 [M],
/// dry_pigment_optics.md §1) and of water.
const GUM_INDEX: f32 = 1.52;
const WATER_INDEX: f32 = 1.333;
/// A particle index where none was recorded: whiting's (ground calcite,
/// 1.59; ω 1.658 and ε 1.486 average 1.60), the bulk of a pastel
/// (`dry::WHITING_INDEX`).
pub const DEFAULT_INDEX: f32 = crate::dry::WHITING_INDEX;
/// The pores' water, which has dissolved the stick's gum (3.3 % w/v at
/// saturation, `gum_in_water`): its surface tension, N/m, that of gum
/// arabic solutions above their aggregation concentration, 46.9 mN/m at
/// 0.5 % (Huang, Kakuda & Cui 2001) to 53.7–57.4 (Al-Assaf et al.) [M];
/// gum arabic is surface-active.
const GAMMA: f32 = 0.050;
/// Water's viscosity at 20 °C, Pa·s [M, handbook], and gum arabic's
/// intrinsic viscosity, mL/g (18, Gómez-Díaz et al. 2008; A. senegal
/// 14–23 [M]), with a Huggins constant of 0.4 [E: 0.3–0.6 for a compact
/// branched polymer in a good solvent; none measured for the gum was
/// found]: the pores' water at 1.74 mPa·s (`pore_viscosity`).
const ETA_WATER: f32 = 1.002e-3;
const GUM_INTRINSIC_ML_G: f32 = 18.0;
const HUGGINS: f32 = 0.4;
/// Convective heat transfer from a vertical surface (a sheet on an easel)
/// to still room air, W/m²K: 2.5, ISO 6946's for horizontal heat flow
/// [M], and Churchill–Chu's 2.5–2.7 for a 0.3–0.5 m sheet 3.7 K below the
/// air [D]; and the water's emissivity, 0.95 [M], for the radiation it
/// takes from the room.
const H_CONV: f64 = 2.5;
const EMISSIVITY: f64 = 0.95;
/// Whatman Grade 1 filter paper's measured Washburn coefficient, m/√s:
/// 0.38 cm/s^½ (D = K² = 14 mm²/s; *Sci. Rep.* 2019, s41598-019-52202-9)
/// [M], the rate of an unsized sheet (`Paper::cobb60` None).
const UNSIZED_K: f32 = 0.38e-2;
/// How the tip is dipped (gestures, not physics): 1 mm into the water, for
/// half a second (`dip_mm`).
pub const DIP_DEPTH_MM: f32 = 1.0;
pub const DIP_S: f32 = 0.5;
/// Remoulded chalk's Atterberg limits, water by mass of solids: plastic
/// 0.244, liquid 0.310 (remoulded Margate chalk, Bialowas, Diambra & Nash
/// 2016; crushed chalk 0.224 / 0.298, Bialowas, Nash & Diambra 2018) [M].
const PLASTIC_LIMIT: f32 = 0.244;
const LIQUID_LIMIT: f32 = 0.310;
/// A remoulded soil's undrained strength at its liquid limit, kPa (1.7,
/// 0.7–2.65 measured; Wroth & Wood 1978), and its rise to the plastic
/// limit (×35: Vardanega & Haigh 2014, 641 fall-cone tests on 101 soils,
/// in place of Wroth & Wood's ×100, which overpredicts) [M]: s_u = 1.7 ·
/// 35^(1 − LI) kPa.
const SU_LIQUID_KPA: f32 = 1.7;
const SU_RANGE: f32 = 35.0;
/// Prandtl's bearing factor for an undrained (purely cohesive) solid, 2 + π:
/// the indentation hardness of a plastic soil, H = 5.14 s_u [D].
const N_C: f32 = 2.0 + std::f32::consts::PI;
/// A drying paste packs from the stick's porosity to near random close
/// packing (0.36): gum arabic disperses it sterically, and the drying
/// menisci pull at 2γ/r ≈ 100 kPa, above a dispersed bed's yield; 0.40
/// [D, audit W15]. Its volume shrinks by (1 − ε)/(1 − 0.40), its coverage
/// stays (`dried_share`).
const PASTE_DRIED_POROSITY: f32 = 0.40;
/// The dried gum's bridges against the finger (`binds`): a dry stick's
/// friction angle, remoulded chalk's 29–34° (Clayton 1978; Razoaki 2000;
/// Bundy 2013) [M], 31.5°; the finger's friction on paper, 0.5 (a dry
/// finger pad under 0–10 N; 0.2–1.75 across surfaces) [M]; pressed at the
/// knee of its pad's growth (`pastel::FINGER_KNEE_N`, 2 N, a firm press),
/// and a soft stick's hardness (soft 0.75, `Stick::mechanics`).
const PHI_DEG: f32 = 31.5;
const FINGER_MU: f32 = 0.5;
const SOFT_STICK: f32 = 0.75;
/// Soil reflectance falls exponentially with water and fits one curve in
/// the degree of saturation (Lobell & Asner 2002); in the visible it is
/// near its wet value by about 20 % water by volume, S ≈ 0.4 at ε 0.45–0.5
/// [M]. So the darkening's share at saturation S is (1 − e^(−S/S₀))/(1 −
/// e^(−1/S₀)), S₀ = 0.2 [E: set by that saturation point, as their fitted
/// constant wasn't retrieved; bounds 0.1–0.3] (`wet_share`).
const WET_S0: f32 = 0.2;
/// The walk's step along a path, pixels (`Canvas::walk`).
const STEP_PX: f32 = 0.7;

/// A particle index as a cell holds it (`Cell::index`), and back.
pub(crate) fn to_index(n: f32) -> u16 {
    ((n - 1.0).clamp(0.0, 6.0) * 10_000.0).round() as u16 + 1
}
pub(crate) fn from_index(k: u16) -> f32 {
    if k == 0 { DEFAULT_INDEX } else { 1.0 + (k - 1) as f32 / 10_000.0 }
}
/// The share of its gum the pastel holds (`Cell::gum`, 0 never wet), and back.
pub(crate) fn to_gum(g: f32) -> u16 {
    (g.clamp(0.0, 1.0) * 65_534.0).round() as u16 + 1
}
pub(crate) fn from_gum(k: u16) -> f32 {
    if k == 0 { 1.0 } else { (k - 1) as f32 / 65_534.0 }
}

/// Bruggeman's effective medium of air and a share `f` of a filler of
/// index `n` (the pore phase), as an index.
pub fn pore_index(f: f32, n: f32) -> f32 {
    let (e1, e2, f) = (1.0f32, n * n, f.clamp(0.0, 1.0));
    let b = (3.0 * (1.0 - f) - 1.0) * e1 + (3.0 * f - 1.0) * e2;
    ((b + (b * b + 8.0 * e1 * e2).sqrt()) / 4.0).sqrt()
}

/// The gum in a stick's volume, g/cm³: its share of the solids' mass.
fn gum_g_cm3() -> f32 {
    GUM_SHARE / (1.0 - GUM_SHARE) * (1.0 - POROSITY) * SOLID_DENSITY
}

/// The share of a stick's pores its dry gum fills (about 0.024).
pub fn gum_pore_share() -> f32 {
    gum_g_cm3() / (GUM_DENSITY * POROSITY)
}

/// The gum dissolved in saturated pores, g/mL (about 0.033).
pub fn gum_in_water() -> f32 {
    gum_g_cm3() / POROSITY
}

/// The pores' gum solution's viscosity, Pa·s: Huggins, η_w (1 + [η]c +
/// k_H ([η]c)²) (about 1.74 mPa·s).
pub fn pore_viscosity() -> f32 {
    let x = GUM_INTRINSIC_ML_G * gum_in_water();
    ETA_WATER * (1.0 + x + HUGGINS * x * x)
}

/// `r` (linear RGB, a reflectance at infinite thickness) with its
/// scattering times `s` (Kubelka–Munk: K/S over `s`).
pub fn km_scatter(r: Rgb, s: f32) -> Rgb {
    if s >= 1.0 {
        return r;
    }
    r.map(|r| {
        let r = r.clamp(1e-4, 0.9999);
        let ks = (1.0 - r) * (1.0 - r) / (2.0 * r) / s.max(1e-3);
        1.0 + ks - (ks * ks + 2.0 * ks).sqrt()
    })
}

/// The share of its full darkening pastel whose pores are filled to `s`
/// shows (`WET_S0`).
pub fn wet_share(s: f32) -> f32 {
    let s = s.clamp(0.0, 1.0);
    (1.0 - (-s / WET_S0).exp()) / (1.0 - (-1.0 / WET_S0).exp())
}

/// How pastel of dry reflectance `r` in cell `c` looks: while wet,
/// darkened by the water in its pores (Mie at the cell's particle index
/// against water, by the share of the pores filled, `wet_share`); once dry
/// again, by the gum it still holds in its pores (Bruggeman of air and
/// dry gum, then Mie).
pub(crate) fn look(c: &Cell, r: Rgb) -> Rgb {
    let n = from_index(c.index);
    if c.water > 0.0 {
        let pores = POROSITY * (c.loose + c.bound);
        let s = if pores > 0.0 { (c.water / pores).min(1.0) } else { 1.0 };
        let wet = km_scatter(r, crate::dry::scatter_in(n, WATER_INDEX));
        let f = wet_share(s);
        [0, 1, 2].map(|k| r[k] + (wet[k] - r[k]) * f)
    } else {
        km_scatter(r, crate::dry::scatter_in(n, pore_index(from_gum(c.gum) * gum_pore_share(), GUM_INDEX)))
    }
}

/// Water's saturated vapour pressure, Pa, at `t` °C (Buck 1981).
fn p_sat(t: f64) -> f64 {
    611.21 * ((18.678 - t / 234.5) * (t / (257.14 + t))).exp()
}

/// Water vapour's diffusivity in air at `t` °C, m²/s (Massman 1998).
fn d_vapour(t: f64) -> f64 {
    2.178e-5 * ((t + 273.15) / 273.15).powf(1.81)
}

/// The mass transfer coefficient of the still-air boundary layer, m/s:
/// `H_CONV` by the Chilton–Colburn analogy, h/(ρ c_p) · Le^(−2/3) (air at
/// 20 °C: ρ c_p 1211 J/m³K, thermal diffusivity 2.15e-5 m²/s).
fn h_mass() -> f64 {
    let le = 2.15e-5 / d_vapour(STUDIO.air_c);
    H_CONV / (1.204 * 1006.0) * le.powf(-2.0 / 3.0)
}

/// Water evaporating from a wet surface in the studio, µm a minute: the
/// boundary layer's mass transfer of the vapour density deficit between
/// the surface (saturated at its own temperature) and the room's air, the
/// surface cooled until the latent heat it loses is what convection and
/// the room's radiation bring it (about 16 °C; 0.71 µm/min) [D].
pub fn evaporation_um_min() -> f32 {
    const M_R: f64 = 0.018_015 / 8.314_462;
    let ta = STUDIO.air_c;
    let tk = |t: f64| t + 273.15;
    let rho = |t: f64| p_sat(t) * M_R / tk(t);
    let air = STUDIO.rh * rho(ta);
    let hm = h_mass();
    let hr = 4.0 * EMISSIVITY * 5.670_374e-8 * tk(ta).powi(3);
    let (mut lo, mut hi) = (ta - 25.0, ta);
    for _ in 0..50 {
        let ts = 0.5 * (lo + hi);
        if 2.454e6 * hm * (rho(ts) - air) > (H_CONV + hr) * (ta - ts) { hi = ts } else { lo = ts }
    }
    let ts = 0.5 * (lo + hi);
    // kg/m²s of water over 1000 kg/m³, in µm a minute
    (hm * (rho(ts) - air).max(0.0) * 1e3 * 60.0) as f32
}

/// The falling rate's coefficient, per µm of water gone from the pores:
/// once the standing water is gone the front recedes into the pastel, and
/// the vapour crosses the dry pores (diffusivity D ε^1.5, Bruggeman's
/// tortuosity) besides the boundary layer: flux = E/(1 + a·u), u the water
/// missing from the pores (µm).
fn falling() -> f32 {
    let d_eff = d_vapour(STUDIO.air_c) * (POROSITY as f64).powf(1.5);
    (h_mass() * 1e-6 / (POROSITY as f64 * d_eff)) as f32
}

/// Water (µm) after `dt` minutes of evaporating, `pores` µm of it held in
/// the pastel's pores: what stands over them at `e` µm a minute, then
/// the pores at the falling rate (`falling`, `a`).
fn water_after(w: f32, pores: f32, dt: f32, e: f32, a: f32) -> f32 {
    if w <= 0.0 || e <= 0.0 {
        return w.max(0.0);
    }
    let over = (w - pores).max(0.0);
    if over >= e * dt {
        return w - e * dt;
    }
    let dt = dt - over / e;
    // u + a u²/2 = E t, from the water already missing from the pores
    let u0 = (pores - w).max(0.0);
    let t = (u0 + 0.5 * a * u0 * u0) / e + dt;
    let u = 2.0 * e * t / (1.0 + (1.0 + 2.0 * a * e * t).sqrt());
    let left = (pores.min(w) - (u - u0)).max(0.0);
    if left < 1e-6 { 0.0 } else { left }
}

/// Minutes until `w` µm of water is gone (as `water_after`).
fn time_to_dry(w: f32, pores: f32, e: f32, a: f32) -> f32 {
    if w <= 0.0 {
        return 0.0;
    }
    let over = (w - pores).max(0.0);
    let p = pores.min(w);
    let u0 = (pores - w).max(0.0);
    let t = |u: f32| (u + 0.5 * a * u * u) / e;
    over / e + t(u0 + p) - t(u0)
}

/// How fast a sheet takes water in, µm (g/m²) over √s: a sized sheet its
/// Cobb60 over √60 s (uptake C₆₀ √(t/60 s): the size's slow wetting, which
/// Cobb measures); an unsized one its porosity times Whatman's measured
/// Washburn coefficient (`UNSIZED_K`).
pub fn uptake_rate(p: &crate::paper::Paper) -> f32 {
    match p.cobb60 {
        Some(c) => c.max(0.0) / 60f32.sqrt(),
        None => p.porosity.clamp(0.05, 0.95) * UNSIZED_K * 1e6,
    }
}

/// The water a sheet's pores can take, µm: the share of its pore volume a
/// liquid reaches (`absorbent`, as for oil: `Canvas::with_paper`) times its
/// porosity and caliper (120 µm for the default sheet). Under a gritted
/// ground's acrylic coat, none: the dried acrylic is a waterproof film over
/// the fibers, so water stands in the coat's tooth with the pastel and only
/// evaporates, as oil does there (`Paper::oil_capacity`). A glue coat is lean
/// and porous (7 % solids by volume, `paper::Binder::Glue`), so water reaches
/// the sheet through it at the sheet's own sizing, as oil does
/// (wet_pastel.md §4.1).
pub fn sheet_capacity_um(p: &crate::paper::Paper) -> f32 {
    if p.grit.is_some_and(|g| g.binder == crate::paper::Binder::Acrylic) {
        return 0.0;
    }
    p.absorbent.clamp(0.0, 1.0) * p.porosity * p.caliper_um()
}

/// The water a sheet holding `held` µm takes in over `dt` minutes from
/// what stands on it: √t uptake (`uptake_rate`) from where it is, to its
/// capacity (`sheet_capacity_um`).
pub fn sink_um(p: &crate::paper::Paper, held: f32, dt: f32) -> f32 {
    sink_by(held, (uptake_rate(p), sheet_capacity_um(p)), dt)
}

fn sink_by(held: f32, (rate, cap): (f32, f32), dt: f32) -> f32 {
    if dt <= 0.0 || held >= cap {
        return 0.0;
    }
    ((held * held + rate * rate * 60.0 * dt).sqrt().min(cap) - held).max(0.0)
}

/// `dt` minutes for a cell's water (`w` µm in and over its pastel, whose
/// pores hold `pores`) and the water the sheet took in under it (`u`), the
/// pastel holding `g` of its gum: the sheet's water rises into pores that
/// aren't full (they draw harder); what stands over the pores sinks into
/// the sheet (`sheet`: its rate and capacity) as it evaporates, taking
/// its share of the dissolved gum; then the sheet's water evaporates
/// through the full pores, then the pores at the falling rate. Returns the
/// water, the sheet's water and the gum held.
#[allow(clippy::too_many_arguments)]
fn settle(w: f32, u: f32, pores: f32, g: f32, dt: f32, e: f32, a: f32, sheet: Option<(f32, f32)>) -> (f32, f32, f32) {
    let (mut w, mut u, mut g, mut t) = (w.max(0.0), u.max(0.0), g, dt.max(0.0));
    let up = (pores - w).max(0.0).min(u);
    w += up;
    u -= up;
    let s0 = (w - pores).max(0.0);
    if s0 > 0.0 && t > 0.0 {
        let sh = sheet.unwrap_or((0.0, 0.0));
        let left = |t: f32| s0 - e * t - sink_by(u, sh, t);
        let t1 = if left(t) > 0.0 {
            t
        } else {
            let (mut lo, mut hi) = (0.0f32, t);
            for _ in 0..48 {
                let m = 0.5 * (lo + hi);
                if left(m) > 0.0 { lo = m } else { hi = m }
            }
            hi
        };
        let sunk = sink_by(u, sh, t1).min(s0);
        let evap = (e * t1).min(s0 - sunk);
        if sunk > 0.0 {
            // the gum leaves with what sinks, at the solution's strength as
            // evaporation concentrates it: (V₁/V₀)^(sunk/(sunk + evaporated))
            let v1 = (w - sunk - evap).max(1e-9);
            g *= (v1 / w).powf(sunk / (sunk + evap));
        }
        w = if left(t1) > 0.0 { w - sunk - evap } else { pores.min(w) };
        u += sunk;
        t -= t1;
    }
    if t > 0.0 && u > 0.0 && e > 0.0 {
        let take = u.min(e * t);
        u -= take;
        t -= take / e;
    }
    if t > 0.0 && u <= 0.0 {
        w = water_after(w, pores, t, e, a);
    }
    (w, if u < 1e-6 { 0.0 } else { u }, g)
}

/// Minutes until the water of `settle` is gone: what stands and what the
/// sheet holds evaporate at the full rate whichever holds it (sinking
/// only moves it between them), the pores' last at the falling rate.
fn settle_minutes(w: f32, u: f32, pores: f32, e: f32, a: f32) -> f32 {
    let up = (pores - w).max(0.0).min(u);
    time_to_dry(w + up, pores, e, a) + (u - up) / e
}

/// The threshold share of its gum dried pastel must keep to be bound: the
/// gum left in solution dries into bridges at every contact, each as the
/// share `g` of a stick's, holding as their volume^(2/3) (a bridge's neck
/// area): g^(2/3) σ_t against the finger's shear τ. σ_t is a dry soft
/// stick's tensile strength, by Prandtl's bearing capacity of a
/// cohesive-frictional solid from its hardness (H = c N_c, c = σ_t tan φ,
/// so H = σ_t (N_q − 1); about 0.19 MPa); τ the finger's friction under a
/// firm press (about 5.6 kPa). g* = (τ/σ_t)^(3/2), about 0.005 [D; σ_t's
/// bench bounds 0.1–1 MPa put it at 0.001–0.1].
pub fn gum_threshold() -> f32 {
    let phi = PHI_DEG.to_radians();
    let nq = (std::f32::consts::PI * phi.tan()).exp() * (std::f32::consts::FRAC_PI_4 + 0.5 * phi).tan().powi(2);
    let sigma_t = Stick::mechanics(SOFT_STICK).0 / (nq - 1.0);
    let f = crate::pastel::FINGER_KNEE_N;
    let tau = FINGER_MU * f / crate::pastel::pad_area(f, None);
    (tau / sigma_t).powf(1.5)
}

/// The share of a cell's pastel its dried gum binds, holding `g` of it:
/// all where the bridges hold against the finger, none where they don't
/// (`gum_threshold`).
pub fn binds(g: f32) -> f32 {
    if g >= gum_threshold() { 1.0 } else { 0.0 }
}

/// A tip's water content, by mass of its solids, at saturation `s`.
fn water_content(s: f32) -> f32 {
    s.clamp(0.0, 1.0) * POROSITY / ((1.0 - POROSITY) * SOLID_DENSITY)
}

/// A wet tip's liquidity index at saturation `s` (0 at chalk's plastic
/// limit, about S 0.53; 1 at its liquid limit, about S 0.67).
pub fn liquidity(s: f32) -> f32 {
    (water_content(s) - PLASTIC_LIMIT) / (LIQUID_LIMIT - PLASTIC_LIMIT)
}

/// The tip's hardness, MPa, of a stick `dry` MPa hard, its pores filled to
/// `s`. Wetted, its gum bridges dissolve and it is a chalk soil: below its
/// plastic limit brittle, its dry hardness; above it plastic, the
/// indentation hardness of its remoulded undrained strength at its
/// liquidity index, N_c s_u (0.31 MPa at the plastic limit, 9 kPa at the
/// liquid limit) [D]; never harder than dry.
pub fn wet_hardness(dry: f32, s: f32) -> f32 {
    let li = liquidity(s);
    if li < 0.0 {
        return dry;
    }
    (N_C * SU_LIQUID_KPA * SU_RANGE.powf(1.0 - li) * 1e-3).min(dry)
}

/// The share of what a wet tip lays that it lays as a slurry, pressed into
/// the tooth as a paste (`Canvas::stick_stroke`), rather than worn off by
/// Archard: none below the plastic limit, all past the liquid limit,
/// between them by the liquidity index (audit W17). Its crumbs go the
/// other way (W8): brittle fracture makes them, the plastic range smears.
pub fn paste_share(s: f32) -> f32 {
    liquidity(s).clamp(0.0, 1.0)
}

/// A paste's particles, dispersed by the water, lie this share of a dry
/// deposit's thickness (`PARTICLE_UM`): primary particles of
/// `PARTICLE_D_UM` cover 1.5/d of area per volume of solids, and a stick's
/// volume holds 1 − ε of solids, so a paste covers as particles
/// d/(1.5 (1 − ε)) thick, against the crumbs' 6 µm (0.42) [D].
pub fn paste_thin() -> f32 {
    PARTICLE_D_UM / (1.5 * (1.0 - POROSITY)) / PARTICLE_UM
}

/// The volume a deposit takes in the tooth once dry, as a share of the
/// stick volume laid, `paste` of it a paste: a paste packs closer as it
/// dries (`PASTE_DRIED_POROSITY`; 0.75 of its volume), crumbs don't.
pub fn dried_share(paste: f32) -> f32 {
    1.0 - paste.clamp(0.0, 1.0) * (1.0 - (1.0 - POROSITY) / (1.0 - PASTE_DRIED_POROSITY))
}

/// The water a dipped tip spreads up the stick by: Lucas–Washburn in its
/// pores, mm²/s (their radius twice the hydraulic radius ε d / 6(1 − ε),
/// the gum solution's γ and η: about 10 mm²/s).
fn tip_spread_mm2_s() -> f32 {
    let r = 2.0 * POROSITY * PARTICLE_D_UM * 1e-6 / (6.0 * (1.0 - POROSITY));
    r * GAMMA / (2.0 * pore_viscosity()) * 1e6
}

/// How far up its end a dipped tip is soaked, mm: the depth it is dipped
/// to and as far as the water wicks up while it is in the jar, 1 mm +
/// √(D t) for half a second (3.2 mm) [D; the depth and time are gestures].
pub fn dip_mm() -> f32 {
    DIP_DEPTH_MM + (tip_spread_mm2_s() * DIP_S).sqrt()
}

impl Stick {
    fn section_mm(&self) -> (f32, f32) {
        match self.section {
            Section::Round { d_mm } => (std::f32::consts::FRAC_PI_4 * d_mm * d_mm, std::f32::consts::PI * d_mm),
            Section::Square { side_mm } => (side_mm * side_mm, 4.0 * side_mm),
        }
    }

    /// The water a soaked tip holds, mm³: its pores over `dip_mm` of it.
    pub fn tip_capacity_mm3(&self) -> f32 {
        POROSITY * self.section_mm().0 * dip_mm()
    }

    /// How wet the tip is, 0 (dry) to 1 (soaked).
    pub fn wetness(&self) -> f32 {
        if self.water <= 0.0 { 0.0 } else { (self.water / self.tip_capacity_mm3()).clamp(0.0, 1.0) }
    }

    /// The water left in the dipped tip (mm³) `minutes` after it held
    /// `water`: it spreads up the dry stick (`tip_spread_mm2_s`: the wet
    /// length l² = l₀² + D t, the tip keeping l₀/l of it), and evaporates
    /// from the tip's end and wet side (`evaporation_um_min`).
    pub fn tip_water(&self, water: f32, minutes: f64) -> f32 {
        if water <= 0.0 {
            return 0.0;
        }
        let t = minutes.max(0.0) as f32;
        let l0 = dip_mm();
        let l = (l0 * l0 + tip_spread_mm2_s() * t * 60.0).sqrt();
        let (area, rim) = self.section_mm();
        let gone = evaporation_um_min() * 1e-3 * t * (area + rim * l);
        ((water - gone).max(0.0) * l0 / l).max(0.0)
    }
}

/// A wet brush's load (`Canvas::wash_pastel`): the water it holds and the
/// most it holds (mm³), the paste of pastel in it (mm³ of stick volume, its
/// dry color and its mean particle index), and how it lays: the water a
/// full load lays at the start of a stroke (µm, the tool's `lay`), the
/// share of the wet pastel under it it takes up in a pass (its `pickup`),
/// and how far a load runs down (mm, its `run`).
#[derive(Clone, Copy, Debug, Default)]
pub struct WashLoad {
    pub water_mm3: f32,
    pub cap_mm3: f32,
    pub paste_mm3: f32,
    pub paste: Rgb,
    pub paste_n: f32,
    /// The share of the paste that is bare carbon (charcoal it picked up
    /// or was ground from) and that carbon's eraser lift: laid again as
    /// charcoal (`pastel::Carried`).
    pub paste_carbon: f32,
    pub paste_lift: f32,
    pub lay_um: f32,
    pub pickup: f32,
    pub run_mm: f32,
}

impl WashLoad {
    /// A brush's load as its tool lays (`tool`, `mm_per_unit`), holding
    /// `cap_mm3` (`Held::capacity_mm3`).
    pub fn for_tool(tool: &crate::bristle::Tool, mm_per_unit: f32, cap_mm3: f32) -> WashLoad {
        WashLoad { cap_mm3, lay_um: tool.lay * crate::surface::COAT_UM, pickup: tool.pickup, run_mm: tool.run * mm_per_unit, ..WashLoad::default() }
    }
}

/// Wet `c` with `um` of water: its gum, if it has pastel, goes into
/// solution (`Cell::gum` from never wet to all of it held). Charcoal
/// brought none (`Cell::carbon_share`): a cell of vine charcoal alone holds
/// no gum, and dries as loose as it was.
pub(crate) fn add_water(c: &mut Cell, um: f32) {
    if um <= 0.0 {
        return;
    }
    c.water += um;
    if c.gum == 0 {
        c.gum = to_gum(1.0 - c.carbon_share());
    }
}

impl Canvas {
    /// `dt` minutes pass for the water in the pastel (`wait`, `settle`):
    /// what stands over the pastel's pores sinks into the sheet as its
    /// sizing lets it, to what its pores take (`sink_um`), taking its share
    /// of the dissolved gum, as it evaporates; then the sheet's water
    /// evaporates up through the pastel's pores, then the pores. Where it
    /// is all gone the gum sets, binding the pastel where enough of it is
    /// left (`binds`, `Cell::gum_set`). Pastel painted over since is
    /// sealed: its water is forgotten. Nothing without the switch or water.
    pub(crate) fn dry_pastel_water(&mut self, dt: f32) {
        if !self.wet_pastel || !(dt > 0.0) || !self.drawing.as_ref().is_some_and(|d| d.damp) {
            return;
        }
        let sheet = self.paper.as_ref().map(|p| (uptake_rate(p), sheet_capacity_um(p)));
        let (e, a) = (evaporation_um_min(), falling());
        let d = self.drawing.as_deref_mut().expect("damp");
        if sheet.is_some() && d.sheet.len() != d.cells.len() {
            d.sheet = vec![0.0; d.cells.len()];
        }
        let mut damp = false;
        for (i, c) in d.cells.iter_mut().enumerate() {
            let u = d.sheet.get(i).copied().unwrap_or(0.0);
            if c.water <= 0.0 && u <= 0.0 {
                continue;
            }
            if self.film[i] > c.film + 1e-4 {
                c.water = 0.0;
                if let Some(s) = d.sheet.get_mut(i) {
                    *s = 0.0;
                }
                continue;
            }
            let before = (c.a, c.reflectance());
            let pores = POROSITY * (c.loose + c.bound);
            if c.gum == 0 && pores > 0.0 {
                // (dry pastel over a damp sheet: it draws the water up)
                c.gum = to_gum(1.0 - c.carbon_share());
            }
            let wet_before = c.water > 0.0 || (pores > 0.0 && u > 0.0);
            let (w, u, g) = settle(c.water, u, pores, from_gum(c.gum), dt, e, a, sheet);
            c.gum = to_gum(g);
            if let Some(s) = d.sheet.get_mut(i) {
                *s = u;
            }
            if w > 0.0 || u > 0.0 {
                c.water = w;
                damp = true;
            } else if wet_before {
                c.gum_set(binds(g));
            }
            self.px[i] = c.repaint(self.px[i], before);
        }
        d.damp = damp;
    }

    /// Minutes until the last water in the pastel and the sheet under it
    /// is gone (`dry`, `settle_minutes`).
    pub(crate) fn pastel_water_minutes(&self) -> f32 {
        if !self.wet_pastel {
            return 0.0;
        }
        let Some(d) = self.drawing.as_ref().filter(|d| d.damp) else { return 0.0 };
        let (e, a) = (evaporation_um_min(), falling());
        let t = d.cells.iter().enumerate().map(|(i, c)| {
            let pores = POROSITY * (c.loose + c.bound);
            settle_minutes(c.water, d.sheet.get(i).copied().unwrap_or(0.0), pores, e, a)
        }).fold(0.0, f32::max);
        // (rounding: a little past the last)
        t * 1.0001 + 1e-4
    }

    /// Water standing in the pastel at canvas point (x, y), µm (0 off the
    /// canvas or where there is none).
    pub fn pastel_water_at(&self, x: f32, y: f32) -> f32 {
        let f = self.f;
        let (gx, gy) = ((x * f.scale).floor() as isize - f.x0 as isize, (y * f.scale).floor() as isize - f.y0 as isize);
        if gx < 0 || gy < 0 || gx >= f.w as isize || gy >= f.h as isize {
            return 0.0;
        }
        self.drawing.as_ref().map_or(0.0, |d| d.cells[gy as usize * f.w + gx as usize].water)
    }

    /// A wet brush drawn along `pts` (units), touching a band `width` units
    /// wide across the path, pressed `pressure` (0..1). It lays water as it
    /// goes (`load.lay_um` at full load, less as it empties), the loose
    /// pastel it wets turns to a paste the bristles pick up (`load.pickup`
    /// of it a pass, as far as the water fills its pores) and, with the
    /// paste they already held, lay down along the path within about
    /// `load.run_mm`, as a paste (`paste_thin`, `dried_share`). Bound pastel stays. Not
    /// under a sheet or on wet paint. Returns (the pastel it laid, mm³; the
    /// hand time, s, as a brush stroke's); what it still holds stays in
    /// `load`. Nothing without the switch.
    pub fn wash_pastel(&mut self, width: f32, pressure: f32, pts: &[(f32, f32)], load: &mut WashLoad) -> (f32, f32) {
        if pts.is_empty() || !self.wet_pastel || !(width.is_finite() && width > 0.0) {
            return (0.0, 0.0);
        }
        let s = self.f.scale;
        let s_mm = self.px_mm();
        // (µm over a pixel to mm³)
        let um_mm3 = s_mm * s_mm * 1e-3;
        let half = width * s * 0.5;
        // (pressed harder, the belly gives more: the bristle model's own
        // law for the film a touch squeezes out, 0.4 + 0.6 p, `bristle::touch`)
        let press = 0.4 + 0.6 * pressure.clamp(0.0, 1.0);
        let steps = self.walk(pts);
        let lay = (STEP_PX * s_mm / load.run_mm.max(1e-3)).min(1.0);
        let pick = (load.pickup * STEP_PX).min(1.0);
        let thin = PARTICLE_UM * paste_thin();
        let n_paste = if load.paste_n > 0.0 { load.paste_n } else { DEFAULT_INDEX };
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        let mut d = self.drawing.take().expect("drawing_mut made it");
        let (mut paste, mut col, mut np) = (load.paste_mm3.max(0.0) / um_mm3, load.paste, n_paste);
        let mut carried = crate::pastel::Carried::of_parts(load.paste_carbon, load.paste_lift);
        let mut laid = 0.0f32;
        for &(c, dir) in &steps {
            let e = (-dir.1, dir.0);
            let nb = ((2.0 * half / STEP_PX).ceil() as usize).max(1);
            let mut band: Vec<usize> = (0..=nb).filter_map(|k| {
                let u = -half + 2.0 * half * k as f32 / nb as f32;
                self.win_px(c.0 + e.0 * u, c.1 + e.1 * u)
            }).collect();
            band.sort_unstable();
            band.dedup();
            band.retain(|&i| !self.sheet_over(i) && self.wet.vol[i] <= 1e-5);
            if band.is_empty() {
                continue;
            }
            let n = band.len() as f32;
            // the water it lays, a visit's share of a pass, as it has it
            let full = if load.cap_mm3 > 0.0 { (load.water_mm3 / load.cap_mm3).clamp(0.0, 1.0) } else { 0.0 };
            let mut per = load.lay_um * full * press * STEP_PX;
            if per * n * um_mm3 > load.water_mm3 {
                per = load.water_mm3.max(0.0) / (n * um_mm3);
            }
            load.water_mm3 = (load.water_mm3 - per * n * um_mm3).max(0.0);
            for &i in &band {
                let cl = &mut d.cells[i];
                if cl.film < 0.0 || film[i] > cl.film + 1e-4 {
                    // bare, or painted over since: a new layer on top
                    cl.restart(film[i], pxs[i]);
                }
                let before = (cl.a, cl.reflectance());
                add_water(cl, per);
                if cl.loose > 0.0 {
                    // the wetted loose pastel, a paste the bristles take up
                    let filled = (cl.water / (POROSITY * (cl.loose + cl.bound)).max(1e-6)).min(1.0);
                    let take = cl.loose * pick * filled;
                    if take > 0.0 {
                        carried.take(paste, take, cl);
                        let total = paste + take;
                        col = [0, 1, 2].map(|q| (col[q] * paste + cl.r[q] * take) / total.max(1e-9));
                        np = (np * paste + from_index(cl.index) * take) / total.max(1e-9);
                        paste = total;
                        let base = cl.floor + cl.non_pastel - cl.non_pastel_floor;
                        cl.a = (base + (cl.a - base).max(0.0) * (1.0 - take / cl.loose)).max(cl.non_pastel);
                        cl.loose -= take;
                    }
                }
                pxs[i] = cl.repaint(pxs[i], before);
            }
            // and lays a share of the paste it holds over the band
            let give = paste * lay;
            if give > 0.0 {
                let per = give / n;
                let q = 1.0 - (-per / thin).exp();
                // (counted at its dried packing: `dried_share`)
                let kept = per * dried_share(1.0);
                for &i in &band {
                    let cl = &mut d.cells[i];
                    let before = (cl.a, cl.reflectance());
                    crate::pastel::lay_into(cl, q, col, kept, crate::pastel::Grains { index: np, ..carried.grains() });
                    pxs[i] = cl.repaint(pxs[i], before);
                }
                paste -= give;
                laid += give;
                d.sticks = true;
                // (charcoal laid again: its cells keep their carbon shares)
                d.carbon |= carried.parts().0 > 0.0 && d.optics();
            }
            d.damp = true;
        }
        self.drawing = Some(d);
        self.px = pxs;
        self.film = film;
        load.paste_mm3 = paste * um_mm3;
        load.paste = col;
        load.paste_n = np;
        (load.paste_carbon, load.paste_lift) = carried.parts();
        let len_mm = (steps.len().max(1) - 1) as f32 * STEP_PX * s_mm;
        (laid * um_mm3, crate::tally::stroke_secs(len_mm as f64, (width * self.mm_per_unit) as f64) as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pastel::{Pose, StrokePoint};

    /// A 150 mm sheet of drawing paper, 160 px, with the drawing's optics
    /// and (or not) the switch.
    fn paper(wet: bool) -> Canvas {
        crate::tests::drawing_paper(160).with_drawing_optics(true).with_wet_pastel(wet)
    }

    /// The same sheet with another sheet's sizing and share of its pores
    /// water reaches.
    fn sheet(c: Canvas, cobb60: Option<f32>, absorbent: f32) -> Canvas {
        let mut p = *c.paper.as_ref().unwrap();
        (p.cobb60, p.absorbent) = (cobb60, absorbent);
        let mut c = c;
        c.paper = Some(p);
        c
    }

    fn stroke(c: &mut Canvas, st: &mut Stick, x0: f32, y0: f32, x1: f32, y1: f32, force: f32) -> f32 {
        let pose = Pose { force, alt: 60f32.to_radians(), az: 45f32.to_radians(), roll: 0.0 };
        c.stick_stroke(st, &[StrokePoint { x: x0, y: y0, pose, speed: 50.0 }, StrokePoint { x: x1, y: y1, pose, speed: 50.0 }]).volume_mm3
    }

    /// A band of dark pastel across the sheet at y 450–550.
    fn band(c: &mut Canvas) {
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        for k in 0..6 {
            let y = 455.0 + 18.0 * k as f32;
            stroke(c, &mut st, 100.0, y, 900.0, y, 3.0);
        }
    }

    /// How dark (below the paper's 0.55) the sheet is over x0–x1, y0–y1.
    fn darkening(c: &Canvas, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
        let f = c.window();
        c.pixels().iter().enumerate().filter(|(i, _)| {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            x >= x0 && x < x1 && y >= y0 && y < y1
        }).map(|(_, p)| 0.55 - (p[0] + p[1] + p[2]) / 3.0).sum()
    }

    /// Water poked into every cell holding pastel, as a brush that moved
    /// nothing would leave it.
    fn wet_all(c: &mut Canvas, um: f32) {
        let d = c.drawing.as_deref_mut().unwrap();
        for (i, cell) in d.cells.iter_mut().enumerate() {
            if cell.loose + cell.bound > 0.0 {
                let before = (cell.a, cell.reflectance());
                add_water(cell, um);
                c.px[i] = cell.repaint(c.px[i], before);
            }
        }
        d.damp = true;
    }

    fn water(c: &Canvas) -> f32 {
        c.drawing.as_ref().map_or(0.0, |d| d.cells.iter().map(|c| c.water).sum())
    }

    fn bound(c: &Canvas) -> f32 {
        c.drawing.as_ref().unwrap().cells.iter().map(|c| c.bound).sum()
    }

    /// A stick dipped in water lays more than the dry stick at the same
    /// force, darker while it is wet, and leaves water; on the painting's
    /// clock the water dries and the stroke lightens.
    #[test]
    fn a_wet_stick_lays_more_and_darker_and_dries_on_the_clock() {
        let (mut dry, mut wet) = (paper(true), paper(true));
        let mut a = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        a.index = 1.5;
        let mut b = a.clone();
        b.water = b.tip_capacity_mm3();
        let (va, vb) = (stroke(&mut dry, &mut a, 100.0, 500.0, 900.0, 500.0, 2.0), stroke(&mut wet, &mut b, 100.0, 500.0, 900.0, 500.0, 2.0));
        assert!(vb > 1.3 * va, "lays more: {vb} vs {va}");
        assert!(b.water > 0.0 && b.water < b.tip_capacity_mm3(), "the tip gave water: {}", b.water);
        assert_eq!((b.hardness, b.wear), (a.hardness, a.wear), "the stick dries back to itself");
        assert!(water(&wet) > 0.0 && water(&dry) == 0.0);
        let f = wet.window();
        let i = (0..f.w * f.h).max_by(|&a, &b| { let d = &wet.drawing.as_ref().unwrap().cells; d[a].water.total_cmp(&d[b].water) }).unwrap();
        let (x, y) = (f.ux(i % f.w) + 0.5 / f.scale, f.uy(i / f.w) + 0.5 / f.scale);
        assert!(wet.feel(x, y).ends_with(", wet"), "{}", wet.feel(x, y));
        // darker for its volume: the same deposit dry against wet
        let db = darkening(&wet, 200.0, 800.0, 450.0, 550.0);
        let mut dried = wet.clone();
        dried.wait(0.05);
        assert!(water(&dried) > 0.0, "still damp after three seconds");
        dried.wait(120.0);
        assert_eq!(water(&dried), 0.0, "dry after two hours");
        assert!(!dried.drawing.as_ref().unwrap().damp);
        let dd = darkening(&dried, 200.0, 800.0, 450.0, 550.0);
        assert!(dd < 0.9 * db, "lighter dry than wet: {dd} vs {db}");
        // the tip's water spreads up the stick: l₀/√(l₀² + D t), l₀ 3.23 mm
        // and D 10.0 mm²/s from γ 50 mN/m, η 1.74 mPa·s, r 0.69 µm
        assert_eq!(b.tip_water(1.0, 0.0), 1.0);
        let (one, ten) = (b.tip_water(1.0, 1.0 / 60.0), b.tip_water(1.0, 10.0 / 60.0));
        assert!((0.68..0.75).contains(&one) && (0.28..0.34).contains(&ten), "{one} {ten}");
    }

    /// The tip by its water: below chalk's plastic limit (S 0.53) it draws
    /// as the dry stick, lays its water with it and sheds as many crumbs;
    /// soaked past its liquid limit (S 0.67) it is a slurry pressed into
    /// the tooth, with no crumbs.
    #[test]
    fn a_wet_tip_is_dry_below_its_plastic_limit_and_a_slurry_past_its_liquid_limit() {
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        let (mut dry, mut damp) = (paper(true), paper(true));
        let mut d = st.clone();
        st.water = 0.5 * st.tip_capacity_mm3();
        assert_eq!(stroke(&mut dry, &mut d, 100.0, 500.0, 900.0, 500.0, 2.0), stroke(&mut damp, &mut st, 100.0, 500.0, 900.0, 500.0, 2.0), "below the plastic limit");
        assert!(water(&damp) > 0.0);
        // soaked: a slurry with no strength, the face conforms to the tooth
        // (it carries the force over far more of it) and fills its pores
        let mut slurry = paper(true);
        let mut s = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        s.water = s.tip_capacity_mm3();
        assert_eq!(paste_share(s.wetness()), 1.0);
        let vs = stroke(&mut slurry, &mut s, 100.0, 500.0, 900.0, 500.0, 2.0);
        let cells = |c: &Canvas| c.drawing.as_ref().unwrap().cells.iter().filter(|c| c.loose > 0.0).count();
        assert!(cells(&slurry) > cells(&dry), "a slurry meets more of the tooth: {} vs {}", cells(&slurry), cells(&dry));
        assert!(vs > 0.0 && s.water < s.tip_capacity_mm3());
    }

    /// The same pastel wetted (without moving it) reads darker while wet,
    /// and once dry is lighter than wet and a little darker than it was
    /// dry (its gum in the pores); a high-index pigment darkens less.
    #[test]
    fn wetted_pastel_darkens_by_its_index_and_dries_back_nearly_to_dry() {
        let run = |n: f32| {
            let mut c = paper(true);
            let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
            st.index = n;
            for k in 0..6 {
                let y = 455.0 + 18.0 * k as f32;
                stroke(&mut c, &mut st, 100.0, y, 900.0, y, 3.0);
            }
            let dry = darkening(&c, 200.0, 800.0, 450.0, 550.0);
            wet_all(&mut c, 40.0);
            let wet = darkening(&c, 200.0, 800.0, 450.0, 550.0);
            c.dry();
            assert_eq!(water(&c), 0.0);
            (dry, wet, darkening(&c, 200.0, 800.0, 450.0, 550.0))
        };
        let (dry, wet, dried) = run(1.5);
        assert!(wet > dried && dried > dry && dried < dry * 1.1, "wet {wet}, dried {dried}, dry {dry}");
        let (dry3, wet3, _) = run(3.0);
        assert!((wet3 - dry3) < 0.5 * (wet - dry), "vermilion's index darkens less: {} vs {}", wet3 - dry3, wet - dry);
    }

    /// A stick's particle index is its pigments' and its whiting's by
    /// scattering: a pure pigment's own, a white pile whiting's, and a
    /// tint's between, so a pale tint darkens in water by its chalk.
    #[test]
    fn a_tint_is_its_pigment_let_down_with_whiting() {
        let pal = crate::Palette::new("wet pastel tints", crate::palette::catalog());
        let at = |name: &str| pal.tubes.iter().position(|t| t.name == name).unwrap();
        let (v, w) = (at("vermilion"), at("lead white"));
        assert!((pal.dry_index(&[(v, 1.0)]) - 3.0).abs() < 1e-4);
        assert!((pal.dry_index(&[(w, 1.0)]) - crate::dry::WHITING_INDEX).abs() < 1e-4);
        let tint = pal.dry_index(&[(v, 1.0), (w, 3.0)]);
        assert!(tint > crate::dry::WHITING_INDEX && tint < 3.0, "{tint}");
        let paler = pal.dry_index(&[(v, 1.0), (w, 10.0)]);
        assert!(paler < tint, "{paler} {tint}");
        assert!(crate::dry::scatter_in(tint, 1.333) < crate::dry::scatter_in(3.0, 1.333));
    }

    /// Dried wet pastel keeps enough of its gum to be bound where little of
    /// its water sank: the finger moves less of it than of dry pastel, and
    /// none of fixed; a stick takes to it more, as to a fixed crust. A
    /// flood the sheet takes in carries its gum away below the threshold
    /// the bridges need, and leaves the pastel loose.
    #[test]
    fn dried_wet_pastel_is_bound_where_it_kept_its_gum() {
        let mut dry = paper(true);
        band(&mut dry);
        let mut gummed = dry.clone();
        wet_all(&mut gummed, 40.0);
        gummed.dry();
        assert_eq!(water(&gummed), 0.0);
        let mut fixed = dry.clone();
        fixed.fix_pastel(None, 0.15);
        let moved = |c: &mut Canvas| {
            let before = c.pixels().to_vec();
            c.rub(&[(500.0, 300.0), (500.0, 700.0)], 1.0, None, 40.0);
            c.pixels().iter().zip(&before).map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()).sum::<f32>()
        };
        let (md, mg, mf) = (moved(&mut dry.clone()), moved(&mut gummed.clone()), moved(&mut fixed));
        assert!(md > 3.0 * mg && mf == 0.0, "dry {md}, dried wet {mg}, fixed {mf}");
        // a light veil flooded with 150 µm: an unsized sheet whose pores all
        // take water drinks it at once, gum and all (g ≈ 0.55 µm of pores
        // over 150 µm, under the threshold); one whose pores take 3 µm
        // keeps most of it standing until it evaporates, and its gum
        let veil = |c: Canvas| {
            let mut c = c;
            let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
            stroke(&mut c, &mut st, 100.0, 500.0, 900.0, 500.0, 0.3);
            wet_all(&mut c, 150.0);
            c.dry();
            c
        };
        let (drunk, kept) = (veil(sheet(paper(true), None, 1.0)), veil(sheet(paper(true), Some(25.0), 0.02)));
        let left_loose = |c: &Canvas| c.drawing.as_ref().unwrap().cells.iter().filter(|c| c.loose > 0.0 && c.bound == 0.0).count();
        assert!(bound(&kept) > 0.0 && left_loose(&kept) == 0, "{}", left_loose(&kept));
        assert!(left_loose(&drunk) > 0 && bound(&drunk) < bound(&kept), "{} {} vs {}", left_loose(&drunk), bound(&drunk), bound(&kept));
        // wetting again binds no more than it holds
        let once = bound(&gummed);
        wet_all(&mut gummed, 40.0);
        gummed.dry();
        assert!((bound(&gummed) - once).abs() <= 1e-3 * once, "{} vs {once}", bound(&gummed));
        // over a full tooth, the dried wet passage takes more pastel than the dry one
        let (mut dry, mut gummed) = (paper(true), paper(true));
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        for _ in 0..8 {
            stroke(&mut dry, &mut st.clone(), 100.0, 500.0, 900.0, 500.0, 3.0);
            stroke(&mut gummed, &mut st.clone(), 100.0, 500.0, 900.0, 500.0, 3.0);
        }
        wet_all(&mut gummed, 40.0);
        gummed.dry();
        let (vd, vg) = (stroke(&mut dry, &mut st.clone(), 100.0, 500.0, 900.0, 500.0, 3.0), stroke(&mut gummed, &mut st, 100.0, 500.0, 900.0, 500.0, 3.0));
        assert!(vg > 1.1 * vd, "takes more: {vg} vs {vd}");
    }

    fn brush(water: f32) -> WashLoad {
        let tool = crate::bristle::Tool::round_sable(40.0);
        let cap = crate::bristle::Held::new(tool.clone(), 1).capacity_mm3(0.15);
        WashLoad { water_mm3: water.min(cap), ..WashLoad::for_tool(&tool, 0.15, cap) }
    }

    /// Water over charcoal (`crate::charcoal`): charcoal brings no gum, so
    /// vine charcoal wetted and dried is as loose as before, while pastel
    /// under it binds, the charcoal bound with it counted as bound carbon.
    /// A wet brush drawn out of a vine band carries the charcoal on as bare
    /// carbon at vine's lift, not as pastel.
    #[test]
    fn water_over_charcoal_leaves_it_charcoal() {
        let vine = |c: &mut Canvas, x0: f32| {
            let mut st = Stick::vine(8.0);
            for k in 0..6 {
                let x = x0 + 8.0 * k as f32;
                stroke(c, &mut st, x, 200.0, x, 800.0, 3.0);
            }
        };
        let bound = |c: &Canvas| c.drawing.as_ref().unwrap().cells.iter().map(|c| c.bound).sum::<f32>();
        // vine alone: no gum, nothing binds
        let mut alone = paper(true);
        vine(&mut alone, 300.0);
        let d = alone.drawing.as_ref().unwrap();
        assert!(d.cells.iter().any(|c| c.carbon > 0.99 && c.loose > 0.0));
        assert!(d.cells.iter().filter(|c| c.loose > 0.0).all(|c| (c.carbon_share() - 1.0).abs() < 1e-4));
        wet_all(&mut alone, 20.0);
        assert!(alone.drawing.as_ref().unwrap().cells.iter().filter(|c| c.loose > 0.0).all(|c| from_gum(c.gum) == 0.0));
        alone.dry();
        assert_eq!(water(&alone), 0.0);
        assert_eq!(bound(&alone), 0.0, "vine has no binder to set");
        // vine over a pastel band: the pastel's gum binds the cell, and the
        // carbon in what binds is bound carbon
        let mut over = paper(true);
        band(&mut over);
        vine(&mut over, 300.0);
        wet_all(&mut over, 20.0);
        over.dry();
        assert!(bound(&over) > 0.0);
        let both: Vec<_> = over.drawing.as_ref().unwrap().cells.iter().filter(|c| c.carbon > 0.0 && c.bound > 0.0).collect();
        assert!(!both.is_empty() && both.iter().all(|c| c.carbon_bound > 0.0 && c.carbon_bound < 1.0));
        // a wet brush through vine lays it further on as charcoal
        let mut c = paper(true);
        vine(&mut c, 300.0);
        let mut load = brush(f32::INFINITY);
        c.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut load);
        assert!(load.paste_carbon > 0.99 && (load.paste_lift - crate::charcoal::VINE_LIFT).abs() < 1e-3, "{} {}", load.paste_carbon, load.paste_lift);
        let f = c.window();
        let d = c.drawing.as_ref().unwrap();
        let past: Vec<_> = (0..f.w).filter(|&x| f.ux(x) > 450.0 && f.ux(x) < 650.0).map(|x| &d.cells[(500.0 * f.scale) as usize * f.w + x]).filter(|c| c.loose > 0.0).collect();
        assert!(!past.is_empty(), "carried past the band");
        assert!(past.iter().all(|c| c.carbon > 0.99 && (c.lift - crate::charcoal::VINE_LIFT).abs() < 1e-3 && c.bare_carbon().is_some()));
    }

    /// A wet brush drawn out of a band of loose pastel onto bare paper
    /// carries it along its path as a paste: the paper past the band
    /// darkens along the path (and not beside it), and all of it is wet.
    /// Water alone on bare paper shows nothing; a sized sheet takes it in
    /// over seconds, an unsized one at once.
    #[test]
    fn a_wet_brush_moves_loose_pastel_along_its_path() {
        let mut c = paper(true);
        let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        for k in 0..6 {
            let x = 300.0 + 8.0 * k as f32;
            stroke(&mut c, &mut st, x, 200.0, x, 800.0, 3.0);
        }
        let mut load = brush(f32::INFINITY);
        let cap = load.cap_mm3;
        let f = c.window();
        let row = |c: &Canvas, x0: f32, x1: f32, y: f32| (0..f.w).filter(|&x| f.ux(x) > x0 && f.ux(x) < x1).map(|x| 0.55 - c.pixels()[(y * f.scale) as usize * f.w + x][0]).sum::<f32>();
        let (past, beside) = (row(&c, 400.0, 650.0, 500.0), row(&c, 400.0, 650.0, 300.0));
        assert!(past.abs() < 1e-4 && beside.abs() < 1e-4, "{past} {beside}");
        let (laid, secs) = c.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut load);
        assert!(laid > 0.0 && secs > 0.0);
        assert!(load.water_mm3 < cap);
        assert!(row(&c, 400.0, 650.0, 500.0) > 0.05, "carried past the band: {}", row(&c, 400.0, 650.0, 500.0));
        assert!(row(&c, 400.0, 650.0, 300.0).abs() < 1e-4, "not beside the path");
        assert!(c.pastel_water_at(500.0, 500.0) > 0.0 && c.pastel_water_at(500.0, 300.0) == 0.0);
        // water alone over bare paper: the sized sheet takes it in over
        // seconds (Cobb 25 g/m²: 3.2 µm in the first second, 1.6 in a
        // quarter of one)
        let mut bare = paper(true);
        let before = bare.pixels().to_vec();
        bare.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut brush(50.0));
        assert_eq!(bare.pixels(), &before[..]);
        let laid = bare.pastel_water_at(500.0, 500.0);
        assert!(laid > 2.0, "{laid}");
        let mut bare_unsized = sheet(bare.clone(), None, 0.8);
        bare.wait(1.0 / 240.0);
        assert!(bare.pastel_water_at(500.0, 500.0) > 0.0, "still standing after a quarter second");
        bare.wait(10.0);
        assert_eq!(bare.pastel_water_at(500.0, 500.0), 0.0, "in the sheet");
        assert_eq!(bare.pixels(), &before[..]);
        bare_unsized.wait(1.0 / 60.0);
        assert_eq!(bare_unsized.pastel_water_at(500.0, 500.0), 0.0, "an unsized sheet drinks it at once");
        // (and it dries out of the sheet on the clock)
        assert!(bare.drawing.as_ref().unwrap().damp);
        bare.dry();
        assert!(!bare.drawing.as_ref().unwrap().damp);
    }

    /// Ground pastel mixed with water on the brush (a paste) lays its color
    /// over bare paper, wet, and bound once dry.
    #[test]
    fn a_paste_on_the_brush_lays_its_color_wet() {
        let mut c = paper(true);
        let mut load = WashLoad { paste_mm3: 4.0, paste: [0.6, 0.05, 0.03], paste_n: 3.0, ..brush(f32::INFINITY) };
        let (laid, _) = c.wash_pastel(40.0, 0.5, &[(200.0, 500.0), (800.0, 500.0)], &mut load);
        // (what it lays it partly picks up again further on: laid counts it each time)
        assert!(laid > 1.0 && load.paste_mm3 < 2.0, "{laid} {}", load.paste_mm3);
        let i = {
            let f = c.window();
            (500.0 * f.scale) as usize * f.w + (400.0 * f.scale) as usize
        };
        let p = c.pixels()[i];
        assert!(p[0] > p[1] + 0.05, "red: {p:?}");
        let cell = c.drawing.as_ref().unwrap().cells[i];
        assert!(cell.water > 0.0 && (from_index(cell.index) - 3.0).abs() < 1e-3);
        c.dry();
        let cell = c.drawing.as_ref().unwrap().cells[i];
        assert!(cell.bound > 0.0 && cell.water == 0.0);
    }

    /// Without the switch a wet tip draws as the dry stick, bit for bit, a
    /// wet brush does nothing and `wait` leaves the drawing as it was.
    #[test]
    fn without_the_switch_nothing_takes_water() {
        let (mut a, mut b) = (paper(false), paper(false));
        let mut dry = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
        let mut wet = dry.clone();
        wet.water = wet.tip_capacity_mm3();
        assert_eq!(stroke(&mut a, &mut dry, 100.0, 500.0, 900.0, 500.0, 2.0), stroke(&mut b, &mut wet, 100.0, 500.0, 900.0, 500.0, 2.0));
        assert_eq!(a.pixels(), b.pixels());
        assert!(b.drawing.as_ref().unwrap().cells.iter().all(|c| c.index == 0 && c.gum == 0 && c.water == 0.0));
        assert_eq!(wet.water, wet.tip_capacity_mm3());
        let before = b.pixels().to_vec();
        assert_eq!(b.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut brush(50.0)), (0.0, 0.0));
        assert_eq!(b.pixels(), &before[..]);
        assert!(!crate::tests::drawing_paper(160).with_wet_pastel(true).wet_pastel(), "needs the drawing's optics");
    }

    /// The water, index and gum in the pastel, the water the sheet took in
    /// and the sheet's sizing are saved and read back, and dry the same
    /// after; a file whose section is damaged is refused.
    #[test]
    fn a_checkpoint_keeps_the_water() {
        let mut c = sheet(paper(true), Some(40.0), 0.8);
        band(&mut c);
        wet_all(&mut c, 20.0);
        c.wait(0.1);
        assert!(c.drawing.as_ref().unwrap().sheet.iter().any(|&u| u > 0.0), "the sheet took some");
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut std::io::Cursor::new(&bytes)).unwrap();
        assert!(r.wet_pastel() && r.drawing.as_ref().unwrap().damp);
        assert_eq!(r.paper.as_ref().unwrap().cobb60, Some(40.0));
        assert_eq!(water(&r), water(&c));
        assert_eq!(r.drawing.as_ref().unwrap().sheet, c.drawing.as_ref().unwrap().sheet);
        assert_eq!(r.pixels(), c.pixels());
        c.wait(3.0);
        r.wait(3.0);
        assert_eq!(r.pixels(), c.pixels());
        let mut again = Vec::new();
        r.write_state(&mut again, "").unwrap();
        let mut c2 = Vec::new();
        c.write_state(&mut c2, "").unwrap();
        assert_eq!(again, c2);
        // the section's mark, damaged
        let at = bytes.windows(8).position(|w| w == WET_MARK.to_le_bytes()).expect("the section");
        let mut bad = bytes.clone();
        bad[at] ^= 1;
        assert!(Canvas::read_state(&mut std::io::Cursor::new(&bad)).is_err());
        // an unsized sheet reads back unsized
        let mut u = sheet(paper(true), None, 0.8);
        band(&mut u);
        let mut b = Vec::new();
        u.write_state(&mut b, "").unwrap();
        assert_eq!(Canvas::read_state(&mut std::io::Cursor::new(&b)).unwrap().0.paper.unwrap().cobb60, None);
        // a canvas without the switch writes no section
        let mut plain = paper(false);
        band(&mut plain);
        let mut b = Vec::new();
        plain.write_state(&mut b, "").unwrap();
        assert!(!b.windows(8).any(|w| w == WET_MARK.to_le_bytes()));
        assert!(!Canvas::read_state(&mut std::io::Cursor::new(&b)).unwrap().0.wet_pastel());
    }

    /// A drawing with both charcoal (its carbon shares, the drawing's
    /// layout 5) and wet pastel (the WETP section's water, index and gum,
    /// and the sheet's water) on a gritted sheet on a strainer comes back
    /// bit for bit, with its carbon, water and gum, and dries the same.
    #[test]
    fn a_checkpoint_keeps_charcoal_and_water_together() {
        use crate::paper::{Binder, Grit, Mount};
        let mut c = paper(true);
        let mut p = c.paper.unwrap();
        p.grit = Some(Grit { size_um: 35.0, cover: 0.6, binder: Binder::Glue });
        c.paper = Some(p.mounted(Mount::Strainer { bar_mm: 30.0, cross: 0 }));
        band(&mut c);
        let mut st = Stick::vine(8.0);
        for k in 0..4 {
            let x = 300.0 + 8.0 * k as f32;
            stroke(&mut c, &mut st, x, 200.0, x, 800.0, 3.0);
        }
        wet_all(&mut c, 20.0);
        c.wait(0.1);
        {
            let d = c.drawing.as_ref().unwrap();
            assert!(d.carbon && d.damp && d.sheet.iter().any(|&u| u > 0.0));
            assert!(d.cells.iter().any(|c| c.carbon > 0.0 && c.water > 0.0 && c.gum != 0 && c.index != 0));
        }
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut std::io::Cursor::new(&bytes)).unwrap();
        let (a, b) = (c.drawing.as_ref().unwrap(), r.drawing.as_ref().unwrap());
        assert!(b.carbon && b.damp && a.sheet == b.sheet);
        assert!(a.cells.iter().zip(&b.cells).all(|(x, y)| x.carbon == y.carbon && x.carbon_bound == y.carbon_bound && x.water == y.water && x.index == y.index && x.gum == y.gum && x.lift == y.lift && x.loose == y.loose));
        assert!(r.paper == c.paper && r.paper.unwrap().grit.is_some() && r.paper.unwrap().drum.is_some());
        assert_eq!(r.pixels(), c.pixels());
        let mut again = Vec::new();
        r.write_state(&mut again, "").unwrap();
        assert!(again == bytes, "rewritten as it was read");
        c.dry();
        r.dry();
        assert_eq!(r.pixels(), c.pixels());
        let (a, b) = (c.drawing.as_ref().unwrap(), r.drawing.as_ref().unwrap());
        assert!(a.cells.iter().zip(&b.cells).all(|(x, y)| x.bound == y.bound && x.carbon_bound == y.carbon_bound));
    }

    /// "WETP", as checkpoint.rs marks the section.
    const WET_MARK: u64 = 0x50544557;

    /// The derived rates and constants, against the arithmetic of the
    /// research note (wet_pastel.md; estimates_audit_2.py).
    #[test]
    fn the_rates_follow_from_the_physics() {
        let e = evaporation_um_min();
        assert!((0.6..0.8).contains(&e), "{e} µm/min");
        // the default sheet: Cobb 25 g/m², √t, to 0.8 × 0.5 × 300 µm
        let mut p = crate::paper::Paper::drawing(1);
        let (s1, s60, cap) = (sink_um(&p, 0.0, 1.0 / 60.0), sink_um(&p, 0.0, 1.0), sink_um(&p, 0.0, 1e4));
        assert!((3.1..3.4).contains(&s1) && (24.5..25.5).contains(&s60) && (119.0..121.0).contains(&cap), "{s1} {s60} {cap}");
        assert_eq!(sink_um(&p, cap, 10.0), 0.0, "full");
        // unsized: Whatman's 0.38 cm/√s fills it within a second
        p.cobb60 = None;
        assert!((sink_um(&p, 0.0, 1.0 / 60.0) - cap).abs() < 1.0);
        // the wet tip: chalk's plastic limit at S 0.53, liquid limit at 0.67
        assert_eq!(wet_hardness(3.9, 0.5), 3.9);
        let half = (0.244 + 0.033) * (0.45 * 2.65) / 0.55;
        let h = wet_hardness(3.9, half);
        assert!((0.049..0.054).contains(&h), "5.14 × 1.7 kPa × √35 = 51.7 kPa: {h}");
        assert!(wet_hardness(3.9, 1.0) < 0.001);
        assert!(paste_share(0.5) == 0.0 && paste_share(1.0) == 1.0);
        assert!((0.022..0.025).contains(&gum_pore_share()));
        assert!((0.40..0.44).contains(&paste_thin()));
        assert!((1.70e-3..1.78e-3).contains(&pore_viscosity()), "{}", pore_viscosity());
        assert!((3.1..3.35).contains(&dip_mm()), "{}", dip_mm());
        let g = gum_threshold();
        assert!((0.004..0.006).contains(&g), "{g}");
        assert!((dried_share(1.0) - 0.75).abs() < 1e-6 && dried_share(0.0) == 1.0);
    }

    #[test]
    fn water_dries_at_a_constant_then_a_falling_rate() {
        let (e, a) = (1.0, falling());
        // standing water: constant rate
        assert_eq!(water_after(10.0, 0.0, 2.0, e, 0.0), 8.0);
        assert_eq!(water_after(10.0, 0.0, 12.0, e, 0.0), 0.0);
        assert!((time_to_dry(10.0, 0.0, e, 0.0) - 10.0).abs() < 1e-6);
        // the pores dry slower than the film, and the two agree on when
        let t = time_to_dry(30.0, 20.0, e, 0.05);
        assert!(t > 30.0, "{t}");
        assert!(water_after(30.0, 20.0, t * 0.999, e, 0.05) > 0.0);
        assert_eq!(water_after(30.0, 20.0, t * 1.001, e, 0.05), 0.0);
        // (the real pastel layer is thin: the falling rate slows it little)
        assert!(time_to_dry(10.0, 10.0, e, a) < 10.05);
    }

    /// Standing water sinks as it evaporates: what sinks takes its share of
    /// the gum, and the sheet's water then dries up through the full pores
    /// before they do.
    #[test]
    fn water_sinks_with_its_gum_and_dries_out_of_the_sheet_first() {
        let (e, a) = (0.71, falling());
        // a fast sheet: 36 of 40 µm go in at once, 4/40 of the gum stays
        let fast = Some((1900.0, 1000.0));
        let (w, u, g) = settle(40.0, 0.0, 4.0, 1.0, 10.0, e, a, fast);
        assert!((w - 4.0).abs() < 1e-4 && (g - 0.1).abs() < 1e-3 && (u - (36.0 - 7.1)).abs() < 0.05, "{w} {u} {g}");
        // no sheet: it all evaporates, and keeps its gum
        let (w, u, g) = settle(40.0, 0.0, 4.0, 1.0, 10.0, e, a, None);
        assert!((w - 32.9).abs() < 1e-3 && u == 0.0 && g == 1.0, "{w} {u} {g}");
        // a full sheet takes no more
        let (_, u, g) = settle(40.0, 10.0, 4.0, 1.0, 1.0, e, a, Some((1900.0, 10.0)));
        assert!(u == 10.0 && g == 1.0, "{u} {g}");
        // the time it takes is what settle needs
        for (w, u, sh) in [(40.0, 0.0, fast), (40.0, 5.0, Some((3.2, 120.0))), (2.0, 30.0, Some((3.2, 120.0)))] {
            let t = settle_minutes(w, u, 4.0, e, a);
            let wet = settle(w, u, 4.0, 1.0, t * 0.999, e, a, sh);
            let dry = settle(w, u, 4.0, 1.0, t * 1.001, e, a, sh);
            assert!(wet.0 + wet.1 > 0.0 && dry.0 + dry.1 == 0.0, "{w} {u}: {wet:?} {dry:?}");
        }
        // the sheet's water rises into pores that aren't full
        let (w, u, _) = settle(1.0, 5.0, 4.0, 1.0, 1e-6, e, a, None);
        assert!((w - 4.0).abs() < 1e-3 && (u - 2.0).abs() < 1e-3, "{w} {u}");
    }

    /// On a gritted ground (`paper::Grit`) the binder decides where water
    /// goes, as it does for oil (`Paper::oil_capacity`): an acrylic coat is a
    /// waterproof film, so the sheet under it takes none and a flood stands
    /// in the tooth with its gum until it evaporates, binding the veil; a
    /// glue coat lets it through to the sheet at the sheet's own sizing, so
    /// an unsized sheet drinks the flood and its gum as bare paper does.
    #[test]
    fn an_acrylic_grit_coat_keeps_the_water_off_the_sheet() {
        use crate::paper::{Binder, Grit};
        let gritted = |binder: Binder| {
            let c = sheet(paper(true), None, 1.0);
            let mut p = *c.paper.as_ref().unwrap();
            p.grit = Some(Grit { size_um: 35.0, cover: 0.6, binder });
            let mut c = c;
            c.paper = Some(p);
            c
        };
        let bare = *sheet(paper(true), None, 1.0).paper.as_ref().unwrap();
        let (acrylic, glue) = (gritted(Binder::Acrylic), gritted(Binder::Glue));
        assert_eq!(sheet_capacity_um(acrylic.paper.as_ref().unwrap()), 0.0);
        assert_eq!(sheet_capacity_um(glue.paper.as_ref().unwrap()), sheet_capacity_um(&bare));
        assert!(sheet_capacity_um(&bare) > 100.0);
        let veil = |c: Canvas| {
            let mut c = c;
            let mut st = Stick::round([0.05, 0.05, 0.3], 0.8, 12.0);
            stroke(&mut c, &mut st, 100.0, 500.0, 900.0, 500.0, 0.3);
            wet_all(&mut c, 150.0);
            c.wait(0.25);
            let soaked: f32 = c.drawing.as_ref().unwrap().sheet.iter().sum();
            c.dry();
            (c, soaked)
        };
        let left_loose = |c: &Canvas| c.drawing.as_ref().unwrap().cells.iter().filter(|c| c.loose > 0.0 && c.bound == 0.0).count();
        let ((sealed, into_acrylic), (through, into_glue)) = (veil(acrylic), veil(glue));
        assert!(into_acrylic == 0.0 && into_glue > 0.0, "{into_acrylic} {into_glue}");
        assert!(bound(&sealed) > 0.0 && left_loose(&sealed) == 0, "{}", left_loose(&sealed));
        assert!(left_loose(&through) > 0 && bound(&through) < bound(&sealed), "{} {} vs {}", left_loose(&through), bound(&through), bound(&sealed));
    }

    #[test]
    fn water_darkens_by_the_share_of_the_pores_it_fills() {
        assert!((pore_index(1.0, 1.333) - 1.333).abs() < 1e-4 && (pore_index(0.0, 1.333) - 1.0).abs() < 1e-6);
        // (1 − e^−2.5)/(1 − e^−5): half full shows 0.924 of its darkening
        assert!(wet_share(0.0) == 0.0 && (wet_share(1.0) - 1.0).abs() < 1e-6 && (wet_share(0.5) - 0.9241).abs() < 1e-3);
        // Mie at 1.7 µm: ultramarine in water keeps 0.214 of its
        // scattering, whiting 0.414, vermilion 0.817
        assert!((crate::dry::scatter_in(1.5, 1.333) - 0.214).abs() < 0.002);
        assert!((crate::dry::scatter_in(1.59, 1.333) - 0.414).abs() < 0.002);
        assert!((crate::dry::scatter_in(3.0, 1.333) - 0.817).abs() < 0.002);
    }
}
