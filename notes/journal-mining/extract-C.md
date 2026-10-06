# Journal mining, batch C (engines 2-5, r21-r31 incl. r31 smoke)

Extracted from the 19 journals in `batch-C.txt`. Citations are `<studio>:<line>`, where studio is the
six-hex suffix of `paint-studio-<id>` and `r31smoke` is `yolo-r31-smoke-journal.md`. Engine per studio
(from the batch file): e2 = 106bf6, b08dc3, 3a958c, 35c488, f99282, 33a5bf, ba1453, 496bb9; e3 = ea8319,
2f7993, 8cc0b8, 24be5f; e5 = d95ea6, 12e572, 7c4dd1, 002de5, aef22d, 49b647, r31smoke. 2f7993, 8cc0b8,
002de5 and 106bf6 are plans only (1-5 lines); b08dc3 is two entries.

"Prompts" below means the r31 easel guide (`claude-paint-r31run/notes/easel_guide.md`), techniques
(`claude-paint-r31run/notes/techniques.md`), shipped studio notes (`gallery-fcf9c110/r31/studio_notes.md`,
cited "SN n") and the r31 brief draft.

## Summary

- The batch is dominated by **repair cycles**, not by first-pass painting. The long journals (3a958c
  719 days, 24be5f 772, 35c488 329, 496bb9 228 and unfinished, ba1453 three full rebuilds) spend most of
  their entries undoing self-inflicted damage. Three causes account for most of it: `blend` over masks that
  hold contrasting wet paint (or touch dry neighbors), background passes whose masks didn't exclude the
  fine things in front (twigs, figures, fruit), and local opaque patches on a thin or blended field that
  dried as visible boxes.
- What got painters out: repainting a **whole field** bounded by real edges with the original piles
  instead of patching; painting fine structure **last** and recording its paths; waiting until paint is
  **dry** (not tacky) before pale over dark; and, from engine 3 on, the **spirits rag** as an undo for
  wet paint over a dry layer. In engine 5, `thinner` gave the first local sky repair that worked
  (d95ea6:79).
- Two pieces of shipped advice disagree with what painters repeatedly observed: `hand="scumble"` laid
  opaque dabs/blobs, not a broken veil (6 studios), and glazes at the advised medium 0.7+ came out as
  saturated bars unless the pigment share and load were also low (5 studios).
- Mixture value was misjudged constantly, in both directions (white-heavy piles too pale, earth/black
  piles too dark, thin films lighter and warmer than the masstone). Painters who laid a small value
  ladder and read it in `mode: "value"` stopped guessing (35c488:91, ba1453:81).
- Late-engine journals (d95ea6, 12e572, 7c4dd1, aef22d, 49b647, r31smoke) share a sitting format: fresh
  look, crops, re-judge each open item as IMPROVED or ACCEPTED with a reason, handoff with live globals.
  It clearly limited churn; several sittings ended with no paint added.
- The r31 smoke painter used the scratch canvas well: trials over a **dried replica** of the real
  underlayer (60-day dried disc, reproduced glaze crackle), several candidates side by side, chosen one
  carried to the painting (r31smoke:53-57, 62-66).

---

## Ground and drawing

**A knifed ground of 140+70 µm levels the weave completely; there's no relief left to break strokes.**
- Kind: easel. Conditions: e2, knife ground, two layers 140 and 70 µm.
- Evidence: "the knife at 140+70um levelled the weave completely ... no weave relief to break strokes
  over" (ba1453:3). No later contradiction.
- Already in prompts? partly (guide, canvas: `"knife"` "levels the weave").
- Still valid? Probably; the guide still says the knife levels the weave.

**Pencil erasures leave ghosts that stay under thin paint.**
- Kind: easel. Conditions: e5, 2B pencil, eraser.
- Evidence: "The first square windows were erased and redrawn tall, so ghosts remain" (002de5:4).
- Already in prompts? yes (guide, Drawing: "leaves a ghost"; SN 52).

**Plan a test strip in a zone that will be buried later.**
- Kind: process. Conditions: e2, several studios.
- Evidence: swatch strips planned under the future dark foreground (106bf6:5, b08dc3:1, 3a958c:1);
  value ladder laid "in a strip of canvas I was going to repaint anyway" (35c488:106). But in two cases
  the test marks then had to be painted out and left ghosts (3a958c:7, 33a5bf:27).
- Already in prompts? superseded in r31 by the scratch canvas (guide, The scratch canvas).
- Still valid? Only for painters without scratch; with scratch, buried strips are unnecessary.

## Color mixing and value

**Mixture value can't be read from the recipe; lay a small value ladder and read it in `mode: "value"` before a pass.**
- Kind: easel/process. Conditions: e2-e5, all substrates.
- Evidence: white+umber/black "came out near-black every time ... The cure was to stop guessing. I laid
  a strip of four candidate mixtures ... read their value with `look mode=value`" (35c488:91);
  "My luminance estimates run ~0.15 too high" (ba1453:26), "Mix roughly twice as dark as you think. I
  calibrated with four test swatches ... should have done that first" (ba1453:81, again 222); "piles with
  lead white 2+ parts come out much paler than I expect - test the value small first" (33a5bf:10, 12, 24);
  "Take the value from what the pile does on the canvas, not from the recipe" (f99282:44); a
  "slightly lighter than the trunk" pile came out "three times too light" (f99282:52). Caveat: the ladder
  only counts if the trial pass is opaque; a body pass without fill reads the ground through the gaps
  (35c488:106).
- Already in prompts? partly (brief: inspect a small trial before repeating an unfamiliar mixture;
  techniques: "Watch the white"; SN 6). Neither names a value ladder or warns which way errors run.
- Still valid? Yes; the error is optical, observed in e2 and e5 alike.

**White-heavy piles converge to one pale chalky tone and hide hue differences; keep white low where hue matters.**
- Kind: easel/craft. Conditions: e2-e5, lead white ≥1.2 parts against ~2 of color.
- Evidence: four water piles "each heavy in lead white (1.2–1.8 parts) came out one uniform pale
  grey-blue — the piles' differences vanished" (12e572:8, 23); haze/tree-line piles with white 0.9 of ~4
  parts came out lighter than the hills (33a5bf:12, 24); sky piles "much paler than wanted" (r31smoke:2);
  "for darks near the horizon use NO or only a trace of lead white" (33a5bf:12).
- Already in prompts? partly (techniques: "Watch the white. Every pile with much lead white is chalky").
- Still valid? Yes.

**A dark that must cover needs a high-hiding pigment (bone black, raw umber); smalt, green earth and similar piles are stains.**
- Kind: easel. Conditions: e2, covering passes over a warm or pale ground.
- Evidence: "A pile of umber, smalt and green earth is a stain, not a covering ... took the bone-black
  one" (f99282:11, 16); "Pale smalt has a hiding of 0.35; a pile that is a third smalt is a third
  transparent ... The covers that worked were raw umber and bone black" (35c488:80); "keep smalt under a
  fifth of every pile" (35c488:86).
- Already in prompts? partly (guide tube table lists hiding; nothing says to read it when choosing a
  cover).
- Still valid? Yes (hiding values unchanged in the guide).

**A thin glaze over a warm ground shows ground-tinted color (olive, pink), not the pile's.**
- Kind: easel. Conditions: e2, warm toned ground, glaze or oil-thinned piles.
- Evidence: "A thin glaze over the warm ground gives olive or pink, not the colour of the pile"
  (35c488:5); "Piles thinned with oil came out warm, whatever the pigment, because the warm ground comes
  through" (f99282:19).
