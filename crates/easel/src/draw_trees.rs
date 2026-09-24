//! Broadleaved trees in Lua: `tree_in{...}` grows an oak, beech, lime,
//! birch or pollard willow into a crown you draw (an `outline{}` or a few
//! points) from a trunk line you draw, and `tree_group{...}` makes a group
//! of field trees in depth from a few drawn crowns. Geometry only: limbs
//! with widths, leaf clumps by depth, lit/shade/gap masks, and the hooked
//! touches a pointed brush lays on the clumps (`t:paint(b, {...})`).
//!
//! ```lua
//! CROWN = outline{{300,120},{420,100},{520,160},{540,260,"c"},{430,320},{300,330},{190,280,"c"},{200,170}, char="soft", seed=2}
//! oak = tree_in{crown=CROWN, trunk={{372,560},{366,420},{380,330}}, species="oak", season="summer", seed=4}
//! oak:paint_wood(brush("round", 2), {color="#3a332c"})
//! work(oak:leaves(), {hand="hatch", tool="round 1.6", color="#2c3624", clip=oak:leaves()})
//! oak:paint(brush("round", oak.touch_w), {color="#6b7a44", lit={0.55, 1}})
//! ```

use crate::api::{S, check_keys, err, frame, mask_of, num, points, seed_of, wrap};
use crate::draw_outline::outline_path;
use mlua::{AnyUserData, Lua, MetaMethod, ObjectLike, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::broadleaf::{Clump, Group, GroupSpec, Limb, Season, Species, Touch, Tree};
use paint::{Frame, Mask};
use std::rc::Rc;

#[derive(Clone)]
pub struct TreeU {
    t: Rc<Tree>,
    season: String,
    haze: f32,
    scale: f32,
    /// Share of the fine wood drawn as lines (the rest is a tone): the
    /// default for `paint_wood` and `twig_mass`.
    detail: f32,
    st: S,
}

/// The default share of the fine wood drawn as lines (see `Tree::drawn`):
/// a bare tree draws about a third of it and leaves the rest to a tone; in
/// leaf, the leaves carry the fine structure and every twig is drawn.
fn detail_for(season: &Season) -> f32 {
    0.35 + 0.65 * season.leaf.clamp(0.0, 1.0)
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

/// `sun=`: {x, y, z} toward the sun (x right, y down, z to you), or a
/// world{} (its sun, turned into canvas axes).
fn sun_of(o: &Table) -> Result<[f32; 3]> {
    Ok(match o.get::<Value>("sun")? {
        Value::Nil => [-0.55, -0.75, 0.35],
        Value::Table(t) => [t.get::<Option<f32>>(1)?.unwrap_or(-0.55), t.get::<Option<f32>>(2)?.unwrap_or(-0.75), t.get::<Option<f32>>(3)?.unwrap_or(0.35)],
        Value::UserData(u) => match u.borrow::<crate::world::WorldU>() {
            Ok(w) => {
                let s = w.w.sun;
                let d = if s.up() { s.dir() } else { s.glow_dir() };
                // world: x right, y up, z away from the viewer
                [d[0], -d[1], -d[2]]
            }
            Err(_) => return err("sun: want {x, y, z} (toward the sun: x right, y down, z to you) or a world{}"),
        },
        v => return err(format!("sun: want {{x, y, z}} or a world{{}}, got {}", v.type_name())),
    })
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

const SPECIES: &str = "oak, beech, lime (or linden), birch, willow (a pollard)";
const SEASONS: &str = "spring, summer, autumn, late_autumn, winter";

const SPECIES_KEYS: &[&str] = &[
    "step", "density", "influence", "kill", "up", "out", "crook", "kink", "inertia", "shell", "voids", "void_size", "depth", "girth", "twig_w", "twigs", "twig_len",
    "twig_spread", "twig_droop", "twig_zig", "twig_along", "clump", "squash", "hang", "fill", "ragged", "leafiness", "leafy_w", "touch", "touch_w", "hook", "droop",
    "flat", "touches", "marcescent", "scaffold", "smooth", "pipe", "leader",
];

fn species_of(o: &Table, name: &str) -> Result<Species> {
    let mut s = Species::named(name).ok_or_else(|| mlua::Error::runtime(format!("species {name:?}: one of {SPECIES}")))?;
    macro_rules! over {
        ($($f:ident),*) => {$( if let Some(v) = num(o, stringify!($f))? { s.$f = v; } )*};
    }
    over!(step, density, influence, kill, up, out, crook, kink, inertia, shell, voids, void_size, depth, twig_w, twigs, twig_len, twig_spread, twig_droop, twig_zig, twig_along,
        clump, squash, hang, fill, ragged, leafiness, leafy_w, touch, touch_w, hook, droop, flat, touches, marcescent, pipe, leader);
    if let Some(v) = num(o, "girth")? {
        s.trunk = v;
    }
    if let Some(v) = o.get::<Option<u32>>("smooth")? {
        s.smooth = v.min(8);
    }
    if let Some(v) = o.get::<Option<Vec<u32>>>("scaffold")? {
        s.scaffold = (v.first().copied().unwrap_or(3), v.get(1).copied().unwrap_or(6));
    }
    s.step = s.step.clamp(0.005, 0.2);
    Ok(s)
}

fn season_of(o: &Table) -> Result<(String, Season)> {
    let name = o.get::<Option<String>>("season")?.unwrap_or_else(|| "summer".into());
    let mut s = Season::named(&name).ok_or_else(|| mlua::Error::runtime(format!("season {name:?}: one of {SEASONS}")))?;
    if let Some(v) = num(o, "leaf")? {
        s.leaf = v.clamp(0.0, 1.0);
    }
    if let Some(v) = num(o, "turn")? {
        s.turn = v.clamp(0.0, 1.0);
    }
    Ok((name, s))
}

fn trunk_of(v: Value) -> Result<Option<Vec<(f32, f32)>>> {
    Ok(match v {
        Value::Nil => None,
        v => {
            let p = line_of(&v, "trunk")?;
            if p.is_empty() {
                return err("trunk: want the foot first, then up to where it forks: {{x, y}, ...}");
            }
            Some(p)
        }
    })
}

const TREE_KEYS: &[&str] = &["crown", "trunk", "species", "season", "sun", "seed", "leaf", "turn", "detail"];

fn tree_in(st: &S, o: Table) -> Result<TreeU> {
    let keys: Vec<&str> = TREE_KEYS.iter().chain(SPECIES_KEYS).copied().collect();
    check_keys(&o, &keys, "tree_in")?;
    let name = o.get::<Option<String>>("species")?.unwrap_or_else(|| "oak".into());
    let sp = species_of(&o, &name)?;
    let (sname, season) = season_of(&o)?;
    let crown = line_of(&o.get::<Value>("crown")?, "tree_in crown")?;
    if crown.len() < 3 {
        return err("tree_in crown: needs at least three points (a closed shape round the crown)");
    }
    let trunk = trunk_of(o.get::<Value>("trunk")?)?;
    let seed = seed_of(st, &o)?;
    let t = Tree::grow(&crown, trunk.as_deref(), &sp, &season, sun_of(&o)?, seed);
    let detail = num(&o, "detail")?.unwrap_or(detail_for(&season)).clamp(0.0, 1.0);
    Ok(TreeU { t: Rc::new(t), season: sname, haze: 0.0, scale: 1.0, detail, st: st.clone() })
}

/// Which clumps or touches: {depth={lo, hi} (-1 back .. 1 front), lit={lo, hi}, dead=bool, turn={lo, hi}}
struct Pick {
    depth: (f32, f32),
    lit: (f32, f32),
    turn: (f32, f32),
    dead: Option<bool>,
}

impl Pick {
    fn of(o: Option<&Table>) -> Result<Pick> {
        let Some(o) = o else { return Ok(Pick { depth: (-2.0, 2.0), lit: (0.0, 1.0), turn: (0.0, 1.0), dead: None }) };
        Ok(Pick { depth: range_of(o, "depth", (-2.0, 2.0))?, lit: range_of(o, "lit", (0.0, 1.0))?, turn: range_of(o, "turn", (0.0, 1.0))?, dead: o.get("dead")? })
    }
    fn half_open(v: f32, r: (f32, f32)) -> bool {
        v >= r.0 && (v < r.1 || r.1 >= 1.0)
    }
    fn clump(&self, c: &Clump) -> bool {
        c.depth >= self.depth.0 && c.depth <= self.depth.1 && Self::half_open(c.lit, self.lit) && Self::half_open(c.turn, self.turn) && self.dead.is_none_or(|d| d == c.dead)
    }
    fn touch(&self, t: &Tree, s: &Touch) -> bool {
        let c = &t.clumps[s.clump];
        c.depth >= self.depth.0 && c.depth <= self.depth.1 && Self::half_open(s.lit, self.lit) && Self::half_open(s.turn, self.turn) && self.dead.is_none_or(|d| d == s.dead)
    }
}

fn touch_table(lua: &Lua, t: &Tree, s: &Touch) -> Result<Table> {
    let o = lua.create_table()?;
    o.set("pts", pts_table(lua, &s.pts)?)?;
    o.set("w", s.w)?;
    o.set("lit", s.lit)?;
    o.set("z", s.z)?;
    o.set("depth", t.clumps[s.clump].depth)?;
    o.set("turn", s.turn)?;
    o.set("dead", s.dead)?;
    o.set("clump", s.clump + 1)?;
    Ok(o)
}

const PAINT_KEYS: &[&str] = &["color", "load", "every", "pressure", "ramps", "shake", "clip", "lit", "depth", "turn", "dead", "fit", "share"];

/// Lay leaf touches with a brush: `color` a color or function(touch) -> color.
fn lay(lua: &Lua, trees: &[Rc<Tree>], b: &AnyUserData, opts: Option<Table>) -> Result<usize> {
    let opts = opts.unwrap_or(lua.create_table()?);
    check_keys(&opts, PAINT_KEYS, "tree:paint")?;
    let pick = Pick::of(Some(&opts))?;
    let color = opts.get::<Value>("color")?;
    if color.is_nil() {
        return err("tree:paint: needs color= (a color, or function(touch) returning one)");
    }
    let load = num(&opts, "load")?.unwrap_or(0.8);
    let every = opts.get::<Option<usize>>("every")?.unwrap_or(10).max(1);
    let press = range_of(&opts, "pressure", (0.75, 0.05))?;
    let ramps = range_of(&opts, "ramps", (0.1, 0.55))?;
    let shake = num(&opts, "shake")?.unwrap_or(0.4);
    let share = num(&opts, "share")?.unwrap_or(1.0).clamp(0.0, 1.0);
    let fit = opts.get::<Option<bool>>("fit")?.unwrap_or(true);
    let clip = opts.get::<Value>("clip")?;
    if !clip.is_nil() {
        mask_of(&clip)?;
    }
    let bw: f32 = b.get("width")?;
    let mut n = 0;
    let mut seen = 0usize;
    for t in trees {
        for s in t.touches.iter().filter(|s| pick.touch(t, s)) {
            seen += 1;
            // a deterministic share of them
            if share < 1.0 && paint::rng::hash2(seen as i64, 7, t.seed) > share {
                continue;
            }
            if n % every == 0 {
                let c = match &color {
                    Value::Function(g) => g.call::<Value>(touch_table(lua, t, s)?)?,
                    c => c.clone(),
                };
                b.call_method::<()>("reload", (c, load))?;
            }
            let so = lua.create_table()?;
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

const WOOD_KEYS: &[&str] = &["color", "load", "every", "min", "max", "twigs", "pressure", "ramps", "shake", "clip", "detail"];

/// Lay the wood between `min` and `max` wide (the local width, so a stout
/// limb's thin end is in the thin band) as strokes that start inside the
/// wood they leave from, pressed to the wood's width along the way (the
/// pressure follows the width through `swell` knots), lifting off only at
/// the tips. The fine wood is drawn at `detail` (see `Tree::drawn`).
fn lay_wood(lua: &Lua, t: &Tree, b: &AnyUserData, opts: Option<Table>, detail: f32) -> Result<usize> {
    let opts = opts.unwrap_or(lua.create_table()?);
    check_keys(&opts, WOOD_KEYS, "tree:paint_wood")?;
    let color = opts.get::<Value>("color")?;
    if color.is_nil() {
        return err("tree:paint_wood: needs color= (a color, or function(limb) returning one)");
    }
    let load = num(&opts, "load")?.unwrap_or(0.85);
    let every = opts.get::<Option<usize>>("every")?.unwrap_or(5).max(1);
    let lo = num(&opts, "min")?.unwrap_or(0.0);
    let hi = num(&opts, "max")?.unwrap_or(f32::MAX);
    let twigs = opts.get::<Option<bool>>("twigs")?.unwrap_or(true);
    let detail = num(&opts, "detail")?.unwrap_or(detail).clamp(0.0, 1.0);
    let ramps = match opts.get::<Value>("ramps")? {
        Value::Nil => None,
        _ => Some(range_of(&opts, "ramps", (0.0, 0.12))?),
    };
    let shake = num(&opts, "shake")?.unwrap_or(0.3);
    let end = num(&opts, "pressure")?;
    let clip = opts.get::<Value>("clip")?;
    let bw: f32 = b.get("width")?;
    let press = |w: f32| -> Result<f32> { b.call_method::<f32>("pressure_for", w.min(bw)) };
    let mut n = 0;
    for s in t.wood_strokes(lo, hi, detail) {
        let l = &t.limbs[s.limb];
        if !twigs && l.twig {
            continue;
        }
        let m = if twigs { s.pts.len() } else { s.own };
        if m < 2 {
            continue;
        }
        let (pts, w) = (&s.pts[..m], &s.w[..m]);
        let tip = s.tip && m == s.pts.len();
        if n % every == 0 || b.call_method::<f32>("fullness", ())? < 0.3 {
            let c = match &color {
                Value::Function(g) => g.call::<Value>(limb_table(lua, s.limb, l)?)?,
                c => c.clone(),
            };
            b.call_method::<()>("reload", (c, load))?;
        }
        // the pressure along the stroke, from the width at even steps of its length
        let mut arc = vec![0.0f32; m];
        for k in 1..m {
            arc[k] = arc[k - 1] + ((pts[k].0 - pts[k - 1].0).powi(2) + (pts[k].1 - pts[k - 1].1).powi(2)).sqrt();
        }
        let total = arc[m - 1].max(1e-6);
        let nk = ((total / (0.75 * bw).max(0.5)).ceil() as usize + 1).clamp(2, 24);
        let mut knots = Vec::with_capacity(nk);
        let mut j = 0;
        for q in 0..nk {
            let d = total * q as f32 / (nk - 1) as f32;
            while j + 2 < m && arc[j + 1] < d {
                j += 1;
            }
            let f = ((d - arc[j]) / (arc[j + 1] - arc[j]).max(1e-6)).clamp(0.0, 1.0);
            let wd = w[j] + (w[j + 1] - w[j]) * f;
            knots.push(press(wd)?.clamp(0.05, 1.0));
        }
        if tip {
            if let Some(e) = end {
                *knots.last_mut().unwrap() = e.clamp(0.02, 1.0);
            }
        }
        let (ra, rr) = ramps.unwrap_or((0.0, if tip { 0.12 } else { 0.0 }));
        let so = lua.create_table()?;
        so.set("pressure", lua.create_sequence_from([1.0f32, 1.0])?)?;
        so.set("swell", lua.create_sequence_from(knots)?)?;
        so.set("ramps", lua.create_sequence_from([ra, if tip { rr } else { 0.0 }])?)?;
        so.set("shake", shake)?;
        if !clip.is_nil() {
            so.set("clip", clip.clone())?;
        }
        b.call_method::<()>("stroke", (pts_table(lua, pts)?, so))?;
        n += 1;
    }
    Ok(n)
}

fn limb_table(lua: &Lua, i: usize, l: &Limb) -> Result<Table> {
    let t = lua.create_table()?;
    t.set("i", i + 1)?;
    t.set("pts", pts_table(lua, &l.pts)?)?;
    t.set("w", lua.create_sequence_from(l.w.iter().copied())?)?;
    t.set("z", lua.create_sequence_from(l.z.iter().copied())?)?;
    t.set("order", l.order)?;
    t.set("parent", l.parent.map(|p| p + 1))?;
    t.set("twig", l.twig)?;
    Ok(t)
}

fn gaps(t: &Tree, f: Frame, reach: f32) -> Mask {
    let m = t.mask(f);
    m.dilate(reach).erode(reach).subtract(&m).mul(&t.crown_mask(f).dilate(reach * 0.5))
}

impl UserData for TreeU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("species", |_, o| Ok(o.t.species.clone()));
        f.add_field_method_get("season", |_, o| Ok(o.season.clone()));
        f.add_field_method_get("foot", |lua, o| pt(lua, o.t.foot));
        f.add_field_method_get("fork", |lua, o| pt(lua, o.t.fork));
        f.add_field_method_get("height", |_, o| Ok(o.t.height));
        f.add_field_method_get("step", |_, o| Ok(o.t.step));
        f.add_field_method_get("grain", |_, o| Ok(o.t.grain));
        f.add_field_method_get("touch_w", |_, o| Ok(o.t.touch_w));
        f.add_field_method_get("haze", |_, o| Ok(o.haze));
        f.add_field_method_get("scale", |_, o| Ok(o.scale));
        f.add_field_method_get("detail", |_, o| Ok(o.detail));
        f.add_field_method_get("crown", |lua, o| pts_table(lua, &o.t.crown));
        f.add_field_method_get("bounds", |lua, o| {
            let b = o.t.bounds();
            lua.create_sequence_from([b.0, b.1, b.2, b.3])
        });
        // trunk first, parents before children: {pts, w, z, order, parent, twig}
        f.add_field_method_get("limbs", |lua, o| {
            let out = lua.create_table()?;
            for (i, l) in o.t.limbs.iter().enumerate() {
                out.raw_set(i + 1, limb_table(lua, i, l)?)?;
            }
            Ok(out)
        });
        // back to front: {x, y, z, depth, r, squash, tilt, fill, lit, shade, turn, dead, limb}
        f.add_field_method_get("clumps", |lua, o| {
            let out = lua.create_table()?;
            for (i, c) in o.t.clumps.iter().enumerate() {
                let t = lua.create_table()?;
                t.set("x", c.at.0)?;
                t.set("y", c.at.1)?;
                t.set("z", c.z)?;
                t.set("depth", c.depth)?;
                t.set("r", c.r)?;
                t.set("squash", c.squash)?;
                t.set("tilt", c.tilt)?;
                t.set("fill", c.fill)?;
                t.set("lit", c.lit)?;
                t.set("shade", c.shade)?;
                t.set("turn", c.turn)?;
                t.set("dead", c.dead)?;
                t.set("limb", c.limb + 1)?;
                out.raw_set(i + 1, t)?;
            }
            Ok(out)
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // t:leaves() or t:leaves{depth={lo, hi}, lit={lo, hi}, turn=, dead=}: clumps by depth or light
        m.add_method("leaves", |_, o, opts: Option<Table>| {
            if let Some(t) = &opts {
                check_keys(t, &["depth", "lit", "turn", "dead"], "tree:leaves")?;
            }
            let p = Pick::of(opts.as_ref())?;
            Ok(wrap(o.t.leaves_where(frame(&o.st)?, |c| p.clump(c))))
        });
        m.add_method("light", |_, o, ()| Ok(wrap(o.t.light(frame(&o.st)?))));
        m.add_method("lit", |_, o, from: Option<f32>| {
            let from = from.unwrap_or(0.55);
            Ok(wrap(o.t.light(frame(&o.st)?).map(move |v| paint::smoothstep(from - 0.04, from + 0.04, v))))
        });
        m.add_method("shade", |_, o, from: Option<f32>| {
            let f = frame(&o.st)?;
            let from = from.unwrap_or(0.55);
            let lit = o.t.light(f).map(move |v| paint::smoothstep(from - 0.04, from + 0.04, v));
            Ok(wrap(o.t.leaves(f).subtract(&lit)))
        });
        m.add_method("gaps", |_, o, reach: Option<f32>| Ok(wrap(gaps(&o.t, frame(&o.st)?, reach.unwrap_or(o.t.grain * 1.6)))));
        // t:wood() all; t:wood(lo, hi): limbs whose base width is in [lo, hi)
        m.add_method("wood", |_, o, (lo, hi): (Option<f32>, Option<f32>)| Ok(wrap(o.t.wood(frame(&o.st)?, lo.unwrap_or(0.0), hi.unwrap_or(f32::MAX)))));
        m.add_method("trunk", |_, o, ()| Ok(wrap(o.t.trunk(frame(&o.st)?))));
        m.add_method("mask", |_, o, ()| Ok(wrap(o.t.mask(frame(&o.st)?))));
        m.add_method("crown_mask", |_, o, ()| Ok(wrap(o.t.crown_mask(frame(&o.st)?))));
        m.add_method("touches", |lua, o, opts: Option<Table>| {
            if let Some(t) = &opts {
                check_keys(t, &["depth", "lit", "turn", "dead"], "tree:touches")?;
            }
            let p = Pick::of(opts.as_ref())?;
            let out = lua.create_table()?;
            let mut i = 0;
            for s in o.t.touches.iter().filter(|s| p.touch(&o.t, s)) {
                i += 1;
                out.raw_set(i, touch_table(lua, &o.t, s)?)?;
            }
            Ok(out)
        });
        // t:paint(brush, {color=, lit=, depth=, turn=, dead=, share=1, every=10, load=0.8, pressure={0.75, 0.05}, ramps=, shake=, clip=, fit=true})
        m.add_method("paint", |lua, o, (b, opts): (AnyUserData, Option<Table>)| lay(lua, std::slice::from_ref(&o.t), &b, opts));
        // t:paint_wood(brush, {color=, min=, max=, twigs=true, every=5, load=, pressure=, ramps=, shake=, clip=})
        m.add_method("paint_wood", |lua, o, (b, opts): (AnyUserData, Option<Table>)| lay_wood(lua, &o.t, &b, opts, o.detail));
        // t:twig_mass(detail): the fine wood not drawn as lines, as a soft tone (0..1)
        m.add_method("twig_mass", |_, o, detail: Option<f32>| Ok(wrap(o.t.twig_mass(frame(&o.st)?, detail.unwrap_or(o.detail).clamp(0.0, 1.0)))));
        m.add_meta_method(MetaMethod::ToString, |_, o, ()| {
            let t = &o.t;
            Ok(format!(
                "tree_in({}, {}, crown {:.0} tall, {} limbs ({} twigs), {} clumps, {} touches, trunk {:.1} wide, step {:.1}, touch {:.2})",
                t.species,
                o.season,
                t.height,
                t.limbs.len(),
                t.limbs.iter().filter(|l| l.twig).count(),
                t.clumps.len(),
                t.touches.len(),
                t.limbs.first().map_or(0.0, |l| l.w[0]),
                t.step,
                t.touch_w
            ))
        });
    }
}

// ---------------------------------------------------------------- groups

pub struct GroupU {
    g: Rc<Group>,
    trees: Vec<Rc<Tree>>,
    season: String,
    detail: f32,
    st: S,
}

const GROUP_KEYS: &[&str] = &["crowns", "trunks", "species", "season", "foot", "count", "horizon", "recede", "air", "spread", "narrow", "sun", "seed", "leaf", "turn", "detail"];

fn tree_group(lua: &Lua, st: &S, o: Table) -> Result<GroupU> {
    check_keys(&o, GROUP_KEYS, "tree_group")?;
    let Some(ct) = o.get::<Option<Table>>("crowns")? else {
        return err("tree_group: needs crowns={outline{...}, ...} (a few drawn crowns)");
    };
    let mut crowns = vec![];
    for v in ct.sequence_values::<Value>() {
        let c = line_of(&v?, "tree_group crown")?;
        if c.len() < 3 {
            return err("tree_group crown: needs at least three points");
        }
        crowns.push(c);
    }
    if crowns.is_empty() {
        return err("tree_group: crowns is empty");
    }
    let mut trunks = vec![];
    if let Some(tt) = o.get::<Option<Table>>("trunks")? {
        for v in tt.sequence_values::<Value>() {
            trunks.push(trunk_of(v?)?);
        }
    }
    let empty = lua.create_table()?;
    let species: Vec<Species> = match o.get::<Value>("species")? {
        Value::Nil => vec![Species::oak()],
        Value::String(s) => vec![species_of(&empty, &s.to_str()?)?],
        Value::Table(t) => t.sequence_values::<String>().map(|s| species_of(&empty, &s?)).collect::<Result<_>>()?,
        v => return err(format!("tree_group species: a name or a list of names, got {}", v.type_name())),
    };
    if species.is_empty() {
        return err(format!("tree_group species: one or more of {SPECIES}"));
    }
    let (sname, season) = season_of(&o)?;
    let mut spec = GroupSpec { sun: sun_of(&o)?, ..GroupSpec::default() };
    if let Some(c) = o.get::<Option<usize>>("count")? {
        spec.extra = c.min(60);
    }
    spec.horizon = num(&o, "horizon")?;
    spec.recede = num(&o, "recede")?.unwrap_or(spec.recede).max(0.0);
    spec.air = num(&o, "air")?.unwrap_or(spec.air).max(0.0);
    spec.spread = num(&o, "spread")?.unwrap_or(spec.spread).max(0.0);
    spec.narrow = num(&o, "narrow")?.unwrap_or(spec.narrow).clamp(0.0, 1.0);
    let foot = num(&o, "foot")?;
    let seed = seed_of(st, &o)?;
    let g = Group::grow(&crowns, &trunks, &species, &season, foot, &spec, seed);
    let trees = g.trees.iter().cloned().map(Rc::new).collect();
    let detail = num(&o, "detail")?.unwrap_or(detail_for(&season)).clamp(0.0, 1.0);
    Ok(GroupU { g: Rc::new(g), trees, season: sname, detail, st: st.clone() })
}

impl GroupU {
    fn idx(&self, i: Option<usize>) -> Result<usize> {
        let n = self.trees.len();
        match i {
            None => err(format!("group: which tree? 1 (farthest) .. {n} (nearest)")),
            Some(i) if i == 0 || i > n => err(format!("group: tree {i} out of 1..{n} (1 is the farthest)")),
            Some(i) => Ok(i - 1),
        }
    }
    fn tree(&self, i: usize) -> TreeU {
        TreeU { t: self.trees[i].clone(), season: self.season.clone(), haze: self.g.haze[i], scale: self.g.scale[i], detail: self.detail, st: self.st.clone() }
    }
}

impl UserData for GroupU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("count", |_, g| Ok(g.trees.len()));
        f.add_field_method_get("horizon", |_, g| Ok(g.g.horizon));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // back to front (1 is the farthest)
        m.add_method("trees", |lua, g, ()| {
            let t = lua.create_table()?;
            for i in 0..g.trees.len() {
                t.raw_set(i + 1, g.tree(i))?;
            }
            Ok(t)
        });
        m.add_method("tree", |_, g, i: Option<usize>| Ok(g.tree(g.idx(i)?)));
        m.add_method("haze", |_, g, i: Option<usize>| Ok(g.g.haze[g.idx(i)?]));
        m.add_method("scale", |_, g, i: Option<usize>| Ok(g.g.scale[g.idx(i)?]));
        m.add_method("shadow", |_, g, ()| Ok(wrap(g.g.shadow(frame(&g.st)?))));
        m.add_method("mask", |_, g, ()| {
            let f = frame(&g.st)?;
            let mut m = Mask::empty(f);
            for t in &g.trees {
                m = m.union(&t.mask(f));
            }
            Ok(wrap(m))
        });
        m.add_meta_method(MetaMethod::ToString, |_, g, ()| {
            let parts: Vec<String> = g.trees.iter().zip(&g.g.haze).map(|(t, h)| format!("{} {:.0} tall haze {:.2}", t.species, t.height, h)).collect();
            Ok(format!("tree_group({} trees, far to near: {})", parts.len(), parts.join(" | ")))
        });
    }
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let s1 = st.clone();
    g.set("tree_in", lua.create_function(move |_, o: Table| tree_in(&s1, o))?)?;
    let s1 = st.clone();
    g.set("tree_group", lua.create_function(move |lua, o: Table| tree_group(lua, &s1, o))?)?;
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
    fn an_oak_grows_into_a_drawn_crown() {
        let out = run(r##"local c = outline{{60,30},{110,22},{150,50},{155,95,"c"},{110,118},{60,120},{25,95,"c"},{28,55}, char="soft", seed=2}
               local t = tree_in{crown=c, trunk={{92,190},{90,120},{94,95}}, species="oak", seed=4}
               assert(#t.limbs > 20, #t.limbs)
               assert(t.limbs[1].order == 0)
               local n = t:leaves():area()
               assert(n > 200, n)
               local lit = t:lit():area()
               assert(lit > 0 and lit < n, lit)
               local front = t:leaves{depth={0, 1}}:area()
               assert(front > 0 and front < n)
               assert(t:gaps():area() > 0)
               for _, k in ipairs(t:touches{lit={0.6, 1}}) do assert(k.lit >= 0.6) end
               local laid = t:paint(brush("round", 1.2), {color="#44502c", lit={0.5, 1}, share=0.5})
               local wood = t:paint_wood(brush("rigger", 0.8), {color="#3a332c", max=3})
               local bare = tree_in{crown=c, trunk={{92,190},{90,120},{94,95}}, species="oak", season="winter", seed=4}
               assert(#bare.limbs == #t.limbs)
               print(tostring(t), laid, wood)"##)
        .unwrap();
        assert!(out.contains("tree_in(oak, summer"), "{out}");
    }

    /// Paint a bare oak on a plain ground and count the dark pieces that
    /// don't touch the tree: every twig must leave from painted wood.
    #[test]
    fn a_bare_oak_paints_without_floating_twigs() {
        let mut s = Session::replay(600).unwrap();
        s.run(r#"canvas{aspect=1.4, seed=11}"#).unwrap();
        s.run(
                r##"local c = outline{{330,150},{460,110},{600,150},{680,260,"c"},{640,380},{500,420},{360,400},{300,300,"c"}, char="soft", seed=2}
               t = tree_in{crown=c, trunk={{495,690},{490,520},{500,420}}, species="oak", season="winter", seed=4}
               assert(math.abs(t.detail - 0.35) < 1e-6, t.detail)
               t:paint_wood(brush("round", 2.4), {color="#2a2520", min=1.2})
               t:paint_wood(brush("rigger", 0.6), {color="#2a2520", max=1.2})
               local m = t:twig_mass()
               assert(m:area() > 0)
               dry()"##,
        )
        .unwrap();
        let c = s.canvas().unwrap();
        let f = c.window();
        let px = c.pixels();
        let lum = |p: &paint::Rgb| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
        let mut l: Vec<f32> = px.iter().map(lum).collect();
        let bg = {
            let mut v = l.clone();
            let k = v.len() * 3 / 4;
            *v.select_nth_unstable_by(k, |a, b| a.total_cmp(b)).1
        };
        let ink = lum(&paint::hex("#2a2520"));
        for v in &mut l {
            *v = (bg - *v) / (bg - ink);
        }
        // dark pieces, 8-connected; a speck is up to 6 px (at 600 px the code
        // before this fix left 14 pieces over 3 px, the largest 16)
        let (w, h) = (f.w, f.h);
        let mut lab = vec![0u32; w * h];
        let mut sizes = vec![0usize];
        for start in 0..w * h {
            if l[start] <= 0.15 || lab[start] != 0 {
                continue;
            }
            let id = sizes.len() as u32;
            let mut stack = vec![start];
            lab[start] = id;
            let mut n = 0;
            while let Some(i) = stack.pop() {
                n += 1;
                let (x, y) = ((i % w) as i64, (i / w) as i64);
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let (xx, yy) = (x + dx, y + dy);
                        if xx < 0 || yy < 0 || xx >= w as i64 || yy >= h as i64 {
                            continue;
                        }
                        let j = yy as usize * w + xx as usize;
                        if l[j] > 0.15 && lab[j] == 0 {
                            lab[j] = id;
                            stack.push(j);
                        }
                    }
                }
            }
            sizes.push(n);
        }
        let tree = (1..sizes.len()).max_by_key(|&i| sizes[i]).unwrap();
        let floating: Vec<usize> = (1..sizes.len()).filter(|&i| i != tree && sizes[i] > 6).map(|i| sizes[i]).collect();
        assert!(sizes[tree] > 5000, "the tree is {} px", sizes[tree]);
        assert!(floating.is_empty(), "pieces off the tree bigger than a speck: {floating:?} (tree {} px)", sizes[tree]);
    }

    #[test]
    fn a_group_and_errors() {
        run(r#"local c = {{40,80},{60,70},{75,85},{70,100},{45,102}}
               local g = tree_group{crowns={c}, species={"oak", "lime"}, foot=140, count=3, seed=5}
               assert(g.count == 4)
               assert(g:haze(1) >= g:haze(4))
               assert(g:shadow():area() > 0)
               local t = g:tree(4)
               assert(t:leaves():area() > 0)"#)
        .unwrap();
        let e = run(r#"tree_in{crown={{0,0},{10,10},{20,0}}, species="palm"}"#).unwrap_err();
        assert!(e.contains("birch"), "{e}");
        let e = run(r#"tree_in{crown={{0,0},{10,10},{20,0}}, season="monsoon"}"#).unwrap_err();
        assert!(e.contains("winter"), "{e}");
    }
}
