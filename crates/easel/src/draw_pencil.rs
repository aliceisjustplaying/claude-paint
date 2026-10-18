//! Drawing in Lua: graphite pencils and black chalk on the ground, the
//! kneaded eraser and fixative (engine: `paint::graphite`); with the
//! paper-and-pastel switch, pastel sticks, the finger and `feel` too
//! (`install_paper`; engine: `paint::pastel`).
//!
//! A pencil is a plain Lua table (`{grade="2B", kind="graphite", worn=0}`)
//! with shared methods, so how far its point has worn is part of the Lua
//! heap: a failed chunk restores it with everything else, and a replay
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
const SMUDGE_KEYS: &[&str] = &["force", "pad", "speed"];
/// The pencils' shared metatable, in the Lua registry: `install_paper`
/// gives it the stick's methods.
const PENCIL_META: &str = "easel.pencil";

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

// ---------------------------------------------------------------- pastel sticks

const STICK_KEYS: &[&str] = &["kind", "soft", "diameter", "length"];
const STROKE_KEYS: &[&str] = &["force", "alt", "azimuth", "roll", "speed"];
/// A pencil's pressure 0..1 as a hand's force on a pastel, N (light 0.1–0.5,
/// normal 0.5–2, heavy 2–5: notes/research/pastel_stick_tribology.md §5).
const FORCE_AT_FULL: f32 = 5.0;

fn validate_pastel_path(c: &Canvas, pts: &[(f32, f32)], pad_radius_mm: Option<f32>) -> Result<()> {
    if pts.len() > 4096 {
        return err("pastel path: a path holds at most 4096 points");
    }
    let mmu = mm_per_unit(c);
    if pts.iter().any(|&(x, y)| !x.is_finite() || !y.is_finite() || !(x * mmu).is_finite() || !(y * mmu).is_finite()) {
        return err("pastel path: point coordinates are finite numbers");
    }
    let mut total = 0.0f32;
    let mut samples = 0.0f64;
    for w in pts.windows(2) {
        let len = if pad_radius_mm.is_some() {
            ((w[1].0 * mmu - w[0].0 * mmu).powi(2) + (w[1].1 * mmu - w[0].1 * mmu).powi(2)).sqrt()
        } else {
            ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt() * mmu
        };
        total += len;
        if let Some(r) = pad_radius_mm {
            samples += (len / (r * 0.5)).ceil().max(1.0) as f64;
        }
    }
    if pad_radius_mm.is_none() {
        samples = (total / c.px_mm()).ceil().max(1.0) as f64 + 1.0;
    }
    if !total.is_finite() || !samples.is_finite() || samples > 100_000.0 || (pad_radius_mm.is_none() && samples * pts.len() as f64 > 10_000_000.0) {
        return err("pastel path: too much resampling; draw a shorter path or use fewer points");
    }
    Ok(())
}

/// Whether `p` is a pastel stick (`pastel(pile)`): only with the
/// paper-and-pastel switch, so without it a pencil's methods are as they were.
fn is_stick(st: &S, p: &Table) -> Result<bool> {
    Ok(st.borrow().paper && p.get::<Option<String>>("kind")?.as_deref() == Some("stick"))
}

/// The stick a stick table holds.
pub(crate) fn stick_of(p: &Table) -> Result<paint::pastel::Stick> {
    use paint::pastel::{Section, Stick};
    let section: String = p.get("section")?;
    let size: f32 = p.get("size")?;
    let section = match section.as_str() {
        "square" => Section::Square { side_mm: size },
        _ => Section::Round { d_mm: size },
    };
    let ft: Table = p.get("facets")?;
    let v: Vec<f32> = ft.sequence_values::<f32>().collect::<Result<_>>()?;
    let facets = v.chunks_exact(4).map(|c| [c[0], c[1], c[2], c[3]]).collect();
    Ok(Stick { section, length_mm: p.get("length")?, facets, hardness: p.get("hardness")?, wear: p.get("wear")?, color: [p.get("r")?, p.get("g")?, p.get("b")?] })
}

