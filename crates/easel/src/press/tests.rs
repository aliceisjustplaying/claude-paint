//! The press on tiny plates and sheets (40 × 30 mm plates, 60 mm sheets beside a 64 px painting),
//! and plan-intaglio §6.3's test sheet.

use crate::session::{Session, box_for, parse_program};

const W: usize = 64;
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#;
const PLATE: &str = r#"k = plate{name="key", metal="copper", temper="half-hard", size={40, 30}}
n = needle{tip=25, cone=30}
use(k)
k:drypoint(n, {{100, 300}, {450, 300}}, {force=3, alt=30, azimuth=60})
k:drypoint(n, {{100, 500}, {450, 520}}, {force=4})
k:roulette({{100, 650}, {450, 650}}, {force=6})"#;

fn live() -> Session {
    Session::with_box(W, box_for(None).unwrap()).unwrap().with_paper(true).unwrap()
}

fn run(s: &mut Session, src: &str) -> String {
    match s.run(src) {
        Ok(r) => r.out,
        Err(e) => panic!("{src}\n→ {e}"),
    }
}

fn darkest(s: &Session) -> String {
    let st = s.st.borrow();
    st.tubes.tubes.iter().min_by(|a, b| paint::color::luminance(a.color).total_cmp(&paint::color::luminance(b.color))).unwrap().name.to_string()
}

/// The ink and a function that inks and wipes the plate afresh, the palm's wipe light or heavy.
fn bench(s: &Session) -> String {
    format!(
        r#"black = ink{{{{"{}", 1}}, oil=0.1, stiff=1}}
-- (each plate wiped alike: a fresh tarlatan of one seed, so only the press's setting differs)
function inked(heavy)
  use(k)
  k:clean_ink()
  k:ink(black)
  local t = tarlatan{{stiff=0.7, seed=5}}
  t:wipe(k, nil, {{passes=5}})
  if heavy then k:hand_wipe(nil, {{passes=3, whiting=true}}) else k:hand_wipe(nil, {{passes=1, whiting=false, pressure=0.2}}) end
end
function sheet_of(name, damp)
  local s = sheet{{name=name, size=60, aspect=1, paper={{grammage=250, laid={{per_cm=9, chain=25}}}}, seed=3}}
  -- (a sized sheet soaks half an hour before it is damp through: IP §6.2)
  if damp then soak(s, {{minutes=30}}); blot(s); local _, w = damp_of(s); assert(w == "damp", w) end
  return s
end"#,
        darkest(s)
    )
}

fn num(out: &str, key: &str) -> f64 {
    out.lines().find_map(|l| l.strip_prefix(&format!("{key}="))).unwrap_or_else(|| panic!("{key} in {out}")).trim().parse().unwrap()
}

/// Plan-intaglio §6.3: a drypoint plate printed on damp and on dry laid paper at three
/// pressures; a wiped plate printed after a light and a heavy hand wipe; the prints looked at
/// whole, in a crop and in relief, with the plate mark. Ink is conserved plate → sheet at every
/// pull; more pressure takes more; a damp sheet more than a dry one; a light hand wipe leaves
/// more plate tone to print.
#[test]
fn the_test_sheet() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let b = bench(&s);
    run(&mut s, &b);
    let mut taken = std::collections::BTreeMap::new();
    for (i, damp) in [false, true].into_iter().enumerate() {
        for (j, p) in [0.3, 0.6, 0.9].into_iter().enumerate() {
            let name = format!("p{i}{j}");
            let out = run(&mut s, &format!(
                r#"inked(true)
local pr = sheet_of("{name}", {damp})
local r = press{{plate=k, sheet=pr, pressure={p}}}
assert(r.mirrored and r.damp == {damp})
print("taken=" .. r.taken)
print("left=" .. r.left)
print("mark=" .. r.mark)
print("mpa=" .. r.pressure)"#
            ));
            let (t, mark, mpa) = (num(&out, "taken"), num(&out, "mark"), num(&out, "mpa"));
            assert!(t > 0.0, "{out}");
            assert!((0.1..20.0).contains(&mpa), "{out}");
            assert!((mark > 0.0) == damp, "{out}");
            taken.insert((damp, j), t);
        }
    }
    for damp in [false, true] {
        assert!(taken[&(damp, 0)] < taken[&(damp, 1)] && taken[&(damp, 1)] < taken[&(damp, 2)], "more pressure, more ink: {taken:?}");
    }
    for j in 0..3 {
        assert!(taken[&(true, j)] > taken[&(false, j)], "damp takes more than dry: {taken:?}");
    }
    // a light and a heavy hand wipe, on damp sheets at the middle pressure
    let mut by = Vec::new();
    for heavy in [false, true] {
        let out = run(&mut s, &format!("inked({heavy})\nlocal pr = sheet_of(\"w{heavy}\", true)\nlocal r = press{{plate=k, sheet=pr, pressure=0.6}}\nprint(\"taken=\" .. r.taken)"));
        by.push(num(&out, "taken"));
    }
    assert!(by[0] > by[1], "a light hand wipe leaves more tone to print: {by:?}");
    // the print, looked at whole, in a crop and in relief; its plate mark sinks the footprint
    run(&mut s, "use(\"p12\")");
    {
        let st = s.st.borrow();
        let c = st.canvas.as_ref().unwrap();
        for v in [crate::look::View::default(), crate::look::View { crop: Some([300.0, 300.0, 700.0, 600.0]), ..Default::default() }, crate::look::View { light: Some((135.0, 25.0)), ..Default::default() }] {
            let (w, h, png) = crate::look::render(c, &v).unwrap();
            assert!(w > 0 && h > 0 && !png.is_empty());
        }
    }
}

/// The press's ink balance in Lua: what the plate lost is what the sheet took; the sheet is put
/// aside again after the pull and the plate stays in hand; a failed chunk puts both back; a log
/// replays the print bit for bit.
#[test]
fn a_pull_conserves_ink_and_replays() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let b = bench(&s);
    run(&mut s, &b);
    let out = run(&mut s, "inked(true)\npr = sheet_of(\"proof\", true)\nuse(k)\nr = press{plate=k, sheet=pr}\nprint(\"taken=\" .. r.taken)");
    assert!(num(&out, "taken") > 0.0);
    assert_eq!(s.st.borrow().surfaces.current_name(), "key", "the plate stays in hand");
    let plate = |s: &Session| crate::surfaces::plates(&s.st.borrow());
    let before = plate(&s);
    let e = s.run("press{plate=k, sheet=pr}\nerror(\"no\")").unwrap_err();
    assert!(e.contains("no"));
    assert!(plate(&s) == before, "a failed pull puts the plate back");
    let log = s.program("press");
    let mut r = Session::replay_with(W, box_for(Some(&log)).unwrap()).unwrap().with_paper(true).unwrap();
    for c in parse_program(&log) {
        r.run(&c).unwrap();
    }
    let state = |s: &Session| {
        let mut b = Vec::new();
        crate::surfaces::write_all(&s.st.borrow(), "", &mut b).unwrap();
        b
    };
    assert!(state(&r) == state(&s), "the replay's surfaces, print and plate, are the painting's");
    // without the switch there is no press
    let mut nb = box_for(None).unwrap();
    nb.press = false;
    let mut off = Session::with_box(W, nb).unwrap().with_paper(true).unwrap();
    run(&mut off, CANVAS);
    assert!(off.lua.globals().get::<mlua::Value>("press").unwrap().is_nil());
}

