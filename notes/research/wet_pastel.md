# Pastel with water

The numbers behind crates/paint/src/wet_pastel.rs (the wet pastel switch, `--@ wet-pastel`). Tags: [M] measured, [D] derived here from measured values or standard physics (the arithmetic shown, or in `dry_pigment_mie.py media`), [P] a practitioner's, museum's or historian's account, [E] an estimate, with the reason it is one. I found no measurement of what water does to pastel itself, optically or mechanically, so every constant is derived from the physics of its parts: Mie scattering against the pore phase, the stick's gum and porosity, capillarity, and the psychrometrics of a still room.

## 1. What a stick is made of: gum, particles, pores

- A pastel stick is powdered pigment with a filler (chalk or whiting, kaolin, talc, sometimes gypsum) and a weak water-soluble binder: gum tragacanth, gum arabic or, from the 20th century, methyl cellulose [M] (notes/research/pastel_stick_tribology.md §1, after WebExhibits and Wikipedia "Pastel").
- **Gum, `GUM_SHARE` = 1.5 % of the stick's mass [D].** Recipes dough a gram of powder with 0.3–0.5 mL of a 1:30 (3.3 %) tragacanth solution [M], so the dry stick holds 1–2 % gum (pastel_stick_tribology.md §1). WebExhibits gives the 1:30 solution; makers' forum recipes run more dilute and some colours take none, so soft sticks may hold less (a fraction of a percent) [P]. 1.5 % is the middle of the derived range.
- **Porosity, `POROSITY` = 0.6 [D].** A Sennelier stick (≈ 13 × 64 mm, 8.5 cm³) weighs about 9 g: ρ ≈ 1.05 g/cm³ against solids of ≈ 2.65 (calcite 2.71, kaolinite 2.6 [M]) gives 1 − 1.05/2.65 ≈ 0.6 (pastel_stick_tribology.md §1). The engine counts a deposit in the stick's volume it wore off (`Cell::loose`), so the deposit's pores are the stick's: the same 0.6 serves the tip and the laid pastel (the PR's first version had 0.4 and 0.45, both estimates).
- **The gum's share of the pores [D]:** gum mass over the solids', times the density ratio, over the pores: 0.015/0.985 × 0.4 × 2.65/(1.4 × 0.6) ≈ **0.019** (dry gum arabic 1.35–1.49 g/cm³ [M]).
- **Particle size, `PARTICLE_D_UM` = 3 µm [E].** The measured constituents run 0.2–20 µm (kaolin 0.2–5, whiting 1–20, earths 1–30; pastel media 1–20 µm measured by vibration studies, pastel_stick_tribology.md §3). No size distribution of a whole stick was found, so 3 µm stands for the typical primary particle. It sets the pores' radius (2 r_h = 2 ε d / 6(1 − ε) ≈ 1.5 µm), the wet tip's capillary strength and the paste's coverage, all below.
- **The fillers.** The engine's sticks are made of their pile's tubes only (`dry.rs` gives their dry colour from the tubes), so it has no filler to apply. Their in-water scattering is tabled below for when it has: calcite (birefringence-averaged) keeps 0.33 of its dry scattering, kaolin 0.24, gypsum 0.18.

## 2. Optics: why wet pastel is darker, and how much

- Dry pastel looks pale because its particles scatter strongly against air (dry_pigment_optics.md). Water (n = 1.333) in the pores lowers the contrast. Mie scattering depends on the relative index m = n_p/n_med and on the size parameter π d n_med/λ, both of which move with the medium.
- **`dry::scatter_in(n, n_med)` [D]:** the transport scattering σ_s(1 − g) per particle volume in a medium n_med, over the same in air, by the repository's own Mie code at the `mie_air` setup (d = 1 µm, lognormal σ_ln 0.5, λ 550 nm): `uv run notes/research/dry_pigment_mie.py media`. Full water:

  | particle n | 1.50 (ultramarine) | 1.56 (Prussian blue) | 1.65 (bone black) | 1.74 (cobalt) | 2.0 (lead white) | 2.4 (chrome yellow) | 3.0 (vermilion) |
  |---|---|---|---|---|---|---|---|
  | S in water / S in air | 0.14 | 0.24 | 0.43 | 0.56 | 0.69 | 0.70 | 0.84 |

  So the darkening is the pigment's own: a stick of ultramarine goes nearly as deep as in oil, vermilion hardly changes. The previous single value, `WATER_S` = 0.3 [E], is gone.