/// Write a stick's wear back into its table (a part of the Lua heap: a
/// failed chunk puts it back, a replay wears it the same).
fn store_stick(lua: &Lua, p: &Table, s: &paint::pastel::Stick) -> Result<()> {
    let ft = lua.create_table()?;
    for f in &s.facets {
        for v in f {
            ft.push(*v)?;
        }
    }
    p.set("facets", ft)?;
    p.set("length", s.length_mm)?;
    Ok(())
}

/// A number or a list of numbers along `n` points (interpolated by index).
fn along(o: Option<&Table>, key: &str, n: usize, default: f32) -> Result<Vec<f32>> {
    let Some(o) = o else { return Ok(vec![default; n]) };
    match o.get::<Value>(key)? {
        Value::Nil => Ok(vec![default; n]),
        Value::Integer(v) => Ok(vec![v as f32; n]),
        Value::Number(v) => Ok(vec![v as f32; n]),
        Value::Table(t) => {
            let v: Vec<f32> = t.sequence_values::<f32>().collect::<Result<_>>()?;
            if v.is_empty() {
                return err(format!("{key}: a number, or a list of them along the stroke"));
            }
            Ok((0..n)
                .map(|i| {
                    if v.len() == 1 || n == 1 {
                        return v[0];
                    }
                    let u = i as f32 / (n - 1) as f32 * (v.len() - 1) as f32;
                    let k = (u.floor() as usize).min(v.len() - 2);
                    v[k] + (v[k + 1] - v[k]) * (u - k as f32)
                })
                .collect())
        }
        o => err(format!("{key}: want a number or a list, got {}", o.type_name())),
    }
}

/// One stroke of a stick along `pts` (units), each point's force (N), alt,
/// azimuth and roll (degrees) and speed (mm/s); the hand lands and lifts
/// as a hand does (pastel.rs `LAND_S`, `LIFT_S`). Returns the volume laid, mm³.
#[allow(clippy::too_many_arguments)]
fn stick_stroke(lua: &Lua, st: &S, p: &Table, pts: &[(f32, f32)], force: &[f32], alt: &[f32], az: &[f32], roll: &[f32], speed: &[f32]) -> Result<f32> {
    use paint::pastel::{Pose, StrokePoint};
    {
        let s = st.borrow();
        let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
        validate_pastel_path(c, pts, None)?;
    }
    let mut stick = stick_of(p)?;
    let base_roll: f32 = p.raw_get::<Option<f32>>("turned")?.unwrap_or(0.0);
    // (a NaN angle finds no contact and lays nothing, without saying so)
    if !base_roll.is_finite() || alt.iter().chain(az).chain(roll).chain(force).chain(speed).any(|v| !v.is_finite()) {
        return err("a pastel stick's force, alt, azimuth, roll and speed are numbers");
    }
    // (the hand's landing and lifting are the stroke's: pastel.rs)
    let sp: Vec<StrokePoint> = (0..pts.len())
        .map(|i| {
            let v = speed[i].max(1.0);
            StrokePoint {
                x: pts[i].0,
                y: pts[i].1,
                pose: Pose { force: force[i].max(0.0), alt: alt[i].clamp(0.0, 90.0).to_radians(), az: az[i].to_radians(), roll: (base_roll + roll[i]).to_radians() },
                speed: v,
            }
        })
        .collect();
    let laid = crate::time::verb(st, crate::time::Verb::Pass, |s| {
        let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
        let l = c.stick_stroke(&mut stick, &sp);
        c.tally_mut().secs += l.secs as f64 + 0.25;
        Ok(l)
    })?;
    store_stick(lua, p, &stick)?;
    Ok(laid.volume_mm3)
}

