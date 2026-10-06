-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=800, aspect=1.25, linen={15,13}, seed=7, ground={{pile={{"lead white",7},{"raw umber",1},{"yellow ochre",0.6}}, um=90, apply="knife", texture=0.35},{pile={{"lead white",10},{"raw umber",0.7}}, um=40, apply="brush"}}}

--@ chunk 2

p_wallD = pile{{"raw umber",6},{"bone black",4},{"Prussian blue",1.5}}
p_wallL = pile{{"raw umber",4},{"red earth",1.5},{"yellow ochre",1.2},{"lead white",1}}
p_table = pile{{"raw umber",5},{"red earth",2.5},{"yellow ochre",1.2},{"bone black",1.5}}
tableline = function(x) return 450 + 4*math.sin(x/150 + 1) + 2*math.sin(x/47) end
m_table = below(tableline)
m_wall  = -m_table
pool = mask(function(x, y) local dx=(x-170)/640; local dy=(y-70)/520; local d=math.sqrt(dx*dx+dy*dy); return clamp(1.2 - d, 0, 1) end)
print(m_table:area(), m_wall:area(), pool:area())

--@ chunk 3
print(work(m_wall, {hand="broad", pile=p_wallD, angle=0, coverage=2.2, length={110,240}, tool="filbert 22", edge="lost", seed=5}))

--@ chunk 4

p_wallD2 = pile{{"raw umber",6},{"bone black",3.5},{"cobalt blue",0.8}}
print(work(m_wall, {hand="broad", pile=p_wallD2, angle=0, coverage=2, length={110,240}, tool="filbert 22", edge="lost", fill=true, seed=11}))

--@ chunk 5

print(work(m_table, {hand="broad", pile=p_table, angle=0.12, coverage=2, length={110,220}, tool="filbert 22", edge="lost", fill=true, seed=3}))

--@ chunk 6

gx = mask(function(x, y) return smoothstep(120, 760, x) end)
print(work(m_table, {hand="broad", pile=p_wallD2, angle=0.1, coverage=0.9, length={110,220}, tool="filbert 22", edge="lost", fill=true, seed=21}))
print(work(m_table * gx, {hand="broad", pile=p_wallD2, angle=0.1, coverage=1.6, length={110,220}, tool="filbert 22", edge="lost", fill=true, seed=22}))

--@ chunk 7

poolT = mask(function(x, y) local dx=(x-300)/420; local dy=(y-600)/280; local d=math.sqrt(dx*dx+dy*dy); return clamp(1.15-d,0,1) end)
print(work(m_wall * pool, {hand="broad", pile=p_wallL, angle=0, coverage=1.3, length={110,230}, tool="filbert 22", edge="lost", fill=true, seed=31}))
print(work(m_table * poolT, {hand="broad", pile=p_wallL, angle=0.08, coverage=1.2, length={100,220}, tool="filbert 22", edge="lost", fill=true, seed=32}))

--@ chunk 8

p_dk = pile{{"raw umber",5},{"bone black",4},{"Prussian blue",1.2}}
print(work(everywhere(), {hand="broad", pile=p_dk, angle=0.05, coverage=1.5, length={90,200}, tool="filbert 22", load=0.5, edge="lost", seed=41}))

--@ chunk 9

print(work(everywhere() * gx, {hand="broad", pile=p_dk, angle=0.05, coverage=1.1, length={90,200}, tool="filbert 22", load=0.5, edge="lost", seed=43}))

--@ chunk 10
q = poly({{228,556},{360,528},{520,518}}, true) print(q:area())

