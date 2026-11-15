# Charcoal: vine (willow) and compressed

Research notes for the easel's `charcoal()` (paint's `charcoal.rs`, and the carbon beds in `dry.rs`). Compiled 2026-11-15 for gap 11 of the Cassatt support plan (notes/cassatt_support.md on branch `notes/cassatt-materials`). Cassatt's preparatory drawings for her prints are "compressed charcoal and graphite over some charcoal lines" (Met, *Maternal Caress* drawing; cassatt_materials.md §4).

**Tags:** **[M]** measured, cited. **[P]** practitioner or museum description, cited, no number. **[D]** derived here from measured inputs (the arithmetic is in `charcoal_optics.py`, this folder: `uv run notes/research/charcoal_optics.py`). **[E]** estimate, with the reason no measurement or physics sets it.

**Bottom line**
- Both kinds are carbon in the tooth, held by nothing but the paper's texture: vine has no binder, and compressed charcoal's gum is glassy at room humidity, so it bridges nothing to the sheet. In the tooth charcoal behaves like a black pastel: loose particles that the finger, the eraser, a puff and a dry brush move, and that fixative binds. So the easel models it with the pastel stick (`pastel.rs`) and gives the stick one new dial, how readily the eraser takes what it lays (`lift`).
- **Color [D].** Carbon absorbs within 0.1 µm, so all a bed of charcoal returns is what its facets reflect (about 9 % against air). Mie in air, from the measured complex index of cellulose char (Jäger et al. 1998), gives vine R∞ 0.021, neutral, whatever the particles' size above 1 µm. Compressed charcoal's powder, packed with its binder into a compact whose pores are finer than light, is an effective medium of index nearer air's: R∞ 0.010.
- **Fixative [D].** The resin wets a share of the pores and halves what the facets reflect there, but carbon's scattering only falls ×1.0–1.6 (a pigment's ×5). A light spray changes the near-black by ΔL* ≈ 0.15, and a full soak by about 1. The small absolute change the engine shows is right. What makes a fixed drawing look deeper is the wetting of a bloom of loose fine dust (greyer, R∞ 0.03) and of the paper between the marks, neither of which the engine models.
- **Lift [D].** A kneaded eraser, soft enough to wrap a particle, out-pulls the paper ×11 on a 3 µm vine splinter (lift 0.98). A particle below the paper's asperity scale (R ≲ 0.3 µm) holds as a smooth sphere, ×1.3 harder than the eraser pulls, so it is taken 0.42 of the time: compressed charcoal's fine particles. Pastel's 0.55 doesn't follow from this; adhesion would give its 1–20 µm particles 0.88–1.
- **Grip [D]: none.** The dial is gone: gum arabic is 18–40 K below its glass transition at room humidity, and a dry binder doesn't flow at the stick's contact pressure.
- **Muddying [D].** The engine mixes the loose layer's reflectances by volume. Kubelka–Munk mixing (K and S by volume) makes pastel over loose charcoal 1.5–2.5× muddier at a few percent of charcoal. The engine's 0.2370 vs 0.2351 understates it; see §6.

---

## 1. What it is

- **Vine and willow charcoal** [P]. These are twigs of willow or vine "heated at a high temperature in an enclosed vessel without oxygen"; the stick has no binding agent, its "minute, splinter-like particles" crumble onto the support, "the medium is easily manipulated", it "requires paper with some texture" to hold, and "to prevent smudging many artists apply a fixative" (Met, Drawing: Charcoal). "The sticks don't contain any binder, so they will be dusty and brittle in texture" (Fine Art Tutorials). Vine "comes in shades of gray"; willow is "darker in color than vine charcoal" (Wikipedia, Charcoal (art)). The easel treats willow as vine (`kind="willow"` is accepted as vine).
- **Compressed charcoal** [P]. "Held together by a gum or wax binder"; "intensity of the shade is determined by hardness": softer grades have less binder and are blacker (Wikipedia; Fine Art Tutorials). One historical recipe is burnt birch, clay, lamp black and a little ultramarine (Wikipedia), so it can be bluish ("blue-black" PITT, Fine Art Tutorials). It is "usually harder in texture than willow and vine charcoal" (Fine Art Tutorials).
- **Stick sizes** [P]. Vine and willow come as "long, thin sticks" in several thicknesses, up to "tree sticks"; compressed as sticks and blocks (Fine Art Tutorials). No source gives diameters. Easel defaults [E]: vine round Ø6 mm and 150 mm long (2–16 mm allowed); compressed square 7 mm and 70 mm long (3–15 mm).

