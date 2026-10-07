//! Characterization captured from Server::look at 4efb739c before moving its parser.
use super::*;

const CANVAS: &str = r#"canvas{size=300, aspect=1.25, seed=3, linen=15, ground={{pile={{"lead white", 4}, {"red earth", 1}}, um=80, apply="knife"}}}"#;
const TOOLS: &str = r#"p = pile{{"lead white", 6}, {"smalt", 1}, medium=0.2}; dk = pile{{"raw umber", 2}, {"bone black", 1}}; k = knife{width=40}; k:load(dk, 0.8); clean = knife{width=20}"#;

const O0: &str = "ERR no canvas yet: the first chunk is canvas{...}";
const O1: &str = "ERR --grid step: want > 0 units";
const O2: &str = "ERR --size needs a value";
const O3: &str = "ERR --size N (px)";
const O4: &str = "ERR --crop needs a value";
const O5: &str = "ERR --crop 1,2,3: want four numeric canvas-unit coordinates x0,y0,x1,y1 (opposite corners, not width/height or pixels); example: --crop 100,100,200,200";
const O6: &str = "ERR --crop NaN,0,2,3: want four numeric canvas-unit coordinates x0,y0,x1,y1 (opposite corners, not width/height or pixels); example: --crop 100,100,200,200";
const O7: &str = "ERR --mode needs a value";
const O8: &str = "ERR --mode wet: normal, value, squint, mirror, relief, gallery (comma-separated)";
const O9: &str = "ERR --light needs a value";
const O10: &str = "ERR --light 45,0: want azimuth,elevation in degrees (elevation 3 to 89)";
const O11: &str = "ERR look: unknown argument \"--unknown\" (--crop x0,y0,x1,y1 --mode value,squint,mirror,relief,gallery --light azimuth,elevation --grid [step] --size N | --palette)";
const O12: &str = "ERR look --palette: no piles on the palette yet";
const O13: &str = "ERR look: --palette takes no other option";
const O14: &str = "ERR look: unknown argument \"--survey\" (--crop x0,y0,x1,y1 --mode value,squint,mirror,relief,gallery --light azimuth,elevation --grid [step] --size N | --palette)";
const O15: &str = "ERR look: unknown argument \"--hold\" (--crop x0,y0,x1,y1 --mode value,squint,mirror,relief,gallery --light azimuth,elevation --grid [step] --size N | --palette)";
const O16: &str = "ERR look: unknown argument \"--compare\" (--crop x0,y0,x1,y1 --mode value,squint,mirror,relief,gallery --light azimuth,elevation --grid [step] --size N | --palette)";
const O17: &str = "ERR look --survey covers the whole canvas at full detail: no --crop or --size";
const O18: &str = "ERR look: --survey and --compare are two looks; ask for one";
const O19: &str = "ERR --compare needs an earlier look's png";
const O20: &str = "ERR --hold needs a knife or a pile: the name of a global that holds one";
const O21: &str = "ERR --at needs a point on the canvas: x,y in units";
const O22: &str = "ERR look: --hold <knife or pile> and --at x,y go together";
const O23: &str = "ERR --at bad: want x,y in units";
const O24: &str = "ERR --at 1: want x,y in units";
const O25: &str = "ERR --at 1,2,3: want x,y in units";
const O26: &str = "ERR look --hold k: no global of that name holds a knife or a pile (there is none yet)";
const O27: &str = "ERR look --hold p: no global of that name holds a knife or a pile (there is none yet)";
const O28: &str = "ERR look --hold clean: no global of that name holds a knife or a pile (there is none yet)";
const O29: &str = "ERR look --hold nope: no global of that name holds a knife or a pile (there is none yet)";
const O30: &str = "ERR look --hold takes --at, --crop, --mode value, squint, relief or gallery and --light, nothing else";
const O31: &str = "ERR look: --hold is a look of its own: no --survey or --compare";
const O32: &str = "ERR --mode --size: normal, value, squint, mirror, relief, gallery (comma-separated)";
const O33: &str = "ERR --light --size: want azimuth,elevation in degrees";
const O34: &str = "OK 60x48:37b1ddba5c65db91";
const O35: &str = "OK 20x16:e72d48682e950f22";
const O36: &str = "OK 1x1:1e727bfdc4fdede6";
const O37: &str = "OK 30x24:440c5ef3ac03f528";
const O38: &str = "OK 27x22:f0d4894c5071096f";
const O39: &str = "OK 60x48:51417d398234c566";
const O40: &str = "OK 60x48:296c0c0838841a62";
const O41: &str = "OK 60x48:2f32a8d004f65796";
const O42: &str = "OK 60x48:377973d4653280b1";
const O43: &str = "ERR --crop is empty or outside the canvas";
const O44: &str = "OK 1000x230:afa4d14ed5ee9bae";
const O45: &str = "OK 30x24:440c5ef3ac03f528 30x24:440c5ef3ac03f528 30x24:440c5ef3ac03f528 30x24:440c5ef3ac03f528";
const O46: &str = "OK 30x24:215d3917e56ff7e3 30x24:7bad5c4782d5d37c 30x24:a1d5b1babe78964f 30x24:1eed9e2a5f6603d4";
const O47: &str = "OK 139x48:24678b5f606fec0f";
const O48: &str = "OK 54x16:aae7f92e2e3b1090";
const O49: &str = "OK 70x22:ce571ed0ac52b3a9";
const O50: &str = "OK 139x48:f0c79d965a8cc9b8";
const O51: &str = "ERR --compare <studio>/missing.png: No such file or directory (os error 2)";
const O52: &str = "ERR --compare <studio>/outside.png: No such file or directory (os error 2)";
const O53: &str = "ERR --compare <studio>/invalid.png: Format error decoding Png: Invalid PNG signature.";
const O54: &str = "OK 14x14:a48cf266df065b0c";
const O55: &str = "OK 14x14:519d71a72a31accf";
const O56: &str = "OK 14x14:80e4e9d5f53d66ec";
const O57: &str = "OK 18x18:c385e7f74e26f750";
const O58: &str = "OK 12x12:c2b79016818e7ac6";
const O59: &str = "ERR look --hold: --at 1400,320 is not on the canvas (1000 x 800 units)";
const O60: &str = "ERR look --hold: --at NaN,320 is not on the canvas (1000 x 800 units)";
const O61: &str = "ERR look --hold: --at inf,320 is not on the canvas (1000 x 800 units)";
const O62: &str = "ERR look --hold: --at 400,320 lies outside the --crop";
const O63: &str = "ERR look --hold clean: the knife is clean; load it first (k:load(pile))";
const O64: &str = "ERR look --hold nope: no global of that name holds a knife or a pile (clean, dk, k, p)";
const O65: &str = "ERR --compare <studio>/--survey: No such file or directory (os error 2)";
const O66: &str = "ERR --compare <studio>/../outside.png: an earlier look is a picture in this studio (<studio>)";
const O67: &str = "ERR --compare <studio>/escaped.png: an earlier look is a picture in this studio (<studio>)";
const O68: &str = "OK 139x48:c70d556220715f26";
const O69: &str = "OK 1226x296:461f3b3093ae1127";
const O70: &str = "OK 14x1:7a09c0cedfbc393f";
const O71: &str = "OK 2442x593:5be409485266f757";
const O72: &str = "OK 828x340:a1ca7a8f32324bfa";
const O73: &str = "OK 850x630:47f04791b183c05f 850x630:47f04791b183c05f";
const O74: &str = "OK 850x630:1fde94d972da8bed 850x630:02c829af1e5406f4";
const O75: &str = "ERR --crop exceeds 1200 pixels per side (crops stay 1:1); coordinates are canvas units, x0,y0,x1,y1 (opposite corners). At this canvas resolution each side may span at most 705.88 units; example: --crop 0.00,0.00,352.00,352.00";

