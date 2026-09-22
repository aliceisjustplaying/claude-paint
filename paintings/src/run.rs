//! Running a painting from the command line: options, stage timing, stopping
//! early to look at a stage, and the finishing recipe (varnish, cracks,
//! raking light) paintings share.
//!
//!   cargo paint <name>                       1000px preview → out/<name>.png
//!   cargo paint <name> -- --full             3200px         → out/<name>_full.png
//!   cargo paint <name> -- --width 1600 --seed 7 --out path.png
//!   cargo paint <name> -- --stop sky         save right after the "sky" stage
//!   cargo paint <name> -- --no-cracks        skip craquelure in the finish

use paint::{Canvas, Fbm, Pigment, Rgb, hex};
use std::path::PathBuf;
use std::time::Instant;

pub struct Run {
    pub width: usize,
    pub seed: u64,
    pub out: PathBuf,
    stop: Option<String>,
    no_cracks: bool,
    t0: Instant,
}

impl Run {
    pub fn new(name: &str) -> Self {
        let args: Vec<String> = std::env::args().collect();
        let get = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1));
        let has = |flag: &str| args.iter().any(|a| a == flag);
        let full = has("--full");
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../out");
        let out = match get("--out") {
            Some(p) => p.into(),
            None if full => root.join(format!("{name}_full.png")),
            None => root.join(format!("{name}.png")),
        };
        Run {
            width: get("--width").and_then(|s| s.parse().ok()).unwrap_or(if full { 3200 } else { 1000 }),
            seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
            out,
            stop: get("--stop").cloned(),
            no_cracks: has("--no-cracks"),
            t0: Instant::now(),
        }
    }

    /// A stage of the painting is done: report the time, and if the run was
    /// asked to stop here, save the canvas and return true (the caller then
    /// returns). Saving dries the wet paint, so a run only saves at its end.
    #[must_use]
    pub fn stage(&self, c: &mut Canvas, name: &str) -> bool {
        eprintln!("  {name:<10} {:>6.2}s", self.t0.elapsed().as_secs_f32());
        if self.stop.as_deref() == Some(name) {
            self.save(c);
            return true;
        }
        false
    }

    /// Finish the painting (see `Finish`) and save it.
    pub fn finish(&self, c: &mut Canvas, f: &Finish) {
        c.dry();
        let var = Fbm::new(self.seed as u32 + 98, 3, 400.0);
        let (base, vary) = (f.varnish_coats, f.varnish_vary);
        c.glaze(&Pigment::varnish(f.varnish), None, |x, y| base + vary * var.get(x, y));
        if let (Some((cell, strength)), false) = (f.cracks, self.no_cracks) {
            c.craquelure(cell, strength, self.seed);
        }
        c.relief(f.relief.0, f.relief.1);
        if !self.stage(c, "finish") {
            self.save(c);
        }
    }

    pub fn save(&self, c: &mut Canvas) {
        c.save(&self.out).unwrap();
        let p = self.out.canonicalize().unwrap_or(self.out.clone());
        eprintln!("wrote {} ({}px) in {:.1}s", p.display(), self.width, self.t0.elapsed().as_secs_f32());
    }
}

/// How a painting is finished: aged varnish (a clear, yellowing film,
/// uneven), craquelure, then the surface lit by raking light.
pub struct Finish {
    pub varnish: Rgb,
    /// Varnish thickness in coats, and how much it varies across the canvas.
    pub varnish_coats: f32,
    pub varnish_vary: f32,
    /// Crack cell size (units) and darkening, if the surface has cracked.
    pub cracks: Option<(f32, f32)>,
    /// Relief lighting strength and gloss.
    pub relief: (f32, f32),
}

impl Finish {
    /// An old varnished painting.
    pub fn aged(relief: (f32, f32)) -> Self {
        Finish { varnish: hex("#e6d3a4"), varnish_coats: 0.4, varnish_vary: 0.12, cracks: Some((12.0, 0.12)), relief }
    }
}
