# Round 5 plan: tight OODA loops toward better paintings

User (after amnesia round 4): rounds 3 and 4 felt like sideways moves; the
paintings must improve. "Tight OODA loops are important."

## Why rounds were sideways
- Every amnesia painter starts from zero craft (spruces fought in round 3
  near and again in round 4 near).
- Each painting is a single first pass; real paintings are reworked.
- No measure of improvement besides eyeballing.

## Two goals, kept apart
- **Amnesia rounds** test tool usability (occasionally, every few loops).
- **Improvement loops** aim for the best paintings.

## The loop (30–45 min)
1. Observe: a painter reworks ONE benchmark painting at the easel for
   20–30 min, reading the craft sketchbook first (notes/sketchbook.md:
   techniques, never pictures).
2. Orient: THREE blind critics score it (median; rubric below) and name the ONE
   costliest defect.
3. Decide: craft (sketchbook note) or tool/physics (one small fix).
4. Act: the fix (< 30 min), merged immediately; next loop.

Benchmarks: the three amnesia briefs (free, green, near), starting from the
round-4 session logs. Scores in notes/scores.md per painting per loop.

Rubric (1–10 each): reads as a Friedrich; reads as oil paint (not digital);
drawing of things (trees, rocks, figures); light and atmosphere; particular
detail (especially foreground); worst defect (named).

Big tool work (envelope growth for trees/spruces/rocks/figures, paint that
warns about open paint, painter-shaped masks, ground research check,
spectral) is split into single-defect fixes the loops pull in.

## Sketchbook rules (so craft raises quality instead of freezing it)
1. Every entry states its ceiling ("gets a readable silhouette; still
   chevron-regular"): recipes are floors to beat, not targets.
2. Entries earn their place: kept only if the critic's score on that axis
   rose when used; beaten entries are replaced, not kept beside the new one.
3. Principles before recipes.
4. Pitfalls are permanent (they only prevent bad results).
5. It opens with: "the best so far, not the best possible; if a passage
   still reads digital, try something new and record it if it scores
   better."
- PAUSED 17:31 (user's usage limit): painter4-free and spectral interrupted. Resume with subagent_resume on their session files (see ~/.pi/agent/sessions/--Users-USER-src-a-claude-paint-easel4-free--/ and ...-spectral--/), message: continue where you left off.
- RESUMED 18:32: painter4-free and spectral.
- Loop 0 (baseline): blind critic scoring 8 paintings (r2 x3, r3 x3, r4 green/near) shuffled as P1..P8; key in scratch judge0_key.txt.
- Loop 1 started 18:42: loop1-green and loop1-near rework the r4 paintings against the critic's defects (25 min, sketchbook); loop1-halo fixes the pale halos (tool). Then re-judge with anchors.
- spectral merged as optional module (not integrated; verdict in notes/spectral.md).
- r4 free done: dolmen AGAIN (same motif as r3 free): convergence on motif, not just mood.
- Loop 1 scored: near +3 (23->26), green +2 (18->20), anchor -1. Sketchbook updated. Loop 2 started 18:58: loop2-near (drift puffs), loop2-grain (horizontal wood-grain ground streaks; tool). loop1-halo still running; green loop 2 waits for it.
- Loop 2 critic2: l2_near 22 vs l1_near 19 in-batch (+3), but anchor fell 27->24 and l1_near 26->19 across batches: single-critic absolute scale is noisy (±4-7). Rule from now: 3 independent critics per batch, median; decisions on in-batch deltas vs anchors.
- loop1-halo merged: halos were an engine bug (contact level averaged thick paint into its thin neighbors; now a running median). Golden re-recorded.
- Loop 3 near and loop 2 green started 19:22 on the halo-fixed engine. Next batch must re-render previous versions with the current engine for fair comparison. loop2-grain still running.
- Loop 3 scored: near +1 (21->22), green 0 (17; halos from cut-out masks + scribbles). Loop 4 started 19:51: loop4-near, loop3-green, tool-firs (firs/woods grown into drawn envelopes). loop2-grain still running.
- loop2-grain: grain gone (coarse anisotropy 7.4-8.5 -> 2.2-2.8) but new 1-px orange dotted contours in thin skies at 3200 and strong swirl; NOT merged; resumed to fix the dashes' root cause and calm the swirl.
- tool-firs merged: firs grown into drawn envelopes + receding fir woods (easel fir{}, fir_wood{}); sketchbook §5 recipe.
- Loop 5 started 20:37: loop5-near (repaint wood/spruce with fir tools, from loop 3), tool-oak (trees grown into drawn crowns), tool-rock (rocks from drawn outlines). loop2-grain-2 still on the dashes (and glaze dotted loops over turf are the same family).
- User (after loop 5): critics should be a PANEL of google/gemini-3.8-flash and
  openai-codex/gpt-6-astra (better vision than Claude); 2 + 2 per batch, medians
  overall and per family. Also: user is "of two minds" about structure tools
  (fir/oak/rock grow into drawn outlines; they don't paint, the painter does):
  finish this round, look at the output, decide then.
- Rest of round 5: tool-oak, tool-rock (+ grain fix if clean) land; one more loop
  per painting (near: rock + stump patch; green: oak); panel judging; stop and
  show before/after with the score history.
- Anonymity: user requires no real name anywhere in the repo. Working tree scrubbed (home paths -> ~), pre-commit hook in the shared hooks dir rejects it, briefs updated. Git HISTORY still contains it (422 lines in old notes): needs a history rewrite + force-push when no agents are running (ask user).
