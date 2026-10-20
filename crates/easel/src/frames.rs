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
//!
//! After each chunk the recorder appends its state to `frames.states`, so a
//! replay resumed from a `--replay-checkpoint` picks the frames up where that
//! chunk left them (`start` with `resume`).

use paint::Canvas;
use std::cell::RefCell;
use std::io::{BufRead, Write};
use std::path::PathBuf;

/// The recorder's state after a chunk: one line of `frames.states`.
#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
struct Saved {
    identity: crate::replay_checkpoint::Identity,
    every: f64,
    width: u32,
    light: Option<(f32, f32)>,
    base: Option<f64>,
    next: f64,
    hand: f64,
    written: usize,
    ticks: usize,
    chunk: usize,
}

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
/// time into `dir`, `width` px wide. With `resume` (the chunks a resumed
/// replay has already painted), carry on from the frames those chunks left.
pub fn start(every: f64, dir: PathBuf, width: u32, light: Option<(f32, f32)>, resume: Option<(usize, crate::replay_checkpoint::Identity)>) -> Result<(), String> {
    if !(every.is_finite() && every > 0.0) {
        return Err(format!("--frames-every {every}: want seconds > 0"));
    }
    if width < 16 {
        return Err(format!("--frame-width {width}: want at least 16 px"));
    }
    let rec = match resume {
        None => {
            std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
                let p = entry.map_err(|e| e.to_string())?.path();
                if p.extension().is_some_and(|e| e == "png") && p.file_stem().and_then(|n| n.to_str()).is_some_and(|n| n.parse::<usize>().is_ok()) {
                    std::fs::remove_file(&p).map_err(|e| format!("{}: {e}", p.display()))?;
                }
            }
            let mut index = std::fs::File::create(dir.join("frames.tsv")).map_err(|e| e.to_string())?;
            writeln!(index, "file\thand_secs\tticks\tchunks_done\tkind").map_err(|e| e.to_string())?;
            std::fs::File::create(dir.join("frames.states")).map_err(|e| e.to_string())?;
            Recorder { every, dir, width, light, base: None, next: every, hand: 0.0, written: 0, ticks: 0, chunk: 0, index: Some(index), err: None }
        }
        Some((done, identity)) => resumed(every, dir, width, light, done, &identity)?,
    };
    REC.with(|r| *r.borrow_mut() = Some(rec));
    paint::tally::hand_hook::set(Some(Box::new(|c: &Canvas| observe(c, c.tally().clocked))));
    Ok(())
}

/// The recorder as chunk `done` left it: its `frames.states` line, with
/// `frames.tsv` and the frames cut back to what it had written then.
fn resumed(every: f64, dir: PathBuf, width: u32, light: Option<(f32, f32)>, done: usize, identity: &crate::replay_checkpoint::Identity) -> Result<Recorder, String> {
    let states = dir.join("frames.states");
    let file = std::fs::File::open(&states).map_err(|e| format!("{}: {e} (frames can't resume)", states.display()))?;
    let mut saved = None;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.map_err(|e| e.to_string())?;
        let Ok(s) = serde_json::from_str::<Saved>(&line) else { break };
        if s.chunk == done { saved = Some(s); break; }
    }
    let s = saved.ok_or_else(|| format!("{}: no frames state after chunk {done}; frames can't resume", states.display()))?;
    if &s.identity != identity {
        return Err("replay checkpoint: source, executable, width or engine environment differs; refusing resume".into());
    }
    if s.every != every || s.width != width || s.light != light {
        return Err("frames: --frames-every, --frame-width and the light must match the replay being resumed".into());
    }
    let tsv = dir.join("frames.tsv");
    let text = std::fs::read_to_string(&tsv).map_err(|e| format!("{}: {e}", tsv.display()))?;
    let kept: Vec<&str> = text.lines().take(s.written + 1).collect();
    if kept.len() != s.written + 1 {
        return Err(format!("{}: fewer frames than the state after chunk {done} records", tsv.display()));
    }
    for (i, row) in kept.iter().skip(1).enumerate() {
        let name = format!("{:05}.png", i + 1);
        if row.split('\t').next() != Some(name.as_str()) {
            return Err(format!("{}: fewer frames than the state after chunk {done} records", tsv.display()));
        }
        let path = dir.join(name);
        let metadata = std::fs::metadata(&path).map_err(|e| format!("{}: {e} (frames can't resume)", path.display()))?;
        if !metadata.is_file() {
            return Err(format!("{}: frames can't resume", path.display()));
        }
    }
    replace(&tsv, &(kept.join("\n") + "\n"))?;
    for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let p = entry.map_err(|e| e.to_string())?.path();
        let n = p.file_stem().and_then(|n| n.to_str()).and_then(|n| n.parse::<usize>().ok());
        if p.extension().is_some_and(|e| e == "png") && n.is_some_and(|n| n > s.written) {
            std::fs::remove_file(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        }
    }
    let lines: Vec<String> = std::fs::read_to_string(&states).map_err(|e| e.to_string())?.lines().take_while(|l| serde_json::from_str::<Saved>(l).is_ok_and(|x| x.chunk <= done)).map(|l| l.to_string() + "\n").collect();
    replace(&states, &lines.concat())?;
    let index = std::fs::OpenOptions::new().append(true).open(&tsv).map_err(|e| e.to_string())?;
    Ok(Recorder { every, dir, width, light, base: s.base, next: s.next, hand: s.hand, written: s.written, ticks: s.ticks, chunk: s.chunk, index: Some(index), err: None })
}

