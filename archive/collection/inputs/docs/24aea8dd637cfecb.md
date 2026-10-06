You're back at the easel. The painting is as you left it. Your brief is in BRIEF.md and your journal in notes/journal.md.

## BRIEF.md

# Paint a picture

Compose and paint one original landscape in the manner of Caspar David
Friedrich, at the easel, a simulator of oil paint on linen. The place,
subject and composition are yours to invent. Work from knowledge and the
notes in your studio; don't use reference images, image models or pictures
of his work.

## Your studio
- The folder ~/src/a/paint-studio-3a958c: this brief, your notes and the easel.
- You paint at the easel: paint a chunk, look at the canvas, paint the
  next. Its tools are `paint`, `look`, `note`, `status` and `log`;
  notes/easel_guide.md explains them.
- There is no undo. To change a passage, paint over it, or lift wet
  paint off with a brush.
- You mix your own paint on the palette, from the tubes, in the
  proportions you choose.
- Time passes on the painting's own clock: every mark takes the time a
  hand takes, and `wait(minutes)` lets it rest, as long as you like; paint
  dries as that time passes. Real time between chunks doesn't count.

## The rules of the studio
- Paint shapes and marks, not computed pictures: don't encode an image in
  masks, amounts or proportions.
- Shapes are drawn, not copied: don't make a mask or stroke by mirroring
  or rotating another mask's or stroke's coordinates. A shape that mirrors
  another is drawn as its own shape, not as `m:at(x, 2*H - y)` or
  `m:at(W - x, y)`. Moving a shape and reusing your own helpers are fine.

## What to read
notes/easel_guide.md; notes/studio_notes.md;
notes/research/friedrich_materials.md (his materials and method, sourced);
notes/research/trees.md (how trees are built) and
notes/research/oil_paint_physics.md as needed.

## Working
- You make every artistic decision.
- Look at your painting often, whole and close up.
- Keep a working journal with `note` as you go: your own working notes.
  You can revise them.
- Develop the painting until you judge it complete.

## Your reply
When you stop working, reply with the painting's title if you give it one
and a few sentences about the picture.

## Your journal (notes/journal.md)

Your own notes, quoted as you wrote them:

> - day 2, 17:49: PLAN. Late-autumn dusk on a heath crest above a fog-filled valley, in the manner of Friedrich. Format 1.4:1 (1000x714). Warm ochre/pink ground under a pale lead-white top ground (like his bought, primed linen). Composition: luminous afterglow low right of centre (x~620, horizon y~400); sky blue at top through pale green to lemon/peach at the horizon. Far violet ridge, a dark-blue tabletop sandstone rock at right (x 690-1000) rising from creamy mist, mist sea with small wooded islands + a tiny spire, then the near dark crest at left with one big old leafless oak (trunk base ~x240,y530, crown to y~80) and a small figure in a dark coat seen from behind at the crest right of the trunk (x~430). Dark foreground: umber/Prussian blue, rim-lit by the afterglow. Order: sky -> far -> mist -> islands -> crest -> oak/twigs -> figure -> last glazes. Pencil underdrawing is on the canvas (2H for ridges, HB for oak skeleton); it will be covered by the dark. Test swatches sit in the lower-left strip (y 640-690), to be covered by the foreground.
>   Learned so far: glaze hand + blend gives smooth gradients; `broad` coverage 2.2 leaves ground gaps (cream) that need a glaze pass; wet dark under a fresh pale pass drags up and greys it - let dark layers dry (about a day) before laying pale mist over them.
> - day 5, 00:40: Day 5 status. Sky (smalt/cobalt top -> green-yellow -> peach glow) laid in glazes + blended; far ridge, dark tabletop rock (right), pale dome island, cream mist sea, dark crest foreground, and the oak's trunk + 20 limbs done as flat dark silhouettes (body_of masks filled with `work` along limb direction).
>   Learned: pointed round (width 3) pressure 0.1/0.3/0.6/1.0 -> 0.44/0.59/1.7/3.7 units; body_of limbs give nice crooked tapered limbs; mask:rim+stipple works for tree-lines. Wet dark under pale paint pulls up and greys it - always wait a day.
>   Critique at this point: mist sea is too flat and empty (reads as snow/void) -> needs modulated ridges, tree-lines, a spire, warm/cool variation; rock needs rim light + tiny trees on top; oak is a candelabra of divergent limbs and has no twig mesh yet, trunk base is a bulb (cover with root flare over crest); foreground needs grass/rocks/rim-light; figure not yet painted (plan: x~450, y~553 on crest, 60 units tall, under the long right limb).
> - day 10, 17:51: MISTAKE (day 10): I repainted the sky glow in x 268-1000 without excluding the oak. The oak's right half (L2 right edge, L3, S1, S3, S10-S12 and their twigs, y 130-414) is veiled under opaque cream sky paint, with a dark smear (limb paint dragged while still tacky) and a hard-edged stipple band at y=130. RULE from now on: (1) paint sky/background first or always subtract the oak; (2) let dark limbs go fully dry (bone-black paint stays 'tacky' for 5+ days) before painting anything near them; (3) stipple coverage functions: high where I want it, check direction.
>   REPAIR PLAN: wait ~3 weeks for full dry; re-lay clean opaque sky over x 262-1000, y 125-414 (blue at top matched to existing, pale green, lemon, gold, peach glow at x~650), blend smooth; then repaint the oak's right half (new, more asymmetrical, crooked, oak-like) on top. Also: cover the test twigs in the lower-left strip with ground, fix the bulb-shaped trunk base (roots gripping the crest), add twigs to L5 / drop the thorn twigs S16/S17, modulate the mist sea (ridges, tree-lines, spire), detail the rock, foreground grass/rim light, the figure, final glazes.
> - day 62, 22:09: Day 62 status/lessons. Repaired the sky right of the oak, but then made the same mistake a THIRD time: the final sky coat (skyAll2) protected only the fat limbs, so the whole fine twig network (S* sprouts + N_* sprouts) is now buried under opaque sky. Also the glaze of chrome yellow/vermilion came out too saturated (cartoon gold). Plan: (1) wait for sky to dry; (2) veil the gold toward lemon/peach; (3) re-darken limbs veiled by gold (L1/S6/S7 region x80-290,y300-390); (4) rebuild the twig network LAST of all tree work, after the sky is final, denser, with a mix of thicker secondary branches and fine twigs; (5) mid-ground: modulate the mist sea (cool lilac shadow glaze in the near mist, warm toward glow), ridges with tree-lines, spire; model the tabletop rock (warm lit left face, shaded right face, ledges, tiny trees on top); (6) right foreground bank with 2-3 spruces to frame the right and answer the oak; grass/rim light on the crest; (7) figure ~60 units tall at x~400,y~541 from behind; (8) last glazes. Never paint sky again once the twigs are on.
> - day 67, 00:34: Day 67 BLUNDER: blend(rect) over a region that included the wet rock base dragged dark rock paint sideways -> hard-edged grey rectangle (x>=640, y>=400) over mist, sea and horizon glow. Lesson: never `blend` a hard rect that contains dissimilar wet paint; blend only masks of one kind of paint, or feather with soft masks. REPAIR PLAN (after drying ~4 days): (1) repaint the mist sea x>=560,y>=398 opaque cream/lilac, feathered at the seam with a soft load_at on x; (2) repaint the rock body y 398-480 (rightPlane/leftPlane) and veil its base softly; (3) then REPAINT THE WHOLE SKY once more in a single wet-in-wet pass over x 0-1000, y 0-405 (no oak protected; seams from old partial masks are visible at x~395 and x~500), (4) after it is dry repaint the entire oak (limbs in firm masks, grown 1 unit, + sprouts), because the left crown limbs are olive from the gold glaze. Tree always last.
> - day 138, 10:57: Day 138. SKY IS NOW FINAL-GROUND (one clean wet-in-wet coat over the whole sky, smooth). Oak is entirely buried again (that is OK: now it gets painted last, over a final dry sky). Order from here: (1) dry ~5 days; (2) top-blue deepening glaze + blend over WHOLE sky region; (3) clouds with edge= soft/lost, thin lean paint (previous pills with hard edges looked fake); (4) repair the white smear band on top of the butte (y~345-360, x 745-870) and spire/tree-lines in the mist; (5) foreground crest: backlit grass blades with a rigger, stones; (6) OAK last: trunk w/ bark, 5-6 crooked boughs, clustered twig masses, thin warm rim light on the glow side; (7) figure; (8) maybe 2-3 small spruces lower right; (9) harmonising glazes. Never `blend` a hard rect over mixed paint; always blend the full region of one kind of paint.
> - day 155, 00:08: Day 155. Stage: sky final (smooth, 5 wet-in-wet bands), far ridges + wooded humps + fog sea, butte with alpenglow face + forest crown + needle/tower, foreground crest with olive/warm/cool variegation. The oak and figure are NOT yet repainted (the previous oak is buried under the final sky) - this is the last big task. Plan for oak: limbs trunk, L2re, L1, L4, L5, stub, S2, S4, S6-S9, S14, S15, R1 family, L2r1, L2r2 (skip S1, S3, S10-S12, S16, S17), then twigs via sprout2 with shorter internodes, bark light on right side, then figure at (435,541) ~58 units tall, then small spruces lower right, grass blades on crest, final glazes. Don't touch sky afterwards.
> - day 175, 08:51: Day 175 mistake #4: the 'fog foot' pass used a mask from y=436 down over the whole right valley with opaque coverage -> buried the wooded humps B/E/F/D and the butte base, left a hard vertical edge at x~640, and the elliptical `lens` wisp sits on the butte like a flying saucer. NEVER use lens() for mist; use noise-stretched masks with soft ends, thin glaze or load gradients. Repair after drying: (1) repaint hill B right flank, E, F, D humps over the dry fog; (2) re-lay talus/butte lower part to y~500 and kill the lens; (3) low soft fog veil only by load gradient; (4) faint horizontal line at y~632 (x110-640) in the foreground from the soilWide rect -> glaze the whole foreground; (5) foreground detail: grass blades with rim light, stones, thistles; (6) rim lights on oak right edges; (7) butte: strata + softer, hazier, slightly smaller feel; darken the pale 'tower'; (8) final glazes.
> - day 211, 16:28: Day 211 lesson: load_at that falls to 0 does NOT fade paint to nothing: a nearly empty brush still lays a thin film of the pile, so a 'fade-out by load' leaves a faint veil and a HARD EDGE at the mask boundary (this made the grey-blue band under the butte at y 428). To fade out use a soft mask (clip weights by mask value) or edge='lost' on a mask whose boundary I draw wavy. Plan: wait for dry, then (b) repaint forested foot of butte y 428-490, (a) then cream fog over it with edge='lost' and a wavy boundary at y~486, x 560-1010, then check left seam x~570.
> - day 221, 18:27: Day 221 mistake #5 (same family!): my 'seam softening' fog pass (x470-640, y402-506) was not masked against the oak -> it veiled the right end of the low limb L4 (x>470) and the roots of S15. FIX after drying (~10 days): repaint L4 from x~440 to tip (546,458) + S15 + their twigs in OakD, then a light lost-edge dissolve of the cream's left edge (x 470-540). RULE: build every background mask as `... - treeAllG - figM:grow(2)`; I even have treeAllG; use it ALWAYS. Also pending: tufts on the right crest (lost under fog), stones/blades in the foreground, butte cliff modelling (darker fissured face), birds, rim light on trunk/limbs.
> - day 233, 22:24: Day 233 night, mistake #6: a final 'deepening' glaze over the whole sky region (GBdeep, Prussian blue too strong) was laid while the L4 limb repair (4 h old), the new cliff (1 h) and its needle/knob were still wet: the glaze strokes dragged them across the sky (dark flags around the butte top and at L4) and the top sky turned streaky saturated blue. Lifted most of the wet glaze with ~160 clean wiped strokes (works: a wiped filbert removes open glaze, leaves piled edge lines). PLAN: wait ~14 days for full dry. Then ONE clean opaque wet-in-wet sky coat (T0..T9 bands as on day 138-143 which gave a smooth sky) over skyFull - cliffProt - treeAllG, blended horizontally x3 + vertically x1 + horizontally; NO extra deepening glaze (colour at the top should stay soft, not saturated). Then re-sprout ALL twigs, recording each polyline into TWIGS so a later background fix can exclude ribbons of them. Then butte: simplify the shape, cool/violet-grey, warm edge only, hazier. Then clouds (2-3 thin stratus bars, soft edge), foreground boulders/path, and stop.
> - day 270, 07:25: Day 270 status. Sky is now one clean wet-in-wet coat (blue top -> cream-green -> lemon -> peach at the horizon), smooth and soft; oak limbs (thick) intact, twigs are under the sky (ghost lines only) -> re-sprout all. The butte is redrawn as a cliff (warm lit left, violet shadow right, forest crown), its forested foot needs a softer, fog-dissolved right and lower edge. TODO in order: (1) re-sprout twigs on every limb (sprout2/3, record polylines); (2) butte foot dissolve; (3) 2-3 thin stratus bars + birds; (4) rim light on limbs; (5) last look at whole + value + mirror; stop when the picture is quiet and unified.
> - day 270, 08:36: Day 270: twigs re-sprouted (1360 polylines recorded in TWIGS), oak now convincing. Then I laid a cream coat (PW2b) under the butte foot with clip=true on a soft mask: IMPORTANT LESSON: clip=true makes a CRISP boundary where the soft mask crosses ~0.5; soft masks do not give soft paint edges. For soft edges use (a) edge='lost' (or edge=function(x,y) of 0..1), (b) stipple with coverage function + feather (density dissolve), (c) wet-in-wet blend within one open session, (d) lose(). The coat left a flat cream rectangle (x>=630, y 452-548) with a hard left edge and the butte foot sitting on it like on a table. PLAN: wait ~6 days; then ONE wet-in-wet fog coat over x>=610 (top at Atop+14, left edge lost, bottom crest), bands lilac (near) / cream / warm cream (far) / peach tint, blend h x3 + v x1; then while it's wet stipple the butte's forested foot into it with decreasing density downward (it fuses/dissolves); after drying: swells (glazes), conifer tips, then stalks (dark dry plant stems against the fog, rooted below the crest at x 470-800), then final look.
> - day 317, 16:04: Day 317 mistake #7: the 'mist' pass (V1/V2) was clipped to a rect whose top (y=396) lies ABOVE the horizon, so it covered a strip of the glow sky; its V2 body reached the ridge top so it buried the wooded ridge entirely; the left seam at x~556 is hard and tone-mismatched; edge='lost' + clip=rect made strokes spill to the rect boundary; the thick twigs in x 500-560 were half-covered (guard radius too small, ghost fragments). The cliff foot (y 396-436) is buried under cream. REPAIR (single wet-in-wet session after ~12 days of drying): 1) paint the cliff's lower part wet (same piles and the same irr/shEdge/litEdge functions), fading to lilac-cream toward the fog line Fc(x)~424; 2) the cream fog coat only BELOW the horizon Atop(x) (exact mask) and below the cliff's fog line, left boundary wavy at x~556 with an edge function that is 'lost' only near that boundary, clip to a mask that extends 70 units left of the seam but never above the horizon / below the crest / onto guard; 3) bands (lilac ridge strip at the horizon, warm cream, peach under the glow, lilac near the viewer), 4) blend horizontally x3 on fog only (not on the cliff), then vertically over the cliff foot band. Afterwards (dry): redraw the damaged twigs from TWIGS polylines (x 470-640), small conifers in the fog at the cliff foot, thin lilac swells, cool/violet glaze on the cliff.
> - day 354, 03:43: Day 354 mistake #8: after stippling a haze over the butte's lower cliff, I called blend() on a hard rect of the cliff top (y340-424): the blender dragged the (partly tacky) veil up and left a hard-edged white-grey curtain across the cliff top at y=340. Lesson: a blend rect must not have a hard edge inside a coloured, partly-set passage: the boundary of the rect becomes a visible line. Repair after drying (~14 d): repaint the cliff wet-in-wet in ONE session whose haze is built as a colour gradient in bands of piles (rock -> 70/30 -> 40/60 -> 15/85 -> fog), then blend ONLY over the cliff mask (cliffM itself, so the edge coincides with the silhouette); later add ribs/joints with the detail brush once it has set. Also hide the faint rectangle left at x704-914, y440-464 by feathered stipple of fog colour.
> - day 420, 08:22: Day 420 mistake #9: softHill2 (hills in the fog) produced flat grey slabs with hard vertical left edges and flat bottoms: `edge=` + clip=mask(rect-based) cannot soften a mask whose own boundary is a hard step; soft edges must be made in the PAINT (colour that merges into the neighbour), not in the mask. NEW METHOD for hills/islands in fog: mask = crisp ragged top silhouette, ends/bottom are just vertical/flat cuts BUT the paint there is exactly the fog colour (fraction f(x,y) of fog in the mix, built with piles at fractions 0/.25/.5/.75/1 laid with load_at weights peaking at the local f, wet-in-wet), then blend() over the same mask: the mask boundary disappears because paint==surroundings. Also the 3 spruces on the right crest were buried by later fog coats -> repaint them last. PLAN: wait 14 d; re-run the fog recipe (FogM2, same noise 'bil') to cover the slabs; wait 12 d; hills by the new method; spruces; clouds (right sky only, x>600, guard twigs); final look.
> - day 468, 14:56: Day 468 evening. State: sky fine; oak (trunk, limbs, 1360 twigs) good and last; figure under the right limb; foreground dark with rim-lit grass and three ragged spruces lower right; butte = pasted-looking, too orange and too contrasty for a far object, its foot is a white-speckled stipple; fog sea right half is an empty flat cream; a ghost rectangle (x545-630, y400-575) from my own stipple 'seam' pass which I then coated 2.5 h later while still open (wet-on-wet mixing => ghost). FINAL PLAN (all in dry-layer discipline, waits >= 14 d): (1) wait 14 d; (2) one more fog coat over x380-720 incl. the ghost rectangle, whole valley gradient with the same recipe, left/right ends faded, blended; (3) wait; (4) hazy cool glaze over the butte graded top->foot (and a very light one over its crown), fog-coloured stipple dissolve at the foot; (5) a few soft lilac hills in the fog (paint-merged), (6) look at the whole in value + mirror; fix only what is plainly wrong, then stop.
> - day 535, 03:02: Day 535: recoated the WHOLE valley (x -8..1012, below the horizon) in one wet-in-wet session (FogAll: cream base, ridge-A strip, warm cream, peach, lilac near viewer, bil-noise shadow) with exact guards (limbs, 1360 recorded twig polylines, figure, staff, foot, spruces) and 4 blends. Seams at x=335/380 and the grey slabs are gone. Left hills (A2L, C) and the right lozenge hills are buried -> repaint them by the fogHill method (paint fractions of fog, fade by colour, crisp ragged tops) after 14 d of drying; then church spire in the fog (right middle), dark crest tufts, a last look. Dark crest tufts above the crest line were buried by this coat too.
> - day 566, 15:47: Day 566. Mistakes #10: (a) final glaze on the cliff (GshadeP/GlitP, vertical filbert-26 strokes) left streaky mottled bands and re-darkened the foot so the butte reads as a cut block; (b) endFix stipple around the right hills made white blotchy halos; (c) the right hills are two stiff arches. FIX PLAN: wait 16 d; ONE session: FogAll coat (valley, excluding cliff above footLine) -> blend x4; right after, while open, cliff recoat with the NB recipe (bNew masks; full colours + haze bands + blends v,v,h), then light fogNear stipple at the foot. After drying: 2-3 small irregular wooded knolls (like left hill C: crown dabs + conifer spikes, closed bumps, no endFix), ONE spire in the fog far right middle (x~600,y~430?) maybe; no more glazes. Then final look and stop.
> - day 616, 09:59: Day 616, final stretch. The picture now reads: a great bare oak (left), a small figure with a staff under its low limb on a dark crest, a fog sea that fills the valley, a rock mesa lit by the afterglow far right, a smooth blue-to-lemon-to-peach sky with a few birds and thin cloud bars, three spruces lower right. Remaining touches (after ~20 days of drying so nothing drags): thin cool glaze over the mesa's heavy dark crown band; sparse low-contrast grass blades lower in the dark foreground; then stop. Learned: dry-on-dry stipple with a coverage ramp + feather dissolves edges (fog bank, knolls, butte foot) far better than any mask-edge trick; wet-in-wet blends on a mask with soft (blurred) boundaries leave no seams; keep every blend/glaze mask free of hard edges inside coloured passages; always spare the recorded twig polylines.
> - day 653, 01:22: Day 653 mistake #11: after the wet-in-wet recoat of the mesa I added a 'footExt' region (y up to 452) painted in the mesa's piles and then blended it through a BLURRED rect: the blender dragged the grey into the fog -> grey rectangular skirt under the mesa (x 695-920, y 395-470), and the mesa modelling of day 615 (which looked good) is gone (flat pale lilac). PLAN (after 16 d dry): (1) mesa modelling dry-on-dry stipple as on day 615 (shade on the right with noise edge, warm left, rib) BUT no blends afterwards; (2) cover the skirt with fog-coloured stipple whose coverage is an ELLIPTICAL falloff (smooth to zero 20 units outside the grey), region mask larger than the support; PW2b-matched colour; no blends; (3) stop and look. If the skirt still reads: a second pass of the same ellipse stipple at lower density in lilac PW3c. Then finish: final overall look, title, reply.
> - day 673, 09:47: Day 673 — finished. Final state: 1000x714 oil on primed linen. A huge bare oak (trunk, roots, ~20 limbs, ~1400 recorded twigs) stands on a dark crest at left; a small man in a dark coat with a staff stands under its low right bough, seen from behind, looking over a fog sea. Sky: smalt/cobalt blue above, cream-green, lemon and peach at the horizon, thin stratus bars and a few birds. Far right: a rock mesa with its warm afterglow edge and a dark wooded crown, its foot dissolving into the mist, with a small wooded knoll in the fog. Lower right: three spruces on the crest. Lessons that cost most: (1) never repaint sky/fog over fine twigs without exact guards; (2) soft edges must be made in the paint (dry stipple with a coverage ramp + feather, or wet-in-wet blends over blurred masks) and never with mask edges; (3) wait for full dryness (>= 2 weeks for dark/black paint) before laying pale paint near it; (4) a blend over a hard-edged region leaves its boundary visible. Things I would still change: the mesa reads a little pasted-on; the fog sea on the right is flat; the trunk wants a touch of rim light.

