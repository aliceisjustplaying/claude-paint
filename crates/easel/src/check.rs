//! `easel check` off the request thread, and requests whose client is gone.
//!
//! `check` is in the replay build only: the runner checks a painter's log after the
//! painter's last sitting (scripts/check_painting). The painter build keeps `client_gone`.
//!
//! `check` replays the whole log in a fresh session, which takes as long as painting it did
//! (minutes; 630 s for round 18g's 33 chunks on a busy machine). It used to run on the one
//! thread that serves every request, so while it ran nothing else was answered: a painter's
//! `timeout 120 easel check` gave up, its `status` calls hung behind it, and the `do`s it
//! sent meanwhile (their clients timed out too) ran and went into the log after the check
//! was done. Now:
//!
//!   - the replay runs on its own thread, against the log and the live canvas as they were
//!     when the check was asked for, and answers its client when done; the session keeps
//!     serving meanwhile;
//!   - it stops as soon as its client is gone (checked between chunks, and every
//!     `HOOK_EVERY` Lua instructions within one), when a newer `check` replaces it, or when
//!     the session closes;
//!   - the server skips any request whose client is gone before it gets to it (`client_gone`),
//!     so a `do` the painter gave up on doesn't run later behind its back.
//!
//! The check's own thread also gets its own mask garbage collection (api.rs keeps it per
//! thread): on the server thread, the check's throwaway state took it over from the live one.

use crate::session::Session;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Lua instructions between two looks at whether the check should stop.
const HOOK_EVERY: u32 = 1_000_000;
const STOPPED: &str = "check stopped";

#[repr(C)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}
#[cfg(target_os = "linux")]
type NFds = u64;
#[cfg(not(target_os = "linux"))]
type NFds = u32;
unsafe extern "C" {
    fn poll(fds: *mut PollFd, n: NFds, timeout: i32) -> i32;
}
const POLLOUT: i16 = 0x4;
const POLLERR: i16 = 0x8;
const POLLHUP: i16 = 0x10;
const POLLNVAL: i16 = 0x20;

/// Whether the client at the other end of `conn` is gone (it closed, timed out or died), so
/// nobody will read the reply. The server has read the whole request (its length comes
/// first), so the check asks the socket: a socket whose peer has closed reports POLLHUP and,
/// on macOS, is no longer writable. (Clients before the length line shut down their sending
/// side, which macOS also reports as POLLHUP, hence the writability test there.)
pub fn client_gone(conn: &UnixStream) -> bool {
    let mut p = PollFd { fd: conn.as_raw_fd(), events: POLLOUT, revents: 0 };
    if unsafe { poll(&mut p, 1, 0) } < 0 {
        return false;
    }
    if p.revents & (POLLERR | POLLNVAL) != 0 {
        return true;
    }
    if cfg!(target_os = "linux") { p.revents & POLLHUP != 0 } else { p.revents & POLLHUP != 0 && p.revents & POLLOUT == 0 }
}

/// What a check compares: the log's chunks and the live canvas (None: no canvas yet) as
/// bits of what's seen and of the surface.
pub struct Input {
    pub width: usize,
    /// The paper-and-pastel switch (session.rs `PAPER_MARK`).
    pub paper: bool,
    /// The box the session paints from.
    pub tubes: paint::Palette,
    pub chunks: Vec<String>,
    pub live: Option<(Vec<u32>, Vec<u32>)>,
}

/// A check running on its own thread.
pub struct Job {
    stop: Arc<AtomicBool>,
}

