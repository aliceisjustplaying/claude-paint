# Source receipts for the review response

These excerpts were read from the original uploaded archive, not a live repository. They support source-level findings; they are not receipts for the counter-review's newly reported Rust runs. Source line numbers are the numbers on the left.

## S01 — Thinned-stroke ceiling and explicit exclusions

Path: `wip/thinner/crates/paint/src/thinner.rs`  
SHA-256: `f772900571317662cbad7ae46c94e99ffe788657f07e57dd2802e95b545e0683`

Lines 64–88:

```text
  64 //! Not modeled, on purpose: solvent evaporating from the brush or the
  65 //! palette pile (the pile keeps its share), solvent soaking into the
  66 //! ground, solvent dissolving set or dry paint underneath, and solvent-wet
  67 //! paint coming up more readily on a brush or rag than the same paint
  68 //! without it (extra pickup). The brush and the rag take solvent with the
  69 //! paint they lift, in the film's own proportions.
  70 
  71 /// One stroke of paint thinned half (`t` = 0.5) adds at most this much wet
  72 /// film (µm) to a pixel. ESTIMATE, set before measuring; the card keeps 84%
  73 /// (load 0.3) and 81% (load 0.6) of its contrast with it, and from 6 to 36
  74 /// µm it kept 62-89% (notes/thinner/RESULTS.md, the sweep).
  75 pub const STROKE_FILM_UM: f32 = 6.0;
  76 
  77 /// The most wet film (paint + solvent, µm) one stroke may add to a pixel,
  78 /// for paint holding the share `t` of solvent: `STROKE_FILM_UM` at 0.5,
  79 /// growing without bound as `t` → 0 (`f32::INFINITY` at 0: no ceiling, the
  80 /// paste law alone), shrinking as `t` → 1. Continuous: a hundredth of
  81 /// thinner barely changes a stroke.
  82 pub fn stroke_limit_um(t: f32) -> f32 {
  83     if t.is_nan() || t <= 0.0 {
  84         return f32::INFINITY;
  85     }
  86     let t = t.min(0.95);
  87     STROKE_FILM_UM * (1.0 - t) / t
  88 }
```

## S02 — Rag minimum stain and solvent-carrying pickup

Path: `wip/thinner/crates/paint/src/rag.rs`  
SHA-256: `2c4cc01c9ba43d3713b8091d830326f94b246e72c8ee0a9c960d865f074ee40e`

Lines 65–73:

```text
  65 const LIFT: f32 = 4.0;
  66 /// What a blot lifts, as a share of a full pass (no drag) [E].
  67 const BLOT: f32 = 0.8;
  68 /// The stain: coats of film the cloth can't take, pigment caught in the
  69 /// tooth, where the cloth reaches all the film; twice as much where it
  70 /// reaches none of it (the hollows) [E].
  71 const STAIN_COATS: f32 = 0.04;
  72 /// How far the cloth bridges between peaks of the surface (mm): about the
  73 /// spacing of the threads of a 15-thread linen [E].
```

Lines 285–305:

```text
 285     /// solvent in the film comes away with the paint, in their proportions
 286     /// there; returns how much (coats; the canvas holds it in µm).
 287     #[inline]
 288     fn rag_take(&mut self, i: usize, take: f32) -> f32 {
 289         let v0 = self.wet.vol[i];
 290         let mut ts = 0.0;
 291         if let Some(s) = self.wet.solv.get_mut(i)
 292             && *s > 0.0
 293             && v0 > 0.0
 294         {
 295             let tu = (*s * take / v0).min(*s);
 296             *s -= tu;
 297             ts = tu / COAT_UM;
 298         }
 299         let v = &mut self.wet.vol[i];
 300         *v -= take;
 301         if self.engine >= 2 && *v < 1e-5 && self.wet.clock.px.len() == self.wet.vol.len() {
 302             self.wet.clock.px[i].cure = 0.0;
 303         }
 304         ts
 305     }
```

Lines 338–381:

```text
 338         rag.evaporate(self.now_min());
 339         let d = rag.damp.clamp(0.0, 1.0);
 340         let k = LIFT * (0.7 + 0.6 * p) * rag.thirst() * (1.0 + DAMP_LIFT * d);
 341         let timed = self.wet.clock.px.len() == self.wet.vol.len();
 342         let mut lifted = 0.0f64;
 343         let mut lifted_s = 0.0f64;
 344         for y in y0..y1 {
 345             let mut row = 0.0f32;
 346             let mut row_s = 0.0f32;
 347             for x in x0..x1 {
 348                 let i = y * f.w + x;
 349                 let v = self.wet.vol[i];
 350                 if v <= 1e-6 {
 351                     continue;
 352                 }
 353                 let e = expo(f.ux(x), f.uy(y));
 354                 if e <= 0.0 {
 355                     continue;
 356                 }
 357                 let fl = if timed { reach_fluid(self.wet.clock.px[i].cure) } else { 1.0 };
 358                 if fl <= 0.0 {
 359                     continue;
 360                 }
 361                 let j = (y - by0) * bw + (x - bx0);
 362                 // the film above the cloth's level, coats
 363                 let level = peaks[j] - reach;
 364                 let near = ((surf[j] - level) / COAT_UM).clamp(0.0, v);
 365                 let avail = near + WICK * (v - near);
 366                 let frac = 1.0 - (-k * fl * e).exp();
 367                 // the stain: more of it where the cloth didn't reach
 368                 let floor = STAIN_COATS * (2.0 - near / v);
 369                 let take = (avail * frac).min(v - floor);
 370                 if take > 0.0 {
 371                     row_s += self.rag_take(i, take);
 372                     row += take;
 373                 }
 374             }
 375             lifted += row as f64;
 376             lifted_s += row_s as f64;
 377         }
 378         let mm3 = lifted * (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
 379         rag.solvent_mm3 += lifted_s * (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
 380         let w_mm = (rag.width * self.mm_per_unit) as f64;
 381         let cap = w_mm * w_mm * (CAP_UM as f64 / 1000.0);
```

## S03 — Wetting floor, mobility, shared maximum and flow limiter

Path: `wip/thinner/crates/paint/src/thinner.rs`  
SHA-256: `f772900571317662cbad7ae46c94e99ffe788657f07e57dd2802e95b545e0683`

Lines 110–147:

```text
 110 /// Mobility (mm²/min) of a wet film half solvent: how fast it levels.
 111 /// ESTIMATE (Orchard 1963, see the module notes).
 112 pub const SPREAD_MM2_MIN: f32 = 0.06;
 113 /// The flow doesn't drain a pixel below this much liquid (µm): a liquid
 114 /// that wets the paint under it leaves a film on the weave's tops, it
 115 /// doesn't run off them bare (Orchard's leveling rate goes as the film's
 116 /// thickness cubed, so the last of a film barely moves). ESTIMATE.
 117 pub const WET_FILM_UM: f32 = 2.0;
 118 
 119 /// Mobility of a film whose liquid holds the share `phi` of solvent: 0
 120 /// without solvent, `SPREAD_MM2_MIN` at one half, more the thinner it is
 121 /// (viscosity falls steeply with solvent). ESTIMATE.
 122 pub fn spread_mm2_min(phi: f32) -> f32 {
 123     if phi.is_nan() || phi <= 0.0 {
 124         return 0.0;
 125     }
 126     let phi = phi.min(0.95);
 127     SPREAD_MM2_MIN * phi / (1.0 - phi)
 128 }
 129 
 130 /// Below this (µm) a pixel's solvent is gone. A chosen numerical cutoff
 131 /// (it lets a wait end with the solvent gone), not a physical constant.
 132 pub(crate) const SOLVENT_FLOOR: f32 = 1e-8;
 133 
 134 use crate::canvas::Canvas;
 135 use crate::surface::COAT_UM;
 136 use crate::wet::{Latent, Prop};
 137 use rayon::prelude::*;
 138 
 139 /// Most substeps of the flow in one step of the clock; past it the flow
 140 /// is slowed to stay stable (a film that thin and that fine-grained flows a
 141 /// little less far per minute than its mobility says). A chosen numerical
 142 /// cutoff, not a physical constant, as are `MAX_OUT` and the explicit
 143 /// scheme's stability factor 0.2 in `spread`.
 144 const MAX_SUBSTEPS: usize = 64;
 145 /// Most of a pixel's liquid that can leave it in one substep. A chosen
 146 /// numerical cutoff.
 147 const MAX_OUT: f32 = 0.5;
```

