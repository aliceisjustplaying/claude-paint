# Batch B: lessons from the working journals

## Summary

The most persistent problems were wet corrections that picked up the underlying paint, thin films judged as if they were opaque and local repairs that introduced larger seams or halos. Reported improvements came from waiting for a stable underlayer, rebuilding a coherent passage and reducing repeated marks. Several confident completion claims were contradicted by the next sitting. These are journal observations, not an independent assessment of the images (paint-studio-f705c2:8–13; paint-studio-c766c5:9–12; paint-studio-58dfc6:29–34).

All 20 journals in [batch B](~/src/a/claude-paint/notes/journal-mining/batch-B.txt) were read completely. They cover r18, r18g, r19 and r20, all marked **engine 1** in that list. Exact loads, coverage values, wait lengths and recipes below are historical conditions, not current defaults. “Confirmed” means a later journal entry supports the observation; it does not establish physical accuracy or visual success. Current applicability is checked only against the four prompt documents named in the extraction brief.

The current brief already asks for whole-picture inspection, comparable material trials and reconsideration when a repair recreates its problem (Brief:35–39). The most useful additions from this batch would make those requirements concrete: protect every foreground mark, distinguish stroke count from hiding and check whether a repair improves the picture at its normal viewing scale (paint-studio-520026:27–32; paint-studio-58dfc6:14,29–34).

The lessons are practical interpretations of the cited observations. Their broader transfer and the final ranking are interpretations, not additional experiments.

### Source key

Journal citations use `paint-studio-<id>:<line>`. Each resolves to `~/src/a/paint-studio-<id>/notes/journal.md`. Ranges include both endpoints. Lane labels identify sources only.

| Studio | Round / lane | Engine | Journal lines |
|---|---|---|---:|
| paint-studio-2564f4 | r18 / MUSE | 1 | 10 |
| paint-studio-c71699 | r18g / GEM | 1 | 15 |
| paint-studio-f705c2 | r18 / MIMO | 1 | 13 |
| paint-studio-3f743e | r18 / GLM | 1 | 14 |
| paint-studio-db6324 | r19 / LUNAB1 | 1 | 5 |
| paint-studio-f0e4ab | r19 / LUNAF1 | 1 | 10 |
| paint-studio-520026 | r18 / BUN | 1 | 51 |
| paint-studio-c766c5 | r18 / DSK | 1 | 12 |
| paint-studio-69c343 | r19 / GEMB1 | 1 | 86 |
| paint-studio-851a37 | r19 / GEMF1 | 1 | 52 |
| paint-studio-2a634e | r19 / MUSEF1 | 1 | 15 |
| paint-studio-11997b | r19 / MIMOF1 | 1 | 4 |
| paint-studio-ebf843 | r19 / CTEST1 | 1 | 8 |
| paint-studio-919cfd | r19 / KIMIF1 | 1 | 6 |
| paint-studio-58dfc6 | r19 / SONF1 | 1 | 34 |
| paint-studio-6e8898 | r19 / SELF1 | 1 | 35 |
| paint-studio-bfd78b | r19 / SONB1 | 1 | 11 |
| paint-studio-b3a3c7 | r19 / SELFH1 | 1 | 5 |
| paint-studio-3cab71 | r20 / NEW1 | 1 | 7 |
| paint-studio-e708c7 | r19 / NEWF1 | 1 | 17 |

Prompt citations use these fixed documents:

- **Guide**: [current easel guide](~/src/a/claude-paint-r31run/notes/easel_guide.md).
- **Techniques**: [current craft notes](~/src/a/claude-paint-r31run/notes/techniques.md).
- **Studio**: [r31 studio observations](~/tmp/gallery-fcf9c110/r31/studio_notes.md).
- **Brief**: [r31 brief draft](~/src/a/claude-paint/notes/round31/BRIEF-draft.md).

### Time sinks, rescue cycles and reported changes of approach

| Cycle | Recorded cost | What changed / outcome |
|---|---|---|
| Repairing a boat on repeatedly overpainted water | Eight attempts reported by day 17; painting finished day 23 | The boat was removed so the light path could carry the picture; the final entry reports a coherent dusk. This is a reported compositional recovery, not proof that the last hull technique worked (paint-studio-3f743e:12–14). |
| Rebuilding still-life fruit and fields | Completion claims on days 31, 33 and 36 were rejected; repaint still under way on day 39 | Full loads and opaque bases addressed hiding, but did not establish lasting modeling. The journal ends with a reported improvement to cloth and another handling warning, not a final review (paint-studio-520026:7–10,23–24,37–51). |
| Soft fruit gradients that became cloudy blobs | Four campaigns, reaching day 112 | Dry bases with stronger shapes improved volume, then soft lit masks used as clips reportedly removed concentric seams. No later sitting verifies the final method (paint-studio-c766c5:9–12). |
| Lagoon, foreground exclusions and trunk-foot repairs | A “final” on day 353 reopened; last painted state day 472 | Water became calm after reducing flecks. The foot cycle generated bushes, skirts, wedges and puddles; a plain trunk and matched bank body coat reduced them. A return with no painting accepted the remaining small flaws (paint-studio-58dfc6:17–18,28–34). |
| Building a cumulus from repeated discs and value zones | “~300 chunks of fighting cumulus,” reset at day 426 | The painter chose tapered bar clouds and explicit water strokes after repeated failures. The journal ends at the new plan, so there is no evidence that the final reset succeeded (paint-studio-bfd78b:7–11). |
| Rebuilding the sky, trees and tomb after geometric artifacts | Completion claims on days 12 and 37; further resets planned through day 75 | Foliage was reduced, expanded, then removed again. A later return found the essential trees absent. The journal ends with another full rebuilding plan, not confirmation that it succeeded (paint-studio-851a37:12,19–23,30–52). |
| Refining stones and path without changing the composition | Initial finish day 90; later campaigns through day 248 | Broad wet films replaced conspicuous small-brush starts, then dry fine marks restored restrained grain. Later entries report quieter stones and removal of white path beadwork while preserving the large intervals (paint-studio-e708c7:6–17). |

## Ground and drawing

### G1. Use the toned ground as a deliberate part of the picture

- **Lesson:** Leave a suitable toned ground visible where it already supplies the intended middle value.
- **Kind:** craft.
- **Conditions:** Engine 1; warm umber/ochre portrait ground left bare as flesh halftone; warm grounds beneath thin still-life paint.
- **Evidence:** The portrait retained the ground “as the flesh halftone” and said it “saved a lot of paint” (paint-studio-6e8898:10–11). A still-life journal observed that its warm ground “glows through the darks” (paint-studio-520026:1–2). Neither includes a controlled comparison with a different ground.
- **Already in the prompts?** Yes: Techniques:28–30 describes toned grounds; Studio:14–15 says they show through thin paint.
- **Still valid?** The general role of visible ground is supported by Guide:53–58 and Studio:14–15. This batch does not test raw cloth or absorbent grounds, so it supplies no lesson about those later materials.

