//! The rag in Lua: a soft cloth bunched into a pad, wiped or pressed over
//! open paint to lift it (engine: `paint::rag`).
//!
//! A rag is a userdata over a `Rag` the studio holds (`Studio::rags`), as a
//! brush is over its `Held`: the painter reads its state (`r.load`,
//! `r.soaked`, `r.damp` as evaporated by now, `r.fold`, `r.width`) and can't write it, so a cleaner face
//! costs a refold and a clean cloth a fresh rag, each with its hand time.
//! A failed chunk puts the rags back with the brushes (session.rs), and a
//! replay loads them the same way. The rag needs no extra canvas fields.

use crate::api::{S, check_keys, err, mask_of, num, points, seed_of};
use crate::time::{self, Verb};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::rag::{PAD_MM, Rag, RagPass, pace};
use std::cell::RefCell;
use std::rc::Rc;

const RAG_KEYS: &[&str] = &["width", "seed"];
const WIPE_KEYS: &[&str] = &["pressure", "angle", "passes", "refold", "seed"];
const BLOT_KEYS: &[&str] = &["pressure", "seed"];

fn no_canvas() -> mlua::Error {
    mlua::Error::runtime("no canvas yet: call canvas{} first")
}

/// A rag in the painter's hand.
pub struct RagU {
    rag: Rc<RefCell<Rag>>,
    st: S,
}

fn pressure_ok(p: f32) -> Result<f32> {
    if !p.is_finite() {
        return err("pressure: want a number 0..1");
    }
    Ok(p.clamp(0.0, 1.0))
}

/// `pressure=`: a number, or a list of numbers along the path.
fn pressures(o: Option<&Table>) -> Result<Vec<f32>> {
    let Some(o) = o else { return Ok(vec![0.5]) };
    match o.get::<Value>("pressure")? {
        Value::Nil => Ok(vec![0.5]),
        Value::Integer(n) => Ok(vec![pressure_ok(n as f32)?]),
        Value::Number(n) => Ok(vec![pressure_ok(n as f32)?]),
        Value::Table(t) => {
            let v: Vec<f32> = t.sequence_values::<f32>().collect::<Result<_>>()?;
            if v.is_empty() {
                return err("pressure: a number 0..1 or a list of them along the path");
            }
            v.into_iter().map(pressure_ok).collect()
        }
        o => err(format!("pressure: want a number or a list, got {}", o.type_name())),
    }
}

/// `seed=`, or the chunk's next seed.
fn seed(st: &S, lua: &Lua, o: Option<&Table>) -> Result<u64> {
    match o {
        Some(o) => seed_of(st, o),
        None => seed_of(st, &lua.create_table()?),
    }
}

