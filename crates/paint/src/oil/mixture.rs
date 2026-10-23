//! Explicit mean-field closure for genuinely stirred MOBILE paint only.
//!
//! Sourced storage convention: IUPAC Henry solubility is linear in gas partial
//! pressure at equilibrium and infinite dilution, section 2.1,
//! https://doi.org/10.1515/pac-2020-0302 and
//! https://goldbook.iupac.org/terms/view/09024. This does not supply a mixed-oil
//! solubility law. Pizzimenti et al., sections 2.2-2.4 and Tables 1-2,
//! https://doi.org/10.1021/acsapm.0c01441, supply separate pigment/oil specimens,
//! not measured mixture coefficients. Lead-ion catalysis in their introduction
//! also limits the independent-material approximation below.
//!
//! Derived conservation projection, using the existing chemistry.rs units:
//! C_j=B_j s_b,j+S_j s_s,j, M=sum C_j a_j, C=sum C_j.
//! P, B and S add. x=sum B_j x_j/B, n=sum B_j n_j/B,
//! I=sum B_j I_j/B. Named binder fractions are also binder-volume weighted.
//! s_b=sum B_j s_b,j/B and s_s=sum S_j s_s,j/S give C_new=C.
//! Set a_new=M/C_new, NOT a volume/binder average of the old activities.
//! Thus both dissolved oxygen amount and a common equilibrium activity survive
//! composition changes. Zero solvent uses the binder-weighted nominal s_s.
//! I remains the engine's recipe-normalized inhibitor inventory, not a measured
//! universal molecular concentration. Only floating-point roundoff is clipped.
//!
//! Explicit constitutive MODELING ASSUMPTIONS, not sourced material constants:
//! homogenization replaces constituent cure histories with the conserved means.
//! For w_j=B_j/B, set k_x=sum w_j k_x,j and k_I=sum w_j k_I,j.
//! A, inhibition strength H and oxidation demand q_x use weights w_j k_x,j/k_x;
//! inhibitor demand q_I uses w_j k_I,j/k_I. These match the corresponding
//! zero-inhibitor/low-conversion rate moments and the first inhibition slope,
//! not every cross-moment or an unmixed population's subsequent evolution.
//! Network rate is binder weighted. Evaporation rate is solvent weighted
//! (binder weighted when S=0). Inactive-rate coefficients use binder weights.
//!
//! Assume subcell oxygen equilibrium and parallel local activity gradients:
//! D0=sum C_j D0_j/C and air_transfer=sum C_j h_air,j/C.
//! log(r)=sum C_j D0_j log(r_j)/sum C_j D0_j fits the parallel permeability's
//! log-slope at n=0; it is not exact at arbitrary n. If D0=0, r is capacity
//! weighted and unused. The unchanged column solver then uses D0*r^n.
//! This declares a homogeneous morphology, not the harmonic resistance of
//! alternating slabs. Solubilities and primitive constitutive laws are retained
//! as independent contributions: nonideal solvent effects, catalyst exchange,
//! new cross-reactions and percolation are uncalibrated. No constants are copied
//! from an unrelated fixture. Like recipes::kinetics_for, composites carry
//! EXTRAPOLATED_RECIPE provenance, never an original specimen's measured label.
//!
//! Optical latent coordinates, tube-part identities and effective native Prop
//! values use native geometric-volume weights. The oil ratio is reanchored to
//! the actual binder/pigment amounts so Fragment::properties does not apply
//! dilution twice. Named binders retain all five fractions. Current drier codes
//! 0 (absent) and one nonzero family combine without losing that family; two
//! distinct nonzero families are rejected because Recipe has no identity list.
//! The mixed Prop retains the existing relative-drier scalar too.
//!
//! Integration: scale normalized source payloads by the physical volumes
//! actually entrained, then call mix_mobile and replace that stirred parcel by
//! its ONE result. Do not mix an unworked overcoat, intact coherent fragments
//! or unentrained material. The caller decides mobility/contact; no arbitrary
//! conversion threshold or history cap is imposed here. Geometry and the new
//! mechanical history are explicit caller inputs, not silently reset stress.
//! A mixture needs positive binder; solvent-only transport stays a separate
//! operation. This is a conservative coarse law, not an exact population solver
//! or measured calendar/transport calibration. Cell remains 32 bytes and no
//! engine dispatch, legacy serialization or other source file changes here.

