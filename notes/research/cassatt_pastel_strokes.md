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

The easel's defaults take the middle of these: `zigzag=true` swings 5 mm to either side, 8 mm per swing, corners rounded 0.4, a run of 40 mm along the line; bands 10 mm apart (its swing's width). `jitter=true` turns marks by up to ±10° (0.17 rad) and stretches them by up to ±30%: the spread of angles within one passage (§2) and of the lengths within one (§3). Plain `p:hatch` keeps the pencil's defaults (1.5 mm apart, 12 mm long), which are face and hand hatching rather than these passages.

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

## Sources

- The Metropolitan Museum of Art, collection records (public API, `collectionapi.metmuseum.org/public/collection/v1/objects/<id>`) for objects 13533 (Nurse Reading to a Little Girl), 10404 (Mother Playing with Child), 10403 (Mother Feeding Child), 10387 (Margot in Orange Dress), 20521 (Portrait of Charles Dikran Kelekian, Age Eight): titles, dates, media, sizes. Read 2026-11-15.
- The Met's photographs of the same sheets, as copied to Wikimedia Commons (file names in §1). Downloaded 2026-11-15 for measurement and not kept.
- B. Jobard and W. Lefer, "Creating evenly-spaced streamlines of arbitrary density", *Visualization in Scientific Computing '97* (Springer, 1997), pp. 43–55: the placement `paint::area_strokes` uses for a direction field.
