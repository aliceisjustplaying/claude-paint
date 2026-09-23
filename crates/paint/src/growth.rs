//! How trees grow: a botanical growth model that returns a skeleton (limbs
//! as polylines with widths), not paint. What a tree looks like when painted
//! belongs to the painter; this module only knows how wood grows.
//!
//! The model follows Palubicki et al., "Self-organizing tree models for image
//! synthesis" (SIGGRAPH 2009), in 3D, projected onto the picture plane:
//!
//! - a tree is a hierarchy of internodes (metamers) ending in nodes that
//!   carry buds: one apical bud at the tip of each shoot and lateral buds in
//!   the leaf axils, placed by phyllotaxy (a spiral divergence angle, or
//!   whorls at the end of each annual shoot as in spruce);
//! - every year each bud gets light from a shadow grid (shadow propagation:
//!   foliage casts a pyramid of shade below it), light is summed from the
//!   buds to the base, and the tree's vigor is handed back out from the base
//!   by the Borchert–Honda rule: at each fork the continuing axis gets the
//!   share `λ·Qm / (λ·Qm + (1−λ)·ΣQl)` (apical dominance `λ`);
//! - a bud with enough vigor breaks and grows a shoot of several internodes;
//!   each new internode keeps its direction, turns toward the least shade
//!   (phototropism), up or down by gravitropism (per branch order: leaders
//!   up, spruce limbs level, birch twigs weeping), and wanders a little;
//! - when a terminal bud aborts (as oak terminal buds often do) the strongest
//!   of the buds clustered below it takes over: sympodial growth, the
//!   zig-zag of old oak limbs; dormant buds on old wood may break later
//!   (reiteration);
//! - branches that earn too little light for their size are shed, leaving a
//!   stub on the parent if they were thick;
//! - widths follow the pipe model (Leonardo's rule): `w^k = Σ w_child^k`
//!   with `k` about 2–2.5, from equal twigs up; wood never shrinks, so a
//!   limb that lost its twigs stays as thick as it grew.
//!
//! A dead or dying tree is the same tree after decline: branches die (the
//! top first: stag-headed), dead wood loses its twigs year by year leaving
//! short claws, and dead limbs break, ending in a jagged break.
//!
//! Limbs of the returned skeleton follow the thickest path through each fork
//! (the way the eye reads a limb), so a sympodial limb is one zig-zag
//! polyline. Species are data (`Habit`), not code paths. Deterministic by
//! seed.

use crate::canvas::Frame;
use crate::mask::Mask;
use crate::rng::Rng;
use crate::shape::Shape;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