### G2. Judge the underdrawing at the scale where it will be used

- **Lesson:** Check a light underdrawing in detail before increasing its strength merely because it disappears in the whole view.
- **Kind:** process.
- **Conditions:** Engine 1; H-pencil drawing on warm cream ground beneath a thin landscape lay-in.
- **Evidence:** The painter reported that an H-pencil drawing was “nearly invisible at whole-canvas view, fine as a guide” (paint-studio-58dfc6:1–2). Other journals record pencil/chalk layout but no observed advantage from making it darker (paint-studio-c71699:3; paint-studio-851a37:5–6).
- **Already in the prompts?** Partly: Guide:457–461 explains pale hard leads and drawing visibility through paint; Guide:619–623 distinguishes whole views from detail. The practical check is not explicit.
- **Still valid?** Both visibility mechanisms remain documented. No exact pencil pressure or visibility threshold can be carried forward from this evidence.

## Color mixing

### C1. Judge a recipe as a laid film over its intended base

- **Lesson:** Test a mixture over the actual kind of underlayer before repeating it, because the same pile can read differently as a thin film.
- **Kind:** easel.
- **Conditions:** Engine 1; earth mixtures over pale fruit bases versus dark ground; warm cream ground under smalt 3 : cobalt 1 : white 1.2.
- **Evidence:** One painter found the same pile “much lighter and more saturated” over pale cloth than dark ground (paint-studio-520026:27). Another identified thin earth films over pale bases as a cause of weak fruit shadows, with dry stronger shapes later improving volume (paint-studio-c766c5:9–12). The blue recipe read as mid periwinkle rather than dark on cream (paint-studio-58dfc6:2).
- **Already in the prompts?** Yes: Brief:39 explicitly requires substrate-matched trials; Studio:24–35 distinguishes thin films from pile appearance; Guide:97–102 explains dependence on the canvas beneath.
- **Still valid?** Supported qualitatively by the current documents. Historical color outcomes and coverage thresholds need fresh trials.

### C2. Verify that a named “dark” is actually dark enough

- **Lesson:** Inspect the value of a proposed dark over its intended passage instead of trusting the pigment name or the name of the pile.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; umber/white and brown piles used for still-life background glazes; pale bases under earth shadows.
- **Evidence:** A painter nearly lost the background twice because piles thought dark were “mid-browns, not darks” (paint-studio-520026:5). Its later “raw umber is a LIGHT earth” rule was an interpretation of those recipes, not an isolated pigment test (paint-studio-520026:16,31). Fruit shadows also needed several full passes over a pale base (paint-studio-c766c5:9).
- **Already in the prompts?** Partly: Techniques:47–49 calls for a real value range; Techniques:105–109 calls for visual mixing. The pile-name trap is not stated.
- **Still valid?** The visual check is supported by Guide:97–102. Do not promote “umber is always light” or “add black” to a universal recipe; the journals did not isolate pigment, thickness and substrate.

### C3. Rebuild a failed warm passage in the intended color family

- **Lesson:** When repeated cool knockbacks destroy a warm passage, restate its warm body color before adding the light again.
- **Kind:** process.
- **Conditions:** Engine 1; dusk cloud repaired with umber/cobalt gray; warm haze and neighboring dark treeline still wet.
- **Evidence:** Gray overpainting turned “grey-violet-mauve”; the painter diagnosed that the amber cloud needed “a patient opaque rebuild in amber tones” (paint-studio-3f743e:6–7). The next treeline attempt picked up pale paint and needed a wait (paint-studio-3f743e:8). The final entry reports warm haze, but does not isolate the success of that particular recipe (paint-studio-3f743e:14).
- **Already in the prompts?** Partly: Techniques:105–109 covers adjusting mixes by eye; Brief:37 requires changing a failing method. This specific repair sequence is absent.
- **Still valid?** A process candidate, not a current pigment law. Guide:97–102 and 614–615 support checking both the new mixture and the wet base.

### C4. Clean the brush when a new color must remain distinct

- **Lesson:** Wipe or reload before switching to a clean color rather than adding the new pile to a contaminated brush.
- **Kind:** easel.
- **Conditions:** Engine 1; self-portrait palette dabs and successive color loads; load amounts unspecified.
- **Evidence:** The journal explicitly distinguishes additive `:load()` from `:reload()` or `:wipe()`, reporting that later dips otherwise became muddy (paint-studio-b3a3c7:3). No later entry contradicts it.
- **Already in the prompts?** Yes: Guide:219–222 defines the operations; Guide:230–232 says brushes retain paint across strokes and chunks.
- **Still valid?** Supported directly by the current guide. Preserve this as a handling reminder rather than a new requirement to clean between every stroke.

## Thin vs thick paint

### T1. Coverage is not an opacity setting

- **Lesson:** If an old color still shows, inspect the film and loading before treating a higher coverage number as guaranteed hiding.
- **Kind:** easel.
- **Conditions:** Engine 1; saturated vermilion beneath repaint; large filbert passes with varying dip intervals and loads.
- **Evidence:** The painter wrote “COVERAGE IS NOT OPACITY” after coverage 2–3 still gave a veil; it reported needing two passes at 8–10 to bury an old strong color (paint-studio-520026:27). A later three-pass near-black repair at coverage 5 still followed a single pass that left vermilion ghosting (paint-studio-520026:49). Those numbers did not prevent later rejection of the overall picture (paint-studio-520026:37).
- **Already in the prompts?** Partly: Guide:321 defines coverage as average layers; Studio:32–35 relates hiding to thickness. The explicit distinction and repair diagnostic are absent.
- **Still valid?** The distinction is documented. The old numerical “solid coat” recipes are not established current defaults.

### T2. Shorten or reload a starving stroke before covering it repeatedly

- **Lesson:** When a pass leaves thin streaks, try a shorter stroke or more frequent full reloads before adding another broad pass.
- **Kind:** easel.
- **Conditions:** Engine 1; large still-life fields; filbert 20, lengths 40–110, coverage 2.4 versus filbert 12, lengths 24–64 and `dips={4,1.0,0.1}`; a later recipe used one full dip per stroke.
- **Evidence:** A large pass laid a “thin veil”; the smaller, shorter alternative reportedly gave a solid coat (paint-studio-520026:17). Later the painter attributed starved streaks to three-stroke dip intervals and preferred `dips={1,1.0,0.0}` (paint-studio-520026:28). This revised the earlier recipe; it did not prove a single universal interval.
- **Already in the prompts?** Yes for the mechanism: Studio:46–48 describes load running out along a long drag; Guide:324–325 documents dip controls. Partly for the troubleshooting sequence.
- **Still valid?** Current guide supports brush-load depletion (Guide:230–232). Trial the local scale rather than copying the old settings.