--@ chunk 11
cpts = {{228,556},{360,528},{520,518},{700,524},{812,566},{840,640},{802,700},{700,728},{560,744},{430,750},{330,738},{250,700},{204,640}} print(#cpts, cpts[1][1])

--@ chunk 12
c2 = poly(cpts, true):roughen(5, 46, 8) print(c2:area())

--@ chunk 13
c3 = poly(cpts, true):soften(2) print(c3:area())

--@ chunk 14

function apple_mask(cx, cy, rx, ry, dip)
  return mask(function(x, y)
    local dx = (x-cx)/rx; local dy = (y-cy)/ry
    local r = math.sqrt(dx*dx+dy*dy)
    local t = (dy + 0.93)/0.22
    r = r + (dip or 0.16) * math.exp(-t*t)
    return clamp((1 - r) * 7, 0, 1)
  end)
end
m_bowl_out   = ellipse(612, 545, 168, 82)
m_bowl_mouth = ellipse(612, 512, 152, 50)
m_bowl   = m_bowl_out - m_bowl_mouth
y516 = function(x) return 516 end
m_bowl_f = m_bowl * below(y516)
m_apA = apple_mask(524, 500, 60, 56, 0.18)
m_apB = apple_mask(612, 490, 64, 60, 0.20)
m_apC = apple_mask(692, 504, 56, 52, 0.18)
m_apD = apple_mask(322, 652, 72, 67, 0.17)
m_apE = apple_mask(470, 706, 54, 50, 0.16)
m_cloth = poly(cpts, true):roughen(5, 46, 8):soften(2)
print(m_bowl:area(), m_apA:area(), m_apD:area(), m_cloth:area())

--@ chunk 15

p_cloth = pile{{"lead white",10},{"yellow ochre",0.7},{"raw umber",0.4}}
p_bowl  = pile{{"yellow ochre",5},{"red earth",2.5},{"lead white",3},{"bone black",0.4}}
p_red   = pile{{"vermilion",6},{"red earth",2},{"yellow ochre",0.8},{"bone black",0.6}}
p_grn   = pile{{"green earth",5},{"yellow ochre",2.5},{"chrome yellow",0.8},{"lead white",1}}
print(work(m_cloth, {hand="broad", pile=p_cloth, angle=0.25, coverage=1.7, length={70,150}, tool="filbert 16", edge="soft", seed=51}))
print(work(m_bowl, {hand="body", pile=p_bowl, coverage=2, edge="soft", seed=52}))
print(work(m_apB, {hand="body", pile=p_grn, coverage=2, edge="soft", seed=53}))
print(work(m_apA, {hand="body", pile=p_red, coverage=2, edge="soft", seed=54}))
print(work(m_apC, {hand="body", pile=p_red, coverage=2, edge="soft", seed=55}))
print(work(m_apD, {hand="body", pile=p_red, coverage=2, edge="soft", seed=56}))
print(work(m_apE, {hand="body", pile=p_grn, coverage=2, edge="soft", seed=57}))

--@ chunk 16

print(work(m_cloth, {hand="broad", pile=p_cloth, angle=0.35, coverage=3.2, length={60,130}, tool="filbert 16", load=0.6, edge="soft", seed=61}))

--@ chunk 17

p_csh = pile{{"lead white",4},{"raw umber",2.6},{"cobalt blue",1.1},{"Prussian blue",0.4}}
m_cast = ellipse(706, 638, 195, 60):soften(34)
m_castD = ellipse(392, 692, 92, 32):soften(26)
m_castE = ellipse(524, 732, 70, 24):soften(20)
m_shad = ((m_cast + m_castD + m_castE):grow(8)) * m_cloth
print(work(m_shad, {hand="body", pile=p_csh, coverage=2, tool="filbert 14", angle=0.3, edge="soft", seed=71}))

--@ chunk 18

p_cloth2 = pile{{"lead white",6},{"raw umber",2.6},{"yellow ochre",0.9}}
p_csh2 = pile{{"lead white",3},{"raw umber",4.5},{"cobalt blue",0.6},{"yellow ochre",0.5}}
print(work(m_cloth, {hand="broad", pile=p_cloth2, angle=0.35, coverage=1.7, length={70,150}, tool="filbert 16", load=0.55, edge="soft", seed=72}))
print(work(m_shad, {hand="body", pile=p_csh2, coverage=2.4, tool="filbert 11", angle=0.3, edge="soft", seed=73}))

--@ chunk 19

cpts2 = {{236,540},{400,512},{580,506},{740,522},{818,556},{846,624},{826,676},{760,706},{664,724},{566,754},{462,760},{356,748},{272,730},{208,692},{186,626},{200,576}}
m_cloth = poly(cpts2):roughen(6, 44, 3):soften(2)
print(work(m_cloth, {hand="broad", pile=p_cloth2, angle=0.35, coverage=1.8, length={70,150}, tool="filbert 16", load=0.55, edge="soft", seed=74}))

--@ chunk 20

cpts3 = {{250,552},{420,518},{560,502},{620,500},{770,526},{834,564},{792,598},{852,634},{802,692},{704,708},{642,748},{560,720},{470,766},{392,730},{300,754},{248,708},{172,692},{212,642},{194,588}}
m_cloth = poly(cpts3):roughen(4, 30, 5):soften(1.6)
print(work(m_cloth, {hand="broad", pile=p_cloth2, angle=0.35, coverage=1.8, length={70,150}, tool="filbert 16", load=0.55, edge="soft", seed=75}))

--@ chunk 21

m_pen = (ellipse(722, 648, 205, 68):soften(32) + ellipse(402, 696, 98, 34):soften(24) + ellipse(540, 738, 76, 26):soften(20)) * m_cloth
m_core = (ellipse(676, 636, 118, 40):soften(20) + ellipse(374, 692, 64, 22):soften(15) + ellipse(508, 732, 48, 17):soften(13)) * m_cloth
cr1 = ribbon({{556,640},{500,676},{436,700}}, 26):soften(14)
cr2 = ribbon({{648,646},{700,672},{742,706}}, 22):soften(13)
cr3 = ribbon({{402,700},{372,730},{352,752}}, 18):soften(11)
cr4 = ribbon({{520,712},{500,742},{486,766}}, 20):soften(12)
m_crease = (cr1 + cr2 + cr3 + cr4) * m_cloth
print(work(m_pen, {hand="body", pile=p_csh2, coverage=1.6, tool="filbert 12", angle=0.3, edge="soft", seed=81}))
print(work(m_core, {hand="body", pile=p_csh2, coverage=1.8, tool="filbert 8", angle=0.3, edge="soft", seed=82}))
print(work(m_crease, {hand="body", pile=p_csh2, coverage=1.5, tool="filbert 7", angle=0.55, edge="soft", seed=83}))

--@ chunk 22

p_red2 = pile{{"vermilion",5},{"red earth",3},{"raw umber",1.6},{"yellow ochre",0.5},{"bone black",0.5}}
p_grn2 = pile{{"green earth",5},{"yellow ochre",2.2},{"raw umber",1},{"chrome yellow",0.5}}
print(work(m_bowl, {hand="body", pile=p_bowl, coverage=2.6, edge="soft", seed=91}))
print(work(m_apB, {hand="body", pile=p_grn2, coverage=2.6, edge="soft", seed=92}))
print(work(m_apA, {hand="body", pile=p_red2, coverage=2.6, edge="soft", seed=93}))
print(work(m_apC, {hand="body", pile=p_red2, coverage=2.6, edge="soft", seed=94}))

--@ chunk 23

sD = body.ellipsoid({322, 652, 0}, {72, 67, 62})
fD = form{sD, light={from={-0.75,-0.62}, front=0.45, ambient=0.12}}
litD = fD:lit{parts={1}, soft=0.1}
local out = ""
for _, p in ipairs({{272,614},{322,652},{372,690},{300,690},{350,620},{322,600},{250,652},{322,712}}) do
  out = out .. string.format("(%d,%d)=%.2f ", p[1], p[2], litD:at(p[1], p[2]))
end
print(out)
print(litD:band(0.45, 0.72, 0.1):area(), litD:band(0.78, 1.0, 0.07):area())

--@ chunk 24

p_red_sh = pile{{"vermilion",3},{"red earth",2},{"raw umber",2},{"bone black",1.2},{"Prussian blue",0.3}}
p_red_lt = pile{{"vermilion",4},{"red earth",2},{"yellow ochre",1.6},{"lead white",1.5}}
p_grn_sh = pile{{"green earth",4},{"raw umber",3},{"bone black",1},{"Prussian blue",0.4}}
p_grn_lt = pile{{"green earth",3},{"yellow ochre",3},{"chrome yellow",1},{"lead white",2}}
function apple_form(cx, cy, rx, ry, rz)
  local s = body.ellipsoid({cx, cy, 0}, {rx, ry, rz or ry})
  return form{s, light={from={-0.75,-0.62}, front=0.45, ambient=0.12}}
end
fD = apple_form(322, 652, 72, 67, 62)
litD = fD:lit{parts={1}, soft=0.42}
print(work(m_apD, {hand="body", pile=p_red2, coverage=2.2, edge="soft", seed=101}))
print(work(m_apD * (-litD), {hand="body", pile=p_red_sh, coverage=1.9, tool="filbert 9", angle=0.5, edge="soft", seed=102}))
print(work(m_apD * litD:band(0.62, 1.0, 0.28), {hand="body", pile=p_red_lt, coverage=1.3, tool="filbert 9", angle=0.5, edge="lost", seed=103}))

--@ chunk 25

cpts4 = {{300,540},{450,506},{620,496},{780,512},{852,552},{886,614},{846,660},{770,700},{700,742},{600,780},{500,786},{420,772},{330,758},{250,724},{150,700},{108,640},{128,586},{210,556}}
m_cloth = poly(cpts4):roughen(7, 38, 9):soften(2)
y490 = function(x) return 490 end
print(work(below(y490), {hand="broad", pile=p_dk, angle=0.08, coverage=1.3, length={90,200}, tool="filbert 22", load=0.5, edge="lost", seed=111}))

--@ chunk 26

m_bowl_out = ellipse(600, 560, 200, 100)
m_bowl_mouth = ellipse(600, 524, 182, 62)
m_bowl = m_bowl_out - m_bowl_mouth
y530 = function(x) return 530 end
m_bowl_f = m_bowl * below(y530)
m_apA = apple_mask(500, 508, 74, 69, 0.18)
m_apB = apple_mask(608, 494, 80, 74, 0.20)
m_apC = apple_mask(700, 514, 70, 64, 0.18)
m_apD = apple_mask(300, 690, 92, 86, 0.17)
m_apE = apple_mask(470, 730, 68, 63, 0.16)
print(work(m_cloth, {hand="broad", pile=p_cloth2, angle=0.35, coverage=2.4, length={70,150}, tool="filbert 16", load=0.6, edge="soft", seed=112}))
print(work(m_bowl, {hand="body", pile=p_bowl, coverage=2.4, edge="soft", seed=113}))
print(work(m_apB, {hand="body", pile=p_grn2, coverage=2.4, edge="soft", seed=114}))
print(work(m_apA, {hand="body", pile=p_red2, coverage=2.4, edge="soft", seed=115}))
print(work(m_apC, {hand="body", pile=p_red2, coverage=2.4, edge="soft", seed=116}))
print(work(m_apD, {hand="body", pile=p_red2, coverage=2.4, edge="soft", seed=117}))
print(work(m_apE, {hand="body", pile=p_grn2, coverage=2.4, edge="soft", seed=118}))

--@ chunk 27
print(wait(300), drying(500, 700), drying(700, 620), drying(300, 690), drying(200, 200))

--@ chunk 28
print(wait(420), drying(500, 700), drying(700, 620), drying(300, 690))

--@ chunk 29

fA = apple_form(500, 508, 74, 69, 64); lA = fA:lit{parts={1}, soft=0.42}
fC = apple_form(700, 514, 70, 64, 60); lC = fC:lit{parts={1}, soft=0.42}
fD = apple_form(300, 690, 92, 86, 80); lD = fD:lit{parts={1}, soft=0.42}
print(work(m_apA, {hand="body", pile=p_red2, coverage=2.2, edge="soft", seed=121}))
print(work(m_apC, {hand="body", pile=p_red2, coverage=2.2, edge="soft", seed=122}))
print(work(m_apD, {hand="body", pile=p_red2, coverage=2.2, edge="soft", seed=123}))
print(work(m_apA * (-lA), {hand="body", pile=p_red_sh, coverage=1.8, tool="filbert 9", angle=0.5, edge="soft", seed=124}))
print(work(m_apC * (-lC), {hand="body", pile=p_red_sh, coverage=1.8, tool="filbert 9", angle=0.5, edge="soft", seed=125}))
print(work(m_apD * (-lD), {hand="body", pile=p_red_sh, coverage=1.8, tool="filbert 9", angle=0.5, edge="soft", seed=126}))
print(work(m_apA * lA:band(0.62, 1.0, 0.3), {hand="body", pile=p_red_lt, coverage=1.2, tool="filbert 9", angle=0.5, edge="lost", seed=127}))
print(work(m_apC * lC:band(0.62, 1.0, 0.3), {hand="body", pile=p_red_lt, coverage=1.2, tool="filbert 9", angle=0.5, edge="lost", seed=128}))
print(work(m_apD * lD:band(0.62, 1.0, 0.3), {hand="body", pile=p_red_lt, coverage=1.2, tool="filbert 9", angle=0.5, edge="lost", seed=129}))

--@ chunk 30

p_red3 = pile{{"vermilion",4},{"red earth",3},{"raw umber",3},{"bone black",1.2},{"yellow ochre",0.4}}
p_red_lt3 = pile{{"vermilion",4},{"red earth",2.5},{"yellow ochre",2},{"lead white",1.2}}
p_red_sh3 = pile{{"raw umber",3},{"red earth",1.5},{"bone black",2},{"Prussian blue",0.6},{"vermilion",0.5}}
p_hi = pile{{"lead white",8},{"yellow ochre",1.2},{"chrome yellow",0.4}}
print(work(m_apD, {hand="body", pile=p_red3, coverage=2.2, edge="soft", seed=131}))
print(work(m_apD * (-lD), {hand="body", pile=p_red_sh3, coverage=2, tool="filbert 9", angle=0.5, edge="soft", seed=132}))
print(work(m_apD * lD:band(0.3, 0.58, 0.12), {hand="body", pile=p_red_sh3, coverage=0.8, tool="filbert 6", angle=0.5, edge="lost", seed=133}))
print(work(m_apD * lD:band(0.66, 1.0, 0.26), {hand="body", pile=p_red_lt3, coverage=1.3, tool="filbert 9", angle=0.5, edge="lost", seed=134}))

--@ chunk 31

lDs = fD:lit{parts={1}, soft=0.9}
print(work(m_apD * (-lDs), {hand="body", pile=p_red_sh3, coverage=1.7, tool="filbert 10", angle=fD:field("fall"), edge="lost", seed=141}))
print(work(m_apD * fD:lit{parts={1}, soft=0.5}:band(0.25, 0.52, 0.2), {hand="body", pile=p_red_sh3, coverage=0.7, tool="filbert 6", angle=fD:field("fall"), edge="lost", seed=142}))
print(work(m_apD * fD:lit{parts={1}, soft=0.5}:band(0.74, 1.0, 0.3), {hand="body", pile=p_red_lt3, coverage=1.0, tool="filbert 10", angle=fD:field("fall"), edge="lost", seed=143}))
hb = brush{kind="round", width=4.5, point=1, stiffness=0.4}
hb:load(p_hi, 0.9)
hb:touch(256, 646, {pressure=0.75})
hb:touch(252, 656, {pressure=0.5})
hb:touch(266, 638, {pressure=0.45})

--@ chunk 32

p_rb = pile{{"vermilion",3},{"red earth",4},{"raw umber",3},{"bone black",1},{"yellow ochre",0.3}}
p_rl = pile{{"vermilion",3.5},{"red earth",3},{"yellow ochre",1},{"lead white",1.6}}
p_rs = pile{{"red earth",2},{"raw umber",2},{"bone black",1.6},{"Prussian blue",0.5},{"vermilion",0.8}}
p_refr = pile{{"lead white",4},{"cobalt blue",0.7},{"red earth",1.2},{"raw umber",0.6}}
print(work(m_apD, {hand="body", pile=p_rb, coverage=2.2, clip=true, edge="found", seed=151}))
print(work(m_apD * fD:lit{parts={1}, soft=0.5}:band(0.7, 1.0, 0.3), {hand="body", pile=p_rl, coverage=1.0, tool="filbert 10", angle=fD:field("fall"), clip=true, edge="lost", seed=152}))
print(work(m_apD * (-lDs), {hand="body", pile=p_rs, coverage=1.6, tool="filbert 10", angle=fD:field("fall"), clip=true, edge="lost", seed=153}))
print(work(m_apD * fD:lit{parts={1}, soft=0.5}:band(0.22, 0.5, 0.22), {hand="body", pile=p_rs, coverage=0.8, tool="filbert 6", angle=fD:field("fall"), clip=true, edge="lost", seed=154}))

--@ chunk 33

print(work(m_apD, {hand="body", pile=p_rs, coverage=3, tool="filbert 14", clip=true, edge="found", seed=161}))

--@ chunk 34

p_dark = pile{{"raw umber",5},{"bone black",4},{"Prussian blue",1.2}}
p_white = pile{{"lead white",10},{"yellow ochre",0.6},{"raw umber",0.3}}
p_bowl2 = pile{{"yellow ochre",3.5},{"red earth",3},{"raw umber",1.2},{"bone black",0.5}}
p_rs2 = pile{{"bone black",3},{"Prussian blue",1},{"raw umber",1.5},{"red earth",1},{"vermilion",0.4}}
p_rb2 = pile{{"vermilion",4},{"red earth",2},{"raw umber",1},{"bone black",0.4}}
p_rl2 = pile{{"lead white",2},{"vermilion",3},{"red earth",1.5},{"yellow ochre",1}}
p_clothmid = pile{{"lead white",6},{"raw umber",2.6},{"yellow ochre",0.9}}
p_gs2 = pile{{"bone black",2},{"Prussian blue",0.8},{"green earth",2},{"raw umber",1.5}}
p_gb2 = pile{{"green earth",5},{"yellow ochre",2},{"raw umber",1},{"chrome yellow",0.5}}
p_gl2 = pile{{"chrome yellow",2},{"lead white",2},{"green earth",2},{"yellow ochre",2}}
print(p_rs2, p_rb2, p_rl2)

--@ chunk 35

print(work(m_apD, {hand="body", pile=p_rb2, coverage=2.5, tool="filbert 12", clip=true, edge="found", seed=171}))
print(work(m_apD * fD:lit{parts={1}, soft=0.5}:band(0.68, 1.0, 0.3), {hand="body", pile=p_rl2, coverage=1.3, tool="filbert 10", angle=fD:field("fall"), clip=true, edge="lost", seed=172}))
print(work(m_apD * (-lDs), {hand="body", pile=p_rs2, coverage=1.9, tool="filbert 10", angle=fD:field("fall"), clip=true, edge="lost", seed=173}))
print(work(m_apD * fD:lit{parts={1}, soft=0.5}:band(0.2, 0.5, 0.22), {hand="body", pile=p_rs2, coverage=0.7, tool="filbert 6", angle=fD:field("fall"), clip=true, edge="lost", seed=174}))

--@ chunk 36

sD2 = body.ellipsoid({300, 690, 0}, {92, 86, 80})
fD2 = form{sD2, light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
lD2 = fD2:lit{parts={1}, soft=1.1}
print(work(m_apD * (-lD2), {hand="body", pile=p_rs2, coverage=1.8, tool="filbert 12", angle=fD2:field("fall"), clip=true, edge="lost", seed=181}))
print(work(m_apD * lD2:band(0.25, 0.55, 0.3), {hand="body", pile=p_rs2, coverage=0.6, tool="filbert 7", angle=fD2:field("fall"), clip=true, edge="lost", seed=182}))
print(work(m_apD * lD2:band(0.72, 1.0, 0.34), {hand="body", pile=p_rl2, coverage=1.2, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="lost", seed=183}))
hb = brush{kind="round", width=5, point=1, stiffness=0.35}
hb:load(p_hi, 0.95)
hb:touch(250, 650, {pressure=0.8})
hb:touch(246, 662, {pressure=0.45})
hb:touch(262, 640, {pressure=0.4})

--@ chunk 37

lD3 = fD2:lit{parts={1}, soft=0.55}
print(work(m_apD * (-lD3), {hand="body", pile=p_rs2, coverage=1.8, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="lost", seed=191}))
print(work(m_apD * lD3:band(0.74, 1.0, 0.2), {hand="body", pile=p_rl2, coverage=1.4, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="lost", seed=192}))
print(work(m_apD * lD3:band(0.34, 0.6, 0.16), {hand="body", pile=p_rs2, coverage=0.55, tool="filbert 6", angle=fD2:field("fall"), clip=true, edge="lost", seed=193}))
hb:reload(p_hi, 0.9)
hb:touch(248, 652, {pressure=0.85})
hb:touch(244, 664, {pressure=0.5})
hb:touch(260, 641, {pressure=0.42})
hb:touch(268, 668, {pressure=0.25})

--@ chunk 38

print(work(m_apD * (-lD3), {hand="body", pile=p_rs2, coverage=1.5, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="lost", seed=201}))
print(work(m_apD * (-lD3), {hand="glaze", pile=p_rb2, coverage=0.5, tool="filbert 20", angle=fD2:field("fall"), clip=true, edge="lost", seed=202}))
print(work(m_apD * lD3:band(0.74, 1.0, 0.2), {hand="body", pile=p_rl2, coverage=1.3, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="lost", seed=203}))
print(work(m_apD * lD3:band(0.74, 1.0, 0.2), {hand="glaze", pile=p_white, coverage=0.45, tool="filbert 20", angle=fD2:field("fall"), clip=true, edge="lost", seed=204}))
print(work(m_apD * lD3:band(0.34, 0.6, 0.16), {hand="body", pile=p_rs2, coverage=0.5, tool="filbert 6", angle=fD2:field("fall"), clip=true, edge="lost", seed=205}))

--@ chunk 39

print(work(m_apD:grow(9) - m_apD, {hand="body", pile=p_clothmid, coverage=1.8, tool="filbert 8", clip=true, edge="lost", seed=211}))
print(work(m_apD * lD3:band(0.7, 1.0, 0.22), {hand="body", pile=p_rl2, coverage=1.2, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="lost", pressure={0.6,0.9}, seed=212}))

--@ chunk 40

print(work(m_apD:grow(20) - m_apD, {hand="body", pile=p_clothmid, coverage=2, tool="filbert 14", clip=true, edge="found", seed=221}))
print(work(m_apD, {hand="body", pile=p_rb2, coverage=2.6, tool="filbert 12", clip=true, edge="found", seed=222}))
print(work(m_apD * (-lD3), {hand="body", pile=p_rs2, coverage=1.5, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="soft", seed=223}))
print(work(m_apD * lD3:band(0.72, 1.0, 0.2), {hand="body", pile=p_rl2, coverage=1.3, tool="filbert 10", angle=fD2:field("fall"), clip=true, edge="soft", seed=224}))
print(work(m_apD * lD3:band(0.36, 0.58, 0.14), {hand="body", pile=p_rs2, coverage=0.5, tool="filbert 6", angle=fD2:field("fall"), clip=true, edge="lost", seed=225}))

--@ chunk 41

p_csh2 = pile{{"lead white",3},{"raw umber",4.5},{"cobalt blue",0.6},{"yellow ochre",0.5}}
p_bowls = pile{{"raw umber",4},{"bone black",1.5},{"cobalt blue",0.4},{"red earth",1}}
p_tab = pile{{"raw umber",4},{"red earth",2},{"yellow ochre",1},{"bone black",1}}
p_refr2 = pile{{"lead white",5},{"cobalt blue",1},{"raw umber",0.8}}
print(p_csh2, p_bowls, p_tab, p_refr2)

--@ chunk 42

s1 = (ellipse(700, 646, 220, 66):soften(34) + ellipse(600, 620, 150, 46):soften(26)) * m_cloth
s2 = ribbon({{382, 690}, {404, 726}, {418, 764}}, 30):soften(15) * m_cloth
s3 = ribbon({{492, 646}, {416, 690}, {352, 736}}, 26):soften(14) * m_cloth
s4 = ribbon({{664, 646}, {740, 678}, {812, 700}}, 24):soften(13) * m_cloth
s5 = m_cloth:rim(22, 11) * below(function(x) return 690 end)
s6 = (ellipse(830, 610, 90, 90):soften(34)) * m_cloth
print(work(s1, {hand="body", pile=p_csh2, coverage=1.8, tool="filbert 14", angle=0.25, clip=true, edge="soft", seed=231}))
print(work(s2 + s3 + s4, {hand="body", pile=p_csh2, coverage=1.6, tool="filbert 8", angle=0.7, clip=true, edge="soft", seed=232}))
print(work(s5, {hand="body", pile=p_csh2, coverage=1.5, tool="filbert 9", angle=0.1, clip=true, edge="soft", seed=233}))
print(work(s6, {hand="body", pile=p_csh2, coverage=1.5, tool="filbert 12", angle=0.4, clip=true, edge="soft", seed=234}))

--@ chunk 43

p_hi = pile{{"lead white",8},{"yellow ochre",1.2},{"chrome yellow",0.4}}
p_csh3 = pile{{"lead white",1.2},{"raw umber",3.5},{"bone black",1.6},{"cobalt blue",0.6},{"red earth",0.4}}
print(p_hi, p_csh3)
print(work(m_cloth, {hand="broad", pile=p_clothmid, angle=0.3, coverage=2.2, length={70,150}, tool="filbert 16", load=0.6, clip=true, edge="lost", seed=241}))
print(work(m_bowl, {hand="body", pile=p_bowl2, coverage=2.4, tool="filbert 12", clip=true, edge="soft", seed=242}))

--@ chunk 44

print(work(s1, {hand="body", pile=p_csh3, coverage=2, tool="filbert 14", angle=0.25, clip=true, edge="soft", seed=251}))
print(work(s2 + s3 + s4, {hand="body", pile=p_csh3, coverage=1.8, tool="filbert 8", angle=0.7, clip=true, edge="soft", seed=252}))
print(work(s5, {hand="body", pile=p_csh3, coverage=1.7, tool="filbert 9", angle=0.1, clip=true, edge="soft", seed=253}))
print(work(s6, {hand="body", pile=p_csh3, coverage=1.7, tool="filbert 12", angle=0.4, clip=true, edge="soft", seed=254}))

--@ chunk 45
print(drying(700, 640), drying(300, 600), drying(600, 700), drying(600, 560), drying(200, 300))

--@ chunk 46
print(wait(1440), drying(700,640), drying(300,600), drying(600,700))

--@ chunk 47
print(wait(1440), drying(700,640), drying(300,600), drying(600,700))

--@ chunk 48
print(wait(2880), drying(700,640), drying(300,600), drying(600,700), drying(600,560), drying(200,300))

--@ chunk 49

print(work(s1, {hand="body", pile=p_csh3, coverage=1.6, tool="filbert 14", angle=0.25, clip=true, edge="soft", seed=261}))
print(work(s2 + s3 + s4, {hand="body", pile=p_csh3, coverage=1.6, tool="filbert 8", angle=0.7, clip=true, edge="soft", seed=262}))
print(work(s5, {hand="body", pile=p_csh3, coverage=1.5, tool="filbert 9", angle=0.1, clip=true, edge="soft", seed=263}))
print(work(s6, {hand="body", pile=p_csh3, coverage=1.5, tool="filbert 12", angle=0.4, clip=true, edge="soft", seed=264}))

--@ chunk 50

s1w = (ellipse(720, 648, 230, 70):soften(50) + ellipse(600, 620, 160, 48):soften(36)) * m_cloth
s1c = (ellipse(672, 636, 120, 36):soften(22) + ellipse(566, 618, 90, 28):soften(18)) * m_cloth
print(work(s1w, {hand="body", pile=p_csh2, coverage=1.7, tool="filbert 16", angle=0.25, clip=true, edge="lost", seed=271}))
print(work(s1c, {hand="body", pile=p_csh3, coverage=1.3, tool="filbert 10", angle=0.25, clip=true, edge="lost", seed=272}))
print(work(s6, {hand="body", pile=p_csh2, coverage=1.5, tool="filbert 14", angle=0.4, clip=true, edge="lost", seed=273}))

--@ chunk 51

print(work(s2 + s3 + s4, {hand="body", pile=p_csh3, coverage=1.1, tool="filbert 7", angle=0.7, clip=true, edge="lost", seed=281}))
lft = (ellipse(300, 636, 150, 90):soften(50) + ellipse(240, 700, 110, 60):soften(40)) * m_cloth
roll = (ribbon({{250, 726}, {420, 764}, {580, 772}, {700, 752}}, 34):soften(16)) * m_cloth
print(work(lft, {hand="body", pile=p_white, coverage=1.3, tool="filbert 16", angle=0.3, clip=true, edge="lost", seed=282}))
print(work(roll, {hand="body", pile=p_white, coverage=1.2, tool="filbert 12", angle=0.15, clip=true, edge="lost", seed=283}))

--@ chunk 52

print(work(m_cloth, {hand="broad", pile=p_csh2, angle=0.3, coverage=1.7, length={80,160}, tool="filbert 18", load=0.6, clip=true, edge="found", seed=291}))

--@ chunk 53

function model_apple(mm, ff, pl, ps, plt, sd)
  local l = ff:lit{parts={1}, soft=0.5}
  work(mm, {hand="body", pile=pl, coverage=2.4, tool="filbert 12", clip=true, edge="found", seed=sd})
  work(mm * (-l), {hand="body", pile=ps, coverage=1.5, tool="filbert 10", angle=ff:field("fall"), clip=true, edge="soft", seed=sd+1})
  work(mm * l:band(0.72, 1.0, 0.2), {hand="body", pile=plt, coverage=1.2, tool="filbert 10", angle=ff:field("fall"), clip=true, edge="soft", seed=sd+2})
  work(mm * l:band(0.34, 0.58, 0.16), {hand="body", pile=ps, coverage=0.45, tool="filbert 6", angle=ff:field("fall"), clip=true, edge="lost", seed=sd+3})
end
fA2 = form{body.ellipsoid({500, 508, 0}, {74, 69, 64}), light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
fC2 = form{body.ellipsoid({700, 514, 0}, {70, 64, 60}), light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
model_apple(m_apA, fA2, p_rb2, p_rs2, p_rl2, 301)
model_apple(m_apC, fC2, p_rb2, p_rs2, p_rl2, 311)
model_apple(m_apD, fD2, p_rb2, p_rs2, p_rl2, 321)

--@ chunk 54

m_bowl_out = ellipse(596, 566, 176, 92)
m_bowl_mouth = ellipse(596, 530, 158, 54)
m_bowl = m_bowl_out - m_bowl_mouth
y534 = function(x) return 534 end
m_bowl_f = m_bowl * below(y534)
m_apA = apple_mask(512, 516, 86, 80, 0.18)
m_apB = apple_mask(606, 498, 92, 86, 0.20)
m_apC = apple_mask(696, 520, 80, 75, 0.18)
m_apD = apple_mask(300, 690, 92, 86, 0.17)
m_apE = apple_mask(478, 736, 72, 66, 0.16)
print(work(m_cloth, {hand="broad", pile=p_csh2, angle=0.3, coverage=1.5, length={80,160}, tool="filbert 18", load=0.6, clip=true, edge="found", seed=331}))
print(work(m_bowl, {hand="body", pile=p_bowl2, coverage=2.4, tool="filbert 12", clip=true, edge="soft", seed=332}))

--@ chunk 55
print(work(m_apE, {hand="body", pile=p_gb2, coverage=1, tool="filbert 9", angle=0.45, curve={0.25,0.15}, clip=true, seed=1}))

--@ chunk 56

function model_apple(mm, ff, pl, ps, plt, sd)
  local l = ff:lit{parts={1}, soft=0.5}
  work(mm, {hand="body", pile=pl, coverage=2.4, tool="filbert 12", clip=true, edge="found", seed=sd})
  work(mm * (-l), {hand="body", pile=ps, coverage=1.5, tool="filbert 9", angle=0.45, curve={0.25, 0.15}, clip=true, edge="soft", seed=sd+1})
  work(mm * l:band(0.72, 1.0, 0.2), {hand="body", pile=plt, coverage=1.2, tool="filbert 9", angle=-0.7, curve={0.3, 0.18}, clip=true, edge="soft", seed=sd+2})
  work(mm * l:band(0.34, 0.58, 0.16), {hand="body", pile=ps, coverage=0.45, tool="filbert 6", angle=0.45, clip=true, edge="lost", seed=sd+3})
end
fB3 = form{body.ellipsoid({606, 498, 0}, {92, 86, 82}), light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
fA3 = form{body.ellipsoid({512, 516, 0}, {86, 80, 76}), light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
fC3 = form{body.ellipsoid({696, 520, 0}, {80, 75, 70}), light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
fE3 = form{body.ellipsoid({478, 736, 0}, {72, 66, 62}), light={from={-0.88,-0.5}, front=0.35, ambient=0.1}}
model_apple(m_apB, fB3, p_gb2, p_gs2, p_gl2, 341)
model_apple(m_apA, fA3, p_rb2, p_rs2, p_rl2, 351)
model_apple(m_apC, fC3, p_rb2, p_rs2, p_rl2, 361)
model_apple(m_apD, fD2, p_rb2, p_rs2, p_rl2, 371)
model_apple(m_apE, fE3, p_gb2, p_gs2, p_gl2, 381)

--@ chunk 57

mi = m_bowl_mouth - (m_apA + m_apB + m_apC)
print(mi:area(), m_bowl_f:area())
print(work(mi, {hand="body", pile=p_bowls, coverage=1.6, tool="filbert 10", angle=0.3, clip=true, edge="soft", seed=391}))
print(work(m_bowl_f, {hand="body", pile=p_bowl2, coverage=1.8, tool="filbert 12", angle=0.2, clip=true, edge="soft", seed=392}))
bowlR = m_bowl * mask(function(x, y) return smoothstep(560, 700, x) end)
print(work(bowlR, {hand="body", pile=p_bowls, coverage=1.4, tool="filbert 10", angle=0.2, clip=true, edge="lost", seed=393}))
bowlL = m_bowl * mask(function(x, y) return smoothstep(560, 470, x) end)
print(work(bowlL, {hand="body", pile=p_white, coverage=0.45, tool="filbert 12", angle=0.2, clip=true, edge="lost", pressure={0.5,0.8}, seed=394}))

--@ chunk 58

function refl_mask(mm, cx, cy, rx, ry)
  return mm * (ellipse(cx + 0.5*rx, cy + 0.68*ry, 0.5*rx, 0.26*ry):soften(14))
end
print(work(refl_mask(m_apA, 512, 516, 86, 80), {hand="body", pile=p_refr2, coverage=0.9, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=401}))
print(work(refl_mask(m_apB, 606, 498, 92, 86), {hand="body", pile=p_refr2, coverage=0.9, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=402}))
print(work(refl_mask(m_apC, 696, 520, 80, 75), {hand="body", pile=p_refr2, coverage=0.9, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=403}))
print(work(refl_mask(m_apD, 300, 690, 92, 86), {hand="body", pile=p_refr2, coverage=0.9, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=404}))
print(work(refl_mask(m_apE, 478, 736, 72, 66), {hand="body", pile=p_refr2, coverage=0.9, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=405}))
hb = brush{kind="round", width=6, point=1, stiffness=0.3}
hb:load(p_hi, 1.0)
for _, p in ipairs({{476,479},{612,437},{662,487},{261,650},{448,706}}) do
  hb:touch(p[1], p[2], {pressure=0.85})
end

--@ chunk 59

cpts5 = {{330,548},{460,522},{620,514},{780,530},{852,566},{884,620},{846,668},{806,700},{830,762},{700,812},{420,830},{160,806},{52,742},{36,656},{86,592},{180,560}}
m_cloth2 = poly(cpts5):roughen(6, 40, 13):soften(2)
m_outside = m_table - m_cloth2
print(work(m_outside, {hand="broad", pile=p_tab, angle=0.1, coverage=1.6, length={90,200}, tool="filbert 22", load=0.55, clip=true, edge="found", seed=411}))

--@ chunk 60

m_fig = m_cloth2 + m_bowl + m_apA + m_apB + m_apC + m_apD + m_apE
m_bg = everywhere() - m_fig
print(work(m_bg, {hand="broad", pile=p_dark, angle=0.05, coverage=0.75, length={100,220}, tool="filbert 22", load=0.5, clip=true, edge="lost", seed=421}))
print(work(m_bg * pool, {hand="broad", pile=p_bowl2, angle=0, coverage=0.7, length={110,230}, tool="filbert 22", load=0.45, clip=true, edge="lost", seed=422}))

--@ chunk 61

print(work(m_bg, {hand="body", pile=p_dark, coverage=3, tool="filbert 14", angle=0.05, curve={0.2,0.1}, clip=true, edge="found", seed=431}))
print(work(m_bg * pool, {hand="body", pile=p_bowls, coverage=0.9, tool="filbert 22", angle=0, curve={0.2,0.1}, clip=true, edge="lost", seed=432}))

--@ chunk 62

m_clothonly = m_cloth2 - (m_bowl + m_apA + m_apB + m_apC + m_apD + m_apE)
print(m_clothonly:area())
print(work(m_clothonly, {hand="broad", pile=p_csh2, angle=0.3, coverage=1.6, length={80,160}, tool="filbert 18", load=0.6, clip=true, edge="found", seed=441}))

--@ chunk 63

print(work(m_clothonly, {hand="broad", pile=p_csh3, angle=0.35, coverage=1.6, length={80,160}, tool="filbert 18", load=0.6, clip=true, edge="found", seed=451}))
crest1 = (ribbon({{120, 792}, {250, 726}, {360, 660}, {440, 600}}, 78):soften(26)) * m_clothonly
print(work(crest1, {hand="body", pile=p_clothmid, coverage=1.7, tool="filbert 16", angle=0.75, curve={0.2,0.12}, clip=true, edge="lost", seed=452}))

--@ chunk 64
print(drying(500,700), drying(200,650)) print(work(m_clothonly, {hand="body", pile=p_csh3, coverage=3, tool="filbert 12", angle=0.3, clip=true, edge="found", seed=461}))

--@ chunk 65

c1 = (ribbon({{100, 786}, {230, 720}, {350, 648}, {440, 588}}, 80):soften(24)) * m_clothonly
c2 = (ribbon({{636, 748}, {740, 708}, {830, 662}}, 56):soften(20)) * m_clothonly
print(work(c1, {hand="body", pile=p_clothmid, coverage=2.2, tool="filbert 16", angle=0.75, curve={0.2,0.12}, clip=true, edge="lost", seed=471}))
print(work(c2, {hand="body", pile=p_clothmid, coverage=2, tool="filbert 14", angle=0.9, clip=true, edge="lost", seed=472}))
d1 = (ellipse(724, 652, 200, 58):soften(40) + ellipse(624, 626, 130, 38):soften(28)) * m_clothonly
d3 = (ellipse(392, 702, 88, 24):soften(20)) * m_clothonly
d4 = (ellipse(542, 744, 64, 20):soften(16)) * m_clothonly
print(work(d1, {hand="body", pile=p_dark, coverage=1.8, tool="filbert 14", angle=0.25, clip=true, edge="lost", seed=473}))
print(work(d3 + d4, {hand="body", pile=p_dark, coverage=1.6, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=474}))

--@ chunk 66

p_gl3 = pile{{"chrome yellow",1},{"lead white",1.5},{"green earth",2.5},{"yellow ochre",1.5},{"raw umber",0.5}}
lB4 = fB3:lit{parts={1}, soft=0.5}
lE4 = fE3:lit{parts={1}, soft=0.5}
print(work(m_apB * lB4:band(0.66, 1.0, 0.24), {hand="body", pile=p_gl3, coverage=1.8, tool="filbert 10", angle=-0.7, curve={0.3,0.18}, clip=true, edge="soft", seed=481}))
print(work(m_apE * lE4:band(0.66, 1.0, 0.24), {hand="body", pile=p_gl3, coverage=1.8, tool="filbert 9", angle=-0.7, curve={0.3,0.18}, clip=true, edge="soft", seed=482}))
bowlL = m_bowl * mask(function(x, y) return smoothstep(640, 470, x) end)
bowlD = m_bowl * mask(function(x, y) return smoothstep(600, 760, x) end)
print(work(bowlL, {hand="body", pile=p_clothmid, coverage=1.5, tool="filbert 12", angle=0.2, clip=true, edge="lost", seed=483}))
print(work(bowlD, {hand="body", pile=p_bowls, coverage=1.7, tool="filbert 12", angle=0.2, clip=true, edge="lost", seed=484}))

--@ chunk 67

function apple_full(mm, ff, pl, ps, plt, sd)
  local l = ff:lit{parts={1}, soft=0.55}
  work(mm, {hand="body", pile=pl, coverage=3, tool="filbert 14", clip=true, edge="found", seed=sd})
  work(mm * (-l), {hand="body", pile=ps, coverage=2, tool="filbert 12", angle=0.45, curve={0.2,0.12}, clip=true, edge="soft", seed=sd+1})
  work(mm * l:band(0.66, 1.0, 0.26), {hand="body", pile=plt, coverage=1.7, tool="filbert 12", angle=-0.7, curve={0.25,0.15}, clip=true, edge="soft", seed=sd+2})
end
apple_full(m_apB, fB3, p_gb2, p_gs2, p_gl3, 491)
apple_full(m_apA, fA3, p_rb2, p_rs2, p_rl2, 495)
apple_full(m_apC, fC3, p_rb2, p_rs2, p_rl2, 499)
apple_full(m_apD, fD2, p_rb2, p_rs2, p_rl2, 503)
apple_full(m_apE, fE3, p_gb2, p_gs2, p_gl3, 507)

--@ chunk 68

print(drying(300,690), drying(600,520))
print(blend(m_apB, {angle=-0.6}))
print(blend(m_apA, {angle=-0.6}))
print(blend(m_apC, {angle=-0.6}))
print(blend(m_apD, {angle=-0.6}))
print(blend(m_apE, {angle=-0.6}))

--@ chunk 69

function deep_shadow(mm, ff, ps, sd)
  local l = ff:lit{parts={1}, soft=0.55}
  work(mm * (-l), {hand="body", pile=ps, coverage=1.3, tool="filbert 12", angle=0.45, curve={0.2,0.1}, clip=true, edge="lost", seed=sd})
  work(mm * l:band(0.22, 0.5, 0.2), {hand="body", pile=ps, coverage=0.6, tool="filbert 7", angle=0.45, clip=true, edge="lost", seed=sd+1})
end
deep_shadow(m_apB, fB3, p_gs2, 511)
deep_shadow(m_apA, fA3, p_rs2, 515)
deep_shadow(m_apC, fC3, p_rs2, 519)
deep_shadow(m_apD, fD2, p_rs2, 523)
deep_shadow(m_apE, fE3, p_gs2, 527)
hb = brush{kind="round", width=6, point=1, stiffness=0.3}
hb:load(p_hi, 1.0)
for _, p in ipairs({{478, 480}, {612, 437}, {664, 488}, {262, 652}, {450, 707}}) do
  hb:touch(p[1], p[2], {pressure=0.9})
end
hb:wipe(0.6)
for _, p in ipairs({{484, 488}, {270, 660}, {456, 714}}) do
  hb:touch(p[1], p[2], {pressure=0.35})
end

--@ chunk 70

b1 = brush("filbert", 20)
b1:load(p_rs2, 0.95)
b1:stroke({{318, 622}, {346, 664}, {352, 708}, {336, 748}}, {pressure={0.85, 0.9, 0.6}, ramps={0.06, 0.45}, shake=1})
b1:stroke({{356, 660}, {372, 700}, {362, 736}}, {pressure={0.7, 0.5}, ramps={0.06, 0.5}, shake=1})
b1:stroke({{306, 636}, {318, 690}, {312, 742}}, {pressure={0.45, 0.3}, ramps={0.08, 0.5}, shake=1})
b2 = brush("filbert", 10)
b2:load(p_rs2, 0.9)
b2:stroke({{286, 632}, {300, 676}, {300, 722}}, {pressure={0.55, 0.4}, ramps={0.1, 0.5}, shake=1})
print(blend(m_apD, {angle=-0.5}))

--@ chunk 71

print(work(m_bowl, {hand="body", pile=p_bowl2, coverage=3, tool="filbert 16", angle=0.2, clip=true, edge="found", seed=531}))
print(work(m_bowl * mask(function(x, y) return smoothstep(660, 470, x) end), {hand="body", pile=p_clothmid, coverage=1.4, tool="filbert 14", angle=0.15, clip=true, edge="lost", seed=532}))
print(work(m_bowl * mask(function(x, y) return smoothstep(600, 780, x) end), {hand="body", pile=p_bowls, coverage=1.9, tool="filbert 14", angle=0.15, clip=true, edge="lost", seed=533}))
print(work(m_bowl_mouth:rim(9, 4) * m_bowl, {hand="body", pile=p_white, coverage=1.5, tool="filbert 5", angle=0.05, clip=true, edge="lost", seed=534}))
print(work(m_bowl:rim(11, 5) * below(function(x) return 628 end), {hand="body", pile=p_csh3, coverage=1.4, tool="filbert 8", angle=0.1, clip=true, edge="lost", seed=535}))
print(blend(m_bowl, {angle=0.1}))

--@ chunk 72

print(work(m_bowl * mask(function(x, y) return smoothstep(560, 720, x) end), {hand="body", pile=p_bowls, coverage=2.2, tool="filbert 16", angle=0.15, clip=true, edge="lost", seed=541}))
print(work(m_bowl_f * mask(function(x, y) return smoothstep(520, 640, y) end), {hand="body", pile=p_csh3, coverage=1.3, tool="filbert 10", angle=0.05, clip=true, edge="lost", seed=542}))
print(wait(360), drying(300,690), drying(600,560), drying(500,700), drying(300,200))

--@ chunk 73
print(wait(600), drying(300,690), drying(600,560), drying(500,700), drying(300,200))

--@ chunk 74

bs = brush("filbert", 17)
bs:load(p_rs2, 0.85)
bs:stroke({{548, 446}, {574, 516}, {550, 586}}, {pressure={0.55, 0.7, 0.45}, ramps={0.12, 0.45}, shake=1.2})
bs:stroke({{726, 454}, {754, 520}, {730, 586}}, {pressure={0.55, 0.7, 0.45}, ramps={0.12, 0.45}, shake=1.2})
bs:stroke({{332, 614}, {360, 690}, {336, 766}}, {pressure={0.55, 0.72, 0.45}, ramps={0.12, 0.45}, shake=1.2})
bs:reload(p_rs2, 0.8)
bs:stroke({{506, 680}, {528, 736}, {510, 790}}, {pressure={0.5, 0.62, 0.4}, ramps={0.12, 0.45}, shake=1.2})
bs:reload(p_gs2, 0.8)
bs:stroke({{636, 422}, {662, 470}, {640, 520}}, {pressure={0.5, 0.62, 0.4}, ramps={0.12, 0.45}, shake=1.2})
hb = brush{kind="round", width=5, point=1, stiffness=0.3}
hb:load(p_hi, 1.0)
for _, p in ipairs({{478, 480}, {612, 437}, {664, 488}, {262, 652}, {450, 707}}) do
  hb:touch(p[1], p[2], {pressure=0.85})
end
hb:wipe(0.55)
for _, p in ipairs({{486, 490}, {270, 660}, {458, 715}, {672, 496}, {620, 445}}) do
  hb:touch(p[1], p[2], {pressure=0.32})
end

--@ chunk 75
print(drying(300,200), drying(900,300), drying(500,470)) print(work(m_bg, {hand="body", pile=p_csh3, coverage=1.0, tool="filbert 26", angle=0.1, clip=true, edge="lost", pressure={0.35,0.6}, seed=551}))

--@ chunk 76
print(work(m_bg, {hand="body", pile=p_bowls, coverage=2.6, tool="filbert 16", angle=0.05, curve={0.2,0.1}, clip=true, edge="found", seed=561})) print(work(m_bg * pool, {hand="body", pile=p_csh3, coverage=1.4, tool="filbert 24", angle=0, curve={0.2,0.1}, load_at=function(x, y) return 0.12 + 0.55*pool:at(x, y) end, clip=true, edge="lost", seed=562}))

--@ chunk 77
print(work(m_bg, {hand="body", pile=p_rs2, coverage=3, tool="filbert 16", angle=0.05, curve={0.2,0.1}, clip=true, edge="found", seed=571})) print(work(m_bg * pool, {hand="body", pile=p_bowls, coverage=1.0, tool="filbert 24", angle=0, curve={0.2,0.1}, load_at=function(x, y) return 0.1 + 0.5*pool:at(x, y) end, clip=true, edge="lost", seed=572}))

--@ chunk 78
print(wait(2880), drying(300,690), drying(600,560), drying(500,700), drying(300,200), drying(150,300))

--@ chunk 79
print(wait(2880), drying(300,690), drying(600,560), drying(500,700), drying(300,200), drying(150,300))

--@ chunk 80

behind = mask(function(x, y) local dx=(x-640)/330; local dy=(y-430)/200; local d=math.sqrt(dx*dx+dy*dy); return clamp(1.1-d,0,1) end)
print(work(m_bg * behind, {hand="body", pile=p_rs2, coverage=1.6, tool="filbert 24", angle=0.05, clip=true, edge="lost", seed=581}))
print(wait(4320), drying(300,690), drying(600,560), drying(500,700), drying(300,200))

--@ chunk 81

print(work(m_bg * behind, {hand="body", pile=p_bowls, coverage=2.8, tool="filbert 18", angle=0.05, clip=true, edge="lost", seed=591}))
print(work(m_bg * behind, {hand="body", pile=p_gs2, coverage=0.9, tool="filbert 24", angle=0.05, clip=true, edge="lost", load_at=function(x, y) return 0.1 + 0.5*behind:at(x, y) end, seed=592}))

--@ chunk 82

print(work(m_bg, {hand="body", pile=p_rs2, coverage=2.8, tool="filbert 16", angle=0.05, curve={0.2,0.1}, clip=true, edge="found", seed=601}))
print(work(m_bg * pool, {hand="body", pile=p_bowls, coverage=1.0, tool="filbert 24", angle=0, curve={0.2,0.1}, load_at=function(x, y) return 0.1 + 0.5*pool:at(x, y) end, clip=true, edge="lost", seed=602}))

--@ chunk 83

glow2 = mask(function(x, y) local dx=(x-420)/300; local dy=(y-330)/230; local d=math.sqrt(dx*dx+dy*dy); return clamp(1.15-d,0,1) end)
print(work(m_bg * glow2, {hand="body", pile=p_bowls, coverage=1.3, tool="filbert 24", angle=0, curve={0.2,0.1}, load_at=function(x, y) return 0.08 + 0.5*glow2:at(x, y) end, clip=true, edge="lost", seed=611}))

--@ chunk 84

print(drying(300,690), drying(600,520))
print(work(m_apD, {hand="body", pile=p_rb2, coverage=2.5, tool="filbert 16", angle=0.1, clip=true, edge="found", seed=621}))
print(work(m_apD * (-lD3), {hand="body", pile=p_rs2, coverage=1.8, tool="filbert 14", angle=0.45, curve={0.2,0.1}, clip=true, edge="soft", seed=622}))
print(work(m_apD * lD3:band(0.7, 1.0, 0.24), {hand="body", pile=p_rl2, coverage=1.8, tool="filbert 14", angle=-0.7, curve={0.25,0.15}, clip=true, edge="soft", seed=623}))
print(blend(m_apD, {angle=-0.5}))

--@ chunk 85

print(work(refl_mask(m_apD, 300, 690, 92, 86), {hand="body", pile=p_refr, coverage=0.8, tool="filbert 9", angle=0.2, clip=true, edge="lost", seed=631}))
hot = ellipse(266, 648, 58, 46):soften(26) * m_apD
print(work(hot, {hand="body", pile=p_rl2, coverage=1.0, tool="filbert 12", angle=-0.6, clip=true, edge="lost", seed=632}))
dim = (ellipse(300, 612, 30, 14):soften(7)) * m_apD
print(work(dim, {hand="body", pile=p_rs2, coverage=1.2, tool="filbert 7", angle=0.1, clip=true, edge="lost", seed=633}))
print(blend(m_apD, {angle=-0.5}))

--@ chunk 86

function apple_final(mm, cx, cy, rx, ry, ff, pl, ps, plt, sd)
  local l = ff:lit{parts={1}, soft=0.55}
  work(mm, {hand="body", pile=pl, coverage=2.5, tool="filbert 16", angle=0.1, clip=true, edge="found", seed=sd})
  work(mm * (-l), {hand="body", pile=ps, coverage=1.8, tool="filbert 14", angle=0.45, curve={0.2,0.1}, clip=true, edge="soft", seed=sd+1})
  work(mm * l:band(0.7, 1.0, 0.24), {hand="body", pile=plt, coverage=1.8, tool="filbert 14", angle=-0.7, curve={0.25,0.15}, clip=true, edge="soft", seed=sd+2})
  local hot = ellipse(cx - 0.37*rx, cy - 0.48*ry, 0.63*rx, 0.57*ry):soften(26) * mm
  work(hot, {hand="body", pile=plt, coverage=0.9, tool="filbert 12", angle=-0.6, clip=true, edge="lost", seed=sd+3})
  local dim = (ellipse(cx, cy - 0.86*ry, 0.35*rx, 0.17*ry):soften(7)) * mm
  work(dim, {hand="body", pile=ps, coverage=0.9, tool="filbert 7", angle=0.1, clip=true, edge="lost", seed=sd+4})
  work(refl_mask(mm, cx, cy, rx, ry), {hand="body", pile=p_refr, coverage=0.55, tool="filbert 9", angle=0.2, clip=true, edge="lost", seed=sd+5})
  blend(mm, {angle=-0.5})
end
apple_final(m_apA, 512, 516, 86, 80, fA3, p_rb2, p_rs2, p_rl2, 641)
apple_final(m_apC, 696, 520, 80, 75, fC3, p_rb2, p_rs2, p_rl2, 651)
apple_final(m_apE, 478, 736, 72, 66, fE3, p_gb2, p_gs2, p_gl3, 661)
print(work(refl_mask(m_apD, 300, 690, 92, 86), {hand="body", pile=p_rb2, coverage=1.0, tool="filbert 10", angle=0.2, clip=true, edge="lost", seed=671}))

--@ chunk 87

for _, t in ipairs({{m_apA, 512, 516, 86, 80, p_rb2}, {m_apC, 696, 520, 80, 75, p_rb2}, {m_apD, 300, 690, 92, 86, p_rb2}, {m_apE, 478, 736, 72, 66, p_gb2}}) do
  local mm, cx, cy, rx, ry, pl = t[1], t[2], t[3], t[4], t[5], t[6]
  work(refl_mask(mm, cx, cy, rx, ry), {hand="body", pile=pl, coverage=1.2, tool="filbert 10", angle=0.2, clip=true, edge="lost", seed=681})
end
apple_final(m_apB, 606, 498, 92, 86, fB3, p_gb2, p_gs2, p_gl3, 691)

--@ chunk 88
print(wait(4320), drying(300,690), drying(606,498), drying(600,600), drying(500,700), drying(300,200))

--@ chunk 89
print(wait(5760), drying(300,690), drying(606,498), drying(600,600), drying(500,700), drying(300,200), drying(200,300))

--@ chunk 90

rimTop = m_bowl_mouth:rim(8, 4) * m_bowl
bowlLit = m_bowl * mask(function(x, y) return smoothstep(690, 500, x) end)
bowlDark = m_bowl * mask(function(x, y) return smoothstep(570, 700, x) end)
pt5 = brush{kind="filbert", width=5, point=0.6}
print(work(bowlLit, {hand="body", pile=p_clothmid, coverage=1.2, tool="filbert 16", angle=0.12, clip=true, edge="lost", pressure={0.5,0.8}, seed=701}))
print(work(rimTop, {hand="body", pile=p_white, coverage=1.5, tool=pt5, angle=0.03, clip=true, edge="lost", seed=702}))
print(work(bowlDark, {hand="body", pile=p_bowls, coverage=1.7, tool="filbert 16", angle=0.12, clip=true, edge="lost", pressure={0.6,0.9}, seed=703}))
print(work(m_bowl_f * mask(function(x, y) return smoothstep(590, 650, y) end), {hand="body", pile=p_bowls, coverage=1.2, tool="filbert 12", angle=0.05, clip=true, edge="lost", seed=704}))
print(work(m_bowl:rim(10, 5) * below(function(x) return 632 end), {hand="body", pile=p_rs2, coverage=1.3, tool=brush{kind="filbert", width=8, point=0.5}, angle=0.03, clip=true, edge="lost", seed=705}))

--@ chunk 91

print(work(m_bowl, {hand="body", pile=p_bowl2, coverage=2.8, tool="filbert 16", angle=0.12, clip=true, edge="found", seed=711}))
print(work(m_bowl * mask(function(x, y) return smoothstep(700, 500, x) end), {hand="body", pile=p_clothmid, coverage=1.3, tool="filbert 16", angle=0.12, clip=true, edge="lost", pressure={0.45,0.75}, seed=712}))
print(work(m_bowl * mask(function(x, y) return smoothstep(580, 700, x) end), {hand="body", pile=p_bowls, coverage=1.7, tool="filbert 16", angle=0.12, clip=true, edge="lost", pressure={0.5,0.85}, seed=713}))
print(work(m_bowl * mask(function(x, y) return smoothstep(600, 655, y) end), {hand="body", pile=p_bowls, coverage=1.3, tool="filbert 12", angle=0.05, clip=true, edge="lost", seed=714}))
print(work(m_bowl_mouth:rim(7, 4) * m_bowl_f * mask(function(x, y) return smoothstep(700, 540, x) end), {hand="body", pile=p_white, coverage=0.9, tool=pt5, angle=0.03, clip=true, edge="lost", pressure={0.6,0.6}, seed=715}))
print(work(m_bowl:rim(9, 4) * below(function(x) return 636 end), {hand="body", pile=p_rs2, coverage=1.1, tool=brush{kind="filbert", width=8, point=0.5}, angle=0.03, clip=true, edge="lost", seed=716}))
print(blend(m_bowl, {angle=0.08}))

--@ chunk 92

rimBand = m_bowl_mouth:rim(10, 5) * m_bowl
print(work(rimBand, {hand="body", pile=p_bowl2, coverage=1.8, tool="filbert 10", angle=0.05, clip=true, edge="lost", seed=721}))
print(work(rimBand * mask(function(x, y) return smoothstep(680, 520, x) end), {hand="body", pile=p_clothmid, coverage=0.9, tool=pt5, angle=0.03, clip=true, edge="lost", pressure={0.5,0.7}, seed=722}))

--@ chunk 93

print(rimBand:at(620, 472), m_bowl:at(620, 472), rimBand:area())
print(work(rimBand, {hand="body", pile=p_bowl2, coverage=2.6, tool="filbert 10", angle=0.05, clip=true, edge="found", seed=731}))

--@ chunk 94

apple_final(m_apB, 606, 498, 92, 86, fB3, p_gb2, p_gs2, p_gl3, 741)
apple_final(m_apA, 512, 516, 86, 80, fA3, p_rb2, p_rs2, p_rl2, 751)
apple_final(m_apC, 696, 520, 80, 75, fC3, p_rb2, p_rs2, p_rl2, 761)
lB5 = fB3:lit{parts={1}, soft=0.55}
print(work(m_apB * lB5:band(0.6, 1.0, 0.3), {hand="body", pile=p_bowls, coverage=0.8, tool="filbert 14", angle=-0.7, clip=true, edge="lost", seed=771}))
print(work(m_apB * lB5:band(0.6, 1.0, 0.3), {hand="body", pile=p_gb2, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, edge="lost", seed=772}))
print(blend(m_apB, {angle=-0.5}))

--@ chunk 95

mleft = m_clothonly * mask(function(x, y) return clamp((330 - x)/140, 0, 1) end)
mlow = m_clothonly * mask(function(x, y) return clamp((y - 700)/90, 0, 1) end)
warmb = m_clothonly * (ellipse(520, 640, 190, 90):soften(60) + ellipse(770, 690, 120, 70):soften(50) + ellipse(300, 740, 130, 70):soften(50))
print(work(mleft, {hand="body", pile=p_csh3, coverage=1.4, tool="filbert 18", angle=0.3, clip=true, edge="lost", seed=781}))
print(work(mlow, {hand="body", pile=p_csh3, coverage=1.2, tool="filbert 16", angle=0.1, clip=true, edge="lost", seed=782}))
print(work(warmb, {hand="body", pile=p_bowl2, coverage=0.55, tool="filbert 20", angle=0.3, clip=true, edge="lost", pressure={0.4,0.7}, seed=783}))

--@ chunk 96

apple_final(m_apD, 300, 690, 92, 86, fD2, p_rb2, p_rs2, p_rl2, 791)
apple_final(m_apE, 478, 736, 72, 66, fE3, p_gb2, p_gs2, p_gl3, 801)
print(work(m_clothonly, {hand="broad", pile=p_csh2, angle=0.3, coverage=2.2, length={80,160}, tool="filbert 18", load=0.55, clip=true, edge="found", seed=811}))

--@ chunk 97

print(work(m_clothonly, {hand="body", pile=p_csh2, coverage=3, tool="filbert 16", angle=0.3, clip=true, edge="found", seed=821}))
print(work(mleft, {hand="body", pile=p_csh3, coverage=1.5, tool="filbert 18", angle=0.3, clip=true, edge="lost", seed=822}))
print(work(mlow, {hand="body", pile=p_csh3, coverage=1.2, tool="filbert 16", angle=0.1, clip=true, edge="lost", seed=823}))
print(work(crest1, {hand="body", pile=p_clothmid, coverage=1.7, tool="filbert 16", angle=0.75, curve={0.2,0.12}, clip=true, edge="lost", seed=824}))

--@ chunk 98
ln1 = 5 print(ln1)

--@ chunk 99
a2 = outline({pts={{100,100},{200,200}}, char="firm", seed=3, closed=true}) print("B ok")

--@ chunk 100

brc = brush{kind="round", width=3.5, point=1, stiffness=0.4}
ln1 = outline({pts={{392, 636}, {356, 690}, {312, 748}}, char="searching", seed=3})
ln2 = outline({pts={{404, 700}, {420, 744}, {428, 788}}, char="broken", seed=5})
ln3 = outline({pts={{452, 618}, {372, 656}, {286, 686}}, char="searching", seed=7})
ln4 = outline({pts={{210, 726}, {350, 764}, {520, 778}, {660, 756}, {768, 716}}, char="searching", seed=11})
ln5 = outline({pts={{790, 636}, {822, 668}, {806, 706}}, char="broken", seed=13})
ln1:paint(brc, {pressure=0.55, dip={p_csh3, 0.5}, every=2})
ln2:paint(brc, {pressure=0.45, dip={p_csh3, 0.5}, every=2})
ln3:paint(brc, {pressure=0.6, dip={p_csh3, 0.55}, every=2})
ln4:paint(brc, {pressure=0.5, dip={p_csh3, 0.45}, every=3})
ln5:paint(brc, {pressure=0.4, dip={p_csh3, 0.45}, every=2})

--@ chunk 101

brc2 = brush{kind="filbert", width=9, point=0.35, stiffness=0.6}
ln3:paint(brc2, {pressure=0.85, dip={p_csh3, 0.75}, every=2})
ln4:paint(brc2, {pressure=0.7, dip={p_csh3, 0.7}, every=3})

--@ chunk 102

apple_final(m_apD, 300, 690, 92, 86, fD2, p_rb2, p_rs2, p_rl2, 821)
apple_final(m_apE, 478, 736, 72, 66, fE3, p_gb2, p_gs2, p_gl3, 831)
k1 = outline({pts={{462, 612}, {432, 638}, {402, 660}}, char="soft", seed=3})
k2 = outline({pts={{398, 694}, {410, 720}, {416, 744}}, char="soft", seed=5})
k3a = outline({pts={{190, 730}, {280, 764}, {370, 792}}, char="soft", seed=7})
k3b = outline({pts={{566, 792}, {668, 764}, {762, 726}}, char="soft", seed=11})
k4 = outline({pts={{796, 634}, {828, 668}, {812, 710}}, char="soft", seed=13})
mc = m_clothonly
k1:paint(brc2, {pressure=0.8, dip={p_csh3, 0.7}, every=2, clip=mc})
k2:paint(brc2, {pressure=0.7, dip={p_csh3, 0.7}, every=2, clip=mc})
k3a:paint(brc2, {pressure=0.7, dip={p_csh3, 0.65}, every=3, clip=mc})
k3b:paint(brc2, {pressure=0.65, dip={p_csh3, 0.65}, every=3, clip=mc})
k4:paint(brc2, {pressure=0.55, dip={p_csh3, 0.6}, every=2, clip=mc})

--@ chunk 103

print(work(m_clothonly, {hand="body", pile=p_csh2, coverage=2.6, tool="filbert 18", angle=0.3, clip=true, edge="found", seed=841}))
f1 = (ribbon({{452, 606}, {400, 640}, {352, 682}}, 66):soften(22)) * m_clothonly
f2 = (ribbon({{404, 690}, {412, 726}, {416, 762}}, 52):soften(20)) * m_clothonly
f3 = (ribbon({{180, 742}, {300, 786}, {420, 800}}, 60):soften(22)) * m_clothonly
f4 = (ribbon({{590, 800}, {700, 766}, {800, 716}}, 56):soften(20)) * m_clothonly
f5 = (ribbon({{790, 628}, {836, 668}, {816, 716}}, 46):soften(18)) * m_clothonly
print(work(f1 + f2, {hand="body", pile=p_csh3, coverage=1.3, tool="filbert 12", angle=0.8, clip=true, edge="lost", seed=842}))
print(work(f3 + f4 + f5, {hand="body", pile=p_csh3, coverage=1.2, tool="filbert 12", angle=0.5, clip=true, edge="lost", seed=843}))
g1 = (ribbon({{470, 620}, {418, 656}, {366, 696}}, 34):soften(16)) * m_clothonly
g2 = (ribbon({{160, 712}, {290, 756}, {420, 776}}, 30):soften(15)) * m_clothonly
print(work(g1 + g2, {hand="body", pile=p_clothmid, coverage=1.2, tool="filbert 11", angle=0.8, clip=true, edge="lost", seed=844}))

--@ chunk 104

mfar = m_clothonly * mask(function(x, y) return clamp((300 - x)/130, 0, 1) * 0.8 + clamp((y - 740)/70, 0, 1) * 0.6 end):blur(6)
print(work(mfar, {hand="body", pile=p_csh3, coverage=2, tool="filbert 18", angle=0.3, clip=true, edge="lost", seed=851}))

--@ chunk 105

print(work(mfar, {hand="body", pile=p_gs2, coverage=1.5, tool="filbert 20", angle=0.3, clip=true, edge="lost", pressure={0.5,0.85}, seed=861}))

--@ chunk 106

print(drying(300,690), drying(512,516), drying(606,498))
hb2 = brush{kind="round", width=7, point=1, stiffness=0.55}
hb2:load(p_hi, 1.0)
for _, p in ipairs({{478, 480}, {574, 462}, {664, 487}, {263, 653}, {448, 708}}) do
  hb2:touch(p[1], p[2], {pressure=0.95})
end
hb2:wipe(0.5)
hb2:reload(p_hi, 0.55)
for _, p in ipairs({{488, 492}, {584, 474}, {674, 498}, {273, 663}, {458, 718}}) do
  hb2:touch(p[1], p[2], {pressure=0.4})
end

--@ chunk 107

mrim = m_bowl_f * ellipse(640, 606, 110, 26):soften(12)
print(work(mrim, {hand="body", pile=p_bowl2, coverage=2.2, tool="filbert 10", angle=0.05, clip=true, edge="lost", seed=871}))
mfringe = m_apB * ellipse(606, 430, 60, 34):soften(16)
print(work(mfringe, {hand="body", pile=p_gb2, coverage=1.6, tool="filbert 12", angle=0.1, clip=true, edge="lost", seed=872}))

--@ chunk 108
print(wait(2880), drying(300,690), drying(512,516), drying(606,498), drying(600,600), drying(500,700))

--@ chunk 109

hb3 = brush{kind="round", width=7, point=1, stiffness=0.7}
hb3:load(p_hi, 1.0)
for _, p in ipairs({{478, 480}, {574, 462}, {664, 487}, {263, 653}, {448, 708}}) do
  hb3:touch(p[1], p[2], {pressure=1.0})
end

--@ chunk 110

hsoft = brush{kind="filbert", width=16, point=0.4, stiffness=0.5}
hsoft:load(p_clothmid, 0.7)
for _, p in ipairs({{482, 484}, {578, 466}, {668, 491}, {267, 657}, {452, 712}}) do
  hsoft:touch(p[1], p[2], {pressure=0.35})
end
hsoft:reload(p_hi, 0.4)
for _, p in ipairs({{484, 487}, {580, 469}, {670, 494}, {269, 660}, {454, 715}}) do
  hsoft:touch(p[1], p[2], {pressure=0.3})
end
sep = brush{kind="filbert", width=7, point=0.5, stiffness=0.7}
sep:load(p_rs2, 0.8)
sep:stroke({{572, 452}, {578, 500}, {574, 548}}, {pressure={0.6,0.75,0.5}, ramps={0.1,0.4}, shake=1.2})
sep:reload(p_rs2, 0.75)
sep:stroke({{670, 452}, {676, 500}, {668, 550}}, {pressure={0.6,0.75,0.5}, ramps={0.1,0.4}, shake=1.2})

--@ chunk 111

msep = (ribbon({{572, 450}, {576, 500}, {572, 550}}, 20):soften(9)) + (ribbon({{670, 450}, {674, 500}, {666, 550}}, 20):soften(9))
msepAB = msep * mask(function(x, y) return smoothstep(590, 566, x) end)
msepB = msep * mask(function(x, y) return smoothstep(564, 588, x) * smoothstep(654, 674, x) end)
msepC = msep * mask(function(x, y) return smoothstep(652, 674, x) end)
msepA = msep * mask(function(x, y) return smoothstep(652, 674, x) * smoothstep(590, 566, x) end)
print(work(msep, {hand="body", pile=p_rs2, coverage=1.5, tool="filbert 10", angle=0.2, clip=true, edge="lost", seed=901}))
print(work(msepAB + msepC, {hand="body", pile=p_rb2, coverage=0.7, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=902}))
print(work(msepB, {hand="body", pile=p_gb2, coverage=0.7, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=903}))

--@ chunk 112

apple_final(m_apB, 606, 498, 92, 86, fB3, p_gb2, p_gs2, p_gl3, 911)
apple_final(m_apA, 512, 516, 86, 80, fA3, p_rb2, p_rs2, p_rl2, 921)
apple_final(m_apC, 696, 520, 80, 75, fC3, p_rb2, p_rs2, p_rl2, 931)
lB6 = fB3:lit{parts={1}, soft=0.55}
print(work(m_apB * lB6:band(0.6, 1.0, 0.3), {hand="body", pile=p_bowls, coverage=0.8, tool="filbert 14", angle=-0.7, clip=true, edge="lost", seed=941}))
print(work(m_apB * lB6:band(0.6, 1.0, 0.3), {hand="body", pile=p_gb2, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, edge="lost", seed=942}))
print(blend(m_apB, {angle=-0.5}))

--@ chunk 113

mg = m_apA + m_apB + m_apC
crv = (ellipse(578, 498, 15, 62):soften(15)) + (ellipse(670, 502, 14, 58):soften(14))
print(work(crv * mg, {hand="body", pile=p_rs2, coverage=1.1, tool="filbert 18", angle=0.2, clip=true, edge="lost", pressure={0.5,0.8}, seed=951}))

--@ chunk 114

apple_final(m_apB, 606, 498, 92, 86, fB3, p_gb2, p_gs2, p_gl3, 961)
apple_final(m_apA, 512, 516, 86, 80, fA3, p_rb2, p_rs2, p_rl2, 971)
apple_final(m_apC, 696, 520, 80, 75, fC3, p_rb2, p_rs2, p_rl2, 981)
lB7 = fB3:lit{parts={1}, soft=0.55}
print(work(m_apB * lB7:band(0.6, 1.0, 0.3), {hand="body", pile=p_bowls, coverage=0.8, tool="filbert 14", angle=-0.7, clip=true, edge="lost", seed=991}))
print(work(m_apB * lB7:band(0.6, 1.0, 0.3), {hand="body", pile=p_gb2, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, edge="lost", seed=992}))
print(blend(m_apB, {angle=-0.5}))
print(wait(2880), drying(512,516), drying(600,600), drying(300,690))

--@ chunk 115

hb4 = brush{kind="round", width=7, point=1, stiffness=0.7}
hb4:load(p_hi, 1.0)
for _, p in ipairs({{478, 480}, {574, 462}, {664, 487}, {263, 653}, {448, 708}}) do
  hb4:touch(p[1], p[2], {pressure=1.0})
end
stem = brush{kind="rigger", width=4, point=1, stiffness=0.75}
stem:load(p_bowls, 0.95)
stem:stroke({{514, 442}, {528, 424}, {540, 414}}, {pressure={0.9, 0.75, 0.4}, ramps={0.05, 0.5}, shake=0.8})
stem:stroke({{608, 418}, {622, 402}, {634, 394}}, {pressure={0.9, 0.75, 0.4}, ramps={0.05, 0.5}, shake=0.8})
stem:stroke({{698, 450}, {712, 434}, {724, 426}}, {pressure={0.85, 0.7, 0.35}, ramps={0.05, 0.5}, shake=0.8})
stem:stroke({{302, 610}, {320, 590}, {336, 578}}, {pressure={0.95, 0.8, 0.45}, ramps={0.05, 0.5}, shake=0.8})
stem:stroke({{480, 674}, {494, 660}, {506, 652}}, {pressure={0.85, 0.7, 0.35}, ramps={0.05, 0.5}, shake=0.8})

--@ chunk 116

stem2 = brush{kind="rigger", width=3.4, point=1, stiffness=0.8}
stem2:load(p_clothmid, 1.0)
stem2:stroke({{510, 442}, {524, 422}, {538, 410}}, {pressure={0.9, 0.7, 0.35}, ramps={0.05, 0.5}, shake=0.6})
stem2:stroke({{604, 418}, {618, 400}, {632, 390}}, {pressure={0.9, 0.7, 0.35}, ramps={0.05, 0.5}, shake=0.6})
stem2:stroke({{694, 450}, {708, 432}, {722, 422}}, {pressure={0.85, 0.65, 0.3}, ramps={0.05, 0.5}, shake=0.6})
hsoft2 = brush{kind="filbert", width=18, point=0.3, stiffness=0.5}
hsoft2:load(p_clothmid, 0.55)
for _, p in ipairs({{482, 484}, {578, 466}, {668, 491}, {267, 657}, {452, 712}}) do
  hsoft2:touch(p[1], p[2], {pressure=0.3})
end

--@ chunk 117

stem3 = brush{kind="rigger", width=3.2, point=1, stiffness=0.8}
stem3:load(p_rs2, 0.95)
stem3:stroke({{514, 444}, {528, 424}, {541, 412}}, {pressure={0.95, 0.8, 0.4}, ramps={0.05, 0.5}, shake=0.6})
stem3:stroke({{608, 420}, {622, 402}, {635, 392}}, {pressure={0.95, 0.8, 0.4}, ramps={0.05, 0.5}, shake=0.6})
stem3:stroke({{698, 452}, {712, 434}, {725, 424}}, {pressure={0.9, 0.75, 0.35}, ramps={0.05, 0.5}, shake=0.6})

--@ chunk 118

sd = brush{kind="rigger", width=6, point=1, stiffness=0.8}
sd:load(p_rs2, 1.0)
sd:stroke({{514, 444}, {528, 424}, {541, 412}}, {pressure={1.0, 0.9, 0.5}, ramps={0.04, 0.5}, shake=0.5})
sd:stroke({{608, 420}, {622, 402}, {635, 392}}, {pressure={1.0, 0.9, 0.5}, ramps={0.04, 0.5}, shake=0.5})
sd:stroke({{698, 452}, {712, 434}, {725, 424}}, {pressure={0.95, 0.85, 0.45}, ramps={0.04, 0.5}, shake=0.5})
sd:reload(p_bowls, 0.9)
sd:stroke({{302, 610}, {320, 590}, {336, 578}}, {pressure={1.0, 0.9, 0.5}, ramps={0.04, 0.5}, shake=0.5})
sd:stroke({{480, 674}, {494, 660}, {506, 652}}, {pressure={0.9, 0.8, 0.4}, ramps={0.04, 0.5}, shake=0.5})

--@ chunk 119

rim = brush{kind="filbert", width=6, point=0.4, stiffness=0.8}
rim:load(p_clothmid, 0.95)
rim:stroke({{452, 550}, {502, 572}, {562, 582}, {622, 583}, {682, 575}, {730, 557}}, {pressure={0.45, 0.85, 0.95, 0.9, 0.7, 0.4}, ramps={0.08, 0.3}, shake=0.5})
rim:wipe(0.5)
rim:reload(p_hi, 0.7)
rim:stroke({{462, 556}, {512, 576}, {572, 584}, {632, 585}}, {pressure={0.35, 0.7, 0.8, 0.5}, ramps={0.1, 0.35}, shake=0.4})

--@ chunk 120

print(work(m_clothonly, {hand="body", pile=p_bowls, coverage=0.75, tool="filbert 22", angle=0.25, clip=true, edge="lost", pressure={0.35,0.6}, seed=1001}))

--@ chunk 121

print(work(m_clothonly, {hand="body", pile=p_csh2, coverage=2.8, tool="filbert 16", angle=0.3, clip=true, edge="found", seed=1011}))
print(work(mfar, {hand="body", pile=p_gs2, coverage=1.5, tool="filbert 20", angle=0.3, clip=true, edge="lost", pressure={0.5,0.85}, seed=1012}))
print(work(crest1, {hand="body", pile=p_clothmid, coverage=1.7, tool="filbert 16", angle=0.75, curve={0.2,0.12}, clip=true, edge="lost", seed=1013}))
print(work(f1 + f2, {hand="body", pile=p_csh3, coverage=1.3, tool="filbert 12", angle=0.8, clip=true, edge="lost", seed=1014}))
print(work(f3 + f4 + f5, {hand="body", pile=p_csh3, coverage=1.2, tool="filbert 12", angle=0.5, clip=true, edge="lost", seed=1015}))

--@ chunk 122

print(work(mfar, {hand="body", pile=p_csh2, coverage=1.8, tool="filbert 20", angle=0.3, clip=true, edge="lost", pressure={0.5,0.85}, seed=1021}))
print(work(mfar, {hand="body", pile=p_csh3, coverage=1.0, tool="filbert 18", angle=0.3, clip=true, edge="soft", seed=1022}))
apple_final(m_apD, 300, 690, 92, 86, fD2, p_rb2, p_rs2, p_rl2, 1023)
apple_final(m_apE, 478, 736, 72, 66, fE3, p_gb2, p_gs2, p_gl3, 1031)

--@ chunk 123
print(wait(2880), drying(300,690), drying(478,736), drying(512,516), drying(500,700), drying(600,600))

--@ chunk 124

hb5 = brush{kind="round", width=7, point=1, stiffness=0.75}
hb5:load(p_hi, 1.0)
for _, p in ipairs({{478, 480}, {574, 462}, {664, 487}, {263, 653}, {448, 708}}) do
  hb5:touch(p[1], p[2], {pressure=1.0})
end

--@ chunk 125

acc = brush{kind="filbert", width=11, point=0.3, stiffness=0.8}
acc:load(p_white, 0.7)
for _, p in ipairs({{252, 698}, {330, 738}, {424, 770}, {562, 760}, {700, 718}, {300, 622}}) do
  acc:touch(p[1], p[2], {pressure=0.3})
end
acc:reload(p_rs2, 0.75)
for _, p in ipairs({{302, 758}, {486, 784}, {660, 744}, {190, 668}}) do
  acc:touch(p[1], p[2], {pressure=0.32})
end

--@ chunk 126

acc2 = brush{kind="filbert", width=9, point=0.5, stiffness=0.85}
acc2:load(p_white, 1.0)
for _, p in ipairs({{250, 696}, {332, 740}, {426, 772}, {566, 758}, {698, 716}, {296, 620}}) do
  acc2:touch(p[1], p[2], {pressure=0.85})
end
acc2:reload(p_rs2, 1.0)
for _, p in ipairs({{306, 760}, {490, 786}, {662, 746}, {188, 670}}) do
  acc2:touch(p[1], p[2], {pressure=0.7})
end

--@ chunk 127

apple_final(m_apD, 300, 690, 92, 86, fD2, p_rb2, p_rs2, p_rl2, 1041)
apple_final(m_apE, 478, 736, 72, 66, fE3, p_gb2, p_gs2, p_gl3, 1051)
fix = brush{kind="filbert", width=13, point=0.3, stiffness=0.85}
fix:load(p_csh2, 1.0)
for _, p in ipairs({{250, 696}, {332, 740}, {426, 772}, {566, 758}, {698, 716}, {296, 620}}) do
  fix:touch(p[1], p[2], {pressure=0.9})
end
fix:reload(p_csh2, 0.9)
for _, p in ipairs({{306, 760}, {490, 786}, {662, 746}, {188, 670}}) do
  fix:touch(p[1], p[2], {pressure=0.9})
end

--@ chunk 128
print(wait(2880), drying(300,690), drying(478,736))

--@ chunk 129

hb6 = brush{kind="round", width=7, point=1, stiffness=0.75}
hb6:load(p_hi, 1.0)
hb6:touch(263, 653, {pressure=1.0})
hb6:touch(448, 708, {pressure=1.0})

--@ chunk 130

st = brush{kind="rigger", width=5, point=1, stiffness=0.85}
st:load(p_bowls, 1.0)
st:stroke({{510, 444}, {526, 424}, {541, 412}}, {pressure={1.0, 0.9, 0.45}, ramps={0.04, 0.45}, shake=0.4})
st:stroke({{604, 420}, {620, 402}, {635, 390}}, {pressure={1.0, 0.9, 0.45}, ramps={0.04, 0.45}, shake=0.4})
st:stroke({{694, 452}, {710, 434}, {725, 422}}, {pressure={0.95, 0.85, 0.4}, ramps={0.04, 0.45}, shake=0.4})
st:reload(p_rs2, 0.8)
st:stroke({{514, 440}, {532, 420}, {544, 412}}, {pressure={0.6, 0.5, 0.25}, ramps={0.08, 0.5}, shake=0.4})
st:stroke({{608, 416}, {626, 398}, {638, 390}}, {pressure={0.6, 0.5, 0.25}, ramps={0.08, 0.5}, shake=0.4})
st:stroke({{698, 448}, {714, 430}, {727, 422}}, {pressure={0.55, 0.45, 0.2}, ramps={0.08, 0.5}, shake=0.4})

--@ chunk 131

mtop = ellipse(606, 438, 78, 44):soften(18) * m_apB
print(work(mtop, {hand="body", pile=p_gb2, coverage=1.5, tool="filbert 10", angle=0.2, clip=true, edge="soft", seed=1101}))
print(work(mtop, {hand="body", pile=p_bowls, coverage=0.6, tool="filbert 12", angle=0.2, clip=true, edge="lost", seed=1102}))

--@ chunk 132

apple_final(m_apB, 606, 498, 92, 86, fB3, p_gb2, p_gs2, p_gl3, 1111)
lB8 = fB3:lit{parts={1}, soft=0.55}
print(work(m_apB * lB8:band(0.6, 1.0, 0.3), {hand="body", pile=p_bowls, coverage=0.8, tool="filbert 14", angle=-0.7, clip=true, edge="lost", seed=1121}))
print(work(m_apB * lB8:band(0.6, 1.0, 0.3), {hand="body", pile=p_gb2, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, edge="lost", seed=1122}))
print(blend(m_apB, {angle=-0.5}))

--@ chunk 133
print(wait(2880), drying(606,498))

--@ chunk 134

hb7 = brush{kind="round", width=7, point=1, stiffness=0.75}
hb7:load(p_hi, 1.0)
hb7:touch(574, 462, {pressure=1.0})
st2 = brush{kind="rigger", width=5, point=1, stiffness=0.85}
st2:load(p_bowls, 1.0)
st2:stroke({{604, 420}, {620, 402}, {635, 390}}, {pressure={1.0, 0.9, 0.45}, ramps={0.04, 0.45}, shake=0.4})
st2:reload(p_rs2, 0.8)
st2:stroke({{608, 416}, {626, 398}, {638, 390}}, {pressure={0.6, 0.5, 0.25}, ramps={0.08, 0.5}, shake=0.4})

--@ chunk 135
for _,n in ipairs{"m_apA","m_apB","m_apC","m_apD","m_apE","m_bowl","m_cloth","m_wall","m_table"} do local v=_G[n]; print(n, v and math.floor(v:area()) or "nil") end
print("forms:", fA3, fB3, fC3, fD2, fE3)
print("tubes:", table.concat(tubes(), ", "))

--@ chunk 136
nb2 = noise{seed=21, period=260}
print(type(nb2), type(nb2(100,100)))
print(nb2:at01(100,100))

--@ chunk 137
nb2 = noise{seed=21, period=260, octaves=4, persistence=0.5}
m_group = (m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl):grow(5)
m_bg = m_wall - m_group - m_cloth
p_bgv = pile{{"raw umber",5},{"bone black",3},{"red earth",1.5},{"Prussian blue",0.6}}
print(work(m_bg, {hand="broad", pile=p_bgv, tool="filbert 22", angle=0.22, coverage=2.0, curve={0.25,0.3}, clip=true, edge="lost", seed=610}))

--@ chunk 138
p_bgl = pile{{"raw umber",4},{"red earth",2},{"yellow ochre",1.4},{"lead white",0.5}}
m_group = (m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl):grow(5)
m_bg = m_wall - m_group - m_cloth
p1 = ellipse(300,180, 470,370):soften(130)
p2 = ellipse(272,166, 330,260):soften(95)
p3 = ellipse(242,152, 205,165):soften(75)
for i, pm in ipairs({p1,p2,p3}) do
  print(work(pm * m_bg, {hand="glaze", pile=p_bgl, coverage=0.75, clip=true, seed=620+i}))
end

--@ chunk 139
p_bgv = pile{{"raw umber",5},{"bone black",3},{"red earth",1.5},{"Prussian blue",0.6}}
m_group = (m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl):grow(5)
m_bg = m_wall - m_group - m_cloth
print(work(m_bg, {hand="broad", pile=p_bgv, tool="filbert 22", angle=0.22, coverage=2.4, curve={0.25,0.3}, clip=true, edge="lost", seed=630}))

--@ chunk 140
m_group = (m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl):grow(5)
m_bg = m_wall - m_group - m_cloth
print(blend(m_bg, {angle=0.15}))

--@ chunk 141
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
m_group = (m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl):grow(5)
m_bg = m_wall - m_group - m_cloth
print(work(m_bg, {hand="broad", pile=p_bgd, tool="filbert 22", angle=0.22, coverage=1.3, curve={0.25,0.3}, clip=true, edge="lost", seed=640}))
p_bgl = pile{{"raw umber",4},{"red earth",2.5},{"yellow ochre",1.6},{"lead white",1}}
lp = ellipse(160,90, 230,180):soften(85)
print(work(lp * m_bg, {hand="broad", pile=p_bgl, tool="filbert 20", angle=0.3, coverage=0.85, clip=true, edge="lost", seed=641}))

--@ chunk 142
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
m_group = (m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl):grow(5)
m_bg = m_wall - m_group - m_cloth
print(blend(m_bg, {angle=0.15}))
print(work(m_bg, {hand="scumble", pile=p_bgd, tool="filbert 9", coverage=1.1, clip=true, edge="lost", seed=650}))
print(blend(m_bg, {angle=0.6}))

--@ chunk 143
g_mid  = pile{{"green earth",4},{"yellow ochre",2},{"raw umber",1.2}}
g_turn = pile{{"green earth",3},{"yellow ochre",1.5},{"raw umber",2.2}}
g_dark = pile{{"raw umber",3},{"green earth",1.5},{"bone black",1.4},{"Prussian blue",0.4}}
g_lt   = pile{{"chrome yellow",2},{"lead white",2.5},{"yellow ochre",2},{"green earth",1}}
g_refr = pile{{"lead white",2.5},{"green earth",1.2},{"raw umber",0.8}}
mmB = m_apB:grow(2.5)
lB = fB3:lit{parts={1}, soft=0.42}
print(work(mmB, {hand="body", pile=g_mid, coverage=3.2, tool="filbert 12", clip=true, edge="soft", seed=700}))
print(work(mmB * lB:band(0.28,0.62,0.22), {hand="body", pile=g_turn, coverage=2.2, tool="filbert 11", angle=2.2, curve={0.2,0.12}, clip=true, edge="soft", seed=701}))
print(work((mmB * (-lB)):band(0,0.45,0.15), {hand="body", pile=g_dark, coverage=2.4, tool="filbert 11", angle=0.5, curve={0.2,0.1}, clip=true, edge="soft", seed=702}))

--@ chunk 144
print(type(p_lt), p_lt)
print(type(g_mid), g_mid)

--@ chunk 145
for _,n in ipairs{"p_gl3","p_bowl2","p_lt2","p_gb2","p_gs2","p_rb2","p_rs2","p_rl2","p_clothmid","p_csh2","p_csh3","p_bowls","p_refr2","p_hi","p_white","p_dark","p_tab","p_rs2"} do print(n, _G[n] and "ok" or "MISSING") end

--@ chunk 146
p_lt = pile{{"lead white",6},{"yellow ochre",1}}
row1 = {p_lt, p_gl3, p_gb2, p_rl2, p_rb2, p_bowl2}
row2 = {p_clothmid, p_csh2, p_bowls, p_refr2, p_white, p_hi}
for i,pl in ipairs(row1) do work(rect(700+45*(i-1), 15, 40,40), {hand="body", pile=pl, coverage=3, tool="filbert 16", clip=true, seed=810+i}) end
for i,pl in ipairs(row2) do work(rect(700+45*(i-1), 60, 40,40), {hand="body", pile=pl, coverage=3, tool="filbert 16", clip=true, seed=830+i}) end

--@ chunk 147
cx, cy, rx, ry = 606, 498, 92, 86
g_lite  = pile{{"chrome yellow",1.2},{"lead white",1.5},{"yellow ochre",2.5},{"green earth",1.5}}
g_lite2 = pile{{"chrome yellow",1},{"lead white",2.5},{"yellow ochre",1.5},{"green earth",0.8}}
mfB = m_apB:grow(2)
dB = function(x, y)
  local dx = (x - cx) / rx
  local dy = (y - cy) / ry
  return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dB(x, y)) end) end
m_shB  = mfB * zt(-0.16, 0.34)
m_sh2B = mfB * zt(0.12, 0.46)
m_litB = mfB * zt(0.34, -0.22)
m_lit2B = mfB * zt(0.08, -0.28)
m_rfB = mfB * mask(function(x, y)
  local dy = (y - cy) / ry
  return smoothstep(0.46, 0.92, dB(x, y)) * smoothstep(0.15, 0.68, dy)
end)
print(work(mfB, {hand="body", pile=p_gb2, coverage=2.8, tool="filbert 14", clip=true, edge="soft", seed=850}))
print(work(m_shB, {hand="body", pile=p_gs2, coverage=1.0, tool="filbert 12", angle=0.5, clip=true, edge="lost", seed=851}))
print(work(m_sh2B, {hand="body", pile=p_gs2, coverage=0.85, tool="filbert 10", angle=0.5, clip=true, edge="lost", seed=852}))
print(work(m_litB, {hand="body", pile=g_lite, coverage=0.95, tool="filbert 12", angle=-0.7, clip=true, edge="lost", seed=853}))
print(work(m_lit2B, {hand="body", pile=g_lite2, coverage=0.8, tool="filbert 9", angle=-0.7, clip=true, edge="lost", seed=854}))
print(work(m_rfB, {hand="body", pile=p_refr2, coverage=0.6, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=855}))
print(blend(mfB, {angle=-0.6}))

--@ chunk 148
cx, cy, rx, ry = 606, 498, 92, 86
g_mid  = pile{{"green earth",4},{"yellow ochre",2.5},{"chrome yellow",0.6},{"lead white",0.6}}
g_lite = pile{{"chrome yellow",0.6},{"lead white",2},{"yellow ochre",3},{"green earth",1.2}}
g_hot  = pile{{"lead white",3},{"yellow ochre",2},{"chrome yellow",0.8},{"green earth",0.5}}
mfB = m_apB:grow(1.5)
dB = function(x, y)
  local dx = (x - cx) / rx
  local dy = (y - cy) / ry
  return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dB(x, y)) end) end
m_shB  = mfB * zt(0.30, 0.62)
m_sh2B = mfB * zt(0.62, 0.95)
m_litB = mfB * zt(0.05, -0.18)
m_lit2B = mfB * zt(-0.25, -0.55)
m_rfB = mfB * mask(function(x, y)
  local dy = (y - cy) / ry
  return smoothstep(0.75, 1.15, dB(x, y)) * smoothstep(0.35, 0.80, dy)
end)
print(work(mfB, {hand="body", pile=g_mid, coverage=3.0, tool="filbert 14", clip=true, edge="found", seed=860}))
print(work(m_shB, {hand="body", pile=p_gs2, coverage=0.9, tool="filbert 12", angle=0.5, clip=true, edge="soft", seed=861}))
print(work(m_sh2B, {hand="body", pile=p_gs2, coverage=0.8, tool="filbert 10", angle=0.5, clip=true, edge="soft", seed=862}))
print(work(m_litB, {hand="body", pile=g_lite, coverage=0.85, tool="filbert 12", angle=-0.7, clip=true, edge="soft", seed=863}))
print(work(m_lit2B, {hand="body", pile=g_hot, coverage=0.7, tool="filbert 9", angle=-0.7, clip=true, edge="soft", seed=864}))
print(work(m_rfB, {hand="body", pile=p_clothmid, coverage=0.45, tool="filbert 8", angle=0.2, clip=true, edge="lost", seed=865}))
print(blend(mfB, {angle=-0.6}))

--@ chunk 149
m_grp = m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl
halo = (m_grp:grow(17) - m_grp):blur(5)
print(work(halo, {hand="body", pile=p_rs2, coverage=1.3, tool="filbert 14", clip=true, edge="lost", seed=870}))

--@ chunk 150
m_grp = m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl
print("grp", math.floor(m_grp:area()))
g17 = m_grp:grow(17)
print("grown", math.floor(g17:area()))
ring = g17 - m_grp
print("ring", math.floor(ring:area()))
print("ring at cloth 600,650:", ring:at(600,650), " at bowl 600,560:", ring:at(600,560), " at appleB 606,498:", ring:at(606,498), " at 150,700:", ring:at(150,700))
h2 = ring:blur(5)
print("blurred", math.floor(h2:area()))
print("blur at 600,650:", h2:at(600,650), " 150,700:", h2:at(150,700), " 606,498:", h2:at(606,498))

--@ chunk 151
b_mid = pile{{"yellow ochre",4},{"red earth",2.5},{"raw umber",1}}
b_lt  = pile{{"yellow ochre",3},{"lead white",2},{"red earth",1.2}}
b_sh  = pile{{"raw umber",4},{"red earth",2},{"bone black",1.2}}
m_bw = m_bowl:grow(2)
dBowl = function(x, y)
  local dx = (x - 596) / 176
  local dy = (y - 604) / 78
  return 0.78 * dx + 0.62 * dy
end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  local yrim = 530 + 54 * math.sqrt(1 - u * u)
  return smoothstep(30, 3, y - yrim)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  local ybot = 566 + 92 * math.sqrt(1 - u * u)
  return smoothstep(34, 8, ybot - y)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.0, tool="filbert 14", clip=true, edge="found", seed=880}))
