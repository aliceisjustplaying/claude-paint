use super::store::{InertCoating, MaterialError};
use crate::mixer::Backing;
use crate::{Canvas, Pigment};

pub(crate) fn over(mut backing: Backing, coating: &InertCoating) -> Backing {
    let pigment = Pigment {
        k: coating.k,
        s: coating.s,
    };
    let color = pigment.over(backing.color, coating.optical_coats);
    let projected = crate::spectral::to_rgb(&backing.reflectance);
    backing.residual = std::array::from_fn(|i| color[i] - projected[i]);
    backing.color = color;
    backing
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oil::checkpoint::{LoadBudget, resident_size};
    use crate::oil::store::tests::paint;
    use crate::oil::store::{DepositMode, FilmStore, RemovalMode, TakeRequest};
    use crate::oil::transfer::MaterialTransfer;

    fn canvas() -> Canvas {
        let mut canvas = Canvas::new_window_with_material(1, 1.0, [0.8; 3], None, 9, 1);
        canvas
            .edit_material(|store| {
                store.deposit(
                    0,
                    paint(20.0 * store.pixel_area(), 1, 1.0),
                    DepositMode::Cover,
                )
            })
            .unwrap();
        canvas
    }

    fn bytes(store: &FilmStore) -> Vec<u8> {
        let mut bytes = Vec::new();
        store.write_to(&mut bytes).unwrap();
        bytes
    }

    fn coating() -> Pigment {
        Pigment {
            k: [0.2, 0.7, 1.4],
            s: [0.03, 0.05, 0.07],
        }
    }

    #[test]
    fn inert_glaze_preserves_exact_coefficients_geometry_and_buried_history() {
        let mut canvas = canvas();
        let before = canvas.material_store().unwrap().clone();
        assert_eq!(&bytes(&before)[..8], b"OILCOL02");
        let color = canvas.material_look_px(0);
        let height = before.surface_height(0).unwrap();
        let pigment = coating();
        canvas.try_glaze(&pigment, None, |_, _| 0.7).unwrap();
        let store = canvas.material_store().unwrap();
        let column = store.read_column(0).unwrap();
        let inert = column.inert_coatings().next().unwrap();
        assert_eq!(inert.k, pigment.k);
        assert_eq!(inert.s, pigment.s);
        assert_eq!(inert.before_packet, 1);
        assert!((inert.optical_coats - 0.7).abs() < 1e-6);
        assert!(inert.thickness_um > 0.0);
        assert!((store.surface_height(0).unwrap() - height - inert.thickness_um).abs() < 1e-5);
        assert_eq!(column.cells, before.read_column(0).unwrap().cells);
        assert_eq!(column.packets, before.read_column(0).unwrap().packets);
        assert_eq!(store.statistics().pigment, before.statistics().pigment);
        assert_eq!(store.statistics().binder, before.statistics().binder);
        for (actual, expected) in canvas
            .material_look_px(0)
            .into_iter()
            .zip(pigment.over(color, 0.7))
        {
            assert!((actual - expected).abs() < 1e-6);
        }
        assert_eq!(before.read_column(0).unwrap().inert_coatings().count(), 0);
    }

    #[test]
    fn inert_coating_stays_below_later_cover_deposits() {
        let mut canvas = canvas();
        canvas.try_glaze(&coating(), None, |_, _| 0.7).unwrap();
        canvas
            .edit_material(|store| {
                let mut cover = paint(10000.0 * store.pixel_area(), 1, 1.0);
                cover.fragments[0].coverage = 1.0;
                store.deposit(
                    0,
                    cover,
                    DepositMode::Cover,
                )
            })
            .unwrap();
        let column = canvas.material_store().unwrap().read_column(0).unwrap();
        assert_eq!(column.packets.len(), 2);
        assert_eq!(column.inert_coatings().next().unwrap().before_packet, 1);
        let coated = canvas.material_look_px(0);
        let mut uncoated = canvas.clone();
        uncoated
            .edit_material(|store| {
                store.with_column_mut(0, |column, _| {
                    column.coatings.clear();
                    Ok(())
                })
            })
            .unwrap();
        for (actual, expected) in coated.into_iter().zip(uncoated.material_look_px(0)) {
            assert!((actual - expected).abs() < 1e-5);
        }
    }

    #[test]
    fn inert_checkpoint_roundtrip_and_aggregate_bounds_include_sparse_entries() {
        let mut canvas = canvas();
        canvas.try_glaze(&coating(), None, |_, _| 0.7).unwrap();
        let store = canvas.material_store().unwrap();
        let encoded = bytes(store);
        assert_eq!(&encoded[..8], b"OILCOL03");
        let counts = store.checkpoint_counts().unwrap();
        assert_eq!(counts.coatings, 1);
        assert_eq!(counts.raw_bytes, encoded.len() as u64);
        let resident = resident_size(1, counts).unwrap();
        let budget = LoadBudget {
            max_raw_bytes: counts.raw_bytes,
            max_resident_bytes: resident,
        };
        let restored = FilmStore::read_from(&mut encoded.as_slice(), budget).unwrap();
        assert_eq!(bytes(&restored), encoded);
        assert_eq!(
            restored.read_column(0).unwrap().to_owned(),
            store.read_column(0).unwrap().to_owned()
        );
        assert!(
            FilmStore::read_from(
                &mut encoded.as_slice(),
                LoadBudget {
                    max_resident_bytes: resident - 1,
                    ..budget
                }
            )
            .is_err()
        );
        assert!(
            FilmStore::read_from(
                &mut encoded.as_slice(),
                LoadBudget {
                    max_raw_bytes: counts.raw_bytes - 1,
                    ..budget
                }
            )
            .is_err()
        );
        let mut unknown = encoded.clone();
        let tag = unknown.len() - 40;
        unknown[tag..tag + 4].copy_from_slice(&2u32.to_le_bytes());
        assert!(FilmStore::read_from(&mut unknown.as_slice(), budget).is_err());
        let mut overflow = encoded;
        overflow[128..136].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(FilmStore::read_from(&mut overflow.as_slice(), budget).is_err());
    }

    #[test]
    fn inert_coated_contact_transport_and_clock_fail_without_mutating_history() {
        let mut canvas = canvas();
        canvas.try_glaze(&coating(), None, |_, _| 0.7).unwrap();
        let before = bytes(canvas.material_store().unwrap());
        assert!(matches!(
            canvas.advance_material_clock(60.0),
            Err(MaterialError::Unsupported(_))
        ));
        assert_eq!(bytes(canvas.material_store().unwrap()), before);
        let store = &mut canvas.material.as_mut().unwrap().store;
        assert!(matches!(
            store.sample(0),
            Err(MaterialError::Unsupported(_))
        ));
        assert!(matches!(
            store.take_top(
                0,
                TakeRequest {
                    amount_um: 1.0,
                    mode: RemovalMode::Coherent
                }
            ),
            Err(MaterialError::Unsupported(_))
        ));
        assert!(matches!(
            store.take_solvent(0, 1.0),
            Err(MaterialError::Unsupported(_))
        ));
        assert!(matches!(
            store.deposit(
                0,
                MaterialTransfer {
                    fragments: vec![],
                    free_solvent: 1.0
                },
                DepositMode::Cover
            ),
            Err(MaterialError::Unsupported(_))
        ));
        assert_eq!(bytes(store), before);
    }

    #[test]
    fn preexisting_hand_time_is_settled_before_the_inert_coating_is_appended() {
        let mut canvas = canvas();
        canvas.set_hand_time(Some(0.25));
        canvas.tally_mut().glaze(0.01);
        assert!(canvas.hand_owed_secs() > 0.0);
        canvas.try_glaze(&coating(), None, |_, _| 0.7).unwrap();
        assert!(canvas.clock() > 0.0);
        assert_eq!(canvas.hand_owed_secs(), 0.0);
        assert_eq!(
            canvas
                .material_store()
                .unwrap()
                .read_column(0)
                .unwrap()
                .inert_coatings()
                .count(),
            1
        );
        let before = bytes(canvas.material_store().unwrap());
        assert!(matches!(
            canvas.try_wait(1.0),
            Err(MaterialError::Unsupported(_))
        ));
        assert_eq!(bytes(canvas.material_store().unwrap()), before);
    }
}

