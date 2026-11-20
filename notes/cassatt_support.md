# Painting like Cassatt on the easel: what it supports and what it lacks

This note checks each material and technique in notes/research/cassatt_materials.md (the research) against the easel as of main 4bbbbec3: notes/easel_guide.md (the guide) and crates/easel/src, crates/paint/src. Each one is marked **yes**, **partly** or **no**. Gaps are ranked at the end by how much they matter for working as she did. Sizes: **S** is days, **M** is a week or two, **L** is a new medium.

## Being fixed separately (not proposed again here)

A painter working on a Cassatt-like pastel hit four problems. Each has its own fix under way:

1. **Pastel pencils can't be sharpened.** `p:sharpen()` refuses on every pastel, including `kind="pencil"`: crates/easel/src/draw_pencil.rs:408–410. Branch `fix/pastel-pencil-sharpen`.
2. **Rose madder is near-white as a dry pastel.** This matters for her pinks and flesh. Branch `fix/dry-lake-color`.
3. **No sanded or gritted paper.** This is her pumice-ground canvas and "gritty primers" (research §2). Branch `feat/sanded-paper`.
4. **The painter didn't know `lay_sheet` keeps pastel inside a shape.** The sheet catches strokes and crumbs over the mask (guide, Drawing: "The sheet keeps what it covers clean"; crates/paint/src/sheet.rs:313). This is how to keep a crisp edge against a reserve of bare paper.

## Oil painting

