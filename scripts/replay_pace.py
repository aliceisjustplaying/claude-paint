#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "pillow"]
# ///
"""Clip schedules for scripts/replay_clip (numeric or dynamic --pace).

    replay_pace.py <frames-dir> --length T [--pace dynamic|P]
                   [--max-hold 1] [--hold 3] [--fps 24] [--final-first 0]
                   [--floor 0.2] [--gamma 0.7] [--hand-pace 0.6] [--min-frames 2]
                   [--ramp 1] [--ramp-span 0.35] [--open-hold 0] [--before 0] [--eval]

Writes <frames-dir>/concat.txt (ffmpeg concat list) from frames.tsv. Numeric
pace uses hand_time^P steps, capped and rescaled without frame quantization.
It needs only Python's stdlib. --ramp, --ramp-span and --open-hold are ignored
for numeric pace, preserving replay_clip's original numeric schedule.

Dynamic pace (the default for direct callers) needs numpy and Pillow: a
frame's screen time follows how much the picture visibly changed when it
arrived, not how long the hand took:

  change_i  mean absolute difference between moment i-1 and moment i, sRGB
            0..255 averaged over channels, at 250 px wide (cached in
            pace-change.tsv); smoothed 1/4, 1/2, 1/4 over neighbors so a single
            blip doesn't spike
  weight_i  (1 - floor) * change_i^gamma / sum  +  floor * hand_i / sum
            hand_i: the step of hand_time^hand_pace (the numeric --pace
            schedule), so quiet stretches still move and order stays readable
  time_i    weight_i scaled to fill --length minus the final hold, capped at
            --max-hold; then quantized to whole video frames, carrying the
            remainder forward: a frame whose time is under --min-frames video
            frames is skipped and its time given to the next one shown, so
            nothing flickers past

The final picture is held --hold seconds; with --open-hold S the bare canvas
opens the clip for S seconds, taken out of the rest. A --length the moments
can't fill, held at most --max-hold each, is refused with the longest they
can (--before: seconds of clip before this list, replay_clip's --final-first,
counted in that message). --final-first opens on the finished painting and
subtracts that time from --length (rounded to five decimals, as replay_clip
has always done); unlike --before, it also writes the prefix. With --eval,
dynamic pace prints how evenly visible change is spread over the clip (per
tenth).
"""

import argparse
import ctypes
import errno
import math
import sys
from pathlib import Path

W = 250


# The numeric path replaces the system awk (One True Awk). Use the same C
# strtod conversion: numeric prefixes, hex, infinities and underflow included.
_STRTOD = ctypes.CDLL(None, use_errno=True).strtod
_STRTOD.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_char_p)]
_STRTOD.restype = ctypes.c_double


def awk_value(text):
    raw, end = str(text).encode(), ctypes.c_char_p()
    ctypes.set_errno(0)
    value = _STRTOD(raw, ctypes.byref(end))
    numeric = (end.value != raw and not end.value.strip(b" \t\n")
               and value != math.inf and ctypes.get_errno() != errno.ERANGE)
    return value, numeric


def awk_string(value):
    # awk's derived integral values use %.30g, not CONVFMT's %.6g.
    return format(value, ".30g" if math.isfinite(value) and value.is_integer() else ".6g")


def awk_compare(x, y):
    # One True Awk compares the difference, treating NaN as equality.
    d = x - y
    return -1 if d < 0 else 1 if d > 0 else 0


