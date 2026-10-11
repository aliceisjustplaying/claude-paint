import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("round4", type=Path)
    parser.add_argument("round5", type=Path)
    parser.add_argument("round6", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    roots = [args.round4, args.round5, args.round6]
    font = ImageFont.truetype("DejaVuSans.ttf", 17)
    small = ImageFont.truetype("DejaVuSans.ttf", 13)
    rows = [
        "1  staining phthalo wash",
        "2  pale ultramarine wash",
        "3  dense ultramarine wash",
        "4  earth wash",
        "5  wet color join",
        "6  dry-brush control",
        "7  dense ultramarine on damp paper",
        "8  same wash on dry paper",
    ]
    names = {2: "3-dense-ultramarine", 4: "5-wet-join", 6: "7-damp-paper", 7: "8-dry-paper"}
    for mode in ["gallery", "color"]:
        sheet = Image.new("RGB", (1024, 978), "#eeeae3")
        draw = ImageDraw.Draw(sheet)
        draw.text((16, 12), f"Watercolor 6 / {mode.title()} / matched swatches", font=font, fill="#222")
        draw.text((16, 40), "16 x 24 mm / 0.05 mm cells / seed 71 / native 1:1 pixels", font=small, fill="#444")
        for x, label in zip([16, 352, 688], ["round 4", "fixed (round 5)", "edges (round 6)"]):
            draw.text((x, 66), label, font=font, fill="#222")
        for row, label in enumerate(rows):
            y = 99 + row * 108
            draw.text((16, y), label, font=small, fill="#333")
            trio = Image.new("RGB", (992, 80), "#eeeae3")
            for col, root in enumerate(roots):
                condition = "original" if row < 6 else ["damp", "dry"][row - 6]
                image = Image.open(root / condition / f"swatches-{mode}.png").convert("RGB")
                assert image.size == (320, 480)
                source_row = row if row < 6 else 2
                crop = image.crop((0, source_row * 80, 320, (source_row + 1) * 80))
                sheet.paste(crop, (16 + col * 336, y + 21))
                trio.paste(crop, (col * 336, 0))
            if mode == "gallery" and row in names:
                trio.save(args.output / f"{names[row]}-gallery-1to1.png")
        sheet.save(args.output / f"watercolor-6-swatches-{mode}.png")


if __name__ == "__main__":
    main()
