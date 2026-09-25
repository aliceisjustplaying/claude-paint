"""Side by side, labeled crops of two renders of the same window: 1:1, and
optionally enlarged (nearest neighbor, so pixels stay pixels) below.

usage: sbs.py a.png b.png x0 y0 x1 y1 out.png "label a" "label b" "title" [zoom]
(x0..y1 in canvas units: the canvas is 1000 units wide)
"""
import sys
from PIL import Image, ImageDraw, ImageFont

a, b = Image.open(sys.argv[1]).convert("RGB"), Image.open(sys.argv[2]).convert("RGB")
x0, y0, x1, y1 = map(float, sys.argv[3:7])
out, la, lb, title = sys.argv[7:11]
zoom = int(sys.argv[11]) if len(sys.argv) > 11 else 0
assert a.size == b.size, (a.size, b.size)
s = a.width / 1000.0
box = tuple(round(v * s) for v in (x0, y0, x1, y1))
ca, cb = a.crop(box), b.crop(box)
w, h = ca.size
font = ImageFont.load_default(size=16)
gap, bar = 12, 26
rows = [(ca, cb, "1:1")]
if zoom:
    rows.append((ca.resize((w * zoom, h * zoom), Image.NEAREST), cb.resize((w * zoom, h * zoom), Image.NEAREST), f"{zoom}x, nearest neighbor"))
# a column is at least as wide as its label
col = lambda r: max(r[0].width, 280)
W = max(2 * col(r) + gap for r in rows)
H = bar + sum(bar + r[0].height + gap for r in rows)
img = Image.new("RGB", (W, H), (255, 255, 255))
d = ImageDraw.Draw(img)
d.text((4, 4), title, fill=(0, 0, 0), font=font)
y = bar
for ra, rb, what in rows:
    d.text((4, y + 4), f"{la}  ({what})", fill=(0, 0, 0), font=font)
    x = col((ra, rb, what)) + gap
    d.text((x + 4, y + 4), f"{lb}  ({what})", fill=(0, 0, 0), font=font)
    img.paste(ra, (0, y + bar))
    img.paste(rb, (x, y + bar))
    y += bar + ra.height + gap
img.save(out, optimize=True)
print(out, img.size, "crop px", box)
