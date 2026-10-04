# Brush exchange, round 2 (Lane A, October 4, 2026)

Branch `e3/a-exchange`, base `d54b423`. Default behavior unchanged: the
exchange stays off (`thinner::set_exchange`), and the default-build
fingerprint (`thinner::tests::fingerprint`) is bit-identical to the base.
Release profile, macOS arm64, rustc 1.97.1. All numbers are new runs except
where marked "round 1". Every parameter below (k, j, the pickup rule) is a
chosen estimate, not a calibrated constant.

Sheets (crop x 200-800, y 200-500 units; 1200 px canvases, raw umber
thinned 0.5, hog flat 40, pressure .85, load .9):

- `exchange-r2-linen.png`: scenes × settings on linen
- `exchange-r2-smooth.png`: the same on the smooth ground
- `exchange-r2-actions.png`: the motion/lift/reload actions over one band
  (crop y 320-380)

Settings: `today` (committed per-stroke ceiling), `(b)`/`(B)` controls,
`kKjJ` (round-1 exchange: release × (1−t)^k, load × (1−t)^j, no
per-stroke pickup bound), `fkKjJ` (the same, keeping the pickup rule
unthinned paint has: `thinner::set_exchange_floor`).

## The controls (b) and (B)

Commits `bd5addc`/`abb5f75` are no longer in any local repository. They
are rebuilt test-only in `bristle::probe::Control` from their description
(`notes/look/REPORT.md` §2, quoted in
`reviews/engine3-independent-review/EVIDENCE.md` E08): on a pixel's first
contact (or ploughed arrival) in a stroke, record its wet film (paint +
solvent) as `base`; the stroke may then lay or plough liquid only up to
`base + ceiling` (b) or `max(base, ceiling)` (B). Reconstructions, not the
original code. They reproduce the recorded qualitative behavior: one
stroke a flat 3.0 µm (recorded 3.00/2.99 edges, 2.97 inside); (B) does not
stack wet passes; (b) stacks.

## Measurement fixes (HANDOVER-3 "Lane A: measurement limitations")

(a) **Paint above the ground.** `exchange_probes` measures
`wet.vol + film − film_after_priming`, so coats moved into the dry film by
drying count. Also printed: the open-film share, solvent, brush inventory
(share of everything loaded) and the material balance (canvas + brushes +
wiped − loaded). Paint balance ≤ 4.8e-6 in every scene (≤ 2.5e-7 where nothing dried); solvent balance is
≤ 2.4e-7 except where `dry()` ran (there it is the evaporated solvent,
which the ledger does not track).

(b) **Ground response with each setting on** (`exchange_response`). One
stroke, net paint laid (µm·px), linen / smooth:

| setting | p .85 load .9 | p .5 load .9 | p .5 load .3 | p .3 load .9 | p .3 load .3 |
|---|---|---|---|---|---|
| today | 47105 / 47103 | 34135 / 34051 | 25450 / 24221 | 27782 / 27712 | 27259 / 31249 |
| (b), (B) | 87162 / 87166 | 69749 / 69765 | 64785 / 63677 | 60381 / 60413 | 48029 / 46062 |
| k4j0 | 73501 / 73516 | 56085 / 56143 | 11529 / 11200 | 46301 / 46365 | 6930 / 4913 |
| fk4j1 | 60316 / 59887 | 48584 / 48616 | 14290 / 14283 | 27553 / 21685 | 5187 / 3183 |

At pressure ≥ .5 the grounds agree within 6% for every setting: the
hairs' reach saturates contact. At pressure .3 they part, and the sign
depends on the setting (fk4j1 load .3: linen 63% more; today load .3:
smooth 15% more). The mechanism is the contact rule (`base`, relief
relative to the surroundings: the smooth ground sits at 0.5, the linen's
peaks reach higher), not the exchange. After a 5-minute wait the grounds
differ in every scene: linen's spread rises (flow into the weave), the
smooth ground's falls.