impl Job {
    /// Ask the check to stop (it answers its client "check stopped").
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// Start checking `input` on a new thread; it answers `conn` itself.
pub fn start(conn: UnixStream, input: Input) -> Result<Job, String> {
    let stop = Arc::new(AtomicBool::new(false));
    let job = Job { stop: stop.clone() };
    std::thread::Builder::new()
        .name("check".into())
        // the painting's Lua runs here, as deep as on the main thread
        .stack_size(64 << 20)
        .spawn(move || {
            let t0 = Instant::now();
            let n = input.chunks.len();
            let r = replay(&conn, &input, &stop);
            let gone = client_gone(&conn);
            let (reply, how) = match &r {
                Ok(true) => (format!("ok\nreplay matches the live canvas within tolerance ({n} chunks, {:.1}s)\n", t0.elapsed().as_secs_f64()), "ok"),
                Ok(false) => (
                    "err\nreplay DIFFERS from the live canvas beyond tolerance; the log is the painting, so close and reopen to continue from it\n".to_string(),
                    "err",
                ),
                Err(e) => (
                    format!("err\n{e}\n"),
                    if gone {
                        "abandoned (the client went away)"
                    } else if stop.load(Ordering::Relaxed) {
                        "stopped (a newer check, or close)"
                    } else {
                        "err"
                    },
                ),
            };
            if !gone {
                let mut conn = conn;
                let _ = conn.write_all(reply.as_bytes());
            }
            eprintln!("check {:.2}s {how}", t0.elapsed().as_secs_f64());
        })
        .map_err(|e| format!("could not start the check: {e}"))?;
    Ok(job)
}

/// Replay the chunks in a fresh session and compare with the live canvas: Ok(same), or Err
/// if a chunk failed or the check was stopped.
fn replay(conn: &UnixStream, input: &Input, stop: &Arc<AtomicBool>) -> Result<bool, String> {
    let give_up = |stop: &AtomicBool, conn: &UnixStream| stop.load(Ordering::Relaxed) || client_gone(conn);
    let mut fresh = Session::replay_with(input.width, input.tubes.clone()).and_then(|s| s.with_paper(input.paper)).map_err(|e| e.to_string())?;
    {
        let (stop, conn) = (stop.clone(), conn.try_clone().map_err(|e| e.to_string())?);
        fresh
            .lua
            .set_hook(mlua::HookTriggers::new().every_nth_instruction(HOOK_EVERY), move |_, _| {
                if give_up(&stop, &conn) { Err(mlua::Error::runtime(STOPPED)) } else { Ok(mlua::VmState::Continue) }
            })
            .map_err(|e| e.to_string())?;
    }
    for (i, c) in input.chunks.iter().enumerate() {
        if give_up(stop, conn) {
            return Err(format!("{STOPPED} before chunk {} of {}", i + 1, input.chunks.len()));
        }
        fresh.run(c).map_err(|e| {
            if give_up(stop, conn) { format!("{STOPPED} in chunk {} of {}", i + 1, input.chunks.len()) } else { format!("replay failed at chunk {}: {e}", i + 1) }
        })?;
    }
    Ok(match (&input.live, fresh.canvas()) {
        (Some((seen, surface)), Some(b)) => matches(seen, surface, &b.seen(), b.surface_um()),
        (None, None) => true,
        _ => false,
    })
}

fn matches(seen: &[u32], surface: &[u32], replay_seen: &[paint::Rgb], replay_surface: &[f32]) -> bool {
    if seen.len() != replay_seen.len() * 3 || surface.len() != replay_surface.len() || replay_seen.len() != replay_surface.len() {
        return false;
    }
    let channel = |v: f32| (paint::color::linear_to_srgb(v) * 255.0).round() as u8;
    seen.iter().zip(replay_seen.iter().flatten()).all(|(&a, &b)| {
        let a = f32::from_bits(a);
        a.is_finite() && b.is_finite() && channel(a).abs_diff(channel(b)) <= 2
    }) && surface.iter().zip(replay_surface).all(|(&a, &b)| {
        let a = f32::from_bits(a);
        a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-6 + 1e-5 * a.abs()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linear(level: u8) -> f32 {
        paint::color::srgb_to_linear(f32::from(level) / 255.0)
    }

    #[test]
    fn visible_color_accepts_two_srgb_levels_at_every_pixel() {
        let live = crate::bits(&[[linear(100); 3]; 4]);
        let surface = crate::bits_f(&[50.0; 4]);
        for level in [98, 99, 100, 101, 102] {
            assert!(matches(&live, &surface, &[[linear(level); 3]; 4], &[50.0; 4]));
        }
        assert!(matches(&crate::bits(&[[0.0; 3]]), &crate::bits_f(&[0.0]), &[[0.0001; 3]], &[0.0]));
    }

    #[test]
    fn visible_color_rejects_three_srgb_levels_in_any_channel() {
        let live = crate::bits(&[[linear(100); 3]]);
        for channel in 0..3 {
            for level in [97, 103] {
                let mut replay = [[linear(100); 3]];
                replay[0][channel] = linear(level);
                assert!(!matches(&live, &crate::bits_f(&[0.0]), &replay, &[0.0]));
            }
        }
    }

    #[test]
    fn surface_uses_absolute_and_relative_tolerance() {
        let color = [[linear(100); 3]];
        let seen = crate::bits(&color);
        for (live, replay, expected) in [(0.0, 1e-6, true), (0.0, 1.1e-6, false), (100.0, 100.0009, true), (100.0, 100.002, false), (-100.0, -100.0009, true)] {
            assert_eq!(matches(&seen, &crate::bits_f(&[live]), &color, &[replay]), expected, "{live} -> {replay}");
        }
    }

    #[test]
    fn nonfinite_color_and_surface_never_match() {
        let color = [[linear(100); 3]];
        let seen = crate::bits(&color);
        let surface = crate::bits_f(&[0.0]);
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(!matches(&crate::bits(&[[value; 3]]), &surface, &[[value; 3]], &[0.0]));
            assert!(!matches(&seen, &surface, &[[value; 3]], &[0.0]));
            assert!(!matches(&crate::bits(&[[value; 3]]), &surface, &color, &[0.0]));
            assert!(!matches(&seen, &crate::bits_f(&[value]), &color, &[value]));
            assert!(!matches(&seen, &surface, &color, &[value]));
            assert!(!matches(&seen, &crate::bits_f(&[value]), &color, &[0.0]));
        }
    }

    #[test]
    fn color_and_surface_counts_must_match() {
        let color = [[linear(100); 3]];
        let seen = crate::bits(&color);
        let surface = crate::bits_f(&[0.0]);
        assert!(!matches(&seen[..2], &surface, &color, &[0.0]));
        assert!(!matches(&seen, &surface, &color.repeat(2), &[0.0]));
        assert!(!matches(&seen, &surface, &color, &[0.0, 0.0]));
        assert!(!matches(&seen, &[], &color, &[0.0]));
        assert!(!matches(&seen, &surface, &color, &[]));
        assert!(!matches(&[], &surface, &[], &[0.0]));
    }
}
