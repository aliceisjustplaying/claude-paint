use super::chemistry::Cell;
use super::constitutive::{self, Parameters};
use super::exchange::{Fragment, MaterialPayload};
use super::knife::{self, Contact, ContactCalibration, KnifeCell, Reach};
use super::store::{Formulation, MaterialError};
use crate::wet::{Latent, Layer, Prop};
use crate::{COAT_UM, Canvas};

pub(crate) struct Cut {
    pub(crate) volume_coats: f32,
    pub(crate) solvent_coats: f32,
    pub(crate) latent: Latent,
    pub(crate) properties: Prop,
    pub(crate) cure: f32,
    pub(crate) receipt: Reach,
}

pub(crate) fn blade_contact(pressure: f32, travel_m: f64, depth_m: f64) -> Contact {
    let calibration = ContactCalibration::default();
    let normalized_pressure = pressure.clamp(0.0, 1.0) as f64;
    Contact {
        normalized_pressure,
        calibration,
        blade_work_j_m2: normalized_pressure
            * calibration.resolved_stress_at_unit_pressure_pa
            * travel_m,
        max_depth_m: depth_m,
        shear_rate_s: 0.0,
        shear_strain: 1.0,
        cut_spacing_m: 1e-3,
    }
}

fn projected(canvas: &Canvas, pixel: usize) -> Result<MaterialPayload, MaterialError> {
    let volume = canvas.wet.vol[pixel];
    if volume <= 0.0 {
        return Ok(MaterialPayload::default());
    }
    canvas.wet.lat[pixel]
        .material
        .as_ref()
        .map(|payload| payload.scaled(volume as f64))
        .ok_or(MaterialError::Invalid(
            "knife projection has no chemical payload",
        ))
}

pub(crate) fn support_height(canvas: &Canvas, pixel: usize) -> Result<f32, MaterialError> {
    let source = canvas.native_material_pixel(pixel)?;
    let store = canvas
        .material_store()
        .ok_or(MaterialError::Invalid("missing knife material"))?;
    store.sample(source)?;
    Ok(store.read_column(source)?.state.support_z_um)
}

pub(crate) fn top_height(canvas: &Canvas, pixel: usize) -> Result<f32, MaterialError> {
    let source = canvas.native_material_pixel(pixel)?;
    let store = canvas
        .material_store()
        .ok_or(MaterialError::Invalid("missing knife material"))?;
    let mut height = store.sample(source)?.height_um as f64;
    let payload = projected(canvas, pixel)?;
    let bound = payload.inventory().solvent;
    height += (canvas.wet.solv[pixel] as f64 - bound).max(0.0);
    for fragment in &payload.fragments {
        height += (fragment.pigment + fragment.binder + fragment.solvent)
            / fragment.history.coverage as f64;
    }
    if !height.is_finite() || height.abs() > f32::MAX as f64 {
        return Err(MaterialError::Invalid("knife height overflow"));
    }
    Ok(height as f32)
}

fn fragment_material(fragment: &Fragment) -> Result<constitutive::Material, MaterialError> {
    let mut latent = Latent::from_color(fragment.recipe.color);
    latent.parts = fragment.recipe.parts.clone();
    let formulation = Formulation::new_with_binders(
        latent,
        fragment.recipe.properties,
        fragment.recipe.kinetics,
        fragment.recipe.binders,
    )?;
    constitutive::evaluate(
        &Cell {
            pigment_um: fragment.pigment as f32,
            binder_um: fragment.binder as f32,
            solvent_um: fragment.solvent as f32,
            oxygen: fragment.oxygen as f32,
            oxidation: fragment.oxidation as f32,
            network: fragment.network as f32,
            inhibitor: fragment.inhibitor as f32,
            recipe: 0,
        },
        &formulation,
        &Parameters::default(),
    )
    .map_err(|_| MaterialError::Invalid("invalid knife material response"))
}

fn optics(mut payload: MaterialPayload) -> (f32, Latent, Prop, f32) {
    let volume = payload.volume();
    let mut layers = Vec::new();
    let mut cure = 0.0;
    for fragment in &payload.fragments {
        if fragment.volume <= 0.0 {
            continue;
        }
        let mut latent = Latent::from_color(fragment.recipe.color);
        latent.parts = fragment.recipe.parts.clone();
        layers.push(Layer {
            share: fragment.volume as f32,
            lat: latent,
            hide: fragment.properties(),
        });
        cure += fragment.volume * fragment.network;
    }
    let (_, mut latent, properties) = Latent::from_layers(layers);
    if volume > 0.0 {
        payload.scale(1.0 / volume);
        latent.material = Some(payload);
    }
    (
        volume as f32,
        latent,
        properties,
        if volume > 0.0 {
            (cure / volume) as f32
        } else {
            0.0
        },
    )
}