print(work(m_bw * zt(0.16, -0.20), {hand="body", pile=b_lt, coverage=0.9, tool="filbert 12", angle=-0.5, clip=true, edge="soft", seed=881}))
print(work(m_bw * zt(0.42, 0.72), {hand="body", pile=b_sh, coverage=0.95, tool="filbert 12", angle=0.5, clip=true, edge="soft", seed=882}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=0.9, tool="filbert 10", angle=0.1, clip=true, edge="soft", seed=883}))
print(work(botlit, {hand="body", pile=p_clothmid, coverage=0.5, tool="filbert 9", angle=0.1, clip=true, edge="soft", seed=884}))
print(blend(m_bw, {angle=0.2}))

--@ chunk 152
r_mid = pile{{"vermilion",3.5},{"red earth",2.5},{"raw umber",0.8}}
r_lt  = pile{{"vermilion",2.5},{"yellow ochre",1.6},{"lead white",1.6},{"red earth",1}}
r_hot = pile{{"lead white",2.6},{"yellow ochre",1.6},{"vermilion",1.2},{"red earth",0.5}}
r_sh  = pile{{"raw umber",3.5},{"red earth",1.5},{"bone black",1.3},{"vermilion",0.4}}
gr_mid = pile{{"green earth",4.5},{"yellow ochre",2},{"chrome yellow",0.4},{"lead white",0.4}}
gr_lt  = pile{{"chrome yellow",0.5},{"lead white",1.6},{"yellow ochre",2.2},{"green earth",2}}
gr_hot = pile{{"lead white",2.6},{"yellow ochre",1.6},{"chrome yellow",0.6},{"green earth",1}}
gr_sh  = pile{{"raw umber",3},{"green earth",2},{"bone black",1.2},{"Prussian blue",0.3}}

