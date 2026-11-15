//! Drawing on the ground: graphite pencils and black chalk.
//!
//! The model, from first principles and kept simple:
//! - **A point dragged over the tooth.** The point rides on the local tops of
//!   the ground (the weave showing through it); pressure lets it bite down
//!   into the hollows. A light line catches only the tops and breaks up into
//!   the grain of the canvas; a heavy one fills in.
//! - **Hardness** is the ratio of clay to graphite in the lead. A hard pencil
//!   (2H) lays less, of a lighter gray, and stays sharp; a soft one (4B) lays
//!   more, darker and glossier, and blunts fast (lines widen as it wears).
//!   Black chalk (carbon black in clay) is a deep matte black, crumbly and
//!   broad.
//! - **Optics.** The deposit covers a fraction `a` of each pixel with flakes of
//!   reflectance `r`; the ground shows between them. It is written into the
//!   dry picture, so every later layer of paint composites over it by
//!   Kubelka–Munk (`Pigment::over`): a thin veil lets it show, body color
//!   hides it. Graphite doesn't take on wet paint.
//! - **Loose until painted over.** A kneaded eraser lifts it, better from the
//!   tops than the hollows and never quite all of it; fixative binds it so the
//!   eraser no longer lifts it; once paint has gone over it, it is sealed in
//!   the picture.
//!
//! A pastel stick (`crate::pastel`) lays its colored particles into the
//! same drawing: a cell's reflectance is RGB (gray for graphite and chalk,
//! the same arithmetic per channel) and it keeps the pastel in the tooth.
//!
//! The bookkeeping (`Drawing`) exists only on a canvas that has been drawn
//! on, so paintings without a drawing are unchanged; a drawing with no
//! stick pastel in it keeps its old layout (`to_f32s`) bit for bit.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::mask::Mask;
use crate::path::arclen;
use crate::rng::{Rng, hash2};
use crate::surface::vnoise;

/// What a point is made of.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Medium {
    Graphite,
    Chalk,
}

/// A drawing point: a graphite pencil of some grade, or black chalk.
#[derive(Clone, Debug)]
pub struct Lead {
    pub medium: Medium,
    /// Grade as softness: 2H = -2, H = -1, F = -0.5, HB = 0, B = 1, 4B = 4.
    pub soft: f32,
    /// Reflectance of the laid flakes (linear, gray).
    pub flake: f32,
    /// Coverage laid in one pass at full contact and pressure (fraction of
    /// what is still bare).
    pub rate: f32,
    /// Most of a pixel's area the deposit can cover.
    pub cap: f32,
    /// Width of the sharpened point, mm.
    pub point_mm: f32,
    /// Length of line (mm) over which the point blunts (width grows toward 3×).
    pub blunt_mm: f32,
    /// How deep into the tooth the point reaches at full pressure, µm.
    pub bite_um: f32,
    /// Irregularity of the deposit (0 smooth, 1 crumbly).
    pub crumble: f32,
}

/// Softness of a pencil grade: "2H" → -2, "HB" → 0, "F" → -0.5, "4B" → 4.
pub fn softness(grade: &str) -> Option<f32> {
    let g = grade.trim().to_ascii_uppercase();
    match g.as_str() {
        "HB" => return Some(0.0),
        "F" => return Some(-0.5),
        _ => {}
    }
    // "2H", "B": ASCII digits then H or B (anything else, accented letters
    // and emoji included, is not a grade)
    let (num, sign) = match g.strip_suffix('H') {
        Some(n) => (n, -1.0),
        None => (g.strip_suffix('B')?, 1.0),
    };
    if !num.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: f32 = if num.is_empty() { 1.0 } else { num.parse().ok()? };
    if !(1.0..=9.0).contains(&n) {
        return None;
    }
    Some(sign * n)
}

impl Lead {
    /// A graphite pencil of softness `soft` (see `softness`), -9 (9H) to 9 (9B).
    pub fn graphite(soft: f32) -> Lead {
        let s = soft.clamp(-9.0, 9.0);
        // share of graphite (vs clay) in the lead, 0..1
        let t = (s + 9.0) / 18.0;
        Lead {
            medium: Medium::Graphite,
            soft: s,
            // clay-rich hard leads lay a pale silvery gray; soft ones a dark
            // gray (graphite is never black: its sheen keeps it gray)
            flake: 0.30 - 0.25 * t.powf(0.8),
            rate: (0.8 * 1.1f32.powf(s)).min(1.0),
            cap: (0.64 + 0.035 * s).clamp(0.35, 0.92),
            point_mm: 0.32 + 0.04 * s.max(0.0),
            blunt_mm: 4000.0 * 0.72f32.powf(s),
            bite_um: 90.0 * 1.1f32.powf(s),
            crumble: 0.2 + 0.03 * s.max(0.0),
        }
    }

    /// A graphite pencil by grade name ("2H", "HB", "2B", ...).
    pub fn pencil(grade: &str) -> Option<Lead> {
        softness(grade).map(Lead::graphite)
    }

    /// Natural black chalk: carbon black in clay. Deep, matte, broad and
    /// crumbly; it wears fast.
    pub fn chalk() -> Lead {
        Lead { medium: Medium::Chalk, soft: 6.0, flake: 0.022, rate: 1.0, cap: 0.93, point_mm: 0.9, blunt_mm: 350.0, bite_um: 150.0, crumble: 0.5 }
    }

    /// Width of the line (mm) after `worn_mm` of drawing since sharpening.
    pub fn width_mm(&self, worn_mm: f32) -> f32 {
        self.point_mm * (1.0 + 2.0 * (1.0 - (-worn_mm.max(0.0) / self.blunt_mm).exp()))
    }

    /// How readily a kneaded eraser takes it (per pass, at the tops).
    fn lift(&self) -> f32 {
        match self.medium {
            Medium::Graphite => 0.9,
            Medium::Chalk => 0.72,
        }
    }
}

/// One pixel of the drawing's bookkeeping.
#[derive(Clone, Copy, Default)]
pub(crate) struct Cell {
    /// Fraction of the pixel covered.
    pub(crate) a: f32,
    /// Loose pastel reflectance with `optics`, otherwise the legacy mixed
    /// reflectance (linear RGB).
    pub(crate) r: Rgb,
    /// How readily an eraser lifts it (coverage-weighted).
    pub(crate) lift: f32,
    /// Fixed coverage: the eraser can't lift below this.
    pub(crate) floor: f32,
    /// The canvas's film (coats) when this was drawn: once paint has gone
    /// over it the film is thicker, and the drawing is sealed.
    pub(crate) film: f32,
    /// A pastel stick's pastel in the tooth, µm (volume per area as it
    /// lies, packed): loose, and bound by fixative (`crate::pastel`).
    pub(crate) loose: f32,
    pub(crate) bound: f32,
    /// The share of the loose and of the bound pastel (by volume) that is a
    /// bare bed of carbon: charcoal (`crate::charcoal`), seen with no
    /// first-surface veil (`Cell::bare_carbon`). 0 for pastel.
    pub(crate) carbon: f32,
    pub(crate) carbon_bound: f32,
    pub(crate) non_pastel: f32,
    pub(crate) non_pastel_floor: f32,
    pub(crate) optics: Option<Optics>,
    /// Water in the pastel's pores and standing on the paper here, µm
    /// (`crate::wet_pastel`; only with drawing optics): while it lasts it
    /// darkens the pastel, and as it dries the gum binds part of it.
    pub(crate) water: f32,
    /// The pastel's mean refractive index (`crate::wet_pastel::to_index`:
    /// 0 not recorded), by volume: how much water in its pores darkens it.
    pub(crate) index: u16,
    /// The share of its sticks' gum the pastel here still holds once it
    /// has been wet (`crate::wet_pastel::to_gum`: 0 never wet).
    pub(crate) gum: u16,
    /// The loose pastel's Kubelka–Munk scattering per µm of it as it lies,
    /// gray, µm⁻¹ (the volume-weighted mean of what was laid: its sticks'
    /// `Stick::scatter`), with the pastel K–M switch (`Canvas::
    /// with_pastel_km_mix`): what mixes it with more by Kubelka–Munk
    /// (`add_grains`). 0 not recorded.
    pub(crate) s: f32,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Optics {
    /// Fixed graphite/chalk, then removable graphite/chalk (both gray).
    pub(crate) fixed: f32,
    pub(crate) graphite: f32,
    pub(crate) bound: Rgb,
    /// The pixel before this drawing, retained to avoid repeated unmixing.
    pub(crate) under: Rgb,
}

impl Cell {
    pub(crate) fn restart(&mut self, film: f32, pixel: Rgb) {
        *self = Cell { film, optics: self.optics.map(|_| Optics { under: pixel, ..Optics::default() }), ..Cell::default() };
    }

    pub(crate) fn repaint(&self, pixel: Rgb, before: (f32, Rgb)) -> Rgb {
        let r = self.reflectance();
        if let Some(optics) = self.optics {
            return cover(optics.under, self.a, r);
        }
        cover(uncover(pixel, before.0, before.1), self.a, r)
    }

    pub(crate) fn reflectance(&self) -> Rgb {
        let Some(optics) = self.optics else { return self.r };
        let g = (self.non_pastel - self.non_pastel_floor).max(0.0);
        let b = 1.0 - (-self.bound / crate::pastel::PARTICLE_UM).exp();
        let l = 1.0 - (-self.loose / crate::pastel::PARTICLE_UM).exp();
        let base = (1.0 - b) * (1.0 - l);
        let total = self.non_pastel * base + b * (1.0 - l) + l;
        if total <= 0.0 { return [0.0; 3] }
        // (water in the pores darkens the pastel while it lasts, and the gum
        // it left once dry, a little: wet_pastel.rs)
        let (r, bound) = if self.water > 0.0 || self.gum > 0 {
            (crate::wet_pastel::look(self, self.r), crate::wet_pastel::look(self, optics.bound))
        } else {
            (self.r, optics.bound)
        };
        [0, 1, 2].map(|k| ((optics.fixed * self.non_pastel_floor + optics.graphite * g) * base + bound[k] * b * (1.0 - l) + r[k] * l) / total)
    }

    /// The water here has dried (`crate::wet_pastel`): the gum it held
    /// in solution sets again at the particles' contacts, binding the share
    /// `held` of the pastel here (`wet_pastel::binds`: all of it where the
    /// gum it kept holds against a finger, none where it doesn't); what is
    /// bound already (by fixative, or by the gum of an earlier wetting)
    /// stays bound. Its coverage in that share raises the floor the eraser
    /// stops at. Only with drawing optics.
    pub(crate) fn gum_set(&mut self, held: f32) {
        self.water = 0.0;
        if self.optics.is_none() || self.loose <= 0.0 {
            return;
        }
        let v = self.loose + self.bound;
        let b = (held.clamp(0.0, 1.0) * v - self.bound).clamp(0.0, self.loose);
        if b <= 0.0 {
            return;
        }
        let Some(mut optics) = self.optics else { return };
        let share = b / self.loose;
        optics.bound = [0, 1, 2].map(|k| (optics.bound[k] * self.bound + self.r[k] * b) / (self.bound + b));
        // (the carbon bound with it is bound carbon, as fixing makes it)
        if self.carbon > 0.0 || self.carbon_bound > 0.0 {
            self.carbon_bound = (self.carbon_bound * self.bound + self.carbon * b) / (self.bound + b);
        }
        let base = self.floor + self.non_pastel - self.non_pastel_floor;
        self.floor = (self.floor + (self.a - base).max(0.0) * share).min(self.a);
        self.bound += b;
        self.loose -= b;
        self.optics = Some(optics);
    }

