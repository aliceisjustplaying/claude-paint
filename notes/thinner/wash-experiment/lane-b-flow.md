# Lane B: thin-film flow candidate (unapproved)

Engine-3 plan, HANDOVER-3.md row B; AGENT_BRIEF_V2 §4a-b; REVIEW_RESPONSE §3-4.
Base `d54b423`. Diagnostics `73b8129` (no behavior change). Candidate: the commit
that adds this note on branch `e3/b-flow`. **Default engine-3 flow behavior changes.
Visual approval is still required.** Engines 1/2 don't use this flow
(`Canvas::wait` runs `wait_on_grid` only for engine ≥ 3 with solvent present).

All runs: release profile, macOS 27.0.1 arm64, Apple M3 Pro, rustc 1.97.1,
through `~/src/a/claude-paint-tools/lockrun`. "Old" = `73b8129` (equal to the base
flow + diagnostics). Prior recorded figures are labeled as such.

## Hypothesis

The hard donor floor (`avail = max(liquid − WET_FILM_UM, 0)`, 2 µm) gives any
total-liquid film ≤ 2 µm zero outflow at any mobility. A bounded thickness-sensitive
slowing, mobility × h³/(h³ + h½³) with h½ = 2 µm, lets thin films level slowly without
speeding up thick ones. With mobility recomputed each substep, it also makes
the mixed-field throttling disappear: below-floor pixels with high solvent no longer
set the shared step. Partial ticks of flow make short waits follow elapsed time.

## Choice: a reviewed variant of `b72c3978`'s h³ law

`b72c3978` tried the same law and `f69b7967` reverted it 20 minutes later. Neither
commit, HANDOVER-2 nor the archived sheet records a defect. HANDOVER-2 records the
revert as pending a reviewed replacement. Its sheet showed a mean difference of 0.35
grey levels. I kept the law for these reasons:

- It is continuous and has no threshold. The thin end follows Orchard's h³ (module
  notes). Neither the saturation nor h½ comes from Orchard. Both are labeled
  **ESTIMATES**: h½ is set equal to the old floor (2 µm), and no measurement supports it.
- It is bounded ≤ 1, so no film flows faster than before. At 10 µm the factor is
  0.992; the existing `SPREAD_MM2_MIN` estimate was matched at that thickness.
