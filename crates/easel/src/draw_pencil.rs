//! Drawing in Lua: graphite pencils, black chalk and colored pastels on the
//! ground, the kneaded eraser, the stump and fixative (engine:
//! `paint::graphite`).
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
const HATCH_KEYS: &[&str] = &["angle", "spacing", "length", "pressure", "graded", "seed"];
const ERASE_KEYS: &[&str] = &["strength", "width"];
const PASTEL_KEYS: &[&str] = &["soft", "point"];
const SIDE_KEYS: &[&str] = &["pressure", "width", "smooth", "tremor", "seed"];
const SMUDGE_KEYS: &[&str] = &["strength", "width", "reach", "force", "pad", "speed"];

/// The lead a pencil table stands for.
fn lead_of(p: &Table) -> Result<Lead> {
    let kind: String = p.get::<Option<String>>("kind")?.unwrap_or_else(|| "graphite".into());
    match kind.as_str() {
        "chalk" => Ok(Lead::chalk()),
        "pastel" => {
            let c: [f32; 3] = [p.get("r")?, p.get("g")?, p.get("b")?];
            let soft: f32 = p.get::<Option<f32>>("soft")?.unwrap_or(0.7);
            let lead = Lead::pastel(c, soft);
            // a pastel pencil: a fine point that keeps
            Ok(match p.get::<Option<f32>>("point")? {
                Some(mm) => Lead { point_mm: mm, blunt_mm: 2000.0, ..lead },
                None => lead,
            })
        }
        "graphite" => {
            let g: String = p.get::<Option<String>>("grade")?.unwrap_or_else(|| "HB".into());
            Lead::pencil(&g).ok_or_else(|| mlua::Error::runtime(format!("pencil grade {g:?}: 9H..H, F, HB, B..9B (e.g. \"2H\", \"HB\", \"2B\", \"4B\")")))
        }
        o => err(format!("pencil kind {o:?}: graphite or chalk (a pastel is made with pastel(pile))")),
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
    draw_with(st, p, lead_of(p)?, marks, seed)
}

/// Draw marks with `lead` (the pencil `p`'s, or its side), wearing `p`.
fn draw_with(st: &S, p: &Table, lead: Lead, marks: &[Mark], seed: u64) -> Result<f32> {
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

// ---------------------------------------------------------------- engine 6: sticks

const STICK_KEYS: &[&str] = &["kind", "soft", "diameter", "length"];
const STROKE_KEYS: &[&str] = &["force", "alt", "azimuth", "roll", "speed"];
/// A pencil's pressure 0..1 as a hand's force on a pastel, N (light 0.1–0.5,
/// normal 0.5–2, heavy 2–5: notes/research/pastel_stick_tribology.md §5).
const FORCE_AT_FULL: f32 = 5.0;

fn is_stick(p: &Table) -> Result<bool> {
    Ok(p.get::<Option<String>>("kind")?.as_deref() == Some("stick"))
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
fn stick_stroke(lua: &Lua, st: &S, p: &Table, pts: &[(f32, f32)], force: &[f32], alt: &[f32], az: &[f32], roll: &[f32], speed: &[f32]) -> Result<f32> {
    use paint::pastel::{Pose, StrokePoint};
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
            if is_stick(&p)? {
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
            if is_stick(&p)? {
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
            if is_stick(&p)? {
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
            let graded = o.as_ref().map(|o| o.get::<Option<bool>>("graded")).transpose()?.flatten().unwrap_or(false);
            let marks = graphite::hatch_marks_graded(&m, angle, spacing, length, pressure, graded, seed);
            if is_stick(&p)? {
                return stick_marks(lua, &st1, &p, &marks);
            }
            draw_marks(&st1, &p, &marks, seed)
        })?)?;
    }
    // p:side(pts, {width=, pressure=0.5, smooth=true, tremor=, seed=}): a
    // pastel laid on its side and drawn along the line, `width` units of it
    // touching (about 12 mm by default): a broad band over the tops of the tooth
    {
        let st1 = st.clone();
        methods.set("side", lua.create_function(move |lua, (p, pts, o): (Table, Value, Option<Table>)| {
            if is_stick(&p)? {
                // engine 6: the stick laid nearly flat across the stroke, a
                // piece `length` mm long (p:snap breaks one off)
                if let Some(o) = &o {
                    check_keys(o, &["force", "pressure", "speed", "roll", "seed", "width", "smooth", "tremor"], "side")?;
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
                return stick_stroke(lua, &st1, &p, &pts, &force, &vec![0.0; n], &az, &roll, &speed);
            }
            if let Some(o) = &o {
                check_keys(o, SIDE_KEYS, "side")?;
            }
            let lead = lead_of(&p)?;
            if lead.medium != paint::Medium::Pastel {
                return err("side: only a pastel is laid on its side (pastel(pile))");
            }
            let pts = points(&pts)?;
            if pts.len() < 2 {
                return err("side: needs at least two points");
            }
            let mmu = {
                let s = st1.borrow();
                mm_per_unit(s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?)
            };
            let width = o.as_ref().map(|o| num(o, "width")).transpose()?.flatten().unwrap_or(12.0 / mmu);
            if !(width > 0.0) {
                return err("side: width wants > 0 (units)");
            }
            let prof = profile(o.as_ref(), 0.5)?;
            let smooth = o.as_ref().map(|o| o.get::<Option<bool>>("smooth")).transpose()?.flatten().unwrap_or(true);
            let tremor = tremor_units(&st1, o.as_ref())?;
            let seed = match &o {
                Some(o) => seed_of(&st1, o)?,
                None => seed_of(&st1, &lua.create_table()?)?,
            };
            let m = graphite::hand_line(&pts, &prof, smooth, false, tremor, seed);
            // the side doesn't wear the point: draw, then put the wear back
            let worn: f32 = p.get::<Option<f32>>("worn")?.unwrap_or(0.0);
            let d = draw_with(&st1, &p, lead.side(width * mmu), &[m], seed)?;
            p.set("worn", worn)?;
            Ok(d)
        })?)?;
    }
    // p:stroke(pts, {force=, alt=, azimuth=, roll=, speed=}): one stroke of
    // an engine-6 stick, each a number or a list along the points: force (N),
    // alt (the stick's angle to the paper, degrees), azimuth (where its upper
    // end points, degrees: 0 to the right, 90 down the canvas), roll (turned
    // about its axis, degrees, added to p:roll's), speed (mm/s)
    {
        let st1 = st.clone();
        methods.set("stroke", lua.create_function(move |lua, (p, pts, o): (Table, Value, Option<Table>)| {
            if !is_stick(&p)? {
                return err("stroke: a stroke of a pastel stick (engine 6); a pencil draws with line");
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
    methods.set("roll", lua.create_function(|_, (p, d): (Table, f32)| {
        if !is_stick(&p)? {
            return err("roll: a pastel stick (engine 6) turns in the fingers");
        }
        if !d.is_finite() {
            return err("roll: degrees, a number");
        }
        let r: f32 = p.raw_get::<Option<f32>>("turned")?.unwrap_or(0.0);
        p.raw_set("turned", (r + d).rem_euclid(360.0))?;
        Ok(())
    })?)?;
    // p:snap(mm): break the stick, keeping a piece that long (its worn end)
    methods.set("snap", lua.create_function(|_, (p, mm): (Table, f32)| {
        if !is_stick(&p)? {
            return err("snap: a pastel stick (engine 6) breaks");
        }
        let l: f32 = p.get("length")?;
        if !(mm > 2.0 && mm < l) {
            return err(format!("snap: keep a piece between 2 mm and its length ({l:.0} mm)"));
        }
        p.set("length", mm)?;
        Ok(())
    })?)?;
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
    meta.set("__tostring", lua.create_function(|_, p: Table| {
        let kind: String = p.get::<Option<String>>("kind")?.unwrap_or_else(|| "graphite".into());
        let worn: f32 = p.get::<Option<f32>>("worn")?.unwrap_or(0.0);
        Ok(if kind == "chalk" {
            format!("black chalk (worn {worn:.0} mm)")
        } else if kind == "stick" {
            let s = stick_of(&p)?;
            let name: String = p.get::<Option<String>>("name")?.unwrap_or_default();
            let sec: String = p.get("section")?;
            format!("pastel {sec} {name} ({} MPa, {:.1} mm long, {} facets)", (s.hardness * 10.0).round() / 10.0, s.length_mm, s.facets.len())
        } else if kind == "pastel" {
            let soft: f32 = p.get::<Option<f32>>("soft")?.unwrap_or(0.7);
            let name: String = p.get::<Option<String>>("name")?.unwrap_or_default();
            format!("pastel {name} (soft {soft:.2}, worn {worn:.0} mm)")
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

    // engine 6: pastel(pile, {kind="soft"|"hard"|"pencil", soft=, diameter=, length=}):
    // a stick of the pile's pigments as they look dry (pastel.rs), that rests on
    // the tooth, wears to facets and lays what it abrades
    // before engine 6: pastel(pile, {soft=0.7, point=}): a pastel stick of the
    // pile's color (its pigments as they look dry: paler than in oil); the
    // pile's medium and thinner don't matter. soft: 0 hard .. 1 very soft;
    // point (mm): a pastel pencil, its point that wide, keeping it (from
    // engine 6 a pencil is kind="pencil" with a diameter)
    // (an older log sees only the globals it saw: pastel and smudge from
    // engine 5, feel from 6, the paper mask from 7; never set before, so a
    // log's walk of its globals is as it was)
    let engine = st.borrow().tubes.engine;
    if engine >= 5 {
        let meta = meta.clone();
        let st1 = st.clone();
        g.set("pastel", lua.create_function(move |lua, (v, o): (Value, Option<Table>)| {
            let pile = crate::api::pile_of(&v, "pastel")?;
            let (engine, tubes) = {
                let s = st1.borrow();
                (s.tubes.engine, s.tubes.clone())
            };
            if engine >= 6 {
                // engine 6: a stick (pastel.rs)
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
                return Ok(p);
            }
            if let Some(o) = &o {
                check_keys(o, PASTEL_KEYS, "pastel")?;
            }
            let soft = o.as_ref().map(|o| num(o, "soft")).transpose()?.flatten().unwrap_or(0.7);
            if !(0.0..=1.0).contains(&soft) {
                return err("pastel: soft is 0 (hard) to 1 (very soft)");
            }
            let point = o.as_ref().map(|o| num(o, "point")).transpose()?.flatten();
            if let Some(mm) = point
                && !(0.2..=10.0).contains(&mm)
            {
                return err("pastel: point is the width of a pastel pencil's point, 0.2 to 10 mm");
            }
            let p = lua.create_table()?;
            p.set("kind", "pastel")?;
            let c = pile.mix.color;
            p.set("r", c[0])?;
            p.set("g", c[1])?;
            p.set("b", c[2])?;
            p.set("soft", soft)?;
            if let Some(mm) = point {
                p.set("point", mm)?;
            }
            p.set("name", pile.recipe())?;
            p.set("worn", 0.0)?;
            p.set_metatable(Some(meta.clone()))?;
            Ok(p)
        })?)?;
    }
    // smudge(mask or pts, {strength=0.6, width=, reach=}): a finger or stump
    // rubbed over loose pastel: drags neighboring colors together (within
    // `reach` units, about 2 mm) and presses it into the tooth
    if engine >= 5 {
        let st1 = st.clone();
        g.set("smudge", lua.create_function(move |_, (a, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, SMUDGE_KEYS, "smudge")?;
            }
            let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
            if st1.borrow().tubes.engine >= 6 {
                // engine 6: a finger (or a stump: pad=) drawn along a path, moving
                // loose pastel (pastel.rs `rub`)
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
                return crate::time::verb_dry(&st1, crate::time::Verb::Pass, |s| {
                    let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                    let secs = c.rub(&pts, force, pad, speed);
                    c.tally_mut().secs += secs as f64 + 0.5;
                    Ok(0)
                });
            }
            let strength = get("strength")?.unwrap_or(0.6);
            let f = frame(&st1)?;
            let mmu = {
                let s = st1.borrow();
                mm_per_unit(s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?)
            };
            let reach = get("reach")?.unwrap_or(2.0 / mmu).max(0.05);
            let m = match &a {
                Value::UserData(_) => (*mask_of(&a)?).clone(),
                Value::Table(_) => {
                    let pts = points(&a)?;
                    // a fingertip, ~8 mm
                    let w = get("width")?.unwrap_or(8.0 / mmu).max(0.1);
                    let pts = if pts.len() == 1 { vec![pts[0], (pts[0].0 + 0.01, pts[0].1)] } else { pts };
                    let ws = vec![w; pts.len()];
                    Mask::from_shape(f, Shape::new().ribbon(&pts, &ws)).blur(0.2 * w)
                }
                o => return err(format!("smudge: want a mask or points, got {}", o.type_name())),
            };
            crate::time::verb(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let moved = c.smudge(&m, strength, reach);
                // hand time: a rub over about 8 cm² a second
                let mm2 = moved as f64 * (c.px_mm() as f64).powi(2);
                c.tally_mut().secs += 1.0 + mm2 / 800.0;
                Ok(moved)
            })
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
            let engine = s.tubes.engine;
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
            c.fix_drawing(m.as_deref());
            if engine >= 6 {
                // a light spray wets about 0.15 of the particles (pastel.rs `fix_pastel`)
                c.fix_pastel(m.as_deref(), 0.15);
            }
            Ok(())
        })?)?;
    }
    // lay_sheet(mask, {grammage=120, tone=}): a sheet of paper laid over the
    // mask to keep it clean (engine 7, paint's sheet.rs); lift_sheet() takes
    // it away with what it caught. It must be lifted in the chunk that laid it.
    if engine >= 7 {
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
    // Engine 7 (paint's sheet.rs). Each returns the volume shed, mm³.
    if engine >= 7 {
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
                if c.engine() < 7 {
                    return Err(mlua::Error::runtime("blow needs engine 7"));
                }
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
                if c.engine() < 7 {
                    return Err(mlua::Error::runtime("tap needs engine 7"));
                }
                let v = c.tap_pastel(a);
                c.tally_mut().secs += 3.0;
                Ok(v)
            })
        })?)?;
    }
    // feel(x, y): what a fingertip feels there (engine 6): the surface, and the
    // pastel in the tooth
    if engine >= 6 {
        let st1 = st.clone();
        // (a query: the painting ages to now first, as `drying` does)
        g.set("feel", lua.create_function(move |_, (x, y): (f32, f32)| {
            crate::time::verb(&st1, crate::time::Verb::Query, |s| {
                let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                Ok(c.feel(x, y))
            })
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
