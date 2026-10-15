# Blending pastel over an area: the finger and the stump

Area smudging of dry soft pastel: its physical basis and numbers, for `paint::pastel` (`Canvas::rub`, `Canvas::rub_area`; notes/pastel.md, "Area smudging").

Compiled 2026-10-15. Text sources only; no images were opened.

Each number is labeled:
- **[M]**: measured in a study
- **[S]**: a standard's or vendor's specified test parameter
- **[P]**: a practitioner's or museum's statement, with no measurement behind it
- **[E]**: an estimate, or arithmetic on the above

"via X" means the number was read in secondary source X, not in the original. Network note: Springer, PMC, nature.com, the Met, the NGA, CAMEO and archive.org were blocked or rate-limited from this machine. Where a primary text could not be read, the item says so.

---

## 0. Bottom line for the simulator (details in §6)

- The single-pass contact law (135 mm² at 1 N, growing as F^0.4) is consistent with measurement at loads up to about 2 N. Above that, measured apparent area nearly stops growing (exponent about 0.14), so cap the growth (§2.1).
- The pressures in finger blending (about 5–20 kPa apparent) are in the same range as standard rub and smear tests (5–45 kPa). So rub-test transfer data are a fair analog (§4).
- For dry particles on a fibrous or porous surface, measured pick-up by a soft, oily, skin-like pad is about 11% of the residue per static contact and about 22% with shear. On a smooth surface it is 39–52%. At high pressures this did not depend on force or dwell time. Sliding roughly doubles transfer (§3.1).
- As the pad loads up, its pick-up falls by about a factor of 3 (§3.1).
- Practice: there are no measured motion studies of pastel blending. Practitioners describe:
  - loose circles with one or two fingers for broad areas;
  - long pulls with the heel of the palm, or the finger, along the form or contours;
  - small circles with the little finger for detail;
  - a second, crossing pass with a stump;
  - cycles of "add pastel, then blend", with warnings against over-blending (§1).

---

## 1. Practice: how pastellists move when blending an area

### 1.1 Historical and technical-art-history sources

| Item | Value | Source | Type |
|---|---|---|---|
| Russell's "sweetening" (stumping) | Blended with "his finger or a stump (a rolled-up piece of paper or cloth)". Method derived from Carriera's sfumato. | Metropolitan Museum, Heilbrunn Timeline essay "The Eighteenth-Century Pastel Portrait", https://www.metmuseum.org/essays/the-eighteenth-century-pastel-portrait (read via search excerpt; page rate-limited). Primary: John Russell, *Elements of Painting with Crayons*, London 1772 (ESTC T101339), 2nd ed. 1777. The full text was not reached (ECCO-TCP returned 403). | [P] secondary |
| Russell's sequence | Outline in chalk, colors laid in, then outlines "smudged using his fingers and crayon" | Yale Center for British Art catalog entry, https://collections.britishart.yale.edu/catalog/tms:15272 (via search excerpt) | [P] |
| 18th-century stumps | Rolled parchment tortillons, used to work pigment across the paper and mix applied pigments | LPOD post reproducing Russell's 1772 tool plate, https://www.lpod.org/wiki/August_2,_2010 (via search excerpt) | [P] |
| Degas, finger | Blended "the central area of pink" of a shoulder with his fingertip. Pressed harder with the stick on a blue collar. | National Gallery of Art, "Pastel, watercolor… Degas" story, https://www.nga.gov/stories/experiment-pastel-watercolor-degas.html (403 here; read via search excerpt) | [P] museum technical observation |
| Degas, stump | Shaded the eyes "by rubbing the pastel with the pointed end of a stump, a rolled piece of leather or paper" | same NGA page | [P] |
| Degas, overall | His vocabulary included "stumped powder" among linear marks and hatching. Juxtaposed color was "far more strident than the blended and fused passages of traditional pastelists". He used fixative locally as a barrier between layers. | Met essay "Precious Powder: Degas", https://www.metmuseum.org/essays/precious-powder-degas (via search excerpt). Boggs & Maheux, *Degas Pastels* (1992), quoted in a Sotheby's catalog: later work was "vigorously hatched, interpenetrating layers". | [P] |
| Met technical overview | "A painterly effect can be achieved by carefully spreading the powder with a finger or a stump" (or a brush). Light strokes leave particles "on the ridges of the paper surface" that "merely drift into the valleys". | Met, "Materials and Techniques: Drawing – Pastel", https://www.metmuseum.org/perspectives/materials-and-techniques-drawing-pastel (via search excerpt) | [P] |
| Tate glossary | Soft pastel "is easily blended… by smudging with a finger, soft cloth or a drawing tool such as a tortillon" | https://www.tate.org.uk/art/art-terms/p/pastel | [P] |
| Mayer | Tortillons are "small stumps of tightly rolled gray or white paper, usually pointed at one end only; used to blend charcoal and pastel". The *Artist's Handbook* text itself was not reached. | Fitzwilliam terminology entry crediting Mayer, https://collection.beta.fitz.ms/id/terminology/term-82355. CAMEO cites Mayer's *Dictionary of Art Terms and Techniques* (CAMEO returned 403). | [P] |

