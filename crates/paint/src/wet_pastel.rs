//! Pastel with water: a stick dipped in water, a wet brush drawn through
//! laid pastel, ground pastel laid as a paste or wash with a brush. Source:
//! notes/research/wet_pastel.md (its sections cited as §n; [M] measured,
//! [D] derived here from measured values, [P] a practitioner's or museum's
//! account, [E] an estimate, each with the reason it is one).
//!
//! - **Water in the pores** (`Cell::water`, µm in the pastel's pores and
//!   standing on the paper) replaces the air between the particles. Their
//!   scattering falls with the index contrast against the pore phase (Mie,
//!   `dry::scatter_in`, at each cell's own mean particle index
//!   `Cell::index`), the pore phase a Bruggeman mixture of air and water by
//!   the share of the pores filled (§2). The dry colors are kept, so it
//!   lightens again as the water goes.
//! - **Where the water goes** (§4). The pastel's pores (r ≈ 1.5 µm, fully
//!   wetted) draw harder than the paper's (r ≈ 6 µm, sized), so they fill
//!   first and empty last. Water standing over them sinks into an
//!   absorbent sheet at the Lucas–Washburn rate (the angle from its sizing,
//!   `Paper::absorbent`), within milliseconds: on a hard-sized sheet
//!   (contact angle over 90°) it stays and evaporates. Evaporation is a
//!   still-air boundary layer's mass transfer of the vapour-pressure
//!   deficit at the studio's temperature and humidity, with the wet
//!   surface cooled to its energy balance (`evaporation_um_min`); once the
//!   standing water is gone it slows as the evaporation front recedes into
//!   the pastel (falling rate).
//! - **The gum** (§1, §3): the stick's gum dissolves in the water and sets
//!   again at the contacts as it dries. What sank into the sheet took its
//!   share of the gum with it, so the pastel is bound in the share of its
//!   gum it still holds (`Cell::gum`): repeated wettings redissolve and
//!   reset the same gum, and never bind more than it can. The gum left in
//!   the pores lowers their contrast a little once dry (Bruggeman again).
//! - **A wet stick** (`Stick::water`): its gum bridges dissolved, the
//!   soaked tip is held together by capillary pressure alone, so it is far
//!   softer (`wet_hardness`) and lays more for the same force; as a paste it
//!   smears rather than crumbles, and spreads its particles into the tooth.
//!   Its water spreads up the dry stick by capillarity (`tip_water`).
//! - **A wet brush** (`Canvas::wash_pastel`) holds water in the space a full
//!   load of paint takes between its hairs, and lays it, picks up the
//!   wetted loose pastel and lays it further on as it lays and picks up wet
//!   paint (`Held::capacity_mm3`, the tool's `lay`, `pickup` and `run`).

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::graphite::Cell;
use crate::pastel::{PARTICLE_UM, Section, Stick};

