# Friedrich notes

Goal: new paintings in Friedrich's manner, composed from his vocabulary and
method. Studies of known works (Monk by the Sea, Two Men) are exercises only,
worked from written descriptions, never images.

Working method (no reference images): light ground → brown umber underpainting
for values → body-color sky (Mixbox pigment gradient) → thin KM glazes → very
soft horizontal strokes → figures last → warm varnish + canvas weave.

## Monk by the Sea study (`paintings/src/bin/friedrich_monk.rs`)

What worked
- Pigment-mixed sky gradient warped by horizontally stretched fbm
  (x * 0.35–0.45, y * 1.4–1.6) reads as layered cloud banks, not blobs.
- Mist bank = semi-opaque dark glaze fading up from the horizon, max ~0.6.
  At 0.8+ it swallows the horizon line.
- Pale veil band (gaussian in y around 0.47h) gives the luminous middle sky.
- Sky strokes: Body medium, opacity ~0.12, streak ≤ 0.2, load 1.0. Lower load
  (dry brush) makes scan-line streaks across the whole sky.
- The figure must overlap the horizon so the head reads against the mist;
  near-black #0d0c0b; add a faint contact shadow glaze or it floats.

What didn't
- Canvas weave below ~3px/thread aliases into a grid (now auto-faded).
- Saturated ochre dune looked like a beach resort. Keep sand gray-ochre.
- Scattered single grass blades read as hairs; clump them.
- Gulls as bright V strokes read as checkmarks; keep them soft, ~0.5 opacity.
- Evenly spread whitecaps read as dashes; concentrate near the shore.

Ideas for next
- A proper Rückenfigur generator (coat, hat, stick, poses) instead of
  hand-placed polygon points.
- Oak generator (gnarled, recursive, with twig density falloff).
- Fog that fills valleys by depth, not just by y.

## Engine v1 lessons (painterly pass)
- Flat `paint()` fills read as digital. Lay in thin (~0.85), then cover with
  `fill_strokes`: broad wet-into-wet (pickup 0.2–0.3), then finer, then a dry
  blender at LOW opacity (~0.3). A blender at 0.6 erases every stroke.
- Per-stroke color jitter needs to be ~0.03 OKLab L to survive glazing; 0.015
  disappears.
- Craquelure: hairline (0.012 × cell), random strength per crack segment,
  clustered. Uniform full-strength cracks look like reptile skin / dried mud.
- Relief ~0.7 with impasto 0.3–0.45 gives visible bristle ridges at 3200px.
- Still digital: fbm-glaze clouds look like smoke. Clouds need shape: paint
  them as lit forms with strokes along their contours.

## Fundamentals plan (after research, see notes/research/)

What makes a Friedrich surface, and how the engine should produce it physically:

1. **Physical units.** Canvas in mm, surface height in µm, one coat of wet
   paint = COAT_UM. Relief lighting from true normals.
2. **Support and ground.** Plain-weave linen 10–16 threads/cm (proxy,
   Eckersberg), warp more regular than weft. Grounds in 2–4 thin layers
   (KÖR p.284): lower spatula layers (ocher/red earth/chalk) level the weave;
   the top layer (lead-white-rich, or reddish-ocher as a mid-tone) carries
   brush striations or a fine roller texture that shows through the paint.
3. **Leveling and pooling.** Each drying layer levels by Orchard's law with a
   yield-stress floor (oil_paint_physics.md §1). Very thin fluid paint pools
   in the ground's valleys; composite KM with the *redistributed* thickness.
   This gives Friedrich's dotted, strokeless gradations (CATS p.127).
4. **Palette.** Paints mixed from his pigments: lead white, smalt (low
   hiding, coarse specks), yellow/red ocher, vermilion, bone black, umber;
   after ~1820 cobalt blue, chrome yellow (ALF; NG p.56).
5. **Application.** Graphite underdrawing with ruled lines that shimmers
   through; very thin underpainting; 1–2 paint layers; skies, mist and far
   hills stippled; firs in short hatched strokes; grass flicked up last.
6. **Aging.** Sequential T-junction craquelure (islands ~2–6 mm) with dirt
   and cupped rims; several yellowed varnish layers, microcracked (milky
   veil); abrasion exposing ground on texture peaks; Carus's dark glaze
   toward the edges for moonlit pictures (MET p.35).

