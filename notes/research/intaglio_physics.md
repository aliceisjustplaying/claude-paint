# Intaglio printmaking: the physics of drypoint, grounds, acid, aquatint, wiping and the press

Research for gap 3 of notes/cassatt_support.md (branch `notes/cassatt-materials`): Cassatt's soft-ground, drypoint, aquatint and color prints of 1879–1894, and her one monotype (gap 12). The plan that uses this note is notes/plan-intaglio.md. Compiled 2026-11-15.

**Tags.**
- **[M]** measured, with a cited source.
- **[D]** derived here from physics or from measured numbers; the arithmetic is shown.
- **[P]** a practitioner's report: workshop times and rules of thumb from printmakers' manuals and suppliers. These are observations, not measurements. They are good enough to check a model against, but not to set a constant.
- **[E]** estimate. Used only where neither physics nor a measurement sets the number. The reason is given each time.

**Bottom line.**
- **Drypoint.** A steel needle drawn over copper ploughs: it pushes metal aside into a burr without cutting a chip. That holds because the needle is pulled with its leading face at a low attack angle, below copper's measured critical angle of 45–85° [M]. At a hand's 0.5–5 N the groove is about **40–170 µm wide and 20–100 µm deep** on half-hard copper [D, from scratch hardness]. With nothing removed, the burr's cross-section is about the groove's [D]. An unfaced copper drypoint gives **about 10–40 good impressions** with burr. Steel facing raises that to about 100 [P]. That ratio is about what Archard wear predicts from the hardness of electrodeposited iron against copper [D].
- **Etching.** Ferric chloride on copper is limited by how fast Fe³⁺ reaches the metal (activation energy about 25 kJ/mol, rate peaking near 2–2.25 M) [M]. A still tray at 20 °C bites **about 0.5–2 µm/min** [D], which matches the workshop times: a faint line in 5–15 min, black in about 40 min [P]. Biting is isotropic, so lines widen by about their depth [D/P]. Nitric acid on zinc is several times faster, gives off gas, and undercuts [P].
- **Aquatint.** In a dust box, settling sorts the rosin by size (Stokes): a plate put in after t seconds takes grains up to about 60 µm (5 s) or 18 µm (60 s) [D]. The working coverage is 40–60 % [P]. The tone comes from the ink held between the dots. It rises with bite until undercut frees the dots; then it falls ("overbitten aquatints print lighter") [P/D].
- **Wiping.** Starched tarlatan bridges a gap as a stiff sheet: its sag goes as **w⁴**. It leaves ink in lines under a millimetre wide and wipes open areas of several millimetres clean except at their edges [D, from fabric bending rigidity]. Inside a fine line, the yarns dip in by r − √(r² − w²/4) [D, the same geometry as notes/research/pastel_brushing.md].
- **Transfer.** At most **about 45–48 %** of the ink in a groove goes to non-absorbent paper, and **60 % or more** to absorbent paper [M, Aoki & Murakami 1962; Pritchard & Finkle 1964]. Transfer rises with the groove's fill and with how far the paper is pushed into it [M].
- **The press.** A hand etching press is rated at about **130 N/mm** of roller (6 t over 45 cm) [M, maker's rating]. Under the felts that is a few MPa to about 15 MPa [D]. Banknote presses run 100–1000 N/mm and 30–250 MPa [M].
- **Damp paper.** A damp pack holds about **0.5–0.6 g of water per g of paper** [D from a printer's recipe]. Paper expands by **0.05–0.12 % per % of moisture** while its fibers take up water [M]. The wet expansion of a sheet is about 1–2.5 % [M], so between dry and damp a 40 cm sheet changes by millimetres. Registration needs every plate printed while the sheet is equally damp.

---

## 1. Drypoint

### 1.1 The metal [M]
- **Copper.** Cassatt's plates were copper (research §5).
  - C110 copper sheet: annealed O60 is Rockwell F40, about 40–50 HV; half-hard H02 is Rockwell B40, about 80–100 HV, tensile 290 MPa, yield 250 MPa [Alcobra; MakerStage; HV from standard conversion tables].
  - Printmakers today use ⅛-hard to half-hard copper. One reports that the extra hardness of half-hard gives "finer details, cleaner cuts and a wider tonal range" [Tuckamore Press, P].
  - Bosse (1645) warned against copper that is too soft [Tuckamore, quoting Bosse].
- **Zinc.** Rolled printmaking zinc is softer, about 40–50 HV (standard tables). Aquatint on zinc is "particularly sensitive" to wear [Kiekeben manual, P].
- **Steel facing.** From 1857, a thin layer of iron was electroplated on the plate [Wikipedia, Etching revival]. Electrodeposited iron is about 150–500 HV, depending on the bath (general electroplating data, not checked against a primary source here).
- **Elasticity.** E is about 120–130 GPa for copper and about 100 GPa for zinc. E/H is about 140 for half-hard copper, so a scratch is almost all plastic and recovers little elastically.

### 1.2 Ploughing or cutting [M]
- **The regimes.** Whether a hard tip moving over a ductile metal ploughs (pushes metal aside, nothing removed), forms a wedge, or cuts a chip depends on its attack angle β, the angle of its leading face to the surface, and on the friction at the interface (the Challen–Oxley slip-line fields) [Challen & Oxley 1979].
- **Copper's critical angle.** Sedriks & Mulhearn measured the critical attack angle for copper, α-brass, lead, nickel and aluminium. It is **45° < β_c < 85°**. Below it, "prows are formed and no material is removed in theory"; above it, "ribbons or chips are produced" [Sedriks & Mulhearn 1963, 1964, as summarized in the toughness paper on ScienceDirect].
- **Spherical tips.** For a round tip, Hokkirigawa & Kato's degree of penetration (Dp = depth / contact radius) sets the mode. Brass goes from ploughing to wedge at Dp ≈ 0.17 and from wedge to cutting at 0.23 (unlubricated). Carbon steel goes at 0.12 and 0.23 [M, Hokkirigawa & Kato 1988].
- **Drypoint against burin [D].**
  - A cone of half-angle θ, tilted by τ toward the direction it moves (pulled, top leaning ahead of the point), meets the copper with its leading face at β = 90° − θ − τ.
  - A drypoint needle (θ ≈ 15–30°) held like a pen at 45–60° to the plate, so τ ≈ 30–45°, gives β ≈ 15–45°. That is below copper's β_c, so it ploughs: a burr, no chip. This is the practice: the needle is pulled, and it throws up a burr.
  - A burin is pushed nearly flat, with its rake face presented at a high attack angle, and cuts a chip that leaves a clean groove. That is why an engraving has little burr (and the engraver scrapes off what there is).
  - Drypoint and engraving are therefore the two sides of the same measured transition.
- **The burr's side [D].** Sideways tilt of the needle throws the burr to the side away from the tilt. An upright needle throws ridges to both sides. Printmakers report this [P: drypoint manuals; Wikipedia, Drypoint; WGA: "if the needle is perpendicular to the plate, it throws burr on both sides"].
- **Stream by stream [D] (`--@ intaglio-contact`).** β_c was measured with a plane face meeting the metal square on (Sedriks & Mulhearn's pyramids, a face leading). A needle's face is a cone, and each stream of metal across the groove meets it at its own slope along the way the needle moves: tan α = −n_x/n_z for the face's outward normal n where the stream reaches the surface. On an upright cone that is cot θ·cos φ. It is steepest at the very front (90° − θ) and lies back toward the flanks.
  - Taking β_c = 60° (the middle of 45–85°), the streams within |u| < a·√(1 − (tan β_c·tan θ)²) of the groove's middle cut and the rest plough: a mixed mode, as Wang & Subhash model for a conical tool [Wang & Subhash 2002].
  - An upright 30° needle then meets the metal at 60° only along its front line. It ploughs and throws burr on both sides, as the printmakers say. The single plane-face test (β = 90° − θ − τ ≥ β_c) made it cut.
  - A fine point does cut upright: 98 % of a 15° cone's groove at 2 N. Pushed, a 30° cone's middle cuts (95 % of the groove at 35° of lean) and its flanks plough.
  - Within the rounded point the face is the sphere's, sloping a·cos φ/(R − d) at the edge, so a light line ploughs even when pushed. That agrees with the low degree of penetration in Hokkirigawa & Kato's sphere tests.
  - Leaning the needle tilts the cone. The engine finds the streams' slopes from the tilted cone's normals, so leaning to the side counts too.
  - That the streams cut or plough independently is the estimate here [E]. Wedge formation between the two is not modelled.

### 1.3 Groove size against force [D]
Scratch hardness for a cone: the load is carried by the front half of the contact, so H_s ≈ 8F/(πw²). For metals H_s is close to the indentation hardness H (within about ±30 %). So w = √(8F/(πH)), and depth d = w/(2 tan θ).

| plate | F | w, θ = 45° | d, θ = 45° | d, θ = 30° |
|---|---|---|---|---|
| half-hard Cu (0.9 GPa) | 0.5 N | 38 µm | 19 µm | 33 µm |
| | 1 N | 53 µm | 27 µm | 46 µm |
| | 2 N | 75 µm | 38 µm | 65 µm |
| | 5 N | 119 µm | 60 µm | 103 µm |
| annealed Cu or Zn (0.45 GPa) | 1 N | 75 µm | 38 µm | 65 µm |
| | 5 N | 168 µm | 84 µm | 146 µm |

- **Force.** A hand presses a pen or stick with about 0.5–5 N (the pastel forces in notes/easel_guide.md). Drypoint is described as hard work, so 1–5 N.
- **Width scales as √(F/H).** So drypoint on zinc or annealed copper is wider and deeper at the same hand, by √2.
- **The tip.** A worn needle is a sphere of radius R at its point, not a sharp cone. Below a depth of about R(1 − sin θ) the groove follows the sphere, and a sphere at shallow Dp ploughs even more readily (§1.2). A needle's tip radius is not published. **[E] 10–50 µm** for a needle sharpened on a stone, from the grit size of sharpening stones (about 10–40 µm); measure it.

### 1.4 The burr's volume and shape [D] [E]
- **Volume.** In pure ploughing nothing is removed: the volume of the two ridges equals the groove's minus the small elastic recovery (degree of wear below 0.15 in the ploughing mode [M, Hokkirigawa & Kato via the Twente thesis]). A 1 N line on half-hard copper (θ = 45°) displaces about 700 µm² of section into its ridges.
- **Height [E].** No profilometry of drypoint burr was found.
  - Finite-element scratch studies show pile-up height growing with the rheological factor (E/σ_y)·tan β [M, Bucaille et al. 2001, qualitative only].
  - Copper's high E/σ_y puts it in the fully plastic, large pile-up regime.
  - A ridge of triangular section with base b and height h holds bh/2. If the burr of one side is about as tall as it is wide at its root (a thin, ragged fin, as drawings of drypoint sections show it), h ≈ √(2·A_side) ≈ 25 µm for the 1 N line.
  - **[E] burr height 0.5–1 × groove depth**, standing on one or both sides. This is the most important unmeasured number in the note: see Unknowns.
- **Shape.** The burr is a work-hardened, torn fin with rough faces, leaning away from the groove [P: descriptions; Wikipedia].

### 1.5 How the burr holds ink [D] [P]
- **The lee.** Ink collects behind the burr. The direction of wiping matters: "wiping along the lines with the hand can take out most of the ink and leave a light gray line" [P, wiping guides].
- **Why [D].** The burr stands above the plate surface as a fence. A cloth or palm moving across it is held off the plate for some distance past it.
  - A cloth of bending stiffness B pressed at p lands again a distance of about (B·h/p)^(1/4) after an obstacle of height h. That is the same plate-bending scale as §5.3.
  - For starched tarlatan (B ≈ 10⁻⁵–10⁻⁴ N·m, §5.3), a 25 µm burr at 10 kPa gives about 0.4–0.7 mm.
  - That treats the cloth as resting on the plate with nothing between. Its face is a compressible layer of crimped yarns, squeezed over the same tens of µm as the burr. Under the contact laws (§5.6, "Beside a burr") the softened cloth lies on that face: it never stands clear beside a drypoint burr, but bears less of the pad's pressure there and takes the ink more slowly, a fine halo.
- **What prints.** A band of ink on the plate surface beside the line, softest away from the burr, plus the ink caught in the burr's rough faces. This is the velvety, soft-edged drypoint line: wider than the groove, darker on the burr's side.
- **The burr itself.** It stands proud of the plate, so it presses into the damp paper and embosses it, carrying ink on its faces.

### 1.6 Wear over an edition [P] [D]
- **Measured editions** [P: dealers' and museums' statements]:
  - unfaced copper: "from twenty to forty good drypoint impressions" [Harvey-Lee], "ten or twenty impressions with burr" [Wikipedia, Drypoint], "no more than 15–20" [Masterworks], "no more than a dozen" (attributed to the Met)
  - after the burr has gone, "the comparatively shallow lines will wear out relatively quickly" [Wikipedia]
  - "proofs prior to steel facing are always richer" [Harvey-Lee]
  - steel-faced: "a hundred or so impressions without signs of deterioration" [Harvey-Lee]
  - plates of Renoir and Morisot reprinted after the burr was gone print as "ghosts" [Wikipedia, Etching revival]
- **Mechanisms [D].**
  - **Flattening in the press.** The fin is pressed down by the paper and felts at MPa pressures (§6). A thin fin of root thickness t bends once the press exceeds its plastic moment, σ_y t²/4 per length. The tallest, thinnest parts go first, so the first proofs are richest.
  - **Abrasion by wiping.** Tarlatan and a palm with whiting (calcium carbonate, Mohs 3, about as hard as copper) polish the burr at every inking. By Archard's law the volume lost goes as K·F·s/H.
- **Steel facing [D].** Raising H from about 90 HV (copper) to about 300–500 HV (deposited iron) cuts the Archard wear rate by 3–5×. That takes an edition of 20–30 to about 60–150, the "hundred or so" reported. The check is order-of-magnitude, since the wear coefficient K also changes between metals.
- **Zinc [D].** At about half copper's hardness, zinc should give about half the burred impressions, 5–15. That agrees with the claim of 25–30 for zinc against about 100 for copper [Grokipedia, weak: probably faced copper].
- **The engine's fin and the press [D].** The engine's burr is a triangle of the groove's section, about as wide at its root as it is tall (2 N: 29 µm high, 43 µm wide). Pressed by felts and damp paper at a few MPa, its top bears at most about πp (the contact's gap closing on it, as the palm's does), some 10 MPa against copper's 880 MPa. Such a fin cannot be crushed in the press: it wears only by abrasion, or by bending if it is far thinner than it is tall. Two pulls from one plate print the same and leave its burr as it was. The burr's real section is Unknown 1, and wear waits for it (phase P7).
- **A model.** With burr height h_n after n impressions, h_n = h_0·exp(−n/N). N is set by the hardness, with **N ≈ 10–20 for copper** from the editions above. This is a fit to practitioners' counts, not a measurement of h; see Unknowns.

### 1.7 Burnisher, scraper, roulette [D] [P]
- **Burnisher.** A polished steel tool rubbed under pressure. Where its contact pressure exceeds the copper's hardness the metal flows, smoothing high spots into the hollows nearby: burr flattened, shallow lines and aquatint closed, so the area prints lighter. A rounded burnisher of radius 5–20 mm, at a hand's 10–50 N, spreads its load over about a millimetre [E], and the mean contact pressure has to reach H (about 0.9 GPa) for the metal to flow. So burnishing works on the highs of a rough patch first: burr, then the dots of an aquatint, then the plate's surface.
- **Scraper.** A three-sided blade held to cut, at a high attack angle (§1.2). It shaves the plate's surface down, removing burr and lines with it. It leaves a shallow dish that is then burnished. A scraped dish lies below the plate's surface, so the press reaches it less: printers' manuals describe scraped corrections that print a little grey until they are burnished and levelled.
- **Roulette.** A toothed wheel rolled over the plate (or the ground). Each tooth indents a pit at Brinell-like pressure H and throws up a little ring of burr. The pit pitch is the tooth pitch, and the pit depth comes from the force per tooth in contact against H. Roulettes are one of the tools for tone in crayon-manner prints. Cassatt is not known to have used one [research §5: not mentioned].

## 2. Grounds

### 2.1 Hard ground [P] [D]
- **Composition.** Beeswax, asphaltum and rosin, for example two parts asphaltum, two of beeswax and one of resin [MFA CAMEO; UMBC; Beguin via Polymetaal]. Commercial ball grounds are bitumen, pitch resin and beeswax.
- **Laying.** Melted onto a warm plate and rolled out [UNSW].
- **Thickness [D/E].** The workshop test is optical: "the colour of strong tea rather than coffee" [UNSW].
  - Asphaltum absorbs strongly in the blue and green. A film seen over polished copper is crossed twice. If "strong tea" means a transmittance of about 0.3–0.6 at 550 nm, the film is a few µm thick.
  - **[E] 2–8 µm**: no absorption coefficient of etching-ground bitumen was found. It could be measured on a microscope slide.
- **The etching needle.** It only has to clear the ground, not cut the metal. The line's width on the plate is the needle's contact width in the ground. The bitten line then widens by undercut (§3.4).

### 2.2 Soft ground: composition and rheology [P] [E]
- **Composition.** Hard ground "diluted with axle grease, tallow, Vaseline or mineral oil" [MFA CAMEO]. One worker's trial: about 3 parts Vaseline to 7 of hard ground was "a bit too hard"; one more part of Vaseline made it right [WetCanvas, P]. Tallow keeps it "slightly soft to touch after it dries".
- **Rheology.** It never sets. It is a tacky, plastic paste whose cohesion is lower than its adhesion to fibers, so paper pressed into it pulls it off the copper.
- **No numbers.** No rheology (yield stress, tack force) or thickness was found for any soft ground. **[E]** By analogy with greases and petrolatum, its yield stress is 10²–10³ Pa at 20 °C, falling steeply with warmth. Petrolatum's yield stress is of that order at room temperature (general rheology data, not checked). Soft-ground users note that a warm room makes the ground pick up everywhere.
- **Thickness [P].** The ground must be thin enough for paper fibers to reach the metal: "If the metal has not been exposed, investigate... not enough pressure exerted during drawing, soft ground rolled on too thickly, top sheet too thick" [Kiekeben manual, P]. **[E] 5–20 µm**, thicker than hard ground: it is rolled from a softer paste.

### 2.3 Soft ground: a line traced through paper [D]
- **Method.** Cassatt laid her drawing on the grounded plate and retraced it with a stylus or pencil, "pulling up the ground where the implement touched the page" [Met, research §5]. The back of one drawing "bears faint lines of soft-ground that adhered to it" [FAMSF; Met]. The ground left the plate and stayed on the paper.
- **Why the line is grainy.** It is the underside of the tracing sheet that presses into the ground. Under the pencil's point (about 0.3–1 mm across, depending on the lead) the hand's force (0.5–3 N) passes through the sheet and spreads by the fibers' through-thickness shear. The paper's underside, a fiber network with exponentially distributed pore depths (notes/research/paper_surface.md; paper_mechanics_and_transport.md §1, §2), is pressed into a layer of thickness t_g.
- **Where the metal is bared.** Where the sheet's local surface reaches within t_g of the plate (its fiber tops pressed through the ground), the ground goes with the paper. The bared share is the paper's bearing (Abbott) curve at the depth the pressure drives it: 1 − e^(−d/µ) with µ the pore depth.
- **What sets the line.**
  - Light pressure bares only the highest fibers: a broken, grainy grey line, like soft pencil on rough paper.
  - Heavy pressure bares most of the contact: a dark, fuller line.
  - The line is as wide as the pressure footprint spread through the sheet: wider for a thicker sheet, and a broad soft lead gives a broad line.
  - The paper's texture (felt marks, laid lines) prints in the line, so the choice of tracing paper is part of the line's character.
- **Reuse.** This is the contact problem the engine already solves for a pencil or a pastel stick on the tooth (crates/paint/src/graphite.rs, pastel.rs, sheet.rs `edge_sink_um`), turned upside down. The plan reuses it.

### 2.4 Soft ground: fabric and objects pressed in [P] [D]
- **The method.** Fabric, leaves or paper are laid on the soft ground, covered with a coated sheet and run through the press "with one blanket removed and pressure slightly lowered" [Kiekeben manual, P]. The ground sticks to what touched it and lifts off with it. Even "lightly touching the ground will produce very detailed finger marks" [Kiekeben].
- **Cassatt.** One c. 1881 print lists "fabric texture" [NGA brochure, research §5].
- **The physics [D].** Under the press (§6) the fabric's height field is pressed into the ground. Wherever the weave's crowns sink deeper than t_g, the metal is bared. The bared share follows the weave's bearing curve at the press's indentation. The engine's linen generator (`canvas{linen=}`) is such a height field.
- **After the press.** The plate dries 2–4 hours before biting, and a soft-ground plate bites 40–60 min on copper in metal salts [Kiekeben, P].

## 3. Acid

### 3.1 Ferric chloride on copper [M] [D]
- **The reaction.** Cu + 2 Fe³⁺ → Cu²⁺ + 2 Fe²⁺, with no gas: "no gases are produced by the ferric when used on copper or brass" [Kiekeben].
- **What limits it.**
  - The activation energy for metallic copper in acidic FeCl₃ is **24.9 kJ/mol**, "inward diffusion... the rate-controlling step" [M, Hydrometallurgy 2010].
  - The rate peaks at **about 2 M** FeCl₃ with 0.84 M HCl [M, McGill thesis], and an optimum at **about 2.25 M**, "independent of temperature and flow rate" [M, Trans. IMF 44, 1966].
  - Commercial 40 % solution (45 Bé, SG 1.45, about 3.5 M) is past the peak. Diluted to about 30 % (34–35 Bé) it "tends to produce the fastest etch rate with copper" [Fortex, maker].
  - The viscosity of 40 % FeCl₃ is 12 cP [M, MG Chemicals 415 datasheet]. That is high, and part of why the stronger solution is slower: diffusion falls as viscosity rises.
- **Measured rates.**
  - MG Chemicals 415, spray or agitated at 50–55 °C: 175–200 µin/min, which is **4.4–5.1 µm/min** [M, datasheet].
  - Die-making patents at about 30 Bé: about 25 µm/min, probably sprayed [patents].
  - Suppliers quote 10–30 µm/min at 40–50 °C [Fortex]. Room temperature with no agitation: 45–90 min for a circuit board's 35 µm of copper [Alliance, P], so about 0.4–0.8 µm/min.
- **The still bath [D].** A diffusion-limited rate is J = D·C/δ for Fe³⁺, and the copper goes at J/2 × its molar volume (7.09 cm³/mol).
  - With D ≈ 1–3 × 10⁻¹⁰ m²/s (Fe³⁺ in a 12 cP bath, about a fifth to a tenth of its dilute value) and C ≈ 2.25–3.5 M, take a natural-convection boundary layer δ ≈ 50–300 µm (an estimate for a still tray; it is thinner with rocking).
  - That gives **0.16–4.5 µm/min, centred on about 0.5–1.5 µm/min**.
  - At 1 µm/min, a 5–15 min bite is 5–15 µm and "prints as a faint delicate greyish mark". A 40 min bite is about 40 µm, "a black and well saturated line" [P, Kiekeben manual]. A further 2 h gives "a wedge-shaped groove" over 100 µm deep, "wider than the original incision" [P]. The derived rate and the workshop times agree.
- **Temperature [D].** With Ea = 25 kJ/mol the rate rises by about **1.4× per 10 °C** (2.6× from 20 to 50 °C). That is gentler than the supplier's "doubles every 10 °C" [P].
- **Copper loading.** Dissolved copper slows the bath and turns it green. MG gives a capacity of 100 g/L of copper [M]. One supplier says to replace the bath once its times have doubled [P].
- **Face up or down.** Sediment slows a face-up bite in plain ferric [P]. Edinburgh etch (four fifths 40 % ferric, one fifth citric acid solution) dissolves the sediment, etches face up, "erodes copper about twice as fast as pure ferric", and works best at 18–30 °C [P, Kiekeben via Polymetaal; Jackson's].

### 3.2 Nitric acid on zinc and copper; Dutch mordant [P]
- **Nitric on zinc** (1 part to 8–10 of water): it gives off bubbles over the bitten area. Under the bubbles the bite is slower, so they are brushed away with a feather. It "underbites": it bites sideways as well as down [Britannica; WGA].
  - Line bites take 10–40 min [P, a printmaker's report].
  - Aquatint steps on 1:8 nitric on zinc are 15–30 s each, darks about 3 min [P, Online Art Center; EtchingInsights].
  - That is about 5–10× faster than ferric on copper.
- **Nitric on copper** is used for big open areas. **Dutch mordant** (potassium chlorate in water with about a tenth of hydrochloric acid) bites copper "much more even and controlled" [Britannica].
- **Cassatt's mordant.** Which mordant Cassatt and Leroy used is not recorded in what was read. In 1880s Paris, nitric acid (eau-forte) and Dutch mordant were the usual mordants for copper. Ferric chloride was used for photogravure from the 1880s, and its use for hand etching then is uncertain. Mathews & Shapiro (1989) may say.
- **No rates.** No measured etch rate (µm/min) of nitric on zinc or of Dutch mordant was found.

### 3.3 Spit bite [P]
Full-strength ferric (45 Bé) brushed onto a finely aquatinted plate bites where it lies: "the longer the ferric stays in one place the darker." Edges are softened by brushing water around them, which dilutes the mordant. Dark tones come "within minutes" [Kiekeben manual].

### 3.4 Undercut [M] [D]
- **The etch factor.** Industry calls depth over lateral undercut on one side the etch factor, K = D/C [M, patent glossary].
  - Acidic cupric chloride on 35 µm copper at 50 µm lines and spaces gives K < 3 [M, patent].
  - A nitric copper etchant gave a vertical-to-lateral ratio of about 2:1 [M, patent].
- **A still bath [D].** A still, diffusion-limited bath with no resist chemistry should bite almost isotropically, K ≈ 1–1.5. A line therefore widens by about twice its depth: a 40 µm line from a 50 µm needle mark ends up about 130 µm wide at the top.
  - Bosse noted that soft copper under nitric undercut "and thereby [lost] the delicacy of the lines" [Tuckamore, quoting Bosse].
- **Narrow lines [D].** A line much narrower than its depth bites more slowly as it deepens: the acid has to diffuse down the slot (aspect-ratio-dependent etching). A thin line bitten as long as a broad one ends shallower.
- **Foul bite** [P]: soft ground resists acid less well than hard ground, and acid gets through pinholes and weak spots as random specks.

## 4. Aquatint

### 4.1 The grain [P] [D]
- **Coverage.** Workers aim for about 50 % rosin coverage. They give 35–65 % [ArtWeb] and 50–60 % [Jackson's]. "Only 20 percent is already on the fragile side. With 40 percent or more the plate should hold up to further etching" [sonic.net].
- **Grain size.** "There are no numerical data published about the average size of the rosin aquatint particles" [Mitjà, Heliogravure II]. Grain size is set by how it is laid:
  - **Dust box [D].** Rosin is stirred into a cloud in a box. The plate goes in after a wait and collects what is still falling. A rosin grain (1.07 g/cm³ [M]) of diameter d settles at Stokes velocity v = ρgd²/(18µ_air).

    | d | v | time to fall 0.6 m |
    |---|---|---|
    | 5 µm | 0.8 mm/s | 12 min |
    | 10 µm | 3.2 mm/s | 3 min |
    | 20 µm | 13 mm/s | 47 s |
    | 40 µm | 52 mm/s | 12 s |
    | 80 µm | 0.2 m/s | 3 s |

    After a wait of t in a box of height H, only grains with v < H/t are still in the air. So a plate put into a 0.6 m box after 5 s takes grains up to about 60 µm, after 15 s up to 35 µm, and after 60 s up to 18 µm. "The longer you wait, the finer the grain" [P] follows from Stokes. How much settles in a given time comes from the concentration of what remains, a decaying cloud.
  - **Shaken through cloth** (a bag of rosin tapped over the plate): coarser grain, at about the mesh of the cloth (100–300 µm openings) [E].
- **Melting [M] [P].** Rosin's ring-and-ball softening point is about 70–80 °C. It flows at 100–130 °C [M, CAMEO; ChemicalBook]. The plate is heated until the grains turn clear [P].
  - Molten rosin wets copper well; it is the classic solder flux. So each grain spreads into a flat dot wider than the grain was.
  - "Fine rosin needs heavier-looking coverage, because the grains are smaller and will spread out less when melted" [P, Jackson's].
  - **[E] a dot spreads to 1.2–1.5× the grain's diameter.** No contact-angle measurement of molten rosin on copper was found.

### 4.2 Bite, tone and over-biting [P] [D]
- **The structure.** The acid bites the connected network of bare metal between the resin dots. The bitten network is "capable of holding ink deposits which result in printed tones that range from the faintest grey to the blackest black" [Kiekeben]. The pits' depth grows with bite time as for lines (§3.1).
- **What sets the tone [D].** The ink left per unit area is the bitten share (1 − c) × the depth the wipe leaves full.
  - The dots hold the cloth off the bottom of each pit. A pit of width w between dots keeps its ink below the yarns' dip, r − √(r² − w²/4) (§5.3).
  - For yarns of radius 75 µm (an estimate for cheesecloth yarn) over a 50 µm pit the dip is about 4 µm. So a pit shallower than a few µm is wiped empty and prints almost white. Light grey needs pits of several µm.
- **What prints.** Transfer (§6.4) takes about half of what is left.
  - An offset-litho black film of about 1 µm reads at optical density 1.6–1.8. That is standard printing practice, not checked against a primary source for this note.
  - A carbon-black etching ink should therefore reach black at a printed film of about 1–2 µm.
  - With 50 % coverage, a 0.7 fill after wiping and a transfer of 0.5, a black needs pits of about **8–15 µm**. At 1 µm/min in ferric (§3.1) that is 10–20 minutes for a medium grain, before undercut.
  - That is shallower than the 40 µm of a black line, because a line keeps its ink across only its width while an aquatint holds ink over half the area.
- **Gravure, the industrial analogue.** Cells run from 2–3 µm deep in the highlights to 40–50 µm in the solids [M, a cylinder maker's guide], and some patents use 3–15 µm.
- **Over-biting [D] [P].**
  - The undercut eats under each dot at about the depth rate (§3.4). Once it reaches the dot's radius the dot falls away.
  - A grain of 20 µm dots goes after about 10 µm of bite. Coarse grains survive longer. This matches the workshop rules [P]:
    - "the smaller the grain of an aquatint, the faster the acid will etch under the rosin"
    - "a very fine-grained aquatint could over-etch after only 10 minutes in Dutch Mordant while a very dense aquatint could be etched 10 hours" [sonic.net]
    - "aquatints can be easily over bitten which typically results in a shallow rather than a dark tone because the dots have been completely eroded" [Kiekeben]
    - biting for many hours is no darker than about an hour [UMBC]
  - Without the dots, the cloth reaches into the area (open bite) and wipes it, so the tone collapses. The prints show it as grey, with darker edges where the cloth could not reach.
- **Even bite [D].** A plate face down in ferric bites alike all over. Its spent acid falls away at Ra ≈ 10¹¹ (Sh = 0.15·Ra^⅓, independent of the plate's size), with a mass-transfer coefficient about 2.4 times a rocked tray's mean laminar one (5°, 4 s, the engine's `natural` and `rocked`). So the rocked flow's thinner layer at the plate's edges (Sh ∝ x^½) changes the bite by under 1 % (Churchill's cubes). Tone variation within an area comes from the grain: each cell's Poisson share of the dust.
  - A coarse grain (10 s wait) bitten 5 min and printed in black varies by OD 0.026 about 0.26 from cell to cell at 0.1 mm.
  - A tint of lead white and Naples yellow prints OD 0.03 at a 10-min bite, too pale for its grain to show.
- **Stopping out.** Times add: a plate bitten 10 min, stopped out in places, then bitten 20 more carries 10 min and 30 min tones [Kiekeben].
- **Wear.** Aquatint wears in the press: "repeated printing is likely to flatten the delicate aquatint", worst on zinc. Print it "with moderate pressure" [Kiekeben].

## 5. Ink and wiping

### 5.1 The ink [M] [P]
- **Composition.** Etching ink is pigment ground stiffly in burnt linseed oil ("plate oil", heavy or light). A printer thins it with plate oil for more plate tone or easier wiping [P: Lybrook; Kiekeben manual]. Cassatt's color inks are not analysed in anything read here.
- **Viscosity.**
  - Black intaglio ink in the Japanese printing bureau's study: **1300 poise = 130 Pa·s** [M, Aoki & Murakami 1962].
  - Banknote intaglio inks: 50–500 poise (5–50 Pa·s) at 20 °C [M, patent US 4240347]. 1–40 Pa·s at 1000 s⁻¹ and 40 °C [M, patent US 8636307]. Machine plates are heated to 50–80 °C.
  - These inks are strongly shear-thinning, with a yield stress.
- **The hand printer's plate.** It is warmed on a hot plate for inking. Linseed oil's viscosity roughly halves for every 15–20 °C of warming (standard oil data), so a warm plate lets the ink go into lines and wipe off the surface more easily.
- **Yield stress.** None was found for etching ink. **[E] 10²–10³ Pa**, as for stiff letterpress and offset inks. It keeps ink in a groove under the cloth's shear and lets it be pushed into fine lines.

### 5.2 Inking [P]
Ink is pushed into the lines with a card, a roller or a dabber (rolled felt), worked in "with a twisting wrist motion, lightly grinding the ink into the tooth of plate" [Lybrook].

### 5.3 Tarlatan [D] [P]
- **The cloth.** Tarlatan is open-weave cotton (cheesecloth) stiffened with starch. "Stiff tarlatan removes less ink from incised lines than regular" [P, Takach]. It is softened before use, by working it against a table or by soaking in warm water [P]. It is bunched into a flat pad and moved in circles.
- **Reach into a wide gap [D].** A pad presses the cloth on at p of a few to tens of kPa (a hand's 20–50 N over 20–50 cm²). The cloth's skin bridges a gap of width w like a beam of bending stiffness B, with sag s = p·w⁴/(384·B).
  - Woven fabrics' bending rigidity (Kawabata KES-F) is about 10⁻⁶–2 × 10⁻⁵ N·m. Starching raises it severalfold, so for tarlatan **B ≈ 10⁻⁵–10⁻⁴ N·m [E: the range for ordinary fabrics is measured; the starched value is extrapolated]**.
  - At 10 kPa:

    | gap w | sag, B = 10⁻⁵ | sag, B = 10⁻⁴ |
    |---|---|---|
    | 0.3 mm | 0.02 µm | 0.002 µm |
    | 1 mm | 2.6 µm | 0.26 µm |
    | 2 mm | 42 µm | 4 µm |
    | 5 mm | 1.6 mm | 0.16 mm |

  - Because sag goes as w⁴, the change from ink kept to ink wiped is sharp, at about **1–3 mm**. That is why an aquatint grain or a crosshatch is needed to hold a tone, and why open bite prints only at its edges. Stiffer (more starched) cloth moves the change to wider gaps, which is why stiff tarlatan takes less from the lines.
- **Reach into a fine line [D].** Inside a groove narrower than the yarn spacing, a single yarn of radius r dips h = r − √(r² − w²/4). For r ≈ 75 µm [E: cheesecloth yarn] this is 4 µm in a 50 µm line, 19 µm in a 100 µm line, and the whole yarn in 150 µm.
  - A fine line therefore keeps nearly all its ink, and a broad deep line loses its top.
  - This is the formula the engine uses for a dry brush's tips over paper pores (notes/research/pastel_brushing.md; crates/paint/src/sheet.rs).
- **What the cloth takes.** Where the cloth touches ink, it takes it by the same rule as the rag (a face that loads and lifts less as it fills, refolded to a clean face). A loaded tarlatan lays a thin film back down: plate tone.

### 5.4 Hand wiping and plate tone [P] [E]
- **The palm.** The printer finishes with the palm, dusted with a little whiting, "to remove the last traces of unwanted ink", until "the bare metal [shines] like a mirror" [P, Grand Palais RMN ateliers].
  - The palm is soft (skin about 0.1–1 MPa). It conforms to the plate surface better than tarlatan and polishes off the thin film.
  - Whiting absorbs oil and is a mild abrasive.
  - Its use wears burr and aquatint (§1.6).
- **Plate tone.** The film left on the flat after wiping. More of it is left with less hand wiping or with more plate oil in the ink [P, Lybrook]. Aquatints "print best with plenty of plate tone" [Kiekeben].
- **Thickness [E].** No measurement of residual plate-tone film was found. If a tarlatan-wiped plate reads as a light grey, the film is a few tenths of a µm (§4.2, at about 1 µm for black). **[E] 0.1–0.5 µm after tarlatan, below 0.05 µm after a full hand wipe.** Weighing a plate before and after wiping would settle it.
- **Retroussage.** A soft cloth passed lightly over the wiped plate draws ink up out of the lines and spreads it beside them. That is done more strongly on a warm plate [P].
- **Hand time.** Cassatt wrote that with Leroy she "only printed eight or ten proofs in the day" of eight hours [research §5]. That is 45–60 min for each three-plate proof inked à la poupée, so about 15–20 minutes to ink and wipe each plate. It is a historical measure of the hand time.

### 5.5 À la poupée [P] [D]
- **The method.** Several colors are dabbed into their areas of one plate with small wads of cloth (poupées, "dolls"), then the plate is wiped [FAMSF via research §5].
- **Cassatt.** Colors that must not touch went on different plates. On one print "some blue from a dress has spread into flowers inked on the same plate" [FAMSF].
- **The physics [D].** Each ink fills its area. The wipe then drags ink across the boundary: a wiping face picks up one color and lays a little of it down further along. That is the rag's smear back over its trailing end (crates/paint/src/rag.rs `SMEAR`).
  - The mixed band is about as wide as the face's contact, millimetres, along the wipe's direction.
  - Ink in a pit is a mixture of both, by volume.
  - In the press the inks do not flow sideways much (§6.4).

### 5.6 What the engine uses (phase P3b, `--@ press`) [D] [M] [E]

crates/paint/src/plate/inked.rs. Each number is derived here, taken from a measurement, or marked [E] with its reason.

- **Where each tool acts [D].** Every inking and wiping tool is moved by the hand along points the painter gives, and acts only where it touches (crates/paint/src/plate/hand.rs). Nothing says where it acts but its path and its own size: no mask, field or function.
  - **An edge** (the card) is held square to its way and passes over the band its length spans; a cell is passed each time the edge crosses its middle. Going over again passes it again.
  - **A pad slid flat** (the tarlatan, the palm, a muslin, a dabber rubbed along) is carried along the path without turning. A point is rubbed for as far as the pad slides while the point is under it; over the pad's width that is its share of a pass: one under the pad's middle on a straight stroke (the rag's `LIFT` is a pass through the pad's middle), the chord's share nearer the rim, none beyond. A pad worked round in small circles rubs the middle all the time. Each per-pass law is compounded over the share: the lift 1 − e^(−rate·share), the palm's split 1 − (1 − q)^share.
  - **A face dabbed** (a dabber, a poupée) touches the disc of its face, pressed flat, at each point.
  - **Soft footprints (under `--@ intaglio-contact`) [D, E].** A dabber's domed felt, a poupée's wad and a bunched tarlatan are smooth compliant bodies. Their contact pressure falls to nothing at its edge; Hertz's (3/2)·p̄·√(1 − ρ²/a²) is the elastic sphere's [D]. A flat pad's uniform disc would need a flat punch's infinite edge pressure.
    - A face's `width` is its contact at the force it is pressed with when none is given. Pressed with F it touches a disc growing as F^⅓ (Hertz).
    - Dabbed, each cell fills its lines by the pressure there over the dab's time.
    - It lays its film where its texture touches through the ink. That share grows with the load (Greenwood–Williamson) until the texture is pressed into the charge, taken at the dabber's working pressure, 28 kPa [E: unmeasured]. So the full 10 µm film covers the middle (ρ < 0.75a), thinning to nothing at the rim: a soft edge.
    - Rubbed along, a face uses the most pressure it meets and Σp·t by the pressure-weighted share.
    - A slid tarlatan pad rubs a point by the pressure along its chord, (3π/8)(1 − u²/r²) of a pass at u from its path instead of the flat pad's √(1 − u²/r²): the same in all, softer at the rim.
    - The palm and the muslin keep the flat pad's share. Skin wets what it touches (below), and the muslin's fibre ends draw by capillarity, not by load.
    - The card's edge and the ground roller's line keep their crisp ends. A stiff card or a cut roller end does print a sharp edge.
  - The tool's size: the card 50 mm [E, as below]; the tarlatan pad 40 mm (a rag's pad bunched over two or three fingers [E], or `width=`); **the palm 80 mm** [M: hand breadth at the metacarpals, 50th percentile 85 mm for British men and 75 mm for women (Pheasant, *Bodyspace*)]; the muslin 40 mm [E]; the dabber's face 30 mm [E]; **the poupée's face 20 mm** [E: a dolly rolled from a strip of felt two inches wide (Belinda Del Pesco's collagraph dauber); a poupée is sized to the colour it carries].

- **The ink's viscosity [D, M, E].**
  - Burnt plate oil at 20 °C: #3 ("medium", the usual heavy grade) taken as a 40-poise lithographic varnish, **4 Pa·s** [M, Printing Ink Manual 1961: a litho ink's varnish of 40 poise]; #00 ("thin", "like cooking oil") **0.1 Pa·s** [E: the makers give no figure; vegetable oils are 0.05–0.1 Pa·s]. Grades between are log-linear.
  - The pigment raises it by Krieger–Dougherty, η = η_oil·(1 − φ/φ_m)^(−2.5·φ_m), φ_m = 0.64 (random close packing) [M, standard suspension rheology]. φ is the tubes' pigment share (their oil from `palette::tube_packing`, the pigments' oil absorption), less the plate oil added.
  - Check: a bone-black ink at φ = 0.45 in #3 oil comes to 28 Pa·s, inside the banknote range of 5–50 Pa·s [M] and below the bureau's 130 Pa·s ink [M] (§5.1).
  - Warmth: η halves every **17.5 °C** (§5.1's 15–20 °C for linseed oil) [M].
  - Shear-thinning is left out: the zero-shear viscosity is used throughout [E: no flow curve for etching ink was found].
- **Filling a line [D].** Ink pushed at pressure p into a slot of width w flows by plane Poiseuille flow, dL/dt = w²p/(12ηL), so in a time t it goes L = w·√(pt/(6η)) deep; against a yield stress τ a plug is held by both walls at L = pw/(2τ). The share filled is L/d, at most **φ_air = 0.9** [E: air trapped at the groove's bottom; plan §5.1, until Unknown 3].
  - The card: 10 N [E] on a bead 50 mm × 2 mm [E], 100 kPa: the pressure is the hand's force over the bead. A point lies under the bead 5 mm [E] over the card's speed (0.25 s at 20 mm/s) each time the edge passes it. The pushes add: the depth pushed goes as √(Σ p·t), so going over twice, across and then diagonally [P], fills a fine line √2 deeper. The dabber: 20 N on a 30 mm face, 28 kPa, 0.2 s a dab [E]; a poupée at the same pressure by default; a face rubbed along, for the length it slides over a point over its speed.
  - With these a line of a few µm fills to φ_air; only a stiff cold ink in a line of 1–2 µm is limited, and warming fills it. The yield stress (**300 Pa** [E], the middle of §5.1's 10²–10³ Pa) limits nothing at the card's pressure.
- **The card's film [D].** The card's edge rides on the ink as Reynolds' inclined slider: h = √(6·K·η·U·ℓ/p), K = 0.0267 at the best inclination [M, Reynolds' slider], U the card's speed (20 mm/s by default [E]), ℓ the 1 mm edge [E], p the bead's pressure. A 28 Pa·s ink leaves about 30 µm on the flat; the dabber leaves 10 µm [E: half, a film's split, of a 20 µm charge].
- **The tarlatan [D, E].** As §5.3: B = 10⁻⁵–10⁻⁴ N·m by `stiff` 0–1, log-linear [E, Unknown 8] (under the contact laws, softened: below, "Beside a burr"); yarn radius 75 µm [E]; the pad presses at 20 kPa × `pressure` (a hand's 20–50 N over 20–50 cm²) [D].
  - **Reach** into an opening of width w at x from its rim: the yarn's chord r − √(r² − w²/4) for w < 2r (its radius r in a wider one) plus the cloth's sag as a beam clamped at both rims, p·x²(w − x)²/(24B), which is §5.3's p·w⁴/(384B) at the middle. A wide opening's cell knows its distance from the rim; a narrow one is averaged across its width.
  - Consequence: an open bite shallower than a yarn's radius (75 µm) is wiped clean to its rims. A deeper one keeps a band at its rim where the yarn and the sag stay above its floor: about 0.5 mm for a 120 µm bite 5 mm wide at B = 5·10⁻⁵ N·m and 10 kPa.
  - **What it takes** follows the rag's lift law (crates/paint/src/rag.rs: `LIFT`, the face's thirst, its creases), the ink's fluidity (`drying::fluid` of its cure), and the face's room (`CAP_UM`). It smears back `SMEAR` of the ink at its surface for each quarter of its width it slides, of which `SOAK` soaks in. That smear is the plate tone. These are the rag's [E] numbers, reused as plan §5.2 decided; the tarlatan's own capacity is unmeasured.
  - With them, on a stiff black (30 µm left by the card), one pass leaves a tone of about 3 µm, three passes 1 µm, five 0.4 µm and eight 0.2 µm. The last two are inside this note's [E] 0.1–0.5 µm (§5.4). The comparison is a check, not a fit. (These were worked out when a pass meant the pad over the whole plate in eight directions; now a pass is the painter's own strokes.)
- **The burr's lee [D].** The cloth is held off the plate for (B·h/p)^¼ (§1.5) **either side** of a burr, as a plate bent over a support lifts on both sides. It is held off in proportion to |sin φ|, φ the angle between the wipe and the line [D/E: a drape sliding along the burr wipes beside it; how fully is unmeasured]. Across a line the lee keeps its ink; along it, it doesn't.
  - Under `--@ intaglio-contact` each cell's piece of burr is a fence segment as long as the cell's share of the line, p/max(|cos|, |sin|), so the segments of a slanting line tile it. Without the switch each piece shielded a strip one cell long, which leaves gaps along a slanting line: a comb at the lee's edge.
    - Each piece sits on its line's centreline across it, found as the centroid of the groove's volume in the line's cells within one cell along it and three across (`Plate::centreline`; the groove is laid at a quarter of a cell, so the centroid finds the line to a few µm). At its cell's middle it followed a staircase of ±p/2 (±21 µm at the easel's cells) along a slanting line, and so did the film beside it: a ±13 % sawtooth with the line's row step.
    - The burr's deposit (`Plate::deposit`, under the laws): each sample of the fin's base carries the same metal, h/2 of its width, and is laid over the cells it covers by area. A cell then reads its piece as a ridge of the fin's own height (2·vol/area = h) wherever its edges cut the base. Weighted by the fin's height at each sample, and dropped into the cell its middle fell in, a cell holding the fin's inner part read up to 1.5 h and a full cell up to a quarter more again: a 2 N line's burr read 44 µm level and 48–55 µm on 1.4° and 5.7° lines, by where it lay in its cells; now 28.8 µm at every angle. The metal is the same, but for what a cell already full can't take (a few per cent where two lines cross).
  - **Beside a burr under `--@ intaglio-contact` [D, M, E]: the cloth on its compressible face.** (This replaces a drape that held the free cloth off the plate for (72·B·h/p)^¼ = 1.5–2 mm and took nothing below it. Printed, a hatch closer than about 3 mm came out solid and every drypoint line had a millimetre-wide blurred band. A review found it unphysical: the cloth's face is compressible over the same order as the burr, starched B fits the unsoftened bolt, and a lee of pure void is wrong.)
    - **The bending rigidity [M, E].** Peirce's cantilever, B = W·c³, with W the cloth's weight per area and c its bending length (Peirce 1930; ASTM D1388). Tarlatan sells at 35–55 g/m² [M: TNTEX's 15- and 17-thread stiff tarlatans 35 and 41 g/m²; Whaleys' 55 g/m²]; the engine takes 45. Printmakers soften it before wiping (§5.3); softened, it is taken to bend like ordinary cotton cloth, c = 2–3 cm by `stiff` [E]. That gives **B = 3.5 × 10⁻⁶ to 1.2 × 10⁻⁵ N·m**, 8.7 × 10⁻⁶ at the default `stiff` 0.7, inside ordinary cotton's 2 × 10⁻⁶ – 1.2 × 10⁻⁵ by KES-FB2. The 5 × 10⁻⁵ of the bolt (c ≈ 5 cm) is not what wipes. (The old path keeps §5.3's 10⁻⁵–10⁻⁴ for the cloth's sag into wide openings; under the laws the softened B serves both.)
    - **The face [M, D, E].** A woven cloth stands far thicker than its fibres would pack: 45 g/m² of cotton is 30 µm solid. Two crossing yarns of 12–26 tex (15 threads/cm at 35 g/m² to 22 threads/inch [M: Talas] at 55 g/m²; 0.13–0.19 mm each at a yarn packing of 0.6) nested by their crimp give **T₀ ≈ 0.25 mm** [D from the yarns, E for the nesting]. Woven cottons lose **20–40 %** of their thickness between KES-FB3's 50 Pa and 5 kPa [M, EMC]; the engine takes 30 %, 75 µm, the same order as a drypoint burr (15–48 µm).
      - A fibre mass compresses by van Wyk's law, **p = K/(T − T_c)³**, T_c an incompressible core. de Jong et al. (1986) and Hu & Newton (1997) found it describes woven fabrics' low-load compression [M]. Through the two KES points, (T₀ − T_c)/(T_m − T_c) = 100^⅓, so T_m − T_c = 20.6 µm.
      - At the pad's 10 kPa the give left is s = T − T_c = 16.3 µm, the face's stiffness k = dp/dT = 3p/s = **1.8 × 10⁹ Pa/m**, and it is squeezed **δ_p = 79 µm** from its free thickness. The review's log-law reading of the same KES data gave 4.5 × 10⁸–10⁹ Pa/m with δ_p = p/k (10–22 µm); van Wyk's law is stiffer at p and its rest squeeze far larger.
      - Lifted u, the face bears **p·(s/(s + u))³**: about half of p at 4 µm, an eighth at 16 µm. It opens a true gap only past u = δ_p.
      - The wad behind the face (Winkler, E 30–100 kPa) is far softer than the cloth's bending over these lengths. It only spreads the hand's pressure evenly.
    - **The cloth over a burr [D].** In plane strain, symmetric about the burr: **B·u'''' = q(u) − p**. Here q is the face's van Wyk pressure, and over the burr's top (|x| ≤ c, c half the fin's base as for the palm) the face lies on a base raised h.
      - It is solved by Newton's method on a grid of a sixtieth of the Winkler length (4B/k)^¼ ≈ 0.37 mm, the burr raised in steps, the far end simply supported ten lengths out.
      - Checks (tests): a burr far below the face's give gives Hetényi's u₀·e^(−βx)(cos βx + sin βx); the burr's extra load balances the face's deficit beside it.
      - A drypoint burr squeezes the face over its top nearly to its core and lifts the cloth by the rest. At 10 kPa, B = 8.7 × 10⁻⁶ N·m: u₀ = 4.4, 17.6 and 37 µm for 15, 29 and 48 µm burrs (c = 10–25 µm). The face beside the burr bears 49 %, 11 % and 3 % of p, rising back to 76 %, 31 % and 10 % at 0.5 mm. It bears all of p again by 1 mm (by 1.5 mm past the 48 µm burr).
      - **No true gap opens** past a drypoint burr at a hand's 2–20 kPa: the lift stays below δ_p = 68–83 µm. The review's 7–19 µm gaps assumed the beam lifted by the whole of h and δ_p = p/k. Only a burr taller than δ_p plus the face's give, some 95 µm, lifts the cloth clear. (A 4 N line's crossing read 85 µm until the burr's deposit was mended, below; it is 48 µm, the line's own.)
      - **The weave's knuckles [D].** Tarlatan bears on the plate at its yarn crossings, 0.67 mm apart (15 threads/cm), with about half its area open mesh, while the Winkler length is 0.37 mm. A continuum is still the right description of the lift: the lift's wave is 2πλ ≈ 2.3 mm long (longer where the face unloads), three or four pitches. Checked by lumping the face's stiffness into rows of knuckles a pitch apart, the row through each point (the worst case, a line along the weave): the lift under a knuckle row is within 15 % of the continuum's at every distance (0.68 against 0.80 of u₀ at 0.2 mm, 0.34 against 0.31 at 0.5 mm, both gone by 0.8 mm). Across the weave at other angles the rows' distances from the line are finer still. The open mesh lowers the cloth's mean stiffness and weight, which the KES and Peirce figures already average over.
      - The beam rings below its place past ¾ of a bending length, by at most e^(−π) of the lift. The wipe reads that ring as full contact [E].
      - Pieces of burr of like height (0.25 µm) and top (0.5 µm) share a solution. Overlapping burrs: the larger lift [E].
    - **What it takes there [D, E].** The film is a liquid 6–13 µm deep (the card's 13 µm, then what the first pass leaves), about a fibre's thickness (12–20 µm). A fibre takes ink by dipping into it and drawing it up (cotton wets linseed oil), not by pressing on the metal through it.
      - So the take goes with the number of the face's fibres in the film. The face's compressible layer keeps its fibres as it swells or is squeezed (van Wyk's homogeneous mass), so their density, and the number in any slab of it at the plate, goes as 1/(T − T_c): lifted u, **s/(s + u) = r^⅓** of the open plate's, r the share of p it bears (`Face::fibres`). It takes at the rate of a touch r^⅓ times as long: the lift law's 1 − e^(−rate·share·r^⅓).
      - Checked against the alternatives. Greenwood–Williamson's real contact ∝ load (the law used before, r) counts solid-on-solid contacts of asperities with an exponential height distribution: right for a dry face or a film thinner than the fibres' roughness, not for fibres standing in a film as deep as they are thick. The take is linear in the fibre count while the fibres in the film are sparse: the face is 30 µm of cotton in some 170 µm pressed, a fibre share of 0.17, so they are. A faster capillary draw through the swollen face's wider pores (Darcy's flow ∝ pore radius) would make the exponent smaller than ⅓, not larger: r^⅓ is the steepest fall the physics allows, as the review found.
      - Beside a 2 N burr r = 0.11 at the fin, 0.31 at 0.5 mm: the take is 0.48 and 0.68 of the open plate's, against 0.11 and 0.31 before. Over two passes that is nearly all of the film, so the lifted band no longer keeps a plateau.
      - **Over the burr's top** the face is squeezed by h − u₀ more than on the flat, its fibres there s/(s + u₀ − h) times denser, 2.6 times over a 2 N fin, so it wipes the fin's top faster than the open plate. It took there at the share it bore beside the fin, and the top kept 1.1 µm after a first pass against the open plate's 0.3. Squeezed, the fibres are no denser than they pack in a yarn (0.6 against the face's 0.17: 3.5 times) [D].
      - **The yarn's shade [D, E].** A yarn is a bundle packed to some 0.6 of its section, all but as full as fibres pack, so below its own radius it keeps its round shape: the face's give is its crimp and hairs, which flex over the weave's pitch. That is how the engine already reaches into a line, by a yarn's chord (§5.3). The same yarn resting on the fin's top falls away beside it as its circle: it stands **h − (R − √(R² − (x − c)²))** off the plate at x from the burr's middle (R the yarn's radius, 75 µm [E, Unknown 8]), h over the fin's top and the line under it, none past c + √(2Rh) ≈ 90 µm for a 2 N fin (57 µm for 0.5 N, 110 µm for 4 N). In that shade the face meets only the ink standing above the yarn, at its fibre share there, and the loose fibre ends draw on the rest (`yarn_shade_um`). The line under the fin is shaded too: the yarn is held off it by the fin, so it reaches h less far into it. This is the ink against the burr, a drypoint line's velvet. The yarn is taken lying along the fin, its round section across it, which shades least [E: a yarn crossing the fin is straight along its length and is held off further, by its bending].
      - Where a true gap g opens (past the face's squeeze, or in the yarn's shade) it reaches only the ink above g, and g less far into the openings, at its fibre share at the gap's edge, so the take is continuous into the gap.
      - **The loose fibre ends [D, M, E].** Where the face stands on the plate they are fibres of its face, counted in r^⅓. Where it stands clear (a true gap, a yarn's shade) they hang in the gap and draw on the film below it.
        - Their tips touch **0.0225** of the plate: Uster hairiness H ≈ 5 cm of protruding fibre per cm of yarn [M: carded ring-spun cotton 4–7], over 3000 m of yarn per m² and 15 µm fibres, is a cover of 0.22 if they lay flat; a hanging end touches with its tip, taken as a tenth [E].
        - Under its tips each lifts up to retroussage's capillary depth of a touch (Lucas–Washburn, about 17 µm a 0.05 s touch at 20 °C), by the share of a pass.
        - They draw from the film only. The cloth's fibres hang into lines wherever it goes, and what it takes from them is its reach.
        - A first version drew the full capillary depth over the whole gap. It wiped the film beside the tallest burr to bare metal, while the face just outside took almost nothing: white dashes along the heaviest line of the proof.
      - **What the lifted face lays back** goes by the same law: it smears back by its fibres in the film, r^⅓ (none in a gap), on the flat and in the openings each by its touch there over its share of the cell. (Split by area, a 2 N line's own cells, where the fibres over the fin are dense, laid most of their share into the groove the yarn can't reach: the line's ink rose from 20.4 to 31.1 µm under the first pass.)
      - **On the plate (the review's scene).** A 2 N line, carded, wiped round twice with the tarlatan refolded between, palmed once along it. Before, the open plate's film 60–75 times over within 0.3 mm, a plateau. Now the cells either side of the line keep 4.4 µm after the first pass, 2.7 µm after the second (36 times the open plate's 0.076) and 1.1 µm after the palm: the velvet. From 0.08 mm on the film is within a fifth of the open plate's after the first pass and the open plate's after the second. The line keeps its ink, 20.4 µm through all three (it gave 2.5 µm before).
      - **Printed (§5.7).** Before, a core at OD 1.58 and a plateau of OD 0.73–0.84 out to ±0.2 mm, OD > 0.1 over 1.12 mm. Now, at the easel's 41 µm pixels, OD 1.64 at the core, 0.17–0.29 one pixel off, nothing from 0.08 mm on: half-peak 0.08 mm, OD > 0.1 over 0.17 mm. At 25 µm pixels: OD 2.1 over ±25 µm, 0.93 at 50 µm (the velvet), 0.04 at 75 µm, nothing past 0.1 mm. The engine's line is narrower than real ones (§5.7) and is not fitted to them.
    - **What it lays, and the wipe's way [D, P].** "Ink tends to pile up in the lee of the burr … if they wipe perpendicularly to the line, they can actually increase the pile of ink on the other side of the line" [P, Wikipedia "Drypoint", after Ross, *The Complete Printmaker*].
      - Each point of the cloth's face slides along its track carrying what it has picked up (`tracks`). Under the laws it lays its smear-back where it carries ink, by its share of contact there.
      - So ink taken off a line is laid on past it, the way the pad goes. A line inked by a narrow card on a clean plate and wiped across once leaves about twice the film past the band on the side the pad goes to as on the side it came from.
      - The direction-blind drape and its test ("the same whichever way it wipes") are gone.
      - Not modelled: the face's compression hysteresis (KES RC 30–60 %). A face dented over the burr would recover only partly on the far side, which would deepen the lee downstream [E].
- **The palm [D, E].** Skin 0.3 MPa [E: §5.4's 0.1–1 MPa, geometric middle], ν = 0.5 [M], pressed at 10 kPa × `pressure` [E].
  - It bulges into a slit as a pressurised crack in an elastic half-space, 2p(1 − ν²)/E·√(x(w − x)) (Sneddon): **about 1 µm into a 100 µm line** at 5 kPa, the whole of an open bite.
  - It is held off past a step h for h·E/(p(1 − ν²)): 2 mm past a 25 µm burr. That is the length over which uniform pressure deflects a half-space by h. It is not the gap around a ridge that carries the load. Over a slanting line it drew sawtooth halos several millimetres wide (a painter's report, 2026-11-17).
  - **Under `--@ intaglio-contact` [D, M, E]: the palm on its ridges.** A smooth skin would be held off a burr over a wide gap: as an elastic half-space (E* = E/(1 − ν²)) on a rigid flat under p, the burr's line load P opens a gap that closes smoothly where the stress intensity vanishes (P = πpa) and stands g(x) = (2pa/E*)·(acosh(a/x) − √(1 − x²/a²)) at x from the burr [D: half the crack-opening displacements of a pressurised crack and of point forces at its middle, Tada, Paris & Irwin], g(c) = h setting a: a ≈ 0.57 mm past a 40 µm burr at 5 kPa. That was the model, and it left ±0.2–0.5 mm untouched beside each line. Real skin bridges further still: the palm's stratum corneum is thick and far stiffer than the tissue under it, a plate on a soft bed. But printers clean beside the lines by hand: "if the printer wipes in the direction of the lines with their hand, they may remove most of the ink, leaving a light gray line" [P, Wikipedia "Drypoint", after Ross, *The Complete Printmaker* (1990) 82–88].
    - **The ridges [M, E].** The palm is friction ridge skin. Fingertip papillary ridges are **59 ± 19 µm high and 485 ± 71 µm apart** [M: laser profilometry of silicone prints, Stücker et al. 2001]; the palm's are taken alike [E]. A sinusoid of that height and pitch has a crest 0.20 mm round, against the 0.3 mm measured on fingertip ridges (Hamilton et al. 2020): the same order, a check.
    - **Westergaard's contact [D].** The ridges, z = Δ(1 − cos 2πx/λ) with Δ half their height, pressed on the flat plate at p: they flatten all over only at p* = π·E*·Δ/λ = 76 kPa, far above a hand's. Below it each ridge touches over sin²(πa/λ) = p/p*, under (2p/sin²(πa/λ))·cos(πx/λ)·√(sin²(πa/λ) − sin²(πx/λ)), and the furrow stands open between: the wave less what that pressure presses the skin in, by the periodic line load's −(2/πE*)·ln|2 sin(πx/λ)| (`Ridges`). At 2.5, 5 and 10 kPa: a = 28, 40, 57 µm, and the furrow's middle stands 53, 49, 42 µm off the plate.
    - **A burr lower than the furrows** (every drypoint line but the heaviest at a firm press) props only the ridge riding it. A ridge whose middle lies within reach(h) of the burr (where its free gap is h: 104, 144 and 220 µm from its middle for 15, 29 and 48 µm burrs at 5 kPa) rests on it and stands off over its own contact. So a point d from the burr is held off for the share of the palm's slide in which the ridge touching it is so propped: the overlap of |ξ| < reach(h) with |ξ − d| < a, over λ (`Ridges::shield`). That is at most 2a/λ = 0.17 at 5 kPa, by the line, and nothing past a + reach(h), 0.18 mm for a 2 N burr. In the held-off share it reaches the ink above the burr's height. The ridges are taken parallel to the line, where they shield most [E: they run every way over a palm; a ridge crossing the line rests on it over a shorter stretch].
    - **A burr taller than the furrows** props the whole skin by what it stands above them, with the half-space gap above (h − clearance for h).
    - On the burr's top the ridge riding it touches the film whole.
    - So the palm wipes beside a line almost as on the open plate: after n passes of split q it leaves at most (1 − q)^(−n·2a/λ) of the open plate's film there, and nothing more from 0.2 mm on. Ross's "most of the ink" is the lee's pile.
    - The shield is the same whichever way the palm moves, a static contact. Overlapping burrs: the larger shield [E].
    - A cell with some of its points propped once kept the rest off too: the old gap read every point's share of contact as none, so the un-gapped part of a cell holding a gap slot was skipped, a one-cell outline 0.5 mm off each burred line (0.80 of the film left against 0.39 and 0.36 either side). Now an un-propped point touches whole.
    - **Carrying within a stroke [D], not modelled.** Skin wets the ink and slides over it: the film under a ridge is sheared between skin and plate, moving at half the palm's speed while under it. Each ridge's passage moves the film it touches about its contact's half-width a (40 µm at 5 kPa) the way the palm goes, and a pass slides some hundred ridges over a point, so the transport is set by how much of the film the ridges' flanks and furrows keep (and whiting soaks up) at each passage, and by whether oily skin slips. Neither is measured. With no slip and nothing kept, a pass would drag the film millimetres; with each ridge keeping its split share, a pass would take everything. The model keeps the per-pass split (what the palm takes leaves the plate) until the share a ridge keeps is measured. Ross's "pile on the other side" when wiping across a line is that transport [P].
  - It parts from the film it touches taking half (a symmetric film split, §6.4), or 0.9 with whiting [E: whiting drinks the oil], once each time the palm passes a point (by its share of a pass). Two passes with whiting take a tarlatan's tone to nearly nothing (well under §5.4's 0.05 µm: the palm is wiped clean between strokes, so it lays nothing back); without whiting each pass leaves about a sixteenth.
  - Whiting's abrasion of burr is left to phase P7 (wear).
- **The polish (under `--@ intaglio-contact`) [D, E].** No wipe takes the film below the ink in the plate's finest scratches.
  - "Mirror polished" plates yield little or no tone and mill-finish ones a significant tone [P, Tuckamore Press].
  - A cotton fibre (12–20 µm) or the palm's skin at kPa bridges a scratch a µm or two across: skin bulges p(1 − ν²)w/E ≈ 0.03 µm into one 1 µm wide at 10 kPa.
  - The plate's roughness is taken as **Ra 0.1 µm** [E: between a mirror polish's ~0.05 µm and a cold-rolled mill finish's 0.2–1 µm, general finishing data; no measurement of a printmaker's plate was found].
  - A cloth's fibres resting on its summits leave at least the void below its mean plane: σ/√(2π) for Gaussian heights with σ = Ra·√(π/2), so **Ra/2 = 0.05 µm** [D; the mean plane is the low bound]. That is the edge of §5.4's [E] "below 0.05 µm after a full hand wipe".
  - The palm does not rest on the summits: at 10 kPa it conforms to undulations whose wavelength passes π·E*·A/p, about 10 µm for this roughness (Johnson). It leaves only the void of the shorter wavelengths, whose share of the roughness is unmeasured. Ra/2 is kept for the palm as its order of magnitude [E].
  - Printed (split 0.6) it is about 0.03 µm of ink: a bone-black film reads OD ≈ 0.01. A trace, not a tone.
  - Without it the palm with whiting took the film to nothing, OD 0.000.
  - The tarlatan itself, refolded to a clean face, leaves about a tenth of its film each pair of passes: 1.05, 0.11 and 0.011 µm after 2, 4 and 6 passes on a card's 15 µm (worked out when a pass meant the pad over the whole plate in eight directions). That comes from the rag's [E] smear-back numbers. Visible plate tone needs a lighter wipe.
- **Retroussage [D, E].** A soft muslin's fibre ends, cotton 15 µm across [M: 12–20 µm], hang into any wider opening. They draw ink up by Lucas–Washburn, h = √(γ·r·t/(2η)): γ = 0.033 N/m for linseed oil [M], pores 10 µm [E], 0.05 s a touch [E], a touch each time the cloth passes a point (by its share of a pass). That is about 17 µm of a 28 Pa·s ink at 20 °C and 28 µm warmed to 45 °C. The ink is laid on the plate within 0.5 mm [E], less the rag's `SOAK` share, which the cloth keeps.
- **Drying [D].** The ink dries as an oil film of its thickness: `drying::rate` with its pigments' rates (`Palette::drying_of`), fattened by added oil, at engine 3's pace (`drying::Pace`). It is wiped or printed by its fluidity, `drying::fluid` of its cure: a stiff black left on a plate a week has set past its gel point. Warmth's effect on the oxidation rate is not modelled [E].
- **The warm plate [D, M, E].** Off the hot plate a plate cools in air with τ = ρ·c·t/h: copper 8960 kg/m³ × 385 J/(kg·K), zinc 7140 × 388 [M]. h = 10 W/(m²·K) over its area, both faces and the bench [E: natural convection from a warm horizontal plate is 2–5 W/(m²·K) a face; polished copper hardly radiates]. A 1.3 mm copper plate has τ = 7.5 min. Warming takes 2 min of hand time [D: ρct over a 300 W/(m²·K) contact is 15 s, four of those come within 2 %; the contact conductance is E].
- **Hand time [P].** Cassatt's 15–20 min to ink and wipe each plate of a three-plate proof (§5.4) is spread over a plate of 35 × 21 cm and the usual sequence: a card's inking, three tarlatan passes and two palm passes. A quarter of the time goes to the card, a half to the tarlatan and a quarter to the palm [E: the split]. That gives 277 mm²/s for the card, 416 for a tarlatan pass and 555 for a palm pass.
  - A tool moved along points takes the area it passes over, each pass counted (a slid pad's by its share of a pass), at the pace of one pass: the card's edge 555 mm²/s (Cassatt's card went over twice), a dab 1390 (her dabber's pace was ten dabs a point, 139), a tarlatan pass 416, a touch of the palm 2220 (her palm pass touched each point four times [E: overlapping strokes]), the muslin 555 [E: as fast as the palm's pass].

### 5.7 Real drypoint lines, measured [M]
The outside reference for the printed line (the engine's physics is derived above, not fitted to these). The review of 2026-11-17 measured The Met's open-access scans of two pure drypoints: rows of pixels across single strokes, sRGB linearised, optical density OD = log₁₀(paper/row), the paper the median of the sheet between strokes, µm per pixel from the plate's size and the image's.

| print | half-peak width | OD > 0.1 | OD > 0.3 |
|---|---|---|---|
| Rembrandt, *Clump of Trees with a Vista*, 1652, drypoint, second of two states (The Met 20.46.4, object 354635; image DP814786, 56.7 µm/px), four strokes | 0.17–0.34 mm | 0.28–1.30 mm | 0.17–0.57 mm |
| Whistler, *Weary*, 1863, drypoint on Japan tissue, fourth state of six (The Met 83.1.28, object 365015; image DP813372) | 0.21 mm | 0.32 mm | |

- The profiles are peaked, with no plateau; hatching 0.1–0.3 mm apart stays open.
- They include what the engine leaves out: the ink caught on the burr's torn faces and under its lean (§1.4–1.5), ink spreading over the damp sheet's fibres, the scan's own blur at 57 µm a pixel, and strokes whose force is unknown (Rembrandt's heavy strokes are the widest).
- **The engine against them.** Its 2 N line (§5.6, "Printed"): a half-peak width of 0.08 mm at the easel's 41 µm pixels, OD > 0.1 over 0.17 mm, OD > 0.3 over 0.08 mm, nothing at 0.3 mm off. Peaked like the real ones, with no plateau, but some half as wide: the yarn's shade is the only velvet it models. Before this change it printed a half-peak of 0.29 mm only because of a plateau (OD 0.7–0.8 out to ±0.2 mm, OD > 0.1 over 1.12 mm), which no real print shows.
- The test (`a_printed_drypoint_line_is_peaked_like_real_ones`) bounds the engine's line from the outside only: half-peak ≤ 0.35 mm and OD ≤ 0.15 at 0.3 mm off.

## 6. The press

### 6.1 Pressure [M] [D] [P]
- **A hand press.** One maker's bench etching press (Rollaco Kudos): top roller Ø 106 mm, 5:1 gearing, **load rating 6 t** over a 45 cm bed. That is **about 130 N/mm** of roller [M, maker]. A forum post gives 1400 lb/in, about 245 N/mm, for intaglio [P, unsourced].
- **Banknote presses.** 100 to over 1000 N/mm [M, Koenig & Bauer patents; Mechanika 2014]. Nip pressures are 30–40 MPa for blind embossing of polymer notes [M, US 6176522], up to about 150 MPa [M, De La Rue patent], with 256 MPa used in a plate stress model.
- **Under the felts [D].**
  - A felt pack 6–11 mm thick: a sizing catcher 1.6 mm, a cushion 6.35 mm and a pusher 3.2 mm [M, Takach blanket specifications].
  - Model the pack as a Winkler layer of stiffness k = E_f/t under a roller of radius R. The line load q = (2/3)·k·a³/R sets the half-width a, and the peak pressure is k·a²/(2R).
  - Wool felt compressed to its working strain has a tangent modulus of 1–20 MPa [E: felt is strongly nonlinear; no figure for press blankets was found].
  - With R = 53–100 mm and q = 130 N/mm, the contact is 1–10 cm long and the pressure **0.5 to about 15 MPa**.
  - The plate stands 1–1.6 mm above the bed, so the felts over it are squeezed harder than beside it. The pressure on the plate is higher than the mean.
- **Measuring it.** This range is wide. Pressure-indicating film (Fuji Prescale) under the felts would settle it; see Unknowns.
- **What workers do.** They set the pressure by proofing, not by gauge: "just do some proof prints and add pressure as needed" [P, WetCanvas].

### 6.2 Damp paper [P] [M] [D]
- **Why it is damp.** It is soaked to soften the fibers and the sizing, so it can be pressed into the lines without splitting and picks up the ink. "Etchings cannot be printed on dry paper" [P, printing manuals]. Soaks run from 30 s to over 5 min, or overnight in a damp pack, then the sheet is blotted until no shiny wet patches remain. Unsized papers and washi are only misted [P].
- **Moisture content [D].** One printer dampens 35 sheets of 10 × 15 in BFK (250 g/m², 24 g each) with under a pint (0.47 L). That is about 13.5 g of water per sheet, **MC ≈ 0.56 g/g** dry basis [P → D]. Soaked and blotted sheets are probably wetter, **0.5–1 g/g [E]**.
- **Softening [E].** Wet paper's stiffness and strength drop by roughly an order of magnitude as the fiber bonds take water (paper physics; wet tensile strength of unsized paper is 5–10 % of dry). The z-compression law of dry paper (ε = φ*(1 − e^(−p/E*)), E* ≈ 4–5 MPa [M, notes/research/paper_mechanics_and_transport.md §1]) softens with it. **[E]** for damp paper: E* ≈ 0.5–1 MPa, so at the press's few MPa the sheet is compressed nearly to its compressible porosity and pressed into the lines.
- **Drying.** Damp paper dries by evaporation from the faces exposed. At the rate used for water on pastel in PR #322 (about 2.5 µm/min of water, an estimate from indoor-air formulas there), a 250 g/m² sheet at 0.56 g/g holds 140 g/m² of water. It loses about 5 % of that per minute per exposed face. Between plates it is kept under plastic or blotters [P].

### 6.3 Embossing and the plate mark [M] [D] [E]
- **The embossed line.** Banknote intaglio leaves a printed line 20–60 µm high, ink and embossed paper together, "tactile" [M, patents]. Engraving depths there are 30–220 µm.
- **The plate mark.** The plate's bevelled edge (filed at about 45° [P]) is pressed into the damp sheet by the felts. The sheet sets in that shape as it dries, since the fiber bonds re-form in the deformed shape.
  - **[E]** the depth of the plate mark is a fair share of the plate's thickness, 0.3–1 mm, falling with paper stiffness and felt hardness.
  - The plate's surface prints a faint tone of plate tone inside the mark. Outside it the paper is clean.

### 6.4 Ink transfer from a groove [M] [D]
- **Aoki & Murakami (1962)**, Government Printing Bureau, Japan [M]:
  - Engraved lines of triangular section (120° at the bottom), 150–454 µm wide, 1300-poise ink, India and Mitsumata papers. Pressure was measured with strain gauges, line and print profiles with a light-section microscope.
  - "Press-in" (how far the paper goes into the line) grows with thinner and softer paper and with pressure. Thickness matters more.
  - Transfer rises with the fill of the line at a fixed press-in, to **a maximum of 45 %**. At a fixed fill it rises with press-in, towards a maximum at 100 % press-in ("an imaginary value").
  - With low press-in, transfer starts low and climbs fast to the maximum as fill rises. With high press-in it starts higher and climbs slowly.
  - Their conclusion: "skilful wiping that leaves plenty of ink in the lines" gives good prints, even with light pressure and hard paper.
- **Pritchard & Finkle (1964)**, gravure cells [M, cited in Chem. Eng. Sci. 2005]: maximum **48 % to non-absorbent** materials, **60 % or more to highly absorbent** ones. Transfer is higher at low speed, low viscosity and in deeper cells.
- **Gravure, by geometry.** Shallow, wide cells empty more completely than deep, narrow ones [M, Rothstein group JNNFM 2012; parametric studies]. An industrial check: banknote recesses up to 100 µm deep leave 10–40 µm of ink, and comparing ranges gives about 50 % [M, patents US 9376579, US 4240347].
- **A model that fits these [D].**
  - The ink in a groove of depth d is filled to φ·d. The paper is pushed in to a depth π·d. They touch only if π + φ ≥ 1.
  - Where they touch, the ink between the paper and the groove's bottom splits by cohesive failure. The split share s is 0.45–0.5 for a non-absorbent sheet (Aoki; Pritchard), and **[E] 0.55–0.7** for damp absorbent paper. Pritchard's absorbent value is about 0.6. Damp paper's pores hold water, which oil-based ink does not enter, so it sits between the two.
  - Where they don't touch, nothing transfers. Across a line's width the press-in varies (the paper bridges a narrow line and fills a wide one, §6.2), so the transfer is the split share times the share of the width in contact.
  - This reproduces Aoki's three findings: the rise with fill, the rise with press-in, and the same maximum.
  - **Press-in against width [D].** The sheet bends into a groove of width w under pressure p as a layer resisting by its through-thickness shear and compression (the anisotropic-layer model in paper_mechanics_and_transport.md §1, with damp moduli). Wide grooves are filled; grooves much narrower than the sheet's thickness are bridged.
- **Burr.** The ink on the burr's faces and in its lee is on or near the surface, so it is in contact at any press-in: it transfers at about the split share. That is why burr prints rich even at light pressure.
- **Mirror and spread.** The print is the plate mirrored. Under pressure the ink also squeezes out sideways at the line's edges by a fraction of its film thickness (a soft line edge); in gravure "spreading" this is a few µm. **[E]** a print's line is 5–15 µm wider than the ink in the plate.
- **What stays on the plate.** The ink not transferred stays in the lines and is wiped into the next inking.

### 6.5 Monotype [P] [D]
- **The process.** Ink is painted or wiped on a smooth plate and printed once. A second, paler pull (a "ghost" or cognate) takes what is left [P, Clark Art; MoMA notes]. Degas took up to four pulls from one plate, reworking the ghosts in pastel [P]. Cassatt's one known use is in a c. 1894 print with drypoint and aquatint (research §5).
- **The physics [D].** A flat film between plate and paper splits by the same share s (§6.4): **about 0.5 for a smooth plate to damp paper, up to 0.6–0.7** [D from Pritchard & Finkle].
  - The first pull takes s of the film, and the ghost takes s of the remaining (1 − s). The ghost therefore carries (1 − s) times the first pull's ink: **about 0.4–0.5**. A third pull carries (1 − s)², 0.15–0.25 of the first.
  - "Light-field" monotype paints the ink on; "dark-field" inks the whole plate and wipes the image out [Clark Art]. Both are inking and wiping (§5) without lines.
  - Degas soaked the paper to take up more ink [P].

### 6.6 What the engine uses (phase P3b, `press{}`) [D] [M] [E]

crates/paint/src/plate/press.rs.

- **The felts [D, M, E].** A Winkler layer of stiffness k = E_f/T under a rigid roller of radius R (§6.1). Squeezed by d, a length of roller carries (4√2/3)·k·√R·d^(3/2) and presses k·d at the middle of a contact 2√(2Rd) long. The values:
  - the felts' thicknesses 1.6 + 6.35 + 3.2 mm [M];
  - the roller Ø 106 mm and the rating 130 N/mm over a 45 cm bed [M];
  - E_f = 4.5 MPa [E: the geometric middle of §6.1's 1–20 MPa; Unknown 2].
  - `pressure` is the share of the rated load the screws set.
  - Over the plate the felts are squeezed by its thickness more than over the bed beside it. The load is shared between the two, so a 1.3 mm plate 35 cm wide at 0.7 takes 2.7 MPa against 2.2 MPa beside it, over a contact 54 mm long. These sit inside §6.1's 0.5–15 MPa.
  - Squeezed past 80 % of their thickness the felts are solid and the press refuses [E: wool felt is about 80 % air].
- **Press-in [D].** Over an opening of width w the sheet has nothing under it. The felt's pressure p pushes it in, and it is held by its through-thickness shear and its bending, as a Timoshenko strip clamped at the rims: v(x) = p·x(w − x)/(2·G·T) + p·x²(w − x)²/(24·D), with D = E·T³/12.
  - G is sheet.rs's 30 MPa (paper_mechanics_and_transport.md §1). E is 4 GPa in plane [M: printing papers 2–8 GPa].
  - Both fall with the sheet's water by damp.rs's `stiffness` (0.22 damp), so a damp sheet goes in about 4.5 times as far.
  - A sheet thicker than the opening is wide bridges it, as Aoki found.
- **The fibre tops [D].** Over an opening the sheet's surface fibres are not pressed flat. The share of them reaching a depth z below its face is the bearing curve of its pores, e^(−z/µ) (paper.rs: µ, the pore depth, about 4–9 µm).
  - So the share of an opening's ink the sheet touches is the mean across its width of e^(−max(0, g − v)/µ), where g is how far below the plate's face the ink stands.
  - This is what lets a fine line print when the sheet barely presses into it (Aoki: transfer starts low at low press-in and climbs fast with fill).
- **The split [M, E].** 0.45 to a non-absorbent sheet [M, Aoki & Murakami], 0.6 to an absorbent one [M, Pritchard & Finkle]. A dry sheet lies between them by `Paper::absorbent` [D: interpolation], a damp sheet 0.6 [E: §6.4's 0.55–0.7].
  - Transfer from an opening is the split × its ink × the share touched × the ink's fluidity (`drying::fluid`: set ink stays). The film on the flat splits whole.
  - It rises with fill and with press-in, toward the split, never past it. A damp sheet takes more than a dry one, by its press-in and its split.
- **The print [D].** Mirrored. The ink is laid at the sheet's pixel under each plate cell, by area (the two pitches can differ by rounding), as wet oil paint of its tubes (`Palette::pile(...).paint(oil)`).
  - It mixes by volume with any wet paint there, its cure carried over (`Canvas::lay_ink`).
  - Nothing is drawn into the sheet's pores: a damp sheet's pores hold water, which oil does not enter (§6.4). Its absorption as the sheet dries is not modelled.
  - The 5–15 µm sideways squeeze at a line's edge [E, §6.4] is below the engine's pixel and is left out.
- **The sheet's size (under `--@ intaglio-contact`) [D, M].** A sheet's pixels are its fibres as made. Damp, it is larger by the strain its fibre walls' water gives it (damp.rs `strain`: β·(min(M, M_fsp) − M_iso), notes/research/damp_paper.md).
  - A mm of the sheet at the pull is 1/(1 + ε) of a mm as made, so the plate is printed over fewer fibres and the dried print is smaller than its plate by that share.
  - The cream laid 140 g/m² sheet soaked 20 min and blotted (0.29 g/g) is 1.2 % larger. A 250 mm plate prints 247 mm.
  - `at` is measured on the sheet as it is, from its top left edge, where it is held (pins or a trapped end, §7.1).
  - Two plates pulled at one moisture meet exactly. A sheet left open 30 min between them (0.29 → 0.15 g/g, 0.8 % less strain) puts the second 0.5 mm off at 60 mm from the held corner: Δε·L (§7.2).
  - Not modelled: pin slop and the stretch of each pass (§7.2's [E] figures), both for phase P6.
- **Embossing and the plate mark [D, E].** A damp sheet (free water in its pores, its bonds open: damp.rs `plastic`) keeps the shape it was pressed into as it dries (§6.3):
  - the lines stand up by their mean press-in;
  - a burr is pressed in by its volume;
  - the footprint sinks by half the plate's thickness [E: §6.3's "fair share", 0.3–1 mm], sloping at the bevel over thickness/tan(bevel).
  - A dry sheet springs back: none [E].
- **Hand time [E].** The bed's travel (the plate's length and the contact) at `speed`, and 45 s to lay the sheet, tissue and felts and lift them: plan §6.2's "about a minute a pass".

## 7. Registration

### 7.1 Methods [P]
- **Trapped paper.** One end of the sheet stays caught under the roller while the plate is changed; the next plate is placed in the same spot, and the sheet is laid down again [Britannica].
- **Mat and needles.** A mat (card or metal) cut to the paper with the plate's place marked. Pinholes are punched through the sheets, and the sheet is held by "two heavy needles through the punched holes", put into the mat's holes and released. The holes are near an edge that is trimmed later [Britannica].
- **Pins on the bed.** Pins outside the plate, well away from it (one worker keeps 150 mm), with the holes reinforced with tape [nontoxic-print.com].
- **Cassatt and Leroy.** One demonstration describes Cassatt's plates "aligned precisely with pins" [research §5, PMA video summary]; the details are uncertain. "All three plates in perfect registration onto one sheet" [FAMSF].

### 7.2 Error [D] [M]
- **Pin slop.** A needle in a hole in damp paper. Holes enlarge over an edition ("small holes would eventually enlarge when printing multicolor editions", so they are taped [P, NDI]).
  - **[E] 0.1–0.3 mm per impression**, as the needle's diameter against the hole's.
  - With two pins a distance L apart, the rotational error is the slop divided by L, so far-apart pins are better [P, NDI]: 0.2 mm over 300 mm is 0.04°.
- **The paper's size [M] [D].**
  - Paper expands by **0.05–0.12 % per % of moisture** content: longer fibers give less [M, Uesaka & Moss in Lindner 2018].
  - Cross-direction wet expansion of machine papers is 2.4–2.75 % [M, Lindner, citing Figueiredo]. Freely dried sheets expand 0.9–1.8 % between 33 and 84 % RH [M, Lindner].
  - A handmade or mould-made sheet has little fiber alignment, so it should expand nearly equally both ways (no measurement found).
  - The expansion levels off once the fiber walls are saturated (fiber saturation, near 25–35 % moisture for cellulose: general wood and pulp data, an estimate for paper). Above that, the extra water is free water in the pores and does not swell the sheet.
  - Damp printing paper at 0.5–1 g/g is above that. It stays at its full wet size while it dries down from there, and starts to shrink only when it falls below saturation.
  - "The paper shrinks in the drying process [and] it is difficult to get it back to the original size when rewetted" [Britannica].
- **What follows [D].**
  - A sheet that dries below fiber saturation between two plates is out by β·ΔMC·L: for β = 0.1 %/% and 5 % of moisture lost, 0.5 % of its size, or 2 mm over 40 cm.
  - All the plates of a color print are printed in one session, with the sheet kept damp between them [P: Cassatt's "eight hours", trapped and pinned methods].
  - Each pass under the roller also stretches the damp sheet a little in the rolling direction. **[E] 0.05–0.3 %** a pass. No measurement was found, and it is a cause of misregistration the trapped-paper method avoids.

## 8. Numbers

| quantity | value | tag |
|---|---|---|
| copper hardness, annealed / half-hard | 40–50 / 80–100 HV | M (Rockwell, converted) |
| zinc hardness | 40–50 HV | M (tables) |
| critical attack angle for chip formation, Cu | 45–85° | M |
| drypoint groove at 1–5 N, half-hard Cu, 90° cone | 53–119 µm wide, 27–60 µm deep | D |
| burr cross-section | ≈ groove cross-section | D |
| burr height | 0.5–1 × groove depth | E |
| good burred impressions, unfaced Cu | 10–40 | P |
| the same, steel-faced | about 100 | P |
| hard ground thickness | 2–8 µm | E |
| soft ground thickness | 5–20 µm | E |
| FeCl₃–Cu activation energy | 24.9 kJ/mol | M |
| FeCl₃ optimum concentration | 2–2.25 M (about 30 %, 35 Bé) | M |
| FeCl₃ on Cu, still, 20 °C | 0.5–2 µm/min | D (checked against P) |
| FeCl₃ on Cu, agitated, 50 °C | 4.4–5.1 µm/min | M |
| black line bite, FeCl₃, Cu | about 40 min | P |
| etch factor (depth / one-side undercut), still bath | 1–1.5 | D |
| nitric 1:8 on zinc, aquatint steps | 15 s–3 min | P |
| rosin density / softening / flow | 1.07 g/cm³ / 70–80 °C / 100–130 °C | M |
| aquatint coverage | 40–60 % | P |
| dust-box grain after a 5 s / 60 s wait (0.6 m box) | ≤ 60 / ≤ 18 µm | D |
| pit depth for an aquatint black | 8–15 µm | D |
| gravure cell depth, highlight to solid | 2–3 to 40–50 µm | M |
| intaglio ink viscosity | 130 Pa·s (bureau ink); 5–50 Pa·s at 20 °C (banknote) | M |
| tarlatan bending rigidity | 10⁻⁵–10⁻⁴ N·m (old path); softened 3.5 × 10⁻⁶ – 1.2 × 10⁻⁵ N·m (contact laws, Peirce W·c³) | E (fabric range M) |
| tarlatan face: free thickness, KES squeeze, stiffness at 10 kPa | 0.25 mm, 30 %, 1.8 × 10⁹ Pa/m | D, M (EMC), E (nesting) |
| width where tarlatan stops holding ink | 1–3 mm | D |
| plate tone after tarlatan / hand wipe | 0.1–0.5 / < 0.05 µm | E |
| hand time per plate inked and wiped | 15–20 min | P (Cassatt's letter) |
| hand-press line load | 130 N/mm (rated) | M |
| pressure on the plate | 0.5–15 MPa | D |
| felts | 1.6 + 6.35 + 3.2 mm | M |
| damp-pack moisture | 0.56 g/g | D |
| hygroexpansion | 0.05–0.12 % per % MC | M |
| wet expansion of a sheet | 1–2.5 % | M |
| ink transfer, non-absorbent / absorbent | ≤ 45–48 % / ≥ 60 % | M |
| ink transfer to damp paper | 55–70 % | E |
| monotype ghost / first pull | 0.4–0.5 | D |
| registration pin slop | 0.1–0.3 mm | E |
| embossed intaglio line height | 20–60 µm | M (banknotes) |

## 9. Unknowns (what to measure)

Ranked by how much each changes a print.
1. **Drypoint burr.**
   - Its height and shape against needle force, angle and tip radius on half-hard copper, and their decay over 1, 5, 10, 20 and 30 pulls.
   - Measure by profilometry or by casting the plate in silicone between pulls, with prints of the same line.
   - Everything about drypoint's look and its edition rests on it.
2. **The press's pressure under the felts.** Over the plate and beside it, with Prescale film on a hand press at working settings, and the felts' compression curve.
3. **Transfer and plate tone.** Weigh a plate before inking, after wiping and after printing, on damp and dry paper, for lines of known depth and width and for an aquatint step wedge. Weigh the film a tarlatan wipe and a hand wipe leave.
4. **Etch rates.** Depth against time, by profilometry of a test strip, for:
   - ferric chloride on copper (35 and 42 Bé; 15, 20 and 25 °C; still and rocked)
   - nitric acid on zinc
   - Dutch mordant on copper, the mordant of Cassatt's period
   - aquatint pit depth against time for a measured grain, and when its dots are lost.
5. **Damp paper.**
   - Moisture content as printed.
   - Its z-compression and bending into a groove: press-in against width and pressure.
   - Its size against moisture through a printing session, and its stretch per pass.
6. **Soft ground.** Thickness as rolled, its yield stress and tack against temperature, and the bared share against pencil force and tracing-paper grade.
7. **Rosin grain.** The size distribution from a dust box against waiting time (by microscope), and the spread of molten dots on copper.
8. **Tarlatan.** Its bending rigidity dry and softened (a Peirce bending length of softened tarlatan would replace the 2–3 cm estimate), its yarn diameter and spacing, and its KES-FB3 compression curve (T₀, EMC, RC).

## Sources

- [Peirce 1930] F. T. Peirce, "The 'handle' of cloth as a measurable quantity", *J. Text. Inst.* 21, T377–T416 (1930): the cantilever bending length and B = W·c³ (ASTM D1388).
- [van Wyk 1946] C. M. van Wyk, "Note on the compressibility of wool", *J. Text. Inst.* 37, T285 (1946); [de Jong et al. 1986] S. de Jong, J. W. Snaith, N. A. Michie, "A mechanical model for the lateral compression of woven fabrics", *Text. Res. J.* 56, 759 (1986); [Hu & Newton 1997] J. Hu, A. Newton, "Low-load lateral-compression behaviour of woven fabrics", *J. Text. Inst.* 88(3), 242 (1997): van Wyk's p = K/(T − T_c)³ for fabrics. KES-FB3 conventions (T₀ at 0.5 gf/cm², T_m at 50 gf/cm², EMC): Kawabata's system as used in e.g. "Effect of weft yarn twist level on mechanical and sensorial comfort of 100 % woven cotton fabrics", *Fashion and Textiles* 5 (2018).
- Tarlatan weights and counts: TNTEX tarlatan 15 and 17 threads, stiff, 35 and 41 g/m² (tntex.eu); Whaleys Bradford cotton tarlatan, 55 g/m²; Talas starched cheesecloth/tarlatan, about 22 threads per inch.
- [Hetényi 1946] M. Hetényi, *Beams on Elastic Foundation* (1946): a beam on a Winkler foundation under a line load.
- [Johnson 1985] K. L. Johnson, *Contact Mechanics* (1985): Hertz contact; full contact of a wavy surface at p* = π·E*·A/λ. [Greenwood & Williamson 1966] "Contact of nominally flat surfaces", *Proc. R. Soc. A* 295, 300: real contact area in proportion to load.
- [Met 354635] Rembrandt van Rijn, *Clump of Trees with a Vista*, 1652, drypoint, second of two states, plate 12.3 × 21.1 cm, The Metropolitan Museum of Art 20.46.4, open access (https://www.metmuseum.org/art/collection/search/354635; image https://images.metmuseum.org/CRDImages/dp/original/DP814786.jpg). [Met 365015] James McNeill Whistler, *Weary*, 1863, drypoint on Japan tissue, fourth state of six, plate 19.6 × 12.8 cm, 83.1.28 (https://www.metmuseum.org/art/collection/search/365015; image DP813372.jpg). Metadata read from the Met's collection API; the profiles were measured by the review (§5.7).
- [Stücker et al. 2001] M. Stücker, M. Geil, S. Kyeck, K. Hoffman, A. Röchling, U. Memmel, P. Altmeyer, "Interpapillary lines — the variable part of the human fingerprint", *J. Forensic Sci.* 46(4), 857–861 (2001), PubMed 11451067 (abstract): papillary ridges 59.0 ± 19.2 µm high, 435.5 ± 57.4 µm wide, 484.9 ± 70.6 µm apart, by laser profilometry of silicone prints.
- [Hamilton et al. 2020] "Fingerprint ridges allow primates to regulate grip", *PNAS* 117 (2020), doi:10.1073/pnas.2001055117: a typical ridge 80 µm high, its crest 0.3 mm round.
- [Westergaard 1939] H. M. Westergaard, "Bearing pressures and cracks", *J. Appl. Mech.* 6, A49–A53 (1939); as in Johnson, *Contact Mechanics*, §13.2: the contact of a sinusoidal surface, sin²(πa/λ) = p̄/p*, p* = π·E*·Δ/λ, and its pressure.
- Wikipedia, "Drypoint" (read 2026-11): "Ink tends to pile up in the lee of the burr"; wiping by hand along the lines may leave "a light gray line"; wiping perpendicular can "increase the pile of ink on the other side of the line"; the deepest strokes print "a feathery black line with a fine, white center". General reference given there: John Ross, *The Complete Printmaker*.

- [Aoki & Murakami 1962] N. Aoki, I. Murakami, "Studies of the Mechanism of Intaglio Printing, on Ink Transfer in the Printing (Part 1)", *Bulletin of the Technical Association of Graphic Arts of Japan* 5(10), 40–53 (1962), doi:10.11413/nig1958.5.40. Abstract read: https://www.jstage.jst.go.jp/article/nig1958/5/10/5_10_40/_article
- [Pritchard & Finkle 1964] cited for transfer maxima in "Flow visualization of the liquid emptying process in scaled-up gravure grooves and cells", *Chem. Eng. Sci.* (2005): https://www.sciencedirect.com/science/article/abs/pii/S0009250905006548 (search summary).
- [Rothstein JNNFM 2012] "Effect of viscoelasticity on liquid transfer during gravure printing", *J. Non-Newtonian Fluid Mech.* 175–176, 64–75: https://people.umass.edu/jpr/pub_files/JNNFM2012v175-176p64-75.pdf (search summary).
- [Challen & Oxley 1979] J. M. Challen, P. L. B. Oxley, *Wear* 53, 229–243 (1979), via https://www.sciencedirect.com/topics/engineering/chip-removal (summary).
- [Sedriks & Mulhearn 1963, 1964] *Wear* 6, 457–466; *Wear* 7, 451–459, as summarized in "Toughness and the transition between cutting and rubbing in abrasive contacts", *Wear* (2006): https://www.sciencedirect.com/science/article/abs/pii/S0043164806001785 (abstract).
- [Wang & Subhash 2002] H. Wang, G. Subhash, "Mechanics of mixed-mode ductile material removal with a conical tool and the size dependence of the specific energy", *J. Mech. Phys. Solids* 50, 1269–1296 (2002): https://impact.ornl.gov/en/publications/mechanics-of-mixed-mode-ductile-material-removal-with-a-conical-t/ (abstract only; cited for a conical tool's mixed cutting and ploughing, not for numbers).
- [WGA] Web Gallery of Art, "Major techniques of printmaking": https://www.wga.hu/tours/german/techniqu.html ("if the needle is perpendicular to the plate, it throws burr on both sides").
- [Tada, Paris & Irwin] H. Tada, P. C. Paris, G. R. Irwin, *The Stress Analysis of Cracks Handbook*, 3rd ed. (ASME 2000): the crack-opening displacement of a crack of length 2a under uniform pressure σ, (4σ/E′)√(a² − x²), and under point forces P at its middle, (4P/(πE′))·acosh(a/|x|); stress intensities σ√(πa) and P/√(πa). Standard results, not re-read for this note.
- [Hokkirigawa & Kato 1988] K. Hokkirigawa, K. Kato, *Tribology International* 21(1), 51–57, values via https://www.sciencedirect.com/topics/materials-science/sliding-friction and the University of Twente thesis "Wear particles formation in cold rolling": https://www.utwente.nl/en/et/ms3/research-chairs/stt/thesis-and-assignments/melkamumekicha-phd-thesis.pdf (summaries).
- [Bucaille et al. 2001] J.-L. Bucaille, E. Felder, G. Hochstetter, *Wear* 249, 422–432: https://www.sciencedirect.com/science/article/abs/pii/S0043164801005385 (abstract).
- [finishing data] Typical Ra of finishes (mirror polish about 0.05 µm; cold-rolled mill finish 0.2–1.0 µm): e.g. https://drametal.com/blog/surface-finish-standards-explained/ (a guide, not a measurement of printmaking plates).
- [Alcobra] Copper product guide: https://alcobrametals.com/copper-product-guide/ ; [MakerStage] https://www.makerstage.com/resources/copper-c110-guide ; [Tuckamore Press] "Copper Plates for Intaglio Printmaking": https://tuckamorepress.com/blogs/technical-deep-dives/copper-plates-for-intaglio-printmaking
- [Harvey-Lee] Elizabeth Harvey-Lee, Drypoint: https://www.elizabethharvey-lee.com/techniques/intaglio/drypoint.htm ; Soft-ground: https://www.elizabethharvey-lee.com/techniques/intaglio/soft_ground_etching.htm
- [Wikipedia] Drypoint: https://en.wikipedia.org/wiki/Drypoint ; Etching revival: https://en.wikipedia.org/wiki/Etching_revival ; Aquatint: https://en.wikipedia.org/wiki/Aquatint
- [Masterworks] https://news.masterworksfineart.com/2017/01/29/drypoint ; [Grokipedia] https://grokipedia.com/page/Drypoint (weak).
- [Kiekeben manual] F. Kiekeben, *Intaglio Manual* (2021): https://nontoxic-print.com/wp-content/uploads/2024/09/FK_Intaglio-Manual-2021.pdf (read in full as extracted text); Edinburgh etch via https://www.polymetaal.nl/beguin/mape/edinburgh_etch.htm ; [Jackson's] https://www.jacksonsart.com/media/pdf/Etching_COPPER_with_ferric_chloride.pdf and aquatint blog https://www.jacksonsart.com/blog/2025/05/05/introduction-to-aquatint-printmaking/
- [Hydrometallurgy 2010] "Application of ferric chloride both as oxidant and complexant to enhance the dissolution of metallic copper": https://www.sciencedirect.com/science/article/abs/pii/S0304386X10002124 (abstract).
- [McGill thesis] "The kinetics of copper etching in ferric chloride-hydrochloric acid solutions": https://escholarship.mcgill.ca/concern/theses/s4655h76p (abstract). [Trans. IMF 1966] "Ferric Chloride as an Etching Material", *Trans. IMF* 44(1): https://www.tandfonline.com/doi/abs/10.1080/00202967.1966.11870005 (abstract). [Cu-ETP] "Chemical etching of Cu-ETP copper", *J. Mater. Process. Technol.*: https://www.sciencedirect.com/science/article/abs/pii/S0924013605000816 (abstract).
- [MG Chemicals 415] datasheet: https://www.farnell.com/datasheets/2867387.pdf ; [Fortex] https://www.fortex.co.uk/product/ferric-chloride-etchant/ ; [Alliance] https://alliancechemical.com/blogs/articles/old-school-cool-pcb-etching-ferric-chloride
- [die patents] US 6582614 and related, ferric chloride at 25–40 Bé (search summary).
- [etch-factor patents] US 9340729 (cupric chloride, K < 3); US 4767661 (nitric, about 2:1) (search summaries).
- [Britannica] Printmaking, etching and colour printing: https://www.britannica.com/art/printmaking/Etching and https://www.britannica.com/art/printmaking/Metal-graphic ; Dutch mordant: https://www.britannica.com/technology/Dutch-mordant ; [WGA] https://www.wga.hu/tours/german/techniqu.html
- [Online Art Center] http://www.onlineartcenter.com/intaglio.html ; [EtchingInsights] https://www.etchinginsights.com/aquatinting-on-zinc-plates/ ; [UMBC] https://userpages.umbc.edu/~custen/ART320/HISTORICAL/aquatint.html and soft ground https://userpages.umbc.edu/~custen/ART320/HISTORICAL/softground.html
- [sonic.net] aquatint process article: http://www.sonic.net/aquatint/PROCESS/aquatintarticle1/processaqu1.html ; [ArtWeb] https://blog.artweb.com/how-to/aquatint/ ; [Mitjà] C. Mitjà, Heliogravure II and V: https://carlesmitja.net/2016/01/15/heliogravure-ii-stochastic-screen/ and https://carlesmitja.net/2016/07/12/heliogravure-v-about-etching/
- [gravure cylinder guide] https://www.regalwoven.com/what-is-rotogravure-cylinder (maker's guide, for cell depths) and https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9519735/
- [CAMEO] Rosin: https://cameo.mfa.org/wiki/Rosin ; Etching ground: https://cameo.mfa.org/wiki/Etching_ground ; [ChemicalBook] rosin: https://www.chemicalbook.com/ChemicalProductProperty_EN_CB1185171.htm
- [UNSW] DART2140 Etching: https://www.making.unsw.edu.au/making-centre/learnresources/printmaking-learning-resources/dart2140-printmaking-etching/ ; [WetCanvas] soft-ground recipe thread and "pressure needed for intaglio?": https://www.wetcanvas.com/forums/topic/pressure-needed-for-intaglio/
- [Lybrook] J. Lybrook, polymer photogravure procedures: https://intaglioeditions.com/procedures/polymer_photogravure.html ; [Takach] tarlatan and blankets: https://shop.takachpress.com/tarlatan-stiff-yard/ , https://www.takachpress.com/access/blankets.htm ; [Grand Palais RMN] https://ateliers.grandpalaisrmn.fr/en/engraving/from-inking-to-printing ; [Waterloo] https://printmaking.uwaterloo.ca/plate-preparation-applying-ink/
- [ink patents] US 8636307 (Sicpa, 1–40 Pa·s at 40 °C), US 4240347 (banknote press: 50–500 P ink, recess and ink depths), US 9376579, US 11148447, CA2559557A1, WO2018209389A1 (search summaries).
- [press load] Rollaco Kudos etching press: https://rollacopresses.co.uk/kudos-etching-press/ ; Koenig & Bauer patents (line pressure ≥ 1000 N/cm) and Mechanika 20(3) 327–331 (2014): https://mechanika.ktu.lt/index.php/Mech/article/download/7393/3776 ; US 6176522 (30–40 MPa) (search summaries).
- [damp paper] https://handprinted.co.uk/blogs/blog/inking-and-printing-an-etching ; https://nontoxic-print.com/intaglio-manual-intaglio-printing/ ; damp-pack quantities: http://www.briarpress.org/31251 (search summaries).
- [Lindner 2018] M. Lindner, "Factors affecting the hygroexpansion of paper", *J. Mater. Sci.* 53, 1–26 (2018): https://link.springer.com/article/10.1007/s10853-017-1358-1 (read as extracted text).
- [registration] https://www.nontoxicprint.com/perfectregistration.htm ; http://www.ndiprintmaking.ca/p_58/
- [Clark Art] printmaking techniques: https://www.clarkart.edu/microsites/impressionist-line/about/printmaking-techniques ; [TheCollector] https://www.thecollector.com/monotypes-by-edgar-degas/ ; [Hamptons Art Hub] https://hamptonsarthub.com/2016/03/28/art-review-the-modern-and-strange-beauty-of-monotypes-by-degas/
- [FAMSF] C. Taylor, "How-To with the Conservators: Mary Cassatt's Color Prints": https://www.famsf.org/stories/how-to-conservators-cassatt-color-prints (blocked; via search summaries and research §5).
- Cassatt-specific claims are from notes/research/cassatt_materials.md §5 (branch `notes/cassatt-materials`), with its tags.
- Not read and needed: A. Stijnman, *Engraving and Etching 1400–2000* (2012), the standard technical history (plates, grounds, mordants, presses); N. M. Mathews & B. S. Shapiro, *Mary Cassatt: The Color Prints* (1989), for her plates, inks, mordant and registration; Part 2 of Aoki & Murakami; Kawabata data for starched cotton gauze.
- [Pheasant, Bodyspace] S. Pheasant, C. Haslegrave, *Bodyspace: Anthropometry, Ergonomics and the Design of Work*: hand breadth at the metacarpals, as tabulated by City University of Hong Kong's online anthropometry (https://personal.cityu.edu.hk/meachan/online%20anthropometry/chapter2/Ch2-29.htm) and RoyMech (https://www.roymech.co.uk/Useful_Tables/Ergonomics/Human_sizes.html).
- [Del Pesco] Belinda Del Pesco, collagraph inking: a dauber rolled from two-inch strips of felt: https://www.belindadelpesco.com/collagraph-tutorial.html/
