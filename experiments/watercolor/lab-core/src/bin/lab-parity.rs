// Mechanical validation of Watercolor Lab, copyright (c) 2026 Ryan McCarty, MIT.
// Source: uxpreview/watercolor-lab a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: experiments/watercolor/THIRD_PARTY_NOTICES.md.
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs};
use watercolor_lab_core::{
    brush::{tool, Brush, Stamp, ToolId},
    paper::PaperKind,
    pigments::{pigment_by_id, Pigment},
    Params, Simulation,
};

type Fields = BTreeMap<String, Vec<[f32; 4]>>;

#[derive(Deserialize)]
struct Reference {
    width: usize,
    height: usize,
    renderer: String,
    #[serde(rename = "subpixelBits")]
    subpixel_bits: u8,
    cases: Vec<Case>,
}
#[derive(Deserialize)]
struct Case {
    name: String,
    params: Params,
    initial: Fields,
    operations: Vec<Operation>,
    captures: Vec<Capture>,
    #[serde(rename = "final")]
    final_fields: Fields,
    renders: Vec<Render>,
}
#[derive(Deserialize)]
struct Render {
    backlight: f32,
    xray: u8,
    rgba: Vec<u8>,
}
#[derive(Deserialize)]
struct Operation {
    kind: String,
    #[serde(default)]
    evap_mul: f32,
    #[serde(default)]
    stamps: Vec<Stamp>,
    pigment: Option<Pigment>,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    radius: f64,
    #[serde(default)]
    seed: u32,
}
#[derive(Deserialize)]
struct Capture {
    pass: String,
    input: Fields,
    fields: Fields,
}

// Source: src/engine/simulation.ts:243–250,733–737 at the pinned commit.
fn upload(sim: &mut Simulation, fields: &Fields) {
    for (name, pair) in [
        ("flow", &mut sim.flow),
        ("flux", &mut sim.flux),
        ("sus_k", &mut sim.sus_k),
        ("sus_s", &mut sim.sus_s),
        ("sus_p", &mut sim.sus_p),
        ("dep_k", &mut sim.dep_k),
        ("dep_s", &mut sim.dep_s),
        ("cap", &mut sim.cap),
        ("dried", &mut sim.dried),
    ] {
        pair.read = fields[name].clone();
        pair.write = fields[name].clone();
    }
    sim.blur_w = fields["blur_w"].clone();
    sim.paper = fields["paper"].clone();
}

// Source: src/engine/simulation.ts:446–575 at the pinned commit, before-swap attachments.
fn capture<'a>(sim: &'a Simulation, pass: &str) -> BTreeMap<&'static str, &'a [[f32; 4]]> {
    BTreeMap::from([
        (
            "flow",
            if pass == "water" {
                sim.flow.write.as_slice()
            } else {
                sim.flow.read.as_slice()
            },
        ),
        (
            "flux",
            if pass == "flux" {
                sim.flux.write.as_slice()
            } else {
                sim.flux.read.as_slice()
            },
        ),
        (
            "sus_k",
            if matches!(pass, "move_pigment" | "transfer_sus" | "dry_clear_sus") {
                sim.sus_k.write.as_slice()
            } else {
                sim.sus_k.read.as_slice()
            },
        ),
        (
            "sus_s",
            if matches!(pass, "move_pigment" | "transfer_sus" | "dry_clear_sus") {
                sim.sus_s.write.as_slice()
            } else {
                sim.sus_s.read.as_slice()
            },
        ),
        (
            "sus_p",
            if matches!(pass, "move_pigment" | "transfer_sus" | "dry_clear_sus") {
                sim.sus_p.write.as_slice()
            } else {
                sim.sus_p.read.as_slice()
            },
        ),
        (
            "dep_k",
            if matches!(pass, "transfer_dep" | "dry_clear_dep") {
                sim.dep_k.write.as_slice()
            } else {
                sim.dep_k.read.as_slice()
            },
        ),
        (
            "dep_s",
            if matches!(pass, "transfer_dep" | "dry_clear_dep") {
                sim.dep_s.write.as_slice()
            } else {
                sim.dep_s.read.as_slice()
            },
        ),
        (
            "cap",
            if pass == "capillary" {
                sim.cap.write.as_slice()
            } else {
                sim.cap.read.as_slice()
            },
        ),
        (
            "dried",
            if pass == "dry_fold" {
                sim.dried.write.as_slice()
            } else {
                sim.dried.read.as_slice()
            },
        ),
        ("blur_w", sim.blur_w.as_slice()),
        ("paper", sim.paper.as_slice()),
    ])
}

