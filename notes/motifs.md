# Stream 5: motifs out of the engine; trees that grow

Principle: the engine provides physics and tools; motifs belong to the
painter. Knowing how trees grow is not cheating; the engine painting the tree
is. So `paint::tree::Oak` (grew *and* painted one oak, the same for every
agent) is gone. The engine now has `paint::growth` (botany only, returns a
skeleton), and painting a tree is the painter's habit in
`paintings/src/trees.rs`.

## What changed

### Engine: `crates/paint/src/growth.rs` (replaces `tree.rs`)
A 3D bud-based growth model after Palubicki et al. 2009, "Self-organizing
tree models for image synthesis", projected onto the picture plane:
- **Metamers and buds:** each node has an apical bud at shoot tips and
  lateral buds in the leaf axils. Phyllotaxy is a spiral divergence (oak
  144°, birch 137.5°), or a whorl at the end of each annual shoot (spruce).
  Terminal clusters (`tip_buds`) give oak's crowded bud cluster.
- **Light:** a shadow-propagation grid, where the last 3 years' shoots
  shade a pyramid below them. Each bud's light depends on the shadow at
  its position and the species' shade tolerance (`shade`: oak and birch
  need light, spruce doesn't).
- **Vigor:** light is summed from the tips to the base. Vigor comes back
  out from the base by Borchert–Honda: the continuing axis gets
  `λ·Qm / (λ·Qm + (1−λ)·ΣQl)` (`apical` = λ). A bud with vigor ≥ 1
  breaks into a shoot of `floor(v)` internodes (capped).
- **Direction:** every new internode keeps its heading and turns down the
  shade gradient (phototropism, `photo`). It bends up or down by
  gravitropism per order (`tropism[order]`: leaders up, spruce limbs level,
  birch twigs weeping) and wanders (`wander`). `flat` squashes depth so
  crowns spread in the picture plane. Lateral shoots leave at
  `branch_angle` from the parent axis, so every branch inherits its
  parent's direction and deflects from it. Nothing is sprinkled.
- **Sympodial growth:** a terminal bud can abort (`abort`), but only where
  lateral buds sit below it to take over. The strongest lateral then
  continues the limb at an angle, which gives the zig-zag of old oak limbs.
  Some buds sleep and break years later (`dormant`, reiteration).
- **Shedding:** a branch that earns too little light per internode is
  shed. If it was thick, it leaves a stub.
- **Widths:** Leonardo's rule `w^k = Σ w_child^k` from equal tips. Wood
  never shrinks, so a limb keeps its girth after it loses its twigs. `k`
  is solved from `trunk`, `twig` and the tip count, and reported as
  `Skeleton::pipe`. It comes out around 1.5–1.9 rather than 2–3 because
  each modeled tip stands for a small cluster of real twigs. Taper along
  a limb is continuous and never increases toward the tip. The root
  flares into buttress roots that run *into* the ground.
- **Decline (dead oak):** branches die with a probability that rises
  toward the top, and the leader's top dies (stag-headed). Dead wood loses
  twigs below a width set by `decay`, leaving short claws of one or two
  internodes. Dead limbs break (`breakage`, a quarter as often for live
  limbs) at a random point along them, and are flagged `broken`.
- **Skeleton:** limbs are read the way the eye reads them. Each one
  follows the thickest path through every fork, slightly favoring
  straight on, so a sympodial limb is one zig-zag polyline. Each `Limb`
  has `pts`, `z` (depth), `w`, `order`, `parent` + `at` (the index on the
  parent where it springs), `dead`, `broken` and `root`. Limbs are
  ordered parents first. `Skeleton::mask/tips/bounds` are provided.
- Species are data: `Habit::oak()`, `dead_oak()`, `birch()`, `spruce()`
  are the same code with different numbers. Individual trees vary by
  field (`Habit { lean: 0.12, decay: 0.45, ..Habit::dead_oak() }`).
  Output is deterministic by seed; a test checks determinism,
  connectivity, height and widths.

