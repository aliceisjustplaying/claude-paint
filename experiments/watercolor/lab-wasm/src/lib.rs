// Copyright (c) 2026 Ryan McCarty. MIT License.
// Port of uxpreview/watercolor-lab a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../../THIRD_PARTY_NOTICES.md.
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use watercolor_lab_core::{
    brush::Stamp, gpu::GpuSimulation, paper::PaperKind, pigments::Pigment, Simulation,
};

#[wasm_bindgen]
pub struct WasmSimulation {
    sim: GpuSimulation,
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

#[wasm_bindgen]
impl WasmSimulation {
    pub async fn create(
        canvas: web_sys::HtmlCanvasElement,
        width: usize,
        height: usize,
        kind: &str,
        seed: u32,
    ) -> Result<Self, JsValue> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(js_error)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or_else(|| JsValue::from_str("WebGPU is unavailable"))?;
        let sim = GpuSimulation::from_adapter(
            &adapter,
            &Simulation::new(width, height, paper_kind(kind)?, seed),
        )
        .await
        .map_err(js_error)?;
        if !surface
            .get_capabilities(&adapter)
            .formats
            .contains(&wgpu::TextureFormat::Bgra8Unorm)
        {
            return Err(JsValue::from_str("WebGPU canvas does not support BGRA8"));
        }
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: canvas.width().max(1),
            height: canvas.height().max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&sim.device, &config);
        Ok(Self {
            sim,
            canvas,
            surface,
            config,
        })
    }

    pub fn set_paper(&mut self, kind: &str, clear: bool, seed: u32) -> Result<(), JsValue> {
        self.sim.set_paper(paper_kind(kind)?, clear, seed);
        Ok(())
    }

    pub fn configure(&mut self, params: &str, forever_wet: bool) -> Result<(), JsValue> {
        self.sim.params = serde_json::from_str(params).map_err(js_error)?;
        self.sim.forever_wet = forever_wet;
        Ok(())
    }

    pub fn reset_state(&mut self) {
        self.sim.reset_state();
    }

    pub fn splat(&mut self, values: &[f64], pigment: &str) -> Result<(), JsValue> {
        let pigment: Option<Pigment> = serde_json::from_str(pigment).map_err(js_error)?;
        self.sim.splat(&stamps(values), pigment.as_ref());
        Ok(())
    }

    pub fn add_salt(&mut self, x: f64, y: f64, radius: f64, seed: u32) {
        self.sim.add_salt(x, y, radius, seed);
    }
    pub fn add_scrub(&mut self, values: &[f64]) {
        self.sim.add_scrub(&stamps(values));
    }
    pub fn dry_fast(&mut self) {
        self.sim.dry_fast();
    }
    pub fn update(&mut self) {
        self.sim.update();
    }
    pub fn step_many(&mut self, steps: usize, evap_mul: f32) {
        self.sim.step_many(steps, evap_mul);
    }

    pub fn render(&mut self, backlight: f32, field: u8) -> Result<(), JsValue> {
        let width = self.canvas.width().max(1);
        let height = self.canvas.height().max(1);
        if width != self.config.width || height != self.config.height {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.sim.device, &self.config);
        }
        let frame = self.surface.get_current_texture().map_err(js_error)?;
        self.sim.render(
            &frame.texture.create_view(&Default::default()),
            height as usize,
            backlight,
            field,
        );
        frame.present();
        Ok(())
    }

    pub fn snapshot(&mut self) {
        self.sim.snapshot();
    }
    pub fn undo(&mut self) -> bool {
        self.sim.undo()
    }
    pub fn clear_sheet(&mut self) {
        self.sim.clear_sheet();
    }

    pub fn probe(&self, x: f64, y: f64, radius: f64) -> js_sys::Promise {
        let read = self.sim.probe(x, y, radius);
        wasm_bindgen_futures::future_to_promise(async move {
            let p = read.await.map_err(js_error)?;
            Ok(js_sys::Float64Array::from(&[p.dried, p.workable, p.wet][..]).into())
        })
    }

    pub fn serialize_dried(&self) -> js_sys::Promise {
        let read = self.sim.read_field(8);
        wasm_bindgen_futures::future_to_promise(async move {
            let data: Vec<f32> = read
                .await
                .map_err(js_error)?
                .into_iter()
                .flatten()
                .collect();
            Ok(js_sys::Float32Array::from(data.as_slice()).into())
        })
    }

    pub fn restore_dried(&mut self, data: &[f32]) {
        self.sim.restore_dried(data);
    }
}

fn js_error(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}

fn paper_kind(kind: &str) -> Result<PaperKind, JsValue> {
    match kind {
        "hot-press" => Ok(PaperKind::HotPress),
        "cold-press" => Ok(PaperKind::ColdPress),
        "rough" => Ok(PaperKind::Rough),
        _ => Err(JsValue::from_str("Unknown paper kind")),
    }
}

fn stamps(values: &[f64]) -> Vec<Stamp> {
    values
        .chunks_exact(6)
        .map(|s| Stamp {
            x: s[0],
            y: s[1],
            radius: s[2],
            water: s[3],
            pigment: s[4],
            dryness: s[5],
        })
        .collect()
}