## What makes a picture Friedrich's, beyond motifs (round 7 research, sourced)

Why this exists: Alice's verdict on *Evening at a Mountain Lake* was "not very
Caspar" (notes/round6/alice_review.md). The materials report
(notes/research/friedrich_materials.md) covers canvas, ground, pigments and layers.
This section covers the rest: composition, light, sky and water, what he left out,
mood, and how imitations go wrong. It comes from written sources only (catalogue
essays and entries, a technical study, contemporaries' letters, Friedrich's own
notes as quoted in print). No images were viewed. Source keys are listed at the
end of this section. Keys marked *(secondary)* are encyclopedia or quotation
digests that summarize the scholarship (Börsch-Supan, Busch, Hofmann, Sumowski);
use them as pointers, not as final authorities. Koerner, Busch, Grave and the
Börsch-Supan/Jähnig catalogue raisonné were not accessible in full (the archive.org
copies are lending-only), so they appear only as others cite them.

A painter's one-page version is `notes/briefs/friedrich_painter.md`.

### Bottom line
- **An order under the scene.** "Friedrich's landscapes usually adhere to a severe
  underlying symmetry. They are also relatively empty" [MW essay, pp.11–12]. What
  separated him from followers who borrowed his dusk, fog and back-turned figures was
  that "severe underlying order", against Dahl's "Baroque tradition without
  preconceived geometry" [MW cat.13].
- **Near and far with nothing between.** A shallow foreground faces an unreachable
  distance. "The middle ground ... has vanished" [RV Rosenblum, p.11]. Johanna
  Schopenhauer, after a studio visit in 1810: "The air ... takes up more than half of
  the space in most of his compositions. Middle- and background are often missing
  because his motifs don't require them" [MW cat.8].
- **Light is the subject, not the sun.** Twilight, fog, veiled moons, and a light
  that comes from behind things. "Friedrich did not care for strong daylight"
  [RV p.60].
- **Leaving out is the method.** He drew ships, nets and gulls on the *Monk* canvas
  and then did not paint them: "Friedrich noticeably emptied the seascape" [CATS
  pp.128–130].
- **Restraint makes the mood.** His figures stand still and do nothing. "Characterized
  by restraint and the absence of exterior movement, Friedrich's works are free of
  direct displays of feeling" [RV Asvarishch, p.35].

### 1. Composition

**Symmetry, axis and hidden geometry.**
- "Friedrich offers a rigidly frontal view that would arrest all motion and align the
  spectator on a central axis, as if before an altarpiece. Throughout his work, a
  lucid geometric order, both explicit and implicit, reigns" [RV Rosenblum, p.9].
  Even when figures stand off-center, "we nevertheless intuit a secret axis of purest
  symmetry toward which they, and everything around them, will eventually gravitate"
  [same page]. The catalogue calls it his "secret love of symmetry": in *The Dreamer*
  the window opening sits exactly at the center [RV p.78].
- *Abbey in the Oakwood* is described as axially symmetric: the central vertical
  passes exactly through the apex of the ruin's window, and the choir wall is flanked
  by four oaks on each side [WP-de Abtei, "Struktur und Ästhetik"]. The Madrid
  catalogue calls it "a solemn symmetry that gives the composition harmony", which
  "imposes itself despite unsettling motifs such as the shapes of the oak branches,
  which break and twitch to their tips" (my translation) [MAD cat.29, p.129]. So
  **symmetry holds the whole, and irregularity lives inside the parts.**
- The German literature lists his canon: "structured pictorial order through
  symmetries, rows, parallel shifts, geometry of triangle, angle, hyperbola,
  diagonal, emphasis on vertical and horizontal"; separation of fore- and background
  by a *Raumsperre* (a barrier across the space); fore- and background confronted as
  silhouetted planes "with no perspective axis leading into depth"; layers of space
  as a means of distance; and (Busch) the golden section (my translation) [WP-de CDF,
  "Komposition"; secondary]. Busch reads hyperbolas opening upward in the skies of
  *Monk* and *Abbey* as a metaphor for infinity [WP-de CDF, "Mystische Geometrie";
  secondary]. In the Dresden *Two Men Contemplating the Moon*, a golden-section
  vertical and the center horizontal cross at the evening star (Busch, reported in
  [MW cat.1 n.1]).