    /// The share of the pastel here, loose and bound, that is bare carbon
    /// (charcoal), by volume: it brought no gum (`crate::wet_pastel`).
    pub(crate) fn carbon_share(&self) -> f32 {
        let v = self.loose + self.bound;
        if v <= 0.0 || (self.carbon <= 0.0 && self.carbon_bound <= 0.0) {
            return 0.0;
        }
        ((self.carbon * self.loose + self.carbon_bound * self.bound) / v).clamp(0.0, 1.0)
    }

    /// `added` µm of pastel of index `n` lands here (`crate::wet_pastel`),
    /// a share `carbon` of it bare carbon: the cell's mean index follows by
    /// volume, and once it has been wet, its gum share too (the new pastel
    /// brings all of its own, its carbon none: vine charcoal has no binder).
    /// Call before adding it to `loose`. Nothing for `n` 0 (no switch).
    pub(crate) fn note_pastel(&mut self, added: f32, n: f32, carbon: f32) {
        if n <= 0.0 || added <= 0.0 {
            return;
        }
        let old = (self.loose + self.bound).max(0.0);
        let w = old + added;
        let n0 = crate::wet_pastel::from_index(self.index);
        self.index = crate::wet_pastel::to_index((n0 * old + n * added) / w);
        if self.gum > 0 {
            let g = crate::wet_pastel::from_gum(self.gum);
            self.gum = crate::wet_pastel::to_gum((g * old + added * (1.0 - carbon.clamp(0.0, 1.0))) / w);
        }
    }

    #[cfg(test)]
    pub(crate) fn add_pastel(&mut self, q: f32, color: Rgb, volume: f32) {
        self.add_grains(q, color, volume, crate::pastel::Grains::PASTEL);
    }

    /// `add_pastel` of a deposit of grains `g`: the eraser takes it as
    /// readily as `g.lift`, and a share `g.carbon` of it is bare carbon.
    pub(crate) fn add_grains(&mut self, q: f32, color: Rgb, volume: f32, g: crate::pastel::Grains) {
        let a1 = (self.a + (1.0 - self.a) * q).min(0.995);
        let old = if self.optics.is_some() { self.loose } else { self.a * (1.0 - q) };
        let added = if self.optics.is_some() { volume } else { q };
        let w = old + added;
        if w > 0.0 || self.optics.is_none() {
            let w = if self.optics.is_some() { w } else { w.max(1e-6) };
            if g.s > 0.0 && self.optics.is_some() {
                // with the pastel K–M switch: particles, not paint films
                let s_old = if self.s > 0.0 { self.s } else { g.s };
                self.r = if g.layer {
                    layered(self.r, self.loose, color, volume, g.s)
                } else {
                    crate::dry::km_mix(self.r, old * s_old, color, added * g.s)
                };
                self.s = (old * s_old + added * g.s) / w;
            } else {
                self.r = [0, 1, 2].map(|k| ((old * self.r[k] + added * color[k]) / w).clamp(0.0, 1.0));
            }
            if self.carbon > 0.0 || g.carbon > 0.0 {
                self.carbon = ((old * self.carbon + added * g.carbon) / w).clamp(0.0, 1.0);
            }
        }
        self.lift = (self.a * (1.0 - q) * self.lift + q * g.lift) / (self.a * (1.0 - q) + q).max(1e-6);
        self.a = a1;
    }

    /// The particle bed at this cell, loose or fixed (pastel and charcoal
    /// alike: chalk, pigment and carbon particles), as it shows: the share
    /// of the pixel it covers and its reflectance (`reflectance`'s, its
    /// water's darkening with it). The picture's first-surface veil
    /// (`canvas::haze`) isn't over it, with the pastel veil switch
    /// (`Canvas::with_pastel_veil`): Mie's scattering of each particle
    /// already holds its surface's reflection, and the bed's R∞ is all it
    /// returns; a powder has no smooth surface for a second one (charcoal.md
    /// §2). Graphite's flakes and black chalk (`non_pastel`) keep the veil.
    /// None where there is none.
    pub(crate) fn bare_bed(&self) -> Option<(f32, Rgb)> {
        self.bed(None)
    }

    /// The bare carbon at this cell (charcoal, loose or fixed: `carbon`),
    /// `bare_bed`'s rule for carbon alone, as the charcoal switch had it
    /// before the pastel veil switch (its colors as laid, dry). None where
    /// there is none.
    pub(crate) fn bare_carbon(&self) -> Option<(f32, Rgb)> {
        if (self.carbon <= 0.0 && self.carbon_bound <= 0.0) || self.a <= 0.0 {
            return None;
        }
        let Some(_) = self.optics else { return Some((self.a * self.carbon, self.r)) };
        self.bed(Some((self.carbon_bound, self.carbon)))
    }

    /// The bed of `bare_bed`, or with `carbon` (its bound and loose shares)
    /// only that much of it, at its dry colors (`bare_carbon`).
    fn bed(&self, carbon: Option<(f32, f32)>) -> Option<(f32, Rgb)> {
        let optics = self.optics?;
        if self.a <= 0.0 {
            return None;
        }
        let b = 1.0 - (-self.bound / crate::pastel::PARTICLE_UM).exp();
        let l = 1.0 - (-self.loose / crate::pastel::PARTICLE_UM).exp();
        let base = (1.0 - b) * (1.0 - l);
        let total = self.non_pastel * base + b * (1.0 - l) + l;
        let (cb, cl) = carbon.unwrap_or((1.0, 1.0));
        let (wb, wl) = (b * (1.0 - l) * cb, l * cl);
        if total <= 0.0 || wb + wl <= 0.0 {
            return None;
        }
        let (r, bound) = if carbon.is_none() && (self.water > 0.0 || self.gum > 0) {
            (crate::wet_pastel::look(self, self.r), crate::wet_pastel::look(self, optics.bound))
        } else {
            (self.r, optics.bound)
        };
        Some((self.a * (wb + wl) / total, [0, 1, 2].map(|k| (bound[k] * wb + r[k] * wl) / (wb + wl))))
    }

    pub(crate) fn bind(&mut self, wet: f32) {
        self.bind_resin(wet, None);
    }

    /// With the fixative index switch: the loose pastel's color once
    /// `resin_um` of resin wets the share of its pores it fills
    /// (`crate::charcoal::wet_share`, its pores' volume against the resin),
    /// by one rule for every particle: over the wetted share K and S are
    /// those in resin, mixed with those in air by share, as
    /// `dry::CarbonBed::color` mixes them. A pigment's or chalk's K stays
    /// and its S falls by Mie at its own index against the resin's
    /// (`dry::scatter_in_resin` of the cell's index, `Cell::index`, its
    /// carbon's share taken out); carbon's K and S are its beds' in resin
    /// (`charcoal::resin_ratios`). The cell's K and S are split between
    /// pastel and carbon by its scattering (`Cell::s`, with the K–M switch)
    /// or else by volume.
    fn resin_dark(&self, resin_um: f32) -> Rgb {
        let ws = crate::charcoal::wet_share(resin_um, self.loose);
        if ws <= 0.0 {
            return self.r;
        }
        let c = self.carbon.clamp(0.0, 1.0);
        // (the cell's index counts its carbon at the index it was recorded with)
        let n = crate::wet_pastel::from_index(self.index);
        let cv = self.carbon_share();
        let n_p = if cv < 0.999 { (n - cv * crate::wet_pastel::DEFAULT_INDEX) / (1.0 - cv) } else { n };
        let f_sp = 1.0 - ws + ws * crate::dry::scatter_in_resin(n_p);
        let (kap, rho) = crate::charcoal::resin_ratios();
        let (k_c, s_c) = crate::charcoal::lying_ks();
        std::array::from_fn(|q| {
            let ks = crate::dry::ks_of(self.r[q]);
            let (ws_c, wk_c) = if c <= 0.0 {
                (0.0, 0.0)
            } else if self.s > 0.0 {
                ((c * s_c / self.s).min(1.0), (c * k_c[q] / (ks * self.s).max(1e-9)).min(1.0))
            } else {
                (c, c)
            };
            let f_sc = 1.0 - ws + ws * rho[q];
            let f_kc = 1.0 - ws + ws * kap[q];
            crate::dry::r_of(ks * (1.0 - wk_c * (1.0 - f_kc)) / ((1.0 - ws_c) * f_sp + ws_c * f_sc))
        })
    }

    /// `bind_resin` with the fixative index switch: the darkening by
    /// `resin_dark` of `resin_um` of resin.
    pub(crate) fn bind_by_index(&mut self, resin_um: f32) {
        let dark = self.resin_dark(resin_um);
        self.bind_dark(dark);
    }

    /// `bind`, a light spray laying `resin_um` µm of resin: where the
    /// loose layer is carbon (`carbon`), the resin wets the share of its
    /// pores it fills (`crate::charcoal::wet_share`) and darkens it as
    /// carbon's facets do against resin (`crate::charcoal::WET_KS`),
    /// not by pastel's rule. Pastel keeps `wet` and its rule.
    pub(crate) fn bind_resin(&mut self, wet: f32, resin_um: Option<f32>) {
        if self.optics.is_none() {
            return;
        }
        let l = self.loose;
        let carbon = if resin_um.is_some() { self.carbon } else { 0.0 };
        let wc = resin_um.map_or(0.0, |t| crate::charcoal::wet_share(t, l));
        let dark: Rgb = std::array::from_fn(|k| {
            let r = self.r[k];
            if carbon > 0.0 {
                // pastel's share by its rule, carbon's by its own (K/S scales)
                let r = r.clamp(1e-4, 0.9999);
                let f = (1.0 - carbon) / (1.0 - wet + wet / 5.0) + carbon * (1.0 - wc + wc * crate::charcoal::WET_KS[k]);
                let ks = (1.0 - r).powi(2) / (2.0 * r) * f;
                return 1.0 + ks - (ks * ks + 2.0 * ks).sqrt();
            }
            if wet == 0.0 { return r }
            let r = r.clamp(1e-4, 0.9999);
            let ks = (1.0 - r).powi(2) / (2.0 * r) / (1.0 - wet + wet / 5.0);
            1.0 + ks - (ks * ks + 2.0 * ks).sqrt()
        });
        self.bind_dark(dark);
    }

