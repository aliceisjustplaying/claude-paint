//! Rocks in Lua: `rock{...}` grows a solid behind an outline you draw (an
//! `outline{}` with the "broken" or "firm" character, plus any crack or
//! plane lines you draw inside it), lights it with your sun (or the world's)
//! and gives back its planes and the masks and stroke directions a painter
//! needs: the light and shadow families, core shadow, reflected light,
//! cracks and their occlusion, the seam at the foot, the shadow it casts on
//! the ground, the faces turned up to the sky (snow) and fields for work's
//! `angle=`. It paints nothing.
//!
//! ```lua
//! o = outline{{300,600,"c"}, {312,524}, {354,458,"c"}, {424,446}, {490,424,"c"}, {560,504,"c"}, {572,600,"c"}, char="broken", seed=3}
//! r = rock{outline=o, cracks={{{420,450}, {430,520}, {418,590}}}, kind="granite", sun={-1, -0.6, 0.4}, seed=5}
//! work(r:mask(), {hand="body", color=function(x, y) return mix("#4a4640", "#b8b0a0", r:value():at(x, y)) end,
//!   angle=r:field("plane"), clip=r:mask()})
//! ```

use crate::api::{S, check_keys, err, frame, num, points, seed_of, wrap};
use crate::draw_outline::{outline_corners, outline_path};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::form::{Light, fall_angle};
use paint::rock::{Rock, RockKind, RockSpec};
use std::sync::Arc;

pub struct RockU {
    r: Arc<Rock>,
    st: S,
}

/// A direction field read straight from a rock (for `work{angle=}`).
#[derive(Clone)]
pub struct RockFieldU {
    r: Arc<Rock>,
    kind: RockField,
}

#[derive(Clone, Copy)]
enum RockField {
    Fall,
    Across,
    Plane,
    Crack,
    Bed,
}

impl RockFieldU {
    /// The field as a closure the engine's threads can call; `fallback` off the rock.
    pub fn angle(&self, fallback: f32) -> Box<dyn Fn(f32, f32) -> f32 + Sync> {
        let (r, kind) = (self.r.clone(), self.kind);
        Box::new(move |x, y| {
            if r.sample(x, y).is_none() {
                return fallback;
            }
            match kind {
                RockField::Fall => r.fall(x, y),
                RockField::Across => r.across(x, y),
                RockField::Plane => r.plane_fall(x, y),
                RockField::Crack => r.along_crack(x, y),
                RockField::Bed => r.bedding(),
            }
        })
    }
}

impl UserData for RockFieldU {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::ToString, |_, f, ()| {
            Ok(format!(
                "rock field({})",
                match f.kind {
                    RockField::Fall => "fall",
                    RockField::Across => "across",
                    RockField::Plane => "plane",
                    RockField::Crack => "crack",
                    RockField::Bed => "bed",
                }
            ))
        });
        m.add_method("at", |_, f, (x, y): (f32, f32)| Ok(f.angle(std::f32::consts::FRAC_PI_2)(x, y)));
    }
}

fn line_of(v: &Value, what: &str) -> Result<Vec<(f32, f32)>> {
    match v {
        Value::UserData(u) => match outline_path(u) {
            Some((p, _)) => Ok(p),
            None => err(format!("{what}: want an outline{{}} or points {{{{x, y}}, ...}}")),
        },
        Value::Table(_) => points(v),
        o => err(format!("{what}: want an outline{{}} or points, got {}", o.type_name())),
    }
}

/// A list of lines: {line, line, ...} where a line is an outline{} or points
/// (a single line of points is accepted too).
fn lines_of(v: &Value, what: &str) -> Result<Vec<Vec<(f32, f32)>>> {
    match v {
        Value::Nil => Ok(Vec::new()),
        Value::UserData(_) => Ok(vec![line_of(v, what)?]),
        Value::Table(t) => {
            // {{x, y}, {x, y}}: one line; {{{x, y}, ...}, outline, ...}: several
            let first: Value = t.get(1)?;
            let one_line = match &first {
                Value::Table(p) => matches!(p.get::<Value>(1)?, Value::Number(_) | Value::Integer(_)),
                Value::Number(_) | Value::Integer(_) => true,
                _ => false,
            };
            if one_line {
                return Ok(vec![points(v)?]);
            }
            t.clone().sequence_values::<Value>().map(|l| line_of(&l?, what)).collect()
        }
        o => err(format!("{what}: want a list of lines (outline{{}} or points), got {}", o.type_name())),
    }
}

