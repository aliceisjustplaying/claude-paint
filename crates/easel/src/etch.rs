//! Etching on plates (`--@ etch`; notes/plan-intaglio.md §4, phase P4).
//!
//! A plate in hand (plate.rs) takes, besides its drypoint tools:
//! - `k:ground("hard", pts, {um=, roller=, speed=})`: a hard ground rolled along the points
//!   with a leather roller: it lies where the roller went;
//! - `k:needle(n, pts, {force=, alt=, azimuth=, speed=})`: an etching needle through the ground;
//! - `k:stop_out(pts, {brush=, load=, pressure=, speed=})`: stopping-out varnish brushed along
//!   the points with a round brush: it lies where the brush laid it, as wide as the brush pressed
//!   out, until the brush runs dry;
//! - `k:immerse{acid=, baume= | molar= | parts=, temp=, face=, rock=, feather=, fresh=}`: the
//!   plate into its tray of ferric chloride, nitric acid or Dutch mordant, where it lies on the
//!   session's clock, whatever else the session does, until `k:lift()` takes it out: the bite is
//!   the minutes between, on the plate's own clock (the session's, which a rest, a save and a
//!   handoff keep);
//! - `k:bite{..., minutes=}`: in, `minutes`, out, in one (the same as `k:immerse{...}`,
//!   `wait(minutes)`, `k:lift()`, bit for bit);
//! - `k:aquatint{box=, wait=, minutes=}`: into the rosin dust box; `k:melt()`: heated until the
//!   grains melt on;
//! - `k:clean(pts, {passes=, width=, speed=})`: a rag dampened with solvent rubbed along the
//!   points: ground, varnish and rosin off where it dissolves through them.
//!
//! The physics is paint's (`etch`, `plate::etched`; notes/research/etch_physics.md). What the
//! acid opens joins the plate's cells as its lines and pits, as the press will read them (phase
//! P3). The plate's coats and its tray go in its section (`PLAT` version 2) through saves,
//! checkpoints and handoffs. No ink or press yet.

use crate::api::{S, check_keys, err, fmt_num, num};
use crate::plate::{NeedleU, Rep, hand_path, in_hand, opt, path};
use crate::surfaces::{Kind, SurfaceU};
use mlua::{Lua, Result, Table, UserDataMethods, Value};
use paint::etch::{self, Acid, Face};
use paint::plate::etched::{BathSpec, InBath};
use paint::plate::{Op, Plate};
use std::rc::Rc;

/// An etching needle's force when none is given (N): plan-intaglio §3.1's "under 0.3 N" [E: no
/// measurement of a hand on an etching needle; it is held lightly, as a pencil].
pub const NEEDLE_N: f32 = 0.3;
/// Its speed (mm/s) [E: freer than a drypoint needle's 30 mm/s (plate.rs): it moves through
/// wax, as a pencil over paper].
pub const NEEDLE_MM_S: f32 = 40.0;
/// Hand time (s) [E: no timed account found for any of them]: laying a hard ground (warming
/// the plate and letting it cool, `etch::GROUND_WARM_S`, and the roller's path at its speed);
/// taking the plate out of its tray, rinsing and drying it
/// 2 min (lowering it in takes seconds, which are the bath's own: they are in its minutes);
/// cranking the dust box 1 min; melting the rosin over a heater 2 min. A solvent rag's time is
/// the area it rubs at a tarlatan pass's pace (`Plate::wash`).
const LIFT_S: f64 = 120.0;
const BOX_S: f64 = 60.0;
const MELT_S: f64 = 120.0;
/// A solvent rag's speed when none is given (mm/s) [E: rubbed as briskly as a tarlatan pad].
pub(crate) const RAG_MM_S: f32 = 100.0;
pub(crate) const CLEAN_HOW: &str = "k:clean({{x, y}, ...}, {passes=, width=<the rag's pad, mm>, speed=}): the solvent rag's path, back and forth or round where the coats should come off";
const GROUND_HOW: &str = "k:ground(\"hard\", {{x, y}, ...}, {um=, roller=<mm>, speed=}): the roller's path over the plate, overlapping strokes to cover it";
const STOP_HOW: &str = "k:stop_out({{x, y}, ...}, {brush=<mm>, load=<0 to 1>, pressure=<0 to 1, or a list along the points>, speed=}): the brush's stroke, dipped again for the next";
/// Ferric chloride as a bath is made up when no strength is given: 38 °Bé (plan-intaglio §4.2's
/// example; about 3 M, a little past the measured optimum) [P].
const FERRIC_BAUME: f64 = 38.0;
/// Nitric acid: 1 part to 8 of water when no strength is given [P: IP §3.2, 1:8–1:10].
const NITRIC_PARTS: f64 = 8.0;

