-- easel session "stroke": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=2, linen=15, seed=21, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}

--@ chunk 2
p = pile{{"raw sienna", 1}}
b = brush("filbert", 60)
b:load(p, 0.6)
b:stroke({{100, 250}, {900, 260}}, {pressure={0.8, 0.6}})
print(b:fullness())

--@ chunk 3
b:stroke({{120, 380}, {880, 360}}, {pressure={0.5, 0.5}})
print(b:fullness())
