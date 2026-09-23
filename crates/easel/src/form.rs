//! Form in Lua: solids the painter models (bodies, ridges, reliefs), a lit
//! depth buffer over the canvas, and what it tells a painter: light and
//! shadow, planes, fall lines, silhouettes and edges. It paints nothing.
//!
//! Every value here is immutable (a solid operation returns a new solid;
//! `form{}` builds and lights the whole depth buffer in one call), so undo
//! and rollback never have to repair them.

use crate::api::{FromLuaValue, S, check_keys, err, frame, num, pair, wrap};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value};
use paint::form::{Sample, V3, aerial};
use paint::{Form, Light, Mask, Relief, Ridge, Sdf, Shade, Solid};
use std::sync::Arc;

/// A solid a painter has modeled.
#[derive(Clone)]
pub enum SolidU {
    Body(Arc<Sdf>),
    Ridge(Arc<Ridge>),
    Relief(Arc<ReliefGrid>),
}

/// A relief's height function, sampled from Lua every `step` units.
pub struct ReliefGrid {
    area: [f32; 4],
    step: f32,
    nx: usize,
    ny: usize,
    /// z per node (NaN: no surface there).
    z: Vec<f32>,
    facet: u16,
}

impl ReliefGrid {
    fn get(&self, x: f32, y: f32) -> Option<(f32, u16)> {
        let fx = (x - self.area[0]) / self.step;
        let fy = (y - self.area[1]) / self.step;
        if fx < 0.0 || fy < 0.0 || fx > (self.nx - 1) as f32 || fy > (self.ny - 1) as f32 {
            return None;
        }
        let (i, j) = ((fx as usize).min(self.nx - 2), (fy as usize).min(self.ny - 2));
        let (tx, ty) = (fx - i as f32, fy - j as f32);
        let at = |a: usize, b: usize| self.z[b * self.nx + a];
        let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
        if a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan() {
            // at the edge of the surface: the nearest node decides
            let n = at(if tx < 0.5 { i } else { i + 1 }, if ty < 0.5 { j } else { j + 1 });
            return if n.is_nan() { None } else { Some((n, self.facet)) };
        }
        Some(((a * (1.0 - tx) + b * tx) * (1.0 - ty) + (c * (1.0 - tx) + d * tx) * ty, self.facet))
    }
}

pub(crate) fn v3(v: &Value, what: &str) -> Result<V3> {
    match v {
        Value::Table(t) => {
            let z: Option<f32> = t.get(3)?;
            Ok([t.get(1)?, t.get(2)?, z.unwrap_or(0.0)])
        }
        _ => err(format!("{what}: want {{x, y, z}}")),
    }
}

fn body_of(v: &Value) -> Result<Arc<Sdf>> {
    match v {
        Value::UserData(u) => match &*u.borrow::<SolidU>()? {
            SolidU::Body(b) => Ok(b.clone()),
            _ => err("want a body (body.ellipsoid, body.block); ridges and terrain don't combine"),
        },
        o => err(format!("want a body, got {}", o.type_name())),
    }
}

pub(crate) fn solid_of(v: &Value) -> Result<SolidU> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<SolidU>()?.clone()),
        o => err(format!("want a solid (body.ellipsoid, body.block, ridge{{}}, terrain{{}}), got {}", o.type_name())),
    }
}

