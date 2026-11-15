# Pastel with water

The numbers behind crates/paint/src/wet_pastel.rs (the wet pastel switch, `--@ wet-pastel`). Tags: [M] measured, [P] a practitioner's, museum's or historian's account, [V] a vendor's statement, [E] an estimate made here. **Almost nothing here is a measurement of pastel itself.** I found no study that measures what water does to pastel optically or mechanically, so the model's constants are estimates built from the neighbouring measurements below.

## 1. What a stick is made of: gum and fillers

- A pastel stick is powdered pigment with a filler (chalk or whiting, kaolin, talc, sometimes gypsum) and a weak water-soluble binder: gum tragacanth, gum arabic or, from the 20th century, methyl cellulose [M] (notes/research/pastel_stick_tribology.md §1, after WebExhibits and Wikipedia "Pastel").
- Recipes dough the powder with about 0.3–0.5 mL of a 3.3 % gum solution per gram. The dry stick then holds **about 1–2 % binder by weight** [D, there], or a few percent by volume (notes/research/dry_pigment_optics.md: "gum ~1–5 % by volume, n ≈ 1.52").
- **The binder dissolves in water again.** Wetting a stick, or wetting laid pastel, takes its gum back into solution. Dried again, the gum is left at the contacts between particles. This is why wet pastel dries "partly bound" (§3). The size of the effect has not been measured: `GUM_BIND` = 0.5, the share of the loose pastel bound by one wetting, is an estimate [E].
- Porosity of a soft stick: not measured in what I found. I use 0.3–0.5 (`TIP_POROSITY` 0.4) [E], from compacted chalk and kaolin powders.

## 2. Optics: why wet pastel is darker