impl V3 {
    const UP: V3 = V3 { x: 0.0, y: 1.0, z: 0.0 };
    fn new(x: f32, y: f32, z: f32) -> Self {
        V3 { x, y, z }
    }
    fn add(self, o: V3) -> V3 {
        V3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    fn sub(self, o: V3) -> V3 {
        V3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    fn mul(self, k: f32) -> V3 {
        V3::new(self.x * k, self.y * k, self.z * k)
    }
    fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    fn cross(self, o: V3) -> V3 {
        V3::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }
    fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn norm(self) -> V3 {
        let l = self.len();
        if l > 1e-9 { self.mul(1.0 / l) } else { V3::UP }
    }
    /// Two unit vectors perpendicular to `self` (unit) and to each other.
    fn basis(self) -> (V3, V3) {
        let h = if self.y.abs() < 0.9 { V3::UP } else { V3::new(1.0, 0.0, 0.0) };
        let a = self.cross(h).norm();
        (a, self.cross(a).norm())
    }
}

/// A species' way of growing, as numbers. See `Habit::oak`, `dead_oak`,
/// `birch`, `spruce`; tweak fields for individuals.
#[derive(Clone, Debug)]
pub struct Habit {
    /// Years of growth.
    pub years: u32,
    /// Apical dominance λ (0.5 = none, 0.7 = strong leader).
    pub apical: f32,
    /// Vigor per unit of light gathered (sets how fast the tree grows).
    pub vigor: f32,
    /// Most internodes a shoot can make in one year.
    pub shoot_max: u32,
    /// Internode length relative to the first year's (young shoots on high
    /// orders are shorter: length × `shorten^order`).
    pub shorten: f32,
    /// Angle between a lateral bud and its parent axis (radians).
    pub branch_angle: f32,
    /// Phyllotactic divergence between successive nodes (radians; 2.4 ≈
    /// 137.5° spiral, π = distichous).
    pub divergence: f32,
    /// Lateral buds in each leaf axil along a shoot (0 = none).
    pub node_buds: u32,
    /// Extra lateral buds clustered at the end of each shoot (oak's cluster,
    /// spruce's whorl), for the leader and for side shoots.
    pub tip_buds: (u32, u32),
    /// Probability per year that a terminal bud aborts and the buds below it
    /// take over (sympodial growth: zig-zag limbs).
    pub abort: f32,
    /// Years a lateral bud stays able to break.
    pub bud_life: u32,
    /// Probability a bud stays dormant indefinitely instead (reiteration:
    /// epicormic shoots on old wood).
    pub dormant: f32,
    /// Gravitropism by branch order 0, 1, 2, 3+: + bends shoots up, − down
    /// (weeping), 0 keeps their angle.
    pub tropism: [f32; 4],
    /// How strongly shoots turn toward the least shade.
    pub photo: f32,
    /// Shade tolerance: the shadow at which a bud gets no light (oak and
    /// birch need light, spruce grows on in shade).
    pub shade: f32,
    /// How much a shoot wanders, per internode.
    pub wander: f32,
    /// 0 = a round crown, 1 = grown in the picture plane only.
    pub flat: f32,
    /// Branches earning less light than this per internode are shed.
    pub shed: f32,
    /// Shed branches thicker than this (fraction of the parent) leave a stub.
    pub stub: f32,
    /// Trunk width at the base / tree height.
    pub trunk: f32,
    /// Width of the youngest twigs / tree height. The pipe-model exponent
    /// `k` (`w^k = Σ w_child^k`) is solved from trunk, twig and the number
    /// of tips; it is reported as `Skeleton::pipe`.
    pub twig: f32,
    /// Root flare: extra width at the foot.
    pub flare: f32,
    /// Buttress roots spreading into the ground.
    pub roots: u32,
    /// Lean of the young trunk (radians, + = right).
    pub lean: f32,
    /// Decline after growth: 0 = alive, 1 = long dead (fraction of limbs
    /// dead, the top first).
    pub decline: f32,
    /// How much of the fine wood dead limbs have lost (0..1).
    pub decay: f32,
    /// Probability a dead limb has broken off somewhere along it; a quarter
    /// of that for live limbs (storms).
    pub breakage: f32,
}

impl Habit {
    /// An open-grown oak: low apical control, twisting sympodial limbs,
    /// clustered buds, a broad crown on a short thick bole.
    pub fn oak() -> Self {
        Habit {
            years: 24,
            apical: 0.53,
            vigor: 2.2,
            shoot_max: 4,
            shorten: 0.9,
            branch_angle: 1.0,
            divergence: 2.51,
            node_buds: 1,
            tip_buds: (2, 1),
            abort: 0.5,
            bud_life: 2,
            dormant: 0.03,
            tropism: [0.2, 0.0, -0.03, -0.06],
            photo: 0.45,
            shade: 3.2,
            wander: 0.16,
            flat: 0.55,
            shed: 0.15,
            stub: 0.5,
            trunk: 0.06,
            twig: 0.0012,
            flare: 0.25,
            roots: 3,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.08,
        }
    }

    /// Friedrich's dead oak: the same oak long dead, stag-headed, the fine
    /// wood gone, big limbs broken.
    pub fn dead_oak() -> Self {
        Habit { decline: 0.8, decay: 0.6, breakage: 0.35, ..Self::oak() }
    }

    /// A birch: slender, a clear leader, fine twigs hanging from the outer
    /// limbs.
    pub fn birch() -> Self {
        Habit {
            years: 20,
            apical: 0.6,
            vigor: 2.4,
            shoot_max: 5,
            shorten: 0.85,
            branch_angle: 0.7,
            divergence: 2.4,
            node_buds: 1,
            tip_buds: (1, 1),
            abort: 0.05,
            bud_life: 1,
            dormant: 0.0,
            tropism: [0.6, 0.12, -0.25, -0.5],
            photo: 0.3,
            shade: 3.0,
            wander: 0.18,
            flat: 0.5,
            shed: 0.14,
            stub: 0.5,
            trunk: 0.03,
            twig: 0.0008,
            flare: 0.1,
            roots: 0,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.03,
        }
    }

    /// A spruce: a strong straight leader, whorls of level limbs at the end
    /// of each year's shoot, their tips turning up, side shoots flat.
    pub fn spruce() -> Self {
        Habit {
            years: 26,
            apical: 0.8,
            vigor: 2.0,
            shoot_max: 3,
            shorten: 0.7,
            branch_angle: 1.35,
            divergence: 2.4,
            node_buds: 0,
            tip_buds: (5, 2),
            abort: 0.0,
            bud_life: 1,
            dormant: 0.0,
            tropism: [1.2, -0.03, -0.1, -0.2],
            photo: 0.08,
            shade: 6.0,
            wander: 0.08,
            flat: 0.4,
            shed: 0.08,
            stub: 0.6,
            trunk: 0.035,
            twig: 0.0012,
            flare: 0.25,
            roots: 0,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.0,
        }
    }

    /// Grow a tree standing at `base` (canvas units, the foot of the trunk),
    /// `height` units tall.
    pub fn grow(&self, base: (f32, f32), height: f32, seed: u64) -> Skeleton {
        Grower::new(self, seed).run().skeleton(self, base, height, seed)
    }
}

/// One limb as the eye reads it: from where it springs to its tip.
#[derive(Clone, Debug)]
pub struct Limb {
    /// Canvas units, base to tip.
    pub pts: Vec<(f32, f32)>,
    /// Depth at each point (units, + toward the viewer).
    pub z: Vec<f32>,
    /// Full width at each point (units).
    pub w: Vec<f32>,
    /// 0 = trunk, 1 = a main limb, 2 = its branches, ...
    pub order: u32,
    /// Index of the limb this one springs from, and the index into its
    /// `pts` where it does.
    pub parent: Option<usize>,
    pub at: usize,
    /// Dead wood all along, from where it springs. A limb can also die
    /// part way: see `dead_from`.
    pub dead: bool,
    /// Index into `pts` from which the limb is dead wood to its tip (the
    /// top of a stag-headed leader, a live limb's dead end): segments
    /// `pts[i]..pts[i + 1]` with `i >= dead_from` are dead. 0 when `dead`;
    /// `pts.len()` when it is alive throughout.
    pub dead_from: usize,
    /// Ends in a break (jagged, blunt) rather than a growing tip.
    pub broken: bool,
    /// A buttress root running into the ground.
    pub root: bool,
}

impl Limb {
    pub fn len(&self) -> f32 {
        self.pts.windows(2).map(|p| dist(p[0], p[1])).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.pts.len() < 2
    }
    /// Whether the segment from `pts[i]` to `pts[i + 1]` is dead wood.
    pub fn dead_at(&self, i: usize) -> bool {
        i >= self.dead_from
    }
    /// Direction of travel at point `i` (unit).
    pub fn dir(&self, i: usize) -> (f32, f32) {
        let n = self.pts.len();
        let a = self.pts[i.saturating_sub(1)];
        let b = self.pts[(i + 1).min(n - 1)];
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let d = (dx * dx + dy * dy).sqrt().max(1e-6);
        (dx / d, dy / d)
    }
}

/// A grown tree: limbs ordered parents first (trunk, main limbs, ... twigs).
#[derive(Clone, Debug)]
pub struct Skeleton {
    pub limbs: Vec<Limb>,
    pub base: (f32, f32),
    pub height: f32,
    /// The pipe-model exponent the widths follow (Leonardo's rule).
    pub pipe: f32,
}

impl Skeleton {
    /// Silhouette mask of the limbs (for glazes, mist, clipping).
    pub fn mask(&self, f: Frame) -> Mask {
        let mut shape = Shape::new();
        for l in self.limbs.iter().filter(|l| !l.is_empty()) {
            shape = shape.ribbon(&l.pts, &l.w);
        }
        Mask::from_shape(f, shape)
    }

    /// Tips of live twigs (where birds perch, where snow does not lie).
    pub fn tips(&self) -> Vec<(f32, f32)> {
        self.limbs.iter().filter(|l| !l.broken && !l.root && !l.is_empty()).map(|l| *l.pts.last().unwrap()).collect()
    }

    /// Bounding box (x0, y0, x1, y1) in canvas units.
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for l in &self.limbs {
            for p in &l.pts {
                b = (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1));
            }
        }
        b
    }
}