    /// Bind the loose pastel, darkened to `dark`, onto the bound.
    fn bind_dark(&mut self, dark: Rgb) {
        let Some(mut optics) = self.optics else { return };
        let g = (self.non_pastel - self.non_pastel_floor).max(0.0);
        let b = self.bound;
        let l = self.loose;
        if self.non_pastel > 0.0 {
            optics.fixed = (optics.fixed * self.non_pastel_floor + optics.graphite * g) / self.non_pastel;
        }
        for k in 0..3 {
            if b + l > 0.0 {
                optics.bound[k] = (optics.bound[k] * b + dark[k] * l) / (b + l);
            }
        }
        if b + l > 0.0 && (self.carbon > 0.0 || self.carbon_bound > 0.0) {
            self.carbon_bound = (self.carbon_bound * b + self.carbon * l) / (b + l);
        }
        self.optics = Some(optics);
        self.floor = self.a;
        self.non_pastel_floor = self.non_pastel;
        self.bound += self.loose;
        self.loose = 0.0;
    }

    pub(crate) fn free_bound(&mut self, floor: f32, freed: f32) {
        if let Some(optics) = self.optics {
            let l = self.loose;
            if l + freed > 0.0 {
                self.r = [0, 1, 2].map(|k| (self.r[k] * l + optics.bound[k] * freed) / (l + freed));
                if self.carbon > 0.0 || self.carbon_bound > 0.0 {
                    self.carbon = (self.carbon * l + self.carbon_bound * freed) / (l + freed);
                }
            }
        }
        self.floor = floor;
    }
}

/// The loose layer's color once `added` µm (as it lies) of particles of R∞
/// `color` and scattering `s` (per µm as it lies) land on top of `loose` µm
/// of loose pastel of color `under` (the pastel K–M switch's stroke,
/// `Grains::layer`). The tooth's layers cover by area as `Cell::reflectance`
/// counts them, 1 − e^(−v/`PARTICLE_UM`): where the new particles fall on the
/// loose pastel they lie over it as a layer (Kubelka's two-layer formula,
/// `dry::km_layer`, of their thickness where they cover); where they fall
/// beside it, they lie side by side with it (an area mix).
fn layered(under: Rgb, loose: f32, color: Rgb, added: f32, s: f32) -> Rgb {
    let p = crate::pastel::PARTICLE_UM;
    let l = 1.0 - (-loose.max(0.0) / p).exp();
    let q = 1.0 - (-added.max(0.0) / p).exp();
    let both = 1.0 - (1.0 - l) * (1.0 - q);
    if !(both > 0.0) || !(q > 0.0) {
        return under;
    }
    let over = crate::dry::km_layer(color, s * added / q, under);
    [0, 1, 2].map(|k| ((l * (1.0 - q) * under[k] + l * q * over[k] + (1.0 - l) * q * color[k]) / both).clamp(0.0, 1.0))
}

/// The loose drawing on a canvas: what was laid where, so it can be lifted,
/// and the guide: where the lines were drawn, over the whole canvas.
#[derive(Clone)]
pub struct Drawing {
    /// The deposit, per pixel of the window (the canvas's optical buffers).
    pub(crate) cells: Vec<Cell>,
    /// The drawn lines as geometry, per pixel of the whole canvas (not just
    /// the window of a crop render): the coverage each line would lay on a
    /// perfectly smooth ground (the lead's rate and cap at the pressure),
    /// with no tooth, no grain and no wet paint to skip, so it is continuous
    /// along the line and the same in a crop render and a whole one.
    guide: Vec<f32>,
    /// What fixative bound of the guide: the eraser can't lift below this.
    guide_floor: Option<Vec<f32>>,
    /// Whether a pastel stick's pastel has landed in the tooth (or the
    /// finger has moved some): the cells are then serialized with their
    /// color and the pastel in the tooth (`to_f32s`).
    pub(crate) sticks: bool,
    /// Whether charcoal has landed in the tooth (with `optics`): the cells
    /// then carry their carbon shares (`Cell::carbon`, layout 5).
    pub(crate) carbon: bool,
    optics: bool,
    /// Whether any cell holds water (`crate::wet_pastel`): a `wait` dries
    /// the drawing only then. Not written: read back from the cells.
    pub(crate) damp: bool,
    /// The water the sheet has taken in under each cell, µm (empty until
    /// water first sinks into it: `crate::wet_pastel`).
    pub(crate) sheet: Vec<f32>,
}

/// How much of the point touches at pressure `p`: none at 0, rising
/// smoothly to all of it at 0.12 and above (a line laid with no pressure
/// lays nothing; a lifting stroke fades out instead of stopping short).
pub fn touch(p: f32) -> f32 {
    let t = (p / 0.12).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// What the eraser takes of the guide per pass at full strength.
const GUIDE_LIFT: f32 = 0.85;

/// How far below zero loose pastel may read back, µm: what float rounding
/// leaves when a dry tool takes all of it from over fixed pastel on a drawing
/// without separate optics (`take_loose` in sheet.rs: about 1e-7 of the
/// cell's pastel), a thousandth of a micron, far less than a particle.
const LOOSE_RESIDUE_UM: f32 = 1e-3;

/// A line to draw: a dense path (units) with the pressure at every point.
#[derive(Clone, Debug, Default)]
pub struct Mark {
    pub pts: Vec<(f32, f32)>,
    pub pressure: Vec<f32>,
}

impl Mark {
    /// Length in units.
    pub fn length(&self) -> f32 {
        self.pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum()
    }
}

/// Catmull–Rom through `pts` (or straight between them when `!smooth`),
/// resampled every `step` units.
pub fn resample(pts: &[(f32, f32)], smooth: bool, step: f32) -> Vec<(f32, f32)> {
    if pts.len() < 2 {
        return pts.to_vec();
    }
    let step = step.max(0.01);
    let n = pts.len();
    let at = |i: isize| pts[i.clamp(0, n as isize - 1) as usize];
    let mut out = vec![pts[0]];
    for i in 0..n - 1 {
        let (p0, p1, p2, p3) = (at(i as isize - 1), at(i as isize), at(i as isize + 1), at(i as isize + 2));
        let len = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
        let k = ((len / step).ceil() as usize).max(1);
        for j in 1..=k {
            let t = j as f32 / k as f32;
            let p = if smooth {
                let (t2, t3) = (t * t, t * t * t);
                let f = |a: f32, b: f32, c: f32, d: f32| 0.5 * (2.0 * b + (c - a) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2 + (3.0 * b - a - 3.0 * c + d) * t3);
                (f(p0.0, p1.0, p2.0, p3.0), f(p0.1, p1.1, p2.1, p3.1))
            } else {
                (p1.0 + (p2.0 - p1.0) * t, p1.1 + (p2.1 - p1.1) * t)
            };
            out.push(p);
        }
    }
    out
}

/// Unit normal of a path at point `i`.
fn normal(p: &[(f32, f32)], i: usize) -> (f32, f32) {
    let (a, b) = (p[i.saturating_sub(1)], p[(i + 1).min(p.len() - 1)]);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l = (dx * dx + dy * dy).sqrt().max(1e-6);
    (-dy / l, dx / l)
}

/// Pressure along a path of `n` points from a profile given at evenly spaced
/// stations (one value: even; two: start to end; more: through them).
pub fn pressure_along(profile: &[f32], s: &[f32]) -> Vec<f32> {
    let total = s.last().copied().unwrap_or(0.0).max(1e-6);
    s.iter()
        .map(|&si| match profile.len() {
            0 => 0.5,
            1 => profile[0],
            m => {
                let u = (si / total).clamp(0.0, 1.0) * (m - 1) as f32;
                let k = (u.floor() as usize).min(m - 2);
                profile[k] + (profile[k + 1] - profile[k]) * (u - k as f32)
            }
        })
        .map(|p| p.clamp(0.0, 1.0))
        .collect()
}

/// A line through `pts`: smoothed (unless `ruler` or `!smooth`), displaced
/// sideways by a small tremor (`tremor` units; none with `ruler`), with the
/// pressure profile along it.
pub fn hand_line(pts: &[(f32, f32)], profile: &[f32], smooth: bool, ruler: bool, tremor: f32, seed: u64) -> Mark {
    let step = 0.25;
    let mut p = resample(pts, smooth && !ruler, step);
    if !ruler && tremor > 0.0 {
        let s = arclen(&p);
        let q = p.clone();
        for i in 0..p.len() {
            let n = normal(&q, i);
            // a slow sway (30-unit scale) and a finer tremor (4-unit scale)
            let w = tremor * (1.4 * (vnoise(s[i] / 30.0, 0.5, seed) - 0.5) + 0.6 * (vnoise(s[i] / 4.0, 3.5, seed + 1) - 0.5));
            p[i] = (q[i].0 + n.0 * w, q[i].1 + n.1 * w);
        }
    }
    let s = arclen(&p);
    let pressure = pressure_along(profile, &s);
    Mark { pts: p, pressure }
}

/// Repeated passes over one path: `passes` light strokes along it, each
/// displaced sideways by amounts on the order of `wander` units, starting
/// and ending a few percent of the length short of or past the ends, and
/// broken into more pieces the longer the path (about one per 120-400
/// units); the first pass at 0.75 of `pressure`.
pub fn sketch_marks(pts: &[(f32, f32)], pressure: f32, passes: usize, wander: f32, smooth: bool, tremor: f32, seed: u64) -> Vec<Mark> {
    let non_pastel = resample(pts, smooth, 0.25);
    let s = arclen(&non_pastel);
    let total = s.last().copied().unwrap_or(0.0);
    if total <= 0.0 {
        return Vec::new();
    }
    let mut rng = Rng::new(seed);
    let mut out = Vec::new();
    for k in 0..passes.max(1) {
        let ps = seed.wrapping_add(k as u64 * 7919);
        let amp = wander * rng.range(0.6, 1.2);
        // each pass has its own sideways offset and bow
        let side = wander * rng.range(-0.6, 0.6);
        let press = pressure * if k == 0 { 0.75 } else { rng.range(0.85, 1.15) };
        // start and end: short of the ends or past them
        let a0 = rng.range(-0.04, 0.06) * total;
        let a1 = total + rng.range(-0.06, 0.05) * total;
        // lift-offs: a long line is broken into a few strokes
        let pieces = 1 + (total / 120.0 * rng.range(0.3, 1.2)) as usize;
        let mut cuts: Vec<f32> = (1..pieces).map(|j| a0 + (a1 - a0) * (j as f32 + rng.range(-0.25, 0.25)) / pieces as f32).collect();
        cuts.insert(0, a0);
        cuts.push(a1);
        for w in cuts.windows(2) {
            let (lo, hi) = (w[0] + rng.range(0.0, 0.015) * total, w[1]);
            if hi - lo < 0.5 {
                continue;
            }
            // points along the non_pastel path (extended straight past its ends)
            let n = ((hi - lo) / 0.25).ceil() as usize + 1;
            let mut p = Vec::with_capacity(n);
            for j in 0..n {
                let a = lo + (hi - lo) * j as f32 / (n - 1) as f32;
                let (pt, nm) = point_at(&non_pastel, &s, a);
                let d = side + amp * (2.0 * vnoise(a / 45.0, k as f32 * 3.1, ps) - 1.0) + tremor * (vnoise(a / 4.0, 1.7, ps + 1) - 0.5);
                p.push((pt.0 + nm.0 * d, pt.1 + nm.1 * d));
            }
            let sp = arclen(&p);
            // each stroke swells in and lifts off
            let pr = pressure_along(&[press * 0.5, press, press * rng.range(0.8, 1.1), press * 0.4], &sp);
            out.push(Mark { pts: p, pressure: pr });
        }
    }
    out
}

/// The point at arc length `a` of a path (extended straight past its ends)
/// and the normal there.
fn point_at(p: &[(f32, f32)], s: &[f32], a: f32) -> ((f32, f32), (f32, f32)) {
    let n = p.len();
    let k = match s.iter().position(|&v| v >= a) {
        Some(0) => 1,
        Some(k) => k,
        None => n - 1,
    };
    let (p0, p1) = (p[k - 1], p[k]);
    let seg = (s[k] - s[k - 1]).max(1e-6);
    let t = (a - s[k - 1]) / seg;
    let pt = (p0.0 + (p1.0 - p0.0) * t, p0.1 + (p1.1 - p0.1) * t);
    (pt, normal(p, k))
}

/// Hatching: short parallel strokes across `m` (where it is > 0.5) at
/// `angle`, `spacing` units apart, each at most `length` units, bowed by up
/// to 6% of its length, heavy where it starts and lifting off at the end.
pub fn hatch_marks(m: &Mask, angle: f32, spacing: f32, length: f32, pressure: f32, seed: u64) -> Vec<Mark> {
    hatch_marks_graded(m, angle, spacing, length, pressure, false, seed)
}

/// `hatch_marks`, and with `graded` the mask is a weight, not a shape:
/// strokes run wherever it is above a trace (0.04), each pressed at every
/// point in proportion to the mask's value there, so a hatched passage
/// fades into the next instead of stopping at a line.
pub fn hatch_marks_graded(m: &Mask, angle: f32, spacing: f32, length: f32, pressure: f32, graded: bool, seed: u64) -> Vec<Mark> {
    hatch_marks_swung(m, angle, spacing, length, pressure, graded, seed, None)
}

/// The hand that swings a mark: each stroke an arc about the wrist, or
/// about the elbow when it sweeps further than the wrist turns, its middle
/// bulging away from the pivot (`area_strokes`, with `p:hatch`'s
/// `--@ pastel-area-strokes`). Lengths in units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pivot {
    /// The wrist's radius: the hand's length.
    pub wrist: f32,
    /// How far the wrist sweeps the tip: its radius times its turn.
    pub reach: f32,
    /// The elbow's radius: forearm and hand.
    pub elbow: f32,
    /// The way to the pivot from the tip (a unit vector).
    pub toward: (f32, f32),
}

impl Pivot {
    /// The bow of a mark of length `len` along the unit direction `dir`:
    /// its middle's offset from the chord along the normal (-dir.1, dir.0).
    /// The share the wrist swings is the chord's part along the circle about
    /// the pivot, `len·|c|` where c is the normal's cosine to the radius; a
    /// sweep `len·|c|` on a radius R sags (len·c)²/(8R) along the radius,
    /// |c| of it across the chord, away from the pivot: -c³·len²/(8R). A mark
    /// along the radius (made by the fingers and arm, not the wrist's turn)
    /// is straight.
    pub fn bow(&self, dir: (f32, f32), len: f32) -> f32 {
        let c = -dir.1 * self.toward.0 + dir.0 * self.toward.1;
        let r = if len * c.abs() <= self.reach { self.wrist } else { self.elbow };
        -c * c * c * len * len / (8.0 * r.max(1e-6))
    }
}

/// `hatch_marks_graded`, each mark bowed by `pivot`'s swing instead of a
/// random bow (the same draws placing the marks).
#[allow(clippy::too_many_arguments)]
pub fn hatch_marks_swung(m: &Mask, angle: f32, spacing: f32, length: f32, pressure: f32, graded: bool, seed: u64, pivot: Option<&Pivot>) -> Vec<Mark> {
    let level = if graded { 0.04 } else { 0.5 };
    let f = m.f;
    let (w, h) = (f.width(), f.height());
    let (ca, sa) = (angle.cos(), angle.sin());
    // bounding box of the mask (units)
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for (i, &v) in m.data.iter().enumerate() {
        if v > level {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
    }
    if x0 > x1 {
        return Vec::new();
    }
    // along (u) and across (v) the strokes
    let corners = [(x0, y0), (x1, y0), (x0, y1), (x1, y1)];
    let (mut v0, mut v1, mut u0, mut u1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for &(x, y) in &corners {
        let (u, v) = (x * ca + y * sa, -x * sa + y * ca);
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    let spacing = spacing.max(0.2);
    let length = length.max(spacing);
    let mut rng = Rng::new(seed);
    let step = (0.5 / f.scale).max(0.2);
    let inside = |u: f32, v: f32| {
        let (x, y) = (u * ca - v * sa, u * sa + v * ca);
        x >= 0.0 && y >= 0.0 && x < w && y < h && m.sample(x, y) > level
    };
    let mut out = Vec::new();
    let mut v = v0 + rng.range(0.0, spacing);
    while v <= v1 {
        let vv = v + rng.range(-0.18, 0.18) * spacing;
        // runs inside the mask along this line
        let mut u = u0;
        let mut run: Option<f32> = None;
        let mut runs = Vec::new();
        while u <= u1 + step {
            let ins = u <= u1 && inside(u, vv);
            match (run, ins) {
                (None, true) => run = Some(u),
                (Some(a), false) => {
                    runs.push((a, u - step));
                    run = None;
                }
                _ => {}
            }
            u += step;
        }
        for (a, b) in runs {
            // split long runs into strokes that overlap or leave small gaps
            let n = ((b - a) / length).ceil().max(1.0) as usize;
            for j in 0..n {
                let (sa_, sb) = (a + (b - a) * j as f32 / n as f32, a + (b - a) * (j + 1) as f32 / n as f32);
                let (sa_, sb) = (sa_ + rng.range(-0.08, 0.1) * length, sb + rng.range(-0.1, 0.06) * length);
                let (sa_, sb) = (sa_.max(a - 0.3 * spacing), sb.min(b + 0.3 * spacing));
                if sb - sa_ < 0.4 * spacing {
                    continue;
                }
                let bow = rng.range(-0.06, 0.06) * (sb - sa_);
                let bow = match pivot {
                    Some(pv) => pv.bow((ca, sa), sb - sa_),
                    None => bow,
                };
                let k = ((sb - sa_) / 0.25).ceil() as usize + 1;
                let pts: Vec<(f32, f32)> = (0..k)
                    .map(|i| {
                        let t = i as f32 / (k - 1) as f32;
                        let uu = sa_ + (sb - sa_) * t;
                        let off = vv + bow * 4.0 * t * (1.0 - t);
                        (uu * ca - off * sa, uu * sa + off * ca)
                    })
                    .collect();
                let p = pressure * rng.range(0.85, 1.12);
                let s = arclen(&pts);
                let mut pr = pressure_along(&[p * 0.8, p, p * 0.8, p * 0.25], &s);
                if graded {
                    // pressed as hard as the weight there asks
                    for (q, pt) in pr.iter_mut().zip(&pts) {
                        let (x, y) = (pt.0.clamp(0.0, w - 1e-3), pt.1.clamp(0.0, h - 1e-3));
                        *q *= m.sample(x, y).clamp(0.0, 1.0);
                    }
                    if pr.iter().cloned().fold(0.0, f32::max) < 0.03 {
                        continue;
                    }
                }
                out.push(Mark { pressure: pr, pts });
            }
        }
        v += spacing;
    }
    out
}

impl Drawing {
    /// Every cell (with its carbon shares and wet pastel water, index and
    /// gum), the guide and the sheet's water, bit for bit (a canvas put away packed keeps
    /// its drawing on disk with it: checkpoint.rs `Aside::spill`), unlike the
    /// checkpoint's layouts, which keep only what their reader needs.
    pub(crate) fn spill(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut b = Vec::with_capacity(1 << 16);
        let put = |b: &mut Vec<u8>, v: f32| b.extend_from_slice(&v.to_bits().to_le_bytes());
        b.extend_from_slice(&(self.cells.len() as u64).to_le_bytes());
        b.extend_from_slice(&[self.sticks as u8, self.optics as u8, self.carbon as u8, self.damp as u8]);
        for c in &self.cells {
            for v in [c.a, c.r[0], c.r[1], c.r[2], c.lift, c.floor, c.film, c.loose, c.bound, c.non_pastel, c.non_pastel_floor, c.carbon, c.carbon_bound, c.water] {
                put(&mut b, v);
            }
            // (the wet pastel switch's index and gum, two u16s in a word; the
            // K–M switch's scattering)
            b.extend_from_slice(&(u32::from(c.index) | u32::from(c.gum) << 16).to_le_bytes());
            put(&mut b, c.s);
            b.push(c.optics.is_some() as u8);
            if let Some(o) = c.optics {
                for v in [o.fixed, o.graphite, o.bound[0], o.bound[1], o.bound[2], o.under[0], o.under[1], o.under[2]] {
                    put(&mut b, v);
                }
            }
            if b.len() >= 1 << 16 {
                w.write_all(&b)?;
                b.clear();
            }
        }
        w.write_all(&b)?;
        crate::checkpoint::spill_vec(w, Some(&self.guide))?;
        crate::checkpoint::spill_vec(w, self.guide_floor.as_deref())?;
        crate::checkpoint::spill_vec(w, Some(&self.sheet))
    }

    /// `spill` read back.
    pub(crate) fn unspill(r: &mut impl std::io::Read) -> std::io::Result<Drawing> {
        let mut u = [0u8; 8];
        r.read_exact(&mut u)?;
        let n = u64::from_le_bytes(u) as usize;
        let mut flags = [0u8; 4];
        r.read_exact(&mut flags)?;
        let at = |b: &[u8], i: usize| f32::from_bits(u32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]));
        let mut cells = Vec::with_capacity(n);
        let (mut q, mut o) = ([0u8; 65], [0u8; 32]);
        for _ in 0..n {
            r.read_exact(&mut q)?;
            let optics = if q[64] == 1 {
                r.read_exact(&mut o)?;
                Some(Optics { fixed: at(&o, 0), graphite: at(&o, 1), bound: [at(&o, 2), at(&o, 3), at(&o, 4)], under: [at(&o, 5), at(&o, 6), at(&o, 7)] })
            } else {
                None
            };
            cells.push(Cell { a: at(&q, 0), r: [at(&q, 1), at(&q, 2), at(&q, 3)], lift: at(&q, 4), floor: at(&q, 5), film: at(&q, 6), loose: at(&q, 7), bound: at(&q, 8), non_pastel: at(&q, 9), non_pastel_floor: at(&q, 10), carbon: at(&q, 11), carbon_bound: at(&q, 12), water: at(&q, 13), index: u16::from_le_bytes([q[56], q[57]]), gum: u16::from_le_bytes([q[58], q[59]]), s: at(&q, 15), optics });
        }
        let guide = crate::checkpoint::unspill_vec(r)?.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "a spilled drawing without its guide"))?;
        let guide_floor = crate::checkpoint::unspill_vec(r)?;
        let sheet = crate::checkpoint::unspill_vec(r)?.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "a spilled drawing without its sheet's water"))?;
        Ok(Drawing { cells, guide, guide_floor, sticks: flags[0] == 1, optics: flags[1] == 1, carbon: flags[2] == 1, damp: flags[3] == 1, sheet })
    }

    /// The bytes its buffers hold (what a canvas put aside keeps of it in
    /// memory: checkpoint.rs `Aside::heap_bytes`).
    pub(crate) fn heap_bytes(&self) -> usize {
        self.cells.capacity() * std::mem::size_of::<Cell>() + self.guide.capacity() * 4 + self.guide_floor.as_ref().map_or(0, |g| g.capacity() * 4) + self.sheet.capacity() * 4
    }

    fn new(n: usize, whole: usize, optics: bool) -> Self {
        Drawing { cells: vec![Cell { film: -1.0, optics: optics.then_some(Optics::default()), ..Cell::default() }; n], guide: vec![0.0; whole], guide_floor: None, sticks: false, carbon: false, optics, damp: false, sheet: Vec::new() }
    }

    /// The serialized layout (the checkpoint's drawing flag): 1 graphite
    /// and chalk only, 3 with pastel and the non-pastel coverage (`sticks`),
    /// 4 with separate retained and removable reflectances (`optics`), 5
    /// that with charcoal's carbon shares (`carbon`).
    pub(crate) fn layout(&self) -> u64 {
        if self.optics && self.carbon { 5 } else if self.optics { 4 } else if self.sticks { 3 } else { 1 }
    }

    /// Whether the drawing has optics (layout 4 or 5).
    pub(crate) fn optics(&self) -> bool {
        self.optics
    }

    /// Serialized as f32s: the window's cells (a, r, lift, floor, film; with
    /// stick pastel, layout 3: a, r, g, b, lift, floor, film, loose, bound,
    /// non_pastel and non_pastel_floor; layout 4 adds fixed and removable
    /// graphite's gray means, bound pastel's RGB mean and the underlying RGB;
    /// layout 5 then the loose and bound carbon shares),
    /// the guide (whole canvas), then 0 or 1 and the guide's floor.
    pub(crate) fn to_f32s(&self) -> impl Iterator<Item = f32> + '_ {
        let sticks = self.sticks || self.optics;
        let optics = self.optics;
        let carbon = self.optics && self.carbon;
        let cells = self.cells.iter().flat_map(move |c| {
            let q = if sticks { [c.a, c.r[0], c.r[1], c.r[2], c.lift, c.floor, c.film, c.loose, c.bound, c.non_pastel, c.non_pastel_floor] } else { [c.a, c.r[0], c.lift, c.floor, c.film, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
            let o = c.optics.unwrap_or_default();
            q.into_iter().take(if sticks { 11 } else { 5 }).chain([o.fixed, o.graphite, o.bound[0], o.bound[1], o.bound[2], o.under[0], o.under[1], o.under[2]].into_iter().take(if optics { 8 } else { 0 })).chain([c.carbon, c.carbon_bound].into_iter().take(if carbon { 2 } else { 0 }))
        });
        let floor = std::iter::once(if self.guide_floor.is_some() { 1.0 } else { 0.0 }).chain(self.guide_floor.iter().flatten().copied());
        cells.chain(self.guide.iter().copied()).chain(floor)
    }

    /// The inverse of `to_f32s`, reading with `get(count)`; `n` window
    /// pixels, `whole` canvas pixels, `layout` the serialized layout (1, 2
    /// through 5; layout 2 predates separate non-pastel coverage). None if the data is inconsistent.
    /// Loose pastel may sit a rounding residue below zero (`LOOSE_RESIDUE_UM`):
    /// it is kept as written.
    pub(crate) fn from_f32s<E>(n: usize, whole: usize, layout: u64, mut get: impl FnMut(usize) -> Result<Vec<f32>, E>) -> Result<Option<Self>, E> {
        let sticks = layout >= 2;
        let cells: Vec<Cell> = if layout == 5 {
            get(n * 21)?.as_chunks::<21>().0.iter().map(|q| Cell { a: q[0], r: [q[1], q[2], q[3]], lift: q[4], floor: q[5], film: q[6], loose: q[7], bound: q[8], non_pastel: q[9], non_pastel_floor: q[10], optics: Some(Optics { fixed: q[11], graphite: q[12], bound: [q[13], q[14], q[15]], under: [q[16], q[17], q[18]] }), carbon: q[19], carbon_bound: q[20], water: 0.0, index: 0, gum: 0, s: 0.0 }).collect()
        } else if layout == 4 {
            get(n * 19)?.as_chunks::<19>().0.iter().map(|q| Cell { a: q[0], r: [q[1], q[2], q[3]], lift: q[4], floor: q[5], film: q[6], loose: q[7], bound: q[8], non_pastel: q[9], non_pastel_floor: q[10], optics: Some(Optics { fixed: q[11], graphite: q[12], bound: [q[13], q[14], q[15]], under: [q[16], q[17], q[18]] }), water: 0.0, index: 0, gum: 0, ..Cell::default()}).collect()
        } else if layout == 3 {
            get(n * 11)?.as_chunks::<11>().0.iter().map(|q| Cell { a: q[0], r: [q[1], q[2], q[3]], lift: q[4], floor: q[5], film: q[6], loose: q[7], bound: q[8], non_pastel: q[9], non_pastel_floor: q[10], ..Cell::default() }).collect()
        } else if sticks {
            get(n * 9)?.as_chunks::<9>().0.iter().map(|q| Cell { a: q[0], r: [q[1], q[2], q[3]], lift: q[4], floor: q[5], film: q[6], loose: q[7], bound: q[8], non_pastel: if q[7] == 0.0 && q[8] == 0.0 { q[0] } else if q[8] == 0.0 { q[5] } else { 0.0 }, non_pastel_floor: if q[8] == 0.0 { q[5] } else { 0.0 }, ..Cell::default() }).collect()
        } else {
            get(n * 5)?.as_chunks::<5>().0.iter().map(|q| Cell { a: q[0], r: [q[1]; 3], lift: q[2], floor: q[3], film: q[4], non_pastel: q[0], non_pastel_floor: q[3], ..Cell::default() }).collect()
        };
        let guide = get(whole)?;
        let has_floor = get(1)?[0];
        let guide_floor = if has_floor == 1.0 { Some(get(whole)?) } else { None };
        let ok = |v: f32| v.is_finite() && (0.0..=1.0).contains(&v);
        let valid = (has_floor == 0.0 || has_floor == 1.0)
            && cells.iter().all(|c| ok(c.a) && c.r.iter().all(|&v| ok(v)) && ok(c.lift) && ok(c.floor) && ok(c.non_pastel) && ok(c.non_pastel_floor) && c.film.is_finite() && c.loose.is_finite() && c.loose > -LOOSE_RESIDUE_UM && c.bound.is_finite() && c.bound >= 0.0 && ok(c.carbon) && ok(c.carbon_bound))
            && guide.iter().chain(guide_floor.iter().flatten()).all(|&v| ok(v));
        let valid = valid && cells.iter().all(|c| c.optics.is_none_or(|o| ok(o.fixed) && ok(o.graphite) && o.bound.into_iter().all(ok) && o.under.into_iter().all(f32::is_finite) && c.non_pastel_floor <= c.non_pastel && c.non_pastel_floor <= c.floor && c.floor + c.non_pastel - c.non_pastel_floor <= c.a + 1e-5));
        Ok(valid.then_some(Drawing { cells, guide, guide_floor, sticks, carbon: layout == 5, optics: layout >= 4, damp: false, sheet: Vec::new() }))
    }
}

