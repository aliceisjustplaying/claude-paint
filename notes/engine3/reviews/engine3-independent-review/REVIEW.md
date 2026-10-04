# Engine 3 independent audit
## Keep the engine. Replace the rag's contact/transfer contract. Make the finish line explicit.

**Scope and confidence.** This review is based on the uploaded October 4 packet, source inspection, examination of its original images, primary reference research, and independent Python calculations. The Rust toolchain was unavailable, and dependency-download access did not work. Consequently there is no new Rust test receipt, no rerender, and no newly approved rag here. Recorded measurements below belong to the packet. The additional scalar probes isolate implementation mechanisms; they do not predict a full stroke or prove physical realism.

**Recommendation.** Do not rewrite the project in Python. Retain the Rust painting core, existing session/replay work, and explicit solvent storage. Stop treating additional noise and `LIFT` sweeps as the main rag solution. Separate contact geometry, pressure, material transfer, and rendering; fix the independently identifiable integration defects first. Adopt the thinner rim experiment selectively, not the entire older rag branch. Resolve the sienna test's meaning openly before changing pigment constants or approved expectations.

## 1. There is not one integrated candidate in this packet

The six source directories represent different states. `shipping/` is main at `4e525e50897807e9b5f734071dfeb330f1a393d3`. `prototype/` is its most developed cloth experiment with `LIFT=4`; `research/` is the diagnostic version with `LIFT=1`. `wip/thinner/` has solvent-film work. `wip/look/` includes the retained net-per-stroke film-ceiling experiment and older procedural rag changes. `wip/engine3-integration/` combines thinner with speed/gate work, not the latest cloth and look work.

The cloth/grip/material patch files are alternative complete patches against shipping, not a sequence to apply. The latest cloth is not accepted. A successful test report from one state says nothing about an unbuilt combination of the others. The correct integration unit is a named commit with an explicit feature manifest and new tests. [E01]

All six `pigment.rs` files have the same SHA-256: `9b57b84f722e72116d0fa972ecbd2e8fda9d5c420e6353ed5c6a1722c37382d5`. This confirms no difference in that module across these snapshots; it does not prove every palette or surrounding optical call is identical. [Independent `probe_results.json`]

## 2. What is established about the rag's appearance

The raised, repeated outlined islands are strongly linked to the starting brush-painted field. In the retained controls, contact maps stay the same while making the starting film uniform or disabling preparation-brush pushing removes the outlines. At quarter pickup, the center retains 12.38%, 12.44%, and 12.43% of its original film respectively. The original baseline starts at 69.29 µm. That establishes an important source-layer cause, not a validated rag. [E01, E10]

The uniform control is still visually rejected. Looking at it independently, its broad pale capsule and softly varying interior do not give enough evidence of purposeful finger pressure, fold contact, and finite dirty-cloth exchange. That is a visual assessment, not proof that soft edges themselves are wrong. A real wipe-out underpainting can intentionally be soft and general. The Palesca process begins with a thin, smoothed layer rather than conspicuous preparation-brush marks. [S1]

At the original lift rate the packet reports 1.99% film remaining but 23.96% darkness remaining. Darkness is a ground-relative luminance deficit, not paint mass or percentage of erased pixels. “It removed 98%” is not, on its own, a disqualification: the ground and film can make strong wiping appropriate. Ground absorbency changes the result; Gamblin explicitly describes wiping toward white on its relatively low-absorbency oil ground. [E01; S2]

Keep the 69 µm test as a loaded-paint stress case. Add an independently constructed thin-film fixture, a single ridge, a protected dry underlayer, and an actual prepared underpainting. The brush and rag investigations can proceed separately using these controls; do not block all rag progress on fixing every brush behavior.

## 3. Rag findings with implementation consequences

### R01 — Partial contact does not protect a persistent uncontacted fraction

`prototype/.../rag.rs:451–457` applies `frac = (1-exp(-k*local*fl*pad))*c`. The comment says this preserves creases however damp the rag is. But repeated overlapping stamps compound the removal. For frozen coefficients, total exposure E, N equal stamps, no floor, and full paint accessibility:

