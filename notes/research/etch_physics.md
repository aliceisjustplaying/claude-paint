# Etching a plate: the ground, the bath, the undercut and the aquatint

Research for phase P4 of notes/plan-intaglio.md (`--@ etch`; crates/paint/src/etch.rs and crates/paint/src/plate/etched.rs). It extends notes/research/intaglio_physics.md (branch `notes/intaglio-plan`, cited as **IP §n**), which sets the workshop facts: the grounds (§2), the mordants (§3), the undercut (§3.4) and the aquatint (§4). This note covers what P4 needed beyond it: a mordant's rate from mass transfer, with no fitted boundary layer; the bubbles of nitric acid; the geometry of the bite under a ground and under rosin dots; and the look of a ground. Compiled 2026-11-16.

Tags as in IP: **[M]** measured, with a source; **[D]** derived here; **[P]** practitioners' report; **[E]** estimate, with the reason.

## 1. The ground and the etching needle

- **Hard ground** is rolled on with a leather roller along the painter's points, 4 µm thick by default. **[E]** IP §2.1 puts it at 2–8 µm. It covers what is already in the metal: a ground laid warm flows into old lines.
  - **It lies where the roller went.** The roller meets the plate along a line its own length (150 mm by default **[P]**: "the ideal roller is about 150 mm long and 50 mm in diameter, covered in leather", *Etching: a guide to traditional techniques*, ch. 11; Artools sell 100 and 200 mm leather rollers). A cell is grounded where that line, moved along the points, crossed over half of it (crates/paint/src/plate/hand.rs). A cell it never crossed is bare and bites as an open face.
  - **Hand time:** the roller's path at 100 mm/s **[E]** and 8 minutes to warm the plate and let it cool **[E]**.