function halo(m, r, sd)
  work((m:grow(r) - m):blur(5), {hand="body", pile=p_rs2, coverage=2.0,
        tool="filbert 12", clip=true, edge="found", seed=sd})
end

function apple(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  halo(m, 13, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  local rf = mf * mask(function(x, y)
    local dy = (y - cy) / ry
    return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
  end)
  work(mf, {hand="body", pile=c_mid, coverage=3.0, tool="filbert 14", clip=true, edge="found", seed=sd+1})
  work(mf * zt(0.30, 0.62), {hand="body", pile=c_sh, coverage=0.9, tool="filbert 12", angle=0.5, clip=true, edge="soft", seed=sd+2})
  work(mf * zt(0.66, 1.00), {hand="body", pile=c_sh, coverage=0.8, tool="filbert 10", angle=0.5, clip=true, edge="soft", seed=sd+3})
  work(mf * zt(0.06, -0.16), {hand="body", pile=c_lt, coverage=0.85, tool="filbert 12", angle=-0.7, clip=true, edge="soft", seed=sd+4})
  work(mf * zt(-0.24, -0.52), {hand="body", pile=c_hot, coverage=0.7, tool="filbert 9", angle=-0.7, clip=true, edge="soft", seed=sd+5})
  work(rf, {hand="body", pile=c_rf, coverage=0.45, tool="filbert 8", angle=0.2, clip=true, edge="soft", seed=sd+6})
  blend(mf, {angle=-0.6})
end

--@ chunk 153
halo(m_bowl, 13, 890)
apple(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, p_clothmid, 900)
apple(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, p_clothmid, 920)

--@ chunk 154
function apple(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  halo(m, 10, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  local rf = mf * mask(function(x, y)
    local dy = (y - cy) / ry
    return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
  end)
  work(mf, {hand="body", pile=c_mid, coverage=3.0, tool="filbert 14", clip=true, edge="found", seed=sd+1})
  work(mf * zt(0.30, 0.62), {hand="body", pile=c_sh, coverage=0.9, tool="filbert 12", angle=0.5, clip=true, edge="soft", seed=sd+2})
  work(mf * zt(0.66, 1.00), {hand="body", pile=c_sh, coverage=0.8, tool="filbert 10", angle=0.5, clip=true, edge="soft", seed=sd+3})
  work(mf * zt(0.06, -0.16), {hand="body", pile=c_lt, coverage=0.85, tool="filbert 12", angle=-0.7, clip=true, edge="soft", seed=sd+4})
  work(mf * zt(-0.24, -0.52), {hand="body", pile=c_hot, coverage=0.7, tool="filbert 9", angle=-0.7, clip=true, edge="soft", seed=sd+5})
  work(rf, {hand="body", pile=c_rf, coverage=0.45, tool="filbert 8", angle=0.2, clip=true, edge="soft", seed=sd+6})
  blend(mf:shrink(5), {angle=-0.6})
end
apple(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, p_clothmid, 940)

--@ chunk 155
gr_mid2 = pile{{"green earth",5},{"yellow ochre",1.6},{"chrome yellow",0.3},{"raw umber",0.6}}
gr_lt2  = pile{{"chrome yellow",0.5},{"lead white",1.2},{"yellow ochre",2},{"green earth",2.2}}
gr_hot2 = pile{{"lead white",2.2},{"yellow ochre",1.4},{"chrome yellow",0.5},{"green earth",1.2}}
gr_sh2  = pile{{"raw umber",3.5},{"green earth",2},{"bone black",1.4},{"Prussian blue",0.3}}
cx, cy, rx, ry = 606, 498, 92, 86
mfB = m_apB:grow(1.5)
dB = function(x, y) local dx=(x-cx)/rx; local dy=(y-cy)/ry; return 0.80*dx+0.58*dy+0.20*(dx*dx+dy*dy) end
local zt = function(a,b) return mask(function(x,y) return smoothstep(a,b,dB(x,y)) end) end
print(work(mfB, {hand="body", pile=gr_mid2, coverage=3.0, tool="filbert 14", clip=true, edge="found", seed=960}))
print(work(mfB * zt(0.26, 0.58), {hand="body", pile=gr_sh2, coverage=1.1, tool="filbert 12", angle=0.5, clip=true, edge="soft", seed=961}))
print(work(mfB * zt(0.62, 0.96), {hand="body", pile=gr_sh2, coverage=0.9, tool="filbert 10", angle=0.5, clip=true, edge="soft", seed=962}))
print(work(mfB * zt(0.0, -0.20), {hand="body", pile=gr_lt2, coverage=0.8, tool="filbert 12", angle=-0.7, clip=true, edge="soft", seed=963}))
print(work(mfB * zt(-0.34, -0.62), {hand="body", pile=gr_hot2, coverage=0.65, tool="filbert 9", angle=-0.7, clip=true, edge="soft", seed=964}))
print(blend(mfB:shrink(5), {angle=-0.6}))
print("cloth top edge:")
for _, x in ipairs{60, 160, 260, 360, 460, 560, 660, 760, 860} do
  local y = 520
  while y < 800 and m_cloth:at(x, y) < 0.5 do y = y + 4 end
  print(x, y)
end

--@ chunk 156
for _, y in ipairs{600, 660, 720, 780, 799} do
  local a, b = 1000, 0
  for x = 0, 999 do if m_cloth:at(x, y) > 0.5 then if x < a then a = x end; if x > b then b = x end end end
  print(y, a, b)
end

--@ chunk 157
cpts5 = {{300,540},{450,506},{620,496},{780,512},{852,552},{886,614},
         {858,666},{898,756},{928,800},{40,800},{40,688},{108,640},{128,586},{210,556}}
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_lt  = pile{{"lead white",8},{"yellow ochre",0.8},{"raw umber",0.5}}
cl_sh  = pile{{"lead white",3.5},{"raw umber",4},{"cobalt blue",0.7},{"yellow ochre",0.4}}
cl_dsh = pile{{"lead white",1.4},{"raw umber",3.4},{"bone black",1.5},{"cobalt blue",0.6}}
litL = mask(function(x, y) return smoothstep(1.15, -0.25, (x - 120) / 520 + (y - 470) / 300) end)
litL2 = mask(function(x, y) return smoothstep(0.80, -0.30, (x - 120) / 520 + (y - 470) / 300) end)
t1 = ribbon({{236,596},{372,652},{508,724}}, 34):soften(24)
t2 = ribbon({{150,652},{296,712},{420,776}}, 30):soften(22)
t3 = ribbon({{706,596},{766,668},{812,762}}, 32):soften(24)
t4 = ribbon({{402,536},{506,596},{596,656}}, 26):soften(20)
c1 = ribbon({{186,566},{318,622},{452,690}}, 26):soften(20)
c2 = ribbon({{104,606},{244,664},{372,724}}, 24):soften(18)
c3 = ribbon({{654,566},{716,634},{766,716}}, 24):soften(20)
bowlsh = (ellipse(672, 664, 200, 58):soften(42) + ellipse(600, 646, 150, 40):soften(30))
print(work(m_cl, {hand="body", pile=p_clothmid, coverage=2.6, tool="filbert 20", angle=0.3, clip=true, edge="soft", seed=970}))
print(work(m_cl * litL, {hand="body", pile=cl_lt, coverage=0.8, tool="filbert 18", angle=0.35, clip=true, edge="soft", seed=971}))
print(work(m_cl * litL2, {hand="body", pile=cl_lt, coverage=0.5, tool="filbert 14", angle=0.35, clip=true, edge="soft", seed=972}))
print(work((t1 + t2 + t3 + t4) * m_cl, {hand="body", pile=cl_sh, coverage=0.85, tool="filbert 16", angle=0.5, clip=true, edge="soft", seed=973}))
print(work((t1 + t3) * m_cl, {hand="body", pile=cl_dsh, coverage=0.5, tool="filbert 12", angle=0.5, clip=true, edge="soft", seed=974}))
print(work((c1 + c2 + c3) * m_cl, {hand="body", pile=cl_lt, coverage=0.55, tool="filbert 12", angle=0.4, clip=true, edge="soft", seed=975}))
print(work(bowlsh * m_cl, {hand="body", pile=cl_dsh, coverage=0.9, tool="filbert 18", angle=0.2, clip=true, edge="soft", seed=976}))
print(blend(m_cl, {angle=0.3}))

--@ chunk 158
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_lt  = pile{{"lead white",8},{"yellow ochre",0.8},{"raw umber",0.5}}
cl_sh  = pile{{"lead white",3.5},{"raw umber",4},{"cobalt blue",0.7},{"yellow ochre",0.4}}
cl_dsh = pile{{"lead white",1.4},{"raw umber",3.4},{"bone black",1.5},{"cobalt blue",0.6}}
litA = mask(function(x, y) return smoothstep(1.30, 0.10, (x - 150) / 480 + (y - 480) / 300) end)
litB = mask(function(x, y) return smoothstep(0.85, -0.10, (x - 150) / 480 + (y - 480) / 300) end)
T = ribbon({{300,560},{240,640},{180,730},{150,800}}, 30):soften(20)
   + ribbon({{400,540},{350,630},{300,720},{270,800}}, 26):soften(18)
   + ribbon({{500,530},{470,620},{440,720},{420,800}}, 24):soften(18)
   + ribbon({{680,530},{720,620},{760,720},{790,800}}, 28):soften(20)
   + ribbon({{790,560},{830,650},{860,740},{880,800}}, 26):soften(20)
K = ribbon({{340,556},{285,640},{230,730},{205,800}}, 26):soften(18)
   + ribbon({{445,536},{400,628},{355,718},{330,800}}, 24):soften(16)
   + ribbon({{560,528},{545,618},{520,718},{500,800}}, 24):soften(16)
   + ribbon({{730,540},{775,628},{812,722},{838,800}}, 26):soften(18)
grpsh = ellipse(700, 660, 210, 62):soften(44) + ellipse(420, 700, 110, 34):soften(28)
       + ellipse(560, 740, 90, 30):soften(24)
print(work(m_cl, {hand="body", pile=cl_sh, coverage=2.8, tool="filbert 20", angle=0.3, clip=true, edge="soft", seed=980}))
print(work(m_cl * litA, {hand="body", pile=p_clothmid, coverage=0.9, tool="filbert 18", angle=0.35, clip=true, edge="soft", seed=981}))
print(work(m_cl * litB, {hand="body", pile=cl_lt, coverage=0.6, tool="filbert 14", angle=0.35, clip=true, edge="soft", seed=982}))
print(work(T * m_cl, {hand="body", pile=cl_dsh, coverage=0.8, tool="filbert 16", angle=0.55, clip=true, edge="soft", seed=983}))
print(work(K * m_cl, {hand="body", pile=cl_lt, coverage=0.55, tool="filbert 14", angle=0.5, clip=true, edge="soft", seed=984}))
print(work(grpsh * m_cl, {hand="body", pile=cl_dsh, coverage=0.8, tool="filbert 18", angle=0.2, clip=true, edge="soft", seed=985}))
print(blend(m_cl, {angle=0.3}))

--@ chunk 159
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_lt  = pile{{"lead white",8},{"yellow ochre",0.8},{"raw umber",0.5}}
cl_dsh = pile{{"lead white",1.6},{"raw umber",3.2},{"bone black",1.6},{"cobalt blue",0.6}}
litA = mask(function(x, y) return smoothstep(1.30, 0.10, (x - 150) / 480 + (y - 480) / 300) end)
litB = mask(function(x, y) return smoothstep(0.85, -0.10, (x - 150) / 480 + (y - 480) / 300) end)
T = ribbon({{300,560},{240,640},{180,730},{150,800}}, 30):soften(20)
   + ribbon({{400,540},{350,630},{300,720},{270,800}}, 26):soften(18)
   + ribbon({{500,530},{470,620},{440,720},{420,800}}, 24):soften(18)
   + ribbon({{680,530},{720,620},{760,720},{790,800}}, 28):soften(20)
   + ribbon({{790,560},{830,650},{860,740},{880,800}}, 26):soften(20)
K = ribbon({{340,556},{285,640},{230,730},{205,800}}, 26):soften(18)
   + ribbon({{445,536},{400,628},{355,718},{330,800}}, 24):soften(16)
   + ribbon({{560,528},{545,618},{520,718},{500,800}}, 24):soften(16)
   + ribbon({{730,540},{775,628},{812,722},{838,800}}, 26):soften(18)
grpsh = ellipse(700, 660, 210, 62):soften(44) + ellipse(420, 700, 110, 34):soften(28)
       + ellipse(560, 742, 90, 30):soften(24)
print(work(m_cl, {hand="body", pile=p_csh2, coverage=5.0, tool="filbert 12", angle=0.3, clip=true, edge="soft", seed=990}))
print(work(m_cl * litA, {hand="body", pile=p_clothmid, coverage=1.2, tool="filbert 16", angle=0.35, clip=true, edge="soft", seed=991}))
print(work(m_cl * litB, {hand="body", pile=cl_lt, coverage=0.7, tool="filbert 13", angle=0.35, clip=true, edge="soft", seed=992}))
print(work(T * m_cl, {hand="body", pile=cl_dsh, coverage=0.9, tool="filbert 16", angle=0.55, clip=true, edge="soft", seed=993}))
print(work(K * m_cl, {hand="body", pile=cl_lt, coverage=0.5, tool="filbert 13", angle=0.5, clip=true, edge="soft", seed=994}))
print(work(grpsh * m_cl, {hand="body", pile=cl_dsh, coverage=0.9, tool="filbert 18", angle=0.2, clip=true, edge="soft", seed=995}))

--@ chunk 160
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_base = pile{{"raw umber",4.5},{"lead white",2.2},{"cobalt blue",0.5},{"yellow ochre",0.5}}
cl_lt  = pile{{"lead white",7},{"yellow ochre",0.8},{"raw umber",0.6}}
cl_dsh = pile{{"raw umber",3.2},{"bone black",1.6},{"lead white",0.8},{"cobalt blue",0.5}}
drapefold = function(x, y) return 1.10 + 0.95 * smoothstep(880, 200, x) + 0.07 * math.sin(y / 95) end
litA = mask(function(x, y) return smoothstep(1.30, 0.10, (x - 150) / 480 + (y - 480) / 300) end)
litB = mask(function(x, y) return smoothstep(0.80, -0.15, (x - 150) / 480 + (y - 480) / 300) end)
T = ribbon({{300,560},{240,640},{180,730},{150,800}}, 30):soften(20)
   + ribbon({{400,540},{350,630},{300,720},{270,800}}, 26):soften(18)
   + ribbon({{500,530},{470,620},{440,720},{420,800}}, 24):soften(18)
   + ribbon({{680,530},{720,620},{760,720},{790,800}}, 28):soften(20)
   + ribbon({{790,560},{830,650},{860,740},{880,800}}, 26):soften(20)
K = ribbon({{340,556},{285,640},{230,730},{205,800}}, 26):soften(18)
   + ribbon({{445,536},{400,628},{355,718},{330,800}}, 24):soften(16)
   + ribbon({{560,528},{545,618},{520,718},{500,800}}, 24):soften(16)
   + ribbon({{730,540},{775,628},{812,722},{838,800}}, 26):soften(18)
grpsh = ellipse(700, 660, 210, 62):soften(44) + ellipse(420, 700, 110, 34):soften(28)
       + ellipse(560, 742, 90, 30):soften(24)
local common = {hand="body", clip=true, edge="soft", angle=drapefold,
                length={26,80}, curve={0.25,0.3}, angle_jitter=0.35, broken=0.3, clump=0.35}
print(work(m_cl, {pile=cl_base, coverage=5.0, tool="filbert 12", seed=1000, table.unpack(common, 1, 5)}))
print(work(m_cl * litA, {pile=p_clothmid, coverage=1.1, tool="filbert 16", seed=1001, table.unpack(common, 1, 5)}))
print(work(m_cl * litB, {pile=cl_lt, coverage=0.7, tool="filbert 13", seed=1002, table.unpack(common, 1, 5)}))
print(work(T * m_cl, {pile=cl_dsh, coverage=0.9, tool="filbert 16", seed=1003, table.unpack(common, 1, 5)}))
print(work(K * m_cl, {pile=cl_lt, coverage=0.55, tool="filbert 13", seed=1004, table.unpack(common, 1, 5)}))
print(work(grpsh * m_cl, {pile=cl_dsh, coverage=0.9, tool="filbert 18", seed=1005, table.unpack(common, 1, 5)}))

--@ chunk 161
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
print(blend(m_cl, {angle=1.2}))

--@ chunk 162
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_base = pile{{"raw umber",4.5},{"lead white",2.2},{"cobalt blue",0.5},{"yellow ochre",0.5}}
drapefold = function(x, y) return 1.10 + 0.95 * smoothstep(880, 200, x) + 0.07 * math.sin(y / 95) end
print(work(m_cl, {hand="body", pile=cl_base, coverage=2.4, tool="filbert 20", angle=drapefold,
  length={40,110}, curve={0.30,0.35}, angle_jitter=0.3, broken=0.25, clump=0.3, clip=true, edge="soft", seed=1010}))
print(blend(m_cl, {angle=1.2}))

--@ chunk 163
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}
drapefold = function(x, y) return 1.10 + 0.95 * smoothstep(880, 200, x) + 0.07 * math.sin(y / 95) end
print(work(m_cl, {hand="body", pile=cl_mid, coverage=2.8, tool="filbert 20", angle=drapefold,
  length={40,110}, curve={0.30,0.35}, angle_jitter=0.3, broken=0.25, clump=0.3, clip=true, edge="soft", seed=1020}))
print(blend(m_cl, {angle=1.2}))

--@ chunk 164
dk = pile{{"bone black",4},{"raw umber",3}}
print(work(rect(100,720,140,70), {hand="body", pile=dk, coverage=3.0, tool="filbert 14", clip=true, edge="found", seed=1030}))

--@ chunk 165
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}
drapefold = function(x, y) return 1.10 + 0.95 * smoothstep(880, 200, x) + 0.07 * math.sin(y / 95) end
print(work(m_cl, {hand="body", pile=cl_mid, coverage=6.0, tool="filbert 12", angle=drapefold,
  length={24,64}, curve={0.22,0.26}, angle_jitter=0.3, broken=0.2, clump=0.25,
  dips={4, 1.0, 0.1}, fill=true, clip=true, edge="soft", seed=1040}))

--@ chunk 166
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_lit  = pile{{"lead white",4},{"raw umber",2.8},{"yellow ochre",0.7}}
cl_tr   = pile{{"lead white",1.2},{"raw umber",3.5},{"bone black",1.6},{"cobalt blue",0.6},{"red earth",0.4}}
drapefold = function(x, y) return 1.10 + 0.95 * smoothstep(880, 200, x) + 0.07 * math.sin(y / 95) end
litA = mask(function(x, y) return smoothstep(1.30, 0.10, (x - 150) / 480 + (y - 480) / 300) end)
litB = mask(function(x, y) return smoothstep(0.80, -0.15, (x - 150) / 480 + (y - 480) / 300) end)
T = ribbon({{300,560},{240,640},{180,730},{150,800}}, 30):soften(20)
   + ribbon({{400,540},{350,630},{300,720},{270,800}}, 26):soften(18)
   + ribbon({{500,530},{470,620},{440,720},{420,800}}, 24):soften(18)
   + ribbon({{680,530},{720,620},{760,720},{790,800}}, 28):soften(20)
   + ribbon({{790,560},{830,650},{860,740},{880,800}}, 26):soften(20)
K = ribbon({{340,556},{285,640},{230,730},{205,800}}, 26):soften(18)
   + ribbon({{445,536},{400,628},{355,718},{330,800}}, 24):soften(16)
   + ribbon({{560,528},{545,618},{520,718},{500,800}}, 24):soften(16)
   + ribbon({{730,540},{775,628},{812,722},{838,800}}, 26):soften(18)
grpsh = ellipse(700, 664, 205, 58):soften(40) + ellipse(420, 706, 108, 32):soften(26)
       + ellipse(560, 748, 88, 28):soften(22)
local C = {hand="body", clip=true, edge="soft", angle=drapefold, tool="filbert 12",
           length={24,64}, curve={0.22,0.26}, angle_jitter=0.3, broken=0.2, clump=0.25,
           dips={4, 1.0, 0.1}, fill=true}
print(work(m_cl * litA, {pile=cl_lit, coverage=0.75, seed=1050, table.unpack(C, 1, 8)}))
print(work(m_cl * litB, {pile=cl_lit, coverage=0.55, seed=1051, table.unpack(C, 1, 8)}))
print(work(T * m_cl, {pile=cl_tr, coverage=0.65, seed=1052, table.unpack(C, 1, 8)}))
print(work(K * m_cl, {pile=cl_lit, coverage=0.45, seed=1053, table.unpack(C, 1, 8)}))
print(work(grpsh * m_cl, {pile=cl_tr, coverage=0.85, seed=1054, table.unpack(C, 1, 8)}))

--@ chunk 167
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}
cl_lit = pile{{"lead white",4},{"raw umber",2.8},{"yellow ochre",0.7}}
cl_tr  = pile{{"lead white",1.2},{"raw umber",3.5},{"bone black",1.6},{"cobalt blue",0.6},{"red earth",0.4}}
drapefold = function(x, y) return 1.10 + 0.95 * smoothstep(880, 200, x) + 0.07 * math.sin(y / 95) end
T = ribbon({{300,560},{240,640},{180,730},{150,800}}, 30):soften(20)
   + ribbon({{400,540},{350,630},{300,720},{270,800}}, 26):soften(18)
   + ribbon({{500,530},{470,620},{440,720},{420,800}}, 24):soften(18)
   + ribbon({{680,530},{720,620},{760,720},{790,800}}, 28):soften(20)
   + ribbon({{790,560},{830,650},{860,740},{880,800}}, 26):soften(20)
grpsh = ellipse(700, 664, 205, 58):soften(40) + ellipse(420, 706, 108, 32):soften(26)
       + ellipse(560, 748, 88, 28):soften(22)
print(work(m_cl, {hand="body", pile=cl_mid, coverage=6.0, tool="filbert 12", angle=drapefold,
  length={24,64}, curve={0.22,0.26}, angle_jitter=0.3, broken=0.2, clump=0.25,
  dips={4, 1.0, 0.1}, fill=true, clip=true, edge="soft", seed=1060}))
print(stipple(T * m_cl, {pile=cl_tr, width=7, coverage=0.9, feather=0.5, cluster=0.3, clip=true, seed=1061}))
print(stipple(grpsh * m_cl, {pile=cl_tr, width=8, coverage=1.0, feather=0.5, cluster=0.3, clip=true, seed=1062}))

--@ chunk 168
b_mid = pile{{"yellow ochre",4},{"red earth",2},{"raw umber",1.2}}
b_lt  = pile{{"yellow ochre",3},{"lead white",2.2},{"red earth",1.2}}
b_sh  = pile{{"raw umber",3.5},{"red earth",1.8},{"bone black",1.4}}
b_rim = pile{{"yellow ochre",2},{"lead white",3},{"raw umber",0.5}}
local C = {hand="body", clip=true, tool="filbert 13", length={20,52}, curve={0.2,0.16},
           angle_jitter=0.3, broken=0.15, dips={4, 1.0, 0.1}, fill=true, edge="soft"}
m_bw = m_bowl:grow(2)
dBowl = function(x, y)
  local dx = (x - 596) / 176
  local dy = (y - 604) / 78
  return 0.78 * dx + 0.62 * dy
end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  local yrim = 530 + 54 * math.sqrt(1 - u * u)
  return smoothstep(34, 4, y - yrim)
end)
rimlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  local yrim = 530 + 54 * math.sqrt(1 - u * u)
  return smoothstep(9, 0, y - yrim) * smoothstep(0.30, -0.45, (x - 596) / 150)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  local ybot = 566 + 92 * math.sqrt(1 - u * u)
  return smoothstep(30, 10, ybot - y)
