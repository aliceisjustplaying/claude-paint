use crate::{Watercolor, PIGMENTS, TICK_NS};
use mlua::{Lua, LuaOptions, StdLib, Table, UserData, UserDataMethods, Value};
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    rc::Rc,
};
use watercolor_lab_core::{
    brush as lab_brush,
    paper::PaperKind,
    pigments::{pigment_by_id, pigments},
};

type Result<T> = mlua::Result<T>;
fn error(e: impl ToString) -> mlua::Error {
    mlua::Error::external(e.to_string())
}
fn number(t: &Table, key: &str, default: f64) -> Result<f64> {
    let n = t.get::<Option<f64>>(key)?.unwrap_or(default);
    if !n.is_finite() {
        return Err(error(format!("{key} must be finite")));
    }
    Ok(n)
}
fn basename(name: &str) -> Result<&str> {
    if name.is_empty()
        || name.len() > 80
        || name.starts_with('.')
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
    {
        return Err(error(
            "use one plain file or tool name, without directories",
        ));
    }
    Ok(name)
}
fn pigment(name: &str) -> Result<usize> {
    match name {
        "ultramarine" => Ok(0),
        "phthalo" => Ok(1),
        "earth" => Ok(2),
        _ => Err(error("pigment must be ultramarine, phthalo or earth")),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brush {
    pub width: f64,
    pub water: f64,
    pub pigment: [f64; PIGMENTS],
    #[serde(default)]
    pub lab_pigments: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub format: String,
    pub canvas: Option<Watercolor>,
    pub brushes: BTreeMap<String, Brush>,
    pub water_supplied: f64,
    pub pigment_supplied: [f64; PIGMENTS],
    pub blotted_water: f64,
    pub blotted_pigment: [f64; PIGMENTS],
    pub applied_sources: Vec<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            format: "experimental-watercolor-studio-1".into(),
            canvas: None,
            brushes: BTreeMap::new(),
            water_supplied: 0.0,
            pigment_supplied: [0.0; PIGMENTS],
            blotted_water: 0.0,
            blotted_pigment: [0.0; PIGMENTS],
            applied_sources: Vec::new(),
        }
    }
}

