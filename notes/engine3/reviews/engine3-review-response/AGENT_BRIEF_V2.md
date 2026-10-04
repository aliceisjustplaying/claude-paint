# Engine 3 implementation brief v2
## Deliver a believable thin-film wipe, not another survey

**Authority:** This supersedes the earlier AGENT_BRIEF.md work order and its instruction to integrate thinner variant (b). It does not authorize modifying protected tests, production defaults, old-engine behavior, merge gates, or visual approval criteria without the project's normal review. No fix has been coded by this document.

Read REVIEW_RESPONSE.md and SOURCE_RECEIPTS.md. The supplied counter-review's Rust runs are reported evidence, not locally reproduced logs. The included Python probes check equations only.

## Decisions already made in this brief

**Rag:** contact/exposure, residual floor and local loading/return transfer are the first appearance investigation. Minimal canonical-input and capacity safeguards belong in this work; a general cloth solver rewrite does not.

**Thinner:** (b) is an experimental control, not the selected production fix. Do not ship (B) or a solvent-only absolute cap by default. Repair elapsed-time flow and diagnose the ceiling/floor interaction. Compare a supply-limited exchange experiment against the existing controls before selecting deposition behavior.

**Sienna:** recommend a reviewed contrast-ratio version of 13(b), preserving the old absolute-difference quantity as a diagnostic and preserving pigment constants initially. Do not change 13(a) or a protected expectation merely to obtain a pass.

**Deferred:** full self-collision; historical colored retired-face state; an implicit/general thin-film solver; a language rewrite; new pigment coefficients without new evidence; artistic noise added to hide transport defects.

## 0. Pin the experiment, not the whole universe

Identify the exact source/configuration baseline. The uploaded snapshots are alternatives and partial integrations, not a stack of patches. Preserve engine-1/2 behavior. Retain one unmodified baseline result for each scene.

Use the existing diagnostic harness where possible. Add only the runtime controls needed for named ablations. Do not first build a generic server, broad plugin framework or new configuration system. Runtime-tunable experimental coefficients are helpful; a large infrastructure rewrite is not this task.

For every result record source/configuration hashes, build profile/platform, seed, ground definition, physical size, pixel spacing, incoming rag state, stroke geometry, pressure and timing. Cache keys must identify the complete relevant input state. Crop-only runs must include upstream contamination and the influence region, not merely matching pixel density.

## 1. Establish the floor interaction with a minimal executable receipt

Construct known fresh nonvolatile films directly, initially on a named zero-retention smooth diagnostic ground. Suggested diagnostic thicknesses are **.25, .5, 1, 3 and 10 µm**; retain the prior **69 µm** field as a stress case, not the sole artistic target. These are test choices, not measured historical film thicknesses.

With a clean rag and prescribed full contact, demonstrate that the old rule cannot lift any measured ≤1 µm pixel. Verify the field, not merely a nearly white image. Use direct fixtures so a brush or lateral pile cannot quietly increase the thickness and mask the defect.

Then use the actual brush at thinner .5, 2/3, .75 and .9. Report both nonvolatile and total-liquid thickness. The nominal law predicts a ≤1 µm paint ceiling from t=2/3, but actual deposition, old wet film and inflow must be measured. Never assume the nominal ceiling is the actual pixel state.

Replace the unconditional minimum stain rule with an explicit retained-ground choice. A zero-retention diagnostic must permit removal of accessible submicron fresh paint. A retained compartment, when enabled, partitions existing paint; it must not create a guaranteed stain or silently reactivate a dry underlayer. No universal replacement constant chosen solely to pass this scene.

**Receipt:** before/after film and lifted material on the direct fixtures; brush-produced film histogram; protected-underlayer check; total material balance. This is source-grounded behavior, not yet a real-cloth appearance approval.

## 2. Isolate contact, local loading and redeposition in the same small scene

Use a controlled contact provider with separate geometric contact and pressure/transfer conductance. Keep a persistent material identity on the active face. Start with a broad contact and a narrow fold-like contact; do not immediately substitute a higher-resolution cloth mesh.

