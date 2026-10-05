//! Every state value a canvas holds, as named arrays: what the small
//! before-change results in `notes/thinner/baseline/` compare field by field
//! (`easel run --dump-state`, and `scripts/state_compare` reads them).
//!
//! It only reads the canvas. It names what the checkpoint (`checkpoint.rs`)
//! stores, value for value, plus the wet layer's film floors (`wet.floor`),
//! which the checkpoint leaves out. A later engine that adds state (solvent,
//! clock fields) adds fields here under new names; a comparison then checks
//! the fields both sides have and lists the rest.
//!
//! Field names are fixed: `scripts/state_compare` matches them by name.
//! Shapes are row major, `[h, w]` or `[h, w, k]` for per-pixel values.

use crate::canvas::Canvas;
use crate::wet::LAT;

/// A field's values, little-endian when written.
pub enum Values {
    F32(Vec<f32>),
    F64(Vec<f64>),
    U32(Vec<u32>),
    U64(Vec<u64>),
}

pub struct Field {
    pub name: &'static str,
    pub shape: Vec<usize>,
    pub values: Values,
}

impl Values {
    pub fn dtype(&self) -> &'static str {
        match self {
            Values::F32(_) => "f32",
            Values::F64(_) => "f64",
            Values::U32(_) => "u32",
            Values::U64(_) => "u64",
        }
    }
    pub fn le_bytes(&self) -> Vec<u8> {
        match self {
            Values::F32(v) => v.iter().flat_map(|x| x.to_le_bytes()).collect(),
            Values::F64(v) => v.iter().flat_map(|x| x.to_le_bytes()).collect(),
            Values::U32(v) => v.iter().flat_map(|x| x.to_le_bytes()).collect(),
            Values::U64(v) => v.iter().flat_map(|x| x.to_le_bytes()).collect(),
        }
    }
}

fn boxed(b: Option<(usize, usize, usize, usize)>) -> Vec<u64> {
    match b {
        None => vec![0, 0, 0, 0, 0],
        Some((a, b, c, d)) => vec![1, a as u64, b as u64, c as u64, d as u64],
    }
}

