# Cézanne at this easel

Goal: an invented oil painting in Cézanne's manner, made from written
knowledge without reference images. Historical evidence and source keys are
in `notes/research/cezanne_materials.md`. Every number below is an untested
suggestion.

## Relationships first

Subject, setting, composition and canvas are the painter's choices. The
technical examples in these notes illustrate possibilities. Warm and cool
patches can give forms weight and connect them with the surrounding space
[AIC10; AIC22; NA]. Constructive marks build the planes, and the form
comes from them.

## The studio must provide the right box

This studio supplies the `cezanne` box: lead white, vermilion, emerald
green, ultramarine blue, cobalt blue, viridian, rose madder, yellow ochre,
chrome yellow, red earth and bone black. Its guide's tube table describes
this box. These are existing pigments, compiled into a dedicated studio
box; no new pigment physics was needed.

The integrator should check the supplied box before the painter starts;
the painter calls `tubes()` to confirm. An existing painting keeps its
logged box. `palette{set_out={...}}` restricts that box; it cannot conjure
a missing tube. The painter chooses which tubes to set out and use; the
box is not a recovered palette for one particular painting.

This mapping assumes a newly started painting on the current engine.
`crates/easel/src/api.rs` gates `turps`, `blot`, named oils and absorbent
ground to engine 4 or later; `thinner` requires engine 3 or later. An older
logged painting retains its engine. Check the studio and log rather than
promising these options to an older session.

Do not substitute Rinmann's green or verdigris for emerald green as if they
were historically identical. If confined to the default box, an approximate
study is possible, but document the restriction; it is a configuration
problem, not a missing pigment implementation.

## Map the hand to existing Lua primitives

| Historical problem | Existing primitive and painter decision |
|---|---|
| A luminous commercial ground, visible between marks [NG; BUT] | `canvas{ground=...}` with lead-white-rich preparation, optionally a small ochre addition. Its thickness and texture are a study choice, not a universal historical recipe. Reserve the dry primed surface by not painting it; do not use `raw` to imitate exposed white priming. |
| Light searching placement, uneven blue/green wash [NG; AIC22] | Optional `pencil("H")` with `:sketch`, or a small round carrying a dilute blue/gray pile, `turps=` or `thinner=`, and `:stroke`. Place a few structural relationships; keep the wash discontinuous. No obligatory complete sepia underpainting. |
| Parallel patches with differentiated direction [BUT; NG; NA] | A flat or filbert brush, `b:load`, several short `b:stroke` marks, then another local pile. For a selected passage, `work(m,{hand="hatch",tool="filbert 8",length={...},angle=...,pile=p,fill=false})` is available. Defaults are starting points, not Cézanne's hand. Set lengths/widths to the physical canvas and inspect the scratch result. |
| A loaded brush that gives successive unequal strokes [BUT] | Keep the same brush and load across marks; it depletes and picks up wet color. Pressure, attack/lift `ramps` and orientation change the footprint. Choose where the patch stops; don't reload every stroke or add noise to a perfect grid. |
| Modulation instead of a monochrome light model [NG; AIC22] | Mix a handful of adjacent warm/cool piles with `pile`, `mix` and `p:add`; inspect with `look` on the palette. Put separate marks beside one another. Values still matter, but no per-pixel shading function supplies the answer. A `form{}` scaffold is optional knowledge, never a field to trace or fill. |
| Limited wet-in-wet contact [BUT] | Put neighboring strokes into paint still `"open"` under `drying(x,y)`. Pickup supplies the physical mixing. Keep most marks differentiated; a broad `blend` campaign would defeat the structure. |
| Transparent overlaps and thin opaque patches [NG] | `turps` leaves a lean thin film; `thinner` supplies solvent that evaporates over painting time. `medium` is added oil, increases fluidity/transparency and slows drying. Use a small brush or shorten `work`'s `hand="glaze"` length for a local overlap: the default glaze spans 120–300 units. A glaze hand is brush handling, not a guarantee of transparent color. |
| A projecting light or unfinished edge held in reserve [NG; LETTER] | Leave exposed ground or an earlier wash. `work` defaults to `fill=false`; use `hug=false` where useful, or stop individual strokes short. Don't manufacture a white-hole pattern after covering everything. |
| Passage across an object/background junction [AIC10; NA] | Paint the two sides in dialogue, allowing a few strokes to cross or share a color. Normal strokes need no hard clip; `work` can overrun its mask. Use the existing `lose(m,{pile=p,where=...})` selectively, not a global edge filter. Lost contact can be a gap, a shared tone or overpaint, not necessarily blur. |
| Late blue searching contours [NG; NA] | A small round or rigger and `:stroke`, interrupting rather than enclosing every form. Revisit drawing while revising color. Do not use a uniform black outline as the whole solution. |
| Returning after set paint [NG; NA] | End a chosen sitting, `wait(minutes)` and check `drying` in the actual passage. For example, `wait(3*24*60)` advances three simulated days, but is not proof that a slow lake is dry. Wait longer if it isn't. No time passes merely while the agent thinks. |
| Local revisions, not a complete repaint [NG; AIC22] | Overpaint a changed relationship and leave some earlier marks exposed. The rag and knife already exist for correction; no historical claim here requires scraping as a routine Cézanne step. There is no undo. |

