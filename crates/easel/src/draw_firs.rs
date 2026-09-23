//! Firs in Lua: `fir{...}` grows one spruce into a silhouette you draw (an
//! `outline{}` or a few points), and `fir_wood{...}` grows a wood of them
//! in receding rows behind a drawn skyline. Geometry only: boughs, needle
//! masks, the light on them and the short hatched strokes a pointed brush
//! lays (`f:paint(b, {...})` lays them for you).
//!
//! ```lua
//! spire = outline{{500,90}, {520,200}, {548,330}, {575,430,"c"}, {500,446}, {425,432,"c"}, {452,330}, {480,200}, char="soft", seed=2}
//! f = fir{envelope=spire, habit="spire", seed=4}
//! work(f:needles(), {hand="hatch", tool="round 1.6", color="#1e2a24", clip=f:needles()})
//! f:paint(brush("round", 1.4), {color="#1b2520", lit={0, 0.55}})
//! f:paint(brush("round", 1.2), {color="#4c5a3e", lit={0.55, 1}})
//! wood = fir_wood{skyline={{0,120}, {300,90}, {640,200}}, foot=380, depth=4, count=12, seed=3}
//! ```

use crate::api::{S, check_keys, err, frame, mask_of, num, points, seed_of, wrap};
use crate::draw_outline::outline_path;
use mlua::{AnyUserData, Lua, MetaMethod, ObjectLike, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::fir::{Fir, FirHabit, Hatch, Wood, WoodSpec, envelope_for};
use std::rc::Rc;

#[derive(Clone)]
pub struct FirU {
    f: Rc<Fir>,
    habit: String,
    st: S,
}

fn pt(lua: &Lua, p: (f32, f32)) -> Result<Table> {
    lua.create_sequence_from([p.0, p.1])
}

fn pts_table(lua: &Lua, pts: &[(f32, f32)]) -> Result<Table> {
    let t = lua.create_table_with_capacity(pts.len(), 0)?;
    for (i, p) in pts.iter().enumerate() {
        t.raw_set(i + 1, pt(lua, *p)?)?;
    }
    Ok(t)
}

/// Points from an outline value or a point list; `closed` if it is a shape.
fn line_of(v: &Value, what: &str) -> Result<Vec<(f32, f32)>> {
    match v {
        Value::UserData(u) => match outline_path(u) {
            Some((p, _)) => Ok(p),
            None => err(format!("{what}: want an outline{{}} or points {{{{x, y}}, ...}}")),
        },
        Value::Table(_) => points(v),
        Value::Nil => err(format!("{what}: missing")),
        o => err(format!("{what}: want an outline{{}} or points, got {}", o.type_name())),
    }
}

fn sun_of(o: &Table) -> Result<(f32, f32, f32)> {
    Ok(match o.get::<Option<Vec<f32>>>("sun")? {
        Some(v) => (v.first().copied().unwrap_or(-0.55), v.get(1).copied().unwrap_or(-0.75), v.get(2).copied().unwrap_or(0.35)),
        None => (-0.55, -0.75, 0.35),
    })
}

const HABITS: &str = "spire, old (or ragged), young, storm";

const FIR_KEYS: &[&str] = &[
    "envelope", "foot", "habit", "seed", "sun", "x", "y", "height", "width", "tiers", "whorl", "inter", "fill", "gap", "broken", "dead_below", "crook", "kink", "rise",
    "droop", "upturn", "pad", "clumpy", "bare_inner", "wind", "trunk", "dead_top",
];

fn fir(st: &S, o: Table) -> Result<FirU> {
    check_keys(&o, FIR_KEYS, "fir")?;
    let hname = o.get::<Option<String>>("habit")?.unwrap_or_else(|| "spire".into());
    let mut h = FirHabit::named(&hname).ok_or_else(|| mlua::Error::runtime(format!("fir habit {hname:?}: one of {HABITS}")))?;
    macro_rules! over {
        ($($f:ident),*) => {$( if let Some(v) = num(&o, stringify!($f))? { h.$f = v; } )*};
    }
    over!(tiers, inter, gap, broken, dead_below, crook, kink, droop, upturn, pad, clumpy, bare_inner, wind, trunk, dead_top);
    if let Some(v) = o.get::<Option<Vec<f32>>>("fill")? {
        h.fill = (v.first().copied().unwrap_or(0.8), v.get(1).copied().unwrap_or(1.0));
    }
    if let Some(v) = o.get::<Option<Vec<f32>>>("rise")? {
        h.rise = (v.first().copied().unwrap_or(0.5), v.get(1).copied().unwrap_or(-0.3));
    }
    if let Some(v) = o.get::<Option<Vec<u32>>>("whorl")? {
        h.per_whorl = (v.first().copied().unwrap_or(4).max(1), v.get(1).copied().unwrap_or(6).max(1));
    }
    let seed = seed_of(st, &o)?;
    let env = match o.get::<Value>("envelope")? {
        Value::Nil => {
            // no drawing: a made-up envelope from x, y (the foot), height, width
            let (Some(x), Some(y), Some(ht)) = (num(&o, "x")?, num(&o, "y")?, num(&o, "height")?) else {
                return err("fir: needs envelope= (an outline{} or points) or x=, y= (the foot) and height=");
            };
            let wd = num(&o, "width")?.unwrap_or(ht * 0.3);
            envelope_for(&hname, (x, y - ht), y - ht * 0.06, wd * 0.5, seed)
        }
        v => line_of(&v, "fir envelope")?,
    };
    if env.len() < 3 {
        return err("fir envelope: needs at least three points (a closed shape: apex, sides, base)");
    }
    let foot = match o.get::<Value>("foot")? {
        Value::Nil => match (num(&o, "x")?, num(&o, "y")?) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        },
        Value::Table(t) => Some((t.get::<f32>(1)?, t.get::<f32>(2)?)),
        v => return err(format!("fir foot: want {{x, y}}, got {}", v.type_name())),
    };
    Ok(FirU { f: Rc::new(Fir::grow(&env, foot, &h, sun_of(&o)?, seed)), habit: hname, st: st.clone() })
}

