// Copyright (c) 2026 Ryan McCarty. MIT License.
// WASM API adapter for uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See ../lab-app/LICENSE for the full license. Simulation math lives in ../lab-core.

use wasm_bindgen::prelude::*;
use watercolor_lab_core::{brush::Stamp, paper::PaperKind, pigments::Pigment, simulation::Simulation};

#[wasm_bindgen]
pub struct WasmSimulation {
    sim: Simulation,
}

#[wasm_bindgen]
impl WasmSimulation {
    // Adapter for src/engine/simulation.ts:173-233 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, kind: &str, seed: u32) -> Result<Self, JsValue> {
        Ok(Self { sim: Simulation::new(width, height, paper_kind(kind)?, seed) })
    }

    // Adapter for src/engine/simulation.ts:239-252 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn set_paper(&mut self, kind: &str, clear: bool, seed: u32) -> Result<(), JsValue> {
        self.sim.set_paper(paper_kind(kind)?, clear, seed);
        Ok(())
    }

    // Adapter for src/engine/simulation.ts:51-100,400-409 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn configure(&mut self, params: &str, forever_wet: bool) -> Result<(), JsValue> {
        self.sim.params = serde_json::from_str(params).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.sim.forever_wet = forever_wet;
        Ok(())
    }

    // Adapter for src/engine/simulation.ts:254-265 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn reset_state(&mut self) { self.sim.reset_state(); }

    // Adapter for src/engine/simulation.ts:270-311 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn splat(&mut self, values: &[f64], pigment: &str) -> Result<(), JsValue> {
        let stamps = stamps(values);
        let pigment: Option<Pigment> = serde_json::from_str(pigment).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.sim.splat(&stamps, pigment.as_ref());
        Ok(())
    }

    // Adapter for src/engine/simulation.ts:316-354 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn add_salt(&mut self, x: f64, y: f64, radius: f64, seed: u32) { self.sim.add_salt(x, y, radius, seed); }

    // Adapter for src/engine/simulation.ts:359-387 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn add_scrub(&mut self, values: &[f64]) { self.sim.add_scrub(&stamps(values)); }

    // Adapter for src/engine/simulation.ts:393-395 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn dry_fast(&mut self) { self.sim.dry_fast(); }

    // Adapter for src/engine/simulation.ts:400-405 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn update(&mut self) { self.sim.update(); }

    // Adapter for src/engine/simulation.ts:408-410 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn step_many(&mut self, steps: usize, evap_mul: f32) { self.sim.step_many(steps, evap_mul); }

    // Display upload adapter for src/engine/simulation.ts:579-619 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn field(&self, name: &str) -> Result<Vec<f32>, JsValue> {
        let data = match name {
            "flow" => &self.sim.flow.read, "flux" => &self.sim.flux.read,
            "susK" => &self.sim.sus_k.read, "susS" => &self.sim.sus_s.read,
            "susP" => &self.sim.sus_p.read, "depK" => &self.sim.dep_k.read,
            "depS" => &self.sim.dep_s.read, "cap" => &self.sim.cap.read,
            "dried" => &self.sim.dried.read, "paper" => &self.sim.paper,
            _ => return Err(JsValue::from_str("Unknown state field")),
        };
        Ok(data.iter().flat_map(|pixel| pixel.iter().copied()).collect())
    }

    // Adapter for src/engine/simulation.ts:637-653 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn snapshot(&mut self) { self.sim.snapshot(); }

    // Adapter for src/engine/simulation.ts:655-664 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn undo(&mut self) -> bool { self.sim.undo() }

    // Adapter for src/engine/simulation.ts:666-676 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn clear_sheet(&mut self) { self.sim.clear_sheet(); }

    // Adapter for src/engine/simulation.ts:682-704 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn probe(&self, x: f64, y: f64, radius: f64) -> Vec<f64> {
        let p = self.sim.probe(x, y, radius);
        vec![p.dried, p.workable, p.wet]
    }

    // Adapter for src/engine/simulation.ts:731-740 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn restore_dried(&mut self, data: &[f32]) { self.sim.restore_dried(data); }
}

// Adapter for src/engine/paper.ts:21-32 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn paper_kind(kind: &str) -> Result<PaperKind, JsValue> {
    match kind {
        "hot-press" => Ok(PaperKind::HotPress),
        "cold-press" => Ok(PaperKind::ColdPress),
        "rough" => Ok(PaperKind::Rough),
        _ => Err(JsValue::from_str("Unknown paper kind")),
    }
}

// Adapter for src/engine/simulation.ts:270-288 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
fn stamps(values: &[f64]) -> Vec<Stamp> {
    values.chunks_exact(6).map(|s| Stamp { x: s[0], y: s[1], radius: s[2], water: s[3], pigment: s[4], dryness: s[5] }).collect()
}
