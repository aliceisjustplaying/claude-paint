//! Looking at the canvas: small JPEGs an agent can read, with the studio's
//! viewing tricks (value, squint, mirror), a preview of wet paint dried, and
//! the aids for placing marks by eye: a coordinate grid, probes (what is at
//! a point) and overlays of geometry `show()`n before it is painted.
//!
//! Overlays never touch the canvas. `show()` and `probe()` record marks
//! only in a live session (`begin` makes a session live); in a replay they
//! do nothing but return what they were given, so a chunk that shows things
//! replays exactly as if it didn't.

use crate::session::{Ran, Session};
use crate::api::{Col, S, check_keys, err, frame, mask_of, num, points, rgb_of};
use mlua::{AnyUserData, Lua, ObjectLike, Result, Table, Value, Variadic};
use paint::color::{linear_to_srgb, luminance, srgb_to_linear, to_oklab};
use paint::{Canvas, Mask, Rgb, Shape};
use std::path::Path;
use std::rc::Rc;

pub struct View {
    /// Crop in canvas units (x0, y0, x1, y1).
    pub crop: Option<[f32; 4]>,
    pub value: bool,
    pub squint: bool,
    pub mirror: bool,
    /// Show wet paint as it will look once it has leveled and dried.
    pub dried: bool,
    /// Also light the surface relief (implies `dried`).
    pub relief: Option<(f32, f32)>,
    /// Longest side of the image, px (None: 1000, or the crop's own pixels
    /// at `scale`, up to 2400).
    pub size: Option<usize>,
    /// Coordinate grid: Some(0) picks the step from the zoom.
    pub grid: Option<f32>,
    /// Points to probe (units).
    pub probes: Vec<(f32, f32)>,
    /// `--show on|off|clear` (None: leave overlays as they are; "toggle").
    pub show: Option<String>,
    /// Render the crop as it looks on a canvas `scale` × 1000 px wide.
    pub scale: Option<f32>,
    /// How long to wait for a `--scale` crop to catch up, s.
    pub wait: f32,
}

impl Default for View {
    fn default() -> Self {
        View { crop: None, value: false, squint: false, mirror: false, dried: false, relief: None, size: None, grid: None, probes: Vec::new(), show: None, scale: None, wait: 90.0 }
    }
}

const LOOK_ARGS: &str = "--crop x0,y0,x1,y1 --scale 3.2 --grid [step] --probe x,y[;x,y] --show [on|off|clear] --mode value|squint|mirror --dried --relief --size N --wait S";

fn is_num(s: Option<&String>) -> bool {
    s.is_some_and(|s| s.parse::<f32>().is_ok())
}

impl View {
    /// Parse look arguments (see `LOOK_ARGS`).
    pub fn parse(args: &[String]) -> std::result::Result<View, String> {
        let mut v = View::default();
        let mut i = 0;
        while i < args.len() {
            let a = args[i].as_str();
            let mut next = || {
                i += 1;
                args.get(i).cloned().ok_or(format!("{a} needs a value"))
            };
            match a {
                "--crop" => {
                    let s = next()?;
                    let p: Vec<f32> = s.split(',').map(|t| t.trim().parse::<f32>()).collect::<std::result::Result<_, _>>().map_err(|_| format!("--crop {s}: want x0,y0,x1,y1 in units"))?;
                    if p.len() != 4 {
                        return Err(format!("--crop {s}: want x0,y0,x1,y1 in units"));
                    }
                    v.crop = Some([p[0].min(p[2]), p[1].min(p[3]), p[0].max(p[2]), p[1].max(p[3])]);
                }
                "--mode" => {
                    for m in next()?.split(',') {
                        match m.trim() {
                            "normal" | "" => {}
                            "value" | "gray" => v.value = true,
                            "squint" | "blur" => v.squint = true,
                            "mirror" => v.mirror = true,
                            o => return Err(format!("--mode {o}: normal, value, squint, mirror (comma-separated)")),
                        }
                    }
                }
                "--value" => v.value = true,
                "--squint" => v.squint = true,
                "--mirror" => v.mirror = true,
                "--dried" | "--wet" | "--dry" => v.dried = true,
                "--relief" => {
                    v.dried = true;
                    v.relief = Some((f32::NAN, f32::NAN));
                }
                "--size" => v.size = Some(next()?.parse().map_err(|_| "--size N (px)".to_string())?),
                "--grid" => {
                    v.grid = Some(0.0);
                    if is_num(args.get(i + 1)) {
                        let s: f32 = args[i + 1].parse().unwrap();
                        i += 1;
                        if !(s > 0.0) {
                            return Err("--grid step: want > 0 units".into());
                        }
                        v.grid = Some(s);
                    }
                }
                "--probe" => {
                    let s = next()?;
                    for p in s.split([';', ' ']).filter(|p| !p.trim().is_empty()) {
                        let xy: Vec<f32> = p.split(',').map(|t| t.trim().parse::<f32>()).collect::<std::result::Result<_, _>>().map_err(|_| format!("--probe {p}: want x,y (units), several separated by ;"))?;
                        if xy.len() != 2 {
                            return Err(format!("--probe {p}: want x,y (units), several separated by ;"));
                        }
                        v.probes.push((xy[0], xy[1]));
                    }
                }
                "--show" => {
                    v.show = Some("toggle".into());
                    if let Some(s) = args.get(i + 1).filter(|s| matches!(s.as_str(), "on" | "off" | "clear" | "toggle")) {
                        v.show = Some(s.clone());
                        i += 1;
                    }
                }
                "--scale" => {
                    let s: f32 = next()?.parse().map_err(|_| "--scale S: the canvas width in thousands of px (3.2 = as at 3200)".to_string())?;
                    // --scale 3200 means the same as 3.2
                    let s = if s > 50.0 { s / 1000.0 } else { s };
                    if !(0.1..=8.0).contains(&s) {
                        return Err("--scale S: between 0.1 and 8 (3.2 = as at 3200 px)".into());
                    }
                    v.scale = Some(s);
                }
                "--wait" => v.wait = next()?.parse().map_err(|_| "--wait S (seconds)".to_string())?,
                o => return Err(format!("look: unknown argument {o:?} ({LOOK_ARGS})")),
            }
            i += 1;
        }
        if v.scale.is_some() && v.crop.is_none() {
            return Err("--scale renders a window: give --crop x0,y0,x1,y1 too".into());
        }
        Ok(v)
    }
}

