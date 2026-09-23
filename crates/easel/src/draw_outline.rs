//! Drawn outlines in Lua: `outline{...}` (a few points become a line a hand
//! drew, and its mask) and `body_of{...}` (a silhouette grown from a
//! skeleton). Geometry only: a brush paints the line with `o:paint(b)`.
//!
//! ```lua
//! rock = outline{{300,600}, {320,450,"c"}, {420,380}, {560,400,"c"}, {650,480}, {680,600,"c"}, {500,620},
//!                char="broken", seed=3}
//! work(rock:mask(), {...})             -- fill it
//! rock:paint(b, {pressure=0.8})        -- draw its contour with a pointed brush
//! sheep = body_of{spine={{100,500},{110,499},{120,500},{126,497},{130,499}}, widths={9,11,10,5,4},
//!                 limbs={{{102,502},{102,510}}, ...}, char="soft"}
//! ```

use crate::api::{S, check_keys, err, frame, mask_of, num, points, seed_of, wrap};
use mlua::{AnyUserData, ObjectLike, Lua, MetaMethod, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::outline::{Bone, Character, Outline, hand_scale};
use std::rc::Rc;

#[derive(Clone)]
pub struct OutlineU {
    o: Rc<Outline>,
    st: S,
}

fn pts_table(lua: &Lua, pts: &[(f32, f32)]) -> Result<Table> {
    let t = lua.create_table_with_capacity(pts.len(), 0)?;
    for (i, p) in pts.iter().enumerate() {
        t.raw_set(i + 1, lua.create_sequence_from([p.0, p.1])?)?;
    }
    Ok(t)
}

/// The drawn line of an outline value (its first line), for other tools
/// that take an outline where they take points.
pub(crate) fn outline_path(v: &AnyUserData) -> Option<(Vec<(f32, f32)>, bool)> {
    let o = v.borrow::<OutlineU>().ok()?;
    let l = o.o.lines.first()?;
    Some((l.pts.clone(), l.closed))
}

/// The corners of an outline value's first line (points), for tools that
/// build on a drawn outline's corners (rocks).
pub(crate) fn outline_corners(v: &AnyUserData) -> Option<Vec<(f32, f32)>> {
    let o = v.borrow::<OutlineU>().ok()?;
    let l = o.o.lines.first()?;
    Some(l.corners.iter().filter_map(|&i| l.pts.get(i).copied()).collect())
}

const CHARS: &str = "firm, searching, broken, soft";

/// The character from `char=`, `amount=` and `lobe=` (units), and the hand's
/// scale (`size=`, or from the extent of `pts`).
fn character(o: &Table, pts: &[(f32, f32)], default: &str) -> Result<(Character, f32)> {
    let name: String = o.get::<Option<String>>("char")?.unwrap_or_else(|| default.into());
    let mut ch = Character::named(&name).ok_or_else(|| mlua::Error::runtime(format!("char {name:?}: one of {CHARS}")))?;
    let scale = match num(o, "size")? {
        Some(s) if s > 0.0 => s,
        Some(s) => return err(format!("size {s}: want > 0 (the hand's scale in units)")),
        None => {
            let (x0, y0, x1, y1) = pts.iter().fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)));
            hand_scale(((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt())
        }
    };
    // explicit options first, then amount= scales them with the rest (so
    // amount=0 is a clean curve whatever lobe= says)
    if let Some(l) = num(o, "lobe")? {
        if l <= 0.0 {
            ch.lobe = 0.0;
        } else {
            ch.lobe = l / scale;
            if ch.lobe_height == 0.0 {
                ch.lobe_height = 0.35;
            }
        }
    }
    if let Some(e) = num(o, "edge")? {
        ch.edge = e.max(0.0) / scale;
        if ch.edge_var == 0.0 {
            ch.edge_var = 0.5;
        }
    }
    if let Some(k) = num(o, "amount")? {
        ch = ch.amount(k);
    }
    Ok((ch, scale))
}

