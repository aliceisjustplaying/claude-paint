# Engine 3 — response to the counter-review

**Date:** October 4, 2026  
**Status:** Corrected assessment and implementation plan. No Rust fix, engine rerender, production merge, or visual approval is claimed.

## Decision

The counter-review corroborates much of the original diagnosis but changes the work order and weakens several implementation recommendations. Prioritize the rag's contact/exposure, artificial residual floor, and local loading/redeposition behavior on a thin-film fixture. Do not make a large mechanics or retired-face-state project a prerequisite. Do not treat thinner variant **(b)** as accepted for integration. Recommend restating sienna 13(b) as a controlled RGB contrast-ratio requirement, with explicit approval and unchanged pigment constants initially.

Keep Rust. The counter-review supplies no evidence that changing languages would fix these behavioral problems. Keep the experimental loop small and make the next delivery a demonstrated operation, not another model survey.

## Evidence boundaries

**Supplied counter-review:** reports scratch-copy Rust runs on macOS, release profile, one principal rag fixture (69 µm, pressure .8, one seed), source/evidence verification, and no physical reference. Its named `rag_probe*.log`, `ship_probe.log`, `look_probe.log`, `thinner_probe*.log` and patch directory are not in the attachments available for this response. I accept the reported results as useful evidence but have not independently inspected those new raw logs or reproduced those Rust runs.

**Independently checked here:** re-read the original packet's shipping, prototype, thinner, and look source paths; examined the old look report and measurement/test code; evaluated isolated source equations in Python. `SOURCE_RECEIPTS.md` provides exact paths, line ranges and source hashes. Twelve Python tests pass. They test calculations, not the Rust engine or the physical appearance of paint. No usable cargo/rustc was found on PATH in this runtime. No outside physical evidence has been added to fill the counter-review's limits.

**New deductions:** the rag-floor/thinned-wash interaction and the mixed-mobility limiter example below are algebra/source findings, not newly rendered engine results.

## 1. Corrections to the first review

### R03/R04: real defects, wrong emphasis for the prototype

The reported prototype film differences are small in the tested fixture: 0.008–0.021 µm against a 1.38 µm mean, despite a large substep-count change. The 704 figure in the first review is an idealized arithmetic illustration, not the Rust run count. Use the newly reported 900 substeps for that two-point run; f32 rounding was explicitly excluded from my illustration. Do not build a regression expectation around 704.

Canonical sampling remains important, particularly on released main, but replacing the entire mechanics integration before contact/transfer experiments is not justified by these measurements. Freeze a canonical pose stream for controlled prototype comparisons; put the general sampler repair in a small, separately testable change.

### R08: unconfirmed robustness risk, not an established image cause

The counter-review did not reproduce material conflicts on straight, return, U or zigzag paths. That does not prove projected overlap is impossible, but it removes the justification for treating it as a current blocker. Preserve an adversarial two-patch regression idea. Defer a major overlap/self-collision implementation unless an actual target scene fails.

### R07: historical colored faces are not required by the current model

The source creates a refolded face with held load and zero mobile paint; only mobile paint enters release budgets. An inaccessible old color cannot contaminate the canvas under that contract. A spatial archive of retired colored faces therefore does not improve the current visible behavior. [S06–S07]

Keep honest aggregate material/capacity accounting. Preserve active-face dirty color. Require historical patch state only when the public model permits reopening old faces or bringing retired paint back into mobile exchange. Do not add a large face-history system to the first useful rag.

### T04: demote the demonstrated reach, retain a targeted mixed-field test

The counter-review's nine 2400-pixel-size cases show no visible consequence of removing the cap; its only engagement occurred where nothing flowed. That is stronger practical evidence than my isolated coefficient example for those scenes. Do not spend the first implementation effort on an implicit/global flow solver.

The source still has a compositional issue: the maximum mobility includes pixels that cannot donate because they are below the flow floor. Such a pixel can set the shared timestep coefficient for another, thicker region. Section 4 gives a direct source-equation test to run before considering this closed. [S03]

