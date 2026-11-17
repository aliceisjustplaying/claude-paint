//! Ink, inking and wiping a plate (`--@ press`; notes/plan-intaglio.md §5, phase P3b).
//!
//! - `black = ink{{"bone black", 3}, {"raw umber", 1}, oil=0.1, stiff=1, name=}`: an etching
//!   ink, tubes from the box ground in burnt plate oil (`oil`: plate oil added, its share of
//!   the ink; `stiff`: the plate oil's grade, 0 light #00 .. 1 heavy #3).
//! - `k:warm(40)`: the plate on the hot plate at 40 °C; it cools in air from then.
//! - `k:ink(black, pts, {by="card" | "dabber" | "poupee", width=, force=, speed=, dab=})`: ink
//!   pushed into the plate's lines and pits and a film left on its flat, where the card's edge is
//!   dragged along the points or a dabber or poupée is dabbed at them (or rubbed along them).
//! - `t = tarlatan{stiff=0.7, width=40, seed=}` and `t:wipe(k, pts, {pressure=, passes=,
//!   speed=})`: starched tarlatan bunched into a pad and wiped along the points (circles,
//!   strokes: whatever they trace); `t:refold()`.
//! - `k:hand_wipe(pts, {pressure=, passes=, whiting=, speed=})`: the palm drawn along the points.
//! - `k:retrousse(pts, {passes=, width=, speed=})`: a soft muslin passed lightly along the points.
//!
//! Every one of them acts only where its tool goes: none takes a mask, a function or a region.
//! - `k:clean_ink()`: solvent and a rag: all the ink off.
//!
//! The physics is paint's (`plate::inked`; notes/research/intaglio_physics.md §5): the ink is
//! an oil film that dries on the shared clock; the tarlatan is a rag (`paint::rag::Rag`) whose
//! cloth reaches into a plate's lines by its yarns and its sag, held off past a burr. The
//! tarlatans are held with the rags (`Studio::rags`), so a failed chunk puts them back and a
//! replay checkpoint checks them as it does the rags. The plate's ink goes in its section
//! (`PLAT` version 3) through saves, checkpoints and handoffs.

use crate::api::{S, check_keys, err, fmt_num, num};
use crate::plate::{Rep, hand_path, in_hand};
use crate::surfaces::{Kind, SurfaceU};
use crate::time::{self, Verb};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::plate::Plate;
use paint::plate::inked::{Ink, InkTool, Inker, WipeReport, pace};
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
/// The card's speed when none is given (mm/s): the card's slider law's (`inked::CARD_MM_S`).
const CARD_SPEED: f32 = paint::plate::inked::CARD_MM_S as f32;
/// A tarlatan pad's and the palm's speed when none is given (mm/s) [E: a wiping hand moves
/// briskly; the hand time is the area passed over at Cassatt's pace, `inked::pace`].
const TARLATAN_MM_S: f32 = 100.0;
const PALM_SPEED: f32 = 100.0;

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

const INK_HOW: &str = "k:ink(black, {{x, y}, ...}, {by=\"card\" | \"dabber\" | \"poupee\", width=<mm>, force=<N>, speed=, dab=true | false}): where the card is dragged, or the dabber or poupée dabbed";
const WIPE_HOW: &str = "t:wipe(k, {{x, y}, ...}, {pressure=, passes=, speed=}): the pad's path, in circles or strokes as the points go";
const PALM_HOW: &str = "k:hand_wipe({{x, y}, ...}, {pressure=, passes=, whiting=, speed=}): the palm's path";
const RETRO_HOW: &str = "k:retrousse({{x, y}, ...}, {passes=, width=, speed=}): the muslin's path";

/// Options a hand's tool no longer takes, refused with what to do instead.
fn refuse_gone(o: Option<&Table>, verb: &str, how: &str) -> Result<()> {
    let Some(o) = o else { return Ok(()) };
    if !matches!(o.get::<Value>("mask")?, Value::Nil) {
        return err(format!("{verb}: it takes no mask: a hand's tool acts only where it goes, so give the points it is moved along: {how}"));
    }
    if !matches!(o.get::<Value>("angle")?, Value::Nil) {
        return err(format!("{verb}: no angle: the hand goes the way the points go, so give points back and forth (or round in circles) where it should wipe: {how}"));
    }
    Ok(())
}

/// The ink tool `by=` names, at `width=` mm (or its usual size), dabbed or (`dab=false`)
/// rubbed along.
fn ink_tool(o: Option<&Table>) -> Result<InkTool> {
    let by = match o.map(|o| o.get::<Option<String>>("by")).transpose()?.flatten().as_deref() {
        None | Some("card") => Inker::Card,
        Some("dabber") => Inker::Dabber,
        Some("poupee") | Some("poupée") => Inker::Poupee,
        Some(b) => return err(format!("ink: by={b:?}: \"card\", \"dabber\" or \"poupee\"")),
    };
    let mut t = InkTool::of(by);
    if let Some(w) = o.map(|o| num(o, "width")).transpose()?.flatten() {
        if !(3.0..=300.0).contains(&w) {
            return err("ink: width is the card's edge or the face's diameter in mm, 3 to 300");
        }
        t.width_mm = w as f64;
    }
    match o.map(|o| o.get::<Option<bool>>("dab")).transpose()?.flatten() {
        None => {}
        Some(_) if by == Inker::Card => return err("ink: a card is dragged along its points; dab= is for a dabber or a poupée"),
        Some(d) => t.moved = !d,
    }
    Ok(t)
}

