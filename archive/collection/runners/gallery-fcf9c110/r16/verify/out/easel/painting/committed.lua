-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=400, aspect=1.25, linen=15, ground={{pile={{"lead white", 1}}, um=100, apply="knife"}}}

--@ chunk 2
print(wait(60))