/// Lay a mark's line into the guide (a whole-canvas buffer of frame `wf`):
/// the path with its width `wid` (units; at least two pixels, so it samples
/// as an unbroken line), laying what `lead` lays on a smooth ground.
fn guide_line(g: &mut [f32], wf: crate::canvas::Frame, lead: &Lead, pts: &[(f32, f32)], wid: &[f32], pressure: &[f32]) {
    let pxu = 1.0 / wf.scale;
    // (pixel, coverage, pressure); one pass per pixel, the nearest
    let mut hits: Vec<(usize, f32, f32)> = Vec::new();
    for i in 0..pts.len() - 1 {
        let (a, b) = (pts[i], pts[i + 1]);
        let (ta, tb) = (touch(pressure[i]), touch(pressure[i + 1]));
        if ta <= 0.0 && tb <= 0.0 {
            continue;
        }
        let hw = 0.5 * wid[i].max(wid[i + 1]).max(2.0 * pxu) + pxu;
        let to_px = |u: f32, n: usize| ((u * wf.scale).floor().max(0.0) as usize).min(n);
        let (px0, px1) = (to_px(a.0.min(b.0) - hw, wf.w), (to_px(a.0.max(b.0) + hw, wf.w) + 1).min(wf.w));
        let (py0, py1) = (to_px(a.1.min(b.1) - hw, wf.h), (to_px(a.1.max(b.1) + hw, wf.h) + 1).min(wf.h));
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l2 = (dx * dx + dy * dy).max(1e-12);
        for y in py0..py1 {
            let yu = wf.uy(y);
            for x in px0..px1 {
                let xu = wf.ux(x);
                let t = (((xu - a.0) * dx + (yu - a.1) * dy) / l2).clamp(0.0, 1.0);
                let d = ((xu - a.0 - t * dx).powi(2) + (yu - a.1 - t * dy).powi(2)).sqrt();
                let w = (wid[i] + (wid[i + 1] - wid[i]) * t).max(2.0 * pxu);
                let cov = (0.5 - (d - 0.5 * w) / pxu).clamp(0.0, 1.0);
                if cov > 0.0 {
                    hits.push((y * wf.w + x, cov, pressure[i] + (pressure[i + 1] - pressure[i]) * t));
                }
            }
        }
    }
    hits.sort_unstable_by(|p, q| p.0.cmp(&q.0).then(q.1.total_cmp(&p.1)));
    hits.dedup_by_key(|h| h.0);
    for (k, cov, p) in hits {
        let dep = (lead.rate * cov * touch(p)).clamp(0.0, 1.0);
        let cap = lead.cap * (0.55 + 0.45 * p);
        g[k] += (cap - g[k]).max(0.0) * dep;
    }
}

