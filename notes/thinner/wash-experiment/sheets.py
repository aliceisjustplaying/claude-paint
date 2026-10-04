# /// script
# dependencies = ["pillow"]
# ///
"""Sheets for the thinner flow and wet-wash experiment (thinner::tests::flow_pictures, wet_rules).

    uv run notes/thinner/wash-experiment/sheets.py <png dir> <out dir>

The PNGs are 2400-px renders of the crop window 260..740 x 190..490 units (2.4 px per unit).
"""
import sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
PX, X0, Y0 = 2.4, 260.0, 190.0
box = lambda u0, v0, u1, v1: tuple(round(t) for t in ((u0 - X0) * PX, (v0 - Y0) * PX, (u1 - X0) * PX, (v1 - Y0) * PX))
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


ALL, CLOSE = box(270, 200, 730, 480), box(560, 255, 680, 300)
strip([("old: never flows below 2 µm", img("flow-old", ALL, 0.6)), ("new: thin films flow slowly", img("flow-new", ALL, 0.6))], dst / "flow.jpg")
strip([("old, close-up of the thinner-0.75 stroke", img("flow-old", CLOSE, 2.5)), ("new, same place", img("flow-new", CLOSE, 2.5))], dst / "flow-close.jpg")
WASH = box(280, 245, 720, 455)
for r in (0, 1, 2):
    strip([("one pass", img(f"r{r}-1pass", WASH, 0.5)), ("three passes, wet", img(f"r{r}-3wet", WASH, 0.5)), ("three passes, dried between", img(f"r{r}-3dry", WASH, 0.5))], dst / f"wash-r{r}.jpg")