#[derive(Clone, Debug)]
struct Bud {
    dir: V3,
    apical: bool,
    born: u32,
    q: f32,
    v: f32,
    /// Stays able to break past `bud_life` (dormant, for reiteration).
    sleeper: bool,
}

#[derive(Clone, Debug)]
struct Node {
    p: V3,
    parent: usize,
    /// Continues its parent's shoot (not the first node of a side branch).
    main: bool,
    kids: Vec<usize>,
    order: u32,
    born: u32,
    /// Phyllotactic phase of the next lateral bud on this axis.
    phase: f32,
    buds: Vec<Bud>,
    alive: bool,
    dead: bool,
    broken: bool,
    /// Largest number of tips the subtree ever carried (wood never shrinks).
    wood: f32,
    q: f32,
    v: f32,
    count: f32,
}

const NONE: usize = usize::MAX;
const G: usize = 160;

struct Grid {
    s: Vec<f32>,
    /// World position of cell (0, 0, 0).
    o: V3,
}

impl Grid {
    fn idx(&self, p: V3) -> Option<usize> {
        let (i, j, k) = ((p.x - self.o.x).floor(), (p.y - self.o.y).floor(), (p.z - self.o.z).floor());
        if i < 0.0 || j < 0.0 || k < 0.0 || i >= G as f32 || j >= G as f32 || k >= G as f32 {
            return None;
        }
        Some((j as usize * G + k as usize) * G + i as usize)
    }
    fn at(&self, p: V3) -> f32 {
        self.idx(p).map(|i| self.s[i]).unwrap_or(0.0)
    }
    /// Foliage at `p` shades a pyramid below it.
    fn cast(&mut self, p: V3, sign: f32) {
        let (ci, cj, ck) = ((p.x - self.o.x).floor() as i32, (p.y - self.o.y).floor() as i32, (p.z - self.o.z).floor() as i32);
        for q in 0..6i32 {
            let j = cj - q;
            if j < 0 || j >= G as i32 {
                continue;
            }
            let a = sign * 1.0 * 0.55f32.powi(q);
            for dk in -q..=q {
                let k = ck + dk;
                if k < 0 || k >= G as i32 {
                    continue;
                }
                for di in -q..=q {
                    let i = ci + di;
                    if i < 0 || i >= G as i32 {
                        continue;
                    }
                    self.s[(j as usize * G + k as usize) * G + i as usize] += a;
                }
            }
        }
    }
}


struct Grower<'a> {
    h: &'a Habit,
    nodes: Vec<Node>,
    rng: Rng,
    grid: Grid,
    year: u32,
    /// Stubs of shed branches: (parent node, direction, wood).
    stubs: Vec<(usize, V3, f32)>,
}

