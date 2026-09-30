//! Legacy: the easel's Lua API as it was before the tube box, so logs
//! painted then replay (notes/amnesia3, notes/amnesia4, notes/loops,
//! notes/lab, notes/lab2, notes/time, notes/fixes: 2026-09-23 to 09-25).
//!
//! A log asks for it in its first chunk: `canvas{style="friedrich",
//! palette="friedrich_1820", ...}` (a named style and palette; a canvas
//! today is given its size, linen and ground). Only then are the old
//! globals set (`mix`, `gradient`, `color`, `rgb`, `shift`, `palette`,
//! `pal`, `paint`, `sample`, `glaze`, `dry`, `clock`, `show`, `rest`,
//! `timesheet`, `tree`, `sward`, `haze`, `tree_in`, `tree_group`, `rock`,
//! `fir`, `fir_wood`, and `w:sky{}`, `w:clouds{}`, `w:ranges{}` on a
//! world), `work`/`stipple` take `color=` and brushes load colors; a canvas
//! set up as today never sees any of it. Only the replay build has it.
//!
//! What they meant, on today's engine:
//! - the styles are `Style::oil_with` on the palettes they named, which are
//!   today's boxes under new names (friedrich_1820 = cobalt box,
//!   friedrich_early = smalt box, `_greens` the same with green tubes), with
//!   the old relief;
//! - `color=` (a color, a function of (x, y), a sky or clouds) was the look
//!   the engine aimed a palette mixture at, stroke by stroke. The aiming is
//!   gone; the color is laid as raw paint (`Handling::color`), as thin as
//!   the pass's medium made it (`paint_numbers`). `pal=` and `aim=` are
//!   accepted and have nothing left to choose;
//! - `glaze(m, {color=, coats=, pigment=})` pours a Kubelka-Munk film after
//!   drying the canvas, `dry()` dries it, and the finishing verbs dry it
//!   first, as they did; `canvas{hand=true}` asked for hand time, which is
//!   always on now; `rest(hours)` waits and starts a new sitting;
//! - trees, meadows, sky, clouds, ranges and haze are the old models,
//!   restored as they were at 2aab443 (growth.rs, atmos.rs, from
//!   crates/paint); broadleaf trees, rocks and firs as they were just
//!   before fdfb0ff took them out (broadleaf.rs, rock.rs, fir.rs from
//!   crates/paint; draw_trees.rs, draw_rocks.rs, draw_firs.rs from the
//!   easel). They only make shapes and colors, which the log paints.
//!
//! Replays are not pixel-identical to the old renders (the engine has
//! changed since), but they run to the end and make the same picture.

use crate::api::{self, FieldBox, S, check_keys, err, frame, mask_of, no_canvas, num, pair, seed_of, support, wrap};
use crate::time::{self, Verb};
use crate::world::WorldU;
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value, Variadic};
use paint::color::{luminance, to_oklab};
use paint::{Handling, Mask, Mix, Palette, Pigment, Rgb, Stipple, Style, hex};
use std::rc::Rc;
use std::sync::Arc;

pub mod atmos;
pub mod broadleaf;
mod draw_firs;
mod draw_rocks;
mod draw_trees;
pub mod fir;
pub mod growth;
pub mod rock;

use atmos::{Cloud, CloudField, CloudPoint, Clouds, Haze, RangeLayer, Ranges, Silhouette, Sky, SkyField};
use growth::{Habit, Skeleton, Sward};

/// The styles' names: a canvas with one of them is a legacy canvas.
const FRIEDRICH: &str = "Caspar David Friedrich (legacy)";
const FRIEDRICH_EARLY: &str = "Caspar David Friedrich, early (legacy)";

/// Is this studio's canvas a legacy canvas (set up by `canvas{style=}`)?
pub fn on(st: &S) -> bool {
    st.borrow().style.as_ref().is_some_and(|s| s.name == FRIEDRICH || s.name == FRIEDRICH_EARLY)
}

/// Does `canvas{}` ask for a legacy canvas?
pub fn asks(o: &Table) -> Result<bool> {
    Ok(o.contains_key("style")? || o.contains_key("palette")?)
}

// ---------------------------------------------------------------- styles, palettes

const PALETTES: &str = "friedrich_1820, friedrich_early, friedrich_1820_greens, friedrich_early_greens";

fn palette_named(name: &str, engine: u32) -> Result<Palette> {
    let mut p = match name {
        "friedrich_1820" | "friedrich" => Palette::cobalt_box(),
        "friedrich_early" => Palette::smalt_box(),
        "friedrich_1820_greens" | "greens" => Palette::cobalt_box_greens(),
        "friedrich_early_greens" => Palette::smalt_box_greens(),
        o => return err(format!("palette {o:?}: {PALETTES}")),
    };
    p.engine = engine;
    Ok(p)
}

/// Friedrich's style as it was: today's oil style (the same linen, ground
/// and brushes) on the 1820 palette, with the old relief.
fn friedrich(palette: Palette) -> Style {
    Style { name: FRIEDRICH, relief: (0.2, 0.02), ..Style::oil_with(palette) }
}

