# Source evidence

All paths are relative to the extracted uploaded `engine3-rag-review/` packet. Line numbers below are original file line numbers. Hashes are of the complete original files. These are inspected source excerpts and recorded reports, not newly executed Rust results.

## E01 — Packet scope and source states

Source: `README.md`  
SHA-256: `180ff754cb0a813e8bd6abbc2162ce34d4fc3cd4144c1fd2ab95a3d8fb42f5cb`

### Lines 1–101

```text
    1: # Package scope: rag, thinner/wash and sienna
    2: 
    3: See [WIP.md](WIP.md) for the additional current source snapshots, reports and images.
    4: 
    5: # Engine 3 rag: external review packet
    6: 
    7: Prepared October 4, 2026. This is an unresolved research and implementation review, not a release candidate. The owner wants an accurate, convincing rag tool without another long sequence of speculative visual tweaks. The latest appearance has not been accepted. [Owner feedback](OWNER_FEEDBACK.md)
    8: 
    9: The package includes the current source, buildable experimental source snapshots, original images, comparison sheets, patches, measurements and recorded test output. `index.html` is the offline visual entry point. Historical experiment pages are preserved as written; this brief supplies the current interpretation and supersedes their earlier hypotheses.
   10: 
   11: ## The important distinction
   12: 
   13: Two different problems became entangled:
   14: 
   15: 1. Repeated outlined “islands” in the wipe were inherited from raised brush marks in the starting paint. The contact traces and controls support that diagnosis.
   16: 2. When starting thickness was made uniform, those islands disappeared, but the owner still rejected the resulting smooth, blurred-looking wipe. Identifying the islands' source did **not** validate the cloth model or solve the rag's appearance.
   17: 
   18: The latest discussion proposed finding a clear real wiping sequence and reproducing its contact behavior before further paint tuning. That reference-led rebuild has **not** happened. The next action was preparation of this review packet. [Latest diagnosis](notes/rag/contact-review/index.html) · [Rejected uniform-layer result](notes/rag/contact-review/uniform/after.png) · [Owner feedback](OWNER_FEEDBACK.md)
   19: 
   20: ## Source states
   21: 
   22: | Directory | Exact meaning |
   23: | --- | --- |
   24: | `shipping/` | Selected source and support files from main commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Complete Rust workspace, tests, lockfile and Lua build configuration. This already contains the released rag, without the deforming-cloth experiments. |
   25: | `prototype/` | The shipping snapshot plus `notes/rag/material-review/prototype.patch`. Connected cloth, moving grip, fine contact sampling, friction and persistent dirty paint. `LIFT = 4.0`. This is the most developed cloth prototype, not an accepted result. |
   26: | `research/` | The shipping snapshot plus `notes/rag/contact-review/research.patch`. Same cloth mechanics, with `LIFT = 1.0` and temporary contact/pickup/redeposition recording. Includes the uniform-thickness control and preparing-brush push-off control. |
   27: | `notes/rag/` | All six retained review stages, original PNGs, comparison images, source copies, patches, metrics and test logs. |
   28: | `background/look/` | Earlier agent's sienna, thinner-outline and procedural-rag experiments from sibling commit `196a673596ea2c5e9c037164f8ce01d90ef11a88`. Context only; not the shipping source or the later cloth prototype. |
   29: 
   30: Cloth, grip and material `prototype.patch` files are **alternative complete patches against the shipping commit**, not a stack to apply sequentially. The snapshots already have the appropriate patch applied. `background/look-vs-shipping.patch` records the earlier sibling's tracked code differences; it is not a proposed merge. [Packaging provenance](PACKAGING.json)
   31: 
   32: ## Run-to-source mapping
   33: 
   34: The dark comparison used the shipping source at `4e525e5` with `notes/rag/dark-review/render.rs`; that mapping is recorded session provenance, not a new rerender verification. Cloth and grip runs use their respective complete patches against the shipping snapshot. Final material images use `prototype/`; pickup images use that snapshot with LIFT = 4, 2 or 1. Contact diagnosis uses `research/`. [Machine-readable mapping](RUN_SOURCES.json)
   35: 
   36: The three intermediate material stages did not retain full workspace snapshots. Their recorded recipe is the grip patch, the paint-step change in `rag.rs` from `1.0 / mmu` to `0.5 / mmu` and replacement of `cloth.rs` by the saved module in the named stage directory. Complete patches reconstructed from that recipe are included for [half-step](background/reconstructed-half-step.patch), [fine-grid](background/reconstructed-fine-grid.patch) and [stick/slip](background/reconstructed-stick-slip.patch). The half-step module keeps mechanical substeps at 0.25 mm travel; fine-grid increases the contact grid to 97 × 97; stick/slip changes friction. Each reconstructed patch applies independently to shipping. Applicability was checked, but byte-identical reproduction of those intermediate images was not rerun or established during packaging.
   37: 
   38: ## Experiments, outcomes and failures
   39: 
   40: | Stage | Change and evidence | What it established or failed to establish |
   41: | --- | --- | --- |
   42: | Earlier look study | Procedural missed folds, surviving streaks, irregular edges and some smear-back. Also compared thinner-outline approaches. [Report](background/look/REPORT.md) | The thinner work traded outlines for overlapping blotches; an absolute film cap broke buildup and was reverted. This report predates the subsequent cloth work. |
   43: | Dark comparison | Shipping source `4e525e5`; same wipe over dark paint at 2400-pixel canvas width. [Page](notes/rag/dark-review/index.html) | Made dry/damp differences easier to inspect. No deforming cloth yet. |
   44: | First cloth | Persistent 19 × 19 mesh, distance constraints, flat support plane and fixed grip. [Page](notes/rag/cloth-review/index.html) | Owner said it was moving in the right direction but the lines were too neat. Five focused paint tests and six session tests passed; not physical validation. |
   45: | Moving grip | Smooth seeded rotation, translation and pressure tilt advanced by travel. [Page](notes/rag/grip-review/index.html) | Owner still described the result as digital. Eleven focused tests passed. |
   46: | Sampling, friction and dirty face | Paint steps 1 → 0.5 mm; contact grid 49² → 97²; position-level friction; paint retained in cloth material coordinates. [Page and intermediate stages](notes/rag/material-review/index.html) | Persistent local color transfer was demonstrated. Twelve focused tests passed. The damp result appeared to clear too much. Neither realistic contact nor removal rate was established. |
   47: | Pickup strength | `LIFT = 4, 2, 1`, all other cloth settings held fixed. [Page](notes/rag/pickup-review/index.html) | Reduced pickup exposed repeated islands. The owner rejected them. No rate was selected. |
   48: | Contact diagnosis | Recorded starting film, cumulative contact, pickup, redeposition and remaining film. Made starting thickness uniform in one control; disabled only the preparing brush's paint-pushing in another. [Page](notes/rag/contact-review/index.html) | Contact maps were identical across all three cases. Controls removed the repeated outlines without materially changing the center's remaining-film percentage. The uniform-layer result was separately rejected for its appearance. |
   49: | Deferred brush transport | Queued paint transfers until the end of a bristle step. [Result](notes/rag/contact-review/transport-check.txt) · [Reconstructed experiment patch](background/deferred-transport-experiment.patch) | Outlines persisted; the change was reverted. Mean RGB differences were only about 0.00003–0.00005 of an 8-bit level. The patch was reconstructed from the recorded edit script for this packet, checked for applicability and not rerun during packaging. |
   50: 
   51: The earlier working hypothesis that the cloth's bumpy preferred shape caused the islands was not supported by the controls. That does not prove the preferred shape, grip model or contact response are otherwise good. [Contact checks](notes/rag/contact-review/verification.txt)
   52: 
   53: ## What the measurements actually mean
   54: 
   55: The dark fixture uses color `#3f2b22`, `paint(0.85, 0.75)`, brush load 0.3, filbert width 22 and gap filling enabled. It is simulated paint, not a measured raw-umber formulation. The rag is 100 canvas units wide (44 mm on a 440 mm-wide canvas), pressure 0.8, dip 0 or 0.5 and travels from `(300,340)` to `(700,340)`. A second wipe uses the same face without refolding. Images show wet paint without baking. [Render source](notes/rag/material-review/render.rs)
   56: 
   57: The latest center measurement covers x = 350–650 and y = 310–370 canvas units. Initial thickness there averages 69.29 µm. At quarter pickup rate, the original layer leaves 8.58 µm (12.38%), the uniform control 8.62 µm (12.44%) and the preparing-brush push-off control 8.75 µm from an initial 70.39 µm (12.43%). These are simulated wet-film quantities, not measured physical results. [Exact data](notes/rag/contact-review/measurements.json)
   58: 
   59: At the original pickup rate, one damp wipe leaves 1.99% of the center's initial film but about 23.96% of its original darkness. “Darkness” is the linear-luminance deficit relative to the primed ground. It is not pigment mass or the percentage of pixels cleared. [Data and method](notes/rag/pickup-review/index.html)
   60: 
   61: Earlier test-log “paint left” percentages use a different blue fixture and a region containing substantial unwiped paint. They must not be substituted for these dark-image center measurements. The same logs also contain a separate repeated thin-tone test. [Original test output](notes/rag/material-review/tests.txt)
   62: 
   63: The diagnostic baseline reproduces the prior quarter-rate PNG exactly. Per-pixel accounting, initial film − pickup + redeposition, matches the final film within 0.00035 µm. Some diagnostic displays clip values at their labeled white point; the unclipped redeposition view is included separately. [Checks](notes/rag/contact-review/verification.txt) · [Accounting](notes/rag/contact-review/baseline/accounting.txt) · [Unclipped view](notes/rag/contact-review/deposit-unclipped.png)
   64: 
   65: ## Code map and modeling limits
   66: 
   67: - `prototype/crates/paint/src/rag/cloth.rs`: cloth shape, constraints, grip, contact rasterization and material coordinates. It starts from noisy height geometry on a disk-mapped regular grid. All vertices are pulled toward transformed targets. Contact is based on gap to a flat plane, not a calibrated contact-pressure solution. There is no self-collision.
   68: - `prototype/crates/paint/src/rag/face.rs`: locally retained paint, capacity, drying and redeposition budgets. This is the persistent dirty-face approximation.
   69: - `prototype/crates/paint/src/rag.rs`: solvent, pickup, residual film floor, path integration and coupling to the cloth. Pickup and solvent coefficients are estimates.
   70: - `shipping/crates/paint/src/bristle.rs`: preparation brush's pickup, deposition and outward/forward paint displacement. This is implicated by the push-off control.
   71: - `shipping/crates/paint/src/wet.rs`, `surface.rs` and `drying.rs`: wet-film state, optics, surface handling and drying.
   72: - `prototype/crates/easel/src/draw_rag.rs` and `session.rs`: Lua-facing rag behavior and rollback, including cloning persistent state.
   73: 
   74: The cloth solver is a coarse position-based approximation. The cited simulation algorithms supply numerical techniques; they do not establish the correctness of this particular rest shape, grip, fabric constants or paint interaction. [Prototype source](prototype/crates/paint/src/rag/cloth.rs)
   75: 
   76: ## Validation and reproduction
   77: 
   78: The recorded 12 focused passes cover six paint tests and six Lua/session tests, with one diagnostic test ignored. The local-color carryover test was also run against the preceding prototype and failed because no paint survived into the later separate contact. These checks support specific behavioral and accounting claims, not visual realism. No full release-validation run was completed for the cloth prototypes. [Passing log](notes/rag/material-review/tests.txt) · [Negative control](notes/rag/material-review/pre-feature-test.txt)
   79: 
   80: Each source snapshot has its own `Cargo.toml`, `Cargo.lock` and `.cargo/config.toml`. Cargo's `iter` profile is for development images; its floating-point output can differ from the release profile. The Lua build config must remain present. External Cargo dependencies are not vendored, so a fresh build needs a Rust/C toolchain and dependency access. No binaries or build caches are included.
   81: 
   82: The ready-made `prototype` example is `rag_cloth_review`, with an output directory argument. The `research` example is `rag_trace`; its three arguments are output directory, mode and scratch checkpoint path. Modes `baseline`, `uniform` and `no-plough` reproduce the retained diagnosis. A fresh task-specific checkpoint is necessary; the diagnostic cache is not source-version checked. The trace writes raw `.f32` fields and images, then the bundled figure scripts can assemble sheets. Raw fields from the original runs were deleted after measurement; their rendered maps, metrics and source remain. Python figure scripts use Pillow and macOS Arial font paths; the original PNGs require no Python or build to view.
   83: 
   84: `notes/rag/pickup-review/render.rs` supplies the exact pickup-sweep example; the three runs used LIFT values 4, 2 and 1. It is a source copy, not an installed Cargo example in the shipping snapshot. The original intermediate material records retained cloth modules and images; the reconstructed patches and their limits are described in the run-to-source mapping above.
   85: 
   86: ## Requested independent assessment
   87: 
   88: A useful review would address:
   89: 
   90: 1. Whether the source-layer diagnosis is justified and whether any inference above overreaches the controls.
   91: 2. Why the remaining uniform-layer rag looks like a soft eraser and which contact or transfer assumptions are responsible.
   92: 3. What physical observations or published data can distinguish plausible cloth marks from added decorative noise.
   93: 4. Whether the current cloth approach can be corrected economically or should be replaced, with the smallest defensible implementation change and its tradeoffs.
   94: 5. Whether brush displacement must be corrected before rag evaluation or can be isolated in separate fixtures.
   95: 6. A short verification plan that separates appearance, physical behavior, conservation and regression safety, with a faster iteration loop.
   96: 
   97: No solution architecture or coefficient is preselected. The required outcome is a believable rag with defensible behavior; a numerical test pass or a more complicated simulation alone is insufficient. [Owner feedback](OWNER_FEEDBACK.md)
   98: 
   99: ## Provenance and scope
  100: 
  101: The shipping checkout remained unchanged. No new commit, PR, merge or external upload was made for this packet. Git history, credentials, build caches, virtual environments and unrelated live painter data are excluded. The package retains original PNG/JPEG images and adds a manifest of delivered files. Historical pages may contain obsolete status language; the source-state table above controls interpretation. [Packaging details](PACKAGING.json)
