# Friedrich materials: readiness for an original engine 5 landscape

**Recommendation: the current 14-tube box is suitable for the original Friedrich brief.** It supplies the principal documented pigment families and the engine supports thin paint, textured layered grounds and precise drawing. This is a judgment of practical suitability, not a claim that its numerical properties reproduce Friedrich's paint. Solvent thinning retains an unreconciled modeling limit: combining `thinner=` and `turps=` in one pile is untested, as detailed below. The brief requests an invented landscape, not a dated replica or every pigment he ever used. [R1; R2; R3; R14 lines 93–110; CATS p.127; ALF pp.344–349; MAD pp.102–103]

**Before painting, correct the materials note's evidence and mapping:** describe its tubes as a selection spanning his career, identify verdigris as a copper-green proxy, mention documented Naples yellow and red lake as unavailable in this box and qualify who applied the egg-white coating. These are proposed documentation changes; this review changes no palette, engine constants or painter instructions. [R4 §§4–5,8–10; R2; CATS p.127; MAD pp.102–103]

## The standard used for Inness

The Inness report distinguishes witness evidence, object-specific analysis, assumptions and unverified claims; its tube mapping explicitly labels lead white assumed. The catalog reconciliation separates source-supported directions from numerical estimates and uses documented use or working materials as the historical inclusion standard. Its preservation rule keeps the original fourteen default tubes unchanged for replay. The final materials-review commit corrected overstatement of oil medium and pigment mappings. That is the standard applied here: verify historical claims, reconcile actual supplied materials and label approximations. No separate standalone Inness review checklist was located; these are the repository's concrete review receipts. [R5 §§4–5,9; R6 lines 12–34,105–120; R7; R8]

**Evidence labels.** “Analytical” means examination of historical paintings. “Inferred” means identification or suitability deduced from that evidence. “Estimate” means a chosen simulator value. No new physical measurements or painter experiments were performed for this review. Catalog numbers explicitly describe approximations, not measurements. [R2 lines 6–9,69–79]

## What the painter actually receives

The Friedrich exporter selects the default tube box; the original opening asks for one original landscape with its place, subject and composition invented. The composition note directs materials questions to the existing Friedrich materials report. The current engine version is 5. [R1; R2 lines 260–282; R3; R9]

The four property columns below are **estimates copied from the current implementation**, not measured historical recipes. Hiding is the input controlling how much one modeled coat conceals its ground; it is calibrated through a gray surrogate, so it is not an independently measured rendered contrast ratio. Stiffness runs from fluid to stiff. Strength weights color mixing. Drying is relative to the model's average, not days. Engine 5 uses the engine-3 tube drying values where provided. [R2 lines 21–53,83–122,495–513; R10]

| Current tube | Hiding / stiffness / strength / drying | Historical fit and limit |
|---|---|---|
| Lead white | 0.82 / 0.80 / 1.00 / 2.00 | Analytically documented in paint and grounds. [CATS p.127; MAD p.103] |
| Smalt | 0.30 / 0.55 / 0.45 / 1.60 | Documented cobalt glass; low hiding suits transparent passages. [NG pp.55–56] |
| Pale smalt | 0.35 / 0.55 / 0.35 / 1.60 | A chosen representation of historically observed varying grades, not a measured particular grade. [NG p.56; R2 lines 86–89] |
| Yellow ochre | 0.80 / 0.70 / 0.80 / 0.80 | Documented yellow earth family. [MAD p.102] |
| Red earth | 0.85 / 0.70 / 0.90 / 1.00 | Proxy for documented red iron-oxide earths. [MAD p.102; R2 line 92] |
| Vermilion | 0.90 / 0.75 / 1.00 / 0.40 | Documented cinnabar, often small details. [MAD p.102] |
| Raw umber | 0.80 / 0.65 / 0.90 / 2.40 | Umber is reported in the Winter Landscape ground; that does not establish one universal brown tube for his whole career. [NG p.55] |
| Bone black | 0.90 / 0.70 / 1.10 / 0.90 | Analytically documented. [MAD p.103] |
| Cobalt blue | 0.55 / 0.60 / 0.80 / 2.20 | Later-period evidence supports it; not interchangeable with early smalt's optical handling. [ALF pp.346–349; NG p.56] |
| Chrome yellow | 0.90 / 0.70 / 1.00 / 1.80 | Identified by inference from elemental data in 1817 and 1818/24 works; those findings do not make it an early universal yellow. [ALF pp.344–349] |
| Prussian blue | 0.35 / 0.45 / 3.00 / 2.40 | Possible in the earlier Winter Landscape mixture; stronger later evidence in Dresden and the 1818/24 painting. [NG p.56; MAD p.103; ALF p.346] |
| Green earth | 0.20 / 0.35 / 0.30 / 0.80 | Documented green family; exact modeled grade and numbers are estimates. [MAD p.103; R2 lines 110–114] |
| Rinmann's green | 0.35 / 0.50 / 0.40 / 1.40 | Cobalt-zinc green identified in individual works; optional use, not a requirement of every Friedrich palette. [MAD p.103] |
| Copper green | 0.25 / 0.40 / 1.00 / 1.60 | Here specifically verdigris. General copper-green evidence and a possible degraded copper-arsenic green do not uniquely identify verdigris. [R2 lines 119–122; MAD p.103] |