impl UserData for SolidU {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // s:turn(center, yaw, pitch, roll)
        m.add_method("turn", |_, s, (c, yaw, pitch, roll): (Value, f32, Option<f32>, Option<f32>)| {
            let b = body_of_self(s)?;
            Ok(SolidU::Body(Arc::new((*b).clone().turn(v3(&c, "turn center")?, yaw, pitch.unwrap_or(0.0), roll.unwrap_or(0.0)))))
        });
        // s:cut(at, normal, facet?, round?): a fracture plane
        m.add_method("cut", |_, s, (at, n, facet, round): (Value, Value, Option<u16>, Option<f32>)| {
            let b = body_of_self(s)?;
            Ok(SolidU::Body(Arc::new((*b).clone().cut(v3(&at, "cut at")?, v3(&n, "cut normal")?, facet.unwrap_or(10), round.unwrap_or(2.0)))))
        });
        // s:rough(amp, period, seed?, ridged?): weathering
        m.add_method("rough", |_, s, (amp, period, seed, ridged): (f32, f32, Option<u32>, Option<bool>)| {
            let b = body_of_self(s)?;
            Ok(SolidU::Body(Arc::new((*b).clone().rough(amp, period, seed.unwrap_or(1), ridged.unwrap_or(false)))))
        });
        m.add_method("facet", |_, s, id: u16| Ok(SolidU::Body(Arc::new((*body_of_self(s)?).clone().facet(id)))));
        m.add_method("union", |_, s, (o, smooth): (Value, Option<f32>)| {
            Ok(SolidU::Body(Arc::new((*body_of_self(s)?).clone().union((*body_of(&o)?).clone(), smooth.unwrap_or(0.0)))))
        });
        m.add_method("subtract", |_, s, (o, smooth): (Value, Option<f32>)| {
            Ok(SolidU::Body(Arc::new((*body_of_self(s)?).clone().subtract((*body_of(&o)?).clone(), smooth.unwrap_or(0.0)))))
        });
        m.add_meta_method(MetaMethod::Add, |_, s, o: Value| Ok(SolidU::Body(Arc::new((*body_of_self(s)?).clone().union((*body_of(&o)?).clone(), 0.0)))));
        m.add_meta_method(MetaMethod::Sub, |_, s, o: Value| Ok(SolidU::Body(Arc::new((*body_of_self(s)?).clone().subtract((*body_of(&o)?).clone(), 0.0)))));
        m.add_method("bounds", |_, s, ()| {
            let b = solid_bounds(s);
            Ok(vec![b[0], b[1], b[2], b[3]])
        });
        m.add_meta_method(MetaMethod::ToString, |_, s, ()| {
            let b = solid_bounds(s);
            let kind = match s {
                SolidU::Body(_) => "body",
                SolidU::Ridge(_) => "ridge",
                SolidU::Relief(_) => "terrain",
            };
            Ok(format!("{kind}(bounds {:.0}, {:.0}, {:.0}, {:.0})", b[0], b[1], b[2], b[3]))
        });
    }
}

fn body_of_self(s: &SolidU) -> Result<Arc<Sdf>> {
    match s {
        SolidU::Body(b) => Ok(b.clone()),
        _ => err("only bodies turn, cut, weather and combine (not ridges or terrain)"),
    }
}

fn solid_bounds(s: &SolidU) -> [f32; 4] {
    match s {
        SolidU::Body(b) => (**b).bounds(),
        SolidU::Ridge(r) => (**r).bounds(),
        SolidU::Relief(g) => g.area,
    }
}

// ---------------------------------------------------------------- the form

/// Anything that holds a lit form: a standalone `Form` or a scene's view.
pub trait FormHolder: Send + Sync {
    fn form(&self) -> &Form;
}
impl FormHolder for Form {
    fn form(&self) -> &Form {
        self
    }
}

/// A lit depth buffer of solids.
pub struct FormU {
    pub src: Arc<dyn FormHolder>,
    pub parts: u16,
}

impl FormU {
    fn f(&self) -> &Form {
        self.src.form()
    }
}

/// A direction field read straight from a form (no grid): see `f:field`.
#[derive(Clone)]
pub struct FieldU {
    pub form: Arc<dyn FormHolder>,
    pub kind: FieldKind,
    pub span: f32,
}

#[derive(Clone, Copy)]
pub enum FieldKind {
    Fall,
    Across,
    Edge,
}

impl UserData for FieldU {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(MetaMethod::ToString, |_, fu, ()| {
            Ok(match fu.kind {
                FieldKind::Fall => "field(fall)".to_string(),
                FieldKind::Across => "field(across)".to_string(),
                FieldKind::Edge => format!("field(edge, span {})", fu.span),
            })
        });
    }
}

impl FieldU {
    /// The field as a closure the engine's threads can call. Off the form it
    /// is `fallback` (0 = left to right).
    pub fn angle(&self, fallback: f32) -> Box<dyn Fn(f32, f32) -> f32 + Sync> {
        let (form, kind, span) = (self.form.clone(), self.kind, self.span);
        Box::new(move |x, y| {
            let form = form.form();
            match kind {
                FieldKind::Fall => form.sample(x, y).map_or(fallback, |s| s.fall()),
                FieldKind::Across => form.sample(x, y).map_or(fallback, |s| s.across()),
                FieldKind::Edge => {
                    if form.sample(x, y).is_some() { form.edge_angle(x, y, span) } else { fallback }
                }
            }
        })
    }
}

