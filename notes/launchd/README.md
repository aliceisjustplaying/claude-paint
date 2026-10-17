# Studio viewers under launchd

In historical notes, private-viewer.invalid and 192.0.2.1 are placeholders for redacted network addresses, not live endpoints.

Two LaunchAgents keep the studio viewer up across sessions and logins, restarting it if it dies
(`KeepAlive`, 15 s throttle):

`notes/launchd/install.sh` gets the local tailnet IPv4 address from `tailscale ip -4` and checks that it is valid before it unloads any viewer service.

- `art.stillwet.studio.tailnet`: `studio.py --host @TAILNET_IP@ --port 8765` (Tailscale only)
- `art.stillwet.studio.public`: `studio.py --public --host 127.0.0.1 --port 8766`, behind the
  Funnel (https://private-viewer.invalid/; off: `tailscale funnel --https=443 off`)

Logs: `~/tmp/studio-viewer-logs/<label>.log`.

    notes/launchd/install.sh                                     # fills in the templates (*.plist.in), (re)loads both
    launchctl kickstart -k gui/$(id -u)/art.stillwet.studio.public                             # restart (after a studio.py change)
    launchctl bootout gui/$(id -u)/art.stillwet.studio.public                                  # stop