/// A pastel stick's porosity, and so its deposit's (a deposit is counted in
/// the stick's volume it wore off: `Cell::loose`). A Sennelier stick of
/// about 8.5 cm³ weighs about 9 g, ρ ≈ 1.05 g/cm³, against solids of about
/// 2.65 (calcite 2.71, kaolinite 2.6): 1 − 1.05/2.65 ≈ 0.6 [D]
/// (pastel_stick_tribology.md §1).
pub const POROSITY: f32 = 0.6;
/// The particles' size, µm: the measured constituents run 0.2–20 µm
/// (kaolin 0.2–5, whiting 1–20, earths 1–30; pastel media 1–20 µm measured,
/// pastel_stick_tribology.md §3). 3 µm [E]: a typical size inside that
/// range, since no size distribution of a stick was found.
pub const PARTICLE_D_UM: f32 = 3.0;
/// The stick's dry gum, a share of its mass: recipes dough 1 g of powder
/// with 0.3–0.5 mL of a 1:30 tragacanth solution, 1–2 % [D]
/// (pastel_stick_tribology.md §1; wet_pastel.md §1).
pub const GUM_SHARE: f32 = 0.015;
/// Densities, g/cm³: the solids (calcite 2.71, kaolinite 2.6 [M]) and dry
/// gum arabic (1.35–1.49 [M], wet_pastel.md §1).
const SOLID_DENSITY: f32 = 2.65;
const GUM_DENSITY: f32 = 1.4;
/// The refractive index of dry gum (gum arabic films 1.51–1.54 [M],
/// dry_pigment_optics.md §1) and of water.
const GUM_INDEX: f32 = 1.52;
const WATER_INDEX: f32 = 1.333;
/// A particle index where none was recorded (as `dry::refractive_index`).
pub const DEFAULT_INDEX: f32 = 1.9;
/// Water at 20 °C: surface tension (N/m) and viscosity (Pa·s) [M,
/// handbook].
const GAMMA: f32 = 0.0728;
const ETA: f32 = 1.002e-3;
/// The studio (the engine has no room conditions of its own): 20 °C (the
/// air sheet.rs assumes) and 50 % relative humidity, the middle of the
/// 40–60 % held for works on paper (wet_pastel.md §4).
pub const STUDIO_C: f64 = 20.0;
pub const STUDIO_RH: f64 = 0.5;
/// Convective heat transfer from a vertical surface (a sheet on an easel)
/// to still room air, W/m²K: 2.5, ISO 6946's for horizontal heat flow
/// [M]; and the water's emissivity, 0.95 [M], for the radiation it takes
/// from the room.
const H_CONV: f64 = 2.5;
const EMISSIVITY: f64 = 0.95;
/// The water contact angle of a sheet's sizing (105°: a good AKD or rosin
/// sizing of writing grades [M]) and of bare cellulose (0°: unsized paper
/// takes water in before an angle can be read [M]); a sheet is the Cassie
/// mixture of the two by `absorbent`, its unsized share (wet_pastel.md §4).
const SIZED_DEG: f32 = 105.0;
const CELLULOSE_DEG: f32 = 0.0;
/// How far up its end a dipped stick is soaked, mm: 1 [E], how deep the
/// hand dips it (`p:wet(share)` is the share of that soaked).
pub const DIP_MM: f32 = 1.0;
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

/// The share of a stick's pores its dry gum fills: gum mass over the
/// solids' (`GUM_SHARE`), by the densities, over the pores (about 0.02).
pub fn gum_pore_share() -> f32 {
    GUM_SHARE / (1.0 - GUM_SHARE) * (1.0 - POROSITY) * SOLID_DENSITY / (GUM_DENSITY * POROSITY)
}

/// The scattering of the pastel in `c` as a share of its dry scattering:
/// while wet, its pores the Bruggeman mixture of air and water by the share
/// filled (water fills the pastel's fine pores before the paper's); once
/// dry again, of air and the gum it still holds. Mie at the cell's mean
/// particle index (`dry::scatter_in`).
pub(crate) fn scatter_share(c: &Cell) -> f32 {
    let n = from_index(c.index);
    let med = if c.water > 0.0 {
        let pores = POROSITY * (c.loose + c.bound);
        pore_index(if pores > 0.0 { c.water / pores } else { 1.0 }, WATER_INDEX)
    } else {
        pore_index(from_gum(c.gum) * gum_pore_share(), GUM_INDEX)
    };
    crate::dry::scatter_in(n, med)
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
    let le = 2.15e-5 / d_vapour(STUDIO_C);
    H_CONV / (1.204 * 1006.0) * le.powf(-2.0 / 3.0)
}