- Already in prompts? partly (SN 2, SN 6).

**Thin dark or earth lines over a pale field dry lighter, warmer and more saturated than the pile; mix them grayer/darker.**
- Kind: easel. Conditions: e5, rigger lines and stalks over light sky/snow.
- Evidence: fine branches "dried a warm mid-gray-brown ... thin rigger lines over a light sky come out
  paler than the pile (studio note 6)" (d95ea6:19); raw umber/ochre grass stalks "read as yellow-ochre and
  too saturated" (d95ea6:90, 92).
- Already in prompts? yes (SN 6, SN 7); the painter cited it but still had to learn it on the canvas.

**For a cool gray use white with pale smalt (and black for depth); umber warms it out of proportion to its volume.**
- Kind: craft/easel. Conditions: e2.
- Evidence: "Raw umber's tinting strength is 0.9 and pale smalt's is 0.35, so a mixture with the two in
  equal volume is warm three to one ... the whole lower canvas was pink for several days" (35c488:108);
  "white and pale smalt carry it and umber spoils it" (35c488:38).
- Already in prompts? no (tinting strengths are in the guide table only).
- Still valid? Yes for the default box; the 54-tube box has other cool options.

**Transparent blues over a yellow underlayer turn green; shadows over a warm base need opaque darks.**
- Kind: craft. Conditions: e2, still life.
- Evidence: ba1453:24-25.
- Already in prompts? no.

**`look hold` and the palette swatch show body color; laid thin in a field, or over a dark passage, the same paint reads differently. Mix a shade darker than looks right on the knife.**
- Kind: easel. Conditions: e2 palette swatch; e5 `hold`.
- Evidence: `work` "lays paint far heavier/darker than the palette's thin-coat swatch" (496bb9:4, e2);
  "`look hold` shows a pile's body color. That's lighter than the same paint thinned in the blended field"
  (d95ea6:53); "hold-check doesn't predict lightness reliably over a dark passage ... Mix darker than you
  think for any accent over dark glass" (7c4dd1:26, 31).
- Already in prompts? partly (guide: hold "shows the paint on the knife, not how it would look laid";
  brief: "use the laid trial").

## Thin vs thick paint, covering

**`coverage` is not film thickness: one pass at high coverage lays about the same film as a modest one; repeat separate passes (with pressure and fill) to build a covering body.**
- Kind: easel. Conditions: e2, `work`, broad/glaze/body hands.
- Evidence: "`work` at coverage 10 lays about the same thin film as coverage 3 ... three separate calls of
  coverage 3 reach a true dark" (35c488:67); later: light-pressure broad/glaze passes hide nothing however
  repeated; "What actually covers is pressed paint — `pressure={0.75,1.0}` — and `fill=true`" (35c488:110,
  130); "Bone black at coverage 4 with fill finally held" (f99282:37).
- Already in prompts? no.
- Still valid? Unknown for e5; the guide defines coverage as "layers of strokes over each point" and
  doesn't say whether it adds thickness.

**A turps lay-in at coverage ~1.2 without fill leaves gaps; a second pass at ~2 with `fill=true` covers.**
- Kind: easel. Conditions: e5 (and e3), turps 0.5-0.65, broad.
- Evidence: 7c4dd1:5, aef22d:2, 49b647:4, 12e572:2 all report the same first-pass gaps and the same fix.
  Thin turps lay-in dry after ~33 h (aef22d:2); red earth at turps 0.5, coverage 2.2 took 4-6 days
  (7c4dd1:5).
- Already in prompts? partly (SN 4, SN 20; techniques "Lay-in: lean, quick, thin" doesn't mention
  gaps).
- Still valid? Yes, e5 observations.

**`fill=true` closes gaps but over a large field lays a stamped pattern of identical dabs; for a large even field use a very broad flat brush with long strokes.**
- Kind: easel. Conditions: e2, body/glaze hands vs broad.
- Evidence: "`fill=true` leaves cottage cheese" (35c488:39); "a wallpaper of lumps" (35c488:112, 132);
  versus "`fill=true` is what completes a passage" (f99282:17). What worked for flat fields: filbert 34-44,
  `hand="broad"`, strokes 90-200, coverage 3-5 at two crossing angles, `curve={0,0}`, `edge="found"`,
  `clip=true`, wait 150 min between (ba1453:116, 186, 215). In e5, `fill=true, clip=true, coverage 2.5-3`
  gave clean flat fields for a hard-edged manner (49b647:18).
- Already in prompts? partly (guide describes `fill`; no field recipe).
- Still valid? fill's dab pattern is e2 evidence; e5 painters used fill without complaint.

**`hand="body"` with a small filbert lays separate worm-like dashes; stroke length must be much larger than brush width for a passage to read as paint.**
- Kind: easel. Conditions: e2 and e5, body hand filbert 6-10.
- Evidence: "discrete worm-like dashes" (ba1453:18, 94); body strokes on a dry lay-in "read as separate
  worm-strokes" (aef22d:6, 11); "body hand leaves dabs ('cottage cheese')" (35c488:6); low coverage
  (<0.8) "lays WORMS, whatever the hand" (ba1453:223).
- Already in prompts? partly (SN 20, SN 22).

**Oil-rich films over a large area caused trouble in engine 2: a reptile crust at medium 0.35, pinholes at 0.3-0.75.**
- Kind: easel. Conditions: e2 only.
- Evidence: "My unifying glaze at medium 0.35 over everywhere() laid a reptile-skin crust" (ba1453:57);
  "At medium 0.3 to 0.75 the picture dried for five days and opened up into hundreds of pinholes with the
  bare linen showing through, worst in vertical bands along the warp" (35c488:110, 131).
- Already in prompts? no.
- Still valid? Doubtful; no e3-e5 journal reports either. Flag as engine-2 behavior.

**An oily, fluid coat of the same color levels dry impasto without changing its color; a knife over it only rides the ridges.**
- Kind: easel. Conditions: e5, dry impasto sun disc, scratch trials dried 60 days.
- Evidence: knife "only caught the ridge tops"; medium 0.65 coat "levelled better. A second coat after
  40 days was smoother again. The colour in normal light was unchanged" (r31smoke:53-58).
- Already in prompts? partly (guide/techniques: knife over dry impasto catches only ridges). The fluid
  coat is not.

## Blending and edges

**`blend` only inside one wet tone: never over a mask (especially a rectangle) that holds contrasting wet paint, and never where the mask touches a dark edge or a dry passage you want to keep.**
- Kind: easel/failure-pattern. Conditions: e2-e5, badger blender, rect or blurred masks.
- Evidence: the single most repeated failure in the batch. "blend(rect) over a region that included the
  wet rock base dragged dark rock paint sideways -> hard-edged grey rectangle" (3a958c:9; again 19, 25);
  "the blender over a mask drags any wet dark ... and leaves the mask's rectangle as a cut edge"
  (ea8319:2); "never blend a rectangle over wet paint of strong contrast ... gray mud" (24be5f:17), then
  "I blended a rectangle again, on reflex" (24be5f:20); "The badger picked the black off the near birches
  and dragged it across the sky in comet streaks" (35c488:37, 83); "keep blend masks >=25 units clear of
  every object" (ba1453:84, 133); a blend over `everywhere()` "dragged wet blue across the DRY sky band and
  DRY pads" (12e572:3); blend with `clip=false` on a wide mask dragged meadow paint across the sky and cost
  the left half of the picture (496bb9:20); 33a5bf:10, 14; aef22d:18.
