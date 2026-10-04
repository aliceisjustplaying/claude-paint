# Overnight work: improve the thinner, check the rag and make tests faster

Work overnight on the paint program. I am going to sleep. Make ordinary choices yourself. If something needs my judgment, write it down and keep doing other useful work.

This is the only work plan to read. It replaces the separate thinner spec and test-speed brief.

There are two main jobs. Improve the thinner and check the rag. Also make the existing tests smaller and faster. Do both jobs. Keep the safety checks described below.

## Rules for tonight

Keep this version named **engine 3**. Round 24 has never made a painting. Do not start a painting tonight.

**Do not replay whole paintings. Do not run expensive replays at all.** This also applies to tests or speed measurements that run a painting behind the scenes. Do not save those expensive runs for later.

We still need fair comparisons. Use tiny test scenes and save their results before changing the code. Those results tell us whether our changes broke something.

Keep the old code, old paintings and saved files. Old paintings can keep using the old code. The new engine 3 does not have to produce the same results as engines 1 and 2. Do not silently open an old painting with the new code.

Do not change the main working copy or the live website. Work in separate copies and save your changes on separate Git branches. A branch is a named version of the code that can be reviewed before it is added to main. Do not add your work to main or create a new round tag tonight.

Keep the existing privacy checks. Do not publish the user's real name, private email or personal folder paths. Do not turn off Git hooks to get around a blocked push.

## Where to start

The main working copy is `~/src/a/claude-paint`. It is at commit `3379b9f`. The website is updated from it automatically. Leave it alone.

The current engine-3 code is in `~/src/a/claude-paint-r24`, on branch `round-24`, at commit `af49348`. This includes the latest drying, rag, failed-action undo and sienna fixes. Use this exact saved version as the starting point, not whatever the branch might point to later.

The thinner work is in `~/src/a/claude-paint-thinner2`, on branch `thinner2`, at commit `c692eae`. It started before the latest fixes. Bring `af49348` into this branch before writing the new tests. Keep the earlier diagnosis and any existing work. Do not erase changes or rewrite Git history. If there are unexpected changes or conflicts, record them before going further.

Keep `~/src/a/claude-paint-r23` and the earlier painting-runner copies. The saved round-23 version, `7b80cb0`, supports old paintings. Do not delete or rebuild it for this job.

The exact starting commit IDs are:

- Current engine 3: `af49348239421791509f7b7c36e9dc39305b26fe`.
- Thinner diagnosis: `c692eae897ae7c206fa9d28e38649062326acb3a`.
- Old painting code: `7b80cb0a112e2ba5273bae952c7cf874a06ee51a`.

Read `~/.pi/agent/AGENTS.md` for the machine's working rules. Earlier reports said 276 tests passed before the thinner was added. That does not prove the new thinner works. No earlier document adds work or changes the rules in this plan.

## Divide the work

You are the lead agent in Pi. Use one agent for the thinner and another for faster tests and the supporting tools. Give each its own working copy. Check that another copy of the same job is not already running before starting it.

Use `anthropic/claude-opus-5-5` with high thinking for builders. Use `openai-codex/gpt-6-astra` with medium thinking for the separate reviewer. Check that these models and the needed tools are available. Report missing tools or models rather than quietly replacing them.

Continue the existing thinner session. Its session-file path is in `~/src/a/claude-paint-resume-2026-10-03.txt`. Use the path only: the old instructions in that file are out of date. Make sure the session is not already running.

Start the test-speed agent from `af49348` in its own working copy on branch `codex/speed`. Make sure the whole repository is available in that copy. If the branch or working copy already exists, inspect it and continue safely. Do not reset it blindly.

## First, save small before-change results

The test-speed agent does this first so the thinner agent has a fair comparison.

Use a few tiny engine-3 scenes: a stroke, a short body pass, overlapping paint, brush pickup, rag actions and waiting. Use 64- or 128-pixel canvases where enough. Do not use a whole painting, long loops or a detailed texture study.

Run them on unchanged `af49348`. Save the inputs, paint-state values and results. Keep enough information to compare the paint's actual state, not just the final picture. Record the code version, compiler version, build settings, canvas size, exact commands, file hashes and times. Use release builds for saved comparisons; the faster iter build can change the last few digits and pixels. Save the small package in `notes/thinner/baseline/`. If you need extra code to read the results, prove on the tiny cases that it does not change them.

The thinner agent must compare against these saved results. It must not create its own expected answers from the code it is trying to test.

Save this small set of tests and results in a separate commit. Review how it was made, then bring that exact commit into the thinner branch. The thinner agent may write tests while waiting, but must not start building the thinner until this comparison is ready and its tests have been approved.