impl<'a> Grower<'a> {
    fn new(h: &'a Habit, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x9e37_79b9_7f4a_7c15);
        let up = V3::new(h.lean.sin(), h.lean.cos(), 0.0);
        let root = Node {
            p: V3::default(),
            parent: NONE,
            main: true,
            kids: vec![],
            order: 0,
            born: 0,
            phase: rng.range(0.0, 2.0 * PI),
            buds: vec![Bud { dir: up, apical: true, born: 0, q: 0.0, v: 0.0, sleeper: false }],
            alive: true,
            dead: false,
            broken: false,
            wood: 1.0,
            q: 0.0,
            v: 0.0,
            count: 0.0,
        };
        let grid = Grid { s: vec![0.0; G * G * G], o: V3::new(-(G as f32) * 0.5, -4.0, -(G as f32) * 0.5) };
        Grower { h, nodes: vec![root], rng, grid, year: 0, stubs: vec![] }
    }

    fn run(mut self) -> Self {
        for y in 1..=self.h.years {
            self.year = y;
            self.step();
        }
        self
    }

    fn live(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.nodes.len()).filter(|&i| self.nodes[i].alive)
    }

    fn step(&mut self) {
        let h = self.h.clone();
        // shade: foliage on the young wood (the last three years' shoots)
        self.grid.s.iter_mut().for_each(|s| *s = 0.0);
        let leafy: Vec<V3> = self.live().filter(|&i| self.nodes[i].born + 3 >= self.year).map(|i| self.nodes[i].p).collect();
        for p in &leafy {
            self.grid.cast(*p, 1.0);
        }
        // light at each bud (its own foliage excluded)
        for i in 0..self.nodes.len() {
            if !self.nodes[i].alive {
                continue;
            }
            let p = self.nodes[i].p;
            let own = if self.nodes[i].born + 3 >= self.year { 1.0 } else { 0.0 };
            for b in self.nodes[i].buds.iter_mut() {
                let s = self.grid.at(p.add(b.dir.mul(0.7)));
                b.q = (1.0 - (s - own).max(0.0) / h.shade).clamp(0.0, 1.0);
            }
        }
        // terminal buds abort now and then; lateral buds die when old
        let year = self.year;
        for i in 0..self.nodes.len() {
            if !self.nodes[i].alive {
                continue;
            }
            let order = self.nodes[i].order;
            let abort = if order == 0 { h.abort * 0.5 } else { h.abort };
            let mut buds = std::mem::take(&mut self.nodes[i].buds);
            // a terminal bud aborts only where buds below can take over
            let heirs = buds.iter().any(|b| !b.apical);
            buds.retain(|b| {
                if b.apical {
                    !(heirs && self.rng.chance(abort))
                } else {
                    b.sleeper || year <= b.born + h.bud_life
                }
            });
            self.nodes[i].buds = buds;
        }
        // light gathered, tips to base
        for i in (0..self.nodes.len()).rev() {
            if !self.nodes[i].alive {
                continue;
            }
            let own: f32 = self.nodes[i].buds.iter().map(|b| b.q).sum();
            let kids: f32 = self.nodes[i].kids.iter().map(|&k| self.nodes[k].q).sum();
            let cnt: f32 = 1.0 + self.nodes[i].kids.iter().map(|&k| self.nodes[k].count).sum::<f32>();
            self.nodes[i].q = own + kids;
            self.nodes[i].count = cnt;
        }
        // shed branches that do not pay their way
        for i in 1..self.nodes.len() {
            let n = &self.nodes[i];
            if !n.alive || n.main || n.born + 3 > self.year || n.count < 3.0 {
                continue;
            }
            if n.q / n.count < h.shed {
                let (par, dir, wood) = (n.parent, n.p.sub(self.nodes[n.parent].p).norm(), n.wood);
                self.stubs.push((par, dir, wood));
                self.kill(i);
                self.nodes[par].kids.retain(|&k| k != i);
            }
        }
        // vigor, base to tips (Borchert–Honda)
        self.nodes[0].v = h.vigor * self.nodes[0].q;
        let lam = h.apical;
        for i in 0..self.nodes.len() {
            if !self.nodes[i].alive {
                continue;
            }
            let v = self.nodes[i].v;
            let main_kid = self.nodes[i].kids.iter().copied().find(|&k| self.nodes[k].main);
            let qm = match main_kid {
                Some(k) => self.nodes[k].q,
                None => self.nodes[i].buds.iter().filter(|b| b.apical).map(|b| b.q).sum(),
            };
            let ql: f32 = self.nodes[i].kids.iter().filter(|&&k| !self.nodes[k].main).map(|&k| self.nodes[k].q).sum::<f32>()
                + self.nodes[i].buds.iter().filter(|b| !b.apical).map(|b| b.q).sum::<f32>();
            let d = lam * qm + (1.0 - lam) * ql;
            if d <= 1e-9 {
                continue;
            }
            let kids = self.nodes[i].kids.clone();
            for k in kids {
                let share = if self.nodes[k].main { lam } else { 1.0 - lam };
                self.nodes[k].v = v * share * self.nodes[k].q / d;
            }
            for b in self.nodes[i].buds.iter_mut() {
                let share = if b.apical { lam } else { 1.0 - lam };
                b.v = v * share * b.q / d;
            }
        }
        // buds with vigor enough break into shoots
        let n0 = self.nodes.len();
        for i in 0..n0 {
            if !self.nodes[i].alive || self.nodes[i].buds.is_empty() {
                continue;
            }
            let buds = std::mem::take(&mut self.nodes[i].buds);
            let mut keep = vec![];
            for b in buds {
                let n = (b.v.floor() as u32).min(h.shoot_max);
                if n == 0 {
                    keep.push(b);
                    continue;
                }
                self.shoot(i, &b, n);
            }
            self.nodes[i].buds = keep;
        }
        // wood: tips carried, never shrinking
        for i in (0..self.nodes.len()).rev() {
            if !self.nodes[i].alive {
                continue;
            }
            let tips = if self.nodes[i].kids.is_empty() { 1.0 } else { self.nodes[i].kids.iter().map(|&k| self.nodes[k].wood).sum() };
            self.nodes[i].wood = self.nodes[i].wood.max(tips);
        }
    }

    fn kill(&mut self, i: usize) {
        let mut stack = vec![i];
        while let Some(j) = stack.pop() {
            self.nodes[j].alive = false;
            stack.extend(self.nodes[j].kids.iter().copied());
        }
    }

    /// Grow a shoot of `n` internodes from bud `b` on node `from`.
    fn shoot(&mut self, from: usize, b: &Bud, n: u32) {
        let h = self.h;
        let order = if b.apical { self.nodes[from].order } else { self.nodes[from].order + 1 };
        let trop = h.tropism[(order as usize).min(3)];
        let len = h.shorten.powi(order as i32);
        let mut d = b.dir;
        let mut prev = from;
        let mut phase = if b.apical { self.nodes[from].phase } else { self.rng.range(0.0, 2.0 * PI) };
        for m in 0..n {
            let p0 = self.nodes[prev].p;
            if m > 0 || b.apical {
                // phototropism: down the shade gradient
                let e = 1.0;
                let gx = self.grid.at(p0.add(V3::new(e, 0.0, 0.0))) - self.grid.at(p0.sub(V3::new(e, 0.0, 0.0)));
                let gy = self.grid.at(p0.add(V3::new(0.0, e, 0.0))) - self.grid.at(p0.sub(V3::new(0.0, e, 0.0)));
                let gz = self.grid.at(p0.add(V3::new(0.0, 0.0, e))) - self.grid.at(p0.sub(V3::new(0.0, 0.0, e)));
                let light = V3::new(-gx, -gy, -gz);
                let light = if light.len() > 1e-4 { light.norm() } else { V3::default() };
                let r = V3::new(self.rng.normal(), self.rng.normal(), self.rng.normal()).mul(h.wander);
                d = d.add(light.mul(h.photo)).add(V3::UP.mul(trop)).add(r).norm();
            } else {
                let r = V3::new(self.rng.normal(), self.rng.normal(), self.rng.normal()).mul(h.wander * 0.5);
                d = d.add(r).norm();
            }
            d.z *= 1.0 - h.flat;
            d = d.norm();
            let p = p0.add(d.mul(len));
            let idx = self.nodes.len();
            let node = Node {
                p,
                parent: prev,
                main: m > 0 || b.apical,
                kids: vec![],
                order,
                born: self.year,
                phase,
                buds: vec![],
                alive: true,
                dead: false,
                broken: false,
                wood: 1.0,
                q: 0.0,
                v: 0.0,
                count: 1.0,
            };
            self.nodes.push(node);
            self.nodes[prev].kids.push(idx);
            let last = m + 1 == n;
            let mut lat = h.node_buds;
            if last {
                lat += if order == 0 { h.tip_buds.0 } else { h.tip_buds.1 };
            }
            let (u, w) = d.basis();
            let mut buds = vec![];
            for k in 0..lat {
                // a whorl spreads evenly; single buds follow the spiral
                let az = if last && lat > 1 { phase + k as f32 * 2.0 * PI / lat as f32 } else { phase };
                phase += h.divergence;
                let side = u.mul(az.cos()).add(w.mul(az.sin()));
                let ang = h.branch_angle * self.rng.range(0.8, 1.2);
                let bd = d.mul(ang.cos()).add(side.mul(ang.sin())).norm();
                let sleeper = self.rng.chance(h.dormant);
                buds.push(Bud { dir: bd, apical: false, born: self.year, q: 0.0, v: 0.0, sleeper });
            }
            if last {
                buds.push(Bud { dir: d, apical: true, born: self.year, q: 0.0, v: 0.0, sleeper: false });
            }
            self.nodes[idx].buds = buds;
            self.nodes[idx].phase = phase;
            prev = idx;
        }
    }

    /// Decline after growth: branches die (the top first), dead wood loses
    /// its twigs leaving claws, limbs break.
    fn decline(&mut self, max_y: f32) {
        let h = self.h;
        let n = self.nodes.len();
        let big = self.nodes[0].wood;
        // death spreads from the crown down; whole branches die at once
        if h.decline > 0.0 {
            for i in 1..n {
                if !self.nodes[i].alive {
                    continue;
                }
                let par = self.nodes[i].parent;
                if self.nodes[par].dead {
                    self.nodes[i].dead = true;
                    continue;
                }
                let t = self.nodes[i].p.y / max_y;
                let stag = self.nodes[i].order == 0 && t > 1.0 - 0.3 * h.decline;
                let branch_root = !self.nodes[i].main;
                let p = h.decline * (0.35 + 0.9 * t) * if self.nodes[i].order <= 1 { 0.8 } else { 0.5 };
                if stag || (branch_root && self.rng.chance(p.min(0.97))) {
                    self.nodes[i].dead = true;
                }
            }
            // dead wood loses the fine twigs; some leave a short claw
            let thin = 1.0 + h.decay * h.decay * 0.025 * big;
            for i in 1..n {
                if !self.nodes[i].alive || !self.nodes[i].dead || self.nodes[i].wood >= thin {
                    continue;
                }
                let par = self.nodes[i].parent;
                if self.nodes[par].wood < thin && self.nodes[par].dead {
                    continue; // removed with its parent
                }
                // i is the first thin node: keep a claw of one or two
                // internodes, or lose it altogether
                if self.rng.chance(0.55 * (1.0 - 0.5 * h.decay)) {
                    let mut j = i;
                    let keep = if self.rng.chance(0.5) { 1 } else { 2 };
                    for _ in 1..keep {
                        match self.nodes[j].kids.iter().copied().filter(|&k| self.nodes[k].alive).max_by(|&a, &b| self.nodes[a].wood.total_cmp(&self.nodes[b].wood)) {
                            Some(k) => {
                                // the claw goes on through its strongest
                                // twig; the others are lost
                                let kids = std::mem::replace(&mut self.nodes[j].kids, vec![k]);
                                for s in kids.into_iter().filter(|&s| s != k) {
                                    self.kill(s);
                                }
                                j = k;
                            }
                            None => break,
                        }
                    }
                    let kids = std::mem::take(&mut self.nodes[j].kids);
                    for k in kids {
                        self.kill(k);
                    }
                } else {
                    self.kill(i);
                    self.nodes[par].kids.retain(|&k| k != i);
                    if self.nodes[par].kids.is_empty() {
                        self.nodes[par].broken = true;
                    }
                }
            }
        }
        // limbs break: dead ones often, live ones in storms
        for i in 1..n {
            let nd = &self.nodes[i];
            if !nd.alive || nd.main || nd.wood < 0.01 * big + 3.0 {
                continue;
            }
            let p = if nd.dead { h.breakage } else { h.breakage * 0.25 };
            if !self.rng.chance(p) {
                continue;
            }
            // walk out along the limb and snap it somewhere
            let mut chain = vec![i];
            let mut j = i;
            while let Some(k) = self.nodes[j].kids.iter().copied().max_by(|&a, &b| self.nodes[a].wood.total_cmp(&self.nodes[b].wood)) {
                chain.push(k);
                j = k;
            }
            if chain.len() < 3 {
                continue;
            }
            let at = chain[((self.rng.range(0.15, 0.7) * chain.len() as f32) as usize).max(1)];
            let kids = std::mem::take(&mut self.nodes[at].kids);
            for k in kids {
                self.kill(k);
            }
            self.nodes[at].broken = true;
            self.nodes[at].buds.clear();
            // the break kills what remains of a live limb beyond its forks
            if !self.nodes[at].dead {
                let mut k = at;
                while k != i && self.nodes[k].kids.len() <= 1 {
                    self.nodes[k].dead = true;
                    k = self.nodes[k].parent;
                }
            }
        }
    }

    /// The limbs as chains of nodes, parents first: each follows the
    /// thickest path through every fork (a little in favor of going
    /// straight on). A limb's chain starts at the node it springs from.
    fn chains(&self) -> Vec<Chain> {
        let mut out: Vec<Chain> = vec![];
        let mut queue: std::collections::VecDeque<(usize, Option<usize>, usize, u32)> = std::collections::VecDeque::new();
        queue.push_back((0, None, 0, 0));
        while let Some((start, parent, at, order)) = queue.pop_front() {
            let mut chain = vec![];
            if start != 0 {
                chain.push(self.nodes[start].parent);
            }
            let mut j = start;
            loop {
                chain.push(j);
                let here = self.nodes[j].p.sub(self.nodes[if self.nodes[j].parent == NONE { j } else { self.nodes[j].parent }].p).norm();
                let next = self.nodes[j].kids.iter().copied().filter(|&k| self.nodes[k].alive).max_by(|&a, &b| {
                    let score = |k: usize| self.nodes[k].wood * (1.2 + 0.3 * self.nodes[k].p.sub(self.nodes[j].p).norm().dot(here));
                    score(a).total_cmp(&score(b))
                });
                for k in self.nodes[j].kids.iter().copied().filter(|&k| self.nodes[k].alive && Some(k) != next) {
                    queue.push_back((k, Some(out.len()), chain.len() - 1, order + 1));
                }
                match next {
                    Some(k) => j = k,
                    None => break,
                }
            }
            out.push(Chain { nodes: chain, start, parent, at, order });
        }
        out
    }

    fn skeleton(mut self, h: &Habit, base: (f32, f32), height: f32, seed: u64) -> Skeleton {
        let max_y0 = self.live().map(|i| self.nodes[i].p.y).fold(1e-3, f32::max);
        self.decline(max_y0);
        let max_y = self.live().map(|i| self.nodes[i].p.y).fold(1e-3, f32::max);
        let s = height / max_y;
        let trunk_w = h.trunk * height;
        let tip_w = (h.twig * height).min(trunk_w * 0.5);
        // Leonardo: w^k = Σ w_child^k from equal tips gives w = tip·N^(1/k);
        // k is whatever makes N tips add up to the trunk
        let pk = (self.nodes[0].wood.max(2.0).ln() / (trunk_w / tip_w).ln()).clamp(1.2, 3.5);
        let tip_w = trunk_w / self.nodes[0].wood.powf(1.0 / pk);
        let width = |wood: f32| tip_w * wood.powf(1.0 / pk);
        let proj = |p: V3| (base.0 + p.x * s, base.1 - p.y * s);
        let mut limbs: Vec<Limb> = vec![];
        for Chain { nodes: chain, start, parent, at, order } in self.chains() {
            let tip = *chain.last().unwrap();
            let mut pts = vec![];
            let mut z = vec![];
            let mut w = vec![];
            for (ci, &nd) in chain.iter().enumerate() {
                pts.push(proj(self.nodes[nd].p));
                z.push(self.nodes[nd].p.z * s);
                // width of the internode leaving this point
                let out = chain.get(ci + 1).map(|&k| self.nodes[k].wood).unwrap_or(self.nodes[nd].wood * 0.7);
                let out = if ci == 0 && start != 0 { self.nodes[start].wood } else { out };
                w.push(width(out));
            }
            // a limb never thickens toward its tip
            for i in 1..w.len() {
                w[i] = w[i].min(w[i - 1]);
            }
            if start == 0 {
                // the foot flares into the roots
                for (i, wi) in w.iter_mut().enumerate() {
                    let y = self.nodes[chain[i]].p.y * s;
                    *wi *= 1.0 + h.flare * (-y / (trunk_w * 0.8)).exp();
                }
            }
            // the internode ending at chain[k + 1] is segment k; death
            // spreads outward, so along a limb it is dead from some point on
            let seg_dead: Vec<bool> = chain.windows(2).map(|p| self.nodes[p[1]].dead).collect();
            let dead_from = seg_dead.iter().position(|&d| d).unwrap_or(pts.len());
            debug_assert!(seg_dead[dead_from.min(seg_dead.len())..].iter().all(|&d| d), "live wood beyond dead wood");
            let dead = dead_from == 0;
            limbs.push(Limb { pts, z, w, order, parent, at, dead, dead_from, broken: self.nodes[tip].broken, root: false });
        }
        // stubs of branches shed long ago
        let mut rng = Rng::new(seed ^ 0x51ab);
        for &(par, dir, wood) in &self.stubs {
            if !self.nodes[par].alive {
                continue;
            }
            let pw = width(self.nodes[par].wood);
            let sw = width(wood);
            if sw < h.stub * pw || sw < 0.004 * height {
                continue;
            }
            let p0 = proj(self.nodes[par].p);
            // find the limb point nearest the stub's root
            let (li, pi) = nearest(&limbs, p0);
            let sw = sw.min(limbs[li].w[pi] * 0.8);
            let l = sw * rng.range(1.0, 2.5);
            let d2 = (dir.x, -dir.y);
            let dl = (d2.0 * d2.0 + d2.1 * d2.1).sqrt().max(0.2);
            let p1 = (p0.0 + d2.0 / dl * l, p0.1 + d2.1 / dl * l);
            limbs.push(Limb {
                pts: vec![p0, p1],
                z: vec![self.nodes[par].p.z * s; 2],
                w: vec![sw * 0.9, sw * 0.8],
                order: limbs[li].order + 1,
                parent: Some(li),
                at: pi,
                dead: true,
                dead_from: 0,
                broken: true,
                root: false,
            });
        }
        // buttress roots running out into the ground
        for r in 0..h.roots {
            let side = if r % 2 == 0 { 1.0 } else { -1.0 };
            let a = rng.range(0.25, 0.6); // below horizontal
            let l = trunk_w * rng.range(0.9, 1.8);
            let x0 = base.0 + side * trunk_w * rng.range(0.1, 0.35);
            let y0 = base.1 - trunk_w * rng.range(0.15, 0.35);
            let pts = vec![(x0, y0), (x0 + side * l * 0.5 * a.cos(), y0 + l * 0.5 * a.sin() * 0.8), (x0 + side * l * a.cos(), y0 + l * a.sin())];
            let w0 = trunk_w * rng.range(0.4, 0.6);
            limbs.push(Limb { pts, z: vec![0.0; 3], w: vec![w0, w0 * 0.55, w0 * 0.2], order: 1, parent: Some(0), at: 0, dead: false, dead_from: 3, broken: false, root: true });
        }
        Skeleton { limbs, base, height, pipe: pk }
    }
}

