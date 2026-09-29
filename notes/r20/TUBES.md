# Round 20: the tube catalog and the boxes (not for painters)

Round 20's research wrote four tube proposals, one per artist:
`sargent_tubes.md` (S), `inness_tubes.md` (I), `alma_tadema_tubes.md`
(A) and `tonn_tubes.md` (T), in `~/tmp/r20-f7522c31/research/`. They overlap and sometimes
disagree. This note records how they became one catalog
(`crates/paint/src/palette.rs`, `catalog()`), with each tube defined once,
and which tubes each box holds (`BOXES`). It stays in the repo. The export
copies only the guide, the physics note and the studio's own materials
note, so no studio gets this file.

All numbers are estimates in the style of `palette.rs`, not measurements.
"Sourced" means the proposal quotes a source for that number or for its
direction (for example "less than chrome yellow"). "Estimate" means the
proposal gives no source.

## Rules

- The tube box's fourteen tubes keep their names and numbers. Their
  definitions moved into `catalog()`, but the values are the same bits
  (`the_tube_box_is_unchanged`), and a round 19 log replays to the same
  PNG and surface (`tests/boxes.rs`, `a_round_19_log_replays_as_before`).
- When only one proposal defines a tube, its line is used as written,
  except where a shared constant (below) replaces a one-off number.
