// Same geometry, brush handling and seeds as the release wipe comparison.
// Changes: 2400px canvas width and dark brown #3f2b22 paint.
// Writes visible wet paint, without baking the canvas or changing its state.
use paint::{Canvas, Crop, Handling, Linen, Mask, Tool, hex};
use paint::rag::Rag;
use std::path::Path;

fn save(c: &Canvas, path: &Path) {
    let f = c.window();
    let pixels = c.seen().iter().flat_map(|p| p.map(|x|
        (paint::color::linear_to_srgb(x).clamp(0.0, 1.0) * 255.0).round() as u8
    )).collect();
    image::RgbImage::from_raw(f.w as u32, f.h as u32, pixels).unwrap().save(path).unwrap();
}

fn main() {
    let out = std::env::args().nth(1).expect("output directory");
    let out = Path::new(&out);
    std::fs::create_dir_all(out).unwrap();
    let crop = Crop { units: [260.0, 190.0, 740.0, 490.0], margin: 30.0 };
    let mut c = Canvas::new_window(2400, 1.5, hex("#b08060"), Some(crop))
        .with_engine(3).with_size_mm(440.0)
        .with_linen(Linen { warp_per_cm: 15.0, weft_per_cm: 15.0, ..Linen::fine(3) });
    c.prime(hex("#e4dcc8"), 0.9, 25.0, 0.6, 0.3, 5);
    c.set_hand_time(Some(15.0));
    let m = Mask::from_fn(c.frame(), |x, y|
        if (100.0..900.0).contains(&x) && (120.0..550.0).contains(&y) { 1.0 } else { 0.0 });
    let no_plough = std::env::args().nth(2).as_deref()==Some("no-plough");
    let mut tool=Tool::filbert(22.0);
    if no_plough { tool.push=0.0; }
    let hd = Handling::new(tool).color(|_, _| hex("#3f2b22"))
        .paint(0.85, 0.75).coverage(1.0).load(0.3).angle(|_, _| 0.0).fill(true);
    let cache_path = std::env::args().nth(3).expect("scratch checkpoint path (third argument)");
    let cache = std::path::Path::new(&cache_path);
    if cache.exists() && !no_plough && std::env::args().nth(2).as_deref()!=Some("fresh") { c = Canvas::read_state(&mut std::fs::File::open(cache).unwrap()).unwrap().0; }
    else { c.work(&m, &hd, 11); c.clock_hand_min(); if !no_plough && std::env::args().nth(2).as_deref()!=Some("fresh") { c.write_state(&mut std::fs::File::create(cache).unwrap(), "rag diagnostic").unwrap(); } }
    if std::env::args().nth(2).as_deref()==Some("uniform") { paint::rag::probe::uniform_film(&mut c,69.294109); }
    save(&c, &out.join("untouched.png"));
    let f = c.window();
    let initial: Vec<f32> = (0..f.h).flat_map(|y| (0..f.w).map(move |x| (x,y))).map(|(x,y)| c.wet_um(f.ux(x),f.uy(y))).collect();
    std::fs::write(out.join("initial-film.f32"),initial.iter().flat_map(|v|v.to_le_bytes()).collect::<Vec<_>>()).unwrap();
    image::GrayImage::from_raw(f.w as u32,f.h as u32,initial.iter().map(|v|(v/100.0*255.0).clamp(0.0,255.0).round() as u8).collect()).unwrap().save(out.join("initial-film.png")).unwrap();
    let mut rag = Rag::new(100.0,7);
    rag.dip(0.5,c.now_min(),c.tally_mut());
    paint::rag::probe::begin(f.w*f.h);
    c.rag_wipe(&mut rag, &[(300.0,340.0),(700.0,340.0)], &[0.8],19);
    save(&c,&out.join("after.png"));
    let t=paint::rag::probe::take().unwrap();
    let mut nodes=String::from("frame,index,x_mm,y_mm,z_mm\n");
    for (frame,shape) in t.shapes.iter().enumerate() { for (i,p) in shape.iter().enumerate() { nodes.push_str(&format!("{frame},{i},{},{},{}\n",p[0],p[1],p[2])); } }
    std::fs::write(out.join("cloth-shape.csv"),nodes).unwrap();
    let left: Vec<f32> = (0..f.h).flat_map(|y| (0..f.w).map(move |x| (x,y))).map(|(x,y)| c.wet_um(f.ux(x),f.uy(y))).collect();
    let error = (0..left.len()).map(|i| (initial[i]-t.pickup[i]+t.deposit[i]-left[i]).abs()).fold(0.0,f32::max);
    println!("Maximum per-pixel film accounting error: {error} um");
    for (name,data,scale) in [("contact",&t.contact,1.0),("pickup",&t.pickup,100.0),("deposit",&t.deposit,5.0),("film",&left,100.0)] {
        let bytes: Vec<u8> = data.iter().flat_map(|&v| v.to_le_bytes()).collect();
        std::fs::write(out.join(format!("{name}.f32")),bytes).unwrap();
        let pixels: Vec<u8> = data.iter().map(|v| (v/scale*255.0).clamp(0.0,255.0).round() as u8).collect();
        image::GrayImage::from_raw(f.w as u32,f.h as u32,pixels).unwrap().save(out.join(format!("{name}.png"))).unwrap();
    }
    std::fs::write(out.join("accounting.txt"),format!("Maximum per-pixel error: {error} um\nDimensions: {} x {}\n",f.w,f.h)).unwrap();
}
