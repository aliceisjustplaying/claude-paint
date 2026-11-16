//! Etching on plates (`--@ etch`; notes/plan-intaglio.md §4, phase P4).
//!
//! A plate in hand (plate.rs) takes, besides its drypoint tools:
//! - `k:ground("hard", {um=})`: a hard ground rolled over its face;
//! - `k:needle(n, pts, {force=, alt=, azimuth=, speed=})`: an etching needle through the ground;
//! - `k:stop_out(mask)`: stopping-out varnish where the mask is;
//! - `k:bite{acid=, baume= | molar= | parts=, temp=, minutes=, face=, rock=, feather=, fresh=}`:
//!   the plate in its tray of ferric chloride, nitric acid or Dutch mordant for `minutes` of the
//!   shared clock (the minutes pass as `wait` passes them, for every surface);
//! - `k:aquatint{box=, wait=, minutes=}`: into the rosin dust box; `k:melt()`: heated until the
//!   grains melt on;
//! - `k:clean()`: solvent: ground, varnish and rosin off.
//!
//! The physics is paint's (`etch`, `plate::etched`; notes/research/etch_physics.md). What the
//! acid opens joins the plate's cells as its lines and pits, as the press will read them (phase
//! P3). The plate's coats and its tray go in its section (`PLAT` version 2) through saves,
//! checkpoints and handoffs. No ink or press yet.

use crate::api::{S, check_keys, err, fmt_num, num};
use crate::plate::{NeedleU, Rep, in_hand, opt, path};
use crate::surfaces::{Kind, SurfaceU};
use mlua::{Lua, Result, Table, UserDataMethods, Value};
use paint::etch::{self, Acid, Face};
use paint::plate::etched::BiteSpec;
use paint::plate::{Op, Plate};
use std::rc::Rc;

/// An etching needle's force when none is given (N): plan-intaglio §3.1's "under 0.3 N" [E: no
/// measurement of a hand on an etching needle; it is held lightly, as a pencil].
pub const NEEDLE_N: f32 = 0.3;
/// Its speed (mm/s) [E: freer than a drypoint needle's 30 mm/s (plate.rs): it moves through
/// wax, as a pencil over paper].
pub const NEEDLE_MM_S: f32 = 40.0;
/// Hand time (s) [E: no timed account found for any of them]: laying a hard ground (warming
/// the plate, rolling, cooling) 10 min; putting the plate in its tray, taking it out, rinsing and
/// drying it 2 min; cranking the dust box 1 min; melting the rosin over a heater 2 min; cleaning
/// off ground and varnish with solvent and rag 5 min.
const GROUND_S: f64 = 600.0;
const BATH_S: f64 = 120.0;
const BOX_S: f64 = 60.0;
const MELT_S: f64 = 120.0;
const CLEAN_S: f64 = 300.0;
/// Brushing stop-out: a brush 8 mm wide at 20 mm/s covers 160 mm² a second [E].
const STOP_MM2_S: f64 = 160.0;
/// Ferric chloride as a bath is made up when no strength is given: 38 °Bé (plan-intaglio §4.2's
/// example; about 3 M, a little past the measured optimum) [P].
const FERRIC_BAUME: f64 = 38.0;
/// Nitric acid: 1 part to 8 of water when no strength is given [P: IP §3.2, 1:8–1:10].
const NITRIC_PARTS: f64 = 8.0;

const BITE_HELP: &str = "k:bite{acid=\"ferric\" | \"nitric\" | \"dutch\", baume=<°Bé> or molar=<mol/L> (ferric), parts=<water to one of acid> or molar= (nitric), temp=<°C>, minutes=<in the bath>, face=\"down\" | \"upright\" | \"up\", rock=true | {tilt=<°>, period=<s>}, feather=true | <s between sweeps>, fresh=true}";

/// The plate `k` names, in hand, with the switch.
fn etching(st: &S, k: &SurfaceU, verb: &str) -> Result<(f32, f32)> {
    if k.kind == Kind::Plate && !st.borrow().tubes.etch {
        return err(format!("{verb}: etching, and this painting doesn't etch (its log's head has no {:?} line)", crate::session::ETCH_MARK));
    }
    let r = in_hand(st, k, verb)?;
    if let Some(p) = st.borrow().plate.as_ref() {
        crate::ink::refuse_inked(p).map_err(|e| mlua::Error::runtime(format!("{verb}: {e}")))?;
    }
    Ok(r)
}

/// The plate in hand, to read.
fn with_plate<R>(st: &S, f: impl FnOnce(&Plate) -> R) -> Result<R> {
    let s = st.borrow();
    Ok(f(s.plate.as_ref().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?))
}

