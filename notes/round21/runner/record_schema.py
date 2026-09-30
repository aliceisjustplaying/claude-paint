"""Structured chain records: what a reader may write (p<n>_observations.json) and the checks it
passes before any of it reaches the next studio (design: structured chain records, section 2-3).

The reader writes only observations, each a few short fields; the runner validates them here,
stamps the metadata and renders the notes through fixed templates (render.py), so the reader
controls no headings, emphasis, quotes or layout.

validate(doc, ctx) returns {"observations", "dropped", "warnings"} or raises Rejected for a
record that can't be used at all (not an object, the wrong schema, no observations or more than
MAX_OBSERVATIONS). A single observation that fails a check is dropped and listed in "dropped"
with why; the record goes on if any remain. Stdlib only (the runner is a uv script).
"""
import copy
import math
import re
import unicodedata

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


def texts(o):
    """An observation's free text and structured strings, as (field, text)."""
    c = o.get("conditions") or {}
    out = [("effect", o.get("effect")), ("cause", o.get("cause")), ("note", c.get("note"))]
    out += [("tubes", t) for t in (c.get("tubes") or []) if isinstance(t, str)]
    return [(f, t) for f, t in out if isinstance(t, str)]


def validate(doc, tubes=None):
    """The observations of a reader's p<n>_observations.json (parsed): kept, dropped and warned.
    Raises Rejected if the record can't be used at all. tubes: the source box's tube names."""
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
    kept, dropped = [], []
    for i, o in enumerate(obs):
        why = shape_problems(o, tubes)
        if why:
            dropped.append({"index": i, "why": why})
        else:
            kept.append(dict(o, index=i))
    if not kept:
        raise Rejected(f"every observation was dropped ({len(dropped)})")
    return {"observations": kept, "dropped": dropped, "warnings": []}
