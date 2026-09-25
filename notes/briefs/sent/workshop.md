# The workshop notebook

Worktree: ~/src/a/claude-paint-workshop (branch workshop). Write ONE file:
notes/workshop.md. Commit it; do not push. Scratch: run
`~/.local/bin/agent-tmp workshop` once and export TMPDIR=TMP=TEMP.

## Why
Over the last days about thirty painters (AI models) in this project each
painted an original picture in the manner of Caspar David Friedrich, from
scratch and alone. Each kept notes on what they tried, what went wrong and
why, and a viewer, Alice, reacted to every picture. None of it reached the
next painter, so each one rediscovered the same things. Write the notebook
a workshop keeps: what painters here tried, what happened and why, told
the way one painter tells another, so the next painter starts from it.

## Sources (`git show <branch>:<path>`; list branches with `git branch -a`)
- Every painter's notes (composition, method, friction, critique): the
  amnesia-* branches (notes/fresh2_*.md), r7-arm1..3, r8-arm1..3,
  r9-arm1..3 (notes/roundN/armN/notes.md), r10-arm1..3, r11-study1..3,
  r11-astra, r11-fable, r11-flash, r12-tree1..3, r13-tree1..3 (their
  notes/*.md), and any other painters' notes you find on main (e.g. the
  lab notes in notes/lab*/, notes/round*/).
- The existing tree notebook, main:notes/craft_trees.md: fold it in as one
  section, shortened.
- Their programs, only to understand what a note describes.
- Session logs where the notes are thin: ~/.pi/agent/sessions/
  (directories named after each worktree).
- What the viewer saw: main:notes/round6/alice_review.md. Use her
  DESCRIPTIONS of what she saw (in our pictures and in real Friedrichs).
  Do NOT pass on rankings, favorites, scores or which picture was best or
  worst, and nothing from the AI critics.

## What to write
Organize by what a painter works on, not by round or painter: the ground
and underdrawing; skies; distance and air; snow, ground and grass; water
and ice; trees; rocks and buildings; figures and animals; small details;
edges and how things sit in their surroundings; the paint surface and
texture; composition; working method, time and looking (including how the
tools behave in practice: rendering, crops, checkpoints, what is slow).
For each: what painters did, what it looked like to them and to the
viewer, why where the record says, what kept going wrong and what held up.
Also: what painters did that a person painting would not, and the reverse.

## Rules
- Causes and effects in words: no code, no parameter values, no function
  or type names, no step-by-step recipes, nothing about getting a good
  score. Say how sure the record is for each claim and cite it
  (branch:file or session).
- NO HINTS of generated trees or plants: never mention growing,
  generating, rules, recursion, skeletons or seeds for trees, even where a
  painter used them. Describe what was painted and how it looked, as if
  every painter had worked by hand.
- Balanced: trees are one section among many, not the center.
- Dense: roughly 300-450 lines. US English, no Oxford comma. Never write
  the user's real name or an absolute home path into the file; the user
  is "Alice", the viewer.

Final reply: a short summary of the sections and the patterns that recur
across them.
