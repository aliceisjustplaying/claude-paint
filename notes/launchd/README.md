# Studio viewers under launchd

Two LaunchAgents keep the studio viewer up across sessions and logins, restarting it if it dies
(`KeepAlive`, 15 s throttle):

- `art.stillwet.studio.tailnet`: `studio.py --host 100.82.115.34 --port 8765` (Tailscale only)
- `art.stillwet.studio.public`: `studio.py --public --host 127.0.0.1 --port 8766`, behind the
  Funnel (https://m3p.tailec2dc.ts.net/; off: `tailscale funnel --https=443 off`)

Logs: `~/tmp/studio-viewer-logs/<label>.log`.

    notes/launchd/install.sh                                     # fills in the templates (*.plist.in), (re)loads both
    launchctl kickstart -k gui/$(id -u)/art.stillwet.studio.public                             # restart (after a studio.py change)
    launchctl bootout gui/$(id -u)/art.stillwet.studio.public                                  # stop
