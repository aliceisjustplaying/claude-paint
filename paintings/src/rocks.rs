//! Rocks and mountains as a painter builds them: first the solid (what it
//! is made of, how it is broken, where the light comes from), then paint laid
//! plane by plane with brushes. The engine's `form` module supplies the
//! solid and the light; everything here (the shapes of the stones, the colors,
//! the order of passes) is this painter's choice.
//!
//! Order of work, the old way: block in the whole mass thinly in its shadow
//! color, strokes running down the planes; then the lit planes in stiffer,
//! lighter paint, each plane stroked its own way so the breaks between them
//! come out hard; the turning halftones fused with a soft brush where the form
//! rounds (never across a break); then the accents: dark joints and crevices,
//! the odd light arris; last the brightest lights, thick, in a few touches.

use paint::color::{Mix, mix};
use paint::form::{Light, PartId, Sample, aerial};
use paint::{Canvas, Fbm, Form, Mask, Rgb, Ridge, Relief, Sdf, Shade, Style, Tool, hex, smoothstep};

/// A panel of the canvas (units): x, y, width, height.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn at(&self, u: f32, v: f32) -> (f32, f32) {
        (self.x + u * self.w, self.y + v * self.h)
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }
}

/// The colors a painter mixes for a stone, one per family of planes.
#[derive(Clone, Copy, Debug)]
pub struct Stone {
    /// Planes square to the light.
    pub light: Rgb,
    /// Planes turning away from it (the halftone).
    pub half: Rgb,
    /// Shadow planes lit by the sky.
    pub shadow: Rgb,
    /// The core shadow, just past the turn, where no reflected light reaches.
    pub core: Rgb,
    /// Light reflected into the shadows from the lit ground (warm).
    pub bounce: Rgb,
    /// Joints, cracks and crevices.
    pub crevice: Rgb,
}

impl Stone {
    /// Weathered granite in daylight: gray, a little warm in the light, cool
    /// in the shadow (Riesengebirge boulders).
    pub fn granite() -> Self {
        Stone {
            light: hex("#b8ac98"),
            half: hex("#857d70"),
            shadow: hex("#4d4d52"),
            core: hex("#2d2b2c"),
            bounce: hex("#6e5c48"),
            crevice: hex("#1c1a19"),
        }
    }
    /// Elbe sandstone: ochre-gray, warm.
    pub fn sandstone() -> Self {
        Stone {
            light: hex("#cdb38a"),
            half: hex("#9c8463"),
            shadow: hex("#5d5249"),
            core: hex("#3a312b"),
            bounce: hex("#7d6247"),
            crevice: hex("#231c18"),
        }
    }
    /// Rügen chalk: white in the light, blue-gray shadows.
    pub fn chalk() -> Self {
        Stone {
            light: hex("#efe9da"),
            half: hex("#cfcabe"),
            shadow: hex("#9aa0a6"),
            core: hex("#7b8088"),
            bounce: hex("#b7ab97"),
            crevice: hex("#5e6168"),
        }
    }
    /// Distant mountain rock: gray-violet in shadow, warm where lit.
    pub fn mountain() -> Self {
        Stone {
            light: hex("#a39580"),
            half: hex("#7a7068"),
            shadow: hex("#4a4a55"),
            core: hex("#34343f"),
            bounce: hex("#5e5550"),
            crevice: hex("#26252c"),
        }
    }

    /// The color for a point lit as `s`: the light family (half → light by how
    /// square the plane is to the light) or the shadow family (core → shadow
    /// by how much sky and reflected light it sees), divided at the turn.
    pub fn at(&self, s: &Shade) -> Rgb {
        let lit = mix(self.half, self.light, s.direct.powf(0.8), Mix::Pigment);
        let open = (s.bounce * 1.8 + (s.sky - 0.5) * 0.5 + 0.15).clamp(0.0, 1.0);
        let dark = mix(self.core, self.shadow, open, Mix::Pigment);
        let dark = mix(dark, self.bounce, (s.bounce * 0.8).min(0.5), Mix::Pigment);
        mix(dark, lit, s.lit(0.14), Mix::Pigment)
    }
}

/// The air between the painter and a distant thing.
#[derive(Clone, Copy, Debug)]
pub struct Air {
    pub color: Rgb,
    /// Distance (in the form's `dist` units) at which 63 % of a color is lost.
    pub visibility: f32,
}

