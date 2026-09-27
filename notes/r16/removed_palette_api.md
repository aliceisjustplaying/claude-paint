# Removed palette API compile checks

Developer-only regression cases, omitted from painter exports.

Color matching is not part of the physical paint API.

No automatic `Palette::mix`:
```compile_fail,E0599
use paint::Palette;
Palette::smalt_box().mix([0.5; 3]);
```

No automatic `Palette::aim`:
```compile_fail,E0599
use paint::Palette;
Palette::smalt_box().aim([0.5; 3], [0.2; 3], 0.3, 1.0);
```

No automatic `Palette::aim_for`:
```compile_fail,E0599
use paint::Palette;
Palette::smalt_box().aim_for([0.5; 3], [0.2; 3], 0.3, 1.0, unsafe { std::mem::zeroed() });
```

No automatic `Palette::paint`:
```compile_fail,E0599
use paint::Palette;
Palette::smalt_box().paint([0.5; 3], 0.3);
```

No automatic `Palette::paint_for`:
```compile_fail,E0599
use paint::Palette;
Palette::smalt_box().paint_for([0.5; 3], [0.2; 3], 0.3, 1.0);
```

No automatic `Canvas::aim`:
```compile_fail,E0599
use paint::{Canvas, Palette};
Canvas::new(10, 1.0, [1.0; 3]).aim(&Palette::smalt_box(), [0.5; 3], (5.0, 5.0), 1.0, 0.3, 1.0);
```

Color matching is not part of the physical paint API.

No automatic `Paint::aimed`:
```compile_fail,E0599
use paint::Paint;
Paint::aimed([0.5; 3], [0.2; 3], 1.0, 0.5, 0.5);
```

No automatic `Paint::tint`:
```compile_fail,E0599
use paint::Paint;
Paint::tint([0.5; 3], 0.5, 0.5);
```

No automatic `Paint::glaze`:
```compile_fail,E0599
use paint::Paint;
Paint::glaze([0.5; 3]);
```

Spectra are supplied directly or converted from RGB, not fitted to targets:
```compile_fail,E0425
use paint::spectral;
spectral::fit_shape([0.5; 3], &[0.5; spectral::N]);
```

