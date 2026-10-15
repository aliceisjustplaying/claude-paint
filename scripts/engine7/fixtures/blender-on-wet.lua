--@ engine 5
--@ box tube box
--@ chunk 1
canvas{size=440, aspect=2.5, linen={18,16}, seed=651,
  ground={{pile={{"lead white",10},{"yellow ochre",0.2}}, um=100, apply="knife"}}}
red = pile{{"vermilion",3},{"lead white",1}, medium=0.2}
blue = pile{{"cobalt blue",3},{"lead white",1}, medium=0.2}
--@ chunk 2
work(rect(160,90,280,220), {hand="body", pile=red, tool="filbert 24", coverage=1.8, angle=1.57})
work(rect(440,90,280,220), {hand="body", pile=blue, tool="filbert 24", coverage=1.8, angle=1.57})
--@ chunk 3
b = brush("badger",24)
for _,y in ipairs{145,200,255} do
  b:wipe(1)
  b:stroke({{260,y},{820,y}}, {pressure=0.65, ramps={0.08,0.15}, orient="across"})
end
print(b:fullness(), drying(560,200))