// ---------------------------------------------------------------- overlays

#[derive(Clone)]
pub enum Kind {
    /// A mask: tinted by coverage, outlined at its 0.5 level.
    Region(Rc<Mask>),
    /// Points: a polyline (closed: a polygon), dots at the vertices, and
    /// their numbers.
    Path { pts: Vec<(f32, f32)>, closed: bool, line: bool, dots: bool, numbers: bool },
    /// A probe: a small cross.
    Cross(f32, f32),
}

#[derive(Clone)]
pub struct Mark {
    pub kind: Kind,
    pub color: Rgb,
    pub label: Option<String>,
}

/// The overlay a live session keeps between looks (Lua app data).
#[derive(Default)]
pub struct Marks {
    pub items: Vec<Mark>,
    /// Set before each chunk: the chunk's first show() replaces the overlay.
    fresh: bool,
    /// Only a live session records marks (a replay's show() is a no-op).
    live: bool,
    pub hidden: bool,
    /// Which run made the overlay ("chunk 12", "try").
    pub from: String,
    probes: usize,
}

/// Make the session live (if it isn't) and start a run: the next show() or
/// probe() replaces the overlay.
pub fn begin(lua: &Lua, from: &str) {
    if lua.app_data_ref::<Marks>().is_none() {
        lua.set_app_data(Marks::default());
    }
    let mut m = lua.app_data_mut::<Marks>().unwrap();
    m.live = true;
    m.fresh = true;
    m.probes = 0;
    m.from = from.to_string();
}

/// Run a chunk to see what it shows, probes and prints, then take it back:
/// the canvas, the globals, the brushes and the clock are as before, the
/// log doesn't have it, and the undo stack keeps every snapshot it had.
pub fn try_chunk(s: &mut Session, src: &str) -> std::result::Result<Ran, String> {
    begin(&s.lua, "try");
    let depth = s.undo_depth;
    // one extra level, so the try's own snapshot doesn't push out the oldest
    s.undo_depth = depth + 1;
    let r = s.run(src).and_then(|ran| s.undo(1).map(|_| ran));
    s.undo_depth = depth;
    r
}

/// The overlay to draw (none if hidden).
pub fn marks(lua: &Lua) -> (Vec<Mark>, String) {
    match lua.app_data_ref::<Marks>() {
        Some(m) if !m.hidden => (m.items.clone(), m.from.clone()),
        _ => (Vec::new(), String::new()),
    }
}

/// `look --show on|off|clear|toggle`; returns a line for the reply.
pub fn show_cmd(lua: &Lua, cmd: &str) -> String {
    if lua.app_data_ref::<Marks>().is_none() {
        lua.set_app_data(Marks::default());
    }
    let mut m = lua.app_data_mut::<Marks>().unwrap();
    match cmd {
        "clear" => {
            m.items.clear();
            "overlay cleared\n".into()
        }
        c => {
            m.hidden = match c {
                "on" => false,
                "off" => true,
                _ => !m.hidden,
            };
            format!("overlay {} ({} marks)\n", if m.hidden { "hidden" } else { "shown" }, m.items.len())
        }
    }
}

const COLORS: [[f32; 3]; 6] = [[255.0, 64.0, 240.0], [30.0, 230.0, 255.0], [255.0, 226.0, 40.0], [120.0, 255.0, 60.0], [255.0, 140.0, 26.0], [255.0, 60.0, 60.0]];

fn palette_color(i: usize) -> Rgb {
    COLORS[i % COLORS.len()].map(|c| srgb_to_linear(c / 255.0))
}

fn push(lua: &Lua, mk: impl FnOnce(usize) -> Mark) {
    let Some(mut m) = lua.app_data_mut::<Marks>() else { return };
    if !m.live {
        return;
    }
    if m.fresh {
        m.items.clear();
        m.hidden = false;
        m.fresh = false;
    }
    let n = m.items.len();
    m.items.push(mk(n));
}

fn is_live(lua: &Lua) -> bool {
    lua.app_data_ref::<Marks>().is_some_and(|m| m.live)
}