fn stroke_table(lua: &Lua, s: &Hatch) -> Result<Table> {
    let t = lua.create_table()?;
    t.set("pts", pts_table(lua, &s.pts)?)?;
    t.set("w", s.w)?;
    t.set("lit", s.lit)?;
    t.set("z", s.z)?;
    t.set("kind", s.kind.name())?;
    t.set("bough", s.bough + 1)?;
    Ok(t)
}

fn range_of(o: &Table, k: &str, dflt: (f32, f32)) -> Result<(f32, f32)> {
    Ok(match o.get::<Value>(k)? {
        Value::Nil => dflt,
        Value::Table(t) => (t.get::<Option<f32>>(1)?.unwrap_or(dflt.0), t.get::<Option<f32>>(2)?.unwrap_or(dflt.1)),
        Value::Number(n) => (n as f32, dflt.1),
        Value::Integer(n) => (n as f32, dflt.1),
        v => return err(format!("{k}: want {{lo, hi}}, got {}", v.type_name())),
    })
}

/// Which strokes: {kind="under"|"top"|"twig"|{...}, lit={lo, hi}, z={lo, hi}}
struct Pick {
    kinds: Option<Vec<String>>,
    lit: (f32, f32),
    z: (f32, f32),
}

impl Pick {
    fn of(o: Option<&Table>) -> Result<Pick> {
        let Some(o) = o else { return Ok(Pick { kinds: None, lit: (0.0, 1.0), z: (f32::MIN, f32::MAX) }) };
        let kinds = match o.get::<Value>("kind")? {
            Value::Nil => None,
            Value::String(s) => Some(vec![s.to_str()?.to_string()]),
            Value::Table(t) => Some(t.sequence_values::<String>().collect::<Result<_>>()?),
            v => return err(format!("kind: \"under\", \"top\", \"twig\" or a list, got {}", v.type_name())),
        };
        Ok(Pick { kinds, lit: range_of(o, "lit", (0.0, 1.0))?, z: range_of(o, "z", (f32::MIN, f32::MAX))? })
    }
    fn takes(&self, s: &Hatch) -> bool {
        let k = self.kinds.as_ref().is_none_or(|ks| ks.iter().any(|k| k == s.kind.name()));
        // lit ranges are half open so {0, 0.55} and {0.55, 1} split the strokes
        let lit = s.lit >= self.lit.0 && (s.lit < self.lit.1 || self.lit.1 >= 1.0);
        k && lit && s.z >= self.z.0 && s.z <= self.z.1
    }
}

