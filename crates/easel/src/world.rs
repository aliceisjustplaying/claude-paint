//! The scene and the air in Lua: one world (camera, ground, water, sun,
//! bodies), what the eye sees in it (a view: sky, land, water, shadows,
//! contact, reflections, and a form of the bodies), the sky and clouds for
//! its sun, haze, and receding mountain ranges.
//!
//! As everywhere in the easel, every value is immutable: `w:place` returns a
//! new world, so undo and rollback never have to repair one. A world is kept
//! as its recipe and rebuilt when a body is added (cheap: nothing is traced
//! until `w:view()`).

use crate::api::{Col, FromLuaValue, S, check_keys, err, frame, num, pair, points, wrap};
use crate::form::{FormHolder, FormU, SolidU, shade_table};
use mlua::{Function, Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value};
use paint::atmos::{CloudField, CloudPoint, RangeLayer, Silhouette};
use paint::scene::{Point, View, What};
use paint::{Cloud, Clouds, Form, Haze, Mask, Ranges, Sdf, Sky, SkyField, Spot, Sun, Water, World};
use std::sync::Arc;

// ---------------------------------------------------------------- ground

/// The painter's ground function sampled over the view: across in X/Z (the
/// canvas direction) and along in log Z (so near ground gets as much detail
/// on the canvas as far).
pub struct GroundGrid {
    u0: f32,
    du: f32,
    l0: f32,
    dl: f32,
    nu: usize,
    nl: usize,
    h: Vec<f32>,
}

impl GroundGrid {
    fn get(&self, x: f32, z: f32) -> f32 {
        let z = z.max(1e-3);
        let fu = ((x / z - self.u0) / self.du).clamp(0.0, (self.nu - 1) as f32);
        let fl = ((z.ln() - self.l0) / self.dl).clamp(0.0, (self.nl - 1) as f32);
        let (i, j) = ((fu as usize).min(self.nu - 2), (fl as usize).min(self.nl - 2));
        let (tu, tl) = (fu - i as f32, fl - j as f32);
        let at = |a: usize, b: usize| self.h[b * self.nu + a];
        (at(i, j) * (1.0 - tu) + at(i + 1, j) * tu) * (1.0 - tl) + (at(i, j + 1) * (1.0 - tu) + at(i + 1, j + 1) * tu) * tl
    }
}

/// Water: its level, and its ripple (slope, across, deep, seed).
type WaterSpec = (f32, Option<(f32, f32, f32, u32)>);

/// How a world was made: rebuilt whole when a body is placed.
#[derive(Clone)]
struct Recipe {
    view: [f32; 4],
    horizon: f32,
    eye: f32,
    fov: f32,
    ground: Option<Arc<GroundGrid>>,
    water: Option<WaterSpec>,
    sun: Sun,
    visibility: Option<f32>,
    backdrop: Option<f32>,
    bodies: Vec<(Spot, Sdf, bool)>,
    /// Motifs painted by hand, at a depth (depth.rs).
    layers: Vec<(String, Arc<Mask>, paint::scene::LayerDepth)>,
}

impl Recipe {
    fn build(&self) -> World {
        let mut w = World::new(self.view, self.horizon, self.eye).fov(self.view[2], self.fov);
        if let Some(g) = &self.ground {
            let g = g.clone();
            w = w.ground(move |x, z| g.get(x, z));
        }
        if let Some((level, ripple)) = self.water {
            let mut wa = Water::new(level);
            if let Some((slope, across, deep, seed)) = ripple {
                wa = wa.ripple(slope, across, deep, seed);
            }
            w = w.water(wa);
        }
        w = w.sun(self.sun);
        if let Some(v) = self.visibility {
            w = w.visibility(v);
        }
        if let Some(b) = self.backdrop {
            w = w.backdrop(b);
        }
        for (spot, sdf, proxy) in &self.bodies {
            if *proxy {
                w.proxy(*spot, sdf.clone());
            } else {
                w.place(*spot, sdf.clone());
            }
        }
        for (name, m, d) in &self.layers {
            w.layer(name, (**m).clone(), *d);
        }
        w
    }
}

#[derive(Clone)]
pub struct WorldU {
    recipe: Arc<Recipe>,
    pub w: Arc<World>,
}

// ---------------------------------------------------------------- spots

#[derive(Clone, Copy)]
pub struct SpotU(pub Spot);

