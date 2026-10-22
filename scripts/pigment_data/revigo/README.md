# Revigo paint reflectance data

Measured reflectance spectra of the oil paints reconstructed for the Revigo
project (Re-assessing Vincent van Gogh's colors), copied unchanged from
https://github.com/fligt/revigo-spectra (commit f09ed72, files under data/),
where they are published under the CC0 1.0 Universal dedication (LICENSE here).
The measurements and their method are described in Geldof, Proaño Gaibor,
Ligterink, Hendriks and Kirchner, "Reconstructing Van Gogh's palette to
determine the optical characteristics of his paints", Heritage Science 6:17
(2018), https://doi.org/10.1186/s40494-018-0181-6 (CC BY 4.0).

Each paint was drawn down 100 µm thick on the black and the white field of a
BYK-Gardner opacity chart, pure and mixed by weight with zinc white and bone
black, and measured 400–700 nm in 10 nm steps with a Konica-Minolta CM-2600d
sphere spectrophotometer, specular included (Saunderson k1 0.04, k2 0.49).

- revigo-mixture-specifications.txt: sample id, the paints in it and their
  weight percentages.
- revigo-aggregated-palette-spectral-data.txt: percent reflectance per sample
  and wavelength; _b and _w are the bare black and white fields, _ob and _ow
  the paint over them.

Paint series: 01–03 eosin lakes (Eosin1Wax, Eosin, Eosin2PbWax), 04 cochineal,
05 red lead (minium), 06 vermilion, 07 zinc white, 08 lead white, 09 viridian,
10 emerald green, 11 ultramarine, 12 cobalt blue, 13 Prussian blue, 14 chrome
yellow (Kremer), 15 chrome yellow lemon (reconstructed after Otero et al.),
16 barium sulphate, 17 calcium carbonate. scripts/build_pigment_tables.py
derives the mixer's primaries from these files.
