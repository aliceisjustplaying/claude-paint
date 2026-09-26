# Clean the painters' folder (r15-base)

Worktree: ~/src/a/claude-paint-r15-base (branch r15-base). This folder is
what AI painters get to paint in. An adversarial review found what in it
would steer them: ~/tmp/paint-r6-b943b1ca/briefs/r15_review_answer.md
(read it all; findings are numbered). Fix the findings listed below. Commit
in small commits; do not push. Scratch: run `~/.local/bin/agent-tmp r15clean`
once and export TMPDIR=TMP=TEMP. Wrap cargo in `timeout`.

## Do
1. Finding 1: remove the subject generators from this folder: the `rock`
   module, the `atmos` module and `form`'s `Ridge` (and anything that exists
   only to serve them: exports, tests, guide sections, examples). If a
   remaining module depends on them, cut the dependency if it's small;
   otherwise report it and leave it.
2. Findings 2 and 19: remove every study program that builds a scene or a
   motif (sky, stipple, tip, edges, brushes, aging, cracks, color, strokes,
   surface, ground: judge each by the review). Keep only studies that are
   abstract material comparisons with neutral labels, and clean their
   labels (no old/new, no verdicts, no artists). If none survive, that's fine.
3. Finding 9: notes/guide.md's minimal program and run.rs's doc example:
   a blank canvas or a neutral swatch, ending with `o.end`/`o.save`, no
   `Finish::aged`. Finishing is demonstrated separately as an option.
4. Finding 13: paintings/src/run.rs: the default width with `--full` is
   2400 (keep other behavior); make its module docs and every command in
   README.md and the guide consistent with 2400 (`--full --width 2400` is
   still fine). No "1000px preview" anywhere.
5. Findings 15, 16, 17, 18, 21, 23, 24 and the comment parts of 7 and 8:
   COMMENTS AND DOCS ONLY. Remove history (rounds, reviews, labs, fixes,
   branches, "old/new", "used to"), people, earlier paintings and painters,
   subjective verdicts ("reads as", "digital", "salt", "the cure"), advice
   that ties tools, pigments or brushes to subjects ("for a sky", "meadows,
   foliage", "twigs, rigging, grasses"), claims about what painters do or
   should do, and references to files that don't exist. Replace with
   present-tense physical and API descriptions (what it computes, its
   parameters, its units). Fix the stale or wrong docs in finding 23 and
   24. Keep citations to notes/research/*.md that exist. Scene-shaped test
   fixtures (finding 9, 7's sheep): rename and relabel them neutrally; do
   not change their numbers or assertions.
6. Finding 16: paintings/src/lib.rs header; the dead reference in
   notes/research/friedrich_materials.md line 3.
7. Finding 3, one part only: delete the "No green data found for ..." list
   of paintings (friedrich_materials.md around line 115). Leave the rest of
   the research notes as they are.
8. Update README.md and notes/guide.md for everything removed.

## Do NOT
- Change engine behavior. Except for removing whole generator modules
  (item 1) and the runner's default width (item 4), no code changes: no
  defaults, no algorithms. Findings 5, 7 and 8 (inverse color search,
  gesture planning in presets, stipple defaults) are decisions for Alice:
  leave the code as is.
- Add any warning, list of removed things or explanation of what the
  folder leaves out, anywhere.
- Touch the research notes beyond items 6 and 7.

## Check before finishing
- `cargo build --release -p paintings` and `cargo test -p paint` pass
  (tests of removed modules go with them; say which).
- grep the whole folder (excluding target/ and .git/) for: Alice, owner,
  round, review, lab, loop, amnesia, fresh, coast, winter #, mountains #,
  moonrise, monk2, thermos, easel, lua, digital, "reads as", "old way",
  Cézanne, sheep, oak, lime, "for a sky", notes/ paths; show what remains
  and why each is fine.
Final reply: what you removed, what you changed, the grep results, and
anything you left for Alice.
