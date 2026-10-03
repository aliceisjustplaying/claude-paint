//! The painter's Lua API over the engine.
//!
//! Everything a chunk can call is registered here as globals. The state a
//! painting builds up (the canvas, the style, the clock, the brushes in the
//! hand, the piles on the palette) lives in `Studio`, shared by the
//! closures through an `Rc`.
//!
//! Paint reaches the canvas only as piles: parts of named tubes knifed
//! together (`pile{}`), loaded on a brush (`b:load(p)`) or dipped into by
//! a covering pass (`work{pile=p}`, `stipple{pile=p}`, a brushed glaze
//! `work(m, {hand="glaze", pile=p})`), and a ground made of a pile. What a pile looks like on the canvas is what
//! the engine's pigment physics makes of it.
//!
//! Painter functions (angle and coverage fields) are Lua closures, which
//! can't run on the engine's rayon threads, so they are sampled serially onto
//! a grid in canvas units first (`FIELD_STEP`, over the mask's bounding box)
//! and read back bilinearly. Mask functions are evaluated at every pixel.

use mlua::{Function, Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value, Variadic};
use paint::palette::Mixture;
use paint::{Apply, Canvas, Fbm, Frame, Gesture, Ground, Handling, Held, Linen, Mask, Order, Orient, Palette, Rgb, Rng, Shape, Stipple, Style, Tool, Touch};
use crate::time::{self, Verb};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[path = "draw_pencil.rs"]
mod draw_pencil;

/// Grid spacing (units) that painter fields are sampled on.
pub const FIELD_STEP: f32 = 2.0;

pub struct Studio {
    pub width: usize,
    pub canvas: Option<Canvas>,
    pub style: Option<Rc<Style>>,
    /// Arguments `canvas{}` was called with (for status).
    pub setup: Option<String>,
    pub seed: u64,
    /// Index of the chunk being run (1-based), for seeds.
    pub chunk: u64,
    /// Engine calls made in this chunk, for automatic seeds.
    pub calls: u64,
    /// Painting time in minutes since `canvas{}` (hand time and waits).
    pub clock: f64,
    pub clock0: f64,
    pub rng: Rng,
    pub brushes: Vec<Weak<RefCell<Held>>>,
    pub out: String,
    /// Time spent evaluating Lua fields in this chunk (s).
    pub field_secs: f64,
    /// The last world view made (`w:view()`): what `visible=`, `behind=` and
    /// `at=` resolve against (depth.rs).
    pub view: Option<crate::world::ViewU>,
    /// The hand's state: the piles on the palette (time.rs).
    pub hand: crate::time::Hand,
    /// The tubes piles are knifed from.
    pub tubes: Rc<Palette>,
}

impl Studio {
    pub fn new(width: usize, tubes: Palette) -> Self {
        Studio { width, canvas: None, style: None, setup: None, seed: 1, chunk: 0, calls: 0, clock: 0.0, clock0: 0.0, rng: Rng::new(1), brushes: Vec::new(), out: String::new(), field_secs: 0.0, view: None, hand: crate::time::Hand::default(), tubes: Rc::new(tubes) }
    }
    /// Start chunk `n`: its randomness depends only on the seed and `n`.
    pub fn begin(&mut self, n: u64) {
        self.chunk = n;
        self.calls = 0;
        self.rng = Rng::new(mixseed(self.seed, n, 0xC0FFEE));
        self.out.clear();
        self.field_secs = 0.0;
    }

    pub(crate) fn auto_seed(&mut self) -> u64 {
        self.calls += 1;
        mixseed(self.seed, self.chunk, self.calls)
    }

    pub fn live_brushes(&mut self) -> Vec<Rc<RefCell<Held>>> {
        self.brushes.retain(|w| w.strong_count() > 0);
        self.brushes.iter().filter_map(|w| w.upgrade()).collect()
    }
}

pub fn mixseed(a: u64, b: u64, c: u64) -> u64 {
    let mut z = a.wrapping_mul(0x9E3779B97F4A7C15) ^ b.wrapping_mul(0xBF58476D1CE4E5B9) ^ c.wrapping_mul(0x94D049BB133111EB);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

pub(crate) type S = Rc<RefCell<Studio>>;

pub(crate) fn err<T>(msg: impl Into<String>) -> Result<T> {
    Err(mlua::Error::runtime(msg.into()))
}

// ---------------------------------------------------------------- fields

/// A painter field sampled on a grid in canvas units.
#[derive(Clone)]
pub(crate) struct Grid<const N: usize> {
    x0: f32,
    y0: f32,
    step: f32,
    nx: usize,
    ny: usize,
    v: Vec<[f32; N]>,
}

impl<const N: usize> Grid<N> {
    pub(crate) fn get(&self, x: f32, y: f32) -> [f32; N] {
        let fx = ((x - self.x0) / self.step).clamp(0.0, (self.nx - 1) as f32);
        let fy = ((y - self.y0) / self.step).clamp(0.0, (self.ny - 1) as f32);
        let (i, j) = ((fx as usize).min(self.nx.saturating_sub(2)), (fy as usize).min(self.ny.saturating_sub(2)));
        let (tx, ty) = (fx - i as f32, fy - j as f32);
        let at = |a: usize, b: usize| self.v[b.min(self.ny - 1) * self.nx + a.min(self.nx - 1)];
        let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
        let mut o = [0.0; N];
        for k in 0..N {
            o[k] = (a[k] * (1.0 - tx) + b[k] * tx) * (1.0 - ty) + (c[k] * (1.0 - tx) + d[k] * tx) * ty;
        }
        o
    }
}

/// Box (units) a field is needed in: the mask's support grown by `pad`.
pub(crate) fn support(m: Option<&Mask>, f: Frame, pad: f32) -> (f32, f32, f32, f32) {
    let (w, h) = (f.width(), f.height());
    let Some(m) = m else { return (0.0, 0.0, w, h) };
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for (i, &v) in m.data.iter().enumerate() {
        if v > 0.0 {
            let (x, y) = (i % f.w, i / f.w);
            x0 = x0.min(x);
            x1 = x1.max(x);
            y0 = y0.min(y);
            y1 = y1.max(y);
        }
    }
    if x0 == usize::MAX {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let s = f.scale;
    (((x0 as f32) / s - pad).max(0.0), ((y0 as f32) / s - pad).max(0.0), ((x1 + 1) as f32 / s + pad).min(w), ((y1 + 1) as f32 / s + pad).min(h))
}

pub(crate) fn sample<const N: usize>(st: &S, fun: &Function, b: (f32, f32, f32, f32), conv: impl Fn(Value) -> Result<[f32; N]>) -> Result<Grid<N>> {
    let t0 = std::time::Instant::now();
    let step = FIELD_STEP;
    let nx = (((b.2 - b.0) / step).ceil() as usize + 1).max(2);
    let ny = (((b.3 - b.1) / step).ceil() as usize + 1).max(2);
    let mut v = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            // nodes past the box (the last row and column) sample just inside
            // it: a field is never asked about the canvas's own edge or beyond
            let (x, y) = ((b.0 + i as f32 * step).min(b.2 - 0.01).max(b.0), (b.1 + j as f32 * step).min(b.3 - 0.01).max(b.1));
            let r: Value = fun.call((x, y))?;
            v.push(conv(r)?);
        }
    }
    st.borrow_mut().field_secs += t0.elapsed().as_secs_f64();
    Ok(Grid { x0: b.0, y0: b.1, step, nx, ny, v })
}

/// `f` sampled on the field grid over `b`, as `sample` samples a Lua function.
#[cfg(feature = "replay")]
pub(crate) fn grid<const N: usize>(st: &S, b: (f32, f32, f32, f32), mut f: impl FnMut(f32, f32) -> Result<[f32; N]>) -> Result<Grid<N>> {
    let t0 = std::time::Instant::now();
    let step = FIELD_STEP;
    let nx = (((b.2 - b.0) / step).ceil() as usize + 1).max(2);
    let ny = (((b.3 - b.1) / step).ceil() as usize + 1).max(2);
    let mut v = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            v.push(f((b.0 + i as f32 * step).min(b.2 - 0.01).max(b.0), (b.1 + j as f32 * step).min(b.3 - 0.01).max(b.1))?);
        }
    }
    st.borrow_mut().field_secs += t0.elapsed().as_secs_f64();
    Ok(Grid { x0: b.0, y0: b.1, step, nx, ny, v })
}

pub(crate) type FieldBox<T> = Box<dyn Fn(f32, f32) -> T + Sync>;

pub(crate) fn scalar_field(st: &S, v: &Value, b: (f32, f32, f32, f32), what: &str) -> Result<FieldBox<f32>> {
    // a noise: read natively (0..1)
    if let Value::UserData(u) = v
        && let Ok(n) = u.borrow::<Noise>()
    {
        let n = *n;
        return Ok(Box::new(move |x, y| n.get01(x, y)));
    }
    match v {
        Value::Function(f) => {
            let what = what.to_string();
            let g = sample::<1>(st, f, b, |r| match r {
                Value::Number(n) => Ok([n as f32]),
                Value::Integer(n) => Ok([n as f32]),
                o => err(format!("{what} function returned {}, want a number", o.type_name())),
            })?;
            Ok(Box::new(move |x, y| g.get(x, y)[0]))
        }
        Value::Number(n) => {
            let n = *n as f32;
            Ok(Box::new(move |_, _| n))
        }
        Value::Integer(n) => {
            let n = *n as f32;
            Ok(Box::new(move |_, _| n))
        }
        o => err(format!("{what}: want a number or a function(x, y), got {}", o.type_name())),
    }
}