const PAINT_KEYS: &[&str] = &["color", "load", "every", "pressure", "ramps", "shake", "clip", "kind", "lit", "z", "fit"];

/// Lay strokes with a brush: `color` a color or function(stroke) -> color.
fn lay(lua: &Lua, trees: &[Rc<Fir>], b: &AnyUserData, opts: Option<Table>) -> Result<usize> {
    let opts = opts.unwrap_or(lua.create_table()?);
    check_keys(&opts, PAINT_KEYS, "fir:paint")?;
    let pick = Pick::of(Some(&opts))?;
    let color = opts.get::<Value>("color")?;
    if color.is_nil() {
        return err("fir:paint: needs color= (a color, or function(stroke) returning one)");
    }
    let load = num(&opts, "load")?.unwrap_or(0.8);
    let every = opts.get::<Option<usize>>("every")?.unwrap_or(10).max(1);
    let press = range_of(&opts, "pressure", (0.75, 0.05))?;
    let ramps = range_of(&opts, "ramps", (0.08, 0.6))?;
    let shake = num(&opts, "shake")?.unwrap_or(0.4);
    let fit = opts.get::<Option<bool>>("fit")?.unwrap_or(true);
    let clip = opts.get::<Value>("clip")?;
    if !clip.is_nil() {
        mask_of(&clip)?;
    }
    let bw: f32 = b.get("width")?;
    let mut n = 0;
    for f in trees {
        for s in f.strokes.iter().filter(|s| pick.takes(s)) {
            if n % every == 0 {
                let c = match &color {
                    Value::Function(g) => g.call::<Value>(stroke_table(lua, s)?)?,
                    c => c.clone(),
                };
                b.call_method::<()>("reload", (c, load))?;
            }
            let so = lua.create_table()?;
            // a pointed brush pressed to the stroke's width
            let p0 = if fit {
                let pw: f32 = b.call_method("pressure_for", s.w.min(bw))?;
                (pw * press.0 / 0.75).clamp(0.05, 1.0)
            } else {
                press.0
            };
            so.set("pressure", lua.create_sequence_from([p0, press.1])?)?;
            so.set("ramps", lua.create_sequence_from([ramps.0, ramps.1])?)?;
            so.set("shake", shake)?;
            if !clip.is_nil() {
                so.set("clip", clip.clone())?;
            }
            b.call_method::<()>("stroke", (pts_table(lua, &s.pts)?, so))?;
            n += 1;
        }
    }
    Ok(n)
}