/// A tool on the plate in hand, `minutes` of the clock passing while it works (a bite's or the
/// dust box's: the shared clock, as `wait` passes it) and `secs` of hand time.
fn work(st: &S, minutes: f64, secs: f64, f: impl FnOnce(&mut Plate) -> std::result::Result<Rep, String>) -> Result<Rep> {
    crate::time::verb_plate(st, crate::time::Verb::Pass, move |s| {
        let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
        let r = f(plate).map_err(mlua::Error::runtime)?;
        let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
        if minutes > 0.0 {
            c.wait(minutes as f32);
        }
        c.tally_mut().secs += secs;
        Ok(r)
    })
}

fn table(lua: &Lua, names: &[&str], r: &Rep) -> Result<Table> {
    let t = lua.create_table()?;
    for (k, v) in names.iter().zip(&r.0) {
        t.set(*k, *v)?;
    }
    Ok(t)
}

fn mask_bits(m: &paint::mask::Mask, w: usize, h: usize) -> std::result::Result<Vec<u8>, String> {
    if (m.f.w, m.f.h) != (w, h) {
        return Err("stop_out: the mask isn't the plate's: make it while the plate is in hand".into());
    }
    let mut bits = vec![0u8; (w * h).div_ceil(8)];
    for (i, v) in m.data.iter().enumerate() {
        if *v >= 0.5 {
            bits[i / 8] |= 1 << (i % 8);
        }
    }
    Ok(bits)
}

fn bool_or<T>(o: &Table, key: &str, on: T, of: impl Fn(&Value) -> Result<Option<T>>) -> Result<Option<T>> {
    match o.get::<Value>(key)? {
        Value::Nil | Value::Boolean(false) => Ok(None),
        Value::Boolean(true) => Ok(Some(on)),
        v => of(&v),
    }
}