/// Angles are interpolated as directions (cos, sin), so a field that wraps
/// through ±π stays smooth.
fn angle_field(st: &S, v: &Value, b: (f32, f32, f32, f32)) -> Result<FieldBox<f32>> {
    // a form's own field (f:field("fall")): read natively, no grid
    if let Value::UserData(u) = v
        && let Ok(fu) = u.borrow::<crate::form::FieldU>()
    {
        return Ok(fu.angle(0.0));
    }
    // an old log's rock field (r:field("plane"), legacy.rs): read natively too
    #[cfg(feature = "replay")]
    if let Some(f) = crate::legacy::angle_field(v) {
        return Ok(f);
    }
    if let Value::Function(f) = v {
        let g = sample::<2>(st, f, b, |r| {
            let a = match r {
                Value::Number(n) => n as f32,
                Value::Integer(n) => n as f32,
                o => return err(format!("angle function returned {}, want radians", o.type_name())),
            };
            Ok([a.cos(), a.sin()])
        })?;
        Ok(Box::new(move |x, y| {
            let [c, s] = g.get(x, y);
            s.atan2(c)
        }))
    } else {
        scalar_field(st, v, b, "angle")
    }
}

// ---------------------------------------------------------------- tables

pub(crate) fn num(t: &Table, k: &str) -> Result<Option<f32>> {
    t.get::<Option<f32>>(k)
}

/// A pair from {a, b} or a single number (both the same).
pub(crate) fn pair(t: &Table, k: &str) -> Result<Option<(f32, f32)>> {
    match t.get::<Value>(k)? {
        Value::Nil => Ok(None),
        Value::Number(n) => Ok(Some((n as f32, n as f32))),
        Value::Integer(n) => Ok(Some((n as f32, n as f32))),
        Value::Table(p) => Ok(Some((p.get(1)?, p.get(2)?))),
        o => err(format!("{k}: want a number or {{a, b}}, got {}", o.type_name())),
    }
}

/// Points from {{x, y}, ...} or {x1, y1, x2, y2, ...}.
pub(crate) fn points(v: &Value) -> Result<Vec<(f32, f32)>> {
    let Value::Table(t) = v else { return err("points: want {{x, y}, ...} or {x1, y1, x2, y2, ...}") };
    let mut out = Vec::new();
    match t.get::<Value>(1)? {
        Value::Table(_) => {
            for p in t.sequence_values::<Table>() {
                let p = p?;
                out.push((p.get(1)?, p.get(2)?));
            }
        }
        _ => {
            let v: Vec<f32> = t.sequence_values::<f32>().collect::<Result<_>>()?;
            if !v.len().is_multiple_of(2) {
                return err("points: a flat list needs an even count (x1, y1, x2, y2, ...)");
            }
            out = v.chunks(2).map(|c| (c[0], c[1])).collect();
        }
    }
    Ok(out)
}

pub(crate) fn check_keys(t: &Table, allowed: &[&str], what: &str) -> Result<()> {
    for kv in t.clone().pairs::<Value, Value>() {
        let (k, _) = kv?;
        if let Value::String(s) = &k {
            let s = s.to_str()?.to_string();
            if !allowed.contains(&s.as_str()) {
                return err(format!("{what}: unknown option {s:?} (options: {})", allowed.join(", ")));
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- tools

const TOOL_KINDS: &str = "round, flat, filbert, fan, rigger, badger, stippler";

fn tool_named(kind: &str, width: f32) -> Result<Tool> {
    if !(width > 0.0 && width.is_finite()) {
        return err(format!("brush width {width}: want > 0 (canvas units; the canvas is 1000 wide)"));
    }
    Ok(match kind {
        "round" | "sable" => Tool::round_sable(width),
        "flat" | "hog" => Tool::hog_flat(width),
        "filbert" => Tool::filbert(width),
        "fan" => Tool::fan(width),
        "rigger" | "liner" => Tool::rigger(width),
        "badger" | "blender" => Tool::badger(width),
        "stippler" => Tool::stippler(width),
        o => return err(format!("brush kind {o:?}: use one of {TOOL_KINDS}")),
    })
}

/// A tool from a `Brush`, "filbert 8", or {kind, width, stiffness=...}.
pub(crate) fn tool_of(v: &Value) -> Result<Tool> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<Brush>()?.held.borrow().tool.clone()),
        Value::String(s) => {
            let s = s.to_str()?;
            let mut it = s.split_whitespace();
            let kind = it.next().unwrap_or("");
            let w: f32 = it.next().and_then(|w| w.parse().ok()).ok_or_else(|| mlua::Error::runtime(format!("tool {s:?}: want \"kind width\", e.g. \"filbert 8\"")))?;
            tool_named(kind, w)
        }
        Value::Table(t) => {
            let kind: String = t.get::<Option<String>>("kind")?.or(t.get::<Option<String>>(1)?).unwrap_or_else(|| "round".into());
            let w: f32 = t.get::<Option<f32>>("width")?.or(t.get::<Option<f32>>(2)?).ok_or_else(|| mlua::Error::runtime("tool: needs a width"))?;
            let mut tool = tool_named(&kind, w)?;
            macro_rules! over {
                ($($f:ident),*) => {$( if let Some(v) = num(t, stringify!($f))? { tool.$f = v; } )*};
            }
            over!(length, stiffness, hair, run, lay, pickup, push, splay, ragged, point);
            if let Some(b) = t.get::<Option<usize>>("bristles")? {
                tool.bristles = b;
            }
            tool.validate().map_err(mlua::Error::runtime)?;
            Ok(tool)
        }
        o => err(format!("tool: want a brush, \"kind width\" or {{kind=, width=}}, got {}", o.type_name())),
    }
}

fn orient_of(v: Value) -> Result<Option<Orient>> {
    Ok(match v {
        Value::Nil => None,
        Value::String(s) => match &*s.to_str()? {
            "across" => Some(Orient::Across),
            "along" => Some(Orient::Along),
            o => return err(format!("orient {o:?}: \"across\", \"along\" or an angle")),
        },
        Value::Number(n) => Some(Orient::Fixed(n as f32)),
        Value::Integer(n) => Some(Orient::Fixed(n as f32)),
        o => return err(format!("orient: got {}", o.type_name())),
    })
}

/// A brush in the hand: it keeps its paint between strokes and chunks.
#[derive(Clone)]
pub struct Brush {
    held: Rc<RefCell<Held>>,
    st: S,
}

impl UserData for Brush {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("width", |_, b| Ok(b.held.borrow().tool.width));
        f.add_field_method_get("kind", |_, b| Ok(format!("{:?}", b.held.borrow().tool.kind).to_lowercase()));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // b:load(pile, amount?): dip into a pile on the palette (amount 0..1 of a full load)
        m.add_method("load", |_, b, (p, amount, extra): (Value, Option<f32>, Value)| {
            let (paint, color) = brushload(&b.st, &p, &extra, "load")?;
            b.held.borrow_mut().load(paint, amount.unwrap_or(0.8));
            time::trip(&b.st, color);
            Ok(())
        });
        // b:reload(pile, amount?): wipe most of the old paint off, then load
        m.add_method("reload", |_, b, (p, amount, extra): (Value, Option<f32>, Value)| {
            let (paint, color) = brushload(&b.st, &p, &extra, "reload")?;
            b.held.borrow_mut().reload(paint, amount.unwrap_or(0.8));
            time::trip(&b.st, color);
            Ok(())
        });
        m.add_method("wipe", |_, b, frac: Option<f32>| {
            b.held.borrow_mut().wipe(frac.unwrap_or(0.85));
            if let Some(c) = b.st.borrow_mut().canvas.as_mut() {
                c.tally_mut().wipe();
            }
            Ok(())
        });
        m.add_method("fullness", |_, b, ()| Ok(b.held.borrow().fullness()));
        // pointed tips: how wide a mark at this pressure, what pressure for this width
        m.add_method("mark_width", |_, b, p: f32| Ok(b.held.borrow().tool.mark_width(p)));
        m.add_method("pressure_for", |_, b, w: f32| Ok(b.held.borrow().tool.pressure_for(w)));
        // b:stroke(points, {pressure=, ramps=, orient=, shake=, swell=, clip=})
        m.add_method("stroke", |_, b, (pts, o): (Value, Option<Table>)| {
            let pts = points(&pts)?;
            if pts.len() < 2 {
                return err("stroke: needs at least two points");
            }
            let mut g = Gesture::new(pts);
            let mut clip = None;
            if let Some(o) = &o {
                check_keys(o, &["pressure", "ramps", "orient", "shake", "swell", "clip"], "stroke")?;
                if let Some((a, z)) = pair(o, "pressure")? {
                    g = g.pressure(a, z);
                }
                if let Some((a, z)) = pair(o, "ramps")? {
                    g = g.ramps(a, z);
                }
                if let Some(or) = orient_of(o.get("orient")?)? {
                    g = g.orient(or);
                }
                if let Some(s) = num(o, "shake")? {
                    g = g.shake(s);
                }
                if let Some(s) = o.get::<Option<Vec<f32>>>("swell")? {
                    g = g.swell(s);
                }
                clip = mask_opt(o.get("clip")?)?;
            }
            time::verb(&b.st, Verb::Marks, |s| {
                s.canvas.as_mut().ok_or_else(no_canvas)?.drag(&mut b.held.borrow_mut(), &g, clip.as_deref());
                Ok(())
            })
        });
        // b:touch(x, y, {pressure=, drag={dx,dy}, twist=, angle=, clip=})
        m.add_method("touch", |_, b, (x, y, o): (f32, f32, Option<Table>)| {
            let mut t = Touch::at(x, y);
            let mut clip = None;
            if let Some(o) = &o {
                check_keys(o, &["pressure", "drag", "twist", "angle", "clip"], "touch")?;
                if let Some(p) = num(o, "pressure")? {
                    t = t.pressure(p);
                }
                if let Some((dx, dy)) = pair(o, "drag")? {
                    t = t.drag(dx, dy);
                }
                if let Some(a) = num(o, "twist")? {
                    t = t.twist(a);
                }
                if let Some(a) = num(o, "angle")? {
                    t = t.angle(a);
                }
                clip = mask_opt(o.get("clip")?)?;
            }
            time::verb(&b.st, Verb::Marks, |s| {
                s.canvas.as_mut().ok_or_else(no_canvas)?.touch(&mut b.held.borrow_mut(), &t, clip.as_deref());
                Ok(())
            })
        });
        m.add_meta_method(MetaMethod::ToString, |_, b, ()| {
            let h = b.held.borrow();
            Ok(format!("brush({:?} {}, {:.0}% full)", h.tool.kind, h.tool.width, 100.0 * h.fullness()).to_lowercase())
        });
    }
}

