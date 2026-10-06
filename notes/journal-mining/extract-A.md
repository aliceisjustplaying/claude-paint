# Journal mining, batch A (r16 and r17, engine 1)

Citations are `<studio>:<line>`, where `<studio>` is the hex suffix of `~/src/a/paint-studio-<studio>/notes/journal.md`.

| studio | round/lane | subject | span |
|---|---|---|---|
| 6399ad | r16 D1 | still life, quinces and pitcher | day 1 (5 entries, unfinished log) |
| 7b31aa | r16 D2 | still life, quince and pitcher | day 1 |
| cd267e | r16 B1 | Elbe meadows, evening | day 43 |
| b5323f | r16 D3 | still life, jug and quince | day 1 |
| 9aae10 | r16 A1 | dolmen in snow | day 153 |
| 3be9e8 | r16 C1 | stoneware jug, lemons | day 53 |
| 9cf69b | r16 A2 | dolmen in snow | day 45 |
| 7ace8e | r16 B2 | Greifswald meadows, evening | day 144 |
| 75c237 | r16 C2 | estuary at low tide | day 147 |
| 96b5e4 | r16 C3 | evening marsh | day 39 |
| 7ff345 | r16 A3 | frozen mere | day 96 |
| 774267 | r16 B3 | meadows before a town | day 106 |
| 1def1a | r17 O1 | estuary with barge | day 30 |
| 4fb403 | r17 O2 | estuary at low tide | day 54 |
| cfa19c | r17 F1 | June meadows, oak | day 133 |
| c6b5dd | r17 O3 | estuary, contre-jour tree | day 47 |
| 497bcf | r17 F2 | June meadows | day 106 |
| 358aea | r17 F3 | Ryck meadows | day 107 |

## Summary

All 18 journals are engine 1: no rag, no knife, no thinner, no scratch canvas. Painters lifted with "a clean flat wiped after every stroke" and tested on canvas corners. The three D-lane still lifes (6399ad, 7b31aa, b5323f) are one-day, few-entry journals with little self-correction. The other 15 are long multi-sitting landscapes and one still life, and they record the same traps over and over. **Unclipped blends and glazes** wreck neighbors. **Rect-masked or local blends and patches** leave seams. **Lead white** swamps mixes and glazes. **Dark over wet light** (and light over wet dark) goes gray or vanishes. **Local patches on a graded field** never match. The fix that worked again and again: let everything dry, then repaint the *whole* passage as one wet layer, clipped, and blend it whole. Value structure was the most common picture-level fault: land as light as the sky, a foreground that needed three or four glazes to come down. Many pictures improved when painters came back after declaring them "finished."

## Ground and drawing

**G1. Open-ended curves in `above()`/`below()` paint a block past their last point. Make every curve span the full canvas width.**
- Kind: easel / failure-pattern. Conditions: mask from a point-list curve that stops short of the edge; engine 1.
- Evidence: "the mound-shade pass (a mask built from a curve that stopped at x=620) also laid a grey rectangle over the field from x 620 to 700" (9cf69b:6). The same thing happened again in another lane: "above(bot) with bot ending at x=470 ... a 20-unit dark column ran from the bank to the bottom edge" (c6b5dd:10). Each cost a whole-field repaint (9cf69b:7, c6b5dd:11-12).
- Already in the prompts? Partly. Studio notes 44: "A mask function that isn't zero far from the spot you mean paints there too." The guide doesn't say how `below`/`above` extend past a curve's last point.
- Still valid? Likely, since the API hasn't changed (guide: `below(function(x) ... end)` or a point list). Not verified.

**G2. Every mask for a background passage should subtract what stands in front of it, and old masks need updating after new objects go in.**
- Kind: process / failure-pattern. Conditions: masks kept as globals across many sittings.
- Evidence: "the horizon groves were painted across the oak's limbs because I didn't subtract the tree from their mask ... every mask from now on subtracts what stands in front" (cfa19c:9). "the river toning had run over the figures at shoulder height (the RIVER mask predates them)" (358aea:16). "the red dress glaze used the skirt polygon, which runs up under the shawl" (774267:11). Stale masks also bit later: "The cream halos around the towers were the old, wider town masks that every cloud repaint had cut around" (cfa19c:18). And "I forgot to cut the crown masks at the ground line" (7ace8e:15).
- Already in the prompts? No. Techniques covers clipping and globals, not keeping masks current.
- Still valid? Yes (process).

**G3. A ribbon's end disc overhangs its endpoint, so a trunk's foot hangs below the ground line.**
- Kind: easel. Evidence: "the trunk ribbon's end disc hangs below the knoll surface like a bulb" (7ff345:3). "ribbon (widths are full widths...)" (96b5e4:2).
- Already in the prompts? Yes. Studio notes 46.

**G4. `poly(pts, true)` smooths corners, and `edge="soft"` eats thin shapes. Draw spires and gables with plain polygons and firm edges.**
- Kind: easel. Evidence: "The town was restated twice because soft edges ate the spires; poly(pts, true) SMOOTHS the corners ... no edge='soft' next to thin shapes" (cfa19c:14).
- Already in the prompts? Partly. Guide: "`poly(pts, true)` -- true: smoothed". Nothing on soft edges and thin shapes.

