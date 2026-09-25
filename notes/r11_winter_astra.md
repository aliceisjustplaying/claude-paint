# The Ford Before Daybreak

Original winter painting, r11-astra. No reference images or earlier paintings consulted.

## Composition and intent
A low, cold valley before sunrise. A frozen watercourse bends out of the foreground toward an opening in the eastern sky. One old, leafless oak occupies the left bank; a traveler has stopped on the right. The stream is a threshold, not a picturesque road: its near ice is broken and its farther course disappears in mist. A very small church on the distant ridge offers an uncertain destination rather than a dominant Gothic emblem.

The unequal banks and displaced figure should keep the composition from becoming a centered symbol. Dark roots and reeds arrest the eye below; the oak's living twigs reach into the large still sky. Particular details will include splintered wood, snow on upward-facing branches, exposed bank earth, ice seams, dry seed heads and the traveler's stick.

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

## Critique
Work in progress: first pass is legible but too diagrammatic in the tree, stone polygons and river shape. The next pass must prioritize connected organic wood and more credible snowbanks over adding further motifs.
