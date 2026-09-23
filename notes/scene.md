# Scene: one world, one sun

Branch `scene`. This stream answers what the user saw in amnesia round 2
(`notes/amnesia2.md`, item 11 and the coast and mountains notes):

- On the coast, the poles' reflections ran straight down while the boulder
  was lit from the side. The picture had two light logics.
- The boulder looked pasted on, and the mountains' tor block floated.
  Nothing grounded where they rest.
- Painters worked perspective out by hand (winter FRICTION 8: a brook's
  width in perspective).

The cause: each `Form` had its own `Light`, and nothing knew where the
ground, the eye or the sun was. Now a `World` holds all three, and every
solid, shadow, contact and reflection takes them from it. Like `form`, it
paints nothing. The study that shows it paints with brushes.

## What changed

- `crates/paint/src/scene.rs` (new; `pub mod scene` in `lib.rs`, plus
  re-exports of `Spot, Sun, View, Water, World`):
  - **Camera.** A pinhole eye `eye` meters up, looking level, with the
    horizon at canvas y `horizon` and a horizontal field of view (default
    45°). The view can be a panel `[x, y, w, h]` of the canvas; outside it
    the world is `What::Off`. Methods:
    - `project(X, Y, Z)` maps a world point to the canvas;
    - `to_ground(x, y)` maps a canvas point to the ground (closed form on
      level ground, a march and bisection over a height function);
    - `ray` gives the line of sight through a canvas point;
    - `scale_at(Z)` gives units per meter at a depth;
    - `height(x, y, m)` gives how tall something `m` meters tall looks
      standing there. A figure as tall as the eye has its head on the
      horizon (tested).
  - **Ground and water.** `ground(|X, Z| height)` takes any gentle height
    function. `Water::new(level)` fills the ground's hollows below that
    level: a pond is a hollow and a shore is ground that runs under the
    sea. `.ripple(slope, across, deep, seed)` perturbs the water's normal
    with long crests across the picture. Queries: `surface`, `is_water`,
    `water_depth`, `ground_normal`.
  - **One sun.** `Sun::deg(azimuth, elevation)`. Azimuth 0 puts the sun
    straight ahead (contre-jour), +90° on the right, −90° on the left, 180°
    behind the painter. `World::sun(..)` also sets the sky light, the
    reflected light and the penumbra to suit it (a low sun gets softer
    edges). `World::light()` is the one `form::Light` every solid is lit
    with. **Below the horizon** there is no direct light and no cast
    shadow. The light points at the glow low over the sun's place, and
    `cast` is `1 − glow` everywhere, so planes turned toward the glow get a
    little light. There is also `sun_canvas()` (where the sun or its glow
    is on the canvas, for painting the sky to match) and `aerial(Z)` (with
    `visibility` in meters).
  - **Bodies.** `world.spot_at(X, Z)` (or `spot(x, y)` from a canvas
    point, or `spot_bed` for the bottom under water) gives a `Spot`: the
    foot on the canvas, units per meter there and a depth. Build the `Sdf`
    in canvas units with `spot.p(right, up, toward)`, `spot.m(meters)` and
    `spot.size(w, h, d)`, then `world.place(spot, sdf)`. A proxy,
    `world.proxy(spot, sdf)`, casts shadows, shows in the water and gets
    contact, but isn't in the `Form`: use it for a figure written with
    gestures. `form_of(frame, &[body])` gives any bodies, proxies included,
    as a `Form` lit by the same sun, so a gesture figure's light side comes
    from it too.
  - **Shadows.** Shadows are traced in the world, in meters, toward the sun
    through every body's SDF (a soft-shadow march). The penumbra grows with
    distance from the caster (tested). Low sun on uneven ground also lets a
    bank shade the hollow behind it. `cast(W, n)` works at any world point.
    Solids and ground both use it, so a pole's shadow on the boulder and
    the boulder's shadow on the sand come from the same sun. The march's
    step is capped at 0.5 m at any distance (a caster past ~250 m used to
    panic the clamp; tested: `a_distant_caster_does_not_panic_the_shadow_march`).
  - **Contact.** `occlusion(W, n, reach, with_ground)` is SDF ambient
    occlusion. `View::contact(reach)` masks the dark seam where solids and
    proxies meet the ground: on the ground around them and on the solid
    just above it.
  - **Reflections.** `View::mirror(x, y)` reflects the view ray about the
    rippled water normal and traces it through the bodies. It returns the
    body seen (or the sky or far shore), `src` (where that thing is seen
    directly on the canvas), the reflected point's normal and `Shade` (so
    a stone's reflection shows its underside as the sun lights it),
    `fresnel` (Schlick for water: small looking down at near water, large
    toward the horizon) and `travel` (how far the ray ran, for blurring).
    Things that aren't modeled, such as a far shore, are taken to stand on
    a screen at `backdrop` meters. Water beyond the backdrop (the screen is
    behind the reflected ray there) mirrors the canvas about the horizon
    instead, so the far shore's reflection runs on under it; before, the
    ray ran backward and sampled the water itself (tested:
    `reflections_beyond_the_backdrop_run_forward`).
    `View::reflections(&[bodies])` is the
    mask of where bodies show in the water.
  - **Perspective helpers.** `ribbon(&[(X, Z)], |t| width_m)` returns a
    `Shape`: a path or brook on the ground that narrows and foreshortens by
    itself (tested). `line(&[(X, Z)])` gives its canvas center line.
    `recede(start, step, n)` gives spots whose spacing recedes (fence
    posts, footprints; tested). `shadow_angle(x, y)` gives the canvas
    direction a shadow runs at a point, for stroking shadows.
  - **`View`** (`world.view(c.frame())`) holds a `form: Form` of the
    visible bodies (all of `form`'s fields and masks work). A body's parts
    behind the ground or water seen in front of them, or below the ground or
    water where they stand, are hidden, so a sunk boulder or a pole in a
    pond has a true waterline (tested: `sunk_surfaces_stay_below_the_waterline`).
    Bodies hide each other by world depth (`Form::add_nearest`), not by
    each body's own form `z`: every body is drawn in its own spot's
    projection, so form `z` values of two bodies do not compare (tested:
    `overlapping_bodies_are_ordered_by_world_depth`). Form `z` is still
    kept for each body's own shading and silhouettes. The form is lit by
    `light_given` with the world-traced cast shadows. `at(x, y) -> Point`
    gives what is there (`What::Sky/Ground/Water/Body(id)/Off`), the world
    point, the normal (form frame), the depth and the `Shade`. Masks:
    `mask(|p| ..)`, `sky`, `land`, `water`, `shadows`, `contact`,
    `reflections`.
- `crates/paint/src/form.rs`, additive only: `Form::light_given(light,
  |x, y, sample| cast)` lights a form with cast shadows traced elsewhere.
  Nothing else in form.rs changed.
- `paintings/src/bin/study_scene.rs` (new): the evidence (below).

## API, briefly

```rust
use paint::scene::{Sun, Water, World, What};

// one world per picture (or per panel)
let mut w = World::new([0.0, 0.0, 1000.0, h], h * 0.36, 1.6)   // view, horizon y, eye m
    .fov(1000.0, 55.0)
    .ground(|x, z| 0.05 + 0.03 * (x * 1.3).sin() - 0.12 * pool(x, z))
    .water(Water::new(0.0).ripple(0.03, 1.4, 0.3, 7))
    .sun(Sun::deg(-72.0, 9.0))          // low, from the left
    .backdrop(700.0);                    // the far shore is ~700 m off

// a boulder on the sand, sunk a hand deep
let s = w.spot_at(3.1, 9.4);
let rock = Sdf::block(s.p(0.0, 0.4, 0.0), s.size(1.35, 1.0, 1.1), s.m(0.36))
    .turn(s.p(0.0, 0.4, 0.0), 0.6, 0.06, -0.08).rough(s.m(0.07), s.m(0.7), 3, false);
let boulder = w.place(s, rock);
// a pole standing on the bottom of the sea
let s = w.spot_bed(-4.2, 21.0);
w.place(s, Sdf::block(s.p(0.0, 1.6, 0.0), s.size(0.13, 3.2 - s.at[1], 0.13), s.m(0.05)));
// a figure written with gestures: a proxy for her shadow, contact and reflection
let s = w.spot_at(0.0, 9.6);
let woman = w.proxy(s, Sdf::ellipsoid(s.p(0.0, 0.8, 0.0), s.size(0.25, 0.85, 0.2)));
figures::woman(c, (s.x, s.y), s.m(1.7), &gown, WomanPose::Standing, seed);

let view = w.view(c.frame());
let sand = |x, y| { let p = view.at(x, y); mix(shade_col, lit_col, smoothstep(0.0, 0.12, p.shade.turn), Mix::Pigment) };
let shadows = view.shadows().mul(&view.land());          // stroke along w.shadow_angle(x, y)
let seam = view.contact(0.22);                           // a dark seam grounds stone and feet
rocks::paint_solid(c, st, &view.form, &Solid { parts: vec![view.part(boulder)], .. });
let mirrored = |x, y| view.mirror(x, y).map(|m| /* m.body, m.src, m.shade, m.fresnel */ ..);
let path = Mask::from_shape(c.frame(), w.ribbon(&[(-2.6, 4.5), (-1.6, 8.6), (-0.4, 9.7)], |t| 1.1 - 0.4 * t));
```

## Evidence

`cargo paint study_scene` paints the same shore three times on one sheet:
two fishermen's poles standing in the sea, a granite boulder at the water's
edge, a woman at the edge of a tide pool, a trodden path running back to
her and a far low shore. The scene is built once per panel; only the sun
and the painter's colors change:

- top: morning, `Sun::deg(-72, 9)`, low side light from the left;
- middle: noon, `Sun::deg(-145, 58)`, high behind the painter's left
  shoulder;
- bottom: dawn, `Sun::deg(18, -4)`: the sun hasn't risen, and the glow is
  over the sea ahead right.

Everything is painted with brushes: sky, far shore, water mirroring the
sky, sand lit by the world's light, cast shadows as a stroke pass along
`shadow_angle` with the penumbra fused, the boulder by
`rocks::paint_solid` from `view.form`, poles dark then lit on the side the
sun finds, reflections stroked from `mirror`, the woman written by
`figures::woman` at `spot.m(1.7)` with her light side from `form_of`, and
last a transparent umber glaze through `contact`.

- `notes/scene/study_scene.jpg`: the 1000px sheet (`out/study_scene.png`,
  ~30–85 s on a busy machine).
- `notes/scene/study_scene_3200_crop.jpg`: `-- --only morning --full --crop
  380,120,900,330`, the woman, her shadow and the boulder at 3200px (~45 s,
  1.9 GB peak).
- `notes/scene/study_scene_masks.jpg`: `-- --masks`, the engine's fields
  unpainted. Gray is the light model's value, blue cast shadow, red
  contact and green where bodies show in the water.

Judged as a painter:

- **One light: yes, in all three.**
  - Morning: every shadow runs right and a little toward us, the woman's
    long and thin, the boulder's broad. The boulder is lit on its left
    face and dark on its right, and the woman has a thin light edge on her
    left.
  - Noon: all the shadows are short and fall back and to the right.
  - Dawn: there are no cast shadows anywhere. The poles and the woman are
    dark against the glow, and only the seams and the reflections tie them
    down.

  The coast painter's two light logics can't happen here: the reflections
  are mirror geometry and the shadows are traced from the same sun.
- **Grounded: mostly.** The boulder's waterline and sand line come from
  the depth test (the sunk part is hidden, not painted over). The umber
  seam darkens where it meets the sand, and in the morning its shadow
  starts at its foot. The woman's hem sits in her contact, and her shadow
  starts at her feet. The poles have a true waterline, and their mirror
  images start at it and break up in the ripples. The tide pool mirrors
  her lower half, upside down, at her feet.
- **Reflections read** best at dawn (still water, long dark pole images)
  and in the morning (broken by ripples). At noon they are weaker, because
  near water looked into from above reflects little (Fresnel). That is
  physically right, but a painter may want to push it.
- **Weak spots.**
  - At 3200px the shadow pass is still a fairly even gray band with a
    firm edge; the fused penumbra is too slight to see.
  - The noon gown's lit side reads a little gray.
  - The far shore is a plain strip.
  - Orange ground flecks show through everywhere. That's friction item 4,
    coverage at passage edges, which belongs to the fixes stream, not this
    one.

## Known issues and next steps

- **Weak perspective per body.** Each body is drawn orthographically at
  its own scale (`spot.s`). That is right for things small against their
  distance: stones, poles and figures. A long wall running into depth
  won't converge; build it from several bodies or use a `ribbon`.
- **Ground limits.** The ground must stay below eye level and mustn't hide
  itself. `to_ground` finds the first crossing along the line of sight, but
  hills above the horizon belong to the painter (a `Ridge` or a painted
  backdrop). The `Form` doesn't hold the ground as a part: ground and
  water are the `View`'s (`at`, `land`, `water`).
- **Shadows on water.** They are computed (`shadows()` includes water),
  but clear water shows a shadow barely at all. The study masks them out
  (`.mul(&land)`). That's the painter's choice.
- **Sun below the horizon.** The glow is modeled as a weak directional
  light (`Sun::glow`, at most 0.4 of the sun) with no cast shadows. There
  is no soft occlusion from the glow's whole patch of sky; contact is
  the only darkening.
- **Cost.** A `View` holds a `Form` (whole canvas, see `notes/form.md`,
  Memory) plus 8 bytes/px (ground depth and ground cast). Rough SDFs
  (`Sdf::rough`) make shadow and reflection marching slower: build
  bodies' bounding spheres small. `mirror` marches a ray per call, so use
  it in color fields (called per stroke), not per pixel, except through
  `reflections()` (once per pixel).
- **Found while painting (for the drying stream):** `Canvas::glaze` over a
  mask at thickness 5 wiped a whole panel back to the bare ground. At 1.3
  it was fine but barely visible. This looks like friction item 2 (settle
  dividing by tiny volumes). I didn't dig further; the study uses
  brushwork for shadows instead.
- **Next:**
  - hand `World` to the painters: build the coast and the mountains'
    tor on it, and add a `Ridge` backdrop hook so distant ranges share
    the sun;
  - blur reflections by `travel`;
  - add a `View::ground_fall` field (strokes following the ground's
    slope);
  - add footprints and other small marks through `recede` with jitter;
  - add easel (Lua) bindings for `World`, `Spot` and `View`.

## Commits

- `262af1c` scene: one world, one sun: camera and ground plane, placed
  bodies, world-traced soft cast shadows, contact occlusion, water mirrors
  with ripples and Fresnel, ground ribbons and receding spots;
  `Form::light_given` (additive)
- `9a2a522` scene: views clipped to a panel, `form_of` for proxies,
  `shadow_angle`; study_scene with three suns and a `--masks` diagnostic
- `33f23d6` study_scene: shadows as a stroke pass along `shadow_angle` with
  a fused penumbra; quieter stone grain and gown light
- `5551785` scene: clippy; study_scene colors from inside the panel;
  previews in `notes/scene`
