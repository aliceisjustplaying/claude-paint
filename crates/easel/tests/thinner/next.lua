local q = pile{{"lead white", 2}, {"raw sienna", 1}, thinner=0.2}
local c = brush("filbert", 12)
c:load(q, 0.7)
c:stroke({{150, 330}, {850, 300}})
wait(2.5)
local r = rag()
r:wipe({{200, 250}, {700, 260}}, {pressure=0.6})
c:stroke({{150, 360}, {850, 380}})