pub(crate) fn no_canvas() -> mlua::Error {
    mlua::Error::runtime("no canvas yet: the first chunk is canvas{size=, aspect=, linen=, ground=}")
}

pub(crate) fn style(st: &S) -> Result<Rc<Style>> {
    st.borrow().style.clone().ok_or_else(no_canvas)
}

pub(crate) fn frame(st: &S) -> Result<Frame> {
    Ok(st.borrow().canvas.as_ref().ok_or_else(no_canvas)?.frame())
}

// ---------------------------------------------------------------- piles

/// A pile on the palette: parts of tubes knifed together, with the oil
/// medium mixed into it.
#[derive(Clone)]
pub struct PileU {
    pub mix: Mixture,
    pub medium: f32,
    /// The parts as the painter gave them (for printing).
    parts: Vec<(String, f32)>,
}

impl UserData for PileU {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("medium", |_, p| Ok(p.medium));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("parts", |lua, p, ()| {
            let t = lua.create_table()?;
            for (name, k) in &p.parts {
                t.push(lua.create_sequence_from([Value::String(lua.create_string(name)?), Value::Number(*k as f64)])?)?;
            }
            Ok(t)
        });
        m.add_meta_method(MetaMethod::ToString, |_, p, ()| {
            let parts: Vec<String> = p.parts.iter().map(|(n, k)| format!("{n} {}", fmt_num(*k))).collect();
            Ok(format!("pile({}; medium {})", parts.join(", "), fmt_num(p.medium)))
        });
    }
}

fn fmt_num(v: f32) -> String {
    if (v - v.round()).abs() < 1e-6 { format!("{}", v.round() as i64) } else { format!("{v}") }
}

/// Parts of tubes from `{{"lead white", 6}, {"smalt", 1}, ...}` (the table's
/// array part): tube indices and fractions by volume summing to 1, and the
/// parts as given.
fn parts_of(tubes: &Palette, t: &Table, what: &str) -> Result<(Vec<(usize, f32)>, Vec<(String, f32)>)> {
    let mut parts: Vec<(usize, f32)> = Vec::new();
    let mut given = Vec::new();
    for e in t.sequence_values::<Value>() {
        let Value::Table(e) = e? else {
            return err(format!("{what}: each part is {{\"tube name\", parts}} (tubes() lists the names)"));
        };
        let name: String = e.get::<Option<String>>(1)?.ok_or_else(|| mlua::Error::runtime(format!("{what}: each part is {{\"tube name\", parts}}")))?;
        let k: f32 = e.get::<Option<f32>>(2)?.ok_or_else(|| mlua::Error::runtime(format!("{what}: {name:?} needs a number of parts")))?;
        if !(k > 0.0 && k.is_finite()) {
            return err(format!("{what}: {name:?}: parts > 0"));
        }
        let i = tubes.tubes.iter().position(|t| t.name == name).ok_or_else(|| {
            mlua::Error::runtime(format!("{what}: no tube {name:?} (tubes: {})", tubes.tubes.iter().map(|t| t.name).collect::<Vec<_>>().join(", ")))
        })?;
        match parts.iter_mut().find(|p| p.0 == i) {
            Some(p) => p.1 += k,
            None => parts.push((i, k)),
        }
        given.push((name, k));
    }
    if parts.is_empty() {
        return err(format!("{what}: needs at least one tube: {{{{\"tube name\", parts}}, ...}} (tubes() lists the names)"));
    }
    let sum: f32 = parts.iter().map(|p| p.1).sum();
    for p in parts.iter_mut() {
        p.1 /= sum;
    }
    Ok((parts, given))
}

pub(crate) fn pile_of(v: &Value, what: &str) -> Result<PileU> {
    match v {
        Value::UserData(u) if u.borrow::<PileU>().is_ok() => Ok(u.borrow::<PileU>()?.clone()),
        Value::Nil => err(format!("{what}: needs a pile (p = pile{{{{\"tube name\", parts}}, ...}})")),
        o => err(format!("{what}: want a pile (made with pile{{...}}), got {}", o.type_name())),
    }
}

/// Paint for one brushload from a pile: the pile, remixed a little (a pile
/// knifed by hand is uneven), and the pile's color (for the palette ledger).
fn brushload(st: &S, p: &Value, extra: &Value, what: &str) -> Result<(paint::Paint, Rgb)> {
    // a legacy canvas's brushes load colors (legacy.rs)
    #[cfg(feature = "replay")]
    if let Some(r) = crate::legacy::brushload(st, p, extra)? {
        return Ok(r);
    }
    if !extra.is_nil() {
        return err(format!("b:{what}(pile, amount): a pile carries its own medium; mix another pile for other paint"));
    }
    let p = pile_of(p, &format!("b:{what}"))?;
    let sty = style(st)?;
    let mut s = st.borrow_mut();
    let tubes = s.tubes.clone();
    let seed = s.rng.next_u64();
    let m = tubes.remix(&p.mix, sty.mix_jitter, &mut Rng::new(seed));
    Ok((m.laid(p.medium), p.mix.color))
}

// ---------------------------------------------------------------- masks

#[derive(Clone)]
pub struct M(pub Rc<Mask>);

pub(crate) fn mask_of(v: &Value) -> Result<Rc<Mask>> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<M>()?.0.clone()),
        o => err(format!("want a mask, got {} (make one with mask(fn), ellipse, poly, rect, below, above, ribbon, everywhere)", o.type_name())),
    }
}
fn mask_opt(v: Value) -> Result<Option<Rc<Mask>>> {
    match v {
        Value::Nil => Ok(None),
        v => mask_of(&v).map(Some),
    }
}

thread_local! {
    /// The Lua state (to collect garbage) and the mask bytes made since the
    /// last collection: Lua doesn't see how big a mask is (29 MB at 3200px),
    /// so without this a loop of mask operations piles up gigabytes.
    static GC: RefCell<(Option<mlua::WeakLua>, usize)> = const { RefCell::new((None, 0)) };
}
const GC_EVERY_BYTES: usize = 400 << 20;

pub(crate) fn wrap(m: Mask) -> M {
    note_bytes(m.data.len() * 4);
    M(Rc::new(m))
}

/// Count memory handed to Lua values; collect garbage every `GC_EVERY_BYTES`.
pub(crate) fn note_bytes(bytes: usize) {
    let lua = GC.with(|g| {
        let mut g = g.borrow_mut();
        g.1 += bytes;
        if g.1 > GC_EVERY_BYTES {
            g.1 = 0;
            g.0.as_ref().and_then(|w| w.try_upgrade())
        } else {
            None
        }
    });
    if let Some(l) = lua {
        let _ = l.gc_collect();
    }
}

impl UserData for M {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("blur", |_, a, r: f32| Ok(wrap((*a.0).clone().blur(r))));
        // m:roughen(units, period?, seed?, edge?): the edge moves in and out
        // by up to about `units` (noise of `period` units), ramping over `edge`
        m.add_method("roughen", |_, a, (amount, period, seed, edge): (f32, Option<f32>, Option<u32>, Option<f32>)| {
            Ok(wrap((*a.0).clone().roughen(seed.unwrap_or(1), period.unwrap_or(40.0), amount, edge.unwrap_or(0.0))))
        });
        // m:soften(units): a soft edge that many units wide
        m.add_method("soften", |_, a, w: f32| Ok(wrap(a.0.soften(move |_, _| w))));
        m.add_method("offset", |_, a, d: f32| Ok(wrap(a.0.offset(d))));
        m.add_method("grow", |_, a, d: f32| Ok(wrap(a.0.dilate(d))));
        m.add_method("shrink", |_, a, d: f32| Ok(wrap(a.0.erode(d))));
        m.add_method("rim", |_, a, (w, soft): (f32, Option<f32>)| Ok(wrap(a.0.rim(w, soft.unwrap_or(1.0)))));
        m.add_method("band", |_, a, (lo, hi, soft): (f32, f32, Option<f32>)| Ok(wrap((*a.0).clone().band(lo, hi, soft.unwrap_or(1.0)))));
        m.add_method("distance", |_, a, ()| Ok(wrap(a.0.distance())));
        m.add_method("invert", |_, a, ()| Ok(wrap((*a.0).clone().invert())));
        m.add_method("at", |_, a, (x, y): (f32, f32)| Ok(a.0.sample(x, y)));
        m.add_method("area", |_, a, ()| {
            let s = a.0.f.scale;
            Ok(a.0.data.iter().map(|&v| v as f64).sum::<f64>() / (s as f64 * s as f64))
        });
        // m:times(fn(x, y) or mask), m:map(fn(v))
        m.add_method("times", |_, a, g: Value| match g {
            Value::Function(f) => Ok(wrap((*a.0).clone().mul(&eval_mask(a.0.f, &f)?))),
            v => Ok(wrap((*a.0).clone().mul(&*mask_of(&v)?))),
        });
        m.add_method("map", |_, a, f: Function| {
            let mut out = (*a.0).clone();
            for v in out.data.iter_mut() {
                *v = f.call(*v)?;
            }
            Ok(wrap(out))
        });
        m.add_meta_method(MetaMethod::Add, |_, a, b: Value| Ok(wrap((*a.0).clone().union(&*mask_of(&b)?))));
        m.add_meta_method(MetaMethod::Mul, |_, a, b: Value| Ok(wrap((*a.0).clone().mul(&*mask_of(&b)?))));
        m.add_meta_method(MetaMethod::Sub, |_, a, b: Value| Ok(wrap((*a.0).clone().subtract(&*mask_of(&b)?))));
        m.add_meta_method(MetaMethod::Unm, |_, a, ()| Ok(wrap((*a.0).clone().invert())));
        m.add_meta_method(MetaMethod::ToString, |_, a, ()| {
            let s = a.0.f.scale;
            let area = a.0.data.iter().map(|&v| v as f64).sum::<f64>() / (s as f64 * s as f64);
            Ok(format!("mask({area:.0} sq units)"))
        });
    }
}

