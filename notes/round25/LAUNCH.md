(•‿•) Round 25 is prepared, not started. The candidate is an original winter landscape in Friedrich's manner, with Claude Opus 5.5 at high thinking, engine 5 and the named 16-tube Friedrich box. [Configuration and hashes](readiness/snapshot.json), [mechanical checks](readiness/verification.log).

The opening is:

> Compose and paint one original landscape in the manner of Caspar David
> Friedrich, at the easel, a simulator of oil paint on linen. The place,
> subject and composition are yours to invent. Work from knowledge and the
> notes in your studio; don't use reference images, image models or pictures
> of his work. Let winter be the season.

The [complete brief](readiness/briefs/friedrich.md) includes the original [Friedrich composition note](runner/friedrich_painter.md) in its reading list. The painter invents the place, subject and composition. The box supplies the original fourteen tubes plus Naples yellow and rose madder. [Exact tube table](readiness/tubes.md).

Source, harness, export, check and finishing are pinned to `e017df8e0394fbe520b41ddac112e69e46621cd2`, checked out detached at `~/src/a/claude-paint-r25run`. The launch configuration is [the R25 runner](runner/r21_chains.py); its runnable copy is `~/tmp/gallery-fcf9c110/r25/r21_chains.py`, lane `FRDC`. It uses `anthropic/claude-opus-5-5`, `--thinking high` and pi-black. [Exact dry-run commands](readiness/dry-run.log). The full commit pin avoids dependence on a mutable local tag.

The exact exported candidate, with its binary, brief and painter-facing notes, is `~/src/a/claude-paint-r25run/prepared/friedrich`. Launch creates a fresh neutral studio from the same source and configuration. The painter export and replay build passed. [Export](readiness/export.log), [replay build](readiness/replay-build.log), [engine probe](readiness/engine-probe.lua). The probe is a temporary mechanical verification canvas, not an artist painting; it was deleted after its log was saved. The candidate's paintings folder remains empty. [Snapshot](readiness/snapshot.json).

There is no sitting cap. The current handoff and recovery prompts, rag refolding behavior, completion review and operation guards are retained from the frozen source. Crashed or interrupted work remains distinct from completed work; `MAX_CRASHES` remains 6. [Runner](runner/r21_chains.py), [source guide](../../notes/easel_guide.md), [verification](readiness/verification.log).

The detached runtime checkout, candidate binary, export executable cache and replay build are retained for launch. The export's temporary canvas and intermediate box compilation directory were removed. No provider authentication probe or painter invocation occurred. [Verification](readiness/verification.log).

Not done yet: the painter has not been launched. The dry-run displays provider commands without executing them; provider availability has not been exercised. [Dry run](readiness/dry-run.log).
