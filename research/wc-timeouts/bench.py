import argparse
import json
from pathlib import Path
import shutil
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("--suite", choices=["policy", "geometry", "state", "all"], default="all")
args = parser.parse_args()
base = Path.home() / ".capy/work/wc-timeouts"
retained = base / "profile-watercolor-studio"
binary = retained if retained.exists() else base / "target/release/watercolor-studio"
cases = []
if args.suite in ["policy", "all"]:
    for policy, flags in [("normal", []), ("utility", ["/usr/sbin/taskpolicy", "-c", "utility"]), ("background", ["/usr/sbin/taskpolicy", "-b"]), ("app", ["/usr/sbin/taskpolicy", "-a"])]:
        cases.append(dict(name=f"policy-{policy}", flags=flags))
if args.suite in ["geometry", "all"]:
    cases.extend([
        dict(name="length-100", length=100),
        dict(name="width-64", width=64),
        dict(name="quarter-paper", paper=[550, 325], length=250),
        dict(name="full-paper-short", length=250),
        dict(name="many-points", points=100),
        dict(name="default-duration", duration=None),
        dict(name="dry-strokes", dry=True),
        dict(name="off-paper", y=-100),
    ])
if args.suite in ["state", "all"]:
    cases.extend([
        dict(name="wet-paper", wet=True),
        dict(name="many-brushes", brushes=3500),
        dict(name="wait-only-dry", strokes=0, wait=15),
        dict(name="wait-only-wet", strokes=0, wait=15, wet=True),
        dict(name="no-work", strokes=0),
    ])
env = {"PATH": "/usr/bin:/bin", "HOME": str(Path.home()), "RAYON_NUM_THREADS": "4"}
for case in cases:
    root = base / "benches" / case["name"]
    if root.exists():
        raise RuntimeError(f"Refusing to reuse {root}")
    if shutil.disk_usage(base).free < 15 * 1024**3:
        raise RuntimeError("Less than 15 GiB free")
    root.mkdir(parents=True)
    (root / "box").write_text("watercolor\n")
    width, height = case.get("paper", [1100, 650])
    brush_width = case.get("width", 16)
    length = case.get("length", 575)
    point_count = case.get("points", 2)
    strokes = case.get("strokes", 4)
    duration = case.get("duration", 0.01)
    attrs = "{pressure=0.5,dry=" + ("true" if case.get("dry") else "false") + (f",duration={duration}" if duration is not None else "") + "}"
    source = f'paper{{width={width},height={height},kind="cold-press",seed=1}}\n'
    if case.get("wet"):
        source += f"for x=50,{width-50},100 do for y=50,{height-50},100 do drop{{x=x,y=y,water=6,radius=55}} end end\n"
    if case.get("brushes"):
        source += f'for i=1,{case["brushes"]} do brush{{name="unused"..i,width=16}} end\n'
    source += f'local b=brush{{name="b",width={brush_width}}}\n'
    source += f"for i=1,{strokes} do b:load{{water=2,pigments={{['sap-green']=0.1}}}} local p={{}} for k=0,{point_count-1} do p[#p+1]={{20+{length}*k/{point_count-1},{case.get('y', 100)}+i*20}} end b:stroke(p,{attrs}) end\n"
    source += f"wait({case.get('wait', 0)})\n"
    path = root / "case.lua"
    path.write_text(source)
    command = [*case.get("flags", []), str(binary), "--box", "watercolor", "run", str(root), str(path)]
    started = time.perf_counter()
    with (root / "stdout.txt").open("w") as stdout, (root / "stderr.txt").open("w") as stderr:
        result = subprocess.run(command, stdout=stdout, stderr=stderr, env=env, timeout=720)
    timing = dict(case=case, command=command, env=env, wall_seconds=time.perf_counter()-started, returncode=result.returncode, stderr=(root / "stderr.txt").read_text(), stdout=(root / "stdout.txt").read_text())
    (root / "timing.json").write_text(json.dumps(timing, indent=2))
    print(json.dumps(timing), flush=True)
