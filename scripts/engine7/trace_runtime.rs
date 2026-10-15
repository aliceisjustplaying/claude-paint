use crate::api::S;
use std::cell::{Cell, RefCell};
use std::time::Instant;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static KIND: RefCell<String> = const { RefCell::new(String::new()) };
    static PALETTE: Cell<bool> = const { Cell::new(false) };
}

pub struct Event {
    st: S,
    name: String,
    brush: usize,
    chunk: u64,
    start: Instant,
    clock: f64,
    owed: f64,
}

pub fn palette() {
    PALETTE.with(|p| p.set(true));
}

pub fn kind(name: &str) {
    KIND.with(|k| *k.borrow_mut() = name.to_owned());
}

pub fn scope(st: &S, name: &str, brush: usize) -> Option<Event> {
    if ACTIVE.with(|a| a.replace(true)) {
        return None;
    }
    KIND.with(|k| k.borrow_mut().clear());
    PALETTE.with(|p| p.set(false));
    let s = st.borrow();
    let owed = s.canvas.as_ref().map_or(0.0, |c| c.hand_owed_secs());
    eprintln!("E7BEGIN\t{}\t{}", s.chunk, name);
    Some(Event {
        st: st.clone(),
        name: name.to_owned(),
        brush,
        chunk: s.chunk,
        start: Instant::now(),
        clock: s.clock,
        owed,
    })
}

impl Drop for Event {
    fn drop(&mut self) {
        let s = self.st.borrow();
        let owed = s.canvas.as_ref().map_or(0.0, |c| c.hand_owed_secs());
        eprintln!(
            "E7TRACE\t{}\t{}\t{}\t{}\t{}\t{:.9}\t{:.9}\t{:.9}\t{:.9}\t{:.9}",
            self.chunk,
            self.name,
            self.brush,
            KIND.with(|k| k.borrow().clone()),
            PALETTE.with(|p| p.get()),
            self.clock,
            s.clock,
            self.owed,
            owed,
            self.start.elapsed().as_secs_f64()
        );
        ACTIVE.with(|a| a.set(false));
    }
}
