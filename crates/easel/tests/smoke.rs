//! A short session at the easel through its commands: open, knife two
//! piles, lay strokes and a passage, wait, look (whole and cropped), check
//! that the log replays within tolerance, save and replay it, keep a journal. What
//! the easel doesn't have errors clearly and changes nothing.
// The replay build: EASEL_ROOT, named sessions and `easel run`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[path = "../../paint/tests/support/image_tolerance.rs"]
mod image_tolerance;

fn root() -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-smoke")
}

fn easel(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env_remove("EASEL_WIDTH").env("EASEL_ROOT", root()).env_remove("EASEL_SESSION").output().unwrap()
}

fn ok(args: &[&str]) -> String {
    let o = easel(args);
    assert!(o.status.success(), "easel {args:?} failed:\n{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

fn fails(args: &[&str]) -> String {
    let o = easel(args);
    assert!(!o.status.success(), "easel {args:?} should fail:\n{}", String::from_utf8_lossy(&o.stdout));
    String::from_utf8(o.stderr).unwrap()
}

/// Closes the session however the test ends.
struct Closing;
impl Drop for Closing {
    fn drop(&mut self) {
        let _ = easel(&["-s", "sketch-smoke", "close"]);
    }
}

// Two grounds, the second rolled: a brushed one (a whole-canvas `work` pass at the live
// width) made this test take a minute, run four times (live, check, run, reopen). Brushed
// grounds are tested in paint (tests::brushed_ground_honors_thickness, ground_grain) and
// replayed in boxes.rs (the round 19 log).
const CANVAS: &str = r#"canvas{size=300, aspect=5, linen={16, 14}, seed=3, ground={{pile={{"lead white", 2}, {"red earth", 1}}, um=100, apply="knife"}, {pile={{"lead white", 6}, {"raw umber", 1}}, um=50, apply="roller"}}}"#;

const PAINT: &str = r#"
a = pile{{"lead white", 6}, {"yellow ochre", 2}, {"vermilion", 0.3}, medium=0.2}
c = pile{{"lead white", 4}, {"cobalt blue", 1}, medium=0.35}
print(a, c)
work(rect(100, 100, 300, 180), {hand="body", pile=c, coverage=3})
b = brush("filbert", 8)
b:load(a, 0.9)
for i = 0, 5 do b:stroke({{520, 40 + 20 * i}, {880, 60 + 20 * i}}, {pressure={0.8, 0.3}}) end
print(wait(24 * 60))
print(drying(300, 200))
"#;

#[test]
fn a_short_session_at_the_easel() {
    let _ = std::fs::remove_dir_all(root());
    std::fs::create_dir_all(root().join("notes")).unwrap();
    let _closing = Closing;
    ok(&["open", "sketch-smoke"]);
    let r = ok(&["-s", "sketch-smoke", "do", CANVAS]);
    assert!(r.contains("ok · chunk 1"), "{r}");
    let r = ok(&["-s", "sketch-smoke", "do", PAINT]);
    assert!(r.contains("pile(lead white 6 (72%), yellow ochre 2 (24%), vermilion 0.3 (4%); medium 0.2)"), "{r}");
    assert!(r.contains("pile(lead white 4 (80%), cobalt blue 1 (20%); medium 0.35)"), "{r}");
    assert!(r.contains("day 2, "), "wait returns the time of day: {r}");
    assert!(r.contains("ok · chunk 2"), "{r}");

    // looking: the whole canvas, and a window with a grid
    let look = ok(&["-s", "sketch-smoke", "look"]);
    let path = look.split_whitespace().next().unwrap();
    assert!(Path::new(path).exists(), "{look}");
    let look = ok(&["-s", "sketch-smoke", "look", "--crop", "450,20,950,180", "--grid", "50", "--mode", "value,squint"]);
    assert!(Path::new(look.split_whitespace().next().unwrap()).exists(), "{look}");

    // probing: a pixel reads as the looks show it (plain, in grays, in the gallery's light,
    // within the looks' 8-bit rounding), and its drying as drying() reads it
    let shown = |mode: &str| {
        let look = ok(&["-s", "sketch-smoke", "look", "--crop", "150,150,155,155", "--mode", mode]);
        image::open(look.split_whitespace().next().unwrap()).unwrap().to_rgb8().get_pixel(0, 0).0.map(i32::from)
    };
    let probe = ok(&["-s", "sketch-smoke", "probe", "--at", "150,150"]);
    let lines: Vec<&str> = probe.lines().collect();
    assert_eq!((lines.len(), lines[0]), (4, "at 150,150"), "{probe}");
    let read = |line: &str, head: &str| -> ([i32; 3], i32) {
        let (rgb, value) = line.strip_prefix(head).and_then(|l| l.split_once(" · value ")).unwrap_or_else(|| panic!("{probe}"));
        let rgb: Vec<i32> = rgb.split(',').map(|v| v.parse().unwrap()).collect();
        ([rgb[0], rgb[1], rgb[2]], value.parse().unwrap())
    };
    let near = |a: [i32; 3], b: [i32; 3]| a.iter().zip(b).all(|(a, b)| (a - b).abs() <= 1);
    let (color, value) = read(lines[1], "color ");
    let linear = color.map(|v| paint::color::srgb_to_linear(v as f32 / 255.0));
    let expected = (paint::color::linear_to_srgb(paint::color::luminance(linear)) * 255.0).round() as i32;
    assert!(near(color, shown("normal")) && (value - expected).abs() <= 1, "{probe}");
    let (color, value) = read(lines[2], "in gallery light ");
    assert!(near([value; 3], shown("value")), "{probe}");
    assert!(near(color, shown("gallery")) && near([value; 3], shown("gallery,value")), "{probe}");
    let felt = r.lines().find(|l| ["open", "setting", "tacky", "dry"].contains(l)).unwrap();
    let probe = ok(&["-s", "sketch-smoke", "probe", "--at", "300,200"]);
    assert!(probe.ends_with(&format!("\ndrying: {felt}\n")), "drying(300, 200) said {felt}: {probe}");
    let probe = ok(&["-s", "sketch-smoke", "probe", "--crop", "300,200,100,100"]);
    assert!(probe.starts_with("over 300,200,100,100, averaged over 120 x 60 pixels\ncolor "), "{probe}");
    let shares: f64 = probe.lines().last().unwrap().strip_prefix("drying: ").unwrap().split(", ").map(|s| s.rsplit_once(' ').unwrap().1.trim_end_matches('%').parse::<f64>().unwrap()).sum();
    assert!((shares - 100.0).abs() <= 2.0, "{probe}");
    for (args, want) in [
        (&["--at", "1001,50"][..], "probe: at 1001,50 is outside the canvas (0,0 to 1000,200)"),
        (&["--at", "1,2,3"][..], "probe: --at 1,2,3: want x,y in canvas units"),
        (&[][..], "probe: want --at x,y or --crop x0,y0,x1,y1 (canvas units)"),
        (&["--scratch", "--at", "1,1"][..], "there is no scratch canvas yet"),
    ] {
        let e = fails(&[&["-s", "sketch-smoke", "probe"][..], args].concat());
        assert!(e.contains(want), "{args:?}: {e}");
    }

    // not in the easel: every one errors, and the log keeps its two chunks
    for cmd in ["undo", "try", "show", "edit", "undone", "redo"] {
        let e = fails(&["-s", "sketch-smoke", cmd, "1"]);
        assert!(e.contains(&format!("no command \"{cmd}\"")), "{cmd}: {e}");
    }
    for arg in ["--dried", "--probe", "--show", "--scale"] {
        let e = fails(&["-s", "sketch-smoke", "look", arg, "1"]);
        assert!(e.contains("unknown argument"), "{arg}: {e}");
    }
    for chunk in [
        "dry()",
        r##"x = mix("#334455", "#887766", 0.5)"##,
        "x = sample(500, 400)",
        "x = probe(500, 400)",
        "show(ellipse(500, 400, 50))",
        r##"b:load("#8090a0", 0.8)"##,
        r##"work(rect(0, 0, 100, 100), {hand="body", color="#8090a0"})"##,
        r##"work(rect(0, 0, 100, 100), {hand="body", pile=a, color=function(x, y) return "#ffffff" end})"##,
    ] {
        let e = fails(&["-s", "sketch-smoke", "do", chunk]);
        assert!(e.contains("the chunk failed and changed nothing"), "{chunk}: {e}");
    }
    // removed from the painter's API: a poured glaze (glazes are brushed:
    // work(m, {hand="glaze", ...})), the drawing mask, the false-color
    // drying look, and waits past 10 years
    for (chunk, want) in [
        ("glaze(rect(0, 0, 10, 10), {pile=a})", "global 'glaze'"),
        ("m = drawing_mask()", "global 'drawing_mask'"),
        ("wait(5259601)", "10 years"),
        ("wait(1e100)", "10 years"),
    ] {
        let e = fails(&["-s", "sketch-smoke", "do", chunk]);
        assert!(e.contains(want) && e.contains("the chunk failed and changed nothing"), "{chunk}: {e}");
    }
    for mode in ["wet", "drying", "stages"] {
        let e = fails(&["-s", "sketch-smoke", "look", "--mode", mode]);
        assert!(e.contains(&format!("--mode {mode}:")), "--mode {mode}: {e}");
    }
    assert!(ok(&["-s", "sketch-smoke", "status"]).starts_with("2 chunks"));

    // the log replays to the live canvas within tolerance, in the session and from the file
    let r = ok(&["-s", "sketch-smoke", "check"]);
    assert!(r.contains("replay matches the live canvas within tolerance (2 chunks"), "{r}");
    let saved = root().join("saved.png");
    ok(&["-s", "sketch-smoke", "save", saved.to_str().unwrap()]);
    ok(&["-s", "sketch-smoke", "close"]);
    let log = root().join("paintings/lua/sketch-smoke.lua");
    let replayed = root().join("replayed.png");
    ok(&["run", log.to_str().unwrap(), "--out", replayed.to_str().unwrap()]);
    image_tolerance::assert_close(&image::open(&saved).unwrap().to_rgba8(), &image::open(&replayed).unwrap().to_rgba8());

    // reopening goes on from the log: a chunk that ran is there for good
    let r = ok(&["open", "sketch-smoke"]);
    assert!(r.contains("resumed 2 chunks"), "{r}");
    // the journal only grows
    ok(&["note", "first entry"]);
    ok(&["note", "second entry", "in two words"]);
    let j = std::fs::read_to_string(root().join("notes/journal.md")).unwrap();
    let lines: Vec<&str> = j.lines().collect();
    assert_eq!(lines.len(), 2, "{j}");
    assert!(lines[0].starts_with("- day 2, ") && lines[0].ends_with(": first entry"), "{j}");
    assert!(lines[1].ends_with(": second entry in two words"), "{j}");
}
