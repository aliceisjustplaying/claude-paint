# Cloud studio publishing

This is an opt-in outbound publisher. The normal `studio.py` API, Mac painters,
minute-by-minute `/studio/` export and existing tunnel keep their current
behavior. Nothing installs or starts a publisher automatically.

The viewer reads Pi-shaped session JSONL: tool calls and results, journal
entries and the PNGs returned by `look`. It also reads display-only Gallery
light and Relief companion PNGs. It does not read checkpoints or engine state.
The local viewer polls its API every two seconds; the traditional static
export polls every minute. A hosted live export polls a small manifest every
two seconds and fetches new snapshot metadata only when its revision changes.

The publisher reuses the public static exporter for one selected painter.
Each changed data file becomes a SHA-256-addressed object. A snapshot maps
logical file names to those objects. Rsync sends new or changed files with
temporary-file rename, then sends `manifest.json` last in a separate command.
Old manifests still reference complete old data during interruption or retry.
No delete or in-place update is used. Unchanged exports transfer no file data.
Old objects remain available for replay and viewers on earlier snapshots.

The studio shows the painter's latest whole-canvas look. A committed paint
chunk updates the code and clock; a later `look` supplies the next canvas
image, as in the local viewer. The publisher does not ask the painter for
extra looks or change what the painter sees. Journal entries come from
recorded `note` calls; arbitrary notes, binaries, keys and checkpoints are
never exported. Public text uses the existing exporter scrubbing.

## Operator setup (after deployment approval)

Use an approved house brief and reading list. Prepare the exported studio
normally, adding a separate private session root:

```sh
scripts/prepare_capy_studio hopper /workspace/paint-studio-abcdef \
  --brief /path/to/approved/BRIEF.md \
  --studio-notes /path/to/approved/studio_notes.md \
  --sessions-root /workspace/live-sessions
```

Register the JSON printed by the script in the cloud painter thread. The
optional `--sessions-root` makes the stdio MCP adapter reuse the remote
adapter's durable viewer journal; it does not expose an HTTP MCP server or
add tools or arguments. Existing registrations without that option are
unchanged. The existing remote MCP adapter already records equivalent
sessions and can also be published.

Provision a dedicated restricted upload key and a verified VPS host key as
described in nixfiles `docs/STILLWET-LIVE.md`. The painter connects as the
dedicated `stillwet-live` user, never `agent`. Start an operator-owned uploader:

```sh
uv run --no-project --script studio/publish_live.py \
  --painter paint-studio-abcdef \
  --sessions-root /workspace/live-sessions \
  --work-dir /workspace/live-publish-work \
  --destination stillwet-live@VPS_HOST:PAINTING/ \
  --identity /path/outside/checkout/live-upload-key \
  --watch
```

Omit `--watch` for one publication. The watch checks completed session writes
every two seconds and retries failed transfers without rerunning paint calls.
It waits if the painter has no session yet. Only the selected painter is
exported. Run one publisher per painting with its own work directory; a file
lock rejects a second process on the same work directory. Multiple different
work directories targeting the same remote painting are not supported.

Viewers open `https://stillwet.art/live/PAINTING/`. OBS uses the fixed
`https://stillwet.art/live/PAINTING/?stream=1` URL. Gallery light remains the
default with Color / Gallery light / Relief controls. A browser with a saved
lighting preference keeps it. No tunnel, local studio restart, plist change
or Mac connection is needed.

All published data is public, including code and journal entries. Review the
intended painter session before enabling publication. Scrubbing replaces the
home path and account name; it is not a generic secret detector. The restricted
key can replace other live publications and fill the live volume, but it cannot
write the gallery. Revoke it after the run. Operators can prune old objects
once viewers disconnect; the publisher never deletes remote history.

## Offline proof

Prepare a new empty exported easel on the cloud machine with an approved
brief. The fixture below runs fixed test chunks, not a model painter. It
starts a local static HTTP server, records stdio tool results and runs the
watch publisher with `--local-root` instead of SSH:

```sh
LIVE_STUDIO=/absolute/path/to/new/paint-studio-abcdef \
LIVE_PROOF_OUT=/absolute/path/to/new/proof \
NODE_PATH=/path/to/playwright/node_modules node studio/test_live.cjs
```

Playwright and Chrome must be installed. The test proves two live Gallery
light snapshots without page reload, failed-chunk rollback, journal updates,
hashed image URLs, replay selection, stream layout and no API requests. It
writes two viewer frames, a stream frame and JSON receipts into the proof
directory. It closes its painter session and stops its HTTP server/uploader.

```sh
uv run --no-project --with pytest --with pillow python -m pytest -q studio/test_publish_live.py
uv run --no-project --script harness/easel_mcp/test_protocol.py ProtocolTests PreparationTests -v
```

The publisher tests additionally run real rsync through a local rrsync forced
command and verify interrupted publication, unchanged-file skipping and
confinement. They do not contact the VPS or use a production key. Actual SSH
authentication, VPS ownership, Caddy activation and OBS playback require the
separately approved go-live checks.
