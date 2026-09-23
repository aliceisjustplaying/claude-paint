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
pub mod drying;
pub mod mask;
pub mod noise;
pub mod path;
pub mod edge;
pub mod form;
pub mod scene;
pub mod palette;
pub mod pigment;
pub mod rng;
mod sched;
pub mod shape;
pub mod growth;
pub mod wet;
pub mod bristle;
pub mod handling;
pub mod stipple;
pub mod style;
pub mod hand;
pub mod surface;

pub use canvas::{Canvas, Crop, Frame, set_crop};
pub use crack::Cracks;
pub use color::{Mix, Rgb, gradient, hex};
pub use growth::{Clump, Flower, Foliage, Habit, Leafing, Limb, Skeleton, Sward, Tuft, Wind};
pub use bristle::{Gesture, Held, Kind, Orient, Tool, Touch};
pub use wet::Paint;
pub use drying::Stage;
pub use palette::{Mixture, Palette, Tube};
pub use handling::{Aim, Handling, Order};
pub use stipple::Stipple;
pub use style::{Apply, Ground, Style};
pub use hand::{Hand, Mark};
pub use surface::{COAT_UM, Linen};
pub use mask::Mask;
pub use form::{Form, Light, Ridge, Relief, Sdf, Shade, Solid};
pub use scene::{Spot, Sun, View, Water, World};
pub use noise::Fbm;
pub use pigment::Pigment;
pub use rng::Rng;
pub use shape::Shape;

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
