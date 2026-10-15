use super::*;
use rayon::prelude::*;

const MIN_WORK: usize = 8192;

pub(super) struct Contact {
    pub bristle: usize,
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub rb: f32,
    pub reach: f32,
}

#[derive(Clone, Copy)]
struct Sample {
    vol: f32,
    lat: Latent,
    hide: Prop,
    cure: f32,
    solvent: f32,
}

struct Cell {
    pixel: usize,
    sample: Sample,
    allowance: f32,
    pickup: f32,
    deposit: f32,
    moves: [(usize, f32); 4],
}

struct Plan {
    bristle: usize,
    phi: f32,
    cap: f32,
    cells: Vec<Cell>,
    bounds: Bounds,
}

unsafe fn sample(sf: Surf, i: usize) -> Sample {
    unsafe {
        Sample {
            vol: *sf.vol.add(i),
            lat: *sf.lat.add(i),
            hide: *sf.hide.add(i),
            cure: if sf.dry.is_null() { 0.0 } else { (*sf.dry.add(i)).cure },
            solvent: if sf.solv.is_null() { 0.0 } else { *sf.solv.add(i) / COAT_UM },
        }
    }
}

fn rectangle(c: &Contact, sf: Surf, lim: Rect) -> Rect {
    let x0 = ((c.a.0.min(c.b.0) - c.rb - 1.0).floor().max(0.0) as usize).max(lim.0).max(sf.ox);
    let y0 = ((c.a.1.min(c.b.1) - c.rb - 1.0).floor().max(0.0) as usize).max(lim.1).max(sf.oy);
    let x1 = ((c.a.0.max(c.b.0) + c.rb + 1.0).ceil().max(0.0) as usize).min(lim.2).min(sf.ox + sf.w);
    let y1 = ((c.a.1.max(c.b.1) + c.rb + 1.0).ceil().max(0.0) as usize).min(lim.3).min(sf.oy + sf.h);
    (x0, y0, x1.max(x0), y1.max(y0))
}

