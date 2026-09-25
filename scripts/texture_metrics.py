# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "pillow", "scipy", "scikit-image"]
# ///
"""Texture measurements for a lossless crop (Round 7 texture forensics,
notes/round7/texture.md). What each number means:

  block8 / block16  blockiness at an 8 / 16 px period: the mean |step|
                    between neighboring pixels, averaged per phase (x mod P,
                    and y mod P), (max phase - mean) / mean, in %, the
                    steepest 2 % of steps clipped (long lines, cracks). A JPEG at
                    q80 shows 10-40 % at 8; noise alone about 1-3 %.
  b8/b7,9           block8 over the mean of the same measure at 7 and 9 px:
                    a JPEG grid is specific to 8 (q80: well above 1), a
                    striated passage is not (about 1)
  L_2-4 ... L_32-64 radial power of the texture's lightness (L*, after
                    subtracting a sigma-24 px blur: the passage's gradient
                    taken out), summed over period bands in px, as the
                    standard deviation that band contributes (L* units).
  slope             log-log slope of that spectrum between 3 and 48 px
  Lhf, ahf, bhf     per-pixel noise at 1-2 px: std of the difference of
                    Gaussians (sigma 0.5 - sigma 2) of L*, a*, b*
  chroma/luma       (ahf^2 + bhf^2)^0.5 / Lhf
  flat%             pixels whose 5x5 neighborhood (after a sigma-1 blur
                    that removes the dither) spans < 0.3 L*: flat patches
  step10%           share of the total gradient magnitude (sigma-1 blurred
                    L*) carried by the steepest 10 % of pixels: flat patches
                    joined by sharp small steps push it up (a smooth random
                    texture: about 25-30 %)
  gkurt             kurtosis of the x-derivative of sigma-1 blurred L*
  grid              regular edges at any period: the mean |step| per column
                    (and per row), its spectrum over periods 4-40 px, peak /
                    median power (the larger of x and y; noise: about 4-8);
                    gridP its period in px
  axis%             share of the gradient energy (sigma-1.5 L*) whose
                    direction lies within 11.25 deg of the x or y axis
                    (isotropic texture: 25 %; square patches push it up)
  cells             patchiness: std of L* 8x8 block means / std of the
                    within-block L* (blotches the size of a JPEG block)

usage: uv run scripts/texture_metrics.py a.png [b.png ...] [--window x0,y0,x1,y1] [--csv]
"""
import sys

import numpy as np
from PIL import Image
from scipy import ndimage as ndi
from skimage.color import rgb2lab


def load(path, win=None):
    im = np.asarray(Image.open(path).convert("RGB"), dtype=np.float64) / 255.0
    if win:
        x0, y0, x1, y1 = win
        im = im[y0:y1, x0:x1]
    return im


def blockiness(L, P):
    out = []
    for axis in (1, 0):
        d = np.abs(np.diff(L, axis=axis))
        # (a few long one-pixel lines, a crack or a glint, must not pass for
        # a block edge: clip the steepest 2 %)
        d = np.minimum(d, np.percentile(d, 98))
        n = d.shape[axis]
        prof = np.array([d.take(range(k, n, P), axis=axis).mean() for k in range(P)])
        out.append((prof.max() - prof.mean()) / prof.mean())
    return 100 * max(out)


BANDS = [(2, 4), (4, 8), (8, 16), (16, 32), (32, 64)]


def spectrum(L):
    t = L - ndi.gaussian_filter(L, 24, mode="reflect")
    h, w = t.shape
    win = np.outer(np.hanning(h), np.hanning(w))
    F = np.fft.fftshift(np.fft.fft2(t * win))
    pw = np.abs(F) ** 2 / (win ** 2).sum()
    fy = np.fft.fftshift(np.fft.fftfreq(h))[:, None]
    fx = np.fft.fftshift(np.fft.fftfreq(w))[None, :]
    fr = np.sqrt(fx ** 2 + fy ** 2)
    per = np.where(fr > 0, 1.0 / np.maximum(fr, 1e-9), np.inf)
    bands = []
    for a, b in BANDS:
        m = (per >= a) & (per < b)
        bands.append(np.sqrt(pw[m].sum() / (h * w)))
    # slope: radial mean in log bins from 3 to 48 px
    edges = np.geomspace(3, 48, 9)
    xs, ys = [], []
    for a, b in zip(edges[:-1], edges[1:]):
        m = (per >= a) & (per < b)
        if m.any():
            xs.append(np.log(1 / np.sqrt(a * b)))
            ys.append(np.log(pw[m].mean()))
    slope = np.polyfit(xs, ys, 1)[0]
    return bands, slope