`remaining = [1 - c*(1-exp(-k*E/N))]^N -> exp(-k*c*E)`.

At pressure .8, damp .5, `LIFT=4`, and a clean face, k=23.6. With E=1 and c=.2, one stamp leaves 80%; 88 stamps leave **1.43986%**; the continuous limit is **0.89152%**. These are independent scalar calculations, not a simulated rag image. Capacity, cure, varying contact, the residual floor, and redeposition change a real run. Nevertheless the claimed protection mechanism is absent. [E02; `probes.py`]

**Fix:** distinguish geometric absence of contact from weak pressure or small transfer conductance. A true gap has zero contact. A weakly rubbing contact can eventually clean a site, which is physically plausible. Do not implement an arbitrary “once per stroke retain c” mask to force streak survival: that introduces call-grouping artifacts instead. Make the transfer rate a function of physical contact time/slip and check subdivision convergence.

### R02 — The main contact scalar is a gap mask, not pressure

`cloth.rs:185–190` makes contact from a smooth transition over 0.18 mm of gap above a flat plane. Nodes are clamped to that plane, and all vertices are continually attracted toward transformed noisy rest positions. The 19×19 mesh does not establish a solved finger-load pressure distribution. A 97×97 raster then interpolates contact, and the rag applies a second radial falloff. These are specific reasons to suspect over-smoothed, generic-pad behavior; they do not establish which single change will produce an accepted image. [E03, E04]

**Fix:** make the contact provider return footprint area, material identity, pressure, local slip, and elapsed time. Begin with a controlled contact footprint to test transfer separately. For the deforming version, use a localized grip or compliant finger backing, meaningful stretch/bend constraints, and a single hand pose. Pressure can initially be an explicitly calibrated surrogate normalized to total force. If derived from constraint multipliers, it must use the solver's actual units and timestep. XPBD is a candidate for reducing iteration/timestep sensitivity and obtaining force estimates, not a guarantee of realistic cloth. [S5]

### R03 — Adding redundant path vertices changes the mechanical simulation

`rag.rs:632–640` subdivides each polyline segment separately. `cloth.rs:84` independently uses `ceil(travel/.25 mm)` mechanical substeps. An ideal 176 mm line gives 352 paint stamps and 704 mechanical substeps. Represent the identical straight line as 1,760 segments of .1 mm and it gives 1,760 of each—**2.5 times the mechanical relaxation** for unchanged geometry and nominal travel. f32 boundary rounding can introduce further discrepancies; the independent probe uses intended exact lengths. [E03, E04]

**Fix:** a canonical global arclength sampler with carried residuals across polyline segments, plus an independent fixed-time/fixed-distance mechanical accumulator or converged quasistatic solve. Simply shrinking `.5` again does not solve this. A collinear-vertex-insertion test should fail on the old implementation and pass within declared numerical tolerance on the new one. Separate polyline partitioning from deliberate hand-lift/new-stroke semantics.

### R04 — Solver travel and displayed travel are different

The cloth receives `[tx*sl,ty*sl]`, but its canvas center also includes sideways wander `off`. The solver therefore omits the derivative of a displacement that moves the actual footprint; corners can magnify the mismatch. It is also advanced a full interval before contact is placed at the interval midpoint. [E03]

**Fix:** derive both simulation and rasterization from the same sequence of world-space hand poses, including offsets, rotation, and pressure. Evaluate the contact at the correct integration phase. Test straight motion, reversal, a corner, and rotation. Preserve seeded variation only as a coherent hand/cloth input, never as an unrelated image-space wobble.

### R05 — Local capacity is not enforced as a hard receiving budget

`Face::thirst` averages local saturation and `rag_contact` then sends pickup to cells according to material weights. A full cell paired equally with an empty cell still participates in a positive aggregate thirst; it receives half the pickup even though it has no capacity. Across the batch, all pixels also see the old capacity before `finish`. `held` is not capped there. Global load clamping is not a substitute for per-cell receiver limits. [E02, E05]

