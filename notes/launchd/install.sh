#!/usr/bin/env bash
# Install (or reinstall) the studio viewers' LaunchAgents from the templates here.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
uvbin=$(dirname "$(command -v uv)")
for t in "$here"/art.stillwet.studio.*.plist.in; do
  label=$(basename "$t" .plist.in)
  dst=~/Library/LaunchAgents/$label.plist
  sed -e "s|@HOME@|$HOME|g" -e "s|@UVBIN@|$uvbin|g" "$t" > "$dst"
  plutil -lint "$dst" >/dev/null
  launchctl bootout "gui/$(id -u)/$label" 2>/dev/null || true
  launchctl bootstrap "gui/$(id -u)" "$dst"
  echo "loaded $label"
done
