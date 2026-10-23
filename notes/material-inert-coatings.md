# Material revision 1: inert finishing coatings

An inert coating is a saved optical operation, not reactive oil paint. The
existing glaze and mastic-varnish paths supply three-channel Kubelka–Munk K/S
coefficients. The store retains those exact coefficients, their optical coat
count and their separately settled geometric thickness. It does not infer
pigment, binder, cure, oxygen permeability, stiffness or fracture properties.

Each tile owns a sparse ordered list of coating records. A record names its
local pixel and the number of reactive packets below it. Reactive cells,
packets and their chemical and mechanical history are unchanged. Several
coatings can occupy one packet boundary, in application order. A later Cover
deposit appends reactive packets above the coatings. Cell refinement and tile
compaction do not change these packet-relative positions.

Rendering traverses that order. A coating applies the original RGB KM transfer
to the current lower stack, then carries its RGB correction in the existing
spectral renderer's residual. The retained lower spectrum is not replaced or
estimated from a finished RGB pixel. A later spectral deposit attenuates the
correction along with the lower stack; the coating cannot recolor an opaque
later deposit. The RGB coating does not claim a measured full spectrum.

Geometry reads include the saved coating thickness without adding it to paint
or solvent inventory. Uncoated stores still write OILCOL02. Coated stores write
OILCOL03 with explicit coating totals and tagged sparse records. The reader
checks those totals, order, ownership, coefficient validity and both raw and
resident budgets before allocation. Canvas checkpoints still count the current
and retained interval-start stores together. Coatings are covered by COW and
the full store checkpoint and state dump.

## Unsupported operations

The finishing compatibility boundary deliberately has no resin contact,
permeability or fracture model. These operations return
`MaterialError::Unsupported` rather than treating resin as oil or exposing the
buried paint directly to air:

- Contact sampling, removal and entrainment on an inert-coated column.
- Native brush, rag, thinner or knife contact when a projected contact window,
  including its required halo, contains an inert coating.
- Solvent addition or removal on an inert-coated column and clock advancement
  requiring the current whole-canvas solvent transport with coatings present.
- Chemistry advancement of a reactive column containing a coating, whether the
  reactive paint is above or below the coating.
- Applying an inert glaze over a substrate that is not touch-dry or still has
  free solvent.
- Drawing-tool contact with a coating. The legacy void-returning drawing and
  native kernel wrappers report the unsupported operation by unwinding; the
  session's failed-chunk guard restores the earlier canvas and tool state.

Geometry-only reads, optical rendering, serialization and Cover deposition
remain available. `try_glaze` and `try_wait` expose the fallible engine paths;
the developer Lua glaze, varnish and wait bindings propagate their errors.
Hand-clock flushing also propagates an unsupported chemistry advance, so a
varnish operation with owed hand time is rejected rather than silently aging
buried paint through an unmodeled coating. A finishing render with hand time
off can retain the coating without advancing that unsupported clock.

The top of a coating is an immediately set inert surface. Its surface stage is
Dry; this does not mark buried reactive material as chemically mature. Once
paint is deposited above it, surface observations use that upper reactive stack
only. General constitutive integration across coating spacers, crack formation
and resin-aware knife pickup are not supplied by this compatibility boundary.