**Fix:** gather candidate edge transfers, limit donor outflow and receiver inflow, then apply them together. Rejected material stays at its donor; do not clamp an overfilled receiver afterward and lose material. `bounded_transfer.py` supplies a small tested reference for this operation. It can underfill because it deliberately does not redistribute rejected demand; check convergence before porting it into the final model.

### R06 — Absorption uses distance, including fictitious distance for a blot

`Face::finish` multiplies existing mobile paint by `.65^(4*travel_widths)`. A stationary blot supplies `BLOT=.8` as that distance, locking about **74.8045%** of its previously mobile paint in one operation. This is unrelated to a measured absorption time; new pickup joins after that absorption. [E05, E06]

**Fix:** distinguish time-dependent absorption from slip-dependent mechanical pickup/release. A blot has nonzero contact time and zero sliding distance. The command's existing nominal duration can provide a default, with an explicit duration control added only if needed. Absorption must advance during holds/waits consistently, not only because another canvas stamp happens.

### R07 — Cloth material identity is better, but folding is still approximate

Persistent paint in material coordinates is valuable. Old-state release budgets before new pickup are also a good design choice. In `Face`, **held already includes mobile paint**; do not count both as separate total material. Cured paint ceases to be mobile while held inventory remains, which is appropriate bookkeeping. [E02, E05]

However `refold()` discards the active spatial face and keeps only scalar soaked load. Individual retained color/material inventory is not preserved in a finite set of inactive faces. Square-width area divided uniformly over a distorted disk is also only an approximate capacity distribution. [E06]

**Fix:** preserve inactive material patches or an explicit retired-face ledger. A new face can be cleaner without magically creating an unlimited clean cloth. Use material area weights for capacities. A v1 model can aggregate inaccessible material by conserved formulation amounts rather than simulating every fiber.

### R08 — Overlapping cloth sheets can lose the correct material location

Triangle rasterization picks the greatest gap-derived touch. At equal clamped contact, triangle order can decide which sheet supplies material coordinates. Bilinear interpolation of UVs from distinct overlapping patches can then select a material location on neither actual contact patch. This is a source-level risk to verify, not a reproduced visible failure. [E04]

**Fix:** retain triangle identity and barycentric/support weights through contact sampling, or blend independent contact contributions without averaging incompatible UVs. Add a folded two-patch colored-transfer test. Whole-rag self-collision can be deferred if the v1 contact representation explicitly prevents ambiguous overlap.

### R09 — Dampness and residual stain are broad multipliers, not a material exchange model

At p=.8 and dip=.5, the current heuristic reaches roughly 152 µm below local peaks and admits 62.5% of paint below that reach into the available amount. Pickup rate is multiplied fivefold. These are estimates, not observations of historical rag handling. A universal 1–2 µm residual paint floor stands in for all ground trapping. [E02, E06]

**Fix:** initially keep simple, labeled calibrated functions, but make ground retention substrate-dependent. Separate accessible mobile film from a small retained/adsorbed ground compartment where the reference calls for it. Do not force every surface to retain the same stained film, or every damp wipe to erase almost uniformly. An optically transparent residual formulation can remain visible without adding an image-space tint.

### R10 — Latest cloth and thinner solvent handling must be reconciled explicitly

The developed cloth prototype predates thinner storage. The thinner snapshot has proportional solvent removal with lifted paint and a rag solvent accounting field, but not this cloth face. “Dip” remains a dampness control, not a full local solvent supply/exchange model. Combining the branches by resolving text conflicts does not establish a correct solvent ledger. [E01, E07]

**Fix:** declare a volume unit for cloth solvent; track dip input, solvent exchanged with the canvas, remaining cloth solvent, and evaporated solvent. For a first consistent model, carry nonvolatile formulation and solvent proportionally unless an independently specified selective-absorption model is added. Include wetness in transfer mobility, capacity, and aging coherently. No unsupported claim of dissolving cured paint is needed; v1 may explicitly leave set layers protected.

