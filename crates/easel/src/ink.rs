//! Ink, inking and wiping a plate (`--@ press`; notes/plan-intaglio.md §5, phase P3b).
//!
//! - `black = ink{{"bone black", 3}, {"raw umber", 1}, oil=0.1, stiff=1, name=}`: an etching
//!   ink, tubes from the box ground in burnt plate oil (`oil`: plate oil added, its share of
//!   the ink; `stiff`: the plate oil's grade, 0 light #00 .. 1 heavy #3).
//! - `k:warm(40)`: the plate on the hot plate at 40 °C; it cools in air from then.
//! - `k:ink(black, {mask=, by="card" | "dabber"})`: ink pushed into the plate's lines and pits
//!   and a film left on its flat.
//! - `t = tarlatan{stiff=0.7, width=40, seed=}` and `t:wipe(k, mask, {pressure=, passes=,
//!   angle=})`: starched tarlatan bunched into a pad, wiped in circles (or back and forth at
//!   `angle` degrees) over the plate; `t:refold()`.
//! - `k:hand_wipe(mask, {pressure=, passes=, whiting=, angle=})`: the palm.
//! - `k:retrousse(mask, {passes=})`: a soft muslin passed lightly over the wiped plate.
//! - `k:clean_ink()`: solvent and a rag: all the ink off.
//!
//! The physics is paint's (`plate::inked`; notes/research/intaglio_physics.md §5): the ink is
//! an oil film that dries on the shared clock; the tarlatan is a rag (`paint::rag::Rag`) whose
//! cloth reaches into a plate's lines by its yarns and its sag, held off past a burr. The
//! tarlatans are held with the rags (`Studio::rags`), so a failed chunk puts them back and a
//! replay checkpoint checks them as it does the rags. The plate's ink goes in its section
//! (`PLAT` version 3) through saves, checkpoints and handoffs.

use crate::api::{S, check_keys, err, fmt_num, num};
use crate::plate::{Rep, in_hand};
use crate::surfaces::{Kind, SurfaceU};
use crate::time::{self, Verb};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::plate::Plate;
use paint::plate::inked::{Ink, Inker, Pattern, WipeReport, pace};
use paint::rag::Rag;
use std::cell::RefCell;
use std::rc::Rc;

const INK_HELP: &str = "ink{{\"<tube>\", <parts>}, ..., oil=<plate oil added, share of the ink, 0 to 0.6>, stiff=<the plate oil: 0 light (#00) to 1 heavy (#3)>, name=}";
const TARLATAN_HELP: &str = "tarlatan{stiff=<starch: 0 soft to 1 stiff>, width=<the pad, mm>, seed=}";
/// A tarlatan pad's width when none is given (mm): a rag's pad (`rag::PAD_MM`), a cloth bunched
/// in the hand.
const PAD_MM: f32 = paint::rag::PAD_MM;
/// A tarlatan's starch when none is given: well starched, as sold [E: Takach sells "stiff"
/// and "regular"; the printer softens it before use].
const STIFF: f32 = 0.7;

/// An ink in the hand, as `ink{}` mixed it.
#[derive(Clone)]
pub struct InkU(pub Rc<Ink>);

impl UserData for InkU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("name", |_, i| Ok(i.0.name.clone()));
        f.add_field_method_get("pigment", |_, i| Ok(i.0.pvc));
        f.add_field_method_get("viscosity", |_, i| Ok(i.0.eta(20.0)));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::ToString, |_, i, ()| Ok(format!("ink {:?} ({} Pa·s at 20 °C, pigment {}% of it)", i.0.name, fmt_num(i.0.eta(20.0) as f32), (i.0.pvc * 100.0).round())));
    }
}

/// A tarlatan in the hand: a rag the studio holds with its rags.
pub struct TarlatanU {
    rag: Rc<RefCell<Rag>>,
    st: S,
}

