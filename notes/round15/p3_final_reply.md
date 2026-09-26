The painting is finished: *Morning on the Baltic shore*. A calm sea before sunrise, with a waning crescent high on the left and a glow right of centre where the sun will rise. A woman stands at the water's edge, dark against the brightest water. Net poles stand on a grassy dune on the left, a granite boulder sits in the right foreground and a brig is hull-down on the horizon. A clean from-scratch render (203 s) came out byte-identical to the final image built from resumed checkpoints.

**Files**
- Final render: `~/src/a/paint-r15-p3/out/r15_p3_full.png` (2400 × 1846 px)
- Program: `~/src/a/paint-r15-p3/paintings/src/bin/r15_p3.rs`
- Notes: `notes/r15_p3.md` (composition, method, limitations, critique) and `notes/craft_r15_p3.md` (materials and tools)
- The 17 stage checkpoints take 7.6 GB in `out/`; I left them there.

**Tool limitations I met, and the workarounds**
1. **Checkpoints go stale on edits that change nothing painted.** One untagged blank line left after a `// ckpt: end`, or removing redundant parentheses, staled every later checkpoint. Workaround: keep tagged regions tight, and use `--stale-ok --ckpt` when I know an edit paints nothing different.
2. **A `// ckpt: from <stage>` tag has to name the first stage that uses the code.** I tagged the cloud helpers `from sky` though only `sky2` used them, so editing them staled the sky checkpoint. I retagged them `from sky2`.
3. **Code the drawing stage uses ties every checkpoint to it.** Changing the drawn dune crest would have re-run the ground and both sky layers. I reshaped the crest after the sky stages instead, which left a pentimento: its pencil line showed through as a crack until I laid the dune paint fuller and glazed the bank.
4. **Darkening passes whose strokes cross two fields came out lighter.** A stroke's paint is mixed from what lies under its start, so vertical strokes starting in the pale sky laid sky-mixed paint over the darker sea. This hit the net's veil and the figure's first reflection. Workaround: `Canvas::glaze` with a transparent pigment and a mask.
5. **Small shapes mixed for an assumed thickness came out pale.** At coverage 4 the `detail()` preset assumes about 8 coats, so it picked a semi-transparent pale smalt for the sails, which showed pale in the thin film a 9-unit sail actually gets. Workaround: `.by_masstone()` with a full load.
6. **`Mask::roughen` re-thresholds the whole mask.** A 200-unit soft fade at the dune's foot became a hard diagonal that read as a cliff. Workaround: roughen only the crest, then multiply the fade in.
7. **Badger passes over one lean sky layer lifted paint off the thread tops.** The ground showed through as a pale lattice, and a third pass made it worse. Workaround: two thin sky layers with a drying pause between them, and no third pass.
8. **Checkpoints are large:** 443 to 549 MB each.