fn set_projection(canvas: &mut Canvas, pixel: usize, payload: MaterialPayload, free_solvent: f64) {
    let solvent = payload.inventory().solvent + free_solvent;
    let coverage = payload.fragments.last().map_or(1.0, |f| f.history.coverage);
    let (volume, latent, properties, cure) = optics(payload);
    canvas.wet.vol[pixel] = volume;
    canvas.wet.lat[pixel] = latent;
    canvas.wet.hide[pixel] = properties;
    canvas.wet.solv[pixel] = solvent as f32;
    canvas.wet.cover[pixel] = coverage;
    canvas.wet.clock.px[pixel].cure = cure;
    canvas.wet.clock.px[pixel].seen = volume;
    canvas.wet.material_changed[pixel] = 1;
}

pub(crate) fn append(
    canvas: &mut Canvas,
    pixel: usize,
    latent: &Latent,
    volume_coats: f32,
    solvent_coats: f32,
) -> Result<(), MaterialError> {
    let mut payload = projected(canvas, pixel)?;
    let old_bound = payload.inventory().solvent;
    let mut free = (canvas.wet.solv[pixel] as f64 - old_bound).max(0.0);
    if volume_coats > 0.0 {
        let incoming = latent
            .material
            .as_ref()
            .ok_or(MaterialError::Invalid(
                "knife deposit has no chemical payload",
            ))?
            .scaled(volume_coats as f64);
        free += (solvent_coats as f64 * COAT_UM as f64 - incoming.inventory().solvent).max(0.0);
        payload.append(&incoming, 1.0);
    } else {
        free += solvent_coats as f64 * COAT_UM as f64;
    }
    set_projection(canvas, pixel, payload, free);
    Ok(())
}

