//! The press (`--@ press`; notes/plan-intaglio.md §6.2, phase P3b).
//!
//! `press{plate=k, sheet=s, at={x, y}, felts={1.6, 6.35, 3.2}, pressure=0.7, roller=53,
//! speed=20, load=130}` runs an inked plate and a sheet through the press: the sheet is taken
//! up (and put aside again after, as `soak` does it), the plate, resting, gives up its ink, and
//! the print lands on the sheet mirrored, the plate's left edge `at` mm from the sheet's (x, y),
//! centred if not given. A damp sheet keeps the plate's shape: its lines stand up, its burr is
//! pressed in, the plate mark sinks the footprint. The physics is paint's (`plate::press`;
//! notes/research/intaglio_physics.md §6).

use crate::api::{S, check_keys, err, num};
use crate::surfaces::{Kind, SurfaceU};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};
use paint::plate::press::{PressSpec, SheetFeel};
use std::rc::Rc;

const PRESS_HELP: &str = "press{plate=k, sheet=s, at={<x mm>, <y mm>}, felts={<mm>, ...}, pressure=<the screws, 0 to 1 of the press's load>, roller=<radius mm>, speed=<mm/s>, load=<the press's rating, N/mm>}";

/// The press's settings from `press{}`.
fn spec(o: &Table) -> Result<PressSpec> {
    let mut s = PressSpec::default();
    if let Some(p) = num(o, "pressure")? {
        if !(0.0..=1.0).contains(&p) {
            return err("press: pressure is the screws, 0 to 1 of the press's load");
        }
        s.pressure = p as f64;
    }
    if let Some(l) = num(o, "load")? {
        if !(10.0..=1500.0).contains(&l) {
            return err("press: load is the press's rated line load, N/mm (a hand press 130; banknote presses to 1000)");
        }
        s.rated_n_mm = l as f64;
    }
    if let Some(r) = num(o, "roller")? {
        if !(10.0..=300.0).contains(&r) {
            return err("press: roller is its radius in mm, 10 to 300");
        }
        s.roller_mm = r as f64;
    }
    if let Some(v) = num(o, "speed")? {
        if !(1.0..=500.0).contains(&v) {
            return err("press: speed is the bed's in mm/s, 1 to 500");
        }
        s.speed_mm_s = v as f64;
    }
    match o.get::<Value>("felts")? {
        Value::Nil => {}
        Value::Table(t) => {
            let f: Vec<f64> = t.sequence_values::<f64>().collect::<Result<_>>()?;
            if f.is_empty() || f.len() > 6 || f.iter().any(|v| !(0.2..=20.0).contains(v)) {
                return err("press: felts are their thicknesses in mm, one to six of 0.2 to 20 (a catcher 1.6, a cushion 6.35, a pusher 3.2)");
            }
            s.felts_mm = f;
        }
        _ => return err("press: felts={<mm>, ...}"),
    }
    Ok(s)
}

/// `press{}`, the global (installed with `ink{}` under the switch).
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    lua.globals().set("press", lua.create_function(move |lua, o: Option<Table>| press(lua, &st, o))?)?;
    Ok(())
}

