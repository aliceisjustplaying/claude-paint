//! One world, one sun: where things stand, how big they look, and where
//! their shadows, contacts and reflections fall.
//!
//! `form` gives a painter a solid lit by a `Light` of its own; nothing makes
//! two solids in one picture agree. This module is the stage they share:
//!
//! - a **camera**: an eye `eye` meters above the ground, looking level, so the
//!   horizon is a line across the picture at eye level; canvas points map to
//!   points on the ground and back, and a thing at a depth has a scale
//!   (units per meter): a 1.7 m figure standing at a canvas point is
//!   `world.height(x, y, 1.7)` units tall (a figure as tall as the eye has its
//!   head on the horizon wherever it stands);
//! - the **ground**: level, or any gentle height function; **water** fills it
//!   below a level (a pond is a hollow in the ground, a shore is ground that
//!   runs under the sea), still or rippled;
//! - **one sun** (azimuth, elevation). Below the horizon there is no direct
//!   light and no cast shadow, only sky light and the afterglow on the side
//!   facing the glow. `World::light` is the `form::Light` every solid is lit
//!   with;
//! - **bodies**: `Sdf` solids placed on the ground at a `Spot` and built in
//!   canvas units there (`spot.p(right, up, toward)` in meters). A body can be
//!   visible (it goes into the `Form` the painter paints from) or a proxy (a
//!   figure the painter writes with gestures still casts a shadow and shows
//!   in the water);
//! - a **`View`** of the world over a canvas frame: the `Form` of the visible
//!   bodies lit by the world's sun, and fields and masks for the painter: what
//!   is where (sky, ground, water, which body), cast shadows traced along the
//!   sun in the world with a penumbra that grows with distance from the
//!   caster, contact occlusion where solids meet the ground, mirror images in
//!   the water (with ripples, darkening and a Fresnel falloff), depth for
//!   aerial perspective;
//! - **perspective helpers**: ribbons on the ground (a path, a brook) that
//!   narrow as they recede, spots at a depth, spacing that recedes.
//!
//! Like `form`, none of this paints: every mark is still the painter's.
//!
//! World coordinates are meters: X to the right, Y up (0 is the ground's
//! datum), Z away from the viewer; the eye is at (0, eye, 0). The sun's
//! azimuth is measured from straight ahead (0: the sun is behind the motif,
//! contre-jour) toward the right (+90°: from the right; −90°: from the left;
//! 180°: from behind the painter). Normals given to the painter are in the
//! `form` frame (x right, y down, z toward the viewer), so `form::Light`,
//! `Shade` and `Sample::fall` work unchanged.

use crate::canvas::Frame;
use crate::form::{Form, Hit, Light, PartId, Sample, Sdf, Shade, Solid, V3, unit};
use crate::mask::Mask;
use crate::noise::Fbm;
use crate::shape::Shape;
use rayon::prelude::*;

#[inline]
fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline]
fn add(a: V3, b: V3, t: f32) -> V3 {
    [a[0] + b[0] * t, a[1] + b[1] * t, a[2] + b[2] * t]
}
/// World vector (Y up, Z away) to the form frame (y down, z toward the viewer).
#[inline]
pub fn to_form(v: V3) -> V3 {
    [v[0], -v[1], -v[2]]
}
/// Form-frame vector to the world.
#[inline]
pub fn to_world(v: V3) -> V3 {
    [v[0], -v[1], -v[2]]
}

/// Form depth per meter of world depth between bodies: large, so a nearer
/// body is always in front of a farther one.
const ZK: f32 = 1000.0;

// -------------------------------------------------------------------- sun

/// The one sun. Angles in radians (`Sun::deg` takes degrees).
#[derive(Clone, Copy, Debug)]
pub struct Sun {
    /// From straight ahead (0, behind the motif) toward the right.
    pub azimuth: f32,
    /// Above the horizon (negative: set or not yet risen).
    pub elevation: f32,
}

impl Sun {
    pub fn deg(azimuth: f32, elevation: f32) -> Self {
        Sun { azimuth: azimuth.to_radians(), elevation: elevation.to_radians() }
    }
    /// Unit vector toward the sun (world).
    pub fn dir(&self) -> V3 {
        let (sa, ca) = self.azimuth.sin_cos();
        let (se, ce) = self.elevation.sin_cos();
        [sa * ce, se, ca * ce]
    }
    /// True if it gives direct light (above the horizon).
    pub fn up(&self) -> bool {
        self.elevation > 0.0
    }
    /// The afterglow (and foreglow) on the sky low over where the sun is:
    /// 0 with the sun high or far below, most when it is just under the
    /// horizon. A fraction of full sunlight.
    pub fn glow(&self) -> f32 {
        let e = self.elevation.to_degrees();
        0.4 * crate::smoothstep(-14.0, -3.0, e) * (1.0 - crate::smoothstep(0.0, 10.0, e))
    }
    /// Unit vector toward the brightest part of the sky low over the sun
    /// (world): where the glow light comes from when the sun is down.
    pub fn glow_dir(&self) -> V3 {
        Sun { azimuth: self.azimuth, elevation: self.elevation.max(0.1) }.dir()
    }
}

// ------------------------------------------------------------------ water

/// Water lying in the ground's hollows up to `level` (m).
pub struct Water {
    pub level: f32,
    /// Ripple slope (0 still, 0.02 a breath of wind, 0.08 choppy).
    pub ripple: f32,
    /// Ripple wavelengths (m): along the wind (across the picture) and in depth.
    pub wave: (f32, f32),
    sx: Fbm,
    sz: Fbm,
}

impl Water {
    pub fn new(level: f32) -> Self {
        Water { level, ripple: 0.0, wave: (1.2, 0.35), sx: Fbm::new(911, 3, 1.0), sz: Fbm::new(912, 3, 1.0) }
    }
    /// Ripples: slope amplitude and wavelengths (m) across and in depth.
    pub fn ripple(mut self, slope: f32, across: f32, deep: f32, seed: u32) -> Self {
        self.ripple = slope;
        self.wave = (across.max(1e-3), deep.max(1e-3));
        self.sx = Fbm::new(seed, 3, 1.0);
        self.sz = Fbm::new(seed + 1, 3, 1.0);
        self
    }
    /// Surface normal (world) at a point of the water.
    pub fn normal(&self, x: f32, z: f32) -> V3 {
        if self.ripple <= 0.0 {
            return [0.0, 1.0, 0.0];
        }
        let (u, v) = (x / self.wave.0, z / self.wave.1);
        // the slope in depth is the strong one: ripples are long crests across
        let gx = 0.4 * self.ripple * self.sx.get(u, v);
        let gz = self.ripple * self.sz.get(u, v);
        unit([-gx, 1.0, -gz])
    }
}

// ----------------------------------------------------------------- bodies

/// A place on the ground and the canvas-unit frame there: build a body's
/// `Sdf` with `p` so it stands at this spot at the right size.
#[derive(Clone, Copy, Debug)]
pub struct Spot {
    /// The foot on the canvas (units).
    pub x: f32,
    pub y: f32,
    /// Units per meter at this depth.
    pub s: f32,
    /// Form depth of the foot (bodies nearer the viewer are in front).
    pub z: f32,
    /// The foot in the world (m).
    pub at: V3,
}

impl Spot {
    /// The canvas-unit point `right` m to the right, `up` m above and
    /// `toward` m toward the viewer from the foot (for `Sdf` centers).
    pub fn p(&self, right: f32, up: f32, toward: f32) -> V3 {
        [self.x + right * self.s, self.y - up * self.s, self.z + toward * self.s]
    }
    /// Meters to units here (for `Sdf` sizes and radii).
    pub fn m(&self, meters: f32) -> f32 {
        meters * self.s
    }
    /// Sizes in meters (width, height, depth) to units.
    pub fn size(&self, w: f32, h: f32, d: f32) -> V3 {
        [w * self.s, h * self.s, d * self.s]
    }
    /// A canvas-unit point of a body built here to the world.
    pub fn world(&self, p: V3) -> V3 {
        [self.at[0] + (p[0] - self.x) / self.s, self.at[1] - (p[1] - self.y) / self.s, self.at[2] - (p[2] - self.z) / self.s]
    }
    /// A world point to this spot's canvas-unit frame.
    pub fn local(&self, w: V3) -> V3 {
        [self.x + (w[0] - self.at[0]) * self.s, self.y - (w[1] - self.at[1]) * self.s, self.z - (w[2] - self.at[2]) * self.s]
    }
}

/// Which body (index into the world's bodies).
pub type BodyId = usize;

/// A solid standing in the world.
pub struct Body {
    pub sdf: Sdf,
    pub spot: Spot,
    /// Goes into the view's `Form` (false: a proxy that only casts shadows,
    /// shows in the water and grounds itself, like a figure written with
    /// gestures).
    pub visible: bool,
    /// Bounding sphere in the world (m).
    center: V3,
    radius: f32,
}

impl Body {
    /// Signed distance in meters from a world point.
    pub fn dist(&self, w: V3) -> f32 {
        self.sdf.eval(self.spot.local(w)).0 / self.spot.s
    }
    /// Unit normal (world) at a world point on the surface.
    pub fn normal(&self, w: V3) -> V3 {
        to_world(self.sdf.normal(self.spot.local(w), 0.08))
    }
    /// Where a ray (world, unit direction) enters and leaves the bounding
    /// sphere grown by `pad` m.
    fn span(&self, o: V3, d: V3, pad: f32) -> Option<(f32, f32)> {
        let q = [o[0] - self.center[0], o[1] - self.center[1], o[2] - self.center[2]];
        let r = self.radius + pad;
        let b = dot(q, d);
        let c = dot(q, q) - r * r;
        let disc = b * b - c;
        if disc < 0.0 {
            return None;
        }
        let s = disc.sqrt();
        let (t0, t1) = (-b - s, -b + s);
        if t1 < 0.0 { None } else { Some((t0.max(0.0), t1)) }
    }
}

