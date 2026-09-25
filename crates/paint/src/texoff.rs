//! Texture forensics switches (Round 7, notes/round7/texture.md). EXPERIMENT
//! ONLY, default off: nothing changes unless the environment variable
//! `PAINT_TEXOFF` names a switch (comma-separated). Each switch takes one
//! source of pixel-scale texture out of a render, so the variants can be
//! compared on the same passages:
//!
//! - `dither`      8-bit save rounds without the triangular dither
//! - `dither_mono` the dither is one value for all three channels (no chroma)
//! - `relief`      no relief lighting of the height field
//! - `weave`       a flat support (the linen's height is zero)
//! - `ground`      the grounds laid flat (no knife texture, no brushed top ground)
//! - `varnish`     no varnish glaze
//! - `cracks`      no craquelure
//! - `stipple`     stipple passes skipped (the sky keeps its broad layer)
//! - `jitter`      piles mixed the same every time (no mix jitter, no color jitter)
//! - `aim`         no aiming over the underlayer (piles mixed by masstone)
//! - `aimfine`     aimed recipes keyed on a 4x finer grid of the underlayer
//! - `fill`        no look-and-fill dabs
//! - `dipcells`    stipple: no patches, a pile's touches scattered over its passage
//! - `hairsoft`    bristle contacts at least 1.2 px (drag) / 1.5 px (touch) in radius
use std::sync::OnceLock;

static SET: OnceLock<Vec<String>> = OnceLock::new();

/// True if `PAINT_TEXOFF` names `name`.
pub fn off(name: &str) -> bool {
    SET.get_or_init(|| {
        std::env::var("PAINT_TEXOFF").map(|s| s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect()).unwrap_or_default()
    })
    .iter()
    .any(|t| t == name)
}