## 2. Optics

- **Charcoal powder reflects under 5 % in the visible**, flat, rising into the near infrared [M] (cometary-dust analogue study, arXiv 2110.05832: "low reflectivity, which remains below 5% in the visible domain"). Charcoal reflectance under oil (%Ro, 0.06–4 %) is a different, specular quantity from polished sections, and isn't used.
- **Carbon's index [M].** Cellulose pyrolysed at 400, 600, 800 and 1000 °C, complex index from reflectance spectra (Jäger, Mutschke & Henning 1998, *A&A* 332:291; tables from the Jena OCDB, `data/carbon/cel400.lnk` etc.). At 550 nm: cel400 1.52 + 0.47i, cel600 1.22 + 0.78i. Their densities (1.44, 1.67 g/cm³) are char's own, so the index already holds its sub-micrometre pores. Charcoal is fired to "around 500 °C" (FAO 1987, *Simple technologies for charcoal making*, ch. 4; 450–500 °C balances friability against fixed carbon). The script takes the mean of cel400 and cel600: 1.43 + 0.61i (610 nm), 1.35 + 0.63i (550), 1.21 + 0.63i (460). Lamp black (soot): 1.95 + 0.79i (Bond & Bergstrom 2006, *Aerosol Sci. Tech.* 40:27).
- **Vine's R∞ [D].** The dry-pigment recipe (dry_pigment_optics.md §2.1, its Mie code): lognormal sizes, transport scattering, K = 2σa, S = 0.75σs(1−g), R∞ by Kubelka–Munk, per channel. Carbon's absorption length (λ/4πk ≈ 0.07 µm) is far below any splinter, so each particle is opaque and returns only its facet reflection: (n−1)²+k² over (n+1)²+k², 0.087 at normal incidence against air. R∞ is therefore almost independent of size: 0.022 at 1 µm, **0.021 at 3 µm**, 0.022 at 20 µm. The splinters are taken as charred cell wall, median 3 µm [E: willow fibre walls are a few µm thick; the result barely depends on it]. Fired at 400 °C instead, 0.018; at 600 °C, 0.028 (more graphitic, more reflective). The engine shows a matte surface with a 4 % first-surface veil (`canvas::haze`), so vine is seen at Y 0.060, L* 29.5. That is slightly above the measured powder's 5 %, because for a bare carbon bed the veil double-counts the facets the Mie already holds.
- **Compressed's R∞ [D].** Charcoal powder pressed with a binder. Gum binder 5 % of the solids by volume [M range: organic binders 2–8 % by weight in pressed charcoal, US 5,221,290; 2.5 % CMC in a black chalk, US 2021/0292589; gum and char densities are alike]. Solids 60 % of the compact [E: a pressed powder compact is 50–70 %; no measurement of a charcoal stick found]. The compact's pores are finer than light, so its crumbs are an effective medium (Bruggeman) of carbon, gum (n 1.47) and air: 1.22 + 0.34i at 550 nm. Its facets reflect less, so R∞ = **0.010**, seen L* 26.6. Sensitivity: solids 50–70 % gives 0.008–0.012; binder 2–10 %, 0.010–0.0095; half the carbon as lamp black, 0.013. The blue tint ("blue-black" PITT) isn't in the physics: carbon is neutral, and the ultramarine some recipes add is unmeasured, so the old (0.022, 0.022, 0.026) is gone. Clay (kaolin) taken into the effective medium darkens it (0.008 at 20 %), but real clay platelets of 0.2–2 µm scatter as particles and grey it ("more binder, harder and lighter", German Wikipedia *Zeichenkohle*). The effective medium can't show that, so the stick is gum-bound.
- **Grey versus black.** Vine reads grey on paper mostly because its marks are lighter and broken, so paper shows between them [P: "aren't as dark as compressed charcoal"; vine "comes in shades of gray"]. A loose dust of fine char (0.1–0.5 µm) is also greyer than the splinters, R∞ 0.025–0.035 (fine soot: 0.04–0.056). That is the "bloom" a loose drawing has. Not modeled: one R∞ per stick, and the fines' share of a mark is unmeasured.
- **Under fixative [D].** "The fixative fills the air gaps between particles", making "light grey values appear slightly darker"; heavy spraying darkens more [P] (Fine Art Tutorials, fixatives guide; ArtParts Fine Arts; AIC Paper Conservation Catalog chs. 3 and 23, "saturation changes").
  - *Share filled.* Krylon Workable Fixatif's SDS (2017, rev. 5) lists, by weight, acetone 46.5 %, toluene 20 %, propane 12.75 %, butane 12.25 % and ethyl 3-ethoxypropionate 3.25 %. The 5.25 % left is the resin (and any additive). With acrylic at ~1.15 g/cm³, the liquid that lands is **5.1 % resin by volume**: what dries in a pore the solution filled.
  - *Pastel's 0.15* (`fix_pastel`'s light spray) is 3× that. Solvent flashed off in flight and resin carried to the drying top by the evaporating solvent would both enrich it, but neither is measured. It stays as pastel's, inside the physical range (0.05 in place, up to 1 for a soak) [E].
  - *Darkening.* Mie with the pores filled by n 1.5: the facets reflect 0.041–0.061 against resin, against 0.082–0.088 against air. Carbon's S falls ×1.59, 1.37, 1.03 (R, G, B); its K barely moves. Vine R∞ (G) goes 0.0211 → 0.0209 (wet 0.05), 0.0205 (0.15), 0.0186 (0.6), 0.0168 (1.0). Seen, L* 29.5 → 29.3 at 0.15 and 28.4 at full wetting.
  - *The engine* binds by pastel's rule (scattering ÷ (1 − w + w/5)), which for this carbon gives 0.0187 at w 0.15, L* 28.9. That is a little more than the physics. The cell can't tell carbon from a pastel without new state, and the difference is under ΔL* 0.5, so it stays.
  - *So for a near-black a small absolute change is right.* Its reflectance is facet reflection plus the first-surface veil, and resin only roughly halves the first. The deeper black and the lost grey of a fixed charcoal drawing are the fine-dust bloom and the paper wetting. The engine's veil stays matte under a light spray, which is right: the resin's menisci in the pores face every way, so their 4 % stays diffuse. A gloss would need a continuous film, and a workable fixative is made not to leave one. Not modeled.

