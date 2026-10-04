# Open questions

Tools proposed and held back until they are better understood. Each entry
says what the tool would be, why it is not built, and what needs studying.

## Holding the knife up to the canvas (not built; needs study)

**The tool:** `look --hold <pile> --at x,y`: a loaded knife's swatch of a
pile shown beside a passage of the canvas, the way a painter holds the
loaded knife up against the picture (or the motif) to compare.

**Why it's held back:** it is a real physical act, but in software it sits
close to the line the easel draws against previews (no `try`, no probe):
it shows paint against the picture before it is laid.

**What to study first:**

- What the act gives a painter: the same light on the mix and the passage,
  the same surround (so simultaneous contrast doesn't fool the eye), and an
  edge-to-edge comparison. The eye judges relations (lighter or darker,
  warmer or cooler) far better than it judges a color alone or remembers one.
  Painters squint while they do it, to see value with the hue and chroma
  turned down.
- What it does not give: how the paint will look laid. On the knife it is
  thick, a separate object with its own gloss. On the canvas it is thinned,
  mixed into wet paint, over an underlayer, and it changes as it dries. A
  painter learns that difference; the knife doesn't show it.
- Whether that gap is enough to keep it on the right side of the preview
  line, or whether a swatch rendered by the engine says more than a knife
  in a painter's hand does.
- Whether the easel even needs it: the painters here paint from memory,
  with no motif to hold the knife against, so only the second use (a mix
  against passages already painted, for relational value) applies. Compare
  how well `look --palette` beside a `look` crop already serves that.
