use super::Material;
use super::store::{FilmStore, MaterialError};
use crate::Canvas;
use crate::color::Rgb;
use crate::mixer::Backing;

impl Canvas {
    pub(crate) fn material_drawing_needed(&self) -> bool {
        self.material_exchange_needed() && !self.drawing_projection
    }

    pub(crate) fn with_material_drawing<R>(&mut self, work: impl FnOnce(&mut Self) -> R) -> R {
        if !self.material_drawing_needed() {
            return work(self);
        }
        assert!(
            self.material_store()
                .unwrap()
                .tiles
                .iter()
                .all(|tile| tile.coatings.is_empty()),
            "unsupported material operation: drawing contact with an inert coating"
        );
        let projected_height = self.wet_surface();
        let baseline = projected_height.clone();
        let color: Vec<_> = (0..self.f.w * self.f.h)
            .map(|i| self.material_look_px(i))
            .collect();
        let film: Vec<_> = (0..self.f.w * self.f.h)
            .map(|i| self.material_film_at(i))
            .collect();
        let px = std::mem::replace(&mut self.px, color);
        let height = std::mem::replace(&mut self.height, projected_height);
        let film = std::mem::replace(&mut self.film, film);
        self.wet.vol = self.material_wet_coats();
        self.drawing_projection = true;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work(self)));
        self.drawing_projection = false;
        let changed_height = std::mem::replace(&mut self.height, height);
        self.px = px;
        self.film = film;
        self.wet.vol.clear();
        self.wet.vol.shrink_to_fit();
        if result.is_ok() && changed_height != baseline {
            self.edit_material(|store| {
                for i in 0..changed_height.len() {
                    let delta = changed_height[i] - baseline[i];
                    if delta != 0.0 {
                        store.with_column_mut(i, |column, _| {
                            column.state.support_z_um += delta;
                            Ok(())
                        })?;
                    }
                }
                Ok(())
            })
            .expect("drawing support relief");
        }
        match result {
            Ok(result) => result,
            Err(error) => std::panic::resume_unwind(error),
        }
    }

    pub(crate) fn material_film_at(&self, i: usize) -> f32 {
        let column = self
            .material_store()
            .expect("material backend")
            .read_column(i)
            .expect("material pixel");
        column.state.film
            + column
                .inert_coatings()
                .map(|coating| coating.thickness_um / crate::COAT_UM)
                .sum::<f32>()
            + column
                .cells
                .iter()
                .map(|cell| {
                    let prop = column.formulations[cell.recipe as usize].prop();
                    let fraction = (1.0 - prop[8]) / (1.0 - prop[8] + prop[8] * prop[4]);
                    if fraction > 0.0 {
                        cell.pigment_um / fraction / crate::COAT_UM
                    } else {
                        cell.binder_um / crate::COAT_UM
                    }
                })
                .sum::<f32>()
    }

    pub(crate) fn sync_material_support(&mut self) {
        let Some(mut material) = self.material.clone() else {
            return;
        };
        material.commit();
        material.store.pixel_area = self.px_mm() as f64 * self.px_mm() as f64;
        for i in 0..self.f.w * self.f.h {
            material
                .store
                .with_column_mut(i, |column, _| {
                    let backing = self
                        .backing
                        .get(i)
                        .copied()
                        .unwrap_or(Backing {
                            reflectance: column.state.reflectance,
                            residual: column.state.residual,
                            color: column.state.color,
                        })
                        .under(self.px[i]);
                    column.state.reflectance = backing.reflectance;
                    column.state.residual = backing.residual;
                    column.state.color = backing.color;
                    column.state.support_z_um = self.height[i];
                    column.state.film = self.film[i];
                    column.state.gloss = self.gloss[i];
                    column.state.absorb = self.absorb[i];
                    Ok(())
                })
                .expect("valid material support");
        }
        self.material = Some(material);
        self.backing.clear();
    }

    pub(crate) fn initialize_material(&mut self) {
        let mut store = FilmStore::new(
            self.f.w as u32,
            self.f.h as u32,
            self.px_mm() as f64 * self.px_mm() as f64,
        )
        .expect("valid material frame");
        store.clock.seconds = self.wet.clock.now * 60.0;
        store.clock.committed_seconds = store.clock.seconds;
        store.clock.pending_seconds = store.clock.seconds;
        for i in 0..self.f.w * self.f.h {
            let backing = self
                .backing
                .get(i)
                .copied()
                .unwrap_or_default()
                .under(self.px[i]);
            store
                .with_column_mut(i, |column, _| {
                    column.state.support_z_um = self.height[i];
                    column.state.film = self.film[i];
                    column.state.gloss = self.gloss[i];
                    column.state.absorb = self.absorb[i];
                    column.state.reflectance = backing.reflectance;
                    column.state.residual = backing.residual;
                    column.state.color = backing.color;
                    Ok(())
                })
                .expect("valid material support");
        }
        self.material = Some(Material::new(store));
        self.backing.clear();
        let now = self.wet.clock.now;
        self.wet = crate::wet::Wet::new(0);
        self.wet.clock.now = now;
        self.base = None;
        self.e7_contact = Default::default();
    }

    pub fn material_store(&self) -> Option<&FilmStore> {
        self.material.as_ref().map(|m| &m.store)
    }

    pub fn edit_material<R>(
        &mut self,
        edit: impl FnOnce(&mut FilmStore) -> Result<R, MaterialError>,
    ) -> Result<R, MaterialError> {
        let mut material = self.material.clone().ok_or(MaterialError::Invalid(
            "canvas does not use material columns",
        ))?;
        material.commit();
        let result = edit(&mut material.store)?;
        self.material = Some(material);
        self.surf_gen += 1;
        self.base = None;
        self.e7_contact = Default::default();
        Ok(result)
    }

    pub(crate) fn install_material_state(&mut self, material: Material) -> std::io::Result<()> {
        let invalid = || {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid material checkpoint geometry or interval",
            )
        };
        let store = &material.store;
        if store.width() as usize != self.f.w
            || store.height() as usize != self.f.h
            || store.pixel_area() != self.px_mm() as f64 * self.px_mm() as f64
        {
            return Err(invalid());
        }
        match &material.interval_start {
            Some(start)
                if start.width() == store.width()
                    && start.height() == store.height()
                    && start.pixel_area() == store.pixel_area()
                    && start.environment == store.environment
                    && start.clock.seconds == store.clock.committed_seconds
                    && start.clock.committed_seconds == start.clock.seconds
                    && store.clock.seconds > start.clock.seconds
                    && store.clock.seconds < store.clock.pending_seconds
                    && store.clock.pending_seconds
                        == start.clock.seconds + super::INTERVAL_SECONDS
                    && start.clock.step == store.clock.step
                    && start.clock.seed == store.clock.seed => {}
            None if store.clock.seconds == store.clock.committed_seconds
                && store.clock.seconds == store.clock.pending_seconds => {}
            _ => return Err(invalid()),
        }
        let n = self.f.w * self.f.h;
        self.px = Vec::with_capacity(n);
        self.height = Vec::with_capacity(n);
        self.film = Vec::with_capacity(n);
        self.gloss = Vec::with_capacity(n);
        self.absorb = Vec::with_capacity(n);
        for i in 0..n {
            let state = store.read_column(i).map_err(|_| invalid())?.state;
            self.px.push(state.color);
            self.height.push(state.support_z_um);
            self.film.push(state.film);
            self.gloss.push(state.gloss);
            self.absorb.push(state.absorb);
        }
        self.absorb_any = self.absorb.iter().any(|&a| a > 0.0);
        self.wet.clock.now = store.clock.seconds / 60.0;
        self.material = Some(material);
        self.base = None;
        self.e7_contact = Default::default();
        Ok(())
    }

    pub(crate) fn material_look_px(&self, i: usize) -> Rgb {
        let store = self.material_store().expect("material backend");
        let column = store.read_column(i).expect("material pixel");
        let mut backing = Backing {
            reflectance: column.state.reflectance,
            residual: column.state.residual,
            color: column.state.color,
        };
        let drawing = self
            .drawing
            .as_ref()
            .map(|drawing| &drawing.cells[i])
            .filter(|cell| cell.a > 0.0 && cell.film >= 0.0);
        let mut drawn = false;
        let mut film = column.state.film;
        let overlay = |backing: &mut Backing, cell: &crate::graphite::Cell| {
            let color = cell.reflectance();
            let reflectance = crate::spectral::from_rgb(color);
            let projected = crate::spectral::to_rgb(&reflectance);
            for w in 0..crate::spectral::N {
                backing.reflectance[w] += cell.a * (reflectance[w] - backing.reflectance[w]);
            }
            for k in 0..3 {
                backing.residual[k] += cell.a * (color[k] - projected[k] - backing.residual[k]);
                backing.color[k] += cell.a * (color[k] - backing.color[k]);
            }
        };
        if let Some(drawing) = drawing.filter(|drawing| drawing.film <= film + 1e-4) {
            overlay(&mut backing, drawing);
            drawn = true;
        }
        let mut coatings = column.inert_coatings().peekable();
        for packet_index in 0..=column.packets.len() {
            while coatings
                .peek()
                .is_some_and(|coating| coating.before_packet as usize == packet_index)
            {
                let coating = coatings.next().unwrap();
                backing = super::inert::over(backing, coating);
                film += coating.thickness_um / crate::COAT_UM;
                if let Some(drawing) =
                    drawing.filter(|drawing| !drawn && drawing.film <= film + 1e-4)
                {
                    overlay(&mut backing, drawing);
                    drawn = true;
                }
            }
            if packet_index == column.packets.len() {
                break;
            }
            let packet = &column.packets[packet_index];
            let coverage = column.coverage[packet_index];
            let under = backing;
            let a = (packet.first_cell - column.first_cell) as usize;
            for cell in &column.cells[a..a + packet.cell_count as usize] {
                let formulation = &column.formulations[cell.recipe as usize];
                let prop = formulation.prop();
                let pigment_fraction = (1.0 - prop[8]) / (1.0 + prop[8] * (prop[4] - 1.0));
                if pigment_fraction > 0.0 {
                    backing = backing.over(
                        &formulation.latent().color,
                        prop[0],
                        cell.pigment_um / (pigment_fraction * crate::COAT_UM * coverage),
                        1.0,
                    );
                }
                film += if pigment_fraction > 0.0 {
                    cell.pigment_um / pigment_fraction / crate::COAT_UM
                } else {
                    cell.binder_um / crate::COAT_UM
                };
            }
            if coverage < 1.0 {
                for k in 0..crate::spectral::N {
                    backing.reflectance[k] = under.reflectance[k]
                        + coverage * (backing.reflectance[k] - under.reflectance[k]);
                }
                for k in 0..3 {
                    backing.residual[k] =
                        under.residual[k] + coverage * (backing.residual[k] - under.residual[k]);
                    backing.color[k] =
                        under.color[k] + coverage * (backing.color[k] - under.color[k]);
                }
            }
            if let Some(drawing) = drawing.filter(|drawing| !drawn && drawing.film <= film + 1e-4) {
                overlay(&mut backing, drawing);
                drawn = true;
            }
        }
        backing.color
    }

    pub(crate) fn material_seen(&self) -> Vec<Rgb> {
        (0..self.f.w * self.f.h)
            .map(|i| {
                if let Some(color) = self.sheet_seen(i) {
                    return color;
                }
                let column = self
                    .material_store()
                    .unwrap()
                    .read_column(i)
                    .expect("material pixel");
                let gloss = if column.cells.is_empty() {
                    column.state.gloss
                } else {
                    1.0
                };
                crate::canvas::haze(self.material_look_px(i), gloss)
            })
            .collect()
    }

    pub(crate) fn material_under(&self, x: f32, y: f32, r: f32) -> Rgb {
        let f = self.f;
        let n = if r * f.scale < 1.5 { 0 } else { 2 };
        let (mut sum, mut count) = ([0.0f32; 3], 0.0f32);
        for j in -n..=n {
            for i in -n..=n {
                let (dx, dy) = (i as f32 / 2.0 * r, j as f32 / 2.0 * r);
                if n > 0 && dx * dx + dy * dy > r * r * 1.01 {
                    continue;
                }
                let (px, py) = (x + dx, y + dy);
                if px < 0.0 || py < 0.0 || px >= f.width() || py >= f.height() || !f.holds(px, py) {
                    continue;
                }
                let color = self.material_look_px(f.index(px, py));
                for k in 0..3 {
                    sum[k] += color[k];
                }
                count += 1.0;
            }
        }
        if count == 0.0 {
            return self.material_look_px(f.index(x, y));
        }
        sum.map(|v| v / count)
    }

    pub(crate) fn material_wet_coats(&self) -> Vec<f32> {
        (0..self.f.w * self.f.h)
            .map(|i| self.material_mobile_coats_at(i))
            .collect()
    }

    pub(crate) fn material_mobile_coats_at(&self, i: usize) -> f32 {
        let column = self
            .material_store()
            .expect("material backend")
            .read_column(i)
            .expect("material pixel");
        if column
            .inert_coatings()
            .last()
            .is_some_and(|coating| coating.before_packet as usize == column.packets.len())
        {
            return 0.0;
        }
        let Some(top) = column.cells.last() else {
            return 0.0;
        };
        let limit =
            super::recipes::observations_for(&column.formulations[top.recipe as usize].kinetics())
                .set_to_touch_network as f32;
        let first_cell = column.inert_coatings().last().map_or(0, |coating| {
            (column.packets[coating.before_packet as usize].first_cell - column.first_cell) as usize
        });
        column.cells[first_cell..]
            .iter()
            .rev()
            .take_while(|cell| cell.network <= limit)
            .map(|cell| cell.pigment_um + cell.binder_um)
            .sum::<f32>()
            / crate::COAT_UM
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphite::{Lead, Mark};

    #[test]
    fn material_drawing_uses_existing_overlay_without_retaining_dense_paint() {
        let mut canvas = Canvas::new_window_with_material(32, 1.0, [0.8; 3], None, 9, 1);
        let support = canvas.px.clone();
        let mut before = Vec::new();
        canvas
            .material_store()
            .unwrap()
            .write_to(&mut before)
            .unwrap();
        canvas.draw(
            &Lead::chalk(),
            &Mark {
                pts: vec![(100.0, 500.0), (900.0, 500.0)],
                pressure: vec![0.8],
            },
            0.0,
            7,
        );
        assert!(
            canvas
                .drawing
                .as_ref()
                .unwrap()
                .cells
                .iter()
                .any(|cell| cell.a > 0.0)
        );
        assert!(canvas.seen().iter().any(|color| color[0] > 0.8));
        assert_eq!(canvas.px, support);
        assert!(canvas.wet.vol.is_empty() && canvas.wet.lat.is_empty());
        assert!(!canvas.drawing_projection);
        let mut after = Vec::new();
        canvas
            .material_store()
            .unwrap()
            .write_to(&mut after)
            .unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn material_drawing_keeps_its_position_between_reactive_packets() {
        use crate::oil::{
            chemistry::{Cell, Kinetics},
            store::{DepositMode, Formulation},
            transfer::{Fragment, MaterialTransfer, TransferCell},
        };
        use std::sync::Arc;
        let mut canvas = Canvas::new_window_with_material(1, 1.0, [0.8; 3], None, 9, 1);
        let red = [0.95, 0.02, 0.02];
        let cell = &mut canvas.drawing_mut().cells[0];
        cell.a = 1.0;
        cell.r = red;
        cell.film = 0.0;
        cell.optics = None;
        assert_eq!(canvas.material_look_px(0), red);
        let paint = crate::Paint::body([0.02, 0.03, 0.9]);
        let recipe =
            Arc::new(Formulation::new(paint.latent(), paint.prop(), Kinetics::default()).unwrap());
        canvas
            .edit_material(|store| {
                let area = store.pixel_area();
                let mut fragment = Fragment::new(
                    vec![TransferCell::from_cell(
                        Cell {
                            pigment_um: 2500.0,
                            binder_um: 2500.0,
                            network: 1.0,
                            ..Cell::default()
                        },
                        recipe,
                        area,
                    )],
                    1.0,
                );
                fragment.reference_area = area;
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
        assert!(canvas.material_look_px(0)[2] > canvas.material_look_px(0)[0]);
        let film = canvas.material_film_at(0);
        canvas.drawing_mut().cells[0].film = film;
        assert_eq!(canvas.material_look_px(0), red);
    }
}
