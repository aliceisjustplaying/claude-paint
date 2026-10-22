-- easel session "ck7-fixture": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}

--@ chunk 2
p = pile{{"lead white", 4}, {"cobalt blue", 1}, medium=0.3}; work(rect(0, 0, 550, 500), {hand="body", pile=p, coverage=2})

--@ chunk 3
wait(30); work(rect(550, 200, 400, 400), {hand="broad", pile=pile{{"bone black", 1}, {"raw umber", 1}}, coverage=1.5})