### T3. Use gap filling only when its dabs suit the surface

- **Lesson:** Check the texture left by `fill=true` before using it to make a quiet surface or a small accent.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; broad background fills over dark ground and very small highlight masks.
- **Evidence:** `fill=true` helped cover a hole-filled jug (paint-studio-f705c2:2,4), but another journal called its background results “leopard spots” and small highlights “cottage cheese” (paint-studio-520026:2,29). Its last entry reports a lozenge/scale texture reduced by one blend (paint-studio-520026:51). These different outcomes argue against either always enabling or always disabling fill.
- **Already in the prompts?** Partly: Guide:331 and Studio:75–77 explain filling bare spots with dabs; the observed texture risk is not explicit.
- **Still valid?** The dab mechanism remains documented. The exact old texture and the claim that one blend is the “ONLY” cure are not verified for the current engine.

### T4. Establish a quiet body plane before adding broken surface marks

- **Lesson:** If a calm plane has become patchy, rebuild its continuous body color before returning restrained texture or glints.
- **Kind:** craft.
- **Conditions:** Engine 1; architectural exterior, window interiors, portrait-sized flesh planes and landscape stones; opaque body coats over dry paint or broad wet films before fine dry marks.
- **Evidence:** Dry body coats made exterior planes calm (paint-studio-3cab71:2). A broken cheek scumble was replaced by “a clean flesh plane and three restrained values” (paint-studio-3cab71:6–7). Broad stone films removed conspicuous starts and heavy scratching, with later fine grain retained (paint-studio-e708c7:12–14). These improvements have later supporting observations.
- **Already in the prompts?** Partly: Techniques:62–69 asks for a hierarchy of marks; Brief:37 asks for a method change after recurring damage. The base-then-texture repair is not explicit.
- **Still valid?** A journal-supported process option. Current Guide:261–266 distinguishes body and fluid leveling, but does not certify these exact recipes.

## Blending and edges

### B1. Stop blending when the underlayer starts showing through

- **Lesson:** Treat a pale weave lattice after blending as a reason to restore the film, not as a reason to blend again.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; lean layers over pale ground; repeated clean-brush blending.
- **Evidence:** Two blends lifted a lean layer and left “a pale lattice” (paint-studio-f705c2:2). Another journal reported pale blobs and eventually preferred one blend over a fuller base (paint-studio-520026:20,28). The lagoon painter accepted remaining weave-dot speckle after repeated blending rather than repair it again (paint-studio-58dfc6:25,28,34).
- **Already in the prompts?** Yes: Studio:134–136 describes lifting and spreading thin spots; Guide:314 quantifies the current blender's lifting.
- **Still valid?** Supported directly. The journals' “one” or “at most twice” rules are useful local restraint, not a validated maximum for every film.

### B2. Avoid a blend boundary cutting through a continuous passage

- **Lesson:** If a local blend leaves a ring or rectangle, rebuild across the affected transition instead of making another smaller patch inside it.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; wet sky, clouds and modeled fruit; bounded rectangular, disc or shrunk-object blend masks.
- **Evidence:** Cloud blending left “halo rings where blended meets unblended sky” and prompted a fresh sky pass (paint-studio-2564f4:3). A rectangular sky blend created cut edges, followed by full-width repaint after drying (paint-studio-851a37:13–15). A later sitting still found cut boundaries, so the first rescue was not permanently successful (paint-studio-851a37:23–25).
- **Already in the prompts?** Yes for the risk: Studio:126–130 describes local blend seams. Partly for repairing the full transition.
- **Still valid?** The seam warning remains current. Neither a soft mask nor a full-width pass guarantees success; it must also protect neighboring objects and use suitable paint.

### B3. Preserve distinct values when blending neighboring objects

- **Lesson:** Blend the transitions within an object without running the blender across adjacent objects whose colors or values must stay separate.
- **Kind:** easel.
- **Conditions:** Engine 1; touching wet fruit and adjacent blue/pale water bands.
- **Evidence:** Whole-group blending dragged one fruit's color onto another (paint-studio-520026:21). Water blending averaged lower blue bands into pale ones, requiring another dry-ground campaign (paint-studio-58dfc6:15). The painter later reported calm water after rebuilding and reducing marks (paint-studio-58dfc6:18,28).
- **Already in the prompts?** Partly: Studio:89–100 explains wet mixing; Studio:128–130 says a soft mask does not prevent dark and light mixing. The object-by-object reminder is absent.
- **Still valid?** The mixing mechanism is current (Guide:614–615). This is about protecting intended separation, not prohibiting every blend across a planned gradient.

### B4. Choose between a clean boundary and a varied contour deliberately

- **Lesson:** Use clipping where paint must stop, then inspect whether the resulting contour needs separate lost or softened stretches.
- **Kind:** process.
- **Conditions:** Engine 1; fruit silhouettes, foreground object restorations and portrait garment/skin boundaries.
- **Evidence:** Clipping cleaned a hole-filled jug contour (paint-studio-f705c2:4) and bounded foreground restorations (paint-studio-69c343:48–49). Hard clipped fruit later read as stickers, leading another painter to reject that contour treatment (paint-studio-520026:8,23). The portrait journal softened accent masks before clipping after cheek “stickers” (paint-studio-6e8898:12–20).
- **Already in the prompts?** Partly: Guide:340–358 distinguishes clipping from edge handling; Studio:62–74 describes exact clipped outlines. The tradeoff is already observable but could be stated together.
- **Still valid?** Both operations remain documented. “Never clip fruit” and “clip almost always” are local judgments, not general laws.

### B5. Finish broad blending before repainting crisp silhouettes

- **Lesson:** Complete broad wet transitions before restating the fine silhouettes they could smear or veil.
- **Kind:** process.
- **Conditions:** Engine 1; sky/fog/water neighboring fresh trees, shorelines, reeds and figures.
- **Evidence:** A glow blend dragged a newly painted shore into gray; a later water blend disturbed a wet bank despite its exclusion mask (paint-studio-58dfc6:3–4). Another painter lost an oak crown to a band blend and paid a day of repair (paint-studio-919cfd:5). Recutting a crown on dry sky with “no blending after” was later retained in a broader final state (paint-studio-2a634e:11,15).
- **Already in the prompts?** Partly: Techniques:125–128 covers wet/set sequencing; Studio:89–100 covers pickup. Brief:35 requires campaign sequencing but does not spell out the silhouette order.
- **Still valid?** Supported by current wet-paint behavior. Historical clearance rules such as “~25 units” or “>8 units” are not safe current distances (paint-studio-58dfc6:4,11; Guide:314,340–348).