## 3. Adhesion: what holds it

- **Vine: only the tooth** [P]. The particles are "relatively large" and there is no binder (Met). "Large, loosely adhering particles" sit on the surface and are "extremely prone to smudging and lifting" (artist guides, search summary).
- **Compressed: the binder holds the crumbs, not the sheet [D].** The binder "creates adhesion beyond mechanical trapping" (Fine Art Tutorials) [P], but a dry binder can only bridge contacts it was wet at, those inside the stick. Gum arabic's glass transition is 62 °C at aw 0.33 and 42.6 °C at aw 0.54, and it is sticky only near Tg [M: Righetto & Netto 2005, *Int. J. Food Prop.* 8:337]. At 20–25 °C and ordinary humidity it is 18–40 K below Tg: glassy. The stick's contact pressure is its hardness, about 5 MPa, while binder bonds in pressed powders form at the tens to hundreds of MPa of tablet compaction [P: pharmaceutics practice, no single source here]. So the deposit is all loose, as Fine Art Tutorials has it: "just like natural charcoal, compressed charcoal is smudgeable and dusty". The `grip` dial (a share bound as it lands) is removed. A wax binder, which flows under pressure, or a damp room where gum's Tg falls to room temperature (above about aw 0.7, extrapolated [E]) would bring it back, with their own physics.
- **Particle size** [P, uncertain]. The Met calls vine's particles "relatively large" and "splinter-like". One unsourced blog gives vine fragments "from several hundred microns down to fine dust", and compressed charcoal as fine particles packed densely. A DTIC report on carbon-loaded paper found coarser carbon particles "came out of the paper very easily" [M, qualitative]. Size sets the lift (§4). R∞ barely depends on it (§2). The deposit's coverage law keeps pastel's particle thickness (6 µm).

