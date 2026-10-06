-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box hopper
--@ engine 5

--@ chunk 1
canvas{size=900, aspect=1.4, linen={16,15}, seed=4021,
  ground={{pile={{"lead white",10},{"yellow ochre",0.25}}, um=120, apply="knife", texture=0.25}}}
palette{set_out={"lead white","cerulean blue","cobalt blue","ultramarine blue","viridian","pale cadmium","cadmium yellow","deep cadmium","yellow ochre","red earth","burnt sienna","bone black"}}
print(W,H)

--@ chunk 2
h = pencil("2B")
local function R(x0,y0,x1,y1,p) h:rule({x0,y0},{x1,y1},{pressure=p or 0.35}) end
-- main facade
R(0,60,600,60) R(0,95,600,95) R(600,40,600,560)
R(0,560,600,560)
-- side face
R(600,60,690,106) R(600,95,690,136) R(690,106,690,550) R(600,560,690,550)
-- upper windows rows
local xs={50,190,330,470}
for _,x in ipairs(xs) do
  R(x,125,x+80,125) R(x,215,x+80,215) R(x,125,x,215) R(x+80,125,x+80,215)
  R(x,255,x+80,255) R(x,345,x+80,345) R(x,255,x,345) R(x+80,255,x+80,345)
end
-- storefront
R(0,370,600,370) R(0,395,600,395)
R(50,410,420,410) R(50,540,420,540) R(50,410,50,540) R(420,410,420,540)
R(455,405,535,405) R(455,405,455,560) R(535,405,535,560)
-- sidewalk / street
R(0,600,1000,590) R(690,550,1000,520)
-- distant buildings
R(690,300,1000,300) R(760,300,760,525) R(880,250,880,520) R(880,250,1000,250)
-- water tower
h:sketch({{900,250},{900,200},{935,190},{970,200},{970,250}},{pressure=0.25})
-- figure guess
h:sketch({{640,600},{640,520},{648,505},{640,500}},{pressure=0.25})

--@ chunk 3
erase(rect(40,115,530,240) + rect(40,245,530,355), {strength=0.9})
local function R(x0,y0,x1,y1,p) h:rule({x0,y0},{x1,y1},{pressure=p or 0.35}) end
for i=0,4 do
  local c = 60 + i*116
  for _,r in ipairs({{118,222},{252,352}}) do
    R(c-28,r[1],c+28,r[1]) R(c-28,r[2],c+28,r[2]) R(c-28,r[1],c-28,r[2]) R(c+28,r[1],c+28,r[2])
  end
end
-- shadow line of unseen building
R(0,290,380,560,0.25)
