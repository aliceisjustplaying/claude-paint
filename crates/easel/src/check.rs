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

#[cfg(feature = "replay")]
use crate::session::Session;
#[cfg(feature = "replay")]
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
#[cfg(feature = "replay")]
use std::sync::Arc;
#[cfg(feature = "replay")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "replay")]
use std::time::Instant;

#[cfg(feature = "replay")]
/// Lua instructions between two looks at whether the check should stop.
const HOOK_EVERY: u32 = 1_000_000;
#[cfg(feature = "replay")]
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

#[cfg(feature = "replay")]
/// What a check compares: the log's chunks and the live canvas (None: no canvas yet) as
/// bits of what's seen and of the surface.
pub struct Input {
    pub width: usize,
    /// The box the session paints from.
    pub tubes: paint::Palette,
    pub chunks: Vec<String>,
    pub live: Option<(Vec<u32>, Vec<u32>)>,
}

#[cfg(feature = "replay")]
/// A check running on its own thread.
pub struct Job {
    stop: Arc<AtomicBool>,
}

#[cfg(feature = "replay")]
impl Job {
    /// Ask the check to stop (it answers its client "check stopped").
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[cfg(feature = "replay")]
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
                Ok(true) => (format!("ok\nreplay matches the live canvas exactly ({n} chunks, {:.1}s)\n", t0.elapsed().as_secs_f64()), "ok"),
                Ok(false) => (
                    "err\nreplay DIFFERS from the live canvas: a chunk depended on state from a failed chunk (e.g. a table it changed); the log is the painting: close and reopen to continue from it\n".to_string(),
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

#[cfg(feature = "replay")]
/// Replay the chunks in a fresh session and compare with the live canvas: Ok(same), or Err
/// if a chunk failed or the check was stopped.
fn replay(conn: &UnixStream, input: &Input, stop: &Arc<AtomicBool>) -> Result<bool, String> {
    let give_up = |stop: &AtomicBool, conn: &UnixStream| stop.load(Ordering::Relaxed) || client_gone(conn);
    let mut fresh = Session::replay_with(input.width, input.tubes.clone()).map_err(|e| e.to_string())?;
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
        (Some((seen, surface)), Some(b)) => *seen == crate::bits(&b.seen()) && *surface == crate::bits_f(b.surface_um()),
        (None, None) => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::client_gone;
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;

    #[test]
    fn a_client_waiting_for_its_reply_is_there_and_one_that_closed_is_gone() {
        let (mut client, mut server) = UnixStream::pair().unwrap();
        // what a client does: send the request (its length first), wait for the reply
        client.write_all(b"7\nstatus\n").unwrap();
        let mut req = [0u8; 9];
        server.read_exact(&mut req).unwrap();
        assert!(!client_gone(&server), "a waiting client was taken for gone");
        drop(client);
        assert!(client_gone(&server), "a closed client was taken for still there");
    }
}
