"""Which of the easel's ready-made handlings a painting's log calls on (round 21 triage, P04).

The easel's `work` hands (broad, body, detail, hatch, glaze, scumble, blend) and the characters
of `outline` and `body_of` choose a brush, stroke lengths and edge for the painter; a painting
that leans on them gets part of its look from them (`tool=` swaps a hand's brush for the
painter's own, counted as work_with_own_tool). This counts the call sites in the log
(paintings/lua/painting.lua), not how often each ran: a call in a loop or a helper counts once.

    uv run --no-project defaults_used.py <painting.lua>      # prints the counts as JSON
"""
import json
import re
import sys

HANDS = ("broad", "body", "detail", "hatch", "glaze", "scumble", "blend")
CHARS = ("firm", "searching", "broken", "soft")


def _code(lua):
    """The log without comments and with string contents blanked (so neither can look like a call)."""
    out, i, n = [], 0, len(lua)
    while i < n:
        if lua.startswith("--[[", i) or lua.startswith("--[==[", i):
            close = "]]" if lua.startswith("--[[", i) else "]==]"
            j = lua.find(close, i)
            i = n if j < 0 else j + len(close)
        elif lua.startswith("--", i):
            j = lua.find("\n", i)
            i = n if j < 0 else j
        elif lua[i] in "\"'":
            q, j = lua[i], i + 1
            while j < n and lua[j] != q:
                j += 2 if lua[j] == "\\" else 1
            out.append(q + lua[i + 1:j].replace("(", " ").replace(")", " ").replace("{", " ").replace("}", " ") + q)
            i = j + 1
        else:
            out.append(lua[i])
            i += 1
    return "".join(out)


def _calls(code, name):
    """The argument text of every call of `name(` or `name{` (balanced brackets)."""
    for m in re.finditer(r"(?<![\w.:])" + re.escape(name) + r"\s*([({])", code):
        depth, j = 0, m.start(1)
        while j < len(code):
            if code[j] in "({[":
                depth += 1
            elif code[j] in ")}]":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        yield code[m.start(1) + 1:j]


def _named(args, key):
    m = re.search(r"\b" + key + r"\s*=\s*([\"'])(\w+)\1", args)
    return m.group(2) if m else None


def defaults_used(lua):
    code = _code(lua)
    work, own_tool = {}, 0
    for args in _calls(code, "work"):
        hand = _named(args, "hand")
        k = hand if hand in HANDS else "body (default)" if hand is None else "another value"
        if hand is None and re.search(r"\bhand\s*=", args):
            k = "set from a variable"
        work[k] = work.get(k, 0) + 1
        if re.search(r"\btool\s*=", args):
            own_tool += 1
    shapes = {}
    for fn, default in (("outline", "firm"), ("body_of", "soft")):
        counts = {}
        for args in _calls(code, fn):
            char = _named(args, "char")
            k = char if char in CHARS else f"{default} (default)" if char is None else "another value"
            if char is None and re.search(r"\bchar\s*=", args):
                k = "set from a variable"
            counts[k] = counts.get(k, 0) + 1
        shapes[fn] = counts
    return {
        "counted": "call sites in the log, not runs",
        "work": work,
        "work_with_own_tool": own_tool,
        "blend": sum(1 for _ in _calls(code, "blend")),
        "outline": shapes["outline"],
        "body_of": shapes["body_of"],
        "brush": sum(1 for _ in _calls(code, "brush")),
    }


if __name__ == "__main__":
    print(json.dumps(defaults_used(open(sys.argv[1]).read()), indent=1))
