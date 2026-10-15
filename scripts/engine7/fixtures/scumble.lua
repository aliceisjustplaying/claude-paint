--@ engine 5
--@ chunk 1
canvas{size=440, aspect=2.5, linen={18,16}, seed=653,
  ground={{pile={{"lead white",10},{"yellow ochre",0.2}}, um=100, apply="knife"}}}
dark = pile{{"ultramarine blue",2},{"bone black",1},{"raw umber",1}}
light = pile{{"lead white",6},{"yellow ochre",1}, blot=0.3}
--@ chunk 2
work(rect(80,60,840,280), {hand="body", pile=dark, tool="filbert 24", coverage=1.2, angle=0.15})
wait(30*24*60)
--@ chunk 3
work(rect(130,90,740,220), {hand="scumble", pile=light, tool="filbert 20", coverage=0.6, angle=-0.2})
print(drying(500,200))
