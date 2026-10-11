use super::{
    BRIDGE_MM, CAP_UM, FACES, LIFT, Pool, Rag, SAG_UM, SLOW_COATS, Touch, WICK, local_max, pace,
    reach_fluid, vn,
};
use crate::{
    COAT_UM, Canvas,
    tally::Tally,
    wet::{LAT, Latent, Prop},
};

pub const FACE_COUNT: usize = 12;
pub const GRID: usize = 3;
pub const CELLS: usize = GRID * GRID;
pub const EXPOSED_SOLVENT_HALF_SECS: f64 = 180.0;
pub const FOLDED_SOLVENT_HALF_SECS: f64 = 540.0;
pub const DRY_MOBILE_HALF_SECS: f64 = 8.0;
pub const WET_MOBILE_HALF_SECS: f64 = 60.0;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SolventLedger {
    pub evaporated_mm3: f64,
    pub discarded_trace_mm3: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub paint_mm3: f64,
    pub solvent_mm3: f64,
    pub latent_moment: [f64; LAT],
    pub property_moment: [f64; 5],
    pub cure_moment: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            paint_mm3: 0.0,
            solvent_mm3: 0.0,
            latent_moment: [0.0; LAT],
            property_moment: [0.0; 5],
            cure_moment: 0.0,
        }
    }
}

impl Material {
    pub fn add(&mut self, other: Self) {
        self.paint_mm3 += other.paint_mm3;
        self.solvent_mm3 += other.solvent_mm3;
        self.cure_moment += other.cure_moment;
        for (a, b) in self.latent_moment.iter_mut().zip(other.latent_moment) {
            *a += b;
        }
        for (a, b) in self.property_moment.iter_mut().zip(other.property_moment) {
            *a += b;
        }
    }

    pub fn take(&mut self, fraction: f64) -> Self {
        let f = fraction.clamp(0.0, 1.0);
        let mut out = *self;
        out.paint_mm3 *= f;
        out.solvent_mm3 *= f;
        out.cure_moment *= f;
        for k in 0..LAT {
            out.latent_moment[k] *= f;
            self.latent_moment[k] -= out.latent_moment[k];
        }
        for k in 0..5 {
            out.property_moment[k] *= f;
            self.property_moment[k] -= out.property_moment[k];
        }
        self.paint_mm3 -= out.paint_mm3;
        self.solvent_mm3 -= out.solvent_mm3;
        self.cure_moment -= out.cure_moment;
        out
    }

    fn picked(paint_mm3: f64, solvent_mm3: f64, lat: Latent, prop: Prop, cure: f32) -> Self {
        Self {
            paint_mm3,
            solvent_mm3,
            latent_moment: lat.map(|v| v as f64 * paint_mm3),
            property_moment: prop.map(|v| v as f64 * paint_mm3),
            cure_moment: cure as f64 * paint_mm3,
        }
    }