- Already in prompts? partly (SN 39: "A local blend can leave a seam ... A soft mask does not prevent wet
  dark and light passages inside it from mixing"; guide: blend lifts ~15% per pass).
- Still valid? Yes; reported in e5 (12e572, aef22d, r31smoke) as well as e2.

**A blend region's boundary shows as a seam unless the paint on both sides of it already matches; a soft mask doesn't fix that.**
- Kind: easel. Conditions: e2-e5.
- Evidence: "blend over a soft mask still leaves a hard seam where the moved paint stops — blend region
  boundaries must lie where neighbouring paint is the same value" (aef22d:26, 38, 48); a blurred rect still
  dragged grey into fog (3a958c:25); counter-evidence that soft masks helped: "blend masks must be soft
  (soften 30-40) where they end inside a wet field" (33a5bf:14), "wet-in-wet blends on a mask with soft
  (blurred) boundaries leave no seams" (3a958c:24). Interpretation: soft edges help only when the wet paint
  inside and outside is close in value.
- Already in prompts? partly (SN 39).

**Blend is strong: one pass is plenty, and it flattens fresh sparse marks into a slab.**
- Kind: easel. Conditions: e5 smoke and e2.
- Evidence: "the blend hand over two wet layers lifts and averages strongly - one pass is already a lot"
  (r31smoke:3); "blend at most one pass, and NEVER over a fresh, sparse body hatch in a narrow band"
  (r31smoke:69); "blend over fresh dabs always flattens to a smooth smear" (aef22d:9); "Blend over a fresh
  opaque white band = flat cream" (12e572:4); "blender over loaded clumps kills the dark" (33a5bf:5).
  Over-blending also left whole pictures "over-smoothed" (ba1453:151, 168, 228).
- Already in prompts? partly (guide: blend lifts ~15% per pass, ~40% in three).

**Broken dabs plus one directional blend while wet turn into a shimmer (water, reflections).**
- Kind: easel. Conditions: e2-e5, dabs or short strokes, wet, blend along the stroke direction.
- Evidence: blotted pebble dabs then "One horizontal blend pass while open -> soft shimmering broken
  water, much better" (aef22d:14); "two horizontal blends over the band while wet fused them into a soft
  luminous shimmer" (12e572:10, 19); reflection recipe "small marks first, then merge them wet"
  (f99282:68). Vertical streaks: "filbert 7 load .45 p .55->.2 strokes 70-170 long THEN vertical badger-20
  blend over soft rect" (aef22d:21), confirmed for light paint (aef22d:43) and a third time on a
  dark-over-light edge (aef22d:62).
- Already in prompts? no.
- Still valid? Yes (e5).

**Soft paint edges come from the paint, not from the mask: `clip=true` on a soft mask gives a crisp edge at ~0.5, and `work` anchors only above 0.3.**
- Kind: easel. Conditions: e2-e5.
- Evidence: "clip=true makes a CRISP boundary where the soft mask crosses ~0.5; soft masks do not give
  soft paint edges" (3a958c:17; contradicts the early b08dc3:2 claim); "soft edges must be made in the
  PAINT (colour that merges into the neighbour), not in the mask" (3a958c:20); "Blurred masks under work()
  produce a visible cloud at the 0.3 contour" (ba1453:61, 181); "a soft mask only softens the edge, it does
  not thin the paint" (f99282:36). What worked: a mask ramp that stays above 0.3 (1.0 down to 0.4) "thins
  the paint smoothly ... That single change is what turned the sky ... into an evening" (35c488:114, 133);
  dry-on-dry stipple with a coverage ramp + `feather` (3a958c:24); hills in fog painted in piles that are
  fractions of the fog color, then blended over the same mask (3a958c:20).
- Already in prompts? partly (SN 16, SN 19, SN 45).

**`edge="soft"`/`"lost"` can carry paint far past the mask, even with `clip=true`; near anything to keep, use `edge="found"` and a separate softening method.**
- Kind: easel. Conditions: e2 (all reports).
- Evidence: "`edge="soft"` on `work` is a loaded brush. It carried pale paint ... clear across the
  canvas" (35c488:68, 83); "edge="lost"/"soft" with clip=true defeats the clipping" (ba1453:54, 88, 201,
  220); "covering them with a body pass using edge="lost" ran strokes 40+ units past the zone — over the
  figure" (33a5bf:21); "edge='lost' + clip=rect made strokes spill to the rect boundary" (3a958c:18).
- Already in prompts? partly (guide: edge "with an overrun that varies and can reach beyond it"; nothing on
  its interaction with `clip`).
- Still valid? Unknown; no e3-e5 painter reports it, and the guide doesn't say which wins.