Lines 186–259:

```text
 186     pub(crate) fn spread(&mut self, dt: f32) {
 187         let Some((bx0, by0, bx1, by1)) = self.wet.dirty else { return };
 188         let (w, h) = (self.f.w, self.f.h);
 189         let dx = self.px_mm();
 190         let timed = self.wet.clock.px.len() == w * h;
 191         // the mobility (mm²/min) of each pixel in the dirty box
 192         // (the canvas holds solvent in µm; the flow works in coats)
 193         let mob = |wet: &crate::wet::Wet, i: usize| -> f32 {
 194             let (v, s) = (wet.vol[i], wet.solv[i] / COAT_UM);
 195             if s <= 0.0 || v + s <= 0.0 {
 196                 return 0.0;
 197             }
 198             let fl = if timed { crate::drying::fluid(wet.clock.px[i].cure) } else { 1.0 };
 199             spread_mm2_min(s / (v + s)) * fl
 200         };
 201         let mut m_max = 0.0f32;
 202         for y in by0..by1.min(h) {
 203             for x in bx0..bx1.min(w) {
 204                 m_max = m_max.max(mob(&self.wet, y * w + x));
 205             }
 206         }
 207         if m_max <= 0.0 {
 208             return;
 209         }
 210         let r_total = m_max * dt / (dx * dx);
 211         let n = ((r_total / 0.2).ceil() as usize).clamp(1, MAX_SUBSTEPS);
 212         // the flow's rate per substep, per mm²/min of mobility
 213         let k = (dt / n as f32 / (dx * dx)).min(0.2 / m_max.max(1e-12));
 214         let (mut x0, mut y0, mut x1, mut y1) = (bx0, by0, bx1.min(w), by1.min(h));
 215         for _ in 0..n {
 216             // liquid can reach one pixel further each substep
 217             (x0, y0, x1, y1) = (x0.saturating_sub(1), y0.saturating_sub(1), (x1 + 1).min(w), (y1 + 1).min(h));
 218             let (rw, rh) = (x1 - x0, y1 - y0);
 219             let wet = &self.wet;
 220             let height = &self.height;
 221             // pass 1: each pixel's outflow (coats of liquid) to its four
 222             // neighbors (left, right, up, down)
 223             let out: Vec<[f32; 4]> = (0..rw * rh)
 224                 .into_par_iter()
 225                 .map(|k2| {
 226                     let (x, y) = (x0 + k2 % rw, y0 + k2 / rw);
 227                     let i = y * w + x;
 228                     let l = wet.vol[i] + wet.solv[i] / COAT_UM;
 229                     let m = mob(wet, i);
 230                     if m <= 0.0 || l <= 0.0 {
 231                         return [0.0; 4];
 232                     }
 233                     let z = height[i] + l * COAT_UM;
 234                     // (within the region: every pixel that can receive is in it)
 235                     let nb = [(x > x0).then(|| i - 1), (x + 1 < x1).then(|| i + 1), (y > y0).then(|| i - w), (y + 1 < y1).then(|| i + w)];
 236                     let mut q = [0.0f32; 4];
 237                     let mut sum = 0.0f32;
 238                     for (d, j) in nb.iter().enumerate() {
 239                         if let Some(j) = *j {
 240                             let zj = height[j] + wet.vol[j] * COAT_UM + wet.solv[j];
 241                             if z > zj {
 242                                 q[d] = m * k * (z - zj) / COAT_UM;
 243                                 sum += q[d];
 244                             }
 245                         }
 246                     }
 247                     // (down to the wetting film, no further)
 248                     let avail = (l - WET_FILM_UM / COAT_UM).max(0.0);
 249                     if sum > MAX_OUT * avail {
 250                         let f = if sum > 0.0 { MAX_OUT * avail / sum } else { 0.0 };
 251                         for v in &mut q {
 252                             *v *= f;
 253                         }
 254                     }
 255                     q
 256                 })
 257                 .collect();
 258             if out.iter().all(|q| q.iter().all(|&v| v <= 0.0)) {
 259                 break;
```

## S04 — Coat thickness units

Path: `wip/thinner/crates/paint/src/surface.rs`  
SHA-256: `10949d67a21698071b3e81c22fb0347e4b0815fc732e3391fab7ceb84187bae9`

Lines 18–28:

```text
  18 use crate::canvas::Canvas;
  19 use crate::rng::hash2;
  20 use rayon::prelude::*;
  21 
  22 /// Thickness of one coat of wet paint (wet volume 1.0), µm. Brushed
  23 /// mock-up coats measure ~60–125 µm; one "coat" here is a lean, thin one.
  24 pub const COAT_UM: f32 = 25.0;
  25 /// Surface tension of drying oil, N/m.
  26 const SIGMA: f32 = 0.035;
  27 /// Time the paint levels before it has set enough to stop, s.
  28 pub(crate) const SET_TIME: f32 = 900.0;
```

## S05 — Released-main partial-contact, smear/soak, segment-local sampler

Path: `shipping/crates/paint/src/rag.rs`  
SHA-256: `c4035386c542e0ed05db63e01b3da2dca03d7fa5254dab14be15b3a314ac7000`

Lines 412–423:

```text
 412                 let j = (y - by0) * bw + (x - bx0);
 413                 // the film above the cloth's level, coats
 414                 let level = peaks[j] - reach;
 415                 let near = ((surf[j] - level) / COAT_UM).clamp(0.0, v);
 416                 let avail = near + wick * (v - near);
 417                 // engine 3: the cloth's contact shares out what the pad's
 418                 // rate takes, so its creases show however damp it is
 419                 let frac = if e3 { (1.0 - (-k * fl * pad).exp()) * c } else { 1.0 - (-k * fl * e).exp() };
 420                 // the stain: more of it where the cloth didn't reach
 421                 let floor = STAIN_COATS * (2.0 - near / v);
 422                 let take = (avail * frac).min(v - floor);
 423                 if take > 0.0 {
```

Lines 470–477:

```text
 470         if pl.vol > 1e-9 {
 471             let rate = SMEAR * pl.vol / (((x1 - x0) * (y1 - y0)) as f32).max(1.0);
 472             let mut bounds: Option<(usize, usize, usize, usize)> = None;
 473             for y in y0..y1 {
 474                 for x in x0..x1 {
 475                     let (pad, _, rim) = expo(f.ux(x), f.uy(y));
 476                     let a = (rate * rim).min(pl.vol - out);
 477                     if pad <= 0.0 || rim <= 0.0 || a <= 0.0 {
```

Lines 492–505:

```text
 492         pl.vol -= out;
 493         // what is left soaks in a little more; this step's lift joins it
 494         let v0 = pl.vol.max(0.0) * (1.0 - SOAK);
 495         let t = v0 + got.vol;
 496         if t > 0.0 {
 497             for q in 0..pl.lat.len() {
 498                 pl.lat[q] = (pl.lat[q] * v0 + got.lat[q]) / t;
 499             }
 500             for q in 0..3 {
 501                 pl.hide[q] = (pl.hide[q] * v0 + got.hide[q]) / t;
 502             }
 503             pl.cure = (pl.cure * v0 + got.cure) / t;
 504         }
 505         pl.vol = t;
```

