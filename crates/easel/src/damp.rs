//! Damp printing paper (`--@ damp-paper`; notes/plan-intaglio.md P3a; paint's `damp`):
//! a sheet of paper keeps its own water, as a printer damps it.
//!
//! - `soak(s, {minutes=, drain=})`: in a tray for `minutes`, then lifted out and held up to
//!   drain for `drain` seconds (30 if not given). `soak(s, {mist=<g/m²>})` sprays or sponges
//!   water on instead (unsized and Japan papers are only misted).
//! - `blot(s, {passes=, blotter={mm=, grammage=}})`: between fresh blotters, one each side.
//! - `pack({s1, s2, ...}, {hours=, dry=})`: the sheets interleaved (with `dry` dry sheets of the
//!   first's make) under a board for `hours`: they come to one dampness, and stay covered.
//! - `cover(s)`, `uncover(s, {faces=})`: under plastic or blotters it keeps its water; lying on
//!   the bench (1 face) or hung up (2) it dries on the clock.
//! - `damp_of(s)`: its moisture content (g of water per g of fiber), a word as a printer would
//!   say it ("shiny", "damp", "drying", "dry"), and a table of the rest (its size now, its
//!   stiffness).
//!
//! Each takes a surface (`sheet{}`'s, `receive`'s) or its name; "painting" is the canvas, if it
//! is a blank sheet of paper. A surface put aside is taken up for the verb and put aside again
//! (as `use` does it), so it is aged to the clock first. The time a verb takes passes on the
//! clock for every surface.

use crate::api::{S, err, no_canvas};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};
use paint::damp::{Blotter, GB003, Medium};

/// Seconds a pass of blotting takes the hand: laying the blotters on and pressing them
/// down. A gesture, not physics [E].
const BLOT_S: f64 = 20.0;
/// Seconds misting a sheet takes [E: a gesture].
const MIST_S: f64 = 10.0;
/// The drain if `soak` names none: half a minute held up by an edge [E: a gesture].
const DRAIN_S: f64 = 30.0;

const SOAK_HELP: &str = "soak(s, {minutes=<in the tray>, drain=<seconds held up to drip, 30>}) or soak(s, {mist=<g of water a m²>})";
const BLOT_HELP: &str = "blot(s, {passes=<1>, blotter={mm=<0.8>, grammage=<330>}})";
const PACK_HELP: &str = "pack({s1, s2, ...}, {hours=<under the board>, dry=<dry sheets interleaved, 0>})";

fn rt(e: impl std::fmt::Display) -> mlua::Error {
    mlua::Error::runtime(e.to_string())
}

/// Run `f` with the surface `v` names current, then make the one current before current again.
fn on<R>(st: &S, v: &Value, verb: &str, f: impl FnOnce(&S) -> Result<R>) -> Result<R> {
    let name = crate::surfaces::name_of(v, verb)?;
    let (was, to) = {
        let s = st.borrow();
        if s.canvas.is_none() && s.surfaces.main_current() {
            return Err(no_canvas());
        }
        if name == s.surfaces.current_name() {
            (None, None)
        } else {
            let Some(to) = s.surfaces.find(&name) else { return Err(crate::surfaces::no_surface(&s.surfaces, verb, &name)) };
            if s.canvas.as_ref().is_some_and(|c| c.has_sheet()) {
                return err(format!("{verb}: a sheet of paper lies on the picture; lift it first (lift_sheet())"));
            }
            (Some(s.surfaces.current), Some(to))
        }
    };
    if let Some(to) = to {
        crate::surfaces::take_up(st, to)?;
    }
    let r = f(st);
    if let Some(was) = was {
        crate::surfaces::take_up(st, was)?;
    }
    r
}

