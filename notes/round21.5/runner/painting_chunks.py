"""Which chunks of a painting's log put marks on the canvas.

The sitting stop rule used to count `--@ chunk` lines, so a sitting whose chunks only asked
questions (`print(wait(60))`, `print(drying(400, 300))`, `print(table.concat(tubes(), ", "))`)
or only mixed piles and loaded brushes counted as painting and earned another sitting.

A chunk paints if its code (comments and strings removed) refers to a mark-making verb of
the easel (notes/easel_guide.md): the globals `canvas work blend stipple lose erase fix` (and the finishing verbs of older
builds), or
the methods `stroke touch paint sketch line rule hatch` (brush, outline and pencil marks).
Painters wrap verbs in helpers (`function dab(x, y) b:touch(x, y) end`), so every function a
painting chunk defines, and every alias it makes of a painting name, becomes a painting name
for the chunks after it. It errs toward "paints": a chunk that only defines a helper around a
verb counts, as does one that mentions a verb it doesn't call. A chunk that queries, mixes,
loads, waits or defines masks and nothing else doesn't.
"""
import re

VERBS = {"canvas", "work", "blend", "stipple", "lose", "erase", "fix",
         "varnish", "cracks", "relief"}   # finishing verbs of older builds (not in the painter build)
METHODS = {"stroke", "touch", "paint", "sketch", "line", "rule", "hatch"}

_CHUNK = re.compile(r"^--@ chunk (\d+)", re.M)
# Long comments/strings first, then comments, then quoted strings (with escapes).
_STRIP = re.compile(
    r"--\[(=*)\[.*?\]\1\]"          # --[[ ... ]] / --[==[ ... ]==]
    r"|\[(=*)\[.*?\]\2\]"           # [[ ... ]]
    r"|--[^\n]*"                    # -- ...
    r'|"(?:\\.|[^"\\\n])*"'         # "..."
    r"|'(?:\\.|[^'\\\n])*'",        # '...'
    re.S,
)
_NAME = r"[A-Za-z_]\w*"
_FUNC_DEFS = [
    re.compile(rf"\bfunction\s+(?:{_NAME}\s*[.:]\s*)*({_NAME})\s*\("),     # function f( / function T:m( / local function f(
    re.compile(rf"(?:\blocal\s+)?\b({_NAME})\s*=\s*function\b"),           # f = function( / local f = function(
]


def _code(chunk: str) -> str:
    """The chunk without comments; strings become empty strings (so `f""` calls still look like calls)."""
    return _STRIP.sub(lambda m: "" if m.group(0).startswith("--") else '""', chunk)


def _refers(code: str, names: set[str], methods: set[str]) -> bool:
    if names:
        # a bare name, not a field (`x.work`, `o:work`), not a table key or assignment (`blend=0.8`)
        alt = "|".join(sorted(map(re.escape, names)))
        if re.search(rf"(?<![\w.:])(?:{alt})(?!\w)(?!\s*=(?!=))", code):
            return True
    if methods:
        alt = "|".join(sorted(map(re.escape, methods)))
        if re.search(rf"[.:]\s*(?:{alt})\s*[({{\"]", code):
            return True
    return False


def split_chunks(source: str) -> list[tuple[int, str]]:
    marks = list(_CHUNK.finditer(source))
    return [(int(m.group(1)), source[m.end(): marks[i + 1].start() if i + 1 < len(marks) else len(source)])
            for i, m in enumerate(marks)]


def classify(source: str) -> list[tuple[int, bool]]:
    """(chunk number, paints) for each chunk of a log, in order."""
    names, methods = set(VERBS), set(METHODS)
    out = []
    for n, chunk in split_chunks(source):
        code = _code(chunk)
        paints = _refers(code, names, methods)
        if paints:
            for pat in _FUNC_DEFS:
                for m in pat.finditer(code):
                    names.add(m.group(1))
                    methods.add(m.group(1))
            # aliases: `dab = work`, `local s = b.stroke`
            alt = "|".join(sorted(map(re.escape, names | methods)))
            for m in re.finditer(rf"(?:\blocal\s+)?\b({_NAME})\s*=\s*(?:{_NAME}\s*[.:]\s*)?(?:{alt})\b(?!\s*[({{\"])", code):
                names.add(m.group(1))
        out.append((n, paints))
    return out


def count_painting_chunks(source: str) -> int:
    return sum(1 for _, paints in classify(source) if paints)
