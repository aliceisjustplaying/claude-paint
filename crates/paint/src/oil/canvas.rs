use super::Material;
use super::store::{FilmStore, MaterialError};
use crate::Canvas;
use crate::color::Rgb;
use crate::mixer::Backing;

impl Canvas {
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
        for (packet, &coverage) in column.packets.iter().zip(column.coverage) {
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
        let store = self.material_store().expect("material backend");
        (0..self.f.w * self.f.h)
            .map(|i| {
                let column = store.read_column(i).expect("material pixel");
                if super::stage(&column) == crate::Stage::Dry {
                    return 0.0;
                }
                column
                    .cells
                    .iter()
                    .map(|cell| cell.pigment_um + cell.binder_um)
                    .sum::<f32>()
                    / crate::COAT_UM
            })
            .collect()
    }
}
