# Heavy-job turn — RETIRED: lockrun APPROVED by lead 2026-10-04 02:52 BST

lockrun status: APPROVED. Use the stable copy: ~/src/a/claude-paint-tools/lockrun
(copied from codex/speed 8b71762, sha256 cfcc8e51e276fb8692a7a74cff58a3ee0e7c5ac1afcdbb38d7843e9b02105185).
Lead verified: scripts/tests/lockrun.sh 17/17 pass (job 2), and used it for the baseline cross-check.

Every heavy command (cargo build/test/check/run, easel runs, anything multi-second CPU) from ANY agent:
  ~/src/a/claude-paint-tools/lockrun --timeout <S> --owner <agent:what> [--log FILE] -- <cmd...>
Limits: 60 s ordinary checks once built; 300 s one exact 2400px thinner card test; 600 s initial builds and final batches.
Exit 124 = UNFINISHED (timeout): save the log, mark unfinished, investigate small. Do not raise the limit or retry the same slow command.
`lockrun --status` shows who holds it. Nested lockrun inside a batch runs directly (LOCKRUN_TOKEN).