// ------------------------------------------------------------------ world

/// Everything the scene shares: camera, ground, water, sun, sky, air and the
/// bodies standing in it.
pub struct World {
    /// The part of the canvas this world is seen in: [x, y, w, h] (units).
    /// Outside it there is nothing (other panels of a sheet).
    pub view: [f32; 4],
    /// Eye height above the ground datum (m).
    pub eye: f32,
    /// The horizon's canvas y (units): eye level.
    pub horizon: f32,
    /// Canvas x straight ahead (units).
    pub cx: f32,
    /// Focal length (units).
    pub focal: f32,
    /// Ground height (m) at (X, Z), or level.
    ground: Option<Box<dyn Fn(f32, f32) -> f32 + Sync + Send>>,
    pub water: Option<Water>,
    pub sun: Sun,
    /// Light from the open sky in full shadow (0..1 of the key light).
    pub sky: f32,
    /// Reflected light from the lit ground into shadows.
    pub bounce: f32,
    /// Penumbra: the sun's apparent size plus haze, as the shadow edge's
    /// width per meter from the caster (0.01 a clear sun, 0.05 hazy).
    pub penumbra: f32,
    /// Distance (m) at which aerial perspective has taken 63 % of a color.
    pub visibility: f32,
    /// Things beyond the modeled ground (a far shore, the sky) are taken to
    /// stand on a screen this far off (m) when the water mirrors them.
    pub backdrop: f32,
    /// How far the ground runs (m) before it is the horizon.
    pub far: f32,
    pub bodies: Vec<Body>,
    /// Motifs the painter paints by hand, registered at a depth so the
    /// world knows what is in front of what (see `Layer`).
    pub layers: Vec<Layer>,
}

impl World {
    /// A camera for a view `view` = [x, y, w, h] (canvas units) of the
    /// canvas, looking level from `eye` meters up, with the horizon at canvas
    /// y `horizon`, a 45° horizontal field of view (change with `fov`), a
    /// level ground, no water and the sun from the upper left behind the
    /// painter.
    pub fn new(view: [f32; 4], horizon: f32, eye: f32) -> Self {
        let mut w = World {
            view,
            eye,
            horizon,
            cx: view[0] + view[2] * 0.5,
            focal: 1.0,
            ground: None,
            water: None,
            sun: Sun::deg(-120.0, 35.0),
            sky: 0.2,
            bounce: 0.2,
            penumbra: 0.02,
            visibility: 2500.0,
            backdrop: 600.0,
            far: 20000.0,
            bodies: Vec::new(),
            layers: Vec::new(),
        };
        w.set_fov(view[2], 45.0);
        w
    }
    fn set_fov(&mut self, width: f32, deg: f32) {
        self.focal = width * 0.5 / (deg.to_radians() * 0.5).tan();
    }
    /// Horizontal field of view (degrees) over a view `width` units wide.
    pub fn fov(mut self, width: f32, deg: f32) -> Self {
        self.set_fov(width, deg);
        self
    }
    /// The ground's height (m) at (X, Z). Keep it gentle: the ground is
    /// assumed not to rise above eye level or hide itself.
    pub fn ground(mut self, h: impl Fn(f32, f32) -> f32 + Sync + Send + 'static) -> Self {
        self.ground = Some(Box::new(h));
        self
    }
    pub fn water(mut self, w: Water) -> Self {
        self.water = Some(w);
        self
    }
    /// The sun, and the sky light, reflected light and penumbra that go with
    /// it (change them after if you like).
    pub fn sun(mut self, sun: Sun) -> Self {
        self.sun = sun;
        let e = sun.elevation.to_degrees();
        // the higher the sun, the more its light dominates the sky's
        self.sky = if sun.up() { 0.28 - 0.1 * crate::smoothstep(5.0, 60.0, e) } else { 0.4 };
        self.bounce = if sun.up() { 0.22 } else { 0.08 };
        // low sun: a longer path through haze, softer edges
        self.penumbra = 0.012 + 0.03 * (1.0 - crate::smoothstep(3.0, 30.0, e));
        self
    }
    pub fn visibility(mut self, meters: f32) -> Self {
        self.visibility = meters;
        self
    }
    pub fn backdrop(mut self, meters: f32) -> Self {
        self.backdrop = meters;
        self
    }

    // ------------------------------------------------ camera and ground

    /// Ground height at (X, Z) (the datum, 0, when level).
    pub fn ground_at(&self, x: f32, z: f32) -> f32 {
        self.ground.as_ref().map_or(0.0, |g| g(x, z))
    }
    /// The visible surface: ground, or water where the ground lies below it.
    pub fn surface(&self, x: f32, z: f32) -> f32 {
        let g = self.ground_at(x, z);
        self.water.as_ref().map_or(g, |w| g.max(w.level))
    }
    /// True where water covers the ground.
    pub fn is_water(&self, x: f32, z: f32) -> bool {
        self.water.as_ref().is_some_and(|w| self.ground_at(x, z) < w.level)
    }
    /// Water depth (m) at (X, Z), 0 on land.
    pub fn water_depth(&self, x: f32, z: f32) -> f32 {
        self.water.as_ref().map_or(0.0, |w| (w.level - self.ground_at(x, z)).max(0.0))
    }
    /// Unit normal (world) of the ground (not the water) at (X, Z).
    pub fn ground_normal(&self, x: f32, z: f32) -> V3 {
        if self.ground.is_none() {
            return [0.0, 1.0, 0.0];
        }
        let e = 0.05;
        let dx = (self.ground_at(x + e, z) - self.ground_at(x - e, z)) / (2.0 * e);
        let dz = (self.ground_at(x, z + e) - self.ground_at(x, z - e)) / (2.0 * e);
        unit([-dx, 1.0, -dz])
    }

    /// Where a world point appears on the canvas (None behind the eye).
    pub fn project(&self, w: V3) -> Option<(f32, f32)> {
        if w[2] <= 1e-4 {
            return None;
        }
        Some((self.cx + self.focal * w[0] / w[2], self.horizon + self.focal * (self.eye - w[1]) / w[2]))
    }
    /// True if a canvas point lies in this world's view.
    pub fn sees(&self, x: f32, y: f32) -> bool {
        let v = self.view;
        x >= v[0] && y >= v[1] && x < v[0] + v[2] && y < v[1] + v[3]
    }
    /// Unit direction of the line of sight through a canvas point (world).
    pub fn ray(&self, x: f32, y: f32) -> V3 {
        unit([(x - self.cx) / self.focal, -(y - self.horizon) / self.focal, 1.0])
    }
    /// The eye.
    pub fn eye_pos(&self) -> V3 {
        [0.0, self.eye, 0.0]
    }
    /// Units per meter at depth `z` (m).
    pub fn scale_at(&self, z: f32) -> f32 {
        self.focal / z.max(1e-3)
    }
    /// The point of the ground (or water) seen at a canvas point, None where
    /// the line of sight goes to the sky.
    pub fn to_ground(&self, x: f32, y: f32) -> Option<V3> {
        if !self.sees(x, y) {
            return None;
        }
        let dy = y - self.horizon;
        let at = |z: f32| ((x - self.cx) * z / self.focal, self.eye - dy * z / self.focal);
        if self.ground.is_none() {
            let level = self.surface(0.0, 0.0);
            if dy <= 1e-5 || level >= self.eye {
                return None;
            }
            let z = self.focal * (self.eye - level) / dy;
            if z > self.far {
                return None;
            }
            let (wx, _) = at(z);
            return Some([wx, level, z]);
        }
        // march out along the line of sight, then bisect the crossing
        let above = |z: f32| {
            let (wx, wy) = at(z);
            wy - self.surface(wx, z)
        };
        let (near, n) = (0.2f32, 72);
        let k = (self.far / near).powf(1.0 / n as f32);
        let (mut a, mut z) = (near, near);
        if above(near) <= 0.0 {
            let (wx, _) = at(near);
            return Some([wx, self.surface(wx, near), near]);
        }
        for _ in 0..n {
            let b = z * k;
            if above(b) <= 0.0 {
                let mut hi = b;
                for _ in 0..22 {
                    let m = 0.5 * (a + hi);
                    if above(m) > 0.0 { a = m } else { hi = m }
                }
                let (wx, _) = at(hi);
                return Some([wx, self.surface(wx, hi), hi]);
            }
            a = b;
            z = b;
        }
        None
    }
    /// The spot on the ground seen at a canvas point.
    pub fn spot(&self, x: f32, y: f32) -> Option<Spot> {
        self.to_ground(x, y).map(|w| self.spot_world(w))
    }
    /// The spot on the ground at (X, Z) meters.
    pub fn spot_at(&self, x: f32, z: f32) -> Spot {
        self.spot_world([x, self.surface(x, z), z])
    }
    /// The spot at (X, Z) on the ground under the water (a pole's foot in a
    /// pond stands on the bottom).
    pub fn spot_bed(&self, x: f32, z: f32) -> Spot {
        self.spot_world([x, self.ground_at(x, z), z])
    }
    fn spot_world(&self, w: V3) -> Spot {
        let (x, y) = self.project(w).unwrap_or((self.cx, self.horizon));
        Spot { x, y, s: self.scale_at(w[2]), z: -w[2] * ZK, at: w }
    }
    /// How tall (units) something `meters` tall looks standing on the
    /// ground at a canvas point (0 above the horizon).
    pub fn height(&self, x: f32, y: f32, meters: f32) -> f32 {
        self.spot(x, y).map_or(0.0, |s| s.m(meters))
    }
    /// Aerial perspective: the fraction of a color lost to the air at
    /// distance `z` (m).
    pub fn aerial(&self, z: f32) -> f32 {
        crate::form::aerial(z, self.visibility)
    }
    /// The canvas angle (radians, y down) along which a shadow runs over the
    /// ground at a canvas point: away from the sun, foreshortened by the
    /// ground. Stroke shadows this way. None in the sky or with the sun down.
    pub fn shadow_angle(&self, x: f32, y: f32) -> Option<f32> {
        if !self.sun.up() {
            return None;
        }
        let g = self.to_ground(x, y)?;
        let d = self.sun.dir();
        let h = (d[0] * d[0] + d[2] * d[2]).sqrt().max(1e-4);
        let step = 0.05 * g[2].max(1.0);
        let (qx, qz) = (g[0] - d[0] / h * step, (g[2] - d[2] / h * step).max(0.3));
        let (a, b) = (self.project(g)?, self.project([qx, self.surface(qx, qz), qz])?);
        Some((b.1 - a.1).atan2(b.0 - a.0))
    }
    /// Where the sun (or the glow over it) is on the canvas, if ahead.
    pub fn sun_canvas(&self) -> Option<(f32, f32)> {
        let d = if self.sun.up() { self.sun.dir() } else { self.sun.glow_dir() };
        if d[2] <= 0.05 {
            return None;
        }
        Some((self.cx + self.focal * d[0] / d[2], self.horizon - self.focal * d[1] / d[2]))
    }

