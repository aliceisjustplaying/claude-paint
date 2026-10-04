# /// script
# dependencies = ["pillow"]
# ///
"""Strips for the damp-wipe candidates (rag::tests::thin::damp_render).

    uv run notes/rag/thin-experiment/damp.py <png dir> <out dir>
"""
import sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
PX, X0, Y0 = 2.4, 260.0, 190.0
box = lambda u0, v0, u1, v1: tuple(round(t) for t in ((u0 - X0) * PX, (v0 - Y0) * PX, (u1 - X0) * PX, (v1 - Y0) * PX))
WORK, EDGE = box(280, 260, 720, 420), box(560, 285, 680, 365)
font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 26)


def img(n, area, k):
    im = Image.open(src / f"{n}.png").convert("RGB").crop(area)
    return im.resize((round(im.width * k), round(im.height * k)), Image.LANCZOS if k < 1 else Image.NEAREST)


def strip(cells, path):
    w, h = cells[0][1].size
    out = Image.new("RGB", (len(cells) * (w + 10) - 10, h + 40), (32, 32, 32))
    d = ImageDraw.Draw(out)
    for k, (t, im) in enumerate(cells):
        d.text((k * (w + 10) + 4, 6), t, fill=(240, 240, 240), font=font)
        out.paste(im, (k * (w + 10), 40))
    out.save(path, quality=92)


for n in ["mild", "mild-press"]:
    for w in ["dry", "damp"]:
        strip([(f"{w}, 1 wipe", img(f"{n}-{w}1", WORK, 0.5)), (f"{w}, 3 wipes", img(f"{n}-{w}3", WORK, 0.5)), (f"close-up of the edge, {w} 1", img(f"{n}-{w}1", EDGE, 1.8))], dst / f"edge-{n}-{w}.jpg")
