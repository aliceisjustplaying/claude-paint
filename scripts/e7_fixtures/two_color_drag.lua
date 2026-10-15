-- easel session "two_color_drag": an engine 7 evidence fixture (scripts/e7_fixtures/README.md).
--@ box hopper
--@ engine 5

--@ chunk 1
canvas{size=300, aspect=1.5, linen={18,16}, seed=702,
  ground={{pile={{"lead white",10},{"yellow ochre",0.25}}, um=90, apply="knife", texture=0.3}}}

--@ chunk 2
blue = pile{{"ultramarine blue",2},{"viridian",0.5},{"lead white",1}, turps=0.4, name="blue"}
yellow = pile{{"cadmium yellow",3},{"lead white",1}, turps=0.2, name="yellow"}
work(everywhere(), {hand="body", pile=blue, coverage=2, fill=true, angle=0})

--@ chunk 3
local b = brush{kind="flat", width=12}
for i = 0, 5 do
  b:reload(yellow, 0.8)
  local y = 110 + i * 90
  b:stroke({{80, y}, {500, y + 20}, {920, y - 10}}, {pressure=0.55 + 0.05 * i})
end
