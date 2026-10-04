-- easel session "wait": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=2, linen=15, seed=21, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}

--@ chunk 2
w = pile{{"lead white", 1}}
q = pile{{"raw sienna", 1}, medium=0.2}
b = brush("filbert", 80)
b:load(w, 0.8)
b:stroke({{100, 150}, {900, 150}})
b:reload(q, 0.8)
b:stroke({{100, 350}, {900, 350}})

--@ chunk 3
print(wait(15))
print(drying(500, 150), drying(500, 350))

--@ chunk 4
for i = 1, 15 do wait(1) end
print(drying(500, 150), drying(500, 350))

--@ chunk 5
print(wait(7.3))
print(wait(7.7))

--@ chunk 6
print(wait(24 * 60))
print(drying(500, 150), drying(500, 350))

--@ chunk 7
print(wait(4 * 24 * 60))
print(drying(500, 150), drying(500, 350))