- The order is abstract first. Sumowski describes a process that starts with "an
  abstract composition of lines and planes" that the landscape motifs then veil
  (my translation) [WP-de CDF, "Mystische Geometrie"; secondary]. Friedrich drew
  ruled lines with ruler, set square and T-square [materials report §3].
- Exceptions exist and are named as exceptions. The 1819 *Two Men Contemplating the
  Moon* "is therefore unusual. The composition is asymmetrical, and the landscape is
  relatively crowded" [MW essay, pp.11–12]. The Berlin *Man and Woman* has an
  "irregular and asymmetrical pictorial construction ... fairly rare in Friedrich's
  work, often characterized by regular geometric arrangements" [WP-en Two Men;
  secondary].

**Foreground cut off from the distance.**
- "Characteristically, Friedrich's spaces offer new and emotionally evocative
  contrasts between a shallow, immediate foreground and an otherworldly,
  unattainable distance beyond ... we have always reached the limits of the material
  world, where we must halt our physical movement" [RV Rosenblum, pp.9–11].
- *Moonrise over the Sea*: shore and sea are "two zones that lie one behind the other
  parallel to the picture plane and make an abrupt break between foreground and
  background" [WP-de Mondaufgang; secondary; my translation]. *Chalk Cliffs on Rügen*:
  "three zones of space arranged parallel to the picture, separated from each other by
  color and motif", with the depth axis given up [WP-de Kreidefelsen; secondary].
  *Wanderer*: two layers, the dark rock and figure set "like a stage flat" before the
  bright panorama [WP-de Wanderer; secondary].
- In the Dresden *Two Men*, "as in many paintings by Friedrich, there is no middle
  ground; the foreground earthly scene is contrasted with the lighted sky"
  [WP-en Two Men; secondary].
- The viewer is often not given a place to stand. Ramdohr complained that to see the
  *Tetschen Altar*'s mountain against the sky that way, the painter "would have had to
  stand several thousand paces away ... From such a distance he would not have been
  able to see any detail" [WP-en Tetschen; secondary, quoting Ramdohr 1809]. An
  exhibition book lists the formal innovation as "unwillingness to construct a
  continuous space from interconnected layers" [same].
- The *Raumsperre* or *Landschaftsriegel* (a barrier across the foreground, such as a
  wall, a railing or a band of plants) blocks the way in [WP-de CDF "Komposition";
  WP-de Frau vor der untergehenden Sonne; secondary]. In *Sisters on the Harbor-View
  Terrace* a balustrade "divides the composition into foreground and background"
  [MW cat.5].

**Repoussoir: sometimes, and never as a guide into depth.**
- The *Monk* "notably lacks a repoussoir", and "the emptiness of the foreground is
  overwhelming" [WP-en Monk; secondary]. It gives up the usual *Rahmenschau* (framed
  view) that Blechen used for his own monk at the sea [WP-de Mönch; secondary].
- Where framing exists, it frames a window, not a path: in *Chalk Cliffs* the front
  zone "opens like a kind of window onto the sea, framed very traditionally by rocks
  and trees" [WP-de Kreidefelsen; secondary]. The critic of evening_lake read its fir
  headland as "an effective repoussoir, drawing the viewer's eye into the luminous
  expanse" (critic_gemini.md). That is a Dutch or Claude device of leading-in, which
  Friedrich mostly refused.

**Horizon and how much is sky.**
- In the *Monk*, the sky above a horizon "drawn as if with a ruler" fills five sixths
  of the surface [WP-de Mönch; secondary]; the English article says "about three
  quarters" [WP-en Monk; secondary]. Either way, most of the canvas. The IRR study
  found the horizon may have been ruled [CATS p.128].
- In *Moonrise over the Sea* "the horizon nearly halves the picture" [WP-de
  Mondaufgang; secondary]. Johanna Schopenhauer: air in more than half of most
  compositions [MW cat.8].
- The horizon line is straight and level. Where it softens, it softens by mist: in
  *Fog Banks* (c.1820), "a strip of light gray mist takes from the horizon its
  character as a sharp limit" [MAD cat.55, p.181; my translation].

