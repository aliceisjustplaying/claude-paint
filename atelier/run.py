# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""A studio's painting, run by its launch agent (atelier.py launch_agent): the unchanged house paint() loop
(house/r21_chains.py) with the studio's one lane, the atelier's system prompt, messages and tools, then atelier finish
once the painter is done. Resumable as the house loop is: rerunning it carries on from run/<lane>/p1_sittings.json.

    run.py [atelier paths] <studio> <n>
"""
import argparse
import json
import os
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import atelier as A  # noqa: E402

# as the house wrapper does: nothing the painter's pi or easel reads comes from the launching environment
INHERITED = ("EASEL_ROOT EASEL_SESSION EASEL_BOX EASEL_WIDTH PAINTER_SITTING_RECOVERY PAINTER_MAX_IMAGES PAINTER_MAX_IMAGE_MB "
             "PAINTER_INPUT_TPM PAINTER_LIMIT_PROBE_S PAINTER_LIMIT_GIVE_UP_H PAINTER_COMPACT_RESERVE PAINTER_COMPACTION_NOTES").split()
TOOLS = "paint,look,probe,note,status,log,read,notebook"


def configure(sid, work):
    """The house runner set up for this painting: its records in work/run, the atelier's texts, the studio's lane."""
    for name in INHERITED:
        os.environ.pop(name, None)
    R = A.house()
    info = A.STUDIOS[sid]
    R.RUN = work / "run"
    R.H = A.REPO / "harness/painter"
    R.TOOLS = TOOLS
    R.HARNESS = ["--no-extensions", "-e", str(R.H / "painter.ts"), "-e", str(R.H / "compaction.ts"),
                 "--system-prompt", str(A.TEXTS / "system_prompt.md"), "--tools", TOOLS,
                 "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]
    R.PAINTER_MSG = A.text("launch_message.md")
    R.SITTING_MESSAGE = A.text("sitting_message.md")
    R.CONTINUE_MESSAGE = A.text("continue_message.md")
    R.LANES = {info["lane"]: R.lane(A.BOX, R.model(info["provider"], info["model"], info["thinking"], black=True),
                                    record_kind="none", reference=False)}
    R.MY_LANES = [info["lane"]]
    return R


def paint(atelier, sid, n):
    """The painting's sittings until the painter is done, then its finish; 0 once it hangs."""
    work = atelier.root / "works" / f"{sid}-{n:03d}"
    folder = Path(json.loads((work / "manifest.json").read_text())["folder"])
    R = configure(sid, work)
    lane = A.STUDIOS[sid]["lane"]
    R.log(f"{lane}1: atelier {sid} painting {n} in {folder}: the house paint() loop")
    if R.paint(lane, 1, folder, work / "run" / lane) is None:
        return 1
    try:
        atelier.finish(sid)
    except A.Refused as why:
        R.log(f"{lane}1: not hung: {why}")
        return 1
    R.log(f"{lane}1: painting {n} hung")
    return 0


def main(argv=None):
    ap = A.with_paths(argparse.ArgumentParser(description=__doc__.split("\n\n")[0]))
    ap.add_argument("studio", choices=list(A.STUDIOS))
    ap.add_argument("number", type=int)
    a = ap.parse_args(argv)
    return paint(A.Atelier(a.root, a.studios, a.data), a.studio, a.number)


if __name__ == "__main__":
    sys.exit(main())