pub(crate) fn shade_table(lua: &Lua, s: &Shade) -> Result<Table> {
    let t = lua.create_table()?;
    t.set("turn", s.turn)?;
    t.set("direct", s.direct)?;
    t.set("cast", s.cast)?;
    t.set("bounce", s.bounce)?;
    t.set("sky", s.sky)?;
    t.set("value", s.value)?;
    Ok(t)
}

/// Fill `t` with a sample (the same table is reused by `f:mask`).
fn fill_sample(lua: &Lua, t: &Table, s: &Sample) -> Result<()> {
    t.set("part", s.part)?;
    t.set("facet", s.facet)?;
    t.set("z", s.z)?;
    t.set("n", lua.create_sequence_from(s.n)?)?;
    t.set("dist", s.dist)?;
    t.set("fall", s.fall())?;
    t.set("across", s.across())?;
    t.set("shade", shade_table(lua, &s.shade)?)?;
    t.set("lit", s.shade.lit(0.12))?;
    Ok(())
}

fn parts_of(o: Option<&Table>, all: u16) -> Result<Vec<u16>> {
    match o.map(|o| o.get::<Value>("parts")).transpose()? {
        None | Some(Value::Nil) => Ok((1..=all).collect()),
        Some(Value::Integer(p)) => Ok(vec![p as u16]),
        Some(Value::Table(t)) => t.sequence_values::<u16>().collect(),
        Some(o) => err(format!("parts: want a part number or a list, got {}", o.type_name())),
    }
}