impl Air {
    pub fn over(&self, col: Rgb, dist: f32) -> Rgb {
        mix(col, self.color, aerial(dist, self.visibility), Mix::Light)
    }
}

// --------------------------------------------------------------- geometry

/// Ground running back from the viewer, below `horizon`: a gently tilted
/// plane (nearer as it comes down the picture), with a little unevenness.
pub fn ground(form: &mut Form, r: Rect, horizon: f32, seed: u32) -> PartId {
    let lumps = Fbm::new(seed, 3, 60.0);
    let g = Relief::new([r.x, horizon, r.x + r.w, r.y + r.h], move |x, y| {
        if y < horizon {
            return None;
        }
        Some(((y - horizon) * 0.12 + 1.5 * lumps.get(x, y * 3.0), 0))
    });
    form.add_at(&g, &move |_, y, _| 0.3 + 1.2 * (1.0 - smoothstep(horizon, horizon + r.h, y)))
}

/// A granite erratic: a rounded mass turned a little, split by three
/// fracture planes (a sloping top toward the light, a flank away from it, a
/// front face), weathered with a ridged grain, sunk in the ground.
/// Facets: 10 top, 11 flank, 12 front, 0 the rounded body.
pub fn boulder(form: &mut Form, r: Rect, horizon: f32, seed: u32) -> PartId {
    let (cx, base) = r.at(0.5, 0.86);
    let (rx, ry, rz) = (r.w * 0.3, r.h * 0.33, r.w * 0.24);
    let zg = (base - horizon) * 0.12;
    // centered at the depth of the ground it stands on, so the ground
    // behind it stays behind and the ground in front hides its foot
    let c = [cx, base - ry * 0.8, zg + rz * 0.05];
    let at = |u: f32, v: f32, w: f32| [c[0] + u * rx, c[1] + v * ry, c[2] + w * rz];
    // a lumpy mass first (weathering rounds a block into a boulder), then
    // the breaks, which are younger than the rounding and cut it cleanly
    let rock = Sdf::ellipsoid(c, [rx, ry, rz])
        .turn(c, 0.3, 0.1, -0.12)
        .rough(r.w * 0.035, r.w * 0.3, seed + 1, false)
        .cut(at(-0.1, -0.5, 0.3), [-0.35, -1.0, 0.45], 10, r.w * 0.008)
        .cut(at(0.58, 0.0, 0.1), [1.0, -0.15, 0.3], 11, r.w * 0.006)
        .cut(at(-0.05, 0.1, 0.7), [-0.3, 0.05, 1.0], 12, r.w * 0.01)
        .cut(at(-0.48, -0.4, 0.3), [-1.0, -0.8, 0.5], 13, r.w * 0.006)
        .cut(at(0.3, -0.62, 0.0), [0.4, -1.0, 0.2], 14, r.w * 0.006)
        .rough(r.w * 0.0018, r.w * 0.05, seed, true);
    form.add(&rock, 0.5)
}

/// A sandstone outcrop (Elbe sandstone): blocks stacked on their bedding
/// planes, split by vertical joints, edges rounded by weather, the surface
/// pitted; a recessed mass behind shows in the joints. Seen a little from
/// above so the tops of the blocks catch the light. Block faces are facets
/// 1–6 (see `Sdf::block`), fresh breaks 10+.
pub fn outcrop(form: &mut Form, r: Rect, seed: u32) -> Vec<PartId> {
    let s = r.w;
    let blocks: [((f32, f32), (f32, f32, f32), f32, f32, f32); 7] = [
        // (center u,v), (size w,h,d as fractions of s), z, yaw, roll
        ((0.5, 0.62), (0.56, 0.36, 0.14), -0.1, 0.25, 0.0),  // recessed core
        ((0.47, 0.8), (0.64, 0.16, 0.18), 0.0, 0.3, 0.01),   // base bed
        ((0.4, 0.64), (0.42, 0.16, 0.16), -0.01, 0.35, -0.02), // second bed
        ((0.69, 0.57), (0.15, 0.36, 0.13), 0.0, 0.2, 0.015),  // tower, right
        ((0.44, 0.475), (0.34, 0.12, 0.14), -0.02, 0.42, 0.04), // cap
        ((0.29, 0.39), (0.12, 0.1, 0.1), -0.03, 0.6, -0.1),   // a block perched on top
        ((0.14, 0.87), (0.13, 0.08, 0.1), 0.08, 0.7, 0.18),   // fallen block in front
    ];
    let mut parts = vec![];
    for (k, &((u, v), (w, h, d), z, yaw, roll)) in blocks.iter().enumerate() {
        let (cx, cy) = r.at(u, v);
        let c = [cx, cy, z * s];
        let size = [w * s, h * r.h / r.w * s * 1.0, d * s];
        let round = size[0].min(size[1]) * 0.2;
        // beds sag and swell: soft lumps break the straight edges first
        let mut b = Sdf::block(c, size, round).turn(c, yaw, 0.32, roll).rough(s * 0.01, s * 0.12, seed + 40 + k as u32, false);
        // one corner broken off along a joint
        if k % 2 == 1 {
            b = b.cut([c[0] - size[0] * 0.42, c[1] - size[1] * 0.35, c[2] + size[2] * 0.3], [-1.0, -0.6, 0.8], 10 + k as u16, round * 0.3);
        }
        let b = b.rough(s * 0.0015, s * 0.03, seed + k as u32, true);
        parts.push(form.add(&b, 0.5));
    }
    parts
}