unsafe fn plan(sf: Surf, br: &Bristle, tool: &Tool, c: &Contact, full: f32, clip: Option<Clip<'_>>, id: u32, lim: Rect) -> Plan {
    unsafe {
        let rect @ (x0, y0, x1, y1) = rectangle(c, sf, lim);
        let liquid = br.vol + br.solvent;
        let phi = if liquid > 0.0 { br.solvent / liquid } else { 0.0 };
        let exch = phi > 0.0 && crate::thinner::exchange();
        let cap = if phi > 0.0 && !exch { crate::thinner::stroke_limit_um(phi) / COAT_UM } else { f32::INFINITY };
        let mut out = Plan { bristle: c.bristle, phi, cap, cells: Vec::new(), bounds: None };
        if x0 == x1 || y0 == y1 {
            return out;
        }
        let (dx, dy) = (c.b.0 - c.a.0, c.b.1 - c.a.1);
        let seg2 = dx * dx + dy * dy;
        let seg = seg2.sqrt();
        let (mx, my) = if seg > 1e-4 { (dx / seg, dy / seg) } else { (0.0, 0.0) };
        let (nx, ny) = (-my, mx);
        let th = 1.0 - c.reach.max(WET_REACH * smoothstep(0.1, 0.8, liquid / full)) * 1.6;
        let give = 0.15 + 0.45 * (1.0 - tool.stiffness).clamp(0.0, 1.0);
        let mut weighted = Vec::with_capacity((x1 - x0) * (y1 - y0));
        let (mut sum_w, mut sum_cov, mut sum_tack, mut sum_k) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let t = if seg2 > 1e-8 { (((px - c.a.0) * dx + (py - c.a.1) * dy) / seg2).clamp(0.0, 1.0) } else { 0.0 };
                let (qx, qy) = (c.a.0 + dx * t - px, c.a.1 + dy * t - py);
                let dist = (qx * qx + qy * qy).sqrt();
                if dist > c.rb + 0.5 {
                    continue;
                }
                let cov = 1.0 - smoothstep(c.rb * 0.5, c.rb + 0.5, dist);
                if cov <= 0.0 {
                    continue;
                }
                sum_cov += cov;
                let i = (y - sf.oy) * sf.w + x - sf.ox;
                let surf = (*sf.base.add(i) + 0.35 * *sf.vol.add(i)).min(1.5);
                let (k, lift) = clip.map_or((1.0, 0.0), |cl| cl.at(y * sf.fw + x));
                let wt = cov * smoothstep(th + lift - give, th + lift + 0.2, surf) * k;
                sum_w += wt;
                sum_k += wt * k * k;
                if !sf.dry.is_null() {
                    let p = *sf.dry.add(i);
                    sum_tack += wt * crate::drying::feel(*sf.vol.add(i), p.cure, p.sub).1;
                }
                if wt > 0.0 {
                    weighted.push((x, y, i, wt));
                }
            }
        }
        if sum_w <= 1e-6 {
            return out;
        }
        let touch = (sum_w / sum_cov.max(1e-6)).min(1.0);
        let travel = (seg / sf.scale).max(c.rb / sf.scale * 0.5);
        let travel = if exch { travel * (1.0 - phi).powf(crate::thinner::exchange_kj().0) } else { travel };
        let mut dep_total = liquid * (1.0 - (-travel / tool.run).exp()) * touch;
        if matches!(clip, Some(Clip::Fence { .. })) {
            dep_total *= (sum_k / sum_w).min(1.0);
        }
        let tack = sum_tack / sum_w;
        if tack > 0.0 {
            let g = crate::drying::grab(tack) * crate::drying::stick(c.b.0, c.b.1, c.rb, br.seed, tack);
            dep_total = (liquid * (1.0 - (-travel * g / tool.run).exp()) * touch).min(liquid * 0.9);
        }
        let ps = (br.hide[1] * (1.0 - br.hide[3]).powi(2)).clamp(0.0, 1.0);
        let stiffen = ((ps - 0.4) / 0.6).clamp(0.0, 1.0);
        if sf.clump && stiffen > 0.0 {
            let clump_u = tool.hair_radius() * (1.0 + 4.0 * stiffen * stiffen);
            let bands = (tool.width / (2.0 * clump_u)).clamp(2.0, 48.0);
            let u = (br.rx.clamp(-1.0, 1.0) + 1.0) * 0.5 * bands;
            let (b0, f) = (u.floor(), u - u.floor());
            let h0 = crate::rng::hash2(b0 as i64, id as i64, 0x51C0);
            let h1 = crate::rng::hash2(b0 as i64 + 1, id as i64, 0x51C0);
            let band = h0 + (h1 - h0) * smoothstep(0.0, 1.0, f);
            dep_total = (dep_total * (1.0 + 0.6 * stiffen * (2.0 * band - 1.0))).min(liquid * 0.95);
        }
        let dep_per_w = dep_total / sum_w * sf.scale * sf.scale;
        let hunger = 0.35 + 0.65 * (1.0 - liquid / full).clamp(0.0, 1.0).powf(1.5);
        let hair = (tool.hair_radius() * sf.scale).min(c.rb);
        let push_k = tool.push * (seg / (2.0 * hair)).clamp(0.0, 1.0) * (hair / c.rb);
        let off = if sf.clump && stiffen > 0.0 && br.rx.abs() > 0.7 {
            (2.0 * tool.hair_radius() * sf.scale * (1.0 + 4.0 * stiffen * stiffen)).max(2.0 * hair).min(3.0)
        } else {
            2.0 * hair
        };
        out.cells.reserve(weighted.len());
        for (x, y, i, wt) in weighted {
            let sample = sample(sf, i);
            let fl = crate::drying::fluid(sample.cure);
            let floor = if *sf.touched.add(i) == id { *sf.floor.add(i) } else { sample.vol * (1.0 - tool.pickup * fl) };
            let allowance = if exch { sample.vol } else { (sample.vol - floor).max(0.0) };
            let own = if !exch && *sf.stroke.add(i) == id { 0.15 } else { 1.0 };
            let pickup = if sample.vol > 1e-6 { (sample.vol * tool.pickup * wt * hunger * own * fl).min(allowance) } else { 0.0 };
            let mut moves = [(i, 0.0); 4];
            let m = sample.vol * push_k * wt * fl;
            if m > 1e-6 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let side = if (px - c.a.0) * nx + (py - c.a.1) * ny >= 0.0 { 1.0 } else { -1.0 };
                let gx = px + (nx * side * 0.75 + mx * 0.45) * off - 0.5;
                let gy = py + (ny * side * 0.75 + my * 0.45) * off - 0.5;
                let (fx, fy) = (gx - gx.floor(), gy - gy.floor());
                let targets = [(0, 0, (1.0 - fx) * (1.0 - fy)), (1, 0, fx * (1.0 - fy)), (0, 1, (1.0 - fx) * fy), (1, 1, fx * fy)];
                for (slot, (dx, dy, share)) in moves.iter_mut().zip(targets) {
                    let (tx, ty) = (gx.floor() + 0.5 + dx as f32, gy.floor() + 0.5 + dy as f32);
                    if tx < lim.0 as f32 || ty < lim.1 as f32 || tx >= lim.2 as f32 || ty >= lim.3 as f32 {
                        continue;
                    }
                    let (tx, ty) = (tx as usize, ty as usize);
                    let j = (ty - sf.oy) * sf.w + tx - sf.ox;
                    if j != i {
                        *slot = (j, m * share * clip.map_or(1.0, |cl| cl.at(ty * sf.fw + tx).0));
                    }
                }
            }
            out.cells.push(Cell { pixel: i, sample, allowance, pickup, deposit: dep_per_w * wt, moves });
        }
        let pad = (off + 2.0) as usize;
        out.bounds = Some((rect.0.saturating_sub(pad).max(sf.ox), rect.1.saturating_sub(pad).max(sf.oy), (rect.2 + pad).min(sf.ox + sf.w), (rect.3 + pad).min(sf.oy + sf.h)));
        out
    }
}

