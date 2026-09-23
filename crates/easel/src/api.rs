//! The painter's Lua API over the engine.
//!
//! Everything a chunk can call is registered here as globals. The state a
//! painting builds up (the canvas, the style, the clock, the brushes in the
//! hand) lives in `Studio`, shared by the closures through an `Rc`.
//!
//! Painter functions (color, angle, coverage fields) are Lua closures, which
//! can't run on the engine's rayon threads, so they are sampled serially onto
//! a grid in canvas units first (`FIELD_STEP`, over the mask's bounding box)
//! and read back bilinearly. Mask functions are evaluated at every pixel.

use mlua::{Function, Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value, Variadic};
use paint::color::{Mix, luminance, mix, to_oklab};
use paint::growth::Habit;
use paint::{Canvas, Cracks, Fbm, Frame, Gesture, Handling, Held, Kind, Mask, Order, Orient, Palette, Pigment, Rgb, Rng, Shape, Stipple, Style, Tool, Touch, hex};
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
    /// Arguments `canvas{}` was called with (for the log and status).
    pub setup: Option<String>,
    pub seed: u64,
    /// Index of the chunk being run (1-based), for seeds.
    pub chunk: u64,
    /// Engine calls made in this chunk, for automatic seeds.
    pub calls: u64,
    /// Painting time in minutes since `canvas{}` (advanced by `wait` and
    /// `dry`; the canvas's own clock also counts the grounds drying).
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
    /// Paint only this window of the canvas (a `look --scale` crop session).
    pub crop: Option<paint::Crop>,
}

impl Studio {
    pub fn new(width: usize) -> Self {
        Studio { width, canvas: None, style: None, setup: None, seed: 1, chunk: 0, calls: 0, clock: 0.0, clock0: 0.0, rng: Rng::new(1), brushes: Vec::new(), out: String::new(), field_secs: 0.0, view: None, crop: None }
    }

    /// Start chunk `n`: its randomness depends only on the seed and `n`.
    pub fn begin(&mut self, n: u64) {
        self.chunk = n;
        self.calls = 0;
        self.rng = Rng::new(mixseed(self.seed, n, 0xC0FFEE));
        self.out.clear();
        self.field_secs = 0.0;
    }

    fn auto_seed(&mut self) -> u64 {
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

// ---------------------------------------------------------------- colors

#[derive(Clone, Copy)]
pub struct Col(pub Rgb);

impl UserData for Col {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("r", |_, c| Ok(c.0[0]));
        f.add_field_method_get("g", |_, c| Ok(c.0[1]));
        f.add_field_method_get("b", |_, c| Ok(c.0[2]));
        f.add_field_method_get("value", |_, c| Ok(luminance(c.0)));
        f.add_field_method_get("L", |_, c| Ok(to_oklab(c.0)[0]));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::ToString, |_, c, ()| Ok(hexstr(c.0)));
        m.add_method("hex", |_, c, ()| Ok(hexstr(c.0)));
        m.add_method("mix", |_, c, (o, t, mode): (Value, f32, Option<String>)| Ok(Col(mix(c.0, rgb_of(&o)?, t, mix_mode(mode.as_deref())?))));
    }
}

