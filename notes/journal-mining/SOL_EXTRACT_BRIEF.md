# Journal mining: extraction brief

Goal: mine painters' working journals for "how to paint at this easel" lessons that could improve the
painter prompts (brief, guide, studio notes). You extract; you don't edit any prompt, guide or note.

Project: claude-paint is an oil-paint simulator. AI painters paint through an easel (Lua chunks), look at
the canvas, and keep a dated journal (`- day N, HH:MM: ...`, simulated painting time). Rounds r16-r31;
the engine changed over time (engine 1 to 5: drying, solvent, rag, knife and absorbent grounds were
added), and the API changed. The current painter guide is ~/src/a/claude-paint-r31run/notes/easel_guide.md,
the current shipped craft notes are ~/src/a/claude-paint-r31run/notes/techniques.md and
~/tmp/gallery-fcf9c110/r31/studio_notes.md, the current brief is
~/src/a/claude-paint/notes/round31/BRIEF-draft.md. Skim those first so you can tell what painters were
already told.

Read every journal in your batch file completely. For each lesson or pattern worth keeping, write an entry:

- **Lesson** (one sentence, as practical advice a painter could act on)
- **Kind**: craft (true of painting generally) / easel (true of this simulator) / process (how to work:
  planning, looking, sequencing, when to stop) / failure-pattern (a recurring trap)
- **Conditions**: substrate and wetness, recipe/medium/load, tool, hand, scale, whatever the journal says
  mattered. Mark the engine.
- **Evidence**: what the painter observed; whether later entries (or later sittings) confirmed or refuted it;
  quote briefly. Cite as `<studio>:<line>` (line number in the journal file).
- **Already in the prompts?** yes (where) / partly / no.
- **Still valid?** If it depends on an engine behavior that may have changed since (engine 1-2 lessons
  especially), say so; don't guess beyond what you can check in the guide.

Also note across your batch:
- the biggest time sinks and repeated rescue cycles, and what (if anything) got painters out of them;
- moments where a painter's approach changed and the picture clearly improved;
- confusion about the tools or the brief (misreadings that a clearer prompt line would prevent);
- anything about trials, testing, the scratch canvas (r31 smoke only), sittings and handoffs, finishing.

Rules: evidence only from the journals (and the guide/notes for "already in the prompts"); no outside
knowledge presented as findings; label your own interpretation as such. Plain US English, no Oxford comma.
Don't name the painter models in lessons (lane names in citations are fine).

Output: one markdown file at ~/src/a/claude-paint/notes/journal-mining/sol-extract-<BATCH>.md: a short summary,
then entries grouped by theme (ground and drawing, color mixing, thin vs thick paint, blending and edges,
glazing/scumbling, drying and timing, corrections and lifting, detail and texture, composition and
looking, process and sittings, tool confusion). Then a final short list: the 10 strongest candidate
prompt improvements from your batch, each with its citations. Report back the path and a 5-line summary.
Do not commit. Do not modify anything else.

Additional rules for this run:
- Do not spawn, delegate to or start any subagents, child tasks or other agents. Do all the work yourself.
- Do not read the other extraction files in this folder (extract-A.md, extract-B.md, extract-C.md, PROPOSALS.md or any sol-* file but your own): this is an independent second pass.
