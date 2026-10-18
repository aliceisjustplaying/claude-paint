use crate::look::{DisplayPreset, View};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;

type Work = Box<dyn FnOnce(&AtomicBool) -> Result<(), String> + Send>;

pub fn enabled() -> bool {
    std::env::var("EASEL_ASYNC_DISPLAY").is_ok_and(|v| v == "1")
}

pub fn new_look(dir: &Path, png: &[u8]) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    loop {
        let mut random = [0u8; 16];
        std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut random)).map_err(|e| e.to_string())?;
        let name = random.iter().map(|b| format!("{b:02x}")).collect::<String>();
        let path = dir.join(format!("{name}.png"));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(png).and_then(|_| f.sync_all()) {
                    let _ = std::fs::remove_file(&path);
                    return Err(e.to_string());
                }
                return Ok(path);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.to_string()),
        }
    }
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let pending = path.with_extension("display-part");
    let result = (|| {
        let mut f = std::fs::File::create(&pending)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        std::fs::rename(&pending, path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&pending);
    }
    result.map_err(|e| format!("{}: {e}", path.display()))
}

pub struct Publication {
    views: Vec<(View, PathBuf)>,
    pending: Vec<PathBuf>,
}

impl Publication {
    pub fn new(views: Vec<(View, PathBuf)>) -> Result<Self, String> {
        let mut publication = Self { views, pending: Vec::new() };
        for (_, path) in &publication.views {
            let dir = path.parent().ok_or("display path needs a directory")?.join(".display");
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let marker = dir.join(path.file_name().ok_or("display path needs a filename")?);
            let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&marker).map_err(|e| e.to_string())?;
            publication.pending.push(marker);
            file.write_all(b"pending\n").map_err(|e| e.to_string())?;
        }
        Ok(publication)
    }

    fn write(&self, preset: &DisplayPreset, path: &Path, png: &[u8]) -> Result<(), String> {
        let dir = path.parent().ok_or("display path needs a directory")?.join(preset.folder);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        atomic_write(&dir.join(path.file_name().ok_or("display path needs a filename")?), png)
    }

    fn complete(mut self) -> Result<(), String> {
        while let Some(path) = self.pending.last() {
            atomic_write(path, b"ready\n")?;
            self.pending.pop();
        }
        Ok(())
    }

    pub fn render(self, optics: paint::PreparedOptics, cancel: &AtomicBool) -> Result<(), String> {
        for preset in crate::look::DISPLAY_PRESETS {
            if cancel.load(Ordering::Acquire) {
                return Ok(());
            }
            let seen = optics.seen_lit(preset.light.0, preset.light.1, 1.0);
            for (view, path) in &self.views {
                if cancel.load(Ordering::Acquire) {
                    return Ok(());
                }
                let mut lit = view.clone();
                lit.light = Some(preset.light);
                let (_, _, png) = crate::look::render_pixels(optics.frame(), &lit, &seen)?;
                self.write(&preset, path, &png)?;
            }
        }
        if cancel.load(Ordering::Acquire) {
            return Ok(());
        }
        self.complete()
    }
}

impl Drop for Publication {
    fn drop(&mut self) {
        for path in &self.pending {
            let _ = atomic_write(path, b"failed\n");
        }
    }
}

pub struct Pool {
    jobs: Option<mpsc::Sender<Work>>,
    slots: mpsc::Receiver<()>,
    release: mpsc::SyncSender<()>,
    cancel: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl Pool {
    pub fn configured() -> Result<Self, String> {
        let limit = rayon::current_num_threads();
        let threads = match std::env::var("EASEL_DISPLAY_THREADS") {
            Ok(value) => value.parse::<usize>().ok().filter(|n| *n > 0 && *n <= limit).ok_or_else(|| format!("EASEL_DISPLAY_THREADS must be from 1 to {limit}"))?,
            Err(std::env::VarError::NotPresent) => limit.min(4),
            Err(e) => return Err(e.to_string()),
        };
        Self::new(threads)
    }

    pub fn new(threads: usize) -> Result<Self, String> {
        let render = rayon::ThreadPoolBuilder::new().num_threads(threads).thread_name(|n| format!("display-render-{n}")).build().map_err(|e| e.to_string())?;
        let (jobs, incoming) = mpsc::channel::<Work>();
        let (release, slots) = mpsc::sync_channel(2);
        release.send(()).unwrap();
        release.send(()).unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        let stopped = cancel.clone();
        let available = release.clone();
        let worker = std::thread::Builder::new().name("display".into()).spawn(move || {
            for job in incoming {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| render.install(|| job(&stopped))));
                match result {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => eprintln!("warning: display companions: {e}"),
                    Err(_) => eprintln!("warning: display companion worker panicked"),
                }
                let _ = available.send(());
            }
        }).map_err(|e| e.to_string())?;
        Ok(Self { jobs: Some(jobs), slots, release, cancel, worker: Some(worker) })
    }

    pub fn submit(&self, capture: impl FnOnce() -> Result<Work, String>) -> Result<(), String> {
        self.slots.recv().map_err(|e| e.to_string())?;
        let work = match capture() {
            Ok(work) => work,
            Err(e) => {
                let _ = self.release.send(());
                return Err(e);
            }
        };
        self.jobs.as_ref().ok_or("display pool is closed")?.send(work).map_err(|e| e.to_string())
    }

    pub fn flush(&self) -> Result<(), String> {
        let (sent, done) = mpsc::channel();
        self.submit(|| Ok(Box::new(move |_| sent.send(()).map_err(|e| e.to_string()))))?;
        done.recv().map_err(|e| e.to_string())
    }

    fn join(&mut self) {
        self.jobs.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Release);
        self.join();
    }
}
