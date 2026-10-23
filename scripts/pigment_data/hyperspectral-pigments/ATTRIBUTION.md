# Hyperspectral Pigments

Hilda Deborah (2022), *Hyperspectral Pigments*, version 1, Zenodo.
https://doi.org/10.5281/zenodo.5592485

The dataset record licenses these files Creative Commons Attribution 4.0
International: https://zenodo.org/records/5592485 . The full legal text is
in `LICENSE`. The research was funded by the Research Council of Norway,
FRIPRO FRINATEK “Metrological texture analysis for hyperspectral images”,
project 274881. The associated publication is H. Deborah, *Hyperspectral
Pigment Dataset*, WHISPERS 2022.

The five source files are unchanged downloads from the record's `files/`
endpoint: `__speclib_averages.sli`, `__speclib_medians.sli`, their ENVI
headers and `__pigmentlistZenodo.xls`. `SHA256SUMS.json` records their integrity. The
image cubes, paper and manufacturer documentation are not redistributed.

The generator selects named spectra, linearly resamples them with stated
endpoint extension and derives explicitly modeled oil coefficients. These
changes are not endorsed by the source author. The source data and derived
numerical tables retain CC BY 4.0 attribution; the repository's MIT code
license does not replace this data license.

The source article recommends spatial medians for metrological comparisons
(section 2.2, https://doi.org/10.1109/WHISPERS56178.2022.9955067). The fitter
uses those medians; spatial means remain available for sensitivity checks.
They are two statistics of the same pixels, not independent measurements.

There are 398 chart positions, 327 product numbers and four backing regions
per chart position. The four `sh` curves are not white-pigment tints. The
library contains no named bare-backing measurements. Native wavelengths
start at 405.369888 nm, so 380–400 nm values are endpoint estimates.

The manufacturer's chart-process description is
https://www.kremer-pigmente.com/elements/resources/products/files/FK_Supplement_EN.pdf .
It describes a single pigment layer, normally gum-arabic-bound, over four
printed gray fields. Its 2025 date does not establish the exact binder of
each earlier specimen. The lightest region supplies a measured appearance
reference, not an infinite-thickness reflectance or measured oil K/S.
