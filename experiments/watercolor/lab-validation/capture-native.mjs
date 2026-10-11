// MIT License, Copyright (c) 2026 Ryan McCarty.
// Capture wrapper for the pinned Watercolor Lab mechanical port.
// Full license: ../THIRD_PARTY_NOTICES.md.
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
const out = resolve(process.argv[2] ?? `${process.env.HOME}/.capy/work/watercolor-lab-port/captures/native`);
const command = spawnSync("cargo", ["run", "--release", "--manifest-path", resolve(import.meta.dirname, "../lab-core/Cargo.toml"), "--bin", "lab-parity", "--", "--marks", resolve(import.meta.dirname, "marks.json"), out, ...process.argv.slice(3)], { stdio: "inherit", env: { ...process.env, CARGO_TARGET_DIR: process.env.CARGO_TARGET_DIR ?? `${process.env.HOME}/.capy/work/watercolor-lab-port/target` } });
if (command.status !== 0) process.exit(command.status ?? 1);
const conversion = spawnSync("python3", ["-c", "from pathlib import Path; from PIL import Image; import sys\nfor p in Path(sys.argv[1]).glob('*.ppm'):\n Image.open(p).save(p.with_suffix('.png')); p.unlink()", out], { stdio: "inherit" });
process.exit(conversion.status ?? 1);
