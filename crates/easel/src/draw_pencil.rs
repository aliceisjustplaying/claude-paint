//! Drawing in Lua: graphite pencils and black chalk on the ground, the
//! kneaded eraser and fixative (engine: `paint::graphite`).
//!
//! A pencil is a plain Lua table (`{grade="2B", kind="graphite", worn=0}`)
//! with shared methods, so how far its point has worn is part of the Lua
//! heap: undo and rollback restore it with everything else, and a replay
//! blunts it the same way.

use crate::api::{S, check_keys, err, frame, mask_of, num, points, seed_of, wrap};
use mlua::{Lua, Result, Table, Value};
use paint::graphite::{self, Lead, Mark};
use paint::{Canvas, Mask, Shape};

const PENCIL_KEYS: &[&str] = &["grade", "kind"];
const LINE_KEYS: &[&str] = &["pressure", "smooth", "ruler", "tremor", "seed"];
const SKETCH_KEYS: &[&str] = &["pressure", "passes", "wander", "smooth", "tremor", "seed"];
const HATCH_KEYS: &[&str] = &["angle", "spacing", "length", "pressure", "seed"];
const ERASE_KEYS: &[&str] = &["strength", "width"];

/// The lead a pencil table stands for.
fn lead_of(p: &Table) -> Result<Lead> {
    let kind: String = p.get::<Option<String>>("kind")?.unwrap_or_else(|| "graphite".into());
    match kind.as_str() {
        "chalk" => Ok(Lead::chalk()),
        "graphite" => {
            let g: String = p.get::<Option<String>>("grade")?.unwrap_or_else(|| "HB".into());
            Lead::pencil(&g).ok_or_else(|| mlua::Error::runtime(format!("pencil grade {g:?}: 9H..H, F, HB, B..9B (e.g. \"2H\", \"HB\", \"2B\", \"4B\")")))
        }
        o => err(format!("pencil kind {o:?}: graphite or chalk")),
    }
}

/// Millimeters per canvas unit.
fn mm_per_unit(c: &Canvas) -> f32 {
    c.px_mm() * c.window().scale
}

/// A pressure profile: a number, or {start, end}, or {a, b, c, ...} along the line.
fn profile(o: Option<&Table>, default: f32) -> Result<Vec<f32>> {
    let Some(o) = o else { return Ok(vec![default]) };
    match o.get::<Value>("pressure")? {
        Value::Nil => Ok(vec![default]),
        Value::Integer(n) => Ok(vec![n as f32]),
        Value::Number(n) => Ok(vec![n as f32]),
        Value::Table(t) => {
            let v: Vec<f32> = t.sequence_values::<f32>().collect::<Result<_>>()?;
            if v.is_empty() {
                return err("pressure: a number 0..1 or a list of them along the line");
            }
            Ok(v)
        }
        o => err(format!("pressure: want a number or a list, got {}", o.type_name())),
    }
}

/// Draw marks with the pencil `p`, wearing its point; returns mm drawn.
fn draw_marks(st: &S, p: &Table, marks: &[Mark], seed: u64) -> Result<f32> {
    let lead = lead_of(p)?;
    let mut worn: f32 = p.get::<Option<f32>>("worn")?.unwrap_or(0.0);
    let mut drawn = 0.0;
    {
        let mut s = st.borrow_mut();
        let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
        for (k, m) in marks.iter().enumerate() {
            let d = c.draw(&lead, m, worn, seed.wrapping_add(k as u64 * 0x9E37));
            worn += d;
            drawn += d;
        }
    }
    p.set("worn", worn)?;
    Ok(drawn)
}

