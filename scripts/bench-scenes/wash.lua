--@ engine 5
--@ chunk 1
canvas{size=400, aspect=4, linen=16, seed=31, ground={{pile={{"lead white", 1}}, um=80, apply="knife"}}}
--@ chunk 2
local b = brush("filbert", 12)
b:load(pile{{"yellow ochre", 1}, thinner=0.5}, 0.5)
for i=1,4 do b:stroke({{100, 40*i}, {800, 40*i}}) end
--@ chunk 3
wait(30)
