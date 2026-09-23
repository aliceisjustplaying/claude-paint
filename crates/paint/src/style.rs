//! Painter style profiles: which tools a painter reaches for and how they
//! handle them. Written from knowledge of the painter, never from images.
//!
//! A `Style` is data: a support and ground, a tool kit and handling
//! parameters for the kinds of passage a painting is made of (sky, body of a
//! form, detail, line, blending). Paintings ask the style for a `Handling`
//! and supply geometry and color.
//!
//! The handling presets (`broad`, `body`, `detail`, `hatch`, `glaze`,
//! `blend`) are a vocabulary of techniques with this painter's tools, not
//! finished recipes: each has a hand (strokes arc, wander, break off, press
//! unevenly) but no direction or look of its own. The painter decides which
//! way strokes run (`angle`), whether they criss-cross (`cross`), how much
//! they bow (`curve`), wander (`drift`) and in what order an area is worked
//! (`order`, `sweep`).

use crate::bristle::{Orient, Tool};
use crate::canvas::Canvas;
use crate::color::{Rgb, hex};
use crate::handling::Handling;
use crate::mask::Mask;
use crate::palette::Palette;
use crate::surface::Linen;

/// Mean thickness (µm) a brushed ground lays per unit of load with the
/// priming brush and coverage used in `Style::prepare` (measured: asking 30
/// µm lays 28, asking 90 lays 97).
const BRUSHED_UM_PER_LOAD: f32 = 318.0;

/// How a ground layer is put on.
#[derive(Clone, Copy, Debug)]
pub enum Apply {
    /// Spread with a priming knife: levels the weave; `texture` 0..1 is the
    /// waviness of the knife.
    Knife { texture: f32 },
    /// Rolled on: a fine, even orange-peel texture.
    Roller,
    /// Brushed with a broad hog brush, mostly across the canvas but by hand:
    /// long overlapping strokes that wander and cross a little; the bristle
    /// striations stay.
    Brush,
}

/// One preparation layer.
#[derive(Clone, Copy, Debug)]
pub struct Ground {
    pub color: Rgb,
    pub hiding: f32,
    /// Thickness, µm.
    pub um: f32,
    /// Stiffness of the priming paste (0 fluid .. 1 stiff).
    pub stiff: f32,
    pub apply: Apply,
}

#[derive(Clone, Debug)]
pub struct Style {
    pub name: &'static str,
    /// Physical width of a typical canvas, mm.
    pub width_mm: f32,
    pub linen: Linen,
    /// Color of the raw linen.
    pub raw: Rgb,
    /// Preparation layers, bottom first.
    pub ground: Vec<Ground>,
    /// Tool for large atmospheric areas (sky, fog, water).
    pub broad: Tool,
    /// Tool for building forms (land, rocks, masses).
    pub body: Tool,
    /// Tool for small forms and edges.
    pub detail: Tool,
    /// Tool for lines: twigs, rigging, grasses.
    pub line: Tool,
    /// Tool for softening wet passages (None = the painter leaves strokes).
    pub blender: Option<Tool>,
    /// Blending passes over broad areas.
    pub blend_passes: usize,
    /// Pressure used when blending (light = gentle fusing).
    pub blend_pressure: f32,
    /// The tube paints this painter mixes everything from.
    pub palette: Palette,
    /// Fraction of oil medium in the paint for body passages and for thin,
    /// atmospheric passages.
    pub body_medium: f32,
    pub thin_medium: f32,
    /// How unevenly each pile is mixed (relative sd of proportions).
    pub mix_jitter: f32,
    /// Surface relief and gloss for the final lighting.
    pub relief: (f32, f32),
}

impl Style {
    /// Caspar David Friedrich (1774–1840), Dresden practice around 1820.
    /// Sources: notes/research/friedrich_materials.md.
    ///
    /// - Fine handwoven plain linen bought ready primed [KÖR p.283];
    ///   10–16 threads/cm (proxy, Eckersberg's Dresden canvases [CATS-E]).
    /// - Dresden grounds: 2–4 thin layers; lower ocher / red earth / chalk
    ///   layers "only served to smooth and even out the weave"; the top layer
    ///   brushed, its striations showing through the thin paint [KÖR p.284].
    ///   *Two Men Contemplating the Moon* has a reddish-ocher top ground used
    ///   as the mid-tone [KÖR p.284]. Ground total ~150–300 µm (proxy).
    /// - One or two very thin paint layers over a thin underpainting; skies,
    ///   mist and far hills stippled; paint pools in the ground texture
    ///   [CATS p.127; NG p.56].
    pub fn friedrich() -> Self {
        Style {
            name: "Caspar David Friedrich",
            width_mm: 440.0,
            linen: Linen { warp_per_cm: 15.0, weft_per_cm: 13.0, ..Linen::fine(1) },
            raw: hex("#a8966f"),
            ground: vec![
                Ground { color: hex("#9a5a36"), hiding: 0.8, um: 110.0, stiff: 0.25, apply: Apply::Knife { texture: 0.35 } },
                Ground { color: hex("#b08457"), hiding: 0.8, um: 70.0, stiff: 0.3, apply: Apply::Knife { texture: 0.3 } },
                Ground { color: hex("#a9785a"), hiding: 0.8, um: 60.0, stiff: 0.35, apply: Apply::Brush },
            ],
            broad: Tool { lay: 0.55, push: 0.03, ragged: 0.4, ..Tool::filbert(22.0) },
            body: Tool { lay: 0.7, push: 0.06, ..Tool::filbert(9.0) },
            detail: Tool::round_sable(2.2),
            line: Tool::rigger(0.6),
            blender: Some(Tool { pickup: 0.15, run: 45.0, ..Tool::badger(40.0) }),
            blend_passes: 3,
            blend_pressure: 0.5,
            palette: Palette::friedrich_1820(),
            body_medium: 0.2,
            thin_medium: 0.45,
            mix_jitter: 0.06,
            relief: (0.2, 0.02),
        }
    }