(c) **Path consistency** (`exchange_paths`, linen). Reference: the exact
line 250→750 in one call with 2 points. A test-only continuation
(`bristle::probe::Cont`) lets a second call continue the same motion with
no lift. Criterion (chosen here, justified by the measured floor): the
same motion partitioned without a lift, and the same line with more
collinear points, must match the reference within the chaotic floor
measured by moving the line 0.001 units (99th-percentile |Δ| and band
mean no larger than the nudge's; canvas total within 0.01%). Largest
pixel differences are not used. A sub-pixel nudge alone moves pixels by
0.3-1.8 µm.

| | today | k4j0 | fk4j1 | fk2j1 |
|---|---|---|---|---|
| nudge 0.001: p99 / band mean | 0.009 / 0.001 | 0.032 / 0.003 | 0.041 / 0.003 | 0.130 / 0.010 |
| 51 points: p99 / band mean | ≤ 0.045 max, 0.000* | 0.001 / 0.000 | 0.001 / 0.000 | 0.004 / 0.001 |
| continued, same id: p99, total | 0.000, 0.00% | 0.000, 0.00% | 0.000, 0.00% | 0.001, 0.00% |
| continued, new id: p99, total | 1.884, +2.05% | 0.000, 0.00% | 0.043, −0.05% | 0.146, −0.04% |
| hairs set down again: p99, total | 0.767, +0.48% | 1.296, −1.20% | 1.202, −1.32% | 3.871, −1.07% |
| step grid restarted: p99, total | 0.233, −0.02% | 0.092, +0.08% | 0.124, +0.16% | 0.382, +0.13% |
| real lift (2 calls): p99, total | 1.414, +0.99% | 2.320, −1.15% | 2.088, −1.09% | 6.107, −0.88% |

(*today's 51-point row from the first run of this probe, before the p99
column: largest 0.045 µm, band mean 0.000.) Every setting passes the
criterion. The 2-call difference is the sum of what a call boundary does:
a new stroke id, the hairs set down afresh (bend and last contact
cleared), a new step grid and a new wander phase downstream. Today's new
id alone adds 2.05% paint (a fresh per-stroke ceiling and pickup floor);
under the exchange it adds nothing (k4j0) or 0.05% (fk4j1, the pickup
floor). Round 1's 36 µm near the cut and 23 µm downstream (k0j0) are the
set-down (37 µm) and the wander restart. A real lift through the public
API is a distinct action and is not held to equality.

(d) **Coupled load effects** (load .3 net ÷ load .9 net, linen, p .85;
holding the load that the hunger and/or the contact read at fill .5):

| | coupled | hunger fixed | contact fixed | both fixed |
|---|---|---|---|---|
| today | 0.740 | 0.864 | 0.755 | 0.879 |
| (b), (B) | 0.957 | 0.957 | 0.992 | 0.992 |
| k4j0 | 0.213 | 0.321 | 0.222 | 0.334 |
| fk4j1 | 0.305 | 0.329 | 0.309 | 0.333 |

With both fixed, the ratio is the supply's (1/3 of the load lays 1/3 the
paint under the exchange; under today's ceiling 0.88). A lower load raises
`hunger` (0.40 → 0.75 today, 0.64 → 0.87 fk4j1), lifting more: with the
contact fixed, freeing the hunger moves the ratio −34% (k4j0), −7%
(fk4j1), −14% (today). The contact (`wet`/`wick` reach) moves it 1-4%.
p .5 gives the same pattern.

## Scenes (linen; smooth within 0.07 µm before the wait for every setting)

Paint above the ground in the middle (µm), relative spread, brush paint
left; "+5" after a matched 5-minute wait from the scene's end:

| setting | 1 stroke | broad (8) | 3 wet | 3 dried | deplete: 1st → 6th stroke, brush left | +5 min, broad |
|---|---|---|---|---|---|---|
| today | 1.60 (.33), 96% | 2.51 (.36) | 4.86 | 7.66 | 1.70 → 1.34, 75% | 2.50 |
| (b) | 2.99 (.01), 92% | 5.29 (.23) | 16.16 | 16.00 | 2.99 → 2.98, 49% | 5.27 |
| (B) | 2.99 (.01), 92% | 3.00 (.00) | 3.00 | 8.98 | 2.99 → 2.98, 50% | 2.99 |
| k0j0 | 23.92 (.47), 38% | 31.52 | 50.02 | 94.51 | 22.73 → 0.43, 1% | 31.48 |
| k4j0 | 2.74 (.42), 93% | 3.11 (.47) | 3.41 | 9.31 | 2.65 → 1.69, 66% | 3.10 |
| k2j1 | 3.67 (.39), 81% | 4.09 (.45) | 4.33 | 12.26 | 3.58 → 1.26, 32% | 4.07 |
| k0j2 | 4.95 (.39), 51% | 5.85 | 6.59 | 17.56 | 4.81 → 0.16, 2% | 5.83 |
| k4j1 | 1.00 (.39), 95% | 1.08 | 1.10 | 3.24 | 0.98 → 0.73, 74% | 1.08 |
| fk4j0 | 5.26 (.45), 87% | 7.23 | 15.52 | 21.66 | 4.93 → 2.39, 46% | 7.20 |
| **fk4j1** | **2.42 (.45), 89%** | **3.22 (.49)** | **6.08** | **9.64** | **2.28 → 1.22, 50%** | **3.21** |
| fk2j1 | 7.92 (.46), 62% | 10.76 | 21.83 | 32.25 | 7.45 → 0.76, 7% | 10.73 |

(k2j0, k0j1, fk0j0, fk2j0, fk0j1, fk0j2 ran too: all heavier than 6.7 µm
for one stroke; numbers in the probe output, not repeated.)

Actions over one band (out 250→750 and back), paint µm:

| setting | one call | cut, no lift | lift, no reload | fresh reload on wet | reload after drying |
|---|---|---|---|---|---|
| today | 1.38 | 1.38 | **2.72** | 2.87 | 3.48 |
| (b) | 3.00 | 3.00 | **5.99** | 5.99 | 5.97 |
| (B) | 3.00 | 3.00 | 3.00 | **3.00** | 5.97 |
| k4j0 | 3.02 | 3.02 | 3.06 | 3.89 | 5.88 |
| fk4j1 | 3.46 | 3.46 | 3.54 | 4.22 | 4.87 |

Today and (b): a lift without reload nearly doubles the film; the new
stroke id renews the allowance, not new material. (B): a fresh reload onto
wet film adds nothing. The exchange settings keep scrub ≈ cut ≈ lift and
make a reload (new material) visibly add.

## Findings

1. **Round-1 exchange (no per-stroke pickup bound) does not stack wet
   passes.** Every hair touch lifts a share of the film; a pass converges
   to a load-dependent equilibrium (k4j0: broad 3.11 → 3 wet 3.41 µm;
   k2j1 4.09 → 4.33). Wet, it behaves like (B), and switching the pickup
   rule for any thinner makes thinner 0.01 lay 47% less paint than none
   (check 8: 66003 vs 123623).
2. **Keeping the unthinned pickup rule (`fk`) restores stacking and
   continuity** (fk4j1: broad 3.22 → 3 wet 6.08; check 8 3.4% off,
   against 3% allowed) while keeping the exchange's path behavior (new id
   −0.05% paint).