impl Canvas {
    /// Whether anything has been drawn on this canvas.
    pub fn has_drawing(&self) -> bool {
        self.drawing.is_some()
    }

    /// What the first-surface veil (`canvas::haze`, `k` of it) would add
    /// over the bare carbon at window pixel `i`, per channel, to be taken
    /// off again: charcoal open to the eye (drawn, not painted over, no
    /// sheet on it) is a bed of particles, seen as its R∞ (`Cell::bare_carbon`).
    /// None where there is none (all a painting without charcoal has).
    /// With the pastel veil switch (`with_pastel_veil`) the same holds for
    /// every particle bed, pastel and charcoal (`Cell::bare_bed`): one rule.
    pub(crate) fn unveiled(&self, i: usize, k: f32) -> Option<Rgb> {
        let d = self.drawing.as_ref().filter(|d| d.carbon || self.pastel_veil)?;
        let c = &d.cells[i];
        if c.film < 0.0 || self.film[i] > c.film + 1e-4 || self.sheet_over(i) {
            return None;
        }
        let (share, r) = if self.pastel_veil { c.bare_bed()? } else { c.bare_carbon()? };
        Some(r.map(|r| share * k * (1.0 - r)))
    }

    pub(crate) fn drawing_mut(&mut self) -> &mut Drawing {
        let (n, whole) = (self.px.len(), self.f.full_w * self.f.full_h);
        self.drawing.get_or_insert_with(|| Box::new(Drawing::new(n, whole, self.drawing_optics)))
    }

