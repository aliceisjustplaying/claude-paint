//! Paint attached to material locations on the active cloth face. All stored
//! volumes are mm³, independent of canvas resolution. Absorbed paint remains
//! in `held`; only still-wet surface paint can return to the canvas.
use super::{Pool, CAP_UM, COAT_UM, SMEAR, SOAK, cloth};
use crate::{drying, wet::{Latent, Prop}};

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Cell {
    pub paint: Pool,
    held: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Face {
    pub cells: Vec<Cell>,
    area: f32,
    at_min: f64,
}

impl Face {
    pub fn new(width_mm: f32, soaked: f32, now: f64) -> Self {
        let area = width_mm * width_mm / cloth::CELLS as f32;
        let held = soaked * area * CAP_UM / 1000.0;
        Self { cells: vec![Cell { held, ..Cell::default() }; cloth::CELLS], area, at_min: now }
    }

    pub fn age(&mut self, now: f64) {
        let elapsed = (now - self.at_min).max(0.0) as f32;
        for cell in &mut self.cells {
            let p = &mut cell.paint;
            if p.vol <= 0.0 { continue; }
            let coats = p.vol / self.area * 1000.0 / COAT_UM;
            p.cure += elapsed * drying::rate(coats, p.hide[1], p.hide[2]);
            // Set paint is retained in the cloth; fresh pickup cannot revive it.
            if p.cure >= drying::GEL { *p = Pool::default(); }
        }
        self.at_min = self.at_min.max(now);
    }

    pub fn thirst(&self, weights: [(usize, f32); 4]) -> f32 {
        let capacity = self.area * CAP_UM / 1000.0;
        weights.iter().map(|&(i, w)| w * (1.0 - (self.cells[i].held / capacity).clamp(0.0, 1.0).powi(2))).sum()
    }

    pub fn budgets(&self, exposure: &[f64]) -> Vec<f64> {
        self.cells.iter().zip(exposure).map(|(cell, &area)| {
            cell.paint.vol as f64 * drying::fluid(cell.paint.cure) as f64
                * (1.0 - (-SMEAR as f64 * area / self.area as f64).exp())
        }).collect()
    }

    pub fn finish(&mut self, picked: &[Pool], deposited: &[f64], travel_widths: f32) {
        // Use physical travel, not stamp count, for soaking into the fibers.
        let surface_left = (1.0 - SOAK).powf(4.0 * travel_widths);
        for ((cell, got), &out) in self.cells.iter_mut().zip(picked).zip(deposited) {
            cell.paint.vol = (cell.paint.vol - out as f32).max(0.0) * surface_left;
            cell.held = (cell.held + got.vol - out as f32).max(0.0);
            mix(&mut cell.paint, got.vol, &got.lat, &got.hide, got.cure);
        }
    }
}

pub(super) fn mix(p: &mut Pool, volume: f32, lat: &Latent, hide: &Prop, cure: f32) {
    if volume <= 0.0 { return; }
    let total = p.vol + volume;
    let a = volume / total;
    for k in 0..p.lat.len() { p.lat[k] += (lat[k] - p.lat[k]) * a; }
    for k in 0..3 { p.hide[k] += (hide[k] - p.hide[k]) * a; }
    p.cure += (cure - p.cure) * a;
    p.vol = total;
}