use super::chemistry::Kinetics;
use super::exchange::{Fragment, MechanicalHistory, Recipe};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MixtureError {
    Empty,
    InvalidFragment(usize),
    InvalidGeometry,
    NoBinder,
    DifferentDrierFamilies,
    DifferentRevisions,
    NumericalFailure,
}

pub fn mix_mobile(
    fragments: &[Fragment],
    reference_area: f64,
    history: MechanicalHistory,
) -> Result<Fragment, MixtureError> {
    let first = fragments.first().ok_or(MixtureError::Empty)?;
    if !reference_area.is_finite()
        || reference_area <= 0.0
        || history
            .tension_n_m
            .iter()
            .chain(&history.moment_n)
            .any(|v| !v.is_finite())
        || !history.coverage.is_finite()
        || history.coverage <= 0.0
        || history.coverage > 1.0
        || !history.interface_damage.is_finite()
        || !(0.0..=1.0).contains(&history.interface_damage)
    {
        return Err(MixtureError::InvalidGeometry);
    }
    let (mut volume, mut pigment, mut binder, mut solvent, mut capacity, mut oxygen) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let mut drier = 0;
    for (index, f) in fragments.iter().enumerate() {
        if [f.volume, f.pigment, f.binder, f.solvent]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || f.volume == 0.0
            || [f.oxygen, f.oxidation, f.network, f.inhibitor]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
            || f.recipe.kinetics.validate().is_err()
            || f.recipe
                .color
                .iter()
                .chain(&f.recipe.properties)
                .any(|v| !v.is_finite())
            || f.recipe
                .parts
                .iter()
                .any(|(_, v)| !v.is_finite() || *v < 0.0)
            || f.recipe.binders.iter().any(|v| !v.is_finite() || *v < 0.0)
            || (f.recipe.binders.iter().sum::<f64>() - 1.0).abs() > 1e-12
            || !f.oxygen_capacity().is_finite()
            || (f.oxygen_capacity() == 0.0 && f.oxygen != 0.0)
        {
            return Err(MixtureError::InvalidFragment(index));
        }
        if f.recipe.kinetics.revision != first.recipe.kinetics.revision {
            return Err(MixtureError::DifferentRevisions);
        }
        let code = f.recipe.kinetics.drier_code;
        if code != 0 {
            if drier != 0 && drier != code {
                return Err(MixtureError::DifferentDrierFamilies);
            }
            drier = code;
        }
        volume += f.volume;
        pigment += f.pigment;
        binder += f.binder;
        solvent += f.solvent;
        capacity += f.oxygen_capacity();
        oxygen += f.oxygen_capacity() * f.oxygen;
    }
    if [volume, pigment, binder, solvent, capacity, oxygen]
        .iter()
        .any(|v| !v.is_finite())
    {
        return Err(MixtureError::NumericalFailure);
    }
    if binder == 0.0 {
        return Err(MixtureError::NoBinder);
    }
    if capacity == 0.0 {
        return Err(MixtureError::NumericalFailure);
    }

    let binder_mean = |field: fn(&Kinetics) -> f64| -> f64 {
        fragments
            .iter()
            .map(|f| f.binder / binder * field(&f.recipe.kinetics))
            .sum()
    };
    let capacity_mean = |field: fn(&Kinetics) -> f64| -> f64 {
        fragments
            .iter()
            .map(|f| f.oxygen_capacity() / capacity * field(&f.recipe.kinetics))
            .sum()
    };
    let mut k = first.recipe.kinetics;
    k.oxygen_solubility = binder_mean(|k| k.oxygen_solubility);
    k.solvent_solubility = if solvent > 0.0 {
        fragments
            .iter()
            .map(|f| f.solvent / solvent * f.recipe.kinetics.solvent_solubility)
            .sum()
    } else {
        binder_mean(|k| k.solvent_solubility)
    };
    k.oxidation_rate_s = binder_mean(|k| k.oxidation_rate_s);
    k.inhibitor_rate_s = binder_mean(|k| k.inhibitor_rate_s);
    let oxidation_rate = k.oxidation_rate_s;
    let reaction_mean = |field: fn(&Kinetics) -> f64| -> f64 {
        if oxidation_rate == 0.0 {
            return binder_mean(field);
        }
        fragments
            .iter()
            .map(|f| {
                (f.binder / binder * f.recipe.kinetics.oxidation_rate_s) / oxidation_rate
                    * field(&f.recipe.kinetics)
            })
            .sum()
    };
    k.autocatalysis = reaction_mean(|k| k.autocatalysis);
    k.inhibition_strength = reaction_mean(|k| k.inhibition_strength);
    k.oxygen_demand = reaction_mean(|k| k.oxygen_demand);
    k.inhibitor_oxygen_demand = if k.inhibitor_rate_s > 0.0 {
        fragments
            .iter()
            .map(|f| {
                (f.binder / binder * f.recipe.kinetics.inhibitor_rate_s) / k.inhibitor_rate_s
                    * f.recipe.kinetics.inhibitor_oxygen_demand
            })
            .sum()
    } else {
        binder_mean(|k| k.inhibitor_oxygen_demand)
    };
    k.network_rate_s = binder_mean(|k| k.network_rate_s);
    k.solvent_evaporation_rate_s = if solvent > 0.0 {
        fragments
            .iter()
            .map(|f| f.solvent / solvent * f.recipe.kinetics.solvent_evaporation_rate_s)
            .sum()
    } else {
        binder_mean(|k| k.solvent_evaporation_rate_s)
    };
    k.diffusivity_um2_s = capacity_mean(|k| k.diffusivity_um2_s);
    k.cured_diffusivity_ratio = if k.diffusivity_um2_s > 0.0 {
        fragments
            .iter()
            .map(|f| {
                (f.oxygen_capacity() / capacity * f.recipe.kinetics.diffusivity_um2_s)
                    / k.diffusivity_um2_s
                    * f.recipe.kinetics.cured_diffusivity_ratio.ln()
            })
            .sum::<f64>()
            .exp()
    } else {
        capacity_mean(|k| k.cured_diffusivity_ratio)
    };
    k.air_transfer_um_s = capacity_mean(|k| k.air_transfer_um_s);
    k.provenance_code = super::recipes::EXTRAPOLATED_RECIPE;
    k.drier_code = drier;

    let mut binders = [0.0; 5];
    let mut color = [0.0f64; crate::wet::LAT];
    let mut properties = [0.0f64; 10];
    let mut parts = std::collections::BTreeMap::<usize, f64>::new();
    let (mut oxidation, mut network, mut inhibitor) = (0.0, 0.0, 0.0);
    for f in fragments {
        let b = f.binder / binder;
        let v = f.volume / volume;
        for (out, share) in binders.iter_mut().zip(f.recipe.binders) {
            *out += b * share;
        }
        for (out, c) in color.iter_mut().zip(f.recipe.color) {
            *out += v * c as f64;
        }
        for (out, p) in properties.iter_mut().zip(f.properties()) {
            *out += v * p as f64;
        }
        for &(id, share) in &f.recipe.parts {
            *parts.entry(id).or_default() += v * share as f64;
        }
        oxidation += b * f.oxidation;
        network += b * f.network;
        inhibitor += b * f.inhibitor;
    }
    let named_total = binders.iter().sum::<f64>();
    for b in &mut binders {
        *b /= named_total;
    }
    let active: Vec<_> = binders
        .iter()
        .enumerate()
        .filter(|(_, v)| **v > 0.0)
        .collect();
    k.binder_code = if active.len() == 1 {
        active[0].0 as u32 + 1
    } else {
        6
    };
    let q = properties[8];
    if pigment > 0.0 && q > 0.0 && q < 1.0 {
        properties[4] = binder / pigment * (1.0 - q) / q;
    }
    let recipe = Recipe {
        color: color.map(|v| v as f32),
        parts: parts
            .into_iter()
            .filter(|(_, v)| *v > 0.0)
            .map(|(id, v)| (id, v as f32))
            .collect(),
        properties: properties.map(|v| v as f32),
        kinetics: k,
        binders,
    };
    let mixed_capacity = binder * k.oxygen_solubility + solvent * k.solvent_solubility;
    if k.validate().is_err()
        || !mixed_capacity.is_finite()
        || mixed_capacity <= 0.0
        || recipe
            .color
            .iter()
            .chain(&recipe.properties)
            .any(|v| !v.is_finite())
        || recipe.parts.iter().any(|(_, v)| !v.is_finite())
    {
        return Err(MixtureError::NumericalFailure);
    }
    Ok(Fragment {
        reference_area,
        volume,
        pigment,
        binder,
        solvent,
        oxygen: (oxygen / mixed_capacity).min(1.0),
        oxidation: oxidation.min(1.0),
        network: network.min(1.0),
        inhibitor: inhibitor.min(1.0),
        recipe,
        history,
    })
}

