#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
exec uv run --no-project --python .venv/bin/python python notes/media-backlog/render_r22_substitute.py
