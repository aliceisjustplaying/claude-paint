# Bath lead-tin-yellow reflectance and tints

Anuradha R. Pallipurath, Jonathan M. Skelton, Paola Ricciardi and Stephen R.
Elliott (2016), *Data for “Estimation of semiconductor-like pigment
concentrations in paint mixtures and their differentiation from paint layers
using first-derivative reflectance spectra”*, University of Bath, version 1.
https://doi.org/10.15125/BATH-00183

The data record licenses each file Creative Commons Attribution 4.0:
https://researchdata.bath.ac.uk/183/ . The full legal text is in `LICENSE`.
University of Bath is the record's rights holder. The source README also
requests citation of the associated paper, *Talanta* 154, 63–72 (2016),
https://doi.org/10.1016/j.talanta.2016.03.052 .

`raw/` contains an unchanged subset of twenty-four spectra from
https://researchdata.bath.ac.uk/183/1/Reflectance-Spectra.tar.gz .
`README.rtf` is unchanged from
https://researchdata.bath.ac.uk/183/3/README.rtf . File hashes are in
`SHA256SUMS.json`. The complete archive and the paper are not redistributed.

The generator averages the three spots for each composition, resamples the
visible spectrum and derives explicitly modeled oil coefficients. Those
transformations are project work, not results endorsed by the source authors.
The raw data and derived numerical tables retain CC BY 4.0 attribution;
the repository's MIT code license does not replace this data license.

The specimens are gum-arabic paints brushed onto microscope glass. LTY is
type-I lead-tin yellow, LW is lead white and RL is red lead. Both the white
tint ladder and the red-lead chromatic mixture ladder are retained with
their available pure endpoints. `LW-LTY_66.7-33.3` means 66.7%
lead white and 33.3% yellow by dry pigment mass, before binder addition.
Three spectra are different spots on one film, not independent preparations.
There is no pure-white spectrum, film thickness or measured backing in this
subset. None of these measurements supplies oil-film K and S.
