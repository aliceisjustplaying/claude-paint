# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy", "pillow"]
# ///
import argparse
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

parser = argparse.ArgumentParser()
parser.add_argument("output", type=Path)
parser.add_argument("images", nargs="+", type=Path)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
images = [Image.open(p).convert("RGB") for p in args.images]
reference = np.asarray(images[0], dtype=np.int16)
metrics = []
for path, image in zip(args.images, images):
    delta = np.abs(np.asarray(image, dtype=np.int16) - reference)
    metrics.append({"path": str(path), "mean_absolute_rgb": delta.mean(axis=(0, 1)).tolist(), "max_channel": int(delta.max()), "changed_pixel_fraction": float(np.any(delta, axis=2).mean())})
width = 800
height = round(images[0].height * width / images[0].width)
sheet = Image.new("RGB", (width * 2, (height + 26) * ((len(images) + 1) // 2)), "#161412")
for i, image in enumerate(images):
    x, y = i % 2 * width, i // 2 * (height + 26)
    sheet.paste(image.resize((width, height), Image.Resampling.LANCZOS), (x, y + 26))
    ImageDraw.Draw(sheet).text((x + 10, y + 7), chr(65 + i), fill="white")
sheet.save(args.output / "full.png")
crop = Image.new("RGB", (840, 446 * ((len(images) + 1) // 2)), "#161412")
for i, image in enumerate(images):
    x, y = i % 2 * 420, i // 2 * 446
    crop.paste(image.crop((600, 600, 1020, 1020)), (x, y + 26))
    ImageDraw.Draw(crop).text((x + 10, y + 7), chr(65 + i), fill="white")
crop.save(args.output / "detail.png")
(args.output / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
print(json.dumps(metrics, indent=2))
