#!/usr/bin/env python3
import csv
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

root = Path(sys.argv[1])
font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
font = ImageFont.truetype(font_path, 19)
small = ImageFont.truetype(font_path, 15)
title = ImageFont.truetype(font_path, 27)
cases = [
    ("01-flat", "Flat wash / edge drying"),
    ("02-bloom", "Wet-in-wet / fresh-water backrun"),
    ("03-graded", "Graded wash"),
    ("04-granulation-staining", "Ultramarine-like | phthalo-like"),
    ("05-dry-brush", "Earth / dry contact on cold-press tooth"),
    ("06-glaze", "Earth below / ultramarine-like glaze above"),
    ("07-lifting", "Damp lifting / ultramarine-like | phthalo-like"),
]
sheet = Image.new("RGB", (1152, 2840), "#f4f1e9")
draw = ImageDraw.Draw(sheet)
draw.text((24, 18), "WATERCOLOR 1 / isolated material pilot", font=title, fill="#20251e")
draw.text((24, 57), "Diffuse gallery-like light", font=small, fill="#50554b")
draw.text((590, 57), "Raking light / paper relief", font=small, fill="#50554b")
for row, (name, label) in enumerate(cases):
    top = 91 + row * 391
    draw.text((24, top), f"{row + 1:02d}  {label}", font=font, fill="#20251e")
    for column, view in enumerate(["gallery", "raking"]):
        image = Image.open(root / f"{name}-{view}.ppm")
        image.save(root / f"{name}-{view}.png")
        image = image.resize((540, 336), Image.Resampling.NEAREST)
        sheet.paste(image, (24 + column * 566, top + 32))
sheet.save(root / "probe-sheet.png")

strip = Image.new("RGB", (1376, 330), "#f4f1e9")
d = ImageDraw.Draw(strip)
d.text((18, 10), "BLOOM / physical seconds after a fresh-water drop", font=title, fill="#20251e")
for i, time in enumerate([0, 0.25, 0.5, 1, 2, 4, 8]):
    x = 18 + i * 193
    d.text((x, 53), f"{time:g} s", font=font, fill="#20251e")
    for row, view in enumerate(["gallery", "raking"]):
        image = Image.open(root / f"bloom-{i}-{view}.ppm")
        image.save(root / f"bloom-{i}-{view}.png")
        strip.paste(image, (x, 81 + row * 119))
strip.save(root / "bloom-strip.png")

frames = []
for path in sorted((root / "slow-bloom").glob("*-gallery.ppm")):
    image = Image.open(path).resize((540, 336), Image.Resampling.NEAREST)
    image = image.quantize(colors=128)
    frames.append(image)
if frames:
    frames[0].save(root / "bloom-slow.gif", save_all=True, append_images=frames[1:],
                   duration=125, loop=0, optimize=False, disposal=2)

rows = list(csv.DictReader((root / "conservation.csv").open()))
report = ["# Conservation", "", "Water is in pixel-mm (multiply by 0.04 mm² for mm³). Pigment is arbitrary areal mass times pixels, not calibrated grams.",
          "", "Water columns: surface, capillary, no bound channel. Pigment columns: suspended, deposited, bound. Boundary outflow is zero.", "",
          "| Probe | Constituent | Input | Surface / suspended | Pore / deposited | Bound | Evaporated | Lifted | Error |",
          "|---|---|---:|---:|---:|---:|---:|---:|---:|"]
for row in rows:
    values = [row["probe"], row["reservoir"]]
    values += [f"{float(row[key]):.7g}" for key in ["input", "surface_or_suspended", "capillary_or_deposited", "bound", "evaporated", "lifted", "error"]]
    report.append("| " + " | ".join(values) + " |")
(root / "conservation.md").write_text("\n".join(report) + "\n")
print("Probe sheet, bloom strip, slow GIF and conservation table written")