/// Points and corner marks: `{x, y}` or `{x, y, "c"}` (a corner); `corners=`
/// a list of indices, `true` (all), or unset (by angle, `corner_angle=`).
type Marked = (Vec<(f32, f32)>, Vec<bool>);

fn marked_points(o: &Table) -> Result<Marked> {
    let src: Table = match o.get::<Value>("pts")? {
        Value::Table(t) => t,
        Value::Nil => o.clone(),
        v => return err(format!("outline pts: want {{{{x, y}}, ...}}, got {}", v.type_name())),
    };
    let mut pts = Vec::new();
    let mut marks = Vec::new();
    let mut any_mark = false;
    match src.get::<Value>(1)? {
        Value::Table(_) => {
            for p in src.sequence_values::<Table>() {
                let p = p?;
                pts.push((p.get::<f32>(1)?, p.get::<f32>(2)?));
                let m = match p.get::<Value>(3)? {
                    Value::Nil => false,
                    Value::Boolean(b) => {
                        any_mark = true;
                        b
                    }
                    Value::String(s) => {
                        any_mark = true;
                        s.to_str()?.starts_with('c')
                    }
                    v => return err(format!("outline point {}: a third element marks a corner (\"c\" or true), got {}", pts.len(), v.type_name())),
                };
                marks.push(m);
            }
        }
        Value::Nil => return err("outline: needs points, e.g. outline{{100,200}, {150,180,\"c\"}, {200,220}, char=\"firm\"}"),
        _ => pts = points(&Value::Table(src))?,
    }
    let n = pts.len();
    let corners = match o.get::<Value>("corners")? {
        Value::Nil if any_mark => marks,
        Value::Nil => {
            let deg = num(o, "corner_angle")?.unwrap_or(60.0);
            let closed = closed_of(o, n)?;
            paint::outline::auto_corners(&pts, closed, deg)
        }
        Value::Boolean(b) => vec![b; n],
        Value::Table(t) => {
            let mut c = vec![false; n];
            for i in t.sequence_values::<usize>() {
                let i = i?;
                if i == 0 || i > n {
                    return err(format!("corners: index {i} out of 1..{n}"));
                }
                c[i - 1] = true;
            }
            c
        }
        v => return err(format!("corners: a list of point indices, true or false, got {}", v.type_name())),
    };
    Ok((pts, corners))
}

fn closed_of(o: &Table, n: usize) -> Result<bool> {
    Ok(match (o.get::<Option<bool>>("closed")?, o.get::<Option<bool>>("open")?) {
        (Some(c), _) => c,
        (None, Some(op)) => !op,
        (None, None) => n >= 3,
    })
}

const OUTLINE_KEYS: &[&str] = &["pts", "corners", "corner_angle", "closed", "open", "char", "seed", "size", "amount", "lobe", "edge"];

fn outline(st: &S, o: Table) -> Result<OutlineU> {
    check_keys(&o, OUTLINE_KEYS, "outline")?;
    let (pts, corners) = marked_points(&o)?;
    if pts.len() < 2 {
        return err("outline: needs at least two points");
    }
    let closed = closed_of(&o, pts.len())?;
    let (ch, scale) = character(&o, &pts, "firm")?;
    let seed = seed_of(st, &o)?;
    Ok(OutlineU { o: Rc::new(Outline::draw(&pts, &corners, closed, ch, seed, Some(scale))), st: st.clone() })
}

const BODY_KEYS: &[&str] = &["spine", "widths", "limbs", "blend", "char", "seed", "size", "amount", "lobe", "edge"];