impl UserData for SpotU {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("x", |_, s| Ok(s.0.x));
        f.add_field_method_get("y", |_, s| Ok(s.0.y));
        f.add_field_method_get("s", |_, s| Ok(s.0.s));
        f.add_field_method_get("z", |_, s| Ok(s.0.z));
        f.add_field_method_get("at", |_, s| Ok(s.0.at.to_vec()));
    }
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        // s:p(right, up, toward) meters from the foot -> {x, y, z} (units, for solids)
        m.add_method("p", |_, s, (r, u, t): (f32, f32, Option<f32>)| Ok(s.0.p(r, u, t.unwrap_or(0.0)).to_vec()));
        m.add_method("m", |_, s, meters: f32| Ok(s.0.m(meters)));
        m.add_method("size", |_, s, (w, h, d): (f32, f32, f32)| Ok(s.0.size(w, h, d).to_vec()));
        m.add_meta_method(MetaMethod::ToString, |_, s, ()| {
            Ok(format!("spot(x {:.1}, y {:.1}, {:.2} units/m, at {:.1}, {:.1}, {:.1} m)", s.0.x, s.0.y, s.0.s, s.0.at[0], s.0.at[1], s.0.at[2]))
        });
    }
}

fn spot_of(v: &Value) -> Result<Spot> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<SpotU>()?.0),
        o => err(format!("want a spot (w:spot_at(X, Z), w:spot(x, y)), got {}", o.type_name())),
    }
}

fn sdf_of(v: &Value) -> Result<Sdf> {
    match crate::form::solid_of(v)? {
        SolidU::Body(b) => Ok((*b).clone()),
        _ => err("only bodies (body.ellipsoid, body.block) stand in a world"),
    }
}

// ---------------------------------------------------------------- the view

/// A view and the world it borrows, kept alive together.
pub struct ViewBox {
    // declared first: dropped before the world it borrows
    pub(crate) view: View<'static>,
    pub(crate) world: Arc<World>,
    /// Form parts: the visible bodies (a proxy casts shadow but has no part).
    parts: usize,
}

impl FormHolder for ViewBox {
    fn form(&self) -> &Form {
        &self.view.form
    }
}

#[derive(Clone)]
pub struct ViewU(pub Arc<ViewBox>);

fn what_str(w: What) -> (&'static str, Option<usize>) {
    match w {
        What::Off => ("off", None),
        What::Sky => ("sky", None),
        What::Ground => ("ground", None),
        What::Water => ("water", None),
        What::Body(b) => ("body", Some(b + 1)),
    }
}

fn point_table(lua: &Lua, p: &Point) -> Result<Table> {
    let t = lua.create_table()?;
    let (what, body) = what_str(p.what);
    t.set("what", what)?;
    t.set("body", body)?;
    t.set("at", p.at.to_vec())?;
    t.set("n", p.n.to_vec())?;
    t.set("dist", p.dist)?;
    t.set("shade", shade_table(lua, &p.shade)?)?;
    t.set("lit", p.shade.lit(0.12))?;
    Ok(t)
}

fn bodies_of(v: Value, n: usize) -> Result<Vec<usize>> {
    Ok(match v {
        Value::Nil => (0..n).collect(),
        Value::Integer(b) => vec![(b as usize).saturating_sub(1)],
        Value::Table(t) => t.sequence_values::<usize>().map(|b| b.map(|b| b.saturating_sub(1))).collect::<Result<_>>()?,
        o => return err(format!("bodies: want a body number or a list, got {}", o.type_name())),
    })
}

