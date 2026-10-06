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

## Mixbox

The paint engine (`crates/paint`) mixes pigments with Mixbox 2.0.0, used
unmodified as the Rust crate `mixbox` (https://crates.io/crates/mixbox),
https://github.com/scrtwpns/mixbox, https://scrtwpns.com/mixbox. It
implements Šárka Sochorová and Ondřej Jamriška, "Practical Pigment Mixing
for Digital Painting", ACM Transactions on Graphics 40(6):234 (SIGGRAPH
Asia 2021), https://doi.org/10.1145/3478513.3480549.

```
Copyright (c) 2022, Secret Weapons. All rights reserved.
Mixbox is provided under the CC BY-NC 4.0 license for non-commercial use only.
If you want to obtain commercial license, please contact: mixbox@scrtwpns.com
```

License: Creative Commons Attribution-NonCommercial 4.0 International,
https://creativecommons.org/licenses/by-nc/4.0/. This project uses it
non-commercially.
