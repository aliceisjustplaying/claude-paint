# Cassatt's papers and mounts: the give of a mounted sheet, and named sheets

Research notes for `paper={mount=, preset=}` (crates/paint/src/paper.rs: `Mount`, `Drum`, `PRESETS`). Compiled 2026-11-15. It builds on:
- `paper_mechanics_and_transport.md` §1: z-compression of paper, and the Winkler bed the pastel contact uses.
- `paper_surface.md`: the fiber-network dials.
- notes/degas-materials.md §1: papers and mounting in her circle.
- The Cassatt research on branch `notes/cassatt-materials` (`notes/research/cassatt_materials.md` §2).

[M] marks a measured or catalogued value, [E] an estimate, and [D] a value derived here from [M] and [E] values.

## 1. Mounts: how much a laid-down sheet gives under a stick

### What she used [M]
Museum records give the mounts, and sometimes say they have changed:
- **Board.**
  - "Pastel on green wove paper, mounted on illustration board" (c. 1881) [Met 10424].
  - "Pastel on wove paper, mounted on cardboard" (c. 1897) [Met 10404].
  - "Blue-gray wove paper (faded to tan), mounted on board" [AIC 1968.81].
- **Canvas.** "Pastel on wove paper, mounted on canvas" (1895–1914) [Met 13533, 10403, 10387, 10396, 10397]. Some were taken off their strainers later [Met M&C].
- **Strainer.**
  - "Pastel on blue-gray wove paper (faded to tan), mounted on canvas, on a strainer" (1888) [AIC 1922.421].
  - "Originally mounted on a strainer" (1899) [Met 10389, 13532].
  - "Cassatt typically executed her pastels on paper that was adhered to canvas and tacked to a wood strainer", whereas most artists by then used millboard [Met M&C].
  - On one strainer-mounted Cassatt, the paper's edges were glued firmly to the strainer and the free bay moved with the climate until it tore at its weakest points [Choi & Makin].
- **Degas, for comparison.** He usually wrapped the sheet over 5–7 mm paperboard, and occasionally stretched it on a stretcher (notes/degas-materials.md §1).