impl UserData for ViewU {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        // the bodies as a form: every form query, mask and field works on it
        f.add_field_method_get("form", |_, v| Ok(FormU { src: v.0.clone(), parts: v.0.parts as u16 }));
    }
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("at", |lua, v, (x, y): (f32, f32)| point_table(lua, &v.0.view.at(x, y)));
        m.add_method("what", |_, v, (x, y): (f32, f32)| Ok(what_str(v.0.view.at(x, y).what).0));
        m.add_method("cast", |_, v, (x, y): (f32, f32)| Ok(v.0.view.cast(x, y)));
        m.add_method("ground_depth", |_, v, (x, y): (f32, f32)| Ok(v.0.view.ground_depth(x, y)));
        m.add_method("part", |_, v, b: usize| Ok(v.0.view.part(b.saturating_sub(1))));
        m.add_method("sky", |_, v, ()| Ok(wrap(v.0.view.sky())));
        m.add_method("land", |_, v, ()| Ok(wrap(v.0.view.land())));
        m.add_method("water", |_, v, ()| Ok(wrap(v.0.view.water())));
        m.add_method("shadows", |_, v, ()| Ok(wrap(v.0.view.shadows())));
        m.add_method("contact", |_, v, reach: Option<f32>| Ok(wrap(v.0.view.contact(reach.unwrap_or(0.25)))));
        m.add_method("reflections", |_, v, bodies: Value| {
            let b = bodies_of(bodies, v.0.world.bodies.len())?;
            Ok(wrap(v.0.view.reflections(&b)))
        });
        // v:bodies_mask(ids): where those bodies are seen
        m.add_method("bodies_mask", |_, v, bodies: Value| {
            let b = bodies_of(bodies, v.0.world.bodies.len())?;
            Ok(wrap(v.0.view.mask(|p| matches!(p.what, What::Body(i) if b.contains(&i)) as u8 as f32)))
        });
        // v:mirror(x, y): what the water shows there, or nil
        m.add_method("mirror", |lua, v, (x, y): (f32, f32)| match v.0.view.mirror(x, y) {
            None => Ok(Value::Nil),
            Some(mi) => {
                let t = lua.create_table()?;
                t.set("body", mi.body.map(|b| b + 1))?;
                t.set("src", vec![mi.src.0, mi.src.1])?;
                t.set("at", mi.at.to_vec())?;
                t.set("n", mi.n.to_vec())?;
                t.set("shade", shade_table(lua, &mi.shade)?)?;
                t.set("fresnel", mi.fresnel)?;
                t.set("travel", mi.travel)?;
                Ok(Value::Table(t))
            }
        });
        // v:mask(function(p) ... end): any rule over what is seen (serial)
        m.add_method("mask", |lua, v, g: Function| {
            let f = v.0.view.f;
            let inv = 1.0 / f.scale;
            let mut data = vec![0.0f32; f.w * f.h];
            for y in 0..f.h {
                for x in 0..f.w {
                    let p = v.0.view.at((x as f32 + 0.5) * inv, (y as f32 + 0.5) * inv);
                    if p.what != What::Off {
                        let r: f32 = g.call(point_table(lua, &p)?)?;
                        data[y * f.w + x] = r.clamp(0.0, 1.0);
                    }
                }
            }
            Ok(wrap(Mask { f, data }))
        });
        crate::depth::view_methods(m);
    }
}

// ---------------------------------------------------------------- sky, clouds

#[derive(Clone)]
pub struct SkyU(pub Arc<SkyField>);

impl UserData for SkyU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("at", |_, s, (x, y): (f32, f32)| Ok(Col(s.0.at(x, y))));
        m.add_method("value", |_, s, (x, y): (f32, f32)| Ok(s.0.value(x, y)));
        m.add_method("airlight", |_, s, x: f32| Ok(Col(s.0.airlight(x))));
        m.add_method("sunlight", |_, s, ()| Ok(Col(s.0.paint(s.0.sky.sunlight()))));
        m.add_meta_method(MetaMethod::ToString, |_, _, ()| Ok("sky (use as color=, or s:at(x, y))"));
    }
}

#[derive(Clone)]
pub struct CloudsU {
    pub sky: Arc<SkyField>,
    pub field: Arc<CloudField>,
}

fn cloud_point_table(lua: &Lua, p: &CloudPoint) -> Result<Table> {
    let t = lua.create_table()?;
    t.set("alpha", p.alpha)?;
    t.set("lit", p.lit)?;
    t.set("glow", p.glow)?;
    t.set("ambient", p.ambient)?;
    t.set("dist", p.dist)?;
    Ok(t)
}

