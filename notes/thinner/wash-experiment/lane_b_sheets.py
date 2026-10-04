# /// script
# dependencies = ["pillow", "numpy"]
# ///
"""Lane B sheets: thinner::tests::flow_pictures before (hard 2 µm floor, 73b8129)
and after (the candidate: bounded h³ slowing, partial-tick flow).

    uv run notes/thinner/wash-experiment/lane_b_sheets.py <png dir> <out dir>

<png dir> holds flow-old.png and flow-new.png: 2400-px renders of the crop window
260..740 x 190..490 units (2.4 px per unit), after a five-minute wait.
"""
import sys
from pathlib import Path
import numpy as np
from PIL import Image, ImageDraw, ImageFont

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
PX, X0, Y0 = 2.4, 260.0, 190.0
box = lambda u0, v0, u1, v1: tuple(round(t) for t in ((u0 - X0) * PX, (v0 - Y0) * PX, (u1 - X0) * PX, (v1 - Y0) * PX))
font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 26)
old, new = (Image.open(src / f"flow-{n}.png").convert("RGB") for n in ("old", "new"))


def img(im, area, k):
    im = im.crop(area)
    return im.resize((round(im.width * k), round(im.height * k)), Image.LANCZOS if k < 1 else Image.NEAREST)


def diff(area, k, gain=8):
    a = np.asarray(old.crop(area), dtype=np.int16)
    b = np.asarray(new.crop(area), dtype=np.int16)
    d = np.clip(np.abs(a - b).max(axis=2) * gain, 0, 255).astype(np.uint8)
    return img(Image.fromarray(d).convert("RGB"), (0, 0, d.shape[1], d.shape[0]), k)


def strip(cells, path):
    w, h = cells[0][1].size
    out = Image.new("RGB", (len(cells) * (w + 10) - 10, h + 40), (32, 32, 32))
    d = ImageDraw.Draw(out)
    for k, (t, im) in enumerate(cells):
        d.text((k * (w + 10) + 4, 6), t, fill=(240, 240, 240), font=font)
        out.paste(im, (k * (w + 10), 40))
    out.save(path)


ALL, CLOSE = box(270, 200, 730, 480), box(560, 255, 680, 300)
strip([("old: hard 2 µm floor", img(old, ALL, 0.6)), ("candidate: bounded h³ slowing", img(new, ALL, 0.6)), ("|difference| × 8", diff(ALL, 0.6))], dst / "lane-b-flow.png")
strip([("old, thinner 0.75 stroke", img(old, CLOSE, 2.5)), ("candidate, same place", img(new, CLOSE, 2.5)), ("|difference| × 8", diff(CLOSE, 2.5))], dst / "lane-b-flow-close.png")
a, b = (np.asarray(im.crop(ALL), dtype=np.float64) for im in (old, new))
d = np.abs(a - b)
print(f"grey levels (0-255), whole sheet area: mean |Δ| {d.mean():.3f}, max {d.max():.0f}; pixels with any channel Δ>=2: {(d.max(axis=2) >= 2).mean() * 100:.2f}%")
for name, (u0, v0, u1, v1) in [("t0.50 stroke", (290, 215, 710, 250)), ("t0.75 stroke", (290, 260, 710, 295)), ("t0.90 stroke", (290, 305, 710, 340)), ("broad wash t0.50", (290, 370, 710, 480))]:
    r = box(u0, v0, u1, v1)
    dd = np.abs(np.asarray(old.crop(r), dtype=np.float64) - np.asarray(new.crop(r), dtype=np.float64))
    print(f"  {name}: mean |Δ| {dd.mean():.3f}, max {dd.max():.0f}")