## Glazing/scumbling

### Z1. Distinguish a tinting glaze from a pale covering veil

- **Lesson:** When a luminous passage has become chalky, trial a pigment-rich transparent glaze rather than adding another white-rich veil.
- **Kind:** easel.
- **Conditions:** Engine 1; lead-white mist/glow glazes over a dark far shore; chrome-yellow/vermilion warm glaze with abundant medium over the horizon.
- **Evidence:** White veils made a “flat cream slab” where dark shore should remain (paint-studio-58dfc6:5). Warm gold/orange glazes “worked far better than the white veils” but also washed out projecting ruin and fir shapes (paint-studio-58dfc6:6). The overall warm sky survived later reviews (paint-studio-58dfc6:28,34).
- **Already in the prompts?** Yes: Studio:32–35 distinguishes lead-white veiling; Techniques:129–130 warns about white; Techniques:170–174 describes transparent glazes.
- **Still valid?** The distinction is current. The historical warm recipe is not a general transparent-glaze recipe; the current notes explicitly tie glazing to pigment transparency and a dry base.

### Z2. Test the laid glaze and its reach before expanding it

- **Lesson:** Inspect a small glaze over a comparable dry passage before assuming that its name, medium share or mask will make it quiet and local.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; glaze hands over broad tonal fields and object shadows; reported medium 0.55–0.62 in one still life; loads often unspecified.
- **Evidence:** Glazes unified speckled fields in one early finish (paint-studio-f705c2:6–7), but later inspection exposed halos and gray veils requiring opaque repaint (paint-studio-f705c2:8–11). Other broad glazes left parallel streaks rather than a stable body shape (paint-studio-520026:51). Pale window glazes made unwanted lacy high-point marks and were replaced with room colors and sparse vertical glints (paint-studio-3cab71:5–7).
- **Already in the prompts?** Yes: Brief:39 requires a laid trial; Guide:340–346 and Studio:140 explain glaze overrun. Partly for these diagnostic examples.
- **Still valid?** Trialing and reach remain current. The old medium values do not establish a universal remedy for speckle or a guaranteed quiet glaze.

### Z3. Change repair method when a bank glaze keeps mottling

- **Lesson:** If a dark-field glaze repeatedly leaves mottles or a rim, compare a matching dry-ground body coat before glazing the area again.
- **Kind:** process.
- **Conditions:** Engine 1; near-black bank and oak foot; SHADEP glaze versus OK.base body paint over dry paint.
- **Evidence:** A foot glaze did not fade and left “a dirty puddle”; the final account says glazes on that bank mottle while thick OK.base over dry paint matches it (paint-studio-58dfc6:32–33). A subsequent unpainted return accepted the repaired foot's small residual flaw (paint-studio-58dfc6:34).
- **Already in the prompts?** Partly: Brief:37 requires reconsidering recurring repair failures. The substrate-specific glaze/body comparison is absent.
- **Still valid?** This is a strongly supported historical local outcome. The current guide does not establish that all bank glazes mottle, so the lesson is to compare methods rather than ban glazing.

## Drying and timing

### D1. Check the actual passage before a covering correction

- **Lesson:** Check drying at the repair and neighboring wet edges, then wait for a stable base when the new color must cover cleanly.
- **Kind:** easel.
- **Conditions:** Engine 1; dark wall/table/treeline paint over pale wet paint; crisp palette dabs over wet wood; reeds over wet bases.
- **Evidence:** Dark cleanup mixed with open jug white and became pale, while the same opaque correction later covered dry paint (paint-studio-f705c2:9). Palette dabs were invisible until the wood dried (paint-studio-b3a3c7:3). Reed stalks first dissolved, then the painter reported that tacky paint held the final stalks (paint-studio-2564f4:6,9–10). The lagoon journal repeatedly relearned the edge check (paint-studio-58dfc6:11,17,20,28).
- **Already in the prompts?** Yes: Guide:601–615 defines stages and underlayer pickup; Techniques:125–128 and 178–179 distinguish open work from stable layering.
- **Still valid?** Supported directly. Use the current stage query, not a blanket historical two-day or 10–12-day wait; Guide:605–613 makes drying material- and thickness-dependent.

### D2. Preserve intentional wet work while diagnosing unwanted mixing

- **Lesson:** Decide whether the next passage needs wet fusion or separate marks before waiting or painting into it.
- **Kind:** process.
- **Conditions:** Engine 1; body modeling, soft glass reflections and stone planes versus crisp silhouettes and detail.
- **Evidence:** A later fruit campaign used dry ground for stronger modeling shapes but a window reflection wet into fresh dark bottle paint (paint-studio-c766c5:11–12). A broad wet stone film followed by the badger gave a continuous shallow rise, with fine marks reserved for its dry surface (paint-studio-e708c7:12–13). These observations qualify the repeated journal rule “always paint dark on dry” (paint-studio-bfd78b:11).
- **Already in the prompts?** Yes: Techniques:125–128 explicitly assigns open paint to soft transitions and set paint to separate strokes.
- **Still valid?** Supported directly. Do not turn repair-driven “always wait” advice into a ban on wet-into-wet painting.

## Corrections and lifting

### R1. Protect the whole visible foreground, including its fine extensions

- **Lesson:** Before repainting a background, include every foreground extension in the exclusion mask or plan to repaint those marks afterward.
- **Kind:** process.
- **Conditions:** Engine 1; water and sky repairs around tree limbs, twig hairs, grass, reeds, ruins and figures.
- **Evidence:** A tree exclusion contained big limbs but omitted hanging twigs (paint-studio-58dfc6:10). Later water exclusions protected the bank and tree but omitted the figure, grasses and reeds standing above the crest (paint-studio-58dfc6:14–16). A table edge crossed a bottle until it was redrawn behind it (paint-studio-c766c5:6–7). Incomplete exclusions repeatedly recreated work.
- **Already in the prompts?** Partly: Techniques:183–185 says to clip near protected objects; Guide:566–577 documents visibility tools. The requirement to include projecting details is absent.
- **Still valid?** Supported by current mask-based clipping and stroke overrun. Protecting the parent mass does not document protection of separately drawn marks.

### R2. Repair a coherent field when patch boundaries become the problem