const BATH_KEYS: &str = "acid=\"ferric\" | \"nitric\" | \"dutch\", baume=<°Bé> or molar=<mol/L> (ferric), parts=<water to one of acid> or molar= (nitric), temp=<°C>, face=\"down\" | \"upright\" | \"up\", rock=true | {tilt=<°>, period=<s>}, feather=true | <s between sweeps>, fresh=true";

fn help(verb: &str) -> String {
    match verb {
        "bite" => format!("k:bite{{{BATH_KEYS}, minutes=<in the bath>}}: in, minutes, out (or k:immerse{{...}} ... k:lift())"),
        _ => format!("k:immerse{{{BATH_KEYS}}}: the plate into its bath, where it bites on the clock until k:lift()"),
    }
}

/// The plate `k` names, in hand, with the switch (and out of its bath: `in_hand`).
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

/// `etching`, for `k:lift()`: the plate in hand, in its bath or not.
fn etching_in_bath(st: &S, k: &SurfaceU, verb: &str) -> Result<()> {
    if k.kind == Kind::Plate && !st.borrow().tubes.etch {
        return err(format!("{verb}: etching, and this painting doesn't etch (its log's head has no {:?} line)", crate::session::ETCH_MARK));
    }
    crate::plate::held(st, k, verb).map(|_| ())
}

/// The minutes a plate has lain in its bath by `now` (its clock), to the second, for words.
fn so_far(b: &InBath, now: f64) -> String {
    fmt_num(((now - b.since).max(0.0) * 60.0).round() as f32 / 60.0)
}

/// Its bath in words: "ferric 2.94 mol/L at 20 °C, face down, rocked".
fn bath_words(b: &BathSpec) -> String {
    let mut w = format!("{} {} mol/L at {} °C, face {}", b.acid.name(), fmt_num((b.molar * 100.0).round() as f32 / 100.0), fmt_num(b.temp_c as f32), b.face.name());
    if b.rock.is_some() {
        w.push_str(", rocked");
    }
    if let Some(f) = b.feather {
        w.push_str(&format!(", feathered every {} s", fmt_num(f as f32)));
    }
    w
}

/// A plate's tool refused while the plate lies in its bath (plate.rs `in_hand`): None if it
/// isn't in one.
pub(crate) fn refuse_in_bath(p: &Plate, now: f64, verb: &str, name: &str) -> Option<String> {
    let b = p.in_bath()?;
    Some(format!(
        "{verb}: the plate {name:?} is in its {} bath ({} min so far): lift it out first (k:lift())",
        b.bath.acid.name(),
        so_far(&b, now)
    ))
}

/// `probe`'s line for a plate in its bath at `now` (its clock): empty if it isn't in one.
pub(crate) fn bath_line(p: &Plate, now: f64) -> String {
    match p.in_bath() {
        None => String::new(),
        Some(b) => format!("in its bath: {}, {} min so far (seen as it would come out now; k:lift() takes it out)\n", bath_words(&b.bath), so_far(&b, now)),
    }
}