impl UserData for CloudsU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        // the sky with its clouds, as paint (also usable directly as color=)
        m.add_method("at", |_, c, (x, y): (f32, f32)| Ok(Col(c.field.color(&c.sky, x, y))));
        m.add_method("cloud", |_, c, (x, y): (f32, f32)| Ok(Col(c.field.cloud_color(x, y))));
        m.add_method("alpha", |_, c, (x, y): (f32, f32)| Ok(c.field.alpha(x, y)));
        m.add_method("lit", |_, c, (x, y): (f32, f32)| Ok(c.field.lit(x, y)));
        m.add_method("glow", |_, c, (x, y): (f32, f32)| Ok(c.field.glow(x, y)));
        m.add_method("soft", |_, c, (x, y): (f32, f32)| Ok(c.field.soft(x, y)));
        // c:mask{alpha={lo, hi}, lit={lo, hi}, shade={lo, hi}}: smoothsteps multiplied
        // (shade = the shadowed bellies, 1 - lit); or c:mask(function(p) ... end)
        m.add_method("mask", |lua, c, o: Value| {
            match o {
                Value::Function(g) => {
                    // serial: sample the rule on the cloud grid's own cells
                    let fr = crate::api::current_frame(lua)?;
                    let inv = 1.0 / fr.scale;
                    let mut data = vec![0.0f32; fr.w * fr.h];
                    for y in 0..fr.h {
                        for x in 0..fr.w {
                            let (ux, uy) = ((x as f32 + 0.5) * inv, (y as f32 + 0.5) * inv);
                            let p = CloudPoint { alpha: c.field.alpha(ux, uy), lit: c.field.lit(ux, uy), glow: c.field.glow(ux, uy), ambient: c.field.ambient(ux, uy), dist: c.field.dist(ux, uy) };
                            if p.alpha > 0.0 {
                                let r: f32 = g.call(cloud_point_table(lua, &p)?)?;
                                data[y * fr.w + x] = r.clamp(0.0, 1.0);
                            }
                        }
                    }
                    Ok(wrap(Mask { f: fr, data }))
                }
                Value::Table(t) => {
                    check_keys(&t, &["alpha", "lit", "shade"], "clouds mask")?;
                    let a = pair(&t, "alpha")?.unwrap_or((0.3, 0.8));
                    let l = pair(&t, "lit")?;
                    let s = pair(&t, "shade")?;
                    let fr = crate::api::current_frame(lua)?;
                    Ok(wrap(c.field.mask(fr, |p| {
                        let mut v = paint::smoothstep(a.0, a.1, p.alpha);
                        if let Some(l) = l {
                            v *= paint::smoothstep(l.0, l.1, p.lit);
                        }
                        if let Some(s) = s {
                            v *= paint::smoothstep(s.0, s.1, 1.0 - p.lit);
                        }
                        v
                    })))
                }
                Value::Nil => {
                    let fr = crate::api::current_frame(lua)?;
                    Ok(wrap(c.field.mask(fr, |p| paint::smoothstep(0.3, 0.8, p.alpha))))
                }
                o => err(format!("clouds mask: want {{alpha=, lit=, shade=}} or a function, got {}", o.type_name())),
            }
        });
        m.add_meta_method(MetaMethod::ToString, |_, _, ()| Ok("clouds (use as color=, or c:at(x, y))"));
    }
}

fn cloud_of(t: &Table) -> Result<Cloud> {
    let kind: String = t.get::<Option<String>>("kind")?.or(t.get::<Option<String>>(1)?).unwrap_or_else(|| "cumulus".into());
    let n = |k: &str| -> Result<f32> { num(t, k)?.ok_or_else(|| mlua::Error::runtime(format!("{kind} cloud: needs {k}"))) };
    let seed = t.get::<Option<u32>>("seed")?.unwrap_or(1);
    let mut c = match kind.as_str() {
        "cumulus" => {
            check_keys(t, &["kind", "x", "z", "base", "width", "height", "seed", "density", "soft", "wind", "breaks", "heap", "reach"], "cumulus")?;
            Cloud::cumulus(n("x")?, n("z")?, n("base")?, n("width")?, n("height")?, seed)
        }
        "bank" => {
            check_keys(t, &["kind", "x0", "x1", "z", "depth", "base", "top", "seed", "density", "soft", "wind", "breaks", "heap", "reach"], "bank")?;
            Cloud::bank(n("x0")?, n("x1")?, n("z")?, n("depth")?, n("base")?, n("top")?, seed)
        }
        "stratus" => {
            check_keys(t, &["kind", "base", "thick", "cover", "seed", "density", "soft", "wind", "breaks", "heap", "reach"], "stratus")?;
            Cloud::stratus(n("base")?, n("thick")?, n("cover")?, seed)
        }
        o => return err(format!("cloud kind {o:?}: cumulus, bank or stratus")),
    };
    if let Some(d) = num(t, "density")? {
        c = c.density(d);
    }
    if let Some(s) = num(t, "soft")? {
        c = c.soft(s);
    }
    if let Some((a, s)) = pair(t, "wind")? {
        c = c.wind(a, s);
    }
    if let Some((p, a)) = pair(t, "breaks")? {
        c = c.breaks(p, a);
    }
    if let Some(h) = num(t, "heap")? {
        c = c.heap(h);
    }
    if let Some((near, far)) = pair(t, "reach")? {
        c = c.reach(near, far);
    }
    Ok(c)
}