/// The light: nil (from the upper left, a little in front), {x, y, z}
/// toward the sun, a light table {from=, front=, ambient=, ...} as for
/// form{}, or a world (its sun).
fn light_of(v: &Value) -> Result<Light> {
    match v {
        Value::Nil => Ok(Light::new((-1.0, -0.65), 0.45).ambient(0.2).bounce(0.3, [0.45, 0.8, 0.3])),
        Value::UserData(u) => match u.borrow::<crate::world::WorldU>() {
            Ok(w) => Ok(w.w.light()),
            Err(_) => err("sun: want {x, y, z} toward the sun, a light {from=, front=, ...} or a world{}"),
        },
        Value::Table(t) => {
            if t.contains_key("from")? || t.contains_key("front")? || t.contains_key("ambient")? {
                crate::form::light_of(t)
            } else {
                let v: Vec<f32> = t.clone().sequence_values::<f32>().collect::<Result<_>>()?;
                if v.len() < 2 {
                    return err("sun: {x, y, z} toward the sun (x right, y down, z toward you)");
                }
                let l = Light::new((v[0], v[1]), v.get(2).copied().unwrap_or(0.4));
                Ok(l.ambient(0.2).bounce(0.3, [-l.dir[0] * 0.6, 0.8, 0.3]))
            }
        }
        o => err(format!("sun: want {{x, y, z}}, a light table or a world, got {}", o.type_name())),
    }
}

const KINDS: &str = "granite (or erratic), sandstone, chalk";
const ROCK_KEYS: &[&str] = &[
    "outline", "cracks", "planes", "corners", "kind", "sun", "seed", "round", "facets", "steep", "tilt", "bulge", "bed", "bed_tilt", "bed_recess", "crack_depth", "crack_width", "joints", "lumps",
    "grain", "flutes", "ground",
];

fn rock(st: &S, o: Table) -> Result<RockU> {
    check_keys(&o, ROCK_KEYS, "rock")?;
    let kname = o.get::<Option<String>>("kind")?.unwrap_or_else(|| "granite".into());
    let kind = RockKind::named(&kname).ok_or_else(|| mlua::Error::runtime(format!("rock kind {kname:?}: one of {KINDS}")))?;
    let mut spec = RockSpec::of(kind);
    macro_rules! over {
        ($($f:ident),*) => {$( if let Some(v) = num(&o, stringify!($f))? { spec.$f = v; } )*};
    }
    over!(round, bulge, bed, bed_tilt, bed_recess, crack_depth, crack_width, joints, lumps, grain, flutes, ground);
    if let Some(n) = o.get::<Option<usize>>("facets")? {
        spec.facets = n.min(40);
    }
    for (k, slot) in [("steep", &mut spec.steep), ("tilt", &mut spec.tilt)] {
        if let Some(v) = o.get::<Option<Vec<f32>>>(k)? {
            *slot = (v.first().copied().unwrap_or(slot.0), v.get(1).copied().unwrap_or(slot.1));
        }
    }
    let ov: Value = o.get("outline")?;
    if ov.is_nil() {
        return err("rock: needs outline= (a closed outline{} with char=\"broken\" or \"firm\", or points)");
    }
    let outline = line_of(&ov, "rock outline")?;
    if outline.len() < 3 {
        return err("rock outline: at least three points (a closed shape)");
    }
    let mut corners = match &ov {
        Value::UserData(u) => outline_corners(u).unwrap_or_default(),
        _ => Vec::new(),
    };
    if let Value::Table(_) = o.get::<Value>("corners")? {
        corners.extend(points(&o.get::<Value>("corners")?)?);
    }
    let cracks = lines_of(&o.get::<Value>("cracks")?, "rock cracks")?;
    let planes = lines_of(&o.get::<Value>("planes")?, "rock planes")?;
    let light = light_of(&o.get::<Value>("sun")?)?;
    let seed = seed_of(st, &o)?;
    let r = Rock::grow(&outline, &corners, &cracks, &planes, &spec, light, seed);
    Ok(RockU { r: Arc::new(r), st: st.clone() })
}

fn pts_table(lua: &Lua, pts: &[(f32, f32)]) -> Result<Table> {
    let t = lua.create_table_with_capacity(pts.len(), 0)?;
    for (i, p) in pts.iter().enumerate() {
        t.raw_set(i + 1, lua.create_sequence_from([p.0, p.1])?)?;
    }
    Ok(t)
}