- **Lesson:** When local patches keep producing halos or seams, restate the affected continuous field and rebuild its foreground once the field is stable.
- **Kind:** process.
- **Conditions:** Engine 1; sky gradients, still-life background/table fields and fog around trees; body color over dry layers.
- **Evidence:** Fresh whole inspection led to wall/table rebuilding before the jug and lemons (paint-studio-f705c2:8–11), then an unpainted fourth sitting accepted them (paint-studio-f705c2:13). After edge-to-edge fog/foreground unification, another painter reported “the picture breathes again” (paint-studio-2a634e:5). Later trunk defects required further work (paint-studio-2a634e:10). Full-width sky repairs repeatedly recurred in the tree landscape (paint-studio-851a37:13–14,23–25,44–52).
- **Already in the prompts?** Partly: Brief:37 says to reconsider a repair that recreates its problem; Studio:128–130 and 155–156 describe boundary seams. The field-first rescue is not explicit.
- **Still valid?** A process option, not permission to reset the whole painting after every flaw. The repeated later rescues warn that field repaint alone is insufficient.

### R3. Inspect the final repair mask's geometry after all operations

- **Lesson:** Check the final mask against the actual object and crest before painting, especially after blurring, growing or subtracting shapes.
- **Kind:** easel.
- **Conditions:** Engine 1; blurred bank/foot repair next to water; foot polygon starting below the softened patch's upper reach.
- **Evidence:** A blur-expanded repair laid a dark wedge above the bank and a band across the trunk (paint-studio-58dfc6:31). The final note attributed the outcome to operation order and advised subtracting the crest before blurring (paint-studio-58dfc6:33). That prescription is the painter's interpretation; the journal does not isolate or prove it as a safe general order.
- **Already in the prompts?** Partly: Studio:144–156 explains whole-canvas masks and seams; Guide:382–397 lists operations. An explicit final-mask check is absent.
- **Still valid?** The geometric check applies to current masks. The guide does not establish the proposed blur/subtraction order as a universal fix, so do not copy it as one.

### R4. Remove an optional motif when repeated rescues weaken the picture

- **Lesson:** If repeated repairs to an optional motif keep worsening the whole, consider removing it and restoring the passage that carries the intended effect.
- **Kind:** process.
- **Conditions:** Engine 1; small boat over multiply repaired water; later fine stone motifs in a quiet fog landscape.
- **Evidence:** Eight boat attempts got worse; after a final hull failed, the painter decided the boat was “not its soul” and removed it (paint-studio-3f743e:12–14). Another painter ultimately sank troublesome stones to invisibility while retaining the main fog/tree/figure arrangement (paint-studio-2a634e:14–15). Both final entries report the simpler composition as accepted.
- **Already in the prompts?** Partly: Brief:37 allows reconsidering composition; Brief:41–43 preserves artistic choice. It does not explicitly identify motif removal as a valid rescue.
- **Still valid?** A process lesson independent of engine physics. It is not a substitute for an essential requirement: another journal later noticed that a reset had removed its essential oak (paint-studio-851a37:31–43).

## Detail and texture

### M1. Keep water glints irregular and subordinate to the water plane

- **Lesson:** Start with a few broken horizontal glints and check them at whole-picture scale before filling the water with repeated marks.
- **Kind:** craft.
- **Conditions:** Engine 1; hatch sparkle, regularly spaced reflection ladders and hundreds of ripple flecks; quiet moonlit or dusk water.
- **Evidence:** Hatch sparkle became a “dense rectangular carpet” (paint-studio-2564f4:5). A regular moon ladder was veiled back and replaced by separated glints retained in later sittings (paint-studio-f0e4ab:4,7–10). Even hatches were covered into quiet water in another journal (paint-studio-ebf843:3,6–8). Six hundred flecks plus blending made a pattern; later calmer water was accepted (paint-studio-58dfc6:17–18,34).
- **Already in the prompts?** Yes: Techniques:65–75 covers distance, direction and scattered glitter; Techniques:131–134 warns against mechanical texture.
- **Still valid?** A compositional lesson. Exact mark counts and coverage are local; no threshold separates all calm water from patterned water.

### M2. Build a cloud as a connected mass rather than repeated equal puffs

- **Lesson:** If a cloud reads as beads or cabbage, simplify it to fewer unequal connected planes before adding separate accents.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; clouds made from ovals, ellipsoid lumps or individually clipped lobe discs with repeated lighting.
- **Evidence:** Flat cloud ovals prompted irregular hand-drawn bodies (paint-studio-2564f4:3). The cumulus journal rejected uniform circles as “fish scales/cabbage” and concluded that puffs needed much greater size variation and no clipped internal lobe edges (paint-studio-bfd78b:8). It later abandoned volumetric cloud building for bar clouds (paint-studio-bfd78b:11). That final tactic remains an unverified plan.
- **Already in the prompts?** Partly: Techniques:91–94 says a form is one connected mass; Techniques:62–64 warns against one mark size. The cloud failure examples add specificity.
- **Still valid?** The connected-mass principle is already current. This batch does not prove that only bar clouds work or that current form tools cannot make cumulus.

### M3. Calibrate fine-line width and load before building a branch network

- **Lesson:** Inspect a representative tapering twig before repeating it across a crown, adjusting pressure and reloads so it does not become a faint hairline or a split fan.
- **Kind:** easel.
- **Conditions:** Engine 1; pointed round/rigger twigs on set sky; width 2.0–2.4 and pressure 0.8 tapering to 0.25 in one journal; polygons used for thicker twigs in another.
- **Evidence:** Brush calibration reportedly changed faint hairlines into substantial tapered twigs (paint-studio-851a37:11). Another painter used clipped polygons above 1.8 units after curved round strokes split into bristle fans, with tapering riggers below that width (paint-studio-58dfc6:7). Its final reviews retained the crown (paint-studio-58dfc6:28,34). The 1.8-unit boundary was a local workaround.
- **Already in the prompts?** Yes for calibration: Guide:226–227 offers mark-width helpers; Guide:238–241 describes points and taper; Studio:39–56 describes pressure and low-load breakup. Brief:39 requires representative trials.
- **Still valid?** Mechanisms are current. Do not import the numerical threshold, a 668-twig target or the journal's botanical claims as validated requirements.

### M4. Let texture follow the form without replacing it

- **Lesson:** When surface marks become the subject, restore the underlying volume and return fewer marks following its direction.
- **Kind:** craft.
- **Conditions:** Engine 1; pottery throwing rings, oak collar circles, bark dabs, stone scratches and repeated frost dots.
- **Evidence:** Prominent pitcher rings triggered opaque body consolidation (paint-studio-69c343:12–18), but a later sitting still found the pitcher flat, so that first rescue was not sufficient (paint-studio-69c343:60–64). Oak dabs and target-like collars prompted continuous longitudinal strokes (paint-studio-851a37:18). Stone films later removed mechanical scratching; path dots were quieted without changing the composition (paint-studio-e708c7:12–17).
- **Already in the prompts?** Yes: Techniques:62–69 requires mark hierarchy and direction; Techniques:91–94 requires a unified form.
- **Still valid?** A supported craft/process lesson. The journals do not independently establish botanical or geological accuracy of the proposed replacement marks.