// ---------------------------------------------------------------- haze, ranges

#[derive(Clone, Copy)]
pub struct HazeU(pub Haze);
impl UserData for HazeU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        // air:loss(eye_m, dist_m, height_m, x): how much of a color the air replaces
        m.add_method("loss", |_, h, (eye, dist, hm, x): (f32, f32, Option<f32>, Option<f32>)| Ok(h.0.loss(eye, dist, hm.unwrap_or(0.0), x.unwrap_or(0.0))));
    }
}

fn haze_of(v: &Value) -> Result<Haze> {
    match v {
        Value::UserData(u) => Ok(u.borrow::<HazeU>()?.0),
        o => err(format!("want haze{{...}}, got {}", o.type_name())),
    }
}

#[derive(Clone)]
pub struct RangeU {
    layer: Arc<RangeLayer>,
    world: Arc<World>,
}

impl UserData for RangeU {
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("crest", |_, r, x: f32| Ok(r.layer.crest(&r.world, x)));
        m.add_method("z_at", |_, r, x: f32| Ok(r.layer.z_at(x)));
        m.add_method("height_at", |_, r, (x, y): (f32, f32)| Ok(r.layer.height_at(&r.world, x, y)));
        m.add_method("haze", |_, r, (air, x, y): (Value, f32, f32)| Ok(r.layer.haze(&r.world, &haze_of(&air)?, x, y)));
        // l:mask(soft?): everything below the crest (hidden parts included)
        m.add_method("mask", |lua, r, soft: Option<f32>| {
            let f = crate::api::current_frame(lua)?;
            let s = soft.unwrap_or(0.6).max(0.05);
            let (l, w) = (r.layer.clone(), r.world.clone());
            let crest: Vec<f32> = (0..f.w).map(|x| l.crest(&w, (x as f32 + 0.5) / f.scale)).collect();
            Ok(wrap(Mask::from_fn(f, |x, y| {
                let c = crest[((x * f.scale) as usize).min(f.w - 1)];
                paint::smoothstep(c - s, c + s, y)
            })))
        });
        // l:ridge{x0=0, x1=1000, depth=200, gully=260}: a form ridge sized for its distance
        m.add_method("ridge", |_, r, o: Option<Table>| {
            let (mut x0, mut x1, mut depth, mut gully) = (0.0, 1000.0, 200.0, 260.0);
            if let Some(o) = &o {
                check_keys(o, &["x0", "x1", "depth", "gully"], "range ridge")?;
                x0 = num(o, "x0")?.unwrap_or(x0);
                x1 = num(o, "x1")?.unwrap_or(x1);
                depth = num(o, "depth")?.unwrap_or(depth);
                gully = num(o, "gully")?.unwrap_or(gully);
            }
            Ok(SolidU::Ridge(Arc::new(r.layer.ridge(&r.world, x0, x1, depth, gully))))
        });
        m.add_meta_method(MetaMethod::ToString, |_, r, ()| Ok(format!("range({:.0} m off at x=500)", r.layer.z_at(500.0))));
    }
}

fn silhouette_of(s: &str) -> Result<Silhouette> {
    Ok(match s {
        "peak" => Silhouette::Peak,
        "dome" => Silhouette::Dome,
        "plateau" => Silhouette::Plateau,
        "saddle" => Silhouette::Saddle,
        "cliff" => Silhouette::Cliff,
        o => return err(format!("range kind {o:?}: peak, dome, plateau, saddle or cliff")),
    })
}

// ---------------------------------------------------------------- world methods

