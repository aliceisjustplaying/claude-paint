# Pointed tips: round sable and rigger

Branch `tip`. This addresses amnesia2 friction item 8: small marks became
beads, blobs, pegs, "hearts" and ladders at 3200px (mountains M12, coast
C14, winter W9).

## What was wrong

I measured three causes in `crates/paint/src/bristle.rs`.

1. **Dashes and ladders.** A hair touched the canvas only where the surface
   reached `th = 1 - reach * 1.6`. A lightly pressed hair has a small
   reach, so it touched only the weave's peaks. Every hairline, and the
   tapering end of every flick, became dry-brush dots at the weave pitch.
   At 1000px the weave is finer than a pixel, so the dots only appeared at
   3200px.
2. **Pegs.** The smallest footprint was still 0.45 of the tool's width,
   and the center hairs had ragged thresholds. A flick was a bar at full
   width that broke into dots where the pressure fell.
3. **Resolution.** Each hair's track had a 0.55px minimum radius. Its paint
   spread as a uniform thin film over whole pixels, and Kubelka–Munk (KM)
   makes a thin film of opaque paint darker than the same paint covering
   part of a pixel. Small marks therefore came out about twice as heavy
   at 1000px as at 3200px. This was true for every tool (see the numbers
   below). The plough also dropped paint on whole pixels, which gave fine
   tracks a dotted ridge along one side.

## The model

`Tool::point` (0..1) says how finely the hairs come to a point: 0 is a
blunt tuft (hog, flat, filbert, fan, badger, stippler) and 1 is a fine
point. `round_sable` and `rigger` have `point: 1.0`. Everything below
applies only when `point > 0`. With `point = 0` the old code runs
unchanged: the old golden matched bit for bit with both presets set to 0.

- **A cone of graded hairs.** A hair at root radius ρ ends P_FULL·ρ² up
  from the point (P_FULL = 0.85, the pressure at which the whole belly is
  down). While the tuft is wet, the touching hairs gather toward the axis
  by √(p/P_FULL). The contact width therefore grows with the pressure
  itself: at light pressure only the point touches (a hairline of about
  two hairs), pressing spreads the belly, and lifting off draws the mark
  down to a point. Uneven hair lengths add unevenness that grows outward,
  so the point stays clean.
- **A wet tip wicks.** A loaded soft tip carries a bead of paint that wets
  the weave's valleys as well as its peaks, however lightly it is pressed:
  contact depth uses `max(reach, wick)`, where `wick` rises with the
  hair's load. When the hair runs dry it skims the peaks again (dry
  brush).
- **Capillary feed.** Paint runs from the belly to the spent tip. Each
  step shares out part of every hair's paint, with an e-folding length of
  two tool widths. Volume is conserved exactly and colors mix through the
  tuft. A hairline uses only a few hairs, so one load draws a long line,
  as a rigger does.
- **The split point.** Cohesion follows the brush's mean load. When the
  tuft runs dry it no longer gathers into a point, and the paint stops
  bridging between hairs (next item). The mark opens up and splits.
- **Hairs share the tuft's width.** At each step every touching hair lays
  a track from halfway to its neighbor on one side to halfway to its
  neighbor on the other, measured across the travel on the real contact
  points. In a wet tuft the paint bridges wider gaps too, so the tracks
  tile the mark. In a dry tuft the gaps stay open.
- **Coverage, not just thickness.** Hair tracks are box-filtered both
  across and along their length, so a hair finer than a pixel lays the
  same paint per unit length wherever the pixel centers fall.
  `Wet::cover` records the share of each pixel that the wet paint covers
  (1 for every blunt deposit). `dry` and `look_px` composite the paint at
  its real thickness over that share and let the rest of the pixel show
  what is under it. As a film levels it closes pinholes: `dry` lifts a
  pixel's share to what two opposite neighbors both hold, so wide marks
  close up while a hairline keeps its share. The plough splats paint
  bilinearly, a hair's width away.

## API

```rust
// pointed tools as before; the point is on by default
let mut b = Held::new(Tool::round_sable(1.6), seed);
b.load(Paint::body(dark), 0.8);

// a flick: pressed on the belly, lifted off to a hairline point
c.drag(&mut b, &Gesture::new(vec![root, mid, tip]).pressure(0.75, 0.0).ramps(0.1, 0.75), None);

// a hairline: the point only, light pressure
c.drag(&mut rg, &Gesture::line(a, e).pressure(0.3, 0.3).ramps(0.05, 0.1), None);

// think in widths: how wide at this pressure, what pressure for this width
let w = tool.mark_width(0.4);          // units
let p = tool.pressure_for(0.5);        // pressure for a 0.5-unit mark
// a custom tool with a blunter point
let t = Tool { point: 0.5, ..Tool::round_sable(3.0) };
```

