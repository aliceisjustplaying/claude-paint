--@ engine 5
--@ chunk 1
canvas{size=400, aspect=4, linen=16, seed=77, ground={{pile={{"lead white", 1}}, um=80, apply="knife"}}}
--@ chunk 2
work(rect(0, 0, 1000, 125), {hand="broad", pile=pile{{"yellow ochre", 2}, {"lead white", 1}}, coverage=0.4})
work(rect(100, 100, 800, 100), {hand="body", pile=pile{{"raw umber", 1}, {"lead white", 2}}, coverage=0.4})
--@ chunk 3
work(rect(200, 70, 600, 100), {hand="detail", pile=pile{{"Prussian blue", 1}, {"lead white", 2}}, coverage=0.2})
wait(30)