/// The stick's way of drawing a pencil's marks: each mark a stroke at the
/// pencil's pressure as force (`FORCE_AT_FULL` at full), the stick held at
/// 60° with its upper end to the lower right (a right hand), at 80 mm/s.
fn stick_marks(lua: &Lua, st: &S, p: &Table, marks: &[Mark]) -> Result<f32> {
    let mut v = 0.0;
    for m in marks {
        // (a mark is dense, every 0.25 units: every eighth point will do)
        let idx: Vec<usize> = (0..m.pts.len()).step_by(8).chain(std::iter::once(m.pts.len() - 1)).collect();
        let pts: Vec<(f32, f32)> = idx.iter().map(|&i| m.pts[i]).collect();
        let force: Vec<f32> = idx.iter().map(|&i| m.pressure[i] * FORCE_AT_FULL).collect();
        let n = pts.len();
        v += stick_stroke(lua, st, p, &pts, &force, &vec![60.0; n], &vec![45.0; n], &vec![0.0; n], &vec![80.0; n])?;
    }
    Ok(v)
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
    let drawn = crate::time::verb(st, crate::time::Verb::Pass, |s| {
        let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
        let mut drawn = 0.0;
        for (k, m) in marks.iter().enumerate() {
            let d = c.draw(&lead, m, worn, seed.wrapping_add(k as u64 * 0x9E37));
            worn += d;
            drawn += d;
        }
        c.tally_mut().draw(marks.len() as u64, drawn as f64);
        Ok(drawn)
    })?;
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
            if is_stick(&st1, &p)? {
                return stick_marks(lua, &st1, &p, &[m]);
            }
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
            if is_stick(&st1, &p)? {
                return stick_marks(lua, &st1, &p, &[m]);
            }
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
            let wander = get("wander")?.unwrap_or(2.0 / mmu).max(0.0);
            let smooth = o.as_ref().map(|o| o.get::<Option<bool>>("smooth")).transpose()?.flatten().unwrap_or(true);
            let tremor = tremor_units(&st1, o.as_ref())?;
            let seed = match &o {
                Some(o) => seed_of(&st1, o)?,
                None => seed_of(&st1, &lua.create_table()?)?,
            };
            let marks = graphite::sketch_marks(&pts, pressure, passes, wander, smooth, tremor, seed);
            if is_stick(&st1, &p)? {
                return stick_marks(lua, &st1, &p, &marks);
            }
            draw_marks(&st1, &p, &marks, seed)
        })?)?;
    }
    // p:hatch(mask, {angle=0.8, spacing=, length=, pressure=0.45, graded=false, seed=}):
    // graded: the mask is a weight (each stroke pressed by its value)
    {
        let st1 = st.clone();
        methods.set("hatch", lua.create_function(move |lua, (p, m, o): (Table, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, if st1.borrow().paper { &["angle", "spacing", "length", "pressure", "graded", "seed"] } else { HATCH_KEYS }, "hatch")?;
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
            let graded = o.as_ref().map(|o| o.get::<Option<bool>>("graded")).transpose()?.flatten().unwrap_or(false);
            let marks = graphite::hatch_marks_graded(&m, angle, spacing, length, pressure, graded, seed);
            if is_stick(&st1, &p)? {
                return stick_marks(lua, &st1, &p, &marks);
            }
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
    // shared by every pencil and held here, out of the heap snapshot's reach: sealed, as
    // the engine's userdata are, so a failed chunk can't change it for good
    let meta = lua.create_table()?;
    meta.set("__index", methods)?;
    meta.set("__metatable", false)?;
    let st1 = st.clone();
    meta.set("__tostring", lua.create_function(move |_, p: Table| {
        let kind: String = p.get::<Option<String>>("kind")?.unwrap_or_else(|| "graphite".into());
        let worn: f32 = p.get::<Option<f32>>("worn")?.unwrap_or(0.0);
        Ok(if kind == "chalk" {
            format!("black chalk (worn {worn:.0} mm)")
        } else if is_stick(&st1, &p)? {
            let s = stick_of(&p)?;
            let name: String = p.get::<Option<String>>("name")?.unwrap_or_default();
            let sec: String = p.get("section")?;
            format!("pastel {sec} {name} ({} MPa, {:.1} mm long, {} facets)", (s.hardness * 10.0).round() / 10.0, s.length_mm, s.facets.len())
        } else {
            format!("pencil {} (worn {worn:.0} mm)", p.get::<Option<String>>("grade")?.unwrap_or_else(|| "HB".into()))
        })
    })?)?;

    lua.set_named_registry_value(PENCIL_META, meta.clone())?;

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
    // fix(mask?): fixative binds the drawing (the eraser no longer lifts it),
    // and a pastel stick's pastel
    {
        let st1 = st.clone();
        g.set("fix", lua.create_function(move |_, a: Value| {
            let m = match &a {
                Value::Nil => None,
                v => Some(mask_of(v)?),
            };
            let mut s = st1.borrow_mut();
            let paper = s.paper;
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            c.fix_drawing(m.as_deref());
            if paper {
                // a light spray wets about 0.15 of the particles (pastel.rs
                // `fix_pastel`; over graphite alone it sets the floor
                // `fix_drawing` just set, and nothing else)
                c.fix_pastel(m.as_deref(), 0.15);
            }
            Ok(())
        })?)?;
    }
    // drawing_guide(): the drawn lines as geometry over the whole canvas (1
    // on a line, unbroken; the same in a crop): the mask to paint into the drawing
    {
        let st1 = st.clone();
        g.set("drawing_guide", lua.create_function(move |_, ()| {
            let s = st1.borrow();
            let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            Ok(wrap(c.drawing_guide()))
        })?)?;
    }
    Ok(())
}

/// The paper-and-pastel switch's verbs (session.rs `PAPER_MARK`): the stick's
/// methods (`p:side`, `p:stroke`, `p:roll`, `p:snap`) on the pencils' shared
/// metatable, and the globals `pastel`, `smudge` and `feel`. Installed by
/// `Session::set_paper` before the first chunk, only when the switch is on,
/// so an older log sees exactly the globals it saw.
pub(crate) fn install_paper(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let meta: Table = lua.named_registry_value(PENCIL_META)?;
    let methods: Table = meta.raw_get("__index")?;
    // p:side(pts, {force=, pressure=, speed=, roll=}): the stick laid nearly
    // flat across the stroke, a piece `length` mm long (p:snap breaks one off)
    {
        let st1 = st.clone();
        methods.set("side", lua.create_function(move |lua, (p, pts, o): (Table, Value, Option<Table>)| {
            if !is_stick(&st1, &p)? {
                return err("side: only a pastel stick is laid on its side (pastel(pile))");
            }
            if let Some(o) = &o {
                check_keys(o, &["force", "pressure", "speed", "roll"], "side")?;
            }
            let pts = points(&pts)?;
            if pts.len() < 2 {
                return err("side: needs at least two points");
            }
            let n = pts.len();
            let force = match o.as_ref().map(|o| o.get::<Value>("force")).transpose()?.unwrap_or(Value::Nil) {
                Value::Nil => profile(o.as_ref(), 0.4)?.iter().map(|p| p * FORCE_AT_FULL).collect::<Vec<_>>(),
                _ => along(o.as_ref(), "force", n, 2.0)?,
            };
            let force = if force.len() == n { force } else { (0..n).map(|i| force[(i * force.len() / n).min(force.len() - 1)]).collect() };
            // across the stroke: the stick's axis at a right angle to its direction
            let az: Vec<f32> = (0..n).map(|i| {
                let (a, b) = (pts[i.saturating_sub(1)], pts[(i + 1).min(n - 1)]);
                (b.1 - a.1).atan2(b.0 - a.0).to_degrees() + 90.0
            }).collect();
            let speed = along(o.as_ref(), "speed", n, 60.0)?;
            let roll = along(o.as_ref(), "roll", n, 0.0)?;
            // (laid flat: the hand presses the piece down along its length)
            stick_stroke(lua, &st1, &p, &pts, &force, &vec![0.0; n], &az, &roll, &speed)
        })?)?;
    }
    // p:stroke(pts, {force=, alt=, azimuth=, roll=, speed=}): one stroke of a
    // pastel stick, each a number or a list along the points: force (N), alt
    // (the stick's angle to the paper, degrees), azimuth (where its upper end
    // points, degrees: 0 to the right, 90 down the canvas), roll (turned about
    // its axis, degrees, added to p:roll's), speed (mm/s)
    {
        let st1 = st.clone();
        methods.set("stroke", lua.create_function(move |lua, (p, pts, o): (Table, Value, Option<Table>)| {
            if !is_stick(&st1, &p)? {
                return err("stroke: a stroke of a pastel stick; a pencil draws with line");
            }
            if let Some(o) = &o {
                check_keys(o, STROKE_KEYS, "stroke")?;
            }
            let pts = points(&pts)?;
            if pts.len() < 2 {
                return err("stroke: needs at least two points");
            }
            let n = pts.len();
            let force = along(o.as_ref(), "force", n, 1.5)?;
            let alt = along(o.as_ref(), "alt", n, 60.0)?;
            let az = along(o.as_ref(), "azimuth", n, 45.0)?;
            let roll = along(o.as_ref(), "roll", n, 0.0)?;
            let speed = along(o.as_ref(), "speed", n, 80.0)?;
            if force.iter().any(|f| !(0.0..=20.0).contains(f)) {
                return err("stroke: force is newtons, 0 to 20 (light 0.1–0.5, normal 0.5–2, heavy 2–5)");
            }
            if speed.iter().any(|v| !(1.0..=2000.0).contains(v)) {
                return err("stroke: speed is mm/s, 1 to 2000 (hatching 60–400)");
            }
            stick_stroke(lua, &st1, &p, &pts, &force, &alt, &az, &roll, &speed)
        })?)?;
    }
    // p:roll(degrees): turn the stick in the fingers (later strokes add theirs)
    {
        let st1 = st.clone();
        methods.set("roll", lua.create_function(move |_, (p, d): (Table, f32)| {
            if !is_stick(&st1, &p)? {
                return err("roll: a pastel stick turns in the fingers");
            }
            if !d.is_finite() {
                return err("roll: degrees, a number");
            }
            let r: f32 = p.raw_get::<Option<f32>>("turned")?.unwrap_or(0.0);
            p.raw_set("turned", (r + d).rem_euclid(360.0))?;
            Ok(())
        })?)?;
    }
    // p:snap(mm): break the stick, keeping a piece that long (its worn end)
    {
        let st1 = st.clone();
        methods.set("snap", lua.create_function(move |_, (p, mm): (Table, f32)| {
            if !is_stick(&st1, &p)? {
                return err("snap: a pastel stick breaks");
            }
            let l: f32 = p.get("length")?;
            if !(mm > 2.0 && mm < l) {
                return err(format!("snap: keep a piece between 2 mm and its length ({l:.0} mm)"));
            }
            p.set("length", mm)?;
            Ok(())
        })?)?;
    }

    // pastel(pile, {kind="soft"|"hard"|"pencil", soft=, diameter=, length=}):
    // a stick of the pile's pigments as they look dry (paint's dry.rs), that
    // rests on the tooth, wears to facets and lays what it abrades (pastel.rs)
    {
        let st1 = st.clone();
        g.set("pastel", lua.create_function(move |lua, (v, o): (Value, Option<Table>)| {
            let pile = crate::api::pile_of(&v, "pastel")?;
            let tubes = st1.borrow().tubes.clone();
            if let Some(o) = &o {
                check_keys(o, STICK_KEYS, "pastel")?;
            }
            let kind: String = o.as_ref().map(|o| o.get::<Option<String>>("kind")).transpose()?.flatten().unwrap_or_else(|| "soft".into());
            let soft = o.as_ref().map(|o| num(o, "soft")).transpose()?.flatten();
            let dia = o.as_ref().map(|o| num(o, "diameter")).transpose()?.flatten();
            // (clamp keeps a NaN: refuse it first)
            if dia.is_some_and(|d| !d.is_finite()) {
                return err("pastel: diameter is the stick's width in mm, a number");
            }
            let color = tubes.dry_color(&pile.mix.parts);
            let stick = match kind.as_str() {
                "soft" => paint::pastel::Stick::round(color, soft.unwrap_or(0.75), dia.unwrap_or(12.0).clamp(4.0, 25.0)),
                "hard" => paint::pastel::Stick::square(color, soft.unwrap_or(0.25), dia.unwrap_or(6.35).clamp(3.0, 15.0)),
                "pencil" => paint::pastel::Stick::pencil(color, soft.unwrap_or(0.35), dia.unwrap_or(4.5).clamp(2.0, 8.0)),
                k => return err(format!("pastel: kind {k:?}: soft (a round stick), hard (a square one) or pencil")),
            };
            if soft.is_some_and(|s| !(0.0..=1.0).contains(&s)) {
                return err("pastel: soft is 0 (hard) to 1 (very soft)");
            }
            let p = lua.create_table()?;
            p.set("kind", "stick")?;
            p.set("section", match (kind.as_str(), stick.section) {
                ("pencil", _) => "pencil",
                (_, paint::pastel::Section::Square { .. }) => "square",
                _ => "round",
            })?;
            p.set("size", match stick.section {
                paint::pastel::Section::Round { d_mm } => d_mm,
                paint::pastel::Section::Square { side_mm } => side_mm,
            })?;
            p.set("hardness", stick.hardness)?;
            p.set("wear", stick.wear)?;
            p.set("r", stick.color[0])?;
            p.set("g", stick.color[1])?;
            p.set("b", stick.color[2])?;
            p.set("name", pile.recipe())?;
            p.set("worn", 0.0)?;
            if let Some(l) = o.as_ref().map(|o| num(o, "length")).transpose()?.flatten() {
                if !(5.0..=200.0).contains(&l) {
                    return err("pastel: length is the stick's length in mm, 5 to 200");
                }
                p.set("length", l)?;
            } else {
                p.set("length", stick.length_mm)?;
            }
            let ft = lua.create_table()?;
            for f in &stick.facets {
                for v in f {
                    ft.push(*v)?;
                }
            }
            p.set("facets", ft)?;
            p.set_metatable(Some(meta.clone()))?;
            Ok(p)
        })?)?;
    }
    // smudge(pts, {force=1, pad=, speed=40}): a finger (or a stump: pad=, its
    // contact in mm²) drawn along a path, moving loose pastel (pastel.rs `rub`)
    {
        let st1 = st.clone();
        g.set("smudge", lua.create_function(move |_, (a, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, SMUDGE_KEYS, "smudge")?;
            }
            let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
            let pts = match &a {
                Value::Table(_) => points(&a)?,
                _ => return err("smudge: a finger is drawn along a path, smudge(pts, {force=, pad=, speed=})"),
            };
            if pts.len() < 2 {
                return err("smudge: needs at least two points");
            }
            let force = get("force")?.unwrap_or(1.0);
            let pad = get("pad")?;
            let speed = get("speed")?.unwrap_or(40.0);
            if !(speed.is_finite() && speed > 0.0) {
                return err("smudge: speed is mm/s, a number above 0");
            }
            if !(0.05..=10.0).contains(&force) || pad.is_some_and(|p| !(0.5..=400.0).contains(&p)) {
                return err("smudge: force is newtons (0.05 to 10); pad, the contact in mm² (a stump: 2–10; a fingertip: about 135 at 1 N)");
            }
            crate::time::verb(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let radius = (pad.unwrap_or(135.0 * force.powf(0.4)) / std::f32::consts::PI).sqrt();
                validate_pastel_path(c, &pts, Some(radius))?;
                let secs = c.rub(&pts, force, pad, speed);
                c.tally_mut().secs += secs as f64 + 0.5;
                Ok(0)
            })
        })?)?;
    }
    // feel(x, y): what a fingertip feels there: the surface, and the pastel
    // in the tooth (a query: the painting ages to now first, as `drying` does)
    {
        let st1 = st.clone();
        g.set("feel", lua.create_function(move |_, (x, y): (f32, f32)| {
            crate::time::verb(&st1, crate::time::Verb::Query, |s| {
                let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                Ok(c.feel(x, y))
            })
        })?)?;
    }
    Ok(())
}