/// The early style: a large canvas, coarser linen and a red ground, on the
/// early palette.
fn friedrich_early(palette: Palette) -> Style {
    use paint::{Apply, Ground, Linen};
    Style {
        name: FRIEDRICH_EARLY,
        width_mm: 1714.0,
        linen: Linen { warp_per_cm: 12.0, weft_per_cm: 11.0, ..Linen::fine(1) },
        ground: vec![
            Ground { color: hex("#b0583a"), hiding: 0.85, um: 110.0, stiff: 0.25, apply: Apply::Knife { texture: 0.35 } },
            Ground { color: hex("#c9ad8a"), hiding: 0.8, um: 70.0, stiff: 0.3, apply: Apply::Knife { texture: 0.3 } },
            Ground { color: hex("#d8c7ab"), hiding: 0.8, um: 30.0, stiff: 0.5, apply: Apply::Roller },
        ],
        ..friedrich(palette)
    }
}

/// `canvas{style=, palette=, aspect=, seed=, size=, hand=}`: a legacy
/// canvas, and the legacy globals. (`hand=true` turned hand time on; it is
/// always on now.)
pub fn canvas(lua: &Lua, st: &S, o: Table) -> Result<Value> {
    check_keys(&o, &["style", "aspect", "seed", "palette", "size", "hand"], "canvas")?;
    if st.borrow().canvas.is_some() {
        return err("the canvas is already set up (canvas{} is the first chunk)");
    }
    let engine = st.borrow().tubes.engine;
    let name: String = o.get::<Option<String>>("style")?.unwrap_or_else(|| "friedrich".into());
    let palname = o.get::<Option<String>>("palette")?;
    let mut sty = match name.as_str() {
        "friedrich" => friedrich(palette_named(palname.as_deref().unwrap_or("friedrich_1820"), engine)?),
        "friedrich_early" => friedrich_early(palette_named(palname.as_deref().unwrap_or("friedrich_early"), engine)?),
        o => return err(format!("style {o:?}: friedrich or friedrich_early")),
    };
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
    let mut c = sty.prepare(width, aspect, seed);
    let h = c.height();
    let pal = Pal(Rc::new(sty.palette.clone()));
    {
        let mut s = st.borrow_mut();
        s.seed = seed;
        s.rng = paint::Rng::new(api::mixseed(seed, s.chunk, 0xC0FFEE));
        s.clock0 = c.clock();
        s.clock = 0.0;
        time::start(&mut c);
        s.hand = time::Hand::default();
        s.canvas = Some(c);
        s.style = Some(Rc::new(sty));
        let sz = num(&o, "size")?.map(|mm| format!(", size={mm}")).unwrap_or_default();
        s.setup = Some(match &palname {
            Some(pn) => format!("legacy: style={name:?}, palette={pn:?}, aspect={aspect}, seed={seed}{sz}"),
            None => format!("legacy: style={name:?}, aspect={aspect}, seed={seed}{sz}"),
        });
    }
    let g = lua.globals();
    let hv = if h.fract() == 0.0 { Value::Integer(h as i64) } else { Value::Number(h as f64) };
    g.set("W", 1000)?;
    g.set("H", hv.clone())?;
    g.set("pal", pal)?;
    install(lua, st)?;
    Ok(hv)
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
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_meta_method(MetaMethod::ToString, |_, c, ()| Ok(hexstr(c.0)));
        m.add_method("hex", |_, c, ()| Ok(hexstr(c.0)));
        m.add_method("mix", |_, c, (o, t, mode): (Value, f32, Option<String>)| Ok(Col(paint::color::mix(c.0, rgb_of(&o)?, t, mix_mode(mode.as_deref())?))));
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

/// A color field: a color, a function(x, y) returning one (sampled on the
/// field grid), a sky or clouds (read natively).
fn color_field(st: &S, v: &Value, b: (f32, f32, f32, f32)) -> Result<FieldBox<Rgb>> {
    if let Value::UserData(u) = v {
        if let Ok(s) = u.borrow::<SkyU>() {
            let s = s.0.clone();
            return Ok(Box::new(move |x, y| s.at(x, y)));
        }
        if let Ok(c) = u.borrow::<CloudsU>() {
            let (sky, cf) = (c.sky.clone(), c.field.clone());
            return Ok(Box::new(move |x, y| cf.color(&sky, x, y)));
        }
    }
    if let Value::Function(f) = v {
        let g = api::sample::<3>(st, f, b, |r| rgb_of(&r))?;
        Ok(Box::new(move |x, y| g.get(x, y)))
    } else {
        let c = rgb_of(v)?;
        Ok(Box::new(move |_, _| c))
    }
}

/// `color_over=`: {shift={dL, da, db}} or function(x, y, under) -> color,
/// read against what is on the canvas before the pass (on the field grid).
fn over_field(st: &S, v: &Value, b: (f32, f32, f32, f32)) -> Result<FieldBox<Rgb>> {
    let under = |x: f32, y: f32| -> Result<Rgb> { Ok(st.borrow().canvas.as_ref().ok_or_else(no_canvas)?.under(x, y, api::FIELD_STEP * 0.5)) };
    match v {
        Value::Table(t) => {
            let sh: Vec<f32> = t.get("shift")?;
            let (dl, da, db) = (sh.first().copied().unwrap_or(0.0), sh.get(1).copied().unwrap_or(0.0), sh.get(2).copied().unwrap_or(0.0));
            let g = api::grid::<3>(st, b, |x, y| Ok(paint::shift(under(x, y)?, dl, da, db)))?;
            Ok(Box::new(move |x, y| g.get(x, y)))
        }
        Value::Function(f) => {
            let g = api::grid::<3>(st, b, |x, y| rgb_of(&f.call::<Value>((x, y, Col(under(x, y)?)))?))?;
            Ok(Box::new(move |x, y| g.get(x, y)))
        }
        o => err(format!("color_over: want {{shift={{dL, da, db}}}} or function(x, y, under), got {}", o.type_name())),
    }
}

/// An angle field read natively: a legacy rock's field (`r:field("plane")`).
pub fn angle_field(v: &Value) -> Option<FieldBox<f32>> {
    match v {
        Value::UserData(u) => u.borrow::<draw_rocks::RockFieldU>().ok().map(|f| f.angle(0.0)),
        _ => None,
    }
}

// ---------------------------------------------------------------- paint

/// Raw paint for an old color: how much it hides and how stiff it is, from
/// the medium it was mixed with. Aimed paint (the default) was mixed to
/// look like its color where it was laid, so it hides well even when thin;
/// masstone paint (a glaze) is the color of its pigment, veiled by its
/// medium.
fn paint_numbers(medium: f32, masstone: bool) -> (f32, f32) {
    let m = medium.clamp(0.0, 0.98);
    let hiding = if masstone { (0.85 * (1.0 - m)).max(0.04) } else { (0.95 - 0.7 * m).clamp(0.2, 0.95) };
    (hiding, 0.8 * (1.0 - m) * (1.0 - m))
}

/// The medium each hand mixed its paint with (the old `Style`'s).
fn hand_medium(hand: &str) -> f32 {
    match hand {
        "broad" => 0.45,
        "detail" => 0.1,
        "hatch" => 0.15,
        "glaze" => 0.9,
        "scumble" => 0.5,
        _ => 0.2,
    }
}

const WORK_KEYS: &[&str] = &["color", "color_over", "jitter", "medium", "pal", "aim", "paint"];

/// The options `work` takes today and the old ones.
pub fn work_keys(st: &S, today: &[&'static str]) -> Vec<&'static str> {
    let mut k = today.to_vec();
    if on(st) {
        k.extend(WORK_KEYS);
    }
    k
}

/// A legacy `work`: its color (see the module docs).
pub fn work(st: &S, o: &Table, h: &mut Handling, hand: &str, blending: bool, b: (f32, f32, f32, f32)) -> Result<()> {
    let masstone = hand == "glaze" || matches!(o.get::<Value>("aim")?, Value::String(s) if &*s.to_str()? == "masstone");
    let (hid, stiff) = match pair(o, "paint")? {
        Some(p) => p,
        None => paint_numbers(num(o, "medium")?.unwrap_or(hand_medium(hand)), masstone),
    };
    h.hiding = hid;
    h.stiff = stiff;
    match (o.get::<Value>("color")?, o.get::<Value>("color_over")?) {
        (Value::Nil, Value::Nil) if blending => {}
        (Value::Nil, Value::Nil) => return err("work: needs a color (a \"#rrggbb\", a color, or function(x, y) returning one) or color_over"),
        (Value::Nil, v) => h.color = over_field(st, &v, b)?,
        (v, _) => h.color = color_field(st, &v, b)?,
    }
    if let Some((l, hue)) = pair(o, "jitter")? {
        h.jitter = (l, hue);
    }
    Ok(())
}

const STIPPLE_KEYS: &[&str] = &["color", "color_over", "jitter", "medium", "pal", "aim", "paint", "fade"];

/// The options `stipple` takes today and the old ones.
pub fn stipple_keys(st: &S, today: &[&'static str]) -> Vec<&'static str> {
    let mut k = today.to_vec();
    if on(st) {
        k.extend(STIPPLE_KEYS);
    }
    k
}

/// A legacy `stipple`: its color (stippled paint was thinned with half
/// medium unless the log said otherwise).
pub fn stipple(st: &S, o: &Table, sp: &mut Stipple, b: (f32, f32, f32, f32)) -> Result<()> {
    let (hid, stiff) = match pair(o, "paint")? {
        Some(p) => p,
        None => paint_numbers(num(o, "medium")?.unwrap_or(0.5), false),
    };
    sp.hiding = hid;
    sp.stiff = stiff;
    match (o.get::<Value>("color")?, o.get::<Value>("color_over")?) {
        (Value::Nil, Value::Nil) => return err("stipple: needs a color or color_over"),
        (Value::Nil, v) => sp.color = over_field(st, &v, b)?,
        (v, _) => sp.color = color_field(st, &v, b)?,
    }
    if let Some((l, hue)) = pair(o, "jitter")? {
        sp.jitter = (l, hue);
    }
    Ok(())
}

/// `b:load(color, amount?, {medium=, at=, coats=, pal=, raw=, hiding=,
/// stiff=})` on a legacy canvas: the color as paint (None: not legacy, or
/// not a color, so the pile it is).
pub fn brushload(st: &S, p: &Value, extra: &Value) -> Result<Option<(paint::Paint, Rgb)>> {
    if !on(st) {
        return Ok(None);
    }
    if let Value::UserData(u) = p
        && let Ok(pu) = u.borrow::<PaintU>()
    {
        return Ok(Some((pu.0, pu.0.color)));
    }
    if !matches!(p, Value::String(_) | Value::Table(_)) && !matches!(p, Value::UserData(u) if u.borrow::<Col>().is_ok()) {
        return Ok(None);
    }
    let want = rgb_of(p)?;
    let o = match extra {
        Value::Nil => None,
        Value::Table(t) => Some(t),
        v => return err(format!("load: options are a table, got {}", v.type_name())),
    };
    Ok(Some((paint_of(want, o)?, want)))
}

fn paint_of(want: Rgb, o: Option<&Table>) -> Result<paint::Paint> {
    let Some(o) = o else {
        let (h, s) = paint_numbers(0.2, false);
        return Ok(paint::Paint::new(want, h, s));
    };
    check_keys(o, &["medium", "at", "coats", "pal", "raw", "hiding", "stiff"], "load")?;
    if o.get::<Option<bool>>("raw")?.unwrap_or(false) {
        return Ok(paint::Paint::new(want, num(o, "hiding")?.unwrap_or(0.85), num(o, "stiff")?.unwrap_or(0.8)));
    }
    let (h, s) = paint_numbers(num(o, "medium")?.unwrap_or(0.2), false);
    Ok(paint::Paint::new(want, h, s))
}

#[derive(Clone, Copy)]
pub struct PaintU(pub paint::Paint);
impl UserData for PaintU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_meta_method(MetaMethod::ToString, |_, p, ()| Ok(format!("paint({}, hiding {:.2}, stiff {:.2})", hexstr(p.0.color), p.0.hiding(), p.0.stiff)));
    }
}

/// A palette (`pal`, `palette(name)`): its tubes, and families of them
/// (`only`, `with`) for `pal=`.
#[derive(Clone)]
pub struct Pal(pub Rc<Palette>);
impl UserData for Pal {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("tubes", |_, p, ()| Ok(p.0.tubes.iter().map(|t| t.name.to_string()).collect::<Vec<_>>()));
        m.add_method("with", |_, p, names: Vec<String>| {
            let all = Palette::tube_box();
            let mut extra = Vec::new();
            for n in &names {
                if !p.0.tubes.iter().any(|t| t.name == n) {
                    extra.push(all.tubes.iter().find(|t| t.name == n).cloned().ok_or_else(|| mlua::Error::runtime(format!("no tube {n:?}")))?);
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
        m.add_meta_method(MetaMethod::ToString, |_, p, ()| Ok(format!("palette {:?}: {}", p.0.name, p.0.tubes.iter().map(|t| t.name).collect::<Vec<_>>().join(", "))));
    }
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

// ---------------------------------------------------------------- globals

fn install(lua: &Lua, st: &S) -> Result<()> {
    let g = lua.globals();
    g.set("color", lua.create_function(|_, v: Value| Ok(Col(rgb_of(&v)?)))?)?;
    g.set("rgb", lua.create_function(|_, (r, gg, b): (f32, f32, f32)| {
        let f = paint::color::srgb_to_linear;
        Ok(Col([f(r / 255.0), f(gg / 255.0), f(b / 255.0)]))
    })?)?;
    g.set("mix", lua.create_function(|_, (a, b, t, mode): (Value, Value, f32, Option<String>)| Ok(Col(paint::color::mix(rgb_of(&a)?, rgb_of(&b)?, t, mix_mode(mode.as_deref())?))))?)?;
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
    g.set("shift", lua.create_function(|_, (c, dl, da, db): (Value, f32, Option<f32>, Option<f32>)| {
        Ok(Col(paint::shift(rgb_of(&c)?, dl, da.unwrap_or(0.0), db.unwrap_or(0.0))))
    })?)?;
    let engine = st.borrow().tubes.engine;
    g.set("palette", lua.create_function(move |_, name: String| Ok(Pal(Rc::new(palette_named(&name, engine)?))))?)?;
    let st1 = st.clone();
    g.set("paint", lua.create_function(move |_, (c, o): (Value, Option<Table>)| {
        let _ = &st1;
        Ok(PaintU(paint_of(rgb_of(&c)?, o.as_ref())?))
    })?)?;
    // sample(x, y, r?): what is on the canvas there
    let st1 = st.clone();
    g.set("sample", lua.create_function(move |_, (x, y, r): (f32, f32, Option<f32>)| {
        let s = st1.borrow();
        Ok(Col(s.canvas.as_ref().ok_or_else(no_canvas)?.under(x, y, r.unwrap_or(1.0))))
    })?)?;
    // show(...): an overlay for looking; nothing on the canvas
    g.set("show", lua.create_function(|_, _: Variadic<Value>| Ok(()))?)?;
    // glaze(mask or nil, {color=, coats=number|fn, pigment=}): dry, then pour a film
    let st1 = st.clone();
    g.set("glaze", lua.create_function(move |_, (m, o): (Value, Table)| {
        check_keys(&o, &["color", "coats", "pigment", "visible", "behind", "at", "view"], "glaze")?;
        let m = match m {
            Value::Nil => None,
            v => Some(mask_of(&v)?),
        };
        let m = crate::depth::restrict(&st1, &o, m)?.0;
        let f = frame(&st1)?;
        let pig = pigment_of(o.get::<Option<String>>("pigment")?.as_deref(), rgb_of(&o.get::<Value>("color")?)?)?;
        let b = support(m.as_deref(), f, 4.0);
        let th = api::scalar_field(&st1, &o.get::<Option<Value>>("coats")?.unwrap_or(Value::Number(0.5)), b, "coats")?;
        time::verb(&st1, Verb::Wait, |s| {
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.dry();
            c.glaze(&pig, m.as_deref(), th);
            Ok(())
        })?;
        Ok(st1.borrow().clock)
    })?)?;
    // dry(): wait until all the paint is touch-dry
    let st1 = st.clone();
    g.set("dry", lua.create_function(move |_, ()| {
        time::verb(&st1, Verb::Wait, |s| {
            s.canvas.as_mut().ok_or_else(no_canvas)?.dry();
            Ok(())
        })?;
        Ok(st1.borrow().clock)
    })?)?;
    let st1 = st.clone();
    g.set("clock", lua.create_function(move |_, ()| {
        time::flush(&st1, true);
        Ok(st1.borrow().clock)
    })?)?;
    // rest(hours = 16): away from the easel; the next mark starts a new
    // sitting with a clean palette. timesheet(): the clock, the sittings,
    // how dry the canvas is and what the hand has done this sitting.
    let sitting = Rc::new(std::cell::Cell::new((1u32, 0.0f64, paint::Tally::default())));
    let (st1, si) = (st.clone(), sitting.clone());
    g.set("rest", lua.create_function(move |_, hours: Option<f64>| {
        let h = hours.unwrap_or(16.0);
        if !(0.0..=24.0 * 365.0).contains(&h) {
            return err("rest(hours): want 0 to a year of hours (default: overnight, 16)");
        }
        time::verb(&st1, Verb::Wait, |s| {
            s.canvas.as_mut().ok_or_else(no_canvas)?.wait((h * 60.0) as f32);
            Ok(())
        })?;
        let mut s = st1.borrow_mut();
        s.hand = time::Hand::default();
        let (n, _, _) = si.get();
        si.set((n + 1, s.clock, s.canvas.as_ref().ok_or_else(no_canvas)?.tally()));
        Ok(s.clock)
    })?)?;
    let (st1, si) = (st.clone(), sitting);
    g.set("timesheet", lua.create_function(move |lua, ()| {
        time::flush(&st1, true);
        let s = st1.borrow();
        let c = s.canvas.as_ref().ok_or_else(no_canvas)?;
        let (n, start, base) = si.get();
        let t: Table = lua.create_table()?;
        let sh = c.stage_shares();
        let k = c.tally().since(&base);
        t.set("clock", s.clock)?;
        t.set("sitting", s.clock - start)?;
        t.set("sittings", n)?;
        t.set("hand", true)?;
        for (i, name) in ["open", "setting", "tacky", "dry"].iter().enumerate() {
            t.set(*name, sh[i])?;
        }
        t.set("strokes", k.strokes)?;
        t.set("touches", k.touches)?;
        t.set("reloads", k.reloads)?;
        t.set("piles", k.remixes)?;
        t.set("hand_min", k.minutes())?;
        Ok(t)
    })?)?;
    let st1 = st.clone();
    g.set("tree", lua.create_function(move |lua, o: Table| tree(lua, &st1, o))?)?;
    let st1 = st.clone();
    g.set("sward", lua.create_function(move |lua, o: Table| sward(lua, &st1, o))?)?;
    g.set("haze", lua.create_function(|_, o: Option<Table>| {
        let o = match o {
            Some(o) => o,
            None => return Ok(HazeU(Haze::new(25_000.0))),
        };
        check_keys(&o, &["visibility", "height", "mist"], "haze")?;
        let mut h = Haze::new(num(&o, "visibility")?.unwrap_or(25_000.0));
        if let Some(k) = num(&o, "height")? {
            h = h.height(k);
        }
        if let Some(t) = o.get::<Option<Table>>("mist")? {
            h = h.mist(t.get(1)?, t.get(2)?, t.get::<Option<f32>>(3)?.unwrap_or(80.0), t.get::<Option<u32>>(4)?.unwrap_or(1));
        }
        Ok(HazeU(h))
    })?)?;
    // round 5's tools: tree_in{}, tree_group{}, rock{}, fir{}, fir_wood{}
    draw_trees::install(lua, st.clone())?;
    draw_rocks::install(lua, st.clone())?;
    draw_firs::install(lua, st.clone())?;
    // the finishing verbs dried the canvas first; varnish had a color
    lua.load(
        r#"
        local varnish0, cracks0, relief0 = varnish, cracks, relief
        if varnish0 then
            varnish = function(o)
                dry()
                local t
                if o then t = {}; for k, v in pairs(o) do if k ~= "color" then t[k] = v end end end
                return varnish0(t)
            end
        end
        if cracks0 then cracks = function(...) dry(); return cracks0(...) end end
        if relief0 then relief = function(...) dry(); return relief0(...) end end
        "#,
    )
    .set_name("legacy")
    .exec()?;
    Ok(())
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
    let (st2, sk2) = (st.clone(), sk.clone());
    t.set("foliage", lua.create_function(move |lua, (_, o): (Value, Option<Table>)| foliage(lua, &st2, &sk2, o))?)?;
    Ok(t)
}

fn foliage(lua: &Lua, st: &S, sk: &Skeleton, o: Option<Table>) -> Result<Table> {
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

/// sward{region=mask, horizon=, near=, height=, spacing=, thin=, smallest=, blades={lo, hi},
///       fan=, curl=, flowers=, kinds=, patch=, patch_size=, wind={lean=, gust=, period=, seed=}, seed=}
fn sward(lua: &Lua, st: &S, o: Table) -> Result<Table> {
    check_keys(&o, &["region", "horizon", "near", "height", "spacing", "thin", "smallest", "blades", "fan", "curl", "flowers", "kinds", "patch", "patch_size", "wind", "seed"], "sward")?;
    let region = mask_of(&o.get::<Value>("region")?)?;
    let mut sw = Sward::default();
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

// ---------------------------------------------------------------- sky, clouds

#[derive(Clone)]
pub struct SkyU(pub Arc<SkyField>);

impl UserData for SkyU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("at", |_, s, (x, y): (f32, f32)| Ok(Col(s.0.at(x, y))));
        m.add_method("value", |_, s, (x, y): (f32, f32)| Ok(s.0.value(x, y)));
        m.add_method("airlight", |_, s, x: f32| Ok(Col(s.0.airlight(x))));
        m.add_method("sunlight", |_, s, ()| Ok(Col(s.0.paint(s.0.sky.sunlight()))));
        m.add_meta_method(MetaMethod::ToString, |_, _, ()| Ok("sky (use as color=, or s:at(x, y))"));
    }
}

#[derive(Clone)]
pub struct CloudsU {
    pub sky: Arc<SkyField>,
    pub field: Arc<CloudField>,
}

fn cloud_point_table(lua: &Lua, p: &CloudPoint) -> Result<Table> {
    let t = lua.create_table()?;
    t.set("alpha", p.alpha)?;
    t.set("lit", p.lit)?;
    t.set("glow", p.glow)?;
    t.set("ambient", p.ambient)?;
    t.set("dist", p.dist)?;
    Ok(t)
}

impl UserData for CloudsU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("at", |_, c, (x, y): (f32, f32)| Ok(Col(c.field.color(&c.sky, x, y))));
        m.add_method("cloud", |_, c, (x, y): (f32, f32)| Ok(Col(c.field.cloud_color(x, y))));
        m.add_method("alpha", |_, c, (x, y): (f32, f32)| Ok(c.field.alpha(x, y)));
        m.add_method("lit", |_, c, (x, y): (f32, f32)| Ok(c.field.lit(x, y)));
        m.add_method("glow", |_, c, (x, y): (f32, f32)| Ok(c.field.glow(x, y)));
        m.add_method("soft", |_, c, (x, y): (f32, f32)| Ok(c.field.soft(x, y)));
        // c:mask{alpha={lo, hi}, lit={lo, hi}, shade={lo, hi}} or c:mask(function(p) ... end)
        m.add_method("mask", |lua, c, o: Value| match o {
            Value::Function(g) => {
                let fr = api::current_frame(lua)?;
                let inv = 1.0 / fr.scale;
                let mut data = vec![0.0f32; fr.w * fr.h];
                for y in 0..fr.h {
                    for x in 0..fr.w {
                        let (ux, uy) = ((x as f32 + 0.5) * inv, (y as f32 + 0.5) * inv);
                        let p = CloudPoint { alpha: c.field.alpha(ux, uy), lit: c.field.lit(ux, uy), glow: c.field.glow(ux, uy), ambient: c.field.ambient(ux, uy), dist: c.field.dist(ux, uy) };
                        if p.alpha > 0.0 {
                            let r: f32 = g.call(cloud_point_table(lua, &p)?)?;
                            data[y * fr.w + x] = r.clamp(0.0, 1.0);
                        }
                    }
                }
                Ok(wrap(Mask { f: fr, data }))
            }
            Value::Table(t) => {
                check_keys(&t, &["alpha", "lit", "shade"], "clouds mask")?;
                let a = pair(&t, "alpha")?.unwrap_or((0.3, 0.8));
                let l = pair(&t, "lit")?;
                let s = pair(&t, "shade")?;
                let fr = api::current_frame(lua)?;
                Ok(wrap(c.field.mask(fr, |p| {
                    let mut v = paint::smoothstep(a.0, a.1, p.alpha);
                    if let Some(l) = l {
                        v *= paint::smoothstep(l.0, l.1, p.lit);
                    }
                    if let Some(s) = s {
                        v *= paint::smoothstep(s.0, s.1, 1.0 - p.lit);
                    }
                    v
                })))
            }
            Value::Nil => {
                let fr = api::current_frame(lua)?;
                Ok(wrap(c.field.mask(fr, |p| paint::smoothstep(0.3, 0.8, p.alpha))))
            }
            o => err(format!("clouds mask: want {{alpha=, lit=, shade=}} or a function, got {}", o.type_name())),
        });
        m.add_meta_method(MetaMethod::ToString, |_, _, ()| Ok("clouds (use as color=, or c:at(x, y))"));
    }
}

fn cloud_of(t: &Table) -> Result<Cloud> {
    let kind: String = t.get::<Option<String>>("kind")?.or(t.get::<Option<String>>(1)?).unwrap_or_else(|| "cumulus".into());
    let n = |k: &str| -> Result<f32> { num(t, k)?.ok_or_else(|| mlua::Error::runtime(format!("{kind} cloud: needs {k}"))) };
    let seed = t.get::<Option<u32>>("seed")?.unwrap_or(1);
    let mut c = match kind.as_str() {
        "cumulus" => {
            check_keys(t, &["kind", "x", "z", "base", "width", "height", "seed", "density", "soft", "wind", "breaks", "heap", "reach"], "cumulus")?;
            Cloud::cumulus(n("x")?, n("z")?, n("base")?, n("width")?, n("height")?, seed)
        }
        "bank" => {
            check_keys(t, &["kind", "x0", "x1", "z", "depth", "base", "top", "seed", "density", "soft", "wind", "breaks", "heap", "reach"], "bank")?;
            Cloud::bank(n("x0")?, n("x1")?, n("z")?, n("depth")?, n("base")?, n("top")?, seed)
        }
        "stratus" => {
            check_keys(t, &["kind", "base", "thick", "cover", "seed", "density", "soft", "wind", "breaks", "heap", "reach"], "stratus")?;
            Cloud::stratus(n("base")?, n("thick")?, n("cover")?, seed)
        }
        o => return err(format!("cloud kind {o:?}: cumulus, bank or stratus")),
    };
    if let Some(d) = num(t, "density")? {
        c = c.density(d);
    }
    if let Some(s) = num(t, "soft")? {
        c = c.soft(s);
    }
    if let Some((a, s)) = pair(t, "wind")? {
        c = c.wind(a, s);
    }
    if let Some((p, a)) = pair(t, "breaks")? {
        c = c.breaks(p, a);
    }
    if let Some(h) = num(t, "heap")? {
        c = c.heap(h);
    }
    if let Some((near, far)) = pair(t, "reach")? {
        c = c.reach(near, far);
    }
    Ok(c)
}

// ---------------------------------------------------------------- haze, ranges

#[derive(Clone, Copy)]
pub struct HazeU(pub Haze);
impl UserData for HazeU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        // air:loss(eye_m, dist_m, height_m, x): how much of a color the air replaces
        m.add_method("loss", |_, h, (eye, dist, hm, x): (f32, f32, Option<f32>, Option<f32>)| Ok(h.0.loss(eye, dist, hm.unwrap_or(0.0), x.unwrap_or(0.0))));
    }
}

fn haze_of(v: &Value) -> Result<Haze> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<HazeU>()?.0),
        o => err(format!("want haze{{...}}, got {}", o.type_name())),
    }
}