- **Each cell's particle index (`Cell::index`) [D]:** a stick's is its pile's tubes' mean refractive index by volume (`Palette::dry_index`, the indices of `dry::refractive_index`). It is averaged by volume, not by scattering, because dry every particle scatters within a factor of about 1.5 of any other per volume (dry_pigment_optics.md §2.2). A cell's index follows the pastel laid in it by volume (stick, crumbs, paste). The finger, the dry brush and the air do not carry it: pastel they move keeps the index of the cell it lands in (an approximation; it matters only where sticks of very different indices are blended and then wetted).
- **Partly filled pores [D].** Water fills the pastel's fine pores first (§4). The pore phase is taken as a Bruggeman effective medium of air and water by the share filled f, n_med = √ε_eff, then Mie at that index: half filled gives n_med = 1.16, so lead white keeps 0.82 of its scattering, ultramarine 0.59. (An effective medium assumes the air and water mix on a scale below the wavelength; menisci between 3 µm particles are near it, so this is the standard first approximation, not a measured curve.) The PR's first version made the darkening linear in f.
- **Dried again [D].** What is left in the pores is the gum, dry (n 1.52, gum arabic films 1.51–1.54 [M], dry_pigment_optics.md §1), at its share of the pores (0.019 × the share of the gum the pastel kept, §3). Bruggeman gives n_med ≈ 1.009, and Mie leaves 0.98–1.0 of the dry scattering: dried pastel is **within a few percent of its dry colour** (0.3 % darker in the engine's test band). The earlier `DRIED_S` = 0.8 [E] has no physical basis: the gum is too little to fill the pores. The saturated look practitioners describe is in the paste spreading into the tooth (more coverage for its volume, §3), not in its scattering.
- **Compaction, not modelled [E].** A drying paste is pulled together by its menisci and may pack closer than the stick's 0.4 solids. How much, and what dependent scattering then does at d ≈ λ, was not found measured (the Percus–Yevick factor applies only to d ≪ λ), so it is left at none.
- Paper itself darkens when wet. That is not modelled: only the pastel darkens.

## 3. Mechanics: the gum, the wet tip, the paste

- **Binding by the gum it kept [D].** Water dissolves the gum: in the stick it sits as bridges at the particles' contacts, and as the water leaves it sets there again. Gum dissolved in water that sinks into the sheet (§4) goes with it, in proportion: a cell's gum share becomes (1 − sunk/water) of what it held (`Cell::gum`). Evaporating water leaves its gum. A stick holds together by all of its gum, so a dried passage that keeps all of it is bound as a stick is, and one that kept a share g has a share g of its pastel bound (`Cell::gum_set`).
  - Repeated wetting redissolves and resets the same gum (gum arabic is water-soluble once dry, as watercolour rewets [M]): it can never bind more than the gum it holds, and what sank never comes back. What is bound already, by fixative or an earlier wetting, stays bound (the engine cannot tell gum-bound from fixative-bound pastel, so a gum share lost later does not loosen it again).
  - New pastel laid over brings its full gum (the share mixes by volume).
  - The earlier `GUM_BIND` = 0.5 of the loose pastel per wetting [E], without limit, is gone. On a sized sheet a passage now dries fully bound; on the default absorbent sheet a soaked one (40 µm of water over about 6 µm of pore water) keeps about 0.15 of its gum and binds that much.
- **A soaked tip, `wet_hardness` [D].** Its gum bridges dissolved, a saturated compact holds together by capillary pressure alone: tensile strength σ ≈ 6 (1 − ε)/ε · γ/d (Schubert's capillary state, cos θ ≈ 1 on chalk and clay) = 6 × 0.67 × 0.0728 N/m / 3 µm ≈ 0.097 MPa. Its indentation hardness is taken as about 3 σ (Tabor's constraint factor) ≈ **0.29 MPa**, never harder than dry. A soft stick (soft 0.75) is about 3.9 MPa dry (pastel.rs `mechanics`): the soaked tip is ~13× softer. Between dry and soaked the hardness follows the tip's saturation linearly (Rumpf: the capillary strength scales with saturation) [E on the line's shape].
  - The Archard wear coefficient K is kept as dry [E]: no measurement of K for a wetted compact was found, and Archard's V = K F s/H already raises the wear through the hardness. The earlier `WET_SOFT` 0.4 and `WET_WEAR` 2.5 are gone.
  - In the engine's test the wet stroke lays 3.5× the dry stroke's volume at 2 N, limited by the tooth it can reach.
  - The factor 3 and the size d are the estimates here. No strength of a wetted gum-bound chalk compact was found. (Pure gum arabic films are weak, 1–2.5 MPa tensile [M]; chalk rock loses strength when saturated [M].)
