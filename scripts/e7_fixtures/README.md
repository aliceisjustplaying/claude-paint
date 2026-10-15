Engine 7 evidence fixtures: three short Lua logs that isolate one paint
behavior each, for side-by-side engine 5 vs engine 7 gallery renders.

- blender_on_wet.lua: red and blue wet fields meeting at x=500, then
  blend() across the seam. Engine 7 should still read as a pull across it.
- two_color_drag.lua: a flat reloaded with cadmium yellow dragged through a
  wet blue field, so the stroke carries both colors as it goes.
- scumble.lua: a dark field left 14 days to dry, then a broken light
  scumble over it.

Each is 300 mm wide; replay at --width 800 for r40's pixel pitch
(0.375 mm/px):

    scripts/e7_replay run scripts/e7_fixtures/scumble.lua --engine 5 --workers 4 --width 800
