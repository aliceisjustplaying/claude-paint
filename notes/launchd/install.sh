#!/usr/bin/env bash
# Install (or reinstall) the studio viewers from the templates here.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
os=$(uname -s)
case "$os" in
  Darwin|Linux) ;;
  *) echo "install.sh: unsupported operating system: $os; no viewer service was changed." >&2; exit 1 ;;
esac
uvbin=$(dirname "$(command -v uv)")
tailnet_ip=$(tailscale ip -4 2>/dev/null) || tailnet_ip=
if [[ ! $tailnet_ip =~ ^100\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$ ]] ||
   (( 10#${BASH_REMATCH[1]} < 64 || 10#${BASH_REMATCH[1]} > 127 ||
      10#${BASH_REMATCH[2]} > 255 || 10#${BASH_REMATCH[3]} > 255 )); then
  echo "install.sh: could not get exactly one valid local tailnet IPv4 address from \`tailscale ip -4\`; no viewer service was changed." >&2
  exit 1
fi
if [[ $os == Linux ]]; then
  unit_dir=${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user
  mkdir -p "$unit_dir" "$HOME/tmp/studio-viewer-logs"
  systemd_uvbin=${uvbin//\\/\\\\}
  systemd_uvbin=${systemd_uvbin//\"/\\\"}
  systemd_uvbin=${systemd_uvbin//%/%%}
  units=()
  for t in "$here"/art.stillwet.studio.*.service.in; do
    unit=$(basename "$t" .in)
    config=$(< "$t")
    config=${config//@UVBIN@/"$systemd_uvbin"}
    config=${config//@TAILNET_IP@/"$tailnet_ip"}
    printf '%s\n' "$config" > "$unit_dir/$unit"
    units+=("$unit")
  done
  systemctl --user daemon-reload
  systemctl --user enable "${units[@]}"
  systemctl --user restart "${units[@]}"
  printf 'loaded %s\n' "${units[@]}"
  exit 0
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
