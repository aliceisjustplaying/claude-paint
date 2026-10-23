mod canvas;
pub mod checkpoint;
pub mod chemistry;
pub mod constitutive;
pub mod depth;
pub mod exchange;
pub mod knife;
pub(crate) mod native;
pub mod recipes;
pub mod store;
pub mod transfer;
#[cfg(feature = "replay-effects")]
pub mod wire;

use chemistry::{Scratch, observe};
use store::{ClockState, FilmStore, MaterialError};

const INTERVAL_SECONDS: f64 = 600.0;

#[derive(Clone, Debug)]
pub(crate) struct Material {
    pub(crate) store: FilmStore,
    pub(crate) interval_start: Option<FilmStore>,
}

impl Material {
    pub(crate) fn new(store: FilmStore) -> Self {
        Self {
            store,
            interval_start: None,
        }
    }

    pub(crate) fn commit(&mut self) {
        self.interval_start = None;
        let clock = &mut self.store.clock;
        clock.committed_seconds = clock.seconds;
        clock.pending_seconds = clock.seconds;
    }

    pub(crate) fn advance_to_with(
        &mut self,
        seconds: f64,
        mut transport: impl FnMut(&mut FilmStore, f64) -> Result<(), MaterialError>,
    ) -> Result<(), MaterialError> {
        if !seconds.is_finite() || seconds < self.store.clock.seconds {
            return Err(MaterialError::Invalid("invalid material clock target"));
        }
        let mut next = self.clone();
        let mut scratch = Scratch::default();
        while next.store.clock.seconds < seconds {
            let start = next
                .interval_start
                .take()
                .unwrap_or_else(|| next.store.clone());
            let t0 = start.clock.committed_seconds;
            let boundary = t0 + INTERVAL_SECONDS;
            let end = seconds.min(boundary);
            let mut trial = start.clone();
            advance_store(&mut trial, end - t0, &mut scratch)?;
            transport(&mut trial, end - t0)?;
            trial.clock = ClockState {
                seconds: end,
                committed_seconds: if end == boundary { end } else { t0 },
                pending_seconds: if end == boundary { end } else { boundary },
                step: start
                    .clock
                    .step
                    .checked_add(u64::from(end == boundary))
                    .ok_or(MaterialError::Capacity)?,
                seed: start.clock.seed,
            };
            if end < boundary {
                next.interval_start = Some(start);
            }
            next.store = trial;
        }
        *self = next;
        Ok(())
    }
}

fn advance_store(
    store: &mut FilmStore,
    seconds: f64,
    scratch: &mut Scratch,
) -> Result<(), MaterialError> {
    let air = store.environment.oxygen_activity;
    for tile in 0..store.tile_count() {
        if store.tiles[tile]
            .columns
            .iter()
            .all(|column| column.cell_count == 0)
        {
            continue;
        }
        let mut edit = store.edit_tile(tile)?;
        let kinetics: Vec<_> = edit.formulations().iter().map(|f| f.kinetics()).collect();
        for pixel in 0..edit.column_count() {
            if edit.read_column(pixel)?.cells.is_empty() {
                continue;
            }
            edit.with_column_mut(pixel, |column, formulations| {
                depth::advance(column, formulations, &kinetics, seconds, air, scratch, None)
            })?;
        }
    }
    Ok(())
}

pub(crate) fn stage(column: &store::ColumnRef<'_>) -> crate::drying::Stage {
    use crate::drying::Stage;
    let model = column
        .cells
        .last()
        .map(|cell| {
            recipes::observations_for(&column.formulations[cell.recipe as usize].kinetics())
        })
        .unwrap_or_default();
    let mut cells = Vec::with_capacity(column.cells.len());
    for (packet, &coverage) in column.packets.iter().zip(column.coverage) {
        let a = (packet.first_cell - column.first_cell) as usize;
        for cell in &column.cells[a..a + packet.cell_count as usize] {
            let mut cell = *cell;
            cell.pigment_um /= coverage;
            cell.binder_um /= coverage;
            cell.solvent_um /= coverage;
            cells.push(cell);
        }
    }
    if cells.iter().all(|cell| cell.thickness_um() == 0.0) {
        return Stage::Dry;
    }
    let observation = observe(&cells, &model).expect("valid material observation");
    if observation.dry_to_touch {
        Stage::Dry
    } else if observation.set_to_touch {
        Stage::Tacky
    } else if observation.surface_network >= 0.5 * model.set_to_touch_network {
        Stage::Setting
    } else {
        Stage::Open
    }
}

