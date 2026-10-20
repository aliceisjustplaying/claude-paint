#!/usr/bin/env bash
result=$1
unset KACHE_LOCAL_ONLY KACHE_TRUST_DOMAIN
export KACHE_CONFIG="$result/kache.toml" KACHE_HOST_CONFIG=''
export KACHE_CACHE_DIR="$result/store" KACHE_RUNTIME_DIR="$result/runtime"
export KACHE_SOCKET_PATH="$result/runtime/daemon.sock"
export KACHE_SEED_NEW_TARGETS=0
export KACHE_DAEMON_IDLE_TIMEOUT=60
cat >"$KACHE_CONFIG" <<'EOF'
[cache]
local_only = true
seed_new_targets = false
daemon_idle_timeout_secs = 60
EOF
printf 'mode=isolated local-only daemon\nconfig=%s\nhost_config=disabled\nstore=%s\nruntime=%s\nsocket=%s\n' "$KACHE_CONFIG" "$KACHE_CACHE_DIR" "$KACHE_RUNTIME_DIR" "$KACHE_SOCKET_PATH" >"$result/cache-isolation.txt"
