#!/usr/bin/env python3
"""Compare state dumps (`easel run --dump-state`, format CPSTATE1) field by field.

    state_compare.py BASELINE NEW [--added-zero] [--ignore NAME]... [--quiet] [--linux-tolerance]

BASELINE and NEW are two .state files (raw, .xz or .gz) or two directories
of them (every chunk-*.state[.xz|.gz] in BASELINE is compared with the file
of the same chunk in NEW, compressed or not).

Every field the baseline has must be in the new dump with the same dtype
and shape. Finite floats compare within absolute limits on every platform;
signed zeros are equal and nonfinite values fail. The text fields (brushes.debug,
rags.debug, studio.debug: Rust Debug text) are parsed into named values,
e.g. brushes[0].bristles[3].vol, and compared value by value the same way.
Fields or named values only the new dump has (a later engine's solvent, say)
are listed, not failed; with --added-zero each of them must be zero
(numbers 0, booleans false, None, empty lists).

Measured absolute limits for the six 128 px engine-3 baseline scenes apply
by default. Other floats allow an absolute error of 1e-7. Relative tolerance
is zero, so errors cannot grow with magnitude. Integers, structure and
added-zero checks stay exact. --linux-tolerance is a compatibility alias
for the default policy. Each float field reports its limit and observed error.

Exit 0: equal on every baseline field; 1: a difference; 2: a usage or
format error. Only Python's standard library.
"""

import argparse
import gzip
import json
import lzma
import math
import os
import re
import struct
import sys

DT = {"f32": ("f", 4), "f64": ("d", 8), "u32": ("I", 4), "u64": ("Q", 8)}

DEFAULT_ABS_TOLERANCE = 1e-7
MAX_REL_TOLERANCE = 0.0

ABS_LIMITS = {
    "color": 5e-6,
    "height": 1.3e-4,
    "film": 6e-6,
    "wet.vol": 8e-6,
    "wet.lat": 1.6e-7,
    "wet.hide": 5e-7,
    "wet.floor": 8e-6,
    "clock.cure": 4e-7,
    "clock.lev": 7e-5,
    "clock.seen": 8e-6,
    "clock.th": 6e-6,
    "clock.sub": 1.6e-5,
    "clock.srate": 2.2e-9,
    "brushes.debug:brushes[].bristles[].vol": 1.1e-3,
    "brushes.debug:brushes[].bristles[].rx": 6.1e-8,
    "brushes.debug:brushes[].bristles[].ry": 2.1e-8,
    "brushes.debug:brushes[].bristles[].thresh": 3.1e-8,
    "brushes.debug:brushes[].bristles[].bend[]": 1.1e-7,
    "brushes.debug:brushes[].bristles[].lat[]": 1.1e-7,
    "brushes.debug:brushes[].bristles[].hide[]": 3.1e-7,
    "brushes.debug:brushes[].bristles[].cure": 6.1e-12,
}

LINUX_LIMITS = ABS_LIMITS


class FormatError(Exception):
    pass


def read_bytes(path):
    if path.endswith(".xz"):
        return lzma.open(path).read()
    if path.endswith(".gz"):
        return gzip.open(path).read()
    with open(path, "rb") as f:
        return f.read()


def load(path):
    b = read_bytes(path)
    if b[:8] != b"CPSTATE1":
        raise FormatError("%s: not a CPSTATE1 state dump" % path)
    (n,) = struct.unpack_from("<Q", b, 8)
    head = json.loads(b[16:16 + n].decode())
    data = b[16 + n:]
    fields = {}
    for f in head["fields"]:
        raw = data[f["offset"]:f["offset"] + f["nbytes"]]
        if len(raw) != f["nbytes"]:
            raise FormatError("%s: field %s is truncated" % (path, f["name"]))
        fields[f["name"]] = (f["dtype"], tuple(f["shape"]), raw)
    return head, fields


# ---------------------------------------------------------------- Rust Debug text

TOKEN = re.compile(r"""
    \s*(?:
      (?P<str>"(?:[^"\\]|\\.)*")
    | (?P<num>-?(?:inf|NaN|\d+(?:\.\d*)?(?:e[-+]?\d+)?))(?![A-Za-z_])
    | (?P<id>[A-Za-z_][A-Za-z0-9_]*)
    | (?P<dots>\.\.)
    | (?P<p>[{}\[\](),:=])
    )""", re.X)


def tokens(text):
    pos, out = 0, []
    while pos < len(text):
        if text[pos:].strip() == "":
            break
        m = TOKEN.match(text, pos)
        if not m or m.end() == pos:
            raise FormatError("can't read Debug text at %r" % text[pos:pos + 40])
        kind = m.lastgroup
        out.append((kind, m.group(kind)))
        pos = m.end()
    return out


