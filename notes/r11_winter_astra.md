# The Ford Before Daybreak

Original winter painting, r11-astra. No reference images or earlier paintings consulted.

## Composition and intent
A low, cold valley before sunrise. A frozen watercourse bends out of the foreground toward an opening in the eastern sky. One old, leafless oak occupies the left bank; a traveler has stopped on the right. The stream is a threshold, not a picturesque road: its near ice is broken and its farther course disappears in mist. A very small church on the distant ridge offers an uncertain destination rather than a dominant Gothic emblem.

The unequal banks and displaced figure should keep the composition from becoming a centered symbol. Dark roots and reeds arrest the eye below; the oak's living twigs reach into the large still sky. Particular details include splintered wood, snow on upward-facing branches, exposed bank earth, ice seams, dry seed heads and the traveler's stick.

Friedrich connections: precise drawing before thin paint, warm earth under a light ground, stippled sky and distance, lead-white snow, fine grass strokes laid last. These are documented in `notes/research/friedrich_materials.md` §§2–6. The metaphoric threshold, bare trees, back-turned traveler and remote sacred building draw on knowledge of his recurring motifs, not an image or a copied composition. Film thicknesses and brush parameters are artistic approximations, not historical measurements.

## Working record
- Read README, material research and the required color, strokes, stipple, form, workflow and motif API notes. Read brush, style and path source plus stroke study.
- Scratch retained at `~/tmp/r11-winter-astra-3de42621`; all render commands set TMPDIR, TMP and TEMP there and use timeout.
- Planned sequence: light prepared linen → drawing → sky → distant land and church → snow and ice → trees → traveler → foreground particulars → very slight varnish and relief.
- All motifs are authored in `paintings/src/bin/r11_winter_astra.rs`. No engine edits and no borrowed tree, rock or figure painting routines.

## FRICTION (running)
1. The default Friedrich ground is red-brown rather than the patchy lead-white upper ground needed for a pale winter dawn (`style.rs`, `Style::friedrich`). Workaround: explicitly choose a light top preparation and reduce ground relief.
2. Brush width is not the deposited width, especially with a pointed tool (`Tool::mark_width`, `pressure_for`). Workaround: solve pressure against each desired branch width and hand off to smaller tools along tapering branches.
3. Whole-canvas masks remain full-sized even for crops (README and workflow notes). Workaround: use direct gestures for tiny motifs and keep large masks local to their stages.

## First preview and revision
The first preview completed in 10.2 seconds. Viewed using `scripts/peek` as `preview-01.jpg` in scratch. The quiet sky and distant scale work. The oak emphatically does not: painting every structural segment as a separate tapered gesture leaves polygonal elbows and pale joints. The firs read like comb teeth, the snow lacks bank structure and the figure stands too far onto the ice. Revise these before accepting a full render.

4. Separate pointed gestures at each tree segment visibly pinch at joins. Workaround: pull each complete bough in one loaded gesture, then texture the continuous wood. The continuous bark highlights were already smoother than the underlying wood, making the error obvious.
5. Very small hatching strokes can become thin combs rather than conifer masses. Workaround: increase overlapping needle-bearing strokes, use irregular tree spacing and group the far wood into unequal masses.
6. The repository pre-commit hook rejects absolute home paths in notes. Workaround: record the retained scratch directory with `~/` rather than spelling the home directory.

## Full-resolution inspection and second revision
A complete 3200px render was produced in 67.3 seconds, alongside a refreshed preview. Examined the whole picture plus actual full-resolution bark and ford crops (`full-03.jpg`, `bark-full-03.jpg`, `ford-full-03.jpg` in scratch). Also rendered a dedicated 3200px oak window with `--crop 110,280,370,620` (30.6 seconds). The crop paid 16.35 seconds for the distant land despite seeing little of it: whole-frame preparation is a real workflow cost here.

The second tree version connects, but the full-size brush marks have squared bristle edges and pinstriped bark. Adopted a different physical method: construct a continuous, tapering bough mask from my own spline drawing, brush body color inside it along the wood, then add broken fine bark marks. This is still paint, not a pixel fill. Sky and snow remain brushed/stippled passages. The traveler's position is corrected and the fir spacing is irregular.

7. A single large pointed brush cannot cover the entire width range of a bough. Large-tool tips vanish before the geometric end and expose detached twigs; continuous large gestures also leave squared hair tracks. Shortening release ramps and overlapping smaller tools helped, but the final workaround is a brushed continuous wood mask with a local stroke-direction field. This costs more planning but gives control of silhouette and form.
8. Full-palette aiming produced unwelcome cyan-green patches in gray ice and over-green conifers. Workaround: restrict ice to lead white, cobalt blue, raw umber and bone black; give wood and needles similarly limited pigment families.
9. A low-contrast soft mask does not automatically yield a soft-looking brushed snow shadow: the first drift pass showed scalloped blue bars. Workaround: widen the transition and approximately halve the requested value change, rather than assuming mask softness alone controls the optical edge.
10. Regularly spaced small shore gestures read as road markings. This is a painter-side misuse of the brush planner, not an engine defect. Workaround: omit most edge accents, vary the bank itself and add pale broken ice shelves rather than uniform white dashes.

