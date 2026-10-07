//! Execute a painting log in one place. Callers keep their setup, diagnostics and
//! observations; the driver owns chunk order and the disposable replay interval.

use crate::painting_log::PaintingLog;
use crate::session::{Ran, Session};

#[derive(Debug)]
pub enum Error {
    Session(String),
    Chunk { number: usize, error: String },
    Observer(String),
}

pub trait Observer {
    fn progress(&mut self, _done: usize, _total: usize) {}
    fn before_chunk(&mut self, _number: usize, _total: usize) -> Result<(), String> {
        Ok(())
    }
    fn chunk_end(&mut self, _session: &Session, _number: usize, _ran: &Ran) -> Result<(), String> {
        Ok(())
    }
}

impl Observer for () {}

pub fn replay(
    log: &PaintingLog,
    make_session: impl FnOnce() -> Result<Session, String>,
    replay_after: bool,
    observer: &mut impl Observer,
) -> Result<Session, Error> {
    let total = log.chunks.len();
    observer.progress(0, total);
    let mut session = make_session().map_err(Error::Session)?;
    session.set_replaying(true);
    for (i, chunk) in log.chunks.iter().enumerate() {
        let number = i + 1;
        observer
            .before_chunk(number, total)
            .map_err(Error::Observer)?;
        let ran = session
            .run(chunk)
            .map_err(|error| Error::Chunk { number, error })?;
        observer
            .chunk_end(&session, number, &ran)
            .map_err(Error::Observer)?;
        observer.progress(number, total);
    }
    session.set_replaying(replay_after);
    Ok(session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use paint::Palette;

    #[test]
    fn observers_see_completed_chunks_in_order_and_can_stop_before_the_next() {
        struct Watch {
            events: Vec<String>,
            stop: bool,
        }
        impl Observer for Watch {
            fn progress(&mut self, done: usize, total: usize) {
                self.events.push(format!("progress {done}/{total}"));
            }
            fn before_chunk(&mut self, number: usize, total: usize) -> Result<(), String> {
                if self.stop && number == 2 {
                    return Err(format!("stopped before {number}/{total}"));
                }
                Ok(())
            }
            fn chunk_end(&mut self, s: &Session, number: usize, ran: &Ran) -> Result<(), String> {
                assert_eq!(s.log.len(), number);
                self.events
                    .push(format!("chunk {number}: {}", ran.out.trim_end()));
                Ok(())
            }
        }
        let log =
            PaintingLog::parse("--@ chunk 1\nx = 1; print(x)\n--@ chunk 2\nx = x + 1; print(x)\n");
        let make = || {
            Session::with_box(32, Palette::named_box(Palette::fallback_box()).unwrap())
                .map_err(|e| e.to_string())
        };
        let mut watch = Watch {
            events: Vec::new(),
            stop: false,
        };
        let session = replay(&log, make, false, &mut watch).unwrap();
        assert_eq!(session.lua.globals().get::<i64>("x").unwrap(), 2);
        assert_eq!(
            watch.events,
            [
                "progress 0/2",
                "chunk 1: 1",
                "progress 1/2",
                "chunk 2: 2",
                "progress 2/2"
            ]
        );
        watch.events.clear();
        watch.stop = true;
        assert!(
            matches!(replay(&log, make, true, &mut watch), Err(Error::Observer(e)) if e == "stopped before 2/2")
        );
        assert_eq!(watch.events, ["progress 0/2", "chunk 1: 1", "progress 1/2"]);
    }

    #[test]
    fn replay_ignores_the_live_deadline_then_restores_the_requested_policy() {
        let busy = "x = 1; for i = 1, 3000000 do end; x = 2";
        let log = PaintingLog::parse(&format!("--@ chunk 1\n{busy}\n"));
        let make = || {
            let mut s = Session::with_box(32, Palette::named_box(Palette::fallback_box()).unwrap())
                .map_err(|e| e.to_string())?;
            s.chunk_limit = std::time::Duration::ZERO;
            Ok(s)
        };
        let mut live = replay(&log, make, false, &mut ()).unwrap();
        assert!(live.run(busy).unwrap_err().contains("was stopped"));
        assert_eq!(live.log.len(), 1);
        assert_eq!(live.lua.globals().get::<i64>("x").unwrap(), 2);
        let mut disposable = replay(&log, make, true, &mut ()).unwrap();
        disposable.run(busy).unwrap();
        assert_eq!(disposable.log.len(), 2);
    }
}
