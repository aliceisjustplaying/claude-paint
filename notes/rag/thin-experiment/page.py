# /// script
# dependencies = ["pillow"]
# ///
"""Plain-language page for the thin-film rag experiment.

    uv run notes/rag/thin-experiment/page.py <png dir> <out dir>

Uses the brush-wash start (wash05) of rag::tests::thin::experiment. For each
rule: one strip of the painted area (before, dry x1, dry x3, damp x1) and a
close-up of dry x3 and damp x1.
"""
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
dst.mkdir(parents=True, exist_ok=True)
PX, X0, Y0 = 2.4, 260.0, 190.0


def box(u0, v0, u1, v1):
    return tuple(round(t) for t in ((u0 - X0) * PX, (v0 - Y0) * PX, (u1 - X0) * PX, (v1 - Y0) * PX))


WORK = box(280, 260, 720, 420)
CLOSE = box(450, 300, 550, 380)
try:
    font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 26)
except OSError:
    font = ImageFont.load_default()


def img(name, area, scale):
    im = Image.open(src / f"{name}.png").convert("RGB").crop(area)
    return im.resize((round(im.width * scale), round(im.height * scale)), Image.LANCZOS if scale < 1 else Image.NEAREST)


def strip(cells, path):
    w, h = cells[0][1].size
    gap, top = 10, 40
    out = Image.new("RGB", (len(cells) * (w + gap) - gap, h + top), (32, 32, 32))
    d = ImageDraw.Draw(out)
    for k, (label, im) in enumerate(cells):
        x = k * (w + gap)
        d.text((x + 4, 6), label, fill=(240, 240, 240), font=font)
        out.paste(im, (x, top))
    out.save(path, quality=92)


RULES = ["Floor", "Zero", "Fixed", "Soft"]
for r in RULES:
    strip([("before", img("wash05-start", WORK, 0.5)), ("dry rag, 1 wipe", img(f"wash05-{r}-dry1", WORK, 0.5)),
           ("dry rag, 3 wipes", img(f"wash05-{r}-dry3", WORK, 0.5)), ("damp rag, 1 wipe", img(f"wash05-{r}-damp1", WORK, 0.5))],
          dst / f"simple-{r}.jpg")
    strip([("close-up: dry rag, 3 wipes", img(f"wash05-{r}-dry3", CLOSE, 3)), ("close-up: damp rag, 1 wipe", img(f"wash05-{r}-damp1", CLOSE, 3))],
          dst / f"simple-{r}-close.jpg")
print("written", dst)
