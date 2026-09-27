"""Identify painting-log chunks that can put marks on the canvas."""

import re


VERBS = {
    "canvas", "work", "blend", "stipple", "lose", "erase", "fix",
    "varnish", "cracks", "relief",
}
METHODS = {"stroke", "touch", "paint", "sketch", "line", "rule", "hatch"}

_CHUNK = re.compile(r"^--@ chunk (\d+)", re.M)
_STRIP = re.compile(
    r"--\[(=*)\[.*?\]\1\]"
    r"|\[(=*)\[.*?\]\2\]"
    r"|--[^\n]*"
    r'|"(?:\\.|[^"\\\n])*"'
    r"|'(?:\\.|[^'\\\n])*'",
    re.S,
)
_NAME = r"[A-Za-z_]\w*"
_FUNC_DEFS = [
    re.compile(rf"\bfunction\s+(?:{_NAME}\s*[.:]\s*)*({_NAME})\s*\("),
    re.compile(rf"(?:\blocal\s+)?\b({_NAME})\s*=\s*function\b"),
]


def _code(chunk):
    return _STRIP.sub(lambda m: "" if m.group(0).startswith("--") else '""', chunk)


def _refers(code, names, methods):
    if names:
        alt = "|".join(sorted(map(re.escape, names)))
        if re.search(rf"(?<![\w.:])(?:{alt})(?!\w)(?!\s*=(?!=))", code):
            return True
    if methods:
        alt = "|".join(sorted(map(re.escape, methods)))
        if re.search(rf'[.:]\s*(?:{alt})\s*[({{"\']', code):
            return True
    return False


def split_chunks(source):
    marks = list(_CHUNK.finditer(source))
    return [
        (int(mark.group(1)), source[mark.end(): marks[i + 1].start() if i + 1 < len(marks) else len(source)])
        for i, mark in enumerate(marks)
    ]


def classify(source):
    """Return ``(chunk number, paints)`` for every chunk in a log."""
    names, methods = set(VERBS), set(METHODS)
    result = []
    for number, chunk in split_chunks(source):
        code = _code(chunk)
        paints = _refers(code, names, methods)
        if paints:
            for pattern in _FUNC_DEFS:
                for match in pattern.finditer(code):
                    names.add(match.group(1))
                    methods.add(match.group(1))
            alt = "|".join(sorted(map(re.escape, names | methods)))
            for match in re.finditer(
                rf'(?:\blocal\s+)?\b({_NAME})\s*=\s*(?:{_NAME}\s*[.:]\s*)?(?:{alt})\b(?!\s*[({{"\'])',
                code,
            ):
                names.add(match.group(1))
        result.append((number, paints))
    return result


def count_painting_chunks(source):
    return sum(1 for _, paints in classify(source) if paints)