// every test here paints from the default box
#[cfg(all(test, tube_box))]
mod tests {
    use crate::session::{Session, logged_paper, parse_program};

    const W: usize = 160;
    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=40, apply="brush"}}}"#;
    const PAPER: &str = r#"canvas{size=200, aspect=1.25, seed=3, paper={grammage=200, tone={{"lead white", 4}, {"raw umber", 0.3}}}}"#;

    fn run(s: &mut Session, src: &str) -> String {
        s.run(src).unwrap_or_else(|e| panic!("{src}: {e}")).out
    }

    /// Every global's name, sorted, and the pencil's methods.
    fn names(s: &mut Session) -> (Vec<String>, String) {
        let g = run(s, "local k = {} for n in pairs(_G) do k[#k + 1] = tostring(n) end table.sort(k) print(table.concat(k, ' '))");
        let m = run(s, "local p = pencil() print(type(p.side), type(p.stroke), type(p.roll), type(p.snap), type(p.line))");
        (g.split_whitespace().map(str::to_string).collect(), m.trim().to_string())
    }

    /// The paper-and-pastel switch brings `pastel`, `smudge` and `feel` and
    /// the stick's methods, and nothing else: a session without it (an older
    /// log) has none of them, so its walk of the globals is as it was.
    #[test]
    fn pastel_comes_with_the_switch_and_only_with_it() {
        let mut off = Session::new(W).unwrap();
        run(&mut off, CANVAS);
        let (g_off, m_off) = names(&mut off);
        assert_eq!(run(&mut off, "print(type(pastel), type(smudge), type(feel))").trim(), "nil\tnil\tnil");
        assert_eq!(m_off, "nil\tnil\tnil\tnil\tfunction");
        let e = off.run(r#"local p = pencil() p.kind = "stick" p:line({{100, 100}, {300, 120}})"#).unwrap_err();
        assert!(e.contains("pencil kind \"stick\""), "{e}");
        let mut on = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut on, CANVAS);
        let (g_on, m_on) = names(&mut on);
        let added: Vec<&String> = g_on.iter().filter(|n| !g_off.contains(n)).collect();
        assert_eq!(added, ["feel", "pastel", "smudge"]);
        assert!(g_off.iter().all(|n| g_on.contains(n)));
        assert_eq!(m_on, "function\tfunction\tfunction\tfunction\tfunction");
        // the easel's own: not the painting's
        assert!(!on.globals().contains("pastel"));
        assert!(Session::new(W).unwrap().with_paper(true).unwrap().with_paper(false).is_err());
    }

    #[test]
    fn graded_hatching_requires_the_paper_switch() {
        for paper in [false, true] {
            let mut s = Session::new(W).unwrap().with_paper(paper).unwrap();
            run(&mut s, CANVAS);
            let result = s.run("pencil():hatch(rect(100, 100, 100, 100), {graded=true})");
            if paper {
                result.unwrap();
            } else {
                assert!(result.unwrap_err().contains("hatch: unknown option \"graded\""));
            }
        }
    }

    #[test]
    fn pastel_paths_are_bounded_before_native_resampling() {
        let mut s = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut s, PAPER);
        run(&mut s, r#"p = pastel(pile{{"vermilion", 1}})
            p:stroke({{100, 100}, {300, 100}})"#);
        for call in ["p:stroke(path)", "p:side(path)", "smudge(path, {pad=0.5})"] {
            for (path, want) in [
                ("{{100, 100}, {math.huge, 100}}", "finite numbers"),
                ("{{100, 100}, {-math.huge, 100}}", "finite numbers"),
                ("{{100, 100}, {0/0, 100}}", "finite numbers"),
                ("{100, 100, 1e100, 100}", "finite numbers"),
                ("{{-3e38, 100}, {3e38, 100}}", "too much resampling"),
                ("{{100, 100}, {1e9, 100}}", "too much resampling"),
                ("(function() local t = {} for i=1,4097 do t[i]={100,100} end return t end)()", "at most 4096 points"),
                ("(function() local t = {} for i=1,4096 do t[i]={i%2*10000,100} end return t end)()", "too much resampling"),
            ] {
                let src = format!("local path = {path}; {call}");
                let e = s.run(&src).unwrap_err();
                assert!(e.contains(want), "{src}: {e}");
            }
        }
        let e = s.run("local path = {} for i=1,4096 do path[i]={i*20,100} end p:stroke(path)").unwrap_err();
        assert!(e.contains("too much resampling"), "{e}");
        run(&mut s, "p:stroke({{100,100}, {100,100}}); smudge({{100,100}, {100,100}})");
    }

    /// A stick on paper: it lays, wears (its facets in the Lua heap), the
    /// finger moves it, fixative binds it and `feel` says so; a failed chunk
    /// puts the stick back as it was, and the log replays bit for bit.
    #[test]
    fn a_stick_draws_wears_and_replays() {
        let mut s = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut s, PAPER);
        assert!(run(&mut s, "print(feel(500, 400))").starts_with("paper: the tooth open"));
        run(&mut s, r#"p = pastel(pile{{"vermilion", 1}, {"lead white", 2}}, {kind="soft"})
            print(p)
            p:stroke({{200, 300}, {800, 320}}, {force=2.5, alt=55})
            p:roll(40)
            p:side({{200, 500}, {800, 500}}, {force=1.5})
            p:snap(30)
            p:line({{200, 650}, {800, 600}}, {pressure=0.6})
            p:hatch(rect(300, 200, 200, 150), {graded=true})"#);
        let shown = run(&mut s, "print(p)");
        assert!(shown.starts_with("pastel round ") && shown.contains("30.0 mm long"), "{shown}");
        let before = run(&mut s, "print(#p.facets)");
        assert!(s.run("p:stroke({{200, 400}, {800, 400}}, {force=3}) error('no')").is_err());
        assert_eq!(run(&mut s, "print(#p.facets)"), before, "a failed chunk wears no stick");
        // (what a fingertip finds along the first stroke: pastel somewhere in it)
        let along = |what: &str| format!("local found = '' for x = 300, 700, 20 do for y = 302, 318, 4 do local f = feel(x, y) if f:find('{what}', 1, true) then found = f end end end print(found)");
        let felt = run(&mut s, &along("pastel, loose"));
        assert!(felt.starts_with("paper: pastel, loose, "), "{felt}");
        run(&mut s, "smudge({{150, 310}, {900, 310}}, {force=1.5})");
        run(&mut s, "fix(rect(0, 0, 1000, 450))");
        let felt = run(&mut s, &along("pastel, fixed"));
        assert!(felt.starts_with("paper: pastel, fixed, "), "{felt}");
        // refused, as numbers
        for (src, want) in [
            ("p:stroke({{100, 100}, {300, 100}}, {force=0/0})", "force"),
            ("p:stroke({{100, 100}, {300, 100}}, {alt=0/0})", "are numbers"),
            ("p:roll(0/0)", "degrees"),
            ("p:snap(500)", "snap"),
            ("smudge({{100, 100}, {300, 100}}, {speed=0/0})", "speed"),
            ("smudge(rect(0, 0, 10, 10))", "path"),
            ("pastel(pile{{\"lead white\", 1}}, {diameter=0/0})", "diameter"),
            ("pastel(pile{{\"lead white\", 1}}, {kind=\"crayon\"})", "kind"),
            ("pencil():stroke({{100, 100}, {300, 100}})", "pastel stick"),
        ] {
            let e = s.run(src).unwrap_err();
            assert!(e.contains(want), "{src}: {e}");
        }
        let log = s.program("stick");
        assert!(logged_paper(&log));
        let mut r = Session::replay(W).unwrap().with_paper(true).unwrap();
        for chunk in parse_program(&log) {
            r.run(&chunk).unwrap();
        }
        let state = |s: &Session| {
            let mut b = Vec::new();
            s.canvas().unwrap().write_state(&mut b, "").unwrap();
            b
        };
        assert!(state(&r) == state(&s), "the log replays bit for bit");
        assert_eq!(run(&mut r, "print(p)"), run(&mut s, "print(p)"));
    }
}
