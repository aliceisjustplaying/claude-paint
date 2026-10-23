use super::exchange::{self, MaterialPayload, MechanicalHistory, Recipe, SinkKind};
use super::store::{DepositMode, Formulation, MaterialError, RemovalMode, TakeRequest};
use super::transfer::{Fragment, MaterialTransfer, TransferCell};
use super::{Material, chemistry::ObservationModel};
use crate::wet::{Latent, Layer, Wet};
use crate::{COAT_UM, Canvas};
use std::sync::Arc;

pub(crate) struct NativeExchange {
    pub(crate) before: Material,
    pub(crate) current: u32,
    surf_gen: u64,
    pub(crate) frame: crate::Frame,
    pub(crate) keep: crate::sched::Rect,
    rect: crate::sched::Rect,
    pub(crate) px: Vec<crate::Rgb>,
    pub(crate) height: Vec<f32>,
    pub(crate) film: Vec<f32>,
    pub(crate) gloss: Vec<f32>,
    pub(crate) absorb: Vec<f32>,
    pub(crate) receptive: Option<Vec<f32>>,
    initial_solvent: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::super::chemistry::{Cell, Kinetics};
    use super::*;
    use crate::{Gesture, Held, Paint, Tool, Touch};

    fn paint() -> Paint {
        let mut paint = Paint::body([0.2, 0.3, 0.4]);
        paint.material_recipe = Some(Recipe {
            color: crate::mixer::encode(&paint.color),
            parts: vec![],
            properties: paint.prop(),
            kinetics: Kinetics {
                solvent_evaporation_rate_s: 0.01,
                ..Kinetics::default()
            },
            binders: [1.0, 0.0, 0.0, 0.0, 0.0],
        });
        paint
    }

    fn canvas(width: usize) -> Canvas {
        Canvas::new_window(width, 1.0, [0.8; 3], None)
            .with_engine(9)
            .with_size_mm(16.0)
            .with_material_revision(1)
    }

    fn state(canvas: &Canvas) -> Vec<u8> {
        let mut bytes = Vec::new();
        canvas
            .material_store()
            .unwrap()
            .write_to(&mut bytes)
            .unwrap();
        bytes
    }

    #[test]
    fn native_brush_commits_fragments_and_releases_dense_material() {
        let mut canvas = canvas(32);
        let mut held = Held::new(Tool::hog_flat(100.0), 4).with_engine(9);
        held.load(paint(), 1.0);
        canvas.drag(
            &mut held,
            &Gesture::line((100.0, 500.0), (900.0, 500.0)),
            None,
        );
        assert!(canvas.material_store().unwrap().statistics().live_cells > 0);
        assert!(
            canvas.wet.vol.is_empty() && canvas.wet.lat.is_empty() && canvas.wet.hide.is_empty()
        );
        assert!(canvas.native_exchange.is_none());
        assert_ne!(canvas.seen(), vec![[0.8; 3]; 32 * 32]);
    }

    #[test]
    fn native_work_does_not_expose_a_slow_base_through_a_set_cover() {
        let mut canvas = canvas(8);
        let fresh = paint();
        let formulation =
            Arc::new(Formulation::new(fresh.latent(), fresh.prop(), Kinetics::default()).unwrap());
        let i = canvas.f.index(500.0, 500.0);
        canvas
            .edit_material(|store| {
                let area = store.pixel_area();
                let cells = [
                    Cell {
                        pigment_um: 25.0,
                        binder_um: 25.0,
                        network: 0.01,
                        ..Cell::default()
                    },
                    Cell {
                        pigment_um: 6.25,
                        binder_um: 6.25,
                        network: 0.4,
                        ..Cell::default()
                    },
                ];
                let mut fragment = Fragment::new(
                    cells
                        .into_iter()
                        .map(|cell| TransferCell::from_cell(cell, formulation.clone(), area))
                        .collect(),
                    1.0,
                );
                fragment.reference_area = area;
                store.deposit(
                    i,
                    MaterialTransfer {
                        fragments: vec![fragment],
                        free_solvent: 0.0,
                    },
                    DepositMode::Cover,
                )
            })
            .unwrap();
        let before = canvas
            .material_store()
            .unwrap()
            .read_column(i)
            .unwrap()
            .cells
            .to_vec();
        let mut held = Held::new(Tool::hog_flat(150.0), 4).with_engine(9);
        held.load(paint(), 1.0);
        canvas.touch(&mut held, &Touch::at(500.0, 500.0), None);
        let after = canvas.material_store().unwrap().read_column(i).unwrap();
        assert_eq!(&after.cells[..2], &before);
    }

