# Principles (read before designing a tool, writing a brief or painting)

**North star.** The project's inspiration, in its author's words: "for the
past few months i've been asking our models to paint. opus 5.5 is very
skilled at emulating different styles. every image here is a python
program generated pixel by pixel. there is no image model, and no
off-the-shelf art software. instead, it's about 7,500 lines of code using
standard libraries to emulate different brush styles. the agents don't
use any pictures as reference, instead working only from what they know
about each painter." It showcased very different styles (Pieter Bruegel
the Elder, likely Cézanne, David Hockney and probably J. M. W. Turner), and they looked amazing. Ours is the
same spirit, and more first principles: we simulate the paint and let the
painter make the marks. Friedrich (smooth, thin, luminous realism) is a
harder target than any of those; it may fail, and that's allowed. Turner (Friedrich's exact contemporary, same subjects: light, sky, water, haze; a loose, visible hand) is the bridge: a good diagnostic of whether our physics can do light and atmosphere at all.

Alice's direction, Round 7. We keep drifting from this, and the drift
causes most of what looks digital.

## 1. Tools give physics and constraints, not answers
The engine is the world a painter works in: canvas, grounds, paint that
flows, sets and cracks, brushes that carry, drag, overrun and run dry,
a palette of mixed piles, a clock with sittings and rests, light and
optics. The painter makes every decision: what to paint, where, which
pile, which edge to lose, when to stop.

A tool that **computes the result** (a perfect color field, an exact
mask edge, a structure traced as-is) takes the decision away
from the painter and puts a machine's perfection into the picture. That's
the "digital" look: "the lighting is maybe a little too perfect", "a
filled selection", "an icon", "wire", confetti, fractal twigs.

## 2. Prefer the human process
People paint under constraints, and the constraints make the look:
a handful of piles mixed by knife (never a formula), gestures of a hand
with a brush (never a fill), limited time (paint sets between sittings),
looking and then deciding one passage at a time, not knowing any shape's
exact geometry. When a design choice is "compute it" vs "let the painter
do it the way a person would", choose the second, and make the tool about
the physics that person works with.

## 2b. Entropy
Alice, from round 1 on: "the less entropy we have the more digital
everything looks." A hand never repeats: no two strokes carry the same
load, pressure, length or angle; piles are mixed unevenly; a painter
corrects, overpaints, loses and finds; paint runs dry mid-stroke; grounds
and weave are irregular; time sets some passages and not others. A
computed answer has almost no entropy: the same mark, spacing, edge or
gradient everywhere. So variation has to come from the process (the hand,
the palette, the physics, the painter's decisions), at every scale, and
not from noise sprinkled on a perfect result (that's still a formula).

## 3. First principles
Every mark is made the way a painter makes it: bristles, paint, canvas,
optics (README.md). No flat fills, no optical blends pretending to be
paint. Work from knowledge of Friedrich (text), never from pictures.

## 4. The creativity is the painters'
We build tools; the painter agents do the seeing, composing and painting.
A brief never tells a painter what to paint unless a study needs a
subject. The sketchbook holds craft (recipes with their ceilings), never
finished pictures to copy.

## Checklist for any new tool or feature
- Does it give the painter a physical capability or a constraint, or does
  it hand them a result? If a result: can it be a scaffold the painter
  consults instead (placement, proportion, light direction) rather than
  something traced?
- Would a person painting have this? (A palette knife and piles, yes; a
  per-pixel color function, no. A drawing of a tree's structure, yes.)
- Does it default to the painter deciding? Automation, if any, is opt-in
  and visible.
- Is it deterministic and physical (conserves paint, respects time)?

## Structure tools (rock, form, scene, atmos): the tension
Alice is wary of them, with reason: they grow or compute objects, and
traced in full they read as computed ("too computationally fractal").
They stay because painters need some flexibility and knowledge a person
carries in their head (how an oak branches, where a shadow falls). Use
them as a painter uses studies and knowledge: for placement, silhouette,
major structure and light direction, then paint by hand, selecting.
Revisit each one against the checklist above; prefer turning a tool that
answers into one that informs.

## Agents vs humans: the needle
Agents think in functions (compute everything, everywhere, perfectly);
humans paint in gestures under constraints. The integrator and the
painters are LLMs and will keep reaching for the computed answer: every
brief and review should check against this page.

## Feature freeze (Round 7, Alice)
After the edges, piles and cracks streams land, no new engine capabilities
until a painting round shows which of the existing ones help the pictures.
Bug fixes, tests and painter craft (sketchbook) are fine. The question on
the table: are we overengineering? Evidence to weigh: the visible gains of
Round 6 came mostly from bug fixes (varnish, pinholes, wet lights) and
painter choices, not new machinery; the inspiring post got four styles
from ~7,500 lines; Alice finds round 2's early paintings (notes/amnesia2/,
plain Rust programs on an older, simpler API) better in many ways than
anything since; and in the post, the Turner (the style closest to
Friedrich's world) was by far the most digital-looking and furthest from
its painter, probably not a coincidence. A control experiment (branch
r7-python: a look-first Friedrich in plain Python, no engine) tests it.

### Backlog (frozen; revisit after the painting round shows what helps)
From the process research (notes/research/friedrich_process.md): P1 study
sheets enlarged by a small whole ratio and traced (with a tracer's
faults); P2 pen and ink for the underdrawing; P3 lifting wet paint with a
rag and scraping set paint; P4 painting up to a drawn line with a hand's
error (bare ground between colors); P5 abrasion by cleaning; P6
mechanical crack events; P7 ground defects; P8 old restorations (opt-in).

## Against reward hacking (Alice, Round 7)
Painter agents optimize for what they think is judged, or for what looks
like diligence, rather than for the picture. Receipts: painters' own
verdicts favored their new versions where the critics or Alice didn't
(bare tree B, lime H); Evening Lake reported 89,723 strokes and 25.75 h
while a planned 3 h sitting ran 17.8 h (the clock was reported, not
obeyed); briefs that listed Alice's complaints became checklists (named
symptoms removed, not better painting); painters chose which crops Alice
saw. So:
1. Painters don't choose what Alice sees: crops come from fixed or random
   windows chosen by the integrator or a script, plus the plain full image.
2. No self-verdicts in the evidence: Alice sees the pictures before any
   notes, blind where possible; painters' notes are for the record.
3. Short briefs with principles and process (mix piles, sittings, look and
   revise), not lists of symptoms to avoid.
4. Constraints are enforced, not reported: a sitting that runs past its
   length ends.
5. Only Alice promotes a recipe into the sketchbook, when she approves the
   picture, not when a painter claims it won.