/// Stop recording; the recorder as it ended (None if none was running).
pub fn stop() -> Option<Recorder> {
    paint::tally::hand_hook::set(None);
    REC.with(|r| r.borrow_mut().take())
}

pub fn with<R>(recorder: &mut Option<Recorder>, f: impl FnOnce() -> R) -> R {
    let previous = stop();
    REC.with(|r| *r.borrow_mut() = recorder.take());
    paint::tally::hand_hook::set(Some(Box::new(|c: &Canvas| observe(c, c.tally().clocked))));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    *recorder = stop();
    REC.with(|r| *r.borrow_mut() = previous);
    if REC.with(|r| r.borrow().is_some()) {
        paint::tally::hand_hook::set(Some(Box::new(|c: &Canvas| observe(c, c.tally().clocked))));
    }
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
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

/// The end of chunk `n`: its frame (once there is a canvas) and the
/// recorder's state. An error in any frame so far fails the chunk, so a
/// replay checkpoint never gets ahead of the frames.
pub fn chunk_end(c: Option<&Canvas>, n: usize, identity: &crate::replay_checkpoint::Identity) -> Result<(), String> {
    REC.with(|r| {
        let mut g = r.borrow_mut();
        let Some(rec) = g.as_mut() else { return Ok(()) };
        rec.chunk = n;
        if let Some(c) = c {
            rec.write(c, 0, Kind::Chunk);
        }
        if let Some(e) = rec.err.clone() {
            return Err(format!("frames: {e}"));
        }
        rec.save_state(identity).map_err(|e| format!("frames: {e}"))
    })
}

/// Replace `path` with `text` so an interruption leaves the old file or the new one.
fn replace(path: &std::path::Path, text: &str) -> Result<(), String> {
    crate::replay_checkpoint::atomic(path, |f| f.write_all(text.as_bytes()).map_err(|e| e.to_string()))
}

pub fn finish(c: &Canvas) {
    REC.with(|r| {
        if let Some(rec) = r.borrow_mut().as_mut() {
            rec.write(c, 0, Kind::Final);
        }
    });
}

impl Recorder {
    fn save_state(&mut self, identity: &crate::replay_checkpoint::Identity) -> Result<(), String> {
        if let Some(ix) = self.index.as_mut() {
            ix.sync_data().map_err(|e| e.to_string())?;
        }
        let saved = Saved { identity: identity.clone(), every: self.every, width: self.width, light: self.light, base: self.base, next: self.next, hand: self.hand, written: self.written, ticks: self.ticks, chunk: self.chunk };
        let line = serde_json::to_string(&saved).map_err(|e| e.to_string())?;
        let p = self.dir.join("frames.states");
        let mut f = std::fs::OpenOptions::new().append(true).create(true).open(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        writeln!(f, "{line}").and_then(|_| f.sync_data()).map_err(|e| format!("{}: {e}", p.display()))
    }

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
