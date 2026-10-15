-- easel session "blender_on_wet": an engine 7 evidence fixture (scripts/e7_fixtures/README.md).
--@ box hopper
--@ engine 5

--@ chunk 1
canvas{size=300, aspect=1.5, linen={18,16}, seed=701,
  ground={{pile={{"lead white",10},{"yellow ochre",0.25}}, um=90, apply="knife", texture=0.3}}}

--@ chunk 2
red = pile{{"red earth",2},{"burnt sienna",1},{"lead white",1}, turps=0.4, name="red"}
blue = pile{{"ultramarine blue",2},{"lead white",1.5}, turps=0.4, name="blue"}
work(rect(0, 0, 500, 667), {hand="body", pile=red, coverage=2, fill=true, angle=0})
work(rect(500, 0, 500, 667), {hand="body", pile=blue, coverage=2, fill=true, angle=0})

--@ chunk 3
blend(rect(250, 170, 500, 330), {angle=0})
