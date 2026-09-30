"""Structured chain records: what a reader may write (p<n>_observations.json) and the checks it
passes before any of it reaches the next studio (design: structured chain records, section 2-3).

The reader writes only observations, each a few short fields; the runner validates them here,
stamps the metadata and renders the notes through fixed templates (render.py), so the reader
controls no headings, emphasis, quotes or layout.

validate(doc, tubes, logs, names) returns {"observations", "dropped", "warnings"} or raises Rejected for a
record that can't be used at all (not an object, the wrong schema, no observations or more than
MAX_OBSERVATIONS). A single observation that fails a check is dropped and listed in "dropped"
with why; the record goes on if any remain. Stdlib only (the runner is a uv script).
"""
import copy
import json
import math
import re
import unicodedata
import subprocess
import tempfile
from pathlib import Path

SCHEMA = "chain-observations/1"
MAX_OBSERVATIONS = 40

# render order is this order
CATEGORIES = ("ground", "pencil", "mixing_piles", "strokes", "brushes", "blending", "wet_into_wet", "glazing",
              "stippling", "drying_and_time", "masks_and_edges", "easel_errors")
# easel verbs, a curated list: not every API name (the legacy tree keys name subjects)
OPERATIONS = ("canvas", "pile", "stroke", "touch", "work", "blend", "stipple", "glaze", "scumble", "hatch",
              "cut_in", "wait", "drying", "mask", "pencil", "erase")
SURFACES = ("bare_ground", "open", "setting", "tacky", "touch_dry", "dry", "mixed", "unknown")
BRUSH_KINDS = ("round", "flat", "filbert", "fan", "rigger", "badger", "stippler", "knife")
BASES = ("printed", "seen", "painter_reported")
ROLES = ("operation", "source", "image", "report")
HANDS = ("broad", "body", "detail", "blend", "hatch", "glaze", "scumble")
EDGES = ("found", "firm", "soft", "loose", "lost")          # easel_guide.md's names for edge=

# (lo, hi, lo is open): the easel's ranges (notes/easel_guide.md: pressure 0..1, point 0..1, a load
# 0..1 of a full one, medium 0..0.95, wait up to 10 years); None is unbounded
BRUSH_RANGES = {"width": (0, 1000, True), "point": (0, 1, False), "load": (0, 1, False),
                "stiffness": (0, 1, False)}
PARAM_RANGES = {"coverage": (0, 50, True), "medium": (0, 0.95, False), "pressure": (0, 1, False),
                "feather": (0, 1, False), "threshold": (0, 1, False), "length": (0, 5000, True),
                "wait_minutes": (0, 10 * 366 * 24 * 60, False), "touches_per_dip": (1, 100000, False),
                "splay": (0, None, False), "edge": (0, 1, False)}
PARAMS = ("coverage", "medium", "pressure", "feather", "fill", "threshold", "length", "hand", "edge",
          "wait_minutes", "touches_per_dip", "splay")
EXTENT_RANGE = (0, 5000, True)

OBS_KEYS = {"category", "operation", "conditions", "effect", "cause", "basis", "evidence"}
REQUIRED = ("category", "operation", "effect", "basis", "evidence")
COND_KEYS = ("surface", "clip", "brush", "params", "tubes", "extent_units", "note")
EVIDENCE_KEYS = {"log", "call", "role"}
LIMITS = {"effect": (20, 200), "cause": (1, 140), "note": (1, 80)}

# markdown-inert: none of these, no URL and at most one sentence
NOT_INERT = re.compile(r"[\n\r\t`#*_>|\[\]{}=<]")
URL = re.compile(r"https?://|www\.|\w+\.(?:com|org|net|io|md|lua|png|json)\b", re.I)
TERMINATOR = re.compile(r"[.!?](?=\s|$)")
# Free text is checked, kept and rendered in its NFKC form (a fullwidth ＃ is #, … is ...), and
# then holds only printable ASCII, Latin letters and these: no control, invisible, bidi or
# line-separator characters (U+200B, U+202E, U+2028, U+0085 hide a word from the checks or break
# the notes' lines) and no letter of another script (a Cyrillic е makes "Usе" another word).
TEXT_EXTRA = frozenset("–—‘’“”·°×±≈−")


