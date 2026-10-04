-- easel session "thinner-determinism": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=1.5, linen=15, seed=4, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=60, apply="knife"}}}

--@ chunk 2
sienna = pile{{"raw sienna", 1}, thinner=0.5}
veil = pile{{"lead white", 3}, {"raw umber", 1}, thinner=0.3}
b = brush("flat", 24)
b:load(sienna, 0.6)
for i = 0, 5 do b:stroke({{120, 150 + 40 * i}, {880, 170 + 40 * i}}, {pressure={0.8, 0.8}}) end
work(rect(200, 420, 600, 200), {hand="broad", pile=veil, load=0.4, angle=0, seed=3})
wait(3.3)

--@ chunk 3
local r = rag()
r:dip(0.5)
r:wipe({{150, 200}, {800, 220}}, {pressure=0.7})
r:blot(500, 300, {pressure=0.6})
work(rect(100, 100, 800, 300), {hand="body", pile=veil, load=0.6, seed=9})
wait(1)

--@ chunk 4
b:load(sienna, 0.8)
b:stroke({{120, 380}, {880, 420}}, {pressure={0.9, 0.9}})