end)
print(work(m_bw, {pile=b_mid, coverage=4.0, seed=1070, table.unpack(C, 1, 8)}))
print(work(m_bw * zt(0.14, -0.22), {pile=b_lt, coverage=1.0, angle=-0.5, seed=1071, table.unpack(C, 1, 8)}))
print(work(m_bw * zt(0.40, 0.70), {pile=b_sh, coverage=1.0, angle=0.5, seed=1072, table.unpack(C, 1, 8)}))
print(work(rimshade, {pile=b_sh, coverage=0.9, angle=0.1, seed=1073, table.unpack(C, 1, 8)}))
print(work(botlit, {pile=p_clothmid, coverage=0.6, angle=0.1, seed=1074, table.unpack(C, 1, 8)}))
print(work(rimlit, {pile=b_rim, coverage=0.7, tool="filbert 8", angle=-0.3, length={12,34}, seed=1075, table.unpack(C, 1, 8)}))

--@ chunk 169
b_mid = pile{{"yellow ochre",4},{"red earth",2},{"raw umber",1.2}}
b_lt  = pile{{"yellow ochre",3},{"lead white",2.2},{"red earth",1.2}}
b_sh  = pile{{"raw umber",3.5},{"red earth",1.8},{"bone black",1.4}}
b_rim = pile{{"yellow ochre",2},{"lead white",3},{"raw umber",0.5}}
local C = {hand="body", clip=true, tool="filbert 14", length={20,52}, curve={0.2,0.16},
           angle_jitter=0.3, dips={4, 1.0, 0.1}, fill=true, edge="soft"}
