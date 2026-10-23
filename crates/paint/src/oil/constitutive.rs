use std::sync::Arc;

use super::chemistry::Cell;
use super::store::Formulation;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parameters {
    pub gel_network: f64,
    pub young_network_pa: f64,
    pub poisson: f64,
    pub fresh_yield_scale_pa: f64,
    pub measured_fresh_shear_yield_pa: Option<f64>,
    pub network_yield_pa: f64,
    pub plastic_viscosity_pa_s: f64,
    pub relaxation_seconds: f64,
    pub oxidation_expansion: f64,
    pub network_contraction: f64,
    pub cohesive_strength_pa: f64,
    pub fracture_energy_j_m2: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            gel_network: 0.2,
            young_network_pa: 200_000.0,
            poisson: 0.35,
            fresh_yield_scale_pa: 75.0,
            measured_fresh_shear_yield_pa: None,
            network_yield_pa: 2_000.0,
            plastic_viscosity_pa_s: 20.0,
            relaxation_seconds: 3_600.0,
            oxidation_expansion: 0.02,
            network_contraction: 0.005,
            cohesive_strength_pa: 20_000.0,
            fracture_energy_j_m2: 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidParameters,
    InvalidCell,
    MissingRecipe,
    InvalidDepth,
    InvalidDiagnostic,
    ChangedComposition,
    NumericalFailure,
}