| technique (research §) | status | where |
|---|---|---|
| Canvas, ground of chosen color and texture (§1) | yes | guide, The canvas: `linen=`, `ground={...}` |
| Paint straight from the tube, mixed on the canvas wet into wet (§1) | yes | `pile{...}` with `medium` 0; wet paint under a new stroke comes up into it (guide, Time) |
| Blending with sweeps of a dry brush while wet (§1) | yes | `blend(m)` / `work{hand="blend"}` (a clean badger that lifts as it moves); a wiped `b:stroke` |
| Contours crisp in places and atmospheric in others; contours lost between figures (§1) | yes | `edge=` (`found` … `lost`), `lose(m, ...)` (guide, Covering an area) |
| Broken flesh of pinks and blues (§1) | yes | `work{piles=...}`, `stipple`, double loading (`b:load(..., {side=, share=})`) |
| Reworking: painting over, scraping, wiping (§1) | yes | knife `k:scrape`, `rag()`; dry paint takes new paint on top |
| Lean matte paint: blotted and turpentine-thinned (peinture à l'essence; uncertain for her, §1) | yes | `pile{..., blot=, turps=}` in one pile: crates/easel/src/api.rs:2137 accepts both (only blot with medium is refused); the guide's blot and turps paragraphs |
| Oil halos around thin paint on paper | no | thin oil on paper goes lean and matte (guide, Paper) but doesn't wick sideways; crates/paint/src/soak.rs is raw cloth and doesn't wick either |
| Her pigments (§1: none analyzed) | partly | `EASEL_BOX=impressionist` (crates/paint/src/palette.rs:698) is the best estimate; no "cassatt" box |
| Unvarnished matte or varnished finish | yes | the gloss model (guide, Gloss); `easel finish --no-varnish` |

## Pastel

| technique (research §) | status | where |
|---|---|---|
| Colored papers: tan, buff, brown, green, blue (§2) | yes | `paper={tone=...}` (guide, Paper) |
| Wove and laid sheets | yes | `paper={laid={...}}`; default wove |
| Paper laid on board, or on canvas over a strainer (§2) | partly | only the give, through `paper={stiffness=}` (crates/easel/src/api.rs:2612); no mount preset; no numbers for what each mount does |
| Pumice-ground canvas, gritty primers (§2) | no | being fixed (item 3 above) |
| Japan paper (counterproofs, §2) | partly | the fiber dials could approximate a thin long-fibered sheet; no preset, no research |
| Faded or aniline papers | no | uncertain for her (research §2); not needed yet |
| Soft and hard sticks (§2) | yes | `pastel(pile, {kind="soft"|"hard"})` |
| Pastel pencils | partly | `kind="pencil"`, but no sharpening (item 1 above) |
| Dry colors of her pinks (§2) | partly | rose madder dry near-white (item 2 above); other lakes not checked |
| Single strokes, side strokes, a point for features (§7) | yes | `p:stroke`, `p:side`, `p:roll`; facets wear (guide, Pastel) |
| Parallel hatching, graded (§7) | yes | `p:hatch(m, {angle=, spacing=, length=, pressure=, graded=})`: one fixed angle per call (crates/easel/src/draw_pencil.rs:370) |
| Slashing diagonals and zigzags over a dress, strokes following a direction field (§7) | partly | by hand with loops of `p:stroke`; no zigzag stroke and no `angle=function(x, y)` for pastel hatching (gap 2) |
| Unblended color over a base (blue over pink, §7) | yes | strokes over strokes; the tooth fills (guide, Pastel) |
| Blending and stumping shadows, finger (§7) | yes | `smudge(pts|mask, {pad=})`; a stump is `pad` ≤ 60 mm² |
| Faces finished and edges open over bare paper (§7) | yes | it is a matter of how much is covered; `lay_sheet` for a clean reserve edge (item 4) |
| Fixative between layers, or none (§2) | yes | `fix(mask)`: a resin spray at a fixed wetting of 0.15 (crates/easel/src/draw_pencil.rs:524; crates/paint/src/pastel.rs:699) |
| Eraser, dry brush, knife, puff and tap on pastel | yes | `erase`, `b:dust`, `k:scrape{edge=}`, `blow`, `tap` |
| Pastel mixed with water: a wet stick, a brush through dry pastel, a paste (§2) | **no** | gap 1 |
| Moisture vapor; crushed pastel sifted onto a damp sheet (§2) | **no** | gap 6 |
| Pastel over set oil | yes | pastel skips wet paint, so let it set first (guide, Pastel) |
| Pastel over gouache or distemper; with metallic paint (§2, §3) | **no** | gaps 8–10 |
| Counterproofs (§2, §6) | **no** | gap 5 |

## Gouache, distemper, watercolor (§3)

| technique | status | where |
|---|---|---|
| Watercolor washes on paper | no | no water medium in crates/paint; plan in notes/plan-edition-2.md §6; physics in notes/research/watercolor_physics.md |
| Gouache (opaque, matte body color) | no | as above; no research note |
| Distemper (glue-bound, matte) on canvas | no | no research note |
| Metallic (gold) paint | no | no metal optics; the gloss model is dielectric |
| Soak on raw canvas | not hers | `canvas{raw=}` (crates/easel/src/draw_soak.rs, crates/paint/src/soak.rs) is oil on bare cloth (notes/research/soak_stain.md), not a water medium |

## Drawing (§4)

| technique | status | where |
|---|---|---|
| Graphite, any grade | yes | `pencil("2B")` (guide, Drawing) |
| Black chalk | yes | `chalk()` (crates/paint/src/graphite.rs:118) |
| Vine and compressed charcoal | partly | no charcoal medium: `chalk()` or a vine-black `pastel` stand in; neither lifts and smears as charcoal does (gap 11) |
| Many studies before the work | partly | the scratch canvas (guide, The scratch canvas), but one at a time and nothing carries over (gap 4) |

## Printmaking (§5)

| technique | status | where |
|---|---|---|
| Soft-ground etching from a traced drawing; fabric texture | no | gap 3 |
| Drypoint with burr | no | gap 3 |
| Aquatint tone and color | no | gap 3 |
| Two or three plates inked à la poupée, registered with pins, printed on damp laid paper, edition of 25, states | no | gap 3 |
| Monotype | no | gap 12 |
| Relief (woodblock) | planned, not hers | notes/plan-woodblock.md on branch `notes/woodblock-plan` (draft 1): relief inks, baren, edition and roles. Intaglio needs its own physics, though the edition, state and registration machinery could be shared |

## Transfer (§6)

| technique | status | where |
|---|---|---|
| Squaring a drawing up | yes | `look` with `grid:` (guide, Looking) |
| The drawn lines as a mask on the same canvas | yes | `drawing_guide()` |
| Tracing from a study onto the work or a plate; mirroring | no | gap 4 |

## The gaps, ranked

The ranking is by how much each gap matters for working as she did. Her output was about 320 pastels, 320 oils and 215 prints (research, intro). Oil is nearly complete today, so the pastel gaps lead. Printmaking is the biggest single missing medium, but it is a separate track.

### 1. Pastel with water (M)
- **Why first.** She painted areas of her finished pastels in "pastel chalk mixed with water" (research §2).
- **What it would model.** Three gestures on the existing pastel cells (`pastel.rs`: loose and bound pastel in the tooth):
  - a stick dipped in water: `p:wet(share)`, giving a creamy, saturated, heavier stroke that wears faster and sheds less
  - a wet brush drawn through laid pastel, turning loose pastel into a paste that moves with the brush and dries partly bound
  - ground pastel mixed with water as a paste or wash, laid with a brush
- While wet, the pastel darkens because the water index-matches the particles. Dry, it is lighter than wet but darker and denser than dry pastel, and partly bound: it takes the finger less and takes new pastel more like a fixed crust.
- **Dependencies:**
  - a water state on paper cells with an evaporation clock, a small subset of plan-edition-2 §6.2's
  - a brush that carries water
  - the paper's absorbency (`paper={absorbent=}` is oil-only today)
- **Research.** No measurements exist (as for pastel itself). Collect what the Degas literature says (notes/degas-materials.md §5) and gum and filler contents. Estimate how wet-and-dried pastel's scattering and binding change. Paper drying times are in notes/research/watercolor_physics.md §2.

### 2. Pastel strokes over an area: diagonals, zigzags, direction fields (S)
- **Why.** Her garments and backgrounds are "slashing diagonal strokes" and loose zigzags over the colored paper (research §7). Today that is a Lua loop over `p:stroke`.
- **What it would model:**
  - `p:hatch` taking `angle=function(x, y)`, as `work` does
  - a `zigzag=` option (amplitude, period, corner rounding) for one continuous stroke
  - `jitter` in length and angle
  - `open=` leaving gaps to the paper in proportion to a mask
- Each mark is still one `p:stroke` with force, altitude and speed, so wear and crumbs stay physical.
- **Dependencies:** none. crates/paint/src/graphite.rs `hatch_marks_graded` is the model.
- **Research:** stroke lengths, angles and spacings measured on reproductions of her pastels of 1890–1910 (an estimate from images, so marked as one).

### 3. Intaglio printmaking: drypoint, soft-ground, aquatint, color printing (L, in phases)
- **Why.** About a third of her work. The color prints of 1890–91 are among her best-known works.
- **What it would model.** Phases, each usable alone:
  1. **The plate.** A copper height field. The **drypoint** needle cuts a groove and throws up a burr whose height follows pressure. The burr holds ink and wears down impression by impression.
  2. **Grounds and acid.** Hard ground and etching needle. **Soft ground** that lifts wherever a traced line or pressed fabric touches it (pressure and pencil width set the line). An acid bath whose bite deepens with time. **Aquatint**: a rosin grain, bitten for a tone, with stopping-out between bites.
  3. **Inking and wiping.** Ink in the grooves and grain, plate tone left by the wipe. **À la poupée**: several inks dabbed into areas of one plate, bleeding a little where they meet (the blue in the flowers, research §5).
  4. **The press.** Damp paper pulls ink from the grooves under pressure and embosses a plate mark. Several plates print on one sheet, registered by pins with a small error.
  5. **States and editions.** A plate reworked between proofs. An edition of 25 with burr wear and hand-inking variation.
- **Dependencies:**
  - `paper.rs` with a damp state (shared with gap 1)
  - KM ink optics (exist)
  - tracing (gap 4)
  - several objects in one studio (plates, sheets)
  - the edition, state and role machinery planned for woodblock (notes/plan-woodblock.md §4)
- **Research:** a new note on intaglio physics. It needs:
  - ink rheology and wiping
  - drypoint burr height and how many good impressions it gives
  - aquatint grain size against tone
  - soft-ground line quality
  - plate pressure and paper dampness
  - Mathews & Shapiro (1989) for her plates, inks and states

### 4. Tracing and transfer between sheets (S–M)
- **Why.** She moved figures from studies to pastels, plates and counterproofs (research §6). The easel has one painting and a scratch canvas, and nothing carries across.
- **What it would model:**
  - studies kept as named sheets beside the painting
  - `trace(study, {pressure=, tool=, mirror=})`: the study's drawn lines (`drawing_guide()`), transferred as a faint line or as a guide mask onto the painting or a plate
  - the traced copy's line quality following the tool and pressure, as a soft-ground line does
- **Dependencies:** more than one canvas in a session (the scratch canvas is a start). Old logs are unaffected, since this is a new verb.
- **Research:** little. Tracing-paper transparency and line loss are estimates.

### 5. Counterproofs (M)
- **Why.** About fifty are known, 1889–1913, most on Japan paper (research §2).
- **What it would model:**
  - `counterproof{paper=..., damp=, pressure=, by="rub"|"press"}`
  - a damp sheet pressed onto the pastel takes a share of the loose pastel, more where it is heavy and loose, little where it is fixed or bound
  - the result is a new, mirrored picture: flatter and softer
  - the original is left lighter
  - the counterproof can then be worked in pastel
- **Dependencies:**
  - sheet.rs already lays a surface over the picture that catches pastel
  - a second picture (gap 4)
  - damp paper (gap 1)
  - a Japan-paper preset
- **Research:** transfer fraction against dampness and pressure (none measured; conservation literature on chalk and pastel counterproofs); Japan paper fiber, grammage and absorbency for the paper dials.

### 6. Moisture vapor and sifted pastel (M)
- **Why.** In a 1914 pastel she sifted crushed pastel onto a surface dampened by vapor, so that it dried in minute clusters (research §2). Degas's steaming is the same tool (notes/degas-materials.md §5).
- **What it would model:**
  - `steam(mask, {seconds=})`: wets the paper and the pastel's surface (the water state of gap 1), so loose pastel softens and partly binds as it dries
  - `sift(pile or stick, mask, {amount=, grain=})`: drops crushed pastel as crumbs with no stroke. On dry paper most stays loose and comes away with `tap` or `blow`. On damp paper it sticks and agglomerates into clusters.
- **Dependencies:** gap 1's water state; the crumb model in pastel.rs (`lay_crumbs`).
- **Research:** the Met article's description of its mock-up; Degas steaming accounts; how particle clusters form as a damp layer dries (estimates).

### 7. Mounts and paper presets for her supports (S)
- **What it would model:**
  - `paper={mount="board"|"canvas"|"strainer"}`, setting the give (`stiffness`) and the drum of a sheet on canvas over a strainer
  - named presets for her sheets: blue, tan, green and brown wove; laid; Japan paper
- **Dependencies:** none; paper.rs already has the dials.
- **Research:** compressibility of paper on board against paper on canvas (estimates); her paper tones measured from museum images (estimates).

### 8. Gouache and watercolor (L)
- **Why.** She used them under and with pastel in 1878–80, and for a few self-portraits.
- **What it would model:** plan-edition-2.md §6 as planned: water and pigment on paper, edges, lifting and rewetting. Gouache is the high-pigment, opaque, matte case: it dries lighter and rewets.
- **Dependencies:** pigment identity, the evaporation clock and rewetting (plan-edition-2 §6.2).
- **Research:** watercolor_physics.md exists. A gouache note is needed: pigment volume, matte scattering, dry shift, rewetting.

### 9. Distemper (M, after 8)
- **Why.** She made two paintings in glue-bound paint (1878–79).
- **What it would model:** pigment in warm glue size, which gels as it cools and dries fast, matte and much lighter than wet. It is laid with a brush on canvas, and pastel can be worked on it.
- **Dependencies:** gap 8's water and drying machinery.
- **Research:** a distemper note. The Degas and Cassatt détrempe of 1879 is in notes/degas-materials.md §3–4. Glue-size rheology and the dry shift need sources.

### 10. Metallic paint (M)
- **Why.** She used gold, and perhaps silver, in 1878–79 works in distemper, gouache and pastel.
- **What it would model:** metal-powder paint whose reflectance is specular and colored and changes with the light's angle (the `look` light and the gallery mode). KM can't do this alone.
- **Dependencies:** a binder (gum or glue from gaps 8–9, or oil, which exists); a metal reflectance model in the looks.
- **Research:** what "gold" paint was in 1879 (bronze powder against shell gold); flake size and orientation against sheen.

### 11. Charcoal (S–M)
- **Why.** She drew studies and underdrawings in charcoal, including compressed charcoal.
- **What it would model:** vine and compressed charcoal as drawing media like `chalk()`. Charcoal is looser and lifts and smears more. Fixative darkens it. Compressed charcoal is darker and binds more.
- **Dependencies:** graphite.rs (the `Lead` dials).
- **Research:** charcoal on paper: deposit, smearing and erasing (practitioner sources, estimates).

### 12. Monotype (M, after 3)
- **Why.** Only one instance is known for her (research §5).
- **What it would model:** ink painted or wiped on a plate and printed once, with a faint second pull.
- **Dependencies:** gap 3's inking and press.

### 13. A Cassatt tube box (S, research first)
- **Why.** No analysis of her oils exists (research §1). Today `EASEL_BOX=impressionist` is the honest choice.
- **What it would take:** a named box only once a source gives her pigments: the NGA's analysis of her three pastel boxes (planned 2014), the Getty SERS study, the PMA 2024 catalogue.
- **Research:** find and read those three.

## How to paint a Cassatt pastel today

The best available today:
- **Paper:** `canvas{paper={tone=..., stiffness=...}}` with a blue, tan or green tone.
- **Box:** `EASEL_BOX=impressionist`.
- **Faces:**
  - a pencil or `chalk()` drawing
  - soft-stick strokes with a point for features, blended with `smudge` and a stump (`pad=`)
  - fixed with `fix(mask)` if more is to go on top
- **Dresses and backgrounds:** looped `p:stroke` diagonals and `p:hatch` at a few angles, left open over the paper.
- **Reserves:** `lay_sheet` for a crisp edge against bare paper.
- **Oil:** `blot=` with `turps=` for a lean lay-in under the pastel, let set before the pastel goes on.
- **Not possible yet:** wet pastel, vapor, sifted pastel, gouache, metallic paint and counterproofs.