/// `f` on the current canvas as a verb with the hand away (its time on the clock).
fn act(st: &S, verb: &'static str, f: impl FnOnce(&mut paint::Canvas) -> std::result::Result<(), String>) -> Result<()> {
    time::verb(st, Verb::Wait, |s| {
        let name = s.surfaces.current_name().to_string();
        let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
        f(c).map_err(|e| rt(format!("{verb}: {name:?}: {e}")))
    })
}

fn opt_num(o: &Table, k: &str) -> Result<Option<f64>> {
    Ok(crate::api::num(o, k)?.map(|v| v as f64))
}

/// What `damp_of` reads (and a replay checkpoint records).
#[derive(serde::Serialize, serde::Deserialize)]
struct Felt {
    mc: f64,
    word: String,
    water: f64,
    standing: f64,
    activity: f64,
    size: (f64, f64),
    made: (f64, f64),
    stiffness: f64,
    z_mpa: f64,
    faces: u8,
    plastic: bool,
}

impl Felt {
    fn of(m: &paint::damp::Moisture) -> Felt {
        Felt { mc: m.mc, word: m.word().to_string(), water: m.water, standing: m.standing, activity: m.activity, size: m.size_mm, made: m.made_mm, stiffness: m.stiffness, z_mpa: m.z_mpa, faces: m.faces, plastic: m.plastic }
    }
}

/// The surface `name`'s moisture now: the current canvas's, or a resting one's from a copy
/// waited up to the clock (the surface itself stays as it rests).
fn feel(s: &crate::api::Studio, name: &str) -> Result<Felt> {
    let not_paper = || rt(format!("damp_of: {name:?} is not a sheet of paper"));
    if name == s.surfaces.current_name() {
        let m = s.canvas.as_ref().ok_or_else(no_canvas)?.moisture().ok_or_else(not_paper)?;
        return Ok(Felt::of(&m));
    }
    let Some(i) = s.surfaces.find(name) else { return Err(crate::surfaces::no_surface(&s.surfaces, "damp_of", name)) };
    let r = s.surfaces.slots[i].rest.as_ref().expect("a surface not current rests");
    let mut c = r.canvas.as_ref().ok_or_else(|| rt("damp_of: a surface without its canvas"))?.restore().map_err(|e| rt(format!("damp_of: {name:?} couldn't be brought back: {e}")))?;
    if s.clock > r.since {
        c.wait((s.clock - r.since) as f32);
    }
    Ok(Felt::of(&c.moisture().ok_or_else(not_paper)?))
}

