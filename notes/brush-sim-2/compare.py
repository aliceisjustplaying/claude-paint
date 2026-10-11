#!/usr/bin/env python3
import csv
import hashlib
import statistics
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

root = Path(sys.argv[1])
variants = ["baseline", "iteration1", "lean2", "iteration2", "wild2"]
scenes = ["crossing", "dark", "wet", "spent"]
font_path = next((p for p in ["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "/System/Library/Fonts/Helvetica.ttc"] if Path(p).exists()), None)
font = ImageFont.truetype(font_path, 17) if font_path else ImageFont.load_default()
small = ImageFont.truetype(font_path, 14) if font_path else font
rows = list(csv.DictReader((root / "metrics.csv").open()))
summary = []
for scene in scenes:
    baseline = next(r for r in rows if r["scene"] == scene and r["variant"] == "baseline")
    baseline_time = statistics.median(float(r["time_s"]) for r in rows if r["scene"] == scene and r["variant"] == "baseline")
    for variant in variants:
        rs = [r for r in rows if r["scene"] == scene and r["variant"] == variant]
        r = rs[0]
        for key in ["paint_used", "input_paint", "remaining_paint", "height_mean_um", "height_p95_um", "height_p99_um", "height_max_um"]:
            assert len({x[key] for x in rs}) == 1, (scene, variant, key)
        time = statistics.median(float(x["time_s"]) for x in rs)
        summary.append(dict(scene=scene, variant=variant, paint_used=float(r["paint_used"]), paint_change_pct=100 * (float(r["paint_used"]) / float(baseline["paint_used"]) - 1), time_ms=time * 1000, time_change_pct=100 * (time / baseline_time - 1), height_mean_um=float(r["height_mean_um"]), height_p95_um=float(r["height_p95_um"]), height_p99_um=float(r["height_p99_um"]), height_max_um=float(r["height_max_um"]), conservation_error=float(r["conservation_error"])))
with (root / "summary.csv").open("w") as f:
    writer = csv.DictWriter(f, fieldnames=list(summary[0]))
    writer.writeheader()
    writer.writerows(summary)

sheet = Image.new("RGB", (2016, 960), "#f1eee7")
d = ImageDraw.Draw(sheet)
d.text((16, 12), "Brush simulation 2 | native 1:1 crops | gallery light 115 / 55 / 1 beside color", font=font, fill="#252525")
matched = any(float(r["load_scale"]) != 1.0 for r in rows)
d.text((16, 38), ("Matched deposited mass; load factors are in metrics.csv." if matched else "Same gestures, seeds and dip budgets.") + " Body is iteration 1; body2 is iteration 2.", font=small, fill="#444444")
for j, variant in enumerate(["baseline", "iteration1", "iteration2"]):
    x = 16 + j * 664
    d.text((x, 64), {"baseline": "Baseline / engine 5", "iteration1": "Iteration 1 / body / engine 7", "iteration2": "Iteration 2 / body2 / engine 10"}[variant], font=font, fill="#252525")
    d.text((x, 90), "Gallery light                                           Color", font=small, fill="#444444")
    for i, scene in enumerate(scenes):
        y = 118 + i * 206
        r = next(r for r in summary if r["scene"] == scene and r["variant"] == variant)
        d.text((x, y), f"{scene.title()} | paint {r['paint_change_pct']:+.1f}% | peak {r['height_max_um'] / 1000:.3f} mm", font=small, fill="#252525")
        for k, view in enumerate(["gallery", "color"]):
            im = Image.open(root / f"{scene}-{variant}-{view}.png")
            assert im.size == (640, 320)
            crop = (180, 80, 500, 260) if scene == "crossing" else (160, 100, 480, 280)
            sheet.paste(im.crop(crop), (x + k * 328, y + 24))
sheet.save(root / "alice-comparison.png")
for scene in scenes:
    sheet = Image.new("RGB", (1328, 1880), "#f1eee7")
    d = ImageDraw.Draw(sheet)
    d.text((16, 12), f"{scene.title()} | native 640 x 320 | gallery light beside color", font=font, fill="#252525")
    for j, variant in enumerate(variants):
        y = 50 + j * 364
        d.text((16, y), variant, font=font, fill="#252525")
        for k, view in enumerate(["gallery", "color"]):
            sheet.paste(Image.open(root / f"{scene}-{variant}-{view}.png"), (16 + k * 648, y + 28))
    sheet.save(root / f"{scene}-comparison.png")

sheet = Image.new("RGB", (3280, 1490), "#f1eee7")
d = ImageDraw.Draw(sheet)
d.text((16, 12), "Higher-entropy controls | gallery light | native pixels", font=font, fill="#252525")
for j, variant in enumerate(variants):
    x = 16 + j * 652
    for i, scene in enumerate(scenes):
        y = 48 + i * 358
        d.text((x, y), f"{variant} / {scene}", font=font, fill="#252525")
        sheet.paste(Image.open(root / f"{scene}-{variant}-gallery.png"), (x, y + 26))
sheet.save(root / "entropy-gallery.png")
with (root / "SHA256SUMS").open("w") as f:
    for p in sorted(root.iterdir()):
        if p.suffix in [".png", ".csv"]:
            f.write(f"{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n")
for variant in variants:
    rs = [r for r in summary if r["variant"] == variant]
    total = sum(r["paint_used"] for r in rs)
    base = sum(r["paint_used"] for r in summary if r["variant"] == "baseline")
    print(variant, "paint", total, "change", 100 * (total / base - 1), "sum median ms", sum(r["time_ms"] for r in rs))
