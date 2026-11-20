# Tracing a drawing onto another sheet

The numbers behind `paint::trace` and the easel's `trace` (`--@ trace`; gap 4 of
notes/cassatt_support.md, phase G4 of notes/plan-intaglio.md). Tags: [M] measured,
[D] derived, [P] practitioner report, [E] estimate.

## 1. What Cassatt did, and what the verb models

- She traced studies onto the final sheet and onto plates. For the 1890–91 colour prints, a
  full-size drawing was laid on a soft-grounded plate and retraced "with a stylus or a
  graphite pencil", and the ground stuck to the drawing's back (notes/research/cassatt_materials.md
  §5, §6). The second and third plates may have been made by tracing again, or by an offset
  of a proof (unconfirmed; Mathews & Shapiro 1989).
- Studio practice, then and now, offers three ways to move a line between sheets:
  1. **A coated back.** A tracing (or the drawing itself) has red chalk, charcoal or
     graphite rubbed on its reverse. It is laid on the new sheet and its lines are gone
     over with a stylus or a hard pencil. The pressure moves the coat across.
  2. **A window.** The two sheets are held to the light and the line is drawn straight
     onto the top one with the tool.
  3. **A blind stylus.** No coat: the stylus leaves an indented line, which shows in raking
     light.
- The verb models all three (`by="transfer"` with `coat=`, `by="window"`, and transfer with
  no coat). Pouncing and squaring are not modelled; neither is recorded for her
  (cassatt_materials.md §6).

## 2. The stylus's contact and the line's width [D, from M]

**The stack.** The stylus presses the sheet that carries the coat onto the sheet below,
which lies on the board. Every paper in the stack compresses through its thickness by
Chen et al.'s measured curve, σ = 0.636(e^{13.54ε} − 1) MPa (already the engine's: sheet.rs
`chen`; notes/research/paper_mechanics_and_transport.md §1).

**Choice.** Chen's slope at zero strain is 0.636 × 13.54 = 8.6 MPa. The paper model's own
through-thickness modulus `Paper::z_mpa` is 39 MPa for the drawing paper (Mann, Baum &
Habeger 1979, on board), and it carries a mount's give folded in (`Paper::mounted`). The
trace keeps Chen's densification exponent β = 13.54 and starts the curve at the paper's own
modulus: σ = (E/β)(e^{βε} − 1), E = `z_mpa`. So a sheet on a soft mount gives more, as it
does under a stick.

**The pores close.** A layer compresses no further once its pores have closed: ε ≤ its
porosity ε_p. That is the physical limit on the line's width.

**A ball on side-by-side springs.** Each spring feels the ball's local sink,
δ(r) = δ₀ − r²/2R. Under pressure p every layer of the stack carries that same p, and their
compressions add: δ(p) = Σ tᵢ εᵢ(p) with εᵢ = min(ε_p, ln(1 + βp/Eᵢ)/β). Then
F = ∫ p 2πr dr = 2πR ∫₀^{δ₀} p dδ, and for each layer that has a closed form,
tᵢ[q/β − (Eᵢ/β²) ln(1 + βq/Eᵢ)], with q the lesser of p and the pressure that closes its
pores. That is solved for p₀ at the hand's force. The contact's radius is a = √(2Rδ₀), and
the line is 2a wide. A test checks the closed form against a quadrature of the pressure
over the contact (`the_contact_carries_the_force`).

**The limit.** The width stops at 2√(2R Σ tᵢ ε_pᵢ), the whole stack dense. It also stops at
2R, the ball sunk to its equator, whichever comes first.

**Numbers.** A 1 mm ball, a natural tracing paper on the drawing paper (§3), computed from
the formulas:

| force | p₀ | sink | width |
|---|---|---|---|
| 0.5 N | 10 MPa | 42 µm | 0.41 mm |
| 1 N | 17 MPa | 54 µm | 0.46 mm |
| 2.5 N | 36 MPa | 72 µm | 0.54 mm |
| 5 N | 71 MPa | 88 µm | 0.59 mm |
| 20 N | 290 MPa | 118 µm | 0.69 mm |
| limit | | 166 µm | 0.81 mm |

- The width grows slowly with force, because Chen's curve stiffens exponentially. At 20 N
  the pressure reaches the fiber wall's hardness (0.27–0.42 GPa by nanoindentation [M];
  sheet.rs `FIBER_WALL_MPA`). Past that, a real stylus starts to emboss and tear. The model
  only densifies [noted, not modelled].
