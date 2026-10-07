//! The studio's field inventory. Rollback deliberately keeps the historical
//! boundary: per-chunk counters, RNG, output and timing are reset by `begin`,
//! not restored. Saves are finishing checkpoints, not Lua/held-tool snapshots.

use crate::board::Board;
use paint::{Canvas, Held, Palette, Rng, Style};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Weak ownership follows Lua's lifetime; snapshots temporarily retain live tools.
pub struct HeldTools<T>(Vec<Weak<RefCell<T>>>);
impl<T> Default for HeldTools<T> {
    fn default() -> Self { Self(Vec::new()) }
}
impl<T> HeldTools<T> {
    pub fn push(&mut self, tool: Weak<RefCell<T>>) { self.0.push(tool); }
    pub fn live(&mut self) -> Vec<Rc<RefCell<T>>> {
        self.0.retain(|w| w.strong_count() > 0);
        self.0.iter().filter_map(Weak::upgrade).collect()
    }
}
impl<T: Clone> HeldTools<T> {
    fn snapshot(&mut self) -> Vec<(Rc<RefCell<T>>, T)> {
        self.live().into_iter().map(|t| { let value = t.borrow().clone(); (t, value) }).collect()
    }
}
fn restore_tools<T: Clone>(tools: &[(Rc<RefCell<T>>, T)]) {
    for (tool, value) in tools { *tool.borrow_mut() = value.clone(); }
}