Lines 549–573:

```text
 549         let len: f32 = pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum();
 550         if pts.len() == 1 || len < 1e-3 {
 551             return self.rag_blot(rag, pts[0].0, pts[0].1, pr(0.0), seed);
 552         }
 553         let mmu = self.mm_per_unit;
 554         self.tally.rag_stroke((len * mmu) as f64, (rag.width * mmu) as f64);
 555         let cs = rag.cloth_seed(seed);
 556         let e3 = self.engine >= 3;
 557         let w = rag.width;
 558         let r0 = 0.5 * w;
 559         let step = 0.5 * r0;
 560         let mut pool = Pool::default();
 561         let mut total = 0.0;
 562         let mut s_at = 0.0f32;
 563         for seg in pts.windows(2) {
 564             let (a, b) = (seg[0], seg[1]);
 565             let (dx, dy) = (b.0 - a.0, b.1 - a.1);
 566             let l = (dx * dx + dy * dy).sqrt();
 567             if l < 1e-6 {
 568                 continue;
 569             }
 570             let (tx, ty) = (dx / l, dy / l);
 571             let n = (l / step).ceil().max(1.0) as usize;
 572             for k in 0..n {
 573                 let (l0, l1) = (l * k as f32 / n as f32, l * (k + 1) as f32 / n as f32);
```

Lines 603–627:

```text
 603                     let c = (r * r - d * d).sqrt();
 604                     let over = ((sp + c).min(sl) - (sp - c).max(0.0)).max(0.0);
 605                     if over <= 0.0 {
 606                         return (0.0, 0.0, 0.0);
 607                     }
 608                     if !e3 {
 609                         let edge = 1.0 - smoothstep(0.55 * r, r, ad);
 610                         return (over / (2.0 * r) * edge, cloth(d * mmu, (s0 + sp) * mmu, cs), 0.0);
 611                     }
 612                     // engine 3: frayed sides, and ends where the cloth comes
 613                     // down and lifts off unevenly across the pad
 614                     let sg = s0 + sp;
 615                     let fray = 0.6 * vn(sg * mmu / FRAY_MM, if d > 0.0 { 3.1 } else { 7.3 }, cs ^ 0xD4) + 0.4 * vn(sg * mmu * 3.0 / FRAY_MM, if d > 0.0 { 5.2 } else { 9.4 }, cs ^ 0xD5);
 616                     let re = r * (1.0 - FRAY * fray);
 617                     let edge = 1.0 - smoothstep(0.5 * re, re, ad);
 618                     let start = ((sg + r * (1.2 * vn(d * mmu / END_MM, 13.7, cs ^ 0xE6) - 0.4)) / (0.25 * r)).clamp(0.0, 1.0);
 619                     let end = ((len + r * (1.2 * vn(d * mmu / END_MM, 11.3, cs ^ 0xE5) - 0.4) - sg) / (0.25 * r)).clamp(0.0, 1.0);
 620                     // the light-pressed rim: the frayed sides and the trailing end
 621                     let rim = smoothstep(0.4 * re, re, ad).max(smoothstep(len - r, len + 0.5 * r, sg));
 622                     (over / (2.0 * r) * edge * start * end, cloth3(d * mmu, sg * mmu, cs, p), rim)
 623                 });
 624             }
 625             s_at += l;
 626         }
 627         total
```

## S06 — Prototype local capacity, global load and release budgets

Path: `prototype/crates/paint/src/rag/face.rs`  
SHA-256: `0a2de74083051c8367a8e34edb51db583b9ae5f51664e7d3bb67e516534a13de`

Lines 1–71:

```text
   1 //! Paint attached to material locations on the active cloth face. All stored
   2 //! volumes are mm³, independent of canvas resolution. Absorbed paint remains
   3 //! in `held`; only still-wet surface paint can return to the canvas.
   4 use super::{Pool, CAP_UM, COAT_UM, SMEAR, SOAK, cloth};
   5 use crate::{drying, wet::{Latent, Prop}};
   6 
   7 #[derive(Clone, Debug, Default, PartialEq)]
   8 pub(super) struct Cell {
   9     pub paint: Pool,
  10     held: f32,
  11 }
  12 
  13 #[derive(Clone, Debug, PartialEq)]
  14 pub(super) struct Face {
  15     pub cells: Vec<Cell>,
  16     area: f32,
  17     at_min: f64,
  18 }
  19 
  20 impl Face {
  21     pub fn new(width_mm: f32, soaked: f32, now: f64) -> Self {
  22         let area = width_mm * width_mm / cloth::CELLS as f32;
  23         let held = soaked * area * CAP_UM / 1000.0;
  24         Self { cells: vec![Cell { held, ..Cell::default() }; cloth::CELLS], area, at_min: now }
  25     }
  26 
  27     pub fn age(&mut self, now: f64) {
  28         let elapsed = (now - self.at_min).max(0.0) as f32;
  29         for cell in &mut self.cells {
  30             let p = &mut cell.paint;
  31             if p.vol <= 0.0 { continue; }
  32             let coats = p.vol / self.area * 1000.0 / COAT_UM;
  33             p.cure += elapsed * drying::rate(coats, p.hide[1], p.hide[2]);
  34             // Set paint is retained in the cloth; fresh pickup cannot revive it.
  35             if p.cure >= drying::GEL { *p = Pool::default(); }
  36         }
  37         self.at_min = self.at_min.max(now);
  38     }
  39 
  40     pub fn thirst(&self, weights: [(usize, f32); 4]) -> f32 {
  41         let capacity = self.area * CAP_UM / 1000.0;
  42         weights.iter().map(|&(i, w)| w * (1.0 - (self.cells[i].held / capacity).clamp(0.0, 1.0).powi(2))).sum()
  43     }
  44 
  45     pub fn budgets(&self, exposure: &[f64]) -> Vec<f64> {
  46         self.cells.iter().zip(exposure).map(|(cell, &area)| {
  47             cell.paint.vol as f64 * drying::fluid(cell.paint.cure) as f64
  48                 * (1.0 - (-SMEAR as f64 * area / self.area as f64).exp())
  49         }).collect()
  50     }
  51 
  52     pub fn finish(&mut self, picked: &[Pool], deposited: &[f64], travel_widths: f32) {
  53         // Use physical travel, not stamp count, for soaking into the fibers.
  54         let surface_left = (1.0 - SOAK).powf(4.0 * travel_widths);
  55         for ((cell, got), &out) in self.cells.iter_mut().zip(picked).zip(deposited) {
  56             cell.paint.vol = (cell.paint.vol - out as f32).max(0.0) * surface_left;
  57             cell.held = (cell.held + got.vol - out as f32).max(0.0);
  58             mix(&mut cell.paint, got.vol, &got.lat, &got.hide, got.cure);
  59         }
  60     }
  61 }
  62 
  63 pub(super) fn mix(p: &mut Pool, volume: f32, lat: &Latent, hide: &Prop, cure: f32) {
  64     if volume <= 0.0 { return; }
  65     let total = p.vol + volume;
  66     let a = volume / total;
  67     for k in 0..p.lat.len() { p.lat[k] += (lat[k] - p.lat[k]) * a; }
  68     for k in 0..3 { p.hide[k] += (hide[k] - p.hide[k]) * a; }
  69     p.cure += (cure - p.cure) * a;
  70     p.vol = total;
  71 }
```

## S06b — Prototype whole-face and local pickup multipliers

Path: `prototype/crates/paint/src/rag.rs`  
SHA-256: `e0242af63021738002d92ec77833c16fe513a42818b61e90848c1ed664f764a3`

Lines 238–242:

```text
 238     /// How readily the face in use still takes paint (1 clean .. 0 full):
 239     /// a half-loaded cloth still drinks; a nearly full one barely does.
 240     fn thirst(&self) -> f32 {
 241         1.0 - self.load.clamp(0.0, 1.0).powi(2)
 242     }
```