fn passes(o: Option<&Table>, default: u32) -> Result<u32> {
    let v = o.map(|o| num(o, "passes")).transpose()?.flatten().unwrap_or(default as f32);
    if v.fract() != 0.0 || !(1.0..=50.0).contains(&v) {
        return err("passes: a whole number, 1 to 50");
    }
    Ok(v as u32)
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
        // t:wipe(k, pts, {pressure=, passes=, speed=}): the pad along pts over the plate in hand
        m.add_method("wipe", |lua, t, (k, pts, o): (Value, Value, Option<Table>)| {
            let Value::UserData(k) = k else { return err(format!("wipe: the plate first: {WIPE_HOW}")) };
            let k = k.borrow::<SurfaceU>().map_err(|_| mlua::Error::runtime(format!("wipe: the plate first: {WIPE_HOW}")))?.clone();
            pressing(&t.st, &k, "wipe")?;
            refuse_gone(o.as_ref(), "wipe", WIPE_HOW)?;
            if let Some(o) = &o {
                check_keys(o, &["pressure", "passes", "speed"], "wipe")?;
            }
            let pts = hand_path(&t.st, &k, "wipe", WIPE_HOW, &pts, o.as_ref(), ("pressure", 0.5, 0.0, 1.0), TARLATAN_MM_S, 2)?;
            let n = passes(o.as_ref(), 1)?;
            let rag = t.rag.clone();
            let r = work(&t.st, move |p, now| {
                if !p.inked() {
                    return Err("wipe: the plate has no ink on it (k:ink(...) first)".into());
                }
                Ok(wipe_rep(&p.wipe(&mut rag.borrow_mut(), &pts, n, now)))
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
    // k:ink(black, pts, {by=, width=, force=, speed=, dab=}): ink where the card or dabber goes
    m.add_method("ink", |lua, k, (i, pts, o): (Value, Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "ink")?;
        let Value::UserData(i) = i else { return err(format!("ink: the ink first: {INK_HOW}, black = {INK_HELP}")) };
        let ink = i.borrow::<InkU>().map_err(|_| mlua::Error::runtime(format!("ink: the ink first: black = {INK_HELP}")))?.0.clone();
        // (the old k:ink(black, {mask=, by=}): its options where the points go)
        if let Value::Table(t) = &pts {
            refuse_gone(Some(t), "ink", INK_HOW)?;
        }
        refuse_gone(o.as_ref(), "ink", INK_HOW)?;
        if let Some(o) = &o {
            check_keys(o, &["by", "width", "force", "speed", "dab"], "ink")?;
        }
        let tool = ink_tool(o.as_ref())?;
        let least = if tool.by == Inker::Card || tool.moved { 2 } else { 1 };
        let pts = hand_path(&st, k, "ink", INK_HOW, &pts, o.as_ref(), ("force", tool.force() as f32, 0.1, 100.0), CARD_SPEED, least)?;
        let r = work(&st, move |p, now| {
            if p.coated() {
                return Err("ink: the plate has a ground, varnish or rosin on it: clean it first (k:clean())".into());
            }
            let r = p.ink(&ink, tool, &pts, now)?;
            Ok(((vec![r.added_um3 * 1e-9, r.film_um, r.fill, r.temp_c, r.eta, r.area_mm2, r.secs], false, false), r.secs))
        })?;
        table(lua, &["added", "film", "fill", "temp", "viscosity", "area", "seconds"], &r)
    });
    // k:hand_wipe(pts, {pressure=, passes=, whiting=, speed=}): the palm along pts
    m.add_method("hand_wipe", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "hand_wipe")?;
        refuse_gone(o.as_ref(), "hand_wipe", PALM_HOW)?;
        if let Some(o) = &o {
            check_keys(o, &["pressure", "passes", "whiting", "speed"], "hand_wipe")?;
        }
        let pts = hand_path(&st, k, "hand_wipe", PALM_HOW, &pts, o.as_ref(), ("pressure", 0.5, 0.0, 1.0), PALM_SPEED, 2)?;
        let n = passes(o.as_ref(), 1)?;
        let whiting = o.as_ref().map(|o| o.get::<Option<bool>>("whiting")).transpose()?.flatten().unwrap_or(true);
        let r = work(&st, move |p, now| {
            if !p.inked() {
                return Err("hand_wipe: the plate has no ink on it (k:ink(...) first)".into());
            }
            Ok(wipe_rep(&p.hand_wipe(&pts, n, whiting, now)))
        })?;
        table(lua, &WIPE_NAMES, &r)
    });
    // k:retrousse(pts, {passes=, width=, speed=}): a soft cloth along pts drawing ink up beside the lines
    m.add_method("retrousse", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        pressing(&st, k, "retrousse")?;
        refuse_gone(o.as_ref(), "retrousse", RETRO_HOW)?;
        if let Some(o) = &o {
            check_keys(o, &["passes", "width", "speed"], "retrousse")?;
        }
        let pts = hand_path(&st, k, "retrousse", RETRO_HOW, &pts, o.as_ref(), ("pressure", 0.5, 0.0, 1.0), PALM_SPEED, 2)?;
        let n = passes(o.as_ref(), 1)?;
        let width = crate::plate::opt(o.as_ref(), "width", PAD_MM, 5.0, 200.0, "the muslin pad's width in mm")? as f64;
        let r = work(&st, move |p, now| {
            if !p.inked() {
                return Err("retrousse: the plate has no ink on it (k:ink(...) first)".into());
            }
            Ok(wipe_rep(&p.retrousse(&pts, width, n, now)))
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