    /// Radius (px) over which the point rides on the tops of the ground.
    fn tooth_px(&self) -> isize {
        (0.35 / self.px_mm()).round().max(1.0) as isize
    }

    /// Draw one line with `lead` along the mark (units), the point having
    /// drawn `worn_mm` since it was sharpened. Returns the mm drawn (the
    /// point wears by that much). Wet paint doesn't take graphite: the line
    /// skips it. The deposit fades to nothing as the pressure goes to zero
    /// (`touch`). The line also goes into the drawing's guide (see
    /// `drawing_guide`), over the whole canvas even in a crop render.
    pub fn draw(&mut self, lead: &Lead, mark: &Mark, worn_mm: f32, seed: u64) -> f32 {
        let pts = &mark.pts;
        if pts.len() < 2 {
            return 0.0;
        }
        let f = self.f;
        let mmu = self.mm_per_unit;
        let pxu = 1.0 / f.scale;
        let s = arclen(pts);
        // width per point (units): the point wears along the line and
        // flattens a little under pressure
        let wid: Vec<f32> = (0..pts.len()).map(|i| lead.width_mm(worn_mm + s[i] * mmu) * (0.8 + 0.4 * mark.pressure[i]) / mmu).collect();
        // footprint: (pixel index, coverage, pressure)
        let mut hits: Vec<(usize, f32, f32)> = Vec::new();
        for i in 0..pts.len() - 1 {
            let (a, b) = (pts[i], pts[i + 1]);
            let hw = 0.5 * wid[i].max(wid[i + 1]).max(pxu) + pxu;
            let (bx0, by0) = (a.0.min(b.0) - hw, a.1.min(b.1) - hw);
            let (bx1, by1) = (a.0.max(b.0) + hw, a.1.max(b.1) + hw);
            let to_px = |u: f32, o: usize, n: usize| ((u * f.scale).floor() as isize - o as isize).clamp(0, n as isize) as usize;
            let (px0, px1) = (to_px(bx0, f.x0, f.w), (to_px(bx1, f.x0, f.w) + 1).min(f.w));
            let (py0, py1) = (to_px(by0, f.y0, f.h), (to_px(by1, f.y0, f.h) + 1).min(f.h));
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = (dx * dx + dy * dy).max(1e-12);
            for y in py0..py1 {
                let yu = f.uy(y);
                for x in px0..px1 {
                    let xu = f.ux(x);
                    let t = (((xu - a.0) * dx + (yu - a.1) * dy) / l2).clamp(0.0, 1.0);
                    let d = ((xu - a.0 - t * dx).powi(2) + (yu - a.1 - t * dy).powi(2)).sqrt();
                    let w = wid[i] + (wid[i + 1] - wid[i]) * t;
                    let cov = (0.5 - (d - (0.5 * w).max(0.5 * pxu)) / pxu).clamp(0.0, 1.0) * (w / pxu).min(1.0);
                    if cov > 0.0 {
                        let p = mark.pressure[i] + (mark.pressure[i + 1] - mark.pressure[i]) * t;
                        hits.push((y * f.w + x, cov, p));
                    }
                }
            }
        }
        guide_line(&mut self.drawing_mut().guide, f.whole(), lead, pts, &wid, &mark.pressure);
        // one deposit per pixel: the nearest pass of the line over it
        hits.sort_unstable_by(|p, q| p.0.cmp(&q.0).then(q.1.total_cmp(&p.1)));
        hits.dedup_by_key(|h| h.0);
        let rt = self.tooth_px();
        let (w, h) = (f.w as isize, f.h as isize);
        let height = &self.height;
        let top = |i: usize| {
            let (x, y) = ((i % f.w) as isize, (i / f.w) as isize);
            let mut m = f32::MIN;
            for j in (y - rt).max(0)..=(y + rt).min(h - 1) {
                for k in (x - rt).max(0)..=(x + rt).min(w - 1) {
                    m = m.max(height[(j * w + k) as usize]);
                }
            }
            m
        };
        // how far each pixel lies below the tops the point rides on (µm)
        let depths: Vec<f32> = hits.iter().map(|&(i, _, _)| top(i) - height[i]).collect();
        let lift = lead.lift();
        // a pixel coarser than the weave's threads holds tops of its own
        // (its averaged height overstates how far the point is from them)
        // and averages many crumbs of the deposit
        let sub = (0.15 / self.px_mm()).min(1.0);
        let (depth_k, crumble) = (sub.powf(0.6), lead.crumble * sub.sqrt());
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let wetv: Vec<bool> = hits.iter().map(|&(i, _, _)| self.wet.vol[i] > 1e-5).collect();
        let mut px = std::mem::take(&mut self.px);
        {
            let d = self.drawing_mut();
            for (k, &(i, cov, p)) in hits.iter().enumerate() {
                if wetv[k] {
                    continue;
                }
                let c = &mut d.cells[i];
                if c.film < 0.0 || film[i] > c.film + 1e-4 {
                    // bare, or painted over since: a new drawing on top
                    c.restart(film[i], px[i]);
                }
                let depth = depths[k] * depth_k;
                let bite = lead.bite_um * p.powf(1.2) + 3.0;
                let contact = (-depth.max(0.0) / bite).exp();
                let (gx, gy) = ((i % f.w + f.x0) as i64, (i / f.w + f.y0) as i64);
                let grain = 1.0 - crumble * hash2(gx, gy, seed);
                let dep = (lead.rate * contact * cov * grain * touch(p)).clamp(0.0, 1.0);
                let cap = lead.cap * (0.55 + 0.45 * p);
                let da = (cap - c.a).max(0.0) * dep;
                if da <= 0.0 {
                    continue;
                }
                let a1 = c.a + da;
                let before = (c.a, c.reflectance());
                if let Some(optics) = c.optics.as_mut() {
                    let g = c.non_pastel - c.non_pastel_floor;
                    optics.graphite = (g * optics.graphite + da * lead.flake) / (g + da);
                } else {
                    c.r = c.r.map(|r| (c.a * r + da * lead.flake) / a1);
                }
                c.lift = (c.a * c.lift + da * lift) / a1;
                c.a = a1;
                c.non_pastel = (c.non_pastel + da).min(a1);
                px[i] = c.repaint(px[i], before);
            }
        }
        self.px = px;
        self.film = film;
        s.last().copied().unwrap_or(0.0) * mmu
    }

    /// A kneaded eraser pressed and rolled over `m` (coverage 0..1): lifts
    /// loose drawing (not fixed, not painted over) by up to `strength` 0..1
    /// of what is there, better from the tops of the tooth than from the
    /// hollows, and never quite all of it. Loose stick pastel leaves the
    /// tooth with what it lifts, so an erased passage takes pastel again. It
    /// doesn't reach under a sheet laid on the picture. It lifts the guide
    /// too (see `drawing_guide`), wherever it goes.
    pub fn erase(&mut self, m: &Mask, strength: f32) {
        self.check_mask(m);
        let f = self.f;
        let Some(d) = self.drawing.as_mut() else { return };
        let sheet = self.sheet.as_ref();
        let st = strength.clamp(0.0, 1.0);
        for (k, g) in d.guide.iter_mut().enumerate() {
            let mv = m.data[k].min(1.0);
            if mv > 0.0 && *g > 0.0 && !sheet.is_some_and(|s| s.covers_whole_pixel(f, k)) {
                if self.sealed_drawing_guide {
                    let (x, y) = (k % f.full_w, k / f.full_w);
                    if x >= f.x0 && x < f.x0 + f.w && y >= f.y0 && y < f.y0 + f.h {
                        let i = (y - f.y0) * f.w + x - f.x0;
                        if self.film[i] > d.cells[i].film.max(0.0) + 1e-4 || self.wet.vol[i] > 1e-5 {
                            continue;
                        }
                    }
                }
                let floor = d.guide_floor.as_ref().map_or(0.0, |fl| fl[k]);
                *g = (*g * (1.0 - (st * mv * GUIDE_LIFT).min(0.97))).max(floor.min(*g));
            }
        }
        let rt = (0.35 / (self.mm_per_unit / f.scale)).round().max(1.0) as isize;
        let (w, h) = (f.w as isize, f.h as isize);
        let height = &self.height;
        for i in 0..self.px.len() {
            let mv = m.data[f.whole_index(i)];
            let c = &mut d.cells[i];
            if mv <= 0.0 || c.a <= 0.0 || self.film[i] > c.film + 1e-4 || self.wet.vol[i] > 1e-5 || self.sheet.as_ref().is_some_and(|s| s.over[i]) {
                continue;
            }
            let (x, y) = ((i % f.w) as isize, (i / f.w) as isize);
            let mut top = f32::MIN;
            for j in (y - rt).max(0)..=(y + rt).min(h - 1) {
                for k in (x - rt).max(0)..=(x + rt).min(w - 1) {
                    top = top.max(height[(j * w + k) as usize]);
                }
            }
            // the eraser touches the tops; the hollows keep more
            let reach = 1.0 - ((top - height[i]) / 10.0).clamp(0.0, 1.0);
            let take = (strength.clamp(0.0, 1.0) * mv.min(1.0) * c.lift * (0.55 + 0.4 * reach)).min(0.97);
            let a1 = (c.a * (1.0 - take)).max(c.floor.min(c.a));
            let before = (c.a, c.reflectance());
            if c.loose > 0.0 && take > 0.0 {
                let removable = c.a - c.floor;
                c.loose *= if removable > 0.0 { (a1 - c.floor).max(0.0) / removable } else { 0.0 };
            }
            if c.optics.is_some() {
                let keep = if c.a > c.floor { (a1 - c.floor) / (c.a - c.floor) } else { 0.0 };
                c.non_pastel = c.non_pastel_floor + (c.non_pastel - c.non_pastel_floor) * keep;
            } else {
                c.non_pastel = (c.non_pastel * (1.0 - take)).max(c.non_pastel_floor.min(c.non_pastel));
            }
            c.a = a1;
            self.px[i] = c.repaint(self.px[i], before);
        }
    }

    /// Fixative over `m` (or all): binds the loose drawing so an eraser no
    /// longer lifts it.
    pub fn fix_drawing(&mut self, m: Option<&Mask>) {
        if let Some(m) = m {
            self.check_mask(m);
        }
        let f = self.f;
        let Some(d) = self.drawing.as_mut() else { return };
        // (a sheet laid on the picture keeps the spray off what it covers)
        let sheet = self.sheet.as_ref();
        for (i, c) in d.cells.iter_mut().enumerate() {
            if m.is_none_or(|m| m.data[f.whole_index(i)] > 0.5) && !sheet.is_some_and(|s| s.over[i]) {
                if c.optics.is_some() {
                    let before = (c.a, c.reflectance());
                    c.bind(0.0);
                    if self.film[i] <= c.film + 1e-4 && self.wet.vol[i] <= 1e-5 {
                        self.px[i] = c.repaint(self.px[i], before);
                    }
                }
                c.floor = c.a;
                c.non_pastel_floor = c.non_pastel;
            }
        }
        let fl = d.guide_floor.get_or_insert_with(|| vec![0.0; d.guide.len()]);
        for (k, (v, g)) in fl.iter_mut().zip(&d.guide).enumerate() {
            if m.is_none_or(|m| m.data[k] > 0.5) && !sheet.is_some_and(|s| s.covers_whole_pixel(f, k)) {
                *v = *g;
            }
        }
    }

