#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy"]
# ///
# Natural black chalk's dry color (R-infinity) from its makeup, by the dry
# pigment recipe charcoal_optics.py uses (Mie in air, Kubelka-Munk): what a
# bare bed of its crumbs returns with no first-surface veil over it
# (notes/research/paper_veil.md §3). And graphite's own surface reflection
# (§4). Run: uv run notes/research/black_chalk_optics.py
#
# Black chalk is a carbonaceous shale: fine clay with about 10-15 % carbon
# by weight diffused through it (historyofdrawing.com, "Drawing
# Materials"; the Met and the National Gallery: carbonaceous shale or
# schist). Its carbon is the shale's organic matter. Its optical constants
# aren't measured for a drawing chalk; mature kerogen's are close to a
# charred cellulose's (both disordered aromatic carbon), so the carbon is
# taken as charcoal_optics.py's 500 C char, with its 400 C and 600 C chars
# and lamp black as bounds [E].
import importlib.util, os
import numpy as np

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("charcoal_optics", os.path.join(here, "charcoal_optics.py"))
co = importlib.util.module_from_spec(spec)
import sys, io
_out, sys.stdout = sys.stdout, io.StringIO()  # (its own tables aren't wanted here)
spec.loader.exec_module(co)
sys.stdout = _out

CLAY_RHO, CARBON_RHO = 2.65, 1.5  # illite/kaolinite; kerogen 1.2-1.6 g/cm3, char 1.44-1.67


def vol_share(wt):
    """Carbon's share of the solids by volume at `wt` by weight."""
    return (wt / CARBON_RHO) / (wt / CARBON_RHO + (1 - wt) / CLAY_RHO)


def separate(wt, carbon, d_c=1.0, d_clay=1.0):
    """Clay platelets and carbon particles as separate scatterers, mixed by
    Kubelka-Munk (K and S by volume)."""
    f = vol_share(wt)
    kc, sc = co.ks(carbon, d_c, 1.0)
    kk, sk = co.ks(lambda lam: co.KAOLIN, d_clay, 1.0)
    return co.km(f * kc + (1 - f) * kk, f * sc + (1 - f) * sk)


def crumbs(wt, carbon, d=3.0, solids=0.9):
    """Crumbs of the shale itself: its carbon finer than light inside the
    clay, an effective medium (Bruggeman) of carbon, clay and the shale's
    pores (a shale's porosity is about 10 %: `solids`)."""
    f = vol_share(wt)
    idx = lambda lam: co.bruggeman([(carbon(lam), solids * f), (co.KAOLIN, solids * (1 - f)), (1.0 + 0j, 1 - solids)])
    k, s = co.ks(idx, d, 1.0)
    return co.km(k, s)


cel = lambda t: (lambda lam: co.char_index(lam, t))
soot = lambda lam: co.SOOT
if __name__ == "__main__":
    print("black chalk R-inf (linear R, G, B), Y and L*, no veil; with the 4 % veil")
    for wt in (0.10, 0.125, 0.15):
        for name, c in (("char 500 C", co.char), ("char 400 C", cel(400.0)), ("char 600 C", cel(600.0)), ("lamp black", soot)):
            for how, r in (("separate", separate(wt, c)), ("crumbs", crumbs(wt, c))):
                y, l = co.lum(r)
                yv, lv = co.lum(co.seen(r))
                print(f"  {wt*100:4.1f} wt% {name:10s} {how:8s} R {r.round(3)}  Y {y:.3f} L* {l:4.1f} | veiled Y {yv:.3f} L* {lv:4.1f}")
    # graphite (ordinary ray, the basal plane: Djurisic & Li 1999, J. Appl.
    # Phys. 85:7404, via refractiveindex.info main/C/Djurisic-o): normal
    # reflectance and absorption length
    print("graphite basal plane, normal incidence")
    for lam, n, k in ((0.45008, 2.6176, 1.3013), (0.55455, 2.7229, 1.4919), (0.60891, 2.8023, 1.5663)):
        r = ((n - 1) ** 2 + k * k) / ((n + 1) ** 2 + k * k)
        print(f"  {lam:.3f} um: n {n:.3f} k {k:.3f}  R {r:.3f}  absorption length {lam / (4 * np.pi * k) * 1000:.0f} nm")