### R11 — Canvas-side smearing needs a mechanical route, not only opacity loss

The cloth path chiefly removes paint and returns stored paint. That can demonstrate color carryover but does not by itself model tangential displacement of the remaining film under a loaded sliding fold. [E02]

**Fix:** after basic transfer is correct, add bounded conservative canvas-side lateral flux driven by actual slip and traction, limited by mobility/yield behavior. Keep it separately observable from pickup and redeposition. A light wipe should not always generate a decorative rim; a loaded shove can. No global blur or predesigned edge-darkening texture should stand in for transported material.

## 4. A minimal defensible rag implementation

Keep the existing public tool commands, seeded identity, session ownership, undo/rollback behavior, and material-face concept. Make the internal boundary explicit:

```text
hand command -> canonical poses -> contact samples -> bounded material exchange -> wet state -> existing optics
```

A useful contact sample contains canvas position, material patch/triangle identity and weights, contact area (mm²), normal pressure (document units), slip (mm), and elapsed contact time (seconds). Geometric coverage and pressure must be separate. A zero-contact sample cannot remove or deposit paint.

A deliberately simple candidate uptake law is:

`requested = accessible_mobile_volume * [1 - exp(-ka(p,phi,cure)*dt - ks(p,phi,cure)*ds)]`.

Here `ka` has units 1/s and `ks` 1/mm; all rates are nonnegative. This is a **proposed calibratable surrogate**, not a measured oil/cloth constitutive law. Sample coverage contributes to accessible volume/conductance consistently. Allocate requests against donor material and receiver capacity. Deposits use prior mobile cloth paint, not freshly collected paint from another pixel of the same batch. Absorption transfers mobile cloth material into retained material over actual time. Geometry, rates, and inventory should converge as numerical subdivision changes.

Preserve at least canvas mobile formulation, protected/retained formulation as required, solvent, cure; and cloth mobile formulation, retained formulation, solvent, active/inactive face state. Track component/formulation amounts if claims require component conservation; latent color vectors themselves are not physical pigment masses. A single mixed wet layer cannot selectively recover the newest of several simultaneously wet layers. Declare that limit instead of silently promising stratigraphic fidelity.

The ledger is: initial paint + supplied paint = canvas paint + cloth paint + explicitly discarded paint, within numerical tolerance. Solvent additionally has dip/load input and an evaporation sink. Refold changes accessibility, not total material. Every transfer stage should report gross pickup, gross redeposition, lateral transport, and net removal separately.

No full Navier–Stokes simulation, spectral renderer, scanned-fiber cloth, or universal historical-material database is required to make this first version useful.

## 5. Thinner: preserve the foundation, repair the remaining model/clock issues

### T01 — Explicit solvent is the right foundation

The thinner snapshot stores solvent beside nonvolatile paint, transports their proportions, and does not turn solvent directly into an alpha value. Its gather-from-old-state flow organization is worth preserving. Recorded tests report all 27 required thinner checks passing, with the separately identified sienna 13(b) failure remaining. These are producer results, not this review's rerun. The estimates and exclusions are documented honestly in `thinner.rs`. [E07, E09]

Solvent changes handling; excessive thinning is not equivalent to harmlessly lowering image opacity. The manufacturer's guidance supports that distinction, not the numerical constants chosen here. [S7]

### T02 — The retained net-per-stroke ceiling is preferable to the gross budget

The old ceiling counts cumulative deposition even after pickup/ploughing removes material; it cannot refill the emptied center correctly. The look branch's **(b)** measures present liquid relative to the baseline when the stroke first reaches a pixel, counts lateral inflow, and permits refill after removal. The retained report changes a representative stroke from approximately 2.2 µm edges / .96 µm center to about 3 µm throughout. [E08]