Lines 413–418:

```text
 413         rag.evaporate(self.now_min());
 414         let d = rag.damp.clamp(0.0, 1.0);
 415         let e3 = self.engine >= 3;
 416         // (engine 3: spirits reach deeper, `DAMP_REACH`)
 417         let (reach, wick) = if e3 { (SAG_UM * (0.25 + 1.5 * p) * (1.0 + DAMP_REACH * d), WICK + (1.0 - WICK) * d) } else { (SAG_UM * (0.25 + 1.5 * p), WICK) };
 418         let k = LIFT * (0.7 + 0.6 * p) * rag.thirst() * (1.0 + DAMP_LIFT * d);
```

Lines 448–463:

```text
 448                 let level = peaks[j] - reach;
 449                 let near = ((surf[j] - level) / COAT_UM).clamp(0.0, v);
 450                 let avail = near + wick * (v - near);
 451                 // engine 3: the cloth's contact shares out what the pad's
 452                 // rate takes, so its creases show however damp it is
 453                 let local = face.as_ref().map_or(1.0, |f| f.thirst(material_weights));
 454                 let frac = if e3 { (1.0 - (-k * local * fl * pad).exp()) * c } else { 1.0 - (-k * fl * e).exp() };
 455                 // the stain: more of it where the cloth didn't reach
 456                 let floor = STAIN_COATS * (2.0 - near / v);
 457                 let take = (avail * frac).min(v - floor);
 458                 if take > 0.0 {
 459                     if face.is_some() {
 460                         let cure = if timed { self.wet.clock.px[i].cure } else { 0.0 };
 461                         for (j, weight) in material_weights {
 462                             face::mix(&mut picked[j], take * volume_mm3 * weight, &self.wet.lat[i], &self.wet.hide[i], cure);
 463                         }
```

Lines 488–529:

```text
 488         if let Some(mut dirty) = face.take() {
 489             // Budget each material cell once, then distribute that budget over
 490             // its actual contacts. Newly lifted paint joins after depositing,
 491             // so traversal order cannot move paint from one pixel to the next.
 492             let budgets = dirty.budgets(&exposure);
 493             let mut deposited = vec![0.0f64; cloth::CELLS];
 494             for y in y0..y1 {
 495                 for x in x0..x1 {
 496                     let (pad, c, _) = expo(f.ux(x), f.uy(y));
 497                     if pad * c <= 0.0 { continue; }
 498                     for (j, weight) in weights(f.ux(x), f.uy(y)) {
 499                         if exposure[j] <= 0.0 || weight <= 0.0 { continue; }
 500                         let amount = (budgets[j] * (weight * pad * c * px_mm * px_mm) as f64 / exposure[j])
 501                             .min((budgets[j] - deposited[j]).max(0.0));
 502                         if amount <= 0.0 { continue; }
 503                         let coats = (amount / volume_mm3 as f64) as f32;
 504                         self.rag_lay(y * f.w + x, coats, &dirty.cells[j].paint);
 505                         deposited[j] += coats as f64 * volume_mm3 as f64;
 506                         lifted -= coats as f64;
 507                     }
 508                 }
 509             }
 510             self.wet.touch(x0, y0, x1, y1);
 511             dirty.finish(&picked, &deposited, material.unwrap().2);
 512             rag.face = Some(dirty);
 513         }
 514         let mm3 = lifted * (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
 515         let w_mm = (rag.width * self.mm_per_unit) as f64;
 516         let cap = w_mm * w_mm * (CAP_UM as f64 / 1000.0);
 517         if pooled || material.is_some() {
 518             // (a step can lay back more than it lifts)
 519             rag.load = (rag.load + (mm3 / cap) as f32).clamp(0.0, 1.0);
 520             rag.soaked = (rag.soaked + (mm3 / (cap * FACES as f64)) as f32).clamp(0.0, 1.0);
 521         } else {
 522             rag.load = (rag.load + (mm3 / cap) as f32).min(1.0);
 523             rag.soaked = (rag.soaked + (mm3 / (cap * FACES as f64)) as f32).min(1.0);
 524         }
 525         mm3
 526     }
 527 
 528     /// Engine 3 (`SMEAR`): the face lays back the share `SMEAR` of the
 529     /// paint at its surface (`pl`) over the pixels in `px` where its rim
```

## S07 — Refold retires the active face

Path: `prototype/crates/paint/src/rag.rs`  
SHA-256: `e0242af63021738002d92ec77833c16fe513a42818b61e90848c1ed664f764a3`

Lines 189–204:

```text
 189     /// A clean rag bunched to a pad `width` units across.
 190     pub fn new(width: f32, seed: u64) -> Self {
 191         Rag { width: width.max(0.1), load: 0.0, soaked: 0.0, damp: 0.0, wet_at: 0.0, fold: 0, seed, cloth: None, face: None }
 192     }
 193 
 194     /// Turn a cleaner, dry face outward. No face is cleaner than the paint
 195     /// soaked through the whole cloth so far (`soaked`) leaves it. Counts
 196     /// the hand time in `t`.
 197     pub fn refold(&mut self, t: &mut Tally) {
 198         self.fold = self.fold.wrapping_add(1);
 199         self.cloth = None;
 200         self.face = None;
 201         self.load = self.soaked.clamp(0.0, 1.0);
 202         self.damp = 0.0;
 203         t.secs += pace::REFOLD;
 204     }
```

## S08 — Scalar hiding is inverted on a grayscale surrogate; RGB is rendered afterward

Path: `wip/look/crates/paint/src/pigment.rs`  
SHA-256: `9b57b84f722e72116d0fa972ecbd2e8fda9d5c420e6353ed5c6a1722c37382d5`

Lines 22–53:

```text
  22 /// K/S of a paint whose masstone (infinitely thick reflectance) is `r`.
  23 #[inline]
  24 pub fn ks_of(r: f32) -> f32 {
  25     let r = r.clamp(0.002, 0.995);
  26     (1.0 - r) * (1.0 - r) / (2.0 * r)
  27 }
  28 
  29 /// Hiding of a unit coat of a paint with masstone reflectance `r` and
  30 /// scattering `s`: its reflectance over black divided by over white (the
  31 /// paint industry's contrast ratio). 0 = clear glaze, 1 = hides completely.
  32 pub fn hiding_of(r: f32, s: f32) -> f32 {
  33     let p = Pigment { k: [s * ks_of(r); 3], s: [s; 3] };
  34     let b = p.over([0.0; 3], 1.0)[0];
  35     let w = p.over([1.0; 3], 1.0)[0];
  36     (b / w.max(1e-6)).clamp(0.0, 1.0)
  37 }
  38 
  39 /// The scattering (per coat) that gives a unit coat of masstone `r`
  40 /// (luminance) the contrast ratio `hiding`. Inverse of `hiding_of`.
  41 pub fn scatter_for(r: f32, hiding: f32) -> f32 {
  42     let h = hiding.clamp(1e-4, 0.9995);
  43     let (mut lo, mut hi) = (-9.0f32, 9.0f32); // ln s
  44     for _ in 0..40 {
  45         let mid = 0.5 * (lo + hi);
  46         if hiding_of(r, mid.exp()) < h {
  47             lo = mid;
  48         } else {
  49             hi = mid;
  50         }
  51     }
  52     (0.5 * (lo + hi)).exp()
  53 }
```

Lines 69–88:

```text
  69 /// Reflectance and transmittance of one channel (or one wavelength) of a
  70 /// layer with absorption `k` and scattering `s` per coat, `x` coats thick
  71 /// (`x` > 0).
  72 #[inline]
  73 pub(crate) fn layer1(k: f32, s: f32, x: f32) -> (f32, f32) {
  74     if s < 1e-6 {
  75         // pure absorber
  76         return (0.0, (-k * x).exp());
  77     }
  78     let a = 1.0 + k / s;
  79     let b = (a * a - 1.0).max(0.0).sqrt();
  80     if b < 1e-3 {
  81         // (nearly) non-absorbing: the b → 0 limit of the formulas below
  82         let sx = s * x;
  83         return (sx / (1.0 + a * sx), 1.0 / (1.0 + a * sx));
  84     }
  85     let bsx = (b * s * x).min(40.0);
  86     let (sh, ch) = (bsx.sinh(), bsx.cosh());
  87     let c = a * sh + b * ch;
  88     (sh / c, b / c)
```

Lines 108–121:

```text
 108     /// A paint of masstone `r` (the color it has laid thick, or over itself)
 109     /// that scatters `s` per coat, the same in every channel: scattering
 110     /// (by white and by particle edges) is nearly flat across the spectrum,
 111     /// absorption carries the hue.
 112     pub fn masstone(r: Rgb, s: f32) -> Self {
 113         let s = s.max(1e-6);
 114         Pigment { k: [s * ks_of(r[0]), s * ks_of(r[1]), s * ks_of(r[2])], s: [s; 3] }
 115     }
 116 
 117     /// A paint of masstone `r` with `hiding` (contrast ratio of a unit coat,
 118     /// measured on the luminance of the masstone).
 119     pub fn masstone_hiding(r: Rgb, hiding: f32) -> Self {
 120         Self::masstone(r, scatter_for(luminance(r), hiding))
 121     }
```

Lines 175–186:

```text
 175     /// Composite a layer of thickness `x` over an opaque substrate.
 176     #[inline]
 177     pub fn over(&self, sub: Rgb, x: f32) -> Rgb {
 178         if x <= 0.0 {
 179             return sub;
 180         }
 181         let (r, t) = self.layer(x);
 182         let mut out = [0.0; 3];
 183         for i in 0..3 {
 184             out[i] = r[i] + t[i] * t[i] * sub[i] / (1.0 - r[i] * sub[i]).max(1e-6);
 185         }
 186         out
```

## S08b — Palette documents hiding and stores raw/burnt values

Path: `wip/thinner/crates/paint/src/palette.rs`  
SHA-256: `23de3b6b5c80d5459ed0dca7254a9aaea2c79b3019d8065375ffe22e8644e271`

Lines 1–31:

```text
   1 //! Palettes: a few tube paints, and mixing on the palette.
   2 //!
   3 //! `Palette::pile` mixes explicitly supplied tube proportions in Mixbox
   4 //! latent space, weighted by each tube's tinting strength. The painter
   5 //! chooses the proportions.
   6 //!
   7 //! Masstone colors, hiding and tinting strength are approximations from
   8 //! pigment knowledge (not measurements); each palette's tube list cites its
   9 //! source.
  10 //!
  11 
  12 #[cfg(test)]
  13 use crate::canvas::Canvas;
  14 use crate::color::{Rgb, hex, luminance};
  15 #[cfg(all(test, tube_box))]
  16 use crate::color::to_oklab;
  17 use crate::drying::drier;
  18 use crate::pigment::{hiding_of, scatter_for};
  19 use crate::rng::Rng;
  20 use crate::wet::Paint;
  21 
  22 /// A tube (or hand-ground) paint.
  23 #[derive(Clone, Debug)]
  24 pub struct Tube {
  25     pub name: &'static str,
  26     /// What the paint is made of, in a few words (the guide's tube table).
  27     pub pigment: &'static str,
  28     /// Masstone color, linear RGB (the paint laid thick).
  29     pub color: Rgb,
  30     /// Hiding power of one coat of the tube paint (0 transparent .. 1
  31     /// opaque): contrast ratio, see `pigment::hiding_of`.
```

Lines 134–135:

```text
 134         #[cfg(any(feature = "box-sargent", feature = "box-inness"))]
 135         tube("raw sienna", "sienna earth, unroasted", "#9a6a2b", 0.4, 0.5, 0.7, drier::SIENNA),
```

Lines 162–163:

```text
 162         #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-hopper"))]
 163         tube("burnt sienna", "roasted sienna earth", "#7c3f24", 0.45, 0.55, 0.9, drier::SIENNA).engine3(drier::engine3::BURNT_SIENNA),
```

## S09 — Look variant definitions and results, including remaining contract failures

Path: `wip/look/notes/look/REPORT.md`  
SHA-256: `643e239dbc6feef5713920e05adcde0132f2dc7a8257e6342dffa5eb1b36ee5d`

Lines 76–102:

```text
  76 On the card itself (twenty strokes at load 0.5, the check's own scene):
  77 
  78 | | paint film | ratio (luminance) | difference kept |
  79 |---|---|---|---|
  80 | unthinned, raw | 37.9 µm | 0.500 | 17.9% |
  81 | unthinned, burnt | 37.9 µm | **0.464** | 9.8% |
  82 | thinned 0.5, raw | 1.64 µm | 0.040 | 87.3% |
  83 | thinned 0.5, burnt | 1.64 µm | **0.033** | 79.7% |
  84 
  85 Scattering per coat: raw 0.296, burnt 0.199. The tubes' own one-coat
  86 hiding is raw 0.40, burnt 0.45. That figure is a ratio taken on the
  87 masstone alone, not over a real black and white; over them, as above,
  88 burnt's ratio is lower.
  89 
  90 **Why they disagree.** Burnt sienna is much darker (masstone luminance
  91 0.080 against 0.174), so it darkens the white band more. That shrinks the
  92 black-to-white difference even though, relative to its own brightness,
  93 it lets more of the black show.
  94 
  95 `siennas.jpg` shows the same thin film of each (thinned 0.5) over light,
  96 mid-grey and dark bands: raw reads as a yellow veil on the light band,
  97 and burnt as a warmer, darker one.
  98 
  99 **Recommendation.** Leave the tubes alone. If you agree, check 13 (b)
 100 should be restated in terms of the contrast ratio: by that measure burnt
 101 sienna already passes ("more transparent than raw", Field/Salter). I
 102 haven't changed the test; that's your call.
```

Lines 121–168:

```text
 121 **What changed:**
 122 
 123 - **(a):** the plough is scaled by the share of paint in the liquid
 124   (× 0.5 at thinner 0.5).
 125 - **(b):** a thinned stroke leaves a pixel holding at most its ceiling
 126   more than it held when the stroke reached it. Paint lifted out makes
 127   room again, and paint ploughed in counts against the ceiling.
 128 - **(B), absolute:** a pixel holds at most the larger of the ceiling and
 129   what it held before.
 130 
 131 Unthinned paint never takes any of these paths: the baseline scenes
 132 without a rag are equal on every field, and the unthinned stroke's
 133 numbers are unchanged.
 134 
 135 | variant | stroke edges / inside, just after (µm) | after 30 min (µm) | broad pass: paint, unevenness (spread ÷ mean) | card kept, load 0.3 / 0.6 | thinner acceptance (27 required) |
 136 |---|---|---|---|---|---|
 137 | today | 2.25, 2.13 / 0.96 | 2.03, 1.93 / 0.96 | 2.8 µm, 0.47 | 84.4% / 81.4% | all pass |
 138 | (a) | 1.56, 1.39 / 1.35 | 1.51, 1.39 / 1.35 | 2.9 µm, 0.47 | 83.5% / 80.3% | 26 pass; **check 15 fails** |
 139 | (a), plough × (1 − share)² | 2.06, 1.95 / 1.60 | 2.02, 1.87 / 1.61 | 2.9 µm, 0.47 | 82.9% / 79.7% | 25 pass; checks 15 and 16 fail |
 140 | **(b), kept** | 3.00, 2.99 / 2.97 | 2.92, 2.98 / 2.97 | 7.7 µm, 0.49 | 73.5% / 71.7% | **all pass** |
 141 | (a) + (b) | 2.82, 2.94 / 2.98 | 2.77, 2.87 / 2.98 | 7.4 µm, 0.49 | 74.1% / 72.3% | all pass |
 142 | (B), reverted | 3.00, 2.99 / 2.97 | 2.92, 2.98 / 2.97 | 2.9 µm, **0.13** | 83.5% / 83.2% | 23 pass; **checks 7, 9, 15, 16 fail** |
 143 
 144 In every row, check 13 (b) fails as before (the siennas, above). "All
 145 pass" means every required test passes and only that known failure
 146 remains. The (a)+(b) and the squared-plough rows were measured with
 147 temporary switches (environment variables) in a single build and were not
 148 committed. The committed (a) and (b) are the same code without the
 149 switch.
 150 
 151 **What the failing checks say:**
 152 
 153 - **Check 15** (solvent doesn't slow the oil cure) needs at least 100
 154   pixels holding 12 µm or more after five thinned passes:
 155   - (a) leaves 79, and the squared plough 46;
 156   - (B) leaves none.
 157 
 158   With less plough, less paint heaps up into thick spots. Nothing about
 159   drying went wrong; the test no longer finds the thick film it measures.
 160 - **Check 16** (spreading mixes the liquids at a boundary) needs at least
 161   20 boundary pixels to gain paint. It got 3 (squared plough) and 2 (B).
 162   Again, the film it relies on isn't there.
 163 - **Check 7** (a second thinned pass adds at least 20% more paint than one)
 164   fails under (B): 20,379 against 23,187, only 14% more.
 165 - **Check 9** (five passes are thicker than one) fails under (B): the
 166   median is 3.0 µm for both.
 167 
 168 (B) contradicts the agreed rule that each thinned pass builds on the last.
```

## S09b — Net ceiling source

Path: `wip/look/crates/paint/src/bristle.rs`  
SHA-256: `01cfbb6698b38227728b7885e458f9c38c003c531d0b36932603048eec82d80e`

Lines 1480–1495:

```text
1480                 // a thinned stroke's ceiling is on the wet film a pixel holds
1481                 // when the stroke is done, against what it held when the
1482                 // stroke first reached it (`Wet::laid`): what the hairs and
1483                 // the plough lift from a pixel is room for paint again, and
1484                 // paint ploughed into a pixel counts against its ceiling as
1485                 // laid paint does
1486                 if capped && *sf.laid_id.add(i) != id {
1487                     *sf.laid_id.add(i) = id;
1488                     *sf.laid.add(i) = *sf.vol.add(i) + *sf.solv.add(i) / COAT_UM;
1489                 }
1490                 let v = *sf.vol.add(i);
1491                 // paint that is setting is stiff: it comes up and moves less
1492                 let fl = if sf.dry.is_null() { 1.0 } else { crate::drying::fluid((*sf.dry.add(i)).cure) };
1493                 // one stroke lifts only part of the film
1494                 if *sf.touched.add(i) != id {
1495                     *sf.touched.add(i) = id;
```

Lines 1537–1554:

```text
1537                 } else if dep_per_w > 0.0 {
1538                     // a thinned load lays liquid: no more than what is left
1539                     // of this pixel's ceiling for this stroke, shared by all
1540                     // its hairs (crate::thinner); the rest stays on the hair
1541                     let mut d = dep_per_w * wt;
1542                     if capped {
1543                         let now = *sf.vol.add(i) + *sf.solv.add(i) / COAT_UM;
1544                         d = d.min((cap - (now - *sf.laid.add(i))).max(0.0));
1545                     }
1546                     if d > 0.0 {
1547                         let cv = &mut *sf.cover.add(i);
1548                         *cv = if fine { ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0) } else { 1.0 };
1549                         sf.add(i, d * (1.0 - phi), &blat, bhide, bcure);
1550                         if !sf.solv.is_null() {
1551                             *sf.solv.add(i) += d * phi * COAT_UM;
1552                         }
1553                         *sf.stroke.add(i) = id;
1554                         laid_liq += d * px_area;
```

Lines 1598–1619:

```text
1598                                 // a clipped stroke can't push paint past its mask:
1599                                 // only the accepted share moves, the rest stays
1600                                 let m = m * share * clip.map_or(1.0, |c| c.at(ty * w + tx).0);
1601                                 // a thinned stroke adds no more wet film to the
1602                                 // pixel it ploughs into than its ceiling there
1603                                 // allows; the solvent goes with the paint
1604                                 // (solvent in µm, as the canvas holds it)
1605                                 let sol = if sf.solv.is_null() { 0.0 } else { *sf.solv.add(i) / COAT_UM };
1606                                 let vi = *sf.vol.add(i);
1607                                 let ms = if sol > 0.0 && vi > 0.0 { (sol * m / vi).min(sol) } else { 0.0 };
1608                                 let (m, ms) = if capped && j != i && m > 0.0 {
1609                                     if *sf.laid_id.add(j) != id {
1610                                         *sf.laid_id.add(j) = id;
1611                                         *sf.laid.add(j) = *sf.vol.add(j) + *sf.solv.add(j) / COAT_UM;
1612                                     }
1613                                     let now = *sf.vol.add(j) + *sf.solv.add(j) / COAT_UM;
1614                                     let room = (cap - (now - *sf.laid.add(j))).max(0.0);
1615                                     let f = if m + ms > room { room / (m + ms) } else { 1.0 };
1616                                     (m * f, ms * f)
1617                                 } else {
1618                                     (m, ms)
1619                                 };
```

## S10 — Old sienna optical and brush-generated measurements

Path: `wip/look/notes/look/logs/sienna.txt`  
SHA-256: `066810020fffcf294f4e46c0734dfbbbb2e7a4ac774d8b2d69b2114006591f7d`

Lines 1–17:

```text
   1 card bands: black Y 0.0126, white Y 0.7912
   2 raw sienna: masstone Y 0.1735, scattering 0.2961 per coat, hiding of one coat 0.4000
   3 burnt sienna: masstone Y 0.0797, scattering 0.1989 per coat, hiding of one coat 0.4500
   4 (1) a uniform film, the engine's own optics (Kubelka-Munk), thinned or not alike (the solvent has no color):
   5      3 µm raw sienna  : contrast ratio on a black/80% white chart Y 0.047  R 0.044 G 0.047 B 0.091; on the card's bands Y 0.062  R 0.060 G 0.062 B 0.115; difference kept on the card 78.8%
   6      3 µm burnt sienna: contrast ratio on a black/80% white chart Y 0.035  R 0.030 G 0.036 B 0.059; on the card's bands Y 0.051  R 0.047 G 0.051 B 0.080; difference kept on the card 66.5%
   7     10 µm raw sienna  : contrast ratio on a black/80% white chart Y 0.169  R 0.143 G 0.177 B 0.798; on the card's bands Y 0.182  R 0.153 G 0.191 B 0.821; difference kept on the card 49.7%
   8     10 µm burnt sienna: contrast ratio on a black/80% white chart Y 0.154  R 0.104 G 0.192 B 0.661; on the card's bands Y 0.166  R 0.116 G 0.206 B 0.695; difference kept on the card 30.6%
   9     25 µm raw sienna  : contrast ratio on a black/80% white chart Y 0.452  R 0.341 G 0.522 B 1.000; on the card's bands Y 0.457  R 0.342 G 0.532 B 1.000; difference kept on the card 20.5%
  10     25 µm burnt sienna: contrast ratio on a black/80% white chart Y 0.438  R 0.277 G 0.761 B 0.999; on the card's bands Y 0.439  R 0.281 G 0.767 B 0.999; difference kept on the card 10.5%
  11     30 µm raw sienna  : contrast ratio on a black/80% white chart Y 0.537  R 0.403 G 0.631 B 1.000; on the card's bands Y 0.539  R 0.402 G 0.639 B 1.000; difference kept on the card 15.6%
  12     30 µm burnt sienna: contrast ratio on a black/80% white chart Y 0.512  R 0.337 G 0.877 B 1.000; on the card's bands Y 0.511  R 0.339 G 0.881 B 1.000; difference kept on the card 8.2%
  13 (2) the card painted as check 13 (b) paints it, twenty strokes at load 0.5:
  14   thinner 0: raw sienna   mean paint 37.92 µm; contrast ratio Y 0.500  R 0.413 G 0.553 B 0.984; difference kept 17.9%
  15   thinner 0: burnt sienna mean paint 37.92 µm; contrast ratio Y 0.464  R 0.343 G 0.655 B 0.968; difference kept 9.8%
  16   thinner 0.5: raw sienna   mean paint 1.64 µm; contrast ratio Y 0.040  R 0.041 G 0.040 B 0.051; difference kept 87.3%
  17   thinner 0.5: burnt sienna mean paint 1.64 µm; contrast ratio Y 0.033  R 0.034 G 0.033 B 0.039; difference kept 79.7%
```