- When two or more proposals define the same tube, each field goes to
  the sourced value if only one is sourced (a source the proposal itself
  marks uncertain doesn't count). Otherwise it goes to the majority value.
  A tie goes to the proposal whose reasoning is more specific, and the
  table says which one that was. T often cites one maker's published
  ratings (Old Holland's opacity, drying and oil content [OH]). Those
  count as sourced.
- Grades that a palette lists separately stay separate tubes, as smalt
  and pale smalt already are: cadmium yellow and pale cadmium, vermilion
  and its orange and Chinese grades, chrome yellow and lemon chrome.
- Drying rates use `drying::drier` constants. New constants are added
  only where no existing one fits the pigment; each has a doc comment
  giving the proposal's reason.

## New `drier::` constants

| constant | rate | from | reason |
|---|---|---|---|
| `RED_LEAD` | 2.2 | S | "exerts a powerful action on drying oils" [AP1 p.114]; above lead white (2.0) |
| `NAPLES_YELLOW` | 1.6 | A | a lead compound, between average and lead white; unsourced (A says so) |
| `ANTWERP_BLUE` | 1.6 | I | Prussian blue (1.8) diluted by an inert base; estimate |
| `COPPER` | 1.6 | S | copper pigments promote drying [AP2 p.136]; the same rate as copper green, which keeps its own `SMALT` constant (same value) |
| `MARS` | 1.1 | S | Mars colors are "good driers for oil paints" [CAMEO-mars]; a little above red earth (1.0) |
| `VIRIDIAN` | 1.0 | S, A | no drier action reported; average (both estimate 1.0) |
| `INDIAN_YELLOW` | 0.8 | S | an early account says it dries "nearly as soon or sooner than" other colors [AP1 p.24]; uncertain |
| `CADMIUM` | 0.6 | I (S and T agree; T cites Old Holland's "m") | "slow but reliable driers" [AP1 p.72]. A proposed 0.5 without a source and flagged it for checking; I's 0.6 rests on the quotation |
| `BITUMEN` | 0.15 | I | slows linseed oil's drying and never fully cures [CAMEO]; below madder lake (0.3) |

## The catalog: new tubes

Columns: masstone, hiding, stiffness, tinting strength, drying.

| tube | pigment | masstone | hid | stiff | str | dry | from | notes |
|---|---|---|---|---|---|---|---|---|
| lead-tin yellow | lead-tin oxide | `#e3cc6a` | 0.85 | 0.75 | 0.6 | 2.0 `LEAD_WHITE` | T | only T. Hiding, body and drying sourced [AP2-LTY pp.91–93]. The brand he uses is not stated, and modern tubes of this name vary (T) |
| zinc white | zinc oxide | `#f3f3ef` | 0.6 | 0.6 | 1.0 | 0.35 `ZINC_WHITE` | S | only S |
| Naples yellow | lead antimonate | `#e2b964` | 0.85 | 0.75 | 0.6 | 1.6 `NAPLES_YELLOW` | A | only A. What an 1880s tube of this name held is unverified (A) |
| lemon chrome | pale lead chromate with lead sulfate | `#eed83c` | 0.8 | 0.7 | 0.8 | 1.8 `CHROME_YELLOW` | I | S: `#f0d32a` 0.85/0.7/0.9. Both say the pale grade hides and tints less than chrome yellow (0.9/1.0). I ties both cuts to the coprecipitated lead sulfate [AP1 p.187], while S marks its cuts as estimates ("assumed slightly lower", "a little weaker"), so I's values. Stiffness and drying agree |
| pale cadmium | cadmium sulfide, a pale grade | `#f0c63c` | 0.85 | 0.7 | 1.1 | 0.6 `CADMIUM` | A, adjusted | A: stiffness 0.6, drying 0.5. It is the same pigment as cadmium yellow, so it gets the same body (0.7) and the sourced cadmium rate (0.6). A's masstone, hiding and strength are kept (they match the other cadmium lines) |
| cadmium yellow | cadmium sulfide | `#e8a51f` | 0.85 | 0.7 | 1.1 | 0.6 `CADMIUM` | I | S: `#efb512` 0.85/0.65/1.1/0.6; T: `#f0b000` 0.9/0.7/1.1/0.6. Strength and drying agree. Hiding: S's 0.85 ("good hiding power" [AP1 p.71]) and T's 0.9 (Old Holland: opaque) are both sourced, so the majority (S, I) gives 0.85. Stiffness 0.7 (I, and T from Old Holland's "l" oil content). The masstone is an estimate in all three, and the tie goes to I, whose drying rate is the sourced one |
| Indian yellow | magnesium and calcium euxanthate | `#e1a11e` | 0.15 | 0.4 | 0.8 | 0.8 `INDIAN_YELLOW` | S | only S |
| Mars yellow | synthetic iron oxide hydroxide | `#c4872b` | 0.85 | 0.7 | 1.1 | 1.1 `MARS` | S | only S |
| transparent oxide yellow | transparent synthetic iron oxide | `#7a4a14` | 0.2 | 0.5 | 0.9 | 1.0 `RED_EARTH` | T | only T; all estimates |
| brown ochre | iron oxide earth, a darker grade | `#86592e` | 0.8 | 0.7 | 0.8 | 0.8 `OCHRE` | A | only A (it copies yellow ochre's numbers) |
| raw sienna | sienna earth, unroasted | `#9a6a2b` | 0.4 | 0.5 | 0.7 | 1.2 `SIENNA` | I | only I |
| orange chrome | basic lead chromate | `#e0712a` | 0.88 | 0.75 | 0.9 | 1.8 `CHROME_YELLOW` | I | only I; hiding, strength and drying sourced [AP1 pp.207–208] |
| red lead | lead tetroxide | `#e0542b` | 0.85 | 0.8 | 0.8 | 2.2 `RED_LEAD` | S | only S |
| orange vermilion | mercuric sulfide, a yellower grade | `#dd4a22` | 0.9 | 0.75 | 1.0 | 0.4 `VERMILION` | A | only A. The shade is inferred from the name (A marks it uncertain). The body is vermilion's |
| Chinese vermilion | mercuric sulfide, a deeper grade | `#b8282e` | 0.9 | 0.75 | 1.0 | 0.4 `VERMILION` | A | as orange vermilion. A's alternative was to keep only the existing vermilion. The pupil's palette lists both grades by name, so both are kept |
| cadmium red | cadmium sulfoselenide | `#c3321f` | 0.9 | 0.7 | 1.1 | 0.6 `CADMIUM` | S, T | T: `#c2302a` 0.9/0.7/1.2/0.6. Hiding and drying agree. Stiffness 0.7 is T's (Old Holland's "l" oil content); S wrote 0.65 "as cadmium yellow", and the catalog's cadmium yellow is 0.7 too. Strength: both are estimates (S "as cadmium yellow", T "strong tinters"), and the tie goes to S, which keeps it equal to cadmium yellow. Masstone: S's, which is described from a source ("an opaque scarlet" [BAS]) |
| Mars red | synthetic iron oxide | `#a33f2a` | 0.9 | 0.7 | 1.2 | 1.1 `MARS` | S | only S |
| Indian red | nearly pure ferric oxide | `#7a3a33` | 0.92 | 0.7 | 1.2 | 1.0 `RED_EARTH` | I | only I. It is a separate pigment from Mars red (natural haematite, purplish [CH p.161]) |
| permanent alizarin | a quinacridone | `#5e1624` | 0.15 | 0.45 | 1.3 | 0.3 `MADDER_LAKE` | T | only T. He says his alizarin is "quinacridone based"; the exact pigment is not stated (T marks it uncertain). A different pigment from rose madder, so a separate tube |
| rose madder | madder lake on alumina | `#8e2238` | 0.1 | 0.35 | 0.9 | 0.3 `MADDER_LAKE` | S | A: `#7e1f3b` 0.1/0.35/0.6. Hiding, stiffness and drying agree. On strength S cites a source (purpurin-rich madders from 1861 had "about fifty times the tinting strength" of older ones [MMJ53 p.175]) and A gives an unsourced "weaker than a crimson lake", so S's strength is used. The masstone is an estimate in both proposals, and S's is kept with its line |
| burnt sienna | roasted sienna earth | `#7c3f24` | 0.45 | 0.55 | 0.9 | 1.2 `SIENNA` | majority | S `#8a4a2b` 0.45/0.55/0.9; I `#7c3f24` 0.45/0.50/0.9; A `#7c3b22` 0.5/0.55/1.0; T `#7b3a1e` 0.6/0.55/0.9. T's hiding cites Old Holland's "opaque" rating but T marks it uncertain, since siennas are semitransparent in thin films. The rest are estimates. Majority by field: masstone I (A's and T's nearly the same), hiding 0.45 (S, I), stiffness 0.55 (S, A, T; T's from Old Holland's "h" oil content), strength 0.9 (S, I, T); SIENNA in all four |
| Mars brown | synthetic iron oxide, roasted | `#5a3a28` | 0.85 | 0.65 | 1.0 | 1.2 `SIENNA` | S | only S. The rate is 1.2 as S proposed: iron oxides dry well but lack the manganese that makes umber fast. It borrows the SIENNA constant for its value |
| bitumen | asphaltum | `#2e2017` | 0.12 | 0.3 | 0.7 | 0.15 `BITUMEN` | I | only I. The engine has no traction crackle from slow films, so this tube is only slow and transparent |
| cerulean blue | cobalt stannate | `#3f82b3` | 0.8 | 0.7 | 0.6 | 1.4 `COBALT_BLUE` | S, T | T: `#2e78b0` 0.8/0.7/0.6/1.4. Hiding: S's 0.7 ("an opaque, greenish blue" [BAS]) and T's 0.8 (Old Holland: opaque, both grades) are both sourced; T's rating is the more specific. Stiffness 0.7 is T's, sourced (Old Holland: low oil content), against S's estimate. Strength and drying agree. Masstone: S's, described from a source ("a light, greenish blue" [BAS]); T's is an estimate |
| ultramarine blue | synthetic ultramarine | `#232a8c` | 0.3 | 0.5 | 1.1 | 0.8 `ULTRAMARINE` | T | S called it "French ultramarine": `#26318c` 0.3/0.5/1.2/0.8. It is one pigment, so it is one tube, under T's plainer name. T's masstone is converted from a published measurement [AP2 p.51]; S's is an estimate. Hiding, stiffness and drying agree. Strength: both read the same quotation [AP2 p.44] ("quite good", S 1.2, T 1.1), and the tie goes to T, whose line has the measured masstone |
| Antwerp blue | Prussian blue on an alumina base | `#26406c` | 0.4 | 0.45 | 1.6 | 1.6 `ANTWERP_BLUE` | I | only I |
| viridian | hydrated chromium oxide | `#1c4a40` | 0.3 | 0.5 | 0.9 | 1.0 `VIRIDIAN` | S | A: `#1f5648` 0.3/0.45/0.7/1.0. Hiding (sourced in S [AP3 p.277]) and drying agree. S's masstone is sourced ("a deep blackish green in oil" [AP3 p.278]). Stiffness and strength are estimates in both, so the tie goes to S, whose line has the sourced masstone |
| emerald green | copper aceto-arsenite | `#23a57a` | 0.6 | 0.6 | 0.6 | 1.6 `COPPER` | S | only S |
| cobalt violet | cobalt phosphate or arsenate | `#7e4c8e` | 0.35 | 0.55 | 0.35 | 1.4 `COBALT_BLUE` | S | only S |

Not in the catalog, because no box uses them: brown pink (I: an uncertain
reading of one OCR'd line), deep cadmium (A: "very little, if at all")
and titanium white (T: demoted; "I basically don't use titanium" since
2022).
Mars orange and ultramarine ash (S: paint box only) were not proposed as
tubes.

## The boxes

A box holds exactly the tubes its research note documents the artist
using. Existing tubes count when the proposal maps a documented pigment to
them. The default `tube box` is unchanged and stays the default.

### `sargent` (23)

lead white, zinc white, lemon chrome, chrome yellow, cadmium yellow,
Indian yellow, yellow ochre, Mars yellow, red lead, vermilion, cadmium
red, Mars red, red earth, rose madder, burnt sienna, Mars brown, bone
black, cerulean blue, cobalt blue, ultramarine blue, viridian, emerald
green, cobalt violet.

(S's "French ultramarine" is the catalog's `ultramarine blue`: one
pigment, one tube.)

- Existing tubes from S's table: lead white, vermilion, bone black, yellow
  ochre, red earth (red iron oxide in the rose layer and a ground
  [MMX]), cobalt blue and chrome yellow (on palettes [HAM]; S marks it
  ✓).
- Judgment calls:
  - *Chrome yellow and lemon chrome both.* HAM names chrome yellow with no
    grade, and APOLLO and BAS name a pale grade.
  - *Burnt sienna.* APOLLO says "sienna" and doesn't say raw or burnt.
    Burnt is S's choice (uncertain), and the box holds one sienna, not
    two.
  - *Indian yellow.* It is documented only in a paint box [BAS], and S
    marks it uncertain for portraits. It is kept because the paint box is
    documentary evidence that he had the tube.
  - *Red lead.* It was found in one painting [BAS]. It is documented, so
    it is kept.
  - *Zinc white.* He used it "very rarely" [APOLLO], and it is on palettes
    [HAM]. It is kept.
- Left out: smalt, pale smalt, Prussian blue, green earth, Rinmann's
  green, copper green and raw umber (S: not used as far as the sources
  go). Also left out: Mars orange and ultramarine ash (paint box only, no
  tube proposed), and magenta and bone brown (purchases, not found in any
  analysis).

### `inness` (13)

lead white, lemon chrome, cadmium yellow, yellow ochre, raw sienna,
orange chrome, red earth, Indian red, raw umber, bitumen, bone black,
cobalt blue, Antwerp blue.

- Existing tubes from I's table: lead white (white of unknown kind, with
  lead white as the proxy), yellow ochre, raw umber, bone black (ivory
  black), cobalt blue and red earth (Venetian red for the canvas stain;
  single witness, which I marks *Used as proxy*).
- Chrome yellow and Prussian blue are left out. I lists them only as
  proxies for lemon chrome and Antwerp blue, which the box now has.
- Judgment calls:
  - *Bitumen is in.* One witness says he used it, and crack patterns
    point to it [MI95; NGA96].
  - *Burnt sienna is out.* I calls it "optional" because only "sienna"
    is named, and raw sienna is named explicitly.
  - *Brown pink is out.* It depends on an uncertain OCR reading ("cobalt*
    brown and pink").
- The single-witness pigments (Antwerp blue, Indian red and lemon chrome
  [SHEL]; orange chrome [MI95]; cadmium [SON]) are in. Each rests on one
  witness, and I ranks all of them among its top three priorities.
- No green is documented, so the box has none.

### `alma-tadema` (13)

lead white, Naples yellow, pale cadmium, yellow ochre, brown ochre, orange
vermilion, Chinese vermilion, red earth, rose madder, burnt sienna, bone
black, cobalt blue, viridian.

- The pupil's palette [MANUAL]: flake white (lead white), brown and yellow
  ochre, Naples yellow, orange and Chinese vermilion, light red (red
  earth), rose madder, burnt sienna, cobalt (cobalt blue) and ivory black
  (bone black).
- The plain `vermilion` is left out: the palette names its two grades.
- Judgment calls:
  - *Pale cadmium and viridian are in.* The manual lists both before its
    statement that "so far" the palette is his, so they fall inside what
    it attributes to him. A marks both uncertain and suggests them as a
    reserve.
  - *Raw umber and deep cadmium are out.* The pupil says he "uses very
    little, if at all" of them. A lists raw umber only as an optional
    reserve.
  - *Chrome yellow is out.* The same manual rejects it.

### `tonn` (14)

lead white, lead-tin yellow, cadmium yellow, yellow ochre, transparent
oxide yellow, cadmium red, red earth, permanent alizarin, burnt sienna,
raw umber, bone black, ultramarine blue, cerulean blue, green earth.

- T's suggested box, plus transparent oxide yellow from its optional
  list. Existing tubes from T's table: lead white, yellow ochre, red earth
  (for English red, a synthetic red iron oxide), raw umber (also standing
  in for red umber, the nearest tube), bone black (for ivory black; the
  same pigment [OH]) and green earth (terre verte, an occasional glaze).
- Judgment calls:
  - *Red earth is in.* He recommends English red in limited palettes (2021,
    2023), but also said in 2021 that he hadn't put it on his palette "in a
    million years" (T: "used, at least earlier"; the materials note:
    uncertain for current use). T keeps it, and so does the box.
  - *Transparent oxide yellow is in.* It was on a posted palette of 2025
    [PAL25], which is documented use. T lists it as optional because
    nothing says how often he uses it. The sargent box keeps Indian yellow
    on similar evidence (a paint box).
  - *Titanium white is out.* He used it for highlights in 2021 and said in
    2022 that he basically doesn't use it; T demotes it to optional, and
    the materials note treats the 2022 statement as current.
  - *Cobalt blue and cobalt violet are out.* Cobalt blue: "haven't used it
    in ages" (2021). Cobalt violet: rare, "I haven't used cobalt violet
    much" (2025). T leaves both out of its box.
  - *Vermilion and zinc white are out.* He owns vermilion but hasn't used
    it, and he avoids zinc.

## Ordering

A box's tubes are listed roughly by hue (whites, yellows, oranges and
reds, earths and browns, black, blues, greens, violet). The default box
keeps its round 19 order. Order changes nothing in how paint mixes. A
pile names its tubes, and a log replays with the same box.

## Engine notes from the proposals, not done in round 20

- S suggests lead white in poppy oil (drying 1.4–1.6) for his whites. Not
  done: the catalog has one lead white, and the tube box's must not
  change.
- S and I suggest a megilp (gelled, fast-drying) pile medium. A suggests
  a copal (resinous) one, and I suggests a siccative (Haarlem or
  Courtrai). The engine has one `medium` (oil), so none of these is
  modeled.
- I suggests traction crackle for bitumen over faster layers. It is not
  modeled.
