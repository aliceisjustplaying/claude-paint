#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["Pillow>=10"]
# ///
import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main():
    parser = argparse.ArgumentParser(description="Compare the two final knife_demo states.")
    parser.add_argument("directory", type=Path)
    parser.add_argument("--reference", choices=["baseline", "knife-2"], default="baseline")
    args = parser.parse_args()
    model = "knife-3" if (args.directory / "knife-3-gallery.png").exists() else "knife-2"
    models = [args.reference, model]
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 19)
    for view in ["gallery", "color"]:
        sheet = Image.new("RGB", (1248, 456), "#202020")
        draw = ImageDraw.Draw(sheet)
        for col, model in enumerate(models):
            x = 16 + col * 616
            draw.text((x, 12), f"{model}: {view}", font=font, fill="white")
            with Image.open(args.directory / f"{model}-{view}.png") as image:
                sheet.paste(image.resize((600, 400), Image.Resampling.LANCZOS), (x, 44))
        sheet.save(args.directory / f"comparison-{view}.png")
    for name, box in [("detail-scrape", (350, 250, 900, 500)), ("detail-ridge", (360, 530, 860, 720))]:
        width, height = box[2] - box[0], box[3] - box[1]
        sheet = Image.new("RGB", (2 * width + 48, height + 56), "#202020")
        draw = ImageDraw.Draw(sheet)
        for col, model in enumerate(models):
            x = 16 + col * (width + 16)
            draw.text((x, 12), f"{model}: Gallery 1:1", font=font, fill="white")
            with Image.open(args.directory / f"{model}-gallery.png") as image:
                sheet.paste(image.crop(box), (x, 44))
        sheet.save(args.directory / f"{name}.png")
    for file in args.directory.glob("*.png"):
        if file.stat().st_size >= 20_000_000:
            raise RuntimeError(f"{file.name} is over the 20 MB limit")
    print("Full comparisons use 600 px panels; the two detail comparisons are unscaled native crops.")


if __name__ == "__main__":
    main()
