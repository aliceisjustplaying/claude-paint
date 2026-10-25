# The skin: oil films that cure from the top down, and wrinkle

Engine 9's material revision 1 (`--@ material 1` in a log's head,
`paint::Canvas::with_material`) keeps every film that has baked into the
dry picture as a *packet* (`crates/paint/src/skin.rs`): how thick it is,
how deep its cure front has reached, the compression its skin has built
and how far wrinkling has moved its soft bulk. Where that skin lies on
paint that is still soft, it wrinkles. Nothing else changes: the surface
drying `drying(x, y)` reports (open, setting, tacky, dry) is the same and
no stroke lays differently; the relief is what moves, and paint laid later
lands on it.

It is opt-in: a new painting's log doesn't have the line, so only a log
whose head names it paints with it. A log without it, a new painting's
included, paints the single-cure film engine 9 was released with, bit for
bit, and engines 1 to 8 are untouched (`skin::tests::legacy_material_keeps_no_packets`; the gate's
replay checks are the witnesses).

## What the paint does

The physics and its sources are in the module doc of `skin.rs`. In short:

- **The front.** Oxygen diffuses through the cured skin to the soft paint
  under it and is consumed there (Deal–Grove): the front slows as the skin
  thickens, and a film laid over it slows it further. Thin films cure
  through in days; a 1 mm one keeps soft paint under its skin for months,
  as Golden's drawdowns and disks do.
