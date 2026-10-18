//! Hand-time frames (replay build): `easel run --frames-every <s>` writes a
//! downscaled picture of the canvas as seen (wet paint as laid, as
//! `deliver` writes it) each time the hand time crosses the next multiple of
//! the interval, so a clip made from them shows the painting being made at
//! the pace of the hand: broad passes quickly, detail slowly, waits for
//! drying skipped (a wait is not hand time).
//!
//! Hand time is watched in two places: after every verb (`time::verb`: a
//! stroke or touch counts its seconds at once) and after every slice of hand
//! time the engine puts on the clock (`paint::tally::hand_hook`: a long
//! `work` pass clocks 15-minute slices as it goes, so it yields a frame per
//! slice). One observation writes at most one frame however many intervals
//! it crossed, so a slice much longer than the interval is one frame, not
//! hundreds of copies; `frames.tsv` keeps each frame's hand time so the clip
//! can hold it for as long as the hand took.
//!
//! The recorder only reads the canvas: a replay with frames paints exactly
//! what one without them does.

use paint::Canvas;
use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;

pub struct Recorder {
    every: f64,
    dir: PathBuf,
    width: u32,
    /// The light frames are seen in (azimuth, elevation), or None: color only.
    light: Option<(f32, f32)>,
    /// `tally().secs` when hand time started (the ground is not hand time).
    base: Option<f64>,
    /// Hand time (s) of the next frame due.
    next: f64,
    /// The latest hand time seen (s).
    pub hand: f64,
    pub written: usize,
    pub ticks: usize,
    pub chunk: usize,
    index: Option<std::fs::File>,
    pub err: Option<String>,
}

thread_local! {
    static REC: RefCell<Option<Recorder>> = const { RefCell::new(None) };
}

/// What a frame marks.
#[derive(Clone, Copy)]
pub enum Kind {
    /// The hand time crossed the next multiple of the interval.
    Tick,
    /// The end of a chunk.
    Chunk,
    /// The finished canvas.
    Final,
}

/// Start recording on this thread: a frame every `every` seconds of hand
/// time into `dir`, `width` px wide.
pub fn start(every: f64, dir: PathBuf, width: u32, light: Option<(f32, f32)>) -> Result<(), String> {
    if !(every.is_finite() && every > 0.0) {
        return Err(format!("--frames-every {every}: want seconds > 0"));
    }
    if width < 16 {
        return Err(format!("--frame-width {width}: want at least 16 px"));
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut index = std::fs::File::create(dir.join("frames.tsv")).map_err(|e| e.to_string())?;
    writeln!(index, "file\thand_secs\tticks\tchunks_done\tkind").map_err(|e| e.to_string())?;
    let rec = Recorder { every, dir, width, light, base: None, next: every, hand: 0.0, written: 0, ticks: 0, chunk: 0, index: Some(index), err: None };
    REC.with(|r| *r.borrow_mut() = Some(rec));
    paint::tally::hand_hook::set(Some(Box::new(|c: &Canvas| observe(c, c.tally().clocked))));
    Ok(())
}

/// Stop recording; the recorder as it ended (None if none was running).
pub fn stop() -> Option<Recorder> {
    paint::tally::hand_hook::set(None);
    REC.with(|r| r.borrow_mut().take())
}

/// Hand time started on this canvas (`time::start`): count from here.
pub fn begin(c: &Canvas) {
    REC.with(|r| {
        if let Some(rec) = r.borrow_mut().as_mut() {
            rec.base = Some(c.tally().secs);
        }
    });
}

/// After a verb: the hand time counted so far, clocked or not.
pub fn after_verb(c: &Canvas) {
    observe(c, c.tally().secs);
}

/// `secs`: the ledger's hand time (s) at this moment.
fn observe(c: &Canvas, secs: f64) {
    REC.with(|r| {
        let mut g = r.borrow_mut();
        let Some(rec) = g.as_mut() else { return };
        let Some(base) = rec.base else { return };
        let h = (secs - base).max(rec.hand);
        rec.hand = h;
        if h < rec.next {
            return;
        }
        let n = ((h - rec.next) / rec.every).floor() as usize + 1;
        rec.next += n as f64 * rec.every;
        rec.ticks += n;
        rec.write(c, n, Kind::Tick);
    });
}

/// The end of chunk `n` (and with `last`, the finished canvas too).
pub fn chunk_end(c: &Canvas, n: usize) {
    REC.with(|r| {
        if let Some(rec) = r.borrow_mut().as_mut() {
            rec.chunk = n;
            rec.write(c, 0, Kind::Chunk);
        }
    });
}

pub fn finish(c: &Canvas) {
    REC.with(|r| {
        if let Some(rec) = r.borrow_mut().as_mut() {
            rec.write(c, 0, Kind::Final);
        }
    });
}

impl Recorder {
    fn write(&mut self, c: &Canvas, ticks: usize, kind: Kind) {
        if self.err.is_some() {
            return;
        }
        if let Err(e) = self.try_write(c, ticks, kind) {
            self.err = Some(e);
        }
    }

    fn try_write(&mut self, c: &Canvas, ticks: usize, kind: Kind) -> Result<(), String> {
        let f = c.window();
        let px = match self.light {
            Some((az, el)) => c.seen_lit(az, el, 1.0),
            None => c.seen(),
        };
        let buf = crate::output::rgb8(&px, f.w);
        let img = image::RgbImage::from_raw(f.w as u32, f.h as u32, buf).ok_or("frame: bad buffer")?;
        let w = self.width.min(f.w as u32);
        let h = ((f.h as f64 * w as f64 / f.w as f64).round() as u32).max(1);
        let small = if w == f.w as u32 { img } else { crate::output::thumbnail(&img, w, h) };
        self.written += 1;
        let name = format!("{:05}.png", self.written);
        let p = self.dir.join(&name);
        small.save(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        let kind = match kind {
            Kind::Tick => "tick",
            Kind::Chunk => "chunk",
            Kind::Final => "final",
        };
        if let Some(ix) = self.index.as_mut() {
            writeln!(ix, "{name}\t{:.1}\t{ticks}\t{}\t{kind}", self.hand, self.chunk).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