/// The bite asked for, from `k:bite{...}`.
fn bite_spec(o: &Table) -> Result<BiteSpec> {
    check_keys(o, &["acid", "baume", "molar", "parts", "temp", "minutes", "face", "rock", "feather", "fresh"], "bite")?;
    let acid = match o.get::<Option<String>>("acid")?.as_deref() {
        Some("ferric") => Acid::Ferric,
        Some("nitric") => Acid::Nitric,
        Some("dutch") => Acid::Dutch,
        Some(a) => return err(format!("bite: acid={a:?}: the box has ferric chloride (\"ferric\"), nitric acid (\"nitric\") and Dutch mordant (\"dutch\")")),
        None => return err(format!("bite: acid=\n{BITE_HELP}")),
    };
    let (baume, molar, parts) = (num(o, "baume")?, num(o, "molar")?, num(o, "parts")?);
    let molar = match acid {
        Acid::Ferric => {
            if parts.is_some() {
                return err("bite: ferric chloride's strength is baume= or molar=");
            }
            match (baume, molar) {
                (Some(_), Some(_)) => return err("bite: baume= or molar=, not both"),
                (Some(b), None) if (5.0..=46.0).contains(&b) => etch::ferric_molar_of_baume(b as f64),
                (Some(_), None) => return err("bite: baume is 5 to 46 °Bé (the stock is 42–45)"),
                (None, Some(m)) if (0.1..=3.5).contains(&m) => m as f64,
                (None, Some(_)) => return err("bite: molar is 0.1 to 3.5 mol/L of ferric chloride (40 % stock is 3.5)"),
                (None, None) => etch::ferric_molar_of_baume(FERRIC_BAUME),
            }
        }
        Acid::Nitric => {
            if baume.is_some() {
                return err("bite: nitric acid's strength is parts= (of water to one of acid) or molar=");
            }
            match (parts, molar) {
                (Some(_), Some(_)) => return err("bite: parts= or molar=, not both"),
                (Some(p), None) if (1.0..=30.0).contains(&p) => etch::NITRIC_CONC_M / (1.0 + p as f64),
                (Some(_), None) => return err("bite: parts is 1 to 30 parts of water to one of acid (1:8 is usual)"),
                (None, Some(m)) if (0.1..=7.9).contains(&m) => m as f64,
                (None, Some(_)) => return err("bite: molar is 0.1 to 7.9 mol/L of nitric acid"),
                (None, None) => etch::NITRIC_CONC_M / (1.0 + NITRIC_PARTS),
            }
        }
        Acid::Dutch => {
            if baume.is_some() || molar.is_some() || parts.is_some() {
                return err("bite: Dutch mordant is made by its recipe (hydrochloric acid 100 g, potassium chlorate 20 g, water 880 g): no strength to give");
            }
            etch::DUTCH_CHLORATE_M
        }
    };
    let temp = num(o, "temp")?.unwrap_or(20.0) as f64;
    if !(5.0..=50.0).contains(&temp) {
        return err("bite: temp is the bath's °C, 5 to 50");
    }
    let minutes = num(o, "minutes")?.ok_or_else(|| mlua::Error::runtime(format!("bite: minutes= (in the bath)\n{BITE_HELP}")))? as f64;
    if !(minutes > 0.0 && minutes <= 24.0 * 60.0) {
        return err("bite: minutes is the time in the bath, over 0 and at most a day");
    }
    let face = match o.get::<Option<String>>("face")?.as_deref() {
        // (ferric face down, as Kiekeben has it; nitric and Dutch in a tray, face up, to be swept)
        None => if acid == Acid::Ferric { Face::Down } else { Face::Up },
        Some("down") => Face::Down,
        Some("upright") => Face::Upright,
        Some("up") => Face::Up,
        Some(f) => return err(format!("bite: face={f:?}: \"down\" (held off the tray's floor), \"upright\" (in a tank) or \"up\" (in a tray)")),
    };
    let rock = bool_or(o, "rock", (etch::ROCK_TILT_DEG, etch::ROCK_PERIOD_S), |v| match v {
        Value::Table(t) => {
            check_keys(t, &["tilt", "period"], "bite rock")?;
            let tilt = num(t, "tilt")?.unwrap_or(etch::ROCK_TILT_DEG as f32) as f64;
            let period = num(t, "period")?.unwrap_or(etch::ROCK_PERIOD_S as f32) as f64;
            if !(0.5..=20.0).contains(&tilt) || !(1.0..=60.0).contains(&period) {
                return err("bite: rock={tilt=<0.5 to 20°>, period=<1 to 60 s>}");
            }
            Ok(Some((tilt, period)))
        }
        _ => err("bite: rock=true or rock={tilt=, period=}"),
    })?;
    let feather = bool_or(o, "feather", etch::FEATHER_S, |v| match v {
        Value::Integer(n) if (1..=600).contains(n) => Ok(Some(*n as f64)),
        Value::Number(n) if (1.0..=600.0).contains(n) => Ok(Some(*n)),
        _ => err("bite: feather=true, or seconds between its sweeps (1 to 600)"),
    })?;
    if feather.is_some() && acid != Acid::Nitric {
        return err("bite: a feather sweeps away nitric acid's bubbles; this mordant gives none");
    }
    if feather.is_some() && face != Face::Up {
        return err("bite: a feather sweeps a plate lying face up in its tray (face=\"up\")");
    }
    let fresh = o.get::<Option<bool>>("fresh")?.unwrap_or(false);
    Ok(BiteSpec { acid, molar, temp_c: temp, minutes, face, rock, feather, fresh })
}