m_bw = m_bowl:grow(2)
dBowl = function(x, y)
  local dx = (x - 596) / 176
  local dy = (y - 604) / 78
  return 0.78 * dx + 0.62 * dy
end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  local yrim = 530 + 54 * math.sqrt(1 - u * u)
  return smoothstep(34, 4, y - yrim)
end)
rimlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  local yrim = 530 + 54 * math.sqrt(1 - u * u)
  return smoothstep(9, 0, y - yrim) * smoothstep(0.30, -0.45, (x - 596) / 150)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  local ybot = 566 + 92 * math.sqrt(1 - u * u)
  return smoothstep(30, 10, ybot - y)
end)
print(work(m_bw, {pile=b_mid, coverage=5.0, seed=1080, table.unpack(C)}))
print(work(m_bw * zt(0.14, -0.22), {pile=b_lt, coverage=1.0, angle=-0.5, seed=1081, table.unpack(C)}))
print(work(m_bw * zt(0.40, 0.70), {pile=b_sh, coverage=1.0, angle=0.5, seed=1082, table.unpack(C)}))
print(work(rimshade, {pile=b_sh, coverage=0.9, angle=0.1, seed=1083, table.unpack(C)}))
print(work(botlit, {pile=p_clothmid, coverage=0.6, angle=0.1, seed=1084, table.unpack(C)}))
print(work(rimlit, {pile=b_rim, coverage=0.7, tool="filbert 8", angle=-0.3, length={12,34}, seed=1085, table.unpack(C)}))

--@ chunk 170
b_mid = pile{{"yellow ochre",4},{"red earth",2},{"raw umber",1.2}}
b_lt  = pile{{"yellow ochre",3},{"lead white",2.2},{"red earth",1.2}}
b_sh  = pile{{"raw umber",3.5},{"red earth",1.8},{"bone black",1.4}}
b_rim = pile{{"yellow ochre",2},{"lead white",3},{"raw umber",0.5}}
m_bw = m_bowl:grow(2)
dBowl = function(x, y)
  local dx = (x - 596) / 176
  local dy = (y - 604) / 78
  return 0.78 * dx + 0.62 * dy
end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(34, 4, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
rimlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(9, 0, y - (530 + 54 * math.sqrt(1 - u * u))) * smoothstep(0.30, -0.45, (x - 596) / 150)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(30, 10, (566 + 92 * math.sqrt(1 - u * u)) - y)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.2, tool="filbert 14", clip=true, fill=true, edge="found", seed=1090}))
print(work(m_bw * zt(0.14, -0.22), {hand="body", pile=b_lt, coverage=1.0, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1091}))
print(work(m_bw * zt(0.40, 0.70), {hand="body", pile=b_sh, coverage=1.0, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1092}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=0.9, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1093}))
print(work(botlit, {hand="body", pile=p_clothmid, coverage=0.6, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1094}))
print(work(rimlit, {hand="body", pile=b_rim, coverage=0.7, tool="filbert 8", angle=-0.3, clip=true, edge="soft", seed=1095}))

--@ chunk 171
b_mid = pile{{"yellow ochre",3},{"red earth",2.5},{"raw umber",2},{"bone black",0.4}}
b_lt  = pile{{"yellow ochre",2.5},{"lead white",1.2},{"red earth",1.5}}
b_sh  = pile{{"raw umber",3},{"bone black",2},{"red earth",1}}
m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(34, 4, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(26, 8, (566 + 92 * math.sqrt(1 - u * u)) - y) * smoothstep(0.45, -0.5, (x - 596) / 150)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.2, tool="filbert 14", clip=true, fill=true, edge="found", seed=1100}))
print(work(m_bw * zt(0.14, -0.22), {hand="body", pile=b_lt, coverage=1.0, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1101}))
print(work(m_bw * zt(0.40, 0.70), {hand="body", pile=b_sh, coverage=1.0, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1102}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=0.9, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1103}))
print(work(botlit, {hand="body", pile=b_lt, coverage=0.45, tool="filbert 10", angle=0.1, clip=true, edge="soft", seed=1104}))

function halo(m, r, sd)
  work((m:grow(r) - m):blur(4), {hand="body", pile=p_rs2, coverage=2.0,
        tool="filbert 12", clip=true, fill=true, edge="found", seed=sd})
end
function apple(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  halo(m, 10, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  local rf = mf * mask(function(x, y)
    local dy = (y - cy) / ry
    return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
  end)
  work(mf, {hand="body", pile=c_mid, coverage=4.0, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.30, 0.62), {hand="body", pile=c_sh, coverage=1.0, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(0.66, 1.00), {hand="body", pile=c_sh, coverage=0.9, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.06, -0.16), {hand="body", pile=c_lt, coverage=0.95, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(-0.24, -0.52), {hand="body", pile=c_hot, coverage=0.8, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+5})
  work(rf, {hand="body", pile=c_rf, coverage=0.5, tool="filbert 8", angle=0.2, clip=true, edge="soft", seed=sd+6})
  blend(mf:shrink(6), {angle=-0.6})
end
apple(m_apD, 300, 690, 92, 86, r_mid, r_lt, r_hot, r_sh, p_clothmid, 1110)
apple(m_apE, 478, 736, 72, 66, gr_mid2, gr_lt2, gr_hot2, gr_sh2, p_clothmid, 1130)
apple(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1150)
apple(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1170)
apple(m_apB, 606, 498, 92, 86, gr_mid2, gr_lt2, gr_hot2, gr_sh2, b_lt, 1190)

--@ chunk 172
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
w_glow1 = pile{{"raw umber",4},{"red earth",2},{"bone black",2}}
w_glow2 = pile{{"raw umber",4},{"red earth",2.5},{"yellow ochre",1.2},{"bone black",1.6}}
print(work(rect(640, 0, 360, 150), {hand="broad", pile=p_bgd, tool="filbert 22", angle=0.22, coverage=2.6, clip=true, fill=true, edge="soft", seed=1200}))
gA = ellipse(210, 120, 300, 220):soften(110)
gB = ellipse(190, 100, 190, 150):soften(90)
print(stipple(gA, {pile=w_glow1, width=9, coverage=0.85, feather=0.5, cluster=0.3, clip=true, seed=1201}))
print(stipple(gB, {pile=w_glow2, width=8, coverage=0.7, feather=0.5, cluster=0.3, clip=true, seed=1202}))

--@ chunk 173
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
p_bgw = pile{{"raw umber",4},{"bone black",3.3},{"red earth",1.3},{"Prussian blue",0.6}}
print(work(rect(0, 0, 1000, 330), {hand="broad", pile=p_bgd, tool="filbert 22", angle=0.22, coverage=2.6, clip=true, fill=true, edge="soft", seed=1210}))
print(work(ellipse(230, 130, 300, 220):soften(120), {hand="broad", pile=p_bgw, tool="filbert 20", angle=0.3, coverage=0.9, clip=true, fill=true, edge="soft", seed=1211}))
print(work(ellipse(195, 105, 180, 140):soften(90), {hand="broad", pile=p_bgw, tool="filbert 18", angle=0.3, coverage=0.7, clip=true, fill=true, edge="soft", seed=1212}))

--@ chunk 174
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}
b_mid = pile{{"yellow ochre",3},{"red earth",2.5},{"raw umber",2},{"bone black",0.4}}
print(work(rect(650, 0, 350, 140), {hand="broad", pile=p_bgd, tool="filbert 22", angle=0.22, coverage=4.0, clip=true, fill=true, edge="found", seed=1220}))
print(work(rect(650, 0, 350, 140), {hand="broad", pile=p_bgd, tool="filbert 18", angle=0.5, coverage=3.0, clip=true, fill=true, edge="found", seed=1221}))
-- the two apples on the cloth: take the hard ring away, put a real cast shadow back
work((m_apD:grow(11) - m_apD), {hand="body", pile=cl_mid, coverage=3.0, tool="filbert 12", clip=true, fill=true, edge="found", seed=1222})
work(ellipse(332, 732, 110, 36):soften(28), {hand="body", pile=p_rs2, coverage=0.85, tool="filbert 16", clip=true, fill=true, edge="soft", seed=1223})
work(ellipse(306, 766, 70, 18):soften(12), {hand="body", pile=p_rs2, coverage=0.6, tool="filbert 10", clip=true, edge="soft", seed=1224})
work((m_apE:grow(11) - m_apE), {hand="body", pile=cl_mid, coverage=3.0, tool="filbert 12", clip=true, fill=true, edge="found", seed=1225})
work(ellipse(502, 770, 88, 30):soften(24), {hand="body", pile=p_rs2, coverage=0.85, tool="filbert 16", clip=true, fill=true, edge="soft", seed=1226})
work(ellipse(482, 792, 56, 15):soften(10), {hand="body", pile=p_rs2, coverage=0.6, tool="filbert 10", clip=true, edge="soft", seed=1227})
-- soften the rings round the fruit in the bowl, and seat them with a contact shadow
for _, q in ipairs{{m_apA, 512, 566, 74}, {m_apC, 696, 568, 68}, {m_apB, 606, 556, 78}} do
  local mm, cx, cy, rw = q[1], q[2], q[3], q[4]
  work(mm:grow(12) - mm, {hand="body", pile=p_rs2, coverage=0.8, tool="filbert 12", clip=true, edge="soft", seed=1230 + cx})
  work(ellipse(cx + 6, cy, rw, 22):soften(16) * m_bw, {hand="body", pile=p_rs2, coverage=0.7, tool="filbert 10", clip=true, edge="soft", seed=1240 + cx})
end

--@ chunk 175
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
print(stipple(rect(620, 0, 380, 170), {pile=p_bgd, width=11, coverage=3.2, clip=true, seed=1250}))
print(stipple(rect(0, 0, 1000, 120), {pile=p_bgd, width=11, coverage=1.6, clip=true, seed=1251}))
print(stipple(rect(0, 120, 1000, 230), {pile=p_bgd, width=11, coverage=1.2, clip=true, seed=1252}))

--@ chunk 176
b_mid = pile{{"yellow ochre",3},{"red earth",2.5},{"raw umber",2},{"bone black",0.4}}
b_lt  = pile{{"yellow ochre",2.5},{"lead white",1.2},{"red earth",1.5}}
b_sh  = pile{{"raw umber",3},{"bone black",2},{"red earth",1}}
stem_p = pile{{"raw umber",4},{"bone black",2},{"red earth",1}}
m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(34, 4, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(26, 8, (566 + 92 * math.sqrt(1 - u * u)) - y) * smoothstep(0.45, -0.5, (x - 596) / 150)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=1260}))
print(work(m_bw * zt(0.14, -0.22), {hand="body", pile=b_lt, coverage=1.1, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1261}))
print(work(m_bw * zt(0.40, 0.70), {hand="body", pile=b_sh, coverage=1.1, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1262}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=0.9, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1263}))
print(work(botlit, {hand="body", pile=b_lt, coverage=0.45, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1264}))

