//! Depth from the world, in Lua: what lies behind what, so a painter never
//! subtracts earlier motifs from a mask by hand.
//!
//! - `w:layer(name, mask, depth)` registers a motif painted by hand (a
//!   figure, a drawn boat) at a depth: meters, a spot, a canvas point
//!   `{x, y}` (the depth of the ground seen there: the figure's feet) or
//!   `"ground"` (it lies on the ground: a path, a glint on the water).
//! - A view answers with masks: `v:visible(x)`, `v:front(x)`, `v:behind(x)`,
//!   `v:at_depth(m)`, `v:between(a, b)`, and soft shadows that fall off
//!   physically: `v:cast_shadow{}`, `v:contact_shadow{}`.
//! - `work`, `stipple`, `glaze` and `blend` take `visible=`, `behind=` and
//!   `at=`, resolved against the last view made (`w:view()`) or `view=v`.
//!
//! Things are named by body number (from `w:place`/`w:proxy`), layer name,
//! `"ground"`, `"water"`, `"surface"` (both), `"sky"`, `"bodies"`,
//! `"layers"`, or a list of these.

use crate::api::{S, check_keys, err, mask_of, num, wrap};
use crate::world::{SpotU, ViewU};
use mlua::{Result, Table, UserDataMethods, Value};
use paint::Mask;
use paint::scene::{LayerDepth, Thing, World};
use std::rc::Rc;
use std::sync::Arc;

/// One name for things in the world.
#[derive(Clone, Debug, PartialEq)]
enum Pick {
    Sky,
    Ground,
    Water,
    Body(usize),
    Layer(usize),
    Bodies,
    Layers,
}

/// A selection: any of these.
#[derive(Clone, Debug)]
pub struct Sel(Vec<Pick>);

impl Sel {
    fn has(&self, t: Thing) -> bool {
        self.0.iter().any(|p| match (p, t) {
            (Pick::Sky, Thing::Sky) | (Pick::Ground, Thing::Ground) | (Pick::Water, Thing::Water) => true,
            (Pick::Body(a), Thing::Body(b)) | (Pick::Layer(a), Thing::Layer(b)) => *a == b,
            (Pick::Bodies, Thing::Body(_)) | (Pick::Layers, Thing::Layer(_)) => true,
            _ => false,
        })
    }
    fn bodies(&self) -> impl Fn(usize) -> bool + Sync + '_ {
        move |b| self.has(Thing::Body(b))
    }
}

fn pick(w: &World, v: &Value, out: &mut Vec<Pick>) -> Result<()> {
    match v {
        Value::Integer(n) => {
            let n = *n as usize;
            if n == 0 || n > w.bodies.len() {
                return err(format!("body {n}: this world has bodies 1 to {}", w.bodies.len()));
            }
            out.push(Pick::Body(n - 1));
        }
        Value::Number(n) if n.fract() == 0.0 => return pick(w, &Value::Integer(*n as i64), out),
        Value::String(s) => {
            let s = s.to_str()?.to_string();
            let p = match s.as_str() {
                "sky" => Pick::Sky,
                "ground" | "land" => Pick::Ground,
                "water" => Pick::Water,
                "surface" => {
                    out.push(Pick::Ground);
                    Pick::Water
                }
                "bodies" => Pick::Bodies,
                "layers" => Pick::Layers,
                name => match w.layer_named(name) {
                    Some(i) => Pick::Layer(i),
                    None => {
                        let names: Vec<&str> = w.layers.iter().map(|l| l.name.as_str()).collect();
                        return err(format!(
                            "no layer {name:?} in this view's world (layers: {}; also \"ground\", \"water\", \"surface\", \"sky\", \"bodies\", \"layers\" or a body number). Register it with w = w:layer({name:?}, mask, depth), then v = w:view()",
                            if names.is_empty() { "none".to_string() } else { names.join(", ") }
                        ));
                    }
                },
            };
            out.push(p);
        }
        Value::Table(t) => {
            for x in t.sequence_values::<Value>() {
                pick(w, &x?, out)?;
            }
        }
        o => return err(format!("want a body number, a layer name, \"ground\", \"water\", \"sky\" or a list, got {}", o.type_name())),
    }
    Ok(())
}

fn sel_of(w: &World, v: &Value) -> Result<Sel> {
    let mut out = Vec::new();
    pick(w, v, &mut out)?;
    Ok(Sel(out))
}