/// show(...): see the README ("Looking by eye").
fn show(lua: &Lua, st: &S, args: Variadic<Value>) -> Result<Value> {
    let first = args.first().cloned().unwrap_or(Value::Nil);
    match (&first, args.get(1)) {
        // show(): clear the overlay
        (Value::Nil, None) => {
            if is_live(lua) {
                let mut m = lua.app_data_mut::<Marks>().unwrap();
                m.items.clear();
                m.fresh = false;
            }
            Ok(Value::Nil)
        }
        // show(x, y, label?)
        (Value::Integer(_) | Value::Number(_), Some(y)) => {
            let x = f32_of(&first)?;
            let y = f32_of(y)?;
            let label = match args.get(2) {
                Some(Value::Nil) | None => None,
                Some(v) => Some(lua_str(lua, v)?),
            };
            push(lua, |n| Mark { kind: Kind::Path { pts: vec![(x, y)], closed: false, line: false, dots: true, numbers: false }, color: palette_color(n), label });
            Ok(first)
        }
        (Value::UserData(_) | Value::Table(_), o) => {
            let o = match o {
                Some(Value::Table(t)) => Some(t.clone()),
                Some(Value::Nil) | None => None,
                Some(v) => return err(format!("show(what, {{options}}): options must be a table, got {}", v.type_name())),
            };
            let mut color = None;
            let mut label = None;
            let (mut closed, mut dots, mut numbers, mut width) = (false, None, None, None);
            if let Some(o) = &o {
                check_keys(o, &["color", "label", "closed", "dots", "numbers", "width", "brush", "pressure"], "show")?;
                if let Some(c) = o.get::<Option<Value>>("color")? {
                    color = Some(rgb_of(&c)?);
                }
                label = o.get::<Option<Value>>("label")?.map(|v| lua_str(lua, &v)).transpose()?;
                closed = o.get::<Option<bool>>("closed")?.unwrap_or(false);
                dots = o.get::<Option<bool>>("dots")?;
                numbers = o.get::<Option<bool>>("numbers")?;
                width = match (o.get::<Value>("width")?, o.get::<Value>("brush")?) {
                    (Value::Nil, Value::Nil) => None,
                    (Value::Nil, Value::UserData(b)) => {
                        let p = num(o, "pressure")?.unwrap_or(0.8);
                        Some(vec![b.call_method::<f32>("mark_width", p).map_err(|_| mlua::Error::runtime("show: brush= wants a brush (brush(\"round\", 4))"))?])
                    }
                    (Value::Table(t), _) => Some(t.sequence_values::<f32>().collect::<Result<_>>()?),
                    (v, _) => Some(vec![f32_of(&v)?]),
                };
            }
            if let Value::UserData(u) = &first {
                let m = mask_of(&first).map_err(|_| mlua::Error::runtime(format!("show: want a mask, points {{{{x, y}}, ...}} or x, y (got {})", ud_name(u))))?;
                push(lua, |n| Mark { kind: Kind::Region(m), color: color.unwrap_or(palette_color(n)), label });
                return Ok(first);
            }
            let pts = points(&first)?;
            if pts.is_empty() {
                return err("show: no points");
            }
            if let Some(ws) = &width {
                if pts.len() < 2 || !(ws.len() == 1 || ws.len() == pts.len()) {
                    return err("show: a path with width= needs >= 2 points and one width (or one per point)");
                }
                if ws.iter().any(|w| !(*w > 0.0)) {
                    return err("show: width= wants > 0 units");
                }
            }
            if !is_live(lua) {
                return Ok(first);
            }
            let dots = dots.unwrap_or(true);
            let numbers = numbers.unwrap_or(pts.len() <= 40);
            if let Some(ws) = width {
                // where a brush of that width would lay paint along the path
                let ws = if ws.len() == 1 { vec![ws[0]; pts.len()] } else { ws };
                let band = Rc::new(Mask::from_shape(frame(st)?, Shape::new().ribbon(&pts, &ws)));
                let lab = label.clone();
                push(lua, |n| Mark { kind: Kind::Region(band), color: color.unwrap_or(palette_color(n)), label: lab });
                let c = lua.app_data_ref::<Marks>().unwrap().items.last().unwrap().color;
                push(lua, |_| Mark { kind: Kind::Path { pts, closed: false, line: true, dots, numbers }, color: c, label: None });
            } else {
                push(lua, |n| Mark { kind: Kind::Path { pts, closed, line: true, dots, numbers }, color: color.unwrap_or(palette_color(n)), label });
            }
            Ok(first)
        }
        (v, _) => err(format!("show: want a mask, points {{{{x, y}}, ...}} or x, y (got {})", v.type_name())),
    }
}

fn ud_name(u: &AnyUserData) -> String {
    match u.type_name() {
        Ok(n) => n.to_string_lossy().to_string(),
        _ => "userdata".into(),
    }
}

fn f32_of(v: &Value) -> Result<f32> {
    match v {
        Value::Integer(i) => Ok(*i as f32),
        Value::Number(n) => Ok(*n as f32),
        o => err(format!("want a number, got {}", o.type_name())),
    }
}

fn lua_str(lua: &Lua, v: &Value) -> Result<String> {
    let ts: mlua::Function = lua.globals().get("tostring")?;
    ts.call::<String>(v.clone())
}

pub fn hex(c: Rgb) -> String {
    let b = |v: f32| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", b(c[0]), b(c[1]), b(c[2]))
}

/// The view or world a probe reads depth from: `given`, else the first
/// global (by name) holding a view, else one holding a world.
fn find_world(lua: &Lua, given: Option<AnyUserData>) -> Result<Option<(String, AnyUserData)>> {
    use crate::world::{ViewU, WorldU};
    if let Some(u) = given {
        if u.is::<ViewU>() || u.is::<WorldU>() {
            return Ok(Some(("given".into(), u)));
        }
        return err("probe(x, y, v): v must be a view (w:view()) or a world");
    }
    let mut found: Vec<(u8, String, AnyUserData)> = Vec::new();
    for kv in lua.globals().pairs::<Value, Value>() {
        let (k, v) = kv?;
        if let (Value::String(k), Value::UserData(u)) = (k, v) {
            let rank = if u.is::<ViewU>() {
                0
            } else if u.is::<WorldU>() {
                1
            } else {
                continue;
            };
            found.push((rank, k.to_str()?.to_string(), u));
        }
    }
    found.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    Ok(found.into_iter().next().map(|(_, k, u)| (k, u)))
}

/// What is at (x, y): the color, its OKLab value, how far the paint there
/// has dried, the wet film, and (with a world) what the eye sees there.
pub fn probe_at(lua: &Lua, c: &Canvas, x: f32, y: f32, given: Option<AnyUserData>) -> Result<Table> {
    let f = c.frame();
    if !(x >= 0.0 && y >= 0.0 && x < f.width() && y < f.height()) {
        return err(format!("probe({x}, {y}): outside the canvas (0..{}, 0..{})", f.width(), f.height()));
    }
    let col = c.under(x, y, 1.0);
    let lab = to_oklab(col);
    let t = lua.create_table()?;
    t.set("x", x)?;
    t.set("y", y)?;
    t.set("color", Col(col))?;
    t.set("hex", hex(col))?;
    t.set("L", lab[0])?;
    t.set("a", lab[1])?;
    t.set("b", lab[2])?;
    t.set("value", luminance(col))?;
    t.set(
        "drying",
        match c.drying_at(x, y) {
            paint::Stage::Open => "open",
            paint::Stage::Setting => "setting",
            paint::Stage::Tacky => "tacky",
            paint::Stage::Dry => "dry",
        },
    )?;
    t.set("wet_um", c.wet_um(x, y))?;
    if let Some((name, u)) = find_world(lua, given)? {
        t.set("world", name)?;
        if u.is::<crate::world::ViewU>() {
            let p: Table = u.call_method("at", (x, y))?;
            for k in ["what", "body", "at", "dist", "lit"] {
                t.set(k, p.get::<Value>(k)?)?;
            }
        } else if let Ok(Some(g)) = u.call_method::<Option<Vec<f32>>>("to_ground", (x, y)) {
            t.set("what", "ground")?;
            if g.len() == 3 {
                t.set("scale", u.call_method::<f32>("scale_at", g[2])?)?;
                t.set("dist", (g[0] * g[0] + g[2] * g[2]).sqrt())?;
            }
            t.set("at", g)?;
        }
    }
    Ok(t)
}