### Painter: `paintings/src/trees.rs`
- `tree(c, &skeleton, &Bark, &TreeHand, seed) -> Mask`. Each limb is one
  continuous movement from where it springs to its tip, trunk first, then
  limbs, then twigs. Where a limb thins past what one brush can do (to
  40%), or is longer than a load lasts, the next brush takes over. It sets
  down **at full pressure inside the previous wet stroke**, which tapers
  off over 1.5 limb-widths, so there is no joint. (A soft attack there
  lifts wet paint and leaves a gap. I found that on a trunk and fixed it.)
  Pressure is solved from the width at each end, from the footprint model
  in `bristle::drag_on`. Round sables are used above 1.6 units, riggers
  below. Live tips lift off to a point. Broken ends stop short and split
  into 3–4 splinters, one longer than the rest, with a touch of pale wood
  on the break (`Bark::wood`). The finest twigs are lean and dry (a haze,
  not a mass). Once the dark is dry, lean dry-brush streaks go down the
  side of big limbs facing `TreeHand::light_from`.
- `grown_spruce(c, &skeleton, needles, seed)`: stem and limbs as above,
  then needles as short strokes hanging from every limb. Their length is
  capped by the limb's own length, so young top limbs make a spire.
- `spruce(c, base, height, Paint, seed)`: the gesture spruce for small and
  distant trees. Its reach now grows linearly from nothing at the tip
  (a straight-sided cone). Brushes get finer as the spire narrows, and the
  leader is a rigger flick lifted to a point. This fixes the finding
  "spruce top doesn't taper".

### Motif APIs take paint, not colors (finding 9)
- `trees::Bark { dark, dead, light, wood }` holds `Paint`s. The spruces
  take a `Paint`. Figures take `Paint`s for every garment. The painter
  mixes them from the palette, e.g. `st.palette.paint(hex("#1f1a16"), 0.25)`
  or `pal.mix(hex(..)).paint(medium)`. Inside the figures, `thin`/`lean`
  derive thinner versions of the given paint, and `figures::mix(a, b, t)`
  knifes two paints together (for rims).
- New `figures::woman(c, at, size, &Gown, WomanPose, seed)`. Friedrich
  women seen from behind: a long high-waisted gown to the ground flaring at
  the hem, a short bodice, close sleeves, hair in a knot, an optional shawl
  with its point down the back, and a rim of light. The three poses are
  `Standing`, `ArmsOpen` (*Woman before the Setting Sun*: arms out and
  down, open hands) and `AtSill(v)` (*Woman at a Window*: leaning, forearms
  on a sill, shoulders and head dropped).
- New `figures::wanderer(...)`: long coat, stick, bare head with blown
  hair, `step` lifting one foot onto a rock.
- Leg/boot gap fixed: each boot is pulled out of the wet trouser leg (shin
  → heel → toe) instead of being a separate dab below it.

### Paintings
- `friedrich_moonrise_valley`: the dead oak is now
  `Habit { lean: 0.12, decay: 0.45, ..Habit::dead_oak() }.grow(..)`,
  painted with `trees::tree`, with light from the afterglow side. The
  spruces and figures take palette paints (the old hiding and stiffness
  are kept).
- `friedrich_monk2`: monk gets palette paints.
- `study_motifs` is now a figure sheet (men, wanderer, women in three
  poses). `study_trees` is new (below).