impl Canvas {
    pub(crate) fn material_glaze(
        &mut self,
        pigment: &Pigment,
        coats: &[f32],
        add: &[f32],
    ) -> Result<(), MaterialError> {
        if pigment
            .k
            .iter()
            .chain(&pigment.s)
            .chain(coats)
            .chain(add)
            .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(MaterialError::Invalid(
                "invalid inert coating coefficients or thickness",
            ));
        }
        if add.iter().all(|value| *value <= 0.0) {
            return Ok(());
        }
        if self.hand_owed_secs() > 0.0 {
            self.material_store().unwrap().check_clock_advance()?;
            self.clock_hand_min();
        }
        let store = self.material_store().expect("material backend");
        for (i, &amount) in add.iter().enumerate() {
            if amount == 0.0 {
                continue;
            }
            let column = store.read_column(i)?;
            if super::stage(&column) != crate::Stage::Dry || column.state.free_solvent_um > 0.0 {
                return Err(MaterialError::Unsupported(
                    "inert glaze requires a touch-dry substrate without free solvent",
                ));
            }
        }
        let surface = self.wet_surface();
        let support = std::mem::replace(&mut self.height, surface);
        let settled =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.settle_film(add, 0.05)));
        self.height = support;
        let settled = match settled {
            Ok(settled) => settled,
            Err(error) => std::panic::resume_unwind(error),
        };
        self.edit_material(|store| {
            for i in 0..settled.len() {
                if add[i] <= 0.0 || settled[i] <= 0.0 {
                    continue;
                }
                store.append_inert_coating(
                    i,
                    InertCoating {
                        before_packet: 0,
                        k: pigment.k,
                        s: pigment.s,
                        optical_coats: coats[i] * settled[i] / add[i],
                        thickness_um: settled[i],
                    },
                )?;
            }
            Ok(())
        })
    }
}
