# Plan: intaglio prints on the easel (drypoint, soft ground, etching, aquatint, à la poupée, the press)

Draft 1, 2026-11-15, for critique. It is gap 3 of notes/cassatt_support.md, with gap 12 (monotype) and the engine work that gaps 4 (tracing) and 5 (counterproofs) need. Physics and sources: notes/research/intaglio_physics.md (cited here as **IP §n**), with its tags: [M] measured, [D] derived, [P] practitioner report, [E] estimate. Decisions for the owner are **I1–I9** at the end. A related plan for relief prints is notes/plan-woodblock.md (branch `notes/woodblock-plan`, draft 1). §8 shares its edition and state machinery.

## 0. The shape

An intaglio print is made in three places: a **plate**, which the artist works and the acid bites; the **printer's bench**, where the plate is inked and wiped; and the **press**, where damp paper is pressed into the plate's lines and pulls the ink out of them.

Each step is a physical exchange the engine can compute:
- **The plate holds metal and air.** A height field below its surface, plus the burr above it.
- **The wipe decides which of that air keeps ink.** That is the cloth's reach (IP §5.3).
- **The press decides which of that ink reaches the paper.** That is the paper's press-in and the ink's split (IP §6.4).

Every look of the print is then the ink's Kubelka–Munk optics on the paper, as oil paint already is.

What is reused:

| need | existing model | where |
|---|---|---|
| the printing paper: fibers, pores, laid lines, tone | `Paper` | crates/paint/src/paper.rs |
| a damp sheet's water clock | PR #322's water per cell and its evaporation and sinking | crates/paint/src/wet_pastel.rs (PR #322) |
| a sheet pressed by an edge, the dent's spread through its thickness | `edge_sink_um`, the anisotropic layer | crates/paint/src/sheet.rs; notes/research/paper_mechanics_and_transport.md §1 |
| bearing (Abbott) curve of a fiber surface: what a contact reaches at a depth | the paper's pore depth µ, 1 − e^(−d/µ) | paper.rs, graphite.rs, pastel.rs |
| a tip's reach into a gap, r − √(r² − w²/4) | the dry brush on pastel | sheet.rs; notes/research/pastel_brushing.md |
| a cloth pad that loads, refolds and smears its load back | `Rag` (face load, `soaked`, folds, `SMEAR`) | crates/paint/src/rag.rs, crates/easel/src/draw_rag.rs |
| ink on paper: pigment in oil, its optics, its drying | the oil film, `pigment.rs`, `drying.rs` | crates/paint/src |
| a fabric's height field for soft-ground textures | the linen weave | `canvas{linen=}` |
| a sheet laid over a picture, catching what lands on it | `lay_sheet` | sheet.rs |
| a mask from a drawing | `drawing_guide()` | graphite.rs |
| hand time | `tally::pace`, `time::verb` | crates/paint/src/tally.rs, crates/easel/src/time.rs |

**What is new:**
- a plate
- several named surfaces in one session
- a bulk moisture state for a whole sheet
- the acid's chemistry
- the press

## 1. Engine: several named surfaces in one session

### 1.1 Why
- **Today.** A session holds one canvas: `Studio::canvas: Option<Canvas>` (crates/easel/src/api.rs). The scratch canvas is a second `Session` with its own Lua state, its own log and its own clock (crates/easel/src/main.rs `Server::scratch`).
- **What intaglio needs.** The plate, the sheet it prints on and the drawing traced onto it have to live in one Lua state and on one clock. A printer moves between them, and an acid bath that bites for 20 minutes passes those minutes for everything.
- **Other gaps.** Tracing (gap 4) and counterproofs (gap 5) need the same: a study beside the work, and a damp sheet pressed onto a pastel.

### 1.2 What
- **The main surface.** `canvas{}` stays the first chunk and makes the main surface. It is named `"painting"`, as today.
- **Further surfaces.** New verbs make named surfaces of other kinds:
  - `sheet{name=, paper={...}}`: a paper surface, the same as `canvas{paper=}`
  - `plate{name=, ...}`: §2
- **Surfaces in Lua.** Each surface is a userdata. Its own verbs are methods (`p:drypoint`, `s:look`). Verbs that act between two surfaces name both:
  - `press{plate=, sheet=}`
  - `trace(from, onto)`
  - `counterproof{from=, onto=}`
- **Which surface the old verbs use.** Existing drawing verbs (`pencil`, `pastel`, brushes, `rag`) act on the current surface. `use(s)` makes `s` current, and `use("painting")` goes back. A log without the switch has only `"painting"`, so every existing verb behaves exactly as before.
- **Looking.** `look`, `probe`, `status` and `log` take `surface: "<name>"`. Without it they show the main surface as today.
- **One clock.** All surfaces share it, so `wait` and the acid's minutes pass for all, and damp paper dries while the plate bites.

### 1.3 Memory (PR #311's one-canvas rule)
- **The rule.** #311 keeps one canvas in memory at a time and packs the other to disk (`Session::rest` / `wake`, `CanvasSnapshot`).
- **Extended.** Only the current surface, and any surface a verb names, is awake. Every other surface rests packed.
  - `press` and `trace` wake two.
  - A plate is smaller than an oil canvas: §2.4, stored tile-sparse.
  - A paper sheet without oil films holds no engine-9 skins.
  - The worst case is the painting's canvas plus a plate.
- **Rollback.** The packed rollback of a failed chunk (#311) is taken for each surface lazily, the first time a chunk wakes it. A failed chunk puts back every surface it touched.
- **The test.** #311's `scratch_canvas_memory` pattern: peak RSS of a session with a painting, a plate and a sheet, staying under the bound with two awake.

### 1.4 Checkpoints and replay
- **Saves and live checkpoints** (crates/paint/src/checkpoint.rs) gain one engine-word bit for "several surfaces" and a versioned `SURF` section:
  - the count, then each surface's name and kind and its own payload (a canvas's existing sections, or a plate's `PLAT` section, §2.4)
  - the current surface's name
- **A file without the bit** is one unnamed surface, `"painting"`, read exactly as today.
- **Readers.** As for #322's `WETP`, a reader that doesn't know the bit refuses the file.
- **Engine bits.** Bits are contended: #86 has 43 and #322 has 44. This plan takes the next free bit at merge time, not a fixed one.
- **Replay checkpoints** (notes/replay-checkpoints.md) re-execute Lua and use recorded results for verbs. What changes:
  - Each new verb goes through `time::verb` and records its post-state.
  - Every surface-dependent read outside a verb gets its own typed trace record, as `drawing_guide` and the rag's dampness do now. New reads: `p:depth(x, y)`, `p:feel(x, y)`, `damp_of(s)`.
  - The studio and tool state hash that each trace entry checks covers every surface's identity and the current one, and the plate tools (needle wear, tarlatan load, dabbers).
- **The switch: `--@ surfaces`.** It is written by new paintings and installs `sheet{}`, `plate{}`, `use` and the `surface:` option. An older log replays with one surface and none of these.

## 2. The plate

### 2.1 The object
```lua
k = plate{name="key", metal="copper", temper="half-hard", size={346, 211}, thickness=1.3,
          bevel=45, at={42, 45}}   -- mm; where it will lie on the sheet (§6)
```
- **`metal`** sets its hardness, modulus and how acid bites it (IP §1.1, §3):
  - `"copper"`: `temper` `"annealed"` 45 HV, `"eighth-hard"` 65 HV [E between the two measured tempers], `"half-hard"` 90 HV
  - `"zinc"`: 45 HV
  - `steel_face()` later deposits iron (§8)
- **`thickness` and `bevel`** set the plate mark (IP §6.3).
- **The plate's own clock state:** grounds, resin and ink, all on the shared clock.
- **Mirroring.** A plate is drawn on as seen from above, and prints mirrored. That holds for whoever draws on it, as it did for her. `look` on a plate has `mirror` (existing) to preview the print.

### 2.2 What each plate cell holds (a statistical plate)
- **Scale.** At the easel's 2400 px over a 430 mm sheet a pixel is about 0.18 mm, while a drypoint line is 40–170 µm wide (IP §1.3).
- **So the plate is statistical,** as paper.rs keeps pores as a mean depth per pixel rather than as pores. Each cell holds:

| field | meaning | set by |
|---|---|---|
| `open` | share of the cell's area below the plate's surface (lines, pits) | needle, acid, roulette, burnisher, scraper |
| `depth` | mean depth of the open part, µm | the same |
| `width` | volume-weighted width of the openings, µm (what the cloth and the paper meet) | the same |
| `burr` | burr height, µm, and its share of the cell | needle, roulette; worn by the press and the wipe (§8) |
| `burr_dir` | the burr's side and the line's direction (a byte) | needle tilt and direction |
| `level` | the surface's own height, µm (a scraped dish is negative) | scraper, burnisher |
| `ground` | ground kind and thickness, µm, and the share bared | `ground`, needle, soft-ground trace, `impress`, stop-out |
| `resin` | aquatint dot coverage c and dot diameter, µm; melted or not | `aquatint`, `melt` |
| `ink` | ink in the openings (µm over the cell) and the film on the surface (µm), each as shares of the session's inks (at most 8) | `ink`, wipes, press |

- **Several marks in one cell combine.** Open shares add up to at most 1. Depth and width are averaged by volume, so the cell holds the ink the lines would hold.
- **Vectors as well as raster.** The tool paths are kept as the log keeps them, as vectors with their forces, so a crop render rasterizes the plate at its own scale (woodblock plan §1, "carved lines stay crisp regardless").
- **Raster resolution.** The plate is laid out in mm, as paper.rs is, at the sheet canvas's pitch. A plate rebuilt at another width gives the same print at that width.

### 2.3 Looking at and feeling a plate
- **The look.** Copper or zinc under the studio light: polished metal with lines dark where they shade, ink black or coloured in them, ground as brown-black lacquer, rosin as a tan haze.
- **Metal optics.** A metal's reflectance is specular and coloured. KM can't render it, and gap 10 (metallic paint) needs it too.
  - Phase P2 needs only copper's and zinc's measured spectral reflectance at normal incidence (from optical constants, e.g. Johnson & Christy 1972 for copper), shaded by `relief`'s existing lighting.
  - The angle dependence (Fresnel) is a later refinement, shared with gap 10.
- **`probe` on a plate** gives depth, open share, burr height, ground and ink at a point.
- **`p:feel(x, y)`** answers in words, as a fingernail would: "a line, about 40 µm deep, with burr", "smooth", "grain".

### 2.4 Storage and checkpoint
- **Tiles.** A plate stores its fields in 64 × 64-cell tiles, allocated when first written. An untouched plate costs almost nothing, and a typical drypoint touches a few percent of its area.
- **Precision.** Fields are f32 where they accumulate (depth, ink) and u8/u16 where they are states (ground kind, burr_dir).
- **The `PLAT` section** (version 1) holds the metal, the size, the tiles, the ink list and the edition counter (§8).

## 3. Tools on the plate (phase P2; acid tools are §4)

### 3.1 Drypoint needle
```lua
n = needle{tip=25, cone=30}          -- tip radius µm, half-angle of the cone, degrees
k:drypoint(n, pts, {force=2, alt=55, azimuth=110, tilt=0, speed=30})
```
- **The groove** (IP §1.3) follows scratch hardness. Contact width w = √(8F/(πH)) for the cone, or the sphere's chord for depth below the tip's radius. The depth follows the cone.
  - Force is in newtons, as for pastel. Each option may be a list along the points, as `p:stroke` takes them.
  - The hand's force rises and falls at the stroke's ends, as for pastel (50 ms up, 40 ms down).
- **Plough or cut** (IP §1.2). The attack angle is β = 90° − θ − τ, with τ the needle's lean along the stroke from `alt` and `azimuth`.
  - Below β_c the needle ploughs: the groove's volume goes into burr, on the sides given by `tilt` (a sideways lean: one side; upright: both).
  - Above β_c (default 60°, the middle of copper's measured 45–85° [M]) it cuts: the volume leaves as a chip and the burr is small.
  - So a needle held upright and pushed makes a cleaner line than one pulled at a lean. The verb reports a chip.
- **The burr** has the groove's volume (IP §1.4, [D]), with height **[E] 0.75 × depth** until the burr is measured (Unknown 1), as a fin leaning away from the groove.
- **Needle wear.** The tip blunts over its path by Archard's law, steel on copper. **[E]** K for steel on annealed copper is from tribology tables. `n:sharpen()` restores it, with hand time.
- **Hand time.** Speed in mm/s. **[E] 20–40 mm/s** for a drypoint line, slower than a pencil: it is hard work.

### 3.2 Burnisher, scraper, roulette (IP §1.7)
```lua
k:burnish(pts or mask, {force=30, radius=10, passes=3})
k:scrape(pts or mask, {force=5, width=8})
k:roulette(pts, {pitch=0.2, tooth=30, force=3})     -- mm between teeth, tooth tip radius µm
```
- **Burnisher.** Where the Hertzian contact of a burnisher of radius `radius` under `force` exceeds the metal's hardness, the plate flows. Each pass the highs (burr, then aquatint dots, then the rims of lines) come down toward the local mean, and shallow openings close. Depth and open share fall, so the area prints lighter.
- **Scraper.** Cuts at a high attack angle. It removes burr and lowers `level` by the depth its force and width reach, taking lines with it.
- **Roulette.** A row of pits at the tooth pitch. Each is an indentation at H with a ring of burr.

### 3.3 Test sheet (P2's gate)
A copper plate and a zinc plate, each with drypoint lines at 0.5, 1, 2 and 5 N, a pulled line and a pushed one, a burnished patch over lines and a scraped correction. Shown in `look` on the plate (relief) and in `probe`.

## 4. Grounds, acid and aquatint (phase P4)

### 4.1 Grounds
```lua
k:ground("hard", {um=4})                  -- rolled on a warm plate (IP §2.1)
k:ground("soft", {um=12, grease=0.4})     -- share of tallow or Vaseline (IP §2.2)
k:needle(n, pts, {force=0.5})             -- an etching needle: clears the ground, barely marks the metal
k:stop_out(mask)                          -- varnish: no acid there
k:clean()                                 -- grounds, resin and stop-out off (solvent): hand time
```
- **The etching needle.** It bares the ground along its contact width. The force needed to clear a ground of thickness t is far below what marks copper (§3.1 at under 0.3 N), so it reports whether it touched the metal.
- **Foul bite.** Soft ground carries a small density of pinholes, more where it is thin. **[E]** No measurement exists; the density is a dial with a default that a bench test would set.

### 4.2 The bath on the clock
```lua
k:bite{acid="ferric", baume=38, temp=20, minutes=15, rock=true}
k:bite{acid="nitric", parts=8, temp=18, minutes=4, feather=true}
k:bite{acid="dutch", minutes=30}
k:spit(mask, {minutes=3})             -- ferric brushed on (IP §3.3)
```
- **Ferric on copper** (IP §3.1). The rate is r = ½·V_Cu·D·C/δ.
  - D falls with the bath's viscosity: 12 cP measured at 40 %, and the rate peaks near 2–2.25 M, as measured.
  - The temperature enters through Ea = 24.9 kJ/mol [M].
  - δ is the boundary layer: still or rocked, face up or down (sediment thickens it).
  - The bath tracks its dissolved copper, which lowers C as it accumulates, up to its 100 g/L capacity [M]. An old bath is slower.
  - **The δ values are [E]** until Unknown 4 is measured, set so that a still 38 Bé bath at 20 °C bites about 1 µm/min, as the derivation and the workshop times agree (IP §3.1).
- **Lateral bite.** Where the ground is bared the acid bites down at r and sideways under the ground's edge at r/K. K ≈ 1.2 [D] for a still ferric bath and 1–2 for nitric [E].
  - A line's width therefore grows with its depth.
  - A line narrower than its depth bites slower as it deepens (the slot's diffusion). Deep bites end in a wedge (IP §3.4).
- **Nitric on zinc** (IP §3.2):
  - Its rate is several times faster [P]. No rate was found, so **[E]** until Unknown 4.
  - Its bubbles cover a share of the bitten area that grows between sweeps of the feather. Under them the bite is slower, so an unfeathered bite is uneven and its lines ragged.
  - Its undercut is larger.
- **Dutch mordant** on copper: slow and even. **[E]** rate until measured.
- **The clock.** A bite advances the shared clock by its minutes, with the hand's time to put the plate in, rock it, rinse and dry it.

### 4.3 Aquatint
```lua
k:aquatint{box=0.6, wait=30, minutes=1}   -- dust box: height m, wait s before the plate goes in, time in it
k:aquatint{shake="fine"}                  -- shaken through cloth
k:melt()                                  -- heats the plate until the grains clear: the dots spread
```
- **The dust box** (IP §4.1, [D]). The grains still in the air after the wait are those whose Stokes velocity is below box/wait. They land on the plate at their settling flux over `minutes`, so coverage c grows as 1 − e^(−n·a) with n grains per area and a their area. A longer wait gives a finer grain and less coverage per minute.
  - The cloud's initial size distribution and density are **[E]**; Unknown 7 measures them.
- **Melting.** Dots spread to **[E] 1.3×** the grain's diameter (IP §4.1).
- **Biting an aquatint** uses §4.2. Between the dots the bare network bites down. Under them the undercut eats in, and a dot whose undercut reaches its radius is lost.
  - The tone rises with bite and falls once the dots go: the over-bitten aquatint the workshops describe (IP §4.2).
  - It falls because the wipe (§5) reaches into open bite, not because a rule caps the tone.
- **Stopping out between bites** gives the stepped tones Cassatt used to relight a face across states (research §5).

### 4.4 Test sheet (P4's gate)
- A hard-ground line plate bitten in steps of 5, 15 and 40 min (still ferric, 20 °C) and the same at 30 °C.
- An aquatint step wedge of 0–40 minutes in fine and coarse grain.
- Each printed (once P3 exists) beside its plate in relief.

## 5. Inking and wiping (phase P3)

### 5.1 Ink
```lua
black = ink{{"bone black", 3}, {"burnt umber", 1}, oil=0.35, stiff=1}  -- parts of tubes; oil share; plate oil: 1 heavy .. 0 light
k:warm(40)                                    -- on the hot plate, °C
k:ink(black, {mask=m, by="card"})             -- pushed into the lines and grain
k:ink(blue, {mask=dress, by="poupee"})        -- à la poupée: a dabber into an area (§5.4)
```
- **What ink is.** Pigment in burnt linseed oil, mixed from the box's tubes as a pile is. Its optics are the pigments' (pigment.rs) at the ink's pigment volume.
- **What its rheology sets:**
  - how well it fills fine lines: an opening fills to a share φ falling with the ink's yield stress against the push of card or dabber
  - how it shears under the cloth
  - **Values.** Viscosity at 20 °C: **[M]** 130 Pa·s for the bureau's ink, with banknote inks at 5–50 Pa·s; falling with warmth by linseed oil's measured Arrhenius slope. The yield stress is **[E]** 10²–10³ Pa (IP §5.1).
- **Fill [E].** Until Unknown 3 is measured, a line or pit filled by card or dabber holds φ = 0.9 of its volume. φ is less for a stiff ink in openings under 30 µm and more on a warm plate.

### 5.2 Wiping
```lua
t = tarlatan{stiff=0.7}                     -- starch, 0 soft .. 1 stiff
t:wipe(k, mask, {pressure=0.5, passes=3, pattern="circles"})
k:hand_wipe(mask, {passes=2, whiting=true})
k:retrousse(mask, {passes=1})
```
The tarlatan is a `Rag` (crates/paint/src/rag.rs): the same face that loads and lifts less as it fills, the same refolds and soaked cloth, the same creases that press unevenly and the same smear back over its trailing end. What changes is what it reaches. The rag reaches the oil wet layer of a canvas through `BRIDGE_MM` and `SAG_UM` (both [E] there); the tarlatan reaches the plate's ink through two derived reaches (IP §5.3):
- **Into an opening narrower than the yarn spacing:** a yarn of radius r dips h = r − √(r² − w²/4). That is the dry brush's formula in sheet.rs, the same code.
- **Into a wide opening:** the cloth sags s = p·w⁴/(384·B), with B its bending rigidity from `stiff`. B is **[E]** 10⁻⁵–10⁻⁴ N·m: the fabric range is measured, the starched value is extrapolated (Unknown 8).
- **What it takes.** The ink above the deeper of the two reaches is in contact, and comes away by the rag's lift law. The film on the plate's flat comes away until the face's smear-back leaves plate tone.
- **Burr.** The cloth is held off the plate in the lee of a burr for (B·h/p)^(1/4) (IP §1.5), with the side from `burr_dir` and the wipe's direction. Wiping across a line keeps its lee ink; wiping along it takes it.

The **hand wipe** is a softer contact (skin, IP §5.4). It reaches the flat fully and polishes the surface film to near nothing, and it reaches only the top few µm of openings. With `whiting` it also abrades burr and aquatint (§8). **Plate tone after each wipe is [E]** (IP §5.4) until Unknown 3.

**Retroussage.** A soft cloth's light pass takes ink from the top of each opening and lays it in a band beside it, as the rag's smear does.

**Hand time.** Cassatt's letter gives 45–60 min per three-plate proof, so 15–20 min to ink and wipe each plate (IP §5.4). The paces for card, dabber, tarlatan and palm are set so that a whole plate's usual sequence comes to that. This is a historical measurement, unlike the rag's estimated paces.

### 5.3 What the reuse needs from rag.rs
- **A new face.** rag.rs's contact and lift run on `Canvas`. The plate is not a canvas, so `Rag` gains a second face: `lift_plate(&mut Plate, ...)` beside its canvas path. It shares the face, load, fold and smear code, with the reach computed from the plate's openings.
- **Old logs.** The canvas path is untouched, so old logs replay unchanged without a switch.

### 5.4 À la poupée (phase P6)
- **Inking.** Each `ink(..., {by="poupee", mask=})` fills its mask's openings with that ink. A cell keeps up to 8 inks as volume shares.
- **The wipe across a boundary.** A tarlatan face loaded with one ink lays a little of it into the next area along its path (the smear). The band where inks mix is as wide as the face's contact along the wipe (IP §5.5): the blue that crept into Cassatt's flowers.
- **Mixing.** Mixed ink in a pit prints as a KM mixture of its inks by volume.

## 6. Damp paper and the press (phase P3)

### 6.1 The sheet's water
```lua
s = sheet{name="proof", paper={grammage=250, laid={per_cm=9, chain=25}, tone={...}}}
soak(s, {minutes=5})          -- or soak(s, {mist=true}) for unsized and Japan papers
blot(s, {passes=2})
damp_of(s)                    -- g of water per g of paper, and "shiny" / "damp" / "dry and crisp"
```
- **The new state.** PR #322 adds water per pastel cell, but it is surface water, and it says the sheet's own water capacity is not modelled. A print needs that: the bulk moisture of the sheet, MC in g/g, per cell.
- **`soak`** fills the pores towards saturation at the rate the sizing lets water in. Sizing slows water: the `absorbent` dial that #322 extends to water.
- **`blot`** draws the water in the pores down towards a damp sheet's 0.5–0.6 g/g (IP §6.2), measured from a printer's damp pack.
- **Drying** is #322's evaporation (its [E] rate, from indoor-air formulas) from each exposed face, on the shared clock. A sheet under blotters or plastic (`cover(s)`) loses none.
- **What the water changes:**
  - **Stiffness.** z-compression and bending moduli fall with MC. **[E]** About tenfold from dry to damp, until Unknown 5 (IP §6.2).
  - **Size.** The sheet expands by β·min(MC, MC_fsp) with β 0.05–0.12 %/% [M], β lower for long rag fibers [M]. The expansion levels off at fiber saturation, MC_fsp ≈ 0.3 [E] (IP §7.2).
  - **Ink uptake.** It moves the ink's split share s between the non-absorbent 0.45 [M] and the absorbent 0.6 [M]. **[E]** 0.6 for a damp sheet.
- **Joining #322.** #322's sinking (surface water into the sheet) becomes a transfer into this bulk state. Under `--@ damp-paper` only, so #322's own logs replay as they were.

### 6.2 The press
```lua
press{plate=k, sheet=s, at={42, 45}, felts={1.6, 6.35, 3.2}, pressure=0.7, roller=100, speed=20}
```
- **The pressure** (IP §6.1).
  - `pressure` 0..1 is the press's screws, up to its line load: a hand press's **130 N/mm [M rating]**, `load=` to change it.
  - The roller (radius mm) on the felt pack (layers in mm) gives the contact width and the pressure profile by the Winkler layer of IP §6.1.
  - Over the plate the felts are squeezed by the plate's thickness more, so the pressure there is higher.
  - Felt modulus is **[E]** until Unknown 2. The verb reports the pressure on the plate in MPa, so a painter can check the setting against a measurement later.
- **Press-in.** Into each opening of width w the damp sheet is pressed by the anisotropic-layer response (paper_mechanics_and_transport.md §1, `edge_sink_um`'s model, with damp moduli). Wide openings fill and narrow ones are bridged. The burr presses into the sheet.
- **Transfer** (IP §6.4). Where the sheet reaches the ink (press-in π and fill φ, π + φ ≥ 1), the ink splits by s, wherever across the opening's width the sheet touches. Elsewhere nothing goes.
  - The burr's ink and the plate tone are always in contact.
  - The ink lands on the sheet mirrored, at the plate's place, as an oil film on paper: pigment, thickness, the existing film code. It squeezes sideways at a line's edge by **[E]** 5–15 µm.
  - What isn't transferred stays on the plate for the next inking.
- **Embossing.** The sheet's height field takes the plate's relief: lines and burr raised, the plate mark pressed in at the bevel (IP §6.3, depth **[E]**). It sets as the sheet dries below fiber saturation.
- **Damp paper and oil ink.** While damp, the sheet's pores hold water, which the oil can't enter. The ink sits on the fibers and is not drawn in until the sheet has dried, then only as `absorbent` allows. It dries as an oil film on the clock, in days.
- **Hand time.** About a minute per pass at `speed` mm/s, with laying the sheet and lifting it.
- **Mass balance.** The ink lost from the plate equals the ink gained by the sheet. That holds to rounding, and is a test.

### 6.3 Test sheet (P3's gate)
- A drypoint plate from P2 printed on damp and on dry laid paper, at three pressures.
- A wiped line plate printed after a light and a heavy hand wipe.
- Looked at whole, in a crop, and in relief: the plate mark and embossed lines.

## 7. Soft ground, tracing and impressions (phase P5)

```lua
study = sheet{name="study", paper={grammage=60, calender=0.6}}   -- thin, smooth: a tracing sheet
use(study); h = pencil("B"); h:line(pts, {pressure=0.5}); use("painting")
trace(study, k, {lead="2B", pressure=0.6, mirror=false})  -- retrace the study's lines laid on the plate
k:impress(fabric{linen=22, thread=0.25}, {pressure=0.4})  -- through the press, one felt out
```
- **`trace(from, onto)`** retraces `from`'s drawn lines (its `drawing_guide()`) with a pencil, pressing the sheet onto whatever lies under it (IP §2.3).
  - **Onto a soft-grounded plate.** The sheet's underside is a paper surface: its fiber tops and pores, from paper.rs. Under the pencil's footprint, spread through the sheet's thickness as `edge_sink_um` spreads a knife's dent, it is pressed into the ground.
  - The ground leaves the plate wherever the fibers reach through its thickness. The share is the sheet's bearing curve at that depth.
  - The bared ground sticks to the sheet's underside, so the study keeps faint ground lines on its back, as hers did (IP §2.3; research §5).
  - **Onto a sheet (gap 4).** The same contact lays the pencil's line through, as a faint graphite or indented line. A sheet of carbon or chalk between them is a later option.
- **`impress`** presses a fabric's height field (the linen generator), leaves or a paper into the soft ground through the press (§6.2 with a felt out). The bared share is the weave's bearing curve at the press's indentation (IP §2.4).
- **Defaults.** Soft-ground thickness and rheology are **[E]** until Unknown 6, chosen so that the workshops' failure cases happen: a thick ground or a thick top sheet bares nothing (IP §2.2).
- **The switches.** `--@ soft-ground` installs these verbs on plates. `trace` between sheets belongs to gap 4's switch, `--@ trace`, which can land first: it needs only P1.

## 8. Several plates, registration, states and editions (phases P6, P7)

### 8.1 Registration (P6)
```lua
pins(s, {holes={{12, 150}, {418, 150}}, needle=1.2, hole=1.4})   -- mm; holes punched in the margin
press{plate=b, sheet=s, register="pins"}                         -- or register="trapped", or at={x, y}
```
- **Pins.** The sheet lands on the pins with its holes' slop: the hole's diameter minus the needle's. The slop is random within it, so two pins give a shift and a small rotation (IP §7.2). Holes enlarge with each use unless taped. **[E]** 0.01 mm per use.
- **The trapped method.** The sheet stays under the roller at one end. It keeps that end's position exactly, and the rest of the sheet's drift from its stretch.
- **The sheet's size.** Every print is placed by the sheet's own size at that moment, from its MC (§6.1). A sheet that dries below fiber saturation between plates is out of register by β·ΔMC·L, and each pass stretches it by **[E]** 0.05–0.3 % in the rolling direction (Unknown 5). Misregistration falls out of the paper, as the woodblock plan wants (woodblock §3, "Climate").
- **Plates from one drawing.** Cassatt's second and third plates were made by a second tracing or by an offset of a proof (research §5: unknown which). Both are available:
  - `trace(study, plate2)`
  - `press{plate=k, sheet=plate2, offset=true}`: a proof pressed onto a soft-grounded plate, the counter-proof mechanism of gap 5
- **The test.** Two plates registered by pins on a sheet kept covered land within the slop. The same with the sheet left to dry 10 minutes between them is out by β·ΔMC·L, computed from the sheet's MC change. Both numbers come from the formulas, not from the code.

### 8.2 States (P7)
```lua
k:state("III")                 -- names the plate as it now stands; a proof can be pulled from any named state
```
- **A state** is a snapshot of the plate, stored as its tool log up to that point, so it costs nothing to keep. A proof's sheet records which state of which plate printed it.
- **Cassatt's prints ran to 4–13 states** (research §5). The relighting of a face by aquatint between states is §4.3's stopping-out.

### 8.3 Editions and wear (P7)
- **What wears.** Each `press` and each wipe wears the plate:
  - **The burr** loses height by the press's flattening and the wipe's abrasion. Its height decays as h_n = h_0·e^(−n/N), with N set by the metal's hardness through Archard's law. N is fitted to the burred impressions reported, **10–20 for copper [P]** (IP §1.6).
  - **Zinc** has about half of that. **Steel-faced** plates about 3–5×: `k:steel_face()` deposits iron (300–500 HV, IP §1.6).
  - **Aquatint dots and shallow openings** wear by the same law with their own height, so a fine aquatint goes grey over an edition (IP §4.2).
- **Not a separate model.** The physics is per impression: N comes out of the hardness and the wear law, and the [P] counts are the check, not the input. N stays labelled as a fit until Unknown 1 is measured.
- **The edition.** As in the woodblock plan (§4), the printer's recipe (plates, inks, wipes, pressure, order) is a Lua function in the log, and an edition is that recipe run n times on fresh sheets.
  - `edition{recipe=r, count=25, sheets={paper=...}}` pulls them all.
  - `pull{recipe=r, impression=12}` pulls one on plates worn to that point. The wear is computed from the counts of the impressions before it, without pulling them.
  - The hand's variation from sheet to sheet (pressure on the wipe, the dabbers) is seeded by the impression number.
- **Roles.** As in the woodblock plan (§4b), the roles are sessions: an artist session makes the plates and marks proofs, and a printer session inks, wipes and pulls the proofs and the edition, with plates and proofs handed between them as files that keep aging in transit (I6). Cassatt inked her color plates beside Leroy ("imprimée par l'artiste et M. Leroy"); her marks on the proofs tell the printer session how.

## 9. Monotype (phase P8)
```lua
k:paint(b, pts)                -- a brush lays ink on a bare plate (light field)
k:ink(black, {mask=all}); t:wipe(k, m) -- or ink all over and wipe the image out (dark field)
press{plate=k, sheet=s}; press{plate=k, sheet=s2}   -- the print and its ghost
```
- **On a bare plate** the ink is a film on the flat. The brush lays it as a pile of ink; a bristle brush on a smooth plate uses the existing bristle model with the plate as a non-absorbent surface (I8).
- **Its split** is s (IP §6.5). The ghost is (1 − s) of the first pull's ink, 0.4–0.5 [D], and a third pull (1 − s)². Degas's pastel-over-ghost works by `use(s2)` and pastel.
- **Cassatt's** one monotype (c. 1894) added to drypoint and aquatint plates is a plate inked in its lines and painted on its flat.

## 10. Switches and checkpoint sections

| switch | installs | checkpoint |
|---|---|---|
| `--@ surfaces` | `sheet{}`, `plate{}` (a bare plate), `use`, `surface:` on the tools | engine bit + `SURF` |
| `--@ plate` | plate tools: needle, burnisher, scraper, roulette; `p:feel`, plate looks | `PLAT` v1 |
| `--@ damp-paper` | `soak`, `blot`, `cover`, `damp_of`; the sheet's MC and size | `DAMP` v1 (MC per cell) |
| `--@ press` | `ink`, `tarlatan`, `hand_wipe`, `retrousse`, `warm`, `press` | `PLAT` v2 (ink) |
| `--@ etch` | grounds, `needle` through ground, `stop_out`, `bite`, `spit`, `aquatint`, `melt`, `clean` | `PLAT` v3 (ground, resin, the bath) |
| `--@ soft-ground` | soft ground, `trace` onto plates, `impress`, `fabric` | `PLAT` v3 |
| `--@ trace` (gap 4) | `trace` between sheets | none |
| `--@ registration` | `pins`, `register=`, `offset=`, à la poupée `by="poupee"` | `DAMP` v2 (holes) |
| `--@ edition` | `state`, `steel_face`, `edition`, `pull`, wear | `EDIT` v1 (states, counters) |
| `--@ monotype` | `p:paint`; brushes on plates | none |

- **Pattern.** Each switch follows `--@ paper` and `--@ wet-pastel`:
  - parsed from the log head
  - kept as a state flag
  - written by new paintings
  - documented in notes/easel_guide.md
  - with an old-log test that replays a log without it bit-identically and refuses its verbs
- **Dependencies.** A switch needs those it builds on (`--@ press` needs `--@ plate` and `--@ damp-paper`). Without them it is accepted and installs nothing, as `--@ wet-pastel` does without `--@ drawing-optics`.

## 11. Phases

| phase | builds | size | needs | gate: tests that prove it |
|---|---|---|---|---|
| **R** | this note's research; bench measurements (§12) in parallel | done / ongoing | — | each [E] that a measurement replaces is re-derived, not tuned |
| **P1** | several named surfaces in one session; rest and wake; rollback of every surface touched; `SURF` checkpoint; `look` and `probe` on surfaces; a surface exported to a file and imported into another session with its age, the artist→printer handoff (I6) (`--@ surfaces`) | M | — | an exported plate imported after a recorded interval arrives aged by it (ink on it dried that long), and bit-identical otherwise. A log without the switch replays bit-identically. A failed chunk that drew on two surfaces leaves both as before. Checkpoint round trip with three surfaces; a damaged `SURF` is refused. Peak RSS with painting + plate + sheet under #311's bound. `wait` passes for every surface. Replay checkpoint across a `use` |
| **P2** | the plate (copper, zinc; tiles; `PLAT`), drypoint needle, burnisher, scraper, roulette, plate look, `p:feel` (`--@ plate`) | M | P1 | groove width ∝ √F (4× force gives 2× width, ±5 %); zinc against half-hard copper gives √2× the width at one force; burr volume within 10 % of the groove's for a pulled needle; a needle pushed above β_c reports a chip and leaves under a fifth of the burr; a tilted needle's burr is on one side; burnishing lowers burr before line depth; checkpoint round trip |
| **P3a** | the sheet's bulk moisture, `soak`, `blot`, `cover`, drying on the clock, size against MC (`--@ damp-paper`) | S–M | P1 (with #322 merged) | a blotted sheet reaches 0.5–0.6 g/g; a covered sheet loses nothing over 30 min; the sheet's size follows β·min(MC, MC_fsp) exactly; #322's logs replay unchanged |
| **P3b** | ink, inking, tarlatan (`Rag` on a plate), hand wipe, retroussage, warm plate; the press: felts, pressure, press-in, transfer, mirror, embossing, plate mark (`--@ press`) | L (two PRs: wipe, then press) | P2, P3a | a line under 0.5 mm keeps its ink under tarlatan and a 5 mm open bite is wiped clean but for its edges (the w⁴ law); wiping across a burr keeps its lee, wiping along it does not; transfer rises with fill and press-in to the split share and never above it; damp sheet > dry sheet; the print is mirrored; ink is conserved plate → sheet + cloth; old rag logs unchanged; the P3 test sheet (§6.3) |
| **P4** | grounds, etching needle, stop-out, the bath (ferric, nitric, Dutch) with undercut and copper loading, spit bite, aquatint dust box and melt (`--@ etch`) | L (two PRs: line etch, then aquatint) | P2 (P3b to print) | at fixed bath, depth ∝ time; +10 °C gives ×1.40 (Ea 24.9 kJ/mol); a 30 % bath is faster than 40 %; the undercut widens a line by 2·depth/K; the dust box's largest grain at a wait matches Stokes' formula; an aquatint's printed tone rises with bite then falls once undercut reaches the dot radius; stopped-out areas keep their earlier depth |
| **P5** | soft ground; `trace` onto plates; `impress`; ground left on the tracing sheet (`--@ soft-ground`) | M | P1, P4 | more pencil force bares more of the line, never more than its footprint; a thicker ground or a thicker tracing sheet bares less, and past a limit nothing; a fabric's weave appears in the bared share at its thread pitch; the study's back holds the ground the plate lost (a mass balance) |
| **P6** | several plates on one sheet: pins, trapped paper, offsets; à la poupée inks and their mixing band (`--@ registration`) | M | P3b | registration error within the pin slop on a covered sheet; β·ΔMC·L after a measured drying; two poupée inks mix only in a band about the face's contact wide, along the wipe |
| **P7** | states, editions, `pull{impression=}`, burr and aquatint wear, steel facing (`--@ edition`) | M | P3b (P4 for aquatint wear) | burred-line ink falls to e⁻¹ over N impressions, with N(steel)/N(Cu) equal to the wear law's hardness ratio; `pull{impression=12}` equals the 12th pull of `edition{}`; a named state reprints identically after later work |
| **P8** | monotype: ink painted on a plate, a ghost (`--@ monotype`) | S | P3b | the ghost has (1 − s) of the first pull's ink and the third (1 − s)² |
| **G4** | `trace` between sheets (gap 4, `--@ trace`) | S | P1 | a traced line follows the study's drawing guide, mirrored when asked; pressure widens it |
| **G5** | counterproof: a damp sheet pressed onto pastel (gap 5) | M | P1, P3a, P3b's press | it takes more loose pastel than bound; the original loses what the copy gains; the copy is mirrored |

**Order.**
- P1 → P2 → P3a (or alongside P2) → P3b. That is the first usable milestone: **a drypoint printed on damp laid paper**, which is Cassatt's late-1880s work.
- Then P4 (etching and aquatint) → P5 (soft ground).
- P6, P7 and P8 come after P3b in any order.
- G4 can go right after P1. G5 after P3b's press.

**Test discipline.**
- Every expected number in these tests comes from a formula in IP, computed in the test from the inputs, or from a measurement. None comes from the code's own output.
- Tests use tiny plates (20–60 mm) and sheets at 64–320 px, never whole prints (notes/agent_brief_template.md).
- None of this changes oil or pastel numbers, so no protected golden file moves (notes/golden_paths.txt).

## 12. Unknowns that need measurement

Ranked, from IP §9; each names what in the plan it would replace.
1. **Drypoint burr:** height and shape against force, angle and tip, and its decay over 30 pulls. It would replace the 0.75 × depth burr and the fitted N (§3.1, §8.3).
2. **The press's pressure under the felts,** over and beside the plate (Prescale), and the felts' compression curve. It would replace the felt modulus (§6.2).
3. **Transfer and plate tone by weighing** (plate inked, wiped, printed; damp and dry): the fill φ, the split s on damp paper, the plate-tone films (§5, §6.2).
4. **Etch rates by profilometry** for ferric (concentration, temperature, rocking), nitric on zinc and Dutch mordant, plus aquatint pit depth and dot loss. It would replace δ, the nitric and Dutch rates and K (§4.2).
5. **Damp paper:** MC as printed, z-compression and press-in against groove width, size against MC through a session, and stretch per pass (§6.1, §8.1).
6. Soft ground's thickness, yield stress and tack, and the bared share against pencil force (§7).
7. Rosin from a dust box: size distribution against wait, and dot spread on melting (§4.3).
8. Tarlatan's bending rigidity, yarn radius and spacing (§5.2).
9. From the literature, not the bench: Mathews & Shapiro (1989) for Cassatt's mordant, inks, papers, plate transfer and registration; Stijnman (2012) for 19th-century presses and felts.

## 13. Decisions (decided 2026-11-15: the most physical and realistic option each time)

- **I6 Roles: two sessions, artist and printer** (the woodblock plan's W1 pattern, two roles instead of three). Each is its own session and sees only what it is handed:
  - **The artist** gets the brief and makes the drawing, the plates (drypoint, grounds, acid, aquatint, soft-ground) and their states, and marks the proofs.
  - **The printer** gets the plates and the marked proofs. It inks and wipes (à la poupée included), damps the paper, registers, and pulls the trial proofs and then the edition (the recipe a `pull` replays).
  - **What passes between them is physical objects as files.** A plate leaves the artist with its whole state (height field, burr, grain, ground, any ink on it), and a proof leaves the printer with its paper, ink, moisture and plate mark.
  - **Time keeps running in transit.** The time elapsed between sessions is recorded with each handoff and applied on arrival: ink on a plate dries, damp paper dries, copper tarnishes if modelled.
  - **One session at a time** (RAM), as in the woodblock plan. Cassatt inked her color plates beside Leroy, so the printer session plays the hand that inks; the artist's marks on the proofs say how.
- **I2 Surfaces: named surfaces inside each role's session**, sharing that session's clock: for the artist the plates, studies and tracing sheets; for the printer the plates and sheets. P1 also builds the export and import of a surface as a file, with its age, which is the handoff in I6. The alternative (several sessions sharing one clock) is dropped: `press` needs the plate and the sheet in one session.
- **I1 Plate resolution: the sheet's pitch with the line geometry kept exactly** (the statistical plate). Wiping and transfer depend on each groove's width and depth (sag ∝ w⁴; transfer by depth), so each cell keeps its distribution of groove widths and depths and the vectors, not an average depth. A crop render rebuilds line shapes from the vectors. A finer grid would add memory without changing the physics the press sees.
- **I3 Ink: an oil film through the existing film code.** Etching ink is linseed oil (burnt plate oil) and pigment, so its drying, gloss and optics are the oil film's. The film memory on sheets is accepted.
- **I4 Metal optics: Fresnel from the metals' complex refractive indices now.** The closed form gives normal incidence and the angular dependence at once, so nothing waits for gap 10; gap 10 reuses it.
- **I5 Box: an 1890s Paris print box.** Copper, zinc, steel facing, papers (laid, Japan), inks and mordants of the period, refusing materials outside it, as woodblock's W2 does. Contents come from the sources in hand and are marked as unconfirmed until Mathews & Shapiro (1989) is read.
- **I7 Shared machinery: P1 surfaces, the handoffs and the edition machinery are built once** for intaglio and woodblock.
- **I8 Monotype brushes: the existing bristle model on the plate as a non-absorbent surface.**
- **I9 Bench measurements: nobody here can make them.** Phases ship with their [D] values derived from physics and their [E] values labelled. The tests read their expected numbers from the note's formulas, so a later measurement replaces a value without rewriting the code. Nothing waits for Unknowns 1–3.
