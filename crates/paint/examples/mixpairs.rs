use paint::mixer::{LATENT_SIZE, decode, encode};
use std::io::BufRead;
// Mixes two linear RGB paints through the mixer's latent, one line per mixture, and prints the linear RGB result:
// "r g b r g b t" mixes the latents at share t of the second paint, as color::mix does;
// "r g b sa r g b sb t" weights each paint's latent by its tinting strength as well, as Palette::pile does.
fn main() {
    let engine9 = std::env::args().any(|arg| arg == "--engine9");
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() == 5 {
            let la = paint::mixer::measured(fields[0]).expect("unknown measured paint");
            let lb = paint::mixer::measured(fields[2]).expect("unknown measured paint");
            let (sa, sb, t): (f32, f32, f32) = (fields[1].parse().unwrap(), fields[3].parse().unwrap(), fields[4].parse().unwrap());
            let (wa, wb) = ((1.0 - t) * sa, t * sb);
            let lat: [f32; LATENT_SIZE] = std::array::from_fn(|k| (la[k] * wa + lb[k] * wb) / (wa + wb).max(1e-9));
            let c = if engine9 { paint::mixer::decode9(&lat) } else { decode(&lat) };
            println!("{} {} {}", c[0], c[1], c[2]);
            continue;
        }
        let v: Vec<f32> = fields.iter().map(|x| x.parse().unwrap()).collect();
        let (a, sa, b, sb, t) = match v.len() {
            7 => ([v[0], v[1], v[2]], 1.0, [v[3], v[4], v[5]], 1.0, v[6]),
            9 => ([v[0], v[1], v[2]], v[3], [v[4], v[5], v[6]], v[7], v[8]),
            _ => continue,
        };
        let (la, lb) = if engine9 { (paint::mixer::encode9(&a), paint::mixer::encode9(&b)) } else { (encode(&a), encode(&b)) };
        let (wa, wb) = ((1.0 - t) * sa, t * sb);
        let lat: [f32; LATENT_SIZE] = std::array::from_fn(|k| (la[k] * wa + lb[k] * wb) / (wa + wb).max(1e-9));
        let c = if engine9 { paint::mixer::decode9(&lat) } else { decode(&lat) };
        println!("{} {} {}", c[0], c[1], c[2]);
    }
}
