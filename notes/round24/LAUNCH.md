# Round 24 launch: the Inness painter on the frozen engine-3 candidate

## Current state

- October 4, 2026: painting candidate `f75e27f`. The old Pi agents are
  stopped and their lane worktrees removed. Development is on the M3.
- Exchange stays off; lane A's experiment is retained on `e3/a-exchange`,
  not integrated. Flow is integrated from `fbe0d43`; rag path consistency
  and carryover from `7e08c5b`, with unused diagnostics removed in `ab4cca6`.
- The owner authorized finishing carryover and concurrent checks. Earlier
  instructions to separate carryover and serialize checks are superseded.
- After the M3 reboot, builds and tests execute normally. Both the Inness
  export and replay easel built successfully.
- Flow: 12 passed, including split waits ([log](readiness/flow.log)).
  The final 2400px sienna card passed: 88.24% and 85.53% contrast retained
  against the 50% minimum ([log](readiness/card-recheck.log)).
- The connected 2400px paint/thinner/rag/carryover/refold/wait sequence
  passed, including close/reopen and exact live-versus-replay canvas and
  checkpoint bytes ([log](readiness/sequence.log), [image](readiness/sequence.png)).
- Full checks completed with two known differences: the unchanged rag
  hollow-lift assertion measures 89.934% against >90%, and the historical
  engine-3 rag baseline differs after the rag changes. These are recorded
  first-painting exceptions; tests and baseline remain unchanged.
  Cargo recheck: 276 passed, 1 failed, 36 ignored
  ([log](readiness/cargo-recheck.log)). The full wrapper hit two timeouts;
  the affected cargo batch and final card completed on targeted reruns.
  A generated Python cache caused the baseline-package check to fail;
  removing that cache restored the unchanged package check
  ([log](readiness/baseline-package-recheck.log)). Full results:
  [summary](readiness/all.json), [logs](readiness/all-logs/).
- Prompts reviewed against the current guide and bindings. Corrected old
  drying times in studio notes, removed repeated viewing-cadence steering
  and clarified Lua option types and pressure. The approved system prompt
  and studio rules remain. Runner checks: 255 passed after these edits
  ([log](readiness/prompts-recheck.log)).
- The detached `claude-paint-r24run` checkout is clean at `f75e27f`.
  This adds the approved simplified thickness note; its research evidence
  stays in the repository and the existing export strips it successfully.
  Engine code is unchanged from the tested candidate. The runner now
  rereads the sitting cap between sittings; all 258 runner checks pass
  ([log](readiness/sitting-cap-after.log)).
  The owner-approved completion condition is in the brief: inspect the
  whole painting and details, continue while an improvement is identified
  and finish when judged resolved. The rendered brief and gallery runner
  copy were verified after this change.
  Refreshed Inness export/probe and runner dry run passed; the copied
  runner uses the revised prompts ([log](readiness/launch-refresh.log)). The private
  viewer returns HTTP 200 at <http://m3p.tailec2dc.ts.net:8765/> from the M3;
  this round did not access the M1. Public sync is enabled. The owner
  explicitly requested live website publication of this engine-3 painting;
  the new studio stays included in the regular export.

Ready to kick off. Not done yet: tag `round-24` at the painting candidate
and start the painter. Launch remains held for the owner's go-ahead.
Raw verification logs are local evidence, not publication files.

## What is set

- `notes/round24/runner/r21_chains.py`: lane `INNS` uses
  `anthropic/claude-opus-5-5` with thinking `high`, and pi-black.
  `TAG = "round-24"` is the one pending value. `BRANCH = TAG`, and
  `BASE = ~/src/a/claude-paint-r24run`. `EXPORT`, `FINISH`, `CHECK`,
  `NAMES` and `strip_sources` resolve under `BASE/scripts/`, and
  `H = BASE/harness/painter`.
- Outside `--dry`, `main()` refuses to start unless `BASE`'s HEAD is
  `TAG`'s commit and has no changes to tracked files (`checkout_problems`).
  The export archives `TAG`, while the harness, the check and the finish
  run from `BASE`'s files. This check keeps them on the same commit.
- `notes/easel_guide.md`, "The rag": includes cross-wipe carryover and
  refolding. Guide review notes below preserve the original checklist.

## At freeze

