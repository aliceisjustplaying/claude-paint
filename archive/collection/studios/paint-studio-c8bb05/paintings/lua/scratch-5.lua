-- easel session "scratch": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{aspect=1.3, ground={{apply="knife", pile={{"lead white", 6}, {"yellow ochre", 1}, {"red earth", 0.4}}, texture=0.4, um=120}}, linen={18, 16}, seed=417, size=650}

--@ chunk 2
sk_sun = pile{{"lead white",3},{"lemon chrome",1},{"cadmium yellow",0.3}, name="sk_sun"}
sk_gold = pile{{"lead white",2},{"cadmium yellow",1},{"Naples yellow",1},{"orange chrome",0.3}, name="sk_gold"}
sun_core = pile{{"lead white",4},{"lemon chrome",0.5}, name="sun_core"}
work(rect(50,50,500,200), {hand="broad", piles={{sk_gold,function() return 1 end},{sk_sun,function(x,y) return x/500 end}}, coverage=2, fill=true, angle=0})
for i,cx in ipairs({150,300,450}) do
  local disc = ellipse(cx, 150, 18, 17)
  work(disc, {hand="detail", piles={{sun_core,function() return 1 end},{sk_sun,function() return 0.3 end}}, coverage=4, fill=true, angle=0, tool={kind="filbert", width=5, lay=4}, clip=disc, seed=431+i})
end
print(wait(60*24*60))
print(drying(150,150), drying(300,150))

--@ chunk 3
local k = knife{width=38}
k:load(sun_core, 0.6)
k:lay({{132,150},{168,150}}, {pressure={0.7,0.7}, angle=math.pi/2})
local k2 = knife{width=14}
for i,y in ipairs({138,146,154,162}) do
  k2:load(sun_core, 0.5)
  local hw = math.sqrt(math.max(0, 17^2 - (y-150)^2))*18/17
  k2:lay({{300-hw+3,y},{300+hw-3,y}}, {pressure={0.8,0.8}})
end

--@ chunk 4
sun_flow = pile{{"lead white",4},{"lemon chrome",0.5}, medium=0.4, name="sun_flow"}
sun_flow2 = pile{{"lead white",4},{"lemon chrome",0.5}, medium=0.65, name="sun_flow2"}
local d3 = ellipse(450,150,18,17)
work(d3, {hand="detail", pile=sun_flow, coverage=3, fill=true, angle=0, tool={kind="filbert", width=6, lay=3}, clip=d3, seed=5})
local d2 = ellipse(300,150,18,17)
work(d2, {hand="detail", pile=sun_flow2, coverage=3, fill=true, angle=0, tool={kind="filbert", width=6, lay=2}, clip=d2, seed=6})
print(wait(2*60))

--@ chunk 5
print(wait(40*24*60)); print(drying(300,150), drying(450,150))

--@ chunk 6
local d2 = ellipse(300,150,18,17)
work(d2, {hand="detail", pile=sun_flow2, coverage=3, fill=true, angle=0.3, tool={kind="filbert", width=6, lay=2}, clip=d2, seed=7})
print(wait(40*24*60))