/// The press switch's verbs: `ink{}` and `tarlatan{}` (the plate's are its methods: `methods`).
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    {
        let st = st.clone();
        g.set("ink", lua.create_function(move |_, o: Option<Table>| make_ink(&st, o))?)?;
    }
    g.set("tarlatan", lua.create_function(move |lua, o: Option<Table>| {
        if let Some(o) = &o {
            check_keys(o, &["stiff", "width", "seed"], "tarlatan")?;
        }
        let stiff = o.as_ref().map(|o| num(o, "stiff")).transpose()?.flatten().unwrap_or(STIFF);
        if !(0.0..=1.0).contains(&stiff) {
            return err(format!("tarlatan: stiff is its starch, 0 soft to 1 stiff\n{TARLATAN_HELP}"));
        }
        let width = o.as_ref().map(|o| num(o, "width")).transpose()?.flatten().unwrap_or(PAD_MM);
        if !(5.0..=200.0).contains(&width) {
            return err(format!("tarlatan: width is the pad's in mm, 5 to 200\n{TARLATAN_HELP}"));
        }
        let sd = match &o {
            Some(o) => crate::api::seed_of(&st, o)?,
            None => crate::api::seed_of(&st, &lua.create_table()?)?,
        };
        let rag = Rc::new(RefCell::new(Rag::tarlatan(width, sd, stiff)));
        st.borrow_mut().rags.push(Rc::downgrade(&rag));
        crate::replay_checkpoint::hold_tools(&st);
        time::unmarked_verb_plate(&st, Verb::Marks, |s| {
            s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.tally_mut().secs += paint::rag::pace::FRESH;
            Ok(())
        })?;
        Ok(TarlatanU { rag, st: st.clone() })
    })?)?;
    Ok(())
}

fn make_ink(st: &S, o: Option<Table>) -> Result<InkU> {
    let o = o.ok_or_else(|| mlua::Error::runtime(INK_HELP))?;
    let mut tubes = Vec::new();
    for v in o.clone().sequence_values::<Value>() {
        let Value::Table(t) = v? else { return err(format!("ink: each part is {{\"<tube>\", <parts>}}\n{INK_HELP}")) };
        let name: String = t.get(1).map_err(|_| mlua::Error::runtime(format!("ink: a part's tube is a name\n{INK_HELP}")))?;
        let parts: f32 = t.get(2).map_err(|_| mlua::Error::runtime(format!("ink: a part's parts are a number\n{INK_HELP}")))?;
        if !(parts > 0.0 && parts <= 1000.0) {
            return err("ink: parts are over 0");
        }
        tubes.push((name, parts));
    }
    for pair in o.clone().pairs::<Value, Value>() {
        let (k, _) = pair?;
        match k {
            Value::Integer(_) => {}
            Value::String(s) if ["oil", "stiff", "name"].contains(&&*s.to_str()?) => {}
            k => return err(format!("ink: unknown key {}\n{INK_HELP}", k.to_string().unwrap_or_default())),
        }
    }
    if tubes.is_empty() {
        return err(format!("ink: name its tubes\n{INK_HELP}"));
    }
    if tubes.len() > 8 {
        return err("ink: at most 8 tubes in an ink");
    }
    let oil = num(&o, "oil")?.unwrap_or(0.0);
    if !(0.0..=0.6).contains(&oil) {
        return err(format!("ink: oil is the plate oil added, its share of the ink, 0 to 0.6\n{INK_HELP}"));
    }
    let stiff = num(&o, "stiff")?.unwrap_or(1.0);
    if !(0.0..=1.0).contains(&stiff) {
        return err(format!("ink: stiff is the plate oil's grade, 0 light (#00) to 1 heavy (#3)\n{INK_HELP}"));
    }
    let name = o.get::<Option<String>>("name")?.unwrap_or_else(|| tubes.iter().map(|t| t.0.clone()).collect::<Vec<_>>().join(" and "));
    let tubes_box = st.borrow().tubes.clone();
    let ink = Ink::of_tubes(&tubes_box, &tubes, oil, stiff, name).map_err(|e| mlua::Error::runtime(format!("ink: {e}")))?;
    Ok(InkU(Rc::new(ink)))
}

/// The plate `k` names, in hand, with the switch.
fn pressing(st: &S, k: &SurfaceU, verb: &str) -> Result<(f32, f32)> {
    if k.kind == Kind::Plate && !st.borrow().tubes.press {
        return err(format!("{verb}: a printer's verb, and this painting has no ink or press (its log's head has no {:?} line)", crate::session::PRESS_MARK));
    }
    in_hand(st, k, verb)
}

/// A plate's tools refuse an inked plate: the printer cleans it first.
pub(crate) fn refuse_inked(p: &Plate) -> std::result::Result<(), String> {
    if p.inked() { Err("the plate is inked: clean it first (k:clean_ink())".into()) } else { Ok(()) }
}

