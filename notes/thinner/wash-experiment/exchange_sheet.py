# /// script
# dependencies = ["pillow"]
# ///
"""Comparison sheets for the brush exchange probes (paint::exchange_probes::exchange_scenes).

    uv run notes/thinner/wash-experiment/exchange_sheet.py <png dir> <out.png> <ground> <set,set,...> [actions]

The PNGs are 1200-px whole-canvas renders (1.2 px per unit) named <ground>-<set>-<scene>.png.
Rows are settings, columns scenes. Default: the crop x 200-800, y 200-500 units of the
stroke, broad pass, 3 passes wet, 3 passes dried between and the depletion scene.
`actions`: the crop x 200-800, y 320-380 of the action scenes over one band.
"""
import sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

src, dst, ground, sets = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3], sys.argv[4].split(",")
actions = len(sys.argv) > 5 and sys.argv[5] == "actions"
PX = 1.2
if actions:
    area, k = (200, 320, 800, 380), 0.5
    scenes = [("scrub", "out and back, one call"), ("cont", "cut at the turn, no lift"), ("lift", "lift at the turn, no reload"),
              ("reload", "fresh reload onto wet film"), ("afterdry", "fresh reload after drying")]
else:
    area, k = (200, 200, 800, 500), 0.5
    scenes = [("stroke", "1 stroke"), ("broad", "broad pass (8)"), ("3wet", "3 passes wet"), ("3dry", "3 passes, dried between"),
              ("deplete", "1 load, 6 strokes")]
box = tuple(round(v * PX) for v in area)
font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 20)
small = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 18)
cw, ch = round((box[2] - box[0]) * k), round((box[3] - box[1]) * k)
lw, top, gap = 110, 32, 6
out = Image.new("RGB", (lw + len(scenes) * (cw + gap), top + len(sets) * (ch + gap)), (32, 32, 32))
d = ImageDraw.Draw(out)
for c, (_, title) in enumerate(scenes):
    d.text((lw + c * (cw + gap) + 4, 6), title, fill=(240, 240, 240), font=font)
names = {"today": "today", "b": "(b) ctl", "B": "(B) ctl"}
for r, s in enumerate(sets):
    y = top + r * (ch + gap)
    d.text((6, y + ch // 2 - 10), names.get(s, s), fill=(240, 240, 240), font=small)
    for c, (scene, _) in enumerate(scenes):
        f = src / f"{ground}-{s}-{scene}.png"
        if f.exists():
            im = Image.open(f).convert("RGB").crop(box).resize((cw, ch), Image.LANCZOS)
            out.paste(im, (lw + c * (cw + gap), y))
# (256 colors: these tones quantize without visible banding, a fifth of the size)
out.quantize(256, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE).save(dst, optimize=True)
print(dst, out.size)