impl UserData for FirU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("apex", |lua, o| pt(lua, o.f.apex));
        f.add_field_method_get("foot", |lua, o| pt(lua, o.f.foot));
        f.add_field_method_get("crown_base", |_, o| Ok(o.f.crown_base));
        f.add_field_method_get("tier", |_, o| Ok(o.f.tier));
        f.add_field_method_get("hatch", |_, o| Ok(o.f.hatch));
        f.add_field_method_get("habit", |_, o| Ok(o.habit.clone()));
        f.add_field_method_get("bounds", |lua, o| {
            let b = o.f.bounds();
            lua.create_sequence_from([b.0, b.1, b.2, b.3])
        });
        f.add_field_method_get("envelope", |lua, o| pts_table(lua, &o.f.envelope));
        // {pts=, w=, dead_from=}: the stem, foot to apex
        f.add_field_method_get("leader", |lua, o| {
            let t = lua.create_table()?;
            t.set("pts", pts_table(lua, &o.f.leader)?)?;
            t.set("w", lua.create_sequence_from(o.f.leader_w.iter().copied())?)?;
            t.set("dead_from", o.f.leader_dead_from.min(o.f.leader.len()) + 1)?;
            Ok(t)
        });
        // back to front: {pts=, w=, pad={{above, below}, ...}, z=, side=, t=, dead=, broken=, minor=}
        f.add_field_method_get("boughs", |lua, o| {
            let out = lua.create_table()?;
            for (i, b) in o.f.boughs.iter().enumerate() {
                let t = lua.create_table()?;
                t.set("pts", pts_table(lua, &b.pts)?)?;
                t.set("w", lua.create_sequence_from(b.w.iter().copied())?)?;
                t.set("pad", pts_table(lua, &b.pad)?)?;
                t.set("z", b.z)?;
                t.set("side", b.side)?;
                t.set("t", b.t)?;
                t.set("dead", b.dead)?;
                t.set("broken", b.broken)?;
                t.set("minor", b.minor)?;
                out.raw_set(i + 1, t)?;
            }
            Ok(out)
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("needles", |_, o, ()| Ok(wrap(o.f.needles(frame(&o.st)?))));
        m.add_method("wood", |_, o, ()| Ok(wrap(o.f.wood(frame(&o.st)?))));
        m.add_method("trunk", |_, o, ()| Ok(wrap(o.f.trunk(frame(&o.st)?))));
        m.add_method("mask", |_, o, ()| Ok(wrap(o.f.mask(frame(&o.st)?))));
        m.add_method("lit", |_, o, from: Option<f32>| Ok(wrap(o.f.lit(frame(&o.st)?, from.unwrap_or(0.55)))));
        m.add_method("shade", |_, o, from: Option<f32>| {
            let f = frame(&o.st)?;
            Ok(wrap(o.f.needles(f).subtract(&o.f.lit(f, from.unwrap_or(0.55)))))
        });
        // the needles with holes closed, and the holes (sky through the crown)
        m.add_method("gaps", |_, o, reach: Option<f32>| {
            let f = frame(&o.st)?;
            let r = reach.unwrap_or(o.f.tier * 0.6);
            let m = o.f.mask(f);
            Ok(wrap(m.dilate(r).erode(r).subtract(&m)))
        });
        m.add_method("strokes", |lua, o, opts: Option<Table>| {
            if let Some(t) = &opts {
                check_keys(t, &["kind", "lit", "z"], "fir:strokes")?;
            }
            let pick = Pick::of(opts.as_ref())?;
            let out = lua.create_table()?;
            let mut i = 0;
            for s in o.f.strokes.iter().filter(|s| pick.takes(s)) {
                i += 1;
                out.raw_set(i, stroke_table(lua, s)?)?;
            }
            Ok(out)
        });
        // f:paint(brush, {color=, lit={lo, hi}, kind=, z=, every=10, load=0.8, pressure={0.75, 0.05}, ramps=, shake=, clip=, fit=true})
        m.add_method("paint", |lua, o, (b, opts): (AnyUserData, Option<Table>)| lay(lua, std::slice::from_ref(&o.f), &b, opts));
        m.add_meta_method(MetaMethod::ToString, |_, o, ()| {
            let f = &o.f;
            Ok(format!(
                "fir({}, {:.0} tall, {} boughs ({} dead, {} broken), {} strokes, tier {:.1}, hatch {:.2})",
                o.habit,
                f.foot.1 - f.apex.1,
                f.boughs.len(),
                f.boughs.iter().filter(|b| b.dead).count(),
                f.boughs.iter().filter(|b| b.broken).count(),
                f.strokes.len(),
                f.tier,
                f.hatch
            ))
        });
    }
}

// ---------------------------------------------------------------- woods

pub struct WoodU {
    w: Rc<Wood>,
    trees: Vec<Vec<Rc<Fir>>>,
    st: S,
}

const WOOD_KEYS: &[&str] = &["skyline", "foot", "depth", "count", "horizon", "recede", "air", "width", "mix", "bare", "sun", "seed"];

