# The workshop notebook: what our painters learned about painting trees

Worktree: ~/src/a/claude-paint-craft-trees (branch craft-trees). Write ONE
file: notes/craft_trees.md. Commit it; do not push. Scratch: run
`~/.local/bin/agent-tmp crafttrees` once and export TMPDIR=TMP=TEMP.

## Why
About twenty AI painters in this project have painted trees in the manner
of Caspar David Friedrich, each from scratch, each alone. Each left notes
on what it tried, what went wrong and why, and a viewer reacted to every
picture. None of that reached the next painter, so each one rediscovered
the same failures. You are writing the notebook a workshop would keep:
what painters here tried with trees, what happened and why, told the way
one painter tells another, so the next painter starts from it.

## Sources (read them; `git show <branch>:<path>`)
- The painters' notes (composition, method, friction, critique):
  amnesia-winter:notes/fresh2_winter.md; r10-arm1:notes/r10_winter_a.md;
  r10-arm2:notes/r10_winter_b.md; r10-arm3:notes/r10_summer.md;
  r11-study1..3:notes/r11_study1..3.md; r11-astra, r11-fable, r11-flash:
  notes/r11_winter_<name>.md; r12-tree1..3:notes/r12_tree1..3.md;
  r13-tree1..3:notes/r13_tree1..3.md; and the round 7-9 arms'
  notes/roundN/armN/notes.md on branches r7-arm1..3, r8-arm1..3,
  r9-arm1..3 (their pollard willows).
- Their programs, only to understand what a note describes (same
  branches, paintings/src/bin/ or paintings/lua/).
- The session logs, where the notes are thin: ~/.pi/agent/sessions/
  (directories named after each worktree, e.g. *claude-paint-r13-tree2*).
- What the viewer saw: notes/round6/alice_review.md on main. Use her
  DESCRIPTIONS of what she saw in the trees ("twigs hanging in the air",
  "brown brooms", "fractal-ey", "wiry", "a trunk that doesn't match its
  top", "every branch carries the same snow") and what she saw in real
  Friedrichs. Do NOT pass on rankings, favorites, scores or which picture
  was best or worst, and nothing from the AI critics: the notebook must
  not become a list of what pleases a judge.
- How Friedrich drew trees: section 5 of notes/research/trees.md.

## What to write (causes and effects, not recipes)
- For each way painters painted trees (thousands of twigs each drawn as
  its own line, the same forking at every scale, a few limbs painted one
  by one, leaves placed by hand, limbs bent or gnarled by hand, the crown
  as masses, snow on limbs, trunks, roots, pollards, dead oaks): what they
  painted, what it looked like to them and to the viewer, and why, where
  a painter or the record explains it.
- Patterns across painters: what kept going wrong, and the few things
  that held up.
- What painters did that a person painting trees would not do, and the
  reverse.
- How Friedrich worked with trees, briefly, from the research section.
- Knowledge in words: no code, no parameter values, no function names, no
  step-by-step recipes; nothing about how to get a good score. Say how
  sure the record is for each claim and cite it (branch:file or session).
- NO HINTS of generated trees: never mention growing, generating,
  algorithms, rules, recursion, skeletons, seeds, procedures or any
  engine or program structure for trees, even when a painter used them.
  Describe only what was painted and how it looked (e.g. "every twig drawn
  as its own line, each forking like the last" and how that read), as if
  every painter had worked by hand.

Length: dense, roughly 150-250 lines. US English, no Oxford comma. Never
write the user's real name or an absolute home path into the file; the
user is "Alice", the viewer.

Final reply: a short summary of the patterns, and the three things you
think a painter most needs to know before painting a tree.