/// One line describing a probe table.
pub fn probe_text(t: &Table) -> Result<String> {
    let (x, y): (f32, f32) = (t.get("x")?, t.get("y")?);
    let mut s = format!(
        "({x:.0}, {y:.0}): {} · OKLab L {:.3} a {:+.3} b {:+.3} · {}",
        t.get::<String>("hex")?,
        t.get::<f32>("L")?,
        t.get::<f32>("a")?,
        t.get::<f32>("b")?,
        t.get::<String>("drying")?
    );
    let wet: f32 = t.get("wet_um")?;
    if wet > 0.05 {
        s.push_str(&format!(", wet {wet:.0} µm"));
    }
    if let Some(w) = t.get::<Option<String>>("world")? {
        let what: String = t.get::<Option<String>>("what")?.unwrap_or_default();
        s.push_str(&format!(" · {w}: {what}"));
        if let Some(b) = t.get::<Option<i64>>("body")? {
            s.push_str(&format!(" {b}"));
        }
        let far = what == "sky" || what == "off";
        if let Some(d) = t.get::<Option<f32>>("dist")?.filter(|d| d.is_finite() && !far) {
            s.push_str(&format!(", {d:.1} m away"));
        }
        if let Some(a) = t.get::<Option<Vec<f32>>>("at")?.filter(|a| !far && a.len() == 3 && a.iter().all(|v| v.is_finite())) {
            s.push_str(&format!(" at X {:.1} Y {:.1} Z {:.1} m", a[0], a[1], a[2]));
        }
        if let Some(sc) = t.get::<Option<f32>>("scale")? {
            s.push_str(&format!(", {sc:.1} units/m"));
        }
    }
    Ok(s)
}

/// Register show() and probe().
pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let s1 = st.clone();
    g.set("show", lua.create_function(move |lua, args: Variadic<Value>| show(lua, &s1, args))?)?;
    let s1 = st.clone();
    g.set(
        "probe",
        lua.create_function(move |lua, (x, y, v): (f32, f32, Option<AnyUserData>)| {
            let t = {
                let s = s1.borrow();
                let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet"))?;
                probe_at(lua, c, x, y, v)?
            };
            push(lua, |n| {
                let color = palette_color(n);
                Mark { kind: Kind::Cross(x, y), color, label: None }
            });
            // number the probe crosses in the order the chunk made them
            if is_live(lua) {
                let mut m = lua.app_data_mut::<Marks>().unwrap();
                m.probes += 1;
                let k = m.probes;
                if let Some(last) = m.items.last_mut() {
                    last.label = Some(format!("P{k}"));
                }
            }
            Ok(t)
        })?,
    )?;
    Ok(())
}

// ---------------------------------------------------------------- rendering

/// Units -> output pixels (and back).
#[derive(Clone, Copy)]
struct Map {
    s: f32,
    px0: f32,
    py0: f32,
    kx: f32,
    ky: f32,
    ow: usize,
    mirror: bool,
}

impl Map {
    fn to(&self, x: f32, y: f32) -> (f32, f32) {
        let ox = (x * self.s - self.px0) * self.kx;
        let oy = (y * self.s - self.py0) * self.ky;
        (if self.mirror { self.ow as f32 - ox } else { ox }, oy)
    }
    fn from(&self, ox: f32, oy: f32) -> (f32, f32) {
        let ox = if self.mirror { self.ow as f32 - ox } else { ox };
        ((ox / self.kx + self.px0) / self.s, (oy / self.ky + self.py0) / self.s)
    }
}

struct Img {
    w: usize,
    h: usize,
    px: Vec<Rgb>,
}

const INK: Rgb = [0.0, 0.0, 0.0];

impl Img {
    fn blend(&mut self, x: i64, y: i64, c: Rgb, a: f32) {
        if x < 0 || y < 0 || x >= self.w as i64 || y >= self.h as i64 || a <= 0.0 {
            return;
        }
        let p = &mut self.px[y as usize * self.w + x as usize];
        let a = a.min(1.0);
        for q in 0..3 {
            p[q] += (c[q] - p[q]) * a;
        }
    }
    fn rect(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, c: Rgb, a: f32) {
        for y in y0..y1 {
            for x in x0..x1 {
                self.blend(x, y, c, a);
            }
        }
    }
    /// An antialiased segment of width `w` px.
    fn seg(&mut self, a: (f32, f32), b: (f32, f32), w: f32, c: Rgb, alpha: f32) {
        let r = w * 0.5 + 1.0;
        let (x0, x1) = ((a.0.min(b.0) - r).floor() as i64, (a.0.max(b.0) + r).ceil() as i64);
        let (y0, y1) = ((a.1.min(b.1) - r).floor() as i64, (a.1.max(b.1) + r).ceil() as i64);
        let (x0, x1) = (x0.max(0), x1.min(self.w as i64));
        let (y0, y1) = (y0.max(0), y1.min(self.h as i64));
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l2 = (dx * dx + dy * dy).max(1e-9);
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let t = (((px - a.0) * dx + (py - a.1) * dy) / l2).clamp(0.0, 1.0);
                let (ex, ey) = (px - a.0 - t * dx, py - a.1 - t * dy);
                let d = (ex * ex + ey * ey).sqrt();
                let cov = (w * 0.5 + 0.5 - d).clamp(0.0, 1.0);
                self.blend(x, y, c, alpha * cov);
            }
        }
    }
    fn disc(&mut self, c0: (f32, f32), r: f32, c: Rgb, alpha: f32) {
        for y in (c0.1 - r - 1.0).floor() as i64..=(c0.1 + r + 1.0).ceil() as i64 {
            for x in (c0.0 - r - 1.0).floor() as i64..=(c0.0 + r + 1.0).ceil() as i64 {
                let d = ((x as f32 + 0.5 - c0.0).powi(2) + (y as f32 + 0.5 - c0.1).powi(2)).sqrt();
                self.blend(x, y, c, alpha * (r + 0.5 - d).clamp(0.0, 1.0));
            }
        }
    }
    /// Text on a dark plate, top-left at (x, y); returns its width.
    fn text(&mut self, x: i64, y: i64, s: &str, fs: i64, c: Rgb) -> i64 {
        let n = s.chars().count() as i64;
        let (w, h) = (n * 4 * fs + fs, 7 * fs);
        self.rect(x, y, x + w, y + h, INK, 0.62);
        for (i, ch) in s.chars().enumerate() {
            let g = glyph(ch);
            for row in 0..5 {
                for col in 0..3 {
                    if g[row] & (4 >> col) != 0 {
                        let (gx, gy) = (x + fs + (i as i64 * 4 + col as i64) * fs, y + fs + row as i64 * fs);
                        self.rect(gx, gy, gx + fs, gy + fs, c, 1.0);
                    }
                }
            }
        }
        w
    }
}

