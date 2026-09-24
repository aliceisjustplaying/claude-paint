-- notes/time: a sky, then a hill's edge painted into it. Variant: afternoon.
--   easel run notes/time/edge_afternoon.lua   (see notes/time.md)
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=7, hand=true}

--@ chunk 2 · clock 0
-- the sky, brought a little past the line the hill will take
HZ = 470
far = noise{seed=3, octaves=5, period=260}
crest = function(x) return 420 - 45*far:at01(x, 0) - 25*math.exp(-((x-640)/120)^2) end
sky = function(x, y) return gradient({{0,"#5d7396"},{0.5,"#9fabb8"},{0.85,"#d9cfb4"},{1,"#e9d6a6"}}, y/HZ) end
skym = above(function(x) return crest(x) + 10 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.5, medium=0.25})
blend(skym, {angle=0})
local t = timesheet(); print(string.format("clock %.0f min · sitting %d, %.0f min · open %.0f%% setting %.0f%% tacky %.0f%% dry %.0f%% · %d strokes, %.0f reloads, %d piles, %.0f min of hand time", t.clock, t.sittings, t.sitting, 100*t.open, 100*t.setting, 100*t.tacky, 100*t.dry, t.strokes, t.reloads, t.piles, t.hand_min))

--@ chunk 3 · clock 0
-- after lunch: four hours away from the easel
rest(4)
-- the hill, its top strokes laid into the sky's edge
ridge = below(crest):roughen(1.5, 12)
work(ridge, {hand="body", color=function(x,y) return mix("#5f6b80", "#8a93a0", (y-380)/100) end, angle=0.05, length={25,70}, coverage=3, medium=0.3})
local t = timesheet(); print(string.format("clock %.0f min · sitting %d, %.0f min · open %.0f%% setting %.0f%% tacky %.0f%% dry %.0f%% · %d strokes, %.0f reloads, %d piles, %.0f min of hand time", t.clock, t.sittings, t.sitting, 100*t.open, 100*t.setting, 100*t.tacky, 100*t.dry, t.strokes, t.reloads, t.piles, t.hand_min))