class Parser:
    def __init__(self, toks):
        self.t, self.i = toks, 0

    def peek(self, k=0):
        return self.t[self.i + k] if self.i + k < len(self.t) else (None, None)

    def take(self, value=None):
        tok = self.peek()
        if value is not None and tok[1] != value:
            raise FormatError("Debug text: expected %r, found %r" % (value, tok[1]))
        self.i += 1
        return tok

    def seq(self, close):
        items = []
        while self.peek()[1] != close:
            items.append(self.value())
            if self.peek()[1] == ",":
                self.take(",")
        self.take(close)
        return items

    def fields(self):
        out = {}
        while self.peek()[1] != "}":
            if self.peek()[0] == "dots":
                self.take()
                continue
            name = self.take()[1]
            self.take(":")
            out[name] = self.value()
            if self.peek()[1] == ",":
                self.take(",")
        self.take("}")
        return out

    def value(self):
        kind, v = self.take()
        if kind == "str":
            return ("str", v)
        if kind == "num":
            return ("num", v)
        if v == "[":
            return ("list", self.seq("]"))
        if v == "(":
            return ("list", self.seq(")"))
        if v == "{":
            return ("struct", self.fields())
        if kind == "id":
            nxt = self.peek()[1]
            if nxt == "{":
                self.take("{")
                return ("struct", self.fields())
            if nxt == "(":
                self.take("(")
                return ("variant", v, self.seq(")"))
            return ("id", v)
        raise FormatError("Debug text: unexpected %r" % v)


def flatten(node, path, out):
    kind = node[0]
    if kind == "struct":
        for k, v in node[1].items():
            flatten(v, "%s.%s" % (path, k) if path else k, out)
    elif kind == "list":
        out[path + ".len"] = ("num", str(len(node[1])))
        for i, v in enumerate(node[1]):
            flatten(v, "%s[%d]" % (path, i), out)
    elif kind == "variant":
        out[path] = ("id", node[1])
        for i, v in enumerate(node[2]):
            flatten(v, "%s.%s%d" % (path, node[1], i), out)
    else:
        out[path] = node


def debug_values(name, text):
    """Named values from a text field."""
    out = {}
    if name == "studio.debug":
        # key=value pairs, each value one Debug value
        p = Parser(tokens(text))
        while p.peek()[0] is not None:
            key = p.take()[1]
            p.take("=")
            flatten(p.value(), key, out)
        return out
    base = name.split(".")[0]
    lines = [l for l in text.split("\n") if l.strip()]
    out[base + ".len"] = ("num", str(len(lines)))
    for i, line in enumerate(lines):
        p = Parser(tokens(line))
        flatten(p.value(), "%s[%d]" % (base, i), out)
        if p.peek()[0] is not None:
            raise FormatError("%s: text after a value on line %d" % (name, i + 1))
    return out


def num(v):
    return float(v.replace("NaN", "nan"))


def zero_value(node):
    kind = node[0]
    if kind == "num":
        return num(node[1]) == 0.0
    if kind == "id":
        return node[1] in ("false", "None")
    if kind == "str":
        return node[1] == '""'
    return False


# ---------------------------------------------------------------- compare

def float_within_limit(name, x, y, limits, observed):
    limit = limits.get(name, ABS_LIMITS.get(name, DEFAULT_ABS_TOLERANCE))
    d = abs(x - y) if math.isfinite(x) and math.isfinite(y) else math.inf
    observed[name] = max(observed.get(name, 0.0), d)
    return math.isfinite(x) and math.isfinite(y) and math.isclose(x, y, abs_tol=limit, rel_tol=MAX_REL_TOLERANCE)


def compare_numeric(name, a, b, problems, limits=None, observed=None):
    limits = limits or {}
    observed = observed if observed is not None else {}
    (da, sa, ra), (db, sb, rb) = a, b
    if da != db or sa != sb:
        problems.append("%s: dtype/shape %s %s -> %s %s" % (name, da, list(sa), db, list(sb)))
        return
    floating = da in ("f32", "f64")
    if not floating and ra == rb:
        return
    code, size = DT[da]
    n = len(ra) // size
    va = struct.unpack("<%d%s" % (n, code), ra)
    vb = struct.unpack("<%d%s" % (n, code), rb)
    if floating and ra == rb and all(math.isfinite(v) for v in va):
        observed.setdefault(name, 0.0)
        return
    failed = []
    worst, at = 0.0, 0
    for i in range(n):
        x, y = va[i], vb[i]
        same = float_within_limit(name, x, y, limits, observed) if floating else x == y
        if same:
            continue
        failed.append(i)
        d = abs(x - y) if math.isfinite(x) and math.isfinite(y) else math.inf
        if d > worst:
            worst, at = d, i
    if failed:
        problems.append("%s: %d of %d values differ%s (largest |diff| %g at flat index %d: %r -> %r)"
                        % (name, len(failed), n, " beyond tolerance" if floating else "",
                           worst, at, va[at], vb[at]))