impl Session {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let state: Self = serde_json::from_slice(bytes).map_err(error)?;
        if state.format != "experimental-watercolor-studio-1" {
            return Err(error("unsupported studio checkpoint"));
        }
        if let Some(canvas) = &state.canvas {
            if canvas.glazes.len() > 12
                || canvas.tick > 1_000_000_000_000
                || canvas.width.checked_mul(canvas.height).is_none_or(|n| {
                    n > if canvas.schema == 5 {
                        2400 * 1600
                    } else {
                        1_500_000
                    }
                })
            {
                return Err(error("checkpoint exceeds experimental layer/clock limits"));
            }
            canvas.validate().map_err(error)?;
        }
        for (name, brush) in &state.brushes {
            basename(name)?;
            if !brush.width.is_finite()
                || !(0.1..=if state.canvas.as_ref().is_some_and(|s| s.schema == 5) {
                    2400.0
                } else {
                    20.0
                })
                    .contains(&brush.width)
                || !brush.water.is_finite()
                || brush.water < 0.0
                || brush.pigment.iter().any(|v| !v.is_finite() || *v < 0.0)
                || brush
                    .lab_pigments
                    .iter()
                    .any(|(id, v)| pigment_by_id(id).is_none() || !v.is_finite() || *v < 0.0)
            {
                return Err(error("invalid brush checkpoint"));
            }
        }
        state.check_balance()?;
        Ok(state)
    }

    pub fn check_balance(&self) -> Result<()> {
        if !self.blotted_water.is_finite()
            || self.blotted_water < 0.0
            || self
                .blotted_pigment
                .iter()
                .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(error("invalid blot inventory"));
        }
        let Some(canvas) = &self.canvas else {
            return Ok(());
        };
        if let Some(lab) = &canvas.lab {
            lab.validate(canvas.width, canvas.height).map_err(error)?;
            if !self.water_supplied.is_finite()
                || self.water_supplied < 0.0
                || self
                    .pigment_supplied
                    .iter()
                    .any(|v| !v.is_finite() || *v < 0.0)
            {
                return Err(error("invalid supplied inventory"));
            }
            let water =
                lab.water_released_mm3 + self.brushes.values().map(|b| b.water).sum::<f64>();
            let pigment = lab.pigment_released
                + self
                    .brushes
                    .values()
                    .map(|b| b.pigment.iter().chain(b.lab_pigments.values()).sum::<f64>())
                    .sum::<f64>();
            if (water - self.water_supplied).abs() > 1e-8 * self.water_supplied.max(1.0)
                || (pigment - self.pigment_supplied.iter().sum::<f64>()).abs()
                    > 1e-8 * self.pigment_supplied.iter().sum::<f64>().max(1.0)
            {
                return Err(error("lab brush-release inventory failed"));
            }
            return Ok(());
        }
        let area = canvas.cell_mm * canvas.cell_mm;
        let b = canvas.balance();
        let water = (b.surface_water + b.capillary_water + canvas.ledger.water_evaporated) * area
            + self.brushes.values().map(|b| b.water).sum::<f64>()
            + self.blotted_water;
        if !self.water_supplied.is_finite()
            || self.water_supplied < 0.0
            || (water - self.water_supplied).abs() > 1e-8 * self.water_supplied.max(1.0)
        {
            return Err(error(format!(
                "studio water balance failed: {water} vs {}",
                self.water_supplied
            )));
        }
        for p in 0..PIGMENTS {
            let mass = (b.suspended[p] + b.deposited[p] + b.bound[p]) * area
                + self.brushes.values().map(|b| b.pigment[p]).sum::<f64>()
                + self.blotted_pigment[p];
            if !self.pigment_supplied[p].is_finite()
                || self.pigment_supplied[p] < 0.0
                || (mass - self.pigment_supplied[p]).abs()
                    > 1e-9 * self.pigment_supplied[p].max(1.0)
            {
                return Err(error(format!("studio pigment {p} balance failed")));
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Frames {
    cadence: u64,
    width: u32,
    prefix: String,
}
struct Context {
    state: Session,
    stage: PathBuf,
    frames: Option<Frames>,
    rendered_frames: u64,
}
impl Context {
    fn canvas(&mut self) -> Result<&mut Watercolor> {
        self.state
            .canvas
            .as_mut()
            .ok_or_else(|| error("call paper{} first"))
    }
}

pub fn render(
    canvas: &Watercolor,
    path: &Path,
    mode: &str,
    width: u32,
    crop: Option<[u32; 4]>,
) -> Result<()> {
    if !(if canvas.schema == 5 { 1 } else { 16 }..=2400).contains(&width) {
        return Err(error(if canvas.schema == 5 {
            "render width must be 1..2400"
        } else {
            "render width must be 16..2400"
        }));
    }
    let height =
        ((width as f64 * canvas.height as f64 / canvas.width as f64).round() as u32).max(1);
    render_size(canvas, path, mode, [width, height], crop)
}

pub fn render_size(
    canvas: &Watercolor,
    path: &Path,
    mode: &str,
    [width, height]: [u32; 2],
    crop: Option<[u32; 4]>,
) -> Result<()> {
    if !(1..=2400).contains(&width) || height == 0 {
        return Err(error("render size needs width 1..2400 and positive height"));
    }
    let rgb = match mode {
        "color" => canvas.rgb_color(),
        "gallery" => canvas.rgb_lit(115.0, 55.0, 1.0),
        "relief" => canvas.rgb_lit(135.0, 25.0, 1.0),
        _ => return Err(error("mode must be color, gallery or relief")),
    };
    let original =
        image::RgbImage::from_raw(canvas.width as u32, canvas.height as u32, rgb).unwrap();
    let image = if width == original.width() && height == original.height() {
        original
    } else {
        image::imageops::resize(
            &original,
            width,
            height,
            image::imageops::FilterType::Lanczos3,
        )
    };
    let image = if let Some([x, y, w, h]) = crop {
        if w == 0
            || h == 0
            || x.checked_add(w).is_none_or(|v| v > width)
            || y.checked_add(h).is_none_or(|v| v > height)
        {
            return Err(error("crop lies outside rendered pixels"));
        }
        image::imageops::crop_imm(&image, x, y, w, h).to_image()
    } else {
        image
    };
    image.save(path).map_err(error)
}

fn points(t: &Table) -> Result<Vec<[f64; 2]>> {
    let mut out = Vec::new();
    for point in t.sequence_values::<Table>() {
        let p = point?;
        let (x, y) = (p.get::<f64>(1)?, p.get::<f64>(2)?);
        if !x.is_finite() || !y.is_finite() {
            return Err(error("coordinates must be finite"));
        }
        out.push([x, y]);
    }
    if out.is_empty() || out.len() > 2000 {
        return Err(error("a path needs 1..2000 points"));
    }
    Ok(out)
}

fn stroke_mask(canvas: &Watercolor, path: &[[f64; 2]], width: f64) -> Vec<(usize, f64, [f64; 2])> {
    let cell_mm = canvas.cell_mm;
    let mut mask = Vec::new();
    let min_x = path.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min) - width;
    let max_x = path.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max) + width;
    let min_y = path.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min) - width;
    let max_y = path.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max) + width;
    let x0 = (min_x / cell_mm).floor().max(0.0) as usize;
    let x1 = ((max_x / cell_mm).ceil().max(0.0) as usize).min(canvas.width);
    let y0 = (min_y / cell_mm).floor().max(0.0) as usize;
    let y1 = ((max_y / cell_mm).ceil().max(0.0) as usize).min(canvas.height);
    for y in y0..y1 {
        for x in x0..x1 {
            let mut hits = 0;
            let mut direction = [1.0, 0.0];
            for sy in 0..4 {
                for sx in 0..4 {
                    let p = [
                        (x as f64 + (sx as f64 + 0.5) / 4.0) * cell_mm,
                        (y as f64 + (sy as f64 + 0.5) / 4.0) * cell_mm,
                    ];
                    let mut distance = (p[0] - path[0][0]).hypot(p[1] - path[0][1]);
                    for pair in path.windows(2) {
                        let d = [pair[1][0] - pair[0][0], pair[1][1] - pair[0][1]];
                        let length = d[0] * d[0] + d[1] * d[1];
                        if length == 0.0 {
                            continue;
                        }
                        let t = (((p[0] - pair[0][0]) * d[0] + (p[1] - pair[0][1]) * d[1])
                            / length)
                            .clamp(0.0, 1.0);
                        let v = (p[0] - pair[0][0] - t * d[0]).hypot(p[1] - pair[0][1] - t * d[1]);
                        if v < distance {
                            distance = v;
                            direction = d;
                        }
                    }
                    let tooth = canvas.paper.height[y * canvas.width + x];
                    hits += usize::from(distance < width * 0.5 + (tooth - 0.5) * 0.45);
                }
            }
            if hits > 0 {
                mask.push((y * canvas.width + x, hits as f64 / 16.0, direction));
            }
        }
    }
    mask
}

fn rectangle(canvas: &Watercolor, t: &Table) -> Result<Vec<(usize, f64, [f64; 2])>> {
    let cell_mm = if canvas.schema == 5 {
        1.0
    } else {
        canvas.cell_mm
    };
    let x0 = number(t, "x", 0.0)?;
    let y0 = number(t, "y", 0.0)?;
    let w = number(t, "width", canvas.width as f64 * cell_mm)?;
    let h = number(t, "height", canvas.height as f64 * cell_mm)?;
    if w <= 0.0 || h <= 0.0 {
        return Err(error("rectangle width and height must be positive"));
    }
    let mut mask = Vec::new();
    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let dx = (((x + 1) as f64 * cell_mm).min(x0 + w) - (x as f64 * cell_mm).max(x0))
                .clamp(0.0, cell_mm);
            let dy = (((y + 1) as f64 * cell_mm).min(y0 + h) - (y as f64 * cell_mm).max(y0))
                .clamp(0.0, cell_mm);
            if dx * dy > 0.0 {
                mask.push((
                    y * canvas.width + x,
                    dx * dy / (cell_mm * cell_mm),
                    [1.0, 0.0],
                ));
            }
        }
    }
    Ok(mask)
}