A point which is never contacted must not exchange material. A low but nonzero contact is allowed to clean over repeated exposure. Do not force a once-per-stroke survival fraction. Record accumulated exposure and ever-contacted fraction: a gap in one stamp need not remain a gap across an entire wipe.

Use a fixed canonical pose stream for comparisons. Do not let redundant input vertices silently change stamp count between ablations. Keep true gaps, applied pressure, elapsed contact time, and sliding distance conceptually distinct.

Run targeted controls, one named hypothesis at a time:

- Old baseline with contact, floor-hit, capacity-hit, pickup/deposit and loading fields.
- No-floor diagnostic to expose how much of the output was the residual bound.
- Local bounded capacity with the extra whole-face thirst multiplier disabled, versus the same local budgets with it enabled.
- Redeposition disabled as a diagnostic, then restored using prior mobile material only.
- Broad versus fold-like contact under matched path/load/ground conditions.

Use combinations only to resolve a demonstrated interaction. A frozen-clean/perfect-sink test may isolate geometry, but must be labeled non-production and must not become the advertised rag.

**Do not remove loading merely because it makes a gradient.** A finite rag should load. Determine whether the measured gradient comes from plausible local saturation/carryover or duplicated global throttling. Gross pickup/deposition are not provenance fractions.

### Minimal transfer requirements within this stage

Each exchange is limited by accessible donor material and available receiver capacity. Batch competing requests against both budgets; rejected transfers stay at their source. Never repair receiver overfill by discarding the excess after pickup. Include compound/solvent accounting as supported by the chosen baseline; do not claim conservation of pigments from an unconstrained latent color vector alone.

`held` in the existing prototype already includes mobile paint. Do not double-count it. Keep active dirty color and an aggregate accounting path for inaccessible material. Do not build a full archive of retired colored faces unless old faces can return to contact.

Keep time-dependent absorption distinct from travel. A blot has zero slip and positive contact time. Use existing action durations or a documented default; do not invent distance to make absorption run.

In the integrated solvent-bearing version, name and account for dip input, canvas exchange, retained cloth solvent, and evaporation. A dampness multiplier alone does not establish that ledger. Do not quietly promise dissolution of set paint when the chosen model excludes it.

**Receipt:** a thin-film dry wipe, damp wipe, dirty continuation across a second color and refolded continuation, plus a blot. Compare start/middle/end film, local held/mobile paint, gross pickup/return and net removal. Show full images and field diagnostics. Explain visual improvement without calling it physically measured.

## 3. Correct the general path contract, including released main

This can be a small independent patch alongside stage 2, not a prerequisite for a mechanics redesign. The counter-review reports a large main-path dependence: 31.2→23.8% film remaining dry, 6.8→3.9% damp, with denser vertices.

Use a global arclength sampler carrying residuals across vertices. Derive solver movement and canvas footprint from one hand pose, including wander. Sample pressure consistently along physical path length. Respect reversals, genuine corners, intentional lifts and reloads. Make nominal contact duration explicit.

Audit per-stamp smear/soak and partial-contact compounding, not just the `ceil` call. The canonical sampler removes input partition dependence; timestep refinement is a separate convergence test. A smaller step alone does not establish a converged material law.

**Receipt:** 2-point and densely subdivided versions of the same continuous line, plus reversal/corner; measured field deltas, capacity/conservation, and profile/platform. Protect old-engine replay rather than changing shared legacy paths silently. Do not encode the old idealized 704 substep example as the expected runtime count.

## 4. Thinner: repair time and identify an actual useful deposition model

### 4a. Elapsed time

Replace absolute one-minute flow bursts with actual elapsed-time integration consistent with evaporation/cure. Evaluate partial intervals before observation or new painting. Save/restore necessary integration state.

Test equal .02 minute waits from .10 and .99 with identical incoming film/cure/solvent and tool state. Also test split/unsplit waits and save/reload. Whole-minute replay equality alone is insufficient.

### 4b. Floor and mobility interaction

Demonstrate old zero outflow for a direct total-liquid film ≤2 µm. Compare with single brush strokes across thinner .5/.75/.9 and with genuinely thicker repeated passes. A higher solvent mobility cannot overcome a hard-zero donor budget.