    #[test]
    fn oversized_native_contact_and_rag_leave_only_authoritative_columns() {
        let mut canvas = canvas(64);
        let mut held = Held::new(Tool::hog_flat(1000.0), 8).with_engine(9);
        held.load(paint().with_thinner(0.2), 2.0);
        canvas.touch(&mut held, &Touch::at(500.0, 500.0).pressure(1.0), None);
        assert!(canvas.material_store().unwrap().statistics().live_cells > 0);
        let mut rag = crate::rag::Rag::new(200.0, 2);
        canvas.rag_blot(&mut rag, 500.0, 500.0, 0.8, 3);
        assert!(canvas.wet.vol.is_empty() && canvas.native_exchange.is_none());
    }

    #[test]
    fn a_failed_native_kernel_restores_the_retained_interval() {
        let mut canvas = canvas(8);
        canvas.wait(3.25);
        let before = state(&canvas);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            canvas.with_native_material(|canvas| {
                canvas.wet.vol[0] = 1.0;
                panic!("failed native kernel");
            })
        }));
        assert!(result.is_err());
        assert_eq!(before, state(&canvas));
        assert!(canvas.wet.vol.is_empty() && canvas.native_exchange.is_none());
    }

    #[test]
    fn native_projection_is_footprint_bounded_and_preserves_untouched_packets() {
        let mut canvas = canvas(64);
        let frame = canvas.f;
        canvas.with_native_material_in(Some((20, 20, 24, 24)), |canvas| {
            assert!(canvas.wet.vol.len() < frame.w * frame.h);
            assert_eq!((canvas.f.full_w, canvas.f.full_h), (frame.full_w, frame.full_h));
            assert!(canvas.f.x0 > 0 && canvas.f.y0 > 0);
        });
        assert_eq!(canvas.f, frame);
        assert!(canvas.wet.vol.is_empty());
        let fresh = paint();
        let formulation = Arc::new(Formulation::new(fresh.latent(), fresh.prop(), Kinetics::default()).unwrap());
        canvas.edit_material(|store| {
            let area = store.pixel_area();
            let mut fragment = Fragment::new([0.01, 0.02].into_iter().map(|network| TransferCell::from_cell(Cell { pigment_um: 2.5, binder_um: 2.5, network, ..Cell::default() }, formulation.clone(), area)).collect(), 0.7);
            fragment.reference_area = area; fragment.tension_n_m = [0.2, 0.1, 0.0]; fragment.moment_n = [1e-8, 2e-8, 0.0];
            store.deposit(0, MaterialTransfer { fragments: vec![fragment], free_solvent: 0.0 }, DepositMode::Cover)
        }).unwrap();
        let before = canvas.material_store().unwrap().read_column(0).unwrap().to_owned();
        canvas.with_native_material(|_| {});
        assert_eq!(before, canvas.material_store().unwrap().read_column(0).unwrap().to_owned());
    }
}

#[cfg(test)]
mod knife_tests {
    use super::*;

    fn canvas() -> Canvas {
        let mut canvas = Canvas::new_window_with_material(32, 1.0, [0.8; 3], None, 9, 1).with_size_mm(50.0);
        canvas.edit_material(|store| store.deposit(16 * 32 + 16, super::super::store::tests::paint(100.0 * store.pixel_area(), 1, 1.0), DepositMode::Cover)).unwrap();
        canvas
    }

    fn bytes(canvas: &Canvas) -> Vec<u8> {
        let mut bytes = Vec::new(); canvas.material_store().unwrap().write_to(&mut bytes).unwrap(); bytes
    }