class Rejected(Exception):
    """The whole record can't be used (the chain stops, as a rejected free-text record does)."""


def plain(text):
    """Free text as it is checked, kept and rendered (NFKC)."""
    return unicodedata.normalize("NFKC", text)


def odd_chars(text):
    """The characters of text that plain text may not hold (see TEXT_EXTRA), in order."""
    return sorted({c for c in text if not (" " <= c <= "~" or c in TEXT_EXTRA
                                            or (c.isalpha() and unicodedata.name(c, "").startswith("LATIN ")))})


def inert_problems(field, text):
    """Why free text isn't a short, markdown-inert sentence of plain text (empty if it is)."""
    if not isinstance(text, str):
        return [f"{field}: not text"]
    text = plain(text)
    out = []
    odd = odd_chars(text)
    if odd:
        out.append(f"{field}: not plain text ("
                   + ", ".join(f"U+{ord(c):04X} {unicodedata.name(c, 'unnamed')}" for c in odd) + ")")
    lo, hi = LIMITS[field]
    if not lo <= len(text) <= hi:
        out.append(f"{field}: {len(text)} characters (want {lo}-{hi})")
    bad = sorted(set(NOT_INERT.findall(text)))
    if bad:
        out.append(f"{field}: not markdown-inert ({' '.join(repr(c) for c in bad)})")
    if URL.search(text):
        out.append(f"{field}: a URL or file name")
    if len(TERMINATOR.findall(text)) > 1:
        out.append(f"{field}: more than one sentence")
    return out


def _number(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)


def _in_range(v, rng):
    lo, hi, open_lo = rng
    return (v > lo if open_lo else v >= lo) and (hi is None or v <= hi)


def value_problems(field, v, rng, enum=None, flag=False):
    """A number or a [lo, hi] range within rng; or, if allowed, a bool (flag) or a token of enum."""
    if flag and isinstance(v, bool):
        return []
    if enum is not None and isinstance(v, str):
        return [] if v in enum else [f"{field}: {v!r} is not one of {', '.join(enum)}"]
    if rng is None:
        return [f"{field}: want {'true or false' if flag else 'one of ' + ', '.join(enum or ())}"]
    vals = v if isinstance(v, list) else [v]
    if isinstance(v, list) and not (len(v) == 2 and all(_number(x) for x in v) and v[0] <= v[1]):
        return [f"{field}: a range is [lo, hi] with lo <= hi"]
    if not all(_number(x) for x in vals):
        return [f"{field}: want a number or a [lo, hi] range"]
    if not all(_in_range(x, rng) for x in vals):
        lo, hi, _ = rng
        return [f"{field}: {v} is outside the easel's range ({lo} to {'any' if hi is None else hi})"]
    return []


def condition_problems(c, tubes=None):
    """What's wrong with an observation's conditions. tubes: the source box's tube names, if known."""
    if not isinstance(c, dict):
        return ["conditions: not an object"]
    out = [f"conditions: unknown key {k!r}" for k in sorted(set(c) - set(COND_KEYS))]
    if c.get("surface") is not None and c["surface"] not in SURFACES:
        out.append(f"conditions.surface: {c['surface']!r} is not one of {', '.join(SURFACES)}")
    if c.get("clip") is not None and not isinstance(c["clip"], bool):
        out.append("conditions.clip: want true, false or null")
    b = c.get("brush")
    if b is not None:
        if not isinstance(b, dict):
            out.append("conditions.brush: not an object")
        else:
            out += [f"conditions.brush: unknown key {k!r}" for k in sorted(set(b) - {"kind", *BRUSH_RANGES})]
            if b.get("kind") is not None and b["kind"] not in BRUSH_KINDS:
                out.append(f"conditions.brush.kind: {b['kind']!r} is not one of {', '.join(BRUSH_KINDS)}")
            for k, rng in BRUSH_RANGES.items():
                if b.get(k) is not None:
                    out += value_problems(f"conditions.brush.{k}", b[k], rng)
    p = c.get("params")
    if p is not None:
        if not isinstance(p, dict):
            out.append("conditions.params: not an object")
        else:
            out += [f"conditions.params: unknown key {k!r}" for k in sorted(set(p) - set(PARAMS))]
            for k, v in p.items():
                if k not in PARAMS or v is None:
                    continue
                if k == "hand":
                    out += value_problems("conditions.params.hand", v, None, HANDS)
                elif k == "fill":
                    out += value_problems("conditions.params.fill", v, None, flag=True)
                elif k == "edge":
                    out += value_problems("conditions.params.edge", v, PARAM_RANGES["edge"], EDGES)
                else:
                    out += value_problems(f"conditions.params.{k}", v, PARAM_RANGES[k], flag=True)
    t = c.get("tubes")
    if t is not None:
        if not (isinstance(t, list) and all(isinstance(x, str) and x for x in t)):
            out.append("conditions.tubes: want a list of tube names (no parts or ratios)")
        elif tubes is not None:
            out += [f"conditions.tubes: {x!r} is not in the box" for x in t if x not in tubes]
    if c.get("extent_units") is not None:
        out += value_problems("conditions.extent_units", c["extent_units"], EXTENT_RANGE)
    if c.get("note") is not None:
        out += inert_problems("note", c["note"])
    return out


