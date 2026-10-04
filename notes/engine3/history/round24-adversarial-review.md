# Round 24 adversarial code and historical review

Two replay-integrity defects need fixing before integration is accepted. Thinner also misses its agreed transparency and evaporation targets. The historical review exposes a sienna drying-order discrepancy and a rag that never loses its solvent.

Reviewed 3 October 2026: integration `round-24` at `3d0ea66`, against `7b80cb0`, plus the active, uncommitted thinner worktree based on `7b80cb0`. Thinner was not merged into this integration snapshot. Its reviewed SHA-256 hashes begin `d465cf54` (thinner.rs), `4d1da858` (thinner_tests.rs), and `6898a6a9` (api.rs). Findings about thinner describe that snapshot; combined engine-3 thinner behavior remains unverified. Production code and tests were not edited.

**1. P1 — Failed chunks can silently change sparse-table length**

[heap.lua:71](~/src/a/claude-paint-r24/crates/easel/src/heap.lua:71) ignores raw length under engine 3 when deciding whether a table was touched. Identical entries and traversal order do not guarantee identical `#` results.

Run these as three separate chunks:

```lua
-- succeeds; prints 0
 t = {}; t[2] = 2; print(#t)
-- fails
 t[1]=1; t[3]=3; t[1]=nil; t[3]=nil; error('stop')
-- succeeds; prints 2
 print(#t)
```

The integrated CLI reports that the failed chunk changed nothing and does not rebuild. Close/reopen restores `#t` to 0. Later painting decisions can therefore differ from the recorded log's replay. Preserve sparse-table border observability when detecting changes, including temporary mutations that restore every entry. The existing sparse-table test changes `sparse[2]` to 99, which forces detection and misses this case.

**2. P1 — Thinner changes engine-2 printed output**

[api.rs:491](~/src/a/claude-paint-thinner/crates/easel/src/api.rs:491) installs the new `thinner` property for every engine. With an engine-2 header, a canvas and ordinary raw-sienna pile, this succeeds on both old and new binaries:

```lua
local ok,v=pcall(function() return p.thinner end)
print(ok,v)
```

Round-23 and brief1 binaries print `true\tnil`; thinner prints `true\t0.0`. This violates the explicit old-log output contract without using any new option. Gate the getter and new pile option by engine. Merely changing the default engine to 3 during integration will not fix old-log replay.

**3. P2 — Thinner has no solvent evaporation stage**

[thinner.rs:65](~/src/a/claude-paint-thinner/crates/paint/src/thinner.rs:65) permanently lowers stiffness and compensates the oil-curing rate. The canvas has no solvent quantity or evaporation clock; its deposited volume also includes solvent indefinitely until the normal drying model processes it. The omission is expressly documented in the same file.

This cannot produce the agreed minutes-to-matte/stiffer wash followed by oil setting over hours. Existing relative-setting tests pass on the thinner branch's engine 2, but do not establish the absolute engine-3 target. Add evaporation separately from oxidative cure and test the integrated engine-3 behavior at the painter API. No combined timing measurement was performed in this review.

