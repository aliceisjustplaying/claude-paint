# Glitch census (claude-paint), worktree ~/src/a/claude-paint-r6-glitch, branch r6-glitch

House rules: ~/tmp/paint-r6-b943b1ca/briefs/common.md (worktree
only, commit often, don't push, scratch via ~/.local/bin/agent-tmp, peek
to view, timeout long commands, anonymity, US English no Oxford comma).
Tests: `cargo test --workspace` now runs in about a minute (optimized
[profile.test]); before you finish also run `cargo test --release -p easel
--test hand_time`. Read notes/workflow.md "Tests: two tiers".

## Why
Alice (the project's owner; call her Alice) judges the paintings by eye.
Her yardstick: "less digital, more bad painter". Her most repeated
complaint at 3200px is small digital-looking artifacts, in her words
(notes/round6/alice_review.md has them all):
- "weird dots on the light part of the rock that look very digital,
  anomaly coded" (lab round 2 rock, version Q = round 1's B recipe:
  notes/lab2/rock_Q-ish crops, see notes/lab2/rock.md for the key);
- "once you zoom into P it also has these dots, the same dots" (lab 2
  lime, P = old recipe: pale specks inside the dark crown);
- "both A and B have the weird artifact-looking whatever" and "an outline:
  remnants of the pencil?" (the rock repaints); "some digitally artifacts
  in the lining" at the rock base even after the varnish fix;
- sky and water: "weird artifacts that look very digital", "glitches",
  "lines that are not blended properly"; the sky "gives JPEG artifact
  vibes" (a lossless check showed that mottle is in the paint:
  notes/round6/jpeg_check/);
- earlier culprits already fixed: the varnish pooling at dried stroke
  edges (notes/varnish.md) made worm lines, hatching and outlines.
Another agent is checking the pencil-outline question specifically
(graphite showing through thin paint); you may note evidence but leave
graphite.rs to them.

## What to do
1. **Reproduce and catalog.** Render at 3200 (crops are fine: `easel run
   <log> --width 3200 --crop x0,y0,x1,y1`) the passages Alice named: the
   lab 2 rock (both versions), the lab 2 lime P, the lab 1 sky and water B,
   l5_near and l3_green (logs: notes/lab/*.lua, notes/lab2/*.lua,
   notes/loops/*.lua), on main. Also render the same crops on the wet
   engine (branch r6-wet, worktree ~/src/a/claude-paint-r6-wet: build its
   easel and run it on the same logs; don't edit that worktree) since
   Alice saw more glitches on its sky.
2. **A detector.** Write a small tool (a paintings bin or a script via
   `uv`) that flags artifact candidates in a render: isolated pixels or
   tiny blobs far from their neighborhood (dots, specks, pinholes), thin
   high-contrast lines along paint edges (outlines, lining), blocky or
   periodic mottle in quiet passages. Output counts and an overlay image.
   Calibrate it on what Alice saw (the crops above) so it finds her dots.
3. **Trace each class to its cause.** Use the stage tools: re-render with
   the log truncated before the finishing chunk (varnish, relief,
   cracks), with relief off, with cracks off, before and after specific
   chunks (`easel` edit/replay or copies of the log), and at 1000 vs 3200.
   For each class say: engine bug (which code), painter recipe (which
   call, e.g. a stipple over the weave, bare ground left in a reservation,
   hairline grass), or real paint behavior that is fine.
4. **Fix the engine bugs** on your branch, each with a test that fails
   first and before/after crops. Recipe problems: write them up as
   sketchbook pitfalls with the fix. Anything that is the wet engine's
   own: write it up precisely for the wet stream (don't fix r6-wet).
5. **Evidence for Alice** (she views on another machine; commit it):
   notes/glitch/ with `alice_census.png` (lossless: the named passages at
   1:1, detector overlay beside each) and `alice_fixes.png` (before over
   after for each engine fix, 1:1 crops, lossless PNG), plus
   notes/glitch.md: the catalog (class, where, cause, status), the
   detector's usage, what's left.
Budget about 2–3 hours. Benchmarks must stay byte-identical unless a fix
is meant to change them (then measure and say where). Final message: a
concise report with the catalog table, commits and image paths.