impl UserData for FormU {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("parts", |_, fu| Ok(fu.parts));
    }
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        m.add_method("sample", |lua, fu, (x, y): (f32, f32)| match fu.f().sample(x, y) {
            None => Ok(Value::Nil),
            Some(s) => {
                let t = lua.create_table()?;
                fill_sample(lua, &t, &s)?;
                Ok(Value::Table(t))
            }
        });
        m.add_method("shade", |lua, fu, (x, y): (f32, f32)| shade_table(lua, &fu.f().shade(x, y)));
        m.add_method("lit_at", |_, fu, (x, y, soft): (f32, f32, Option<f32>)| Ok(fu.f().shade(x, y).lit(soft.unwrap_or(0.12))));
        m.add_method("value", |_, fu, (x, y): (f32, f32)| Ok(fu.f().shade(x, y).value));
        m.add_method("fall", |_, fu, (x, y): (f32, f32)| Ok(fu.f().fall(x, y)));
        m.add_method("across", |_, fu, (x, y): (f32, f32)| Ok(fu.f().across(x, y)));
        m.add_method("part", |_, fu, (x, y): (f32, f32)| Ok(fu.f().part(x, y)));
        m.add_method("dist", |_, fu, (x, y, far): (f32, f32, Option<f32>)| Ok(fu.f().dist(x, y, far.unwrap_or(1e9))));
        m.add_method("bend", |_, fu, (x, y, span): (f32, f32, Option<f32>)| Ok(fu.f().bend(x, y, span.unwrap_or(2.5))));
        m.add_method("edge_angle", |_, fu, (x, y, span): (f32, f32, Option<f32>)| Ok(fu.f().edge_angle(x, y, span.unwrap_or(2.5))));
        // f:field("fall" | "across" | "edge", span?): an angle field for work{angle=}
        m.add_method("field", |_, fu, (kind, span): (String, Option<f32>)| {
            let kind = match kind.as_str() {
                "fall" => FieldKind::Fall,
                "across" => FieldKind::Across,
                "edge" => FieldKind::Edge,
                o => return err(format!("field {o:?}: \"fall\", \"across\" or \"edge\"")),
            };
            Ok(FieldU { form: fu.src.clone(), kind, span: span.unwrap_or(2.5) })
        });
        // masks
        m.add_method("parts_mask", |_, fu, parts: Value| {
            let ps = match parts {
                Value::Integer(p) => vec![p as u16],
                Value::Table(t) => t.sequence_values::<u16>().collect::<Result<_>>()?,
                Value::Nil => (1..=fu.parts).collect(),
                o => return err(format!("parts_mask: want a part or a list, got {}", o.type_name())),
            };
            Ok(wrap(fu.f().mask(|s| if ps.contains(&s.part) { 1.0 } else { 0.0 })))
        });
        // f:lit{parts=, soft=}: the light family; f:shadow{...}: the rest
        m.add_method("lit", |_, fu, o: Option<Table>| {
            if let Some(o) = &o {
                check_keys(o, &["parts", "soft"], "lit")?;
            }
            let ps = parts_of(o.as_ref(), fu.parts)?;
            let soft = o.as_ref().map(|o| num(o, "soft")).transpose()?.flatten().unwrap_or(0.12);
            Ok(wrap(fu.f().mask(|s| if ps.contains(&s.part) { s.shade.lit(soft) } else { 0.0 })))
        });
        m.add_method("shadow", |_, fu, o: Option<Table>| {
            if let Some(o) = &o {
                check_keys(o, &["parts", "soft"], "shadow")?;
            }
            let ps = parts_of(o.as_ref(), fu.parts)?;
            let soft = o.as_ref().map(|o| num(o, "soft")).transpose()?.flatten().unwrap_or(0.12);
            Ok(wrap(fu.f().mask(|s| if ps.contains(&s.part) { 1.0 - s.shade.lit(soft) } else { 0.0 })))
        });
        // f:silhouette{parts=, soft=0.4, haze={k, visibility}}: the outline, its
        // edge soft + k·aerial(dist, visibility)² units wide
        m.add_method("silhouette", |_, fu, o: Option<Table>| {
            if let Some(o) = &o {
                check_keys(o, &["parts", "soft", "haze"], "silhouette")?;
            }
            let ps = parts_of(o.as_ref(), fu.parts)?;
            let soft = o.as_ref().map(|o| num(o, "soft")).transpose()?.flatten().unwrap_or(0.4);
            let haze = o.as_ref().map(|o| pair(o, "haze")).transpose()?.flatten();
            Ok(wrap(fu.f().silhouette(&ps, |s| match haze {
                Some((k, vis)) => soft + k * aerial(s.dist, vis).powi(2),
                None => soft,
            })))
        });
        // f:edges{turn=0.8, step=3, span=2.5, concave=false}: plane breaks and overlaps
        m.add_method("edges", |_, fu, o: Option<Table>| {
            let (mut turn, mut step, mut span, mut concave) = (0.8, 3.0, 2.5, false);
            if let Some(o) = &o {
                check_keys(o, &["turn", "step", "span", "concave"], "edges")?;
                turn = num(o, "turn")?.unwrap_or(turn);
                step = num(o, "step")?.unwrap_or(step);
                span = num(o, "span")?.unwrap_or(span);
                concave = o.get::<Option<bool>>("concave")?.unwrap_or(false);
            }
            let mut e = fu.f().edges(turn, step, span);
            if concave {
                let f = e.f;
                let form = fu.f();
                let hollow = Mask::from_fn(f, |x, y| paint::smoothstep(0.3, 0.7, -form.bend(x, y, span)));
                e = e.mul(&hollow);
            }
            Ok(wrap(e))
        });
        // f:mask(function(s) ... end): any rule over samples, at every pixel
        // on the form (serial; `s` is reused: copy what you keep)
        m.add_method("mask", |lua, fu, g: mlua::Function| {
            let f = fu.f().f;
            let mut data = vec![0.0f32; f.w * f.h];
            let t = lua.create_table()?;
            let inv = 1.0 / f.scale;
            for y in 0..f.h {
                for x in 0..f.w {
                    if let Some(s) = fu.f().sample((x as f32 + 0.5) * inv, (y as f32 + 0.5) * inv) {
                        fill_sample(lua, &t, &s)?;
                        let v: f32 = g.call(t.clone())?;
                        data[y * f.w + x] = v.clamp(0.0, 1.0);
                    }
                }
            }
            Ok(wrap(Mask { f, data }))
        });
        m.add_meta_method(MetaMethod::ToString, |_, fu, ()| Ok(format!("form({} parts)", fu.parts)));
    }
}