**Mark scale.** Canvas width is always 1000 units. If, for example, the
painter chooses a 500 mm-wide canvas, one unit is 0.5 mm. Butler's inferred quarter-/half-inch brush widths would
therefore correspond to nominal widths around 13/25 units; actual footprint
depends on pressure. Her one-inch stroke in a late painting would be about
51 units long. These figures describe particular late works; the painter
sets the canvas size and brush sizes.
Try a flat/filbert patch and a smaller searching round on the scratch canvas.
Compare at whole-picture distance before committing to one scale.

**Touch is available, but not the signature.** `b:touch` provides a physical
press/hold/lift with optional drift; `brush("stippler",width)` and
`stipple(m,{pile=...,width=...,coverage=...})` exist. A touch can make an
isolated accent or link a small gap. Most constructive marks need an actual
drag, not a screen of dots. `notes/stipple.md` contains older Rust-side
color-aiming examples; use the present Lua pile API, not target-color
functions or its automatic field construction.

## A campaign, not a complete algorithm

1. Read the brief. Choose the picture's own spatial and color relations;
   lightly place them and keep the ground available.
2. Mix a few piles. Test neighboring warm/cool patches over the same
   preparation on scratch, including one wet edge and one dry overlap.
3. Work one local relationship, then answer it elsewhere in the picture,
   so near and far passages develop together. The exact historical order
   after lay-in is unknown.
4. Look at the whole and a detail. Decide which relationship needs revision,
   whether some paint must first set, and where an earlier state should remain.
5. Return in separate sittings. Stop when the
   relationships hold, not when every bit of ground disappears. Don't equate
   stroke count, elapsed months or accumulated layers with fidelity.

The engine supplies time and paint state; this note does not invent a
`sitting()` API or claim that the guide's clock automatically enforces the
integrator's budget. Evidence shown to Alice follows `principles.md`: plain
full image and independently chosen crops, no painter self-verdict.

## Gaps, ranked by likely harm to this oil study

There is **no demonstrated blocking gap in stroke construction, modulation,
passage, reserve or multi-session overpainting**. The following are fidelity
limits, not requests to break the feature freeze. Their visual impact is a
research judgment; no Cézanne rendering has been tested in this task.

1. **Historical paint formulation and calibration — moderate, uncertain
   impact.** Tubes implement pigment identities but the guide calls their
   material properties estimates. Butler found substantial chalk in some
   paints; the palette API does not give the painter inert extender content
   or historical particle-size distributions. These can affect thin paint's
   opacity, body and matte appearance. `medium`, `blot`, solvent and ground
   absorption offer handling choices, not an exact substitute for the sampled
   formulations. This is the largest remaining oil-material fidelity gap,
   not evidence that the existing colors cannot work.
2. **Specific oil/resin preparations — low to moderate, uncertain impact.**
   Named linseed/poppy/walnut oils exist, but do not expose the identified
   degree of heat-bodying or a separate resin-bearing Haarlem siccative.
   That preparation was ordered, not detected; requiring it before painting
   would overstate the evidence. Use existing oil/solvent handling and actual
   state checks rather than pretending to reconstruct its drying exactly.
3. **True watercolor-on-paper behavior — low for this oil study, high
   for a literal late watercolor.** The documented watercolor process
   needs a water-based binder, paper and drying wash behavior. The current
   guide supplies an oil easel, not that medium. Thin oil can approximate
   some translucent patch relationships but is not watercolor. This is no
   reason to make all late oils transparent: the technical studies warn
   against that equivalence.

Non-single-point perspective, uneven finish and changing stroke direction
are **painter decisions**, not missing physics. Already-implemented pigment
boxes and touches are not engine gaps. Record any limitations and let the
picture determine whether another capability is needed.