/// A painter's mask function at every pixel center (serial: Lua).
fn eval_mask(f: Frame, g: &Function) -> Result<Mask> {
    let mut data = vec![0.0f32; f.w * f.h];
    let inv = 1.0 / f.scale;
    for y in 0..f.h {
        let yu = (y as f32 + 0.5) * inv;
        for x in 0..f.w {
            let v: f32 = g.call(((x as f32 + 0.5) * inv, yu))?;
            data[y * f.w + x] = v.clamp(0.0, 1.0);
        }
    }
    Ok(Mask { f, data })
}

pub(crate) fn curve_of(v: &Value, w: f32) -> Result<Vec<(f32, f32)>> {
    match v {
        Value::Function(f) => {
            let n = 250;
            (0..=n).map(|i| {
                let x = w * i as f32 / n as f32;
                Ok((x, f.call::<f32>(x)?))
            }).collect()
        }
        v => points(v),
    }
}

// ---------------------------------------------------------------- noise

#[derive(Clone, Copy)]
pub enum NoiseKind {
    Fbm(Fbm),
    Octaves(paint::noise::Octaves),
}

/// A noise field: fBm, ridged or billowed octaves, optionally seen through
/// a domain warp and stretched along a direction.
#[derive(Clone, Copy)]
pub struct Noise {
    kind: NoiseKind,
    warp: Option<paint::noise::Warp>,
    stretch: Option<paint::noise::Aniso>,
}

impl Noise {
    fn get(&self, x: f32, y: f32) -> f32 {
        let (mut x, mut y) = (x, y);
        if let Some(a) = &self.stretch {
            (x, y) = a.at(x, y);
        }
        if let Some(w) = &self.warp {
            (x, y) = w.at(x, y);
        }
        match &self.kind {
            NoiseKind::Fbm(f) => f.get(x, y),
            NoiseKind::Octaves(o) => o.get(x, y),
        }
    }
    fn get01(&self, x: f32, y: f32) -> f32 {
        match (&self.kind, &self.warp, &self.stretch) {
            (NoiseKind::Fbm(f), None, None) => f.get01(x, y),
            _ => (self.get(x, y) * 0.5 + 0.5).clamp(0.0, 1.0),
        }
    }
}

impl UserData for Noise {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::Call, |_, n, (x, y): (f32, f32)| Ok(n.get(x, y)));
        m.add_method("at", |_, n, (x, y): (f32, f32)| Ok(n.get(x, y)));
        m.add_method("at01", |_, n, (x, y): (f32, f32)| Ok(n.get01(x, y)));
    }
}

pub struct WorleyU(paint::noise::Worley);
impl UserData for WorleyU {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // c:at(x, y) -> f1, f2, edge, rand: distances to the nearest two cell
        // points (units), how near a cell edge (0 on it), and a stable 0..1 per cell
        m.add_method("at", |_, w, (x, y): (f32, f32)| {
            let c = w.0.get(x, y);
            Ok((c.f1, c.f2, c.edge(), c.rand()))
        });
        m.add_meta_method(MetaMethod::Call, |_, w, (x, y): (f32, f32)| Ok(w.0.get(x, y).f1));
    }
}

// ---------------------------------------------------------------- handling

const WORK_KEYS: &[&str] = &[
    "hand", "pile", "tool", "length", "coverage", "angle", "angle_jitter", "load_at", "cut_in", "pressure", "orient", "dips", "blender", "scrub", "clip",
    "threshold", "ramps", "shake", "curve", "cross", "drift", "tail", "broken", "swell", "clump", "order", "mix_jitter", "seed", "ruler", "load", "hug",
    "fill", "visible", "behind", "at", "view", "edge",
];

const EDGE_KEYS: &[&str] = &["found", "soft", "lost", "period", "seed", "quality", "waver", "reach"];

/// A named edge quality (0 found .. 1 lost).
fn edge_name(s: &str) -> Result<f32> {
    Ok(match s {
        "found" | "crisp" | "hard" => 0.0,
        "firm" => 0.25,
        "soft" => 0.5,
        "loose" => 0.75,
        "lost" => 1.0,
        o => return err(format!("edge {o:?}: \"found\", \"firm\", \"soft\", \"loose\" or \"lost\" (or a number 0..1, a function, a mask or a table)")),
    })
}

/// The quality field of `edge=` as a whole-canvas mask (0 found .. 1 lost).
pub(crate) fn edge_quality(st: &S, v: &Value, region: &Mask, b: (f32, f32, f32, f32), seed: u64) -> Result<Mask> {
    let f = region.f;
    match v {
        Value::String(s) => {
            let q = edge_name(&s.to_str()?)?;
            Ok(Mask::from_fn(f, move |_, _| q))
        }
        Value::UserData(u) if u.borrow::<M>().is_ok() => Ok((*u.borrow::<M>()?.0).clone()),
        Value::Table(t) => {
            check_keys(t, EDGE_KEYS, "edge")?;
            if let Some(q) = t.get::<Option<Value>>("quality")? {
                return edge_quality(st, &q, region, b, seed);
            }
            let (fo, so, lo) = (t.get::<Option<f32>>("found")?.unwrap_or(0.0), t.get::<Option<f32>>("soft")?.unwrap_or(0.0), t.get::<Option<f32>>("lost")?.unwrap_or(0.0));
            if !(fo >= 0.0 && so >= 0.0 && lo >= 0.0) || fo + so + lo <= 0.0 {
                return err("edge={found=, soft=, lost=}: shares of the contour, at least one above 0");
            }
            let period = t.get::<Option<f32>>("period")?.unwrap_or(40.0);
            let sd = t.get::<Option<u64>>("seed")?.unwrap_or(seed);
            Ok(paint::fence::stretches(region, (fo, so, lo), period, 6.0, sd))
        }
        v => {
            let g = scalar_field(st, v, b, "edge")?;
            Ok(Mask::from_fn(f, move |x, y| g(x, y).clamp(0.0, 1.0)))
        }
    }
}

