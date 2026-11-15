# Cassatt's pastel strokes over an area, 1895–1908: lengths, angles, spacings

Measured on museum reproductions, for a pastel stick's `p:hatch` over an area (the easel's `--@ pastel-area-strokes`; notes/easel_guide.md, Drawing). Compiled 2026-11-15. It backs gap 2 of notes/cassatt_support.md and §7 of notes/research/cassatt_materials.md (both on branch `notes/cassatt-materials`).

**Every number here is an estimate [E].** Each was read off a photograph of the sheet, at a scale taken from the museum's recorded size. No source I found gives stroke measurements for her. Treat the numbers as typical ranges, not values.

---

## 0. Bottom line

| passage | strokes | angle | length | width | apart |
|---|---|---|---|---|---|
| Backgrounds, open (1902, 1908) | slashing diagonals, broken over the paper or an underlayer | 40–65° rising to the right; flatter (28–35°) toward an upper right corner | 30–80 mm | 2–6 mm (edge or point), to ~8 mm (side) | 5–15 mm between distinct strokes; they overlap in denser parts |
| Garments, dense (1902) | long strokes along the folds, laid over each other, no paper showing | 74–86° (near vertical) in a skirt falling from the knee | 50–100 mm and more | 3–8 mm | overlapping |
| Garments, zigzag (1895) | one continuous back-and-forth stroke | legs along the form, the run across it | runs 12 mm (tight) to 30–60 mm (open) | 1.5–3 mm | see below |

Zigzags, measured in one jacket (1895):
- **tight, looped:** legs about 6 mm (a swing of 3 mm to either side), about 4 mm along the run per swing there and back, rounded turns, a run of about 12 mm;
- **open, a "ladder":** legs 10–20 mm (a swing of 5–10 mm), legs 4–8 mm apart (8–14 mm per swing there and back), runs of 30–60 mm, sharper turns.

The easel's defaults take the middle of these: `zigzag=true` swings 5 mm to either side, 8 mm per swing, a sine (§5), a run of 40 mm along the line; bands 10 mm apart (its swing's width). `jitter=true` turns marks by up to ±10° (0.17 rad) and stretches them by up to ±30%: the spread of angles within one passage (§2) and of the lengths within one (§3). A plain stick `p:hatch` lays the open backgrounds' strokes: **8 mm apart** (inside the 5–15 mm between distinct strokes, near its geometric middle, 8.7 mm) and **50 mm long** (the middle of 30–80 mm). A pencil keeps its 1.5 mm and 12 mm, face and hand hatching rather than these passages. How the hand holds and moves the stick (50°, a speed that goes with the stroke's length, a bow about the wrist) is in §5.

---

## 1. Sheets and method

Five pastels at the Metropolitan Museum of Art; four were measured (Mother Playing with Child was looked at only). Their sizes and media are from the Met's collection records (its public API, object pages). The images are the Met's photographs as copied to Wikimedia Commons (file names below).

| sheet | date | medium (Met) | size (Met) | image (Commons) | px | scale |
|---|---|---|---|---|---|---|
| Nurse Reading to a Little Girl | 1895 | pastel on wove paper, mounted on canvas | 60 × 73 cm | Nurse Reading to a Little Girl MET DT7250.jpg | 3811 × 3144 | 5.17 px/mm |
| Mother Playing with Child | ca. 1897 | pastel on wove paper, mounted on cardboard | 64.8 × 80 cm | Mother Playing with Child MET DT5262.jpg | 3811 × 3049 | 4.76 px/mm |
| Mother Feeding Child | 1898 | pastel on wove paper, mounted on canvas | 64.8 × 81.3 cm | Mother Feeding Child MET DT5263.jpg | 3809 × 3059 | 4.67 px/mm |
| Margot in Orange Dress | 1902 | pastel on wove paper, mounted on canvas | 72.4 × 59.4 cm | Margot in Orange Dress MET DT203405.jpg | 3078 × 3713 | 5.09 px/mm |
| Portrait of Charles Dikran Kelekian, Age Eight | 1908 | pastel | 66 × 52.7 cm | Portrait of Charles Dikran Kelekian, Age Eight MET DP304420.jpg | 3117 × 3864 | 5.79 px/mm |

**Scale.** The sheet's extent in each photograph (where it is brighter than the black surround) divided by the Met's width. The photographs' aspect ratios agree with the recorded sizes within 1.5%. Error: a few percent, more if a recorded size is a sight size (one Met record says "sight" for another sheet).