// Each field is declared once, with its rollback group, initial value and
// projection policy. The policy writes digest/dump/save together and reads the
// same save field. A deliberate omission is visible at the field declaration.
// The three ranks at each declaration preserve save, Debug and dump ordering.
macro_rules! inventory {
    ($width:ident, $tubes:ident;
     rollback { $($r:ident: $rt:ty = $ri:expr => $rp:ident @ ($rh:expr, $rd:expr, $rn:expr);)* }
     chunk { $($c:ident: $ct:ty = $ci:expr => $cp:ident @ ($ch:expr, $cd:expr, $cn:expr);)* }
     fixed { $($f:ident: $ft:ty = $fi:expr => $fp:ident @ ($fh:expr, $fd:expr, $fn:expr);)* }
     tools { $($t:ident: $tt:ty => $td:expr;)* }) => {
        pub struct Studio {
            $(pub $r: $rt,)* $(pub $c: $ct,)* $(pub $f: $ft,)*
            $(pub $t: HeldTools<$tt>,)*
        }
        pub struct Snapshot {
            $($r: $rt,)* $( $t: Vec<(Rc<RefCell<$tt>>, $tt)>,)*
        }
        impl Studio {
            pub fn new($width: usize, $tubes: Palette) -> Self {
                Self { $($r: $ri,)* $($c: $ci,)* $($f: $fi,)* $($t: HeldTools::default(),)* }
            }
            pub fn snapshot(&mut self) -> Snapshot {
                Snapshot { $($r: self.$r.clone(),)* $($t: self.$t.snapshot(),)* }
            }
            pub fn restore(&mut self, snap: &Snapshot) {
                $(restore_tools(&snap.$t);)*
                $(self.$r = snap.$r.clone();)*
            }
            fn projection(&mut self, kind: ProjectionKind) -> Projection {
                let mut p = Projection { kind, ..Default::default() };
                p.durable = true;
                $(p.order = ($rh, $rd, $rn); project!($rp, self, p, $r);)*
                p.durable = false;
                $(p.order = ($ch, $cd, $cn); project!($cp, self, p, $c);)*
                $(p.order = ($fh, $fd, $fn); project!($fp, self, p, $f);)*
                if !matches!(kind, ProjectionKind::Header) { $(p.tools.push(($td, stringify!($t), self.$t.live().iter().map(|v| format!("{:?}", v.borrow())).collect()));)* }
                p.header.sort_by_key(|v| v.0);
                p.debug.sort_by_key(|v| v.0);
                p.numbers.sort_by_key(|v| v.0);
                p
            }
            #[cfg(feature = "replay")]
            pub fn read_header(&mut self, h: &Header<'_>, version: u32) -> Result<(), String> {
                $(read!($rp, self, h, version, $r);)*
                $(read!($cp, self, h, version, $c);)*
                $(read!($fp, self, h, version, $f);)*
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
enum ProjectionKind {
    #[default]
    Header,
    #[cfg(any(test, feature = "replay"))]
    Identity,
    #[cfg(feature = "replay")]
    Dump,
}

#[derive(Default)]
struct Projection {
    #[cfg(feature = "replay")]
    canvas_fields: Vec<paint::state_dump::Field>,
    board: String,
    board_default: bool,
    kind: ProjectionKind,
    durable: bool,
    order: (usize, usize, usize),
    #[cfg(any(test, feature = "replay"))]
    canvas: Vec<u8>,
    debug: Vec<(usize, String, String, bool)>,
    numbers: Vec<(usize, paint::state_dump::Field)>,
    header: Vec<(usize, String, String)>,
    tools: Vec<(usize, &'static str, Vec<String>)>,
}
impl Projection {
    fn save(&mut self, name: &str, value: impl ToString) { if matches!(self.kind, ProjectionKind::Header) { self.header.push((self.order.0, name.into(), value.to_string())); } }
    fn debug(&mut self, name: &str, value: impl std::fmt::Debug) { self.debug.push((self.order.1, name.into(), format!("{value:?}"), self.durable)); }
    fn u64(&mut self, name: &'static str, value: u64) {
        self.save(name.strip_prefix("studio.").unwrap(), value);
        self.numbers.push((self.order.2, paint::state_dump::Field { name, shape: vec![1], values: paint::state_dump::Values::U64(vec![value]) }));
    }
    fn clock(&mut self, name: &'static str, value: f64) {
        self.save(name.strip_prefix("studio.").unwrap(), format!("{:016x}", value.to_bits()));
        self.numbers.push((self.order.2, paint::state_dump::Field { name, shape: vec![1], values: paint::state_dump::Values::F64(vec![value]) }));
    }
    #[cfg(any(test, feature = "replay"))]
    fn studio_debug(&self, durable_only: bool) -> String {
        self.debug.iter().filter(|(_, _, _, durable)| !durable_only || *durable).map(|(_, k, v, _)| format!("{k}={v}")).collect::<Vec<_>>().join(" ")
    }
}
macro_rules! project {
    (skip, $s:ident, $p:ident, $f:ident) => {};
    (canvas, $s:ident, $p:ident, $f:ident) => {
        #[cfg(any(test, feature = "replay"))]
        if matches!($p.kind, ProjectionKind::Identity) && let Some(c) = &$s.$f { c.write_state(&mut $p.canvas, "").expect("checkpoint to memory"); }
        #[cfg(feature = "replay")]
        if matches!($p.kind, ProjectionKind::Dump) && let Some(c) = &$s.$f { $p.canvas_fields = c.state_fields(); }
    };
    (integer, $s:ident, $p:ident, $f:ident) => {
        $p.u64(concat!("studio.", stringify!($f)), $s.$f);
        $p.debug(stringify!($f), $s.$f);
    };
    (clock, $s:ident, $p:ident, $f:ident) => {
        $p.clock(concat!("studio.", stringify!($f)), $s.$f);
        $p.debug(stringify!($f), $s.$f);
    };
    (debug, $s:ident, $p:ident, $f:ident) => { $p.debug(stringify!($f), &$s.$f); };
    (setup, $s:ident, $p:ident, $f:ident) => {
        if let Some(v) = &$s.$f { $p.save("setup", v.replace('\n', " ")); }
        $p.debug("setup", &$s.$f);
    };
    (style, $s:ident, $p:ident, $f:ident) => {
        if let Some(v) = &$s.$f {
            $p.save("style", v.name);
            $p.save("relief", format!("{:08x},{:08x}", v.relief.0.to_bits(), v.relief.1.to_bits()));
        }
    };
    (hand, $s:ident, $p:ident, $f:ident) => {
        let piles = &$s.$f.piles.piles;
        $p.save("piles", piles.iter().map(|v| format!("{:08x}{:08x}{:08x}", v[0].to_bits(), v[1].to_bits(), v[2].to_bits())).collect::<Vec<_>>().join(","));
        $p.numbers.push(($p.order.2, paint::state_dump::Field { name: "studio.piles", shape: vec![piles.len(), 3], values: paint::state_dump::Values::F32(piles.iter().flat_map(|v| *v).collect()) }));
        $p.debug("piles", &$s.$f.piles);
    };
    (board, $s:ident, $p:ident, $f:ident) => {
        $p.save("board", $s.$f.save_text());
        $p.board = format!("{:?}", &$s.$f);
        $p.board_default = $s.$f.is_default();
    };
    (width, $s:ident, $p:ident, $f:ident) => { $p.save("width", $s.$f); };
    (tubes, $s:ident, $p:ident, $f:ident) => {
        $p.save("box", $s.$f.name); $p.save("engine", $s.$f.engine);
    };
}
#[cfg(feature = "replay")]
macro_rules! read {
    (skip, $s:ident, $h:ident, $v:ident, $f:ident) => {};
    (canvas, $s:ident, $h:ident, $v:ident, $f:ident) => {};
    (debug, $s:ident, $h:ident, $v:ident, $f:ident) => {};
    (width, $s:ident, $h:ident, $v:ident, $f:ident) => {};
    (tubes, $s:ident, $h:ident, $v:ident, $f:ident) => {};
    (integer, $s:ident, $h:ident, $v:ident, $f:ident) => { $s.$f = crate::save::parse($h, stringify!($f))?; };
    (clock, $s:ident, $h:ident, $v:ident, $f:ident) => { $s.$f = f64::from_bits(crate::save::hex64($h, stringify!($f))?); };
    (setup, $s:ident, $h:ident, $v:ident, $f:ident) => { $s.$f = $h.get("setup").map(|v| v.to_string()); };
    (board, $s:ident, $h:ident, $v:ident, $f:ident) => {
        $s.$f = if $v == 1 { Board::default() } else { Board::read_text(crate::save::field($h, "board")?)? };
    };
    (style, $s:ident, $h:ident, $v:ident, $f:ident) => {
        if $h.contains_key("style") {
            let rel = crate::save::field($h, "relief")?;
            let bits = |v: &str| u32::from_str_radix(v, 16).map(f32::from_bits).map_err(|_| format!("the save's relief={rel:?} isn't two hex floats"));
            let (a, b) = rel.split_once(',').ok_or_else(|| format!("the save's relief={rel:?} isn't two hex floats"))?;
            $s.$f = Some(Rc::new(Style { relief: (bits(a)?, bits(b)?), ..Style::oil_with((*$s.tubes).clone()) }));
        }
    };
    (hand, $s:ident, $h:ident, $v:ident, $f:ident) => {
        $s.$f.piles.piles = crate::save::field($h, "piles")?.split(',').filter(|p| !p.is_empty()).map(|p| {
            let q = |i: usize| p.get(i * 8..i * 8 + 8).and_then(|v| u32::from_str_radix(v, 16).ok()).map(f32::from_bits);
            match (p.len(), q(0), q(1), q(2)) {
                (24, Some(a), Some(b), Some(c)) => Ok([a, b, c]),
                _ => Err(format!("the save's pile {p:?} isn't three hex floats")),
            }
        }).collect::<Result<_, _>>()?;
    };
}

inventory! { width, tubes;
    rollback {
        canvas: Option<Canvas> = None => canvas @ (0, 0, 0);
        style: Option<Rc<Style>> = None => style @ (9, 0, 0);
        setup: Option<String> = None => setup @ (8, 5, 0);
        // Scratch canvas setup text; not a finishing input.
        canvas_src: Option<String> = None => skip @ (0, 0, 0);
        seed: u64 = 1 => integer @ (3, 0, 0);
        clock: f64 = 0.0 => clock @ (6, 1, 3);
        clock0: f64 = 0.0 => clock @ (7, 2, 4);
        hand: crate::time::Hand = Default::default() => hand @ (11, 6, 5);
        board: Board = Default::default() => board @ (12, 8, 0);
        // Last world reference; rollback only, as before.
        view: Option<crate::world::ViewU> = None => skip @ (0, 0, 0);
    }
    chunk {
        chunk: u64 = 0 => integer @ (4, 3, 1);
        calls: u64 = 0 => integer @ (5, 4, 2);
        rng: Rng = Rng::new(1) => debug @ (0, 7, 0);
        out: String = String::new() => skip @ (0, 0, 0);
        field_secs: f64 = 0.0 => skip @ (0, 0, 0);
    }
    fixed {
        width: usize = width => width @ (2, 0, 0);
        tubes: Rc<Palette> = Rc::new(tubes) => tubes @ (0, 0, 0);
    }
    tools { brushes: Held => 0; knives: paint::Knife => 3; rags: paint::rag::Rag => 1; }
}

#[cfg(feature = "replay")]
type Header<'a> = std::collections::BTreeMap<&'a str, &'a str>;

/// Unhashed replay identity, safe to carry to check's worker thread. The
/// contract is the checkpoint plus durable studio and held-tool state.
/// Per-chunk counters and RNG are excluded: `begin` resets them, and a failed
/// chunk historically leaves them changed without changing the painting.
/// Lua, world references and derived canvas caches remain outside it.
#[cfg(any(test, feature = "replay"))]
#[derive(Debug, PartialEq, Eq)]
pub struct StateIdentity { canvas: Vec<u8>, studio: String, board: String, tools: Vec<(usize, &'static str, Vec<String>)> }
#[cfg(any(test, feature = "replay"))]
impl StateIdentity {
    pub fn matches(&self, other: &Self) -> bool { self == other }
}
impl Studio {
    #[cfg(any(test, feature = "replay"))]
    pub fn state_identity(&mut self) -> StateIdentity {
        let p = self.projection(ProjectionKind::Identity);
        StateIdentity { studio: p.studio_debug(true), canvas: p.canvas, board: p.board, tools: p.tools }
    }
    pub fn save_header(&mut self) -> String {
        self.projection(ProjectionKind::Header).header.into_iter().map(|(_, k, v)| format!("{k}={v}\n")).collect()
    }
    #[cfg(feature = "replay")]
    pub fn digest_line(&mut self, n: usize, secs: f64) -> String {
        let p = self.projection(ProjectionKind::Identity);
        let tools = |name| &p.tools.iter().find(|(_, k, _)| *k == name).unwrap().2;
        let brushes = tools("brushes");
        let brushes_h = crate::save::fnv1a(brushes.iter().chain(tools("rags")).cloned().collect::<Vec<_>>().join("\n").as_bytes());
        // Brushes/rags retain their historical combined field; every other
        // held-tool kind gets its own field from the inventory automatically.
        let other_tools: String = p.tools.iter().filter(|(_, name, values)| *name != "brushes" && *name != "rags" && !values.is_empty())
            .map(|(_, name, values)| format!(" {name}={:016x} n{name}={}", crate::save::fnv1a(values.join("\n").as_bytes()), values.len())).collect();
        let board_s = if p.board_default { String::new() } else { format!(" board={:016x}", crate::save::fnv1a(p.board.as_bytes())) };
        let canvas = if p.canvas.is_empty() { 0 } else { crate::save::fnv1a(&p.canvas) };
        format!("chunk {n} secs={secs:.3} canvas={canvas:016x} brushes={brushes_h:016x} nbrushes={} studio={:016x}{other_tools}{board_s}\n", brushes.len(), crate::save::fnv1a(p.studio_debug(false).as_bytes()))
    }
    #[cfg(feature = "replay")]
    pub fn dump_fields(&mut self) -> Vec<(String, &'static str, Vec<usize>, Vec<u8>)> {
        let mut p = self.projection(ProjectionKind::Dump);
        let mut fields = Vec::new();
        let mut num = |f: paint::state_dump::Field| fields.push((f.name.into(), f.values.dtype(), f.shape, f.values.le_bytes()));
        for f in std::mem::take(&mut p.canvas_fields) { num(f); }
        for (_, f) in &p.numbers {
            fields.push((f.name.into(), f.values.dtype(), f.shape.clone(), f.values.le_bytes()));
        }
        let mut text: Vec<_> = p.tools.iter().map(|(order, k, v)| (*order, format!("{k}.debug"), v.join("\n"))).collect();
        text.push((2, String::from("studio.debug"), p.studio_debug(false)));
        text.push((usize::MAX, String::from("studio.board"), p.board));
        text.sort_by_key(|v| v.0);
        for (_, name, text) in text {
            let b = text.into_bytes();
            fields.push((name, "utf8", vec![b.len()], b));
        }
        fields
    }
}

#[cfg(all(test, feature = "replay", tube_box))]
mod tests {
    use crate::session::Session;

    const CANVAS: &str = r#"canvas{size=300, aspect=1, seed=3, linen=15, ground={{pile={{"lead white", 1}}, um=80, apply="knife"}}}"#;

    // Same chunk/call counts and canvas: neither counter nor a pixel comparison
    // can stand in for the board and held-tool parts of the replay identity.
    #[test]
    fn comparison_and_dumps_include_board_and_held_tools() {
        for (a, b, field) in [
            ("palette{dirty=0.2}", "palette{dirty=0.8}", "studio.board"),
            ("k = knife{width=20}", "k = knife{width=40}", "knives.debug"),
            ("b = brush('round', 3)", "b = brush('round', 5)", "brushes.debug"),
            ("r = rag{width=20}", "r = rag{width=40}", "rags.debug"),
        ] {
            let mut left = Session::new(64).unwrap();
            let mut right = Session::new(64).unwrap();
            for s in [&mut left, &mut right] { s.run(CANVAS).unwrap(); }
            left.run(a).unwrap(); right.run(b).unwrap();
            assert!(!left.state_identity().matches(&right.state_identity()), "{field} was invisible to comparison");
            let l = left.st.borrow_mut().dump_fields();
            let r = right.st.borrow_mut().dump_fields();
            let bytes = |fields: Vec<(String, &'static str, Vec<usize>, Vec<u8>)>| fields.into_iter().find(|f| f.0 == field).unwrap().3;
            assert_ne!(bytes(l), bytes(r), "{field} was invisible to the dump");
            let l = left.st.borrow_mut().digest_line(2, 0.0);
            let r = right.st.borrow_mut().digest_line(2, 0.0);
            assert_ne!(l, r, "{field} was invisible to the digest");
        }
    }

    #[test]
    fn a_discarded_chunk_still_matches_its_committed_state() {
        let mut s = Session::new(64).unwrap();
        s.run(CANVAS).unwrap();
        s.run("p = pile{{'lead white', 1}}; b = brush('round', 3); b:load(p)").unwrap();
        let before = s.state_identity();
        let e = s.run("palette{dirty=0.9}; b:stroke({10,10,90,90}); wait(30); error('discard')").unwrap_err();
        assert!(e.contains("discard"));
        assert!(before.matches(&s.state_identity()), "check must not report a discarded chunk's counters as changed painting state");
    }

    // The new persisted contract: a restored finishing checkpoint shows the
    // same dirty palette, including recipes, names, scraped heaps and residue.
    #[test]
    fn save_two_keeps_the_board_and_save_one_defaults_it() {
        let dir = std::env::temp_dir().join(format!("easel-board-save-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("live.ckpt");
        let mut s = Session::new(64).unwrap();
        s.run(CANVAS).unwrap();
        s.run(r#"palette{dirty=0.8, set_out={"lead white", "cobalt blue"}}
            p = pile{{"lead white", 3}, {"cobalt blue", 1}, name="heap\n=λ", medium=0.2, thinner=0.3}
            q = pile{{"cobalt blue", 1}}
            b = brush("flat", 5); b:load(p); b:load(q)
            p:add{{"cobalt blue", 1}}
            for i=1,20 do pile{{"lead white", 1}} end"#).unwrap();
        let board_look = |s: &Session| { let st = s.st.borrow(); crate::palette_look::render(&st.board, &st.tubes) };
        let palette = board_look(&s).unwrap();
        let program = s.program("t");
        assert!(crate::save::write(&s, &program, &path).unwrap());
        let restored = crate::save::read(&path).unwrap();
        assert_eq!(restored.chunks, s.log.len());
        assert_eq!(restored.log_fnv, crate::save::fnv1a(program.as_bytes()));
        assert_eq!(board_look(&restored.session).unwrap(), palette);
        assert_eq!(restored.session.st.borrow_mut().save_header(), s.st.borrow_mut().save_header(), "save fields must keep exact float bits");
        let board_dump = |s: &Session| s.st.borrow_mut().dump_fields().into_iter().find(|f| f.0 == "studio.board").unwrap().3;
        assert_eq!(board_dump(&restored.session), board_dump(&s), "all board fields survive the save, even when not visible in a palette look");
        let (canvas, header) = paint::Canvas::read_state(&mut std::fs::File::open(&path).unwrap()).unwrap();
        assert!(header.starts_with("easel save 2\n"));
        let old = header.lines().filter(|l| !l.starts_with("board=")).collect::<Vec<_>>().join("\n").replacen("easel save 2", "easel save 1", 1) + "\n";
        canvas.write_state(&mut std::fs::File::create(&path).unwrap(), &old).unwrap();
        let mut restored = crate::save::read(&path).unwrap().session;
        let mut fresh = Session::new(64).unwrap(); fresh.run(CANVAS).unwrap();
        assert_eq!(board_dump(&restored), board_dump(&fresh), "v1 has an untouched board");
        assert_eq!(restored.run("print(wait(0))").unwrap().out, s.run("print(wait(0))").unwrap().out, "v1 still restores the hand clock");
        // A v2 file with missing or malformed board data cannot silently default it.
        for board in [None, Some("xyz"), Some("00")] {
            let head = old.replacen("easel save 1", "easel save 2", 1) + &board.map(|b| format!("board={b}\n")).unwrap_or_default();
            canvas.write_state(&mut std::fs::File::create(&path).unwrap(), &head).unwrap();
            assert!(crate::save::read(&path).is_err());
        }
        std::fs::remove_dir_all(dir).unwrap();
    }
}
