# A new painting (claude-paint, Round 7)

You are a painter at claude-paint's easel: a live Lua painting session
over a physical oil-paint simulator (bristle brushes carrying wet paint on
primed linen, Kubelka–Munk optics). Paint one new, original painting in
the manner of Caspar David Friedrich (1774–1840). What to paint is
entirely your choice: subject, composition, season, hour, format.

Work from knowledge only: what is known about Friedrich's materials,
method, habits and motifs (notes/friedrich.md, notes/research/). Never
look at or search for pictures of his (or anyone's) paintings, and don't
copy a known painting: make a new one in his manner.

## Who judges, and how
Alice (the project's owner) judges by eye. Her yardstick, in her words:
"less digital, more bad painter", which is good: a painting that looks
like a person made it with a brush beats one that looks rendered. She
looks at the whole and then zooms in at 3200px, where "digital-looking"
artifacts, hard cut-out edges, hairline wire and mechanical repetition
cost the most. Her reviews so far: notes/round6/alice_review.md.

## Read before you paint
1. crates/easel/README.md (the easel, its verbs and tools).
2. notes/sketchbook.md (the craft so far: recipes with numbers, their
   ceilings, pitfalls; start with "Hierarchical detail, not maximal
   subtraction" at the top).
3. The Round 6 lab, where single subjects were painted two or three ways
   and judged: notes/round6.md, notes/lab/*.md and notes/lab2/*.md (each
   has the key, what won with Alice or the critics and a sketchbook
   candidate), notes/round6/alice_review.md. Take what won as a floor,
   not a formula.
4. notes/time.md (hand time and sittings: opt-in with
   `canvas{..., hand=true}`; painting in sittings with rests lets paint
   set naturally) and notes/workflow.md.

## Setup
- Worktree ~/src/a/claude-paint-r7-paint-wet, branch r7-paint-wet. Work only
  there; don't push; commit your files early and often (sessions can die).
- `cargo build --release -p easel` first. Name your session explicitly
  (`easel -s <name> ...`); the easel works in the checkout you run it in.
- Scratch: `~/.local/bin/agent-tmp paintwet`; export TMPDIR=TMP=TEMP there.
  Never write to /tmp. Wrap long commands in `timeout`.
- Other agents share the machine (a glitch hunt and a pencil check are
  running): work at 1000px while painting; 3200 at the end.
- View PNGs only through `scripts/peek` (it's for you: small JPEGs). Look
  constantly, like a painter: whole, squint, value, crops at 3200.
- Don't modify crates/paint or crates/easel. If the engine gets in your
  way, work around it and write down exactly what went wrong (with crops):
  that's valuable.

## Budget
Take your time: quality over speed. About 2–3 hours of work. Finish
something you believe in rather than something ambitious and half-done.

## Deliver (commit it)
- The session log as `paintings/lua/<name>.lua` (replayable: `easel run`),
  renders `out/<name>.png` (1000) and `out/<name>_full.png` (3200).
- For Alice (she views on another machine, so commit it; lossless):
  `notes/paint1/<name>_alice.png`: the whole at 1000px beside or above
  three 1:1 crops from the 3200 render of the passages you care most
  about. Build it with `magick` from the PNG renders (never from JPEGs);
  label with `-font /System/Library/Fonts/Helvetica.ttc`.
- `notes/paint1/<name>.md`: what you painted and why (the idea, the
  composition, the palette, the order of work and sittings), what you
  think works and what doesn't, the engine problems you hit, and
  SKETCHBOOK CANDIDATES (a principle plus a recipe with numbers) for
  anything you found that beats the sketchbook.
- Final reply: the painting's name, the sheet path and a short honest
  self-assessment.

US English, no Oxford comma. The project is published under the pseudonym
"alice": never write the user's real name or an absolute home path into
any committed file (write `~/...`); a pre-commit hook rejects violations.
