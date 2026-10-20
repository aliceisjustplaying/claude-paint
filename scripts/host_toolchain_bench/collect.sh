#!/usr/bin/env bash
set -euo pipefail
find "${1:-$HOME/capy-measure}" -maxdepth 4 -type f \( -name build.time -o -name gate.time -o -path '*/mold/gate.log' \) -print -exec cat {} \;
