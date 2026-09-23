//! Study sheet for grown trees: the engine grows skeletons (`Habit::grow`),
//! the painter paints them (`paintings::trees`).
//!
//!   cargo paint study_trees                    sheet: oaks, birches / dead oaks, spruces
//!   cargo paint study_trees -- --one oak       one big tree filling the canvas
//!   cargo paint study_trees -- --one dead_oak --width 3200

use paint::{Habit, Style, hex};
use paintings::trees::{self, Bark, TreeHand};

fn main() {
    let o = paintings::run::Run::new("study_trees");
    let args: Vec<String> = std::env::args().collect();
    let one = args.iter().position(|a| a == "--one").and_then(|i| args.get(i + 1)).cloned();
    let st = Style::friedrich();
    let pal = &st.palette;
    let aspect = if one.is_some() { 1.0 } else { 1.25 };
    let mut c = st.prepare(o.width, aspect, 1);
    let h = c.height();
    // paints mixed on the palette
    let bark = Bark {
        dark: pal.mix(hex("#241e19")).paint(0.3),
        dead: Some(pal.mix(hex("#302a24")).paint(0.3)),
        light: Some(pal.mix(hex("#8f846f")).paint(0.35)),
        wood: Some(pal.mix(hex("#b3a384")).paint(0.2)),
    };
    let needles = pal.mix(hex("#1c211d")).paint(0.1);
    let hand = TreeHand::default();
    let habit = |name: &str| match name {
        "oak" => Habit::oak(),
        "dead_oak" => Habit::dead_oak(),
        "birch" => Habit::birch(),
        "spruce" => Habit::spruce(),
        _ => panic!("unknown habit {name}"),
    };
    if let Some(name) = one {
        let sk = habit(&name).grow((500.0, h * 0.97), h * 0.92, o.seed);
        eprintln!("{name}: {} limbs", sk.limbs.len());
        if name == "spruce" {
            trees::grown_spruce(&mut c, &sk, needles, o.seed);
        } else {
            trees::tree(&mut c, &sk, &bark, &TreeHand { finest: 0.25, ..hand }, o.seed);
        }
        c.relief(0.5, 0.0);
        o.save(&mut c);
        return;
    }
    // top: three oaks and two birches; bottom: three dead oaks, two spruces
    let rows = [(h * 0.48, ["oak", "oak", "oak", "birch", "birch"]), (h * 0.975, ["dead_oak", "dead_oak", "dead_oak", "spruce", "spruce"])];
    for (r, (ground, names)) in rows.iter().enumerate() {
        for (i, name) in names.iter().enumerate() {
            let x = 100.0 + i as f32 * 200.0;
            let seed = 11 + (r * 5 + i) as u64 * 7 + o.seed;
            let ht = if *name == "spruce" { h * 0.42 } else { h * 0.4 };
            let sk = habit(name).grow((x, *ground), ht, seed);
            eprintln!("{name} seed {seed}: {} limbs", sk.limbs.len());
            if *name == "spruce" {
                trees::grown_spruce(&mut c, &sk, needles, seed);
            } else {
                trees::tree(&mut c, &sk, &bark, &hand, seed);
            }
        }
    }
    // the small gesture spruce, for distant trees, in a row of sizes
    for (k, s) in [8.0f32, 14.0, 24.0, 40.0].iter().enumerate() {
        trees::spruce(&mut c, (870.0 + k as f32 * 30.0 - 40.0, h * 0.5), *s, needles, 90 + k as u64);
    }
    c.relief(0.5, 0.0);
    o.save(&mut c);
}
