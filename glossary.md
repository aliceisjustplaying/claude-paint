# Glossary

**Chunk.** One submitted Lua program. A successful chunk is one entry in the painting log; it can perform many painting operations.

**Committed.** A chunk has finished successfully and its log update has been saved. A computed result whose log could not be saved is not a normal successful command.

**Discarded.** A failed chunk's changes are not kept as part of the painting. A rebuild can be needed before the session is ready again.

**Running.** The session is executing an accepted command. This is separate from the simulated painting clock.

**Done.** The requested operation has returned its result. It does not mean the painting is artistically finished.

**Saved.** Qualified by the object: the log records successful chunks; a PNG records an image; a checkpoint records state for finishing. These are not interchangeable.

**Dirty.** Not a UI state in the easel. The product has no unsaved-document badge; persistence failure instead causes an integrity error.

**Selected.** The session or painter targeted by subsequent commands or viewer actions, not a selected region on the painted canvas.

**Explicit abort.** The caller interrupts its request. Stopping a client does not by itself prove the background server stopped an already running chunk.

**Another action.** A second request or browser action starts while the first is pending.

**Environment failure.** A process, connection or storage operation fails independently of the painting's Lua logic.

**Target changed externally.** A log, session file or displayed stream changes outside the current action.

**Input channel changed.** The same operation is requested through a different input form, keyboard route or tool surface.

**Canvas.** The painting surface, including dry ground and paint films. Logical coordinates differ from output pixels.

**Ground.** The prepared dry layers over linen present before painting begins.

**Tube box.** The available named pigments for a painting. Its identity travels in the painting log.

**Pile.** A mixture of named tube paint and optional oil medium, available to load a brush.

**Medium.** Added oil in a pile, affecting transparency, flow and drying.

**Mask.** A coverage map used to plan or restrict painting. It is not itself visible paint.

**Passage.** An area painted with planned brush strokes or touches.

**Hand.** A preset approach to laying strokes, such as body, broad or glaze.

**Clipping.** Restricting a mark to a mask, distinct from merely planning a stroke inside a mask.

**Look.** An image of the current painting or palette, without changing the painting.

**Session.** The background easel state or, when explicitly called a pi session, the painter's conversation record.

**Sitting.** One painter conversation within the viewer's combined history for a studio folder.

**Replay.** Either executing a painting log again or playing recorded looks in the viewer. The document names which meaning applies.

**Hand time.** Simulated time spent on painting operations. Wall-clock computation time is separate.

**Open paint.** Paint still workable enough to blend or lift; setting, tacky and dry describe later stages.

**Checkpoint.** Stored canvas state used for finishing, not a replacement for the replayable Lua log.

**View.** In looking, a rendered image configuration; in space, a traced geometric reference for visibility and shadows.

**Harness.** The pi extensions that present painting tools and manage the painter's context.

**Compaction.** Replacing older conversation context with a summary for later model requests.

**Held brush.** A brush object with its own tool shape and retained paint. Reusing it continues that paint history; constructing another brush creates a separate holder.

**Fullness.** Paint remaining in a brush relative to a nominal full load. Additive loading can make this value exceed one.

**Stroke.** A brush mark following a submitted path of at least two points, with pressure and orientation controls.

**Touch.** A pressed brush tip at a submitted position, optionally dragging or twisting during contact.

**Ramps.** Fractions of a stroke used for pressing down and lifting off.

**Swell.** Submitted pressure factors distributed along a stroke.

**Orientation.** The brush's wide-axis direction: across travel, along travel or a fixed angle.

**Pickup.** Existing canvas paint entering a brush and potentially affecting later marks.
||||||| parent of a6d3826 (docs: add watching/studio.md)

**Event.** In the studio viewer, one recorded item in a painter history, such as a thought, tool call, result-associated image or sitting boundary. An event is not necessarily a committed painting chunk.

**Whole look.** A recorded untransformed, uncropped canvas look used as the studio viewer's main painting image and ordinary replay step. Palette and reference pictures are distinct.

**Rewound.** The studio viewer is showing an earlier selected event without automatically advancing when new events arrive.

**Static export.** A copy of the studio page, painter histories and pictures served as files. It can refresh when newer exported files arrive but does not call the live easel.