- **The hand's force.** `pressure=` 0..1 is 0..5 N, as for the pastel pencil
  (draw_pencil.rs `FORCE_AT_FULL`; a hand draws at 2–3 N on a tablet, Xin, Bi & Ren 2012
  [M]). `force=` sets it in N.
- **The ball.** Tracing and embossing styluses come with balls of 1, 1.5, 2, 2.5 and 3 mm
  across (retail sets, e.g. Xiem XST01 at 1 and 1.5 mm) [M]. The default is 1 mm, the
  finest. A pencil used as the stylus presses with its point's width
  (`Lead::width_mm`).

## 3. The sheets

**Natural tracing paper** [M, D]:
- 90 g/m² measures 76 µm (ISO 534; HP's C3868A and Alps' natural tracing papers both state
  76 µm / 3 mil). Its opacity is 25 % (TAPPI T-425).
- That is 1.18 g/cm³. Against the fiber wall's 1.5 g/cm³, 21 % of it is pores, which is why
  it is transparent.
- At the drawing paper's fiber width (25 µm) and coarseness (0.16 mg/m), 90 g/m² is 14.1
  fibers deep. A caliper of 76 µm then makes the collapsed fibers 4.3 µm thick
  (`trace::tracing_paper`).
- Its z modulus is the drawing paper's, 39 MPa [E]. Nobody has measured tracing paper's.

**The drawing paper** (the sheet below, or the study used as its own back): the engine's
`Paper::drawing`, 300 µm caliper, 50 % pores.

## 4. The coat [P, D, E]

**How much.** Practitioners rub soft graphite (4B or softer) or chalk over the back of the
lines. They then work it into the tooth with a tissue in circles and shake off the excess
dust (watercolorpainting.com, "Watercolor technique for trying ideas with tracing paper"; a
WetCanvas forum thread on transferring from tracing paper) [P]. What stays is what the tooth
holds: its pores filled to the fiber tops. In the paper model the mean pore depth is
µ = t_f ε/(1 − ε) (`Paper::pore_um`; the mean of the per-pixel micro-roughness paper.rs
lays) [D]:
- the tracing paper holds 1.1 µm;
- a 160 g/m² drawing study, coated on its own back, holds 6 µm.

**What touches.** Where the ball's sink δ reaches into the sheet below, the coat touches
the share of that sheet's surface within δ of its fiber tops, 1 − e^{−δ/µ}. This is the
same exponential bearing curve as the pastel stick's (pastel.rs `bearing`) and the knife's
reach (sheet.rs `below`) [D].

**What comes away: `SPLIT` = 0.5 [E].**
- A bed of loose particles pressed between two papers, held about equally by both, parts at
  its weakest plane.
- If the bed is uniform, that plane is equally likely at any depth, so half the bed comes
  away on average.
- Nothing measured was found. Graphite transfer papers (Saral) are sold on their faint,
  erasable line, not on a transfer fraction.
- **The measurement that would set it:** weigh a coated tracing before and after a ruled
  set of lines at known force, and weigh the receiving sheet.

**The faint line, from the formulas.** At a line's middle the sink (70 µm) is far deeper
than either sheet's pores, so the coat touches nearly everywhere. Half of it comes away:
- From tracing paper: 0.57 µm, covering 9 % of the sheet (1 − e^{−0.57/6}, by pastel's
  coverage law, `PARTICLE_UM` = 6 µm). That is a faint line, as transfers are.
- From a drawing study's own back: 3 µm, covering 39 %.

**Conservation.** The coat that leaves the back lies in the sheet's tooth as loose grains
of what it was rubbed from: a pencil's flake gray and its eraser lift, or a stick's colour,
lift and carbon (charcoal stays bare carbon). The bookkeeping is exact on the receiving
sheet's pixel grid:
- the back holds µ over every pixel it covers;
- each line takes its share of what is still there;
- a crossing line finds less.

Tested (`what_leaves_the_back_lands_on_the_sheet`, and at the easel
`what_leaves_the_coat_lands_on_the_sheet_and_the_study_is_unchanged`).

**Graphite as a coat** is modelled as loose powder in the tooth, which rubbed graphite is.
Graphite's sheen is not modelled; no medium here has one.

## 5. The tracer's hand [M, D]