fn fir_wood(st: &S, o: Table) -> Result<WoodU> {
    check_keys(&o, WOOD_KEYS, "fir_wood")?;
    let sky = line_of(&o.get::<Value>("skyline")?, "fir_wood skyline")?;
    if sky.len() < 2 {
        return err("fir_wood skyline: needs at least two points (the tops of the wood, left to right)");
    }
    let low = sky.iter().map(|p| p.1).fold(f32::MIN, f32::max);
    let high = sky.iter().map(|p| p.1).fold(f32::MAX, f32::min);
    let foot = match o.get::<Value>("foot")? {
        Value::Nil => vec![(0.0, low + (low - high).max(40.0) * 1.2)],
        Value::Number(n) => vec![(0.0, n as f32)],
        Value::Integer(n) => vec![(0.0, n as f32)],
        v => line_of(&v, "fir_wood foot")?,
    };
    let mut spec = WoodSpec { sun: sun_of(&o)?, ..WoodSpec::default() };
    if let Some(d) = o.get::<Option<usize>>("depth")? {
        if !(1..=8).contains(&d) {
            return err(format!("fir_wood depth {d}: rows of trees, 1..8"));
        }
        spec.rows = d;
    }
    if let Some(c) = o.get::<Option<usize>>("count")? {
        spec.count = c.clamp(1, 200);
    }
    spec.horizon = num(&o, "horizon")?;
    spec.recede = num(&o, "recede")?.unwrap_or(spec.recede).max(0.0);
    spec.air = num(&o, "air")?.unwrap_or(spec.air).max(0.0);
    spec.width = range_of(&o, "width", spec.width)?;
    spec.bare = range_of(&o, "bare", spec.bare)?;
    if let Some(m) = o.get::<Option<Table>>("mix")? {
        check_keys(&m, &["spire", "old", "young"], "fir_wood mix")?;
        spec.mix = (num(&m, "spire")?.unwrap_or(0.0), num(&m, "old")?.unwrap_or(0.0), num(&m, "young")?.unwrap_or(0.0));
        if spec.mix.0 + spec.mix.1 + spec.mix.2 <= 0.0 {
            return err("fir_wood mix: give some weight to spire, old or young");
        }
    }
    let seed = seed_of(st, &o)?;
    let w = Wood::grow(&sky, &foot, &spec, seed);
    let trees = w.rows.iter().map(|r| r.trees.iter().cloned().map(Rc::new).collect()).collect();
    Ok(WoodU { w: Rc::new(w), trees, st: st.clone() })
}

impl WoodU {
    fn row(&self, r: Option<usize>) -> Result<usize> {
        let n = self.w.rows.len();
        match r {
            None => err(format!("wood: which row? 1 (front) .. {n}")),
            Some(r) if r == 0 || r > n => err(format!("wood: row {r} out of 1..{n} (1 is the front)")),
            Some(r) => Ok(r - 1),
        }
    }
}