## Color mixing

**C1. Lead white dominates every mix: a pile with much white comes out far paler than its tube ratios suggest. Mix darks and shadows deeper than seems right, and keep white out of them.**
- Kind: easel / craft. Conditions: wet mixes and blends; engine 1.
- Evidence: "lead white swamps everything at these proportions (... smalt 4 : white 1 is only a mid slate)" (9aae10:2). "Lead white dominates every wet mix here" (3be9e8:8). "Opaque shadow mixes on the jug again came out far too pale once blended; lead white wins" (3be9e8:17). "mauve piles came out pale pink (white took over)" (96b5e4:1). "Water v3 ... came out near-white (lead white takes over again)" (96b5e4:5). Also 358aea:2, 774267:2, 9cf69b:11 ("mix bark tones darker than I think"), 9aae10:11.
- Already in the prompts? Partly. Techniques: "Every pile with much lead white is chalky. Keep white for the lights." `look --palette` exists to check this.
- Still valid? Probably. The tube table still gives lead white hiding 0.82 and tinting 1.0. Engine 1 may have weighted it differently; not verified.

**C2. Thin films of umber mixes read light and tan. For dark twigs and trunks against a sky, use a bone-black-rich mix.**
- Kind: easel. Evidence: "thin films of the umber mixture came out tan, lighter than the zenith, wrong for trees against an evening sky" (9aae10:6). "The dead limbs are far too pale, like peeled sticks" (9cf69b:3). "Willow trunks came out pink (umber+white)" (358aea:11).
- Already in the prompts? Yes. Studio notes 6-7 (thin dark films come out lighter and warmer; umber fringe dries warm brown).

**C3. Smalt is a weak stainer. A darkening glaze over a large pale passage needs Prussian blue or black in it.**
- Kind: easel. Conditions: snow and land glazes; smalt tinting 0.45.
- Evidence: "smalt is too weak a stainer (tint 0.45) for this; the ground still reads pale. Next glaze will carry a little Prussian blue and black" (7ff345:11). After that: "third ground glaze with Prussian blue and black in it - the foreground finally sits down" (7ff345:12). Also "first far too pale (smalt is weak)" (774267:2).
- Already in the prompts? Partly. The tube table lists tinting strength. No advice ties it to glazes.
- Still valid? Smalt is still in the table at 0.45.

**C4. A fresh light mix over an aged, glazed passage won't match. Match with much deeper mixes than the original recipe, or repaint the whole passage.**
- Kind: easel. Evidence: "the fresh lead-white mixes read far paler than the aged sky ... restating with much deeper mixes (white 4 : cobalt 1.4 : smalt 1.1) matched" (cfa19c:12). "fresh paint over light or dark underlayers reads differently each time" (cfa19c:13). "my first mixes had far too much white (the old plain has sunk under glazes)" (9aae10:11). "Painted it out with a cream mix that dried lighter than the sky" (7ace8e:11).
- Already in the prompts? No.
- Still valid? Unknown. My interpretation: the underlayer shows through (studio notes 5-6), so a "matching" recipe isn't a matching look.

**C5. A pile with a dark cloak next to it (dress under shawl) picks up its color when laid against or over it wet. Paint the pale accessory after the dark has set.**
- Kind: easel / failure-pattern. Evidence: "The shawl picked up red from the wet dress" (cd267e:6). The same in 774267:7, 358aea:11 ("came up pink"), 7ace8e:18 (kept as warm light). The repair everywhere was to restate it over dry paint (774267:12, 358aea:16).
- Already in the prompts? Yes. Studio notes 25, 29.

## Thin vs thick paint

**T1. A light opaque mix (lead-white heavy) used as a "scumble" lays opaque blobs or blocks, not a veil.**
- Kind: easel / failure-pattern. Conditions: `hand="scumble"`, white-heavy pile, over wet or dry paint, engine 1.
- Evidence: "a lead-white-heavy pile at load 0.25 is not a scumble here" (1def1a:8). "A scumble over the far mud went on as an opaque scalloped block" (75c237:9). "white bricks (scumble filbert on thin ellipses, too loaded)" (497bcf:2). "chalky scrubbed marks" (4fb403:6). "the scumble spread warm and wide" (7ff345:13). The one success came from a lower-white mix thinly worked: "thin scumble of white with smalt, heavily badgered sideways" (7ace8e:20).
- Already in the prompts? Partly. Techniques: "Scumble: an opaque light paint dragged thin (hand="scumble", little load) over a dry darker one". Studio notes 8: a thin pale veil lightens. Nobody warns that it comes out blocky.
- Still valid? Unknown. The scumble hand may have changed since engine 1.

## Blending and edges

