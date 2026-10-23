use super::chemistry::{ChemistryError, FixtureId, Kinetics, ObservationModel};

pub const EXTRAPOLATED_RECIPE: u32 = 100;

pub fn observations_for(kinetics: &Kinetics) -> ObservationModel {
    match kinetics.provenance_code {
        1 => FixtureId::S3Lwlo450.fixture().observations,
        2 => FixtureId::S3Ublo450.fixture().observations,
        3 => FixtureId::S3Lwso450.fixture().observations,
        4 => FixtureId::S3Ubso450.fixture().observations,
        5 => FixtureId::S2Golden76.fixture().observations,
        _ => ObservationModel::default(),
    }
}

pub fn kinetics_for(
    pigments: &[(&str, f64)],
    binders: [f64; 5],
    relative_drier: f64,
) -> Result<Kinetics, ChemistryError> {
    if binders.iter().any(|v| !v.is_finite() || *v < 0.0)
        || pigments.iter().any(|(_, v)| !v.is_finite() || *v < 0.0)
        || !relative_drier.is_finite() || relative_drier < 0.0
    {
        return Err(ChemistryError::InvalidKinetics);
    }
    let oil = binders.iter().sum::<f64>();
    if !oil.is_finite() || oil <= 0.0 || binders[4] > 0.0 {
        return Err(ChemistryError::InvalidKinetics);
    }
    let mut pigments = pigments.to_vec();
    pigments.sort_by(|a, b| a.0.cmp(b.0).then_with(|| a.1.total_cmp(&b.1)));
    pigments.retain(|(_, v)| *v > 0.0);
    if pigments.is_empty() { pigments.push(("unspecified", 1.0)); }
    let mut merged: Vec<(&str, f64)> = Vec::new();
    for (name, weight) in pigments {
        if let Some(last) = merged.last_mut().filter(|last| last.0 == name) { last.1 += weight; }
        else { merged.push((name, weight)); }
    }
    let pigments = merged;
    let total = pigments.iter().map(|(_, v)| *v).sum::<f64>();
    if !total.is_finite() || total <= 0.0 { return Err(ChemistryError::InvalidKinetics); }
    let mut rows = Vec::new();
    let mut expected_drier = 0.0;
    let mut extrapolated = false;
    for (name, weight) in pigments {
        let name = name.to_ascii_lowercase();
        let kind = match name.as_str() {
            "lead white" | "leadwhite" => 1,
            "ultramarine blue" | "synthetic ultramarine" | "ultramarineblue" => 2,
            "titanium white" | "titanium dioxide" => 3,
            _ => { extrapolated = true; 0 }
        };
        expected_drier += weight / total * match kind { 1 => 2.0, 2 => 0.95, _ => 1.0 };
        for (binder, share) in binders.iter().copied().enumerate().filter(|(_, v)| *v > 0.0) {
            let id = match (kind, binder) {
                (1, 0) => FixtureId::S3Lwlo450,
                (1, 1) => FixtureId::S3Lwso450,
                (2, 0) => FixtureId::S3Ublo450,
                (2, 1) => FixtureId::S3Ubso450,
                (1, _) => { extrapolated = true; FixtureId::S3Lwlo450 },
                (2, _) => { extrapolated = true; FixtureId::S3Ublo450 },
                (_, 0) => FixtureId::S2Golden76,
                _ => { extrapolated = true; FixtureId::S2Golden76 }
            };
            let mut law = id.fixture().kinetics;
            let binder_activity: f64 = match binder { 2 => 0.8, 3 => 0.6, _ => 1.0 };
            law.oxidation_rate_s *= binder_activity;
            law.inhibitor_rate_s *= binder_activity.sqrt();
            rows.push((law, weight / total * share / oil));
        }
    }
    let mut result = rows[0].0;
    macro_rules! mean {
        ($($field:ident),+) => { $(result.$field = rows.iter().map(|(k, w)| k.$field * w).sum();)+ };
    }
    mean!(oxygen_solubility, solvent_solubility, diffusivity_um2_s, cured_diffusivity_ratio,
        air_transfer_um_s, oxidation_rate_s, autocatalysis, oxygen_demand, inhibitor_rate_s,
        inhibitor_oxygen_demand, inhibition_strength, network_rate_s);
    let activity = relative_drier / expected_drier;
    result.oxidation_rate_s *= activity;
    result.inhibitor_rate_s *= activity.sqrt();
    result.solvent_evaporation_rate_s = 1.0 / (60.0 * crate::thinner::TAU_MIN);
    let active: Vec<_> = binders.iter().enumerate().filter(|(_, v)| **v > 0.0).collect();
    result.binder_code = if active.len() == 1 { active[0].0 as u32 + 1 } else { 6 };
    result.provenance_code = if !extrapolated && rows.len() == 1 && (activity - 1.0).abs() < 1e-6 {
        rows[0].0.provenance_code
    } else {
        EXTRAPOLATED_RECIPE
    };
    result.drier_code = u32::from(rows.iter().any(|(k, _)| k.drier_code != 0));
    result.validate()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_recipes_preserve_named_binders_and_source_families() {
        let linseed = kinetics_for(&[("lead white", 1.0)], [1.0, 0.0, 0.0, 0.0, 0.0], 2.0).unwrap();
        let safflower = kinetics_for(&[("lead white", 1.0)], [0.0, 1.0, 0.0, 0.0, 0.0], 2.0).unwrap();
        assert_eq!((linseed.binder_code, linseed.provenance_code), (1, 1));
        assert_eq!((safflower.binder_code, safflower.provenance_code), (2, 3));
        assert_ne!(linseed.network_rate_s, safflower.network_rate_s);
        assert_eq!(linseed.oxidation_rate_s, FixtureId::S3Lwlo450.fixture().kinetics.oxidation_rate_s);
        assert_eq!(linseed, kinetics_for(&[("lead white", 1.0), ("lead white", 1.0)], [1.0, 0.0, 0.0, 0.0, 0.0], 2.0).unwrap());
        let unknown = kinetics_for(&[("bone black", 1.0)], [1.0, 0.0, 0.0, 0.0, 0.0], 0.4).unwrap();
        assert_eq!(unknown.provenance_code, EXTRAPOLATED_RECIPE);
        let parts = [("lead white", 2.0), ("ultramarine blue", 1.0)];
        let a = kinetics_for(&parts, [0.7, 0.3, 0.0, 0.0, 0.0], 1.65).unwrap();
        let b = kinetics_for(&[parts[1], parts[0]], [0.7, 0.3, 0.0, 0.0, 0.0], 1.65).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.binder_code, 6);
        assert!(kinetics_for(&parts, [0.0; 5], 1.0).is_err());
    }
}
