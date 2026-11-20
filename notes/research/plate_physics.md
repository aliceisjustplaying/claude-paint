# The plate's physics: hardness, the burnisher and scraper, and metal optics

Research for phase P2 of notes/plan-intaglio.md (`--@ plate`; crates/paint/src/plate.rs). It extends notes/research/intaglio_physics.md (branch `notes/intaglio-plan`, cited as **IP §n**), which sets the drypoint groove, the burr and the regimes of ploughing and cutting. This note covers what P2 needed beyond it: how a hardness number becomes a contact pressure, the burnisher's and scraper's mechanics, and how polished copper and zinc look. Compiled 2026-11-15.

Tags as in IP: **[M]** measured, with a source; **[D]** derived here; **[P]** practitioners' report; **[E]** estimate, with the reason.

## 1. What the model keeps (plan-intaglio I1)

The plate is laid out at the printing sheet's pitch, about 0.18 mm, while a drypoint line is 40–170 µm wide (IP §1.3). So each cell keeps its openings binned by width, each bin's open share, volume and width, together with the burr's volume, share and side, the line's direction and the surface's own level. The tools' paths are kept too, so a crop is rasterized again at its own pitch.

Every tool keeps the metal balance: the metal above the original flat is burr plus raised level, less openings and lowered level. Ploughing, burnishing and the roulette conserve it [D, plastic flow is incompressible]. Cutting and scraping lower it by exactly what they report as chips.

## 2. Hardness and elastic constants

- **Meyer, not Vickers [D].** A Vickers number is the load over the indent's sloping area. The 136° pyramid makes that area 1/sin 68° = 1.0785 times the projected one. The load a scratch or an indent carries per *projected* area (Meyer's hardness) is therefore H = 1.0785 × HV × 9.807 MPa. Half-hard copper (90 HV) is 0.95 GPa, annealed copper and zinc (45 HV) 0.48 GPa. IP §1.3's table used 0.9 GPa: widths here come out about 3 % narrower than that table's.
- **Elastic constants [M, handbook tables].**
  - Copper: E 125 GPa (IP §1.1: 120–130), ν 0.34.
  - Zinc: E 100 GPa (IP §1.1), ν 0.25.
  - Hardened tool steel: E 210 GPa, ν 0.30.
  - The steel–copper contact's reduced modulus is 1/E* = (1 − ν₁²)/E₁ + (1 − ν₂²)/E₂ = 88 GPa.

## 3. Burnisher, scraper, roulette

### 3.1 The burnisher [D]
A polished steel sphere of radius R (5–20 mm, IP §1.7) rubbed under F (10–50 N, IP §1.7).

- **On the flat plate (Hertz).**
  - Contact radius a = (3FR/4E*)^(1/3), peak pressure p₀ = 3F/(2πa²), elastic approach δ = a²/R.
  - 30 N on a 10 mm burnisher over copper: a = 137 µm, p₀ = 0.76 GPa, δ = 1.9 µm.
  - That is below half-hard copper's hardness, so the flat plate doesn't flow. This matches IP §1.7: burnishing works on the highs first.
- **On the burr: plastic bearing.**
  - Take ridges of triangular section of height h that cover a share s of the plate. The share of metal above a level z is s(1 − z/h).
  - A sphere whose bottom is at z₀ touches what lies above z₀ + r²/2R. With the metal flowing at H wherever touched, the load carried is H·s·π·R·(h − z₀)²/h.
  - Setting that to F gives **z₀ = h − √(F·h/(π·H·s·R))**. Everything above z₀, and above the sphere's sag off the path, is pressed flat.
  - Example: a 47 µm burr over 6 % of the plate, under 30 N and R = 10 mm, comes down to about 20 µm in the first pass and is gone in three.
  - The share s is averaged over the burnisher's reach into the ridges, ρ = √(2Rh), so the answer doesn't depend on the cell pitch.
  - Once what is left is lower than the contact's elastic approach δ there, it can't hold the burnisher off the land, and it goes flat.
