# Charcoal: vine (willow) and compressed

Research notes for the easel's `charcoal()` (paint's `charcoal.rs`). Compiled 2026-11-15 for gap 11 of the Cassatt support plan (notes/cassatt_support.md on branch `notes/cassatt-materials`). Cassatt's preparatory drawings for her prints are "compressed charcoal and graphite over some charcoal lines" (Met, *Maternal Caress* drawing; cassatt_materials.md §4).

**Tags:** **[M]** measured, cited. **[P]** practitioner or museum description, cited, no number. **[E]** estimate; tunable.

**Bottom line**
- Both kinds are carbon in the tooth, held by nothing but the paper's texture (vine) or partly by a gum or wax binder (compressed). In the tooth charcoal behaves like a black pastel: loose particles that the finger, the eraser, a puff and a dry brush move, and that fixative binds. So the easel models it with the pastel stick (`pastel.rs`) and gives the stick two new dials: how readily the eraser takes what it lays (`lift`), and what share of it the binder holds (`grip`).
- **Vine:** no binder, a grey-black powder, the easiest dry medium to erase and to smear. In the easel: lift 1 (graphite 0.9, black chalk 0.72, pastel 0.55), grip 0, R∞ ≈ 0.045 [E].
- **Compressed:** binder-held, "ultra dark", smears and erases less, harder to remove completely. In the easel: lift 0.35, grip 0.5, R∞ ≈ 0.022 [E].
- **Fixative** darkens it a little. The resin fills the air gaps between the particles, and lighter values shift most [P]. The easel uses pastel's fixative (a light spray wets 0.15 of the particles), which darkens a charcoal deposit a few percent.
- No measurement of charcoal *on paper* was found: no particle-size distribution of a drawn mark, no friction or erasure numbers. Every dial is an estimate placed against the qualitative order the sources agree on.

---

## 1. What it is

- **Vine and willow charcoal** [P]. These are twigs of willow or vine "heated at a high temperature in an enclosed vessel without oxygen"; the stick has no binding agent, its "minute, splinter-like particles" crumble onto the support, "the medium is easily manipulated", it "requires paper with some texture" to hold, and "to prevent smudging many artists apply a fixative" (Met, Drawing: Charcoal). "The sticks don't contain any binder, so they will be dusty and brittle in texture" (Fine Art Tutorials). Vine "comes in shades of gray"; willow is "darker in color than vine charcoal" (Wikipedia, Charcoal (art)). The easel treats willow as vine (`kind="willow"` is accepted as vine).
- **Compressed charcoal** [P]. "Held together by a gum or wax binder"; "intensity of the shade is determined by hardness": softer grades have less binder and are blacker (Wikipedia; Fine Art Tutorials). One historical recipe is burnt birch, clay, lamp black and a little ultramarine (Wikipedia), so it can be bluish ("blue-black" PITT, Fine Art Tutorials). It is "usually harder in texture than willow and vine charcoal" (Fine Art Tutorials).
- **Stick sizes** [P]. Vine and willow come as "long, thin sticks" in several thicknesses, up to "tree sticks"; compressed as sticks and blocks (Fine Art Tutorials). No source gives diameters. Easel defaults [E]: vine round Ø6 mm and 150 mm long (2–16 mm allowed); compressed square 7 mm and 70 mm long (3–15 mm).

## 2. Optics

- **Charcoal powder reflects under 5 % in the visible**, flat, rising into the near infrared [M] (cometary-dust analogue study, arXiv 2110.05832: "low reflectivity, which remains below 5% in the visible domain"). Charcoal reflectance under oil (%Ro, 0.06–4 %) is a different, specular quantity from polished sections, and isn't used.
- **Grey versus black.** Vine reads grey on paper for two reasons. Its marks are lighter and broken, so paper shows between them, and its loose powder scatters at every carbon–air surface [P: "aren't as dark as compressed charcoal"; vine "comes in shades of gray"]. Compressed is "ultra dark black" and "matte" (Fine Art Tutorials).
- **Easel values [E].** The dry R∞ of the stick, linear: vine 0.045, a neutral grey-black just under the measured 5 % ceiling; compressed 0.022, slightly blue (0.022, 0.022, 0.026), at about black chalk's 0.022. Both are matte: the drawing has no sheen in the easel (graphite's sheen isn't modeled either, only its gray).
- **Under fixative** [P]. "The fixative fills the air gaps between particles", making "light grey values appear slightly darker"; "slight darkening of the lighter values" is the common change, and heavy spraying darkens more (Fine Art Tutorials, fixatives guide; ArtParts Fine Arts). The AIC Paper Conservation Catalog lists "saturation changes", meaning darkening, of the media when a consolidant or fixative is added, and "compaction or saturation of the media" as a risk of fixing charcoal (PCC ch. 23, Consolidation/Fixing/Facing; ch. 3, Media Problems). The easel's fixative is pastel's, by Kubelka–Munk with scattering cut over the share wetted (`fix_pastel`, wet 0.15). For a charcoal deposit of R∞ 0.045 that is about 7 % less reflectance [D: ks = (1−r)²/2r = 10.2, ÷0.88 → r′ = 0.042].

## 3. Adhesion: what holds it

