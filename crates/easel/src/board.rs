//! The palette board: the heaps of paint knifed out on it, as a painter keeps
//! one through a sitting.
//!
//! A pile (`pile{}`) is a heap on the board. The painter can knife more tube
//! paint into a heap (`p:add{}`) and look again (`look --palette`), steering a
//! mix by eye; knife two heaps together into a new one (`mix{}`); set out only
//! some tubes (`palette{set_out=}`), as a painter sets out a limited palette;
//! and keep the board as dirty as painters do (`palette{dirty=}`): a brush
//! that arrives at a heap carrying the last paint leaves a little of it there,
//! and the smears of the mixing area seep into each new heap. That drift, all
//! of it in tube parts mixed by the palette's own physics, is what makes the
//! colors of a painting share a family. `palette{clean=true}` scrapes the
//! mixing area and skims the heaps.
//!
//! Untouched (no `palette{}`, no `p:add`, no `mix`), every pile is exactly the
//! pile it was knifed as: old logs replay as they always did.

use paint::Palette;

/// Heaps that stay on the board; the oldest beyond this is scraped off.
pub const LIVE: usize = 16;
/// The most dirt a heap takes before it is all dirt the brush finds (share of its volume).
const DIRT_CAP: f32 = 0.5;
/// Paint a brush leaves in a heap on one visit, at `dirty = 1` (share of the heap).
const VISIT: f32 = 0.03;
/// How fast the mixing area's smears follow the paint used.
const RESIDUE_FOLLOW: f32 = 0.12;
/// How much of the mixing area seeps into a new heap, at `dirty = 1`.
const SEEP: f32 = 0.08;

#[derive(Clone, Debug)]
pub struct Heap {
    pub id: u64,
    pub name: Option<String>,
    /// The paint as knifed and added to: (tube, volume), the first knifing
    /// summing to 1.
    pub parts: Vec<(usize, f32)>,
    /// The parts' sum at the first knifing, as given (to scale `p:add`).
    pub given_sum: f32,
    pub medium: f32,
    pub solvent: f32,
    pub oil_rate: f32,
    /// Paint other brushes left in it: (tube, volume).
    pub dirt: Vec<(usize, f32)>,
    /// Knifed into or dirtied since it was knifed: its mixture is computed again.
    pub changed: bool,
    /// Knifed into (`p:add`) since it was knifed.
    pub added: bool,
    pub scraped: bool,
}

