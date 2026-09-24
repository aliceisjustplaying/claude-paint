# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "pillow", "scipy"]
# ///
"""Glitch census: flag small digital-looking artifacts in a render.

    uv run scripts/glitch.py RENDER.png [--out OVERLAY.png] [--scale 3.2] [--json]

Three classes, all measured in OKLab against the pixel's own neighborhood
(notes/glitch.md explains the calibration):

- specks: isolated pixels or tiny blobs (area <= ~12 px at 3200) that differ
  from the local median by more than SPECK_DE while the ring around them is
  quiet (dots, pinholes, flecks of an alien color);
- lines: thin (<= ~2.5 px wide), long (>= ~8 px) high-contrast runs; those
  that lie along a value edge of the smoothed picture are "edge lines"
  (outlines, lining, pooled worms), the rest "free lines" (hairlines);
- flecks: patches up to ~80 px at 3200 of an alien hue (OKLab a, b off the
  neighborhood by FLECK_CHROMA): bare ground slivers, rings, stray color;
- mottle: 24x24 windows in quiet passages (the smoothed picture barely
  changes there) whose fine band-pass texture is strong (salt-and-pepper,
  crunchy stipple over the weave, "JPEG vibes").

The overlay puts the render on the left and, on the right, a dimmed gray
copy with specks in red, flecks in magenta, edge lines in cyan, free lines in blue and mottle
windows tinted yellow. Pixel sizes scale with --scale (render px per canvas
unit; 3.2 for a 3200 render of a 1000-unit canvas).
"""
import argparse
import json
import sys

import numpy as np
from PIL import Image
from scipy import ndimage as ndi

# Thresholds (OKLab units; 0.01 L is roughly a just-noticeable step in a flat field).
SPECK_DE = 0.05       # a speck differs from its neighborhood median by this much
SPECK_RING = 0.3      # ... and the ring around it varies at most this fraction of that
LINE_DE = 0.04        # a line pixel differs from the local median by this much
VISIBLE = 0.03       # a speck is "visible" if its ΔE, blurred as the eye blurs one
                      # pixel at 1:1 (gaussian σ 1 px at 3200), still peaks this high
FLECK_CHROMA = 0.035  # a fleck's hue/chroma (OKLab a, b) differs from its neighborhood by this much
MOTTLE_RMS = 0.004    # band-pass L rms in a quiet window above this is mottle
                      # (lossless 3200 crops: lab 1 sky A, "JPEG vibes", median 0.0051;
                      #  sky B 0.0022; l3_green 0.0025)
QUIET_GRAD = 0.0022   # smoothed L gradient per px below this is a quiet passage


def srgb_to_oklab(rgb):
    c = rgb.astype(np.float64) / 255.0
    lin = np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)
    m1 = np.array([[0.4122214708, 0.5363325363, 0.0514459929],
                   [0.2119034982, 0.6806995451, 0.1073969566],
                   [0.0883024619, 0.2817188376, 0.6299787005]])
    lms = np.cbrt(lin @ m1.T)
    m2 = np.array([[0.2104542553, 0.7936177850, -0.0040720468],
                   [1.9779984951, -2.4285922050, 0.4505937099],
                   [0.0259040371, 0.7827717662, -0.8086757660]])
    return lms @ m2.T


