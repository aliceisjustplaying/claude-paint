# Journal mining: extract B

Batch B: 20 journals, rounds r18-r20, all engine 1, default box (NEW1 hopper box). Citations are
`<studio>:<line>` in `~/src/a/paint-studio-<studio>/notes/journal.md`. "Guide" is
`claude-paint-r31run/notes/easel_guide.md`, "studio notes #N" is `gallery-fcf9c110/r31/studio_notes.md`,
"techniques" is `claude-paint-r31run/notes/techniques.md`, "brief" is `round31/BRIEF-draft.md`.

| studio | lane | lines | studio | lane | lines |
|---|---|---|---|---|---|
| 2564f4 | MUSE r18 | 10 | 851a37 | GEMF1 r19 | 52 |
| c71699 | GEM r18g | 15 | 2a634e | MUSEF1 r19 | 15 |
| f705c2 | MIMO r18 | 13 | 11997b | MIMOF1 r19 | 4 |
| 3f743e | GLM r18 | 14 | ebf843 | CTEST1 r19 | 8 |
| db6324 | LUNAB1 r19 | 5 | 919cfd | KIMIF1 r19 | 6 |
| f0e4ab | LUNAF1 r19 | 10 | 58dfc6 | SONF1 r19 | 34 (30 KB) |
| 520026 | BUN r18 | 51 | 6e8898 | SELF1 r19 | 35 |
| c766c5 | DSK r18 | 12 | bfd78b | SONB1 r19 | 11 (10 KB) |
| 69c343 | GEMB1 r19 | 86 | b3a3c7 | SELFH1 r19 | 5 |
| 3cab71 | NEW1 r20 | 7 | e708c7 | NEWF1 r19 | 17 |

## Summary

- The dominant engine-1 failure is **wet paint moved by a later pass**: blends and glazes dragging a
  neighbor that was painted minutes earlier, dark corrections over wet pale coming out pale, details
  dissolving into wet beds. Nearly every long journal records it, several more than once
  (58dfc6 records the same class of mistake six times). The cure every painter converged on: check
  `drying()`, wait until dry, then correct.
- The second is **masks that don't mean what the painter thinks**: unclipped passes running past the
  edge, exclusion masks that forget twigs, figures or reeds standing in front, rectangular patch masks
  leaving rectangles, blurred-then-subtracted masks leaking, `edge="lost"` throwing a near-black over
  the whole canvas.
- Third: **coverage and load are not opacity**. Thin films over dark bloom, coverage 2-3 is a veil,
  `load_at` does not fade dark paint, coverage counts layers.
- Process: premature "finished" verdicts overturned on the next look at 1:1 are the norm (520026 three
  times, c766c5 three times, 2a634e three times, 58dfc6 twice, 851a37 twice). Long rescue cycles on one
  passage (a boat, a trunk foot, a cumulus) ended only when the painter stopped, simplified or switched
  technique.
- Three GEM-lane journals (c71699, 69c343, 851a37) are mostly narrative self-praise with few
  observations; the r19 restrained lanes (db6324, f0e4ab, ebf843, e708c7, 3cab71) are short, quiet and
  observational and record few disasters.

---

## Ground and drawing

**1. A toned ground left bare can serve as a halftone.**
- Kind: craft
- Conditions: warm umber/ochre ground, portrait flesh; engine 1.
- Evidence: "Warm umber/ochre ground, left bare in a few places as the flesh halftone - classic and it
  saved a lot of paint" (6e8898:10-11). Not tested further.
- Already in the prompts? partly (techniques "Ground": "A toned ground ... sets the middle value";
  studio notes #2).
- Still valid? yes; ground behavior isn't engine-dependent per guide.

**2. Thin blues over a warm cream ground come out lighter and grayer than the pile; mix darker than you
think.**
- Kind: easel
- Conditions: warm cream two-layer ground; smalt 3 : cobalt 1 : white 1.2; engine 1.
- Evidence: "thin blues over it come out light and slightly greyed (smalt 3 : cobalt 1 : white 1.2
  reads as a mid periwinkle-blue, not dark)" (58dfc6:2). Consistent with 11997b:4 ("the wood pile goes
  lilac over pale ground").
