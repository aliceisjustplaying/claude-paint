//! The scene in Lua: one world (camera, ground, water, sun, and the bodies
//! the painter places there) and what the eye sees in it (a view: where
//! the sky, the land and the water are seen, shadows cast by the sun,
//! contact, mirror images in still water, and a form of the bodies). It is
//! a scaffold for placing things in perspective and for where light and
//! shadow fall on the painter's shapes; it paints nothing and knows
//! nothing of any subject.
//!
//! As everywhere in the easel, every value is immutable: `w:place` returns a
//! new world, so a failed chunk never has to repair one. A world is kept
//! as its recipe and rebuilt when a body is added (cheap: nothing is traced
//! until `w:view()`).

use crate::api::{FromLuaValue, S, check_keys, err, frame, num, points, wrap};
use crate::form::{FormHolder, FormU, SolidU, shade_table};
use mlua::{Function, Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value};
use paint::scene::{Point, View, What};
use paint::{Form, Mask, Sdf, Spot, Sun, Water, World};
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
    /// The painting's engine version (`paint::ENGINE`).
    engine: u32,
}

impl Recipe {
    fn build(&self) -> World {
        let mut w = World::new(self.view, self.horizon, self.eye).fov(self.view[2], self.fov).engine(self.engine);
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
    fn built_z(&self, part: u16, z: f32) -> f32 {
        self.view.built_z(part, z)
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


// ---------------------------------------------------------------- world methods

impl UserData for WorldU {
    fn add_fields<F: mlua::UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("horizon", |_, w| Ok(w.w.horizon));
        f.add_field_method_get("eye", |_, w| Ok(w.w.eye));
        f.add_field_method_get("bodies", |_, w| Ok(w.w.bodies.len()));
    }
    fn add_methods<M_: UserDataMethods<Self>>(m: &mut M_) {
        // an old log's w:sky{}, w:clouds{}, w:ranges{} (legacy.rs; nil otherwise)
        #[cfg(feature = "replay")]
        m.add_meta_function(MetaMethod::Index, |lua, (_, k): (Value, Value)| match k {
            Value::String(k) => crate::legacy::world_method(lua, &k.to_str()?),
            _ => Ok(Value::Nil),
        });
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
        let mut r = Recipe { view, horizon, eye, fov, ground: None, water, sun, visibility: num(&o, "visibility")?, backdrop: num(&o, "backdrop")?, bodies: Vec::new(), layers: Vec::new(), engine: st1.borrow().tubes.engine };
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

    Ok(())
}
