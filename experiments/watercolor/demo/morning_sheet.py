import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

root = Path(sys.argv[1])
prefix = sys.argv[2] if len(sys.argv) > 2 else ""
font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 20)
small = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 16)
frames = [Image.open(root / f"{prefix}bloom-start.png").resize((480, 320), Image.Resampling.LANCZOS)]
paths = sorted(root.glob(f"{prefix}bloom-????????.png"))
if len(paths) != 32:
    raise ValueError("expected 32 quarter-second bloom frames")
frames += [Image.open(paths[index]) for index in [0, 1, 3, 7, 15, 31]]
strip = Image.new("RGB", (1720, 300), "#f4f1e9")
draw = ImageDraw.Draw(strip)
draw.text((15, 12), "Public Lua API / clean-water backrun / physical seconds", font=font, fill="#20251e")
for i, (time, image) in enumerate(zip([0, 0.25, 0.5, 1, 2, 4, 8], frames)):
    x = 15 + i * 244
    draw.text((x, 49), f"{time:g} s", font=small, fill="#20251e")
    detail = image.crop((208, 113, 304, 201)).resize((230, 211), Image.Resampling.LANCZOS)
    strip.paste(detail, (x, 76))
strip.save(root / f"{prefix}bloom-strip.png")
movie = [frames[0]] + [Image.open(path) for path in paths]
movie[0].save(root / f"{prefix}bloom-slow.gif", save_all=True, append_images=movie[1:],
              duration=480, loop=0, disposal=2)
print("Morning bloom detail strip and slow loop written")
