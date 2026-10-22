# Third-party notices

## spectral.js

`crates/paint/src/spectral.rs` ports code and data (the RGB→reflectance
basis spectra, the D65-weighted CIE color matching functions, the
Kubelka–Munk K/S conversions and the `mix` function) from spectral.js 3.0,
https://github.com/rvanwijnen/spectral.js, commit
bb2b05c9d1e65ae824d47e3b1cc17ea32c8ee68f.

```
MIT License

Copyright (c) 2025 Ronald van Wijnen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## StyLua

The studio's display-only Lua formatter bundles unmodified WebAssembly and
browser bindings from `@johnnymorganz/stylua` 2.5.2, published at
https://registry.npmjs.org/@johnnymorganz/stylua/-/stylua-2.5.2.tgz.
Source: https://github.com/JohnnyMorganz/StyLua/tree/v2.5.2.
License: MPL-2.0, included in `studio/vendor/stylua/LICENSE.md`.
These assets load locally only when the code view is opened.

## Independent pigment mixer

`crates/paint/src/mixer.rs` implements the pigment-concentration and RGB
residual method from Šárka Sochorová and Ondřej Jamriška, "Practical
Pigment Mixing for Digital Painting", ACM Transactions on Graphics
40(6):234 (2021), https://doi.org/10.1145/3478513.3480549.

The implementation, the derived K/S coefficients and the generated tables
are original project work under the root MIT license. They were not
copied from the authors' library, lookup tables or fitted coefficients.
The table generator uses the D65/CIE color conversion data from the MIT
spectral.js port credited above and the measured paint reflectance data
credited below. Scientific attribution is not a redistribution of the
paper or a claim of patent clearance.

## Revigo paint reflectance data

`scripts/pigment_data/revigo` holds, unchanged, the two data files of
https://github.com/fligt/revigo-spectra (commit f09ed72): reflectance
spectra of the oil paints reconstructed for the Revigo project by the
Cultural Heritage Agency of the Netherlands (RCE), from which
`scripts/build_pigment_tables.py` derives the mixer's primaries. The
repository dedicates its data and code to the public domain under CC0 1.0
Universal (https://creativecommons.org/publicdomain/zero/1.0/; the full
text is in `scripts/pigment_data/revigo/LICENSE`) and asks that
publications cite Geldof, Proaño Gaibor, Ligterink, Hendriks and Kirchner,
"Reconstructing Van Gogh's palette to determine the optical
characteristics of his paints", Heritage Science 6:17 (2018),
https://doi.org/10.1186/s40494-018-0181-6 (CC BY 4.0), which describes
the measurements. The article is cited, not redistributed.

## Other dependency and asset licenses

Rust dependencies declare MIT, Apache-2.0, BSD, ISC, Zlib, 0BSD or equivalent
commercial-use terms, sometimes as alternatives. Choose a permissive
alternative where the license expression offers one. The full resolved
inventory is in `notes/pigment-mixer-dependencies.json`, with license and
asset actions in `notes/pigment-mixer-licenses.md`.

The table generator uses pinned NumPy and SciPy as build tools, not engine
runtime dependencies. Their Linux wheels include BSD OpenBLAS/LAPACK,
the GCC runtime under its GPL runtime exception and LGPL libquadmath.
Preserve the complete wheel notices if distributing that tool environment;
the generated numerical tables do not contain those libraries.

The Python MCP stack can include certifi's Mozilla CA bundle (MPL-2.0).
Pillow wheels carry MIT-CMU and their bundled native-library notices;
FreeType offers its FTL alternative. Preserve the distributions' complete
notices when bundling them. Unpinned external painter packages and global
extensions are not a verified deployed dependency inventory.

FFmpeg and libx264 are external executables used by capture/export scripts,
not libraries linked into the Rust engine. Their own GPL/LGPL obligations
apply if distributing those tools. Invoking them does not add a
noncommercial condition to the engine.

The watercolor experiment separately credits its MIT Watercolor Lab port
and OFL-1.1 Figtree fonts in `experiments/watercolor/THIRD_PARTY_NOTICES.md`
and `experiments/watercolor/lab-app/ASSETS.md`. Those font files retain their
OFL notice. DejaVu Sans used by experimental sheet scripts is external;
its Bitstream Vera/DejaVu notices apply if bundling that font.

Technical image fixtures are project renders. Reference-library quotations,
museum links and research catalogs have source-specific rights, not a
blanket grant from this repository's code license. A source citation or an
`open_access_image` flag alone is not permission to redistribute an image.