/// The crest of a range: a base line with peaks ((x, height, half-width),
/// shoulders falling off like `pow`), broken by noise at two scales.
pub fn skyline(x: f32, base: f32, peaks: &[(f32, f32, f32)], pow: f32, rough: (&Fbm, f32), fine: (&Fbm, f32)) -> f32 {
    let mut y = base + rough.1 * rough.0.get(x, 0.5) + fine.1 * fine.0.get(x, 2.5);
    for &(px, ph, pw) in peaks {
        let d = ((x - px).abs() / pw).min(1.0);
        y -= ph * (1.0 - d).powf(pow);
    }
    y
}

/// A chalk cliff (Rügen): a near-vertical white face whose top breaks into
/// pointed pinnacles, fluted by rain runnels, crossed by faint flint bands,
/// with a scree slope at its foot; a second, farther cliff beyond.
/// Returns (near cliff, far cliff).
pub fn chalk_cliff(form: &mut Form, r: Rect, seed: u32) -> (PartId, PartId) {
    let rough = Fbm::new(seed, 3, r.w * 0.25);
    let fine = Fbm::new(seed + 1, 3, r.w * 0.02);
    let (x0, x1) = (r.x, r.x + r.w * 0.78);
    let y0 = r.y + r.h * 0.22;
    let peaks = [
        (r.x + r.w * 0.2, r.h * 0.07, r.w * 0.07),
        (r.x + r.w * 0.36, r.h * 0.13, r.w * 0.09),
        (r.x + r.w * 0.5, r.h * 0.06, r.w * 0.05),
    ];
    let foot = r.y + r.h * 0.9;
    // the cliff edge drops away toward the beach at the right, in steps
    let crest = |x: f32| {
        let t = smoothstep(r.x + r.w * 0.52, x1, x);
        let drop = (t * 0.8 + 0.2 * t * t) * (foot - y0 - r.h * 0.02);
        skyline(x, y0, &peaks, 1.2, (&rough, r.h * 0.03), (&fine, r.h * 0.008)) + drop
    };
    let near = Ridge::new(x0, x1, crest, foot - y0 + r.h * 0.2, seed)
        .lean(0.14, 1.6)
        .gullies(r.w * 0.026, 0.5)
        .fan(0.08)
        .strata(r.h * 0.11, r.w * 0.0015, 0.03)
        .base(foot);
    let near_id = form.add(&near, 0.6);
    let rough2 = Fbm::new(seed + 5, 3, r.w * 0.2);
    let far = Ridge::new(r.x + r.w * 0.55, r.x + r.w, |x| {
        let rise = smoothstep(r.x + r.w * 0.6, r.x + r.w * 0.95, x) * r.h * 0.1;
        r.y + r.h * 0.5 - rise + r.h * 0.015 * rough2.get(x, 0.3) + r.h * 0.004 * fine.get(x, 7.0)
    }, r.h * 0.2, seed + 9)
        .lean(0.15, 1.0)
        .gullies(r.w * 0.02, 0.5)
        .fan(0.1)
        .base(r.y + r.h * 0.535)
        .z0(-50.0);
    let far_id = form.add(&far, 2.0);
    (near_id, far_id)
}