/// The mask `v` names as the plate's raster (one value a cell), or None (the whole plate).
fn plate_mask(st: &S, v: &Value, verb: &str) -> Result<Option<Vec<f32>>> {
    if let Value::Nil = v {
        return Ok(None);
    }
    let m = crate::api::mask_of(v)?;
    let (w, h) = st.borrow().plate.as_ref().map(|p| p.size_px()).ok_or_else(|| mlua::Error::runtime("no plate in hand"))?;
    if (m.f.w, m.f.h) != (w, h) {
        return err(format!("{verb}: the mask isn't the plate's: make it while the plate is in hand"));
    }
    Ok(Some(m.data.clone()))
}

/// The pad's way: in circles, or back and forth at `angle` degrees.
fn pattern(o: Option<&Table>) -> Result<Pattern> {
    match o.map(|o| num(o, "angle")).transpose()?.flatten() {
        None => Ok(Pattern::Circles),
        Some(a) if a.is_finite() => Ok(Pattern::Along(a)),
        Some(_) => err("angle: degrees, the way the strokes go (0 to the right, 90 down the plate)"),
    }
}

fn passes(o: Option<&Table>, default: u32) -> Result<u32> {
    let v = o.map(|o| num(o, "passes")).transpose()?.flatten().unwrap_or(default as f32);
    if v.fract() != 0.0 || !(1.0..=50.0).contains(&v) {
        return err("passes: a whole number, 1 to 50");
    }
    Ok(v as u32)
}

fn pressure(o: Option<&Table>) -> Result<f32> {
    let v = o.map(|o| num(o, "pressure")).transpose()?.flatten().unwrap_or(0.5);
    if !(0.0..=1.0).contains(&v) {
        return err("pressure: 0 to 1 (0.5 a working wipe)");
    }
    Ok(v)
}

/// The plate in hand, worked on: `f` gets the plate and the clock now (minutes), and the hand
/// time it returns goes on the plate's clock.
fn work(st: &S, f: impl FnOnce(&mut Plate, f64) -> std::result::Result<(Rep, f64), String>) -> Result<Rep> {
    time::verb_plate(st, Verb::Pass, move |s| {
        let now = s.canvas.as_ref().ok_or_else(crate::api::no_canvas)?.now_min();
        let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
        let (r, secs) = f(plate, now).map_err(mlua::Error::runtime)?;
        s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.tally_mut().secs += secs;
        Ok(r)
    })
}

const WIPE_NAMES: [&str; 9] = ["before", "after", "taken", "laid", "kept", "film", "lines", "seconds", "temp"];

/// A wipe's report as it is recorded (mm³ and µm), and its hand time.
fn wipe_rep(r: &WipeReport) -> (Rep, f64) {
    let mm3 = |v: f64| v * 1e-9;
    ((vec![mm3(r.before_um3), mm3(r.after_um3), mm3(r.taken_um3), mm3(r.laid_um3), mm3(r.kept_um3), r.film_um, r.lines_um, r.secs, r.temp_c], false, false), r.secs)
}

fn table(lua: &Lua, names: &[&str], r: &Rep) -> Result<Table> {
    let t = lua.create_table()?;
    for (k, v) in names.iter().zip(&r.0) {
        t.set(*k, *v)?;
    }
    Ok(t)
}

