# Open questions

Tools proposed and held back until they are better understood. Each entry
says what the tool would be, why it is not built, and what needs studying.

## Holding the knife up to the canvas (built; speculative: it may not belong)

**The tool:** `look --hold <knife or pile> --at x,y` (the look tool's `hold`
and `at`): a passage of the canvas at full detail (240 units square around
the point, or `--crop`) with a loaded knife held over it, the blade's end at
the point, the way a painter holds the knife up against the picture to
compare a mix with a passage.

- **What is held.** A knife (`k = knife{...}`) with what is on it, a mix
  scraped off the canvas included; or a pile by its global's name, as a
  fresh full load from its heap as it is on the board now (a dirty board's
  heap with its dirt).
- **The blade is the engine's.** The paint is laid thick on a steel blade
  with the knife verb, at the painting's pixels to the unit, its
  millimetres and its engine, so it has the body, ridges and torn ends the
  paint would have on a real blade.
- **One light, one surround.** The blade is seen exactly as the passage is:
  `mode: "value"` grays both, `squint` blurs both, `relief`, `gallery` and
  `light` light both.
- **It only reads.** No hand time, nothing in the log, the knife keeps its
  load, the state digest is as it was (tested).

**What it does not show, because the act doesn't:** how the paint will look
laid. On the knife the paint is thick, a separate object. On the canvas it
is thinned by the brush, mixed into what is wet there, over what is under
it, and it changes as it dries. A painter learns that difference; holding
the knife up doesn't show it, and neither does this.

**Why it is speculative:** it sits close to the line the easel draws against
previews (no `try`, no probe): it shows paint against the picture before it
is laid. It is complete as a tool and kept in one place (`hold_look`, two
arguments, one session lookup, a test), so it can be taken or left whole.

**For the owner to decide:**

- Whether a look that shows unlaid paint against the picture is on the right
  side of that line at all, given that it shows only what the physical act
  shows.
- Whether painters would use it to compare (a relation: lighter or darker,
  warmer or cooler) or to shop for a color before committing. A round's
  journals would show which.
- Whether the easel needs it: the painters paint from memory, so there is no
  motif to hold the knife against, only passages already painted.