def evidence_shape_problems(ev):
    if not (isinstance(ev, list) and 1 <= len(ev) <= 6):
        return ["evidence: want 1 to 6 items"]
    out = []
    for i, e in enumerate(ev):
        if not isinstance(e, dict) or set(e) != EVIDENCE_KEYS:
            out.append(f"evidence {i}: want exactly log, call and role")
            continue
        if not (isinstance(e["log"], int) and not isinstance(e["log"], bool) and e["log"] >= 1):
            out.append(f"evidence {i}: log is a 1-based number")
        if not (isinstance(e["call"], str) and 0 < len(e["call"]) <= 200):
            out.append(f"evidence {i}: call is a tool-call id")
        if e["role"] not in ROLES:
            out.append(f"evidence {i}: role {e['role']!r} is not one of {', '.join(ROLES)}")
    return out


def shape_problems(o, tubes=None):
    """Types, enums, limits and markdown-inert text of one observation (empty if it passes)."""
    if not isinstance(o, dict):
        return ["not an object"]
    out = [f"unknown key {k!r}" for k in sorted(set(o) - OBS_KEYS)]
    out += [f"missing {k}" for k in REQUIRED if k not in o]
    if "category" in o and o["category"] not in CATEGORIES:
        out.append(f"category: {o['category']!r} is not one of {', '.join(CATEGORIES)}")
    if "operation" in o and o["operation"] not in OPERATIONS:
        out.append(f"operation: {o['operation']!r} is not one of {', '.join(OPERATIONS)}")
    if "basis" in o and o["basis"] not in BASES:
        out.append(f"basis: {o['basis']!r} is not one of {', '.join(BASES)}")
    if "effect" in o:
        out += inert_problems("effect", o["effect"])
    if o.get("cause") is not None:
        out += inert_problems("cause", o["cause"])
    if o.get("conditions") is not None:
        out += condition_problems(o["conditions"], tubes)
    if "evidence" in o:
        out += evidence_shape_problems(o["evidence"])
    return out


def normalized(o):
    """o with its free text and tube names in NFKC (plain), so what is checked is what is kept."""
    if not isinstance(o, dict):
        return o
    o = copy.deepcopy(o)
    for k in ("effect", "cause"):
        if isinstance(o.get(k), str):
            o[k] = plain(o[k])
    c = o.get("conditions")
    if isinstance(c, dict):
        if isinstance(c.get("note"), str):
            c["note"] = plain(c["note"])
        if isinstance(c.get("tubes"), list):
            c["tubes"] = [plain(t) if isinstance(t, str) else t for t in c["tubes"]]
    return o