impl UserData for WorldU {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("horizon", |_, w| Ok(w.w.horizon));
        f.add_field_method_get("eye", |_, w| Ok(w.w.eye));
        f.add_field_method_get("bodies", |_, w| Ok(w.w.bodies.len()));
    }
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        m.add_method("spot", |_, w, (x, y): (f32, f32)| Ok(w.w.spot(x, y).map(SpotU)));
        m.add_method("spot_at", |_, w, (x, z): (f32, f32)| Ok(SpotU(w.w.spot_at(x, z))));
        m.add_method("spot_bed", |_, w, (x, z): (f32, f32)| Ok(SpotU(w.w.spot_bed(x, z))));
        m.add_method("project", |_, w, (x, y, z): (f32, f32, f32)| Ok(w.w.project([x, y, z]).map(|(a, b)| vec![a, b])));
        m.add_method("to_ground", |_, w, (x, y): (f32, f32)| Ok(w.w.to_ground(x, y).map(|p| p.to_vec())));
        m.add_method("scale_at", |_, w, z: f32| Ok(w.w.scale_at(z)));
        m.add_method("height", |_, w, (x, y, meters): (f32, f32, f32)| Ok(w.w.height(x, y, meters)));
        m.add_method("aerial", |_, w, z: f32| Ok(w.w.aerial(z)));
        m.add_method("ground_at", |_, w, (x, z): (f32, f32)| Ok(w.w.ground_at(x, z)));
        m.add_method("is_water", |_, w, (x, z): (f32, f32)| Ok(w.w.is_water(x, z)));
        m.add_method("shadow_angle", |_, w, (x, y): (f32, f32)| Ok(w.w.shadow_angle(x, y)));
        m.add_method("sun_canvas", |_, w, ()| Ok(w.w.sun_canvas().map(|(a, b)| vec![a, b])));
        // w:place(spot, body) / w:proxy(spot, body) -> the new world, the body's number
        m.add_method("place", |_, w, (s, b): (Value, Value)| place(w, spot_of(&s)?, sdf_of(&b)?, false));
        m.add_method("proxy", |_, w, (s, b): (Value, Value)| place(w, spot_of(&s)?, sdf_of(&b)?, true));
        // w:layer(name, mask, depth) -> the new world, the layer's number (depth.rs)
        m.add_method("layer", |_, w, (name, mask, depth): (Value, Value, Value)| {
            let (name, mask, d) = crate::depth::layer_args(&w.w, &name, &mask, &depth)?;
            let mut r = (*w.recipe).clone();
            r.layers.push((name, Arc::new(mask), d));
            let n = r.layers.len();
            let world = Arc::new(r.build());
            Ok((WorldU { recipe: Arc::new(r), w: world }, n))
        });
        // w:ribbon({{X, Z}, ...}, width_m or function(t) -> m): a path on the ground
        m.add_method("ribbon", |lua, w, (p, width): (Value, Value)| {
            let pts = points(&p)?;
            let shape = match width {
                Value::Function(f) => {
                    // sampled along the path (serial Lua)
                    let ws: Vec<f32> = (0..=64).map(|i| f.call::<f32>(i as f32 / 64.0)).collect::<Result<_>>()?;
                    w.w.ribbon(&pts, |t| ws[((t.clamp(0.0, 1.0) * 64.0).round() as usize).min(64)])
                }
                v => {
                    let k = f32::from_lua_value(v)?;
                    w.w.ribbon(&pts, |_| k)
                }
            };
            Ok(wrap(Mask::from_shape(crate::api::current_frame(lua)?, shape)))
        });
        m.add_method("line", |_, w, p: Value| Ok(w.w.line(&points(&p)?).into_iter().map(|(a, b)| vec![a, b]).collect::<Vec<_>>()));
        // w:recede({X, Z}, {dX, dZ}, n): spots stepping away (fence posts, footprints)
        m.add_method("recede", |_, w, (start, step, n): (Vec<f32>, Vec<f32>, usize)| {
            if start.len() < 2 || step.len() < 2 {
                return err("recede({X, Z}, {dX, dZ}, n)");
            }
            Ok(w.w.recede((start[0], start[1]), (step[0], step[1]), n).into_iter().map(SpotU).collect::<Vec<_>>())
        });
        // w:view(): what the eye sees (traced once; keep it)
        m.add_method("view", |lua, w, ()| {
            let f = crate::api::current_frame(lua)?;
            let world = w.w.clone();
            let v: View<'_> = world.view(f);
            // SAFETY: the view borrows the world, which the box keeps alive
            // in an Arc (its address never moves) and drops after the view.
            let view: View<'static> = unsafe { std::mem::transmute::<View<'_>, View<'static>>(v) };
            crate::api::note_bytes(f.w * f.h * 40);
            let parts = w.w.bodies.iter().filter(|b| b.visible).count();
            let vu = ViewU(Arc::new(ViewBox { view, world: w.w.clone(), parts }));
            // the view depth options (visible=, behind=, at=) use by default
            if let Some(st) = lua.app_data_ref::<S>() {
                st.borrow_mut().view = Some(vu.clone());
            }
            Ok(vu)
        });
        // w:sky{haze=, uneven={amount, period_m, seed}, layer={alt, thick, density, uneven, seed},
        //       overcast=, fill=, altitude=, cell=6, exposure=, balance=}
        m.add_method("sky", |_, w, o: Option<Table>| {
            let mut sky = Sky::new(w.w.sun);
            let mut cell = 6.0;
            let (mut exposure, mut balance) = (None, None);
            if let Some(o) = &o {
                check_keys(o, &["haze", "uneven", "layer", "overcast", "fill", "altitude", "cell", "exposure", "balance"], "sky")?;
                if let Some(h) = num(o, "haze")? {
                    sky = sky.haze(h);
                }
                if let Some(t) = o.get::<Option<Table>>("uneven")? {
                    sky = sky.uneven(t.get(1)?, t.get::<Option<f32>>(2)?.unwrap_or(30_000.0), t.get::<Option<u32>>(3)?.unwrap_or(1));
                }
                if let Some(t) = o.get::<Option<Table>>("layer")? {
                    sky = sky.layer(t.get(1)?, t.get(2)?, t.get(3)?, t.get::<Option<f32>>(4)?.unwrap_or(0.5), t.get::<Option<u32>>(5)?.unwrap_or(1));
                }
                if let Some(v) = num(o, "overcast")? {
                    sky = sky.overcast(v);
                }
                if let Some(v) = num(o, "fill")? {
                    sky = sky.fill(v);
                }
                if let Some(v) = num(o, "altitude")? {
                    sky = sky.altitude(v);
                }
                cell = num(o, "cell")?.unwrap_or(cell);
                exposure = num(o, "exposure")?;
                balance = num(o, "balance")?;
            }
            let mut f = SkyField::new(sky, &w.w, cell);
            if let Some(k) = exposure {
                f = f.exposure(k);
            }
            if let Some(b) = balance {
                let l = f.sky.sunlight();
                f = f.balance(l, b);
            }
            Ok(SkyU(Arc::new(f)))
        });
        // w:clouds{sky=s, cell=2, {kind="cumulus", ...}, {kind="bank", ...}, ...}
        m.add_method("clouds", |_, w, o: Table| {
            let sky = match o.get::<Value>("sky")? {
                Value::UserData(u) => u.borrow::<SkyU>()?.0.clone(),
                _ => return err("clouds: needs sky = w:sky{...}"),
            };
            let cell = num(&o, "cell")?.unwrap_or(2.0);
            let list: Vec<Cloud> = o.sequence_values::<Table>().map(|t| cloud_of(&t?)).collect::<Result<_>>()?;
            if list.is_empty() {
                return err("clouds: list at least one {kind=\"cumulus\"|\"bank\"|\"stratus\", ...}");
            }
            let field = Clouds::new(list).field(&sky, &w.w, cell);
            Ok(CloudsU { sky, field: Arc::new(field) })
        });
        // w:ranges{near=, far=, count=, seed=, heights={near_m, far_m}, irregular=, oblique=,
        //          kinds={"peak", "dome", ...}, regular=false} -> layers, nearest first
        m.add_method("ranges", |_, w, o: Table| {
            check_keys(&o, &["near", "far", "count", "seed", "heights", "irregular", "oblique", "kinds", "regular"], "ranges")?;
            let mut r = Ranges::new(num(&o, "near")?.unwrap_or(4000.0), num(&o, "far")?.unwrap_or(40_000.0), o.get::<Option<usize>>("count")?.unwrap_or(4), o.get::<Option<u32>>("seed")?.unwrap_or(1));
            if let Some((a, b)) = pair(&o, "heights")? {
                r = r.heights(a, b);
            }
            if let Some(k) = num(&o, "irregular")? {
                r = r.irregular(k);
            }
            if let Some(k) = num(&o, "oblique")? {
                r = r.oblique(k);
            }
            if let Some(k) = o.get::<Option<Vec<String>>>("kinds")? {
                let ks: Vec<Silhouette> = k.iter().map(|s| silhouette_of(s)).collect::<Result<_>>()?;
                r = r.kinds(&ks);
            }
            let layers = if o.get::<Option<bool>>("regular")?.unwrap_or(false) { r.regular(&w.w) } else { r.build(&w.w) };
            Ok(layers.into_iter().map(|l| RangeU { layer: Arc::new(l), world: w.w.clone() }).collect::<Vec<_>>())
        });
        m.add_meta_method(MetaMethod::ToString, |_, w, ()| {
            Ok(format!("world(horizon y {:.0}, eye {} m, sun az {:.0}° el {:.0}°, {} bodies)", w.w.horizon, w.w.eye, w.w.sun.azimuth.to_degrees(), w.w.sun.elevation.to_degrees(), w.w.bodies.len()))
        });
    }
}