impl UserData for RockU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("kind", |_, r| Ok(r.r.kind.name()));
        f.add_field_method_get("r", |_, r| Ok(r.r.r));
        f.add_field_method_get("base", |_, r| Ok(r.r.base));
        f.add_field_method_get("bounds", |lua, r| {
            let b = r.r.bounds;
            lua.create_sequence_from([b.0, b.1, b.2, b.3])
        });
        f.add_field_method_get("outline", |lua, r| pts_table(lua, &r.r.outline));
        f.add_field_method_get("foot", |lua, r| pts_table(lua, &r.r.foot));
        // {{pts=, crack=, grown=}, ...}: the drawn lines and the joints grown from corners
        f.add_field_method_get("seams", |lua, r| {
            let out = lua.create_table()?;
            for (i, s) in r.r.seams.iter().enumerate() {
                let t = lua.create_table()?;
                t.set("pts", pts_table(lua, &s.pts)?)?;
                t.set("crack", s.crack)?;
                t.set("grown", s.grown)?;
                out.raw_set(i + 1, t)?;
            }
            Ok(out)
        });
        // {{kind=, n=, fall=, turn=, value=, up=, area=}, ...} in plane order (r:plane(i))
        f.add_field_method_get("planes", |lua, r| {
            let areas = r.r.plane_areas();
            let out = lua.create_table()?;
            for (i, p) in r.r.planes.iter().enumerate() {
                let t = lua.create_table()?;
                t.set("kind", p.kind.name())?;
                t.set("n", lua.create_sequence_from(p.n)?)?;
                t.set("fall", fall_angle(p.n))?;
                let s = r.r.light.shade(p.n, 0.0);
                t.set("turn", s.turn)?;
                t.set("value", s.value)?;
                t.set("up", (-p.n[1]).max(0.0))?;
                let cells = areas.iter().find(|a| a.0 == i).map_or(0, |a| a.1);
                t.set("area", cells)?;
                out.raw_set(i + 1, t)?;
            }
            Ok(out)
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("mask", |_, r, ()| Ok(wrap(r.r.mask(frame(&r.st)?))));
        m.add_method("lit", |_, r, soft: Option<f32>| Ok(wrap(r.r.lit(frame(&r.st)?, soft.unwrap_or(0.12)))));
        m.add_method("shadow", |_, r, soft: Option<f32>| Ok(wrap(r.r.shadow(frame(&r.st)?, soft.unwrap_or(0.12)))));
        m.add_method("halftone", |_, r, ()| Ok(wrap(r.r.halftone(frame(&r.st)?))));
        m.add_method("core", |_, r, ()| Ok(wrap(r.r.core(frame(&r.st)?))));
        m.add_method("reflected", |_, r, ()| Ok(wrap(r.r.reflected(frame(&r.st)?))));
        m.add_method("value", |_, r, ()| Ok(wrap(r.r.value(frame(&r.st)?))));
        m.add_method("occlusion", |_, r, ()| Ok(wrap(r.r.occlusion(frame(&r.st)?))));
        m.add_method("cracks", |_, r, ()| Ok(wrap(r.r.cracks(frame(&r.st)?))));
        m.add_method("up", |_, r, (lo, hi): (Option<f32>, Option<f32>)| Ok(wrap(r.r.up(frame(&r.st)?, lo.unwrap_or(0.35), hi.unwrap_or(0.7)))));
        m.add_method("plane", |_, r, i: usize| {
            if i == 0 || i > r.r.planes.len() {
                return err(format!("rock:plane({i}): planes are 1..{}", r.r.planes.len()));
            }
            Ok(wrap(r.r.plane(frame(&r.st)?, i - 1)))
        });
        m.add_method("arrises", |_, r, span: Option<f32>| Ok(wrap(r.r.arrises(frame(&r.st)?, span.unwrap_or(2.0)))));
        // r:contact{inside=4, outside=6}
        m.add_method("contact", |_, r, o: Option<Table>| {
            let (mut i, mut out) = (r.r.r * 0.05 + 1.5, r.r.r * 0.07 + 2.0);
            if let Some(o) = &o {
                check_keys(o, &["inside", "outside"], "rock:contact")?;
                i = num(o, "inside")?.unwrap_or(i);
                out = num(o, "outside")?.unwrap_or(out);
            }
            Ok(wrap(r.r.contact(frame(&r.st)?, i, out)))
        });
        m.add_method("cast", |_, r, ()| Ok(wrap(r.r.cast(frame(&r.st)?))));
        // r:snow{amount=0.6, depth=, seed=}
        m.add_method("snow", |_, r, o: Option<Table>| {
            let (mut amount, mut depth, mut seed) = (0.6, r.r.r * 0.05 + 1.0, 1u32);
            if let Some(o) = &o {
                check_keys(o, &["amount", "depth", "seed"], "rock:snow")?;
                amount = num(o, "amount")?.unwrap_or(amount);
                depth = num(o, "depth")?.unwrap_or(depth);
                seed = o.get::<Option<u32>>("seed")?.unwrap_or(seed);
            }
            Ok(wrap(r.r.snow(frame(&r.st)?, amount, depth, seed)))
        });
        // r:field("fall" | "across" | "plane" | "crack" | "bed"): an angle field for work{angle=}
        m.add_method("field", |_, r, kind: String| {
            let kind = match kind.as_str() {
                "fall" => RockField::Fall,
                "across" => RockField::Across,
                "plane" => RockField::Plane,
                "crack" => RockField::Crack,
                "bed" => RockField::Bed,
                o => return err(format!("rock:field {o:?}: \"fall\", \"across\", \"plane\", \"crack\" or \"bed\"")),
            };
            Ok(RockFieldU { r: r.r.clone(), kind })
        });
        // r:sample(x, y): nil off the rock, else {n, z, plane, turn, direct, cast, bounce, sky, value, lit, core, reflected, ao, crack, up}
        m.add_method("sample", |lua, r, (x, y): (f32, f32)| {
            let Some(s) = r.r.sample(x, y) else { return Ok(Value::Nil) };
            let t = lua.create_table()?;
            t.set("n", lua.create_sequence_from(s.n)?)?;
            t.set("z", s.z)?;
            t.set("plane", s.plane + 1)?;
            t.set("turn", s.shade.turn)?;
            t.set("direct", s.shade.direct)?;
            t.set("cast", s.shade.cast)?;
            t.set("bounce", s.shade.bounce)?;
            t.set("sky", s.shade.sky)?;
            t.set("value", s.value())?;
            t.set("lit", s.shade.lit(0.12))?;
            t.set("core", s.core())?;
            t.set("reflected", s.reflected())?;
            t.set("ao", s.ao)?;
            t.set("crack", s.crack)?;
            t.set("up", s.up())?;
            Ok(Value::Table(t))
        });
        m.add_method("bend", |_, r, (x, y, span): (f32, f32, Option<f32>)| Ok(r.r.bend(x, y, span.unwrap_or(2.0))));
        m.add_meta_method(MetaMethod::ToString, |_, r, ()| {
            let k = &r.r;
            let big = k.plane_areas().iter().filter(|(_, n)| *n > 200).count();
            Ok(format!(
                "rock({}, {:.0}x{:.0}, R {:.0}, {} planes ({} sizable), {} lines ({} grown), lit {:.0}%)",
                k.kind.name(),
                k.bounds.2 - k.bounds.0,
                k.bounds.3 - k.bounds.1,
                k.r,
                k.planes.len(),
                big,
                k.seams.len(),
                k.seams.iter().filter(|s| s.grown).count(),
                k.lit_share() * 100.0
            ))
        });
    }
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let s1 = st.clone();
    lua.globals().set("rock", lua.create_function(move |_, o: Table| rock(&s1, o))?)?;
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
    fn a_rock_grows_from_a_drawn_outline() {
        let out = run(r##"local o = outline{{40,150,"c"}, {46,110}, {70,80,"c"}, {110,72}, {150,86,"c"}, {168,120}, {166,150,"c"}, char="broken", seed=3}
               local r = rock{outline=o, cracks={{{100,80}, {104,110}, {98,140}}}, kind="granite", sun={-1, -0.6, 0.4}, seed=5}
               local m = r:mask():area()
               assert(m > 1000, m)
               local lit, sh = r:lit():area(), r:shadow():area()
               assert(lit > 0.1 * m and sh > 0.1 * m, lit .. " " .. sh)
               assert(r:cracks():area() > 0)
               assert(r:up():area() > 0)
               assert(r:cast():area() > 0)
               assert(r:contact():area() > 0)
               assert(#r.planes > 8)
               local s = r:sample(100, 110)
               assert(s and s.plane >= 1 and s.value >= 0)
               local f = r:field("plane")
               work(r:mask(), {hand="body", color="#77706a", angle=f, clip=r:mask(), coverage=1})
               local s2 = rock{outline={{200,150},{210,120},{240,110},{260,130},{258,150}}, kind="sandstone", seed=2}
               print(tostring(r), tostring(s2))"##)
        .unwrap();
        assert!(out.contains("rock(granite") && out.contains("rock(sandstone"), "{out}");
        let e = run(r#"rock{outline={{0,0},{10,0},{5,8}}, kind="basalt"}"#).unwrap_err();
        assert!(e.contains("granite"), "{e}");
    }
}
