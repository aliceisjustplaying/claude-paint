# Alice's review of Round 6 (live first reactions, 2026-09-24)

The measuring stick, in her words: **"less digital and more bad painter.
Which is good, to be clear."** A painting that looks like a bad painter made
it is the right direction; a painting that looks digital is the wrong one.

## Lab round 1 (`notes/lab/`; A old way, B new way, C third try)
- **Bare tree:** A not great; B "different bad". **C least bad**, "because
  you don't have branches in the middle of nowhere, but the tree is sort of
  unnatural." The big branches are actually fine; the small branches get
  very unnatural. "I could almost say this is almost a style, but it just
  looks kind of unnatural."
- **Foliage:** A not great, B not great either; **C is an improvement**.
  "For foliage I need to study Caspar's work myself." (Alice may study
  pictures; the painters still work from knowledge only.)
- **Rock:** A "not the worst rock"; **prefers A's texture**. A's shadow is
  "super wrong"; B has "a different bad shadow".
- **Sky:** A's texture "looks all wrong; it almost looks like JPEG
  artifacts". **B improved: "less digital and more bad painter"**, the
  right direction; still somewhat digital.
- **Water:** not terribly water-like. A's green bank looks abnormal. **B
  improved**: more bad painter, the green is a lot less bad; a lot more
  work to do.
- **Across sky and water: the orange/peach strokes aren't blended; they
  read as harsh strokes.** The "unclipped lights" wet-paint crop: "there
  might be something there."