**Gap.** No conservation or technical study measures blending motion: stroke length, speed, number of passes or force. Everything below about motion is practitioners' advice or an estimate.

### 1.2 Practitioners' motion descriptions

| Item | Value | Source | Type |
|---|---|---|---|
| First blend, broad area | "With loose circular movements, rub in the pastel using your index and middle fingers. This first blend can be hard work and if you find areas that won't blend at all add more pastel." | Sandra Orme, "The Perfect Blend", *Paint & Create* (SAA magazine), Jan 2017, https://community.saa.co.uk/?p=432194 (full text read) | [P] |
| Directional pull | "use the base of your palm and pull the pastel along in the direction of the hills, up and across". Later: "long strokes and blend in the direction of the contours". | same | [P] |
| Detail | "From now on, use your little finger to blend using small circular movements" | same | [P] |
| Soft tool | Colour Shaper (silicone tip) used with "gentle pressure to soften… repeating over and over the edges" | same | [P] |
| Threshold behavior | On toothy paper, "put one stroke of pastel on it and then try blending…not a lot happens! … as you add layers it will blend more readily" | same | [P]. Implies blending needs a minimum loose load above what the tooth holds. |
| Pressure and tooth | "Using light pressure on the side of the pastel ensures you won't fill the tooth… the harder you rub the more you will push it into the surface and into the colours underneath" | same | [P] |
| Workflow | Repeated cycles: add pastel, blend, add, blend | same | [P] |
| Stump, circular | "use small circular movements or rub lightly"; "There is no need to press down" | Caran d'Ache, "How to use a blending stump", https://www.carandache.com/ca/en/drawing-how-to-use-blending-stumps-and-pencil-blenders | [P] manufacturer |
| Stump, crossing direction | Pencil strokes "rubbed vertically and then stumps were rubbed horizontally (15–25 secs)". Tortillon "rubbed… with circular gestures and medium to low pressure (20 secs)". | Sousa & Buchanan, "Observational Model of Blenders and Erasers in Computer-Generated Pencil Rendering", Graphics Interface '99, pp. 157–166, Fig. 5 caption, https://graphicsinterface.org/wp-content/uploads/gi1999-21.pdf | [P] observed reference swatches (graphite, not pastel) |
| Stump angle | Holding the stump at an angle increases contact area | Wikipedia "Stump (drawing)", citing *Art of Drawing* (Sterling 2003, pp. 32–35) and Parks & Parks 2009 pp. 36–37 | [P] |
| Chamois | Dragged or wiped across large areas. Wrapped round a finger for small areas. Also used to lift pigment and highlights. | art-is-fun.com; zest-it.com; retailer copy | [P] low authority |
| Over-blending | "the more the layers of pastel are smeared, the duller the final outcome may appear". Finger blending "breaks down that crystalline structure, dulling the surface". | Artists Network, "Basic Pastel Techniques Part One" and "Pastel Q&A: blending colors" (403 here; via search excerpts) | [P] |
| Number of blends | "blend only once or twice per area" | low-quality site, via search | [P] weak; treat as folklore |
| Crossing at junctions | "blend in a perpendicular direction at that area to get a smooth transition" | oil-pastel course excerpt, via search | [P] weak |

---

## 2. Mechanics

### 2.1 Fingertip contact area vs force: checking A ∝ F^0.4