**B1. A blend confined to a rect or zone leaves that zone's outline. Blend the whole field, or use a mask shaped like the passage itself.**
- Kind: easel / failure-pattern. Conditions: badger `blend` with rect or partial masks.
- Evidence: "Lesson: blend the whole field, not a soft-edged zone - blending a zone leaves its outline" (3be9e8:2), confirmed at 3be9e8:7 ("Keeps confirming: blend whole fields"). "no rect-edged blends over body passages" (cd267e:3). "blended with a rect mask, which left rectangular seams" (7ace8e:6). "the badger smeared it into a hard-edged grey rectangle" (75c237:3), later fixed by blending "within a mask shaped like the reflection itself (not a rectangle, the mistake of day 3)" (75c237:13). Also b5323f:5, 96b5e4:3, 7ace8e:24, cd267e:7.
- Already in the prompts? Partly. Studio notes 39: "A local blend can leave a seam where lifted or moved paint meets the untouched paint outside that mask."
- Still valid? Studio notes 39 says yes.

**B2. Never blend unclipped near anything else wet or freshly painted, especially fresh darks. Clip every blend near a neighbor.**
- Kind: easel / failure-pattern. The most repeated failure in the batch.
- Evidence: "an unclipped blend over the reflection dragged the foresail tack into the sky" (1def1a:4). "the blender also dragged white across the tacky towers ... Lesson: clip blends away from anything else that is not dry" (cfa19c:5). "Rules from now on: clip=true on every blend near anything else" (cfa19c:14). "no unclipped blends anywhere near wet dark paint" (c6b5dd:17). "no unclipped blending near fresh dark" (358aea:17), restated at 358aea:21 as "never blend near fresh darks". "no unclipped blending across the shoreline" (4fb403:7, again 4fb403:8). Also 7ace8e:23, 96b5e4:6, 7ff345:8.
- Already in the prompts? Partly. Studio notes 39: "`blend` stays inside its mask by default. Unclipped, it drags wet paint across the mask's edge." In engine 1 the badger evidently wasn't clipped by default.
- Still valid? The default changed. The guide now clips `blend` by default, so the raw failure may be rarer. The underlying lesson (a blend near fresh darks smears them) still holds per studio notes 39 ("A soft mask does not prevent wet dark and light passages inside it from mixing").

**B3. A big badger over a small modeled object averages it to one flat value. Model small forms with direct graded strokes and don't blend them.**
- Kind: easel / process. Conditions: 40-unit badger over a jug or lemons.
- Evidence: "the 40-unit blender over a whole small object erases the modelling" (3be9e8:3). "a badger blend at 0.6 rad dragged it all into diagonal streaks. No more blender on small forms" (3be9e8:11). What worked: "seven piles from olive to pale, curved strokes along the long axis ... dark to light, no blending. They turn now" (3be9e8:13). Same pattern: "blend only a narrow zone, never the whole shape after accents" (1def1a:6).
- Already in the prompts? No.
- Still valid? Probably. The blend hand is still a badger 40 that lifts about 15% per pass (guide hand table).

**B4. A clean badger over a wet dark lifts it toward the lighter layer beneath. Don't blend darks to smooth them: glaze them when dry.**
- Kind: easel. Evidence: "the clean badger lifts wet dark paint back toward the lighter layers beneath - each blend over the trunk lightened it ... no more blending on darks" (cd267e:10). "tried to deepen it but the blend lifted most of the dark" (4fb403:4). "badgering a horizontal band ... Rule again: no blends across a light/dark boundary unless the dark is the wet part" (3be9e8:22).
- Already in the prompts? Partly. The guide says the blender "lifts paint as well as moving it". No line says what that does to darks.

**B5. Blending over dry paint shows the weave as a pale lattice.**
- Kind: easel. Evidence: "never blend across dry water" (96b5e4:5). "the blend over dry paint showed the weave as a lattice" (c6b5dd:14). "left a lattice patch" (75c237:10).
- Already in the prompts? Partly. Studio notes 41 (repeated blending of a lean layer). Guide: the blender "can't move set or dry paint".

**B6. A clipped pass on a soft (ramped) mask still cuts a hard edge, not a soft transition.**
- Kind: easel / tool confusion. Evidence: 3be9e8:5 assumed that "clip weighs by mask value, so soft masks give soft transitions". It was refuted at 3be9e8:8: "clip does not soften at a mask ramp, it cut hard bands".
- Already in the prompts? Partly. Studio notes 16 ("weighted by the mask's value", "reproduces the mask's outline exactly") and 19.

**B7. A sky in many overlapping wet bands, blended with long level (ruler) badger passes across the full width, comes out glassy. Stippled sky gradations take hours and set before blending.**
- Kind: easel / process. Evidence: "blended it long and level: it is glassy now" (cd267e:7). "blended level with long ruler passes while open" (7ace8e:2). "the stipple took hours and the paint set before blending ... eight graded piles in narrow overlapping bands, top down, blended in overlapping sections as I go while wet" (358aea:2).
- Already in the prompts? Partly. Techniques has the sky gradient. The guide's `piles=` graded option ("with no seams between masks") is the current tool for this; engine-1 painters didn't have it or didn't use it.
- Still valid? Banded piles should now be replaceable by `piles=`. Check before promoting.

## Glazing and scumbling

