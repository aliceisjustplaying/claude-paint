# Complete mark-making matrix pass

Agent-run on 2026-10-04 against clean source `4e525e50897807e9b5f734071dfeb330f1a393d3`, preserved default release executable. Public CLI sessions use 2400×480 canvases, size100, aspect5 and separate disposable EASEL_ROOTs. No model provider or mock rendering was used. Source remained unchanged.

## Reproduction and raw receipts

The runnable fixtures are [marks.rb](../fixtures/matrix/marks.rb), [shapes.rb](../fixtures/matrix/shapes.rb), [supplement.rb](../fixtures/matrix/supplement.rb) and [extended.rb](../fixtures/matrix/extended.rb), sharing [driver.rb](../fixtures/matrix/driver.rb). Each takes a binary, a new short disposable root and an image/results output directory. They open/close actual sessions and preserve every submitted source, exit status and output. [Initial marks](matrix-images.json), [corrected shapes](matrix-shapes2.json), [supplement](matrix-supplement.json) and [extended comparisons](matrix-extended.json) are the recorded results. No assertion is passed merely because a constructor accepted an option.

## Brushes

The [brush atlas](matrix-brush-atlas.png) shows separately controlled ramps, swell, shake, orientation, pressure and touch controls. Along orientation narrows the flat brush's track; pressure and ramps alter its taper; drag extends a touch horizontally and angle/twist rotate its footprint. The pointed-round stroke broadens from left to right under pressure0.1→0.9. All11 table overrides were accepted and painted matching paths.

[Decoded pixel comparisons](matrix-mark-metrics.json) found zero differences between default/explicit stroke and touch controls. Reload matched explicit wipe0.85 then load exactly, while the fresh pale brush differed visibly from the residue-bearing reload. The wipe left canvas pixels unchanged and the independent rag's fields unchanged. Separate successful chunks depleted retained brush paint. A stroke crossing the wet red stripe carried red paint onto clear ground, unlike the matched unstriped control in the [rag/pickup atlas](matrix-rag-atlas.png).

Clip stroke bounds were x720–839,y96–287 pixels, exactly inside logical rect(300,40,50,80). The clipped touch stayed x720–839. Unclipped controls extended outside both limits. A filename in place of pile, extra medium argument and attempted brush setter failed. A pile containing medium loaded successfully.

## Shapes and fixture correction

**The earlier runtime evidence and the first new shape fixture used an invalid calling form in several outline examples:** `outline(points,{options})`. The public API takes one table, `outline{pts=points,...}`. The second argument was silently ignored. Those original character/corner/seed comparisons are invalid evidence. The corrected single-table runtime produced0/4 corners for false/true,1/2 for explicit indices and rejected index99. Character plans contained8 firm,31 searching,24 broken and43 soft strokes. Omitted character matched firm; body omission matched explicit soft. Seeded repetitions matched length and mask area; mutated original points/corner tables left the first outline unchanged. The corrected [shape atlas](matrix-shape-passage-atlas.png) shows the resulting character and boundary differences.

Geometry construction left the PNG unchanged. Below/above samples selected opposite sides; equal scalar/list ribbon widths gave equal areas. Grow/offset increased area, shrink reduced it, rim selected the interior edge and blur softened the boundary without changing the original mask. Roughening changed the derived area only. Signed distance and a zero band behaved as described. Sampling beyond left/right canvas bounds equaled the corresponding edge samples. Callback masks retained old0.25 coverage after the source variable changed to0.75. Polygon smoothing changed area. Invalid body spine/limb/width cases rejected. Size0.9 and lobe0.49 rejected with the correct single-table form; size1,lobe0.5 and zero lobe accepted. Amount0 removed lobes even when lobe20 was supplied.

Noise callable/at matched; at01 returned bounded values for fbm/ridged/billow/plain. Warp/stretch changed the value. Worley callable matched its first returned distance and returned the other documented fields. Default/explicit noise and Worley options matched. Uneven default/explicit sequences matched; changing seed changed them. Random samples remained within requested uniform ranges; this finite probe does not prove an entire probability distribution.

## Drawing

The [drawing atlas](matrix-drawing-atlas.png) compares line rounding, rulers, pressure profiles, tremor, sketch passes/wander, hatch angle/spacing/length/pressure and hard/soft graphite versus chalk. Soft graphite and chalk leave darker, wider bands than hard graphite. Sketch12 has more separated marks than sketch1; wander0 brings repeated passes together. Hatch angle rotates the marks, spacing20 reduces their density and shorter length fragments them.

[Pixel receipts](matrix-mark-metrics.json) confirm default=explicit line0.5, sketch0.3/3passes and hatch0.45. Passes0=1 and99=12. Erase strength−1=0 and2=1. Changing supplied-mask erase width4→20 had no pixel effect. One-point path erasing was accepted. A fixed band retained drawing after erase while the loose band lightened; newly drawn loose material could subsequently lift. Wet paint blocked new pencil deposit at the sampled covered center.