The live sitting cap is the positive integer in
`~/tmp/gallery-fcf9c110/r24/run/max_sittings.txt` (currently 4).
The runner rereads it between sittings; the current sitting completes
normally. Missing or invalid contents retain the last valid cap (initially
4). This file stays outside the painter's studio and messages. Changing it
after the runner has stopped requires resuming the runner.

Steps 1 and 2 must use the same name. If the tag is not `round-24`, change
`TAG` in `notes/round24/runner/r21_chains.py` on the candidate before
tagging it: the runner copied in step 4 comes from the tag.

1. Tag the frozen commit in the shared repo:
   `git -C ~/src/a/claude-paint tag -a round-24 <commit> -m "round 24: engine-3 Inness painting"`
2. The detached checkout already exists. Confirm it is at the candidate
   selected by the tag. For a fresh checkout:
   `git -C ~/src/a/claude-paint worktree add --detach ~/src/a/claude-paint-r24run round-24`
3. Pre-build both easels under the lock. The runner's own builds then
   reuse them instead of compiling outside lockrun. Start sccache first
   with `SCCACHE_IDLE_TIMEOUT=0` (HANDOVER-2 "Health").
   ```
   cd ~/src/a/claude-paint-r24run
   R16_BRANCH=round-24 ~/src/a/claude-paint-tools/lockrun --timeout 1800 --owner r24-launch -- \
     scripts/export_r16_studio inness "$TMPDIR/r24-probe-studio"
   ~/src/a/claude-paint-tools/lockrun --timeout 1800 --owner r24-launch -- scripts/replay_easel
   ```
   The export builds the inness painter easel into
   `target/studio-build/code-<hash>/built/inness/easel`. It also runs the
   export's own checks: tube table, painter names, box tubes and the probe.
   `replay_easel` builds the replay easel that check and finish use, in
   `target/`. Read the "Guide statements to verify" below in
   `$TMPDIR/r24-probe-studio/notes/easel_guide.md`, then delete the probe
   studio.
4. Copy the runner into the gallery folder. The viewer labels runs from
   `~/tmp/gallery-*/r*/run/studios.json`:
   ```
   mkdir -p ~/tmp/gallery-fcf9c110/r24
   rsync -a --exclude run --exclude __pycache__ ~/src/a/claude-paint-r24run/notes/round24/runner/ ~/tmp/gallery-fcf9c110/r24/
   cd ~/tmp/gallery-fcf9c110/r24 && uv run r21_chains.py --dry
   ```
   `--dry` should print `--thinking high`, `R16_BRANCH=round-24` and
   paths under `claude-paint-r24run`.
5. Check the viewer:
   `curl -s -o /dev/null -w "%{http_code}\n" http://m3p.tailec2dc.ts.net:8765/`
   should print 200. If it doesn't, reload the LaunchAgent:
   ```
   launchctl bootout gui/$(id -u)/art.stillwet.studio.tailnet
   launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/art.stillwet.studio.tailnet.plist
   ```
6. Keep `art.stillwet.studio.sync` enabled for live publication at
   <https://stillwet.art/studio/>. It exports every minute through
   `~/src/a/stillwet/sync-studio.sh`; the new round-24 studio is included
   automatically. Preserve the three existing exclusions. The exporter's
   identity scrubbing and publication checks remain enabled. Once the
   painter starts, verify its entry and growing events on the public site.
7. Launch, the same way as round 23:
   ```
   cd ~/tmp/gallery-fcf9c110/r24 && mkdir -p run && \
     (nohup caffeinate -i uv run r21_chains.py --only INNS > run/runner.INNS.out 2>&1 < /dev/null & disown)
   ```
8. Confirm that the painter is using the easel. `run/chains.log` should
   show `INNS1: exporting inness studio from round-24`, then
   `easel "painting" open`, then `sitting 1 (claude-opus-5-5, ...)`. The
   chunk count should then grow:
   ```
   s=$(python3 -c 'import json;print(json.load(open("run/studios.json"))["INNS1"])')
   grep -c '^--@ chunk' ~/src/a/$s/paintings/lua/painting.lua
   ```
   The viewer URL for the painter is
   `http://m3p.tailec2dc.ts.net:8765/?p=<studio>`. The painter is listed
   under round r24, lane INNS.

## Viewer

- `art.stillwet.studio.tailnet` (LaunchAgent; templates and `install.sh`
  in `notes/launchd/`): `studio.py --host 100.82.115.34 --port 8765`,
  started from `~/src/a/claude-paint`. It is bound to the Tailscale
  address only, with no Serve route and no Funnel. Its log is
  `~/tmp/studio-viewer-logs/art.stillwet.studio.tailnet.log`.