fn place(w: &WorldU, spot: Spot, sdf: Sdf, proxy: bool) -> Result<(WorldU, usize)> {
    let mut r = (*w.recipe).clone();
    r.bodies.push((spot, sdf, proxy));
    let id = r.bodies.len();
    let world = Arc::new(r.build());
    Ok((WorldU { recipe: Arc::new(r), w: world }, id))
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    // world{horizon=, eye=1.6, fov=50, view={x, y, w, h}, ground=function(X, Z) return m end,
    //       water={level=0, ripple={slope, across, deep, seed}}, sun={azimuth=, elevation=},
    //       visibility=, backdrop=}
    let st1 = st.clone();
    g.set("world", lua.create_function(move |_, o: Table| {
        check_keys(&o, &["horizon", "eye", "fov", "view", "ground", "water", "sun", "visibility", "backdrop"], "world")?;
        let f = frame(&st1)?;
        let view = match o.get::<Option<Vec<f32>>>("view")? {
            Some(v) if v.len() == 4 => [v[0], v[1], v[2], v[3]],
            Some(_) => return err("world view: {x, y, w, h}"),
            None => [0.0, 0.0, f.width(), f.height()],
        };
        let horizon = num(&o, "horizon")?.unwrap_or(view[1] + view[3] * 0.5);
        let eye = num(&o, "eye")?.unwrap_or(1.6);
        let fov = num(&o, "fov")?.unwrap_or(45.0);
        let sun = match o.get::<Option<Table>>("sun")? {
            Some(t) => {
                check_keys(&t, &["azimuth", "elevation"], "sun")?;
                Sun::deg(num(&t, "azimuth")?.unwrap_or(-120.0), num(&t, "elevation")?.unwrap_or(35.0))
            }
            None => Sun::deg(-120.0, 35.0),
        };
        let water = match o.get::<Option<Table>>("water")? {
            Some(t) => {
                check_keys(&t, &["level", "ripple"], "water")?;
                let ripple = match t.get::<Option<Table>>("ripple")? {
                    Some(r) => Some((r.get(1)?, r.get::<Option<f32>>(2)?.unwrap_or(1.4), r.get::<Option<f32>>(3)?.unwrap_or(0.3), r.get::<Option<u32>>(4)?.unwrap_or(1))),
                    None => None,
                };
                Some((num(&t, "level")?.unwrap_or(0.0), ripple))
            }
            None => None,
        };
        let mut r = Recipe { view, horizon, eye, fov, ground: None, water, sun, visibility: num(&o, "visibility")?, backdrop: num(&o, "backdrop")?, bodies: Vec::new(), layers: Vec::new() };
        if let Some(gf) = o.get::<Option<Function>>("ground")? {
            // sample over the view: X/Z across, log Z along
            let cam = r.build();
            let half = (view[2] * 0.5 / cam.focal) * 1.6;
            let (nu, nl) = (257usize, 257usize);
            let (l0, l1) = (0.25f32.ln(), cam.far.max(1.0).ln());
            let (du, dl) = (2.0 * half / (nu - 1) as f32, (l1 - l0) / (nl - 1) as f32);
            let mut h = Vec::with_capacity(nu * nl);
            for j in 0..nl {
                let z = (l0 + j as f32 * dl).exp();
                for i in 0..nu {
                    let x = (-half + i as f32 * du) * z;
                    h.push(gf.call::<f32>((x, z))?);
                }
            }
            r.ground = Some(Arc::new(GroundGrid { u0: -half, du, l0, dl, nu, nl, h }));
        }
        let w = Arc::new(r.build());
        Ok(WorldU { recipe: Arc::new(r), w })
    })?)?;

    // haze{visibility=28000, height=900, mist={top_m, density, uneven_m, seed}}
    g.set("haze", lua.create_function(|_, o: Option<Table>| {
        let o = match o {
            Some(o) => o,
            None => return Ok(HazeU(Haze::new(25_000.0))),
        };
        check_keys(&o, &["visibility", "height", "mist"], "haze")?;
        let mut h = Haze::new(num(&o, "visibility")?.unwrap_or(25_000.0));
        if let Some(k) = num(&o, "height")? {
            h = h.height(k);
        }
        if let Some(t) = o.get::<Option<Table>>("mist")? {
            h = h.mist(t.get(1)?, t.get(2)?, t.get::<Option<f32>>(3)?.unwrap_or(80.0), t.get::<Option<u32>>(4)?.unwrap_or(1));
        }
        Ok(HazeU(h))
    })?)?;
    Ok(())
}
