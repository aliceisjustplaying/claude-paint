-- easel session "pickup": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=2, linen=15, seed=21, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}

--@ chunk 2
k = pile{{"bone black", 1}}
b = brush("filbert", 80)
b:load(k, 0.8)
b:stroke({{100, 250}, {900, 250}})

--@ chunk 3
q = brush("flat", 60)
q:stroke({{500, 60}, {500, 440}}, {pressure={0.7, 0.7}})
print(q:fullness())

--@ chunk 4
q:stroke({{700, 60}, {700, 200}})
print(q:fullness())
b:wipe(0.85)
print(b:fullness())
