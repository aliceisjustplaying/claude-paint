-- easel session "lookaid": a painting replayed chunk by chunk.
--   easel run paintings/lua/lookaid.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=7}

--@ chunk 2 · clock 0

HZ = 430
sky = function(x, y) return gradient({{0,"#5d7396"},{0.6,"#9fabb8"},{1,"#e9d6a6"}}, y/HZ) end
work(above(function(x) return HZ + 15 end), {hand="broad", color=sky, angle=0, coverage=4.5})

--@ chunk 3 · clock 0

w = world{horizon=HZ, eye=1.7, fov=50, sun={azimuth=-125, elevation=30}, ground=function(X, Z) return 1.2*math.sin(X/40) * math.min(1, Z/60) end}
v = w:view()
work(v:land(), {hand="body", color=function(x, y) return mix("#6b6a48", "#3a3a28", (y-HZ)/300) end, angle=0, coverage=3})

--@ chunk 4 · clock 0
-- Looking by eye: in a live session these draw on the next look; in a
-- replay they do nothing, so this chunk paints exactly what it would
-- without them (a reopened session too). To see them, run this chunk
-- with `easel try` in a session and look.
sheep = {{612,560},{640,548},{668,552},{680,566},{660,578},{626,578}}
show(sheep, {closed=true, label="sheep"})
show(ellipse(300, 520, 60, 25), {label="pond"})
b = brush("round", 6)
path = show({{100,650},{180,610},{260,630},{330,590}}, {brush=b, label="path"})
local p = probe(300, 520)
print("pond:", p.hex, p.drying, p.what, string.format("%.1f m", p.dist))
b:load(mix(p.color, "#3a3a28", 0.5), 0.8)
b:stroke(path)