#[derive(Clone)]
pub struct RangeU {
    layer: Arc<RangeLayer>,
    world: Arc<paint::World>,
}

impl UserData for RangeU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("crest", |_, r, x: f32| Ok(r.layer.crest(&r.world, x)));
        m.add_method("z_at", |_, r, x: f32| Ok(r.layer.z_at(x)));
        m.add_method("height_at", |_, r, (x, y): (f32, f32)| Ok(r.layer.height_at(&r.world, x, y)));
        m.add_method("haze", |_, r, (air, x, y): (Value, f32, f32)| Ok(r.layer.haze(&r.world, &haze_of(&air)?, x, y)));
        // l:mask(soft?): everything below the crest (hidden parts included)
        m.add_method("mask", |lua, r, soft: Option<f32>| {
            let f = api::current_frame(lua)?;
            let s = soft.unwrap_or(0.6).max(0.05);
            let (l, w) = (r.layer.clone(), r.world.clone());
            let crest: Vec<f32> = (0..f.w).map(|x| l.crest(&w, (x as f32 + 0.5) / f.scale)).collect();
            Ok(wrap(Mask::from_fn(f, |x, y| {
                let c = crest[((x * f.scale) as usize).min(f.w - 1)];
                paint::smoothstep(c - s, c + s, y)
            })))
        });
        m.add_meta_method(MetaMethod::ToString, |_, r, ()| Ok(format!("range({:.0} m off at x=500)", r.layer.z_at(500.0))));
    }
}

