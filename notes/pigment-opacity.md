# RGB-only tube opacity in engine 9

The 39 catalog tubes without Revigo identities remain estimates, not measured
oil paints. Their catalog RGB and hiding inputs are unchanged. Engine 9 now
uses both inputs to choose a spectral surrogate; engines 8 and earlier keep
their original RGB reconstruction.

The surrogate is a nonnegative blend of the existing Revigo basis paints.
Its initial shape's mean scattering matches the catalog's estimated scattering, within the
available basis range. Among blends with that scattering, a bounded search
chooses a close reference color, then retains the linear RGB residual. A
one-dimensional optical-depth solve makes the reconstructed paint's 25 µm
black/white contrast match the estimated hiding input before tinting strength
is applied. This keeps opacity meaningful even when calibrated basis spectra
change. The
residual fades with transmission as it does for other reconstructed colors.
The surrogate's coefficients, not just its reference RGB, survive palette and
tool mixing. The fitted blend is a stand-in: it does not identify the tube's
chemical composition or turn these estimates into measured oil-paint K and S.

Yellow ochre and raw umber use named joint fits instead of the unconstrained
depth solve. Changing depth alone also changed their contribution to white
mixtures: the ochre extent grew threefold and made a pale ground saturated
yellow. The joint objective retains the catalog's estimated 25 µm hiding,
the pure reference color and the pre-opacity white-tint ladder at 1%, 5%,
10%, 25% and 50% color. It fits the basis shape and its extensive amount
together. Scattering remains derived from those same K and S coefficients,
so mixing does not average an independent opacity scale or introduce cross
terms. Catalog tinting strengths are unchanged.

The regression allows 0.015 Euclidean OKLab distance from the frozen legacy
tints in both palette color and a 120 µm film, and 0.01 absolute contrast
ratio error from the two estimated hiding inputs. These are preservation
tolerances for the existing catalog model, not measured oil-paint accuracy.
The cadmium stand-in and its opacity regression remain unchanged here; a
separate source-data layer owns its replacement and any source-based color
change. No glaze bound or exact legacy baseline is changed.

This corrects the reversed behavior of cadmium red and green earth. Natural
Pigments describes [PR108 cadmium red as opaque](https://www.naturalpigments.com/cadmium-red-medium-oil-paint.html),
including a 6 mil drawdown. Michael Harding describes
[natural PG23 terre verte](https://michaelharding.co.uk/oil-colours/terre-verte-115)
as weak and suitable for glazing. Those descriptions justify the material
ranking, not exact contrast ratios at a particular thickness. The numerical
contrast-ratio regression is an engine check of that ranking and of the
previously ignored hiding input.

No non-oil reflectance dataset was imported. The reference inventory in the
calibration research distinguishes gum-arabic, powder and other measured
stand-ins from actual oil films; those sources do not provide new oil-film
coefficients here. Thickness predictions for RGB-only tubes remain estimates.