/// The switch's verbs, installed beside the paper-and-pastel switch's only when the log has
/// the switch, so an older log sees exactly the globals it saw.
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    // soak(s, {minutes=, drain=}) or soak(s, {mist=}): returns its moisture content, g/g
    {
        let st = st.clone();
        g.set("soak", lua.create_function(move |_, (v, o): (Value, Option<Table>)| {
            let o = o.ok_or_else(|| rt(SOAK_HELP))?;
            crate::api::check_keys(&o, &["minutes", "drain", "mist"], "soak")?;
            let (minutes, drain, mist) = (opt_num(&o, "minutes")?, opt_num(&o, "drain")?, opt_num(&o, "mist")?);
            let job: Box<dyn FnOnce(&mut paint::Canvas) -> std::result::Result<(), String>> = match (minutes, mist) {
                (Some(m), None) => {
                    if !(m > 0.0 && m <= crate::api::MAX_WAIT_MIN) {
                        return err(format!("soak: minutes={m}: want minutes in the tray, more than 0\n{SOAK_HELP}"));
                    }
                    let d = drain.unwrap_or(DRAIN_S);
                    if !(1.0..=3600.0).contains(&d) {
                        return err(format!("soak: drain={d}: want seconds held up to drip, 1 to 3600\n{SOAK_HELP}"));
                    }
                    Box::new(move |c| c.damp_soak(m, d))
                }
                (None, Some(w)) => {
                    if drain.is_some() {
                        return err(format!("soak: drain= is for a sheet lifted from the tray, not a misted one\n{SOAK_HELP}"));
                    }
                    if !(0.0..=1000.0).contains(&w) {
                        return err(format!("soak: mist={w}: want g of water a square meter, 0 to 1000\n{SOAK_HELP}"));
                    }
                    Box::new(move |c| {
                        c.damp_mist(w)?;
                        c.wait((MIST_S / 60.0) as f32);
                        Ok(())
                    })
                }
                _ => return err(format!("soak: minutes= (in a tray) or mist= (sprayed on), one of them\n{SOAK_HELP}")),
            };
            on(&st, &v, "soak", |st| {
                act(st, "soak", job)?;
                time::verb(st, Verb::Query, |s| Ok(feel(s, &s.surfaces.current_name().to_string())?.mc))
            })
        })?)?;
    }
    // blot(s, {passes=, blotter={mm=, grammage=}}): returns its moisture content
    {
        let st = st.clone();
        g.set("blot", lua.create_function(move |_, (v, o): (Value, Option<Table>)| {
            let (passes, b) = match &o {
                None => (1, GB003),
                Some(o) => {
                    crate::api::check_keys(o, &["passes", "blotter"], "blot")?;
                    let passes = o.get::<Option<u32>>("passes")?.unwrap_or(1);
                    if !(1..=20).contains(&passes) {
                        return err(format!("blot: passes={passes}: want 1 to 20\n{BLOT_HELP}"));
                    }
                    let b = match o.get::<Option<Table>>("blotter")? {
                        None => GB003,
                        Some(t) => {
                            crate::api::check_keys(&t, &["mm", "grammage"], "blot: blotter")?;
                            let b = Blotter { mm: opt_num(&t, "mm")?.unwrap_or(GB003.mm), grammage: opt_num(&t, "grammage")?.unwrap_or(GB003.grammage) };
                            if !b.valid() {
                                return err(format!("blot: blotter={{mm={}, grammage={}}}: want 0.05 to 5 mm and a grammage the thickness can hold\n{BLOT_HELP}", b.mm, b.grammage));
                            }
                            b
                        }
                    };
                    (passes, b)
                }
            };
            on(&st, &v, "blot", |st| {
                act(st, "blot", |c| {
                    c.damp_blot(passes, b)?;
                    c.wait((passes as f64 * BLOT_S / 60.0) as f32);
                    Ok(())
                })?;
                time::verb(st, Verb::Query, |s| Ok(feel(s, &s.surfaces.current_name().to_string())?.mc))
            })
        })?)?;
    }
    // cover(s), uncover(s, {faces=}): the faces open to the room's air
    {
        let st1 = st.clone();
        g.set("cover", lua.create_function(move |_, v: Value| on(&st1, &v, "cover", |st| act(st, "cover", |c| c.damp_cover(0))))?)?;
        let st1 = st.clone();
        g.set("uncover", lua.create_function(move |_, (v, o): (Value, Option<Table>)| {
            let faces = match &o {
                None => 1,
                Some(o) => {
                    crate::api::check_keys(o, &["faces"], "uncover")?;
                    o.get::<Option<u8>>("faces")?.unwrap_or(1)
                }
            };
            if !(1..=2).contains(&faces) {
                return err("uncover(s, {faces=1}): lying on the bench (1) or hung up (2)");
            }
            on(&st1, &v, "uncover", |st| act(st, "uncover", |c| c.damp_cover(faces)))
        })?)?;
    }
    // pack({s1, s2, ...}, {hours=, dry=}): interleaved under a board, they come to one dampness
    {
        let st = st.clone();
        g.set("pack", lua.create_function(move |_, (list, o): (Option<Table>, Option<Table>)| {
            let list = list.ok_or_else(|| rt(PACK_HELP))?;
            let o = o.ok_or_else(|| rt(PACK_HELP))?;
            crate::api::check_keys(&o, &["hours", "dry"], "pack")?;
            let hours = opt_num(&o, "hours")?.ok_or_else(|| rt(format!("pack: hours= (how long they lie under the board)\n{PACK_HELP}")))?;
            if !(hours >= 0.0 && hours * 60.0 <= crate::api::MAX_WAIT_MIN) {
                return err(format!("pack: hours={hours}: want 0 or more\n{PACK_HELP}"));
            }
            let dry = o.get::<Option<u32>>("dry")?.unwrap_or(0);
            if dry > 1000 {
                return err(format!("pack: dry={dry}: at most 1000 dry sheets\n{PACK_HELP}"));
            }
            let sheets: Vec<Value> = list.sequence_values::<Value>().collect::<Result<_>>()?;
            let mut names = Vec::new();
            for v in &sheets {
                let n = crate::surfaces::name_of(v, "pack")?;
                if names.contains(&n) {
                    return err(format!("pack: {n:?} is in the pack twice"));
                }
                names.push(n);
            }
            if names.is_empty() || names.len() + (dry as usize) < 2 {
                return err(format!("pack: at least two sheets, damp or dry, to interleave\n{PACK_HELP}"));
            }
            // each sheet as it lies in the pack (aged to the clock), and the dry sheets of the first's make
            let mut media: Vec<Medium> = Vec::new();
            let mut filler = None;
            for v in &sheets {
                let (m, f) = on(&st, v, "pack", |st| time::verb(st, Verb::Query, |s| {
                    let name = s.surfaces.current_name().to_string();
                    let c = s.canvas.as_ref().ok_or_else(no_canvas)?;
                    let m = c.damp_medium().map_err(|e| rt(format!("pack: {name:?}: {e}")))?;
                    let p = c.paper().expect("a sheet");
                    let f = Medium::sheet(p, paint::damp::dry_fiber(p) * paint::damp::room_moisture());
                    Ok(((m.fiber, m.pores, m.height, m.water), (f.fiber, f.pores, f.height, f.water)))
                }))?;
                media.push(Medium { fiber: m.0, pores: m.1, height: m.2, water: m.3 });
                filler.get_or_insert(Medium { fiber: f.0, pores: f.1, height: f.2, water: f.3 });
            }
            let mut all = media.clone();
            all.extend(std::iter::repeat_n(filler.expect("a sheet"), dry as usize));
            paint::damp::equalise(&mut all);
            for (v, m) in sheets.iter().zip(&all) {
                let w = m.water;
                on(&st, v, "pack", |st| act(st, "pack", |c| c.damp_set(w, 0)))?;
            }
            // they lie under the board: the hours pass for every surface
            time::verb(&st, Verb::Wait, |s| {
                s.canvas.as_mut().ok_or_else(no_canvas)?.wait((hours * 60.0) as f32);
                Ok(())
            })?;
            Ok(all.iter().take(names.len()).map(|m| m.water / m.fiber).collect::<Vec<f64>>())
        })?)?;
    }
    // damp_of(s): moisture content (g/g), a word, and a table of the rest
    {
        let st = st.clone();
        g.set("damp_of", lua.create_function(move |lua, v: Value| {
            let name = crate::surfaces::name_of(&v, "damp_of")?;
            let f = time::verb(&st, Verb::Query, |s| feel(s, &name))?;
            let t = lua.create_table()?;
            t.set("mc", f.mc)?;
            t.set("water", f.water)?;
            t.set("standing", f.standing)?;
            t.set("activity", f.activity)?;
            t.set("width", f.size.0)?;
            t.set("height", f.size.1)?;
            t.set("made_width", f.made.0)?;
            t.set("made_height", f.made.1)?;
            t.set("stiffness", f.stiffness)?;
            t.set("z_mpa", f.z_mpa)?;
            t.set("faces", f.faces)?;
            t.set("plastic", f.plastic)?;
            Ok((f.mc, f.word, t))
        })?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
