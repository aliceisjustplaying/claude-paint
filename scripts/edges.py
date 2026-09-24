# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "pillow", "scipy"]
# ///
"""Edge profile: how crisp, how varied and how straight a picture's edges are.

    uv run scripts/edges.py RENDER.png --box x0,y0,x1,y1 [--mode column|all] [--scale 3.2] [--json]
                                      [--plot OUT.png]

All positions are render pixels. Lightness is OKLab L.

- `column`: a mostly level edge (a ridge, a shore, a skyline) running across the box.
  In each pixel column the edge is the strongest vertical step in L (smoothed a
  little along x). Per column: the edge's y, its contrast (the L difference
  between the median of 8-24 px above and below) and its width (the distance
  between the 10% and 90% crossings of the step, px). Summaries: width median
  and spread, the share of columns where the edge is "found" (width <= 1.2
  units), "soft" (1.2-4 units) and "lost" (contrast under a third of the
  median, or width > 4 units), the variation of width along the edge (the
  coefficient of variation of the width smoothed over 1.5 units: 0 is one
  width all along) and the waver (RMS of the edge's y about its own
  8-unit running mean, units: 0 is a curve drawn by a ruler).
- `all`: every edge pixel in the box (a local maximum of the gradient over a
  contrast floor) is profiled along its gradient direction the same way. For
  silhouettes (firs, figures) whose edges run every way.

`--plot` writes a small strip: the crop with the found edges tinted by width
(green found, yellow soft, red lost; column mode) or the edge pixels (all).
"""
import argparse
import json

import numpy as np
from PIL import Image
from scipy import ndimage as ndi


def oklab_L(rgb):
    c = rgb.astype(np.float64) / 255.0
    lin = np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)
    r, g, b = lin[..., 0], lin[..., 1], lin[..., 2]
    l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b
    m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b
    s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b
    l_, m_, s_ = np.cbrt(l), np.cbrt(m), np.cbrt(s)
    return 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_


def step_width(p, lo_ref, hi_ref):
    """Width (px) between the 10% and 90% crossings of a monotone-ish step profile p."""
    if abs(hi_ref - lo_ref) < 1e-6:
        return np.nan
    t = (p - lo_ref) / (hi_ref - lo_ref)
    n = len(t)
    c = n // 2

    def cross(level, direction):
        # walk out from the center to the first crossing of `level`
        i = c
        if direction > 0:
            while i + 1 < n and t[i] < level:
                i += 1
            if i == 0 or t[i] < level:
                return np.nan
            a, b = t[i - 1], t[i]
            return i - 1 + (level - a) / (b - a + 1e-12)
        else:
            while i - 1 >= 0 and t[i] > level:
                i -= 1
            if i == n - 1 or t[i] > level:
                return np.nan
            a, b = t[i], t[i + 1]
            return i + (level - a) / (b - a + 1e-12)

    # t rises from ~0 (index 0) to ~1 (index n-1)
    x90 = cross(0.9, +1)
    x10 = cross(0.1, -1)
    return x90 - x10


def column_mode(L, scale, near=8, far=24):
    h, w = L.shape
    Ls = ndi.gaussian_filter1d(L, 1.0, axis=1)
    gy = np.abs(np.diff(ndi.gaussian_filter1d(Ls, 0.7, axis=0), axis=0))
    ys = np.argmax(gy[far:h - far - 1], axis=0) + far
    out = []
    for x in range(w):
        y = ys[x]
        col = Ls[:, x]
        above = np.median(col[y - far:y - near])
        below = np.median(col[y + near + 1:y + far + 1])
        prof = col[y - far:y + far + 1].copy()
        # make it rise
        if above > below:
            prof = prof[::-1]
            lo, hi = below, above
        else:
            lo, hi = above, below
        wd = step_width(prof, lo, hi)
        out.append((x, float(y) + 0.5, float(abs(above - below)), float(wd)))
    a = np.array(out)
    con, wid, ey = a[:, 2], a[:, 3], a[:, 1]
    med_con = np.nanmedian(con)
    unit = scale
    lost = (con < med_con / 3) | (wid > 4 * unit) | np.isnan(wid)
    found = (~lost) & (wid <= 1.2 * unit)
    soft = (~lost) & (~found)
    wsm = ndi.uniform_filter1d(np.nan_to_num(wid, nan=np.nanmedian(wid)), max(1, int(1.5 * unit)))
    trend = ndi.uniform_filter1d(ey, max(3, int(8 * unit)), mode="nearest")
    waver = float(np.sqrt(np.mean((ey - trend) ** 2)) / unit)
    s = {
        "mode": "column",
        "columns": int(len(a)),
        "contrast_median_L": round(float(med_con), 4),
        "width_units_p10_p50_p90": [round(float(np.nanpercentile(wid, q)) / unit, 2) for q in (10, 50, 90)],
        "found_soft_lost": [round(float(found.mean()), 3), round(float(soft.mean()), 3), round(float(lost.mean()), 3)],
        "width_variation_cv": round(float(np.std(wsm) / max(np.mean(wsm), 1e-6)), 3),
        "contrast_variation_cv": round(float(np.nanstd(con) / max(np.nanmean(con), 1e-6)), 3),
        "waver_units": round(waver, 3),
    }
    return s, a, (found, soft, lost)