#[derive(Default)]
struct Budget {
    pickup: f64,
    moved: f64,
    allowance: f32,
    volume: f32,
    removed: f64,
    solvent_removed: f64,
    used: f32,
    initialized: bool,
    touched: bool,
}

fn reserve(used: &mut f32, cap: f32, volume: f32) -> f32 {
    let accepted = volume.min((cap - *used).max(0.0));
    if cap.is_finite() {
        *used += accepted;
    }
    accepted
}

#[derive(Clone, Copy)]
enum Origin {
    Brush(usize),
    Canvas(Sample),
}

struct Incoming {
    pixel: usize,
    volume: f32,
    solvent: f32,
    origin: Origin,
}

unsafe fn merge(sf: Surf, bristles: &mut [Bristle], tool: &Tool, plans: &[Plan], id: u32, bounds: &mut Bounds) {
    unsafe {
        let Some(rect) = plans.iter().filter_map(|p| p.bounds).reduce(|a, b| (a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3))) else { return };
        let bw = rect.2 - rect.0;
        let local = |i: usize| (i / sf.w + sf.oy - rect.1) * bw + i % sf.w + sf.ox - rect.0;
        let mut budgets: Vec<Budget> = (0..bw * (rect.3 - rect.1)).map(|_| Budget::default()).collect();
        let mut incoming = Vec::new();
        for p in plans {
            for c in &p.cells {
                let b = &mut budgets[local(c.pixel)];
                b.pickup += c.pickup as f64;
                b.moved += c.moves.iter().map(|m| m.1 as f64).sum::<f64>();
                b.allowance = c.allowance;
                b.volume = c.sample.vol;
                b.touched = true;
            }
        }
        let area = 1.0 / (sf.scale * sf.scale);
        let mut deposited = vec![0.0f64; bristles.len()];
        let mut picked = vec![(0.0f64, [0.0f64; LAT], [0.0f64; 5], 0.0f64, 0.0f64); bristles.len()];
        for p in plans {
            for c in &p.cells {
                let b = &budgets[local(c.pixel)];
                let pickup_scale = if b.pickup > 0.0 { (b.allowance as f64 / b.pickup).min(1.0) } else { 1.0 };
                let outgoing = b.pickup * pickup_scale + b.moved;
                let scale = if outgoing > 0.0 { (b.volume as f64 / outgoing).min(1.0) } else { 1.0 };
                let pickup = (c.pickup as f64 * pickup_scale * scale) as f32;
                let solvent = if c.sample.vol > 0.0 { c.sample.solvent * pickup / c.sample.vol } else { 0.0 };
                let b = &mut budgets[local(c.pixel)];
                b.removed += pickup as f64;
                b.solvent_removed += solvent as f64;
                let got = &mut picked[p.bristle];
                let v = pickup as f64 * area as f64;
                got.0 += v;
                for k in 0..LAT { got.1[k] += c.sample.lat[k] as f64 * v; }
                for k in 0..5 { got.2[k] += c.sample.hide[k] as f64 * v; }
                got.3 += c.sample.cure as f64 * v;
                got.4 += solvent as f64 * area as f64;
                let accept = |budgets: &mut [Budget], pixel: usize, amount: f32, cap: f32| {
                    let b = &mut budgets[local(pixel)];
                    if !b.initialized {
                        b.used = if !sf.laid.is_null() && *sf.laid_id.add(pixel) == id { *sf.laid.add(pixel) } else { 0.0 };
                        b.initialized = true;
                    }
                    reserve(&mut b.used, cap, amount)
                };
                let d = accept(&mut budgets, c.pixel, c.deposit, p.cap);
                if d > 0.0 {
                    deposited[p.bristle] += d as f64 * area as f64;
                    incoming.push(Incoming { pixel: c.pixel, volume: d * (1.0 - p.phi), solvent: d * p.phi, origin: Origin::Brush(p.bristle) });
                }
                for &(dest, requested) in &c.moves {
                    let m = (requested as f64 * scale) as f32;
                    let ms = if c.sample.vol > 0.0 { c.sample.solvent * m / c.sample.vol } else { 0.0 };
                    if m <= 0.0 { continue }
                    let accepted = accept(&mut budgets, dest, m + ms, p.cap);
                    let f = accepted / (m + ms);
                    let (m, ms) = (m * f, ms * f);
                    let b = &mut budgets[local(c.pixel)];
                    b.removed += m as f64;
                    b.solvent_removed += ms as f64;
                    incoming.push(Incoming { pixel: dest, volume: m, solvent: ms, origin: Origin::Canvas(c.sample) });
                }
            }
        }
        for (j, b) in budgets.iter().enumerate() {
            let (x, y) = (rect.0 + j % bw, rect.1 + j / bw);
            let i = (y - sf.oy) * sf.w + x - sf.ox;
            if b.touched && *sf.touched.add(i) != id {
                *sf.touched.add(i) = id;
                let fl = if sf.dry.is_null() { 1.0 } else { crate::drying::fluid((*sf.dry.add(i)).cure) };
                *sf.floor.add(i) = *sf.vol.add(i) * (1.0 - tool.pickup * fl);
            }
            if b.removed > 0.0 {
                sf.take(i, (b.removed as f32).min(*sf.vol.add(i)));
            }
            if !sf.solv.is_null() && b.solvent_removed > 0.0 {
                *sf.solv.add(i) = (*sf.solv.add(i) - b.solvent_removed as f32 * COAT_UM).max(0.0);
            }
            if b.initialized && !sf.laid.is_null() {
                *sf.laid_id.add(i) = id;
                *sf.laid.add(i) = b.used;
            }
        }
        for e in incoming {
            let (lat, hide, cure) = match e.origin {
                Origin::Brush(bi) => (bristles[bi].lat, bristles[bi].hide, bristles[bi].cure),
                Origin::Canvas(s) => (s.lat, s.hide, s.cure),
            };
            sf.add(e.pixel, e.volume, &lat, hide, cure);
            if !sf.solv.is_null() { *sf.solv.add(e.pixel) += e.solvent * COAT_UM; }
            *sf.cover.add(e.pixel) = 1.0;
            if matches!(e.origin, Origin::Brush(_)) { *sf.stroke.add(e.pixel) = id; }
        }
        for (i, b) in bristles.iter_mut().enumerate() {
            let liquid = b.vol + b.solvent;
            let gone = (deposited[i] as f32).min(liquid);
            let phi = if liquid > 0.0 { b.solvent / liquid } else { 0.0 };
            b.vol = (b.vol - gone * (1.0 - phi)).max(0.0);
            b.solvent = (b.solvent - gone * phi).max(0.0);
            let got = picked[i];
            b.solvent += got.4 as f32;
            if got.0 > 0.0 {
                let lat = got.1.map(|v| (v / got.0) as f32);
                let hide = got.2.map(|v| (v / got.0) as f32);
                b.cure = mix_cure(b.cure, b.vol, (got.3 / got.0) as f32, got.0 as f32);
                mix_into(&mut b.vol, &mut b.lat, &mut b.hide, got.0 as f32, &lat, hide);
            }
        }
        grow(bounds, rect.0, rect.1, rect.2, rect.3);
    }
}