    fn pool(&self, unit: f64) -> Pool {
        let v = self.paint_mm3;
        Pool {
            vol: (v / unit) as f32,
            lat: self.latent_moment.map(|x| (x / v) as f32),
            hide: self.property_moment.map(|x| (x / v) as f32),
            cure: (self.cure_moment / v) as f32,
            solv: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    pub mobile: Material,
    pub bound: Material,
}

impl Cell {
    fn paint(&self) -> f64 {
        self.mobile.paint_mm3 + self.bound.paint_mm3
    }
    fn solvent(&self) -> f64 {
        self.mobile.solvent_mm3 + self.bound.solvent_mm3
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Face {
    pub cells: [Cell; CELLS],
    pub work_secs: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cloth {
    pub faces: [Face; FACE_COUNT],
    pub active: usize,
    pub face_capacity_mm3: f64,
    pub width_mm: f32,
    pub hand_time: bool,
    pub solvent_input_mm3: f64,
    pub evaporated_solvent_mm3: f64,
    pub waste: Material,
    pub gross_pickup_mm3: f64,
    pub redeposited_mm3: f64,
    pub solvent_released_mm3: f64,
    pub solvent_absorbed_mm3: f64,
    pub at_min: f64,
    pub at_hand_secs: f64,
}

impl Cloth {
    pub fn inventory(&self) -> Material {
        let mut total = Material::default();
        for face in &self.faces {
            for cell in &face.cells {
                total.add(cell.mobile);
                total.add(cell.bound);
            }
        }
        total
    }

    pub fn mobile_mm3(&self) -> f64 {
        self.faces
            .iter()
            .flat_map(|f| &f.cells)
            .map(|c| c.mobile.paint_mm3)
            .sum()
    }

    pub fn discard(&mut self) {
        self.waste.add(self.inventory());
        self.faces = [Face::default(); FACE_COUNT];
    }

    fn advance(&mut self, seconds: f64) {
        if seconds <= 0.0 {
            return;
        }
        let cap = self.face_capacity_mm3 / CELLS as f64;
        for (face_index, face) in self.faces.iter_mut().enumerate() {
            let half = if face_index == self.active {
                EXPOSED_SOLVENT_HALF_SECS
            } else {
                FOLDED_SOLVENT_HALF_SECS
            };
            let retain = (-seconds * std::f64::consts::LN_2 / half).exp();
            for cell in &mut face.cells {
                let sat = (cell.solvent() / cap).max(0.0);
                let stiffness = if cell.mobile.paint_mm3 > 0.0 {
                    (cell.mobile.property_moment[1] / cell.mobile.paint_mm3).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let lambda = std::f64::consts::LN_2 / half;
                let saturated_secs = if sat > 1.0 {
                    (sat.ln() / lambda).min(seconds)
                } else {
                    0.0
                };
                let dt = seconds - saturated_secs;
                let b = (WET_MOBILE_HALF_SECS - DRY_MOBILE_HALF_SECS) * sat.min(1.0);
                let exposure = saturated_secs / WET_MOBILE_HALF_SECS
                    + (dt
                        + ((DRY_MOBILE_HALF_SECS + b * (-lambda * dt).exp())
                            / (DRY_MOBILE_HALF_SECS + b))
                            .ln()
                            / lambda)
                        / DRY_MOBILE_HALF_SECS;
                cell.bound.add(cell.mobile.take(
                    1.0 - (-exposure * std::f64::consts::LN_2 / (1.0 + 0.5 * stiffness)).exp(),
                ));
                for store in [&mut cell.mobile, &mut cell.bound] {
                    let lost = store.solvent_mm3 * (1.0 - retain);
                    store.solvent_mm3 -= lost;
                    self.evaporated_solvent_mm3 += lost;
                }
            }
        }
    }

    pub(super) fn sync(&mut self, now_min: f64, hand_secs: f64) {
        let dt = ((now_min - self.at_min) * 60.0
            + if self.hand_time {
                0.0
            } else {
                hand_secs - self.at_hand_secs
            })
        .max(0.0);
        self.advance(dt);
        self.at_min = self.at_min.max(now_min);
        self.at_hand_secs = self.at_hand_secs.max(hand_secs);
    }
}

impl Rag {
    pub fn with_transport(mut self, canvas: &Canvas) -> Self {
        assert!(
            self.transport.is_none()
                && self.load == 0.0
                && self.soaked == 0.0
                && self.damp == 0.0
                && self.solvent_mm3 == 0.0,
            "rag-1 requires a fresh rag"
        );
        assert!(
            canvas.engine() >= 3 && self.width.is_finite() && self.width > 0.0,
            "rag-1 requires engine 3+ and a finite pad"
        );
        let width_mm = self.width * canvas.mm_per_unit();
        self.transport = Some(Cloth {
            faces: [Face::default(); FACE_COUNT],
            active: 0,
            face_capacity_mm3: width_mm as f64 * width_mm as f64 * CAP_UM as f64 / 1000.0,
            width_mm,
            hand_time: canvas.hand_time().is_some(),
            solvent_input_mm3: 0.0,
            evaporated_solvent_mm3: 0.0,
            waste: Material::default(),
            gross_pickup_mm3: 0.0,
            redeposited_mm3: 0.0,
            solvent_released_mm3: 0.0,
            solvent_absorbed_mm3: 0.0,
            at_min: canvas.now_min(),
            at_hand_secs: canvas.tally().secs,
        });
        self
    }

    pub fn age_transport(&mut self, canvas: &Canvas) {
        if let Some(cloth) = &mut self.transport {
            cloth.sync(canvas.now_min(), canvas.tally().secs);
        }
        self.transport_summary();
    }

    pub(super) fn transport_summary(&mut self) {
        let Some(cloth) = &self.transport else { return };
        let face = &cloth.faces[cloth.active];
        self.load =
            (face.cells.iter().map(Cell::paint).sum::<f64>() / cloth.face_capacity_mm3) as f32;
        self.soaked =
            (cloth.inventory().paint_mm3 / (cloth.face_capacity_mm3 * FACES as f64)) as f32;
        self.damp = (face.cells.iter().map(Cell::solvent).sum::<f64>() / cloth.face_capacity_mm3)
            .clamp(0.0, 1.0) as f32;
        self.solvent_mm3 = cloth.inventory().solvent_mm3;
        self.wet_at = cloth.at_min;
    }

    pub fn refold_at(&mut self, now_min: f64, tally: &mut Tally) {
        if self.transport.is_none() {
            self.refold(tally);
            return;
        }
        assert!(
            now_min.is_finite(),
            "experimental cloth fold time must be finite"
        );
        let cloth = self.transport.as_mut().unwrap();
        cloth.sync(now_min, tally.secs);
        cloth.advance(pace::REFOLD);
        cloth.at_hand_secs += pace::REFOLD;
        if cloth.hand_time {
            cloth.at_min += pace::REFOLD / 60.0;
        }
        cloth.active = (cloth.active + 1) % FACE_COUNT;
        self.fold = cloth.active as u32;
        tally.secs += pace::REFOLD;
        self.transport_summary();
    }

    pub(super) fn transport_dip(&mut self, amount: f32, now_min: f64, tally: &mut Tally) {
        assert!(
            amount.is_finite() && now_min.is_finite(),
            "rag-1 dip must be finite"
        );
        let cloth = self.transport.as_mut().unwrap();
        cloth.sync(
            now_min
                + if cloth.hand_time {
                    pace::DIP / 60.0
                } else {
                    0.0
                },
            tally.secs + pace::DIP,
        );
        let target = amount.clamp(0.0, 1.0) as f64 * cloth.face_capacity_mm3 / CELLS as f64;
        for cell in &mut cloth.faces[cloth.active].cells {
            let added = (target - cell.solvent()).max(0.0);
            cell.mobile.solvent_mm3 += added;
            cloth.solvent_input_mm3 += added;
            let rewet = (added / (cloth.face_capacity_mm3 / CELLS as f64)).min(1.0) * 0.12;
            cell.mobile.add(cell.bound.take(rewet));
        }
        tally.secs += pace::DIP;
        self.transport_summary();
    }
}

#[derive(Clone, Copy)]
struct Contact {
    i: usize,
    cell: usize,
    exposure: f64,
    rim: f64,
    press: f32,
}

impl Canvas {
    pub fn rag_solvent_ledger(&self) -> Option<SolventLedger> {
        self.wet.solvent_ledger
    }

    pub(super) fn check_rag_transport(&self, rag: &Rag, pts: &[(f32, f32)], pressure: &[f32]) {
        let cloth = rag.transport.as_ref().unwrap();
        assert!(
            self.engine >= 3
                && rag.width.is_finite()
                && (cloth.width_mm - rag.width * self.mm_per_unit).abs() < 1e-5
                && cloth.hand_time == self.hand_time().is_some(),
            "rag-1 scale and clock policy are fixed at construction"
        );
        assert!(
            pts.iter().all(|p| p.0.is_finite() && p.1.is_finite())
                && pressure.iter().all(|p| p.is_finite()),
            "rag-1 contact must be finite"
        );
    }

    pub(super) fn rag_transport_contact(
        &mut self,
        rag: &mut Rag,
        bbox: (f32, f32, f32, f32),
        pressure: f32,
        touch: Touch,
        center: (f32, f32),
        seed: u64,
        expo: impl Fn(f32, f32) -> (f32, f32, f32, f32),
    ) -> f64 {
        let f = self.f;
        let r = (
            (bbox.0 * f.scale).floor().max(0.0) as usize,
            (bbox.1 * f.scale).floor().max(0.0) as usize,
            ((bbox.2 * f.scale).ceil().max(0.0) as usize + 1).min(f.full_w),
            ((bbox.3 * f.scale).ceil().max(0.0) as usize + 1).min(f.full_h),
        );
        let cloth = rag.transport.as_mut().unwrap();
        let Some((x0, y0, x1, y1)) = f.clip(r) else {
            cloth.advance(touch.secs as f64);
            cloth.at_hand_secs += touch.secs as f64;
            if cloth.hand_time {
                cloth.at_min += touch.secs as f64 / 60.0;
            }
            return 0.0;
        };
        let p = pressure.clamp(0.0, 1.0);
        let px_mm = self.px_mm();
        let area = (px_mm as f64).powi(2);
        let unit = area * COAT_UM as f64 / 1000.0;
        let cap = cloth.face_capacity_mm3 / CELLS as f64;
        let face = &cloth.faces[cloth.active];
        let mut weights = [0.0; CELLS];
        let mut smear_weights = [0.0; CELLS];
        let mut contacts = Vec::new();
        for y in y0..y1 {
            for x in x0..x1 {
                let (ux, uy) = (f.ux(x), f.uy(y));
                let (pad, c, rim, press) = expo(ux, uy);
                if pad <= 0.0 {
                    continue;
                }
                let (u, v) = (
                    (ux - center.0) * self.mm_per_unit,
                    (uy - center.1) * self.mm_per_unit,
                );
                let detail = vn(
                    u / 1.3,
                    v / 2.1 + face.work_secs as f32 * 0.025 * p,
                    seed ^ 0xA6F1,
                );
                let contact = (c * (0.55 + 0.45 * detail)
                    + 0.12 * p * (1.0 - (-face.work_secs as f32 / 4.0).exp()) * detail)
                    .clamp(0.0, 1.0);
                let exposure = (pad * contact) as f64;
                if exposure <= 0.0 {
                    continue;
                }
                let col = (((u / cloth.width_mm + 0.5) * GRID as f32).floor() as i32)
                    .clamp(0, GRID as i32 - 1) as usize;
                let row = (((v / cloth.width_mm + 0.5) * GRID as f32).floor() as i32)
                    .clamp(0, GRID as i32 - 1) as usize;
                let cell = row * GRID + col;
                let rim = if touch.slip_mm > 0.0 {
                    0.15 + 0.85 * rim as f64
                } else {
                    1.0
                };
                weights[cell] += exposure;
                smear_weights[cell] += exposure * rim;
                contacts.push(Contact {
                    i: y * f.w + x,
                    cell,
                    exposure,
                    rim,
                    press,
                });
            }
        }
        self.wet.ensure_solvent();
        self.wet
            .solvent_ledger
            .get_or_insert_with(SolventLedger::default);
        let mut released = [0.0; CELLS];
        let mut laid = [Material::default(); CELLS];
        let face = &mut cloth.faces[cloth.active];
        for k in 0..CELLS {
            let covered =
                (weights[k] * area / (cloth.width_mm as f64).powi(2) * CELLS as f64).min(1.0);
            let sat = (face.cells[k].solvent() / cap).clamp(0.0, 1.0);
            let flood = crate::smoothstep(0.25, 0.85, sat as f32) as f64;
            let release = 1.0
                - (-covered
                    * (touch.secs as f64 * (0.03 + 3.0 * p as f64 * flood)
                        + touch.slip_mm as f64 / cloth.width_mm as f64 * 0.8 * flood))
                    .exp();
            for store in [&mut face.cells[k].mobile, &mut face.cells[k].bound] {
                let solvent = store.solvent_mm3 * release;
                store.solvent_mm3 -= solvent;
                released[k] += solvent;
            }
            let mobile = &face.cells[k].mobile;
            let phi = mobile.solvent_mm3 / (mobile.paint_mm3 + mobile.solvent_mm3).max(1e-12);
            let transfer = 1.0
                - (-covered
                    * (touch.slip_mm as f64 / cloth.width_mm as f64 * (0.6 + 2.0 * phi)
                        + touch.secs as f64 * 0.08 * phi))
                    .exp();
            laid[k] = face.cells[k].mobile.take(transfer);
            face.cells[k].mobile.solvent_mm3 += laid[k].solvent_mm3;
            laid[k].solvent_mm3 = 0.0;
        }
        for c in &contacts {
            let solvent = released[c.cell] * c.exposure / weights[c.cell];
            self.wet.solv[c.i] += (solvent * 1000.0 / area) as f32;
        }
        let rb = ((BRIDGE_MM / px_mm).round() as usize).max(1);
        let (bx0, by0, bx1, by1) = (
            x0.saturating_sub(rb),
            y0.saturating_sub(rb),
            (x1 + rb).min(f.w),
            (y1 + rb).min(f.h),
        );
        let bw = bx1 - bx0;
        let surf: Vec<f32> = (by0..by1)
            .flat_map(|y| (bx0..bx1).map(move |x| (y, x)))
            .map(|(y, x)| {
                let i = y * f.w + x;
                self.height[i] + self.wet.vol[i].max(0.0) * COAT_UM + self.wet.solv[i]
            })
            .collect();
        let peaks = local_max(&surf, bw, by1 - by0, rb);
        let timed = self.wet.clock.px.len() == self.wet.vol.len();
        let mut asks = Vec::new();
        let mut wanted = [0.0; CELLS];
        for c in &contacts {
            let v = self.wet.vol[c.i];
            if v <= 1e-6 {
                continue;
            }
            let solvent = self.wet.solv[c.i] / COAT_UM;
            let phi = solvent / (solvent + v);
            let stiff = self.wet.hide[c.i][1].clamp(0.0, 1.0);
            let shear = (touch.slip_mm / (0.25 * cloth.width_mm)).clamp(0.0, 1.0);
            let drive = p * c.press * (1.0 + 0.3 * shear);
            let yield_drive = 0.12 + 0.55 * stiff * (1.0 - phi).powi(2);
            let unyielded = (-(drive / yield_drive).powi(4)).exp();
            let fl = if timed {
                reach_fluid(self.wet.clock.px[c.i].cure)
            } else {
                1.0
            } / (1.0
                + 10.0 * stiff.powi(2) * (1.0 - phi).powi(3) * unyielded / (0.1 + drive));
            let j = (c.i / f.w - by0) * bw + (c.i % f.w - bx0);
            let reach = SAG_UM * (0.25 + 1.5 * p * c.press) * (1.0 + 0.5 * phi);
            let near = ((surf[j] - peaks[j] + reach) / COAT_UM / (1.0 + solvent / v)).clamp(0.0, v);
            let wick = (WICK + (1.0 - WICK) * phi) * c.press;
            let avail = near + wick * (v - near);
            let thirst = 1.0 - (face.cells[c.cell].paint() / cap).clamp(0.0, 1.0).powi(2);
            let rate = LIFT as f64
                * (0.7 + 0.6 * p as f64)
                * fl as f64
                * thirst
                * (1.0 + 2.0 * phi as f64);
            let take = (avail as f64 * (1.0 - (-rate * c.exposure).exp()) * v as f64
                / (v as f64
                    + SLOW_COATS as f64 * (2.0 - near as f64 / v as f64)
                        / (1.0 + 2.0 * phi as f64)))
                .min(v as f64);
            wanted[c.cell] += take * unit;
            asks.push((c.i, c.cell, take));
        }
        let mut got = [Material::default(); CELLS];
        for (i, k, ask) in asks {
            let room = (cap - face.cells[k].paint()).max(0.0);
            let scale = if wanted[k] > room {
                room / wanted[k]
            } else {
                1.0
            };
            let take = (ask * scale) as f32;
            if take <= 0.0 {
                continue;
            }
            let paint = take as f64 * unit;
            let lat = self.wet.lat[i];
            let prop = self.wet.hide[i];
            let cure = if timed {
                self.wet.clock.px[i].cure
            } else {
                0.0
            };
            let solvent = self.rag_take(i, take) as f64 * unit;
            got[k].add(Material::picked(paint, solvent, lat, prop, cure));
        }
        let mut solvent_asks = Vec::new();
        let mut solvent_wanted = [0.0; CELLS];
        for c in &contacts {
            let solvent = self.wet.solv[c.i] as f64 * area / 1000.0;
            if solvent <= 0.0 {
                continue;
            }
            let sat =
                ((face.cells[c.cell].solvent() + got[c.cell].solvent_mm3) / cap).clamp(0.0, 1.0);
            let fraction = 1.0
                - (-(LIFT as f64) * (0.7 + 0.6 * p as f64) * c.exposure * (1.0 - sat.powi(2)))
                    .exp();
            let ask = solvent * fraction;
            solvent_wanted[c.cell] += ask;
            solvent_asks.push((c.i, c.cell, ask));
        }
        let solvent_room: [f64; CELLS] =
            std::array::from_fn(|k| (cap - face.cells[k].solvent() - got[k].solvent_mm3).max(0.0));
        for (i, k, ask) in solvent_asks {
            let room = solvent_room[k];
            let scale = if solvent_wanted[k] > room {
                room / solvent_wanted[k]
            } else {
                1.0
            };
            let before = self.wet.solv[i];
            self.wet.solv[i] = (before - (ask * scale * 1000.0 / area) as f32).max(0.0);
            got[k].solvent_mm3 += (before as f64 - self.wet.solv[i] as f64) * area / 1000.0;
        }
        for c in &contacts {
            let store = &laid[c.cell];
            let weight = c.exposure * c.rim / smear_weights[c.cell];
            if store.paint_mm3 > 0.0 {
                self.rag_lay(
                    c.i,
                    (store.paint_mm3 * weight / unit) as f32,
                    &store.pool(unit),
                    (store.solvent_mm3 * weight * 1000.0 / area) as f32,
                );
            } else {
                self.wet.solv[c.i] += (store.solvent_mm3 * weight * 1000.0 / area) as f32;
            }
        }
        let mut net = 0.0;
        for k in 0..CELLS {
            net += got[k].paint_mm3 - laid[k].paint_mm3;
            cloth.gross_pickup_mm3 += got[k].paint_mm3;
            cloth.redeposited_mm3 += laid[k].paint_mm3;
            cloth.solvent_absorbed_mm3 += got[k].solvent_mm3;
            cloth.solvent_released_mm3 += released[k] + laid[k].solvent_mm3;
            face.cells[k].mobile.add(got[k]);
        }
        face.work_secs += touch.secs as f64;
        self.wet.touch(x0, y0, x1, y1);
        cloth.advance(touch.secs as f64);
        cloth.at_hand_secs += touch.secs as f64;
        if cloth.hand_time {
            cloth.at_min += touch.secs as f64 / 60.0;
        }
        net
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canvas(stiff: f32, solvent: f32) -> Canvas {
        let mut c = Canvas::new(96, 1.0, [0.5, 0.4, 0.3])
            .with_engine(5)
            .with_size_mm(100.0);
        c.wet.ensure_solvent();
        for i in 0..c.wet.vol.len() {
            if i % 96 < 48 {
                c.wet.vol[i] = 0.8;
                c.wet.lat[i] = mixbox::linear_float_rgb_to_latent(&[0.06, 0.1, 0.2]);
                c.wet.hide[i] = [0.8, stiff, 1.1, 0.2, 0.7];
                c.wet.solv[i] = solvent * COAT_UM;
            }
        }
        c
    }

    fn inventory(c: &Canvas, r: &Rag) -> Material {
        let mut total = Material::default();
        let unit = (c.px_mm() as f64).powi(2) * COAT_UM as f64 / 1000.0;
        for i in 0..c.wet.vol.len() {
            total.add(Material::picked(
                c.wet.vol[i] as f64 * unit,
                c.wet.solv.get(i).copied().unwrap_or(0.0) as f64 / COAT_UM as f64 * unit,
                c.wet.lat[i],
                c.wet.hide[i],
                c.wet.clock.px.get(i).map_or(0.0, |p| p.cure),
            ));
        }
        let cloth = r.transport.as_ref().unwrap();
        total.add(cloth.inventory());
        total.add(cloth.waste);
        total.solvent_mm3 += cloth.evaporated_solvent_mm3 - cloth.solvent_input_mm3;
        if let Some(ledger) = c.rag_solvent_ledger() {
            total.solvent_mm3 += ledger.evaporated_mm3 + ledger.discarded_trace_mm3;
        }
        total
    }

    fn close(a: Material, b: Material) {
        let near = |x: f64, y: f64| assert!((x - y).abs() < 2e-6 * x.abs().max(1.0), "{x} != {y}");
        near(a.paint_mm3, b.paint_mm3);
        near(a.solvent_mm3, b.solvent_mm3);
        near(a.cure_moment, b.cure_moment);
        for (x, y) in a.latent_moment.into_iter().zip(b.latent_moment) {
            near(x, y);
        }
        for (x, y) in a.property_moment.into_iter().zip(b.property_moment) {
            near(x, y);
        }
    }

    #[test]
    fn dip_absorb_repeat_refold_evaporate_and_discard_close_every_moment() {
        let mut c = canvas(0.6, 0.5);
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        let initial = inventory(&c, &r);
        r.dip(0.95, c.now_min(), c.tally_mut());
        close(initial, inventory(&c, &r));
        let fixed = (c.pixels().to_vec(), c.surface_um().to_vec());
        c.rag_wipe(&mut r, &[(150.0, 500.0), (850.0, 500.0)], &[0.6], 41);
        close(initial, inventory(&c, &r));
        let cloth = r.transport.as_ref().unwrap();
        assert!(
            cloth.solvent_released_mm3 > 0.0
                && cloth.solvent_absorbed_mm3 > 0.0
                && cloth.redeposited_mm3 > 0.0
        );
        r.refold_at(c.now_min(), c.tally_mut());
        c.rag_blot(&mut r, 300.0, 500.0, 0.6, 42);
        close(initial, inventory(&c, &r));
        for _ in 1..FACE_COUNT {
            r.refold_at(c.now_min(), c.tally_mut());
        }
        c.rag_wipe(&mut r, &[(850.0, 650.0), (150.0, 650.0)], &[0.5], 43);
        close(initial, inventory(&c, &r));
        assert_eq!(fixed.0, c.pixels());
        assert_eq!(fixed.1, c.surface_um());
        let cloth = r.transport.as_mut().unwrap();
        cloth.advance(3600.0);
        close(initial, inventory(&c, &r));
        assert!(r.transport.as_ref().unwrap().mobile_mm3() < 1e-5);
        r.transport.as_mut().unwrap().discard();
        close(initial, inventory(&c, &r));
        assert_eq!(
            r.transport.as_ref().unwrap().inventory(),
            Material::default()
        );
    }

    #[test]
    fn absorbing_a_wash_wets_a_dry_face_and_keeps_mobile_dirt_after_aiming() {
        let mut c = canvas(0.1, 2.0);
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        c.rag_wipe(&mut r, &[(150.0, 500.0), (400.0, 500.0)], &[0.6], 41);
        assert!(r.damp > 0.0 && r.transport.as_ref().unwrap().mobile_mm3() > 0.1);
        let mobile = r.transport.as_ref().unwrap().mobile_mm3();
        let before = c.wet_total();
        c.rag_wipe(&mut r, &[(650.0, 500.0), (850.0, 500.0)], &[0.4], 41);
        assert!(c.wet_total() > before);
        assert!(r.transport.as_ref().unwrap().mobile_mm3() > 0.5 * mobile);
    }

    #[test]
    fn refolding_preserves_a_used_face_and_returning_can_redeposit_it() {
        let mut c = canvas(0.2, 0.0);
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        r.dip(0.6, c.now_min(), c.tally_mut());
        c.rag_blot(&mut r, 300.0, 500.0, 0.7, 41);
        let original = r.transport.as_ref().unwrap().faces[0];
        let mass = original.cells.iter().map(Cell::paint).sum::<f64>();
        assert!(mass > 0.0);
        r.refold_at(c.now_min(), c.tally_mut());
        assert_eq!(r.transport.as_ref().unwrap().active, 1);
        assert_eq!(r.load, 0.0);
        c.rag_wipe(&mut r, &[(650.0, 700.0), (850.0, 700.0)], &[0.5], 41);
        assert_eq!(c.wet_um(750.0, 700.0), 0.0);
        for _ in 1..FACE_COUNT {
            r.refold_at(c.now_min(), c.tally_mut());
        }
        assert_eq!(r.transport.as_ref().unwrap().active, 0);
        let returned = &r.transport.as_ref().unwrap().faces[0];
        assert!((returned.cells.iter().map(Cell::paint).sum::<f64>() - mass).abs() < 1e-10);
        assert!(
            returned
                .cells
                .iter()
                .map(|s| s.mobile.paint_mm3)
                .sum::<f64>()
                > 0.1 * mass
        );
        let before = c.wet_total();
        c.rag_wipe(&mut r, &[(650.0, 700.0), (850.0, 700.0)], &[0.5], 41);
        assert!(c.wet_total() > before);
    }

    #[test]
    fn localized_dirt_stays_on_its_contact_track() {
        let mut c = canvas(0.1, 0.0);
        c.wet.vol.fill(0.0);
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        r.transport.as_mut().unwrap().faces[0].cells[0].mobile = Material::picked(
            1.0,
            1.0,
            mixbox::linear_float_rgb_to_latent(&[0.06, 0.1, 0.2]),
            [0.8, 0.1, 1.0, 0.0, 1.0],
            0.0,
        );
        c.rag_blot(&mut r, 500.0, 500.0, 0.6, 41);
        let mut laid = 0.0;
        for i in 0..c.wet.vol.len() {
            if c.wet.vol[i] > 0.0 {
                assert!(c.f.ux(i % c.f.w) < 500.0 && c.f.uy(i / c.f.w) < 500.0);
                laid += c.wet.vol[i];
            }
        }
        assert!(laid > 0.0);
    }

    #[test]
    fn wet_floods_and_lifts_while_barely_damp_retains_more_dragged_dirt() {
        let base = canvas(0.8, 0.0);
        let mut outcomes = Vec::new();
        for amount in [0.15, 0.95] {
            let mut c = base.clone();
            let mut r = Rag::new(126.0, 7).with_transport(&c);
            r.dip(amount, c.now_min(), c.tally_mut());
            c.rag_wipe(&mut r, &[(150.0, 500.0), (850.0, 500.0)], &[0.4], 41);
            let t = r.transport.as_ref().unwrap();
            outcomes.push((t.solvent_released_mm3, t.gross_pickup_mm3));
        }
        assert!(outcomes[1].0 > 10.0 * outcomes[0].0, "{outcomes:?}");
        assert!(outcomes[1].1 > 2.0 * outcomes[0].1, "{outcomes:?}");
    }

    #[test]
    fn stiffness_and_actual_film_solvent_control_gentle_pickup() {
        let mut lifts = Vec::new();
        for (stiff, solvent) in [(0.1, 0.0), (0.9, 0.0), (0.9, 1.6)] {
            let mut c = canvas(stiff, solvent);
            let mut r = Rag::new(126.0, 7).with_transport(&c);
            c.rag_blot(&mut r, 300.0, 500.0, 0.25, 41);
            lifts.push(r.transport.as_ref().unwrap().gross_pickup_mm3);
        }
        assert!(
            lifts[0] > 3.0 * lifts[1] && lifts[2] > 3.0 * lifts[1],
            "{lifts:?}"
        );
    }

    #[test]
    fn firm_sliding_contact_yields_fresh_paste_and_retains_useful_carry() {
        let base = canvas(0.7, 0.0);
        let mut old = base.clone();
        let mut legacy = Rag::new(126.0, 7);
        let baseline = old.rag_wipe(&mut legacy, &[(150.0, 500.0), (850.0, 500.0)], &[0.6], 41);
        let mut lifts = Vec::new();
        for pressure in [0.25, 0.6] {
            let mut c = base.clone();
            let mut r = Rag::new(126.0, 7).with_transport(&c);
            let initial = inventory(&c, &r);
            let lifted = c.rag_wipe(&mut r, &[(150.0, 500.0), (850.0, 500.0)], &[pressure], 41);
            close(initial, inventory(&c, &r));
            lifts.push(lifted);
            if pressure == 0.6 {
                assert!(
                    lifted >= 0.4 * baseline,
                    "{lifted} versus legacy {baseline}"
                );
                let before = c.wet_total();
                c.rag_wipe(&mut r, &[(650.0, 700.0), (850.0, 700.0)], &[0.4], 41);
                assert!(c.wet_total() > before);
                close(initial, inventory(&c, &r));
            }
        }
        assert!(lifts[1] > 2.0 * lifts[0], "{lifts:?}");
    }

    #[test]
    fn solvent_half_lives_and_binding_are_independent_of_pad_width() {
        for width in [126.0, 400.0] {
            let mut c = canvas(0.2, 0.0);
            let mut r = Rag::new(width, 7).with_transport(&c);
            r.dip(0.6, c.now_min(), c.tally_mut());
            let t = r.transport.as_mut().unwrap();
            let start = t.inventory().solvent_mm3;
            t.advance(EXPOSED_SOLVENT_HALF_SECS);
            assert!((t.inventory().solvent_mm3 / start - 0.5).abs() < 1e-12);
            assert!((t.evaporated_solvent_mm3 / start - 0.5).abs() < 1e-12);
            let cell = &mut t.faces[0].cells[0];
            cell.mobile = Material::picked(1.0, 0.0, [0.0; LAT], [0.8, 0.0, 1.0, 0.0, 1.0], 0.0);
            cell.bound = Material::default();
            t.advance(DRY_MOBILE_HALF_SECS);
            assert!((t.faces[0].cells[0].mobile.paint_mm3 - 0.5).abs() < 1e-12);
        }
    }

    #[test]
    fn explicit_wait_and_hand_time_are_counted_once() {
        for hand in [false, true] {
            let mut c = canvas(0.1, 0.0);
            if hand {
                c.set_hand_time(Some(1.0));
            }
            let mut r = Rag::new(126.0, 7).with_transport(&c);
            r.dip(0.6, c.now_min(), c.tally_mut());
            let start = r.solvent_mm3;
            if hand {
                c.clock_hand_min();
            }
            c.wait(3.0);
            r.age_transport(&c);
            assert!((r.solvent_mm3 / start - 0.5).abs() < 1e-8);
            let previous = r.transport;
            r.age_transport(&c);
            assert_eq!(previous, r.transport);
        }
    }

    #[test]
    fn drying_a_wet_dirty_face_is_independent_of_wait_partition() {
        let mut c = canvas(0.7, 0.0);
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        r.dip(0.6, c.now_min(), c.tally_mut());
        let cloth = r.transport.as_mut().unwrap();
        for f in &mut cloth.faces {
            f.cells[0].mobile.add(Material::picked(
                1.0,
                10.0,
                [0.1; LAT],
                [0.8, 0.7, 1.0, 0.0, 1.0],
                0.0,
            ));
        }
        let mut a = *cloth;
        let mut b = *cloth;
        a.advance(240.0);
        b.advance(60.0);
        b.advance(120.0);
        b.advance(60.0);
        close(a.inventory(), b.inventory());
        assert!((a.mobile_mm3() - b.mobile_mm3()).abs() < 1e-12);
        assert!((a.evaporated_solvent_mm3 - b.evaporated_solvent_mm3).abs() < 1e-12);
    }

    #[test]
    fn a_wait_before_refolding_ages_the_old_exposed_face_in_both_clock_modes() {
        for hand in [false, true] {
            let mut c = canvas(0.7, 0.0);
            if hand {
                c.set_hand_time(Some(1.0));
            }
            let mut r = Rag::new(126.0, 7).with_transport(&c);
            r.dip(1.0, c.now_min(), c.tally_mut());
            let original_solvent = r.solvent_mm3;
            let (mut ca, mut cb, mut a, mut b) = (c.clone(), c.clone(), r, r);
            ca.wait(3.0);
            cb.wait(3.0);
            a.refold_at(ca.now_min(), ca.tally_mut());
            a.age_transport(&ca);
            b.age_transport(&cb);
            b.refold_at(cb.now_min(), cb.tally_mut());
            b.age_transport(&cb);
            assert_eq!(a.transport, b.transport);
            let expected = original_solvent
                * 0.5
                * (-pace::REFOLD * std::f64::consts::LN_2 / EXPOSED_SOLVENT_HALF_SECS).exp();
            assert!((a.solvent_mm3 - expected).abs() < 1e-8);
            assert_eq!(a.transport.as_ref().unwrap().active, 1);
        }
    }

    #[test]
    fn solvent_only_puddles_are_absorbed_even_when_paint_capacity_is_full() {
        let mut c = canvas(0.2, 0.0);
        c.wet.vol.fill(0.0);
        let mut donor = Rag::new(126.0, 7).with_transport(&c);
        donor.dip(1.0, c.now_min(), c.tally_mut());
        c.rag_blot(&mut donor, 500.0, 500.0, 0.9, 41);
        let before = c.solvent_total();
        assert!(before > 0.0);
        for full in [false, true] {
            let mut c = c.clone();
            let mut receiver = Rag::new(126.0, 8).with_transport(&c);
            if full {
                let cloth = receiver.transport.as_mut().unwrap();
                let cap = cloth.face_capacity_mm3 / CELLS as f64;
                for cell in &mut cloth.faces[0].cells {
                    cell.bound =
                        Material::picked(cap, 0.0, [0.1; LAT], [0.8, 0.7, 1.0, 0.0, 1.0], 0.2);
                }
            }
            let initial = inventory(&c, &receiver);
            let paint_before = receiver.transport.as_ref().unwrap().inventory().paint_mm3;
            c.rag_blot(&mut receiver, 500.0, 500.0, 0.9, 41);
            close(initial, inventory(&c, &receiver));
            assert!(c.solvent_total() < before);
            assert!(receiver.damp > 0.0 && receiver.solvent_mm3 > 0.0);
            let cloth = receiver.transport.as_ref().unwrap();
            assert!(cloth.solvent_absorbed_mm3 > 0.0);
            assert_eq!(cloth.inventory().paint_mm3, paint_before);
            let cap = cloth.face_capacity_mm3 / CELLS as f64;
            assert!(
                cloth.faces[0]
                    .cells
                    .iter()
                    .all(|cell| cell.solvent() <= cap + 1e-6)
            );
        }
    }

    #[test]
    fn nonzero_cure_moments_survive_mechanical_exchange_binding_and_disposal() {
        let mut c = canvas(0.4, 0.0);
        c.wet.clock.px = vec![crate::drying::Px::FRESH; c.wet.vol.len()];
        for (i, p) in c.wet.clock.px.iter_mut().enumerate() {
            p.cure = if i % 96 < 24 { 0.12 } else { 0.28 };
            p.seen = c.wet.vol[i];
        }
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        let initial = inventory(&c, &r);
        assert!(initial.cure_moment > 0.0);
        r.dip(0.6, c.now_min(), c.tally_mut());
        c.rag_wipe(&mut r, &[(150.0, 500.0), (850.0, 500.0)], &[0.6], 41);
        close(initial, inventory(&c, &r));
        assert!(r.transport.as_ref().unwrap().inventory().cure_moment > 0.0);
        for _ in 0..FACE_COUNT {
            r.refold_at(c.now_min(), c.tally_mut());
        }
        r.dip(0.95, c.now_min(), c.tally_mut());
        c.rag_wipe(&mut r, &[(650.0, 700.0), (850.0, 700.0)], &[0.4], 41);
        close(initial, inventory(&c, &r));
        r.transport.as_mut().unwrap().discard();
        close(initial, inventory(&c, &r));
        assert!(r.transport.as_ref().unwrap().waste.cure_moment > 0.0);
    }

    #[test]
    fn solvent_only_waits_have_an_independent_evaporation_and_trace_ledger() {
        let mut c = canvas(0.2, 0.0);
        c.wet.vol.fill(0.0);
        let mut r = Rag::new(126.0, 7).with_transport(&c);
        r.dip(1.0, c.now_min(), c.tally_mut());
        c.rag_blot(&mut r, 500.0, 500.0, 0.9, 41);
        let unit = (c.mm_per_unit() as f64).powi(2) * COAT_UM as f64 / 1000.0;
        let initial = c.solvent_total() * unit;
        let protected = (c.pixels().to_vec(), c.surface_um().to_vec());
        c.wait(2.0);
        let first = c.rag_solvent_ledger().unwrap();
        assert!(first.evaporated_mm3 > 0.0 && c.solvent_total() > 0.0);
        c.wait(2.0);
        assert!(c.rag_solvent_ledger().unwrap().evaporated_mm3 > first.evaporated_mm3);
        c.wait(60.0);
        let loss = c.rag_solvent_ledger().unwrap();
        assert_eq!(c.solvent_total(), 0.0);
        assert!(loss.discarded_trace_mm3 > 0.0);
        assert!(
            (loss.evaporated_mm3 + loss.discarded_trace_mm3 - initial).abs()
                < 2e-6 * initial.max(1.0)
        );
        assert_eq!(protected.0, c.pixels());
        assert_eq!(protected.1, c.surface_um());
    }
}