**Figures: few, still, central and seen from behind.**
- "Friedrich's figures never look up at the sky, as curious stargazers do. They
  either look straight ahead, or ... they look down, which accentuates their state of
  introspection" [MW essay, pp.11–12].
- He places them "either in the center or ... where the relatively large figures
  stand farther to the left" so the viewer shares their angle of vision [MW cat.1].
  Reworking *Moonrise by the Sea*, he "tightened the composition by placing a large
  boulder center stage" and "by enlarging the figures and moving them to the apex of
  the composition" helped the viewer's contemplation. In the earlier version the
  figures "are smaller and do not extend over the horizon line" [MW cat.7].
- The Rückenfigur is "isolated ... alone or in small groups, holding a dialogue with
  nature without action" (my translation) [WP-de CDF "Rückenfigur"; secondary].
  "Larger and completely motionless figures have become an allegory of yearning"
  instead of "picturesque staffage or ... a measure of scale" [MW cat.8].
- Scale varies. The monk is tiny against the sky [WP-en Monk], while in *Two Men*
  "the figures are larger than in most of the artist's works" [MW essay, pp.12–14]. What
  stays constant is stillness and the axis, not a size.
- The vertical against horizontal layers: in the *Monk* the composition is "a
  horizontal layering; the monk forms the only vertical" (my translation) [WP-de
  Mönch; secondary].

**Compiled, not copied from a spot.** "Friedrich never painted from nature, but he
did compose his paintings ... from drawings that he culled from his 'motif stock'"
and "thought nothing of combining motifs ... from completely different regions"
[MW cat.4]. He kept the moon in the same spot in three versions of one subject,
which "followed his inner vision instead of nature" [MW essay, p.16]. His own rule:
"Close your bodily eye, so that you may see your picture first with the spiritual
eye" (his notes of c.1830) [WP-en CDF; WQ; secondary].

### 2. Light and sky

- **The sun is hidden or gone.** A visitor to the unfinished *Monk* in 1809: "The sky
  is pure and indifferently calm, no storm, no sun, no moon, no thunderstorm" (Helene
  von Kügelgen, my translation) [WP-de Mönch; secondary]. In the *Tetschen Altar*, the
  left golden-section vertical "runs through the middle of the hidden sun" behind the
  mountain [WP-de Tetschener Altar; secondary], and Friedrich's own program says "This
  sun set and the world was no longer able to apprehend the departed light" [WP-en
  Tetschen]. In the *Abbey*, "only the highest part of the ruins and the tips of the
  leafless oaks are lit by the setting sun" [WP-en Abbey; secondary].
- **Contre-jour, with the brightest light at the horizon.** In the *Abbey* the lower
  canvas is diffuse darkness in browns that lighten into the larger sky, and "the zone
  where the two color surfaces meet forms an imaginary horizon with the clearest
  light" [MAD cat.29, p.130; my translation]. In the Berlin *Man and Woman*, "the
  light of early dusk creates a contrast between the dark foreground and the luminous
  infinity of the sky" [MW essay, p.16].
- **Twilight and fog as distance.** "To ordinary viewers, Friedrich seemed to make his
  landscapes even more inaccessible by immersing them in twilight, fog, or ...
  impending darkness", Novalis's alienation that "turned the familiar into the
  unfamiliar" [MW cat.8]. Friedrich to Carus: "When a landscape is enveloped in mist
  it appears larger, more majestic, and increases the power of imagination" [WQ,
  citing Hinz 1968; secondary].
- **Veiled moon.** "Friedrich's preference for a veiled, partially hidden moon. He
  rarely painted a starkly full moon, but usually covered it partly with clouds"
  [MW essay, pp.13–14]; "in his paintings, Friedrich usually kept the full moon
  partially veiled with clouds" [MW cat.9].
- **One light, one time of day, the whole picture in it.** The Dresden *Two Men*: the
  moon "bathes the landscape and sky in an all-pervasive, rust-brown haze" [MW
  cat.1]. Carus, who knew Friedrich's studio, attacked painters who "deck out the sky
  in all the colors that it displays after sunset, while bathing the foreground in
  bright sunshine" [CAR p.125].