- **Fewer crumbs [E].** The crumbs (`SHED` = 1 − 1/6.6, measured for graphite) come from brittle fracture. A saturated compact held by capillary pressure deforms plastically, as a soil above its plastic limit does, and smears where it touches. So the shed share falls to 0 at saturation, linearly with it. The line is an estimate: no measurement of a wet pastel's debris was found. This replaces `WET_SHED` = 0.6.
- **The paste spreads, `paste_thin` [D].** Dry, the deposit covers as particles 6 µm thick (`PARTICLE_UM`, crumbs and fragments). Wet, the water disperses it to primary particles: d = 3 µm spheres cover 1.5/d of area per volume of solids, and a stick's volume holds 1 − ε of solids, so a paste covers as particles d/(1.5(1 − ε)) = 5 µm thick: **0.83** of the dry thickness (was `PASTE_THIN` 0.5 [E]).
- **The water it lays [D].** A saturated paste carries its pores' water: ε = 0.6 of its volume, times the tip's saturation. (The first version had ε/(1 − ε), a volume of water per volume of solids applied to stick volume.)
- **The tip's water [D].** A dip soaks `DIP_MM` = 1 mm of the end [E: how deep the hand dips is a gesture, not physics; `p:wet(share)` sets the share of that]. The water then spreads up the dry stick by Lucas–Washburn, l² = l₀² + D t with D = r γ/(2η) ≈ 54 mm²/s (r ≈ 1.5 µm, §1), and the tip keeps l₀/l of it. It also evaporates from the end and the wet side at the rate of §4. So a dip serves about a stroke: after one second the tip holds 0.13 of its water. The gum solution is taken to flow as water does, which holds for gum arabic at the ≈ 3 % the pores' water dissolves. A tragacanth stick would gel its pores and hold its water far longer; that is not modelled. This replaces `TIP_DRY_MIN` = 2 min [E].
- **"A dampened stick can be drawn over earlier layers without lifting them"** [P] (notes/degas-materials.md §5, N-A 618). This agrees with a softer stick that lays more and sheds less.

## 4. Water on the paper: where it goes, how fast it goes

- **Which pores hold the water [D].** Capillary suction is 2γ cos θ/r. The pastel's pores (r ≈ 1.5 µm, chalk and clay wetted, cos θ ≈ 1) pull about 5× harder than a sized drawing paper's (r ≈ 6 µm, cos θ ≈ 0.75). So the pastel's pores fill first and empty last, and only water standing over them reaches the sheet.
- **Sinking: Lucas–Washburn [D].** Depth l = √(r γ cos θ t / 2η), with:
  - r the paper's own mean pore height t_f ε/(1 − ε) (paper.rs; 6 µm for the default sheet);
  - water at 20 °C, γ = 72.8 mN/m and η = 1.002 mPa·s [M, handbook];
  - θ from the sizing, as a Cassie mixture of sized and bare fiber by `absorbent` (now the unsized share of the fiber surface): cos θ = a cos 0° + (1 − a) cos 105°. A good AKD or rosin sizing of writing grades gives about 105° [M] (IPPTA 2007). Unsized paper takes water in before an angle can be read, so θ ≈ 0° [M] (*Colloids Surf. A* 2000, sized and unsized paper).
  - The sheet draws in its porosity times that depth. At the default `absorbent` 0.8, θ ≈ 42°, and about 6 mm of water goes in within a second: whatever stands over the pastel sinks at once. Below `absorbent` ≈ 0.21 the angle passes 90° and nothing sinks; water stays and evaporates, the hard-sized sheet of practice.
  - Real sized papers take water in more slowly than this, over seconds to a minute (Cobb tests), because the size itself wets slowly. That kinetics is not modelled: on the engine's time steps of minutes the difference does not show.
  - The PR's first version had `SINK_MIN` = 1.5 min / absorbent [E].
