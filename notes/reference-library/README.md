# Painter reference library

`manifest.json` selects 20 reviewed texts or bounded excerpts from existing notes.
`scripts/build_reference_library` reads the selected committed revision, checks
each source hash and applies the existing source-stripping transform. A changed
source fails closed until its selection is reviewed again. The exporter ships
only `notes/reference-library.json`, not this inventory or source filenames.

Both the pi painter and the regular Antigravity MCP expose `reference` with the
same query/id/page contract. Optional `reference_list` returns the catalog's ids
and titles only when called, after the painter chooses a subject or technical
question. The initial instructions do not display that catalog.
Search matches every whitespace-separated word,
case-insensitively, across title, keywords and full text. Results stay in index
order; no match returns an empty list, never a default topic. Retrieval returns
every character in consecutive pages of at most 4,000 UTF-8 bytes. The exported
library tests check all replies stay under Antigravity's 8,000-byte threshold.

## Coverage

- **Subject structure:** tree growth, taper, junctions and eight species; snow,
  hoarfrost and rime deposition on branches.
- **Subject handling:** historical observations of rock, water and snow paint;
  two short artificial-light passages; one examination-backed fruit/cloth
  example. These are narrow cases, not comprehensive subject manuals.
- **Materials:** oil and watercolor physics, capillary flow and optics in raw
  canvas, pigment behavior and optical mixing. Source caveats and estimates
  remain in the served text.
- **Easel techniques:** ground, spatter, uneven loading, impasto, knife,
  blending, glazing, scumbling, rag and drying. Real-material physics is not a
  promise that the simulator implements every effect.

The subject entries are marked `"subject": true` in the manifest. The every and
blank studios, whose painter chooses its own subject, are exported without them
(`scripts/build_reference_library --without-subjects`).

## Exclusions and gaps

Composition briefs and artist process recipes are excluded, including the
Friedrich composition guide and wet-on-wet winter-lake sequence. Artist reports
are not served wholesale: only the explicit neutral passages in the manifest
are included. Full citations, researcher comments and source paths stay in the
repository. The normal artist-name isolation check still applies to the export.

Archived painter output, journals, workshop/sketchbook lessons, obsolete engine
notes, duplicate historical copies and private/off-repository material are not
library sources. The duplicate oil-physics text is indexed once. The pigment
selection omits the obsolete bitumen/crackle assertion. The technique selection
omits obsolete tool commands, whole-picture prescriptions and archived sitting
recipes. The botanical source in round 25 is supplied research, not a painter
journal. The older artist-associated tree report supplies only its additional
snow/frost passage.

There are no adequate neutral references here for anatomy, faces/hands,
animals/birds, flowers, architecture/interiors, perspective construction or
cloth-fold structure. Sky/cloud/mountain/reflection fragments in the wet-on-wet
note lack a traceable bibliography and are not indexed. Fruit and light coverage
is brief; rock/water/snow handling does not cover geology or wave mechanics.
No new research was invented to fill these gaps.

## Free-subject Friedrich preparation

Export a new studio from the corrected revision, then use
`scripts/prepare_friedrich_studio` with the approved house runner directory and
studio notes. It copies the house template unchanged, substitutes only its
opening, studio path and reading list, and prepares a separate runner copy.
That copy enables `reference` and removes the mandatory composition/tree
reading and copying. The generated `run.py` connects the supplied studio to the
corrected source checkout and uses the unchanged house sitting loop with Opus
5.5 xhigh. `--check` prints the configured command without launching anything.
The copied historical entrypoint is disabled; it cannot prepare another studio
through its old defaults. Approved studio notes are included in both places.
Historical runners and studios are never modified. Preparation refuses a used
studio and never launches a painter.

Independent review of the painter-facing wording is recorded on the pull request.
