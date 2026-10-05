-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box friedrich
--@ engine 5

--@ chunk 1
local t = tubes(); canvas{size=300, aspect=1.25, linen=15, ground={{pile={{t[1], 1}}, um=80, apply="knife"}}}

--@ chunk 2
local t = tubes(); b = brush("round", 4); b:load(pile{{t[#t], 1}}, 0.8); b:stroke({{200, 300}, {800, 320}})
