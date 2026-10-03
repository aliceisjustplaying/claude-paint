# The giverny box

A box for painting water and light in the manner of late Impressionism:
bright, high-key, no blacks and no earth darks but yellow ochre.

| tube | why it is here |
|---|---|
| lead white | the body of every light |
| zinc white | a colder, cleaner white for scumbles and glints |
| lemon chrome, chrome yellow, cadmium yellow | three yellows from cool to warm, for sunlight and greens |
| yellow ochre | a quiet warm yellow |
| vermilion | the warm red |
| rose madder | a transparent cool red: true pinks with white, and glazes |
| cobalt violet | violet without mixing a muddy red and blue |
| cobalt blue, ultramarine blue, cerulean blue | a mid blue, a deep warm transparent blue, an opaque green-blue |
| viridian | a transparent cool green, for glazes and deep water |
| emerald green | a bright opaque green |

Every tube is an existing catalog tube with its catalog numbers (palette.rs);
the box adds no new pigment data. The choice of tubes is the painter's own,
from memory of what the late Impressionists are generally held to have used,
not a sourced materials study like `notes/research/*_materials.md`.

A painting in this box writes `--@ box giverny` in its log, so it always
replays with these tubes. Build an easel that paints from it with
`cargo build --release -p easel --features box-giverny` plus a `box` file
next to the executable (or `EASEL_BOX=giverny`) naming `giverny`.