| Item | Value | Source | Type |
|---|---|---|---|
| Finger on **paper**, apparent area, low load | A_app = 107·F^0.42 mm² (F in N, 0.4–1.8 N). Grew from 78 to 130 mm² below 2 N. | Liu X., Carré M.J., Zhang Q., Lu Z., Matcher S.J., Lewis R. (2018), "Measuring contact area in a sliding human finger-pad contact", *Skin Research & Technology*. Accepted manuscript §3.1.1, https://eprints.whiterose.ac.uk/id/eprint/116164/ (full text read). Method: inked right index finger pressed on a white paper sheet at 25–40°, loads 0.5–24.5 N, one subject. | [M] |
| Same, high load | A_app = 130·F^0.14 mm² (3–24 N). Plateau at about 200 mm². | same | [M] |
| Same, real (ridge) area | A_real = 27·F^0.50 (low load), 31·F^0.28 (high load). Rose from 16 to 39 mm² below 2 N and from 40 to 94 mm² at high load. | same | [M] |
| Sliding vs static | Apparent area 7% smaller when sliding (230 → 214 mm² at 6 N, middle finger on glass, 20 mm/s stage) | same, §3.2, Fig. 14 | [M] |
| Ink prints (on paper) | Area 23–194 mm² as contact length went from 6 to 21 mm. Ridges were 38–59% of the outline area (about 50% on average). | Tomlinson S.E. (2009), PhD thesis, Univ. Sheffield, "Understanding the friction between human fingers and contacting surfaces", Table 5.1, https://etheses.whiterose.ac.uk/id/eprint/150/ (full text read) | [M] |
| Plateau load | Contact grows roughly linearly, then plateaus at 2.0–5.5 N, depending on the volunteer (5 volunteers, middle finger on glass, high-speed video). Plateau areas were up to about 160–300 mm² by volunteer (read from figure axes). | same, §5.2.1, Fig. 5.4 | [M] |
| Shape | Contact length/width ratio rose from 1.0–1.5 at small contacts to about 2.0 at 18 mm length (elliptical, elongated along the finger) | same, Table 5.3 | [M] |
| Soneda & Nakano | Apparent area exponent 0.52, real area exponent 0.68, for 0.1–5 N, optical method. Another summary of their data gives 0.478–0.641 (apparent) and 0.583–0.806 (real). | Soneda T., Nakano K. (2010), "Investigation of vibrotactile sensation of human fingerpads by observation of contact zones", *Tribology Int.* 43:210–217. Via Frontiers 2020 (below) and search summaries; primary not reached. | [M] secondary |
| Consensus range | Low-force exponents: 0.36–0.42 (apparent), 0.42–0.58 (ridge). Sources: Warman & Ennos 2009; van Kuilenburg 2013; Lin 2015; Dzidek 2017; Liu 2018. | Forsbach F., Heß M. et al. (2020), "A Fingerpad Contact Model for Electrovibration…", *Front. Mech. Eng.* 6:567386, https://www.frontiersin.org/journals/mechanical-engineering/articles/10.3389/fmech.2020.567386 (full text read). Also used a Dzidek-based ridge fit A_R = 74·F^0.52 mm². | [M] review |
| Other exponents | van Kuilenburg: 0.36. Han et al. 1996 (CCD, acrylic): 0.2–0.4. Xydas & Kao: 0.09 for real fingers. Delhaye 2014: 0.36–0.52 below 2 N. | via Liu 2018 §3.1 and search summaries | [M] secondary |
| Hertz reference | A ∝ F^(2/3) for a smooth elastic sphere. Measured finger exponents are below this because skin stiffens under load (Tomlinson §5.4.1). | Tomlinson 2009, Eq. 5.1 | theory |
| Moisture | Pressing exponents of 0.67 / 0.68 / 0.50 for dry / natural / moist finger (FE model fitted to data) | IEEE ToH 2021, doi 10.1109/toh.2021.3077549 (via search) | [M] secondary |

**Conflicts.**
1. Warman & Ennos are summarized elsewhere as 0.54–0.85 below 2 N, but Forsbach puts the apparent-area consensus at 0.36–0.42. The primary was not checked.
2. Soneda & Nakano (0.52 / 0.68) sit above the 0.36–0.42 consensus. Their load range and optical method differ.
3. One search summary misreported Soneda & Nakano's 0.52 ± 0.06 / 0.68 ± 0.09 as friction coefficients. They are area exponents.

**Check of the simulator law (A = 135·F^0.4) against Liu 2018 on paper [E]:**

| F (N) | Simulator (mm²) | Liu apparent (mm²) | Ratio |
|---|---|---|---|
| 0.5 | 102 | 80 | 1.27 |
| 1 | 135 | 107 | 1.26 |
| 2 | 178 | 143 | 1.24 |
| 5 | 257 | 163 | 1.58 |
| 10 | 339 | 179 | 1.89 |

- The exponent 0.4 matches the measured low-load exponent (0.42, and the 0.36–0.42 consensus).
- The magnitude at 1 N is about 25% above Liu's single subject. That is within the spread between people: Tomlinson's plateau areas reach about 160–300 mm². It is also plausible for blending, which uses a flatter finger than Liu's 25–40°.
- Above about 2 N the simulator over-predicts the area. Measured growth saturates (exponent 0.14). See §6.