### Sienna's substrate-band mismatch: housekeeping, not the explanation

The counter-review gives same-card ratios of .182 raw and .166 burnt at 10 µm. Those values are already present in the packet's detailed diagnostic log. The original report's compressed table used different substrates for its two columns, but correcting that does not remove the ranking disagreement. My emphasis on that mismatch was excessive. [S10]

### Thinner (b): withdraw the integration recommendation

Variant (b) repairs one accounting interpretation of the old per-stroke ceiling. That is not enough to select it as the production appearance model. The old report already showed its broad-pass mean increasing from 2.8 to 7.7 µm, relative spread .47 to .49, and card-difference retention 84.4% to 73.5% at the cited load. The counter-review reproduces a 7.73 µm broad pass and a near-flat 3 µm single stroke. I should have made the adoption conditional, not assigned an agent to port it. [S09; supplied counter-review, T02 and pushback]

Retain (b) as an experimental control. Do not silently promote it, revert to (B), or change pigments to compensate for its extra paint.

## 2. Four additions accepted from the counter-review

### Released-main path partitioning is more consequential

Reported remaining film changes from 31.2% to 23.8% dry and 6.8% to 3.9% damp for the same line with more input vertices. Those are **7.4 and 2.9 percentage points** of the starting film, not 7.4% and 2.9% relative changes. Main's segment-local `ceil` sampler, partial-contact compounding, and per-call smear/soak operations provide source-level mechanisms. [S05]

Repair this on the relevant engine-3 path; do not accidentally change engine-1/2 replay behavior. Test the same continuous action with inserted collinear vertices separately from genuine hand lifts or reloads. Fixing main does not require delaying the prototype appearance work behind a full cloth solver replacement.

### The rejected quarter-rate result has a strong loading gradient

The counter-review reports 2.3 µm remaining near the start and 20.2 µm near the end, plus gross redeposition equivalent to 6.19 µm against a final mean of 8.62 µm. That is evidence to investigate loading and return transfer, not to add more contact noise.

Do not say that 6.19/8.62, or about 72%, of the final paint necessarily came from the cloth. Gross redeposition can cycle through later pickup. That quotient is not a provenance fraction. Nor does a contact correlation of −.29 alone establish every causal contribution. Record pickup, redeposition, net removal, and final film separately.

The source throttles pickup by both `rag.thirst()` and material-cell `Face::thirst()`. If both happen to represent the same uniform saturation u, the factor becomes `(1-u²)²`, rather than one factor `1-u²`. At u=.8 these are .1296 and .36. This is an illustrative double-throttling calculation, not the measured stroke state. [S06; `probe_results.json`]

A finite rag should become less effective as it loads. Do not demand zero loading gradient. Test whether the whole-face penalty is double-counting capacity that local budgets already model; remove it experimentally only while retaining correct receiving limits.

### Highly thinned single-stroke flow is blocked by the floor

The nominal total-liquid ceiling is `H(t)=6(1-t)/t` µm. The flow's donor availability is `max(H-2,0)`. The ceiling reaches that floor at t=.75. At higher dilution, a ceiling-limited single pass on a bare baseline cannot supply flow under this rule, even though its nominal mobility rises. [S01, S03]

This supports the reported zero-flow cases. It does not imply every painting at t≥.75 can never flow: overlapping strokes, existing wet material, changed local composition, or accumulations can produce thicker film. The fixture and measured state matter.

### The thinned sienna card is stale

The supplied counter-review reports 5.36 µm from final code where the old report says 1.64 µm: approximately 3.27 times the recorded thickness. Treat that brush-generated card as stale relative to the final branch until rerendered. [S09–S10; supplied counter-review]

Do not discard every old diagnostic indiscriminately. Uniform-film optical calculations do not depend on brush deposition, and remain independently checkable when their pigment/optical source is unchanged. Require source/configuration, fixture, and measured-thickness identifiers on replacement cards. A stale brush card must not trigger a pigment retune.

## 3. New finding: the rag floor can block the thin wash before the flow floor does

