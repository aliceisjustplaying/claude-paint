"""The next studio's notes from structured chain records (p<n>_record.json, chain-record/1): the
runner writes every word the painter reads, through fixed templates, and checks each record
against the studio it goes to first (design: structured chain records, section 4).

render(records, recipient, compat) returns the notes text to append to studio_notes.md (empty if
nothing is inherited) and a report of what went in and what was left out and why, which the
runner keeps as run/<lane>/p<n>_inherited.json. Nothing is left out silently.

| record vs. recipient            | action                                                         |
| medium or support kind differs  | the whole record is left out                                   |
| code commit differs             | the whole record is left out, unless record_compat.json pairs them |
| box differs                     | mixing_piles, and any observation naming a tube not in the box, is left out |

What is not rendered: evidence, slot, round, reader and the condition label (the painter can't
open the logs, and nothing tells it there was another painter). The support line implies an
earlier canvas.
"""
import hashlib
import json
import textwrap

from record_schema import CATEGORIES, PAINTER_WORDS, PARAMS, odd_chars, word_hits

RECORD_SCHEMA = "chain-record/1"
CONDITION = "chain-inherited, non-neutral"
HEADER = "\n## More notes from the studio\n"
TITLES = {"ground": "The canvas and ground", "pencil": "Pencil", "mixing_piles": "Mixing piles",
          "strokes": "Strokes", "brushes": "Brushes", "blending": "Blending", "wet_into_wet": "Wet into wet",
          "glazing": "Glazing", "stippling": "Stippling", "drying_and_time": "Drying and time",
          "masks_and_edges": "Masks and edges", "easel_errors": "Easel errors"}
SURFACE = {"bare_ground": "on bare ground", "open": "into open paint", "setting": "into setting paint",
           "tacky": "on tacky paint", "touch_dry": "on touch-dry paint", "dry": "on dry paint",
           "mixed": "over paint in mixed states", "unknown": None}
TAGS = {"printed": "(printed by the easel)", "painter_reported": "(reported, not verified)"}
WIDTH = 96
# how a ground layer is put on: the easel's own words (canvas{ground={{apply=...}}}), the only ones
# a support line renders
GROUND_APPLY = ("knife", "roller", "brush")
LINEN_RANGE = (4, 60)                  # threads per cm the easel accepts


def num(v):
    """A number or a [lo, hi] range as the notes write it: 2.0 is 2, a range is lo–hi."""
    if isinstance(v, list):
        return "–".join(num(x) for x in v)
    if isinstance(v, bool):
        return "on" if v else "off"
    return str(int(v)) if float(v).is_integer() else f"{v:g}"


def words(items):
    """a, b and c (no serial comma)."""
    return items[0] if len(items) == 1 else ", ".join(items[:-1]) + " and " + items[-1]


def when(o):
    """The "When:" clause, built from the operation and its conditions."""
    c = o.get("conditions") or {}
    out = [o["operation"].replace("_", " ")]
    if c.get("tubes"):
        out.append(words(c["tubes"]))
    b = c.get("brush") or {}
    if b:
        brush = b.get("kind") or "brush"
        if b.get("width") is not None:
            brush += f" {num(b['width'])}" + (" wide" if isinstance(b["width"], list) else "")
        out.append(" ".join([brush] + [f"{k} {num(b[k])}" for k in ("point", "load", "stiffness")
                                       if b.get(k) is not None]))
    if c.get("clip") is not None:
        out.append("clipped" if c["clip"] else "unclipped")
    if SURFACE.get(c.get("surface")):
        out.append(SURFACE[c["surface"]])
    p = c.get("params") or {}
    for k in ("hand",) + tuple(k for k in PARAMS if k != "hand"):
        v = p.get(k)
        if v is None:
            continue
        if k == "hand":
            out.append(f"{v} hand")
        elif k == "fill":
            out.append("filled" if v else "no fill")
        elif k == "edge" and isinstance(v, str):
            out.append(f"{v} edge")
        elif k == "wait_minutes":
            out.append(f"{num(v)} minutes")
        elif k == "touches_per_dip":
            out.append(f"{num(v)} touches per dip")
        else:
            out.append(f"{k} {num(v)}")
    if c.get("extent_units") is not None:
        out.append(f"over a passage {num(c['extent_units'])} units wide")
    if c.get("note"):
        out.append(c["note"])
    return ", ".join(out)


def tag(o):
    if o["basis"] == "seen":
        n = len({(e["log"], e["call"]) for e in o.get("resolved") or [] if e["role"] == "operation"})
        return "(seen once)" if n <= 1 else f"(seen in {n} passes)"
    return TAGS[o["basis"]]


def line(o):
    """An observation's bullet as one line, before wrapping."""
    effect = o["effect"].rstrip()
    effect += "" if effect[-1:] in ".!?" else "."
    cause = f" {o['cause'].rstrip('.')}." if o.get("cause") else ""
    return f"- {effect}{cause} When: {when(o)}. {tag(o)}"


def bullet(o):
    return "\n".join(textwrap.wrap(line(o), WIDTH, subsequent_indent="  ", break_on_hyphens=False)) + "\n"


def _thread_count(v):
    return (isinstance(v, (int, float)) and not isinstance(v, bool) and v == v
            and LINEN_RANGE[0] <= v <= LINEN_RANGE[1])