fn hexstr(c: Rgb) -> String {
    let b = |v: f32| (paint::color::linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", b(c[0]), b(c[1]), b(c[2]))
}

fn mix_mode(s: Option<&str>) -> Result<Mix> {
    Ok(match s.unwrap_or("light") {
        "light" | "oklab" => Mix::Light,
        "pigment" | "paint" => Mix::Pigment,
        "linear" => Mix::Linear,
        o => return err(format!("mix mode {o:?}: use \"light\", \"pigment\" or \"linear\"")),
    })
}

/// A color from a hex string, a `Col`, or a table {r, g, b} (linear 0..1).
pub fn rgb_of(v: &Value) -> Result<Rgb> {
    match v {
        Value::String(s) => {
            let s = s.to_str()?;
            let t = s.trim_start_matches('#');
            if t.len() != 6 || !t.chars().all(|c| c.is_ascii_hexdigit()) {
                return err(format!("color {s:?}: want \"#rrggbb\""));
            }
            Ok(hex(&s))
        }
        Value::UserData(u) => Ok(u.borrow::<Col>()?.0),
        Value::Table(t) => Ok([t.get(1).or_else(|_| t.get("r"))?, t.get(2).or_else(|_| t.get("g"))?, t.get(3).or_else(|_| t.get("b"))?]),
        o => err(format!("not a color: {}", o.type_name())),
    }
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
    fn get(&self, x: f32, y: f32) -> [f32; N] {
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

fn sample<const N: usize>(st: &S, fun: &Function, b: (f32, f32, f32, f32), conv: impl Fn(Value) -> Result<[f32; N]>) -> Result<Grid<N>> {
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

pub(crate) type FieldBox<T> = Box<dyn Fn(f32, f32) -> T + Sync>;

fn color_field(st: &S, v: &Value, b: (f32, f32, f32, f32)) -> Result<FieldBox<Rgb>> {
    // a sky or clouds: read natively on the engine's threads
    if let Value::UserData(u) = v {
        if let Ok(s) = u.borrow::<crate::world::SkyU>() {
            let s = s.0.clone();
            return Ok(Box::new(move |x, y| s.at(x, y)));
        }
        if let Ok(c) = u.borrow::<crate::world::CloudsU>() {
            let (sky, cf) = (c.sky.clone(), c.field.clone());
            return Ok(Box::new(move |x, y| cf.color(&sky, x, y)));
        }
    }
    if let Value::Function(f) = v {
        let g = sample::<3>(st, f, b, |r| rgb_of(&r))?;
        Ok(Box::new(move |x, y| g.get(x, y)))
    } else {
        let c = rgb_of(v)?;
        Ok(Box::new(move |_, _| c))
    }
}

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
    // a rock's field (r:field("plane")): read natively too
    if let Value::UserData(u) = v
        && let Ok(fu) = u.borrow::<crate::draw_rocks::RockFieldU>()
    {
        return Ok(fu.angle(0.0));
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
fn tool_of(v: &Value) -> Result<Tool> {
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

fn palette_of(st: &S, v: Value) -> Result<Rc<Palette>> {
    match v {
        Value::Nil => Ok(Rc::new(style(st)?.palette.clone())),
        Value::UserData(u) => Ok(u.borrow::<Pal>()?.0.clone()),
        o => err(format!("pal: want a palette, got {}", o.type_name())),
    }
}

impl UserData for Brush {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("width", |_, b| Ok(b.held.borrow().tool.width));
        f.add_field_method_get("kind", |_, b| Ok(format!("{:?}", b.held.borrow().tool.kind).to_lowercase()));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // b:load(color, amount?, {medium=, at={x,y}, coats=, pal=, raw=, hiding=, stiff=})
        m.add_method("load", |_, b, (c, amount, o): (Value, Option<f32>, Option<Table>)| {
            let paint = paint_for(&b.st, &c, o.as_ref(), (b.held.borrow().tool.width * 0.5).max(1.0))?;
            b.held.borrow_mut().load(paint, amount.unwrap_or(0.8));
            Ok(())
        });
        m.add_method("reload", |_, b, (c, amount, o): (Value, Option<f32>, Option<Table>)| {
            let paint = paint_for(&b.st, &c, o.as_ref(), (b.held.borrow().tool.width * 0.5).max(1.0))?;
            b.held.borrow_mut().reload(paint, amount.unwrap_or(0.8));
            Ok(())
        });
        m.add_method("wipe", |_, b, frac: Option<f32>| {
            b.held.borrow_mut().wipe(frac.unwrap_or(0.85));
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
            let mut s = b.st.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.drag(&mut b.held.borrow_mut(), &g, clip.as_deref());
            Ok(())
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
            let mut s = b.st.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.touch(&mut b.held.borrow_mut(), &t, clip.as_deref());
            Ok(())
        });
        m.add_meta_method(MetaMethod::ToString, |_, b, ()| {
            let h = b.held.borrow();
            Ok(format!("brush({:?} {}, {:.0}% full)", h.tool.kind, h.tool.width, 100.0 * h.fullness()).to_lowercase())
        });
    }
}

fn no_canvas() -> mlua::Error {
    mlua::Error::runtime("no canvas yet: start with canvas{style=\"friedrich\", aspect=1.4, seed=1}")
}

fn style(st: &S) -> Result<Rc<Style>> {
    st.borrow().style.clone().ok_or_else(no_canvas)
}

pub(crate) fn frame(st: &S) -> Result<Frame> {
    Ok(st.borrow().canvas.as_ref().ok_or_else(no_canvas)?.frame())
}

/// Paint for a brush: mixed from the palette to `c` (masstone), or aimed at
/// the look over what is at `at`, or a raw paint.
fn paint_for(st: &S, c: &Value, o: Option<&Table>, r: f32) -> Result<paint::Paint> {
    if let Value::UserData(u) = c
        && let Ok(p) = u.borrow::<PaintU>()
    {
        return Ok(p.0);
    }
    let want = rgb_of(c)?;
    let sty = style(st)?;
    let Some(o) = o else { return Ok(sty.palette.paint(want, sty.body_medium)) };
    check_keys(o, &["medium", "at", "coats", "pal", "raw", "hiding", "stiff"], "load")?;
    if o.get::<Option<bool>>("raw")?.unwrap_or(false) {
        return Ok(paint::Paint::new(want, num(o, "hiding")?.unwrap_or(0.85), num(o, "stiff")?.unwrap_or(0.8)));
    }
    let pal = palette_of(st, o.get("pal")?)?;
    let medium = num(o, "medium")?.unwrap_or(sty.body_medium);
    if let Some((x, y)) = pair(o, "at")? {
        let s = st.borrow();
        let cv = s.canvas.as_ref().ok_or_else(no_canvas)?;
        return Ok(cv.aim(&pal, want, (x, y), r, medium, num(o, "coats")?.unwrap_or(0.8)));
    }
    Ok(pal.paint(want, medium))
}

// ---------------------------------------------------------------- paint, palette

#[derive(Clone, Copy)]
pub struct PaintU(pub paint::Paint);
impl UserData for PaintU {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::ToString, |_, p, ()| Ok(format!("paint({}, hiding {:.2}, stiff {:.2})", hexstr(p.0.color), p.0.hiding(), p.0.stiff)));
    }
}

const PALETTES: &str = "friedrich_1820, friedrich_early, friedrich_1820_greens, friedrich_early_greens";

pub(crate) fn palette_named(name: &str) -> Result<Palette> {
    Ok(match name {
        "friedrich_1820" | "friedrich" => Palette::friedrich_1820(),
        "friedrich_early" => Palette::friedrich_early(),
        "friedrich_1820_greens" | "greens" => Palette::friedrich_1820_greens(),
        "friedrich_early_greens" => Palette::friedrich_early_greens(),
        o => return err(format!("palette {o:?}: {PALETTES}")),
    })
}

/// Tubes a palette can be given (`pal:with{...}`).
fn extra_tube(name: &str) -> Result<paint::Tube> {
    let mut all = Palette::green_tubes();
    all.push(Palette::copper_green());
    all.extend(Palette::friedrich_1820().tubes);
    all.extend(Palette::friedrich_early().tubes);
    all.into_iter().find(|t| t.name == name).ok_or_else(|| mlua::Error::runtime(format!("no tube {name:?} (extra tubes: prussian blue, green earth, Rinmann's green, copper green; see pal:tubes())")))
}

#[derive(Clone)]
pub struct Pal(pub Rc<Palette>);
impl UserData for Pal {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("tubes", |_, p, ()| Ok(p.0.tubes.iter().map(|t| t.name.to_string()).collect::<Vec<_>>()));
        // pal:with{"copper green"}: this palette and more tubes
        m.add_method("with", |_, p, names: Vec<String>| {
            let mut extra = Vec::new();
            for n in &names {
                if !p.0.tubes.iter().any(|t| t.name == n) {
                    extra.push(extra_tube(n)?);
                }
            }
            Ok(Pal(Rc::new(p.0.with(extra))))
        });
        m.add_method("only", |_, p, names: Vec<String>| {
            for n in &names {
                if !p.0.tubes.iter().any(|t| t.name == n) {
                    let have: Vec<_> = p.0.tubes.iter().map(|t| t.name).collect();
                    return err(format!("no tube {n:?} (tubes: {})", have.join(", ")));
                }
            }
            let refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
            Ok(Pal(Rc::new(p.0.only(&refs))))
        });
        // pal:mix(color) -> masstone color it can reach, recipe, error
        m.add_method("mix", |_, p, c: Value| {
            let mx = p.0.mix(rgb_of(&c)?);
            Ok((Col(mx.color), p.0.recipe(&mx), mx.error))
        });
        // pal:aim(want, under, medium?, coats?) -> the paint whose look over `under` is `want`
        m.add_method("aim", |_, p, (want, under, medium, coats): (Value, Value, Option<f32>, Option<f32>)| {
            let mx = p.0.aim(rgb_of(&want)?, rgb_of(&under)?, medium.unwrap_or(0.2), coats.unwrap_or(1.0));
            Ok((Col(mx.color), p.0.recipe(&mx), mx.error))
        });
        m.add_method("paint", |_, p, (c, medium): (Value, Option<f32>)| Ok(PaintU(p.0.paint(rgb_of(&c)?, medium.unwrap_or(0.2)))));
        m.add_meta_method(MetaMethod::ToString, |_, p, ()| Ok(format!("palette {:?}: {}", p.0.name, p.0.tubes.iter().map(|t| t.name).collect::<Vec<_>>().join(", "))));
    }
}

// ---------------------------------------------------------------- masks

#[derive(Clone)]
pub struct M(pub Rc<Mask>);

pub(crate) fn mask_of(v: &Value) -> Result<Rc<Mask>> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<M>()?.0.clone()),
        o => err(format!("want a mask, got {} (make one with mask(fn), ellipse, poly, rect, below, ribbon, everywhere)", o.type_name())),
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
    "hand", "tool", "length", "coverage", "angle", "angle_jitter", "color", "jitter", "medium", "pal", "aim", "load_at", "cut_in", "pressure",
    "orient", "dips", "blender", "scrub", "clip", "threshold", "ramps", "shake", "curve", "cross", "drift", "tail", "broken", "swell", "clump",
    "order", "mix_jitter", "seed", "ruler", "paint", "load", "color_over", "hug", "visible", "behind", "at", "view",
];

