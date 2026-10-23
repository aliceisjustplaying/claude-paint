//! claude-paint: a small procedural painting engine.
//!
//! Coordinates are in "units": the canvas is always 1000 units wide and
//! `1000 / aspect` units tall, regardless of pixel resolution. That keeps a
//! painting program resolution independent (preview and full renders match).
//!
//! Colors are linear-light RGB reflectances in 0..1.

pub mod canvas;
pub mod checkpoint;
pub mod color;
pub mod crack;
pub mod dry;
pub mod drying;
pub mod mask;
pub mod mixer;
pub mod noise;
pub mod oil;
pub mod path;
pub mod edge;
pub mod fence;
pub mod form;
pub mod graphite;
pub mod rag;
pub mod scene;
pub mod palette;
pub mod paper;
pub mod pastel;
pub mod pigment;
pub mod prepared_optics;
pub mod rng;
#[cfg(feature = "render-snapshot")]
pub mod render_snapshot;
mod sched;
pub mod shape;
pub mod sheet;
pub mod soak;
pub mod spectral;
pub mod wet;
pub mod bristle;
pub mod engine7;
pub mod handling;
pub mod stipple;
pub mod state_dump;
pub mod tally;
pub mod style;
pub mod hand;
pub mod outline;
pub mod surface;
pub mod thinner;

pub use canvas::{Canvas, Crop, Frame, set_crop};
pub use crack::Cracks;
pub use color::{Mix, Rgb, gradient, hex, shift};
pub use bristle::{Gesture, Held, Kind, Knife, Orient, Part, Spatter, Tool, Touch};
pub use wet::Paint;
pub use drying::Stage;
pub use palette::{Mixture, Palette, Tube};
pub use handling::{Handling, Order};
pub use stipple::Stipple;
pub use tally::Tally;
pub use soak::Fabric;
pub use style::{Apply, Ground, Style};
pub use hand::{Hand, Mark};
pub use outline::{Bone, Character, Outline};
pub use surface::{COAT_UM, Linen};
pub use mask::Mask;
pub use form::{Form, Light, Relief, Sdf, Shade, Solid};
pub use scene::{Spot, Sun, View, Water, World};
pub use noise::Fbm;
pub use pigment::Pigment;
pub use prepared_optics::PreparedOptics;
pub use graphite::{Lead, Medium};
pub use rng::Rng;
pub use shape::Shape;

/// The default engine version for low-level canvases and palettes. The easel
/// selects engine 9 for new paintings and records it in the log. A painting replays
/// with the version it was painted with (its log's `--@ engine` line; a log
/// without one is version 1), so a fix that changes what paint does never
/// changes a past painting. Exceptions: all engines now use this crate's
/// pigment mixer, and engine-5 solvent waits were
/// accelerated in October 2026, accepting replay drift for existing logs
/// (notes/thinner/ACCEPTANCE.md, check 17):
/// - 1: every painting before the version was recorded.
/// - 2: fresh paint over drying paint mixes into its cure as it is laid (a
///   stroke right after it feels the film as it would after `wait(0)`); a
///   world's thin far bodies keep their depth (`World::add_body`), and rays
///   from far off (reflections) don't step over them (`World::trace`).
/// - 3: oil paint dries 2.5 times slower and stays open for 60% of its
///   time to touch-dry, not 15% (`drying::Pace`): a brushstroke of lead
///   white is open for 13 h and gels after 27 h (engine 2: 1.4 h and 2.7 h),
///   touch-dry after 45 h (18 h); bone black, cobalt blue, Prussian blue,
///   raw sienna, the cadmiums, the ultramarines, rose madder and permanent
///   alizarin dry at new rates (`Tube::drying_3`, `drying::drier::engine3`), and a box keeps
///   its engine when tubes are added (`Palette::with`);
///   and the easel's `pairs` and `next` walk every table in a fixed order, so
///   a failed chunk is put back without a rebuild from the log (easel
///   session.rs `canonical_tables`);
///   and the easel has a rag (`rag`, crates/easel/src/draw_rag.rs); an older
///   log replays with exactly the globals it had.
/// - 4: stiff paint holds its relief, hairs clump in it, films bridge the
///   weave; solvent, oil, absorbent grounds and gloss (notes/engine-4.md).
/// - 5: knife-laid paint tears where it parts from the blade (`Canvas::knife`).
/// - 7: engine 5, with the parts `engine7`
///   reimplements for speed. A log asks for it with `--@ engine 7`.
/// - 8: engine 7, and a stroke's mark reaches the end of its path
///   (`REACH_END`, `bristle::drag_on`).
/// - 9: engine 8 with the crate's pigment mixer as the new painting default.
///
/// A switch rides on engine 5 and 7 without a number of its own (an easel
/// log's head names it; a log without it replays as it was painted):
/// - the drain switch (`Canvas::with_drain`, `--@ drain`): an absorbent
///   ground drains paint as its pigment allows (`bristle::ground_drain`): to
///   the oil left when the pigment packs, and below it only for pigments
///   coarser than the ground's pores, the packed layer growing up from the
///   ground while the surface keeps its oil, flow and gloss; a rag's smear
///   is drained as a brush's paint is, and blotting stops at the drained
///   floor (notes/research/pigment_oil.md).
pub const ENGINE: u32 = 5;

/// Engine 8: a brush stroke's mark reaches the end of its path. Before it
/// the hairs trailing behind the hand left the canvas that far short of the
/// end, so a stroke drawn up to a point (a limb to where it forks) stopped
/// a few pixels before it.
pub const REACH_END: u32 = 8;

/// Engine 9: engine 8's behavior with the crate's universal pigment mixer.
pub const PIGMENT_MIXER: u32 = 9;

/// The engine versions a log, checkpoint or snapshot may name: `ENGINE` and
/// those before it, engine 7 (`engine7`), engine 8 (`REACH_END`) and engine 9
/// (`PIGMENT_MIXER`). There is no engine 6.
pub const SUPPORTED_ENGINES: [u32; 8] = [1, 2, 3, 4, 5, engine7::ENGINE, REACH_END, PIGMENT_MIXER];

/// Whether engine version `v` is one this crate paints with.
pub fn supported_engine(v: u32) -> bool {
    SUPPORTED_ENGINES.contains(&v)
}

pub fn supported_material_revision(engine: u32, revision: u32) -> bool {
    supported_engine(engine) && (revision == 0 || (engine >= 9 && revision == 1))
}

/// Hermite smoothstep.
#[inline]
pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
#[cfg(test)]
mod tests;