The [extended receipts](matrix-extra-metrics.json) confirm exact default/explicit physical-size behavior at100mm and400mm for tremor0.15mm, wander2mm, hatch spacing/length and eraser width4mm. A custom brush, different-grade pencil and rag did not change a later seeded HB line: exact pixel equality. The rag dampness clock changed for a pencil line but remained identical through erase,fix and sharpen.

## Rags

Pressure−1=0 and2=1 pixel-for-pixel. Omitted pressure/passes matched0.5/1 exactly. One-point and0.0001-unit wipes matched blot exactly. Path and profile controls changed lifting; a mask wipe extended beyond its planning boundary. Loaded aliases shared contamination; independent cloth started clean. Refolding retained soaked contamination, reset face load to soaked and cleared dampness. Zero dip did not instantly dry a damp rag. Three simulated minutes halved dampness; further aging below0.01 made it zero. A60-second real pause left clock,drying stage,dampness and PNG unchanged.

Automatic refold−1 matched0, and2 matched1; the low threshold produced11 folds with dry final face, versus0 folds at threshold1. Passes20 ran and retained the same loaded cloth. Fresh, loaded and refolded faces left different subsequent blot patterns. Rag rollback restored image,clock and all recorded cloth fields. Empty mask and off-canvas blot left load/soaked unchanged. Invalid finite/shape/option cases produced the documented errors. A30-day-aged region reported dry and returned zero rag load; fresh paint above a dry underlayer could be lifted while the underlayer remained visible.

## Shared interruption boundary

[The live transport fixture](../fixtures/matrix/transport.rb) submits an actual brush,mask,drawing,rag,passage or wait operation followed by bounded Lua work. [Receipts](matrix-transport.json) establish queued disconnect, disconnect after execution begins, serialization, captured file input, external log edits, killed server and persistence failures. The observed interrupt is inside the accepted chunk after the native operation, not halfway through a single native brush/rag algorithm. Rows whose claim is chunk serialization or accepted-source immutability use that shared boundary evidence; this is not a claim of native instruction-level control.

## Remaining controlled cases

[Final mark receipts](matrix-final.json) and the [final atlas](matrix-final-atlas.png) cover outline painting pressure/shake/ramps/clip and full explicit defaults. Default pressure1/shake0.3/every3/load0.6 matches its explicit control. Lower pressure breaks up the outline; shake4 bends it; ramps change its ends and clipping confines the visible fragments. A combined brush/pencil/mask failure restored recorded fields and the exact PNG. Constructor and print behavior were also inspected. Historical engine2 printed `nil` and passed `assert(rag==nil)`; its subsequent render error is expected because that compatibility probe deliberately has no canvas.

The10,000 uniform samples had mean0.4978978, variance0.084221 and range0.00000614–0.999948. Normal samples had mean0.002000 and variance0.998599. These are observed distribution checks, not exhaustive statistical proof.

The first brush preference comparison was confounded by different numbers of constructors consuming deterministic random seeds. The corrected [equal-constructor-count fixture](../fixtures/matrix/preferences.rb) varied the first brush's settings only; the later default brush produced zero differing pixels. [Receipts](matrix-preferences.json) retain the actual construction and marks. An accepted but invalid image filename failed both the mask callback constructor and rag wipe.

[Exact channel comparison](matrix-channels/root.json) establishes identical decoded output through inline source, file, stdin and the real pi painter tool for the combined brush/mask/pencil/rag/wait program. [Missing-server probes](matrix-missing.json) separately reject every corresponding operation after the actual session was closed.

[Stage-boundary receipts](matrix-stage-boundary.json) show the matched fresh rag collecting0.8408317 face load from open paint. The same contact over explicitly setting paint collected0.7389184 ([material receipts](matrix-material-boundaries.json)); dry/tacky contacts collected zero. A long native wipe was then calibrated near the setting boundary: its center changed from open at17:43 to setting at18:52 within one actual `r:wipe`, with1119 refolds and near-saturated cloth. [Native-aging receipts](matrix-native2.json) retain both the5-minute calibration and crossing run. This establishes aging during the native rag operation without a fake timing hook.

The final [decoded comparison receipts](matrix-final-metrics.json) record zero differences for combined rollback, outline-paint defaults, equal-constructor brush preferences and passage preferences. [Corrected closure receipts](matrix-outline-closure.json) separately establish open=true, closed=true, both flags, an empty consumed inset, distinct closed-path endpoints and opposite open-outline above/below regions. These supersede the earlier two-argument flag probe.

The [tool/cloth atlas](matrix-tools-cloth-atlas.png) retains all11 brush table-option marks and the fresh/loaded/refolded face comparison. In the later blot region, decoded grayscale mean was0.482280 for fresh cloth,0.451092 for loaded cloth and0.601420 after refolding: the loaded face exposed less light ground and refolding restored more lifting capacity. The region is separate from the earlier loading wipe.

The blot means use crop96×96 pixels at offset1032,216 (logical x430–470,y90–130), then ImageMagick grayscale mean. This excludes the earlier horizontal loading wipe at y60.