fn silhouette_of(s: &str) -> Result<Silhouette> {
    Ok(match s {
        "peak" => Silhouette::Peak,
        "dome" => Silhouette::Dome,
        "plateau" => Silhouette::Plateau,
        "saddle" => Silhouette::Saddle,
        "cliff" => Silhouette::Cliff,
        o => return err(format!("range kind {o:?}: peak, dome, plateau, saddle or cliff")),
    })
}

/// The world's legacy methods (`w:sky{}`, `w:clouds{}`, `w:ranges{}`) on a
/// legacy canvas; nil otherwise, as for any other unknown method.
pub fn world_method(lua: &Lua, key: &str) -> Result<Value> {
    let legacy = lua.app_data_ref::<S>().is_some_and(|st| on(&st));
    if !legacy {
        return Ok(Value::Nil);
    }
    let f = match key {
        // w:sky{haze=, uneven={amount, period_m, seed}, layer={alt, thick, density, uneven, seed},
        //       overcast=, fill=, altitude=, cell=6, exposure=, balance=}
        "sky" => lua.create_function(|_, (w, o): (mlua::UserDataRef<WorldU>, Option<Table>)| {
            let mut sky = Sky::new(w.w.sun);
            let mut cell = 6.0;
            let (mut exposure, mut balance) = (None, None);
            if let Some(o) = &o {
                check_keys(o, &["haze", "uneven", "layer", "overcast", "fill", "altitude", "cell", "exposure", "balance"], "sky")?;
                if let Some(h) = num(o, "haze")? {
                    sky = sky.haze(h);
                }
                if let Some(t) = o.get::<Option<Table>>("uneven")? {
                    sky = sky.uneven(t.get(1)?, t.get::<Option<f32>>(2)?.unwrap_or(30_000.0), t.get::<Option<u32>>(3)?.unwrap_or(1));
                }
                if let Some(t) = o.get::<Option<Table>>("layer")? {
                    sky = sky.layer(t.get(1)?, t.get(2)?, t.get(3)?, t.get::<Option<f32>>(4)?.unwrap_or(0.5), t.get::<Option<u32>>(5)?.unwrap_or(1));
                }
                if let Some(v) = num(o, "overcast")? {
                    sky = sky.overcast(v);
                }
                if let Some(v) = num(o, "fill")? {
                    sky = sky.fill(v);
                }
                if let Some(v) = num(o, "altitude")? {
                    sky = sky.altitude(v);
                }
                cell = num(o, "cell")?.unwrap_or(cell);
                exposure = num(o, "exposure")?;
                balance = num(o, "balance")?;
            }
            let mut f = SkyField::new(sky, &w.w, cell);
            if let Some(k) = exposure {
                f = f.exposure(k);
            }
            if let Some(b) = balance {
                let l = f.sky.sunlight();
                f = f.balance(l, b);
            }
            Ok(SkyU(Arc::new(f)))
        })?,
        // w:clouds{sky=s, cell=2, {kind="cumulus", ...}, {kind="bank", ...}, ...}
        "clouds" => lua.create_function(|_, (w, o): (mlua::UserDataRef<WorldU>, Table)| {
            let sky = match o.get::<Value>("sky")? {
                Value::UserData(u) => u.borrow::<SkyU>()?.0.clone(),
                _ => return err("clouds: needs sky = w:sky{...}"),
            };
            let cell = num(&o, "cell")?.unwrap_or(2.0);
            let list: Vec<Cloud> = o.sequence_values::<Table>().map(|t| cloud_of(&t?)).collect::<Result<_>>()?;
            if list.is_empty() {
                return err("clouds: list at least one {kind=\"cumulus\"|\"bank\"|\"stratus\", ...}");
            }
            let field = Clouds::new(list).field(&sky, &w.w, cell);
            Ok(CloudsU { sky, field: Arc::new(field) })
        })?,
        // w:ranges{near=, far=, count=, seed=, heights={near_m, far_m}, irregular=, oblique=,
        //          kinds={"peak", "dome", ...}, regular=false} -> layers, nearest first
        "ranges" => lua.create_function(|_, (w, o): (mlua::UserDataRef<WorldU>, Table)| {
            check_keys(&o, &["near", "far", "count", "seed", "heights", "irregular", "oblique", "kinds", "regular"], "ranges")?;
            let mut r = Ranges::new(num(&o, "near")?.unwrap_or(4000.0), num(&o, "far")?.unwrap_or(40_000.0), o.get::<Option<usize>>("count")?.unwrap_or(4), o.get::<Option<u32>>("seed")?.unwrap_or(1));
            if let Some((a, b)) = pair(&o, "heights")? {
                r = r.heights(a, b);
            }
            if let Some(k) = num(&o, "irregular")? {
                r = r.irregular(k);
            }
            if let Some(k) = num(&o, "oblique")? {
                r = r.oblique(k);
            }
            if let Some(k) = o.get::<Option<Vec<String>>>("kinds")? {
                let ks: Vec<Silhouette> = k.iter().map(|s| silhouette_of(s)).collect::<Result<_>>()?;
                r = r.kinds(&ks);
            }
            let layers = if o.get::<Option<bool>>("regular")?.unwrap_or(false) { r.regular(&w.w) } else { r.build(&w.w) };
            Ok(layers.into_iter().map(|l| RangeU { layer: Arc::new(l), world: w.w.clone() }).collect::<Vec<_>>())
        })?,
        _ => return Ok(Value::Nil),
    };
    Ok(Value::Function(f))
}

