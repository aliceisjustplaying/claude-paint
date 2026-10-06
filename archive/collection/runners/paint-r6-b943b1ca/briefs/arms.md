# A new painting (claude-paint, Round 7, three arms)

You are a painter. Paint one new, original painting in the manner of
Caspar David Friedrich (1774–1840) with claude-paint, a physical oil-paint
simulator (read README.md). Subject, composition, season, hour and format
are entirely your choice. Work from knowledge only: never look at or search
for pictures of his (or anyone's) paintings, and don't copy a known work.

Read first: notes/principles.md, notes/briefs/friedrich_painter.md (his
composition and restraint), notes/briefs/friedrich_hand.md (how he worked),
notes/sketchbook.md (craft; recipes are floors, not formulas).

Alice judges the result by eye, whole and zoomed in: "less digital, more
bad painter". She sees your plain renders first, with crops chosen by
someone else. So make the picture good everywhere, not in a few passages.

{ARM}

## Setup and rules
- Worktree {WT}, branch {BRANCH}. Work only there; commit your files early
  and often; don't push. `cargo build --release` first.
- Scratch: `~/.local/bin/agent-tmp {NAME}`; export TMPDIR=TMP=TEMP there;
  never /tmp. Wrap long commands in `timeout`. Two other painters share the
  machine: work at 1000px, render 3200 at the end.
- Look at your work constantly with `scripts/peek` (whole and crops at
  3200).
- Don't modify crates/paint or crates/easel. If the engine gets in your
  way, work around it and note exactly what happened.
- Budget: about 2–3 hours. Finish something you believe in.

## Deliver (commit)
- Your painting's source (see your arm) and plain renders copied to
  `notes/round7/{NAME}/painting_1000.png` and `painting_3200.png`
  (lossless PNG). Don't make sheets or choose crops.
- `notes/round7/{NAME}/notes.md`: short and factual: what you painted,
  your order of work, the tools and techniques you used, engine problems
  you hit. No assessment of how good it is (Alice judges).
- Final reply: the title and the paths. Nothing else.

US English, no Oxford comma. The project is published under the pseudonym
"alice": never write the user's real name or an absolute home path into
committed files.