**`load_at` falling to 0 doesn't fade paint to nothing: a nearly empty brush still lays a film and the mask edge shows.**
- Kind: easel. Conditions: e2.
- Evidence: 3a958c:13; ba1453:173 ("with long strokes it lays STREAKS ... Low load values give dry-brush
  with hard edges"); but ba1453:120 found `load_at` gave "a real gradient" once, and r31smoke:7 used a load
  ramp 0.25-0.7 for a glaze with success.
- Already in prompts? no.

**`lose()` with a contrasting pile makes a zipper or spikes, not a lost edge.**
- Kind: easel. Conditions: e2-e5.
- Evidence: "a row of regular dark comma strokes on both sides — a 'zipper' edge" (12e572:6, 21); "lose()
  on a dark shape puts black spikes round it" (ba1453:224); "sawtooth spiky pink strokes" (aef22d:5); sky
  paint into a wet dark canopy "stark white scratches" (496bb9:11). Partial success: thinned dark paint
  dragged across scalloped crowns read "as twigs and air" (24be5f:33).
- Already in prompts? no.

**To dissolve a seam or break a hard-edged mass, hatch over DRY paint with interlocking noise-edged fingers of the two neighboring recipes, instead of blending.**
- Kind: easel/process. Conditions: e5, dry substrate, body hand filbert 3, coverage 0.7-1.6.
- Evidence: cast-shadow bays and tongues (r31smoke:12, 14, 17: "The blend averages, while hatching over
  dry paint keeps the marks"); haze step at x≈870 after a blend and a glaze both failed (r31smoke:73, 76).
  Related: e2 "dry-on-dry stipple with a coverage ramp + feather dissolves edges ... far better than any
  mask-edge trick" (3a958c:24).
- Already in prompts? no.

**Evenly spaced parallel marks along a line read as a fence; strokes that follow the form's own direction and overlap the line read better.**
- Kind: craft. Conditions: e5 smoke, eave fringe; e2 lift strokes.
- Evidence: three scratch trials, the third worked (r31smoke:29, 31); lifting in vertical stripes left "a
  ribbed 'fence'" (33a5bf:12, 15).
- Already in prompts? partly (SN 22).

## Glazing and scumbling

**`hand="scumble"` (and blotted light scumbles) lays opaque dabs or blobs, not a broken veil; a lead-white mix is opaque at any load.**
- Kind: easel/failure-pattern. Conditions: e2-e5, scumble hand, load 0.25-0.45, over dry or wet.
- Evidence: "scumble-hand makes blobs" (ea8319:2); "hand="scumble" makes round dark dabs" (ba1453:35); "a
  scumble of lead-white mix is OPAQUE here at any load" (33a5bf:15, 26); "the scumble went down thick and
  opaque in fat blotches" (24be5f:12, 21); blotted brick scumble over dry brick "too light, hard-edged"
  (7c4dd1:8); "separate opaque light blobs, not a veil" twice (49b647:11, 19); path scumble an "opaque light
  rope" (496bb9:12); gap scumble "crisp opaque white rectangles" (aef22d:12).
- Already in prompts? Contradicted: techniques: "Scumble: an opaque light paint dragged thin
  (`hand="scumble"`, little load) over a dry darker one, broken by the surface". SN 8 says a thin
  lead-white veil "veils and lightens", which painters never managed with the scumble hand.
- Still valid? Yes, reported in e5 (7c4dd1, aef22d, 49b647). What worked instead: thinner veils (next
  entry), or a whole-field repaint with a two-pile gradient (49b647:29, 44).

**For a veil that integrates, use `thinner` (≈0.55) in several light passes; it changes little per pass but never pastes.**
- Kind: easel. Conditions: e5, filbert 14, load 0.5, coverage 1, 2-3 passes, over dry.
- Evidence: "Thin veils WORK: thinner 0.55 piles ... translucent veils that unify the willow foot without
  hard edges" (aef22d:30, 37, 51: "integrate but change little"); a thin film over dry blotted dabs "breaks on
  their ridges and leaves a fine pale net" (aef22d:31); local sky repair (next section, d95ea6:79).
- Already in prompts? partly (guide documents thinner; nothing presents it as the veil/repair tool).
- Still valid? Yes, e5 feature.

**A glaze needs little pigment as well as much medium: at medium ~0.7 with an ordinary pigment load it comes out as saturated bars or boxes.**
- Kind: easel. Conditions: e2-e5, glaze hand over dry paint.
- Evidence: cadmium/madder at medium .7 "stayed fully saturated lemon and magenta bars" (12e572:5); the
  fix: "medium ≥.7 and very little pigment (≤.6 parts total) at load ≤.35" (12e572:33, 35); warm glaze at
  medium .7 "garish orange patch with hard rectangular edges" (7c4dd1:9); ultramarine at medium .7-.9 with
  `clip=true` "saturated dark-ultramarine rectangle" (aef22d:19); shadow glaze at .7 "too saturated, patchy
  purple" (r31smoke:6); "cartoon gold" (3a958c:8); "glaze hand still lays a lot - use load 0.3-0.5 for a
  true glaze" (496bb9:7).
- Already in prompts? partly; techniques gives "plenty of medium (0.7 to 0.85)" and "a little of
  anything" but no load or pigment-share guidance.
- Still valid? Yes (e5).

**A glaze or wash over a large dry area at low load gives isolated strokes or stripes, not a veil; glazes in narrow bands pool.**
- Kind: easel. Conditions: e2-e5, glaze hand over dry.
- Evidence: "A glaze `work` over a large dry area at low load gives isolated strokes, not a veil. Test it
  on a small corner first" (d95ea6:36, 40); "a glaze brushed over the whole grove left horizontal stripes"
  (24be5f:33); "A 'wash' of a transparent-ish pile over a dry passage dry-brushes and shows the underlayer"
  (ba1453:191); "the loaded start of every stroke behind as an opaque lozenge" (f99282:20); "glazes in
  narrow bands pool into blots" (r31smoke:67, 69); glaze+blend over dry relief leaves crackle in the
  furrows (r31smoke:61, 69).
- Already in prompts? partly (SN 24 loaded starts; SN 43 glaze unclipped).

**A whole-canvas "unifying" or "harmonizing" glaze is not free; it has wrecked finished passages repeatedly.**
- Kind: failure-pattern. Conditions: e2-e5.
- Evidence: "A unifying glaze at medium 0.28 over the whole canvas ... put a mass of pale curls over the
  entire painting. I lost the objects ... if the surfaces are already even, do not touch them"
  (ba1453:140); "One of those cost me the whole picture" (f99282:20); a dark deepening glaze over the whole
  sky "went almost black" (35c488:89); a whole-sky glaze laid while repairs were wet dragged them
  (3a958c:15); "A glaze is not a repair" (f99282:54).
- Already in prompts? no (techniques recommends glazes to unify; the brief doesn't).

**A translucent glaze only grays an old mark; to bury it, cover with opaque paint.**
- Kind: craft/easel. Conditions: e2.
- Evidence: "A translucent glaze does not bury a mark; it only greys it" (35c488:36, 57); "opaque repaint
  can't match a glazed passage - glaze over the repaint instead" (496bb9:15), which is the reverse case.
- Already in prompts? no.

**Over dry opaque paint, an overlay meant to show needs strong pigment and load ≥0.65; faint loads barely register.**
- Kind: easel. Conditions: e5, cloud reflections over dry cream.
- Evidence: load .2-.5 "barely shows" (12e572:16); the retry with more pigment and load .65-.85 "This time
  it shows" (12e572:26, 31).
- Already in prompts? no.

## Drying and timing

**Pale over dark only when the dark is dry, not tacky: over tacky paint the pale breaks into patches; over open paint it mixes in.**
- Kind: easel. Conditions: e2-e5.
- Evidence: blinds over tacky dark glass: "broken patchy coverage, dark showing through in blotches
  (stick-slip). Wait until drying()=="dry"" (49b647:5, 17, 46); "wet dark under a fresh pale pass drags up
  and greys it - let dark layers dry" (3a958c:2, 4); "The canvas was open for 3 days and every new stroke
  just MIXED into the wet paint" (ba1453:95); "Paint onto SET paint or not at all ... One pass, then wait
  18-22 h" (ba1453:189); "Ten days between sessions, and most of the failures simply went away" (35c488:81);
  a light glaze over open dark "lifts it and leaves a ghost" (f99282:9, 21); "do darks in one session, wait
  till dry, then lights over" (7c4dd1:5).
- Already in prompts? partly (SN 26, SN 31; techniques: glazes/scumbles want dry).

**Real drying in these paintings ran much longer than painters expected; plan waits of days to weeks.**
- Kind: easel. Conditions: various.
- Evidence: bone black limbs "tacky for 5+ days" (3a958c:6), wait ">= 2 weeks for dark/black paint" before
  pale near it (3a958c:26, e2); oil-rich lead-white glazes "tacky for five weeks" (f99282:22, e2); thin body
  coats medium 0.03-0.05 over earlier layers "10-17 days" (7c4dd1:6, 14, e5); earth/black details at medium
  .08 10-20 days, lead-white fields 3-8 days (49b647:21, 49, e5); "Thick, light, lead-white patches took ~20
  days to be touch-dry" (d95ea6:30, e5); small marks dry in 12-14 days (7c4dd1:26).
- Already in prompts? partly (guide Time section gives rates). Note: the e5 lead-white observations
  (3-8 and ~20 days) are longer than the guide's "touch-dry in under two days" for a loaded broad stroke;
  thickness, layering and medium may explain it, but I can't confirm from the guide.

**`drying(x, y)` at one point says nothing about the whole path; check several points before a wipe or a pale pass.**
- Kind: easel/process. Conditions: e5.
- Evidence: "A rag wipe on a 'dry' area still lifted a pale track down to near ground: drying() at a single
  point is not the whole path" (d95ea6:11, 29); "Check drying() at several points before each campaign"
  (7c4dd1:14, 31).
- Already in prompts? partly (SN 50 says passages set at different rates; no advice to sample several
  points).

**Every glaze or coat is open long enough to drag neighbors laid within hours; a fresh repair near a passage is a hazard until it dries.**
- Kind: failure-pattern. Conditions: e2.
- Evidence: a deepening glaze laid "while the L4 limb repair (4 h old), the new cliff (1 h) ... were still
  wet: the glaze strokes dragged them across the sky" (3a958c:15); a ghost rectangle from coating a stipple
  "2.5 h later while still open" (3a958c:21).
- Already in prompts? partly (SN 29).

## Corrections and lifting

**Repaint a whole field, bounded by real edges, with the original piles and handling; local patches on a thin or blended field dry as visible boxes.**
- Kind: process/failure-pattern. Conditions: e2-e5, skies, meadows, flat facades, snow.
- Evidence: "a sky laid thin and blended into the warm ground can't be patched with opaque paint ... thick
  paint reads paler/whiter than the thin film" (d95ea6:7); "full-band repaints with the original piles
  match; local patches don't" (d95ea6:9, 12, 24), "Two coats about 10 days apart hide dark ghosts"
  (d95ea6:24); "To correct a flat field, repaint the WHOLE field bounded by real edges, with a gradient of
  piles ... It left no seams" (49b647:20, 29, 44); "Only whole-area wet sessions have worked" (33a5bf:22,
  26); "A fix that is a passage is cheaper than a fix that is a patch" (35c488:73, 82: "A narrow repaint is
  worse than a wide one"); "The patches are piling up as visible boxes ... Decision: stop patching"
  (24be5f:19); "opaque body paint over the textured meadow always reads as a patch" (24be5f:35); a sky
  rectangle "pasted panel" fixed by repainting the whole sky (33a5bf:9; 49b647:30). Cost: whole-field
  repaints also bury everything in the field, which drove the repeated losses in 496bb9:21, 24.
- Already in prompts? partly (SN 48 cut edge; brief: "If the repair recreates the problem, reconsider the
  composition or method"). No line says "whole field, real edges, original piles".
- Still valid? Yes (e5 evidence in d95ea6, 49b647).

**To match an existing passage, re-knife the recipe it was actually painted with (read it from the log); a lookalike pile shows as a step.**
- Kind: process/easel. Conditions: e5.
- Evidence: lookalike piles "dried to a pale pink stripe ... Third trial, re-knifed from the original
  recipes in the log ... matched the value" (r31smoke:30-31, confirmed 45, 64); "Small restatements over DRY
  paint with the original piles match in one coat if the edge is real ... An approximate mix shows as a
  slight value step" (49b647:57); "same handling and the same piles laid as a full layer" (d95ea6:9).
- Already in prompts? partly (guide scratch: "the same pile, the same handling"; nothing about the log as
  a recipe source).

**A spirits rag lifts wet paint off a dry layer almost completely; it is the practical undo (engine 3+).**
- Kind: easel. Conditions: e3-e5, rag dip 0.8-1.0, pressure 0.8-0.95, 2-3 passes (or 10-25 for large
  accidents), paint over a set/dry film.
- Evidence: "spirit rag cleans wet paint off dry sky perfectly — good eraser" (ea8319:2); "Wet paint over a
  dry film always comes off with fresh spirit rags (dip 1.0, pressure 1)" (d95ea6:39); "a reliable undo for
  wet-over-dry" (12e572:22, 31); "It was my eraser three times" (7c4dd1:14); "the reliable undo"
  (aef22d:4, 36); "lifting with a spirits rag before the paint sets is the real undo" (r31smoke:10, 69). It
  works poorly on thick, medium-free layers (12e572:8, 22) and on tacky paint, where it "only scars it"
  (24be5f:11, 26). Side effects: pale halo or streak where each wipe ends (aef22d:15, 36; r31smoke:40);
  refolding one rag drags lifted dark across light, so take a fresh rag per pass (d95ea6:14, 26); next to
  crisp edges it smears residue onto them (49b647:10, 19).
- Already in prompts? partly (guide, The rag: lifts open paint, "Paint laid over a set film lifts down to
  that film", smears along edges). Techniques still says "No undo: a mistake is painted over, glazed,
  scraped or let dry and restated" with no mention of the rag.
- Still valid? Yes for e3-e5. Not available in e2 journals.

**Near a finished dry silhouette, wipe inside the silhouette mask right after hatching near it: the rag lifts only the new wet overshoot.**
- Kind: easel. Conditions: e5, hatch strokes overshoot ~half a stroke length.
- Evidence: r31smoke:43, 45.
- Already in prompts? no.

**Thinned paint (`thinner` 0.35-0.6) tints a dry pale patch in a thin sky without laying a new opaque body; the first local sky repair that worked.**
- Kind: easel. Conditions: e5, several light passes, clipped to a slightly grown soft mask.
- Evidence: d95ea6:74, 79 ("It's the first local sky repair that worked at this easel"); `thinner=` goes
  inside the pile braces (d95ea6:80).
- Already in prompts? no (guide explains thinner mechanically).

**A failed modulation was almost always too far in value or saturation from what lay under it; keep corrections and accents within one value step.**
- Kind: craft/process. Conditions: e5.
- Evidence: "Each one went wrong because its value or saturation differed too much ... keep it within a step
  of the surrounding value" (7c4dd1:15); the retry "worked ... the value stayed within a step of the glass,
  and the shapes are rectangular and architectural" (7c4dd1:20, 31); four failed trials over dry willow:
  "Diagnosis: value jump too big" (aef22d:13); "any local modulation must be very close in value AND have no
  mask boundary" (49b647:10); warm touches "close in value to the pads = integrate" (aef22d:54).
- Already in prompts? no.

**When a passage must be flattened, clip to the mask grown 5-7 units; clipping to the mask itself leaves the old paint as a fringe at the soft rim.**
- Kind: easel. Conditions: e2.
- Evidence: ba1453:144, 196.
- Already in prompts? no.

**A carve or patch mask must be (old paint ∩ outside the new outline), never a bounding rectangle.**
- Kind: process. Conditions: e5.
- Evidence: "I forgot that the rect also held open sky with fine twigs, so it laid a pale opaque rectangle
  over twigs and the sky gradient" (d95ea6:14, 25).
- Already in prompts? no.

**Restating a line by eye beside a dry line misses by a couple of units and doubles it; restate only from a recorded path.**
- Kind: easel/process. Conditions: e5.
- Evidence: d95ea6:35, 41; then the recorded paths were found in globals and restated exactly with
  `pressure_for(w×k)`, giving the crown "a dark armature" (d95ea6:59-62, 69-70). "Before declaring a path
  'unrecorded,' dump every table global" (d95ea6:69).
- Already in prompts? no.

**Don't alternate repairs on one passage: each cover creates the next fault.**
- Kind: failure-pattern. Conditions: e2.
- Evidence: "I put a veil on the two quinces ... got worms, covered the worms with gold, got flat light
  discs, covered those with mottle, got worms again. Each repair cost more than the original pass would have"
  (ba1453:226); "every further stroke I have put into those three things this month has made them worse"
  (f99282:56, 74).
- Already in prompts? partly (brief: "If the repair recreates the problem, reconsider").

## Detail and texture

**Paint fine structure (twigs, figures, reflections) last, once, after every background coat is final; and exclude it from every later background mask.**
- Kind: process. Conditions: e2-e5.
- Evidence: 3a958c's eleven numbered mistakes were mostly sky or fog passes burying the oak's twigs
  ("same mistake a THIRD time", 3a958c:8; "Tree always last", 9-10; "RULE: build every background mask as
  `... - treeAllG - figM:grow(2)`", 14); it began recording all 1360 twig polylines so later passes could
  exclude them (3a958c:15, 17, 22). "Every glaze over the midground buried the grove again ... paint the
  mist first and the wood last, once" (35c488:8, 12, 139); "every object's mask must exclude the things in
  front of it" (ba1453:32, 137); "the figure was buried twice by passes meant for the water" (35c488:10);
  meadow repaint covered the trunks (496bb9:9); forgot to subtract the pad zone (12e572:8). Pitfall: `grow()`
  after subtracting objects undoes the subtraction (ba1453:221).
- Already in prompts? partly (techniques: "Clip a pass near anything it must not touch"; guide Depth
  layers).

**Twigs must grow from drawn branches; branches need straight runs with angular kinks, not smooth alternating bends or straight fans.**
- Kind: craft. Conditions: e2, e5.
- Evidence: "Random seeds in an annulus read as fluff" (d95ea6:12, 27); "Hand-listed branch points with
  alternating gentle bends read as snakes. Oak needs straight runs and kinks" (d95ea6:10, 28); trees as
  "fans of straight sticks ... Fixed by giving every limb a start angle AND a tip angle, and cutting children
  to two per node" (f99282:8).
- Already in prompts? no.

**A stroke with a pressure ramp to zero ends in a point and reads as an object (spindle, almond, candle); give trunks and reflections blunt ends.**
- Kind: easel/craft. Conditions: e2.
- Evidence: "Every trunk I drew with a pressure ramp came out a spindle ... every reflection I drew short
  came out a leaf, an almond, a white flame" (35c488:35, 47); fronds with pressure tapering to 0.05 "came out
  as wavy drip-lines" (12e572:2, keep pressure ≥0.3).
- Already in prompts? partly (guide, Brushes: a pointed brush "ends in a point"; SN 10).

**`shake` on a narrow brush sends the tip wandering far; leave it off fine brushes.**
- Kind: easel. Conditions: e2, rigger 3.
- Evidence: "sends the tip of a 3-unit rigger wandering two hundred units ... No `shake` on anything finer
  than a filbert 12" (35c488:34). Related: gesture `wobble=6` gave drip lines; ≤2 worked (12e572:2, 36).
- Already in prompts? no.
- Still valid? Unknown for e5; the guide doesn't say how shake scales with width.

**Lozenges come from stroke size relative to shape size: a stroke as long as the shape turns its outline into the mark. Use small strokes in small shapes.**
- Kind: easel. Conditions: e2, body/scumble hand over 90×12 ellipses.
- Evidence: "Lozenges are not a mistake of colour or coverage, they are a mistake of stroke size against shape
  size ... Small strokes (hand="detail", 4-14 units) over the same masks gave a fine broken tone" (f99282:60);
  glaze hand "on an object 40 units across it combs the surface" (ba1453:91).
- Already in prompts? no.

**Light or shadow on a small form is a broad gradient over the whole form; a narrow highlight band lays a string of beads.**
- Kind: easel/craft. Conditions: e2.
- Evidence: "A narrow ribbon of highlight (width 5-12) plus short strokes is a STRING OF BEADS, every time"
  (ba1453:123); "A clipped `work` over a band about two units wide comes out as a string of beads. Light a
  cylinder by over-stroking the whole form to one side" (35c488:118, 135); trunk lit flanks "a pale zipper ...
  a chain of beads ... strips of tape" (f99282:47, 52).
- Already in prompts? partly (SN 21, SN 37).

**Small modeled objects: lay the full sequence (dark mass, mid, light on an offset shape, dark crescent, blend inside the mask, re-break with mottle) in one run.**
- Kind: easel/process. Conditions: e2, still-life fruit.
- Evidence: "That whole sequence, run without interruption, gives quinces. Running HALF of it gives potatoes"
  (ba1453:217, 127, 225); "work() with `load_at` ... and then `blend(m)` inside the object mask" was the only
  smooth modeling (ba1453:182).
- Already in prompts? no.

**Hand-placed gestures from small helper functions beat `work()` over masks for marks that carry a passage (reflections, pads, flowers).**
- Kind: easel/process. Conditions: e5, e2.
- Evidence: "What worked: hand-placed gestures from small helpers ... each with its own random length, load
  and pressure, rather than work() over masks" (12e572:18, 31, 35); after lobed masks and fill dabs failed,
  "direct b:stroke calls - long horizontal strokes with a big filbert ... in bands" gave a continuous field
  (496bb9:23-24); far shore broken by hand-plotted rigger trees (f99282:59).
- Already in prompts? partly (guide: "Use gestures for the marks that carry the picture").

**Isolated sparse marks on a smooth dry field read as graphic; texture must be dense, veiled or close in value.**
- Kind: easel (hypothesis). Conditions: e5.
- Evidence: aef22d:6, 22 ("small-scale additions keep failing because they're isolated marks on smooth dry
  fields"), 41; dark flecks over a dry light field "look stamped on. A light spirits-rag wipe while they're
  wet breaks them into the texture" (r31smoke:23, 25); light incidents in a dark bank read as "sparks, eyes,
  chalk scratches" (f99282:53); "bright marks on dark water read as sparks" (f99282:25).
- Already in prompts? partly (techniques: glitter "big uniform dabs read as beads").

**Pads (and similar floating shapes) read when loose: straight short flat-brush strokes with gaps, unblended, over dry; curved strokes make coins and blending inside the outline makes a disc.**
- Kind: easel/craft. Conditions: e5.
- Evidence: aef22d:28-29 ("Key change vs failures: straight flat-brush marks (no curve), over dry, no
  blending"), 39; "Pads read as pads when loose ... A solid base with stacked rows looks like a lozenge"
  (12e572:13, 20); "Never blend pads inside their own outline" (12e572:30); `curve` with long strokes "made
  arcs like eyebrows" (12e572:4).
- Already in prompts? no.

**Reflections are carried by tone, laid after the object, in a value between water and object; marks of light on water read as sparks.**
- Kind: craft. Conditions: e2.
- Evidence: "Reflections on water are carried by tone, not by marks ... darken the lake, then lift a soft
  wedge under the mountain and blend it" (f99282:35, 25); reflections of nearly pure white "read as white
  candles ... never lighter than the water it lies in" (35c488:21, 11, 35).
- Already in prompts? no (techniques covers glitter only).

## Composition and looking

**Decide each passage's value once, in a list, and keep it; most rebuilds came from a passage changing value.**
- Kind: process. Conditions: e2.
- Evidence: "decide the value of each passage once ... and stick to it" (35c488:14), "learn a third time"
  (35c488:41, 52, 86, 139); after rebuilding with a stacked range "from about fifteen at the near trunks to
  about ninety in the vapour, with the dark band of the far wood in between" the picture held (35c488:125).
- Already in prompts? partly (techniques: "Plan in values", "The full value range").

**Squint and value checks caught the biggest problems: everything in the middle values, no dark middle band.**
- Kind: process. Conditions: e2.
- Evidence: "far too close in value - tree mid-gray, land mid, sky only slightly lighter" (496bb9:6); "The
  whole picture sits between about 70 and 95 in value ... I have one light and it is everywhere, so it is
  nowhere" (35c488:46, 120).
- Already in prompts? yes (techniques; brief).

**Look at 1:1 before declaring a finish; whole views hid faults that reversed the verdict.**
- Kind: process. Conditions: e2.
- Evidence: "I called it finished on day 14. That was a false finish" (35c488:12, 19); "looking at it
  properly at 1:1 rather than at thumbnail size. I have to reverse the last entry" (35c488:42).
- Already in prompts? yes (brief: detail crops when finishing; guide: survey after each campaign).

**Measure against a grid before deciding a passage is wrong.**
- Kind: process. Conditions: e2.
- Evidence: a pale streak taken for scum "is the lit crest of the near bank ... I was wrong twice ... before I
  measured it against a grid" (f99282:72).
- Already in prompts? partly (techniques: grid for coordinates).

**Check gallery/relief light before adding: old slabs and ridges show there even when invisible in color.**
- Kind: easel/process. Conditions: e5.
- Evidence: d95ea6:50, 54; r31smoke:18, 52 (sun disc impasto as "a crusty disc ... like a biscuit").
- Already in prompts? partly (techniques Impasto: check relief and gallery).

**Taking something out can improve the picture more than fixing it.**
- Kind: craft/process. Conditions: e2, e5.
- Evidence: the cloth removed: "The picture is stronger without it" (ba1453:68, 113); a grass tuft lifted:
  "the tuft brought a near foreground and a second subject into a picture whose whole strength is its
  emptiness ... ask first whether the picture wants anything there at all" (d95ea6:90, 92).
- Already in prompts? no.

**Moments where a change of approach clearly improved the picture** (my selection):
- Whole meadow repainted wet in one sitting with the grove's long shadow: "Big change for the better ... now
  holds the picture together" (33a5bf:20).
- Whole sky repainted wet, then filbert-5 leaf strokes feathered into it: "big tree now reads as foliage
  against the air" (33a5bf:9).
- Canopy rebuilt as 12 separate foliage masses with sky painted into the gaps (496bb9:8); grove redrawn as
  four masses with a real gap under the canopy, "the change I like most" (24be5f:29).
- A building-shadow wedge "unified the left half" (7c4dd1:6).
- Stack repainted with a 4-pile graded body pass across the form: "reads as straw, warm-to-cool turning
  form" (r31smoke:4, 10).
- Far shore: individual trees rising above the wood line "broke the comb" (f99282:59).
- Crown restated from recorded paths at half width: "a dark armature that grades out into the paler fine
  lace" (d95ea6:62).

## Process and sittings

**Late-engine sittings that re-judged each open item as IMPROVED or ACCEPTED (with a reason) limited churn and ended cleanly.**
- Kind: process. Conditions: e5 (d95ea6, 12e572, 7c4dd1, aef22d, 49b647, r31smoke).
- Evidence: e.g. "Judged each passage that was still unresolved ... ACCEPTED ... More local work would make it
  worse" (d95ea6:31-36); "Nothing new needed doing, so this sitting added no paint" (7c4dd1:39); "Every
  passage I flagged has now been improved or accepted for a stated reason" (r31smoke:93); look-only sittings
  (aef22d:64-71, 49b647:60-68). Contrast with e2 journals without this structure that cycled for hundreds of
  days (3a958c, 24be5f).
- Interpretation: aef22d declared FINISH five times (49, 56, 59, 63, 71) with tiny changes in between;
  repeated sittings past a stable finish produced little.
- Already in prompts? partly (brief: finishing loop "Is there something you would enjoy taking further?").

**Handoffs that list live globals (masks, piles, recorded paths) let later sittings work precisely.**
- Kind: process. Conditions: e5.
- Evidence: r31smoke:20, 27, 40, 47 ("globals still alive: shadow4_m, stack2_m ..."); d95ea6 found its
  recorded GROUPS/CROWN only by dumping globals (d95ea6:59, 69).
- Already in prompts? no.

**Written lessons didn't prevent repeats; the same trap recurred two to four times within one journal.**
- Kind: failure-pattern/process. Conditions: all.
- Evidence: "Writing a lesson down is not the same as having learned it" (35c488:83); "day 59's notes were
  right about all of these, and I broke every one of them again" (ba1453:219); "I blended a rectangle again,
  on reflex" (24be5f:20); 3a958c's "mistake #5 (same family!)" (3a958c:14).
- Interpretation: a prompt-level rule may help more than the painter's own journal, because the trap is hit
  before the journal is reread.
- Already in prompts? n/a.

**Stop repainting a passage that has been corrected three times and got worse; accept it and say why.**
- Kind: process. Conditions: e2-e5.
- Evidence: f99282:56, 74; 33a5bf:26 ("Every local fix on dry paint here has left a seam"); 3a958c:28 ("every
  further pass would put the twigs and the fog at risk"); r31smoke:70.
- Already in prompts? partly (brief: "If the repair recreates the problem, reconsider the composition or
  method").

**Scratch canvas trials work when they reproduce the real underlayer and its drying; several candidates side by side, then the chosen one on the painting.**
- Kind: process. Conditions: e5 r31 smoke only.
- Evidence: plum hatch tried "on scratch first, over a dried sh_core/sh_mid2 hatch" (r31smoke:12); three
  violet-fleck piles on a dry gold hatch (r31smoke:23); three eave fringes over a dried roof (r31smoke:29); sun
  disc copy dried 60 days, three levelling trials (r31smoke:53-57); poplar crackle reproduced by repeating the
  glaze+blend and drying 60 days, three covers compared (r31smoke:62-66). Limit: a scratch trial on the right
  recipe still failed on the painting when the pile wasn't the passage's actual recipe (r31smoke:30). Some
  trials were run on the painting and lifted with the rag instead (r31smoke:42).
- Already in prompts? yes (guide, The scratch canvas: "lay the underlayer you mean to work over, and let it dry
  as long as it has on the painting"; brief).
- Note: the smoke painter discovered the scratch canvas mid-painting (first scratch use at day 175); earlier
  trials were "on the canvas edge" in other studios, which 33a5bf:15 flagged as misleading ("Must stop testing
  on the canvas edge and then going straight to a broad pass").

**Biggest time sinks and rescue cycles** (my reading):
- 3a958c (e2): four whole-sky repaints and many fog recoats over an oak whose twigs weren't excluded; got out
  by painting the oak last, recording every twig polyline, and guarding all later masks (3a958c:10, 15, 22).
- 35c488 (e2): three full rebuilds from "the ground"; got out by deciding values once, covering with
  pressed opaque passes, and laying zones in a fixed order (35c488:52-64, 125).
- ba1453 (e2): about forty chunks chasing soft gradients with `work`; settled for flat fields plus
  blend-inside-mask on objects (ba1453:171-186).
- 496bb9 (e2): land repainted wholesale three times after blend accidents, losing pond, figure and trees each
  time; journal ends unfinished at day 228 (496bb9:20-24).
- 24be5f (e3): the grove reworked from day 9 to day 739; got out by plastering it with sky and rebuilding as
  separate masses painted wet into wet with the sky (24be5f:15-16, 29).
- 33a5bf (e2) and f99282 (e2): local patching on dry paint leaving seams; got out by whole-area wet sessions
  (33a5bf:22) or by stopping (f99282:56).

## Tool confusion

**`rect(x, y, w, h)` takes width and height, not a second corner; passing y2 flooded the canvas three times.**
- Kind: easel/confusion. Conditions: e2.
- Evidence: "rect(x, y, w, h) takes a HEIGHT. I had been passing the second y. Every band ... ran to the
  bottom of the canvas" (f99282:27, 33).
- Already in prompts? partly (guide signature `rect(x, y, w, h)`; look crops use two corners, which may
  feed the confusion).

**`smoothstep(a, b, y)` makes a half-plane, not a band; print `m:at(x, y)` at a few points before painting a new mask.**
- Kind: easel/confusion. Conditions: e2.
- Evidence: "`work` did exactly what it was told and painted pale cream over every pixel from y=455 to the
  bottom of the canvas" (35c488:95, 99, 102); "print(m:at(x,y)) at five heights costs a line and would have
  saved me two floods" (35c488:104, 129); "print its area and bounding box before painting on it.
  `m:band(0.55, 1.0)` ... gave me a mask 168000 sq units where I expected 25000" (ba1453:147); a soft mask
  with a low threshold and fill "painted the whole lake white" (f99282:18).
- Already in prompts? no (guide lists `m:at`, `m:area`, `smoothstep` without semantics or advice).

**`below(f)` selects y > f (toward the bottom); a `below()` notch fills everything under the curve.**
- Kind: confusion. Conditions: e3, e5.
- Evidence: "below(f) means y>f" (ea8319:2); "`below(curve) * rect` fills everything under the curve inside
  the rect" (d95ea6:33, 38); e2: "`below - below` returns nothing at all, and it fails silently"
  (35c488:69).
- Already in prompts? yes for direction (guide: "`below` selects larger y values"); the silent `below -
  below` result is not mentioned (validity in e5 unknown).

**A pass clips to its own mask, not to the picture: masks must subtract what stands in front; `glaze`, `body`, `broad`, `hatch`, `scumble` aren't clipped.**
- Kind: confusion. Conditions: e2.
- Evidence: "A pass over a mask clips its own boundary, not the picture's" (35c488:10); "A wall glaze with
  hand="glaze" is NOT clipped — it laid pale caterpillars over the bottle" (ba1453:29); "glaze hand is unclipped
  and streaky" (b08dc3:2); stipple "without `clip` (it ignores its own mask)" (35c488:83).
- Already in prompts? yes (guide, "What stays inside the mask"; SN 16-17, 43).

**`work`'s `coverage` took only a number in engine 2; `stipple`'s took a function.**
- Kind: confusion. Conditions: e2.
- Evidence: 106bf6:5; 35c488:70; f99282:15, 63; ba1453:120.
- Already in prompts? Changed: the guide now says noise "can be passed to `coverage=` or `load_at=`".
- Still valid? No (per the guide), so drop.

**Small API slips that cost chunks: `ribbon` needs one width per point; `-m`, not `1 - m`; `grow()` after subtracting undoes the subtraction; `thinner=` goes inside the pile braces.**
- Kind: confusion. Conditions: e2, e5.
- Evidence: 35c488:71, 69; ba1453:221; d95ea6:80.
- Already in prompts? partly (guide shows `ribbon(points, widths)`, `-m`, `pile{..., thinner=}`).

**`body_of` silhouettes wander enough to scallop a smooth trunk; use `ribbon` with many points for long smooth outlines.**
- Kind: easel. Conditions: e2.
- Evidence: "its outline wanders nine units either side of the spine ... I proved it by walking the edge"
  (35c488:116, 134). 3a958c:4 found body_of limbs gave "nice crooked tapered limbs", a different use.
- Already in prompts? partly (guide: `body_of` default char is `soft`, "lobes ... mask edge lost in places").

**`canvas{}` can be set only once; the ground can't be revised after painting starts.**
- Kind: confusion. Conditions: e2.
- Evidence: ba1453:4.
- Already in prompts? partly (guide: the first chunk is `canvas{}`).

**`blend` with `clip=false` spreads paint outside its mask.**
- Kind: confusion. Conditions: e2.
- Evidence: 496bb9:20.
- Already in prompts? yes (guide: blend keeps paint inside the mask by default; SN 39 "Unclipped, it drags wet
  paint across the mask's edge").

---

## Ten strongest candidate prompt improvements from batch C

1. **Correct the scumble advice.** `hand="scumble"` and light lead-white scumbles laid opaque dabs, blobs or
   rectangles, not a broken veil, in e2-e5; integrating veils came from `thinner` ≈0.55 in several light
   passes. Cites: ea8319:2, ba1453:35, 33a5bf:15, 24be5f:12, 21, 7c4dd1:8, 49b647:11, 19, aef22d:12, 30, 37;
   contradicts techniques "Glazing and scumbling".
2. **Give glazes a pigment share and load, not just medium.** At medium ~0.7 with ordinary pigment they came
   out as saturated bars; what worked was ≤0.6 parts pigment, medium ≥0.7, load ≤0.35. Cites: 12e572:5, 33,
   35, 7c4dd1:9, aef22d:19, r31smoke:6, 496bb9:7, 3a958c:8.
3. **Blend rule.** Blend only inside one wet tone, with the mask's boundary where paint already matches, one
   pass, never a rectangle over contrasting wet paint, never touching dark edges or dry passages to keep.
   Cites: 3a958c:9, 19, 25, ea8319:2, 24be5f:17, 20, 35c488:37, ba1453:84, 133, 12e572:3, aef22d:26, 38,
   r31smoke:3, 69, 496bb9:20.
4. **Name the spirits rag as the undo** for wet paint over a dry layer in techniques ("Keeping control" now
   says "No undo"), with its limits: not on tacky or thick medium-free paint; leaves a streak where a wipe
   ends; fresh rag per pass. Cites: ea8319:2, d95ea6:14, 26, 39, 12e572:22, 7c4dd1:14, aef22d:36, r31smoke:10,
   40, 24be5f:11.
5. **Whole field, not patch.** To correct a field, repaint all of it up to real edges with the original piles
   (re-knifed from the log), over dry paint; local opaque patches on thin or blended fields dry as boxes.
   Cites: d95ea6:7, 24, 49b647:44, 57, 33a5bf:22, 35c488:73, 82, 24be5f:19, r31smoke:31, 45.
6. **Value ladder before a pass.** Mixture value misjudged in both directions; lay three to six candidates
   opaquely (scratch) and read them in `mode: "value"`; white-heavy piles go pale and lose hue, earth/black
   piles go near-black, thin lines over pale fields go lighter and warmer. Cites: 35c488:91, 106, ba1453:26,
   81, 222, 33a5bf:10, 24, 12e572:23, f99282:44, 52, d95ea6:53.
7. **Paint fine structure last and guard it.** Exclude twigs, figures and objects from every later background
   mask (subtract after any `grow()`), and keep their paths in globals so they can be excluded or restated.
   Cites: 3a958c:8, 10, 14, 15, 35c488:8, 10, ba1453:32, 221, 12e572:8, 496bb9:9, d95ea6:41, 59-70.
8. **Pale over dark only when the dark is dry, and check `drying()` at several points**, since tacky paint
   breaks the pale into patches and open paint mixes into it. Cites: 49b647:5, 17, ba1453:95, 189, 3a958c:2,
   26, 35c488:81, f99282:9, d95ea6:11, 29, 7c4dd1:14.
9. **Keep corrections within one value step**, and soften seams by hatching over dry paint with noise-edged
   fingers of the neighboring recipes rather than blending or glazing. Cites: 7c4dd1:15, 20, aef22d:13, 54,
   49b647:10, r31smoke:17, 73, 76, 3a958c:24.
10. **Verify a new mask before painting it** (`print(m:at(...))` at a few points, `m:area()`): half-plane
    `smoothstep`, `rect` height vs corner and remapped `band()` caused whole-canvas floods. Cites: 35c488:95,
    102, 104, f99282:27, 33, ba1453:147, f99282:18.

Runners-up: warn against whole-canvas "unifying" glazes (ba1453:140, f99282:20, 35c488:89); add the
"dabs + one directional blend while wet = shimmer/reflection" recipe (aef22d:14, 21, 62, 12e572:10,
f99282:68); a fluid same-color coat levels dry impasto where a knife can't (r31smoke:58).