#[cfg(test)]
mod tests {
    use super::super::exchange::{Inventory, MaterialPayload};
    use super::*;

    fn fragment() -> Fragment {
        Fragment {
            reference_area: 1.0,
            volume: 1.0,
            pigment: 10.0,
            binder: 10.0,
            solvent: 5.0,
            oxygen: 0.5,
            oxidation: 0.2,
            network: 0.1,
            inhibitor: 0.3,
            recipe: Recipe {
                color: [0.2; crate::wet::LAT],
                parts: vec![(0, 1.0)],
                properties: [2.0, 0.8, 2.0, 0.0, 1.0, 0.7, 0.6, 0.0, 0.5, 0.0],
                kinetics: Kinetics {
                    oxygen_solubility: 0.2,
                    solvent_solubility: 0.4,
                    oxidation_rate_s: 0.001,
                    inhibitor_rate_s: 0.002,
                    binder_code: 1,
                    drier_code: 1,
                    provenance_code: 1,
                    ..Kinetics::default()
                },
                binders: [1.0, 0.0, 0.0, 0.0, 0.0],
            },
            history: MechanicalHistory::default(),
        }
    }

    fn inventory(fragments: Vec<Fragment>) -> [f64; 7] {
        let Inventory {
            pigment,
            binder,
            solvent,
            oxygen,
            oxidized_binder,
            networked_binder,
            inhibitor,
        } = MaterialPayload { fragments }.inventory();
        [
            pigment,
            binder,
            solvent,
            oxygen,
            oxidized_binder,
            networked_binder,
            inhibitor,
        ]
    }

