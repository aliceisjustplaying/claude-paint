// Copyright (c) 2026 Ryan McCarty. MIT License.
// Port of uxpreview/watercolor-lab a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../../THIRD_PARTY_NOTICES.md.
use crate::{
    paper::PaperKind,
    simulation::{Pixel, Simulation},
    Params,
};
use std::borrow::Cow;
use wgpu::util::DeviceExt;

include!("shaders/passes.rs");

const VERTEX: &str = r#"
#version 450
layout(location=0) out vec2 vUV;
void main() {
    vUV = vec2(float((gl_VertexIndex << 1) & 2), float(gl_VertexIndex & 2));
    gl_Position = vec4(vUV.x * 2.0 - 1.0, 1.0 - vUV.y * 2.0, 0.0, 1.0);
}
"#;

const SPLAT_VERTEX: &str = r#"
#version 450
layout(set=0,binding=0,std140) uniform Settings { vec4 q[32]; };
layout(location=0) in vec4 aStamp;
layout(location=1) in vec4 aPaint;
layout(location=0) out vec2 vLocal;
layout(location=1) out vec2 vStampUV;
layout(location=2) flat out vec4 vStamp;
layout(location=3) flat out vec4 vPaint;
void main() {
    vec2 corners[6] = vec2[6](vec2(-1,-1),vec2(1,-1),vec2(-1,1),vec2(-1,1),vec2(1,-1),vec2(1,1));
    vLocal = corners[gl_VertexIndex];
    vStamp = aStamp;
    vPaint = aPaint;
    vStampUV = (aStamp.xy + vLocal * aStamp.z) / q[0].zw;
    gl_Position = vec4(vStampUV.x * 2.0 - 1.0, 1.0 - vStampUV.y * 2.0, 0.0, 1.0);
}
"#;

struct Target {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

struct Pass {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    inputs: Vec<usize>,
}

pub struct GpuSimulation {
    pub width: usize,
    pub height: usize,
    pub params: Params,
    pub forever_wet: bool,
    pub fast_dry_steps: i32,
    pub adapter_info: wgpu::AdapterInfo,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    fields: Vec<[Target; 2]>,
    current: [usize; 11],
    passes: Vec<Pass>,
    uniforms: wgpu::Buffer,
    sampler: wgpu::Sampler,
    undo: Vec<Vec<Target>>,
}

impl GpuSimulation {
    pub async fn new(
        width: usize,
        height: usize,
        kind: PaperKind,
        seed: u32,
    ) -> Result<Self, String> {
        Self::from_state(&Simulation::new(width, height, kind, seed)).await
    }

    pub async fn from_state(state: &Simulation) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .filter(|adapter| adapter.get_info().device_type != wgpu::DeviceType::Cpu)
            .ok_or("No GPU adapter")?;
        Self::from_adapter(&adapter, state).await
    }

    pub async fn from_adapter(adapter: &wgpu::Adapter, state: &Simulation) -> Result<Self, String> {
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Watercolor Lab"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
        let fields = (0..11)
            .map(|_| {
                std::array::from_fn(|_| {
                    Self::target(
                        &device,
                        state.width,
                        state.height,
                        wgpu::TextureFormat::Rgba16Float,
                    )
                })
            })
            .collect();
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 512,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut sim = Self {
            width: state.width,
            height: state.height,
            params: state.params,
            forever_wet: state.forever_wet,
            fast_dry_steps: state.fast_dry_steps,
            adapter_info: adapter.get_info(),
            device,
            queue,
            fields,
            current: [0; 11],
            passes: Vec::new(),
            uniforms,
            sampler,
            undo: Vec::new(),
        };
        for (index, (name, source, inputs)) in SOURCES.iter().enumerate() {
            let outputs = match *name {
                "MOVE_PIGMENT_FRAG" | "TRANSFER_SUS_FRAG" | "DRY_CLEAR_SUS_FRAG" => 3,
                "TRANSFER_DEP_FRAG" | "DRY_CLEAR_DEP_FRAG" => 2,
                "SPLAT_FRAG" => 4,
                _ => 1,
            };
            sim.passes
                .push(sim.create_pass(name, source, inputs, outputs, index));
        }
        sim.upload(state);
        Ok(sim)
    }