3. **No setting passes the current protected thinner contract.**
   `scripts/test_thinner_acceptance --quick` with each as the default
   (temporary build, reverted; `today` control 21/22, only 13 (b)):

   | default | checks failing besides 13 (b) |
   |---|---|
   | k4j0 | 5 (start 0.19 µm < θ .75), 7 (+0% vs ≥ 20%), 8 (−47%), 9 (thicker film median), 14 (6.76 µm > 6), 15 (0 px ≥ 12 µm) |
   | k2j1 | 5 (0.33), 7 (+0%), 8 (−47%), 9, 14 (6.21), 15 (0 px), 16 (10 px < 20) |
   | fk4j0 | 5 (0.70), 6 (6.87 > 6), 7 (+10%), 9 (per-pixel law), 14 (26.4), 17 (cure at 23 px) |
   | fk4j1 | 5 (0.33), 7 (+7.9%), 8 (−3.4% vs 3%), 14 (8.84), 15 (2 px), 16 (9 px) |

   Checks 6 and 14 assert the per-stroke ceiling itself; 7 asserts "each
   stroke has its own limit"; 5, 15 and 16 assume the ceiling's
   deposition amounts. An exchange default needs an approved contract
   change for these, which this lane did not make.

## Recommendation

Candidate for visual review: **fk4j1** (exchange on, k = 4, j = 1,
unthinned pickup rule kept). Reasons: one stroke 2.42 µm and a broad pass
3.22 µm, between today (1.60/2.51) and (b) (2.99/5.29); wet passes stack
(+89% over three, today +94%) without (b)'s 16 µm; a brush runs down over
six strokes (2.28 → 1.22, half its paint left); a lift without reload is
nearly the same motion as no lift (+2%, today +97%) while a reload adds
(+22%); linen and smooth agree at deposition and part after the wait;
paint balance within 3.3e-7. Against it: relative spread .45-.49 (today
.33-.36): the broad pass shows stroke-to-stroke streaks; the strokes'
ends fray (the release ramp with j's smaller load); and the checks above.

Not prepared as a default commit: it is not reliable against the
protected contract (checks 5, 7, 8, 14, 15, 16). Turning it on is three
initial values in `crates/paint/src/thinner.rs` (`EXCHANGE` true,
`EXCH_KJ` (4.0, 1.0), `EXCH_FLOOR` true) plus the approved check changes.
The pickup rule kept in fk is still the per-stroke floor unthinned paint
uses (renewed per stroke id); whether that stays is a general path-contract
question (brief §3), not specific to thinner.

## Reproduce

```
EXCH_DIR=<dir> EXCH_GROUNDS=linen EXCH_SETS="today;b;B;4,0;2,1;f4,1" \
  cargo test --release -p paint --lib exchange_probes::exchange_scenes -- --ignored --nocapture
EXCH_GROUNDS=linen EXCH_SETS="today;4,0;f4,1;f2,1" \
  cargo test --release -p paint --lib exchange_probes::exchange_ -- --ignored --nocapture --test-threads 1 --skip exchange_scenes
uv run notes/thinner/wash-experiment/exchange_sheet.py <dir> out.png linen today,b,B,k4j0,k2j1,fk4j0,fk4j1,fk2j1 [actions]
```
