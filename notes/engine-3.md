# Engine 3: impasto, the knife, the relief look

Engine 3 is what a new painting is painted with (`--@ engine 3` in its log).
A log that names engine 1 or 2 replays exactly as before: every change below
is gated on the canvas's engine, or is a new option an old log never uses.
Checked: `paintings/lua/nympheas2.lua` (engine 2, 17 chunks) replays to a PNG
byte-identical to the one saved when it was painted.

## Why

A thick, stiff stroke dried into a smooth round tube. The leveling model
(surface.rs `settle_for`) froze relief only up to the paint's yield stress,
which `rheology` put at 5 to 300 Pa; stiff tube paint is above 1000 Pa and up
to about 3000 Pa (notes/research/oil_paint_physics.md). At 300 Pa the
furrows of a brush leveled flat, and only stroke-scale ridges stayed. The
painter never saw relief either: looks showed color only.

## What changed

- **Yield stress** (`surface.rs`, `rheology_at`): engine 3 spans 5 to
  3000 Pa (`5·600^stiff`; was `5·60^stiff`). Lead white from the tube
  (stiffness 0.8) now holds about 835 Pa.
- **Films bridge fine relief** (`settle_for`, `BRIDGE_UM` 40 µm): a thick
  film's top is shaped by the brush, not by the weave under it, so the
  weave's relief fades from its surface over some tens of µm of paint (it
  printed through even millimetre impasto before).
- **Clumping hair** (`bristle.rs` `exchange`, `Surf::clump`): in stiff paint
  the hairs gather into clumps, `hair · (1 + 6·stiff²)` across. A clump lays
  more paint and the gaps between clumps less (furrows along the stroke,
  averaging out across the brush), and a clump throws the paint each hair
  ploughs aside its own width (up to 3 px) instead of a sub-pixel hair's, so
  walls rise along the stroke's edges. Each hair still moves only its own
  share of paint.
- **Blotting** (`pile{..., blot=0..0.5}`): oil drawn out of the paint, a
  negative medium in `Mixture::paint`: stiffness × (1+blot)² (capped at 1),
  scattering × (1+blot).
- **The knife** (`Canvas::knife`, Lua `knife{width=}`): a flexible steel blade
  (it follows relief over `FLEX_MM` 4 mm and bridges finer hollows) resting on
  the dry surface's high points, standing off them by a gap that closes with
  pressure (300 µm × (1−p)^1.5). Wet paint above the blade is cut into its
  bead, or pressed out past its ends in ridges; below it, within the gap and
  `PRESS_IN_UM` (60 µm), the bead fills to the blade's level. Scraping keeps
  what it cuts. Knives are snapshotted with the brushes, so a failed chunk
  leaves them as they were.
- **The relief look** (`look --mode relief [--light az,el]`,
  `Canvas::seen_lit`): the dry relief plus the wet films (bridged as above)
  under a raking light (default from the upper left at 25°), with cast
  shadows marched along the light, a third of the light as ambient, and a
  sheen on wet paint. It changes nothing on the canvas. `relief()` (the
  finished picture's light) is unchanged.
- **Sketches**: a session whose name starts with `sketch` paints at 600 px
  instead of 2400 (about 16 times faster) for trying out a composition; its
  log replays at the same width.
- **Materials** (`Paint::solvent`, `Paint::oil`, two more entries in the wet
  paint's `Prop`, mixed by volume):
  - `pile{turps=0..0.9}`: turpentine makes paint flow on the brush (it
    doesn't clump) and evaporates as it is laid (`Surf::add`): the film is
    that much thinner, of the paint's own body.
  - `pile{oil="linseed"|"walnut"|"poppy"}`: drying ×1, 0.8, 0.6.
  - `canvas{ground={{..., absorbent=true}}}`: a chalk and glue ground holds
    `ABSORB_COATS` (0.6 coats) of oil it draws out of paint laid on it
    (`Surf::add`): the film loses that oil, gets stiffer and leaner. Dry
    paint seals it.
  - **Gloss** (`Canvas::gloss`): a baked film's gloss follows its oil
    (smoothstep 0.15..1.3), blended with the surface under it for thin
    films; varnish sets it to 1. Engine 3 shows a matte surface with the
    first-surface reflection it scatters back (`SURFACE_REFLECTANCE` 4%,
    `haze`) in looks and saved pictures. Checkpoint format 9.
  - Not modeled: yellowing, fading of lakes, color change of chromates.
- **The giverny and impressionist boxes**: see `notes/giverny-box.md` and
  `notes/impressionist-materials.md` (from analyses of the paintings).

## Not done

- Drying cracks in thick paint, and through-drying (a thick film's skin
  dries over a soft interior) are still not modeled.
- The relief light in `finish`/`relief()` still uses its fixed light; a
  finished painting is best shown with `look --mode relief` at a gentle
  elevation, or `relief()` once it is retuned for engine 3's thicker paint.