`mark_width` is the geometric contact width, and the measured ink width
comes within about ±30% of it (the table below). `Touch` with a pointed
tool also gathers toward the point: a light touch leaves a small dot and
a hard press leaves the belly.

## Evidence

The study is `cargo paint study_tip`. It paints birds (V-flicks with a
body touch), a brig's rigging, a signature-like line, grass flicked up,
bare twigs and spruce tips on a Friedrich ground. For 3200px crops use
`--full --crop 20,20,330,220` (birds), `360,50,640,300` (rigging),
`30,480,330,650` (grass), `680,360,980,650` (spruce) and `680,120,980,280`
(signature). Before-and-after images are in `notes/tip/`:

- `before_*_3200.jpg` → `after_*_3200.jpg`. **Rigging:** dashed lines
  became continuous hairlines. **Grass:** blunt pegs that broke into dots
  became blades flicked up to hair points. **Birds:** blobby V's became
  wings that start pressed at the body and lift off to fine tips.
  **Spruce:** bars with blunt ends became needles tapering to points.
  **Signature:** reads thick-thin with pressure; the underline flicks off
  to a hairline, and a nearly spent load breaks up as a dry sable does.
- `*_cmp.jpg`: the 1000px render (top) against the 3200px crop,
  box-downsampled in linear light (bottom). Before, the 1000px marks were
  about twice as heavy. After, the two match.
- `moonrise_*`: Moonrise before and after. The dead oak's limbs now taper
  to real twigs. At 3200px the figures keep their silhouettes with a fine
  sable grain. At 1000px they, the valley spruces and the crescent are
  slimmer than before (see "Changes to existing output").

Measured by `cargo test --release -p paint probe_ink_width -- --ignored
--nocapture`. Ink width is the darkening summed across a straight mark,
in units:

| tool, pressure | before: 1000px / 3200px | after: 1000px / 3200px |
|---|---|---|
| rigger 0.5, p 0.4 | 1.22 / 0.71 | 0.30 / 0.30 |
| sable 1.6, p 0.4 | 1.85 / 0.99 | 0.57 / 0.57 |
| sable 1.6, p 0.7 | 2.35 / 1.51 | 0.96 / 0.94 |
| sable 3, p 0.9 | 4.45 / 2.77 | 2.39 / 2.32 |

Tests are in `bristle::tip_tests`: resolution independence (within 20%,
500px against 1600px), width follows pressure and a flick tapers, a light
hairline on linen at full size has no gaps, and feeding conserves paint.
With the point switched off, the hairline test finds 13 gaps and the
resolution test fails (1.50 against 1.01).

## Changes to existing output

- **Golden re-recorded** (twice, in both tip commits). The scene drags a
  round sable and a rigger. Blunt tools are bit-identical.
- **Checkpoint format** is now `PAINTCK2` (it adds `Wet::cover`), so old
  `.ckpt` files are refused. Re-run with `--ckpt`.
- **Pointed marks are true to size.** At 3200px, typical pressures
  (0.7–0.9) give 85–95% of the old width. Rigger lines are about half
  their old width, because the old 0.55px minimum doubled them. The
  1000px preview no longer inflates small marks, so existing paintings'
  previews show slimmer details (Moonrise's figures, spruces, crescent and
  star). Their full renders change much less. A painter who wants the old
  weight should take a bigger brush or press harder (`pressure_for`
  helps).
- `Style::detail()` and `line_tool` are a round sable and a rigger, so
  handlings built on them get the pointed model too.

## Open issues

- **Remaining grain.** Wide marks made of several strokes at mid pressure
  keep a slight grain of lighter pixels at 3200px, and at 1000px the
  Moonrise oak trunk shows a few light flecks. Pinhole closing handles
  one- and two-pixel gaps, not larger ones.
- **`mark_width` is approximate** (±30%). It ignores the thinner paint at
  a round mark's edges and splay above P_FULL.
- **Gaps between neighboring strokes** at low pressure are real, since the
  marks are narrower now. Handlings that space strokes by the tool's width
  may show more ground between detail strokes. I haven't checked this
  across the handlings.
- **Blunt tools still depend on resolution.** A filbert 3 lays ink 3.08
  units wide at 1000px against 1.73 at 3200px. The coast painter's
  low-load filbert "ladders" (C14) are dry brush on the weave, and I left
  them alone as asked. The same coverage model would carry over, but it
  would change every painting.
- **Study motifs are schematic.** The twigs and spruce in `study_tip` are
  the study's gestures, not a growth model.
- A convenience API (`Gesture::flick`, `Hand::flick`) would help. The
  gestures above are only two builder calls, so I left it out.
