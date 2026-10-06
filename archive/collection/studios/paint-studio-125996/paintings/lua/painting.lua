-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=900, aspect=1.3, linen={13, 12}, seed=1809,
  ground={
    {pile={{"red earth",3},{"yellow ochre",2},{"lead white",4}}, um=70, apply="knife", texture=0.3},
    {pile={{"lead white",9},{"yellow ochre",0.6},{"raw umber",0.25}}, um=60, apply="roller"}
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
-- Composition skeleton (kept as globals so later chunks reuse it)
HY = 470
dune_top = {{0,372},{50,356},{110,342},{170,337},{240,339},{300,347},{350,363},{410,392},{470,430},{520,462},{560,486},{585,500}}
dune_base = {{585,500},{520,545},{455,590},{395,650},{340,720},{310,769}}
shore = {{585,500},{700,536},{850,575},{1000,610}}
path = {{300,769},{312,720},{325,670},{322,620},{338,575},{352,535},{338,495},{322,455},{318,420},{302,390},{288,365},{282,348}}

dolmen_L = {{181,347},{185,318},{183,297},{192,287},{214,285},{224,291},{227,320},{223,347}}
dolmen_R = {{290,349},{287,322},{291,299},{300,292},{320,294},{327,304},{326,330},{322,350}}
dolmen_C = {{165,293},{174,277},{205,267},{255,262},{300,265},{335,273},{343,286},{331,297},{300,292},{255,289},{215,291},{180,298}}

oak_spine = {{76,356,44},{78,335,36},{84,310,31},{90,285,28},{94,262,26},{97,240,24}}
oak_limbs = {
  {{92,262,17},{74,232,15},{55,200,12},{40,165,9},{30,128,7},{24,95,5},{22,62,3},{28,30,1.5}},
  {{95,255,16},{130,240,14},{172,232,12},{214,218,9},{252,205,7},{292,192,5},{330,176,3},{360,160,1.5}},
  {{98,238,16},{108,205,14},{118,170,11},{126,135,9},{125,100,6},{134,66,4},{150,34,2}},
  {{80,305,11},{52,290,8},{28,288,4}},
}

h = pencil("H")
h:rule({440,HY},{1000,HY},{pressure=0.3})
h:line(dune_top, {pressure=0.4})
h:line(dune_base, {pressure=0.3})
h:line(shore, {pressure=0.3})
h:line(path, {pressure=0.25})
for _, s in ipairs({dolmen_L, dolmen_R, dolmen_C}) do
  local pts = {}
  for _, p in ipairs(s) do pts[#pts+1] = {p[1], p[2]} end
  pts[#pts+1] = {s[1][1], s[1][2]}
  h:line(pts, {pressure=0.45, smooth=true})
end
-- oak: trunk edges and limb centrelines (light)
local function edges(pts)
  local L, R = {}, {}
  for i, p in ipairs(pts) do
    local q = pts[math.min(i+1, #pts)]
    local o = pts[math.max(i-1, 1)]
    local dx, dy = q[1]-o[1], q[2]-o[2]
    local d = math.sqrt(dx*dx+dy*dy)
    local nx, ny = -dy/d, dx/d
    L[#L+1] = {p[1]+nx*p[3]/2, p[2]+ny*p[3]/2}
    R[#R+1] = {p[1]-nx*p[3]/2, p[2]-ny*p[3]/2}
  end
  return L, R
end
local L, R = edges(oak_spine)
h:line(L, {pressure=0.4}); h:line(R, {pressure=0.4})
for _, limb in ipairs(oak_limbs) do
  local l, r = edges(limb)
  h:line(l, {pressure=0.3}); h:line(r, {pressure=0.3})
end
print("ok")

--@ chunk 3
local tests = {
 {{"smalt",4},{"lead white",3}},
 {{"smalt",3},{"lead white",3},{"bone black",0.2}},
 {{"cobalt blue",2},{"lead white",4}},
 {{"cobalt blue",2},{"smalt",2},{"lead white",4}},
 {{"pale smalt",4},{"lead white",3}},
 {{"pale smalt",3},{"lead white",4},{"green earth",1}},

 {{"lead white",6},{"chrome yellow",0.4}},
 {{"lead white",6},{"chrome yellow",0.3},{"vermilion",0.2}},
 {{"lead white",5},{"vermilion",0.5},{"yellow ochre",0.4}},
 {{"lead white",5},{"red earth",0.5},{"vermilion",0.2}},
 {{"lead white",5},{"Rinmann's green",1},{"pale smalt",1}},
 {{"lead white",5},{"green earth",1.5},{"chrome yellow",0.1}},

 {{"raw umber",3},{"bone black",1}},
 {{"bone black",2},{"Prussian blue",1},{"raw umber",2}},
 {{"raw umber",3},{"Prussian blue",1}},
 {{"smalt",3},{"raw umber",2},{"bone black",1}},
 {{"raw umber",2},{"green earth",2},{"bone black",1}},
 {{"raw umber",2},{"red earth",1},{"bone black",1}},

 {{"copper green",2},{"raw umber",2}},
 {{"Prussian blue",1},{"chrome yellow",1},{"raw umber",1}},
 {{"yellow ochre",3},{"raw umber",1}},
 {{"lead white",3},{"raw umber",1},{"red earth",0.5}},
 {{"lead white",4},{"smalt",2},{"vermilion",0.3}},
 {{"lead white",4},{"cobalt blue",1},{"vermilion",0.4}},
}
local idx = 0
for r = 0, 3 do
  for c = 0, 5 do
    idx = idx + 1
    local p = pile(tests[idx])
    work(rect(c*50+2, 555 + r*52, 46, 46), {hand="body", pile=p, coverage=2, clip=true})
  end
end
print("done")

--@ chunk 4
function interp(pts, x)
  if x <= pts[1][1] then return pts[1][2] end
  for i = 2, #pts do
    if x <= pts[i][1] then
      local a, b = pts[i-1], pts[i]
      local t = (x - a[1]) / (b[1] - a[1])
      return a[2] + (b[2]-a[2])*t
    end
  end
  return pts[#pts][2]
end

sky = above(function(x) return math.min(interp(dune_top, x) + 30, HY + 12) end)

sky_top = pile{{"smalt",5},{"cobalt blue",1.5},{"lead white",1.5},{"raw umber",0.2}, medium=0.15}
local band = rect(-40, -40, 1080, 170) * sky
work(band, {hand="broad", pile=sky_top, angle=function(x,y) return 0.04*math.sin(x/170) end,
            coverage=1.6, fill=true, seed=11})
print(drying(500, 50))

--@ chunk 5
sky_b2 = pile{{"smalt",3},{"cobalt blue",2},{"lead white",3}, medium=0.15}
sky_b3 = pile{{"cobalt blue",2},{"smalt",1},{"lead white",5}, medium=0.15}
sky_b4 = pile{{"lead white",6},{"pale smalt",1},{"Rinmann's green",1},{"green earth",0.5}, medium=0.15}
sky_b5 = pile{{"lead white",6},{"chrome yellow",0.6},{"vermilion",0.35},{"yellow ochre",0.3}, medium=0.15}
sky_b6 = pile{{"lead white",6},{"chrome yellow",1.0}, medium=0.15}

local function sband(y0, y1, p, sd)
  work(rect(-40, y0, 1080, y1 - y0) * sky, {hand="broad", pile=p,
       angle=function(x,y) return 0.04*math.sin(x/170 + y/40) end,
       coverage=1.5, fill=true, seed=sd})
end
sband(105, 245, sky_b2, 21)
sband(225, 345, sky_b3, 22)
sband(325, 415, sky_b4, 23)
sband(395, 465, sky_b5, 24)
sband(440, 495, sky_b6, 25)
print(drying(500, 300))
