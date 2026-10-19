# Remote easel

The painter runs in Capy while this HTTP MCP service and its exported easel
run on the Mac. The existing studio viewers read its tool journal and image
results. No viewer code or existing tunnel configuration needs to change.
Select the new studio in the viewer and OBS using the normal launch workflow.

## Start locally

Export a fresh studio with `scripts/prepare_capy_studio`. Name its folder
`paint-studio-<hex>` so the existing viewer recognizes it. Use an approved
house BRIEF and studio notes. Create a random token in a private file:

```sh
mkdir -p ~/.config/easel-mcp
chmod 700 ~/.config/easel-mcp
python3 - <<'PY'
from pathlib import Path
import secrets
path = Path.home() / '.config/easel-mcp/token'
with path.open('x') as file:
    path.chmod(0o600)
    file.write(secrets.token_urlsafe(32) + '\n')
PY

uv run --no-project --script harness/easel_mcp/http_server.py \
  --studio /path/to/paint-studio-<hex> \
  --token-file ~/.config/easel-mcp/token --port 8790
```

Use a free loopback port. The token can instead come from `EASEL_MCP_TOKEN`.
It is never printed, placed in a URL or written into the launchd plist.
Requests require `Authorization: Bearer <token>`. Access logging is disabled.

One studio has one remote adapter process and one bearer credential, assigned
to one logical painter. A process-level studio lock rejects a second remote
adapter, calls are serialized and the easel retains its own daemon session
lock. Reconnected clients with that same credential are the same painter.
Do not share its credential with independent painters or run a local painter
against the same studio. Local operator access to `bin/easel` is still possible;
the adapter is not an operating-system access boundary.

## Calls and reconnects

The five tools keep their original descriptions and easel arguments. HTTP
adds required `call_id` to every tool. Use a unique ID for each new call and
the identical ID, tool name and arguments for each retry. Reusing an ID with
different arguments is rejected.

The adapter commits a durable pending record before invoking the easel and
stores its complete reply, including images, afterward. Retries return that
reply without invoking the easel again. Dropping the client does not cancel
the running chunk. A slow call returns a “still running” reply after 20 seconds
by default; retry that same call to retrieve its result. This bounds HTTP
requests while the easel keeps its existing chunk and rebuild time budgets.

Admission is bounded to one running call and four waiting calls. A new call
beyond that gets a “busy, retry later” error and is not recorded or executed;
retry it unchanged after the queue drains. Retries of admitted calls and
cached results remain available even while the queue is full.

Completed tasks stay in memory for at most ten minutes, with at most eight
retained. The oldest is evicted when the count limit is reached, and a timer
expires tasks even if no client reconnects. The durable SQLite result is kept
after memory eviction, so a later reconnect still retrieves the same result
without executing the call again. These limits do not expire durable call IDs.

After a server crash, a completed record is still replayable. A pending record
has an uncertain outcome and fails closed: the adapter never runs it again.
Use new `log` and `status` calls to inspect the painting, and ask the operator
if the result is unclear. Never automatically submit that chunk under a new ID.
There is no claim of atomic exactly-once execution across an engine process
and an adapter process without a shared transaction.

`out/easel/mcp/` holds the lock, SQLite call cache and studio event journal.
Keep it with the studio across restarts. Do not delete the call cache to fix
a retry. The journal is a pi-compatible projection of actual tool calls and
results, not a transcript of Capy's thoughts or final messages. A symlink in
`~/.pi/agent/sessions/` makes existing viewers and static studio sync find it.
Image captions still name the easel's normal files and gallery/relief
companions, so the existing display lighting works. For isolated tests,
override `--sessions-root` to keep the proof out of the live picker and sync.

## Stage the second quick tunnel

The existing r25tunnel is a quick tunnel to the studio viewer. This service
uses a separate quick tunnel and leaves that launch agent alone. Prepare,
but do not load, two new plists:

```sh
scripts/prepare_remote_easel_launchd \
  --studio /path/to/paint-studio-<hex> \
  --token-file ~/.config/easel-mcp/token \
  --out ~/.capy/work/easel-mcp-remote-launch --port 8790
```

The helper writes `art.stillwet.easel-mcp.http.plist` and
`art.stillwet.easel-mcp.tunnel.plist` on macOS, with KeepAlive and a 15-second restart
throttle. On Linux the same helper writes `.service` files for systemd's user manager,
with `Restart=always` and `RestartSec=15`. Output stays in the staging folder's `logs/`.
It does not run launchctl, systemctl or cloudflared and refuses to stage directly
in `Library/LaunchAgents` or the Linux user's systemd configuration directory.
The Linux units can be installed with `systemctl --user link /absolute/staged/*.service`
and started individually with `systemctl --user start art.stillwet.easel-mcp.http.service`.
The staging folder must remain in place for logging. Loading the tunnel service still
requires approval on either platform.

The proposed tunnel command is:

```sh
cloudflared tunnel --no-autoupdate --url http://127.0.0.1:8790 \
  --http-host-header localhost:8790 --logfile /path/to/staged/logs/tunnel.log
```

The origin Host override lets the SDK retain its localhost-only Host check.
Quick tunnels [do not support SSE](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/do-more-with-tunnels/trycloudflare/),
so this adapter uses stateless Streamable HTTP with JSON responses. It does
not depend on SSE delivery or HTTP session affinity. Quick-tunnel addresses
change when the tunnel restarts. Print the current Capy registration:

```sh
scripts/easel_mcp_registration /path/to/staged/logs/tunnel.log
```

The helper uses the last trycloudflare URL in that log and names the secret
`EASEL_MCP_AUTHORIZATION`. Configure that secret through Capy's secure flow
with the full `Bearer <token>` header value. Its printed configuration contains
only a secret reference. After the usual marketplace lookup, submit it as an
HTTP MCP registration, or update the existing server's URL after a restart.
An old log is not a health check; confirm the new tunnel is running before
updating the registration. A named tunnel with a stable hostname is the
long-term option and requires Alice's Cloudflare account and explicit routing
approval. Quick tunnels are a development service with no uptime guarantee.

## Proof and tests

The fast protocol tests cover real HTTP transport, authorization, unique
writer ownership, serialization, dropped clients, bounded retries, cache
recovery, inline images, viewer ingestion and the configuration helpers:

```sh
uv run --no-project --script harness/easel_mcp/test_remote.py -v
```

The real-easel proof script works on Linux or macOS. It refuses a studio with
an existing painting, paints two tiny test chunks, retries one and verifies
that the existing viewer code serves the same returned image. By default it
starts an isolated loopback viewer and does not open a public tunnel:

```sh
uv run --no-project --script scripts/prove_remote_easel \
  --studio /path/to/empty/paint-studio-<hex> \
  --sessions-root ~/.capy/work/easel-mcp-remote-proof/sessions \
  --out ~/.capy/work/easel-mcp-remote-proof/output
```

For an approved proof through the already-running Mac viewer, instead supply
`--sessions-root ~/.pi/agent/sessions --viewer-url http://127.0.0.1:8769`.
That registers a fresh test studio with the live picker and automatic studio
sync, so do not choose this variant unless that visibility is intended.

## Cloud-only bench fallback

For independent bench painters, keep the cloud easel and stream nothing live.
A future read-only sync can copy its tool journal, `paintings/lua/painting.lua`,
`notes/journal.md` and look PNGs with gallery/relief companions into a dedicated
Mac mirror. Rewrite the projected session's cwd to the Mac mirror and keep
look captions relative to it. Publish completed JSONL records and referenced
images together, using an atomic staging-directory swap; never copy a partial
record or allow the Mac to execute the mirror's painting log. Do not copy a
Linux easel binary or a checkpoint. A viewer session link can then show the
mirror without granting it a writer. This fallback is a design only; the
Mac-hosted easel avoids its consistency and sync delays.
