"""Round 23's report numbers for one painting: its log and its pi session files.

    python3 notes/round23/metrics.py <studio>/paintings/lua/painting.lua <session.jsonl> [...]

- chunks: `--@ chunk` lines in the log.
- failed chunks: `paint` tool results the session marks as errors (a chunk that failed is not in the log).
- piles mixed: `pile{` calls in the log (round 22.1's 118 is this count).
- test strokes: chunks of the log whose own comments call their marks a test, trial, swatch or sample and
  that lay paint (a load or a covering verb). A heuristic: it reads the painter's comments.
- lifts: chunks of the log that wipe a brush (`b:wipe(`) and then stroke with that brush before loading it
  again: a clean brush dragged through paint to take it off (round 22.1's "brush-wipe lifts").
- looks, palette looks: `look` tool calls, and those with `palette: true`.
"""
import json
import re
import sys

log, sessions = sys.argv[1], sys.argv[2:]
chunks = re.split(r"^--@ chunk.*$", open(log).read(), flags=re.M)[1:]

TEST = re.compile(r"--[^\n]*\b(tests?|trials?|swatch(es)?|samples?)\b", re.I)
LAYS = re.compile(r":(re)?load\(|\b(work|stipple)\(")


def is_lift(src):
    # a wipe of brush v, then a stroke or touch of v with no load of v between
    for m in re.finditer(r"\b(\w+):wipe\(", src):
        v = m.group(1)
        rest = src[m.end():]
        s = re.search(rf"\b{v}:(stroke|touch)\(", rest)
        load = re.search(rf"\b{v}:(re)?load\(", rest)
        if s and (not load or s.start() < load.start()):
            return True
    return False


calls, failed, looks, palette = {}, 0, 0, 0
for f in sessions:
    for line in open(f):
        r = json.loads(line)
        m = r.get("message") or {}
        if r.get("type") != "message":
            continue
        if m.get("role") == "assistant":
            for c in m.get("content", []):
                if c.get("type") == "toolCall":
                    calls[c["id"]] = c
                    if c["name"] == "look":
                        looks += 1
                        palette += (c.get("arguments") or {}).get("palette") is True
        elif m.get("role") == "toolResult" and m.get("toolName") == "paint" and m.get("isError"):
            failed += 1

print(json.dumps({
    "chunks": len(chunks),
    "failed chunks": failed,
    "piles mixed": sum(c.count("pile{") for c in chunks),
    "test strokes (chunks)": sum(1 for c in chunks if TEST.search(c) and LAYS.search(c)),
    "lifts (chunks)": sum(1 for c in chunks if is_lift(c)),
    "looks": looks,
    "palette looks": palette,
}, indent=1))
