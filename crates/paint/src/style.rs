//! Style profiles: a support and ground, a tool kit, a palette and
//! handling presets.
//!
//! A `Style` is data: linen, ground layers, five tools (`broad`, `body`,
//! `detail`, `line`, `blender`), a palette, medium fractions and blending
//! settings. Paintings ask the style for a `Handling` and supply geometry
//! and color.
//!
//! The handling presets (`broad`, `body`, `detail`, `hatch`, `glaze`,
//! `blend`) choose stroke lengths, paths, pressure variation, placement and
//! order. Their fields are public and are listed in each preset below;
//! every one can be changed with the `Handling` builder methods, for
//! example stroke direction (`angle`), crossing (`cross`), bow (`curve`),
//! wander (`drift`) and the order an area is worked in (`order`, `sweep`).

use crate::bristle::{Orient, Tool};
use crate::canvas::Canvas;
use crate::color::{Rgb, hex};
use crate::handling::Handling;
use crate::mask::Mask;
use crate::palette::Palette;
use crate::surface::Linen;

/// Mean thickness (µm) a brushed ground (`brush_ground`) lays for a load of
/// the priming brush: about `BRUSHED_UM_AT_FULL * load^BRUSHED_EXP`
/// (measured: load 0.160 lays 37.8 µm, 0.279 lays 73.7, 0.386 lays 111.3;
/// a fuller brush leaves fewer gaps for the filling dabs to close, and
/// laying off takes a thin film along with it, so the thickness grows a
/// little faster than the load).
const BRUSHED_UM_AT_FULL: f32 = 364.0;
const BRUSHED_EXP: f32 = 1.24;

/// How a ground layer is put on.
#[derive(Clone, Copy, Debug)]
pub enum Apply {
    /// Spread with a priming knife: levels the weave; `texture` 0..1 is the
    /// waviness of the knife.
    Knife { texture: f32 },
    /// Rolled on: a fine, even orange-peel texture.
    Roller,
    /// Brushed with a broad hog brush: spread in crossing strokes whose
    /// direction drifts over the canvas, then laid off with light passes of
    /// the clean brush; fine, broken bristle striations stay (see
    /// `brush_ground`).
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
    /// Tool for large areas of thin paint (`broad`).
    pub broad: Tool,
    /// Tool for body color (`body`).
    pub body: Tool,
    /// Tool for small marks and edges (`detail`, `hatch`).
    pub detail: Tool,
    /// Tool for lines (`line_tool`).
    pub line: Tool,
    /// Tool for softening wet passages (`blend`; None: no blender).
    pub blender: Option<Tool>,
    /// Blending passes over broad areas.
    pub blend_passes: usize,
    /// Pressure used when blending (light = gentle fusing).
    pub blend_pressure: f32,
    /// The tube paints every preset mixes from.
    pub palette: Palette,
    /// Fraction of oil medium in the paint for body color (`body`) and for
    /// thin paint (`broad`).
    pub body_medium: f32,
    pub thin_medium: f32,
    /// How unevenly each pile is mixed (relative sd of proportions).
    pub mix_jitter: f32,
    /// Surface relief and gloss for the final lighting.
    pub relief: (f32, f32),
}

impl Style {
    /// A profile built from notes/research/friedrich_materials.md.
    ///
    /// - Plain linen, 15 × 13 threads/cm (10–16 threads/cm proxy range, §1),
    ///   440 mm wide.
    /// - Ground (§2): two knifed layers (110 µm and 70 µm) that level the
    ///   weave, then a brushed top layer (60 µm) whose striations stay;
    ///   total 240 µm (proxy range ~150–300 µm).
    /// - `Palette::friedrich_1820` (§4).
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
            relief: (0.06, 0.006),
        }
    }

    /// `friedrich` on a 1714 mm canvas of 12 × 11 threads/cm linen, with
    /// `Palette::friedrich_early` and the three-layer ground of
    /// notes/research/friedrich_materials.md §2: bright red, then two light
    /// brown layers, the first two knifed, the third rolled on (a fine
    /// texture) [CATS p.127].
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

    /// A primed canvas: this style's linen and physical width (`width_mm`),
    /// with the ground layers applied bottom first.
    pub fn prepare(&self, width_px: usize, aspect: f32, seed: u64) -> Canvas {
        self.prepare_on(Canvas::new(width_px, aspect, self.raw), seed)
    }

    /// `prepare`, holding only the window `crop` (a crop render), whatever
    /// `set_crop` says.
    pub fn prepare_window(&self, width_px: usize, aspect: f32, seed: u64, crop: Option<crate::canvas::Crop>) -> Canvas {
        self.prepare_on(Canvas::new_window(width_px, aspect, self.raw, crop), seed)
    }

    fn prepare_on(&self, c: Canvas, seed: u64) -> Canvas {
        let mut c = c
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
                    brush_ground(&mut c, g, s);
                    // (`prime` counts its own layers)
                    c.ground_um += g.um;
                }
            }
        }
        c
    }
}

