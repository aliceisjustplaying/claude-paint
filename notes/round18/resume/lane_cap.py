"""Round 18 resume: apply the patched sitting rule (a crashed sitting that added painting counts
toward MAX_SITTINGS; r18r_open.next_sitting on disk) to the runner that is already running with
the old rule in memory.

For each lane: when no sitting is running and the new rule says the painter is done, but the
runner has started its pre-sitting `bin/easel open` anyway, stop that open. The runner's lane then
stops before a painter starts ("lane stops (the easel didn't open)"); this script then reruns
`r18r_open.py --only <lane>`, which reads the records with the new rule, marks the painter
painted and finishes it.
"""
import json
import subprocess
import sys
import time
from pathlib import Path

R18R = Path.home() / "tmp/gallery-fcf9c110/r18r"
RUN = Path.home() / "tmp/gallery-fcf9c110/r18/run"
LOG = RUN / "runner.log"
sys.path.insert(0, str(R18R))
from r18r_open import next_sitting  # noqa: E402  (the patched rule)

LANES = {"DSK": "c766c5", "MIMO": "f705c2", "BUN": "520026", "GLM": "3f743e"}
START = LOG.stat().st_size


def log(msg):
    with open(LOG, "a") as f:
        f.write(f"{time.strftime('%F %T')} LANE-CAP: {msg}\n")
    print(msg, flush=True)


def new_log():
    return LOG.read_bytes()[START:].decode(errors="replace")


def opens(hexid):
    studio = str(Path.home() / f"src/a/paint-studio-{hexid}")
    ps = subprocess.run(["/bin/ps", "-Ao", "pid,command"], capture_output=True, text=True).stdout
    return [l.split(None, 1)[0] for l in ps.splitlines()[1:]
            if f"{studio}/bin/easel open" in l or f"{studio}/bin/easel serve" in l]


log("watching DSK, MIMO, BUN, GLM: a crashed sitting that painted counts toward the 4")
stopped, rerun = set(), set()
while not all(l in rerun or (RUN / l / "p1.done").exists() for l in LANES):
    for lane, hexid in LANES.items():
        if lane in rerun or (RUN / lane / "p1.done").exists():
            continue
        s = json.loads((RUN / lane / "p1_sittings.json").read_text())
        if any(x.get("status") == "running" for x in s):
            continue
        if lane not in stopped and next_sitting(s) is None and (p := opens(hexid)):
            subprocess.run(["kill", *p])
            stopped.add(lane)
            log(f"{lane}: done under the new rule; stopped the runner's pre-sitting open {p} (no new sitting)")
        if lane in stopped and f"{lane}: lane stops" in new_log():
            subprocess.Popen(["uv", "run", "r18r_open.py", "--only", lane], cwd=R18R,
                             stdout=open(LOG, "a"), stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
            rerun.add(lane)
            log(f"{lane}: reran r18r_open.py --only {lane} to mark it painted and finish it")
    time.sleep(5)
log("all lanes handled")