## Your globals

(the 150 most recently assigned of 1116)
- `mA2Lx`: mask(20921 sq units)
- `stipHill2`: function
- `PCp`: pile(lead white 2, smalt 1.9, raw umber 0.55, red earth 0.3, Prussian blue 0.07; medium 0)
- `PCp2`: pile(lead white 1.6, smalt 2, raw umber 0.7, red earth 0.3, Prussian blue 0.1; medium 0)
- `mCx`: mask(13082 sq units)
- `CtopFx`: function
- `PCd`: pile(lead white 1.2, smalt 1.9, raw umber 0.8, red earth 0.3, Prussian blue 0.14; medium 0)
- `PCl`: pile(lead white 2.6, smalt 1.5, raw umber 0.4, yellow ochre 0.25, red earth 0.3; medium 0)
- `cM`: mask(3653 sq units)
- `dotFix`: mask(110 sq units)
- `HNp`: pile(lead white 2.4, smalt 1.8, red earth 0.3, raw umber 0.3, Prussian blue 0.06; medium 0)
- `HNp2`: pile(lead white 2, smalt 1.9, red earth 0.3, raw umber 0.42, Prussian blue 0.1; medium 0)
- `HRp`: pile(lead white 3.4, smalt 1.5, red earth 0.34, raw umber 0.12, Prussian blue 0.02; medium 0)
- `HRp2`: pile(lead white 2.9, smalt 1.7, red earth 0.34, raw umber 0.18, Prussian blue 0.04; medium 0)
- `hR1`: mask(16828 sq units)
- `hR2`: mask(8717 sq units)
- `PWend`: pile(lead white 6, pale smalt 0.7, red earth 0.2, yellow ochre 0.1; medium 0)
- `endFix`: function
- `A2Lb`: table with 12 entries
- `A2Lfn`: function
- `bumpHill`: function
- `mA2Lb`: mask(20403 sq units)
- `C3b`: table with 11 entries
- `Cfn`: function
- `bumpHill2`: function
- `cM2`: mask(3654 sq units)
- `mCb`: mask(13072 sq units)
- `spot`: mask(263 sq units)
- `R1fn`: function
- `R1pts`: table with 9 entries
- `R2fn`: function
- `R2pts`: table with 6 entries
- `mR1`: mask(18278 sq units)
- `mR2`: mask(7406 sq units)
- `bP`: mask(13387 sq units)
- `FogAll2`: mask(150638 sq units)
- `fa2`: function
- `bNew2`: mask(14352 sq units)
- `hb2`: table with 4 entries
- `nShade2`: mask(5166 sq units)
- `nWarm2`: mask(4435 sq units)
- `footReg2`: mask(8137 sq units)
- `lowCover2`: mask(2951 sq units)
- `C3c`: table with 11 entries
- `Cfn2`: function
- `cM3`: mask(3669 sq units)
- `mCc`: mask(12642 sq units)
- `trunkSpots`: mask(3240 sq units)
- `K1fn`: function
- `K1pts`: table with 10 entries
- `K2fn`: function
- `K2pts`: table with 7 entries
- `mK1`: mask(14321 sq units)
- `mK2`: mask(8120 sq units)
- `lift3`: brush(filbert 26, 0% full)
- `liftReg`: mask(59434 sq units)
- `edgeBlend`: mask(10246 sq units)
- `softB`: mask(22253 sq units)
- `FogR`: mask(107883 sq units)
- `fadeR`: function
- `fr_`: function
- `Kbody`: mask(8973 sq units)
- `KbodyP`: pile(lead white 3, smalt 1.7, red earth 0.34, raw umber 0.2, Prussian blue 0.05; medium 0)
- `KbodyP2`: pile(lead white 2.5, smalt 1.8, red earth 0.32, raw umber 0.3, Prussian blue 0.08; medium 0)
- `Kd`: function
- `Kpts`: table with 13 entries
- `Ktop`: function
- `Kup`: mask(2524 sq units)
- `Kx0`: number 598
- `Kx1`: number 792
- `clipHills`: mask(41353 sq units)
- `PWk`: pile(lead white 6.6, pale smalt 0.55, red earth 0.2, yellow ochre 0.1; medium 0)
- `kBot`: function
- `kDens`: function
- `kEnd`: function
- `knollReg`: mask(16856 sq units)
- `soft2`: mask(38374 sq units)
- `seamD`: function
- `seamReg2`: mask(20504 sq units)
- `soft3`: mask(27068 sq units)
- `GfgK`: pile(raw umber 3, bone black 1.2, Prussian blue 0.4, red earth 0.35; medium 0.7)
- `fgK`: mask(122418 sq units)
- `PBod`: pile(cobalt blue 0.43956044, lead white 3.956044, raw umber 1.0989012, red earth 0.7692308, smalt 3.7362638; medium 0)
- `PShd`: pile(Prussian blue 0.25974026, lead white 2.5974026, raw umber 2.2077923, red earth 0.77922076, smalt 4.155844; medium 0)
- `PWrm`: pile(lead white 5.755396, red earth 1.678657, smalt 1.0791367, vermilion 0.2877698, yellow ochre 1.1990408; medium 0)
- `bFace`: mask(11748 sq units)
- `shDens`: function
- `wmDens`: function
- `CrownD`: pile(raw umber 1, Prussian blue 0.55, bone black 0.5, green earth 0.25, smalt 0.5; medium 0)
- `crownBand`: mask(881 sq units)
- `fdens`: function
- `footStip2`: mask(7903 sq units)
- `bandDens`: function
- `bandLow`: mask(854 sq units)
- `stripL`: mask(703 sq units)
- `stripR`: mask(857 sq units)
- `topC`: function
- `footAll`: mask(4888 sq units)
- `legDens`: function
- `legL`: mask(674 sq units)
- `legR`: mask(898 sq units)
- `fDens2`: function
- `footX`: mask(14977 sq units)
- `softF`: mask(23396 sq units)
- `stf2`: brush(rigger 2, 84% full)
- `GcrownV`: pile(lead white 5, pale smalt 1.4, red earth 0.28, smalt 0.4, yellow ochre 0.08; medium 0.85)
- `crownReg`: mask(2405 sq units)
- `topZone`: mask(2777 sq units)
- `tzLit`: mask(679 sq units)
- `tzShd`: mask(1100 sq units)
- `GunifyD`: pile(smalt 1.3, raw umber 0.75, red earth 0.3, Prussian blue 0.08, lead white 0.25; medium 0.85)
- `capBand`: mask(4223 sq units)
- `bNew3`: mask(14352 sq units)
- `nShade3`: mask(5166 sq units)
- `nWarm3`: mask(4435 sq units)
- `mFace`: mask(12853 sq units)
- `ribD`: function
- `shDens2`: function
- `wmDens2`: function
- `fDens3`: function
- `footW`: mask(7468 sq units)
- `ringL`: mask(729 sq units)
- `ringM`: mask(1498 sq units)
- `ringR`: mask(597 sq units)
- `ringT`: mask(892 sq units)
- `bNew4`: mask(18960 sq units)
- `footExt`: mask(5470 sq units)
- `hb4`: table with 6 entries
- `zShade4`: mask(6980 sq units)
- `zWarm4`: mask(6187 sq units)
- `softFoot`: mask(12396 sq units)
- `PfA`: pile(lead white 7, pale smalt 0.6, red earth 0.12, yellow ochre 0.12; medium 0)
- `PfB`: pile(lead white 6, pale smalt 0.55, red earth 0.22, yellow ochre 0.2, smalt 0.2; medium 0)
- `PfC`: pile(lead white 5.5, pale smalt 0.6, red earth 0.28, yellow ochre 0.25, raw umber 0.08; medium 0)
- `sdRect`: function
- `skDens`: function
- `skirtM`: mask(40300 sq units)
- `lmLit`: mask(2137 sq units)
- `lmShd`: mask(2613 sq units)
- `lowMesa`: mask(6784 sq units)
- `blendZoneM`: mask(11828 sq units)
- `dsD`: function
- `dsM`: mask(13587 sq units)
- `mCoolD`: function
- `mShdD`: function
- `mTop`: mask(11469 sq units)
- `mWrmD`: function
- `colL`: mask(4887 sq units)
- `dissD`: function
- `dissR`: mask(12639 sq units)

## The canvas clock

- Latest journal entry stamped: day 673, 09:47

## The canvas

The whole canvas as it was when this was written: ~/src/a/paint-studio-3a958c/out/easel/painting/50186a9d-0309-402c-832a-3593e851a34c.png