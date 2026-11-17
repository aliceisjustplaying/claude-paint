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
  k:clean_ink({{{{0, 100}}, {{1000, 100}}, {{1000, 375}}, {{0, 375}}, {{0, 650}}, {{1000, 650}}}}, {{passes=3}})
  k:ink(black, {{{{0, 375}}, {{1000, 375}}}})
  local t = tarlatan{{stiff=0.7, seed=5}}
  t:wipe(k, {{{{0, 375}}, {{1000, 375}}}}, {{passes=5}})
  if heavy then k:hand_wipe({{{{0, 375}}, {{1000, 375}}}}, {{passes=3, whiting=true}}) else k:hand_wipe({{{{0, 375}}, {{1000, 375}}}}, {{passes=1, whiting=false, pressure=0.2}}) end
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
/// pull. Where the sheet touches the ink it takes its split share, and only where it touches
/// (IP §6.4): so more pressure never takes less, and takes more only while the sheet's press-in
/// falls short of the ink in some opening. The dry sheet, stiff, falls short of the deeper
/// lines' ink at every pressure here, and takes more as it is pressed harder. The damp sheet,
/// soft, is pressed nearly to the ink in every line already at a third of the press's rating:
/// it takes nearly its whole split share of the plate's ink (`SPLIT_DAMP`) and never more, so
/// more pressure gains it almost nothing. (Most of this plate's ink is in its deep lines, the
/// lee's film being wiped under the contact laws, and the sheet reaches 96–98 % of their width
/// at 0.3–0.9: above 95 % of the split share. While a wide band of film beside each burr, which
/// the sheet always touches, held most of the ink, it was above 97 %.) It takes more than the dry sheet at every pressure. A
/// light hand wipe leaves more plate tone to print.
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
            if damp {
                // never more than the split share of all the plate's ink, and nearly all of it
                let all = t + num(&out, "left");
                let most = paint::plate::press::SPLIT_DAMP * all;
                assert!(t <= most * (1.0 + 1e-9) && t > 0.95 * most, "damp at {p}: {t} of its split share {most}: {out}");
            }
            taken.insert((damp, j), t);
        }
    }
    for damp in [false, true] {
        assert!(taken[&(damp, 0)] <= taken[&(damp, 1)] && taken[&(damp, 1)] <= taken[&(damp, 2)], "more pressure never takes less: {taken:?}");
    }
    assert!(taken[&(false, 0)] < taken[&(false, 1)] && taken[&(false, 1)] < taken[&(false, 2)], "the dry sheet reaches further into the lines pressed harder: {taken:?}");
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


/// An engine-9 painting's head, as a log names its switches (`session::box_for`): the
/// session's box is made from it as a replay's is.
const HEAD9: &str = "-- easel session \"inks\"\n--@ box impressionist\n--@ engine 9\n--@ palette-carried-paint\n--@ glaze-film\n--@ oil-k\n--@ drain\n--@ oils\n--@ paper\n--@ paper-presets\n--@ surfaces\n--@ plate\n--@ etch\n--@ damp-paper\n--@ press\n";

fn live9() -> Session {
    let b = box_for(Some(HEAD9)).unwrap();
    assert!(b.engine == paint::PIGMENT_MIXER && b.glaze_film && b.carried_paint && b.oils && b.press);
    Session::replay_with(W, b).unwrap().with_paper(true).unwrap()
}