/// A 3×5 pixel font (rows top to bottom; bit 4 = left column).
fn glyph(c: char) -> [u8; 5] {
    match c.to_ascii_uppercase() {
        '0' => [7, 5, 5, 5, 7],
        '1' => [2, 6, 2, 2, 7],
        '2' => [7, 1, 7, 4, 7],
        '3' => [7, 1, 3, 1, 7],
        '4' => [5, 5, 7, 1, 1],
        '5' => [7, 4, 7, 1, 7],
        '6' => [7, 4, 7, 5, 7],
        '7' => [7, 1, 1, 2, 2],
        '8' => [7, 5, 7, 5, 7],
        '9' => [7, 5, 7, 1, 7],
        '-' => [0, 0, 7, 0, 0],
        '+' => [0, 2, 7, 2, 0],
        '.' => [0, 0, 0, 0, 2],
        ',' => [0, 0, 0, 2, 4],
        ':' => [0, 2, 0, 2, 0],
        '=' => [0, 7, 0, 7, 0],
        '/' => [1, 1, 2, 4, 4],
        '(' => [1, 2, 2, 2, 1],
        ')' => [4, 2, 2, 2, 4],
        '#' => [5, 7, 5, 7, 5],
        '_' => [0, 0, 0, 0, 7],
        '?' => [7, 1, 2, 0, 2],
        'A' => [2, 5, 7, 5, 5],
        'B' => [6, 5, 6, 5, 6],
        'C' => [3, 4, 4, 4, 3],
        'D' => [6, 5, 5, 5, 6],
        'E' => [7, 4, 6, 4, 7],
        'F' => [7, 4, 6, 4, 4],
        'G' => [3, 4, 5, 5, 3],
        'H' => [5, 5, 7, 5, 5],
        'I' => [7, 2, 2, 2, 7],
        'J' => [1, 1, 1, 5, 2],
        'K' => [5, 5, 6, 5, 5],
        'L' => [4, 4, 4, 4, 7],
        'M' => [5, 7, 7, 5, 5],
        'N' => [6, 5, 5, 5, 5],
        'O' => [2, 5, 5, 5, 2],
        'P' => [6, 5, 6, 4, 4],
        'Q' => [2, 5, 5, 6, 3],
        'R' => [6, 5, 6, 5, 5],
        'S' => [3, 4, 2, 1, 6],
        'T' => [7, 2, 2, 2, 2],
        'U' => [5, 5, 5, 5, 7],
        'V' => [5, 5, 5, 5, 2],
        'W' => [5, 5, 7, 7, 5],
        'X' => [5, 5, 2, 5, 5],
        'Y' => [5, 5, 2, 2, 2],
        'Z' => [7, 1, 2, 4, 7],
        _ => [0, 0, 0, 0, 0],
    }
}

/// A round grid step (units) of at least `min`.
fn nice_step(min: f32) -> f32 {
    let mut p = 10f32.powf(min.max(1e-3).log10().floor());
    loop {
        for m in [1.0, 2.0, 2.5, 5.0] {
            if m * p >= min {
                return m * p;
            }
        }
        p *= 10.0;
    }
}

fn fmt_units(v: f32) -> String {
    if (v - v.round()).abs() < 1e-3 { format!("{}", v.round() as i64) } else { format!("{v:.1}") }
}

/// Lines that read over light and dark paint: darken the light, lighten the dark.
fn grid_line(img: &mut Img, x: i64, y: i64, a: f32) {
    if x < 0 || y < 0 || x >= img.w as i64 || y >= img.h as i64 {
        return;
    }
    let l = luminance(img.px[y as usize * img.w + x as usize]);
    let c = if l > 0.16 { [0.0, 0.01, 0.03] } else { [0.75, 0.92, 1.0] };
    img.blend(x, y, c, a);
}

fn draw_grid(img: &mut Img, m: &Map, step: f32, fs: i64) {
    let ppu = m.s * m.kx;
    let major = if step > 0.0 { step } else { nice_step(90.0 / ppu) };
    let minor = [5.0, 4.0, 2.0].into_iter().map(|d| major / d).find(|s| s * ppu >= 14.0);
    let (u0, v0) = m.from(0.0, 0.0);
    let (u1, v1) = m.from(img.w as f32, img.h as f32);
    let (ua, ub) = (u0.min(u1), u0.max(u1));
    let (va, vb) = (v0.min(v1), v0.max(v1));
    let thick = if fs >= 3 { 2 } else { 1 };
    let mut lines = |st: f32, alpha: f32, w: i64| {
        let mut k = (ua / st).ceil();
        while k * st <= ub {
            let (ox, _) = m.to(k * st, 0.0);
            for dx in 0..w {
                for y in 0..img.h as i64 {
                    grid_line(img, ox.floor() as i64 + dx, y, alpha);
                }
            }
            k += 1.0;
        }
        let mut k = (va / st).ceil();
        while k * st <= vb {
            let (_, oy) = m.to(0.0, k * st);
            for dy in 0..w {
                for x in 0..img.w as i64 {
                    grid_line(img, x, oy.floor() as i64 + dy, alpha);
                }
            }
            k += 1.0;
        }
    };
    if let Some(mi) = minor {
        lines(mi, 0.2, 1);
    }
    lines(major, 0.55, thick);
    // labels along the top and left edges
    let lab: Rgb = [0.95, 0.95, 0.85];
    let mut k = (ua / major).ceil();
    let mut last = -1000;
    let mut xs = Vec::new();
    while k * major <= ub {
        xs.push(k * major);
        k += 1.0;
    }
    if m.mirror {
        xs.reverse();
    }
    for u in xs {
        let (ox, _) = m.to(u, 0.0);
        let x = ox.round() as i64 + 3;
        if x > last && x < img.w as i64 - 2 * fs {
            last = x + img.text(x, 2, &fmt_units(u), fs, lab) + 4 * fs;
        }
    }
    let mut k = (va / major).ceil();
    let mut last = 7 * fs + 4;
    while k * major <= vb {
        let (_, oy) = m.to(0.0, k * major);
        let y = oy.round() as i64 + 3;
        if y > last && y < img.h as i64 - 16 * fs {
            img.text(2, y, &fmt_units(k * major), fs, lab);
            last = y + 9 * fs;
        }
        k += 1.0;
    }
    let legend = match minor {
        Some(mi) => format!("GRID {} / {} UNITS", fmt_units(major), fmt_units(mi)),
        None => format!("GRID {} UNITS", fmt_units(major)),
    };
    img.text(2, img.h as i64 - 7 * fs - 2, &legend, fs, lab);
}