Test a bounded, thickness-sensitive slowing or explicit ground-retention model rather than a discontinuous universal floor, but label its parameters as chosen estimates. Preserve nonnegativity, real elapsed time and material balance; do not replace one unmeasured magic constant with another and call it calibrated.

Before demoting the substep cap permanently, run the mixed-field test: an active moderate-solvent leveling patch with/without a separate high-solvent below-floor patch in the same dirty region. Log maximum mobility, maximum donor-eligible mobility, requested/used substeps, scheduled physical time, and actual transport. Recompute a safe bound when state changes; excluding initially inactive pixels is not automatically stable.

A new global solver is not a prerequisite unless these target fixtures establish the need. Instrument and profile first.

### 4c. Deposition: compare, do not assume (b) won

Preserve baseline, (b) and (B) as controls. Reproduce single stroke, broad overlap, actual brush load depletion, and subsequent-pass buildup on linen and a smooth ground. Report actual thickness and overlap variation; measure both early and after a matched wait.

The experiment to prefer next is finite-rate exchange between finite brush and canvas reservoirs, using local contact/pressure, solvent and cure, with conservative displacement. Neither a new command ID nor a fresh cap baseline should act as an unrecorded material source. A fresh reload is actual new material and must remain distinguishable from an uninterrupted scrub.

Do not adopt an untested soft-target formula that simply converges to the same absolute slab as (B). Establish supply and exchange semantics before picking constants. Check these distinct actions: uninterrupted motion, same motion partitioned into calls without a physical lift, real lift without reload, fresh reload onto wet film, and painting after drying.

Keep the approved buildup contract unless separately changed. Tests for cure or boundary mixing should directly construct the material state they need; retain brush-generated end-to-end coverage separately. Protected fixture edits require approval.

**Receipt:** same-candidate stroke/broad-pass comparison with film, solvent, brush inventory, appearance and test outcomes. No pigment retune to hide extra thickness. No claim that check-count success alone selects a visual result.

## 5. Sienna: make the recommended requirement change explicit

Recommend replacing 13(b)'s absolute-difference ranking with a lower **RGB luminance contrast ratio** for burnt versus raw at equal nonvolatile thickness on identical named substrates. Preserve the old diagnostic, with a name that says it measures retained absolute substrate difference.

Use predeclared, existing diagnostic thicknesses (3, 10, 25 and 30 µm) and report actual substrate reflectances/RGB values. Keep direct optical drawdowns separate from brush-painted cards. Include per-channel results as diagnostics, without claiming every channel must rank alike.

The catalog's .40/.45 are input calibration scalars derived through a grayscale surrogate; clarify this without silently changing serialized values. The current actual RGB one-coat diagnostic is approximately .452 raw/.438 burnt on black/.8-white. Do not change scalar hiding or 13(a) just to make their numerical ordering resemble the output diagnostic.

Regenerate the brush-dependent sienna card from the exact final candidate: the counter-review says the recorded 1.64 µm is now 5.36 µm. Never reuse that old card as evidence for the new brush. This requirement recommendation is not a test-change approval or proof of historically calibrated pigments.

## 6. One connected demonstration and the real release checks

First obtain a visible result from stages 1–2. Do not wait for every deferred project to run an experimental painting.

Then, on the exact integrated candidate, make a small complete sequence: brush-made underpainting → measured film → wiped lights → dirty two-color carryover → cleaner refold → thin and body overpaint → wait → save/reopen → replay. The user must be able to see the result at working size and detail size.

Separately finish the recorded approvals, gate promotion/integration, regression sensitivity, target-platform checks and meaningful later-log replay coverage. Do not declare a release from experimental images or from an unapproved test rewrite.

## Required response from the implementing agent

Give the exact base and result source/configuration IDs, the named hypothesis, changed modules, commands actually run, profile/platform, before/after numerical fields and images, actual failures, and the remaining decision. Keep reported prior measurements separate from new runs.

The next milestone is a demonstrated thin-film wipe with bounded dirty transfer. Do not answer this brief with another unbounded audit, an elaborate unrendered cloth model, a massless visual effect, or a success claim based only on tests.
