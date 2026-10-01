"""Shapes copied from other shapes in a painting's log (round 21 triage, P02).

The brief's rule: a mask or stroke isn't made by mirroring or rotating another's coordinates (a
reflection in water is drawn as its own shape, not as m:at(x, 2*H - y)). Moving a shape (x + 40)
and reusing helpers are allowed. This finds the forms round 19's audit.py looked for, less the
shifted ones: masks read at reflected or swapped coordinates and points with a reflected
coordinate. Flags are for a person to read, not verdicts.

    uv run --no-project copied_shapes.py <painting.lua>
"""
import json
import re
import sys

AT = re.compile(r":at\s*\(((?:[^()]|\([^()]*\))*)\)")
COORD = r"(?:[a-z]?[xy][a-z0-9_]*)"           # x, y, x0, yy, px, gy, y_top ...
REFLECTED = re.compile(rf"^\s*-\s*{COORD}\b|[\w)\]]\s*-\s*{COORD}\s*$")   # -x, 804 - y, 2*H - y
POINT_REFLECTED = re.compile(rf"\{{[^{{}}]*?,\s*(?:\d{{3,}}(?:\.\d+)?|2\s*\*\s*\w+)\s*-\s*{COORD}\s*\}}"
                             rf"|\{{\s*(?:\d{{3,}}(?:\.\d+)?|2\s*\*\s*\w+)\s*-\s*{COORD}\s*,[^{{}}]*\}}")


def _args(s):
    out, depth, cur = [], 0, ""
    for ch in s:
        depth += ch in "([{"
        depth -= ch in ")]}"
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    return [a.strip() for a in out + [cur]]


def _axis(arg):
    return {n.lstrip("abcdefghijklmnopqrstuvwz")[:1] for n in re.findall(rf"\b{COORD}\b", arg)} & {"x", "y"}


def copied_shapes(lua):
    """[{"chunk", "kind", "code"}] for every mirrored or rotated coordinate read in the log."""
    found, chunk = [], 0
    for line in lua.splitlines():
        m = re.match(r"--@ chunk (\d+)", line)
        if m:
            chunk = int(m.group(1))
            continue
        code = line.split("--", 1)[0]
        for m in AT.finditer(code):
            args = _args(m.group(1))
            if len(args) != 2:
                continue
            ax, ay = _axis(args[0]), _axis(args[1])
            if any(REFLECTED.search(a) for a in args):
                found.append({"chunk": chunk, "kind": "mirrored", "code": m.group(0)})
            elif ("y" in ax and "x" not in ax) or ("x" in ay and "y" not in ay):
                found.append({"chunk": chunk, "kind": "rotated", "code": m.group(0)})
        for m in POINT_REFLECTED.finditer(code):
            found.append({"chunk": chunk, "kind": "mirrored point", "code": m.group(0)})
    return found


if __name__ == "__main__":
    print(json.dumps(copied_shapes(open(sys.argv[1]).read()), indent=1))