impl UserData for RagU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        // read only: a write is an error (mlua has no setter for them)
        f.add_field_method_get("width", |_, r| Ok(r.rag.borrow().width));
        f.add_field_method_get("load", |_, r| Ok(r.rag.borrow().load));
        f.add_field_method_get("soaked", |_, r| Ok(r.rag.borrow().soaked));
        f.add_field_method_get("fold", |_, r| Ok(r.rag.borrow().fold));
        // as evaporated by now (rag.rs `evaporate`), without changing the rag
        f.add_field_method_get("damp", |_, r| {
            let now = r.st.borrow().canvas.as_ref().map(|c| c.now_min());
            let g = r.rag.borrow();
            Ok(now.map_or(g.damp, |n| g.damp_at(n)))
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::NewIndex, |_, _, (k, _): (Value, Value)| -> Result<()> {
            let k = k.to_string().unwrap_or_else(|_| "that".into());
            err(format!("a rag's {k} can't be set: the cloth changes only as it is used; r:refold() turns a cleaner part outward, rag() takes a fresh one"))
        });
        m.add_meta_method(MetaMethod::ToString, |_, r, ()| {
            let r = r.rag.borrow();
            Ok(format!("rag(width {:.0}, load {:.2})", r.width, r.load))
        });
        // r:wipe(mask, {pressure=, angle=, passes=, refold=, seed=}) or r:wipe(pts, {pressure=, seed=})
        m.add_method("wipe", |lua, u, (a, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, WIPE_KEYS, "wipe")?;
            }
            let pr = pressures(o.as_ref())?;
            match &a {
                Value::UserData(_) => {
                    let m = mask_of(&a)?;
                    let get = |k: &str| -> Result<Option<f32>> { o.as_ref().map(|o| num(o, k)).transpose().map(Option::flatten) };
                    if pr.len() != 1 {
                        return err("wipe: over a mask, pressure is one number");
                    }
                    let angle = get("angle")?.unwrap_or(0.0);
                    let passes = get("passes")?.unwrap_or(1.0);
                    if !angle.is_finite() {
                        return err("wipe: angle is in radians");
                    }
                    if !(1.0..=20.0).contains(&passes) || passes.fract() != 0.0 {
                        return err(format!("wipe: passes {passes}: a whole number 1..20"));
                    }
                    let refold = match get("refold")? {
                        Some(l) if l.is_finite() => Some(l.clamp(0.0, 1.0)),
                        Some(_) => return err("wipe: refold is a load 0..1"),
                        None => None,
                    };
                    let sd = seed(&u.st, lua, o.as_ref())?;
                    let pass = RagPass { pressure: pr[0], angle, passes: passes as u32, refold, seed: sd };
                    time::verb(&u.st, Verb::Pass, |s| {
                        let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                        c.rag_region(&mut u.rag.borrow_mut(), &m, &pass);
                        Ok(())
                    })
                }
                Value::Table(_) => {
                    if let Some(o) = &o {
                        for k in ["angle", "passes", "refold"] {
                            if o.contains_key(k)? {
                                return err(format!("wipe: {k}= is for wiping a mask; along a path, give pressure= (and seed=)"));
                            }
                        }
                    }
                    let pts = points(&a)?;
                    if pts.is_empty() || pts.iter().any(|p| !p.0.is_finite() || !p.1.is_finite()) {
                        return err("wipe: want points {{x, y}, ...} along the path");
                    }
                    let sd = seed(&u.st, lua, o.as_ref())?;
                    time::verb(&u.st, Verb::Marks, |s| {
                        let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                        c.rag_wipe(&mut u.rag.borrow_mut(), &pts, &pr, sd);
                        Ok(())
                    })
                }
                o => err(format!("wipe: want a mask or points, got {}; use r:wipe(rect(100,100,200,200)) or r:wipe(o:mask()) for a closed outline; an open outline uses o:below(), o:above() or o:band(10)", o.type_name())),
            }
        });
        // r:blot(x, y, {pressure=, seed=}): the pad pressed straight down and lifted off
        m.add_method("blot", |lua, u, (x, y, o): (f32, f32, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, BLOT_KEYS, "blot")?;
            }
            if !x.is_finite() || !y.is_finite() {
                return err("blot: x and y are canvas units");
            }
            let pr = pressures(o.as_ref())?;
            if pr.len() != 1 {
                return err("blot: pressure is one number");
            }
            let sd = seed(&u.st, lua, o.as_ref())?;
            time::verb(&u.st, Verb::Marks, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                c.rag_blot(&mut u.rag.borrow_mut(), x, y, pr[0], sd);
                Ok(())
            })
        });
        // r:dip(amount?): the face in use dipped into spirits (0..1, 0.5 a light dip)
        m.add_method("dip", |_, u, amount: Option<f32>| {
            let a = amount.unwrap_or(0.5);
            if !a.is_finite() {
                return err("dip: amount is 0..1");
            }
            time::verb(&u.st, Verb::Marks, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                let now = c.now_min();
                u.rag.borrow_mut().dip(a, now, c.tally_mut());
                Ok(())
            })
        });
        // r:refold(): a cleaner face of the cloth outward
        m.add_method("refold", |_, u, ()| {
            time::verb(&u.st, Verb::Marks, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                u.rag.borrow_mut().refold(c.tally_mut());
                Ok(())
            })
        });
    }
}