**Period and inventory.** The box combines early and later materials. The chronological studies support a shift toward cobalt blue after approximately 1820, with overlap and uncertain painting dates. For this undated original landscape, a coherent selection within the box suffices; claiming that all fourteen form a single documented period palette would overstate the evidence. [ALF pp.342,348–349; R3]

**Optional omissions.** Naples yellow is documented in Winter Landscape and the Dresden study. A red lake is also analytically documented, including alizarin identified in The Great Enclosure. Neither is in the default box. The catalog already has lead-antimonate Naples yellow and rose madder, but they belong to other boxes. These are candidates only if a complete historical inventory or those specific mixtures become a requirement. Changing the global default would exceed this review and conflict with its documented preservation rule. [NG p.56; MAD pp.102–103; R2 lines 125–128,165–169,277–282; R6 lines 19–22]

| Optional candidate | Existing catalog estimates: hiding / stiffness / strength / engine-5 drying | Tradeoff |
|---|---|---|
| Naples yellow, lead antimonate | 0.85 / 0.75 / 0.60 / 1.60 | Adds the documented yellow family, but its exact numerical properties and drying remain estimates. [R2 line 128; R10 lines 126–128; NG p.56] |
| Rose madder, madder lake | 0.10 / 0.35 / 0.90 / 0.40 | Represents the documented red-lake family; exact Friedrich preparation and hue are unestablished. [R2 lines 167–169; R10 lines 195–197; MAD p.102] |

## Paint behavior, ground and tools

**Opacity and mixing — suitable direction, uncalibrated magnitude.** Weak, low-hiding smalt and green earth versus stronger Prussian blue are purposeful model choices. The engine mixes tube colors in strength-weighted Mixbox space and derives absorption/scattering from chosen color and hiding. It does not establish physical spectra, glass-particle sizes or exact historical tinting strength. Pale smalt having slightly more input hiding than ordinary smalt is a grade estimate, not evidence of a historical contradiction or a reason to retune it. [R2 lines 6–9,21–46,83–122,500–513; R11 lines 1–18,30–51; NG p.56]

**Drying — estimates, not Friedrich timing evidence.** Current rates are literature-informed estimates; cobalt blue and Prussian blue have engine-3 overrides, while smalt keeps 1.6. Engine 5 inherits these choices. Walnut oil is available and modeled at 0.8 of the linseed rate, consistent with representing a slower oil; walnut was identified in both paint and ground of Winter Landscape. This single-object result does not establish every Friedrich painting's binder. The code's reference stroke timings and thickness response do not validate real historical drying at every thickness. No drying-rate correction was established by this documentary review. [R10 lines 41–61,97–115,177–197; R2 lines 495–497,568–569; R12 lines 1793–1800; NG p.55]

**Solvent thinning — two different models with a known gap.** `thinner=` remains in the wet film and evaporates over painting time; `turps=` evaporates as paint is laid. Their simultaneous use in one pile is documented as untested. Brush clumping and spatter read only `turps=`: `thinner=` does not reduce clumping through its solvent share. The suitability judgment above therefore leaves combined-solvent behavior and these brush effects physically unverified; it does not authorize a model change. [R14 lines 93–110]

**Ground — visual reconstruction supported, chemical reconstruction incomplete.** Fine plain-weave linen and several knife/roller/brush-applied ground layers are supported. The Berlin pair's red lower layer and two light-brown upper layers, followed by very thin paint gathering in the ground texture, fit the available construction. Their surface texture should not be equated with bare woven cloth. Numerical thread counts or micron thicknesses chosen for a new canvas remain estimates; the materials note's 10–16 threads/cm example is another painter's commercial Dresden cloth, not a Friedrich measurement. [CATS pp.126–127; R4 §1; R13 lines 41–68]

Grounds use tube-color mixtures; the API has no dedicated chalk, barytes or sizing ingredient and no ground oil selector. Its absorbent option represents chalk/glue behavior rather than reproducing the chemistry of a chalk/lead-white oil ground. Thus matching a white or warm ground's appearance is possible, but chemically exact preparation is not. These gaps do not block the original landscape brief. [R12 lines 2119–2154; NG p.55; MAD p.102; R3]