/// Three ranges receding: each farther one higher on the picture, smaller in
/// its forms (finer gullies), paler in the air. Nearest first in the result.
pub fn ranges(form: &mut Form, r: Rect, seed: u32) -> Vec<PartId> {
    let specs = [
        // base v, peaks (u, height v, half-width u), gully (u), dist
        (0.64, vec![(0.2, 0.2, 0.26), (0.47, 0.06, 0.1), (0.9, 0.1, 0.25)], 0.09, 1.0),
        (0.47, vec![(0.05, 0.05, 0.14), (0.66, 0.15, 0.3), (0.78, 0.1, 0.08)], 0.055, 2.2),
        (0.36, vec![(0.33, 0.1, 0.2), (0.42, 0.06, 0.05), (0.86, 0.05, 0.15)], 0.035, 3.6),
    ];
    let mut ids = vec![];
    for (k, (v, peaks, gully, dist)) in specs.iter().enumerate() {
        let rough = Fbm::new(seed + 10 * k as u32, 4, r.w * 0.18);
        let fine = Fbm::new(seed + 10 * k as u32 + 1, 3, r.w * 0.025);
        let pk: Vec<(f32, f32, f32)> = peaks.iter().map(|&(u, h, w)| (r.x + u * r.w, h * r.h, w * r.w)).collect();
        let base = r.y + v * r.h;
        let scale = 1.0 / (1.0 + k as f32 * 0.6);
        // shoulders: the nearer range's peaks are steep, the far ones rounder
        let pow = [1.5, 1.2, 0.9][k];
        let crest = |x: f32| skyline(x, base, &pk, pow, (&rough, r.h * 0.04 * scale), (&fine, r.h * 0.012 * scale));
        let ridge = Ridge::new(r.x, r.x + r.w, crest, r.y + r.h - base + r.h * 0.2, seed + k as u32)
            .lean(0.9, 0.7)
            .gullies(gully * r.w, 0.5)
            .fan(1.0)
            .z0(-(k as f32) * 200.0);
        let d = *dist;
        // the foot of a range is a little nearer than its crest
        ids.push(form.add_at(&ridge, &move |_, _, z| d - 0.002 * z));
    }
    ids
}

// ---------------------------------------------------------------- painting

/// How the painter paints one solid: its color at a point, and how big.
pub struct Solid<'a> {
    pub parts: Vec<PartId>,
    /// Color for a point of the form (use the sample's shade, facet, dist).
    pub color: &'a (dyn Fn(f32, f32, &Sample) -> Rgb + Sync),
    /// Silhouette softness (units) at a point of the form.
    pub soft: &'a (dyn Fn(&Sample) -> f32 + Sync),
    /// Brush scale (1 = the 500-unit study size).
    pub scale: f32,
    /// Dark accents in joints and crevices (0 = none).
    pub accents: f32,
    pub seed: u64,
}

/// Which way strokes run on a plane: down the fall line on walls, around the
/// form on tops that face the sky.
fn stroke_angle(form: &Form, x: f32, y: f32) -> f32 {
    match form.sample(x, y) {
        Some(s) if s.n[1] < -0.55 => s.across(),
        Some(s) => s.fall(),
        None => std::f32::consts::FRAC_PI_2,
    }
}

/// The same point as if it lay in shadow: the color a painter blocks in with.
fn shadowed(s: &Sample, light: &Light) -> Sample {
    Sample { shade: light.shade(s.n, 1.0), ..*s }
}