fn work(st: &S, mask: Rc<Mask>, o: Table, preset: Option<&str>) -> Result<()> {
    #[cfg(feature = "replay")]
    check_keys(&o, &crate::legacy::work_keys(st, WORK_KEYS), "work")?;
    #[cfg(not(feature = "replay"))]
    check_keys(&o, WORK_KEYS, "work")?;
    let (mask, limit) = crate::depth::restrict_mask(st, &o, mask)?;
    let sty = style(st)?;
    let f = frame(st)?;
    let hand: String = o.get::<Option<String>>("hand")?.unwrap_or_else(|| preset.unwrap_or("body").to_string());
    let blending = hand == "blend" || o.get::<Option<bool>>("blender")?.unwrap_or(false);
    let pile = match o.get::<Value>("pile")? {
        Value::Nil if blending => None,
        // a legacy canvas's passes paint colors (legacy.rs)
        #[cfg(feature = "replay")]
        Value::Nil if crate::legacy::on(st) => None,
        v => Some(pile_of(&v, "work")?),
    };
    let tubes = st.borrow().tubes.clone();
    let mut h: Handling = match hand.as_str() {
        "broad" => sty.broad(),
        "body" => sty.body(),
        "detail" => sty.detail(),
        "hatch" => sty.hatch(),
        "glaze" => sty.glaze(),
        "scumble" => sty.scumble(),
        "blend" => sty.blend().ok_or_else(|| mlua::Error::runtime("this style has no blender"))?,
        o => return err(format!("hand {o:?}: broad, body, detail, hatch, glaze, scumble or blend")),
    };
    if let Some(t) = o.get::<Option<Value>>("tool")? {
        h.tool = tool_of(&t)?;
    }
    let len = h.length.1.max(h.length.0);
    if let Some((a, b)) = pair(&o, "length")? {
        h = h.length(a, b);
    }
    let pad = h.length.1.max(len) + h.tool.width * 2.0;
    let b = support(Some(&mask), f, pad);
    if let Some(c) = o.get::<Option<f32>>("coverage")? {
        h = h.coverage(c);
    }
    if let Some(v) = o.get::<Option<Value>>("angle")? {
        h.angle = angle_field(st, &v, b)?;
    }
    if let Some(a) = num(&o, "angle_jitter")? {
        h = h.angle_jitter(a);
    }
    if let Some(on) = o.get::<Option<bool>>("hug")? {
        h = h.hug(on);
    }
    if let Some(p) = &pile {
        h = h.piled(&tubes, p.mix.clone(), p.medium);
    }
    #[cfg(feature = "replay")]
    if pile.is_none() && crate::legacy::on(st) {
        crate::legacy::work(st, &o, &mut h, &hand, blending, b)?;
    }
    if let Some(on) = o.get::<Option<bool>>("fill")? {
        h = h.fill(on);
    }
    if let Some(v) = o.get::<Option<Value>>("load_at")? {
        h.load_at = Some(scalar_field(st, &v, b, "load_at")?);
    }
    if let Some(t) = o.get::<Option<Value>>("cut_in")? {
        h = h.cut_in(tool_of(&t)?);
    }
    if let Some((a, z)) = pair(&o, "pressure")? {
        h = h.pressure(a, z);
    }
    if let Some(or) = orient_of(o.get("orient")?)? {
        h = h.orient(or);
    }
    if let Some(d) = o.get::<Option<Table>>("dips")? {
        let (e, l, w) = (d.get::<Option<usize>>(1)?.unwrap_or(h.dip_every), d.get::<Option<f32>>(2)?.unwrap_or(h.load), d.get::<Option<f32>>(3)?.unwrap_or(h.wipe));
        h = h.dips(e, l, w);
    }
    if let Some(l) = num(&o, "load")? {
        h = h.load(l);
    }
    if o.get::<Option<bool>>("blender")?.unwrap_or(false) {
        h = h.blender();
    }
    if let Some(n) = o.get::<Option<usize>>("scrub")? {
        h = h.scrub(n);
    }
    let (clip, limit) = clip_opt(&o, limit)?;
    if let Some(c) = clip {
        h = h.clip(c);
    }
    if let Some(t) = num(&o, "threshold")? {
        h = h.threshold(t);
    }
    if let Some((a, r)) = pair(&o, "ramps")? {
        h = h.ramps(a, r);
    }
    if let Some(s) = num(&o, "shake")? {
        h = h.shake(s);
    }
    // (drawn once, here: edge= must not change the pass's strokes)
    let pass_seed = seed_of(st, &o)?;
    // edge=: carry the passage to the region's edge as a brush does (found, soft, lost)
    let edge_v = o.get::<Value>("edge")?;
    if !edge_v.is_nil() {
        if h.cut_in.is_some() {
            return err("work: edge= and cut_in= both say how the edge is made; give one");
        }
        let seed = pass_seed ^ 0xED6E_F00D;
        let q = edge_quality(st, &edge_v, &mask, b, seed)?;
        let (waver, reach) = match &edge_v {
            Value::Table(t) => (t.get::<Option<f32>>("waver")?.unwrap_or(1.0), t.get::<Option<f32>>("reach")?.unwrap_or(1.0)),
            _ => (1.0, 1.0),
        };
        let fence = paint::fence::Fence::new(&mask, &q, h.tool.width, waver, reach, seed);
        h = h.fence(std::sync::Arc::new(fence));
    }
    if o.get::<Option<bool>>("ruler")?.unwrap_or(false) {
        h = h.ruler();
    }
    match o.get::<Value>("curve")? {
        Value::Nil => {}
        Value::Table(t) => {
            let (bow, wave) = (t.get(1)?, t.get::<Option<f32>>(2)?.unwrap_or(h.wave));
            h = h.curve(bow, wave)
        }
        v => {
            let (bow, wave) = (f32::from_lua_value(v)?, h.wave);
            h = h.curve(bow, wave)
        }
    }
    if let Some(c) = num(&o, "cross")? {
        h = h.cross(c);
    }
    if let Some((a, s)) = pair(&o, "drift")? {
        h = h.drift(a, s);
    }
    if let Some(t) = num(&o, "tail")? {
        h = h.tail(t);
    }
    if let Some(t) = num(&o, "broken")? {
        h = h.broken(t);
    }
    if let Some(t) = num(&o, "swell")? {
        h = h.swell(t);
    }
    if let Some(t) = num(&o, "clump")? {
        h = h.clump(t);
    }
    if let Some(t) = num(&o, "mix_jitter")? {
        h = h.mix_jitter(t);
    }
    match o.get::<Value>("order")? {
        Value::Nil => {}
        Value::String(s) => {
            h = h.order(match &*s.to_str()? {
                "passages" => Order::Passages,
                "scatter" => Order::Scatter,
                "down" => Order::Sweep(std::f32::consts::FRAC_PI_2),
                "across" => Order::Sweep(0.0),
                o => return err(format!("order {o:?}: \"passages\", \"scatter\", \"down\", \"across\" or a sweep angle")),
            })
        }
        v => h = h.sweep(f32::from_lua_value(v)?),
    }
    if let Some(l) = limit {
        h = h.limit(l);
    }
    h.tool.validate().map_err(mlua::Error::runtime)?;
    let seed = pass_seed;
    // (dipping into the piles on the palette)
    time::verb(st, Verb::Pass, |s| {
        s.canvas.as_mut().ok_or_else(no_canvas)?.work_with(&mut s.hand.piles, &mask, &h, seed);
        Ok(())
    })
}

pub(crate) trait FromLuaValue: Sized {
    fn from_lua_value(v: Value) -> Result<Self>;
}
impl FromLuaValue for f32 {
    fn from_lua_value(v: Value) -> Result<f32> {
        match v {
            Value::Number(n) => Ok(n as f32),
            Value::Integer(n) => Ok(n as f32),
            o => err(format!("want a number, got {}", o.type_name())),
        }
    }
}

/// `clip=` on a pass: true, false, or a mask the strokes are clipped to,
/// soft edges and all (the deposit is scaled by the mask's value), combined
/// with any limit from `behind=`/`at=`.
fn clip_opt(o: &Table, limit: Option<std::sync::Arc<Mask>>) -> Result<(Option<bool>, Option<std::sync::Arc<Mask>>)> {
    match o.get::<Value>("clip")? {
        Value::Nil => Ok((None, limit)),
        Value::Boolean(b) => Ok((Some(b), limit)),
        Value::UserData(u) if u.borrow::<M>().is_ok() => {
            let m = (*u.borrow::<M>()?.0).clone();
            let m = match limit {
                Some(l) => m.mul(&l),
                None => m,
            };
            Ok((Some(false), Some(std::sync::Arc::new(m))))
        }
        v => err(format!("clip= is true, false or a mask, got {}", v.type_name())),
    }
}

pub(crate) fn seed_of(st: &S, o: &Table) -> Result<u64> {
    Ok(match o.get::<Option<u64>>("seed")? {
        Some(s) => s,
        None => st.borrow_mut().auto_seed(),
    })
}

const STIPPLE_KEYS: &[&str] = &[
    "tool", "width", "pile", "pressure", "coverage", "dips", "drag", "twist", "cluster", "feather", "clip", "mix_jitter", "seed", "visible", "behind", "at", "view",
];

fn stipple(st: &S, mask: Rc<Mask>, o: Table) -> Result<()> {
    #[cfg(feature = "replay")]
    check_keys(&o, &crate::legacy::stipple_keys(st, STIPPLE_KEYS), "stipple")?;
    #[cfg(not(feature = "replay"))]
    check_keys(&o, STIPPLE_KEYS, "stipple")?;
    let (mask, limit) = crate::depth::restrict_mask(st, &o, mask)?;
    let f = frame(st)?;
    let tool = match o.get::<Option<Value>>("tool")? {
        Some(t) => tool_of(&t)?,
        None => Tool::stippler(num(&o, "width")?.unwrap_or(2.0)),
    };
    let b = support(Some(&mask), f, tool.width * 2.0 + 4.0);
    let tubes = st.borrow().tubes.clone();
    // a legacy canvas stipples colors (legacy.rs)
    #[cfg(feature = "replay")]
    let legacy = o.get::<Value>("pile")?.is_nil() && crate::legacy::on(st);
    #[cfg(not(feature = "replay"))]
    let legacy = false;
    let mut sp = if legacy {
        Stipple::new(tool)
    } else {
        let pile = pile_of(&o.get::<Value>("pile")?, "stipple")?;
        Stipple::new(tool).piled(&tubes, pile.mix.clone(), pile.medium)
    };
    #[cfg(feature = "replay")]
    if legacy {
        crate::legacy::stipple(st, &o, &mut sp, b)?;
    }
    // the touches' pressure is the painter's unless `feather` is asked for
    sp = sp.feather(num(&o, "feather")?.unwrap_or(0.0));
    if let Some(v) = o.get::<Option<Value>>("coverage")? {
        sp.coverage = scalar_field(st, &v, b, "coverage")?;
    }
    if let Some((a, z)) = pair(&o, "pressure")? {
        sp = sp.pressure(a, z);
    }
    if let Some(d) = o.get::<Option<Table>>("dips")? {
        let (e, l, w) = (d.get::<Option<usize>>(1)?.unwrap_or(sp.dip_every), d.get::<Option<f32>>(2)?.unwrap_or(sp.load), d.get::<Option<f32>>(3)?.unwrap_or(sp.wipe));
        sp = sp.dips(e, l, w);
    }
    match o.get::<Value>("drag")? {
        Value::Nil => {}
        Value::Table(t) => sp = sp.drag(t.get(1)?, t.get::<Option<f32>>(2)?),
        v => sp = sp.drag(f32::from_lua_value(v)?, None),
    }
    if let Some(t) = num(&o, "twist")? {
        sp = sp.twist(t);
    }
    match o.get::<Value>("cluster")? {
        Value::Nil => {}
        Value::Table(t) => sp = sp.cluster(t.get(1)?, t.get::<Option<f32>>(2)?),
        v => sp = sp.cluster(f32::from_lua_value(v)?, None),
    }
    let (clip, limit) = clip_opt(&o, limit)?;
    if let Some(c) = clip {
        sp = sp.clip(c);
    }
    if let Some(j) = num(&o, "mix_jitter")? {
        sp = sp.mix_jitter(j);
    }
    if let Some(l) = limit {
        sp = sp.limit(l);
    }
    sp.tool.validate().map_err(mlua::Error::runtime)?;
    let seed = seed_of(st, &o)?;
    time::verb(st, Verb::Pass, |s| {
        s.canvas.as_mut().ok_or_else(no_canvas)?.stipple_with(&mut s.hand.piles, &mask, &sp, seed);
        Ok(())
    })
}