**Drawing and application — available methods fit the evidence.** Pencil grades, black chalk and ruled lines support precise underdrawing. The engine also offers thin brush passages and stippling. A quill/ink drawing material and egg-white coating are not present in the supplied APIs. A generic brush or finishing varnish should not be relabeled as either historical material. The Berlin coating's application by Friedrich himself is explicitly unresolved. [R13 lines 440–457; R12 lines 1994–2074; CATS pp.127–132]

**Aging — distinguish new paint from today's objects.** Smalt degradation changes surviving paintings' color. The 2025 reconstruction provides a possible cooler original appearance, not measured original RGB values for the simulator. Its experimental mock-up oils and driers are not evidence that Friedrich used those recipes. Current tube colors and the drying model contain no smalt-specific potassium-leaching/color-loss mechanism; deliberate aging effects therefore do not authenticate a historical reconstruction. [NPJ Methods and Results; R2 lines 83–89; R10]

## Exact proposed note corrections

1. **The tubes here:** add “This box is a selection spanning Friedrich's career, not one dated palette. Naples yellow and a red lake are documented but are not supplied here.” [R4 §9; R2 lines 277–282; NG p.56; MAD pp.102–103]
2. **Copper-green mapping:** replace the implication that historical copper greens equal this tube with “Copper green here models verdigris; it is a proxy, not a specific identification of Friedrich's copper-containing greens.” [R4 §§8–9; R2 line 122; MAD p.103]
3. **Binding media:** replace the blanket egg-white attribution with “An early egg-white coating was found on the Berlin pair; whether Friedrich or another hand applied it is unknown.” [R4 §5; CATS p.127]
4. **Pigment evidence and citation:** add the Dresden red-lake finding and correct the Mäder proceedings page reference from 101–103 to the article's printed 102–104. Preserve object-specific uncertainty and refrain from converting the copper/arsenic inference into a definite pigment identification. [R4 §§4,8,Sources; MAD pp.102–104]

No composition-brief change is needed: its invented-subject requirement is compatible with this evidence. No essential palette addition or numerical material change is established for that scope. [R3; R2; CATS; ALF; MAD]

## Sources and implementation receipts

- **CATS:** Mösl and Schneider, “Romantic icons,” *CATS Proceedings III*, pp.124–133. Full article checked, especially printed p.127. https://pure.kb.dk/ws/portalfiles/portal/10104814/CATS_proceedings_III.pdf
- **NG:** Leighton, Reeve and Burnstock, *National Gallery Technical Bulletin* 13 (1989), pp.44–60. Museum's full-text RTF checked; sections “Technical examination” and “The painting technique.” https://www.nationalgallery.org.uk/media/15725/leighton_reeve_burnstock1989.rtf
- **ALF:** Alfeld, Mösl and Reiche, *X-Ray Spectrometry* 50 (2021), pp.341–350. Full-text results, Table 2 and discussion checked. https://pure.tudelft.nl/ws/portalfiles/portal/94318269/xrs.3195.pdf
- **MAD:** Mäder et al., “Caspar David Friedrich: Befunde über seine Farbmittel im Gemäldebestand der SKD,” *METALLA* Sonderheft 13 (2025), printed pp.102–104. Full article checked. https://metalla.org/index.php/METALLA/en/issue/download/393/75
- **NPJ:** de Mecquenem et al., *npj Heritage Science* 13, 388 (2025). Full Methods and Results checked. https://pure.tudelft.nl/ws/portalfiles/portal/251868136/s40494-025-01953-y.pdf
- **R1:** [Friedrich export selection](../../scripts/export_r16_studio), line 59.
- **R2:** [Actual tube catalog and palette behavior](../../crates/paint/src/palette.rs).
- **R3:** [Original opening](../round24/runner/r21_chains.py), lines 270–276; [composition brief](../briefs/friedrich_painter.md), lines 1–7.
- **R4:** [Existing Friedrich materials report](friedrich_materials.md).
- **R5:** [Existing Inness materials report](inness_materials.md).
- **R6:** [Catalog reconciliation and inclusion standard](../r20/TUBES.md).
- **R7:** [Materials-review and box reconciliation receipts](../r20/BUILD.md), lines 84–145.
- **R8:** Final materials-review corrections, repository commit `b05effc85552d1d85c525ba7b6937b40e20f5aba`.
- **R9:** [Engine version](../../crates/paint/src/lib.rs), line 93.
- **R10:** [Drying constants and their limits](../../crates/paint/src/drying.rs).
- **R11:** [Absorption/scattering model](../../crates/paint/src/pigment.rs).
- **R12:** [Painter API, oil selection and ground inputs](../../crates/easel/src/api.rs).
- **R13:** [Current materials and tool guide](../easel_guide.md).
- **R14:** [Two solvent-thinning models and their unreconciled limits](../engine-4.md), lines 93–110.
