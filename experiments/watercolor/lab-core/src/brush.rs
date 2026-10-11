// Copyright (c) 2026 Ryan McCarty. MIT License.
// Mechanical port of uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See experiments/watercolor/THIRD_PARTY_NOTICES.md for the full license.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolId {
    Round,
    Mop,
    Rigger,
    Drybrush,
    Spatter,
    Water,
    Lift,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolSpec {
    pub id: ToolId,
    pub name: String,
    pub size_scale: f64,
    pub water: f64,
    pub pigment: f64,
    pub spacing: f64,
    pub dryness: f64,
    pub depletion: f64,
    pub scatter: f64,
}

/// Source: src/paint/brush.ts:35–45 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn tool(id: ToolId) -> ToolSpec {
    let (name, size_scale, water, pigment, spacing, dryness, depletion, scatter) = match id {
        ToolId::Round => ("Round", 1.0, 1.0, 1.0, 0.22, 0.0, 0.004, 0.0),
        ToolId::Mop => ("Mop", 2.4, 1.8, 0.55, 0.25, 0.0, 0.002, 0.0),
        ToolId::Rigger => ("Rigger", 0.32, 0.55, 1.5, 0.18, 0.0, 0.009, 0.0),
        ToolId::Drybrush => ("Dry brush", 1.15, 0.12, 1.2, 0.16, 1.0, 0.012, 0.08),
        ToolId::Spatter => ("Spatter", 0.38, 0.9, 1.2, 2.6, 0.0, 0.006, 3.2),
        ToolId::Water => ("Water", 1.3, 1.6, 0.0, 0.22, 0.0, 0.0, 0.0),
        ToolId::Lift => ("Lift", 1.2, 0.8, 0.0, 0.2, 0.0, 0.0, 0.0),
    };
    ToolSpec {
        id,
        name: name.into(),
        size_scale,
        water,
        pigment,
        spacing,
        dryness,
        depletion,
        scatter,
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Stamp {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub water: f64,
    pub pigment: f64,
    pub dryness: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Brush {
    pub tool: ToolSpec,
    pub size: f64,
    pub water_amount: f64,
    pub pigment_load: f64,
    pub reservoir: f64,
    pub carry: f64,
    pub last_x: f64,
    pub last_y: f64,
    pub seed: u32,
    pub radius_scale: f64,
}

/// Source: src/paint/brush.ts:82–85 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn brush_radius(tool: &ToolSpec, size: f64) -> f64 {
    let eased = 0.012 + 0.988 * size * size;
    (eased * 68.0 * tool.size_scale).max(1.1)
}

impl Brush {
    /// Source: src/paint/brush.ts:87–98 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn begin(
        tool: ToolSpec,
        size: f64,
        water_amount: f64,
        pigment_load: f64,
        x: f64,
        y: f64,
        seed: u32,
        radius_scale: f64,
    ) -> Self {
        Self {
            tool,
            size,
            water_amount,
            pigment_load,
            reservoir: 1.0,
            carry: 0.0,
            last_x: x,
            last_y: y,
            seed,
            radius_scale,
        }
    }

    /// Source: src/paint/brush.ts:101–107 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    fn rand(&mut self) -> f64 {
        self.seed = self.seed.wrapping_add(0x6d2b79f5);
        let mut t = self.seed;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        (t ^ (t >> 14)) as f64 / 4294967296.0
    }

    /// Source: src/paint/brush.ts:109–129 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    fn make_stamp(&mut self, x: f64, y: f64, pressure: f64) -> Stamp {
        let radius =
            brush_radius(&self.tool, self.size) * self.radius_scale * (0.65 + 0.5 * pressure);
        let water =
            self.tool.water * (0.25 + 0.95 * self.water_amount) * (0.55 + 0.45 * self.reservoir);
        let pigment = self.tool.pigment * self.pigment_load * self.reservoir;
        self.reservoir =
            (self.reservoir - self.tool.depletion * (0.5 + self.pigment_load)).max(0.15);
        let mut sx = x;
        let mut sy = y;
        if self.tool.scatter > 0.0 {
            let angle = self.rand() * std::f64::consts::PI * 2.0;
            let dist = self.rand().sqrt() * self.tool.scatter * radius;
            sx += angle.cos() * dist;
            sy += angle.sin() * dist;
        }
        let size_jitter = if self.tool.scatter > 0.0 {
            0.4 + 0.85 * self.rand()
        } else {
            1.0
        };
        Stamp {
            x: sx,
            y: sy,
            radius: radius * size_jitter,
            water,
            pigment,
            dryness: self.tool.dryness,
        }
    }

    /// Source: src/paint/brush.ts:136–157 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn stroke_to(&mut self, x: f64, y: f64, pressure: f64) -> Vec<Stamp> {
        let mut stamps = Vec::new();
        let dx = x - self.last_x;
        let dy = y - self.last_y;
        let dist = dx.hypot(dy);
        let radius = brush_radius(&self.tool, self.size) * self.radius_scale;
        let spacing = (radius * self.tool.spacing).max(0.9);
        if dist <= 1e-6 {
            return stamps;
        }
        let mut along = spacing - self.carry;
        while along <= dist {
            let t = along / dist;
            stamps.push(self.make_stamp(self.last_x + dx * t, self.last_y + dy * t, pressure));
            along += spacing;
        }
        self.carry = dist - (along - spacing);
        self.last_x = x;
        self.last_y = y;
        stamps
    }

    /// Source: src/paint/brush.ts:160–162 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    pub fn tap_stamp(&mut self, pressure: f64) -> Stamp {
        self.make_stamp(self.last_x, self.last_y, pressure)
    }
}