    // -------------------------------------------------------- bodies

    /// Place a body built at `spot` (its `Sdf` in canvas units from
    /// `spot.p`/`spot.m`). Visible bodies go into the view's `Form`.
    pub fn place(&mut self, spot: Spot, sdf: Sdf) -> BodyId {
        self.add_body(spot, sdf, true)
    }
    /// A proxy body: casts shadows, shows in the water and is grounded, but
    /// the painter paints it some other way (a figure written with gestures).
    pub fn proxy(&mut self, spot: Spot, sdf: Sdf) -> BodyId {
        self.add_body(spot, sdf, false)
    }
    fn add_body(&mut self, spot: Spot, sdf: Sdf, visible: bool) -> BodyId {
        let (lo, hi) = sdf.aabb();
        let (a, b) = (spot.world(lo), spot.world(hi));
        let center = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5];
        let radius = 0.5 * ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt() + 0.05;
        self.bodies.push(Body { sdf, spot, visible, center, radius });
        self.bodies.len() - 1
    }
    /// The nearest body surface from a world point: distance (m) and body.
    pub fn nearest(&self, w: V3) -> (f32, Option<BodyId>) {
        let mut best = (f32::INFINITY, None);
        for (i, b) in self.bodies.iter().enumerate() {
            let q = [w[0] - b.center[0], w[1] - b.center[1], w[2] - b.center[2]];
            if dot(q, q).sqrt() - b.radius > best.0 {
                continue;
            }
            let d = b.dist(w);
            if d < best.0 {
                best = (d, Some(i));
            }
        }
        best
    }

    // --------------------------------------------------------- light

    /// The `form::Light` every solid in this world is lit with. With the
    /// sun up it is the sun; with the sun down it is the glow low over
    /// where the sun is, and `cast` everywhere is at least `1 - glow` (see
    /// `View`), so planes turned toward the glow get a little light and
    /// there is no direct sun anywhere.
    pub fn light(&self) -> Light {
        let d = to_form(if self.sun.up() { self.sun.dir() } else { self.sun.glow_dir() });
        let mut l = Light::new((d[0], d[1]), d[2]).ambient(self.sky).penumbra(self.penumbra).across_parts(false);
        l.dir = d;
        // reflected light from the lit ground: from below, the side away from the sun
        l.bounce(self.bounce, [-d[0] * 0.6, 0.8, -d[2] * 0.6])
    }

    /// How deep a world point lies in cast shadow (0..1), `n` its normal
    /// (world). Traced toward the sun through every body; the penumbra
    /// grows with distance from the caster. With the sun down: `1 - glow`
    /// everywhere (no cast shadows, only the glow).
    pub fn cast(&self, w: V3, n: V3) -> f32 {
        if !self.sun.up() {
            return 1.0 - self.sun.glow();
        }
        let l = self.sun.dir();
        let o = add(w, n, 0.01);
        let k = self.penumbra.max(1e-3);
        let mut res = 1.0f32;
        for b in &self.bodies {
            let Some((t0, t1)) = b.span(o, l, 0.3) else { continue };
            let mut t = t0.max(0.02);
            let mut steps = 0;
            while t < t1 && steps < 160 {
                let d = b.dist(add(o, l, t));
                if d < 1e-3 {
                    return 1.0;
                }
                res = res.min(d / (k * t));
                if res < 0.01 {
                    return 1.0;
                }
                // the minimum step grows with distance, but never past the cap
                t += d.clamp((0.004 + 0.002 * t).min(0.5), 0.5);
                steps += 1;
            }
        }
        // the terrain itself (a bank shading the hollow behind it)
        if self.ground.is_some() && l[1] < 0.5 {
            let mut t = 0.3;
            while t < 60.0 {
                let p = add(o, l, t);
                let h = p[1] - self.ground_at(p[0], p[2]);
                if h < 0.0 {
                    return 1.0;
                }
                res = res.min(h / (k * t * 4.0));
                t *= 1.25;
            }
        }
        1.0 - crate::smoothstep(0.0, 1.0, res.clamp(0.0, 1.0))
    }

    /// Occlusion (0..1) at a world point with normal `n`: how much of the
    /// sky nearby solids (and, for a solid, the ground) hide within `reach`
    /// meters. It is the dark seam where a rock or a foot meets the ground
    /// and the darkening under an overhang.
    pub fn occlusion(&self, w: V3, n: V3, reach: f32, with_ground: bool) -> f32 {
        let mut occ = 0.0;
        let mut wt = 1.0;
        let mut norm = 0.0;
        for i in 1..=5 {
            let h = reach * i as f32 / 5.0;
            let q = add(w, n, h);
            let mut d = self.nearest(q).0;
            if with_ground {
                d = d.min(q[1] - self.surface(q[0], q[2]));
            }
            occ += wt * ((h - d) / h).clamp(0.0, 1.0);
            norm += wt;
            wt *= 0.7;
        }
        (occ / norm * 1.2).clamp(0.0, 1.0)
    }

    /// The first body a ray (world, unit direction) meets within `reach` m:
    /// the point and the body.
    pub fn trace(&self, o: V3, d: V3, reach: f32) -> Option<(V3, BodyId)> {
        let mut best: Option<(f32, BodyId)> = None;
        for (i, b) in self.bodies.iter().enumerate() {
            let Some((t0, t1)) = b.span(o, d, 0.0) else { continue };
            let t1 = t1.min(reach).min(best.map_or(f32::INFINITY, |bb| bb.0));
            let mut t = t0;
            let mut steps = 0;
            while t < t1 && steps < 200 {
                let dist = b.dist(add(o, d, t));
                if dist < 2e-3 {
                    best = Some((t, i));
                    break;
                }
                t += dist.max(0.002 + 0.001 * t);
                steps += 1;
            }
        }
        best.map(|(t, i)| (add(o, d, t), i))
    }

    // --------------------------------------------------- perspective

    /// A band lying on the ground along `pts` (X, Z in m, smoothed through
    /// them), `width(t)` m wide (t 0..1 along it): a path, a brook, a
    /// furrow. It narrows and foreshortens as it recedes by itself.
    pub fn ribbon(&self, pts: &[(f32, f32)], width: impl Fn(f32) -> f32) -> Shape {
        let c = self.resample(pts, 12);
        let n = c.len();
        if n < 2 {
            return Shape::new();
        }
        let (mut left, mut right) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for i in 0..n {
            let (a, b) = (c[i.saturating_sub(1)], c[(i + 1).min(n - 1)]);
            let (tx, tz) = (b.0 - a.0, b.1 - a.1);
            let l = (tx * tx + tz * tz).sqrt().max(1e-6);
            let (nx, nz) = (-tz / l, tx / l);
            let hw = 0.5 * width(i as f32 / (n - 1) as f32);
            for (side, out) in [(1.0, &mut left), (-1.0, &mut right)] {
                let (x, z) = (c[i].0 + side * nx * hw, (c[i].1 + side * nz * hw).max(0.3));
                if let Some(p) = self.project([x, self.surface(x, z), z]) {
                    out.push(p);
                }
            }
        }
        right.reverse();
        left.extend(right);
        Shape::new().poly(&left)
    }
    /// The canvas line of a path on the ground (X, Z in m), smoothed.
    pub fn line(&self, pts: &[(f32, f32)]) -> Vec<(f32, f32)> {
        self.resample(pts, 12).into_iter().filter_map(|(x, z)| self.project([x, self.surface(x, z), z])).collect()
    }
    /// `n` spots from `start` (X, Z) stepping by `step` (m) each time: fence
    /// posts, stones along a path, footprints; the spacing recedes by itself.
    pub fn recede(&self, start: (f32, f32), step: (f32, f32), n: usize) -> Vec<Spot> {
        (0..n).map(|i| self.spot_at(start.0 + step.0 * i as f32, start.1 + step.1 * i as f32)).collect()
    }
    /// Catmull-Rom through the points, `k` samples per segment.
    fn resample(&self, pts: &[(f32, f32)], k: usize) -> Vec<(f32, f32)> {
        let n = pts.len();
        if n < 3 {
            return pts.to_vec();
        }
        let mut out = Vec::with_capacity((n - 1) * k + 1);
        for i in 0..n - 1 {
            let p0 = pts[i.saturating_sub(1)];
            let (p1, p2) = (pts[i], pts[i + 1]);
            let p3 = pts[(i + 2).min(n - 1)];
            for j in 0..k {
                let t = j as f32 / k as f32;
                let (t2, t3) = (t * t, t * t * t);
                let f = |a: f32, b: f32, c: f32, d: f32| 0.5 * (2.0 * b + (-a + c) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2 + (-a + 3.0 * b - 3.0 * c + d) * t3);
                out.push((f(p0.0, p1.0, p2.0, p3.0), f(p0.1, p1.1, p2.1, p3.1)));
            }
        }
        out.push(pts[n - 1]);
        out
    }

    /// A `Form` of the given bodies alone (visible or proxies), lit by this
    /// world's sun with its cast shadows: the light side of a figure the
    /// painter writes with gestures, from the same sun as everything else.
    /// Part ids follow the order given (1, 2, …).
    pub fn form_of(&self, f: Frame, bodies: &[BodyId]) -> Form {
        let mut form = Form::new(f);
        let mut spots = Vec::new();
        for &b in bodies {
            form.add(&self.bodies[b].sdf, self.bodies[b].spot.at[2]);
            spots.push(self.bodies[b].spot);
        }
        form.light_given(self.light(), |x, y, s| {
            let w = spots[s.part as usize - 1].world([x, y, s.z]);
            self.cast(w, to_world(s.n))
        });
        form
    }

    /// See the world over a canvas frame (build it on `c.frame()`).
    pub fn view(&self, f: Frame) -> View<'_> {
        View::new(self, f)
    }
}