function apple2(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  local rf = mf * mask(function(x, y)
    local dy = (y - cy) / ry
    return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
  end)
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.26, 0.58), {hand="body", pile=c_sh, coverage=1.1, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(0.60, 0.95), {hand="body", pile=c_sh, coverage=0.9, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.04, -0.18), {hand="body", pile=c_lt, coverage=1.0, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(-0.28, -0.56), {hand="body", pile=c_hot, coverage=0.85, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+5})
  work(rf, {hand="body", pile=c_rf, coverage=0.55, tool="filbert 8", angle=0.2, clip=true, edge="soft", seed=sd+6})
  blend(mf:shrink(10), {angle=-0.6})
  local cav = ellipse(cx - 0.10 * rx, cy - 0.86 * ry, 0.27 * rx, 0.12 * ry):soften(8) * mf
  work(cav, {hand="body", pile=c_sh, coverage=0.85, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+7})
end
function stem(cx, cy, rx, ry, sd)
  local sb = brush{kind="round", width=5, point=1, stiffness=0.45}
  sb:load(stem_p, 0.95)
  sb:stroke({{cx - 0.09 * rx, cy - 0.80 * ry}, {cx - 0.17 * rx, cy - 1.00 * ry}, {cx - 0.21 * rx, cy - 1.17 * ry}},
            {pressure={0.95, 0.75, 0.25}, ramps={0.05, 0.5}, shake=1})
end
apple2(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1270)
apple2(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1290)
apple2(m_apB, 606, 498, 92, 86, gr_mid2, gr_lt2, gr_hot2, gr_sh2, b_lt, 1310)
stem(512, 516, 86, 80, 1330)
stem(696, 520, 80, 75, 1331)
stem(606, 498, 92, 86, 1332)

--@ chunk 177
-- richer colour, crisper modelling, no smoothing blend
r_mid = pile{{"vermilion",4},{"red earth",2},{"raw umber",0.6},{"bone black",0.3}}
r_lt  = pile{{"vermilion",3},{"yellow ochre",1.5},{"lead white",1.2}}
r_hot = pile{{"vermilion",1.5},{"yellow ochre",1.5},{"lead white",2.2}}
r_sh  = pile{{"raw umber",3},{"red earth",1.2},{"bone black",1.8},{"Prussian blue",0.4}}
gr_mid = pile{{"green earth",5},{"yellow ochre",1.5},{"chrome yellow",0.3}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2},{"chrome yellow",0.8},{"lead white",0.8}}
gr_hot = pile{{"chrome yellow",1.5},{"lead white",2.5},{"yellow ochre",1.5}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.5},{"bone black",2},{"Prussian blue",0.4}}
b_mid = pile{{"raw umber",3},{"red earth",2},{"yellow ochre",2},{"bone black",0.5}}
b_lt  = pile{{"yellow ochre",2.5},{"red earth",1.5},{"lead white",1}}
b_sh  = pile{{"raw umber",3},{"bone black",2.2},{"red earth",0.8}}
b_rim = pile{{"yellow ochre",1.5},{"lead white",2.5},{"raw umber",0.4}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}

m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
local ztB = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(30, 3, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
rimlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(8, 0, y - (530 + 54 * math.sqrt(1 - u * u))) * smoothstep(0.25, -0.5, (x - 596) / 150)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(24, 8, (566 + 92 * math.sqrt(1 - u * u)) - y) * smoothstep(0.45, -0.5, (x - 596) / 150)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=1340}))
print(work(m_bw * ztB(0.12, -0.24), {hand="body", pile=b_lt, coverage=1.1, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1341}))
print(work(m_bw * ztB(0.38, 0.68), {hand="body", pile=b_sh, coverage=1.1, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1342}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=1.0, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1343}))
print(work(botlit, {hand="body", pile=b_lt, coverage=0.5, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1344}))
print(work(rimlit, {hand="body", pile=b_rim, coverage=0.8, tool="filbert 7", angle=-0.3, clip=true, fill=true, edge="soft", seed=1345}))

function apple3(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  local rf = mf * mask(function(x, y)
    local dy = (y - cy) / ry
    return smoothstep(0.78, 1.18, d(x, y)) * smoothstep(0.28, 0.76, dy)
  end)
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.24, 0.56), {hand="body", pile=c_sh, coverage=1.15, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(0.58, 0.95), {hand="body", pile=c_sh, coverage=0.95, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.10, -0.10), {hand="body", pile=c_lt, coverage=0.95, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(-0.22, -0.48), {hand="body", pile=c_hot, coverage=0.8, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+5})
  work(rf, {hand="body", pile=c_rf, coverage=0.6, tool="filbert 8", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+6})
  local cav = ellipse(cx - 0.08 * rx, cy - 0.85 * ry, 0.28 * rx, 0.13 * ry):soften(9) * mf
  work(cav, {hand="body", pile=c_sh, coverage=0.9, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+7})
end
function catchlight(cx, cy, rx, ry, pile, sd)
  local hb = brush{kind="round", width=7, point=1, stiffness=0.3}
  hb:load(pile, 1.0)
  hb:touch(cx - 0.40 * rx, cy - 0.44 * ry, {pressure=0.85})
  hb:wipe(0.45)
  hb:touch(cx - 0.50 * rx, cy - 0.14 * ry, {pressure=0.4})
end
function stem(cx, cy, rx, ry, sd)
  local sb = brush{kind="round", width=7, point=1, stiffness=0.5}
  sb:load(stem_p, 1.0)
  sb:stroke({{cx - 0.07 * rx, cy - 0.82 * ry}, {cx - 0.15 * rx, cy - 0.98 * ry}},
            {pressure={1.0, 0.55}, ramps={0.06, 0.45}, shake=1})
end
apple3(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1350)
apple3(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1370)
apple3(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, b_lt, 1390)
apple3(m_apD, 300, 690, 92, 86, r_mid, r_lt, r_hot, r_sh, cl_mid, 1410)
apple3(m_apE, 478, 736, 72, 66, gr_mid, gr_lt, gr_hot, gr_sh, cl_mid, 1430)
for _, q in ipairs{{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot},{300,690,92,86,r_hot},{478,736,72,66,gr_hot}} do
  stem(q[1], q[2], q[3], q[4])
end
for _, q in ipairs{{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot},{300,690,92,86,r_hot},{478,736,72,66,gr_hot}} do
  catchlight(q[1], q[2], q[3], q[4], q[5])
end

--@ chunk 178
r_mid = pile{{"vermilion",2.6},{"red earth",3},{"raw umber",1.2}}
r_lt  = pile{{"vermilion",2},{"red earth",1.6},{"yellow ochre",1.6},{"lead white",1}}
r_hot = pile{{"vermilion",1.2},{"yellow ochre",1.6},{"lead white",2.4}}
r_sh  = pile{{"raw umber",3},{"red earth",1.4},{"bone black",1.6},{"Prussian blue",0.3}}
gr_mid = pile{{"green earth",5},{"yellow ochre",1.6},{"raw umber",0.8}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2.2},{"chrome yellow",0.6},{"lead white",0.6}}
gr_hot = pile{{"chrome yellow",1},{"lead white",2.6},{"yellow ochre",1.6}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.6},{"bone black",1.8},{"Prussian blue",0.4}}
b_mid = pile{{"raw umber",3},{"red earth",2},{"yellow ochre",1.6},{"bone black",0.6}}
b_lt  = pile{{"yellow ochre",2.2},{"red earth",1.6},{"lead white",0.9}}
b_sh  = pile{{"raw umber",3},{"bone black",2.2},{"red earth",0.8}}
b_rim = pile{{"yellow ochre",1.5},{"lead white",2.5},{"raw umber",0.4}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}

m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
local ztB = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(28, 3, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
rimlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(7, 0, y - (530 + 54 * math.sqrt(1 - u * u))) * smoothstep(0.25, -0.5, (x - 596) / 150)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(22, 8, (566 + 92 * math.sqrt(1 - u * u)) - y) * smoothstep(0.45, -0.5, (x - 596) / 150)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=1450}))
print(work(m_bw * ztB(0.12, -0.24), {hand="body", pile=b_lt, coverage=1.0, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1451}))
print(work(m_bw * ztB(0.38, 0.68), {hand="body", pile=b_sh, coverage=1.0, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1452}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=0.9, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1453}))
print(work(botlit, {hand="body", pile=b_lt, coverage=0.5, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1454}))
print(blend(m_bw, {angle=0.2}))
print(work(rimlit, {hand="body", pile=b_rim, coverage=0.8, tool="filbert 7", angle=-0.3, clip=true, fill=true, edge="soft", seed=1455}))

function apple4(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  local rf = mf * mask(function(x, y)
    local dy = (y - cy) / ry
    return smoothstep(0.78, 1.18, d(x, y)) * smoothstep(0.28, 0.76, dy)
  end)
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.24, 0.56), {hand="body", pile=c_sh, coverage=1.0, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(0.58, 0.95), {hand="body", pile=c_sh, coverage=0.85, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.10, -0.10), {hand="body", pile=c_lt, coverage=0.9, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(-0.22, -0.48), {hand="body", pile=c_hot, coverage=0.75, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+5})
  work(rf, {hand="body", pile=c_rf, coverage=0.55, tool="filbert 8", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+6})
  local cav = ellipse(cx - 0.08 * rx, cy - 0.85 * ry, 0.28 * rx, 0.13 * ry):soften(9) * mf
  work(cav, {hand="body", pile=c_sh, coverage=0.85, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+7})
  blend(mf:shrink(8), {angle=-0.6})
end
function catchlight(cx, cy, rx, ry, pile)
  local hb = brush{kind="round", width=7, point=1, stiffness=0.3}
  hb:load(pile, 1.0)
  hb:touch(cx - 0.40 * rx, cy - 0.44 * ry, {pressure=0.8})
  hb:wipe(0.45)
  hb:touch(cx - 0.50 * rx, cy - 0.14 * ry, {pressure=0.35})
end
function stem(cx, cy, rx, ry)
  local sb = brush{kind="round", width=7, point=1, stiffness=0.5}
  sb:load(stem_p, 1.0)
  sb:stroke({{cx - 0.07 * rx, cy - 0.82 * ry}, {cx - 0.15 * rx, cy - 0.98 * ry}},
            {pressure={1.0, 0.55}, ramps={0.06, 0.45}, shake=1})
end
apple4(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1460)
apple4(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1480)
apple4(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, b_lt, 1500)
apple4(m_apD, 300, 690, 92, 86, r_mid, r_lt, r_hot, r_sh, cl_mid, 1520)
apple4(m_apE, 478, 736, 72, 66, gr_mid, gr_lt, gr_hot, gr_sh, cl_mid, 1540)
local fruitq = {{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot},{300,690,92,86,r_hot},{478,736,72,66,gr_hot}}
for _, q in ipairs(fruitq) do stem(q[1], q[2], q[3], q[4]) end
for _, q in ipairs(fruitq) do catchlight(q[1], q[2], q[3], q[4], q[5]) end

--@ chunk 179
gr_mid = pile{{"green earth",5},{"yellow ochre",1.6},{"raw umber",0.8}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2.2},{"chrome yellow",0.6},{"lead white",0.6}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.6},{"bone black",1.8},{"Prussian blue",0.4}}
r_sh  = pile{{"raw umber",3},{"red earth",1.4},{"bone black",1.6},{"Prussian blue",0.3}}
b_sh  = pile{{"raw umber",3},{"bone black",2.2},{"red earth",0.8}}
cl_tr  = pile{{"lead white",1.2},{"raw umber",3.5},{"bone black",1.6},{"cobalt blue",0.6},{"red earth",0.4}}
function deepen(m, cx, cy, rx, ry, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  work(mf * zt(0.20, 0.54), {hand="body", pile=c_sh, coverage=1.2, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+1})
  work(mf * zt(0.56, 0.95), {hand="body", pile=c_sh, coverage=1.0, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+2})
  local base = ellipse(cx + 0.04 * rx, cy + 0.88 * ry, 0.74 * rx, 0.20 * ry):soften(11) * mf
  work(base, {hand="body", pile=c_sh, coverage=0.75, tool="filbert 9", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+3})
  blend(mf:shrink(8), {angle=-0.6})
end
deepen(m_apA, 512, 516, 86, 80, r_sh, b_lt, 1560)
deepen(m_apC, 696, 520, 80, 75, r_sh, b_lt, 1570)
deepen(m_apB, 606, 498, 92, 86, gr_sh, b_lt, 1580)
deepen(m_apD, 300, 690, 92, 86, r_sh, cl_mid, 1590)
deepen(m_apE, 478, 736, 72, 66, gr_sh, cl_mid, 1600)
-- deepen the bowl's shadow side and the shadow under its rim
m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
work(m_bw * mask(function(x, y) return smoothstep(0.34, 0.66, dBowl(x, y)) end),
     {hand="body", pile=b_sh, coverage=1.3, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1610})
work(m_bw * mask(function(x, y)
      local u = clamp((x - 596) / 158, -1, 1)
      return smoothstep(26, 3, y - (530 + 54 * math.sqrt(1 - u * u)))
    end), {hand="body", pile=b_sh, coverage=1.1, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1611})
blend(m_bw, {angle=0.2})
-- and the group's shadow on the cloth
grpsh = ellipse(700, 664, 205, 58):soften(40) + ellipse(420, 706, 108, 32):soften(26)
       + ellipse(560, 748, 88, 28):soften(22)
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
print(stipple(grpsh * m_cl, {pile=cl_tr, width=8, coverage=1.3, feather=0.5, cluster=0.3, clip=true, seed=1612}))

--@ chunk 180
m_grp = m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl
print(blend(m_grp, {angle=0.35, tool="filbert 30"}))
print(blend(m_grp, {angle=-0.9, tool="filbert 20"}))

--@ chunk 181
r_mid = pile{{"vermilion",2.6},{"red earth",3},{"raw umber",1.2}}
r_lt  = pile{{"vermilion",2},{"red earth",1.6},{"yellow ochre",1.6},{"lead white",1}}
r_hot = pile{{"vermilion",1.2},{"yellow ochre",1.6},{"lead white",2.4}}
r_sh  = pile{{"raw umber",3},{"red earth",1.4},{"bone black",1.6},{"Prussian blue",0.3}}
gr_mid = pile{{"green earth",5},{"yellow ochre",1.2},{"raw umber",1}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2.2},{"chrome yellow",0.6},{"lead white",0.6}}
gr_hot = pile{{"chrome yellow",1},{"lead white",2.6},{"yellow ochre",1.6}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.6},{"bone black",1.8},{"Prussian blue",0.4}}
b_mid = pile{{"raw umber",3.2},{"red earth",2},{"yellow ochre",1.4},{"bone black",0.8}}
b_lt  = pile{{"yellow ochre",2.2},{"red earth",1.6},{"lead white",0.9}}
b_sh  = pile{{"raw umber",3},{"bone black",2.2},{"red earth",0.8}}
b_rim = pile{{"yellow ochre",1.5},{"lead white",2.5},{"raw umber",0.4}}

m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
local ztB = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=1620}))
print(work(m_bw * ztB(0.10, -0.26), {hand="body", pile=b_lt, coverage=0.7, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1621}))
print(work(m_bw * ztB(0.24, 0.56), {hand="body", pile=b_sh, coverage=0.6, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1622}))
print(work(m_bw * ztB(0.52, 0.84), {hand="body", pile=b_sh, coverage=0.5, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=1623}))
print(work(m_bw * mask(function(x, y)
      local u = clamp((x - 596) / 158, -1, 1)
      return smoothstep(26, 3, y - (530 + 54 * math.sqrt(1 - u * u)))
    end), {hand="body", pile=b_sh, coverage=0.7, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1624}))
print(blend(m_bw:shrink(8), {angle=0.2}))

function apple5(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.14, -0.06), {hand="body", pile=c_lt, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(-0.18, -0.44), {hand="body", pile=c_hot, coverage=0.6, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.16, 0.48), {hand="body", pile=c_sh, coverage=0.6, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(0.44, 0.78), {hand="body", pile=c_sh, coverage=0.5, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+5})
  work(mf * zt(0.74, 1.02), {hand="body", pile=c_sh, coverage=0.45, tool="filbert 9", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+6})
  work(mf * mask(function(x, y)
        local dy = (y - cy) / ry
        return smoothstep(0.78, 1.18, d(x, y)) * smoothstep(0.28, 0.76, dy)
      end), {hand="body", pile=c_rf, coverage=0.5, tool="filbert 8", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+7})
  work(ellipse(cx - 0.08 * rx, cy - 0.85 * ry, 0.28 * rx, 0.13 * ry):soften(9) * mf,
       {hand="body", pile=c_sh, coverage=0.8, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+8})
  blend(mf:shrink(12), {angle=-0.6})
end
apple5(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1630)
apple5(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1650)
apple5(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, b_lt, 1670)
apple5(m_apD, 300, 690, 92, 86, r_mid, r_lt, r_hot, r_sh, cl_mid, 1690)
apple5(m_apE, 478, 736, 72, 66, gr_mid, gr_lt, gr_hot, gr_sh, cl_mid, 1710)

--@ chunk 182
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_hi = pile{{"lead white",4.5},{"raw umber",2.6},{"yellow ochre",0.7}}
cl_lo = pile{{"raw umber",2.4},{"bone black",1.6},{"lead white",0.6},{"cobalt blue",0.4}}
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
litA = mask(function(x, y) return smoothstep(1.30, 0.05, (x - 150) / 480 + (y - 480) / 300) end)
litB = mask(function(x, y) return smoothstep(0.80, -0.20, (x - 150) / 480 + (y - 480) / 300) end)
T = ribbon({{300,560},{240,640},{180,730},{150,800}}, 30):soften(20)
   + ribbon({{400,540},{350,630},{300,720},{270,800}}, 26):soften(18)
   + ribbon({{500,530},{470,620},{440,720},{420,800}}, 24):soften(18)
   + ribbon({{680,530},{720,620},{760,720},{790,800}}, 28):soften(20)
   + ribbon({{790,560},{830,650},{860,740},{880,800}}, 26):soften(20)
K = ribbon({{340,556},{285,640},{230,730},{205,800}}, 26):soften(18)
   + ribbon({{445,536},{400,628},{355,718},{330,800}}, 24):soften(16)
   + ribbon({{560,528},{545,618},{520,718},{500,800}}, 24):soften(16)
   + ribbon({{730,540},{775,628},{812,722},{838,800}}, 26):soften(18)
grpsh = ellipse(700, 664, 205, 58):soften(40) + ellipse(420, 706, 108, 32):soften(26)
       + ellipse(560, 748, 88, 28):soften(22)
print(stipple(m_cl * litA, {pile=cl_hi, width=9, coverage=0.75, feather=0.5, cluster=0.3, clip=true, seed=1730}))
print(stipple(m_cl * litB, {pile=cl_hi, width=8, coverage=0.55, feather=0.5, cluster=0.3, clip=true, seed=1731}))
print(stipple(T * m_cl, {pile=cl_lo, width=7, coverage=0.85, feather=0.55, cluster=0.35, clip=true, seed=1732}))
print(stipple(K * m_cl, {pile=cl_hi, width=7, coverage=0.6, feather=0.55, cluster=0.35, clip=true, seed=1733}))
print(stipple(grpsh * m_cl, {pile=cl_lo, width=8, coverage=1.1, feather=0.5, cluster=0.3, clip=true, seed=1734}))
print(stipple(rect(620, 0, 380, 170), {pile=p_bgd, width=10, coverage=3.0, clip=true, seed=1735}))

--@ chunk 183
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
cl_hi = pile{{"lead white",4.5},{"raw umber",2.6},{"yellow ochre",0.7}}
cl_lo = pile{{"raw umber",2.2},{"bone black",2},{"yellow ochre",0.4}}
T = ribbon({{310,556},{250,640},{190,730},{160,800}}, 38):soften(26)
   + ribbon({{420,536},{368,628},{318,718},{288,800}}, 34):soften(24)
   + ribbon({{520,528},{492,618},{466,718},{446,800}}, 32):soften(22)
   + ribbon({{672,528},{712,618},{752,718},{782,800}}, 36):soften(24)
   + ribbon({{782,556},{824,646},{856,740},{876,800}}, 34):soften(24)
K = ribbon({{356,552},{300,636},{244,726},{218,800}}, 34):soften(24)
   + ribbon({{466,532},{420,624},{374,714},{349,800}}, 32):soften(22)
   + ribbon({{578,526},{562,616},{538,716},{518,800}}, 32):soften(22)
   + ribbon({{726,538},{770,626},{806,720},{832,800}}, 34):soften(24)
   + ribbon({{120,626},{80,700},{60,760}}, 30):soften(22)
print(stipple(T * m_cl, {pile=cl_lo, width=8, coverage=2.2, feather=0.5, cluster=0.35, clip=true, seed=1740}))
print(stipple(K * m_cl, {pile=cl_hi, width=8, coverage=1.5, feather=0.5, cluster=0.35, clip=true, seed=1741}))

--@ chunk 184
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
print(work(rect(660, 0, 340, 130), {hand="body", pile=p_bgd, coverage=6.0, tool="filbert 18", angle=0.2,
  clip=true, fill=true, edge="found", seed=1750}))
print(work(rect(660, 0, 340, 130), {hand="body", pile=p_bgd, coverage=4.0, tool="filbert 12", angle=0.7,
  clip=true, fill=true, edge="found", seed=1751}))

b_rim = pile{{"yellow ochre",1.5},{"lead white",2.5},{"raw umber",0.4}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}
r_hot = pile{{"vermilion",1.2},{"yellow ochre",1.6},{"lead white",2.4}}
gr_hot = pile{{"chrome yellow",1},{"lead white",2.6},{"yellow ochre",1.6}}
m_bw = m_bowl:grow(2)
rimlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(8, 1, y - (530 + 54 * math.sqrt(1 - u * u))) * smoothstep(0.10, -0.55, (x - 596) / 150)
end)
print(work(rimlit, {hand="body", pile=b_rim, coverage=0.9, tool="filbert 7", angle=-0.3, clip=true, fill=true, edge="soft", seed=1752}))

local fruitq = {{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot},
                {300,690,92,86,r_hot},{478,736,72,66,gr_hot}}
for _, q in ipairs(fruitq) do
  local sb = brush{kind="round", width=6, point=1, stiffness=0.5}
  sb:load(stem_p, 1.0)
  sb:stroke({{q[1] - 0.08 * q[3], q[2] - 0.86 * q[4]},
              {q[1] - 0.16 * q[3], q[2] - 1.02 * q[4]},
              {q[1] - 0.21 * q[3], q[2] - 1.16 * q[4]}},
            {pressure={1.0, 0.7, 0.2}, ramps={0.06, 0.45}, shake=1})
end
-- catchlights: one bright touch on the light side of each apple, one weaker below it
for _, q in ipairs(fruitq) do
  local hb = brush{kind="round", width=8, point=1, stiffness=0.3}
  hb:load(q[5], 1.0)
  hb:touch(q[1] - 0.38 * q[3], q[2] - 0.42 * q[4], {pressure=0.75})
  hb:wipe(0.4)
  hb:touch(q[1] - 0.48 * q[3], q[2] - 0.10 * q[4], {pressure=0.32})
end
-- a few broken reflected lights in the shadow side, to keep the darks alive
local bounce = pile{{"lead white",2},{"red earth",1.4},{"raw umber",0.8}}
for _, q in ipairs{{512,516,86,80},{696,520,80,75},{606,498,92,86},{300,690,92,86},{478,736,72,66}} do
  local rb = brush{kind="round", width=9, point=1, stiffness=0.35}
  rb:load(bounce, 0.75)
  rb:stroke({{q[1] + 0.30 * q[3], q[2] + 0.52 * q[4]}, {q[1] + 0.46 * q[3], q[2] + 0.36 * q[4]}},
            {pressure={0.55, 0.15}, ramps={0.2, 0.6}, shake=1})
end

--@ chunk 185
r_sh  = pile{{"raw umber",3},{"red earth",1.4},{"bone black",1.6},{"Prussian blue",0.3}}
gr_sh = pile{{"raw umber",3},{"green earth",1.6},{"bone black",1.8},{"Prussian blue",0.4}}
b_sh  = pile{{"raw umber",3},{"bone black",2.2},{"red earth",0.8}}
b_mid = pile{{"raw umber",3.2},{"red earth",2},{"yellow ochre",1.4},{"bone black",0.8}}
b_lt  = pile{{"yellow ochre",2.2},{"red earth",1.6},{"lead white",0.9}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}