fn work(st: &S, mask: Rc<Mask>, o: Table, preset: Option<&str>) -> Result<()> {
    check_keys(&o, WORK_KEYS, "work")?;
    let (mask, limit) = crate::depth::restrict_mask(st, &o, mask)?;
    let sty = style(st)?;
    let f = frame(st)?;
    let hand: String = o.get::<Option<String>>("hand")?.unwrap_or_else(|| preset.unwrap_or("body").to_string());
    let medium = num(&o, "medium")?;
    // a palette must outlive the handling
    let pal_v = o.get::<Value>("pal")?;
    let pal_keep: Option<Rc<Palette>> = match &pal_v {
        Value::Nil | Value::Boolean(false) => None,
        v => Some(palette_of(st, v.clone())?),
    };
    let mut h: Handling = match hand.as_str() {
        "broad" => sty.broad(),
        "body" => sty.body(),
        "detail" => sty.detail(),
        "hatch" => sty.hatch(),
        "glaze" => sty.glaze(medium.unwrap_or(0.9)),
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
    match o.get::<Value>("color")? {
        Value::Nil if hand != "blend" && o.get::<Value>("color_over")?.is_nil() => {
            return err("work: needs a color (a \"#rrggbb\", a color, or function(x, y) returning one) or color_over")
        }
        Value::Nil => {}
        v => h.color = color_field(st, &v, b)?,
    }
    if let Some((l, hue)) = pair(&o, "jitter")? {
        h = h.jitter(l, hue);
    }
    if let Some(co) = over_field(st, &o, b)? {
        h = h.color_over(co);
    }
    if let Some(on) = o.get::<Option<bool>>("hug")? {
        h = h.hug(on);
    }
    if let Value::Boolean(false) = pal_v {
        h.palette = None;
    }
    if let Some(p) = &pal_keep {
        let m = h.palette.map(|(_, m)| m).unwrap_or(sty.body_medium);
        h = h.mixed(p, m);
    }
    if let Some(m) = medium
        && h.palette.is_some()
    {
        h = h.medium(m);
    }
    if let Some((hid, stiff)) = pair(&o, "paint")? {
        h = h.paint(hid, stiff);
    }
    match o.get::<Value>("aim")? {
        Value::Nil => {}
        Value::String(s) if &*s.to_str()? == "laid" => h = h.aim_laid(),
        Value::String(s) if &*s.to_str()? == "masstone" => h = h.by_masstone(),
        Value::Number(n) => h = h.aim(n as f32),
        Value::Integer(n) => h = h.aim(n as f32),
        _ => return err("aim: \"laid\", \"masstone\" or a number of coats"),
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
    if let Some(c) = o.get::<Option<bool>>("clip")? {
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
    let seed = seed_of(st, &o)?;
    let mut s = st.borrow_mut();
    s.canvas.as_mut().ok_or_else(no_canvas)?.work(&mask, &h, seed);
    Ok(())
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

pub(crate) fn seed_of(st: &S, o: &Table) -> Result<u64> {
    Ok(match o.get::<Option<u64>>("seed")? {
        Some(s) => s,
        None => st.borrow_mut().auto_seed(),
    })
}

const STIPPLE_KEYS: &[&str] = &[
    "tool", "width", "pressure", "coverage", "color", "medium", "pal", "aim", "dips", "drag", "twist", "cluster", "feather", "clip", "jitter", "mix_jitter", "seed", "paint", "color_over", "fade", "visible", "behind", "at", "view",
];

type OverBox = Box<dyn Fn(f32, f32, Rgb) -> Rgb + Sync>;

/// `color_over`: {shift={dL, da, db}} (native: relative to whatever the
/// stroke lands on) or function(x, y, under) -> color (sampled every 2 units,
/// with `under` what is on the canvas there before this pass).
fn over_field(st: &S, o: &Table, b: (f32, f32, f32, f32)) -> Result<Option<OverBox>> {
    match o.get::<Value>("color_over")? {
        Value::Nil => Ok(None),
        Value::Table(t) => {
            let sh: Vec<f32> = t.get("shift")?;
            let (dl, da, db) = (sh.first().copied().unwrap_or(0.0), sh.get(1).copied().unwrap_or(0.0), sh.get(2).copied().unwrap_or(0.0));
            Ok(Some(Box::new(move |_, _, u| paint::shift(u, dl, da, db))))
        }
        Value::Function(f) => {
            let t0 = std::time::Instant::now();
            let step = FIELD_STEP;
            let nx = (((b.2 - b.0) / step).ceil() as usize + 1).max(2);
            let ny = (((b.3 - b.1) / step).ceil() as usize + 1).max(2);
            let mut v = Vec::with_capacity(nx * ny);
            for j in 0..ny {
                for i in 0..nx {
                    let (x, y) = ((b.0 + i as f32 * step).min(b.2 - 0.01).max(b.0), (b.1 + j as f32 * step).min(b.3 - 0.01).max(b.1));
                    let under = st.borrow().canvas.as_ref().ok_or_else(no_canvas)?.under(x, y, step * 0.5);
                    let r: Value = f.call((x, y, Col(under)))?;
                    v.push(rgb_of(&r)?);
                }
            }
            st.borrow_mut().field_secs += t0.elapsed().as_secs_f64();
            let g = Grid::<3> { x0: b.0, y0: b.1, step, nx, ny, v };
            Ok(Some(Box::new(move |x, y, _| g.get(x, y))))
        }
        o => err(format!("color_over: want {{shift={{dL, da, db}}}} or function(x, y, under), got {}", o.type_name())),
    }
}

fn stipple(st: &S, mask: Rc<Mask>, o: Table) -> Result<()> {
    check_keys(&o, STIPPLE_KEYS, "stipple")?;
    let (mask, limit) = crate::depth::restrict_mask(st, &o, mask)?;
    let f = frame(st)?;
    let tool = match o.get::<Option<Value>>("tool")? {
        Some(t) => tool_of(&t)?,
        None => Tool::stippler(num(&o, "width")?.unwrap_or(2.0)),
    };
    let b = support(Some(&mask), f, tool.width * 2.0 + 4.0);
    let pal_keep: Option<Rc<Palette>> = match o.get::<Value>("pal")? {
        Value::Boolean(false) => None,
        v => Some(palette_of(st, v)?),
    };
    let mut sp = Stipple::new(tool);
    if let Some(p) = &pal_keep {
        sp = sp.mixed(p, num(&o, "medium")?.unwrap_or(0.5));
    }
    if let Some((h, s)) = pair(&o, "paint")? {
        sp = sp.paint(h, s);
    }
    match o.get::<Value>("color")? {
        Value::Nil if o.get::<Value>("color_over")?.is_nil() => return err("stipple: needs a color or color_over"),
        Value::Nil => {}
        v => sp.color = color_field(st, &v, b)?,
    }
    if let Some(v) = o.get::<Option<Value>>("coverage")? {
        sp.coverage = scalar_field(st, &v, b, "coverage")?;
    }
    if let Some((a, z)) = pair(&o, "pressure")? {
        sp = sp.pressure(a, z);
    }
    if let Some(a) = o.get::<Option<bool>>("aim")? {
        sp = sp.aim(a);
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
    if let Some(k) = num(&o, "feather")? {
        sp = sp.feather(k);
    }
    if let Some(c) = o.get::<Option<bool>>("clip")? {
        sp = sp.clip(c);
    }
    if let Some((l, h)) = pair(&o, "jitter")? {
        sp = sp.jitter(l, h);
    }
    if let Some(j) = num(&o, "mix_jitter")? {
        sp = sp.mix_jitter(j);
    }
    if let Some(co) = over_field(st, &o, b)? {
        sp = sp.color_over(co);
    }
    if let Some(k) = num(&o, "fade")? {
        sp = sp.fade(k);
    }
    if let Some(l) = limit {
        sp = sp.limit(l);
    }
    sp.tool.validate().map_err(mlua::Error::runtime)?;
    let seed = seed_of(st, &o)?;
    let mut s = st.borrow_mut();
    s.canvas.as_mut().ok_or_else(no_canvas)?.stipple(&mask, &sp, seed);
    Ok(())
}

fn pigment_of(kind: Option<&str>, c: Rgb) -> Result<Pigment> {
    Ok(match kind.unwrap_or("transparent") {
        "transparent" => Pigment::transparent(c),
        "semi" => Pigment::semi(c),
        "opaque" => Pigment::opaque(c),
        "varnish" => Pigment::varnish(c),
        o => return err(format!("pigment {o:?}: transparent, semi, opaque or varnish")),
    })
}

// ---------------------------------------------------------------- trees

fn tree(lua: &Lua, st: &S, o: Table) -> Result<Table> {
    check_keys(&o, &["habit", "x", "y", "height", "seed", "years"], "tree")?;
    let mut habit = match o.get::<Option<String>>("habit")?.or(o.get::<Option<String>>(1)?).as_deref().unwrap_or("oak") {
        "oak" => Habit::oak(),
        "dead_oak" | "dead oak" => Habit::dead_oak(),
        "birch" => Habit::birch(),
        "spruce" => Habit::spruce(),
        "beech" => Habit::beech(),
        "alder" => Habit::alder(),
        "willow" => Habit::willow(),
        h => return err(format!("habit {h:?}: oak, dead_oak, birch, spruce, beech, alder or willow")),
    };
    if let Some(y) = o.get::<Option<u32>>("years")? {
        habit.years = y;
    }
    let (x, y, height) = (o.get::<f32>("x")?, o.get::<f32>("y")?, o.get::<f32>("height")?);
    let seed = seed_of(st, &o)?;
    let sk = Rc::new(habit.grow((x, y), height, seed));
    let t = lua.create_table()?;
    let limbs = lua.create_table()?;
    for (i, l) in sk.limbs.iter().enumerate() {
        let lt = lua.create_table()?;
        let pts = lua.create_table()?;
        for (k, p) in l.pts.iter().enumerate() {
            pts.set(k + 1, lua.create_sequence_from([p.0, p.1])?)?;
        }
        lt.set("pts", pts)?;
        lt.set("w", lua.create_sequence_from(l.w.iter().copied())?)?;
        lt.set("z", lua.create_sequence_from(l.z.iter().copied())?)?;
        lt.set("order", l.order)?;
        lt.set("parent", l.parent.map(|p| p + 1))?;
        lt.set("at", l.at + 1)?;
        lt.set("dead", l.dead)?;
        lt.set("dead_from", l.dead_from + 1)?;
        lt.set("broken", l.broken)?;
        lt.set("root", l.root)?;
        limbs.set(i + 1, lt)?;
    }
    t.set("limbs", limbs)?;
    let tips = lua.create_table()?;
    for (k, p) in sk.tips().iter().enumerate() {
        tips.set(k + 1, lua.create_sequence_from([p.0, p.1])?)?;
    }
    t.set("tips", tips)?;
    let bb = sk.bounds();
    t.set("bounds", lua.create_sequence_from([bb.0, bb.1, bb.2, bb.3])?)?;
    let (st2, sk2) = (st.clone(), sk.clone());
    t.set("mask", lua.create_function(move |_, _: Variadic<Value>| Ok(wrap(sk2.mask(frame(&st2)?))))?)?;
    // t:foliage{sun={x, y, z}, seed=, winter=false, years=, clump=, spacing=, squash=, droop=,
    //           fill=, ragged=, bare=, tip=, inner=, inner_w=, spray=}
    let (st2, sk2) = (st.clone(), sk.clone());
    t.set("foliage", lua.create_function(move |lua, (_, o): (Value, Option<Table>)| foliage(lua, &st2, &sk2, o))?)?;
    Ok(t)
}

fn foliage(lua: &Lua, st: &S, sk: &paint::Skeleton, o: Option<Table>) -> Result<Table> {
    let mut leaf = sk.leaf;
    let mut sun = (-0.55, -0.75, 0.35);
    let mut seed = None;
    if let Some(o) = &o {
        check_keys(o, &["sun", "seed", "winter", "years", "clump", "spacing", "squash", "droop", "fill", "ragged", "bare", "tip", "inner", "inner_w", "spray"], "foliage")?;
        if let Some(v) = o.get::<Option<Vec<f32>>>("sun")? {
            sun = (v.first().copied().unwrap_or(-0.55), v.get(1).copied().unwrap_or(-0.75), v.get(2).copied().unwrap_or(0.35));
        }
        seed = o.get::<Option<u64>>("seed")?;
        if o.get::<Option<bool>>("winter")?.unwrap_or(false) {
            leaf.years = 0;
        }
        if let Some(y) = o.get::<Option<u32>>("years")? {
            leaf.years = y;
        }
        macro_rules! over {
            ($($f:ident),*) => {$( if let Some(v) = num(o, stringify!($f))? { leaf.$f = v; } )*};
        }
        over!(clump, spacing, squash, droop, fill, ragged, bare, tip, inner, inner_w, spray);
    }
    let seed = match seed {
        Some(s) => s,
        None => st.borrow_mut().auto_seed(),
    };
    let fo = Rc::new(sk.foliage_with(&leaf, sun, seed));
    let t = lua.create_table()?;
    let clumps = lua.create_table()?;
    for (i, c) in fo.back_to_front().into_iter().enumerate() {
        let ct = lua.create_table()?;
        ct.set("at", vec![c.at.0, c.at.1])?;
        ct.set("x", c.at.0)?;
        ct.set("y", c.at.1)?;
        ct.set("z", c.z)?;
        ct.set("r", c.r)?;
        ct.set("squash", c.squash)?;
        ct.set("tilt", c.tilt)?;
        ct.set("fill", c.fill)?;
        ct.set("lit", c.lit)?;
        ct.set("shade", c.shade)?;
        ct.set("limb", c.limb + 1)?;
        ct.set("mass", c.mass + 1)?;
        clumps.set(i + 1, ct)?;
    }
    t.set("clumps", clumps)?;
    let b = fo.bounds();
    t.set("bounds", vec![b.0, b.1, b.2, b.3])?;
    t.set("grain", fo.grain())?;
    let (s1, f1) = (st.clone(), fo.clone());
    t.set("mask", lua.create_function(move |_, _: Variadic<Value>| Ok(wrap(f1.mask(frame(&s1)?))))?)?;
    let (s1, f1) = (st.clone(), fo.clone());
    t.set("lit", lua.create_function(move |_, _: Variadic<Value>| Ok(wrap(f1.lit(frame(&s1)?))))?)?;
    let (s1, f1) = (st.clone(), fo.clone());
    t.set("envelope", lua.create_function(move |_, (_, reach): (Value, Option<f32>)| Ok(wrap(f1.envelope(frame(&s1)?, reach.unwrap_or(6.0)))))?)?;
    let (s1, f1) = (st.clone(), fo.clone());
    t.set("gaps", lua.create_function(move |_, (_, reach): (Value, Option<f32>)| Ok(wrap(f1.gaps(frame(&s1)?, reach.unwrap_or(6.0)))))?)?;
    Ok(t)
}

// ---------------------------------------------------------------- meadows

/// sward{region=mask, horizon=, near=, height=, spacing=, thin=, smallest=, blades={lo, hi},
///       fan=, curl=, flowers=, kinds=, patch=, patch_size=, wind={lean=, gust=, period=, seed=}, seed=}
fn sward(lua: &Lua, st: &S, o: Table) -> Result<Table> {
    check_keys(&o, &["region", "horizon", "near", "height", "spacing", "thin", "smallest", "blades", "fan", "curl", "flowers", "kinds", "patch", "patch_size", "wind", "seed"], "sward")?;
    let region = mask_of(&o.get::<Value>("region")?)?;
    let mut sw = paint::Sward::default();
    macro_rules! over {
        ($($f:ident),*) => {$( if let Some(v) = num(&o, stringify!($f))? { sw.$f = v; } )*};
    }
    over!(horizon, near, height, spacing, thin, smallest, fan, curl, flowers, patch, patch_size);
    if let Some(k) = o.get::<Option<u32>>("kinds")? {
        sw.kinds = k;
    }
    if let Some(b) = o.get::<Option<Vec<u32>>>("blades")? {
        sw.blades = (b.first().copied().unwrap_or(3), b.get(1).copied().unwrap_or(7));
    }
    if let Some(w) = o.get::<Option<Table>>("wind")? {
        check_keys(&w, &["lean", "gust", "period", "seed"], "wind")?;
        sw.wind.lean = num(&w, "lean")?.unwrap_or(sw.wind.lean);
        sw.wind.gust = num(&w, "gust")?.unwrap_or(sw.wind.gust);
        sw.wind.period = num(&w, "period")?.unwrap_or(sw.wind.period);
        sw.wind.seed = w.get::<Option<u64>>("seed")?.unwrap_or(sw.wind.seed);
    }
    let seed = seed_of(st, &o)?;
    let tufts = sw.grow(&region, seed);
    let out = lua.create_table()?;
    for (i, tf) in tufts.iter().enumerate() {
        let t = lua.create_table()?;
        t.set("x", tf.at.0)?;
        t.set("y", tf.at.1)?;
        t.set("scale", tf.scale)?;
        t.set("height", tf.height)?;
        t.set("lean", tf.lean)?;
        t.set("lush", tf.lush)?;
        let blades = lua.create_table()?;
        for (k, b) in tf.blades.iter().enumerate() {
            blades.set(k + 1, vec![vec![b[0].0, b[0].1], vec![b[1].0, b[1].1], vec![b[2].0, b[2].1]])?;
        }
        t.set("blades", blades)?;
        if let Some(fl) = &tf.flower {
            let ft = lua.create_table()?;
            ft.set("x", fl.at.0)?;
            ft.set("y", fl.at.1)?;
            ft.set("r", fl.r)?;
            ft.set("kind", fl.kind)?;
            t.set("flower", ft)?;
        }
        out.set(i + 1, t)?;
    }
    Ok(out)
}

// ---------------------------------------------------------------- setup

/// The whole canvas's frame, for userdata methods that only get `lua`.
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
            lua.create_function(move |_, (a, b): (Option<i64>, Option<i64>)| {
                let r = s1.borrow_mut().rng.f() as f64;
                Ok(match (a, b) {
                    (None, _) => Value::Number(r),
                    (Some(m), None) => Value::Integer(1 + ((r * m as f64).floor() as i64).min(m - 1)),
                    (Some(m), Some(n)) => Value::Integer(m + ((r * (n - m + 1) as f64).floor() as i64).min(n - m)),
                })
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

    // canvas{style=, aspect=, seed=}
    {
        let st = st.clone();
        g.set(
            "canvas",
            lua.create_function(move |lua, o: Option<Table>| {
                let o = o.unwrap_or(lua.create_table()?);
                check_keys(&o, &["style", "aspect", "seed", "palette", "size"], "canvas")?;
                if st.borrow().canvas.is_some() {
                    return err("the canvas is already set up (canvas{} is the first chunk; undo back past it to change it)");
                }
                let name: String = o.get::<Option<String>>("style")?.unwrap_or_else(|| "friedrich".into());
                let mut sty = match name.as_str() {
                    "friedrich" => Style::friedrich(),
                    "friedrich_early" => Style::friedrich_early(),
                    o => return err(format!("style {o:?}: friedrich or friedrich_early")),
                };
                let palname = o.get::<Option<String>>("palette")?;
                if let Some(pn) = &palname {
                    sty.palette = palette_named(pn)?;
                }
                // size: the painting's width in mm (the style's by default)
                if let Some(mm) = num(&o, "size")? {
                    if !(50.0..=5000.0).contains(&mm) {
                        return err("size: the canvas width in mm, 50 to 5000");
                    }
                    sty.width_mm = mm;
                }
                let aspect = num(&o, "aspect")?.unwrap_or(1.4);
                if !(0.2..=5.0).contains(&aspect) {
                    return err("aspect: width / height, between 0.2 and 5");
                }
                let seed = o.get::<Option<u64>>("seed")?.unwrap_or(1);
                let width = st.borrow().width;
                let crop = st.borrow().crop;
                let c = if crop.is_some() { sty.prepare_window(width, aspect, seed, crop) } else { sty.prepare(width, aspect, seed) };
                let h = c.height();
                let pal = Pal(Rc::new(sty.palette.clone()));
                {
                    let mut s = st.borrow_mut();
                    s.seed = seed;
                    s.rng = Rng::new(mixseed(seed, s.chunk, 0xC0FFEE));
                    s.clock0 = c.clock();
                    s.clock = 0.0;
                    s.canvas = Some(c);
                    s.style = Some(Rc::new(sty));
                    let sz = num(&o, "size")?.map(|mm| format!(", size={mm}")).unwrap_or_default();
                    s.setup = Some(match &palname {
                        Some(pn) => format!("style={name:?}, palette={pn:?}, aspect={aspect}, seed={seed}{sz}"),
                        None => format!("style={name:?}, aspect={aspect}, seed={seed}{sz}"),
                    });
                }
                let gl = lua.globals();
                // whole numbers as Lua integers (so `print(H)` says 714, not 714.0)
                let hv = if h.fract() == 0.0 { Value::Integer(h as i64) } else { Value::Number(h as f64) };
                gl.set("W", 1000)?;
                gl.set("H", hv.clone())?;
                gl.set("pal", pal)?;
                Ok(hv)
            })?,
        )?;
    }

    // colors
    g.set("color", lua.create_function(|_, v: Value| Ok(Col(rgb_of(&v)?)))?)?;
    g.set("rgb", lua.create_function(|_, (r, gg, b): (f32, f32, f32)| {
        let f = paint::color::srgb_to_linear;
        Ok(Col([f(r / 255.0), f(gg / 255.0), f(b / 255.0)]))
    })?)?;
    g.set("mix", lua.create_function(|_, (a, b, t, mode): (Value, Value, f32, Option<String>)| Ok(Col(mix(rgb_of(&a)?, rgb_of(&b)?, t, mix_mode(mode.as_deref())?))))?)?;
    // gradient({{0, "#..."}, {0.5, "#..."}, ...}, t, mode?)
    g.set("gradient", lua.create_function(|_, (stops, t, mode): (Table, f32, Option<String>)| {
        let mut s = Vec::new();
        for p in stops.sequence_values::<Table>() {
            let p = p?;
            s.push((p.get::<f32>(1)?, rgb_of(&p.get::<Value>(2)?)?));
        }
        if s.is_empty() {
            return err("gradient: needs stops {{t, color}, ...}");
        }
        Ok(Col(paint::gradient(&s, t, mix_mode(mode.as_deref())?)))
    })?)?;
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
    // worley{seed=, period=, jitter=}: cells (stones, cracked mud, clumps)
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
    // shift(color, dL, da, db): the same color moved in OKLab (darker, bluer, ...)
    g.set("shift", lua.create_function(|_, (c, dl, da, db): (Value, f32, Option<f32>, Option<f32>)| {
        Ok(Col(paint::shift(rgb_of(&c)?, dl, da.unwrap_or(0.0), db.unwrap_or(0.0))))
    })?)?;
    // palette(name): a set of tubes
    g.set("palette", lua.create_function(|_, name: String| Ok(Pal(Rc::new(palette_named(&name)?))))?)?;

    // looking at the canvas
    {
        let st = st.clone();
        g.set("sample", lua.create_function(move |_, (x, y, r): (f32, f32, Option<f32>)| {
            let s = st.borrow();
            let c = s.canvas.as_ref().ok_or_else(no_canvas)?;
            Ok(Col(c.under(x, y, r.unwrap_or(1.0))))
        })?)?;
    }

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
        // ribbon(points, widths): a band along a line (a limb, a path, a stream)
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
        let st1 = st.clone();
        g.set("paint", lua.create_function(move |_, (c, o): (Value, Option<Table>)| Ok(PaintU(paint_for(&st1, &c, o.as_ref(), 2.0)?)))?)?;
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
        // glaze(mask or nil, {color=, coats=number|fn, pigment=})
        let st1 = st.clone();
        g.set("glaze", lua.create_function(move |_, (m, o): (Value, Table)| {
            check_keys(&o, &["color", "coats", "pigment", "visible", "behind", "at", "view"], "glaze")?;
            let m = crate::depth::restrict(&st1, &o, mask_opt(m)?)?.0;
            let f = frame(&st1)?;
            let pig = pigment_of(o.get::<Option<String>>("pigment")?.as_deref(), rgb_of(&o.get::<Value>("color")?)?)?;
            let b = support(m.as_deref(), f, 4.0);
            let th = scalar_field(&st1, &o.get::<Option<Value>>("coats")?.unwrap_or(Value::Number(0.5)), b, "coats")?;
            let mut s = st1.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            // a glaze goes over dry paint: the painter waits for what is
            // under it to dry first, and that time passes on the clock
            c.glaze(&pig, m.as_deref(), th);
            let now = c.clock() - s.clock0;
            let waited = now - s.clock;
            if waited > 0.5 {
                let note = format!("glaze: waited {} for the paint under it to dry (clock {:.0} min)\n", span(waited), now);
                s.out.push_str(&note);
            }
            s.clock = now;
            Ok(now)
        })?)?;
    }

    // time
    {
        let st1 = st.clone();
        g.set("dry", lua.create_function(move |_, ()| {
            let mut s = st1.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.dry();
            let now = c.clock() - s.clock0;
            s.clock = now;
            Ok(now)
        })?)?;
        // wait(minutes): the paint ages where it lies (the engine's drying
        // model: open, setting, tacky, touch-dry by pigment, film and oil)
        let st1 = st.clone();
        g.set("wait", lua.create_function(move |_, minutes: f64| {
            if minutes.is_nan() || minutes < 0.0 {
                return err("wait(minutes): want >= 0");
            }
            let mut s = st1.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.wait(minutes as f32);
            let now = c.clock() - s.clock0;
            s.clock = now;
            Ok(now)
        })?)?;
        // drying(x, y): "open", "setting", "tacky" or "dry"
        let st1 = st.clone();
        g.set("drying", lua.create_function(move |_, (x, y): (f32, f32)| {
            let s = st1.borrow();
            let c = s.canvas.as_ref().ok_or_else(no_canvas)?;
            Ok(match c.drying_at(x, y) {
                paint::Stage::Open => "open",
                paint::Stage::Setting => "setting",
                paint::Stage::Tacky => "tacky",
                paint::Stage::Dry => "dry",
            })
        })?)?;
        let st1 = st.clone();
        g.set("clock", lua.create_function(move |_, ()| Ok(st1.borrow().clock))?)?;
    }

    // finishing
    {
        let st1 = st.clone();
        g.set("varnish", lua.create_function(move |_, o: Option<Table>| {
            let (col, coats, vary, seed) = match &o {
                None => (hex("#e6d3a4"), 0.4, 0.12, 98),
                Some(o) => {
                    check_keys(o, &["color", "coats", "vary", "seed"], "varnish")?;
                    let col = match o.get::<Value>("color")? {
                        Value::Nil => hex("#e6d3a4"),
                        v => rgb_of(&v)?,
                    };
                    (col, num(o, "coats")?.unwrap_or(0.4), num(o, "vary")?.unwrap_or(0.12), o.get::<Option<u32>>("seed")?.unwrap_or(98))
                }
            };
            let mut s = st1.borrow_mut();
            let var = Fbm::new(s.seed as u32 + seed, 3, 400.0);
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.dry();
            c.glaze(&Pigment::varnish(col), None, |x, y| coats + vary * var.get(x, y));
            Ok(())
        })?)?;
        let st1 = st.clone();
        g.set("cracks", lua.create_function(move |_, o: Option<Table>| {
            let mut s = st1.borrow_mut();
            let mut k = Cracks::aged(s.seed);
            if let Some(o) = &o {
                check_keys(o, &["island_mm", "ground_um", "width_um", "depth_um", "cupping_um", "dirt", "corners", "vary", "veil", "seed"], "cracks")?;
                // unset: fitted to this canvas's ground (Cracks::aged)
                k.island_mm = num(o, "island_mm")?.or(k.island_mm);
                k.ground_um = num(o, "ground_um")?.or(k.ground_um);
                k.width_um = num(o, "width_um")?.or(k.width_um);
                macro_rules! over {
                    ($($f:ident),*) => {$( if let Some(v) = num(o, stringify!($f))? { k.$f = v; } )*};
                }
                over!(depth_um, cupping_um, dirt, vary, veil);
                if let Some(c) = o.get::<Option<bool>>("corners")? {
                    k.corners = c;
                }
                if let Some(sd) = o.get::<Option<u64>>("seed")? {
                    k.seed = sd;
                }
            }
            s.canvas.as_mut().ok_or_else(no_canvas)?.crack(&k);
            Ok(())
        })?)?;
        let st1 = st.clone();
        g.set("relief", lua.create_function(move |_, (strength, gloss): (Option<f32>, Option<f32>)| {
            let sty = style(&st1)?;
            let mut s = st1.borrow_mut();
            s.canvas.as_mut().ok_or_else(no_canvas)?.relief(strength.unwrap_or(sty.relief.0), gloss.unwrap_or(sty.relief.1));
            Ok(())
        })?)?;
    }

    crate::form::install(lua, st.clone())?;
    crate::world::install(lua, st.clone())?;
    draw_pencil::install(lua, st.clone())?;
    // looking by eye: show() overlays and probe() (look.rs)
    crate::look::install(lua, st.clone())?;
    crate::draw_outline::install(lua, st.clone())?;
    crate::draw_firs::install(lua, st.clone())?;
<<<<<<< HEAD
    crate::draw_trees::install(lua, st.clone())?;
||||||| 862fe6a
=======
    crate::draw_rocks::install(lua, st.clone())?;
>>>>>>> tool-rock

    // trees
    {
        let st1 = st.clone();
        g.set("tree", lua.create_function(move |lua, o: Table| tree(lua, &st1, o))?)?;
        let st1 = st.clone();
        g.set("sward", lua.create_function(move |lua, o: Table| sward(lua, &st1, o))?)?;
    }

    // silence unused warnings for kinds referenced only in docs
    let _ = Kind::Round;
    Ok(())
}


/// A painting-time span in words: "40 min", "5.2 h", "9.8 days".
pub(crate) fn span(minutes: f64) -> String {
    if minutes < 90.0 {
        format!("{minutes:.0} min")
    } else if minutes < 48.0 * 60.0 {
        format!("{:.1} h", minutes / 60.0)
    } else {
        format!("{:.1} days", minutes / (24.0 * 60.0))
    }
}
