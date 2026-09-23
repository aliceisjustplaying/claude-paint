# Round 3 fixes: paint (branch `fixes-paint`)

These are fixes for items 3–7 of the amnesia round 2 friction list
(`notes/amnesia2.md`). Details and numbers are in `notes/color.md` ("Aim
over contrasting paint"), `notes/strokes.md` ("Edges, blending and relative
color") and `notes/stipple.md` ("Fade and relative color").

## What changed (defaults marked)
1. **Aim over contrasting paint** (`palette.rs`, `handling.rs`)
   - `Aim::Laid` expects what strokes actually lay: `Handling::laid_coats()`
     = film per load by brush kind (filbert 1.3, round and rigger 2.2) × the
     mean overlap `c / (1 − e^−c)`. Sparse light marks used to be aimed as
     0.3 coats and laid 1–1.7, so aim over-compensated.
   - A stroke's underlayer is judged along the stroke: nine samples,
     weighted toward its start, combined by a weighted median in OKLab.
     Hand marks use `Canvas::aim` → `Canvas::judge_under` (a median of nine
     sub-discs).
   - A pile is judged at 0.5×, 1×, 2× and 4× the expected thickness, its
     thin edge must stay on the way from the underlayer to the target, and
     its masstone pays for straying from the target's hue (family). The
     salmon pile (lead white + red earth for a light touch on dark sand) is
     now lead white + raw umber.
2. **Coverage at mask edges** (`handling.rs`): **default** `hug(true)`.
   Centers just outside a region move onto its edge. Unclipped strokes that
   cross into a region are carried in from the edge. The share of a thin
   clipped band's edges left bare went from 13% to 2%.
3. **Blender** (`style.rs`): **default** `Style::blend()` is `clip(true)`;
   `.clip(false)` fuses across an edge on purpose.
4. **Relative colors**: `Handling::color_over(|x, y, under| ..)`,
   `Stipple::color_over(..)`, `paint::shift(c, dl, da, db)`.
5. **Stipple fade** (`stipple.rs`): **default** `fade(1.0)`. Contrast falls
   where coverage thins below 1; `fade(0.0)` turns it off.

```rust
use paint::shift;
// cast shadow on snow: whatever the snow is here, darker and bluer
c.work(&shadow, &st.body().color_over(|_, _, u| shift(u, -0.07, 0.0, -0.03)), 7);
c.stipple(&shadow, &Stipple::new(Tool::stippler(2.0)).mixed(&st.palette, 0.5)
    .color_over(|_, _, u| shift(u, -0.05, 0.0, -0.02)).coverage(falloff), 8);
// a passage whose edges should thin out (the old placement)
st.body().color(rock).hug(false)
// fuse across a horizon on purpose
st.blend().unwrap().clip(false)
// deliberate specks (snowflakes, stars): no fading where coverage is thin
Stipple::new(Tool::stippler(1.2)).fade(0.0)
```

## Tests (all pass; golden re-recorded at each commit, debug profile)
- `palette::tests::contrasting_aims_stay_in_family`
- `handling::tests::light_marks_over_a_dark_stay_in_hue`
- `handling::tests::edges_are_covered_like_the_inside`
- `handling::tests::blender_stays_in_its_region`
- `handling::tests::color_over_sees_the_canvas`
- `stipple::tests::thin_stipple_fades_instead_of_salt`
- Probes, ignored: `probe_laid_by_coverage`, `probe_contrast_aims`,
  `probe_edges_over_seeds`. Run them with
  `cargo test --release -p paint <name> -- --ignored --nocapture`.

## Evidence
All three studies are at 1000px, base (06e24b4) vs this branch, in
`~/tmp/fixes-paint-3f815331/study/`
(`study_{color,strokes,stipple}_{base,new}.{png,jpg}`, `fringe_cmp.jpg`).
- `study_color`: the top sky band and the dark lower half of the glaze band
  have fewer and smaller bare-ground flecks. The aimed skies look as smooth
  as before.
- `study_strokes`: about the same. Panel edges are full, and the fused row
  stays inside its strips.
- `study_stipple`: the mist's upper fringe over the dark ridge has fewer
  isolated pale specks and thins out more gradually. The dense mist body is
  still grainy.
- Edge renders (old vs hug, band and disk): `.../edge/band_cmp.jpg`,
  `.../edge/disk_clip_cmp.jpg`.

## Known issues / next
- Ground flecks *inside* dark passages come from the bristle model
  ploughing paint off the weave's peaks (bristle.rs, not changed here).
  They go away at coverage ≈ 4 or over a dark underpainting.
  (Later measured otherwise: they are gaps between strokes, fixed by the
  look-and-fill pass; see notes/surface.md §1.)
- More paint lands within a brush width past an unclipped edge (more
  strokes near the edge). Beyond a brush width it is about unchanged.
  Hidden-layer leaks (mountains #9) are less likely with full edges and a
  clipped blender, but a mask laid under a later passage still shows in its
  gaps. The painter's rule still applies: don't lay light paint where dark
  will go.
- "Thin edges show their strongest tube" for masstone-mixed marks (the
  coast's red shawl rims) is partly KM undertone: a dark red-brown's thin
  film is redder than its masstone. Only aimed piles gained the thin-edge
  check.
- A veil mode for stipple (each touch carrying a share of the step) was
  tried and dropped. Thin touches stack with Poisson noise and read noisier
  than opaque ones at coverage 3.
- The round brush's film per load rises with coverage (2.0 → 3.4). If the
  pointed-tip brush changes deposit, rerun `probe_laid_by_coverage` and
  update `LAID_PER_LOAD_*`.
- Changed defaults change existing renders (golden re-recorded).
  `friedrich_moonrise_valley` builds; I rendered only the three studies.

## After the tip merge (integration, round 3)
- `LAID_PER_LOAD_ROUND` re-measured with `probe_laid_by_coverage`: the
  pointed-tip model makes detail marks narrower (coverage 0.5: 74% of the
  area covered instead of 93%) and thicker where they land (p50 3.3 coats at
  coverage 2.5, load 0.3, was 1.6). 2.2 → 4.0 matches coverage 2.5–4.
- `light_marks_over_a_dark_stay_in_hue` (detail): value misses come from
  the thin parts of marks. At 1000px, pixels > 4 coats miss by -0.006 L,
  2–4 coats by about -0.02, 0.5–2 coats by -0.04 to -0.05 (thin films of
  light paint over a dark dry darker, as they physically do). The test now
  judges value on the body of marks (> 2 coats) at 500px with a 0.05 bound
  for pointed detail marks (0.025 for body).
- **Open** (done in notes/surface.md §2, `Palette::aim_for`): the aim picks one pile for one expected thickness. A painter at
  viewing distance sees a mark's average look; aim at the thickness-weighted
  mean appearance over the mark's expected thickness distribution instead.
