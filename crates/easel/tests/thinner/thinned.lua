sienna = pile{{"raw sienna", 1}, thinner=0.5}
veil = pile{{"lead white", 3}, {"raw umber", 1}, thinner=0.3}
b = brush("flat", 24)
b:load(sienna, 0.6)
for i = 0, 5 do b:stroke({{120, 150 + 40 * i}, {880, 170 + 40 * i}}, {pressure={0.8, 0.8}}) end
work(rect(200, 420, 600, 200), {hand="broad", pile=veil, load=0.4, angle=0, seed=3})
