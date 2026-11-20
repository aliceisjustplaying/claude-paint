#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy"]
# ///
# What hue a hematite earth's tint with white has, by Mie and Kubelka–Munk,
# and what the researched tubes' rule (one wavelength-flat S fitted to the
# masstone, notes in crates/paint/src/mixer/researched.json "flat_s_limitation")
# makes of it: notes/research/pastel_layering.md §5.
# uv run notes/research/earth_tint_mie.py
import importlib.util, pathlib
import numpy as np
spec = importlib.util.spec_from_file_location("mie", pathlib.Path(__file__).with_name("dry_pigment_mie.py"))
mie = importlib.util.module_from_spec(spec); spec.loader.exec_module(mie)

# Hematite (alpha-Fe2O3), ordinary ray: M. R. Querry, Optical constants,
# CRDC-CR-85034 (1985), via refractiveindex.info (Fe2O3/Querry-o, CC0).
QUERRY = [(0.40, 2.756, 1.294), (0.41, 2.863, 1.271), (0.42, 2.962, 1.231), (0.43, 3.052, 1.173), (0.44, 3.126, 1.101),
          (0.45, 3.181, 1.020), (0.46, 3.217, 0.943), (0.47, 3.242, 0.874), (0.48, 3.264, 0.809), (0.49, 3.280, 0.741),
          (0.50, 3.282, 0.675), (0.51, 3.272, 0.622), (0.52, 3.262, 0.587), (0.53, 3.266, 0.564), (0.54, 3.286, 0.538),
          (0.55, 3.318, 0.498), (0.56, 3.348, 0.437), (0.57, 3.361, 0.357), (0.58, 3.348, 0.274), (0.59, 3.312, 0.202),
          (0.60, 3.265, 0.149), (0.61, 3.218, 0.114), (0.62, 3.175, 0.089), (0.63, 3.135, 0.073), (0.64, 3.102, 0.065),
          (0.65, 3.074, 0.057), (0.66, 3.051, 0.051), (0.67, 3.029, 0.043), (0.68, 3.008, 0.038), (0.69, 2.990, 0.034),
          (0.70, 2.972, 0.031)]
lam = np.array([q[0] for q in QUERRY]); n = np.array([q[1] for q in QUERRY]); k = np.array([q[2] for q in QUERRY])

def cmf(l):  # CIE 1931 2-degree, the multi-lobe fit of Wyman, Sloan & Shirley (JCGT 2013)
    g = lambda x, m, s1, s2: np.exp(-0.5*((x-m)/(s1 if x < m else s2))**2)
    return (1.056*g(l,599.8,37.9,31.0)+0.362*g(l,442.0,16.0,26.7)-0.065*g(l,501.1,20.4,26.2),
            0.821*g(l,568.8,46.9,40.5)+0.286*g(l,530.9,16.3,31.1),
            1.217*g(l,437.0,11.8,36.0)+0.681*g(l,459.0,26.0,13.8))
def planck(l, T=6504):  # D65 stood in for by its correlated colour temperature
    l = l*1e-9; return 1/(l**5*(np.exp(1.4388e-2/(l*T))-1))
L = lam*1000; C = np.array([cmf(l) for l in L]); E = planck(L); W = (C.T*E).sum(1)
def lab(R):
    X = (C.T*E*R).sum(1)/W[1]; Xn = W/W[1]
    f = lambda t: np.cbrt(t) if t > 0.008856 else 7.787*t+16/116
    fx, fy, fz = [f(X[i]/Xn[i]) for i in range(3)]
    a, b = 500*(fx-fy), 200*(fy-fz)
    return 116*fy-16, a, b, np.degrees(np.arctan2(b, a)) % 360, np.hypot(a, b)
km = lambda ks: 1 + ks - np.sqrt(ks*ks + 2*ks)

NMED = 1.48  # linseed oil, as the researched tubes are oil films
Sw = np.array([0.75*mie.sizeavg(2.0, NMED, 1.0, lam=l)[0] for l in lam])  # lead white, d 1 um
for d in (0.2, 0.4, 1.0):
    S = np.zeros_like(lam); K = np.zeros_like(lam)
    for i, l in enumerate(lam):
        s, a = mie.sizeavg(complex(n[i], k[i]), NMED, d, lam=l)
        S[i] = 0.75*s; K[i] = 2*a
    print(f"hematite d={d} um in oil: S(450, 550, 650 nm) = {S[5]:.2f} {S[15]:.2f} {S[25]:.2f} /um, S650/S550 {S[25]/S[15]:.2f}")
    print("   masstone  L* %.1f a* %.1f b* %.1f h %.1f C %.1f" % lab(km(K/S)))
    Kflat = (K/S)*S.mean()  # the masstone's K/S over one flat S
    for c in (0.3, 0.1, 0.03, 0.01):
        t = lab(km(c*K/(c*S+(1-c)*Sw))); f = lab(km(c*Kflat/(c*S.mean()+(1-c)*Sw)))
        print(f"   {c:4.2f} in white: Mie L* {t[0]:.1f} h {t[3]:5.1f} C {t[4]:4.1f} | flat S L* {f[0]:.1f} h {f[3]:5.1f} C {f[4]:4.1f} | flat - Mie dh {f[3]-t[3]:+.1f}")
