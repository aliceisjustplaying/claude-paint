-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 3

--@ chunk 1
canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; b=brush("round",4); b:load(p,0.8); b:stroke({{100,80},{600,80}})