## Lab round 2 (`notes/lab2/`, blind sheets; decoded afterwards)
Her yardstick again: "we are slowly moving from looks digital, looks still
kind of pixels [voice-to-text had "Pixar"; Alice: not Pixar, likely
"pixels"], to bad painter, which is great. I love that."
- **Rock: P (hierarchical H).** P's 3200 crop "looks actually pretty good";
  zoomed out, "more like bad painter, which again, good". Q (round 1's B)
  has dots on the lit part that look like a digital anomaly. The shadow is
  "probably still very wrong".
- **Lime: P (the OLD recipe A).** "Q [hierarchical] is really bad." "We
  have a lot of work to do in the 3D department." P zoomed in has the same
  dots she saw on the rock.
- **Oak in leaf: split.** The tree is better in P (hierarchical), but
  everything else is better in Q (old recipe): the foreground, the shadow,
  even the sky somewhat. The sky still has "JPEG artifact vibes".
- **Bare: B (C2, round 1's C on the new angular oak).** "Still unnatural,
  but slightly less so. Like improvement."
- **Bare row: B (LINES ONLY), "absolutely B"** over the distant haze. "A
  lot more work to do with B." So "select near, veil far" is out: lines at
  every distance.
- **The JPEG question:** checked (`notes/round6/jpeg_check/`): a lossless
  PNG crop of sky A and the quality-80 JPEG of the same window look the
  same; the crunchy mottle is in the paint (the stipple over the weave),
  not in the compression. PNGs are there for her to verify.

## Wet repaints (`notes/round6/wet_experiment/lossless/`; decoded afterwards)
- **l3_green: maybe A (main)**, hard to tell. A has "the weird dots
  problem, weird digital-looking artifacts"; B (wet) does not. Grass looks
  more natural on A. The rock is too strong for the rest of the image; the
  background is "actually not half bad"; the sky still gives JPEG vibes.
- **rock_B: A (wet)**, "the shading seems better on A", though "the physics
  is off, I think". Both have "the weird artifact-looking whatever" and an
  outline ("remnants of the pencil?").
- **sky_B: torn.** Both have digital-looking artifacts, A (wet) more. Zoomed
  out, "lines that are not blended properly", but "more bad painter". "The
  A crop is better if it would not have those glitches." **"We need to
  track down those glitches."**
- **water_B: B (wet), by elimination.** "I'm starting to be impressed with
  the green blob." Zoomed in both look very digital, A (main) more so.
  **"All the lines in both the sky and the water, those orangey, earthy,
  browny lines, look horrible."**

## Varnish fix: "definitely after" (approved to merge)
Rock base much better (still some digital artifacts in the lining); the
oak "before looks horrible, after actually looks passable, zoomed in";
the boulder "before absolutely horrible, after a lot less bad"; the green
rock "before horrible, after less bad".

Note: all four wet repaints were rendered BEFORE the varnish fix; the dots,
outlines and digital lining may be largely the varnish bug.

## Round 7: Evening at a Mountain Lake (wet engine, free subject)
First reactions (she wanted the plain painting, not only sheets: added
`notes/paint1/evening_lake_1000.png`, `_3200.png`, `_aged_3200.png`):
- "Not very Caspar." "Every cloud seems to be obsessed with this one
  figure, which is fine, fascinating." "Every cloud seems to be drawing
  the exact same moon inside [in] a different phase": the cloud shapes
  repeat the crescent.
- **The lighting is quite good, but "maybe a little too perfect".** (Checked:
  no lighting tool was used; the painter wrote the sky's light as smooth
  math, a 6-stop color gradient plus a Gaussian glow, `evening_lake.lua`
  lines 108–115, so every stroke got a mathematically perfect color.)
- The sky still has a lot to improve, but it's better. The lake and the
  sky look very similar ("I get it, it kind of should be").
- The mountain: not that great. The reflection (the mirror): "actually not
  half bad". The foliage: less bad, a long way to go. The grass: still
  very digital. The two rocks: not half bad. Still the boundaries.
- **Aging: "we do want the age stuff", but the craquelure is "still too
  neat, still too digital": the crack algorithm needs work.**

## Round 7 comparison (notes/round7/compare/; not really blind: she knew two)
- **B, round 2's winter: still the best.** "Still flawed", but "a better
  tree, better composition and so on", and "good entropy, perceived".
- **A, the Python control: "very simplistic, very digital tells, the sky
  is very gradient-like."**
- **C, Evening at a Mountain Lake: "just very different and very much not
  there. Digital in a different way."**
Reading: the look-first control didn't beat the engine, and the current
process didn't beat round 2. The best picture is still the one whose
painter authored its own motifs (fresh2_winter.rs: its own wood, prune,
gnarl, limb_snow, spruce, walker_fig).

## Piles (notes/round7/piles/)
She guessed B (the formula) was the piles version; A was piles. "Maybe I'm
biased now that I know I picked wrong, but don't discard piles yet,
because I think we may have something there." The helper stays parked on
branch r7-piles (unmerged). The fair test: a painter who works from piles
from the start (choose a pile, paint a passage with it, then the next),
not a formula painting with its colors swapped.

## The winter port (notes/round7/winter_port/)
**The original (round 2) wins.** Today's version at 3200 "shows the
tell-tale too-perfect lighting"; "the trees are worse and yes, they are
thinner, which is bad here"; the difference between today and today-wet
"is very subtle". So two of today's changes made the painter's own work
worse: the pointed-brush default (thinner marks) and something that
smooths the light (candidates: relief 0.2 → 0.06, removing the canvas
grain and mottle; other engine changes since round 2).

## The winter A/B (notes/round7/winter_ab/)
"d and the original are actually remarkably close. There may be cases d
almost edges out the original." In the fir-group crop the original is good
and the other two bad. **"So we want d"**: today's engine with the pointed
brush default off and the glaze min-film floor off. "We don't want to fall
into the trap of copying the round 2 original."

## Round 7: the three arms (notes/round7/arms/; blind; A = arm 1 Lua
## without procedural tools, B = arm 2 Rust program, C = arm 3 Lua with
## everything)
Live, late at night:
- **A (Lua, no procedural tools):** "a very fat trunk, and then like
  spikes as trees... like the brush you use for shaving" (she laughed).
  The sky looks okay; the water pretty okay; the big background trees
  reflecting in the pond "actually pretty good". The entropy "still reads
  like JPEG artifacts, even though these are PNG files". Cracks less
  uniform, "doesn't quite hit the mark". **A genuine rendering bug: a
  whitish horizontal line through one of the foreground grass patches.**
  "Overall fits with a good painting."
- **B (Rust):** "one of the ones that used algorithms for lighting" (she
  guessed right: "the Rust one felt the most computational"). "In many
  ways better than anything before." "Almost eerily similar to A in
  structure: the sky, the pond, the grass in the foreground, the trees in
  the background reflecting." The reflection is nice; the sky less
  JPEG-artifacty than A's; the pond some, less than A. Trees "not that
  bad".
- **C (Lua with everything):** "the sky is fine, maybe the most digital";
  the lake "actually pretty good, but the strokes a little too digital";
  the front grass a little rough. **"Weirdly, C has the best vibe
  somehow... there's something about C that feels different. Both A and
  B feel perhaps too precise."**
- Overall: **"so far none of them are moving me. The thing about round
  two is that it almost moved something in me, especially the winter
  one."** **Convergence: "Claude, without any extra instructions around
  themes, converges on roughly the same painting"** (pond, sky, trees
  reflecting, foreground grass); suggests light-touch steering of subject
  ("paint a winter scene"). Wonders whether "the easel with everything
  introduces some entropy". **"I'm more confused than ever which direction
  we should go."**

## Alice on Rounds 8 and 9 (morning of 2026-09-25, blind, before the keys)
Round 8:
- W (arm 3): a rendering glitch around the figure. Doesn't move me, but the
  right direction visually. The sky is still the same "JPEG artifacty".
  To be clear: the sky *should* have texture, a lot more texture; we're just
  not doing the texture right yet.
- Y: the round 2 original re-rendered on the new engine (correct guess).
- Z (arm 2): recycles the same elements in a different way. Sky maybe too
  muted. The tree is good, the rock is good. The figure disappears into the
  snow; that should not happen.
- X (arm 1): again the "computational light simulation" effect. And the
  bizarre trees: a trunk, then straight spikes everywhere. "I don't think
  those trees even exist." They keep popping up; fix that.
Round 9:
- K (arm 1): the same bad spiky trees.
- L: the control.
- M (arm 2): surprisingly nice. The cracks still aren't right, in a
  different way now, but we still need the cracks. The spiky trees are
  horrible, but mentally removing them, the composition is "not half bad";
  "there's something different about it".
- N (arm 3): bad; the tree is not good; feels like a regression.
Spiky trees (the pollard willow, X, K, M): "I do not think that's a
natural structure." Traced: an earlier agent's idea (notes/green.md:202)
built into the engine as "straight rods rising" (broadleaf.rs:331).
Overall: a little unhappy and frustrated, though "at least we are course
correcting now". Wants to see the exact prompts given to the painters
(round 7 showed the brief told painters what to paint).
Decisions: yes, fix the "strokes over dry paint keep their outlines" bug
(failing test first). Keep cracks, but they're wrong. Skies need *more*
texture, done right, not less (so Round 9's "no stippled veil" line fixed
the wrong thing).

## Alice on Round 10 (2026-09-25, unlabeled A, B, C; live reaction)
"This feels like progress." First real quality jump in how they look;
"not half bad ... almost tempted to say they're good, especially B and C".
All three have an ominous "offness" she hasn't felt before and can't name.
- A: trees nice, but zoomed in, twigs hang in the air. The snow is quite
  nice; the stone is nice. Something ominous, something off.
- B: impressive. The cracks still need work. Still some neatness: the
  foreground grass just stops. Still the wiry trees (left background too):
  "I think those are still wrong". The big trees and bushes finally look
  nice, but the trunk is wrong: part too smooth, then a separate bit that
  doesn't match. "Finally ... the right-ish direction."
- C: also ominous; "probably the best one ... technically pretty good".
  The texture still reads as JPEG artifacts ("I do not yet know how" to
  fix it). The moon has a weird halo; "not sure that's how it works".

## Alice on resolution (2026-09-25)
Retire the separate 1000px preview: paint, look and deliver at one size.
3200 was an agent's pick in the first engine commit (2bc5dfd), with no
recorded reason. After Round 11, try about 0.2 mm per pixel, i.e.
2250-2400 px for a small Friedrich-sized canvas (Winter Landscape is
32.5 x 45 cm, notes/research/friedrich_materials.md:18). The engine's
default canvas is 70 cm wide (crates/paint/src/canvas.rs:253). "Let's see
what it yields." Not 8570 (Monk size): the problems are decisions, not
resolution, and renders would take ~20 min.

## Alice on Round 11 (2026-09-25, unlabeled; live)
Whole paintings (D, E, F):
- D: the cracks still need work ("I say this every time"). The big tree
  starts fine but its top ends are weird. The figure is really nice; the
  pond is nice. An emptiness that doesn't do anything for her. The trees on
  the right look terrible; the fence not right, very digital. The moon is
  rotated: "the moon does not look like that ever". The background is
  "machine in a bad way". Seems to be missing things.
- E: "looks so digital". Composition okay; the sky not half bad and not
  that digital; the tree nice but too crisp. Everything but the sky
  screams digital.
- F: doesn't love the composition; "it doesn't come together". But the
  sky is the nicest, the moon is the right shape, the tree gives "more bad
  painter vibes and a lot less digital vibes". The trees on the right a
  bit digital; the figure's shadow nice but maybe not accurate.
  "Paradoxically, F might be the best one ... in sort of a bad painter way."
Trunk studies (X, Y, Z):
- X: the small branches look abnormal: "branches do not look like that".
  The snow is rather nice. The sky generic. The tree's shading simplistic.
- Y: the snow has nice texture; the tree's shading feels better (not sure
  it's accurate).
- Z: the tree a little too neat; the small branches again: "trees only do
  that when they're in extreme distress". A third take on snow; more work
  in the reflection.
- "Three different ways of snow ... I'm not saying anyone is better":
  interesting in itself.

## Alice looking at real Friedrichs (2026-09-25, voice notes)
Cracks:
- *Monk by the Sea*: the cracks almost flow, running down a little to the
  right: a direction.
- Another painting: a very different pattern, clustered, random "in a
  sort of human random way".
- Her favorite painting has few cracks (maybe restored).
- "Cracks can be clustered sometimes."
So: real craquelure has a direction, uneven density (clusters) and a
varying overall amount per painting. Ours is one uniform net.
Trees (her favorite, a winter scene): "the guy's so good at trees";
every branch carries snow, the same amount of remnant snow on each, and
"that adds so much". The tree has "so much texture", probably many small
strokes.

## Outside reaction (2026-09-25)
The person whose post started this project, shown round 10's A, B and C:
"wow!!", "The trees are amazing", "The second one especially" (B, the
summer lime tree). Two humans (Alice: "impressive") rank B high; both model
judges ranked it last ("identical leaf sprites"). Human and model judges
diverge most on this picture.

## Alice on Round 12, the tree studies (2026-09-25, unlabeled P, Q, R)
- "Looks very fractal-ey, which is the opposite of what we are going for."
- Yes, the brooms (the twig ends).
- Q (the middle one) is too symmetrical; "idk". R and P "not half bad".
- The snow "not too bad but room for improvement".
- The brown leaves (last year's, hanging low) are "an interesting idea".
