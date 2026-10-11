// Mechanical port of Watercolor Lab, copyright (c) 2026 Ryan McCarty, MIT.
// Source: uxpreview/watercolor-lab a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See experiments/watercolor/THIRD_PARTY_NOTICES.md for the full MIT license.
pub mod brush;
pub mod km;
pub mod paper;
pub mod pigments;
pub mod simulation;

pub use simulation::{Params, PingPong, Probe, Simulation, DEFAULT_PARAMS};