## S10b — Sienna measurement fixture and shared-substrate comparisons

Path: `wip/look/crates/paint/tests/look_sienna.rs`  
SHA-256: `4399eed9152223fd37126e3c988a64a90115416815d823db9a7ec61d1ec91eae`

Lines 71–113:

```text
  71     let c0 = card();
  72     let (kb, kw) = (mean_rgb(&c0, BLACK), mean_rgb(&c0, WHITE));
  73     let diff0 = luminance(kw) - luminance(kb);
  74     println!("card bands: black Y {:.4}, white Y {:.4}", luminance(kb), luminance(kw));
  75     for name in ["raw sienna", "burnt sienna"] {
  76         let p = sargent(name);
  77         println!("{name}: masstone Y {:.4}, scattering {:.4} per coat, hiding of one coat {:.4}", luminance(p.color), p.scatter, p.hiding());
  78     }
  79     // (25 µm is one coat: the tubes' `hiding` is the contrast ratio of one coat on luminance alone)
  80     println!("\n(1) a uniform film, the engine's own optics (Kubelka-Munk), thinned or not alike (the solvent has no color):");
  81     for um in [3.0f32, 10.0, 25.0, 30.0] {
  82         let x = um / paint::COAT_UM;
  83         for name in ["raw sienna", "burnt sienna"] {
  84             let p = sargent(name);
  85             let pig = Pigment::masstone(p.color, p.scatter);
  86             // the standard chart: black (0) and a white of 80% reflectance
  87             let (b, w) = (pig.over([0.0; 3], x), pig.over([0.8; 3], x));
  88             // the card's own black and white bands
  89             let (cb, cw) = (pig.over(kb, x), pig.over(kw, x));
  90             let kept = (luminance(cw) - luminance(cb)) / diff0;
  91             println!("  {um:>4} µm {name:<12}: contrast ratio on a black/80% white chart {}; on the card's bands {}; difference kept on the card {:.1}%", ratio(b, w), ratio(cb, cw), 100.0 * kept);
  92         }
  93     }
  94     println!("\n(2) the card painted as check 13 (b) paints it, twenty strokes at load 0.5:");
  95     for t in [0.0f32, 0.5] {
  96         for name in ["raw sienna", "burnt sienna"] {
  97             let p = sargent(name);
  98             let (b, w, um) = painted(c0.clone(), if t > 0.0 { p.with_thinner(t) } else { p });
  99             println!("  thinner {t}: {name:<12} mean paint {um:.2} µm; contrast ratio {}; difference kept {:.1}%", ratio(b, w), 100.0 * (luminance(w) - luminance(b)) / diff0);
 100         }
 101     }
 102 }
 103 
 104 /// The two siennas, the same thin film (thinner 0.5, load 0.5, the same
 105 /// strokes), over light, mid-grey and dark bands: one 256 px panel each,
 106 /// written to $LOOK_OUT as sienna_raw.png and sienna_burnt.png.
 107 #[test]
 108 #[ignore = "a picture for notes/look"]
 109 fn siennas_side_by_side() {
 110     let out = std::env::var("LOOK_OUT").expect("LOOK_OUT");
 111     let pal = Palette::named_box("sargent").unwrap();
 112     let idx = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
 113     let grey = pal.pile(vec![(idx("lead white"), 0.6), (idx("bone black"), 0.4)]).laid(0.0);
```

## S11 — Current check 13(b)

Path: `wip/thinner/crates/paint/tests/thinner_pigments.rs`  
SHA-256: `53924605cde088b16056fd31d69d57f96c5eae8d5c4674858326823bb0ca06cc`

Lines 101–120:

```text
 101 /// Check 13 (b): measured the card's way, an equal (solvent-free) film of
 102 /// burnt sienna lets at least as much of the card's black/white contrast
 103 /// show as raw sienna. Field/Salter 1869 §155 calls burnt sienna "more
 104 /// transparent than the raw earth". Prints both tubes' hiding and
 105 /// scattering per coat alongside.
 106 #[test]
 107 fn c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna() {
 108     for name in ["raw sienna", "burnt sienna"] {
 109         let t = paint::palette::catalog().into_iter().find(|t| t.name == name).unwrap();
 110         println!("{name}: hiding {} (contrast ratio of one coat), scattering {} per coat, masstone luminance {}", t.hiding, scatter_for(luminance(t.color), t.hiding), luminance(t.color));
 111     }
 112     let (raw, film_raw) = card_kept(sargent("raw sienna"));
 113     let (burnt, film_burnt) = card_kept(sargent("burnt sienna"));
 114     let worst = film_raw.iter().zip(&film_burnt).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
 115     let total: f32 = film_raw.iter().sum();
 116     println!("card contrast showing: raw sienna {:.2}%, burnt sienna {:.2}% (films differ by at most {worst} µm; raw's mean {} µm)", 100.0 * raw, 100.0 * burnt, total / film_raw.len() as f32);
 117     assert!(total > 0.0, "the strokes laid paint");
 118     assert!(worst <= 1e-3, "the two films are equal (solvent-free): they differ by up to {worst} µm at a pixel");
 119     assert!(burnt >= raw, "burnt sienna shows {:.2}% of the card, raw sienna {:.2}%: Field/Salter (§155) has burnt the more transparent", 100.0 * burnt, 100.0 * raw);
 120 }
```

## S12 — Clock boundary flow scheduling

Path: `wip/thinner/crates/paint/src/drying.rs`  
SHA-256: `f4d0df02422cf403f4566fe96444b21a72144c3c2d897122e44060032be50531`

Lines 377–413:

```text
 377         let dt = minutes.max(0.0);
 378         assert!(dt.is_finite(), "Canvas::wait: minutes must be finite (got {minutes}); use dry() to wait until touch-dry");
 379         if self.engine >= 3 && self.wet.has_solvent(self.f.w) {
 380             self.wait_on_grid(dt);
 381             return;
 382         }
 383         self.age(dt);
 384         self.wet.clock.now += dt as f64;
 385     }
 386 
 387     /// Wait `dt` minutes with solvent in the paint (engine 3,
 388     /// `crate::thinner`): the solvent's loss, the flow it gives the paint
 389     /// and the oil's drying all step on one clock, in steps that end on
 390     /// whole minutes counted from the canvas's start (not from this wait),
 391     /// so a wait split on whole minutes is exactly the wait in one, and
 392     /// brushwork's hand time (which waits too) keeps the same grid. The
 393     /// solvent's loss and the oil's drying run over every step, whole or
 394     /// part (both exact in closed form); the flow runs once per whole
 395     /// minute, as each minute of the grid ends. Once the last solvent is
 396     /// gone the rest of the wait is the ordinary one.
 397     fn wait_on_grid(&mut self, dt: f32) {
 398         let start = self.wet.clock.now;
 399         let end = start + dt as f64;
 400         let mut t = start;
 401         while t < end {
 402             let next = (t.floor() + 1.0).min(end);
 403             let step = (next - t) as f32;
 404             self.evaporate(step);
 405             self.age(step);
 406             if next == next.floor() {
 407                 self.spread(1.0);
 408             }
 409             t = next;
 410             self.wet.clock.now = t;
 411             if !self.wet.has_solvent(self.f.w) {
 412                 if end > t {
 413                     self.age((end - t) as f32);
```