fn draw_mark(img: &mut Img, m: &Map, mk: &Mark, fs: i64) {
    let lw = if fs >= 3 { 2.5 } else { 1.6 };
    match &mk.kind {
        Kind::Region(mask) => {
            let (w, h) = (img.w, img.h);
            let mut inside = vec![false; w * h];
            let (mut bx, mut by) = (i64::MAX, i64::MAX);
            for oy in 0..h {
                for ox in 0..w {
                    let (u, v) = m.from(ox as f32 + 0.5, oy as f32 + 0.5);
                    let a = mask.sample(u, v).clamp(0.0, 1.0);
                    if a > 0.01 {
                        img.blend(ox as i64, oy as i64, mk.color, 0.22 * a);
                    }
                    if a >= 0.5 {
                        inside[oy * w + ox] = true;
                        if (oy as i64, ox as i64) < (by, bx) {
                            (bx, by) = (ox as i64, oy as i64);
                        }
                    }
                }
            }
            let t = if fs >= 3 { 2 } else { 1 };
            for oy in 0..h {
                for ox in 0..w {
                    let i = oy * w + ox;
                    let edge = (ox + 1 < w && inside[i] != inside[i + 1]) || (oy + 1 < h && inside[i] != inside[i + w]);
                    if edge {
                        for d in 0..t {
                            img.blend(ox as i64 + d, oy as i64, mk.color, 0.95);
                            img.blend(ox as i64, oy as i64 + d, mk.color, 0.95);
                        }
                    }
                }
            }
            if let Some(l) = &mk.label
                && bx != i64::MAX
            {
                img.text(bx, (by - 8 * fs).max(0), l, fs, mk.color);
            }
        }
        Kind::Path { pts, closed, line, dots, numbers } => {
            let o: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| m.to(x, y)).collect();
            let mut segs: Vec<((f32, f32), (f32, f32))> = o.windows(2).map(|p| (p[0], p[1])).collect();
            if *closed && o.len() > 2 {
                segs.push((o[o.len() - 1], o[0]));
            }
            if *line {
                for &(a, b) in &segs {
                    img.seg(a, b, lw + 2.0, INK, 0.5);
                }
                for &(a, b) in &segs {
                    img.seg(a, b, lw, mk.color, 1.0);
                }
            }
            if *dots {
                let r = if o.len() == 1 { 4.0 } else { 2.5 } * fs as f32 / 2.0;
                for &p in &o {
                    img.disc(p, r + 1.2, INK, 0.7);
                    img.disc(p, r, mk.color, 1.0);
                }
            }
            if *numbers && o.len() > 1 {
                for (i, &p) in o.iter().enumerate() {
                    img.text(p.0 as i64 + 2 * fs, p.1 as i64 + fs, &(i + 1).to_string(), fs, mk.color);
                }
            }
            if let (Some(l), Some(p)) = (&mk.label, o.first()) {
                img.text(p.0 as i64 + 4 * fs, p.1 as i64 - 8 * fs, l, fs, mk.color);
            }
        }
        Kind::Cross(x, y) => {
            let p = m.to(*x, *y);
            let a = 5.0 * fs as f32;
            for (d0, d1) in [((-a, 0.0), (a, 0.0)), ((0.0, -a), (0.0, a))] {
                img.seg((p.0 + d0.0, p.1 + d0.1), (p.0 + d1.0, p.1 + d1.1), lw + 2.0, INK, 0.7);
            }
            for (d0, d1) in [((-a, 0.0), (a, 0.0)), ((0.0, -a), (0.0, a))] {
                img.seg((p.0 + d0.0, p.1 + d0.1), (p.0 + d1.0, p.1 + d1.1), lw, mk.color, 1.0);
            }
            if let Some(l) = &mk.label {
                img.text((p.0 + a * 0.6) as i64, (p.1 + a * 0.4) as i64, l, fs, mk.color);
            }
        }
    }
}

