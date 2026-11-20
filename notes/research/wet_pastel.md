# Pastel with water

The numbers behind crates/paint/src/wet_pastel.rs (the wet pastel switch, `--@ wet-pastel`). Tags: [M] measured, [D] derived here from measured values or standard physics (the arithmetic shown, or in `dry_pigment_mie.py media`), [P] a practitioner's, museum's or historian's account, [E] an estimate, with the reason it is one. I found no measurement of what water does to pastel itself, optically or mechanically, so every constant is derived from the physics of its parts: Mie scattering against the pore phase, the stick's gum and porosity, capillarity, soil mechanics, the measured uptake of paper, and the psychrometrics of a still room. Section 8 lists what the estimates audit (notes/estimates-audit-2.md, W1–W23) changed and what is still an estimate.

## 1. What a stick is made of: gum, particles, pores

- A pastel stick is powdered pigment with a filler (chalk or whiting, kaolin, talc, sometimes gypsum) and a weak water-soluble binder: gum tragacanth, gum arabic or, from the 20th century, methyl cellulose [M] (notes/research/pastel_stick_tribology.md §1, after WebExhibits and Wikipedia "Pastel").
- **Gum, `GUM_SHARE` = 1.5 % of the stick's mass [D].** Recipes dough a gram of powder with 0.3–0.5 mL of a 1:30 (3.3 %) tragacanth solution [M], so the dry stick holds 1–2 % gum (pastel_stick_tribology.md §1). WebExhibits gives the 1:30 solution; makers' forum recipes run more dilute and some colours take none, so soft sticks may hold less (a fraction of a percent) [P]. 1.5 % is the middle of the derived range.
- **Porosity, `POROSITY` = 0.55 [D], bounds 0.50–0.61.** A Sennelier stick is listed at 9.07 g (retail listings for White 525 and Ivory Black 513, the same weight for several colours, so possibly with its wrapper). At its listed 2.65 × 0.45 in (6.7 × 1.14 cm, 6.9 cm³) it is ρ 1.32 g/cm³: 1 − 1.32/2.65 = 0.50 against solids of 2.65 (calcite 2.71, kaolinite 2.6 [M]), 0.54 against 2.9 (a stick with denser pigments). At the nominal ½ × 2½ in (8.0 cm³) it is 1.13 g/cm³, 0.57–0.61. The engine counts a deposit in the stick's volume it wore off (`Cell::loose`), so the deposit's pores are the stick's. (Was 0.6, from a rounder 8.5 cm³ and 9 g.)
- **The gum's share of the pores [D]:** gum mass over the solids', times the density ratio, over the pores: 0.015/0.985 × 0.45 × 2.65/(1.4 × 0.55) ≈ **0.024** (dry gum arabic 1.35–1.49 g/cm³ [M]).
- **Particle size, `PARTICLE_D_UM` = 1.7 µm [D].** The historic pastel filler is Champagne or Meudon whiting. Art-Chimie's *Blanc de Meudon* specifies d50 2.4–2.7 µm and D97 13–15 µm; Omya's (Mon-Droguiste) d50 3 µm, d98 25 µm, 38 % under 2 µm; Kenzai's d50 2.4, d98 20 µm, 42 % under 2 µm [M]. A volume lognormal through d50 2.55 and D97 13–15 has σ_ln = ln(D97/d50)/1.881 = 0.87–0.94, and its Sauter mean d₃₂ = d50 e^(−σ²/2) = **1.64–1.75 µm** (Omya's: 1.76; Kenzai's: 1.41). Capillary strength, the pores' hydraulic radius and coverage per volume all scale with d₃₂ (surface per volume), not with d50. The pores' radius: 2 r_h = 2 ε d / 6(1 − ε) = 0.69 µm. No size distribution of a whole stick was found (searched); this is the filler's, which makes up most of a tint.
- **The pores' water [D].** At saturation the pores hold 0.55 g of water per cm³ of stick and dissolve its 0.018 g of gum: **3.3 % w/v**. Gum arabic is surface-active: 46.9 mN/m at 0.5 % at equilibrium (Huang, Kakuda & Cui 2001), 53.7–57.4 mN/m for conventional and matured *A. senegal* (Al-Assaf et al.), and practically constant above its aggregation concentration [M]: **γ = 50 mN/m** (`GAMMA`). (A ferulic-acid study's abstract gives 52.5 mN/m at 1 % for the modified gum and 31.2 for the native one, a pairing its own sentence contradicts; it isn't used.) Its viscosity comes from the measured intrinsic viscosity, 18 mL/g (Gómez-Díaz, Navaza & Quintáns-Riveiro 2008; *A. senegal* 14–23 mL/g [M]), by Huggins with k_H = 0.4 [E: 0.3–0.6 for a compact branched polymer in a good solvent; none measured for the gum was found]: η = 1.002 × (1 + 0.594 + 0.4 × 0.594²) = **1.74 mPa·s** (`pore_viscosity`; k_H 0.3–0.6 gives 1.70–1.80). The audit's 1.3 mPa·s used 2.6 % and no Huggins term. That the gum is arabic, not tragacanth, is supported by GC/MS of a commercial early-20th-century pastel (Boccioni drawing, *J. Cult. Herit.*) [M]; a tragacanth stick would gel its pores, which is not modelled.
- **The filler, whiting [D].** The engine's piles are tubes; a pile's whites (`lead white`, `zinc white`) stand for the whiting a pastel tint is let down with (`dry::particle_index`). The stick's dry colour stays what `dry.rs` gives from its tubes. For the wet optics only, the stick is its other pigments with the share of whiting that gives that colour's luminance by Kubelka–Munk: whiting at n 1.59 (ground calcite; ω 1.658 and ε 1.486 average 1.60 [M]), its dry colour L* 94, a* 0.6, b* 4.1 (Omya Blanc de Meudon, DIN 6174 [M]; linear RGB 0.892, 0.847, 0.790), and its transport scattering per volume, 0.331 µm⁻¹ at d₃₂ 1.7 µm against lead white's 0.879 at the 1 µm `mie_air` is tabled at: **0.376** of lead white's (`dry_pigment_mie.py media`). A pile of whites alone is whiting; a pile without one is its pigments. A pile with no record (`DEFAULT_INDEX`) is whiting's 1.59 (was 1.9).

