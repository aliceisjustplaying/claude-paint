import argparse
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageStat


def pair(a, b, output, width, crop):
    with Image.open(a) as left, Image.open(b) as right:
        left, right = left.convert("RGB"), right.convert("RGB")
        if left.size != right.size:
            raise ValueError("comparison dimensions differ")
        diff = ImageChops.difference(left, right)
        stats = {"size": left.size, "mean_abs_rgb": ImageStat.Stat(diff).mean,
                 "max_abs_rgb": [extreme[1] for extreme in diff.getextrema()]}
        if crop is not None:
            x, y, size = crop
            left = left.crop((x, y, x + size, y + size))
            right = right.crop((x, y, x + size, y + size))
            stats["native_crop_xy_size"] = crop
        else:
            height = round(left.height * width / left.width)
            left = left.resize((width, height), Image.Resampling.LANCZOS)
            right = right.resize((width, height), Image.Resampling.LANCZOS)
        canvas = Image.new("RGB", (left.width * 2 + 12, left.height + 30), "#eeeae3")
        draw = ImageDraw.Draw(canvas)
        draw.text((8, 8), "Engine 5 | Gallery 115, 55, 1", fill="black")
        draw.text((left.width + 20, 8), "Engine 7 | Gallery 115, 55, 1", fill="black")
        canvas.paste(left, (0, 30))
        canvas.paste(right, (left.width + 12, 30))
        canvas.save(output)
        Path(output).with_suffix(".json").write_text(json.dumps(stats, indent=2) + "\n")
        print(json.dumps(stats))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Gallery pairs; numerical differences are not visual acceptance")
    parser.add_argument("engine5")
    parser.add_argument("engine7")
    parser.add_argument("output")
    parser.add_argument("--width", type=int, default=720)
    parser.add_argument("--crop", type=int, nargs=3, metavar=("X", "Y", "SIZE"))
    args = parser.parse_args()
    pair(args.engine5, args.engine7, args.output, args.width, args.crop)
