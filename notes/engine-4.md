# Engine 4: impasto, the knife, the relief look

Engine 4 brought the changes below; a new painting is painted with engine 5
(`--@ engine 5` in its log), which adds the knife's tears (at the end).
A log that names engine 1, 2 or 3 replays exactly as before: every change below
is gated on the canvas's engine, or is a new option an old log never uses.
Checked: an engine 2 painting of 17 chunks replays to a PNG byte-identical to
the one saved when it was painted.

## Why

A thick, stiff stroke dried into a smooth round tube. The leveling model
(surface.rs `settle_for`) froze relief only up to the paint's yield stress,
which `rheology` put at 5 to 300 Pa; stiff tube paint is above 1000 Pa and up
to about 3000 Pa (notes/research/oil_paint_physics.md). At 300 Pa the
furrows of a brush leveled flat, and only stroke-scale ridges stayed. The
painter never saw relief either: looks showed color only.

## What changed

- **Yield stress** (`surface.rs`, `rheology_at`): engine 4 spans 5 to
  3000 Pa (`5·600^stiff`; was `5·60^stiff`). Lead white from the tube
  (stiffness 0.8) now holds about 835 Pa.
- **Films bridge fine relief** (`settle_for`, `BRIDGE_UM` 40 µm): a thick
  film's top is shaped by the brush, not by the weave under it, so the
  weave's relief fades from its surface over some tens of µm of paint (it
  printed through even millimetre impasto before).
- **Clumping hair** (`bristle.rs` `exchange`, `Surf::clump`): in stiff paint
  the hairs gather into clumps, `hair · (1 + 4·s²)` across, where `s` runs from 0 at a stiffness of 0.4 (as
  on the brush, thinned by any solvent) to 1 at 1. A clump lays
  more paint and the gaps between clumps less (furrows along the stroke,
  averaging out across the brush), and a clump throws the paint each hair
  ploughs aside its own width (up to 3 px) instead of a sub-pixel hair's, so
  walls rise along the stroke's edges. Each hair still moves only its own
  share of paint.
- **Blotting** (`pile{..., blot=0..0.5}`): oil drawn out of the paint, a
  negative medium in `Mixture::paint`: that share of the oil gone,
  stiffness × (1+blot)² (capped at 1),
  scattering × (1+blot).
- **The knife** (`Canvas::knife`, Lua `knife{width=}`): a flexible steel blade
  (it follows relief over `FLEX_MM` 4 mm and bridges finer hollows) resting on
  the dry surface's high points, standing off them by a gap that closes with
  pressure (300 µm × (1−p)^1.5). Wet paint above the blade is cut into its
  bead, or pressed out past its ends in ridges; below it, within the gap and
  `PRESS_IN_UM` (60 µm), the bead fills to the blade's level. Scraping keeps
  what it cuts. Knives are snapshotted with the brushes, so a failed chunk
  leaves them as they were. A log of an engine before 4 has no `knife`
  global, as it had none when it was painted.