**Angles.** A structure tensor over a crop of 118–150 mm, sampled at 2 px/mm (notes/research/cassatt_stroke_orientation.py). It gives the dominant stroke direction and a coherence (0: no direction; 1: all strokes parallel). For backgrounds I used the red-minus-blue channel, which separates ochre or orange strokes from a green or blue ground; for garments, luminance. Coherences were 0.15–0.34: one direction dominates, with a broad spread around it (the paper's grain is in the crop too).

**Lengths, widths, spacings.** Read by eye from full-resolution crops with a 1 cm grid drawn over them (and a 2 mm grid on a 2× zoom of the zigzags). A stroke's length is from where it lands to where it lifts, where both ends can be seen; strokes that run into others were not counted.

**Not done.** The autocorrelation across the strokes in the same script found no regular period in any crop: her strokes are not evenly spaced. Spacings are therefore read by eye. Only five sheets, all from one museum, mostly 1895–1908; no sheet from 1890–94 had a usable large image that I found.

## 2. Angles

| sheet, passage | crop at (px) | angle | coherence |
|---|---|---|---|
| Margot 1902, background upper left | 150, 150 | 58° | 0.22 |
| Margot, background top middle | 1100, 150 | 40° | 0.24 |
| Margot, background left | 150, 1100 | 51° | 0.28 |
| Margot, background upper right | 2300, 300 | 29° | 0.23 |
| Margot, dress (four crops) | 500–2000, 2550–3000 | 74°, 77°, 82°, 86° | 0.22–0.33 |
| Kelekian 1908, background upper left | 100, 300 | 61° | 0.32 |
| Kelekian, background left | 100, 1300 | 74° | 0.29 |
| Kelekian, background upper right | 2300, 400 | 28° | 0.24 |
| Kelekian, background right | 2300, 1500 | 35° | 0.15 |
| Mother Feeding 1898, background (three crops) | | 3°, 33° (147°), 177° | 0.18–0.34 |

Angles are counterclockwise from the horizontal as seen, so 45° rises to the right. The strokes **fan**: steeper on the left of a background, flatter toward its upper right, as a right arm swinging from the elbow would lay them. That is why `angle=` takes a function. Mother Feeding's background is blended broad side strokes, near horizontal, with little to measure.

## 3. Lengths, widths, spacings

**Margot (1902), background** (crop 150, 150, 16 cm). Ochre strokes over a green layer, broken by the tooth into dashes; light pressure. Single strokes 30–60 mm long, 2–4 mm wide; distinct strokes 5–10 mm apart, about a fifth of the area ochre.

**Margot, dress** (crop 700, 2550). Long overlapping strokes of two or three oranges along the folds, 50–100 mm and longer, 3–8 mm wide; no paper shows.

**Kelekian (1908), background** (crop 100, 300). Broad diagonal strokes of green, blue-green and some ochre, 40–80 mm long, 3–8 mm wide, 10–20 mm apart where they are distinct; the gray paper and earlier strokes show between them.

**Nurse Reading (1895), jacket** (crop 2600, 1700). Long dark blue-green diagonals 40–80 mm at about 50–60°. Over them, ochre and pale blue zigzags:
- the ochre one at the top is a tight looped zigzag: its run about 12 mm, three swings, legs about 6 mm (2 mm grid, 2× zoom);
- below it the ochre legs lengthen to 10–15 mm, 4–5 mm apart: the same stroke opening out as it runs;
- a pale blue "ladder" along the sleeve's edge: legs 15–20 mm, 6–8 mm apart, joined along the edge by the return strokes, over a run of 50–70 mm.

**Nurse Reading, lawn** (crop 2950, 600). Blended; no strokes to measure.

## 4. Uncertainties

- Reproductions flatten the surface: light, broken strokes over a matching layer are hard to see, so open passages may hold more strokes than counted.
- A stroke that overlaps another may be counted as one longer stroke.
- Five sheets from one collection. Her pastels of 1890–94 (the Met's "Woman on a Bench" is c. 1881, outside the range) were not measured.
- Whether a "zigzag" was one continuous movement or separate strokes is a reading of the photograph: the ochre one's turns are continuous; the ladder's joins could be separate strokes.

## 5. The hand: hold, speed, bow

These are not measured on her sheets. They come from measurements of hands and of drawing, and apply to any right-handed pastellist.

- **Altitude 50°.** A held pen's measured altitude: "the small amplitude of the pen angle variations (2.5 degrees) with respect to the average value (50 degrees)" (Schomaker & Plamondon 1990, four writers with a ballpoint on a tablet that read the angle), as paper.rs's reference hold (`REF_ALTITUDE_DEG`). A tablet stylus is held steeper, 62.4° ± 7.5° (Xin et al., pastel_stick_tribology.md §5), but a stick held in the fingers is closer to the pen. Azimuth stays 45°, upper end to the lower right: measured azimuth tables exist (US 8,090,203, table 2) but were not reached.
- **Speed.** A single handwriting stroke takes 100–150 ms for 2–10 mm (Thomassen & Van Galen, *J. Forensic Doc. Exam.*: "the typical duration of a stroke is 100 to 150 ms"; "strokes (taking 100 to 150 ms for a length of 2 to 10 mm)"). So a 12 mm mark goes at 80 mm/s, 150 ms. Longer lines take longer, but less than in proportion: Cao & Zhai's fit for straight lines drawn with a pen is T = 68.8 L^0.469 ms, L in mm (CHI 2007; the constants as quoted in Zhai, Kristensson & Appert, *Foundations and Trends in HCI* 5(2), 2012, §5.3.2; the CHI paper itself was not reachable). The easel takes the exponent and keeps the 150 ms anchor: v = 80 (L / 12 mm)^0.531 mm/s, 171 mm/s for 50 mm (293 ms). Cao & Zhai's own constant gives 221 ms at 12 mm: their lines were drawn to a precision, slower than hatching. A zigzag goes at its legs' speed: legs √((2A)² + (P/2)²) = 10.8 mm at the defaults, 75.5 mm/s, a leg in 143 ms. As a check, Thomassen & Van Galen's preferred cycle of stroke pairs, about 4 Hz, would give two 10.8 mm legs in 250 ms, 86 mm/s.
- **Zigzag corners: a sine.** A continuous back-and-forth is a hand oscillating across the line while it drifts along it, which Hollerbach's oscillation model of handwriting (*Biol. Cybern.* 39, 1981) makes near-sinusoidal. At the defaults (A = 5 mm, P = 8 mm) a sine's radius at its turns is P²/(4π²A) = 0.32 mm, far under the stick's 1.5–3 mm line, so the turns still look sharp, as the "ladder" does.
- **Bow.** A mark swung about the wrist is an arc of radius R, the hand's length: 189.3 mm, ANSUR II's mean over its 6068 men and women (men 193.3, women 181.1; computed from the public data files, `handlength`). A sweep s on that radius sags s²/(8R) at its middle, away from the pivot. For a mark at angle φ to the circle about the wrist, the wrist sweeps s = L cos φ of it (the rest is the fingers' and arm's straight push: the wrist's and the fingers' preferred directions are about orthogonal, Schomaker & Plamondon 1990, table 1), and the sag shows cos φ across the chord: L² cos³φ / (8R). At the default angle (51.6°) and hand (45°) the mark is 6.6° off the circle, so a 50 mm mark bows 1.6 mm and a 12 mm one 0.09 mm. The wrist turns about 40° in radial and ulnar deviation together for most tasks (Ryu et al., *J. Hand Surg.* 16A (1991) 409: "40 degrees of combined radial-ulnar deviation"), a reach of 189.3 × 40° = 132 mm. A longer sweep swings from the elbow: R = forearm and hand, 467.0 mm (ANSUR II, `forearmhandlength`). The hand lies where the azimuth points. The same pivot would explain the fan of §2, but the angles are left to `angle=`.

## Sources

- The Metropolitan Museum of Art, collection records (public API, `collectionapi.metmuseum.org/public/collection/v1/objects/<id>`) for objects 13533 (Nurse Reading to a Little Girl), 10404 (Mother Playing with Child), 10403 (Mother Feeding Child), 10387 (Margot in Orange Dress), 20521 (Portrait of Charles Dikran Kelekian, Age Eight): titles, dates, media, sizes. Read 2026-11-15.
- The Met's photographs of the same sheets, as copied to Wikimedia Commons (file names in §1). Downloaded 2026-11-15 for measurement and not kept.
- A. J. W. M. Thomassen and G. P. Van Galen, "Temporal features of handwriting: challenges for forensic analysis", *Journal of Forensic Document Examination* (online, ISSN 2640-0677), jfde.org article 255: stroke durations.
- X. Cao and S. Zhai, "Modeling human performance of pen stroke gestures", *CHI '07*, pp. 1495–1504, doi 10.1145/1240624.1240850; constants read in S. Zhai, P. O. Kristensson, C. Appert et al., "Foundational issues in touch-surface stroke gesture design: an integrative review", *Foundations and Trends in HCI* 5(2) (2012) 97–205, doi 10.1561/1100000012, §5.3.2.
- J. M. Hollerbach, "An oscillation theory of handwriting", *Biological Cybernetics* 39 (1981) 139–156.
- L. R. B. Schomaker and R. Plamondon, "The relation between pen force and pen-point kinematics in handwriting", *Biological Cybernetics* 63 (1990) 277–289 (preprint at ai.rug.nl): pen altitude, wrist and finger axes.
- ANSUR II (2012 Anthropometric Survey of U.S. Army Personnel), public data files, male (4082) and female (1986): `handlength`, `forearmhandlength`. Read 2026-11-15.
- J. Y. Ryu, W. P. Cooney, L. J. Askew, K. N. An and E. Y. Chao, "Functional ranges of motion of the wrist joint", *J. Hand Surg. Am.* 16 (1991) 409–419 (PubMed 1861019).
- B. Jobard and W. Lefer, "Creating evenly-spaced streamlines of arbitrary density", *Visualization in Scientific Computing '97* (Springer, 1997), pp. 43–55: the placement `paint::area_strokes` uses for a direction field.
