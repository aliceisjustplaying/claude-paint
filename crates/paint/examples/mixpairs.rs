use paint::color::{mix, Mix};
use std::io::BufRead;
// Reads lines "r g b r g b t" (linear RGB of two paints and the share of the second) and prints the mixed linear RGB.
fn main() {
    for line in std::io::stdin().lock().lines() {
        let v: Vec<f32> = line.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        if v.len() != 7 { continue; }
        let c = mix([v[0], v[1], v[2]], [v[3], v[4], v[5]], v[6], Mix::Pigment);
        println!("{} {} {}", c[0], c[1], c[2]);
    }
}
