# Studio record: what the paint and tools did
## Mixing piles
- Piles mostly of lead white, laid in broad bands, came out paler and higher in key than wanted. Deeper passages needed far less white.
- A dark earth pile with a little white, laid as body and then blended, came out lighter and redder than the same pile left unblended beside it.
- A dark body pile with some white over a dry dark field read too light. The same pile with no white, loaded 0.9 at coverage 3.5, gave a solid dark mass.
- Small shapes filled with a pile heavy in lead white stayed the brightest things in their area and read as cut out.
## Strokes and brushes
- Body strokes planned inside a noise-shaped mask came out stroky and wormy until blended.
- `detail` work along a band 4 to 7 units wide came out as a row of dots or beads, not a line.
- A filbert 7 dragged at pressure 0.35 falling to 0.05 left thin, even, mechanical streaks.
- Short filbert 4 dashes stacked in a narrow band looked choppy and regular. Long thin strokes (60–220 units, pressure 0.15–0.45) crossing a passage read as quiet horizontal texture and hid hard seams under them.
- A rigger 2.2–2.4 with a full point, load 0.7–0.8 and pressure 0.65 falling to 0.4, laid a clean dark line over dry paint.
- A rigger 1.4 flicked up from a line at pressure 0.4–0.7 to 0 made tufts too small to read, and dark ones vanished on a dark field. A rigger 1.8 with 14–40 unit strokes, after a 3-hour wait, read.
- A flat 4.5 (load 0.9, pressure 0.9 to 0.8, `orient="across"`) laid solid narrow verticals over a tacky field.
- A rigger 1.5 in two short curved strokes, pressure rising then falling, made a small clean V-shaped mark.
- A clipped `detail` fill of thin tapering shapes 3–5 units wide came out as spikes, too thin.
- Two flat 5 strokes (load 0.6–0.8, pressure 0.5–0.7) clipped to a shape covered an unwanted pale patch cleanly.
- A pointed round 1.6 at load 0.3–0.35 and pressure 0.35 falling to 0.15, clipped, laid a faint hairline.
## Masks and edges
- Subtracting a shape from a mask doesn't protect it: the unclipped strokes of the next pass ran over it and buried it. Passing the same mask as `clip=` kept them off.
- `edge={found=0.5, soft=0.5}` on a body pass gave a serrated, spiky edge. Its strokes also stopped short of the canvas's bottom edge and left pale bumps until a clipped band was laid there.
- A rim band made as `m - m:shrink(2.2)`, limited by a side mask, also caught the shape's bottom edge and laid a pale hatched patch there.
- A pale rigger line clipped to `m:shrink(0.3)` landed as a faint stripe inside the edge, not on it.
- `above(446)` with a bare number is an error ("points: want ..."). A function or a point list works.
- A chunk that errors changes nothing and can be rerun as is.
## Blending
- Blending a whole shape after accents spread the accents across it and dragged fresh dark lines into streaks. Blending only a narrow seam with `tool="filbert 8"` kept most of a line.
- `blend` with `clip=false` on a grown mask dragged wet paint far sideways. It pulled a dark edge into the light field beside it as a fuzzy lump and lifted a dark shape.
- Clipped blends of horizontal bands left hard straight seams where the blend masks ended.
- Dotted light touches along a wet edge fused into a soft edge when a band across that edge was blended.
- Thin light streaks blended within a shrunk mask faded almost to nothing.
- A patch repainted over dry paint stayed visible as a lighter disc. Blending its edge and later a thin scumble softened it but left a halo.
## Wet into wet
- Dark hatched dashes (`hatch`, clipped) laid into a wet pale passage and then blended came out lighter and pinker than their pile, as a flat block. The same kind of dashes, unclipped and unblended with ragged ends, in a freshly laid wet passage read as broken horizontal marks.
- A white-heavy scumble (filbert 6, load 0.25, pressure 0.12–0.3, coverage 0.5) over a dry dark field went on as opaque pale blobs. Two blends spread the pale over the whole field.
- A light opaque pile scumbled with a filbert 3 went on as hot opaque patches. The same color at medium 0.3, in a pointed round 3 at load 0.35, laid a subtle line.
## Stippling
- A width-3 stipple into a wet passage came out lighter than its pile. Four hours later (setting to tacky) it was still lighter. `detail` strokes over it a day later, when it was tacky, gave the dark.
## Glazing
- Glazes at medium 0.6–0.7 over tacky or dry mid tones barely darkened them. Clipped over tacky paint, the glaze came out streaky with a lighter fringe along the clip. Blending it while wet softened the streaks.
- A glaze at medium 0.45 over a dry passage went on near-black and streaky, far stronger than those, with a hard top edge.
- A glaze at medium 0.72 over dry paint left dark blobs. Two horizontal blends while it was wet turned them into soft smudges.
- A glaze at medium 0.75 over a dry pale passage sank into the weave as crossing streaks. Two blends and then a light one (pressure 0.2–0.35) evened it without clearing it.
- A glaze at medium 0.55 over light shapes that were still setting didn't dull them. Once they were dry, a clipped `detail` fill (round 2.5, medium 0.6) changed their tone fully.
## Drying and time
- White-rich body passages were tacky about 22 hours after laying and dry a day later.
- Earth-and-umber body on a shape was tacky at 23 and 43 hours and dry by about 3.3 days.
- Broad body bands were dry after 32 hours. Dashes worked wet into them and blended were still tacky then and dry a day later.
- A dark black-and-umber body at coverage 3.5 was still tacky after 52 hours.
- Dark earth body was tacky 36 hours later. A second coat laid on the tacky one was still tacky 30 hours after that.
- Small, thick white-heavy fills were still setting or tacky about 1.3 days later and dry after 3 more days.
- `wait()` prints the new time, and `drying(x, y)` reports open, setting, tacky or dry.
## Looking and the session
- A crop can be at most 500 units wide at 2400 px. Crops 560–600 units wide were refused.
- Value mode showed values bunched in the light-to-mid range when the color view hid it. Squint and mirror views were used to check the masses.
- `bin/easel check` took about 300 s for 77–81 chunks and needs the session open. After `close` it replies "no painting open".
- `open` replays the log, so globals from earlier chunks (masks, piles) are there again in later sittings.
- Shell state doesn't carry between commands. `cd` to the studio and export the scratch variables in every call.