**Speed: the steering law.** Following a line is steering through a tunnel as wide as the
line.
- Accot & Zhai (CHI '97, "Beyond Fitts' law") measured it with a stylus on a Wacom UD-1825
  tablet. For the straight tunnel, MT = −188 + 78·(A/W) ms (r² = 0.968), with a 6.4 %
  error rate.
- The hand's speed grows linearly with the tunnel's width, v ≈ W/τ, with τ ≈ 50 ms
  (straight), 70 ms (narrowing) and 110 ms (spiral).
- The model takes b = 78 ms: a line of length L and width W takes L·b/W [M]. A 0.4 mm line
  is traced at about 5 mm/s.
- Each line also takes a set-down, the pencil's 0.3 s (`pace::PENCIL_LINE` [E]).
- The tunnels measured were 1–4.5 mm wide (20–90 px at 20 px/cm), wider than a pencil
  line. Using the law for 0.3–1 mm lines is an extrapolation [noted].

**Stray: the effective width.** ISO 9241-9 (and Soukoreff & MacKenzie 2004) set the
effective width of a pointing task at 4.133 σ, the width that holds 96 % of the hits. A
tracer steering at the law's speed keeps within the line about that often (Accot & Zhai's
6.4 % of trials crossed a wall somewhere along the whole path). So σ = W/4.133 [D]: 0.1 mm
on a 0.4 mm line.

**How fast it wanders.** In visually guided tracking the hand corrects in discrete
submovements, 2–3 a second (Miall, Weir & Stein 1993, J. Motor Behav. 25:53; the figure as
later papers quote it, e.g. Susilaradeya et al. 2019) [M]. At speed v a stray wanders over
v/2.5 Hz of the line: 2 mm at 5 mm/s. `Hand::follow` gives it as five sinusoids about that
length whose standard deviation is exactly σ (tested).

**Generations.** Through tracing paper the line is traced twice by hand (study → tracing
paper, tracing paper → sheet), so independent strays add: σ√2. Straight from the study, or
at a window, it is traced once.

**What is lost.** The tracer follows the line's middle, smoothed through its own path. Any
wobble in the study's line finer than about σ, or shorter than the wander, is replaced by
the tracer's own. A line fainter than `LEVEL` is not followed (§6).

## 6. Estimates and what is not modelled

- `LEVEL` = 0.25 of the guide's reading [E]: the faintest line a tracer follows. A firm line
  reads 1, a light 2H line about 0.5, and an eraser's ghost below that.
- **Not modelled:**
  - how visible a line is through tracing paper (its 25 % opacity) or through a sheet held to
    a window: no opacity limit is applied;
  - the dent a blind or heavy stylus leaves in the sheet;
  - the pencil's own line on the tracing paper's front;
  - the coat used up on the stick that rubbed it;
  - a pencil used as the stylus wearing.
- Where a line crosses wet paint it lays nothing, and the trace says how many pixels it
  skipped.
- **The seam for soft ground (P5):** `guide_paths`, `Hand` and `contact` are the same for a
  pencil pressed through a drawing onto a soft-grounded plate. What the contact does to the
  ground is P5's (plan-intaglio §7).

## Sources

- Accot, J., Zhai, S. (1997). Beyond Fitts' law: models for trajectory-based HCI tasks.
  CHI '97, 295–302. doi:10.1145/258549.258760 (PDF read: experiment 2, eq. 8, eqs. 22–24).
- Accot, J., Zhai, S. (1999). Performance evaluation of input devices in trajectory-based
  tasks. CHI '99, 466–472 (tablet and mouse fastest; coefficients not read).
- Soukoreff, R. W., MacKenzie, I. S. (2004). Towards a standard for pointing device
  evaluation. IJHCS 61:751 (the effective width, 4.133 σ; ISO 9241-9).
- Miall, R. C., Weir, D. J., Stein, J. F. (1993). Intermittency in human manual tracking
  tasks. J. Motor Behav. 25:53–63; rate as quoted by Susilaradeya, Xu, Hall, Lackner,
  Nachev (2019), eLife.
- Chen et al. (2020): paper's z-compression (as in paper_mechanics_and_transport.md §1).
- HP Natural Tracing Paper C3868A data sheet; Alps Natural Tracing Paper (90 g/m², 76 µm).
- Retail stylus sets (Xiem XST01: 1 and 1.5 mm balls; five-piece sets 1–3 mm).
- watercolorpainting.com, "Watercolor technique for trying ideas with tracing paper"; a
  WetCanvas forum thread, "Transferring from tracing paper" (graphite rubbed on the back
  and the excess wiped off).
