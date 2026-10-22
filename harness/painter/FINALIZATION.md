# Finish worker

`finalize.py` runs on the host, separately from the painter. It discovers
`*/run-config.json` below each `--runners` directory and reads the runner's
`run/OPUS/p1_outcome.json`. Only `finished` and `finalization_failed` queue work.
It takes the run's `runner.lock` before touching its easel or stream, so an
active painter is never finalized. Aliases of the same studio and run are deduplicated.
The runner publishes a recoverable outcome before attempting close, so interruption
during the final save doesn't lose the finish trigger.

The worker's `--state` directory contains one durable job directory per studio/run.
`status.json` records the original outcome, attempts, errors, retry time, video
path and stream disposition. Rendering failures retry after 60 seconds, without
another painter sitting. The original failed-finalization outcome stays visible
even after recovery. `painting.mp4` is published only after ffprobe counts exactly
600 frames, 30 fps and 20.000 seconds. A partial encode isn't a video result.

Rendering uses the recorded gallery-lit hand frames. The first process image is
the first changed image after the last unpainted chunk, using the engine's
`frames/index.tsv` painted flag. The opening is 0.5 seconds of the final, followed
by 16.5 seconds of process and a 3-second final hold. All clips currently use the
plain final, including non-Friedrich paintings. No aging or finishing effects are
added. The source frames, checkpoint and log aren't modified.

After a failed close, the worker retries close against the retained easel. A dead
easel is recovered into the worker's directory from a copy of its latest
`replay.ckpt`, using `run --require-replay-checkpoint`. An absent, corrupt or
incompatible checkpoint is an error, never permission to replay from chunk zero.
The studio's bundled easel must contain the strict-checkpoint option; old studios
can still render complete recorded frames, but dead-easel recovery on an older
binary reports a visible failure. Saved checkpoints are rendered with
`render-checkpoint --mode gallery --log committed.lua`, which checks log identity.

The grace period is 900 seconds from the durable outcome's `finished_at` Unix
timestamp (the outcome file's mtime for older finished runs), not worker startup.
The stream is considered only after a verified video exists. A failed render or
save keeps the stream running until it succeeds.

`--streams` is a JSON array with at most one assignment, matching the host's
single OBS instance. Multiple assignments are rejected. The entry names an OBS websocket
endpoint, its password file, program scene, browser input and painting URL template:

```json
[
  {
    "endpoint": "ws://127.0.0.1:4455",
    "password_file": "/run/secrets/obs-websocket-password",
    "scene": "Stillwet",
    "input": "Browser Source",
    "url_template": "https://stillwet.art/live/{painting}/?stream=1"
  }
]
```

The placeholder is the studio's suffix after `paint-studio-`. This is an ownership
check, not a command to change OBS. The current program scene must match and have
exactly one visible item: that browser input, with exactly that painting's URL.
The worker checks the scene and URL again immediately before `StopStream`. Another
painting, another scene, extra visible sources, failed requests and missing
assignments never trigger a stop. An unassigned job stays `stream_not_assigned`
and is checked again on the next scan. OBS has no conditional-stop request, so
operators changing its assignment concurrently aren't an atomic transaction;
the repeated check narrows that protocol limitation without changing OBS.
The job is complete only after OBS reports that its output is inactive.

An empty stream array disables all OBS calls, including for saved-data proofs.
`--once` performs one scan. The normal loop scans every 30 seconds and holds a
singleton lock in the state directory. Secrets and host-specific paths belong
in runtime configuration, not the worker.

The cheap tests use fake OBS, including an authenticated local websocket server:

```sh
uv run --no-project --with pillow --with websockets python harness/painter/test/finalize_test.py -v
```