impl UserData for TarlatanU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("width", |_, t| Ok(t.rag.borrow().width));
        f.add_field_method_get("load", |_, t| Ok(t.rag.borrow().load));
        f.add_field_method_get("soaked", |_, t| Ok(t.rag.borrow().soaked));
        f.add_field_method_get("fold", |_, t| Ok(t.rag.borrow().fold));
        f.add_field_method_get("stiff", |_, t| Ok(t.rag.borrow().cloth.as_ref().map_or(STIFF, |c| c.stiff)));
        // the ink it holds, mm³
        f.add_field_method_get("ink", |_, t| Ok(t.rag.borrow().cloth.as_ref().map_or(0.0, |c| c.held_um3 * 1e-9)));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::NewIndex, |_, _, (k, _): (Value, Value)| -> Result<()> {
            let k = k.to_string().unwrap_or_else(|_| "that".into());
            err(format!("a tarlatan's {k} can't be set: the cloth changes only as it is used; t:refold() turns a cleaner part outward, tarlatan{{}} takes a fresh one"))
        });
        m.add_meta_method(MetaMethod::ToString, |_, t, ()| {
            let r = t.rag.borrow();
            Ok(format!("tarlatan(width {:.0} mm, load {:.2})", r.width, r.load))
        });
        // t:wipe(k, mask, {pressure=, passes=, angle=}): the pad over the plate in hand
        m.add_method("wipe", |lua, t, (k, mask, o): (Value, Value, Option<Table>)| {
            let Value::UserData(k) = k else { return err("wipe: the plate first: t:wipe(k, mask, {...})") };
            let k = k.borrow::<SurfaceU>().map_err(|_| mlua::Error::runtime("wipe: the plate first: t:wipe(k, mask, {...})"))?.clone();
            pressing(&t.st, &k, "wipe")?;
            if let Some(o) = &o {
                check_keys(o, &["pressure", "passes", "angle"], "wipe")?;
            }
            let mask = plate_mask(&t.st, &mask, "wipe")?;
            let (pr, n, pat) = (pressure(o.as_ref())?, passes(o.as_ref(), 3)?, pattern(o.as_ref())?);
            let rag = t.rag.clone();
            let r = work(&t.st, move |p, now| {
                if !p.inked() {
                    return Err("wipe: the plate has no ink on it (k:ink(...) first)".into());
                }
                Ok(wipe_rep(&p.wipe(&mut rag.borrow_mut(), mask.as_deref(), pr, n, pat, now)))
            })?;
            if t.rag.borrow().load >= paint::rag::FULL {
                t.st.borrow_mut().out.push_str("tarlatan: the face in use is full of ink; t:refold() turns out a cleaner one\n");
            }
            table(lua, &WIPE_NAMES, &r)
        });
        // t:refold(): a cleaner face outward
        m.add_method("refold", |_, t, ()| {
            time::unmarked_verb_plate(&t.st, Verb::Marks, |s| {
                let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
                let mut r = t.rag.borrow_mut();
                r.refold(c.tally_mut());
                if let Some(cl) = r.cloth.as_mut() {
                    // (what was wet at the old face is folded inside)
                    cl.pool_um3 = 0.0;
                }
                Ok(())
            })
        });
    }
}

/// The press switch's methods on a plate (plate.rs `methods`).
pub(crate) fn methods<M: UserDataMethods<SurfaceU>>(m: &mut M) {
    // k:warm(temp): on the hot plate at temp °C
    m.add_method("warm", |lua, k, temp: f64| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "warm")?;
        if !(20.0..=90.0).contains(&temp) {
            return err("warm: the hot plate's °C, 20 to 90 (printers ink at 35–50)");
        }
        let r = work(&st, move |p, now| {
            // (the time on the hot plate passes first; it comes off at the end)
            p.warm(temp, now + pace::WARM_S / 60.0);
            Ok(((vec![temp, pace::WARM_S], false, false), pace::WARM_S))
        })?;
        table(lua, &["temp", "seconds"], &r)
    });
    // k:ink(black, {mask=, by=}): ink into the lines, a film on the flat
    m.add_method("ink", |lua, k, (i, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "ink")?;
        let Value::UserData(i) = i else { return err(format!("ink: the ink first: k:ink(black, {{mask=, by=}}), black = {INK_HELP}")) };
        let ink = i.borrow::<InkU>().map_err(|_| mlua::Error::runtime(format!("ink: the ink first: black = {INK_HELP}")))?.0.clone();
        if let Some(o) = &o {
            check_keys(o, &["mask", "by"], "ink")?;
        }
        let by = match o.as_ref().map(|o| o.get::<Option<String>>("by")).transpose()?.flatten().as_deref() {
            None | Some("card") => Inker::Card,
            Some("dabber") => Inker::Dabber,
            Some("poupee") => return err("ink: by=\"poupee\" (à la poupée) is the registration switch's, later"),
            Some(b) => return err(format!("ink: by={b:?}: \"card\" or \"dabber\"")),
        };
        let mask = match &o {
            Some(o) => plate_mask(&st, &o.get::<Value>("mask")?, "ink")?,
            None => None,
        };
        let r = work(&st, move |p, now| {
            if p.coated() {
                return Err("ink: the plate has a ground, varnish or rosin on it: clean it first (k:clean())".into());
            }
            let r = p.ink(&ink, mask.as_deref(), by, now)?;
            Ok(((vec![r.added_um3 * 1e-9, r.film_um, r.fill, r.temp_c, r.eta, r.area_mm2, r.secs], false, false), r.secs))
        })?;
        table(lua, &["added", "film", "fill", "temp", "viscosity", "area", "seconds"], &r)
    });
    // k:hand_wipe(mask, {pressure=, passes=, whiting=, angle=}): the palm
    m.add_method("hand_wipe", |lua, k, (mask, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "hand_wipe")?;
        if let Some(o) = &o {
            check_keys(o, &["pressure", "passes", "whiting", "angle"], "hand_wipe")?;
        }
        let mask = plate_mask(&st, &mask, "hand_wipe")?;
        let (pr, n, pat) = (pressure(o.as_ref())?, passes(o.as_ref(), 2)?, pattern(o.as_ref())?);
        let whiting = o.as_ref().map(|o| o.get::<Option<bool>>("whiting")).transpose()?.flatten().unwrap_or(true);
        let r = work(&st, move |p, now| {
            if !p.inked() {
                return Err("hand_wipe: the plate has no ink on it (k:ink(...) first)".into());
            }
            Ok(wipe_rep(&p.hand_wipe(mask.as_deref(), pr, n, whiting, pat, now)))
        })?;
        table(lua, &WIPE_NAMES, &r)
    });
    // k:retrousse(mask, {passes=}): a soft cloth drawing ink up beside the lines
    m.add_method("retrousse", |lua, k, (mask, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "retrousse")?;
        if let Some(o) = &o {
            check_keys(o, &["passes"], "retrousse")?;
        }
        let mask = plate_mask(&st, &mask, "retrousse")?;
        let n = passes(o.as_ref(), 1)?;
        let r = work(&st, move |p, now| {
            if !p.inked() {
                return Err("retrousse: the plate has no ink on it (k:ink(...) first)".into());
            }
            Ok(wipe_rep(&p.retrousse(mask.as_deref(), n, now)))
        })?;
        table(lua, &WIPE_NAMES, &r)
    });
    // k:clean_ink(): solvent and a rag
    m.add_method("clean_ink", |lua, k, (): ()| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "clean_ink")?;
        let r = work(&st, |p, _| Ok(((vec![p.clean_ink() * 1e-9, pace::CLEAN_S], false, false), pace::CLEAN_S)))?;
        table(lua, &["taken", "seconds"], &r)
    });
}