## 4. Erasing

- **Order** [P]. Vine is the easiest dry medium to erase: "easy to erase", "soft, liftable properties" (Fine Art Tutorials); it lifts "off easily by dusting or erasing" (Wikipedia). Compressed is "more difficult to erase" (Fine Art Tutorials).
- **Physics [D].** A particle goes with the eraser if the eraser pulls harder than the paper holds. Hamaker constants (10⁻²⁰ J):
  - cellulose 5.8 (Bergström et al. 1999);
  - wood char taken as activated carbon, 5.0–7.8 (adsorption data, via a Chem. Eng. Sci. review; graphite is 24 ± 13, Gallego-Fuente et al. 2026): 6.4 [E within the range];
  - the eraser's rubber or vinyl putty as a hydrocarbon polymer, 6.5 [E: non-metallic solids are 5–10].

  The forces:
  - *The eraser.* It is soft (E ~0.1 MPa [E]; W/E ≈ 0.6 µm, above the particles' roughness), so it wraps a particle: JKR pull-off 1.5πWR, with W = A/12πD₀² = 63 mJ/m² (D₀ 0.165 nm).
  - *The paper, coarse particles.* A rigid, rough particle above the asperity scale holds 10–150 nN whatever its size [M: irregular 40–65 µm grains 50–150 nN, Nagaashi et al. 2021; 20 µm spheres on rough glass, Moutinho et al.; pastel_brushing.md §1]. Take 39 nN, a decade wide (lognormal, ±1σ spanning ×10).
  - *The paper, fine particles.* Below the asperity scale (0.14–0.5 µm effective contact radius, Nagaashi) a particle sits on one smooth asperity: DMT, A·R/6D₀².

  | R | eraser | paper | taken |
  |---|---|---|---|
  | 0.1–0.2 µm | 30–59 nN | 37–75 nN | 0.42 |
  | 0.5 µm | 148 nN | 39 nN | 0.88 |
  | 1.5 µm | 444 nN | 39 nN | 0.98 |
  | ≥ 3 µm | ≥ 0.9 µN | 39 nN | 1.00 |
- **Easel values.** The kneaded eraser's take per pass is `strength × lift × reach` (graphite.rs `erase`).
  - Vine **lift 0.98** [D]: splinters of R 1.5 µm.
  - Compressed **lift 0.42** [D/E]: its deposit taken as all fine particles, the physical floor. Whole crumbs would lift as vine. The share of fines in a mark is unmeasured, and the floor is the end the practitioners' "more difficult to erase" points to.
  - The old 0.35 is below anything adhesion allows.
  - **Pastel's 0.55** (`PASTEL_LIFT`, unchanged so old logs replay) doesn't follow from adhesion: its 1–20 µm chalk and pigment would be taken 0.88–1. Holding 45 % back would need that share of its coverage in sub-0.3 µm grains, or held deeper than the eraser reaches (the engine's `reach` term already takes the hollows). It stays an estimate.
  - Charcoal that the finger has smeared is laid down again at pastel's lift (0.55) [E, pastel's]: smudged charcoal is pressed into the tooth [P: common practitioner advice].
- **Graphite** keeps its 0.9 (graphite.rs). Black chalk keeps its 0.72.

## 5. Smearing and dust

- The finger, a stump or a chamois blends charcoal (Wikipedia). In the easel the finger picks up 22 % of the loose deposit per pass (pastel_blending.md §3.1, measured for a dry residue). Both charcoals lie loose (§3), so the finger moves them about alike (test: share smeared, vine 0.44, compressed 0.39). Graphite isn't moved by the finger at all in the easel (its smearing isn't modeled).
- A puff and a tap take loose crumbs (pastel_removal_air_blade.md). Vine is "dusty" (Fine Art Tutorials), and the conservation catalog calls charcoal's "extreme friability" the problem with treating it (PCC ch. 3).

