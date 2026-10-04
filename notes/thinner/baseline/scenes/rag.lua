-- easel session "rag": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=2, linen=15, seed=21, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}

--@ chunk 2
d = pile{{"raw umber", 1}}
b = brush("filbert", 80)
b:load(d, 0.8)
b:stroke({{60, 400}, {440, 400}})
print(wait(5 * 24 * 60))
print(drying(250, 400))

--@ chunk 3
p = pile{{"raw sienna", 1}}
b:reload(p, 0.8)
b:stroke({{60, 100}, {440, 100}})
b:load(p, 0.8)
b:stroke({{60, 220}, {440, 220}})
b:load(p, 0.8)
b:stroke({{560, 100}, {940, 100}})
b:load(p, 0.8)
b:stroke({{560, 220}, {940, 220}})

--@ chunk 4
r = rag{width=60, seed=4}
r:wipe({{80, 100}, {420, 100}}, {pressure=0.6})
print(r)

--@ chunk 5
r:blot(250, 220, {pressure=0.6})
print(r)

--@ chunk 6
s = rag{width=60, seed=5}
s:dip(0.6)
s:wipe({{580, 100}, {920, 100}}, {pressure=0.6})
print(s, s.damp)

--@ chunk 7
r:refold()
r:wipe({{80, 400}, {420, 400}}, {pressure=0.8})
print(r)
s:wipe(rect(560, 180, 380, 80), {pressure=0.5, angle=0, seed=2})
print(s, s.damp)

--@ chunk 8
b:load(p, 0.6)
b:stroke({{250, 40}, {260, 300}})
print(b:fullness())
