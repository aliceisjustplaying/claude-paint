//! claude-paint: a small procedural painting engine.
//!
//! Coordinates are in "units": the canvas is always 1000 units wide and
//! `1000 / aspect` units tall, regardless of pixel resolution. That keeps a
//! painting program resolution independent (preview and full renders match).
//!
//! Colors are linear-light RGB reflectances in 0..1.

#[cfg(box_conflict)]
compile_error!("paint: more than one box-* feature without all-boxes: a painter's build has one box (box-<name>), the replay build all of them (all-boxes)");

pub mod canvas;
pub mod checkpoint;
pub mod color;
pub mod crack;
pub mod drying;
pub mod mask;
pub mod noise;
pub mod path;
pub mod edge;
pub mod fence;
pub mod form;
pub mod graphite;
pub mod rag;
pub mod scene;
pub mod palette;
pub mod pigment;
pub mod rng;
mod sched;
pub mod shape;
pub mod soak;
pub mod spectral;
pub mod wet;
pub mod bristle;
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
pub use graphite::{Lead, Medium};
pub use rng::Rng;
pub use shape::Shape;

/// The engine version new paintings are painted with. A painting replays
/// with the version it was painted with (its log's `--@ engine` line; a log
/// without one is version 1), so a fix that changes what paint does never
/// changes a past painting:
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
pub const ENGINE: u32 = 5;

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