- Already in the prompts? partly (studio notes #2, #6).
- Still valid? likely; studio notes #6 says the same for thin films.

**3. An H-pencil underdrawing is invisible at whole view and fine as a guide.**
- Kind: easel
- Conditions: H pencil on cream ground; engine 1.
- Evidence: 58dfc6:2.
- Already in the prompts? partly (studio notes #52 covers graphite under paint).
- Still valid? yes.

**4. A broad toning pass with the glaze hand leaves separate loud strokes; tone with a dense opaque
body pass instead.**
- Kind: easel
- Conditions: toning a whole canvas; `hand="glaze"`; engine 1.
- Evidence: "First toning pass failed: glaze hand laid separate loud strokes, ground showing between"
  (3f743e:2); fixed with "dense lead/umber/black/ochre broad pass x2, blended" (3f743e:3). Same finding
  for a sky: "the glaze hand alone leaves open strokes; needs coverage 2+ with fill, then one blend"
  (919cfd:2).
- Already in the prompts? partly (studio notes #20 on gaps; techniques suggests toning in `canvas{}`
  ground, which avoids the problem).
- Still valid? probably; `work` still leaves gaps without `fill` (guide table).

## Color mixing

**5. Raw umber is a mid-value earth: a dark needs bone black, and umber + white is mid-gray.**
- Kind: easel
- Conditions: piles judged as "darks" on the palette; engine 1.
- Evidence: "raw umber is a LIGHT earth. A 'dark' pile of raw umber + lead white is mid-value; you need
  bone black in it to get a dark" (520026:16); restated at 520026:31. Background "nearly lost twice by
  glazing it with a pile that is lighter than I judged" (520026:5).
- Already in the prompts? no (the tube table gives hiding and strength, not value; techniques suggests
  `look --palette`).
- Still valid? likely (tube optics, not drying); `look --palette` now exists to check it.

**6. Judge a pile against the ground it will lie on, not on the palette: the same pile is lighter and
more saturated over a pale passage than over a dark one.**
- Kind: easel
- Conditions: thin films, earth and vermilion piles; engine 1.
- Evidence: "a pile has to be mixed against the ground it will sit on, not judged on the palette"
  (520026:27); "thin films of earth lighten over a pale base, so the shadows needed building in several
  full passes" (c766c5:9); thick piles matched by eye came out "yellow-khaki on the black" because "the
  visible bank is a thin film that reads lighter than any thick pile" (58dfc6:29). Resolved by reusing
  the pile that had made the bank (58dfc6:30-33).
- Already in the prompts? partly (studio notes #5, #6; brief's trial paragraph: "A held pile shows
  thick paint on steel; use the laid trial").
- Still valid? yes; studio notes #6 confirms on current engine.

**7. To match an existing passage, reuse the pile and handling that made it rather than mixing a new
pile by eye.**
- Kind: process
- Conditions: repairs on a dark bank; engine 1.
- Evidence: "OK.base ... is the pile that matches the bank" (58dfc6:30); "thick OK.base over dry paint
  matches the bank exactly" (58dfc6:33); "No more colour matching by eye of thick piles" (58dfc6:30).
- Already in the prompts? no.
- Still valid? yes (interpretation: follows from studio notes #5).

**8. Some mixes go gray or violet unexpectedly: green earth + raw umber in lemon mid-tones, umber +
cobalt knocks, ochre + red earth for "earthenware".**
- Kind: craft
- Conditions: various; engine 1.
- Evidence: "avoiding green earth + raw umber in mid-tones, which read grey" (f705c2:11); "The grey
  overpaint went grey-violet-mauve - the umber/cobalt mix is too cool" (3f743e:7); "yellow ochre + red
  earth is a pumpkin, not earthenware. Earthenware wants raw umber + red earth + a little lead white"
  (520026:31).
- Already in the prompts? partly (techniques: "`look --palette` ... a violet 'grey' ... show there").
- Still valid? yes.

**9. Warm transparent glazes make a glow luminous; lead-white veils make it chalky.**
- Kind: craft
- Conditions: horizon glow over dry sky; chrome yellow + vermilion with lots of medium; engine 1.
- Evidence: "my lead-white glazes (veil, mist) made a flat cream slab" (58dfc6:5); "Warm glazes of
  gold/orange (lots of medium) over the horizon zone worked far better than the white veils"
  (58dfc6:6). Confirmed in final reading (58dfc6:28).
- Already in the prompts? partly (techniques "Watch the white", "Glazing ... rescues a chalky passage";
  studio notes #8).
- Still valid? yes.

## Thin vs thick paint

**10. Coverage is not opacity: to bury a strong old color takes high coverage in several passes on dry
paint.**
- Kind: easel
- Conditions: body hands over dry saturated vermilion; engine 1.
- Evidence: "COVERAGE IS NOT OPACITY. A pass at coverage 2-3 is a veil. To bury a strong old colour
  (saturated vermilion) I had to go to coverage 8-10 in two passes" (520026:27); "three crossing passes
  of the near-black at coverage 5 (a single pass left the old vermilion ghosting through)" (520026:49);
  "the pear is transparent and muddy, the table showing through it" (c766c5:6).
- Already in the prompts? partly (studio notes #2, #11, #20; guide says coverage is "layers of strokes").
- Still valid? probably; may depend on the load model, which has changed (unverified).

**11. Small, thin applications of a lead-white or vermilion pile over a dark glow like lamps; lay small
lights as single thick touches or with a white-free mix.**
- Kind: easel
- Conditions: highlights on dark fruit; `fill=true` over small masks; engine 1.
- Evidence: "THIN FILMS BLOOM ... I made three separate glowing highlights and had to paint them out ...
  fill=true over a small mask beads the paint into cottage cheese; a single touch is the only clean way
  to lay a small light spot" (520026:29). Related: "A pile much LIGHTER than what is under it reads as
  orange spray" (520026:18).
- Already in the prompts? partly (studio notes #8, #15 on a pressed touch).
- Still valid? likely.

**12. Coverage counts layers: sparse marks need coverage below 1 and `fill=false`.**
- Kind: easel
- Conditions: bark ridges on a trunk via `work`; engine 1.
- Evidence: "work(coverage=2.4) means 2.4 LAYERS over every point, not 'sparse' ... For sparse marks the
  coverage must be ~0.3-0.5 with fill=false" (58dfc6:17); restated in final lessons (58dfc6:28). Even at
  coverage 1.1 bark came out as "orange camouflage blotches" (58dfc6:26). bfd78b:9: "with coverage 7
  even tiny loads give visible opaque paint".
- Already in the prompts? partly (guide option table: "layers of strokes over each point on average").
- Still valid? yes; guide definition unchanged.

**13. Varying the load (`load_at`) neither fades nor softens an edge of opaque paint; softness comes
from mask geometry, stipple density or full-width strips.**
- Kind: easel
- Conditions: dark trunk foot, salmon path base on water, fruit value ramps; engine 1.
- Evidence: "work() load falloff does NOT fade dark paint (hiding is too high; even load 0.3 is opaque).
  Softness must come from stipple density, not load" (58dfc6:29); "load_at does not make a soft edge ...
  paint FULL WIDTH strips and vary colour, never shape by amount" (bfd78b:9); load_at gradient "came out
  a patchwork of blobs, because the load varies dip by dip" (c766c5:12); "do NOT use load_at value zones
  (blotchy)" (bfd78b:7). Counter-evidence: "`hb` slices with load_at ramps are a reliable way to build
  gradients" (bfd78b:4), not repeated later.
- Already in the prompts? no (techniques recommends `load_at` to vary impasto; brief bans encoding
  images in amounts).
- Still valid? unknown; load model may have changed since engine 1.

## Blending and edges

**14. Never blend or glaze near paint that is still wet: the blender lifts it and drags it across, even
when the mask excludes it.**
- Kind: easel / failure-pattern
- Conditions: badger `blend` and `hand="glaze"` passes near fresh darks (shore, bank, reeds, tree);
  engine 1.
- Evidence: "the blender dragged the still-open shore into a grey smear" (58dfc6:3); "the badger still
  dragged the wet bank edge out onto the water ... never run blend() within ~25 units of anything wet"
  (58dfc6:4); "The glaze brush lifted the wet dark and dragged it across the water in long smears"
  (58dfc6:11); "never blend a band that crosses fresh dark silhouettes (the oak crown smeared; cost a day
  of repair)" (919cfd:5); "blend mask reached x=780 and dragged wet sky paint over the dry island"
  (bfd78b:7); "a full-sky blend dragged a pale veil over the whole canvas, bleaching the darks"
  (3f743e:4); face lost "once to a blend over an oversized glow mask" (b3a3c7:3).
- Already in the prompts? partly (studio notes #29, #38, #39; techniques: glazes want dry).
- Still valid? partly changed: guide now says `blend` "keeps their paint inside the mask" by default
  ("What stays inside the mask"), so the "excluded but still dragged" case may be fixed; the glaze hand
  is still unclipped (studio notes #43) and studio notes #39 says a soft mask doesn't stop wet passages
  inside it mixing.

**15. Blend each object separately, with a mask a little smaller and softened, at most once or twice
over an opaque base.**
- Kind: easel
- Conditions: fruit, cloth; badger; engine 1.
- Evidence: "Never blend a whole group of touching objects: it drags one fruit's colour onto the next"
  (520026:21); "blend(m) leaves a hard disc if the mask is a shrunk copy of the shape - use
  (m:shrink(3)):soften(3)" (520026:19); "blend() over a lean layer lifts the lead-white ground through in
  pale blobs. It needs a thick opaque base under it first, and even then only twice" (520026:20).
- Already in the prompts? partly (studio notes #41, #39).
- Still valid? partly (blend clipping may differ now).

**16. Bands of a gradient laid full width and blended once fuse smoothly; patching a gradient with a
rectangle never does.**
- Kind: easel
- Conditions: sky gradients, hand=broad coverage 3 fill=true, then one `blend(angle=0)`; engine 1.
- Evidence: "horizontal bands with hand=broad, coverage 3, fill=true, then one blend(angle=0) pass over
  the whole area gives a smooth gradation in one go" (58dfc6:2), confirmed day 29 (58dfc6:6) and in final
  lessons (58dfc6:28); "never patch a sky gradient with a rectangular mask (the pale slab) - repaint
  full-width tonal bands instead" (919cfd:5); rect blend mask "revealed ... cut edges" so the whole sky
  was relaid x=0-1000 (851a37:13). Caveat: blending six wet bands "averaged the lower blue bands into the
  pale ones" (58dfc6:15).
- Already in the prompts? partly (studio notes #47, #48; `piles=` in guide for graded color).
- Still valid? likely; `piles=` may now be the simpler route.

**17. One blend over a broad `fill=true` coat removes its scale texture; more blends lift the ground.**
- Kind: easel
- Conditions: `hand="broad"`, `fill=true`, solid coats; engine 1.
- Evidence: "hand="broad" with fill=true leaves a lozenge/scale texture, and the ONLY thing that fixes
  it is one blend over the coat" (520026:51); "Then ONE blend over the opaque coat to fuse the strokes,
  and no more blending after that" (520026:28); f705c2:2 "blending a lean layer twice lifts paint off the
  weave and leaves a pale lattice".
- Already in the prompts? partly (studio notes #41).
- Still valid? likely.

**18. A clipped pass through a soft (blurred) mask fades the strokes where the mask fades, giving a seam-free
turn on a form; but on dark opaque paint a blurred clip still reads as a hard mass.**
- Kind: easel
- Conditions: fruit lit side over mid shadow color (c766c5); dark trunk foot (58dfc6); engine 1.
- Evidence: "Clipped to a soft mask, the strokes fade out where the mask fades, so the lit side
  dissolves into the shadow with no seam ... they read round at last" (c766c5:12). Contradicting
  condition: "a soft (blurred) mask used as a clip limits nothing on dark paint: opaque dark laid through
  it reads as a hard mass with a soft edge that still shows at 1:1" (58dfc6:33).
- Already in the prompts? partly (studio notes #16: clip "weighted by the mask's value"; #19 unclipped
  soft edges don't soften).
- Still valid? unknown; the two results need a trial (interpretation: the difference may be value
  contrast and hiding).

**19. Blur a mask after you subtract what it must avoid, never before.**
- Kind: easel
- Conditions: mask arithmetic for a pool at a trunk foot; engine 1.
- Evidence: "all caused by blurring the mask BEFORE subtracting: the blur pushed the top edge of the mask
  up ... a dark wedge ... now sits on the water" (58dfc6:31); "subtract the water/bank crest BEFORE
  blurring, never after" (58dfc6:33).
- Already in the prompts? no.
- Still valid? yes (mask math, not engine).

**20. `edge="lost"` throws paint far past the line; never use it on a big dark passage.**
- Kind: easel
- Conditions: occlusion shadow ring, near-black pile, coverage 1.3; engine 1.
- Evidence: "it ran the film right across the whole canvas and blacked out the bowl, the cloth and four
  of the five apples ... keep 'lost' for small detail strokes" (520026:9), repeated as a rule
  (520026:23, 48). 11997b:4 found "soft/lost edge shares" useful on soft country.
- Already in the prompts? partly (guide "Edges": "lost: the passage runs well past the line").
- Still valid? probably; the reach may have changed.

**21. Hard-edged small ellipses read as stickers; soften them before clipping.**
- Kind: easel
- Conditions: cheek accents, portrait; engine 1.
- Evidence: "cheek 'stickers' from hard-edged ellipses used without softening first ... soften() before
  clip=true for any accent smaller than the broad masses" (6e8898:16-19).
- Already in the prompts? partly (studio notes #45).
- Still valid? yes.

**22. A hard edge-light along a silhouette reads as an outline; a sphere reads from lay-in, terminator,
core shadow and a quiet reflected light.**
- Kind: craft
- Conditions: apples; engine 1.
- Evidence: 520026:30. c766c5:11: lemons "carry a hard dark crescent at the terminator", softened with a
  mid ring.
- Already in the prompts? no.
- Still valid? yes.

## Glazing and scumbling

**23. A glaze over a large soft mask leaves parallel streaks and blotchy pools at loaded stroke starts;
it is not a substitute for body paint.**
- Kind: easel
- Conditions: `hand="glaze"` over big areas, unblended; engine 1.
- Evidence: "a glaze over a big soft mask leaves visible parallel streaks" (520026:51); "the column glaze,
  unblended, left blotchy pale pools (loaded stroke starts)" (58dfc6:14); "hand="glaze" drags pale
  streaks - use hand="body" for modelling" (520026:4); glazes on a dark bank "mottle" (58dfc6:32-33);
  "a single glaze-blend over dry ground just marbled it" (c766c5:11).
- Already in the prompts? partly (studio notes #24, #43).
- Still valid? likely.

**24. A glaze veil kills the contrast it was meant to quiet; restate darks by value, put haze only at
the foot.**
- Kind: craft
- Conditions: misty glaze over far shore; engine 1.
- Evidence: "a glaze veil kills contrast it only meant to quiet" (919cfd:5); far shore "washed into a
  ghost: use value, and put haze only at the foot" (58dfc6:10); "a misty glaze softened everything and
  the ruin now reads as a faint ghost" (58dfc6:9).
- Already in the prompts? no.
- Still valid? yes.

**25. Pale glazes and veils over dry textured paint catch the high points as lacy marks.**
- Kind: easel
- Conditions: pale glaze on windows; engine 1.
- Evidence: "The pale glazes caught the high points as conspicuous little lacy marks" (3cab71:5).
- Already in the prompts? partly (studio notes #3, #40).
- Still valid? yes.

**26. A heavy-loaded scumble runs as a visible streak; a high-load ochre glaze over dark goes khaki.**
- Kind: easel
- Conditions: mist scumble wedge; yellow ochre glaze at high load over dark wall; engine 1.
- Evidence: "The mist wedge turned into a pale scumble cascade ... the load was too heavy" (3f743e:6);
  "Wall glow first went on as khaki smudges (yellow ochre glaze at high load over dark)" (f705c2:11).
- Already in the prompts? partly (techniques: scumble "little load").
- Still valid? yes.

## Drying and timing

**27. Dark laid into wet pale lifts the pale and comes out paler; wait for dry before any dark
correction.**
- Kind: easel
- Conditions: halo cleanup, treeline, trunk; engine 1.
- Evidence: "my first halo cleanup ... mixed with the still-open white of the jug and came out pale -
  dark over wet pale lifts the pale. Waited to day 29 ... the same opaque pass over the ring covered
  cleanly. From then on every correction waited for the paint to be dry" (f705c2:9); "Treeline repay
  failed: laid wet into the wet amber haze, came up chalky grey" (3f743e:8); "dark on wet paler paint
  comes out paler - always wait for dry" (58dfc6:17); "dark-on-dark repairs done on wet paint always
  ghost; check drying() first" (58dfc6:20); "wet white under the bowl came up into every repaint"
  (c766c5:3); "RULE learned: dark shapes go on DRY paint" (bfd78b:2).
- Already in the prompts? yes (studio notes #28, #25; techniques "Let layers dry between campaigns").
- Still valid? yes; drying times have changed, so "how long" must come from `drying()`.

**28. Fine marks (stalks, highlights, dabs) laid into open paint dissolve; let the bed set, then they
sit on top.**
- Kind: easel
- Conditions: reed stalks, palette dabs, fruit highlights; engine 1.
- Evidence: "the first stalks dissolved into the wet reed bases ... After an overnight rest the paint is
  tacky, so stalks should now sit on top" (2564f4:6); "the palette's colour dabs were invisible until the
  wood layer was left to actually dry" (b3a3c7:3); "crisp accents need dry ground under them"
  (c766c5:5); "The head and bare arm picked up their wet underlayers" (3cab71:3).
- Already in the prompts? yes (studio notes #26, #32).
- Still valid? yes.

**29. Repeated wet-in-wet passes remix rather than add hiding; let a layer set before a covering coat.**
- Kind: easel
- Conditions: portrait darkening coats; engine 1.
- Evidence: 6e8898:21-23.
- Already in the prompts? partly (studio notes #25).
- Still valid? yes.

**30. On setting or tacky paint, repair with pressed touches or clipped detail, not dragged strokes.**
- Kind: easel
- Conditions: pale mist setting; engine 1.
- Evidence: "strokes dragged across setting pale mist lift it and smear; repair with pressed touches or
  clipped detail work at high coverage, never more strokes" (919cfd:4), applied again at 919cfd:6.
- Already in the prompts? partly (studio notes #31 tacky grabs).
- Still valid? probably.

**31. Waiting times observed: thick dark paint needed 10-12 days before repairs held.**
- Kind: easel
- Conditions: bone-black-based darks, thick; engine 1.
- Evidence: "every correction has to wait for dry (about 10-12 days for thick dark paint)" (58dfc6:28).
- Already in the prompts? partly (guide "Time": bone black touch-dry 3½-5 days, thicker slower).
- Still valid? no as a number; engine drying changed. Use `drying()`.

## Corrections and lifting

**32. Surgical fixes with small masks that cross other objects do more harm than the flaw; repaint the
whole object or field.**
- Kind: failure-pattern
- Conditions: still life, landscape; engine 1.
- Evidence: "Surgical fix-ups with masks that cross other objects do more harm than the flaw. Repaint
  the object instead" (520026:22); rect repair of trunk fork left "rectangular tone steps" (58dfc6:20);
  dark glaze rect "left an olive rectangle with hard top and left edges" (58dfc6:27); "work() with a
  hard-edged rect/glow mask leaves a visible rectangle. Always feather well outside" (520026:14); "a faint
  lighter rectangle of sky" (bfd78b:8); 2a634e:14 "no new small rects".
- Already in the prompts? partly (studio notes #48).
- Still valid? yes.

**33. When a repair recreates the problem, stop: the fifth repair of the same spot made things worse
than the original flaw.**
- Kind: failure-pattern / process
- Conditions: trunk foot (58dfc6), moored boat (3f743e); engine 1.
- Evidence: "Before I started today the foot was fine at whole-picture scale and only had a flat bottom
  edge at 1:1: I am making the picture worse by tinkering" (58dfc6:30); "Every earlier repair of the foot
  introduced a larger flaw (ghosts, wedges, bushes, skirts) ... I am stopping" (58dfc6:34); "The boat area
  has eaten eight attempts and gets worse each time" (3f743e:12), resolved by removing the boat
  (3f743e:13).
- Already in the prompts? yes (brief: "If the repair recreates the problem, reconsider the composition
  or method before expanding it").
- Still valid? yes.

**34. Judge a repair at whole scale as well as at 1:1.**
- Kind: process
- Conditions: any local repair; engine 1.
- Evidence: "stipple clouds of blue-black read as a bush at whole-picture scale; check at whole scale
  after each intervention, not only at 1:1" (58dfc6:33).
- Already in the prompts? yes (brief: "After a local repair, check its effect on the whole picture").
- Still valid? yes.

**35. Over-correction ghosts: paint laid over a reworked zone leaves ghost edges; one full-load
covering coat over the whole zone on dry paint is cleaner than many small ones.**
- Kind: easel
- Conditions: boat zone, old headland; engine 1.
- Evidence: "Painting over a zone repeatedly on the sim leaves ghost edges" (3f743e:12); "A ghost of the
  old headland still shows through in the sky" (bfd78b:2); "one unifying coat of the trunk black over the
  whole trunk ... then stop touching the trunk" (58dfc6:27).
- Already in the prompts? partly (techniques "No undo").
- Still valid? likely.

**36. Accidents can be kept: a smear can become mist, an over-cover can clean a sky.**
- Kind: process
- Conditions: various; engine 1.
- Evidence: "The smear does make a nice mist bank near the sun, so I will keep some of that as haze"
  (58dfc6:3); "Mistake turned gift — the smeared veil dried as thin mist banks" (2a634e:7); "accidentally
  painted the whole upper sky ... out ... happy accident: the sky is now a clean luminous gradient"
  (bfd78b:6); 3f743e:4 reframed a bleaching blend as "misty silver dusk"; 6e8898:24-33.
- Already in the prompts? partly (brief: "You're free to reconsider a passage").
- Still valid? yes.

## Detail and texture

**37. Repeated identical marks read as pattern: glitter, ripples, sparkle and frost need few, uneven
marks.**
- Kind: craft / failure-pattern
- Conditions: water glints, hatch sparkle, frost dots; engine 1.
- Evidence: "Hatch sparkle came out as a dense rectangular carpet" (2564f4:5); "The moon's reflected
  ladder was too regular, so I veiled it back" (f0e4ab:4); "short hatch marks read too evenly at a
  distance" (ebf843:3); "600 flecks in even distribution plus a blend turned the calm lagoon into a
  pattern of blue dashes" (58dfc6:18); "identical white frost dots across the path slabs" (e708c7:15);
  "stipple touches read as a dot-matrix over the weave" (11997b:4).
- Already in the prompts? yes (techniques "Glitter", "Vary the touch").
- Still valid? yes.

**38. For ripples and water streaks use explicit strokes; `work` with a flat brush on water scribbles.**
- Kind: easel
- Conditions: water, flat brush, coverage > 1; engine 1.
- Evidence: "work() with flat brush at coverage>1 on water = hairy chaotic scribble ... make ripples with
  explicit brush:stroke calls" (bfd78b:10).
- Already in the prompts? no.
- Still valid? unknown (stroke planner changed since).

**39. Clouds built from many discs or value zones read as cabbage or cotton; build them as one passage
lit from the silhouette, or as tapered dry-brush bars or stippled strata.**
- Kind: craft / easel
- Conditions: cumulus and strata; engine 1.
- Evidence: "cloud-as-pile-of-discs attempt FAILED (looks like fish scales/cabbage)" (bfd78b:8);
  "Decision after ~300 chunks of fighting cumulus: stop building volumetric clouds ... Go with what
  worked: long tapered dry-brush BAR clouds" (bfd78b:11); "Clouds came out as flat ovals" (2564f4:3);
  stippled strata "(drag=14, coverage ~2.2, width 3) were the right technique" (58dfc6:22), confirmed
  "no blending, so they stay crisp" (58dfc6:23); body + underside cloud bars were "cartoon stripes"
  (58dfc6:22).
- Already in the prompts? partly (techniques "A form is one mass", "Light is soft").
- Still valid? likely.

**40. Twigs: thick limbs as filled clipped polygons, thin twigs as single pointed strokes with pressure
falling to zero; a round brush on a curved wide path splays into hair.**
- Kind: easel
- Conditions: oak, rigger/pointed round; engine 1.
- Evidence: "twigs wider than 1.8 units are filled as clipped polygons (a round brush along a curved
  path splits into bristle fans and looks like hair), thinner ones are single rigger strokes with
  pressure falling to 0" (58dfc6:7); "A pointed brush of width 2.0-2.4 at pressure 0.8 tapering to 0.25
  yields substantive 1.5-unit to 0.5-unit twigs, rather than faint hairlines" (851a37:11); trunks
  redone "as drawn tapered strokes, not masses" (2a634e:5-6).
- Already in the prompts? partly (studio notes #9, #10, #14; `b:gesture` in guide).
- Still valid? probably; brush model changed (point, gesture added).

**41. Small accents need a value close to their surroundings: pale blades on a near-black bank read as
toothpicks, a warm rim on a backlit trunk as a glowing crack.**
- Kind: craft
- Conditions: grass, rim light; engine 1.
- Evidence: "new blades ... came out pale straw-yellow on a near-black bank: a row of toothpicks ...
  redo the blades in a dark olive (about 40% lighter than the bank at most)" (58dfc6:24); "the orange
  'rim light' on the trunk reads as a glowing crack; a backlit tree should have almost none" (58dfc6:22);
  "mushroom-pale boulders with bright stick grasses" (2a634e:14); "the stems are bright white shards"
  (520026:8).
- Already in the prompts? no.
- Still valid? yes.

**42. Fine surface weathering: lay a broad wet film first and badger it, then add fine touches over the
dry film.**
- Kind: easel / process
- Conditions: stone faces, path slabs; engine 1.
- Evidence: "The first small-brush weathering made conspicuous separate starts. A quicker, broader wet
  film followed by the badger gives the stone a continuous, shallow rise instead; the finer mineral
  touches belong over that dry film" (e708c7:12), repeated on path slabs (e708c7:16-17).
- Already in the prompts? partly (studio notes #40).
- Still valid? likely.

**43. Stipple lays soft tone pattern-free when it is darker than what is under it; to knit scattered
pale dots, stipple a mid value darker than the dots.**
- Kind: easel
- Conditions: stipple over lighter/darker passages; engine 1.
- Evidence: "Stipple is the right tool for soft tone: a dark pile over a lighter passage lays smooth and
  pattern-free. A pile much LIGHTER than what is under it reads as orange spray" (520026:18); "knit the
  speckle ... with a mid pearl-gray stipple (darker than the white dots, so they sit as its lights)"
  (919cfd:6). Counter: "a regular stipple mesh" over pale fruit (c766c5:9); mist stipple left "white
  sparkle dots" (58dfc6:25).
- Already in the prompts? partly (studio notes #35).
- Still valid? likely.

**44. A ring-shaped stipple or shadow at an object's edge reads as a dirty halo; a contact shadow is a
pool under the object offset away from the light.**
- Kind: craft / failure-pattern
- Conditions: still life; engine 1.
- Evidence: "no ring-shaped shadow stipple at an object edge (that is the dirty halo I keep getting)"
  (520026:23); "A contact shadow must be a pool just under the object and offset away from the light, not
  a ring around its base" (520026:34); "the earlier 'cut-in' band had left dark halo rings around jug and
  lemons" (f705c2:8); "Severe pale halos encircle the pitcher and quince, caused by polygon margin
  cutouts in the previous wall glazing pass" (69c343:61).
- Already in the prompts? partly (techniques "Forms meet each other physically").
- Still valid? yes.

**45. Value ramps built as nested discs posterize into rings.**
- Kind: easel
- Conditions: fruit modeling; engine 1.
- Evidence: "Tried a nested-disc value ramp: it posterized into visible concentric rings, and blending
  the rings left wood-grain" (c766c5:12).
- Already in the prompts? no.
- Still valid? yes.

**46. Soft forms painted as soft wet gradients fuse to mush; on dry ground use harder shapes: mid body,
shadow with a curved terminator, lit side, reflected rim.**
- Kind: craft / easel
- Conditions: fruit; engine 1.
- Evidence: "painting soft gradients on open paint fuses them to mush. The fix is dry ground and harder
  shapes ... they finally have volume" (c766c5:11); value ladder plan "each pass opaque OVER the one under
  it so nothing thin ever sits on the dark" (520026:44).
- Already in the prompts? no.
- Still valid? likely.

## Composition and looking

**47. A high-key picture needs real darks; check value early.**
- Kind: craft
- Conditions: estuary; engine 1.
- Evidence: "value study shows nothing darker than mid-grey ... Drama must come from CONTRAST now"
  (bfd78b:5), after ~200 chunks; "deepen the darks — the picture has no real dark yet" (bfd78b:3).
- Already in the prompts? yes (techniques "The full value range").
- Still valid? yes.

**48. Restraint in the late stage: leave broad intervals quiet, add few marks, stop before detail
weakens the mood.**
- Kind: process
- Conditions: r19 landscape lanes; engine 1.
- Evidence: db6324:5 "decided not to sharpen the small figure or add more water marks"; f0e4ab:8 "these
  last marks should let the water breathe, not turn it into pattern"; ebf843:7; e708c7:6 "stopping before
  additional detail weakens the solitude"; 3cab71:5-6 "The picture should remain about waiting, not about
  inventory"; 2564f4:9 "The paint taught me restraint".
- Already in the prompts? partly (brief "Before adding repeated small details, improve a weakening
  relationship").
- Still valid? yes.

**49. Two touching forms need a dark crease between them or they merge.**
- Kind: craft
- Conditions: lemons; engine 1.
- Evidence: f705c2:5, kept at f705c2:13.
- Already in the prompts? no.
- Still valid? yes.

**50. A form lit by a hand-placed coordinate needs every part to follow it (a handle got the body's
shadow).**
- Kind: craft
- Conditions: jug handle; engine 1.
- Evidence: "the handle sat far from that coordinate and was painted entirely in shadow by mistake ...
  fixed by giving the handle its own light coordinate" (f705c2:10).
- Already in the prompts? partly (`form{}` in guide).
- Still valid? yes.

## Process and sittings

**51. "Finished" judged at whole view is routinely wrong at 1:1; look whole and close after the paint
has dried before declaring.**
- Kind: process / failure-pattern
- Conditions: all; engine 1.
- Evidence: "Looked at it whole and at 1:1 and I do not agree with my last entry: it is not done"
  (520026:8; again 520026:23, 37); "What I had called finished still read flat" (c766c5:9; again
  c766c5:10); 2a634e:10, 14; 58dfc6:29; 851a37:23, 31; f705c2:8; e708c7:7, 11, 15.
- Already in the prompts? yes (brief: "use `look` ... explore detail crops in normal color after your
  latest changes").
- Still valid? yes.

**52. Write the campaign plan down in order before a large repaint; each stage before the next goes
over it.**
- Kind: process
- Conditions: full repaint of a still life; engine 1.
- Evidence: "This is the last full repaint; I am writing the plan down first this time so the work is
  deliberate" (520026:37-48), with paint order rules ("All cloth work now goes through free = cloth -
  bowl - fruit, and the fruit are painted last", 520026:32). Outcome: next entry reports a cleaner ground
  and cloth (520026:49); journal ends mid-repaint, so not confirmed to the end.
- Already in the prompts? partly (brief "Work one campaign at a time").
- Still valid? yes.

**53. Paint order: background and anything behind first, objects in front last; any exclusion mask must
include everything standing in front (twigs, figure, reeds).**
- Kind: process / failure-pattern
- Conditions: landscape with tree and figure; engine 1.
- Evidence: "any mask that says 'not the tree' must include the twigs" (58dfc6:10); sky mask "did not
  exclude the tree, so opaque sky went over the upper crown" (58dfc6:13); "any object standing in front
  of the water needs to be in the exclusion mask, or be painted after the water" (58dfc6:14); tufts and
  reeds covered when relaying water (58dfc6:12); "chunk 309 covered the island with sky by mistake"
  (bfd78b:10); "I laid cloth passes that painted straight over the fruit" (520026:32).
- Already in the prompts? partly (guide Depth: `behind=`/`visible=`; techniques "Clip a pass near
  anything it must not touch").
- Still valid? yes.

**54. Keep the coordinates of drawn elements in globals so they can be repainted after an accident.**
- Kind: process / easel
- Conditions: tree twig lists; engine 1.
- Evidence: "Twigs are not lost: BR2 / BR still hold their coordinates" (58dfc6:10); "I have the
  coordinates of every limb ... so nothing is lost" (58dfc6:13). Caveat: "replaying a seeded loop but
  painting only part of it does NOT reproduce the original marks, because brush strokes consume the
  random stream" (58dfc6:33).
- Already in the prompts? partly (techniques "Masks from earlier chunks": keep as globals; guide on
  deterministic randomness).
- Still valid? yes.

**55. Test swatches painted on the painting itself have to be painted out later and get caught by
blends.**
- Kind: process
- Conditions: swatches in a strip "the dark foreground will cover"; engine 1.
- Evidence: 58dfc6:1, then blend "dragged my swatches at the bottom because they were still wet"
  (58dfc6:2); "test swatches painted in the water at bottom right ... must be painted out" (bfd78b:7);
  four swatches to cover after 12 days (58dfc6:29); "sunk the orange test remnant" (2a634e:13).
- Already in the prompts? yes (brief and guide now offer the scratch canvas).
- Still valid? the scratch canvas removes the need.

**56. Narrative journals record little that helps: entries that describe intent in grand terms
("museum-quality masterwork") without observation repeat the same flaws.**
- Kind: failure-pattern (interpretation)
- Conditions: c71699, 69c343, 851a37; engine 1.
- Evidence: 69c343 declared "complete" four times (69c343:56, 70, 78) after defects found at 69c343:60-64;
  851a37 rebuilt the sky across the full width at least four times (851a37:13, 20, 25, 47) and the last
  entry is a plan, with the oak still not painted (851a37:44-52); c71699's entries are almost entirely
  descriptive.
- Already in the prompts? partly (brief asks to "keep useful observations ... substrate and wetness,
  recipe ratios").
- Still valid? yes.

## Tool confusion

**57. Unclipped body/broad/glaze passes run past the mask by a stroke length.**
- Kind: easel
- Conditions: garment next to hair, background glaze next to face; engine 1.
- Evidence: "unclipped broad/body strokes run well past their mask by their own stroke length - my first
  garment pass shot dark blue-grey strokes deep into the hair" (6e8898:12-15); "`work`/`blend`/`glaze`
  calls are NOT bounded by the mask you pass unless clip=true ... Lost the face three times" (b3a3c7:3);
  "clip=true with the field mask minus the objects is the way to hug a contour" (f705c2:8).
- Already in the prompts? yes (guide "What stays inside the mask"; studio notes #16-17; techniques "Clip").
- Still valid? yes for body/broad/hatch/glaze/scumble; blend now clipped by default.

**58. `b:load` adds to what is on the brush; reload or wipe when switching colors.**
- Kind: easel
- Conditions: palette dabs; engine 1.
- Evidence: b3a3c7:3.
- Already in the prompts? partly (guide: `b:reload` "wipe most of the old paint off").
- Still valid? yes.

**59. A full load per stroke (`dips={1,1.0,0}`) avoids starved streaks in big solid coats; a small
brush with short strokes covers better than a wide one with long strokes.**
- Kind: easel
- Conditions: solid coats over large areas; engine 1.
- Evidence: "With dips={3,...} the load runs out along the stroke and I get starved streaks" (520026:28);
  "a filbert 20 with 40-110 unit strokes at coverage 2.4 over 250k units lays a thin veil. filbert 12,
  length 24-64, dips={4,1.0,0.1} and fill=true gives a solid coat" (520026:17); "fill=true makes leopard
  spots" (520026:2).
- Already in the prompts? partly (studio notes #11).
- Still valid? likely; load model may have changed.

**60. API errors that cost chunks: `below(600)`, `coverage=` as a function, `table.unpack` on a
string-keyed table, `local` helpers between chunks.**
- Kind: easel
- Conditions: engine 1.
- Evidence: "`below(600)` is an error ... cost me three failed chunks" (520026:33); "coverage= as a
  function is NOT supported in this build" (520026:13); "An option table with only string keys has no
  array part, so table.unpack(t) returns NOTHING and every option is silently dropped" (520026:15);
  "functions defined with `local` in one chunk do not exist in the next" (58dfc6:7).
- Already in the prompts? partly: `below` takes a curve or list (guide "Masks"); locals (guide "How
  chunks behave"); coverage now accepts a function (guide `work` table), so that one is obsolete.
  `table.unpack` silently dropping options: no.
- Still valid? `coverage` function: no longer true. Others: yes.

**61. Calling `look` in parallel with `paint` returned no image; look after the paint returns.**
- Kind: easel (harness)
- Conditions: r19; engine 1.
- Evidence: bfd78b:1.
- Already in the prompts? no.
- Still valid? unknown.

**62. A blend's angle taken from a form field left cut edges across lit lumps; use constant angles.**
- Kind: easel
- Conditions: cloud shadow band; engine 1.
- Evidence: bfd78b:3; "blend only inside the shape's own mask with constant small angles" (bfd78b:6).
- Already in the prompts? no.
- Still valid? unknown.

---

## Across the batch

**Time sinks and rescue cycles.**
- 58dfc6 (472 painted days): six recorded instances of blend/glaze dragging wet neighbors or masks
  missing front objects (58dfc6:3, 4, 10, 11, 13, 14), then five rounds on the trunk foot
  (58dfc6:19-33). Out: stopping and accepting small faults (58dfc6:34).
- bfd78b: "~300 chunks of fighting cumulus" (bfd78b:11). Out: switching to tapered dry-brush bars.
- 3f743e: the boat "eaten eight attempts" (3f743e:12); out by removing the boat (3f743e:13). The streak
  field likewise: "STOP fighting the streak field" (3f743e:10).
- 520026: three full repaints of the still life; each verdict of "done" overturned (520026:8, 23, 37).
- 851a37: sky relaid full width repeatedly; the oak added, removed and re-planned; ends unfinished.
- c766c5: four campaigns on fruit (c766c5:9-12) before "they read round at last".

**Approach changes that clearly improved the picture.**
- c766c5:11-12: dry ground and harder shapes, then clipped soft lit ellipse over mid shadow.
- 58dfc6:6: warm transparent glazes instead of white veils for the glow; 58dfc6:23 stippled strata.
- e708c7:12: broad wet film plus badger before fine touches.
- 2a634e:5: "unified fog and foreground edge to edge — the picture breathes again".
- f705c2:9: waiting for dry before every correction.
- 520026:49: writing the plan first; one soft light pool instead of a boolean cut.

**Confusion a clearer prompt line would prevent.**
- Coverage read as sparseness or as opacity (58dfc6:17, 520026:27).
- `load_at` assumed to fade or soften (58dfc6:29, bfd78b:9, c766c5:12).
- `edge="lost"` assumed to mean "soften" (520026:9).
- Exclusion masks that forget thin things in front (58dfc6:10, 14).
- Blurring before subtracting (58dfc6:31).
- Unclipped passes (6e8898:12, b3a3c7:3).

**Trials, sittings, finishing.**
- Trials were laid on the painting itself and became repairs (58dfc6:1-2, 29; bfd78b:7; 2a634e:13). No
  scratch canvas existed in r18-r20.
- Return visits after a "final" entry found real defects in most long journals (see entry 51), but
  58dfc6:34 and 3cab71:7 show a return visit that correctly painted nothing.
- Several painters used long waits well between sittings: "Rested the painting five days, then worked
  on dry ground" (c766c5:7).

---

## Ten strongest candidate prompt improvements

1. **Don't blend or glaze within a brush-width of paint that is still open; check `drying()` at the
   edge first.** 58dfc6:3, 4, 11, 28; 919cfd:5; bfd78b:7; 3f743e:4; b3a3c7:3. (Check against the
   current clipped blend before adding.)
2. **Correct darks only on dry paint: dark into wet pale comes out pale and ghosts.** f705c2:9;
   3f743e:8; 58dfc6:17, 20; c766c5:3; bfd78b:2.
3. **Coverage counts layers, not opacity: coverage 2-3 is a veil over a strong color; sparse marks need
   coverage below 1 with `fill=false`.** 520026:27, 49; 58dfc6:17, 26; bfd78b:9.
4. **Exclusion masks must include everything standing in front (twigs, figures, reeds), or paint those
   after; consider `behind=`.** 58dfc6:10, 12, 13, 14; bfd78b:10; 520026:32.
5. **`load_at` doesn't fade or soften opaque paint; soften with mask geometry, stipple density or
   full-width strips.** 58dfc6:29; bfd78b:9, 7; c766c5:12.
6. **Repair a gradient or field full width, not with a rectangular patch; blur masks after
   subtracting.** 919cfd:5; 851a37:13; 520026:14; 58dfc6:20, 27, 31, 33.
7. **Judge a pile against the passage it will lie on; to match a passage reuse its pile; raw umber
   alone isn't a dark.** 520026:16, 27, 31; c766c5:9; 58dfc6:29-30.
8. **Small accents need values near their surroundings (pale blades, rim lights, white stems read as
   toothpicks and cracks).** 58dfc6:22, 24; 2a634e:14; 520026:8.
9. **When the same spot fails twice, stop and change method or drop the motif; compare at whole
   scale.** 58dfc6:30, 33, 34; 3f743e:10, 12, 13; bfd78b:11.
10. **`edge="lost"` reaches far: don't use it on big dark passages.** 520026:9, 23, 48.