## API in short
```rust
use paint::Habit;
use paintings::trees::{self, Bark, TreeHand};
let oak = Habit { lean: 0.1, ..Habit::dead_oak() }.grow((115.0, ground), 400.0, seed);
let bark = Bark {
    dark: pal.paint(hex("#1f1a16"), 0.25),
    dead: Some(pal.paint(hex("#26211c"), 0.25)),
    light: Some(pal.paint(hex("#4a4034"), 0.35)),
    wood: Some(pal.paint(hex("#5d5244"), 0.3)),
};
let mask = trees::tree(&mut c, &oak, &bark, &TreeHand { light_from: (0.7, -0.7), ..Default::default() }, seed);
// your own habit: perches, snow on limbs, crows...
for l in oak.limbs.iter().filter(|l| l.order <= 2 && !l.dead) { /* l.pts, l.w, l.dir(i) */ }
let perches = oak.tips();

let sp = Habit::spruce().grow((x, y), 180.0, seed);
trees::grown_spruce(&mut c, &sp, pal.paint(hex("#1c211d"), 0.1), seed);
trees::spruce(&mut c, (x, y), 12.0, pal.paint(hex("#353846"), 0.1), seed); // small/distant

figures::woman(&mut c, (x, y), 60.0, &Gown { dress, shawl: Some(shawl), hair, skin, rim: Some(rim) }, WomanPose::ArmsOpen, seed);
```

## Evidence
All renders are in `~/tmp/motifs-cd4f3fe7/evidence/` (PNG plus a
peeked JPG where noted):
- `before_old_oak_study_motifs.png`: the old engine `Oak`. Claw twigs are
  sprinkled pointing up regardless of their limb, the broken stubs are
  round knobs and the roots dangle.
- `study_trees.png` (`cargo paint study_trees`): three oaks, two birches,
  three dead oaks, two grown spruces and four small gesture spruces.
- `oak_3200.png` + `oak_3200_crown_crop.jpg`
  (`cargo paint study_trees -- --one oak --width 3200 --seed 2`), a crop of
  the crown. **Do the twigs grow out of their limbs coherently? Yes.**
  Every twig springs from a branch at a branching angle to it, keeps the
  branch's general heading, and tapers to a point. Branches narrow at each
  fork (pipe model), and there are no free-floating or reversed claws.
- `dead_oak_1600.jpg`: stag-headed, crooked limbs, claws, broken ends with
  splinters.
- `spruce_1600.jpg`: a narrow cone that ends in a real spire.
- `moonrise_1000.png`: the painting with the grown dead oak.
- `study_motifs_figures.png`: figures, including the women and the
  gap-free boots.

## Known issues / next
- **Ladder ridges on thick limbs:** fine bands across big limbs under
  raking light. The cause appears to be ploughed ridges of stiff paint
  (it already showed on the old oak). More medium (0.25–0.3) reduces it but
  doesn't remove it. The next step would be to look at `bristle` push and
  pickup for long single strokes with big round brushes (stream 2's
  territory).
- **Live oak and birch fringe at 1000px:** it still gathers into small
  tufts at branch ends, a real oak habit that reads as clumps at small
  sizes. It is fine at 3200px. The finest twigs are painted lean, but a
  painter might also skip or merge twigs below a pixel.
- **Limbs are sometimes too smooth ("noodles")**, especially on dead
  oaks. Catmull–Rom through the nodes rounds the sympodial elbows. A
  `Limb` could carry an "elbow" flag at sympodial switches so the painter
  can make a sharper turn there.
- **Grown spruce at large sizes:** the needle strokes bead into dots
  along the limbs, and the stem is too thick and too uniform. The needles
  want longer, overlapping, drooping strokes, and more second-order
  shoots.
- **Birch:** no white bark or dark marks yet. That's a painter habit
  (`Bark` for birch plus a marks pass) I didn't reach. The trunk foot is a
  round cap; a painter would bury it in grass.
- The Leonardo exponent is solved rather than fixed at 2–3, because the
  model has thousands of tips, not the hundreds of thousands a real oak
  has. That trade-off is documented in `Habit::twig`.
- `AtSill` hides the forearms. A window frame and sill are the painter's
  job. The wanderer's `step` is subtle at 0.06.
- Cost: a 1000px sheet of 10 trees takes about 4 s. A single 3200px oak
  takes 15–20 s. Growth itself takes well under 100 ms per tree.