// Validation metric for RGBA16F fields declared in src/engine/shaders.ts:10–23 at pinned commit.
fn differences(expected: &[[f32; 4]], actual: &[[f32; 4]], width: usize) -> Value {
    assert_eq!(expected.len(), actual.len());
    let mut max = 0.0_f64;
    let mut sum = 0.0_f64;
    let mut max_half_ulps = 0.0_f64;
    let mut over_tolerance = 0;
    let mut worst = json!(null);
    let mut exceeding = Vec::new();
    for (pixel, (a, b)) in expected.iter().zip(actual).enumerate() {
        for channel in 0..4 {
            let difference = (a[channel] as f64 - b[channel] as f64).abs();
            assert!(difference.is_finite(), "nonfinite parity value");
            let magnitude = (a[channel] as f64).abs().max((b[channel] as f64).abs());
            let half_ulp = if magnitude < 2.0_f64.powi(-14) {
                2.0_f64.powi(-24)
            } else {
                2.0_f64.powf(magnitude.log2().floor() - 10.0)
            };
            max_half_ulps = max_half_ulps.max(difference / half_ulp);
            let tolerance = 1e-5_f64.max(4.0 * half_ulp);
            if difference > tolerance {
                over_tolerance += 1;
                if exceeding.len() < 4 {
                    exceeding.push(json!({ "x": pixel % width, "y": pixel / width, "channel": channel, "original": a[channel], "rust": b[channel], "difference": difference, "tolerance": tolerance }));
                }
            }
            if difference > max {
                max = difference;
                worst = json!({ "x": pixel % width, "y": pixel / width, "channel": channel, "original": a[channel], "rust": b[channel] });
            }
            sum += difference;
        }
    }
    json!({ "max_abs": max, "mean_abs": sum / (expected.len() * 4) as f64, "max_half_ulps": max_half_ulps, "over_tolerance": over_tolerance, "worst": worst, "exceeding": exceeding })
}

// Conservation channels: src/engine/shaders.ts:10–23 at the pinned commit.
fn totals(fields: &Fields) -> Value {
    let water: f64 = fields["flow"]
        .iter()
        .zip(&fields["cap"])
        .map(|(f, c)| f[2] as f64 + c[0] as f64)
        .sum();
    let pigment: f64 = fields["sus_k"]
        .iter()
        .zip(&fields["dep_k"])
        .map(|(s, d)| s[3] as f64 + d[3] as f64)
        .sum();
    json!({ "standing_plus_capillary_water": water, "suspended_plus_deposited_pigment": pigment })
}

// Capture diagnostic channels/thresholds: src/engine/shaders.ts:10–23,396,623–625,650–652 at pinned commit.
fn stage_stats(sim: &Simulation) -> Value {
    let mut count = 0;
    let mut water_sum = 0.0_f64;
    let mut water_max = 0.0_f32;
    let mut wet_sum = 0.0_f64;
    let mut wet_max = 0.0_f32;
    let mut sheen_cells = 0;
    let mut damp_cells = 0;
    let mut cured_cells = 0;
    let live_pigment: f64 = sim
        .sus_k
        .read
        .iter()
        .zip(&sim.dep_k.read)
        .map(|(s, d)| s[3] as f64 + d[3] as f64)
        .sum();
    for i in 0..sim.width * sim.height {
        if sim.sus_k.read[i][3] + sim.dep_k.read[i][3] + sim.dried.read[i][3] <= 1e-4 {
            continue;
        }
        count += 1;
        let f = sim.flow.read[i];
        water_sum += f[2] as f64;
        water_max = water_max.max(f[2]);
        wet_sum += f[3] as f64;
        wet_max = wet_max.max(f[3]);
        if f[2] > 0.05 {
            sheen_cells += 1;
        }
        if f[2] <= 0.002 && f[3] >= 0.02 {
            damp_cells += 1;
        }
        if f[3] < 0.02 {
            cured_cells += 1;
        }
    }
    json!({ "painted_cells": count, "mean_surface_depth": water_sum / count.max(1) as f64, "max_surface_depth": water_max,
        "mean_flow_a": wet_sum / count.max(1) as f64, "max_flow_a": wet_max, "sheen_fraction": sheen_cells as f64 / count.max(1) as f64,
        "damp_fraction": damp_cells as f64 / count.max(1) as f64, "cured_fraction": cured_cells as f64 / count.max(1) as f64,
        "live_pigment_total": live_pigment,
        "thresholds": "source sheen starts at flow.b>0.05; flow.b<=0.002 permits wetness decay; dry-fold uses flow.a<0.02" })
}