- **What sank is the sheet's.** It spreads through the paper away from the pastel and no longer darkens it. The sheet's own wetness, its darkening, and water spreading sideways are not modelled.
- **Evaporation: psychrometrics [D], `evaporation_um_min`.**
  - The flux is h_m (ρ_v,sat(T_s) − RH · ρ_v,sat(T_a)).
  - h_m comes from ISO 6946's convective coefficient for a vertical surface in still room air, h_c = 2.5 W/m²K [M], by the Chilton–Colburn analogy h_m = h_c/(ρ c_p) · Le^(−2/3). With D_v = 2.48 × 10⁻⁵ m²/s at 20 °C (Massman 1998) [M], Le = 0.87, so h_m = 2.27 mm/s.
  - The saturated vapour pressure follows Buck (1981) [M].
  - The wet surface cools until the latent heat it loses equals what convection and the room's radiation bring it (4εσT³ = 5.4 W/m²K, ε 0.95). It settles near 16.3 °C.
  - The result is **0.71 µm of water a minute** (about 1 mm a day; an isothermal surface would give 1.2). That sits inside the 0.8–3 µm/min range of the indoor pool correlations (§ Sources) at its still-air end. It replaces `EVAP_UM_MIN` = 2.5 [E].
- **The studio [E].** The engine has no room conditions (sheet.rs assumes 20 °C air), so `STUDIO_C` = 20 °C and `STUDIO_RH` = 50 %. 50 % is the conventional middle of the 40–60 % held for works on paper; a real studio's humidity is not recorded in any painting.
- **Falling rate [D].** Once the standing water is gone, the evaporation front recedes into the pastel's pores. The vapour then also crosses the dry pores above it, with diffusivity D_v ε^1.5 (Bruggeman's tortuosity), so the flux becomes E/(1 + a u), where u is the water missing from the pores and a = h_m/(ε D_eff) ≈ 3.3 × 10⁻⁴ per µm.
  - The pastel layer is thin (tens of µm), so this slows its last water by well under a percent. In still air the boundary layer, not the pores, limits drying.
  - Handprint's slow "damp" stage (watercolor_physics.md §2) is water deep in the sheet, which the engine does not track.
- **What this gives.** A wet stick's line (a few µm of water) dries in minutes. A brushed wash of about 25–40 µm on a hard-sized sheet takes about an hour. On the default absorbent sheet the water over the pastel goes at once and only the pores' water dries. Handprint's 20 minutes for "a generous wash" [P] counts a sheet that takes most of its water into itself and spreads it sideways, which the engine does not follow.

## 5. The brush: how much it holds and lays

- **Capacity, `Held::capacity_mm3` [D from the engine].** Water fills the space between the hairs that a full load of paint takes: every bristle's full load (`Held::full`, the tool's lay × track × run over its hair overlap), in mm³. A 10 mm round sable holds 0.14 mL. This replaces `BRUSH_WATER_MM3` = 0.9 mm³/mm² [E]; `spatter`'s own `load_mm3` says 0.08 mL for a 10 mm brush, the same order. The brush model's own numbers are estimates of that model, not new ones.
- **Paste, `amount=` [D].** A paste at the stick's own packing fills the space between the hairs, so at most `capacity` of stick volume. Its solids (1 − ε of it) push out the water, and the water is trimmed to capacity − (1 − ε) × paste. `amount` is the share of that most. This replaces `BRUSH_PASTE_SHARE` = 0.25 [E].
- **How it lays, picks up and carries [D from the engine].** The water a full load lays at the start of a stroke is the tool's `lay` (25 µm a coat for a sable; was `WASH_UM` = 30 [E]). The share of the wetted paste under it that it takes up in a pass is the tool's `pickup`, the share of wet paint a bristle takes up (0.1 for a sable; was `PICK_WET` 0.5 [E]). It lays what it holds within its `run`, the length over which a load runs down (was `CARRY_MM` 12 [E]). The wetted paste is wet paint to the brush, so its handling is its handling of paint.
- **Pressure [E].** A pressure of 0 to 1 lays 0.5 to 1.5 of a middling press's water. No measurement of a brush's water against its pressure was found.
- **Hand time [D from the engine].** A wash is priced as a brush stroke (the steering law, tally.rs `stroke_secs`; was `WASH_SPEED` 60 mm/s [E]). A dip is a reach to the jar, priced as a trip to the palette (`RELOAD`, 2.5 s). Grinding pastel into it is priced as mixing a pile (`REMIX`, 20 s). The PR's first version had 2 s and 10 s.

