# Engine 3: handover 3 (short), October 4, 2026 (night)

Current starting point on branch `rag`. Milestone: tools ready for an
Inness painting, then run it and use the result to select further tweaks.
HANDOVER-2.md supplies the broader route; the status below supersedes its
uncommitted-work and cutoff notes.

## Done this session

- `f6411630`: `FLOW_MIN` cutoff removed (owner's call); check 9's per-pixel
  bound loosened 1e-4 → 2e-4 + 2δ (owner's decision, in ACCEPTANCE.md
  "Decisions", hash updated). `--quick`: 21/22, only 13 (b) (expected) and
  check 2 `rag` scene (pending rebaseline). `wait(30)` at 2400 px: 21.8 s
  (was 9.3 s).
- PR #3 (flakypuff, impasto/knife/lit looks): reviewed, deferred until after
  the Inness painting by the owner. Doesn't touch thinner.rs or rag.rs; merges
  into `rag` without textual conflicts.

## In progress (uncommitted, default behavior unchanged)

Finite-supply exchange experiment (AGENT_BRIEF_V2 §4c), owner-approved
direction: **no fixed layer / no ceiling**. Film thickness should come from
brush supply and contact, not a target height.

- `thinner.rs`: thread-local switches `set_exchange(bool)` and
  `set_exchange_kj(k, j)`. Ignored probes: `exchange_scenes` (env
  `EXCH_DIR`, `EXCH_SETS="today;k,j;..."`, `EXCH_LINEN_ONLY`),
  `exchange_side`, `c09_probe`, `fingerprint`.
- `bristle.rs`, when exchange is on for a thinned hair: no per-stroke
  ceiling, pickup without the per-stroke floor/own-stroke discount, release
  rate × (1−t)^k, thinned load × (1−t)^j (also in the crop-window ghost path).
- Round 1 (k=j=0): far too heavy. One stroke 23.9 µm vs 1.6 today; brush
  dumps 62%. Sheet: `notes/thinner/wash-experiment/exchange-r1.png`.
- Planned comparison: **both** mechanisms (k and j). First correct the
  measurement limitations below. The grid run
  `EXCH_LINEN_ONLY=1 EXCH_SETS="today;2,0;4,0;0,1;0,2;2,1;4,1"` was started
  but aborted before output. After the measurement corrections, rerun
  and build a sheet (crop units x 200-800, y 200-500). Review the exact
  candidate before selecting defaults. Temp scripts are not durable inputs.

## Measurement limitations and next work

- **Dry-pass buildup:** `exchange_scenes` currently measures only
  `wet.vol`. It omits earlier coats transferred to `film` by drying.
  Measure total deposited paint above the original ground before using
  the wet/dry-pass table to assess buildup.
- **Ground response:** the initial full-load/high-pressure scene gives
  similar linen and smooth results. The follow-up pressure/load sweep in
  `exchange_side` runs before enabling exchange, so its differences
  describe the baseline. Repeat with exchange explicitly enabled before
  drawing conclusions about the candidate's response to the ground.
- **Path consistency:** two drag calls differ from one downstream of the
  cut (largest pixel difference: baseline 4.9 µm, exchange 36 µm).
  Restarting bristle wander is a candidate explanation, not validation of
  equivalent motion. Even within one call, 2 versus 51 points differs by
  0.045 µm in the baseline and 3.4 µm with exchange. Resolve the discrepancy
  or establish an explicit, justified acceptance criterion before tuning.
- **Coupled load effects:** the smaller-load setting does not isolate
  deposition. Lower fullness also raises pickup through `hunger` and
  changes contact with the weave through `wet`/`wick` in `bristle.rs`.
  Report these coupled effects rather than attributing the result solely
  to less paint being laid. The chosen exponents are estimates, not
  calibrated physical constants.

The immediate task is to make these comparisons reliable, then evaluate
k/j candidates. Preserve the experimental default-off boundary.

## Then

HANDOVER-2 route: thin-film floor replacement, rag correctness (§3/§2),
sienna 13 (b), smoke sequence, freeze, Inness launch. Then PR #3.