## 2. Optics: why wet pastel is darker, and how much

- Dry pastel looks pale because its particles scatter strongly against air (dry_pigment_optics.md). Water (n = 1.333) in the pores lowers the contrast. Mie scattering depends on the relative index m = n_p/n_med and on the size parameter π d n_med/λ, both of which move with the medium.
- **`dry::scatter_in(n, n_med)` [D]:** the transport scattering σ_s(1 − g) per particle volume in a medium n_med, over the same in air, by the repository's own Mie code (lognormal σ_ln 0.5, λ 550 nm) at the particles' d₃₂ = 1.7 µm (§1): `uv run notes/research/dry_pigment_mie.py media`. Full water:

  | particle n | 1.50 (ultramarine) | 1.56 (Prussian blue) | 1.59 (whiting) | 1.65 (bone black) | 2.0 (lead white) | 2.4 (chrome yellow) | 3.0 (vermilion) |
  |---|---|---|---|---|---|---|---|
  | S in water / S in air | 0.214 | 0.359 | 0.414 | 0.495 | 0.659 | 0.720 | 0.817 |

  So the darkening is the pigment's own: a stick of ultramarine goes nearly as deep as in oil, vermilion hardly changes. These match the audit's run (estimates_audit_2.py §W4). The PR's first table was computed at 1 µm (ultramarine 0.14, lead white 0.69) while its particles were 3 µm. Whiting averaged over its birefringence (ω twice, ε once) keeps 0.397, within 4 % of the isotropic 1.59 row the engine uses; kaolin 0.35, gypsum 0.27.