#[cfg(test)]
mod tests {
    use crate::session::{Session, box_for, parse_program};

    /// Replay an old log to its end at a small width; the chunk that fails, if one does.
    fn replays(log: &str) {
        let path = format!("{}/../../notes/{log}", env!("CARGO_MANIFEST_DIR"));
        let text = std::fs::read_to_string(&path).unwrap();
        let mut s = Session::replay_with(200, box_for(Some(&text)).unwrap()).unwrap();
        for (i, c) in parse_program(&text).iter().enumerate() {
            if let Err(e) = s.run(c) {
                panic!("{log}: chunk {} failed:\n{e}", i + 1);
            }
        }
        assert!(s.st.borrow().canvas.is_some());
    }

    #[test]
    fn easel3_free_replays() {
        replays("amnesia3/easel3_free.lua");
    }
    #[test]
    fn easel3_green_replays() {
        replays("amnesia3/easel3_green.lua");
    }
    #[test]
    fn easel3_near_replays() {
        replays("amnesia3/easel3_near.lua");
    }
    #[test]
    fn easel4_free_replays() {
        replays("amnesia4/easel4_free.lua");
    }
    #[test]
    fn easel4_green_replays() {
        replays("amnesia4/easel4_green.lua");
    }
    #[test]
    fn easel4_near_replays() {
        replays("amnesia4/easel4_near.lua");
    }
}