### M5. Use a continuous stroke when separate touches make beads or dashes

- **Lesson:** For a small continuous accent, trial one loaded dragged stroke rather than a row of touches or a broad low-coverage fill.
- **Kind:** easel.
- **Conditions:** Engine 1; cloth fold accents, bowl rims and glass highlights; explicit filbert 6 stroke with clipping in one still life.
- **Evidence:** Long thin explicit accent strokes reportedly stayed crisp on cloth and bowl (paint-studio-520026:35). A broad low-coverage glass highlight came out patchy and dashed, prompting a single window reflection treatment (paint-studio-c766c5:10–11). Later fruit work does not reassess the bottle highlight separately.
- **Already in the prompts?** Yes: Studio:78–79 describes dashed narrow-band planning; Studio:119–120 contrasts touch beads with continuous dragged ridges. Guide:251–259 offers deliberate gestures.
- **Still valid?** The distinction is current. The old brush width is not a default, and a crisp accent still needs a suitable underlayer.

## Composition and looking

### L1. Check that the light has a darker structure to act against

- **Lesson:** Use a whole value view to identify missing contrast before adding more luminous detail.
- **Kind:** process.
- **Conditions:** Engine 1; high-key sky/water with only a mid-gray range; thin pale fruit gradients.
- **Evidence:** A value study found nothing darker than mid-gray except a bottom water edge, changing the next campaign to strengthening contrast (paint-studio-bfd78b:5). The fruit painter found low-contrast blended forms weak and reported volume only after clearer body/shadow/light shapes (paint-studio-c766c5:10–12). The cumulus journal has no final confirmation of its contrast plan.
- **Already in the prompts?** Yes: Techniques:14–16 and 47–49 explicitly require value masses and a full range; Brief:37 requires checking large relationships.
- **Still valid?** A process lesson. A quiet high-key intention is not automatically a defect; the journal diagnosis depended on the intended drama or volume.

### L2. Recheck contact, attachment and scale after background repairs

- **Lesson:** After repainting the surroundings, inspect whether objects remain attached, grounded and legible at the intended viewing distance.
- **Kind:** process.
- **Conditions:** Engine 1; pitcher handle and foot, trunks crossed by fog, small chapel/moon reflection and foreground stone bases.
- **Evidence:** A jug handle needed its own light coordinate and two rebuilds before a later sitting accepted it (paint-studio-f705c2:10,13). Trunks earlier accepted as fogged were later found broken (paint-studio-2a634e:9–11). A dry return recovered a swallowed chapel and almost-lost light path with spare marks (paint-studio-f0e4ab:6–10). Dry earth pools strengthened stone setting (paint-studio-e708c7:7–10).
- **Already in the prompts?** Yes in principle: Techniques:53–56 addresses physical meetings; Brief:37 asks for spatial relationships and whole-picture checks. Partly for checking attachment after repairs.
- **Still valid?** Independent of engine-specific thresholds. No formal scale calculation in these journals should be treated as a measurement of real-world accuracy.

### L3. Preserve the quiet intervals that carry the intended effect

- **Lesson:** Before adding another motif or texture, check whether the existing empty interval already serves the picture better.
- **Kind:** process.
- **Conditions:** Engine 1; restrained dusk water, expansive sky, empty street and small figures used for human scale.
- **Evidence:** A moon/lamp landscape stopped without sharpening its figure because “quiet scale and the rough, broad edges are the point” (paint-studio-db6324:4–5). Sparse tarn glints were retained across return inspections (paint-studio-ebf843:6–8). The storefront painter preserved empty pavement and an unused chair, confirmed in a final return (paint-studio-3cab71:4–7). A stone-detail campaign retained the large empty intervals (paint-studio-e708c7:14–17).
- **Already in the prompts?** Partly: Brief:37 asks about intended effect and Brief:51–56 asks whether further work is wanted; Techniques:131–134 says to leave passages quiet. These journals supply concrete stopping criteria.
- **Still valid?** A process lesson independent of engine behavior, conditional on the chosen intention rather than a rule that every painting must be sparse.

## Process and sittings

### P1. Keep material trials from becoming unplanned picture content

- **Lesson:** Use a comparable trial area and decide how its marks will be kept separate from, or removed from, the picture before testing.
- **Kind:** process.
- **Conditions:** Engine 1; swatches in a bottom strip intended for later foreground coverage, water swatches and dark-bank color matching; no scratch canvas recorded.
- **Evidence:** Wet bottom-strip swatches were dragged by a later blend (paint-studio-58dfc6:1–2). Four trunk-foot swatches needed covering after drying and did not solve color matching (paint-studio-58dfc6:29–30). Water test swatches also needed painting out (paint-studio-bfd78b:7), and another final cleanup covered an orange test remnant (paint-studio-2a634e:13).
- **Already in the prompts?** Yes for current comparable scratch trials: Brief:39 and Guide:698–715. Partly for planning removal when testing on the painting.
- **Still valid?** The process risk remains relevant. None of this batch's journals tests the r31 scratch canvas; do not attribute its isolation or independent clock to historical experience.

### P2. Verify a completion claim with the latest whole and detail views

- **Lesson:** Base the finish decision on the current canvas at whole and detail scales rather than the preceding session's description of success.
- **Kind:** process.
- **Conditions:** Engine 1; dry return sittings after confident completion statements; whole views, squint and close crops.
- **Evidence:** A finished still life reopened with halos and gray veils, then a fourth sitting accepted the repaired state without further paint (paint-studio-f705c2:7–13). Apple finishes were rejected repeatedly (paint-studio-520026:7–8,10,23–24,37). A pitcher completion claim was followed by severe defects (paint-studio-69c343:56–64). Later glowing prose on days 57 and 88 contains no comparably specific verification of all those defects (paint-studio-69c343:70–86).
- **Already in the prompts?** Yes: Brief:35–39 and 51–56; Techniques:43–46 and Guide:658–661 require whole/detail inspection.
- **Still valid?** A process lesson. Interpretation: the specificity and later survival of an observation are stronger evidence than celebratory final prose; the journals alone cannot establish the images' quality.

### P3. Stop a repair cycle when the cure is larger than the flaw