## Improve the thinner

Thinner must let the painter lay a thinner, more see-through layer of paint. The solvent must disappear over painting time while the paint remains. Adding thinner must not secretly change the pigment's own color or strength.

Using no thinner, or setting it to zero, must keep the current engine-3 paint behavior unchanged in the small comparison cases. New saved files may have a new format to hold the solvent. There is no need to convert old experimental engine-3 saves.

Keep this exact target:

- Use raw sienna with thinner set to `0.5`.
- Paint one `body` pass over a black-and-white card.
- Test brush load `0.3` and brush load `0.6` separately.
- Use a canvas width of **2400 pixels**.
- After the solvent has gone, **at least half the original difference between black and white must still show**, at both loads.
- Wait at least ten times the model's evaporation time. Also check that less than one-thousandth of the added solvent remains. Both checks are required.

Do not lower that target or shrink that test. Use smaller tests while editing. Run the exact card test when the thinner first works and when needed to confirm the finished result. Reuse a valid result if nothing affecting it has changed.

Check that the brush cannot lay unlimited paint on one spot just because a stroke has more points or more overlapping bristles. Paint that was not laid must stay on the brush. More pressure and overlapping strokes must still behave sensibly. An emptying brush must lay less paint.

Check that paint and solvent are accounted for when brushes, rags and spreading paint move them. Solvent must not create color or change the paint's own drying rate. More thinner must produce a smooth change, not a sudden jump.

Saving and reopening must keep the same state, including solvent and time. A failed action must undo all its changes. Splitting a wait into smaller waits must pass the timing checks listed below. Running with different thread counts must give the same result.

### How the thinner should behave

The painter sets `thinner` on a paint pile, from 0 to 0.9. It is the share of solvent by volume. It is different from added oil, called `medium`: oil stays in the film, while solvent leaves. Keep pigment values unchanged.

For thinned paint, give each canvas pixel a limit on how much wet paint one stroke can add. All bristles and parts of that stroke share the same limit. A separate stroke gets a new limit. Paint that cannot be laid stays on the brush. The limit should fade away smoothly as thinner approaches zero. At zero, use the existing paint behavior unchanged.

Track solvent separately from paint at each pixel. Solvent makes wet paint flow more, then the paint firms up as solvent leaves. Keep the paint's own drying rules based on paint thickness and its original stiffness, not the temporary softness caused by solvent.

Use a smooth exponential loss of solvent: the same fraction leaves in each equal time step at a fixed paint thickness. In code, the rule is `solvent = starting_solvent * exp(-time / tau)`. Here `tau` is the evaporation time for that paint thickness. Thicker paint takes longer. The exact thickness limit and evaporation rate are estimates. Label them as estimates and show measurements across several thinner settings, not just the passing setting. Cite real sources for physical claims; do not present a chosen value as a measurement from a source.

Keep solvent loss, spreading and drying on a shared clock with one-minute steps measured from the canvas's start time. Do not restart the clock at each wait command. Brushwork advances the same clock. Preserve the old calculation path when there is no solvent, so unthinned paint stays unchanged.

When a brush or rag takes paint, it also takes solvent in the same ratio as the paint at that spot. Spreading paint moves both together. Making solvent-wet paint easier to pick up is optional for this version; say whether you included it. Moving the solvent with paint is required. The rag's own dampness remains separate.

Do not add solvent loss from the palette or brush, solvent soaking into the ground or solvent dissolving already-dry paint in this version. State these limits in the guide and report.

Use the new save format, `PAINTCK9`, for engine 3. Save the solvent and all clock state needed to continue exactly. The new program must refuse incompatible old logs or saves before painting and tell the user to use the preserved old version. Check this with tiny sample files. Do not build an old-save conversion system or delete all old code.

Update the painter's guide to explain thinner and its limits in plain words. Show thinned piles as thinned in the palette view. Do not tell the painter when to use the feature.

### The 19 thinner checks

Write a named test for each check below. If a check says a small difference is allowed, choose and explain that limit before building. Both reviewers must approve it. Do not change the fixed card target or the timing limit below.