- **Skies built in bands and wedges, and worked hardest.** An oil study of 1824 shows
  "a formation of cloud and light [that] cuts diagonally across the whole surface,
  like a wedge. Below it narrower bands of cloud, parallel to the horizon" and the
  catalogue calls it a scheme "that appears in later works" [MAD cat.74, p.219; my
  translation]. "Skies always demanded his most intense concentration" [MW essay,
  p.19]. He refused Goethe's 1816 request to illustrate Howard's cloud types: "the
  reducing of clouds to a rigid meteorological classification was tantamount to an
  overthrow of landscape art" [MW cat.11].
- **Gradations.** A sky over a Baltic view "goes without break from pale orange at the
  center of the horizon to pale blue above, where thin clouds of pale yellow and deep
  orange set an accent in the middle of the sky" [MAD cat.72, p.217; my translation].
  Technically the smoothness is thin paint pooling in the ground, not blending
  [materials report §6].

### 3. Sky and water

- **Water is calm.** "The artist's painted seascapes always show calm waters", even
  though he roamed the Rügen cliffs in high wind [MW cat.7].
- **The water repeats the sky, fainter.** In *Moonrise over the Sea* the moon's
  "shades of brilliant yellow, violet, and blue" in the sky "are echoed more
  transparently on the sea below" [MW cat.7].
- **Mirrored geometry, not mirrored objects.** In the *Monk*, "the water acts as the
  negative form of the shore strip, which the sky, tearing open above, takes up in
  mirror image. The increase of brightness is likewise mirrored at the horizon line"
  [WP-de Mönch; secondary; my translation]. *Moonrise*: the horizon is "drawn like a
  coordinate between two mirror-image hyperbolic curves", the opening in the cloud
  bank above and the silhouette of the stones below [WP-de Mondaufgang; secondary].
- **Value in the water moves on its own.** In *The Stages of Life* "the water darkens
  toward the horizon and reaches a deep blue, then lightens slightly near the
  horizon", and "this principle of composition rules the whole canvas" [MAD cat.89,
  p.249; my translation].
- Alice on evening_lake: "The lake and the sky look very similar ('I get it, it kind
  of should be')". The sources suggest the water should echo the sky's colors
  **more transparently and in its own value order**, not reproduce it.

### 4. Palette, tonality and layering (beyond the materials report)

- **One dominant tonality per picture, near monochrome.** "His very choice of
  subject—whether moonlit night, sunset, or morning—calls for a single dominant
  tonality that sometimes borders on monochrome" [RV Asvarishch, p.35]. The 1819 *Two
  Men*: "The blue green of the two men's garb is the only other color in the
  near-monochrome haze of rust and brown" [MW essay, pp.13–14]. Early oils look "dry and
  almost monochrome", later ones "lighter and also pastose" (my translation) [WP-de
  CDF "Arbeitsweise"; secondary].
- **Zones of color, not one unifying atmosphere.** Goethe complained that Friedrich
  "does not strive to adjust his colors to one another or to create a harmony" [MW
  essay, pp.13–14]. The *Tetschen* literature notes "the absence of a unifying tonality"
  [WP-en Tetschen; secondary]. A Rügen view is "divided into four zones" of color:
  red-brown field, deep green hill, blue distant valley, evening sky [MAD cat.72,
  p.217]. So: **one tonal key, with each zone holding its own color**, rather than an
  aerial perspective blending everything into everything.
- **One small accent.** *Chalk Cliffs*: "the color triad green-white-blue is raised by
  the woman's red dress" [WP-de Kreidefelsen; secondary]. The *Two Men*'s blue-green
  garb [MW essay, pp.13–14].
- **Layers thin from top to bottom.** "Layer on layer [applied] like a glaze, thinning
  from top to bottom and tinting the underdrawing's scaffold with the local color";
  "foreground and background of a painting are often treated differently" (my
  translation) [WP-de CDF "Arbeitsweise"; WP-de Abtei "Malweise"; secondary]. In *The
  Sea of Ice* the far elements are "less detailed than the foreground ... from single
  delicate strokes on the blue-in-blue horizon the eye assembles the object" [HH].
- **Precision is kept, even in shadow.** "Nothing is incidental in a picture ... The
  proper subordination of the parts to the whole is not achieved by neglecting
  incidental features, but by correct grouping and by the distribution of light and
  shadow" (Friedrich) [WQ, citing Friedenthal 1963, pp.33–34]. The drawing is "rich in
  detail" with "no gradation of detail according to significance" [materials report
  §3, CATS p.132].

