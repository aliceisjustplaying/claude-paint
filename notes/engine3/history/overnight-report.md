# Overnight report: engine 3 with the thinner and faster tests (2026-10-04)

Plan: `~/src/a/claude-paint-overnight-plan.md` (sha256 `382fc2de…8e30`) plus your chat decisions (`claude-paint-overnight-decisions.md`). Full step log: `~/src/a/claude-paint-overnight-log.md`. Reviews: `~/src/a/claude-paint-reviews/`.

No painting, no whole-painting replay, no round tag. `main` and the website are unchanged: local and remote `main` are still `3379b9f`.

## Branches (all pushed to origin)

| branch | head | what |
|---|---|---|
| `engine3-overnight` | `d6318d1` | **the combined result**: engine 3 (af49348) + thinner + faster tests + safeguards. Tested code commit: `0abf792` |
| `thinner2` | `995fbab` | thinner work alone (tests frozen at `48be56a`) |
| `codex/speed` | `2b85ff6` | faster tests, lockrun, safeguards (before the combine) |
| `codex/safeguards` | helper branch, merged into `codex/speed` |
| `refs/notes/test-receipts`, `refs/notes/golden-approvals` | pushed | test receipts and saved-answer approvals |

Working copies: `~/src/a/claude-paint-engine3`, `-thinner2`, `-speed`. Approved tools: `~/src/a/claude-paint-tools/{lockrun,gate}` (`gate-prev` = the earlier copy).

## Bottom line

- `scripts/test --all --candidate 0abf792`: **NOT ALL GREEN, only because of check 13(b)**, the known pre-existing sienna-order failure that waits for you. All 21 other check steps passed (785 counted tests); build phase 149 s, check phase 348 s, each under its 600 s limit. Receipt: `git notes --ref=test-receipts show 0abf792`.
- Saved-answer check: `gate/golden_approve check --candidate 0abf792 --base 3379b9f` → `golden check passed` (147 protected changes, all approved).
- **Can you paint with it?** Engine 3 plus the thinner passes everything except 13(b), but the gate won't merge it until you decide 13(b). Look at the appearance questions below before a painting uses thinner.

## Thinner

- **Card (2400 px, raw sienna, thinner 0.5, one body pass):** load 0.3 kept **84.4%** of the contrast, load 0.6 **81.4%** (target ≥ 50%). Waited 35 min, ≥ 10 τ (τ = 3.328 min after the pass and at measurement). Solvent left: 7.0e-7 of what was there after the pass (< 1e-3). Unthinned, the same card keeps 13% / 6%. Source: `notes/thinner/RESULTS.md` "The card", log `notes/thinner/logs/all_final.txt`.
- **Sweep 0 to 0.9** (480 px card), contrast kept, load 0.3: 19, 26, 48, 64, 74, 82, 89, 94, 97, 99%. Load 0.6: 5, 15, 35, 53, 68, 78, 86, 92, 97, 99%.
- **Acceptance:** `scripts/test_thinner_acceptance --all` → 27/27 required pass, exit 3 = only 13(b). Check 2 (no thinner = unchanged af49348 state and pictures): pass. Old-log replays: 11/11 as af49348.
- **Tests:** written first, reviewed by me and astra over 5 rounds, frozen at `48be56a`. Six tests changed after the code existed (3 test bugs, 3 setups); every change was re-reviewed. No assertion and no limit changed, and the card test was never touched.
- **Estimates (labeled, no measured source):** stroke ceiling 6 µm at thinner 0.5; τ = 2 min × (1 + paint/100 µm) (the 20 → 100 µm change is disclosed: it was made after a card run); flow 0.06 mm²/min; 2 µm wetting film. Sources cited (checked by the reviewer): Jennings 1902 (turpentine on paper, sets scale only), Hansen 1967, Orchard 1963, Landau and Levich 1942.
- **Left out by design (stated in the guide):** no solvent loss from palette or brush, no soaking into the ground, no dissolving dry paint, no extra pickup of solvent-wet paint.