const SMALL: &[(&str, &str, &str)] = &[
    ("", O0, O34),
    ("--size 20", O0, O35),
    ("--size 0", O0, O36),
    ("--size 9999", O0, O34),
    ("--size 20 --size 30", O0, O37),
    ("--crop 50,40,500,400", O0, O38),
    ("--crop 500,400,50,40 --size 20", O0, O38),
    ("--mode value,squint,mirror,relief", O0, O39),
    ("--light 45,15 --mode gallery,gray,blur", O0, O39),
    ("--mode normal,", O0, O34),
    ("--grid", O0, O40),
    ("--grid 100", O0, O41),
    ("--grid 0", O1, O1),
    ("--grid NaN", O1, O1),
    ("--grid inf", O0, O42),
    ("--grid -1", O1, O1),
    ("--size", O2, O2),
    ("--size bad", O3, O3),
    ("--crop", O4, O4),
    ("--crop 1,2,3", O5, O5),
    ("--crop NaN,0,2,3", O6, O6),
    ("--crop 0,0,0,0", O0, O43),
    ("--mode", O7, O7),
    ("--mode wet", O8, O8),
    ("--light", O9, O9),
    ("--light 45,0", O10, O10),
    ("--unknown", O11, O11),
    ("--palette", O12, O44),
    ("--palette --palette", O12, O44),
    ("--palette --mode normal,", O12, O44),
    ("--palette --size 20", O13, O13),
    ("--palette --survey", O14, O14),
    ("--survey --palette", O14, O14),
    ("--palette --hold k --at 400,320", O15, O15),
    ("--hold --palette", O15, O15),
    ("--palette --compare earlier.png", O16, O16),
    ("--palette --unknown", O11, O11),
    ("--survey", O0, O45),
    ("--survey --survey", O0, O45),
    ("--survey --mode value,mirror,relief --grid 100", O0, O46),
    ("--survey --crop", O17, O17),
    ("--survey --crop 0,0,100,100", O17, O17),
    ("--survey --size", O17, O17),
    ("--survey --mode --size", O17, O17),
    ("--survey --unknown", O0, O11),
    ("--survey --grid 0", O0, O1),
    ("--survey --compare earlier.png", O18, O18),
    ("--compare", O19, O19),
    ("--compare earlier.png", O0, O47),
    ("--compare earlier.png --size 20", O0, O48),
    ("--compare earlier.png --crop 50,40,500,400", O0, O49),
    ("--compare earlier.png --compare earlier.png --mode value", O0, O50),
    ("--compare missing.png", O0, O51),
    ("--compare outside.png", O0, O52),
    ("--compare invalid.png", O0, O53),
    ("--compare earlier.png --mode wet", O8, O8),
    ("--hold", O20, O20),
    ("--at", O21, O21),
    ("--hold k", O22, O22),
    ("--at 400,320", O22, O22),
    ("--hold k --at", O21, O21),
    ("--hold k --at bad", O23, O23),
    ("--hold k --at 1", O24, O24),
    ("--hold k --at 1,2,3", O25, O25),
    ("--hold k --at 400,320", O26, O54),
    ("--at 400,320 --hold p", O27, O55),
    ("--hold p --hold k --at 1,1 --at 400,320", O26, O54),
    ("--hold k --at 400,320 --mode value,squint,relief --light 45,15", O26, O56),
    ("--hold k --at 400,320 --crop 500,500,200,200", O26, O57),
    ("--hold k --at 100,100 --crop -50,-50,200,200", O26, O58),
    ("--hold k --at 100,100 --crop 0,0,200,200", O26, O58),
    ("--hold k --at 1400,320", O26, O59),
    ("--hold k --at NaN,320", O26, O60),
    ("--hold k --at inf,320", O26, O61),
    ("--hold k --at 400,320 --crop 0,0,100,100", O26, O62),
    ("--hold clean --at 400,320", O28, O63),
    ("--hold nope --at 400,320", O29, O64),
    ("--hold k --at 400,320 --size 20", O30, O30),
    ("--hold k --at 400,320 --grid", O30, O30),
    ("--hold k --at 400,320 --mirror", O30, O30),
    ("--hold k --at 400,320 --mode mirror", O30, O30),
    ("--hold k --at 400,320 --unknown", O11, O11),
    ("--hold k --at 400,320 --survey", O31, O31),
    ("--hold k --at 400,320 --compare earlier.png", O31, O31),
    ("--hold k --at 400,320 --survey --compare earlier.png", O18, O18),
    ("--hold k --survey", O22, O22),
    ("--hold k --at bad --unknown", O23, O23),
    ("--hold k --at 400,320 --size bad", O3, O3),
    ("--mode --survey", O0, O7),
    ("--size --hold k --at 400,320", O2, O2),
    ("--compare --survey", O0, O65),
    ("--at --hold", O22, O22),
    ("--compare ../outside.png", O0, O66),
    ("--compare escaped.png", O0, O67),
    ("--compare <absolute>", O0, O47),
    ("--compare earlier.png --mode", O32, O32),
    ("--compare earlier.png --light", O33, O33),
    ("--compare earlier.png --mode normal --mode", O32, O32),
    ("--compare earlier.png --crop 0,0,100,100 --mode", O7, O7),
    ("--compare earlier.png --size 20 --light", O9, O9),
    ("--compare earlier.png --grid", O0, O68),
    ("--compare earlier.png --light --hold k --at 400,320", O31, O31),
    ("--compare earlier.png --mode --survey", O18, O18),
];

