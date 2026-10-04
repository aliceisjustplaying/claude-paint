# Round 24: round 23's Inness lane on engine 3 (rag, realistic drying, no rebuilds after a failed chunk)

A copy of round 23's runner (its tracked files, not its `run/` data). Lane `INNS` as before: one Claude Opus 5.5 painter in Inness's studio, `MAX_SITTINGS = 4`, `record_kind="none"`.
- Painter-facing: engine 3 (crates/paint/src/lib.rs): the rag (`rag()`, notes/easel_guide.md "The rag"); oil paint open and drying on the sources' clock (lead white workable ~13 h, touch-dry ~2 days; burnt sienna before raw sienna, the historical order); a failed chunk is put back without a rebuild from the log.
- The brief's undo line now reads "paint over it, or lift wet paint with a rag or a brush" (was "lift wet paint off with a brush"; plan-edition-2.md D9).
- The guide: blending lifts paint as well as moving it (measured: ~15% of a thin wet film in one pass, ~40% in three); Lua: no `math.atan2` (use `math.atan(y, x)`), `%d` needs an integer, lists and tables use braces.
- The painter: `anthropic/claude-opus-5-5`, thinking high (was xhigh), pi-black.
- `TAG = "round-24"` (PENDING: the integration owner names the tag on the final engine-3 commit at freeze), `BRANCH = TAG`, `BASE` = its detached checkout at ~/src/a/claude-paint-r24run, `H` = `BASE`'s harness. `main()` won't start (except `--dry`) unless `BASE`'s HEAD is the tag's commit with no changes to tracked files (`checkout_problems`).
- The guide's "The rag" follows the selected rag (`6f31f79`): no stain that always stays (the last of a film comes away more and more slowly; a dry rag leaves a pale tint), the light-pressed rim, spirits working gradually.
- Runs from ~/tmp/gallery-fcf9c110/r24/ (a copy of this folder from the tag's checkout), its data in `run/` there; the studio viewer finds its `run/studios.json` there.

# Round 23: round 22.1's Inness lane, with the palette look and the fast finish

A copy of round 22.1's runner (its tracked files, not its `run/` data). Lane `INNS` as in round 22.1: one
Claude Opus 5.5 painter (`anthropic/claude-opus-5-5`, thinking xhigh, pi-black) in Inness's studio, the
plain inness brief, `MAX_SITTINGS = 4`, `record_kind="none"`.

- `BRANCH = "round-23"`, `BASE` and `H` the `claude-paint-r23` checkout of that tag: code, harness,
  guide and scripts from brief 1 (branch `brief1`).
- The only painter-facing change: the `look` tool takes `palette: true` (a row for each pile a global
  holds: laid thick, one thin and one very thin coat over the canvas's ground color, the thin coat over a
  black and white card), and the guide's "Looking" says so. It reads only: no hand time, nothing in the
  log.
- Not painter-facing: the session writes `out/easel/painting/live.ckpt` when it closes; `finish_painting`
  finishes from it (`easel finish`, seconds) and compresses it to `live.ckpt.zst`; `check_painting`
  replays once (no `easel check`) and compares the replay's save with it byte for byte.
- Runs from `~/tmp/gallery-fcf9c110/r23/` (a copy of this folder), its data in `run/` there.

# Round 22.1: George Inness, one Opus 5.5 painter

A copy of round 22's runner (its tracked files, not its `run/` data). One lane, `INNS`: one Claude
Opus 5.5 painter (`anthropic/claude-opus-5-5`, thinking xhigh, pi-black) in Inness's studio, the
first painter in it (his box, his materials note), `record_kind="none"`: no reader.

- `BRANCH = "round-22.1"`: the export reads the notes from the tag, and `inness_materials.md` was
  edited on main on 2026-10-02 (`be06229`: his working times, a caveat and an anecdote out of the
  painter's copy), which `round-22` doesn't have. Code, harness and export scripts are round 21.5's,
  so `BASE` and `H` stay; the inness easel is already built in `BASE`'s `target/studio-build`.
- The brief is the plain inness one (`OPENING["inness"]`, `READING["inness"]`), byte for byte
  round 21.5's and round 22's `--briefs` output. No `trees.md` (friedrich only), no reference
  pictures: inness isn't in `ARTIST`, so the lane gets no `reference/` and `--only INNS` doesn't need
  `~/src/a/tonn-reference`.
- `LANES` is INNS alone. Round 22's `TONN` and `BUNT` run from round 22's runner; their code paths
  are kept here, unchanged.

# Round 22: Kendric Tonn, with pictures of his paintings

A copy of round 21.5's runner (its tracked files, not its `run/` data). One lane, `TONN`: one Claude
Fable 5.1 painter (`anthropic/claude-fable-5-1`, thinking xhigh, pi-black) in Tonn's studio (his box,
his materials note), `record_kind="none"`: no reader. `BRANCH = "round-22"`; the harness, the Rust code
and the export scripts are unchanged since round-21.5, so `H` stays the `claude-paint-r21.5` checkout
and `BASE` stays `claude-paint-r21` (the export builds the tonn box's easel in its
`target/studio-build`).

New: until now painters never saw pictures of the artist's work. Tonn gave his permission, and the
owner supplies them:

- **Where they go:** `REFERENCE` (`~/src/a/tonn-reference`, outside the repo): jpg, jpeg, png or webp
  files and, if she likes, a `README.md` describing them. When the tonn studio is exported the runner
  copies them into `<studio>/reference/` with her README, or with one that lists the files
  (`copy_reference`). Her files stay as they are.
- **What pi shows:** the painter looks at them with its `read` tool (pi's, limited to the studio). pi's
  read goes by the content (`utils/mime.js`: JPEG, PNG unless animated, GIF, WebP, BMP) and shows a
  picture as it is if it fits 2000x2000 px and 4.5 MB of base64. A larger one it resizes at every read
  and tells the model the original size. The limits are Fable 5.1's `inputLimits.images.resize` in
  pi-ai's `anthropic.json`. The runner copies a picture over 2000 px a side resized to fit, in its own
  format, and copies a picture its EXIF turns upright (pi sends one that fits byte for byte, EXIF and
  all). The harness keeps the newest 20 images and at most 12 MB of base64 in a request
  (`context-images.ts`); the runner logs how much the pictures come to.
- **Nothing starts without them:** with no folder, no picture in it, a file pi wouldn't show as an
  image, or another painter's name in the README or a file name (the export's
  `check_studio_names`, which reads only `notes/`), `main()` stops before any export or painter and
  says what to put where.
- **The brief:** the tonn opening says the pictures are in `reference/`, there with his permission, to
  study for his manner, and that the picture isn't a copy or a version of any of his; it still rules out
  other reference images and image models. With exactly one picture it reads in the singular
  (`OPENING_ONE`). The reading list names `reference/ (his paintings; reference/README.md lists them)`
  between the materials note and the physics note. Nothing else a painter reads changes: the brief
  template, the system prompt, the sitting messages and every other profile's brief are round 21.5's,
  byte for byte. `--briefs` writes `tonn.md` (several pictures) and `tonn-one-picture.md`.
- The runner now needs Pillow (`# dependencies` in the script header; the tests run with
  `--with pillow`).

- The painter's reply is read from the session file (its last assistant message that is text with no
  pending tool call) rather than from `pi --print`'s stdout, which came back empty on the
  Anthropic/pi-black path once while the session kept the text; stdout is the fallback. The sitting-1
  reply ("The Old Oak above the Fog") was recovered from the session and is back in `p1_s1_final.txt`.

