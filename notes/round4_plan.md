# Round 4 plan: drawing (2026-09-23)

The painters' main complaint in amnesia round 3 (notes/amnesia3.md): they
can't draw. "A painter draws it; I compiled it." "Every mark was a
coordinate I estimated from a JPEG." The user approved all four remedies.

| stream | branch | owns |
|---|---|---|
| pencil underdrawing: graphite/chalk on the ground, sketch-look-correct, shows through thin paint | `pencil` | new crates/paint/src/graphite.rs; easel/src/draw_pencil.rs |
| looking by eye: grid overlay, probe, preview Lua geometry as an overlay before painting | `lookaid` | easel look.rs, main.rs look command |
| drawn outlines: a few rough points become a hand-drawn shape (curves, corners, irregularity) and masks | `outline` | new crates/paint/src/outline.rs; easel/src/draw_outline.rs |
| depth masks from the world ("behind X", "in front of X", soft cast shadows) + edit chunk N in place | `depth` | scene.rs masks; easel session.rs/main.rs chunk editing; easel/src/depth.rs |

Then: review (neutral brief), fixes, amnesia round 4 at the easel with the
same three briefs as round 3 (free, green, near) for comparison. Stop.

## Standing instruction
Take your time; quality over speed; look at every render before merging.

## Status log
- round 4 streams launched: pencil, lookaid, outline, depth. Round-3 full renders regenerating into out/easel3_*_full.png from notes/amnesia3/*.lua (worktrees removed).
- pencil merged. Open: tree painted via drawing_mask shows beaded dots along limbs (painted_1000.jpg); look --mode drawing hook pending (lookaid owns look.rs); checkpoint doesn't save eraser bookkeeping.
- depth, lookaid, outline merged (conflicts in easel api.rs/main.rs/README resolved; crops follow edits). Live check: try/show/probe/grid/edit/show N/check all work together. 134 paint + 17+1 easel tests. Next: review round 4.