    /// Friedrich's early Berlin grounds (*Monk by the Sea*, *Abbey in the
    /// Oakwood*): bright red, then two light brown layers, the first two put
    /// on with a spatula, the third looking rolled on, with a finely
    /// textured structure he exploited [CATS p.127].
    pub fn friedrich_early() -> Self {
        Style {
            width_mm: 1714.0,
            linen: Linen { warp_per_cm: 12.0, weft_per_cm: 11.0, ..Linen::fine(1) },
            ground: vec![
                Ground { color: hex("#b0583a"), hiding: 0.85, um: 110.0, stiff: 0.25, apply: Apply::Knife { texture: 0.35 } },
                Ground { color: hex("#c9ad8a"), hiding: 0.8, um: 70.0, stiff: 0.3, apply: Apply::Knife { texture: 0.3 } },
                Ground { color: hex("#d8c7ab"), hiding: 0.8, um: 30.0, stiff: 0.5, apply: Apply::Roller },
            ],
            palette: Palette::friedrich_early(),
            ..Self::friedrich()
        }
    }

    /// A primed canvas as this painter bought or made it: linen of the
    /// painter's usual kind and physical size, and the ground layers.
    pub fn prepare(&self, width_px: usize, aspect: f32, seed: u64) -> Canvas {
        let mut c = Canvas::new(width_px, aspect, self.raw)
            .with_size_mm(self.width_mm)
            .with_linen(Linen { seed, ..self.linen });
        for (k, g) in self.ground.iter().enumerate() {
            if g.um <= 0.0 {
                continue;
            }
            let s = seed * 31 + k as u64;
            match g.apply {
                Apply::Knife { texture } => c.prime(g.color, g.hiding, g.um, g.stiff, texture, s),
                Apply::Roller => c.prime(g.color, g.hiding, g.um, g.stiff, 0.8, s),
                Apply::Brush => {
                    // a broad hog brush pulled across in long strokes, band
                    // by band, the arm swinging: the strokes run mostly
                    // across the canvas but wander, bow and cross a little,
                    // and stiff paste keeps the bristle marks
                    let all = Mask::from_fn(c.f, |_, _| 1.0);
                    let hog = Tool { lay: 1.2, ragged: 0.2, ..Tool::hog_flat(40.0) };
                    let col = g.color;
                    let h = Handling::new(hog)
                        .color(move |_, _| col)
                        .paint(g.hiding, g.stiff)
                        .angle(|_, _| 0.0)
                        .angle_jitter(0.04)
                        .curve(0.04, 0.3)
                        .drift(0.25, 450.0)
                        .cross(0.1)
                        .tail(0.1)
                        .broken(0.1)
                        .swell(0.12)
                        .length(250.0, 600.0)
                        .coverage(3.5)
                        .pressure(0.8, 0.95)
                        .dips(1, (g.um / BRUSHED_UM_PER_LOAD).min(1.0), 0.3)
                        .jitter(0.004, 0.002)
                        .shake(0.15);
                    c.work(&all, &h, s);
                    c.dry();
                }
            }
        }
        c
    }