This distinction is historically grounded: [Arthur H. Church, *The Chemistry of Paints and Painting*, oil-painting chapter](https://chestofbooks.com/science/chemistry/Paints/4-Oil-Painting-And-Spirit-Fresco-Continued.html) distinguishes solvent escape from oil hardening and describes the former as easier. It does not establish a universal numeric setting time.

**4. P2 — Half-thinned raw sienna fails the imprimatura target**

The live test at [thinner_tests.rs:282](~/src/a/claude-paint-thinner/crates/easel/src/thinner_tests.rs:282) fails: at 50% thinner, the rendered layer preserves approximately 49% and 20% of the underlying black/white card's contrast at brush loads 0.3 and 0.6. The required minimum is 50% for both. The substantial failure is the higher-load case, not rounding at 49%.

Verified by executing the existing release test binary with `thinner_tests::an_earth_thinned_half_is_an_imprimatura --exact --nocapture`. The test remains enabled and honestly documents its failure. Repair the rendered deposition/optics to meet the agreed target; retaining a failing test does not make the feature complete. [Field/Salter, 1869, raw sienna and burnt sienna entries](https://www.gutenberg.org/files/20915/20915-h/20915-h.htm) support their transparency qualitatively. The 50% threshold is the project's target, not a historical measurement.

**5. P2 — Historical raw/burnt sienna drying order needs correction or justification**

[palette.rs:135](~/src/a/claude-paint-r24/crates/paint/src/palette.rs:135) raises raw sienna's engine-3 drying factor to 2.3 while burnt sienna retains 1.2. The calibration fixture at equal 1.5-coat thickness gives raw sienna touch-dry at 45.25 hours and burnt sienna at 84.5 hours.

`DRYING_TABLE=1 cargo test -p paint strokes_dry_within_the_sources_ranges -- --nocapture` reproduces the table. [Field/Salter, 1869, §§50 and 155](https://www.gutenberg.org/files/20915/20915-h/20915-h.htm) describes calcination as improving sienna's drying. The new ordering follows modern manufacturer categories, but has not been justified for the historical earths named by the engine. Resolve that discrepancy and cover the intended ordering. This is a historical calibration finding; the manual does not prove every modern raw/burnt formulation has identical relative timing.

**6. P2 — A solvent-damp rag never dries out**

[rag.rs:135](~/src/a/claude-paint-r24/crates/paint/src/rag.rs:135) only increases `damp`; refolding is the only action that clears it. Verified with engine 3 after creating a canvas:

```lua
r=rag(); r:dip(0.5); print(r.damp)
wait(10080) -- one week
print(r.damp)
```

Both values are 0.5, so the old dip still gives the same solvent lift multiplier in a later sitting. Model the loss of volatile solvent with elapsed painting time. [Jennings, *Paint & Colour Mixing* (1902), “Testing the Purity of Turpentine”](https://www.gutenberg.org/cache/epub/56738/pg56738-images.html) describes a few drops evaporating from paper in minutes. That supports evaporation, but is not a measured lifetime for a bunched, paint-loaded rag.

**7. P2 — New drying tests fail supported restricted builds**

The complete pigment inventories in [drying.rs:959](~/src/a/claude-paint-r24/crates/paint/src/drying.rs:959) and [drying.rs:1004](~/src/a/claude-paint-r24/crates/paint/src/drying.rs:1004) are unconditional even when those tubes were excluded by features.

Verified failures:

```sh
cargo test -p paint --no-default-features strokes_dry_within_the_sources_ranges
# no tube "raw sienna"
cargo test -p paint --no-default-features fast_pigments_dry_before_slow_ones
# no tube "permanent alizarin"
```

Gate complete-catalog fixtures to the relevant features or exercise the tubes available in each supported configuration.

**Historical scope and rag calibration**

Rag removal itself has period support: the Beards' [*The American Girl's Handy Book*, painting chapter, pp. 254–255](https://www.gutenberg.org/cache/epub/52051/pg52051-images.html) describes a turpentine-dipped rag for uncovering an obscured outline. This supports removing wet paint, but supplies no 95% removal threshold or universal residual stain. The engine's fixed 0.04–0.08-coat stain and its documented 14% residual tone are estimates. Film removed and visible color removed are different acceptance criteria; the current rag test principally bounds the former. No additional numeric optical defect is asserted here.

Scumbling also needs a precise target. [Parkhurst, *The Painter in Oil* (1897), chapter XXIII, pp. 230–231](https://www.gutenberg.org/files/30877/30877-h/30877-h.htm) describes a lightly charged brush rubbing body color over dry paint, without thinning. A half-thinned lead-white opacity check can be useful, but is not by itself a historical scumbling test; brush loading and broken coverage matter too.

For earlier centuries, [White, Pilc and Kirby, *Analyses of Paint Media*, National Gallery Technical Bulletin 19 (1998), pp. 74–95](https://www.nationalgallery.org.uk/technical-bulletin/white_pilc_kirby1998) documents differences among historical schools: Dutch linseed and heat-bodied oils, occasional walnut oil, and later French poppyseed oil and added driers. The implication for this engine is that one pigment-only table plus generic oil/solvent represents a chosen material formulation, not all oil painting across centuries. The 60-hour baseline, 60% open share, reference stroke thickness and rag lift constants remain calibration estimates. These sources do not validate their exact values.

**Validation and limits**

The default drying suite passed 15 tests; all five rag API tests passed, including rollback and old-global behavior. Existing engine-1 golden paintings and the engine-2 golden passed. Repaints' focused engine-3 and legacy table-order/rebuild tests passed despite the reproduced sparse-table gap. Thinner's existing pigment-deposition and relative-setting checks passed; its imprimatura test failed. Goldens establish their covered pixels/relief, not all old printed output or complete state. This was a focused review, not a complete workspace test run. Thinner was reviewed separately while still under development.
