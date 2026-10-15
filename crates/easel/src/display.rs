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

    #[cfg(test)]
    fn drain(mut self) {
        self.join();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    struct Directory(PathBuf);

    impl Directory {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!("easel-display-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Directory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn publishes_only_after_both_companions_are_complete() {
        let dir = Directory::new();
        let path = new_look(&dir.0, b"color").unwrap();
        let marker = dir.0.join(".display").join(path.file_name().unwrap());
        let job = Publication::new(vec![(View::default(), path.clone())]).unwrap();
        assert_eq!(std::fs::read(&marker).unwrap(), b"pending\n");
        for preset in crate::look::DISPLAY_PRESETS {
            job.write(&preset, &path, preset.name.as_bytes()).unwrap();
            assert_eq!(std::fs::read(&marker).unwrap(), b"pending\n");
        }
        job.complete().unwrap();
        assert_eq!(std::fs::read(&marker).unwrap(), b"ready\n");
        assert_eq!(std::fs::read(&path).unwrap(), b"color");
        for preset in crate::look::DISPLAY_PRESETS {
            assert_eq!(std::fs::read(dir.0.join(preset.folder).join(path.file_name().unwrap())).unwrap(), preset.name.as_bytes());
        }
        assert!(Publication::new(vec![(View::default(), path)]).is_err());
        assert_eq!(std::fs::read(&marker).unwrap(), b"ready\n");
    }

    #[test]
    fn abandoned_publication_never_advertises_a_partial_pair() {
        let dir = Directory::new();
        let path = new_look(&dir.0, b"color").unwrap();
        let job = Publication::new(vec![(View::default(), path.clone())]).unwrap();
        job.write(&crate::look::DISPLAY_PRESETS[0], &path, b"gallery").unwrap();
        drop(job);
        assert_eq!(std::fs::read(dir.0.join(".display").join(path.file_name().unwrap())).unwrap(), b"failed\n");
        let next = new_look(&dir.0, b"next color").unwrap();
        assert_ne!(path, next);
        assert_eq!(std::fs::read(path).unwrap(), b"color");
    }

    #[test]
    fn a_render_write_failure_leaves_the_previous_complete_revision_intact() {
        let dir = Directory::new();
        let canvas = paint::Canvas::new(32, 1.25, [0.2, 0.3, 0.4]);
        let first = new_look(&dir.0, b"first color").unwrap();
        Publication::new(vec![(View::default(), first.clone())]).unwrap().render(canvas.prepare_optics(), &AtomicBool::new(false)).unwrap();
        let complete = std::fs::read(dir.0.join(".gallery").join(first.file_name().unwrap())).unwrap();
        let second = new_look(&dir.0, b"second color").unwrap();
        std::fs::create_dir(dir.0.join(".relief").join(second.file_name().unwrap())).unwrap();
        let publication = Publication::new(vec![(View::default(), second.clone())]).unwrap();
        let pool = Pool::new(1).unwrap();
        pool.submit(|| {
            let optics = canvas.prepare_optics();
            Ok(Box::new(move |cancel| publication.render(optics, cancel)))
        }).unwrap();
        pool.flush().unwrap();
        assert_eq!(std::fs::read(dir.0.join(".display").join(second.file_name().unwrap())).unwrap(), b"failed\n");
        assert_eq!(std::fs::read(dir.0.join(".display").join(first.file_name().unwrap())).unwrap(), b"ready\n");
        assert_eq!(std::fs::read(dir.0.join(".gallery").join(first.file_name().unwrap())).unwrap(), complete);
    }

    #[test]
    #[cfg(feature = "replay")]
    fn painting_can_advance_while_companions_keep_their_original_revision() {
        let dir = Directory::new();
        let mut session = crate::Session::new(64).unwrap();
        session.run("canvas{size=400, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 1}}, um=100, apply=\"knife\"}}}").unwrap();
        let pool = std::rc::Rc::new(Pool::new(1).unwrap());
        let (finish, wait) = mpsc::channel();
        pool.submit(|| Ok(Box::new(move |_| { wait.recv().unwrap(); Ok(()) }))).unwrap();
        let mut server = crate::Server {
            name: "display-test".into(), build: 0, s: session, frames: false, written: None, replayed: 0, scratch: None, display: Some(pool.clone()),
            #[cfg(feature = "state-store")] committed: None,
            #[cfg(feature = "state-store")] state_failed: false,
        };
        let lit = View { light: Some(crate::GALLERY_LIGHT), ..View::default() };
        let expected = crate::look::render(&server.s.canvas().unwrap(), &lit).unwrap().2;
        let first = dir.0.join("first.png");
        server.look(&[], Some(first.clone())).unwrap();
        assert_eq!(std::fs::read(dir.0.join(".display/first.png")).unwrap(), b"pending\n");
        server.s.run("p = pile{{\"cobalt blue\",1}}; work(rect(0,0,1000,800), {hand=\"broad\", pile=p, coverage=1})").unwrap();
        finish.send(()).unwrap();
        let second = dir.0.join("second.png");
        server.look(&[], Some(second.clone())).unwrap();
        pool.flush().unwrap();
        assert_eq!(std::fs::read(dir.0.join(".gallery/first.png")).unwrap(), expected);
        let now = crate::look::render(&server.s.canvas().unwrap(), &lit).unwrap().2;
        assert_ne!(expected, now);
        assert_eq!(std::fs::read(dir.0.join(".gallery/second.png")).unwrap(), now);
        for path in [first, second] {
            assert_eq!(std::fs::read(dir.0.join(".display").join(path.file_name().unwrap())).unwrap(), b"ready\n");
        }
    }

    #[test]
    #[cfg(feature = "replay")]
    #[ignore = "explicit checkpoint-backed Server::look latency measurement"]
    fn measure_server_look_latency() {
        let checkpoint = std::env::var("EASEL_DISPLAY_BENCH_CHECKPOINT").unwrap();
        let output = PathBuf::from(std::env::var("EASEL_DISPLAY_BENCH_OUTPUT").unwrap());
        std::fs::create_dir_all(&output).unwrap();
        let restored = crate::save::read(Path::new(&checkpoint)).unwrap();
        let mut server = crate::Server {
            name: "display-bench".into(), build: 0, s: restored.session, frames: false, written: None, replayed: restored.chunks, scratch: None,
            display: if enabled() { Some(std::rc::Rc::new(Pool::configured().unwrap())) } else { None },
            #[cfg(feature = "state-store")] committed: None,
            #[cfg(feature = "state-store")] state_failed: false,
        };
        for iteration in 0..5 {
            let start = std::time::Instant::now();
            server.look(&[], Some(output.join(format!("look-{iteration}.png")))).unwrap();
            let reply = start.elapsed().as_secs_f64();
            if let Some(pool) = &server.display { pool.flush().unwrap(); }
            println!("DISPLAY_BENCH\t{}\t{}\t{}\t{reply:.6}\t{:.6}", checkpoint, rayon::current_num_threads(), iteration, start.elapsed().as_secs_f64());
        }
        server.look(&["--crop".into(), "350,200,650,500".into()], Some(output.join("detail.png"))).unwrap();
        if let Some(pool) = &server.display { pool.flush().unwrap(); }
    }

    #[test]
    fn drain_finishes_jobs_in_order() {
        let pool = Pool::new(1).unwrap();
        let (tx, rx) = mpsc::channel();
        for n in 0..2 {
            let tx = tx.clone();
            pool.submit(|| Ok(Box::new(move |cancel| {
                assert!(!cancel.load(Ordering::Acquire));
                tx.send(n).unwrap();
                Ok(())
            }))).unwrap();
        }
        pool.drain();
        assert_eq!(rx.try_iter().collect::<Vec<_>>(), [0, 1]);
    }

    #[test]
    fn bounds_captures_before_they_allocate() {
        let pool = Pool::new(1).unwrap();
        let (finish, wait) = mpsc::channel();
        let (entered, started) = mpsc::channel();
        pool.submit(|| Ok(Box::new(move |_| {
            entered.send(()).unwrap();
            wait.recv().unwrap();
            Ok(())
        }))).unwrap();
        started.recv().unwrap();
        pool.submit(|| Ok(Box::new(|_| Ok(())))).unwrap();
        let (captured, observed) = mpsc::channel();
        let submitter = std::thread::spawn(move || {
            pool.submit(|| {
                captured.send(()).unwrap();
                Ok(Box::new(|_| Ok(())))
            }).unwrap();
            pool.drain();
        });
        assert!(matches!(observed.recv_timeout(Duration::from_millis(50)), Err(mpsc::RecvTimeoutError::Timeout)));
        finish.send(()).unwrap();
        observed.recv_timeout(Duration::from_secs(5)).unwrap();
        submitter.join().unwrap();
    }

    #[test]
    fn dropping_cancels_and_joins_owned_work() {
        let pool = Pool::new(1).unwrap();
        let (entered, started) = mpsc::channel();
        let (done, finished) = mpsc::channel();
        pool.submit(|| Ok(Box::new(move |cancel| {
            entered.send(()).unwrap();
            while !cancel.load(Ordering::Acquire) {
                std::thread::yield_now();
            }
            done.send(()).unwrap();
            Ok(())
        }))).unwrap();
        started.recv().unwrap();
        let (queued, cancelled) = mpsc::channel();
        pool.submit(|| Ok(Box::new(move |cancel| {
            queued.send(cancel.load(Ordering::Acquire)).unwrap();
            Ok(())
        }))).unwrap();
        drop(pool);
        finished.try_recv().unwrap();
        assert!(cancelled.try_recv().unwrap());
    }

    #[test]
    fn errors_and_panics_release_capacity() {
        let pool = Pool::new(1).unwrap();
        assert!(pool.submit(|| Err("capture failed".into())).is_err());
        pool.submit(|| Ok(Box::new(|_| Err("render failed".into())))).unwrap();
        pool.submit(|| Ok(Box::new(|_| panic!("render panic")))).unwrap();
        let (tx, rx) = mpsc::channel();
        pool.submit(|| Ok(Box::new(move |_| { tx.send(()).unwrap(); Ok(()) }))).unwrap();
        pool.drain();
        rx.try_recv().unwrap();
    }
}
