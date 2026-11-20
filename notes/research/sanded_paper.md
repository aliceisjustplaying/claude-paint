# Sanded pastel paper: a gritted ground

These notes back `paper::Grit` (the easel's `paper={grit=}`). They cover the
coat of grit that makes a sanded pastel paper, a pastel board or a homemade
pumice ground, and what it changes for a pastel stick. Compiled 2026-11-15.
It extends paper_surface.md (the fiber sheet) and pastel_stick_tribology.md
(the stick, Archard wear, the tooth filling).

**Tags:** **[M]** measured or stated by a cited source. **[D]** derived here
from cited values, with the arithmetic shown. **[E]** an estimate with no
direct measurement; treat it as a tunable range.

**The main gap.** I found no profilometry, pore volume or stick-wear
measurement of any artists' sanded paper. The makers grade their grit with
abrasive grit numbers but don't say which standard they use. The layer
counts below are makers' and painters' claims. The coat's geometry is
worked out from first principles (FEPA's grain sizes, random packing, the
binder's solids, where the stick rests) and the coated-abrasive literature,
which describes sandpaper, not pastel paper. The stick's wear comes from
Rabinowicz's abrasive model with the grains' measured angularity. The only
estimate in it is where an alumina grain sits in the measured range. The
binder's wet level is a bench value, set at the lean end of its bounds by
Colourfix's coat weight.

---

## 1. What a gritted ground is

### Grit, binder and coat
- **[M] Grit materials.** Pumice, marble dust or rottenstone (Wikipedia
  "Pastel"); aluminium oxide on modern papers (UART: "aluminum oxide
  particles uniformly" deposited, per a retailer; Pastel Premier: "double
  primed and coated with aluminum oxide", then "a final seal coat").
- **[M] Binders.**
  - **Acrylic.** Art Spectrum Colourfix is an acrylic primer with grit,
    "waterproof when dry". It can be brushed or rolled onto paper, board,
    canvas, metal or glass. Colourfix papers are 300 g/m² hot-pressed
    watercolor paper screen-printed with that primer.
  - **Starch.** Church (1915) gives a homemade ground: "a thin and even
    coat of powdered pumice mixed with liquefied starch" on Bristol board
    or stout drawing paper.
  - **Glue.** Liotard's paper preparation used "ground pumice stone and
    fish-glue" (a manuscript, via Jeffares). In Rome some pastellists coated
    canvas with parchment glue mixed with sieved marble dust and pumice
    (Chaperon 1788 §323, via Jeffares).
- **[M] Grit size by grade.** These are the FEPA P (ISO 6344) median sizes:

  | grade | P240 | P320 | P400 | P600 | P800 |
  |---|---|---|---|---|---|
  | median size, µm | 58.5 | 46.2 | 35.0 | 25.8 | 21.8 |

  The CAMI grades of the same number are finer (320 ≈ 35 µm, 400 ≈ 22 µm,
  600 ≈ 15 µm; metallography.org).
- **[M] Makers' grades.**
  - UART: 240 ("the largest tooth"), 320 ("extra layering"), 400 ("a
    favorite"), 500, 600 ("detailed work") and 800 ("finest").
  - Pastel Premier: 320 and 400, plus a 600 extra fine.
  - Neither maker says which standard the numbers follow. **[E]** The
    easel takes FEPA P, which gives the coarser reading. FEPA's P scale
    (ISO 6344) is the coated-abrasive standard outside North America, and
    UART is coated electrostatically, as sandpaper is. Its origin is
    disputed: Dakota Art Pastels lists Germany, while Jackson's and an
    Australian retailer list the US. So origin can't settle it, and the
    missing "P" in the US retailers' names is not evidence for CAMI either.
    A CAMI reading would give 22–23 µm at 400 and a tooth 37% smaller. A
    micrograph of a UART or Pastel Premier surface at a known scale would
    settle it; none was found published.
- **[M] Open and closed coats.** In a closed coat the grain covers 100% of
  the backing. An open coat covers 50–70%, which "leaves significant
  valleys between the grains". Closed coats "clog or load quickly" on soft
  materials (Benchmark Abrasives).
- **[M] How far the grain sits in the binder.** In one sandpaper patent
  the size coat is 55–90% of the grain's length. The make and size coats
  together are under 90% of it, so at least about 10% of the grain stands
  proud (US 7,294,048). A pastel coat needs open valleys, so it is closer
  to a make coat alone than to a full size coat; §1's packing below sets the
  binder's level from its solids rather than from this.
- **[M] Grain size spread.** FEPA's sedimentation limits (by mass) for P400
  are ds3 58.9 µm, ds50 35.0 µm, ds94 25.2 µm; for P500 48.3, 30.2 and 21.5
  µm (a supplier's table of the FEPA 43-2 limits, whitefusedalumina.net).
  - **[D]** A lognormal through them has σ(ln d) = ln(35.0/25.2)/1.555 =
    0.21 and ln(58.9/35.0)/1.881 = 0.28 at P400, and 0.22 and 0.25 at P500.
    The easel takes **0.23** for every grade (`GRIT_SPREAD`).
  - **[D]** Sedimentation weighs grains by mass, so by number the median
    is d50·e^(−3σ²) = 0.85 d50.
- **[M] Cover.** Spheres dropped one by one at random, none overlapping, jam
  at a projected cover of 0.547 (random sequential adsorption; Feder 1980).
  An open coat of 50–70% (Benchmark Abrasives) is about that.
  - **[D]** A coated abrasive's coat weights give its cover. Grade 50
    (336 µm) carries 618 g/m² of fused alumina (US 5,417,726), 618/3.95 =
    156 µm of grain. A sphere's volume over its shadow is 2d/3, so the cover
    is 156/(0.667 · 336) = 0.69. Grade 24 (764 µm, 1400 g/m²) also gives 0.69.
    A "closed" coat is that dense, not 1.
  - `cover` stays the user's (0.3–1, default 0.6).
- **[D] Coat thickness.** One grain deep, so 22–60 µm from P800 to P240,
  with its tallest grains about 1.3 d50 up. That is thin next to the felt
  grain's millimetre cells, so the sheet's grain shows through at the mm
  scale, and the easel keeps the sheet's relief under the coat.

### The tooth: the open volume of the grain packing
The code is `Grit::tooth_um`. The arithmetic is in `sanded_paper_coat.py`,
next to this note.

- **The coat.** The grains are spheres of FEPA's spread (above), lying at
  random on the sheet with the given cover.
  - **The binder is laid level with the median grain's top**
    (`BINDER_WET` = 1 d50). That is the leanest wet coat that carries its
    grains, which is how a primer is brushed or rolled. **[D, a bench
    value]** A cross-section micrograph would measure it. Its bounds are
    1–1.3 d50: below the median grain's top the coat doesn't hold half its
    grains, and 1.3 d50 reaches the tallest ones. A thicker wet coat gives
    less tooth (below).
  - **[M/D] The coat weight puts it at the lean bound.**
    - Colourfix is 300 g/m² hot-pressed paper screen-printed with its
      primer to make 340 g/m² (Art Spectrum's brochure and Colourfix Smooth
      page; Jackson's; Talas). So the dry coat weighs 40 g/m².
    - The model's coat at a cover of 0.6 holds 13.6 µm of grain at P400
      (cover · 2/3 · E[d³]/E[d²], with E[d³]/E[d²] = d50·e^(−σ²/2) = 34.1 µm
      by number), plus the dry binder. At 1 µm per g/cm³ per g/m², with
      acrylic at 1.1 g/cm³ (`sanded_paper_coat.py`, `coat_weight`):

      | grade | grit | binder at d50 | binder at 1.3 d50 |
      |---|---|---|---|
      | P400 | quartz or marble, 2.65 g/cm³ | 47.0 g/m² | 51.9 g/m² |
      | P400 | alumina, 3.95 g/cm³ | 64.7 | 69.7 |
      | P500 | quartz | 40.6 | 44.8 |
      | P600 | quartz | 34.6 | 38.3 |

    - Art Spectrum doesn't name its grit or its size, and calls the surface
      a "fine tooth". At the easel's P400, every coat inside the bounds
      outweighs 40 g/m², and the leanest comes closest. At P500 with a
      quartz grit, the leanest level is the one that weighs 40 g/m². A
      tinted primer's pigment also weighs something, which leaves even less
      for binder. So the measured weight points to the lean bound, and the
      easel keeps d50.
    - The weight can't separate grain size from binder level: at P600
      quartz, 40 g/m² would allow 1.44 d50, past the bound. Both weights are
      nominal, and neither maker states a tolerance.
  - **It dries to its solids by volume.** The dry binder fills the coat
    between the grains from the sheet up.
  - **[M/D] Acrylic:** an acrylic emulsion is 46–47% solids by weight
    (Rhoplex AC-33, the conservators' standard; specific gravity 1.04). At a
    polymer density of 1.1–1.15 that is 0.43–0.45 by volume: take **0.45**.
  - **[M/D] Glue:** a glue size is mixed 1:10 to 1:20 by weight
    (Cennini's 100 g per litre for panels, via Natural Pigments; 1:16 and
    1:20 elsewhere). With gelatin at 1.35 g/cm³ that is 0.04–0.07 by volume:
    take **0.07**, the stronger glue a grain coat needs. A starch paste is
    about as lean. Between 0.05 and 0.10, the tooth changes by under 4%.
- **Where the stick rests.** The tooth is full once the pastel reaches the
  level where the stick's face rests, and from then on the stick slides on
  its own pastel (the existing fill and third-body model). The face rests
  where the tallest grains carry the stick's real contact, F/(σp·A₀): **1%**
  (`GRIT_FACE`). **[D]**
  - The default soft stick (`soft` 0.75) has σp = 30^0.25 · 2^0.75 = 3.9 MPa
    (`Stick::mechanics`). The hand's default force is 2.0–2.5 N (0.4–0.5 ×
    `FORCE_AT_FULL` 5 N; a hatching 0.45, 2.25 N).
  - At 2.25 N that gives 2.9% on a fresh 20 mm² crescent, 1.1% on 50 mm²
    and 0.39% on a fully worn facet: 148 mm², a 12 mm stick at 50°.
  - A stick wears flat at once (pastel_stick_tribology.md §4), so take the
    geometric middle of the range, 1%.
  - It stays a constant of the sheet rather than each stroke's
    F/(σp·A₀). The tooth is the coat's pore field (`Canvas::micro`): the
    pastel fills it and the fill model reads it. Each step's force
    already sets how far the stick sinks into that relief (`pastel::seat`,
    against the stick's σp). A per-stroke share would count the load twice,
    and would change a filled pixel's room after the fact.
- **The tooth** is the open volume between the binder and that level. **[D]**:

  | grade | acrylic, µm | glue, µm | × drawing paper (6 µm) | binder level / d50 (acrylic, glue) | face / d50 | grains carrying the stick, per mm² | their depth in the stick, µm |
  |---|---|---|---|---|---|---|---|
  | P240 | 40.8 | 54.8 | 6.8–9.1 | 0.46, 0.05 | 1.37 | 5 | 7.6 |
  | P320 | 32.2 | 43.3 | 5.4–7.2 | 0.46, 0.05 | 1.37 | 8 | 6.0 |
  | P400 | 24.4 | 32.8 | 4.1–5.5 | 0.46, 0.05 | 1.37 | 15 | 4.6 |
  | P500 | 21.0 | 28.3 | 3.5–4.7 | 0.46, 0.05 | 1.37 | 20 | 3.9 |
  | P600 | 18.0 | 24.2 | 3.0–4.0 | 0.46, 0.05 | 1.37 | 27 | 3.4 |
  | P800 | 15.2 | 20.4 | 2.5–3.4 | 0.46, 0.05 | 1.37 | 38 | 2.8 |

  - The tooth goes as the grain size: 0.70 d50 in acrylic and 0.94 d50 in
    glue, at a cover of 0.6.
  - **The spread matters most.** The stick rests on the coarse tail, the few
    percent of grains above ds3, at 1.37 d50. The open volume under them is
    about twice what grains of one size would leave: 11 µm at P400.
  - **P400 in acrylic, with one input changed at a time:**
    - 24.4 µm as it stands (face 1%)
    - face 2% → 21.0; 5% → 16.4
    - solids 0.35 → 26.6; 0.55 → 22.2
    - a wet coat 1.3 d50 deep → 19.9
    - σ 0.20 → 22.3; 0.27 → 27.3
    - cover 0.3 → 24.9; 0.85 → 22.8; 1 → 21.6
  - A closed coat holds a little less, as the abrasive trade has it: closed
    coats load sooner.
- **[D] The pastel reaches all of it.**
  - The open volume lies above the binder, among the top grains, in valleys
    tens of µm wide. Pastel particles are 1–20 µm, with fines of 60 nm to
    5 µm (pastel_stick_tribology.md §3).
  - A packed bed's throats would stop the coarse ones. Terzaghi's filter
    rule gives a constriction of about D15/5, and the throat between three
    touching spheres is 0.155 d: 5.4 µm at P400. But a coat one grain deep
    has no bed below its top grains. In acrylic, the binder fills what lies
    below its level; in glue it barely reaches 0.05 d.
  - So no size filter is applied.
- **[D] A multi-coat primer** (Colourfix brushed on thickly, gesso and
  pumice in coats) adds open volume below the top grains only if it is
  leaner than its CPVC. An acrylic primer isn't, which is why it is
  waterproof and seals the sheet. Its tooth is the top layer's, as above.
- **The bearing the model gives it.**
  - **[D]** The model has one pore depth µ per pixel. It sets both what the
    tooth holds (µ, the right number here) and the bearing curve near the
    top, 1 − e^(−o/µ).
  - The real coat's bearing rises more slowly under the face: 2 µm below
    it, the grains cover 1.5% at P400, against 7.9% for an exponential of
    the same mean. Only the coarse tail is there.
  - So the stick sinks a little less into the model's grit than into a real
    coat's. Archard's wear depends on the real contact area F/σp, not on the
    depth, so this changes where the pressure falls, not how much is worn
    or held.
- **[D] Scatter.** The grains over a 0.25 mm pixel are cover·px²/E[πr²],
  about 48 at P400. A pixel's tooth scatters as Gamma(k)/k over those k
  grains, as the paper's pores do (paper_surface.md §4).

## 2. How many layers it holds
- **[M] Makers' claims.**
  - Pastel Premier 320 and 400 "accept as many as 25 layers of pastel"
    (Jerry's Artarama, Art Department LLC and others).
  - Colourfix primers are "ideal for artists who are looking to do
    multiple layers of pastel without the need for fixative".
- **[M] Painters and reviewers.**
  - Sanded papers hold "10–20+ layers of pastel compared to 2–3 layers on
    regular pastel paper" (Paul Rubens Shop guide).
  - Smoother papers "fill up quickly. Once the shallow valleys are full,
    the paper goes slick" (same guide).
  - A UART reviewer found it "took in quite a few layers, even after
    fixative" (The Artistic Gnome).
  - Jackson's compared 400–800 grades with 1 and 4 layers, in photographs
    only.
- **[D] What the physics gives against the claims.**
  - At P320–P400, the coat's open volume is 4.1–7.2× the paper's pores
    (§1, acrylic to glue). P240 gives up to 9×, P600–P800 2.5–4×.
  - The claims are 10–25 layers against 2–3, i.e. 4–8×, and they are made
    for the 320 and 400 grades. **The physics falls inside the claims, and
    is kept as it is, not tuned to them.**
- **What "a layer" means.**
  - **A painter's layer is a covering application**, not a stroke at a set
    force. The grains file the stick about 3× as fast as paper does (§3).
  - So at the same force, a sanded sheet fills in a few more strokes than
    paper does: 3× the pastel per stroke, into 4.1–5.5× the room.
  - Painters say it takes a lighter touch, and laid that way, each pass as
    thick as one on fresh paper, it holds many more.
  - **Counted in passes of fresh paper's thickness,** the model's drawing
    paper takes 3.1 and P400 in acrylic 13.8.
  - **The rest of the gap** is likely the makers' "as many as 25" (a
    ceiling, for glue-leaner or coarser coats) and "a layer" as a light
    scumble. Nothing measures either.
- **Test thresholds** (pastel.rs `a_sanded_sheet_takes_layer_on_layer`).
  These come from the sources and the derivation above, not from the model.
  The same hatching at 1 N runs 30 times, until both sheets refuse.
  - The sanded sheet holds at least 3.5× the pastel. The derivation gives
    4.1 at P400 in acrylic (the model, 4.5).
  - Counted in layers of a fresh paper pass, drawing paper holds fewer than
    5 (claims: 2–3) and P400 at least 10 (claims: 10–25).

## 3. How it wears a stick
- **[M] Painters.** Sanded papers "eat up pastels and coloured pencils
  fast" (Fine Art Tutorials). They "eat pastel faster" and "can chew up
  soft sticks and fingers" (Paul Rubens Shop). The surface "will also
  consume your pastels very quickly" (Ellen Questel). One painter thinks
  grit in an acrylic base eats pastel less (WetCanvas). That is a single
  anecdote, and the easel doesn't use it.
- **[M] Abrasive wear.** In the abrasive model the wear coefficient is the
  asperities' mean tan θ over π. Two-body abrasion runs K = 10⁻²–10⁻¹, and
  three-body abrasion, where loose grains roll, 10⁻³–10⁻² (Rabinowicz 1995,
  via ScienceDirect Topics). The stick's own K against paper (0.01–0.06,
  pastel_stick_tribology.md) is already in the two-body range.
- **[M] Hardness.**
  - An abrasive scratches once it is about 1.2× harder than the worn
    surface. Beyond that the wear rate depends only weakly on the hardness
    ratio (Tabor; Hutchings 2004; Pintaude).
  - The grit: fused alumina about 20 GPa, silicon carbide about 25 GPa
    (2500 HV, metallographic.com), pumice a volcanic glass of Mohs 5–6
    (about 5–6 GPa), marble dust calcite (Mohs 3, about 1.3 GPa).
  - **A paper fiber's wall** is 0.24–0.55 GPa by nanoindentation (Wimmer
    et al. 1997, 0.28 GPa in spruce; a review's 0.35–0.55).
  - **A pastel compact** is 2–30 MPa.
  - **[D]** Grit and fiber are both 10–10,000× harder than the stick, so
    both abrade it in the hard regime, and their hardness drops out. The
    grit material doesn't matter, and the easel doesn't ask for it.
  - (Graphite on paper is different. A 6B lead is 50 MPa and a 4H one
    1.1 GPa, against the fiber's 0.3–0.5 GPa, so harder leads sit near or
    below the 1.2× threshold. That is likely part of why graphite wears
    20–100× more on frosted glass than on paper (Archambault). The ratio
    doesn't carry over to pastel.)
- **[B/D] The wear ratio, `GRIT_WEAR` = 3.0.** Rabinowicz gives K as the
  groove's cross-section over half the bearing area. For a cone that is
  (2/π)·tan α at an attack angle α. The model's wear is K × the real
  contact × the distance slid (pastel.rs), so the grit's factor is
  K_grit/K_fiber. The same share of the groove comes off the same friable
  compact on either, since it crumbles rather than piling up.
  - **The fiber.** The stick rests on fiber crossings.
    - A crossing is a ridge 12.5 µm in radius across the fiber (ω = 25 µm,
      `Paper::drawing`). Along the fiber it is 100–300 µm, where the upper
      fiber bends over the lower.
    - A layer covers ln(1/ε) = 0.69 of the sheet (paper_surface.md). Its
      fibers are 0.69/ω = 27.6 mm per mm², so they cross (2/π)·27.6² = 489
      times per mm².
    - **Greenwood–Williamson, plastic, at the stick's 1% real contact**
      (`GRIT_FACE`, §1), on a drawing paper of Rq 3–5 µm
      (paper_surface.md): 14–34 crossings carry the stick, each 1.2–2.1 µm
      into it.
    - A ridge at random to the stroke gives K = (8/3π)(2/π)·√(δ/2R) =
      **0.120–0.155**, 0.137 in the middle.
  - **The grain.** The stick rests on the tallest grains (§1's table),
    2.8–7.6 µm deep in it from P800 to P240.
    - That is far past a crushed grain's tip radius: about 1 µm (Sin, Saka
      & Suh 1979; they note it can't be measured exactly). So the grain cuts
      with its facets, not its rounded tip.
    - **[M] The facets' slope is measured as angularity.** Hamblin &
      Stachowiak's spike parameter SPQ fits each corner that stands outside
      a particle's equal-area circle, and averages cos(θ/2) over them, θ
      being the corner's apex angle. A circle has 0, a square 0.707, an
      equilateral triangle 0.866 (Mikli et al. 2001, Table 3).
      - Over six abrasives of 0.25–0.3 mm (glass, silica sand, garnet,
        quartz, alumina, SiC), SPQ runs 0.023–0.601 (Stachowiak &
        Stachowiak 2000, quoted by Mikli et al.). The values for each
        abrasive are not in the quote. The round glass beads, which wear
        least, are presumably the low end. Angular quartz and SiC give the
        most friction (Stachowiak & Stachowiak, *Wear* 249 (2001) 201), so
        the top presumably belongs to crushed grains like these and the
        crushed alumina in the set.
      - Freshly fractured WC-Co powder measures 0.63–0.65: the fine
        fraction, which stays there however often it is milled because it is
        "the product of direct fracture" (Mikli et al., Table 5). Repeated
        milling rounds the coarse fraction from 0.65 to 0.52.
    - **[M] Chalk is the measured proxy for pastel.** Hamblin & Stachowiak's
      two-body tests abraded chalk pins, and their wear rates rose nearly
      linearly with the abrasives' mean SPQ (*Tribol. Trans.* 39 (1996)
      803; the linearity as cited by Stachowiak & Stachowiak 2001). So
      wear here scales with the corners' slope, as Rabinowicz's model
      assumes.
    - **[D] The attack angle.** A corner of apex θ meets the stick at
      90° − θ/2:
      - SPQ 0.50: 30.0°, K = 0.37
      - SPQ 0.55: 33.4°, K = 0.42
      - SPQ 0.60: 36.9°, K = 0.48
    - **[E] Where in that range.** The per-abrasive SPQ table wasn't reached
      (the 2000–2001 papers are paywalled), so alumina's own value is
      unknown. Its bounds are 0.50–0.60: 0.60 is the angular end of the
      measured abrasives, and 0.50 a hexagon's corner. The easel takes
      **0.55**, their middle and a little under fresh fracture.
    - The PR's first estimate, a cube's corner at random (SPQ 0.71, K =
      0.64 from `sanded_paper_coat.py`), is sharper than any measured
      abrasive.
  - **The ratio.** It is 0.419/0.137 = **3.05** at SPQ 0.55 and the middle
    of the paper's roughness, so take **3.0**. Over SPQ 0.50–0.60 and Rq
    3–5 µm it is 2.4–4.0.
  - **[D] No grade effect.**
    - Below about 100 µm, metals wear less on finer grit (Misra & Finnie
      1981). Sin et al. trace this to grooves that become as shallow as the
      grain's tip radius. With blunt tips it shows; with sharp ones it
      doesn't (Gåhlin & Jacobson 1999, via Pintaude).
    - A pastel is about 1000× softer than a metal, so every grade's grains
      sit 3–7 µm deep in it, past their tips. The factor is the same from
      P240 to P800.
  - **Absolute values.** The stick's K against paper (0.01–0.06,
    pastel_stick_tribology.md) is 0.1–0.5 of the fiber's geometric 0.13–0.16.
    That share comes off. On grit the same share gives K ≈ 0.04–0.3, within
    Rabinowicz's two-body range (10⁻²–10⁻¹).
  - In the model, 3.0 multiplies the stick's wear where it touches the grit
    above the fill. A fixed crust over the grit is pastel, and files the
    stick as it does on paper (`CRUST_WEAR`).
  - **Test** (`grit_wears_the_stick_faster`): the first stroke on fresh P400
    takes 2.4–4.0× what it does on drawing paper. The model gives 3.1.

## 4. Fixative
- **[M]** Colourfix is meant to take many layers "without the need for
  fixative". UART takes layers "even after fixative". Pastel Premier
  withstands "scrubbing or reworking", and its 320 and 400 take water,
  mineral spirits and alcohol (heavy alcohol "may soften the surface").
- **[E] The easel doesn't change fixative for grit.**
  - Fixative binds the loose pastel in the tooth, as on paper.
  - The bound crust is filed again at `CRUST_WEAR`, and the grit tips
    above the fill go on abrading at `GRIT_WEAR`.
  - A fixed sanded passage therefore takes pastel again, as a fixed paper
    one does. Because the grit's tooth is larger, it is seldom needed.

## 5. Oil, water and the ground on paper
- **[M]** Colourfix primer takes "oils, gouache, watercolours". For oil on
  paper one tip is to size the paper first with acrylic medium or gesso.
  The primed surface is waterproof (Jerry's Artarama; Delta Art; Jackson's).
- **[E] How the easel takes oil on a gritted sheet** (`Paper::oil_capacity`):
  - The coat's open volume takes oil, as an absorbent ground's pores do.
  - An acrylic coat seals the fibers under it: the sheet's own pores take
    no oil.
  - A glue or starch coat is lean and lets oil through to the sheet, so
    its pores take oil as plain paper's do.
- **A ground laid over a gritted sheet** (`canvas{ground=}`) is a paint
  film. A thin one leaves the grit's tooth open, and one as thick as the
  tooth fills it (`Canvas::micro_um`). Gesso brushed over sanded paper
  does fill it.
- **[E] The knife.** On a gritted sheet a blade rides hard grains. It
  doesn't flatten the tooth or tear fibers loose (no fiber is bare), and
  a scalpel shaves no fiber layer. It still takes loose pastel.

## 6. When gritted grounds were available
- **18th century: pumice on the support.**
  - **[M]** Pastellists drew on coarse blue wrapping paper strong enough
    "to withstand rubbing with pumice, a technique that artists used to
    produce … tooth" (Metropolitan Museum, "The Rise of Pastel in the
    Eighteenth Century").
  - **[M]** Liotard's recipe used ground pumice in fish-glue. Rome's
    pastellists used marble dust and pumice in parchment glue on canvas
    (Chaperon, *Traité de la peinture au pastel*, 1788). Both are via
    Jeffares's *Prolegomena*.
- **Late 19th century: pumice paper and sandpaper as named supports.**
  - **[M]** John Henry Twachtman, *Landscape*, after 1890: "pastel on pumice
    paper, mounted on cardboard" (Metropolitan Museum, Rogers Fund 1925; catalogued in
    Bolger & Shelley, *American Pastels in the Metropolitan Museum of
    Art*, 1989).
  - **[M]** Twachtman's *Pastures, Branchville, Connecticut*, c. 1888–91,
    is on "gray-green sandpaper … (paper: P. L. Breveté)" (Cleveland
    Museum of Art 1946.72).
  - **[M]** Elmer MacRae's *Schooner at Dock*, 1911 or 1914, is pastel on
    pumice paper mounted on board (listed on the same Metropolitan Museum
    page; the page could not be re-read for this note, HTTP 429, so this
    and the Twachtman record are as a search quoted them).
- **Early 20th century.** **[M]** "For pastel work a specially prepared
  paper is now generally employed. This has a surface of finely-powdered
  pumice" (Church, *The Chemistry of Paints and Painting*, 4th ed., 1915,
  pp. 322–323).
- **Modern.** **[M]** Colourfix (Art Spectrum, Australia), UART, Pastel
  Premier (USA) and others make acrylic or aluminium-oxide sanded papers and
  boards. **[E]** I couldn't confirm when each product was introduced.
- **So, for the easel:**
  - A glue or starch pumice ground fits any date from the 18th century on.
  - A commercial pumice paper fits from at least about 1890 (Twachtman).
  - An acrylic sanded paper is a late-20th-century support.

---

## Sources

- FEPA P sizes: Universal Photonics, grit size comparison chart (https://universalphotonics.com/Content/refdocs/_upiRef10b.pdf); HLH, blasting media grit chart (https://hlh-js.com/resource/blog/blasting-media-grit-size-chart-grit-mesh-microns-inches-conversion/); Fine Tools, abrasive grit conversion (https://www.fine-tools.com/blog/conversion-chart-abrasives-grit-sizes); metallography.org grit size chart (https://www.metallography.org/resources/grit-size-chart).
- Open and closed coats: Benchmark Abrasives, "A Complete Guide To Coated Abrasives" (https://benchmarkabrasives.com/blogs/abrasives/what-are-coated-abrasives). Size coat and protrusion: US 7,294,048, "Abrasive article" (https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/7294048).
- UART grades: UART (https://www.uartpastelpaper.com/); Texas Art Supply (https://www.texasart.com/products/uart-premium-sanded-pastel-paper); Jackson's Art Blog, "UArt Pastel Paper" (https://www.jacksonsart.com/blog/2015/12/04/uart-pastel-paper/); The Artistic Gnome, "UART Sanded Paper Review" (https://theartisticgnome.com/uart-sanded-paper-review/).
- Pastel Premier: Jerry's Artarama (https://www.jerrysartarama.com/global-arts-premier-sanded-paper-320-grit-italian-clay-sanded-paper-sheets-10-pack-20x26-quot); Art Department LLC (https://artdepartmentsupply.com/products/pastel-premier-sanded-pastel-paper-12x16-white); FLAX (https://flaxart.com/pastel-premier-sanded-pastel-paper/).
- Colourfix: Art Spectrum, Colourfix brochure (https://artspectrum.com.au/wp-content/uploads/2017/10/Colourfix-DL-Web.pdf); Delta Art technical sheet (https://www.deltaart.ca/images/uploads/file/coloufixprimer[1].pdf); Jerry's Artarama (https://www.jerrysartarama.com/art-spectrum-colourfix-pastel-primer); Jackson's (https://www.jacksonsart.com/en-ca/art-spectrum-colourfix-pastel-primer).
- Colourfix's weight (300 g/m² hot-pressed paper printed to 340 g/m²): Art Spectrum, Colourfix Smooth (https://artspectrum.com.au/products/colourfix-smooth/); Jackson's, Colourfix Original 340gsm (https://www.jacksonsart.com/en-us/art-spectrum-colourfix-original-340gsm-23x30cm-apx-9x12in-sand); Talas (https://www.talasonline.com/Colorfix-Pastel-Paper).
- UART's origin and process: Dakota Art Pastels (https://www.dakotapastels.com/products/UArt-Sheets: Germany, 300 g/m²); Jackson's (https://www.jacksonsart.com/uart-sanded-pastel-paper-800-grade-sheets: US, electrostatic coating); softpastels.com.au (https://www.softpastels.com.au/pastel_paper/uart/uart.php: US).
- Particle angularity: Mikli, Käerdi, Kulu & Besterci, "Characterization of powder particle morphology", *Proc. Estonian Acad. Sci. Eng.* 7 (2001) 22–34 (https://kirj.ee/wp-content/plugins/kirj/pub/eng-1-2001-22-34_20230304173220.pdf): SPQ's definition, the regular shapes (Table 3), WC-Co powders (Table 5), and the 0.0231–0.6008 range for six abrasives citing Stachowiak & Stachowiak, NORDTRIB 2000, vol. 2, 434–443. Hamblin & Stachowiak, "Description of abrasive particle shape and its relation to two-body abrasive wear", *Tribol. Trans.* 39 (1996) 803 (https://www.tandfonline.com/doi/abs/10.1080/10402009608983598). Stachowiak & Stachowiak, "The effects of particle characteristics on three-body abrasive wear", *Wear* 249 (2001) 201 (https://www.sciencedirect.com/science/article/abs/pii/S0043164801005579), which cites the chalk tests' near-linear wear against SPQ. Stachowiak, "Particle angularity and its relationship to abrasive and erosive wear", *Wear* 241 (2000) 214 (https://www.sciencedirect.com/science/article/abs/pii/S0043164800003781). The per-abrasive SPQ values are in these last two, not reached.
- Painters on layers and wear: Paul Rubens Shop, "Soft Pastel Paper" (https://www.paulrubensshop.com/blogs/knowledge/soft-pastel-paper); Fine Art Tutorials, "The Best Pastel Paper" (https://finearttutorials.com/guide/best-pastel-paper/); Ellen Questel (https://www.ellenquestel.com/blog/161250/introduction-to-soft-pastels-and-some-tips-on-how-to-work-with-them); WetCanvas, "Wallis Sanded Paper" (https://www.wetcanvas.com/forums/topic/wallis-sanded-paper/).
- Abrasive wear coefficients: ScienceDirect Topics, "Abrasion" (https://www.sciencedirect.com/topics/materials-science/abrasion), citing Rabinowicz, *Friction and Wear of Materials*, 2nd ed. (1995), and Rabinowicz, Dunn & Russell, *Wear* 4, 345 (1961).
- FEPA P sedimentation limits (ds3, ds50, ds94): whitefusedalumina.net, "White Fused Alumina Powder" (https://whitefusedalumina.net/product/white-fused-alumina-powder/); ISO 6344-3:2021 is the governing standard (https://www.boutique.afnor.org/en-gb/standard/iso-634432021/coated-abrasives-determination-and-designation-of-grain-size-distribution-p/xs138520/280048).
- Binder solids: MFA CAMEO, "Rhoplex AC-33" (https://cameo.mfa.org/wiki/Rhoplex_AC-33); AIC Conservation Wiki, "BPG Adhesives" (https://conservation-wiki.com/wiki/BPG_Adhesives); Natural Pigments, "Gesso Grounds from Ancient Recipes" (https://www.naturalpigments.com/artist-materials/gesso-grounds-ancient-recipes); Mathieson Fine Art (https://www.mathiesonart.co.uk/blog/how-to-prepare-a-panel-for-oil-painting).
- Coat weights: US 5,417,726, "Coated abrasive backing" (https://patents.google.com/patent/US5417726). Random sequential adsorption: Feder, "Random sequential adsorption", *J. Theor. Biol.* 87, 237 (1980) (https://www.sciencedirect.com/science/article/abs/pii/0022519380903586).
- Filter throats: Fannin (2008) on Terzaghi's filter rule, via Taylor et al., HR Wallingford (https://eprints.hrwallingford.com/1167/1/PA_1_10-Taylor-H.pdf).
- Abrasive size and tip effects: Misra & Finnie, "On the size effect in abrasive and erosive wear", *Wear* 65, 359 (1981) (https://www.sciencedirect.com/science/article/abs/pii/0043164881900624); Sin, Saka & Suh, "Abrasive wear mechanisms and the grit size effect", *Wear* 55, 163 (1979); Pintaude, "Characteristics of Abrasive Particles and Their Implications on Wear" (InTech, 2011; https://cdn.intechopen.com/pdfs/15604/InTech-Characteristics_of_abrasive_particles_and_their_implications_on_wear.pdf), citing Rabinowicz et al. (1961), Gåhlin & Jacobson (1999) and Coronado & Sinatora (2009).
- Fiber wall hardness: Wimmer, Lucas, Tsui & Oliver, *Wood Sci. Technol.* 31, 131 (1997); review in "Experimental micromechanical characterisation of wood cell walls", *Wood Sci. Technol.* (2012) (https://link.springer.com/content/pdf/10.1007/s00226-012-0515-6.pdf). SiC hardness: metallographic.com, SiC paper brochure (https://www.metallographic.com/Brochures/SiCpaper.pdf).
- Hardness ratio: Hutchings, "Modelling abrasive wear", *Wear* (2004) (https://www.sciencedirect.com/science/article/abs/pii/S0043164804002595); Pintaude, "An Overview of the Hardness Differential Required for Abrasion" (https://www.researchgate.net/publication/245371525); ScienceDirect Topics, "Hutchings" (https://www.sciencedirect.com/topics/engineering/hutchings).
- History: Metropolitan Museum, "The Rise of Pastel in the Eighteenth Century" (https://www.metmuseum.org/perspectives/the-rise-of-pastel-in-the-eighteenth-century); Jeffares, *Prolegomena to Pastels & pastellists* (https://pastellists.com/Misc/Prolegomena.pdf); Metropolitan Museum, Twachtman, *Landscape* (https://www.metmuseum.org/art/collection/search/12844) and Bolger & Shelley, *American Pastels in the Metropolitan Museum of Art* (1989; https://www.metmuseum.org/met-publications/american-pastels-in-the-metropolitan-museum-of-art); Cleveland Museum of Art 1946.72 (https://www.clevelandart.org/art/1946.72); Church, *The Chemistry of Paints and Painting*, 4th ed. (Seeley, Service & Co., 1915), pp. 322–323, quoted at https://noteaccess.com/MATERIALS/PPS.htm; Wikipedia, "Pastel" (https://en.wikipedia.org/wiki/Pastel).