# a chunk's line in a tool result: it ran (ok · chunk 129 ...) or failed (the chunk failed ...)
CHUNK_RAN = re.compile(r"\bok · chunk (\d+)\b")
CHUNK_FAILED = re.compile(r"the chunk failed")
# result lines that say nothing the easel printed about the paint: the chunk line, a look's file, a note
PLAIN_LINE = re.compile(r"^\s*$|\bchunk \d+\b|\.png\b|^noted in |^Successfully wrote ")
PNG = re.compile(r"\.png\b", re.I)
# a bash command that runs a chunk at the easel (easel do, as round 16 and 17 painters did)
EASEL_DO = re.compile(r"\beasel\s+do\b")


class LogIndex:
    """The tool calls of one pi session log (JSON lines): id -> where it is, what it ran and what
    came back. Ids are opaque (toolu_..., call_...). A result's isError is kept but not trusted:
    a chunk that failed can come back with isError false, so failure is read from the text."""

    ARGS_MAX = 64_000
    TEXT_MAX = 16_000

    def __init__(self, path):
        self.path = str(path)
        self.calls = {}
        self.duplicates = set()                   # ids more than one call used: evidence can't cite them
        with open(path, errors="replace") as f:
            for i, line in enumerate(f, 1):
                if '"toolCall"' not in line and '"toolResult"' not in line:
                    continue
                try:
                    m = json.loads(line).get("message") or {}
                except ValueError:
                    continue
                if not isinstance(m, dict):
                    continue
                if m.get("role") == "assistant":
                    for p in m.get("content") or []:
                        if isinstance(p, dict) and p.get("type") == "toolCall" and isinstance(p.get("id"), str):
                            if p["id"] in self.calls:
                                self.duplicates.add(p["id"])
                            self.calls[p["id"]] = {"line": i, "tool": p.get("name"),
                                                   "args": json.dumps(p.get("arguments"))[:self.ARGS_MAX],
                                                   "result_line": None, "text": "", "image": False, "is_error": None}
                elif m.get("role") == "toolResult" and m.get("toolCallId") in self.calls:
                    c = self.calls[m["toolCallId"]]
                    if c["result_line"] is not None:
                        continue
                    parts = [p for p in m.get("content") or [] if isinstance(p, dict)]
                    c.update(result_line=i, is_error=m.get("isError"),
                             text="\n".join(p.get("text") or "" for p in parts if p.get("type") == "text")[:self.TEXT_MAX],
                             image=any(p.get("type") == "image" for p in parts))


def _call_facts(c):
    ran = CHUNK_RAN.search(c["text"])
    return {"line": c["line"], "tool": c["tool"], "result_line": c["result_line"],
            "chunk": int(ran.group(1)) if ran else None, "failed": bool(CHUNK_FAILED.search(c["text"]))}


def _runs_chunk(c):
    """A call that ran a chunk at the easel: the paint tool, or bash running `easel do` (a read,
    a grep or a cat of an earlier reply shows the same chunk line and isn't one)."""
    return c["tool"] == "paint" or (c["tool"] == "bash" and bool(EASEL_DO.search(c["args"])))


def _shows_image(c):
    """A call that put an image in front of the painter: a look, a read of a .png, or a result
    that carries an image (a result that only names a .png, like an ls or a bash `easel look`,
    showed a file name, not a picture)."""
    return c["tool"] == "look" or c["image"] or (c["tool"] == "read" and bool(PNG.search(c["args"])))


def _code(c):
    """A call's arguments as the painter wrote them (JSON's escaped line breaks read as breaks)."""
    return c["args"].replace("\\n", "\n").replace("\\t", "\t")


def _printed(c):
    """Whether a result holds easel output beyond the chunk line (a value, a state, an error)."""
    return any(not PLAIN_LINE.search(l) for l in c["text"].splitlines())