fn deposit(
    ctx: &mut Context,
    name: &str,
    mask: &[(usize, f64, [f64; 2])],
    attrs: &Table,
) -> Result<()> {
    let area = ctx.canvas()?.cell_mm.powi(2);
    let mut brush = ctx
        .state
        .brushes
        .get(name)
        .cloned()
        .ok_or_else(|| error("unknown brush"))?;
    let dry = attrs
        .get::<Option<bool>>("dry")?
        .unwrap_or(brush.water < 0.001);
    let pressure = number(attrs, "pressure", 0.35)?;
    if !(0.0..=1.0).contains(&pressure) {
        return Err(error("pressure must be 0..1"));
    }
    let sum: f64 = mask.iter().map(|m| m.1).sum();
    if sum == 0.0 {
        return Ok(());
    }
    let water = if dry {
        0.0
    } else {
        brush.water.min(sum * area * 0.12)
    };
    let fraction = if dry {
        (sum * area * 0.012 / brush.pigment.iter().sum::<f64>().max(1e-30)).min(1.0)
    } else {
        water / brush.water.max(1e-30)
    };
    let allocated = brush.pigment.map(|v| v * fraction);
    let grade = number(attrs, "grade", 1.0)?;
    if !(0.0..=1.0).contains(&grade) {
        return Err(error("grade must be 0..1"));
    }
    let canvas = ctx.canvas()?;
    let ymin = mask.iter().map(|m| m.0 / canvas.width).min().unwrap();
    let ymax = mask.iter().map(|m| m.0 / canvas.width).max().unwrap();
    let grid_width = canvas.width;
    let graded = |i: usize| {
        1.0 - (1.0 - grade) * (i / grid_width - ymin) as f64 / (ymax - ymin).max(1) as f64
    };
    let pigment_sum: f64 = mask.iter().map(|m| m.1 * graded(m.0)).sum();
    let mut used = [0.0; PIGMENTS];
    for &(i, coverage, direction) in mask {
        let load = allocated.map(|v| v * coverage * graded(i) / pigment_sum / area);
        if dry {
            let before = canvas.active.deposited[i];
            canvas.dry_contact_directional(i, load, pressure, direction);
            for p in 0..PIGMENTS {
                used[p] += (canvas.active.deposited[i][p] - before[p]) * area;
            }
        } else {
            canvas.add_covered(i, water * coverage / sum / area, load, coverage);
        }
    }
    brush.water -= water;
    for p in 0..PIGMENTS {
        brush.pigment[p] = (brush.pigment[p] - if dry { used[p] } else { allocated[p] }).max(0.0);
    }
    ctx.state.brushes.insert(name.to_owned(), brush);
    Ok(())
}

