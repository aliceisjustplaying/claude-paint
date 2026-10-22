use paint::mixer::{LATENT_SIZE, decode, encode};
use std::io::BufRead;
// Mixes two linear RGB paints through the mixer's latent, one line per mixture, and prints the linear RGB result:
// "r g b r g b t" mixes the latents at share t of the second paint, as color::mix does;
// "r g b sa r g b sb t" weights each paint's latent by its tinting strength as well, as Palette::pile does.
fn main() {
    for line in std::io::stdin().lock().lines() {
        let v: Vec<f32> = line.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (a, sa, b, sb, t) = match v.len() {
            7 => ([v[0], v[1], v[2]], 1.0, [v[3], v[4], v[5]], 1.0, v[6]),
            9 => ([v[0], v[1], v[2]], v[3], [v[4], v[5], v[6]], v[7], v[8]),
            _ => continue,
        };
        let (la, lb) = (encode(&a), encode(&b));
        let (wa, wb) = ((1.0 - t) * sa, t * sb);
        let lat: [f32; LATENT_SIZE] = std::array::from_fn(|k| (la[k] * wa + lb[k] * wb) / (wa + wb).max(1e-9));
        let c = decode(&lat);
        println!("{} {} {}", c[0], c[1], c[2]);
    }
}