/// An ink prints as the oil paint a brush lays from its tubes: the pile of the same parts
/// (whatever they sum to: shares by volume) thinned with its plate oil as medium.
#[test]
fn an_ink_prints_as_the_pile_of_its_tubes_in_its_plate_oil() {
    let mut s = live9();
    run(&mut s, r#"canvas{size=100, aspect=1, seed=3, paper={grammage=250}}"#);
    let tubes = s.st.borrow().tubes.clone();
    for (recipe, given) in [
        (r#"{"vermilion", 1}"#, vec![("vermilion", 1.0)]),
        (r#"{"lead white", 1}, {"rose madder", 1}"#, vec![("lead white", 1.0), ("rose madder", 1.0)]),
        (r#"{"Prussian blue", 1}, {"lead white", 0.3}"#, vec![("Prussian blue", 1.0), ("lead white", 0.3)]),
    ] {
        run(&mut s, &format!("p = pile{{{recipe}, medium=0.1}}"));
        let brush = s.lua.globals().get::<mlua::AnyUserData>("p").unwrap().borrow::<crate::api::PileU>().unwrap().paint();
        let parts: Vec<(String, f32)> = given.iter().map(|&(n, p)| (n.to_string(), p)).collect();
        let ink = paint::plate::inked::Ink::of_tubes(&tubes, &parts, 0.1, 1.0, "i".into()).unwrap();
        let printed = ink.paint(&tubes).unwrap();
        let lat = |p: &paint::Paint| p.latent().color;
        let close = |a: f32, b: f32| (a - b).abs() <= 1e-5 * a.abs().max(b.abs()).max(1e-6);
        assert!(lat(&printed).iter().zip(lat(&brush).iter()).all(|(a, b)| close(*a, *b)), "{recipe}: the pigment");
        assert!(printed.prop().iter().zip(brush.prop().iter()).all(|(a, b)| close(*a, *b)), "{recipe}: {:?} against the brush's {:?}", printed.prop(), brush.prop());
        assert_eq!(printed.color, brush.color, "{recipe}");
    }
}

/// Several colours on one plate, three here: an aquatint, each ink laid on its own stripe by a
/// card dragged down it, each stripe wiped in small circles within it with one tarlatan
/// refolded between, pressed on damp paper (engine 9). Each stripe
/// prints its own ink: the cloth lays back what it carries where it is, the last few cm of
/// its slide (`rag::SMEAR`), not the plate's inks mixed.
#[test]
fn a_plate_inked_in_stripes_prints_each_its_own_ink() {
    let mut s = live9();
    run(&mut s, r#"canvas{size=200, aspect=4/3, seed=11, paper={grammage=250, tone={{"lead white", 6}, {"yellow ochre", 0.15}}}}"#);
    run(&mut s, r#"inks = {ink{{"vermilion", 1}, name="v"}, ink{{"Prussian blue", 1}, name="pb"}, ink{{"chrome yellow", 1}, name="cy"}}
c = plate{name="col", metal="copper", size={150, 60}}
use(c)
c:aquatint{box=0.6, wait=30, minutes=1}
c:melt()
c:bite{acid="ferric", baume=38, minutes=3}
c:clean({{0, 40}, {1000, 40}, {1000, 140}, {0, 140}, {0, 240}, {1000, 240}, {1000, 340}, {0, 340}, {0, 395}, {1000, 395}}, {passes=3})
for i, k in ipairs(inks) do c:ink(k, {{(i-0.5)*1000/3, 0}, {(i-0.5)*1000/3, c.H}}, {width=50}) end
t = tarlatan{stiff=0.7}
-- each stripe wiped in small circles within it, the cloth refolded to a clean face between
for i = 1, 3 do
  local pts = {}
  for j = 0, 32 do
    local a = j / 16 * math.pi
    pts[#pts + 1] = {(i - 0.5) * 1000 / 3 + 50 * math.cos(a), c.H / 2 + 50 * math.sin(a)}
  end
  t:wipe(c, pts, {pressure=0.5, passes=2})
  t:refold()
end
use("painting")
soak("painting", {minutes=20}); blot("painting")
r = press{plate=c, sheet="painting", pressure=0.7}"#);
    let st = s.st.borrow();
    let c = st.canvas.as_ref().unwrap();
    let seen = c.seen();
    let f = c.window();
    let (w, h) = (f.w, f.h);
    let mm = c.mm_per_unit() / f.scale;
    // the print, centred and mirrored: plate stripe i (of 50 mm) lands at the sheet's
    // x0 + 150 − (i + ½)·50 mm
    let x0 = (200.0 - 150.0) / 2.0;
    let at = |i: usize| {
        let x = ((x0 + 150.0 - (i as f32 + 0.5) * 50.0) / mm) as usize;
        let (mut sum, mut n) = ([0.0f32; 3], 0.0);
        for y in h / 2 - 2..h / 2 + 2 {
            for dx in 0..3 {
                let p = seen[y * w + x + dx - 1];
                (0..3).for_each(|q| sum[q] += p[q]);
                n += 1.0;
            }
        }
        sum.map(|v| v / n)
    };
    let (v, pb, cy) = (at(0), at(1), at(2));
    assert!(v[0] > 1.5 * v[1] && v[0] > 1.5 * v[2], "vermilion prints red: {v:?}");
    assert!(pb[2] > pb[0] && pb[2] > 0.0, "Prussian blue prints blue: {pb:?}");
    assert!(cy[0] > 2.0 * cy[2] && cy[1] > 2.0 * cy[2] && cy[0] > cy[1], "chrome yellow prints a warm yellow: {cy:?}");
}

/// The printer's choice of ink (IP §5.8): the box's darkest tube as its own paste in light
/// plate oil with oil added, and ground stiff as a copperplate ink in heavy plate oil
/// (`paste="copperplate"`): the stiff one is more viscous and has a higher yield stress; on the
/// same plate, inked and wiped alike, its lines keep more of their ink, its card leaves more
/// plate tone, and pulled on the same damp sheet it prints more ink, each µm of it darker. An
/// unknown paste is refused.
#[test]
fn a_copperplate_ink_wipes_and_prints_stiffer_than_a_loose_one() {
    let mut s = live();
    run(&mut s, CANVAS);
    run(&mut s, PLATE);
    let d = darkest(&s);
    let e = s.run(&format!("ink{{{{\"{d}\", 1}}, paste=\"oil\"}}")).unwrap_err();
    assert!(e.contains("copperplate"), "{e}");
    let out = run(&mut s, &format!(
        r#"loose = ink{{{{"{d}", 1}}, oil=0.3, stiff=0}}
stiff = ink{{{{"{d}", 1}}, paste="copperplate"}}
print("eta=" .. stiff.viscosity / loose.viscosity)
print("yield=" .. stiff.yield / loose.yield)
print("pigment=" .. stiff.pigment / loose.pigment)
function pull(i, name)
  use(k)
  k:clean_ink({{{{0, 100}}, {{1000, 100}}, {{1000, 375}}, {{0, 375}}, {{0, 650}}, {{1000, 650}}}}, {{passes=3}})
  k:ink(i, {{{{0, 375}}, {{1000, 375}}}})
  local t = tarlatan{{stiff=0.7, seed=5}}
  local w = t:wipe(k, {{{{0, 375}}, {{1000, 375}}}}, {{passes=4}})
  print(name .. "_lines=" .. w.lines)
  print(name .. "_film=" .. w.film)
  local pr = sheet{{name=name, size=60, aspect=1, paper={{grammage=250}}, seed=3}}
  soak(pr, {{minutes=30}}); blot(pr)
  local r = press{{plate=k, sheet=pr, pressure=0.6}}
  print(name .. "_taken=" .. r.taken)
end
pull(loose, "loose")
pull(stiff, "stiff")"#
    ));
    assert!(num(&out, "eta") > 10.0 && num(&out, "yield") > 2.0 && num(&out, "pigment") > 1.0, "{out}");
    assert!(num(&out, "stiff_lines") > num(&out, "loose_lines"), "the lines keep more of the stiff ink: {out}");
    assert!(num(&out, "stiff_film") > num(&out, "loose_film"), "more plate tone: {out}");
    assert!(num(&out, "stiff_taken") > num(&out, "loose_taken"), "it prints more: {out}");
}
