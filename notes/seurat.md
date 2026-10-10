# Seurat at this easel

Read `principles.md` first. This is a craft translation, not a recipe for a
finished picture. Historical findings and source keys are in
`research/seurat_materials.md`; the recommendations below are **interpretation**.
Work from words and knowledge, never reference images. No painting or engine
change was made for this research.

## What makes the picture Seurat's

**Structure before texture.** A deliberate arrangement of horizontal bands and
vertical accents, with diagonals or curves that prevent a dead diagram. The
harbors make this especially clear [NG-G; ORSAY]. In figure compositions, bodies
read as held silhouettes and related intervals, with the classical-frieze
ambition documented for *Grande Jatte* [AIC]. Stillness is an effect of the
whole arrangement, not a command to freeze every subject; the later dance and
circus pictures are exceptions to a universal “nothing moves” rule.

**Tone before the dots.** The Conté drawings teach masses of light and shadow
and edges made by adjacent tones [YALE]. The oil painting must already hold as
masses before its divided color is worked. A patterned surface cannot repair an
unresolved composition.

**Divided color, but mixed piles.** A blue-green shadow, a violet, a pink and a
white-tinted blue can each be mixed physically, then laid separately. Relate
neighbors deliberately: the documented orange–blue and green–purple contrasts
are more useful than scattering equal quantities of every primary [NG24].
White-tinted body paint is legitimate here; the generic warning against white
in `techniques.md` is not a historical ban on Seurat's tints.

**Varied marks that belong to forms.** Sweeps, crosses, dots and short dashes;
fatter near beach strokes, smaller distant ones, finer touches for faces. Later
does not always mean smaller. Do not imitate stippling by spatter, or put one
uniform dot screen over a smooth completed picture [NG24 pp.27–30].

## Map the process to tools that exist now

The current Lua contract is `easel_guide.md`, checked against
`crates/easel/src/api.rs`. `stipple.md` is the Rust implementation history:
its old target-color/aiming examples are **not** painter Lua instructions.

| purpose | existing primitives | how to use them here |
| --- | --- | --- |
| Conté tonal study | Scratch canvas; limited black/white `pile`; lightly loaded `brush:stroke` or `gesture`; value/squint `look` | Make a small monochrome oil study of the masses. This is an analogy, not simulated Conté on Michallet paper. |
| Croqueton | Small `canvas{size=..., aspect=...}` on scratch; broad brushes, lean piles | Try the motif quickly in broad strokes. Current support remains cloth: do not call it a wood-panel simulation. |
| White oil preparation | `canvas{linen=..., ground={...}}`, lead-white pile, knife or brush preparation | A white oil ground suits the later study. Do not default to `absorbent=true`; historical grounds are work-specific. |
| Drawing and reserves | Fine `brush:stroke`/`gesture`; painter-defined passage masks; brush up to a boundary | Place essential silhouettes and leave their space while working around them. Masks guide anchoring, not perfect filled edges. No tracing from images. |
| Lean colored lay-in | `pile{..., turps=...}`; `work` or broad `gesture` strokes | Establish sky, water, land and shadow as colored masses. Keep paint thin. |
| Balayé and directional underpainting | `gesture`, `stroke`, or `work` with direction, length and cross options; `drying(x,y)` | Sweep and cross strokes by passage. Wet-in-wet is documented for *Bridge*'s underpainting; it is useful here, not required everywhere. |
| Separate pointillist touches | `brush{kind="stippler",width=...}`, `b:load`, `b:touch`; `stipple(m,{pile=...,width=...,coverage=...,pressure=...,dips=...,drag=...})` | Work one passage and pile at a time. Use several related piles in separate short campaigns; keep earlier strokes visible where wanted. |
| Dashes, modeled features and masts | Short `b:stroke`/`gesture`; `b:touch` with `drag`; small round/flat/stippler | Change direction with form. A face is not made by increasing dot density over a generic head. |
| Separate campaigns | `wait(minutes)` and `drying(x,y)` | Wait until the actual supporting film is dry where clean separation matters. Real time thinking does not dry paint. |
| Local border contrasts | Brush a reserved edge strip, then touches/dashes from locally chosen piles | Treat each side/corner according to its neighbor. Border is painted matter, not a post-processing frame. |
| Repair | Overpainting; wet lifting/wiping or knife scraping per guide | Correct masses before adding more dots. Dry paint cannot be erased by wishing. |
| Two viewing distances | Whole `look`, `mode:"value,squint"`, full-detail `crop`, gallery/relief modes | Judge mass and color integration in the whole view and pigment separation in crops. Let the integrator choose final evidence crops. |

**Glazes are available, but secondary.** Oil-rich piles and glaze handling exist;
use a local transparent correction only if it serves the painting. A global
glaze homogenizes the separated colors and is not the documented core Seurat
sequence. Similarly, blend the underpainting only where needed, not the final
dry divided-color surface.

## Palette and mark trials before the painting

Call `tubes()` first. The guide's compact table is not the complete current tube
catalog. `palette.rs` provides the dedicated `seurat` box with the core
Seurat pigments and strontium/zinc yellow, but boxes are build/studio-specific:
`palette{set_out=...}` restricts tubes already present; it cannot add absent ones.
The integrator can supply that box with `box-seurat` without adding engine capabilities.

For a first late coastal study, select from lead white, cobalt blue, ultramarine
blue, viridian, emerald green, chrome/cadmium yellows, vermilion and one red lake.
Do not feel obliged to use all of them. Add zinc or strontium yellow only if the
selected study warrants it and the box contains it. `rose madder`
is the box's practical tube proxy for historical lakes, not an exact analysed batch.
Keep earths/black available for an early-study route rather than claiming Seurat
never used them. The current catalogue lacks manganese violet; mix a blue/lake
violet as a declared approximation if that particular late border needs it.

