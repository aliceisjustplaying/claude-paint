# Estimated wet-paint densities for engine 9

Every finished-paint density here is modeled, not measured. Inputs live
in `scripts/pigment_densities.json`; `calibrated-densities.bin` contains
derived g/cm³ values in the paint order recorded by `calibrated.json`.

For oil mass fraction `w`, the model is
`rho_paint = 1/(w/rho_oil + (1-w)/rho_pigment)`. Component volumes add
without air, contraction or drying shrinkage. Source mixture masses are
finished-paint masses. Bone black uses the 2.29 g/ml solid density cited
from Gettens and Stout, not CAMEO's 0.7–0.8 g/cm³ bulk-powder figure:
powder voids must not be counted as solid volume.

## Measured source inputs

[Geldof et al. (2018), Table 5](https://www.nature.com/articles/s40494-018-0181-6/tables/5)
reports average oil mass percentages: barium sulfate 28, calcium carbonate
28, zinc white 37, eosin 37, ultramarine 41, bone black 48, cobalt blue 51
and Prussian blue 56. These paints use poppyseed oil except eosin and bone
black, which use linseed oil. These are measured oil averages, not
wet-paint density measurements.

## Sourced component values and estimates

These museum compilation entries were read on 2026-10-23. Their handbook
values describe materials, not the exact reconstruction batches. Values
chosen inside ranges remain estimates. The JSON records each paint's
source, oil and estimate status.

| component | g/cm³ used | source and limit |
|---|---:|---|
| poppyseed oil | 0.925 | [CAMEO](https://cameo.mfa.org/wiki/Poppy_seed_oil), estimate within 0.924–0.926 |
| linseed oil | 0.93 | [CAMEO](https://cameo.mfa.org/wiki/Linseed_oil), estimate within 0.921–0.936 |
| cobalt blue | 3.83 | [CAMEO](https://cameo.mfa.org/wiki/Cobalt_blue) |
| zinc white | 5.6 | [CAMEO](https://cameo.mfa.org/wiki/Zinc_white), estimate within 5.47–5.65 |
| bone black | 2.29 | [CAMEO](https://cameo.mfa.org/wiki/Bone_black), cited Gettens/Stout solid density |
| barium sulfate | 4.36 | [CAMEO](https://cameo.mfa.org/wiki/Barium_sulfate), cited Gettens/Stout value |
| calcium carbonate | 2.7 | [CAMEO](https://cameo.mfa.org/wiki/Calcium_carbonate), low end of 2.7–2.95 |
| chrome yellow | 5.96 | [CAMEO](https://cameo.mfa.org/wiki/Chrome_yellow), cited Gettens/Stout value |
| lemon chrome | 6.1 | Estimate within that chromate range, not a measured chromate/sulfate mixture |
| lead white | 6.7 | [CAMEO](https://cameo.mfa.org/wiki/Lead_white), cited Gettens/Stout value |
| Prussian blue | 1.83 | [CAMEO](https://cameo.mfa.org/wiki/Prussian_blue) |
| vermilion | 8.09 | [CAMEO](https://cameo.mfa.org/wiki/Vermilion), cited Gettens/Stout value |
| viridian | 3.32 | [CAMEO](https://cameo.mfa.org/wiki/Viridian) |
| emerald green | 3.27 | [CAMEO](https://cameo.mfa.org/wiki/Emerald_green) |
| ultramarine | 2.35 | Catalog proxy from [existing research](research/pigment_oil.md), not a reconstruction measurement |
| minium | 8.8 | Catalog proxy from the same research |
| eosin, cochineal and both wax lakes | 2.0 | Estimated lake substrate; exact substrate, wax and lead fractions are unknown |

## Remaining estimated oil fractions

For paints absent from Table 5, `w=1.45*OA/(100+1.45*OA)` uses the catalog
grinding estimate. OA inputs, grams of oil per 100 grams of pigment, are
20 for chrome yellow, 24 for lemon chrome, 70 for cochineal, 13 for emerald,
10 for lead white, 9 for minium, 10 for vermilion and 80 for viridian.
Their sources and limits are in the existing research above. Both wax
lakes use eosin's 0.37 oil fraction as a labeled proxy, not a Table 5
measurement of those paints. Vermilion uses linseed oil; the other
estimated non-lake paints use poppyseed oil. These assumptions are not
parameters fitted against held-out color rows.

This supplies one coherent mass/volume model for fitting, scoring and the
runtime's volume API. Exact historical paint densities and the uncertainty
of the source mixture ladders remain unresolved.
