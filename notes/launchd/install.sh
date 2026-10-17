#!/usr/bin/env bash
# Install (or reinstall) the studio viewers' LaunchAgents from the templates here.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
uvbin=$(dirname "$(command -v uv)")
tailnet_ip=$(tailscale ip -4 2>/dev/null) || tailnet_ip=
if [[ ! $tailnet_ip =~ ^100\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$ ]] ||
   (( 10#${BASH_REMATCH[1]} < 64 || 10#${BASH_REMATCH[1]} > 127 ||
      10#${BASH_REMATCH[2]} > 255 || 10#${BASH_REMATCH[3]} > 255 )); then
  echo "install.sh: could not get exactly one valid local tailnet IPv4 address from \`tailscale ip -4\`; no viewer service was changed." >&2
  exit 1
fi
for t in "$here"/art.stillwet.studio.*.plist.in; do
  label=$(basename "$t" .plist.in)
  dst=~/Library/LaunchAgents/$label.plist
  sed -e "s|@HOME@|$HOME|g" -e "s|@UVBIN@|$uvbin|g" -e "s|@TAILNET_IP@|$tailnet_ip|g" "$t" > "$dst"
  plutil -lint "$dst" >/dev/null
  launchctl bootout "gui/$(id -u)/$label" 2>/dev/null || true
  launchctl bootstrap "gui/$(id -u)" "$dst"
  echo "loaded $label"
done
