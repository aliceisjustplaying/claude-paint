--@ box inness
--@ engine 2
--@ chunk 1
canvas{size=400, aspect=1.4, linen=15, seed=7, ground={{pile={{"lead white", 4}, {"raw umber", 1}}, um=90, apply="knife"}}}
--@ chunk 2
sky = pile{{"lead white", 4}, {"cobalt blue", 1}, {"yellow ochre", 0.3}, thinner=0.7}
dark = pile{{"raw umber", 2}, {"bone black", 1}, thinner=0.7}
field = pile{{"yellow ochre", 2}, {"raw sienna", 2}, {"lead white", 1}, thinner=0.7}
work(above(function(x) return 300 + 30 * math.sin(x / 170) end), {hand="broad", pile=sky, load=0.3, seed=1})
work(ellipse(380, 330, 170, 140), {hand="body", pile=dark, load=0.3, seed=2})
work(below(function(x) return 340 + 30 * math.sin(x / 170) end), {hand="broad", pile=field, load=0.3, angle=0.1, seed=3})
