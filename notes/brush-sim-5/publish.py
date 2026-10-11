#!/usr/bin/env python3
import argparse
import csv
import io
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from common import FILES, NATIVE, ROI, VARIANTS, native, scratch, sha256, verify

PREFIX = "iteration-5-"


def crop_spec(text):
    name, coordinates = text.split(":", 1)
    if not name.replace("-", "").isalnum() or name == "dark-wall":
        raise argparse.ArgumentTypeError("Use a unique alphanumeric crop label")
    try:
        box = tuple(int(value) for value in coordinates.split(","))
        x0, y0, x1, y1 = box
    except ValueError as error:
        raise argparse.ArgumentTypeError("Crop must be NAME:x0,y0,x1,y1") from error
    if not 0 <= x0 < x1 <= NATIVE[0] or not 0 <= y0 < y1 <= NATIVE[1]:
        raise argparse.ArgumentTypeError("Crop must lie inside the native final PNG")
    return name, box


def save_image(image, path):
    image.save(path)
    with Image.open(path) as saved:
        saved.load()
        if saved.mode != image.mode or saved.size != image.size or saved.tobytes() != image.tobytes():
            raise ValueError(f"PNG encoding changed pixels: {path}")


def build(root, destination, controls_only=False, extra_crops=()):
    receipt = json.loads((root / "receipt.json").read_text())
    names = ["baseline", "lean4"] + ([] if controls_only else list(VARIANTS))
    summaries = {name: verify(receipt["selected"][name]) for name in names}
    if any(destination.glob(f"{PREFIX}*")):
        raise ValueError("Iteration-five files already exist; publication never overwrites anything")
    staging = destination.parent / f".{destination.name}-iteration-5-staging"
    crops = {"dark-wall": ROI}
    for label, box in extra_crops:
        if label in crops:
            raise ValueError(f"Duplicate crop label: {label}")
        crops[label] = box
    staging.mkdir(parents=True, exist_ok=False)
    checks = []
    try:
        for name in names:
            directory = Path(receipt["selected"][name]["directory"])
            for filename in FILES:
                output = staging / f"{PREFIX}{name}-{filename}"
                shutil.copy2(directory / filename, output)
                if sha256(directory / filename) != sha256(output):
                    raise ValueError("Published source bytes changed")
            for view in ("gallery", "color"):
                image = native(directory / f"{view}.png")
                for label, box in crops.items():
                    crop = image.crop(box)
                    output = staging / f"{PREFIX}{name}-{label}-{view}.png"
                    save_image(crop, output)
                    checks.append({"artifact": output.name, "source": str(directory / f"{view}.png"),
                                   "crop": list(box), "native_pixel_bytes_equal": True})

        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 18)

        def sheet(sheet_names, views, crop_label, filename):
            box = crops[crop_label] if crop_label else (0, 0, *NATIVE)
            width, height = box[2] - box[0], box[3] - box[1]
            output = Image.new("RGB", (width * len(sheet_names), (height + 64) * len(views)), "#f1ece2")
            draw = ImageDraw.Draw(output)
            pasted = []
            for row, view in enumerate(views):
                for column, name in enumerate(sheet_names):
                    selected = receipt["selected"][name]
                    label = f"{name} | {view.title()} | factor {selected['factor']:.6g}"
                    sublabel = f"volume {selected['deposited_volume']:,.3f} | error {selected['percent_error']:+.5f}%"
                    x, y = column * width, row * (height + 64)
                    draw.text((x + 10, y + 7), label, font=font, fill="#202936")
                    draw.text((x + 10, y + 32), sublabel, font=font, fill="#202936")
                    path = staging / f"{PREFIX}{name}-{'%s-' % crop_label if crop_label else ''}{view}.png"
                    with Image.open(path) as image:
                        image.load()
                        output.paste(image, (x, y + 64))
                        region = (x, y + 64, x + width, y + 64 + height)
                        if output.crop(region).tobytes() != image.tobytes():
                            raise ValueError("Pasted sheet crop differs from native source pixels")
                        pasted.append((path, region))
            path = staging / f"{PREFIX}{filename}.png"
            save_image(output, path)
            with Image.open(path) as encoded:
                for source, region in pasted:
                    with Image.open(source) as image:
                        if encoded.crop(region).tobytes() != image.tobytes():
                            raise ValueError("Encoded sheet crop differs from native source pixels")
            checks.append({"artifact": path.name, "pasted_regions_verified_after_png_encoding": len(pasted), "native_pixel_bytes_equal": True})

        sheet(names, ["gallery"], None, "gallery-comparison")
        sheet(names, ["color"], None, "color-comparison")
        for label in crops:
            sheet(names, ["gallery", "color"], label, f"{label}-comparison")
        if not controls_only:
            sheet(["baseline", "lean4", "wild5"], ["gallery"], None, "higher-entropy-gallery-option")
            sheet(["baseline", "lean4", "wild5"], ["gallery", "color"], "dark-wall", "higher-entropy-dark-wall-option")
        buffer = io.StringIO()
        rows = [{"variant": name, "engine": receipt["selected"][name]["engine"],
                 "factor": receipt["selected"][name]["factor"], **summaries[name],
                 "candidate_invocations": receipt["selected"][name]["invocations"],
                 "current_harness_invocations": receipt["selected"][name].get("current_harness_invocations", 1),
                 "checkpoint_available": receipt["selected"][name]["checkpoint_available"]} for name in names]
        writer = csv.DictWriter(buffer, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
        (staging / f"{PREFIX}material-volume-summary.csv").write_text(buffer.getvalue())
        publication = {"variants": names, "controls_only_preview": controls_only, "no_resizing_or_image_filters": True,
                       "native": list(NATIVE), "crops_native_half_open": crops, "checks": checks,
                       "higher_entropy_option": None if controls_only else "wild5; visual judgment required, no quality claim",
                       "checkpoints_excluded_from_publication": True}
        (staging / f"{PREFIX}publication.json").write_text(json.dumps(publication, indent=2) + "\n")
        shutil.copy2(root / "receipt.json", staging / f"{PREFIX}receipt.json")
        if (root / "mesh-analysis.json").exists():
            shutil.copy2(root / "mesh-analysis.json", staging / f"{PREFIX}mesh-analysis.json")
        (staging / f"{PREFIX}README.md").write_text(
            "# Iteration-five native comparison\n\n"
            + ("Controls-only tool verification; new presets are absent.\n\n" if controls_only else "Matched baseline, lean4 and three new presets.\n\n")
            + "Gallery is the default display. Full studies are untouched native 1200 x 800 PNGs. Crops and sheets preserve native 1:1 RGB bytes, checked after PNG encoding. No image filters or resizing were used.\n\n"
            + "Deposited volume is whole final material_total minus the common primed initial, in coats times square canvas units. It is not physical mass. Matching tolerance is 0.03%. Chunk material and solvent scalars are in the original materials.csv files; the summary records whole-study deltas.\n\n"
            + "Baseline is an explicitly retained original-volume engine-5 control from iteration-four wind-down files. Its checkpoint is absent. Its original invocation command was not retained; the receipt labels this unknown. Lean4 is the one fresh engine-15 replay at factor 1.546, reused without repainting. New accepted candidates keep their one invocation's Canvas and paired views in scratch; no accepted-candidate rerun occurs. Checkpoints are excluded from this publication.\n\n"
            + "Mesh analysis, when present, is diagnostic numerical analysis of CSV fields. Geometry-only is an engine-18 relight of the same saved lean4 Canvas, not a brush replay. Spectral statistics do not modify display pixels. The higher-entropy wild5 option requires visual judgment in gallery light.\n")
        paths = sorted(staging.iterdir())
        if any(path.stat().st_size > 100_000_000 for path in paths):
            raise ValueError("Artifact exceeds the 100 MB Drive limit")
        manifest = "".join(f"{sha256(path)}  {path.name}\n" for path in paths)
        (staging / f"{PREFIX}SHA256SUMS").write_text(manifest)
        destination.mkdir(parents=True, exist_ok=True)
        for path in sorted(staging.iterdir()):
            target = destination / path.name
            with path.open("rb") as source, target.open("xb") as output:
                shutil.copyfileobj(source, output)
            if sha256(target) != sha256(path):
                raise ValueError("Staged publication changed during final copy")
    finally:
        shutil.rmtree(staging)
    return len(list(destination.glob(f"{PREFIX}*")))


def main():
    parser = argparse.ArgumentParser(description="Stage native iteration-five artifacts in scratch. Captain copies to Drive only after approval.")
    parser.add_argument("root", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("--controls-only", action="store_true", help="Explicit tool-verification preview; never claims new variants exist")
    parser.add_argument("--extra-crop", action="append", type=crop_spec, default=[], metavar="NAME:x0,y0,x1,y1")
    args = parser.parse_args()
    count = build(scratch(args.root), scratch(args.destination), args.controls_only, args.extra_crop)
    print(f"Verified {count} iteration-five artifacts in scratch; nothing copied to Drive")


if __name__ == "__main__":
    main()
