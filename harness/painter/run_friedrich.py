#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys

HERE = Path(__file__).resolve().parent
CONFIG = json.loads((HERE / "run-config.json").read_text())
STUDIO = Path(CONFIG["studio"])
SOURCE = Path(CONFIG["source"])
RUN = Path(CONFIG["run"])
for name in "EASEL_ROOT EASEL_SESSION EASEL_BOX EASEL_WIDTH PAINTER_SITTING_RECOVERY PAINTER_MAX_IMAGES PAINTER_MAX_IMAGE_MB PAINTER_INPUT_TPM PAINTER_LIMIT_PROBE_S PAINTER_LIMIT_GIVE_UP_H PAINTER_COMPACT_RESERVE PAINTER_COMPACTION_NOTES PAINTER_OPEN_PASSAGES PAINTER_CRITIC PAINTER_INBOX_DIR PAINTER_INBOX_SITTING".split():
    os.environ.pop(name, None)
sys.path.insert(0, str(HERE))
spec = importlib.util.spec_from_file_location("house_runner", HERE / "r21_chains.py")
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)
R.BASE = SOURCE
R.BRANCH = CONFIG["source_revision"]
R.H = SOURCE / "harness/painter"
R.RUN = RUN
R.HARNESS = ["--no-extensions", "-e", str(R.H / "painter.ts"), "-e", str(R.H / "compaction.ts"), "--system-prompt", str(R.H / "system_prompt.md"), "--tools", R.TOOLS, "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]
MODEL = R.model("anthropic", "claude-opus-5-5", "xhigh", black=True)
if CONFIG.get("critic"):
    # the critic (harness/painter/visit.ts) on the painter's model; its sessions and notes beside the run's records
    MODEL["env"] = {**(MODEL["env"] or {}), "PAINTER_CRITIC": json.dumps({"args": (["-e", str(R.BLACK)] if MODEL["black"] else []) + MODEL["args"], "dir": str(RUN / "critic")})}
R.LANES = {"OPUS": R.lane("friedrich", MODEL, record_kind="none", reference=False)}
R.MY_LANES = ["OPUS"]
launch = R.run


def run(cmd, cwd, out, err, env=None):
    sitting = re.fullmatch(r"p\d+_s(\d+)(?:_part\d+)?_final\.txt", Path(out).name)
    if sitting:
        run_dir = Path(out).parent.resolve()
        (Path(cwd) / ".painter-run").write_text(str(run_dir) + "\n")
        env = {**(env or {}), "PAINTER_INBOX_DIR": str(run_dir), "PAINTER_INBOX_SITTING": sitting[1]}
    return launch(cmd, cwd, out, err, env=env)


R.run = run
assert subprocess.check_output(["git", "-C", str(SOURCE), "rev-parse", "HEAD"]).decode().strip() == R.BRANCH, "prepared source revision changed"
assert (STUDIO / "bin/box").read_text().strip() == "friedrich"
assert R.brief("friedrich", STUDIO, reference=False).replace("~/src/a/" + STUDIO.name, str(STUDIO)) == (STUDIO / "BRIEF.md").read_text(), "prepared brief changed"
if "--check" in sys.argv:
    m = R.LANES["OPUS"]["model"]
    print(json.dumps({**CONFIG, "model": m, "chunks": R.count_chunks(STUDIO), "painting": R.count_painting(STUDIO), "first_sitting_cmd": R.painter_cmd(m), "first_sitting_env": R.painter_env(m, R.PAINTER_MSG), "later_sitting_env": R.painter_env(m, R.SITTING_MESSAGE), "max_crashes": R.MAX_CRASHES, "limit_probe_s": R.LIMIT_PROBE_S, "limit_give_up_h": R.LIMIT_GIVE_UP_H}, indent=2))
    raise SystemExit(0)
RUN.mkdir(parents=True, exist_ok=True)
(RUN / "OPUS").mkdir(exist_ok=True)
with (RUN / "runner.lock").open("a") as lock:
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    mapping = RUN / "studios.json"
    if mapping.exists():
        assert json.loads(mapping.read_text()) == {"OPUS1": STUDIO.name}, "run belongs to another studio"
    else:
        mapping.write_text(json.dumps({"OPUS1": STUDIO.name}) + "\n")
    if R.paint("OPUS", 1, STUDIO, RUN / "OPUS") is None:
        raise SystemExit(1)