/// The longest `wait` (minutes): 10 years of 365.25 days.
const MAX_WAIT_MIN: f64 = 10.0 * 365.25 * 24.0 * 60.0;

pub(crate) fn current_frame(lua: &Lua) -> Result<Frame> {
    let st = lua.app_data_ref::<S>().ok_or_else(|| mlua::Error::runtime("no studio"))?;
    frame(&st)
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    lua.set_app_data(st.clone());
    GC.with(|g| *g.borrow_mut() = (Some(lua.weak()), 0));
    let g = lua.globals();

    // output goes to the chunk's reply
    {
        let st = st.clone();
        g.set(
            "print",
            lua.create_function(move |lua, args: Variadic<Value>| {
                let tostring: Function = lua.globals().get("tostring")?;
                let parts: Vec<String> = args.iter().map(|a| tostring.call::<String>(a.clone())).collect::<Result<_>>()?;
                let mut s = st.borrow_mut();
                s.out.push_str(&parts.join("\t"));
                s.out.push('\n');
                Ok(())
            })?,
        )?;
    }

    // deterministic randomness: reseeded per chunk from the canvas seed
    {
        let math: Table = g.get("math")?;
        let s1 = st.clone();
        math.set(
            "random",
            // Stock Lua 5.4's arguments and errors. Ranges up to 2^24 wide
            // draw from one f32 as they always have (old logs replay the
            // same); wider ones draw 64-bit integers.
            lua.create_function(move |lua, args: Variadic<Value>| {
                let int = |i: usize| -> Result<i64> {
                    let v = args[i].clone();
                    match lua.coerce_integer(v.clone())? {
                        Some(k) => Ok(k),
                        None if lua.coerce_number(v.clone())?.is_some() => {
                            err(format!("bad argument #{} to 'random' (number has no integer representation)", i + 1))
                        }
                        None => err(format!("bad argument #{} to 'random' (number expected, got {})", i + 1, v.type_name())),
                    }
                };
                let (m, n) = match args.len() {
                    0 => return Ok(Value::Number(s1.borrow_mut().rng.f() as f64)),
                    1 => match int(0)? {
                        0 => return Ok(Value::Integer(s1.borrow_mut().rng.next_u64() as i64)),
                        n => (1, n),
                    },
                    2 => (int(0)?, int(1)?),
                    _ => return err("wrong number of arguments to 'random'"),
                };
                if m > n {
                    return err("bad argument #1 to 'random' (interval is empty)");
                }
                let rng = &mut s1.borrow_mut().rng;
                let lim = n.wrapping_sub(m) as u64;
                if lim < 1 << 24 {
                    let w = lim as i64 + 1;
                    let r = rng.f() as f64;
                    return Ok(Value::Integer(m + ((r * w as f64).floor() as i64).min(w - 1)));
                }
                // uniform in 0..=lim: draw under the smallest all-ones mask, retry above lim
                let mask = u64::MAX >> lim.leading_zeros();
                let x = loop {
                    let x = rng.next_u64() & mask;
                    if x <= lim {
                        break x;
                    }
                };
                Ok(Value::Integer((m as u64).wrapping_add(x) as i64))
            })?,
        )?;
        let s2 = st.clone();
        math.set("randomseed", lua.create_function(move |_, s: u64| {
            s2.borrow_mut().rng = Rng::new(s);
            Ok(())
        })?)?;
        let s3 = st.clone();
        g.set("rand", lua.create_function(move |_, (a, b): (Option<f32>, Option<f32>)| {
            let mut s = s3.borrow_mut();
            Ok(match (a, b) {
                (None, _) => s.rng.f(),
                (Some(hi), None) => s.rng.range(0.0, hi),
                (Some(lo), Some(hi)) => s.rng.range(lo, hi),
            })
        })?)?;
        let s4 = st.clone();
        g.set("randn", lua.create_function(move |_, (mean, sd): (Option<f32>, Option<f32>)| {
            Ok(mean.unwrap_or(0.0) + sd.unwrap_or(1.0) * s4.borrow_mut().rng.normal())
        })?)?;
    }

    // canvas{size=, aspect=, linen=, ground={...}, seed=}
    {
        let st = st.clone();
        g.set(
            "canvas",
            lua.create_function(move |lua, o: Option<Table>| {
                let o = o.ok_or_else(|| mlua::Error::runtime(CANVAS_HELP))?;
                // an old log's canvas: a named style and palette (legacy.rs)
                #[cfg(feature = "replay")]
                if crate::legacy::asks(&o)? {
                    return crate::legacy::canvas(lua, &st, o);
                }
                check_keys(&o, &["size", "aspect", "linen", "ground", "seed", "raw"], "canvas")?;
                if st.borrow().canvas.is_some() {
                    return err("the canvas is already set up (canvas{} is the first chunk)");
                }
                let mm = num(&o, "size")?.ok_or_else(|| mlua::Error::runtime(format!("canvas: size= (the width in mm)\n{CANVAS_HELP}")))?;
                if !(50.0..=5000.0).contains(&mm) {
                    return err("canvas: size is the canvas width in mm, 50 to 5000");
                }
                let aspect = num(&o, "aspect")?.ok_or_else(|| mlua::Error::runtime(format!("canvas: aspect= (width / height)\n{CANVAS_HELP}")))?;
                if !(0.2..=5.0).contains(&aspect) {
                    return err("canvas: aspect is width / height, between 0.2 and 5");
                }
                let (warp, weft) = pair(&o, "linen")?.ok_or_else(|| mlua::Error::runtime(format!("canvas: linen= (threads per cm: one number, or {{warp, weft}})\n{CANVAS_HELP}")))?;
                if !((4.0..=60.0).contains(&warp) && (4.0..=60.0).contains(&weft)) {
                    return err("canvas: linen threads per cm, 4 to 60");
                }
                let tubes = st.borrow().tubes.clone();
                // a raw canvas: no ground, the bare cloth soaks up pours
                let fabric = match o.get::<Option<String>>("raw")? {
                    None => None,
                    Some(n) => Some(paint::Fabric::named(&n).ok_or_else(|| mlua::Error::runtime(format!("canvas: raw= names the cloth: {}", paint::Fabric::names())))?),
                };
                let ground = match (&fabric, o.get::<Value>("ground")?) {
                    (Some(_), Value::Nil) => Vec::new(),
                    (Some(_), Value::Table(t)) if t.is_empty() => Vec::new(),
                    (Some(_), _) => return err("canvas: a raw canvas has no ground (leave ground= out)"),
                    (None, g) => ground_of(&tubes, &g)?,
                };
                let seed = o.get::<Option<u64>>("seed")?.unwrap_or(1);
                let mut sty = Style { name: "oil", width_mm: mm, linen: Linen { warp_per_cm: warp, weft_per_cm: weft, ..Linen::fine(1) }, ground, ..Style::oil_with((*tubes).clone()) };
                if let Some(f) = fabric {
                    sty.raw = f.color;
                }
                let width = st.borrow().width;
                let mut c = sty.prepare(width, aspect, seed);
                if let Some(f) = fabric {
                    c.raw_canvas(f, seed);
                }
                let h = c.height();
                {
                    let mut s = st.borrow_mut();
                    s.seed = seed;
                    s.rng = Rng::new(mixseed(seed, s.chunk, 0xC0FFEE));
                    s.clock0 = c.clock();
                    s.clock = 0.0;
                    // the clock counts from here: the ground is the colorman's
                    time::start(&mut c);
                    s.hand = time::Hand::default();
                    s.canvas = Some(c);
                    s.style = Some(Rc::new(sty));
                    s.setup = Some(match fabric {
                        None => format!("size={}, aspect={aspect}, linen={{{warp}, {weft}}}, seed={seed}", fmt_num(mm)),
                        Some(f) => format!("size={}, aspect={aspect}, linen={{{warp}, {weft}}}, raw={:?}, seed={seed}", fmt_num(mm), f.name),
                    });
                }
                let gl = lua.globals();
                // whole numbers as Lua integers (so `print(H)` says 714, not 714.0)
                let hv = if h.fract() == 0.0 { Value::Integer(h as i64) } else { Value::Number(h as f64) };
                gl.set("W", 1000)?;
                gl.set("H", hv.clone())?;
                Ok(hv)
            })?,
        )?;
    }

    // tubes(): the names of the tubes in the box
    {
        let st = st.clone();
        g.set("tubes", lua.create_function(move |_, ()| Ok(st.borrow().tubes.tubes.iter().map(|t| t.name.to_string()).collect::<Vec<_>>()))?)?;
    }

    // pile{{"lead white", 6}, {"smalt", 1}, ..., medium=0.2}: knife a pile
    // from tubes, in parts by volume, with that share of oil medium
    {
        let st = st.clone();
        g.set("pile", lua.create_function(move |_, t: Table| {
            check_keys(&t, &["medium"], "pile")?;
            let medium = num(&t, "medium")?.unwrap_or(0.0);
            if !(0.0..=0.95).contains(&medium) {
                return err("pile: medium is the share of oil medium mixed in, 0 (as from the tube) to 0.95");
            }
            let tubes = st.borrow().tubes.clone();
            let (parts, given) = parts_of(&tubes, &t, "pile")?;
            let mix = tubes.pile(parts);
            if st.borrow().canvas.is_none() {
                return Err(no_canvas());
            }
            // knifing it takes the hand a while
            time::knife(&st, mix.color);
            Ok(PileU { mix, medium, parts: given })
        })?)?;
    }

    // numbers
    g.set("smoothstep", lua.create_function(|_, (a, b, x): (f32, f32, f32)| Ok(paint::smoothstep(a, b, x)))?)?;
    g.set("lerp", lua.create_function(|_, (a, b, t): (f32, f32, f32)| Ok(paint::lerp(a, b, t)))?)?;
    g.set("clamp", lua.create_function(|_, (x, a, b): (f32, Option<f32>, Option<f32>)| Ok(x.clamp(a.unwrap_or(0.0), b.unwrap_or(1.0))))?)?;
    // noise{seed=, octaves=, period=, persistence=, kind="fbm"|"ridged"|"billow",
    //       warp={period, amount, twice}, stretch={angle, k}}
    g.set("noise", lua.create_function(|_, o: Option<Table>| {
        let o = match o {
            None => return Ok(Noise { kind: NoiseKind::Fbm(Fbm::new(1, 4, 200.0)), warp: None, stretch: None }),
            Some(o) => o,
        };
        check_keys(&o, &["seed", "octaves", "period", "persistence", "kind", "warp", "stretch"], "noise")?;
        let (seed, oct, period) = (o.get::<Option<u32>>("seed")?.unwrap_or(1), o.get::<Option<usize>>("octaves")?.unwrap_or(4), num(&o, "period")?.unwrap_or(200.0));
        let pers = o.get::<Option<f64>>("persistence")?;
        use paint::noise::{Fold, Octaves};
        let kind = match o.get::<Option<String>>("kind")?.as_deref().unwrap_or("fbm") {
            "fbm" => NoiseKind::Fbm(Fbm::new(seed, oct, period).with_persistence(pers.unwrap_or(0.5))),
            k @ ("ridged" | "billow" | "plain") => {
                let fold = match k {
                    "ridged" => Fold::Ridged,
                    "billow" => Fold::Billow,
                    _ => Fold::Plain,
                };
                let mut n = Octaves::new(seed, oct, period, fold);
                if let Some(p) = pers {
                    n = n.persistence(p as f32);
                }
                NoiseKind::Octaves(n)
            }
            k => return err(format!("noise kind {k:?}: fbm, ridged, billow or plain")),
        };
        let warp = match o.get::<Option<Table>>("warp")? {
            None => None,
            Some(t) => {
                let mut w = paint::noise::Warp::new(seed.wrapping_add(17), t.get(1)?, t.get(2)?);
                if t.get::<Option<bool>>(3)?.unwrap_or(false) {
                    w = w.twice();
                }
                Some(w)
            }
        };
        let stretch = pair(&o, "stretch")?.map(|(a, k)| paint::noise::Aniso::new(a, k));
        Ok(Noise { kind, warp, stretch })
    })?)?;
    // worley{seed=, period=, jitter=}: cells
    g.set("worley", lua.create_function(|_, o: Option<Table>| {
        let (seed, period, jitter) = match &o {
            None => (1, 40.0, None),
            Some(o) => {
                check_keys(o, &["seed", "period", "jitter"], "worley")?;
                (o.get::<Option<u32>>("seed")?.unwrap_or(1), num(o, "period")?.unwrap_or(40.0), num(o, "jitter")?)
            }
        };
        let mut w = paint::noise::Worley::new(seed, period);
        if let Some(j) = jitter {
            w = w.jitter(j);
        }
        Ok(WorleyU(w))
    })?)?;
    // uneven(n, lo, hi, irregular?, clump?, seed?): n positions between lo and hi,
    // spaced as a hand spaces them (lognormal gaps, grouped in clumps)
    g.set("uneven", lua.create_function(|_, (n, lo, hi, irr, clump, seed): (usize, f32, f32, Option<f32>, Option<f32>, Option<u32>)| {
        Ok(paint::noise::uneven(n, lo, hi, irr.unwrap_or(0.6), clump.unwrap_or(0.3), seed.unwrap_or(1)))
    })?)?;
    // masks
    {
        let st1 = st.clone();
        g.set("mask", lua.create_function(move |_, f: Function| Ok(wrap(eval_mask(frame(&st1)?, &f)?)))?)?;
        let st1 = st.clone();
        g.set("everywhere", lua.create_function(move |_, ()| Ok(wrap(Mask::full(frame(&st1)?))))?)?;
        let st1 = st.clone();
        g.set("ellipse", lua.create_function(move |_, (cx, cy, rx, ry): (f32, f32, f32, Option<f32>)| {
            Ok(wrap(Mask::from_shape(frame(&st1)?, Shape::new().ellipse(cx, cy, rx, ry.unwrap_or(rx)))))
        })?)?;
        let st1 = st.clone();
        g.set("rect", lua.create_function(move |_, (x, y, w, h): (f32, f32, f32, f32)| Ok(wrap(Mask::from_shape(frame(&st1)?, Shape::new().rect(x, y, w, h)))))?)?;
        let st1 = st.clone();
        g.set("poly", lua.create_function(move |_, (p, smooth): (Value, Option<bool>)| {
            let pts = points(&p)?;
            if pts.len() < 3 {
                return err("poly: needs at least three points");
            }
            let s = if smooth.unwrap_or(false) { Shape::new().smooth_poly(&pts) } else { Shape::new().poly(&pts) };
            Ok(wrap(Mask::from_shape(frame(&st1)?, s)))
        })?)?;
        // below(curve, bottom?): everything under a curve (points or function(x) -> y)
        let st1 = st.clone();
        g.set("below", lua.create_function(move |_, (c, bottom): (Value, Option<f32>)| {
            let f = frame(&st1)?;
            let pts = curve_of(&c, f.width())?;
            Ok(wrap(Mask::from_shape(f, Shape::new().below(&pts, bottom.unwrap_or(f.height() + 1.0)))))
        })?)?;
        let st1 = st.clone();
        g.set("above", lua.create_function(move |_, c: Value| {
            let f = frame(&st1)?;
            let pts = curve_of(&c, f.width())?;
            Ok(wrap(Mask::from_shape(f, Shape::new().below(&pts, f.height() + 1.0)).invert()))
        })?)?;
        // ribbon(points, widths): a band along a line
        let st1 = st.clone();
        g.set("ribbon", lua.create_function(move |_, (p, w): (Value, Value)| {
            let pts = points(&p)?;
            let ws: Vec<f32> = match w {
                Value::Table(t) => t.sequence_values::<f32>().collect::<Result<_>>()?,
                v => vec![f32::from_lua_value(v)?; pts.len()],
            };
            if ws.len() != pts.len() || pts.len() < 2 {
                return err("ribbon: needs >= 2 points and one width per point (or one width)");
            }
            Ok(wrap(Mask::from_shape(frame(&st1)?, Shape::new().ribbon(&pts, &ws))))
        })?)?;
    }

    // brushes
    {
        let st1 = st.clone();
        g.set("brush", lua.create_function(move |_, (a, w): (Value, Option<f32>)| {
            let tool = match (&a, w) {
                (Value::String(s), Some(w)) => tool_named(&s.to_str()?, w)?,
                (v, _) => tool_of(v)?,
            };
            let seed = st1.borrow_mut().auto_seed();
            let held = Rc::new(RefCell::new(Held::new(tool, seed)));
            st1.borrow_mut().brushes.push(Rc::downgrade(&held));
            Ok(Brush { held, st: st1.clone() })
        })?)?;
    }

    // covering areas
    {
        let st1 = st.clone();
        g.set("work", lua.create_function(move |_, (m, o): (Value, Table)| work(&st1, mask_of(&m)?, o, None))?)?;
        let st1 = st.clone();
        g.set("blend", lua.create_function(move |lua, (m, o): (Value, Option<Table>)| {
            let o = o.unwrap_or(lua.create_table()?);
            work(&st1, mask_of(&m)?, o, Some("blend"))
        })?)?;
        let st1 = st.clone();
        g.set("stipple", lua.create_function(move |_, (m, o): (Value, Table)| stipple(&st1, mask_of(&m)?, o))?)?;
    }

    // time
    {
        // wait(minutes): time passes with the hand away from the canvas; the
        // paint ages where it lies (open, setting, tacky, touch-dry by
        // pigment, film and oil). Returns the time of day. At most 10 years.
        let st1 = st.clone();
        g.set("wait", lua.create_function(move |_, minutes: f64| {
            if !(0.0..=MAX_WAIT_MIN).contains(&minutes) {
                return err(format!("wait({minutes}): want minutes from 0 to {MAX_WAIT_MIN} (10 years); days are fine: wait(3 * 24 * 60)"));
            }
            time::verb(&st1, Verb::Wait, |s| {
                s.canvas.as_mut().ok_or_else(no_canvas)?.wait(minutes as f32);
                Ok(())
            })?;
            Ok(time::time_of_day(st1.borrow().clock))
        })?)?;
        // drying(x, y): "open", "setting", "tacky" or "dry": the paint there, as a knuckle feels it
        let st1 = st.clone();
        g.set("drying", lua.create_function(move |_, (x, y): (f32, f32)| {
            time::verb(&st1, Verb::Query, |s| {
                Ok(match s.canvas.as_ref().ok_or_else(no_canvas)?.drying_at(x, y) {
                    paint::Stage::Open => "open",
                    paint::Stage::Setting => "setting",
                    paint::Stage::Tacky => "tacky",
                    paint::Stage::Dry => "dry",
                })
            })
        })?)?;
    }

    // soak-stain on a raw canvas
    {
        // pour(mask, {pile=, thinner=, ml=, tilt={angle, amount}, seed=}):
        // the pile thinned with turpentine, poured where the mask says
        let st1 = st.clone();
        g.set("pour", lua.create_function(move |_, (m, o): (Value, Table)| {
            let m = mask_of(&m)?;
            check_keys(&o, &["pile", "thinner", "ml", "tilt", "seed"], "pour")?;
            let p = pile_of(&o.get::<Value>("pile")?, "pour")?;
            let thinner = num(&o, "thinner")?.ok_or_else(|| mlua::Error::runtime("pour: thinner= (parts of turpentine to one part of the pile)"))?;
            if !(0.5..=50.0).contains(&thinner) {
                return err("pour: thinner is parts of turpentine to one part of the pile, 0.5 to 50 (thicker paint doesn't soak in)");
            }
            let ml = num(&o, "ml")?.ok_or_else(|| mlua::Error::runtime("pour: ml= (how much is poured, millilitres)"))?;
            if !(0.05..=5000.0).contains(&ml) {
                return err("pour: ml is how much is poured, 0.05 to 5000 millilitres");
            }
            let tilt = match pair(&o, "tilt")? {
                None => None,
                Some((a, g)) if (0.0..=1.0).contains(&g) => Some((a, g)),
                Some(_) => return err("pour: tilt={angle, amount}: the direction it runs downhill (radians) and how steeply, 0 to 1"),
            };
            let tubes = st1.borrow().tubes.clone();
            let (mut pig, mut mob) = (0.0f32, 0.0f32);
            for &(i, f) in &p.mix.parts {
                let (phi, mb) = paint::soak::tube_soak(tubes.tubes[i].name);
                pig += f * phi;
                mob += f * mb;
            }
            let pigment = pig * (1.0 - p.medium);
            let paint = paint::pigment::Pigment::masstone(p.mix.color, p.mix.scatter * (1.0 - p.medium).max(1e-3));
            let seed = match o.get::<Option<u64>>("seed")? {
                Some(s) => s,
                None => st1.borrow_mut().auto_seed(),
            };
            let pour = paint::Pour { paint, pigment, oil: 1.0 - pigment, mobility: mob, thinner, ml, tilt, seed };
            time::verb(&st1, Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                let r = c.pour(&m, &pour).map_err(mlua::Error::runtime)?;
                let mut out = format!("soaked {:.1} ml over {:.0} cm² in {:.1} min", r.soaked_ml, r.area_cm2, r.spread_s / 60.0);
                if r.lost_ml > 0.005 {
                    out.push_str(&format!("; {:.1} ml found no room and pooled off", r.lost_ml));
                }
                if r.halo_ml > 0.001 {
                    out.push_str(&format!("; oil will creep up to {:.0} mm past the colour ({:.2} ml)", r.halo_mm, r.halo_ml));
                }
                Ok(out)
            })
        })?)?;
        // blot(mask, {strength=}): a rag or sponge pressed on the wet stain
        let st1 = st.clone();
        g.set("blot", lua.create_function(move |lua, (m, o): (Value, Option<Table>)| {
            let m = mask_of(&m)?;
            let o = o.unwrap_or(lua.create_table()?);
            check_keys(&o, &["strength"], "blot")?;
            let strength = num(&o, "strength")?.unwrap_or(0.7);
            if !(0.0..=1.0).contains(&strength) {
                return err("blot: strength 0 to 1");
            }
            time::verb(&st1, Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                c.blot(&m, strength).map_err(mlua::Error::runtime)
            })
        })?)?;
        // soaked(x, y): what is in the cloth there, in words
        let st1 = st.clone();
        g.set("soaked", lua.create_function(move |_, (x, y): (f32, f32)| {
            time::verb(&st1, Verb::Query, |s| {
                let c = s.canvas.as_ref().ok_or_else(no_canvas)?;
                Ok(c.soaked_at(x, y).unwrap_or_else(|| if c.is_raw() { "outside the canvas".into() } else { "primed (nothing soaks in)".into() }))
            })
        })?)?;
    }

    #[cfg(feature = "finish")]
    crate::finish::install(lua, st.clone())?;

    crate::form::install(lua, st.clone())?;
    crate::world::install(lua, st.clone())?;
    draw_pencil::install(lua, st.clone())?;
    crate::draw_outline::install(lua, st.clone())?;
    crate::draw_edges::install(lua, st.clone())?;
    Ok(())
}




