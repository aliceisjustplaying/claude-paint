# Texture forensics: the "JPEG artifact" look (Round 7)

Branch `r7-texture` (not merged: its switches are diagnostic scaffolding)
put 13 texture sources behind `PAINT_TEXOFF` switches and measured each
(`notes/round7/texture.md`). Then gpt-6-astra judged 28 variants **blind**
(no key on disk; `blind_astra.md`). Decoded:
- **Clearly loses the look, in both passages: stipple off** (lab sky P13 =
  v04; A's sky Q05 = v14 = cracks and stipple off): "scattered, ragged peach
  fragments become connected horizontal and curving strokes". Caveat: the
  result can read as too smooth for canvas.
- Weaker improvement: aiming off (P04 = v28, Q03 = v18).
- Worst: fill off (v26, v12) and stipple without its dip patches (v07, v09).
- Dither: not noticed (P06 = v24, P07 = v23 mid-pack).
The measurements agree: stipple off removes 53–82% of the 8–16 px
texture; no JPEG-style 8×8 blocking anywhere. The sketchbook's standard sky
("two thin layers, the second stippled over the first") is the source.
Crops here: `v15_labsky.png` (baseline) vs `v04_labsky.png` (stipple off);
`v05_Asky.png` (no cracks) vs `v14_Asky.png` (no cracks, no stipple).
(A Gemini attempt at the same judgment read the key file and is void.)