// Matched script interpreter: scripts/scenes.mjs:56–72, src/ui/app.ts:1028–1037 at pinned commit.
fn capture_marks(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() < 4 {
        return Err("usage: lab-parity --marks MARKS.json OUTPUT_DIR [MARK_NAME...]".into());
    }
    let data: Value = serde_json::from_slice(&fs::read(&args[2])?)?;
    let width = data["width"].as_u64().unwrap() as usize;
    let height = data["height"].as_u64().unwrap() as usize;
    let seed = data["seed"].as_u64().unwrap() as u32;
    let out = std::path::Path::new(&args[3]);
    fs::create_dir_all(out)?;
    let mut timings = Vec::new();
    for mark in data["marks"].as_array().unwrap() {
        let name = mark["name"].as_str().unwrap();
        if args.len() > 4 && !args[4..].iter().any(|s| s == name) {
            continue;
        }
        let start = std::time::Instant::now();
        let mut sim = Simulation::new(width, height, PaperKind::ColdPress, seed);
        let mut tool_id = ToolId::Round;
        let mut pigment = pigment_by_id("french-ultramarine");
        let mut size = 0.42;
        let mut water = 0.55;
        let mut load = 0.65;
        let mut stages = Vec::new();
        let setup = json!({ "name": "setup", "program": mark["program"] });
        for stage in std::iter::once(&setup).chain(data["stages"].as_array().unwrap().iter()) {
            for op in stage["program"].as_array().unwrap() {
                let a = &op[1];
                match op[0].as_str().unwrap() {
                    "paper" => sim.set_paper(serde_json::from_value(a.clone())?, true, seed),
                    "tool" => tool_id = serde_json::from_value(a.clone())?,
                    "pig" => pigment = pigment_by_id(a.as_str().unwrap()),
                    "size" => size = a.as_f64().unwrap(),
                    "water" => water = a.as_f64().unwrap(),
                    "load" => load = a.as_f64().unwrap(),
                    "step" => sim.step_many(
                        a.as_u64().unwrap() as usize,
                        op.get(2).and_then(Value::as_f64).unwrap_or(1.0) as f32,
                    ),
                    "path" => {
                        let points = a.as_array().unwrap();
                        let scale = width as f64 / 1056.0;
                        let mut brush = Brush::begin(
                            tool(tool_id),
                            size,
                            water,
                            load,
                            points[0]["x"].as_f64().unwrap() * scale,
                            points[0]["y"].as_f64().unwrap() * scale,
                            12345,
                            scale,
                        );
                        for point in &points[1..] {
                            let stamps = brush.stroke_to(
                                point["x"].as_f64().unwrap() * scale,
                                point["y"].as_f64().unwrap() * scale,
                                0.55,
                            );
                            sim.splat(
                                &stamps,
                                if brush.tool.pigment > 0.0 {
                                    pigment.as_ref()
                                } else {
                                    None
                                },
                            );
                            sim.step_many(
                                op.get(2).and_then(Value::as_u64).unwrap_or(1) as usize,
                                1.0,
                            );
                        }
                    }
                    kind => return Err(format!("unsupported mark instruction {kind}").into()),
                }
            }
            let stage_name = stage["name"].as_str().unwrap();
            if stage_name == "setup" {
                let mut diagnostic = sim.clone();
                let before = stage_stats(&diagnostic)["live_pigment_total"]
                    .as_f64()
                    .unwrap();
                let mut after = before;
                diagnostic.substep_with_callback(1.0, |pass, sim| {
                    if pass == "move_pigment" {
                        after = sim
                            .sus_k
                            .write
                            .iter()
                            .zip(&sim.dep_k.read)
                            .map(|(s, d)| s[3] as f64 + d[3] as f64)
                            .sum();
                    }
                });
                fs::write(
                    out.join(format!("{name}-drift.json")),
                    serde_json::to_vec_pretty(
                        &json!({ "mark": name, "width": width, "height": height,
                    "just_laid_live_pigment": before, "first_advection": { "before": before, "after": after, "delta": after-before, "percent": 100.0*(after-before)/before },
                    "source": "src/engine/shaders.ts:192–194", "diagnostic_uses_clone_so_matched_capture_timing_is_unchanged": true }),
                    )?,
                )?;
                continue;
            }
            let rgba = sim.render_rgba(0.0, 0);
            let mut image = format!("P6\n{width} {height}\n255\n").into_bytes();
            image.extend(rgba.chunks_exact(4).flat_map(|p| p[..3].iter().copied()));
            fs::write(out.join(format!("{name}-{stage_name}.ppm")), image)?;
            stages.push(json!({ "name": stage_name, "elapsed_ms": start.elapsed().as_secs_f64() * 1000.0, "stats": stage_stats(&sim) }));
            fs::write(
                out.join(format!("{name}-timing.json")),
                serde_json::to_vec_pretty(
                    &json!({ "mark": name, "stages": stages, "complete": false }),
                )?,
            )?;
        }
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let timing =
            json!({ "mark": name, "elapsed_ms": elapsed_ms, "stages": stages, "complete": true });
        fs::write(
            out.join(format!("{name}-timing.json")),
            serde_json::to_vec_pretty(&timing)?,
        )?;
        println!("{name}: {elapsed_ms:.1} ms");
        timings.push(timing);
        fs::write(
            out.join("timings.json"),
            serde_json::to_vec_pretty(
                &json!({ "width": width, "height": height, "seed": seed, "view": "upstream render_rgba backlight=0 xray=0", "stages_are_fixed_step_samples": true, "timings": timings }),
            )?,
        )?;
    }
    Ok(())
}