impl Heap {
    pub fn volume(&self) -> f32 {
        self.parts.iter().map(|p| p.1).sum()
    }
    pub fn dirt_volume(&self) -> f32 {
        self.dirt.iter().map(|p| p.1).sum()
    }
    /// What a brush takes from it: its parts and its dirt together, as
    /// fractions summing to 1.
    pub fn fractions(&self) -> Vec<(usize, f32)> {
        let mut all: Vec<(usize, f32)> = self.parts.clone();
        for &(i, v) in &self.dirt {
            match all.iter_mut().find(|p| p.0 == i) {
                Some(p) => p.1 += v,
                None => all.push((i, v)),
            }
        }
        let s: f32 = all.iter().map(|p| p.1).sum();
        all.retain(|p| p.1 > 0.0);
        all.iter_mut().for_each(|p| p.1 /= s.max(1e-9));
        all
    }
    /// The share of what a brush takes that is dirt.
    pub fn dirt_share(&self) -> f32 {
        let d = self.dirt_volume();
        d / (self.volume() + d).max(1e-9)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Board {
    pub heaps: Vec<Heap>,
    pub next_id: u64,
    /// How dirty the painter keeps the board, 0 (clean, as before) .. 1.
    pub dirty: f32,
    /// The tubes set out, if the painter set out a limited palette.
    pub set_out: Option<Vec<usize>>,
    /// The paint the hand last carried from the board: fractions and medium.
    pub last: Option<(Vec<(usize, f32)>, f32)>,
    /// The mixing area's smears: fractions of tubes, and how much there is (0..1).
    pub residue: Vec<(usize, f32)>,
    pub residue_amount: f32,
}

/// `a` moved toward `b` by `t` (both fractions summing to 1).
fn blend(a: &[(usize, f32)], b: &[(usize, f32)], t: f32) -> Vec<(usize, f32)> {
    let mut out: Vec<(usize, f32)> = a.iter().map(|&(i, f)| (i, f * (1.0 - t))).collect();
    for &(i, f) in b {
        match out.iter_mut().find(|p| p.0 == i) {
            Some(p) => p.1 += f * t,
            None => out.push((i, f * t)),
        }
    }
    out.retain(|p| p.1 > 1e-6);
    out
}

impl Board {
    /// Knife a new heap of these parts (fractions summing to 1; the parts'
    /// sum as given). Returns its id. On a dirty board the mixing area's
    /// smears seep into it.
    pub fn knife(&mut self, parts: Vec<(usize, f32)>, given_sum: f32, medium: f32, solvent: f32, oil_rate: f32, name: Option<String>) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let mut h = Heap { id, name, parts, given_sum, medium, solvent, oil_rate, dirt: Vec::new(), changed: false, added: false, scraped: false };
        if self.dirty > 0.0 && self.residue_amount > 0.0 {
            let v = SEEP * self.dirty * self.residue_amount;
            h.dirt = self.residue.iter().map(|&(i, f)| (i, f * v)).collect();
            h.changed = true;
        }
        self.heaps.push(h);
        self.scrape_old();
        id
    }

    fn scrape_old(&mut self) {
        let live = self.heaps.iter().filter(|h| !h.scraped).count();
        if live > LIVE {
            let mut extra = live - LIVE;
            for h in self.heaps.iter_mut() {
                if extra == 0 {
                    break;
                }
                if !h.scraped {
                    h.scraped = true;
                    extra -= 1;
                }
            }
        }
        // the record of scraped heaps is kept only while something could name them
        if self.heaps.len() > 4 * LIVE {
            let n = self.heaps.len() - 4 * LIVE;
            let mut k = 0;
            self.heaps.retain(|h| {
                if h.scraped && k < n {
                    k += 1;
                    false
                } else {
                    true
                }
            });
        }
    }

    /// A scraped heap `id` knifed again fresh: its own paint (and what was
    /// added to it), no dirt, back on the board as the newest heap. Nothing
    /// for a heap on the board.
    fn revive(&mut self, id: u64) {
        if let Some(h) = self.heap_mut(id)
            && h.scraped
        {
            h.scraped = false;
            h.dirt.clear();
            h.changed = h.added;
            let pos = self.heaps.iter().position(|h| h.id == id).unwrap();
            let h = self.heaps.remove(pos);
            self.heaps.push(h);
            self.scrape_old();
        }
    }

    pub fn heap(&self, id: u64) -> Option<&Heap> {
        self.heaps.iter().find(|h| h.id == id)
    }
    pub fn heap_mut(&mut self, id: u64) -> Option<&mut Heap> {
        self.heaps.iter_mut().find(|h| h.id == id)
    }

    /// The hand goes to heap `id` with a brush that carries `carry` (0..1) of
    /// the last paint: on a dirty board it leaves some of that paint in the
    /// heap; the hand then carries this heap's paint, and the mixing area's
    /// smears follow it. A scraped heap is knifed again, fresh. Nothing
    /// happens on a clean board.
    pub fn visit(&mut self, id: u64, carry: f32) {
        self.revive(id);
        if self.dirty <= 0.0 {
            return;
        }
        let dirty = self.dirty;
        let last = self.last.clone();
        let Some(h) = self.heap_mut(id) else { return };
        if let Some((lp, _)) = &last {
            let own = h.fractions();
            // what differs from the heap is what dirties it
            let differs: f32 = lp.iter().map(|&(i, f)| (f - own.iter().find(|p| p.0 == i).map(|p| p.1).unwrap_or(0.0)).max(0.0)).sum();
            if differs > 0.01 {
                let room = (DIRT_CAP * h.volume() - h.dirt_volume()).max(0.0);
                let v = (VISIT * dirty * carry.clamp(0.0, 1.0) * h.volume()).min(room);
                if v > 0.0 {
                    for &(i, f) in lp {
                        match h.dirt.iter_mut().find(|p| p.0 == i) {
                            Some(p) => p.1 += f * v,
                            None => h.dirt.push((i, f * v)),
                        }
                    }
                    h.changed = true;
                }
            }
        }
        let fr = h.fractions();
        let medium = h.medium;
        self.residue = if self.residue_amount <= 0.0 { fr.clone() } else { blend(&self.residue, &fr, RESIDUE_FOLLOW) };
        self.residue_amount = (self.residue_amount + RESIDUE_FOLLOW).min(1.0);
        self.last = Some((fr, medium));
    }

    /// Knife more tube paint into heap `id`: `add` in the heap's own parts
    /// (as its recipe was given), with the added paint's `medium` (0: from the tube).
    /// The added paint is tube paint: linseed, no turpentine. (A pile's
    /// `thinner` is the pile's, not the heap's: the added paint is thinned
    /// with the rest, so p:add keeps it.) A scraped heap is knifed again
    /// first, as a brush finds it.
    pub fn add(&mut self, id: u64, add: &[(usize, f32)], medium: f32) -> Result<(), String> {
        self.revive(id);
        let h = self.heap_mut(id).ok_or("p:add: that pile is no longer on the palette")?;
        let k = 1.0 / h.given_sum.max(1e-9);
        let before = h.volume();
        let mut added = 0.0;
        for &(i, v) in add {
            let v = v * k;
            added += v;
            match h.parts.iter_mut().find(|p| p.0 == i) {
                Some(p) => p.1 += v,
                None => h.parts.push((i, v)),
            }
        }
        let whole = (before + added).max(1e-9);
        h.medium = (h.medium * before + medium * added) / whole;
        // (no turpentine and linseed's rate in the added paint; a pile of
        // neither, as before engine 4, stays exactly 0 and 1)
        h.solvent = h.solvent * before / whole;
        h.oil_rate = (h.oil_rate * before + added) / whole;
        h.changed = true;
        h.added = true;
        Ok(())
    }

    /// Scrape the mixing area clean and skim the heaps' dirty tops.
    pub fn clean(&mut self) {
        self.residue.clear();
        self.residue_amount = 0.0;
        self.last = None;
        for h in self.heaps.iter_mut() {
            if !h.dirt.is_empty() {
                h.dirt.iter_mut().for_each(|p| p.1 *= 0.3);
                h.changed = true;
            }
        }
    }

    /// The heaps on the board now, oldest first.
    pub fn live(&self) -> Vec<&Heap> {
        self.heaps.iter().filter(|h| !h.scraped).collect()
    }

    /// A line about each live heap: its name, recipe and how dirty it is.
    pub fn describe(&self, tubes: &Palette) -> String {
        let mut s = format!(
            "palette: {} heap(s); dirty {}{}\n",
            self.live().len(),
            self.dirty,
            match &self.set_out {
                Some(v) => format!("; set out: {}", v.iter().map(|&i| tubes.tubes[i].name).collect::<Vec<_>>().join(", ")),
                None => String::new(),
            }
        );
        for (n, h) in self.live().iter().enumerate() {
            let recipe = h.fractions().iter().filter(|p| p.1 >= 0.005).map(|&(i, f)| format!("{} {:.2}", tubes.tubes[i].name, f)).collect::<Vec<_>>().join(" + ");
            s += &format!(
                "  {}. {}{}: {}{}\n",
                n + 1,
                h.name.clone().unwrap_or_else(|| format!("pile {}", h.id)),
                if h.medium > 0.0 { format!(" (medium {:.2})", h.medium) } else if h.medium < 0.0 { format!(" (blotted {:.2})", -h.medium) } else { String::new() },
                recipe,
                if h.dirt_share() > 0.005 { format!("  [{:.0}% dirt]", 100.0 * h.dirt_share()) } else { String::new() }
            );
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_clean_board_leaves_heaps_as_knifed() {
        let mut b = Board::default();
        let a = b.knife(vec![(0, 0.7), (1, 0.3)], 1.0, 0.0, 0.0, 1.0, None);
        let c = b.knife(vec![(2, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        for _ in 0..10 {
            b.visit(a, 1.0);
            b.visit(c, 1.0);
        }
        assert!(!b.heap(a).unwrap().changed && !b.heap(c).unwrap().changed);
        assert!(b.heap(a).unwrap().dirt.is_empty());
    }

    #[test]
    fn a_dirty_board_carries_paint_between_heaps_up_to_a_cap() {
        let mut b = Board { dirty: 1.0, ..Default::default() };
        let a = b.knife(vec![(0, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        let c = b.knife(vec![(1, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        b.visit(a, 1.0);
        b.visit(c, 1.0);
        let h = b.heap(c).unwrap();
        assert!(h.dirt.iter().any(|p| p.0 == 0), "the brush from heap a left some of a in c");
        for _ in 0..500 {
            b.visit(a, 1.0);
            b.visit(c, 1.0);
        }
        assert!(b.heap(c).unwrap().dirt_share() <= DIRT_CAP / (1.0 + DIRT_CAP) + 1e-4);
        b.clean();
        assert!(b.heap(c).unwrap().dirt_share() < 0.2);
    }

    #[test]
    fn adding_to_a_heap_follows_its_recipes_units() {
        let mut b = Board::default();
        // knifed as {"white", 4}, {"blue", 1}: fractions 0.8, 0.2, given sum 5
        let a = b.knife(vec![(0, 0.8), (1, 0.2)], 5.0, 0.0, 0.0, 1.0, None);
        b.add(a, &[(1, 1.0)], 0.0).unwrap();
        let f = b.heap(a).unwrap().fractions();
        let blue = f.iter().find(|p| p.0 == 1).unwrap().1;
        assert!((blue - 2.0 / 6.0).abs() < 1e-5, "4 white + 2 blue: a third blue ({blue})");
    }

    #[test]
    fn added_tube_paint_dilutes_the_turps_and_the_oil() {
        let mut b = Board::default();
        let a = b.knife(vec![(0, 1.0)], 1.0, 0.0, 0.4, 0.6, None);
        b.add(a, &[(0, 1.0)], 0.0).unwrap();
        let h = b.heap(a).unwrap();
        assert!((h.solvent - 0.2).abs() < 1e-6 && (h.oil_rate - 0.8).abs() < 1e-6, "{} {}", h.solvent, h.oil_rate);
    }

    #[test]
    fn adding_to_a_scraped_heap_knifes_it_again() {
        let mut b = Board { dirty: 1.0, ..Default::default() };
        let first = b.knife(vec![(0, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        let other = b.knife(vec![(1, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        b.visit(other, 1.0);
        b.visit(first, 1.0);
        assert!(!b.heap(first).unwrap().dirt.is_empty());
        for _ in 0..LIVE {
            b.knife(vec![(1, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        }
        assert!(b.heap(first).unwrap().scraped);
        b.add(first, &[(0, 1.0)], 0.0).unwrap();
        let h = b.heap(first).unwrap();
        assert!(!h.scraped && h.dirt.is_empty());
        assert_eq!(b.heaps.last().unwrap().id, first, "back on the board as the newest heap");
        assert_eq!(b.live().len(), LIVE);
    }

    #[test]
    fn the_oldest_heap_is_scraped_and_knifed_again_when_used() {
        let mut b = Board::default();
        let first = b.knife(vec![(0, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        for _ in 0..LIVE {
            b.knife(vec![(1, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        }
        assert!(b.heap(first).unwrap().scraped);
        b.visit(first, 1.0);
        assert!(!b.heap(first).unwrap().scraped);
        assert_eq!(b.live().len(), LIVE);
    }

    /// A heap scraped and knifed again on a clean board is the pile exactly as
    /// knifed (a recomputed mixture would differ in the last bits and change
    /// every replay after it).
    #[test]
    fn a_heap_knifed_again_is_unchanged() {
        let mut b = Board::default();
        let first = b.knife(vec![(0, 0.7), (1, 0.2), (2, 0.1)], 1.0, 0.0, 0.0, 1.0, None);
        for _ in 0..LIVE {
            b.knife(vec![(1, 1.0)], 1.0, 0.0, 0.0, 1.0, None);
        }
        b.visit(first, 1.0);
        assert!(!b.heap(first).unwrap().changed);
    }
}
