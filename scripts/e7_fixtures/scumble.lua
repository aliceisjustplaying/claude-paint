-- easel session "scumble": an engine 7 evidence fixture (scripts/e7_fixtures/README.md).
--@ box hopper
--@ engine 5

--@ chunk 1
canvas{size=300, aspect=1.5, linen={18,16}, seed=703,
  ground={{pile={{"lead white",10},{"yellow ochre",0.25}}, um=90, apply="knife", texture=0.3}}}

--@ chunk 2
dark = pile{{"burnt sienna",2},{"ultramarine blue",1.5},{"bone black",0.5}, turps=0.5, name="dark"}
light = pile{{"lead white",4},{"yellow ochre",0.5},{"cerulean blue",0.2}, name="light"}
work(everywhere(), {hand="body", pile=dark, coverage=2, fill=true, angle=0.3})

--@ chunk 3
wait(14 * 24 * 60)

--@ chunk 4
work(ellipse(500, 333, 330, 220), {hand="scumble", pile=light, coverage=0.5, pressure=0.25})