/// A depth in meters: a number, a spot, or a canvas point `{x, y}` (the
/// ground seen there).
fn meters(w: &World, v: &Value) -> Result<f32> {
    match v {
        Value::Integer(n) => Ok(*n as f32),
        Value::Number(n) => Ok(*n as f32),
        Value::UserData(u) => Ok(u.borrow::<SpotU>()?.0.at[2]),
        Value::Table(t) => {
            let (x, y): (f32, f32) = (t.get(1)?, t.get(2)?);
            match w.to_ground(x, y) {
                Some(p) => Ok(p[2]),
                None => err(format!("({x}, {y}) is not on the ground (sky or outside the view): give a depth in meters or a spot")),
            }
        }
        o => err(format!("want a depth: meters, a spot, or a canvas point {{x, y}} on the ground, got {}", o.type_name())),
    }
}

/// `w:layer(name, mask, depth)`'s depth.
pub(crate) fn layer_depth(w: &World, v: &Value) -> Result<LayerDepth> {
    match v {
        Value::String(s) if &*s.to_str()? == "ground" => Ok(LayerDepth::Ground),
        v => meters(w, v).map(LayerDepth::At),
    }
}

// ---------------------------------------------------------------- view methods

/// Depth masks on a view (registered from world.rs).
pub fn view_methods<M: UserDataMethods<ViewU>>(m: &mut M) {
    // v:visible(x): where x is seen (less whatever is in front of it)
    m.add_method("visible", |_, v, x: Value| {
        let s = sel_of(&v.0.world, &x)?;
        Ok(wrap(v.0.view.depths().visible(&|t| s.has(t))))
    });
    // v:front(x): what hides x, where x is
    m.add_method("front", |_, v, x: Value| {
        let s = sel_of(&v.0.world, &x)?;
        Ok(wrap(v.0.view.depths().front(&|t| s.has(t))))
    });
    // v:behind(x): where a pass lying just behind x shows
    m.add_method("behind", |_, v, x: Value| {
        let s = sel_of(&v.0.world, &x)?;
        Ok(wrap(v.0.view.depths().behind(&|t| s.has(t))))
    });
    // v:at_depth(m or spot or {x, y}): where a pass that far off shows
    m.add_method("at_depth", |_, v, z: Value| Ok(wrap(v.0.view.depths().at_depth(meters(&v.0.world, &z)?))));
    m.add_method("between", |_, v, (a, b): (Value, Value)| Ok(wrap(v.0.view.depths().between(meters(&v.0.world, &a)?, meters(&v.0.world, &b)?))));
    // v:seen(x, y): what is seen there, nearest first: {what, body, layer, depth, share}
    m.add_method("seen", |lua, v, (x, y): (f32, f32)| {
        let out = lua.create_table()?;
        for (i, (th, d, share)) in v.0.view.depths().seen_at(x, y).into_iter().enumerate() {
            let t = lua.create_table()?;
            match th {
                Thing::Sky => t.set("what", "sky")?,
                Thing::Ground => t.set("what", "ground")?,
                Thing::Water => t.set("what", "water")?,
                Thing::Body(b) => {
                    t.set("what", "body")?;
                    t.set("body", b + 1)?;
                }
                Thing::Layer(l) => {
                    t.set("what", "layer")?;
                    t.set("layer", v.0.world.layers[l].name.clone())?;
                }
            }
            t.set("depth", if d.is_finite() && d < f32::MAX { Some(d) } else { None })?;
            t.set("share", share)?;
            out.raw_set(i + 1, t)?;
        }
        Ok(out)
    });
    m.add_method("layers", |_, v, ()| Ok(v.0.world.layers.iter().map(|l| l.name.clone()).collect::<Vec<_>>()));
    // v:cast_shadow{soft=1, from=bodies}: shadows on the ground and water
    // where they're seen, the penumbra growing with distance from the caster
    m.add_method("cast_shadow", |_, v, o: Option<Table>| {
        let (soft, from) = match &o {
            Some(o) => {
                check_keys(o, &["soft", "from"], "cast_shadow")?;
                (num(o, "soft")?.unwrap_or(1.0), o.get::<Value>("from")?)
            }
            None => (1.0, Value::Nil),
        };
        let s = if from.is_nil() { Sel(vec![Pick::Bodies]) } else { sel_of(&v.0.world, &from)? };
        let by = s.bodies();
        Ok(wrap(v.0.view.soft_shadows(soft, &by)))
    });
    // v:contact_shadow{reach=0.4, from=bodies}: the sky hidden near bodies,
    // darkest in the crease, falling off smoothly
    m.add_method("contact_shadow", |_, v, o: Option<Table>| {
        let (reach, from) = match &o {
            Some(o) => {
                check_keys(o, &["reach", "from"], "contact_shadow")?;
                (num(o, "reach")?.unwrap_or(0.4), o.get::<Value>("from")?)
            }
            None => (0.4, Value::Nil),
        };
        let s = if from.is_nil() { Sel(vec![Pick::Bodies]) } else { sel_of(&v.0.world, &from)? };
        let by = s.bodies();
        Ok(wrap(v.0.view.occlusion(reach, &by)))
    });
}

