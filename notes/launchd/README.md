# Studio viewers under launchd and systemd

In historical notes, private-viewer.invalid and 192.0.2.1 are placeholders for redacted network addresses, not live endpoints.

Two LaunchAgents keep the studio viewer up across sessions and logins, restarting it if it dies
(`KeepAlive`, 15 s throttle):

`notes/launchd/install.sh` gets the local tailnet IPv4 address from `tailscale ip -4` and checks that it is valid before it unloads any viewer service.
It picks the service manager from `uname -s`: Darwin gets the LaunchAgents below, unchanged; Linux gets
the systemd user services described under Linux; any other OS is rejected before anything is changed.

- `art.stillwet.studio.tailnet`: `studio.py --host @TAILNET_IP@ --port 8765` (Tailscale only)
- `art.stillwet.studio.public`: `studio.py --public --host 127.0.0.1 --port 8766`, behind the
  Funnel (https://private-viewer.invalid/; off: `tailscale funnel --https=443 off`)

Logs: `~/tmp/studio-viewer-logs/<label>.log`.

    notes/launchd/install.sh                                     # fills in the templates (*.plist.in), (re)loads both
    launchctl kickstart -k gui/$(id -u)/art.stillwet.studio.public                             # restart (after a studio.py change)
    launchctl bootout gui/$(id -u)/art.stillwet.studio.public                                  # stop

## Linux

On Linux the same two viewers run as systemd `--user` services, `art.stillwet.studio.tailnet.service`
and `art.stillwet.studio.public.service`, written to `${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user`.
Each runs `uv` (resolved from the installer's `PATH`) in `%h/src/a/claude-paint` with the same host,
port and `--public` arguments as on Darwin, restarts with `Restart=always` and `RestartSec=15s`, and
appends stdout and stderr to the same per-label log file. The installer validates the tailnet address
before it touches any unit, creates `~/tmp/studio-viewer-logs`, runs `systemctl --user daemon-reload`,
then enables and restarts both units.

Prerequisites: `uv` and `tailscale` on `PATH`, the checkout at `~/src/a/claude-paint`, and a running
systemd user session (`systemctl --user` reaches the user manager). Enabled units start at login. Keeping
them running after logout needs user lingering (`loginctl enable-linger`), which the installer does not
configure.

    notes/launchd/install.sh                                         # fills in *.service.in, daemon-reload, enable + restart
    systemctl --user restart art.stillwet.studio.public.service      # restart (after a studio.py change)
    systemctl --user stop art.stillwet.studio.public.service         # stop (until next login or restart)
    tail -f ~/tmp/studio-viewer-logs/art.stillwet.studio.public.log  # viewer output
    journalctl --user -u art.stillwet.studio.public.service          # systemd's start/stop messages

### Linux studio support

CPU oil studios run on Linux as well as macOS. `scripts/export_r16_studio` builds the painter-only
easel. `scripts/prepare_capy_studio` writes its brief and prints its MCP registration, and
`scripts/prepare_agy_studio` and `scripts/prepare_friedrich_studio` write the launcher for their
painter. Building needs Rust, a C compiler, Perl, Python and `uv`. The house runner
(`atelier/house/r21_chains.py`) also calls `ps` and `lsof`. Launching a real painter still needs that
painter's CLI, its extensions and its account credentials.

The viewer (`studio/studio.py`), the stdio and HTTP MCP adapters and `studio/export_static.py` run on
Linux. There the static exporter reads each painter's working directory from `/proc/<pid>/cwd`; on
macOS it still asks `lsof`. `scripts/prepare_remote_easel_launchd` keeps its name but stages systemd
user `.service` files on Linux and `.plist` files on macOS. Staging never starts a service or opens a
tunnel. The [remote adapter notes](../../harness/easel_mcp/REMOTE.md) cover installing the staged units.

`scripts/test` runs the retained checks on Linux. They include tests that mock both the macOS
LaunchAgent install and the Linux systemd install. `scripts/prove_remote_easel` paints on a real
exported easel and reads the result back through a loopback viewer, with no model painter and no
tunnel. Running launchd itself needs a Mac, and watercolor's Mac GPU backend has no Linux support.