fn light_of(t: &Table) -> Result<Light> {
    check_keys(t, &["from", "front", "ambient", "bounce", "bounce_from", "penumbra", "reach", "thickness", "across_parts"], "light")?;
    let (fx, fy) = pair(t, "from")?.unwrap_or((-1.0, -0.7));
    let mut l = Light::new((fx, fy), num(t, "front")?.unwrap_or(0.5));
    if let Some(a) = num(t, "ambient")? {
        l = l.ambient(a);
    }
    if let Some(b) = num(t, "bounce")? {
        let from = match t.get::<Value>("bounce_from")? {
            Value::Nil => l.bounce_dir,
            v => v3(&v, "bounce_from")?,
        };
        l = l.bounce(b, from);
    }
    if let Some(p) = num(t, "penumbra")? {
        l = l.penumbra(p);
    }
    if let Some(r) = num(t, "reach")? {
        l = l.reach(r);
    }
    if let Some(th) = num(t, "thickness")? {
        l = l.thickness(th);
    }
    if let Some(a) = t.get::<Option<bool>>("across_parts")? {
        l = l.across_parts(a);
    }
    Ok(l)
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let body = lua.create_table()?;
    body.set("ellipsoid", lua.create_function(|_, (c, r): (Value, Value)| Ok(SolidU::Body(Arc::new(Sdf::ellipsoid(v3(&c, "center")?, v3(&r, "radii")?)))))?)?;
    body.set("block", lua.create_function(|_, (c, size, round): (Value, Value, Option<f32>)| {
        Ok(SolidU::Body(Arc::new(Sdf::block(v3(&c, "center")?, v3(&size, "size")?, round.unwrap_or(2.0)))))
    })?)?;
    body.set("half_space", lua.create_function(|_, (at, n): (Value, Value)| Ok(SolidU::Body(Arc::new(Sdf::half_space(v3(&at, "at")?, v3(&n, "normal")?)))))?)?;
    g.set("body", body)?;

    // ridge{x0=, x1=, crest=fn(x) or points, depth=, seed=, lean={lean, foot},
    //       gullies={spacing, carve}, fan=, strata={spacing, step, tilt}, z0=, base=}
    g.set("ridge", lua.create_function(|_, o: Table| {
        check_keys(&o, &["x0", "x1", "crest", "depth", "seed", "lean", "gullies", "fan", "strata", "z0", "base"], "ridge")?;
        let x0 = num(&o, "x0")?.unwrap_or(0.0);
        let x1 = num(&o, "x1")?.unwrap_or(1000.0);
        let crest: Vec<f32> = match o.get::<Value>("crest")? {
            Value::Function(f) => {
                let n = (x1 - x0).ceil().max(1.0) as usize + 1;
                (0..n).map(|i| f.call::<f32>(x0 + i as f32)).collect::<Result<_>>()?
            }
            v @ Value::Table(_) => {
                let pts = crate::api::points(&v)?;
                if pts.len() < 2 {
                    return err("ridge crest: at least two points");
                }
                let n = (x1 - x0).ceil().max(1.0) as usize + 1;
                (0..n)
                    .map(|i| {
                        let x = x0 + i as f32;
                        let k = pts.windows(2).position(|w| x <= w[1].0).unwrap_or(pts.len() - 2);
                        let (a, b) = (pts[k], pts[k + 1]);
                        let t = ((x - a.0) / (b.0 - a.0).max(1e-6)).clamp(0.0, 1.0);
                        Ok(a.1 + (b.1 - a.1) * t)
                    })
                    .collect::<Result<_>>()?
            }
            _ => return err("ridge: needs crest = function(x) or a list of points"),
        };
        let depth = num(&o, "depth")?.unwrap_or(300.0);
        let seed = o.get::<Option<u32>>("seed")?.unwrap_or(1);
        let mut r = Ridge::new(x0, x1, |x| crest[((x - x0).round().max(0.0) as usize).min(crest.len() - 1)], depth, seed);
        if let Some((l, f)) = pair(&o, "lean")? {
            r = r.lean(l, f);
        }
        if let Some((s, c)) = pair(&o, "gullies")? {
            r = r.gullies(s, c);
        }
        if let Some(f) = num(&o, "fan")? {
            r = r.fan(f);
        }
        if let Some(t) = o.get::<Option<Table>>("strata")? {
            r = r.strata(t.get(1)?, t.get(2)?, t.get::<Option<f32>>(3)?.unwrap_or(0.0));
        }
        if let Some(z) = num(&o, "z0")? {
            r = r.z0(z);
        }
        if let Some(b) = num(&o, "base")? {
            r = r.base(b);
        }
        Ok(SolidU::Ridge(Arc::new(r)))
    })?)?;

    // terrain{area={x0, y0, x1, y1}, height=function(x, y) return z (or nil) end, step=1, facet=0}
    // (the engine's Relief; `relief()` is the finishing verb)
    g.set("terrain", lua.create_function(|_, o: Table| {
        check_keys(&o, &["area", "height", "step", "facet"], "terrain")?;
        let a: Vec<f32> = o.get("area")?;
        if a.len() != 4 {
            return err("terrain: area = {x0, y0, x1, y1}");
        }
        let area = [a[0].min(a[2]), a[1].min(a[3]), a[0].max(a[2]), a[1].max(a[3])];
        let h: mlua::Function = o.get("height")?;
        let step = num(&o, "step")?.unwrap_or(1.0).max(0.25);
        let nx = ((area[2] - area[0]) / step).ceil() as usize + 1;
        let ny = ((area[3] - area[1]) / step).ceil() as usize + 1;
        let mut z = Vec::with_capacity(nx * ny);
        for j in 0..ny {
            for i in 0..nx {
                let v: Option<f32> = h.call((area[0] + i as f32 * step, area[1] + j as f32 * step))?;
                z.push(v.unwrap_or(f32::NAN));
            }
        }
        let facet = o.get::<Option<u16>>("facet")?.unwrap_or(0);
        Ok(SolidU::Relief(Arc::new(ReliefGrid { area, step, nx: nx.max(2), ny: ny.max(2), z, facet })))
    })?)?;

    // form{ {solid, dist=0.5}, {solid, dist={at, per_z}}, ..., light={from=, front=, ...} }
    let st1 = st.clone();
    g.set("form", lua.create_function(move |_, o: Table| {
        let f = frame(&st1)?;
        let mut form = Form::new(f);
        let mut parts = 0u16;
        for kv in o.clone().pairs::<Value, Value>() {
            let (k, _) = kv?;
            match &k {
                Value::Integer(_) => {}
                Value::String(s) if &*s.to_str()? == "light" => {}
                _ => return err("form{}: a list of {solid, dist=} entries and light={...}"),
            }
        }
        for e in o.clone().sequence_values::<Value>() {
            let (solid, dist): (SolidU, Value) = match e? {
                v @ Value::UserData(_) => (solid_of(&v)?, Value::Nil),
                Value::Table(t) => {
                    check_keys(&t, &["dist"], "form entry")?;
                    (solid_of(&t.get::<Value>(1)?)?, t.get("dist")?)
                }
                o => return err(format!("form entry: want a solid or {{solid, dist=}}, got {}", o.type_name())),
            };
            let (at, per_z) = match dist {
                Value::Nil => (0.0, 0.0),
                Value::Table(t) => (t.get(1)?, t.get::<Option<f32>>(2)?.unwrap_or(0.0)),
                v => (f32::from_lua_value(v)?, 0.0),
            };
            let dist = move |_: f32, _: f32, z: f32| at + per_z * z;
            match &solid {
                SolidU::Body(b) => form.add_at(&**b, &dist),
                SolidU::Ridge(r) => form.add_at(&**r, &dist),
                SolidU::Relief(g) => {
                    let g = g.clone();
                    let rel = Relief::new(g.area, move |x, y| g.get(x, y));
                    form.add_at(&rel, &dist)
                }
            };
            parts += 1;
        }
        if let Some(l) = o.get::<Option<Table>>("light")? {
            form.light(light_of(&l)?);
        }
        crate::api::note_bytes(f.w * f.h * 28);
        Ok(FormU { src: Arc::new(form), parts })
    })?)?;

    g.set("aerial", lua.create_function(|_, (d, vis): (f32, f32)| Ok(aerial(d, vis)))?)?;
    Ok(())
}
