(•̀ᴗ•́) **Stop adding object detail until edges, contact and paint handling improve.** My diagnosis is not “models cannot paint trees.” It is that the current process makes separately finished objects, then struggles to make them belong to one picture. Surface rendering compounds that problem.

Image references below are filenames in `notes/showcase/`, unless another directory is given. These are visual judgments, not experimentally established engine causes.

## 1. What looks digital

- **Objects look pasted onto their surroundings.** In `2_near_loop5_best.jpg`, the large rock sits above a nearly continuous pale-gray base strip. Its right face changes value along a conspicuously clean seam. The stump occupies a rectangular patch of differently painted snow. These are stronger spatial errors than insufficient stone texture. In `8_tool_rocks.jpg`, dark outlines along the bottoms and smooth, sharply bounded cast shadows resemble rendered assets placed on a plane.
- **Detail lacks hierarchy.** In `6_tool_firs.jpg`, the sparse right-hand fir exposes hanging, disconnected-looking foliage packets; the dense center fir has a ladder of bright openings along its stem. Their boundaries remain insistently sharp. In `7_tool_trees.jpg`, leafy crowns have punched-out sky holes while distant trees remain little crown-on-stick symbols. More branching detail has not solved the organization of foliage into convincing masses.
- **The surface repeats itself across unlike passages.** `10b_ground_grain_after.jpg` replaces much of the horizontal pattern in `10a_ground_grain_before.jpg` with curved streaks, but those streaks still dominate a quiet sky passage. In `9b_halo_after.jpg`, stones still have embossed, wormlike interiors and the figure retains a light edge. A reduced halo is not yet an integrated edge.
- **There is real progress, not a global regression.** Comparing showcase 1→2, the wood gains openings and depth. Comparing 3→4, the valley gains atmospheric restraint and a less cartoonish sky, though the oak remains topiary-like. Round 2’s `notes/amnesia2/fresh2_coast.jpg` benefits from broad quiet areas and a restrained arrangement; its schematic net and floating-looking figure do not establish superior drawing. Easier subject matter is a confound.

## 2. The diagnosis is partly right

The paint problem is credible, but “relief lighting causes the plateau” is unproven. The guide exposes `relief(strength, gloss)`; test it without repainting. Flat lighting cannot repair the rock’s base strip or the tree’s punched holes.

The missing issue is **edge and value organization across objects**: where boundaries disappear, where shadows join forms and where detail should stop. Treating every silhouette as a region to finish independently encourages cutouts. The guide’s examples repeatedly use `clip=mask`; that is a hypothesis worth isolating, not a reason to ban masks.

There is also a workflow contradiction: `notes/sketchbook.md` §1 prescribes “dry() before any passage that goes over earlier work.” That protects against muddy accidents but makes wet interaction exceptional. Test controlled wet and tacky passages rather than merely demanding rougher strokes.

Critics are noisy, not demonstrably motif-only: `notes/scores.md` records substantial cross-batch drift and repeated criticism of halos, seams and shadows. Loop 5’s +1 median improvement includes one critic preferring the predecessor. Do not treat that as decisive validation of the tool architecture.

## 3. Keep structure; stop tracing everything

Structure-first is not a dead end. The bare oak in `7_tool_trees.jpg` is the strongest evidence against abandoning it: its branching is much more persuasive than the adjacent leaf masses.

Keep geometry for placement, major branching, light and occlusion. Stop making every generated feature a painting obligation. Paint one connected dark crown, a few unequal light masses and selected branch fragments; cut back only a few sky openings. Let brush deposits determine local boundaries. For rocks, start with three connected value families and ground contact before adding fractures. Compare this against the existing detailed recipe using identical geometry, palette and framing.

## 4. Ross: exercises, not a new destination

Borrow a tightly bounded mark-making lab, not another full-painting sprint. `notes/research/bob_ross.md` documents corner taps, fan-brush push-ups, dark-before-light masses and broken knife deposits. Those are useful tests. The same document labels rheology numbers as estimates; they are not calibration data.

A complete Ross implementation risks replacing Friedrich symbols with evergreen-and-mountain symbols. A better benchmark is subject-neutral: one foliage mass, one rock touching ground and one quiet sky passage. Return each successful technique to a Friedrich painting immediately. No new reference images are needed.

## 5. Three next changes, in order

1. **Separate surface artifacts from painting errors.** Compare identical saved paint states with current, reduced and zero relief lighting, at whole-image and native crop scales. Hold everything else fixed. Success: blind owner preference for less digital surfaces without losing form. If contours remain, investigate edge handling separately.
2. **Replace detail-first recipes with mass-and-edge studies.** Compare the existing detailed recipe against the simplified recipe in §3, holding geometry, palette and drying fixed. Success: the rock looks grounded and foliage reads as volume with fewer marks, without continuous pale rims or punched holes. Test a second composition before adoption.
3. **Test paint interaction instead of drying by default.** Repeat the same foliage-light and sky-blending gestures over dry, tacky and open paint, holding brush, load and colors fixed. Success: an interacting passage has less pasted-on edging without muddy values or exposed ground. Adopt selective waits only where that advantage survives transfer to a full painting.

For all three, freeze engine/version pairs, randomize presentation and judge surface, contact and drawing separately. Require repeated owner preference on studies and full-painting transfers; use critics diagnostically, not a one-point total as permission to merge. This addresses the drift in `notes/scores.md`.
