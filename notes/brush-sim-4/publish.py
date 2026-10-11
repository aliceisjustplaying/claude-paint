#!/usr/bin/env python3
import hashlib
import json
import shutil
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

root, destination = map(lambda p: Path(p).resolve(), sys.argv[1:3])
chosen = sys.argv[3] if len(sys.argv) > 3 else "wild4"
destination.mkdir(parents=True, exist_ok=True)
receipt = json.loads((root / "receipt.json").read_text())
font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 19)
prefix = "iteration-4-"
native = (1200, 800)
crop = (160, 180, 680, 460)
variants = list(receipt["selected"])
for name in variants:
    source = root / "matched" / name
    for view in ["gallery", "color"]:
        image = Image.open(source / f"{view}.png").convert("RGB")
        assert image.size == native
        shutil.copy2(source / f"{view}.png", destination / f"{prefix}{name}-{view}.png")
        image.crop(crop).save(destination / f"{prefix}{name}-dark-wall-{view}.png")
    for file in ["study.lua", "materials.csv", "digests.txt"]:
        shutil.copy2(source / file, destination / f"{prefix}{name}-{file}")
    for file in [f"{name}-metrics.csv", f"{name}-profile.csv"]:
        if (root / file).exists():
            shutil.copy2(root / file, destination / f"{prefix}{file}")


def sheet(names, views, detail, filename):
    width, height = (crop[2] - crop[0], crop[3] - crop[1]) if detail else native
    output = Image.new("RGB", (width * len(names), (height + 56) * len(views)), "#f1ece2")
    draw = ImageDraw.Draw(output)
    for row, view in enumerate(views):
        for column, name in enumerate(names):
            data = receipt["selected"][name]
            label = f"{name} | {view.title()} | volume {data['deposited_volume']:,.0f}"
            x, y = column * width, row * (height + 56)
            draw.text((x + 12, y + 17), label, font=font, fill="#202936")
            image = Image.open(destination / f"{prefix}{name}-{'dark-wall-' if detail else ''}{view}.png").convert("RGB")
            output.paste(image, (x, y + 56))
            assert image.tobytes() == output.crop((x, y + 56, x + width, y + 56 + height)).tobytes()
    output.save(destination / f"{prefix}{filename}.png")


sheet(["baseline", "body3", chosen], ["gallery"], False, "gallery-comparison")
if receipt.get("target_policy") == "baseline":
    shutil.copy2(destination / f"{prefix}gallery-comparison.png", destination / f"{prefix}original-volume-gallery.png")
sheet(["baseline", "body3", chosen], ["color"], False, "color-comparison")
sheet(variants, ["gallery", "color"], True, "dark-wall-comparison")
sheet(["lean4", "body4", "wild4"], ["gallery"], False, "entropy-gallery")
shutil.copy2(root / "receipt.json", destination / f"{prefix}receipt.json")
manifest = "".join(f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n" for path in sorted(destination.glob(f"{prefix}*")) if path.is_file() and not path.name.endswith("SHA256SUMS"))
(destination / f"{prefix}SHA256SUMS").write_text(manifest)
assert all(p.stat().st_size <= 100_000_000 for p in destination.glob(f"{prefix}*"))
print("Published untouched native pixels with iteration-4 labels")