- On 2026-10-04 it was not running: launchd reported `spawn failed`, last
  exit 78. The last log entry was an earlier `Can't assign requested
  address` (it bound before the Tailscale address existed). `kickstart -k`
  hung. Lane E reloaded it with the `bootout`/`bootstrap` in step 5, and it
  has run since then. `http://m3p.tailec2dc.ts.net:8765/` answered 200 on
  this machine and from the M1 Pro (`ssh m1p curl ...`). `/api/sessions`
  listed round 23's painter as `r23`/`INNS`.
- Existing Serve routes, unchanged (`tailscale serve status`, both tailnet
  only): `https://m3p.tailec2dc.ts.net` → `127.0.0.1:8766` (the
  `--public` viewer, `art.stillwet.studio.public`, also not running) and
  `:8443` → `localhost:5733`.
- `art.stillwet.studio.sync` runs every minute. It exports every painter
  not in its `--skip` list to the public stillwet `/studio/`
  (`~/src/a/stillwet/sync-studio.sh`), so the round-24 painter will be
  published there unless it is added to the list.

## Original guide review checklist

Reviewed against `b0b2393`: exchange remains off, thinner and drying
contracts match the guide, flow and carryover checks pass. The existing
blend figures describe the unchanged unthinned brush behavior; they were
not remeasured in this run. The following is the original lane checklist.

The rag statements were already wrong at `d54b423` and are now fixed:

- "a thin stain of the color always stays" was replaced. On engine 3 the
  last of the film comes away as `take × v/(v+h)`, with
  `h = SLOW_COATS × hollow / (1 + SLOW_DAMP × damp)` (`rag.rs`). The test
  `a_dry_rag_leaves_a_pale_tint_and_spirits_lift_nearly_to_the_ground`
  covers it.
- The light-pressed rim was added (`RIM_PRESS` 0.3, `RIM_EDGE` 0.85), and
  "the pad's soft edge" became "the pad's rim".
- Spirits "work gradually: wipe after wipe takes the paint nearly to the
  ground" was added (`DAMP_LIFT3`, `DAMP_REACH`, `DAMP_WICK`). This needs
  rechecking if Lane C changes the rate.

Already true at `d54b423`, but Lanes A, B or C may change them. Each one
needs checking against the frozen candidate:

| guide | statement | what may change it |
|---|---|---|
| Thinner, l.126-131 | one stroke lays "at most a set thickness of liquid ... however many hairs or how many times the stroke passes ... about 6 µm at thinner 0.5"; "what the stroke can't lay stays in the brush ... another stroke over the same spot lays more" | A: finite-supply exchange with no per-stroke ceiling (`STROKE_FILM_UM`) would make this wrong |
| Thinner, l.132-134 | the solvent leaves "in a thin film a few minutes, longer in a thicker one" | B: timing in `drying.rs`/`thinner.rs` (`TAU_MIN` 2, `TAU_DOUBLING_UM` 100) |
| Thinner, l.136-138 | the wet paint "levels and spreads a little" while solvent is there | B: replacing the 2 µm floor (`WET_FILM_UM`) with bounded slowing/retention |
| Thinner, l.145-148 | "Solvent-wet paint comes up on a brush or rag no more readily than the same paint without it"; the thickness limits are estimates | A: exchange pickup for thinned hairs; C: the rag's lift reads only cure (`reach_fluid`) today |
| Brushes, l.169-171 | "strokes from one load run dry, and a brush that has been through wet paint carries some of it" | A: brush depletion/pickup |
| Covering, l.203 | `blend`: "one pass takes up about 15% of a thin wet film, three passes about 40%" (a measured number) | A (pickup) and B (thin-film flow): re-measure |
| Rag, l.357-372 | tops before hollows, pale tint, rim, streaks, smear-back on the rim and trailing end, spirits gradual | C: path sampling, bounded pickup/return, refolding |
| Rag, l.374-379 | a loaded face lifts less; `refold`; `r.soaked` | C: dirty carryover and refolding |
| Time, l.490 | "Thinner paint dries sooner and thicker paint later: twice a stroke's thickness takes 1.6 times as long" | B, if it changes `drying.rs` |

The Inness export checks and replay build completed. The runner's real
(non-dry) start remains part of launch.
