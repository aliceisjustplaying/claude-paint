extern crate paint;
use paint::tally::{Piles,Tally};
fn main(){
 let colors: Vec<[f32;3]>=(0..3).flat_map(|r|(0..3).flat_map(move|g|(0..3).map(move|b|[r as f32/2.,g as f32/2.,b as f32/2.]))).take(17).collect();
 let mut p=Piles::default();let mut t=Tally::default();
 for (i,c) in colors.iter().take(16).enumerate(){let old=t.secs;p.knife(&mut t,*c);println!("mix {} seconds {} retained {}",i+1,t.secs-old,p.piles.len());assert_eq!(p.piles.len(),i+1);assert_eq!(t.secs-old,20.);}
 let mut sixteen=p.clone();let old=t.secs;sixteen.trip(&mut t,colors[0]);println!("oldest at16 seconds {}",t.secs-old);assert_eq!(t.secs-old,2.5);
 p.knife(&mut t,colors[16]);assert_eq!(p.piles.len(),16);let old=t.secs;p.trip(&mut t,colors[0]);println!("oldest after17 seconds {} retained {}",t.secs-old,p.piles.len());assert_eq!(t.secs-old,22.5);
 let old=t.secs;p.trip(&mut t,colors[16]);println!("newest seconds {}",t.secs-old);assert_eq!(t.secs-old,2.5);
}