- **Lesson:** After each local repair, compare the whole picture and stop or change method if the intervention creates a more visible problem.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; tiny trunk-foot hem flaw visible mostly at detail scale; multiple dark-on-dark attempts and long waits.
- **Evidence:** A stipple repair became a big black bush; the painter wrote “I am making the picture worse by tinkering” (paint-studio-58dfc6:30). Further repairs added a wedge and puddle (paint-studio-58dfc6:31–32). A final unpainted return accepted a faint hem because earlier fixes introduced larger flaws (paint-studio-58dfc6:34).
- **Already in the prompts?** Yes: Brief:37 explicitly requires a whole-picture repair check and reconsideration before expanding a recurring failure. This case is a strong worked example.
- **Still valid?** A process lesson independent of the precise old ghosting behavior. The accepted flaw and intended scale belong to this picture.

### P4. Carry observed state and unresolved problems into the next sitting

- **Lesson:** End a sitting with the observed state, protected marks and the next dependency so the next campaign does not repeat an already diagnosed mistake.
- **Kind:** process.
- **Conditions:** Engine 1; multi-sitting landscapes with waits, reusable masks, twig coordinates and explicit remaining-work lists; no explicit inter-painter handoff recorded.
- **Evidence:** A resumed journal lists the current masses, an owed underdrawing and the need to let land/mist set (paint-studio-11997b:1–4). Another records seeds, affected chunks, dry-time dependencies and finish checks (paint-studio-58dfc6:16), but subsequently repeats wet-edge and exclusion mistakes (paint-studio-58dfc6:17,19–20). Notes therefore help preserve state without guaranteeing adherence.
- **Already in the prompts?** Partly: Brief:39 and 44–45 ask for useful observations and a journal; Guide:668–669 documents persistent globals. A concrete resume-state format is absent.
- **Still valid?** A process candidate. Interpretation: these entries support better continuity within one painter's sittings, not a proven handoff protocol between painters. Some journals have duplicated or retrospective dates, so elapsed-time claims should follow their explicit accounts rather than infer session identity from timestamps alone (paint-studio-851a37:23; paint-studio-58dfc6:31–33).

### P5. Evaluate accidents against the intended picture before keeping them

- **Lesson:** Keep an accidental effect only after checking that it supports the composition and does not conceal a structural defect.
- **Kind:** process.
- **Conditions:** Engine 1; sky-wide pale veil, portrait copper hair and textured background, fog crossing tree stems.
- **Evidence:** A sky accident changed the aim to silver dusk and was accepted in the final account (paint-studio-3f743e:4,14). Portrait hair/background accidents were kept because they supported warmth and direction, with no later return to test the judgment (paint-studio-6e8898:24–35). Fog across trunks was first treated as a gift but later judged to have broken them (paint-studio-2a634e:7–10).
- **Already in the prompts?** Yes in principle: Brief:37 evaluates intended effect and Brief:42–43 permits reconsideration. The need to distinguish atmosphere from hidden damage is implicit.
- **Still valid?** A process lesson. The contradictory trunk observations specifically prevent treating every attractive smear as a successful atmospheric solution.

### P6. Wait for painting to complete before looking at its result

- **Lesson:** Request the look after the paint call returns so the inspection corresponds to the completed campaign.
- **Kind:** process.
- **Conditions:** Engine 1 studio tooling; concurrent `paint` and `look` calls early in a landscape session.
- **Evidence:** The painter reported that “paint + look in parallel gave me no image for early looks” and chose to look after paint returned (paint-studio-bfd78b:1). No later journal isolates whether the old failure was fixed.
- **Already in the prompts?** Yes in effect: Brief:35 orders painting, inspection and then writing the next campaign. Guide:3–4 says Lua chunks run one at a time.
- **Still valid?** The observation sequence remains appropriate. The current guide does not document the old concurrent-call failure, so do not claim that its exact error still occurs.

## Tool confusion

### U1. Read edge loss as stroke reach, not as a blur operation

- **Lesson:** Before using `edge="lost"` near important content, inspect a small trial of how far its strokes carry the paint.
- **Kind:** easel.
- **Conditions:** Engine 1; near-black occlusion ring, coverage 1.3, lost edge; mask probed afterward and reported zero over protected cloth.
- **Evidence:** The lost-edge pass blacked out most of a still life despite a ring mask; the painter concluded that edge loss was not a generic softening control (paint-studio-520026:9). Later entries retained the warning (paint-studio-520026:23,48). A different journal reported soft/lost shares useful for broad country, so a universal ban is unsupported (paint-studio-11997b:4).
- **Already in the prompts?** Yes: Guide:350–358 defines lost edges as strokes running past the contour; Guide:340–348 distinguishes anchoring from clipping. Techniques:183–185 warns about overrun.
- **Still valid?** Supported directly. The exact blacking-out extent is historical; the current guide gives no universal maximum reach for a chosen edge setting.

### U2. Use coverage to choose mark density, not a percentage label

- **Lesson:** For sparse texture, begin with low coverage and no gap filling, then inspect how many marks actually remain.
- **Kind:** easel.
- **Conditions:** Engine 1; bark-ridge pass at coverage 2.4 mistaken for sparse marks; later local proposals around 0.3–0.5 with `fill=false`.
- **Evidence:** Coverage 2.4 covered the lower trunk in brown; the painter corrected its interpretation to layers per point (paint-studio-58dfc6:17). A later coverage 1.1 bark pass still made orange camouflage, eventually covered with plain trunk paint (paint-studio-58dfc6:26–28). Low density alone was not shown to cure excessive color or contrast.
- **Already in the prompts?** Partly: Guide:321 and 331 defines coverage and fill; Techniques:62–69 describes hierarchy. The explicit sparse-mark example is absent.
- **Still valid?** The definition remains current. The proposed 0.3–0.5 range is a historical starting point, not a current standard.

### U3. Do not transplant old API limitations into the current guide

- **Lesson:** Check the current option's documented type before copying a workaround from an older journal.
- **Kind:** process.
- **Conditions:** Engine 1; `coverage` supplied as a function, numeric `below(600)` and helper functions declared local in an earlier chunk.
- **Evidence:** One build rejected functional coverage with “converting Lua function to f32” (paint-studio-520026:13), while another journal reports functional stipple coverage (paint-studio-58dfc6:28). Numeric `below(600)` caused three failed chunks (paint-studio-520026:33). Local helpers disappeared between chunks (paint-studio-58dfc6:7). These are distinct API/type problems.
- **Already in the prompts?** Yes: Guide:321 documents numeric `work` coverage, Guide:362 documents numeric or functional stipple coverage, Guide:388–401 documents curves/point lists for `below` and Guide:665–669 covers option types and globals.
- **Still valid?** The `below` and local/global distinctions are still documented. The old functional-coverage ban needs the operation specified: current `stipple` explicitly supports it. Guide:407 loosely mentions passing noise to `coverage`; this does not resolve functional `work` coverage against its numeric table entry. No engine behavior was tested here.