## Final corrections
- Viewed another native-resolution window, `--full --crop 70,475,370,670`, in `wood-stones-05.jpg`. It exposed the ribbon's rounded trunk foot hanging below the roots. Truncated that cap at an uneven turf line and painted separate tapered root ribbons into the bank. This is a geometry limitation to remember: a round stroke end is not a planted trunk.
- Broke the repeated stone silhouette with individual vertex changes and roughened masks. Snow caps now have discontinuous edges and the exposed stone has separate dark and lighter passages.
- Added a small stand of dried umbellifers near the lower-right bank. Removed the uniform blue shadow dash under every grass tuft: without a risen sun those dashes were both mechanical and unjustified.
- The full ford crop caught three hand-placed melt marks outside the stream. This was my coordinate error, not an engine bug. Their x positions now derive from `stream(y)` so they remain inside the ice.
- Reduced bark-highlight contrast, softened the snow hollows and let the fir group recede toward the right rather than putting smaller trees progressively closer.

## Final method, stage by stage
1. **Preparation:** 640 mm-wide fine linen with warm lower grounds and a light 48 µm rolled top ground. Numerical thickness is a simulator choice, not a measured Friedrich value.
2. **Drawing:** light graphite for the principal boughs and stream axis, fixed before painting.
3. **Sky:** curved crossing strokes, restricted five-pigment family and a wet stipple pass with a 2.5-unit tip. Cool gray-blue above, a restrained mauve transition and warm gray-yellow toward the eastern horizon. No photographic sky or image texture.
4. **Distance:** three uneven low ridges, a tiny solid church, distant trees and a stippled mist belt. Air is represented by value, hue and edge choices rather than copied scenery.
5. **Snow and ice:** a continuous cold snow field, warmer reflected light, a meandering ice passage, sparse exposed banks and two broken pale ice shelves. Color families for the ice exclude chrome yellow.
6. **Snow relief:** shallow, softened depressions and a contact hollow beneath the oak. No invented hard sunlight.
7. **Woods:** my own branch architecture converted to a continuous ribbon mask, brushed along local wood directions with earth-pigment body color. Fine twig growth is connected to those branches. Bark fissures and snow rests are separate final strokes. The firs are my own overlapping short hatches.
8. **Banks and traveler:** individually weathered stones, interrupted snow caps, a back-turned figure in a coat, boots, hat, staff and faint approaching footprints. All are local geometry and brush gestures authored here.
9. **Winter particulars:** clustered grass, seed heads, forked umbellifers, exposed earth and a fallen branch. These go over the finished snow.
10. **Finish:** a very slight warm varnish (0.055 coats), restrained relief and no decorative craquelure. Aging is not used to disguise awkward drawing.

## Honest final critique
The best passage is the low dawn: the large quiet sky, remote church and small traveler establish scale without an oversized emblem. The asymmetrical oak and bent watercourse make an original composition with a clear pause at its center. Close viewing rewards the forked stems, ice cracks, broken bark and subdued footprints.

The weakest passage is still the near wood: it is more smoothly designed than a truly observed old oak, and the smaller branches retain a schematic recursive quality. The distant firs remain conspicuously conical. The snow's brushed transitions can read as soft digital patches, while the ice shelves and stones are too cleanly planar. At full resolution this is a restrained program-painted landscape, not a convincing substitute for Friedrich's minute, irregular hand. I would improve observation and edge variety before adding more objects. No claim of historical reconstruction is intended.

## Top five friction points
1. **Brush taper and joins:** one tool cannot span a thick bough through a hairline tip without premature disappearance or pinched joins. Final workaround: a connected mask brushed along the wood, separate fine twigs and shorter release ramps.
2. **Palette hue surprises:** broadly aimed gray passages showed cyan/green patches. Final workaround: restricted pigment families for ice, wood and needles.
3. **Soft mask versus soft paint edge:** soft snow masks initially made scalloped blue bars. Final workaround: wider transitions, much smaller value changes and less uniform contact marking.
4. **Small hatches versus coherent masses:** sparse conifer strokes read as comb teeth. Final workaround: denser overlap, unequal tree spacing and several grouped silhouettes; the remaining conical look is noted above.
5. **Crop preparation cost:** a small full-resolution crop still paid nearly 20 seconds for the distant-land stage. Final workaround: direct gestures for tiny details, scoped masks and whole renders when a crop would save little. No engine changes.

## Accepted deliverables and checks
- Program: `paintings/src/bin/r11_winter_astra.rs`.
- Preview: `out/r11_winter_astra.png`, **1000 × 699**, final render **17.6 s**.
- Full: `out/r11_winter_astra_full.png`, **3200 × 2238**, final render **80.4 s**.
- Viewed only through `scripts/peek`: `accepted-preview.jpg`, `accepted-full.jpg` and `accepted-ford.jpg` in the retained scratch directory. Earlier actual full-resolution crops of the oak, roots and stones are retained there too.
- Both final `cargo paint` commands completed successfully under `timeout 900`. `file` confirmed PNG dimensions. `git diff --check` passed. No engine or shared source files were edited. The render PNGs stay in the repository's ignored `out/` directory; the program and this note are committed.
- Preview SHA-256: `c143e35f609ec2d667da310149c9e97d9a26778aedd2e9ae59c4bc511876a919`.
- Full SHA-256: `856431ba6e11db834fdc0fd0682885cd73374cc80dc8c3e4597b8ddab641562c`.