1. **The card:** pass the exact two-load, 2400-pixel, 50% target above after both evaporation checks pass.
2. **No thinner:** leaving thinner out or setting it to zero gives exactly the same existing paint-state values as the small saved tests from af49348. New solvent values are zero. The new save header and added fields may differ; existing paint values may not.
3. **Old files:** the new program refuses an incompatible old log or PAINTCK8 save early and names the old version to use. Tiny headers are enough to test this.
4. **Nothing vanishes:** account for all paint and solvent before and after a stroke, including pickup. Any solvent missing from the brush and canvas must have evaporated or left with the rag. State the allowed rounding error.
5. **The brush runs out:** it lays less paint as it empties. At the same thickness limit, load 0.6 lasts farther than load 0.3.
6. **Pressure:** more pressure lays more paint, up to the stroke's limit.
7. **Overlap:** two overlapping passes leave more paint than one.
8. **Smooth changes:** thinner 0.01 makes only a small change from zero. Increasing thinner through 0, 0.1, 0.2 and so on to 0.9 must not make the card less visible.
9. **Evaporation:** solvent falls over painting time toward zero. The wet film loses the volume of solvent that leaves. Solvent itself adds no color or hiding power.
10. **Wet handling and drying:** paint with solvent spreads more than after the solvent leaves. At equal remaining paint thickness, compare when thinned and unthinned paint gel and become touch-dry. They must agree within the approved limit. Test extra pickup strength if included; otherwise report that it is omitted.
11. **Save and reopen:** keep all paint, solvent and time state. Check halfway through evaporation and between whole-minute steps. Continuing from a save must match continuing without closing. Use a tiny new log to check saved state against replayed state.
12. **Repeatable results:** the same tiny log gives exactly the same saved bytes with one worker thread and four.
13. **Pigment ordering:** leave pigment values unchanged. If a proposed change affects hiding or color, stop for review; the existing rule requires burnt sienna to be more transparent than raw sienna. Do not change pigments to force the card test to pass.
14. **Stroke points:** giving the same path twice or ten times as many points must not meaningfully change paint at each pixel. Overlapping bristles in one stroke must still share the same limit.
15. **Drying inputs:** calculate oil drying from the paint's own stiffness and solvent-free thickness. Temporary softness from solvent must not slow oil drying by itself.
16. **Moving paint:** brush pickup, rag lifting and spreading must carry solvent in the local paint-to-solvent ratio.
17. **Waits:** with one-minute clock steps, waiting 15 minutes once must exactly match waiting one minute 15 times. Waiting 7.3 minutes and then 7.7 minutes must differ from a single 15-minute wait by no more than 0.01% in solvent, thickness and cure. Define how zero values are compared before approval.
18. **Undo after failure:** a failed action after a thinned stroke restores solvent, paint, clock state and any temporary stroke limits. The next action must see the same state as if the failed action never happened.
19. **No solvent color:** two films with the same paint in the same places but different amounts of solvent must render identically. Solvent may change the picture by moving paint; it must not add an optical effect of its own.

## Check whether the rag actually works

Engine 3 has never painted a painting. Number checks alone are not enough to judge the rag.

Make one small test picture with simple swatches, no detailed scene. Show before and after views of:

- Wiping normal wet paint with a dry cloth.
- Blotting normal wet paint.
- Wiping with a solvent-damp cloth.
- The same actions on thinned paint.
- A dry-paint control.
- A brush stroke over a wiped area.

Keep each panel at most 256 pixels wide and the whole study within the short-test time limit. Reuse images from other tests when possible.

Look at the actual picture. Check that paint was lifted where expected, the ground shows through, edges make sense and blotting looks different from a directional wipe. Check whether painting over the wiped area works sensibly.

Measure how much paint was removed and how much color remains. An earlier report found a stain of about 14% of the tone's color. That is an unresolved limitation, not proof that the rag can wipe back to the ground.

Have the separate reviewer inspect the picture too. Fix clear defects. Write down appearance questions that need my judgment. Do not claim a small study proves how a whole painting will look.

## Review the tests before building

The thinner agent first writes the tests, their small inputs and the commands that run them. Include all 19 thinner checks and the rag study. List each check, what it tests, how to run it and what result is expected.

Put the tests and their measuring helpers in separate files where possible. This lets the implementation change without changing the tests. Record file hashes so the reviewer can see whether approved tests or saved answers changed later.

Save the check list, test names, exact commands and chosen error limits in `notes/thinner/ACCEPTANCE.md`. Save final results in `notes/thinner/RESULTS.md`. Include the rag study in the approved list too. Missing code may make a test fail to compile at first; report that honestly and do not comment out its body.

The full-size target test must have Rust's `#[ignore = "slow"]` marker, but the required test command must run it explicitly. A skipped test is not a passed test. A command that finds no matching tests must fail.

Provide `scripts/test_thinner_acceptance --quick` for ordinary edits and `scripts/test_thinner_acceptance --all` for all agreed thinner checks. These commands must work without unfinished tools from the other branch.