Integrate (b) selectively and revalidate the combined state. It is a good correction to this engine's chosen deposition contract, not proof of a universal physical ceiling. Overlapping passes still become blotchy: the reported broad-pass mean goes 2.8 to 7.7 µm and relative spread stays about .49. The rejected absolute cap (B) makes a smoother veil by breaking repeated-layer buildup. Do not restore it.

After (b), adjust brush loading, liquid-film deposition, overlap planning, and genuine wet redistribution as separate hypotheses. Some overlap darkening can be appropriate; a brush filled with a wash is not necessarily equivalent to independent identical translucent stamps. Do not lower pigment opacity simply to hide a transport issue.

### T03 — The 6 µm law has a strong and explicitly stylized meaning

`stroke_limit_um(t)=6*(1-t)/t` limits liquid. With unchanged local composition, the nonvolatile cap is `6*(1-t)^2/t`: 3 µm at t=.5, about .0667 µm at t=.9. Dilution affects both the deposited liquid ceiling and paint fraction. That may be a useful model, but its relationship to speed, pressure, brush load, and viscosity is not calibrated. [E07; `probe_results.json`]

Document the control as a material fraction, not “percent transparency.” Separate geometry/stroke partition invariance from the legitimate effect of reloading and laying another pass.

### T04 — The flow limiter changes physical elapsed flow time with resolution

`thinner.rs:210–213` caps explicit substeps at 64 and clips the diffusion coefficient per step. When capped, `effective_dt = n*k*dx² < requested_dt`. At fixed **post-evaporation** phi=.95, fluidity 1, a 440 mm canvas, and a requested minute, independent calculations give full flow time at 480 px, .37739 min at 2400 px, and .09435 min at 4800 px. These are frozen-coefficient limiter calculations, not claims about every actual t=.95 stroke; evaporation changes local phi before flow. At phi=.5 and 2400 px the cap does not engage. [E07]

**Fix:** do not silently replace the requested physics with a slower rate. Use enough substeps, a stable implicit/semi-implicit solver, or a physically fixed transport grid with conservative coupling. Instrument cap activation now. Profile alternatives before picking the final implementation. A single high-mobility pixel currently contributes to the global maximum and can affect the whole dirty region's timestep.

### T05 — Flow advances on absolute one-minute boundaries

`drying.rs:397–410` evaporates and ages every partial interval but calls `spread(1.0)` only when an absolute minute ends. A fresh film observed for .02 min starting at clock .10 receives no flow update; the same short interval starting at .99 triggers a nominal full minute. The limiter may reduce that full minute, but the phase discontinuity remains. This follows from the schedule, independently of an actual rendering. [E09; `probes.py`]

**Fix:** advance flow over actual elapsed time and preserve deterministic integration state through save/restore. A fixed timestep accumulator must flush or evaluate partial intervals appropriately when observing or modifying the canvas; simply delaying a full batch still causes an observation discontinuity. Test time-origin shifts and waits ending just before/after a boundary, as well as split waits. Exact equality for whole-minute partitions is not enough.

### T06 — The flow is a surface-diffusion surrogate, not general capillary leveling

The current flux follows neighboring total surface height differences with an estimated mobility. Its own notes admit the estimate is matched to one wavelength. It also prevents outflow below a fixed 2 µm **liquid** film. That floor implies different nonvolatile residues at different solvent fractions and can preserve a numerical thickness imprint. [E07]

For the first release, a clearly named conservative leveling surrogate may suffice after calibration and resolution tests. Add depth-dependent mobility and substrate-dependent wetting only where evidence demands it. A true thin-film capillary model needs pressure from curvature (with appropriate wetting/gravity conditions), not merely more neighbor diffusion. Do not make a major new fluid solver a prerequisite for the first useful painting.

### T07 — One evaporation parameter was changed to satisfy a test's wait assumption

The source explicitly explains moving the thickness scale from 20 to 100 µm because redistribution increased maximum thickness enough to invalidate a test's “ten times tau” waiting interval. That is not a measured physical reason to select 100 µm. [E07]