## S13 — Buildup and thickness-dependent evaporation fixture

Path: `wip/thinner/crates/paint/tests/thinner_physics.rs`  
SHA-256: `e0408f09dbccde323b4e525e17a355c5a705218c20af1e2577976b4bc59b7e04`

Lines 128–169:

```text
 128             .iter()
 129             .enumerate()
 130             .flat_map(|(k, &y)| {
 131                 let mut r = profile(&c, y, 2.0, 20.0, 980.0, 10.0);
 132                 if k % 2 == 1 {
 133                     r.reverse();
 134                 }
 135                 r
 136             })
 137             .collect();
 138         let early = p[1..6].iter().copied().fold(0.0, f32::max);
 139         let late = mean(&p[p.len() - 10..]) as f32;
 140         assert!(early > theta, "load {load}: the stroke starts laying paint ({early} µm, θ {theta} µm)");
 141         assert!(late < 0.5 * early, "load {load}: an emptying brush lays less ({early} µm early, {late} µm over the last 100 units)");
 142         let last = p.iter().rposition(|&v| v >= theta).expect("some paint above θ");
 143         assert!(last + 1 < p.len(), "load {load}: the brush ran out before the stroke's end");
 144         reach.push(10.0 * (last + 1) as f32);
 145         println!("load {load}: paint above θ for {} units of the rows (profile {p:?})", reach[reach.len() - 1]);
 146     }
 147     assert!(reach[1] >= reach[0] + 30.0, "load 0.6 lays paint farther than 0.3: to {} vs {}", reach[1], reach[0]);
 148 }
 149 
 150 /// Check 6. Pressure: the same thinned load lays more paint at more
 151 /// pressure (at least 5% more per step, so a tie fails), and never more
 152 /// than the stroke limit on any pixel (rounding: 1e-3 relative + 1e-3 µm).
 153 #[test]
 154 fn c06_more_pressure_lays_more_paint_up_to_the_stroke_limit() {
 155     let t = 0.5;
 156     let lim = stroke_limit_um(t);
 157     let mut laid = Vec::new();
 158     for pr in [0.3f32, 0.6, 0.9] {
 159         let mut c = canvas(400);
 160         let w0 = wet_film(&c);
 161         let mut h = brush(Tool::filbert(20.0), 31, raw_sienna().with_thinner(t), 0.6);
 162         let p0 = h.carried().0;
 163         stroke(&mut c, &mut h, vec![(150.0, 500.0), (450.0, 510.0)], pr);
 164         laid.push(p0 - h.carried().0);
 165         let most = wet_film(&c).iter().zip(&w0).map(|(b, a)| b - a).fold(0.0, f32::max);
 166         assert!(most <= lim * 1.001 + 1e-3, "pressure {pr}: a pixel took {most} µm of wet film; the stroke limit is {lim} µm");
 167     }
 168     assert!(laid[1] > 1.05 * laid[0] && laid[2] > 1.05 * laid[1], "paint laid at pressure 0.3, 0.6, 0.9: {laid:?}");
 169 }
```

Lines 199–234:

```text
 199 /// Check 9. Evaporation follows `solvent = s0 × exp(-t / τ(h))` at a fixed
 200 /// paint thickness h, τ from `evaporation_tau_min` (the model's own,
 201 /// labeled estimate; the equation is the plan's and is computed here).
 202 /// Two films on a flat ground, one thinned pass and five passes (thicker).
 203 /// Every minute, at every pixel whose paint changed by at most 1e-3 of
 204 /// itself (spreading barely touched it) and still holds ≥ 0.01 µm of
 205 /// solvent: the share of its solvent left equals exp(-1 min / τ(h)) within
 206 /// 1e-4 + 2 × the paint's relative change (a pixel whose paint moved by δ
 207 /// can have its solvent moved by about that much again). At least 200 such
 208 /// pixel-minutes in each film; the thick film loses a smaller share per
 209 /// minute. Immediate loss, linear loss or a wrong rate fail. Also: τ is
 210 /// positive, finite and never shorter for a thicker film; total solvent
 211 /// falls every minute; paint stays (`BALANCE_REL`: only solvent leaves the
 212 /// wet film); after ten times the slowest τ, under a thousandth is left.
 213 /// That the solvent adds no color is check 19.
 214 #[test]
 215 fn c09_the_solvent_evaporates_and_the_film_loses_its_volume() {
 216     let mut prev_tau = 0.0f64;
 217     for um in [0.5f32, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0] {
 218         let tau = evaporation_tau_min(um);
 219         assert!(tau.is_finite() && tau > 0.0 && tau >= prev_tau, "τ({um} µm) = {tau} min after {prev_tau}");
 220         prev_tau = tau;
 221     }
 222     let mut c = smooth_canvas(300);
 223     thinned_patch(&mut c, raw_sienna(), 0.5, (100.0, 450.0), (300.0, 700.0), 41);
 224     for k in 0..5 {
 225         thinned_patch(&mut c, raw_sienna(), 0.5, (550.0, 900.0), (300.0, 700.0), 141 + 20 * k);
 226     }
 227     let groups = [in_rect(&c, (150.0, 350.0, 400.0, 650.0)), in_rect(&c, (600.0, 350.0, 850.0, 650.0))];
 228     let h0 = paint_um(&c);
 229     let med: Vec<f32> = groups.iter().map(|g| median(&g.iter().map(|&(i, _, _)| h0[i]).collect::<Vec<_>>())).collect();
 230     assert!(med[1] > 1.5 * med[0], "the thick film is thicker: median {} vs {} µm", med[1], med[0]);
 231     let (s0, p0) = (c.solvent_total(), c.wet_total());
 232     assert!(s0 > 0.0, "solvent laid");
 233     let tau_max = evaporation_tau_min(max_paint_um(&c));
 234     let minutes = (10.0f64 * tau_max).ceil() as usize + 1;
```

## S14 — RGB luminance and sRGB conversion

Path: `wip/look/crates/paint/src/color.rs`  
SHA-256: `8aba2a75810b5016665dcf7d7e5a1fa9eb731bd5c913db0e0e2cc1cf918e3494`

Lines 1–22:

```text
   1 //! Color helpers. All `Rgb` values are linear-light.
   2 
   3 pub type Rgb = [f32; 3];
   4 
   5 #[inline]
   6 pub fn srgb_to_linear(c: f32) -> f32 {
   7     if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
   8 }
   9 
  10 #[inline]
  11 pub fn linear_to_srgb(c: f32) -> f32 {
  12     let c = c.clamp(0.0, 1.0);
  13     if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
  14 }
  15 
  16 /// Parse "#rrggbb" (sRGB) into linear RGB.
  17 pub fn hex(s: &str) -> Rgb {
  18     let s = s.trim_start_matches('#');
  19     let v = u32::from_str_radix(s, 16).expect("bad hex color");
  20     let f = |shift: u32| srgb_to_linear(((v >> shift) & 0xff) as f32 / 255.0);
  21     [f(16), f(8), f(0)]
  22 }
```

Lines 60–63:

```text
  60 #[inline]
  61 pub fn luminance(c: Rgb) -> f32 {
  62     0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]
  63 }
```