// ---------------------------------------------------------------- pass options

/// A pass's seed mask and its hard limit.
pub type Restricted = (Option<Rc<Mask>>, Option<Arc<Mask>>);

/// Apply `visible=`, `behind=` and `at=` to a pass: the mask it seeds
/// strokes in (None: the whole canvas, for `glaze(nil, ...)`) and a hard
/// limit no bristle may cross (strokes still overshoot the region's own
/// edges, but never into what is in front). Unchanged if none is given.
pub fn restrict(st: &S, o: &Table, m: Option<Rc<Mask>>) -> Result<Restricted> {
    let (vis, beh, at) = (o.get::<Value>("visible")?, o.get::<Value>("behind")?, o.get::<Value>("at")?);
    if vis.is_nil() && beh.is_nil() && at.is_nil() {
        return Ok((m, None));
    }
    let view = match o.get::<Value>("view")? {
        Value::Nil => st.borrow().view.clone().ok_or_else(|| {
            mlua::Error::runtime("visible=, behind= and at= need a world view: v = w:view() first (the last view made is used), or pass view=v")
        })?,
        Value::UserData(u) => u.borrow::<ViewU>()?.clone(),
        o => return err(format!("view: want a view (w:view()), got {}", o.type_name())),
    };
    let f = crate::api::frame(st)?;
    let mut out = Mask::full(f);
    let w = &view.0.world;
    let d = view.0.view.depths();
    if !vis.is_nil() {
        let s = sel_of(w, &vis)?;
        out = out.mul(&d.visible(&|t| s.has(t)));
    }
    if !beh.is_nil() {
        // a mask is a thing in front of everything; names go by depth
        let (masks, names): (Vec<Value>, Vec<Value>) = match &beh {
            Value::Table(t) => t.sequence_values::<Value>().collect::<Result<Vec<_>>>()?.into_iter().partition(|x| mask_of(x).is_ok()),
            x if mask_of(x).is_ok() => (vec![x.clone()], vec![]),
            x => (vec![], vec![x.clone()]),
        };
        for x in &masks {
            out = out.subtract(&*mask_of(x)?);
        }
        if !names.is_empty() {
            let mut picks = Vec::new();
            for x in &names {
                pick(w, x, &mut picks)?;
            }
            let s = Sel(picks);
            // behind each of them (not behind their union's farthest part)
            for p in &s.0 {
                let one = Sel(vec![p.clone()]);
                out = out.mul(&d.behind(&|t| one.has(t)));
            }
        }
    }
    if !at.is_nil() {
        out = out.mul(&d.at_depth(meters(w, &at)?));
    }
    let seed = match m {
        Some(m) => (*m).clone().mul(&out),
        None => out.clone(),
    };
    Ok((Some(Rc::new(seed)), Some(Arc::new(out))))
}

/// The same for a pass that needs a mask.
pub fn restrict_mask(st: &S, o: &Table, m: Rc<Mask>) -> Result<(Rc<Mask>, Option<Arc<Mask>>)> {
    let (m, l) = restrict(st, o, Some(m))?;
    Ok((m.expect("a mask in, a mask out"), l))
}