### 5. What he left out

- **Four things.** The *Monk* "makes do in an elementary way with only four objects:
  monk, beach, sea and sky" (my translation) [WP-de Mönch; secondary].
- **Removed things.** *Monk*: three large, precisely drawn ships and fishing nets,
  never painted: "thus emptying the seascape radically" [CATS abstract; pp.128–130].
  Twenty small gulls were added last, instead [same].
- **"That is all."** A visitor in 1817 described his pictures: a beach, the sea,
  clouds, a young man walking and "observing the waves and changing colors, that is
  all" and a funeral procession, "nothing else". "His paintings are actually lyrical
  poems" [MW Monrad, pp.23–24, quoting Peder Hjort].
- **A bare foreground.** Friedrich's view of Dresden keeps "the foreground zone ...
  starkly bare", while "the reality-wedded Carus dotted the foreground with plants and
  flowers" [MW cat.10]. On a planned cross by the Baltic: "there is no church in it,
  not a single tree or plant, not a blade of grass" (letter to Louise Seidler, 1815)
  [WQ, citing Rosen and Zerner 1984, p.63].
- **No action, no story.** Helene von Kügelgen missed "the consolation that movement
  or narrative might provide" [WP-en Monk; secondary]. In the far-land pictures of
  1824–28 he "breaks completely with staffage" and shows "spaces of deliberate
  emptiness" (my translation) [WP-de CDF "Fernmotive"; secondary].
- **Not more than can be seen.** Of another painter's moonlit landscape: "one sees
  more than one would wish, or that can actually be seen by moonlight" [WQ, citing
  Friedenthal 1963, p.33].
- **No storm, no strong sun.** Calm seas [MW cat.7]; "did not care for strong daylight"
  [RV p.60]; the *Monk*'s final sky had "no storm, no sun, no moon" at one stage
  [WP-de Mönch].
- **Not every picture is empty.** "Friedrich's relative empty landscapes fall into two
  main categories: either they serve as views toward an infinity where light is the
  object to be contemplated or they serve as settings for more tangible objects of
  contemplation, such as ruins, churches, crosses, sailing ships, or ... trees"
  [MW cat.6]. Emptiness is organized around one object or around the light.

### 6. Mood, and how restraint makes it

- Contemporaries called his pictures "odd," "barren," "monotonous," "mysteriously
  religious," "melancholy and desolate" and "affecting the heart more than the eye"
  [MW essay, p.20; cat.8].
- Kleist on the *Monk*: "since in its monotony and boundlessness it has no foreground
  except the frame, when viewing it, it is as if one's eyelids had been cut away"
  [WP-en Monk; secondary]. Hofmann: an "aesthetics of monotony" against the older
  "aesthetics of variety" [same].
- Friedrich: "Every truthful work of art must express a definite feeling ... rather
  than try to unite all sensations" [WQ; secondary]. And from a 1809 letter: "If a
  painting has a soulful effect on the viewer ... then it has fulfilled the first
  requirement of a work of art. However bad it might be in drawing, color, handling"
  [WQ, citing Börsch-Supan/Jähnig 1973 pp.182–83, trans. Britt].
- Symbols stay open. Rosenblum warns against reading him "as a rebus to be solved";
  "one of the most potent aspects of his genius is to transcend a one-to-one reading
  of conventional symbols" [RV Rosenblum, pp.14–16]. Of the *Cross on the Baltic*:
  "for those who see it that way, a consolation, and for those who don't ... just a
  cross" [WQ, citing Rosen and Zerner].

### 7. How imitations go wrong (with receipts)

1. **Borrowing the motifs without the order.** Dahl took from Friedrich "the
   mysterious, mood-enhancing effects of dusk, twilight, and fog" and "two figures,
   seen from the back, at the water's edge", but these were "certain superficial
   similarities": Friedrich's landscapes "were usually subjected to a severe
   underlying order, while Dahl's were still rooted in a Baroque tradition without
   preconceived geometry" [MW cat.13].
2. **Small figures for scale and anecdote.** Dahl's "smaller, decorative figures ...
   provide a sense of scale and add a note of cozy domesticity" [MW cat.13]; his
   mother and child wave at a returning boat, a story told in his letter [MW cat.14].
   Friedrich's figures are motionless allegories [MW cat.8].