def support_line(s):
    """The record's support in the runner's words only: linen, its thread counts if they are two
    numbers in the easel's range and the ground layers' ways of being laid that are the easel's
    (GROUND_APPLY); anything else in the record's support isn't rendered."""
    s = s if isinstance(s, dict) else {}
    line = "Observed on linen"
    linen = s.get("linen")
    if isinstance(linen, list) and len(linen) == 2 and all(_thread_count(v) for v in linen):
        line += f" {num(linen[0])} by {num(linen[1])} threads per cm"
    layers = [a for a in s.get("ground_layers") or [] if isinstance(a, str) and a in GROUND_APPLY]
    if layers:
        line += ", over a ground laid by " + ", then ".join(layers)
    return line + "."


def rendered_problems(o):
    """What the rendered bullet says, read as a whole, that its fields each didn't: a command or a
    place split across fields ("..., so" in one, "clip every blend" in the next), another painter,
    or a character plain text may not hold (a record changed after it was validated)."""
    raw = line(o)
    odd = odd_chars(raw)
    flat = " ".join(raw.split())
    drops = ([f"rendered: not plain text ({', '.join(f'U+{ord(c):04X}' for c in odd)})"] if odd else [])
    drops += [f"rendered: {name} ({words!r})" for name, words in word_hits(flat)[0]]
    words = sorted({w.lower() for w in PAINTER_WORDS.findall(flat)})
    return drops + ([f"rendered: another painter ({', '.join(words)})"] if words else [])


def block(record, observations):
    """One record's notes: its support line, then its observations by category (in CATEGORIES order)."""
    out = [support_line(record.get("support")) + "\n"]
    for cat in CATEGORIES:
        these = [o for o in observations if o["category"] == cat]
        if these:
            out.append(f"\n### {TITLES[cat]}\n" + "".join(bullet(o) for o in these))
    return "".join(out)


def same_code(a, b, compat):
    return a == b or [a, b] in compat or [b, a] in compat


def load_compat(path):
    """record_compat.json (operator-written, committed): [[commit, commit], ...] that count as one engine."""
    return json.loads(path.read_text()) if path.exists() else []


def render(records, recipient, compat=()):
    """records: [(slot, chain-record/1 or None if it is missing)], in slot order. recipient: {"medium", "support_kind",
    "commit", "box": {"name", "tubes_sha256"}, "tubes": [names]}. Returns (notes text, report)."""
    compat = [list(p) for p in compat]
    blocks, report = [], []
    for slot, r in records:
        if r is None:
            report.append({"slot": slot, "observations": 0, "inherited": 0, "dropped": [],
                           "excluded": f"no record (p{slot}_record.json is missing)"})
            continue
        obs = r["observations"]
        entry = {"slot": slot, "observations": len(obs), "inherited": 0, "excluded": None, "dropped": []}
        report.append(entry)
        if r.get("schema") != RECORD_SCHEMA:
            entry["excluded"] = f"schema {r.get('schema')!r}, not {RECORD_SCHEMA!r}"
        elif r.get("medium") != recipient["medium"]:
            entry["excluded"] = f"medium {r.get('medium')!r}, the studio's is {recipient['medium']!r}"
        elif (r.get("support") or {}).get("kind") != recipient["support_kind"]:
            entry["excluded"] = (f"support {(r.get('support') or {}).get('kind')!r}, "
                                 f"the studio's is {recipient['support_kind']!r}")
        elif not same_code(r["code"]["commit"], recipient["commit"], compat):
            entry["excluded"] = (f"code {r['code']['commit']}, the studio's is {recipient['commit']} "
                                 f"(not paired in record_compat.json)")
        if entry["excluded"]:
            continue
        kept = obs
        if r["box"] != recipient["box"]:
            tubes = set(recipient["tubes"])
            kept = []
            for o in obs:
                missing = [t for t in (o.get("conditions") or {}).get("tubes") or [] if t not in tubes]
                if o["category"] == "mixing_piles" or missing:
                    entry["dropped"].append({"index": o["index"], "why": "box " + (
                        f"{r['box']['name']} is not the studio's {recipient['box']['name']}: mixing piles"
                        if o["category"] == "mixing_piles" else f"has no {', '.join(missing)}")})
                else:
                    kept.append(o)
        whole = []
        for o in kept:
            why = rendered_problems(o)
            if why:
                entry["dropped"].append({"index": o["index"], "why": "; ".join(why)})
            else:
                whole.append(o)
        kept = whole
        entry["inherited"] = len(kept)
        if kept:
            blocks.append(block(r, kept))
    text = (HEADER + "\n" + "\n".join(blocks)) if blocks else ""
    return text, report


def summary(tag, report):
    """F2: inherited 14 of 17 observations (p1: 9/9, p2: 5/8, 3 box)."""
    got = sum(e["inherited"] for e in report)
    of = sum(e["observations"] for e in report)
    parts = [f"p{e['slot']}: " + (f"excluded, {e['excluded']}" if e["excluded"] else f"{e['inherited']}/{e['observations']}")
             for e in report]
    box = sum(1 for e in report for d in e["dropped"] if d["why"].startswith("box"))
    whole = sum(1 for e in report for d in e["dropped"] if d["why"].startswith("rendered"))
    return (f"{tag}: inherited {got} of {of} observations ("
            + ", ".join(parts + ([f"{box} box"] if box else []) + ([f"{whole} rendered"] if whole else [])) + ")")


def inherited(report, recipient, notes_bytes):
    """What p<n>_inherited.json keeps: every record's inclusion, exclusion and drops, and the hash
    of the notes the painter got."""
    return {"recipient": recipient, "records": report,
            "inherited": sum(e["inherited"] for e in report), "of": sum(e["observations"] for e in report),
            "notes_sha256": hashlib.sha256(notes_bytes).hexdigest()}