    /// The graphite deposit as a mask: 1 on a firm line, fading with the
    /// deposit (0.5 of a pixel covered or more counts as 1). Includes drawing
    /// that paint has since covered, until something is drawn over it.
    ///
    /// It is the physical deposit, broken by the tooth and the grain of the
    /// lead, and it is known only where the canvas holds pixels: in a crop
    /// render it is zero outside the window. To paint into the drawing (a
    /// mask to plan strokes with) use `drawing_guide`.
    pub fn drawing_mask(&self) -> Mask {
        let whole = self.f.whole();
        let mut m = Mask::empty(whole);
        if let Some(d) = &self.drawing {
            let f = self.f;
            for (i, c) in d.cells.iter().enumerate() {
                m.data[f.whole_index(i)] = (c.a / 0.5).min(1.0);
            }
        }
        m
    }

    /// The drawing as geometry: where the lines were drawn, over the whole
    /// canvas (a crop render has all of it too), at least two pixels wide.
    /// It reads like `drawing_mask` would on a perfectly smooth ground: 1 on
    /// a firm line (0.5 of a pixel covered or more), about 0.5 on a light
    /// 2H line, fading to nothing as the pressure goes to zero. The eraser
    /// lifts it (never below what fixative bound); paint over the drawing
    /// doesn't change it. Unlike `drawing_mask` it follows each line
    /// continuously instead of breaking up in the tooth and the grain.
    /// This is the mask to paint into the drawing with, e.g. its firm lines:
    /// `work(&c.drawing_guide().band(0.7, 1.0, 0.1), ...)`.
    pub fn drawing_guide(&self) -> Mask {
        let whole = self.f.whole();
        match &self.drawing {
            Some(d) => Mask { f: whole, data: d.guide.iter().map(|a| (a / 0.5).min(1.0)).collect() },
            None => Mask::empty(whole),
        }
    }

    /// The drawing alone, as it would look on a white ground (the pixels of
    /// the window): what an infrared reflectogram shows of an underdrawing.
    pub fn drawing_view(&self) -> Vec<Rgb> {
        const PAPER: f32 = 0.82;
        match &self.drawing {
            None => vec![[PAPER; 3]; self.px.len()],
            Some(d) => d.cells.iter().map(|c| cover([PAPER; 3], c.a, c.reflectance())).collect(),
        }
    }
}

/// The ground under a deposit of coverage `a` and flake reflectance `r`.
pub(crate) fn uncover(p: Rgb, a: f32, r: Rgb) -> Rgb {
    if a <= 0.0 {
        return p;
    }
    let k = 1.0 / (1.0 - a).max(1e-3);
    [((p[0] - a * r[0]) * k).max(0.0), ((p[1] - a * r[1]) * k).max(0.0), ((p[2] - a * r[2]) * k).max(0.0)]
}