    #[test]
    fn admitted_native_knife_take_maps_the_window_and_cannot_restore_removed_history() {
        let mut canvas = canvas();
        let pixel = 16 * 32 + 16;
        let before = canvas.material_store().unwrap().surface_height(pixel).unwrap();
        canvas.with_native_material_in(Some((14, 14, 18, 18)), |canvas| {
            let local = (16 - canvas.f.y0) * canvas.f.w + 16 - canvas.f.x0;
            assert_eq!(canvas.native_material_pixel(local).unwrap(), pixel);
            assert!(canvas.native_material_pixel(canvas.f.w * canvas.f.h).is_err());
            let removed = canvas.native_knife_take(local, 12.0).unwrap();
            let inventory = removed.inventory();
            assert!((inventory.pigment + inventory.binder - 12.0).abs() < 1e-4);
            assert!((canvas.height[local] - (before - 12.0)).abs() < 1e-4);
            assert_eq!(canvas.wet.material_changed[local], 1);
        });
        assert!((canvas.material_store().unwrap().sample(pixel).unwrap().inventory_um - 88.0).abs() < 1e-4);
        assert!(canvas.wet.vol.is_empty() && canvas.native_exchange.is_none());
    }

    #[test]
    fn failed_native_knife_take_restores_the_retained_base_and_history() {
        let mut canvas = canvas();
        let before = bytes(&canvas);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            canvas.with_native_material_in(Some((14, 14, 18, 18)), |canvas| {
                let local = (16 - canvas.f.y0) * canvas.f.w + 16 - canvas.f.x0;
                canvas.native_knife_take(local, 12.0).unwrap();
                panic!("failed admitted knife kernel");
            });
        }));
        assert!(result.is_err());
        assert_eq!(bytes(&canvas), before);
        assert!(canvas.wet.vol.is_empty() && canvas.native_exchange.is_none());
    }
}

fn payload(transfer: MaterialTransfer, area: f64) -> MaterialPayload {
    let mut fragments = Vec::new();
    for packet in transfer.fragments {
        let height = packet.cells.iter().map(TransferCell::volume).sum::<f64>()
            / area
            / packet.coverage as f64
            * 1e-6;
        let mut bottom = -0.5 * height;
        for cell in packet.cells {
            let h = cell.volume() / area / packet.coverage as f64 * 1e-6;
            let top = bottom + h;
            let fraction = h / height;
            let history = MechanicalHistory {
                tension_n_m: std::array::from_fn(|k| {
                    (packet.tension_n_m[k] as f64 * fraction
                        + 6.0 * packet.moment_n[k] as f64 * (top * top - bottom * bottom)
                            / height.powi(3)) as f32
                }),
                moment_n: packet
                    .moment_n
                    .map(|m| (m as f64 * fraction.powi(3)) as f32),
                interface_damage: packet.interface_damage,
                provenance: packet.provenance,
                coverage: packet.coverage,
            };
            fragments.push(exchange::Fragment {
                reference_area: packet.reference_area,
                volume: (cell.pigment + cell.binder) / area / COAT_UM as f64,
                pigment: cell.pigment / area,
                binder: cell.binder / area,
                solvent: cell.solvent / area,
                oxygen: cell.oxygen as f64,
                oxidation: cell.oxidation as f64,
                network: cell.network as f64,
                inhibitor: cell.inhibitor as f64,
                recipe: Recipe {
                    binders: cell.formulation.binders(),
                    color: cell.formulation.latent().color,
                    parts: cell.formulation.latent().parts.clone(),
                    properties: cell.formulation.prop(),
                    kinetics: cell.formulation.kinetics(),
                },
                history,
            });
            bottom = top;
        }
    }
    MaterialPayload { fragments }
}

