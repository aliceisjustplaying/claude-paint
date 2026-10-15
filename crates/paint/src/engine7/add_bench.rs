//! Engine 7's `add` against engine 5's own `Surf::add` on a recorded trace of
//! real calls (and the `take`s between them): the same final film, and ns
//! per call. Run with
//! `E7_ADD_TRACE=<trace> cargo test --release -p paint --lib engine7::add::bench -- --ignored --nocapture`.
//! The trace is 128-byte records from an instrumented engine 5 replay:
//! kind (0 add, 1 take), flags (lean, cure_now, dry, absorb), call site
//! (1 deposit, 2 plough, 0 other), canvas, pixel, exchange call, then the
//! call's v, latent, properties and cure, then the pixel's volume, latent,
//! properties, cure and absorbency before the call. `<trace>.canvases`
//! lists each canvas's pixel count.

use super::{Film, Paint7};
use crate::bristle::Surf;
use crate::drying::Px;
use crate::wet::{LAT, Latent, Prop};
use std::hint::black_box;
use std::time::Instant;

struct Rec {
    kind: u8,
    site: u8,
    canvas: usize,
    i: usize,
    call: u32,
    v: f32,
    lat: Latent,
    hide: Prop,
    cure: f32,
    pre: [f32; 15],
}

#[derive(Clone)]
struct Bufs {
    vol: Vec<f32>,
    lat: Vec<Latent>,
    hide: Vec<Prop>,
    dry: Vec<Px>,
    absorb: Vec<f32>,
    lean: bool,
    cure_now: bool,
}

impl Bufs {
    fn state(&self, i: usize) -> [f32; 15] {
        let mut s = [0.0; 15];
        s[0] = self.vol[i];
        s[1..8].copy_from_slice(&self.lat[i]);
        s[8..13].copy_from_slice(&self.hide[i]);
        s[13] = self.dry.get(i).map_or(0.0, |p| p.cure);
        s[14] = self.absorb.get(i).copied().unwrap_or(0.0);
        s
    }

    fn surf(&mut self) -> Surf {
        let null = std::ptr::null_mut();
        Surf {
            w: self.vol.len(),
            h: 1,
            ox: 0,
            oy: 0,
            fw: self.vol.len(),
            fh: 1,
            scale: 1.0,
            vol: self.vol.as_mut_ptr(),
            lat: self.lat.as_mut_ptr(),
            hide: self.hide.as_mut_ptr(),
            stroke: null as *mut u32,
            touched: null as *mut u32,
            floor: null as *mut f32,
            cover: null as *mut f32,
            base: std::ptr::null(),
            dry: if self.dry.is_empty() { std::ptr::null_mut() } else { self.dry.as_mut_ptr() },
            cure_now: self.cure_now,
            clump: true,
            lean: self.lean,
            absorb: if self.absorb.is_empty() { std::ptr::null_mut() } else { self.absorb.as_mut_ptr() },
            solv: null as *mut f32,
            laid: null as *mut f32,
            laid_id: null as *mut u32,
            e7: false,
        }
    }
}

fn load(path: &str) -> (Vec<Rec>, Vec<Bufs>) {
    let raw = std::fs::read(path).expect("E7_ADD_TRACE");
    let canv: Vec<usize> = std::fs::read_to_string(format!("{path}.canvases")).unwrap().lines().map(|l| l.split(' ').nth(1).unwrap().parse().unwrap()).collect();
    let f = |b: &[u8], k: usize| f32::from_le_bytes(b[12 + 4 * k..16 + 4 * k].try_into().unwrap());
    let recs: Vec<Rec> = raw
        .chunks_exact(128)
        .map(|b| Rec {
            kind: b[0],
            site: b[2],
            canvas: b[3] as usize,
            i: u32::from_le_bytes(b[4..8].try_into().unwrap()) as usize,
            call: u32::from_le_bytes(b[8..12].try_into().unwrap()),
            v: f(b, 0),
            lat: std::array::from_fn(|k| f(b, 1 + k)),
            hide: std::array::from_fn(|k| f(b, 8 + k)),
            cure: f(b, 13),
            pre: std::array::from_fn(|k| f(b, 14 + k)),
        })
        .collect();
    let mut bufs: Vec<Option<Bufs>> = vec![None; canv.len()];
    let mut seen: Vec<Vec<bool>> = canv.iter().map(|&n| vec![false; n]).collect();
    for (r, b) in recs.iter().zip(raw.chunks_exact(128)) {
        let n = canv[r.canvas];
        let flags = b[1];
        let bf = bufs[r.canvas].get_or_insert_with(|| Bufs {
            vol: vec![0.0; n],
            lat: vec![[0.0; LAT]; n],
            hide: vec![[0.0; 5]; n],
            dry: if flags & 4 != 0 { vec![Px::FRESH; n] } else { vec![] },
            absorb: if flags & 8 != 0 { vec![0.0; n] } else { vec![] },
            lean: flags & 1 != 0,
            cure_now: flags & 2 != 0,
        });
        assert_eq!((bf.lean, bf.cure_now, !bf.dry.is_empty(), !bf.absorb.is_empty()), (flags & 1 != 0, flags & 2 != 0, flags & 4 != 0, flags & 8 != 0), "a canvas's flags changed inside the trace");
        if !seen[r.canvas][r.i] {
            seen[r.canvas][r.i] = true;
            let p = &r.pre;
            bf.vol[r.i] = p[0];
            bf.lat[r.i].copy_from_slice(&p[1..8]);
            bf.hide[r.i].copy_from_slice(&p[8..13]);
            if let Some(d) = bf.dry.get_mut(r.i) {
                d.cure = p[13];
            }
            if let Some(a) = bf.absorb.get_mut(r.i) {
                *a = p[14];
            }
        }
    }
    (recs, bufs.into_iter().map(|b| b.unwrap_or(Bufs { vol: vec![], lat: vec![], hide: vec![], dry: vec![], absorb: vec![], lean: false, cure_now: false })).collect())
}