3. **A busy foreground.** Carus "dotted the foreground with plants and flowers" where
   Friedrich left it bare [MW cat.10].
4. **Drama in the sky.** In a Dahl study, "the drama is confined to the sky" of fast
   clouds and a disintegrating halo [MW cat.12]; Dahl's late skies used "free and
   adventurous brush strokes ... far away from the purity and intensity of
   Friedrich's oeuvre" [WP-en Dahl; secondary]. Friedrich's waters are calm [MW cat.7].
5. **A bright bare moon at dead center.** Gille's moon, "bright as a penny, hangs in
   the exact center" [MW cat.16], against Friedrich's veiled moon [MW cat.9].
   Friedrich placed centers by geometry, but veiled the light.
6. **Figures looking up.** "Friedrich's figures never look up at the sky, as curious
   stargazers do" [MW essay, pp.11–12].
7. **Formulas instead of forms.** Carus on the "handy cursive script that had been
   devised to represent cloud forms, 'foliage,' ocean waves, or mountain slopes", and
   copies that give "paintings that remind us only of paintings and never of nature
   itself" [CAR p.123]. Friedrich: "Spiritual affinity leads to similarity in work, but
   such affinity is something entirely different from mimicry" [WQ, citing Friedenthal
   1963, p.32].
8. **Symbol stacking.** Reading or painting the pictures "almost as if it were a
   Rosetta stone" [RV Rosenblum, p.14]. The Dresden *Two Men*'s fir, dead oak, rock and
   branch are already "theatrical props" to modern eyes [MW essay, pp.11–12]; that
   picture is the exception, and "relatively crowded".
9. **Taking the *Monk* as the norm.** The *Monk* "stands alone in the œuvre" and
   "remained unfollowed" [RAD, abstract and p.1]; Friedrich did not return to "the formal radicalism of
   the *Monk*", except in *The Great Enclosure* (my translation) [WP-de Mönch; secondary]. Most of his
   pictures are symmetric, ordered and hold one object of contemplation [MW cat.6].
10. **Illogical light.** A sunset sky over a sunlit foreground [CAR p.125].
11. **Anachronistic materials.** A landscape was de-attributed partly because its
    leaves use cobalt blue with chrome yellow, anachronistic for 1798/99 [materials
    report §9, MÄD p.102].

### 8. Tensions to keep in mind
- *Crisp or soft?* Ramdohr: "every little twig, every needle on the firs ... the outer
  silhouette is completely exact" [WP-en Tetschen]. Yet the horizon dissolves in mist
  [MAD cat.55], distance is less detailed [HH] and skies are stippled [materials §6].
  So edges are **exact where a form stands against the light and lost where the
  distance goes into haze**, which is a decision, not a filter. This bears on the
  r7 edges stream: making every contour soft would be as wrong as making every contour
  equally hard.
- *Symmetric or not?* Mostly symmetric [MW essay; RV Rosenblum], with named
  asymmetric exceptions [MW essay on the 1819 *Two Men*].
- *One tonality or none?* One key per picture [RV Asvarishch, p.35] but no
  harmonizing of zones [MW essay pp.13–14; WP-en Tetschen].
- *Monk's sky share* is "five sixths" in one source and "three quarters" in another
  [WP-de Mönch; WP-en Monk]. Neither was measured by me.

### 9. Evening at a Mountain Lake against these (from its notes, not its image)
Read from notes/paint1/evening_lake.md and the two critiques, not from the picture:
- **No axis.** The headland's skyline "falls on a diagonal from the upper left"; the
  range "rises to a dome right of center"; the man stands at x 802 of 1000; the glow at
  x 600. That is a Baroque diagonal composition, Dahl's mode [MW cat.13].
- **A walkable middle ground.** Near bank, headland, lake, range and sky are layered as
  a continuous space with a repoussoir leading in (critic_gemini.md), where Friedrich
  cuts the near from the far [RV Rosenblum, pp.9–11].
- **Too many things, equally explicit.** Firs, spires, spit, reeds, range, mist, cloud
  bank, moon, man, stick, two stones and grass. The critic: "make fewer things equally
  explicit" (critic_astra.md). Friedrich: "that is all" [MW Monrad].
- **A small figure at the side.** A figure off the axis, small and below the horizon
  band, is staffage [MW cat.7, cat.13].