def measure(im):
    lab = rgb2lab(im)
    L, a, b = lab[..., 0], lab[..., 1], lab[..., 2]
    r = {}
    r["block8"] = blockiness(L, 8)
    r["block16"] = blockiness(L, 16)
    r["b8/b7,9"] = r["block8"] / (0.5 * (blockiness(L, 7) + blockiness(L, 9)))
    bands, slope = spectrum(L)
    for (lo, hi), v in zip(BANDS, bands):
        r[f"L_{lo}-{hi}"] = v
    r["slope"] = slope
    dog = lambda c: ndi.gaussian_filter(c, 0.5) - ndi.gaussian_filter(c, 2.0)
    r["Lhf"], r["ahf"], r["bhf"] = (dog(c).std() for c in (L, a, b))
    r["chroma/luma"] = np.hypot(r["ahf"], r["bhf"]) / r["Lhf"]
    Ls = ndi.gaussian_filter(L, 1.0)
    rng = ndi.maximum_filter(Ls, 5) - ndi.minimum_filter(Ls, 5)
    r["flat%"] = 100 * (rng < 0.3).mean()
    gy, gx = np.gradient(Ls)
    g = np.hypot(gx, gy).ravel()
    g.sort()
    r["step10%"] = 100 * g[int(0.9 * len(g)):].sum() / g.sum()
    dx = gx.ravel() - gx.mean()
    r["gkurt"] = (dx ** 4).mean() / (dx ** 2).mean() ** 2
    peaks = []
    for axis in (0, 1):
        prof = np.abs(np.diff(L, axis=1 - axis)).mean(axis=axis)
        prof = prof - ndi.uniform_filter1d(prof, 65, mode="reflect")
        P = np.abs(np.fft.rfft(prof * np.hanning(len(prof)))) ** 2
        f = np.fft.rfftfreq(len(prof))
        m = (f >= 1 / 40) & (f <= 1 / 4)
        k = np.argmax(P[m])
        peaks.append((P[m][k] / np.median(P[m]), 1 / f[m][k], "yx"[axis]))
    best = max(peaks)
    r["grid"], r["gridP"] = best[0], best[1]
    L15 = ndi.gaussian_filter(L, 1.5)
    gy2, gx2 = np.gradient(L15)
    e = gx2 ** 2 + gy2 ** 2
    th = np.degrees(np.arctan2(gy2, gx2)) % 90
    ax = (th < 11.25) | (th > 78.75)
    r["axis%"] = 100 * e[ax].sum() / e.sum()
    h, w = (L.shape[0] // 8) * 8, (L.shape[1] // 8) * 8
    B = L[:h, :w].reshape(h // 8, 8, w // 8, 8)
    t = ndi.gaussian_filter(L, 24)[:h, :w].reshape(h // 8, 8, w // 8, 8)
    B = B - t
    r["cells"] = B.mean(axis=(1, 3)).std() / B.std(axis=(1, 3)).mean()
    return r


def main():
    args = sys.argv[1:]
    win = None
    csv = "--csv" in args
    args = [x for x in args if x != "--csv"]
    if "--window" in args:
        i = args.index("--window")
        win = tuple(int(v) for v in args[i + 1].split(","))
        del args[i : i + 2]
    rows = [(p, measure(load(p, win))) for p in args]
    keys = list(rows[0][1].keys())
    if csv:
        print("file," + ",".join(keys))
        for p, r in rows:
            print(p + "," + ",".join(f"{r[k]:.4g}" for k in keys))
    else:
        for p, r in rows:
            print(p)
            print("  " + "  ".join(f"{k} {r[k]:.3g}" for k in keys))


if __name__ == "__main__":
    main()
