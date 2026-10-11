#!/usr/bin/env python3
import argparse
import csv
import hashlib
import math
import statistics
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


VARIANTS = ["baseline", "body2", "lean3", "body3", "wild3"]
SCENES = ["crossing", "spent", "dark", "wet", "thin-dark", "impasto-light", "lit-window"]
LABELS = {"baseline": "Baseline / engine 5", "body2": "Body2 / engine 10", "lean3": "Lean3 / engine 12", "body3": "Body3 / engine 13", "wild3": "Wild3 / engine 14"}
FONT_PATH = next((p for p in ["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "/System/Library/Fonts/Helvetica.ttc"] if Path(p).exists()), None)
FONT = ImageFont.truetype(FONT_PATH, 17) if FONT_PATH else ImageFont.load_default()
SMALL = ImageFont.truetype(FONT_PATH, 14) if FONT_PATH else FONT
PAPER = "#f1eee7"
INK = "#252525"


def save(image, path):
    if path.exists():
        raise FileExistsError(f"Refusing to replace {path}")
    image.save(path)
    if path.stat().st_size >= 20_000_000:
        path.unlink()
        raise ValueError(f"{path.name} exceeds the 20 MB delivery limit")


def native(path, size):
    with Image.open(path) as source:
        assert source.size == size, (path, source.size, size)
        return source.convert("RGB")


def checksum(root, paths):
    with (root / "comparison-SHA256SUMS").open("x") as f:
        for path in sorted(paths):
            f.write(f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n")


def fixtures(root):
    rows = list(csv.DictReader((root / "metrics.csv").open()))
    values = list(csv.DictReader((root / "luminance-thickness.csv").open()))
    assert rows, "No fixture metrics"
    modes = {r["mode"] for r in rows}
    assert len(modes) == 1, modes
    matched = modes == {"matched-total-canvas-net-paint"}
    scenes = [s for s in SCENES if any(r["scene"] == s for r in rows)]
    variants = [v for v in VARIANTS if any(r["variant"] == v for r in rows)]
    summary = []
    for scene in scenes:
        for variant in variants:
            runs = [r for r in rows if r["scene"] == scene and r["variant"] == variant]
            assert runs, (scene, variant)
            first = runs[0]
            for key in first:
                if key not in ["round", "time_s"]:
                    assert len({r[key] for r in runs}) == 1, (scene, variant, key)
            for key, value in first.items():
                if key not in ["scene", "variant", "mode"]:
                    assert math.isfinite(float(value)), (scene, variant, key)
            row = dict(first)
            row.pop("round")
            row["time_s"] = statistics.median(float(r["time_s"]) for r in runs)
            row["max_abs_paint_ledger_error"] = max(abs(float(r["paint_ledger_error"])) for r in runs)
            row["max_abs_solvent_ledger_error"] = max(abs(float(r["solvent_ledger_error"])) for r in runs)
            summary.append(row)
    with (root / "summary.csv").open("x") as f:
        writer = csv.DictWriter(f, fieldnames=list(summary[0]))
        writer.writeheader()
        writer.writerows(summary)
    primary = [v for v in ["baseline", "body2", "body3"] if v in variants]
    sheet = Image.new("RGB", (16 + 664 * len(primary), 112 + 218 * len(scenes)), PAPER)
    draw = ImageDraw.Draw(sheet)
    draw.text((16, 10), "Brush simulation 3 | native 1:1 crops | Gallery 115 / 55 / 1 beside Color", font=FONT, fill=INK)
    draw.text((16, 36), "Matched TOTAL final canvas net paint; factors and achieved errors retained." if matched else "Equal requested loads, identical gestures and seeds; actual input and residuals retained.", font=SMALL, fill=INK)
    crops = {"crossing": (180, 80, 500, 260), "lit-window": (160, 60, 480, 240), "impasto-light": (160, 80, 480, 260)}
    for column, variant in enumerate(primary):
        x = 16 + column * 664
        draw.text((x, 64), LABELS[variant], font=FONT, fill=INK)
        draw.text((x, 88), "Gallery light                                           Color", font=SMALL, fill=INK)
        for i, scene in enumerate(scenes):
            y = 112 + i * 218
            roi = "light_core" if scene in ["lit-window", "impasto-light"] else "core"
            value = next(r for r in values if r["scene"] == scene and r["variant"] == variant and r["view"] == "color" and r["roi"] == roi)
            draw.text((x, y), f"{scene} | Color Y50 {float(value['y_median']):.3f} | wet {float(value['wet_mean_um']):.1f} um | grad {float(value['gradient_mean_y_per_px']):.3f}", font=SMALL, fill=INK)
            for k, view in enumerate(["gallery", "color"]):
                image = native(root / f"{scene}-{variant}-{view}.png", (640, 320))
                sheet.paste(image.crop(crops.get(scene, (160, 100, 480, 280))), (x + k * 328, y + 26))
    outputs = [root / "alice-comparison.png"]
    save(sheet, outputs[-1])
    for scene in scenes:
        sheet = Image.new("RGB", (1328, 52 + 366 * len(variants)), PAPER)
        draw = ImageDraw.Draw(sheet)
        draw.text((16, 12), f"{scene} | native 640 x 320 | Gallery beside Color from the SAME final state", font=FONT, fill=INK)
        for j, variant in enumerate(variants):
            y = 52 + j * 366
            draw.text((16, y), LABELS[variant], font=FONT, fill=INK)
            for k, view in enumerate(["gallery", "color"]):
                sheet.paste(native(root / f"{scene}-{variant}-{view}.png", (640, 320)), (16 + k * 648, y + 28))
        outputs.append(root / f"{scene}-full-comparison.png")
        save(sheet, outputs[-1])
    controls = [v for v in ["baseline", "body3", "lean3", "wild3"] if v in variants]
    sheet = Image.new("RGB", (16 + 652 * len(controls), 48 + 358 * len(scenes)), PAPER)
    draw = ImageDraw.Draw(sheet)
    draw.text((16, 12), "Higher-entropy controls | native Gallery pixels | no filters", font=FONT, fill=INK)
    for j, variant in enumerate(controls):
        for i, scene in enumerate(scenes):
            x, y = 16 + j * 652, 48 + i * 358
            draw.text((x, y), f"{variant} / {scene}", font=FONT, fill=INK)
            sheet.paste(native(root / f"{scene}-{variant}-gallery.png", (640, 320)), (x, y + 26))
    outputs.append(root / "entropy-gallery.png")
    save(sheet, outputs[-1])
    paths = outputs + [root / "summary.csv"] + list(root.glob("*-color.png")) + list(root.glob("*-gallery.png")) + [root / "metrics.csv", root / "luminance-thickness.csv", root / "transport.csv", root / "scope.txt"] + list(root.glob("edge-profile.csv"))
    checksum(root, paths)
    for row in summary:
        print(row["scene"], row["variant"], "factor", row["load_factor"], "net", row["net_paint"], "match %", row["match_error_pct"], "paint error", row["paint_ledger_error"], "solvent error", row["solvent_ledger_error"])


def morning(root, experimental):
    outputs = []
    for view in ["gallery", "color"]:
        sheet = Image.new("RGB", (2432, 852), PAPER)
        draw = ImageDraw.Draw(sheet)
        for column, variant in enumerate(["baseline", experimental]):
            x = 16 + column * 1216
            draw.text((x, 12), f"Morning / {variant} | {view.title()} | native 1200 x 800", font=FONT, fill=INK)
            sheet.paste(native(root / f"morning-{variant}-{view}.png", (1200, 800)), (x, 44))
        outputs.append(root / f"morning-{view}-comparison.png")
        save(sheet, outputs[-1])
    sheet = Image.new("RGB", (2432, 1704), PAPER)
    draw = ImageDraw.Draw(sheet)
    for row, variant in enumerate(["baseline", experimental]):
        y = row * 852
        gallery = native(root / f"morning-{variant}-gallery.png", (1200, 800))
        color = native(root / f"morning-{variant}-color.png", (1200, 800))
        draw.text((16, y + 12), f"Morning / {variant} | Gallery 115 / 55 / 1", font=FONT, fill=INK)
        draw.text((1232, y + 12), "Color | same final wet canvas", font=FONT, fill=INK)
        sheet.paste(gallery, (16, y + 44))
        sheet.paste(color, (1232, y + 44))
    outputs.append(root / "morning-comparison.png")
    save(sheet, outputs[-1])
    for detail, crop in [("window", (696, 132, 984, 444)), ("vessel", (240, 444, 528, 756))]:
        sheet = Image.new("RGB", (16 + 304 * 4, 376), PAPER)
        draw = ImageDraw.Draw(sheet)
        draw.text((16, 10), f"{detail.title()} | native 1:1 detail | baseline then {experimental}", font=FONT, fill=INK)
        for j, (variant, view) in enumerate((v, w) for v in ["baseline", experimental] for w in ["gallery", "color"]):
            x = 16 + j * 304
            draw.text((x, 36), f"{variant} / {view}", font=SMALL, fill=INK)
            sheet.paste(native(root / f"morning-{variant}-{view}.png", (1200, 800)).crop(crop), (x, 60))
        outputs.append(root / f"morning-{detail}-detail.png")
        save(sheet, outputs[-1])
    checksum(root, outputs + list(root.glob("morning-*-gallery.png")) + list(root.glob("morning-*-color.png")))


def main():
    parser = argparse.ArgumentParser(description="Native brush-sim-3 comparison sheets; never replays or filters a painting.")
    parser.add_argument("root", type=Path)
    parser.add_argument("--morning", metavar="EXPERIMENTAL", help="Read morning-baseline and morning-EXPERIMENTAL Color/Gallery PNG pairs")
    args = parser.parse_args()
    if args.morning:
        if args.morning == "baseline":
            parser.error("The experimental variant must differ from baseline")
        morning(args.root, args.morning)
    else:
        fixtures(args.root)


if __name__ == "__main__":
    main()