- **Each cell's particle index (`Cell::index`) [D]:** a stick's is its pigments' and its whiting's indices weighted by scattering × volume (`Palette::dry_index`, `dry::particle_index`): each part scatters in water by its own contrast, so it counts as much as it scatters (was by volume alone, with no filler). Whiting scatters dry at about 0.4 of lead white per volume, so a pale tint, mostly chalk, darkens in water by its chalk whatever its pigment: vermilion with three parts of white goes from 3.0 to between 1.59 and 3.0. A cell's index follows the pastel laid in it by volume (stick, crumbs, paste). The finger, the dry brush and the air do not carry it: pastel they move keeps the index of the cell it lands in (an approximation; it matters only where sticks of very different indices are blended and then wetted).
- **Partly filled pores [B, E on S₀].** Water wets the grains' surfaces and contacts before it fills the pores, so a half-filled bed is darker than an effective medium of air and water gives. Soil reflectance falls exponentially with moisture, one curve for different soils when moisture is the degree of saturation (Lobell & Asner 2002, *SSSAJ* 66:722, four soils, 400–2500 nm), and in the visible it saturates near 20 % water by volume [M]. The engine blends the dry and the fully wet reflectance by f(S) = (1 − e^(−S/S₀))/(1 − e^(−1/S₀)), **S₀ = 0.2** [E]: their fitted constant wasn't retrieved, and 20 % by volume is S ≈ 0.4 at ε 0.45–0.5, where e^(−0.4/0.2) leaves 0.13 to go (bounds 0.1–0.3). Half full shows 0.92 of the full darkening (`wet_share`). The first version used a Bruggeman effective medium of air and water by the share filled (half full: lead white kept 0.82 of its scattering).
- **Dried again [D].** What is left in the pores is the gum, dry (n 1.52, gum arabic films 1.51–1.54 [M], dry_pigment_optics.md §1), at its share of the pores (0.024 × the share of the gum the pastel kept, §3). Bruggeman gives n_med ≈ 1.011, and Mie leaves 0.98–1.0 of the dry scattering: dried pastel is **within a few percent of its dry colour** (0.3 % darker in the engine's test band). The earlier `DRIED_S` = 0.8 [E] has no physical basis: the gum is too little to fill the pores. The saturated look practitioners describe is in the paste spreading into the tooth (more coverage for its volume, §3), not in its scattering.
- **Compaction of a drying paste [D].** Gum arabic is a steric dispersant, so a paste is dispersed, not flocculated, and its drying menisci pull at 2γ/r ≈ 100 kPa (r ≈ 1 µm), well above a dispersed bed's yield: it packs toward random close packing (ε 0.36). The engine takes **ε 0.40** for a dried paste: its volume shrinks by (1 − 0.55)/(1 − 0.40) = 0.75 (`dried_share`), counted when it is laid, and its coverage stays. Dependent scattering at d ≈ λ isn't in the Mie (the Percus–Yevick factor applies only to d ≪ λ), so S per volume is left as is. Only paste packs: a brush's paste, and the share of a wet stick's deposit it lays as a slurry (§3). Pastel wetted where it lies keeps its packing.
- Paper itself darkens when wet. That is not modelled: only the pastel darkens.

## 3. Mechanics: the gum, the wet tip, the paste

- **Binding by the gum it kept: a threshold [D].** Water dissolves the gum: in the stick it sits as bridges at the particles' contacts, and as the water leaves it sets there again. Gum dissolved in water that sinks into the sheet (§4) goes with it. With the measured uptake of §4 a wash sinks over seconds to minutes, while gum arabic (D ≈ 2 × 10⁻¹¹ m²/s by Stokes–Einstein for a hydrodynamic radius of about 10 nm [E: the order of its bulk fraction, not looked up]) crosses a 40 µm layer in about a minute, so the column is taken as well mixed. What sinks takes the gum at the solution's strength as evaporation concentrates it: kept = (V₁/V₀)^(sunk/(sunk + evaporated)), which is 1 − sunk/V₀ without evaporation (`settle`). The gum left dries into smaller bridges at every contact; a bridge holds as its neck's area, volume^(2/3), so with a share g left each contact holds g^(2/3) of a stick's. The pastel is bound where g^(2/3) σ_t beats the finger's shear τ, and loose where it doesn't (`binds`, `gum_threshold`):
  - σ_t, a dry stick's tensile strength, from its hardness by Prandtl's bearing capacity of a cohesive-frictional solid: H = c N_c with c = σ_t tan φ, so H = σ_t (N_q − 1); φ 31.5° (remoulded chalk 29–34°: Clayton 1978; Razoaki 2000; Bundy 2013 [M]) gives N_q − 1 = 20.9, and a soft stick (soft 0.75, 3.9 MPa) σ_t = **0.19 MPa**, inside the audit's bench bounds of 0.1–1 MPa for a gum-bound chalk compact.
  - τ, the finger's friction: μ 0.5 for a dry fingertip sliding under 0–10 N (0.2–1.75 across flat surfaces) [M, as summarised in *Tribol. Int.* 2010, S0301679X10003221, and *Procedia Manuf.* 2015, S2405451815000641; no finger-on-paper value was retrieved] times its pressure at a firm 2 N press over the engine's own pad (178 mm², `pastel::pad_area`): 11.2 kPa, τ = **5.6 kPa**.
  - g* = (τ/σ_t)^(3/2) = **0.005** [D]; σ_t's bounds put it at 0.001–0.1. So a passage that kept even a small share of its gum is bound; one flooded with many times its pores' water on a sheet that drinks it is left loose.
  - Repeated wetting redissolves and resets the same gum (gum arabic is water-soluble once dry, as watercolour rewets [M]); what sank never comes back. What is bound already, by fixative or an earlier wetting, stays bound.
  - New pastel laid over brings its full gum (the share mixes by volume).
  - Evaporation also carries gum toward the drying top (as it does fixative), strengthening the crust. That is not modelled.
  - The first version bound the share of the pastel equal to the share of its gum kept (0.15 on the default sheet).
- **The wet tip is a soil, `wet_hardness` [B].** Its gum bridges dissolved, a wetted chalk compact is a remoulded chalk soil, and soil mechanics has measured its strength against water content. The tip's water content by mass of solids is w = S ε ρ_w/((1 − ε) ρ_s) = 0.461 S. Remoulded Margate chalk has plastic limit 24.4 % and liquid limit 31.0 % (Bialowas, Diambra & Nash 2016, table 1; crushed chalk 22.4 / 29.8 %, Bialowas, Nash & Diambra 2018) [M], so the tip passes its plastic limit at **S 0.53** and its liquid limit at **S 0.67**; its liquidity index is LI = (w − 0.244)/0.066.
  - **Below the plastic limit** it is brittle and keeps its dry hardness.
  - **In its plastic range** its remoulded undrained strength falls with LI. Wroth & Wood (1978, *Can. Geotech. J.* 15:137) set it at 1.7 kPa at the liquid limit (measured 0.7–2.65) and ×100 at the plastic limit: s_u = 170 e^(−4.6 LI) kPa. Vardanega & Haigh (2014, *Can. Geotech. J.* 51:1073; 641 fall-cone tests on 101 soils) find the ×100 overpredicts and a ratio of about 35 fits (0.2 < LI < 1.1) [M]; the engine uses theirs, **s_u = 1.7 × 35^(1 − LI) kPa**. Its indentation hardness is Prandtl's undrained bearing capacity, H = (2 + π) s_u = 5.14 s_u [D]: **0.31 MPa** at the plastic limit, 52 kPa at LI 0.5 (S 0.60), 9 kPa at the liquid limit. Tabor's factor of 3, which the first version used on a capillary strength, is for a frictionless plastic metal.
  - **Past the liquid limit** it is a slurry, with no strength to wear against. It is laid as paint is (below), not by Archard.
  - The drained upper bound, c N_c with c = σ_t tan φ from Schubert's capillary strength (6 (1 − ε)/ε · γ/d = 0.14 MPa at γ 50 mN/m, d 1.7 µm), is 15–28 × σ_t = 2.2–4.1 MPa: a quick stroke loads the tip undrained, so the undrained strength applies.
  - A kaolin-rich stick (kaolinite PL 0.25–0.40, LL 0.30–0.75: White 1949, *Am. Mineral.* 34:508 [M]) would still be plastic when soaked; the engine's sticks are chalk-filled (§1).
  - The Archard wear coefficient K is kept as dry below the plastic limit (0.01–0.06, the dry range). The bench measures the weight laid per force and length at known S.
- **The slurry is pressed into the tooth [D].** Past the liquid limit the tip conforms to the paper (at 9 kPa it carries a 2 N press over 230 mm², more than its face) and the face presses its slurry into the open pores of the tooth it touches: the paper's micro-relief holds µ (its mean pore depth below the fibre tops, paper.rs) per pixel, less what pastel already fills, and that is what the slurry lays, as paint fills the valleys under a brush. Between the plastic and liquid limits the deposit is the Archard wear and the slurry's in the share LI (`paste_share`). What the slurry lays takes its pores' water with it (ε S of its volume), so a dipped tip lays its slurry until its water no longer keeps it past the liquid limit: the wetted volume limits it. In the engine's test the soaked stroke reaches more of the tooth than the dry one.
- **Crumbs by the limits [B].** The crumbs (`SHED` = 1 − 1/6.6, measured for graphite) come from brittle fracture. Below the plastic limit the tip sheds them as dry; the plastic range smears; past the liquid limit the paste flows: the shed share falls with LI from full at the plastic limit to none at the liquid limit (was linear in the saturation).
- **The paste spreads, `paste_thin` [D].** Dry, the deposit covers as particles 6 µm thick (`PARTICLE_UM`, crumbs and fragments). Wet, the water disperses it to primary particles: d = 1.7 µm spheres cover 1.5/d of area per volume of solids, and a stick's volume holds 1 − ε of solids, so a paste covers as particles d/(1.5(1 − ε)) = 2.5 µm thick: **0.42** of the dry thickness (was 0.83 at 3 µm and ε 0.6).
- **The water it lays [D].** A saturated paste carries its pores' water: ε = 0.55 of its volume, times the tip's saturation. (The first version had ε/(1 − ε), a volume of water per volume of solids applied to stick volume.)
- **The tip's water [D].** The water spreads up the dry stick by Lucas–Washburn in its pores, D = r γ/(2η) with r 0.69 µm, γ 50 mN/m and η 1.74 mPa·s (§1): **D ≈ 10 mm²/s** (the first version's 54 mm²/s took 3 µm particles and pure water; the audit's 13 took η 1.3). It wicks up while the tip is in the jar too, so a dip soaks `dip_mm` = 1 mm + √(D t_dip) = **3.2 mm** of the end for a 1 mm, half-second dip (the depth and time are gestures, not physics; `p:wet(share)` sets the share of that). Afterwards l² = l₀² + D t and the tip keeps l₀/l of it: after a second 0.71, after ten 0.31, so a dip serves a few strokes. It also evaporates from the end and the wet side at the rate of §4. A tragacanth stick would gel its pores and hold its water far longer; that is not modelled.
- **"A dampened stick can be drawn over earlier layers without lifting them"** [P] (notes/degas-materials.md §5, N-A 618). This agrees with a softer stick that lays more and sheds less.

## 4. Water on the paper: where it goes, how fast it goes

- **Which pores hold the water [D].** Capillary suction is 2γ cos θ/r. The pastel's pores (r ≈ 0.7 µm, chalk and clay wetted) pull several times harder than a drawing paper's (r ≈ 6 µm, paper.rs), so the pastel's pores fill first and empty last, and only water standing over them reaches the sheet. The sheet's water also rises back into pastel pores that aren't full.
- **Sinking: the sheet's measured uptake, to its pores' capacity [B].** Water into the sheet is U(t) = min(cap, A √t) (`sink_um`, `uptake_rate`, `sheet_capacity_um`):
  - **A sized sheet** takes water at the rate of its sizing, which an angle can't give: the size wets slowly. Its Cobb60 (ISO 535, TAPPI T 441: g/m² taken up in 60 s, = µm of water) sets A = C₆₀/√60 s. The default sheet is **Cobb 25 g/m²** (`Paper::cobb60`): sized writing, office and bond papers measure 22–30, unsized 50 and more (paperonweb, *Properties of paper*, by TAPPI T 441 / ISO 535 / ASTM D 3285) [M]; Canson Mi-Teintes is gelatin-sized (retail listings) [P]. No Cobb value of an artist's drawing or pastel paper was found (searched). It takes 3.2 µm in the first second, 25 µm in a minute.
  - **An unsized sheet** (`cobb=false`) takes water as fast as its pores draw it: Whatman Grade 1 filter paper measures a Washburn coefficient K = 0.38 cm/s^½ (D = 14 mm²/s; *Sci. Rep.* 2019, s41598-019-52202-9, from 4 cm wicked in 110 s) [M], so A = ε K: it fills a 300 µm sheet within milliseconds. (ISO 535 isn't meant for unsized papers.)
  - **The cap** is what the sheet's pores take: the share of its pore volume a liquid reaches (`Paper::absorbent`, which already gives a sheet's oil capacity in `Canvas::with_paper`) × ε × caliper: 0.8 × 0.5 × 300 µm = **120 µm** for the default sheet. Once full it takes no more, and the rest stands and evaporates.
  - The uptake is followed from the water the sheet already holds, u² + A² t, so a sheet that has dried partly takes water again at its rate.
  - The Cobb and Whatman rates are measured with water; the pores' 3.3 % gum would slow a Washburn flow by (γ/η) to 0.53 at most, less in a dilute wash, which is not applied. Lateral spreading through the sheet (a blot on unsized paper) isn't modelled either.
  - With Cobb 25 a 40 µm wash over a thin layer stands for about 2.5 minutes and sinks almost entirely; at 0.71 µm/min little of it evaporates first. It keeps the gum of its pores' water: the column is well mixed (above), and the share kept is about pores/(pores + wash).
  - The first version took Washburn at the paper's geometric pore radius with a contact angle mixed by Cassie from `absorbent` (105° sized, 0° bare) and no cap: about 6.4 mm of water in the first second, 10⁴ times the measured rate.
- **What sank is the sheet's** (`Drawing::sheet`, per cell). It no longer darkens the pastel's look directly, but the pastel's pores, which draw harder, stay full while the sheet beneath still holds water: it evaporates up through them first, and the pastel dries last. The sheet dries through its face only (a sheet pinned to a board). The paper's own darkening when wet and water spreading sideways through it are not modelled.
- **Evaporation: psychrometrics [D], `evaporation_um_min`.**
  - The flux is h_m (ρ_v,sat(T_s) − RH · ρ_v,sat(T_a)).
  - h_m comes from ISO 6946's convective coefficient for a vertical surface in still room air, h_c = 2.5 W/m²K [M], by the Chilton–Colburn analogy h_m = h_c/(ρ c_p) · Le^(−2/3). With D_v = 2.48 × 10⁻⁵ m²/s at 20 °C (Massman 1998) [M], Le = 0.87, so h_m = 2.27 mm/s.
  - The saturated vapour pressure follows Buck (1981) [M].
  - The wet surface cools until the latent heat it loses equals what convection and the room's radiation bring it (4εσT³ = 5.4 W/m²K, ε 0.95). It settles near 16.3 °C.
  - The result is **0.71 µm of water a minute** (about 1 mm a day; an isothermal surface would give 1.2). That sits inside the 0.8–3 µm/min range of the indoor pool correlations (§ Sources) at its still-air end. It replaces `EVAP_UM_MIN` = 2.5 [E].
- **The studio, `crate::climate::STUDIO` [D within bounds].** One setting for every humidity-dependent effect (today evaporation; later the gum's glass transition and the capillary hold of particles on a sheet): 20 °C (the air sheet.rs assumes) and 50 % RH. Heating Paris's outdoor air without adding moisture (Montsouris: annual mean 12.8 °C, 1991–2020; RH 76 %, 1961–90, Infoclimat via Wikipedia *Climate of Paris*) gives 48 % at 20 °C; January's (5.4 °C, 83 %) gives 36 % at 18 °C; an unheated July room stays near 68 %. Evaporation at those is about 0.7, 0.86 and 0.46 µm/min. 50 % sits in that range; the museum band of 40–60 % is for storage, not studios, and no measured historic studio record was found. A log-head room setting would change it in one place.
- **Falling rate [D].** Once the standing water is gone, the evaporation front recedes into the pastel's pores. The vapour then also crosses the dry pores above it, with diffusivity D_v ε^1.5 (Bruggeman's tortuosity), so the flux becomes E/(1 + a u), where u is the water missing from the pores and a = h_m/(ε D_eff) ≈ 3.3 × 10⁻⁴ per µm.
  - The pastel layer is thin (tens of µm), so this slows its last water by well under a percent. In still air the boundary layer, not the pores, limits drying.
  - Handprint's slow "damp" stage (watercolor_physics.md §2) is water deep in the sheet, which the engine does not track.
- **What this gives.** A wet stick's line (a few µm of water) dries in minutes. A brushed wash of about 25–40 µm sinks into the default sheet over a minute or two and then dries out of it through the pastel at 0.71 µm/min: about an hour, the pastel dark until the sheet under it is dry. On a sheet whose pores are full it stands and evaporates in the same time. Handprint's 20 minutes for "a generous wash" [P] counts a sheet that spreads its water sideways into dry paper and dries from both faces, which the engine does not follow.

### 4.1 Water on a gritted ground (`paper={grit=}`, sanded_paper.md)

- **Acrylic coat [D].** A dried acrylic film is a waterproof binder over the fibers (sanded_paper.md; the reason an acrylic coat already seals the paper against oil, `Paper::oil_capacity`). Water laid on it stands in the coat's tooth with the pastel and leaves only by evaporation: `sheet_capacity_um` is 0. So a wash on a sanded sheet keeps all its gum and binds, where the same flood on an unsized sheet is drunk with its gum and stays loose. It follows from the binder, with no new constant.
- **Glue coat [D].** Animal glue or starch at 7 % solids by volume (sanded_paper.md §1) leaves most of the coat open, so water passes through to the sheet as oil does, at the sheet's own sizing and to its own capacity. The glue is itself a size (gelatin is what Cobb sizing measures), so no separate rate is added.
- **Not modelled:** a glue coat softening and swelling in water (it is water-soluble and can let grains go under a wet brush); water filling the coat's open tooth as standing room (it is counted with the pastel's pores and the standing water, as on paper).

