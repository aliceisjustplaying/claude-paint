//! Live full-resolution crops (`easel look --crop ... --scale 3.2`).
//!
//! A crop render replays the program on a canvas that holds only a window
//! (`paint::Crop`), which takes about as long as a whole replay at the
//! session's width. So each window is kept: a background thread owns a
//! session painting that window at the asked width and follows the live
//! log chunk by chunk (undoing when the live session undoes). The first
//! look at a new window waits for a full replay; after that a look only
//! waits for the chunks run since the last one.

use crate::session::Session;
use paint::{Canvas, Crop};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Held while a session makes its canvas: `paint::set_crop` is process-wide
/// and read when a canvas is made, so the live session and a crop session
/// must not make theirs at the same time.
pub static CANVAS_LOCK: Mutex<()> = Mutex::new(());

/// Extra units painted around the asked window (so a nearby look reuses
/// the same session) and the margin painted but never shown.
const PAD: f32 = 30.0;
const MARGIN: f32 = 40.0;
/// Crop sessions kept at once.
const KEEP: usize = 2;
/// Undo depth of a crop session (to follow the live session's undos).
const UNDO: usize = 8;

#[derive(Default)]
struct Shared {
    /// The chunks the published canvas has run.
    done: Vec<String>,
    canvas: Option<Canvas>,
    relief: (f32, f32),
    /// Chunks run so far towards the target, and the target's length.
    progress: (usize, usize),
    error: Option<String>,
    /// Seconds spent painting chunks in this window.
    secs: f64,
}

struct Worker {
    width: usize,
    window: [f32; 4],
    tx: Sender<Vec<String>>,
    shared: Arc<(Mutex<Shared>, Condvar)>,
    used: Instant,
}

#[derive(Default)]
pub struct Crops {
    workers: Vec<Worker>,
}

/// What a crop look got: a canvas (maybe behind the live log) and a note.
pub struct Got {
    pub canvas: Canvas,
    pub relief: (f32, f32),
    pub note: String,
}

impl Crops {
    /// Tell every crop session about the live log (after a chunk or undo),
    /// so each catches up in the background.
    pub fn sync(&mut self, log: &[String]) {
        self.workers.retain(|w| w.tx.send(log.to_vec()).is_ok());
    }

    /// The window `crop` (units) painted `width` px wide, following `log`.
    /// Waits up to `wait` s for it to catch up with the log.
    pub fn get(&mut self, width: usize, crop: [f32; 4], canvas_h: f32, log: &[String], wait: f32) -> Result<Got, String> {
        let inside = |w: &Worker| w.width == width && w.window[0] <= crop[0] && w.window[1] <= crop[1] && w.window[2] >= crop[2] && w.window[3] >= crop[3];
        let i = match self.workers.iter().position(inside) {
            Some(i) => i,
            None => {
                let window = [(crop[0] - PAD).max(0.0), (crop[1] - PAD).max(0.0), (crop[2] + PAD).min(1000.0), (crop[3] + PAD).min(canvas_h)];
                if self.workers.len() >= KEEP {
                    let old = (0..self.workers.len()).min_by_key(|&k| self.workers[k].used).unwrap();
                    self.workers.remove(old);
                }
                self.workers.push(spawn(width, window));
                self.workers.len() - 1
            }
        };
        let w = &mut self.workers[i];
        w.used = Instant::now();
        let _ = w.tx.send(log.to_vec());
        let t0 = Instant::now();
        let (lock, cv) = &*w.shared;
        let mut sh = lock.lock().unwrap();
        loop {
            if sh.done.as_slice() == log {
                break;
            }
            if let Some(e) = &sh.error {
                return Err(format!("the {width}px crop session failed: {e}"));
            }
            let left = Duration::from_secs_f32(wait.max(0.0)).saturating_sub(t0.elapsed());
            if left.is_zero() {
                break;
            }
            sh = cv.wait_timeout(sh, left.min(Duration::from_millis(500))).unwrap().0;
        }
        let behind = log.len() as i64 - common(&sh.done, log) as i64;
        let (k, n) = sh.progress;
        let note = if sh.done.as_slice() == log {
            format!("crop at {width}px: current (chunk {}; window {:.0},{:.0},{:.0},{:.0} kept; {:.0}s painting it so far)\n", log.len(), w.window[0], w.window[1], w.window[2], w.window[3], sh.secs)
        } else if sh.canvas.is_some() {
            format!("crop at {width}px: still catching up in the background ({k}/{n} chunks); this look shows chunk {} ({behind} behind). Look again soon, or --wait longer\n", sh.done.len())
        } else {
            return Err(format!("crop at {width}px: painting the window in the background ({k}/{n} chunks, {:.0}s so far; the first look at a window replays the whole log once). Look again in a while, or pass --wait 600", t0.elapsed().as_secs_f64().max(sh.secs)));
        };
        let canvas = sh.canvas.clone().unwrap();
        Ok(Got { canvas, relief: sh.relief, note })
    }
}

