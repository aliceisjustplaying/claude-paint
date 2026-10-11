mod panel_study;

use paint::Canvas;
use paint::panel::Topography;
use std::path::Path;

fn inventory(c: &Canvas) -> f64 {
    c.material_total()
        + c.panel_oil_absorbed().map_or(0.0, |a| {
            a.iter().map(|&v| v as f64).sum::<f64>() / (c.window().scale as f64).powi(2)
        })
}

fn scene(name: &str, width: usize, engine: u32, scale: f32) -> (Canvas, f64) {
    let mut c =
        panel_study::prepare(name, width, Topography::QuarterSawnOakGallery).with_engine(engine);
    let before = inventory(&c);
    panel_study::winter(&mut c, scale, true);
    let added = inventory(&c) - before;
    (c, added)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let dir = Path::new(
        args.get(1)
            .expect("panel_winter_3 <directory> [width] [load-scale]"),
    );
    let width: usize = args.get(2).map_or(1200, |s| s.parse().unwrap());
    std::fs::create_dir_all(dir).unwrap();
    let factor = if let Some(s) = args.get(3) {
        s.parse().unwrap()
    } else {
        let target = scene("chalk", 360, 5, 1.0).1;
        let (mut lo, mut hi) = (0.65, 1.3);
        for _ in 0..10 {
            let mid = (lo + hi) * 0.5;
            if scene("chalk", 360, 10, mid).1 < target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) * 0.5
    };
    let mut receipt = String::from(
        "case,checkpoint_bytes,roundtrip_differing_bytes,view_differing_f32_channels\n",
    );
    let mut amounts =
        String::from("case,engine,load_scale,paint_plus_absorbed_oil_coat_unit_area\n");
    for (name, engine, scale, label) in [
        ("chalk", 5, 1.0, "oak-baseline"),
        ("chalk", 10, 1.0, "oak-body2-equal-input"),
        ("chalk", 10, factor, "oak-body2-matched"),
        ("chalk", 9, 1.0, "oak-lean2-equal-input"),
        ("linen", 10, factor, "linen-body2"),
        ("smooth", 10, factor, "smooth-body2"),
        ("oil", 10, factor, "oil-body2"),
    ] {
        let (c, deposited) = scene(name, width, engine, scale);
        panel_study::views(&c, dir, label, &mut receipt);
        amounts.push_str(&format!("{label},{engine},{scale:.9},{deposited:.9}\n"));
    }
    std::fs::write(dir.join("roundtrip.csv"), receipt).unwrap();
    std::fs::write(dir.join("inventory.csv"), amounts).unwrap();
    std::fs::write(dir.join("load-scale.txt"), format!("{factor:.9}\n")).unwrap();
}
