# Every painting on main's engine and on the wet merge

Every painting program we have, rendered at 2400 px on two engines:

- **main**: branch `wet-before`, which is main at `4abe5b4` plus the ports;
- **wet**: branch `wet-merge`, which is the merge `84cb412` (`MERGE.md`) plus the same ports.

The ports are byte-identical on both branches (`PORTS.md`). 36 paintings
rendered on both. One could not be rendered (round 1's winter) and one
wasn't ported (r7-python); see [Skipped](#skipped).

**Bottom line.** The wet engine changes every painting, but only a little:
the mean difference runs from 0.31 to 1.14 OKLab ΔE ×100, where about 2
is the smallest step you notice side by side (`notes/spectral.md`: "a
just-noticeable difference is about 0.02"). The change is almost always
the same one. A light laid into a wet dark now stays on top as a separate
stroke, where main sank it into the dark. That is what the branch was
for, and it shows best in r15 p2's snowy oak, r10 arm 3's lime and the
grass in round 4's free painting. The same behavior has costs:

- soft blends turn into hard-edged shapes;
- thin strokes laid over dry sky break into a dotted weave;
- a few thin light marks come out thinner or fade.

The wet engine takes 2.1 times as long in total.

## Where the pictures are

All files are in `~/tmp/wet-merge-9d4eadac/`; none are committed.

| path | what |
|---|---|
| `before/NAME.png`, `after/NAME.png` | the renders, 2400 px wide, main and wet |
| `sheets/NAME.png` | one sheet per painting, lossless: main above wet, whole, with 3 numbered crop windows outlined; below, the 3 crops at 1:1, main left and wet right |
| `look/NAME.jpg`, `look/NAME_cK.jpg` | JPEG previews of each sheet and of the crops I looked at closely |
| `stats/NAME.json` | the difference numbers in the table |
| `diag/` | the two diagnostics under [Regressions](#regressions) |
| `logs/jobs.tsv`, `logs/NAME_{before,after}.log` | every render's exit code, wall time and peak memory; its output |
| `sheets.py`, `imgdiff.py`, `render_one.sh`, `make_table.py` | the scripts that made all of this |

**Crop windows.** Each window is 900 × 560 px. The sheet picks the three
windows that don't overlap and have the largest mean difference (`sheets.py`).
So the crops show the places where the engines disagree most, not a fair
sample of the painting.

**The difference.** Each pixel's OKLab ΔE ×100 between the two renders
(`imgdiff.py`). The table gives its mean over the painting and its 95th
percentile.

**Times and memory.** Each render ran as
`/usr/bin/time -l timeout 1800 <binary> --full --width 2400` (or
`easel run ... --width 2400` for a Lua log), from a release build of its
branch (`render_one.sh`). Times are wall-clock minutes and memory is the
peak resident set in GB. Renders ran two at a time from one queue, so each
time includes sharing the machine with another render. Treat them as
rough.

## The paintings

"About the same" means I saw no difference at sheet scale or in the crops.
Every sentence comes from the sheet and its crops.

| painting | ΔE mean | ΔE p95 | min (main / wet) | GB (main / wet) | what changed |
|---|---|---|---|---|---|
| round 1 coast | 0.92 | 2.57 | 3.8 / 8.5 | 1.1 / 1.3 | Wet: the thin cloud bars in the sky and the dark streaks on the sea are fainter; the brig, the figure and the anchor are unchanged. |
| round 1 mountains | 0.84 | 3.13 | 5.5 / 13.1 | 0.7 / 0.9 | Wet: the dark rim along each range's crest is gone and the mist under each range runs more evenly; the cross, the spruces and the figure are unchanged. |
| round 1 winter | – | – | timed out / timed out | 0.7 / 0.8 | Not rendered: the snow-impasto loop never ends on either engine (timed out at 1800 s on both), so the archived file can't be the version behind the picture Alice saw. |
| round 2 coast | 0.59 | 1.71 | 2.1 / 4.7 | 1.0 / 1.1 | Wet: the thin pink cloud streaks laid over the dry sky break into a regular grid of dots on the weave's peaks, with a small ring round a bare hole (the same with r6-wet's own plough, so it's the wet model, not the merge); the net's strokes are more contrasted; the sea, poles, woman and boulder are about the same. |
| round 2 mountains | 0.48 | 1.56 | 2.1 / 4.7 | 1.0 / 1.2 | Wet: the stippled mist between the ranges is grayer, with fewer white flecks, so the ranges' feet read darker; the wanderer, the spruces and the sea of fog are about the same. |
| round 2 winter (main's `fresh2_winter`) | 0.40 | 1.01 | 1.8 / 4.3 | 1.0 / 1.3 | Wet: the snow touches on the spruces and along the fence rails are fewer and smaller, the oak's limb snow a thinner band and its pale trunk streaks nearly gone, so the dark forms read blacker and more solid. |
| round 3 easel, free (dolmen by the sea) | 0.50 | 1.27 | 2.6 / 5.7 | 2.0 / 2.3 | Wet: the thin horizon cloud streaks break into a dotted, weave-bound texture and the sea is more speckled; the trees, the dolmen and the glints are about the same. |
| round 3 easel, green (summer tree) | 0.87 | 3.76 | 3.5 / 7.3 | 2.2 / 2.6 | Wet: the crown's lit clumps are a lighter yellow-green and its shadow side darker, so the tree has more contrast; the pale branches drawn over the shadow side mostly disappear. |
| round 3 easel, near (rock with birch and bracken) | 0.85 | 3.05 | 5.3 / 8.6 | 1.9 / 2.2 | Wet: more of the bracken fronds stay bright over the dark foreground and the rock's lit face is a little lighter; the spruces and the birch are about the same. |
| round 4 easel, free (dead tree and dolmen at dusk) | 0.77 | 2.73 | 2.2 / 5.9 | 1.6 / 1.9 | Wet: the pale grass blades laid into the dark field stay visible as a scatter of light strokes where main sank them, and the clouds lose the rim main drew along their edges; the tree, the dolmen and the town are about the same. |
| round 4 easel, green (summer valley) | 1.14 | 4.38 | 3.4 / 5.7 | 1.9 / 2.2 | Wet: the pale limbs inside the big tree's crown mostly disappear under the foliage and the light touches on the dark hedge stay as larger, paler rosettes that read as polka dots; the thistle stem is thinner; the clouds and the valley are about the same. |
| round 4 easel, near (erratic in snow) | 0.73 | 2.76 | 2.7 / 4.9 | 2.8 / 3.2 | Wet: the spruce skeletons in the dark wood behind the rock show through as a faint lattice of straight lines and a soft pink cloud patch collapses into small rust-colored specks; the rock's dark flecks and the spruces' snow are fewer and smaller. |
| loop 3 green (`notes/loops/l3_green.lua`) | 0.69 | 2.32 | 4.0 / 6.7 | 2.1 / 2.3 | Wet: the hedge's light rosettes are larger and paler (the polka dots panel 3 found on the branch), the valley floor carries more separate light dabs and the river is a lighter band; the tree and the sky are about the same. |
| loop 5 near (`notes/loops/l5_near.lua`) | 1.09 | 4.13 | 2.9 / 5.2 | 2.4 / 2.8 | Wet: the translucent pale band across the wood's foot (the pale veil `notes/wet.md` §3 credits the branch with removing) is gone and the spruces keep their inner needle strokes down to the snow; the rock and the stump are about the same. |
| r7 arm 1, pollard willow on a lake | 0.82 | 2.42 | 3.4 / 6.3 | 2.1 / 2.5 | Wet: the willow's trunk shows more pale bark streaks and one dark crack stroke, and the grass at its foot stays light over the dark ground; the rods, the far shore and the sky are about the same. |
| r7 arm 2, poplars by a pond | 0.55 | 1.75 | 2.1 / 5.1 | 1.1 / 1.4 | Wet: the reed blades are finer and a lighter olive, so the reed beds read thinner; the poplars and their reflections are about the same. |
| r7 arm 3, the bodden | 0.73 | 1.97 | 2.8 / 6.1 | 2.2 / 2.6 | Wet: the gray mist band under the far town breaks up to show pale yellow water through it, and the water below is lighter and more even; the town, the windmill and the sky are about the same. |
| r8 arm 1, frozen pond with a cross | 0.36 | 1.02 | 2.0 / 4.3 | 1.7 / 2.1 | Close to unchanged: on wet the frozen pond is a little lighter and its dark shoreline softer. |
| r8 arm 2, the way to the ruined choir | 0.54 | 1.41 | 1.8 / 4.4 | 1.0 / 1.2 | Wet: the mist over the foot of the spruce group is a lighter, more even veil and their snow touches a little brighter; the oak, the ruin and the rock are about the same. |
| r8 arm 3, snowy hill with a church | 0.41 | 1.05 | 0.8 / 1.5 | 2.0 / 2.4 | Close to unchanged: on wet the spruces show a little more of their lighter green and the church is a little paler in the haze. |
| r9 arm 1, pollard willows in the snow | 0.65 | 1.87 | 3.4 / 7.2 | 1.9 / 2.2 | Wet: the willows' rods are thinner, with more sky between them and a less solid dark mass at the head, and the long low rods break into dashes; the frozen ditch is a little darker along its edge. |
| r9 arm 2, winter morning on the Ryck | 0.63 | 1.87 | 3.2 / 8.2 | 1.0 / 1.1 | Wet: the dark bark strokes on the near willow stay as separate black streaks where main blended them into the brown, and the snow at its foot shows hatched strokes; the river and the row of willows are about the same. |
| r9 arm 3, wayside cross | 0.44 | 1.15 | 2.0 / 4.8 | 1.7 / 2.0 | Close to unchanged: on wet the tree's trunk and main limbs carry faint lighter bark strokes. |
| r10 arm 1, dolmen in the snow | 0.51 | 1.24 | 2.1 / 4.6 | 1.2 / 1.3 | Nearly the same picture; on wet the big limbs carry a slightly paler lit edge, the dolmen's snow cap has a softer lower edge and the drift strokes on the snow are a little softer. |
| r10 arm 2, frozen pond | 0.38 | 1.01 | 2.1 / 4.5 | 0.9 / 1.1 | Wet: the lighter green needle strokes inside the dark spruces stay visible where main sank them into the dark; the spruce snow, the pond and the oak are about the same. |
| r10 arm 3, summer lime | 0.71 | 2.28 | 3.6 / 7.4 | 0.8 / 1.0 | Wet: the leaf touches keep their light yellow-greens on top of the dark crown, so it reads lighter and more speckled; some sky holes are smaller. |
| r11 Astra, winter | 0.31 | 0.82 | 0.6 / 1.3 | 0.9 / 1.2 | Almost unchanged; on wet the pale lit lines and the snow along the tree's limbs are a little thinner. |
| r11 Fable, winter | 0.40 | 0.99 | 1.7 / 4.0 | 1.0 / 1.2 | Nearly unchanged; on wet the spruces are a little blacker and the grass in front of them dimmer; the strip of far wood carries more small dark flecks. |
| r11 Flash, winter | 0.60 | 1.63 | 2.2 / 5.2 | 1.1 / 1.2 | Wet: the dark opening under the dolmen is a hard black shape laid on top of the wet stone where main mixed it into a soft gray, the stones' lighter dabs stay separate, the frost stipple along the far wood's foot is gone and the spruces are a lighter olive. |
| r14 first attempt, the ploughed field | 0.63 | 2.08 | 3.5 / 6.0 | 1.2 / 1.3 | Wet: the small pale puddle glints along the track are mostly gone and the grass strip along it reads paler and more separate; the furrows are about the same. |
| r14 p1, morning in the mountains | 0.59 | 1.77 | 2.7 / 6.6 | 1.1 / 1.2 | Wet: the small pale cloud touches stay as separate light patches on the blue instead of melting into it, and the mist covers more of the hills, so their dark patches show through it less. |
| r14 p2, evening town on the coast | 0.56 | 1.46 | 2.2 / 5.0 | 1.1 / 1.3 | Wet: the moor's dark touches stay separate, a pattern of rounded dabs where main blended them into one brown, and the pale pools are thinner slivers; the town is unchanged. |
| r14 p3, Baltic shore | 0.54 | 1.52 | 2.0 / 4.7 | 1.1 / 1.3 | Wet: the thin dark cloud streaks low in the sky are fainter and partly gone; the posts, the figure, the boulder and the sea's glints are unchanged. |
| r15 p1, misty mountains | 0.67 | 2.07 | 3.5 / 7.3 | 1.0 / 1.3 | Wet: the two cloud streaks become hard-edged lilac ribbons of even width where main melted them into soft feathered bands, and a mist stroke's end sits as a pale blob on a dark ridge; the dark halo main left round the rock on the hill is gone. |
| r15 p2, oak in snow with a church | 1.04 | 2.70 | 2.4 / 5.8 | 1.2 / 1.3 | Wet: the snow and pale bark strokes along the oak's limbs stay on top of the wet dark as white and blue-gray streaks, so the oak reads snow-laden where main sank them into the limbs; the sky's cloud bands are more distinct. |
| r15 p3, Baltic shore | 0.51 | 1.47 | 3.4 / 8.0 | 1.3 / 1.5 | Wet: the dune's lighter strokes stay as visible bands on its dark slope where main mixed them into one smooth brown; the sea, the net, the boat and the figure are about the same. |
| moonrise valley (main's `friedrich_moonrise_valley`) | 0.69 | 1.99 | 3.6 / 8.5 | 0.9 / 1.0 | Wet: the dark cloud strokes stay a distinct band across the moon instead of melting into the warm light and the far hill's foot breaks into dark flecks along the top of the mist; the younger figure's light collar shrinks. |

**Time.** Main's renders add up to 1.65 h and the wet engine's to 3.54 h
(`logs/jobs.tsv`, exit code 0 only). Per painting, wet takes 1.6 times as
long (round 3 near) to 2.7 times (round 4 free). Peak memory is 0.7-2.8 GB on
main and 0.9-3.2 GB on wet. The highest is round 4 near on wet.

## Most and least changed

By mean ΔE:

- **Round 4 easel, green (1.14).** The pale limbs inside the crown mostly
  disappear, and the hedge's light touches become large pale rosettes.
- **Loop 5 near (1.09).** The pale veil across the wood's foot is gone and
  the spruces keep their needle strokes down to the snow. This is the
  branch's intended win.
- **r15 p2, oak in snow (1.04).** The limb snow stays on top of the wet
  dark bark, so the oak reads snow-laden. This is the clearest improvement
  in the set.
- **r11 Astra (0.31).** Almost unchanged; only the lit lines and the limb
  snow are a little thinner.
- **r8 arm 1, frozen pond (0.36).** Only the pond is a little lighter and
  its shoreline softer.
- **r10 arm 2, frozen pond (0.38).** Only the lighter needle strokes inside
  the dark spruces stay visible.

## Regressions

These are what I would call worse on wet, by kind. Each is visible in the
painting's sheet crops.

1. **Thin strokes over dry sky break into a dotted weave** (round 2 coast,
   round 3 free), in the coast with a small ring round a bare hole. This
   comes from the wet model itself, not the merge. The round 2 coast cloud
   looks the same when the merge is built with the branch's own plough in
   place of main's per-hair one: see `diag/diag_r2coast_cloud.png`, top to bottom main, the
   merge with the branch's plough, the merge.
2. **Soft blends turn hard.**
   - r15 p1's cloud streaks become even lilac ribbons with hard edges.
   - r14 p2's moor becomes rounded dabs.
   - Moonrise's far hill breaks into dark flecks along the top of the mist.
   - A pink cloud in round 4 near becomes rust-colored specks.
3. **Thin light marks come out thinner or fainter.**
   - Round 2 winter: the spruce and fence snow and the oak's pale trunk
     streaks.
   - Round 4 near: the spruces' snow.
   - r11 Astra: the limb snow.

   In round 2 winter the trunk streaks fade the same way with the branch's
   plough, so that part is the wet model. The limb snow is thinner partly
   because of main's per-hair plough. The oak crop `diag/diag_oak.jpg` (full
   size: `diag/fresh2_winter_plough_diagnostic.png`) has the same three rows
   as the coast diagnostic; the middle row keeps a thicker band of limb snow.
4. **Structure lost or exposed.**
   - Round 4 green and round 3 green lose the pale limbs inside their crowns.
   - Round 4 near's spruce skeletons show through the dark wood as a
     lattice of straight lines.
   - r9 arm 1's long willow rods break into dashes.

The difference numbers don't separate these from the improvements: a light
that stays on top and a blend that stops blending both count as change.

## Skipped

- **Round 1 winter** (`ab_r1_winter.rs`) was ported but never finished.
  Its snow-impasto loop never ends: once a pull reaches the end of its line
  it steps back and pulls to the end again. Both engines hit
  `timeout 1800` (exit 124, `logs/jobs.tsv`). The details are in `PORTS.md`.
- **r7-python** has no engine to run on (`PORTS.md`, "Not ported").
- **Round 2 winter** is main's own `fresh2_winter`, so it is rendered
  once per engine, not ported.