- **Vine: only the tooth** [P]. The particles are "relatively large" and there is no binder (Met). "Large, loosely adhering particles" sit on the surface and are "extremely prone to smudging and lifting" (artist guides, search summary). Easel: all of its deposit is loose (grip 0).
- **Compressed: the binder too** [P]. The binder "creates adhesion beyond mechanical trapping"; the marks are "more permanent than vine charcoal" (Fine Art Tutorials). Sources disagree on how much it smudges. Most say it resists smudging. Fine Art Tutorials says "just like natural charcoal, compressed charcoal is smudgeable and dusty", though it makes less dust than vine. Easel [E]: half of what it lays is bound as it lands (grip 0.5), so the finger, a puff and a dry brush move only the other half.
- **Particle size** [P, uncertain]. The Met calls vine's particles "relatively large" and "splinter-like". One unsourced blog gives vine fragments "from several hundred microns down to fine dust", and compressed charcoal as fine particles packed densely. A powdered charcoal is sold "milled to ... 100µ" (Art Discount). A DTIC report on carbon-loaded paper found coarser carbon particles "came out of the paper very easily" [M, qualitative]. The easel keeps pastel's particle thickness (6 µm) and crumb law for both kinds: particle size isn't a dial (not modeled).

## 4. Erasing

- **Vine is the easiest dry medium to erase** [P]: "easy to erase", "soft, liftable properties" (Fine Art Tutorials); vine and willow "lift off easily by dusting or erasing" and are "favored ... for preliminary sketches" (Wikipedia). It also comes off with a chamois or a dry brush (Wikipedia: blending with "fingers, tortillons or a chamois", lifting with a kneaded eraser).
- **Compressed is harder to erase** [P]: "more difficult to erase" (Fine Art Tutorials); it "can still be lightened" but resists erasure (search summary of artist guides).
- **Graphite** sits between them in practice: it erases well but burnishes into the fibers and leaves a ghost (the easel's existing model, lift 0.9).
- **Easel values [E].** The kneaded eraser's take per pass is `strength × lift × reach` (graphite.rs `erase`). Vine lift 1.0, compressed 0.35. Compressed's bound half also sits under the eraser's floor: the eraser lightens its loose half and leaves the rest as a stain. Charcoal that the finger has smeared is laid down again at pastel's lift (0.55): smudged charcoal is pressed into the tooth and is harder to erase [P: common practitioner advice; not cited to a measurement].

## 5. Smearing and dust

- The finger, a stump or a chamois blends charcoal (Wikipedia). Vine smears much more than graphite, and compressed somewhat less than vine [P, §3]. In the easel the finger picks up 22 % of the loose deposit per pass (pastel_blending.md §3.1, measured for a dry residue). Vine's deposit is all loose and compressed's half, so the finger carries about half as much compressed charcoal as vine, as a share of the line. Graphite isn't moved by the finger at all in the easel (its smearing isn't modeled).
- A puff and a tap take loose crumbs (pastel_removal_air_blade.md) and leave what is bound. Vine is "dusty" (Fine Art Tutorials), and the conservation catalog calls charcoal's "extreme friability" the problem with treating it (PCC ch. 3).

## 6. Over and under other media

- **Paint over charcoal** seals it, as with every drawing (graphite.rs).
- **Pastel over unfixed charcoal** [P]. Charcoal dust muddies pastel laid over it. In the easel, pastel laid on loose charcoal mixes with it by volume in the tooth's loose layer (`Cell::add_pastel`), so it comes out greyer. Over fixed charcoal it lies on the bound crust and keeps its color. Not modeled: the stick picking up charcoal and dirtying.
- **Partial fixing.** "A drawing may be only partially fixed, the lower layer being fixed before the last application of charcoal" (PCC ch. 3). The easel does this with `fix(mask)` or a fix between passes.

## 7. Deposit [E]

Not measured. The pastel stick's Archard law lays K·F·s/σp. The easel uses vine σp 4 MPa, K 0.05, and compressed σp 5 MPa ("harder in texture") with K 0.07 (a friable compact). Compressed therefore lays slightly more volume at a given force, and darker. Both are inside pastel.rs's ranges (σp 2–30 MPa, K 0.01–0.06; compressed's K just above, as more friable than a pastel).

## Sources

- The Metropolitan Museum of Art, "Drawing: Charcoal" (materials and techniques), metmuseum.org/about-the-met/collection-areas/drawings-and-prints/materials-and-techniques/drawing/charcoal (quoted via search excerpt; the page itself returned 429).
- Fine Art Tutorials, "The Best Charcoal for Drawing: A Review", finearttutorials.com/guide/best-charcoal-for-drawing/; and "The Best Charcoal and Pastel Fixatives", finearttutorials.com/guide/best-charcoal-and-pastel-fixatives/.
- Wikipedia, "Charcoal (art)", en.wikipedia.org/wiki/Charcoal_(art).
- American Institute for Conservation, Book and Paper Group, *Paper Conservation Catalog*: ch. 3 "Media Problems" (1985) and ch. 23 "Consolidation/Fixing/Facing" (1988), cool.culturalheritage.org/coolaic/sg/bpg/pcc/ (quoted via search excerpts; ch. 23 is a scan).
- NEDCC, Preservation Leaflet 7.5, "Conservation Treatment for Works of Art and Unbound Artifacts on Paper", nedcc.org.
- Cometary dust analogues for physics experiments, arXiv:2110.05832 (charcoal powder reflectance < 5 % in the visible).
- ArtParts Fine Arts, "Fixative Spray for Charcoal: A Professional Guide", artpartsfinearts.com.au.
- Art Discount, "A Beginner's Guide to Artists' Charcoal", artdiscount.co.uk.
- DTIC ADA147673, "Preparation of Paper Containing Activated Carbon".