    /// Broad atmospheric passage (sky, fog, sea): long soft strokes of thin
    /// paint swung from the elbow, so they bow into long arcs and their
    /// direction wanders across the passage. Give it a direction (`angle`);
    /// the default is across the canvas.
    pub fn broad(&self) -> Handling<'_> {
        Handling::new(self.broad.clone())
            .length(80.0, 220.0)
            .coverage(2.5)
            .mixed(&self.palette, self.thin_medium)
            .mix_jitter(self.mix_jitter)
            .pressure(0.55, 0.8)
            .dips(2, 0.4, 0.5)
            .angle_jitter(0.06)
            .curve(0.06, 0.25)
            .drift(0.22, 350.0)
            .tail(0.15)
            .broken(0.08)
            .swell(0.18)
            .ramps(0.12, 0.4)
    }

    /// Building a form in body color: shorter strokes from the wrist, more
    /// bowed and broken, the direction wandering from patch to patch. Point
    /// them along the form (`angle`).
    pub fn body(&self) -> Handling<'_> {
        Handling::new(self.body.clone())
            .length(20.0, 60.0)
            .coverage(2.5)
            .mixed(&self.palette, self.body_medium)
            .mix_jitter(self.mix_jitter * 1.4)
            .pressure(0.6, 0.9)
            .dips(2, 0.56, 0.6)
            .angle_jitter(0.12)
            .curve(0.08, 0.3)
            .drift(0.3, 150.0)
            .tail(0.15)
            .broken(0.1)
            .swell(0.22)
    }

    /// Small forms and edges, cut in precisely.
    pub fn detail(&self) -> Handling<'_> {
        Handling::new(self.detail.clone())
            .length(4.0, 14.0)
            .coverage(3.0)
            .mixed(&self.palette, 0.1)
            .mix_jitter(self.mix_jitter)
            .pressure(0.7, 0.95)
            .dips(3, 0.9 * 0.8, 0.8)
            .clip(true)
            .threshold(0.1)
            .curve(0.04, 0.2)
            .drift(0.1, 60.0)
            .tail(0.1)
            .broken(0.0)
            .swell(0.2)
    }

    /// Short hatched strokes side by side, "like a closely woven textile"
    /// [NG pp.49–50]: conifers, grass, the texture of a far slope. One
    /// family of short, nearly straight strokes laid passage by passage; give
    /// them their direction (`angle`), or `cross` them.
    pub fn hatch(&self) -> Handling<'_> {
        Handling::new(Tool { width: self.detail.width * 1.2, ..self.detail.clone() })
            .length(5.0, 12.0)
            .coverage(2.5)
            .mixed(&self.palette, 0.15)
            .mix_jitter(self.mix_jitter)
            .pressure(0.6, 0.9)
            .dips(4, 0.7, 0.7)
            .angle_jitter(0.1)
            .curve(0.03, 0.1)
            .drift(0.15, 40.0)
            .tail(0.08)
            .broken(0.0)
            .swell(0.15)
            .clump(0.5)
            .ramps(0.05, 0.3)
    }

    /// A glaze or thin scumble brushed over dry paint: a soft brush, paint
    /// that is mostly medium (`medium` ≈ 0.85–0.95 for a transparent glaze,
    /// ≈ 0.6 for a veiling scumble), laid thinly in long strokes. Vary the
    /// depth with `load_at`; fuse it afterwards with `blend()`.
    pub fn glaze(&self, medium: f32) -> Handling<'_> {
        let soft = Tool { stiffness: 0.3, lay: 0.8, pickup: 0.08, ragged: 0.2, ..Tool::filbert(self.broad.width * 1.2) };
        Handling::new(soft)
            .mixed(&self.palette, medium)
            .mix_jitter(self.mix_jitter * 0.5)
            .length(120.0, 300.0)
            .coverage(2.5)
            .pressure(0.5, 0.7)
            .dips(2, 0.35, 0.5)
            .angle_jitter(0.05)
            .curve(0.05, 0.25)
            .drift(0.2, 350.0)
            .tail(0.12)
            .broken(0.06)
            .swell(0.18)
            .ramps(0.2, 0.4)
    }

    /// Clean soft blender passes over a wet passage: light crossing strokes
    /// (± 0.2 rad to `angle`), worked top to bottom so the blender never
    /// drags paint back up a gradient (change with `order`/`sweep`).
    pub fn blend(&self) -> Option<Handling<'_>> {
        let t = self.blender.clone()?;
        Some(
            Handling::new(t)
                .length(120.0, 300.0)
                .coverage(3.0)
                .pressure(self.blend_pressure * 0.8, self.blend_pressure)
                .dips(3, 0.0, 0.9)
                .blender()
                .angle_jitter(0.05)
                .cross(0.2)
                .curve(0.06, 0.3)
                .drift(0.2, 300.0)
                .broken(0.0)
                .sweep(std::f32::consts::FRAC_PI_2),
        )
    }

    /// A brush held for lines.
    pub fn line_tool(&self, width: f32) -> Tool {
        Tool { width, length: width * 5.0, ..self.line.clone() }
    }

    /// Scumbling handling (not Friedrich's habit, for painters who use it).
    pub fn scumble(&self) -> Handling<'_> {
        Handling::new(self.body.clone()).scrub(3).mixed(&self.palette, 0.5).length(10.0, 20.0).orient(Orient::Across)
    }
}
