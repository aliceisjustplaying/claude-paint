# Passage and clock matrix pass

Agent-run 2026-10-04, clean pinned source `4e525e50897807e9b5f734071dfeb330f1a393d3`, default release easel, actual2400×480 CLI sessions in disposable roots. [Runnable passage fixture](../fixtures/matrix/passages.rb), [raw command receipts](matrix-passages.json), [supplement](matrix-supplement.json) and [extended receipts](matrix-extended.json) preserve the exact narrower aspect5 fixtures. These replace the checklist's square-region coordinates where recorded; they do not claim the unrun square image.

## Passage controls

The [atlas](matrix-shape-passage-atlas.png) shows load, spatial direction, tool width/pressure, coverage/fill, order, edge/cut-in, stipple and lose controls. A spatial angle callback was invoked18079 times; the matching scalar control has a different orientation field. The load callback made the left half of the region visibly lighter than the right. Round2 versus filbert12 produced respectively fine separated and broad overlapping marks. Sparse fillfalse leaves gaps; filltrue adds material and increases simulated time09:00→09:10. Across/down/scatter produced different sequences. Lost edge is irregular; cut-in forms a tighter rectangular edge. Stipple variants alter dot width/density, while lose returned126 executed strokes around the boundary.

A successful W plus marker produced exactly chunk2 and time09:06; another W produced chunk3,time09:11 and cumulative paint. One hundred individual touches plus W made one chunk6. Two W calls made one chunk7. A local mask/pile disappeared by the next chunk while globals remained userdata. Wrong mask/pile types, unknown tube, conflicting edge/cut-in, invalid keys even on empty/tiny masks and an erroring spatial callback all rejected. The validation baseline PNG stayed unchanged. Unsupported undo was nil. These operations ran locally through the actual easel without a model connection.

