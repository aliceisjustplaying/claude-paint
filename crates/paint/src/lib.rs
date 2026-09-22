//! claude-paint: a small procedural painting engine.
//!
//! Coordinates are in "units": the canvas is always 1000 units wide and
//! `1000 / aspect` units tall, regardless of pixel resolution. That keeps a
//! painting program resolution independent (preview and full renders match).
//!
//! Colors are linear-light RGB reflectances in 0..1.

pub mod brush;
pub mod canvas;
pub mod cli;
pub mod color;
pub mod fill;
pub mod mask;
pub mod noise;
pub mod pigment;
pub mod rng;
pub mod shape;
pub mod tree;
pub mod wet;
pub mod bristle;
pub mod handling;
pub mod style;
pub mod hand;
pub mod surface;

pub use brush::{Brush, Medium};
pub use canvas::{Canvas, Frame};
pub use color::{Mix, Rgb, gradient, hex};
pub use fill::StrokeFill;
pub use tree::Oak;
pub use bristle::{Gesture, Held, Kind, Orient, Tool};
pub use wet::Paint;
pub use handling::Handling;
pub use style::{Apply, Ground, Style};
pub use hand::{Hand, Mark};
pub use surface::{COAT_UM, Linen};
pub use mask::Mask;
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
