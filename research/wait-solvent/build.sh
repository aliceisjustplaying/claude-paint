#!/usr/bin/env bash
set -euo pipefail
cargo build --release -p easel --no-default-features --features box-every
mkdir -p research/wait-solvent/painter/bin
cp target/release/easel research/wait-solvent/painter/bin/easel
printf 'every\n' > research/wait-solvent/painter/bin/box
cp THIRD_PARTY_NOTICES.md research/wait-solvent/painter/bin/THIRD_PARTY_NOTICES.md
shasum -a 256 research/wait-solvent/painter/bin/easel
cargo build --release -p paint --example wait_probe
