# Studio record: what the paint and tools did
## Mixing piles
- Lead white dominates a mix. Even with four parts smalt to one of white, the paint was only a mid value. Deep or mid tones need little white.
- New paint mixed to match an older passage that had been glazed over came out too light at three parts white to one of blue. One and a half to two parts white matched. Small test swatches on a dry spot found the mix.
- `print(pile)` shows a pile's parts and medium. Piles kept as globals were still usable 100 painting days later.
## Strokes and fills
- On dry paint, `work` with the detail hand, `load=1`, `pressure={0.6,0.9}` and coverage 3 to 4 laid a fully opaque fill every time.
- The same dark pile at ordinary load and pressure came out pale and mauve-gray. Its thin film looked lighter than the pile.
- Thin lines of an earth-rich dark (rigger and pointed round) came out tan, lighter than a mid blue-gray field around them. Restroking the same paths with a pile rich in bone black made them read dark.
- A broad-hand wash at coverage 2 with `fill=true` came out lighter and more opaque than planned and hid the pencil drawing. Where the bands overlapped it left a row of lumps.
- Thin pale strokes laid along the upper edge of dark lines read gray and metallic, and some looked detached from the line, with pointed ends.
- Pale ridges drawn with a pointed round (pressure 0.35 to 0.15) over a dark mass came out too bright and scratchy.
- Pairs of short dragged filbert marks along a path read as a row of pills. A light rim on the far edge made each hollow look raised.
- Short horizontal scumbled dabs (filbert 3, pressure 0.35 to 0.15) laid lighter than the pile promised.
- The body hand with `edge="soft"` over a small patch laid a flat, opaque slab with a hard lower edge, even at medium 0.55.
- Opaque rectangular repair patches showed as rectangles. A patch whose edges ran along existing shape edges left no seam.
## Wet into wet
- A dark full-load pass into a wet mid tone only lifted it, even at coverage 5 and `load=1`. The result stayed mid-gray.
- Pale over wet dark went gray and dirty. Dark over wet pale went gray, streaky and ragged-edged. Both happened again every time the underlayer was still wet.
- New paint laid over test swatches that were 20 minutes old came out lighter where it crossed them.
- A full-load dark pass over tacky paint gave a solid, continuous dark mass.
- Once the underlayer was dry, both light over dark and dark over light sat cleanly on top.
## Blending
- Two blending passes over a wet banded layer softened it but left faint bands. A stipple and then a blend left speckles in the transition, and one more blend fused them.
- A clipped blend at a mask's edge left the hard edge. Two unclipped blends across a 50-unit band dragged the wet paint over the line and softened it.
- Blending a rectangle that held a wet patch among older dry paint smeared the patch's paint into pale blurs around it.
- A blend ring around a dry patch's edge only partly hid the edge.
## Glazing
- The glaze hand isn't clipped. On a band about 50 units tall its long horizontal strokes ran well past the mask on both sides and filled the neighboring passages.
- A dark pile at medium 0.65, brushed on with the detail hand and a filbert 5 at coverage 2, was nearly opaque and flattened the texture under it. Where its mask was cut by a `rect` it left a lighter seam.
- A glaze at medium 0.75 and coverage 1.2 over a broad dry field, clipped below a curve, left a hard straight top edge. It went down in cool swirls, and it was dry within 5 days.
## Stippling and texture
- Coverage driven by noise (`mask:times(noise)` and then `work`) laid a mottle far stronger and more even than intended, like a leopard skin.
- Stippling into a wet layer and then blending fused the touches.
## Drying and time
- Lead-white-rich layers at medium 0.15 to 0.2 were mostly touch-dry in 20 hours and fully dry in 26. Smalt-rich strips were still tacky after 22 hours.
- Dark piles with little white (umber, bone black, medium 0.05 to 0.1) laid thick were setting at 6 hours and still tacky at 16 and 26 hours.
- Stacked dark passes stayed tacky for 5 days and were dry by about 10. Pale caps over dark and repeated repairs often needed 7 to 9 days.
- `drying(x,y)` at several points before crossing a passage prevented the muddying. After a wait, check again: different spots dried days apart.
- Varnish needed every point to be touch-dry. `varnish{coats=0.3}` warmed the pale passages slightly.
## Masks and edges
- `ribbon(pts, w)` takes the full width: a 100-unit ribbon of width 10 has an area of 1078.
- `m:grow()` on a shape's mask spread paint onto an adjacent shape. It had to be painted back.
- `noise{stretch=...}` takes positional values `{angle, k}`. Named keys failed with "error converting Lua nil to f32".
- A chunk that stopped with a runtime or syntax error changed nothing, as the guide says.
- `clip=true` on a body pass kept a narrow band's edge crisp.
## Brushes
- A round 4 with `point=1` stays at 0.58 wide up to pressure 0.2. It is 1.2 at 0.4, 3.0 at 0.7 and 4.9 at 1.
- A rigger 1.5 stays at 0.56 up to pressure 0.4, then 1.16 at 0.7 and 1.95 at 1. `pressure_for` a width below that minimum returns about 0.
- Short strokes with pressure falling to 0 and ramps of 0.6 drew noticeably shorter marks than their paths.
## Looking at the canvas
- A crop may be at most 1200 px per side, which is about 500 canvas units. A 1000-unit crop was refused, so use `--size 1000` for a whole view.
- `--mode squint` showed that the lower field was as light as the lightest band, which the color view had hidden. `--mode value` confirmed the dark and light masses.
- Close crops at 1:1 caught seams, streaks and wet-mixing damage that a whole view didn't show.
- Compute time: 6,200 thin strokes took 23 s, and one blend over the upper half of the canvas took 47 s.