**Z1. A glaze must use low-hiding tubes and no white. Earths (umber hides 0.8) and any white make it opaque or chalky.**
- Kind: easel / craft. Evidence: "a reed glaze with raw umber in it went on opaque (umber hides 0.8) ... Lesson: a glaze wants the low-hiding tubes - smalt, Prussian, green earth" (7ff345:14). "glaze strokes with some white in them are too opaque" (3be9e8:6). "A white-bearing glaze on the cattle greyed them into pinkish ghosts ... lead white in a glaze lightens (again)" (497bcf:31). "white + smalt in the pile" made a pale reflection (c6b5dd:3).
- Already in the prompts? Partly. Techniques: "Glaze: a transparent paint (lakes, viridian, ultramarine, a little of anything)". "A little of anything" arguably invites earths. Studio notes 8 covers white veils.

**Z2. Glaze strength depends on medium and load. Test it small first: at medium 0.3 to 0.75 it can go on dark, crisp or as an opaque block, and at about 0.85 with a half load and a blend it veils.**
- Kind: easel. Conditions: engine 1; smalt glaze over pale snow.
- Evidence: "at medium 0.75 it went down as an opaque lavender block ... A second test at medium 0.92 and low load laid only a few starved streaks" (9cf69b:9). "pale smalt alone with a trace of umber and white, medium 0.85, load 0.5, then blended: a gentle cool veil" (9cf69b:10), then applied successfully with `load_at` grading (9cf69b:11). "Cloud-shadow glaze (medium 0.3) went on far too dark and crisp" (497bcf:6). "Pure umber/green-earth/black glaze on the jug went on far too dark" (3be9e8:9). "the glaze came far stronger than the earlier sail glazes, a near-black streaky band" (1def1a:5). "What I would do differently: test every glaze small first" (9cf69b:11). The same at 9aae10:12.
- Already in the prompts? Partly. Techniques gives medium 0.7 to 0.85. The brief asks for trials on scratch.
- Still valid? The numbers are engine-1 specific. The lesson to trial glazes is current.

**Z3. A glaze over tacky paint grabs into blotches and streaks. Wait for touch-dry.**
- Kind: easel. Evidence: "Path glaze went on over tacky paint and grabbed into blotches like bark" (7ace8e:10). "chained a glaze onto it while still tacky. It grabbed into streaks" (7ace8e:22). "Thin scratchy cores dragged over tacky cloud paint came out as stick-and-slip scribbles" (c6b5dd:8).
- Already in the prompts? Yes. Studio notes 31. Techniques: glazes want the passage dry.

**Z4. Glazes and the glaze brush aren't clipped: a 26-unit, 120-300-unit stroke runs well past the mask. Clip glazes.**
- Kind: easel / failure-pattern. Evidence: "spilled past its rect (glaze brush is 26 units wide - must clip)" (cd267e:9). "a glaze (hand=glaze is never clipped) on the track threw long brown smears up across the green strip" (497bcf:16). "ran past its mask in long horizontal strokes, filled the openings with brown" (9aae10:11). "a feathered glaze across the strip seams spilled into the grey horse" (7ace8e:21). "both of my glazes went wrong at the edges" (9aae10:12).
- Already in the prompts? Yes. Guide ("a `glaze` stroke is 120–300 units by default"), studio notes 43, techniques "Clip".

**Z5. Over a dry, textured passage a glaze sits in the weave or the tops depending on load: a heavy one reads as grain, a light one barely darkens.**
- Kind: easel. Evidence: "a glaze over dry paint barely darkened (sat on weave tops)" (96b5e4:4). "the glaze sits in the weave and reads as granite grain" (774267:11). "weave showing through in the glaze like pooled paint" (7ace8e:3).
- Already in the prompts? Partly. Techniques: over impasto, glaze pools in valleys.

**Z6. The shadows of a pale object came best from transparent glazes over its dry light body, each blended within one whole field, not from opaque darker mixes.**
- Kind: craft / easel. Evidence: "in this medium the shadows of a pale object have to come from thin glazes over dry light paint, badgered within a whole field, not from opaque darker mixes, which the lead white swallows" (3be9e8:25). Built through 3be9e8:17-19 (umber glaze at medium 0.88, then a cooler second glaze: "The jug turns now"). Contrast 9cf69b:11 and 774267:11, where the boulder succeeded only once dry thin glazes were used ("Lesson: stop working it wet", 774267:10).
- Already in the prompts? Partly. Techniques: a glaze "rescues a chalky passage".

## Drying and timing

**D1. Dark over wet light goes gray and streaky, and light touches or stipple into a wet dark vanish. Wait until the under-layer has set (3 to 4 days on these films) before crossing it.**
- Kind: easel. Evidence: "lesson learned again and again: dark over wet light goes grey and streaky ... Now waiting three or four days before crossing any wet passage" (9aae10:8). "Stipple onto wet dark paint vanishes into it" (cd267e:7). "foliage lights stippled wet dissolved into it as the notes warned" (c6b5dd:3). "shade stipple fused into the wet; second pass when dry" (774267:6), which then worked dry (774267:8). "First dark pass over the wet brown only lifted it" (9aae10:4). "my first snow cap went into wet stone and came out dirty grey" (9cf69b:5). Also 96b5e4:3, 358aea:12, 497bcf:5.
- Already in the prompts? Yes. Studio notes 26-28, 32. Techniques "Wet into wet".
- Still valid? Yes per studio notes. The day counts depend on pigment (guide Time section).