fn press(lua: &Lua, st: &S, o: Option<Table>) -> Result<Table> {
    let o = o.ok_or_else(|| mlua::Error::runtime(PRESS_HELP))?;
    check_keys(&o, &["plate", "sheet", "at", "felts", "pressure", "roller", "speed", "load"], "press")?;
    let plate_name = match o.get::<Value>("plate")? {
        Value::UserData(u) => u.borrow::<SurfaceU>().map_err(|_| mlua::Error::runtime("press: plate= is a plate"))?.name.clone(),
        Value::String(s) => s.to_str()?.to_string(),
        _ => return err(format!("press: plate=\n{PRESS_HELP}")),
    };
    let sheet_v: Value = o.get("sheet")?;
    let sheet_name = crate::surfaces::name_of(&sheet_v, "press")?;
    {
        let s = st.borrow();
        let kind = |n: &str| s.surfaces.find(n).map(|i| s.surfaces.slots[i].kind);
        if kind(&plate_name) != Some(Kind::Plate) {
            return err(format!("press: {plate_name:?} isn't a plate"));
        }
        match kind(&sheet_name) {
            Some(Kind::Sheet) => {}
            Some(Kind::Painting) if s.paper => {}
            _ => return err(format!("press: {sheet_name:?} isn't a sheet of paper")),
        }
    }
    let spec = spec(&o)?;
    let at = crate::api::pair(&o, "at")?;
    let r = crate::damp::on(st, &sheet_v, "press", |st| {
        time::verb(st, Verb::Marks, |s| {
            let pi = s.surfaces.find(&plate_name).ok_or_else(|| mlua::Error::runtime("press: the plate is gone"))?;
            let tubes = s.tubes.clone();
            let clock = s.clock;
            let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
            let rest = s.surfaces.slots[pi].rest.as_mut().ok_or_else(|| mlua::Error::runtime("press: the plate is in hand"))?;
            // the plate's clock: its own, read against the studio's (surfaces.rs `wake_up`)
            let now = rest.own.clock0 + clock;
            let plate = Rc::make_mut(rest.own.plate.as_mut().ok_or_else(|| mlua::Error::runtime("press: the plate has no state"))?);
            if !plate.inked() {
                return Err(mlua::Error::runtime("press: the plate has no ink on it (k:ink(...) and wipe it first)"));
            }
            let paper = *c.paper().ok_or_else(|| mlua::Error::runtime(format!("press: {sheet_name:?} isn't a sheet of paper")))?;
            let m = c.moisture().ok_or_else(|| mlua::Error::runtime("press: the sheet has no moisture"))?;
            let feel = SheetFeel { thick_um: paper.thick_um as f64, stiffness: m.stiffness, damp: m.plastic, absorbent: paper.absorbent as f64 };
            // the inks as the sheet's oil paint: their tubes from the box, their added plate oil as medium
            let mut paints = Vec::new();
            for ink in plate.inking().map(|k| k.inks.clone()).unwrap_or_default() {
                let parts: Vec<(usize, f32)> = ink.tubes.iter().filter_map(|(n, p)| tubes.tubes.iter().position(|t| t.name == n.as_str()).map(|i| (i, *p))).collect();
                if parts.len() != ink.tubes.len() {
                    return Err(mlua::Error::runtime(format!("press: the ink {:?} is made of tubes this box hasn't", ink.name)));
                }
                paints.push(tubes.pile(parts).paint(ink.oil));
            }
            let (sheet_w, sheet_h) = (c.mm_per_unit() as f64 * 1000.0, c.mm_per_unit() as f64 * c.height() as f64);
            let at = match at {
                Some((x, y)) => (x as f64, y as f64),
                None => ((sheet_w - plate.width_mm as f64) / 2.0, (sheet_h - plate.height_mm as f64) / 2.0),
            };
            let r = plate.press(c, at, &spec, &feel, &paints, now).map_err(|e| mlua::Error::runtime(format!("press: {e}")))?;
            c.tally_mut().secs += r.secs;
            let q = r.squeeze;
            Ok((vec![q.on_plate_pa * 1e-6, q.beside_pa * 1e-6, q.contact_mm, q.strain, r.split, r.taken_um3 * 1e-9, r.after_um3 * 1e-9, r.mark_um, r.secs], r.damp))
        })
    })?;
    let t = lua.create_table()?;
    for (k, v) in ["pressure", "beside", "contact", "strain", "split", "taken", "left", "mark", "seconds"].iter().zip(&r.0) {
        t.set(*k, *v)?;
    }
    t.set("damp", r.1)?;
    t.set("mirrored", true)?;
    Ok(t)
}

#[cfg(test)]
mod tests;