### 4.2 The named sheets' sizing (`paper="tan wove"` …, cassatt_papers.md)

- **The wove and laid sheets keep Cobb 25 [D].** Machine-made paper was internally (engine) sized with rosin and alum from the 1830s–1840s on, and paper made 1850–1950 is rosin-alum sized (Britannica, *Papermaking*; Handpapermaking, *Sizing at Hayle Mill*; AIC Book and Paper Group Annual 29, the aging of rosin-alum sized papers). That is the writing and bond papers' sizing, Cobb 22–30 (§4), so the 1880s–1900s colored wove sheets keep the default 25. The mold-made rag laid sheet is gelatin sized as Canson's is, and keeps 25 as well [E: no Cobb measured on a period sheet].
- **Japan paper is unsized [P].** "Most Japanese papers are unsized and, as a result, are generally more absorbent" (Japanese Paper Place, *Washi and printmaking*); kozo printing sheets are sold sized only for woodcut. Its `cobb60` is None, so water enters at its pores' Washburn rate, to its capacity (0.8 × 0.6 × its caliper).

### 4.3 Water over charcoal (`charcoal()`, charcoal.md)

- **Charcoal brings no gum [D].** Vine charcoal is a charred twig with no binder (charcoal.md), so the gum a wetted cell holds is the pastel's share of its volume (`Cell::carbon_share`): a cell of vine alone dries as loose as it was, and pastel under charcoal binds it as its own gum allows, the carbon that binds counted as bound carbon (seen bare, as fixed charcoal is).
- **A wet brush carries charcoal as charcoal [D].** What it picks up keeps its share of carbon and that carbon's lift (`pastel::Carried`, as the finger and the dry brush carry it), across passes in the brush's load; a stick ground into the water (`b:dip(share, {pastel=})`) adds its carbon.
- **Left out.** Compressed charcoal's gum (about 5 % against pastel's 1.5 %, charcoal.md) is counted as none, because a cell doesn't keep which charcoal it holds: wetted compressed charcoal binds less than it should. Carbon's index in water (n ≈ 2) is the stick's default, not carbon's; a bed of R∞ 0.02 hardly darkens either way. A charcoal stick's own wet tip isn't modelled: it is porous char (or carbon in gum), not a chalk soil with measured plastic and liquid limits, so `p:wet` refuses it.

