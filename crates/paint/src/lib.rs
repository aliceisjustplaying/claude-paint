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
pub mod tally;
pub mod style;
pub mod hand;
pub mod outline;
pub mod surface;

pub use canvas::{Canvas, Crop, Frame, set_crop};
pub use crack::Cracks;
pub use color::{Mix, Rgb, gradient, hex, shift};
pub use bristle::{Gesture, Held, Kind, Orient, Tool, Touch};
pub use wet::Paint;
pub use drying::Stage;
pub use palette::{Mixture, Palette, Tube};
pub use handling::{Handling, Order};
pub use stipple::Stipple;
pub use tally::Tally;
pub use soak::{Fabric, Pour, Poured};
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
pub const ENGINE: u32 = 2;

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