/// Paint a solid of a lit `form` with brushes, dark planes first and lights
/// last. See the module docs for the order of work.
pub fn paint_solid(c: &mut Canvas, st: &Style, form: &Form, sd: &Solid) {
    let f = c.frame();
    let light = &form.lighting().expect("light the form first");
    let parts = &sd.parts;
    let sc = sd.scale;
    let sil = form.silhouette(parts, sd.soft);
    let color = sd.color;
    let col_at = |x: f32, y: f32| form.sample(x, y).filter(|s| parts.contains(&s.part)).map(|s| color(x, y, &s));
    let fallback = |x: f32, y: f32| {
        // just outside the solid (soft edges): the nearest color inside
        for r in [1.0f32, 2.5, 5.0, 9.0] {
            for (dx, dy) in [(r, 0.0), (-r, 0.0), (0.0, r), (0.0, -r)] {
                if let Some(cc) = col_at(x + dx * sc, y + dy * sc) {
                    return cc;
                }
            }
        }
        hex("#808080")
    };
    let full = |x: f32, y: f32| col_at(x, y).unwrap_or_else(|| fallback(x, y));

    // 1. block in the whole mass in its shadow colors, thin, down the planes
    let dark = |x: f32, y: f32| match form.sample(x, y).filter(|s| parts.contains(&s.part)) {
        Some(s) => color(x, y, &shadowed(&s, light)),
        None => fallback(x, y),
    };
    let body_tool = Tool { width: 7.0 * sc, ..st.body.clone() };
    let hd = paint::Handling::new(body_tool.clone())
        .mixed(&st.palette, 0.35)
        .mix_jitter(st.mix_jitter * 1.4)
        .color(dark)
        .angle(|x, y| stroke_angle(form, x, y))
        .angle_jitter(0.15)
        .length(10.0 * sc, 28.0 * sc)
        .coverage(2.6)
        .pressure(0.6, 0.9)
        .dips(2, 0.56, 0.6)
        .clip(true)
        .threshold(0.2);
    c.work(&sil, &hd, sd.seed);

    // 2. the shadow planes worked again in their own colors (core darker,
    //    reflected light lifting the far side), and the halftones
    let shadow_side = form.mask(|s| if parts.contains(&s.part) { 1.0 - s.shade.lit(0.1) } else { 0.0 }).mul(&sil);
    let hd = paint::Handling::new(Tool { width: 5.0 * sc, ..body_tool.clone() })
        .mixed(&st.palette, 0.25)
        .mix_jitter(st.mix_jitter)
        .color(full)
        .angle(|x, y| stroke_angle(form, x, y))
        .angle_jitter(0.12)
        .length(6.0 * sc, 18.0 * sc)
        .coverage(2.0)
        .pressure(0.55, 0.85)
        .dips(2, 0.5, 0.6)
        .clip(true)
        .threshold(0.3);
    c.work(&shadow_side, &hd, sd.seed + 1);

    // 3. the lit planes: stiffer, lighter paint, each plane stroked its own
    //    way, so where planes break the change is abrupt
    let lit = form.mask(|s| if parts.contains(&s.part) { s.shade.lit(0.12) } else { 0.0 }).mul(&sil);
    let hd = paint::Handling::new(Tool { width: 5.5 * sc, ..body_tool.clone() })
        .mixed(&st.palette, 0.18)
        .mix_jitter(st.mix_jitter)
        .color(full)
        .angle(|x, y| stroke_angle(form, x, y))
        .angle_jitter(0.1)
        .length(8.0 * sc, 24.0 * sc)
        .coverage(2.4)
        .pressure(0.6, 0.9)
        .dips(2, 0.6, 0.7)
        .clip(true)
        .threshold(0.35);
    c.work(&lit, &hd, sd.seed + 2);

    // 4. fuse each plane with a soft clean brush along it, and the turning
    //    halftones where the form rounds; never across a break
    let breaks = form.edges(0.7, 3.0 * sc, 2.0 * sc).dilate(2.5 * sc);
    let turning = form
        .mask(|s| if parts.contains(&s.part) { 0.6 + 0.4 * smoothstep(-0.25, 0.0, s.shade.turn) * (1.0 - smoothstep(0.25, 0.5, s.shade.turn)) } else { 0.0 })
        .mul(&sil.erode(2.0 * sc))
        .subtract(&breaks);
    let soft = paint::Handling::new(Tool { pickup: 0.15, ..Tool::badger(9.0 * sc) })
        .blender()
        .angle(|x, y| stroke_angle(form, x, y))
        .angle_jitter(0.1)
        .length(10.0 * sc, 25.0 * sc)
        .coverage(1.6)
        .pressure(0.22, 0.32)
        .dips(3, 0.0, 0.9)
        .clip(true)
        .threshold(0.4);
    c.work(&turning, &soft, sd.seed + 3);

    // 5. accents: dark joints and crevices where the surface folds in, thin
    //    strokes of a pointed brush along them
    if sd.accents > 0.0 {
        // only the big breaks: judged over a few units, so the grain of the
        // stone doesn't count
        let sp = 2.5 * sc;
        let cracks = form
            .mask(|s| if parts.contains(&s.part) { 1.0 } else { 0.0 })
            .mul(&form.edges(0.8, 3.0 * sc, sp))
            .mul(&Mask::from_fn(f, |x, y| smoothstep(0.3, 0.7, -form.bend(x, y, sp))))
            .mul(&sil.erode(1.0 * sc));
        let crev = |x: f32, y: f32| {
            let base = full(x, y);
            let s = form.sample(x, y);
            let deep = s.map_or(0.5, |s| 1.0 - s.shade.value);
            mix(base, dark_of(x, y, form, color), 0.5 + 0.4 * deep, Mix::Pigment)
        };
        let hd = paint::Handling::new(Tool::round_sable(1.6 * sc))
            .mixed(&st.palette, 0.15)
            .color(crev)
            .angle(|x, y| form.edge_angle(x, y, sp))
            .angle_jitter(0.08)
            .length(3.0 * sc, 9.0 * sc)
            .coverage(0.9 * sd.accents)
            .pressure(0.5, 0.8)
            .dips(3, 0.6, 0.8)
            .clip(true)
            .threshold(0.35);
        c.work(&cracks, &hd, sd.seed + 4);
    }

    // 6. last, the brightest lights: a few thick touches where planes face
    //    the light squarely
    let top = form.mask(|s| if parts.contains(&s.part) { smoothstep(0.6, 0.85, s.shade.direct) } else { 0.0 }).mul(&sil.erode(1.5 * sc));
    let hd = paint::Handling::new(Tool { lay: 1.1, ..Tool::filbert(3.2 * sc) })
        .mixed(&st.palette, 0.1)
        .mix_jitter(st.mix_jitter)
        .color(full)
        .angle(|x, y| stroke_angle(form, x, y))
        .angle_jitter(0.3)
        .length(8.0 * sc, 20.0 * sc)
        .coverage(0.3)
        .pressure(0.5, 0.8)
        .dips(2, 0.8, 0.8)
        .clip(true)
        .threshold(0.5);
    c.work(&top, &hd, sd.seed + 5);
    c.dry();
}

