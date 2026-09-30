//! Frames of a program painting as it runs (replay tooling for the stillwet timelapses; ported to round
//! 10's engine on 2026-09-28). Off unless `PAINT_FRAMES_DIR` is set; then a picture of the canvas as seen
//! (the dry picture with any wet paint on it) is written each time the hand time of the work painted so far
//! crosses the next multiple of `PAINT_FRAMES_EVERY` seconds (default 60), `PAINT_FRAMES_WIDTH` px on the
//! long side (default 1920), plus the finished canvas when the program saves it. `frames.tsv` lists each
//! frame with its hand time, as `scripts/replay_clip` expects.
//!
//! Round 10's programs paint with hand time off: a pass is one batch of tiles. With frames on, a pass runs
//! its tiles in consecutive groups (same order, same stroke ids), so a frame can be taken between them; the
//! recorder only reads the canvas, so the painting comes out exactly as without it.

use crate::Canvas;
use crate::color::linear_to_srgb;
use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;

struct Rec {
    dir: PathBuf,
    long: u32,
    every: f64,
    /// Hand time (s) of the work painted so far.
    done: f64,
    next: f64,
    written: usize,
    index: std::fs::File,
}

thread_local! {
    static REC: RefCell<Option<Option<Rec>>> = const { RefCell::new(None) };
}

fn with<T>(f: impl FnOnce(&mut Rec) -> T) -> Option<T> {
    REC.with(|r| {
        let mut g = r.borrow_mut();
        if g.is_none() {
            *g = Some(from_env());
        }
        g.as_mut().unwrap().as_mut().map(f)
    })
}

fn from_env() -> Option<Rec> {
    let dir = PathBuf::from(std::env::var_os("PAINT_FRAMES_DIR")?);
    let num = |k: &str, d: f64| std::env::var(k).ok().and_then(|v| v.parse::<f64>().ok()).filter(|v| *v > 0.0).unwrap_or(d);
    std::fs::create_dir_all(&dir).expect("PAINT_FRAMES_DIR");
    let mut index = std::fs::File::create(dir.join("frames.tsv")).expect("frames.tsv");
    writeln!(index, "file\thand_secs\tticks\tchunks_done\tkind").unwrap();
    let every = num("PAINT_FRAMES_EVERY", 60.0);
    Some(Rec { dir, long: num("PAINT_FRAMES_WIDTH", 1920.0) as u32, every, done: 0.0, next: every, written: 0, index })
}

/// Whether frames are being recorded.
pub fn on() -> bool {
    with(|_| ()).is_some()
}

/// The seconds between frames (with frames on).
pub(crate) fn every() -> f64 {
    with(|r| r.every).unwrap_or(f64::INFINITY)
}

/// `secs` more hand time of work is on the canvas: write a frame if it crossed the next multiple.
pub(crate) fn painted(c: &Canvas, secs: f64) {
    with(|r| {
        r.done += secs.max(0.0);
        if r.done >= r.next {
            r.next += (((r.done - r.next) / r.every).floor() + 1.0) * r.every;
            r.write(c, "tick");
        }
    });
}

/// The finished canvas (from `Canvas::save`).
pub(crate) fn finish(c: &Canvas) {
    with(|r| r.write(c, "final"));
}

impl Rec {
    fn write(&mut self, c: &Canvas, kind: &str) {
        let (w, h) = (c.f.w, c.f.h);
        let seen = c.seen();
        let mut buf = vec![0u8; w * h * 3];
        for (i, p) in seen.iter().enumerate() {
            for k in 0..3 {
                buf[i * 3 + k] = (linear_to_srgb(p[k]) * 255.0).round().clamp(0.0, 255.0) as u8;
            }
        }
        let img = image::RgbImage::from_raw(w as u32, h as u32, buf).unwrap();
        let s = self.long as f64 / w.max(h) as f64;
        let (fw, fh) = (((w as f64 * s).round() as u32).max(2) & !1, ((h as f64 * s).round() as u32).max(2) & !1);
        let small = image::imageops::resize(&img, fw, fh, image::imageops::FilterType::Triangle);
        self.written += 1;
        let name = format!("{:05}.png", self.written);
        small.save(self.dir.join(&name)).expect("frame");
        writeln!(self.index, "{name}\t{:.1}\t{}\t0\t{kind}", self.done, self.written).unwrap();
    }
}
