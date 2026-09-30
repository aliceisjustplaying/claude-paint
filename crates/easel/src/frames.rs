//! Replay frames for an engine without a hand-time clock (ported from
//! claude-paint 57137d4 `easel run --frames-every`, minimal): `easel run
//! <log> --frames-dir <dir> [--frame-width 1000]` writes the canvas as the
//! engine saves it (a dried clone, as `save`/`look` show it) after every
//! covering verb (`work`, `blend`, `stipple`, `glaze`: the chunk's own
//! sub-steps), at every chunk end and at the finish, plus frames.tsv. This
//! engine has no hand time, so `hand_secs` is the frame's cumulative index,
//! not seconds: equal steps, no invented pacing.
//!
//! The recorder only reads the canvas (it saves a clone): the replay paints
//! what one without frames does.

use paint::Canvas;
use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;

pub struct Recorder {
    dir: PathBuf,
    width: u32,
    pub written: usize,
    pub steps: usize,
    pub chunk: usize,
    index: Option<std::fs::File>,
    pub err: Option<String>,
}

thread_local! {
    static REC: RefCell<Option<Recorder>> = const { RefCell::new(None) };
}

pub fn start(dir: PathBuf, width: u32) -> Result<(), String> {
    if width < 16 {
        return Err(format!("--frame-width {width}: want at least 16 px"));
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut index = std::fs::File::create(dir.join("frames.tsv")).map_err(|e| e.to_string())?;
    writeln!(index, "file\thand_secs\tticks\tchunks_done\tkind").map_err(|e| e.to_string())?;
    let rec = Recorder { dir, width, written: 0, steps: 0, chunk: 0, index: Some(index), err: None };
    REC.with(|r| *r.borrow_mut() = Some(rec));
    Ok(())
}

pub fn stop() -> Option<Recorder> {
    REC.with(|r| r.borrow_mut().take())
}

/// After a covering verb: a sub-step frame.
pub fn step(st: &crate::api::S) {
    if !REC.with(|r| r.borrow().is_some()) {
        return;
    }
    let s = st.borrow();
    if let Some(c) = s.canvas.as_ref() {
        REC.with(|r| {
            if let Some(rec) = r.borrow_mut().as_mut() {
                rec.steps += 1;
                rec.write(c, "step");
            }
        });
    }
}

pub fn chunk_end(c: &Canvas, n: usize) {
    REC.with(|r| {
        if let Some(rec) = r.borrow_mut().as_mut() {
            rec.chunk = n;
            rec.write(c, "chunk");
        }
    });
}

pub fn finish(c: &Canvas) {
    REC.with(|r| {
        if let Some(rec) = r.borrow_mut().as_mut() {
            rec.write(c, "final");
        }
    });
}

impl Recorder {
    fn write(&mut self, c: &Canvas, kind: &str) {
        if self.err.is_some() {
            return;
        }
        if let Err(e) = self.try_write(c, kind) {
            self.err = Some(e);
        }
    }

    fn try_write(&mut self, c: &Canvas, kind: &str) -> Result<(), String> {
        self.written += 1;
        let name = format!("{:05}.png", self.written);
        let p = self.dir.join(&name);
        let mut k = c.clone();
        k.save(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        let img = image::open(&p).map_err(|e| e.to_string())?.to_rgb8();
        if img.width() > self.width {
            let h = ((img.height() as f64 * self.width as f64 / img.width() as f64).round() as u32).max(1);
            image::imageops::thumbnail(&img, self.width, h).save(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        }
        if let Some(ix) = self.index.as_mut() {
            // hand_secs: the cumulative frame index (no hand clock in this engine)
            writeln!(ix, "{name}\t{}\t0\t{}\t{kind}", self.written, self.chunk).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
