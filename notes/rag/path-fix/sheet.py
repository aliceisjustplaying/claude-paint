# /// script
# dependencies = ["pillow", "numpy"]
# ///
"""Sheets for the engine-3 rag path fix and transfer checks.

    uv run notes/rag/path-fix/sheet.py <before dir> <after dir> <out dir>

The directories hold `rag::tests::path::dump` output (`RAG_PNG`): `<name>.png`
(the canvas, dried) and `<name>.vol` (u32 width, u32 height, µm of open paint
per pixel, f32 LE). <before dir> comes from the base revision's probe run
(partition only), <after dir> from the fixed revision (partition and
rag::tests::path3::ledger).

Writes path.png (same motion given with few and many points, before and
after: pictures and |difference| maps) and ledger-smooth.png,
ledger-linen.png (each controlled-film case: start, end, film left as a
share of the fixture's thickness).
"""
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

before, after, out = (Path(a) for a in sys.argv[1:4])
out.mkdir(parents=True, exist_ok=True)
UNITS = (1000.0, 1000.0 / 1.5)
BAND = (150.0, 230.0, 850.0, 450.0)  # the painted band and a margin, units
try:
    font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 18)
except OSError:
    font = ImageFont.load_default()


def vol(path):
    b = path.read_bytes()
    w, h = np.frombuffer(b[:8], "<u4")
    return np.frombuffer(b[8:], "<f4").reshape(h, w)


def crop_box(w, h):
    sx, sy = w / UNITS[0], h / UNITS[1]
    return (round(BAND[0] * sx), round(BAND[1] * sy), round(BAND[2] * sx), round(BAND[3] * sy))


def picture(d, name):
    im = Image.open(d / f"{name}.png").convert("RGB")
    return im.crop(crop_box(*im.size))


def field(d, name):
    v = vol(d / f"{name}.vol")
    x0, y0, x1, y1 = crop_box(v.shape[1], v.shape[0])
    return v[y0:y1, x0:x1]


def heat(a, top):
    """0 black .. top white, through a warm ramp; above top, cyan."""
    t = np.clip(a / top, 0.0, 1.0)
    rgb = np.stack([np.clip(1.6 * t, 0, 1), np.clip(1.6 * t - 0.5, 0, 1), np.clip(2.0 * t - 1.0, 0, 1)], -1)
    rgb[a > top] = (0.2, 0.9, 1.0)
    return Image.fromarray((rgb * 255).astype(np.uint8))


def sheet(rows, labels, heads, title, path):
    w, h = rows[0][0].size
    lab, gap, top = 260, 6, 56
    W = lab + len(heads) * (w + gap)
    H = top + len(rows) * (h + gap)
    s = Image.new("RGB", (W, H), (30, 30, 30))
    d = ImageDraw.Draw(s)
    d.text((8, 6), title, fill=(235, 235, 235), font=font)
    for k, hd in enumerate(heads):
        d.text((lab + k * (w + gap), 30), hd, fill=(200, 200, 200), font=font)
    for r, (cells, label) in enumerate(zip(rows, labels)):
        y = top + r * (h + gap)
        d.multiline_text((8, y + 4), label, fill=(235, 235, 235), font=font, spacing=2)
        for k, c in enumerate(cells):
            s.paste(c.resize((w, h)), (lab + k * (w + gap), y))
    s.save(path)
    print(path)


# 1. path partition: 3 µm direct film, linen and smooth, dry wipe
for g in ("linen", "smooth"):
    rows, labels = [], []
    for name, plain, dense in (("line", 2, 51), ("reversal", 3, 51), ("corner", 3, 51)):
        cells = []
        for d in (before, after):
            a, b = f"part-{g}-{name}-{plain}", f"part-{g}-{name}-{dense}"
            cells += [picture(d, a), picture(d, b), heat(np.abs(field(d, b) - field(d, a)), 0.5)]
        rows.append(cells)
        labels.append(f"{name}\n{plain} vs {dense} points")
    sheet(rows, labels,
          ["before: few points", "before: many points", "before: |Δ| 0..0.5 µm", "after: few points", "after: many points", "after: |Δ| 0..0.5 µm"],
          f"Same continuous motion, few vs many collinear input points ({g}, direct 3 µm raw sienna, dry rag, pressure 0.8). Cyan: |Δ| > 0.5 µm.",
          out / f"path-{g}.png")

# 2. controlled-film ledger cases
CASES = [
    ("film0.25-dry", "0.25 µm, dry wipe"), ("film1-dry", "1 µm, dry wipe"), ("film1-damp", "1 µm, damp wipe (dip 0.5)"),
    ("film3-dry", "3 µm, dry wipe"), ("film10-dry", "10 µm, dry wipe"), ("film69-dry", "69 µm stress, dry wipe"),
    ("film3-reversal", "3 µm, reversal"), ("film3-corner", "3 µm, corner"), ("film3-blot", "3 µm, blot"),
    ("two-dirty", "umber | white, one wipe\nacross both"), ("two-lifted", "umber, lift,\nsame face on white"),
    ("two-refold", "umber, refold,\nwhite"), ("under-damp", "1 µm over dry 20 µm\number, damp (dip 1)"),
]
for g in ("smooth", "linen"):
    rows, labels = [], []
    for name, label in CASES:
        s0, s1 = field(after, f"ledger-{g}-{name}-start"), field(after, f"ledger-{g}-{name}-end")
        # the fixture's thickness (the brushed ridges outside it are thicker)
        ref = max(float(np.median(s0[s0 > 0])), 1e-6)
        rows.append([picture(after, f"ledger-{g}-{name}-start"), picture(after, f"ledger-{g}-{name}-end"), heat(s1 / ref, 1.0)])
        labels.append(label)
    sheet(rows, labels, ["start", "after", "open film / fixture thickness (0..1; cyan > 1)"],
          f"Controlled direct films, {g} ground, engine 3 rag after the path fix (pressure 0.8, pad 100 units = 44 mm)",
          out / f"ledger-{g}.png")
