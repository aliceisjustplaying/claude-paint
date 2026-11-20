# Damp printing paper: a sheet's own water

The numbers behind crates/paint/src/damp.rs (the damp-paper switch, `--@ damp-paper`; notes/plan-intaglio.md P3a). It builds on notes/research/intaglio_physics.md §6.2 and §7.2 (cited as **IP §n**) and on PR #322's water model, notes/research/wet_pastel.md §4 (cited as **WP §4**). Tags: [M] measured, with a source; [D] derived here from measured values or standard physics, the arithmetic shown; [P] a practitioner's account; [E] an estimate, with the reason it is one.

**Bottom line.**
- A sheet's water is kept for the whole sheet: g of water per m², and how many faces are open to the air.
- **Where it is.** The fiber walls hold it by a measured sorption isotherm, up to fiber saturation (0.29 g/g). Past that it fills the pores between the fibers, to #322's capacity. Past that it stands on the faces, and the sheet is shiny.
- **Soaking** takes water through both faces at the sheet's measured Cobb rate. **Draining** leaves a gravity-thinned film on each face (Jeffreys).
- **Blotting and the damp pack** are capillary equilibrium between sheets in contact. A soaked 250 g/m² sheet blotted once between 0.8 mm cotton blotters comes to **0.40 g/g** [D]. A printer's pack of a pint to 35 sheets comes to the plan's **0.56 g/g** of water over the conditioned sheet [D from P].
- **Drying** is #322's psychrometric evaporation, **0.71 g/m² a minute** from each open face while free water is there. Below fiber saturation it slows with the walls' water activity, toward the room's **0.078 g/g**.
- **Stiffness** falls by Nissan's measured law to **0.22** of the dry sheet's at fiber saturation, and no further [D from M]. That is about fourfold, where the plan estimated tenfold.
- **Size.** A machine-made sheet grows **2.5 %** across its machine direction and 0.6 % along it, damp against dry [D from M]. That is inside the measured 2.4–2.75 % cross-direction wet expansion of machine papers [M].

## 1. Where the water is