## 5a. The three gestures, as the engine has them

1. **A dipped stick:** `p:wet(share)`, then `p:stroke`. The deposit is wet: dark while wet by its pigments' index, bound once dry by the gum it kept.
2. **A wet brush through laid pastel:** `b:dip(share)`, then `b:wash(pts)`. The loose pastel it wets turns to a paste that moves along with the brush.
3. **Ground pastel with water:** `b:dip(share, {pastel=p, amount=})`, then `b:wash`. The brush holds a paste of the stick's colour and index; a little pastel in much water is a wash, much pastel in little water a paste. The stick shortens by what was ground off it.

## 5b. Memory

Each drawing cell grows from 80 to 88 bytes, for every painting with a drawing, wet or not:
- the water (f32);
- the particles' mean index (u16);
- the gum share (u16).

The two u16s share one 4-byte slot, so the index and the gum cost no more than the water alone did in the first version's alignment. I considered a separate, lazily allocated buffer:
- **Repaints.** The index and the water are needed by `Cell::reflectance`, which every tool calls at about 30 repaint sites across graphite.rs, pastel.rs and sheet.rs. A side buffer means a lookup at each of them, or passing it through all of them.
- **Where the index comes from.** The index must be recorded when dry pastel is first laid, before any water, so the buffer could not wait for the first wetting.

So the fields stay in the cell.

## 6. Historic accounts

- **Degas, one c. 1882 pastel** [P] (notes/degas-materials.md §5, after the Getty). All three gestures appear in it:
  - the sharpened tip dipped in water and drawn with, giving "a creamy, saturated line"
  - wet sticks applied with a brush
  - pastel ground and mixed with water into a paste
- **Degas, other accounts:**
  - Musée d'Orsay: he used pastel "dry or mixed with water, crushed or worked with steam" [P] (musee-orsay.fr, "Degas the pastel artist").
  - Thyssen-Bornemisza technical study of *At the Milliner's*: he sometimes crushed sticks and mixed them with water "to create a watercolour effect or achieve a pasty texture" [P] (museothyssen.org).
  - Princeton: in *Dancers* he applied pastel "both wet and dry" [P] (artmuseum.princeton.edu).
  - Kimberly Schenck (NGA, 2023) found crushed pastel mixed with a liquid in the bodices of *Two Women*, the exact application left open [P] (as cited by thehistoryofart.org; not read directly).