The thinner snapshot itself contains both laws; this is not merely a hypothetical merge interaction. `COAT_UM=25` and `STAIN_COATS=.04` give a **minimum 1 µm nonvolatile residual**. The rag bounds pickup by `v-floor` and applies it only when positive. With full reach the floor is 1 µm; with less reach it can be higher. Thus a pixel containing ≤1 µm nonvolatile paint cannot donate paint through this rag rule, regardless of contact strength or lift rate. [S02, S04]

With unchanged composition, the nominal fresh-stroke nonvolatile ceiling is:

`h_p(t)=(1-t)H(t)=6(1-t)²/t` µm.

Setting `h_p=1` gives `6t²-13t+6=0`; the root in the control range is **t=2/3**.

| Thinner fraction | Nominal total liquid | Nominal nonvolatile paint | Consequence under the current floors |
|---|---:|---:|---|
| .50 | 6.000 µm | 3.000 µm | Some paint can lift; liquid is above flow floor. |
| 2/3 | 3.000 µm | 1.000 µm | Paint reaches minimum rag floor; flow still eligible. |
| .75 | 2.000 µm | .500 µm | Rag cannot lift this film; no flow availability. |
| .90 | .667 µm | .0667 µm | Both floor rules block the nominal thin film. |

**Qualifications:** exact thresholds are algebraic, not guarantees of f32 equality. The paint ceiling assumes a fresh bare baseline, unchanged mixture, and no additional lateral or later-pass material. The invariant about a measured ≤1 µm pixel follows directly from the rag rule. Subsequent flow can change local thickness; cured/protected layers and dirty-cloth deposition are separate issues. No complete Rust stroke was simulated here.

**Implementation consequence:** correcting pressure/contact alone cannot make that film lift. The rag and thinner need a shared fixture using actual measured nonvolatile film, not only a 69 µm stress layer. Replace the unconditional residual floor with an explicit, bounded retained-ground model—or use a zero-retention ground as the first named diagnostic. Do not merely lower the universal floor to a new magic number.

A conservative retention compartment must partition existing paint, not create a guaranteed residual amount. A newly supplied thin film cannot be assigned more retained pigment than it contains. Ground-specific retention is an explicitly labeled model choice until a matched physical reference supports its behavior.

## 4. New targeted test for T04: an inactive pixel can throttle an active region

`spread` computes `m_max` from solvent fraction and cure across the dirty region before testing the donor's liquid against `WET_FILM_UM`. The shared substep coefficient uses this maximum. [S03]

In a frozen-coefficient direct `spread(1)` test at 440 mm/2400 pixels:

- An active phi=.5 region gives mobility .06 and needs 9 substeps; the scheduled coefficient covers one minute.
- A separate phi=.95 pixel below 2 µm has zero donor availability but mobility 1.14 for the maximum calculation. It requests 170 substeps, capped to 64; the common coefficient then covers approximately **.3774 minutes**.

The latter does not predict a particular rendered difference: geometry, evolving composition, MAX_OUT and early stopping also matter. It demonstrates why isolated high-thinner/no-flow tests do not bound mixed-scene effects. Test an active leveling patch with and without an isolated high-solvent, below-floor patch in the same dirty region. Keep local boundary conditions identical; record shared schedule and actual active-patch transport. Do not make a new solver a prerequisite before obtaining this receipt.

Simply excluding initially inactive donors from the maximum is not automatically safe: an inactive location can become active after receiving liquid. A numerical repair needs a valid bound or recomputation as the state changes.

## 5. Resolve the three open choices

### Rag order: appearance-critical contact and transfer first, with minimal safeguards inside

Choose the direction of **D first**, but not the literal sequence that postpones all capacity work until after transfer tuning. A local receiver limit is part of correct transfer, not an unrelated cleanup task. Use a fixed canonical path during diagnostic comparisons; repair released main's sampler separately and early. Defer the general mechanics overhaul, retired colored faces, and unreproduced overlap work.