/// Render what the painter asked to see (with the overlay `marks`) and
/// write it as a JPEG. The canvas may be a crop window (a `--scale` look).
pub fn look(c: &Canvas, relief_default: (f32, f32), v: &View, marks: &[Mark], out: &Path) -> std::result::Result<(usize, usize), String> {
    let f = c.window();
    let px: Vec<Rgb> = if v.dried {
        let mut d = c.clone();
        d.dry();
        if let Some((s, g)) = v.relief {
            let (s, g) = if s.is_nan() { relief_default } else { (s, g) };
            d.relief(s, g);
        }
        d.pixels().to_vec()
    } else {
        c.seen()
    };
    // crop: units -> whole-canvas pixels -> pixels of the held window
    let (wx0, wy0, wx1, wy1) = (f.x0, f.y0, f.x0 + f.w, f.y0 + f.h);
    let (x0, y0, x1, y1) = match v.crop {
        None => (wx0, wy0, wx1, wy1),
        Some([a, b, cc, d]) => {
            let s = f.scale;
            let cl = |u: f32, lo: usize, hi: usize| ((u * s).round().max(0.0) as usize).clamp(lo, hi);
            let r = (cl(a, wx0, wx1), cl(b, wy0, wy1), cl(cc, wx0, wx1), cl(d, wy0, wy1));
            if r.2 <= r.0 || r.3 <= r.1 {
                return Err("--crop is empty or outside the canvas".into());
            }
            r
        }
    };
    let (cw, ch) = (x1 - x0, y1 - y0);
    let long = cw.max(ch);
    let size = v.size.unwrap_or(if v.scale.is_some() { long.clamp(1000, 2400) } else { 1000 });
    // fit to size: average down, or enlarge by whole pixels (so pixels stay honest)
    let (ow, oh, img): (usize, usize, Vec<Rgb>) = if long > size {
        let k = size as f32 / long as f32;
        let (ow, oh) = (((cw as f32 * k).round() as usize).max(1), ((ch as f32 * k).round() as usize).max(1));
        let mut img = vec![[0.0f32; 3]; ow * oh];
        for oy in 0..oh {
            let (sy0, sy1) = (y0 + oy * ch / oh, (y0 + (oy + 1) * ch / oh).max(y0 + oy * ch / oh + 1));
            for ox in 0..ow {
                let (sx0, sx1) = (x0 + ox * cw / ow, (x0 + (ox + 1) * cw / ow).max(x0 + ox * cw / ow + 1));
                let mut acc = [0.0f32; 3];
                for y in sy0..sy1 {
                    for x in sx0..sx1 {
                        let p = px[(y - wy0) * f.w + x - wx0];
                        for q in 0..3 {
                            acc[q] += p[q];
                        }
                    }
                }
                let n = ((sy1 - sy0) * (sx1 - sx0)) as f32;
                img[oy * ow + ox] = [acc[0] / n, acc[1] / n, acc[2] / n];
            }
        }
        (ow, oh, img)
    } else {
        // round the enlargement up while the image stays within 1.6 × size
        let up = size.div_ceil(long);
        let k = if long * up * 5 <= size * 8 { up } else { (size / long).max(1) };
        let (ow, oh) = (cw * k, ch * k);
        let img = (0..ow * oh).map(|i| px[(y0 - wy0 + (i / ow) / k) * f.w + x0 - wx0 + (i % ow) / k]).collect();
        (ow, oh, img)
    };
    let mut img = img;
    if v.squint {
        // half-closed eyes: detail goes, the big shapes and values stay
        img = blur(&img, ow, oh, (ow.max(oh) as f32 * 0.012).max(2.0));
    }
    if v.value {
        for p in img.iter_mut() {
            let l = luminance(*p);
            *p = [l; 3];
        }
    }
    if v.mirror {
        for row in img.chunks_mut(ow) {
            row.reverse();
        }
    }
    // the aids, drawn over the picture in output pixels
    let map = Map { s: f.scale, px0: x0 as f32, py0: y0 as f32, kx: ow as f32 / cw as f32, ky: oh as f32 / ch as f32, ow, mirror: v.mirror };
    let fs = if ow.max(oh) >= 1500 { 3 } else { 2 };
    let mut im = Img { w: ow, h: oh, px: img };
    if let Some(step) = v.grid {
        draw_grid(&mut im, &map, step, fs);
    }
    for mk in marks {
        draw_mark(&mut im, &map, mk, fs);
    }
    for (i, &(x, y)) in v.probes.iter().enumerate() {
        draw_mark(&mut im, &map, &Mark { kind: Kind::Cross(x, y), color: [1.0, 0.8, 0.0], label: Some((i + 1).to_string()) }, fs);
    }
    let buf: Vec<u8> = im.px.iter().flat_map(|p| p.map(|c| (linear_to_srgb(c) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let file = std::fs::File::create(out).map_err(|e| e.to_string())?;
    let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(file), 88);
    enc.encode(&buf, ow as u32, oh as u32, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok((ow, oh))
}

/// Three box blurs ≈ a Gaussian of sd `r`.
fn blur(img: &[Rgb], w: usize, h: usize, r: f32) -> Vec<Rgb> {
    let k = ((r * 0.6).round() as usize).max(1);
    let mut a = img.to_vec();
    let mut b = a.clone();
    for _ in 0..3 {
        for y in 0..h {
            for x in 0..w {
                let (lo, hi) = (x.saturating_sub(k), (x + k).min(w - 1));
                let mut s = [0.0; 3];
                for i in lo..=hi {
                    for q in 0..3 {
                        s[q] += a[y * w + i][q];
                    }
                }
                let n = (hi - lo + 1) as f32;
                b[y * w + x] = [s[0] / n, s[1] / n, s[2] / n];
            }
        }
        for y in 0..h {
            for x in 0..w {
                let (lo, hi) = (y.saturating_sub(k), (y + k).min(h - 1));
                let mut s = [0.0; 3];
                for j in lo..=hi {
                    for q in 0..3 {
                        s[q] += b[j * w + x][q];
                    }
                }
                let n = (hi - lo + 1) as f32;
                a[y * w + x] = [s[0] / n, s[1] / n, s[2] / n];
            }
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{Session, root};

    const W: usize = 160;

    fn bits(c: &Canvas) -> Vec<u32> {
        c.seen().iter().flat_map(|p| p.map(f32::to_bits)).chain(c.surface_um().iter().map(|v| v.to_bits())).collect()
    }

    const SETUP: [&str; 3] = [
        r##"canvas{style="friedrich", aspect=1.5, seed=3}"##,
        r##"HZ = 300
           work(above(function(x) return HZ end), {hand="broad", color=function(x, y) return mix("#6f84a8", "#e0d4b0", y/HZ) end, angle=0, coverage=2})"##,
        r##"w = world{horizon=HZ, eye=1.7, fov=50, sun={azimuth=-120, elevation=30}}
           v = w:view()
           b = brush("round", 4); b:load("#303830", 0.9); b:stroke({{100, 450}, {700, 400}})"##,
    ];

    fn live(undo: usize) -> Session {
        let mut s = Session::new(W, undo).unwrap();
        for (i, c) in SETUP.iter().enumerate() {
            begin(&s.lua, &format!("chunk {}", i + 1));
            s.run(c).unwrap();
        }
        s
    }

    const SHOWS: &str = r##"
        m = show(ellipse(400, 300, 100))
        pts = show({{50, 50}, {250, 200}, {450, 100}}, {closed=true, label="sheep", color="#00ff00"})
        show(150, 200, "oak")
        p = probe(250, 437)
        assert(p.hex:sub(1, 1) == "#" and p.L > 0 and p.drying == "open" and p.wet_um > 0, p.hex)
        assert(p.what == "ground" and p.dist > 0 and p.world == "v", tostring(p.what))
        q = probe(250, 100, v)
        assert(q.what == "sky")
        show({{50, 500}, {700, 500}}, {width=30})
        show({{50, 500}, {700, 500}}, {brush=b, label="stroke"})
        print(#pts, p.hex)"##;

    #[test]
    fn show_and_probe_leave_the_canvas_and_the_replay_alone() {
        let mut s = live(4);
        let before = bits(&s.canvas().unwrap());
        begin(&s.lua, "chunk 4");
        s.run(SHOWS).unwrap();
        assert_eq!(before, bits(&s.canvas().unwrap()), "showing and probing paint nothing");
        let (marks, from) = marks(&s.lua);
        assert_eq!((marks.len(), from.as_str()), (9, "chunk 4"), "region, polygon, point, 2 probe crosses, 2 x (band + path)");
        // the next chunk's show() replaces the overlay; a chunk without one keeps it
        begin(&s.lua, "chunk 5");
        s.run("x = 1").unwrap();
        assert_eq!(super::marks(&s.lua).0.len(), 9);
        begin(&s.lua, "chunk 6");
        s.run("show(10, 10)").unwrap();
        assert_eq!(super::marks(&s.lua).0.len(), 1);
        assert_eq!(show_cmd(&s.lua, "off"), "overlay hidden (1 marks)\n");
        assert!(super::marks(&s.lua).0.is_empty());
        show_cmd(&s.lua, "toggle");
        assert_eq!(super::marks(&s.lua).0.len(), 1);
        show_cmd(&s.lua, "clear");
        assert!(super::marks(&s.lua).0.is_empty());
        // the log (shows and all) replays to the same canvas, and a replay keeps no overlay
        let mut r = Session::replay(W).unwrap();
        for c in &s.log {
            r.run(&c.src).unwrap();
        }
        assert_eq!(bits(&s.canvas().unwrap()), bits(&r.canvas().unwrap()));
        assert!(r.lua.app_data_ref::<Marks>().is_none_or(|m| m.items.is_empty()));
        // bad arguments fail the chunk (so nothing is logged) in a live session and a replay alike
        for bad in ["show({{1, 2}}, {width=3})", "show(everywhere(), {colour='#fff'})", "show('x')", "probe(-5, 10)"] {
            assert!(s.run(bad).is_err(), "{bad}");
            assert!(r.run(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn try_rolls_back_and_keeps_the_overlay() {
        let mut s = live(2);
        let before = bits(&s.canvas().unwrap());
        let (n, clock) = (s.log.len(), s.st.borrow().clock);
        let ran = try_chunk(&mut s, r##"t = 1; b:load("#ff0000"); b:stroke({{0, 0}, {150, 100}}); wait(60); show({{0, 0}, {150, 100}}); print("tried")"##).unwrap();
        assert!(ran.out.contains("tried"));
        assert_eq!(before, bits(&s.canvas().unwrap()), "the tried stroke is gone");
        assert_eq!((s.log.len(), s.st.borrow().clock), (n, clock));
        assert_eq!(marks(&s.lua).0.len(), 1, "its overlay stays for the next look");
        s.run("assert(t == nil)").unwrap();
        s.undo(1).unwrap();
        // both undo snapshots survived the try
        s.undo(1).unwrap();
        assert_eq!(s.log.len(), n - 1);
        // a failing try changes nothing either
        let e = try_chunk(&mut s, "u = 2; error('stop')").unwrap_err();
        assert!(e.contains("stop"));
        s.run("assert(u == nil)").unwrap();
    }

    #[test]
    fn looks_draw_the_aids_without_touching_the_canvas() {
        let mut s = live(1);
        begin(&s.lua, "chunk 4");
        s.run(SHOWS).unwrap();
        let dir = root().join("target/easel-look-test");
        let c = s.canvas().unwrap().clone();
        let before = bits(&c);
        let (ms, _) = marks(&s.lua);
        let plain = dir.join("plain.jpg");
        let aided = dir.join("aided.jpg");
        let v = View::parse(&[]).unwrap();
        assert_eq!(look(&c, (0.5, 0.1), &v, &[], &plain).unwrap(), (1120, 749), "enlarged 7x (rounded up)");
        let args: Vec<String> = ["--grid", "100", "--probe", "250,450;500,150", "--crop", "100,50,600,500", "--mirror"].iter().map(|s| s.to_string()).collect();
        let v = View::parse(&args).unwrap();
        assert_eq!((v.grid, v.probes.len()), (Some(100.0), 2));
        let (w, h) = look(&c, (0.5, 0.1), &v, &ms, &aided).unwrap();
        assert_eq!((w, h), (1040, 936), "a 500 x 450 unit crop at 0.16 px/unit (80 x 72 px), enlarged 13x (rounded up)");
        assert_eq!(before, bits(&c));
        assert_eq!(before, bits(&s.canvas().unwrap()));
        assert!(View::parse(&["--scale".into(), "3.2".into()]).is_err(), "--scale needs --crop");
        assert_eq!(View::parse(&["--crop".into(), "0,0,10,10".into(), "--scale".into(), "3200".into()]).unwrap().scale, Some(3.2));
    }

    #[test]
    fn a_crop_session_follows_the_log_through_undo() {
        let log: Vec<String> = SETUP.iter().map(|c| c.trim_end().to_string()).collect();
        let mut crops = crate::crop::Crops::default();
        let crop = [200.0, 150.0, 600.0, 450.0];
        let two = crops.get(320, crop, 666.7, &log[..2], 300.0).unwrap();
        let three = crops.get(320, crop, 666.7, &log, 300.0).unwrap();
        assert!(three.note.contains("current"), "{}", three.note);
        assert_eq!(three.canvas.window().scale, 0.32);
        assert!(three.canvas.window().w < 320, "a window, not the whole canvas");
        assert_ne!(bits(&two.canvas), bits(&three.canvas));
        // the live session undid a chunk: the crop session undoes it too
        let back = crops.get(320, crop, 666.7, &log[..2], 300.0).unwrap();
        assert_eq!(bits(&two.canvas), bits(&back.canvas));
        // and following the log matches painting the window from scratch
        let again = crops.get(320, [250.0, 200.0, 550.0, 400.0], 666.7, &log, 300.0).unwrap();
        let mut fresh = Session::replay(320).unwrap();
        fresh.st.borrow_mut().crop = Some(paint::Crop { units: [170.0, 120.0, 630.0, 480.0], margin: 40.0 });
        for c in &log {
            fresh.run(c).unwrap();
        }
        assert_eq!(bits(&again.canvas), bits(&fresh.canvas().unwrap()));
    }
}