## 5. The brush: how much it holds and lays

- **Capacity, `Held::capacity_mm3` [C, with one estimate].** A round's belly is taken as a cone of the brush's width and a hair length 4.7 times that, the liquid filling 0.6 of it. The ratio is Rosemary & Co Series 33 kolinsky rounds' (retail tables: size 4 3.4 × 15.7 mm, 6 3.9 × 20 mm, 8 5.2 × 24.2 mm, 4.6–5.1) [M]; the 0.6 is the hairs' packing in a wet bundle [E: no gravimetric measurement of a sable's load was found; retailers say only "3× a synthetic"]. A 6 mm round holds about 0.16 mL, a 10 mm one 0.74 mL. Other shapes keep the engine's full paint load (`Held::full`). The first version used the full paint load for every brush (0.14 mL for a 10 mm round sable).
- **Paste, `amount=` [D].** A paste at the stick's own packing fills the space between the hairs, so at most `capacity` of stick volume. Its solids (1 − ε of it) push out the water, and the water is trimmed to capacity − (1 − ε) × paste. `amount` is the share of that most. This replaces `BRUSH_PASTE_SHARE` = 0.25 [E].
- **How it lays, picks up and carries [D from the engine].** The water a full load lays at the start of a stroke is the tool's `lay` (25 µm a coat for a sable; was `WASH_UM` = 30 [E]). The share of the wetted paste under it that it takes up in a pass is the tool's `pickup`, the share of wet paint a bristle takes up (0.1 for a sable; was `PICK_WET` 0.5 [E]). It lays what it holds within its `run`, the length over which a load runs down (was `CARRY_MM` 12 [E]). The wetted paste is wet paint to the brush, so its handling is its handling of paint.
- **Pressure [D from the engine].** The water a press lays follows the bristle model's own law for the film a touch squeezes out of the belly, 0.4 + 0.6 p (`bristle.rs`, `touch`), as paint strokes use. That law is itself the brush model's estimate (W23). The first version had 0.5 + p.
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

The water the sheet has taken in under each cell (§4) is the other case: only the drying (`wait`, `dry`) and the checkpoint read it, never a repaint. So it is a side buffer, `Drawing::sheet` (4 bytes a cell), allocated the first time a wet painting's water dries, and empty in every other painting.

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
- A tragacanth stick's gelled pores, and the gum's slowing of the sheet's uptake.
- Gum carried to the drying top of a passage.
- The finger, the dry brush and the air carrying a cell's particle index with the pastel they move.

## 8. The estimates audit, applied

notes/estimates-audit-2.md (branch notes/estimates-audit-2), the #322 table. Each source was opened and checked before use; where a check changed the audit's number, the checked value is used.

| # | Constant | Was | Now | Source or derivation | Checked |
|---|---|---|---|---|---|
| W1 | Water into the sheet | Washburn at the paper's pore radius, Cassie angle from `absorbent`, no cap (6.4 mm in 1 s) | Cobb √t (25 g/m² default) or Whatman K 0.38 cm/√s unsized, capped at absorbent × ε × caliper (120 µm) | paperonweb; *Sci. Rep.* 2019 | yes |
| W2 | Wet tip hardness | Schubert × Tabor 3 (0.29 MPa), linear in S | dry below S 0.53; 5.14 × 1.7 × 35^(1−LI) kPa above; slurry past S 0.67 | Bialowas 2016; Wroth & Wood 1978; Vardanega & Haigh 2014 (×35, not ×100) | yes |
| W3 | Particle size | 3 µm [E] | 1.7 µm (d₃₂) | Art-Chimie, Omya, Kenzai whiting | yes |
| W4 | Wet Mie table; filler; default index | 1 µm table, no filler, 1.9 | 1.7 µm table; whiting share by KM luminance; S-weighted index; 1.59 | repo Mie; Omya L*a*b* | yes |
| W5 | Binding | share = gum kept | threshold g ≥ (τ/σ_t)^1.5 = 0.005 | Prandtl from the dry hardness; finger μ 0.5 | partly (σ_t derived, not measured) |
| W6 | Studio | 20 °C, 50 % in wet_pastel.rs | the same, as `climate::STUDIO` | Montsouris normals | yes |
| W7 | Porosity | 0.60 | 0.55 | Sennelier listing | yes (listing weight) |
| W8 | Shed | SHED (1 − S) | full to the plastic limit, none past the liquid limit | W2's limits | yes |
| W9 | Dip depth | 1 mm | 1 mm + √(D × 0.5 s) = 3.2 mm | W11 | — |
| W10 | Pore water γ, η | 72.8 mN/m, 1.002 mPa·s | 50 mN/m, 1.74 mPa·s | Huang 2001; Al-Assaf; [η] 18 mL/g (Gómez-Díaz 2008) + Huggins | yes; η differs from the audit's 1.3 |
| W11 | Tip Washburn D | 54 mm²/s | 10 mm²/s | W3, W7, W10 | — |
| W12 | Paste coverage thickness | 0.83 | 0.42 | W3, W7 | — |
| W13 | Partly filled pores | Bruggeman air/water | exponential in S, S₀ 0.2 | Lobell & Asner 2002 (form; S₀ [E]) | form yes; constant not retrieved |
| W14 | Brush water | full paint load (0.14 mL / 10 mm) | cone, L = 4.7 d, 0.6 filled (0.16 mL / 6 mm) | Rosemary Series 33 tables | ratio yes; 0.6 [E] |
| W15 | Paste compaction | none | dries to ε 0.40 (volume × 0.75) | audit's derivation | — |
| W16 | Wash pressure | 0.5 + p [E] | 0.4 + 0.6 p, the bristle touch law | bristle.rs | yes |
| W17 | Archard wet | dry K throughout | dry K below PL; slurry fill past LL; between by LI | W2 | — |
| W18–W21 | h_c, emissivity, gum n and ρ, falling rate | kept | kept | confirmed by the audit | — |
| W22 | Gum share | 1.5 % | kept (D: bounds 0.3–3 %) | no assay found | — |
| W23 | Brush lay, pickup, run | the tool's | kept (the brush model's estimates) | — | — |