## 6. Over and under other media

- **Paint over charcoal** seals it, as with every drawing (graphite.rs).
- **Pastel over unfixed charcoal** [P]. Charcoal dust muddies pastel laid over it. In the easel, pastel laid on loose charcoal mixes with it in the tooth's loose layer (`Cell::add_pastel`), so it comes out greyer; over fixed charcoal it lies on the bound crust and keeps its color.
- **Is the muddying right? [D]: too weak.** `add_pastel` averages the loose layer's reflectances by volume. Particles mixed in a powder mix by Kubelka–Munk instead: K and S add by volume, and carbon's K/S is ~23 against a yellow pastel's ~0.13 (in green). The script's example is a yellow pastel (0.75, 0.6, 0.1) whose S is chalk filler's in air (0.55 µm⁻¹) into vine charcoal:

  | charcoal by volume | the engine's mixing | Kubelka–Munk |
  |---|---|---|
  | 2 % | (0.735, 0.588, 0.098) | (0.704, 0.578, 0.100) |
  | 5 % | (0.714, 0.571, 0.096) | (0.649, 0.549, 0.099) |
  | 10 % | (0.677, 0.542, 0.092) | (0.578, 0.504, 0.098) |

  The loss from the pastel's own color is 1.5–2.5× larger by KM in red and green. In blue, where the yellow already absorbs, it is about none. The test's small margin (0.2370 over loose against 0.2351 over fixed) comes from this and from the thin charcoal line's small share of the volume. Fixing it means KM mixing in `add_pastel`, which every pastel shares. Behind a switch it is its own change, not this one's.
- **Partial fixing.** "A drawing may be only partially fixed, the lower layer being fixed before the last application of charcoal" (PCC ch. 3). The easel does this with `fix(mask)` or a fix between passes.

## 7. Deposit [E]

