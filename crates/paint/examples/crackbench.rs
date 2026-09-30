//! Scratch harness: load a canvas state dumped just before `cracks{}`,
//! crack it, light the relief, save.
//!   crackbench STATE OUT.png [old] [key=value ...]
use paint::{Canvas, Cracks};
use std::io::BufReader;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let t = std::time::Instant::now();
    let (mut c, _) = Canvas::read_state(&mut BufReader::new(std::fs::File::open(&a[1]).unwrap())).unwrap();
    eprintln!("read {:.1}s", t.elapsed().as_secs_f32());
    let mut k = Cracks::aged(47);
    for kv in &a[3..] {
        if kv == "even" { k = Cracks::even(47); continue; }
        let (key, v) = kv.split_once('=').unwrap();
        let x: f32 = v.parse().unwrap();
        match key {
            "dirt" => k.dirt = x,
            "vary" => k.vary = x,
            "veil" => k.veil = x,
            "island_mm" => k.island_mm = Some(x),
            "width_um" => k.width_um = Some(x),
            "depth_um" => k.depth_um = x,
            "cupping_um" => k.cupping_um = x,
            "seed" => k.seed = x as u64,
            "hierarchy" => k.hierarchy = x,
            "patchy" => k.patchy = x,
            "grain" => k.grain = x,
            "grime" => k.grime = x,
            _ => panic!("key {key}"),
        }
    }
    let t = std::time::Instant::now();
    c.crack(&k);
    eprintln!("crack {:.2}s", t.elapsed().as_secs_f32());
    let rs: f32 = std::env::var("RELIEF").ok().and_then(|v| v.parse().ok()).unwrap_or(0.06);
    c.relief(rs, 0.006);
    c.save(&a[2]).unwrap();
}