**D2. A long pass ages as it goes: by the end of a large stippled or banded field the start has set and won't blend.**
- Kind: easel. Evidence: 358aea:2 (above). Sky stipple "took hours".
- Already in the prompts? Yes. Guide ("painted in slices of 15 minutes"), studio notes 51.

**D3. Wait for full touch-dry before varnishing. Painters waited weeks to months, and a light varnish (0.25 to 0.3 coats) warmed and unified.**
- Kind: easel. Evidence: 75c237:14 ("All touch-dry after four months"), 774267:13, 7ff345:18, 3be9e8:25, 9cf69b:11, 9aae10:13.
- Already in the prompts? Partly. Techniques: "A varnish (where the easel has one)".
- Still valid? The current guide documents no varnish call. Unverified.

## Corrections and lifting

**K1. Don't patch a smooth graded surface locally (sky, water, field). Let it dry and repaint the whole passage as one wet layer, cut around what's in front, and blend it whole.**
- Kind: easel / process. The strongest cross-lane lesson.
- Evidence: "patching a smooth graded surface locally never matches. Decision: ... repaint the whole open water ... as a second full layer" (c6b5dd:11), then "The strip is gone" (c6b5dd:12). Again for the low sky (c6b5dd:20-21). "Lesson: in a sky, don't correct locally; repaint the whole passage" (cfa19c:13), and "Local patches made pale ghost spires, the old lesson again, so I repainted the whole cumulus body" (cfa19c:18). "Re-laid the whole field below the mound in three fresh wet bands ... the rectangle is gone" (9cf69b:7). "will let it dry and repaint the whole hay field and the green strip below it as passages, not patches" (497bcf:16-17). Also 1def1a:7 ("Repainted the whole water in one pass ... for coherence"), 7ace8e:10, 96b5e4:6, cd267e:7.
- Already in the prompts? No. The brief asks for a check after a local repair ("If the repair recreates the problem, reconsider").
- Still valid? Yes (process). Whole-field repaints cost time but ended the cycles.

**K2. Wet paint (even a fresh glaze) comes off almost entirely with a clean brush wiped after every stroke, in crossing passes. Use it to undo a bad glaze or test.**
- Kind: easel. Conditions: engine 1, before the rag existed.
- Evidence: "Lifted it off while wet with a clean flat, wiping after every stroke, in crossing passes: it came away almost entirely" (9cf69b:9). "Lifted it while open with a clean flat, wiped after every stroke, three passes: it came off clean to the dry reeds below" (7ff345:14), and again at 7ff345:15. A partial lift at 3be9e8:9 left "a speckled light-brown veil".
- Already in the prompts? Yes, in updated form. The brief and guide: "lift wet paint with a rag or a brush". Studio notes 25.
- Still valid? Yes. The rag is now the dedicated tool (engine 3+).

**K3. A thing that can't be fixed in place is often better restated whole over dry paint, opaque, from its mask, than repaired piecemeal.**
- Kind: process. Evidence: "Repainted the stones opaque in one dark body ... It finally reads as stones set up by people long dead" (9aae10:11). "restate the whole silhouette crisply from the town mask" (7ace8e:23-24). "Waited for everything to dry and laid a new cap opaque" (9aae10:7). "Lower elder restated as a dark mass" (497bcf:27).
- Already in the prompts? Partly. Techniques: "No undo: a mistake is painted over, glazed, scraped or let dry and restated".

**K4. Keep test swatches off the picture, or let them dry before working over them.**
- Kind: process. Evidence: "small swatches found it, though I spoiled one pass by working over the swatches while wet" (9aae10:11). "swatch tests at the bottom edge, to be painted over" (9aae10:2). Corner glaze tests that then had to be lifted off "so the corner will not get it twice" (9cf69b:10).
- Already in the prompts? Superseded. The scratch canvas exists now (guide, "The scratch canvas"). This batch shows why it's needed.

## Detail and texture

**X1. Thresholded noise patches of two or three piles, or short hatch over a field, read as camouflage, bricks or cobbles. Only a blend rescues them.**
- Kind: easel / failure-pattern. Evidence: "lesson: stop using thresholded noise patches for piles" (96b5e4:4). "Marsh hatch in lit/dark noise patches came out as bricks" (96b5e4:3). "camouflage-busy" (497bcf:3). "short hatch touches - came out as cobbles" (cd267e:5). "flat brush dabs ... looked like cobbles until one horizontal blend fused it" (75c237:6). "crosswalk stripes" (4fb403:3). "a barcode" (497bcf:29).
- Already in the prompts? Partly. Techniques "Many close piles" and "Vary the touch". `work`'s `piles=` with weight functions blends continuously.

