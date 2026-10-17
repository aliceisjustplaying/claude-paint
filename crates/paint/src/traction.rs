//! Traction cracks: a fast-setting film parts over paint still soft beneath.
//! Through-hardening and opening use absolute setting dates, so a long wait
//! makes the same damage as shorter waits. Geometry reuses `crack::network`.
//! Rates, thresholds and opening scales are estimates, not a fracture solver.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::drying::{GEL, Pace, TOUCH_DRY_MIN, TOUCH_DRY_MIN_3, rate};
use crate::smoothstep;
use crate::surface::COAT_UM;
use rayon::prelude::*;
use std::io::{self, Read, Write};

const THROUGH: f32 = 6.0;
const SOFT0: f32 = 30.0;
const SOFT1: f32 = 120.0;
const OPEN_MIN: f64 = 30.0 * 24.0 * 60.0;
const FINEST_MM: f32 = 1.6;
const LAST_GEN: u8 = 4;
const GAP_MAX: f32 = 0.2;
const NETWORK_MEMORY_BUDGET: usize = 1024 << 20;
const NETWORK_TILE_MM: f32 = 300.0;
const NETWORK_HALO_MM: f32 = 32.0;
const NETWORK_GRID: f32 = 1.0 / 3.0;
const NETWORK_BLEND_MM: f32 = 4.0;
const NETWORK_EDGE_JITTER_MM: f32 = 8.0;
/// Most regions built at once (each gets an equal share of the budget).
const NETWORK_PARALLEL: usize = 8;
const MARK: u64 = 0x4e4f_4954_4341_5254; // TRACTION
const SECTION_V: u64 = 1;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Soft {
    amount: f64,
    rate: f64,
    at: f64,
}

