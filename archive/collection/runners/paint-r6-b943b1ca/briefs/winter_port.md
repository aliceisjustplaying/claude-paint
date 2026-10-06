# Port round 2's winter painting to today's engine (branch r7-winter)

House rules: ~/tmp/paint-r6-b943b1ca/briefs/r7_common.md.

Alice judges round 2's winter painting the best picture this project has
made ("a better tree, better composition", "good entropy, perceived"),
still better than anything since. Its painter authored its own motifs in
Rust: `git show origin/amnesia-winter:paintings/src/bin/fresh2_winter.rs`
(its own wood, prune, gnarl, limb_snow, spruce, walker_fig, crow). The
question: **the same painter's choices on today's engine: better or worse?**

## Do
1. Port fresh2_winter.rs to today's engine (main, this worktree) as
   `paintings/src/bin/fresh2_winter.rs` (plus notes/round7/winter_port.md).
   **Change only what the API forces.** Keep every choice the painter made:
   composition, colors, tools, stroke plans, seeds, the order of work, its
   own motif functions, its finish (its own `Cracks` values: fill new
   fields with the values that reproduce its old look, i.e. the
   `Cracks::even`-style settings, and note it). Where today's API has no
   equivalent, do the closest faithful thing and list every such change.
   Today's defaults that change the look anyway (relief 0.06, the varnish
   fix, the pinhole fix, clip masks honored, `[profile.test]`) are part of
   "today's engine": don't undo them.
2. Render at 1000 and 3200: `cargo paint fresh2_winter -- --full`.
3. Also render it on the wet engine: build the same port in a scratch
   worktree of branch r6-wet (`git worktree add $TMPDIR/wet r6-wet`, copy
   your fresh2_winter.rs in; fix only what differs) at 1000 and 3200.
4. The original round 2 renders (built today from its original branch)
   are at ~/tmp/paint-r6-b943b1ca/r2winter/out/fresh2_winter.png
   (1000) and fresh2_winter_full.png (3200).
5. For Alice (lossless PNG, committed under notes/round7/winter_port/):
   the three versions whole at 1000 side by side (original, today,
   today-wet) with plain labels, and three 1:1 crops from the 3200 renders
   at fixed windows chosen by geometry, not taste: the bare tree's crown,
   the ruin with the path, the fir group (same windows for all three).
   Also the plain whole images. Don't write verdicts on the sheet.
6. notes/round7/winter_port.md: every API change made, what differs in
   the renders (measured: mean/max diff, where), and nothing else: no
   self-assessment of which is better (Alice judges).
Budget about 1.5 h. Commit; don't push. Final message: paths, the list of
forced changes, and the measured differences.