- **Where the flattened burr goes.** The fin leans away from its line (IP §1.4), so pressed flat it folds onto the land beside the line, which it raises by its metal. The line stays open while the burr holds the burnisher off. That is the order the plan asks for: the burr goes before the line does.
- **Then the rims (Hill's slip-line fields).**
  - The edge of a groove is a wedge of metal: the plate's flat on one face, the groove's free wall on the other, included angle φ = π − (the wall's slope).
  - Loaded on one face beside a free face, such a wedge yields at **p = 2k(1 + φ − π/2)**. For the flat surface (φ = π) that is Prandtl's 2k(1 + π/2), the hardness, and for a right angle 2k [M, slip-line theory: Hill 1950; the ScienceDirect "slip-line field" summary gives p = 2k(2γ + 1 − π/2) for an obtuse wedge under one-sided load].
  - The rim of a 30° needle's groove (walls at 60°) yields at 0.59 H. That is 0.56 GPa on half-hard copper, under the 30 N burnisher's 0.76 GPa.
  - Where the Hertz pressure p₀√(1 − r²/a²) exceeds the rim's limit, the rim is pushed down plastically by the contact's approach there, δ − r²/2R, while the land around it springs back. Each pass therefore moves 2·δ·d of metal per length into a groove of depth d: its two rims, each a depth's width of metal that has lost its constraint, sunk by δ.
  - The land beside the groove comes down by the metal it gave. For the 30 N burnisher on a 63 µm deep line that is about 12 % of the groove's section a pass, so a line goes in a dozen passes. Workshops describe burnishing out a line as long, repeated rubbing [P].
  - **[D, order of magnitude]** The rim zone's width, taken as the groove's depth, is the slip-line field's scale, not a measured number.

### 3.2 The scraper [D]
- **The edge.** A three-sided scraper's edges are 60° (an equilateral section, IP §1.7).
- **Depth a stroke.** Drawn across the plate with its edge set to cut, its front face carries the load at the hardness, as IP §1.3's scratch law has it for a cone. For an edge of length b and half-angle ψ = 30°, F = H·b·t·tan ψ, so **t = F/(H·b·tan ψ)**. Example: 5 N on an 8 mm blade over half-hard copper cuts 1.1 µm a stroke.
- **What it takes.** A stroke takes the burr off above the plate first, then a slab t deep. A V-groove deeper than t loses its top and keeps its V, so its width and share go as (d − t)/d and its volume as the square. A shallower groove is gone.
- **What it leaves.** A shallow dish, its level below the plate's flat (IP §1.7: scraped corrections print grey until burnished level). The scrapings are counted.

### 3.3 The roulette [D] [E]
- **One tooth at a time.** A tooth bears on the plate while the wheel rolls, and its pit is a static indent at the hardness over the whole projected contact: πa² = F/H.
- **The pit.** The depth follows the tooth's point: a sphere of the tooth's radius blending into a cone, as for the needle. The pits lie a pitch apart.
- **The burr.** Each pit throws up a ring of its own volume [D, incompressibility], as tall over its depth as a drypoint burr: **[E] 0.75**, IP §1.4's.
- **[E] The tooth's cone, 45°.** A roulette's teeth are small pyramids, and no measured angle was found.

## 4. Metal optics (plan-intaglio I4)

- **Fresnel from the complex index.** A metal reflects specularly. Its reflectance follows from the complex index N = n + ik:
  - cos θ_t = √(1 − sin²θ/N²) (complex)
  - r_s = (cos θ − N cos θ_t)/(cos θ + N cos θ_t)
  - r_p = (N cos θ − cos θ_t)/(N cos θ + cos θ_t)
  - R = (|r_s|² + |r_p|²)/2 for unpolarized light
  - At normal incidence this is ((n − 1)² + k²)/((n + 1)² + k²).
- **Copper [M].** Johnson & Christy 1972 (Phys. Rev. B 6, 4370), the standard set for the noble metals, read from refractiveindex.info.
  - At 520 nm n = 1.17, k = 2.61, so R = 0.62. At 660 nm n = 0.22, k = 3.75, so R = 0.94.
  - The step between them is copper's colour.
- **Zinc [M, with spread].** Werner, Glantschnig & Ambrosch-Draxl 2009 (J. Phys. Chem. Ref. Data 38, 1013), derived from reflection electron energy-loss spectra.
  - At 551 nm n = 0.93, k = 4.99, so R = 0.87, rising slowly to the red: a bluish grey.
  - The sources disagree. Motulevich & Shubin 1969 give k ≈ 10 near 580 nm (R = 0.97). Querry 1987's tabulated set gives n ≈ 13, which is not a metal's (probably a pressed or oxidised sample).
  - Werner's set is taken as the middle and most recent. **Measure it:** a polished printmaker's zinc plate's spectral reflectance would settle it, and tarnish with it.
- **A V-groove traps light [D].** Light going straight into a V-groove with walls at slope α reflects from wall to wall until it climbs out.
  - Each reflection takes the Fresnel share at its incidence. With walls at 60° (a 30° needle) a ray hits both walls before leaving.
  - So the groove sends back R(θ₁)·R(θ₂)…, which is why a line on polished copper reads darker than the plate.
  - The model traces this in the groove's section over 32 entry points.
- **The burr's faces** reflect at their own slope.
- **The light.** In a uniform room light a cell's colour is the area-weighted sum of the flat at normal incidence, each opening's return at its walls' slope, and the burr's faces. Under a lamp (`--mode relief`, `--light`) the cells' mean height is lit by the canvas's own light code (`PreparedLighting`).
- **Not modelled yet:**
  - the environment's own pattern (the dark room a plate reflects, the printer's white tissue)
  - tarnish
  - the spread of light by the burr's torn faces

## 5. Estimates still standing

| what | value | why |
|---|---|---|
| burr height / groove depth | 0.75 | IP §1.4 [E]: no profilometry of drypoint burr (IP Unknown 1) |
| needle point radius | 25 µm | IP §1.3 [E] 10–50 µm |
| hand speed with a needle, burnisher, scraper | 30 mm/s | plan §3.1 [E] 20–40 mm/s |
| copper, eighth-hard | 65 HV | plan §2.1 [E], between the measured tempers |
| roulette tooth half-angle | 45° | no source found |
| rim zone width | the groove's depth | slip-line scale, order of magnitude |
| critical attack angle | 60° | IP §1.2 [M] 45–85°, its middle |

## Sources

- P. B. Johnson, R. W. Christy, "Optical constants of the noble metals", *Phys. Rev. B* 6, 4370–4379 (1972), doi:10.1103/PhysRevB.6.4370; tabulated in the refractiveindex.info database (CC0), `database/data/main/Cu/nk/Johnson.yml`.
- W. S. M. Werner, K. Glantschnig, C. Ambrosch-Draxl, "Optical constants and inelastic electron-scattering data for 17 elemental metals", *J. Phys. Chem. Ref. Data* 38, 1013–1092 (2009), doi:10.1063/1.3243762; refractiveindex.info `Zn/nk/Werner.yml`.
- G. P. Motulevich, A. A. Shubin, "The optical constants of zinc", *Sov. Phys. JETP* 29, 24–26 (1969); refractiveindex.info `Zn/nk/Motulevich.yml` (the spread).
- M. R. Querry, *Optical constants of minerals and other materials from the millimeter to the ultraviolet*, CRDEC-CR-88009 (1987); refractiveindex.info `Zn/nk/Querry.yml` (the spread).
- R. Hill, *The Mathematical Theory of Plasticity* (Oxford, 1950), ch. 8 (compression of a wedge; the flat punch); the one-sided obtuse-wedge limit p = 2k(1 + φ − π/2) via the ScienceDirect topic summary "Slip line field": https://www.sciencedirect.com/topics/engineering/slip-line-field
- K. L. Johnson, *Contact Mechanics* (Cambridge, 1985), ch. 4 (Hertz: contact radius, peak pressure, approach).
- K. Hokkirigawa, K. Kato, "An experimental and theoretical investigation of ploughing, cutting and wedge formation during abrasive wear", *Tribology International* 21(1), 51–57 (1988), doi:10.1016/0301-679X(88)90128-4: degree of wear near 0 ploughing, toward 1 cutting (via https://scispace.com/papers/an-experimental-and-theoretical-investigation-of-ploughing-3zwhg6klr2 and https://www.tandfonline.com/doi/full/10.1080/10402004.2016.1206647, summaries).
- Vickers geometry (136° pyramid; HV = 1.8544 F/d²): ISO 6507-1.
- Elastic constants of copper, zinc and tool steel: standard handbook tables (ASM Handbook vol. 2; CRC Handbook), as IP §1.1.