fn tremor_units(st: &S, o: Option<&Table>) -> Result<f32> {
    if let Some(t) = o.map(|o| num(o, "tremor")).transpose()?.flatten() {
        return Ok(t.max(0.0));
    }
    let s = st.borrow();
    let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
    // a steady hand: about 0.15 mm of sway
    Ok(0.15 / mm_per_unit(c))
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let methods = lua.create_table()?;

    // p:line(pts, {pressure=, smooth=true, ruler=false, tremor=, seed=})
    {
        let st1 = st.clone();
        methods.set("line", lua.create_function(move |lua, (p, pts, o): (Table, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, LINE_KEYS, "line")?;
            }
            let pts = points(&pts)?;
            if pts.len() < 2 {
                return err("line: needs at least two points");
            }
            let prof = profile(o.as_ref(), 0.5)?;
            let smooth = o.as_ref().map(|o| o.get::<Option<bool>>("smooth")).transpose()?.flatten().unwrap_or(true);
            let ruler = o.as_ref().map(|o| o.get::<Option<bool>>("ruler")).transpose()?.flatten().unwrap_or(false);
            let tremor = tremor_units(&st1, o.as_ref())?;
            let seed = match &o {
                Some(o) => seed_of(&st1, o)?,
                None => seed_of(&st1, &lua.create_table()?)?,
            };
            let m = graphite::hand_line(&pts, &prof, smooth, ruler, tremor, seed);
            draw_marks(&st1, &p, &[m], seed)
        })?)?;
    }
    // p:rule(a, b, {pressure=}): a straight line against a ruler
    {
        let st1 = st.clone();
        methods.set("rule", lua.create_function(move |lua, (p, a, b, o): (Table, Value, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["pressure", "seed"], "rule")?;
            }
            let (a, b) = (points(&a)?, points(&b)?);
            if a.len() != 1 || b.len() != 1 {
                return err("rule(a, b): two points {x, y}");
            }
            let prof = profile(o.as_ref(), 0.5)?;
            let seed = match &o {
                Some(o) => seed_of(&st1, o)?,
                None => seed_of(&st1, &lua.create_table()?)?,
            };
            let m = graphite::hand_line(&[a[0], b[0]], &prof, false, true, 0.0, seed);
            draw_marks(&st1, &p, &[m], seed)
        })?)?;
    }
    // p:sketch(pts, {pressure=0.3, passes=3, wander=, smooth=true, seed=})
    {
        let st1 = st.clone();
        methods.set("sketch", lua.create_function(move |lua, (p, pts, o): (Table, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, SKETCH_KEYS, "sketch")?;
            }
            let pts = points(&pts)?;
            if pts.len() < 2 {
                return err("sketch: needs at least two points");
            }
            let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
            let pressure = get("pressure")?.unwrap_or(0.3);
            let passes = get("passes")?.unwrap_or(3.0).clamp(1.0, 12.0) as usize;
            let mmu = {
                let s = st1.borrow();
                mm_per_unit(s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?)
            };
            let wander = get("wander")?.unwrap_or(1.2 / mmu).max(0.0);
            let smooth = o.as_ref().map(|o| o.get::<Option<bool>>("smooth")).transpose()?.flatten().unwrap_or(true);
            let tremor = tremor_units(&st1, o.as_ref())?;
            let seed = match &o {
                Some(o) => seed_of(&st1, o)?,
                None => seed_of(&st1, &lua.create_table()?)?,
            };
            let marks = graphite::sketch_marks(&pts, pressure, passes, wander, smooth, tremor, seed);
            draw_marks(&st1, &p, &marks, seed)
        })?)?;
    }
    // p:hatch(mask, {angle=0.8, spacing=, length=, pressure=0.45, seed=})
    {
        let st1 = st.clone();
        methods.set("hatch", lua.create_function(move |lua, (p, m, o): (Table, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, HATCH_KEYS, "hatch")?;
            }
            let m = mask_of(&m)?;
            let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
            let mmu = {
                let s = st1.borrow();
                mm_per_unit(s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?)
            };
            let angle = get("angle")?.unwrap_or(-0.9);
            // by default ~1.5 mm apart (at least 2.5 units, to read in a
            // look), strokes up to ~12 mm: a hand's hatching
            let spacing = get("spacing")?.unwrap_or((1.5 / mmu).max(2.5));
            let length = get("length")?.unwrap_or(12.0 / mmu);
            let pressure = get("pressure")?.unwrap_or(0.45);
            if !(spacing > 0.0 && length > 0.0) {
                return err("hatch: spacing and length want > 0 (units)");
            }
            let seed = match &o {
                Some(o) => seed_of(&st1, o)?,
                None => seed_of(&st1, &lua.create_table()?)?,
            };
            let marks = graphite::hatch_marks(&m, angle, spacing, length, pressure, seed);
            draw_marks(&st1, &p, &marks, seed)
        })?)?;
    }
    // p:sharpen(): a fresh point
    methods.set("sharpen", lua.create_function(|_, p: Table| p.set("worn", 0.0))?)?;
    // p:width(): the width of the line it draws now, in units (at pressure 0.5)
    {
        let st1 = st.clone();
        methods.set("width", lua.create_function(move |_, p: Table| {
            let lead = lead_of(&p)?;
            let worn: f32 = p.get::<Option<f32>>("worn")?.unwrap_or(0.0);
            let s = st1.borrow();
            let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            Ok(lead.width_mm(worn) / mm_per_unit(c))
        })?)?;
    }
    let meta = lua.create_table()?;
    meta.set("__index", methods)?;
    meta.set("__tostring", lua.create_function(|_, p: Table| {
        let kind: String = p.get::<Option<String>>("kind")?.unwrap_or_else(|| "graphite".into());
        let worn: f32 = p.get::<Option<f32>>("worn")?.unwrap_or(0.0);
        Ok(if kind == "chalk" {
            format!("black chalk (worn {worn:.0} mm)")
        } else {
            format!("pencil {} (worn {worn:.0} mm)", p.get::<Option<String>>("grade")?.unwrap_or_else(|| "HB".into()))
        })
    })?)?;

    // pencil{grade="2B"} or pencil("2B"); pencil{kind="chalk"} or chalk()
    {
        let m1 = meta.clone();
        g.set("pencil", lua.create_function(move |lua, a: Value| {
            let p = lua.create_table()?;
            match a {
                Value::Nil => p.set("grade", "HB")?,
                Value::String(s) => p.set("grade", s)?,
                Value::Table(o) => {
                    check_keys(&o, PENCIL_KEYS, "pencil")?;
                    p.set("grade", o.get::<Option<String>>("grade")?.unwrap_or_else(|| "HB".into()))?;
                    if let Some(k) = o.get::<Option<String>>("kind")? {
                        p.set("kind", k)?;
                    }
                }
                o => return err(format!("pencil: want {{grade=\"2B\"}} or \"2B\", got {}", o.type_name())),
            }
            if p.get::<Option<String>>("kind")?.is_none() {
                p.set("kind", "graphite")?;
            }
            p.set("worn", 0.0)?;
            lead_of(&p)?;
            p.set_metatable(Some(m1.clone()))?;
            Ok(p)
        })?)?;
        let meta = meta.clone();
        g.set("chalk", lua.create_function(move |lua, ()| {
            let p = lua.create_table()?;
            p.set("kind", "chalk")?;
            p.set("worn", 0.0)?;
            p.set_metatable(Some(meta.clone()))?;
            Ok(p)
        })?)?;
    }

    // erase(mask or pts, {strength=0.9, width=}): a kneaded eraser
    {
        let st1 = st.clone();
        g.set("erase", lua.create_function(move |_, (a, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, ERASE_KEYS, "erase")?;
            }
            let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
            let strength = get("strength")?.unwrap_or(0.9);
            let f = frame(&st1)?;
            let m = match &a {
                Value::UserData(_) => (*mask_of(&a)?).clone(),
                Value::Table(_) => {
                    let pts = points(&a)?;
                    let mmu = {
                        let s = st1.borrow();
                        mm_per_unit(s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?)
                    };
                    // a kneaded eraser pinched to a blunt point, ~4 mm
                    let w = get("width")?.unwrap_or(4.0 / mmu).max(0.1);
                    let pts = if pts.len() == 1 { vec![pts[0], (pts[0].0 + 0.01, pts[0].1)] } else { pts };
                    let ws = vec![w; pts.len()];
                    Mask::from_shape(f, Shape::new().ribbon(&pts, &ws)).blur(0.12 * w)
                }
                o => return err(format!("erase: want a mask or points, got {}", o.type_name())),
            };
            let mut s = st1.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            c.erase(&m, strength);
            Ok(())
        })?)?;
    }
    // fix(mask?): fixative binds the drawing (the eraser no longer lifts it)
    {
        let st1 = st.clone();
        g.set("fix", lua.create_function(move |_, a: Value| {
            let m = match &a {
                Value::Nil => None,
                v => Some(mask_of(v)?),
            };
            let mut s = st1.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            c.fix_drawing(m.as_deref());
            Ok(())
        })?)?;
    }
    // drawing_mask(): where the drawing is (1 on a firm line), also under paint
    {
        let st1 = st.clone();
        g.set("drawing_mask", lua.create_function(move |_, ()| {
            let s = st1.borrow();
            let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            Ok(wrap(c.drawing_mask()))
        })?)?;
    }
    Ok(())
}