- **The compression.** Skin that has just cured keeps oxidizing and
  expands a little (`EXPAND`, scaled by the paint's binder share), held by
  the film around it: a compressive membrane force that builds over days.
  Material that cures later joins stress-free.
- **The wrinkle.** The skin is a plate on a gel (the soft bulk, with a
  modulus from the paint's yield stress). Past a critical compression it
  buckles at the wavelength that needs the least (Allen 1969; Huang and
  Suo), and at every hour of the painting's clock it sits at its
  equilibrium: the slopes it has made take up the compression past the
  threshold (von Kármán). The threshold is a strain, about 0.1% on deep
  medium-rich paint and 4% on stiff tube paint, against the 1.35% a skin
  reaches: medium-rich paint wrinkles, tube paint straight from the tube
  holds, as Ranquet et al. (2023) saw on their 58–75 Pa and 3200 Pa films.
  That is the onset on deep bulk (a skin's wavelength or more of soft
  paint under it, 0.4 mm and up). Under a thinner bulk the gel is
  yielded by the first micrometer of wrinkle (a thin layer shears
  `6A / qH²`), so the paste's hold is its yield stress as a plug flow
  from trough to crest, measured against the slopes the skin has at its
  wavelength (brush marks and wrinkle so far, `Packet::slope`): a
  200 µm medium-rich knife slab wrinkles at 2–4 mm, a blotted one of ten
  times the yield stress holds at that thickness, and a brush film cures
  through before its skin can push (under about 100 µm). Treating that
  bulk as an elastic layer, as the first cut did, held the 200 µm slab
  flat with a 1/H³ stiffness: 22.8 N/m against 6.7 N/m of compression at
  day 20, at a wavelength of 1 mm.
  The pattern is the relief the skin cured over, band-passed at its own
  wavelength, so wrinkles lie along the brush marks. A wavelength between
  two of the drawn bands is drawn as a mix of both (by its logarithm), so
  a skin whose wavelength changes from place to place has no seam where
  it passes from one band to the next. A skin can't ripple two ways at
  one place without stretching, so each place keeps one way: the pattern
  settles as an effective Swift–Hohenberg equation has wrinkling skins
  settle (Stoop et al. 2015), into a labyrinth of ridges a wavelength
  apart that wind, branch and turn, and grow longer the longer they
  settle, leaning along the marks the skin cured over. The settling uses
  Gaussians, which favor no direction (two boxes favor the grid's two,
  and a labyrinth settled through them comes out a maze of right
  angles), on a grid as coarse as the band allows. Each place settles
  at its own wavelength, not its band's, so the spacing of the ridges
  follows the skin's thickness and its paste from place to place
  (settled at the band's alone, a slab whose wavelength varies by half
  either way came out ridged at one spacing throughout). The skin is
  compressed the same every way, and a ripple relieves it only across
  itself, so a skin rippled across a knife's drag is still compressed
  along it, and its ridges bend, as a herringbone's do (Chen and
  Hutchinson 2004; Audoly and Boudaoud 2008), until their slopes are as
  steep both ways. They bend where the paint under the skin lies higher
  or lower over a few wavelengths (`BEND`), each place moving across its
  ridges by as much as tilts them to make up the share of the slopes
  that runs along them; a settled labyrinth left straight runs in long
  rows across the drag joined by short jogs at right angles. (A ridge's
  axis has no sign, so every place moves the same way round across its
  ridges; moving each along its own axis tore the ridges wherever that
  axis turned past the grid's.) The bulk moves sideways under the skin
  and the total paint is conserved.
- **The folds.** The settle says where the ridges run, not how tall they
  stand: it brings every crest to the same height (measured on the fat
  slab, the tallest tenth of crests over the lowest tenth goes from 3–8
  before it to 1.05 after it). The paste under the skin thins as it
  flows (Herschel–Bulkley, flow index about one half, `FLOW_INDEX`), and
  the plate presses on it in proportion to the crest's height, so a crest
  taller than those around it grows faster than in proportion and takes
  the compression first, the way a skin past its onset gathers into folds
  while the wrinkles beside them relax (Pocivavsek et al. 2008; Brau et
  al. 2011). Each ridge grows as the square of how tall the relief along
  it already stands (averaged a wavelength each way along the ridge: the
  skin can't be tall at one point of a ridge and low a wavelength along
  without bending), against that of four wavelength windows around
  (`FOLD`), and its slopes are measured and spent over that much skin, so
  a fold relieves the skin it gathers from. Left alone the slab came out
  as one even texture; with the folds it has ridged patches and flatter
  ground between.
- **The give.** A yielded paste gives way point by point, where the skin
  presses it past its yield stress, and the skin presses in proportion
  to the wrinkle's height there: the paste gives under the highest
  crests and deepest troughs first and holds between them. The skin makes
  only the slopes the paste lets it make, and the compression it can't
  spend drives the paste harder at the next tick, so a few strong folds
  take the compression first and quieter skin lies between them.
- **The unevenness.** No film is the same everywhere. The medium mixed
  into a paint is spread through its film with a standard deviation of
  three tenths of the medium (`OIL_SPREAD`): a knife-thinned pile is
  streaked fatter and leaner, and a fatter spot is a softer paste with
  more binder to expand. Its drier is spread with a standard deviation of
  0.5 in the log of the drying rate (`DRIER_SPREAD`), so the skin is
  thicker in some places than others. Both vary over patches of 16 mm down
  to 0.5 mm and are seeded per film by its gel time: paint laid a day
  later was mixed afresh. Tube paint without medium keeps its oil even;
  only its drier varies. On the 198 µm fat knife slab of the demo this
  spreads the yield stress from 7 to 46 Pa (10th to 90th percentile),
  around its 17 Pa. Both spreads are estimates, set by eye against
  photographs of wrinkled oil: real wrinkles bend and branch, and strong
  folds lie among quieter skin.
- **The clock.** Fronts, skins and wrinkles step every six hours of the
  painting's clock (`TICK_MIN`), never at a wait's own ends, so a wait
  split any way leaves the same relief
  (`wrinkles_are_the_same_however_the_wait_is_split`).

A 600 µm medium-rich film ends with about 0.25 rms slope (some 100 µm rms
of relief at the brush marks' spacing); a 1 mm one, whose skin forms
faster, 0.15. A fast lean cover over slow fat paint wrinkles about as much
as the fat paint alone would: the cover thickens the plate, which asks
for more compression, about as much as the extra skin supplies.

## Where it lives

- `crates/paint/src/skin.rs`: the packets, the front, the compression and
  the wrinkle; `Canvas::with_material`, `skin_bake` and `skin_advance`.
- `crates/paint/src/drying.rs`: `bake` hands each baked film to
  `skin_bake`; `wait`, `wait_on_grid` and `dry` call `skin_advance` over
  the clock span they cover.
- `crates/paint/src/checkpoint.rs`: a material-1 canvas writes `PAINTC22`:
  the base format it would have written (19, 20 or 21) and then a skin
  section (the material, the last tick, each pixel's packets). Material-0
  canvases write what they wrote before.
- `crates/paint/src/palette.rs`, `style.rs`: the box carries `material`
  and a `Style` sets it on the canvas after the ground is laid, so the
  ground is a cured floor, not a film.
- `crates/easel/src/session.rs`: `MATERIAL_MARK`, `logged_material` and
  `box_for` (a new painting gets material 0; material 1 comes only from a
  log's line, which needs engine 9); `save.rs` restores it from the canvas.

## Cost

The skin's six-hourly tick is a pass over the live packets, and a wrinkle
pass (box blurs over the dirty box at each wavelength in use) only when a
wavelength of skin asks for `GROW` of slope or more, or a skin that could
wrinkle has its slopes to measure. The 90-day demo of a 440 mm canvas at
2400 px (two knife slabs and eight brush strokes) takes 784 s against
41 s at material 0, on four cores. In a live session at 2400 px (a
studio's log replayed at this material, four cores), a call that waits 2
to 3 days takes 17 to 30 s and one that waits 4 days 40 to 44 s, against
3 to 5 s at material 0; paint calls are unchanged (a median of 8.2 s
against 8.1 s). Most of a wait call is the twice-daily pass over the
whole skin at its finest wavelength.
