# Agent implementation brief: an accepted Engine 3 rag, not another noise patch

## Goal and authority

Work from the explicitly selected source snapshot. The uploaded packet contains competing complete states, not a merge-ready stack. Nothing in this brief authorizes modifying approved tests, merging main, promoting gate tools, or declaring an image accepted without review. Keep engine 1/2 behavior protected. Record source/configuration hashes and the exact renderer profile with every result.

**Definition of success:** a pressure/handling-responsive rag which conserves material, carries local dirty paint and solvent coherently, is stable under numerical/path partition changes, and produces approved marks in a matched underpainting scene. The current uniform-control image remains rejected. More irregularity, lower error on one scalar, or a larger cloth mesh is not success on its own.

Read REVIEW.md and EVIDENCE.md. The Python code is an independently tested algorithmic reference, not a replacement paint model.

## Work package A — Establish one experimental baseline and a fast harness

Select and identify an integration baseline containing the desired thinner and session changes. Port the required cloth functionality deliberately; do not apply all complete patches sequentially, and do not import the look branch's older rag just to obtain its bristle fix.

Move only experimental, engine-3 parameters into a versioned configuration loaded at runtime. Preserve production defaults until reviewed. Start a persistent worker driven by the existing Lua interface or a minimal JSON protocol. Reuse immutable fixture state with source/schema/config hashes. Emit raw film, solvent, contact, pressure or pressure-surrogate, cloth material, gross pickup/deposit, and timing fields—not only PNGs.

Fixtures: uniform film; one known ridge; loaded brush-prepared field; thin earth-color underpainting; colored dirty-carryover strip; protected dry underlayer. Include ground type and physical size. A crop must include needed influence halos and the incoming tool state.

**Acceptance:** configuration change requires no source rebuild; result manifest identifies the state; cold and warm cache produce equivalent declared fields; no production-default or golden changes hidden in the harness patch.

## Work package B — Fix rag trajectory and integration semantics first

Replace segment-local ceil sampling with a global arclength sampler carrying residual distance across vertices. Define the final endpoint interval and pressure interpolation consistently. Preserve deliberate lift/restart semantics as distinct from redundant polyline vertices.

Compute a single world-space hand pose including wander, rotation, pressure, and translation. Derive solver motion and raster placement from the same pose sequence. Use a mechanical accumulator or converged quasi-static solve rather than “one extra relaxation per short input segment.” Avoid a full-step solver position being displayed at a half-step location.

**Tests before and after:** the same line expressed with 2, 20, and 1,761 points; reversal; a right-angle turn; rotation; multiple paint/mechanical step sizes; equal nominal duration with different sampling. Compare fields and material balance, not just RGB. Existing source should demonstrate the partition defect before its replacement passes. Do not demand bit equality across intentionally different discretizations: publish a convergence tolerance based on the fields' units.

## Work package C — Enforce material budgets and a real time ledger

Keep the old-state release-before-new-pickup pattern. Replace aggregate thirst as the sole capacity defense with donor and receiver budgets across all contact edges. The reference `allocate()` limits both sides and leaves rejected material at its source. Port with tests, then assess whether its intentional non-redistribution bias is acceptable or needs bounded redistribution.

Store mobile and retained material without double-counting `held`, which already includes mobile in the old model. Preserve retired faces or a conserved inactive-face ledger across refolds. Use area-weighted capacities. Separate absorption time from sliding distance: a stationary blot has dt>0 and ds=0.

Integrate explicit rag solvent: input on dip/load, exchange with canvas, retained liquid, and evaporation. Preserve formulation amounts, not merely a scalar dirty percentage. Initially use proportional paint/solvent exchange unless selective filtration is explicitly modeled. Track protected dry material outside the movable pool.

**Acceptance:** no donor overdraw, no receiver overfill, no material creation/destruction on clamp/refold/rollback, per-component conservation within justified tolerance, solvent closure including evaporation, no transfer without contact, no reactivation of set paint, and a two-color carryover/refold scene. Test duplicate/permuted contact requests and traversal/thread variants. The delivered Python tests check the reference budget, not the required Rust implementation.

## Work package D — Replace gap-only contact with a calibrated contact contract

First make the transfer code accept an explicit contact sample type. Suggested information, not an API that must be copied literally:

```rust
// Proposed interface sketch; it has NOT been compiled in the supplied project.
struct RagContactSample {
    canvas_mm: [f32; 2],
    material_patch: u32,
    material_weights: [(u32, f32); 3], // e.g. triangle barycentric weights
    area_mm2: f32,
    normal_pressure: f32,           // choose and document consistent units
    slip_mm: [f32; 2],
    dt_seconds: f32,
}
```

Keep geometric coverage separate from pressure. Test the transfer layer with an explicit controlled footprint before changing the cloth solver. This test footprint is not the final rag artwork and not a claim of measured contact.