/// Brush adapter for src/paint/brush.ts:87–161 at a19bfa2a026856a98ceb400a8eec41f091f04a11; units and finite inventory stay outside the core.
fn lab_stroke(
    ctx: &mut Context,
    name: &str,
    path: &Table,
    attrs: &Table,
    scrub: bool,
    wash_grade: f64,
) -> Result<()> {
    let xy = points(path)?;
    let pressure = number(attrs, "pressure", 0.35)?;
    if !(0.0..=1.0).contains(&pressure) {
        return Err(error("pressure must be 0..1"));
    }
    let grade = number(attrs, "grade", 1.0)?;
    if !(0.0..=1.0).contains(&grade) {
        return Err(error("grade must be 0..1"));
    }
    let ymin = xy.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
    let ymax = xy.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);
    let length: f64 = xy
        .windows(2)
        .map(|p| (p[1][0] - p[0][0]).hypot(p[1][1] - p[0][1]))
        .sum();
    let cells = ctx.canvas()?.schema == 5;
    let duration = number(
        attrs,
        "duration",
        (length / if cells { 800.0 } else { 40.0 }).max(0.0625),
    )?;
    if !(0.0..=600.0).contains(&duration) || (length > 0.0 && duration == 0.0) {
        return Err(error(
            "a moving stroke needs a positive duration up to 600 seconds",
        ));
    }
    let mut positions = Vec::new();
    let mut traveled = 0.0;
    for (i, point) in path.sequence_values::<Table>().enumerate() {
        let point = point?;
        if i > 0 {
            traveled += (xy[i][0] - xy[i - 1][0]).hypot(xy[i][1] - xy[i - 1][1]);
        }
        let p = point.get::<Option<f64>>(3)?.unwrap_or(pressure);
        let t = point.get::<Option<f64>>(4)?.unwrap_or(if length > 0.0 {
            duration * traveled / length
        } else {
            0.0
        });
        if !p.is_finite()
            || !(0.0..=1.0).contains(&p)
            || !t.is_finite()
            || !(0.0..=600.0).contains(&t)
            || (i == 0 && t != 0.0)
            || positions
                .last()
                .is_some_and(|v: &([f64; 2], f64, f64)| t < v.2 || (xy[i] != v.0 && t == v.2))
        {
            return Err(error(
                "stroke points are {x,y,pressure,time_seconds}; times start at zero and increase",
            ));
        }
        positions.push((xy[i], p, t));
    }
    let mut inventory = ctx.state.brushes[name].clone();
    let area = ctx.canvas()?.cell_mm.powi(2);
    let cell = if cells { 1.0 } else { ctx.canvas()?.cell_mm };
    let tool = lab_brush::tool(if scrub {
        lab_brush::ToolId::Lift
    } else {
        lab_brush::ToolId::Round
    });
    let radius_scale = inventory.width / (2.0 * cell * lab_brush::brush_radius(&tool, 0.5));
    let mut stroke = lab_brush::Brush::begin(
        tool,
        0.5,
        1.0,
        1.0,
        xy[0][0] / cell,
        xy[0][1] / cell,
        1,
        radius_scale,
    );
    let dry = attrs
        .get::<Option<bool>>("dry")?
        .unwrap_or(inventory.water < 0.001);
    let total_pigment = inventory
        .pigment
        .iter()
        .chain(inventory.lab_pigments.values())
        .sum::<f64>();
    let mut mix = pigment_by_id("french-ultramarine").unwrap();
    mix.k = [0.0; 3];
    mix.s = [0.0; 3];
    mix.density = 0.0;
    mix.granulation = 0.0;
    mix.staining = 0.0;
    let components: Vec<_> = if cells {
        inventory
            .lab_pigments
            .iter()
            .map(|(id, mass)| (id.as_str(), *mass))
            .collect()
    } else {
        ["french-ultramarine", "phthalo-blue", "burnt-sienna"]
            .into_iter()
            .zip(inventory.pigment)
            .collect()
    };
    for (id, mass) in components {
        let pigment =
            pigment_by_id(id).ok_or_else(|| error(format!("missing ported pigment {id}")))?;
        let fraction = (mass / total_pigment.max(1e-30)) as f32;
        for c in 0..3 {
            mix.k[c] += pigment.k[c] * fraction;
            mix.s[c] += pigment.s[c] * fraction;
        }
        mix.density += pigment.density * fraction;
        mix.granulation += pigment.granulation * fraction;
        mix.staining += pigment.staining * fraction;
    }
    let mut schedule = vec![(0.0, stroke.tap_stamp(positions[0].1))];
    for pair in positions.windows(2) {
        let stamps = stroke.stroke_to(pair[1].0[0] / cell, pair[1].0[1] / cell, pair[1].1);
        let distance = (pair[1].0[0] - pair[0].0[0]).hypot(pair[1].0[1] - pair[0].0[1]);
        for stamp in stamps {
            let fraction = if distance > 0.0 {
                (stamp.x * cell - pair[0].0[0]).hypot(stamp.y * cell - pair[0].0[1]) / distance
            } else {
                1.0
            };
            schedule.push((
                pair[0].2 + (pair[1].2 - pair[0].2) * fraction.clamp(0.0, 1.0),
                stamp,
            ));
        }
    }
    let mut elapsed_ns = 0;
    for (time, mut stamp) in schedule {
        let target_ns = (time * 1e9).round() as u64;
        ctx.canvas()?.wait_ns(target_ns.saturating_sub(elapsed_ns));
        elapsed_ns = target_ns;
        if dry {
            stamp.water = 0.0;
            stamp.dryness = 1.0;
        }
        if scrub {
            stamp.pigment = 0.0;
        }
        stamp.pigment *= wash_grade
            * (1.0
                - (1.0 - grade)
                    * ((stamp.y * cell - ymin) / (ymax - ymin).max(1e-30)).clamp(0.0, 1.0));
        let lab = ctx.canvas()?.lab.as_mut().unwrap();
        let (ww, pw) = lab.stamp_weights(&stamp);
        let wanted_water = stamp.water * 0.055 * ww * area;
        let wanted_pigment = stamp.pigment * 0.13 * pw * area;
        let water = wanted_water.min(inventory.water);
        let pigment = wanted_pigment.min(
            inventory
                .pigment
                .iter()
                .chain(inventory.lab_pigments.values())
                .sum::<f64>(),
        );
        if wanted_water > 0.0 {
            stamp.water *= water / wanted_water;
        }
        if wanted_pigment > 0.0 {
            stamp.pigment *= pigment / wanted_pigment;
        }
        lab.splat(
            &[stamp],
            if total_pigment > 0.0 && !scrub {
                Some(&mix)
            } else {
                None
            },
        );
        if scrub {
            lab.add_scrub(&[stamp]);
        }
        inventory.water = (inventory.water - water).max(0.0);
        let remaining = inventory
            .pigment
            .iter()
            .chain(inventory.lab_pigments.values())
            .sum::<f64>();
        for p in inventory
            .pigment
            .iter_mut()
            .chain(inventory.lab_pigments.values_mut())
        {
            *p *= (1.0 - pigment / remaining.max(1e-30)).max(0.0);
        }
        lab.water_released_mm3 += water;
        lab.pigment_released += pigment;
    }
    let end_ns = (positions.last().unwrap().2.max(duration) * 1e9).round() as u64;
    ctx.canvas()?.wait_ns(end_ns.saturating_sub(elapsed_ns));
    ctx.state.brushes.insert(name.to_owned(), inventory);
    Ok(())
}