fn run_e5(recs: &[Rec], bufs: &mut [Bufs]) {
    let surfs: Vec<Surf> = bufs.iter_mut().map(|b| b.surf()).collect();
    for r in recs {
        let sf = &surfs[r.canvas];
        unsafe {
            if r.kind == 0 {
                sf.add(r.i, r.v, &r.lat, r.hide, r.cure);
            } else {
                sf.take(r.i, r.v);
            }
        }
    }
}

/// As the engine 7 kernel calls it: one `Film` per canvas, the deposit's
/// `Paint7` made once per exchange call, the plough's once per call.
fn run_e7(recs: &[Rec], bufs: &mut [Bufs]) {
    let surfs: Vec<Surf> = bufs.iter_mut().map(|b| b.surf()).collect();
    let films: Vec<Film> = surfs.iter().map(Film::of).collect();
    let lean: Vec<bool> = surfs.iter().map(|s| s.lean).collect();
    let mut dep: Option<(u32, Paint7)> = None;
    for r in recs {
        unsafe {
            if r.kind == 0 {
                let film = &films[r.canvas];
                if r.site == 1 {
                    let p = match dep {
                        Some((c, p)) if c == r.call => p,
                        _ => {
                            let p = Paint7::new(lean[r.canvas], &r.lat, r.hide, r.cure);
                            dep = Some((r.call, p));
                            p
                        }
                    };
                    film.add(r.i, r.v, &p);
                } else {
                    film.add(r.i, r.v, &Paint7::new(lean[r.canvas], &r.lat, r.hide, r.cure));
                }
            } else {
                surfs[r.canvas].take(r.i, r.v);
            }
        }
    }
}

/// The replay without the blend: the takes, and each add's volume alone
/// (so the takes see the same film), to subtract from both.
fn run_base(recs: &[Rec], bufs: &mut [Bufs]) {
    let surfs: Vec<Surf> = bufs.iter_mut().map(|b| b.surf()).collect();
    for r in recs {
        let sf = &surfs[r.canvas];
        unsafe {
            if r.kind == 1 {
                sf.take(r.i, r.v);
            } else if r.v > 0.0 {
                *sf.vol.add(r.i) += if sf.lean && r.hide[3] > 0.0 { r.v * (1.0 - r.hide[3].clamp(0.0, 0.95)) } else { r.v };
            }
        }
    }
}

/// Pre-call states the replay reproduces exactly; misses are pixels other
/// code changed between calls (rag, knife, aging, solvent).
fn replay_matches(recs: &[Rec], bufs: &mut [Bufs], e7: bool) -> usize {
    let mut ok = 0;
    for (k, r) in recs.iter().enumerate() {
        if bufs[r.canvas].state(r.i).iter().zip(&r.pre).all(|(a, b)| a.to_bits() == b.to_bits()) {
            ok += 1;
        }
        // (resync the pixel to the recorded state, so one foreign change
        // doesn't count against every later call)
        let b = &mut bufs[r.canvas];
        let p = &r.pre;
        b.vol[r.i] = p[0];
        b.lat[r.i].copy_from_slice(&p[1..8]);
        b.hide[r.i].copy_from_slice(&p[8..13]);
        if let Some(d) = b.dry.get_mut(r.i) {
            d.cure = p[13];
        }
        if let Some(a) = b.absorb.get_mut(r.i) {
            *a = p[14];
        }
        if e7 { run_e7(&recs[k..k + 1], bufs) } else { run_e5(&recs[k..k + 1], bufs) }
    }
    ok
}

