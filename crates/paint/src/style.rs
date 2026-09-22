//! Painter style profiles: which tools a painter reaches for and how they
//! handle them. Written from knowledge of the painter, never from images.
//!
//! A `Style` is data: a ground, a tool kit and handling parameters for the
//! kinds of passage a painting is made of (sky, body of a form, detail,
//! line, blending). Paintings ask the style for a `Handling` and supply only
//! geometry and color.

use crate::bristle::{Orient, Tool};
use crate::color::{Rgb, hex};
use crate::handling::Handling;

#[derive(Clone, Debug)]
pub struct Style {
    pub name: &'static str,
    /// Priming layers, bottom first (color, KM thickness of the glaze over the
    /// previous one). The first is laid as an opaque ground.
    pub ground: Vec<(Rgb, f32)>,
    /// Linen thread size and weave strength.
    pub weave: (f32, f32),
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
    /// Paint consistency for body passages (hiding, body).
    pub body_paint: (f32, f32),
    /// Paint consistency for atmospheric passages.
    pub thin_paint: (f32, f32),
    /// Palette-mixing inconsistency per dip (OKLab L, a/b).
    pub jitter: (f32, f32),
    /// Surface relief and gloss for the final lighting.
    pub relief: (f32, f32),
}

impl Style {
    /// Caspar David Friedrich (1774–1840).
    ///
    /// - Fine linen; for *Monk by the Sea* a bright reddish first ground under
    ///   lighter brown priming (CATS technical study,
    ///   https://pure.kb.dk/ws/portalfiles/portal/10104814/CATS_proceedings_III.pdf).
    /// - Underdrawing, underpaint, paint layer, glaze (same source); thin,
    ///   glazing application with sepia-like brown glazes (Conserva 2014-1,
    ///   https://ahnp.ub.uni-heidelberg.de/journals/conserva/article/download/106747/102002).
    /// - Skies fused smooth with soft hair and blending; detail with fine
    ///   sable; very little impasto.
    pub fn friedrich() -> Self {
        Style {
            name: "Caspar David Friedrich",
            ground: vec![(hex("#b0583a"), 0.0), (hex("#c9ad8a"), 5.0), (hex("#d8c7ab"), 3.0)],
            weave: (0.9, 0.35),
            broad: Tool { lay: 0.55, push: 0.03, ragged: 0.4, ..Tool::filbert(22.0) },
            body: Tool { lay: 0.7, push: 0.06, ..Tool::filbert(9.0) },
            detail: Tool::round_sable(2.2),
            line: Tool::rigger(0.6),
            blender: Some(Tool { pickup: 0.15, run: 45.0, ..Tool::badger(40.0) }),
            blend_passes: 3,
            blend_pressure: 0.5,
            body_paint: (0.8, 0.7),
            thin_paint: (0.6, 0.5),
            jitter: (0.012, 0.004),
            relief: (0.2, 0.02),
        }
    }

    /// Broad atmospheric passage (sky, fog, sea): long soft strokes, thin paint.
    pub fn broad<'a>(&self) -> Handling<'a> {
        Handling::new(self.broad.clone())
            .length(80.0, 220.0)
            .coverage(2.5)
            .paint(self.thin_paint.0, self.thin_paint.1)
            .jitter(self.jitter.0, self.jitter.1)
            .pressure(0.55, 0.8)
            .dips(2, 0.8, 0.5)
            .angle_jitter(0.03)
            .ramps(0.12, 0.4)
    }

    /// Building a form in body color.
    pub fn body<'a>(&self) -> Handling<'a> {
        Handling::new(self.body.clone())
            .length(20.0, 60.0)
            .coverage(2.5)
            .paint(self.body_paint.0, self.body_paint.1)
            .jitter(self.jitter.0 * 1.4, self.jitter.1 * 1.4)
            .pressure(0.6, 0.9)
            .dips(2, 0.8, 0.6)
    }

    /// Small forms and edges, cut in precisely.
    pub fn detail<'a>(&self) -> Handling<'a> {
        Handling::new(self.detail.clone())
            .length(4.0, 14.0)
            .coverage(3.0)
            .paint(0.95, 0.8)
            .jitter(self.jitter.0, 0.0)
            .pressure(0.7, 0.95)
            .dips(3, 0.9, 0.8)
            .clip(true)
            .threshold(0.1)
    }

    /// Clean soft blender passes over a wet passage.
    pub fn blend<'a>(&self) -> Option<Handling<'a>> {
        let t = self.blender.clone()?;
        Some(
            Handling::new(t)
                .length(120.0, 300.0)
                .coverage(3.0)
                .pressure(self.blend_pressure * 0.8, self.blend_pressure)
                .dips(3, 0.0, 0.9)
                .blender()
                .angle_jitter(0.02),
        )
    }

    /// A brush held for lines.
    pub fn line_tool(&self, width: f32) -> Tool {
        Tool { width, length: width * 5.0, ..self.line.clone() }
    }

    /// Scumbling handling (not Friedrich's habit, for painters who use it).
    pub fn scumble<'a>(&self) -> Handling<'a> {
        Handling::new(self.body.clone()).scrub(3).paint(0.5, 0.5).length(10.0, 20.0).orient(Orient::Across)
    }
}