impl Parameters {
    pub fn validate(&self) -> Result<(), Error> {
        if !self.gel_network.is_finite()
            || !(0.0..1.0).contains(&self.gel_network)
            || !self.poisson.is_finite()
            || self.poisson <= -1.0
            || self.poisson >= 0.5
            || [
                self.young_network_pa,
                self.fresh_yield_scale_pa,
                self.network_yield_pa,
                self.plastic_viscosity_pa_s,
                self.relaxation_seconds,
                self.oxidation_expansion,
                self.network_contraction,
                self.cohesive_strength_pa,
                self.fracture_energy_j_m2,
            ]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || self.relaxation_seconds == 0.0
            || self
                .measured_fresh_shear_yield_pa
                .is_some_and(|v| !v.is_finite() || v < 0.0)
        {
            return Err(Error::InvalidParameters);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Material {
    pub coherence: f64,
    pub young_pa: f64,
    pub poisson: f64,
    pub shear_yield_pa: f64,
    pub plastic_viscosity_pa_s: f64,
    pub relaxation_seconds: f64,
    pub cohesive_strength_pa: f64,
    pub fracture_energy_j_m2: f64,
}

impl Material {
    pub fn validate(&self) -> Result<(), Error> {
        if !self.coherence.is_finite()
            || !(0.0..=1.0).contains(&self.coherence)
            || !self.poisson.is_finite()
            || self.poisson <= -1.0
            || self.poisson >= 0.5
            || [
                self.young_pa,
                self.shear_yield_pa,
                self.plastic_viscosity_pa_s,
                self.relaxation_seconds,
                self.cohesive_strength_pa,
                self.fracture_energy_j_m2,
            ]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(Error::InvalidParameters);
        }
        Ok(())
    }
}

fn validate_cell(cell: &Cell) -> Result<(), Error> {
    if [cell.pigment_um, cell.binder_um, cell.solvent_um]
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0)
        || [cell.oxygen, cell.oxidation, cell.network, cell.inhibitor]
            .iter()
            .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
    {
        return Err(Error::InvalidCell);
    }
    Ok(())
}

pub fn evaluate(
    cell: &Cell,
    formulation: &Formulation,
    parameters: &Parameters,
) -> Result<Material, Error> {
    parameters.validate()?;
    validate_cell(cell)?;
    let h = cell.thickness_um();
    if h == 0.0 {
        return Ok(Material {
            poisson: parameters.poisson,
            relaxation_seconds: parameters.relaxation_seconds,
            ..Material::default()
        });
    }
    let pigment = cell.pigment_um as f64 / h;
    let solvent = cell.solvent_um as f64 / h;
    let dry_binder = cell.binder_um as f64
        / (cell.pigment_um as f64 + cell.binder_um as f64).max(f64::MIN_POSITIVE);
    let prop = formulation.prop();
    let tube_oil = (prop[8] as f64).clamp(0.0, 1.0);
    let packed_oil = (prop[5] as f64).clamp(0.0, 1.0);
    let packing =
        (1.0 - tube_oil) / (1.0 - tube_oil + tube_oil * packed_oil).max(f64::MIN_POSITIVE);
    let packing_factor = 1.0 + pigment * (1.0 - packing);
    let binders = formulation.binders();
    let binder_sum: f64 = binders.iter().sum();
    if binders.iter().any(|v| !v.is_finite() || *v < 0.0)
        || !binder_sum.is_finite()
        || binder_sum <= 0.0
    {
        return Err(Error::InvalidParameters);
    }
    let oil_viscosity = binders
        .iter()
        .zip([1.0, 0.85, 0.7, 0.75, 8.0])
        .map(|(w, factor)| w * factor / binder_sum)
        .sum::<f64>();
    let network_factor = binders
        .iter()
        .zip([1.0, 0.9, 0.8, 0.8, 1.1])
        .map(|(w, factor)| w * factor / binder_sum)
        .sum::<f64>();
    let t = ((cell.network as f64 - parameters.gel_network) / (1.0 - parameters.gel_network))
        .clamp(0.0, 1.0);
    let coherence = t * t * (3.0 - 2.0 * t);
    let connected = coherence * dry_binder;
    let reinforcement = 1.0 + 2.5 * pigment + 8.0 * pigment * pigment;
    let dilution = (-6.0 * solvent).exp();
    let fresh_yield = parameters.measured_fresh_shear_yield_pa.unwrap_or_else(|| {
        parameters.fresh_yield_scale_pa
            * (pigment / 0.32).powi(2)
            * (4.0 * (pigment - 0.32)).exp()
            * oil_viscosity
            * packing_factor
            * dilution
    });
    let material = Material {
        coherence: connected,
        young_pa: parameters.young_network_pa
            * connected
            * network_factor
            * reinforcement
            * dilution,
        poisson: parameters.poisson,
        shear_yield_pa: fresh_yield + parameters.network_yield_pa * connected * network_factor,
        plastic_viscosity_pa_s: parameters.plastic_viscosity_pa_s
            * oil_viscosity
            * (1.0 + 20.0 * pigment * pigment)
            * (1.0 + 100.0 * connected * connected)
            * dilution,
        relaxation_seconds: parameters.relaxation_seconds * (1.0 + 20.0 * connected),
        cohesive_strength_pa: parameters.cohesive_strength_pa * connected * network_factor,
        fracture_energy_j_m2: parameters.fracture_energy_j_m2 * connected * network_factor,
    };
    material.validate()?;
    Ok(material)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChemicalIncrement {
    pub free_strain: f64,
    pub tension_increment_pa: f64,
    pub retained_stress_fraction: f64,
}

pub fn chemical_increment(
    old: &Cell,
    new: &Cell,
    formulation: &Formulation,
    parameters: &Parameters,
    dt_seconds: f64,
) -> Result<ChemicalIncrement, Error> {
    let material = evaluate(old, formulation, parameters)?;
    validate_cell(new)?;
    if !dt_seconds.is_finite() || dt_seconds < 0.0 {
        return Err(Error::InvalidParameters);
    }
    if old.recipe != new.recipe
        || old.pigment_um != new.pigment_um
        || old.binder_um != new.binder_um
        || new.oxidation < old.oxidation
        || new.network < old.network
    {
        return Err(Error::ChangedComposition);
    }
    let binder_fraction = old.binder_um as f64
        / (old.pigment_um as f64 + old.binder_um as f64).max(f64::MIN_POSITIVE);
    let free_strain = binder_fraction
        * (parameters.oxidation_expansion * (new.oxidation as f64 - old.oxidation as f64)
            - parameters.network_contraction * (new.network as f64 - old.network as f64));
    let x = dt_seconds / material.relaxation_seconds;
    let source_retention = if x > 0.0 { -(-x).exp_m1() / x } else { 1.0 };
    let result = ChemicalIncrement {
        free_strain,
        tension_increment_pa: -material.young_pa / (1.0 - material.poisson)
            * free_strain
            * source_retention,
        retained_stress_fraction: (-x).exp(),
    };
    if [
        result.free_strain,
        result.tension_increment_pa,
        result.retained_stress_fraction,
    ]
    .iter()
    .any(|v| !v.is_finite())
    {
        return Err(Error::NumericalFailure);
    }
    Ok(result)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DepthCell {
    pub z_lo_m: f64,
    pub z_hi_m: f64,
    pub young_pa: f64,
    pub poisson: f64,
}

pub type Matrix = [[f64; 3]; 3];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rigidity {
    pub thickness_m: f64,
    pub z_ref_m: f64,
    pub a: Matrix,
    pub b: Matrix,
    pub d: Matrix,
    pub neutral_axis_m: f64,
    pub neutral_bending_n_m: f64,
}

pub fn integrate_depth(cells: &[DepthCell], z_ref_m: f64) -> Result<Rigidity, Error> {
    integrate_depth_iter(cells.iter().copied().map(Ok), z_ref_m)
}

fn integrate_depth_iter(
    cells: impl Iterator<Item = Result<DepthCell, Error>>,
    z_ref_m: f64,
) -> Result<Rigidity, Error> {
    if !z_ref_m.is_finite() {
        return Err(Error::InvalidDepth);
    }
    let mut result = Rigidity {
        z_ref_m,
        ..Rigidity::default()
    };
    let mut previous_hi = None;
    let mut neutral_weight = 0.0;
    let mut neutral_offset = 0.0;
    for cell in cells {
        let cell = cell?;
        if !cell.z_lo_m.is_finite()
            || !cell.z_hi_m.is_finite()
            || cell.z_hi_m < cell.z_lo_m
            || previous_hi.is_some_and(|hi| cell.z_lo_m < hi)
            || !cell.young_pa.is_finite()
            || cell.young_pa < 0.0
            || !cell.poisson.is_finite()
            || cell.poisson <= -1.0
            || cell.poisson >= 0.5
        {
            return Err(Error::InvalidDepth);
        }
        previous_hi = Some(cell.z_hi_m);
        let h = cell.z_hi_m - cell.z_lo_m;
        let z = (cell.z_lo_m - z_ref_m) + 0.5 * h;
        let q = cell.young_pa / (1.0 - cell.poisson * cell.poisson);
        let tangent = [
            [q, q * cell.poisson, 0.0],
            [q * cell.poisson, q, 0.0],
            [0.0, 0.0, 0.5 * q * (1.0 - cell.poisson)],
        ];
        let weight = q * h;
        if weight > 0.0 {
            let total = neutral_weight + weight;
            let delta = z - neutral_offset;
            result.neutral_bending_n_m += weight * h * h / 12.0
                + delta * delta * (neutral_weight / total) * weight;
            neutral_offset += delta * (weight / total);
            neutral_weight = total;
        }
        result.thickness_m += h;
        for i in 0..3 {
            for j in 0..3 {
                result.a[i][j] += tangent[i][j] * h;
                result.b[i][j] += tangent[i][j] * h * z;
                result.d[i][j] += tangent[i][j] * h * (z * z + h * h / 12.0);
            }
        }
    }
    result.neutral_axis_m = z_ref_m + neutral_offset;
    if result
        .a
        .iter()
        .chain(result.b.iter())
        .chain(result.d.iter())
        .flatten()
        .chain([&result.neutral_axis_m, &result.neutral_bending_n_m])
        .any(|v| !v.is_finite())
    {
        return Err(Error::NumericalFailure);
    }
    Ok(result)
}

pub fn integrate_column(
    cells: &[Cell],
    formulations: &[Arc<Formulation>],
    parameters: &Parameters,
) -> Result<Rigidity, Error> {
    parameters.validate()?;
    let thickness_m = cells
        .iter()
        .map(|cell| cell.thickness_um() * 1e-6)
        .sum::<f64>();
    let mut z = 0.0;
    let depth = cells.iter().map(|cell| {
        let formulation = formulations
            .get(cell.recipe as usize)
            .ok_or(Error::MissingRecipe)?;
        let material = evaluate(cell, formulation, parameters)?;
        let h = cell.thickness_um() * 1e-6;
        let result = DepthCell {
            z_lo_m: z,
            z_hi_m: z + h,
            young_pa: material.young_pa,
            poisson: material.poisson,
        };
        z += h;
        Ok(result)
    });
    integrate_depth_iter(depth, 0.5 * thickness_m)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccuracyFloors {
    pub extension_n_m: f64,
    pub bending_n_m: f64,
    pub threshold: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RefinementAccuracy {
    pub extension_relative: f64,
    pub coupling_relative: f64,
    pub bending_relative: f64,
    pub threshold_relative: f64,
    pub threshold_sign_changed: bool,
}

impl RefinementAccuracy {
    pub fn maximum_relative(self) -> f64 {
        self.extension_relative
            .max(self.coupling_relative)
            .max(self.bending_relative)
            .max(self.threshold_relative)
    }
}

pub fn refinement_accuracy(
    coarse: &Rigidity,
    refined: &Rigidity,
    thresholds: [f64; 2],
    floors: AccuracyFloors,
) -> Result<RefinementAccuracy, Error> {
    if [floors.extension_n_m, floors.bending_n_m, floors.threshold]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.0)
        || thresholds.iter().any(|v| !v.is_finite())
        || [coarse, refined].iter().any(|r| {
            r.a.iter()
                .chain(r.b.iter())
                .chain(r.d.iter())
                .flatten()
                .chain([&r.neutral_bending_n_m, &r.z_ref_m])
                .any(|v| !v.is_finite())
        })
    {
        return Err(Error::InvalidDiagnostic);
    }
    let mut refined = *refined;
    let shift = refined.z_ref_m - coarse.z_ref_m;
    for i in 0..3 {
        for j in 0..3 {
            refined.d[i][j] += 2.0 * shift * refined.b[i][j] + shift * shift * refined.a[i][j];
            refined.b[i][j] += shift * refined.a[i][j];
        }
    }
    if refined
        .b
        .iter()
        .chain(refined.d.iter())
        .flatten()
        .any(|v| !v.is_finite())
    {
        return Err(Error::NumericalFailure);
    }
    let norm = |m: &Matrix| m.iter().flatten().fold(0.0_f64, |v, x| v.max(x.abs()));
    let difference = |a: &Matrix, b: &Matrix| {
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .fold(0.0_f64, |v, (x, y)| v.max((x - y).abs()))
    };
    let a_scale = norm(&coarse.a)
        .max(norm(&refined.a))
        .max(floors.extension_n_m);
    let d_scale = norm(&coarse.d)
        .max(norm(&refined.d))
        .max(floors.bending_n_m);
    let neutral_scale = coarse
        .neutral_bending_n_m
        .abs()
        .max(refined.neutral_bending_n_m.abs())
        .max(floors.bending_n_m);
    Ok(RefinementAccuracy {
        extension_relative: difference(&coarse.a, &refined.a) / a_scale,
        coupling_relative: difference(&coarse.b, &refined.b) / (a_scale.sqrt() * d_scale.sqrt()),
        bending_relative: (difference(&coarse.d, &refined.d) / d_scale)
            .max((coarse.neutral_bending_n_m - refined.neutral_bending_n_m).abs() / neutral_scale),
        threshold_relative: (thresholds[0] - thresholds[1]).abs()
            / thresholds[0]
                .abs()
                .max(thresholds[1].abs())
                .max(floors.threshold),
        threshold_sign_changed: (thresholds[0] < 0.0) != (thresholds[1] < 0.0),
    })
}

#[cfg(test)]
mod tests {
    use super::super::chemistry::Kinetics;
    use super::*;
    use crate::wet::Latent;

    fn recipe() -> Formulation {
        Formulation::new(Latent::default(), [0.0; 10], Kinetics::default()).unwrap()
    }

    fn cell(network: f32) -> Cell {
        Cell {
            pigment_um: 32.0,
            binder_um: 68.0,
            network,
            oxidation: network,
            ..Cell::default()
        }
    }

    #[test]
    fn oil_constitutive_depth_integration_and_refinement() {
        let h = 40e-6;
        let e = 200_000.0;
        let nu = 0.35;
        let full = DepthCell {
            z_lo_m: 0.0,
            z_hi_m: h,
            young_pa: e,
            poisson: nu,
        };
        let one = integrate_depth(&[full], h / 2.0).unwrap();
        let two = integrate_depth(
            &[
                DepthCell {
                    z_hi_m: h / 2.0,
                    ..full
                },
                DepthCell {
                    z_lo_m: h / 2.0,
                    ..full
                },
            ],
            h / 2.0,
        )
        .unwrap();
        let expected = e * h.powi(3) / (12.0 * (1.0 - nu * nu));
        assert!((one.neutral_bending_n_m / expected - 1.0).abs() < 1e-12);
        assert!((one.neutral_axis_m - h / 2.0).abs() < 1e-18);
        let floors = AccuracyFloors {
            extension_n_m: 1e-12,
            bending_n_m: 1e-24,
            threshold: 1e-12,
        };
        assert!(
            refinement_accuracy(&one, &two, [1.0, 1.0], floors)
                .unwrap()
                .maximum_relative()
                < 1e-12
        );
        let shifted = integrate_depth(&[full], 0.0).unwrap();
        assert!(
            refinement_accuracy(&one, &shifted, [1.0, 1.0], floors)
                .unwrap()
                .maximum_relative()
                < 1e-12
        );
        let thin = DepthCell { z_lo_m: 1e-3, z_hi_m: 1e-3 + 1e-9, ..full };
        let thin_rigidity = integrate_depth(&[thin], 0.0).unwrap();
        let thin_expected = e * (thin.z_hi_m - thin.z_lo_m).powi(3)
            / (12.0 * (1.0 - nu * nu));
        assert!((thin_rigidity.neutral_bending_n_m / thin_expected - 1.0).abs() < 1e-12);
        let layered = integrate_depth(
            &[
                DepthCell {
                    z_hi_m: h / 2.0,
                    young_pa: 0.0,
                    ..full
                },
                DepthCell {
                    z_lo_m: h / 2.0,
                    young_pa: 2.0 * e,
                    ..full
                },
            ],
            h / 2.0,
        )
        .unwrap();
        assert!((layered.a[0][0] / one.a[0][0] - 1.0).abs() < 1e-12);
        assert!((layered.neutral_axis_m / h - 0.75).abs() < 1e-12);
        let error = refinement_accuracy(&one, &layered, [0.1, -0.1], floors).unwrap();
        assert!(error.bending_relative > 0.5);
        assert!(error.threshold_sign_changed);
        assert!(error.maximum_relative() >= 1.0);
        assert!(
            integrate_depth(
                &[DepthCell {
                    poisson: -1.0,
                    ..full
                }],
                0.0
            )
            .is_err()
        );
    }

    #[test]
    fn oil_constitutive_stress_free_birth_and_signed_strain() {
        let recipe = recipe();
        let parameters = Parameters {
            network_contraction: 0.0,
            ..Parameters::default()
        };
        let fresh = cell(0.0);
        let born = cell(0.7);
        let increment = chemical_increment(&fresh, &born, &recipe, &parameters, 60.0).unwrap();
        assert!(increment.free_strain > 0.0);
        assert_eq!(increment.tension_increment_pa, 0.0);
        let older = Cell {
            oxidation: 0.8,
            ..born
        };
        let expansive = chemical_increment(&born, &older, &recipe, &parameters, 60.0).unwrap();
        assert!(expansive.tension_increment_pa < 0.0);
        let shrink = Parameters {
            oxidation_expansion: 0.0,
            network_contraction: 0.02,
            ..parameters
        };
        let contracted = cell(0.8);
        assert!(
            chemical_increment(&born, &contracted, &recipe, &shrink, 60.0)
                .unwrap()
                .tension_increment_pa
                > 0.0
        );
        assert!(expansive.retained_stress_fraction < 1.0);
    }

    #[test]
    fn oil_constitutive_inventory_recipe_and_smoothness() {
        let recipe = recipe();
        let p = Parameters::default();
        let paint = cell(0.6);
        let rich = Cell {
            binder_um: 136.0,
            ..paint
        };
        let a = evaluate(&paint, &recipe, &p).unwrap();
        let b = evaluate(&rich, &recipe, &p).unwrap();
        assert!(a.shear_yield_pa > b.shear_yield_pa);
        assert_ne!(a.young_pa, b.young_pa);
        let packing_recipe = Formulation::new(
            Latent::default(),
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.8, 0.0],
            Kinetics::default(),
        )
        .unwrap();
        assert_ne!(
            a.shear_yield_pa,
            evaluate(&paint, &packing_recipe, &p)
                .unwrap()
                .shear_yield_pa
        );
        let stand = Formulation::new_with_binders(
            Latent::default(),
            [0.0; 10],
            Kinetics::default(),
            [0.0, 0.0, 0.0, 0.0, 1.0],
        )
        .unwrap();
        let stand_material = evaluate(&paint, &stand, &p).unwrap();
        assert!(stand_material.plastic_viscosity_pa_s > a.plastic_viscosity_pa_s);
        assert_ne!(stand_material.young_pa, a.young_pa);
        let formulas = [Arc::new(recipe.clone())];
        let one = integrate_column(&[paint], &formulas, &p).unwrap();
        let half = Cell {
            pigment_um: 16.0,
            binder_um: 34.0,
            ..paint
        };
        let two = integrate_column(&[half, half], &formulas, &p).unwrap();
        assert!((one.neutral_bending_n_m / two.neutral_bending_n_m - 1.0).abs() < 1e-12);
        let below = evaluate(&cell(0.2 - 1e-5), &recipe, &p).unwrap();
        let above = evaluate(&cell(0.2 + 1e-5), &recipe, &p).unwrap();
        assert_eq!(below.young_pa, 0.0);
        assert!(above.young_pa < 0.01);
        let measured = Parameters {
            measured_fresh_shear_yield_pa: Some(3200.0),
            ..p
        };
        assert_eq!(
            evaluate(&cell(0.0), &recipe, &measured)
                .unwrap()
                .shear_yield_pa,
            3200.0
        );
    }
}
