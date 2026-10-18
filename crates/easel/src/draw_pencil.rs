//! Drawing in Lua: graphite pencils and black chalk on the ground, the
//! kneaded eraser and fixative (engine: `paint::graphite`); with the
//! paper-and-pastel switch, pastel sticks, the finger, `feel`, a paper mask,
//! a puff and a tap too (`install_paper`; engine: `paint::pastel`,
//! `paint::sheet`).
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
const SMUDGE_KEYS: &[&str] = &["force", "pad", "speed", "angle", "passes", "pattern"];
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
pub(crate) fn stick_of(p: &Table, charcoal: bool) -> Result<paint::pastel::Stick> {
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
    let material = if charcoal {
        paint::pastel::Material {
            particle_um: p.get::<Option<f32>>("particle_um")?.unwrap_or(paint::pastel::PARTICLE_UM),
            shed: p.get::<Option<f32>>("shed")?.unwrap_or(paint::pastel::SHED),
            lift: p.get::<Option<f32>>("lift")?.unwrap_or(0.55),
            binder: p.get::<Option<f32>>("binder")?.unwrap_or(0.0),
        }
    } else { paint::pastel::Material::default() };
    if !(0.1..=100.0).contains(&material.particle_um) || !(0.0..=1.0).contains(&material.shed) || !(0.0..=1.0).contains(&material.lift) || !(0.0..=1.0).contains(&material.binder) {
        return err("a stick's particle_um is 0.1 to 100 µm; its shed, lift and binder are 0 to 1");
    }
    Ok(Stick { section, length_mm: p.get("length")?, facets, hardness: p.get("hardness")?, wear: p.get("wear")?, color: [p.get("r")?, p.get("g")?, p.get("b")?], material })
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
    let charcoal = st.borrow().tubes.engine == paint::CHARCOAL;
    let mut stick = stick_of(p, charcoal)?;
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
    let laid = crate::time::verb_dry(st, crate::time::Verb::Pass, |s| {
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
            let charcoal = st1.borrow().tubes.engine == paint::CHARCOAL;
            let s = stick_of(&p, charcoal)?;
            let name: String = p.get::<Option<String>>("name")?.unwrap_or_default();
            let sec: String = p.get("section")?;
            if charcoal && p.contains_key("particle_um")? {
                format!("{name}, {} ({} MPa, {:.1} mm long, {} facets)", if sec == "pencil" { "sharpened" } else { &sec }, (s.hardness * 10.0).round() / 10.0, s.length_mm, s.facets.len())
            } else {
                format!("pastel {sec} {name} ({} MPa, {:.1} mm long, {} facets)", (s.hardness * 10.0).round() / 10.0, s.length_mm, s.facets.len())
            }
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
/// metatable, and the globals `pastel`, `smudge`, `feel`, `lay_sheet`,
/// `lift_sheet`, `blow` and `tap`. Installed by
/// `Session::set_paper` before the first chunk, only when the switch is on,
/// so an older log sees exactly the globals it saw.
pub(crate) fn install_paper(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let meta: Table = lua.named_registry_value(PENCIL_META)?;
    let methods: Table = meta.raw_get("__index")?;
    if st.borrow().tubes.engine == paint::CHARCOAL {
        let meta = meta.clone();
        g.set("charcoal", lua.create_function(move |lua, o: Option<Table>| {
            if let Some(o) = &o {
                check_keys(o, &["kind", "grade", "size"], "charcoal")?;
            }
            let text = |key: &str, default: &str| -> Result<String> { Ok(o.as_ref().map(|o| o.get::<Option<String>>(key)).transpose()?.flatten().unwrap_or_else(|| default.into())) };
            let kind = text("kind", "vine")?;
            let grade = text("grade", "medium")?;
            let size = text("size", "thin")?;
            if !matches!(size.as_str(), "thin" | "thick") {
                return err("charcoal: size is \"thin\" or \"thick\"");
            }
            if kind != "pencil" && o.as_ref().is_some_and(|o| o.contains_key("grade").unwrap_or(false)) {
                return err("charcoal: grade is for kind=\"pencil\"");
            }
            if !matches!(kind.as_str(), "vine" | "willow") && o.as_ref().is_some_and(|o| o.contains_key("size").unwrap_or(false)) {
                return err("charcoal: size is for kind=\"vine\" or \"willow\"");
            }
            let stick = paint::pastel::Stick::charcoal(&kind, &grade, size == "thick").ok_or_else(|| mlua::Error::runtime("charcoal: kind is \"vine\", \"willow\", \"compressed\" or \"pencil\"; a pencil's grade is \"soft\", \"medium\" or \"hard\""))?;
            let p = lua.create_table()?;
            p.set("kind", "stick")?;
            p.set("section", if kind == "pencil" { "pencil" } else { "round" })?;
            let paint::pastel::Section::Round { d_mm } = stick.section else { unreachable!() };
            p.set("size", d_mm)?;
            p.set("hardness", stick.hardness)?;
            p.set("wear", stick.wear)?;
            p.set("particle_um", stick.material.particle_um)?;
            p.set("shed", stick.material.shed)?;
            p.set("lift", stick.material.lift)?;
            p.set("binder", stick.material.binder)?;
            p.set("r", stick.color[0])?;
            p.set("g", stick.color[1])?;
            p.set("b", stick.color[2])?;
            p.set("name", match kind.as_str() { "pencil" => format!("{grade} charcoal pencil"), "compressed" => "compressed charcoal".into(), _ => format!("{size} {kind} charcoal") })?;
            p.set("worn", 0.0)?;
            store_stick(lua, &p, &stick)?;
            p.set_metatable(Some(meta.clone()))?;
            Ok(p)
        })?)?;
        let st = st.clone();
        g.set("charcoal_powder", lua.create_function(move |lua, (v, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["depth", "seed"], "charcoal_powder")?;
            }
            let mask = mask_of(&v)?;
            let depth = o.as_ref().map(|o| num(o, "depth")).transpose()?.flatten().unwrap_or(0.6);
            if !(0.0..=5.0).contains(&depth) {
                return err("charcoal_powder: depth is the dust's average thickness, 0 to 5 µm");
            }
            let seed = seed_of(&st, &o.unwrap_or(lua.create_table()?))?;
            crate::time::verb_dry(&st, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                Ok(c.charcoal_powder(&mask, depth, seed))
            })
        })?)?;
    }
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
    // contact in mm²) drawn along a path, moving loose pastel (pastel.rs
    // `rub`); smudge(mask, {force=, pad=, speed=, angle=30, passes=2,
    // pattern="circles"|"strokes"}): a fingertip (or a stump) worked over an
    // area, pressing force × the mask (pastel.rs `rub_area`; its defaults by
    // the tool, from notes/research/pastel_blending.md)
    {
        let st1 = st.clone();
        g.set("smudge", lua.create_function(move |_, (a, o): (Value, Option<Table>)| {
            use paint::pastel::{AreaRub, Pattern, SMUDGE_MAX_PASSES, SMUDGE_MIN_FORCE, SMUDGE_PATH_FORCE, SMUDGE_PATH_SPEED};
            if let Some(o) = &o {
                check_keys(o, SMUDGE_KEYS, "smudge")?;
            }
            let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
            let (force, pad, speed) = (get("force")?, get("pad")?, get("speed")?);
            if speed.is_some_and(|v| !(v.is_finite() && v > 0.0)) {
                return err("smudge: speed is mm/s, a number above 0");
            }
            if force.is_some_and(|f| !(SMUDGE_MIN_FORCE..=10.0).contains(&f)) || pad.is_some_and(|p| !(0.5..=400.0).contains(&p)) {
                return err("smudge: force is newtons (0.05 to 10); pad, the contact in mm² (a stump: 2–10; a fingertip: about 135 at 1 N)");
            }
            let (angle, passes) = (get("angle")?, get("passes")?);
            let pattern: Option<String> = o.as_ref().map(|o| o.get::<Option<String>>("pattern")).transpose()?.flatten();
            match &a {
                Value::Table(_) => {
                    if angle.is_some() || passes.is_some() || pattern.is_some() {
                        return err("smudge: angle=, passes= and pattern= are for an area (a mask); along a path the finger follows the points");
                    }
                    let pts = points(&a)?;
                    if pts.len() < 2 {
                        return err("smudge: needs at least two points");
                    }
                    let (force, speed) = (force.unwrap_or(SMUDGE_PATH_FORCE), speed.unwrap_or(SMUDGE_PATH_SPEED));
                    crate::time::verb_dry(&st1, crate::time::Verb::Pass, |s| {
                        let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                        let radius = (paint::pastel::pad_area(force, pad) / std::f32::consts::PI).sqrt();
                        validate_pastel_path(c, &pts, Some(radius))?;
                        let secs = c.rub(&pts, force, pad, speed);
                        c.tally_mut().secs += secs as f64 + 0.5;
                        Ok(0)
                    })
                }
                Value::UserData(_) => {
                    let m = mask_of(&a)?;
                    if angle.is_some_and(|v| !(v.is_finite() && (-360.0..=360.0).contains(&v))) {
                        return err("smudge: angle is the lanes' direction in degrees, -360 to 360");
                    }
                    let passes = match passes {
                        None => None,
                        Some(p) if p.fract() == 0.0 && (1.0..=SMUDGE_MAX_PASSES as f32).contains(&p) => Some(p as u32),
                        Some(_) => return err(format!("smudge: passes is a whole number, 1 to {SMUDGE_MAX_PASSES}")),
                    };
                    let pattern = match pattern.as_deref() {
                        None => None,
                        Some("circles") => Some(Pattern::Circles),
                        Some("strokes") => Some(Pattern::Strokes),
                        Some(p) => return err(format!("smudge: pattern {p:?}: \"circles\" (loose circles, a fingertip's way) or \"strokes\" (straight, back and forth: a stump's)")),
                    };
                    let o = AreaRub { force, pad_mm2: pad, speed, angle_deg: angle, passes, pattern };
                    crate::time::verb_dry(&st1, crate::time::Verb::Pass, |s| {
                        let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                        let out = c.rub_area(&m, o).map_err(mlua::Error::runtime)?;
                        c.tally_mut().secs += out.secs as f64 + 0.5;
                        Ok(0)
                    })
                }
                _ => err("smudge: a finger is drawn along a path or worked over an area, smudge(pts, {force=, pad=, speed=}) or smudge(mask, {force=, pad=, speed=, angle=, passes=, pattern=})"),
            }
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
    // lay_sheet(mask, {grammage=120, tone=}): a sheet of paper laid over the
    // mask to keep it clean (paint's sheet.rs); lift_sheet() takes it away
    // with what it caught. It must be lifted in the chunk that laid it.
    {
        let st1 = st.clone();
        g.set("lay_sheet", lua.create_function(move |_, (a, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["grammage", "tone"], "lay_sheet")?;
            }
            let m = mask_of(&a)?;
            let grammage = o.as_ref().map(|o| num(o, "grammage")).transpose()?.flatten().unwrap_or(120.0);
            if !(40.0..=400.0).contains(&grammage) {
                return err("lay_sheet: grammage is g/m² (40 to 400; a mask is usually 80–160)");
            }
            let mut s = st1.borrow_mut();
            let tone = match o.as_ref().map(|o| o.get::<Value>("tone")).transpose()?.unwrap_or(Value::Nil) {
                Value::Nil => paint::hex("#ece6d8"),
                Value::Table(t) => {
                    let parts = crate::api::parts_of(&s.tubes, &t, "lay_sheet tone")?.0;
                    s.tubes.dry_color(&parts)
                }
                v => return err(format!("lay_sheet: tone is parts of tubes, {{{{\"lead white\", 3}}, ...}}, not a {}", v.type_name())),
            };
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            // the sheet's thickness and pores: a drawing paper of that grammage (paper.rs)
            let paper = paint::paper::Paper { grammage, ..paint::paper::Paper::drawing(0) };
            let mu = c.mean_micro_um().unwrap_or(8.0);
            c.lay_sheet(&m, paper.caliper_um(), mu, tone).map_err(mlua::Error::runtime)?;
            c.tally_mut().secs += 4.0;
            Ok(())
        })?)?;
        let st1 = st.clone();
        g.set("lift_sheet", lua.create_function(move |_, ()| {
            let mut s = st1.borrow_mut();
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            if !c.lift_sheet() {
                return err("lift_sheet: no sheet lies on the picture");
            }
            c.tally_mut().secs += 2.0;
            Ok(())
        })?)?;
    }
    // blow(x, y, {distance=50, speed=12, nozzle=8}): a puff of air at the
    // picture from `distance` mm, leaving the lips (or a bulb's nozzle,
    // `nozzle` mm) at `speed` m/s (blowing hard: 12 on average, 6–64); tap({g=100}):
    // the board's edge struck on the table (50–400 g; an upright sheet: 1).
    // Paint's sheet.rs. Each returns the volume shed, mm³.
    {
        let st1 = st.clone();
        g.set("blow", lua.create_function(move |_, (x, y, o): (f32, f32, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["distance", "speed", "nozzle"], "blow")?;
            }
            let get = |k: &str, d: f32| -> Result<f32> { Ok(o.as_ref().map(|o| num(o, k)).transpose()?.flatten().unwrap_or(d)) };
            let (h, u, d) = (get("distance", 50.0)?, get("speed", 12.0)?, get("nozzle", 8.0)?);
            if !(5.0..=500.0).contains(&h) || !(1.0..=80.0).contains(&u) || !(1.0..=40.0).contains(&d) {
                return err("blow: distance 5–500 mm, speed 1–80 m/s (a hard blow 12, peaks to 64; a bulb 20–60), nozzle 1–40 mm (lips about 8)");
            }
            crate::time::verb_dry(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let v = c.blow_pastel(x, y, h, u, d);
                c.tally_mut().secs += 1.5;
                Ok(v)
            })
        })?)?;
        let st1 = st.clone();
        g.set("tap", lua.create_function(move |_, o: Option<Table>| {
            if let Some(o) = &o {
                check_keys(o, &["g"], "tap")?;
            }
            let a = o.as_ref().map(|o| num(o, "g")).transpose()?.flatten().unwrap_or(100.0);
            if !(0.1..=1000.0).contains(&a) {
                return err("tap: g, the board's acceleration in gravities (a tapped edge 50–400; an upright sheet 1)");
            }
            crate::time::verb_dry(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let v = c.tap_pastel(a);
                c.tally_mut().secs += 3.0;
                Ok(v)
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

    #[test]
    fn charcoal_materials_are_opt_in_and_replay_with_their_wear() {
        for engine in [5, 7, 8, 9] {
            for paper in [false, true] {
                if engine == paint::CHARCOAL && paper { continue; }
                let mut tubes = paint::Palette::tube_box();
                tubes.engine = engine;
                let mut old = Session::with_box(W, tubes).unwrap().with_paper(paper).unwrap();
                assert_eq!(run(&mut old, "print(charcoal, charcoal_powder)").trim(), "nil\tnil");
                if paper {
                    let error = old.run(r#"canvas{size=80,aspect=1,paper={preset="bristol"}}"#).unwrap_err();
                    assert!(error.contains("paper: unknown option \"preset\""), "{error}");
                }
            }
        }
        for preset in ["newsprint", "toned laid", "bristol"] {
            let mut tubes = paint::Palette::tube_box();
            tubes.engine = paint::CHARCOAL;
            tubes.knife_skim = true;
            tubes.blend_feather = true;
            tubes.brush_skim = true;
            tubes.glaze_film = true;
            tubes.rag_lift = true;
            let mut s = Session::with_box(W, tubes.clone()).unwrap().with_paper(true).unwrap();
            run(&mut s, &format!(r#"canvas{{size=80, aspect=1, paper={{preset="{preset}"}}, seed=13}}"#));
            run(&mut s, r#"
                sticks = {charcoal(), charcoal{kind="vine",size="thick"},
                    charcoal{kind="willow"}, charcoal{kind="willow",size="thick"},
                    charcoal{kind="compressed"}, charcoal{kind="pencil",grade="soft"},
                    charcoal{kind="pencil",grade="medium"}, charcoal{kind="pencil",grade="hard"}}
                for i,p in ipairs(sticks) do
                    p:stroke({{100,i*90},{800,i*90}},{force=1.2,alt=60})
                end
                charcoal_powder(rect(100,800,200,100),{depth=0.3,seed=7})
                smudge({{150,850},{500,850}},{pad=4})
                erase(rect(400,0,200,1000))
                w=pastel(pile{{"lead white",1}})
                w:stroke({{650,800},{850,900}},{force=2})
            "#);
            let before = run(&mut s, "for _,p in ipairs(sticks) do print(p.length,table.concat(p.facets,',')) end");
            let log = s.program("charcoal");
            let mut replay = Session::replay_with(W, tubes).unwrap().with_paper(true).unwrap();
            for chunk in parse_program(&log) { run(&mut replay, &chunk); }
            assert_eq!(s.st.borrow().canvas.as_ref().unwrap().pixels(), replay.st.borrow().canvas.as_ref().unwrap().pixels());
            let mut bytes = Vec::new();
            s.canvas().unwrap().write_state(&mut bytes, "charcoal").unwrap();
            let (restored, _) = paint::Canvas::read_state(&mut std::io::Cursor::new(&bytes)).unwrap();
            assert_eq!(restored.engine(), paint::CHARCOAL);
            let mut again = Vec::new();
            restored.write_state(&mut again, "charcoal").unwrap();
            assert_eq!(bytes, again);
            assert_eq!(run(&mut replay, "for _,p in ipairs(sticks) do print(p.length,table.concat(p.facets,',')) end"), before);
        }
    }

    /// Every global's name, sorted, and the pencil's methods.
    fn names(s: &mut Session) -> (Vec<String>, String) {
        let g = run(s, "local k = {} for n in pairs(_G) do k[#k + 1] = tostring(n) end table.sort(k) print(table.concat(k, ' '))");
        let m = run(s, "local p = pencil() print(type(p.side), type(p.stroke), type(p.roll), type(p.snap), type(p.line))");
        (g.split_whitespace().map(str::to_string).collect(), m.trim().to_string())
    }

    /// The paper-and-pastel switch brings `pastel`, `smudge`, `feel`,
    /// `lay_sheet`, `lift_sheet`, `blow` and `tap` and the stick's methods,
    /// and nothing else: a session without it (an older log) has none of
    /// them, so its walk of the globals is as it was.
    #[test]
    fn pastel_comes_with_the_switch_and_only_with_it() {
        let mut off = Session::new(W).unwrap();
        run(&mut off, CANVAS);
        let (g_off, m_off) = names(&mut off);
        assert_eq!(run(&mut off, "print(type(pastel), type(smudge), type(feel), type(blow), type(tap))").trim(), "nil\tnil\tnil\tnil\tnil");
        assert_eq!(m_off, "nil\tnil\tnil\tnil\tfunction");
        let e = off.run(r#"local p = pencil() p.kind = "stick" p:line({{100, 100}, {300, 120}})"#).unwrap_err();
        assert!(e.contains("pencil kind \"stick\""), "{e}");
        let mut on = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut on, CANVAS);
        let (g_on, m_on) = names(&mut on);
        let added: Vec<&String> = g_on.iter().filter(|n| !g_off.contains(n)).collect();
        assert_eq!(added, ["blow", "feel", "lay_sheet", "lift_sheet", "pastel", "smudge", "tap"]);
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
            ("smudge(5)", "a path or worked over an area"),
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

    /// An area smudge comes with the switch: `smudge(mask, {angle=,
    /// passes=})` works the finger over the mask (the pastel inside it
    /// moves, the hand's clock runs on), refuses `angle=` and `passes=`
    /// along a path and bad numbers plainly, and the log replays bit for bit.
    #[test]
    fn smudge_works_an_area_and_replays() {
        let mut s = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut s, PAPER);
        run(&mut s, r#"p = pastel(pile{{"vermilion", 1}}, {kind="soft"})
            q = pastel(pile{{"cobalt blue", 1}}, {kind="soft"})
            for y = 300, 390, 15 do p:stroke({{150, y}, {850, y}}, {force=2.5, alt=70}) end
            for y = 405, 495, 15 do q:stroke({{150, y}, {850, y}}, {force=2.5, alt=70}) end"#);
        let px = |s: &Session| s.canvas().unwrap().pixels().to_vec();
        let secs = |s: &Session| s.canvas().unwrap().tally().secs;
        let (before, t0) = (px(&s), secs(&s));
        run(&mut s, "smudge(rect(300, 250, 400, 300), {angle=20, passes=3})");
        assert!(px(&s) != before, "the finger moved pastel");
        assert!(secs(&s) > t0 + 1.0, "the hand took its time: {} s", secs(&s) - t0);
        run(&mut s, "smudge(rect(300, 250, 400, 300), {pad=6, force=0.8})");
        run(&mut s, r#"smudge(rect(300, 250, 400, 300), {pattern="strokes", passes=1})"#);
        for (src, want) in [
            ("smudge({{100, 100}, {300, 100}}, {angle=30})", "for an area"),
            ("smudge({{100, 100}, {300, 100}}, {passes=2})", "for an area"),
            ("smudge(rect(300, 250, 400, 300), {angle=0/0})", "angle"),
            ("smudge(rect(300, 250, 400, 300), {passes=1.5})", "passes"),
            ("smudge(rect(300, 250, 400, 300), {passes=0})", "passes"),
            ("smudge(rect(300, 250, 400, 300), {force=20})", "force"),
            ("smudge(rect(300, 250, 400, 300), {spin=1})", "unknown option"),
            (r#"smudge(rect(300, 250, 400, 300), {pattern="zigzag"})"#, "pattern"),
            (r#"smudge({{100, 100}, {300, 100}}, {pattern="circles"})"#, "for an area"),
        ] {
            let e = s.run(src).unwrap_err();
            assert!(e.contains(want), "{src}: {e}");
        }
        let log = s.program("area smudge");
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
        assert_eq!(secs(&r), secs(&s));
    }

    /// A paper mask: laid and lifted within one chunk (a chunk that ends
    /// with it down fails); while it lies only the stick, the finger, the
    /// eraser and fixative work (a pencil line is refused); what it covered
    /// is clean once it is lifted, and the paper beside it took pastel.
    #[test]
    fn a_paper_mask_lies_for_the_dry_tools_within_a_chunk() {
        let mut s = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut s, PAPER);
        run(&mut s, r#"p = pastel(pile{{"vermilion", 1}}, {kind="soft"})"#);
        let e = s.run("lay_sheet(rect(0, 0, 500, 800))").unwrap_err();
        assert!(e.contains("the chunk ended with a sheet laid on the picture"), "{e}");
        let e = s.run("lay_sheet(rect(0, 0, 500, 800)) pencil():line({{100, 100}, {300, 120}})").unwrap_err();
        assert!(e.contains("a sheet of paper lies on the picture"), "{e}");
        let e = s.run("lay_sheet(rect(0, 0, 500, 800)) lay_sheet(rect(0, 0, 500, 800))").unwrap_err();
        assert!(e.contains("already lies"), "{e}");
        assert!(s.run("lay_sheet(rect(0, 0, 500, 800), {grammage=10}) lift_sheet()").unwrap_err().contains("grammage"));
        run(&mut s, r#"lay_sheet(rect(0, 0, 500, 800), {tone={{"lead white", 3}}})
            for y = 380, 420, 10 do p:stroke({{100, y}, {900, y}}, {force=2.5, alt=55}) end
            smudge({{100, 400}, {900, 400}})
            erase(rect(0, 0, 200, 800))
            fix()
            lift_sheet()"#);
        assert!(run(&mut s, "print(feel(250, 400))").starts_with("paper: the tooth open"));
        let found = run(&mut s, "local found = '' for x = 600, 880, 20 do local f = feel(x, 400) if f:find('pastel', 1, true) then found = f end end print(found)");
        assert!(found.starts_with("paper: pastel, fixed"), "{found}");
        assert!(s.run("lift_sheet()").unwrap_err().contains("no sheet lies"));
    }

    /// A dry brush and a knife's edge on pastel come with the switch: without
    /// it a brush has exactly the methods it had (no `dust`) and a scrape
    /// takes no `edge=`. With it
    /// `b:dust` lifts loose pastel into the bristles (in the brush's text only
    /// once it holds some; `b:wipe(1)` empties it), works while a sheet lies
    /// (a knife doesn't), refuses a loaded brush; a scalpel's edge cuts into
    /// fixed pastel where a palette knife's leaves it; the log replays bit for bit.
    #[test]
    fn a_dry_brush_and_a_knife_edge_on_pastel_come_with_the_switch() {
        let mut off = Session::new(W).unwrap();
        run(&mut off, CANVAS);
        assert_eq!(run(&mut off, r#"local b = brush("filbert", 30) print(type(b.dust), type(b.stroke))"#).trim(), "nil\tfunction");
        let e = off.run("knife{width=60}:scrape({{100, 100}, {300, 100}}, {edge=1})").unwrap_err();
        assert!(e.contains(r#"unknown option "edge" (options: pressure, angle, lift)"#), "{e}");

        let mut s = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut s, PAPER);
        run(&mut s, r#"p = pastel(pile{{"vermilion", 1}}, {kind="soft"})
            for y = 300, 500, 6 do p:stroke({{150, y}, {850, y + 10}}, {force=3, alt=50}) end
            b = brush("filbert", 40)"#);
        assert_eq!(run(&mut s, "print(type(b.dust))").trim(), "function");
        let text = |s: &Session| format!("{:?}", s.st.borrow_mut().live_brushes()[0].borrow());
        assert!(!text(&s).contains("pastel_mg"), "a clean brush's text is as it was");
        let lifted: f32 = run(&mut s, "print(b:dust({{120, 400}, {900, 400}}))").trim().parse().unwrap();
        assert!(lifted > 0.0, "the dry brush lifted pastel: {lifted} mm³");
        assert!(text(&s).contains("pastel_mg"), "and holds some");
        run(&mut s, "b:wipe(1)");
        assert!(!text(&s).contains("pastel_mg"), "wiped clean");
        run(&mut s, "lay_sheet(rect(0, 0, 500, 800)) b:dust({{550, 350}, {850, 350}}) lift_sheet()");
        let e = s.run("lay_sheet(rect(0, 0, 500, 800)) knife{width=60}:scrape({{600, 300}, {800, 300}}) lift_sheet()").unwrap_err();
        assert!(e.contains("a dry brush (b:dust)"), "{e}");
        let e = s.run(r#"local w = brush("round", 10) w:load(pile{{"lead white", 1}}) w:dust({{100, 100}, {300, 100}})"#).unwrap_err();
        assert!(e.contains("has paint on it"), "{e}");
        assert!(s.run("b:dust({{100, 100}, {300, 100}}, {tip=0.5})").unwrap_err().contains("tip"));
        assert!(s.run("knife{width=60}:scrape({{100, 100}, {300, 100}}, {edge=0})").unwrap_err().contains("edge"));
        assert!(s.run("knife{width=60}:lay({{100, 100}, {300, 100}}, {edge=1})").unwrap_err().contains("unknown option"));
        run(&mut s, "fix(rect(0, 0, 1000, 800))");
        let state = |s: &Session| {
            let mut b = Vec::new();
            s.canvas().unwrap().write_state(&mut b, "").unwrap();
            b
        };
        let px = |s: &Session| s.canvas().unwrap().pixels().to_vec();
        let fixed = px(&s);
        run(&mut s, "knife{width=200}:scrape({{500, 280}, {500, 520}}, {pressure=0.5, angle=0})");
        assert!(px(&s) == fixed, "a palette knife at a light hand leaves fixed pastel as it was");
        run(&mut s, "knife{width=10}:scrape({{500, 280}, {500, 520}}, {pressure=0.5, angle=0, edge=1})");
        assert!(px(&s) != fixed, "a scalpel cuts into it");

        let log = s.program("dry tools");
        let mut r = Session::replay(W).unwrap().with_paper(true).unwrap();
        for chunk in parse_program(&log) {
            r.run(&chunk).unwrap();
        }
        assert!(state(&r) == state(&s), "the log replays bit for bit");
        assert_eq!(text(&r), text(&s));
    }

    /// A tap and a puff shed loose pastel (each says how much left the
    /// picture), work while a sheet lies, refuse what no knock or breath
    /// gives, and the log replays bit for bit.
    #[test]
    fn a_tap_and_a_puff_shed_pastel_and_replay() {
        let mut s = Session::new(W).unwrap().with_paper(true).unwrap();
        run(&mut s, PAPER);
        run(&mut s, r#"p = pastel(pile{{"vermilion", 1}}, {kind="soft"})
            for y = 300, 500, 6 do p:stroke({{150, y}, {850, y + 10}}, {force=3, alt=50}) end"#);
        let shed: f32 = run(&mut s, "print(tap({g=1}))").trim().parse().unwrap();
        assert!(shed > 0.0, "an upright sheet drops its biggest crumbs: {shed} mm³");
        let blown: f32 = run(&mut s, "print(blow(500, 400, {distance=50, speed=21}))").trim().parse().unwrap();
        assert!(blown > 0.0, "a hard puff blows pastel off: {blown} mm³");
        run(&mut s, "lay_sheet(rect(0, 0, 500, 800)) blow(700, 400) tap() lift_sheet()");
        assert!(s.run("blow(500, 400, {distance=1})").unwrap_err().contains("distance"));
        assert!(s.run("blow(500, 400, {nozzel=8})").unwrap_err().contains("unknown option"));
        assert!(s.run("tap({g=0})").unwrap_err().contains("gravities"));
        let state = |s: &Session| {
            let mut b = Vec::new();
            s.canvas().unwrap().write_state(&mut b, "").unwrap();
            b
        };
        let log = s.program("blow and tap");
        let mut r = Session::replay(W).unwrap().with_paper(true).unwrap();
        for chunk in parse_program(&log) {
            r.run(&chunk).unwrap();
        }
        assert!(state(&r) == state(&s), "the log replays bit for bit");
    }
}
