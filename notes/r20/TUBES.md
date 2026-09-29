# Round 20: the tube catalog and the boxes (not for painters)

Round 20's research wrote five tube proposals, one per artist:
`sargent_tubes.md` (S), `inness_tubes.md` (I), `alma_tadema_tubes.md`
(A), `tonn_tubes.md` (T) and `hopper_tubes.md` (H), in `~/tmp/r20-f7522c31/research/`. They overlap and sometimes
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
| `INDIAN_YELLOW` | 0.8 | S | an early account has it drying in oil "nearly as soon or sooner than" other colors [AP1 p.24]; set near average. Uncertain |
| `BONE_BROWN` | 0.3 | Salter | bone and ivory browns are "bad driers in oil" (Field's *Chromatography*, revised by Salter, 1869, §243); below bone black (0.4). Estimate |
| `CADMIUM` | 0.6 | I (S and T agree; T cites Old Holland's "m") | "slow but reliable driers" [AP1 p.72]. A proposed 0.5 without a source and flagged it for checking; I's 0.6 rests on the quotation |
| `BITUMEN` | 0.15 | I | slows linseed oil's drying and never fully cures [CAMEO]; below madder lake (0.3) |

`INDIAN_YELLOW` went out with the Indian yellow tube in the first
reconciliation and came back with it under the revised rule (below).

## The catalog: new tubes

Columns: masstone, hiding, stiffness, tinting strength, drying.

| tube | pigment | masstone | hid | stiff | str | dry | from | notes |
|---|---|---|---|---|---|---|---|---|
| lead-tin yellow | lead-tin oxide | `#e3cc6a` | 0.85 | 0.75 | 0.6 | 2.0 `LEAD_WHITE` | T | only T. Hiding, body and drying sourced [AP2-LTY pp.91–93]. The brand he uses is not stated, and modern tubes of this name vary (T) |
| zinc white | zinc oxide | `#f3f3ef` | 0.6 | 0.6 | 1.0 | 0.35 `ZINC_WHITE` | S | only S |
| Naples yellow | lead antimonate | `#e2b964` | 0.85 | 0.75 | 0.6 | 1.6 `NAPLES_YELLOW` | A | only A. What an 1880s tube of this name held is unverified (A) |
| lemon chrome | pale lead chromate with lead sulfate | `#eed83c` | 0.8 | 0.7 | 0.8 | 1.8 `CHROME_YELLOW` | I | S: `#f0d32a` 0.85/0.7/0.9. Both say the pale grade hides and tints less than chrome yellow (0.9/1.0). I ties both cuts to the coprecipitated lead sulfate [AP1 p.187], while S marks its cuts as estimates ("assumed slightly lower", "a little weaker"), so I's values. Stiffness and drying agree |
| pale cadmium | cadmium sulfide, a pale grade | `#f0c63c` | 0.85 | 0.7 | 1.1 | 0.6 `CADMIUM` | A, adjusted | A: stiffness 0.6, drying 0.5. It is the same pigment as cadmium yellow, so it gets the same body (0.7) and the sourced cadmium rate (0.6). A's masstone, hiding and strength are kept (they match the other cadmium lines) |
| deep cadmium | cadmium sulfide, a deep grade | `#e8861e` | 0.9 | 0.6 | 1.2 | 0.6 `CADMIUM` | A, adjusted | A's optional tube 9: masstone, hiding, stiffness and strength as A wrote them ("as for pale cadmium, with a deeper orange masstone and slightly higher hiding"; all estimates). A's drying 0.5 is replaced by the sourced cadmium rate, as for pale cadmium. H proposed it later for "cadmium deep": `#e2861e` 0.88/0.7/1.1/0.6, also all estimates and nearly the same. The tube was already in the alma-tadema box, whose paintings must replay as painted, so its numbers stay A's and the hopper box shares it |
| Indian yellow | magnesium and calcium euxanthate | `#e1a11e` | 0.15 | 0.4 | 0.8 | 0.8 `INDIAN_YELLOW` | S | only S. Hiding: particles "uniformly clear and transparent" [AP1 p.22]; stiffness and strength are estimates |
| cadmium yellow | cadmium sulfide | `#e8a51f` | 0.85 | 0.7 | 1.1 | 0.6 `CADMIUM` | I | S: `#efb512` 0.85/0.65/1.1/0.6; T: `#f0b000` 0.9/0.7/1.1/0.6. Strength and drying agree. Hiding: S's 0.85 ("good hiding power" [AP1 p.71]) and T's 0.9 (Old Holland: opaque) are both sourced, so the majority (S, I) gives 0.85. Stiffness 0.7 (I, and T from Old Holland's "l" oil content). The masstone is an estimate in all three, and the tie goes to I, whose drying rate is the sourced one |
| Mars yellow | synthetic iron oxide hydroxide | `#c4872b` | 0.85 | 0.7 | 1.1 | 1.1 `MARS` | S | only S |
| transparent oxide yellow | transparent synthetic iron oxide | `#7a4a14` | 0.2 | 0.5 | 0.9 | 1.0 `RED_EARTH` | T | only T; all estimates |
| brown ochre | iron oxide earth, a darker grade | `#86592e` | 0.8 | 0.7 | 0.8 | 0.8 `OCHRE` | A | only A (it copies yellow ochre's numbers) |
| raw sienna | sienna earth, unroasted | `#9a6a2b` | 0.4 | 0.5 | 0.7 | 1.2 `SIENNA` | I | only I (S proposed burnt sienna only; the sargent box took raw sienna later, at I's numbers) |
| orange chrome | basic lead chromate | `#e0712a` | 0.88 | 0.75 | 0.9 | 1.8 `CHROME_YELLOW` | I | only I; hiding, strength and drying sourced [AP1 pp.207–208] |
| Mars orange | synthetic iron oxide, an orange grade | `#b8602a` | 0.5 | 0.7 | 1.1 | 1.1 `MARS` | Salter | not proposed (S: paint box only). The period account: "a subdued orange of the burnt Sienna class, but without the brown tinge ... with much transparency" (Salter's Field, 1869, §159). Masstone between Mars yellow and Mars red, without sienna's brown (estimate). Hiding 0.5 for "much transparency": near burnt sienna (0.45), well below S's estimates for Mars yellow and red (0.85, 0.9), which are modern opaque grades. Stiffness and strength as Mars yellow (estimates). The Mars colors "dry well in proportion to their depth" (Salter §38), so `MARS` |
| red lead | lead tetroxide | `#e0542b` | 0.85 | 0.8 | 0.8 | 2.2 `RED_LEAD` | S | only S |
| orange vermilion | mercuric sulfide, a yellower grade | `#dd4a22` | 0.9 | 0.75 | 1.0 | 0.4 `VERMILION` | A | only A. The shade is inferred from the name (A marks it uncertain). The body is vermilion's |
| Chinese vermilion | mercuric sulfide, a deeper grade | `#b8282e` | 0.9 | 0.75 | 1.0 | 0.4 `VERMILION` | A | as orange vermilion. A's alternative was to keep only the existing vermilion. The pupil's palette lists both grades by name, so both are kept |
| cadmium red | cadmium sulfoselenide | `#c3321f` | 0.9 | 0.7 | 1.1 | 0.6 `CADMIUM` | S, T | T: `#c2302a` 0.9/0.7/1.2/0.6. Hiding and drying agree. Stiffness 0.7 is T's (Old Holland's "l" oil content); S wrote 0.65 "as cadmium yellow", and the catalog's cadmium yellow is 0.7 too. Strength: both are estimates (S "as cadmium yellow", T "strong tinters"), and the tie goes to S, which keeps it equal to cadmium yellow. Masstone: S's, which is described from a source ("an opaque scarlet" [BAS]) |
| Mars red | synthetic iron oxide | `#a33f2a` | 0.9 | 0.7 | 1.2 | 1.1 `MARS` | S | only S |
| Indian red | nearly pure ferric oxide | `#7a3a33` | 0.92 | 0.7 | 1.2 | 1.0 `RED_EARTH` | I | only I. It is a separate pigment from Mars red (natural haematite, purplish [CH p.161]) |
| permanent alizarin | a quinacridone | `#5e1624` | 0.15 | 0.45 | 1.3 | 0.3 `MADDER_LAKE` | T | only T. He says his alizarin is "quinacridone based"; the exact pigment is not stated (T marks it uncertain). A different pigment from rose madder, so a separate tube |
| rose madder | madder lake on alumina | `#8e2238` | 0.1 | 0.35 | 0.9 | 0.3 `MADDER_LAKE` | S | A: `#7e1f3b` 0.1/0.35/0.6. Hiding, stiffness and drying agree. On strength S cites a source (purpurin-rich madders from 1861 had "about fifty times the tinting strength" of older ones [MMJ53 p.175]) and A gives an unsourced "weaker than a crimson lake", so S's strength is used. The masstone is an estimate in both proposals, and S's is kept with its line |
| magenta | fuchsine (aniline) lake on alumina | `#8f1650` | 0.1 | 0.35 | 1.5 | 0.3 `MADDER_LAKE` | period literature | not proposed (S: a purchase only). Fuchsine (discovered 1858) laked on alumina; a London colorman's "Magenta" of 1896 was an "Aniline Lake" (Carlyle, *The Artist's Assistant* p.506, read through Dootson 2019). Masstone a deep bluish crimson (estimate). Hiding and stiffness as rose madder (a dye lake on alumina; estimate). Strength 1.5, above rose madder: the coal-tar dyes are intense (estimate). Drying at madder lake's rate, as a lake (estimate). It fades in light: "a want of permanence has been fatal to their success. Mauve is more durable than magenta" (Salter, ch. on coal-tar colors). The engine doesn't model fading |
| burnt sienna | roasted sienna earth | `#7c3f24` | 0.45 | 0.55 | 0.9 | 1.2 `SIENNA` | majority | S `#8a4a2b` 0.45/0.55/0.9; I `#7c3f24` 0.45/0.50/0.9; A `#7c3b22` 0.5/0.55/1.0; T `#7b3a1e` 0.6/0.55/0.9. T's hiding cites Old Holland's "opaque" rating but T marks it uncertain, since siennas are semitransparent in thin films. The rest are estimates. Majority by field: masstone I (A's and T's nearly the same), hiding 0.45 (S, I), stiffness 0.55 (S, A, T; T's from Old Holland's "h" oil content), strength 0.9 (S, I, T); SIENNA in all four |
| Mars brown | synthetic iron oxide, roasted | `#5a3a28` | 0.85 | 0.65 | 1.0 | 1.2 `SIENNA` | S | only S. The rate is 1.2 as S proposed: iron oxides dry well but lack the manganese that makes umber fast. It borrows the SIENNA constant for its value |
| bone brown | bone roasted until brown | `#4b3527` | 0.6 | 0.6 | 0.9 | 0.3 `BONE_BROWN` | Salter | not proposed (S: a purchase only). Bone "roast[ed] ... until by partial charring [it] become[s] of a brown colour throughout"; "bad driers in oil"; "the palest of these colours are the most opaque" (Salter §243). Masstone a dark warm brown (estimate). Hiding 0.6, below bone black (0.9), for a partly charred bone that is not all carbon (estimate). Stiffness and strength estimates. Drying: new `BONE_BROWN` |
| bitumen | asphaltum | `#2e2017` | 0.12 | 0.3 | 0.7 | 0.15 `BITUMEN` | I | only I. The engine has no traction crackle from slow films, so this tube is only slow and transparent |
| cerulean blue | cobalt stannate | `#3f82b3` | 0.8 | 0.7 | 0.6 | 1.4 `COBALT_BLUE` | S, T | T: `#2e78b0` 0.8/0.7/0.6/1.4. Hiding: S's 0.7 ("an opaque, greenish blue" [BAS]) and T's 0.8 (Old Holland: opaque, both grades) are both sourced; T's rating is the more specific. Stiffness 0.7 is T's, sourced (Old Holland: low oil content), against S's estimate. Strength and drying agree. Masstone: S's, described from a source ("a light, greenish blue" [BAS]); T's is an estimate |
| ultramarine blue | synthetic ultramarine | `#232a8c` | 0.3 | 0.5 | 1.1 | 0.8 `ULTRAMARINE` | T | S called it "French ultramarine": `#26318c` 0.3/0.5/1.2/0.8. It is one pigment, so it is one tube, under T's plainer name. T's masstone is converted from a published measurement [AP2 p.51]; S's is an estimate. Hiding, stiffness and drying agree. Strength: both read the same quotation [AP2 p.44] ("quite good", S 1.2, T 1.1), and the tie goes to T, whose line has the measured masstone |
| ultramarine ash | natural ultramarine, a pale last extraction | `#7d8aa8` | 0.15 | 0.5 | 0.3 | 0.8 `ULTRAMARINE` | AP2 | not proposed (S: paint box only). The last extraction of natural ultramarine, "containing a high proportion of colorless material and both few and small blue particles ... because of its high degree of transparency, was valued as a pale blue glazing pigment" [AP2 p.39]; a London colorman listed it 1892–1916 [AP2 p.51]. Masstone a pale gray-blue; hiding 0.15; strength 0.3 (mostly colorless matter); stiffness as ultramarine blue; drying `ULTRAMARINE`. All estimates from that description |
| Antwerp blue | Prussian blue on an alumina base | `#26406c` | 0.4 | 0.45 | 1.6 | 1.6 `ANTWERP_BLUE` | I | only I |
| viridian | hydrated chromium oxide | `#1c4a40` | 0.3 | 0.5 | 0.9 | 1.0 `VIRIDIAN` | S | A: `#1f5648` 0.3/0.45/0.7/1.0. Hiding (sourced in S [AP3 p.277]) and drying agree. S's masstone is sourced ("a deep blackish green in oil" [AP3 p.278]). Stiffness and strength are estimates in both, so the tie goes to S, whose line has the sourced masstone |
| emerald green | copper aceto-arsenite | `#23a57a` | 0.6 | 0.6 | 0.6 | 1.6 `COPPER` | S | only S |
| cobalt violet | cobalt phosphate or arsenate | `#7e4c8e` | 0.35 | 0.55 | 0.35 | 1.4 `COBALT_BLUE` | S | only S (T left it out of its box; the tonn box took it later, at S's numbers) |

Not in the catalog, because no box uses them: brown pink (I: an
uncertain reading of one OCR'd line) and titanium white (T: demoted; "I
basically don't use titanium" since 2022).
Mars orange, magenta, bone brown and ultramarine ash were not proposed as
tubes (S: paint box or purchase only). They came in under the revised
rule; their numbers are from the period literature, as the table says.

## The boxes

The rule (revised by the user, 2026-09-29; **to be revisited later**):

- **Historical painters** (Sargent, Inness, Alma-Tadema, Hopper): a tube
  is in the box if the pigment is
  1. **documented in use**: found by analysis, named in his own words, or
     described by a witness; or
  2. **in his working materials**: in a paint box or on a palette he
     painted from, or bought repeatedly; or
  3. **a one-off purchase that nothing contradicts**: bought once, and no
     source says he didn't use it.

  Out: a pigment he is documented as not using, or as rejecting or
  avoiding.
- **A living painter** (Tonn): the box follows his current practice,
  including pigments he uses only rarely now. Pigments he used only in
  the past, owns but doesn't use, or avoids are out.
- The first version of the rule (the same day) took only tier 1 for the
  historical painters; "Reconciled with the notes" records that pass and
  "The revised rule applied" the changes since.
- Each box's left-out pigments are listed below ("Left out under the
  rule"), with their evidence. The list stays in this file; no studio
  gets it.
- A tube in the box from tier 2 or 3 is named in the note by its evidence
  ("in a paint box of about 1884–88", "bought in 1888"), so the painter
  reads how it is documented.

Existing tubes count when the proposal maps a documented pigment to
them. A note's "the tubes here" (§9) maps only to tubes in its box, and
every tube in the box is named in the note's pigment table or in §9
(`scripts/tests/box_notes.sh` checks both on the exported studios). The
default `tube box` is unchanged and stays the default.

### `sargent` (28)

lead white, zinc white, lemon chrome, chrome yellow, cadmium yellow,
Indian yellow, yellow ochre, Mars yellow, raw sienna, Mars orange, red
lead, vermilion, cadmium red, Mars red, red earth, rose madder, magenta,
burnt sienna, Mars brown, bone brown, bone black, cerulean blue, cobalt
blue, ultramarine blue, ultramarine ash, viridian, emerald green, cobalt
violet.

(S's "French ultramarine" is the catalog's `ultramarine blue`: one
pigment, one tube.)

- Existing tubes from S's table: lead white, vermilion, bone black, yellow
  ochre, red earth (red iron oxide in the rose layer and a ground
  [MMX]), cobalt blue and chrome yellow (on palettes [HAM]; S marks it
  ✓).
- Judgment calls:
  - *Chrome yellow and lemon chrome both.* HAM names chrome yellow with no
    grade, and APOLLO and BAS name a pale grade.
  - *Raw and burnt sienna both.* APOLLO finds "sienna" in his paintings
    and doesn't say raw or burnt; a paint box of about 1884–88 held two
    tubes of raw sienna and one of burnt [VCB]. The grade used is
    unknown, so the box holds both (the note maps "sienna → raw sienna
    and burnt sienna"). S proposed burnt only.
  - *Red lead.* It was found in one painting [BAS]. It is documented, so
    it is kept.
  - *Zinc white.* He used it "very rarely" [APOLLO], and it is on palettes
    [HAM]. It is kept.
- Tier 2 (a paint box of about 1884–88, 27 tubes, which he painted from
  [BAS]): Indian yellow ("genuine Indian yellow"), Mars orange and
  ultramarine ash ("natural ultramarine ash"). The user's decision named
  the first two; ultramarine ash has the same evidence (the same
  sentence of BAS), so the rule takes it too. It is its own commit, so
  it can be reverted alone.
- Tier 3 (one purchase each, from a London colorman [NPG]): magenta
  ("Magenta in tubes", 1888) and bone brown (1899). No source says he
  didn't use them.
- Left out: smalt, pale smalt, green earth, Rinmann's green and copper
  green (S: not used as far as the sources go). Prussian blue and umber:
  see "Left out under the rule".

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
    brown and pink"). The note's table row is now a research comment, so
    the painter's copy doesn't name it.
- The single-witness pigments (Antwerp blue, Indian red and lemon chrome
  [SHEL]; orange chrome [MI95]; cadmium [SON]) are in. Each rests on one
  witness, and I ranks all of them among its top three priorities.
- No green is documented, so the box has none.

### `alma-tadema` (15)

lead white, Naples yellow, pale cadmium, deep cadmium, yellow ochre,
brown ochre, orange vermilion, Chinese vermilion, red earth, rose madder,
burnt sienna, raw umber, bone black, cobalt blue, viridian.

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
  - *Raw umber and deep cadmium are in.* The manual adds both to the
    palette and says he "uses very little, if at all" of them [MANUAL
    p.13]. That is on his palette in practice, so under the rule they are
    in. Deep cadmium is A's optional tube 9, new to the catalog; raw
    umber is the tube box's.
  - *Chrome yellow is out.* The same manual rejects it.

### `tonn` (14)

lead white, lead-tin yellow, cadmium yellow, yellow ochre, transparent
oxide yellow, cadmium red, permanent alizarin, burnt sienna, raw umber,
bone black, ultramarine blue, cerulean blue, green earth, cobalt violet.

- T's suggested box less red earth, plus transparent oxide yellow from
  its optional list. Existing tubes from T's table: lead white, yellow
  ochre, raw umber (also standing in for red umber, the nearest tube),
  bone black (for ivory black; the same pigment [OH]) and green earth
  (terre verte, an occasional glaze).
- Judgment calls:
  - *Red earth is out.* It stood for English red, which he recommends in
    limited palettes (2021, 2023) but said in 2021 he hadn't put on his
    palette "in a million years". Not current practice. T kept it; the
    box doesn't. The note's §9 maps nothing to red earth.
  - *Transparent oxide yellow is in.* It was on a posted palette of 2025
    [PAL25], which is current use. T lists it as optional because
    nothing says how often he uses it.
  - *Titanium white is out.* He used it for highlights in 2021 and said in
    2022 that he basically doesn't use it; T demotes it to optional, and
    the materials note treats the 2022 statement as current.
  - *Cobalt blue is out:* "haven't used it in ages" (2021).
  - *Cobalt violet is in* (revised rule: rare current use counts). He
    mentions it in 2020 and in May 2025, "I haven't used cobalt violet
    much" [CVI20; CVI25]. T left it out of its box. The catalog's tube is
    S's, now compiled in with the tonn box too.
  - *Vermilion and zinc white are out.* He owns vermilion but hasn't used
    it, and he avoids zinc.
- English red, cobalt blue, titanium white, vermilion and zinc white are
  named in the note only in research comments, so the painter's copy
  doesn't name them. Cobalt violet is in the note's table ("used rarely;
  mentioned in 2020 and in May 2025").

### `hopper` (13)

lead white, zinc white, pale cadmium, cadmium yellow, deep cadmium,
yellow ochre, red earth, burnt sienna, bone black, cerulean blue, cobalt
blue, ultramarine blue, viridian.

H's proposed box, all tier 1 (named in his record books or his 1959
interview [LED; MORSE]), with no new catalog tube. Lead white, yellow
ochre and bone black are in every build; the other ten were in the
catalog for other boxes and are now compiled in with `box-hopper` too.

- Existing tubes from H's table: lead white (flake, silver and Cremnitz
  white), zinc white (early, and given up for lead white [MORSE; LEV]),
  cerulean blue, cobalt blue, ultramarine blue ("ultramarine"; "permanent
  blue" in his wife's diary), viridian, pale cadmium ("cad. yellow
  light"), cadmium yellow, yellow ochre and burnt sienna.
- Judgment calls:
  - *Deep cadmium* for "cadmium deep" (two entries). The hue isn't
    stated; H reads a deep yellow or orange from a mixture with viridian
    described as "slightly olive". The catalog's deep cadmium (A's, see
    the catalog table) is that.
  - *Light red maps to red earth.* Light red is a red iron oxide (a
    roasted ochre) in period color lists, and the evidence is one
    pigment list in one entry [LED]. H proposed a separate optional
    `light red` (`#b4533a`) but calls its description unsourced and red
    earth "close enough". The alma-tadema box already maps its pupil's
    "light red" to red earth, so the same pigment gets the same tube.
  - *Black maps to bone black.* The one entry says "touch of black" and
    doesn't name the kind; bone (ivory) black was the usual artists'
    black of the period (H's inference). The note labels both red earth
    and bone black "a proxy" in §9.
  - *Burnt sienna is in, uncertain.* Its two mentions may be color
    words rather than pigments [LED]. They name the pigment itself, in
    his own record book, so they count as documented use; brown pink
    (inness) is out because the word itself may not be there. The note
    marks it *Uncertain*.
  - *Zinc white is in.* He used it (record books, a canvas of 1935, his
    own account) before giving it up; the rule counts use at any time
    for a historical painter.
- Tiers 2 and 3 add nothing: no paint box, palette or purchase of his in
  the sources names a pigment (his record books name makers, whites and
  oils).
- Not documented for him in the sources consulted (H): vermilion,
  cadmium red, madder or alizarin, the chromes, Prussian blue, emerald
  green, Naples yellow, umbers, raw sienna, Mars colors, smalt, green
  earth and copper greens. Not documented isn't the same as not used,
  but the rule needs a document, so they're out.
- The fugitive yellow that turned a green blue on one canvas [NGA-E] is
  unidentified; no tube stands for it.

## Left out under the rule

Pigments the notes or research connect with the artist that the box
leaves out, with their evidence. Keys are the research notes'.

- **Sargent:**
  - Prussian blue and umber: listed among the paints on a palette of his
    (the Signet Society palette) only in a painter's blog summarizing the
    society's 2017 newsletter; the museum's own summary of the palettes
    [HAM] names neither, and no analysis of his paintings found Prussian
    blue. The palette would be tier 2 if the list is confirmed (VCB, the
    full paper, is the place to check). Out until then; the note keeps
    them in a research comment.
  - Nothing else: every pigment the note records him owning is now in the
    box.
- **Inness:** brown pink is out: its one witness may read "brown and pink"
  instead [SHEL p.32], so it is not documented at all. No paint box,
  palette or purchase of his names a color. Burnt sienna stays out: the
  witness names "sienna" and "raw sienna" [SON], and no working
  materials of his show a burnt grade (unlike Sargent's paint box).
- **Alma-Tadema:** chrome yellow is out: the manual rejects it ("its
  stability is too doubtful") [MANUAL p.13]. The Mommen and Blockx
  supplies [BARR; NPG-B] name no colors, and the Madderton palette
  (1902–04) [NPG-M] was not found, so tiers 2 and 3 add nothing.
- **Hopper:** H's light red tube (not in the catalog; see the hopper
  box). Nothing he is documented as not using is in the box; he gave up
  zinc white and poppy oil for lead white and linseed oil [MORSE], and
  zinc white is in because he used it before that.
- **Tonn** (past, owned but unused, or avoided):
  - English red (red earth's pigment): recommended for limited palettes
    (2021, 2023) but "haven't even put [it] on my palette in a million
    years" [ERED21, 2021].
  - Cobalt blue: "haven't used it in ages" [BLUE21, 2021].
  - Titanium white: added in 2021 as a second white [WHITE21]; "I
    basically don't use titanium" [TI22, July 2022].
  - Vermilion: owns a tube "that I never used" [VERM20; CAD22].
  - Zinc white: he warns against zinc-containing paints [ZINC20].

## Reconciled with the notes (2026-09-29)

The boxes were changed to follow the first version of the rule
(historical painters: documented use only).

- **sargent:** Indian yellow out (paint box only). Raw sienna in:
  sienna is documented in his paintings [APOLLO], grade unknown, and his
  paint box held raw and burnt [VCB]. The note's §9 maps "sienna → raw
  sienna and burnt sienna". The `INDIAN_YELLOW` drying rate went with the
  tube. Still 23 tubes.
- **inness:** unchanged (13). The note's brown pink row became a research
  comment, since even its reading is uncertain.
- **alma-tadema:** raw umber and deep cadmium in (the manual lists both on
  his palette, used "very little, if at all"); 15 tubes. Deep cadmium is a
  new catalog tube (above). The note's §9 maps both to their tubes.
- **tonn:** red earth out (English red is not on his palette now); 13
  tubes. Red earth is now compiled in only with the boxes that hold it
  (the default, sargent, inness, alma-tadema). Titanium white's table
  row, its clause in the summary and its mention in a drying quotation
  became research comments, like the other pigments he doesn't use now.

## The revised rule applied (2026-09-29)

- **sargent:** 23 → 28. Indian yellow, Mars orange and ultramarine ash
  in (tier 2: the paint box of about 1884–88 [BAS]); magenta (bought
  1888) and bone brown (bought 1899) in (tier 3 [NPG]). Mars orange,
  magenta, bone brown and ultramarine ash are new catalog tubes, Indian
  yellow comes back at S's numbers, and `INDIAN_YELLOW` comes back with
  it; `BONE_BROWN` is new. The note's §4 names each with its evidence
  ("in a paint box of about 1884–88", "bought in 1888") and §9 maps them.
- **tonn:** 13 → 14. Cobalt violet in (rare current use: 2020 and May
  2025). The note's table row says so.
- **inness, alma-tadema:** unchanged. Their notes' research comments
  hold no paint box, palette or purchase naming a pigment (see "Left out
  under the rule").
- **hopper:** a new box under this rule (13, see its section), with its
  note `notes/research/hopper_materials.md`, the export profile `hopper`
  and the feature `box-hopper`.

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
- H notes his turpentine-rich start ("almost pure turpentine" to "pure
  oil"): the easel has no solvent, only the oil-medium share. H also
  notes his whites in poppy oil (1939–45), which would dry slower than
  the catalog's lead white; not modeled, like S's poppy-oil white.
