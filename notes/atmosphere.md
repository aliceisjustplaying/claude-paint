# Atmosphere: structure at the scale of the sky

Branch `atmosphere`. This stream answers the user's verdict on amnesia round 2
(`notes/amnesia2.md`): "the sky still looks a little too neat", "the
separation between the sea and the sky is too neat", "the mountains read as
waves."

The diagnosis: we had added entropy at the scale of single marks (stipple
grain, stroke curvature), but not at the scale of the picture's structure:

- Every sky was a smooth vertical ramp from zenith to horizon, often with a
  centered glow.
- Horizons were ruled lines.
- Receding ranges were near-parallel silhouettes of similar amplitude and
  even spacing, so they read as waves.

This stream adds physics and tools, not looks. The painter still decides
every color and mark.

## What changed

- **`crates/paint/src/noise.rs`** (additions only; `Fbm` is unchanged,
  still `Copy`, and its test still checks it against the noise crate):
  - `Octaves` with `Fold::{Plain, Ridged, Billow}`, in 2-D and 3-D, and a
    level of detail (`get_lod(x, y, footprint)`, `get3(p, footprint)`) that
    fades out octaves finer than a sample's footprint. This stops far clouds
    and far crests from aliasing.
  - `Warp`: domain warping, with `twice()` for folds within folds, following
    Quilez (<https://iquilezles.org/articles/warp/>).
  - `Aniso`: stretching along a direction (wind, bedding).
  - `Worley`: cellular noise returning `f1`, `f2`, `edge()`, a stable cell
    `id`/`rand()` and the feature point. A feature lies anywhere in its
    cell, so the search runs past the 3×3 block (to 7×7), skipping cells
    whose nearest corner is already farther than F2. The 3×3 search missed
    nearer features two cells away (tested against brute force:
    `worley_finds_the_true_nearest_features`).
  - Hand irregularity: `uneven(n, lo, hi, irregular, clump, seed)` gives
    lognormal gaps grouped into clumps. `vary(v, amount, i, seed)` and
    `rand01` are stable per-key randomness.
  - Tests: `toolkit_is_copy_and_ranged` and `uneven_is_uneven_and_deterministic`.
- **`crates/paint/src/atmos.rs`** (new; `pub mod atmos`, re-exported from
  `lib.rs` as `Cloud, Clouds, Haze, Ranges, Sky, SkyField`).
  - **`Sky`**: single scattering by air (Rayleigh) and haze (Mie,
    Cornette–Shanks phase), with ozone absorption, integrated along each line
    of sight in a spherical atmosphere over a spherical Earth.
    - It follows Nishita et al. 1993, "Display of the Earth taking into
      account atmospheric scattering"
      (<https://dl.acm.org/doi/10.1145/166117.166140>). The constants are
      Bruneton's (<https://ebruneton.github.io/precomputed_atmospheric_scattering/>):
      Rayleigh 5.802/13.558/33.1e-6 per m, Mie 3.996e-6, ozone
      0.650/1.881/0.085e-6.
    - I resample Rayleigh by λ⁻⁴, and ozone by the Chappuis band's shape, at
      the dominant wavelengths of the sRGB primaries (611/549/464 nm). The
      constants are in `atmos.rs` (`const RAYLEIGH`, `const OZONE`). At
      680/440 nm a 5° sun came out `[0.53, 0.24, 0.06]` and the twilight
      zenith came out purple. After the change they are
      `[0.32, 0.24, 0.10]` and a deep blue (`rgb/l 0.74 0.98 1.96`, from the
      ignored `probe_twilight` test).
    - The twilight blue comes from ozone (Hulburt 1953,
      <https://www.patarnott.com/atms411/pdf/class2018/TwighlightColors.pdf>).
    - The Earth's shadow and the Belt of Venus aren't special-cased. A
      sample the Earth hides from the sun gets no direct light. The belt is
      reddened sunlight on the air above the shadow (Atmospheric Optics,
      <https://www.atoptics.org.uk/fz527.htm>,
      <https://www.atoptics.org.uk/fza60.htm>).
    - Single scattering alone leaves the shadow black, so a *fill* scatters
      the dome's own mean light (`Sky::dome`) isotropically. This lights the
      shadow blue-gray, as skylight does. `dome` is cached with the
      parameters it was computed from (sun, haze, g, uneven, alt, layers),
      so changing a public field such as `sky.sun` after sampling gives the
      same sky as a fresh one (tested: `sky_dome_follows_field_changes`).
    - Knobs:
      - `haze(k)`;
      - `uneven(amount, period_m, seed)`: aerosol density wanders from place
        to place, so the glow and the horizon brighten unevenly;
      - `layer(alt, thick, density, uneven, seed)`: a haze layer;
      - `overcast(0..1)`: a blend toward the CIE standard overcast sky,
        L/Lz = (1 + 2 sin e)/3, after Moon & Spencer 1942
        (<https://journals.sagepub.com/doi/full/10.1177/1477153517740611>);
      - `altitude(m)`, `fill`.
    - Also `sunlight()`/`sunlight_at(p)` (the sun's color reaching a
      point: reddened, or black in the Earth's shadow) and
      `transmittance(d, dist)`.
  - **`SkyField`**: the sky over a `World`'s camera, on a grid (the sky is
    smooth, and a cell of 6–8 units is plenty).
    - `at(x, y)` gives paint's range (linear RGB, 0–0.86); below the horizon
      it gives the sky that calm water mirrors there.
    - `value`, `radiance`, `airlight(x)` (the haze color for far things at
      canvas x), `dir`, `paint(rad)`.
    - The exposure is automatic: the 97th percentile of the sky maps to a
      light paint, and brighter light rolls off and bleaches. Adjust it with
      `exposure(k)`; `balance(light, amount)` adapts the eye to the day's
      light.
  - **`Cloud`** volumes in world meters:
    - `cumulus(x, z, base, width, height, seed)`: a flat base and a dome of
      billowed towers;
    - `bank(x0, x1, z, depth, base, top, seed)`: a long heaped mass whose top
      swells and falls, with ragged ends;
    - `stratus(base, thick, cover, seed)`: a warped, wind-stretched deck.
    - Tuning: `density`, `soft` (a hard cauliflower edge or a fibrous one),
      `wind(angle, stretch)`, `breaks(period, fraction)` (holes in a fraction
      of Worley cells, each its own size), `heap`, `reach`.
  - **`Clouds::field(&sky_field, &world, cell)`** marches every view ray
    through the volumes and lights them with the one sun. The march covers
    only the spans where the ray is inside some cloud's box, each piece
    with the finest step of the clouds over it (64 steps across each
    cloud's own span), and skips the gaps between them. It used to spread
    64 samples over the whole range from the nearest to the farthest cloud,
    so a distant bank could make a near heap fall between samples and
    vanish (tested: `a_distant_cloud_does_not_hide_a_near_one`). In
    study_sky the top panel's big heap now shows its flat dark base and
    paler sunlit towers where it was a rounded dark lump. The march:
    - Beer–Lambert toward the sun (the lit edge);
    - a two-lobe Henyey–Greenstein phase with multiple-scattering octaves
      (Wrenninge et al. 2013, "Oz: The Great and Volumetric") for the silver
      lining against the sun;
    - two-stream diffuse transmission, 1/(1 + 0.75(1 − g)τ), for the light
      that reaches a deck's underside and a heap's belly;
    - sky light from above, shaded by the cloud over the sample;
    - bounce from the ground;
    - the air in front of the cloud;
    - the Earth's shadow and reddening for a sun near or below the horizon
      (high clouds lit pink from below after sunset).
    - It returns a `CloudField`:
      - `alpha` (how much sky it hides);
      - `lit` (0 in the shadowed belly, 1 on the lit edge);
      - `glow` (light sent to the eye, phase included: strong on thin edges
        against the sun);
      - `ambient`, `dist`;
      - `soft(x, y)` (how many units the edge takes to go from clear to
        solid there);
      - `color` (the cloud over the sky, as paint), `cloud_color`;
      - `mask(frame, |p| ..)`.
  - **`Ranges::new(near_m, far_m, count, seed)`** with `heights`,
    `irregular`, `oblique` and `kinds`. `build(&world)` returns
    `Vec<RangeLayer>`, nearest first.
    - Distances are spaced unevenly and clumped in log distance (`uneven`).
    - Each range runs at its own slant (its distance, and so its haze, change
      along it).
    - Some ranges enter from one side and end.
    - Each is built from masses of different kinds (`Silhouette::{Peak,
      Dome, Plateau, Saddle, Cliff}`), sized from their height, placed
      unevenly, leaning and merged by a soft max (notches where two meet).
    - The flanks are warped, and there is ridged crest detail at two scales
      with a level of detail. The Earth's curvature lowers far ranges
      (d²/2R, refraction k = 0.13).
    - `regular(&world)` builds the old way for comparison: even spacing,
      parallel to the picture, similar peaks.
    - `RangeLayer` gives `crest(&world, x)` (canvas y), `height_at`,
      `z_at`, `haze(&world, &Haze, x, y)` and `ridge(&world, x0, x1, depth,
      gully_m)` (a `form::Ridge` whose gullies are sized in meters at the
      range's distance).
  - **`Haze::new(visibility_m)`**, with `height(scale_m)` and
    `mist(top_m, density, uneven_m, seed)`.
    - `loss(eye, dist, h, x)` integrates the exponential haze profile along
      the line of sight (Koschmieder β = 3.912/V). The foot of a range is
      hazier than its crest.
    - Valley mist has an uneven top.
  - Tests: `day_sky_structure` (horizon brighter and paler than the zenith,
    more than 2× brighter toward the sun than away, low sun reddened),
    `twilight_structure` (the belt is brighter and warmer than the Earth's
    shadow, the afterglow is warm, sunlight at 15 km but none at the
    ground), `sky_field_maps_to_paint` (in range, and a row across the sky
    varies by more than 15 %), `clouds_have_lit_edges_and_shadowed_bellies`,
    `ranges_are_uneven`.
- **`paintings/src/bin/study_sky.rs`** (new): the evidence (below). No other
  files were touched: `scene.rs`, `form.rs` and the other streams' files are
  unchanged.

## API, briefly

```rust
use paint::atmos::{Cloud, Clouds, Haze, Ranges, Sky, SkyField};
use paint::noise::{uneven, Octaves, Warp, Worley};

let w = World::new([0.0, 0.0, 1000.0, h], h * 0.6, 30.0).fov(1000.0, 58.0).sun(Sun::deg(-19.0, 4.5));
// the sky for that sun: uneven haze, a haze layer at 900 m
let sky = SkyField::new(Sky::new(w.sun).haze(2.6).uneven(0.6, 30_000.0, 11).layer(900.0, 500.0, 1.6, 0.8, 3), &w, 6.0);
let sky = sky.balance(sky.sky.sunlight(), 0.25);            // the eye half adapted to the low sun
// clouds in the world, lit by the same sun
let cf = Clouds::new(vec![
    Cloud::bank(-2_000.0, 30_000.0, 38_000.0, 9_000.0, 700.0, 2_600.0, 4),
    Cloud::cumulus(3_600.0, 10_000.0, 1_100.0, 2_600.0, 1_300.0, 8),
    Cloud::stratus(1_300.0, 700.0, 0.97, 31).wind(-0.5, 1.6).breaks(14_000.0, 0.22),
]).field(&sky, &w, 2.0);
let sky_col = |x, y| cf.color(&sky, x, y);                 // sky with clouds, as paint
let bellies = cf.mask(c.frame(), |p| smoothstep(0.5, 0.9, p.alpha) * (1.0 - p.lit));
let lit_edges = cf.mask(c.frame(), |p| smoothstep(0.4, 0.9, p.alpha) * smoothstep(0.35, 0.8, p.lit));
// the sea mirrors sf.at(x, y) below the horizon; far water fades to the airlight
let air = Haze::new(28_000.0).height(900.0).mist(140.0, 6.0, 90.0, 7);
let sea_far = |x, y| mix(sea(x, y), sky.airlight(x), air.loss(w.eye, z_of(y), 0.0, 0.0), Mix::Light);
// ranges: uneven, oblique, varied; one Ridge each for the painter's form
for l in Ranges::new(4_000.0, 50_000.0, 5, 17).heights(220.0, 2600.0).build(&w) {
    let ridge = l.ridge(&w, 0.0, 1000.0, 200.0, 260.0);                 // form::Ridge
    let hazed = |x, y, stone| mix(stone, sky.airlight(x), l.haze(&w, &air, x, y), Mix::Light);
}
// irregular spacing for anything repeated
let posts = uneven(9, 120.0, 860.0, 0.6, 0.4, 5);
```

## Evidence

`cargo paint study_sky` (~60–75 s at 1000px on a busy machine) paints one
landscape (the sea from a 30 m cliff top, with ranges built by `Ranges`
running out into it) under three skies, plus a panel of ranges old vs new.
Everything is painted with brushes and stipple:

- a broad sky lay-in fused with the badger;
- a stipple pass over the sky;
- cloud bodies, the lit edges in small sable strokes, and thin cloud in
  stipple, with soft edges fused (by `soft`);
- the sea in long level strokes, fused, mirroring the sky with Fresnel and
  fading to the airlight with distance, and far water stippled;
- ranges from a `Form` of their `Ridge`s lit by `World::light()`: near ones
  stroked down the fall lines with lit spurs, far ones laid thin and
  stippled, all hazed by `RangeLayer::haze`.

`--fields` shows the engine's fields unpainted (a diagnostic, ~15–25 s).

- `notes/atmosphere/study_sky.jpg`: the 1000px sheet (`out/study_sky.png`).
- `notes/atmosphere/study_sky_3200_crop.jpg`: `-- --full --crop 0,0,520,300`,
  the left half of the morning panel at 3200px (~55 s).

Judged against the user's words, honestly:

- **"The sky still looks a little too neat": much less, but not solved.**
  - No sky is a vertical ramp any more:
    - Morning: the glow sits left of center, falls off faster upward than
      sideways and is heavier on one side; the right half of the sky is
      darker and grayer.
    - Overcast: a gray deck with a break across it.
    - Twilight: blue overhead, through yellow to an orange glow low over
      the sun's place, with high clouds lit pink from below and gray-violet
      ones in shadow.
    - All of this comes from the physics, not from a painted gradient.
  - What still reads neat or wrong:
    - The morning cumulus is a dark, hard blob that looks stuck on at
      3200px. That's physically right against a low sun, but the painting
      pass needs a softer, lit crown and less exposure contrast.
    - The overcast break's lit edges are long, thin, level white slits.
      Perspective foreshortens a hole in a deck, but the slits are too
      regular.
    - The painted sky still has a uniform touch everywhere; the handling
      doesn't yet change between cloud and clear sky.
- **"The separation between the sea and the sky is too neat": fixed in
  haze, not everywhere.**
  - In the morning crop the horizon is lost in the glow. The far water
    fades into the airlight, which is the sky just above the horizon there,
    so sea and sky meet without a line. Uneven haze makes the join brighter
    in some places than others.
  - In the overcast and twilight panels the foot of the ranges and the sea
    still meet at a straight level line (`Ridge::base` at one y). Under the
    twilight glow the sea keeps a hard-edged orange band.
- **"The mountains read as waves": the old panel (bottom left) still does;
  the new one (bottom right) doesn't, but it isn't a good range yet.**
  - The new panel has no rhythm: a long massif with a notch, a plateau, a
    gap and a lone peak at the right, with haze that steps unevenly because
    the ranges run obliquely at uneven distances.
  - But the silhouettes are still geometric (a peak is a cone with teeth,
    a plateau is a trapezoid), the fine crest detail reads as sawtooth at
    1000px, and only two or three layers read before the haze takes them.

## Known issues and next steps

- **Cloud painting, not the cloud physics, is the weak part.** A painter
  should paint the crown from `lit`/`glow`, the belly from `ambient`, and
  lose the edges where `soft` is large. The study's passes are simple. Next:
  a cloud passage in a real painting.
- **Contre-jour exposure.** The auto exposure takes the 97th percentile of
  the sky, so near a low sun everything else is dark. `exposure(k)` and
  `balance` help. A painter's exposure (the glow bleached, the shadows
  lifted) may want a local tone curve.
- **Cost.** The sky is cheap (a 6-unit grid, a fraction of a second). Cloud
  fields march 64 steps per cloud crossed, per 2-unit cell, with light and
  sky marches: ~2–10 s per panel at 1000px. Use a coarser `cell` for large decks.
- **Ranges.**
  - Masses are analytic profiles plus noise. Next: carve masses with
    `Worley` ridges (arêtes and cirques), vary crest detail per mass kind
    (plateaus flat, cliffs with a sheer drop cut into the face), and draw
    the foot as a shore or valley floor with its own uneven line instead of
    `Ridge::base`.
  - `RangeLayer::haze` treats the face as standing at the crest's distance.
- **Sky model limits.** It uses single scattering plus an isotropic fill,
  not full multiple scattering, so deep twilight (sun below −8°) is too
  dark. There's no sun disk, no crepuscular rays and no cloud shadows on
  the haze.
- **Easel bindings** for `Sky`, `SkyField`, `Clouds` and `Ranges` still need
  writing.

## Commits

- `139c7bb` noise: toolkit (Octaves plain/ridged/billow 2-D and 3-D with
  level of detail, Warp, Aniso, Worley, uneven spacing with clumping, vary)
- `580d9f3` atmos: physical sky, SkyField, cloud volumes, receding Ranges; tests
- later commits: sRGB-dominant wavelengths, two-stream cloud diffuse light,
  masses sized by height, warped flanks, breaks in a fraction of cells,
  two-scale crest detail; study_sky; previews
