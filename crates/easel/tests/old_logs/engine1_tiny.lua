-- easel session "engine1_tiny": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=300, aspect=1.5, linen={16, 14}, seed=9, ground={{pile={{"yellow ochre", 2}, {"red earth", 1}, {"lead white", 1}}, um=95, apply="knife", texture=0.2}, {pile={{"lead white", 3}, {"yellow ochre", 1}}, um=30, apply="brush"}}}

--@ chunk 2
local c = chalk()
c:line({{100, 400}, {900, 410}}, {pressure={0.3, 0.4, 0.35}})
h = pencil("2H")
h:sketch({{200, 150}, {400, 120}, {600, 180}}, {pressure=0.3})

--@ chunk 3
sky = pile{{"smalt", 3}, {"cobalt blue", 2}, {"lead white", 3}}
work(rect(0, 0, 1000, 300), {hand="broad", pile=sky, angle=0.1, coverage=1.4, fill=true})
glow = pile{{"lead white", 4}, {"yellow ochre", 1.2}, {"vermilion", 0.4}}
work(rect(0, 200, 1000, 100):soften(40), {hand="glaze", pile=glow})

--@ chunk 4
print(wait(20))
print(drying(500, 150))
land = pile{{"raw umber", 2.5}, {"cobalt blue", 1.5}, {"lead white", 0.4}}
work(rect(0, 300, 1000, 367), {hand="body", pile=land, angle=-0.08, coverage=2.1})
b = brush("filbert", 12)
b:load(sky, 0.8)
b:stroke({{100, 250}, {500, 330}}, {pressure={0.7, 0.4}})
b:reload(glow, 0.6)
b:touch(700, 250, {pressure=0.6, drag={1, 0}})
blend(rect(300, 260, 400, 80), {angle=0})

--@ chunk 5
print(wait(24 * 60))
print(drying(500, 150), drying(500, 400))
d = pile{{"lead white", 6}, {"yellow ochre", 1.1}}
work(ellipse(720, 180, 30, 30), {hand="detail", pile=d, coverage=1.6, fill=true, clip=true})
o = outline{{300, 500, "c"}, {420, 450}, {560, 520, "c"}, char="firm", seed=3}
o:paint(b, {pressure=0.8, dip={d, 0.6}, every=3})
print(wait(3 * 24 * 60))
print(drying(500, 150), drying(500, 400))