### The model
The pastel contact (crates/paint/src/pastel.rs `seat`) treats the surface as a Winkler bed of stiffness k (MPa per µm). For a sheet alone on a hard drawing board, k = E_z / caliper (`Paper::z_mpa`; 39 MPa measured on board [M, Mann, Baum & Habeger]). A mount adds its own spring in series:

  1/k = caliper / E_z + 1/k_backing (+ the bay's drum on a strainer)

`Paper::mounted` folds the backing into `z_mpa`, so the sheet still gives one number per pixel. The strainer's drum depends on where the stick is (`Paper::give_at`). With no mount, the sheet behaves exactly as it did before mounts existed. Every backing below is derived from measured moduli under one reference contact; the estimates left are marked.

### The reference contact [D from M]
Each mount's give depends on the size of the patch the stick presses on (a nonlocal support has no single Winkler stiffness). The patch is one reference contact, the same for every mount:
- **Force** F = 2.5 N: drawing on a tablet, 2–3 N [M, Xin, Bi & Ren 2012; pastel_stick_tribology.md §5].
- **Patch**: the full facet of a round soft stick, R = 6 mm (Ø 11–13 mm), worn flat at altitude θ = 50° (a pen is held at about 50° [M, Schomaker & Plamondon 1990]). The facet is an ellipse of semi-axes R and R/sin θ (pastel_stick_tribology.md §4). Its equal-area radius is **a = R/√sin θ = 6.86 mm**, its area 148 mm² [D].
- **Pressure** p = F/πa² = **0.017 MPa** [D].
- **What the Winkler springs need.** The contact model gives each pixel its own spring, so a pixel's surroundings don't move unless loaded. The right spring is the patch's pressure over **how far the patch sinks relative to the surface round it**. For a layered bed (board, cloth) the surface a few mm away is undeflected, so this is the patch's own sink. For the drum it is the sink of the patch's middle below its rim. The whole bay also sinks (millimetres, below), but that carries the stick and the surface round it down together.
- **Units.** A sink of δ µm under p MPa is k = p/δ MPa/µm. Equivalently, a local stiffness F/δ (N/µm) over the patch's area. N/mm³ = MPa/mm = 10⁻³ MPa/µm.
- **Choice of patch** [E]. A fresh stick touches on a smaller crescent (10–50 mm² in the tribology note), and a smaller patch stiffens the drum and board. The full facet is the worn stick's, which is how a pastel is mostly used (it wears flat at once, §4 there). Making the patch follow the stick's actual face is a possible refinement (the estimates audit's M2): it would need every mount's give per stroke, but board and canvas are folded into the sheet's z modulus, and the paper section's checkpoint layout is shared with #317 and kept as it is. So the constant stays the worn facet, as the audit allows. Its 50° altitude is the measured pen altitude.

### Board: 0.015 MPa/µm [D]
- **Inputs.**
  - Thickness t = **2.3 mm** [M]. Her board mounts are catalogued as "illustration board" (Met 10424) and "cardboard" (Met 10404). A heavyweight illustration board (Crescent No. 100, 24-ply) is 0.09 in = 2.3 mm (0.085–0.095 in); a medium one (No. 300) is about 1.4 mm [M, retailers' specifications]. Degas's 5–7 mm boards are a separate case.
  - Through-thickness modulus E_z = **34 MPa** [M]: Stenberg's quasi-static through-thickness fit for paperboard, E_initial = 34 MPa (*Int. J. Solids Struct.* 40 (2003) 7483, parameter table). The 39 MPa of Mann, Baum & Habeger (1979) is an ultrasonic reading, stiffer than a stroke's slow loading (see the bending below). Whole-board values run as low as 17 MPa (Persson, quoted in KTH thesis diva2:1738574), a lower bound.
  - Out-of-plane shear G_xz = **32 MPa** [M]: G_initial from the same Stenberg fit, so that E_z and G_xz come from one board. It lies inside Nygårds's 30–85 MPa for paperboard plies (paper_mechanics_and_transport.md §1).
  - The paste or glue layer is tens of µm of a GPa-modulus film: its compliance is under 10⁻³ of the board's, so it is left out [D].
- **Model.** The board's plies are about 190× stiffer in plane than through the thickness (E₁₁/E₃₃ [M]). They barely stretch, so the board is a shear-coupled (Pasternak) layer on the table, as paper_mechanics_and_transport.md §1 derives for a sheet:
  - k = E_z/t = 14.8 N/mm³
  - shear G_xz·t/3
  - spreading length L = t√(G_xz/(3E_z)) = 1.29 mm
- **Under the patch** (uniform, radius a): the middle sinks (p/k)(1 − (a/L)K₁(a/L)), so k_board = k/(1 − 0.015) = **0.0150 MPa/µm** [D].
  - With the same moduli, 1.4 mm (medium illustration board) gives 0.024 and 5 mm 0.0084. A Hankel-transform check that adds the sheet's own bending on top (D below) changes it by under 1%.
  - Before the audit (3 mm, 39 and 50 MPa, the ultrasonic E_z): 0.0141.
- **The default sheet** (300 µm at 39 MPa, k 0.13) on board: 0.0135 MPa/µm, an equivalent z of **4.0 MPa** [D].
- **Old value** 0.1 MPa/µm [E], from a guessed ×10 spread. The measured shear spreads the load far less than that.

### Canvas: 0.00055 MPa/µm [D]
- **Inputs.**
  - A painting canvas is 0.3–0.6 mm thick [M]; t = 0.4 mm.
  - Woven fabric compacts as t ∝ p^(−B). A single woven flax (linen) ply goes from 1.345 mm at 1 kPa to 0.946 mm at 100 kPa, so B = ln(1.345/0.946)/ln 100 = **0.0764** [M, Rayyaan et al. 2021]. A warp-knit flax and a nonwoven tape lost the same ≈30% over that range [M].
- **Model.** The cloth is thin beside the patch (0.4 against 6.9 mm), so it is a Winkler spring E/t. For t ∝ p^(−B), the tangent modulus is E = −t dp/dt = p/B = **0.22 MPa** at the reference pressure. So k = **0.00055 MPa/µm** [D].
  - The sheet's bending over the cloth (spreading length (D/k)^(1/4) = 2 mm) changes the patch's sink by under 10% (Hankel check). It is left out.
- **The default sheet on canvas**: 0.00055 MPa/µm, equivalent z **0.17 MPa** [D]. The cloth dominates.
- **Bounds: this needs a bench measurement** [D, between bounds].
  - The data are dry flax for composites, not glue-sized linen with paste in its weave, which should be firmer. Dry flax is the **soft bound**: k = 0.00055 MPa/µm.
  - The **stiff bound** is a weave fully filled with dried paste and size, which then compacts like paperboard: E_z ≥ 17 MPa (the lowest whole-board value, under Board) over 0.4 mm, so k ≤ 0.04 MPa/µm.
  - A thin paste layer leaves most of the weave's interstices open, so the true value should sit near the soft end, within a factor of a few of dry flax. The engine keeps the soft bound, B = 0.0764, until a test sets it.
  - **The test.** Paste a 300 µm drawing sheet onto glue-sized linen with wheat-starch paste and let it dry. On a hard table, press a flat 13.7 mm indenter (the reference patch's diameter) at 2.5 N. The sink (expect 10–40 µm) gives k directly. The same rig on board would check the board's derivation.
  - At low pressure the cloth is softer still: the tangent ∝ p.
- **Old value** 0.015 [E], from a guessed packed-cloth modulus of 5–10 MPa. The measured compaction law gives a cloth 25× softer at a stick's pressure.

### Strainer: the bay's drum [D]
- **Tension** T = **0.13 N/mm** (130 N/m) [D from M].
  - A canvas stretched by hand reaches about 200–300 N/m, and loses 30–50% of it in the first 48 hours [M, Berger & Russell 1990, repeated measurements; their Fig. 1 canvas held 205 N/m]. A pastel is worked days after the sheet is tacked, so 200 × (1 − 0.35) = 130 N/m.
  - That lies inside the measured working band of canvas paintings, 0.12–0.24 kN/m [Heritage Science 2022]. "Optimal" is 1–2 N/cm [Academia.edu].
  - Before the audit: 0.2 N/mm, the fresh working value.
- **Bending** D = E t³/12 of the sheet [D].
  - Paper's in-plane modulus goes with its density. The measured E₁₁ = 190 × 39 = 7.4 GPa and E₂₂ = 3.47 GPa [M, Mann, Baum & Habeger 1979] have a geometric mean of 5.07 GPa. Over a board's density of about 700 kg/m³ [E: linerboard, 9 GPa at 700 kg/m³ MD], that is 7.24 MN·m/kg.
  - Those moduli are **ultrasonic**. Paper's ultrasonic engineering modulus is 25–40% above the mechanically measured one, because paper is viscoelastic and the two tests differ in time scale by orders of magnitude (17 µs against 0.24 s; Waterhouse, IPST Technical Paper 495, 1993, citing Habeger and Batten). A stroke loads the sheet over tens of ms, nearer the mechanical scale, so the modulus is divided by 1.3: **5.6 MN·m/kg** [D from M].
  - The drawing sheet (160 g/m², 300 µm, 533 kg/m³) then has E = 3.0 GPa and **D = 6.7 N·mm** (8.7 before the correction). The 45 g/m² Japan paper has 0.60 N·mm.
  - The canvas adds little bending: it is soft in plane and slackens on the compressed side. It is left out [E].
  - The old D ≈ 40 N·mm [E] was 5× too high even for its own inputs.
- **The bay as a plate.** The sheet on the canvas is a pre-tensioned plate, D∇⁴w − T∇²w = p, of bending length **ℓ = √(D/T) = 7.2 mm** (drawing sheet). Its point-load solution is w = −(F/2πT)[ln r + K₀(r/ℓ)]. For a uniform patch of radius a, β = a/ℓ, the closed forms of a disk's log potential and K₀ potential give the middle's sink below the rim:
  - **δ = (F/2πT)·h(β), h(β) = ½ − (2/β) K₁(β)(I₀(β) − 1)**
  - h(β) → ½ for a membrane (β → ∞), and (β²/4)(ln(2/β) − γ + 3/8) for a plate (β → 0).
  - A numerical Hankel transform of (2J₁(qa)/qa)(1 − J₀(qa))/(Dq³ + Tq) agrees to 10⁻⁹.
  - So k_bay = p/δ = **2T/(a² h(β))**. For the drawing sheet, β = 0.956, h = 0.172: **k_bay = 3.23 × 10⁻⁵ MPa/µm**, a sink of 530 µm across the patch [D]. (Before the audit, at 0.2 N/mm and 7.24 MN·m/kg: β = 1.04, h = 0.187, 4.55 × 10⁻⁵ and 370 µm.)
  - In series with the sheet and cloth: 3.05 × 10⁻⁵, equivalent z **0.0091 MPa**.
  - Old value 0.008 [E].
- **The bars.** The canvas runs over the bars and is held there: it can't sink, but it is free to turn (simply supported, w = ∇²w = 0 on a straight edge).
  - A mirror patch of opposite sign across each bar's edge meets that exactly, as does one of the same sign across each corner.
  - A mirror's membrane part (ln r) is harmonic over the patch, so it lowers the patch's middle and rim alike and leaves the sink unchanged (mean-value property).
  - Its bending part is (F/2πT)(2/β)I₁(β)K₀(r/ℓ) at distance r. Over the patch, that takes (I₀(β) − 1) times itself off the sink.
  - So **h(x) = h(β) − (2/β)I₁(β)(I₀(β) − 1)[ΣK₀(2d_edge/ℓ) − ΣK₀(2d_corner/ℓ)]**, and the give there is k_bay·h(β)/h(x).
  - This is exact for a patch clear of the bars (d > a). Nearer, it is the mirror's continuation, which firms the sheet up to the bar's own give at the bar's edge, without a step. (A rigid facet held up partly by the bar can't sink into the bay beside it.)
- **What falls out.**
  - The bay is as firm as canvas at the bar's edge, softens within a few bending lengths (the drawing sheet: within 10% of the bay's give 9 mm in from the edge), and then gives the same everywhere further in.
  - The bay's size drops out of the local give. It sets only the whole sheet's sink under the hand: the Navier series for a 600 × 500 mm canvas on 50 mm bars, under the patch at the centre, gives **9.8 mm**, a point stiffness of 255 N/m. That is the felt "drum". (Before the audit, on 40 mm bars at 0.2 N/mm: 6.6 mm and 380 N/m.)
  - Used as a Winkler spring (255 N/m over the patch, 1.7 × 10⁻⁶ MPa/µm), it would sink the stick 9.8 mm into a surface whose surroundings stay put. The real surroundings go down with it.
  - The old law (give growing as the log of the distance from the bar across the whole bay) was that global sink's shape, and is replaced.
- **Bars and crossbars are inputs:** `mount={"strainer", bar=<mm>, cross=<n>}`.
  - The bars are **50 mm** by default [M]. The original rectangular strainers of Jens Juel's portraits (Copenhagen, late 18th century) had bars 4–6 cm wide, measured from the creases and craquelure they left in the paint [Slotsgaard, *Arts* 11 (2022) 55]. Vermeer's are narrower (2.1–2.7 cm, RKD), but two centuries earlier. No measurement of her own strainers was found. A 60 × 80 cm sheet is a No. 25 *paysage* (81 × 60 cm).
  - n crossbars as wide divide the longer side into equal bays. The canvas is taken to rest on them as on the outer bars; crossbars set back from the canvas are not modeled.

### What a firmer bed does to the pastel laid [D]
**The question.** Painters say a firm support "bites". Does the contact model's direction (softer beds lay more) hold?
- **Archard.** The volume worn off the stick is K·F·s/σp: load and distance, not pressure.
  - The real contact is plastic on the soft stick (A_real = F/σp, Bowden–Tabor), so a softer bed spreading the load over more paper changes how many asperities touch, not the real area or the volume.
  - It is measured so for graphite: a fresh tip (Ø 0.5 mm) and a worn one (Ø 1 mm), 4× apart in nominal pressure, transfer the same volume (pastel_stick_tribology.md §2).
  - In the engine, Σ bearing × σp = F at every step, so the volume a stroke would wear off fresh tooth is the same on every mount.
  - A pressure-dependent wear coefficient (ploughing to cutting as asperities penetrate deeper, Hokkirigawa & Kato) would favor the firm bed. It isn't seen at these pressures for graphite and has no pastel data, so it is not added.
- **The tooth's peaks.** A fiber asperity is tens of µm across, under a 150–300 µm sheet. Its stress decays within about its own size (Saint-Venant), so what lies under the sheet can't change how an asperity meets the stick. The mount acts only at the patch's scale (mm).
  - So there is no derivable mechanism by which a soft bed lets the tooth "ride" rather than file the stick, and none is added.
- **What does change** is the third body. Pastel laid in the pores is loose, and the stick slides on it without abrading.
  - A stroke's face passes each point many times. On a firm bed the load sits on a small apparent contact, whose pores the stroke itself has just filled.
  - A soft bed spreads the same real contact over more fresh tooth.
- **The engine's numbers** (one stroke: 12 mm round soft stick, 30 mm at 3 N and 60°, the default sheet, a 300 mm canvas; re-run on this branch after the audit's constants).
  - The volume the same stroke would wear off unfilled tooth: **1.31 mm³ on every bed**.

  | bed | laid (mm³) | contact on the fresh sheet (mm², mean over the stroke's points) |
  |---|---|---|
  | hard drawing board | 1.04 | 1.7 |
  | board | 1.10 | 2.8 |
  | canvas | 1.16 | 14.3 |
  | strainer's bay | 1.16 | 72 |

- **Past canvas the laid volume levels off.** A sweep of the bay's give (the same stroke, the drum's give set by hand) shows the contact growing steadily, while what is laid stays at 1.13–1.17 mm³ with isolated dips:

  | bay's give (MPa/µm) | 5.5e-4 | 2e-4 | 1e-4 | 6e-5 | 5e-5 | 4.2e-5 | 3.05e-5 | 2.5e-5 | 2e-5 | 1e-5 |
  |---|---|---|---|---|---|---|---|---|---|---|
  | laid (mm³) | 1.13 | 1.15 | 1.14 | 1.02 | 1.17 | 1.17 | 1.15 | 1.06 | 1.16 | 1.14 |
  | contact (mm²) | 22 | 29 | 44 | 51 | 59 | 61 | 77 | 79 | 95 | 124 |

  - The dips are the same with other paper seeds and finer stroke sampling. Their cause is not known: they may be a numerical artifact of the contact or a real effect of the stick's shape. This is an open question.
  - So the model gives no order between canvas and a strainer's bay. The bay laid 1.167 against canvas's 1.159 before the audit and 1.157 after it, both within this scatter.
  - The test `pastel::tests::a_softer_bed_lays_more_within_archards_wear` checks hard board < board < canvas, that the bay lays more than a hard board, and the Archard bound on every bed.
- **Conclusion.**
  - The model's direction is right. It is bounded by Archard: a softer bed can only recover what the filled tooth withholds, here at most 21%. Past canvas, nothing more is recovered.
  - The painters' "bite" is then not more pastel per stroke. It is more likely:
    - the narrower, crisper contact of a firm bed (1.7 against 72 mm² here);
    - the hand not losing millimetres of travel to the drum;
    - the drum's rebound under a quick stroke, which is dynamic and not modeled.
  - No source quantifies it.

### Order
For any sheet, stiffest first:
1. no mount (a hard drawing board)
2. board
3. canvas, which a strainer equals over its bars and within about a bending length of them
4. a strainer's bay, which gives a little more at every step inward over the next few bending lengths, then the same throughout

The tests are `paper::tests::mounts_order_the_give`, `paper::tests::the_mounts_are_their_derived_numbers` and `pastel::tests::a_stick_sinks_further_on_a_strainer_than_on_canvas_or_board`.

## 2. Named sheets

### Her papers [M]
- **Tones.** Tan, buff, brown, green and blue are recorded, and late in life she preferred blue [cassatt_materials.md §2; Met M&C].
- **Fading.** Several blue and blue-gray sheets have faded to tan:
  - "wove paper (originally blue)" [Met 10405]
  - "blue-gray wove paper (faded to tan)" [AIC 1922.421, 1968.81]

  So some sheets catalogued as tan today may once have been blue. Degas's aniline-dyed papers faded the same way (notes/degas-materials.md §1).
- **Wove and laid.**
  - Wove sheets are the rule.
  - A laid sheet [Met 13532].
  - Off-white and ivory laid papers for the color prints, watermarked "ED&Cie" and "PL BAS" [cassatt_materials.md §5].
- **Japan paper** for the counterproofs. One NGA counterproof is "pastel over pastel counterproof on chine collé" [NGA 1980.45.9], and a dealer's sheet of the same subject is "pastel counterproof on Japan paper" [Cavalier].
  - Kozo fibers average about **10 mm** (10–15) by **30 µm** [M, CAMEO]. Gampi is about 3 mm by 12 µm [M, Melbourne fibre atlas].
  - Printmaking gampi is 28–42.5 g/m², and lining tissues 10–20 g/m² [M, Japanese Paper Place; Ozu].

### The dials [E]
- **The colored wove sheets.** No measurements of these sheets were found. They were commercial pastel papers, machine-made from wood pulp with some rag. The preset uses estimates for such a sheet:
  - 130 g/m²
  - mean fiber 1.6 mm (softwood 2–3.5 mm with hardwood and groundwood fines; paper.rs)
  - a light felt mark (0.7 mm cells, 14 µm deep)
  - calender 0.25
  - otherwise the drawing paper's values
- **Brown wove.** An unbleached wood-pulp sheet: heavier (150 g/m²), coarser (0.2 mg/m), shorter fibers (1.4 mm), cloudier (floc 0.6), a deeper felt mark.
- **Cream laid.** A rag laid sheet:
  - 110 g/m² (Ingres-type papers about 100–120)
  - fiber 2.2 mm
  - laid lines 9 per cm, chain lines 25 mm apart, a 0.15 fiber deficit over the wires (paper.rs's handmade range)
  - a felt mark
- **Japan paper** (`"japan paper"`, named for the sheet as the others are). Long fibers, thin and smooth:
  - 45 g/m² (the dial's floor is 40)
  - fiber 10 mm (the dial's ceiling) by 30 µm
  - coarseness 0.1 mg/m
  - porosity 0.6 (an open, bulky sheet)
  - well formed (floc 0.15)
  - no felt (the sheets are brushed onto drying boards)
  - calender 0.6 (smooth)
- **Not set.** Every preset keeps the drawing paper's absorbency (0.8) and give (39 MPa), because no source gives either for these sheets. A mount changes the give.
- **Absorbency needs a bench measurement** [D, between bounds]. The dial is the share of the sheet's pore volume that takes oil (paper.rs `absorbent`).
  - Upper bound **1**: a waterleaf (unsized) sheet, all its pores open. Oil wets cellulose completely (≈ 34 against 48–57 mN/m, paper_surface.md §4).
  - Lower bound: a hard gelatin-sized sheet, whose size fills part of the surface pores. Sizing slows water much more than oil, so this bound is well above 0 for oil, but no measurement of its value was found.
  - 0.8 sits between them, toward the open end, as for a lightly sized sheet. A Cobb test (ISO 535) gives water uptake. Oil uptake on period-type papers would need an oil-drop or oil-Cobb test. Nothing published was found for these sheets.

### Tones: measured from museum images, then fitted to tubes [E]
- **Method.** I took Wikimedia Commons reproductions (960 px) of museum photographs. For each, I took the median sRGB over patches where the paper is bare. Medians drop the pastel flecks.
- **Caveats.** These are photographs of sheets as they are now:
  - white balance and the photographer's color management are unknown
  - colored papers fade (above)
  - Japan paper yellows with age

  Every tone here is therefore an estimate.

| sheet | target sRGB | from |
|---|---|---|
| tan wove | #918262 | measured: NGA 1985.64.81 (*The Black Hat*, "pastel on tan wove paper"). Lower-right reserve (145, 130, 98); the edge patches carry gray pastel, (131–141, 126–131, 112–113) |
| green wove | #9ca296 | measured: Met 10424 (*Woman on a Bench*, "green wove paper"), four reserves: #97a095, #9ca296, #9fa194, #a0a699. It reads gray-green now; it may have faded, as Degas's greens did |
| blue wove | #6b86a8 | estimate: an unfaded mid blue. I found no bare reserve of a blue sheet in an image I could measure |
| gray-blue wove | #8a96a0 | estimate: AIC's "blue-gray" before fading |
| buff wove | #c9b68f | estimate |
| brown wove | #7f6248 | estimate. A High Museum photograph of a sketch "on brown paper" measured #a36a6f (pink): an unreliable color photograph, not used |
| cream laid | #e4dac2 | estimate: ivory laid |
| japan paper | #ebe3cc | estimate of the sheet as made. The NGA counterproof's sheet measures #dbc79a today (three reserves: #dbc79a, #dbc698, #ddc99c), aged and yellowed |

**Fitting the tones.** Each target was fitted to parts of the catalog's tubes, as the engine dries them (engine 9's `Palette::dry_color` of the "every" box). The fit was a grid search over up to four tubes, with ΔE in CIELAB and a small penalty for each extra tube. A preset's tone is computed from the catalog whatever the painting's box, since a dyed sheet isn't made from the painter's tubes. An engine 5–8 painting dries the same parts a little differently.

| sheet | tone (parts) | engine 9 dry color | ΔE |
|---|---|---|---|
| blue wove | lead white 1, ultramarine blue 1, bone black 0.45 | #6d86a7 | 0.9 |
| gray-blue wove | lead white 1, ultramarine blue 1, viridian 1.5, red earth 1.5 | #8995a0 | 0.3 |
| tan wove | yellow ochre 0.2, bitumen 1, vine black 0.23 | #9d866c | 5.2 |
| buff wove | lead white 1, yellow ochre 2.2, raw umber 3, Prussian blue 0.02 | #cdb891 | 1.2 |
| green wove | lead white 1, yellow ochre 0.65, viridian 0.45, bone black 0.45 | #9ba296 | 0.3 |
| brown wove | raw sienna 1, bitumen 3 | #7d5f48 | 2.0 |
| cream laid | lead white 1, yellow ochre 0.15, raw sienna 0.02, green earth 3 | #e5dac3 | 0.9 |
| japan paper | lead white 1, yellow ochre 0.1, raw sienna 0.1, viridian 0.02 | #eae1ca | 0.6 |

The tan sits 5 ΔE lighter and less yellow than the photograph. The dry earths can't reach that chroma at that lightness, so it is the closest the tubes give.

## Gaps
- No measured grammage, fiber or surface data for her colored papers; the dials are estimates.
- No measurement of a mounted sheet's give to check the derivations against: no indentation data on paper laid on board or canvas, and none on a tensioned canvas under a point load at a stick's scale.
- No compaction data for glue-sized, paste-filled painting linen (the cloth's law is dry flax's, the soft bound; the bench test is under Canvas), and no bending stiffness measured on artists' papers (D comes from board's measured ultrasonic moduli, scaled by density and brought to a mechanical time scale).
- No oil-absorbency measurement on period papers (the bounds are under the dials).
- Whether the dips in the laid volume over a very soft bed (under "What a firmer bed does") are numerical or physical.
- The tones still estimated (blue, gray-blue, buff, brown, cream laid, Japan paper as made) could be measured from bare reserves in open-access museum images, by the median-patch method used for tan and green. The AIC's blue-gray sheets (1922.421, 1968.81) have faded; an unfaded edge under a former mat line would be the target. The AIC's API was not reachable from here (a bot challenge).
- Shelley (1989) describes *Woman on a Bench* as "green laminated paperboard" (BPG Annual 16), so the green sheet may want a board mount by default. The full technical description of her papers is Bertalan, "Cassatt's Paper: Finding the Right Sheet" (2008), not reached.
- The reference patch is a worn stick's full facet; a fresh stick's smaller patch would meet a firmer drum and board.
- Blue tones are unmeasured. The best next step is a bare reserve in a high-quality museum image of a late blue sheet (the Met's 1914 *Mother and Child* is mostly covered).
- Keys, slack canvas (tension loss), crossbars set back from the canvas, and the drum's dynamics (rebound under a quick stroke) are not modeled.

## Sources
- [Met 10389, 10403, 10404, 10405, 10387, 10396, 10397, 10424, 13532, 13533] The Met Collection API object records (medium lines), https://collectionapi.metmuseum.org/public/collection/v1/objects/{id}, read 2026-11-15.
- [Met M&C] "Mary Cassatt in a Modernist Light: A Close Look at Mother and Child", The Met, *Perspectives*: https://www.metmuseum.org/perspectives/mary-cassatt-modernist-mother-child (via search summary).
- [AIC 1922.421] *Mother's Goodnight Kiss*: https://www.artic.edu/artworks/81527/mother-s-goodnight-kiss (via search summary); [AIC 1968.81] *Sleepy Nicolle*: https://www.artic.edu/artworks/28826/sleepy-nicolle (via search summary).
- [NGA 1985.64.81] *The Black Hat*: https://www.nga.gov/artworks/66479-black-hat; image: https://commons.wikimedia.org/wiki/File:Mary_Cassatt,_The_Black_Hat,_c._1890,_NGA_66479.jpg
- [NGA 1980.45.9] *Sara Wearing a Bonnet and Coat*: https://www.nga.gov/artworks/57658-sara-wearing-bonnet-and-coat; image: https://commons.wikimedia.org/wiki/File:Mary_Cassatt,_Sara_Wearing_a_Bonnet_and_Coat,_c._1904-1906,_NGA_57658.jpg
- [Cavalier] Cavalier Galleries listing, "pastel counterproof on Japan paper", 62.2 × 45.7 cm (via search summary).
- Met 10424 image: https://commons.wikimedia.org/wiki/File:Woman_on_a_Bench_MET_DP-13450-001.jpg
- High Museum sketch on brown paper (not used): https://commons.wikimedia.org/wiki/File:Sketch_of_a_Mother_Looking_Down_at_Thomas_by_Mary_Cassatt,_c._1893,_pastel_on_brown_paper,_High_Museum_of_Art.jpg
- [Choi & Makin] "Treatment and Housing Techniques for Pastel Paintings on Paper", AIC Book and Paper Group Annual 32: https://cool.culturalheritage.org/coolaic/sg/bpg/annual/v32/bpga32-06.pdf (via search summary).
- [Heritage Science 2022] "Mechanical properties and moisture-related dimensional change of canvas paintings – canvas and glue sizing": https://www.nature.com/articles/s40494-022-00794-3 (0.12–0.24 kN/m; 0.2 kN/m working value).
- [Academia.edu] "About the choice of tension for canvas paintings": https://www.academia.edu/8773879/about_the_choiche_of_tension_for_canvas_paintings (1–2 N/cm).
- [Berger & Russell] G. A. Berger and W. H. Russell, "Deterioration of Surfaces Exposed to Environmental Changes", JAIC 29(1), 1990, 45–76: https://cool.culturalheritage.org/jaic/articles/jaic29-01-004.html (hand stretching to 200–300 N/m; 30–50% tension loss in the first 48 h; read 2026-11-15).
- [Stenberg 2003] N. Stenberg, "A model for the through-thickness elastic–plastic behaviour of paper", *Int. J. Solids Struct.* 40 (2003) 7483–7498, parameter table (E_initial 34 MPa, G_initial 32 MPa): https://www.researchgate.net/publication/223388379 (table read via search summary). Persson's whole-board values (down to 17 MPa) as quoted in the KTH thesis https://www.diva-portal.org/smash/get/diva2:1738574/FULLTEXT01.pdf.
- [Waterhouse 1993] J. F. Waterhouse, "The Ultrasonic Testing of Paper and Board: Principles and Applications", IPST Technical Paper Series 495 (ultrasonic engineering modulus 25–40% above the mechanical one): https://repository.gatech.edu/server/api/core/bitstreams/487753d4-9757-43d1-9979-3f26bec5a8fa/content
- [Crescent] Crescent No. 100 illustration board, 24-ply, 0.09 in (2.3 mm): United Now, Aboveground Art Supplies, FLAX listings (via search summary); https://www.unitednow.com/Crescent-100-Illustration-Board-24-Ply-Cold-Press-15x20
- [Slotsgaard 2022] T. L. Slotsgaard, "Tracing the Oval: Characterisation and Construction of Original Danish Eighteenth-Century Strainers from Portraits by Jens Juel", *Arts* 11(3) (2022) 55 (rectangular strainer bars 4–6 cm): https://www.mdpi.com/2076-0752/11/3/55 (via search summary); RKD, Counting Vermeer §2.5 (Vermeer's 2.1–2.7 cm bars, as cited by the estimates audit).
- [Shelley 1989; Bertalan 2008] as cited in "Construction Paper", BPG Annual 16, https://cool.culturalheritage.org/coolaic/sg/bpg/annual/v16/bp16-07.html, and Dirda, BPG Annual 35 (2016), https://cool.culturalheritage.org/coolaic/sg/bpg/annual/v35/bpga35-18.pdf
- [Kawabata-system reviews] KES-FB3 compression (EMC, LC): https://www.sciencedirect.com/topics/engineering/kawabata-evaluation-system; Utkun 2021, cotton fabrics on KES (ResearchGate 357374410).
- [Rayyaan et al. 2021] "Compaction behaviour of various flax fabric structures during composite manufacturing", arXiv 2110.06110, Table 2 (single-ply thickness at 1 and 100 kPa): https://arxiv.org/pdf/2110.06110
- [Nygårds 2008] paperboard ply shear moduli, as tabulated in paper_mechanics_and_transport.md §1.
- [Xin, Bi & Ren 2012; Schomaker & Plamondon 1990] hand force and pen angle, as cited in pastel_stick_tribology.md §5.
- [Hokkirigawa & Kato] wear-mode maps (degree of penetration), cited for the mechanism only.
- Linerboard modulus and density (9 GPa MD at 700 kg/m³): ScienceDirect topic "Tensile stiffness", https://www.sciencedirect.com/topics/engineering/tensile-stiffness
- [Mann et al.; Kananen et al.] as cited in paper_mechanics_and_transport.md §1.
- [CAMEO] Kozo: https://cameo.mfa.org/wiki/Kozo; [Melbourne fibre atlas] Gampi: https://cultural-conservation.unimelb.edu.au/gampi.html; [Japanese Paper Place] https://www.japanesepaperplace.com/news/washi-and-printmaking/; [Ozu] https://ozuwashi.com/pages/faq