- **Moisture content (MC)** is g of water per g of dry fiber. A sheet's grammage is measured conditioned at ISO 187's 23 °C and 50 % RH, so its dry fiber is grammage / (1 + M(0.5)) (`dry_fiber`).
- **The walls: GAB isotherm [M].** M = M₀ C K a / ((1 − K a)(1 − K a + C K a)), with a the water activity.
  - Bleached loblolly pine kraft at 25 °C, fitted over a = 0.12–0.93: M₀ 5.04 %, C 13.9, K 0.83. This is from the table of published fits in Parker, Bronlund & Mawson (2006), read through a search summary of the table's row.
  - At 50 % RH it gives **0.078 g/g**. Sized papers are commonly 6–8 % [P, the general figure]. Kraft paper at 25 °C is about 15 wt% at high humidity [M, same review's summary].
  - The review notes GAB fails above about 85 % RH. So its value at a = 1, **0.29 g/g**, is an extrapolation.
  - **Check [D].** That 0.29 is the fiber saturation point measured for cellulose fibers, 0.25–0.35 (wood and pulp data; IP §7.2). It agrees with the engine's own sheet: `Paper::drawing`'s fiber, 0.16 mg/m over 25 × 6 µm, has a density of 1.07 g/cm³ against cellulose's 1.5. Its own void is 1/1.07 − 1/1.5 = **0.27 cm³/g**, about the water its walls take at saturation.
  - The engine therefore takes the walls as holding water by the isotherm up to M_fsp = M(1), with free water present only above it (`activity`).
- **The pores.** Free water fills the pores between fibers, up to #322's capacity: `absorbent` × ε × caliper (WP §4). For the default sheet at 250 g/m² that is 0.8 × 0.5 × 469 µm = **188 g/m²**. Past it, water stands on the faces.
  - The capacity of a 250 g/m² sheet is 232 g/m² of fiber × 0.29 + 188 = 255 g/m², or **1.10 g/g**.
- **Sorption hysteresis** (desorption runs above adsorption) is left out. So is the irreversible shrinkage on a first drying (Pulkkinen et al.'s Fig. 2): one isotherm, the adsorption fit.

## 2. Soaking and draining

- **Into the tray [D from M].** A sheet under water takes it through both faces at #322's measured rate, A √t on each face. A is Cobb60/√60 s for a sized sheet (Cobb 25 g/m² by default); an unsized sheet takes ε K with Whatman's K (WP §4).
  - The two faces together take 2A √t, followed from what the sheet already holds above its conditioned water, as #322 follows it.
  - It stops at the capacity (§1).
  - A 250 g/m² sheet takes 112 g/m² in 5 minutes and fills in about 22 minutes. That sits with practitioners' 30 s to "over 5 minutes", or overnight for hard-sized sheets (IP §6.2) [P].
- **Draining [D].** A film on a vertical wall drains as h(x, t) = √(η x/(ρ g t)), with x measured down from the top (Jeffreys 1930, the standard gravity-drainage solution once the film is much thinner than at withdrawal).
  - Over a sheet L tall, held by its top edge, the mean film is ⅔ √(η L/(ρ g t)) on each face, using water at 20 °C (η 1.002 mPa·s, ρ 998 kg/m³).
  - A 0.3 m sheet drained 30 s keeps **21 µm** a face. A 150 mm sheet keeps 15 µm.
  - The drain time is the printer's gesture, 30 s by default [E: not physics].
- **Misting** (`soak(s, {mist=})`) adds the water sprayed or sponged on. Unsized and Japan papers are misted, not soaked [P, IP §6.2]. Its spread through the sheet isn't followed: the state is the whole sheet's.

## 3. Blotting and the damp pack: capillary equilibrium

- **The physics [D].** Sheets in contact exchange water until its suction is the same in all of them.
  - **The walls first.** Below saturation the walls' suction is hygroscopic: at a = 0.9 it is RT ln a / V_w = 14 MPa, against a pore's few kPa. So the walls fill first. While they are not all full, they share the water at one activity, which with one isotherm is one moisture content.
  - **Then the pores.** Past the walls' saturation, free water fills the pores at one capillary suction. A slit pore of height h holds water against a suction of 2γ/h, so water fills every pore below one height h*, in every sheet alike.
- **Pore heights [M].** Paper's pore heights are near exponential about their mean (Niskanen & Rajatora 2002, handsheet cross-sections). The mean is t·ε/(1 − ε) for fiber thickness t (Dodson 2001, the large-n limit: paper_mechanics_and_transport.md §2, as paper.rs uses it).
  - A pore holds volume in proportion to its height, so the pore volume below h* is the Gamma(2) share 1 − e^(−x)(1 + x), with x = h*/h̄ (`filled_share`).
  - The default sheet's pores average 6 µm.
  - **Check.** The in-plane pore radii of random fiber networks are spread with a CV of 0.79 (Corte & Lloyd; Sampson 2001, eq. 96–97) [M/D]. That spread is the same order.
- **Water past all the pores** stands between the sheets, shared equally (`equalise`).
- **The blotter [M, E].**
  - Whatman/Cytiva GB003 is 0.8 mm thick, pure cotton linters, unsized [M, maker's listing].
  - Its grammage is not published. A pack of 25 at 300 × 600 mm ships at 1.64 kg gross, so it is at most 364 g/m²; **330** allows for the packing [E].
  - Its fiber is taken as the engine's (1.07 g/cm³, 6 µm thick) [E: no linters' dimensions to hand]. Its porosity is therefore 1 − 0.41/1.07 = 0.61, its pores average 9.5 µm, and it holds 490 g/m² of free water.
  - A printmaker's blotter may be thinner. `blot(s, {blotter={mm=, grammage=}})` takes another.
- **What it gives [D].** A 250 g/m² sheet 150 mm tall, soaked full, drained 30 s and blotted once between two GB003 blotters, comes to **0.40 g/g**. A 0.3 m sheet keeps a thicker film and comes to 0.42.
  - The blotters' own walls take 2 × 306 × (0.29 − 0.078) = 130 g/m² of it first.
  - Thinner blotters leave more: two 0.34 mm, 190 g/m² sheets [E: a light blotter's make] leave 0.50.
  - It stays past fiber saturation (soft through, "damp"), with its free water in its finer pores.
  - This is below IP §6.2's estimate for soaked and blotted sheets, 0.5–1 g/g [E], which it replaces.
  - Taking the blotter's fiber as linters' thicker, uncollapsed one (about 10 µm [E]) would leave more, 0.47.
- **Contact time.** The model takes each pass to equilibrium. Water crosses a pressed blotter's pores in well under a second (Whatman's Washburn D = 14 mm²/s fills 0.8 mm in 0.05 s, WP §4) and enters the walls in about as long (a 3 µm wall at D ≈ 10⁻¹⁰ m²/s [E]). A printer's press of a few seconds is longer [D].
- **The damp pack [P → D].** One printer damps 35 sheets of 10 × 15 in BFK (250 g/m², 24 g each) with under a pint, 0.47 L (IP §6.2).
  - That is 13.4 g a sheet, **0.56 g/g** over the conditioned sheet, the plan's number.
  - On a dry-fiber basis, with the sheet's own conditioned water, it is (13.4 + 1.7)/22.3 = 0.68 g/g at most. The engine reports the dry basis.
  - Interleaved under a board, the sheets come to equilibrium. Being one make, they come to one MC, so the pack's MC is its mass balance whichever sheets were wetted.
  - Covered, they lose nothing while they lie.
- **The pack's time** is taken as long enough to equalise [E]. Liquid water between pressed damp sheets moves by capillarity through 0.5 mm in minutes, if the free Washburn rate above is any guide, and an overnight pack is far longer. No measured equalisation time was found.

## 4. Drying on the clock

- **Free water: #322's evaporation [D].** While free water reaches an open face, the face is a wet surface in the studio (`climate::STUDIO`, 20 °C, 50 % RH). It is the still-air boundary layer's mass transfer, cooled to its energy balance: **0.71 µm (g/m²) a minute** (WP §4, `evaporation_um_min`).
  - A sheet lying on the bench has one face open; one hung up has two.
  - The constant rate holds down to fiber saturation, because capillary supply far outruns this flux. The pores' water moves at the Washburn rates of §3, millimetres a second, against 0.012 µm/s evaporating [D]. That differs from a paper machine's falling rate at 1–1.5 g/g, under heat fluxes thousands of times larger.
- **The walls' water [D].** Below fiber saturation the surface's vapour is a times saturation, with a the walls' activity from the isotherm.
  - The same energy balance gives the flux e(a) (`evaporation_at`), falling to 0 at the room's RH.
  - dM/dt = −faces · e(a(M)) / fiber. A sheet comes toward the room's moisture, 0.078, and never below it.
  - The engine integrates it once, on a grid in u = ln(a − RH) (`Clock`), where the integrand M′(a)(a − RH)/e(a) is smooth. A wait reads the clock forward, so a wait split in two dries as one wait, to rounding.
  - The test checks it against a step-by-step (RK4) integration of the law.
- **Covered** (`cover`: plastic, blotters, a board, a damp pack) it loses nothing (IP §6.2) [P].
- **What this gives [D].**
  - A blotted 250 g/m² sheet at 0.40 g/g holds (0.40 − 0.29) × 232 = 25 g/m² of free water. On the bench it is damp for **35 minutes**.
  - Fresh from the tray with its film it holds about 210 g/m² free, which lasts **five hours** on one face.
  - Between plates a printer keeps it covered [P].
- **Not modelled.** The sheet's edges drying first, and water moving sideways within it, are left out: the state is the whole sheet's.

## 5. Stiffness and plasticity

- **Nissan's law [M].** Paper's modulus falls exponentially with moisture above 5 %: ln E = A − B·MC, with B nearly the same for every paper (Nissan's hydrogen-bond theory; Linvill & Östlund 2014).
  - Caulfield (1990) gives about 7 % per 1 % of moisture, so **B = 7 per g/g**. This is read through a summary of his review; the FPL PDF would not load.
  - Zauscher, Caulfield & Nissan (1996–97) confirm the form on 24 papers. A sonic modulus falls at 75 % of the quasi-static rate.
- **Past fiber saturation the free water breaks no more bonds** (Nissan's theory counts bound water only) [D]. The modulus is held at its saturation value, `stiffness`: exp(−7 × (0.29 − 0.078)) = **0.22** of the conditioned sheet's.
  - The plan's estimate was "about tenfold" [E]. This is about fourfold, derived.
  - The wet tensile *strength* of unsized paper, 5–10 % of dry (IP §6.2), falls further than its stiffness. That is the bonds breaking, not their stiffness.
- **The press's z-compression (P3b).** Kananen, Rajatora & Niskanen's E* of 4–5 MPa (paper_mechanics_and_transport.md §1) becomes **0.9–1.1 MPa** damp. IP §6.2 estimated 0.5–1 MPa. `Moisture::z_mpa` scales the sheet's own `z_mpa` the same way, for the press to use.
- **Plasticity [D from IP §6.3].** Past fiber saturation the bonds are opened by water, so a shape pressed into the sheet sets as it dries below saturation and the bonds re-form in it: the plate mark and the embossed lines. `Moisture::plastic` says whether it is so; the press (P3b) uses it. How much of a deformation is kept is left to P3b and Unknown 5 (IP §9).

## 6. Size against moisture

- **Strain [M → D].** ε = β (min(M, M_fsp) − M_iso) along each direction, where M_iso is the conditioned moisture at which the sheet's size is measured.
  - Water past fiber saturation fills pores and swells no fibers, so a sheet keeps its full wet size while damp and shrinks only once below saturation (IP §7.2).
- **The coefficients, % strain per % MC [M].**
  - **Isotropic handsheets** of hardwood fibers (eucalyptus, birch, acacia, 0.7–1 mm), freely dried, measure **0.064–0.107, mean 0.085**, by OPTIDIM (Pulkkinen, Fiskari & Alopaeus 2009, Appendix A).
  - **Fiber length.** Longer fibers give less: **0.05–0.12** (Uesaka & Moss, in Lindner 2018).
    - The engine runs log-linearly in fiber length from 0.085 at 0.8 mm to 0.05 at 3 mm, constant beyond. The 3 mm for the low end is an estimate [E]: the review gives the trend, not the lengths.
    - At the default 2 mm: **0.059**.
  - **Machine-made sheets** expand **3–5 times more across the machine direction (CD) than along it (MD)** [M]:
    - Uesaka, as quoted in a drying chapter: 0.140 against 0.036 %/%RH.
    - "Bridging network properties to the effective hygro-expansivity of paper" (2015): 0.210 against 0.048 %/%MC.
    - Up to 7 times in Lindner's review.
    - Mill papers' CD measures 0.1–0.15 (Kajanto & Niskanen 1996, in Pulkkinen 2009).
    - The engine takes **4**, about the isotropic geometric mean (Pulkkinen's dynamic sheets keep their mean near the isotropic value). At 2 mm that gives MD **0.029** and CD **0.118**, inside the mill range.
  - **A laid sheet** (made on a mould, by hand or cylinder) or Japan paper (long kozo fibers, handmade) is taken as isotropic. A mould sheet has little fiber alignment (IP §7.2: no measurement found) [D].
  - The machine direction is taken to run down the sheet (its height). That is a convention, since a sheet doesn't say which way it was cut from the web.
- **Check [M].** Soaked through, the default machine sheet grows 0.118 × (0.292 − 0.078) = **2.52 %** across. Cross-direction wet expansion of machine papers measures **2.4–2.75 %** (Lindner, after Figueiredo; IP §7.2). Freely dried sheets give 0.9–1.8 % between 33 and 84 % RH.
- **What follows (P6).** A sheet that dries below saturation between plates is out by β·ΔM·L (IP §7.2). `damp_of(s).width` and `height` give its size now.

## 7. What is left out

- **Per-cell moisture.** The plan's `DAMP` v1 is "MC per cell". Every verb here (soak, blot, pack, drying from a face) acts on the whole sheet alike, so the state is the whole sheet's.
  - #322's per-cell water still holds what a wash sank. Its sinking now finds the damp sheet's free water already in the pores and takes in only what room is left (`dry_pastel_water`).
  - The section is versioned, so a per-cell v2 can come with the press or the counterproof, where press-in and transfer are local.
- **Sorption hysteresis** and the irreversible shrinkage of a first drying.
- **Stretch per pass** under the roller (IP §7.2, [E] 0.05–0.3 %): P6.
- **Wetting a drawn or painted sheet** (a counterproof's damp sheet, gap 5) is refused for now: what water does to pastel or ink on the sheet in the tray isn't modelled.
- **Blotters re-used damp, sheets of different sizes in one pack** (the pack shares water per area, as if the sheets lay over one another), and the pack's equalisation time (§3).

## 8. Estimates and choices still in it

| what | value | why it is one |
|---|---|---|
| GB003's grammage | 330 g/m² | not published; ≤ 364 from its shipping weight |
| blotter fiber | the engine's (1.07 g/cm³, 6 µm) | no linters' dimensions to hand |
| fiber length of the low β end | 3 mm | the review gives the trend, not the lengths |
| CD/MD ratio | 4 | the middle of the measured 3–5 |
| MD down the sheet | convention | a sheet doesn't say which way it was cut |
| drain 30 s, blot 20 s a pass, mist 10 s | gestures | the printer's, not physics |
| a pack equalises fully | — | no measured equalisation time; its scale is minutes against a night |

## Sources

- Parker, M. E., Bronlund, J. E. & Mawson, A. J. (2006). Moisture sorption isotherms for paper and paperboard in food chain conditions. *Packag. Technol. Sci.* 19:193–209, doi:10.1002/pts.719 (GAB fits table; via https://www.researchgate.net/publication/227933833 and https://www.academia.edu/8608357, search summaries).
- Pulkkinen, I., Fiskari, J. & Alopaeus, V. (2009). The effect of hardwood fiber morphology on the hygroexpansivity of paper. *BioResources* 4(1):126–141, https://bioresources.cnr.ncsu.edu/BioRes_04/BioRes_04_1_0126_Pulkkinen_HW_Fiber_Morphol_Hygroexpan.pdf (read: Appendix A, the Kajanto & Niskanen 1996 range).
- Lindner, M. (2018). Factors affecting the hygroexpansion of paper. *J. Mater. Sci.* 53:1–26, https://link.springer.com/article/10.1007/s10853-017-1358-1 (Uesaka & Moss's 0.05–0.12; CD up to 7× MD; via IP and search summaries).
- "Bridging network properties to the effective hygro-expansivity of paper: experiments and modelling" (2015), https://www.researchgate.net/publication/276934574 (β 0.048 MD, 0.210 CD; search summary).
- Uesaka, T., as quoted in "Paper drying in the manufacturing process", https://www.researchgate.net/publication/331998171 (0.036 MD, 0.140 CD %/%RH; search summary).
- Linvill, E. & Östlund, S. (2014). The combined effects of moisture and temperature on the mechanical response of paper. *Exp. Mech.* 54:1329–1341, https://kth.diva-portal.org/smash/get/diva2:756953/FULLTEXT01 (read: Nissan's form, valid above 5 %).
- Caulfield, D. F. (1990). Effect of moisture and temperature on the mechanical properties of paper. In *Paper Physics* (Perkins, Mark, Thorpe, eds.), Syracuse, pp. 50–62, https://www.fpl.fs.usda.gov/documnts/pdf1990/caulf90a.pdf (the 7 % per 1 % via a search summary; the PDF returned 502).
- Zauscher, S., Caulfield, D. F. & Nissan, A. H. (1996, 1997). The influence of water on the elastic modulus of paper, Parts 1 and 2. *Tappi J.* 79(12):178–182; 80(1):214–223 (abstracts).
- Sampson, W. W. (2001). The structural characterisation of fibre networks in papermaking processes. 12th FRS, Oxford, https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/03/2001.2.1205.pdf (read: Corte & Lloyd's pore radii, eq. 95–97).
- Niskanen & Rajatora (2002); Dodson (2001); Kananen, Rajatora & Niskanen (2001): as cited in notes/research/paper_mechanics_and_transport.md §1–2.
- Jeffreys, H. (1930). The draining of a vertical plate. *Proc. Camb. Phil. Soc.* 26:204–205.
- Whatman/Cytiva GB003 blotting paper: https://www.sigmaaldrich.com/US/en/product/aldrich/wha10427818 , https://www.dutscher.com/article/034826 (0.8 mm, cotton linters; the pack's shipping weight).
- The damp pack's recipe, practitioners' soaking and blotting: IP §6.2 and its sources (briarpress.org/31251, handprinted.co.uk, nontoxic-print.com).
- The studio and the evaporation law: notes/research/wet_pastel.md §4 and its sources.