/// Register a layer on a world (see world.rs `WorldU::with_layer`).
pub(crate) fn layer_args(w: &World, name: &Value, mask: &Value, depth: &Value) -> Result<(String, Mask, LayerDepth)> {
    let name = match name {
        Value::String(s) => s.to_str()?.to_string(),
        o => return err(format!("layer(name, mask, depth): name is a string, got {}", o.type_name())),
    };
    if matches!(name.as_str(), "sky" | "ground" | "land" | "water" | "surface" | "bodies" | "layers") {
        return err(format!("layer name {name:?} is taken (it names a part of the world)"));
    }
    let m = mask_of(mask)?;
    Ok((name, (*m).clone(), layer_depth(w, depth)?))
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    const SETUP: &str = r##"canvas{style="friedrich", aspect=1.5, seed=3}
w = world{horizon=300, eye=1.6, sun={azimuth=-80, elevation=20},
  ground=function(X, Z) return Z > 20 and -1 or 0.1 end, water={level=0}}
local s = w:spot_at(0, 12)
w, stone = w:place(s, body.ellipsoid(s:p(0, 0.3, 0), s:size(1.2, 0.8, 1.0)))
fs = w:spot_at(-2.5, 9)
fig = rect(fs.x - fs:m(0.3), fs.y - fs:m(1.7), fs:m(0.6), fs:m(1.7))
w = w:layer("figure", fig, fs)
v = w:view()"##;

    fn px(s: &Session, x: f32, y: f32) -> [f32; 3] {
        let c = s.canvas().unwrap();
        let f = c.frame();
        c.seen()[f.index(x, y)]
    }

    #[test]
    fn passes_keep_behind_what_is_in_front() {
        let mut s = Session::new(240, 2).unwrap();
        s.run(SETUP).unwrap();
        // the masks know the figure stands over the sea
        s.run(r#"local sea = v:visible("water")
                 local y = fs.y - fs:m(1.2)
                 assert(sea:at(fs.x, y) < 0.01, sea:at(fs.x, y))
                 assert(sea:at(fs.x - 120, y) > 0.99)
                 assert(v:behind("figure"):at(fs.x, y) < 0.01)
                 assert(v:front(stone):at(500, 380) < 0.01)
                 local seen = v:seen(fs.x, y)
                 assert(seen[1].what == "layer" and seen[1].layer == "figure" and seen[1].share == 1 and seen[2] == nil, seen[1].what)
                 local beside = v:seen(fs.x - 120, y)
                 assert(beside[1].what == "water" and beside[1].share == 1)
                 assert(#v:layers() == 1)"#)
            .unwrap();
        s.run(r#"fx, fy = fs.x, fs.y - fs:m(1.2)"#).unwrap();
        let fxy: (f32, f32) = s.lua.load("return fx, fy").eval().unwrap();
        let before_in = px(&s, fxy.0, fxy.1);
        let before_out = px(&s, fxy.0 - 120.0, fxy.1);
        // a sea veil behind the figure
        s.run(r##"work(below(function() return 302 end), {hand="broad", color="#20304a", coverage=3, behind="figure"})"##).unwrap();
        assert_eq!(before_in, px(&s, fxy.0, fxy.1), "the figure's place is untouched");
        assert_ne!(before_out, px(&s, fxy.0 - 120.0, fxy.1), "the sea beside it is painted");
        // glints only where the water is seen: none on the figure or the stone
        s.run(r##"stipple(below(function() return 302 end), {width=2, color="#e0e0d0", coverage=2, visible="water"})"##).unwrap();
        assert_eq!(before_in, px(&s, fxy.0, fxy.1));
        // glaze too, and a mask in behind=
        s.run(r##"glaze(nil, {color="#402010", coats=0.3, behind={"figure", stone}, at=30})"##).unwrap();
        assert_eq!(before_in, px(&s, fxy.0, fxy.1));
        // an unknown layer says what there is
        let e = s.run(r##"work(everywhere(), {color="#fff", behind="boat"})"##).unwrap_err();
        assert!(e.contains("no layer \"boat\"") && e.contains("figure"), "{e}");
        // the replay agrees
        let mut r = Session::replay(240).unwrap();
        for c in &s.log {
            r.run(&c.src).unwrap();
        }
        assert_eq!(s.canvas().unwrap().seen(), r.canvas().unwrap().seen());
    }

    #[test]
    fn the_current_view_rolls_back() {
        let mut s = Session::new(160, 2).unwrap();
        s.run(r#"canvas{aspect=1.5, seed=2}"#).unwrap();
        let e = s.run(r##"work(everywhere(), {color="#fff", visible="ground"})"##).unwrap_err();
        assert!(e.contains("need a world view"), "{e}");
        // a failed chunk's view is not left behind
        let _ = s.run(r#"local w = world{horizon=200}; local v = w:view(); error("stop")"#).unwrap_err();
        assert!(s.st.borrow().view.is_none());
        s.run(r#"w = world{horizon=200}; v = w:view()"#).unwrap();
        assert!(s.st.borrow().view.is_some());
        s.undo(1).unwrap();
        assert!(s.st.borrow().view.is_none());
    }

    #[test]
    fn soft_shadow_masks_have_no_rings() {
        let mut s = Session::replay(200).unwrap();
        s.run(SETUP).unwrap();
        s.run(r#"local sh = v:cast_shadow{soft=2}
                 local c = v:contact_shadow{reach=0.5, from=stone}
                 local st = w:spot_at(0, 12)
                 -- nothing above the stone in the sky: no ring
                 assert(c:at(st.x, st.y - st:m(1.5)) == 0)
                 assert(c:at(st.x - st:m(1.3), st.y + 2) > 0.1, c:at(st.x - st:m(1.3), st.y + 2))
                 assert(sh:area() > 100)"#)
            .unwrap();
    }
}