impl UserData for WoodU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("depth", |_, w| Ok(w.w.rows.len()));
        f.add_field_method_get("horizon", |_, w| Ok(w.w.horizon));
        // {{trees={fir, ...}, scale=, haze=, foot={{x, y}, ...}}, ...} front row first
        f.add_field_method_get("rows", |lua, w| {
            let out = lua.create_table()?;
            for (i, r) in w.w.rows.iter().enumerate() {
                let t = lua.create_table()?;
                let trees = lua.create_table()?;
                for (k, f) in w.trees[i].iter().enumerate() {
                    trees.raw_set(k + 1, FirU { f: f.clone(), habit: "wood".into(), st: w.st.clone() })?;
                }
                t.set("trees", trees)?;
                t.set("scale", r.scale)?;
                t.set("haze", r.haze)?;
                t.set("foot", pts_table(lua, &r.foot)?)?;
                out.raw_set(i + 1, t)?;
            }
            Ok(out)
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("needles", |_, w, r: Option<usize>| Ok(wrap(w.w.needles(frame(&w.st)?, w.row(r)?))));
        m.add_method("wood", |_, w, r: Option<usize>| Ok(wrap(w.w.wood(frame(&w.st)?, w.row(r)?))));
        m.add_method("lit", |_, w, (r, from): (Option<usize>, Option<f32>)| Ok(wrap(w.w.lit(frame(&w.st)?, w.row(r)?, from.unwrap_or(0.55)))));
        m.add_method("all", |_, w, r: Option<usize>| Ok(wrap(w.w.all(frame(&w.st)?, w.row(r)?))));
        // row r less every row in front of it
        m.add_method("visible", |_, w, r: Option<usize>| {
            let f = frame(&w.st)?;
            let r = w.row(r)?;
            let mut m = w.w.all(f, r);
            for q in 0..r {
                m = m.subtract(&w.w.all(f, q));
            }
            Ok(wrap(m))
        });
        // everything, all rows
        m.add_method("mask", |_, w, ()| {
            let f = frame(&w.st)?;
            let mut m = w.w.all(f, 0);
            for q in 1..w.w.rows.len() {
                m = m.union(&w.w.all(f, q));
            }
            Ok(wrap(m))
        });
        m.add_method("floor", |_, w, ()| Ok(wrap(w.w.floor(frame(&w.st)?))));
        // w:paint(brush, row, {...}): as fir:paint, over every tree of a row
        m.add_method("paint", |lua, w, (b, r, opts): (AnyUserData, Option<usize>, Option<Table>)| {
            let r = w.row(r)?;
            lay(lua, &w.trees[r], &b, opts)
        });
        m.add_meta_method(MetaMethod::ToString, |_, w, ()| {
            let rows: Vec<String> = w.w.rows.iter().map(|r| format!("{} trees scale {:.2} haze {:.2}", r.trees.len(), r.scale, r.haze)).collect();
            Ok(format!("fir_wood({} rows: {}; horizon {:.0})", rows.len(), rows.join(" | "), w.w.horizon))
        });
    }
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let s1 = st.clone();
    g.set("fir", lua.create_function(move |_, o: Table| fir(&s1, o))?)?;
    let s1 = st.clone();
    g.set("fir_wood", lua.create_function(move |_, o: Table| fir_wood(&s1, o))?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    fn run(src: &str) -> Result<String, String> {
        let mut s = Session::replay(200).unwrap();
        s.run(r#"canvas{aspect=1.4, seed=11}"#).unwrap();
        s.run(src).map(|r| r.out)
    }

    #[test]
    fn a_fir_grows_into_a_drawn_outline() {
        let out = run(r##"local env = outline{{100,20},{112,60},{128,110},{140,150,"c"},{100,156},{60,150,"c"},{72,110},{88,60}, char="soft", seed=2}
               local f = fir{envelope=env, habit="old", seed=4}
               assert(#f.boughs > 20, #f.boughs)
               local n = f:needles():area()
               assert(n > 500, n)
               local lit = f:lit():area()
               assert(lit > 0 and lit < n, lit)
               local s = f:strokes{kind="under", lit={0.6, 1}}
               for _, k in ipairs(s) do assert(k.lit >= 0.6 and k.kind == "under") end
               local b = brush("round", 1.2)
               local laid = f:paint(b, {color="#20302a", lit={0, 0.6}})
               print(tostring(f), laid)"##)
        .unwrap();
        assert!(out.contains("fir(old"), "{out}");
    }

    #[test]
    fn a_wood_has_rows_and_masks() {
        run(r#"local w = fir_wood{skyline={{0,60},{200,40},{400,90}}, foot=200, depth=3, count=6, seed=5}
               assert(w.depth == 3)
               local rows = w.rows
               assert(#rows[3].trees >= #rows[1].trees)
               assert(rows[3].haze > rows[1].haze)
               local v2 = w:visible(2):area()
               assert(v2 > 0 and v2 < w:all(2):area())
               assert(w:floor():area() > 0)
               local ok, e = pcall(function() return w:needles(4) end)
               assert(not ok and tostring(e):find("out of"), e)"#)
        .unwrap();
        let e = run(r#"fir{envelope={{0,0},{1,1},{2,0}}, habit="pine"}"#).unwrap_err();
        assert!(e.contains("spire"), "{e}");
    }
}