- **The needle** (`k:needle`) is the drypoint needle of plate_physics.md, held lightly (default 0.3 N **[E]**, plan §3.1's "under 0.3 N"). Two things happen along its path:
  - **In the ground.** The point parts the ground where it is wider than the ground is thick. The window it leaves is the point's width at the ground's top, the height t + d above its lowest point, with t the ground's thickness and d its mark in the metal.
  - **On the metal.** The ground carries a little of the load: its flow pressure, 5 MPa **[E]** (waxes and bitumens indent at 1–10 MPa, a hundredth of copper's hardness), over the point's section in the ground. That is a few mN against the hand's tenths of a newton. The metal carries the rest and is marked by the scratch law, w = √(8F/πH).
  - At 0.3 N a 40 µm point marks half-hard copper about 3 µm deep, with a burr of the mark's volume. It ploughs, never cuts. Elastic contact alone would need far less load than any hand gives: the Hertz yield onset of a 25–50 µm point on copper is under a millinewton. So an etching needle always scratches a little, as printmakers know and do not mind: the acid opens the line anyway.
- **Bitten lines have no burr.** The needle's burr stands in the window's foot. The acid attacks a fin from both faces, and an inward offset z of a triangle scales it about its incentre by 1 − z/r, with r its inradius [D, geometry]. A 1–3 µm burr is gone in the first micrometres of bite.
- **Stopping-out varnish** (`k:stop_out(pts, …)`) is asphaltum and rosin in turpentine brushed along the painter's points: no acid reaches what it covers. It is kept per cell, at the plate's pitch, where the strip covers over half the cell. Nothing tells it where to go but the brush's path; its footprint, edge and amount come from the brush and the liquid (§1.1).

### 1.1 A stroke of stopping-out [M] [D] [E]

- **The brush's load.** A sable round's belly is a cone of its width d and 4.7 d long **[M]** (Rosemary & Co Series 33: 15.7/3.4, 20/3.9 and 24.2/5.2 mm hair over width for sizes 4, 6 and 8), the liquid filling 0.6 of it **[E]** (the wet brush's own estimate, wet_pastel.md): V = 0.6 · π/12 · d² · 4.7 d, 44 mm³ for a size 6 (3.9 mm), times the load asked for.
- **The width it lays.** A wet tuft is a cone, so its mark widens with the pressure until its whole belly is down at pressure 0.85 (the bristle model's law, bristle.rs `P_FULL`): w = d · min(p/0.85, 1).
- **The film** is 20 µm thick **[E]**, the varnish's thickness for its look (§5). The brush lays w · 20 µm a millimetre of stroke until V is spent, then runs dry: a size 6 at pressure 0.5 lays some 950 mm of stroke on a full belly; a 1 mm brush a tenth full, 3.7 mm.
- **The edge.** The laid strip is a ridge of section A = h·w. On a clean plate the varnish wets the metal completely, so its edge creeps out by Cox–Voinov, θ³ = 9 ln(L/λ) · ηU/γ, with the ridge's contact angle θ = 3A/(2a²) for a parabolic section of half-width a. Integrated, a⁷ = a₀⁷ + 7γ(3A/2)³t/(9η ln(L/λ)) **[D]**, Tanner's law for a two-dimensional ridge.
  - γ = 27 mN/m, turpentine's **[M]** (handbook values for turpentine and α-pinene, 26–27 mN/m near 20 °C).
  - η = 1 Pa·s **[E]**: makers call stop-out of "thicker consistency" than straw-hat varnish, so above a brushing paint's 0.1–0.3 Pa·s; no measurement.
  - ln(L/λ) = 9.2 **[M]** (Bonn et al., *Rev. Mod. Phys.* 81, 739 (2009): ln 10⁴ fitted across fluids in complete wetting).
  - t = 10 minutes **[P]**: stop-out dries in about 10 minutes under a fan (Glasgow Print Studio's handbook) and in about an hour in still air (MungoLux's instructions); a film stops flowing well before it is dry to the touch, so the shorter.
  - A 2.3 mm strip (a size 6 at pressure 0.5) creeps 26 µm each side. The viscosity is the weak estimate: at 0.1 Pa·s it would creep 0.18 mm, at 10 Pa·s 3 µm. Either way the edge is mostly the brush's at the pitch a plate is laid out at (some 0.1–1.5 mm a cell).
- **Where the brush comes down and lifts** it leaves the disc of its tuft, the strip's width across.
- **Hand time** is the stroke at 20 mm/s **[E]**.
- **Solvent** (`k:clean()`) takes the ground, the varnish and the rosin off. What the acid opened stays as the cells' openings.
- **Smoking** a ground with a taper (1911 Britannica) is not modelled: the plan does not cover it.

## 2. The bath: a rate from mass transfer

Every mordant here bites at the rate its oxidant reaches the metal:

  **r = V_M · k · C / ν**

- V_M is the metal's molar volume: copper 7.09, zinc 9.16 cm³/mol [M].
- C is the oxidant's concentration in the bath.
- ν is the mol of oxidant used per mol of metal, from the reaction's stoichiometry:
  - ferric chloride: 2 Fe³⁺ per Cu or Zn [M]
  - nitric acid on copper: 8/3 HNO₃, giving 2/3 NO [M, textbook]
  - nitric acid on dilute zinc: 2.5 HNO₃, giving 1/4 N₂O [M, textbook]
  - Dutch mordant: 1/3 ClO₃⁻ per Cu (ClO₃⁻ + 6 H⁺ + 3 Cu → Cl⁻ + 3 Cu²⁺ + 3 H₂O) [M, textbook]
- k = D/δ is the mass-transfer coefficient. Nothing is fitted in it.

### 2.1 D and the bath's liquid [M] [D]
- **Limiting ionic diffusion coefficients** at 25 °C [M, CRC Handbook, "Ionic conductivity and diffusion at infinite dilution"]: Fe³⁺ 0.604, Cl⁻ 2.032, H⁺ 9.311, NO₃⁻ 1.902 and ClO₃⁻ 1.720 × 10⁻⁹ m²/s.
- **The salt's diffusivity.** Where no other electrolyte carries the current, the salt diffuses as a whole, at the Nernst–Hartley D = (z₊ + |z₋|)D₊D₋/(|z₋|D₊ + z₊D₋) [D, standard]. That gives FeCl₃ 0.733 and HNO₃ 3.16 × 10⁻⁹ m²/s. Chlorate in Dutch mordant is a minor ion in excess hydrochloric acid, so it moves at its own 1.72 × 10⁻⁹.
- **Viscosity and density** at 20 °C are from the CRC Handbook's "Concentrative properties of aqueous solutions" (Wolf 1966; Söhnel & Novotný 1985) [M], for FeCl₃ 0.5–40 % and HNO₃ 0.5–40 %. For example:

  | FeCl₃ | mol/L | g/cm³ | mPa·s |
  |---|---|---|---|
  | 20 % | 1.457 | 1.1816 | 2.570 |
  | 28 % | 2.189 | 1.2679 | 4.038 |
  | 32 % | 2.595 | 1.3153 | 5.274 |
  | 40 % | 3.496 | 1.4176 | 9.674 |

  The 40 % row matches Environment Canada's 9.655 mPa·s. MG Chemicals' 12 cP is for their commercial etchant, which carries HCl.
- **Stokes–Einstein** gives D at the bath's viscosity: D ∝ T/μ [D]. In 28 % FeCl₃ at 20 °C that is 1.6 × 10⁻¹⁰ m²/s, inside IP §3.1's 1–3 × 10⁻¹⁰.
- **Baumé.** SG = 145/(145 − °Bé), the heavy-liquid scale [M], read back to molarity through the CRC densities. 38 °Bé is about 2.9 M; 45 °Bé is the 40 % stock.
- **Nitric** `parts` of water to one of the 70 % acid (15.8 M [M]) is 15.8/(1 + parts) M [D, neglecting the contraction on mixing].
- **Dutch mordant** is the 1911 Britannica's recipe: hydrochloric acid 100 g, potassium chlorate 20 g, water 880 g. That is 0.165 M chlorate in about 1.1 M HCl, whose CRC row is 1.018 g/cm³ and 1.059 mPa·s.

### 2.2 k: how the bath moves [M] [D]
**What drives it.** Where the oxidant runs out at the face, the solution holds the dissolved metal instead: M_metal·C/ν more mass per volume [D, neglecting the solution's small change of volume]. In 28 % ferric over copper that is 70 kg/m³, 5.5 % of the bath's density. The spent acid is heavier, so it falls.

**Face down** (held off the tray's floor, as Kiekeben has it for ferric [P]). The heavy layer falls away, as from the lower face of a cold plate.
- Sh = 0.54 Ra^¼ for Ra < 10⁷, and 0.15 Ra^⅓ above [M, Lloyd & Moran 1974; Incropera].
- L is the plate's area over its perimeter, and Ra = g(Δρ/ρ)L³/(νD).
- Above 10⁷, k does not depend on L.

**Upright** in a tank. The heavy layer runs down the face: Churchill & Chu 1975 over the whole range, with L the plate's height [M].

**Face up, still.** The heavy layer lies on the face and is stable, so there is no convection. The oxidant arrives by diffusion alone, at the flux C·√(D/(πt)) of a semi-infinite layer.
- The depth then grows as √t: z = (V_M/ν)·2C·√(Dt/π) [D].
- That is the physics of the workshops' warning that a face-up plate in plain ferric bites slowly [P].
- A bite's steps integrate it exactly: ∫k dt = 2t·k(t).

**Rocked** (`rock=true`). A tray tilted ±θ every T, slower than its sloshing period 2L/√(gh), keeps its surface level.
- A wedge of the bath, L²θ/8 in section, crosses the middle each quarter period. So U = L²θ/(2hT) at most there, and (2/π)·U on average [D].
- Over the plate this is laminar flow over a flat plate: Sh = 0.664 Re^½ Sc^⅓ [M, Pohlhausen; Incropera].
- Combined with natural convection as Churchill's cubes, k = (k_n³ + k_f³)^⅓ [M, Churchill 1977].
- The rocking hand's tilt (5°) and period (4 s), the bath's depth (20 mm) and the tray's margin (50 mm a side) are **[E]**: unmeasured.

### 2.3 What comes out [D]
- **Ferric chloride, 28 %, face down, 20 °C:** about **1.1 µm/min**, with δ ≈ 65 µm.
  - That is the middle of IP §3.1's derived 0.5–2 µm/min.
  - It matches the workshop times: a faint line in 5–15 min, black (about 40 µm) in 40 min [P].
  - Rocked face up (5° each 4 s: about 1 cm/s over the plate) it is about 0.3 µm/min, δ ≈ 240 µm: rocking a face-up tray sweeps the spent acid off, but slowly. Room temperature with no agitation is 0.4–0.8 µm/min for circuit boards [P, Alliance].
- **The optimum near 2–2.25 M** [M: McGill thesis; Trans. IMF 1966] comes out of the CRC viscosities, not a rule.
  - D falls as 1/μ while C rises, and the convection brings its own powers. Face down in the Ra^⅓ regime, r ∝ C^{4/3}/μ. Rocked, r ∝ C·D^{2/3}ν^{−1/6}.
  - Either way the rate peaks where d ln μ/d ln C reaches about 1.2–1.3, which the table puts at 2.0–2.2 M.
  - It is "independent of temperature and flow rate", as measured, because both regimes give about the same exponent.
  - A 30 % bath bites faster than 40 %, as Fortex says [P].
- **Temperature.**
  - Ferric chloride takes its measured activation energy, Ea = 24.9 kJ/mol [M, Hydrometallurgy 2010]: ×1.40 per 10 °C from 20 °C. The 20 °C transport is the CRC's.
  - Nitric acid and Dutch mordant are dilute, so their D and viscosity follow water's (Vogel's equation [M]): about 20 kJ/mol.
- **The bath's loading.**
  - The tray tracks the metal the plate has given it, and its oxidant falls by ν per mol, so an old bath bites slower.
  - The tray's capacity follows from the stoichiometry. A 3.5 M ferric bath can take 1.75 M of copper, 111 g/L, against MG Chemicals' rated 100 g/L [M]: a check, not an input.
  - A tray is the plate plus its margin, 20 mm deep **[E]**. Its rate within a bite follows its loading, in steps of a minute.
- **The bath on the clock.** A plate lies in its tray from `immerse` to `lift`, and nothing in the bath depends on what the hand does meanwhile. The bite is therefore worked out when the plate comes out, from the minutes it lay there, in the same steps as a bite of those minutes taken in one. In, other work, out is the same bite as in, wait, out.
  - A plate taken out and put back starts a fresh diffusion layer [D]. Rinsing clears the spent acid, and a still face-up plate's layer grows again as √t from its return.
  - **A plate left in** (for days) bites on, for as long as the session's clock runs. Past a day the steps lengthen to a sixtieth of the time passed, or to the time the bath takes to use a hundredth of its oxidant left, whichever is shorter. Both are numerical choices, not physics: the loading changes as little in each step as in a minute's early on.
  - **A spent bath.** The bath stops biting once its oxidant falls below a thousandth of what it was made up with. That is a bound, not an estimate: all the bath could still dissolve is that share of what it held. Before then the rate falls with the oxidant left, so the depth approaches the stoichiometric capacity over the bitten area.
  - **A bite deeper than the plate** is reported, not modelled. The acid would open the back where the plate is holed, and the back's varnish isn't modelled.
- **Nitric 1:8 on zinc,** rocked and feathered, bites about 1.7 µm/min, **5.6×** ferric on copper in the same rocked tray. IP §3.2 has 5–10× [P].

### 2.4 Nitric acid's bubbles [M] [D] [E]
- **Where.** Nitric acid gives off gas over the bitten metal [P], and under a bubble it does not bite.
- **How big.** A bubble leaves the face at Fritz's diameter, d = 0.0208·θ·√(σ/(gΔρ)) [M, Fritz 1935]. That is 2.2 mm in water for a contact angle of 40° **[E]**: unmeasured on etched zinc.
- **How much gas.** It comes at the reaction's share of the metal bitten (1/4 mol N₂O per Zn), as an ideal gas at the bath's temperature [D].
- **Coverage.** A face covered by bubbles of diameter d at share θ holds (2/3)·d·θ of gas per area. Unswept, the bubbles grow to their departure size and cover a share **[E] 0.5** of the face: grown bubbles sit about a diameter apart.
- **The feather.** Swept every T_f (`feather=true`: 30 s **[E]**, the manuals say only "brushed away"), the cover grows back toward 0.5 with time constant τ = (2/3)·d·θ_max/q. Its mean over a sweep is θ_max·(1 − (τ/T_f)(1 − e^(−T_f/τ))) [D].
- **Effect.** For 1:8 on zinc the mean cover is about 0.15: a feathered bite runs about 1.7× an unswept one.
- **Not modelled:** the bubbles' stirring, which would speed the bite, and the ragged edges an unswept bite leaves.

### 2.5 Dutch mordant [E]
- **No rate.** No measured rate exists (IP §3.2). Where the bath is stirred, chlorate arrives faster than it reacts (the bath "acts slowly"): the rate is the surface reaction's. Still and face up, its chlorate (0.165 M) arrives by diffusion alone, about 0.2 µm/min after ten minutes, and that is the bound. The model takes the lesser of the two.
- **The rate used:** **0.5 µm/min at 20 °C [E]**, from "a very fine-grained aquatint could over-etch after only 10 minutes in Dutch Mordant" (IP §4.2 [P]): a 10 µm grain lost to an isotropic undercut of 5 µm in 10 min.
- **Activation energy:** **50 kJ/mol [E]**, the textbook range of a surface reaction's control, against diffusion's under 25.
- **Stirring** changes it only until the reaction is the slower step: a rocked Dutch bath bites at the reaction's rate.
- **Ferric chloride** in an 1890s Paris box is allowed: it was used for photogravure from the 1880s (IP §3.2).

## 3. The undercut and the slot

- **The etch factor K** is depth over the sideways bite on one side (IP §3.4):
  - ferric 1.2 [D: a still, diffusion-limited bath bites almost isotropically, 1–1.5, the corner under the ground shadowed from the mouth]
  - nitric 1.0 [E: IP's 1–2, its low end, since it "underbites"]
  - Dutch 1.0 [D: a surface reaction bites every bare face at one rate]
  - The 1911 Britannica says Dutch mordant "does not widen the lines to any perceptible degree". Against nitric, which bites faster and lifts the ground's edges with its gas, that holds. K = 1 from the physics is kept, and the discrepancy noted.
- **A line's section.** A line bitten z under a window w₀ has a flat floor as wide as the window and two quarter-ellipses of semi-axes z and z/K under its edges [D].
  - Its area is w₀z + πz²/(2K).
  - Its width at the face (once the ground is off) is **w₀ + 2z/K**: it widens with its depth.
- **The slot.** A narrow line bites slower as it deepens: the acid has to diffuse down it (aspect-ratio-dependent etching, IP §3.4).
  - The front's rate is r·δ/(δ + g·(t + z)), with t the ground's thickness and g = δ/(δ + w) the share of the slot's depth the diffusion layer cannot enter [D, an interpolation].
  - The limits: a narrow slot (w ≪ δ) adds all of it, and a wide dish (w ≫ δ) none, since the layer follows its floor.
  - It integrates exactly over a step: (δ + gt)z + gz²/2 = r·δ·t.
  - This neglects the spreading of the field over a narrow mouth, which speeds a fresh fine line at first. The model errs slow for the first micrometres.
- **Classes.** Lines bared at different times (needled after a bite, or stopped out between bites) bite to different depths. Each cell keeps up to two classes of them; a third merges into the one nearest in depth.
- **Mixed baths.** A cell bitten in two acids is written with the last bath's K.

## 4. Aquatint

### 4.1 The dust box [M] [D] [E]
- **Settling.** A rosin grain (1.07 g/cm³ [M]) settles in still air at its Stokes speed, v = ρgd²/(18µ) with µ = 1.81 × 10⁻⁵ Pa·s [M].
  - Its Reynolds number stays under 1 below about 70 µm. The slip correction is under 3 % above 5 µm.
  - After a wait t in a box of height H, the largest grain still in the air has v·t = H: **d = √(18µH/(ρgt))**. In a 0.6 m box that is 61 µm after 5 s and 18 µm after 60 s, as in IP §4.1.
- **Landing.** A well-stirred cloud settles as a front, each size at its own speed. A plate at the floor, put in after t and left m, takes size d at n(d)·[min(H, v(t + m)) − v·t]⁺ per area [D].
- **The cloud** is **[E]** in every number (Unknown 7):
  - log-normal by mass, median 30 µm, geometric spread 2: a pounded resin
  - 82 g/m³ just after it is stirred: the loading that gives the workshops' 50 % cover (IP §4.1 [P]) on the plan's example (0.6 m, 30 s wait, a minute in)
  - So a 5 s wait covers 80 % with grains up to 61 µm, and a 60 s wait 30 % with grains up to 18 µm: "the longer you wait, the finer the grain" [P].
- **Coverage.** Grains land at random, so they cover 1 − e^(−Σnπd²/4) of the face (a Boolean model).
- **Speckle.** Each cell takes a Poisson count of each size, at its mean over the cell's area: the cell-to-cell scatter of a real grain. A crop drawn again at a finer pitch scatters them again, with the same statistics.
- **Melting** (`k:melt()`). Each grain spreads to **1.3×** its diameter **[E]** (IP §4.1's 1.2–1.5; no contact angle of molten rosin on copper was found).
  - Loose rosin floats off in the bath, and the acid bites the bare face. The verb says so.

### 4.2 The bite between the dots [D]
- **The front.** The acid bites down z between the dots and sideways under them.
  - A point y below the face, inside a dot's edge by u, is reached once (uK)² + y² ≤ z². So the pillar under a dot of radius R is R − √(z² − y²)/K wide at depth y.
  - Once z > KR the dot is lost. Its pillar's tip is then at √(z² − K²R²).
- **The metal left** at depth y covers c(y) = 1 − e^(−πΣn·max(0, R − √(z² − y²)/K)²): the shrunken dots, still a Boolean model.
  - That treats the erosion of a union of discs as the union of eroded discs, which slightly overstates what is left where dots overlap.
- **The pits.**
  - Their volume is ∫(1 − c)dy, Simpson's rule over 64 steps.
  - Their share at the face is 1 − c(0).
  - Their width is the free path between the dots, 1/(2Σn·R): a Boolean model's mean chord, π(1 − c)/S_A.
  - The slot slows their floor, as for a line.
- **Over-biting.**
  - Once every dot is lost, the face drops to the tallest pillar's tip.
  - What is left is one opening over the whole face: the remnants of the pillars, far apart. The pits have merged into an open bite.
- **The tone.** The ink a wipe's yarns leave is IP §4.2's share × (depth − dip), with the yarn's dip r − √(r² − w²/4) for a yarn of radius 75 µm **[E]**.
  - It rises with the bite while the dots stand.
  - It peaks before they are lost: their shrinking widens the gaps the yarns reach into.
  - It is nothing once they are gone (z > KR): "overbitten aquatints print lighter" [P], with no rule capping it.
  - The press and its cloth (P3) will read the same openings with their own reach. `probe` prints this tone as a guide.
- **Stepped tones.** Stopping out between bites gives stepped tones: times add, as Kiekeben has it [P].

## 5. How a ground looks [M] [E]

- **A thin absorbing film over the metal.** The film's face reflects r₁ = ((n − 1)/(n + 1))². The light that enters crosses the film twice and meets the metal, whose Fresnel reflection against the film uses N/n_film. The internal reflections are summed incoherently [D]:

  R = r₁ + (1 − r₁)²·R_m·T²/(1 − r₁R_m·T²), with T = e^(−αt)

- **Bitumen.**
  - Its index is 1.6 [M, asphalt binders by ellipsometry].
  - Its absorption at 550 nm is 0.1 /µm **[E]**: the workshop's "strong tea" (IP §2.1), a double pass through 4 µm leaving 0.45.
  - It rises to the blue as an Urbach tail of 0.3 eV **[E]**. Asphaltenes absorb with an Urbach edge (Mullins), but its slope for an etching ground is unmeasured.
  - So a 4 µm ground reads dark brown over copper, and the 20 µm varnish **[E]** near black.
- **The needle's windows** show the bare metal, bright. Once bitten, they show the walls of the line they open, by the V-groove's trapped light (plate_physics.md §4).
- **Rosin.**
  - Its index is 1.54 [M, CAMEO]. It is pale amber, 0.02 /µm at 450 nm **[E]**, with the same tail.
  - A melted dot is a flat lens 0.39 of its grain's diameter thick (its volume spread to 1.3 diameters).
  - A loose grain is a powder, diffusely 0.6 of the light through it **[E]**.
  - Each cell shows its own share of dots, so the grain shows as speckle.

## 6. Persistence

- **PLAT version 2** is written when an etching tool has touched a plate.
  - The etching tools are among its uses, each with its own data after its points (a length and the bytes). A bite keeps its steps, each step's flat dose and diffusion layer, so a crop rebuilt from the vectors bites to the same depths.
  - A ground's use keeps its roller's length and points; a stopping-out's its brush, load and points (each point's force the hand's pressure). A crop rebuilt from the vectors lays them again at its own pitch.
  - After the tiles come the coats: the ground's thickness, the metal the acid took, the rosin layers (seed, melted, grains per size), the tray (its acid, strength as made, volume and the metal it holds), the bath the plate lies in if it does (its acid, strength, temperature, face, rocking, feathering, and when it went in, on the plate's clock), the etched cells' states, and the cells the ground lies on (a bit a cell; none without a ground).
- **A plate no etching tool has touched** is still version 1, byte for byte, and version 1 is still read.

## 7. Estimates still standing

| what | value | why |
|---|---|---|
| hard ground thickness | 4 µm | IP §2.1 [E] 2–8 µm |
| ground's flow pressure | 5 MPa | waxes and bitumens 1–10 MPa; unmeasured for a ground |
| etching needle force, speed | 0.3 N, 40 mm/s | plan §3.1; no measurement |
| stop-out thickness | 20 µm | a brushed varnish, unmeasured |
| stop-out viscosity | 1 Pa·s | "thicker consistency"; unmeasured (its edge's creep goes as η^(−1/7)) |
| a sable's belly filled | 0.6 | the hairs' packing; no gravimetric measurement |
| roller speed, warming the plate | 100 mm/s, 8 min | no timed account |
| bend carried round smoothly | 30° | a hand follows a curve given by points; it pivots at a corner |
| tray: depth, margin | 20 mm, 50 mm | unmeasured |
| rocking: tilt, period | 5°, 4 s | unmeasured |
| bubbles: contact angle, cover unswept, feather period | 40°, 0.5, 30 s | unmeasured |
| nitric's etch factor | 1.0 | IP §3.4's 1–2, its low end |
| Dutch mordant: rate, activation energy | 0.5 µm/min, 50 kJ/mol | from a [P] over-etch time; textbook range |
| dust cloud: median, spread, loading | 30 µm, 2, 82 g/m³ | Unknown 7; loading from the [P] 50 % cover |
| rosin spread on melting | 1.3 | IP §4.1 [E] 1.2–1.5 |
| bitumen absorption and Urbach slope | 0.1 /µm at 550 nm, 0.3 eV | the "strong tea" test; no spectrum of a ground |
| rosin absorption, loose powder's reflectance | 0.02 /µm at 450 nm, 0.6 | unmeasured |
| yarn radius (for the tone guide) | 75 µm | IP §4.2 [E] |
| hand times: bath, box, melt | 2, 1, 2 min | no timed account |
| solvent rag: cloth weight, saturation share, a solvent rag's pace | 150 g/m², 0.2, a tarlatan pass's | §8 |
| the plate's polish under a thinned coat; a pinhole's radius; the least bared share that bites | Ra 0.1 µm, 2.5 µm, 10⁻⁴ | §8 |
| cross-links taken as the cure (the sol of set ink) | — | §9 |
| the caustic stripper's front, its rinse, a tray as strong as the paste | 50 µm/h, all it loosened | §9 |

Each would be replaced by IP §9's bench measurements (Unknowns 4, 6 and 7) without rewriting the code. The tests read their expected numbers from the formulas.

## 8. The solvent rag: cleaning a plate where the hand rubs [M] [D] [E]

A printer takes a ground, stopping-out varnish, rosin or ink off with a cotton rag dampened with
solvent (turpentine; petroleum spirit), rubbed over the plate by hand. Nothing dissolves where the
rag doesn't go, so `k:clean` and `k:clean_ink` take the points the rag is moved along, as the
tarlatan does, and act through the rag's own footprint: a pad (`rag::PAD_MM`, 40 mm, or `width=`)
slid along them (`Plate::slide`; under the contact laws pressed soft, `slide_pressed`), each cell
rubbed for its share of a pass.

**What the face carries.** A damp rag's face lays its solvent under it: the cloth's weight,
150 g/m² [E: cotton sheeting and jersey rags weigh 100–200 g/m²], times what a cloth wrung damp
holds, 0.5 g a gram [M: cotton's water retention value, 0.45–0.55 g/g, ISO 23714; a hand-wrung
cloth holds as much or more], over turpentine's 0.86 g/cm³ [M]: some 87 µm of solvent per area.

**Coats.** The solvent dissolves a coat until it is saturated, 0.2 of its volume [E]: asphaltum
and rosin dissolve freely in turpentine (stopping-out varnish *is* asphaltum in turpentine, and
rosin the residue of turpentine's distillation [M, CAMEO]), but beeswax only partly at room
temperature (two thirds of a beeswax sample in C9–C15 alkanes, a petroleum spirit's cut [M:
Kastratović 2022]), and a hard ground is held by its wax. So a whole pass dissolves some 17 µm of
coat [D]: a 4 µm ground comes off in one pass under the pad's middle (out to r·√(1 − (4/17)²) of a
40 mm pad's radius on a straight stroke), a 20 µm varnish in two, a melted rosin grain's lens
((2/3)·d/1.3², the look's) in one or two; loose rosin is swept off by any touch. The coats are
taken from the top: varnish first, then what lies under it. The rag carries off what it dissolves,
so a coat it doesn't dissolve through is left thinner by that much, and the next pass starts on
what is left: one pass under the middle of a flat pad leaves a 20 µm varnish 2.6 µm thick, and a
second takes the rest. (Under the contact laws a pressed pad rubs the point under its middle
3π/8 ≈ 1.18 of a pass, 20.5 µm, enough for the varnish in one.) The plate keeps, per cell, what the
rag has taken of its varnish, of its ground and of its melted rosin. A melted grain is a lens
whose face the solvent takes evenly: a spherical cap whose sphere shrinks about its centre, so its
foot draws in to √(2ρ′h′ − h′²) (ρ′, h′ the sphere's radius and the cap's height less what was
taken) [D, geometry], and the grain is gone once its height is taken. Loose rosin goes at a touch;
the rosin layer keeps the cells the rag took it off (its record's bit 1, then a bit a cell). The
solvent's residue (0.3 µm of solution at most a fifth coat, 0.06 µm) is left out [E: below the
pinholed thickness that follows, it resists nothing]. A cell left bare loses the acid's memory
there; what the acid opened stays as its openings.

**A thinned coat** still keeps the acid off while it is continuous. It is first breached over the
polished metal's summits: with the metal's heights Gaussian about their mean plane (σ =
Ra·√(π/2), Ra 0.1 µm, `inked::POLISH_RA_UM` [E]) and the coat's face levelled by the solvent that
thinned it, a coat t µm over the mean plane bares the share Φ(−t/σ) of the face [D]: 1 µm bares
under 10⁻¹⁵, 0.5 µm 3·10⁻⁵, 0.2 µm 5 %, 0.1 µm 21 %. A cell bared over less than 10⁻⁴ holds [E: a
ten-thousandth of the face bitten shows in no print]. Where it is bared more, each breach is a
pinhole some 5 µm across (radius 2.5 µm [E: between a fine polish's scratches, a µm or two wide,
and the 10 µm undulations the palm's skin conforms to]), the pinholes a Poisson field of number n
with 1 − e^(−πnr₀²) the bared share, and the acid bites a pit under each, widening under the coat
as it deepens as it does under a ground: its radius at depth y r₀ + √(z² − y²)/K, the pits a
Boolean model of discs, the open share at depth y 1 − e^(−πn·r(y)²), their volume its integral and
their width the union's mean chord (1 − c)/(2·n·r·c) (c the metal left at the face) [D]. The pit's
mouth is the coat's thickness, its slot share by the pit's width (§3). This is foul-biting: a
speckle of pits where the ground was rubbed too thin, merging into an open bite as they deepen.
Needle lines under a varnish bite only once the varnish is gone (over a window the varnish lies as
deep as the ground besides, so it is the last to pinhole). The model keeps one face bite per cell:
a cell bitten through pinholes and then cleaned bare bites on as a bare face.

**Saves.** The per-cell thinning goes in the plate's section under the laws word's bit 1
(`PLAT` version 4: bit 0 the contact laws, bit 1 a plate with a coat thinned or a stripper on its
ink, §9): each etched cell then carries three more words (what the rag took of its varnish,
ground and rosin) and each ink cell what the stripper has loosened and whether it lies there. A
plate with neither is written byte for byte as before; a save without the bit reads with whole
coats and no stripper.

**Ink.** On the flat the rubbing mixes the solvent with the film f it meets, and the cloth drinks
the mixture but for what a wiped smooth face keeps, 0.3 µm [M: a cellulose wiper over a wetted
surface leaves 0.1–0.5 g/m² behind, US 8,778,086 and 9,345,376]: a whole pass keeps
0.3/(f + 87) of the film, the power m of it for a share m [D]. A 10 µm film is down to 0.03 µm
after one pass under the middle and gone after two. In an opening the solvent fills the space
above the ink and joins it to the cloth, whose pores draw the ink up by capillarity, as
retroussage's fibres do: Lucas–Washburn at the ink's own viscosity at the plate's warmth, for the
time the pad lies over the point (its share of the pad's width over the speed) [D]. So a stiff
ink stays deep in a line after a pass that empties a loose one (an 80 µm line of a 28 Pa·s ink
keeps all but 34 µm of its depth for 0.2 s; a 0.3 Pa·s ink is drawn 335 µm), and a warm plate
gives its ink up sooner. Ink thinner than an oil's monolayer, 2 nm [M: Langmuir films of
triglycerides, 1.5–2.5 nm], is none: a plate rubbed over twice or three times is clean, and
the plate's tools take it again. The rag keeps what it takes and lays nothing back. Ink past
its gel point is taken only as far as it is sol (§9).

**Hand time** is the area rubbed (each pass counted) at a tarlatan pass's pace [E].

## 9. Set ink and the caustic stripper [M] [D] [E]

Ink left on a plate dries as an oil film (`drying`, at engine 3's pace): past its gel point
(cure `GEL`) it is a cross-linked network. A solvent swells such a network but cannot dissolve it;
it can only take out the sol, the molecules not yet joined to it [M: Phenix 2002, *JAIC* 41,
61–90: turpentine and white spirit are low to low-moderate swellers of linseed paint films, still
short of equilibrium after 100 minutes on a 130–140 µm film; Baij et al. 2020, *Heritage Science*
8:43, a review of solvent action on oil paint: leaching takes the low-molecular-weight fraction,
the network stays].

**How much is sol.** Past the gel point the sol falls as the Charlesby–Pinner law has it for
chains joined at random, s + √s = 2·q_g/q, q the cross-links and q_g those at the gel point [M:
A. Charlesby, S. H. Pinner, *Proc. R. Soc. A* 249, 367 (1959)], with the cross-links taken as the
cure (the oxygen uptake `drying` follows is what cross-links the oil) [E]. A film just past its gel
point is nearly all sol; at cure 0.3 it is 38 %, at 0.5 18 %, touch-dry (cure 1) 6 % [D]. Brief
contact with acetone, ethanol or dichloroethane takes 3–4 % by weight from linseed films seven
months old [M: as reported in Phenix & Sutherland 2001, "The cleaning of paintings: effects of
organic solvents on oil paint films", *Reviews in Conservation* 2]; long immersion takes 15–45 %
[M: Stolow; Baij et al. 2020, Fig. 7], which includes the products of ageing's chain scission,
which `drying` doesn't follow.

**The rag on set ink** (`Plate::clean_ink`). As a pass comes to a cell, what it can't take is fixed:
the network, (1 − s) of the ink below any face a stripper has loosened, on the flat and in each
opening. On the flat everything above the network is kept by RESIDUE/(f + SOLVENT) a whole pass as
wet ink is (§8); in an opening the cloth's capillary draw takes the loosened face and the sol and
stops at the network. The ink left holds less sol, so its cure rises to that of a film holding the
sol that is left (s inverted: q = 2q_g/(s + √s)), and the next pass takes next to nothing. A
touch-dry plate keeps some 94 % of its ink under any number of passes. That the sol
mixes with the rag's solvent within a pass, as wet ink does, is a simplification [E: inside the
network it must diffuse out; turpentine goes into a linseed film over tens of minutes at
100 µm (Phenix), 0.3 µm in the 0.4 s the pad lies over a point; the measured brief take, 3–4 %,
is of the order of the whole sol of a dry film].

**The stripper.** A printer takes set ink off with a caustic stripper: sodium hydroxide in water,
laid on as a paste or the plate laid in a tray of it. It hydrolyses the oil's ester links
(saponification): what it has passed through is soap of the oil's fatty acids and loose fragments
of the network, which water rinses off and a rag wipes as a liquid [M: caustic strippers break the
paint film down chemically by hydrolysing its esters, where solvent strippers swell it]. Its front
goes in from the face of each deposit (the film, and the ink in each opening) at a steady 50 µm an
hour [E: Peel Away 1, a caustic paste of pH 12, takes up to 30 coats of oil, alkyd or lead paint in
1–24 h at 10–32 °C (24–48 h at 2–9 °C), by its data sheet; 30 coats of a house paint's 40 µm dry
film (1–2 mil a coat) in a day. A steady front, not one slowing as √t, as alkali takes an ester
film by eroding its face]. So a 15 µm film is loosened in 18 minutes and the ink in an 80 µm line
in an hour and a half. Copper is untouched by it [M: copper resists sodium hydroxide at room
temperature]. The front goes in on the plate's clock while the stripper lies there (`wait` is the
dwell), until a rinse under water, the whole plate's, washes off the stripper and all it has
loosened [E: soap dissolves in water and the loose network goes with it], or a solvent rag wipes the
stripper off where it goes, taking the loosened ink as wet ink. A tray of the stripper works as the
paste does [E: the paste's strength; a stronger lye would go faster]. Temperature is left out
[E: the data sheet's cold times are about twice its warm ones].

## Sources

- CRC Handbook of Chemistry and Physics, "Concentrative properties of aqueous solutions: density, refractive index, freezing point depression, and viscosity" (from A. V. Wolf, *Aqueous Solutions and Body Fluids*, 1966; O. Söhnel, P. Novotný, *Densities of Aqueous Solutions of Inorganic Substances*, 1985): FeCl₃, HNO₃, HCl rows at 20 °C. Read from the copy at https://chemistry.mdma.ch/hiveboard/rhodium/pdf/chemical-data/prop_aq.pdf
- CRC Handbook, "Ionic conductivity and diffusion at infinite dilution": D of Fe³⁺, Cl⁻, H⁺, NO₃⁻, ClO₃⁻.
- Environment Canada, *Ferric chloride: environmental and technical information for problem spills* (1984), viscosity of 40 % FeCl₃: https://publications.gc.ca/collections/collection_2019/eccc/En48-10-21-1984-eng.pdf ; MG Chemicals 415 datasheet (12 cP, 100 g/L capacity): https://www.farnell.com/datasheets/2867387.pdf
- "Application of ferric chloride both as oxidant and complexant to enhance the dissolution of metallic copper", *Hydrometallurgy* (2010) (Ea 24.9 kJ/mol), and the McGill thesis and *Trans. IMF* 44 (1966) on the optimum: as cited in IP §3.1.
- J. R. Lloyd, W. R. Moran, "Natural convection adjacent to horizontal surface of various planforms", *J. Heat Transfer* 96, 443 (1974); S. W. Churchill, H. H. S. Chu, "Correlating equations for laminar and turbulent free convection from a vertical plate", *Int. J. Heat Mass Transfer* 18, 1323 (1975); S. W. Churchill, "A comprehensive correlating equation for laminar, assisting, forced and free convection", *AIChE J.* 23, 10 (1977); F. P. Incropera et al., *Fundamentals of Heat and Mass Transfer* (the flat-plate and horizontal-plate correlations, used by the heat–mass analogy).
- W. Fritz, "Berechnung des Maximalvolumens von Dampfblasen", *Physikalische Zeitschrift* 36, 379 (1935): bubble departure diameter.
- Vogel's equation for water's viscosity (A = 2.414 × 10⁻⁵ Pa·s, B = 247.8 K, C = 140 K): the standard fit, e.g. in Reid, Prausnitz & Poling, *The Properties of Gases and Liquids*.
- 1911 Encyclopædia Britannica, "Etching": Dutch mordant's recipe, its slowness and lack of ebullition, its lines' widening against nitric's, smoking a ground: https://en.wikisource.org/wiki/1911_Encyclop%C3%A6dia_Britannica/Etching
- Asphalt binder optical constants by spectroscopic ellipsometry (n ≈ 1.6): "Spectroscopic Ellipsometry of Asphalt Binder: A Study of Optical Constants" (via https://www.researchgate.net/publication/336407885); asphaltene Urbach edges: I. N. Evdokimov et al., "On the Nature of UV/Vis Absorption Spectra of Asphaltenes" (via https://www.researchgate.net/publication/245332232), after O. C. Mullins (abstracts only).
- Rosin's index and density: CAMEO, https://cameo.mfa.org/wiki/Rosin
- The ground roller: *Etching: a guide to traditional techniques*, ch. 11 "Hard ground etching" (https://zoboko.com/text/dnxwnxol/etching-a-guide-to-traditional-techniques/11); Artools leather rollers at Polymetaal (https://www.polymetaal.nl/contents/en-uk/d517_Pagina_525.html).
- Stop-out drying: Glasgow Print Studio, *Etching Handbook* (https://www.gpsart.co.uk/DownloadDocs/GPS_Etching_Handbook.pdf); MungoLux asphaltum varnish (https://mungolux.com/asphaltum-hard-ground_2).
- Cox–Voinov and Tanner's law, ln(L/λ): D. Bonn, J. Eggers, J. Indekeu, J. Meunier, E. Rolley, "Wetting and spreading", *Rev. Mod. Phys.* 81, 739 (2009) (https://www.phys.ens.psl.eu/~foldingslidingstretchinglab/papers/Bonn_RMP_2009.pdf).
- Sable rounds' hair lengths and widths: Rosemary & Co Series 33 (as in wet_pastel.md).
- V. Kastratović, "Some aspects of beeswax hydrolysis", *Agriculture and Forestry* 68(4) (2022): beeswax's solubility in solvents at room temperature: http://www.agricultforest.ac.me/data/20221215-07%20Kastratovic.pdf
- Cotton's water retention value (ISO 23714; 0.45–0.55 g/g for cotton): textile fibre handbooks.
- US patents 8,778,086 and 9,345,376, "Method of cleaning residue from a surface using a high efficiency disposable cellulosic wiper": residue left by a wiper, 0.1–0.5 g/m².
- The workshop facts (grounds, times, coverage, over-biting): IP §2–§4 and its sources (Kiekeben; Jackson's; sonic.net; UMBC).
- A. Phenix, "The swelling of artists' paints in organic solvents. Part 2: comparative swelling powers of selected organic solvents and solvent mixtures", *JAIC* 41(1), 61–90 (2002): https://cool.culturalheritage.org/jaic/articles/jaic41-01-005.html
- L. Baij, J. Hermans, B. Ormsby, P. Noble, P. Iedema, K. Keune, "A review of solvent action on oil paint", *Heritage Science* 8:43 (2020): https://doi.org/10.1186/s40494-020-00388-x (leaching and its amounts, Stolow's data, swelling and diffusion).
- A. Phenix, K. Sutherland, "The cleaning of paintings: effects of organic solvents on oil paint films", *Reviews in Conservation* 2 (2001) (brief exposure takes 3–4 % by weight from seven-month-old linseed films), as cited in Baij et al.
- A. Charlesby, S. H. Pinner, "Analysis of the solubility behaviour of irradiated polyethylene and other polymers", *Proc. R. Soc. Lond. A* 249, 367–386 (1959): the sol fraction of chains cross-linked at random.
- Dumond, Peel Away 1 Paint Remover technical data sheet (pH about 12; up to 30 layers; 1–24 h, 24–48 h in the cold): https://www.arcat.com/datasheets/dumond/TDS_-_Peel_Away_1_Paint_Remover_-_UPDATED_2023_04.pdf
- Paint stripper (caustic strippers hydrolyse the film's esters; solvent strippers swell it): https://en.wikipedia.org/wiki/Paint_stripper