#[derive(Clone)]
struct Handle {
    ctx: Rc<RefCell<Context>>,
    name: String,
}
impl UserData for Handle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("status", |lua, this, ()| {
            let c = this.ctx.borrow();
            let b = &c.state.brushes[&this.name];
            let t = lua.create_table()?;
            let cells = c.state.canvas.as_ref().is_some_and(|s| s.schema == 5);
            t.set(if cells { "width" } else { "width_mm" }, b.width)?;
            t.set(if cells { "water" } else { "water_mm3" }, b.water)?;
            let pigments = lua.create_table()?;
            if cells {
                for (id, mass) in &b.lab_pigments {
                    pigments.set(id.as_str(), *mass)?;
                }
            } else {
                for (p, name) in ["ultramarine", "phthalo", "earth"].iter().enumerate() {
                    pigments.set(*name, b.pigment[p])?;
                }
            }
            t.set("pigments", pigments)?;
            Ok(t)
        });
        methods.add_method("load", |_, this, t: Table| {
            let mut ctx = this.ctx.borrow_mut();
            let cells = ctx.canvas()?.schema == 5;
            let water = number(&t, "water", 0.0)?;
            if !(0.0..=100.0).contains(&water) {
                return Err(error("water load must be 0..100 mm3"));
            }
            let mut pigment_load = [0.0; PIGMENTS];
            let mut lab_load = BTreeMap::new();
            if let Some(colors) = t.get::<Option<Table>>("pigments")? {
                for pair in colors.pairs::<String, f64>() {
                    let (name, mass) = pair?;
                    if !mass.is_finite() || !(0.0..=10.0).contains(&mass) {
                        return Err(error("pigment load must be 0..10"));
                    }
                    if cells {
                        if pigment_by_id(&name).is_none() {
                            return Err(error("unknown library pigment id; call palette()"));
                        }
                        lab_load.insert(name, mass);
                    } else {
                        pigment_load[pigment(&name)?] = mass;
                    }
                }
            }
            let brush = ctx.state.brushes.get_mut(&this.name).unwrap();
            brush.water += water;
            for (p, mass) in pigment_load.iter().enumerate() {
                brush.pigment[p] += mass;
            }
            let lab_mass = lab_load.values().sum::<f64>();
            for (id, mass) in lab_load {
                *brush.lab_pigments.entry(id).or_default() += mass;
            }
            ctx.state.water_supplied += water;
            for (p, mass) in pigment_load.iter().enumerate() {
                ctx.state.pigment_supplied[p] += mass;
            }
            ctx.state.pigment_supplied[0] += lab_mass;
            Ok(())
        });
        methods.add_method(
            "stroke",
            |lua, this, (path, attrs): (Table, Option<Table>)| {
                let points = points(&path)?;
                let mut ctx = this.ctx.borrow_mut();
                let attrs = match attrs {
                    Some(t) => t,
                    None => lua.create_table()?,
                };
                if ctx.canvas()?.lab.is_some() {
                    return lab_stroke(&mut ctx, &this.name, &path, &attrs, false, 1.0);
                }
                let width = ctx.state.brushes[&this.name].width;
                let mask = stroke_mask(ctx.canvas()?, &points, width);
                deposit(&mut ctx, &this.name, &mask, &attrs)
            },
        );
        methods.add_method("wash", |lua, this, t: Table| {
            let mut ctx = this.ctx.borrow_mut();
            if ctx.canvas()?.lab.is_some() {
                rectangle(ctx.canvas()?, &t)?;
                let x = number(&t, "x", 0.0)?;
                let y = number(&t, "y", 0.0)?;
                let width = number(&t, "width", 1.0)?;
                let height = number(&t, "height", 1.0)?;
                let brush_width = ctx.state.brushes[&this.name].width;
                let rows = (height / (brush_width * 0.7)).ceil().max(1.0) as usize;
                let grade = number(&t, "grade", 1.0)?;
                if !(0.0..=1.0).contains(&grade) {
                    return Err(error("grade must be 0..1"));
                }
                for row in 0..rows {
                    let yy = y + (row as f64 + 0.5) * height / rows as f64;
                    let path = lua.create_table()?;
                    path.set(1, lua.create_sequence_from([x, yy])?)?;
                    path.set(2, lua.create_sequence_from([x + width, yy])?)?;
                    lab_stroke(
                        &mut ctx,
                        &this.name,
                        &path,
                        &t,
                        false,
                        1.0 - (1.0 - grade) * row as f64 / (rows - 1).max(1) as f64,
                    )?;
                }
                return Ok(());
            }
            let mask = rectangle(ctx.canvas()?, &t)?;
            deposit(&mut ctx, &this.name, &mask, &t)
        });
        methods.add_method("lift", |lua, this, path: Table| {
            let points = points(&path)?;
            let mut ctx = this.ctx.borrow_mut();
            if ctx.canvas()?.lab.is_some() {
                return lab_stroke(&mut ctx, &this.name, &path, &lua.create_table()?, true, 1.0);
            }
            let mut brush = ctx.state.brushes[&this.name].clone();
            let mask = stroke_mask(ctx.canvas()?, &points, brush.width);
            let area = ctx.canvas()?.cell_mm.powi(2);
            let sum: f64 = mask.iter().map(|m| m.1).sum();
            if sum == 0.0 {
                return Ok(());
            }
            let water = brush.water.min(sum * area * 0.045);
            brush.water -= water;
            let canvas = ctx.canvas()?;
            for &(i, coverage, _) in &mask {
                canvas.add_covered(i, water * coverage / sum / area, [0.0; PIGMENTS], coverage);
                let before_w = canvas.ledger.water_lifted;
                let before_p = canvas.ledger.pigment_lifted;
                canvas.lift(i, coverage * 0.95);
                brush.water += (canvas.ledger.water_lifted - before_w) * area;
                for (p, before) in before_p.iter().enumerate() {
                    brush.pigment[p] += (canvas.ledger.pigment_lifted[p] - before) * area;
                }
            }
            ctx.state.brushes.insert(this.name.clone(), brush);
            Ok(())
        });
    }
}

