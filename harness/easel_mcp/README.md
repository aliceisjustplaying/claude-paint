# Easel MCP

A stdio adapter for an exported painter studio, using Python 3.10+, uv and
the official MCP SDK (`mcp==1.30.0`). The five easel tools have the names,
schemas and descriptions of `harness/painter/easel-tools.ts`. No shell,
replay, save, close or finishing tools are exposed. `wait` is a Lua function,
called through `paint`.

## Configuration

Use `scripts/prepare_capy_studio` as described in the root README. Give it
an approved house BRIEF and, if that brief names it, an approved studio
notes file. The script changes only the brief's studio path. The guide,
techniques, physics note and profile's materials note come from the existing
exporter. It checks that every notes path named in the brief exists.

The printed JSON is a stdio registration, for example:

```json
{
  "command": "/absolute/path/to/uv",
  "args": ["run", "--no-project", "--script", "/path/to/studio/bin/mcp/server.py", "--studio", "/path/to/studio"],
  "cwd": "/path/to/studio",
  "env": {}
}
```

Alternatively, run the repository adapter directly:

```sh
uv run --no-project --script harness/easel_mcp/server.py --studio /path/to/studio
```

`EASEL_ROOT` can select the adapter's studio instead of `--studio`. It is
not passed to the easel: the painter binary locates its studio from its own
`bin/easel` location. Use the exporter-produced painter binary, copied into
the studio, not a symlink or a developer/replay binary. The easel subprocess
receives only `PATH`, `HOME` and optional `RAYON_NUM_THREADS`, as in pi.

All calls on one adapter are sequential. The adapter reattaches or opens
the easel on the first call, follows rebuild readiness and preserves the
harness's time budgets. Ending the MCP process leaves the easel open for
the next sitting. Closing the easel is an operator action.

Replies use the existing painter-visible formatting: no chunk counters or
compute times, random look filenames, scratch tags and full-detail survey
tiles. Easel errors become `isError: true` text without a wrapper; the same
counter hiding and look-option wording as pi still apply. Journal revisions
keep the original text in `out/easel/journal-revisions.jsonl`.

`PAINTER_MAX_IMAGES` (default 20) and `PAINTER_MAX_IMAGE_MB` (default 12 MB
of base64) limit a survey reply. The remaining tiles stay on disk and are
listed in the caption. The Capy painter needs its existing file-reading
capability for BRIEF, notes, the full log and those tiles; this adapter does
not replace Capy's read tool or restrict its other tools. MCP returns the
original PNG bytes. Image resizing and old-image context pruning belong to
the host, so the pi harness's per-model resizing and history pruning are
not implemented here.

For Capy's stdio registration, use a transport with `type: "stdio"`, the
printed `command`, `args` and `cwd`, plus `environment_names: []`. No
credential is needed. To pass optional runtime settings, name those
variables in `environment_names` and configure their values on the
thread's machine. The initial `uv run` needs package-download access;
subsequent launches use uv's cache. Register only after the studio exists
on the painter thread's own machine. Registration does not provision it.

## Tests

The fast tests run an official MCP client over actual stdio and a fixture
easel. They cover initialization, tool wording, paint, inline look images,
survey budgets, scratch options, compare confinement, journal revision,
status, log, bare environment and errors:

```sh
scripts/lockrun --timeout 60 -- uv run --no-project --script harness/easel_mcp/test_protocol.py -q
```

An optional real-easel test copies a prepared studio's binaries and adapter
to a fresh temporary studio and launches it with the printed uv command.
It paints three chunks, looks through MCP and confirms
that a failed chunk leaves both log and rendered image unchanged. It closes
its own easel when finished. No model painter or painting replay is used.

```sh
EASEL_MCP_STUDIO=/path/to/exported/studio \
EASEL_MCP_PROOF=/path/to/proof-output \
  scripts/lockrun --timeout 60 -- uv run --no-project --script harness/easel_mcp/test_protocol.py -q
```

The optional proof output contains the returned PNG, caption, successful
Lua log and failed chunk's error. The fast tests do not build Rust.