- **The relief look** (`look --mode relief [--light az,el]`,
  `Canvas::seen_lit`): the dry relief plus the wet films (bridged as above)
  under a raking light (default from the upper left at 25°), with cast
  shadows marched along the light, a third of the light as ambient, and a
  sheen on wet paint. It changes nothing on the canvas. `relief()` (the
  finished picture's light) is unchanged.
- **Lit saves and the gallery light**: `save [path] --light az,el` or
  `--gallery` (and `run ... --gallery`) write the picture lit on its relief;
  `look --mode gallery` lights it at 55° from above and a little left. The
  lit view rounds paint edges (slopes soft-limited as in `relief`), casts
  shadows only from relief a pixel resolves (the surface blurred by a
  pixel), and fills shadows with more of the room's light the higher the
  light (ambient 0.3 at 10°, 0.55 at 55°). At the gallery light thick light
  dabs keep 0.96 of their flat brightness; at 10° they shadow one another.
- **Sketches**: a session whose name starts with `sketch` paints at 600 px
  instead of 2400 (about 16 times faster) for trying out a composition; its
  log says so (`--@ sketch`) and replays at the same width under any name.
- **Materials** (`Paint::solvent`, `Paint::oil`, two more entries in the wet
  paint's `Prop`, mixed by volume):
  - `pile{turps=0..0.9}`: turpentine makes paint flow on the brush (it
    doesn't clump) and evaporates as it is laid (`Surf::add`): the film is
    that much thinner, of the paint's own body. (Engine 3's `thinner=` is
    upstream's, beside it: its solvent stays in the open film and leaves
    over painting time.)
  - `pile{oil="linseed"|"walnut"|"poppy"}`: drying ×1, 0.8, 0.6.
  - `canvas{ground={{..., absorbent=true}}}`: a chalk and glue ground holds
    `ABSORB_COATS` (0.6 coats) of oil it draws out of paint laid on it
    (`Surf::add`): the film loses that oil, gets stiffer and leaner. Dry
    paint seals it.
  - **Gloss** (`Canvas::gloss`): a baked film's gloss follows its oil
    (smoothstep 0.15..1.3), blended with the surface under it for thin
    films; varnish sets it to 1. Engine 4 shows a matte surface with the
    first-surface reflection it scatters back (`SURFACE_REFLECTANCE` 4%,
    `haze`) in looks and saved pictures. Checkpoint format 10 (`PAINTC10`) for a painting of engine
    4 or later: format 9 with the solvent and oil in the wet paint, the gloss
    and the absorbency. An older engine's painting is written as it always was
    (formats 8 and 9), byte for byte, and its state digests and state dumps
    are the ones recorded for it.
  - Not modeled: yellowing, fading of lakes, color change of chromates.
- **The giverny and impressionist boxes**: see `notes/research/giverny_materials.md` and
  `notes/research/impressionist_materials.md` (from analyses of the paintings).

## Two ways to thin paint

`pile{thinner=}` (engine 3, upstream's `crate::thinner`) and `pile{turps=}`
(engine 4, this note) both put solvent in a pile, and they model it
differently. They are kept side by side, each as it was written:

| | `thinner=` | `turps=` |
|---|---|---|
| the solvent is | a quantity beside the paint (on each bristle, in each pixel's open film) | a share among the paint's own properties |
| a loaded brush holds | that much liquid, part paint and part solvent | that much paint, carrying its share |
| one stroke lays | at most a ceiling of wet film (about 6 µm at 0.5) | what the brush lays, with no ceiling |
| it evaporates | over painting time (minutes; longer from a thick film) | as the paint is laid: the film is thinner by its share |
| on the canvas | it stays for a while, and the film flows and levels | none of it is ever on the canvas |
| a brush or rag that lifts the paint | takes the solvent with it | has none to take |
| on the brush | the hairs are as with unthinned paint | thinned paint doesn't clump the hairs, and spatters more readily |

Where they meet, and nothing reconciles them:
- A pile given both is thinned both ways (untested).
- Clumping and spatter read only `turps=`'s share: paint thinned with
  `thinner=` still clumps as its stiffness says.

## Engine 5: the knife tears

A knife-laid slab parts from the blade raggedly: at its ends, along its
leading edge (up to 8 mm in), where the blade's reach into the hollows runs
out, and in the ridge left where the blade lifts (`Canvas::knife`, a value
noise along and across the blade, gated on engine 5). Engine 4 logs keep the
clean-edged knife and replay as they were.
The torn paint stays under the blade, out of the bead, so a torn pull runs
dry where a whole one does and covers less. The tears are about a millimetre
across and never finer than a few pixels, so a sketch or a narrow preview
tears too, more coarsely.

`run --gallery` also lights the timelapse frames (`frames::start` takes a
light).

## Not done

- Drying cracks in thick paint, and through-drying (a thick film's skin
  dries over a soft interior) are still not modeled.
- The relief light in `finish`/`relief()` still uses its fixed light; a
  finished painting is best shown with `look --mode relief` at a gentle
  elevation, or `relief()` once it is retuned for engine 4's thicker paint.