def all_mode(L, scale, floor=0.04, near=5, far=14):
    Ls = ndi.gaussian_filter(L, 0.8)
    gx = ndi.sobel(Ls, axis=1) / 8
    gy = ndi.sobel(Ls, axis=0) / 8
    g = np.hypot(gx, gy)
    # non-maximum suppression along the gradient (4 directions)
    ang = (np.round(np.arctan2(gy, gx) / (np.pi / 4)) % 4).astype(int)
    keep = np.zeros_like(g, dtype=bool)
    for k, (dy, dx) in enumerate([(0, 1), (1, 1), (1, 0), (1, -1)]):
        sel = ang == k
        a = np.roll(np.roll(g, dy, 0), dx, 1)
        b = np.roll(np.roll(g, -dy, 0), -dx, 1)
        keep |= sel & (g >= a) & (g >= b)
    h, w = L.shape
    keep[:far + 1] = keep[-far - 1:] = False
    keep[:, :far + 1] = keep[:, -far - 1:] = False
    ys, xs = np.nonzero(keep & (g > floor / 4))
    rows = []
    t = np.arange(-far, far + 1, dtype=np.float64)
    for y, x in zip(ys, xs):
        nx, ny = gx[y, x] / g[y, x], gy[y, x] / g[y, x]
        px, py = x + nx * t, y + ny * t
        prof = ndi.map_coordinates(Ls, [py, px], order=1, mode="nearest")
        lo = np.median(prof[:far - near])
        hi = np.median(prof[far + near + 1:])
        con = hi - lo
        if con < floor:
            continue
        wd = step_width(prof, lo, hi)
        if np.isnan(wd):
            continue
        rows.append((x, y, con, wd))
    a = np.array(rows) if rows else np.zeros((0, 4))
    if len(a) == 0:
        return {"mode": "all", "edge_px": 0}, a, None
    wid = a[:, 3] / scale
    s = {
        "mode": "all",
        "edge_px": int(len(a)),
        "contrast_median_L": round(float(np.median(a[:, 2])), 4),
        "width_units_p10_p50_p90": [round(float(np.percentile(wid, q)), 2) for q in (10, 50, 90)],
        "found_soft_share": [round(float((wid <= 1.2).mean()), 3), round(float(((wid > 1.2) & (wid <= 4)).mean()), 3)],
        "width_cv": round(float(np.std(wid) / max(np.mean(wid), 1e-6)), 3),
    }
    return s, a, None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("png")
    ap.add_argument("--box", required=True, help="x0,y0,x1,y1 render pixels")
    ap.add_argument("--mode", default="column", choices=["column", "all"])
    ap.add_argument("--scale", type=float, default=3.2, help="render px per canvas unit")
    ap.add_argument("--floor", type=float, default=0.04, help="all mode: least L contrast of an edge")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--plot")
    ap.add_argument("--label", default="")
    a = ap.parse_args()
    x0, y0, x1, y1 = [int(v) for v in a.box.split(",")]
    img = np.asarray(Image.open(a.png).convert("RGB"))[y0:y1, x0:x1]
    L = oklab_L(img)
    if a.mode == "column":
        s, rows, cls = column_mode(L, a.scale)
    else:
        s, rows, cls = all_mode(L, a.scale, a.floor)
    s["label"] = a.label
    s["box"] = [x0, y0, x1, y1]
    if a.plot:
        vis = img.copy()
        if a.mode == "column" and cls is not None:
            found, soft, lost = cls
            for (x, y, _, _), f, so in zip(rows, found, soft):
                c = (40, 220, 60) if f else ((240, 210, 40) if so else (230, 40, 40))
                yy = int(y)
                vis[max(0, yy - 1):yy + 1, int(x)] = c
        else:
            for x, y, _, wd in rows:
                u = wd / a.scale
                c = (40, 220, 60) if u <= 1.2 else ((240, 210, 40) if u <= 4 else (230, 40, 40))
                vis[int(y), int(x)] = c
        Image.fromarray(vis).save(a.plot)
    print(json.dumps(s) if a.json else "\n".join(f"{k}: {v}" for k, v in s.items()))


if __name__ == "__main__":
    main()
