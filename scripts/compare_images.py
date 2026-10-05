#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow>=11,<13"]
# ///
"""Cross-build RGB comparison: <=1 level/channel in <=0.01% of pixels.

Dimensions and alpha must match exactly. Same-build replay checks use exact
comparison instead. Exit 0 for a match, 1 for drift, 2 for invalid input.
"""
import argparse
import sys

from PIL import Image, ImageChops


def compare(expected, actual):
    with Image.open(expected) as a, Image.open(actual) as b:
        if a.mode not in ("RGB", "RGBA") or b.mode not in ("RGB", "RGBA"):
            raise ValueError("expected 8-bit RGB or RGBA images")
        if a.size != b.size:
            return False, f"dimensions differ: {a.size} != {b.size}"
        a, b = a.convert("RGBA"), b.convert("RGBA")
        delta = ImageChops.difference(a, b)
        red, green, blue, alpha = delta.split()
        if alpha.getextrema()[1] != 0:
            return False, "alpha differs"
        rgb_delta = ImageChops.lighter(ImageChops.lighter(red, green), blue)
        histogram = rgb_delta.histogram()
        pixels = a.width * a.height
        changed = pixels - histogram[0]
        maximum = rgb_delta.getextrema()[1]
        # Integer arithmetic: never round the allowed count up for small images.
        ok = maximum <= 1 and changed * 10_000 <= pixels
        return ok, f"{changed}/{pixels} pixels changed; max RGB delta {maximum}/255; allowed {pixels // 10_000} pixels, max 1/255"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("expected")
    parser.add_argument("actual")
    args = parser.parse_args()
    try:
        ok, detail = compare(args.expected, args.actual)
    except (OSError, ValueError) as error:
        print(f"image comparison: {error}", file=sys.stderr)
        return 2
    print(f"image comparison: {'PASS' if ok else 'FAIL'}: {detail}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
