use super::constitutive::Material;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReleaseInterface {
    pub strength_pa: f64,
    pub separation_work_j_m2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KnifeCell {
    pub thickness_m: f64,
    pub material: Material,
    pub damage: f64,
    pub release: Option<ReleaseInterface>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactCalibration {
    pub resolved_stress_at_unit_pressure_pa: f64,
}

impl Default for ContactCalibration {
    fn default() -> Self {
        Self {
            resolved_stress_at_unit_pressure_pa: 20_000.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Contact {
    pub normalized_pressure: f64,
    pub calibration: ContactCalibration,
    pub blade_work_j_m2: f64,
    pub max_depth_m: f64,
    pub shear_rate_s: f64,
    pub shear_strain: f64,
    pub cut_spacing_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidContact,
    InvalidCell(usize),
    NumericalFailure,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellReceipt {
    pub removed_fraction: f64,
    pub coherent_removed_fraction: f64,
    pub fragment_damage: f64,
    pub rheology_work_j_m2: f64,
    pub fracture_work_j_m2: f64,
    pub adhesion_work_j_m2: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reach {
    pub depth_m: f64,
    pub removed_fraction: f64,
    pub spent_work_j_m2: f64,
    pub remaining_work_j_m2: f64,
    pub cells: Vec<CellReceipt>,
}

pub fn reach(cells: &[KnifeCell], contact: Contact) -> Result<Reach, Error> {
    if !contact.normalized_pressure.is_finite()
        || !(0.0..=1.0).contains(&contact.normalized_pressure)
        || [
            contact.calibration.resolved_stress_at_unit_pressure_pa,
            contact.blade_work_j_m2,
            contact.max_depth_m,
            contact.shear_rate_s,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0)
        || [contact.shear_strain, contact.cut_spacing_m]
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.0)
    {
        return Err(Error::InvalidContact);
    }
    let mut thickness = 0.0;
    for (i, cell) in cells.iter().enumerate() {
        if !cell.thickness_m.is_finite()
            || cell.thickness_m < 0.0
            || !cell.damage.is_finite()
            || !(0.0..=1.0).contains(&cell.damage)
            || cell.material.validate().is_err()
            || cell.release.is_some_and(|r| {
                [r.strength_pa, r.separation_work_j_m2]
                    .iter()
                    .any(|v| !v.is_finite() || *v < 0.0)
            })
        {
            return Err(Error::InvalidCell(i));
        }
        thickness += cell.thickness_m;
    }
    if !thickness.is_finite() {
        return Err(Error::NumericalFailure);
    }
    let stress =
        contact.normalized_pressure * contact.calibration.resolved_stress_at_unit_pressure_pa;
    let mut result = Reach {
        cells: vec![CellReceipt::default(); cells.len()],
        remaining_work_j_m2: contact.blade_work_j_m2,
        ..Reach::default()
    };
    if stress == 0.0 || contact.blade_work_j_m2 == 0.0 {
        return Ok(result);
    }
    for (i, cell) in cells.iter().enumerate() {
        if cell.thickness_m == 0.0 {
            continue;
        }
        let h = cell
            .thickness_m
            .min((contact.max_depth_m - result.depth_m).max(0.0));
        if h == 0.0 {
            break;
        }
        let intact = 1.0 - cell.damage;
        let flow_stress = cell.material.shear_yield_pa
            + cell.material.plastic_viscosity_pa_s * contact.shear_rate_s;
        let cohesive_stress = intact * cell.material.cohesive_strength_pa;
        let interface = cell.release.unwrap_or(ReleaseInterface {
            strength_pa: 0.0,
            separation_work_j_m2: 0.0,
        });
        let required_stress = (flow_stress + cohesive_stress).max(intact * interface.strength_pa);
        let adhesion_work = intact * interface.separation_work_j_m2;
        let rheology_density = flow_stress * contact.shear_strain;
        let fracture_density = intact * cell.material.fracture_energy_j_m2 / contact.cut_spacing_m;
        let work_density = rheology_density + fracture_density;
        if [
            required_stress,
            adhesion_work,
            rheology_density,
            fracture_density,
            work_density,
        ]
        .iter()
        .any(|v| !v.is_finite())
        {
            return Err(Error::NumericalFailure);
        }
        if stress <= required_stress || result.remaining_work_j_m2 <= adhesion_work {
            break;
        }
        let available = result.remaining_work_j_m2 - adhesion_work;
        let removed = if work_density > 0.0 {
            h.min(available / work_density)
        } else {
            h
        };
        if removed <= 0.0 {
            break;
        }
        let fraction = removed / cell.thickness_m;
        let receipt = CellReceipt {
            removed_fraction: fraction,
            coherent_removed_fraction: fraction * cell.material.coherence,
            fragment_damage: if cell.material.coherence > 0.0 {
                1.0
            } else {
                cell.damage
            },
            rheology_work_j_m2: rheology_density * removed,
            fracture_work_j_m2: fracture_density * removed,
            adhesion_work_j_m2: adhesion_work,
        };
        let spent =
            receipt.rheology_work_j_m2 + receipt.fracture_work_j_m2 + receipt.adhesion_work_j_m2;
        result.depth_m += removed;
        result.spent_work_j_m2 += spent;
        result.remaining_work_j_m2 = (result.remaining_work_j_m2 - spent).max(0.0);
        result.cells[i] = receipt;
        if removed < cell.thickness_m {
            break;
        }
    }
    result.removed_fraction = if thickness > 0.0 {
        result.depth_m / thickness
    } else {
        0.0
    };
    if !result.spent_work_j_m2.is_finite() || !result.depth_m.is_finite() {
        return Err(Error::NumericalFailure);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::super::chemistry::{Cell, Kinetics};
    use super::super::constitutive::{Parameters, evaluate};
    use super::super::store::Formulation;
    use super::*;
    use crate::wet::Latent;

    fn material(network: f32) -> Material {
        let recipe = Formulation::new(Latent::default(), [0.0; 10], Kinetics::default()).unwrap();
        evaluate(
            &Cell {
                pigment_um: 32.0,
                binder_um: 68.0,
                network,
                oxidation: network,
                ..Cell::default()
            },
            &recipe,
            &Parameters::default(),
        )
        .unwrap()
    }

    fn contact(pressure: f64, work: f64) -> Contact {
        Contact {
            normalized_pressure: pressure,
            calibration: ContactCalibration::default(),
            blade_work_j_m2: work,
            max_depth_m: 1e-3,
            shear_rate_s: 0.1,
            shear_strain: 1.0,
            cut_spacing_m: 1e-3,
        }
    }

    #[test]
    fn oil_knife_pressure_monotonicity_and_finite_set_reach() {
        let cells: Vec<_> = [0.0, 0.4, 0.7, 1.0]
            .into_iter()
            .map(|n| KnifeCell {
                thickness_m: 30e-6,
                material: material(n),
                damage: 0.0,
                release: None,
            })
            .collect();
        let mut previous = 0.0;
        for p in [0.0, 0.001, 0.02, 0.1, 0.3, 0.6, 1.0] {
            let result = reach(&cells, contact(p, 100.0)).unwrap();
            assert!(result.depth_m >= previous);
            previous = result.depth_m;
        }
        assert!(reach(&cells, contact(0.6, 100.0)).unwrap().depth_m > 60e-6);
        let set = [KnifeCell {
            thickness_m: 100e-6,
            material: material(0.8),
            damage: 0.0,
            release: None,
        }];
        assert_eq!(reach(&set, contact(0.001, 100.0)).unwrap().depth_m, 0.0);
        assert!(reach(&set, contact(1.0, 100.0)).unwrap().depth_m > 0.0);
        let hard = [KnifeCell {
            material: Material {
                cohesive_strength_pa: 1e6,
                ..material(1.0)
            },
            ..set[0]
        }];
        assert_eq!(reach(&hard, contact(1.0, 100.0)).unwrap().depth_m, 0.0);
    }

    #[test]
    fn oil_knife_work_receipts_preserve_top_down_order() {
        let cells = [
            KnifeCell {
                thickness_m: 100e-6,
                material: material(0.8),
                damage: 0.1,
                release: None,
            },
            KnifeCell {
                thickness_m: 50e-6,
                material: material(0.0),
                damage: 0.0,
                release: None,
            },
        ];
        let before = cells;
        let limited = reach(&cells, contact(1.0, 0.05)).unwrap();
        assert!(limited.cells[0].removed_fraction > 0.0);
        assert!(limited.cells[0].removed_fraction < 1.0);
        assert_eq!(limited.cells[1].removed_fraction, 0.0);
        assert!(limited.cells[0].coherent_removed_fraction > 0.0);
        assert!(limited.cells[0].fragment_damage > cells[0].damage);
        let sum: f64 = limited
            .cells
            .iter()
            .map(|r| r.rheology_work_j_m2 + r.fracture_work_j_m2 + r.adhesion_work_j_m2)
            .sum();
        assert!((sum - limited.spent_work_j_m2).abs() < 1e-14);
        assert!((limited.spent_work_j_m2 + limited.remaining_work_j_m2 - 0.05).abs() < 1e-14);
        assert_eq!(cells, before);
        let mut previous = 0.0;
        for work in [0.0, 0.01, 0.05, 0.2, 1.0, 10.0] {
            let result = reach(&cells, contact(1.0, work)).unwrap();
            assert!(result.depth_m >= previous);
            assert!(result.spent_work_j_m2 <= work + 1e-12);
            previous = result.depth_m;
        }
    }

    #[test]
    fn oil_knife_only_explicit_interfaces_cost_adhesion_work() {
        let plain = KnifeCell {
            thickness_m: 50e-6,
            material: material(0.8),
            damage: 0.0,
            release: None,
        };
        let bonded = KnifeCell {
            release: Some(ReleaseInterface {
                strength_pa: 500.0,
                separation_work_j_m2: 0.5,
            }),
            ..plain
        };
        assert!(reach(&[plain], contact(1.0, 0.1)).unwrap().depth_m > 0.0);
        assert_eq!(reach(&[bonded], contact(1.0, 0.1)).unwrap().depth_m, 0.0);
        let result = reach(&[bonded], contact(1.0, 2.0)).unwrap();
        assert_eq!(result.cells[0].adhesion_work_j_m2, 0.5);
        assert!(
            reach(
                &[KnifeCell {
                    material: Material {
                        young_pa: f64::NAN,
                        ..plain.material
                    },
                    ..plain
                }],
                contact(1.0, 2.0)
            )
            .is_err()
        );
        assert!(
            reach(
                &[plain],
                Contact {
                    normalized_pressure: f64::NAN,
                    ..contact(1.0, 2.0)
                }
            )
            .is_err()
        );
    }
}