/// The etching tools, a plate's methods (plate.rs `methods`).
pub(crate) fn methods<M: UserDataMethods<SurfaceU>>(m: &mut M) {
    // k:ground("hard", {um=}): a hard ground rolled over the face
    m.add_method("ground", |lua, k, (kind, o): (Option<String>, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "ground")?;
        match kind.as_deref() {
            Some("hard") => {}
            Some("soft") => return err("ground: soft ground is its own switch, later (--@ soft-ground)"),
            _ => return err("ground: k:ground(\"hard\", {um=<µm>})"),
        }
        if let Some(o) = &o {
            check_keys(o, &["um"], "ground")?;
        }
        let um = opt(o.as_ref(), "um", etch::GROUND_UM as f32, 1.0, 30.0, "the ground's thickness in µm (2–8 for a hard ground rolled thin)")?;
        let r = work(&st, 0.0, GROUND_S, move |p| {
            if p.coated() {
                return Err("ground: the plate has a ground, varnish, rosin or the acid's work on it: clean it first (k:clean())".into());
            }
            p.apply(Op::Ground { um });
            Ok((vec![um as f64, GROUND_S], false, false))
        })?;
        table(lua, &["um", "seconds"], &r)
    });
    // k:needle(n, pts, {force=, alt=, azimuth=, speed=}): an etching needle through the ground
    m.add_method("needle", |lua, k, (n, pts, o): (Value, Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "needle")?;
        let Value::UserData(n) = n else { return err("needle: the needle first: k:needle(n, pts, {...}), n = needle{tip=, cone=}") };
        let n = *n.borrow::<NeedleU>().map_err(|_| mlua::Error::runtime("needle: the needle first: n = needle{tip=, cone=}"))?;
        if let Some(o) = &o {
            check_keys(o, &["force", "alt", "azimuth", "speed"], "needle")?;
        }
        let pts = path(&st, k, "needle", &pts, o.as_ref(), NEEDLE_N, 5.0, NEEDLE_MM_S)?;
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, move |s| {
            let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            if plate.ground_um() <= 0.0 {
                return Err(mlua::Error::runtime("needle: an etching needle draws through a ground: lay one first (k:ground(\"hard\")), or draw in the metal itself (k:drypoint)"));
            }
            let r = plate.apply(Op::Needle { tip_um: n.tip, cone_deg: n.cone, pts });
            let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
            c.tally_mut().secs += r.secs;
            Ok((vec![r.width_um, r.depth_um, r.groove_um3, r.chip_um3, r.length_mm, r.secs], r.ploughed || r.cut, r.cut))
        })?;
        if !r.1 {
            st.borrow_mut().out.push_str("needle: it didn't reach the metal (too light for the ground): nothing will bite there\n");
        }
        let t = table(lua, &["width", "depth", "groove", "chip", "length", "seconds"], &r)?;
        t.set("touched", r.1)?;
        Ok(t)
    });
    // k:stop_out(mask): stopping-out varnish brushed where the mask is
    m.add_method("stop_out", |lua, k, m: Value| {
        let st = crate::api::studio(lua)?;
        let (mm, _) = etching(&st, k, "stop_out")?;
        let mask = crate::api::mask_of(&m)?;
        let (w, h) = with_plate(&st, |p| p.size_px())?;
        let bits = mask_bits(&mask, w, h).map_err(mlua::Error::runtime)?;
        let cells = mask.data.iter().filter(|v| **v >= 0.5).count() as f64;
        let mm2 = cells * (mm as f64 / w as f64).powi(2);
        let secs = mm2 / STOP_MM2_S;
        let r = work(&st, 0.0, secs, move |p| {
            p.apply(Op::StopOut { w: w as u32, h: h as u32, bits });
            Ok((vec![mm2, secs], false, false))
        })?;
        table(lua, &["area", "seconds"], &r)
    });
    // k:bite{...}: the plate in its tray for minutes of the shared clock
    m.add_method("bite", |lua, k, o: Option<Table>| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "bite")?;
        let o = o.ok_or_else(|| mlua::Error::runtime(BITE_HELP))?;
        let spec = bite_spec(&o)?;
        let r = work(&st, spec.minutes, BATH_S, move |p| {
            let r = p.bite(spec);
            Ok((vec![r.rate_um_min, r.delta_um, r.bubbles, r.deepest_um, r.metal_um3, r.loaded_g_l, r.molar, spec.minutes], r.washed, false))
        })?;
        if r.1 {
            st.borrow_mut().out.push_str("bite: the rosin wasn't melted on (k:melt()): it floated off, and the acid bit the bare face\n");
        }
        let t = table(lua, &["rate", "layer", "bubbles", "depth", "metal", "loaded", "molar", "minutes"], &r)?;
        t.set("washed", r.1)?;
        Ok(t)
    });
    // k:aquatint{box=, wait=, minutes=}: into the rosin dust box
    m.add_method("aquatint", |lua, k, o: Option<Table>| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "aquatint")?;
        if let Some(o) = &o {
            check_keys(o, &["box", "wait", "minutes"], "aquatint")?;
        }
        let h = opt(o.as_ref(), "box", 0.6, 0.2, 3.0, "the box's height in m")? as f64;
        let wait = opt(o.as_ref(), "wait", 30.0, 0.0, 3600.0, "seconds between stirring the cloud and putting the plate in")? as f64;
        let minutes = opt(o.as_ref(), "minutes", 1.0, 0.05, 60.0, "minutes the plate lies in the box")? as f64;
        let per: Vec<f32> = etch::dust_box(h, wait, minutes).iter().map(|&v| v as f32).collect();
        let cover = etch::cover(&per.iter().map(|&v| v as f64).collect::<Vec<_>>(), 1.0);
        let largest = etch::largest_grain(h, wait).min(etch::dust_bin(etch::DUST_BINS - 1).1);
        let name = k.name.clone();
        let r = work(&st, wait / 60.0 + minutes, BOX_S, move |p| {
            if p.ground_um() > 0.0 {
                return Err("aquatint: the plate has a ground: the rosin goes on bare metal (k:clean() first)".into());
            }
            // (the grains' places: from the plate's name and its history)
            let seed = name.bytes().fold(p.ops.len() as u64, |h, b| h.wrapping_mul(0x100_0000_01B3).wrapping_add(b as u64));
            p.apply(Op::Aquatint { per_um2: per, seed });
            Ok((vec![largest, cover, wait, minutes], false, false))
        })?;
        table(lua, &["largest", "cover", "wait", "minutes"], &r)
    });
    // k:melt(): the plate heated until its rosin melts on
    m.add_method("melt", |lua, k, (): ()| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "melt")?;
        let r = work(&st, 0.0, MELT_S, |p| {
            if p.coats().is_none_or(|c| c.dust.iter().all(|d| d.melted)) {
                return Err("melt: no loose rosin on the plate to melt (k:aquatint{} first)".into());
            }
            p.apply(Op::Melt);
            Ok((vec![MELT_S], false, false))
        })?;
        table(lua, &["seconds"], &r)
    });
    // k:clean(): ground, varnish and rosin off with solvent
    m.add_method("clean", |lua, k, (): ()| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "clean")?;
        let r = work(&st, 0.0, CLEAN_S, |p| {
            p.apply(Op::Clean);
            Ok((vec![CLEAN_S], false, false))
        })?;
        table(lua, &["seconds"], &r)
    });
}