const CANVAS_HELP: &str = "canvas{size=<mm>, aspect=<width / height>, linen=<threads per cm>, ground={{pile={{\"<tube>\", <parts>}, ...}, um=<µm>, apply=\"<knife|roller|brush>\"}, ...}, seed=<n>}\n  size: width in mm; aspect: width / height; linen: threads per cm (or {warp, weft});\n  ground: layers bottom first, each a pile of tubes, a thickness in µm and how it is put on (\"knife\", \"roller\" or \"brush\"; a knife takes texture=0..1)\n  or raw=\"cotton duck\" (or \"linen\") and no ground: the bare cloth, for pour()";

/// Ground layers from `{{pile={{tube, parts}, ...}, um=, apply=, texture=}, ...}`,
/// bottom first: each the paste its tubes make (masstone, hiding, stiffness).
fn ground_of(tubes: &Palette, v: &Value) -> Result<Vec<Ground>> {
    let Value::Table(t) = v else {
        return err(format!("canvas: ground= (the layers, bottom first)\n{CANVAS_HELP}"));
    };
    let mut out = Vec::new();
    for l in t.sequence_values::<Value>() {
        let Value::Table(l) = l? else { return err(format!("canvas: each ground layer is a table\n{CANVAS_HELP}")) };
        check_keys(&l, &["pile", "um", "apply", "texture"], "ground layer")?;
        let Value::Table(p) = l.get::<Value>("pile")? else { return err(format!("canvas: a ground layer needs pile={{{{tube, parts}}, ...}}\n{CANVAS_HELP}")) };
        let (parts, _) = parts_of(tubes, &p, "ground")?;
        let m = tubes.pile(parts);
        let um = num(&l, "um")?.ok_or_else(|| mlua::Error::runtime("canvas: a ground layer needs um= (its thickness in µm)"))?;
        if !(5.0..=400.0).contains(&um) {
            return err("canvas: a ground layer is 5 to 400 µm thick");
        }
        let texture = num(&l, "texture")?.unwrap_or(0.3).clamp(0.0, 1.0);
        let apply = match l.get::<Option<String>>("apply")?.as_deref() {
            Some("knife") => Apply::Knife { texture },
            Some("roller") => Apply::Roller,
            Some("brush") => Apply::Brush,
            _ => return err("canvas: a ground layer's apply= is \"knife\", \"roller\" or \"brush\""),
        };
        out.push(Ground { color: m.color, hiding: m.hiding, um, stiff: m.stiff, apply });
    }
    if out.is_empty() {
        return err(format!("canvas: ground= needs at least one layer\n{CANVAS_HELP}"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    fn run(src: &str) -> Result<String, String> {
        Session::replay(200).unwrap().run(src).map(|r| r.out)
    }

    // a raw canvas takes no ground: an empty table is none, anything in it
    // (in either part of the table) is refused
    #[test]
    fn a_raw_canvas_refuses_a_ground() {
        if let Err(e) = run(r#"canvas{size=300, aspect=1, linen=15, raw="cotton duck", ground={}}"#) {
            panic!("{e}");
        }
        for g in ["{um=50}", r#"{{pile={{"lead white", 1}}, um=50, apply="knife"}}"#] {
            let e = run(&format!(r#"canvas{{size=300, aspect=1, linen=15, raw="cotton duck", ground={g}}}"#)).unwrap_err();
            assert!(e.contains("a raw canvas has no ground"), "{g}: {e}");
        }
    }

    // math.random: stock Lua's errors, whole 64-bit ranges, and the draws
    // old logs made from small ordered ranges unchanged
    #[test]
    fn math_random_keeps_old_draws_and_refuses_empty_ranges() {
        let draws = r#"math.randomseed(7)
            local t = {}
            for _ = 1, 6 do t[#t + 1] = math.random(2, 4) end
            for _ = 1, 3 do t[#t + 1] = math.random(100) end
            t[#t + 1] = math.random(-3, 3)
            t[#t + 1] = math.random(5.0)
            t[#t + 1] = string.format("%.9f", math.random())
            print(table.concat(t, " "))"#;
        assert_eq!(run(draws).unwrap(), "4 4 3 2 2 3 85 32 34 0 3 0.080516815\n");
        for (call, want) in [
            ("math.random(5, 1)", "interval is empty"),
            ("math.random(-5)", "interval is empty"),
            ("math.random(1.5)", "number has no integer representation"),
            ("math.random(1, 2, 3)", "wrong number of arguments"),
        ] {
            let e = run(call).unwrap_err();
            assert!(e.contains(want), "{call}: {e}");
        }
        run("math.randomseed(7)
             local a = math.random(math.mininteger, math.maxinteger)
             assert(math.type(a) == 'integer')
             local odd = false
             for _ = 1, 64 do
               local x = math.random(0, 1 << 40)
               assert(x >= 0 and x <= 1 << 40)
               odd = odd or x % 2 == 1
             end
             assert(odd, 'wide ranges reach every integer')
             assert(math.type(math.random(0)) == 'integer')")
        .unwrap();
    }
}
