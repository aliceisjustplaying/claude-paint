-- easel session "body": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=200, aspect=2, linen=15, seed=21, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}

--@ chunk 2
p = pile{{"raw sienna", 2}, {"lead white", 1}}
work(rect(300, 120, 400, 260), {hand="body", pile=p, seed=3})