### U4. Pass keyed option tables intact

- **Lesson:** Pass the option table itself when it contains named fields instead of unpacking it as positional arguments.
- **Kind:** easel.
- **Conditions:** Engine 1 Lua chunks; string-keyed option table with no array entries.
- **Evidence:** A journal reports that `table.unpack(t)` returned nothing from such a table, silently dropping the intended options (paint-studio-520026:15). The journal gives no later isolated test, but explicitly records the failed construction.
- **Already in the prompts?** Partly: Guide:223–225 and 294–299 shows named option tables passed intact; Guide:683–684 distinguishes Lua list/table syntax. This unpacking trap is not stated.
- **Still valid?** Current examples use intact tables, but the guide does not explain `table.unpack` semantics beyond naming it (Guide:677–679). Treat the historical trap as an API-use warning, not a newly verified engine result.

### U5. Do not assume partial seeded replay preserves the marks

- **Lesson:** Do not assume that rerunning only selected marks from a seeded loop will reproduce their earlier positions or shapes.
- **Kind:** easel.
- **Conditions:** Engine 1; grass-loop partial restoration after a water repair; wrapper intended to consume the original two random calls while skipping most paint calls.
- **Evidence:** The proposed partial replay appears in the repair plan (paint-studio-58dfc6:31). The final account says it did not reproduce the original blades because brush strokes also consumed the random stream (paint-studio-58dfc6:33). This explicitly overturns the earlier plan.
- **Already in the prompts?** Partly: Guide:670–674 documents per-chunk deterministic randomness and optional pass seeds, but does not promise arbitrary partial-loop equivalence.
- **Still valid?** The limitation is plausible within current documented sequencing, but the guide does not specify how many random values each brush call consumes. Do not promote the historical full-loop workaround to a current guarantee or assume that moving code to a new chunk preserves its seed.

### U6. Treat lifting as a wet-paint operation, not a finishing penalty

- **Lesson:** Check whether paint is still liftable before diagnosing a failed correction as damage caused merely by touching a dry surface.
- **Kind:** failure-pattern.
- **Conditions:** Engine 1; dry-return landscape statements versus documented wet-paint pickup; no isolated rag or knife experiment in this batch.
- **Evidence:** A painter twice ended with a claim that further work “only lifts what is dry” despite marking the whole painting dry (paint-studio-2a634e:12,15). Other journals locate unwanted pickup in open or insufficiently set underlayers and obtain clean covering after drying (paint-studio-f705c2:9; paint-studio-c766c5:3,7). Interpretation: the first statement conflates a justified stopping judgment with an unsupported mechanism.
- **Already in the prompts?** Yes: Studio:89–91 and 124–125 says a clean brush cannot lift set/dry paint; Guide:509–512 limits rag lifting to open/setting paint.
- **Still valid?** The stopping choice may remain sound, but the claimed dry-lifting mechanism is contradicted by the current documents. Journal uses of “wipe,” “erase” or “bath” often mean overpainting; they do not establish trials of the later rag or knife tools (paint-studio-69c343:23; paint-studio-3f743e:12–13).

## Ten strongest candidate prompt improvements

These are candidate clarifications or examples, not edits to the prompts. Their order is an interpretation of how recurrent and consequential the recorded failures were. Existing coverage is stated so an editor can reinforce a requirement without presenting it as new.

1. **Make finish checks explicitly evidence-based:** compare the latest whole and detail views with the claimed result; a preceding “finished” note is not evidence that the current canvas holds. Already required in Brief:35–39,51–56; these journals supply a warning example. Citations: paint-studio-520026:7–8,23,37; paint-studio-f705c2:8–13; paint-studio-69c343:56–64.
2. **State that coverage counts strokes/layers, not opacity:** when a repaint ghosts, check substrate, stroke length and reloads before escalating coverage. Partly covered by Guide:321 and Studio:32–35. Citations: paint-studio-520026:17,27–28,49; paint-studio-c766c5:9–12.
3. **Add a complete-foreground exclusion reminder:** parent masks must account for hanging twigs, reeds, grasses and figures, or those details belong after the background campaign. General clipping is already in Techniques:183–185. Citations: paint-studio-58dfc6:10–16; paint-studio-c766c5:6–7; paint-studio-520026:32.
4. **Make the drying check local to the repair and its neighbors:** require a stable underlayer for clean covering, while preserving intentional wet fusion; historical wait lengths are not recipes. Mechanism already in Guide:601–615 and Techniques:125–128. Citations: paint-studio-f705c2:9; paint-studio-b3a3c7:3; paint-studio-58dfc6:11,17,20; paint-studio-e708c7:12.
5. **Give a concrete stop-or-change-method example:** if a detail repair makes a larger whole-picture flaw, accept the smaller residual or change the approach before adding paint. Already required in Brief:37. Citations: paint-studio-58dfc6:29–34; paint-studio-3f743e:12–14.
6. **Explain the local blend seam and lifting symptoms together:** pale lattice, halos and rectangles are reasons to inspect the film and transition, not continue blending automatically. Mechanisms already in Studio:124–136. Citations: paint-studio-f705c2:2; paint-studio-2564f4:3; paint-studio-851a37:13–14; paint-studio-520026:19–21,28.
7. **Reinforce substrate-matched trials with a failed-color example:** a thick pile or familiar earth name cannot predict the value of its thin film over pale or dark paint. Already explicit in Brief:39 and Studio:24–35. Citations: paint-studio-520026:5,27; paint-studio-58dfc6:2,29–30; paint-studio-c766c5:9–12.
8. **Distinguish a white veil from a transparent tint and test its reach:** a pale glaze can erase contrast or mark the surface instead of creating light. Already addressed by Studio:32–35 and Guide:340–346; an example would connect the warnings. Citations: paint-studio-58dfc6:5–6,19; paint-studio-3cab71:5–7; paint-studio-f705c2:6,8–11.
9. **Show how repeated detail becomes an unwanted pattern:** trial a few glints, texture marks or cloud planes at whole scale before multiplying them; restore a continuous base when marks dominate. General hierarchy already in Techniques:62–75,131–134. Citations: paint-studio-f0e4ab:4,7–10; paint-studio-58dfc6:17–18; paint-studio-bfd78b:8,11; paint-studio-e708c7:12–17.
10. **Add a concise sitting-state journal pattern:** observed result, wetness, recipe/handling, protected details, unresolved defect and next dependency; distinguish observations from hypotheses and plans from successful outcomes. Brief:39 already requests material conditions, while this would improve continuity. Citations: paint-studio-11997b:1–4; paint-studio-58dfc6:16–20,31–33; paint-studio-851a37:31–52.