**Fix:** evaluate residual solvent directly, integrate a conservative bound for a thickness-dependent tau, or declare a completion tolerance. Keep material constants independently justified; do not retune them to preserve a mistaken test estimate.

### T08 — Some failed tests are fixture failures, not assertions disproved

The look report's reduced-plough experiment fails a cure test because it no longer generates enough pixels above 12 µm. Another case lacks enough boundary transport pixels. Those are fragile fixture assumptions. Construct controlled thicknesses/boundaries directly for narrow physics tests, while retaining separate brush-generated end-to-end tests. Changing a protected fixture still needs explicit approval. [E08]

## 6. Sienna: distinguish three questions instead of choosing the passing number

The historical source describes burnt sienna as more transparent than raw, but supplies no modern numerical test. The project currently compares two different quantities. [S3; E11]

`C = Y_over_black / Y_over_white` is a contrast-ratio/hiding diagnostic. Lower C indicates less hiding relative to the film's own reflected brightness.

`D = (Y_after_white - Y_after_black)/(Y_before_white - Y_before_black)` measures retained **absolute** substrate luminance difference. It is useful for whether a glaze destroys a drawing's value separation.

The report gives C=.169 raw versus .154 burnt at 10 µm, while its D is 49.7% raw versus 30.6% burnt. **The two columns also use different underlying bands:** C is on synthetic black/80% white; D uses the painted card's actual bands. The source prints both ratios, but the report compresses this distinction. Rebuild the table with both metrics on the same explicitly named substrates. [E11]

There is no mathematical contradiction. For example an ideal dark absorbing, nonscattering layer over perfect black can have C=0 while reducing the over-white brightness to .001: almost all absolute value difference is lost. Thus “lower ratio” does not by itself prove a more readable underdrawing. [Independent counterexample]

Also, the public ASTM D2805 scope covers air-dry coatings with Y above 15%. The report gives burnt-sienna masstone Y≈8%, and its measurement is synthetic RGB rather than the full standard's procedure. Call the new test a contrast-ratio diagnostic; do not call it ASTM certification or evidence that a historical material has been physically matched. [S4]

**Recommendation:** leave pigment constants unchanged provisionally. Preserve D as an underpainting/readability diagnostic; add C at controlled thickness and substrate as a distinct hiding test. Agree explicitly which requirement 13(b) should enforce and record approval for its replacement. It is acceptable to discover the original test encoded the wrong interpretation; it is not acceptable to silently substitute whichever quantity passes.

The module computes a scalar scatter from masstone luminance and scalar hiding, then applies RGB optics. A scalar tube value .40 or .45 therefore does not alone rank every colored black/white result. Only fit pigment K/S or other optical parameters after controlled drawdowns show an actual discrepancy. More spectral channels would not fix a misdefined test or an incorrect film thickness. [E11]

## 7. Rust and iteration

The expensive loop is not just “Rust compiles slowly”; source recompilation, full-canvas preparation, rendering, waiting, and broad release checks have been entangled with visual parameter tuning. The project already defines `profile.iter` with incremental compilation and 16 codegen units. Its release profile disables incremental compilation and uses one codegen unit to protect the project's chosen golden workflow. Cargo documents the associated compile/runtime tradeoffs. [E12; S6]

Move experimental constants into a versioned runtime configuration. Keep a persistent Rust worker and drive it through the existing Lua scripting or a small JSON command protocol. Python can generate parameter sweeps, analyze fields, and assemble comparisons. A Python/Rust binding is optional, not the first prerequisite. This gives much of the experimental convenience without rewriting the renderer, session state, tool behavior, and replay contracts.

Use physically matched small regions at final pixel spacing, with sufficient influence halos and correct incoming cloth state. A crop that omits upstream pickup cannot reproduce a full stroke merely because its central pixels have the same resolution. Use low-resolution runs for gross behavior only, then validate numerical convergence and final appearance. Cache fixture state by source/schema/configuration hashes; the packet's original checkpoint helper is not source-version checked. [E01]