fn dark_of(x: f32, y: f32, form: &Form, color: &(dyn Fn(f32, f32, &Sample) -> Rgb + Sync)) -> Rgb {
    match form.sample(x, y) {
        Some(s) => {
            let mut s2 = s;
            s2.shade = Shade { turn: -0.3, direct: 0.0, cast: 1.0, bounce: 0.0, sky: 0.2, value: 0.0 };
            color(x, y, &s2)
        }
        None => hex("#303030"),
    }
}

/// Paint mountain ranges (far to near, as given nearest first): each one's
/// silhouette softer the farther it is; strokes down the fall lines so the
/// gullies and spurs come out of the brushwork; lit spurs laid over the
/// shadowed gullies; mist gathering at the feet. `color` is the painter's
/// stone color before the air; `mist(x, y, k)` how much valley mist lies in
/// front of range `k` (0 nearest) there.
pub fn paint_ranges(c: &mut Canvas, st: &Style, form: &Form, ids: &[PartId], air: &Air, color: &(dyn Fn(f32, f32, &Sample) -> Rgb + Sync), mist: &(dyn Fn(f32, f32, usize) -> f32 + Sync), scale: f32, seed: u64) {
    let light = form.lighting().expect("light the form first");
    for (k, &id) in ids.iter().enumerate().rev() {
        let d0 = form.mask(|s| if s.part == id { s.dist } else { 0.0 });
        let far = d0.data.iter().cloned().fold(0.0f32, f32::max);
        let a = aerial(far, air.visibility);
        // edges lost in the air: a hard crest near, a soft one far
        let sil = form.silhouette(&[id], |s| (0.4 + 5.0 * aerial(s.dist, air.visibility).powi(2)) * scale);
        let mist = |x: f32, y: f32| mist(x, y, k);
        let col = move |x: f32, y: f32, s: &Sample| {
            let base = air.over(color(x, y, s), s.dist);
            mix(base, air.color, mist(x, y), Mix::Light)
        };
        let at = |x: f32, y: f32| {
            let s = form.sample(x, y).filter(|s| s.part == id).or_else(|| form.sample(x, y + 3.0 * scale).filter(|s| s.part == id)).or_else(|| form.sample(x, y + 8.0 * scale).filter(|s| s.part == id));
            s.map_or(air.color, |s| col(x, y, &s))
        };
        let sz = (1.0 - 0.25 * k as f32) * scale;
        // body: the whole range, thin, strokes down the fall lines
        let hd = paint::Handling::new(Tool { width: 6.0 * sz, ..st.body.clone() })
            .mixed(&st.palette, 0.4)
            .mix_jitter(st.mix_jitter * (1.0 - a))
            .color(|x, y| {
                let s = form.sample(x, y).filter(|s| s.part == id);
                match s {
                    Some(s) => col(x, y, &shadowed(&s, &light)),
                    None => at(x, y),
                }
            })
            .angle(|x, y| form.fall(x, y))
            .angle_jitter(0.12)
            .length(8.0 * sz, 30.0 * sz)
            .coverage(2.6)
            .pressure(0.55, 0.85)
            .dips(2, 0.5, 0.6)
            .clip(true)
            .threshold(0.15);
        c.work(&sil, &hd, seed + 10 * k as u64);
        // the lit spurs, in stiffer paint over the gullies' shadow
        let lit = form.mask(|s| if s.part == id { s.shade.lit(0.12) } else { 0.0 }).mul(&sil);
        let hd = paint::Handling::new(Tool { width: 4.0 * sz, lay: 1.2, ..st.body.clone() })
            .mixed(&st.palette, 0.2)
            .mix_jitter(st.mix_jitter * (1.0 - a))
            .color(at)
            .angle(|x, y| form.fall(x, y))
            .angle_jitter(0.1)
            .length(5.0 * sz, 16.0 * sz)
            .coverage(2.0)
            .pressure(0.55, 0.85)
            .dips(2, 0.55, 0.7)
            .clip(true)
            .threshold(0.3);
        c.work(&lit, &hd, seed + 10 * k as u64 + 1);
        // gullies: dark strokes down the deepest folds, only near enough to read
        if a < 0.5 {
            let sp = 1.5 * scale;
            let folds = Mask::from_fn(c.frame(), |x, y| if form.part(x, y) == id { smoothstep(0.08, 0.3, -form.bend(x, y, sp)) * (1.0 - mist(x, y)) } else { 0.0 })
                .mul(&sil.erode(2.0 * scale));
            let hd = paint::Handling::new(Tool::round_sable(1.6 * sz))
                .mixed(&st.palette, 0.25)
                .color(|x, y| {
                    let s = form.sample(x, y).filter(|s| s.part == id);
                    s.map_or(air.color, |s| {
                        let mut s2 = s;
                        s2.shade.direct = 0.0;
                        s2.shade.bounce = 0.0;
                        col(x, y, &s2)
                    })
                })
                .angle(|x, y| form.fall(x, y))
                .angle_jitter(0.1)
                .length(4.0 * sz, 14.0 * sz)
                .coverage(0.8)
                .pressure(0.5, 0.8)
                .dips(3, 0.5, 0.8)
                .clip(true)
                .threshold(0.3);
            c.work(&folds, &hd, seed + 10 * k as u64 + 2);
        }
        c.dry();
        // mist at the foot: a thin veil in the air's color, lying level,
        // then fused
        let veil = Mask::from_fn(c.frame(), |x, y| mist(x, y)).mul(&sil.dilate(4.0 * scale));
        let hd = paint::Handling::new(Tool { width: 14.0 * scale, lay: 0.7, ..st.broad.clone() })
            .mixed(&st.palette, 0.6)
            .mix_jitter(st.mix_jitter * 0.5)
            .color(|_, _| air.color)
            .angle(|_, _| 0.0)
            .angle_jitter(0.05)
            .length(40.0 * scale, 110.0 * scale)
            .coverage(1.6)
            .pressure(0.4, 0.6)
            .dips(2, 0.35, 0.6)
            .load_at(|x, y| mist(x, y))
            .threshold(0.15);
        c.work(&veil, &hd, seed + 10 * k as u64 + 3);
        if let Some(b) = st.blend() {
            let b = b.angle(|_, _| 0.0).length(60.0 * scale, 160.0 * scale).pressure(0.25, 0.35).threshold(0.2);
            c.work(&veil, &b, seed + 10 * k as u64 + 4);
        }
        c.dry();
    }
}

