# /// script
# dependencies = ["pillow"]
# ///
"""Image sheets for the thin-film rag experiment (rag::tests::thin::experiment).

    uv run notes/rag/thin-experiment/sheet.py <png dir> <out dir>

The PNGs are 2400-px renders of the crop window 260..740 x 190..490 units
(2.4 px per unit). For each start: rows = rules, columns = dry x1, dry x3,
damp x1; a working-size sheet (the painted area at half size) and a detail
sheet (native pixels around the wipe's middle, shown at 2x, no resampling
blur).
"""
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
dst.mkdir(parents=True, exist_ok=True)
PX = 2.4
X0, Y0 = 260.0, 190.0
RULES = [("Floor", "today: 1 µm floor"), ("Zero", "B: no floor"), ("Fixed", "C: 0.5 µm tooth"), ("Soft", "D: slowing")]
WIPES = [("dry1", "dry, 1 wipe"), ("dry3", "dry, 3 wipes"), ("damp1", "damp, 1 wipe")]
STARTS = [("film1", "direct 1 µm film"), ("film3", "direct 3 µm film"), ("wash05", "brush wash, thinner 0.5")]


def box(u0, v0, u1, v1):
    return tuple(round(t) for t in ((u0 - X0) * PX, (v0 - Y0) * PX, (u1 - X0) * PX, (v1 - Y0) * PX))


WORK = box(280, 245, 720, 435)
DETAIL = box(500 - 62.5, 340 - 42, 500 + 62.5, 340 + 42)
try:
    font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 22)
except OSError:
    font = ImageFont.load_default()


def cell(path, kind):
    im = Image.open(path).convert("RGB")
    if kind == "work":
        im = im.crop(WORK)
        return im.resize((im.width // 2, im.height // 2), Image.LANCZOS)
    im = im.crop(DETAIL)
    return im.resize((im.width * 2, im.height * 2), Image.NEAREST)


for sname, slabel in STARTS:
    for kind in ("work", "detail"):
        start = cell(src / f"{sname}-start.png", kind)
        w, h = start.size
        lab, gap, top = 210, 8, 40
        W = lab + 3 * (w + gap)
        H = top + (h + gap) + top + 4 * (h + gap)
        sheet = Image.new("RGB", (W, H), (32, 32, 32))
        d = ImageDraw.Draw(sheet)
        d.text((8, 8), f"{slabel}: start ({'working size, half' if kind == 'work' else 'detail, native px at 2x'})", fill=(235, 235, 235), font=font)
        sheet.paste(start, (lab, top))
        y = top + h + gap + top
        for j, (_, wl) in enumerate(WIPES):
            d.text((lab + j * (w + gap) + 6, y - 30), wl, fill=(235, 235, 235), font=font)
        for rule, rl in RULES:
            d.text((8, y + h // 2 - 12), rl, fill=(235, 235, 235), font=font)
            for j, (wn, _) in enumerate(WIPES):
                sheet.paste(cell(src / f"{sname}-{rule}-{wn}.png", kind), (lab + j * (w + gap), y))
            y += h + gap
        sheet.save(dst / f"{sname}-{kind}.jpg", quality=92)
print("sheets in", dst)