const LARGE: &[(&str, &str, &str)] = &[
    ("--compare earlier.png", O0, O69),
    ("--compare earlier.png --size 800", O0, O69),
    ("--compare earlier.png --size 0", O0, O70),
    ("--compare earlier.png --size 9999", O0, O71),
    ("--compare earlier.png --crop 0,0,200,200", O0, O72),
    ("--survey", O0, O73),
    ("--survey --mode mirror,gallery --grid 100", O0, O74),
    ("--crop 0,0,900,600", O0, O75),
    ("--hold k --at 400,320 --crop 0,0,900,600", O26, O75),
];

#[test]
fn requests_preserve_argument_errors_and_png_bytes() {
    let dir = std::env::temp_dir().join(format!("easel-look-requests-{}", std::process::id()));
    let studio = dir.join("studio");
    std::fs::create_dir_all(&studio).unwrap();
    let studio = studio.canonicalize().unwrap();
    let outside = dir.join("outside.png");
    std::fs::write(&outside, b"outside").unwrap();
    std::os::unix::fs::symlink(&outside, studio.join("escaped.png")).unwrap();
    std::fs::write(studio.join("invalid.png"), b"invalid").unwrap();
    let mut earlier = image::RgbImage::new(7, 5);
    for (x, y, p) in earlier.enumerate_pixels_mut() {
        *p = image::Rgb([(x * 31) as u8, (y * 47) as u8, 19]);
    }
    earlier.save(studio.join("earlier.png")).unwrap();
    for (width, canvas, cases) in [(60, CANVAS.to_owned(), SMALL), (1700, CANVAS.replace("aspect=1.25", "aspect=2.7"), LARGE)] {
        for with_canvas in [false, true] {
            let mut s = Session::new(width).unwrap();
            if with_canvas {
                s.run(&canvas).unwrap();
                s.run(TOOLS).unwrap();
            }
            for &(args, empty, painted) in cases {
                let args = args.replace("<absolute>", studio.join("earlier.png").to_str().unwrap());
                let a = args.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
                let observed = (|| {
                    let request = Request::parse(&a)?;
                    let rendered = request.render(&s, &studio)?;
                    let mut images = Vec::new();
                    for picture in rendered.pictures {
                        let picture = picture?;
                        let im = image::load_from_memory(&picture.png).unwrap();
                        assert_eq!((im.width() as usize, im.height() as usize), (picture.width, picture.height));
                        images.push(format!("{}x{}:{:016x}", im.width(), im.height(), crate::save::fnv1a(&picture.png)));
                    }
                    Ok::<_, String>(format!("OK {}", images.join(" ")))
                })().unwrap_or_else(|e| format!("ERR {e}")).replace(studio.to_str().unwrap(), "<studio>");
                assert_eq!(observed, if with_canvas { painted } else { empty }, "width={width}, canvas={with_canvas}, {args}");
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