-- 1. bury the white bounce slashes: repaint the shadow side of every apple, then fuse
local fruitq = {{512,516,86,80,r_sh,m_apA,1},{696,520,80,75,r_sh,m_apC,1},
                {606,498,92,86,gr_sh,m_apB,1},{300,690,92,86,r_sh,m_apD,0},
                {478,736,72,66,gr_sh,m_apE,0}}
for _, q in ipairs(fruitq) do
  local cx, cy, rx, ry, c_sh, m, oncloth = q[1], q[2], q[3], q[4], q[5], q[6], q[7]
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  work(mf * zt(0.40, 0.80), {hand="body", pile=c_sh, coverage=1.2, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1760 + cx})
  work(mf * zt(0.70, 1.05), {hand="body", pile=c_sh, coverage=0.9, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=1770 + cx})
  blend(mf:shrink(12), {angle=-0.6})
end
-- 2. the beaded rim light becomes the rim's own shadow
m_bw = m_bowl:grow(2)
rimband = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(13, 1, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
print(work(rimband, {hand="body", pile=b_sh, coverage=1.3, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1780}))
-- 3. stems: take the hairs away, redraw short and thick
for _, q in ipairs({{512,516,86,80,p_bgd},{696,520,80,75,p_bgd},{606,498,92,86,p_bgd},
                    {300,690,92,86,cl_mid},{478,736,72,66,cl_mid}}) do
  local cx, cy, rx, ry, sur = q[1], q[2], q[3], q[4], q[5]
  local band = ellipse(cx - 0.14 * rx, cy - 1.12 * ry, 0.30 * rx, 0.30 * ry)
  print(work(band, {hand="body", pile=sur, coverage=2.4, tool="filbert 10", clip=true, fill=true, edge="soft", seed=1790 + cx}))
end
for _, q in ipairs({{512,516,86,80},{696,520,80,75},{606,498,92,86},{300,690,92,86},{478,736,72,66}}) do
  local sb = brush{kind="round", width=9, point=1, stiffness=0.55}
  sb:load(stem_p, 1.0)
  sb:stroke({{q[1] - 0.07 * q[3], q[2] - 0.87 * q[4]},
              {q[1] - 0.15 * q[3], q[2] - 1.00 * q[4]},
              {q[1] - 0.19 * q[3], q[2] - 1.09 * q[4]}},
            {pressure={1.0, 0.8, 0.4}, ramps={0.06, 0.4}, shake=1})
end
-- 4. the bowl gets a form: shadow right, reflected light low-left, foot shadow
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
print(work(m_bw * mask(function(x, y) return smoothstep(0.30, 0.62, dBowl(x, y)) end),
      {hand="body", pile=b_sh, coverage=0.9, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1810}))
print(work(m_bw * mask(function(x, y)
        local u = clamp((x - 596) / 160, -1, 1)
        return smoothstep(20, 7, (566 + 92 * math.sqrt(1 - u * u)) - y) * smoothstep(0.40, -0.55, (x - 596) / 150)
      end), {hand="body", pile=b_lt, coverage=0.6, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1811}))
print(work(m_bw * mask(function(x, y)
        local u = clamp((x - 596) / 160, -1, 1)
        return smoothstep(16, 3, (566 + 92 * math.sqrt(1 - u * u)) - y)
      end), {hand="body", pile=b_sh, coverage=0.8, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1812}))
print(blend(m_bw:shrink(8), {angle=0.2}))

--@ chunk 186
r_mid = pile{{"vermilion",2.4},{"red earth",3.2},{"raw umber",1.3}}
r_lt  = pile{{"vermilion",1.8},{"red earth",2},{"yellow ochre",1.6},{"lead white",0.9}}
r_hot = pile{{"vermilion",1.1},{"yellow ochre",1.7},{"lead white",2.3}}
r_sh  = pile{{"raw umber",3},{"red earth",1.5},{"bone black",1.5},{"Prussian blue",0.3}}
gr_mid = pile{{"green earth",5},{"yellow ochre",1.3},{"raw umber",1}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2.2},{"chrome yellow",0.5},{"lead white",0.6}}
gr_hot = pile{{"chrome yellow",0.9},{"lead white",2.6},{"yellow ochre",1.6}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.7},{"bone black",1.7},{"Prussian blue",0.4}}
b_mid = pile{{"raw umber",3.2},{"red earth",2},{"yellow ochre",1.4},{"bone black",0.8}}
b_lt  = pile{{"yellow ochre",2.2},{"red earth",1.6},{"lead white",0.9}}
b_sh  = pile{{"raw umber",3},{"bone black",2.2},{"red earth",0.8}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}

-- ---- the bowl first, so the fruit covers it where they overlap
m_bw = m_bowl:grow(2)
dBowl = function(x, y) return 0.78 * (x - 596) / 176 + 0.62 * (y - 604) / 78 end
local ztB = function(a, b) return mask(function(x, y) return smoothstep(a, b, dBowl(x, y)) end) end
rimshade = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 158, -1, 1)
  return smoothstep(11, 1, y - (530 + 54 * math.sqrt(1 - u * u)))
end)
footsh = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(15, 3, (566 + 92 * math.sqrt(1 - u * u)) - y)
end)
botlit = m_bw * mask(function(x, y)
  local u = clamp((x - 596) / 160, -1, 1)
  return smoothstep(26, 9, (566 + 92 * math.sqrt(1 - u * u)) - y) * smoothstep(0.42, -0.55, (x - 596) / 150)
end)
print(work(m_bw, {hand="body", pile=b_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=1820}))
print(work(m_bw * ztB(0.10, -0.26), {hand="body", pile=b_lt, coverage=0.7, tool="filbert 12", angle=-0.5, clip=true, fill=true, edge="soft", seed=1821}))
print(work(m_bw * ztB(0.24, 0.56), {hand="body", pile=b_sh, coverage=0.6, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=1822}))
print(work(m_bw * ztB(0.52, 0.84), {hand="body", pile=b_sh, coverage=0.5, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=1823}))
print(work(rimshade, {hand="body", pile=b_sh, coverage=1.1, tool="filbert 8", angle=0.1, clip=true, fill=true, edge="soft", seed=1824}))
print(work(botlit, {hand="body", pile=b_lt, coverage=0.55, tool="filbert 10", angle=0.1, clip=true, fill=true, edge="soft", seed=1825}))
print(work(footsh, {hand="body", pile=b_sh, coverage=0.7, tool="filbert 9", angle=0.1, clip=true, fill=true, edge="soft", seed=1826}))
print(blend(m_bw:shrink(8), {angle=0.2}))

-- ---- the fruit
function apple6(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.14, -0.06), {hand="body", pile=c_lt, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(-0.18, -0.44), {hand="body", pile=c_hot, coverage=0.55, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.16, 0.48), {hand="body", pile=c_sh, coverage=0.55, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(0.44, 0.78), {hand="body", pile=c_sh, coverage=0.5, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+5})
  work(mf * zt(0.74, 1.02), {hand="body", pile=c_sh, coverage=0.45, tool="filbert 9", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+6})
  work(mf * mask(function(x, y)
        local dy = (y - cy) / ry
        return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
      end), {hand="body", pile=c_rf, coverage=0.45, tool="filbert 9", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+7})
  work(ellipse(cx - 0.08 * rx, cy - 0.84 * ry, 0.26 * rx, 0.12 * ry):soften(9) * mf,
       {hand="body", pile=c_sh, coverage=0.8, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+8})
  blend(mf:shrink(12), {angle=-0.6})
end
apple6(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1830)
apple6(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1850)
apple6(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, b_lt, 1870)
apple6(m_apD, 300, 690, 92, 86, r_mid, r_lt, r_hot, r_sh, cl_mid, 1890)
apple6(m_apE, 478, 736, 72, 66, gr_mid, gr_lt, gr_hot, gr_sh, cl_mid, 1910)
-- stems and catchlights, and nothing else
local fruitq = {{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot},
                {300,690,92,86,r_hot},{478,736,72,66,gr_hot}}
for _, q in ipairs(fruitq) do
  local sb = brush{kind="round", width=9, point=1, stiffness=0.55}
  sb:load(stem_p, 1.0)
  sb:stroke({{q[1] - 0.07 * q[3], q[2] - 0.86 * q[4]},
              {q[1] - 0.15 * q[3], q[2] - 1.00 * q[4]},
              {q[1] - 0.19 * q[3], q[2] - 1.10 * q[4]}},
            {pressure={1.0, 0.8, 0.35}, ramps={0.06, 0.4}, shake=1})
end
for _, q in ipairs(fruitq) do
  local hb = brush{kind="round", width=8, point=1, stiffness=0.3}
  hb:load(q[5], 1.0)
  hb:touch(q[1] - 0.38 * q[3], q[2] - 0.42 * q[4], {pressure=0.7})
  hb:wipe(0.4)
  hb:touch(q[1] - 0.48 * q[3], q[2] - 0.10 * q[4], {pressure=0.28})
end

--@ chunk 187
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
bounce_w = pile{{"raw umber",3.6},{"red earth",2.2},{"bone black",2.6}}
cl_hi = pile{{"lead white",4.5},{"raw umber",2.6},{"yellow ochre",0.7}}
-- warm light thrown back onto the wall by the bowl and the fruit
bw = ellipse(600, 430, 300, 150):soften(110)
bw2 = ellipse(600, 400, 190, 100):soften(80)
print(stipple(bw, {pile=bounce_w, width=9, coverage=0.7, feather=0.55, cluster=0.3, clip=true, seed=1930}))
print(stipple(bw2, {pile=bounce_w, width=8, coverage=0.45, feather=0.55, cluster=0.3, clip=true, seed=1931}))
-- and the light reaching across the cloth to the left of the group
cl1 = ellipse(330, 640, 210, 110):soften(80) * m_cl
cl2 = ellipse(280, 660, 130, 70):soften(60) * m_cl
print(stipple(cl1, {pile=cl_hi, width=9, coverage=0.6, feather=0.55, cluster=0.3, clip=true, seed=1932}))
print(stipple(cl2, {pile=cl_hi, width=8, coverage=0.45, feather=0.55, cluster=0.3, clip=true, seed=1933}))

--@ chunk 188
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
print(stipple(ellipse(600, 420, 340, 180):soften(40), {pile=p_bgd, width=10, coverage=3.0, clip=true, seed=1940}))
print(stipple(rect(240, 250, 700, 260), {pile=p_bgd, width=10, coverage=2.0, clip=true, seed=1941}))

--@ chunk 189
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
p_bgw = pile{{"raw umber",4},{"bone black",3.3},{"red earth",1.3},{"Prussian blue",0.6}}
print(work(rect(200, 170, 800, 340), {hand="broad", pile=p_bgd, tool="filbert 22", angle=0.22,
  coverage=3.5, clip=true, fill=true, edge="soft", seed=1950}))
print(work(rect(200, 170, 800, 340), {hand="broad", pile=p_bgd, tool="filbert 18", angle=0.8,
  coverage=2.0, clip=true, fill=true, edge="soft", seed=1951}))
print(work(ellipse(220, 120, 300, 210):soften(120), {hand="broad", pile=p_bgw, tool="filbert 20", angle=0.3, coverage=0.8, clip=true, fill=true, edge="soft", seed=1952}))
print(work(ellipse(190, 100, 180, 140):soften(90), {hand="broad", pile=p_bgw, tool="filbert 18", angle=0.3, coverage=0.6, clip=true, fill=true, edge="soft", seed=1953}))

--@ chunk 190
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
p_tab = pile{{"raw umber",4},{"red earth",2},{"yellow ochre",1},{"bone black",1}}
p_bgw = pile{{"raw umber",4},{"bone black",3.3},{"red earth",1.3},{"Prussian blue",0.6}}
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
m_group = m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl
m_bg = m_wall - m_group - m_cloth
m_tabarea = m_table - m_cl - m_group
print(work(m_bg, {hand="broad", pile=p_bgd, tool="filbert 22", angle=0.22, coverage=2.6, clip=true, fill=true, edge="soft", seed=1960}))
print(work(m_tabarea, {hand="broad", pile=p_tab, tool="filbert 20", angle=0.15, coverage=2.6, clip=true, fill=true, edge="soft", seed=1961}))
print(work(m_bg * ellipse(230, 120, 300, 210):soften(120), {hand="broad", pile=p_bgw, tool="filbert 20", angle=0.3, coverage=0.8, clip=true, fill=true, edge="soft", seed=1962}))
print(work(m_bg * ellipse(195, 100, 175, 135):soften(90), {hand="broad", pile=p_bgw, tool="filbert 18", angle=0.3, coverage=0.6, clip=true, fill=true, edge="soft", seed=1963}))

r_mid = pile{{"vermilion",2.4},{"red earth",3.2},{"raw umber",1.3}}
r_lt  = pile{{"vermilion",1.8},{"red earth",2},{"yellow ochre",1.6},{"lead white",0.9}}
r_hot = pile{{"vermilion",1.1},{"yellow ochre",1.7},{"lead white",2.3}}
r_sh  = pile{{"raw umber",3},{"red earth",1.5},{"bone black",1.5},{"Prussian blue",0.3}}
gr_mid = pile{{"green earth",5},{"yellow ochre",1.3},{"raw umber",1}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2.2},{"chrome yellow",0.5},{"lead white",0.6}}
gr_hot = pile{{"chrome yellow",0.9},{"lead white",2.6},{"yellow ochre",1.6}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.7},{"bone black",1.7},{"Prussian blue",0.4}}
b_mid = pile{{"raw umber",3.2},{"red earth",2},{"yellow ochre",1.4},{"bone black",0.8}}
b_lt  = pile{{"yellow ochre",2.2},{"red earth",1.6},{"lead white",0.9}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}
function apple6(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.14, -0.06), {hand="body", pile=c_lt, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(-0.18, -0.44), {hand="body", pile=c_hot, coverage=0.55, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.16, 0.48), {hand="body", pile=c_sh, coverage=0.55, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(0.44, 0.78), {hand="body", pile=c_sh, coverage=0.5, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+5})
  work(mf * zt(0.74, 1.02), {hand="body", pile=c_sh, coverage=0.45, tool="filbert 9", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+6})
  work(mf * mask(function(x, y)
        local dy = (y - cy) / ry
        return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
      end), {hand="body", pile=c_rf, coverage=0.45, tool="filbert 9", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+7})
  work(ellipse(cx - 0.08 * rx, cy - 0.84 * ry, 0.26 * rx, 0.12 * ry):soften(9) * mf,
       {hand="body", pile=c_sh, coverage=0.8, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+8})
  blend(mf:shrink(12), {angle=-0.6})
end
apple6(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 1970)
apple6(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 1990)
apple6(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, b_lt, 2010)
local fq = {{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot}}
for _, q in ipairs(fq) do
  local sb = brush{kind="round", width=9, point=1, stiffness=0.55}
  sb:load(stem_p, 1.0)
  sb:stroke({{q[1] - 0.07*q[3], q[2] - 0.86*q[4]}, {q[1] - 0.15*q[3], q[2] - 1.00*q[4]},
              {q[1] - 0.19*q[3], q[2] - 1.10*q[4]}},
            {pressure={1.0, 0.8, 0.35}, ramps={0.06, 0.4}, shake=1})
end
for _, q in ipairs(fq) do
  local hb = brush{kind="round", width=8, point=1, stiffness=0.3}
  hb:load(q[5], 1.0)
  hb:touch(q[1] - 0.38*q[3], q[2] - 0.42*q[4], {pressure=0.7})
  hb:wipe(0.4)
  hb:touch(q[1] - 0.48*q[3], q[2] - 0.10*q[4], {pressure=0.28})
end

--@ chunk 191
tab_d = pile{{"raw umber",3.6},{"bone black",3.4},{"red earth",1.2},{"Prussian blue",0.4}}
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
m_group = m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl
m_tabarea = m_table - m_cl - m_group
print(stipple(m_tabarea, {pile=tab_d, width=10, coverage=3.5, clip=true, seed=2020}))
print(stipple(m_tabarea, {pile=tab_d, width=9, coverage=2.0, clip=true, seed=2021}))

--@ chunk 192
tab_d = pile{{"raw umber",3.6},{"bone black",3.4},{"red earth",1.2},{"Prussian blue",0.4}}
p_bgd = pile{{"raw umber",4},{"bone black",4},{"Prussian blue",0.7}}
m_cl = poly(cpts5, true):roughen(6, 40, 11):soften(2)
m_group = m_apA + m_apB + m_apC + m_apD + m_apE + m_bowl
m_tabarea = m_table - m_cl - m_group
m_bg = m_wall - m_group - m_cloth
print(work(m_tabarea, {hand="broad", pile=tab_d, tool="filbert 24", angle=0.10, coverage=1.6, clip=true, fill=true, edge="soft", seed=2030}))
print(work(m_bg, {hand="broad", pile=p_bgd, tool="filbert 24", angle=0.30, coverage=1.2, clip=true, fill=true, edge="soft", seed=2031}))

--@ chunk 193
r_mid = pile{{"vermilion",2.4},{"red earth",3.2},{"raw umber",1.3}}
r_lt  = pile{{"vermilion",1.8},{"red earth",2},{"yellow ochre",1.6},{"lead white",0.9}}
r_hot = pile{{"vermilion",1.1},{"yellow ochre",1.7},{"lead white",2.3}}
r_sh  = pile{{"raw umber",3},{"red earth",1.5},{"bone black",1.5},{"Prussian blue",0.3}}
gr_mid = pile{{"green earth",5},{"yellow ochre",1.3},{"raw umber",1}}
gr_lt  = pile{{"green earth",3},{"yellow ochre",2.2},{"chrome yellow",0.5},{"lead white",0.6}}
gr_hot = pile{{"chrome yellow",0.9},{"lead white",2.6},{"yellow ochre",1.6}}
gr_sh  = pile{{"raw umber",3},{"green earth",1.7},{"bone black",1.7},{"Prussian blue",0.4}}
b_lt  = pile{{"yellow ochre",2.2},{"red earth",1.6},{"lead white",0.9}}
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}
stem_p = pile{{"raw umber",4},{"bone black",2.2},{"red earth",0.8}}
function apple7(m, cx, cy, rx, ry, c_mid, c_lt, c_hot, c_sh, c_rf, sd)
  local mf = m:grow(1.5)
  local d = function(x, y)
    local dx = (x - cx) / rx
    local dy = (y - cy) / ry
    return 0.80 * dx + 0.58 * dy + 0.20 * (dx * dx + dy * dy)
  end
  local zt = function(a, b) return mask(function(x, y) return smoothstep(a, b, d(x, y)) end) end
  work(mf, {hand="body", pile=c_mid, coverage=3.4, tool="filbert 14", clip=true, fill=true, edge="found", seed=sd+1})
  work(mf * zt(0.14, -0.06), {hand="body", pile=c_lt, coverage=0.7, tool="filbert 12", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+2})
  work(mf * zt(-0.18, -0.44), {hand="body", pile=c_hot, coverage=0.55, tool="filbert 9", angle=-0.7, clip=true, fill=true, edge="soft", seed=sd+3})
  work(mf * zt(0.16, 0.48), {hand="body", pile=c_sh, coverage=0.55, tool="filbert 12", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+4})
  work(mf * zt(0.44, 0.78), {hand="body", pile=c_sh, coverage=0.45, tool="filbert 10", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+5})
  work(mf * zt(0.74, 1.02), {hand="body", pile=c_sh, coverage=0.35, tool="filbert 9", angle=0.5, clip=true, fill=true, edge="soft", seed=sd+6})
  work(mf * mask(function(x, y)
        local dy = (y - cy) / ry
        return smoothstep(0.80, 1.20, d(x, y)) * smoothstep(0.30, 0.78, dy)
      end), {hand="body", pile=c_rf, coverage=0.45, tool="filbert 9", angle=0.2, clip=true, fill=true, edge="soft", seed=sd+7})
  work(ellipse(cx - 0.08 * rx, cy - 0.84 * ry, 0.26 * rx, 0.12 * ry):soften(9) * mf,
       {hand="body", pile=c_sh, coverage=0.8, tool="filbert 8", clip=true, fill=true, edge="soft", seed=sd+8})
  blend((mf:shrink(3)):soften(3), {angle=-0.6})
end
apple7(m_apA, 512, 516, 86, 80, r_mid, r_lt, r_hot, r_sh, b_lt, 2040)
apple7(m_apC, 696, 520, 80, 75, r_mid, r_lt, r_hot, r_sh, b_lt, 2060)
apple7(m_apB, 606, 498, 92, 86, gr_mid, gr_lt, gr_hot, gr_sh, b_lt, 2080)
apple7(m_apD, 300, 690, 92, 86, r_mid, r_lt, r_hot, r_sh, cl_mid, 2100)
apple7(m_apE, 478, 736, 72, 66, gr_mid, gr_lt, gr_hot, gr_sh, cl_mid, 2120)
local fq = {{512,516,86,80,r_hot},{696,520,80,75,r_hot},{606,498,92,86,gr_hot},
            {300,690,92,86,r_hot},{478,736,72,66,gr_hot}}
for _, q in ipairs(fq) do
  local sb = brush{kind="round", width=10, point=1, stiffness=0.6}
  sb:load(stem_p, 1.0)
  sb:stroke({{q[1] - 0.06*q[3], q[2] - 0.85*q[4]}, {q[1] - 0.13*q[3], q[2] - 0.96*q[4]},
              {q[1] - 0.17*q[3], q[2] - 1.03*q[4]}},
            {pressure={1.0, 0.75, 0.3}, ramps={0.08, 0.4}, shake=1})
end
for _, q in ipairs(fq) do
  local hb = brush{kind="round", width=8, point=1, stiffness=0.3}
  hb:load(q[5], 1.0)
  hb:touch(q[1] - 0.38*q[3], q[2] - 0.42*q[4], {pressure=0.7})
  hb:wipe(0.4)
  hb:touch(q[1] - 0.48*q[3], q[2] - 0.10*q[4], {pressure=0.28})
end

--@ chunk 194
cl_mid = pile{{"raw umber",3},{"bone black",1.5},{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4}}
warm_sh = pile{{"raw umber",2.4},{"bone black",1.6},{"red earth",1}}
for _, q in ipairs{{m_apD, 2}, {m_apE, 2}} do
  local m = q[1]
  print(work(m:grow(7) - m:grow(1.5), {hand="body", pile=cl_mid, coverage=2.6,
        tool="filbert 10", clip=true, fill=true, edge="found", seed=2140 + q[2]}))
end
print(stipple(ellipse(332, 732, 108, 36):soften(26) + ellipse(502, 770, 86, 30):soften(24),
      {pile=warm_sh, width=8, coverage=1.2, feather=0.5, cluster=0.3, clip=true, seed=2150}))
print(stipple(ellipse(306, 766, 70, 18):soften(12) + ellipse(482, 792, 56, 15):soften(10),
      {pile=warm_sh, width=7, coverage=1.0, feather=0.5, cluster=0.3, clip=true, seed=2151}))

--@ chunk 195
print(wait(8 * 60))

--@ chunk (finishing)
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)
