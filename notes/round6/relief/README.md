# Relief A/B (Round 6, step 1)

The two best logs (notes/loops/l5_near.lua, notes/loops/l3_green.lua)
replayed at 3200px, identical except the last call: `relief(s, g)` with
off (0, 0), vlow (0.06, 0.006), low (0.12, 0.012) and cur (0.2, 0.02, the
Friedrich style default, `crates/paint/src/style.rs`). Whole images:
`l5_near_*.jpg`, `l3_green_*.jpg`. Sheets: the same 700×500 px window of
the 3200px render at each strength (top left off, top right vlow, bottom
left low, bottom right cur).

What changes with the relief: the canvas weave grid and the stroke-ridge
creases. At cur the green sky is covered in embossed diagonal creases
(`green_sky_sheet.jpg`) and the tree's sky holes get raised outlines
(`green_tree_sheet.jpg`); at vlow both are nearly gone; off looks like a
print.

What doesn't change: the knife-cut hatching on the near rock's shadow
face, the pale rim along its silhouette, the fog rectangle in the wood
(`near_rockedge_sheet.jpg`) and the pale halos inside the green tree's
sky holes are the same at every strength. They're not the relief
lighting. Correction (later the same day): the knife-cut hatching wasn't
the paint either: it was the varnish pooling at the foot of every dried
stroke edge (branch `r6-varnish`, `notes/round6/varnish/`). The rims,
the fog rectangle and the halos are in the paint.

Alice picked 0.06; it's the Friedrich default now.