impl Soft {
    fn valid(self, now: f64) -> bool {
        [self.amount, self.rate, self.at].iter().all(|v| v.is_finite())
            && self.amount >= 0.0
            && self.rate >= 0.0
            && (self.amount == 0.0 || self.rate > 0.0)
            && self.at >= 0.0
            && self.at <= now + 1e-6
    }
    fn at(self, now: f64) -> f32 {
        if self.amount == 0.0 {
            return 0.0;
        }
        (self.amount * (-(now - self.at).max(0.0) * self.rate).exp()) as f32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Surface {
    color: Rgb,
    height: f32,
    film: f32,
    gloss: f32,
    absorb: f32,
}

impl Surface {
    fn capture(c: &Canvas, i: usize) -> Self {
        Self {
            color: c.px[i],
            height: c.height[i],
            film: c.film[i],
            gloss: c.gloss[i],
            absorb: c.absorb[i],
        }
    }
    fn mix(self, under: Self, a: f32) -> Self {
        if a <= 0.0 {
            return self;
        }
        if a >= 1.0 {
            return under;
        }
        let lerp = |t: f32, u: f32| t + (u - t) * a;
        Self {
            color: std::array::from_fn(|k| lerp(self.color[k], under.color[k])),
            height: lerp(self.height, under.height),
            film: lerp(self.film, under.film),
            gloss: lerp(self.gloss, under.gloss),
            absorb: lerp(self.absorb, under.absorb),
        }
    }
    fn apply(self, c: &mut Canvas, i: usize) {
        c.px[i] = self.color;
        c.height[i] = self.height;
        c.film[i] = self.film;
        c.gloss[i] = self.gloss;
        c.absorb[i] = self.absorb;
        c.absorb_any |= self.absorb > 0.0;
    }
    fn values(self) -> [f32; 7] {
        [self.color[0], self.color[1], self.color[2], self.height, self.film, self.gloss, self.absorb]
    }
    fn valid(self) -> bool {
        self.values().iter().all(|v| v.is_finite())
            && self.color.iter().all(|v| (0.0..=1.0).contains(v))
            && self.film >= 0.0
            && (0.0..=1.0).contains(&self.gloss)
            && self.absorb >= 0.0
    }
    fn read(r: &mut impl Read) -> io::Result<Self> {
        let mut q = [0.0; 7];
        for v in &mut q {
            *v = get_f32(r)?;
        }
        let surface = Self {
            color: [q[0], q[1], q[2]],
            height: q[3],
            film: q[4],
            gloss: q[5],
            absorb: q[6],
        };
        if !surface.valid() {
            return Err(bad("checkpoint traction surface is invalid"));
        }
        Ok(surface)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CrackPx {
    i: usize,
    under: Surface,
    top: Surface,
    d_mm: f32,
    hw_mm: f32,
    shown: f32,
    set_at: f64,
    dry_at: f64,
}

impl CrackPx {
    pub(crate) fn index(&self) -> usize {
        self.i
    }
    fn valid(self, n: usize, now: f64) -> bool {
        self.i < n
            && self.under.valid()
            && self.top.valid()
            && [self.d_mm, self.hw_mm, self.shown].iter().all(|v| v.is_finite())
            && self.d_mm >= 0.0
            && self.hw_mm >= 0.0
            && (0.0..=1.0).contains(&self.shown)
            && self.set_at.is_finite()
            && self.dry_at.is_finite()
            && self.set_at >= 0.0
            && self.set_at <= now + 1e-6
            && self.dry_at > self.set_at
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Traction {
    soft: Vec<Soft>,
    pub(crate) cracks: Vec<CrackPx>,
    net: NetCache,
}

impl Traction {
    pub(crate) fn write(&self, w: &mut impl Write, n: usize, now: f64) -> io::Result<()> {
        // Check all state before writing, without copying the canvas-sized arrays.
        if !self.soft.is_empty() && self.soft.len() != n {
            return Err(bad("checkpoint traction softness length is invalid"));
        }
        if self.soft.iter().any(|s| !s.valid(now)) {
            return Err(bad("checkpoint traction softness is invalid"));
        }
        if self.cracks.len() > n {
            return Err(bad("checkpoint traction crack count is invalid"));
        }
        if !self.cracks.is_empty() && self.soft.is_empty() {
            return Err(bad("checkpoint traction cracks have no substrate state"));
        }
        if self.cracks.windows(2).any(|pair| pair[0].i >= pair[1].i) {
            return Err(bad("checkpoint traction crack indices are not ordered"));
        }
        if self.cracks.iter().any(|c| !c.valid(n, now)) {
            return Err(bad("checkpoint traction crack is invalid"));
        }
        put_u64(w, MARK)?;
        put_u64(w, SECTION_V)?;
        put_u64(w, u64::from(!self.soft.is_empty()))?;
        if !self.soft.is_empty() {
            let mut bytes = Vec::with_capacity(1 << 16);
            for chunk in self.soft.chunks((1 << 16) / 24) {
                bytes.clear();
                for s in chunk {
                    for v in [s.amount, s.rate, s.at] {
                        bytes.extend_from_slice(&v.to_le_bytes());
                    }
                }
                w.write_all(&bytes)?;
            }
        }
        put_u64(w, self.cracks.len() as u64)?;
        for c in &self.cracks {
            let mut bytes = [0u8; 92];
            bytes[..8].copy_from_slice(&(c.i as u64).to_le_bytes());
            let mut at = 8;
            for v in c.under.values().into_iter().chain(c.top.values()).chain([c.d_mm, c.hw_mm, c.shown]) {
                bytes[at..at + 4].copy_from_slice(&v.to_le_bytes());
                at += 4;
            }
            bytes[at..at + 8].copy_from_slice(&c.set_at.to_le_bytes());
            bytes[at + 8..].copy_from_slice(&c.dry_at.to_le_bytes());
            w.write_all(&bytes)?;
        }
        Ok(())
    }

    pub(crate) fn read(r: &mut impl Read, n: usize, now: f64) -> io::Result<Self> {
        if get_u64(r)? != MARK {
            return Err(bad("checkpoint traction mark is invalid"));
        }
        if get_u64(r)? != SECTION_V {
            return Err(bad("checkpoint traction section version is unsupported"));
        }
        let count = match get_u64(r)? {
            0 => 0,
            1 => n,
            _ => return Err(bad("checkpoint traction softness flag is invalid")),
        };
        let mut soft = Vec::with_capacity(count);
        for _ in 0..count {
            let s = Soft {
                amount: get_f64(r)?,
                rate: get_f64(r)?,
                at: get_f64(r)?,
            };
            if !s.valid(now) {
                return Err(bad("checkpoint traction softness is invalid"));
            }
            soft.push(s);
        }
        let count = usize::try_from(get_u64(r)?)
            .ok()
            .filter(|&k| k <= n)
            .ok_or_else(|| bad("checkpoint traction crack count is invalid"))?;
        if count > 0 && soft.is_empty() {
            return Err(bad("checkpoint traction cracks have no substrate state"));
        }
        let mut cracks = Vec::with_capacity(count);
        for _ in 0..count {
            let i = usize::try_from(get_u64(r)?)
                .ok()
                .filter(|&i| i < n)
                .ok_or_else(|| bad("checkpoint traction crack index is invalid"))?;
            if cracks.last().is_some_and(|c: &CrackPx| c.i >= i) {
                return Err(bad("checkpoint traction crack indices are not ordered"));
            }
            let c = CrackPx {
                i,
                under: Surface::read(r)?,
                top: Surface::read(r)?,
                d_mm: get_f32(r)?,
                hw_mm: get_f32(r)?,
                shown: get_f32(r)?,
                set_at: get_f64(r)?,
                dry_at: get_f64(r)?,
            };
            if !c.valid(n, now) {
                return Err(bad("checkpoint traction crack is invalid"));
            }
            cracks.push(c);
        }
        Ok(Self {
            soft,
            cracks,
            net: NetCache::default(),
        })
    }
}

fn bad(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn put_u64(w: &mut impl Write, value: u64) -> io::Result<()> {
    w.write_all(&value.to_le_bytes())
}
fn get_u64(r: &mut impl Read) -> io::Result<u64> {
    let mut b = [0; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}
fn get_f64(r: &mut impl Read) -> io::Result<f64> {
    Ok(f64::from_bits(get_u64(r)?))
}
fn get_f32(r: &mut impl Read) -> io::Result<f32> {
    let mut b = [0; 4];
    r.read_exact(&mut b)?;
    Ok(f32::from_le_bytes(b))
}

fn softness(t_um: f32, stiff: f32) -> f32 {
    t_um.max(0.0) * (0.3 + 0.7 * (1.0 - stiff.clamp(0.0, 1.0)))
}
fn through_rate(th: f32, stiff: f32, drying: f32) -> f32 {
    3.0 * rate(th, stiff, drying) * (TOUCH_DRY_MIN / TOUCH_DRY_MIN_3) / (THROUGH * (1.0 + th.max(0.0)))
}
fn cracking(soft_um: f32, stiff: f32, top_rate: f32, under_rate: f32) -> f32 {
    if soft_um <= SOFT0 {
        return 0.0;
    }
    ((soft_um - SOFT0) / SOFT1).min(1.0)
        * (0.4 + 0.6 * stiff.clamp(0.0, 1.0))
        * smoothstep(2f32.ln(), 20f32.ln(), (top_rate / under_rate.max(1e-12)).max(1e-6).ln())
}
fn gens_for(t_um: f32) -> u8 {
    LAST_GEN.saturating_sub((t_um.max(1.0) / COAT_UM).log2().max(0.0).floor() as u8).max(1)
}

// The indexed network is derived solely from the canvas's physical size and seed.
// It is rebuilt after restoring a checkpoint, rather than saved as live state.
#[derive(Debug)]
pub(crate) struct DryNet {
    size_mm: [f32; 2],
    tile: [usize; 2],
    origin_mm: [f32; 2],
    /// Bytes it holds once built.
    memory_cost: usize,
    /// Most bytes its build held at once (the stress grid and segment index
    /// are freed before it is done).
    build_cost: usize,
    seed: u64,
    /// Segments: ends (mm), relative opening at each end, generation.
    segs: Vec<([f32; 2], [f32; 2], f32, f32, u8)>,
    cell: f32,
    cols: usize,
    rows: usize,
    cells: Vec<Vec<u32>>,
}

impl DryNet {
    /// A budget with `reserved` bytes already spoken for.
    fn budget(size_mm: [f32; 2], reserved: usize) -> crate::crack::Budget {
        let mut budget = crate::crack::Budget::limited(size_mm, NETWORK_MEMORY_BUDGET);
        if reserved > budget.used() {
            budget.charge::<u8>(reserved - budget.used());
        }
        budget
    }

    fn new(size_mm: [f32; 2], seed: u64, tile: [usize; 2], reserved: usize) -> DryNet {
        use crate::crack::{Cracks, End, network_with_budget};
        let k = Cracks {
            island_mm: Some(FINEST_MM),
            ground_um: Some(300.0),
            corners: false,
            grain: 0.0,
            ..Cracks::aged(seed)
        };
        // The region starts on a whole-canvas stress cell, so its grid, flaws
        // and strengths coincide with its neighbours' where they overlap.
        let gc = k.island().max(0.2) * NETWORK_GRID;
        let origin_mm = tile.map(|i| ((i as f32 * NETWORK_TILE_MM - NETWORK_HALO_MM).max(0.0) / gc).floor() * gc);
        let tile_mm = std::array::from_fn(|i| (((tile[i] + 1) as f32 * NETWORK_TILE_MM + NETWORK_HALO_MM).min(size_mm[i]) - origin_mm[i]).max(0.0));
        let budget = Self::budget(size_mm, reserved);
        let (net, mut budget) = network_with_budget(&k, size_mm, origin_mm, tile_mm, [0.7, 0.7], NETWORK_GRID, budget, true);
        let mut segs = Vec::new();
        let total: usize = net.arms.iter().map(|a| a.pts.len().saturating_sub(1)).sum();
        budget.reserve(&mut segs, total);
        let mut along = Vec::new();
        for a in &net.arms {
            let n = a.pts.len();
            if n < 2 {
                continue;
            }
            // length along the arm, for the taper to a free end
            along.clear();
            budget.reserve(&mut along, n);
            along.resize(n, 0.0f32);
            for j in 1..n {
                along[j] = along[j - 1] + ((a.pts[j][0] - a.pts[j - 1][0]).powi(2) + (a.pts[j][1] - a.pts[j - 1][1]).powi(2)).sqrt();
            }
            let len = along[n - 1].max(1e-6);
            let open = |j: usize| {
                let taper = if a.end == End::Free {
                    ((len - along[j]) / (0.35 * len)).clamp(0.0, 1.0).sqrt()
                } else {
                    1.0
                };
                a.opening.clamp(0.2, 1.5) * taper
            };
            for j in 1..n {
                budget.push(&mut segs, (a.pts[j - 1], a.pts[j], open(j - 1), open(j), a.generation));
            }
        }
        let cell = 2.0f32;
        let (cols, rows) = (((tile_mm[0] / cell).ceil() as usize).max(1), ((tile_mm[1] / cell).ceil() as usize).max(1));
        drop(along);
        let mut cells = budget.filled(cols * rows, Vec::new());
        for (si, &(a, b, ..)) in segs.iter().enumerate() {
            let (x0, x1) = (
                (a[0].min(b[0]) / cell).floor().max(0.0) as usize,
                ((a[0].max(b[0]) / cell).floor().max(0.0) as usize).min(cols - 1),
            );
            let (y0, y1) = (
                (a[1].min(b[1]) / cell).floor().max(0.0) as usize,
                ((a[1].max(b[1]) / cell).floor().max(0.0) as usize).min(rows - 1),
            );
            for y in y0..=y1.max(y0) {
                for x in x0..=x1.max(x0) {
                    if x < cols && y < rows {
                        budget.push(&mut cells[y * cols + x], si as u32);
                    }
                }
            }
        }
        DryNet {
            size_mm,
            tile,
            origin_mm,
            memory_cost: budget.used() - reserved,
            build_cost: budget.peak() - reserved,
            seed,
            segs,
            cell,
            cols,
            rows,
            cells,
        }
    }

    /// The nearest crack of generation at most `gmax` to `p` (mm), within a
    /// cell: its distance (mm) and its opening there; None if none is near.
    fn nearest(&self, p: [f32; 2], gmax: u8) -> Option<(f32, f32)> {
        let p = [p[0] - self.origin_mm[0], p[1] - self.origin_mm[1]];
        let (cx, cy) = ((p[0] / self.cell).floor() as isize, (p[1] / self.cell).floor() as isize);
        let mut best: Option<(f32, f32)> = None;
        for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let (x, y) = (cx + dx, cy + dy);
            if x < 0 || y < 0 || x as usize >= self.cols || y as usize >= self.rows {
                continue;
            }
            let dx = (x as f32 * self.cell - p[0]).max(p[0] - (x + 1) as f32 * self.cell).max(0.0);
            let dy = (y as f32 * self.cell - p[1]).max(p[1] - (y + 1) as f32 * self.cell).max(0.0);
            if best.is_some_and(|b| dx * dx + dy * dy > b.0) {
                continue;
            }
            for &si in &self.cells[y as usize * self.cols + x as usize] {
                let (a, b, wa, wb, g) = self.segs[si as usize];
                if g > gmax {
                    continue;
                }
                let ab = [b[0] - a[0], b[1] - a[1]];
                let l2 = (ab[0] * ab[0] + ab[1] * ab[1]).max(1e-12);
                let t = (((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1]) / l2).clamp(0.0, 1.0);
                let d = (p[0] - a[0] - t * ab[0]).powi(2) + (p[1] - a[1] - t * ab[1]).powi(2);
                if best.is_none_or(|bst| d < bst.0) {
                    best = Some((d, wa + (wb - wa) * t));
                }
            }
        }
        best.map(|(d, open)| (d.sqrt(), open))
    }

    fn sample(&self, c: &CrackPx, point: [f32; 2], weight: f32) -> (f32, f32) {
        if weight <= 0.0 { return (f32::INFINITY, 0.0); }
        let gmax = gens_for(c.d_mm);
        let Some((d, open)) = self.nearest(point, gmax) else { return (f32::INFINITY, 0.0) };
        let island = FINEST_MM * 1.6f32.powi((LAST_GEN - gmax) as i32);
        (d, 0.5 * GAP_MAX * c.hw_mm * open * island * weight)
    }
}

/// The cache of a canvas's `DryNet` (equal whatever it holds: it is made
/// again alike).
#[derive(Debug, Default)]
pub(crate) struct NetCache(pub Option<std::sync::Arc<DryNet>>);

impl Clone for NetCache {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for NetCache {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Canvas {
    pub fn check_traction_memory(&self) -> Result<(), String> {
        if !self.traction_enabled { return Ok(()); }
        let pm = self.px_mm();
        let size = [self.f.full_w as f32 * pm, self.f.full_h as f32 * pm];
        crate::crack::check_network_grid(size, size.map(|v| v.min(NETWORK_TILE_MM + 2.0 * NETWORK_HALO_MM)), FINEST_MM, NETWORK_GRID, NETWORK_MEMORY_BUDGET)
    }

    fn seed_of_traction(&self) -> u64 {
        self.linen.map_or(11, |l| l.seed) ^ 0x0d3f_ec75 ^ 0xc4ac
    }

    fn traction_net(&mut self, tile: [usize; 2], reserved: usize) -> std::sync::Arc<DryNet> {
        let pm = self.px_mm();
        let size_mm = [self.f.full_w as f32 * pm, self.f.full_h as f32 * pm];
        let seed = self.seed_of_traction();
        if let Some(ref net) = self.traction.net.0
            && net.size_mm == size_mm
            && net.seed == seed
            && net.tile == tile
        {
            let mut budget = crate::crack::Budget::limited(size_mm, NETWORK_MEMORY_BUDGET);
            if reserved > budget.used() {
                budget.charge::<u8>(reserved - budget.used());
            }
            budget.charge::<u8>(net.memory_cost);
            return net.clone();
        }
        self.traction.net.0 = None;
        let net = std::sync::Arc::new(DryNet::new(size_mm, seed, tile, reserved));
        self.traction.net.0 = Some(net.clone());
        net
    }

    /// After leveling, before compositing: remember both exposed surfaces.
    pub(crate) fn traction_before(
        &mut self,
        ex: (usize, usize, usize, usize),
        add: &[f32],
        t: &[f32],
        under_height: &[f32],
        now: f64,
        dates: &[(usize, [f64; 2])],
    ) -> Vec<CrackPx> {
        let (w, ew) = (self.f.w, ex.2 - ex.0);
        if self.traction.soft.is_empty() {
            self.traction.soft.resize(self.px.len(), Soft::default());
        }
        self.traction.cracks.retain(|c| {
            let (x, y) = (c.i % w, c.i / w);
            !(x >= ex.0 && x < ex.2 && y >= ex.1 && y < ex.3 && add[(y - ex.1) * ew + x - ex.0] > 0.0)
        });
        let mut found = Vec::new();
        for y in ex.1..ex.3 {
            for x in ex.0..ex.2 {
                let k = (y - ex.1) * ew + x - ex.0;
                if add[k] <= 0.0 || t[k] <= 0.0 {
                    continue;
                }
                let i = y * w + x;
                let p = self.wet.clock.px[i];
                let hd = self.wet.hide[i];
                let th = if p.th > 0.0 { p.th } else { t[k] / COAT_UM };
                let top_rate = rate(th, hd[1], hd[2]);
                let set_rate = Pace::of(self.engine).set_rate(top_rate);
                let fallback = (now - ((p.cure - GEL).max(0.0) / set_rate) as f64).max(0.0);
                let [set_at, dry_at] = dates
                    .binary_search_by_key(&i, |p| p.0)
                    .map(|j| dates[j].1)
                    .unwrap_or([fallback, fallback + ((1.0 - GEL) / set_rate) as f64]);
                let soft = self.traction.soft[i];
                let under = soft.at(set_at);
                let severity = cracking(under, hd[1], top_rate, soft.rate as f32);
                if severity > 0.0 {
                    let mut surface = Surface::capture(self, i);
                    surface.height = under_height[k];
                    found.push(CrackPx {
                        i,
                        under: surface,
                        top: surface,
                        d_mm: t[k],
                        hw_mm: severity,
                        shown: 0.0,
                        set_at,
                        dry_at,
                    });
                }
                let more = softness(t[k], hd[1]);
                let tr = through_rate(th, hd[1], hd[2]);
                let total = under + more;
                if total > 0.0 {
                    // Average through-hardening times at the gel crossing.
                    let time = (if under > 0.0 { under as f64 / soft.rate.max(1e-12) } else { 0.0 } + more as f64 / (tr as f64).max(1e-12)) / total as f64;
                    self.traction.soft[i] = Soft {
                        amount: total as f64,
                        rate: 1.0 / time.max(1e-6),
                        at: set_at,
                    };
                }
            }
        }
        if found.is_empty() {
            return found;
        }
        let (pm, ox, oy) = (self.px_mm(), self.f.x0, self.f.y0);
        let size_mm = [self.f.full_w as f32 * pm, self.f.full_h as f32 * pm];
        let counts = size_mm.map(|v| (v / NETWORK_TILE_MM).ceil().max(1.0) as usize);
        let point_of = |c: &CrackPx| [(c.i % w + ox) as f32 * pm, (c.i / w + oy) as f32 * pm];
        if counts == [1, 1] {
            let net = self.traction_net([0, 0], 0);
            return found.into_par_iter().filter_map(|mut c| {
                let (d, hw) = net.sample(&c, point_of(&c), 1.0);
                (d < hw + pm).then(|| {
                    c.d_mm = d;
                    c.hw_mm = hw;
                    c
                })
            }).collect();
        }
        let mut budget = crate::crack::Budget::limited(size_mm, NETWORK_MEMORY_BUDGET);
        if found.len() > u32::MAX as usize || counts[0].checked_mul(counts[1]).is_none_or(|n| n > u32::MAX as usize) {
            budget.refused();
        }
        let reach = NETWORK_BLEND_MM + NETWORK_EDGE_JITTER_MM;
        let tiles_of = |c: &CrackPx| {
            let p = point_of(c);
            let lo = p.map(|v| ((v - reach) / NETWORK_TILE_MM).floor().max(0.0) as usize);
            let hi = std::array::from_fn::<_, 2, _>(|i| (((p[i] + reach) / NETWORK_TILE_MM).floor() as usize).min(counts[i] - 1));
            (lo, hi)
        };
        let n = found.iter().map(|c| {
            let (lo, hi) = tiles_of(c);
            (hi[0] - lo[0] + 1) * (hi[1] - lo[1] + 1)
        }).sum();
        let seed = self.seed_of_traction();
        // A warm cache is used if it holds a needed region, and dropped
        // before any other region is built (as if it had not been there).
        let cached = self.traction.net.0.take().filter(|net| net.size_mm == size_mm && net.seed == seed);
        if let Some(net) = &cached {
            budget.charge::<u8>(net.memory_cost);
        }
        let mut queries = Vec::new();
        budget.reserve(&mut queries, n);
        for (i, c) in found.iter().enumerate() {
            let (lo, hi) = tiles_of(c);
            for y in lo[1]..=hi[1] {
                for x in lo[0]..=hi[0] {
                    queries.push(((y * counts[0] + x) as u32, i as u32));
                }
            }
        }
        queries.sort_unstable();
        let mut best = budget.filled(found.len(), (f32::INFINITY, 0.0));
        let mut sampled = Vec::new();
        let largest = queries.chunk_by(|a, b| a.0 == b.0).map(|q| q.len()).max().unwrap_or(0);
        budget.reserve(&mut sampled, largest);
        let reserved = budget.used() - cached.as_ref().map_or(0, |net| net.memory_cost + 64);
        let coverage = |d: f32, hw: f32| (((d + 0.5 * pm).min(hw) - (d - 0.5 * pm).max(-hw)).max(0.0) / pm).min(1.0);
        let edge = crate::noise::Fbm::new(seed as u32 ^ 0x74ec, 2, 24.0);
        let weight_of = |tile: [usize; 2], point: [f32; 2]| (0..2).map(|axis| {
            let start = tile[axis] as f32 * NETWORK_TILE_MM;
            let end = start + NETWORK_TILE_MM;
            let offset = |boundary| NETWORK_EDGE_JITTER_MM * edge.get(boundary + axis as f32 * 10000.0, point[1 - axis]).clamp(-1.0, 1.0);
            let left = if tile[axis] == 0 || point[axis] >= start + reach { 1.0 } else {
                let boundary = start + offset(start);
                smoothstep(boundary - NETWORK_BLEND_MM, boundary + NETWORK_BLEND_MM, point[axis])
            };
            let right = if tile[axis] + 1 == counts[axis] || point[axis] <= end - reach { 1.0 } else {
                let boundary = end + offset(end);
                1.0 - smoothstep(boundary - NETWORK_BLEND_MM, boundary + NETWORK_BLEND_MM, point[axis])
            };
            left * right
        }).product::<f32>();
        let mut sample = |net: &DryNet, pixels: &[(u32, u32)], best: &mut Vec<(f32, f32)>| {
            let tile = net.tile;
            pixels.par_iter().map(|&(_, i)| {
                let c = &found[i as usize];
                let point = point_of(c);
                // Relative to the strongest region here: a crack both regions
                // agree on keeps its full width across the border, and only
                // cracks one region alone has fade out.
                let (lo, hi) = tiles_of(c);
                let mut strongest = 0.0f32;
                for y in lo[1]..=hi[1] {
                    for x in lo[0]..=hi[0] {
                        strongest = strongest.max(weight_of([x, y], point));
                    }
                }
                let weight = (weight_of(tile, point) / strongest.max(1e-6)).min(1.0);
                let (d, hw) = net.sample(c, point, weight);
                (i, d, hw)
            }).collect_into_vec(&mut sampled);
            for &(i, d, hw) in &sampled {
                let previous = &mut best[i as usize];
                // a total order, so the result is the same whatever order
                // the regions are visited in
                let (a, b) = (coverage(d, hw), coverage(previous.0, previous.1));
                if a > b || (a == b && a > 0.0 && (hw > previous.1 || (hw == previous.1 && d < previous.0))) {
                    *previous = (d, hw);
                }
            }
        };
        let mut groups: Vec<(usize, &[(u32, u32)])> = queries.chunk_by(|a, b| a.0 == b.0).map(|q| (q[0].0 as usize, q)).collect();
        let tile_of = |id: usize| [id % counts[0], id / counts[0]];
        if let Some(net) = cached {
            if let Some(at) = groups.iter().position(|g| tile_of(g.0) == net.tile) {
                sample(&net, groups.remove(at).1, &mut best);
            }
        }
        // The regions are independent: build several at once, each within an
        // equal share of what the budget has left. A region that does not fit
        // its share is built again alone, so the result never depends on the
        // batching, only whether it fits the whole budget does.
        let mut cost = 0usize;
        let mut last = None;
        let mut rest = &groups[..];
        while !rest.is_empty() {
            let free = NETWORK_MEMORY_BUDGET.saturating_sub(reserved);
            let batch = if cost == 0 { 1 } else { (free / (cost + cost / 8).max(1)).clamp(1, NETWORK_PARALLEL) }.min(rest.len());
            let (now, later) = rest.split_at(batch);
            rest = later;
            last = None;
            let nets: Vec<Option<DryNet>> = if batch == 1 {
                vec![Some(DryNet::new(size_mm, seed, tile_of(now[0].0), reserved))]
            } else {
                let share = free / batch;
                let charged = reserved + (batch - 1) * share;
                now.par_iter().map(|g| {
                    crate::crack::speculatively(|| DryNet::new(size_mm, seed, tile_of(g.0), charged)).ok()
                }).collect()
            };
            let mut alive: usize = nets.iter().flatten().map(|net| net.memory_cost + 64).sum();
            for (g, net) in now.iter().zip(nets) {
                let net = match net {
                    Some(net) => {
                        alive -= net.memory_cost + 64;
                        net
                    }
                    // the regions still held count against the rebuild
                    None => DryNet::new(size_mm, seed, tile_of(g.0), reserved + alive),
                };
                cost = cost.max(net.build_cost);
                sample(&net, g.1, &mut best);
                last = Some(net);
            }
        }
        drop(sample);
        if let Some(net) = last {
            self.traction.net.0 = Some(std::sync::Arc::new(net));
        }
        for (c, (d, hw)) in found.iter_mut().zip(best) {
            c.d_mm = d;
            c.hw_mm = hw;
        }
        found.retain(|c| c.hw_mm > 0.0);
        found
    }

    pub(crate) fn traction_set(&mut self, mut new: Vec<CrackPx>) {
        for c in &mut new {
            c.top = Surface::capture(self, c.i);
        }
        self.traction.cracks.extend(new);
        self.traction.cracks.sort_by_key(|c| c.i);
        self.traction.cracks.dedup_by_key(|c| c.i);
    }

    pub(crate) fn traction_open(&mut self, now: f64) {
        if self.traction.cracks.is_empty() {
            return;
        }
        let pm = self.px_mm();
        let mut cracks = std::mem::take(&mut self.traction.cracks);
        let mut changed = false;
        cracks.retain_mut(|c| {
            // A later glaze, varnish or other surface edit owns this pixel.
            if Surface::capture(self, c.i) != c.top.mix(c.under, c.shown) {
                return false;
            }
            let age = (now - c.set_at).max(0.0);
            let sub = GEL + (1.0 - GEL) * ((now - c.set_at) / (c.dry_at - c.set_at).max(1e-9)).clamp(0.0, 1.0) as f32;
            let mature = if age >= 5.0 * OPEN_MIN { 1.0 } else { (1.0 - (-age / OPEN_MIN).exp()) as f32 };
            let opening = smoothstep(GEL, 1.0, sub) * (0.3 + 0.7 * mature);
            let a = c.hw_mm * opening;
            let cover = (((c.d_mm + 0.5 * pm).min(a) - (c.d_mm - 0.5 * pm).max(-a)).max(0.0) / pm).min(1.0);
            if cover > c.shown {
                c.top.mix(c.under, cover).apply(self, c.i);
                c.shown = cover;
                changed = true;
            }
            sub < 1.0 || age < 5.0 * OPEN_MIN
        });
        self.traction.cracks = cracks;
        if changed {
            self.surf_gen += 1;
        }
    }
}

#[cfg(test)]
#[path = "traction_tests.rs"]
mod tests;

#[cfg(test)]
mod seam_probe {
    use super::*;

    /// Share of crack length in the blend zone of a region border that the
    /// neighbouring region reproduces within `tol` mm (EVAL_MM canvas).
    #[test]
    #[ignore]
    fn region_overlap_agreement() {
        let mm: f32 = std::env::var("EVAL_MM").ok().map_or(640.0, |v| v.parse().unwrap());
        let size = [mm, mm];
        let a = DryNet::new(size, 7, [0, 0], 0);
        let b = DryNet::new(size, 7, [1, 0], 0);
        let reach = NETWORK_BLEND_MM + NETWORK_EDGE_JITTER_MM;
        for tol in [0.1f32, 0.3, 0.8] {
            let agree = |p: &DryNet, q: &DryNet| {
                let (mut tot, mut ok) = (0.0f32, 0.0f32);
                for &(s0, s1, _, _, g) in &p.segs {
                    let m = [0.5 * (s0[0] + s1[0]) + p.origin_mm[0], 0.5 * (s0[1] + s1[1]) + p.origin_mm[1]];
                    if (m[0] - NETWORK_TILE_MM).abs() > reach || m[1] > NETWORK_TILE_MM - 20.0 || g > LAST_GEN { continue; }
                    let l = ((s1[0] - s0[0]).powi(2) + (s1[1] - s0[1]).powi(2)).sqrt();
                    tot += l;
                    if q.nearest(m, LAST_GEN).is_some_and(|(d, _)| d < tol) { ok += l; }
                }
                ok / tot.max(1e-6)
            };
            println!("tol {tol} mm: A reproduced by B {:.3}, B by A {:.3}", agree(&a, &b), agree(&b, &a));
        }
    }
}
