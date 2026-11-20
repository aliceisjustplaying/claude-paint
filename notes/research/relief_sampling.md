# Sampling the support's relief on the pixel grid (`--@ relief-prefilter`)

## The fault

A canvas of 16 × 15 threads/cm painted at 2400 px on 92 cm has pixels of
0.383 mm. The warp threads come every 0.625 mm, 0.61 cycles a pixel, past
the grid's Nyquist limit of 0.5. The support was laid by reading the weave
(`surface::linen_um`) at 2 × 2 points a pixel and averaging them, which is
close to a box over the pixel. A box passes 64% of a ridge at Nyquist and
50% at 0.6 cycles a pixel, and what passes folds back to 1 − 0.61 = 0.39
cycles a pixel (and the weft to 0.43). Thin paint settles into that folded
relief, and the picture shows it as a regular checker of about 2.5 pixels:
`the-garden-at-marly.png`, x 1200–1600, y 350–670.

Measured on the weave alone (`prefiltered_weave_has_no_aliased_peak`), the
pixels read that way hold about 100–230 times the power the weave itself has
at the folded frequency, for 14, 16 and 20 threads/cm. On the crimped weave
that replaced it (notes/research/weave_geometry.md) they hold 113, 132 and
24 times, and the footprint leaves 0.009–0.013.

## The footprint

A grid samples correctly only what it can carry, below 0.5 cycles a pixel
(Shannon). The ideal pre-filter is a sinc. A finite one is windowed:

- the footprint is a sinc cut at 0.35 cycles a pixel, windowed to ±6 pixels
  by a Kaiser window with β = 6 (J. F. Kaiser and R. W. Schafer, "On the use
  of the I0-sinh window for spectrum analysis", IEEE Trans. ASSP 28(1),
  1980; A. V. Oppenheim and R. W. Schafer, *Discrete-Time Signal
  Processing*, §7.5, the Kaiser window design);
- its gain is 0.99–1.0 up to 0.25 cycles a pixel, 0.5 at 0.35, 0.003 at 0.5
  and below 0.001 past it; its weights sum to one, so the mean is kept
  exactly; the weights go to −16% of the center's beside it, so a sharp step
  rings by a few percent of the step (the support's relief has none at the
  pixel scale);
- a Gaussian was tried first: one wide enough to stop a thread just past
  Nyquist (14 threads/cm folds to 0.46) also blurs everything the pixels
  resolve, and at σ = 0.6 px the Marly case still kept 28 times the weave's
  power at the folded frequency.

The cut, reach and β say how the render is sampled, not what the canvas is:
no physics or measurement sets them. They are the textbook choice for
anti-aliasing a grid, estimates, checked by the tests.

## Reading the weave finely enough

The footprint needs the weave read at points finer than the pixels. The
weave is the threads' tops on their crimped paths (`weave.rs`,
notes/research/weave_geometry.md): continuous along each thread, but
dropping at a thread's edge where it lies over a lower thread or a hole.
So the relief has harmonics that decay slowly, and a regular grid of
samples folds them back as a faint pattern that beats with the weave. One
point at random in each cell of the grid (stratified sampling) turns that
into noise. Measured on 160 × 160 pixels of the Marly weave, against the
footprint of the weave read 48 times a thread (the averaged relief varies
by 23 µm rms):

| samples a thread | stratified, rms error |
|---|---|
| 4  | 5.9 µm |
| 8  | 2.5 µm |
| 12 | 1.1 µm |
| 16 | 0.8 µm |

(The first weave, `linen_um`, stepped where a weft passed from under one
warp to over the next. Its averaged relief varied by 6.4 µm, read with
1.5 µm of error at 8 a thread.)

The easel reads 8 a thread, stratified, at most 8 a pixel each way. Each
point needs the thicknesses of the four crossings round it, read once for
the window into a table (`weave::CrossTable`). A point then costs about
1.2 times what `linen_um` did, so the linen of the full Marly canvas costs
about 1.2 times its earlier 12 CPU-seconds (scaled from the per-point
times, not timed on the canvas), about 2 for each knifed ground layer.
It is paid once, when the canvas is laid.

## Pixels wider than a thread

Where a pixel is at least as wide as the coarser thread, neither the
threads nor their over and under (a period of two threads) fall inside
the footprint's band. The pixel can hold only the weave's mean and how it
varies from thread to thread and slub to slub. The easel then lays
`weave::WeaveMean`: the weave's mean over a period of over and under
(read 48 × 48 points a thread), tabulated against the warp's and the
weft's thickness for the thicknesses the threads have there. At 1 mm a
pixel this has the weave's mean within 0.2% and its variation within 4%.
It follows the weave averaged pixel by pixel within 1.6 µm rms
(`coarse_pixels_hold_the_weaves_mean`), at a cost set by the pixels rather
than the canvas's area.

## What is still there

The plain weave's over and under at 16 × 15 threads/cm is a diagonal
pattern of two threads, 1.25 × 1.33 mm, about 3.3 pixels at 0.383 mm. The
pixels resolve it, so it stays, as a fine diagonal grain of the warp's
knuckles: its spectral peak at (0.306, 0.288) cycles a pixel is the
weave's own. Thicker threads and slubs (millimeters to centimeters) stay
at their scale (`prefiltered_weave_keeps_what_the_pixels_resolve`).

## Other surfaces

- A knifed ground's texture (`Canvas::prime_paint`, value noise on 0.3 and
  0.9 mm lattices) was read at one point a pixel; it is now the footprint
  average of the paint laid, read twice a cell of the finer lattice.
- A paper's felt (`paper::felt_um`, cells of 0.8 mm with rims a fifth of a
  cell) was read at each pixel's center; it is now its footprint average.
- A paper's fibers are laid as segments whose mass is shared between the
  four pixels round each point (a tent filter): random, not periodic, so
  they show no checker. Its pores (`micro`) are a statistic of the pores a
  pixel holds. A sanded paper's grit (an unmerged branch) is likewise a
  statistic of the grains a pixel holds. None of these change.
