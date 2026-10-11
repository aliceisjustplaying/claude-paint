import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("current", type=Path)
    parser.add_argument("fixed", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    font = ImageFont.truetype("DejaVuSans.ttf", 17)
    small = ImageFont.truetype("DejaVuSans.ttf", 13)
    rows = [
        "1  staining phthalo wash",
        "2  pale ultramarine wash",
        "3  dense ultramarine wash",
        "4  earth wash",
        "5  wet color join",
        "6  dry-brush control",
    ]
    names = ["phthalo", "pale-ultramarine", "dense-ultramarine", "earth", "wet-join", "dry-brush"]
    for mode in ["gallery", "color"]:
        images = [Image.open(root / f"swatches-{mode}.png").convert("RGB")
                  for root in [args.current, args.fixed]]
        assert all(im.size == (320, 480) for im in images)
        sheet = Image.new("RGB", (688, 762), "#eeeae3")
        draw = ImageDraw.Draw(sheet)
        draw.text((16, 12), f"Watercolor 5 / {mode.title()} / matched swatches", font=font, fill="#222")
        draw.text((16, 40), "16 x 24 mm / 0.05 mm cells / seed 71 / native 1:1 pixels", font=small, fill="#444")
        for x, label in zip([16, 352], ["current (round 4)", "fixed"]):
            draw.text((x, 66), label, font=font, fill="#222")
        for row, (label, name) in enumerate(zip(rows, names)):
            y = 99 + row * 108
            draw.text((16, y), label, font=small, fill="#333")
            pair = Image.new("RGB", (656, 80), "#eeeae3")
            for col, image in enumerate(images):
                crop = image.crop((0, row * 80, 320, (row + 1) * 80))
                sheet.paste(crop, (16 + col * 336, y + 21))
                pair.paste(crop, (col * 336, 0))
            if mode == "gallery":
                pair.save(args.output / f"{row + 1}-{name}-{mode}-1to1.png")
        sheet.save(args.output / f"watercolor-5-swatches-{mode}.png")


if __name__ == "__main__":
    main()