**Still estimates, with why:**
- **S₀ = 0.2** (W13): Lobell & Asner's fitted constant wasn't retrieved; set by the visible's saturation point, bounds 0.1–0.3.
- **Huggins k_H = 0.4** (W10): none measured for gum arabic; 0.3–0.6 moves η by ±3 %.
- **The brush's 0.6 liquid fill** (W14): no gravimetric measurement of a sable's load.
- **The finger's press for the binding threshold**, 2 N (W5): a firm press, the knee of the pad's growth; a gesture. At the default 0.6 N g* is 0.002, at 10 N 0.04.
- **σ_t** (W5) is derived from the engine's own soft-stick hardness (itself the tribology note's range), not measured: the bench's tensile test of a dry stick would set it (bounds 0.1–1 MPa).
- **Gum share 1.5 %** (W22, D): no assay; ash or TGA weight loss of a commercial stick.
- **The dip's 1 mm and 0.5 s** (W9): gestures.
- **The studio's 50 %** (W6): a choice inside measured bounds (36–68 %).

## Sources
- Britannica, *Papermaking* (Illig's rosin-alum sizing, published 1807, in wide use about 25 years later): https://www.britannica.com/technology/papermaking
- Handpapermaking, *Sizing at Hayle Mill* (rosin sizing in common use from the 1840s): https://www.handpapermaking.org/magazine-articles/sizing-at-hayle-mill
- AIC Book and Paper Group Annual 29, *Study of the aging behavior of rosin-alum sized papers* (paper of 1850–1950 internally sized with rosin and alum): https://cool.culturalheritage.org/coolaic/sg/bpg/annual/v29/bpga29-21.pdf
- Japanese Paper Place, *Washi and printmaking* ("Most Japanese papers are unsized"): https://www.japanesepaperplace.com/news/washi-and-printmaking/

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

- Wroth, C. P. & Wood, D. M. (1978), "The correlation of index properties with some basic engineering properties of soils," *Can. Geotech. J.* 15(2):137–145.
- Vardanega, P. J. & Haigh, S. K. (2014), "The undrained strength–liquidity index relationship," *Can. Geotech. J.* 51(9):1073–1086, doi 10.1139/cgj-2013-0169.
- Bialowas, G., Diambra, A. & Nash, D. (2016), "Small strain stiffness evolution of reconstituted medium density chalk" (remoulded Margate chalk, LL 31.0 %, PL 24.4 %); Bialowas, Nash & Diambra (2018), "Stress and time-dependent properties of crushed chalk" (LL 29.8 %, PL 22.4 %).
- White, W. A. (1949), "Atterberg plastic limits of clay minerals," *Am. Mineral.* 34:508.
- paperonweb, "Properties of Paper" (Cobb by grade): https://www.paperonweb.com/paperpro.htm; ISO 535; TAPPI T 441.
- "Paper stacks for uniform rehydration of dried reagents in paper microfluidic devices," *Sci. Rep.* 9 (2019), s41598-019-52202-9 (Whatman Grade 1, K = 0.38 cm/s^½).
- Art-Chimie, *Carbonate de calcium, Blanc de Meudon* (D50 2.4–2.7 µm, D97 13–15 µm): https://www.art-chimie-online.com/produits-chimiques/89-carbonate-de-calcium-blanc-de-meudon.html; Mon-Droguiste (Omya Blanc de Meudon: L*a*b* 94/0.6/4.1, d50 3 µm, d98 25 µm): https://www.mon-droguiste.com/blanc-meudon-omya-rouge.html; Kenzai, *Blanc de Meudon*.
- Huang, X., Kakuda, Y. & Cui, W. (2001), *Food Hydrocolloids* (gum arabic 46.9 mN/m at 0.5 %); Al-Assaf et al., "Hydrocolloids with emulsifying capacity. Part 1" (57.4 and 53.7 mN/m).
- Gómez-Díaz, D., Navaza, J. M. & Quintáns-Riveiro, L. C. (2008), "Intrinsic viscosity and flow behaviour of Arabic gum aqueous solutions," *Int. J. Food Prop.* 11 (18 mL/g).
- Lobell, D. B. & Asner, G. P. (2002), "Moisture effects on soil reflectance," *SSSAJ* 66(3):722–727, doi 10.2136/sssaj2002.7220.
- Rosemary & Co Series 33 dimensions: Artsavingsclub and Hobby Heaven listings.
- Sennelier extra-soft pastel, White 525 and Ivory Black 513 (9.07 g; 2.65 × 0.45 × 0.45 in): retail listings.
- Climate of Paris (Wikipedia; Montsouris RH 1961–90 from Infoclimat).
- Finger friction: "The frictional pattern of tactile sensations in anthropomorphic fingertip," *Tribol. Int.* (2010), S0301679X10003221 (0.5 dry, 0–10 N, from earlier studies); "Quantitative assessment of friction perception for fingertip touching with different roughness surface" (2015), S2405451815000641 (0.2–1.75). Skedung et al. 2010, *Tribol. Lett.* 37:389, measured fingers on 21 printing papers; its values weren't retrieved.
