# Round 15: a second painter chain, from an audited studio
Same question as Round 14 (do painters learn from each other's notes?),
without what steered Round 14. Alice: "only keep the things they need and
pass along only the things they pass along".

**The studio** (branch r15-base, exported per painter with `git archive`,
built fresh; no git, no old builds): the engine, the stage runner, five
abstract studies, notes/guide.md (written for this round: the engine as a
painter uses it, no history, people, past paintings or motif recipes) and
the two research notes (the engine-building sections cut). Removed from
what painters saw in Round 14: ready-made figures and rocks, every past
painting's program, the workshop notebook, the developer notes, the subject
generators (rock, atmos, form's Ridge) and scene-shaped studies; history,
people and verdicts scrubbed from engine comments (comments only; engine
behavior unchanged apart from removing those modules and the runner's
default --full width, now 2400).

**Adversarial review:** Astra (medium) reviewed the studio and the briefs
before the chain: `astra_review.md` (24 findings). Acted on: 1-3 (partly),
6, 9-24. Left for Alice (engine behavior): 5 (the inverse color-recipe
search, `aim`/`mix`), 7 (gesture planning in the handling presets), 8
(stipple defaults); the research notes keep Friedrich's paintings and
methods (4, 20).

**The painters:** claude-opus-5-5 at thinking high, run as `pi --print
--no-context-files --no-skills --no-prompt-templates` (no global
AGENTS.md or skills), one after another (`chain.sh`). Briefs
(notes/briefs/sent/r15_p*.md): no project history, no duration or
deadline ("Develop the painting until you judge it complete"), full
commands, craft notes as "a materials-and-tools record" (operation, effect,
explanation), which is all that is passed on.
