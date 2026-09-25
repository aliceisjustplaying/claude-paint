# Round 14: a chain of painters who learn from each other
Three painters (claude-opus-5-5, thinking high, Rust), one after another.
Each starts in a plain folder exported from branch r14-base (no git, no
history): no growth model (as r13-base), notes/workshop.md (the workshop
notebook: what ~30 earlier painters tried and what happened, by subject;
no rankings, no critics, no hints of generated trees) in place of the tree
research. Theme: "A landscape. Any season, any time of day, any subject
Friedrich might have chosen." 2400 px, 120-180 minutes.
Each writes craft notes for the painters after (causes and effects in
words; no code, no settings, no verdict on their own picture). Painter 2
also reads painter 1's; painter 3 reads both. Nobody sees another's
pictures or code, or any judgment.
Question: do painters 2 and 3 paint better than painter 1?
Briefs: notes/briefs/sent/r14_p*.md. Folders: ~/src/a/paint-r14-p1..3;
their files are copied back to branches r14-p1..3 afterward.

## Painter 1 and the notebook (Alice)
Painter 1 took the notebook as instructions (avoided every motif it said
painters converge on, added every "missing" thing). Plan: revise the
notebook (cut composition, subject choice and the lesson-like summaries;
keep per-subject cause and effect) and restart the chain.
If that fails too, Alice's method: one Claude reads the painters' session
logs one by one, in chronological order, taking notes as it goes, so the
notebook grows as a sequential understanding (as she has done with
therapy transcripts) rather than a grep across everything.

## What we learned during the chain (for Alice, morning)
- Painter 1 (revised notebook) stopped at 65 of 120-180 minutes: it named
  the empty foreground as the weakest part, tried a path and stones, reverted
  them and called the picture done ("returns are diminishing"). Agents treat a
  budget as a ceiling to come in under.
- Painter 2 read the time as a wall-clock deadline ("a full render by around
  02:36") and chose its subject for safety ("play to what renders well") and
  by avoiding painter 1's motif ("mountain mist was already used"). Its
  candidates included a ploughed field with a far town: the first painter 1's
  picture, which it never saw (the same model reaching the same idea twice).
- Painters load Alice's global AGENTS.md (kaomoji, receipts, comms skills).
- Painter 3's brief replaces the budget with: "You have about three hours at
  the easel. There's no deadline for any render, and nothing is gained by
  finishing early; use the time the picture needs."
- The open problem (Alice): anything we pass on steers them in ways we don't
  want, and editing notes line by line is whack-a-mole. Needs a method, not
  more edits (candidates: Alice's chronological-reading method; letting the
  painter choose what to read; passing only failures tied to a subject the
  painter has already chosen).