/// `status`'s words for the plates lying in their baths: " · in the bath: key (ferric,
/// 12 min)", or empty. A plate's clock is the studio's plus its offset (surfaces.rs `Own`).
pub(crate) fn baths(s: &crate::api::Studio) -> String {
    let mut out = Vec::new();
    let mut one = |name: &str, p: Option<&Rc<Plate>>, clock0: f64| {
        if let Some(b) = p.and_then(|p| p.in_bath()) {
            out.push(format!("{name} ({}, {} min)", b.bath.acid.name(), so_far(&b, s.clock + clock0)));
        }
    };
    one(s.surfaces.current_name(), s.plate.as_ref(), s.clock0);
    for slot in &s.surfaces.slots {
        if let Some(r) = slot.rest.as_ref() {
            one(&slot.name, r.own.plate.as_ref(), r.own.clock0);
        }
    }
    if out.is_empty() { String::new() } else { format!(" · in the bath: {}", out.join(", ")) }
}

/// The plate in hand's clock (minutes): its canvas's, which is the session's clock read on the
/// plate, kept by a rest, a save and a handoff.
fn plate_clock(s: &crate::api::Studio) -> Result<f64> {
    Ok(s.canvas.as_ref().ok_or_else(crate::api::no_canvas)?.clock())
}

/// A bite's report as it is recorded and returned.
fn bite_rep(r: &paint::plate::etched::BiteReport) -> Rep {
    (vec![r.rate_um_min, r.delta_um, r.bubbles, r.deepest_um, r.metal_um3, r.loaded_g_l, r.molar, r.minutes], r.washed, r.spent)
}
const BITE_NAMES: [&str; 8] = ["rate", "layer", "bubbles", "depth", "metal", "loaded", "molar", "minutes"];