**X2. Lit accents (grass tips, glints, highlights) scattered evenly read as confetti or yellow hairs. Lay dark and mid first, keep lights rare and clumped, and glaze them down if they pop.**
- Kind: craft / failure-pattern. Evidence: "Lit grass tips came out as bright yellow confetti evenly spread" (96b5e4:6), and the lesson: "keep light accents rare from the first pass" (96b5e4:8). "sparse light ones read as yellow hairs, so a second, denser dark/mid layer unified it" (774267:9). "first scattered evenly (a sprinkle ...), then clumped" (7ff345:9). "too many and too even, like a spotted rug" (9cf69b:8). "the light sky-caught blades came out too bright ... (like frost)" (7ace8e:17).
- Already in the prompts? Partly. Techniques "Thick where it counts", "hierarchy of marks", spatter "a little goes a long way".

**X3. Objects drawn as neat geometry read as icons: lollipop willows, broccoli crowns, pill footprints, cake or hut dolmens, mushroom pollards. Fix them by building structure (trunk, limbs, fans of rods) and breaking the outline.**
- Kind: craft / failure-pattern. Evidence: lollipops (cd267e:6, 497bcf:11, 358aea:7), broccoli (cfa19c:10, 358aea:6,12), haystack woods (9cf69b:3), bottle firs (9aae10:3), cake dolmen (9aae10:4), hut or mushroom dolmen (9cf69b:5), regular scallop snow (9aae10:7, 9cf69b:7), mushroom pollards (358aea:22). What fixed them: "fans of upright rods hatched outward from each knuckle ... each tree a different height and breadth" (497bcf:33). "Opened a gap of sky and rise under each crown ... hung leafy lobes back down over the rods" (358aea:22). Distant woods as "runs of small overlapping crowns of different heights" (9cf69b:3; also 7ace8e:7, 774267:4).
- Already in the prompts? Partly. Techniques "Small things ... read as icons" and "A form is one mass".

**X4. A row of identical lit crowns reads as balls ("every ball lit alike"). Shade the whole crown from one big form, then relight the clusters.**
- Kind: craft / easel (form{}). Evidence: "It read as broccoli, every ball lit alike, so I darkened the whole right side and the undersides with a stipple driven by one big crown ellipsoid. The crown now has a single light" (cfa19c:8), later refined (cfa19c:16, 19).
- Already in the prompts? Partly. Techniques "A form is one mass".

## Composition and looking

**V1. Plan the foreground and land dark from the start. In these evening pictures, land laid in at sky value took three or four glazes to bring down.**
- Kind: craft / process. Evidence: "What I would do differently: plan the foreground as dark from the start (it took four glazes to find it)" (96b5e4:8). "the whole picture is now far too close in value: the snow matches the sky" (7ff345:2), fixed only at 7ff345:16 ("At last the sky's glow stands clear of the ground in the squint"), roughly 70 days later. "squinting showed the land as light as the glow" (9aae10:9). "the range is too narrow, mud barely darker than water" (75c237:12). Also 9cf69b:8, 4fb403:3, 1def1a:4.
- Already in the prompts? Partly. Techniques "The full value range" and "Plan in values". Brief: "After the first large masses, consider whether the whole picture..."

**V2. Set the scale of trees, animals and figures from the eye height before painting them.**
- Kind: craft / process. Evidence: "By that measure my pollard willows are 1.5-2 m high: shrubs, not willows. The horses are toys too" (358aea:14), which forced repainting the whole row. "plan the scale of trees and animals from the eye height before painting anything" (358aea:21). "a stream 4 m wide 600 m off would be a hair, not a band" (497bcf:8). "at 11 units long they are blobs" (497bcf:18). Successful upfront checks: 497bcf:12, 7ace8e:16 ("Sizes set from an eye about 8 m above the plain").
- Already in the prompts? No. The guide has `world{}`, `w:scale_at(Z)` and `w:height`, but no prompt says to use them for scale.

**V3. Flat land in perspective needs converging edges and horizontal shelves. A channel that widens into the distance and domed banks read as cliffs and hills.**
- Kind: craft. Evidence: "the channel between them WIDENS toward the horizon, so its edges diverge instead of converging" (75c237:7). "too flat and too domed: they read as hills" (75c237:2, 5). "the channel reads like a road" (4fb403:2). "The river reads as a road: its width is too even" (cfa19c:6).
- Already in the prompts? Partly. Techniques "Forms meet each other physically", "Marks get smaller with distance".

**V4. Contre-jour objects are near-silhouettes: model them in varied darks and keep light to a thin rim facing the glow.**
- Kind: craft. Evidence: "the light is low and behind the tree ... so the tree is contre-jour, nearly a silhouette ... keep lights only as a faint warm rim" (c6b5dd:5-6). "a narrow broken band of warm light ... clipped to a 2-unit band and not blended, so it reads as backlight" (1def1a:11), after a whole-sail blend had failed (1def1a:6).
- Already in the prompts? Partly. Techniques "Shadows take color", "Light is soft and quiet".