def compare_text(name, a, b, problems, added, added_zero, limits=None, observed=None, ignore_values=()):
    limits = limits or {}
    observed = observed if observed is not None else {}
    ta, tb = a[2].decode(), b[2].decode()
    va = debug_values(name, ta)
    vb = va if ta == tb else debug_values(name, tb)
    bad = 0
    for k, x in va.items():
        if k in ignore_values:
            continue
        y = vb.get(k)
        field = name + ":" + re.sub(r"\[\d+\]", "[]", k)
        if y is None:
            if bad < 20:
                problems.append("%s: %s is gone (was %s)" % (name, k, x[1]))
            bad += 1
        else:
            floating = x[0] == y[0] == "num" and not re.fullmatch(r"-?\d+", x[1]) and not re.fullmatch(r"-?\d+", y[1])
            if floating:
                same = float_within_limit(field, num(x[1]), num(y[1]), limits, observed)
            else:
                same = x == y
            if same:
                continue
            if bad < 20:
                problems.append("%s: %s %s -> %s" % (name, k, x[1], y[1] if len(y) > 1 else y))
            bad += 1
    if bad > 20:
        problems.append("%s: (more differences not listed)" % name)
    for k, y in vb.items():
        if k in ignore_values:
            continue
        if k not in va:
            added.append("%s: %s = %s" % (name, k, y[1]))
            if added_zero and not zero_value(y) and not k.endswith(".len"):
                problems.append("%s: added %s is not zero (%s)" % (name, k, y[1]))


def compare_files(pa, pb, args):
    ha, fa = load(pa)
    hb, fb = load(pb)
    problems, added = [], []
    for name, a in fa.items():
        if name in args.ignore:
            continue
        b = fb.get(name)
        if b is None:
            problems.append("%s: missing from the new dump" % name)
        elif a[0] == "utf8" or b[0] == "utf8":
            if a[0] != b[0]:
                problems.append("%s: dtype %s -> %s" % (name, a[0], b[0]))
            else:
                compare_text(name, a, b, problems, added, args.added_zero, args.limits, args.observed,
                             getattr(args, "ignore_values", {}).get(name, ()))
        else:
            compare_numeric(name, a, b, problems, args.limits, args.observed)
    for name, b in fb.items():
        if name in fa or name in args.ignore:
            continue
        added.append("%s (%s %s)" % (name, b[0], list(b[1])))
        if args.added_zero and b[0] != "utf8":
            code, size = DT[b[0]]
            vals = struct.unpack("<%d%s" % (len(b[2]) // size, code), b[2])
            nz = sum(1 for v in vals if v != 0)
            if nz:
                problems.append("%s: added field has %d nonzero values" % (name, nz))
    return ha, problems, added


def state_files(d):
    out = {}
    for n in sorted(os.listdir(d)):
        m = re.match(r"(chunk-\d+)\.state(\.xz|\.gz)?$", n)
        if m:
            out[m.group(1)] = os.path.join(d, n)
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("baseline")
    ap.add_argument("new")
    ap.add_argument("--added-zero", action="store_true", help="fields only the new dump has must be zero")
    ap.add_argument("--ignore", action="append", default=[], help="skip a field (repeatable)")
    ap.add_argument("--quiet", action="store_true", help="print only differences and the verdict")
    ap.add_argument("--linux-tolerance", action="store_true", help="compatibility alias: numeric tolerance is always enabled")
    args = ap.parse_args()
    args.limits = ABS_LIMITS
    args.observed = {}
    try:
        if os.path.isdir(args.baseline):
            if not os.path.isdir(args.new):
                raise FormatError("%s is a directory but %s is not" % (args.baseline, args.new))
            fa, fb = state_files(args.baseline), state_files(args.new)
            if not fa:
                raise FormatError("%s holds no chunk-*.state files" % args.baseline)
            pairs = []
            for k, p in fa.items():
                if k not in fb:
                    print("%s: missing from %s" % (k, args.new))
                    pairs.append((k, p, None))
                else:
                    pairs.append((k, p, fb[k]))
            extra = sorted(set(fb) - set(fa))
            if extra:
                print("only in the new dumps (not compared): %s" % ", ".join(extra))
        else:
            pairs = [(os.path.basename(args.baseline), args.baseline, args.new)]
        failed = 0
        for k, pa, pb in pairs:
            if pb is None:
                failed += 1
                continue
            _, problems, added = compare_files(pa, pb, args)
            if problems:
                failed += 1
                print("%s: DIFFERENT" % k)
                for p in problems:
                    print("  " + p)
            elif not args.quiet:
                print("%s: within numeric tolerance on every baseline field" % k)
            if added and not args.quiet:
                print("  only in the new dump: %d (%s%s)" % (len(added), "; ".join(added[:6]), "; ..." if len(added) > 6 else ""))
    except FormatError as e:
        print("state_compare: %s" % e, file=sys.stderr)
        return 2
    for name, observed in sorted(args.observed.items()):
        print("%s: maximum allowed |diff| %.9g; maximum observed |diff| %.9g; relative tolerance %.9g" %
              (name, args.limits.get(name, DEFAULT_ABS_TOLERANCE), observed, MAX_REL_TOLERANCE))
    verdict = "DIFFERENT" if failed else "WITHIN NUMERIC TOLERANCE"
    print("state_compare: %s (%d of %d chunks differ)" % (verdict, failed, len(pairs)))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
