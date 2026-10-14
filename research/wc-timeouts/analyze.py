import argparse
import json
import math
from pathlib import Path
import re
import shlex

parser = argparse.ArgumentParser()
parser.add_argument("--root", type=Path, default=Path.home() / ".capy/work/wc-timeouts")
args = parser.parse_args()
base = args.root
rows = json.loads((base / "census.json").read_text())
training = []
metrics = {}
for row in rows:
    values = {key: float(value) for key, value in re.findall(r"(\w+)=([\d.]+)", row["trace"])}
    metrics[row["name"]] = values
    if row["trace_exit"] or row.get("result", {}).get("isError") or not values.get("stamps"):
        continue
    area = {1: 1100*650, 2: 1180*700, 3: 1100*660}[row["lane"]] / (1100*650)
    training.append(([1, values["stamps"]*area, values["duration"]+values["waits"], values["drops"]*area], row["elapsed"], row["name"]))
size = 4
matrix = [[sum(x[i]*x[j] for x, y, name in training) for j in range(size)] + [sum(x[i]*y for x, y, name in training)] for i in range(size)]
for i in range(size):
    scale = matrix[i][i]
    matrix[i] = [value/scale for value in matrix[i]]
    for j in range(size):
        if i != j:
            scale = matrix[j][i]
            matrix[j] = [value-scale*other for value, other in zip(matrix[j], matrix[i])]
coefficients = [matrix[i][-1] for i in range(size)]
model = dict(label="Inferred least-squares model of historical successful calls, not a runtime measurement or extrapolation guarantee", training=len(training), coefficients=dict(intercept=coefficients[0], area_weighted_stamps=coefficients[1], painter_seconds=coefficients[2], area_weighted_drops=coefficients[3]), rmse=math.sqrt(sum((sum(b*v for b, v in zip(coefficients, x))-y)**2 for x, y, name in training)/len(training)))
model["predictions"] = {}
for name in ["rw1-L628", "rw1-L770", "rw1-L792", "rw2-L390", "rw2-L751", "rw3-L262", "rw3-L452"]:
    row = next(row for row in rows if row["name"] == name)
    values = metrics[name]
    area = {1: 1100*650, 2: 1180*700, 3: 1100*660}[row["lane"]] / (1100*650)
    features = [1, values["stamps"]*area, values["duration"]+values["waits"], values["drops"]*area]
    model["predictions"][name] = sum(b*v for b, v in zip(coefficients, features))
timings = []
for directory in [base / "reproductions", base / "benches"]:
    for path in sorted(directory.glob("*/timing.json")):
        timing = json.loads(path.read_text())
        timing["receipt"] = str(path)
        timings.append(timing)
history = base / "history/rw1/timings.json"
if history.exists():
    timings.extend(json.loads(history.read_text()))
(base / "measurements.json").write_text(json.dumps(dict(model=model, metrics=metrics, timings=timings), indent=2))
with (base / "timings.md").open("w") as output:
    output.write("# Measured runs\n\nEach command used `PATH=/usr/bin:/bin HOME=/Users/alice RAYON_NUM_THREADS=4`, unless its receipt says otherwise. Lua source and serialized-state writes are confined to the research directory.\n\n| Case | Wall seconds | Exit | Exact command |\n|---|---:|---:|---|\n")
    for timing in timings:
        name = timing.get("name", timing.get("case", {}).get("name"))
        output.write(f"| {name} | {timing['wall_seconds']:.6f} | {timing['returncode']} | `{shlex.join(timing['command'])}` |\n")
        if timing.get("launch_command"):
            output.write(f"| launch setup for {name} | Not separately timed | 0 | `{shlex.join(timing['launch_command'])}` |\n")
print(json.dumps(model, indent=2))
print(f"Collected {len(timings)} timing receipts")
