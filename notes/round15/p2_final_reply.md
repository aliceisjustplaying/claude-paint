I've painted "Oak in the snow at dusk" and judge it complete. A from-scratch render takes about 148 s, and a render resumed from a checkpoint came out byte-identical.

The picture: an old stag-headed oak on a snowy rise at the left, silhouetted against an apricot afterglow under a grey-blue sky. A Gothic church stands on the horizon, and a path winds to a lone walker heading toward it. There is also a receding fence line, dry grass through the snow, crows and the evening star.

**Render paths**
- Final: `~/src/a/paint-r15-p2/out/r15_p2_full.png` (2400 × 1714)
- Program: `~/src/a/paint-r15-p2/paintings/src/bin/r15_p2.rs`
- Notes: `notes/r15_p2.md` and `notes/craft_r15_p2.md`
- Stage checkpoints (15 files, about 6 GB) are still in `out/`; delete them if you need the space.

**Tool limitations met, and workarounds** (the engine is unchanged)
1. **Helper code makes every checkpoint stale.** Any edit above `main`, even an added `use` import, stales every checkpoint. I tagged the tree generator `// ckpt: from oak` and used `--stale-ok` only once, after the import change. The path geometry also feeds the drawing stage, so editing it meant a full re-render.
2. **`Shape::ribbon` leaves holes at sharp elbows.** Its outline crosses itself there and the fill cancels. I built each limb from one ribbon per segment plus a disc at each joint.
3. **A clip mask can't tell one limb from another.** Lit-side strokes of a thin limb streaked across the thick limb it joins. I painted all wood dark first, then put the lit edge on its own rim mask derived from the limb shape.
4. **No stroke-direction field from geometry.** I built my own 2-unit grid holding the direction of the thickest limb in each cell and passed it to `angle`.
5. **Single strokes (`c.drag`) never fill in thin spots.** Crossing strokes into wet paint lifted each other and left pale patches at the limb junctions. A unifying pass of dark bark over the whole limb mask fixed it.
6. **Ribbon end caps reach half a width past the end.** The trunk's round cap showed through the snow at its foot. I cut the limb mask off at the snow line.
7. **Stipple lace and stroke blotches.**
   - Stippling after the badger left pale lace; stippling before it fused the touches into soft cirrus.
   - In the snow the stipple still made foam-like patches, so I dropped it there and used two badger passes.
   - Faint lighter blotches where broad strokes start remain in the snow.
8. **Unclipped strokes overrun edges by design.** Snow strokes ran up over the distant woods; `clip(true)` on every snow pass fixed it.
9. **Painting wet into wet lifts the paint below.** The first trunk, painted into the still-wet snow, came out pale; drying the canvas first fixed it.
10. **Previewing tree shapes cost a render.** I added two environment variables, `OAK_SVG` (write the tree as an SVG and exit) and `OAK_SEED` (pick a candidate), and compared eight seeds. ImageMagick couldn't read that SVG, so I used `rsvg-convert`. `magick montage` failed for lack of fonts, so I used `+append`/`-append`.

In my own critique, the snow is the weakest part: smooth, almost airbrushed, with little of Friedrich's stippled surface. The oak's twigs are uniformly crisp, with no haze in the far parts of the crown. The upper sky is livelier than he usually allows.