/// What a lift (or a bite in one) says beyond its numbers.
fn lift_words(st: &S, p_thick_um: f64, r: &Rep) {
    let mut s = st.borrow_mut();
    if r.1 {
        s.out.push_str("bite: the rosin wasn't melted on (k:melt()): it floated off, and the acid bit the bare face\n");
    }
    if r.2 {
        s.out.push_str("lift: the bath was spent before the plate came out: its oxidant is used up, and it bit no more\n");
    }
    if r.0[3] >= p_thick_um {
        s.out.push_str(&format!("lift: the acid has bitten {} µm deep, past the plate's {} µm: it is holed there\n", fmt_num(r.0[3].round() as f32), fmt_num(p_thick_um.round() as f32)));
    }
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

/// The plate's stopped-out cells.
fn stopped_cells(p: &Plate) -> usize {
    let (w, h) = p.size_px();
    (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| p.etch(x, y).stopped).count()
}

fn bool_or<T>(o: &Table, key: &str, on: T, of: impl Fn(&Value) -> Result<Option<T>>) -> Result<Option<T>> {
    match o.get::<Value>(key)? {
        Value::Nil | Value::Boolean(false) => Ok(None),
        Value::Boolean(true) => Ok(Some(on)),
        v => of(&v),
    }
}

/// The bath asked for, from `k:immerse{...}` or `k:bite{...}` (`verb`), and a bite's minutes.
fn bath_spec(o: &Table, verb: &str) -> Result<(BathSpec, Option<f64>)> {
    let bite = verb == "bite";
    check_keys(o, if bite { &["acid", "baume", "molar", "parts", "temp", "minutes", "face", "rock", "feather", "fresh"] } else { &["acid", "baume", "molar", "parts", "temp", "face", "rock", "feather", "fresh"] }, verb)?;
    // (the bath's messages name the verb asked)
    let named = |e: mlua::Error| match e {
        mlua::Error::RuntimeError(m) => mlua::Error::RuntimeError(m.replacen("bite:", &format!("{verb}:"), 1)),
        e => e,
    };
    bath_of(o, verb).map_err(named).and_then(|b| {
        if !bite {
            return Ok((b, None));
        }
        let minutes = num(o, "minutes")?.ok_or_else(|| mlua::Error::runtime(format!("bite: minutes= (in the bath)\n{}", help("bite"))))? as f64;
        if !(minutes > 0.0 && minutes <= 24.0 * 60.0) {
            return err("bite: minutes is the time in the bath, over 0 and at most a day (for longer, k:immerse{...} and k:lift() when it comes out)");
        }
        Ok((b, Some(minutes)))
    })
}

fn bath_of(o: &Table, verb: &str) -> Result<BathSpec> {
    let acid = match o.get::<Option<String>>("acid")?.as_deref() {
        Some("ferric") => Acid::Ferric,
        Some("nitric") => Acid::Nitric,
        Some("dutch") => Acid::Dutch,
        Some(a) => return err(format!("bite: acid={a:?}: the box has ferric chloride (\"ferric\"), nitric acid (\"nitric\") and Dutch mordant (\"dutch\")")),
        None => return err(format!("bite: acid=\n{}", help(verb))),
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
    Ok(BathSpec { acid, molar, temp_c: temp, face, rock, feather, fresh })
}

/// The etching tools, a plate's methods (plate.rs `methods`).
pub(crate) fn methods<M: UserDataMethods<SurfaceU>>(m: &mut M) {
    // k:ground("hard", pts, {um=, roller=, speed=}): a hard ground rolled along pts
    m.add_method("ground", |lua, k, (kind, pts, o): (Option<String>, Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "ground")?;
        match kind.as_deref() {
            Some("hard") => {}
            Some("soft") => return err("ground: soft ground is its own switch, later (--@ soft-ground)"),
            _ => return err(format!("ground: {GROUND_HOW}")),
        }
        if let Some(o) = &o {
            check_keys(o, &["um", "roller", "speed"], "ground")?;
        }
        let pts = hand_path(&st, k, "ground", GROUND_HOW, &pts, o.as_ref(), ("pressure", 0.5, 0.0, 1.0), etch::ROLL_MM_S as f32, 2)?;
        let roller = opt(o.as_ref(), "roller", etch::ROLLER_MM as f32, 20.0, 300.0, "the roller's length in mm (a ground roller 100–200)")?;
        let had = with_plate(&st, |p| p.ground_um())?;
        let um = opt(o.as_ref(), "um", if had > 0.0 { had } else { etch::GROUND_UM as f32 }, 1.0, 30.0, "the ground's thickness in µm (2–8 for a hard ground rolled thin)")?;
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, move |s| {
            let p = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            // (more ground rolled onto a plate that holds only ground: the same ground on more of it)
            if p.coated() && !p.only_ground() {
                return Err(mlua::Error::runtime("ground: the plate has varnish, rosin, needle lines or the acid's work on it: clean it first (k:clean({{x, y}, ...}))"));
            }
            if p.only_ground() && p.ground_um() != um {
                return Err(mlua::Error::runtime(format!("ground: the plate's ground is {} µm: roll more of the same (um={}), or clean it first (k:clean({{{{x, y}}, ...}}))", fmt_num(p.ground_um()), fmt_num(p.ground_um()))));
            }
            let r = p.apply(Op::Ground { um, roller_mm: roller, pts });
            let secs = r.secs + etch::GROUND_WARM_S;
            let share = 100.0 * p.ground_share();
            s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.tally_mut().secs += secs;
            Ok((vec![um as f64, share, r.length_mm, secs], false, false))
        })?;
        table(lua, &["um", "covered", "length", "seconds"], &r)
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
    // k:stop_out(pts, {brush=, load=, pressure=, speed=}): stopping-out varnish brushed along pts
    m.add_method("stop_out", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "stop_out")?;
        if let Some(o) = &o {
            check_keys(o, &["brush", "load", "pressure", "speed"], "stop_out")?;
        }
        let pts = hand_path(&st, k, "stop_out", STOP_HOW, &pts, o.as_ref(), ("pressure", 0.5, 0.01, 1.0), etch::BRUSH_MM_S as f32, 1)?;
        let brush = opt(o.as_ref(), "brush", etch::STOP_BRUSH_MM as f32, 0.5, 30.0, "the round brush's width in mm (a size 6 sable 3.9)")?;
        let load = opt(o.as_ref(), "load", 1.0, 0.01, 1.0, "how full its belly is")?;
        let plan = Plate::stopping(brush as f64, load as f64, &pts);
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, move |s| {
            let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            let before = stopped_cells(plate);
            let r = plate.apply(Op::StopOut { brush_mm: brush, load, pts });
            let mm = plate.width_mm as f64 / plate.size_px().0 as f64;
            let area = (stopped_cells(plate) - before) as f64 * mm * mm;
            s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.tally_mut().secs += r.secs;
            Ok((vec![area, r.length_mm, plan.width_mm, plan.spread_mm * 1000.0, plan.left_mm3, r.secs], plan.dry, false))
        })?;
        if r.1 {
            st.borrow_mut().out.push_str(&format!("stop_out: the brush ran dry {} mm into the {} mm stroke: dip it again (another k:stop_out) for the rest\n", fmt_num(r.0[1].round() as f32), fmt_num(plan.total_mm.round() as f32)));
        }
        let t = table(lua, &["area", "length", "width", "edge", "left", "seconds"], &r)?;
        t.set("dry", r.1)?;
        Ok(t)
    });
    // k:immerse{...}: the plate into its tray, to bite on the clock until k:lift()
    m.add_method("immerse", |lua, k, o: Option<Table>| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "immerse")?;
        let o = o.ok_or_else(|| mlua::Error::runtime(help("immerse")))?;
        let (bath, _) = bath_spec(&o, "immerse")?;
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, move |s| {
            let now = plate_clock(s)?;
            let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            let im = plate.immerse(bath, now).map_err(|e| mlua::Error::runtime(format!("immerse: {e}")))?;
            let t = im.tray;
            Ok((vec![t.molar(plate.metal), t.metal_mol * etch::molar(plate.metal).0 * 1000.0 / t.litres, t.litres, now], im.washed, im.fresh))
        })?;
        st.borrow_mut().out.push_str(&format!("immerse: {:?} into {} ({}){}\n", k.name, bath_words(&bath), if r.2 { "a fresh bath" } else { "the tray it had" }, if r.1 { "; its loose rosin floated off (k:melt() first)" } else { "" }));
        let t = table(lua, &["molar", "loaded", "litres", "since"], &r)?;
        t.set("washed", r.1)?;
        t.set("fresh", r.2)?;
        Ok(t)
    });
    // k:lift(): the plate out of its tray, rinsed and dried: it has bitten since k:immerse{}
    m.add_method("lift", |lua, k, (): ()| {
        let st = crate::api::studio(lua)?;
        etching_in_bath(&st, k, "lift")?;
        let thick = with_plate(&st, |p| p.thickness_mm as f64 * 1000.0)?;
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, |s| {
            let now = plate_clock(s)?;
            let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            let r = plate.lift(now).map_err(|e| mlua::Error::runtime(format!("lift: {e} (k:immerse{{...}} puts it in)")))?;
            s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.tally_mut().secs += LIFT_S;
            Ok(bite_rep(&r))
        })?;
        lift_words(&st, thick, &r);
        let t = table(lua, &BITE_NAMES, &r)?;
        t.set("washed", r.1)?;
        t.set("spent", r.2)?;
        Ok(t)
    });
    // k:bite{..., minutes=}: in, minutes of the shared clock, out
    m.add_method("bite", |lua, k, o: Option<Table>| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "bite")?;
        let o = o.ok_or_else(|| mlua::Error::runtime(help("bite")))?;
        let (bath, minutes) = bath_spec(&o, "bite")?;
        let minutes = minutes.expect("a bite's minutes");
        let thick = with_plate(&st, |p| p.thickness_mm as f64 * 1000.0)?;
        // (k:immerse, wait, k:lift: the same steps on the plate and its clock)
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, move |s| {
            let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
            let washed = plate.immerse(bath, c.clock()).map_err(|e| mlua::Error::runtime(format!("bite: {e}")))?.washed;
            c.wait(minutes as f32);
            let r = plate.lift(c.clock()).map_err(|e| mlua::Error::runtime(format!("bite: {e}")))?;
            c.tally_mut().secs += LIFT_S;
            Ok(bite_rep(&paint::plate::etched::BiteReport { washed, ..r }))
        })?;
        lift_words(&st, thick, &r);
        let t = table(lua, &BITE_NAMES, &r)?;
        t.set("washed", r.1)?;
        t.set("spent", r.2)?;
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
                return Err("aquatint: the plate has a ground: the rosin goes on bare metal (k:clean({{x, y}, ...}) first)".into());
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
    // k:clean(pts, {passes=, width=, speed=}): a solvent rag rubbed along the points
    m.add_method("clean", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        etching(&st, k, "clean")?;
        if let Some(o) = &o {
            check_keys(o, &["passes", "width", "speed"], "clean")?;
        }
        let pts = hand_path(&st, k, "clean", CLEAN_HOW, &pts, o.as_ref(), ("pressure", 0.5, 0.0, 1.0), RAG_MM_S, 2)?;
        let passes = crate::plate::int_opt(o.as_ref(), "passes", 1, 50)?;
        let pad = opt(o.as_ref(), "width", paint::rag::PAD_MM, 5.0, 200.0, "the rag's pad in mm")?;
        let r = crate::time::verb_plate(&st, crate::time::Verb::Pass, move |s| {
            let p = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
            let cell_mm2 = p.pitch_um() * p.pitch_um() * 1e-6;
            let had = p.coat_cells();
            let r = p.apply(Op::Wash { pad_mm: pad, passes, pts });
            let left = p.coat_cells();
            s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.tally_mut().secs += r.secs;
            Ok((vec![had.saturating_sub(left) as f64 * cell_mm2, left as f64 * cell_mm2, r.length_mm, r.secs], false, false))
        })?;
        table(lua, &["cleaned", "coated", "length", "seconds"], &r)
    });
}