    fn close(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.0),
            "{a} != {b}"
        );
    }

    #[test]
    fn heterogeneous_mobile_mixing_conserves_all_seven_amounts() {
        let a = fragment();
        let mut b = a.clone();
        b.binder = 30.0;
        b.solvent = 15.0;
        b.volume = 3.0;
        b.oxygen = 0.8;
        b.oxidation = 0.9;
        b.network = 0.95;
        b.inhibitor = 0.6;
        b.recipe.kinetics.oxygen_solubility = 1.2;
        b.recipe.kinetics.solvent_solubility = 0.1;
        let expected_capacity = a.oxygen_capacity() + b.oxygen_capacity();
        let expected = inventory(vec![a.clone(), b.clone()]);
        let c = mix_mobile(&[a, b], 4.0, MechanicalHistory::default()).unwrap();
        close(c.volume, 4.0);
        close(c.oxygen_capacity(), expected_capacity);
        close(c.oxygen, expected[3] / expected_capacity);
        assert!((c.oxygen - 0.725).abs() > 0.01);
        for (a, b) in expected.into_iter().zip(inventory(vec![c])) {
            close(a, b);
        }
        assert_eq!(std::mem::size_of::<super::super::chemistry::Cell>(), 32);
    }

    #[test]
    fn capacity_and_common_activity_survive_composition_changes_and_remixing() {
        let a = fragment();
        let mut b = a.clone();
        b.binder = 30.0;
        b.solvent = 20.0;
        b.recipe.kinetics.oxygen_solubility = 1.3;
        b.recipe.kinetics.solvent_solubility = 0.05;
        let expected_capacity = a.oxygen_capacity() + b.oxygen_capacity();
        let c = mix_mobile(&[a.clone(), b], 1.0, MechanicalHistory::default()).unwrap();
        close(c.oxygen_capacity(), expected_capacity);
        close(c.oxygen, a.oxygen);
        let expected = inventory(vec![c.clone(), a.clone()]);
        let d = mix_mobile(&[c, a], 1.0, MechanicalHistory::default()).unwrap();
        for (a, b) in expected.into_iter().zip(inventory(vec![d])) {
            close(a, b);
        }
    }

    #[test]
    fn optical_ids_named_binders_and_drier_provenance_survive() {
        let a = fragment();
        let mut b = a.clone();
        b.volume = 3.0;
        b.binder = 30.0;
        b.recipe.color = [0.6; crate::wet::LAT];
        b.recipe.parts = vec![(7, 1.0)];
        b.recipe.binders = [0.0, 1.0, 0.0, 0.0, 0.0];
        b.recipe.kinetics.binder_code = 2;
        b.recipe.kinetics.drier_code = 0;
        b.recipe.kinetics.provenance_code = 4;
        let scatter = 0.25 * a.properties()[0] + 0.75 * b.properties()[0];
        let c = mix_mobile(&[a, b], 1.0, MechanicalHistory::default()).unwrap();
        assert_eq!(c.recipe.parts, vec![(0, 0.25), (7, 0.75)]);
        assert!((c.recipe.color[0] - 0.5).abs() < 1e-6);
        assert!((c.properties()[0] - scatter).abs() < 1e-6);
        assert_eq!(c.recipe.binders, [0.25, 0.75, 0.0, 0.0, 0.0]);
        assert_eq!(c.recipe.kinetics.binder_code, 6);
        assert_eq!(c.recipe.kinetics.drier_code, 1);
        assert_eq!(
            c.recipe.kinetics.provenance_code,
            super::super::recipes::EXTRAPOLATED_RECIPE
        );
    }

    #[test]
    fn coefficients_use_declared_physical_weights_not_optical_weights() {
        let a = fragment();
        let mut b = a.clone();
        b.binder = 30.0;
        b.volume = 1.0;
        b.recipe.kinetics.oxidation_rate_s = 0.003;
        b.recipe.kinetics.autocatalysis = 4.0;
        b.recipe.kinetics.oxygen_demand = 0.5;
        let expected_a =
            (0.25 * 0.001 * a.recipe.kinetics.autocatalysis + 0.75 * 0.003 * 4.0) / 0.0025;
        let c = mix_mobile(&[a, b], 1.0, MechanicalHistory::default()).unwrap();
        close(c.recipe.kinetics.oxidation_rate_s, 0.0025);
        close(c.recipe.kinetics.autocatalysis, expected_a);
        close(
            c.recipe.kinetics.oxygen_demand,
            (0.25 * 0.001 * 0.16 + 0.75 * 0.003 * 0.5) / 0.0025,
        );
    }

    #[test]
    fn one_output_replaces_duplicate_histories_without_a_history_cap() {
        let a = fragment();
        let c = mix_mobile(&vec![a; 1024], 2.0, MechanicalHistory::default()).unwrap();
        close(c.binder, 10240.0);
        close(c.oxygen, 0.5);
        assert_eq!(c.recipe.parts, vec![(0, 1.0)]);
        assert_eq!(c.reference_area, 2.0);
    }

    #[test]
    fn zero_rates_and_absent_solvent_need_no_division_by_zero() {
        let mut a = fragment();
        a.solvent = 0.0;
        a.recipe.kinetics.oxidation_rate_s = 0.0;
        a.recipe.kinetics.inhibitor_rate_s = 0.0;
        a.recipe.kinetics.diffusivity_um2_s = 0.0;
        let c = mix_mobile(&[a.clone(), a], 1.0, MechanicalHistory::default()).unwrap();
        assert_eq!(c.recipe.kinetics.oxidation_rate_s, 0.0);
        assert_eq!(c.recipe.kinetics.inhibitor_rate_s, 0.0);
        assert_eq!(c.recipe.kinetics.diffusivity_um2_s, 0.0);
        close(c.recipe.kinetics.solvent_solubility, 0.4);
        close(c.oxygen, 0.5);
    }

    #[test]
    fn unsupported_identity_and_invalid_material_fail_explicitly() {
        let a = fragment();
        let mut b = a.clone();
        b.recipe.kinetics.drier_code = 9;
        assert_eq!(
            mix_mobile(&[a.clone(), b], 1.0, MechanicalHistory::default()),
            Err(MixtureError::DifferentDrierFamilies)
        );
        let mut b = a.clone();
        b.recipe.kinetics.revision += 1;
        assert_eq!(
            mix_mobile(&[a.clone(), b], 1.0, MechanicalHistory::default()),
            Err(MixtureError::DifferentRevisions)
        );
        let mut b = a;
        b.oxygen = f64::NAN;
        assert_eq!(
            mix_mobile(&[b], 1.0, MechanicalHistory::default()),
            Err(MixtureError::InvalidFragment(0))
        );
        assert_eq!(
            mix_mobile(&[], 1.0, MechanicalHistory::default()),
            Err(MixtureError::Empty)
        );
    }
}
