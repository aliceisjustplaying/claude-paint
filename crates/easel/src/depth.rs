//! Depth from the world, in Lua: what lies behind what, so a painter
//! needn't subtract earlier shapes from a mask by hand.
//!
//! - `w:layer(name, mask, depth)` registers a shape painted by hand at a
//!   depth: meters, a spot, a canvas point `{x, y}` (the depth of the
//!   ground seen there, where the shape stands) or `"ground"` (it lies
//!   flat on the ground).
//! - A view answers with masks: `v:visible(x)`, `v:front(x)`, `v:behind(x)`,
//!   `v:at_depth(m)`, `v:between(a, b)`, and soft shadows that fall off
//!   physically: `v:cast_shadow{}`, `v:contact_shadow{}`.
//! - `work`, `stipple` and `blend` take `visible=`, `behind=` and
//!   `at=`, resolved against the last view made (`w:view()`) or `view=v`.
//!
//! Things are named by body number (from `w:place`/`w:proxy`), layer name,
//! `"ground"`, `"water"`, `"surface"` (both), `"sky"`, `"bodies"`,
//! `"layers"`, or a list of these. `visible=`, `v:visible` and `behind=`
//! also take masks (alone or in a list): a mask is a thing in front of
//! everything.

use crate::api::{S, check_keys, err, mask_of, num, paint_mask, paint_mask_of, wrap};
use crate::world::{SpotU, ViewU};
use mlua::{Result, Table, UserDataMethods, Value};
use paint::Mask;
use paint::scene::{Depths, LayerDepth, Thing, World};
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

/// Split a selection into masks and the names (numbers, strings) around
/// them, lists flattened.
fn split_masks(v: &Value, masks: &mut Vec<Rc<Mask>>, names: &mut Vec<Value>) -> Result<()> {
    match v {
        Value::Table(t) => {
            for x in t.sequence_values::<Value>() {
                split_masks(&x?, masks, names)?;
            }
        }
        x if mask_of(x).is_ok() => masks.push(mask_of(x)?),
        x => names.push(x.clone()),
    }
    Ok(())
}

/// Where a selection is seen. A mask in it is a thing in front of
/// everything (as in `behind=`): it is seen wherever it is, and it hides
/// the named things behind it, so the union is `1 - (1 - m)(1 - seen)`.
fn visible_of(w: &World, d: &Depths, v: &Value, normalize: impl Fn(Rc<Mask>) -> Rc<Mask>) -> Result<Mask> {
    let (mut masks, mut names) = (Vec::new(), Vec::new());
    split_masks(v, &mut masks, &mut names)?;
    let mut picks = Vec::new();
    for x in &names {
        pick(w, x, &mut picks)?;
    }
    let s = Sel(picks);
    let mut out = d.visible(&|t| s.has(t));
    for m in masks {
        let m = normalize(m);
        out = out.invert().mul(&(*m).clone().invert()).invert();
    }
    Ok(out)
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
    m.add_method("visible", |_, v, x: Value| Ok(wrap(visible_of(&v.0.world, v.0.view.depths(), &x, |m| m)?)));
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
/// strokes in (None: the whole canvas) and a hard
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
        out = out.mul(&visible_of(w, d, &vis, |m| paint_mask(st, m))?);
    }
    if !beh.is_nil() {
        // a mask is a thing in front of everything; names go by depth
        let (masks, names): (Vec<Value>, Vec<Value>) = match &beh {
            Value::Table(t) => t.sequence_values::<Value>().collect::<Result<Vec<_>>>()?.into_iter().partition(|x| mask_of(x).is_ok()),
            x if mask_of(x).is_ok() => (vec![x.clone()], vec![]),
            x => (vec![], vec![x.clone()]),
        };
        for x in &masks {
            out = out.subtract(&*paint_mask_of(st, x)?);
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
    if st.borrow().tubes.engine >= 9 {
        out = out.map(|v| v.clamp(0.0, 1.0));
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