pub(super) unsafe fn stamp(sf: Surf, bristles: &mut [Bristle], tool: &Tool, contacts: &[Contact], full: f32, clip: Option<Clip<'_>>, id: u32, bounds: &mut Bounds, lim: Rect) -> bool {
    if sf.ox != 0 || sf.oy != 0 || sf.w != sf.fw || sf.h != sf.fh || contacts.len() < 8 {
        return false;
    }
    let work: usize = contacts.iter().map(|c| { let r = rectangle(c, sf, lim); (r.2 - r.0) * (r.3 - r.1) }).sum();
    if work < MIN_WORK {
        return false;
    }
    unsafe {
        let plans: Vec<_> = contacts.par_iter().map(|c| plan(sf, &bristles[c.bristle], tool, c, full, clip, id, lim)).collect();
        merge(sf, bristles, tool, &plans, id, bounds);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::hex;

    fn fixture(thinner: f32, volume: f32) -> (Canvas, Held, Vec<Contact>) {
        let mut canvas = Canvas::new(128, 1.0, hex("#e8e0d0")).with_engine(5);
        let red = Paint::body(hex("#c33628"));
        let blue = Paint::body(hex("#2458a4"));
        canvas.wet.ensure_solvent();
        for i in 0..canvas.wet.vol.len() {
            let paint = if i % 128 < 64 { red } else { blue };
            canvas.wet.vol[i] = volume;
            canvas.wet.lat[i] = paint.latent();
            canvas.wet.hide[i] = paint.prop();
            canvas.wet.solv[i] = volume * 0.3 * COAT_UM;
        }
        let mut held = Held::new(Tool::hog_flat(180.0), 7);
        held.load(Paint::body(hex("#e4b748")).with_thinner(thinner), 0.8);
        let contacts = (0..held.bristles.len()).map(|bristle| Contact {
            bristle,
            a: (48.0, 52.0 + (bristle % 8) as f32),
            b: (80.0, 60.0 + (bristle % 8) as f32),
            rb: 4.0,
            reach: 0.9,
        }).collect();
        (canvas, held, contacts)
    }

    fn totals(c: &Canvas, h: &Held) -> (f64, f64, [f64; LAT], [f64; 5]) {
        let area = 1.0 / (c.f.scale as f64).powi(2);
        let mut total = (0.0, 0.0, [0.0; LAT], [0.0; 5]);
        for i in 0..c.wet.vol.len() {
            let volume = c.wet.vol[i] as f64 * area;
            total.0 += volume;
            total.1 += c.wet.solv[i] as f64 / COAT_UM as f64 * area;
            for k in 0..LAT { total.2[k] += volume * c.wet.lat[i][k] as f64; }
            for k in 0..5 { total.3[k] += volume * c.wet.hide[i][k] as f64; }
        }
        for b in &h.bristles {
            total.0 += b.vol as f64;
            total.1 += b.solvent as f64;
            for k in 0..LAT { total.2[k] += b.vol as f64 * b.lat[k] as f64; }
            for k in 0..5 { total.3[k] += b.vol as f64 * b.hide[k] as f64; }
        }
        total
    }

    fn run(c: &mut Canvas, h: &mut Held, contacts: &[Contact], clip: Option<Clip<'_>>, id: u32) {
        let sf = c.surf();
        let tool = h.tool.clone();
        let full = h.full();
        assert!(unsafe { stamp(sf, &mut h.bristles, &tool, contacts, full, clip, id, &mut None, (0, 0, sf.w, sf.h)) });
    }

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() <= 2e-6 * a.abs().max(1.0), "{a} != {b}");
    }

    #[test]
    fn overlapping_contacts_conserve_paint_solvent_and_mixture() {
        for thinner in [0.0, 0.8] {
            for volume in [0.00001, 0.1, 2.0] {
                let (mut c, mut h, contacts) = fixture(thinner, volume);
                let before = totals(&c, &h);
                for _ in 0..8 { run(&mut c, &mut h, &contacts, None, 1); }
                let after = totals(&c, &h);
                close(before.0, after.0);
                close(before.1, after.1);
                for k in 0..LAT { close(before.2[k], after.2[k]); }
                for k in 0..5 { close(before.3[k], after.3[k]); }
                assert!(c.wet.vol.iter().chain(&c.wet.solv).all(|v| v.is_finite() && *v >= 0.0));
                assert!(h.bristles.iter().all(|b| b.vol.is_finite() && b.vol >= 0.0 && b.solvent.is_finite() && b.solvent >= 0.0));
            }
        }
    }

    #[test]
    fn overlapping_thinned_deposits_share_the_stroke_ceiling() {
        let (mut c, mut h, contacts) = fixture(0.8, 2.0);
        let mut cap = 0.0f32;
        for _ in 0..12 {
            cap = h.bristles.iter().map(|b| crate::thinner::stroke_limit_um(b.solvent / (b.vol + b.solvent)) / COAT_UM).fold(cap, f32::max);
            run(&mut c, &mut h, &contacts, None, 1);
            assert!(c.wet.laid.iter().all(|&v| v <= cap + 1e-6));
        }
        assert!(c.wet.laid.iter().any(|&v| v > 0.0));
    }

    #[test]
    fn worker_count_does_not_change_frozen_batch_results() {
        let run_with = |workers| {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();
            let (mut c, mut h, contacts) = fixture(0.6, 0.2);
            pool.install(|| run(&mut c, &mut h, &contacts, None, 1));
            (c.wet.vol, c.wet.lat, c.wet.hide, c.wet.solv, format!("{h:?}"))
        };
        assert_eq!(run_with(1), run_with(4));
    }

    #[test]
    fn zero_mask_blocks_pickup_deposit_and_ploughing() {
        let (mut c, mut h, contacts) = fixture(0.6, 0.2);
        let mask = Mask::empty(c.f);
        let before = totals(&c, &h);
        run(&mut c, &mut h, &contacts, Some(Clip::Mask(&mask)), 1);
        assert_eq!(before, totals(&c, &h));
    }

    #[test]
    fn tiny_contacts_stay_serial_without_mutation() {
        let (mut c, mut h, _) = fixture(0.0, 0.2);
        let contacts: Vec<_> = (0..h.bristles.len()).map(|bristle| Contact { bristle, a: (64.0, 64.0), b: (65.0, 64.0), rb: 0.55, reach: 0.9 }).collect();
        let before = totals(&c, &h);
        let sf = c.surf();
        let full = h.full();
        let tool = h.tool.clone();
        assert!(!unsafe { stamp(sf, &mut h.bristles, &tool, &contacts, full, None, 1, &mut None, (0, 0, sf.w, sf.h)) });
        assert_eq!(before, totals(&c, &h));
    }
}