fn transfer(payload: MaterialPayload, area: f64) -> Result<MaterialTransfer, MaterialError> {
    let mut fragments = Vec::new();
    for fragment in payload.fragments {
        if fragment.pigment + fragment.binder + fragment.solvent == 0.0 {
            continue;
        }
        let mut latent = Latent::from_color(fragment.recipe.color);
        latent.parts = fragment.recipe.parts;
        let formulation = Arc::new(Formulation::new_with_binders(
            latent,
            fragment.recipe.properties,
            fragment.recipe.kinetics,
            fragment.recipe.binders,
        )?);
        fragments.push(Fragment {
            cells: vec![TransferCell {
                pigment: fragment.pigment * area,
                binder: fragment.binder * area,
                solvent: fragment.solvent * area,
                oxygen: fragment.oxygen as f32,
                oxidation: fragment.oxidation as f32,
                network: fragment.network as f32,
                inhibitor: fragment.inhibitor as f32,
                formulation,
            }],
            tension_n_m: fragment.history.tension_n_m,
            moment_n: fragment.history.moment_n,
            reference_area: fragment.reference_area,
            interface_damage: fragment.history.interface_damage,
            provenance: fragment.history.provenance,
            coverage: fragment.history.coverage,
        });
    }
    Ok(MaterialTransfer {
        fragments,
        free_solvent: 0.0,
    })
}

impl Canvas {
    pub(crate) fn native_material_pixel(&self, local: usize) -> Result<usize, MaterialError> {
        let exchange = self.native_exchange.as_ref().ok_or(MaterialError::Invalid("no active native material exchange"))?;
        if local >= self.f.w * self.f.h { return Err(MaterialError::OutOfBounds); }
        Ok((exchange.rect.1 + local / self.f.w) * exchange.frame.w + exchange.rect.0 + local % self.f.w)
    }

    pub(crate) fn native_knife_take(&mut self, local: usize, amount_um: f32) -> Result<MaterialPayload, MaterialError> {
        let pixel = self.native_material_pixel(local)?;
        if !amount_um.is_finite() || amount_um < 0.0 { return Err(MaterialError::Invalid("invalid admitted knife removal")); }
        if amount_um == 0.0 { return Ok(MaterialPayload::default()); }
        let material = self.material.as_mut().ok_or(MaterialError::Invalid("missing material backend"))?;
        if material.store.read_column(pixel)?.inert_coatings().next().is_some() { return Err(MaterialError::Unsupported("coherent knife contact with an inert coating")); }
        let area = material.store.pixel_area();
        let removed = material.store.take_top(pixel, TakeRequest { amount_um, mode: RemovalMode::Coherent })?;
        let height = material.store.surface_height(pixel)?;
        let removed = payload(removed, area);
        if removed.fragments.is_empty() { return Ok(removed); }
        self.height[local] = height;
        self.px[local] = self.material_look_px(pixel);
        self.film[local] = self.material_film_at(pixel);
        self.wet.material_changed[local] = 1;
        self.surf_gen = self.surf_gen.wrapping_add(1);
        self.base = None;
        self.e7_contact = Default::default();
        Ok(removed)
    }