// ------------------------------------------------------------------- view

/// What is seen at a canvas point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum What {
    /// Outside the world's view (another panel).
    Off,
    Sky,
    Ground,
    Water,
    Body(BodyId),
}

/// Everything known at one canvas point.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub what: What,
    /// The world point (m).
    pub at: V3,
    /// Unit normal (form frame; for water, the rippled surface).
    pub n: V3,
    /// Distance in depth (m), for aerial perspective.
    pub dist: f32,
    /// How it is lit by the world's light (cast shadows from the world).
    pub shade: Shade,
}

/// The mirror image seen in the water at a canvas point.
#[derive(Clone, Copy, Debug)]
pub struct Mirror {
    /// The body seen in the water, if any (else sky or far shore).
    pub body: Option<BodyId>,
    /// Where the reflected thing is seen directly on the canvas: sample the
    /// painted canvas there, or evaluate a color field there.
    pub src: (f32, f32),
    /// The reflected point in the world and its normal (form frame) and
    /// light, for coloring a body's underside the way it is lit.
    pub at: V3,
    pub n: V3,
    pub shade: Shade,
    /// Share of the light the water reflects (Schlick, water): small looking
    /// down at the near water, large at a grazing angle toward the horizon.
    pub fresnel: f32,
    /// How far the reflected ray ran to what it shows (m); ripples blur a
    /// reflection more the farther this is.
    pub travel: f32,
}

/// Solids placed in a world, as the `Form` sees them: their sunk parts are
/// hidden by the ground in front of them.
struct Placed<'a> {
    world: &'a World,
    body: &'a Body,
    depth: &'a [f32],
    f: Frame,
}

impl Solid for Placed<'_> {
    fn bounds(&self) -> [f32; 4] {
        self.body.sdf.bounds()
    }
    fn hit(&self, x: f32, y: f32) -> Option<Hit> {
        if !self.world.sees(x, y) {
            return None;
        }
        let h = self.body.sdf.hit(x, y)?;
        let w = self.body.spot.world([x, y, h.z]);
        let zg = self.depth[self.f.index(x, y)];
        // behind the ground or water seen at this point, or below the ground
        // or water where it stands: sunk, hidden
        if w[2] > zg + 0.005 || w[1] < self.world.surface(w[0], w[2]) - 0.002 {
            return None;
        }
        Some(h)
    }
}

/// A world seen over a canvas frame: the `Form` of its visible bodies lit by
/// the world's one sun, the ground under them, and masks and fields for
/// painting it. Build on the whole frame (`c.frame()`).
pub struct View<'w> {
    pub world: &'w World,
    pub f: Frame,
    /// The visible bodies as a lit form (fields, masks, silhouettes).
    pub form: Form,
    /// Depth (m) of the ground or water per pixel (infinite: sky).
    depth: Vec<f32>,
    /// Cast shadow on the ground or water per pixel.
    ground_cast: Vec<f32>,
    /// Form part of each body (0: a proxy).
    parts: Vec<PartId>,
    /// What lies behind what at every pixel, built on first use.
    depths: std::sync::OnceLock<Depths>,
}

impl<'w> View<'w> {
    fn new(world: &'w World, f: Frame) -> Self {
        let inv = 1.0 / f.scale;
        let xy = |i: usize| (((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv);
        let depth: Vec<f32> = (0..f.w * f.h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = xy(i);
                world.to_ground(x, y).map_or(f32::INFINITY, |w| w[2])
            })
            .collect();
        let mut form = Form::new(f);
        let mut parts = vec![0; world.bodies.len()];
        // far to near is not needed (the depth buffer sorts by world depth:
        // each body's form z is in its own spot's projection), but the
        // bodies' ids follow the painter's order
        for (i, b) in world.bodies.iter().enumerate() {
            if b.visible {
                let p = Placed { world, body: b, depth: &depth, f };
                let spot = b.spot;
                parts[i] = form.add_nearest(&p, &move |x, y, z| spot.world([x, y, z])[2]);
            }
        }
        let body_of: Vec<BodyId> = {
            let mut v = vec![usize::MAX; parts.iter().copied().max().unwrap_or(0) as usize + 1];
            for (i, p) in parts.iter().enumerate() {
                if *p != 0 {
                    v[*p as usize] = i;
                }
            }
            v
        };
        let light = world.light();
        form.light_given(light, |x, y, s| {
            let b = &world.bodies[body_of[s.part as usize]];
            let w = b.spot.world([x, y, s.z]);
            world.cast(w, to_world(s.n))
        });
        let ground_cast: Vec<f32> = (0..f.w * f.h)
            .into_par_iter()
            .map(|i| {
                if !depth[i].is_finite() {
                    return 0.0;
                }
                let (x, y) = xy(i);
                let w = world.to_ground(x, y).unwrap_or([0.0, 0.0, depth[i]]);
                world.cast(w, world.ground_normal(w[0], w[2]))
            })
            .collect();
        View { world, f, form, depth, ground_cast, parts, depths: std::sync::OnceLock::new() }
    }

    /// The form part of a body (0 for a proxy).
    pub fn part(&self, b: BodyId) -> PartId {
        self.parts[b]
    }
    fn body_at(&self, part: PartId) -> Option<BodyId> {
        self.parts.iter().position(|p| *p == part && part != 0)
    }

    /// What is seen at a canvas point.
    pub fn at(&self, x: f32, y: f32) -> Point {
        let w = self.world;
        let light = w.light();
        if let Some((s, b)) = self.form.sample(x, y).and_then(|s| self.body_at(s.part).map(|b| (s, b))) {
            let at = w.bodies[b].spot.world([x, y, s.z]);
            return Point { what: What::Body(b), at, n: s.n, dist: at[2], shade: s.shade };
        }
        if !w.sees(x, y) {
            return Point { what: What::Off, at: [0.0; 3], n: [0.0, 0.0, 1.0], dist: f32::INFINITY, shade: Shade::default() };
        }
        let i = self.f.index(x, y);
        let z = self.depth[i];
        if !z.is_finite() {
            let d = w.ray(x, y);
            return Point { what: What::Sky, at: add(w.eye_pos(), d, w.far), n: [0.0, 0.0, 1.0], dist: f32::INFINITY, shade: Shade::default() };
        }
        let at = w.to_ground(x, y).unwrap_or([0.0, 0.0, z]);
        let water = w.is_water(at[0], at[2]);
        let nw = if water { w.water.as_ref().unwrap().normal(at[0], at[2]) } else { w.ground_normal(at[0], at[2]) };
        let n = to_form(nw);
        Point { what: if water { What::Water } else { What::Ground }, at, n, dist: z, shade: light.shade(n, self.ground_cast[i]) }
    }

    /// Cast shadow (0..1) at a canvas point: on the ground, the water or a
    /// visible body.
    pub fn cast(&self, x: f32, y: f32) -> f32 {
        match self.form.sample(x, y) {
            Some(s) => s.shade.cast,
            None => self.ground_cast[self.f.index(x, y)],
        }
    }

    /// A mask from any rule over what is seen: `view.mask(|p| ...)`.
    pub fn mask(&self, g: impl Fn(&Point) -> f32 + Sync) -> Mask {
        let f = self.f;
        let inv = 1.0 / f.scale;
        let data = (0..f.w * f.h).into_par_iter().map(|i| g(&self.at(((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv))).collect();
        Mask { f, data }
    }
    /// Where the sky is seen.
    pub fn sky(&self) -> Mask {
        self.mask(|p| (p.what == What::Sky) as u8 as f32)
    }
    /// Where land is seen (not covered by a visible body).
    pub fn land(&self) -> Mask {
        self.mask(|p| (p.what == What::Ground) as u8 as f32)
    }
    /// Where water is seen.
    pub fn water(&self) -> Mask {
        self.mask(|p| (p.what == What::Water) as u8 as f32)
    }

    /// Cast shadows of every body on the ground and water (not on the
    /// bodies: their own shading has them): 1 in full shadow, soft at the
    /// penumbra. With the sun down: empty.
    pub fn shadows(&self) -> Mask {
        let up = self.world.sun.up();
        self.mask(|p| if up && matches!(p.what, What::Ground | What::Water) { p.shade.cast } else { 0.0 })
    }

    /// Contact occlusion: the dark seam where bodies (visible or proxy) meet
    /// the ground, and the ground darkened close around them, within
    /// `reach` m (0.15 for a stone on sand, 0.05 for a foot). 0..1.
    pub fn contact(&self, reach: f32) -> Mask {
        let w = self.world;
        self.mask(|p| match p.what {
            What::Ground | What::Water => w.occlusion(p.at, to_world(p.n_ground(w)), reach, false),
            What::Body(_) => {
                // only near the ground: the seam, not every hollow of the stone
                let hgt = p.at[1] - w.surface(p.at[0], p.at[2]);
                if hgt > reach * 3.0 { 0.0 } else { w.occlusion(p.at, to_world(p.n), reach, true) }
            }
            What::Sky | What::Off => 0.0,
        })
    }

    /// The mirror image in the water at a canvas point (None off the water).
    pub fn mirror(&self, x: f32, y: f32) -> Option<Mirror> {
        let w = self.world;
        let water = w.water.as_ref()?;
        let i = self.f.index(x, y);
        if !self.depth[i].is_finite() || self.form.sample(x, y).is_some() {
            return None;
        }
        let p = w.to_ground(x, y)?;
        if !w.is_water(p[0], p[2]) {
            return None;
        }
        let v = unit([p[0], p[1] - w.eye, p[2]]);
        let n = water.normal(p[0], p[2]);
        let vn = dot(v, n);
        let mut r = add(v, n, -2.0 * vn);
        r[1] = r[1].max(0.002);
        let r = unit(r);
        let cos = (-vn).clamp(0.0, 1.0);
        let fresnel = 0.02 + 0.98 * (1.0 - cos).powi(5);
        if let Some((hit, b)) = w.trace(add(p, r, 0.01), r, w.backdrop) {
            let body = &w.bodies[b];
            let nb = body.normal(hit);
            let nf = to_form(nb);
            let shade = w.light().shade(nf, w.cast(hit, nb));
            let src = w.project(hit).unwrap_or((x, y));
            let travel = ((hit[0] - p[0]).powi(2) + (hit[1] - p[1]).powi(2) + (hit[2] - p[2]).powi(2)).sqrt();
            return Some(Mirror { body: Some(b), src, at: hit, n: nf, shade, fresnel, travel });
        }
        // the far shore and the sky, taken to stand on the backdrop
        // (only ahead of the ray: water beyond the backdrop sees the sky)
        let (src, travel, at) = if r[2] > 1e-4 && w.backdrop > p[2] {
            let t = (w.backdrop - p[2]) / r[2];
            let q = add(p, r, t);
            (w.project(q).unwrap_or((x, y)), t, q)
        } else {
            ((x, 2.0 * w.horizon - y), w.far, add(p, r, w.far))
        };
        Some(Mirror { body: None, src, at, n: [0.0, 0.0, 1.0], shade: Shade::default(), fresnel, travel })
    }

    /// Where bodies are seen mirrored in the water (`bodies` empty: all of
    /// them), 0..1 by the water's reflectance relative to its maximum.
    pub fn reflections(&self, bodies: &[BodyId]) -> Mask {
        self.mask(|p| {
            if p.what != What::Water {
                return 0.0;
            }
            match self.mirror_at(p) {
                Some(Mirror { body: Some(b), .. }) if bodies.is_empty() || bodies.contains(&b) => 1.0,
                _ => 0.0,
            }
        })
    }
    fn mirror_at(&self, p: &Point) -> Option<Mirror> {
        let (x, y) = self.world.project(p.at)?;
        self.mirror(x, y)
    }

    /// Depth (m) of the ground or water seen at a canvas point (infinite
    /// in the sky).
    pub fn ground_depth(&self, x: f32, y: f32) -> f32 {
        self.depth[self.f.index(x, y)]
    }
}

impl Point {
    /// The ground's normal (world) under a ground or water point.
    fn n_ground(&self, w: &World) -> V3 {
        to_form(w.ground_normal(self.at[0], self.at[2]))
    }
    /// The form sample for a body point.
    pub fn sample(&self, part: PartId) -> Sample {
        Sample { part, facet: 0, z: 0.0, n: self.n, dist: self.dist, shade: self.shade }
    }
}

// ------------------------------------------------------------------ depth

/// How deep a painter's layer lies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayerDepth {
    /// At a fixed distance (m): a figure at its spot's depth (`spot.at[2]`).
    At(f32),
    /// On the ground or the water, at whatever depth that is seen at each
    /// pixel: a path, a patch of heather, a glint on the sea.
    Ground,
}

/// A motif the painter paints by hand (a figure written with gestures, a
/// drawn tree, a boat), registered as a canvas mask at a depth, so the
/// world knows what it hides and what hides it.
#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub mask: Mask,
    pub depth: LayerDepth,
}

/// One thing seen along a line of sight.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Thing {
    Sky,
    Ground,
    Water,
    Body(BodyId),
    Layer(usize),
}