/// One limb's nodes (see `Grower::chains`).
struct Chain {
    nodes: Vec<usize>,
    start: usize,
    parent: Option<usize>,
    at: usize,
    order: u32,
}

fn nearest(limbs: &[Limb], p: (f32, f32)) -> (usize, usize) {
    let mut best = (0, 0, f32::MAX);
    for (li, l) in limbs.iter().enumerate() {
        for (pi, q) in l.pts.iter().enumerate() {
            let d = dist(*q, p);
            if d < best.2 {
                best = (li, pi, d);
            }
        }
    }
    (best.0, best.1)
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_connected() {
        for h in [Habit::oak(), Habit::dead_oak(), Habit::birch(), Habit::spruce()] {
            let a = h.grow((500.0, 900.0), 400.0, 7);
            let b = h.grow((500.0, 900.0), 400.0, 7);
            assert_eq!(a.limbs.len(), b.limbs.len());
            assert!(a.limbs.len() > 20, "too few limbs: {}", a.limbs.len());
            let top = a.bounds().1;
            assert!((900.0 - top - 400.0).abs() < 1.0, "height {}", 900.0 - top);
            for (i, l) in a.limbs.iter().enumerate() {
                assert!(l.w.iter().all(|w| w.is_finite() && *w > 0.0));
                if let Some(p) = l.parent {
                    assert!(p < i, "parents come first");
                    if !l.root {
                        // a limb springs from a point on its parent
                        let q = a.limbs[p].pts[l.at];
                        assert!(dist(q, l.pts[0]) < 1e-3 || l.broken, "limb {i} detached");
                    }
                }
            }
        }
    }

    /// A dying tree as the grower left it (what `skeleton` starts from).
    fn declined(h: &Habit, seed: u64) -> Grower<'_> {
        let mut g = Grower::new(h, seed).run();
        let max_y = g.live().map(|i| g.nodes[i].p.y).fold(1e-3, f32::max);
        g.decline(max_y);
        g
    }

    /// Every exported segment carries its internode's live/dead state: the
    /// stag-headed top of a live leader is dead wood.
    #[test]
    fn limbs_keep_the_dead_wood_along_them() {
        let h = Habit::dead_oak();
        let mut part_dead = 0;
        for seed in [2, 7, 11] {
            let g = declined(&h, seed);
            let chains = g.chains();
            let s = h.grow((0.0, 0.0), 400.0, seed);
            for (i, c) in chains.iter().enumerate() {
                let l = &s.limbs[i];
                assert_eq!(l.pts.len(), c.nodes.len());
                for k in 0..c.nodes.len() - 1 {
                    assert_eq!(l.dead_at(k), g.nodes[c.nodes[k + 1]].dead, "seed {seed} limb {i} segment {k}");
                }
                assert_eq!(l.dead, l.dead_from == 0);
                part_dead += (l.dead_from > 0 && l.dead_from < l.pts.len()) as usize;
            }
            if seed == 7 {
                // the reviewer's case: 20 of the leader's 55 nodes are dead
                let trunk = &s.limbs[0];
                assert!(!trunk.dead && trunk.dead_from < trunk.pts.len(), "{} of {}", trunk.dead_from, trunk.pts.len());
                assert_eq!(trunk.pts.len() - 1 - trunk.dead_from, (1..chains[0].nodes.len()).filter(|&k| g.nodes[chains[0].nodes[k]].dead).count());
            }
        }
        assert!(part_dead > 0);
    }

    /// Dead wood keeps claws of at most two internodes past the width set
    /// by `decay`: no side shoots survive on a claw.
    #[test]
    fn dead_claws_are_short() {
        let h = Habit::dead_oak();
        for seed in [2, 7, 11, 23, 54, 61] {
            let g = declined(&h, seed);
            let thin = 1.0 + h.decay * h.decay * 0.025 * g.nodes[0].wood;
            let is_thin_dead = |i: usize| g.nodes[i].dead && g.nodes[i].wood < thin;
            let mut deepest = 0;
            for i in g.live().filter(|&i| i > 0 && is_thin_dead(i) && !is_thin_dead(g.nodes[i].parent)) {
                let mut stack = vec![(i, 1)];
                while let Some((j, d)) = stack.pop() {
                    deepest = deepest.max(d);
                    stack.extend(g.nodes[j].kids.iter().filter(|&&k| g.nodes[k].alive).map(|&k| (k, d + 1)));
                }
            }
            assert!(deepest <= 2, "seed {seed}: a thin dead subtree {deepest} deep");
        }
    }

    #[test]
    fn pipe_model_widths() {
        let s = Habit::oak().grow((0.0, 0.0), 300.0, 3);
        let trunk = &s.limbs[0];
        assert!(trunk.w[0] > 300.0 * 0.07);
        // children are never wider than the parent where they spring
        for l in s.limbs.iter().skip(1).filter(|l| !l.root) {
            let p = &s.limbs[l.parent.unwrap()];
            assert!(l.w[0] <= p.w[l.at] * 1.3 + 1e-3, "{} > {} (order {} broken {} dead {} at {}/{} parent order {})", l.w[0], p.w[l.at], l.order, l.broken, l.dead, l.at, p.pts.len(), p.order);
        }
    }
}