def evidence_problems(o, logs):
    """Whether o's evidence resolves in logs (LogIndex, in the brief's order) and backs its basis
    and category. Returns (problems, resolved)."""
    out, resolved, found = [], [], []
    for i, e in enumerate(o["evidence"]):
        if not 1 <= e["log"] <= len(logs):
            out.append(f"evidence {i}: log {e['log']} is not one of the {len(logs)} logs")
            continue
        c = logs[e["log"] - 1].calls.get(e["call"])
        if e["call"] in logs[e["log"] - 1].duplicates:
            out.append(f"evidence {i}: log {e['log']} has more than one tool call {e['call']} (ambiguous)")
            continue
        if c is None:
            out.append(f"evidence {i}: log {e['log']} has no tool call {e['call']}")
            continue
        if c["result_line"] is None:
            out.append(f"evidence {i}: log {e['log']} call {e['call']} has no result")
            continue
        found.append((e, c))
        resolved.append(dict(log=e["log"], call=e["call"], role=e["role"], **_call_facts(c)))
    if out:
        return out, resolved
    ops = [(e, c) for e, c in found if e["role"] == "operation"]
    if not ops:
        return ["no operation evidence"], resolved
    ran = [(e, c) for e, c in ops if _runs_chunk(c) and (CHUNK_RAN.search(c["text"]) or CHUNK_FAILED.search(c["text"]))]
    if not ran:
        return ["no operation evidence shows a chunk that ran or failed (a paint call or bash running easel do)"], resolved
    first = min((e["log"], c["line"]) for e, c in ran)
    if o["basis"] == "seen" and not any(e["role"] == "image" and _shows_image(c) and (e["log"], c["line"]) >= first
                                        for e, c in found):
        out.append("basis seen: no image evidence looked at after the operation")
    if o["basis"] == "printed" and not any(_printed(c) for _, c in ran):
        out.append("basis printed: the operation's result holds nothing beyond the chunk line")
    if o["category"] == "easel_errors" and not any(CHUNK_FAILED.search(c["text"]) for _, c in ran):
        out.append("easel_errors: no operation evidence shows a chunk that failed")
    return out, resolved


def evidence_warnings(o, i, logs):
    """A warning when the chunk code the evidence holds (a paint call's, or a cited source's)
    doesn't name the observation's operation; nothing when no code was cited (easel do -f)."""
    code = [_code(c) for e in o["evidence"] if 1 <= e["log"] <= len(logs)
            for c in [logs[e["log"] - 1].calls.get(e["call"])]
            if c and (e["role"] == "source" or (e["role"] == "operation" and c["tool"] == "paint"))]
    if code and not any(re.search(r"\b" + re.escape(o["operation"]) + r"\b", t) for t in code):
        return [{"index": i, "field": "operation", "pattern": "OPERATION_NOT_IN_CODE", "words": o["operation"]}]
    return []


# What reader_brief.md excludes, in the short declarative fields of a structured record. Two tiers:
# a clear hit drops the observation (what to do: a command or a directive to the reader; where
# things sit in the picture: a part of the picture, the foreground, a position), an ambiguous one
# only warns (the same words in plain material facts: "the edge of the canvas", "a background
# wash", "the pile's composition"). Measured on every real record and studio_notes.md
# (test_record_words.py). Patterns only: a paraphrase, another language or a hint gets past them.
# a sentence, a "Label: " clause, a parenthesis or "To darken it, ..."
_LEAD = r"(?:^|[.;!?]\s+|:\s+|\(\s*|^to [\w ]{1,40},\s+)"
# commands that start a sentence; not the easel's names (clip, check, work, blend...) or words that
# start plain facts as adjectives or nouns (thin, set, run, mix, load, paint)
_COMMANDS = (r"always|never|don'?t|do not|avoid|try|remember|use|put|place|keep|make|let|leave|save|lay(?!-)|pass"
             r"|add|give|subtract|repaint|restate|shorten|lengthen|judge|prefer|reserve|be (?:careful|sure)"
             r"|consider|choose|pick|stop|start|begin|dilute|wipe|reload|aim")
