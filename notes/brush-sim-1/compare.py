#!/usr/bin/env python3
import csv
import hashlib
import statistics
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

root = Path(sys.argv[1])
variants = ["baseline", "lean", "body", "wild"]
scenes = ["loaded", "spent", "curve", "crossing", "scumble", "passage"]
font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 17)
small = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 14)
rows = list(csv.DictReader((root / "metrics.csv").open()))
table = []
for scene in scenes:
    base = next(r for r in rows if r["scene"] == scene and r["variant"] == "baseline")
    baseline_time = statistics.median(float(r["time_per_chunk_s"]) for r in rows if r["scene"] == scene and r["variant"] == "baseline")
    for variant in variants:
        rs = [r for r in rows if r["scene"] == scene and r["variant"] == variant]
        r = rs[0]
        time = statistics.median(float(x["time_per_chunk_s"]) for x in rs)
        table.append(dict(scene=scene, variant=variant, paint_used=float(r["paint_used"]), paint_change_pct=100 * (float(r["paint_used"]) / float(base["paint_used"]) - 1), time_per_chunk_ms=1000 * time, time_change_pct=100 * (time / baseline_time - 1), height_mean_um=float(r["height_mean_um"]), height_p95_um=float(r["height_p95_um"]), height_p99_um=float(r["height_p99_um"]), height_max_um=float(r["height_max_um"]), conservation_error=float(r["conservation_error"])))
with (root / "summary.csv").open("w") as f:
    w = csv.DictWriter(f, fieldnames=list(table[0]))
    w.writeheader()
    w.writerows(table)

sheet = Image.new("RGB", (4 * 664 + 24, 100 + 6 * 202), "#f1eee7")
draw = ImageDraw.Draw(sheet)
draw.text((16, 12), "Brush simulation 1 | native 1:1 crops | 160 mm canvas, 0.25 mm/pixel", font=font, fill="#252525")
draw.text((16, 38), "Gallery light 115 / 55 / 1 beside color. Same gestures, pressure, brush seeds and total dip budgets.", font=small, fill="#444444")
for j, variant in enumerate(variants):
    x = 16 + j * 664
    draw.text((x, 64), f"{variant.title()}  |  " + {"baseline": "engine 5", "lean": "release 0.85", "body": "release 1.05", "wild": "release 0.95, entropy 2.8"}[variant], font=font, fill="#252525")
    draw.text((x, 86), "Gallery light" + " " * 54 + "Color", font=small, fill="#444444")
for i, scene in enumerate(scenes):
    y = 108 + i * 202
    crop = {"loaded": (160, 100, 480, 260), "spent": (160, 100, 480, 260), "curve": (160, 80, 480, 240), "crossing": (180, 80, 500, 240), "scumble": (160, 100, 480, 260), "passage": (160, 100, 480, 260)}[scene]
    for j, variant in enumerate(variants):
        x = 16 + j * 664
        r = next(r for r in table if r["scene"] == scene and r["variant"] == variant)
        draw.text((x, y), f"{scene.title()} | paint {r['paint_change_pct']:+.1f}% | peak {r['height_max_um']/1000:.3f} mm", font=small, fill="#252525")
        for k, view in enumerate(["gallery", "color"]):
            im = Image.open(root / f"{scene}-{variant}-{view}.png")
            assert im.size == (640, 320)
            sheet.paste(im.crop(crop), (x + k * 328, y + 24))
sheet.save(root / "alice-comparison.png")
for scene in scenes:
    full = Image.new("RGB", (1312, 4 * 358 + 60), "#f1eee7")
    d = ImageDraw.Draw(full)
    d.text((16, 12), f"{scene.title()} | native 640 x 320 | gallery light beside color", font=font, fill="#252525")
    for j, variant in enumerate(variants):
        y = 50 + j * 358
        d.text((16, y), variant.title(), font=font, fill="#252525")
        for k, view in enumerate(["gallery", "color"]):
            full.paste(Image.open(root / f"{scene}-{variant}-{view}.png"), (16 + k * 648, y + 28))
    full.save(root / f"{scene}-comparison.png")
with (root / "SHA256SUMS").open("w") as f:
    for p in sorted(root.iterdir()):
        if p.suffix in [".png", ".csv"]:
            f.write(f"{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n")
for variant in variants:
    rs = [r for r in table if r["variant"] == variant]
    total = sum(r["paint_used"] for r in rs)
    base = sum(r["paint_used"] for r in table if r["variant"] == "baseline")
    print(variant, "paint", total, "change", 100 * (total / base - 1), "median sum ms", sum(r["time_per_chunk_ms"] for r in rs), "max um", max(r["height_max_um"] for r in rs))