/// An inked cell as a fingernail (and the eye) finds it: words, and (ink in its lines µm,
/// film µm, cure).
pub(crate) fn feel(p: &Plate, x: usize, y: usize) -> (String, Vec<f64>) {
    let ic = p.ink_cell(x, y);
    if ic.total() <= 0.0 {
        return (String::new(), vec![0.0, 0.0, 0.0]);
    }
    let set = if p.ink_set(x, y) { "set" } else if ic.cure > 0.0 { "tacky" } else { "wet" };
    (format!("{set} ink, {} µm in the lines and a film of {} µm", fmt_num((ic.lines() * 10.0).round() / 10.0), fmt_num((ic.film * 100.0).round() / 100.0)), vec![ic.lines() as f64, ic.film as f64, ic.cure as f64])
}

/// `probe` on an inked plate over cells (x0, y0, x1, y1): its ink.
pub(crate) fn probe(p: &Plate, r: (usize, usize, usize, usize), now: f64) -> String {
    let Some(k) = p.inking() else { return String::new() };
    let (x0, y0, x1, y1) = r;
    let n = ((x1 - x0) * (y1 - y0)).max(1) as f64;
    let (mut lines, mut film, mut set) = (0.0f64, 0.0f64, 0usize);
    for y in y0..y1 {
        for x in x0..x1 {
            let c = p.ink_cell(x, y);
            lines += c.lines() as f64;
            film += c.film as f64;
            set += p.ink_set(x, y) as usize;
        }
    }
    let names: Vec<&str> = k.inks.iter().map(|i| i.name.as_str()).collect();
    let mut out = format!("ink ({}): {} µm in the lines, a film of {} µm on the flat", names.join(", "), fmt_num((lines / n * 10.0).round() as f32 / 10.0), fmt_num((film / n * 100.0).round() as f32 / 100.0));
    if set > 0 {
        out.push_str(&format!(" · set over {}%", (100.0 * set as f64 / n).round()));
    }
    out.push_str(&format!(" · the plate at {} °C\n", fmt_num(p.temp_c(now).round() as f32)));
    out
}

#[cfg(test)]
mod tests;
