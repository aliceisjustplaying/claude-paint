# Texture forensics: what makes paint read as "JPEG artifacts"? (branch r7-texture)

House rules: ~/tmp/paint-r6-b943b1ca/briefs/r7_common.md (read it).
Worktree ~/src/a/claude-paint-r7-texture, branch r7-texture (from main).
Feature freeze: no engine changes committed; temporary switches in this
branch for experiments are fine (documented, default off).

Alice keeps seeing something like "JPEG artifacts" in the paint texture of
our renders although they are lossless PNG ("the entropy still reads like
JPEG artifacts"; "it's hard to put into words"; "I think the jpeg artifacts
are not the crackle at all, it's something about texture"). A lossless
check showed the look is in the paint, not the compression
(notes/round6/jpeg_check/). It was there before craquelure was part of the
finish (the Round 6 lab sky A: notes/lab/sky_A.lua, finished without
cracks). Tonight's paintings show it: notes/round7/arms/ (A, B, C and their
crops; key.md); A and its sky and pond are the worst per Alice, B's sky
less so.

## Do
1. **Suspects.** List every source of pixel-scale texture in a render and
   find each in the code: the 8-bit save's triangular dither (canvas.rs
   `save`: independent per channel), the relief lighting of the height
   field (weave and ridges), the linen weave and ground texture, bristle
   footprints and their per-pixel deposit (aliasing on the pixel grid,
   tile boundaries), stipple touches, the look-and-fill dabs, the varnish,
   craquelure, the per-pixel noise in color_over/aim, anything else.
2. **Switch each off, one at a time**, on the same passages: arm 1's sky
   and pond (`git show r7-arm1:paintings/lua/willows.lua`), arm 2's sky
   (`git show r7-arm2:paintings/src/bin/pond_poplars.rs`) and the Round 6
   lab sky A (notes/lab/sky_A.lua). Render at 3200 (crops are fine:
   `--crop`), same windows each time. Scratch copies of logs are fine; for
   engine switches use env vars read in the code, default off.
3. **Measure** each variant against the baseline: blockiness at 8/16 px
   periods, a radial power spectrum of the texture in a quiet passage,
   per-pixel chroma noise (Lab a/b variance at 1–2 px), the prevalence of
   flat patches with sharp small steps (tone quantization), and anything
   else that captures "looks like JPEG" (JPEG artifacts are 8×8 blocking,
   ringing near edges, chroma bleeding and blotchy smooth gradients).
4. **Evidence:** lossless 1:1 crops of each variant (same windows) in
   notes/round7/texture/ with a neutral numbering (v01, v02, ...) and a
   separate key file mapping numbers to switches (so they can be judged
   blind), plus notes/round7/texture.md: suspects, where each lives in the
   code, the measurements and what each switch changes. No verdict on
   which one "fixes" it: the integrator will have the variants judged
   blind.
Budget about 1.5–2 h. Commit; don't push. Final message: short report with
paths and the headline measurements.