- **Steam** dissolved a layer into a film, or melted it into a paste reworked with a brush or knife as it dried [P] (Reff, quoted second-hand; Artist's Network). Not modelled: research gap 6.
- **Cassatt.** Areas of finished pastels are "painted in pastel chalk mixed with water, a technique learned from Degas" [P] (PMA 2024, via Weiss). The c. 1879 canvas pastel was built using "moisture to create a multilayered surface" [P] (PMA, *In the Loge*). How she applied the water is not recorded (cassatt_materials.md §2, on branch notes/cassatt-materials).

## 7. What is left out

- Water flowing sideways (backruns, blooms), and the edge darkening of a drying wash.
- The paper's own darkening when wet, and cockling.
- Steam and moisture vapour (gap 6).
- A finger in wet paste: it moves the loose pastel as it does dry pastel.
- The sheet's own water after it sinks, its darkening, and the slow drying from inside it.
- The size's wetting kinetics (Cobb uptake over seconds), compaction of a drying paste, and a tragacanth stick's gelled pores.
- The finger, the dry brush and the air carrying a cell's particle index with the pastel they move.

## Sources

- Musée d'Orsay, "Degas the pastel artist." https://www.musee-orsay.fr/en/articles/degas-pastel-artist-219377
- Museo Nacional Thyssen-Bornemisza, "Edgar Degas. At the Milliner's. Technical study of a pastel." https://www.museothyssen.org/en/conservation/edgar-degas-milliners-technical-study-pastel
- Princeton University Art Museum, Degas, *Dancers*. https://artmuseum.princeton.edu/art/collections/objects/9449
- Art Institute of Chicago, Degas pastels, AIC Book and Paper Group Annual 5. https://cool.culturalheritage.org/coolaic/sg/bpg/annual/v05/bpga05-07.pdf
- Artist's Network, "Pastel Pointers: a steamy situation." https://www.artistsnetwork.com/art-mediums/pastel/pastel-pointers-a-steamy-situation/
- thehistoryofart.org, "Edgar Degas Pastels" (citing Schenck 2023, Shelley 2020). https://www.thehistoryofart.org/edgar-degas/pastels/
- A. TenWolde, "The Effect of Indoor Humidity on Water Vapor Release in Homes," ORNL Buildings X, 2007. https://web.ornl.gov/sci/buildings/conf-archive/2007%20B10%20papers/071_TenWolde.pdf
- US patent 9,114,355 (evaporation correlations for still and moving air). https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/9114355
- In this repository: notes/research/pastel_stick_tribology.md, dry_pigment_optics.md, watercolor_physics.md, pastel_blending.md; notes/degas-materials.md §5; notes/research/cassatt_materials.md §2 (branch notes/cassatt-materials).

- ISO 6946:2017, Building components: thermal resistance, Annex C (convective surface coefficients: 2.5 W/m²K for horizontal heat flow).
- A. L. Buck, "New equations for computing vapor pressure and enhancement factor," *J. Appl. Meteorol.* 20 (1981) 1527–1532.
- W. J. Massman, "A review of the molecular diffusivities of H₂O, CO₂, …in air," *Atmos. Environ.* 32 (1998) 1111–1127.
- T. H. Chilton and A. P. Colburn, "Mass transfer (absorption) coefficients," *Ind. Eng. Chem.* 26 (1934) 1183–1187 (the heat and mass transfer analogy).
- D. A. G. Bruggeman, *Ann. Phys.* 416 (1935) 636–664 (the effective medium; the ε^1.5 tortuosity of porous media).
- R. Lucas, *Kolloid-Z.* 23 (1918) 15; E. W. Washburn, *Phys. Rev.* 17 (1921) 273.
- A. B. D. Cassie and S. Baxter, *Trans. Faraday Soc.* 40 (1944) 546 (the contact angle of a mixed surface).
- H. Schubert, "Tensile strength of agglomerates," *Powder Technol.* 11 (1975) 107–119; H. Rumpf (1958), the strength of granules held by liquid bridges.
- D. Tabor, *The Hardness of Metals* (1951) (hardness ≈ 3 × flow stress).
- IPPTA Journal 2007 no. 4, art. 15 (contact angle 105°, "fairly good for writing and printing paper grades", at Cobb 25–48 g/m²): https://ippta.co/wp-content/uploads/2021/01/2007_Issue_4_IPPTA_Articel_15.pdf
- "Contact angle measurement and surface energetics of sized and unsized paper," *Colloids Surf. A* (2000) (water too fast into unsized paper for an angle): https://www.sciencedirect.com/science/article/abs/pii/S0927775700004544
- M. A. Hubbe, "Paper's resistance to wetting," *BioResources* 2(1) (2007) 106–145 (good sizing > 90°): https://bioresources.cnr.ncsu.edu/BioRes_02/BioRes_02_1_106_145_Hubbe_Sizing_of_Paper_Review.pdf
- Gum arabic films, tensile strength 1.1–2.4 MPa unfilled (*Food Chem.* 2021, S030881462100203X; *J. Saudi Dent. Soc.* 2023, S1319610323001175).
- WebExhibits, "Pigments in pastels" (1:30 tragacanth): https://www.webexhibits.org/pigments/intro/pastel.html