**V5. Reflections: draw them freshly into wet water as horizontal broken dashes without blending. Reflections of dark posts are nearly as dark as the posts.**
- Kind: craft / easel. Evidence: "drew the barge reflection freshly into the wet as horizontal hatched dashes ... not blended - it finally reads as a reflection" (1def1a:7). "reflections of dark posts in bright water should be nearly as dark as the posts themselves" (4fb403:12-13). A vertical-stroke reflection blended in a rect failed (75c237:3).
- Already in the prompts? No (glitter only).

**V6. A small element placed where it has no value contrast (a pale moon in a cream sky) disappears. Place it where the ground contrasts with it.**
- Kind: craft. Evidence: "a crescent ... in the cream part of the sky had almost no contrast ... put the crescent higher, where the sky has turned blue enough" (7ace8e:11). "a second smalt-violet glaze on the upper sky, masked round the moon, deepened the zenith and made the moon read" (7ff345:8).
- Already in the prompts? No.

## Process and sittings

**P1. Treat a failed lay-in as underpainting: let it dry and redraw over it rather than fight it wet.**
- Kind: process. Evidence: "Decided to treat all this as an underpainting: let it dry, then redraw the land flatter" (4fb403:2-3). "Treat this as underpainting only" (497bcf:2). "Accepting that as the light underlayer" (3be9e8:5). "Let it dry two days as an underlayer" (358aea:2).
- Already in the prompts? Partly. Techniques "Lay-in: lean, quick, thin".

**P2. Accidents that read as something plausible in the whole view are often worth keeping and developing.**
- Kind: process. Evidence: pale disc becomes earthshine (7ace8e:12). Merged willows become a distant grove (7ace8e:14). A spreading scumble becomes "a low mist lying in the hollow ... Accident kept rather than fought" (7ff345:13). A khaki block becomes a rye field (497bcf:20). A dark strip becomes a hedgerow (497bcf:5). A dark streaky water band becomes wind (1def1a:5). A salt-glaze jug (3be9e8:9).
- Already in the prompts? Partly. Brief: "deliberately accept it with your reason."

**P3. Returning after declaring "finished" and looking whole, close and in value often found one or two real faults, and fixing only those improved the picture.**
- Kind: process. Evidence: "Back with fresh eyes. The mainsail read as a flat cut-out ... Everything else I left alone" (1def1a:11-12). "came back with fresh eyes. The near mud was a flat, empty brown" (4fb403:10-13). "back at the easel after judging it done, and I didn't agree with myself" (358aea:22-23: "the pollards, the path's foot and the town were the three things my earlier 'done' had left unresolved"). Crown and river rebuilt after a first "Done" (cfa19c:16-19).
- Already in the prompts? Partly. Brief "When you feel ready to finish ... Is there something you would enjoy taking further?"

**P4. Stop on a passage when each pass costs as much as it gives.**
- Kind: process. Evidence: "I am calling it finished rather than working the jug further, since every extra pass there has cost as much as it gave" (3be9e8:24). "I stopped before it got worse" (9aae10:12). "Further patching would do more harm than good" (c6b5dd:23). "More work would only muddy it" (1def1a:10).
- Already in the prompts? Partly. Brief: "If the repair recreates the problem, reconsider".

**P5. Use mirror and squint views to catch ruled boundaries and value collapse.**
- Kind: process. Evidence: "Mirror view flagged the ruled meadow/foreground boundary" (96b5e4:8). Squint checks drove the major corrections (9aae10:9, 75c237:12, 7ff345:16).
- Already in the prompts? Yes. Guide `look` modes. Techniques "Plan in values".

## Tool confusion

- **Medium missed entirely.** "The guide has pile medium (0-0.95) for transparent glazes, which I had missed" (3be9e8:17), on day 25 after many failed glazes. A prominent glaze recipe line prevents this. Today's techniques has one.
- **Clip on soft masks** assumed to soften (3be9e8:5), refuted at 3be9e8:8. See B6.
- **Subtracting masks doesn't protect against unclipped strokes.** "Subtracting masks did not protect them (studio notes said so; I forgot)" (c6b5dd:15). Covered in guide "What stays inside the mask".
- **Open curves in `below`/`above`.** See G1.
- **Noise `stretch` takes positional values** (9aae10:10). Now in the guide.
- **Chunk files via an unset variable ran another studio's file** (cfa19c:2). This is a harness or workflow issue, not a paint one.
- **An endless loop in a chunk hung the easel**, recovered by replay (3be9e8:14).
- **"Glaze" read as any thin coat**: painters put white or umber in glazes (Z1).

## Time sinks and rescue cycles