# after ", so" or ", then" a command; the easel's verbs and the like only with an object: ", so run
# it with a long limit"
_THEN_COMMANDS = _COMMANDS + r"|run|mix|load|set|paint|glaze|blend|wait|work|thin|clip|check"
_OBJECT = r"(?:the|a|an|it|them|each|every|all|both|more|less|your|longer|shorter)"
# an easel verb that starts a sentence with an object is a command too ("Clip every blend ...",
# "Mix the darks ..."): material facts start with the verb as an adjective or noun ("Thin paint
# lets ...", "Clip on a body pass ..."), not with the verb and the/every/it (review C, finding 2)
_EASEL_VERBS = _THEN_COMMANDS + r"|stipple|scumble|hatch|mask|erase|cut|restate|lift|crop"
PRESCRIPTION_DROP = re.compile(
    _LEAD + r"(?:" + _COMMANDS + r")\b(?!-|\s+of\b)"
    r"|" + _LEAD + r"(?:" + _EASEL_VERBS + r")\s+" + _OBJECT + r"\b"
    r"|,\s*(?:so|then|and then)\s+(?:(?:" + _COMMANDS + r")\b(?!-)|(?:" + _THEN_COMMANDS + r")\s+" + _OBJECT + r"\b)"
    r"|\b(?:you|one) (?:should|must|need to|have to|can|could|will want|'ll want|may want)\b"
    r"|\b(?:should|must) (?:always|never|be)\b|\b(?:be sure|make sure|it is best|it's best|best to|better to)\b"
    r"|\bthe (?:trick|fix|cure|answer|remedy|way) is\b", re.I)
PRESCRIPTION_WARN = re.compile(r"\b(?:always|never|instead|works? (?:well|best)|worked well|is best|reads? (?:best|well)"
                               r"|suits?|should)\b",
                               re.I)
# the picture, not the canvas: "one side of the canvas" is where the noise fell, a material fact
_PICTURE = r"(?:picture|painting|composition|image|frame|scene|view)"
PLACEMENT_DROP = re.compile(
    r"\b(?:center|centre|middle|top|bottom|upper|lower|left|right|third|quarter|half|side)s? of the " + _PICTURE
    + r"\b|\bon the (?:left|right)(?: side)? of the " + _PICTURE + r"\b"
    r"|\b(?:in|into|across|over|through) the (?:foreground|middle ?ground|distance)\b|\bfocal point\b", re.I)
PLACEMENT_WARN = re.compile(
    r"\b(?:edge|corner|border)s? of the " + _PICTURE + r"\b|\bon the (?:left|right)\b|\b\w+ of the canvas\b"
    r"|\b(?:at|in|near|toward|towards) the (?:center|centre|middle)\b(?! of)"
    r"|\b(?:horizon|foreground|background|middle ?ground|composition|focal|motif)\b"
    # a position (y 330, x~500, y at 340; not a size, 20 x 30): also how far a tool's paint reached
    # ("a curve that stopped at x 620 painted out to x 700")
    r"|(?<!\d )(?<!\d)\b[xy]\s?[~≈]\s?\d|(?<!\d )(?<!\d)\b[xy] (?:(?:at|of|near|about|around) )?\d{2,}\b", re.I)


# as round 19's free-text gate (r19_chains.py): another painter, and a line of code (only a warning here:
# markdown-inert text can't hold code, so a hit is a word like "function" at the start)
PAINTER_WORDS = re.compile(r"\b(?:painters?|claude|opus|sonnet|gpt|gemini|kimi|mimo|codex|anthropic|openai)\b", re.I)
CODE_LINE = re.compile(r"^\s*(?:[-*]\s+)?(?:local\s|function\b|(?:for|while)\s.*\bdo\b|if\s.*\bthen\b|end\s*$"
                       r"|[A-Za-z_][\w.]*(?:\[[^\]]*\])?\s*=[^=])")


def studio_names(script, own, tmp):
    """A names check for validate: strings -> the indices of those holding a painter's name, by
    scripts/check_studio_names (less the studio's own artist), run on a scratch studio in tmp."""
    def check(strings):
        with tempfile.TemporaryDirectory(dir=tmp) as t:
            (Path(t) / "notes").mkdir()
            (Path(t) / "notes" / "fields.md").write_text("".join(x.replace("\n", " ") + "\n" for x in strings))
            r = subprocess.run([str(script), t, *own], capture_output=True, text=True)
        if r.returncode == 0:
            return set()
        hits = {int(m.group(1)) - 1 for m in re.finditer(r"^notes/fields\.md:(\d+):", r.stderr, re.M)}
        if not hits:
            raise RuntimeError(f"check_studio_names failed: {r.stderr.strip()[:300]}")
        return hits
    return check