Wet and30-day-dry red-earth layers received the same pale brush stroke. The wet case mixed the contrasting paint; the aged layer remained visibly underneath. Earlier recorded clean-blend controls establish that a confirmed dry layer is unchanged by clean blending. Shared depth/clip behavior is recorded in [space evidence](matrix-space.md#depth-clip-combined).

## Drying stages and material comparisons

The initial single-touch sample missed bristles and read dry; that fixture is not used as evidence of wet-film aging. It was replaced with coverage3 detail patches and an explicitly confirmed open center.

Red earth progressed open→setting at day2 01:24→tacky at day2 17:24→dry at day9 17:24. Raw umber was setting at day1 13:24,tacky at17:24 and dry by day3 17:24. Lead white was setting at day1 17:24 and dry by day5 17:24. Bone black remained tacky at day9 17:24. These observations establish differing pigment behavior and all four reported labels for these fixtures, not real-world curing predictions.

A60-second wall pause preserved clock,stage,dampness and every decoded PNG pixel. Successful explicit waits advanced the simulated clock immediately. Wait5259600 followed by wait1 in the same chunk succeeded;5259601 and NaN rejected. The boundary execution took milliseconds and did not sleep for its simulated duration. Existing rollback receipts and the new cloth rollback compare restored time and image after a failed chunk.

## Subminute timing and palette eviction

The public rag dampness field supplied an independent continuous-time measurement: it halves every180 simulated seconds. Each20-second pile creation changed damp1→0.9258747101; a2.5-second reload changed1→0.9904191494. Three distinct pile creations accumulated60seconds. Drawing consumed time; erase/fix/sharpen left dampness unchanged. Rounded minute displays concealed shorter intervals as described.

The original17-white/red mixtures did **not** establish17 palette entries: close colors deduplicated, so the oldest reload still cost2.5seconds. A wider pigment mix also reused colors. Exact eviction was therefore tested at the real public engine boundary, [ledger.rs](../fixtures/matrix/ledger.rs), linked against the same compiled production paint library. [Output](matrix-ledger.txt) shows16 separately admitted colors,2.5seconds for the oldest before overflow,22.5seconds after the17th color (20mix+2.5reload),16 retained entries and2.5seconds for the newest. This is an actual production ledger, not a model of it. The CLI additionally confirms that retained `p1` stays userdata and can load a brush after many other mixtures.

## Intermediate aging slices

[The real replay fixture](../fixtures/matrix/slices.lua) uses a77-minute stipple passage with `--frames-every 1`. The production frame observer recorded intermediate frames at954.3,1932.4,2897.3,3847.9 and4628.3seconds before the final chunk frame. [Frame index](matrix-slices.tsv) retains these receipts. This establishes intermediate slicing and residual flush, rather than merely a final minute display.

**Precision correction:** “15-minute slices” is a threshold at completed work boundaries, not an exact900-second timer. The observed gaps were approximately950–978seconds for this fixture. The stronger checklist interpretation of exactly15minutes is a documentation failure; it does not establish a simulator defect.

## Shared command boundaries

[Actual per-feature transport cases](matrix-transport.json) cover passage/wait disconnects,serialization,captured input,external log mutation,server death and persistence faults. They observe a pending accepted chunk after an actual native operation. They establish the shared accepted-source/commit boundary; they do not claim a user can pause halfway through one native painting instruction. Provider-driven use of the same Lua is covered by [the actual painter harness](matrix-harness.md#actual-painting-and-image-history).

## Final material and interaction checks

[Finer five-minute material sampling](matrix-material-boundaries.json) resolves the initially equal coarse observations: medium0 reached setting14:03/tacky18:58/dry day4 03:33; medium0.5 reached15:23/21:38/day4 21:18; medium0.9 reached15:48/22:33/day5 02:58. The oil-medium setting therefore changes actual observed drying boundaries. Coverage1 and3 also gave different stage progression in [matched layer cases](matrix-final-passages.json).

Confirmed open, setting, tacky and dry patches received the same clean blend and pale brush stroke. The [final atlas](matrix-final-atlas.png) shows mixing/continuation over open paint and a cleaner opaque band over set films. Wet pale paint above a dry red layer was lifted to reveal that retained layer. Varnish on the confirmed-open fixture rejected with a dry-state error; after30days the same call succeeded ([material receipts](matrix-material-boundaries.json)). Real complete finishing recipes and their actual drying loop are covered by [delivery evidence](matrix-delivery.md#restricted-state).

An empty-mask custom glaze followed by the standard seeded body pass matched the unmodified-default control pixel-for-pixel. Found/soft/lost edges were run separately. The actual harness program produced identical output through all three CLI forms, and the shared [four-channel receipt](matrix-channels/root.json) includes the real painter tool. A300-million-iteration Lua computation between clock/dampness queries consumed1.18 wall seconds while both queries stayed09:00/damp1.0.

## Public CLI observability limit

T059 cannot be established fully with the current live public diagnostics. Dampness reads painting time including owed time; `wait` and `drying` force owed time onto the clock before returning; a completed chunk flushes it before saving; frame timestamps record accumulated hand time. These approaches were exercised, but none exposes **clocked versus still-owed time inside the chunk immediately before the one-minute threshold**. Existing owner tests establish final arithmetic and slicing, not every requested pre-query/post-query grain boundary. The ordinary CLI cannot distinguish those quantities. The production-module observation below resolved the row without changing the product.

The intermediate aging frames are also retained as a compact [replay video](matrix-slices.mp4); the numerical frame index is the timing receipt.

## Exact hand-time flush observation

The initial public-CLI observability limit above was resolved with a one-off harness compiling the unchanged production API, session and time modules against the existing Cargo-built paint/mlua/image libraries. No production API, export or clock was changed. It invoked the real Lua brush methods between session chunk boundaries and read their existing Studio/Canvas state; the final trial used normal `Session::run` to observe its end-of-chunk flush. [Fixture](../fixtures/matrix/clock-grain.rb), [production-module entry](../fixtures/matrix/clock-grain.rs), [transcript](matrix-clock-grain.txt).

The first touch left0.341275seconds owed without changing the clock. Touch176 crossed from59.723165seconds owed to a flushed clock increase of1.001074minutes and zero owed. Separate drying-query and zero-wait trials flushed0.341275seconds each. Normal chunk completion also flushed the touch's time. This is direct execution of the production clock owner, not a claim that the ordinary CLI exposes those intermediate fields.
