-- easel session "pencil_try": a painting replayed chunk by chunk.
--   easel run paintings/lua/pencil_try.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich_early", size=440, aspect=1.4, seed=7}

--@ chunk 2 · clock 0

p = pencil{grade="2H"}
p:sketch({{0, 420}, {300, 424}, {600, 418}, {1000, 423}}, {pressure=0.3})
b = pencil("4B")
b:line({{100, 500}, {300, 480}, {500, 520}}, {pressure=0.7})
c = chalk()
c:line({{100, 560}, {300, 540}, {500, 580}}, {pressure=0.6})
h = pencil("2B")
h:hatch(ellipse(700, 550, 80, 50), {pressure=0.4})
h:sharpen()
h:line({{600, 440}, {800, 470}}, {pressure=0.2})
h:line({{600, 460}, {800, 490}}, {pressure=0.5})
h:line({{600, 480}, {800, 510}}, {pressure=0.9})
print(p, b, c, h, b:width())