- Material balance and nonnegativity come from the unchanged flux form and
  `MAX_OUT` (≤ ½ of a pixel's liquid per substep). The donor floor had no role in them.
- An explicit ground-retention compartment would add a second state variable and
  another unmeasured amount. The brief allows either; this one needs one parameter.

What this candidate adds beyond `b72c3978` (that commit had none of it):

1. `spread` recomputes the largest mobility before every substep and splits the
   remaining time at the new bound. It fuses this into pass 2 and keeps the old even
   split when mobility doesn't change. If the 64-substep cap would cut the remaining
   time short, it takes maximal stable substeps and reports the lost time
   (`FlowStep::scheduled_min`).
2. `wait_on_grid` (`drying.rs`) flows each grid segment over its own duration,
   including partial ticks. Before, flow ran only as a whole tick ended. Grid-aligned
   splits stay bit-identical because the segments are the same.
3. `FlowStep` diagnostics (m_max, donor-eligible m, peak m, requested/used
   substeps, scheduled minutes). Ignored probes are in `crates/paint/src/thinner/flow_probes.rs`.

`WET_FILM_UM` is removed. Its value lives on as `THIN_FILM_UM` (h½) with a new meaning.

## Results

`cargo test --release -p paint --lib thinner::flow_probes -- --ignored --nocapture --test-threads 1`

### Direct films (smooth zero-relief ground, 0.1833 mm pixels, 80×80 px square, no evaporation, flow only, 1 min)

Paint share that left the square (%). Paint/solvent balance was within 6e-7
relative in every row of both versions. Nonnegativity was asserted.

| liquid µm | φ .5 old | φ .5 cand | φ .75 old | φ .75 cand | φ .9 old | φ .9 cand |
|---:|---:|---:|---:|---:|---:|---:|
| 0.25 | 0 | 0.017 | 0 | 0.051 | 0 | 0.145 |
| 0.5 | 0 | 0.129 | 0 | 0.345 | 0 | 0.800 |
| 1 | 0 | 0.696 | 0 | 1.397 | 0 | 2.372 |
| 2 | 0 | 1.886 | 0 | 3.099 | 0 | 5.025 |
| 3 | 2.917 | 2.539 | 4.024 | 4.219 | 6.478 | 6.926 |
| 10 | 3.531 | 3.458 | 6.161 | 6.016 | 10.465 | 10.223 |
| 69 | 3.600 | 3.598 | 6.351 | 6.345 | 10.932 | 10.921 |

Old: zero outflow for every direct total-liquid film ≤ 2 µm, at every solvent share.
For 3 µm films at φ .75/.9 the candidate moves more paint than the old code, even
though their mobility is lower (0.77). The old floor also blocked the first 2 µm of
outflow there. The 69 µm stress case is unchanged within 0.2% relative.
`wait(5)` is similar; see the probe output.

### Brush strokes (hog flat 40, load .9, pressure .85, primed ground, 2400 px / 440 mm, then `wait(5)`)

Film figures are the same in both versions (deposition is unchanged).

| thinner, passes | paint p50/p90 µm | liquid p50/p90 µm | share liquid ≤ 2 µm | 5 min moved, old | candidate |
|---|---|---|---:|---:|---:|
| .50 × 1 | 1.70 / 2.52 | 3.40 / 5.04 | .16 | 9.98% | 9.48% |
| .50 × 3 | 3.51 / 5.14 | 7.02 / 10.28 | .04 | 9.63% | 9.42% |
| .75 × 1 | 0.30 / 0.43 | 1.19 / 1.70 | 1.00 | **0** | 5.64% |
| .75 × 3 | 0.62 / 0.89 | 2.47 / 3.54 | .30 | 8.01% | 9.25% |
| .90 × 1 | 0.04 / 0.06 | 0.41 / 0.57 | 1.00 | **0** | 1.52% |
| .90 × 3 | 0.09 / 0.12 | 0.85 / 1.20 | 1.00 | **0** | 5.13% |

The three repeated passes reload the brush each time (`Held::new` per pass). These
strokes come from the base brush. Lane A's exchange will change them, so these
checks need rerunning after integration.

### Elapsed time (three thinner-.5 strokes, 2400 px, after `wait(0.25)`; the clock is set directly to minute 3 + phase, with identical incoming state)

| | old | candidate |
|---|---|---|
| `wait(.02)` from .10 / .99 | 0.14630% / 0.14625% | 0.17517% / 0.17516% |
| `wait(.005)` from .10 / .99 (no tick end inside) | **0 / 0** | 0.04448% / 0.04448% |
| `wait(.02)` from 32 phases across one tick | 0.146–0.288% (×1.98) | 0.17516–0.17556% (×1.002) |
| .02 vs .01 + .01, max \|Δ\| paint | 0 | 5.3e-5 µm |
| .02 vs 20 × .001, max \|Δ\| paint | 1.1e-6 µm | 7.5e-4 µm |
| save/reload after .01, then .01, vs the same split without reload | bit-identical | bit-identical |

The old .10/.99 pair agrees only because both .02-minute windows contain exactly one
tick end. The phase sweep shows the old one-tick quantization. Off-grid splits now
differ more because every partial segment runs at least one substep. 7.5e-4 µm is
about 1e-4 of the film. Grid-aligned splits remain exact (check 17's exact cases pass).

### Mixed field (REVIEW_RESPONSE §4; active φ .5, 20 µm ± 50% 2 mm ripple; same dirty region)

| case | call | m_max / donor m / peak m | requested / used | scheduled min | active moved | max \|Δ active\| vs alone |
|---|---|---|---|---:|---:|---:|
| + separate thin φ .95 (0.667 µm), old | `spread(1)` | 1.14 / 0.06 / – | 170 / 64 | **0.3774** | 3.99% (alone 8.00%) | 2.75 µm |
| same, candidate | `spread(1)` | 0.06 / 0.06 / 0.06 | 9 / 9 | 1.0000 | 7.99% (= alone) | 0 |
| + adjacent thin φ .95, candidate | `spread(1)` | 0.06 / 0.06 / 0.0721 | 9 / **11** | 1.0000 | 7.95% | 0.072 µm |
| + separate 10 µm φ .95 (a real donor), candidate | `spread(1)` | 1.131 / 1.131 / 1.131 | 169 / 64 | 0.3804 | 4.00% | 2.73 µm |
| + separate thin φ .95, old | 64 × `spread(1/64)` | 1.14 / 0.06 | 3 / 192 | 1.0000 | 7.82% (alone 7.84%) | 0.016 µm |
| + separate thin φ .95, candidate | 64 × `spread(1/64)` | 0.06 / 0.06 / 0.06 | 1 / 64 | 1.0000 | 7.83% (= alone) | 0 |
| + separate 10 µm φ .95, candidate | 64 × `spread(1/64)` | 1.131 / 1.131 / 1.131 | 3 / 192 | 1.0000 | 7.81% | 0.016 µm |

- The old code reproduces the review's direct `spread(1)` case exactly (170 → 64,
  0.3774 min). The shared-timestep issue is real for a single 1-minute call.
- `wait` calls `spread` with at most 1/64 minute, so the cap did not bind for these
  2400 px cases in either version. The old in-wait effect was 0.016 µm, from three
  times as many substeps.
- In the candidate, the thin high-solvent patch has mobility 1.14 × 0.0357 = 0.041,
  below the active patch. When the adjacent thin patch receives liquid, the
  recomputed bound rises (peak 0.0721) and the substeps rise (9 → 11). The bound tracks
  the changing state; inactive pixels aren't merely excluded.
- A genuinely thick, high-solvent donor still sets the shared step, which is correct.
  The cap still truncates a single `spread(1)`. On the 1/64 grid, the cap binds only
  above 819·dx² mm²/min: 27.5 at 2400 px / 440 mm, 1.1 at 12000 px, against a maximum
  mobility of 1.14. This is a limitation at very fine resolution, not a fix.

### Conservation

Every probe row conserves paint (and solvent in flow-only runs) to within 6e-7
relative (f32 sums). Brush scenes over `wait(5)`: ≤ 6e-7. Check 4 and check 16 pass (below).

### Cost (`thinner::tests::wait_cost`, 12 thinner-.5 strokes, 2400 px)

| | wait(30) | wait(240) |
|---|---:|---:|
| prior record (HANDOVER-3, `f6411630`) | 21.8 s | – |
| old, this machine today (2 runs) | 23.21 / 23.54 s | 31.40 / 31.46 s |
| candidate (2 builds) | 31.42 / 31.65 s | 40.99 / 42.48 s |

`thinner::flow_probes::cost_split` reproduces the same 30 minutes tick by tick. Both
versions use **1920 substeps**, so the substep count is unchanged. Flow time is 18.4 s
old against 27.0 s candidate, about +47% per substep. About 460k of 1.3M solvent pixels
hold ≤ 2 µm liquid. They now actually move and mix (`mix_into` plus write-back), where
the floor used to zero them. The recomputed bound costs little: a separate pass
measured 31.42 s, the fused version 31.65 s. No optimization was attempted (out of scope).

## Images

`thinner::tests::flow_pictures` (raw umber, thinner .5/.75/.9 strokes and a broad .5
wash, 5 min wait), built once from each version. Sheets: `lane_b_sheets.py` (`uv run`).

- `lane-b-flow.png`: old, candidate and |difference| × 8
- `lane-b-flow-close.png`: the thinner-.75 stroke, magnified 2.5×

Mean |Δ| 0.405 grey levels, max 17. 8.9% of pixels differ by ≥ 2 in some channel.
By band: .5 stroke 0.63 (max 15), .75 stroke 0.43 (max 4), .9 stroke 0.016 (max 1),
broad wash 0.46 (max 17). The largest differences lie along stroke edges and ridges.
`flow.jpg` and `flow-close.jpg` in this directory are the archived `b72c3978` images.

## `scripts/test_thinner_acceptance --quick` (candidate, before the probe-only edit to `mixed_field`)

The runner reported 9 of 22 because one paint test failed. Cargo exited 101, so the
runner marks every paint test in that binary NOT PASSED. Each one printed `ok` except:

- **c17 FAILED (caused by the candidate):** "from 0.4 min: 15 min and 15 × 1 min:
  cure differs at 6 pixels". Every exact case and the 7.3 + 7.7 case pass. The
  6 pixels started bare and hold 3.2–4.2e-4 µm of paint (1.3–1.7× the bare threshold
  `ZERO_UM` = 2.5e-4 µm). Paint and solvent agree within 1e-4. Cure differs by up to
  7.3e-5 (≤ 2.1%). Cause: `age` runs at whole minutes and wait ends, and it ages every
  pixel with ≥ 1e-5 coats over the whole step. A pixel the flow wets partway through a
  step is aged as if it had been wet the whole step. The 0.4 and 15 × 1 layouts put
  those steps in different places. That is about 0.5 min of aging at about 1.3e-4/min,
  which matches the difference. The old floor almost never created such pixels after
  the first tick. The slowed thin edge keeps wetting a sparse fringe throughout the wait.
  This is an existing drying-step attribution, now exposed. It is not a flow-timing error.
- check 13 (b): expected failure (unchanged).
- check 2: the `rag` scene differs (5 of 8 chunks), the same in its `thinner=0`
  variant. This matches the known pending rag rebaseline. The other five scenes
  match af49348's state and PNG. Not separately rerun on the base.

`cargo test --release -p paint --lib`: 184 passed, 0 failed (22 ignored).
The easel thinner tests in the same run: 8 passed.

## Limitations

- h½ = 2 µm and the saturating form are chosen estimates, not calibration.
- The fringe: a stroke edge now creeps a sub-nanometer fringe outward (the c17 pixels).
  At these thicknesses it is invisible.
- The cap still truncates single long `spread` calls and very fine grids (above).
- Brush checks use the base brush. Rerun them after Lane A integrates.
- Cost rises by about 35% per thinned wait.
- No physical reference was used.

## Needs the owner's decision

1. Visual approval of the candidate (images above) as the engine-3 default.
2. c17. Recommendation: fix it in the engine, not in the test. Give each pixel the
   time it was wetted, or backdate the cure inherited by flow, so `age` doesn't
   over-age pixels wetted mid-step. This changes `Px` and the checkpoint format, which
   goes beyond this lane's patch. The substitute is a protected-test change that
   exempts the cure of near-bare films, but any thickness cutoff for that is a new
   unmeasured constant. Neither is implemented.