def word_hits(text):
    """(drops, warnings) for one field's text: [(pattern name, matched words)]."""
    def hits(pairs):
        return [(name, m.group(0).strip(" .;:!?,")) for name, pat in pairs for m in [pat.search(text)] if m]
    drops = hits((("PRESCRIPTION", PRESCRIPTION_DROP), ("PLACEMENT", PLACEMENT_DROP)))
    dropped = {n for n, _ in drops}
    warns = [(n, w) for n, w in hits((("PRESCRIPTION", PRESCRIPTION_WARN), ("PLACEMENT", PLACEMENT_WARN)))
             if n not in dropped]
    return drops, warns


def texts(o):
    """An observation's free text and structured strings, as (field, text)."""
    c = o.get("conditions") or {}
    out = [("effect", o.get("effect")), ("cause", o.get("cause")), ("note", c.get("note"))]
    out += [("tubes", t) for t in (c.get("tubes") or []) if isinstance(t, str)]
    return [(f, t) for f, t in out if isinstance(t, str)]


def word_problems(o, i, names_hit):
    """Hard drops (another painter, a painter's name, a clear prescription or placement) and
    warnings for observation o (index i). names_hit: the fields (field, text) holding a name."""
    drops, warns = [], []
    for field, text in texts(o):
        words = sorted({w.lower() for w in PAINTER_WORDS.findall(text)})
        if words:
            drops.append(f"{field}: another painter ({', '.join(words)})")
        if (field, text) in names_hit:
            drops.append(f"{field}: a painter's name")
        if field == "tubes":
            continue
        d, w = word_hits(text)
        drops += [f"{field}: {name} ({words!r})" for name, words in d]
        warns += [{"index": i, "field": field, "pattern": name, "words": words} for name, words in w]
        if CODE_LINE.search(text):
            warns.append({"index": i, "field": field, "pattern": "CODE_LINE", "words": text[:40]})
    return drops, warns


def validate(doc, tubes=None, logs=None, names=None):
    """The observations of a reader's p<n>_observations.json (parsed): kept, dropped and warned.
    Raises Rejected if the record can't be used at all. tubes: the source box's tube names; logs:
    the session logs (LogIndex) in the brief's order, against which evidence is resolved (None
    checks the shape only); names: a names check (studio_names) over every string field. A kept
    observation carries "resolved": its evidence's calls."""
    if not isinstance(doc, dict):
        raise Rejected("the record is not a JSON object")
    if doc.get("schema") != SCHEMA:
        raise Rejected(f"schema is {doc.get('schema')!r}, not {SCHEMA!r}")
    if set(doc) != {"schema", "observations"}:
        raise Rejected(f"unknown keys {sorted(set(doc) - {'schema', 'observations'})}")
    obs = doc["observations"]
    if not isinstance(obs, list) or not obs:
        raise Rejected("no observations")
    if len(obs) > MAX_OBSERVATIONS:
        raise Rejected(f"{len(obs)} observations (at most {MAX_OBSERVATIONS})")
    obs = [normalized(o) for o in obs]
    kept, dropped, warnings = [], [], []
    shaped = [shape_problems(o, tubes) for o in obs]
    fields = [(i, f, t) for i, o in enumerate(obs) if not shaped[i] for f, t in texts(o)]
    hit = names([t for _, _, t in fields]) if names and fields else set()
    named = {(i, f, t) for k, (i, f, t) in enumerate(fields) if k in hit}
    for i, o in enumerate(obs):
        why = shaped[i]
        resolved = None
        if not why:
            why, warns = word_problems(o, i, {(f, t) for j, f, t in named if j == i})
        if not why and logs is not None:
            why, resolved = evidence_problems(o, logs)
            warns += [] if why else evidence_warnings(o, i, logs)
        if not why:
            warnings += warns
        if why:
            dropped.append({"index": i, "why": why})
        else:
            kept.append(dict(o, index=i, **({"resolved": resolved} if resolved is not None else {})))
    if not kept:
        raise Rejected(f"every observation was dropped ({len(dropped)})")
    return {"observations": kept, "dropped": dropped, "warnings": warnings}