// Source orchestration: src/engine/simulation.ts:408–410,446–575 at the pinned commit.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--marks") {
        return capture_marks(&args);
    }
    if args.len() != 3 {
        return Err("usage: lab-parity ORIGINAL_PARITY.json OUTPUT.json".into());
    }
    let reference: Reference = serde_json::from_slice(&fs::read(&args[1])?)?;
    let mut cases = Vec::new();
    let mut failures = 0;
    let mut isolated_failures = 0;
    for case in reference.cases {
        let mut sim = Simulation::new(reference.width, reference.height, PaperKind::ColdPress, 71);
        sim.raster_subpixel_bits = reference.subpixel_bits;
        sim.params = case.params;
        upload(&mut sim, &case.initial);
        let start = std::time::Instant::now();
        let mut pass_index = 0;
        let mut passes = Vec::new();
        for op in &case.operations {
            let mut compare = |pass: &str, sim: &Simulation| {
                let expected = &case.captures[pass_index];
                assert_eq!(expected.pass, pass);
                let fields: BTreeMap<_, _> = capture(sim, pass)
                    .into_iter()
                    .map(|(name, values)| {
                        let metric = differences(&expected.fields[name], values, reference.width);
                        failures += metric["over_tolerance"].as_u64().unwrap();
                        (name, metric)
                    })
                    .collect();
                let mut isolated =
                    Simulation::new(reference.width, reference.height, PaperKind::ColdPress, 71);
                isolated.raster_subpixel_bits = reference.subpixel_bits;
                isolated.params = case.params;
                upload(&mut isolated, &expected.input);
                match pass {
                    "blur_wet" => isolated.blur_wet(),
                    "flux" => isolated.flux_pass(),
                    "water" => isolated.water(op.evap_mul),
                    "move_pigment" => isolated.move_pigment(),
                    "transfer_dep" => isolated.transfer_dep(),
                    "transfer_sus" => isolated.transfer_sus(),
                    "capillary" => isolated.capillary(op.evap_mul),
                    "dry_fold" => isolated.dry_fold(),
                    "dry_clear_sus" => isolated.dry_clear_sus(),
                    "dry_clear_dep" => isolated.dry_clear_dep(),
                    "splat" => isolated.splat(&op.stamps, op.pigment.as_ref()),
                    "salt" => isolated.add_salt(op.x, op.y, op.radius, op.seed),
                    "scrub" => isolated.add_scrub(&op.stamps),
                    _ => panic!("unknown pass {pass}"),
                }
                let isolated_fields: BTreeMap<_, _> = capture(&isolated, pass)
                    .into_iter()
                    .map(|(name, values)| {
                        let metric = differences(&expected.fields[name], values, reference.width);
                        isolated_failures += metric["over_tolerance"].as_u64().unwrap();
                        (name, metric)
                    })
                    .collect();
                passes.push(json!({ "index": pass_index, "pass": pass, "fields": fields, "isolated_fields": isolated_fields }));
                pass_index += 1;
            };
            if op.kind == "substep" {
                sim.substep_with_callback(op.evap_mul, &mut compare);
            } else {
                match op.kind.as_str() {
                    "splat" => sim.splat(&op.stamps, op.pigment.as_ref()),
                    "salt" => sim.add_salt(op.x, op.y, op.radius, op.seed),
                    "scrub" => sim.add_scrub(&op.stamps),
                    _ => panic!("unknown operation {}", op.kind),
                }
                compare(&op.kind, &sim);
            }
        }
        assert_eq!(pass_index, case.captures.len());
        let mut renders = Vec::new();
        let mut isolated_render =
            Simulation::new(reference.width, reference.height, PaperKind::ColdPress, 71);
        isolated_render.raster_subpixel_bits = reference.subpixel_bits;
        isolated_render.params = case.params;
        upload(&mut isolated_render, &case.final_fields);
        for render in &case.renders {
            let mut metrics = BTreeMap::new();
            for (name, state) in [("accumulated", &sim), ("isolated", &isolated_render)] {
                let rgba = state.render_rgba(render.backlight, render.xray);
                assert_eq!(rgba.len(), render.rgba.len());
                let differences: Vec<_> = rgba
                    .iter()
                    .zip(&render.rgba)
                    .map(|(a, b)| a.abs_diff(*b) as u64)
                    .collect();
                let max = differences.iter().copied().max().unwrap_or(0);
                let mean = differences.iter().sum::<u64>() as f64 / differences.len() as f64;
                let over_tolerance = differences.iter().filter(|d| **d > 2).count();
                if name == "isolated" {
                    isolated_failures += over_tolerance as u64;
                }
                metrics.insert(name, json!({ "max_byte_difference": max, "mean_byte_difference": mean, "over_tolerance": over_tolerance, "tolerance_bytes": 2 }));
            }
            renders.push(
                json!({ "backlight": render.backlight, "xray": render.xray, "metrics": metrics }),
            );
        }
        let mut result = json!({ "name": case.name, "elapsed_ms": start.elapsed().as_secs_f64() * 1000.0, "passes": passes });
        result["renders"] = json!(renders);
        if case.name == "closed-system" {
            let rust_final: Fields = capture(&sim, "")
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value.to_vec()))
                .collect();
            result["conservation"] = json!({
                "before": totals(&case.initial), "original_after": totals(&case.final_fields), "rust_after": totals(&rust_final),
                "conditions": "evap=evap_edge=absorb=cap_dry=0, evap_mul=0, no salt/scrub, all wetness=1 prevents dry-fold; semi-Lagrangian pigment drift and half-float quantization remain upstream behavior"
            });
        }
        cases.push(result);
    }
    let output = json!({ "upstream_commit": "a19bfa2a026856a98ceb400a8eec41f091f04a11", "original_renderer": reference.renderer,
        "width": reference.width, "height": reference.height, "storage": "RGBA16F", "orientation": "GL-bottom-up", "raster_subpixel_bits": reference.subpixel_bits,
        "tolerance": "max(1e-5 absolute, 4 half-float ULPs at compared component magnitude)", "samples_over_tolerance": failures, "isolated_samples_over_tolerance": isolated_failures, "cases": cases });
    fs::write(&args[2], serde_json::to_vec_pretty(&output)?)?;
    println!(
        "{} cases, {} accumulated and {} isolated samples over tolerance; {}",
        output["cases"].as_array().unwrap().len(),
        failures,
        isolated_failures,
        args[2]
    );
    if isolated_failures > 0 {
        return Err("isolated per-pass parity exceeded declared float tolerance".into());
    }
    Ok(())
}