pub(crate) fn cut(
    canvas: &mut Canvas,
    pixel: usize,
    contact: Contact,
) -> Result<Option<Cut>, MaterialError> {
    let source = canvas.native_material_pixel(pixel)?;
    let projected = projected(canvas, pixel)?;
    let free = (canvas.wet.solv[pixel] as f64 - projected.inventory().solvent).max(0.0);
    let mut candidates = Vec::new();
    if free > 0.0 {
        candidates.push(KnifeCell {
            thickness_m: free * 1e-6,
            material: constitutive::Material::default(),
            damage: 0.0,
            release: None,
        });
    }
    for fragment in projected.fragments.iter().rev() {
        if fragment.volume <= 0.0 {
            return Err(MaterialError::Invalid(
                "knife bound solvent requires a carrier",
            ));
        }
        candidates.push(KnifeCell {
            thickness_m: (fragment.pigment + fragment.binder + fragment.solvent)
                / fragment.history.coverage as f64
                * 1e-6,
            material: fragment_material(fragment)?,
            damage: fragment.history.interface_damage as f64,
            release: None,
        });
    }
    let projected_end = candidates.len();
    let mut retained_inventory = Vec::new();
    {
        let store = canvas
            .material_store()
            .ok_or(MaterialError::Invalid("missing knife material"))?;
        store.sample(source)?;
        let column = store.read_column(source)?;
        for (packet, coverage) in column.packets.iter().zip(column.coverage).rev() {
            let start = (packet.first_cell - column.first_cell) as usize;
            for cell in column.cells[start..start + packet.cell_count as usize]
                .iter()
                .rev()
            {
                if cell.pigment_um == 0.0 && cell.binder_um == 0.0 {
                    return Err(MaterialError::Invalid(
                        "knife bound solvent requires a carrier",
                    ));
                }
                let material = constitutive::evaluate(
                    cell,
                    &column.formulations[cell.recipe as usize],
                    &Parameters::default(),
                )
                .map_err(|_| MaterialError::Invalid("invalid retained knife material"))?;
                candidates.push(KnifeCell {
                    thickness_m: cell.thickness_um() / *coverage as f64 * 1e-6,
                    material,
                    damage: packet.interface_damage as f64,
                    release: None,
                });
                retained_inventory.push(cell.thickness_um());
            }
        }
    }
    let receipt = knife::reach(&candidates, contact)
        .map_err(|_| MaterialError::Invalid("invalid knife contact"))?;
    if receipt.depth_m == 0.0 {
        return Ok(None);
    }
    let mut next = MaterialPayload::default();
    let mut removed = MaterialPayload::default();
    let projected_start = usize::from(free > 0.0);
    for (index, fragment) in projected.fragments.iter().enumerate() {
        let result = receipt.cells[projected_start + projected.fragments.len() - index - 1];
        if result.removed_fraction == 0.0 {
            next.fragments.push(fragment.clone());
            continue;
        }
        let input = MaterialPayload {
            fragments: vec![fragment.clone()],
        };
        let geometry = (fragment.pigment + fragment.binder + fragment.solvent) / fragment.volume;
        let (mut below, mut above) =
            input.split_top_with_geometry(fragment.volume * result.removed_fraction, geometry);
        for slice in below.fragments.iter_mut().chain(above.fragments.iter_mut()) {
            slice.reference_area = fragment.reference_area;
        }
        for fragment in &mut above.fragments {
            fragment.history.interface_damage = result.fragment_damage as f32;
        }
        next.fragments.append(&mut below.fragments);
        removed.fragments.append(&mut above.fragments);
    }
    let lower_amount = receipt.cells[projected_end..]
        .iter()
        .zip(&retained_inventory)
        .map(|(r, amount)| r.removed_fraction * amount)
        .sum::<f64>();
    if lower_amount > 0.0 {
        let mut amount = lower_amount as f32;
        if amount as f64 > lower_amount {
            amount = f32::from_bits(amount.to_bits() - 1);
        }
        if amount > 0.0 {
            let mut lower = canvas.native_knife_take(pixel, amount)?;
            for fragment in &mut lower.fragments {
                if fragment_material(fragment)?.coherence > 0.0 {
                    fragment.history.interface_damage = 1.0;
                }
            }
            lower.fragments.append(&mut removed.fragments);
            removed = lower;
        }
    }
    let free_removed = if free > 0.0 {
        free * receipt.cells[0].removed_fraction
    } else {
        0.0
    };
    let solvent = removed.inventory().solvent + free_removed;
    set_projection(canvas, pixel, next, free - free_removed);
    let (volume_coats, latent, properties, cure) = optics(removed);
    Ok(Some(Cut {
        volume_coats,
        solvent_coats: (solvent / COAT_UM as f64) as f32,
        latent,
        properties,
        cure,
        receipt,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::chemistry::Kinetics;
    use super::super::store::DepositMode;
    use super::super::transfer::{Fragment as Packet, MaterialTransfer, TransferCell};
    use super::*;
    use crate::{Knife, Paint};
    use std::sync::Arc;

    fn canvas() -> Canvas {
        Canvas::new(8, 1.0, [0.8; 3])
            .with_engine(9)
            .with_size_mm(8.0)
            .with_material_revision(1)
    }

    fn packet(area: f64, network: f32, hard: bool) -> Packet {
        let paint = Paint::body([0.2, 0.3, 0.4]);
        let formulation = Arc::new(
            Formulation::new_with_binders(
                paint.latent(),
                paint.prop(),
                Kinetics::default(),
                if hard {
                    [0.0, 0.0, 0.0, 0.0, 1.0]
                } else {
                    [1.0, 0.0, 0.0, 0.0, 0.0]
                },
            )
            .unwrap(),
        );
        let cell = if hard {
            Cell {
                pigment_um: 72.0,
                binder_um: 8.0,
                oxidation: 1.0,
                network: 1.0,
                ..Cell::default()
            }
        } else {
            Cell {
                pigment_um: 160.0,
                binder_um: 340.0,
                oxygen: 0.25,
                oxidation: network,
                network,
                inhibitor: 0.2,
                ..Cell::default()
            }
        };
        let mut packet = Packet::new(vec![TransferCell::from_cell(cell, formulation, area)], 1.0);
        packet.reference_area = area;
        packet.provenance = if hard { 100 } else { 200 };
        packet
    }

    #[test]
    fn native_knife_local_admission_preserves_lower_history_and_work() {
        let mut canvas = canvas();
        let pixel = canvas.f.index(500.0, 500.0);
        canvas
            .edit_material(|store| {
                let area = store.pixel_area();
                let mut hard = packet(area, 1.0, true);
                hard.tension_n_m = [0.1, 0.2, 0.0];
                hard.moment_n = [1e-8, -2e-8, 0.0];
                let mut skin = packet(area, 0.8, false);
                skin.tension_n_m = [-0.4, -0.2, 0.0];
                skin.moment_n = [5e-8, 0.0, 0.0];
                store.deposit(
                    pixel,
                    MaterialTransfer {
                        fragments: vec![hard, skin],
                        free_solvent: 0.0,
                    },
                    DepositMode::Cover,
                )
            })
            .unwrap();
        let before = canvas
            .material_store()
            .unwrap()
            .read_column(pixel)
            .unwrap()
            .to_owned();
        let cut = canvas.with_native_material(|canvas| {
            let local = canvas.f.index(500.0, 500.0);
            assert!(
                cut(canvas, local, blade_contact(0.01, 0.001, 0.001))
                    .unwrap()
                    .is_none()
            );
            let admitted = cut(canvas, local, blade_contact(1.0, 0.001, 0.001))
                .unwrap()
                .unwrap();
            assert!((admitted.receipt.depth_m - 500e-6).abs() < 1e-10);
            assert!(
                (admitted.receipt.spent_work_j_m2 + admitted.receipt.remaining_work_j_m2 - 20.0)
                    .abs()
                    < 1e-12
            );
            admitted
        });
        let after = canvas
            .material_store()
            .unwrap()
            .read_column(pixel)
            .unwrap()
            .to_owned();
        assert_eq!(after.cells, before.cells[..1]);
        assert_eq!(after.packets, before.packets[..1]);
        assert_eq!(after.coverage, before.coverage[..1]);
        let removed = cut
            .latent
            .material
            .as_ref()
            .unwrap()
            .scaled(cut.volume_coats as f64);
        assert_eq!(removed.fragments.len(), 1);
        let fragment = &removed.fragments[0];
        assert_eq!(fragment.network, before.cells[1].network as f64);
        assert_eq!(fragment.oxidation, before.cells[1].oxidation as f64);
        assert_eq!(fragment.history.provenance, 200);
        assert_eq!(fragment.history.tension_n_m, before.packets[1].tension_n_m);
        assert_eq!(fragment.history.moment_n, before.packets[1].moment_n);
        assert_eq!(fragment.history.interface_damage, 1.0);
        assert!((removed.inventory().pigment - 160.0).abs() < 1e-5);
        assert!((removed.inventory().binder - 340.0).abs() < 1e-5);
    }

    #[test]
    fn native_knife_pressure_ramp_does_not_unlock_the_low_pressure_prefix() {
        let mut canvas = canvas();
        canvas
            .edit_material(|store| {
                let area = store.pixel_area();
                for pixel in 0..64 {
                    store.deposit(
                        pixel,
                        MaterialTransfer {
                            fragments: vec![packet(area, 0.8, false)],
                            free_solvent: 0.0,
                        },
                        DepositMode::Cover,
                    )?;
                }
                Ok(())
            })
            .unwrap();
        let low_pixel = canvas.f.index(500.0, 200.0);
        let high_pixel = canvas.f.index(500.0, 800.0);
        let low_before = canvas
            .material_store()
            .unwrap()
            .read_column(low_pixel)
            .unwrap()
            .to_owned();
        let high_before = canvas
            .material_store()
            .unwrap()
            .sample(high_pixel)
            .unwrap()
            .inventory_um;
        let mut knife = Knife::new(500.0);
        canvas.knife(
            &mut knife,
            &[(500.0, 125.0), (500.0, 875.0)],
            (0.02, 1.0),
            Some(0.0),
            false,
            0.0,
        );
        assert_eq!(
            canvas
                .material_store()
                .unwrap()
                .read_column(low_pixel)
                .unwrap()
                .to_owned(),
            low_before
        );
        assert!(
            canvas
                .material_store()
                .unwrap()
                .sample(high_pixel)
                .unwrap()
                .inventory_um
                < high_before
        );
        assert!(knife.fullness() > 0.0);
        assert!(canvas.native_exchange.is_none() && canvas.wet.vol.is_empty());
    }

    #[test]
    fn native_knife_ridge_transfer_keeps_coherent_slices_separate() {
        let mut canvas = canvas();
        let left = canvas.f.index(250.0, 500.0);
        let right = canvas.f.index(750.0, 500.0);
        let destination = canvas.f.index(500.0, 500.0);
        canvas
            .edit_material(|store| {
                let area = store.pixel_area();
                for (pixel, network) in [(left, 0.8), (right, 0.7)] {
                    store.deposit(
                        pixel,
                        MaterialTransfer {
                            fragments: vec![packet(area, network, false)],
                            free_solvent: 0.0,
                        },
                        DepositMode::Cover,
                    )?;
                }
                Ok(())
            })
            .unwrap();
        canvas.with_native_material(|canvas| {
            let destination = canvas.f.index(500.0, 500.0);
            for x in [250.0, 750.0] {
                let local = canvas.f.index(x, 500.0);
                let cut = cut(canvas, local, blade_contact(1.0, 0.001, 0.001))
                    .unwrap()
                    .unwrap();
                append(
                    canvas,
                    destination,
                    &cut.latent,
                    cut.volume_coats,
                    cut.solvent_coats,
                )
                .unwrap();
            }
            let payload = projected(canvas, destination).unwrap();
            assert_eq!(payload.fragments.len(), 2);
            assert_eq!(payload.fragments[0].network, 0.8_f32 as f64);
            assert_eq!(payload.fragments[1].network, 0.7_f32 as f64);
        });
        let column = canvas
            .material_store()
            .unwrap()
            .read_column(destination)
            .unwrap();
        assert_eq!(column.cells.len(), 2);
        assert_eq!(column.cells[0].network, 0.8);
        assert_eq!(column.cells[1].network, 0.7);
    }
}