Not measured. The pastel stick's Archard law lays K·F·s/σp. The easel uses vine σp 4 MPa, K 0.05, and compressed σp 5 MPa ("harder in texture") with K 0.07 (a friable compact). Compressed therefore lays slightly more volume at a given force. These stay estimates: no hardness or wear rate of a charcoal stick was found. Both are inside pastel.rs's ranges (σp 2–30 MPa, K 0.01–0.06; compressed's K just above, as more friable than a pastel). The stick sizes (vine Ø6 mm × 150, compressed 7 mm × 70) are estimates too [§1: no source gives diameters].

## Estimates left, and why

| Constant | Value | Why it is an estimate |
|---|---|---|
| firing temperature | 500 °C | FAO's commercial charcoal; no art maker states its kiln temperature (Coates: "gradual rises in temperature", no figure) |
| splinter size (vine) | 3 µm | no size distribution of a drawn mark found; R∞ changes < 5 % over 1–20 µm |
| compressed solids, binder | 60 %, 5 % gum | no recipe with numbers for an art stick found; binder from the nearest patents, solids from pressed compacts |
| fixative wet share | 0.15 (pastel's) | the SDS sets 0.05 in place; the enrichment by solvent flashing and evaporative migration is unmeasured |
| Hamaker, eraser modulus | 6.4, 6.5 ×10⁻²⁰ J; 0.1 MPa | within measured ranges for carbon and polymers; the eraser's own not measured |
| compressed's fine share | all fines | the share of a mark in fine particles is unmeasured; the floor is taken |
| σp, K, stick sizes | §7 | no measurement of a charcoal stick |
| smeared charcoal's lift | pastel's 0.55 | pastel's own estimate |

## Sources

- Jäger, Mutschke & Henning 1998, "Optical properties of carbonaceous dust analogues", *A&A* 332:291; tables: Jena Database of Optical Constants for Cosmic Dust, www2.astro.uni-jena.de/Laboratory/OCDB/carbon.html (cel400, cel600).
- FAO 1987, *Simple technologies for charcoal making*, Forestry Paper 41, ch. 4, fao.org/4/x5328e/x5328e05.htm.
- Bond & Bergstrom 2006, "Light absorption by carbonaceous particles: an investigative review", *Aerosol Sci. Tech.* 40:27.
- Krylon Workable Fixatif Spray Coating, Safety Data Sheet (Sherwin-Williams, 9/9/2017, version 5), section 3.
- Righetto & Netto 2005, "Effect of encapsulating materials on water sorption, glass transition and stability of juice from immature acerola", *Int. J. Food Prop.* 8:337.
- Gallego-Fuente et al. 2026, "Hamaker constants of van der Waals materials under clean-interface conditions", *Adv. Sci.* (graphite 24 ± 13 ×10⁻²⁰ J); activated carbon 5.0–7.8 ×10⁻²⁰ J as cited in Chem. Eng. Sci. (S0009250901000331).
- Nagaashi et al. 2021, Moutinho et al., Bergström et al. 1999: via pastel_brushing.md §1.
- US 5,221,290 (charcoal briquettes, organic binder 2–8 %); US 2021/0292589 A1 (graphite chalk, CMC 2.5 %).
- German Wikipedia, "Zeichenkohle" (binder and hardness; soot and clay in pressed charcoal).
- The Metropolitan Museum of Art, "Drawing: Charcoal" (materials and techniques), metmuseum.org/about-the-met/collection-areas/drawings-and-prints/materials-and-techniques/drawing/charcoal (quoted via search excerpt; the page itself returned 429).
- Fine Art Tutorials, "The Best Charcoal for Drawing: A Review", finearttutorials.com/guide/best-charcoal-for-drawing/; and "The Best Charcoal and Pastel Fixatives", finearttutorials.com/guide/best-charcoal-and-pastel-fixatives/.
- Wikipedia, "Charcoal (art)", en.wikipedia.org/wiki/Charcoal_(art).
- American Institute for Conservation, Book and Paper Group, *Paper Conservation Catalog*: ch. 3 "Media Problems" (1985) and ch. 23 "Consolidation/Fixing/Facing" (1988), cool.culturalheritage.org/coolaic/sg/bpg/pcc/ (quoted via search excerpts; ch. 23 is a scan).
- NEDCC, Preservation Leaflet 7.5, "Conservation Treatment for Works of Art and Unbound Artifacts on Paper", nedcc.org.
- Cometary dust analogues for physics experiments, arXiv:2110.05832 (charcoal powder reflectance < 5 % in the visible).
- ArtParts Fine Arts, "Fixative Spray for Charcoal: A Professional Guide", artpartsfinearts.com.au.
- Art Discount, "A Beginner's Guide to Artists' Charcoal", artdiscount.co.uk.
- DTIC ADA147673, "Preparation of Paper Containing Activated Carbon".