/// A coated cell as a fingernail finds it: words, and (ground µm, rosin's share).
pub(crate) fn feel(p: &Plate, x: usize, y: usize) -> (String, Vec<f64>) {
    let e = p.etch(x, y);
    let ground = p.ground_um() as f64;
    let rosin = p.rosin_cover(x, y);
    let mut words = Vec::new();
    if e.stopped {
        words.push("varnished".to_string());
    }
    if ground > 0.0 {
        let bared = e.lines.iter().any(|l| l.lam > 0.0);
        words.push(format!("hard ground {} µm{}", fmt_num(ground as f32), if bared { ", a needle line through it" } else { "" }));
    }
    if rosin > 0.0 {
        let melted = p.coats().is_some_and(|c| c.dust.iter().all(|d| d.melted));
        words.push(format!("{} rosin over {}%", if melted { "melted" } else { "loose" }, (100.0 * rosin).round()));
    }
    (words.join(", "), vec![ground, rosin])
}

/// `probe` on a coated plate over cells (x0, y0, x1, y1): its coats, the acid's bite and its tray.
pub(crate) fn probe(p: &Plate, r: (usize, usize, usize, usize)) -> String {
    let (x0, y0, x1, y1) = r;
    let n = ((x1 - x0) * (y1 - y0)).max(1) as f64;
    let (mut stopped, mut rosin, mut deepest, mut held) = (0.0f64, 0.0f64, 0.0f32, 0.0f64);
    for y in y0..y1 {
        for x in x0..x1 {
            let e = p.etch(x, y);
            stopped += e.stopped as u8 as f64;
            rosin += p.rosin_cover(x, y);
            deepest = deepest.max(e.lines.iter().map(|l| l.z).fold(e.z, f32::max));
            held += p.cell(x, y).openings().map(|(s, w, d)| etch::held_um(s as f64, w as f64, d as f64)).sum::<f64>();
        }
    }
    let mut out = String::new();
    if p.ground_um() > 0.0 {
        out.push_str(&format!("hard ground {} µm\n", fmt_num(p.ground_um())));
    }
    if stopped > 0.0 {
        out.push_str(&format!("stopped out over {}%\n", (100.0 * stopped / n).round()));
    }
    if rosin > 0.0 {
        out.push_str(&format!("rosin over {}%\n", (100.0 * rosin / n).round()));
    }
    out.push_str(&format!("bitten up to {} µm · a wipe's yarns would leave {} µm of ink\n", fmt_num((deepest * 10.0).round() / 10.0), fmt_num((held / n * 100.0).round() as f32 / 100.0)));
    if let Some(t) = p.coats().and_then(|c| c.tray) {
        out.push_str(&format!(
            "its tray: {} {} mol/L as made, {} L, {} g/L of metal taken in · the acid has taken {} mg from the plate\n",
            t.acid.name(),
            fmt_num((t.molar0 * 100.0).round() as f32 / 100.0),
            fmt_num((t.litres * 100.0).round() as f32 / 100.0),
            fmt_num((t.metal_mol * etch::molar(p.metal).0 * 1000.0 / t.litres * 1000.0).round() as f32 / 1000.0),
            fmt_num((p.bitten_um3() * 1e-18 * etch::molar(p.metal).0 / etch::molar(p.metal).1 * 1e6 * 1000.0).round() as f32 / 1000.0)
        ));
    }
    out
}

#[cfg(test)]
mod tests;
