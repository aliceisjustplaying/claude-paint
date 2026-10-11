#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow>=11,<13"]
# ///
"""Platform-aware PNG comparison: exact unless known platforms differ.

Cross-platform RGB differences must be <=2 levels/channel in strictly <0.1%
of pixels. Dimensions and alpha must match exactly. Unknown origins and
same-platform comparisons require identical bytes. Exit 0 for a match,
1 for drift, 2 for invalid input.
"""
import argparse
import platform
import sys
from pathlib import Path

from PIL import Image, ImageChops

PLATFORMS = ("macos-aarch64", "macos-x86_64", "linux-aarch64", "linux-x86_64", "windows-aarch64", "windows-x86_64")


def current_platform():
    os = {"Darwin": "macos", "Linux": "linux", "Windows": "windows"}.get(platform.system())
    arch = {"arm64": "aarch64", "aarch64": "aarch64", "AMD64": "x86_64", "x86_64": "x86_64"}.get(platform.machine())
    return f"{os}-{arch}" if os and arch else "unknown"


def compare(expected, actual, expected_platform="unknown", actual_platform="unknown"):
    if expected_platform == "current":
        expected_platform = current_platform()
    if actual_platform == "current":
        actual_platform = current_platform()
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
        if expected_platform not in PLATFORMS or actual_platform not in PLATFORMS or expected_platform == actual_platform:
            ok = Path(expected).read_bytes() == Path(actual).read_bytes()
            return ok, f"byte-exact comparison ({expected_platform} -> {actual_platform}); {changed}/{pixels} pixels changed; max RGB delta {maximum}/255"
        ok = maximum <= 2 and changed * 1_000 < pixels
        return ok, f"cross-platform ({expected_platform} -> {actual_platform}); {changed}/{pixels} pixels changed; max RGB delta {maximum}/255; allowed {(pixels - 1) // 1_000} pixels, max 2/255"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("expected", nargs="?")
    parser.add_argument("actual", nargs="?")
    parser.add_argument("--print-platform", action="store_true")
    platforms = ["unknown", "current", *PLATFORMS]
    parser.add_argument("--expected-platform", choices=platforms, default="unknown")
    parser.add_argument("--actual-platform", choices=platforms, default="unknown")
    args = parser.parse_args()
    if args.print_platform:
        print(current_platform())
        return 0
    if args.expected is None or args.actual is None:
        parser.error("expected and actual images are required")
    try:
        ok, detail = compare(args.expected, args.actual, args.expected_platform, args.actual_platform)
    except (OSError, ValueError) as error:
        print(f"image comparison: {error}", file=sys.stderr)
        return 2
    print(f"image comparison: {'PASS' if ok else 'FAIL'}: {detail}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