/// Whether a session painting with engine `engine` has the rag: from engine
/// 3. An older log replays without the global `rag`, so what it prints from
/// `pairs(_G)` (how many globals, in what order) is what it printed when it
/// was painted.
pub fn has_rag(engine: u32) -> bool {
    engine >= 3
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    // rag() or rag{width=<units>, seed=}: a clean cloth taken and bunched into a pad
    let st1 = st.clone();
    lua.globals().set("rag", lua.create_function(move |lua, o: Option<Table>| {
        if let Some(o) = &o {
            check_keys(o, RAG_KEYS, "rag")?;
        }
        let mmu = st1.borrow().canvas.as_ref().ok_or_else(no_canvas)?.mm_per_unit();
        let width = match o.as_ref().map(|o| num(o, "width")).transpose()?.flatten() {
            Some(w) if w > 0.0 && w.is_finite() => w,
            Some(w) => return err(format!("rag: width {w}: want > 0 (canvas units)")),
            None => PAD_MM / mmu,
        };
        let sd = seed(&st1, lua, o.as_ref())?;
        let rag = Rc::new(RefCell::new(Rag::new(width, sd)));
        time::verb(&st1, Verb::Marks, |s| {
            s.rags.push(Rc::downgrade(&rag));
            s.canvas.as_mut().ok_or_else(no_canvas)?.tally_mut().secs += pace::FRESH;
            Ok(())
        })?;
        Ok(RagU { rag, st: st1.clone() })
    })?)?;
    Ok(())
}

// every test here paints from the default box
#[cfg(all(test, tube_box))]
mod tests {
    use crate::session::Session;
    use paint::rag::pace as rag_pace;
    use paint::tally::{pace, touch_secs};