    pub(crate) fn material_rect(
        &self,
        points: &[(f32, f32)],
        padding: f32,
    ) -> Option<crate::sched::Rect> {
        if points.is_empty() || points.iter().any(|p| !p.0.is_finite() || !p.1.is_finite()) {
            return None;
        }
        let (mut x0, mut y0, mut x1, mut y1) = (
            f32::INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NEG_INFINITY,
        );
        for &(x, y) in points {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        let f = self.f;
        Some((
            ((x0 - padding) * f.scale).floor().max(0.0) as usize,
            ((y0 - padding) * f.scale).floor().max(0.0) as usize,
            (((x1 + padding) * f.scale).ceil().max(0.0) as usize).min(f.full_w),
            (((y1 + padding) * f.scale).ceil().max(0.0) as usize).min(f.full_h),
        ))
    }

    pub(crate) fn advance_material_clock(&mut self, seconds: f64) -> Result<(), MaterialError> {
        assert!(
            self.native_exchange.is_none(),
            "cannot advance time inside a native material kernel"
        );
        let before = self
            .material
            .take()
            .ok_or(MaterialError::Invalid("missing material backend"))?;
        let mut next = before.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            next.advance_to_with(seconds, |store, dt| {
                if store.statistics().solvent == 0.0 {
                    return Ok(());
                }
                self.material = Some(Material::new(store.clone()));
                self.with_native_material(|canvas| {
                    let mut solvent = Vec::new();
                    for i in 0..canvas.f.w * canvas.f.h {
                        let bound = canvas.wet.lat[i]
                            .material
                            .as_ref()
                            .map_or(0.0, |m| m.inventory().solvent * canvas.wet.vol[i] as f64);
                        let free = (canvas.wet.solv[i] as f64 - bound).max(0.0);
                        let mut remaining = (free
                            * (-(dt / 60.0)
                                / crate::thinner::evaporation_tau_min(canvas.wet.vol[i] * COAT_UM))
                            .exp()) as f32;
                        let floor = crate::thinner::SOLVENT_FLOOR.max(
                            1e-5 * (canvas.wet.vol[i] * COAT_UM + crate::thinner::THIN_FILM_UM),
                        );
                        if remaining < floor {
                            remaining = 0.0;
                        }
                        canvas.wet.solv[i] = bound as f32 + remaining;
                        if canvas.wet.solv[i] > 0.0 {
                            solvent.push(i);
                        }
                    }
                    canvas.spread((dt / 60.0) as f32, &mut solvent);
                });
                *store = self.material.take().unwrap().store;
                Ok(())
            })
        }));
        match result {
            Ok(Ok(())) => {
                self.material = Some(next);
                self.release_material_exchange(false);
                Ok(())
            }
            Ok(Err(error)) => {
                self.material = Some(before);
                self.release_material_exchange(false);
                Err(error)
            }
            Err(error) => {
                self.material = Some(before);
                self.release_material_exchange(false);
                std::panic::resume_unwind(error)
            }
        }
    }

    pub(crate) fn material_exchange_needed(&self) -> bool {
        self.material.is_some() && self.native_exchange.is_none()
    }

    pub(crate) fn with_native_material<R>(&mut self, work: impl FnOnce(&mut Self) -> R) -> R {
        self.with_native_material_in(None, work)
    }

    pub(crate) fn with_native_material_in<R>(
        &mut self,
        bounds: Option<crate::sched::Rect>,
        work: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if !self.material_exchange_needed() {
            return work(self);
        }
        self.begin_material_exchange(bounds)
            .expect("material exchange projection");
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work(self)));
        match result {
            Ok(result) => {
                if let Err(error) = self.finish_material_exchange() {
                    self.abort_material_exchange();
                    panic!("material exchange commit: {error}");
                }
                result
            }
            Err(error) => {
                self.abort_material_exchange();
                std::panic::resume_unwind(error)
            }
        }
    }

    fn begin_material_exchange(
        &mut self,
        bounds: Option<crate::sched::Rect>,
    ) -> Result<(), MaterialError> {
        let before = self
            .material
            .clone()
            .ok_or(MaterialError::Invalid("missing material backend"))?;
        let mut material = before.clone();
        material.commit();
        let original = self.f;
        let mut rect = bounds
            .and_then(|bounds| original.clip(bounds))
            .unwrap_or((0, 0, original.w, original.h));
        let r = ((1.5 / self.px_mm()).round() as usize).max(1);
        let halo = (3 * r).div_ceil(2).max(1) + (r / 3).max(1);
        rect = (
            rect.0.saturating_sub(halo),
            rect.1.saturating_sub(halo),
            (rect.2 + halo).min(original.w),
            (rect.3 + halo).min(original.h),
        );
        let width = rect.2 - rect.0;
        let height = rect.3 - rect.1;
        let n = width * height;
        let source = |i: usize| (rect.1 + i / width) * original.w + rect.0 + i % width;
        for i in 0..n {
            if material.store.read_column(source(i))?.inert_coatings().next().is_some() { return Err(MaterialError::Unsupported("native contact with an inert coating")); }
        }
        let mut wet = Wet::new(n);
        wet.current = self.wet.current;
        wet.clock.now = self.clock();
        wet.clock.px = vec![crate::drying::Px::FRESH; n];
        wet.material_sinks.resize_with(n, Vec::new);
        wet.material_changed.resize(n, 0);
        wet.ensure_solvent();
        let area = material.store.pixel_area();
        for i in 0..n {
            let index = source(i);
            let column = material.store.read_column(index)?;
            let limit = column
                .cells
                .last()
                .map(|cell| {
                    super::recipes::observations_for(
                        &column.formulations[cell.recipe as usize].kinetics(),
                    )
                    .set_to_touch_network as f32
                })
                .unwrap_or(ObservationModel::default().set_to_touch_network as f32);
            wet.stroke[i] = column.state.stroke;
            wet.touched[i] = column.state.touched;
            wet.laid_id[i] = column.state.laid_id;
            wet.laid[i] = column.state.laid;
            wet.floor[i] = column.state.floor;
            wet.solv[i] = column.state.free_solvent_um;
            material.store.with_column_mut(index, |column, _| {
                column.state.free_solvent_um = 0.0;
                Ok(())
            })?;
            let amount = material.store.sample_with_limit(index, limit)?.mobile_um;
            if amount <= 0.0 {
                continue;
            }
            let removed = material.store.take_top(
                index,
                TakeRequest {
                    amount_um: amount,
                    mode: RemovalMode::Mobile { max_network: limit },
                },
            )?;
            let mut material_payload = payload(removed, area);
            let coverage = material_payload
                .fragments
                .last()
                .map_or(1.0, |f| f.history.coverage);
            let volume = material_payload.volume() as f32;
            let mut layers = Vec::new();
            let mut cure = 0.0;
            for fragment in &material_payload.fragments {
                if fragment.volume == 0.0 {
                    continue;
                }
                let mut latent = Latent::from_color(fragment.recipe.color);
                latent.parts = fragment.recipe.parts.clone();
                layers.push(Layer {
                    share: fragment.volume as f32,
                    lat: latent,
                    hide: fragment.properties(),
                });
                wet.solv[i] += fragment.solvent as f32;
                cure +=
                    fragment.volume as f32 * fragment.network as f32 / limit * crate::drying::GEL;
            }
            if volume > 0.0 {
                let (_, mut latent, prop) = Latent::from_layers(layers);
                material_payload.scale(1.0 / volume as f64);
                latent.material = Some(material_payload);
                wet.vol[i] = volume;
                wet.lat[i] = latent;
                wet.hide[i] = prop;
                wet.cover[i] = coverage;
                wet.clock.px[i].cure = cure / volume;
                wet.clock.px[i].seen = volume;
                wet.touch(i % width, i / width, i % width + 1, i / width + 1);
            }
        }
        self.material = Some(material);
        let mut px = Vec::with_capacity(n);
        let mut surface = Vec::with_capacity(n);
        let mut film = Vec::with_capacity(n);
        let mut gloss = Vec::with_capacity(n);
        let mut absorb = Vec::with_capacity(n);
        let mut receptive = self.receptive.as_ref().map(|_| Vec::with_capacity(n));
        for i in 0..n {
            let index = source(i);
            let column = self.material_store().unwrap().read_column(index)?;
            px.push(self.material_look_px(index));
            surface.push(self.material_store().unwrap().sample(index)?.height_um);
            film.push(column.state.film);
            gloss.push(column.state.gloss);
            absorb.push(column.state.absorb);
            if let Some(map) = &mut receptive {
                map.push(self.receptive.as_ref().unwrap()[index]);
            }
        }
        self.native_exchange = Some(NativeExchange {
            before,
            current: self.wet.current,
            surf_gen: self.surf_gen,
            frame: original,
            keep: self.keep,
            rect,
            px: std::mem::replace(&mut self.px, px),
            height: std::mem::replace(&mut self.height, surface),
            film: std::mem::replace(&mut self.film, film),
            gloss: std::mem::replace(&mut self.gloss, gloss),
            absorb: std::mem::replace(&mut self.absorb, absorb),
            receptive: std::mem::replace(&mut self.receptive, receptive),
            initial_solvent: wet.solv.clone(),
        });
        self.f = crate::Frame {
            w: width,
            h: height,
            x0: original.x0 + rect.0,
            y0: original.y0 + rect.1,
            ..original
        };
        self.keep = (0, 0, width, height);
        self.wet = wet;
        self.surf_gen += 1;
        self.base = None;
        self.e7_contact = Default::default();
        Ok(())
    }

    fn finish_material_exchange(&mut self) -> Result<(), MaterialError> {
        let mut material = self
            .material
            .clone()
            .ok_or(MaterialError::Invalid("missing material backend"))?;
        let area = material.store.pixel_area();
        let exchange = self.native_exchange.as_ref().expect("native exchange");
        let source = |i: usize| {
            (exchange.rect.1 + i / self.f.w) * exchange.frame.w + exchange.rect.0 + i % self.f.w
        };
        for i in 0..self.f.w * self.f.h {
            if self.wet.material_changed[i] == 0
                && self.wet.material_sinks[i].is_empty()
                && self.wet.solv[i] == exchange.initial_solvent[i]
            {
                let mut original = exchange.before.store.read_column(source(i))?.to_owned();
                original.state.stroke = self.wet.stroke[i];
                original.state.touched = self.wet.touched[i];
                original.state.laid_id = self.wet.laid_id[i];
                original.state.laid = self.wet.laid[i];
                original.state.floor = self.wet.floor[i];
                original.state.absorb = self.absorb[i];
                material.store.with_column_mut(source(i), |column, _| {
                    *column = original;
                    Ok(())
                })?;
                continue;
            }
            let volume = self.wet.vol[i];
            let mut material_payload = self.wet.lat[i].material.clone().unwrap_or_default();
            if volume > 1e-8 && material_payload.fragments.is_empty() {
                return Err(MaterialError::Invalid(
                    "native paint has no explicit material formulation",
                ));
            }
            material_payload.scale(volume as f64);
            let bound_solvent = material_payload.inventory().solvent;
            material.store.deposit(
                source(i),
                transfer(material_payload, area)?,
                DepositMode::Cover,
            )?;
            material.store.with_column_mut(source(i), |column, _| {
                column.state.free_solvent_um =
                    (self.wet.solv[i] as f64 - bound_solvent).max(0.0) as f32;
                column.state.stroke = self.wet.stroke[i];
                column.state.touched = self.wet.touched[i];
                column.state.laid_id = self.wet.laid_id[i];
                column.state.laid = self.wet.laid[i];
                column.state.floor = self.wet.floor[i];
                column.state.absorb = self.absorb[i];
                for sink in &self.wet.material_sinks[i] {
                    if sink.kind == SinkKind::Ground {
                        column.state.ground_oil += sink.material.inventory().binder as f32;
                    }
                }
                Ok(())
            })?;
        }
        self.material = Some(material);
        self.release_material_exchange(false);
        Ok(())
    }

    fn abort_material_exchange(&mut self) {
        self.release_material_exchange(true);
    }

    fn release_material_exchange(&mut self, rollback: bool) {
        let rect = if let Some(exchange) = self.native_exchange.take() {
            if rollback {
                self.material = Some(exchange.before);
                self.wet.current = exchange.current;
            }
            self.f = exchange.frame;
            self.keep = exchange.keep;
            self.px = exchange.px;
            self.height = exchange.height;
            self.film = exchange.film;
            self.gloss = exchange.gloss;
            self.absorb = exchange.absorb;
            self.receptive = exchange.receptive;
            self.surf_gen = exchange.surf_gen + u64::from(!rollback);
            exchange.rect
        } else {
            (0, 0, self.f.w, self.f.h)
        };
        let current = self.wet.current;
        self.wet = Wet::new(0);
        self.wet.current = current;
        self.wet.clock.now = self.clock();
        for y in rect.1..rect.3 {
            for x in rect.0..rect.2 {
                let i = y * self.f.w + x;
                let state = self
                    .material_store()
                    .unwrap()
                    .read_column(i)
                    .expect("material pixel")
                    .state
                    .clone();
                self.px[i] = state.color;
                self.height[i] = state.support_z_um;
                self.film[i] = state.film;
                self.gloss[i] = state.gloss;
                self.absorb[i] = state.absorb;
            }
        }
        self.absorb_any = self.absorb.iter().any(|&a| a > 0.0);
        self.base = None;
        self.e7_contact = Default::default();
    }
}
