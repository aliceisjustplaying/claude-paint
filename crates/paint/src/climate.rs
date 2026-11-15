//! The room a picture is worked in: one air temperature and one relative
//! humidity that every humidity-dependent effect reads, so they can't
//! disagree. Today that is the evaporation of water from wet pastel
//! (`crate::wet_pastel`); a gum's glass transition and the capillary hold
//! of particles on a sheet hang on the same humidity, and should read it
//! here when they are modelled.
//!
//! Source: notes/research/wet_pastel.md §4 (the studio) and
//! notes/estimates-audit-2.md W6.

/// Room air: temperature (°C) and relative humidity (0..1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Climate {
    pub air_c: f64,
    pub rh: f64,
}

/// The studio every painting is worked in: 20 °C (the air sheet.rs already
/// assumes) and 50 % relative humidity. Heating Paris's outdoor air
/// (Montsouris: annual mean 12.8 °C, 1991–2020, at 76 % RH, 1961–90) to
/// 20 °C without adding moisture gives 48 %; January's (5.4 °C, 83 %) at
/// 18 °C gives 36 %; an unheated July room stays near 68 % [D]. 50 % sits
/// in that range, at the middle
/// of the 40–60 % held for works on paper. No measured record of a
/// historic studio's humidity was found, so the value is a choice inside
/// those bounds, not a measurement. One setting, so a later room setting
/// (a log-head line) changes every effect at once.
pub const STUDIO: Climate = Climate { air_c: 20.0, rh: 0.5 };
