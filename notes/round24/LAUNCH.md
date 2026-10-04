# Round 24 launch: the Inness painter on the frozen engine-3 candidate

Prepared on lane E (branch `e3/e-painter`, from `d54b423`). The runner,
guide and viewer are ready except for the final revision, which is still
pending. The integration owner runs these steps at freeze. Nothing here has
run yet: no tag, no `claude-paint-r24run` checkout and no painter launch.

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
- `notes/easel_guide.md`, "The rag": updated to match the selected rag
  (`6f31f79`). See "Guide statements to verify" below.

## At freeze

Steps 1 and 2 must use the same name. If the tag is not `round-24`, change
`TAG` in `notes/round24/runner/r21_chains.py` on the candidate before
tagging it: the runner copied in step 4 comes from the tag.

1. Tag the frozen commit in the shared repo:
   `git -C ~/src/a/claude-paint tag -a round-24 <commit> -m "round 24: engine-3 Inness painting"`
2. Create the detached checkout:
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
6. Launch, the same way as round 23:
   ```
   cd ~/tmp/gallery-fcf9c110/r24 && mkdir -p run && \
     (nohup caffeinate -i uv run r21_chains.py --only INNS > run/runner.INNS.out 2>&1 < /dev/null & disown)
   ```
7. Confirm that the painter is using the easel. `run/chains.log` should
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

## Guide statements to verify on the final build

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

Not verifiable until the final build: the export of the inness studio
(build, tube table, names and box-tube checks), the replay build used by
check/finish, the guide's numbers above against the candidate, and the
runner's real (non-dry) start against the tag's checkout.