impl Canvas {
    /// The canvas's state as named fields (see the module docs).
    pub fn state_fields(&self) -> Vec<Field> {
        let f = self.f;
        let (h, w) = (f.h, f.w);
        let px = |k: usize| vec![h, w, k];
        let wt = &self.wet;
        let ck = &wt.clock;
        let mut v = vec![
            Field { name: "frame", shape: vec![10], values: Values::U64([f.w, f.h, f.x0, f.y0, f.full_w, f.full_h, self.keep.0, self.keep.1, self.keep.2, self.keep.3].map(|x| x as u64).to_vec()) },
            Field { name: "scale", shape: vec![1], values: Values::F32(vec![f.scale]) },
            Field { name: "mm_per_unit", shape: vec![1], values: Values::F32(vec![self.mm_per_unit]) },
            Field {
                name: "linen",
                shape: vec![5],
                values: Values::F64(match self.linen {
                    None => vec![0.0; 5],
                    Some(l) => vec![l.warp_per_cm as f64, l.weft_per_cm as f64, l.crown_um as f64, l.slubs as f64, 1.0],
                }),
            },
            Field { name: "linen_seed", shape: vec![1], values: Values::U64(vec![self.linen.map_or(0, |l| l.seed)]) },
            Field { name: "surf_gen", shape: vec![1], values: Values::U64(vec![self.surf_gen]) },
            Field { name: "engine", shape: vec![1], values: Values::U32(vec![self.engine]) },
            Field { name: "ground_um", shape: vec![1], values: Values::F32(vec![self.ground_um]) },
            Field { name: "color", shape: px(3), values: Values::F32(self.px.iter().flat_map(|p| *p).collect()) },
            Field { name: "height", shape: px(1), values: Values::F32(self.height.clone()) },
            Field { name: "film", shape: px(1), values: Values::F32(self.film.clone()) },
            Field { name: "wet.vol", shape: px(1), values: Values::F32(wt.vol.clone()) },
            Field { name: "wet.lat", shape: px(LAT), values: Values::F32(wt.lat.iter().flat_map(|l| *l).collect()) },
            // Keep the legacy hiding/stiffness/drying field byte-compatible.
            // Engine 4's additional properties have their own fields below.
            Field { name: "wet.hide", shape: px(3), values: Values::F32(wt.hide.iter().flat_map(|p| [p[0], p[1], p[2]]).collect()) },
            Field { name: "wet.stroke", shape: px(1), values: Values::U32(wt.stroke.clone()) },
            Field { name: "wet.touched", shape: px(1), values: Values::U32(wt.touched.clone()) },
            Field { name: "wet.floor", shape: px(1), values: Values::F32(wt.floor.clone()) },
            Field { name: "wet.cover", shape: px(1), values: Values::F32(wt.cover.clone()) },
            Field { name: "wet.current", shape: vec![1], values: Values::U32(vec![wt.current]) },
            Field { name: "wet.dirty", shape: vec![5], values: Values::U64(boxed(wt.dirty)) },
            Field { name: "clock.now", shape: vec![1], values: Values::F64(vec![ck.now]) },
            Field { name: "clock.mark", shape: vec![1], values: Values::U32(vec![ck.mark]) },
            Field { name: "clock.tacky", shape: vec![5], values: Values::U64(boxed(ck.tacky)) },
        ];
        // per-pixel drying state: empty (shape [0]) until the first wait
        let n = if ck.px.is_empty() { None } else { Some(px(1)) };
        type Get = fn(&crate::drying::Px) -> f32;
        let parts: [(&'static str, Get); 6] = [
            ("clock.cure", |p| p.cure),
            ("clock.lev", |p| p.lev),
            ("clock.seen", |p| p.seen),
            ("clock.sub", |p| p.sub),
            ("clock.srate", |p| p.srate),
            ("clock.th", |p| p.th),
        ];
        for (name, get) in parts {
            v.push(Field { name, shape: n.clone().unwrap_or(vec![0]), values: Values::F32(ck.px.iter().map(get).collect()) });
        }
        let drawing: Vec<f32> = self.drawing.as_ref().map(|d| d.to_f32s().collect()).unwrap_or_default();
        v.push(Field { name: "drawing", shape: vec![drawing.len()], values: Values::F32(drawing) });
        v.push(Field { name: "hand_slice", shape: vec![2], values: Values::F32(self.hand_slice.map_or(vec![0.0, 0.0], |m| vec![1.0, m])) });
        v.push(Field { name: "tally", shape: vec![9], values: Values::U64(self.tally.to_words().to_vec()) });
        // added with the thinner (engine 3): the solvent in the open film,
        // µm per pixel (0 where there is none, and before engine 3)
        v.push(Field { name: "wet.solvent", shape: px(1), values: Values::F32(if wt.solv.len() == h * w { wt.solv.clone() } else { vec![0.0; h * w] }) });
        if self.engine >= 4 {
            v.push(Field { name: "wet.turps", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[3]).collect()) });
            v.push(Field { name: "wet.oil", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[4]).collect()) });
            v.push(Field { name: "gloss", shape: px(1), values: Values::F32(self.gloss.clone()) });
            v.push(Field { name: "absorb", shape: px(1), values: Values::F32(self.absorb.clone()) });
        }
        // engine 6: each film's oil when its pigment packs and when drained,
        // the share of it packed on an absorbent ground, its tube paint's
        // oil by volume and the share of its oil that is waxed
        if self.engine >= 6 {
            v.push(Field { name: "wet.packed_oil", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[5]).collect()) });
            v.push(Field { name: "wet.floor_oil", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[6]).collect()) });
            v.push(Field { name: "wet.packed_share", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[7]).collect()) });
            v.push(Field { name: "wet.oil_volume", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[8]).collect()) });
            v.push(Field { name: "wet.wax", shape: px(1), values: Values::F32(wt.hide.iter().map(|p| p[9]).collect()) });
        }
        v
    }
}