fn widths_of(v: Value, n: usize, what: &str) -> Result<Vec<f32>> {
    let w: Vec<f32> = match v {
        Value::Table(t) => t.sequence_values::<f32>().collect::<Result<_>>()?,
        Value::Number(x) => vec![x as f32; n],
        Value::Integer(x) => vec![x as f32; n],
        Value::Nil => return err(format!("{what}: needs widths (one per point, or one number)")),
        v => return err(format!("{what} widths: got {}", v.type_name())),
    };
    if w.len() != n {
        return err(format!("{what}: {} widths for {n} points", w.len()));
    }
    if w.iter().any(|&x| x.is_nan() || x <= 0.0) {
        return err(format!("{what}: widths must be > 0"));
    }
    Ok(w)
}

fn body_of(st: &S, o: Table) -> Result<OutlineU> {
    check_keys(&o, BODY_KEYS, "body_of")?;
    let spine = points(&o.get::<Value>("spine")?)?;
    if spine.is_empty() {
        return err("body_of: needs spine={{x, y}, ...} (one point is a blob)");
    }
    let mut bones = vec![Bone { widths: widths_of(o.get("widths")?, spine.len(), "body_of spine")?, pts: spine }];
    if let Some(ls) = o.get::<Option<Table>>("limbs")? {
        for (k, l) in ls.sequence_values::<Table>().enumerate() {
            let l = l?;
            // {pts={...}, widths={...}} or {{x,y}, {x,y}, widths=...} or {{x,y}, {x,y}} (width 1)
            let pts = match l.get::<Value>("pts")? {
                Value::Nil => points(&Value::Table(l.clone()))?,
                v => points(&v)?,
            };
            if pts.len() < 2 {
                return err(format!("body_of limb {}: needs at least two points", k + 1));
            }
            let w = match l.get::<Value>("widths")? {
                Value::Nil => match l.get::<Value>("width")? {
                    Value::Nil => return err(format!("body_of limb {}: needs widths={{...}} or width=", k + 1)),
                    v => widths_of(v, pts.len(), "body_of limb")?,
                },
                v => widths_of(v, pts.len(), "body_of limb")?,
            };
            bones.push(Bone { pts, widths: w });
        }
    }
    let all: Vec<(f32, f32)> = bones.iter().flat_map(|b| b.pts.iter().zip(&b.widths).flat_map(|(p, w)| [(p.0 - w * 0.5, p.1 - w * 0.5), (p.0 + w * 0.5, p.1 + w * 0.5)])).collect();
    let (ch, scale) = character(&o, &all, "soft")?;
    let blend = num(&o, "blend")?.unwrap_or(0.8).max(0.0);
    let seed = seed_of(st, &o)?;
    Ok(OutlineU { o: Rc::new(Outline::body(&bones, blend, ch, seed, Some(scale))), st: st.clone() })
}

const PAINT_KEYS: &[&str] = &["pressure", "shake", "clip", "dip", "every", "ramps"];

