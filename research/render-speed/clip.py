# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy", "pillow"]
# ///
import argparse
import importlib.util
import json
from pathlib import Path
from types import SimpleNamespace
import time

import numpy as np

parser = argparse.ArgumentParser()
parser.add_argument("frames", type=Path)
parser.add_argument("output", type=Path)
args = parser.parse_args()
start = time.monotonic()
spec = importlib.util.spec_from_file_location("pace", Path(__file__).resolve().parents[2] / "scripts/replay_pace.py")
pace = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pace)
moments = pace.moments(args.frames)
changes = pace.changes(args.frames, moments)
hands = np.array([x[1] for x in moments])
config = SimpleNamespace(length=19.5, hold=3, floor=0.2, gamma=0.7, hand_pace=0.6, ramp=1, ramp_span=0.35, max_hold=1)
for _ in range(20):
    allocations = pace.quantize(pace.schedule(changes, hands, config), 30, 2)
    if sum(allocations) == 495:
        break
    config.length += (495 - sum(allocations)) / 30
if sum(allocations) != 495 or allocations[0] != 0:
    raise RuntimeError("invalid process schedule")
final = moments[-1][0]
sequence = [final] * 15
for (name, _), count in zip(moments[:-1], allocations):
    sequence.extend([name] * count)
sequence.extend([final] * 90)
assert len(sequence) == 600
args.output.mkdir(parents=True, exist_ok=True)
for i, name in enumerate(sequence):
    dest = args.output / f"{i:04}.png"
    dest.symlink_to((args.frames / name).resolve())
result = {"wall_seconds": time.monotonic() - start, "captured_moments": len(moments), "shown_process_moments": sum(x > 0 for x in allocations), "video_frames": len(sequence), "opening_frames": 15, "process_frames": 495, "hold_frames": 90, "fps": 30}
(args.output / "schedule.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