/// A coated cell as a fingernail finds it: words, and (ground µm, rosin's share).
pub(crate) fn feel(p: &Plate, x: usize, y: usize) -> (String, Vec<f64>) {
    let e = p.etch(x, y);
    let ground = p.ground_at(x, y);
    let rosin = p.rosin_cover(x, y);
    let mut words = Vec::new();
    if e.stopped {
        words.push(if e.stop_taken > 0.0 { format!("varnished, thinned to {} µm", fmt_num(((etch::STOP_UM as f32 - e.stop_taken) * 10.0).round() / 10.0)) } else { "varnished".to_string() });
    }
    if ground > 0.0 {
        let bared = e.lines.iter().any(|l| l.lam > 0.0);
        let thinned = if e.ground_taken > 0.0 { format!(" (thinned from {} µm)", fmt_num(p.ground_um())) } else { String::new() };
        words.push(format!("hard ground {} µm{thinned}{}", fmt_num((ground as f32 * 100.0).round() / 100.0), if bared { ", a needle line through it" } else { "" }));
    }
    if p.pinholes_at(x, y).is_some() {
        words.push(format!("pinholed over {}% of it", fmt_num((1000.0 * etch::bared_share(p.coat_um(x, y))).round() as f32 / 10.0)));
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
        let laid = (y0..y1).flat_map(|y| (x0..x1).map(move |x| (x, y))).filter(|&(x, y)| p.grounded(x, y)).count() as f64;
        out.push_str(&format!("hard ground {} µm over {}%\n", fmt_num(p.ground_um()), (100.0 * laid / n).round()));
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