You and the separate reviewer must read the tests and their helpers. Check that every promised behavior is covered and that the saved before-change answers came from the right code. Have the reviewer use a separate copy of the exact tests commit. Record both reviews, the tests commit, this instruction file's hash, the saved-results commit and the hashes of approved test files. The separate reviewer may ask other review agents for help. Neither reviewer edits the builder's files.

Only then may the thinner agent implement the feature. Missing tests or wrong test commands can be corrected and reviewed again. A change to an agreed physical target needs my decision. Record that question and continue other work.

Once approved, keep the tests, helpers and saved answers fixed. Fix the program to pass them. Do not hide a changed test rule inside production code. Any needed change to an approved test goes back for review.

## Make the existing tests smaller and faster

This is required work. Do not replace it with simply skipping slow tests.

Read the tests and any existing timing records. Find oversized canvases, needless brush strokes, repeated setup and duplicate checks. Replace big scenes with the smallest examples that still catch the same bugs.

For each changed test, explain what failure it protects against and why the smaller test still catches it. Keep separate checks when they protect against different failures. Share setup only when doing so does not make one test depend on another.

Do not weaken assertions or invent easier expected answers. Remove a duplicate test only when another named test still checks the same behavior. Have another agent review the test changes.

For a whole-painting test, find a small example of the specific bug it was meant to catch. If no small replacement is ready, report the missing coverage. Do not run the expensive painting or pretend the replacement has been proved.

Keep small full-resolution checks for laying paint, solvent evaporation, rag lifting and drying. Use just a few controlled strokes. The fixed 2400-pixel thinner card is not allowed to shrink.

The test-speed agent changes existing tests. It must not change the thinner agent's newly approved tests. Keep its test changes separate from the small before-change results so each can be reviewed and shared on its own.

## Keep the full test plan visible

Make `scripts/test` run the normal fast checks, aiming for about a minute once the code is built. Make `scripts/test --all` run the remaining required, time-limited checks only after its list has been reviewed. Neither command may replay whole paintings.

List every skipped test, its name, why it is skipped and whether it checks correctness or is only an experiment. Use existing timing records or say the time is unknown. Keep ordinary diagnostic experiments as plain ignored tests. Mark useful longer correctness checks `#[ignore = "slow"]` and run them by exact name. Do not assume Cargo will select them from the reason text automatically. Useful longer checks must be named and run explicitly when required. Do not run all ignored experiments by accident.

Include relevant checks for the command-line scripts and supported build settings, including builds without the default features. Read these checks first to make sure they do not secretly replay paintings.

Keep the previously reported `replay_env.sh` failure visible until it is understood with a small example. Do not quietly remove it or call it passed.

Record what each command ran, how many tests ran, failures and times. Measure build time separately. Do not run an old expensive test just to get a before-time for your report.

Write `notes/agent_brief_template.md` for future builders. Explain the fast command, the saved comparison results, required final checks, the one-heavy-job rule and the ban on expensive replays. Link it from the repository's working instructions.

## Stop jobs from fighting over the machine

Allow only one heavy build or test job at a time across all agents and working copies. Until the tool below works, you assign each job its turn. Write down who owns the current turn. Give the next job a turn only after the old job and its child processes have stopped.

Build `scripts/lockrun` to enforce this on macOS and Linux. Use Python's standard `fcntl.flock` on macOS and Linux, with one shared lock file per user in the system temp folder. Do not erase the previous job record while trying to get the lock. Run each command in its own process group so stopping it also stops its children. Record the owner, command, group, unique job token and start time before work can begin. Use a pipe so the child waits until that record is safely written. If the helper dies before giving permission, the child must exit without starting work.

Check that the tool handles normal completion, cancellation, timeouts and crashes. If the helper crashes but its child job is alive, a second job must not start. Do not kill unrelated processes. Use tiny dummy jobs to test all of this, not painting renders.

Use one outer lock for a batch. Inner commands must not try to take that same lock again and get stuck waiting for themselves.

Use these limits:

- Ordinary checks: one minute once built.
- One exact thinner card test: five minutes.
- Initial builds and final check batches: ten minutes each.

Stop the whole job when its time runs out. Save its logs and mark it unfinished. Investigate with small cases. Do not quietly extend the limit, weaken the target or keep retrying the same slow command.

Start long commands through `job_run` so you can respond when they finish. Do not sit in sleep or polling loops.

## Build the checks that protect main

Build these safeguards and test them with tiny dummy Git projects. Do not use them to change the real main version tonight.