pub trait MechanicalCoupling {
    fn advance(
        &mut self,
        interval_start: &FilmStore,
        end_seconds: f64,
    ) -> Result<FilmStore, MaterialError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Canvas, Paint, Stage};
    use chemistry::{Cell, Kinetics};
    use std::sync::Arc;
    use store::{DepositMode, Formulation};
    use transfer::{Fragment, MaterialTransfer, TransferCell};

    fn canvas() -> Canvas {
        Canvas::new_window_with_material(1, 1.0, [0.8; 3], None, 9, 1)
    }

    fn add(canvas: &mut Canvas, thickness: f32, network: f32, rate: f64) {
        let paint = Paint::body([0.2, 0.3, 0.4]);
        let kinetics = Kinetics {
            network_rate_s: rate,
            ..Kinetics::default()
        };
        let formulation =
            Arc::new(Formulation::new(paint.latent(), paint.prop(), kinetics).unwrap());
        canvas
            .edit_material(|store| {
                let cell = Cell {
                    pigment_um: thickness * 0.5,
                    binder_um: thickness * 0.5,
                    oxygen: 0.4,
                    oxidation: 0.8,
                    network,
                    ..Cell::default()
                };
                let mut fragment = Fragment::new(
                    vec![TransferCell::from_cell(
                        cell,
                        formulation,
                        store.pixel_area(),
                    )],
                    1.0,
                );
                fragment.reference_area = store.pixel_area();
                store.deposit(
                    0,
                    MaterialTransfer {
                        fragments: vec![fragment],
                        free_solvent: 0.0,
                    },
                    DepositMode::Cover,
                )
            })
            .unwrap();
    }

    fn bytes(canvas: &Canvas) -> Vec<u8> {
        let mut bytes = Vec::new();
        canvas
            .material_store()
            .unwrap()
            .write_to(&mut bytes)
            .unwrap();
        bytes
    }

    fn base_network(canvas: &Canvas) -> f32 {
        canvas
            .material_store()
            .unwrap()
            .read_column(0)
            .unwrap()
            .cells[0]
            .network
    }

    #[test]
    fn idle_columns_allocate_no_legacy_material_planes() {
        let mut c = canvas();
        c.wait(3.25);
        assert!(c.wet.vol.is_empty() && c.wet.lat.is_empty() && c.wet.hide.is_empty());
        assert!(c.wet.clock.px.is_empty() && c.wet.solv.is_empty());
        assert_eq!(c.material_store().unwrap().statistics().live_cells, 0);
        let clone = c.clone();
        assert_eq!(clone.material_store().unwrap().statistics().shared_tiles, 1);
    }

    #[test]
    fn touch_dry_surface_retains_and_advances_slow_buried_film() {
        let mut c = canvas();
        add(&mut c, 50.0, 0.0, 1e-8);
        add(&mut c, 12.5, 0.4, 1e-3);
        assert_eq!(c.drying_at(500.0, 500.0), Stage::Dry);
        c.wait(120.0);
        assert!(base_network(&c) > 0.0 && base_network(&c) < 0.01);
        assert_eq!(c.drying_at(500.0, 500.0), Stage::Dry);
        let column = c.material_store().unwrap().read_column(0).unwrap();
        assert_eq!(column.packets.len(), 2);
        assert!(column.cells.len() > 2);
    }

    #[test]
    fn dry_stops_at_surface_touch_without_hardening_the_base() {
        let mut c = canvas();
        add(&mut c, 50.0, 0.0, 1e-8);
        add(&mut c, 12.5, 0.34, 1e-3);
        c.dry();
        assert_eq!(c.drying_at(500.0, 500.0), Stage::Dry);
        assert!(c.clock() > 0.0 && base_network(&c) < 0.01);
        assert!(c.material_store().unwrap().statistics().live_cells > 0);
    }

    #[test]
    fn split_observations_keep_the_canonical_interval() {
        let mut whole = canvas();
        add(&mut whole, 50.0, 0.0, 1e-5);
        let mut split = whole.clone();
        whole.wait(21.75);
        for minutes in [3.25, 7.0, 11.5] {
            split.wait(minutes);
        }
        assert_eq!(bytes(&whole), bytes(&split));
        assert_eq!(
            whole
                .material
                .as_ref()
                .unwrap()
                .interval_start
                .as_ref()
                .unwrap()
                .clock
                .seconds,
            1200.0
        );
    }

    #[test]
    fn explicit_revision_roundtrip_keeps_pending_clock_and_rollback() {
        let mut c = canvas();
        add(&mut c, 50.0, 0.0, 1e-5);
        c.wait(3.25);
        let mut saved = Vec::new();
        c.write_state(&mut saved, "material=1").unwrap();
        assert_eq!(&saved[..8], b"PAINTC22");
        let (mut restored, header) = Canvas::read_state(&mut saved.as_slice()).unwrap();
        assert_eq!(header, "material=1");
        assert_eq!(restored.material_revision(), 1);
        assert!(restored.wet.vol.is_empty());
        c.wait(18.5);
        restored.wait(18.5);
        assert_eq!(bytes(&c), bytes(&restored));
        let before = bytes(&restored);
        let result: Result<(), _> = restored.edit_material(|store| {
            store.with_column_mut(0, |column, _| {
                column.cells[0].network = 0.9;
                Ok(())
            })?;
            Err(MaterialError::Invalid("rollback"))
        });
        assert!(result.is_err());
        assert_eq!(before, bytes(&restored));
        let legacy = Canvas::new_window(1, 1.0, [0.8; 3], None).with_engine(9);
        assert_eq!(legacy.material_revision(), 0);
        let mut old = Vec::new();
        legacy.write_state(&mut old, "").unwrap();
        assert_eq!(&old[..8], b"PAINTC19");
        assert_eq!(
            Canvas::read_state(&mut old.as_slice())
                .unwrap()
                .0
                .material_revision(),
            0
        );
    }

    #[test]
    fn removing_only_binder_preserves_pigment_optical_depth() {
        let mut c = canvas();
        add(&mut c, 25.0, 0.0, 1e-5);
        let before = c.material_look_px(0);
        c.edit_material(|store| {
            store.with_column_mut(0, |column, _| {
                column.cells[0].binder_um *= 0.5;
                Ok(())
            })
        })
        .unwrap();
        assert_eq!(before, c.material_look_px(0));
    }

    #[test]
    fn checkpoint_budget_counts_current_and_pending_stores_together() {
        let mut c = canvas();
        add(&mut c, 50.0, 0.0, 1e-5);
        c.wait(3.25);
        let material = c.material.as_ref().unwrap();
        let start = material.interval_start.as_ref().unwrap();
        let current_counts = material.store.checkpoint_counts().unwrap();
        let start_counts = start.checkpoint_counts().unwrap();
        let current_resident = checkpoint::resident_size(1, current_counts).unwrap();
        let start_resident = checkpoint::resident_size(1, start_counts).unwrap();
        let mut encoded = Vec::new();
        c.write_state(&mut encoded, "").unwrap();
        for budget in [
            checkpoint::LoadBudget {
                max_raw_bytes: u64::MAX,
                max_resident_bytes: current_resident.max(start_resident),
            },
            checkpoint::LoadBudget {
                max_raw_bytes: current_counts.raw_bytes.max(start_counts.raw_bytes),
                max_resident_bytes: u64::MAX,
            },
        ] {
            assert!(
                Canvas::read_state_with_material_budget(&mut encoded.as_slice(), budget).is_err()
            );
        }
        let budget = checkpoint::LoadBudget {
            max_raw_bytes: current_counts.raw_bytes + start_counts.raw_bytes,
            max_resident_bytes: current_resident + start_resident,
        };
        assert!(Canvas::read_state_with_material_budget(&mut encoded.as_slice(), budget).is_ok());
        encoded[40..48].copy_from_slice(&2u64.to_le_bytes());
        assert!(Canvas::read_state(&mut encoded.as_slice()).is_err());
    }
}