## The warmup: lane BUNT, Tonn's studio without the pictures

Before the Fable painter, one Space Bunny painter (`opencode-go/space-bunny-free`, thinking max, as
round 21's FRDC chain: `BUNNY`, no extra environment, the key pi keeps for opencode-go) paints in
Tonn's studio as it was designed before the pictures: `"BUNT": lane("tonn", BUNNY, record_kind="none",
reference=False)`. Lane `TONN` is unchanged (its briefs are byte for byte the ones before this change).

- **`lane(..., reference=False)`:** the studio gets no `reference/` folder, and its brief is round
  21.5's for tonn, byte for byte but for the studio's path (`OPENING_PLAIN`, `READING_PLAIN`: "Work
  from knowledge and the notes in your studio; don't use reference images, image models or pictures
  of his work.", no reference line in the reading list). `--briefs` writes it as `tonn-no-pictures.md`.
- **Preflight:** only a lane that is among the lanes to run and gets pictures needs `REFERENCE`.
  `--only BUNT` starts with no `~/src/a/tonn-reference` at all; `--only TONN` (or both) still stops
  without pictures.
- **Two processes, one run folder:** the lanes are started as two processes from this folder, hours
  apart (`--only BUNT`, later `--only TONN`). Each lane's markers, sittings and logs are in its own
  `run/<lane>/`; `run/chains.log` is appended a line at a time by both. What changed:
  - a process watches only its own lanes' studios (`MY_LANES`, `our_studios`). Before, each process's
    watchdog also recorded the other lane's painting log in `run/monitor/`, and two processes
    comparing one log against what the other last recorded could report a history that "got
    shorter" when it hadn't;
  - `run/studios.json` is written whole (a temporary file, then renamed), so the other process's
    watchdog never reads half of it.
  - Not changed: each process needs its own output file, since `>` empties the file it names:
    `run/runner.BUNT.out` and `run/runner.TONN.out`.
- Against round 21's runner (which FRDC runs on), what a Space Bunny painter gets differently here is
  the harness: `H` is round 21.5's (no counters or machine time in the tools' replies, "Your brief is
  BRIEF.md in the studio", "You are not being evaluated in a quantifiable way"); round 21's harness
  is `BASE`'s. The launch command, the usage-limit waiting (`limits.ts`, the runner's probes) and the
  sitting rules are the same.

# Round 21.5: round 21.4 with trees.md's old-oak lead taken out

A copy of round 21.4's runner (SONF: one Sonnet 5.5 painter, read by GPT-6.1 Sol after it, no
second painter). Every Sonnet 5.5 painter in this studio (round 19's SONF and rounds 21 to 21.4)
planned a dusk scene around a "stag-headed" oak, the phrase of trees.md's last "In short" point;
one said "Since trees.md emphasizes proper tree architecture, I want to include this bare,
stag-headed old oak". `trees.md`, otherwise unchanged:

- "In short" loses point 5 ("Old trees shrink downward rather than simply dying. An ancient oak ...
  becomes "stag-headed" ...").
- Point 4 ends at "then dieback and a shrinking crown" (it went on "... and asks when the
  short-shoot stage is simply normal in old oaks").
- "Stag-headed" is gone from the terms and from the oak's "Old and damaged oaks" (its quote
  "Stagheaded oaks are classic examples of early stage retrenchment").
- The species are in alphabetical order: alder, beech, birch, lime, oak, pine, spruce, willow
  (oak was first).

Prompts, harness and guide as round 21.4; nothing recompiles.

# Round 21.4: round 21.3 with no machine minutes, costs or pixel limits

A copy of round 21.3's runner. The painting's own clock stays; what goes:

- The harness's timeout reads "the easel didn't answer; `log` shows whether the chunk ran" (it named
  12 minutes). The easel's "the chunk ran longer than 10 minutes and was stopped" reads "the chunk
  didn't finish and was stopped" (no chunk has come near it: across 43 studios' server logs, 2,676
  chunks, the longest took 219 s).
- `status` leaves out the pixel width (was `2400px · <setup>`).
- A crop's limit is in canvas units: the easel's "crop exceeds 1200 pixels per side" reads "a crop
  may be at most 500 units on either side".
- `notes/easel_guide.md`: no "2400 pixels wide", "1600 pixels", "3 MB" or "1200 pixels" (a crop is
  at most 500 units, at full detail, 2.4 pixels to a unit), and the memory note says masks and forms
  are large instead of "A mask costs 4 bytes a pixel, a form 28".
- Harness and guide only: no easel change, nothing recompiles.

# Round 21.3: round 21.2 without counters or machine time

A copy of round 21.2's runner. Round 21.2's prompts stay as they are; what the painter sees from
its tools loses everything that counts its work or times the machine (the painting's own clock stays):

- `paint` replies `ok` (was `ok · chunk N`; its compute time was already hidden).
- A Lua error names its chunk `[string "chunk"]` (was `[string "chunk N"]`).
- `status` leaves out the chunk count (was `N chunks · 2400px · <setup>`).
- `log` marks chunks `--@ chunk` (was `--@ chunk N`); the log file itself is unchanged.
- A look is renamed to a random UUID and shown without the easel's seconds (was
  `look-NNNN.png (1000x714, 0.04s)`), also the canvas view of a compaction or later sitting.
- The compaction and later-sitting summary leaves out "Chunks in the log: N (last: chunk N)" and
  each global's chunk number.
- `notes/easel_guide.md`'s tool table says so. All in the harness and the guide: no easel change,
  nothing recompiles.

# Round 21.2: round 21.1 plus one line

A copy of round 21.1's runner. Round 21.1's Sonnet 5.5 painter (killed after 3 chunks) still set
itself a budget ("Real time isn't the constraint—chunk count and my effort are, though there's no
hard limit, so I'll aim for roughly 60–120 chunks, efficient but thorough").

- `harness/painter/system_prompt.md`: round 21.1's, plus a second paragraph: "You are not being
  evaluated in a quantifiable way."
- `r21_chains.py`: `BRANCH = "round-21.2"`, the painter's harness `H` from `claude-paint-r21.2`;
  everything else as round 21.1 (export, check and finishing from `claude-paint-r21`, round 21's
  easel build reused: nothing recompiles). One lane, SONF.

# Round 21.1: round 21 with the painter's prompts tweaked

A copy of round 21's runner (`notes/round21/runner` at round-21, 83d4929), started 2026-10-01 after
the first minutes of a round-21 Sonnet 5.5 painter (killed) planned for a tool-call budget nothing set
("I don't know the exact tool-call budget, but I should plan conservatively for maybe 80-150 calls").
Round 21's FRDC chain goes on, unchanged, on round 21's prompts.

- `harness/painter/system_prompt.md`: "Your brief follows in the first message." (the first message
  points to BRIEF.md) is now "Your brief is BRIEF.md in the studio."
- `brief_template.md`: the clock bullet loses "The clock isn't a budget or a target."
- `r21_chains.py`: `BRANCH = "round-21.1"`; the painter's harness `H` comes from the round-21.1
  checkout (`claude-paint-r21.1`), the export, check and finishing scripts still from `claude-paint-r21`,
  so the export reuses round 21's easel build (same code trees): nothing recompiles. One lane, SONF.

# Round 21 runner: changes after the import

`r21_chains.py` was imported verbatim as prepared on 2026-09-30 (round 20's runner with `BASE`
and `BRANCH` pointing at the round-21 checkout). Since then (`git log -- notes/round21/runner`):

- **Probes** (round 19's N14): a usage-limit probe counts as the provider answering only when it
  exits 0 with "ok" or "okay"; a timeout, error exit or odd reply is `PROBE FAILED` and asked again;
  an error waiting won't fix (a refused key, no credit, an unknown model, a malformed request)
  stops the painter at once with `PROBE ERROR`.
- **The reader** (round 19's reader launch and N11): it runs as isolated as the painter
  (`reader.ts`, `reader_system_prompt.md`, read and write only; GPT-6.1 Sol through openai-codex since 2026-10-01, pi-black only if the reader is Anthropic) and reads
  only the logs, the journal and its brief and writes only its record (`READER_SCOPE`,
  `reader-scope.ts`). A reader that fails is logged with its exit.
- **Free-text flags** (round 19's N12): lines that look like what to do or where things go in the
  picture are logged as `RECORD FLAGGED` and listed in `p<n>_record.flags.md`; the record still
  goes on. Round 19's reject gate (`record_problems`) isn't part of round 21: a free-text record
  reaches the next studio as it did in the imported runner.

## Structured chain records (opt-in per lane)

The free-text record (`p<n>_record.md`, appended verbatim under "## More notes from the studio")
stays the default (`record_kind="free-text"`): the next studio's notes are byte for byte the
imported runner's (pinned against `fixtures/r21_chains_as_imported.py`) and `--dry` is unchanged.
A lane declared with `lane(..., record_kind="structured")` instead gets:

- **The reader writes observations, not notes:** `p<n>_observations.json` (`chain-observations/1`,
  `reader_brief_structured.md`): each a category, an easel operation, conditions from closed lists
  (tubes by name, no parts), one short markdown-inert sentence of effect and the tool calls in the
  numbered logs that show it. `reader.ts` refuses a write that isn't JSON.
- **Checks per observation** (`record_schema.py`): shape, enums and the easel's ranges; plain text
  only (checked and kept in NFKC; no control, invisible, bidi or line-separator character, no
  letter of another script); evidence that resolves in the logs and backs its basis (a chunk that
  ran or failed in a `paint` call or a bash `easel do`, an image shown after it by `look`, a read
  of a `.png` or an image part, something printed; an id two calls share can't be cited); another
  painter or a painter's name; a clear command ("Use...", "Lay...", "Clip every blend...",
  "..., so shorten them") or a place in the picture ("the lower third of the picture", "in the
  foreground"). A failing observation is dropped and logged, and the record goes on. Ambiguous
  words ("the edge of the canvas", "a background wash", a coordinate, "never", "instead") and
  chunk code that doesn't name the operation only warn. On the 716 sentences of the 11 released
  r16 and r17 records exactly their 13 clear prescriptions drop
  (`fixtures/r16_r17_record_sentences.json`); no sentence of any round's `studio_notes.md` drops.
- **The runner writes the notes** (`record_render.py`): a support line in the runner's words (the
  linen's thread counts and the easel's knife, roller or brush, read from the code of the canvas
  call with comments left out), then each category's observations with `When:` and how they are
  known ("printed by the easel", "seen once", "reported, not verified"). Each bullet is checked
  again as it reads whole, so a command split across fields drops it. Evidence, slot, round and
  reader aren't rendered. A record of another schema, medium, support or code commit (unless
  `record_compat.json` pairs the commits) is left out; another box leaves out mixing piles and
  tubes the box lacks. What each studio inherited, and what was left out and why, is in
  `p<n>_inherited.json` with the hash of its notes.
- **Stamped as round 21's:** the round comes from `BRANCH` (`round-21`) and the commit from what
  it names in `BASE` (the commit the export builds from).
- **The condition is declared:** `run/<lane>/condition.json` labels a structured chain lane
  `chain-inherited, non-neutral`, with the reader and the hashes of its briefs and of the checking
  and rendering code. The painter's notes keep the neutral heading and don't say they're
  inherited. Gallery caption for such a lane: "A chain: each painter after the first inherits
  notes read from the painters before it, so its results aren't comparable with a blank studio's."
- A structured lane can't continue another lane's free-text records (`records=`), and a run folder
  holding the other kind of record is refused at startup.
- A copy of the runner that runs structured lanes needs `record_schema.py`, `record_render.py`
  and `reader_brief_structured.md` beside it; a free-text lane runs without them.
