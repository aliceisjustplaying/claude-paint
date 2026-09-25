"""Side by side, labeled, 1:1 crops of two renders of the same window.

usage: sbs.py a.png b.png x0 y0 x1 y1 out.png "label a" "label b" [title]
(x0..y1 in canvas units: the canvas is 1000 units wide)
"""
import sys
from PIL import Image, ImageDraw, ImageFont

a, b = Image.open(sys.argv[1]).convert("RGB"), Image.open(sys.argv[2]).convert("RGB")
x0, y0, x1, y1 = map(float, sys.argv[3:7])
out, la, lb = sys.argv[7], sys.argv[8], sys.argv[9]
title = sys.argv[10] if len(sys.argv) > 10 else None
assert a.size == b.size, (a.size, b.size)
s = a.width / 1000.0
box = tuple(round(v * s) for v in (x0, y0, x1, y1))
ca, cb = a.crop(box), b.crop(box)
w, h = ca.size
try:
    font = ImageFont.load_default(size=16)
except TypeError:
    font = ImageFont.load_default()
gap, bar = 12, 26
top = bar + (bar if title else 0)
img = Image.new("RGB", (2 * w + gap, h + top), (255, 255, 255))
d = ImageDraw.Draw(img)
if title:
    d.text((4, 4), title, fill=(0, 0, 0), font=font)
d.text((4, top - bar + 4), la, fill=(0, 0, 0), font=font)
d.text((w + gap + 4, top - bar + 4), lb, fill=(0, 0, 0), font=font)
img.paste(ca, (0, top))
img.paste(cb, (w + gap, top))
img.save(out, optimize=True)
print(out, img.size, "crop px", box)