impl World {
    /// Register a motif painted by hand at a depth (see `Layer`). Returns
    /// its index; masks name it as `Thing::Layer(index)`.
    pub fn layer(&mut self, name: &str, mask: Mask, depth: LayerDepth) -> usize {
        self.layers.push(Layer { name: name.to_string(), mask, depth });
        self.layers.len() - 1
    }
    /// The layer called `name` (the last one, if several share it).
    pub fn layer_named(&self, name: &str) -> Option<usize> {
        self.layers.iter().rposition(|l| l.name == name)
    }

    /// `cast` with a penumbra of your choosing (the shadow edge's width per
    /// meter from the caster) and only the bodies `casters` picks. The
    /// terrain shades too if `terrain`. 0 lit, 1 in full shadow.
    pub fn cast_soft(&self, w: V3, n: V3, penumbra: f32, casters: &(dyn Fn(BodyId) -> bool + Sync), terrain: bool) -> f32 {
        if !self.sun.up() {
            return 0.0;
        }
        let l = self.sun.dir();
        let o = add(w, n, 0.01);
        let k = penumbra.max(1e-3);
        let mut res = 1.0f32;
        for (i, b) in self.bodies.iter().enumerate() {
            if !casters(i) {
                continue;
            }
            // the soft shadow reaches past the body by the penumbra's width
            let Some((t0, t1)) = b.span(o, l, 0.3 + k * 8.0) else { continue };
            let mut t = t0.max(0.02);
            let mut steps = 0;
            while t < t1 && steps < 200 {
                let d = b.dist(add(o, l, t));
                if d < 1e-3 {
                    return 1.0;
                }
                res = res.min(d / (k * t));
                if res < 0.005 {
                    return 1.0;
                }
                t += d.clamp((0.004 + 0.002 * t).min(0.5), 0.5);
                steps += 1;
            }
        }
        if terrain && self.ground.is_some() && l[1] < 0.5 {
            let mut t = 0.3;
            while t < 60.0 {
                let p = add(o, l, t);
                let h = p[1] - self.ground_at(p[0], p[2]);
                if h < 0.0 {
                    return 1.0;
                }
                res = res.min(h / (k * t * 4.0));
                t *= 1.25;
            }
        }
        1.0 - crate::smoothstep(0.0, 1.0, res.clamp(0.0, 1.0))
    }

    /// How much of the sky (0..1, cosine-weighted) the bodies `by` (and the
    /// ground, if `with_ground`) hide from a world point with normal `n`
    /// (world), counting only what lies within `reach` m. Occluders fade out
    /// toward `reach`, so the darkening falls off smoothly with distance:
    /// about 0.5 in the crease where a stone meets flat ground, nothing a
    /// reach away. This is the contact shadow and the dark under an overhang.
    pub fn sky_occlusion(&self, w: V3, n: V3, reach: f32, by: &(dyn Fn(BodyId) -> bool + Sync), with_ground: bool) -> f32 {
        let reach = reach.max(1e-3);
        let near: Vec<&Body> = self
            .bodies
            .iter()
            .enumerate()
            .filter(|(i, b)| {
                by(*i) && {
                    let q = [w[0] - b.center[0], w[1] - b.center[1], w[2] - b.center[2]];
                    dot(q, q).sqrt() < b.radius + reach
                }
            })
            .map(|(_, b)| b)
            .collect();
        if near.is_empty() && !with_ground {
            return 0.0;
        }
        // a frame around the normal
        let t1 = unit(if n[1].abs() < 0.9 { [n[2], 0.0, -n[0]] } else { [0.0, -n[2], n[1]] });
        let t2 = [n[1] * t1[2] - n[2] * t1[1], n[2] * t1[0] - n[0] * t1[2], n[0] * t1[1] - n[1] * t1[0]];
        let o = add(w, n, 0.01 * reach + 0.005);
        const DIRS: usize = 16;
        let mut seen = 0.0;
        for k in 0..DIRS {
            // cosine-weighted directions on the hemisphere (a Fibonacci spiral)
            let u = (k as f32 + 0.5) / DIRS as f32;
            let (r, up) = (u.sqrt(), (1.0 - u).sqrt());
            let (s, c) = (k as f32 * 2.399_963).sin_cos();
            let d = unit([t1[0] * r * c + t2[0] * r * s + n[0] * up, t1[1] * r * c + t2[1] * r * s + n[1] * up, t1[2] * r * c + t2[2] * r * s + n[2] * up]);
            let mut vis = 1.0f32;
            let mut t = 0.04 * reach;
            while t < reach {
                let q = add(o, d, t);
                let mut dist = near.iter().map(|b| b.dist(q)).fold(f32::INFINITY, f32::min);
                if with_ground {
                    dist = dist.min(q[1] - self.surface(q[0], q[2]));
                }
                // a cone of half-angle ~27°; what lies near the reach counts less
                let fade = crate::smoothstep(0.45 * reach, reach, t);
                vis = vis.min((dist / (0.5 * t)).max(fade).clamp(0.0, 1.0));
                if vis <= 0.0 {
                    break;
                }
                t += dist.clamp(0.03 * reach, 0.25 * reach);
            }
            seen += vis;
        }
        1.0 - seen / DIRS as f32
    }
}

/// Coverage and depth of one body over the pixels it may cover.
struct Patch {
    x0: usize,
    y0: usize,
    w: usize,
    h: usize,
    cov: Vec<f32>,
    dep: Vec<f32>,
}

/// What lies behind what at every pixel: the ground or water (or sky),
/// every visible body (not proxies: they are never seen) and every layer, each with its coverage
/// (soft at its edges) and its distance (m). Masks from it are
/// front-to-back composites, so the soft edge of a figure over the sea
/// hides the sea exactly as much as it covers it, and nothing is ever
/// subtracted twice. Build with `View::depths`.
pub struct Depths {
    pub f: Frame,
    /// Depth of the ground or water seen (infinite: sky).
    surface: Vec<f32>,
    water: Vec<bool>,
    bodies: Vec<Option<Patch>>,
    layers: Vec<(Vec<f32>, LayerDepth)>,
}