- Dry pastel looks pale because its particles scatter strongly against air: index contrast is high (pigments n ≈ 1.5–2.7; chalk ≈ 1.49/1.66, kaolin ≈ 1.56, gypsum ≈ 1.52) (dry_pigment_optics.md §2, tables). That note puts chalk's scattering in oil at about S_dry/12, kaolin's at about S_dry/45 and gypsum's at about S_dry/140.
- Water (n = 1.333) fills the same pores. The relative index then falls between air's and oil's (n ≈ 1.48): for chalk m ≈ 1.6 in air, 1.2 in water, 1.08 in oil.
- The engine's fixative already takes scattering to about a fifth for particles of n ≈ 2 in resin of n ≈ 1.5 (pastel.rs `fix_pastel`). Scale that as (m − 1)^p, with p fitted so that resin gives 1/5 (p ≈ 1.5). Water then leaves particles of n ≈ 2 about 0.36 of their dry scattering, and chalk about 0.2.
  - **`WATER_S` = 0.3** [E]: a fully wetted layer scatters 0.3 as much as dry. The darkening scales with the share of its pores the water fills (`VOID` = 0.45 of a powder layer's volume, random packing 0.4–0.6 [E]).
- This is the same "dry shift" watercolourists describe: "Wet paint looks dark and intense, and it dulls and lightens as it dries", complete when the sheet is dry (watercolor_physics.md §2, after Handprint [P]).
- **Dried again, it is not as it was.** Practitioners describe wet-worked pastel as more saturated and denser than dry (Degas's "creamy, saturated" line, §6). Two plausible causes: the fines and the redissolved gum are drawn into the contacts as the water leaves, and the paste packs closer than loose dry powder. **`DRIED_S` = 0.8** [E] of dry scattering: darker than dry, much lighter than wet. For comparison, fixative's light spray (wetting 0.15) leaves 0.88.
- A paste also covers more of the paper for its volume: it flows into the tooth instead of resting on the tops. **`PASTE_THIN` = 0.5** [E] of a dry deposit's particle thickness.
- Paper itself darkens when wet. That is not modelled (out of scope): only the pastel darkens.

## 3. Mechanics: the wet stick, the paste

- **Softer and wearing faster.** Wet chalk and clay compacts lose most of their strength when wet. That is common knowledge in soil and ceramic practice; I found no measurement for pastel. Estimates [E]:
  - `WET_SOFT` = 0.4: a soaked tip's hardness is 0.4 of the dry stick's, so it sinks further into the tooth and touches more.
  - `WET_WEAR` = 2.5: its Archard wear coefficient is 2.5 times the dry one.
  - Together these make a wet tip lay roughly twice as much at the same force: 2.3 mm³ against 1.0 in the easel test.
- **Fewer crumbs.** The paste holds together: the share shed as crumbs (`SHED`, about 0.85 dry) falls by `WET_SHED` = 0.6 at full wetness [E].
- **The water it lays.** The paste carries water at the soaked stick's pore ratio, φ/(1 − φ) ≈ 0.7 of its volume (`PASTE_WATER`).
- **The tip.** A dip soaks about 1 mm of the tip (`DIP_MM`) [E]. Its water spreads up the stick and dries off it with an e-fold of 2 minutes (`TIP_DRY_MIN`) [E]: a dipped stick draws wet for a few strokes and a minute or two.
- **Dry again, partly bound.** The finger and the dry tools move only loose pastel. A stick files bound pastel as a crust (pastel.rs `CRUST_WEAR`). So binding half of it (`GUM_BIND`) makes the passage resist the finger less than fixed pastel and more than dry pastel, and take new pastel more like a fixed crust. The brief asked for exactly this; the size of the effect is an estimate.
- **"A dampened stick can be drawn over earlier layers without lifting them"** [P] (notes/degas-materials.md §5, N-A 618). This agrees with a softer stick that lays more and sheds less.

## 4. Water on the paper: how fast it goes

- **Stages.** Handprint times a generous wash on level paper in studio conditions [P] (watercolor_physics.md §2):
  - shiny at 2 min
  - satin at 4 min
  - moist at 7 min ("looks just dried but feels cool")
  - damp at 10 min
  - dry at 20 min
  Drying depends on the amount of water, the paper's absorbency, temperature and humidity.
- **Evaporation.** I found no measured value for still indoor air. Standard engineering correlations at 20 °C and 50 % RH give about 50–185 g/m²·h, i.e. 0.8–3 µm of water a minute (ASHRAE-style pool correlations, US patent 9,114,355; TenWolde 2007, ORNL buildings conference). These are my own evaluations of the formulas, not values the sources give.
  - **`EVAP_UM_MIN` = 2.5** [E]. At that rate a brushed wash of about 30–50 µm left on the surface dries in 12–20 minutes, as Handprint's stages have it.
- **Sinking.** A slack-sized sheet takes water in quickly and "dries faster"; a hard-sized sheet holds it on the surface (watercolor_physics.md §1, after Lundgren 2025 [P]).
  - The engine reuses the paper's `absorbent` (0..1, until now oil-only): surface water sinks with an e-fold of `SINK_MIN` / absorbent, where **`SINK_MIN` = 1.5 min** at absorbent 1 [E].
  - On the default drawing paper (0.8), a 30 µm wash is gone from the surface in about 4 minutes. On a sized sheet (absorbent 0.1) it takes about 10.
  - The sheet's own capacity, and the water left inside it, are not modelled. Nor is cockling.
- **How much a brush lays.** `WASH_UM` = 30 µm in one pass at full load and middling pressure [E]. That is a brushed film, not Handprint's flooded 1.5–3 mm.
- **How much a brush holds.** `BRUSH_WATER_MM3` = 0.9 mm³ per mm² of width squared [E]: about 0.1 mL for a 10 mm brush.
- **Paste on the brush.** At most `BRUSH_PASTE_SHARE` = 0.25 of its water's volume [E].
- **How the paste moves.** A wet brush picks up `PICK_WET` = 0.5 of the wetted loose pastel under it in one pass [E] (a fingertip takes 0.22 of dry pastel, measured: pastel_blending.md §3.1). It lays what it carries within an e-fold of `CARRY_MM` = 12 mm [E].

## 5. The three gestures, as the engine has them

1. **A dipped stick:** `p:wet(share)`, then `p:stroke`. The deposit is wet: dark while wet, partly bound once dry.
2. **A wet brush through laid pastel:** `b:dip(share)`, then `b:wash(pts)`. The loose pastel it wets turns to a paste that moves along with the brush.
3. **Ground pastel with water:** `b:dip(share, {pastel=p, amount=})`, then `b:wash`. The brush holds a paste of the stick's colour; a little pastel in much water is a wash, much pastel in little water a paste. The stick shortens by what was ground off it.

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
- The sheet's water capacity and slower drying from inside the sheet.
- Steam and moisture vapour (gap 6).
- A finger in wet paste: it moves the loose pastel as it does dry pastel.
- How much gum a repeated wetting can set: each drying binds `GUM_BIND` of what is still loose, with no limit set by the stick's gum content.

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