Knife a small working set of piles, examine the palette, paint trials, then
adjust with `p:add`. Do not create a coordinate-to-RGB function, solve per-pixel
target colors or mechanically copy a color-wheel diagram. Palette dirt is
available, but clean/reload before a passage where unintended pickup would ruin
separation. Sixteen active piles are enough for a passage, not the entire
picture's lifetime supply.

**Proposed calibration, not a historical measurement:** on scratch, test tool
widths around 1.5, 3 and 5 canvas units, at several pressures, with short drags
as well as touches. At a 500 mm canvas width, one unit is 0.5 mm. The stipple
implementation note estimates a mid-pressure mark at about 0.8 × tool width;
look at the actual deposited marks rather than treating this as an exact law.
The guide's full-detail view has 2.4 pixels/unit, so the smallest trial is only
a few pixels across and can disappear in the whole view. Do not force historical
millimeter dots onto an arbitrarily reduced canvas.

Compare (a) touches into an open colored underlayer and (b) the same touches
after `wait` and verified `drying`. Wet pickup fuses the former; a dry layer
allows the latter to remain separated. Try successive sparse passes, leaving
visible underpaint, before a dense pass. `coverage` is touches per point, **not**
dot spacing in mm or a percentage of unpainted canvas. Multiple passes can
overlap. Choose size, loading and density by looking, not by a historical number
the sources do not supply.

## Targets for a first study, in words

These are ranked **study recommendations**, not a requirement to copy a known
canvas. The painter should invent a related scene from the written principles.
If Alice explicitly commissions a named study, these descriptions suffice to
start without supplying a reference image.

1. **The Channel of Gravelines, Grand Fort-Philippe (1890), best first target.**
   Nearly equal pale blue/lilac sky and pale golden open beach, divided by a
   narrow blue channel and low seawalls/buildings. A signal tower and masts rise
   above, two mooring posts answer below. A small grassy triangle and a diagonal
   light band interrupt the sand; the apparent horizontal rises subtly to the
   right. There are only distant people. A locally contrasting dotted border
   completes the color relationship [NG-G]. It tests divided tone, sea/land
   contrast and still geometry without requiring many anatomical figures.
2. **Port-en-Bessin, outer harbor at high tide (1888), next.** Sloping cliffs
   confront flat jetties and horizon, with upright masts; a winding road softens
   the strict order and wild near grasses disturb the stillness. The harbor is
   emptied of people [ORSAY]. It adds asymmetry and a greater hierarchy of
   land/water marks without a crowd.
3. **An early Seine croqueton, a preparation rather than the pointillist goal.**
   A small river reach with fishermen, barges and distant industrial smokestacks,
   stated with broad strokes [MET-S]. This is the lowest-cost test of masses,
   reserves and directional handling, but will not prove mature divisionism.
4. **Bathers or Grande Jatte, later figure studies.** *Bathers*: still bathers on
   a grassy riverbank, broad colored grass and smoother flesh under later dotted
   revisions [NG24]. *Grande Jatte*: a park crowd arranged with the dignity and
   held silhouettes of a classical frieze, shade contrasted with sun [AIC].
   Start with a few figures in an invented river scene, not the whole monumental
   crowd. Reproducing its roster is a much harder composition/anatomy test than
   proving the paint method.

## Engine gaps, ranked by harm to this target

These are limitations, **not feature requests**. The feature freeze stands.

1. **Color fidelity: highest for a faithful technical reconstruction, moderate
   for a style study.** Pigment colors/strengths are estimates; mixing uses
   Mixbox latent colors and the layered optics use RGB scattering/absorption
   (`palette.rs`), not measured full spectral data from Seurat's paints. The eye's
   simultaneous-contrast response is not a separate engine color operation.
   Juxtaposition is possible today, but exact historical optical relationships
   cannot be certified. Choose piles by viewing trials; do not compensate with
   a computed brightness boost.
2. **No neighbor-aware multicolor stipple placement: moderate at large scale,
   low in a small hand-worked study.** A batch uses a jittered grid, one pile
   and one tool size. Independent color passes need not interleave and may cover
   earlier dots. Manual touches and smaller campaigns solve the artistic need
   today, at a time cost. No need for an automatic whole-picture dot allocator.
3. **No actual Conté/Michallet or rigid wooden panel: low for a finished late
   canvas, high if the deliverable were a faithful drawing/croqueton.** Monochrome
   oil on scratch can test tonal organization; it cannot reproduce greasy dry
   crayon catching laid-paper tooth. A smooth cloth ground cannot reproduce bare
   mahogany absorption and grain.
4. **No work-specific pigment degradation chemistry: low for fresh intention,
   high for reconstructing present condition.** Zinc-yellow alteration, lake
   fading and environmental chemistry are not reproduced by simply advancing
   drying time. Do not make browned green dots the default palette.

Resolution and brush-time cost are practical constraints, not missing painting
primitives. Ten thousand touches already mean roughly an hour of simulated hand
time before dips; hundred-thousand-dot passes can consume a day. Work in bounded
campaigns and follow whatever sitting/rest limits the studio enforces. Do not
inflate stroke counts to demonstrate diligence.

**A painter can start today**, provided the studio exposes the existing tubes.
Start with a small coastal study, verify separated color over a dry lay-in on
scratch, then compose and paint. The largest uncertainties are fidelity and
judgment, not the absence of a dot tool.
