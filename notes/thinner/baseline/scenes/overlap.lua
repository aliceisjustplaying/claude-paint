-- easel session "overlap": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=2, linen=15, seed=21, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}

--@ chunk 2
w = pile{{"lead white", 1}}
b = brush("filbert", 70)
b:load(w, 0.7)
b:stroke({{100, 150}, {900, 170}})

--@ chunk 3
b:load(w, 0.7)
b:stroke({{100, 160}, {900, 160}})
u = pile{{"raw umber", 1}}
c = brush("flat", 70)
c:load(u, 0.7)
c:stroke({{500, 40}, {520, 460}})
print(b:fullness(), c:fullness())
