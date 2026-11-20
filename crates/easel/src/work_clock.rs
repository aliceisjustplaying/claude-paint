//! How much computing a live chunk has done, whatever else the machine runs (session.rs
//! `CHUNK_LIMIT`).
//!
//! A chunk's wall time grows with the machine's load: with several builds running, one pastel
//! hatch over a third of a 2400 px sheet took 300-500 s, and chunks that fit on an idle machine
//! were stopped. The CPU time a thread has used grows only while it runs, so it is what the
//! limit counts. A chunk computes on two kinds of thread:
//!
//!   - the session's own thread, which runs the Lua and the engine's serial work;
//!   - the engine's rayon threads (the global pool), which run its parallel work while the
//!     session's thread waits for them.
//!
//! A chunk's work is the session thread's CPU time plus the pool's CPU time spread over the
//! pool's threads: what the chunk would take on the same machine idle, if its parallel work
//! kept every pool thread busy. On an idle machine that is at most the wall time (the two
//! kinds of thread take turns), so a chunk that fit the old wall-clock limit fits this one.
//! Other threads of the process (the display companions' own pool, the request thread) are
//! not counted.

/// The CPU time (s) used so far by the calling thread and, summed, by the engine's pool.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Work {
    pub thread: f64,
    pub pool: f64,
}

impl Work {
    /// Now, read on the thread that runs the chunk.
    pub fn now() -> Work {
        #[cfg(test)]
        if let Some(w) = FAKE.with(|f| f.get()) {
            return w;
        }
        Work { thread: clock(libc::CLOCK_THREAD_CPUTIME_ID), pool: pool_cpu() }
    }

    /// The work done since `start` (s): the thread's CPU time plus the pool's spread over its
    /// threads.
    pub fn since(&self, start: &Work) -> f64 {
        (self.thread - start.thread).max(0.0) + (self.pool - start.pool).max(0.0) / pool_threads() as f64
    }
}

#[cfg(test)]
thread_local! {
    /// A work clock a test sets for its own thread instead of the machine's (the machine's pool
    /// is shared with the tests running beside it).
    pub static FAKE: std::cell::Cell<Option<Work>> = const { std::cell::Cell::new(None) };
}

fn clock(id: libc::clockid_t) -> f64 {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: a valid out-pointer; an unknown clock leaves it zero
    if unsafe { libc::clock_gettime(id, &mut t) } != 0 {
        return 0.0;
    }
    t.tv_sec as f64 + t.tv_nsec as f64 * 1e-9
}

/// The CPU clocks of the global rayon pool's threads, asked of each thread once (the pool's
/// threads live as long as the process).
#[cfg(target_os = "linux")]
fn pool_clocks() -> &'static [libc::clockid_t] {
    static CLOCKS: std::sync::OnceLock<Vec<libc::clockid_t>> = std::sync::OnceLock::new();
    CLOCKS.get_or_init(|| {
        rayon::broadcast(|_| {
            let mut id: libc::clockid_t = 0;
            // SAFETY: the calling thread's own handle and a valid out-pointer
            (unsafe { libc::pthread_getcpuclockid(libc::pthread_self(), &mut id) } == 0).then_some(id)
        })
        .into_iter()
        .flatten()
        .collect()
    })
}

#[cfg(target_os = "linux")]
fn pool_cpu() -> f64 {
    pool_clocks().iter().map(|&id| clock(id)).sum()
}

#[cfg(target_os = "linux")]
fn pool_threads() -> usize {
    pool_clocks().len().max(1)
}

/// Without per-thread clocks of other threads (macOS): the whole process's CPU time but the
/// calling thread's, which also counts the process's few other threads.
#[cfg(not(target_os = "linux"))]
fn pool_cpu() -> f64 {
    (clock(libc::CLOCK_PROCESS_CPUTIME_ID) - clock(libc::CLOCK_THREAD_CPUTIME_ID)).max(0.0)
}

#[cfg(not(target_os = "linux"))]
fn pool_threads() -> usize {
    rayon::current_num_threads().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// A fixed amount of computing: `n` steps of a dependent chain of multiplies, some
    /// nanoseconds each however busy the machine is (a fixed wall time could hold any amount
    /// of computing: on this machine under load 58, 150 ms of spinning was 4 ms on a CPU).
    fn compute(n: u64) {
        let mut x = 1u64;
        for _ in 0..n {
            x = std::hint::black_box(x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407));
        }
        std::hint::black_box(x);
    }

    #[test]
    fn the_threads_own_computing_counts() {
        let a = Work::now();
        // at least 30 ms on any machine this runs on (a multiply's latency is some 3 cycles)
        compute(30_000_000);
        let b = Work::now();
        assert!(b.thread - a.thread > 0.01, "{a:?} {b:?}");
        assert!(b.since(&a) >= b.thread - a.thread);
    }

    #[test]
    fn waiting_is_not_computing() {
        let a = Work::now();
        std::thread::sleep(Duration::from_millis(300));
        let b = Work::now();
        // asleep, as if the machine ran something else instead
        assert!(b.thread - a.thread < 0.05, "{a:?} {b:?}");
    }

    #[test]
    fn the_pools_computing_counts_spread_over_its_threads() {
        let a = Work::now();
        // every pool thread computes while this thread waits
        rayon::broadcast(|_| compute(30_000_000));
        let b = Work::now();
        assert!(b.thread - a.thread < 0.05, "{a:?} {b:?}");
        // other tests share the pool, so only a lower bound
        assert!(b.pool - a.pool > 0.01 * pool_threads() as f64, "{a:?} {b:?}");
        assert!(b.since(&a) > 0.01, "{a:?} {b:?}");
    }
}
