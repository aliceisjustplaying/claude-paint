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

CPU oil studios support Linux as well as macOS: `scripts/export_r16_studio` builds the painter-only
easel, `scripts/prepare_capy_studio` prepares its brief and MCP registration, and
`scripts/prepare_agy_studio` or `scripts/prepare_friedrich_studio` prepares the corresponding launcher.
Rust, a C compiler, Perl, Python and `uv` are required; the pi house runner also uses `ps` and `lsof`.
Painter launches still require the chosen CLI, its extensions and its account credentials.

`studio/studio.py`, the stdio/HTTP MCP adapters and `studio/export_static.py` run on Linux.
The static exporter reads Linux painter working directories from `/proc` instead of the macOS
`lsof` fallback. `scripts/prepare_remote_easel_launchd` keeps its existing name but stages systemd
user `.service` files on Linux and `.plist` files on macOS. Staging does not start either service
or open a tunnel. The [remote adapter notes](../../harness/easel_mcp/REMOTE.md) describe that setup.

`scripts/test` runs the retained gate on Linux, including both mocked macOS service installation
and Linux unit tests. `scripts/prove_remote_easel` exercises a real exported easel and a loopback
viewer without a model painter or a public tunnel. Native launchd execution requires a Mac;
watercolor's Mac GPU backend is not supported on Linux.