```

## E02 — Rag contact, pickup, release, load

Source: `prototype/crates/paint/src/rag.rs`  
SHA-256: `e0242af63021738002d92ec77833c16fe513a42818b61e90848c1ed664f764a3`

### Lines 370–525

```text
  370:     /// then takes this step's lift into the pool. Lifts the open
  371:     /// paint and loads the rag; returns the volume lifted (mm³).
  372:     /// With `material`, instead transfer through persistent cloth cells;
  373:     /// the third tuple entry is travel as a fraction of the pad width.
  374:     ///
  375:     /// The cloth bridges between the local peaks of the surface (the ground,
  376:     /// set paint and the wet film on it, within `BRIDGE_MM`) and sags into
  377:     /// the hollows by `SAG_UM` more as it is pressed: the wet paint above
  378:     /// that level is in reach; below it, in the hollows of the weave and
  379:     /// between ridges, the fibers wick only a share (`WICK`).
  380:     fn rag_contact(&mut self, rag: &mut Rag, bbox: (f32, f32, f32, f32), pressure: f32, mut pool: Option<&mut Pool>, material: Option<(&cloth::Contact, [f32; 2], f32)>, expo: impl Fn(f32, f32) -> (f32, f32, f32)) -> f64 {
  381:         let f = self.f;
  382:         let s = f.scale;
  383:         let r = ((bbox.0 * s).floor().max(0.0) as usize, (bbox.1 * s).floor().max(0.0) as usize, ((bbox.2 * s).ceil().max(0.0) as usize + 1).min(f.full_w), ((bbox.3 * s).ceil().max(0.0) as usize + 1).min(f.full_h));
  384:         if r.2 <= r.0 || r.3 <= r.1 {
  385:             return 0.0;
  386:         }
  387:         let Some((x0, y0, x1, y1)) = f.clip(r) else { return 0.0 };
  388:         let p = pressure.clamp(0.0, 1.0);
  389:         let px_mm = self.px_mm();
  390:         let volume_mm3 = px_mm * px_mm * COAT_UM / 1000.0;
  391:         let mut face = material.map(|_| {
  392:             let mut f = rag.face.take().unwrap_or_else(|| face::Face::new(rag.width * self.mm_per_unit, rag.soaked, self.now_min()));
  393:             f.age(self.now_min());
  394:             f
  395:         });
  396:         let material_mmu = self.mm_per_unit;
  397:         let weights = |x: f32, y: f32| {
  398:             material.map(|(c, center, _)| c.weights((x-center[0]) * material_mmu, (y-center[1]) * material_mmu))
  399:                 .unwrap_or([(0, 0.0); 4])
  400:         };
  401:         let mut picked = vec![Pool::default(); if face.is_some() { cloth::CELLS } else { 0 }];
  402:         let mut exposure = vec![0.0f64; picked.len()];
  403:         // the surface (µm) and its local peaks, over the box and a bridge's
  404:         // reach around it
  405:         let rb = ((BRIDGE_MM / px_mm).round() as usize).max(1);
  406:         let (bx0, by0, bx1, by1) = (x0.saturating_sub(rb), y0.saturating_sub(rb), (x1 + rb).min(f.w), (y1 + rb).min(f.h));
  407:         let bw = bx1 - bx0;
  408:         let surf: Vec<f32> = (by0..by1).flat_map(|y| (bx0..bx1).map(move |x| (y, x))).map(|(y, x)| {
  409:             let i = y * f.w + x;
  410:             self.height[i] + self.wet.vol[i].max(0.0) * COAT_UM
  411:         }).collect();
  412:         let peaks = local_max(&surf, bw, by1 - by0, rb);
  413:         rag.evaporate(self.now_min());
  414:         let d = rag.damp.clamp(0.0, 1.0);
  415:         let e3 = self.engine >= 3;
  416:         // (engine 3: spirits reach deeper, `DAMP_REACH`)
  417:         let (reach, wick) = if e3 { (SAG_UM * (0.25 + 1.5 * p) * (1.0 + DAMP_REACH * d), WICK + (1.0 - WICK) * d) } else { (SAG_UM * (0.25 + 1.5 * p), WICK) };
  418:         let k = LIFT * (0.7 + 0.6 * p) * rag.thirst() * (1.0 + DAMP_LIFT * d);
  419:         let timed = self.wet.clock.px.len() == self.wet.vol.len();
  420:         let mut lifted = 0.0f64;
  421:         let mut got = Pool::default();
  422:         let pooled = pool.is_some();
  423:         for y in y0..y1 {
  424:             let mut row = 0.0f32;
  425:             for x in x0..x1 {
  426:                 let i = y * f.w + x;
  427:                 let v = self.wet.vol[i];
  428:                 let (pad, c, _) = expo(f.ux(x), f.uy(y));
  429:                 let material_weights = weights(f.ux(x), f.uy(y));
  430:                 if face.is_some() {
  431:                     for (j, w) in material_weights {
  432:                         exposure[j] += (w * pad * c * px_mm * px_mm) as f64;
  433:                     }
  434:                 }
  435:                 if v <= 1e-6 {
  436:                     continue;
  437:                 }
  438:                 let e = pad * c;
  439:                 if e <= 0.0 {
  440:                     continue;
  441:                 }
  442:                 let fl = if timed { reach_fluid(self.wet.clock.px[i].cure) } else { 1.0 };
  443:                 if fl <= 0.0 {
  444:                     continue;
  445:                 }
  446:                 let j = (y - by0) * bw + (x - bx0);
  447:                 // the film above the cloth's level, coats
  448:                 let level = peaks[j] - reach;
  449:                 let near = ((surf[j] - level) / COAT_UM).clamp(0.0, v);
  450:                 let avail = near + wick * (v - near);
  451:                 // engine 3: the cloth's contact shares out what the pad's
  452:                 // rate takes, so its creases show however damp it is
  453:                 let local = face.as_ref().map_or(1.0, |f| f.thirst(material_weights));
  454:                 let frac = if e3 { (1.0 - (-k * local * fl * pad).exp()) * c } else { 1.0 - (-k * fl * e).exp() };
  455:                 // the stain: more of it where the cloth didn't reach
  456:                 let floor = STAIN_COATS * (2.0 - near / v);
  457:                 let take = (avail * frac).min(v - floor);
  458:                 if take > 0.0 {
  459:                     if face.is_some() {
  460:                         let cure = if timed { self.wet.clock.px[i].cure } else { 0.0 };
  461:                         for (j, weight) in material_weights {
  462:                             face::mix(&mut picked[j], take * volume_mm3 * weight, &self.wet.lat[i], &self.wet.hide[i], cure);
  463:                         }
  464:                     }
  465:                     if pooled {
  466:                         let l = &self.wet.lat[i];
  467:                         for q in 0..l.len() {
  468:                             got.lat[q] += l[q] * take;
  469:                         }
  470:                         let h = &self.wet.hide[i];
  471:                         for q in 0..3 {
  472:                             got.hide[q] += h[q] * take;
  473:                         }
  474:                         if timed {
  475:                             got.cure += self.wet.clock.px[i].cure * take;
  476:                         }
  477:                         got.vol += take;
  478:                     }
  479:                     self.rag_take(i, take);
  480:                     row += take;
  481:                 }
  482:             }
  483:             lifted += row as f64;
  484:         }
  485:         if let Some(pl) = pool.as_deref_mut() {
  486:             lifted -= self.rag_smear(pl, &got, (x0, y0, x1, y1), &expo);
  487:         }
  488:         if let Some(mut dirty) = face.take() {
  489:             // Budget each material cell once, then distribute that budget over
  490:             // its actual contacts. Newly lifted paint joins after depositing,
  491:             // so traversal order cannot move paint from one pixel to the next.
  492:             let budgets = dirty.budgets(&exposure);
  493:             let mut deposited = vec![0.0f64; cloth::CELLS];
  494:             for y in y0..y1 {
  495:                 for x in x0..x1 {
  496:                     let (pad, c, _) = expo(f.ux(x), f.uy(y));
  497:                     if pad * c <= 0.0 { continue; }
  498:                     for (j, weight) in weights(f.ux(x), f.uy(y)) {
  499:                         if exposure[j] <= 0.0 || weight <= 0.0 { continue; }
  500:                         let amount = (budgets[j] * (weight * pad * c * px_mm * px_mm) as f64 / exposure[j])
  501:                             .min((budgets[j] - deposited[j]).max(0.0));
  502:                         if amount <= 0.0 { continue; }
  503:                         let coats = (amount / volume_mm3 as f64) as f32;
  504:                         self.rag_lay(y * f.w + x, coats, &dirty.cells[j].paint);
  505:                         deposited[j] += coats as f64 * volume_mm3 as f64;
  506:                         lifted -= coats as f64;
  507:                     }
  508:                 }
  509:             }
  510:             self.wet.touch(x0, y0, x1, y1);
  511:             dirty.finish(&picked, &deposited, material.unwrap().2);
  512:             rag.face = Some(dirty);
  513:         }
  514:         let mm3 = lifted * (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
  515:         let w_mm = (rag.width * self.mm_per_unit) as f64;
  516:         let cap = w_mm * w_mm * (CAP_UM as f64 / 1000.0);
  517:         if pooled || material.is_some() {
  518:             // (a step can lay back more than it lifts)
  519:             rag.load = (rag.load + (mm3 / cap) as f32).clamp(0.0, 1.0);
  520:             rag.soaked = (rag.soaked + (mm3 / (cap * FACES as f64)) as f32).clamp(0.0, 1.0);
  521:         } else {
  522:             rag.load = (rag.load + (mm3 / cap) as f32).min(1.0);
  523:             rag.soaked = (rag.soaked + (mm3 / (cap * FACES as f64)) as f32).min(1.0);
  524:         }
  525:         mm3
```

## E03 — Rag path sampling and world placement

Source: `prototype/crates/paint/src/rag.rs`  
SHA-256: `e0242af63021738002d92ec77833c16fe513a42818b61e90848c1ed664f764a3`

### Lines 600–672

```text
  600:     /// lifted (mm³). One point is a blot (`rag_blot`).
  601:     pub fn rag_wipe(&mut self, rag: &mut Rag, pts: &[(f32, f32)], pressure: &[f32], seed: u64) -> f64 {
  602:         if pts.is_empty() {
  603:             return 0.0;
  604:         }
  605:         let pr = |t: f32| -> f32 {
  606:             match pressure.len() {
  607:                 0 => 0.5,
  608:                 1 => pressure[0],
  609:                 n => {
  610:                     let x = t.clamp(0.0, 1.0) * (n - 1) as f32;
  611:                     let i = (x as usize).min(n - 2);
  612:                     pressure[i] + (pressure[i + 1] - pressure[i]) * (x - i as f32)
  613:                 }
  614:             }
  615:         };
  616:         let len: f32 = pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum();
  617:         if pts.len() == 1 || len < 1e-3 {
  618:             return self.rag_blot(rag, pts[0].0, pts[0].1, pr(0.0), seed);
  619:         }
  620:         let mmu = self.mm_per_unit;
  621:         self.tally.rag_stroke((len * mmu) as f64, (rag.width * mmu) as f64);
  622:         let cs = rag.cloth_seed(seed);
  623:         let e3 = self.engine >= 3;
  624:         let w = rag.width;
  625:         let r0 = 0.5 * w;
  626:         // A persistent contact footprint must be swept at finer spacing than
  627:         // the old procedural exposure; otherwise individual stamps repeat.
  628:         let step = if e3 { (0.5 / mmu).min(0.5 * r0) } else { 0.5 * r0 };
  629:         let mut pool = Pool::default();
  630:         let mut total = 0.0;
  631:         let mut s_at = 0.0f32;
  632:         for seg in pts.windows(2) {
  633:             let (a, b) = (seg[0], seg[1]);
  634:             let (dx, dy) = (b.0 - a.0, b.1 - a.1);
  635:             let l = (dx * dx + dy * dy).sqrt();
  636:             if l < 1e-6 {
  637:                 continue;
  638:             }
  639:             let (tx, ty) = (dx / l, dy / l);
  640:             let n = (l / step).ceil().max(1.0) as usize;
  641:             for k in 0..n {
  642:                 let (l0, l1) = (l * k as f32 / n as f32, l * (k + 1) as f32 / n as f32);
  643:                 let sm = s_at + 0.5 * (l0 + l1);
  644:                 // the pad's width and line wander as the cloth shifts in the hand
  645:                 let wob = 2.0 * vn(sm / (2.5 * w), 0.5, cs ^ 0xA1) - 1.0;
  646:                 let side = 2.0 * vn(sm / (3.0 * w), 1.5, cs ^ 0xB2) - 1.0;
  647:                 let (r, off) = if e3 {
  648:                     // (engine 3: more, and at a second, quicker scale)
  649:                     let wob2 = 2.0 * vn(sm / (0.7 * w), 2.5, cs ^ 0xA7) - 1.0;
  650:                     let side2 = 2.0 * vn(sm / (0.7 * w), 3.5, cs ^ 0xB8) - 1.0;
  651:                     (r0 * (1.0 + WANDER_W * (0.65 * wob + 0.35 * wob2)), WANDER_OFF * w * (0.65 * side + 0.35 * side2))
  652:                 } else {
  653:                     (r0 * (1.0 + 0.12 * wob), 0.06 * w * side)
  654:                 };
  655:                 let p = pr(sm / len);
  656:                 let (ax, ay) = (a.0 + tx * l0, a.1 + ty * l0);
  657:                 let sl = l1 - l0;
  658:                 if e3 {
  659:                     let contact = rag.press_cloth(mmu, p, [tx * sl * mmu, ty * sl * mmu]);
  660:                     let cx = ax + tx * sl * 0.5 - ty * off;
  661:                     let cy = ay + ty * sl * 0.5 + tx * off;
  662:                     let radius = contact.extent() / mmu;
  663:                     let bbox = (cx - radius, cy - radius, cx + radius, cy + radius);
  664:                     total += self.rag_contact(rag, bbox, p, None, Some((&contact, [cx, cy], sl / w)), |x, y| {
  665:                         let (dx, dy) = (x - cx, y - cy);
  666:                         let c = contact.at(dx * mmu, dy * mmu);
  667:                         let radial = dx.hypot(dy) / radius;
  668:                         let pad = sl / w * (1.0 - smoothstep(0.75, 1.0, radial));
  669:                         let rim = smoothstep(0.55, 1.0, radial) * c;
  670:                         (pad, c, rim)
  671:                     });
  672:                     continue;
```

## E04 — Cloth geometry, relaxation and contact raster

Source: `prototype/crates/paint/src/rag/cloth.rs`  
SHA-256: `f57db9eb6b823990f29c15213246986ae2ed52835c67eedb136d29a19f63ad48`

### Lines 13–230

```text
   13: const N: usize = 19;
   14: pub(super) const CELLS: usize = N * N;
   15: const MAP: usize = 97;
   16: const ITERATIONS: usize = 8;
   17: // All mechanical coefficients below are visual-model estimates.
   18: const GRIP: f32 = 0.045;
   19: const STATIC_FRICTION: f32 = 0.8;
   20: const SLIDING_FRICTION: f32 = 0.35;
   21: const FIBER_MM: f32 = 0.18;
   22: 
   23: #[derive(Clone, Debug, PartialEq)]
   24: pub(super) struct Cloth {
   25:     rest: Vec<[f32; 3]>,
   26:     pos: Vec<[f32; 3]>,
   27:     links: Vec<(usize, usize, f32, f32)>,
   28:     width: f32,
   29:     seed: u64,
   30:     travel_mm: f32,
   31: }
   32: 
   33: pub(super) struct Contact {
   34:     field: Vec<f32>,
   35:     material: Vec<[f32; 2]>,
   36:     extent: f32,
   37: }
   38: 
   39: impl Cloth {
   40:     pub(super) fn new(width: f32, seed: u64) -> Self {
   41:         let mut rest = Vec::with_capacity(N * N);
   42:         for y in 0..N {
   43:             for x in 0..N {
   44:                 let u = 2.0 * x as f32 / (N - 1) as f32 - 1.0;
   45:                 let v = 2.0 * y as f32 / (N - 1) as f32 - 1.0;
   46:                 // Square-to-disk mapping keeps the grid connected. Initial
   47:                 // bunching is seeded once per fold, never once per stroke.
   48:                 let px = u * (1.0 - 0.5 * v * v).sqrt();
   49:                 let py = v * (1.0 - 0.5 * u * u).sqrt();
   50:                 let edge = 0.85 + 0.3 * vn(px * 3.0, py * 3.0, seed ^ 0x12);
   51:                 let px = px * 0.5 * width * edge;
   52:                 let py = py * 0.5 * width * edge;
   53:                 let crease = 0.65 * vn(px / 3.0, py / 8.0, seed)
   54:                     + 0.35 * vn(px / 9.0, py / 12.0, seed ^ 0x51ED);
   55:                 let z = 4.0 * crease + 0.7 * (u * u + v * v);
   56:                 rest.push([px, py, z]);
   57:             }
   58:         }
   59:         let mut links = Vec::new();
   60:         for y in 0..N {
   61:             for x in 0..N {
   62:                 let a = y * N + x;
   63:                 // Neighbors resist stretch and shear. Second neighbors
   64:                 // resist bending weakly, preserving folds without rigidity.
   65:                 for (dx, dy, stiffness) in [(1, 0, 0.85), (0, 1, 0.85), (1, 1, 0.6), (-1, 1, 0.6), (2, 0, 0.12), (0, 2, 0.12)] {
   66:                     let (xx, yy) = (x as i32 + dx, y as i32 + dy);
   67:                     if xx < 0 || xx >= N as i32 || yy >= N as i32 { continue; }
   68:                     let b = yy as usize * N + xx as usize;
   69:                     let length = (0..3).map(|k| (rest[a][k] - rest[b][k]).powi(2)).sum::<f32>().sqrt();
   70:                     links.push((a, b, length, stiffness));
   71:                 }
   72:             }
   73:         }
   74:         Self { pos: rest.clone(), rest, links, width, seed, travel_mm: 0.0 }
   75:     }
   76: 
   77:     /// Advance by physical travel, independent of canvas pixel resolution.
   78:     /// The plane is the coarse support surface; the paint engine separately
   79:     /// resolves linen peaks and the solvent's reach into its hollows.
   80:     pub(super) fn press(&mut self, pressure: f32, travel: [f32; 2]) -> Contact {
   81:         let distance = travel[0].hypot(travel[1]);
   82:         // Hold mechanical travel per substep constant when refining paint
   83:         // sampling; otherwise half-sized stamps double the relaxation rate.
   84:         let steps = if distance > 0.0 { (distance / 0.25).ceil().max(1.0) as usize } else { 4 };
   85:         let motion = [travel[0] / steps as f32, travel[1] / steps as f32];
   86:         let depression = 0.8 + 3.0 * pressure.clamp(0.0, 1.0);
   87:         for _ in 0..steps {
   88:             self.travel_mm += distance / steps as f32;
   89:             // Hand motion changes the grip, not the rendered contact mask.
   90:             // Persist its phase across wipes; refolding creates a new grip.
   91:             // These amplitudes are estimates, not measured hand mechanics.
   92:             let phase = self.travel_mm / self.width;
   93:             let drift = |channel| {
   94:                 0.65 * (2.0 * vn(phase / 1.4, channel, self.seed ^ 0x4719) - 1.0)
   95:                     + 0.35 * (2.0 * vn(phase / 0.45, channel, self.seed ^ 0x82AB) - 1.0)
   96:             };
   97:             let angle = 0.55 * drift(1.3);
   98:             let (sin, cos) = angle.sin_cos();
   99:             let shift = [0.09 * self.width * drift(3.7), 0.09 * self.width * drift(7.1)];
  100:             let tilt = [1.6 * drift(11.3), 1.6 * drift(17.9)];
  101:             let targets: Vec<[f32; 3]> = self.rest.iter().map(|rest| {
  102:                 let u = rest[0] / (0.5 * self.width);
  103:                 let v = rest[1] / (0.5 * self.width);
  104:                 [cos * rest[0] - sin * rest[1] + shift[0],
  105:                  sin * rest[0] + cos * rest[1] + shift[1],
  106:                  rest[2] - depression + tilt[0] * u + tilt[1] * v]
  107:             }).collect();
  108:             // Express the previous world-space positions in the moving hand
  109:             // frame. Contacting points stick there until tangential pull
  110:             // exceeds their normal-contact friction budget.
  111:             let previous: Vec<[f32; 3]> = self.pos.iter().map(|p| [p[0] - motion[0], p[1] - motion[1], p[2]]).collect();
  112:             self.pos.clone_from(&previous);
  113:             for iteration in 0..ITERATIONS {
  114:                 for (p, target) in self.pos.iter_mut().zip(&targets) {
  115:                     for k in 0..3 {
  116:                         p[k] += GRIP * (target[k] - p[k]);
  117:                     }
  118:                 }
  119:                 // Alternate order to avoid a persistent solver-direction bias.
  120:                 for j in 0..self.links.len() {
  121:                     let i = if iteration % 2 == 0 { j } else { self.links.len() - 1 - j };
  122:                     let (a, b, rest, stiffness) = self.links[i];
  123:                     let d: [f32; 3] = std::array::from_fn(|k| self.pos[b][k] - self.pos[a][k]);
  124:                     let length = d.iter().map(|x| x * x).sum::<f32>().sqrt();
  125:                     if length < 1e-6 { continue; }
  126:                     let amount = 0.5 * stiffness * (length - rest) / length;
  127:                     for (k, delta) in d.into_iter().enumerate() {
  128:                         self.pos[a][k] += amount * delta;
  129:                         self.pos[b][k] -= amount * delta;
  130:                     }
  131:                 }
  132:                 // Position-level Coulomb friction, Macklin et al. 2014,
  133:                 // section 6.1, eq. 24: https://mmacklin.com/uppfrta_preprint.pdf
  134:                 // Coefficients above remain uncalibrated estimates.
  135:                 for (p, old) in self.pos.iter_mut().zip(&previous) {
  136:                     if p[2] >= 0.0 { continue; }
  137:                     let normal = -p[2];
  138:                     p[2] = 0.0;
  139:                     let delta = [p[0] - old[0], p[1] - old[1]];
  140:                     let tangent = delta[0].hypot(delta[1]);
  141:                     let fraction = if tangent <= STATIC_FRICTION * normal { 1.0 }
  142:                         else { (SLIDING_FRICTION * normal / tangent).min(1.0) };
  143:                     p[0] -= fraction * delta[0];
  144:                     p[1] -= fraction * delta[1];
  145:                 }
  146:             }
  147:         }
  148:         self.contact()
  149:     }
  150: 
  151:     fn contact(&self) -> Contact {
  152:         let extent = self.pos.iter().flat_map(|p| [p[0].abs(), p[1].abs()]).fold(self.width * 0.5, f32::max) + 0.5;
  153:         let mut contact = Contact { field: vec![0.0; MAP * MAP], material: vec![[0.0; 2]; MAP * MAP], extent };
  154:         for y in 0..N - 1 {
  155:             for x in 0..N - 1 {
  156:                 let a = y * N + x;
  157:                 for ids in [[a, a + 1, a + N], [a + 1, a + N + 1, a + N]] {
  158:                     let p = ids.map(|i| self.pos[i]);
  159:                     contact.triangle(p, ids.map(|i| [(i % N) as f32, (i / N) as f32]));
  160:                 }
  161:             }
  162:         }
  163:         contact
  164:     }
  165: }
  166: 
  167: impl Contact {
  168:     pub(super) fn extent(&self) -> f32 { self.extent }
  169: 
  170:     fn triangle(&mut self, p: [[f32; 3]; 3], uv: [[f32; 2]; 3]) {
  171:         let to_grid = |v: f32| (v / self.extent + 1.0) * 0.5 * (MAP - 1) as f32;
  172:         let q = p.map(|p| [to_grid(p[0]), to_grid(p[1])]);
  173:         let area = (q[1][0] - q[0][0]) * (q[2][1] - q[0][1]) - (q[1][1] - q[0][1]) * (q[2][0] - q[0][0]);
  174:         if area.abs() < 1e-6 { return; }
  175:         let lo = |k| q.iter().map(|p| p[k]).fold(f32::INFINITY, f32::min).floor().max(0.0) as usize;
  176:         let hi = |k| q.iter().map(|p| p[k]).fold(f32::NEG_INFINITY, f32::max).ceil().min((MAP - 1) as f32) as usize;
  177:         for y in lo(1)..=hi(1) {
  178:             for x in lo(0)..=hi(0) {
  179:                 let dx = x as f32 - q[0][0];
  180:                 let dy = y as f32 - q[0][1];
  181:                 let b = (dx * (q[2][1] - q[0][1]) - dy * (q[2][0] - q[0][0])) / area;
  182:                 let c = ((q[1][0] - q[0][0]) * dy - (q[1][1] - q[0][1]) * dx) / area;
  183:                 let a = 1.0 - b - c;
  184:                 if a < 0.0 || b < 0.0 || c < 0.0 { continue; }
  185:                 let gap = a * p[0][2] + b * p[1][2] + c * p[2][2];
  186:                 let touch = 1.0 - smoothstep(0.0, FIBER_MM, gap);
  187:                 let i = y * MAP + x;
  188:                 if touch > self.field[i] {
  189:                     self.field[i] = touch;
  190:                     self.material[i] = std::array::from_fn(|k| a * uv[0][k] + b * uv[1][k] + c * uv[2][k]);
  191:                 }
  192:             }
  193:         }
  194:     }
  195: 
  196:     pub(super) fn at(&self, x: f32, y: f32) -> f32 {
  197:         self.sample(x, y).0
  198:     }
  199: 
  200:     /// Bilinear weights in the cloth's material coordinates. Paint stays on
  201:     /// those material locations when their projected positions deform.
  202:     pub(super) fn weights(&self, x: f32, y: f32) -> [(usize, f32); 4] {
  203:         let (contact, uv) = self.sample(x, y);
  204:         if contact <= 0.0 { return [(0, 0.0); 4]; }
  205:         let u = uv[0].clamp(0.0, (N - 1) as f32);
  206:         let v = uv[1].clamp(0.0, (N - 1) as f32);
  207:         let ix = (u as usize).min(N - 2);
  208:         let iy = (v as usize).min(N - 2);
  209:         let (fx, fy) = (u - ix as f32, v - iy as f32);
  210:         let i = iy * N + ix;
  211:         [(i, (1.0 - fx) * (1.0 - fy)), (i + 1, fx * (1.0 - fy)),
  212:          (i + N, (1.0 - fx) * fy), (i + N + 1, fx * fy)]
  213:     }
  214: 
  215:     fn sample(&self, x: f32, y: f32) -> (f32, [f32; 2]) {
  216:         let gx = (x / self.extent + 1.0) * 0.5 * (MAP - 1) as f32;
  217:         let gy = (y / self.extent + 1.0) * 0.5 * (MAP - 1) as f32;
  218:         if gx < 0.0 || gy < 0.0 || gx >= (MAP - 1) as f32 || gy >= (MAP - 1) as f32 { return (0.0, [0.0; 2]); }
  219:         let (ix, iy) = (gx as usize, gy as usize);
  220:         let (fx, fy) = (gx - ix as f32, gy - iy as f32);
  221:         let i = iy * MAP + ix;
  222:         let mut c = 0.0;
  223:         let mut uv = [0.0; 2];
  224:         for (j, w) in [(i, (1.0-fx)*(1.0-fy)), (i+1, fx*(1.0-fy)), (i+MAP, (1.0-fx)*fy), (i+MAP+1, fx*fy)] {
  225:             let a = w * self.field[j];
  226:             c += a;
  227:             for k in 0..2 { uv[k] += a * self.material[j][k]; }
  228:         }
  229:         if c > 0.0 { for v in &mut uv { *v /= c; } }
  230:         (c, uv)
```

## E05 — Persistent dirty face and budgets

Source: `prototype/crates/paint/src/rag/face.rs`  
SHA-256: `0a2de74083051c8367a8e34edb51db583b9ae5f51664e7d3bb67e516534a13de`

### Lines 1–71

```text
    1: //! Paint attached to material locations on the active cloth face. All stored
    2: //! volumes are mm³, independent of canvas resolution. Absorbed paint remains
    3: //! in `held`; only still-wet surface paint can return to the canvas.
    4: use super::{Pool, CAP_UM, COAT_UM, SMEAR, SOAK, cloth};
    5: use crate::{drying, wet::{Latent, Prop}};
    6: 
    7: #[derive(Clone, Debug, Default, PartialEq)]
    8: pub(super) struct Cell {
    9:     pub paint: Pool,
   10:     held: f32,
   11: }
   12: 
   13: #[derive(Clone, Debug, PartialEq)]
   14: pub(super) struct Face {
   15:     pub cells: Vec<Cell>,
   16:     area: f32,
   17:     at_min: f64,
   18: }
   19: 
   20: impl Face {
   21:     pub fn new(width_mm: f32, soaked: f32, now: f64) -> Self {
   22:         let area = width_mm * width_mm / cloth::CELLS as f32;
   23:         let held = soaked * area * CAP_UM / 1000.0;
   24:         Self { cells: vec![Cell { held, ..Cell::default() }; cloth::CELLS], area, at_min: now }
   25:     }
   26: 
   27:     pub fn age(&mut self, now: f64) {
   28:         let elapsed = (now - self.at_min).max(0.0) as f32;
   29:         for cell in &mut self.cells {
   30:             let p = &mut cell.paint;
   31:             if p.vol <= 0.0 { continue; }
   32:             let coats = p.vol / self.area * 1000.0 / COAT_UM;
   33:             p.cure += elapsed * drying::rate(coats, p.hide[1], p.hide[2]);
   34:             // Set paint is retained in the cloth; fresh pickup cannot revive it.
   35:             if p.cure >= drying::GEL { *p = Pool::default(); }
   36:         }
   37:         self.at_min = self.at_min.max(now);
   38:     }
   39: 
   40:     pub fn thirst(&self, weights: [(usize, f32); 4]) -> f32 {
   41:         let capacity = self.area * CAP_UM / 1000.0;
   42:         weights.iter().map(|&(i, w)| w * (1.0 - (self.cells[i].held / capacity).clamp(0.0, 1.0).powi(2))).sum()
   43:     }
   44: 
   45:     pub fn budgets(&self, exposure: &[f64]) -> Vec<f64> {
   46:         self.cells.iter().zip(exposure).map(|(cell, &area)| {
   47:             cell.paint.vol as f64 * drying::fluid(cell.paint.cure) as f64
   48:                 * (1.0 - (-SMEAR as f64 * area / self.area as f64).exp())
   49:         }).collect()
   50:     }
   51: 
   52:     pub fn finish(&mut self, picked: &[Pool], deposited: &[f64], travel_widths: f32) {
   53:         // Use physical travel, not stamp count, for soaking into the fibers.
   54:         let surface_left = (1.0 - SOAK).powf(4.0 * travel_widths);
   55:         for ((cell, got), &out) in self.cells.iter_mut().zip(picked).zip(deposited) {
   56:             cell.paint.vol = (cell.paint.vol - out as f32).max(0.0) * surface_left;
   57:             cell.held = (cell.held + got.vol - out as f32).max(0.0);
   58:             mix(&mut cell.paint, got.vol, &got.lat, &got.hide, got.cure);
   59:         }
   60:     }
   61: }
   62: 
   63: pub(super) fn mix(p: &mut Pool, volume: f32, lat: &Latent, hide: &Prop, cure: f32) {
   64:     if volume <= 0.0 { return; }
   65:     let total = p.vol + volume;
   66:     let a = volume / total;
   67:     for k in 0..p.lat.len() { p.lat[k] += (lat[k] - p.lat[k]) * a; }
   68:     for k in 0..3 { p.hide[k] += (hide[k] - p.hide[k]) * a; }
   69:     p.cure += (cure - p.cure) * a;
   70:     p.vol = total;
   71: }
```

## E06 — Rag parameters, dip, fold, blot

Source: `prototype/crates/paint/src/rag.rs`  
SHA-256: `e0242af63021738002d92ec77833c16fe513a42818b61e90848c1ed664f764a3`

### Lines 62–114

```text
   62: /// The pad's width, mm, when none is given: a cloth bunched over two or
   63: /// three fingers [E].
   64: pub const PAD_MM: f32 = 40.0;
   65: /// Lift per pass, a rate: one pass of a clean face at pressure 0.5, through
   66: /// the pad's middle where a crease presses fully, takes `1 - exp(-LIFT)`
   67: /// (98%) of the fresh paint in its reach; the folds between creases (down
   68: /// to 0.4 of that contact) and the pad's soft edge take less. Set so one
   69: /// pass lifts about half of a thin fresh sky film, as in the table in
   70: /// notes/rag/README.md [E].
   71: const LIFT: f32 = 4.0;
   72: /// What a blot lifts, as a share of a full pass (no drag) [E].
   73: const BLOT: f32 = 0.8;
   74: /// The stain: coats of film the cloth can't take, pigment caught in the
   75: /// tooth, where the cloth reaches all the film; twice as much where it
   76: /// reaches none of it (the hollows) [E].
   77: const STAIN_COATS: f32 = 0.04;
   78: /// How far the cloth bridges between peaks of the surface (mm): about the
   79: /// spacing of the threads of a 15-thread linen [E].
   80: const BRIDGE_MM: f32 = 0.7;
   81: /// How deep below the local peaks the cloth reaches at pressure 0.5 (µm);
   82: /// it goes from a quarter of this barely touching to 1.75 times pressed
   83: /// hard [E].
   84: const SAG_UM: f32 = 60.0;
   85: /// Share of the paint out of the cloth's reach its fibers wick up anyway [E].
   86: const WICK: f32 = 0.25;
   87: /// How much one face of the pad holds before it lifts nothing more: a film
   88: /// this thick (µm) over the face (`width` square). Cotton cloth some 0.3 mm
   89: /// thick, with paint caught in its surface as well [E].
   90: const CAP_UM: f32 = 400.0;
   91: /// A face dipped in spirits lifts wet paint more readily: its lift rate
   92: /// is `1 + DAMP_LIFT × damp` times a dry face's, and that is all a dip
   93: /// does. "To strengthen the lights, dip the rag into OMS and then wipe them
   94: /// out" (R. Palesca, wipe-out underpainting). Set so a rag dipped at 0.5
   95: /// and gone over a thin tone three times by hand takes 95% of the film from
   96: /// the hollows as well as the tops (the test
   97: /// `a_dry_rag_leaves_a_pale_tint_and_spirits_lift_nearly_to_the_ground`) [E].
   98: const DAMP_LIFT: f32 = 8.0;
   99: /// Spirits evaporate from a damp face as the painting goes on: half of what
  100: /// is left goes every `DAMP_HALF_MIN` minutes of painting time (the clock,
  101: /// hand time included), and below `DRY_DAMP` the face is dry, some 17
  102: /// minutes after a dip at 0.5. "Pour a few drops on a sheet of white
  103: /// writing paper; if it is pure the mark will evaporate in a few minutes"
  104: /// (W. J. Pearce [Jennings], Paint & Colour Mixing, 1902, "To Test the
  105: /// Purity of Turpentine",
  106: /// https://www.gutenberg.org/cache/epub/56738/pg56738-images.html); a
  107: /// bunched cloth holds more than a few drops and shields part of it, so
  108: /// it takes somewhat longer [E].
  109: pub const DAMP_HALF_MIN: f64 = 3.0;
  110: /// A face this little damp is dry [E].
  111: const DRY_DAMP: f32 = 0.01;
  112: /// How many faces a rag can be refolded to before none is clean: a cloth
  113: /// about 30 cm square [E].
  114: const FACES: f32 = 12.0;
```

### Lines 194–227

```text
  194:     /// Turn a cleaner, dry face outward. No face is cleaner than the paint
  195:     /// soaked through the whole cloth so far (`soaked`) leaves it. Counts
  196:     /// the hand time in `t`.
  197:     pub fn refold(&mut self, t: &mut Tally) {
  198:         self.fold = self.fold.wrapping_add(1);
  199:         self.cloth = None;
  200:         self.face = None;
  201:         self.load = self.soaked.clamp(0.0, 1.0);
  202:         self.damp = 0.0;
  203:         t.secs += pace::REFOLD;
  204:     }
  205: 
  206:     /// Dip the face in use into spirits, the reach starting at `now_min`
  207:     /// (`Canvas::now_min`): `amount` 0..1 (a light dip about 0.5). It is
  208:     /// that damp when the hand is back (`pace::DIP` later), and stays damp
  209:     /// until it is refolded or the spirits evaporate (`evaporate`). Counts
  210:     /// the hand time in `t`.
  211:     pub fn dip(&mut self, amount: f32, now_min: f64, t: &mut Tally) {
  212:         self.evaporate(now_min + pace::DIP / 60.0);
  213:         self.damp = self.damp.max(amount.clamp(0.0, 1.0));
  214:         t.secs += pace::DIP;
  215:     }
  216: 
  217:     /// The spirits in the face in use evaporated up to `now_min` minutes of
  218:     /// painting time (`Canvas::now_min`): `DAMP_HALF_MIN`.
  219:     pub fn evaporate(&mut self, now_min: f64) {
  220:         let dt = now_min - self.wet_at;
  221:         if dt > 0.0 && self.damp > 0.0 {
  222:             self.damp = (self.damp as f64 * (-dt * std::f64::consts::LN_2 / DAMP_HALF_MIN).exp()) as f32;
  223:             if self.damp < DRY_DAMP {
  224:                 self.damp = 0.0;
  225:             }
  226:         }
  227:         self.wet_at = self.wet_at.max(now_min);
```

### Lines 238–250

```text
  238:     /// How readily the face in use still takes paint (1 clean .. 0 full):
  239:     /// a half-loaded cloth still drinks; a nearly full one barely does.
  240:     fn thirst(&self) -> f32 {
  241:         1.0 - self.load.clamp(0.0, 1.0).powi(2)
  242:     }
  243: 
  244:     fn cloth_seed(&self, s: u64) -> u64 {
  245:         self.seed ^ (self.fold as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ s.wrapping_mul(0xD6E8_FEB8_6659_FD93)
  246:     }
  247: 
  248:     fn press_cloth(&mut self, mmu: f32, pressure: f32, travel: [f32; 2]) -> cloth::Contact {
  249:         let seed = self.cloth_seed(0);
  250:         self.cloth.get_or_insert_with(|| cloth::Cloth::new(self.width * mmu, seed)).press(pressure, travel)
```

### Lines 718–728

```text
  718:     pub fn rag_blot(&mut self, rag: &mut Rag, x: f32, y: f32, pressure: f32, seed: u64) -> f64 {
  719:         self.tally.rag_blot();
  720:         if self.engine >= 3 {
  721:             let mmu = self.mm_per_unit;
  722:             let contact = rag.press_cloth(mmu, pressure, [0.0, 0.0]);
  723:             let radius = contact.extent() / mmu;
  724:             return self.rag_contact(rag, (x - radius, y - radius, x + radius, y + radius), pressure, None, Some((&contact, [x, y], BLOT)), |px, py| {
  725:                 let (dx, dy) = (px - x, py - y);
  726:                 let pad = BLOT * (1.0 - smoothstep(0.75, 1.0, dx.hypot(dy) / radius));
  727:                 (pad, contact.at(dx * mmu, dy * mmu), 0.0)
  728:             });
```

## E07 — Thinner laws, estimates and integration limiter

Source: `wip/thinner/crates/paint/src/thinner.rs`  
SHA-256: `f772900571317662cbad7ae46c94e99ffe788657f07e57dd2802e95b545e0683`

### Lines 1–128

```text
    1: //! Thinner: solvent (turpentine, spirits) knifed into a pile (engine 3).
    2: //!
    3: //! A thinned pile is paint plus a share `t` of solvent by volume
    4: //! (`Paint::with_thinner`). On the brush the solvent rides with the paint
    5: //! (`Bristle::solvent`); laid, it sits in the open film beside the paint
    6: //! (`Wet::solv`, in µm) and leaves it over painting
    7: //! time. Paint is never mixed with solvent: pigment, oil, stiffness and the
    8: //! drying rate stay the paint's own, the film's optics and its oil cure
    9: //! read only the paint, and the solvent is a separate quantity beside it.
   10: //!
   11: //! Three things the solvent does, all engine 3, all estimates:
   12: //!
   13: //! - **A thinned stroke lays a thin film** (`stroke_limit_um`). One stroke
   14: //!   adds at most this much wet film (paint + solvent) to any pixel, a
   15: //!   ceiling shared by all the hairs and all the parts of the stroke; what
   16: //!   the brush can't lay stays on it. A thin, fluid liquid leaves a thinner
   17: //!   film behind a moving surface than a thick one: the film drawn up by a
   18: //!   moving plate grows with viscosity as (ηU)^(2/3) (Landau and Levich,
   19: //!   "Dragging of a liquid by a moving plate", Acta Physicochimica URSS 17,
   20: //!   42-54, 1942; a law for a plate drawn out of a liquid, used here only
   21: //!   qualitatively, not as a calibrated brush law). The model assumes that
   22: //!   turpentine, far more fluid than oil paint, makes the thinned paint
   23: //!   more fluid the more of it there is; no measurement of a paint's
   24: //!   viscosity against its thinner is used. The size of the ceiling is an
   25: //!   ESTIMATE, set so that raw
   26: //!   sienna thinned half lays an imprimatura (notes/thinner/RESULTS.md);
   27: //!   it fades out as thinner goes to 0, where today's paint path runs
   28: //!   unchanged.
   29: //! - **It evaporates** (`evaporation_tau_min`): at a fixed paint thickness
   30: //!   the same share leaves in each equal time step, solvent = s0 ×
   31: //!   exp(-t / τ). A few drops of pure turpentine on white writing paper
   32: //!   evaporate "in a few minutes" (Arthur Seymour Jennings, Paint & Colour
   33: //!   Mixing, 1902, "To Test the Purity of Turpentine", pp. 74-75,
   34: //!   https://www.gutenberg.org/cache/epub/56738/pg56738-images.html): a
   35: //!   purity test for the turpentine, not a measurement of its release from
   36: //!   an oil-paint film, so it only sets the scale of `TAU_MIN`. In a paint
   37: //!   film the solvent left in it has to diffuse out through the
   38: //!   film, so a thicker film holds it longer (C. M. Hansen, The Three
   39: //!   Dimensional Solubility Parameter and Solvent Diffusion Coefficient,
   40: //!   Danish Technical Press, 1967, §5.2: evaporation from the surface and
   41: //!   diffusion out of the film are separate stages, and a thicker film
   42: //!   releases its solvent more slowly). τ's numbers, and its linear growth
   43: //!   with thickness, are ESTIMATES: no source gives them for oil paint and
   44: //!   turpentine.
   45: //! - **It makes the wet paint flow** (`spread_mm2_min`): while it is there
   46: //!   the film levels, liquid running from higher to lower ground, carrying
   47: //!   paint and solvent in the proportions they have where it starts. A
   48: //!   surface-tension-driven film levels a ripple of wavelength λ at a rate
   49: //!   of about σh³(2π/λ)⁴ / 3η (S. E. Orchard, "On surface levelling in
   50: //!   viscous liquids and gels", Applied Scientific Research A 11, 451-464;
   51: //!   the publisher dates it July 1963, https://doi.org/10.1007/BF03184629,
   52: //!   while some secondary sources give 1962; notes/research/oil_paint_physics.md).
   53: //!   The engine uses a diffusion of the wet surface with one mobility, the
   54: //!   ESTIMATE `SPREAD_MM2_MIN` at thinner 0.5, that leaves a wetting film
   55: //!   (`WET_FILM_UM`) where it runs off. Matching Orchard's decay at one
   56: //!   wavelength, D = σh³(2π/λ)² / 3η, with these inputs, all ESTIMATES
   57: //!   (none measured for this paint): surface tension σ = 0.03 N/m, film
   58: //!   h = 10 µm, viscosity η = 0.1 Pa·s (paint thinned half; the note gives
   59: //!   1 Pa·s for a medium-rich glaze) and wavelength λ = 2 mm, gives about
   60: //!   0.06 mm²/min. That is a wavelength-specific estimate, not a measured
   61: //!   mobility. Unthinned paint doesn't flow here, as before (it levels when
   62: //!   it sets, `drying`).
   63: //!
   64: //! Not modeled, on purpose: solvent evaporating from the brush or the
   65: //! palette pile (the pile keeps its share), solvent soaking into the
   66: //! ground, solvent dissolving set or dry paint underneath, and solvent-wet
   67: //! paint coming up more readily on a brush or rag than the same paint
   68: //! without it (extra pickup). The brush and the rag take solvent with the
   69: //! paint they lift, in the film's own proportions.
   70: 
   71: /// One stroke of paint thinned half (`t` = 0.5) adds at most this much wet
   72: /// film (µm) to a pixel. ESTIMATE, set before measuring; the card keeps 84%
   73: /// (load 0.3) and 81% (load 0.6) of its contrast with it, and from 6 to 36
   74: /// µm it kept 62-89% (notes/thinner/RESULTS.md, the sweep).
   75: pub const STROKE_FILM_UM: f32 = 6.0;
   76: 
   77: /// The most wet film (paint + solvent, µm) one stroke may add to a pixel,
   78: /// for paint holding the share `t` of solvent: `STROKE_FILM_UM` at 0.5,
   79: /// growing without bound as `t` → 0 (`f32::INFINITY` at 0: no ceiling, the
   80: /// paste law alone), shrinking as `t` → 1. Continuous: a hundredth of
   81: /// thinner barely changes a stroke.
   82: pub fn stroke_limit_um(t: f32) -> f32 {
   83:     if t.is_nan() || t <= 0.0 {
   84:         return f32::INFINITY;
   85:     }
   86:     let t = t.min(0.95);
   87:     STROKE_FILM_UM * (1.0 - t) / t
   88: }
   89: 
   90: /// Evaporation time (minutes) of the solvent in a film too thin to hold it
   91: /// back. ESTIMATE ("a few minutes", Jennings 1902).
   92: pub const TAU_MIN: f64 = 2.0;
   93: /// The film thickness (µm of paint) that doubles it. ESTIMATE, with no
   94: /// source for its size. First set at 20 µm; at that, thinned paint flowing
   95: /// into the weave's hollows during the wait deepens the deepest film enough
   96: /// (77 → 81 µm on the card) that the time the card test waits (ten times τ
   97: /// after the pass) falls short of ten times τ at the end. 100 µm keeps the
   98: /// dependence (a thicker film still holds its solvent longer) with less
   99: /// sensitivity; notes/thinner/RESULTS.md.
  100: pub const TAU_DOUBLING_UM: f64 = 100.0;
  101: 
  102: /// The solvent's evaporation time (minutes) at a pixel holding `paint_um`
  103: /// of paint (solvent-free): over `dt` minutes, `exp(-dt / τ)` of it stays.
  104: /// Positive, finite, never shorter for a thicker film.
  105: pub fn evaporation_tau_min(paint_um: f32) -> f64 {
  106:     let h = if paint_um.is_finite() { paint_um.max(0.0) as f64 } else { 0.0 };
  107:     TAU_MIN * (1.0 + h / TAU_DOUBLING_UM)
  108: }
  109: 
  110: /// Mobility (mm²/min) of a wet film half solvent: how fast it levels.
  111: /// ESTIMATE (Orchard 1963, see the module notes).
  112: pub const SPREAD_MM2_MIN: f32 = 0.06;
  113: /// The flow doesn't drain a pixel below this much liquid (µm): a liquid
  114: /// that wets the paint under it leaves a film on the weave's tops, it
  115: /// doesn't run off them bare (Orchard's leveling rate goes as the film's
  116: /// thickness cubed, so the last of a film barely moves). ESTIMATE.
  117: pub const WET_FILM_UM: f32 = 2.0;
  118: 
  119: /// Mobility of a film whose liquid holds the share `phi` of solvent: 0
  120: /// without solvent, `SPREAD_MM2_MIN` at one half, more the thinner it is
  121: /// (viscosity falls steeply with solvent). ESTIMATE.
  122: pub fn spread_mm2_min(phi: f32) -> f32 {
  123:     if phi.is_nan() || phi <= 0.0 {
  124:         return 0.0;
  125:     }
  126:     let phi = phi.min(0.95);
  127:     SPREAD_MM2_MIN * phi / (1.0 - phi)
  128: }
```

### Lines 139–147

```text
  139: /// Most substeps of the flow in one step of the clock; past it the flow
  140: /// is slowed to stay stable (a film that thin and that fine-grained flows a
  141: /// little less far per minute than its mobility says). A chosen numerical
  142: /// cutoff, not a physical constant, as are `MAX_OUT` and the explicit
  143: /// scheme's stability factor 0.2 in `spread`.
  144: const MAX_SUBSTEPS: usize = 64;
  145: /// Most of a pixel's liquid that can leave it in one substep. A chosen
  146: /// numerical cutoff.
  147: const MAX_OUT: f32 = 0.5;
```

### Lines 176–225

```text
  176:     /// Solvent-wet paint flows for `dt` minutes: the wet surface (relief +
  177:     /// paint + solvent) levels by diffusion, each pixel's liquid running to
  178:     /// lower neighbors at its own mobility (`spread_mm2_min` of its solvent
  179:     /// share, slowed as its oil approaches the gel point, `drying::fluid`).
  180:     /// What moves carries the paint (pigment, scattering, stiffness, drying
  181:     /// rate, cure) and the solvent of the pixel it leaves, in their
  182:     /// proportions there; paint without solvent doesn't flow out. Each
  183:     /// substep computes every pixel's outflow from the state before it,
  184:     /// then every pixel gathers its inflow: the result doesn't depend on
  185:     /// the order pixels are visited in (or the threads).
  186:     pub(crate) fn spread(&mut self, dt: f32) {
  187:         let Some((bx0, by0, bx1, by1)) = self.wet.dirty else { return };
  188:         let (w, h) = (self.f.w, self.f.h);
  189:         let dx = self.px_mm();
  190:         let timed = self.wet.clock.px.len() == w * h;
  191:         // the mobility (mm²/min) of each pixel in the dirty box
  192:         // (the canvas holds solvent in µm; the flow works in coats)
  193:         let mob = |wet: &crate::wet::Wet, i: usize| -> f32 {
  194:             let (v, s) = (wet.vol[i], wet.solv[i] / COAT_UM);
  195:             if s <= 0.0 || v + s <= 0.0 {
  196:                 return 0.0;
  197:             }
  198:             let fl = if timed { crate::drying::fluid(wet.clock.px[i].cure) } else { 1.0 };
  199:             spread_mm2_min(s / (v + s)) * fl
  200:         };
  201:         let mut m_max = 0.0f32;
  202:         for y in by0..by1.min(h) {
  203:             for x in bx0..bx1.min(w) {
  204:                 m_max = m_max.max(mob(&self.wet, y * w + x));
  205:             }
  206:         }
  207:         if m_max <= 0.0 {
  208:             return;
  209:         }
  210:         let r_total = m_max * dt / (dx * dx);
  211:         let n = ((r_total / 0.2).ceil() as usize).clamp(1, MAX_SUBSTEPS);
  212:         // the flow's rate per substep, per mm²/min of mobility
  213:         let k = (dt / n as f32 / (dx * dx)).min(0.2 / m_max.max(1e-12));
  214:         let (mut x0, mut y0, mut x1, mut y1) = (bx0, by0, bx1.min(w), by1.min(h));
  215:         for _ in 0..n {
  216:             // liquid can reach one pixel further each substep
  217:             (x0, y0, x1, y1) = (x0.saturating_sub(1), y0.saturating_sub(1), (x1 + 1).min(w), (y1 + 1).min(h));
  218:             let (rw, rh) = (x1 - x0, y1 - y0);
  219:             let wet = &self.wet;
  220:             let height = &self.height;
  221:             // pass 1: each pixel's outflow (coats of liquid) to its four
  222:             // neighbors (left, right, up, down)
  223:             let out: Vec<[f32; 4]> = (0..rw * rh)
  224:                 .into_par_iter()
  225:                 .map(|k2| {
```

## E08 — Thinner rim experiments and fixtures

Source: `wip/look/notes/look/REPORT.md`  
SHA-256: `643e239dbc6feef5713920e05adcde0132f2dc7a8257e6342dffa5eb1b36ee5d`

### Lines 104–205

```text
  104: ## 2. The thinned stroke's rim
  105: 
  106: **The cause, verified.** The scene is the one of `single_stroke_edge.jpg`
  107: (`crates/paint/tests/look_rim.rs`). With the plough and the pickup both
  108: off (a diagnostic setting, not committed), a single thinned stroke is a
  109: flat 3.0 µm from edge to edge, which is exactly its ceiling. With only the
  110: plough off, it is 2.7 µm at the edges and 2.0 µm inside. With only the
  111: pickup off, it is 2.4 and 1.4 µm. So the lead's hypothesis holds:
  112: 
  113: - the plough moves paint from the middle to the edges and accounts for
  114:   most of the rim;
  115: - the pickup adds a little more, since the edge hairs touch less and so
  116:   lift less.
  117: 
  118: The ceiling counted only paint the stroke added, so neither the paint
  119: taken nor the paint pushed was held back by it.
  120: 
  121: **What changed:**
  122: 
  123: - **(a):** the plough is scaled by the share of paint in the liquid
  124:   (× 0.5 at thinner 0.5).
  125: - **(b):** a thinned stroke leaves a pixel holding at most its ceiling
  126:   more than it held when the stroke reached it. Paint lifted out makes
  127:   room again, and paint ploughed in counts against the ceiling.
  128: - **(B), absolute:** a pixel holds at most the larger of the ceiling and
  129:   what it held before.
  130: 
  131: Unthinned paint never takes any of these paths: the baseline scenes
  132: without a rag are equal on every field, and the unthinned stroke's
  133: numbers are unchanged.
  134: 
  135: | variant | stroke edges / inside, just after (µm) | after 30 min (µm) | broad pass: paint, unevenness (spread ÷ mean) | card kept, load 0.3 / 0.6 | thinner acceptance (27 required) |
  136: |---|---|---|---|---|---|
  137: | today | 2.25, 2.13 / 0.96 | 2.03, 1.93 / 0.96 | 2.8 µm, 0.47 | 84.4% / 81.4% | all pass |
  138: | (a) | 1.56, 1.39 / 1.35 | 1.51, 1.39 / 1.35 | 2.9 µm, 0.47 | 83.5% / 80.3% | 26 pass; **check 15 fails** |
  139: | (a), plough × (1 − share)² | 2.06, 1.95 / 1.60 | 2.02, 1.87 / 1.61 | 2.9 µm, 0.47 | 82.9% / 79.7% | 25 pass; checks 15 and 16 fail |
  140: | **(b), kept** | 3.00, 2.99 / 2.97 | 2.92, 2.98 / 2.97 | 7.7 µm, 0.49 | 73.5% / 71.7% | **all pass** |
  141: | (a) + (b) | 2.82, 2.94 / 2.98 | 2.77, 2.87 / 2.98 | 7.4 µm, 0.49 | 74.1% / 72.3% | all pass |
  142: | (B), reverted | 3.00, 2.99 / 2.97 | 2.92, 2.98 / 2.97 | 2.9 µm, **0.13** | 83.5% / 83.2% | 23 pass; **checks 7, 9, 15, 16 fail** |
  143: 
  144: In every row, check 13 (b) fails as before (the siennas, above). "All
  145: pass" means every required test passes and only that known failure
  146: remains. The (a)+(b) and the squared-plough rows were measured with
  147: temporary switches (environment variables) in a single build and were not
  148: committed. The committed (a) and (b) are the same code without the
  149: switch.
  150: 
  151: **What the failing checks say:**
  152: 
  153: - **Check 15** (solvent doesn't slow the oil cure) needs at least 100
  154:   pixels holding 12 µm or more after five thinned passes:
  155:   - (a) leaves 79, and the squared plough 46;
  156:   - (B) leaves none.
  157: 
  158:   With less plough, less paint heaps up into thick spots. Nothing about
  159:   drying went wrong; the test no longer finds the thick film it measures.
  160: - **Check 16** (spreading mixes the liquids at a boundary) needs at least
  161:   20 boundary pixels to gain paint. It got 3 (squared plough) and 2 (B).
  162:   Again, the film it relies on isn't there.
  163: - **Check 7** (a second thinned pass adds at least 20% more paint than one)
  164:   fails under (B): 20,379 against 23,187, only 14% more.
  165: - **Check 9** (five passes are thicker than one) fails under (B): the
  166:   median is 3.0 µm for both.
  167: 
  168: (B) contradicts the agreed rule that each thinned pass builds on the last.
  169: That's why it is reverted.
  170: 
  171: **What you see** (`thinned_strokes.jpg`):
  172: 
  173: - **(a):** strokes are still outlined, a little more faintly.
  174: - **(b):** a single stroke is now an even tone with no rim. The broad pass
  175:   loses its outlined slivers, but now reads as blotchy. Where strokes
  176:   overlap, each lays its own ceiling, so the pass holds 7.7 µm and is
  177:   darker. Its unevenness is about the same as today's, now as overlaps
  178:   rather than outlines.
  179: - **(B):** the broad pass becomes one even veil, but only by breaking the
  180:   build-up rule.
  181: 
  182: **About the card.** Under (b) it keeps 73.5% and 71.7%, down from 84.4%
  183: and 81.4%, because the passes now stack. That is still above the half its
  184: test name asks for.
  185: 
  186: **Recommendation:** keep (b). It removes the cause of the rim and passes
  187: everything. Today's thin 2.8 µm broad pass existed partly because the
  188: plough and pickup kept removing earlier strokes' paint. If the stacking
  189: looks too dark or blotchy, a smaller `STROKE_FILM_UM` is the knob; that's
  190: a separate, small sweep I didn't run. Don't take (a): with (b) in place it
  191: changes nothing, and alone it fails check 15.
  192: 
  193: ## 3. The rag, less tidy (`crates/paint/src/rag.rs`, engine 3 only)
  194: 
  195: New constants are marked [E] (estimates), as the file does.
  196: 
  197: 1. **Folds that miss** (`FOLD_MISS` 0.25..0.5, `FOLD_PRESS` 0.2): the
  198:    cloth's contact goes from 0 in the gaps between creases to 1.
  199:    - About a tenth of the pad misses at pressure 0.5, and about 3% at 0.9.
  200:    - The mean contact is 0.74, close to today's 0.75.
  201:    - My first try, contact down to 0 over a much wider range of the crease
  202:      noise, removed only a third of the paint. Its numbers are in the
  203:      first run, not shown here.
  204: 2. **Streaks survive a damp cloth.**
  205:    - The cloth's contact now multiplies the share taken after the pad's
```

## E08b — Net ceiling implementation

Source: `wip/look/crates/paint/src/bristle.rs`  
SHA-256: `01cfbb6698b38227728b7885e458f9c38c003c531d0b36932603048eec82d80e`

### Lines 1478–1489

```text
 1478:                 }
 1479:                 let i = (y - oy) * bw_buf + x - ox;
 1480:                 // a thinned stroke's ceiling is on the wet film a pixel holds
 1481:                 // when the stroke is done, against what it held when the
 1482:                 // stroke first reached it (`Wet::laid`): what the hairs and
 1483:                 // the plough lift from a pixel is room for paint again, and
 1484:                 // paint ploughed into a pixel counts against its ceiling as
 1485:                 // laid paint does
 1486:                 if capped && *sf.laid_id.add(i) != id {
 1487:                     *sf.laid_id.add(i) = id;
 1488:                     *sf.laid.add(i) = *sf.vol.add(i) + *sf.solv.add(i) / COAT_UM;
 1489:                 }
```

### Lines 1535–1556

```text
 1535:                     sf.add(i, dep_per_w * wt, &blat, bhide, bcure);
 1536:                     *sf.stroke.add(i) = id;
 1537:                 } else if dep_per_w > 0.0 {
 1538:                     // a thinned load lays liquid: no more than what is left
 1539:                     // of this pixel's ceiling for this stroke, shared by all
 1540:                     // its hairs (crate::thinner); the rest stays on the hair
 1541:                     let mut d = dep_per_w * wt;
 1542:                     if capped {
 1543:                         let now = *sf.vol.add(i) + *sf.solv.add(i) / COAT_UM;
 1544:                         d = d.min((cap - (now - *sf.laid.add(i))).max(0.0));
 1545:                     }
 1546:                     if d > 0.0 {
 1547:                         let cv = &mut *sf.cover.add(i);
 1548:                         *cv = if fine { ((if *sf.vol.add(i) < 1e-6 { 0.0 } else { *cv }) + wt * excl).min(1.0) } else { 1.0 };
 1549:                         sf.add(i, d * (1.0 - phi), &blat, bhide, bcure);
 1550:                         if !sf.solv.is_null() {
 1551:                             *sf.solv.add(i) += d * phi * COAT_UM;
 1552:                         }
 1553:                         *sf.stroke.add(i) = id;
 1554:                         laid_liq += d * px_area;
 1555:                     }
 1556:                 }
```

### Lines 1598–1621

```text
 1598:                                 // a clipped stroke can't push paint past its mask:
 1599:                                 // only the accepted share moves, the rest stays
 1600:                                 let m = m * share * clip.map_or(1.0, |c| c.at(ty * w + tx).0);
 1601:                                 // a thinned stroke adds no more wet film to the
 1602:                                 // pixel it ploughs into than its ceiling there
 1603:                                 // allows; the solvent goes with the paint
 1604:                                 // (solvent in µm, as the canvas holds it)
 1605:                                 let sol = if sf.solv.is_null() { 0.0 } else { *sf.solv.add(i) / COAT_UM };
 1606:                                 let vi = *sf.vol.add(i);
 1607:                                 let ms = if sol > 0.0 && vi > 0.0 { (sol * m / vi).min(sol) } else { 0.0 };
 1608:                                 let (m, ms) = if capped && j != i && m > 0.0 {
 1609:                                     if *sf.laid_id.add(j) != id {
 1610:                                         *sf.laid_id.add(j) = id;
 1611:                                         *sf.laid.add(j) = *sf.vol.add(j) + *sf.solv.add(j) / COAT_UM;
 1612:                                     }
 1613:                                     let now = *sf.vol.add(j) + *sf.solv.add(j) / COAT_UM;
 1614:                                     let room = (cap - (now - *sf.laid.add(j))).max(0.0);
 1615:                                     let f = if m + ms > room { room / (m + ms) } else { 1.0 };
 1616:                                     (m * f, ms * f)
 1617:                                 } else {
 1618:                                     (m, ms)
 1619:                                 };
 1620:                                 if j != i && m > 0.0 {
 1621:                                     if ms > 0.0 {
```

## E09 — Absolute-minute flow clock

Source: `wip/thinner/crates/paint/src/drying.rs`  
SHA-256: `f4d0df02422cf403f4566fe96444b21a72144c3c2d897122e44060032be50531`

### Lines 376–429

```text
  376:     pub fn wait(&mut self, minutes: f32) {
  377:         let dt = minutes.max(0.0);
  378:         assert!(dt.is_finite(), "Canvas::wait: minutes must be finite (got {minutes}); use dry() to wait until touch-dry");
  379:         if self.engine >= 3 && self.wet.has_solvent(self.f.w) {
  380:             self.wait_on_grid(dt);
  381:             return;
  382:         }
  383:         self.age(dt);
  384:         self.wet.clock.now += dt as f64;
  385:     }
  386: 
  387:     /// Wait `dt` minutes with solvent in the paint (engine 3,
  388:     /// `crate::thinner`): the solvent's loss, the flow it gives the paint
  389:     /// and the oil's drying all step on one clock, in steps that end on
  390:     /// whole minutes counted from the canvas's start (not from this wait),
  391:     /// so a wait split on whole minutes is exactly the wait in one, and
  392:     /// brushwork's hand time (which waits too) keeps the same grid. The
  393:     /// solvent's loss and the oil's drying run over every step, whole or
  394:     /// part (both exact in closed form); the flow runs once per whole
  395:     /// minute, as each minute of the grid ends. Once the last solvent is
  396:     /// gone the rest of the wait is the ordinary one.
  397:     fn wait_on_grid(&mut self, dt: f32) {
  398:         let start = self.wet.clock.now;
  399:         let end = start + dt as f64;
  400:         let mut t = start;
  401:         while t < end {
  402:             let next = (t.floor() + 1.0).min(end);
  403:             let step = (next - t) as f32;
  404:             self.evaporate(step);
  405:             self.age(step);
  406:             if next == next.floor() {
  407:                 self.spread(1.0);
  408:             }
  409:             t = next;
  410:             self.wet.clock.now = t;
  411:             if !self.wet.has_solvent(self.f.w) {
  412:                 if end > t {
  413:                     self.age((end - t) as f32);
  414:                 }
  415:                 break;
  416:             }
  417:         }
  418:         self.wet.clock.now = end;
  419:     }
  420: 
  421:     /// `wait` on the minute grid until no solvent is left: to the next
  422:     /// whole minute, then a whole minute at a time. Ends: each minute every
  423:     /// pixel keeps at most exp(-1 / τ) of its solvent, τ finite, and below
  424:     /// `SOLVENT_FLOOR` it is gone.
  425:     fn wait_out_solvent(&mut self) {
  426:         while self.wet.has_solvent(self.f.w) {
  427:             let now = self.wet.clock.now;
  428:             let step = (now.floor() + 1.0 - now) as f32;
  429:             self.wait_on_grid(if step > 0.0 { step } else { 1.0 });
```

## E09b — Recorded thinner verdict

Source: `wip/thinner/notes/thinner/RESULTS.md`  
SHA-256: `376a14e6a5674aec8f0ee34d4d851a1ff75eb30e5e3af6551059d4b13b7cc96d`

### Lines 1–85

```text
    1: # Thinner: results
    2: 
    3: The thinner is implemented in engine 3 on branch `thinner2` (not pushed).
    4: 
    5: **Final state.** The final acceptance run (`--all`, below) passes all 27
    6: required tests, the card included. Its one failure is check 13 (b), the
    7: pre-existing finding left to the user, so it exits 3 ("NOT ALL GREEN").
    8: The tests are the frozen set at 48be56a: the round-3 approval (b2a0e14)
    9: plus the corrections approved in rounds 4 and 5
   10: (`~/src/a/claude-paint-reviews/thinner-tests-review-astra-r5.md`;
   11: TESTS_PHASE_REPORT.md). The protected baseline (`notes/thinner/baseline/`)
   12: is untouched since a97c3a6. The dumpers gained one field, under a new
   13: name.
   14: 
   15: **The card target is met at 2400 px**: thinned raw sienna keeps 84.4% of
   16: the card's contrast at load 0.3 and 81.4% at load 0.6. Today, unthinned,
   17: it keeps 13% and 6% (DIAGNOSIS.md).
   18: 
   19: | commit | what |
   20: |---|---|
   21: | df9058c | the thinner: solvent beside the paint, the per-stroke ceiling, evaporation on the minute grid, the flow, PAINTCK9, Lua, dumper field |
   22: | 18548cf | τ doubles at 100 µm, not 20 (the card's wait guard); the guide's Thinner section |
   23: | afc7898 | the flow leaves a wetting film (the 2400 px lattice defect); `scripts/thinner_sheet` |
   24: | 1e27682 | the first RESULTS.md, sheets, logs |
   25: | 0a661f6 | a brush's `Debug` text (the `brushes=` digest, the dump's `brushes.debug`) is af49348's byte for byte before engine 3 |
   26: | e135129 | round 4 test corrections: c03 ×2, c18, c04, c05 (approved) |
   27: | 4c15082, ffef02e | the rag study re-rendered after afc7898; RESULTS.md |
   28: | 48be56a | round 5 test corrections: c16, the rag sheet's directory (approved; the frozen set) |
   29: | 7687ad3 | τ disclosure; `weave_or_brush.jpg` |
   30: | de5557c | the first final run; `single_stroke_edge.jpg`; the mutation sources |
   31: | (this commit) | the final review's fixes (`~/src/a/claude-paint-reviews/thinner-final-review-astra.md`): sources corrected, thinned paint refused before engine 3 in the Rust API, `dry()` waits the solvent out, stroke-ceiling reset on id wrap; three unit tests; the final runs again; this file |
   32: 
   33: The thinner's code last changed in this commit. Between 0a661f6 and
   34: it, commits changed only tests, scripts or notes.
   35: 
   36: **The final review's fixes:**
   37: 
   38: - **Sources** (finding 1). *Paint & Colour Mixing* is by Arthur Seymour
   39:   Jennings, and its "few minutes" is a purity test: drops of turpentine
   40:   on writing paper, pp. 74-75. It sets the scale of τ only. Orchard's
   41:   paper is dated July 1963 by its publisher (some secondary sources say
   42:   1962). "Orders of magnitude" is replaced by the model's qualitative
   43:   assumption. The σ, h, η and λ behind the mobility are labeled
   44:   estimates, and `SOLVENT_FLOOR`, `MAX_SUBSTEPS`, `MAX_OUT` and the 0.2
   45:   stability factor are labeled chosen numerical cutoffs. The guide says
   46:   "a thinner film", not "thinner, leaner". The same misattribution in
   47:   af49348's `rag.rs` comment is corrected too.
   48: - **Thinned paint before engine 3** (finding 2). `Canvas::drag`, `touch`,
   49:   `work` and `stipple` panic before drawing when a thinned brush or
   50:   piled pass meets an engine-1/2 canvas, rather than let its solvent
   51:   vanish. Lua already refused it. Unthinned paint never reaches the
   52:   check's message.
   53: - **`dry()` with solvent** (finding 2). On engine 3 it now runs the clock
   54:   on whole minutes, as `wait` does, until the solvent has gone, and only
   55:   then takes the bake shortcut. Before, it baked the film as it lay.
   56:   Without solvent, it is unchanged.
   57: - **Stroke ids wrapping** (finding 3). `next_stroke_ids` forgets every
   58:   stroke's ceiling when the ids wrap.
   59: - **Tests** (`crates/paint/src/thinner.rs`, `mod tests`, in the paint
   60:   lib). Each failed before its fix, run with that fix taken out:
   61:   - a thinned brush or pass is refused on engines 1 and 2 by all four
   62:     entry points, with nothing drawn (before the fix: no panic);
   63:   - `dry()` equals waiting the solvent out and then drying (before the
   64:     fix: a different picture);
   65:   - a stroke under a wrapped, reused id lays at least 90% of what it
   66:     lays under a fresh id (before the fix: −83 against 318; after: 302;
   67:     the gap left is `Wet::stroke`'s reused id, which predates the
   68:     thinner).
   69: 
   70: ## The final runs
   71: 
   72: Every command ran through `~/src/a/claude-paint-tools/lockrun` on this
   73: machine (rustc 1.97.1, release), at this commit's code and the frozen
   74: tests (48be56a).
   75: 
   76: | command | result | exit | time | log (sha256) |
   77: |---|---|---|---|---|
   78: | `THINNER_RAG_STUDY_DIR=<temp dir> scripts/test_thinner_acceptance --all` (lockrun `--timeout 600`, as is every command here) | 27 of 27 required tests pass; 13 (b) the expected failure | **3** (NOT ALL GREEN) | 78 s | `logs/all_final.txt` (`0fa5387a…6f05`) |
   79: | `scripts/test_thinner_acceptance --quick` | 22 of 22 required pass; 13 (b) the expected failure | **3** | 100 s (with the build) | `logs/quick_final.txt` (`1b9b8cc5…6ce6`) |
   80: | `cargo test --release -p paint --lib` (existing tests and the three new thinner unit tests) | 183 passed, 0 failed, 8 ignored | 0 | 141 s | `logs/paint_lib_final.txt` (`487226df…a61b`) |
   81: | `cargo test --release -p easel --bin easel -- --skip thinner_tests::` (existing tests) | 71 passed, 0 failed, 1 ignored | 0 | 56 s | `logs/easel_bin_final.txt` (`6efd6d5f…5b88`) |
   82: | `cargo test --release -p easel --test determinism --test session_integrity`, then `-p paint --test curved_drag_nan --test ground_grain` | 4 + 10 + 2 + 1 passed, 0 failed (ground_grain's 3 ignored as before) | 0, 0 | 82 s + 5 s | `logs/integration_final.txt` (`b20ae4d8…a523`) |
   83: | speed's `scripts/tests/old_logs.sh <this build's easel>`, read-only from `~/src/a/claude-paint-speed` | all 11 cases replay as af49348 did (PNGs and per-chunk digests) | 0 | 6 s | `logs/old_logs_final.txt` (`9b5e98af…9e08`) |
   84: 
   85: `--all` runs the card. The earlier separate `--card` run
```

## E10 — Retained diagnostic measurements

Source: `notes/rag/contact-review/measurements.json`  
SHA-256: `23e260310f1ba8d5f8b61b0551ea2ff5f53c908a7b662cd74346e44e25a61532`

### Lines 1–26

```text
    1: {
    2:   "baseline": {
    3:     "initial-film": 69.2941088259772,
    4:     "film": 8.579133468892794,
    5:     "contact": 0.7014544272947666,
    6:     "pickup": 66.94016585499048,
    7:     "deposit": 6.225191432067458,
    8:     "film_left_percent": 12.380754459860547
    9:   },
   10:   "uniform": {
   11:     "initial-film": 69.29410552978516,
   12:     "film": 8.618402190581193,
   13:     "contact": 0.7014544272947666,
   14:     "pickup": 66.8640798026397,
   15:     "deposit": 6.188375937924893,
   16:     "film_left_percent": 12.43742469101746
   17:   },
   18:   "no-plough": {
   19:     "initial-film": 70.39180568936399,
   20:     "film": 8.753185349647646,
   21:     "contact": 0.7014544272947666,
   22:     "pickup": 67.97268770980752,
   23:     "deposit": 6.334068138952609,
   24:     "film_left_percent": 12.43494930116593
   25:   }
   26: }
```

## E11 — Sienna metrics and recorded results

Source: `wip/look/notes/look/REPORT.md`  
SHA-256: `643e239dbc6feef5713920e05adcde0132f2dc7a8257e6342dffa5eb1b36ee5d`

### Lines 41–102

```text
   41: ## 1. Siennas: which is more see-through
   42: 
   43: Measured by `crates/paint/tests/look_sienna.rs` (ignored; run with
   44: `--ignored --nocapture`); the output is in `logs/sienna.txt`. It uses the
   45: same card as the burnt-against-raw check (thinner check 13 (b)), which is
   46: unchanged.
   47: 
   48: **The standard measure.** The contrast ratio is the paint's reflectance
   49: over black divided by its reflectance over white, for the same film
   50: (ASTM D2805 / ISO 6504-3, https://en.wikipedia.org/wiki/Hiding_power).
   51: Lower means more see-through.
   52: 
   53: **By the standard ratio, burnt sienna is the more see-through of the two,
   54: at every thickness, thinned or not.** By today's difference measure
   55: (how much of the card's black-to-white difference still shows), raw comes
   56: out ahead. The two measures disagree, and the check uses the difference.
   57: 
   58: Equal uniform films, the engine's own optics. The solvent has no color, so
   59: thinned and unthinned paint look the same at an equal paint film. The
   60: chart is black and 80% white:
   61: 
   62: | paint film | raw: ratio (luminance) | burnt: ratio (luminance) | raw: difference kept | burnt: difference kept |
   63: |---|---|---|---|---|
   64: | 3 µm | 0.047 | **0.035** | 78.8% | 66.5% |
   65: | 10 µm | 0.169 | **0.154** | 49.7% | 30.6% |
   66: | 25 µm (one coat) | 0.452 | **0.438** | 20.5% | 10.5% |
   67: | 30 µm | 0.537 | **0.512** | 15.6% | 8.2% |
   68: 
   69: Per channel at 10 µm (red, green, blue), the two differ by hue:
   70: 
   71: - raw: 0.143, 0.177, 0.798;
   72: - burnt: 0.104, 0.192, 0.661.
   73: 
   74: Burnt is lower in red and blue, higher in green.
   75: 
   76: On the card itself (twenty strokes at load 0.5, the check's own scene):
   77: 
   78: | | paint film | ratio (luminance) | difference kept |
   79: |---|---|---|---|
   80: | unthinned, raw | 37.9 µm | 0.500 | 17.9% |
   81: | unthinned, burnt | 37.9 µm | **0.464** | 9.8% |
   82: | thinned 0.5, raw | 1.64 µm | 0.040 | 87.3% |
   83: | thinned 0.5, burnt | 1.64 µm | **0.033** | 79.7% |
   84: 
   85: Scattering per coat: raw 0.296, burnt 0.199. The tubes' own one-coat
   86: hiding is raw 0.40, burnt 0.45. That figure is a ratio taken on the
   87: masstone alone, not over a real black and white; over them, as above,
   88: burnt's ratio is lower.
   89: 
   90: **Why they disagree.** Burnt sienna is much darker (masstone luminance
   91: 0.080 against 0.174), so it darkens the white band more. That shrinks the
   92: black-to-white difference even though, relative to its own brightness,
   93: it lets more of the black show.
   94: 
   95: `siennas.jpg` shows the same thin film of each (thinned 0.5) over light,
   96: mid-grey and dark bands: raw reads as a yellow veil on the light band,
   97: and burnt as a warmer, darker one.
   98: 
   99: **Recommendation.** Leave the tubes alone. If you agree, check 13 (b)
  100: should be restated in terms of the contrast ratio: by that measure burnt
  101: sienna already passes ("more transparent than raw", Field/Salter). I
  102: haven't changed the test; that's your call.
```

## E11b — Sienna test distinguishes substrates

Source: `wip/look/crates/paint/tests/look_sienna.rs`  
SHA-256: `4399eed9152223fd37126e3c988a64a90115416815d823db9a7ec61d1ec91eae`

### Lines 67–110

```text
   67: 
   68: #[test]
   69: #[ignore = "a measurement for notes/look"]
   70: fn siennas_by_contrast_ratio() {
   71:     let c0 = card();
   72:     let (kb, kw) = (mean_rgb(&c0, BLACK), mean_rgb(&c0, WHITE));
   73:     let diff0 = luminance(kw) - luminance(kb);
   74:     println!("card bands: black Y {:.4}, white Y {:.4}", luminance(kb), luminance(kw));
   75:     for name in ["raw sienna", "burnt sienna"] {
   76:         let p = sargent(name);
   77:         println!("{name}: masstone Y {:.4}, scattering {:.4} per coat, hiding of one coat {:.4}", luminance(p.color), p.scatter, p.hiding());
   78:     }
   79:     // (25 µm is one coat: the tubes' `hiding` is the contrast ratio of one coat on luminance alone)
   80:     println!("\n(1) a uniform film, the engine's own optics (Kubelka-Munk), thinned or not alike (the solvent has no color):");
   81:     for um in [3.0f32, 10.0, 25.0, 30.0] {
   82:         let x = um / paint::COAT_UM;
   83:         for name in ["raw sienna", "burnt sienna"] {
   84:             let p = sargent(name);
   85:             let pig = Pigment::masstone(p.color, p.scatter);
   86:             // the standard chart: black (0) and a white of 80% reflectance
   87:             let (b, w) = (pig.over([0.0; 3], x), pig.over([0.8; 3], x));
   88:             // the card's own black and white bands
   89:             let (cb, cw) = (pig.over(kb, x), pig.over(kw, x));
   90:             let kept = (luminance(cw) - luminance(cb)) / diff0;
   91:             println!("  {um:>4} µm {name:<12}: contrast ratio on a black/80% white chart {}; on the card's bands {}; difference kept on the card {:.1}%", ratio(b, w), ratio(cb, cw), 100.0 * kept);
   92:         }
   93:     }
   94:     println!("\n(2) the card painted as check 13 (b) paints it, twenty strokes at load 0.5:");
   95:     for t in [0.0f32, 0.5] {
   96:         for name in ["raw sienna", "burnt sienna"] {
   97:             let p = sargent(name);
   98:             let (b, w, um) = painted(c0.clone(), if t > 0.0 { p.with_thinner(t) } else { p });
   99:             println!("  thinner {t}: {name:<12} mean paint {um:.2} µm; contrast ratio {}; difference kept {:.1}%", ratio(b, w), 100.0 * (luminance(w) - luminance(b)) / diff0);
  100:         }
  101:     }
  102: }
  103: 
  104: /// The two siennas, the same thin film (thinner 0.5, load 0.5, the same
  105: /// strokes), over light, mid-grey and dark bands: one 256 px panel each,
  106: /// written to $LOOK_OUT as sienna_raw.png and sienna_burnt.png.
  107: #[test]
  108: #[ignore = "a picture for notes/look"]
  109: fn siennas_side_by_side() {
  110:     let out = std::env::var("LOOK_OUT").expect("LOOK_OUT");
```

## E11c — Scalar versus RGB optical parameterization

Source: `wip/look/crates/paint/src/pigment.rs`  
SHA-256: `9b57b84f722e72116d0fa972ecbd2e8fda9d5c420e6353ed5c6a1722c37382d5`

### Lines 22–54

```text
   22: /// K/S of a paint whose masstone (infinitely thick reflectance) is `r`.
   23: #[inline]
   24: pub fn ks_of(r: f32) -> f32 {
   25:     let r = r.clamp(0.002, 0.995);
   26:     (1.0 - r) * (1.0 - r) / (2.0 * r)
   27: }
   28: 
   29: /// Hiding of a unit coat of a paint with masstone reflectance `r` and
   30: /// scattering `s`: its reflectance over black divided by over white (the
   31: /// paint industry's contrast ratio). 0 = clear glaze, 1 = hides completely.
   32: pub fn hiding_of(r: f32, s: f32) -> f32 {
   33:     let p = Pigment { k: [s * ks_of(r); 3], s: [s; 3] };
   34:     let b = p.over([0.0; 3], 1.0)[0];
   35:     let w = p.over([1.0; 3], 1.0)[0];
   36:     (b / w.max(1e-6)).clamp(0.0, 1.0)
   37: }
   38: 
   39: /// The scattering (per coat) that gives a unit coat of masstone `r`
   40: /// (luminance) the contrast ratio `hiding`. Inverse of `hiding_of`.
   41: pub fn scatter_for(r: f32, hiding: f32) -> f32 {
   42:     let h = hiding.clamp(1e-4, 0.9995);
   43:     let (mut lo, mut hi) = (-9.0f32, 9.0f32); // ln s
   44:     for _ in 0..40 {
   45:         let mid = 0.5 * (lo + hi);
   46:         if hiding_of(r, mid.exp()) < h {
   47:             lo = mid;
   48:         } else {
   49:             hi = mid;
   50:         }
   51:     }
   52:     (0.5 * (lo + hi)).exp()
   53: }
   54: 
```

### Lines 108–121

```text
  108:     /// A paint of masstone `r` (the color it has laid thick, or over itself)
  109:     /// that scatters `s` per coat, the same in every channel: scattering
  110:     /// (by white and by particle edges) is nearly flat across the spectrum,
  111:     /// absorption carries the hue.
  112:     pub fn masstone(r: Rgb, s: f32) -> Self {
  113:         let s = s.max(1e-6);
  114:         Pigment { k: [s * ks_of(r[0]), s * ks_of(r[1]), s * ks_of(r[2])], s: [s; 3] }
  115:     }
  116: 
  117:     /// A paint of masstone `r` with `hiding` (contrast ratio of a unit coat,
  118:     /// measured on the luminance of the masstone).
  119:     pub fn masstone_hiding(r: Rgb, hiding: f32) -> Self {
  120:         Self::masstone(r, scatter_for(luminance(r), hiding))
  121:     }
```

## E12 — Existing build profiles

Source: `prototype/Cargo.toml`  
SHA-256: `75a64c7dd3bb643922d175f45db13d6ce995400d05330740051d65abc17f9f2d`

### Lines 1–33

```text
    1: [workspace]
    2: resolver = "3"
    3: members = ["crates/paint", "crates/easel"]
    4: 
    5: [profile.release]
    6: opt-level = 3
    7: debug = false
    8: # Deterministic codegen: paintings must replay byte for byte, and an
    9: # incremental build (or several codegen units) can partition and optimize the
   10: # same code differently from one rebuild to the next, moving a few pixels by
   11: # 1/255. Slower rebuilds are the price.
   12: incremental = false
   13: codegen-units = 1
   14: 
   15: # Tests run optimized (the physics tests took ~10 min unoptimized), with
   16: # debug assertions and overflow checks kept on. Not incremental, so a test
   17: # build's floats don't wobble between rebuilds either (the golden scene
   18: # and the debug replay hashes are recorded with this profile).
   19: [profile.test]
   20: opt-level = 2
   21: incremental = false
   22: debug-assertions = true
   23: overflow-checks = true
   24: 
   25: # Development iteration only (`cargo build --profile iter -p easel`, into
   26: # target/iter): release optimizations, but incremental with 16 codegen units,
   27: # so a one-line change rebuilds in seconds. Its floats may differ from the
   28: # release build's by a few pixels (see [profile.release]): real paintings,
   29: # goldens and studio exports keep the release profile (notes/workflow.md).
   30: [profile.iter]
   31: inherits = "release"
   32: incremental = true
   33: codegen-units = 16
```

## E13 — Recorded integration verdict, coverage and open items

Source: `wip/engine3-integration/notes/speed/FINAL_REPORT.md`  
SHA-256: `44e90dcde055590104db64932a4398e8235294eadd44e8685a443f81754fb504`

### Lines 1–169

```text
    1: # Faster tests, the test commands and the merge safeguards: the final report
    2: 
    3: This covers the "faster tests" half of the overnight plan, the job lock and
    4: the checks that protect main. The thinner's own results are in
    5: `notes/thinner/RESULTS.md`. Everything is on branch `engine3-overnight`
    6: (this worktree, `~/src/a/claude-paint-engine3`): engine 3 at af49348, the
    7: thinner (final at 995fbab) and the speed work, combined without conflicts
    8: (`notes/speed/COMBINE.md`). Main and the website are unchanged, nothing was
    9: painted or replayed, and the lead pushes.
   10: 
   11: ## Where it stands
   12: 
   13: The final combined commit, **`0abf7929c44159475e3fcb327329c7b932477aa6`**,
   14: was tested by `scripts/test --all --candidate` in a fresh full checkout, with
   15: the pinned job lock:
   16: 
   17: - Verdict: **NOT ALL GREEN (known pre-existing failure: thinner check 13(b),
   18:   user decision)**. Every other check passed. Check 13 (b) waits for your
   19:   decision (burnt sienna versus raw sienna on the card; `notes/thinner/ACCEPTANCE.md`
   20:   check 13). Until then no candidate can pass and the merge tool refuses it,
   21:   as designed.
   22: - Build phase: 3 builds in 149 s (of a 600 s limit). Check phase: 21 steps,
   23:   785 counted tests, in 348 s (of 600).
   24: - Receipt: `git notes --ref=test-receipts show 0abf792` (local until the lead
   25:   pushes `refs/notes/test-receipts`), files in
   26:   `~/src/a/claude-paint-receipts/0abf792…/20261004T071745-1604/`.
   27: 
   28: ## Tests made smaller or faster
   29: 
   30: No assertion was loosened and no expected answer of an existing test changed.
   31: Times are one test alone in the test profile, before and after.
   32: 
   33: | test | the bug it still catches | before | after |
   34: |---|---|---|---|
   35: | `smoke a_short_session_at_the_easel` | the easel's commands and error messages (open, do, look, check, save, run, reopen, journal). The second ground is rolled instead of brushed; brushed grounds are tested elsewhere | 58.8 s | 7.7 s |
   36: | `delivery save_delivers_the_wet_canvas_as_seen…` | a saved picture that isn't the wet canvas as seen, or a replay that delivers different bytes. It now paints only the area it checks; delivering the dry picture instead fails it (107 levels off) | 19.7 s | 7.0 s |
   37: | `determinism` hand time, state digests, rag | a replay that changes with the thread count. Now at 480 px (seeding brushes by thread fails all of them); the 2400 px versions run in `--all` | over 60 s, 8.0 s, 38.6 s | 6.7 s, 0.6 s, 2.9 s |
   38: | `rag nothing_is_lifted_past_the_gel_point` | a rag lifting paint that has set. It ages the canvas in hour steps, then two-minute steps, to the same "nothing open" point | over 60 s | 9.6 s |
   39: | the thick-swatch test (split in three) | the palette's thick swatch differing from paint laid thick | 11.3 s | 3.8 s each, side by side |
   40: | `boxes a_round_19_log_replays_as_before` | the round 19 log no longer replaying to its picture. It moved to `--all`; a 320 px version runs in the fast checks | 26.6 s | 0 in fast (about 40 s in `--all`) |
   41: | the whole Rust suite | | 102.5 s (`cargo test --workspace`) | 42 to 46 s (four test binaries at a time, no doctests) |
   42: 
   43: Two existing tests had bugs, which I fixed without changing what they assert:
   44: - `replay_env.sh` had failed since round 23. Its 5-second clip couldn't be
   45:   filled under the clip tool's 1-second hold limit; it now passes
   46:   `--max-hold 2` and passes.
   47: - `session_integrity rebuilding_serves_progress…` treated "the socket file
   48:   exists" as "the server is ready". In the final full run, under load, it
   49:   connected before the server listened (twice in two runs). It now waits for
   50:   a connection; two full runs then passed.
   51: 
   52: ## The test commands
   53: 
   54: - **`scripts/test`** (fast): builds, then the Rust tests, the old-log
   55:   replacements, the before-change baseline (state and pictures equal to
   56:   af49348) and the thinner's quick acceptance checks. On the final head:
   57:   builds 91 s, checks 63 s, about **2.5 minutes** in all when the builds are
   58:   needed; the checks alone about a minute. It ends NOT ALL GREEN (13 (b)),
   59:   exit 4.
   60: - **`scripts/test --all`**: everything required, in two phases of at most 600 s each
   61:   (builds; checks). Times are above. The slow tests, the 3200 px ground
   62:   check and the thinner's `--all` (with the 2400 px card) run side by side,
   63:   and so do the build-feature checks and the script checks that mostly wait.
   64: - Every step has a time limit and a minimum number of tests. A run that
   65:   finds no tests, skips its checks or runs out of time can't pass. A
   66:   known failure has to be named in the list with its exact exit and message.
   67: - `notes/agent_brief_template.md` tells future builders how to use them.
   68: 
   69: ## What isn't run, and why
   70: 
   71: `notes/speed/SKIPPED.md` lists every skipped test. In short:
   72: 
   73: - Seven tests that **replayed whole paintings** (engine-1 studios and six
   74:   legacy logs) never run. The plan forbids replays. They are replaced by
   75:   `scripts/tests/old_logs.sh`: eleven tiny cases (the first chunks of those
   76:   logs, a synthetic engine-1 log, a synthetic log of the legacy verbs, the
   77:   round 19 log), all compared with af49348's unchanged release build. Breaking the
   78:   engine choice or the legacy code fails them. **Missing coverage:** the later
   79:   chunks' own arguments and interactions, and the engine-1 studios' later
   80:   chunks.
   81: - Twelve diagnostics and experiments stay ignored (probes, timings, tables).
   82: - Four documentation examples aren't compiled (they're snippets).
   83: - The thinner's acceptance tests run through their own release runner, not
   84:   the regular Rust step.
   85: - Not run anywhere: the studio export scripts that need a round branch and
   86:   painter builds, and the runner tests of rounds 17 to 23 (round 24's run).
   87: 
   88: ## Script and build-setting checks
   89: 
   90: All pass in `--all`: the painter build for one box
   91: (`--no-default-features --features box-inness`, 14 tests; it didn't even
   92: compile before), every box combination (`box_features.sh`), the replay build
   93: without the finishing verbs (the smoke test), `replay_env.sh`,
   94: `check_live.sh`, `replay_clip.sh`, `box_tubes.sh`, `studio_names.sh`,
   95: `peek.sh`, the round 24 runner tests (254) and the studio viewer tests (23,
   96: none skipped).
   97: 
   98: ## Small full-resolution checks
   99: 
  100: All of these passed in the final run. "Full resolution" means the painting's
  101: own pixel density (2400 px across, or more), on a small window or a few strokes.
  102: 
  103: - **Laying paint:** the thinner's 2400 px card, the fixed target. Thinned raw
  104:   sienna keeps 84.4% of the card's contrast at load 0.3 and 81.4% at load
  105:   0.6, where at least 50% was required (thinner RESULTS). Also the 3200 px
  106:   pointed-hatch check in `crates/paint/src/tests.rs` and the 3200 px ground
  107:   check (below).
  108: - **Rag lifting:** the rag tests at 2400 px in a crop window: lifting from the
  109:   tops first, the pale tint of a dry rag, spirits lifting nearly to the
  110:   ground, nothing lifted past the gel point. The thinner's rag study (its
  111:   picture `notes/thinner/rag_study.jpg`) was reviewed separately.
  112: - **Drying:** checked across resolutions rather than at 2400 px.
  113:   `stages_dont_depend_on_resolution` compares 400 and 1200 px of the same
  114:   physical film. The other drying tests (against the sources' ranges, fast
  115:   and slow pigments, thick films, split waits) use small canvases with real
  116:   physical sizes.
  117: - **Solvent evaporation:** the thinner's checks 9 and 17 and the card's wait
  118:   (at least ten evaporation times, under one-thousandth of the solvent left).
  119: - **The 3200 px ground check** (`thin_blend_bares_ground`) runs in `--all`
  120:   and passes. See the open items.
  121: 
  122: ## Job lock, saved answers and merge safeguards
  123: 
  124: - **`scripts/lockrun`** allows one heavy job at a time on the machine; the
  125:   lead's approved copy is `~/src/a/claude-paint-tools/lockrun`. 17 checks
  126:   cover crashes, timeouts, cancellation, nesting, stale tokens and daemons;
  127:   all pass. Every heavy job tonight ran through it.
  128: - **`scripts/test`'s own process handling:** 17 checks
  129:   (`scripts/tests/test_runner.sh`), all pass. They cover timeouts, cancels, a
  130:   killed coordinator, leftover and detached processes (the easel's session
  131:   server), nesting inside a batch, known failures and the two phases.
  132: - **Saved answers** (`notes/golden_paths.txt`): the baseline, the old-log
  133:   answers and inputs, the thinner's frozen tests, the test lists, the runner,
  134:   the gate tools and the lock. A change to any of them needs an approval
  135:   recorded by someone other than its builder (`scripts/golden_approve`).
  136: - **The merge gate** (`test_candidate`, `merge_candidate`): 140 checks on
  137:   dummy repositories, all pass. Four review rounds; round 4 approved them,
  138:   and 11 of 12 deliberate breakages were caught (the 12th couldn't be made).
  139:   A receipt binds the exact commit, list, runner, lock and checkout; the
  140:   merge refuses anything but a pass, refuses if main moved, and updates the
  141:   published copy's files too. **Tested only on dummy repositories**; it
  142:   has never touched the real main.
  143: - **Approval status** (`golden_approve check --candidate 0abf792 --base 3379b9f`):
  144:   147 protected changes, 137 approved, **10 waiting for the lead**:
  145:   `crates/paint/src/state_dump.rs`, `notes/golden_paths.txt`, both test
  146:   lists, `scripts/test`, `scripts/test_candidate`, `scripts/merge_candidate`,
  147:   `scripts/safeguards_lib.py`, `scripts/tests/safeguards.sh` and
  148:   `scripts/tests/test_runner.sh`.
  149: 
  150: ## Open items for you
  151: 
  152: 1. **Check 13 (b)**, the thinner's: burnt sienna's transparency against the
  153:    period source. Until it is decided, nothing can pass the gate.
  154: 2. **`thin_blend_bares_ground`'s bound.** It allows twice the reference's
  155:    bare ground, plus 20 pixels. It didn't catch the brushed-ground crest bug its own
  156:    comment describes (raising the brush's push to 0.3 or 0.9 still passed:
  157:    7476 bare pixels against a limit of 9570). Tightening it is a change to an
  158:    approved answer, so it is your call.
  159: 3. **The new gate copy.** `~/src/a/claude-paint-tools/gate-next/` (two
  160:    phases, known failures) was reviewed and approved in round 4 but not yet
  161:    promoted: the approved `gate/` copy can't test this branch (it doesn't
  162:    read the new list format). Promote it after the lead records the 10
  163:    approvals.
  164: 4. **Linux is untested.** Everything ran on macOS. The lock and the runner
  165:    use portable calls (and `/proc` for environments on Linux), but nothing
  166:    was run there.
  167: 5. **Smaller things.** A test that kills the runner's coordinator leaves an
  168:    empty `/tmp/cpt.*` directory. The existing `frames.rs` test leaves an empty
  169:    `out/test/`. The legacy logs' later chunks aren't covered (above).
```