def census(rgb, scale=3.2):
    lab = srgb_to_oklab(rgb)
    h, w, _ = lab.shape
    k = max(1.0, scale / 3.2)
    med_size = int(round(9 * k)) | 1
    med = np.stack([ndi.median_filter(lab[..., i], size=med_size) for i in range(3)], -1)
    dev = lab - med
    de = np.sqrt((dev ** 2).sum(-1))
    # local variation of the neighborhood (robust): mean |deviation| in a wider box
    seen = ndi.gaussian_filter(de, 1.0 * k)

    out = {"size": [w, h]}
    specks, lines_edge, lines_free = [], [], []

    # smoothed picture: where are value edges?
    Ls = ndi.gaussian_filter(lab[..., 0], 3 * k)
    gy, gx = np.gradient(Ls)
    grad = np.hypot(gx, gy)

    cand = de > min(SPECK_DE, LINE_DE)
    lbl, n = ndi.label(cand, structure=np.ones((3, 3)))
    objs = ndi.find_objects(lbl)
    max_speck = 12 * k * k
    for i, sl in enumerate(objs, 1):
        if sl is None:
            continue
        m = lbl[sl] == i
        area = int(m.sum())
        ys, xs = np.nonzero(m)
        ys = ys + sl[0].start
        xs = xs + sl[1].start
        peak = float(de[ys, xs].max())
        cy, cx = float(ys.mean()), float(xs.mean())
        d = dev[ys, xs].mean(0)
        if area <= max_speck and (sl[0].stop - sl[0].start) <= 5 * k + 1 and (sl[1].stop - sl[1].start) <= 5 * k + 1:
            if peak < SPECK_DE:
                continue
            # the ring: neighborhood variation excluding the blob itself
            y0, y1 = max(0, sl[0].start - 4), min(h, sl[0].stop + 4)
            x0, x1 = max(0, sl[1].start - 4), min(w, sl[1].stop + 4)
            win = de[y0:y1, x0:x1].copy()
            win[lbl[y0:y1, x0:x1] == i] = np.nan
            rv = float(np.nanmedian(win)) if np.isfinite(win).any() else 0.0
            if rv > SPECK_RING * peak:
                continue
            specks.append({"x": round(cx, 1), "y": round(cy, 1), "area": area, "de": round(peak, 3),
                           "ink": round(float(de[ys, xs].sum()), 3), "seen": round(float(seen[ys, xs].max()), 3),
                           # lighter and grayer than around it: an underlayer showing through
                           "pinhole": bool(d[0] > 0 and np.hypot(*lab[ys, xs, 1:].mean(0)) < np.hypot(*med[ys, xs, 1:].mean(0))),
                           "dL": round(float(d[0]), 3), "da": round(float(d[1]), 3), "db": round(float(d[2]), 3)})
            continue
        if area < 6 * k:
            continue
        if peak < LINE_DE:
            continue
        pts = np.stack([ys, xs], 1).astype(float)
        cov = np.cov(pts.T) if area > 2 else np.eye(2)
        ev = np.sort(np.linalg.eigvalsh(cov))[::-1]
        length = 4.0 * np.sqrt(max(ev[0], 1e-9))
        width = area / max(length, 1.0)
        if length < 8 * k or width > 2.5 * k:
            continue
        g = float(np.median(grad[ys, xs]))
        rec = {"x": round(cx, 1), "y": round(cy, 1), "area": area, "length": round(float(length), 1),
               "width": round(float(width), 2), "de": round(peak, 3), "dL": round(float(d[0]), 3),
               "edge_grad": round(g, 4)}
        (lines_edge if g > 2 * QUIET_GRAD else lines_free).append(rec)

    # flecks: small patches of an alien hue or chroma (bare ground slivers,
    # rings), judged on OKLab a, b alone
    chroma = np.hypot(dev[..., 1], dev[..., 2])
    ring_c = ndi.median_filter(chroma, size=int(round(15 * k)) | 1)
    fl = chroma > FLECK_CHROMA
    flbl, fn = ndi.label(fl, structure=np.ones((3, 3)))
    flecks = []
    for i, sl in enumerate(ndi.find_objects(flbl), 1):
        if sl is None:
            continue
        m = flbl[sl] == i
        area = int(m.sum())
        if area > 80 * k * k:
            continue
        ys, xs = np.nonzero(m)
        ys = ys + sl[0].start
        xs = xs + sl[1].start
        peak = float(chroma[ys, xs].max())
        if float(np.median(ring_c[ys, xs])) > 0.35 * peak:
            continue
        d = dev[ys, xs].mean(0)
        flecks.append({"x": round(float(xs.mean()), 1), "y": round(float(ys.mean()), 1), "area": area,
                       "chroma": round(peak, 3), "da": round(float(d[1]), 3), "db": round(float(d[2]), 3)})

    # mottle: band-pass texture in quiet windows
    L = lab[..., 0]
    bp = ndi.gaussian_filter(L, 0.8 * k) - ndi.gaussian_filter(L, 3.0 * k)
    Lq = ndi.gaussian_filter(L, 6 * k)
    qy, qx = np.gradient(Lq)
    qgrad = np.hypot(qx, qy)
    win = int(round(24 * k))
    mott = []
    quiet_n = 0
    rms_list = []
    for y in range(0, h - win + 1, win):
        for x in range(0, w - win + 1, win):
            if np.percentile(qgrad[y:y + win, x:x + win], 90) > QUIET_GRAD:
                continue
            quiet_n += 1
            r = float(np.sqrt((bp[y:y + win, x:x + win] ** 2).mean()))
            rms_list.append(r)
            if r > MOTTLE_RMS:
                mott.append({"x": x, "y": y, "rms": round(r, 4)})

    area_mp = w * h / 1e6
    out.update({
        "specks": len(specks), "edge_lines": len(lines_edge), "free_lines": len(lines_free),
        "specks_per_mpx": round(len(specks) / area_mp, 1),
        "visible_specks": sum(1 for x in specks if x["seen"] >= VISIBLE),
        "pinholes": sum(1 for x in specks if x["seen"] >= VISIBLE and x["pinhole"]),
        "speck_ink_per_mpx": round(sum(x["ink"] for x in specks) / area_mp, 1),
        "flecks": len(flecks), "fleck_px_per_mpx": round(sum(x["area"] for x in flecks) / area_mp, 1),
        "lines_per_mpx": round((len(lines_edge) + len(lines_free)) / area_mp, 1),
        "quiet_windows": quiet_n, "mottle_windows": len(mott),
        "mottle_share": round(len(mott) / quiet_n, 3) if quiet_n else None,
        "quiet_bandpass_rms_median": round(float(np.median(rms_list)), 4) if rms_list else None,
        "speck_list": specks, "fleck_list": flecks, "edge_line_list": lines_edge, "free_line_list": lines_free, "mottle_list": mott,
        "_win": win,
    })
    return out, lbl