fn common(a: &[String], b: &[String]) -> usize {
    a.iter().zip(b).take_while(|(x, y)| x == y).count()
}

fn spawn(width: usize, window: [f32; 4]) -> Worker {
    let (tx, rx) = channel::<Vec<String>>();
    let shared: Arc<(Mutex<Shared>, Condvar)> = Arc::default();
    let sh = shared.clone();
    std::thread::Builder::new()
        .name(format!("crop-{width}"))
        .spawn(move || follow(width, window, rx, sh))
        .expect("spawn a crop thread");
    Worker { width, window, tx, shared, used: Instant::now() }
}

/// The crop thread: follow the latest log it was sent.
fn follow(width: usize, window: [f32; 4], rx: Receiver<Vec<String>>, shared: Arc<(Mutex<Shared>, Condvar)>) {
    let (lock, cv) = &*shared;
    let mut sess: Option<Session> = None;
    let mut ran: Vec<String> = Vec::new();
    let mut target: Vec<String> = match rx.recv() {
        Ok(t) => t,
        Err(_) => return,
    };
    'outer: loop {
        // take the newest log
        loop {
            match rx.try_recv() {
                Ok(t) => target = t,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }
        // step back to what the live log still shares
        let p = common(&ran, &target);
        if p < ran.len() {
            let ok = sess.as_mut().is_some_and(|s| s.undo(ran.len() - p).is_ok());
            if ok {
                ran.truncate(p);
            } else {
                sess = None;
                ran.clear();
            }
        }
        let mut s = match sess.take() {
            Some(s) => s,
            None => match Session::new(width, UNDO) {
                Ok(s) => s,
                Err(e) => {
                    lock.lock().unwrap().error = Some(e.to_string());
                    cv.notify_all();
                    return;
                }
            },
        };
        lock.lock().unwrap().error = None;
        while ran.len() < target.len() {
            let src = target[ran.len()].clone();
            let t0 = Instant::now();
            let r = if s.canvas().is_none() {
                let _g = CANVAS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
                paint::set_crop(Some(Crop { units: window, margin: MARGIN }));
                let r = s.run(&src);
                paint::set_crop(None);
                r
            } else {
                s.run(&src)
            };
            if let Err(e) = r {
                let mut sh = lock.lock().unwrap();
                sh.error = Some(format!("chunk {}: {}", ran.len() + 1, e.lines().last().unwrap_or("")));
                cv.notify_all();
                drop(sh);
                sess = Some(s);
                // wait for a different log
                match rx.recv() {
                    Ok(t) => {
                        target = t;
                        continue 'outer;
                    }
                    Err(_) => return,
                }
            }
            ran.push(src);
            {
                let mut sh = lock.lock().unwrap();
                sh.secs += t0.elapsed().as_secs_f64();
                sh.progress = (ran.len(), target.len());
            }
            // a newer log (an undo, a new chunk) redirects the work
            match rx.try_recv() {
                Ok(t) => {
                    target = t;
                    sess = Some(s);
                    continue 'outer;
                }
                Err(TryRecvError::Disconnected) => return,
                Err(TryRecvError::Empty) => {}
            }
        }
        // publish
        {
            let mut sh = lock.lock().unwrap();
            sh.done = ran.clone();
            sh.canvas = s.canvas().map(|c| c.clone());
            sh.relief = s.st.borrow().style.as_ref().map(|st| st.relief).unwrap_or((0.5, 0.1));
            sh.progress = (ran.len(), target.len());
            cv.notify_all();
        }
        sess = Some(s);
        match rx.recv() {
            Ok(t) => target = t,
            Err(_) => return,
        }
    }
}