The immediate target is one sequence: a known thin underpainting, a dry wipe, a damp stronger wipe, local dirty carryover, and a cleaner refold continuation. A controlled contact provider isolates material behavior first. Then compare a broad contact and a narrow fold-shaped contact. A more elaborate cloth simulation earns its place only when those simpler controlled contacts behave correctly and the missing deformation is demonstrated.

### Thinner: no winning hard-cap variant yet

Reject the choice between shipping (b) now and shipping (B) now. The counter-review supplies no accepted broad-pass result for (b), and (B) contradicts the agreed buildup behavior.

The proposed solvent-only absolute cap is not ready either. At fixed solvent-rich conditions it still suppresses repeated-pass deposition like an absolute cap. It changes behavior based on a trace-solvent boundary and treats unthinned-but-uncured paint differently from solvent-bearing paint. Solvent presence and oil cure are different state variables in this engine. These are design consequences, not physical calibration evidence.

For the next experiment, retain the budget-accounting lessons but test **finite-rate, finite-supply brush/canvas exchange**, with pressure/contact, brush load, local solvent and cure as inputs. A stroke ID should not renew an arbitrary physical allowance for a motion that never lifted; genuine reloads change the brush reservoir. Pickup/ploughing must move actual material and free actual capacity, not automatically justify a refill to a fixed slab height.

This is an experimental direction, not a demonstrated replacement. Do not insert an uncalibrated equilibrium-height formula that simply recreates (B) under another name. Compare continuous scrubbing without reload, lifted continuation, fresh reload onto wet paint, and a new pass after drying. Keep check 7's current buildup requirement until an explicitly approved contract change; separately repair cure/mixing fixtures that accidentally depend on one brush's deposition pattern.

Fix elapsed-time flow and the floor/deposition interaction before trying to rescue the broad wash with a large diffusion rate. No amount of flow tuning can move a donor whose allowed outflow is hard-zero.

### Sienna 13(b): recommend a contrast-ratio requirement

For the stated requirement that burnt is darker yet more transparent, use **lower RGB luminance contrast ratio at equal nonvolatile thickness on the same named substrates**. Keep absolute difference retained as a separate underpainting-value diagnostic, not the acceptance proxy for that historical sentence. This is a recommendation for a reviewed requirement change, not a unilateral test edit.

The current source equations reproduce the old synthetic one-coat ratios (raw about .452, burnt about .438) with Python arithmetic. At 3, 10, 25 and 30 µm on identical synthetic substrates, the two metrics still rank oppositely. Thus the bands issue is not the cause. [S08, S10; probes]

The .40/.45 catalog values are real and should not be concealed. `scatter_for` inverts a grayscale surrogate constructed from masstone luminance; the renderer then applies channel-dependent absorption to RGB. The stored scalar therefore need not equal the final RGB luminance ratio. [S08]

Keep serialized values stable initially. Clarify the calibration parameter's meaning, and expose the measured diagnostic separately where needed. Do not lower burnt's scalar just to make metadata look consistent; that changes optical behavior and check 13(a), without establishing a better physical match. Predefine the thickness/substrate comparisons and a numerical tolerance; do not select a generous margin from the current pass. A ratio test is not a complete validation of historical paint.

## 6. Revised stopping condition

Do not block every visual experiment on release infrastructure, but do not call an unapproved experimental build a released Engine 3. The packet's approvals, old-engine compatibility, ineffective regression, gate integration, and platform/replay coverage remain separate release obligations.

The next useful delivery must contain an actual before/after result on the thin-film rag sequence, along with measured film/contact/loading fields and a precise explanation of what changed. It must not merely contain a larger test count or more noise.

Then run the brush-to-rag connection on the exact combined candidate: prepare the thin wash with the brush, confirm its measured thickness, wipe lights, cross into another color, refold, overpaint, wait, save/reopen and replay. Numerical conservation and visual review remain different acceptance axes. Neither substitutes for the other.

`AGENT_BRIEF_V2.md` replaces the previous work order. The original review remains a record of the earlier assessment; this response explicitly supersedes its unconditional (b) integration instruction and its broad B/C-before-D sequencing.