def awk_power(x, exponent):
    if exponent >= 0 and math.modf(exponent)[0] == 0:
        # awk uses recursive squaring for nonnegative integer exponents;
        # its C int conversion saturates at INT_MAX on the serving Mac.
        def ipow(n):
            if n <= 0:
                return 1.0
            v = ipow(n // 2)
            return v * v if n % 2 == 0 else x * v * v
        return ipow(min(int(exponent), 2147483647) if math.isfinite(exponent) else 2147483647)
    try:
        return math.pow(x, exponent)
    except OverflowError:
        return -math.inf if x < 0 and exponent % 2 == 1 else math.inf
    except ValueError:
        if x == 0 and exponent < 0:
            return -math.inf if math.copysign(1, x) < 0 and exponent % 2 == 1 else math.inf
        return math.nan


def numeric_moments(d: Path):
    """awk's LF-only records and $2 + 0, including blank/missing fields."""
    records = (d / "frames.tsv").read_bytes().decode().split("\n")
    if records[-1] == "":
        records.pop()  # a terminating LF is not an extra record
    out = []
    for line in records[1:]:
        fields = line.split("\t")
        f = fields[0]
        h = awk_value(fields[1] if len(fields) > 1 else "")[0]
        if out and awk_compare(out[-1][1], h) == 0:
            out[-1] = (f, h)
        else:
            out.append((f, h))
    return out


def moments(d: Path):
    """frames.tsv rows as (file, hand_secs), frames at the same hand time
    merged into the last one (as it is after any wait)."""
    out = []
    for line in (d / "frames.tsv").read_text().splitlines()[1:]:
        f, h = line.split("\t")[:2]
        h = float(h)
        if out and out[-1][1] == h:
            out[-1] = (f, h)
        else:
            out.append((f, h))
    return out


def load(p: Path):
    import numpy as np
    from PIL import Image

    im = Image.open(p).convert("RGB")
    return np.asarray(im.resize((W, max(1, round(im.height * W / im.width))), Image.BILINEAR), dtype=np.float32)


def changes(d: Path, ms):
    """change_i (arriving at moment i; 0 for the first), cached per file pair."""
    import numpy as np

    cache = d / "pace-change.tsv"
    known = {}
    if cache.exists():
        for line in cache.read_text().splitlines():
            a, b, c = line.split("\t")
            known[(a, b)] = float(c)
    out, prev, prev_f, new = [0.0], None, ms[0][0], []
    for f, _ in ms[1:]:
        key = (prev_f, f)
        if key not in known:
            if prev is None:
                prev = load(d / prev_f)
            cur = load(d / f)
            known[key] = float(np.abs(cur - prev).mean())
            new.append(key)
            prev = cur
        else:
            prev = None
        out.append(known[key])
        prev_f = f
    if new:
        cache.write_text("".join(f"{a}\t{b}\t{known[(a, b)]:.5f}\n" for a, b in known))
    return np.array(out)


def schedule(c, h, a):
    """Seconds per moment but the last, which the final hold shows."""
    import numpy as np

    n = len(c)
    cs = c.copy()
    cs[1:-1] = 0.25 * c[:-2] + 0.5 * c[1:-1] + 0.25 * c[2:]
    ch = cs[1:] ** a.gamma
    hand = np.diff(h ** a.hand_pace)
    w = np.zeros(n)
    w[1:] = (1 - a.floor) * ch / max(ch.sum(), 1e-12) + a.floor * hand / max(hand.sum(), 1e-12)
    w[0] = 0.0  # the bare ground arrives with no change: its time comes from the next frame's
    if a.ramp < 1:
        # speed up the opening: moments in the first --ramp-span of hand time get
        # their weight scaled from --ramp up to 1 (smoothstep), so the lay-in
        # and first washes go by quickly and the later work keeps its time
        u = np.clip(h[: n] / max(h[-1], 1e-12) / max(a.ramp_span, 1e-6), 0, 1)
        w *= a.ramp + (1 - a.ramp) * (u * u * (3 - 2 * u))
    w = w[:-1]  # the last moment's time is the hold, not a share of the rest
    want = a.length - a.hold
    k = want / w.sum()
    for _ in range(50):
        capped = w * k > a.max_hold
        free = w[~capped].sum()
        if free <= 0:
            break
        k2 = (want - capped.sum() * a.max_hold) / free
        if abs(k2 - k) < 1e-9:
            break
        k = k2
    # each frame is held for the change it brought (held after a big change,
    # so the new paint can be seen)
    return np.minimum(w * k, a.max_hold)


def quantize(t, fps, min_frames):
    """Whole video frames per moment, carrying remainders forward; frames
    under min_frames are skipped (their time goes to the next shown)."""
    frames, carry = [], 0.0
    for x in t:
        carry += x * fps
        if carry + 1e-6 >= min_frames:
            n = int(carry + 1e-6)  # a whole frame summed as 23.9999... is a frame
            frames.append(n)
            carry -= n
        else:
            frames.append(0)
    return frames


def entry(name: str) -> str:
    """A concat list's file line for a frame beside the list (ffmpeg reads it
    relative to the list): each quote in the name closes, escapes and reopens."""
    return "file '" + name.replace("'", "'\\''") + "'\n"


def write_schedule(ms, secs, a):
    """One concat encoding for either pace, including both finished-frame holds."""
    lines = []
    if a.final_first is not None and a.before > 0:
        lines.append(f"{entry(a.first_name)}duration {a.before:.5f}\n")
    for (f, _), d in zip(ms[:-1], secs):
        if d > 0:
            lines.append(f"{entry(f)}duration {d:.5f}\n")
    last = entry(ms[-1][0] if ms else "")
    # The final entry takes the duration before it: hold less two frames,
    # a one-frame repeat, then the last frame once more (also one frame).
    lines.append(f"{last}duration {a.hold - 2 / a.fps:.5f}\n{last}duration {1 / a.fps:.5f}\n{last}")
    (a.dir / "concat.txt").write_text("".join(lines))


def numeric_schedule(ms, a):
    """The original numeric policy: no quantization or dynamic opening flags.

    Keep the scalar operation order and 20 capped-rescaling iterations from
    replay_clip's awk schedule, including its feasibility tolerances/messages.
    """
    if awk_compare(a.length, a.hold) <= 0:
        sys.exit(f"replay_clip: --length must exceed the {a.hold_text} s final hold")
    pace = awk_value(a.pace)[0]
    w = [awk_power(later[1], pace) - awk_power(earlier[1], pace) for earlier, later in zip(ms, ms[1:])]
    want = a.length - a.hold
    m = sum(x > 0 for x in w)
    if want > m * a.max_hold + 1e-6:
        msg = (f"replay_clip: --length {awk_string(a.length + a.before)} is longer than the clip can run: "
               f"{m} moment{'' if m == 1 else 's'} held at most {a.max_hold_text} s each (--max-hold) "
               f"plus the {a.hold_text} s final hold")
        if a.before > 0:
            msg += f" plus the {a.final_first if a.final_first is not None else awk_string(a.before)} s --final-first"
        msg += f" come to at most {math.modf((m * a.max_hold + a.hold + a.before) * 100 + 1e-6)[1] / 100:.2f} s; "
        if m > 0:
            msg += f"raise --max-hold to at least {math.modf(want / m * 100 + 0.999999)[1] / 100:.2f} or "
        sys.exit(msg + "lower --length")
    def over_cap(d):
        return (awk_compare(d, a.max_hold) > 0 if a.cap_numeric
                else awk_string(d) > a.max_hold_text)

    k = 0.0
    for _ in range(20):
        capped, free = 0.0, 0.0
        for x in w:
            if k > 0 and over_cap(x * k):
                capped += a.max_hold
            else:
                free += x
        if awk_compare(free, 0) <= 0:
            break
        k2 = (want - capped) / free
        if awk_compare(k2, k) == 0:
            break
        k = k2
    secs, total, lines = [], 0.0, []
    for x in w:
        d = x * k
        if over_cap(d):
            d = a.max_hold
        secs.append(d)
        if awk_compare(d, 0) > 0:
            lines.append(f"{entry(ms[len(secs) - 1][0])}duration {d:.5f}\n")
            total += d
    # awk writes these entries before its consistency check. A refusal must
    # leave the same partial concat file (without prefix or final hold).
    (a.dir / "concat.txt").write_text("".join(lines))
    if total - want > 0.5 / a.fps or want - total > 0.5 / a.fps:
        sys.exit(f"replay_clip: the holds come to {total:.3f} s, not {want:.3f}")
    print(f"{len(ms)} moments, {(ms[-1][1] if ms else 0) / 60:.1f} min of hand time, pace {a.pace}: about {total + a.hold:.1f} s of clip", file=sys.stderr)
    return secs


def evenness(c, secs):
    """sqrt-change shown per tenth of the clip (%), as a vector."""
    import numpy as np

    t = np.cumsum(secs) / max(sum(secs), 1e-12)
    b = np.minimum((t * 10).astype(int), 9)
    s = np.sqrt(c)
    v = np.array([s[b == k].sum() for k in range(10)])
    return 100 * v / v.sum()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dir", type=Path)
    ap.add_argument("--pace", default="dynamic", help="dynamic or a numeric hand-time exponent")
    ap.add_argument("--length", default="50")
    ap.add_argument("--max-hold", default="1.0")
    ap.add_argument("--hold", default="3.0")
    ap.add_argument("--fps", type=int, default=24)
    ap.add_argument("--floor", default=0.2)
    ap.add_argument("--gamma", default=0.7)
    ap.add_argument("--hand-pace", type=float, default=0.6)
    ap.add_argument("--min-frames", type=int, default=2)
    ap.add_argument("--ramp", default=1.0, help="opening speed-up: weight factor at the start (1 = none)")
    ap.add_argument("--open-hold", default=0.0, help="seconds the bare canvas is shown first")
    ap.add_argument("--ramp-span", default=0.35, help="share of hand time over which the ramp eases to 1")
    ap.add_argument("--before", default="0.0", help="seconds of clip before this list, for the length message")
    ap.add_argument("--final-first", help="open on the finished painting, taken out of --length")
    ap.add_argument("--eval", action="store_true", help="print evenness for this schedule and for plain --hand-pace")
    a = ap.parse_args()
    # Keep literal hold/cap spellings for the numeric path's existing messages.
    a.hold_text, a.max_hold_text = a.hold, a.max_hold
    if a.pace != "dynamic":
        a.hold = awk_value(a.hold)[0]
        a.max_hold, a.cap_numeric = awk_value(a.max_hold)
        a.before = awk_value(a.final_first if a.final_first is not None else a.before)[0]
        a.length = awk_value(a.length)[0]
        if a.final_first is not None:
            a.length = awk_value(f"{a.length - a.before:.5f}")[0]
        (a.dir / "concat.txt").write_text("")
        ms = numeric_moments(a.dir)
        # The old prefix uses tail -1, including a blank final row/header.
        a.first_name = (a.dir / "frames.tsv").read_bytes().decode().removesuffix("\n").split("\n")[-1].split("\t")[0]
        secs = numeric_schedule(ms, a)
        write_schedule(ms, secs, a)
        return
    numbers = ["length", "hold", "max_hold", "before"]
    if a.pace == "dynamic":
        numbers += ["floor", "gamma", "ramp", "open_hold", "ramp_span"]
    # Numeric pace ignores dynamic-only flags, even nonnumeric values, just
    # as the original shell path did. Only dynamic pace interprets them.
    for name in numbers:
        value = getattr(a, name)
        try:
            setattr(a, name, float(value))
        except ValueError:
            ap.error(f"argument --{name.replace('_', '-')}: invalid float value: {value!r}")
    if a.final_first is not None:
        a.before = float(a.final_first)
        a.length = float(f"{a.length - a.before:.5f}")
    ms = moments(a.dir)
    a.first_name = ms[-1][0]
    secs = dynamic_schedule(ms, a)
    write_schedule(ms, secs, a)


def dynamic_schedule(ms, a):
    import numpy as np

    if a.length <= a.hold + a.open_hold:
        sys.exit(f"replay_pace: --length must exceed the {a.hold} s final hold plus the {a.open_hold} s opening hold")
    opening = round(a.open_hold * a.fps)
    h = np.array([x[1] for x in ms])
    c = changes(a.dir, ms)
    # the moments shown, each held at most --max-hold (in whole video frames),
    # may not fill --length
    m = int((schedule(c, h, a) > 0).sum())
    want = a.length - a.hold - opening / a.fps
    most = math.floor(m * a.max_hold * a.fps + 1e-6) / a.fps
    if want > most + 1e-6:
        before = f" plus the {a.before:g} s --final-first" if a.before > 0 else ""
        opened = f" plus the {opening / a.fps:g} s --open-hold" if opening else ""
        raise_ = f"raise --max-hold to at least {math.ceil(math.ceil(want * a.fps - 1e-6) / a.fps / m * 100) / 100:.2f} or " if m else ""
        sys.exit(
            f"replay_pace: --length {a.length + a.before:g} is longer than the clip can run: {m} moment{'' if m == 1 else 's'} held at most {a.max_hold:g} s each (--max-hold)"
            f" plus the {a.hold:g} s final hold{opened}{before} come to at most {math.floor((most + a.hold + opening / a.fps + a.before) * 100 + 1e-6) / 100:.2f} s;"
            f" {raise_}lower --length"
        )
    # quantizing drops a little time (skipped frames' remainders): aim a
    # little long until the clip lands on --length
    target, aim = a.length, a.length
    for _ in range(20):
        a.length = aim - opening / a.fps
        fr = quantize(schedule(c, h, a), a.fps, a.min_frames)
        fr[0] += opening  # the bare canvas: its weight is 0, so it's shown only if held
        got = sum(fr) / a.fps + a.hold
        if abs(got - target) < 0.5 / a.fps:
            break
        aim += target - got
    if abs(got - target) >= 0.5 / a.fps:
        sys.exit(f"replay_pace: the schedule runs {got:.3f} s, not the {target} s asked for")
    a.length = target
    shown = sum(n > 0 for n in fr)
    total = sum(fr) / a.fps + a.hold
    print(f"{len(ms)} moments, {shown + 1} shown, {h[-1] / 60:.1f} min of hand time, pace dynamic (floor {a.floor}, gamma {a.gamma}): about {total:.1f} s of clip", file=sys.stderr)
    if a.eval:
        secs = [n / a.fps for n in fr] + [0.0]
        e = evenness(c, secs)
        # the numeric --pace schedule: each frame held for the hand time to
        # the next, on the curve hand_time^pace
        hand = np.append(np.diff(h ** a.hand_pace), 0.0)
        e2 = evenness(c, hand / hand.sum() * (a.length - a.hold))
        for name, v in (("dynamic", e), (f"hand^{a.hand_pace}", e2)):
            print(f"{name:10s} per tenth: {' '.join(f'{x:4.1f}' for x in v)}   min {v.min():.1f} max {v.max():.1f} sd {v.std():.1f}", file=sys.stderr)
    return [n / a.fps for n in fr]


if __name__ == "__main__":
    main()
