-- easel session "easel4_free": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_free.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
H0 = canvas{style="friedrich", aspect=1.45, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 452
mn = noise{seed=5, octaves=4, period=90}
mound = function(x) return HZ + 3 - 52*math.exp(-((x-430)/175)^2) - 14*math.exp(-((x-250)/90)^2) + 3*mn(x,0) end
oak = tree{habit="dead_oak", x=372, y=mound(372)+5, height=318, seed=11}
-- the dolmen: a heavy granite capstone on boulders, one fallen beside it
cap = outline{{498,394,"c"},{502,378},{522,364},{556,358},{590,362},{614,372},{624,388,"c"},{608,402},{566,404},{528,402}, char="broken", seed=4}
up1 = outline{{507,398},{522,395},{537,400},{539,414},{533,421,"c"},{510,419,"c"},{505,409}, char="broken", seed=5}
up2 = outline{{552,402},{566,402},{569,420},{556,424,"c"},{550,414}, char="broken", seed=6}
up3 = outline{{586,400},{604,398},{616,404},{619,422},{614,436,"c"},{590,437,"c"},{584,420}, char="broken", seed=7}
fall = outline{{628,440,"c"},{632,428},{650,423},{668,428},{674,439,"c"},{650,442}, char="broken", seed=9}
path = {{520,690},{566,640},{612,572},{636,515},{650,480},{644,460},{610,446}}
town = {{800,452},{806,443},{812,443},{812,436},{816,432},{820,436},{820,443},{826,443},{829,424},{832,410},{835,424},{838,443},{858,443},{860,434},{870,434},{872,443},{888,443},{892,439},{896,443},{904,452}}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
local mp = {} for x = 0, 1000, 25 do mp[#mp+1] = {x, mound(x)} end
h:sketch(mp, {pressure=0.3})
h:sketch(path, {pressure=0.28})
h:sketch({{770,112},{758,122},{756,138},{766,150}}, {pressure=0.3})
b = pencil("HB")
for _, l in ipairs(oak.limbs) do
  if l.order <= 2 and #l.pts >= 2 then b:line(l.pts, {pressure=(l.order == 0) and 0.6 or 0.45}) end
end
for _, o in ipairs({cap, up1, up2, up3, fall}) do for _, p in ipairs(o:paths()) do b:line(p, {pressure=0.55, smooth=false}) end end
b:line(town, {pressure=0.4, smooth=false})
b:line({{655,448},{653,458},{651,470}}, {pressure=0.5})
b:line({{659,448},{660,458},{661,470}}, {pressure=0.5})
