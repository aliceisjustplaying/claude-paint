#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["Pillow>=10"]
# ///
import argparse
import csv
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main():
    parser = argparse.ArgumentParser(description="Label the native knife_probe views without repainting.")
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    probes = [
        ("slab", "Laid slab | pressure 0.35 to 0.5, tilt 15"),
        ("edge", "Edge stroke | blade along path, tilt 80"),
        ("scrape", "Scrape-back | fresh orange over set blue, pressure 1"),
        ("veil", "Thin veil | smooth set blue, pressure 0.75 to 0.8"),
        ("wet-mix", "Wet into wet | yellow blade crosses fresh blue"),
        ("ridges", "Impasto ridges | light pressure, lift 0.35"),
    ]
    font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
    font = ImageFont.truetype(font_path, 17)
    header = ImageFont.truetype(font_path, 23)
    crop = (40, 70, 600, 245)
    width, height = crop[2] - crop[0], crop[3] - crop[1]
    models = ["baseline", "knife-1"]
    if (args.directory / "slab-knife-2-gallery.png").exists():
        models.append("knife-2")
    if (args.directory / "slab-knife-3-gallery.png").exists():
        models.append("knife-3")
    round_number = len(models) - 1
    for view, label in [("color", "Color"), ("gallery", "Gallery 115 / 55 / 1"), ("relief", "Relief 135 / 25 / 1")]:
        sheet = Image.new("RGB", (len(models) * (width + 16) + 16, 100 + len(probes) * (height + 40)), "#202020")
        draw = ImageDraw.Draw(sheet)
        draw.text((16, 10), f"Knife round {round_number}: {label}", font=header, fill="white")
        for col, model in enumerate(models):
            draw.text((16 + col * (width + 16), 53), model.capitalize(), font=font, fill="white")
        for row, (probe, label) in enumerate(probes):
            y = 90 + row * (height + 40)
            draw.text((16, y), label, font=font, fill="white")
            for col, model in enumerate(models):
                with Image.open(args.directory / f"{probe}-{model}-{view}.png") as image:
                    sheet.paste(image.crop(crop), (16 + col * (width + 16), y + 28))
        sheet.save(args.directory / f"sheet-{view}.png")
    with (args.directory / "volume.csv").open() as source:
        rows = list(csv.DictReader(source))
    print(f"{len(rows)} closed ledgers; max paint error = {max(abs(float(r['paint_error_mm3'])) for r in rows):.9f} mm^3")
    print(f"max solvent error = {max(abs(float(r['solvent_error_mm3'])) for r in rows):.9f} mm^3")


if __name__ == "__main__":
    main()