### 2.2 Forces people use

| Item | Value | Source | Type |
|---|---|---|---|
| Free tactile exploration | Mean normal force 0.49 N at 86 mm/s (raised targets). 0.64 N at 76.7 mm/s (recessed targets). Force unchanged when friction was raised with sucrose. | Smith A.M., Gosselin G., Houde B. (2002), "Deployment of fingertip forces in tactile exploration", *Exp. Brain Res.* 147:209–218 (via search excerpt of abstract) | [M] |
| Typical exploration force | 0.3 N is "typical of forces observed in prior studies of tactile exploration" | 2018 study quoted via search; *Sci. Rep.* 8:5300-series, "Contact geometry and mechanics predict friction forces during tactile surface exploration" | [M] secondary |
| Speed–force coupling | Lighter forces go with higher speeds (negative correlation) | J Neurophysiol 2015 (monkey/human), via search | [M] secondary |
| Natural pen-tip force | 0.82–3.16 N when drawing or writing (resting 0.78–1.58 N, stylus on screen) | IBM study "Pen pressure control in trajectory-based interaction", via search | [M] secondary; an anchor for stump forces |
| Fingertip friction regime change | At about 2 N | Skedung et al. 2010, via Vilhena 2023 §1 | [M] secondary |
| Painters' blending force | **No measurement found.** Practitioners say "light", "gentle", "no need to press". | §1.2 | [P] |
| Blending force estimate | Finger 0.2–1 N (light) up to about 2 N (firm "rub in"). Stump 0.5–2 N. Brush or Colour Shaper below 0.3 N. | anchored on exploration (0.3–0.64 N) and pen forces (0.8–3.2 N) | [E] |

### 2.3 Finger sliding speeds

| Item | Value | Source | Type |
|---|---|---|---|
| Free exploration | 77–86 mm/s mean | Smith et al. 2002 | [M] |
| Commonly cited range | 33–110 mm/s; "80 to 100 mm/s"; studies pick 40/80/120 mm/s to span natural exploration | various haptics papers, via search | [M] secondary |
| Skin-friction literature | Tests run at 0.5–135 mm/s. Liu chose 20 mm/s "to simulate the movement of real finger". Volunteers slid at 10–28 mm/s when asked to go steadily. | Liu 2018 §2.2 | [S]/[M] |
| Hand rubbing of topical products | Final rubbing speed 10–100 mm/s | arXiv 2201.10008 (hand-rub rheology) | [M]/review |
| Circular fingertip spreading | Speed and force were self-chosen and varied by person and sample (numbers are in figures only) | Fehlberg M. et al. (2025), "A Touch of Stribeck – Finger-Pad Friction in Viscous Liquid Spreading", *Tribol. Lett.* 73:91, doi 10.1007/s11249-025-02024-w. Full text not reached; supplement read. | [M], no extractable number |
| Blending speed estimate | Finger 30–100 mm/s. Circles about 1.5–3 Hz at 10–20 mm diameter (about 50–190 mm/s tangential). Drift across the area 5–20 mm/s. | | [E] |

### 2.4 Skin–paper friction