impl UserData for OutlineU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("scale", |_, o| Ok(o.o.scale));
        f.add_field_method_get("closed", |_, o| Ok(o.o.lines.first().is_some_and(|l| l.closed)));
        f.add_field_method_get("ramps", |_, o| Ok(vec![o.o.ch.ramps.0, o.o.ch.ramps.1]));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("mask", |_, o, ()| {
            // no lines at all (an inset that ate the shape) is an empty mask;
            // lines with none closed have no inside
            if !o.o.lines.is_empty() && !o.o.lines.iter().any(|l| l.closed) {
                return err("outline:mask(): this line is open; use :below() or :above() (or closed=true)");
            }
            Ok(wrap(o.o.mask(frame(&o.st)?)))
        });
        m.add_method("below", |_, o, bottom: Option<f32>| {
            let f = frame(&o.st)?;
            Ok(wrap(o.o.below(f, bottom.unwrap_or(f.height() + 1.0))))
        });
        m.add_method("above", |_, o, ()| Ok(wrap(o.o.above(frame(&o.st)?))));
        // o:band(width, taper?): a band along the line (taper 1: as wide as the pressure)
        m.add_method("band", |_, o, (w, taper): (f32, Option<f32>)| {
            if w.is_nan() || w <= 0.0 {
                return err("band(width): want > 0");
            }
            Ok(wrap(o.o.band(frame(&o.st)?, w, taper.unwrap_or(0.0).clamp(0.0, 1.0))))
        });
        // o:inset(d, amount?) / o:offset(d, amount?): a parallel outline
        m.add_method("inset", |_, o, (d, k): (f32, Option<f32>)| Ok(OutlineU { o: Rc::new(o.o.offset(-d, k.unwrap_or(0.4))), st: o.st.clone() }));
        m.add_method("offset", |_, o, (d, k): (f32, Option<f32>)| Ok(OutlineU { o: Rc::new(o.o.offset(d, k.unwrap_or(0.4))), st: o.st.clone() }));
        // o:path(i?): the drawn line as {{x, y}, ...} (a closed one doesn't repeat its first point)
        m.add_method("path", |lua, o, i: Option<usize>| {
            let l = o.o.lines.get(i.unwrap_or(1).saturating_sub(1)).ok_or_else(|| mlua::Error::runtime(format!("path: this outline has {} line(s)", o.o.lines.len())))?;
            pts_table(lua, &l.pts)
        });
        m.add_method("paths", |lua, o, ()| {
            let t = lua.create_table()?;
            for (i, l) in o.o.lines.iter().enumerate() {
                t.raw_set(i + 1, pts_table(lua, &l.pts)?)?;
            }
            Ok(t)
        });
        m.add_method("corners", |lua, o, ()| {
            let t = lua.create_table()?;
            if let Some(l) = o.o.lines.first() {
                for (i, &c) in l.corners.iter().enumerate() {
                    t.raw_set(i + 1, lua.create_sequence_from([l.pts[c].0, l.pts[c].1])?)?;
                }
            }
            Ok(t)
        });
        // o:strokes(): how the hand drew it, {{pts={{x,y},...}, pressure={...}}, ...}
        m.add_method("strokes", |lua, o, ()| {
            let t = lua.create_table()?;
            for (i, s) in o.o.strokes.iter().enumerate() {
                let st = lua.create_table()?;
                st.set("pts", pts_table(lua, &s.pts)?)?;
                st.set("pressure", lua.create_sequence_from(s.pressure.iter().copied())?)?;
                t.raw_set(i + 1, st)?;
            }
            Ok(t)
        });
        m.add_method("length", |_, o, ()| Ok(o.o.length()));
        // o:at(t) -> x, y, tx, ty, nx, ny (n points outward)
        m.add_method("at", |_, o, t: f32| {
            let (p, tg, n) = o.o.at(t).ok_or_else(|| mlua::Error::runtime("at: empty outline"))?;
            Ok((p.0, p.1, tg.0, tg.1, n.0, n.1))
        });
        // o:paint(brush, {pressure=1, shake=0.3, ramps=, clip=, dip={color, amount}, every=3})
        m.add_method("paint", |lua, o, (b, opts): (AnyUserData, Option<Table>)| {
            let opts = opts.unwrap_or(lua.create_table()?);
            check_keys(&opts, PAINT_KEYS, "outline:paint")?;
            let pk = num(&opts, "pressure")?.unwrap_or(1.0);
            let shake = num(&opts, "shake")?.unwrap_or(0.3);
            let clip = match opts.get::<Value>("clip")? {
                Value::Nil => None,
                v => {
                    mask_of(&v)?;
                    Some(v)
                }
            };
            let ramps: Option<Table> = opts.get("ramps")?;
            let dip: Option<Table> = opts.get("dip")?;
            let every = opts.get::<Option<usize>>("every")?.unwrap_or(3).max(1);
            let mut n = 0;
            for g in o.o.gestures(pk, shake) {
                if let Some(d) = &dip
                    && n % every == 0
                {
                    b.call_method::<()>("load", (d.get::<Value>(1)?, d.get::<Option<f32>>(2)?.unwrap_or(0.6)))?;
                }
                let so = lua.create_table()?;
                so.set("pressure", lua.create_sequence_from([g.pressure.0, g.pressure.1])?)?;
                match &ramps {
                    Some(r) => so.set("ramps", r.clone())?,
                    None => so.set("ramps", lua.create_sequence_from([g.attack, g.release])?)?,
                }
                so.set("swell", lua.create_sequence_from(g.swell.iter().copied())?)?;
                so.set("shake", g.shake)?;
                if let Some(c) = &clip {
                    so.set("clip", c.clone())?;
                }
                b.call_method::<()>("stroke", (pts_table(lua, &g.pts)?, so))?;
                n += 1;
            }
            Ok(n)
        });
        m.add_meta_method(MetaMethod::ToString, |_, o, ()| {
            let l = &o.o.lines;
            Ok(format!(
                "outline({} line(s), {}, length {:.0}, {} strokes, {} corners, scale {:.0})",
                l.len(),
                if l.first().is_some_and(|l| l.closed) { "closed" } else { "open" },
                o.o.length(),
                o.o.strokes.len(),
                l.iter().map(|l| l.corners.len()).sum::<usize>(),
                o.o.scale
            ))
        });
    }
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let s1 = st.clone();
    g.set("outline", lua.create_function(move |_, o: Table| outline(&s1, o))?)?;
    let s1 = st.clone();
    g.set("body_of", lua.create_function(move |_, o: Table| body_of(&s1, o))?)?;
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

    // review 4 (drawing), finding 5: an inset that eats a closed shape is an
    // empty outline, and its mask is empty (so a rim keeps the whole shape)
    #[test]
    fn an_inset_that_consumes_the_shape_masks_nothing() {
        run(r#"local o = outline{{100,100},{110,100},{110,110},{100,110}, corners=true, amount=0, edge=0, seed=1}
               local inset = o:inset(7)
               assert(#inset:paths() == 0, #inset:paths())
               local m = inset:mask()
               assert(m:area() == 0, m:area())
               local rim = o:mask() - m
               assert(math.abs(rim:area() - o:mask():area()) < 1e-3, rim:area())"#)
        .unwrap();
        // a nonempty open line still has no inside
        let e = run(r#"outline{{100,100},{200,120},{300,100}, closed=false}:mask()"#).unwrap_err();
        assert!(e.contains("this line is open"), "{e}");
    }

    // finding 6: explicit character options come before amount=, so
    // amount=0 is a clean curve whatever lobe= says
    #[test]
    fn amount_zero_is_clean_even_with_explicit_lobes() {
        run(r#"local function dev(o) local m = 0 for _, p in ipairs(o:path()) do m = math.max(m, math.abs(p[2] - 200)) end return m end
               for _, ch in ipairs({"soft", "firm"}) do
                 local clean = outline{{100,200},{500,200}, char=ch, amount=0, edge=0, seed=1}
                 local lobed = outline{{100,200},{500,200}, char=ch, amount=0, lobe=24, edge=0, seed=1}
                 assert(dev(clean) < 0.01, ch .. " clean " .. dev(clean))
                 assert(dev(lobed) < 0.01, ch .. " lobed " .. dev(lobed))
                 -- amount scales the explicit lobes: 1 shows them, 2 doubles them
                 local one = outline{{100,200},{500,200}, char=ch, amount=1, lobe=24, edge=0, seed=1}
                 local two = outline{{100,200},{500,200}, char=ch, amount=2, lobe=24, edge=0, seed=1}
                 assert(dev(one) > 2, ch .. " one " .. dev(one))
                 assert(dev(two) > dev(one) * 1.3, ch .. " two " .. dev(two) .. " one " .. dev(one))
               end"#)
        .unwrap();
    }
}