/// A deposit of coverage `a` and flake reflectance `r` over `under`.
pub(crate) fn cover(under: Rgb, a: f32, r: Rgb) -> Rgb {
    [under[0] * (1.0 - a) + a * r[0], under[1] * (1.0 - a) + a * r[1], under[2] * (1.0 - a) + a * r[2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat() -> Canvas {
        Canvas::new_window(400, 1.5, [0.8; 3], None).with_size_mm(440.0)
    }

    /// Mean color over a band of the window (units).
    fn band_mean(c: &Canvas, y0: f32, y1: f32, x0: f32, x1: f32) -> Rgb {
        let f = c.window();
        let (mut s, mut n) = ([0.0f32; 3], 0.0);
        for (i, q) in c.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x >= x0 && x < x1 && y >= y0 && y < y1 {
                for k in 0..3 {
                    s[k] += q[k];
                }
                n += 1.0;
            }
        }
        s.map(|v| v / n)
    }

    /// Darkening laid by one 2B line at pressure `p` on a flat ground.
    fn laid(p: &[f32]) -> (Canvas, f32) {
        let mut c = flat();
        let m = hand_line(&[(100.0, 100.0), (900.0, 100.0)], p, false, true, 0.0, 5);
        c.draw(&Lead::pencil("2B").unwrap(), &m, 0.0, 9);
        let dark: f32 = c.pixels().iter().map(|q| 0.8 - q[0]).sum();
        (c, dark)
    }

    #[test]
    fn sealed_guides_survive_erasing_in_whole_and_windowed_canvases() {
        for crop in [None, Some(crate::Crop { units: [350.0, 50.0, 850.0, 150.0], margin: 0.0 })] {
            for enabled in [false, true] {
                let mut c = Canvas::new_window(400, 1.5, [0.8; 3], crop).with_size_mm(440.0).with_sealed_drawing_guide(enabled);
                let mark = hand_line(&[(100.0, 100.0), (900.0, 100.0)], &[0.6], false, true, 0.0, 5);
                c.draw(&Lead::pencil("2B").unwrap(), &mark, 0.0, 9);
                let f = c.window();
                for i in 0..c.px.len() {
                    let x = f.ux(i % f.w);
                    if x < 500.0 {
                        c.film[i] += 1.0;
                    } else if x > 650.0 {
                        c.wet.vol[i] = 1.0;
                    }
                }
                let before = c.drawing.as_ref().unwrap().guide.clone();
                c.erase(&Mask::full(c.frame()), 1.0);
                let after = &c.drawing.as_ref().unwrap().guide;
                for (i, &g) in before.iter().enumerate() {
                    let (x, y) = (c.frame().ux(i % f.full_w), c.frame().uy(i / f.full_w));
                    let sealed = enabled && f.holds(x, y) && (x < 500.0 || x > 650.0);
                    assert_eq!(after[i], if sealed { g } else { g * (1.0 - GUIDE_LIFT) }, "guide pixel {i}, enabled={enabled}, window={f:?}");
                }
            }
        }
    }

    /// Graded hatching presses each stroke by the weight under it: over a
    /// ramp it lays more where the weight is high, nothing where it is zero,
    /// and the passage fades with no line where an ungraded hatch stops.
    #[test]
    fn graded_hatching_follows_the_weight() {
        let f = flat().frame();
        let ramp = Mask::from_fn(f, |x, _| ((x - 200.0) / 600.0).clamp(0.0, 1.0));
        let mut c = flat();
        let lead = Lead::pencil("4B").unwrap();
        for m in hatch_marks_graded(&ramp, 1.2, 4.0, 30.0, 0.8, true, 3) {
            c.draw(&lead, &m, 0.0, 7);
        }
        let dark = |x0: f32, x1: f32| 0.8 - band_mean(&c, 60.0, 600.0, x0, x1)[0];
        let (d0, d1, d2, d3) = (dark(100.0, 190.0), dark(300.0, 400.0), dark(500.0, 600.0), dark(700.0, 800.0));
        assert!(d0 < 1e-4, "no weight, nothing laid: {d0}");
        assert!(d1 < d2 && d2 < d3, "graded: {d1} {d2} {d3}");
        // the old call is unchanged: hatch_marks is the ungraded one
        let (a, b) = (hatch_marks(&ramp, 1.2, 4.0, 30.0, 0.8, 3), hatch_marks_graded(&ramp, 1.2, 4.0, 30.0, 0.8, false, 3));
        assert!(a.len() == b.len() && a.iter().zip(&b).all(|(a, b)| a.pts == b.pts && a.pressure == b.pressure));
    }

    #[test]
    fn pastel_legacy_cells_are_still_readable() {
        for (loose, bound) in [(0.0, 0.0), (8.0, 0.0), (8.0, 4.0)] {
            let mut values = [0.8, 0.1, 0.2, 0.3, 0.55, 0.4, 0.0, loose, bound, 0.0].into_iter();
            let d = Drawing::from_f32s::<()>(1, 0, 2, |n| Ok(values.by_ref().take(n).collect())).unwrap().unwrap();
            assert_eq!(d.layout(), 3);
            assert_eq!(d.cells[0].loose, loose);
            assert_eq!(d.cells[0].bound, bound);
            assert_eq!(d.cells[0].floor, 0.4);
            assert!(values.next().is_none());
        }
    }

    /// A drawing in graphite alone keeps its old serialized layout (five
    /// numbers a cell, flag 1), so older states and digests are unchanged.
    #[test]
    fn graphite_drawing_keeps_its_layout() {
        let (c, _) = laid(&[0.6]);
        let d = c.drawing.as_ref().unwrap();
        assert!(!d.sticks && d.layout() == 1);
        let n = c.pixels().len();
        assert_eq!(d.to_f32s().count(), n * 5 + c.frame().w * c.frame().h + 1);
    }

    #[test]
    fn old_drawing_layouts_keep_their_original_reflectance() {
        for (layout, data) in [
            (1, vec![0.8, 0.1, 0.55, 0.4, 0.0, 0.0]),
            (2, vec![0.8, 0.1, 0.2, 0.3, 0.55, 0.4, 0.0, 8.0, 4.0, 0.0]),
            (3, vec![0.8, 0.1, 0.2, 0.3, 0.55, 0.4, 0.0, 8.0, 4.0, 0.2, 0.1, 0.0]),
        ] {
            let mut values = data.into_iter();
            let d = Drawing::from_f32s::<()>(1, 0, layout, |n| Ok(values.by_ref().take(n).collect())).unwrap().unwrap();
            assert!(values.next().is_none());
            assert!(d.cells[0].optics.is_none());
            assert_eq!(d.cells[0].reflectance(), if layout == 1 { [0.1; 3] } else { [0.1, 0.2, 0.3] });
        }
    }

    #[test]
    fn saturated_fixed_pastel_can_take_removable_color() {
        let mut c = Cell { optics: Some(Optics::default()), ..Cell::default() };
        c.add_pastel(1.0, [0.1; 3], 20.0);
        c.loose = 20.0;
        c.bind(0.0);
        let original = c.reflectance();
        c.add_pastel(0.9, [0.7, 0.06, 0.02], 8.0);
        c.loose += 8.0;
        assert_eq!(c.a, c.floor);
        assert!((c.reflectance()[0] - original[0]).abs() > 0.1);
        c.loose = 0.0;
        assert_eq!(c.reflectance(), original);
        c.add_pastel(0.9, [0.7, 0.06, 0.02], 8.0);
        c.loose = 8.0;
        c.bind(0.15);
        c.free_bound(c.non_pastel_floor, c.bound);
        c.loose += c.bound;
        c.bound = 0.0;
        assert!(c.r[0] > c.r[1]);
    }

    /// With the pastel K–M switch two pastels mixed intimately in the tooth
    /// mix as powders do: K and S add by volume. A yellow pastel (0.75,
    /// 0.6, 0.1) whose particles scatter as chalk filler in air (S = 0.75 ×
    /// 0.73 µm⁻¹: dry_pigment_optics.md table 2.2) with a share of vine
    /// charcoal (`dry::VINE_CHARCOAL`) comes out as charcoal.md §6 tabulates
    /// by Kubelka–Munk, not as the average of their reflectances: at 5 %
    /// charcoal G 0.549, where the average is 0.571 (estimates-audit-2,
    /// fault 1). Dark grains darken a powder well past their share (Clark
    /// 1983): the KM loss from the yellow is more than 1.5× the average's in
    /// red and green (1.7–3.1× over 2–10 % in the table), and about none in
    /// blue, where the yellow already absorbs.
    #[test]
    fn pastel_mixed_in_the_tooth_darkens_as_powders_do() {
        use crate::pastel::Grains;
        let yellow = [0.75f32, 0.6, 0.1];
        let s_y = 0.75 * 0.73;
        let vine = crate::dry::VINE_CHARCOAL;
        let (r_c, s_c) = (vine.color(0.0), crate::charcoal::bed_scatter(&vine));
        // (charcoal.md §6: the Kubelka–Munk column, and the engine's average
        // at 5 %; the table takes carbon's S per channel, the cell its gray
        // mean, 2 % off in red and blue)
        for (c, want) in [(0.02f32, [0.704f32, 0.578, 0.100]), (0.05, [0.649, 0.549, 0.099]), (0.10, [0.578, 0.504, 0.098])] {
            let mut cell = Cell { optics: Some(Optics::default()), film: 0.0, ..Cell::default() };
            cell.add_grains(0.9, yellow, 1.0 - c, Grains { s: s_y, ..Grains::PASTEL });
            cell.loose += 1.0 - c;
            cell.add_grains(0.5, r_c, c, Grains { s: s_c, carbon: 1.0, ..Grains::PASTEL });
            for q in 0..3 {
                assert!((cell.r[q] - want[q]).abs() < 2.5e-3, "{c}: {:?} against {want:?}", cell.r);
            }
            assert!((cell.s - ((1.0 - c) * s_y + c * s_c)).abs() < 1e-6, "S by volume");
            let mut avg = Cell { optics: Some(Optics::default()), film: 0.0, ..Cell::default() };
            avg.add_grains(0.9, yellow, 1.0 - c, Grains::PASTEL);
            avg.loose += 1.0 - c;
            avg.add_grains(0.5, r_c, c, Grains { carbon: 1.0, ..Grains::PASTEL });
            if c == 0.05 {
                assert!((avg.r[1] - 0.571).abs() < 1e-3, "{:?}", avg.r);
            }
            for q in 0..2 {
                let (km, lin) = (yellow[q] - cell.r[q], yellow[q] - avg.r[q]);
                assert!(km > 1.5 * lin, "{c} {q}: {km} against {lin}");
            }
            assert!((cell.r[2] - yellow[2]).abs() < 0.003, "blue barely moves: {:?}", cell.r);
        }
    }

    /// Kubelka's layer over a ground, as `Cell::add_grains` lays a stroke
    /// over loose pastel: it agrees with Kubelka's closed form, R = [(R₂ −
    /// R∞)/R∞ − R∞(R₂ − 1/R∞) e^{SX(1/R∞ − R∞)}] / [(R₂ − R∞) − (R₂ −
    /// 1/R∞) e^{SX(1/R∞ − R∞)}]; a layer that doesn't absorb returns [R₂ +
    /// SX(1 − R₂)]/[1 + SX(1 − R₂)]; a thick layer its own R∞ and a thin one
    /// the ground.
    #[test]
    fn a_layer_over_loose_pastel_is_kubelkas() {
        let closed = |ri: f64, sx: f64, r2: f64| {
            let e = (sx * (1.0 / ri - ri)).exp();
            ((r2 - ri) / ri - ri * (r2 - 1.0 / ri) * e) / ((r2 - ri) - (r2 - 1.0 / ri) * e)
        };
        for (top, sx, under) in [(0.6f32, 0.5f32, 0.021f32), (0.6, 2.0, 0.021), (0.1, 1.0, 0.8), (0.3, 0.2, 0.5), (0.85, 3.0, 0.05)] {
            let got = crate::dry::km_layer([top; 3], sx, [under; 3])[0];
            let want = closed(top as f64, sx as f64, under as f64) as f32;
            assert!((got - want).abs() < 2e-5, "{top} {sx} {under}: {got} against {want}");
        }
        for (sx, r2) in [(1.0f32, 0.0f32), (0.5, 0.3), (4.0, 0.1)] {
            let got = crate::dry::km_layer([0.9999; 3], sx, [r2; 3])[0];
            let want = (r2 + sx * (1.0 - r2)) / (1.0 + sx * (1.0 - r2));
            assert!((got - want).abs() < 2e-3, "white {sx} {r2}: {got} against {want}");
        }
        assert!((crate::dry::km_layer([0.6; 3], 60.0, [0.02; 3])[0] - 0.6).abs() < 1e-5);
        assert!((crate::dry::km_layer([0.6; 3], 1e-6, [0.02; 3])[0] - 0.02).abs() < 1e-5);
    }

    /// A stroke laid over loose charcoal lies over it (`Grains::layer`): it
    /// keeps more of its own color than the same pastel mixed into the
    /// charcoal (the finger's, crumbs'), and the more of it is laid the
    /// nearer its own; where it falls beside the charcoal it is its own
    /// color, an area mix.
    #[test]
    fn a_stroke_over_loose_charcoal_lies_over_it() {
        use crate::pastel::Grains;
        let yellow = [0.75f32, 0.6, 0.1];
        let s_y = 0.75 * 0.73 * (1.0 - crate::wet_pastel::POROSITY);
        let vine = crate::dry::VINE_CHARCOAL;
        let s_c = crate::charcoal::bed_scatter(&vine) * crate::charcoal::VINE_SOLIDS * crate::charcoal::VINE_COVER_UM / crate::pastel::PARTICLE_UM;
        let laid = |v: f32, layer: bool| {
            let mut cell = Cell { optics: Some(Optics::default()), film: 0.0, ..Cell::default() };
            cell.add_grains(0.5, vine.color(0.0), 6.0, Grains { s: s_c, carbon: 1.0, ..Grains::PASTEL });
            cell.loose += 6.0;
            cell.add_grains(0.5, yellow, v, Grains { s: s_y, layer, ..Grains::PASTEL });
            cell.r[1]
        };
        for v in [2.0f32, 6.0, 18.0] {
            assert!(laid(v, true) > laid(v, false) + 0.05, "{v}: layered {} mixed {}", laid(v, true), laid(v, false));
        }
        assert!(laid(2.0, true) < laid(6.0, true) && laid(6.0, true) < laid(18.0, true) && laid(18.0, true) < yellow[1]);
        // (none loose to lie on: its own color)
        let mut bare = Cell { optics: Some(Optics::default()), film: 0.0, ..Cell::default() };
        bare.add_grains(0.5, yellow, 3.0, Grains { s: s_y, layer: true, ..Grains::PASTEL });
        assert_eq!(bare.r, yellow);
    }

    /// Fixative by the particles' index: S in resin over S in air is the
    /// repository's Mie at each index (estimates-audit-2, fault 3: 0.49 for n
    /// 2.0, 0.61 for 2.4, 0.72 for 3.0, 0.30 for 1.74, 0.14 for 1.65, 0.056
    /// for whiting, 0.027 for 1.56, about 0 for ultramarine), not one fifth.
    /// A pastel wetted over `ws` of its pores then has K/S ÷ (1 − ws + ws·ρ):
    /// lead white, vermilion and the earths darken less than the old rule
    /// had them, ultramarine and whiting more. Pure charcoal fixed this way
    /// is its beds' own R∞ with that share in resin (`CarbonBed::color`),
    /// pastel's rule and carbon's one rule.
    #[test]
    fn fixative_darkens_each_particle_by_its_index() {
        for (n, want) in [(2.0f32, 0.49f32), (2.4, 0.61), (3.0, 0.72), (1.74, 0.30), (1.65, 0.14), (1.59, 0.056), (1.56, 0.027), (1.50, 0.0)] {
            assert!((crate::dry::scatter_in_resin(n) - want).abs() < 0.006, "{n}: {}", crate::dry::scatter_in_resin(n));
        }
        // 1.5 µm of resin into 18.2 µm of loose pastel (pores 0.55): 0.15 wetted, the old rule's share
        let (t, loose) = (1.5f32, 1.5 / (0.55 * 0.15));
        assert!((crate::charcoal::wet_share(t, loose) - 0.15).abs() < 1e-5);
        let fixed = |n: f32, r: f32| {
            let mut c = Cell { optics: Some(Optics::default()), film: 0.0, a: 0.9, r: [r; 3], loose, index: crate::wet_pastel::to_index(n), ..Cell::default() };
            c.bind_by_index(t);
            c.optics.unwrap().bound[0]
        };
        let old = |r: f32| {
            let mut c = Cell { optics: Some(Optics::default()), film: 0.0, a: 0.9, r: [r; 3], loose, ..Cell::default() };
            c.bind(0.15);
            c.optics.unwrap().bound[0]
        };
        let ks = |r: f32| (1.0 - r) * (1.0 - r) / (2.0 * r);
        let rinf = |q: f32| 1.0 + q - (q * q + 2.0 * q).sqrt();
        for (n, rho, r) in [(2.0f32, 0.49f32, 0.85f32), (3.0, 0.72, 0.35), (2.1, 0.53, 0.4), (1.5, 0.0, 0.3), (1.59, 0.056, 0.85)] {
            let want = rinf(ks(r) / (1.0 - 0.15 + 0.15 * rho));
            assert!((fixed(n, r) - want).abs() < 2e-3, "{n}: {} against {want}", fixed(n, r));
            if n >= 1.9 {
                assert!(fixed(n, r) > old(r), "{n}: over-darkened before ({} against {})", old(r), fixed(n, r));
            } else {
                assert!(fixed(n, r) < old(r), "{n}: under-darkened before ({} against {})", old(r), fixed(n, r));
            }
        }
        // a thin line is wetted through, a thick layer hardly: 3 µm against 30 µm
        let (thin, thick) = (crate::charcoal::wet_share(t, 3.0), crate::charcoal::wet_share(t, 30.0));
        assert!((thin - 0.909).abs() < 1e-3 && (thick - 0.0909).abs() < 1e-3);
        // pure vine charcoal: its bed with that share of its pores in resin
        let vine = crate::dry::VINE_CHARCOAL;
        for loose in [3.0f32, 10.0, 30.0] {
            let mut c = Cell { optics: Some(Optics::default()), film: 0.0, a: 0.9, r: vine.color(0.0), loose, carbon: 1.0, ..Cell::default() };
            c.bind_by_index(t);
            let want = vine.color(crate::charcoal::wet_share(t, loose));
            let got = c.optics.unwrap().bound;
            for q in 0..3 {
                assert!((got[q] - want[q]).abs() < 3e-4, "{loose}: {got:?} against {want:?}");
            }
        }
    }
}