For deforming cloth, localize the grip or add a compliant finger backing; use a coherent normal load; retain material identity through overlap; and make contact respond to a documented support model. A calibrated pressure surrogate is acceptable for v1 if named honestly. XPBD may improve stiffness/time behavior, but does not supply material constants or correct folds automatically.

Use a nonnegative time/slip-dependent uptake law and bounded budgets. Preserve genuinely disconnected gaps; do not force a fixed fraction of paint to survive every call. Add conservative canvas-side shear transport only as a separately measured stage, after pickup/redeposit behavior is understandable. Do not add decorative texture to the resulting film.

**Acceptance:** chosen real reference and matching preparation documented; contact footprint responds meaningfully to hand shape/load; output distinguishes blot, broad wipe, narrow fold, dirty wipe and refold; sample/mesh/time convergence; qualitative visual review. Both broad softness and localized edges should be possible under appropriate handling. No “always ragged” target.

## Work package E — Integrate the thinner rim correction selectively

Port look experiment (b), the net-per-stroke liquid ceiling, into the chosen integration state. Capture each pixel's baseline consistently on first touch, including lateral arrival. Pickup frees room and lateral inflow consumes room. Preserve ID wrap handling. Do not reintroduce the absolute cap (B), disable all brush ploughing, or merge unrelated procedural-rag changes.

Use direct thickness fixtures for tests intended to isolate curing or boundary mixing. Keep separate brush-generated end-to-end tests. Any protected fixture/expectation change requires independent approval.

**Acceptance:** single-stroke ridge/cap diagnostic, second-pass and five-pass buildup, overpainting, unthinned baseline, solvent/paint conservation, wrap/replay tests. Review overlap darkness separately; passing the single-stroke rim test is not proof that the broad wash is solved.

## Work package F — Repair thinner's time and resolution behavior

Instrument the substep cap and effective integrated flow time. Replace silent rate clipping with an integration strategy that advances the declared physical time. Benchmark sufficient substeps, implicit/semi-implicit integration, and a fixed physical transport grid before choosing one.

Replace absolute one-minute flow bursts with actual-elapsed-time integration. Save/restore its integration state. Define what happens on a partial time interval before observation or a new paint operation; a deferred full batch is not sufficient. Keep evaporation and cure coupled on the same actual time, with a declared operator-splitting/convergence scheme.

Treat the 2 µm liquid floor, thickness-dependent evaporation constant, and deposition ceiling as named estimates. Do not adjust them merely to satisfy a test's predicted wait duration. Test remaining solvent directly.

**Acceptance:** 480/2400/4800 px at the same physical size and relevant high-solvent cases; starts at clock .10 and .99 with equal short waits; just-before/just-after minute boundaries; split/unsplit waits; save/reload; conservation; old engine protection. Publish error and runtime, not just a passing boolean.

## Work package G — Resolve the sienna requirement without metric substitution

Leave pigments unchanged initially. Report contrast ratio and absolute substrate-difference retention on the SAME named substrates, at controlled nonvolatile thicknesses; also report measured thicknesses of brush-painted cards. Preserve RGB/per-channel information.

Propose a wording change for 13(b) only after agreeing what it means: lower hiding at equal film, readability of a value pattern, or another explicit target. Keep the historical source qualitative. Do not label the synthetic diagnostic ASTM-compliant. Retain the old quantity as a diagnostic even if its acceptance role changes.

**Acceptance:** independent approval of the requirement; no threshold chosen solely to pass current output; uniform-thickness and drawn-card tests separated. Fit optical coefficients only if the agreed physical/visual target actually contradicts current output.

## Work package H — Finish one actual painting and the release path

Resolve the packet's ten protected approvals, insensitive ground-baring regression, 13(b), and gate format/promotion through the project's actual authorization process. Validate on the target platform; the packet's previous runs were macOS only. Restore selected later-log interaction coverage rather than claiming eleven tiny excerpts equal seven complete replays.

Run focused tests during development and the required full release suite on the exact combined candidate. Produce a small complete painting using underpainting, wiping, dirty carryover/refold, thinning, body paint, waiting, saving, reopening, and replay. Deliver the actual image and field evidence for review.

**Stop condition:** the specified painting and rag sequence are accepted and the exact combined release passes its agreed safety/engineering checks. Further spectral optics, full cloth self-collision, selective pigment filtration, or a general capillary solver are future scope unless a demonstrated failure in this target requires them.

## Mandatory result format for every package

Provide the exact base and resulting source/config identifiers; one stated hypothesis; changed modules; before/after fixture commands and fields; actual tests run and platform/profile; visible failure or improvement; known limits; and the next decision needed. Distinguish recorded prior results from new execution. Never report “physically accurate,” “fixed,” or “all green” merely because a number improved, a model became more complicated, or an unreviewed test was changed.