    fn target(
        device: &wgpu::Device,
        width: usize,
        height: usize,
        format: wgpu::TextureFormat,
    ) -> Target {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Target { texture, view }
    }

    fn create_pass(
        &self,
        name: &str,
        source: &str,
        inputs: &[usize],
        outputs: usize,
        index: usize,
    ) -> Pass {
        let mut entries = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];
        if !inputs.is_empty() {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            });
        }
        for i in 0..inputs.len() {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32 + 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }
        let layout = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(name),
                entries: &entries,
            });
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(name),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let stamp = (10..13).contains(&index);
        let display = index >= 13;
        let vertex_source = if stamp {
            SPLAT_VERTEX.to_string()
        } else if display {
            VERTEX
                .replace(
                    "gl_Position =",
                    "vec2 screenUV = vUV; vUV.y = 1.0 - vUV.y; gl_Position =",
                )
                .replace("vec4(vUV.x", "vec4(screenUV.x")
                .replace("- vUV.y *", "- screenUV.y *")
        } else {
            VERTEX.to_string()
        };
        let vertex = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Glsl {
                    shader: Cow::Owned(vertex_source),
                    stage: wgpu::naga::ShaderStage::Vertex,
                    defines: Default::default(),
                },
            });
        let source = source
            .replacen(
                "#version 450",
                "#version 450\n#define texture(TEX, UV) textureLod(TEX, UV, 0.0)",
                1,
            )
            .replace(
                "hash12(gl_FragCoord.xy)",
                "hash12(vec2(gl_FragCoord.x, q[10].x - gl_FragCoord.y))",
            );
        let fragment = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(name),
                source: wgpu::ShaderSource::Glsl {
                    shader: Cow::Owned(source),
                    stage: wgpu::naga::ShaderStage::Fragment,
                    defines: Default::default(),
                },
            });
        let targets = vec![
            Some(wgpu::ColorTargetState {
                format: if display {
                    wgpu::TextureFormat::Bgra8Unorm
                } else {
                    wgpu::TextureFormat::Rgba16Float
                },
                blend: if stamp {
                    Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    })
                } else {
                    None
                },
                write_mask: wgpu::ColorWrites::ALL,
            });
            outputs
        ];
        let attributes = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4];
        let buffers = [wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &attributes,
        }];
        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex,
                    entry_point: Some("main"),
                    buffers: if stamp { &buffers } else { &[] },
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment,
                    entry_point: Some("main"),
                    targets: &targets,
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        Pass {
            pipeline,
            layout,
            inputs: inputs.to_vec(),
        }
    }

    pub fn upload(&mut self, state: &Simulation) {
        for (i, data) in [
            &state.flow.read,
            &state.flux.read,
            &state.sus_k.read,
            &state.sus_s.read,
            &state.sus_p.read,
            &state.dep_k.read,
            &state.dep_s.read,
            &state.cap.read,
            &state.dried.read,
            &state.blur_w,
            &state.paper,
        ]
        .into_iter()
        .enumerate()
        {
            let bytes: Vec<u16> = data
                .iter()
                .flat_map(|p| p.map(|v| half::f16::from_f32(v).to_bits()))
                .collect();
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.fields[i][self.current[i]].texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                bytemuck::cast_slice(&bytes),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.width as u32 * 8),
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width: self.width as u32,
                    height: self.height as u32,
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    fn settings(&self, evap: f32) -> [f32; 128] {
        let p = self.params;
        let mut data = [0.0; 128];
        data[..24].copy_from_slice(&[
            1.0 / self.width as f32,
            1.0 / self.height as f32,
            self.width as f32,
            self.height as f32,
            p.gravity,
            p.damp,
            p.slope,
            p.tension,
            p.salt_pull,
            p.evap,
            p.evap_edge,
            evap,
            p.absorb,
            p.vel_scale,
            p.advect,
            p.edge_drift,
            p.settle,
            p.gran_bias,
            p.lift,
            p.salt_lift,
            p.scrub_lift,
            p.edge_settle,
            p.cap_diff,
            p.cap_dry,
        ]);
        data[36] = p.zoom_grain;
        data
    }

    fn run(&self, encoder: &mut wgpu::CommandEncoder, pass: usize, outputs: &[usize]) {
        let views: Vec<_> = outputs
            .iter()
            .map(|i| &self.fields[*i][1 - self.current[*i]].view)
            .collect();
        self.draw(encoder, pass, &views, None, 1);
    }

    fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        index: usize,
        views: &[&wgpu::TextureView],
        stamps: Option<&wgpu::Buffer>,
        count: u32,
    ) {
        let is_stamp = stamps.is_some();
        let pass = &self.passes[index];
        let mut bindings = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: self.uniforms.as_entire_binding(),
        }];
        if !pass.inputs.is_empty() {
            bindings.push(wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            });
        }
        for (j, i) in pass.inputs.iter().enumerate() {
            bindings.push(wgpu::BindGroupEntry {
                binding: j as u32 + 2,
                resource: wgpu::BindingResource::TextureView(
                    &self.fields[*i][self.current[*i]].view,
                ),
            });
        }
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pass.layout,
            entries: &bindings,
        });
        let attachments: Vec<_> = views
            .iter()
            .map(|view| {
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if is_stamp {
                            wgpu::LoadOp::Load
                        } else {
                            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })
            })
            .collect();
        let mut render = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render.set_pipeline(&pass.pipeline);
        render.set_bind_group(0, &group, &[]);
        if let Some(buffer) = stamps {
            render.set_vertex_buffer(0, buffer.slice(..));
        }
        render.draw(0..if is_stamp { 6 } else { 3 }, 0..count);
    }

    pub fn substep(&mut self, evap: f32) {
        self.step_many(1, evap);
    }

    pub fn step_many(&mut self, count: usize, evap: f32) {
        self.queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::cast_slice(&self.settings(evap)),
        );
        for start in (0..count).step_by(32) {
            let mut encoder = self.device.create_command_encoder(&Default::default());
            for _ in start..(start + 32).min(count) {
                for (pass, outputs, swap) in [
                    (0, &[9][..], true),
                    (1, &[1][..], true),
                    (2, &[0][..], true),
                    (3, &[2, 3, 4][..], true),
                    (5, &[5, 6][..], false),
                    (4, &[2, 3, 4][..], false),
                ] {
                    self.run(&mut encoder, pass, outputs);
                    if swap {
                        for i in outputs {
                            self.current[*i] = 1 - self.current[*i];
                        }
                    }
                }
                for i in [2, 3, 4, 5, 6] {
                    self.current[i] = 1 - self.current[i];
                }
                self.run(&mut encoder, 6, &[7]);
                self.current[7] = 1 - self.current[7];
                self.run(&mut encoder, 7, &[8]);
                self.current[8] = 1 - self.current[8];
                self.run(&mut encoder, 8, &[2, 3, 4]);
                self.run(&mut encoder, 9, &[5, 6]);
                for i in [2, 3, 4, 5, 6] {
                    self.current[i] = 1 - self.current[i];
                }
            }
            self.queue.submit([encoder.finish()]);
        }
    }

    pub fn update(&mut self) {
        let count = self.params.substeps + if self.fast_dry_steps > 0 { 13 } else { 0 };
        self.step_many(
            count,
            if self.fast_dry_steps > 0 {
                28.0
            } else if self.forever_wet {
                0.0
            } else {
                1.0
            },
        );
        if self.fast_dry_steps > 0 {
            self.fast_dry_steps -= count as i32;
        }
    }

    pub fn splat(
        &mut self,
        stamps: &[crate::brush::Stamp],
        pigment: Option<&crate::pigments::Pigment>,
    ) {
        self.splat_water_profile(stamps, pigment, false);
    }

    pub fn splat_water_profile(
        &mut self,
        stamps: &[crate::brush::Stamp],
        pigment: Option<&crate::pigments::Pigment>,
        spread_water: bool,
    ) {
        if stamps.is_empty() {
            return;
        }
        let mut settings = self.settings(1.0);
        settings[37] = if spread_water { 1.0 } else { 0.0 };
        if let Some(p) = pigment {
            settings[24..27].copy_from_slice(&p.k);
            settings[28..31].copy_from_slice(&p.s);
            settings[32..35].copy_from_slice(&[p.density, p.granulation, p.staining]);
        } else {
            settings[32..35].copy_from_slice(&[0.4, 0.0, 0.3]);
        }
        let instances: Vec<[f32; 8]> = stamps
            .iter()
            .take(4096)
            .enumerate()
            .map(|(i, s)| {
                [
                    s.x as f32,
                    (self.height as f64 - s.y) as f32,
                    s.radius as f32,
                    (s.water * 0.055) as f32,
                    if pigment.is_some() {
                        (s.pigment * 0.13) as f32
                    } else {
                        0.0
                    },
                    s.dryness as f32,
                    i as f32,
                    0.0,
                ]
            })
            .collect();
        self.stamp(10, &[0, 2, 3, 4], &settings, &instances);
    }

    pub fn add_scrub(&mut self, stamps: &[crate::brush::Stamp]) {
        let instances: Vec<[f32; 8]> = stamps
            .iter()
            .take(4096)
            .map(|s| {
                [
                    s.x as f32,
                    (self.height as f64 - s.y) as f32,
                    s.radius as f32,
                    0.0,
                    0.35,
                    0.0,
                    0.0,
                    0.0,
                ]
            })
            .collect();
        self.stamp(12, &[7], &self.settings(1.0), &instances);
    }

    pub fn add_salt(&mut self, x: f64, y: f64, radius: f64, seed: u32) {
        let mut state = seed;
        let mut rand = || {
            state = state.wrapping_add(0x6d2b79f5);
            let mut t = state;
            t = (t ^ (t >> 15)).wrapping_mul(t | 1);
            t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
            (t ^ (t >> 14)) as f64 / 4294967296.0
        };
        let count = 18 + (rand() * 12.0).floor() as usize;
        let instances: Vec<[f32; 8]> = (0..count)
            .map(|i| {
                let angle = rand() * std::f64::consts::TAU;
                let dist = rand().sqrt() * radius;
                [
                    (x + angle.cos() * dist) as f32,
                    (self.height as f64 - y - angle.sin() * dist) as f32,
                    (4.0 + rand() * 4.5) as f32,
                    0.0,
                    (0.35 + rand() * 0.5) as f32,
                    0.0,
                    i as f32,
                    0.0,
                ]
            })
            .collect();
        self.stamp(11, &[7], &self.settings(1.0), &instances);
    }

    fn stamp(&self, pass: usize, outputs: &[usize], settings: &[f32; 128], instances: &[[f32; 8]]) {
        if instances.is_empty() {
            return;
        }
        self.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(settings));
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(instances),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let views: Vec<_> = outputs
            .iter()
            .map(|i| &self.fields[*i][self.current[*i]].view)
            .collect();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.draw(
            &mut encoder,
            pass,
            &views,
            Some(&buffer),
            instances.len() as u32,
        );
        self.queue.submit([encoder.finish()]);
    }

    pub fn render(&self, view: &wgpu::TextureView, height: usize, backlight: f32, xray: u8) {
        let mut settings = self.settings(1.0);
        settings[35] = backlight;
        settings[37] = xray as f32;
        settings[38] = [1.0, 0.12, 0.6, 0.15, 0.06, 0.03, 1.0][(xray as usize).min(6)];
        settings[40] = height as f32;
        self.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&settings));
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.draw(
            &mut encoder,
            if xray == 0 { 13 } else { 14 },
            &[view],
            None,
            1,
        );
        self.queue.submit([encoder.finish()]);
    }

    pub async fn render_rgba(&self, backlight: f32, xray: u8) -> Result<Vec<u8>, String> {
        let target = Self::target(
            &self.device,
            self.width,
            self.height,
            wgpu::TextureFormat::Bgra8Unorm,
        );
        self.render(&target.view, self.height, backlight, xray);
        let bytes = self.read_texture(&target.texture, 4).await?;
        Ok(bytes
            .chunks_exact(4)
            .flat_map(|p| [p[2], p[1], p[0], p[3]])
            .collect())
    }

    fn clear_field(&self, encoder: &mut wgpu::CommandEncoder, field: usize, color: wgpu::Color) {
        for target in &self.fields[field] {
            let attachments = [Some(wgpu::RenderPassColorAttachment {
                view: &target.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })];
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
    }

    pub fn reset_state(&mut self) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        for field in 0..9 {
            let color = if field == 8 {
                let p = crate::km::PAPER_REFLECTANCE;
                wgpu::Color {
                    r: p[0] as f64,
                    g: p[1] as f64,
                    b: p[2] as f64,
                    a: 0.0,
                }
            } else {
                wgpu::Color::TRANSPARENT
            };
            self.clear_field(&mut encoder, field, color);
        }
        self.queue.submit([encoder.finish()]);
        self.undo.clear();
        self.fast_dry_steps = 0;
    }

    pub fn set_paper(&mut self, kind: PaperKind, clear: bool, seed: u32) {
        let data: Vec<_> = crate::paper::generate_paper(self.width, self.height, kind, seed)
            .into_iter()
            .map(crate::simulation::quantize)
            .collect();
        self.write_field(10, &data);
        if clear {
            self.reset_state();
        }
    }

    pub fn write_field(&self, field: usize, data: &[Pixel]) {
        let bytes: Vec<u16> = data
            .iter()
            .flat_map(|p| p.map(|v| half::f16::from_f32(v).to_bits()))
            .collect();
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.fields[field][self.current[field]].texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&bytes),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.width as u32 * 8),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: self.width as u32,
                height: self.height as u32,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn restore_dried(&mut self, data: &[f32]) {
        if data.len() != self.width * self.height * 4 {
            return;
        }
        self.reset_state();
        let pixels: Vec<Pixel> = data
            .chunks_exact(4)
            .map(|p| crate::simulation::quantize([p[0], p[1], p[2], p[3]]))
            .collect();
        self.write_field(8, &pixels);
    }

    pub fn dry_fast(&mut self) {
        self.fast_dry_steps = 480;
    }

    pub fn snapshot(&mut self) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let saved: Vec<_> = [0, 2, 3, 4, 5, 6, 7, 8]
            .iter()
            .map(|i| {
                let target = Self::target(
                    &self.device,
                    self.width,
                    self.height,
                    wgpu::TextureFormat::Rgba16Float,
                );
                encoder.copy_texture_to_texture(
                    self.fields[*i][self.current[*i]].texture.as_image_copy(),
                    target.texture.as_image_copy(),
                    self.extent(),
                );
                target
            })
            .collect();
        self.queue.submit([encoder.finish()]);
        self.undo.push(saved);
        if self.undo.len() > 3 {
            self.undo.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        let Some(saved) = self.undo.pop() else {
            return false;
        };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        for (i, target) in [0, 2, 3, 4, 5, 6, 7, 8].iter().zip(saved) {
            encoder.copy_texture_to_texture(
                target.texture.as_image_copy(),
                self.fields[*i][self.current[*i]].texture.as_image_copy(),
                self.extent(),
            );
        }
        self.clear_field(&mut encoder, 1, wgpu::Color::TRANSPARENT);
        self.queue.submit([encoder.finish()]);
        self.fast_dry_steps = 0;
        true
    }

    pub fn clear_sheet(&mut self) {
        self.snapshot();
        let undo = std::mem::take(&mut self.undo);
        self.reset_state();
        self.undo = undo;
    }

    fn extent(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width as u32,
            height: self.height as u32,
            depth_or_array_layers: 1,
        }
    }

    pub fn probe(
        &self,
        x: f64,
        y: f64,
        radius: f64,
    ) -> impl std::future::Future<Output = Result<crate::Probe, String>> + 'static {
        let r = radius.round().max(1.0) as isize;
        let cx = (x + 0.5).floor() as isize;
        let cy = (self.height as f64 - y + 0.5).floor() as isize;
        let x0 = (cx - r).max(0);
        let y0 = (cy - r).max(0);
        let x1 = (cx + r + 1).min(self.width as isize);
        let y1 = (cy + r + 1).min(self.height as isize);
        let reads: Vec<_> = if x1 > x0 && y1 > y0 {
            [(8, 0), (5, 1), (2, 1), (0, 2)]
                .into_iter()
                .map(|(field, destination)| (destination, self.read_field(field)))
                .collect()
        } else {
            Vec::new()
        };
        let width = self.width;
        async move {
            let mut probe = crate::Probe {
                dried: 0.0,
                workable: 0.0,
                wet: 0.0,
            };
            for (destination, read) in reads {
                let data = read.await?;
                let mut total = 0.0;
                for y in y0..y1 {
                    for x in x0..x1 {
                        total += data[y as usize * width + x as usize][3] as f64;
                    }
                }
                let mean = total / ((x1 - x0) * (y1 - y0)) as f64;
                match destination {
                    0 => probe.dried += mean,
                    1 => probe.workable += mean,
                    _ => probe.wet += mean,
                }
            }
            Ok(probe)
        }
    }

    pub fn read_field(
        &self,
        field: usize,
    ) -> impl std::future::Future<Output = Result<Vec<Pixel>, String>> + 'static {
        let read = self.read_texture(&self.fields[field][self.current[field]].texture, 8);
        async move {
            let bytes = read.await?;
            Ok(bytes
                .chunks_exact(8)
                .map(|pixel| {
                    std::array::from_fn(|i| {
                        half::f16::from_bits(u16::from_le_bytes([pixel[i * 2], pixel[i * 2 + 1]]))
                            .to_f32()
                    })
                })
                .collect())
        }
    }

    fn read_texture(
        &self,
        texture: &wgpu::Texture,
        bytes_per_pixel: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + 'static {
        let stride = (self.width * bytes_per_pixel).div_ceil(256) * 256;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (stride * self.height) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(stride as u32),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: self.width as u32,
                height: self.height as u32,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);
        let (sender, receiver) = futures_channel::oneshot::channel();
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let _ = sender.send(result);
            });
        #[cfg(not(target_arch = "wasm32"))]
        self.device.poll(wgpu::Maintain::Wait);
        let width = self.width;
        let height = self.height;
        async move {
            receiver
                .await
                .map_err(|e| e.to_string())?
                .map_err(|e| e.to_string())?;
            let mapped = buffer.slice(..).get_mapped_range();
            let mut data = Vec::with_capacity(width * height * bytes_per_pixel);
            for row in mapped.chunks_exact(stride) {
                data.extend_from_slice(&row[..width * bytes_per_pixel]);
            }
            drop(mapped);
            buffer.unmap();
            Ok(data)
        }
    }

    pub async fn download(&self, state: &mut Simulation) -> Result<(), String> {
        for (i, pair) in [
            &mut state.flow,
            &mut state.flux,
            &mut state.sus_k,
            &mut state.sus_s,
            &mut state.sus_p,
            &mut state.dep_k,
            &mut state.dep_s,
            &mut state.cap,
            &mut state.dried,
        ]
        .into_iter()
        .enumerate()
        {
            pair.read = self.read_field(i).await?;
        }
        state.blur_w = self.read_field(9).await?;
        Ok(())
    }
}