/// Water evaporating from a wet surface in the studio, µm a minute: the
/// boundary layer's mass transfer of the vapour density deficit between
/// the surface (saturated at its own temperature) and the room's air, the
/// surface cooled until the latent heat it loses is what convection and
/// the room's radiation bring it (about 16 °C; 0.71 µm/min) [D].
pub fn evaporation_um_min() -> f32 {
    const M_R: f64 = 0.018_015 / 8.314_462;
    let ta = STUDIO_C;
    let tk = |t: f64| t + 273.15;
    let rho = |t: f64| p_sat(t) * M_R / tk(t);
    let air = STUDIO_RH * rho(ta);
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
    let d_eff = d_vapour(STUDIO_C) * (POROSITY as f64).powf(1.5);
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

/// The cosine of the water contact angle of a sheet whose unsized share is
/// `absorbent` (Cassie: the sized and bare fiber surface by their shares).
pub fn cos_contact(absorbent: f32) -> f32 {
    let a = absorbent.clamp(0.0, 1.0);
    a * CELLULOSE_DEG.to_radians().cos() + (1.0 - a) * SIZED_DEG.to_radians().cos()
}

/// The water a sheet draws in over `dt` minutes, µm: Lucas–Washburn, its
/// pores' radius their mean height t·ε/(1 − ε) (paper.rs), the depth
/// √(r γ cosθ t / 2η), times its porosity. None at 90° or more.
pub fn sink_um(p: &crate::paper::Paper, dt: f32) -> f32 {
    let c = cos_contact(p.absorbent);
    if c <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let eps = p.porosity.clamp(0.05, 0.95);
    let r = p.thick_um * 1e-6 * eps / (1.0 - eps);
    let d = r * GAMMA * c / (2.0 * ETA);
    eps * (d * dt * 60.0).sqrt() * 1e6
}

/// The soaked tip's hardness, MPa, of a stick `dry` MPa hard: its gum
/// bridges dissolved, it holds together by the capillary pressure of its
/// saturated pores alone, σ = 6 (1 − ε)/ε · γ/d (Schubert's capillary
/// state), and yields at about three times that (Tabor) [D]; never harder
/// than dry.
pub fn wet_hardness(dry: f32) -> f32 {
    let sigma = 6.0 * (1.0 - POROSITY) / POROSITY * GAMMA / (PARTICLE_D_UM * 1e-6);
    (3.0 * sigma * 1e-6).min(dry)
}

/// A paste's particles, dispersed by the water, lie this share of a dry
/// deposit's thickness (`PARTICLE_UM`): primary particles of
/// `PARTICLE_D_UM` cover 1.5/d of area per volume of solids, and a stick's
/// volume holds 1 − ε of solids, so a paste covers as particles
/// d/(1.5 (1 − ε)) thick, against the crumbs' 6 µm [D].
pub fn paste_thin() -> f32 {
    PARTICLE_D_UM / (1.5 * (1.0 - POROSITY)) / PARTICLE_UM
}

/// The water a dipped tip spreads up the stick by: Lucas–Washburn in its
/// pores, mm²/s (their radius twice the hydraulic radius ε d / 6(1 − ε);
/// the gum solution taken as water: a 3 % gum arabic solution is nearly
/// as fluid, a tragacanth gel far less, which is not modelled).
fn tip_spread_mm2_s() -> f32 {
    let r = 2.0 * POROSITY * PARTICLE_D_UM * 1e-6 / (6.0 * (1.0 - POROSITY));
    r * GAMMA / (2.0 * ETA) * 1e6
}

impl Stick {
    fn section_mm(&self) -> (f32, f32) {
        match self.section {
            Section::Round { d_mm } => (std::f32::consts::FRAC_PI_4 * d_mm * d_mm, std::f32::consts::PI * d_mm),
            Section::Square { side_mm } => (side_mm * side_mm, 4.0 * side_mm),
        }
    }

    /// The water a soaked tip holds, mm³: its pores over `DIP_MM` of it.
    pub fn tip_capacity_mm3(&self) -> f32 {
        POROSITY * self.section_mm().0 * DIP_MM
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
        let l = (DIP_MM * DIP_MM + tip_spread_mm2_s() * t * 60.0).sqrt();
        let (area, rim) = self.section_mm();
        let gone = evaporation_um_min() * 1e-3 * t * (area + rim * l);
        ((water - gone).max(0.0) * DIP_MM / l).max(0.0)
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
/// solution (`Cell::gum` from never wet to all of it held).
pub(crate) fn add_water(c: &mut Cell, um: f32) {
    if um <= 0.0 {
        return;
    }
    c.water += um;
    if c.gum == 0 {
        c.gum = to_gum(1.0);
    }
}

impl Canvas {
    /// `dt` minutes pass for the water in the pastel (`wait`): what stands
    /// over the pastel's pores sinks into an absorbent sheet (`sink_um`),
    /// taking its share of the dissolved gum; the rest evaporates
    /// (`water_after`); where it is gone the gum sets (`Cell::gum_set`).
    /// Pastel painted over since is sealed: its water is forgotten.
    /// Nothing without the switch or water.
    pub(crate) fn dry_pastel_water(&mut self, dt: f32) {
        if !self.wet_pastel || !(dt > 0.0) || !self.drawing.as_ref().is_some_and(|d| d.damp) {
            return;
        }
        let sink = self.paper.as_ref().map_or(0.0, |p| sink_um(p, dt));
        let (e, a) = (evaporation_um_min(), falling());
        let d = self.drawing.as_deref_mut().expect("damp");
        let mut damp = false;
        for (i, c) in d.cells.iter_mut().enumerate() {
            if c.water <= 0.0 {
                continue;
            }
            if self.film[i] > c.film + 1e-4 {
                c.water = 0.0;
                continue;
            }
            let before = (c.a, c.reflectance());
            let pores = POROSITY * (c.loose + c.bound);
            let sunk = (c.water - pores).max(0.0).min(sink);
            if sunk > 0.0 {
                let held = from_gum(c.gum) * (1.0 - sunk / c.water);
                c.gum = to_gum(held);
                c.water -= sunk;
            }
            let w = water_after(c.water, pores, dt, e, a);
            if w > 0.0 {
                c.water = w;
                damp = true;
            } else {
                c.gum_set(from_gum(c.gum));
            }
            self.px[i] = c.repaint(self.px[i], before);
        }
        d.damp = damp;
    }

    /// Minutes until the last water in the pastel is gone (`dry`).
    pub(crate) fn pastel_water_minutes(&self) -> f32 {
        if !self.wet_pastel {
            return 0.0;
        }
        let Some(d) = self.drawing.as_ref().filter(|d| d.damp) else { return 0.0 };
        let (e, a) = (evaporation_um_min(), falling());
        // (what sinks does so within a second: a minute's capacity)
        let sink = self.paper.as_ref().map_or(0.0, |p| sink_um(p, 1.0));
        let t = d.cells.iter().map(|c| {
            let pores = POROSITY * (c.loose + c.bound);
            time_to_dry(c.water - (c.water - pores).max(0.0).min(sink), pores, e, a)
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
    /// `load.run_mm`, as a paste (`paste_thin`). Bound pastel stays. Not
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
        // (pressed harder, the belly gives more: 0.5 to 1.5 of a middling
        // press's [E], as no measurement of a brush's water against its
        // pressure was found)
        let press = 0.5 + pressure.clamp(0.0, 1.0);
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
                for &i in &band {
                    let cl = &mut d.cells[i];
                    let before = (cl.a, cl.reflectance());
                    cl.add_pastel(q, col, per);
                    cl.note_pastel(per, np);
                    cl.loose += per;
                    pxs[i] = cl.repaint(pxs[i], before);
                }
                paste -= give;
                laid += give;
                d.sticks = true;
            }
            d.damp = true;
        }
        self.drawing = Some(d);
        self.px = pxs;
        self.film = film;
        load.paste_mm3 = paste * um_mm3;
        load.paste = col;
        load.paste_n = np;
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

    fn sized(c: Canvas) -> Canvas {
        let mut p = *c.paper.as_ref().unwrap();
        p.absorbent = 0.1;
        let mut c = c;
        c.paper = Some(p);
        c
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
        dried.wait(1.0);
        assert!(water(&dried) > 0.0, "still damp after a minute");
        dried.wait(120.0);
        assert_eq!(water(&dried), 0.0, "dry after two hours");
        assert!(!dried.drawing.as_ref().unwrap().damp);
        let dd = darkening(&dried, 200.0, 800.0, 450.0, 550.0);
        assert!(dd < 0.9 * db, "lighter dry than wet: {dd} vs {db}");
        // the tip's water spreads up the stick within seconds
        assert!(b.tip_water(1.0, 0.0) == 1.0 && b.tip_water(1.0, 1.0 / 60.0) < 0.2);
    }

    /// The same pastel wetted (without moving it) reads darker while wet,
    /// and once dry is lighter than wet and a little darker than it was
    /// dry (its gum in the pores); a high-index pigment darkens less.
    #[test]
    fn wetted_pastel_darkens_by_its_index_and_dries_back_nearly_to_dry() {
        let run = |n: f32| {
            let mut c = sized(paper(true));
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

    /// Dried wet pastel on a sized sheet keeps its gum and is bound by it:
    /// the finger moves less of it than of dry pastel, and none of fixed;
    /// a stick takes to it more, as to a fixed crust. On an absorbent sheet
    /// the water that sank took gum with it, so less is bound.
    #[test]
    fn dried_wet_pastel_is_bound_by_the_gum_it_kept() {
        let mut dry = sized(paper(true));
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
        // on the absorbent sheet the sunk water carried most of the gum away
        let mut soaked = paper(true);
        band(&mut soaked);
        wet_all(&mut soaked, 40.0);
        soaked.dry();
        let bound = |c: &Canvas| c.drawing.as_ref().unwrap().cells.iter().map(|c| c.bound).sum::<f32>();
        assert!(bound(&soaked) < 0.7 * bound(&gummed), "{} vs {}", bound(&soaked), bound(&gummed));
        // wetting again binds no more than the gum it holds
        let once = bound(&gummed);
        wet_all(&mut gummed, 40.0);
        gummed.dry();
        assert!((bound(&gummed) - once).abs() <= 1e-3 * once, "{} vs {once}", bound(&gummed));
        // over a full tooth, the dried wet passage takes more pastel than the dry one
        let (mut dry, mut gummed) = (sized(paper(true)), sized(paper(true)));
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

    /// A wet brush drawn out of a band of loose pastel onto bare paper
    /// carries it along its path as a paste: the paper past the band
    /// darkens along the path (and not beside it), and all of it is wet.
    /// Water alone on bare paper shows nothing.
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
        // water alone over bare paper
        let mut bare = paper(true);
        let before = bare.pixels().to_vec();
        bare.wash_pastel(40.0, 0.5, &[(250.0, 500.0), (700.0, 500.0)], &mut brush(50.0));
        assert_eq!(bare.pixels(), &before[..]);
        assert!(bare.pastel_water_at(500.0, 500.0) > 0.0);
        // (an absorbent sheet takes it at once)
        bare.wait(0.01);
        assert_eq!(bare.pastel_water_at(500.0, 500.0), 0.0);
        assert_eq!(bare.pixels(), &before[..]);
    }

    /// Ground pastel mixed with water on the brush (a paste) lays its color
    /// over bare paper, wet, and partly bound once dry.
    #[test]
    fn a_paste_on_the_brush_lays_its_color_wet() {
        let mut c = sized(paper(true));
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

    /// The water, index and gum in the pastel are saved and read back, and
    /// dry the same after; a file whose section is damaged is refused.
    #[test]
    fn a_checkpoint_keeps_the_water() {
        let mut c = sized(paper(true));
        band(&mut c);
        wet_all(&mut c, 20.0);
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut std::io::Cursor::new(&bytes)).unwrap();
        assert!(r.wet_pastel() && r.drawing.as_ref().unwrap().damp);
        assert_eq!(water(&r), water(&c));
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
        // a canvas without the switch writes no section
        let mut plain = paper(false);
        band(&mut plain);
        let mut b = Vec::new();
        plain.write_state(&mut b, "").unwrap();
        assert!(!b.windows(8).any(|w| w == WET_MARK.to_le_bytes()));
        assert!(!Canvas::read_state(&mut std::io::Cursor::new(&b)).unwrap().0.wet_pastel());
    }

    /// "WETP", as checkpoint.rs marks the section.
    const WET_MARK: u64 = 0x50544557;

    /// The derived rates: evaporation in still studio air under a
    /// millimeter a day to a few; a sized sheet takes no water, an
    /// absorbent one millimeters within a second; a soaked tip far softer.
    #[test]
    fn the_rates_follow_from_the_physics() {
        let e = evaporation_um_min();
        assert!((0.6..0.8).contains(&e), "{e} µm/min");
        let mut p = crate::paper::Paper::drawing(1);
        assert!(sink_um(&p, 1.0 / 60.0) > 1000.0);
        p.absorbent = 0.1;
        assert_eq!(sink_um(&p, 10.0), 0.0);
        assert!(wet_hardness(3.9) < 0.1 * 3.9 && wet_hardness(0.1) == 0.1);
        assert!((0.018..0.022).contains(&gum_pore_share()));
        assert!((0.8..0.9).contains(&paste_thin()));
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

    #[test]
    fn water_darkens_by_the_share_of_the_pores_it_fills() {
        assert!((pore_index(1.0, 1.333) - 1.333).abs() < 1e-4 && (pore_index(0.0, 1.333) - 1.0).abs() < 1e-6);
        let mid = pore_index(0.5, 1.333);
        assert!(mid > 1.12 && mid < 1.2, "{mid}");
        let r = [0.6, 0.4, 0.2];
        let s = |f: f32| crate::dry::scatter_in(1.5, pore_index(f, 1.333));
        let (half, full) = (km_scatter(r, s(0.5)), km_scatter(r, s(1.0)));
        for k in 0..3 {
            assert!(full[k] < half[k] && half[k] < r[k], "{k}");
        }
        // ultramarine in water keeps about a seventh of its scattering, vermilion most
        assert!((crate::dry::scatter_in(1.5, 1.333) - 0.137).abs() < 0.01);
        assert!(crate::dry::scatter_in(3.0, 1.333) > 0.8);
    }
}