fn same(a: &[Bufs], b: &[Bufs]) -> bool {
    let bits = |x: &[f32]| x.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
    a.iter().zip(b).all(|(a, b)| {
        bits(&a.vol) == bits(&b.vol)
            && bits(a.lat.as_flattened()) == bits(b.lat.as_flattened())
            && bits(b.hide.as_flattened()) == bits(a.hide.as_flattened())
            && bits(&a.absorb) == bits(&b.absorb)
            && a.dry.iter().zip(&b.dry).all(|(p, q)| p.cure.to_bits() == q.cure.to_bits())
    })
}

#[test]
#[ignore]
fn bench() {
    let path = std::env::var("E7_ADD_TRACE").expect("E7_ADD_TRACE");
    let (recs, start) = load(&path);
    let adds = recs.iter().filter(|r| r.kind == 0).count();
    let by_site = |s| recs.iter().filter(|r| r.kind == 0 && r.site == s).count();
    println!("trace: {} records, {adds} adds (deposit {}, plough {}, other {}), {} takes", recs.len(), by_site(1), by_site(2), by_site(0), recs.len() - adds);

    let (m5, m7) = (replay_matches(&recs, &mut start.clone(), false), replay_matches(&recs, &mut start.clone(), true));
    println!("pre-call states reproduced: engine 5 {m5}/{n}, engine 7 {m7}/{n}", n = recs.len());
    let (mut e5, mut e7) = (start.clone(), start.clone());
    run_e5(&recs, &mut e5);
    run_e7(&recs, &mut e7);
    println!("engine 7 film bit-identical to engine 5: {}", same(&e5, &e7));

    let reps: usize = std::env::var("E7_ADD_REPS").ok().map_or(15, |x| x.parse().unwrap());
    let mut t = [vec![], vec![], vec![]];
    for _ in 0..reps {
        for (k, f) in [run_base as fn(&[Rec], &mut [Bufs]), run_e5, run_e7].iter().enumerate() {
            let mut b = start.clone();
            let t0 = Instant::now();
            f(black_box(&recs), black_box(&mut b));
            t[k].push(t0.elapsed().as_secs_f64());
            black_box(&b);
        }
    }
    let med = |v: &mut Vec<f64>| {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[v.len() / 2]
    };
    let (tt, t5, t7) = (med(&mut t[0]), med(&mut t[1]), med(&mut t[2]));
    let ns = |x: f64| (x - tt) * 1e9 / adds as f64;
    println!("median of {reps}: replay without the blend {:.1} ms, engine 5 {:.1} ms, engine 7 {:.1} ms", tt * 1e3, t5 * 1e3, t7 * 1e3);
    println!("ns per add (replay without the blend subtracted): engine 5 {:.2}, engine 7 {:.2}, speedup {:.3}x", ns(t5), ns(t7), ns(t5) / ns(t7));
}

/// Every branch engine 5's `add` has (lean paint, a film too thin to
/// cure, an absorbent ground, no drying state, engine 1's deferred cure)
/// ends bit for bit where `Surf::add` leaves it.
#[test]
fn same_film_as_engine_5() {
    let n = 64;
    let mut x = 0x9E37_79B9u32;
    let mut rnd = move || {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        (x >> 8) as f32 / (1u32 << 24) as f32
    };
    for flags in 0..16u8 {
        let b = Bufs {
            vol: (0..n).map(|k| if k % 7 == 0 { 0.0 } else { rnd() * 2.0 }).collect(),
            lat: (0..n).map(|_| std::array::from_fn(|_| rnd())).collect(),
            hide: (0..n).map(|_| std::array::from_fn(|_| rnd())).collect(),
            dry: if flags & 4 != 0 { (0..n).map(|_| Px { cure: rnd(), ..Px::FRESH }).collect() } else { vec![] },
            absorb: if flags & 8 != 0 { (0..n).map(|k| if k % 3 == 0 { 0.0 } else { rnd() * 0.3 }).collect() } else { vec![] },
            lean: flags & 1 != 0,
            cure_now: flags & 2 != 0,
        };
        let recs: Vec<Rec> = (0..4 * n)
            .map(|k| Rec {
                kind: if k % 5 == 4 { 1 } else { 0 },
                site: (k % 3) as u8,
                canvas: 0,
                i: (k * 7) % n,
                call: (k / 8) as u32,
                v: if k % 11 == 0 { 0.0 } else if k % 13 == 0 { 1e-7 } else { rnd() * 0.5 },
                lat: std::array::from_fn(|_| rnd()),
                hide: std::array::from_fn(|j| if j == 3 && k % 2 == 0 { 0.0 } else { rnd() }),
                cure: rnd(),
                pre: [0.0; 15],
            })
            .map(|mut r| {
                if r.kind == 1 {
                    r.v = r.v.min(0.01);
                }
                r
            })
            .collect();
        let (mut e5, mut e7) = (vec![b.clone()], vec![b.clone()]);
        run_e5(&recs, &mut e5);
        run_e7(&recs, &mut e7);
        assert!(same(&e5, &e7), "flags {flags:04b}");
    }
}
