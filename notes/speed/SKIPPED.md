# Rag release check scope

`all.tsv` and `fast.tsv` currently run the same release checks. They run the
paint library, easel unit tests, paint integration tests, and the easel's
boxes, delivery, determinism, smoke, and session-integrity tests. Replay
checks retain their existing live-width settings. A separate Inness painter
build checks the feature-limited application.

Seven full-painting replay tests are excluded: six `legacy::tests` replay
cases and `session::tests::logs_without_an_engine_line_replay_as_before`.
The unchanged eleven-case `old_logs.sh` package covers their initial
chunks plus synthetic legacy operations. Later painting chunks are not
covered by these replacements. No old golden is re-recorded.

Existing ignored diagnostics stay ignored. The ignored 3200-pixel ground
blend check is outside this release's rag changes; brushwork and ground
construction are unchanged. Unchanged studio, export, and round-runner
Python tooling is outside this check set. The gate coordinator, lock, and
safeguards have their own script tests in the manifest.

Thinner is absent from this release. Its checks and experimental baseline
are not part of the rag-only candidate.