- **Dolmen, 9aae10 days 25-114**: snow cap, granite mottling, glaze spill, rematching the plain. Got out by repainting the stones opaque in one body and rematching with small swatches (9aae10:11).
- **Jug, 3be9e8 days 1-48**: opaque shadows swallowed by white and blends flattening the form. Got out by switching to transparent glazes over a dry light body (3be9e8:17-19).
- **Sky patches around the oak, cfa19c days 20-133**: patches never matched, and an unclipped sky repaint carved the crown. Got out by whole-passage repaints with clipped strokes and blends (cfa19c:14, 18).
- **Patch cycles in c6b5dd days 12-42**: column, pale strip, glow spill, wisps, low-sky patchwork. Each local fix made a new seam. Got out by repainting whole bands (c6b5dd:12, 21).
- **Town halo, 7ace8e days 95-108**: a glaze spill was cut back with sky color, which made a halo; badgering the halo chewed the spires. Got out by restating the whole silhouette from its mask (7ace8e:24).
- **Willows, 358aea days 15-90 and 497bcf days 25-94**: lollipop to broccoli to palms to mushrooms. Got out by fixing scale from eye height (358aea:14) and by building fans of rods with lobes hung over them (358aea:22, 497bcf:33).
- **Land value, 7ff345 days 2-73**: four glazes, the first ones smalt-only and too weak, until Prussian and black went in (7ff345:12, 16).

## Moments the approach changed and the picture improved

- 3be9e8:13: blended modeling dropped for seven graded piles in curved strokes; the lemons "turn now".
- 3be9e8:17-19: opaque shadows dropped for medium-0.88 glazes; "The jug turns now."
- 9cf69b:10-11: three small glaze trials, then a `load_at`-graded veil; "The snow now darkens toward us."
- 1def1a:7: reflection redrawn as unblended horizontal dashes into wet water.
- 75c237:7-8: diagnosing diverging channel edges and making them converge.
- c6b5dd:5-6: rethinking the tree as contre-jour silhouette.
- cfa19c:8: one big crown form for shading instead of per-clump lighting.
- 358aea:14: rescaling everything from eye height.

## Trials, scratch, sittings, finishing

- No scratch canvas in engine 1. Painters tested on canvas corners and margins (9aae10:2, 9cf69b:9-10) and lifted or painted over the tests. Both A1 and A2 named "test every glaze small first" / "swatches before every patch" as what they'd do differently (9cf69b:11, 9aae10:12).
- Sittings are logged as long simulated waits of 2 to 14 days between campaigns. The deliberate wait-before-crossing rule (9aae10:8) cut wet-over-wet failures.
- Day-1 still lifes (7b31aa, b5323f, 6399ad) finished or stopped within hours with no drying between layers and little self-critique. 6399ad's journal ends mid-plan. My interpretation: these lanes didn't use time.
- Several painters reopened after "finished" and improved the picture (P3). Several logged "replay check matches exactly" when closing (c6b5dd:24).

## Ten strongest candidate prompt improvements

1. **Repaint whole passages, don't patch graded fields.** Let it dry, relay the whole sky, water or field as one wet layer, cut around what's in front, and blend it whole. (c6b5dd:11-12, c6b5dd:20-21, cfa19c:13, cfa19c:18, 9cf69b:7, 497bcf:16-17, 1def1a:7)
2. **Blend whole fields or passage-shaped masks, never rect zones.** A zone blend leaves its outline. (3be9e8:2, 3be9e8:7, cd267e:3, 7ace8e:6, 75c237:3, 75c237:13, b5323f:5)
3. **Clip every blend and glaze near anything wet, especially fresh darks.** It is still the most repeated failure even with the notes. Now partly handled by the clipped-by-default blend; the glaze is still unclipped. (cfa19c:5, cfa19c:14, c6b5dd:17, 358aea:17, 4fb403:7, 497bcf:16, cd267e:9, 9aae10:11)
4. **Glaze recipe line:** low-hiding tubes only (smalt, Prussian, green earth, lakes), no white, no earths. Add Prussian or black when smalt is too weak. Trial the medium and load first. (7ff345:14, 7ff345:11-12, 3be9e8:6, 497bcf:31, 9cf69b:9-10, 497bcf:6)
5. **Shadows on pale objects from glazes over a dry light body**, not opaque darker mixes, which the white swallows. (3be9e8:25, 3be9e8:17-19, 774267:11)
6. **Don't badger small modeled forms or darks.** The 40-unit blender flattens modeling and lifts darks toward the layer beneath. Model with graded piles in direct strokes, and glaze darks when dry. (3be9e8:3, 3be9e8:11, 3be9e8:13, cd267e:10, 4fb403:4, 1def1a:6)
7. **Set scale from eye height before painting trees, animals and figures** (`world{}` / `w:scale_at`). (358aea:14, 358aea:21, 497bcf:8, 497bcf:18, 7ace8e:16)
8. **Plan the land and foreground dark from the start in evening and backlit pictures.** Check the squint early. (96b5e4:8, 7ff345:2, 7ff345:16, 9aae10:9, 75c237:12)
9. **Masks must span the canvas and stay current.** `below`/`above` curves must cross the full width, and passage masks must subtract later foreground objects. (9cf69b:6, c6b5dd:10, cfa19c:9, 358aea:16, 774267:11, cfa19c:18)
10. **Mix deeper than the recipe says when matching aged or glazed passages, and keep white out of darks.** Fresh white mixes read paler than the passage they patch. (cfa19c:12, 9aae10:11, 7ace8e:11, 3be9e8:17, 9cf69b:11)