Profile likely hotspots before rewriting: repeated surface/local-maximum construction, duplicate contact and UV sampling, per-substep target/history allocation in cloth, and freshly allocated outflow/next-state buffers in leveling. Reuse storage; consider separable sliding maxima where semantics are preserved. Keep behavior and optimization changes separate so a speed patch cannot hide a physical change. These are code-inspection opportunities, not measured speedups.

Recorded integration numbers—fast checks about 63 seconds after builds, the full candidate 149 seconds building and 348 checking—are not the cost that every coefficient trial should pay. Preserve release validation at decision points, not after every knob adjustment. An exact replay is an engineering property; it does not establish realistic paint. [E13]

## 8. Remaining integration/release work

The packet's final speed/thinner report records `0abf7929c44159475e3fcb327329c7b932477aa6` as **NOT ALL GREEN**, with 13(b) unresolved. It also identifies a ground-baring test that still passes an injected defect: 7,476 bare pixels versus a 9,570 allowance. Repair its sensitivity with an approved expectation change, not a silent loosened bound. [E13]

Ten protected-file approvals remain in that snapshot: state dump, golden-path list, both test lists, test runner, candidate/merge tools, safeguards library, and their tests. The reviewed new gate is not promoted; the old gate cannot read the new format. Gate checks were on dummy repositories, not the real main. Linux was untested in those recorded runs. Seven full-painting replay tests were replaced by eleven small cases, leaving later legacy-log interactions uncovered. This review did not repair or validate any of those external workflow states. [E13]

The essential finish line is a single explicit integrated version with accepted rag behavior, conserved material/solvent and state persistence, chosen brush-ceiling semantics, agreed sienna tests, effective regression tests, and a working approved release path. Not every possible scientific refinement must ship first.

## 9. Reference/acceptance scene that actually tests painting

Use a thin earth-color underpainting on a specified ground. Make a dry broad wipe, a damp stronger wipe, a narrow fold/finger highlight, a dirty continuation crossing a second color, a refolded cleaner continuation, and a blot. Repeat over a more loaded field and a tackier field. Include a protected dry underlayer. Record before/after film, contact/pressure, mobile/retained cloth material, solvent, pickup/deposit, and the final image. State which physical quantities are controlled versus merely assumed. [Qualitative sequence: S1–S2; fixture design proposed here]

The artistic criterion is not “maximum irregularity.” It is that the marks follow the selected handling and layer: soft when blended, more localized when pressed by a fold, dirty where a contaminated face carries paint, and clean where a fresh accessible patch is used. Compare like preparation, ground, cloth shape, and motion. Online process photographs support qualitative appearance; they do not identify pickup coefficients or exact cloth pressure.

Numerical acceptance should include insertion of redundant path points; sample/time/mesh refinement; time-origin shifts; mirrored/rotated trajectories; no-contact behavior; finite local capacity; component/solvent ledger; first/second/refold differences; set-layer protection; save/reload/rollback; and declared thread/replay reproducibility. Separate these checks from visual approval.

Then make one small complete painting: underpaint, wipe lights, refine with a dirty and refolded rag, add a thin pass, add body paint, wait, save, reload, and replay. Inspect it at working size and detail size. This is the practical Engine 3 milestone; endless isolated swatches are not a substitute.

## Decision summary

Keep Rust and the working infrastructure. Make parameter experiments runtime-driven. First correct path/pose consistency and donor/receiver budgets, then replace the gap-only contact contract with pressure-aware, time/slip-driven exchange. Carry solvent into the same state model. Adopt net rather than gross per-stroke ceiling accounting without reinstating an absolute film cap. Fix thinner's clock and resolution-dependent flow limiter. Separate sienna hiding from retained value contrast. Require one integrated, visually accepted painting before calling the release finished.

The tested Python probes identify actionable mechanisms. They are deliberately not presented as a new “physically accurate” rag whose appearance has never been seen.