- **A bare, crisp moon in open blue**, where he usually veiled it [MW cat.9], and
  clouds that echo its crescent (Alice).
- **Light written as a formula** (a Gaussian glow at a sun position) rather than one
  brightest band where the dark land zone meets the light sky zone [MAD cat.29].

### Sources (round 7 section)
- **[MW]** Rewald, S. (ed.), *Caspar David Friedrich: Moonwatchers*, Metropolitan
  Museum of Art 2001; essays by Rewald (pp.9–21) and Monrad (pp.23–29), catalogue
  entries by Rewald. Full text: https://archive.org/details/CasparDavidFriedrichMoonwatchers
  (the Met PDF, https://resources.metmuseum.org/resources/metpublications/pdf/Caspar_David_Friedrich_Moonwatchers.pdf,
  has no text layer). Essay pages are from the djvu text's page markers and may be off
  by one; catalogue numbers are exact. This is [MET] in the materials report, whose
  "p.35" is a PDF page.
- **[RV]** Rewald, S. (ed.), *The Romantic Vision of Caspar David Friedrich: Paintings
  and Drawings from the U.S.S.R.*, Metropolitan Museum of Art and Art Institute of
  Chicago 1990; essays by Robert Rosenblum and Boris Asvarishch.
  https://archive.org/details/the-romantic-vision-of-caspar-david-friedrich.-paintings-and-drawings-from-the-ussr
- **[MAD]** Hofmann, W. (scientific dir.), *Caspar David Friedrich: Pinturas y
  dibujos*, Museo del Prado, Madrid 1992 (Spanish; translations mine).
  https://archive.org/details/caspar-david-friedrich.-pinturas-y-dibujos
- **[CAR]** Carus, C. G., *Nine Letters on Landscape Painting*, trans. D. Britt, Getty
  2002, letters 7–9 (pp.123–125 cited).
  https://humanities-web.s3.us-east-2.amazonaws.com/german/prod/2021-01/Carus%2C%20Carl%20Gustav%20-%20Nine%20Letters%20on%20Landscape%20Painting%20-%20letters%207-9%20%281%29.pdf
- **[CATS]** Mösl, K. and Schneider, F., "Romantic icons: a technical study of the
  underdrawing for Caspar David Friedrich's Monk by the Sea and Abbey in the
  Oakwood," *CATS Proceedings III*, pp.124–133.
  https://pure.kb.dk/ws/portalfiles/portal/10104814/CATS_proceedings_III.pdf
- **[HH]** Hamburger Kunsthalle, *Das Eismeer* GigaPixel commentary.
  https://cdfriedrich.de/gigapixel/
- **[RAD]** Radnóti, S., "Being and Nothing: Caspar David Friedrich: The Monk by the
  Sea," *Acta Historiae Artium* 59 (2018). http://real.mtak.hu/91506/7/170.2018.59.1.7.pdf
- **[WQ]** *(secondary)* Wikiquote, "Caspar David Friedrich" (quotations with their
  print sources: Friedenthal 1963; Rosen and Zerner 1984; Britt's translation of the
  1809 letter; Hinz 1968). https://en.wikiquote.org/wiki/Caspar_David_Friedrich
- **[WP-de ...]** *(secondary)* German Wikipedia: "Caspar David Friedrich" (sections
  Arbeitsweise, Komposition, Fernmotive, Rückenfigur, Mystische Geometrie), "Der
  Mönch am Meer", "Abtei im Eichwald", "Kreidefelsen auf Rügen", "Wanderer über dem
  Nebelmeer", "Mondaufgang am Meer", "Tetschener Altar", "Frau vor der untergehenden
  Sonne". https://de.wikipedia.org/wiki/Caspar_David_Friedrich and the article
  titles under https://de.wikipedia.org/wiki/
- **[WP-en ...]** *(secondary)* English Wikipedia: "Caspar David Friedrich", "The Monk by the Sea", "Cross in the
  Mountains" (Tetschen), "Two Men Contemplating the Moon", "The Abbey in the Oakwood",
  "Johan Christian Dahl". https://en.wikipedia.org/wiki/The_Monk_by_the_Sea etc.
- Materials report keys ([MÄD], [materials §n]): notes/research/friedrich_materials.md.