    const W: usize = 240;
    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=40, apply="brush"}}}"#;
    const SKY: &str = r#"sky = pile{{"lead white", 3}, {"smalt", 2}}; work(rect(100, 100, 800, 400), {hand="broad", pile=sky, angle=0, coverage=2})"#;

    fn run(s: &mut Session, src: &str) -> String {
        s.run(src).unwrap_or_else(|e| panic!("{src}: {e}")).out
    }

    fn clock(s: &Session) -> f64 {
        s.st.borrow().clock
    }

    fn bits(s: &Session) -> Vec<u32> {
        s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect()
    }

    /// A wipe along a path takes the time to bring the pad down (Fitts) and
    /// drag it at the rag's pace; a blot a touch and a press; a refold its
    /// own time; a wipe over a region the sum of its strokes (minutes).
    #[test]
    fn the_rag_takes_hand_time() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, SKY);
        let mm = s.canvas().unwrap().mm_per_unit() as f64;
        run(&mut s, "r = rag(); print(r)");
        let t0 = clock(&s);
        run(&mut s, "r:wipe({{200, 300}, {600, 300}}, {pressure=0.6})");
        let (len, w) = (400.0 * mm, 40.0);
        let want = pace::FITTS_A + pace::FITTS_B * (1.0 + len / (2.0 * w)).log2() + len / rag_pace::RAG_MM_S;
        assert!((clock(&s) - t0 - want / 60.0).abs() < 1e-6, "wipe: {} min, want {}", clock(&s) - t0, want / 60.0);
        let t0 = clock(&s);
        run(&mut s, "r:blot(500, 250); r:refold(); r:dip()");
        let want = touch_secs() + rag_pace::BLOT_PRESS + rag_pace::REFOLD + rag_pace::DIP;
        assert!((clock(&s) - t0 - want / 60.0).abs() < 1e-6, "blot, refold and dip: {} min, want {}", clock(&s) - t0, want / 60.0);
        let t0 = clock(&s);
        run(&mut s, "r:wipe(rect(150, 150, 700, 300), {pressure=0.5, passes=2, refold=0.6})");
        let dt = clock(&s) - t0;
        assert!(dt > 0.2 && dt < 5.0, "a region 300 × 130 mm wiped twice: {dt} min");
    }

    /// The spirits evaporate from a dipped rag as the painting's clock runs
    /// (paint::rag `DAMP_HALF_MIN`, 3 minutes): `r.damp` falls by about half
    /// over three minutes and is 0 a week later; a dip wets it again.
    #[test]
    fn a_dipped_rag_dries_as_time_passes() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        assert_eq!(run(&mut s, "r = rag(); r:dip(0.5); print(r.damp)").trim(), "0.5");
        // damp as of the hand's return from the cup, half as damp three minutes later
        let three: f64 = run(&mut s, "wait(3); print(r.damp)").trim().parse().unwrap();
        assert!((three - 0.25).abs() < 1e-6, "{three}");
        assert_eq!(run(&mut s, "wait(10080); print(r.damp)").trim(), "0.0");
        assert_eq!(run(&mut s, "r:dip(0.5); print(r.damp)").trim(), "0.5");
    }

    /// A failed chunk takes back what the rag did: the canvas, the clock and
    /// the rag's load (a field of its Lua table) are as they were.
    #[test]
    fn a_failed_chunk_takes_back_the_rag() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, SKY);
        run(&mut s, "r = rag{width=60}");
        let (before, t0) = (bits(&s), clock(&s));
        let e = s.run("r:wipe(rect(150, 150, 700, 300), {pressure=0.8}); r:blot(400, 200); assert(r.load > 0); error('stop')").unwrap_err();
        assert!(e.contains("stop"), "{e}");
        assert!(bits(&s) == before);
        assert_eq!(clock(&s), t0);
        run(&mut s, "assert(r.load == 0 and r.soaked == 0 and r.fold == 0)");
        // the same chunk without the error lifts paint
        run(&mut s, "r:wipe(rect(150, 150, 700, 300), {pressure=0.8}); r:blot(400, 200)");
        assert!(bits(&s) != before);
        assert!(run(&mut s, "print(r.load)").trim().parse::<f64>().unwrap() > 0.0);
    }

    /// The painter can't clean the cloth by hand: its state is read only,
    /// so a clean face costs a refold and a clean cloth a fresh rag, each
    /// with its hand time. A refused write changes nothing.
    #[test]
    fn the_cloth_can_t_be_cleaned_by_assignment() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, SKY);
        let t0 = clock(&s);
        run(&mut s, "r = rag()");
        assert!((clock(&s) - t0 - rag_pace::FRESH / 60.0).abs() < 1e-6, "a fresh rag: {} min", clock(&s) - t0);
        run(&mut s, "r:wipe(rect(150, 150, 700, 300), {pressure=0.8}); load0, soaked0, fold0 = r.load, r.soaked, r.fold");
        let before = bits(&s);
        for f in ["load", "soaked", "damp", "fold", "width"] {
            for src in [format!("r.{f} = 0"), format!("rawset(r, {f:?}, 0)"), format!("getmetatable(r).__index = function() return 0 end")] {
                assert!(s.run(&src).is_err(), "{src} was allowed");
            }
            let e = s.run(&format!("r.{f} = 0")).unwrap_err();
            assert!(e.contains("can't be set") && e.contains("r:refold()"), "{e}");
        }
        run(&mut s, "assert(r.load == load0 and r.load > 0.1 and r.soaked == soaked0 and r.fold == fold0)");
        assert!(bits(&s) == before);
        // a refold is the way to a cleaner face, and it takes its time
        let t0 = clock(&s);
        run(&mut s, "r:refold(); assert(r.load < load0 and r.fold == fold0 + 1)");
        assert!((clock(&s) - t0 - rag_pace::REFOLD / 60.0).abs() < 1e-6);
    }

    /// An engine-2 log replays with the globals it had: no `rag`, and
    /// `pairs(_G)` walks the same names in the same order. The count and
    /// hash were printed by the round-23 easel (7b80cb0) for this program;
    /// a log of today's engine has the rag.
    #[test]
    fn an_older_log_sees_the_globals_it_saw() {
        let walk = r#"local n, keys = 0, {}
for k in pairs(_G) do n = n + 1; keys[n] = tostring(k) end
local h = 0
for _, k in ipairs(keys) do for i = 1, #k do h = (h * 31 + k:byte(i)) % 4294967296 end end
print(n, h, type(rag))"#;
        let mut tubes = paint::Palette::tube_box();
        tubes.engine = 2;
        let mut old = Session::with_box(W, tubes).unwrap();
        run(&mut old, CANVAS);
        assert_eq!(run(&mut old, walk).trim(), "69\t602616302\tnil");
        let mut new = Session::new(W).unwrap();
        assert!(paint::ENGINE >= 3);
        run(&mut new, CANVAS);
        assert_eq!(run(&mut new, "print(type(rag))").trim(), "function");
    }

    /// Mistyped options and arguments are errors that say what is wanted.
    #[test]
    fn the_rag_refuses_what_it_cannot_do() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, "r = rag()");
        for (src, want) in [
            ("r:wipe(rect(0, 0, 100, 100), {presure=1})", "unknown option"),
            ("r:wipe({{0, 0}, {100, 0}}, {passes=2})", "for wiping a mask"),
            ("r:wipe(rect(0, 0, 100, 100), {passes=0})", "passes"),
            ("r:wipe(5)", "want a mask or points"),
            ("rag{width=-1}", "width"),
            ("r.wipe({}, rect(0, 0, 10, 10))", "self"),
        ] {
            let e = s.run(src).unwrap_err();
            assert!(e.contains(want), "{src}: {e}");
        }
    }
}