## Rag study (`notes/thinner/rag_study.jpg`)

Wipes lift along their path, the blot is a small round lift, the damp wipe clears closest to the ground, a stroke over the wipe lays paint and the dry control doesn't change. Two reviews agree it works sensibly. Numbers: dry wipe on wet paint lifts 90.5% and leaves 47.6% of the tone. Damp wipe lifts 97.2% and leaves **20.8%** (the earlier report said ~14%; unresolved: one pass can't wipe back to the ground).

## Faster tests

- Rust suite: 102.5 s → 42–46 s. `scripts/test` (fast): about 1 minute of checks once built, about 2.5 minutes with builds.
- Biggest cuts (same assertions, each mutation-checked): smoke 58.8 → 7.7 s, delivery 19.7 → 7.0 s, determinism (hand time / digests / rag) >60 / 8.0 / 38.6 → 6.7 / 0.6 / 2.9 s (2400 px versions kept in `--all`), rag gel-point >60 → 9.6 s. Full table: `notes/speed/FINAL_REPORT.md`.
- Seven tests that replayed whole paintings never run now. They're replaced by 11 tiny old-log cases checked against unchanged af49348 (astra reproduced every answer from a fresh build). **Missing coverage:** the later chunks of those logs.
- Bugs fixed: `replay_env.sh` (failing since round 23: clip hold limit), the painter build's unit tests (didn't compile), a `session_integrity` socket race under load.
- Skipped-test list: `notes/speed/SKIPPED.md`. Script and build-setting checks (`--no-default-features`, every box, replay without finishing verbs, the studio and runner tests) all pass in `--all`.

## Job controls and safeguards

- `lockrun`: 17/17 checks; every heavy job tonight ran through it.
- Runner process handling: 17/17. Gate (`test_candidate`, `merge_candidate`, `golden_approve`): 140/140 on dummy repos; astra deliberately broke 12 checks one at a time and 11 were caught (the 12th can't happen). Four review rounds; round 1 rejected the tools, and the fixes are in.
- **Never run against the real main.** Merging needs a pass, so the gate refuses this branch until 13(b) is settled.

## Needs your decision

1. **Check 13(b):** at equal film over the card, raw sienna shows 17.91% of the contrast and burnt 9.85%, the reverse of Field/Salter. The failure predates the thinner (it happens on unchanged code). Fix the tube values, restate the check, or accept? Nothing merges until then.
2. **Thinned paint's look:** (a) every thinned stroke has a darker rim at its free edge that doesn't level (`single_stroke_edge.jpg`), so washes read as separate flakes (`weave_or_brush.jpg`); (b) fine grain; (c) the 0/2/5/15 min row looks identical (slow flow). Changing it means changing estimates.
3. **Brush capacity:** a thinned brush keeps what it can't lay. It lasts about 2 canvas widths at load 0.3 and 5 at load 0.6, against about 1 unthinned. Is that right?
4. **Damp ≈ dry rag on thinned paint** (68.3% vs 68.1% lifted).
5. **Old files:** engine-1/2 logs replay unchanged; PAINTCK8 engine-3 saves are refused, naming af49348. Old engine-3 logs that probe the new `thinner` key could branch differently (there's no revision marker).
6. **Check 8's 0.005 allowance** (reviewer-approved noise allowance).
7. **`thin_blend_bares_ground`** doesn't catch the crest bug its comment names; tightening changes an approved answer.

## Not done or known gaps

- Linux untested (lockrun and runner).
- The fail-without-fix runs for the thinner's 3 newest unit tests are described, not logged.
- Receipts in `refs/notes/test-receipts` contain macOS temp paths (`/var/folders/...`). No name, email or home path; the privacy scan found 0 hits for the name in branches and notes.
- `notes/speed/FINAL_REPORT.md` says "10 approvals waiting" and "gate-next not promoted". Both happened after it was written: approvals recorded at `0abf792`, gate promoted.