| Item | Value | Source | Type |
|---|---|---|---|
| Dry skin, general | µ > 0.2, typically about 0.5, roughly independent of pressure | Derler S., Gerhardt L.-C. (2012), "Tribology of skin: review and analysis of experimental results for the friction coefficient of human skin", *Tribol. Lett.* 45:1–27, doi 10.1007/s11249-011-9854-y. Abstract via search; PDF unreachable. | [M] review |
| Moist or wet skin | µ typically > 1, strongly dependent on pressure | same | [M] review |
| Skin (palm/forearm) on 7 papers | µ = 0.26–0.68 at 60 ± 10 mm/s, load ramped to 11 ± 2 N, stroke 75 mm (palm) / 105 mm (forearm), paper RH 32/55/83%. Higher on the palm. | Vilhena L., Afonso L., Ramalho A. (2023), "Skin Friction: Mechanical and Tribological Characterization of Different Papers Used in Everyday Life", *Materials* 16:5724, §2–3, https://estudogeral.uc.pt/bitstream/10316/112185/1/ (full text read) | [M] |
| Finger on 21 printing papers | Load 0.2–5.2 N. Rougher (uncoated) papers gave lower µ than smoother coated papers (less contact). µ fell on repeated stroking over 15 back-and-forth cycles, more on coated papers. XPS showed skin lipids transferred to the paper and lubricated it. Only partial recovery. Small topographic changes. | Skedung L. et al. (2010), "Finger friction measurements on coated and uncoated printing papers", *Tribol. Lett.* 37:389–399 (abstract via VTT CRIS, https://cris.vtt.fi/en/publications/finger-friction-measurements-on-coated-and-uncoated-printing-pape/) | [M]; absolute µ values not reached |
| Finger µ falls with pressure | µ ∝ p^n with n = −0.79 to −0.96 (dry finger on smooth glass) and −1.05 to −1.42 (wet) | Derler et al. 2009, *Tribol. Int.* 42:1565, via Liu 2018 §3.1 | [M] secondary |

**Conflict.** Gee et al. (finger on paper, 2–20 N) and Skedung (0.2–5.2 N) report different µ on printing paper. Vilhena attributes the difference to the load range.

### 2.5 Paper stump and tortillon geometry

| Item | Value | Source | Type |
|---|---|---|---|
| Construction | Soft paper "tightly wound into a stick and sanded to a point at both ends". Also felt or leather. The tortillon is looser, with a hollow core, and pointed at one end. | Wikipedia "Stump (drawing)"; Mayer via Fitzwilliam | [P] |
| Diameters | #1, #2, #4, #6, #8 = 3/16", 1/4", 3/8", 1/2", 5/8" (4.8, 6.4, 9.5, 12.7, 15.9 mm) | University of Utah bookstore listing, https://www.store.utah.edu/Blending-Stumps-Assrt. The size-to-diameter mapping and mm conversion are an inference. | [S] vendor / [E] |
| Stiffness and tip contact | **No measurement found.** | — | — |
| Fraction of the tool footprint in contact | 0.2 (tortillon), 0.5 (stump) | Sousa & Buchanan 1999 §3.2, Fig. 3 | [E], their tuned graphics parameter |
| Tip contact estimate | Point use 1–5 mm². Tapered flank at about 30° tilt: 3–6 mm wide × 5–15 mm long, so about 10–60 mm² footprint, of which about 50% is in contact. Tortillon point 0.5–3 mm². Effectively rigid compared with a finger: contact set by geometry, not force. So contact pressure is several times a finger's at equal force. | | [E] |

---

## 3. Transport: pick-up, redeposition, oil, moisture, tooth

### 3.1 Measured particle transfer to soft, skin-like pads

| Item | Value | Source | Type |
|---|---|---|---|
| Dry residue to an oiled chamois-leather skin analog, single contact, 1 cm² | **Static: 11.5 ± 1.9%** from porous unfinished MDF, **38.8 ± 2.3%** from stainless steel. **With rotation (shear) during contact: 22.2 ± 2.3% (porous), 51.7 ± 2.7% (non-porous).** No dependence on force (10–240 N on 1 cm², i.e. 100–2400 kPa) or dwell time (2–300 s). | Amaral M.A., Hatten E., Gibson A.P., Morgan R.M. (2022), "The impact of force, time, and rotation on the transfer of ammonium nitrate: a reductionist approach to understanding evidence dynamics", *Science & Justice*, doi 10.1016/j.scijus.2021.12.006, §3.3–3.5. Accepted manuscript https://discovery.ucl.ac.uk/10141286/ (full text read). The residue was recrystallized salt, not loose pigment. Recipient: chamois leather with oil added to mimic skin. | [M] |
| Hand presses on dust-loaded surfaces | Only about 1/3 of the projected hand contacts smooth surfaces in a press. Transfer falls as surface roughness rises: carpet averages about 1/10 of stainless steel. Damp hands transfer significantly more. Consecutive presses cut transfer by a factor of about 3 as the skin loads, and about 100 presses are needed to reach equilibrium. | Rodes C.E., Newsome J.R., Vanderpool R.W., Antley J.T., Lewis R.G. (2001), "Experimental methodologies and preliminary transfer factor data for estimation of dermal exposures to particles", *J. Expo. Anal. Environ. Epidemiol.* 11:123–139, abstract (https://www.nature.com/articles/7500150) | [M] (abstract only) |
| Soil to skin, repeated contact | A second contact added only about 8% more adhered soil or sand. Mean loading 0.74 mg/cm² on cadaver skin vs 0.21 mg/cm² on cotton (10–50 kPa, 10–50 s). | Ferguson et al. 2009, mechanical chamber study (via search summary; EPA HERO 470306/470307) | [M] secondary |
| Graphics prior art (graphite) | Paper → blender: 0.7 × local pressure of the lead present. Blender → paper: 0.5 × pressure of the load the blender carries. The blender keeps part of what it releases (t = 0.2 tortillon, 0.5 stump; smaller t means more storage). | Sousa & Buchanan 1999 §4.4, Eqs. 4–9 | [E], tuned by eye; not measured |

### 3.2 Pressing fines into pores and loss of tooth

- **Tooth:** harder rubbing pushes pastel "into the surface and into the colours underneath". Light pressure avoids filling the tooth (Orme 2017) [P]. A light stroke leaves pastel on the ridges, which only drifts into the valleys (Met) [P]. No measured pore-filling data were found.
- **Rough surfaces hold particles:** carpet transfer is 1/10 of steel (Rodes 2001) and porous MDF releases about 1/3 as much as steel (Amaral 2022) [M]. This supports a model in which particles lodged in valleys or pores are hard to pick up. Blending moves material from the exposed ridges into the protected valleys, where later passes rarely reach it [E].

### 3.3 Skin oils and moisture

| Item | Value | Source | Type |
|---|---|---|---|
| Lipid transfer finger → paper | Shown by XPS. Lowers µ over 15 stroke cycles (see §2.4). | Skedung 2010 | [M] |
| Latent print mass | Mean 6.73 µg per print (range 0.33–29 µg) | Croxton (2008) microbalance, via a review (Hull repository) | [M] secondary |
| Implied areal deposit | About 3–7 µg/cm² per touch, assuming a 1–2 cm² print [E]. Water is 20–70 wt% of fresh marks (via review), so lipid is about 1–5 µg/cm² [E]. | | [E] |
| Effect on medium | Finger smudging "deposits oil from the skin onto the drawing surface, making the surface less receptive" | Wikipedia "Stump (drawing)" | [P] |
| Discoloration | "oils from the skin can be deposited on the paper, causing discoloration" | Artists Network Q&A (via search) | [P] |
| Counterpoint | The palm and finger pads have no sebaceous glands; much fingerprint oil is transferred from the face | BYU conservators blog, https://conservators.byu.edu/why-arent-you-wearing-gloves | [P] |
| Conservators | The friable powder "increases potential for inadvertently removing or altering media with handling and manipulation" (Getty conservator, quoted) | The Art Newspaper 2018, via search | [P] |
| Moisture | Moist skin µ > 1 (Derler & Gerhardt 2012). Damp hands transfer more particles (Rodes 2001). | | [M] |

**Conflict.** Art sources treat finger oil as harmful; one conservator blog argues fingertip oil is overstated. No measured effect of skin oil on how well pastel adheres afterwards was found.

### 3.4 Color: does blending darken or saturate?

- **Practitioners:** blending and smearing "dull" or "mute" color (Artists Network, forums) [P]. Pressing harder drives pigment "into the colours underneath" (Orme) [P].
- **Fixative** does darken pastel, because a binder replaces the air between particles and reduces scattering. This mechanism is widely repeated but was not quantified in the sources found (justpaint.org survey; Wikipedia "Pastel"; studio manuals) [P].
- **Compaction alone** (no binder) does *not* darken powders in measured visible-range studies. Compact olivine and meteorite layers had *higher* reflectance than fluffy ones (JAXA repository record 20720). Porous fly ash looked redder than compact (JPGU 2003) [M]. Packing and roughness effects were confounded in those studies.
- **Conflict, and a reading [E]:** the "dull or muddy" effect of blending is mainly explained by:
  - mixing of colors within one cell, which loses the chroma that unmixed side-by-side strokes keep;
  - filling the paper valleys, which raises coverage. On white paper that reads as darker and more saturated; on toned paper it moves toward the pastel color.

  It is not explained by an intrinsic darkening of compacted dry powder. Do not add a separate lightness penalty for compaction.

---

## 4. Quantitative rub and smear tests (pressure comparison)

| Test | Parameters | Source | Type |
|---|---|---|---|
| ASTM D5264 (Sutherland rub tester, printed packaging) | 4 lb weight (17.8 N) standard, 2 lb (8.9 N) option. About 4 in² (2581 mm²) effective pad, so about 1 psi (6.9 kPa). Motor speeds 21/42/85/106 cycles/min. Specimen about 6 × 3 in. Rubber pads. Standardized receptor paper. Result graded on transfer to the receptor. | Vendor pages (Gardco, Thwing-Albert, Labthink) and patent text (US PTO full-text pdfs, via search). The standard itself is paywalled (ASTM D5264-98(2019)). | [S] |
| AATCC 8 crockmeter (textiles; also the basis of ASTM F1319 for copy and print smudge) | 16 mm diameter finger (201 mm²), 9 N ± 10% (about 45 kPa), 104 mm stroke, 10 full cycles at 1 cycle/s (mean speed about 208 mm/s [E]) | testex / worldoftest / Paramount vendor sheets (via search) | [S] |
| ASTM F1319 (withdrawn 2020) | Crockmeter method for "abrasion and smudge resistance of images produced from business copy products". Measures material moved to the cloth or redeposited on the sample, by density before and after rubbing. | ASTM store abstract https://store.astm.org/f1319-94r11.html | [S] |
| Toner fix (rub) tests | Silbon paper at 4.9 kPa (50 g/cm²), 5 reciprocations, % density loss. Or sand eraser 500 gf on 15 × 7.5 mm (about 44 kPa [E]), 5 reciprocations. Grades: <1% (A) … ≥10% (E) density loss. | Japanese toner patents (US PTO, via search) | [S]. Fused, bound toner: an analog for *bound* pastel, which should barely move. |
| Finger blending, for comparison | 1 N over 107 mm² apparent ≈ 9.3 kPa; over 27 mm² real ≈ 37 kPa. 0.5 N ≈ 6 kPa; 2 N ≈ 15 kPa apparent. | from Liu 2018 fits | [E] |

**Takeaway [E].** Finger blending (about 5–20 kPa apparent) is in the same pressure band as Sutherland (6.9 kPa), toner rub (4.9 kPa) and crockmeter (45 kPa). The roughly force-independent transfer seen by Amaral (100–2400 kPa) suggests that per-pass pick-up depends weakly on force once contact is made. Force acts mainly through contact *area* and through pushing material into the tooth. No published per-pass transfer fraction exists for loose pastel or charcoal. The pencil-drawing wear preprint (Research Square rs-10448967, "The physics of pencil drawing") could not be read.

---

## 5. Things not found (searched)

- Any motion-capture or force measurement of artists blending pastel (passes, stroke length, speed).
- Stump stiffness or tip contact measurements.
- Per-pass pick-up and redeposition fractions for dry pastel or charcoal under a finger.
- A measured effect of skin oil on the later adhesion of pastel.
- Russell 1772 primary text, Mayer *Handbook* text, Doerner, Jeffares technique essays, Maheux on Degas (all blocked or not found).

---

## 6. Recommended parameters for the simulator

1. **Contact law (finger)**
   - A_app = 135·F^0.4 mm² for F ≤ 2 N. Above 2 N, A_app = 178·(F/2)^0.14 mm², capped near 200–250 mm². The exponents are Liu 2018 [M]; the 135 prefactor is the current value and sits within the spread between people (Tomlinson 2009) [M].
   - Real/apparent area ratio of about 0.2 at 0.5 N rising to about 0.45 at high load (Liu 2018) [M]. Ridges cover about 50% of the outline (Tomlinson) [M].
   - Ellipse length/width of about 1.5–2.0 (Tomlinson Table 5.3) [M].
   - Sliding area 0.93 × static (Liu 2018) [M].
2. **Per-tool pad**
   - Index finger: as above.
   - Two fingers (index + middle) for broad first blends: about 2 × finger area, side by side (Orme 2017 [P]; area [E]).
   - Little finger: about 0.6 × index area [E].
   - Heel of palm for contour pulls: about 400–1200 mm² [E].
   - Stump: rigid, footprint 10–60 mm² on the flank, 1–5 mm² at the point, contact fraction 0.5 (Sousa 1999 [E]). Diameters 4.8–15.9 mm (vendor [S]).
   - Tortillon: 0.5–3 mm², contact fraction 0.2 [E].
   - Chamois over a finger: finger area × about 1.2, conformal (leather nap fills ridges) [E].
   - Colour Shaper or soft brush: 20–80 mm², very light load [E].
3. **Force**
   - Finger 0.3–1 N default, up to 2 N for a firm "rub-in". Anchors: 0.49–0.64 N measured in exploration (Smith 2002) [M]; light pressure (Orme [P]).
   - Stump 0.5–2 N (pen forces 0.8–3.2 N [M] as anchor).
   - Brush or Colour Shaper 0.05–0.3 N.
   - Chamois wipe 0.3–1 N.
   - All tool forces except the finger default are [E].
4. **Speed**
   - Finger 30–100 mm/s (Smith 2002: 77–86 mm/s [M]; common range 33–110 mm/s [M]).
   - Stump 20–60 mm/s [E].
   - Chamois or palm wipe 100–200 mm/s [E]; the crockmeter's about 208 mm/s mean is a reference [S].
   - Assume friction is independent of speed in this range (dry skin µ is roughly constant [M]).
5. **Stroke pattern (area mode)**
   - Default for broad areas: loose circles of 10–20 mm diameter at about 1.5–3 Hz, drifting across the area at 5–20 mm/s (Orme [P]; numbers [E]).
   - Form-following mode: long linear strokes (40–120 mm) along the contour or form direction, back and forth (Orme "pull… in the direction of the hills" [P]).
   - Detail: small circles of 3–8 mm with the little finger or a stump (Orme; Caran d'Ache [P]).
6. **Spacing and overlap**
   - Lane spacing ≤ 0.5 × pad width, so each point gets ≥ 2 contacts per traverse. With circles, every point sees about (pad width + circle diameter)/(drift per cycle) ≈ 5–10 contacts per traverse [E].
7. **Passes and crossing**
   - 1–3 traverses. The second is at about 90° to the first (Sousa 1999: vertical, then horizontal stump [P]; perpendicular at color junctions [P]).
   - Total contacts per point about 5–20 [E].
   - Expose an "add pastel → blend" loop rather than many blend passes, since practitioners warn that over-blending dulls the color [P].
   - Enforce a minimum loose load below which blending does almost nothing ("one stroke… not a lot happens", Orme [P]).
8. **Pick-up and lay-down per contact**
   - Base pick-up of *loose* pastel on toothy paper ≈ 10–15% static and ≈ 20–25% sliding. From the exposed ridges (smooth-like) up to 40–50%; from valleys ≈ 5–10%. Anchors: Amaral 2022 porous 11.5% static / 22.2% sheared, smooth steel 38.8% / 51.7% [M]; Rodes 2001 rough ≈ 1/10 of smooth [M]. The ridge/valley split is [E].
   - Make pick-up nearly independent of force once in contact (Amaral [M]). Force should act through area and through driving material into valleys (Orme [P]).
   - Reduce pick-up as the pad loads: about ×1/3 at saturation (Rodes 2001 [M]). Skin capacity is of order 0.2–0.7 mg/cm² (Ferguson 2009 soil data [M]; applying it to pastel is [E]).
   - Lay-down: a fraction of the pad's load per contact, about 0.3–0.5 (Sousa 0.5 × pressure, tool keeps part [E]). A stump stores less than a finger [E].
   - Bound or fixed pastel stays. Toner rub tests treat <1–3% density loss over 5 rubs as a well-bound image [S].
9. **Oil and moisture (finger only)**
   - About 1–5 µg/cm² lipid per touch [E]. Over many passes (Skedung: 15 cycles), slowly raise the "bound" fraction of fines in touched cells and lower µ [E, direction from M].
   - Damp-finger option: pick-up × about 1.5–2 and µ above 1 (direction from Rodes 2001 and Derler & Gerhardt 2012 [M]; magnitude [E]).
10. **Color**
    - Let color change emerge from mixing within each cell plus increased valley coverage. Do not add a separate darkening term for compaction (§3.4: conflicting evidence; compaction without a binder does not reliably darken).

---

## 7. What `paint::pastel` takes from this

- **Contact** (`pad_area`): 135·F^0.4 mm² up to 2 N, then 178·(F/2)^0.14 (§2.1, §6.1). The finger in `rub` always slides, so the sliding factor (0.93) is not applied separately; the ellipse and the real/apparent split are not modeled (a round pad).
- **Pick-up**: 22 % of the loose pastel under the pad a pass (Amaral, porous, sheared), independent of force; a skin pad's falls to a third as it loads to 0.45 mg/cm² (Rodes; Ferguson's soil capacity, for pastel [E]). Not modeled: the ridge/valley split ([E]), skin oil and moisture (§6.9), a darkening term for compaction (§6.10 says not to).
- **Lay-down**: 0.4 of the pad's load a pass (§6.8 [E]).
- **Area mode** (`rub_area`): a fingertip in loose 15 mm circles at 2 Hz drifting at 10 mm/s, 0.6 N; a stump (pad ≤ 60 mm²) in straight strokes at 40 mm/s, 1 N, each lane rubbed there and back; lanes half the width a lane sweeps apart (the pad's, plus a circle's); two passes crossed at 90° (§6.3–6.7). Not modeled: the two-finger, little-finger, palm, tortillon, chamois and Colour Shaper pads (§6.2), and a minimum loose load below which blending does nothing (§6.7: the pick-up share is the same at any load).