/// A brushed top ground: the paste is first spread with a broad hog brush
/// in crossing strokes whose direction wanders over the canvas, then laid
/// off while wet with light passes of the unloaded brush held low, which
/// skim rather than plough, level the spreading's stroke edges and leave
/// only fine, broken bristle striations.
fn brush_ground(c: &mut Canvas, g: &Ground, s: u64) {
    let all = Mask::from_fn(c.frame(), |_, _| 1.0);
    let hog = Tool { lay: 1.2, ragged: 0.2, ..Tool::hog_flat(40.0) };
    let col = g.color;
    // the stroke direction drifts over the canvas, patch by patch (~90
    // units = 40 mm; up to ±0.7 rad, horizontal on average)
    let turn = move |x: f32, y: f32| 1.4 * (crate::surface::vnoise(x / 90.0, y / 90.0, s ^ 0x9e37) - 0.5);
    // the paste is pushed a little ahead of the bristles, not ploughed
    // into ridges (with a hog's own push, 0.3, it piles 2x as thick at the
    // stroke edges: tall, sharp crests that thin paint drains off)
    let spread = Handling::new(Tool { push: 0.15, ..hog.clone() })
        .color(move |_, _| col)
        .paint(g.hiding, g.stiff)
        .angle(turn)
        .angle_jitter(0.15)
        .cross(0.5)
        .curve(0.04, 0.3)
        .drift(0.25, 300.0)
        .tail(0.1)
        .broken(0.1)
        .swell(0.12)
        .length(100.0, 250.0)
        .coverage(3.5)
        .pressure(0.8, 0.95)
        .dips(1, (g.um / BRUSHED_UM_AT_FULL).powf(1.0 / BRUSHED_EXP).min(1.0), 0.3)
        .jitter(0.004, 0.002)
        .shake(0.15);
    c.work(&all, &spread, s);
    // laying off: the clean brush drawn lightly through the wet paste, held
    // low so it skims the paste and barely pushes it
    let lay_off = Handling::new(Tool { push: 0.03, ..hog })
        .blender()
        .angle(turn)
        .angle_jitter(0.12)
        .cross(0.15)
        .curve(0.06, 0.4)
        .drift(0.3, 250.0)
        .tail(0.2)
        .broken(0.3)
        .swell(0.25)
        .length(120.0, 380.0)
        .coverage(3.5)
        .pressure(0.35, 0.55)
        .dips(2, 0.0, 0.8)
        .shake(0.3);
    c.work(&all, &lay_off, s + 7);
    c.dry();
}

impl Style {
    /// The broad tool with thin paint (`thin_medium`): strokes 80–220 units
    /// long that bow into long arcs (`curve`) and whose direction wanders
    /// over 350-unit patches (`drift`). Set the direction with `angle`; the
    /// default is horizontal.
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

    /// The body tool with body color (`body_medium`): strokes 20–60 units
    /// long, more bowed and broken than `broad`, the direction wandering
    /// over 150-unit patches. Set the direction with `angle`.
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

    /// The detail tool with paint thinned by 0.1 medium: strokes 4–14 units
    /// long, clipped to the mask (`clip`).
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

    /// Short hatched strokes side by side: one family of nearly straight
    /// strokes 5–12 units long, clumped (`clump`), laid area by area. Set
    /// their direction with `angle`, or `cross` them.
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
    /// depth with `load_at`. The color is the glaze paint's masstone (not
    /// aimed); add `.aim(coats)` to aim it at a look instead.
    pub fn glaze(&self, medium: f32) -> Handling<'_> {
        let soft = Tool { stiffness: 0.3, lay: 0.8, pickup: 0.08, ragged: 0.2, ..Tool::filbert(self.broad.width * 1.2) };
        Handling::new(soft)
            .mixed(&self.palette, medium)
            .by_masstone()
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
    ///
    /// The blender stays inside the region it is given (`clip(true)`, soft
    /// where the mask is soft). An unclipped blender drags wet paint across
    /// the mask's edge, where later passages show it in their gaps. To blend
    /// across an edge, give it a mask that spans the edge, or
    /// `.clip(false)`.
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
                .clip(true)
                .sweep(std::f32::consts::FRAC_PI_2),
        )
    }

    /// A brush held for lines.
    pub fn line_tool(&self, width: f32) -> Tool {
        Tool { width, length: width * 5.0, ..self.line.clone() }
    }

    /// Scumbling: the body tool worked back and forth (`scrub(3)`) with
    /// paint thinned by 0.5 medium, strokes 10–20 units long, the brush's
    /// wide axis across its travel (`Orient::Across`).
    pub fn scumble(&self) -> Handling<'_> {
        Handling::new(self.body.clone()).scrub(3).mixed(&self.palette, 0.5).length(10.0, 20.0).orient(Orient::Across)
    }
}