def overlay(rgb, res, lbl):
    h, w, _ = rgb.shape
    gray = rgb.mean(-1, keepdims=True).repeat(3, -1) * 0.55 + 60
    ov = gray.copy()
    win = res["_win"]
    for m in res["mottle_list"]:
        y, x = m["y"], m["x"]
        ov[y:y + win, x:x + win] = ov[y:y + win, x:x + win] * 0.6 + np.array([255, 220, 0]) * 0.4
    def paint(lst, color, r):
        for s in lst:
            y, x = int(round(s["y"])), int(round(s["x"]))
            yy, xx = np.ogrid[-r:r + 1, -r:r + 1]
            ring = (np.abs(np.hypot(yy, xx) - r) < 0.9)
            for dy, dx in zip(*np.nonzero(ring)):
                py, px = y + dy - r, x + dx - r
                if 0 <= py < h and 0 <= px < w:
                    ov[py, px] = color
    # line pixels themselves
    for lst, color in ((res["edge_line_list"], (0, 230, 255)), (res["free_line_list"], (40, 90, 255))):
        for s in lst:
            y, x = int(round(s["y"])), int(round(s["x"]))
            i = lbl[min(h - 1, max(0, y)), min(w - 1, max(0, x))]
            if i:
                ov[lbl == i] = color
            else:
                paint([s], color, 3)
    paint(res["fleck_list"], (255, 0, 255), 7)
    paint([x for x in res["speck_list"] if x["seen"] < VISIBLE], (150, 60, 60), 4)
    paint([x for x in res["speck_list"] if x["seen"] >= VISIBLE], (255, 30, 30), 5)
    sheet = np.concatenate([rgb.astype(float), np.full((h, 8, 3), 30.0), ov], 1)
    return Image.fromarray(np.clip(sheet, 0, 255).astype(np.uint8))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("png")
    ap.add_argument("--out", help="overlay sheet (PNG): render | marks")
    ap.add_argument("--scale", type=float, default=3.2, help="render px per canvas unit (3.2 at 3200)")
    ap.add_argument("--box", help="x0,y0,x1,y1 in pixels: analyze only this window")
    ap.add_argument("--json", action="store_true", help="print every mark as JSON")
    a = ap.parse_args()
    rgb = np.asarray(Image.open(a.png).convert("RGB"))
    if a.box:
        x0, y0, x1, y1 = map(int, a.box.split(","))
        rgb = rgb[y0:y1, x0:x1]
    res, lbl = census(rgb, a.scale)
    if a.out:
        overlay(rgb, res, lbl).save(a.out)
    res.pop("_win")
    if a.json:
        print(json.dumps(res))
    else:
        keys = ["size", "specks", "visible_specks", "pinholes", "specks_per_mpx", "speck_ink_per_mpx", "flecks", "fleck_px_per_mpx", "edge_lines", "free_lines", "lines_per_mpx",
                "quiet_windows", "mottle_windows", "mottle_share", "quiet_bandpass_rms_median"]
        print(" ".join(f"{k}={res[k]}" for k in keys))


if __name__ == "__main__":
    sys.exit(main())