/// One entry of a pixel's stack: depth (m), coverage, thing.
pub type Seen = (f32, f32, Thing);

impl Depths {
    fn new(view: &View) -> Self {
        let world = view.world;
        let f = view.f;
        let inv = 1.0 / f.scale;
        let surface = view.depth.clone();
        let water: Vec<bool> = (0..f.w * f.h)
            .into_par_iter()
            .map(|i| {
                if !surface[i].is_finite() || world.water.is_none() {
                    return false;
                }
                let (x, y) = (((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv);
                world.to_ground(x, y).is_some_and(|p| world.is_water(p[0], p[2]))
            })
            .collect();
        // proxies are stand-ins (their shadow and reflection): what is seen of a
        // figure written with gestures is the layer its outline is registered as
        let bodies = world.bodies.iter().map(|b| if b.visible { Self::patch(world, b, f) } else { None }).collect();
        let layers = world
            .layers
            .iter()
            .map(|l| {
                let cov = if l.mask.f.w == f.w && l.mask.f.h == f.h {
                    l.mask.data.clone()
                } else {
                    (0..f.w * f.h).map(|i| l.mask.sample(((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv)).collect()
                };
                (cov, l.depth)
            })
            .collect();
        Depths { f, surface, water, bodies, layers }
    }

    /// A body's coverage and depth: one ray per pixel, four at its edges.
    fn patch(world: &World, b: &Body, f: Frame) -> Option<Patch> {
        let [bx0, by0, bx1, by1] = b.sdf.bounds();
        let v = world.view;
        let (ux0, uy0, ux1, uy1) = (bx0.max(v[0]), by0.max(v[1]), bx1.min(v[0] + v[2]), by1.min(v[1] + v[3]));
        let px = |u: f32, n: usize| ((u * f.scale).max(0.0) as usize).min(n);
        let (x0, y0) = (px(ux0.floor(), f.w).saturating_sub(1), px(uy0.floor(), f.h).saturating_sub(1));
        let (x1, y1) = ((px(ux1, f.w) + 2).min(f.w), (px(uy1, f.h) + 2).min(f.h));
        if x1 <= x0 || y1 <= y0 {
            return None;
        }
        let (w, h) = (x1 - x0, y1 - y0);
        let inv = 1.0 / f.scale;
        let probe = |x: f32, y: f32| -> Option<f32> {
            let hit = b.sdf.hit(x, y)?;
            let p = b.spot.world([x, y, hit.z]);
            // sunk below the ground or water where it stands: hidden
            if p[1] < world.surface(p[0], p[2]) - 0.002 {
                return None;
            }
            Some(p[2])
        };
        let center: Vec<Option<f32>> = (0..w * h).into_par_iter().map(|k| probe(((x0 + k % w) as f32 + 0.5) * inv, ((y0 + k / w) as f32 + 0.5) * inv)).collect();
        let (cov, dep): (Vec<f32>, Vec<f32>) = (0..w * h)
            .into_par_iter()
            .map(|k| {
                let (i, j) = (k % w, k / w);
                let c = center[k];
                let at = |a: isize, b: isize| -> bool {
                    let (ii, jj) = (i as isize + a, j as isize + b);
                    ii >= 0 && jj >= 0 && (ii as usize) < w && (jj as usize) < h && center[jj as usize * w + ii as usize].is_some()
                };
                let me = c.is_some();
                if at(-1, 0) == me && at(1, 0) == me && at(0, -1) == me && at(0, 1) == me {
                    return c.map_or((0.0, f32::INFINITY), |d| (1.0, d));
                }
                let (mut n, mut s) = (0, 0.0);
                for (ox, oy) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                    if let Some(d) = probe(((x0 + i) as f32 + ox) * inv, ((y0 + j) as f32 + oy) * inv) {
                        n += 1;
                        s += d;
                    }
                }
                if n == 0 { (0.0, f32::INFINITY) } else { (n as f32 / 4.0, s / n as f32) }
            })
            .unzip();
        Some(Patch { x0, y0, w, h, cov, dep })
    }

    /// The things seen at pixel `i`, nearest first: (depth m, coverage, thing).
    pub fn stack(&self, i: usize, out: &mut Vec<Seen>) {
        out.clear();
        let (px, py) = (i % self.f.w, i / self.f.w);
        for (b, p) in self.bodies.iter().enumerate() {
            let Some(p) = p else { continue };
            if px >= p.x0 && py >= p.y0 && px < p.x0 + p.w && py < p.y0 + p.h {
                let k = (py - p.y0) * p.w + (px - p.x0);
                if p.cov[k] > 1e-4 {
                    out.push((p.dep[k], p.cov[k], Thing::Body(b)));
                }
            }
        }
        let s = self.surface[i];
        for (l, (cov, d)) in self.layers.iter().enumerate() {
            let c = cov[i];
            if c > 1e-4 {
                let z = match d {
                    LayerDepth::At(z) => *z,
                    // just in front of the ground it lies on
                    LayerDepth::Ground => {
                        if s.is_finite() { s - 0.01 } else { f32::MAX }
                    }
                };
                out.push((z, c.min(1.0), Thing::Layer(l)));
            }
        }
        let what = if !s.is_finite() {
            Thing::Sky
        } else if self.water[i] {
            Thing::Water
        } else {
            Thing::Ground
        };
        out.push((s, 1.0, what));
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    /// The stack at a canvas point.
    pub fn at(&self, x: f32, y: f32) -> Vec<Seen> {
        let mut v = Vec::new();
        self.stack(self.pixel(x, y), &mut v);
        v
    }
    fn pixel(&self, x: f32, y: f32) -> usize {
        let px = ((x * self.f.scale) as isize).clamp(0, self.f.w as isize - 1) as usize;
        let py = ((y * self.f.scale) as isize).clamp(0, self.f.h as isize - 1) as usize;
        py * self.f.w + px
    }
    /// How much of each thing is seen at a canvas point, nearest first:
    /// (thing, depth m, share of the pixel 0..1).
    pub fn seen_at(&self, x: f32, y: f32) -> Vec<(Thing, f32, f32)> {
        let mut t = 1.0;
        let mut out = Vec::new();
        for (d, c, th) in self.at(x, y) {
            if t <= 1e-5 {
                break;
            }
            out.push((th, d, c * t));
            t *= 1.0 - c;
        }
        out
    }
    /// A mask from any rule over each pixel's stack (nearest first).
    pub fn map(&self, g: impl Fn(&[Seen]) -> f32 + Sync) -> Mask {
        let f = self.f;
        let data = (0..f.w * f.h)
            .into_par_iter()
            .map_init(Vec::new, |buf, i| {
                self.stack(i, buf);
                g(buf).clamp(0.0, 1.0)
            })
            .collect();
        Mask { f, data }
    }
    /// Where the things `sel` picks are seen: their coverage less whatever
    /// lies in front of them. `visible(|t| t == Thing::Ground)` is the
    /// ground not covered by any body or layer.
    pub fn visible(&self, sel: &(dyn Fn(Thing) -> bool + Sync)) -> Mask {
        self.map(|s| {
            let (mut t, mut v) = (1.0, 0.0);
            for &(_, c, th) in s {
                if sel(th) {
                    v += c * t;
                }
                t *= 1.0 - c;
                if t < 1e-6 {
                    break;
                }
            }
            v
        })
    }
    /// Everything in front of what `sel` picks, where it is: the parts of
    /// it that are hidden, and by how much.
    pub fn front(&self, sel: &(dyn Fn(Thing) -> bool + Sync)) -> Mask {
        self.map(|s| {
            let Some(d) = s.iter().filter(|e| sel(e.2)).map(|e| e.0).reduce(f32::min) else { return 0.0 };
            let mut t = 1.0;
            for &(dk, c, th) in s {
                if dk >= d {
                    break;
                }
                if !sel(th) {
                    t *= 1.0 - c;
                }
            }
            1.0 - t
        })
    }
    /// Where a pass that lies just behind what `sel` picks would show: not
    /// where it is, and not where anything in front of it is. 1 where it
    /// isn't at all.
    pub fn behind(&self, sel: &(dyn Fn(Thing) -> bool + Sync)) -> Mask {
        self.map(|s| {
            let Some(d) = s.iter().filter(|e| sel(e.2)).map(|e| e.0).reduce(f32::max) else { return 1.0 };
            let mut t = 1.0;
            for &(dk, c, _) in s {
                if dk > d {
                    break;
                }
                t *= 1.0 - c;
            }
            t
        })
    }
    /// Where a pass lying `z` m away would show: everything nearer (ground
    /// included) hides it.
    pub fn at_depth(&self, z: f32) -> Mask {
        self.map(|s| {
            let mut t = 1.0;
            for &(dk, c, _) in s {
                if dk >= z {
                    break;
                }
                t *= 1.0 - c;
            }
            t
        })
    }
    /// Whatever is seen between `near` and `far` m.
    pub fn between(&self, near: f32, far: f32) -> Mask {
        self.map(|s| {
            let (mut t, mut v) = (1.0, 0.0);
            for &(dk, c, _) in s {
                if dk >= near && dk <= far {
                    v += c * t;
                }
                t *= 1.0 - c;
            }
            v
        })
    }
}

impl View<'_> {
    /// What lies behind what at every pixel (traced on first use, ~0.1–0.5
    /// s at 1000px; 12 bytes per pixel plus the bodies' patches).
    pub fn depths(&self) -> &Depths {
        self.depths.get_or_init(|| Depths::new(self))
    }

    /// Cast shadows on the ground and water where they are seen, softer
    /// than the world's own by `soft` (1: the sun's penumbra and the haze;
    /// 2: twice as wide), from the bodies `casters` picks. The penumbra grows
    /// with distance from the caster, so a shadow is crisp at a stone's foot
    /// and soft at its far end, as in nature. Figures and motifs registered
    /// as layers hide it where they stand in front.
    pub fn soft_shadows(&self, soft: f32, casters: &(dyn Fn(BodyId) -> bool + Sync)) -> Mask {
        let w = self.world;
        if !w.sun.up() {
            return Mask::empty(self.f);
        }
        let k = w.penumbra * soft.max(0.05);
        let d = self.depths();
        let f = self.f;
        let inv = 1.0 / f.scale;
        let data = (0..f.w * f.h)
            .into_par_iter()
            .map_init(Vec::new, |buf, i| {
                if !self.depth[i].is_finite() {
                    return 0.0;
                }
                d.stack(i, buf);
                let mut t = 1.0;
                let mut seen = 0.0;
                for &(_, c, th) in buf.iter() {
                    if matches!(th, Thing::Ground | Thing::Water) {
                        seen = t;
                        break;
                    }
                    t *= 1.0 - c;
                }
                if seen <= 1e-4 {
                    return 0.0;
                }
                let (x, y) = (((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv);
                let Some(p) = w.to_ground(x, y) else { return 0.0 };
                seen * w.cast_soft(p, w.ground_normal(p[0], p[2]), k, casters, true)
            })
            .collect();
        Mask { f, data }
    }

    /// The contact shadow: the sky the bodies `by` hide from the ground and
    /// water within `reach` m, where the ground is seen (see
    /// `World::sky_occlusion`), and on the visible bodies the sky hidden by
    /// the ground and the other bodies, near the ground. It is darkest in
    /// the crease and falls off smoothly with distance, with no edge.
    pub fn occlusion(&self, reach: f32, by: &(dyn Fn(BodyId) -> bool + Sync)) -> Mask {
        let w = self.world;
        let d = self.depths();
        let f = self.f;
        let inv = 1.0 / f.scale;
        let data = (0..f.w * f.h)
            .into_par_iter()
            .map_init(Vec::new, |buf, i| {
                let (x, y) = (((i % f.w) as f32 + 0.5) * inv, ((i / f.w) as f32 + 0.5) * inv);
                d.stack(i, buf);
                let mut t = 1.0;
                let mut v = 0.0;
                for &(_, c, th) in buf.iter() {
                    let share = c * t;
                    if share > 1e-4 {
                        match th {
                            Thing::Ground | Thing::Water => {
                                if let Some(p) = w.to_ground(x, y) {
                                    v += share * w.sky_occlusion(p, w.ground_normal(p[0], p[2]), reach, by, false);
                                }
                            }
                            Thing::Body(b) if self.parts[b] != 0 => {
                                if let Some(s) = self.form.sample(x, y).filter(|s| s.part == self.parts[b]) {
                                    let p = w.bodies[b].spot.world([x, y, s.z]);
                                    if p[1] - w.surface(p[0], p[2]) < reach {
                                        v += share * w.sky_occlusion(p, to_world(s.n), reach, &|o| o != b && by(o), true);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    t *= 1.0 - c;
                    if t < 1e-4 {
                        break;
                    }
                }
                v
            })
            .collect();
        Mask { f, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> World {
        World::new([0.0, 0.0, 1000.0, 700.0], 300.0, 1.6)
    }

    fn pole(w: &mut World, x: f32, z: f32) -> BodyId {
        let s = w.spot_at(x, z);
        w.place(s, Sdf::block(s.p(0.0, 1.5, 0.0), s.size(0.12, 3.0, 0.12), s.m(0.01)))
    }

    #[test]
    fn projection_round_trips_and_figures_scale_with_depth() {
        let w = world();
        let p = [2.0, 0.0, 12.0];
        let (x, y) = w.project(p).unwrap();
        let g = w.to_ground(x, y).unwrap();
        assert!((g[0] - 2.0).abs() < 1e-3 && (g[2] - 12.0).abs() < 1e-2, "{g:?}");
        // a figure as tall as the eye has its head on the horizon
        let h = w.height(x, y, 1.6);
        assert!((y - h - w.horizon).abs() < 0.05, "{} vs {}", y - h, w.horizon);
        // twice as far, half as tall
        let (x2, y2) = w.project([2.0, 0.0, 24.0]).unwrap();
        assert!((w.height(x2, y2, 1.7) * 2.0 - w.height(x, y, 1.7)).abs() < 0.05);
        assert!(w.to_ground(500.0, 250.0).is_none());
        // with an uneven ground the march finds it too
        let w = world().ground(|x, z| 0.2 * (x * 0.3).sin() + 0.01 * z);
        let (x, y) = w.project([1.0, w.ground_at(1.0, 10.0), 10.0]).unwrap();
        let g = w.to_ground(x, y).unwrap();
        assert!((g[2] - 10.0).abs() < 0.05, "{g:?}");
    }

    #[test]
    fn one_sun_casts_every_shadow_the_same_way() {
        // sun from the left, low: shadows fall to the right
        let mut w = world().sun(Sun::deg(-80.0, 20.0));
        pole(&mut w, -1.0, 10.0);
        pole(&mut w, 2.0, 14.0);
        // the shadow runs away from the sun: right, and a little toward us
        let d = w.sun.dir();
        let back = -d[2] / -d[0];
        for (x, z) in [(-1.0f32, 10.0f32), (2.0, 14.0)] {
            let right = w.cast([x + 2.0, 0.0, z + 2.0 * back], [0.0, 1.0, 0.0]);
            let left = w.cast([x - 2.0, 0.0, z - 2.0 * back], [0.0, 1.0, 0.0]);
            assert!(right > 0.5 && left < 0.05, "right {right} left {left}");
        }
        // penumbra grows with distance from the caster: the edge is wider far out
        let edge = |d: f32| {
            let v: Vec<f32> = (0..200).map(|k| w.cast([-1.0 + d, 0.0, 10.0 + d * back + (k as f32 - 100.0) * 0.004], [0.0, 1.0, 0.0])).collect();
            v.iter().filter(|c| **c > 0.05 && **c < 0.95).count()
        };
        assert!(edge(6.0) > edge(1.0), "{} vs {}", edge(6.0), edge(1.0));
        // below the horizon: no shadows at all, the same everywhere
        let w2 = world().sun(Sun::deg(-80.0, -4.0));
        assert_eq!(w2.cast([1.0, 0.0, 10.0], [0.0, 1.0, 0.0]), w2.cast([-1.0, 0.0, 10.0], [0.0, 1.0, 0.0]));
        assert!(w2.cast([0.0, 0.0, 10.0], [0.0, 1.0, 0.0]) > 0.5);
    }

    #[test]
    fn a_view_agrees_on_the_light_and_grounds_its_bodies() {
        let mut w = world().sun(Sun::deg(-70.0, 25.0));
        let s = w.spot_at(0.0, 8.0);
        let rock = w.place(s, Sdf::ellipsoid(s.p(0.0, 0.3, 0.0), s.size(1.2, 0.9, 1.0)));
        let v = w.view(Frame::new(500, 350, 0.5));
        let part = v.part(rock);
        assert!(part > 0);
        // the left of the rock is lit, the right in shadow, and on the
        // ground the shadow lies right of it, not left
        let (x, y) = (s.x, s.y - s.m(0.5));
        let left = v.form.sample(x - s.m(0.5), y).unwrap();
        let right = v.form.sample(x + s.m(0.5), y).unwrap();
        assert!(left.shade.direct > right.shade.direct);
        let shadows = v.shadows();
        let r = shadows.sample(s.x + s.m(1.6), s.y - 1.0);
        let l = shadows.sample(s.x - s.m(1.6), s.y - 1.0);
        assert!(r > 0.5 && l < 0.05, "r {r} l {l}");
        // the sunk part is hidden: nothing of the rock below its foot line
        assert!(v.form.sample(s.x, s.y + s.m(0.35)).is_none());
        // the contact seam: darker right at the foot than a meter off
        let c = v.contact(0.2);
        let near = c.sample(s.x - s.m(0.3), s.y + s.m(0.1));
        let far = c.sample(s.x - s.m(2.5), s.y + s.m(0.1));
        assert!(near > far + 0.05, "near {near} far {far}");
    }

    #[test]
    fn still_water_mirrors_a_pole_straight_down() {
        let mut w = world().water(Water::new(0.0)).ground(|_, z| if z > 9.0 { -1.0 } else { 0.1 });
        let s = w.spot_bed(0.0, 12.0);
        let id = w.place(s, Sdf::block(s.p(0.0, 1.5, 0.0), s.size(0.15, 3.0, 0.15), 0.0));
        let v = w.view(Frame::new(500, 350, 0.5));
        let foot = w.project([0.0, 0.0, 12.0]).unwrap();
        // below the waterline, straight down, the pole is seen in the water
        let m = v.mirror(foot.0, foot.1 + 10.0).unwrap();
        assert_eq!(m.body, Some(id));
        assert!((m.src.0 - foot.0).abs() < 1.0 && m.src.1 < foot.1, "{:?} vs {:?}", m.src, foot);
        // beside it: sky, the mirror image of the sky above
        let m2 = v.mirror(foot.0 + 60.0, foot.1 + 10.0).unwrap();
        assert!(m2.body.is_none() && m2.src.1 < w.horizon);
        // grazing water toward the horizon reflects more than near water
        let near = v.mirror(foot.0 + 60.0, w.project([0.0, 0.0, 9.5]).unwrap().1).unwrap();
        let far = v.mirror(foot.0 + 60.0, w.project([0.0, 0.0, 100.0]).unwrap().1).unwrap();
        assert!(far.fresnel > near.fresnel * 2.0, "{} vs {}", far.fresnel, near.fresnel);
    }

    #[test]
    fn a_distant_caster_does_not_panic_the_shadow_march() {
        // at 310 m the march's growing minimum step used to pass its 0.5 m cap
        let mut w = world().sun(Sun::deg(0.0, 1.0));
        let s = w.spot_at(0.0, 310.0);
        w.place(s, Sdf::block(s.p(0.0, 5.5, 0.0), s.size(2.0, 12.0, 2.0), 0.0));
        let c = w.cast([0.0, 0.0, 10.0], [0.0, 1.0, 0.0]);
        assert!((0.0..=1.0).contains(&c), "{c}");
    }

    #[test]
    fn overlapping_bodies_are_ordered_by_world_depth() {
        // A (anchored at 10 m, 0.1 m deep) is seen in front of B (anchored at
        // 10.1 m, 1 m deep, so its front face is at 9.6 m): B's face is nearer
        let mut w = world();
        let a = w.spot_at(0.0, 10.0);
        let b = w.spot_at(0.0, 10.1);
        let sa = Sdf::block(a.p(0.0, 1.0, 0.0), a.size(1.0, 2.0, 0.1), 0.0);
        let sb = Sdf::block(b.p(0.0, 1.0, 0.0), b.size(1.0, 2.0, 1.0), 0.0);
        let (x, y) = (501.0, 365.0);
        let da = a.world([x, y, sa.hit(x, y).unwrap().z])[2];
        let db = b.world([x, y, sb.hit(x, y).unwrap().z])[2];
        assert!(db < da, "{db} vs {da}");
        w.place(a, sa);
        let ib = w.place(b, sb);
        let v = w.view(Frame::new(500, 350, 0.5));
        let p = v.at(x, y);
        assert_eq!(p.what, What::Body(ib));
        assert!((p.dist - db).abs() < 0.01, "{} vs {db}", p.dist);
    }

    #[test]
    fn sunk_surfaces_stay_below_the_waterline() {
        let mut w = world().ground(|_, _| -1.0).water(Water::new(0.0));
        let s = w.spot_at(0.0, 10.0);
        w.place(s, Sdf::block(s.p(0.0, 0.0, 0.0), s.size(1.0, 2.0, 2.0), 0.0));
        let v = w.view(Frame::new(500, 350, 0.5));
        for yy in 0..40 {
            for xx in -20..=20 {
                let (x, y) = (s.x + xx as f32 * 0.5, s.y - 10.0 + yy as f32 * 0.5);
                let p = v.at(x, y);
                if let What::Body(_) = p.what {
                    assert!(p.at[1] >= w.surface(p.at[0], p.at[2]) - 0.01, "({x},{y}) {:?}", p.at);
                }
            }
        }
        // the part above the water is still seen
        assert!(matches!(v.at(s.x, s.y - s.m(0.5)).what, What::Body(_)));
    }

    #[test]
    fn reflections_beyond_the_backdrop_run_forward() {
        let w = world().ground(|_, _| -1.0).water(Water::new(0.0));
        assert!(w.backdrop < 1000.0);
        let v = w.view(Frame::new(100, 70, 0.1));
        let (x, y) = w.project([0.0, 0.0, 1000.0]).unwrap();
        let p = w.to_ground(x, y).unwrap();
        assert!(p[2] > w.backdrop, "{p:?}");
        let m = v.mirror(x, y).unwrap();
        assert!(m.travel > 0.0, "{}", m.travel);
        assert!(m.at[1] > 0.0 && m.at[2] > p[2], "{:?}", m.at);
        assert!(m.src.1 < w.horizon, "{:?}", m.src);
    }


    #[test]
    fn depth_masks_know_what_is_in_front() {
        // a sea from 20 m out, a boulder at 12 m, a painted figure at 9 m
        // standing in front of the boulder's left half
        let mut w = world().ground(|_, z| if z > 20.0 { -1.0 } else { 0.1 }).water(Water::new(0.0));
        let s = w.spot_at(0.0, 12.0);
        let stone = w.place(s, Sdf::ellipsoid(s.p(0.0, 0.3, 0.0), s.size(1.2, 0.8, 1.0)));
        let fs = w.spot_at(-0.6, 9.0);
        let f = Frame::new(500, 350, 0.5);
        let fig = Mask::from_shape(f, Shape::new().rect(fs.x - fs.m(0.25), fs.y - fs.m(1.7), fs.m(0.5), fs.m(1.7)));
        let li = w.layer("figure", fig, LayerDepth::At(fs.at[2]));
        let v = w.view(f);
        let d = v.depths();
        let is = |t: Thing| move |x: Thing| x == t;
        // the stone is hidden where the figure stands in front of it
        let vis = d.visible(&is(Thing::Body(stone)));
        let (fx, fy) = (fs.x, s.y - s.m(0.4));
        assert!(vis.sample(fx, fy) < 0.01, "{}", vis.sample(fx, fy));
        assert!(vis.sample(s.x + s.m(0.6), fy) > 0.99);
        let front = d.front(&is(Thing::Body(stone)));
        assert!(front.sample(fx, fy) > 0.99 && front.sample(s.x + s.m(0.6), fy) < 0.01);
        // a sea veil painted behind the figure keeps off it; one behind
        // the stone keeps off the stone and the figure over it
        let sea_y = w.project([0.0, 0.0, 60.0]).unwrap().1;
        let behind_fig = d.behind(&is(Thing::Layer(li)));
        assert!(behind_fig.sample(fx, fy) < 0.01 && behind_fig.sample(fx + 150.0, sea_y) > 0.99);
        let behind_stone = d.behind(&is(Thing::Body(stone)));
        assert!(behind_stone.sample(s.x + s.m(0.6), fy) < 0.01 && behind_stone.sample(fx, fy) < 0.01);
        // the water, uncovered, is the water less the figure's shoulders over it
        let water = d.visible(&is(Thing::Water));
        let head = (fs.x, fs.y - fs.m(1.2));
        assert!(head.1 > w.horizon && head.1 < w.project([0.0, 0.0, 20.0]).unwrap().1, "the head is over the sea");
        assert!(water.sample(head.0, head.1) < 0.01 && water.sample(head.0 - 150.0, head.1) > 0.99);
        // a pass at 15 m: hidden by the stone and the figure, not by the far sea
        let at = d.at_depth(15.0);
        assert!(at.sample(fx, fy) < 0.01 && at.sample(s.x + s.m(0.6), fy) < 0.01 && at.sample(fx + 150.0, sea_y) > 0.99);
        // between 8 and 13 m: the figure and the stone, not the sky
        let mid = d.between(8.0, 13.0);
        assert!(mid.sample(fx, fy) > 0.99 && mid.sample(fx, 50.0) < 0.01);
        // the stone's silhouette is soft over a pixel, not stair-stepped
        let partial = vis.data.iter().filter(|c| **c > 0.05 && **c < 0.95).count();
        assert!(partial > 20, "{partial}");
    }

    #[test]
    fn soft_shadows_fall_off_without_rings() {
        let mut w = world().sun(Sun::deg(-80.0, 20.0));
        let s = w.spot_at(0.0, 10.0);
        let pole = w.place(s, Sdf::block(s.p(0.0, 1.0, 0.0), s.size(0.3, 2.0, 0.3), s.m(0.02)));
        let v = w.view(Frame::new(500, 350, 0.5));
        let all = |_: BodyId| true;
        let (sharp, soft) = (v.soft_shadows(1.0, &all), v.soft_shadows(3.0, &all));
        // across the shadow near its foot and far out: the edge widens with
        // distance, and more with `soft`
        let d = w.sun.dir();
        let back = -d[2] / -d[0];
        let edge = |m: &Mask, dist: f32| {
            let n = (0..400).filter(|k| {
                let z = 10.0 + dist * back + (*k as f32 - 200.0) * 0.005;
                let (x, y) = w.project([dist, 0.0, z]).unwrap();
                let c = m.sample(x, y);
                c > 0.05 && c < 0.95
            });
            n.count()
        };
        assert!(edge(&sharp, 4.0) > edge(&sharp, 0.8), "{} {}", edge(&sharp, 4.0), edge(&sharp, 0.8));
        assert!(edge(&soft, 4.0) > edge(&sharp, 4.0), "{} {}", edge(&soft, 4.0), edge(&sharp, 4.0));
        // the contact shadow: darkest at the foot, falling off smoothly
        // (monotone, no step) and nothing a reach away or in the sky
        let occ = v.occlusion(0.6, &|b| b == pole);
        let prof: Vec<f32> = (0..60).map(|k| {
            let (x, y) = w.project([-0.16 - k as f32 * 0.012, 0.0, 9.84]).unwrap();
            occ.sample(x, y)
        }).collect();
        assert!(prof[0] > 0.2, "{prof:?}");
        assert!(prof[59] < 0.02, "{prof:?}");
        for p in prof.windows(2) {
            assert!(p[1] <= p[0] + 0.03, "not monotone: {prof:?}");
            assert!(p[0] - p[1] < 0.12, "a step: {prof:?}");
        }
        assert_eq!(occ.sample(s.x, 20.0), 0.0);
        assert!(occ.sample(s.x, s.y - s.m(1.95)) < 0.05, "not up the pole");
    }

    #[test]
    fn a_ribbon_narrows_as_it_recedes() {
        let w = world();
        let m = Mask::from_shape(Frame::new(500, 350, 0.5), w.ribbon(&[(0.0, 3.0), (0.5, 10.0), (0.0, 40.0)], |_| 1.0));
        let row = |y: f32| (0..500).filter(|x| m.sample(*x as f32 * 2.0 + 1.0, y) > 0.5).count();
        let (near, far) = (row(w.project([0.0, 0.0, 4.0]).unwrap().1), row(w.project([0.0, 0.0, 30.0]).unwrap().1));
        assert!(near > far * 4 && far >= 1, "near {near} far {far}");
        // spacing recedes
        let posts = w.recede((2.0, 4.0), (0.0, 2.0), 5);
        let gaps: Vec<f32> = posts.windows(2).map(|p| p[0].y - p[1].y).collect();
        assert!(gaps.windows(2).all(|g| g[1] < g[0]));
    }
}
