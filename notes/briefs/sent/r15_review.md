# Adversarial review: what will bias these painters?

You are reviewing, adversarially, everything a chain of AI painters will
see before they paint. Find what will steer them in ways the project does
not want. Read-only: do not edit anything. Write your review to
~/tmp/paint-r6-b943b1ca/briefs/r15_review_answer.md.

## The experiment
Three AI painters (claude-opus-5-5) each paint one original landscape in
the manner of Caspar David Friedrich with a physical oil-paint simulator,
writing the painting as a Rust program. They run one after another. Each
starts in a plain copy of the folder below (no git). Painter 2 also reads
painter 1's craft notes; painter 3 reads both. That is the only thing
passed on. The question is whether painters learn from each other's
notes. The painters run without the user's global instructions or skills.

## What the project wants (the user's principles)
- The engine gives physics and tools; the painter makes every artistic
  decision. Tools that compute a result (a grown tree, a ready-made figure,
  a recipe) put a machine's answer into the picture.
- Painters choose their subject freely. Anything that tells them what to
  paint, what to avoid, what earlier painters did or what a viewer liked
  steers them. Past attempts showed this: a notebook of past painters'
  lessons made a painter avoid every motif it mentioned and add every
  "missing" thing; a list of ready-made motifs made painters reuse them.
- No history, people, verdicts, taste or past paintings in what they read.
  Negative statements ("don't do X", "we don't say Y") also steer.
- Time: painters tended to treat the budget as a ceiling or a wall-clock
  deadline and stop early; the brief tries to take that pressure off.

## What to review
1. The three briefs: ~/tmp/paint-r6-b943b1ca/briefs/r15_p1.md,
   r15_p2.md, r15_p3.md.
2. The folder every painter starts from:
   ~/src/a/claude-paint-r15-base (branch r15-base; review the working
   tree, not the git history, which the painters won't have). All of it:
   README.md, notes/ (guide.md and research/), paintings/src (run.rs,
   lib.rs, the study programs), crates/paint (source, module docs,
   comments and tests), scripts/peek, .cargo/config.toml, Cargo files.

## What to report
For each finding: file:line, the exact text, how it could steer a painter
(what it pushes them toward or away from), how confident you are and a
proposed fix (delete, reword with the new text, or keep with a reason).
Order by likely impact. Look especially for: named past paintings or
painters; motif recipes or preset motifs (functions that paint a subject);
opinions of what looks good or bad; history; references to files or tools
that don't exist; wording that makes painters hurry or play safe; wording
in the briefs that frames the experiment or invites them to perform; study
programs whose content amounts to a scene or a style; anything in the
research notes that reads as instructions rather than facts. Also say what
you would keep that looks suspicious but is fine. Be concrete and
thorough; skim nothing that a painter would read.

Your final message: the same review.