fn install(lua: &Lua, ctx: Rc<RefCell<Context>>) -> Result<()> {
    let globals = lua.globals();
    let c = ctx.clone();
    globals.set(
        "paper",
        lua.create_function(move |_, t: Table| {
            let mut c = c.borrow_mut();
            if c.state.canvas.is_some() {
                return Err(error("paper already exists; resume with a new chunk"));
            }
            let kind = t
                .get::<Option<String>>("kind")?
                .unwrap_or("cold-press".into());
            let solver = t.get::<Option<String>>("solver")?.unwrap_or("lab".into());
            let seed = t.get::<Option<u64>>("seed")?.unwrap_or(71);
            if solver == "lab" {
                if t.contains_key("size")? || t.contains_key("aspect")? || t.contains_key("cell")? {
                    return Err(error("Lab paper uses width and height in cells"));
                }
                let kind = match kind.as_str() {
                    "cold-press" => PaperKind::ColdPress,
                    "hot-press" => PaperKind::HotPress,
                    "rough" => PaperKind::Rough,
                    _ => return Err(error("paper kind must be cold-press, hot-press or rough")),
                };
                c.state.canvas = Some(
                    Watercolor::new_lab(
                        t.get::<Option<usize>>("width")?.unwrap_or(1056),
                        t.get::<Option<usize>>("height")?.unwrap_or(704),
                        u32::try_from(seed).map_err(error)?,
                        kind,
                    )
                    .map_err(error)?,
                );
                return Ok(());
            }
            if solver != "current" {
                return Err(error("solver must be current or lab"));
            }
            let size = number(&t, "size", 36.0)?;
            let aspect = number(&t, "aspect", 1.5)?;
            let cell_mm = number(&t, "cell", 0.05)?;
            if !(0.025..=0.1).contains(&cell_mm) {
                return Err(error("paper cell must be 0.025..0.1 mm"));
            }
            if !(2.0..=60.0).contains(&size) || !(0.5..=3.0).contains(&aspect) {
                return Err(error("paper size must be 2..60 mm, aspect 0.5..3"));
            }
            let w = (size / cell_mm).round() as usize;
            let h = (w as f64 / aspect).round() as usize;
            if w * h > 1_500_000 {
                return Err(error("experimental paper exceeds 1500000 cells"));
            }
            let mut paper = Watercolor::new_at(w, h, seed, cell_mm);
            match kind.as_str() {
                "cold-press" => {}
                "hot-press" => {
                    paper.paper.height = crate::correlate(&paper.paper.height, w, h, 8);
                    paper.paper.sizing.fill(0.75);
                }
                _ => return Err(error("paper kind must be cold-press or hot-press")),
            }
            c.state.canvas = Some(paper);
            Ok(())
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "brush",
        lua.create_function(move |_, t: Table| {
            let name = t.get::<String>("name")?;
            basename(&name)?;
            let owner = c.clone();
            let mut c = c.borrow_mut();
            if let Some(existing) = c.state.brushes.get(&name) {
                if let Some(width) = t.get::<Option<f64>>("width")? {
                    if !width.is_finite() || (width - existing.width).abs() > 1e-12 {
                        return Err(error(
                            "an existing named brush keeps its width; choose a new name",
                        ));
                    }
                }
            } else {
                let cells = c.canvas()?.schema == 5;
                let width = number(&t, "width", if cells { 24.0 } else { 1.0 })?;
                if !(if cells { 1.0 } else { 0.1 }..=if cells { 2400.0 } else { 20.0 })
                    .contains(&width)
                {
                    return Err(error(if cells {
                        "brush width must be 1..2400 cells"
                    } else {
                        "brush width must be 0.1..20 mm"
                    }));
                }
                c.state.brushes.insert(
                    name.clone(),
                    Brush {
                        width,
                        water: 0.0,
                        pigment: [0.0; PIGMENTS],
                        lab_pigments: BTreeMap::new(),
                    },
                );
            }
            drop(c);
            Ok(Handle { ctx: owner, name })
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "drop",
        lua.create_function(move |_, t: Table| {
            let x = number(&t, "x", 0.0)?;
            let y = number(&t, "y", 0.0)?;
            let water = number(&t, "water", 1.0)?;
            let mut c = c.borrow_mut();
            let canvas = c.canvas()?;
            let cells = canvas.schema == 5;
            let radius = number(&t, "radius", if cells { 12.0 } else { 0.75 })?;
            if !(0.0..=100.0).contains(&water)
                || !(if cells { 1.0 } else { 0.1 }..=if cells { 1200.0 } else { 10.0 })
                    .contains(&radius)
            {
                return Err(error(if cells {
                    "drop water must be 0..100 and radius 1..1200 cells"
                } else {
                    "drop needs nonnegative water and radius 0.1..10 mm"
                }));
            }
            let cell_mm = if cells { 1.0 } else { canvas.cell_mm };
            let area = canvas.cell_mm.powi(2);
            if let Some(lab) = &mut canvas.lab {
                let mut stamp = lab_brush::Stamp {
                    x: x / cell_mm,
                    y: y / cell_mm,
                    radius: radius / cell_mm,
                    water: 1.0,
                    pigment: 0.0,
                    dryness: 0.0,
                };
                let (weight, _) = lab.stamp_weights(&stamp);
                if weight == 0.0 {
                    return Err(error("drop is outside paper"));
                }
                stamp.water = water / (0.055 * weight * area);
                lab.splat(&[stamp], None);
                lab.water_released_mm3 += water;
                c.state.water_supplied += water;
                return Ok(());
            }
            let mut mask = Vec::new();
            let mut sum = 0.0;
            for yy in 0..canvas.height {
                for xx in 0..canvas.width {
                    let d = ((xx as f64 + 0.5) * cell_mm - x).powi(2)
                        + ((yy as f64 + 0.5) * cell_mm - y).powi(2);
                    let weight = (-d / (2.0 * radius * radius)).exp();
                    if weight > 0.0001 {
                        mask.push((yy * canvas.width + xx, weight));
                        sum += weight;
                    }
                }
            }
            if sum == 0.0 {
                return Err(error("drop is outside paper"));
            }
            for (i, w) in mask {
                canvas.add(i, water * w / sum / (cell_mm * cell_mm), [0.0; PIGMENTS]);
            }
            c.state.water_supplied += water;
            Ok(())
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "frames",
        lua.create_function(move |_, v: Value| {
            let mut c = c.borrow_mut();
            if matches!(v, Value::Boolean(false)) {
                c.frames = None;
                return Ok(());
            }
            let Value::Table(t) = v else {
                return Err(error("frames expects a table or false"));
            };
            let ns = (number(&t, "every", 0.25)? * 1e9).round();
            if ns < TICK_NS as f64 || ns > 60e9 || !(ns as u64).is_multiple_of(TICK_NS) {
                return Err(error(
                    "frame interval must be a positive multiple of 0.0625 seconds",
                ));
            }
            let prefix = t.get::<Option<String>>("prefix")?.unwrap_or("frame".into());
            basename(&prefix)?;
            let width = t.get::<Option<u32>>("width")?.unwrap_or(480);
            if !(16..=1200).contains(&width) {
                return Err(error("frame width must be 16..1200"));
            }
            c.frames = Some(Frames {
                cadence: ns as u64 / TICK_NS,
                width,
                prefix,
            });
            Ok(())
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "wait",
        lua.create_function(move |_, seconds: f64| {
            if !seconds.is_finite() || !(0.0..=600.0).contains(&seconds) {
                return Err(error("wait is 0..600 seconds"));
            }
            let mut c = c.borrow_mut();
            let frames = c.frames.clone();
            let stage = c.stage.clone();
            let ns = (seconds * 1e9).round() as u64;
            if let Some(f) = &frames {
                let s = c.canvas()?;
                let end = s.tick + (s.residual_ns + ns) / TICK_NS;
                let count = end / f.cadence - s.tick / f.cadence;
                if c.rendered_frames + count > 1024 {
                    return Err(error("frame hook exceeds 1024 frames per chunk"));
                }
                c.rendered_frames += count;
            }
            let canvas = c.canvas()?;
            if let Some(f) = frames {
                let mut failure = None;
                canvas.wait_ns_observed(ns, f.cadence, |s| {
                    if failure.is_none() {
                        if let Err(e) = render(
                            s,
                            &stage.join(format!("{}-{:08}.png", f.prefix, s.tick)),
                            "gallery",
                            f.width,
                            None,
                        ) {
                            failure = Some(e);
                        }
                    }
                });
                if let Some(e) = failure {
                    return Err(e);
                }
            } else {
                canvas.wait_ns(ns);
            }
            Ok(canvas.tick as f64 * TICK_NS as f64 / 1e9 + canvas.residual_ns as f64 / 1e9)
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "glaze",
        lua.create_function(move |_, ()| {
            let mut c = c.borrow_mut();
            let canvas = c.canvas()?;
            if canvas.glazes.len() >= 12 {
                return Err(error(
                    "experimental studio supports at most 12 archived glazes",
                ));
            }
            canvas.finish_glaze().map_err(error)
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "blot",
        lua.create_function(move |_, t: Table| {
            let mut c = c.borrow_mut();
            if c.canvas()?.lab.is_some() {
                return Err(error(
                    "blot is unavailable for solver lab; use the lift brush",
                ));
            }
            let mask = rectangle(c.canvas()?, &t)?;
            let canvas = c.canvas()?;
            let before_w = canvas.ledger.water_lifted;
            let before_p = canvas.ledger.pigment_lifted;
            for (i, coverage, _) in mask {
                canvas.lift(i, coverage);
            }
            let area = canvas.cell_mm.powi(2);
            let water = (canvas.ledger.water_lifted - before_w) * area;
            let pigments: [f64; PIGMENTS] =
                std::array::from_fn(|p| (canvas.ledger.pigment_lifted[p] - before_p[p]) * area);
            c.state.blotted_water += water;
            for (p, mass) in pigments.iter().enumerate() {
                c.state.blotted_pigment[p] += mass;
            }
            Ok(())
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "look",
        lua.create_function(move |lua, t: Option<Table>| {
            let t = match t {
                Some(t) => t,
                None => lua.create_table()?,
            };
            let name = t
                .get::<Option<String>>("path")?
                .unwrap_or("look.png".into());
            basename(&name)?;
            if !name.ends_with(".png") {
                return Err(error("look filename must end .png"));
            }
            let mode = t.get::<Option<String>>("mode")?.unwrap_or("color".into());
            let width = t.get::<Option<u32>>("width")?.unwrap_or(1200);
            let crop = if let Some(r) = t.get::<Option<Table>>("crop")? {
                Some([r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?])
            } else {
                None
            };
            let mut c = c.borrow_mut();
            let path = c.stage.join(&name);
            render(c.canvas()?, &path, &mode, width, crop)?;
            Ok(name)
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "save",
        lua.create_function(move |_, name: Option<String>| {
            let name = name.unwrap_or("saved.json".into());
            basename(&name)?;
            if !name.ends_with(".json") || name == "checkpoint.json" {
                return Err(error(
                    "named saves need .json and cannot use checkpoint.json",
                ));
            }
            let c = c.borrow();
            c.state.check_balance()?;
            write_state(&c.stage.join(&name), &c.state)?;
            Ok(name)
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "clock",
        lua.create_function(move |_, ()| {
            let mut c = c.borrow_mut();
            let s = c.canvas()?;
            Ok(s.tick as f64 * TICK_NS as f64 / 1e9 + s.residual_ns as f64 / 1e9)
        })?,
    )?;
    let c = ctx.clone();
    globals.set(
        "balance",
        lua.create_function(move |lua, ()| {
            let c = c.borrow();
            c.state.check_balance()?;
            let t = lua.create_table()?;
            t.set("water_supplied_mm3", c.state.water_supplied)?;
            t.set("conserved", c.state.canvas.as_ref().is_none_or(|s| s.lab.is_none()))?;
            if let Some(s) = &c.state.canvas {
                if let Some(lab)=&s.lab {
                    t.set("solver","lab")?;
                    t.set("surface_water_mm3",lab.surface_water(s.cell_mm))?;
                    t.set("pore_water_mm3",lab.pore_water(s.cell_mm))?;
                    t.set("water_released_mm3",lab.water_released_mm3)?;
                    t.set("pigment_released",lab.pigment_released)?;
                    t.set("live_pigment",lab.simulation.sus_k.read.iter().chain(&lab.simulation.dep_k.read).map(|v|v[3] as f64).sum::<f64>()*s.cell_mm.powi(2))?;
                    t.set("max_surface_depth_mm",lab.simulation.flow.read.iter().map(|v|v[2] as f64).fold(0.0_f64,f64::max))?;
                    t.set("dry",lab.is_dry())?;
                    t.set("max_surface_water", lab.simulation.flow.read.iter().map(|v| v[2] as f64).fold(0.0_f64, f64::max))?;
                    t.set("max_wetness", lab.simulation.flow.read.iter().map(|v| v[3] as f64).fold(0.0_f64, f64::max))?;
                    t.set("inventory_note","Reference advection and half-float rounding can change mass. Dried pigment is stored as reflectance, so per-pigment conservation cannot be recovered.")?;
                    return Ok(t);
                }
                let b = s.balance();
                t.set("surface_water_mm3", b.surface_water * s.cell_mm.powi(2))?;
                t.set("pore_water_mm3", b.capillary_water * s.cell_mm.powi(2))?;
                t.set(
                    "max_surface_depth_mm",
                    s.water.iter().copied().fold(0.0_f64, f64::max),
                )?;
                t.set(
                    "dry",
                    s.water.iter().chain(&s.capillary).all(|v| *v <= 1e-10)
                        && s.suspended.iter().flatten().all(|v| *v <= 1e-10),
                )?;
            }
            Ok(t)
        })?,
    )?;
    globals.set(
        "palette",
        lua.create_function(move |lua, ()| {
            if ctx
                .borrow()
                .state
                .canvas
                .as_ref()
                .is_none_or(|s| s.schema == 5)
            {
                lua.create_sequence_from(pigments().into_iter().map(|p| p.id))
            } else {
                lua.create_sequence_from(["ultramarine", "phthalo", "earth"])
            }
        })?,
    )?;
    Ok(())
}

struct Lock(PathBuf);
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn write_state(path: &Path, state: &Session) -> Result<()> {
    let mut writer = BufWriter::new(fs::File::create(path).map_err(error)?);
    serde_json::to_writer(&mut writer, state).map_err(error)?;
    writer.flush().map_err(error)
}

pub fn init(root: &Path) -> Result<()> {
    fs::create_dir_all(root).map_err(error)?;
    if root.join("box").exists() {
        return Err(error("studio already exists"));
    }
    fs::write(root.join("box"), "watercolor\n").map_err(error)
}

pub fn load(root: &Path) -> Result<Session> {
    if fs::read_to_string(root.join("box")).map_err(error)?.trim() != "watercolor" {
        return Err(error("not an experimental watercolor studio"));
    }
    if root.join("checkpoint.json").exists() {
        Session::decode(&fs::read(root.join("checkpoint.json")).map_err(error)?)
    } else {
        Ok(Session::default())
    }
}

pub fn restore(root: &Path, name: &str) -> Result<()> {
    basename(name)?;
    if !name.ends_with(".json") {
        return Err(error("checkpoint filename must end .json"));
    }
    let lock = root.join(".writer-lock");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock)
        .map_err(error)?;
    let _guard = Lock(lock);
    load(root)?;
    let state = Session::decode(&fs::read(root.join(name)).map_err(error)?)?;
    let temp = root.join(format!(".restore-{}", std::process::id()));
    if temp.exists() {
        return Err(error(
            "an interrupted restore file exists; inspect it before retrying",
        ));
    }
    let result = (|| {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)
            .map_err(error)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &state).map_err(error)?;
        writer.flush().map_err(error)?;
        writer.get_ref().sync_all().map_err(error)?;
        fs::rename(&temp, root.join("checkpoint.json")).map_err(error)
    })();
    let _ = fs::remove_file(temp);
    result
}

pub fn run(root: &Path, source: &str) -> Result<Session> {
    let lock = root.join(".writer-lock");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock)
        .map_err(error)?;
    let _guard = Lock(lock);
    let mut state = load(root)?;
    if let Some(canvas) = &mut state.canvas {
        if canvas.schema == 5 {
            canvas.lab.as_mut().unwrap().require_gpu().map_err(error)?;
        }
    }
    if state.applied_sources.iter().any(|s| s == source) {
        return Err(error(
            "this exact chunk already ran; submit a new continuation chunk",
        ));
    }
    let stage = root.join(format!(".pending-{}", std::process::id()));
    if stage.exists() {
        return Err(error(
            "pending files from an interrupted run; inspect them before retrying",
        ));
    }
    fs::create_dir(&stage).map_err(error)?;
    let ctx = Rc::new(RefCell::new(Context {
        state,
        stage: stage.clone(),
        frames: None,
        rendered_frames: 0,
    }));
    let result = (|| {
        let lua = Lua::new_with(
            StdLib::MATH | StdLib::TABLE | StdLib::STRING | StdLib::UTF8,
            LuaOptions::default(),
        )?;
        lua.set_memory_limit(128 * 1024 * 1024)?;
        lua.globals().set("dofile", Value::Nil)?;
        lua.globals().set("loadfile", Value::Nil)?;
        let budget = Cell::new(0u32);
        lua.set_hook(
            mlua::HookTriggers::new().every_nth_instruction(10_000),
            move |_, _| {
                budget.set(budget.get() + 1);
                if budget.get() > 200 {
                    return Err(error("Lua instruction budget exceeded"));
                }
                Ok(mlua::VmState::Continue)
            },
        )?;
        install(&lua, ctx.clone())?;
        lua.load(source).set_name("watercolor chunk").exec()?;
        let mut c = ctx.borrow_mut();
        if c.state.canvas.is_none() {
            return Err(error("chunk made no paper"));
        }
        c.state.check_balance()?;
        c.state.applied_sources.push(source.to_owned());
        write_state(&stage.join("checkpoint.json"), &c.state)?;
        for entry in fs::read_dir(&stage).map_err(error)? {
            let entry = entry.map_err(error)?;
            if entry.file_name() != "checkpoint.json" {
                fs::rename(entry.path(), root.join(entry.file_name())).map_err(error)?;
            }
        }
        fs::rename(stage.join("checkpoint.json"), root.join("checkpoint.json")).map_err(error)?;
        Ok(())
    })();
    let _ = fs::remove_dir_all(stage);
    result.map(|()| {
        Rc::try_unwrap(ctx)
            .ok()
            .expect("context still held")
            .into_inner()
            .state
    })
}