Implement `scripts/test --all --candidate <commit>` to test an exact saved code version in a fresh, clean working copy. Remove the temporary working copy and its build files afterward. It must refuse the wrong version or extra uncommitted changes. Run only the reviewed list of small, time-limited checks. A missing check or a timeout cannot produce a pass.

Save proof of the result outside the tested commit, so recording the result does not change the code's identity. Record the commit, file-tree hash, main version it was based on, build settings, test results and log hashes. Git notes under `refs/notes/test-receipts` can hold this proof. Fetch and push those notes explicitly; a normal branch push does not include them.

Protect the saved test answers too. Later changes to expected answers, test inputs or the code that measures them need separate approval tied to the exact changes. The builder cannot approve its own answer changes. You may verify the first small comparison from the fixed starting code; later answer changes need my approval. Use `refs/notes/golden-approvals` for the record. Passing tests is not permission to change their answers.

Build `scripts/merge_candidate` to check this proof before adding a version to main. It must confirm that the exact code passed, approved tests are unchanged and any answer changes have the required approval.

It must also check that main has not moved since the tested version was prepared. Check both the remote main and the published local working copy. Refuse uncommitted changes that could be overwritten. The remote update must fail if main changes between checking and updating it. First confirm that the proposed version includes the recorded main commit in its history. The guarded update is `git push --force-with-lease=main:<recorded-main-commit> origin <tested-commit>:main`. Together, these checks prevent rewriting history or overwriting a changed main. Test this on dummy repositories only tonight.

After a successful remote update, update the local files too. Do not merely move a Git reference and leave the website's files behind. If the remote update succeeds but the local update fails, stop and explain recovery. Do not overwrite local work.

Test missing proof, failed tests, proof for another version, changed main, changed approved tests, unapproved answer changes and dirty working copies. Have another agent review the safeguards. Keep all privacy checks and Git hooks enabled.

## Keep enough notes to recover after a crash

Keep `~/src/a/claude-paint-overnight-log.md`. Add a short entry for each completed step, approval or failure. Record time, agent and session, code version, instructions used, commands, results, saved reports and next action. Keep earlier entries.

Before starting workers, check that a harmless completed job can wake you. Record incoming completion messages and check the files or results they claim. Workers should save their reports before announcing they are done.

Do not wait for a `.jsonl.exit` file to stay on disk. The current tool deletes that file after reading it. A missing file does not mean work is still running. A commit or a completion message alone does not prove tests passed.

If you crash, the workers may keep running, but approvals will not advance by themselves. The next lead agent must read the notes, session results, live jobs and Git state before continuing. Do not start duplicate jobs.

If a message is not delivered, use the identified herdr pane and check that the worker acknowledges it. Seeing the typed text on screen is not enough.

On a tool or worker crash, check what already finished before retrying. Retry the interrupted step at most twice. Do not treat a real test failure as a reason to repeat the same run. Fix the cause or report it.

## Finish with an honest report

Have another agent check the finished thinner code, smaller tests, job controls, merge safeguards and rag picture. Reuse the saved images for review. Do not rerender them just for reassurance.

Combine reviewed work only in a separate feature working copy. Run the affected short checks on the exact combined version. Keep main and the website unchanged.

Save and push feature-branch changes after the normal privacy checks. A pushed branch is not proof the work passed. Report passed, failed, timed-out and unfinished checks separately. Do not call both jobs complete just because one finished.

In the morning, show me:

- The exact branches and code versions containing the work.
- What works and what still fails.
- Both full-size card results, waiting times and remaining-solvent measurements.
- A few small images, including the rag study and any visible defects. Reuse test outputs for a simple sheet showing raw sienna and lead white at loads 0.1, 0.3 and 0.6, and thinned paint at 0, 2, 5 and 15 minutes. Keep the actual post-evaporation target measurement even if it needs longer than the pictured 15 minutes.
- Which tests became smaller, which bugs they still catch and their new times.
- The skipped-test list, script/build checks and any missing coverage.
- Results for the small full-resolution checks.
- Whether the job controls, saved-answer protection and merge safeguards passed their checks.
- Anything unfinished or needing my decision.

No painting launch, round tag, update to main or website change tonight. No expensive replay now or later.

## Use this file as the single plan

Everything required for tonight is above. The former thinner spec and test-speed brief are retired. Do not read them as extra instructions. Record the hash of this file at the start so both reviewers know which plan was used.

Before reporting completion, check that both jobs, all 19 thinner checks, the rag picture, the small full-resolution checks, the skipped-test list, script/build checks, future-agent instructions, job controls and merge safeguards have a result or a clearly stated unfinished status. Do not silently drop a job.
