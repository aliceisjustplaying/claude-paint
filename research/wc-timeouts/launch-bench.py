import argparse
import json
import os
from pathlib import Path
import plistlib
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("--profile", action="store_true")
parser.add_argument("--limit", type=int, default=900)
parser.add_argument("--app", action="store_true")
parser.add_argument("--checkpoint", type=Path)
args = parser.parse_args()
base = Path.home() / ".capy/work/wc-timeouts"
label = "art.stillwet.wc-timeouts-reproduction"
target = f"gui/{os.getuid()}/{label}"
root_label = "rw1-L628-launchd-profile" if args.profile else "rw1-L628-launchd"
if args.app:
    root_label += "-app"
root = base / "reproductions" / root_label
if root.exists():
    raise RuntimeError(f"Refusing to reuse {root}")
program = ["/etc/profiles/per-user/alice/bin/uv", "run", "--no-project", "--python", str(base / ".venv/bin/python"), str(base / "reproduce.py"), "rw1-L628", "--label", root_label, "--sample", "--limit", str(args.limit)]
if args.profile:
    program.append("--profile")
if args.checkpoint:
    program.extend(["--checkpoint", str(args.checkpoint)])
config = dict(Label=label, ProgramArguments=program, RunAtLoad=True, KeepAlive=False, WorkingDirectory=str(base), StandardOutPath=str(base / f"{root_label}.out"), StandardErrorPath=str(base / f"{root_label}.err"), EnvironmentVariables=dict(PATH="/usr/bin:/bin:/usr/sbin:/sbin"))
if args.app:
    config["ProcessType"] = "Interactive"
path = base / f"{root_label}.plist"
path.write_bytes(plistlib.dumps(config))
if subprocess.run(["/bin/launchctl", "print", target], capture_output=True).returncode == 0:
    raise RuntimeError(f"Refusing to replace existing {target}")
command = ["/bin/launchctl", "bootstrap", f"gui/{os.getuid()}", str(path)]
subprocess.run(command, check=True)
try:
    started = time.monotonic()
    while not (root / "timing.json").exists():
        if time.monotonic() - started > args.limit + 90:
            raise RuntimeError("Launch reproduction did not produce a timing receipt")
        time.sleep(2)
    result = json.loads((root / "timing.json").read_text())
    result["launch_command"] = command
    result["launch_configuration"] = config
    result["launchctl_print"] = subprocess.run(["/bin/launchctl", "print", target], capture_output=True, text=True).stdout
    (root / "timing.json").write_text(json.dumps(result, indent=2))
    print(json.dumps(result, indent=2))
finally:
    subprocess.run(["/bin/launchctl", "bootout", target], check=True)
