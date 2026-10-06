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

--@ chunk 196
local n=0; for k,v in pairs(_G) do if k:match("^%a") then n=n+1 end end; print("globals:",n); for k,v in pairs(_G) do if type(v)=="table" or k:match("^p_") then print(k, tostring(v)) end end

--@ chunk 197
p_wall  = pile{{"raw umber", 4}, {"bone black", 4}, {"red earth", 0.7}, {"Prussian blue", 0.4}}
p_walll = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1.3}, {"yellow ochre", 0.6}}
p_tbl   = pile{{"raw umber", 5}, {"red earth", 1.6}, {"yellow ochre", 0.6}, {"bone black", 1.2}}
w = everywhere()
work(w, {pile=p_wall, tool="filbert 12", length={26,64}, coverage=2.2,
         dips={4,1.0,0.1}, fill=true, edge="soft", angle=0.9})
print("base laid")

--@ chunk 198
work(everywhere(), {pile=p_wall, hand="broad", coverage=3.0, fill=true,
                    load=1.0, pressure={1.0,0.8}, dips={3,1.0,0.05}, edge="soft"})
print("coat 2")

--@ chunk 199
work(rect(0, 340, 1000, 460), {pile=p_wall, hand="broad", coverage=3.0, fill=true,
      load=1.0, pressure={1.0,1.0}, dips={2,1.0,0.0}, edge="soft"})
print("coat 3")

--@ chunk 200
low = rect(0, 330, 1000, 470)
work(low, {pile=p_wall, tool="flat 20", length={30,70}, coverage=3.5, fill=true,
           load=1.0, pressure={1.0,1.0}, dips={2,1.0,0.0}, edge="soft"})
stipple(low, {pile=p_wall, width=3.2, coverage=2.2, pressure={0.8,0.6}, clip=true})
print("coat 4")

--@ chunk 201
p_reset = pile{{"lead white", 3}, {"raw umber", 5}, {"bone black", 1.5}, {"cobalt blue", 0.4}}
low = rect(0, 330, 1000, 470)
work(low, {pile=p_reset, tool="flat 20", length={30,70}, coverage=3.0, fill=true,
           load=1.0, pressure={1.0,1.0}, dips={2,1.0,0.0}, edge="soft"})
work(low, {pile=p_wall, tool="flat 20", length={30,70}, coverage=2.0, fill=true,
           load=1.0, pressure={1.0,1.0}, dips={2,1.0,0.0}, edge="soft"})
print("reset")

--@ chunk 202
p_walll = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1.3}, {"yellow ochre", 0.6}}
p_glow  = pile{{"raw umber", 3.4}, {"bone black", 1.1}, {"red earth", 1.6}, {"yellow ochre", 1.3}, {"lead white", 0.6}}
p_dk    = pile{{"bone black", 3}, {"raw umber", 1.6}, {"Prussian blue", 0.9}}

pool1 = ellipse(430, 300, 540, 400):soften(200)
pool2 = ellipse(400, 320, 380, 270):soften(150)
pool3 = ellipse(360, 330, 240, 165):soften(110)

work(pool1, {pile=p_walll, tool="flat 20", length={30,70}, coverage=1.6, fill=true,
             load=1.0, pressure={1.0,0.9}, dips={3,1.0,0.05}, edge="soft"})
work(pool2, {pile=p_walll, tool="flat 20", length={26,60}, coverage=1.3, fill=true,
             load=1.0, pressure={1.0,0.9}, dips={3,1.0,0.05}, edge="soft"})
work(pool3, {pile=p_glow,  tool="flat 16", length={22,50}, coverage=1.2, fill=true,
             load=1.0, pressure={1.0,0.9}, dips={3,1.0,0.05}, edge="soft"})

dark_side = rect(600, 0, 400, 470):soften(160)
work(dark_side, {pile=p_dk, tool="flat 20", length={30,70}, coverage=1.2, fill=true,
                 load=1.0, pressure={1.0,0.9}, dips={3,1.0,0.05}, edge="soft"})
print("wall graded")

--@ chunk 203
print(wait(0))

--@ chunk 204
p_wB = pile{{"raw umber", 4}, {"bone black", 3}, {"red earth", 1.2}, {"Prussian blue", 0.5}}
p_wL = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1.6}, {"yellow ochre", 0.9}}
p_wLL= pile{{"raw umber", 3.5}, {"bone black", 1.4}, {"red earth", 1.8}, {"yellow ochre", 1.6}, {"lead white", 0.6}}

wr = rect(0, 0, 1000, 500)
pool1 = ellipse(430, 290, 540, 400):soften(200)
pool2 = ellipse(400, 320, 380, 270):soften(150)
pool3 = ellipse(355, 335, 235, 160):soften(105)

work(wr, {pile=p_wB, hand="broad", coverage=3.5, fill=true, load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
work(pool1, {pile=p_wL, hand="broad", coverage=3.0, fill=true, load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
work(pool2, {pile=p_wL, hand="broad", coverage=2.4, fill=true, load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
work(pool3, {pile=p_wLL, hand="broad", coverage=1.8, fill=true, load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
blend(wr:soften(50), {angle=0.4})
print("wall solid", wait(0))

--@ chunk 205
p_dkc = pile{{"raw umber", 3}, {"bone black", 3.6}, {"Prussian blue", 1.2}, {"cobalt blue", 0.4}}
p_tblB= pile{{"raw umber", 5}, {"red earth", 1.6}, {"yellow ochre", 0.8}, {"bone black", 1.2}}
p_tblL= pile{{"raw umber", 4}, {"red earth", 1.8}, {"yellow ochre", 1.3}, {"bone black", 0.4}}

work(rect(620, 0, 380, 500):soften(180), {pile=p_dkc, hand="broad", coverage=3.0, fill=true,
     load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})

tbl = below(function(x) return 468 - 0.04*x end):soften(7)
work(tbl, {pile=p_tblB, hand="broad", coverage=3.0, fill=true, load=1.0,
           pressure={1.0,0.85}, dips={3,1.0,0.05}, angle=0.25})
work(below(function(x) return 468 - 0.04*x end) * ellipse(300, 640, 520, 300):soften(170),
     {pile=p_tblL, hand="broad", coverage=2.6, fill=true, load=1.0,
      pressure={1.0,0.85}, dips={3,1.0,0.05}, angle=0.25})
print("table", wait(0))

--@ chunk 206
p_deep = pile{{"raw umber", 4}, {"bone black", 2.6}, {"red earth", 0.8}, {"yellow ochre", 0.4}, {"cobalt blue", 0.35}}
work(everywhere(), {pile=p_deep, hand="broad", coverage=3.5, fill=true, load=1.0,
                    pressure={1.0,0.85}, dips={3,1.0,0.05}})
work(everywhere(), {pile=p_deep, hand="broad", coverage=3.0, fill=true, load=1.0,
                    pressure={1.0,0.85}, dips={3,1.0,0.05}})
print("deep field", wait(0))

--@ chunk 207
p_deep2 = pile{{"raw umber", 4}, {"bone black", 3.2}, {"red earth", 0.7}, {"yellow ochre", 0.35}, {"cobalt blue", 0.45}}
work(rect(470, 0, 530, 520):soften(340), {pile=p_deep2, hand="broad", coverage=3.0, fill=true,
     load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
work(rect(520, 400, 480, 400):soften(240), {pile=p_deep2, hand="broad", coverage=3.0, fill=true,
     load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
blend(rect(0, 0, 1000, 500):soften(60), {angle=0.4})
print("wall right", wait(0))

--@ chunk 208
p_tbl  = pile{{"raw umber", 4.5}, {"red earth", 1.8}, {"yellow ochre", 1.0}, {"bone black", 1.0}}
p_tbll = pile{{"raw umber", 4}, {"red earth", 1.6}, {"yellow ochre", 1.5}, {"lead white", 0.5}, {"bone black", 0.4}}
p_corner = pile{{"bone black", 2.4}, {"raw umber", 2}, {"Prussian blue", 0.6}}

tline = function(x) return 468 - 0.04*x end
tbl = below(tline):soften(8)
work(tbl, {pile=p_tbl, hand="broad", coverage=3.2, fill=true, load=1.0,
           pressure={1.0,0.85}, dips={3,1.0,0.05}, angle=0.2})
work(tbl * ellipse(250, 560, 470, 230):soften(160), {pile=p_tbll, hand="broad", coverage=2.4,
     fill=true, load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}, angle=0.2})

-- the corner where the wall meets the table: a soft dark band, a shade under it
work(below(tline) * below(function(x) return tline(x) + 34 end):soften(26), {pile=p_corner,
     hand="broad", coverage=2.4, fill=true, load=1.0, pressure={1.0,0.85}, dips={3,1.0,0.05}})
corner_line = ribbon({{-10, 470}, {250, 462}, {500, 452}, {760, 441}, {1010, 430}}, 5)
work(corner_line, {pile=p_corner, hand="detail", coverage=1.6, fill=true, pressure=0.9, clip=true})
print("table", wait(0))

--@ chunk 209
p_corner = pile{{"bone black", 2.4}, {"raw umber", 2}, {"Prussian blue", 0.6}}
tline = function(x) return 468 - 0.04*x end
tbl = below(tline):soften(8)
work(tbl, {pile=p_tbl, hand="broad", coverage=2.6, fill=true, load=1.0,
           pressure={1.0,0.85}, dips={3,1.0,0.05}, angle=0.2})
-- bury the ruled-looking corner line, then lay it again, soft and broken
work(below(function(x) return tline(x) - 22 end) * above(function(x) return tline(x) + 12 end):soften(14),
     {pile=p_tbl, hand="body", coverage=2.2, fill=true, load=1.0, pressure={1.0,0.9}, dips={3,1.0,0.05}})
c2 = ribbon({{-10, 470}, {180, 465}, {380, 458}, {430, 452, "c"}, {520, 456}, {700, 446}, {860, 438, "c"}, {1010, 431}}, 9)
work(c2, {pile=p_corner, hand="detail", coverage=1.2, fill=true, pressure=0.7,
          edge={found=0.5, soft=0.5, period=90, seed=4}})
print("corner", wait(0))

--@ chunk 210
p_cl_hi  = pile{{"lead white", 9}, {"yellow ochre", 1.2}, {"chrome yellow", 0.3}}
p_cl_mid = pile{{"lead white", 6}, {"yellow ochre", 1.2}, {"raw umber", 0.9}}
p_cl_sh  = pile{{"lead white", 2}, {"raw umber", 4}, {"cobalt blue", 1.0}, {"red earth", 0.6}}
p_cl_dp  = pile{{"raw umber", 4}, {"lead white", 0.8}, {"cobalt blue", 1.2}, {"bone black", 1.0}}

cloth = poly({{22, 594}, {150, 560}, {300, 536}, {470, 517}, {640, 506}, {742, 517},
              {830, 557}, {882, 621}, {858, 692}, {758, 750}, {600, 792}, {420, 813},
              {200, 823}, {40, 820}, {-20, 800}, {-20, 700}, {-4, 640}}, true)
cloth_area = cloth:area()
work(cloth, {pile=p_cl_mid, hand="broad", coverage=3.2, fill=true, load=1.0,
             pressure={1.0,0.85}, dips={3,1.0,0.05}, angle=0.5})
print("cloth base", cloth_area, wait(0))

--@ chunk 211
work(cloth, {pile=p_cl_mid, hand="broad", coverage=3.0, fill=true, load=1.0,
             pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5})
print("cloth solid", wait(0))

--@ chunk 212
blend(cloth:soften(18), {angle=0.5})
print("cloth blended", wait(0))

--@ chunk 213
-- fold lights
L1 = ribbon({{20,586},{180,556},{360,532},{520,518},{650,512}}, 46):soften(26)
L2 = ribbon({{350,596},{250,650},{150,712},{60,782}}, 64):soften(30)
L3 = ribbon({{300,580},{210,606},{120,640},{30,690}}, 40):soften(24)
L4 = ribbon({{520,650},{490,712},{455,790}}, 58):soften(28)
L5 = ribbon({{740,545},{800,590},{848,650}}, 52):soften(26)
L6 = ribbon({{862,600},{878,650},{862,700}}, 34):soften(20)
-- fold shadows
S1 = ribbon({{420,548},{560,528},{690,522}}, 44):soften(24)
S2 = ribbon({{330,600},{360,650},{400,700}}, 46):soften(24)
S3 = ribbon({{600,676},{640,730},{672,792}}, 44):soften(24)
S4 = ribbon({{700,610},{760,660},{800,712}}, 50):soften(26)
S5 = ellipse(806, 792, 150, 70):soften(40)
S6 = ellipse(52, 792, 130, 78):soften(46)
-- deepest, under the bowl
D1 = ribbon({{400,596},{520,620},{650,634},{762,626}}, 40):soften(22)
D2 = ribbon({{402,700},{344,752},{302,796}}, 36):soften(20)

lay = function(m, p, a) work(m*cloth, {pile=p, hand="body", coverage=2.2, fill=true,
      load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=a, length={30,70}}) end

lay(L1, p_cl_hi, -0.12)  lay(L3, p_cl_hi, -0.40)  lay(L2, p_cl_hi, -0.60)
lay(L4, p_cl_hi, -1.14)  lay(L5, p_cl_hi,  0.77)  lay(L6, p_cl_hi,  1.57)
lay(S1, p_cl_sh, -0.16)  lay(S2, p_cl_sh,  1.10)  lay(S3, p_cl_sh,  1.16)
lay(S4, p_cl_sh,  0.87)  lay(S5, p_cl_sh,  0.10)  lay(S6, p_cl_sh,  0.30)
lay(D1, p_cl_dp,  0.10)  lay(D2, p_cl_dp,  1.24)
blend(cloth:soften(18), {angle=0.5})
print("folds", wait(0))

--@ chunk 214
p_gz   = pile{{"raw umber", 3}, {"bone black", 1.6}, {"Prussian blue", 0.4}, medium=0.35}
p_clm2 = pile{{"lead white", 4.5}, {"raw umber", 2.0}, {"yellow ochre", 0.9}, {"cobalt blue", 0.3}}
tline = function(x) return 468 - 0.04*x end
tbl = below(tline):soften(8)
cloth_r = cloth:roughen(9, 70, 12)

-- bury the hairy fringe of strokes that ran past the cloth onto the table
work(tbl * -cloth_r, {pile=p_tbl, hand="broad", coverage=2.5, fill=true, load=1.0,
      pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.2, clip=true})
-- a warm dark glaze to sink the whole cloth a step and unify it
work(cloth_r, {hand="glaze", pile=p_gz, coverage=1.2, angle=0.5, clip=true})
print("glazed", wait(0))

--@ chunk 215
p_clm2  = pile{{"lead white", 4.5}, {"raw umber", 2.0}, {"yellow ochre", 0.9}, {"cobalt blue", 0.3}}
p_cl_sh2= pile{{"raw umber", 5}, {"lead white", 1.2}, {"cobalt blue", 1.3}, {"bone black", 0.6}}
p_cl_dp2= pile{{"bone black", 2}, {"raw umber", 3}, {"Prussian blue", 0.8}, {"lead white", 0.3}}
cloth_r = cloth:roughen(9, 70, 12)

work(cloth_r, {pile=p_clm2, hand="broad", coverage=2.4, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
blend(cloth_r:soften(18), {angle=0.5})

R = function(pts, w, s) return ribbon(pts, w):soften(s) end
L1 = R({{500,608},{330,700},{140,790}}, 62, 30)
L2 = R({{480,640},{392,722},{332,802}}, 44, 24)
L3 = R({{598,640},{620,722},{642,802}}, 52, 26)
L4 = R({{664,638},{744,712},{824,782}}, 56, 28)
L5 = R({{30,580},{300,545},{560,520},{700,516}}, 44, 26)
L6 = R({{18,600},{56,662},{88,732}}, 38, 22)
S1 = R({{432,690},{352,760},{302,812}}, 50, 26)
S2 = R({{662,700},{702,762},{742,816}}, 44, 24)
S3 = R({{832,650},{872,722}}, 44, 24)
S4 = R({{352,600},{302,650}}, 44, 24)
S5 = R({{432,600},{570,630},{720,640}}, 46, 26)
S6 = ellipse(834, 800, 145, 62):soften(34)
S7 = ellipse(36, 800, 115, 60):soften(34)
D1 = R({{452,606},{580,634},{706,642}}, 28, 16)
D2 = R({{420,716},{366,772}}, 30, 16)

lay = function(m, p, a, cov) work(m*cloth_r, {pile=p, hand="body", coverage=cov, fill=true,
      load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=a, length={30,70}, clip=true}) end

lay(L1, p_cl_hi, -0.62, 2.0)  lay(L2, p_cl_hi, -1.02, 1.8)  lay(L3, p_cl_hi,  1.42, 1.8)
lay(L4, p_cl_hi,  0.61, 1.8)  lay(L5, p_cl_hi, -0.08, 1.8)  lay(L6, p_cl_hi,  1.20, 1.6)
lay(S1, p_cl_sh2, -1.05, 2.2) lay(S2, p_cl_sh2,  0.80, 2.0)  lay(S3, p_cl_sh2,  1.30, 2.0)
lay(S4, p_cl_sh2,  0.30, 2.0)  lay(S5, p_cl_sh2,  0.12, 2.0)  lay(S6, p_cl_sh2,  0.20, 2.2)
lay(S7, p_cl_sh2,  0.30, 2.2)  lay(D1, p_cl_dp2,  0.12, 2.0) lay(D2, p_cl_dp2, -1.05, 2.0)
blend(cloth_r:soften(18), {angle=0.5})
print("folds 2", wait(0))

--@ chunk 216
p_reset = pile{{"lead white", 3}, {"raw umber", 5}, {"bone black", 1.5}, {"cobalt blue", 0.4}}
p_r_lit = pile{{"vermilion", 5}, {"red earth", 1.5}, {"yellow ochre", 1.5}, {"lead white", 1.5}}
p_r_mid = pile{{"vermilion", 5}, {"red earth", 3}, {"raw umber", 1.2}, {"yellow ochre", 0.6}}
p_r_sh  = pile{{"vermilion", 2}, {"red earth", 2.5}, {"raw umber", 2}, {"bone black", 1.2}}
p_r_dk  = pile{{"red earth", 2}, {"raw umber", 2}, {"bone black", 2}, {"Prussian blue", 0.5}}
p_g_lit = pile{{"yellow ochre", 3}, {"chrome yellow", 1.5}, {"lead white", 2}, {"green earth", 1.5}}
p_g_mid = pile{{"green earth", 5}, {"yellow ochre", 2}, {"chrome yellow", 1}}
p_g_sh  = pile{{"green earth", 4}, {"raw umber", 2}, {"bone black", 1.5}, {"Prussian blue", 0.5}}
p_bo_lit= pile{{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 2.5}}
p_bo_mid= pile{{"yellow ochre", 3.5}, {"red earth", 3}, {"raw umber", 1.2}, {"bone black", 0.5}}

chart = {p_cl_hi, p_clm2, p_cl_sh2, p_cl_dp2, p_r_lit, p_r_mid, p_r_sh, p_r_dk, p_g_lit, p_g_mid}
work(rect(800, 655, 200, 145), {pile=p_reset, hand="broad", coverage=2.6, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
for i, p in ipairs(chart) do
  c, r = (i-1) % 5, math.floor((i-1)/5)
  m = rect(800 + c*40, 655 + r*72, 40, 72)
  work(m, {pile=p, hand="body", coverage=2.6, fill=true, load=1.0,
           pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
end
print("chart")

--@ chunk 217
p_cl_fall = pile{{"raw umber", 2.5}, {"bone black", 1.2}, {"Prussian blue", 0.5}, {"lead white", 0.6}}
tline = function(x) return 468 - 0.04*x end
tbl = below(tline):soften(8)
cloth_r = cloth:roughen(9, 70, 12)
work(rect(790, 640, 210, 160), {pile=p_tbl, hand="broad", coverage=2.6, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})

-- knock the cloth back to its mid value
work(cloth_r, {pile=p_clm2, hand="broad", coverage=2.2, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
-- the light falls off to the lower right
work(cloth_r * ellipse(900, 850, 460, 330):soften(150), {pile=p_cl_fall, hand="broad",
     coverage=1.8, fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})

R = function(pts, w, s) return ribbon(pts, w):soften(s) end
L1 = R({{500,608},{330,700},{140,790}}, 54, 30)
L2 = R({{480,640},{392,722},{332,802}}, 38, 24)
L3 = R({{598,640},{620,722},{642,802}}, 44, 26)
L4 = R({{664,638},{744,712},{824,782}}, 46, 28)
L5 = R({{30,580},{300,545},{560,520},{700,516}}, 36, 26)
L6 = R({{18,600},{56,662},{88,732}}, 32, 22)
S1 = R({{432,690},{352,760},{302,812}}, 50, 26)
S2 = R({{662,700},{702,762},{742,816}}, 44, 24)
S3 = R({{832,650},{872,722}}, 44, 24)
S4 = R({{352,600},{302,650}}, 44, 24)
S5 = R({{432,600},{570,630},{720,640}}, 46, 26)
S6 = ellipse(838, 802, 140, 58):soften(32)
S7 = ellipse(30, 802, 112, 58):soften(32)
D1 = R({{452,606},{580,634},{706,642}}, 30, 16)
D2 = R({{420,716},{366,772}}, 32, 16)

lay = function(m, p, a, cov) work(m*cloth_r, {pile=p, hand="body", coverage=cov, fill=true,
      load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=a, length={30,70}, clip=true}) end

lay(S1, p_cl_sh2, -1.05, 2.2) lay(S2, p_cl_sh2,  0.80, 2.0)  lay(S3, p_cl_sh2,  1.30, 2.0)
lay(S4, p_cl_sh2,  0.30, 2.0)  lay(S5, p_cl_sh2,  0.12, 2.0)  lay(S6, p_cl_sh2,  0.20, 2.2)
lay(S7, p_cl_sh2,  0.30, 2.2)  lay(D1, p_cl_dp2,  0.12, 2.0) lay(D2, p_cl_dp2, -1.05, 2.0)
lay(L1, p_cl_hi, -0.62, 1.8)  lay(L2, p_cl_hi, -1.02, 1.6)  lay(L3, p_cl_hi,  1.42, 1.6)
lay(L4, p_cl_hi,  0.61, 1.6)  lay(L5, p_cl_hi, -0.08, 1.6)  lay(L6, p_cl_hi,  1.20, 1.4)
blend(cloth_r:soften(18), {angle=0.5})
print("cloth 4", wait(0))

--@ chunk 218
cloth_r = cloth:roughen(9, 70, 12)
work(cloth_r, {pile=p_clm2, hand="broad", coverage=2.8, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
work(cloth_r, {pile=p_clm2, hand="broad", coverage=2.0, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
print("base only", wait(0))

--@ chunk 219
p_cl_hi2 = pile{{"lead white", 10}, {"yellow ochre", 0.8}, {"chrome yellow", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

S1 = R({{470,600},{400,690},{330,790}}, 66, 34)
S2 = R({{640,650},{690,730},{716,800}}, 52, 28)
S3 = R({{760,610},{830,680},{872,740}}, 54, 28)
S4 = R({{400,560},{520,535}}, 46, 26)
S5 = R({{120,600},{70,680}}, 40, 24)
S6 = ellipse(16, 800, 110, 60):soften(30)
S7 = ellipse(852, 800, 130, 55):soften(30)
D1 = R({{440,606},{580,634},{716,642}}, 34, 18)
D2 = R({{420,660},{370,740}}, 30, 16)
D3 = R({{800,650},{840,700}}, 26, 14)
C1 = R({{30,578},{300,543},{560,518},{700,514}}, 26, 16)
C2 = R({{360,650},{250,716},{140,786}}, 34, 20)
C3 = R({{540,660},{542,730}}, 28, 18)
C4 = R({{700,640},{780,700}}, 30, 18)
C5 = R({{24,610},{50,670},{70,720}}, 22, 14)
C6 = R({{856,600},{868,652}}, 22, 14)
C7 = R({{300,782},{450,802}}, 24, 16)

lay = function(m, p, a, cov) work(m*cloth_r, {pile=p, hand="body", coverage=cov, fill=true,
      load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=a, length={28,64}, clip=true}) end

lay(S1, p_cl_sh2, -1.05, 2.2) lay(S2, p_cl_sh2,  1.05, 2.2)  lay(S3, p_cl_sh2,  0.90, 2.2)
lay(S4, p_cl_sh2, -0.10, 2.0) lay(S5, p_cl_sh2,  1.25, 2.0)  lay(S6, p_cl_sh2,  0.20, 2.2)
lay(S7, p_cl_sh2,  0.20, 2.2)  lay(D1, p_cl_dp2,  0.10, 2.0) lay(D2, p_cl_dp2, -1.10, 2.0)
lay(D3, p_cl_dp2,  0.90, 2.0)
work(cloth_r * ellipse(920, 860, 480, 340):soften(160), {pile=p_cl_fall, hand="broad",
     coverage=1.8, fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
lay(C1, p_cl_hi2, -0.08, 1.6) lay(C2, p_cl_hi2, -0.85, 1.6) lay(C3, p_cl_hi2,  1.50, 1.4)
lay(C4, p_cl_hi2,  0.72, 1.4) lay(C5, p_cl_hi2,  1.15, 1.4) lay(C6, p_cl_hi2,  1.50, 1.4)
lay(C7, p_cl_hi2,  0.20, 1.4)
print("folds 3", wait(0))

--@ chunk 220
p_cl_wd  = pile{{"lead white", 1.2}, {"raw umber", 4}, {"bone black", 1.2}, {"red earth", 0.8}}
p_cl_wdp = pile{{"bone black", 3}, {"raw umber", 2}, {"red earth", 0.5}, {"Prussian blue", 0.3}}
p_cl_hi2 = pile{{"lead white", 10}, {"yellow ochre", 1.2}, {"chrome yellow", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- bury the value chart
work(rect(788, 636, 212, 164), {pile=p_tbl, hand="broad", coverage=3.0, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
work(rect(788, 636, 212, 164), {pile=p_tbl, hand="broad", coverage=2.4, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})

-- sink the whole cloth to a mid-light, then lay the hollows into the wet paint
work(cloth_r, {pile=p_clm2, hand="broad", coverage=2.2, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
work(cloth_r, {pile=p_cl_fall, hand="broad", coverage=1.5, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})

Z1 = R({{450,600},{380,690},{310,780}}, 110, 60)
Z2 = R({{740,600},{820,690}}, 90, 50)
Z3 = R({{330,570},{480,545}}, 70, 40)
Z4 = ellipse(862, 802, 160, 70):soften(40)
Z5 = ellipse(8, 802, 120, 70):soften(40)
Z6 = R({{300,700},{220,772}}, 70, 40)
zw = function(m, a) work(m*cloth_r, {pile=p_cl_wd, tool="filbert 16", length={40,90},
        coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zw(Z1, -1.0) zw(Z2, 0.95) zw(Z3, -0.15) zw(Z4, 0.2) zw(Z5, 0.3) zw(Z6, -0.9)
work((R({{452,606},{586,634},{716,642}}, 36, 20) + R({{404,682},{366,752}}, 30, 18))*cloth_r,
     {pile=p_cl_wdp, tool="filbert 12", length={30,64}, coverage=1.8, fill=true, load=1.0,
      pressure={0.95,0.8}, dips={1,1.0,0.0}, clip=true})
blend(cloth_r:soften(20), {angle=0.5})

-- the crests, on top, opaque, warm white
kc = function(m, a) work(m*cloth_r, {pile=p_cl_hi2, tool="filbert 12", length={30,64},
        coverage=1.5, fill=true, load=1.0, pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=a, clip=true}) end
kc(R({{30,578},{300,543},{560,518},{700,514}}, 30, 18), -0.08)
kc(R({{370,640},{250,720},{150,790}}, 38, 22), -0.85)
kc(R({{540,660},{545,740}}, 30, 20),  1.50)
kc(R({{700,640},{790,710}}, 34, 20),  0.72)
kc(R({{24,610},{52,672},{72,724}}, 24, 16), 1.15)
kc(R({{858,600},{870,654}}, 24, 16),  1.50)
kc(R({{320,786},{460,804}}, 26, 18),  0.20)
print("cloth 5", wait(0))

--@ chunk 221
p_cl_base = pile{{"lead white", 5}, {"raw umber", 3}, {"yellow ochre", 1}, {"red earth", 0.5}}
p_cl_lite = pile{{"lead white", 8}, {"raw umber", 1.2}, {"yellow ochre", 0.8}}
p_cl_wd   = pile{{"lead white", 1.2}, {"raw umber", 4}, {"bone black", 1.2}, {"red earth", 0.8}}
p_cl_fall = pile{{"raw umber", 2.5}, {"bone black", 1.2}, {"Prussian blue", 0.5}, {"lead white", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

work(cloth_r, {pile=p_cl_base, hand="broad", coverage=2.4, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})

Lt1 = R({{180,570},{330,600},{420,700}}, 150, 80)
Lt2 = R({{150,720},{270,782}}, 120, 70)
Lt3 = R({{530,650},{548,730}}, 64, 36)
zone = function(m, p, a) work(m*cloth_r, {pile=p, tool="filbert 20", length={50,100},
        coverage=1.5, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zone(Lt1, p_cl_lite, 0.4) zone(Lt2, p_cl_lite, 0.5) zone(Lt3, p_cl_lite, 1.4)

Sh1 = R({{520,600},{430,690},{360,780}}, 130, 70)
Sh2 = R({{780,600},{840,700}}, 100, 55)
Sh3 = R({{430,555},{560,530}}, 80, 45)
Sh4 = ellipse(872, 802, 170, 75):soften(45)
Sh5 = ellipse(0, 802, 120, 70):soften(45)
Sh6 = R({{866,570},{880,652}}, 50, 28)
zone(Sh1, p_cl_wd, -1.0) zone(Sh2, p_cl_wd, 0.95) zone(Sh3, p_cl_wd, -0.15)
zone(Sh4, p_cl_wd, 0.2) zone(Sh5, p_cl_wd, 0.3) zone(Sh6, p_cl_wd, 1.4)
work(cloth_r * ellipse(940, 870, 520, 380):soften(180), {pile=p_cl_fall, tool="filbert 20",
     length={50,100}, coverage=1.3, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
blend(cloth_r:soften(20), {angle=0.5})
print("cloth 6", wait(0))

--@ chunk 222
p_clm3  = pile{{"lead white", 2}, {"raw umber", 4}, {"red earth", 1}, {"yellow ochre", 0.6}, {"bone black", 0.8}}
p_cl_lite= pile{{"lead white", 8}, {"raw umber", 1.2}, {"yellow ochre", 0.8}}
p_cl_dk  = pile{{"raw umber", 2}, {"bone black", 2}, {"red earth", 0.5}, {"Prussian blue", 0.25}}
p_cl_fall2=pile{{"raw umber", 3}, {"bone black", 1.6}, {"red earth", 0.5}}
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

work(cloth_r, {pile=p_clm3, hand="broad", coverage=2.4, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
work(cloth_r, {pile=p_clm3, hand="broad", coverage=1.8, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})

zone = function(m, p, a, cov, t) work(m*cloth_r, {pile=p, tool=t or "filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end

-- lights: the back-left roll, the left of the group, a crest in front
zone(R({{150,568},{310,592},{400,690}}, 140, 80), p_cl_lite, 0.4, 1.6)
zone(R({{60,650},{180,730},{250,800}}, 120, 70), p_cl_lite, 0.5, 1.5)
zone(R({{520,644},{546,724}}, 60, 34), p_cl_lite, 1.4, 1.4)
zone(R({{700,580},{780,600},{830,650}}, 90, 50), p_cl_lite, 0.7, 1.2)

-- darks: the hollow under the bowl, the right hollow, the back hollow, the corners
zone(R({{520,600},{430,690},{360,780}}, 130, 70), p_cl_dk, -1.0, 1.7)
zone(R({{780,600},{840,700}}, 100, 55), p_cl_dk, 0.95, 1.7)
zone(R({{430,555},{560,530}}, 80, 45), p_cl_dk, -0.15, 1.5)
zone(ellipse(876, 804, 170, 75):soften(45), p_cl_dk, 0.2, 1.7)
zone(ellipse(0, 804, 120, 70):soften(45), p_cl_dk, 0.3, 1.7)
zone(R({{866,570},{880,652}}, 50, 28), p_cl_dk, 1.4, 1.5)
work(cloth_r * ellipse(940, 870, 520, 380):soften(180), {pile=p_cl_fall2, tool="filbert 20",
     length={50,100}, coverage=1.2, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
blend(cloth_r:soften(20), {angle=0.5})
print("cloth 7", wait(0))

--@ chunk 223
p_clm4 = pile{{"lead white", 1.5}, {"raw umber", 4.5}, {"red earth", 1.2}, {"yellow ochre", 0.5}, {"bone black", 1}}
cloth_r = cloth:roughen(9, 70, 12)
work(cloth_r, {pile=p_clm4, hand="broad", coverage=1.0, fill=true, load=1.0,
     pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.5, clip=true})
for i = 1, 3 do
  work(rect(786, 634, 214, 166), {pile=p_tbl, hand="broad", coverage=3.0, fill=true,
       load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
end
print("settled", wait(0))

--@ chunk 224

print(pcall(function() return ribbon({{500,628},{640,668}}, 96) end))
print(pcall(function() return ellipse(580,500,200,64) + ellipse(580,545,196,62) end))
print(pcall(function() return (ellipse(580,500,200,64):blur(4):roughen(4,60,3)) end))
print(pcall(function() local a=ellipse(1,1,2,2) return ellipse(3,3,4,4) - a end))
print(pcall(function() return ribbon({{500,628},{640,668}}, {96, 80}) end))

--@ chunk 225

p_bo_lit = pile{{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 2.5}}
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
tline = function(x) return 468 - 0.04*x end
print(pcall(function() return below(tline) * (rect(700,560,300,240):soften(90) + rect(786,634,214,166)) * -cloth_r end))
print(pcall(function() return R({{500,628},{640,668},{772,706}}, 96, 44)*cloth_r end))
print(pcall(function() return R({{528,632},{636,662}}, 42, 20)*cloth_r end))

--@ chunk 226

p_bo_lit = pile{{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 2.5}}
z = rect(0,0,0.3,0.3)
print(pcall(function() work(z, {pile=p_bo_lit, tool="filbert 20", length={40,90}, coverage=2.6, fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true}) end))
print(pcall(function() work(z, {pile=p_bo_lit, tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.6, clip=true}) end))
print(pcall(function() blend(z, {angle=0.5}) end))
print(pcall(function() work(z, {pile=p_bo_lit, tool="filbert 16", length={30,64}, coverage=1.4, fill=true, load=1.0, pressure={0.9,0.8}, dips={1,1.0,0.0}, angle=1.2, clip=true}) end))

--@ chunk 227

R = function(pts, w, s) return ribbon(pts, w):soften(s) end
sil = ellipse(580,500,200,64) + ellipse(580,545,196,62) + ellipse(580,585,165,54) +
      ellipse(580,612,120,40) + ellipse(580,624,66,22)
print(pcall(function() return sil:blur(4):roughen(4, 60, 3) end))
inner = ellipse(580, 500, 186, 56):roughen(3, 50, 5)
print(pcall(function() return (ellipse(580,500,200,64):blur(2) - inner) end))
bowl = sil:blur(4):roughen(4, 60, 3)
print(pcall(function() return bowl - inner end))
print(pcall(function() return ellipse(580, 470, 170, 44):soften(30) end))

--@ chunk 228

for _, k in ipairs{"p_tbl","cloth","cloth_r","p_clm3","p_cl_lite","p_cl_dk","p_cl_fall2","p_bo_lit","p_bo_mid","p_bo_sh","p_bo_dk","p_bo_rf","R","tline","p_cl_hi2","p_cl_wd","p_cl_wdp"} do
  local v = _G[k]; print(k, type(v), (type(v)=="table" or type(v)=="function") and "" or tostring(v))
end

--@ chunk 229

print(pcall(function() return pile{{"red earth", 2.5}, {"raw umber", 2}, {"bone black", 1.2}} end))
print(pcall(function() local q = pile{{"raw umber", 1.5}, {"bone black", 2.5}, {"Prussian blue", 0.5}, {"lead white", 0.2}} return q end))
print(tubes())

--@ chunk 230
p_bo_lit = pile{{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 2.5}}
p_bo_mid = pile{{"yellow ochre", 3.5}, {"red earth", 3}, {"raw umber", 1.2}, {"bone black", 0.5}}
p_bo_sh  = pile{{"red earth", 2.5}, {"raw umber", 2}, {"bone black", 1.2}}
p_bo_dk  = pile{{"raw umber", 1.5}, {"bone black", 2.5}, {"Prussian blue", 0.5}, {"lead white", 0.2}}
tline = function(x) return 468 - 0.04*x end
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
print("piles ok")

--@ chunk 231
tline = function(x) return 468 - 0.04*x end
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
sil = ellipse(580,500,200,64) + ellipse(580,545,196,62) + ellipse(580,585,165,54) +
      ellipse(580,612,120,40) + ellipse(580,624,66,22)
bowl = sil:blur(4):roughen(4, 60, 3)
inner = ellipse(580, 500, 186, 56):roughen(3, 50, 5)
rim   = (ellipse(580,500,200,64):blur(2) - inner)
wall  = bowl - inner
print("geoms ok", bowl, wall, rim)
work(below(tline) * (rect(700,560,300,240):soften(90) + rect(786,634,214,166)) * -cloth_r,
     {pile=p_tbl, hand="broad", coverage=2.8, fill=true, load=1.0, pressure={1.0,0.9},
      dips={1,1.0,0.0}, angle=0.2, clip=true})
work(R({{500,628},{640,668},{772,706}}, 96, 44)*cloth_r, {pile=p_bo_dk, tool="filbert 20",
     length={50,100}, coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{528,632},{636,662}}, 42, 20)*cloth_r, {pile=p_bo_dk, tool="filbert 12",
     length={30,64}, coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
print("shadow ok")

--@ chunk 232
sil = ellipse(580,500,200,64) + ellipse(580,545,196,62) + ellipse(580,585,165,54) +
      ellipse(580,612,120,40) + ellipse(580,624,66,22)
bowl = sil:blur(4):roughen(4, 60, 3)
inner = ellipse(580, 500, 186, 56):roughen(3, 50, 5)
wall  = bowl - inner
work(bowl, {pile=p_bo_mid, tool="filbert 20", length={40,90}, coverage=2.6, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.5, clip=true})
print("base ok")

--@ chunk 233
sil = ellipse(580,500,200,64) + ellipse(580,545,196,62) + ellipse(580,585,165,54) +
      ellipse(580,612,120,40) + ellipse(580,624,66,22)
bowl = sil:blur(4):roughen(4, 60, 3)
inner = ellipse(580, 500, 186, 56):roughen(3, 50, 5)
wall  = bowl - inner
work(inner, {pile=p_bo_sh, tool="filbert 16", length={40,90}, coverage=1.9, fill=true,
     load=1.0, pressure={0.9,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
print("inner ok")
work(ellipse(580, 470, 170, 44):soften(30), {pile=p_bo_dk, tool="filbert 16", length={30,64},
     coverage=1.5, fill=true, load=1.0, pressure={0.9,0.8}, dips={1,1.0,0.0}, clip=true})
print("inner dark ok")

--@ chunk 234
sil = ellipse(580,500,200,64) + ellipse(580,545,196,62) + ellipse(580,585,165,54) +
      ellipse(580,612,120,40) + ellipse(580,624,66,22)
bowl = sil:blur(4):roughen(4, 60, 3)
inner = ellipse(580, 500, 186, 56):roughen(3, 50, 5)
wall  = bowl - inner
work(wall * ellipse(430, 520, 190, 150):soften(70), {pile=p_bo_lit, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.6, clip=true})
work(wall * ellipse(700, 590, 200, 130):soften(80), {pile=p_bo_sh, tool="filbert 16",
     length={40,90}, coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.6, clip=true})
work(bowl * below(function() return 600 end):soften(24), {pile=p_bo_dk, tool="filbert 16",
     length={30,64}, coverage=1.4, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
blend(bowl:soften(6), {angle=0.5})
print("bowl modelled", wait(0))

--@ chunk 235
p_bo2_lit = pile{{"yellow ochre", 3.5}, {"red earth", 2}, {"lead white", 2.5}, {"raw umber", 0.8}}
p_bo2_mid = pile{{"yellow ochre", 3}, {"red earth", 3}, {"raw umber", 2}, {"bone black", 0.5}}
p_bo2_sh  = pile{{"red earth", 2}, {"raw umber", 2.5}, {"bone black", 1}, {"cobalt blue", 0.3}}
p_bo2_dk  = pile{{"raw umber", 1.5}, {"bone black", 2.5}, {"Prussian blue", 0.4}}

tline = function(x) return 468 - 0.04*x end
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the chart rectangle, feathered well past its edges this time
work(below(tline) * rect(690, 520, 310, 280):soften(110) * -cloth_r,
     {pile=p_tbl, hand="broad", coverage=2.8, fill=true, load=1.0, pressure={1.0,0.9},
      dips={1,1.0,0.0}, angle=0.2, clip=true})

rim_o  = ellipse(580, 500, 200, 64)
skirt  = poly({{380,500},{384,528},{398,558},{422,582},{456,600},{500,614},{542,622},
               {580,626},{618,622},{660,614},{704,600},{738,582},{762,558},{776,528},{780,500}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
inner  = ellipse(580, 500, 184, 54):roughen(2, 45, 11)
wall   = bowl - inner

work(bowl, {pile=p_bo2_mid, tool="filbert 20", length={40,90}, coverage=2.8, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.4, clip=true})
-- the inside: dark, deepest at the back
work(inner, {pile=p_bo2_sh, tool="filbert 16", length={40,90}, coverage=2.2, fill=true,
     load=1.0, pressure={0.9,0.8}, dips={1,1.0,0.0}, angle=0.2, clip=true})
work(inner * ellipse(580, 462, 168, 40):soften(26), {pile=p_bo2_dk, tool="filbert 16",
     length={30,64}, coverage=1.8, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, clip=true})
-- the outside: the light comes over the left shoulder and dies on the right
work(wall * ellipse(420, 545, 170, 145):soften(66), {pile=p_bo2_lit, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(716, 588, 175, 125):soften(70), {pile=p_bo2_sh, tool="filbert 16",
     length={40,90}, coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 596 end):soften(22), {pile=p_bo2_dk, tool="filbert 16",
     length={30,64}, coverage=1.5, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
-- the rim: the brightest band, running round the front
work((rim_o - inner) * below(function(x) return 508 - 0.0*(x-580) end):soften(14),
     {pile=p_bo2_lit, tool="filbert 12", length={30,64}, coverage=1.7, fill=true, load=1.0,
      pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.25, clip=true})
blend(bowl:soften(5), {angle=0.4})
print("bowl 2", wait(0))

--@ chunk 236
p_bo3_mid = pile{{"raw umber", 4}, {"red earth", 2}, {"bone black", 1}, {"yellow ochre", 0.6}}
p_bo3_lit = pile{{"raw umber", 2}, {"red earth", 1.2}, {"yellow ochre", 1}, {"lead white", 2.2}}
p_bo3_sh  = pile{{"raw umber", 2.5}, {"red earth", 1}, {"bone black", 1.5}, {"Prussian blue", 0.3}}
p_bo3_dk  = pile{{"bone black", 3}, {"raw umber", 1.5}, {"Prussian blue", 0.4}, {"red earth", 0.4}}

rim_o  = ellipse(580, 500, 200, 64)
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
wall   = bowl - inner
rimb   = rim_o - inner

work(bowl, {pile=p_bo3_mid, tool="filbert 20", length={40,90}, coverage=2.8, fill=true,
     load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.4, clip=true})
-- inside: dark, deepest at the back
work(inner, {pile=p_bo3_dk, tool="filbert 16", length={40,90}, coverage=2.0, fill=true,
     load=1.0, pressure={0.9,0.8}, dips={1,1.0,0.0}, angle=0.2, clip=true})
-- the outside: light over the left shoulder, dying away to the right and under
work(wall * ellipse(448, 542, 155, 122):soften(62), {pile=p_bo3_lit, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(720, 586, 160, 120):soften(64), {pile=p_bo3_sh, tool="filbert 16",
     length={40,90}, coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 592 end):soften(20), {pile=p_bo3_dk, tool="filbert 16",
     length={30,64}, coverage=1.5, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
-- the rim: a narrow light on the front-left arc only
work(rimb * ellipse(480, 540, 130, 90):soften(46), {pile=p_bo3_lit, tool="filbert 12",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
blend(bowl:soften(5), {angle=0.4})
print("bowl 3", wait(0))

--@ chunk 237
cA = pile{{"raw umber", 2}, {"red earth", 1.2}, {"yellow ochre", 1}, {"lead white", 2.2}}
cB = pile{{"raw umber", 2.5}, {"lead white", 3}, {"yellow ochre", 0.6}, {"red earth", 0.5}}
cC = pile{{"raw umber", 3.5}, {"bone black", 1.2}, {"lead white", 1}, {"red earth", 0.8}}
cD = pile{{"bone black", 2}, {"raw umber", 2}, {"lead white", 1.5}, {"Prussian blue", 0.4}}
cand = {cA, cB, cC, cD}
for i, p in ipairs(cand) do
  m = rect(404 + (i-1)*48, 500, 48, 66)
  work(m, {pile=p, hand="body", coverage=3.0, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, clip=true})
end
print("chart2")

--@ chunk 238
p_w4   = pile{{"bone black", 3}, {"raw umber", 2.2}, {"Prussian blue", 0.7}, {"cobalt blue", 0.35}, {"red earth", 0.4}}
p_w4_l = pile{{"raw umber", 2.6}, {"bone black", 1.4}, {"yellow ochre", 0.7}, {"lead white", 0.5}, {"red earth", 0.5}}
p_tb4  = pile{{"raw umber", 3.5}, {"bone black", 1.8}, {"red earth", 1}, {"Prussian blue", 0.3}}
tline = function(x) return 468 - 0.04*x end
wr = rect(0, 0, 1000, 476):soften(24)
tbl = below(tline):soften(8)

work(wr, {pile=p_w4, hand="broad", coverage=2.8, fill=true, load=1.0, pressure={1.0,1.0},
          dips={1,1.0,0.0}, angle=0.9, clip=true})
work(tbl, {pile=p_tb4, hand="broad", coverage=2.8, fill=true, load=1.0, pressure={1.0,1.0},
          dips={1,1.0,0.0}, angle=0.25, clip=true})
pool1 = ellipse(400, 300, 520, 380):soften(190)
pool2 = ellipse(390, 330, 350, 250):soften(140)
pool3 = ellipse(350, 350, 210, 145):soften(100)
work(pool1 * wr, {pile=p_w4_l, hand="broad", coverage=2.4, fill=true, load=1.0,
     pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.6, clip=true})
work(pool2 * wr, {pile=p_w4_l, hand="broad", coverage=1.8, fill=true, load=1.0,
     pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.6, clip=true})
work(pool3 * wr, {pile=p_w4_l, hand="broad", coverage=1.4, fill=true, load=1.0,
     pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.6, clip=true})
work(tbl * ellipse(230, 580, 430, 210):soften(150), {pile=p_w4_l, hand="broad", coverage=1.5,
     fill=true, load=1.0, pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.25, clip=true})
blend(wr:soften(24), {angle=0.5})
print("background", wait(0))

--@ chunk 239
p_bo4_lit = pile{{"raw umber", 3}, {"lead white", 1.8}, {"bone black", 0.6}, {"yellow ochre", 0.5}}
p_bo4_mid = pile{{"raw umber", 3.5}, {"bone black", 1.2}, {"lead white", 1}, {"red earth", 0.8}}
p_bo4_sh  = pile{{"raw umber", 2.5}, {"bone black", 2}, {"red earth", 0.6}, {"Prussian blue", 0.3}}
p_bo4_dk  = pile{{"bone black", 3}, {"raw umber", 1.5}, {"Prussian blue", 0.5}, {"red earth", 0.3}}
cloth_r = cloth:roughen(9, 70, 12)

-- the old bowl's wings stick out past the new profile: put the cloth back
work((ellipse(358,506,44,30):soften(16) + ellipse(802,506,44,30):soften(16))*cloth_r,
     {pile=p_clm4, hand="broad", coverage=2.4, fill=true, load=1.0, pressure={1.0,0.9},
      dips={1,1.0,0.0}, angle=0.3, clip=true})

rim_o  = ellipse(580, 500, 200, 64)
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
wall   = bowl - inner
rimb   = rim_o - inner

work(bowl, {pile=p_bo4_mid, hand="broad", coverage=3.0, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.4, clip=true})
work(inner, {pile=p_bo4_dk, tool="filbert 16", length={40,90}, coverage=2.4, fill=true,
     load=1.0, pressure={0.9,0.8}, dips={1,1.0,0.0}, angle=0.2, clip=true})
work(inner * ellipse(580, 466, 170, 40):soften(26), {pile=p_bo4_dk, tool="filbert 16",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, clip=true})
work(wall * ellipse(452, 540, 150, 118):soften(58), {pile=p_bo4_lit, tool="filbert 16",
     length={40,90}, coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(722, 584, 155, 118):soften(60), {pile=p_bo4_sh, tool="filbert 16",
     length={40,90}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 590 end):soften(20), {pile=p_bo4_dk, tool="filbert 16",
     length={30,64}, coverage=1.5, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
work(rimb * ellipse(474, 538, 126, 86):soften(42), {pile=p_bo4_lit, tool="filbert 12",
     length={30,64}, coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
blend(bowl:soften(5), {angle=0.4})
print("bowl 4", wait(0))

--@ chunk 240
p_bo4_mid = pile{{"raw umber", 3.5}, {"bone black", 1.2}, {"lead white", 1}, {"red earth", 0.8}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
wall   = bowl - inner
rimb   = rim_o - inner

work((ellipse(356,506,48,34):soften(16) + ellipse(804,506,48,34):soften(16) + ellipse(580,648,120,26):soften(18))*cloth_r,
     {pile=p_clm4, hand="broad", coverage=5.0, fill=true, load=1.0, pressure={1.0,1.0},
      dips={1,1.0,0.0}, angle=0.3, clip=true})
work(bowl, {pile=p_bo4_mid, hand="broad", coverage=4.0, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.4, clip=true})
work(bowl, {pile=p_bo4_mid, hand="broad", coverage=3.0, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.4, clip=true})
work(inner, {pile=p_bo4_dk, hand="broad", coverage=3.0, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.2, clip=true})
print("bowl base opaque", wait(0))

--@ chunk 241
p_bo4_lit = pile{{"raw umber", 3}, {"lead white", 1.8}, {"bone black", 0.6}, {"yellow ochre", 0.5}}
p_bo4_sh  = pile{{"raw umber", 2.5}, {"bone black", 2}, {"red earth", 0.6}, {"Prussian blue", 0.3}}
p_bo4_dk  = pile{{"bone black", 3}, {"raw umber", 1.5}, {"Prussian blue", 0.5}, {"red earth", 0.3}}
p_sh_c    = pile{{"raw umber", 2.5}, {"bone black", 1.8}, {"red earth", 0.4}, {"Prussian blue", 0.4}}
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
rim_o  = ellipse(580, 500, 200, 64)
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
wall   = bowl - inner
rimb   = rim_o - inner

-- clear the old orange off the cloth all round the bowl
clear = (ellipse(580, 668, 350, 200):soften(56) + ellipse(352, 508, 52, 38):soften(16) +
         ellipse(808, 508, 52, 38):soften(16)) * cloth_r * -bowl
work(clear, {pile=p_clm4, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
-- the bowl's own shadow on the cloth, close in and to the lower right
work(R({{520,626},{650,662},{782,700}}, 92, 40)*cloth_r, {pile=p_sh_c, tool="filbert 20",
     length={50,100}, coverage=2.0, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{536,628},{644,652}}, 44, 22)*cloth_r, {pile=p_bo4_dk, tool="filbert 12",
     length={30,64}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
-- the outside of the bowl, modelled again over the new base
work(wall * ellipse(452, 540, 150, 118):soften(58), {pile=p_bo4_lit, tool="filbert 16",
     length={40,90}, coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(722, 584, 155, 118):soften(60), {pile=p_bo4_sh, tool="filbert 16",
     length={40,90}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 588 end):soften(20), {pile=p_bo4_dk, tool="filbert 16",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
work(rimb * ellipse(474, 538, 126, 86):soften(42), {pile=p_bo4_lit, tool="filbert 12",
     length={30,64}, coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
blend(bowl:soften(5), {angle=0.4})
print("bowl 6", wait(0))

--@ chunk 242
p_r_mid  = pile{{"vermilion", 5}, {"red earth", 3}, {"raw umber", 1.2}, {"yellow ochre", 0.6}}
p_r_lit  = pile{{"vermilion", 5}, {"red earth", 1.5}, {"yellow ochre", 1.2}, {"lead white", 1.2}}
p_r_sh   = pile{{"vermilion", 2}, {"red earth", 2.5}, {"raw umber", 2}, {"bone black", 1.2}}
p_r_dk   = pile{{"red earth", 2}, {"raw umber", 2}, {"bone black", 2}, {"Prussian blue", 0.5}}
p_r_refl = pile{{"red earth", 1.5}, {"lead white", 1.6}, {"vermilion", 0.6}, {"yellow ochre", 0.5}}
p_r_hi   = pile{{"vermilion", 3}, {"lead white", 3}, {"yellow ochre", 0.8}, {"red earth", 0.5}}

D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_r_mid, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(268, 616, 84, 80):soften(28), {pile=p_r_lit, tool="filbert 16", length={40,90},
     coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(342, 702, 92, 86):soften(24), {pile=p_r_sh, tool="filbert 16", length={40,90},
     coverage=2.2, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(358, 716, 74, 70):soften(20), {pile=p_r_dk, tool="filbert 16", length={30,64},
     coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(306, 738, 84, 32):soften(15), {pile=p_r_refl, tool="filbert 12", length={30,64},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(264, 612, 25, 21):soften(9) * D:grow(4), {pile=p_r_hi, tool="filbert 8",
     length={16,34}, coverage=1.4, fill=true, load=1.0, pressure={0.9,0.7}, clip=true})
print("apple D", wait(0))

--@ chunk 243
p_r_mid2 = pile{{"red earth", 4}, {"vermilion", 2}, {"raw umber", 1.5}, {"yellow ochre", 0.5}}
p_r_lit2 = pile{{"red earth", 2.4}, {"vermilion", 3}, {"yellow ochre", 1.4}, {"lead white", 1.2}}
p_r_sh2  = pile{{"red earth", 2}, {"raw umber", 2}, {"bone black", 1.4}, {"vermilion", 0.5}}
p_r_dk2  = pile{{"raw umber", 1.5}, {"bone black", 2.2}, {"red earth", 1}, {"Prussian blue", 0.3}}
p_r_refl2= pile{{"red earth", 1.2}, {"lead white", 2}, {"vermilion", 0.4}, {"yellow ochre", 0.6}}
p_hi3    = pile{{"lead white", 5}, {"vermilion", 0.8}, {"red earth", 0.4}, {"yellow ochre", 0.3}}
p_r_y    = pile{{"yellow ochre", 3}, {"red earth", 1}, {"vermilion", 0.5}}
p_r_c    = pile{{"red earth", 3}, {"vermilion", 1}, {"bone black", 0.6}, {"Prussian blue", 0.3}}

D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_r_mid2, hand="broad", coverage=3.0, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(276, 632, 78, 74):soften(30), {pile=p_r_lit2, tool="filbert 16", length={40,90},
     coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(322, 680, 88, 84):soften(26), {pile=p_r_sh2, tool="filbert 16", length={40,90},
     coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(352, 708, 76, 72):soften(22), {pile=p_r_dk2, tool="filbert 16", length={30,64},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 742, 80, 28):soften(14), {pile=p_r_refl2, tool="filbert 12", length={30,64},
     coverage=1.5, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
-- the apple's own colour varies: warm patches where the sun struck, crimson elsewhere
work(D * ellipse(258, 600, 58, 50):soften(26), {pile=p_r_y, tool="filbert 20", length={40,90},
     coverage=0.9, fill=true, load=1.0, pressure={0.85,0.7}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(340, 640, 64, 58):soften(30), {pile=p_r_c, tool="filbert 20", length={40,90},
     coverage=0.9, fill=true, load=1.0, pressure={0.85,0.7}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(250, 690, 46, 40):soften(24), {pile=p_r_y, tool="filbert 16", length={30,64},
     coverage=0.7, fill=true, load=1.0, pressure={0.85,0.7}, clip=true})
work(ellipse(262, 610, 22, 18):soften(8) * D:grow(6), {pile=p_hi3, tool="filbert 7",
     length={12,26}, coverage=1.3, fill=true, load=1.0, pressure={0.9,0.7}, clip=true})
print("apple D 2", wait(0))

--@ chunk 244
p_mid3 = pile{{"red earth", 4}, {"vermilion", 1.5}, {"raw umber", 1.5}, {"yellow ochre", 0.4}}
p_lit3 = pile{{"red earth", 3}, {"vermilion", 2.2}, {"yellow ochre", 0.9}, {"lead white", 0.7}}
p_sh3  = pile{{"raw umber", 2}, {"bone black", 2}, {"red earth", 1.5}, {"Prussian blue", 0.3}}
p_dk3  = pile{{"bone black", 3}, {"raw umber", 1.2}, {"red earth", 0.6}, {"Prussian blue", 0.4}}
p_refl3= pile{{"red earth", 1.5}, {"lead white", 1.5}, {"vermilion", 0.3}, {"yellow ochre", 0.5}}
p_hi3  = pile{{"lead white", 4}, {"vermilion", 1}, {"red earth", 0.5}, {"yellow ochre", 0.3}}
p_mot_a= pile{{"yellow ochre", 2}, {"red earth", 2}, {"vermilion", 0.8}}
p_mot_b= pile{{"red earth", 3}, {"bone black", 1}, {"vermilion", 0.5}}

D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_mid3, hand="broad", coverage=3.2, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(272, 628, 76, 72):soften(30), {pile=p_lit3, tool="filbert 16", length={40,90},
     coverage=1.5, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(352, 706, 80, 76):soften(20), {pile=p_sh3, tool="filbert 16", length={40,90},
     coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(368, 720, 58, 54):soften(16), {pile=p_dk3, tool="filbert 16", length={30,64},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 744, 76, 26):soften(12), {pile=p_refl3, tool="filbert 12", length={30,64},
     coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
-- fine flecks of skin colour, stippled so they read as texture and not as spots
stipple(D, {pile=p_mot_a, width=2, coverage=0.45, pressure={0.5,0.35}, cluster={0.5, 26}, clip=true})
stipple(D, {pile=p_mot_b, width=2, coverage=0.35, pressure={0.5,0.35}, cluster={0.5, 30}, clip=true})
work(ellipse(260, 608, 20, 16):soften(7) * D:grow(6), {pile=p_hi3, tool="filbert 6",
     length={10,22}, coverage=1.0, fill=true, load=1.0, pressure={0.85,0.6}, clip=true})
print("apple D 3", wait(0))

--@ chunk 245
p_mid4 = pile{{"red earth", 4}, {"vermilion", 1.2}, {"raw umber", 2}, {"bone black", 0.6}, {"yellow ochre", 0.3}}
p_lit4 = pile{{"red earth", 3}, {"vermilion", 1.6}, {"yellow ochre", 0.7}, {"lead white", 0.4}, {"raw umber", 0.6}}
p_tr4  = pile{{"red earth", 2.5}, {"raw umber", 1.5}, {"bone black", 1}, {"vermilion", 0.8}}
p_sh4  = pile{{"raw umber", 2}, {"bone black", 2.4}, {"red earth", 1}, {"Prussian blue", 0.3}}
p_refl4= pile{{"red earth", 1.2}, {"lead white", 1.2}, {"vermilion", 0.3}, {"yellow ochre", 0.4}}
p_hi4  = pile{{"lead white", 3}, {"vermilion", 0.8}, {"red earth", 0.4}, {"yellow ochre", 0.3}}
p_grn_mid = pile{{"green earth", 4}, {"chrome yellow", 1.6}, {"yellow ochre", 1.2}}
p_grn_lit = pile{{"yellow ochre", 2.6}, {"chrome yellow", 1.2}, {"lead white", 1.4}, {"green earth", 1.6}}
p_grn_sh  = pile{{"green earth", 3}, {"raw umber", 1.6}, {"bone black", 1.6}, {"Prussian blue", 0.4}}
p_grn_dk  = pile{{"bone black", 2.6}, {"green earth", 1.6}, {"Prussian blue", 0.6}}

-- value chart, hidden in the bowl's interior
chart = {p_mid4, p_lit4, p_tr4, p_sh4, p_refl4, p_hi4, p_grn_mid, p_grn_lit, p_grn_sh, p_grn_dk}
for i, p in ipairs(chart) do
  work(rect(482 + (i-1)*28, 498, 28, 48), {pile=p, hand="body", coverage=3.0, fill=true,
       load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
end

D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_mid4, hand="broad", coverage=3.2, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(268, 622, 66, 62):soften(28), {pile=p_lit4, tool="filbert 16", length={40,90},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(326, 676, 86, 82):soften(24), {pile=p_tr4, tool="filbert 16", length={40,90},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(360, 710, 72, 68):soften(18), {pile=p_sh4, tool="filbert 16", length={40,90},
     coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 744, 76, 24):soften(12), {pile=p_refl4, tool="filbert 12", length={30,64},
     coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(260, 608, 19, 15):soften(7) * D:grow(6), {pile=p_hi4, tool="filbert 5",
     length={10,20}, coverage=1.6, fill=false, load=1.0, pressure={0.8,0.6}, clip=true})
print("apple D 4", wait(0))

--@ chunk 246
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_mid4, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D, {pile=p_mid4, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.2, clip=true})
work(D, {pile=p_mid4, tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
         pressure={1.0,0.9}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(268, 622, 66, 62):soften(28), {pile=p_lit4, tool="filbert 16", length={40,90},
     coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(326, 676, 86, 82):soften(24), {pile=p_tr4, tool="filbert 16", length={40,90},
     coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(360, 710, 72, 68):soften(18), {pile=p_sh4, tool="filbert 16", length={40,90},
     coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 744, 76, 24):soften(12), {pile=p_refl4, tool="filbert 12", length={30,64},
     coverage=1.4, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(260, 608, 19, 15):soften(7) * D:grow(6), {pile=p_hi4, tool="filbert 5",
     length={10,20}, coverage=1.6, fill=false, load=1.0, pressure={0.8,0.6}, clip=true})
print("apple D 5", wait(0))

--@ chunk 247

work(rect(430,690,60,60), {pile=p_mid4, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
work(rect(500,690,60,60), {pile=p_mid4, hand="broad", coverage=10.0, fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
work(rect(570,690,60,60), {pile=p_mid4, hand="body", coverage=3.0, fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
print("diag")

--@ chunk 248

c1 = pile{{"red earth", 5}, {"bone black", 2}, {"raw umber", 2}, {"vermilion", 0.8}, {"yellow ochre", 0.3}}
c2 = pile{{"red earth", 4}, {"bone black", 3}, {"raw umber", 2.5}, {"vermilion", 0.6}}
c3 = pile{{"red earth", 3}, {"bone black", 2}, {"raw umber", 2}, {"vermilion", 1.2}, {"yellow ochre", 0.4}}
c4 = pile{{"red earth", 5}, {"bone black", 1.5}, {"raw umber", 2}, {"vermilion", 1.5}, {"yellow ochre", 0.4}}
c5 = pile{{"green earth", 5}, {"bone black", 2}, {"yellow ochre", 1.2}, {"chrome yellow", 0.8}}
c6 = pile{{"green earth", 4}, {"bone black", 1.2}, {"yellow ochre", 1.6}, {"chrome yellow", 1.2}, {"lead white", 0.8}}
cand = {c1,c2,c3,c4,c5,c6}
for i,p in ipairs(cand) do
  work(rect(424 + (i-1)*56, 636, 55, 60), {pile=p, hand="broad", coverage=5.0, fill=true,
       load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, clip=true})
end
print("chart3")

--@ chunk 249
p_rmid = pile{{"red earth", 3}, {"bone black", 2}, {"raw umber", 2}, {"vermilion", 1.2}, {"yellow ochre", 0.4}}
p_rlit = pile{{"red earth", 4}, {"vermilion", 1.6}, {"yellow ochre", 0.8}, {"lead white", 0.5}, {"raw umber", 1}}
p_rtr  = pile{{"red earth", 4}, {"bone black", 3}, {"raw umber", 2.5}, {"vermilion", 0.6}}
p_rsh  = pile{{"bone black", 3}, {"raw umber", 2}, {"red earth", 1}, {"Prussian blue", 0.3}}
p_rrfl = pile{{"red earth", 1.2}, {"lead white", 1.5}, {"yellow ochre", 0.5}}
p_rhi  = pile{{"lead white", 3.5}, {"vermilion", 0.9}, {"red earth", 0.4}, {"yellow ochre", 0.3}}
p_stem = pile{{"bone black", 2}, {"raw umber", 2}, {"red earth", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)

D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
-- the shadow it throws on the cloth, away from the light
work(ellipse(352, 722, 100, 34):soften(30) * cloth_r * -D:grow(8), {pile=p_rsh, tool="filbert 20",
     length={50,100}, coverage=2.0, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(ellipse(318, 736, 66, 20):soften(12) * cloth_r * -D:grow(4), {pile=p_stem, tool="filbert 12",
     length={30,64}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
-- the apple
work(D, {pile=p_rmid, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(268, 622, 66, 62):soften(28), {pile=p_rlit, tool="filbert 16", length={40,90},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(326, 676, 86, 82):soften(24), {pile=p_rtr, tool="filbert 16", length={40,90},
     coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(360, 710, 72, 68):soften(18), {pile=p_rsh, tool="filbert 16", length={40,90},
     coverage=1.9, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 744, 76, 24):soften(12), {pile=p_rrfl, tool="filbert 12", length={30,64},
     coverage=1.4, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(262, 612, 19, 15):soften(7) * D:grow(6), {pile=p_rhi, tool="filbert 5",
     length={10,20}, coverage=1.5, fill=false, load=1.0, pressure={0.8,0.6}, clip=true})
-- stalk and calyx
sb = brush{kind="round", width=7, point=0.9}
sb:load(p_stem, 0.95)
sb:stroke({{301, 574}, {307, 560}, {316, 546}}, {pressure={0.95,0.7,0.1}, clip=D:grow(14)})
print("apple D 6", wait(0))

--@ chunk 250
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
zone = function(m, p, a, cov) work(m*cloth_r, {pile=p, tool="filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end

work(cloth_r, {pile=p_clm4, hand="broad", coverage=1.3, fill=true, load=1.0, pressure={1.0,0.9},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
-- put the fold structure back on top of the darker cloth
zone(R({{150,568},{310,592},{400,690}}, 140, 80), p_cl_lite, 0.4, 1.3)
zone(R({{60,650},{180,730},{250,800}}, 120, 70), p_cl_lite, 0.5, 1.2)
zone(R({{520,644},{546,724}}, 60, 34), p_cl_lite, 1.4, 1.1)
zone(R({{700,580},{780,600},{830,650}}, 90, 50), p_cl_lite, 0.7, 1.0)
zone(R({{520,600},{430,690},{360,780}}, 130, 70), p_cl_dk, -1.0, 1.4)
zone(R({{780,600},{840,700}}, 100, 55), p_cl_dk, 0.95, 1.4)
zone(R({{430,555},{560,530}}, 80, 45), p_cl_dk, -0.15, 1.3)
zone(ellipse(876, 804, 170, 75):soften(45), p_cl_dk, 0.2, 1.4)
zone(ellipse(0, 804, 120, 70):soften(45), p_cl_dk, 0.3, 1.4)
work(cloth_r * ellipse(940, 870, 520, 380):soften(180), {pile=p_cl_fall2, tool="filbert 20",
     length={50,100}, coverage=1.2, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
print("cloth down", wait(0))

--@ chunk 251
p_rbase = pile{{"red earth", 4}, {"bone black", 3}, {"raw umber", 2.5}, {"vermilion", 0.6}}
p_rlit2 = pile{{"red earth", 3}, {"bone black", 2}, {"raw umber", 2}, {"vermilion", 1.2}, {"yellow ochre", 0.4}}
p_rmid2 = pile{{"red earth", 3}, {"bone black", 1.6}, {"raw umber", 1.8}, {"vermilion", 1.6}, {"yellow ochre", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the value charts left swatches on the cloth in front of the bowl
work(rect(414, 626, 364, 140):soften(24) * cloth_r, {pile=p_clm4, hand="broad", coverage=4.5,
     fill=true, load=1.0, pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.4, clip=true})
work(R({{414,660},{560,700},{700,726}}, 80, 40) * cloth_r, {pile=p_cl_dk, tool="filbert 20",
     length={50,100}, coverage=1.6, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
-- the bowl's shadow again, over the cleared ground
work(R({{520,626},{650,662},{782,700}}, 92, 40)*cloth_r, {pile=p_bo4_dk, tool="filbert 20",
     length={50,100}, coverage=1.8, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{536,628},{644,652}}, 44, 22)*cloth_r, {pile=p_stem, tool="filbert 12",
     length={30,64}, coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
-- apple D again, a value darker all round
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_rbase, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(270, 626, 68, 64):soften(28), {pile=p_rlit2, tool="filbert 16", length={40,90},
     coverage=1.5, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(300, 660, 76, 72):soften(24), {pile=p_rmid2, tool="filbert 16", length={40,90},
     coverage=1.4, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(330, 684, 84, 80):soften(22), {pile=p_rtr, tool="filbert 16", length={40,90},
     coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(362, 712, 70, 66):soften(16), {pile=p_rsh, tool="filbert 16", length={40,90},
     coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 744, 76, 24):soften(12), {pile=p_rrfl, tool="filbert 12", length={30,64},
     coverage=1.4, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(262, 612, 19, 15):soften(7) * D:grow(6), {pile=p_rhi, tool="filbert 5",
     length={10,20}, coverage=1.5, fill=false, load=1.0, pressure={0.8,0.6}, clip=true})
print("apple D 7", wait(0))

--@ chunk 252
p_rbase2 = pile{{"red earth", 3}, {"bone black", 1.6}, {"raw umber", 1.8}, {"vermilion", 1.6}, {"yellow ochre", 0.6}}
p_rlit3  = pile{{"vermilion", 2.5}, {"red earth", 2.5}, {"yellow ochre", 1}, {"lead white", 0.6}}
p_rrfl2  = pile{{"red earth", 1.2}, {"lead white", 1}, {"yellow ochre", 0.5}, {"vermilion", 0.3}}
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
work(D, {pile=p_rbase2, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
         dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(270, 626, 68, 64):soften(28), {pile=p_rlit3, tool="filbert 16", length={40,90},
     coverage=1.5, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(330, 684, 84, 80):soften(22), {pile=p_rtr, tool="filbert 16", length={40,90},
     coverage=1.7, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(362, 712, 70, 66):soften(16), {pile=p_rsh, tool="filbert 16", length={40,90},
     coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(D * ellipse(304, 744, 74, 22):soften(11), {pile=p_rrfl2, tool="filbert 12", length={30,64},
     coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(262, 612, 19, 15):soften(7) * D:grow(6), {pile=p_rhi, tool="filbert 5",
     length={10,20}, coverage=1.5, fill=false, load=1.0, pressure={0.8,0.6}, clip=true})
rg = brush{kind="round", width=7, point=0.95}
rg:load(p_stem, 1.0)
rg:stroke({{302, 576}, {308, 561}, {317, 546}}, {pressure={1.0,0.65,0.05}})
print("apple D 8", wait(0))

--@ chunk 253
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
p_rhalf = pile{{"red earth", 3}, {"bone black", 2}, {"raw umber", 2}, {"vermilion", 1}}
-- a coherent turn from the light into the core shadow
work(D * (ellipse(330, 682, 86, 82):soften(24) - ellipse(272, 628, 70, 66):soften(28)),
     {pile=p_rhalf, tool="filbert 16", length={40,90}, coverage=1.2, fill=true, load=1.0,
      pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=0.3, clip=true})
-- the shadow-side edge melts into the shadow it throws
work((D:grow(7):soften(9) - D) * ellipse(368, 706, 150, 120):soften(30), {pile=p_rsh,
     tool="filbert 16", length={40,90}, coverage=1.3, fill=true, load=1.0, pressure={0.85,0.7},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
blend(D:soften(5), {angle=0.3})
work(ellipse(263, 613, 17, 13):soften(6) * D:grow(6), {pile=p_rhi, tool="filbert 8",
     length={14,26}, coverage=1.1, fill=false, load=1.0, pressure={0.7,0.5}, clip=true})
rg = brush{kind="round", width=8, point=0.95}
rg:load(p_stem, 1.0)
rg:stroke({{303, 578}, {309, 562}, {318, 547}}, {pressure={1.0,0.6,0.05}})
work(ellipse(303, 580, 13, 7):soften(4), {pile=p_stem, tool="filbert 5", length={8,16},
     coverage=1.4, fill=false, load=1.0, pressure={0.85,0.6}, clip=D:grow(3)})
print("apple D 9", wait(0))

--@ chunk 254
p_rb3 = pile{{"red earth", 3}, {"bone black", 1.4}, {"raw umber", 1.6}, {"vermilion", 1.8}, {"yellow ochre", 0.7}, {"lead white", 0.5}}
p_rl4 = pile{{"vermilion", 2.6}, {"red earth", 2.4}, {"yellow ochre", 1}, {"lead white", 1}}
p_gb  = pile{{"green earth", 4}, {"bone black", 1.2}, {"yellow ochre", 1.6}, {"chrome yellow", 1.2}, {"lead white", 1.4}}
p_gl  = pile{{"green earth", 3}, {"yellow ochre", 1.6}, {"chrome yellow", 1}, {"lead white", 2}}
p_gt  = pile{{"green earth", 4.5}, {"bone black", 1.8}, {"yellow ochre", 1}, {"chrome yellow", 0.7}}
p_gs  = pile{{"green earth", 2.4}, {"bone black", 2.6}, {"Prussian blue", 0.5}, {"raw umber", 0.8}}
p_gr  = pile{{"yellow ochre", 1.6}, {"lead white", 1.6}, {"green earth", 0.6}}
p_gh  = pile{{"lead white", 4}, {"chrome yellow", 0.8}, {"yellow ochre", 0.6}, {"green earth", 0.5}}

rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)

ball = function(m, base, lit, tr, sh, refl, cx, cy, r)
  work(m, {pile=base, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.30*r, cy-0.29*r, 0.68*r, 0.64*r):soften(0.28*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.5, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.30*r, cy+0.29*r, 0.84*r, 0.80*r):soften(0.22*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.62*r, cy+0.57*r, 0.70*r, 0.66*r):soften(0.16*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=1.8, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.74*r, 0.22*r):soften(0.11*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  blend(m:soften(4), {angle=0.3})
end

A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
ball(A, p_rb3, p_rl4, p_rtr, p_rsh, p_rrfl2, 500, 486, 86)
ball(C, p_rb3, p_rl4, p_rtr, p_rsh, p_rrfl2, 690, 496, 78)
ball(B, p_gb,  p_gl,  p_gt,  p_gs,  p_gr,  600, 448, 94)
print("fruit in the bowl", wait(0))

--@ chunk 255
p_gb2 = pile{{"green earth", 5}, {"bone black", 1.2}, {"chrome yellow", 0.8}, {"red earth", 0.4}, {"lead white", 1}}
p_gl2 = pile{{"green earth", 3.5}, {"chrome yellow", 1.4}, {"lead white", 1.6}, {"yellow ochre", 0.4}}
p_gt2 = pile{{"green earth", 5}, {"bone black", 2}, {"chrome yellow", 0.5}, {"raw umber", 0.6}}
p_gs2 = pile{{"green earth", 2.2}, {"bone black", 2.8}, {"Prussian blue", 0.6}}
p_gr2 = pile{{"yellow ochre", 2}, {"lead white", 1.6}, {"green earth", 0.8}}
p_gh2 = pile{{"lead white", 4}, {"chrome yellow", 0.6}, {"green earth", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o

ball = function(m, base, lit, tr, sh, refl, cx, cy, r)
  work(m, {pile=base, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.52*r, 0.48*r):soften(0.20*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.4, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.22*r, cy+0.24*r, 0.78*r, 0.74*r):soften(0.20*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.58*r, cy+0.54*r, 0.66*r, 0.62*r):soften(0.14*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.74*r, 0.22*r):soften(0.11*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  blend(m:soften(4), {angle=0.3})
end

-- the rim is too bright and cuts across the fruit; take it down a step
work((rim_o - ellipse(580, 500, 182, 53)) * -nearrim, {pile=p_bo4_mid, tool="filbert 12",
     length={30,64}, coverage=1.4, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rim_o * ellipse(455, 505, 130, 80):soften(44), {pile=p_bo4_lit, tool="filbert 12",
     length={30,64}, coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})

A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
ball(A, p_rbase2, p_rlit3, p_rtr, p_rsh, p_rrfl2, 500, 486, 86)
ball(C, p_rbase2, p_rlit3, p_rtr, p_rsh, p_rrfl2, 690, 496, 78)
ball(B, p_gb2, p_gl2, p_gt2, p_gs2, p_gr2, 600, 448, 94)

E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
work(ellipse(512, 768, 84, 26):soften(22) * cloth_r * -E:grow(6), {pile=p_gs2, tool="filbert 16",
     length={40,90}, coverage=1.8, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
ball(E, p_gb2, p_gl2, p_gt2, p_gs2, p_gr2, 470, 712, 82)
print("fruit 2", wait(0))

--@ chunk 256
p_gb3 = pile{{"green earth", 5}, {"bone black", 1.4}, {"copper green", 1.2}, {"chrome yellow", 0.5}, {"lead white", 1}}
p_gl3 = pile{{"green earth", 3}, {"chrome yellow", 1.2}, {"lead white", 1.6}, {"copper green", 0.4}}
p_gt3 = pile{{"green earth", 4}, {"bone black", 2.4}, {"copper green", 0.8}, {"raw umber", 0.8}}
p_stem2 = pile{{"raw umber", 2}, {"bone black", 1.2}, {"red earth", 0.8}}
p_shw  = pile{{"raw umber", 2.2}, {"bone black", 2}, {"red earth", 0.4}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
rimb   = rim_o - ellipse(580, 500, 182, 53)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the old bowl's wings off the sides, back to cloth
work((ellipse(374, 506, 40, 26):soften(12) + ellipse(788, 504, 40, 26):soften(12))*cloth_r,
     {pile=p_clm4, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0},
      dips={1,1.0,0.0}, clip=true})
-- the bowl's shadow, redone warm and smooth
work(R({{508,622},{650,660},{790,700}}, 96, 42)*cloth_r, {pile=p_shw, tool="filbert 20",
     length={50,100}, coverage=2.2, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{534,626},{648,652}}, 46, 22)*cloth_r, {pile=p_bo4_dk, tool="filbert 16",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
blend((R({{508,622},{650,660},{790,700}}, 96, 42) + R({{534,626},{648,652}}, 46, 22))*cloth_r:soften(20),
      {angle=0.5})
-- the rim down a step, with a narrow light left on the near-left arc only
work(rimb, {pile=p_bo4_mid, tool="filbert 12", length={30,64}, coverage=1.6, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rimb * ellipse(452, 512, 96, 56):soften(34), {pile=p_bo4_lit, tool="filbert 12",
     length={30,64}, coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})

-- the two green apples again, deeper
ball = function(m, base, lit, tr, sh, refl, cx, cy, r)
  work(m, {pile=base, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.52*r, 0.48*r):soften(0.20*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.4, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.22*r, cy+0.24*r, 0.78*r, 0.74*r):soften(0.20*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.58*r, cy+0.54*r, 0.66*r, 0.62*r):soften(0.14*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.74*r, 0.22*r):soften(0.11*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  blend(m:soften(4), {angle=0.3})
end
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
ball(B, p_gb3, p_gl3, p_gt3, p_gs2, p_gr2, 600, 448, 94)
ball(E, p_gb3, p_gl3, p_gt3, p_gs2, p_gr2, 470, 712, 82)

-- stalks
rg = brush{kind="round", width=7, point=0.95}
rg:load(p_stem2, 1.0)
rg:stroke({{503, 414}, {509, 398}, {514, 384}}, {pressure={1.0,0.6,0.05}})
rg:reload(p_stem2, 1.0)
rg:stroke({{605, 368}, {612, 352}, {618, 338}}, {pressure={1.0,0.6,0.05}})
rg:reload(p_stem2, 1.0)
rg:stroke({{474, 644}, {480, 628}, {486, 614}}, {pressure={1.0,0.6,0.05}})
rg:reload(p_stem2, 1.0)
rg:stroke({{692, 430}, {698, 416}, {704, 404}}, {pressure={1.0,0.6,0.05}})
print("fixups", wait(0))

--@ chunk 257
p_bo5_lit = pile{{"raw umber", 2.6}, {"lead white", 2.2}, {"bone black", 0.5}, {"yellow ochre", 0.5}}
p_bo5_mid = pile{{"raw umber", 3.2}, {"bone black", 1.4}, {"lead white", 0.8}, {"red earth", 0.7}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)

-- clean the old shadow smear off the cloth
work(rect(486, 600, 340, 168):soften(30) * cloth_r * -E:grow(10) * -D:grow(10),
     {pile=p_clm4, hand="broad", coverage=2.6, fill=true, load=1.0, pressure={1.0,0.9},
      dips={1,1.0,0.0}, angle=0.4, clip=true})
-- the bowl's form, laid in again
work(wall, {pile=p_bo5_mid, hand="broad", coverage=3.5, fill=true, load=1.0, pressure={1.0,1.0},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
work(wall * ellipse(456, 546, 140, 104):soften(56), {pile=p_bo5_lit, tool="filbert 16",
     length={40,90}, coverage=1.8, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(716, 586, 148, 110):soften(56), {pile=p_bo4_sh, tool="filbert 16",
     length={40,90}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 582 end):soften(20), {pile=p_bo4_dk, tool="filbert 16",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
work(rimb, {pile=p_bo5_mid, tool="filbert 12", length={30,64}, coverage=1.4, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rimb * ellipse(452, 514, 100, 58):soften(32), {pile=p_bo5_lit, tool="filbert 12",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
blend(bowl:soften(5), {angle=0.4})
-- the bowl's shadow: close, at the foot, and away to the lower right
work(R({{560,634},{680,674},{792,708}}, 78, 34)*cloth_r, {pile=p_shw, tool="filbert 20",
     length={50,100}, coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{524,622},{600,638},{660,630}}, 24, 13)*cloth_r, {pile=p_bo4_dk, tool="filbert 12",
     length={30,64}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.2, clip=true})
blend((R({{560,634},{680,674},{792,708}}, 78, 34) + R({{524,622},{600,638},{660,630}}, 24, 13))
      * cloth_r:soften(16), {angle=0.5})
print("bowl 7", wait(0))

--@ chunk 258
p_bo5_lit = pile{{"raw umber", 2.6}, {"lead white", 2.2}, {"bone black", 0.5}, {"yellow ochre", 0.5}}
p_bo5_mid = pile{{"raw umber", 3.2}, {"bone black", 1.4}, {"lead white", 0.8}, {"red earth", 0.7}}
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
fruit = A + C + B
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner

-- the rim again, this time keeping clear of the fruit
rf = rimb - fruit
work(rf, {pile=p_bo5_mid, tool="filbert 12", length={30,64}, coverage=2.0, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(448, 512, 92, 54):soften(30), {pile=p_bo5_lit, tool="filbert 12",
     length={30,64}, coverage=1.2, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(716, 520, 100, 60):soften(34), {pile=p_bo4_sh, tool="filbert 12",
     length={30,64}, coverage=1.5, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
-- the body: a softer turn from the lit shoulder into the shadow
work(wall * ellipse(452, 550, 128, 96):soften(52), {pile=p_bo5_lit, tool="filbert 16",
     length={40,90}, coverage=1.2, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(712, 588, 140, 104):soften(52), {pile=p_bo4_sh, tool="filbert 16",
     length={40,90}, coverage=1.6, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
blend(bowl:soften(5), {angle=0.4})

-- the three fruit once more, on top of the rim
ball = function(m, base, lit, tr, sh, refl, cx, cy, r)
  work(m, {pile=base, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.52*r, 0.48*r):soften(0.20*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.4, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.22*r, cy+0.24*r, 0.78*r, 0.74*r):soften(0.20*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.58*r, cy+0.54*r, 0.66*r, 0.62*r):soften(0.14*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.74*r, 0.22*r):soften(0.11*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  blend(m:soften(4), {angle=0.3})
end
ball(A, p_rbase2, p_rlit3, p_rtr, p_rsh, p_rrfl2, 500, 486, 86)
ball(C, p_rbase2, p_rlit3, p_rtr, p_rsh, p_rrfl2, 690, 496, 78)
ball(B, p_gb3, p_gl3, p_gt3, p_gs2, p_gr2, 600, 448, 94)
print("rim and fruit", wait(0))

--@ chunk 259
p_bo6_lit = pile{{"raw umber", 2.4}, {"red earth", 1}, {"yellow ochre", 0.8}, {"lead white", 2}}
p_bo6_mid = pile{{"raw umber", 3}, {"red earth", 1.6}, {"yellow ochre", 0.5}, {"bone black", 1}, {"lead white", 0.6}}
p_bo6_sh  = pile{{"raw umber", 2.4}, {"red earth", 1}, {"bone black", 2}, {"Prussian blue", 0.3}}
p_bo6_dk  = pile{{"bone black", 3.2}, {"raw umber", 1.4}, {"Prussian blue", 0.5}, {"red earth", 0.3}}
p_rlit5 = pile{{"vermilion", 3.2}, {"red earth", 2}, {"yellow ochre", 0.8}, {"lead white", 0.5}}
p_rtr2 = pile{{"red earth", 3.5}, {"bone black", 2.6}, {"raw umber", 2.4}, {"vermilion", 0.8}}
p_rsh2 = pile{{"bone black", 3.5}, {"raw umber", 1.8}, {"red earth", 0.8}, {"Prussian blue", 0.4}}
p_rhi2 = pile{{"lead white", 4.5}, {"vermilion", 0.7}, {"red earth", 0.3}, {"yellow ochre", 0.3}}
p_glit5= pile{{"green earth", 2.6}, {"chrome yellow", 1.2}, {"lead white", 2.2}, {"copper green", 0.3}}
p_gtr4 = pile{{"green earth", 4}, {"bone black", 2.8}, {"copper green", 0.6}, {"raw umber", 0.8}}
p_gsh4 = pile{{"bone black", 3.2}, {"green earth", 1.6}, {"Prussian blue", 0.8}}
p_ghi3 = pile{{"lead white", 4.5}, {"chrome yellow", 0.5}, {"green earth", 0.5}}
p_rrfl3= pile{{"red earth", 1.4}, {"lead white", 1.1}, {"yellow ochre", 0.5}, {"vermilion", 0.3}}
p_grfl3= pile{{"yellow ochre", 2.2}, {"lead white", 1.4}, {"green earth", 0.7}}

rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + C + B

-- the bowl in warm earthenware
rf = rimb - fruit
work(wall, {pile=p_bo6_mid, hand="broad", coverage=3.2, fill=true, load=1.0, pressure={1.0,1.0},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
work(wall * ellipse(448, 548, 124, 94):soften(50), {pile=p_bo6_lit, tool="filbert 16",
     length={40,90}, coverage=1.5, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(714, 588, 142, 106):soften(52), {pile=p_bo6_sh, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 580 end):soften(20), {pile=p_bo6_dk, tool="filbert 16",
     length={30,64}, coverage=1.5, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
work(rf, {pile=p_bo6_mid, tool="filbert 12", length={30,64}, coverage=2.0, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(446, 510, 88, 52):soften(28), {pile=p_bo6_lit, tool="filbert 12",
     length={30,64}, coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(718, 518, 96, 58):soften(32), {pile=p_bo6_sh, tool="filbert 12",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
blend(bowl:soften(5), {angle=0.4})

ball = function(m, base, lit, tr, sh, refl, cx, cy, r, hi)
  work(m, {pile=base, hand="broad", coverage=4.5, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.52*r, 0.48*r):soften(0.20*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.6, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.22*r, cy+0.24*r, 0.78*r, 0.74*r):soften(0.20*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.8, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.58*r, cy+0.54*r, 0.66*r, 0.62*r):soften(0.14*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.2, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.74*r, 0.22*r):soften(0.11*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  blend(m:soften(4), {angle=0.3})
  work(ellipse(cx-0.40*r, cy-0.47*r, 0.17*r, 0.13*r):soften(0.06*r) * m:grow(6), {pile=hi,
       tool="filbert 7", length={12,22}, coverage=1.1, fill=false, load=1.0,
       pressure={0.7,0.5}, clip=true})
end
ball(A, p_rbase2, p_rlit5, p_rtr2, p_rsh2, p_rrfl3, 500, 486, 86, p_rhi2)
ball(C, p_rbase2, p_rlit5, p_rtr2, p_rsh2, p_rrfl3, 690, 496, 78, p_rhi2)
ball(D, p_rbase2, p_rlit5, p_rtr2, p_rsh2, p_rrfl3, 300, 655, 100, p_rhi2)
ball(B, p_gb3, p_glit5, p_gtr4, p_gsh4, p_grfl3, 600, 448, 94, p_ghi3)
ball(E, p_gb3, p_glit5, p_gtr4, p_gsh4, p_grfl3, 470, 712, 82, p_ghi3)
print("final modelling", wait(0))

--@ chunk 260
p_cal = pile{{"red earth", 2}, {"bone black", 1.4}, {"raw umber", 1.4}}
p_stm = pile{{"raw umber", 1.6}, {"bone black", 1.4}, {"red earth", 0.9}}
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)

spot = function(m, lit, hi, cx, cy, r)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.30*r, 0.24*r):soften(0.08*r) * m, {pile=lit, tool="filbert 12",
       length={20,44}, coverage=2.0, fill=true, load=1.0, pressure={0.9,0.7},
       dips={1,1.0,0.0}, clip=true})
  work(ellipse(hx, hy, 0.19*r, 0.15*r):soften(0.07*r) * m, {pile=hi, tool="filbert 8",
       length={12,26}, coverage=1.2, fill=true, load=1.0, pressure={0.75,0.55},
       dips={1,1.0,0.0}, clip=true})
  work(ellipse(hx+0.03*r, hy-0.02*r, 0.09*r, 0.07*r):soften(0.03*r) * m, {pile=hi,
       tool={kind="round", width=7, point=1}, length={10,20}, coverage=1.0, fill=true,
       load=1.0, pressure={0.5,0.35}, clip=true})
end
spot(A, p_rlit5, p_rhi2, 500, 486, 86)
spot(C, p_rlit5, p_rhi2, 690, 496, 78)
spot(D, p_rlit5, p_rhi2, 300, 655, 100)
spot(B, p_glit5, p_ghi3, 600, 448, 94)
spot(E, p_glit5, p_ghi3, 470, 712, 82)

-- calyx and stalk for each
rg = brush{kind="round", width=8, point=0.9}
cal = function(m, cx, cy, r)
  work(ellipse(cx+0.02*r, cy-0.97*r, 0.15*r, 0.07*r):soften(0.03*r) * m:grow(8),
       {pile=p_cal, tool={kind="round", width=8, point=1}, length={10,20}, coverage=1.2,
        fill=true, load=1.0, pressure={0.6,0.4}, clip=true})
end
cal(A, 500, 486, 86) cal(C, 690, 496, 78) cal(D, 300, 655, 100)
cal(B, 600, 448, 94) cal(E, 470, 712, 82)
rg:load(p_stm, 1.0)
rg:stroke({{504, 410}, {509, 396}, {515, 382}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{605, 360}, {611, 346}, {618, 332}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{694, 424}, {700, 410}, {707, 396}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{304, 564}, {309, 550}, {316, 536}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{474, 638}, {479, 624}, {486, 610}}, {pressure={1.0,0.55,0.05}})
print("highlights and stalks", wait(0))

--@ chunk 261
p_rsh3 = pile{{"red earth", 2}, {"raw umber", 2.4}, {"bone black", 1.8}, {"yellow ochre", 0.3}}
p_gsh5 = pile{{"green earth", 2}, {"raw umber", 1.6}, {"bone black", 2.2}, {"yellow ochre", 0.3}}
p_bo7_mid = pile{{"raw umber", 2.6}, {"red earth", 2.2}, {"yellow ochre", 0.8}, {"bone black", 0.8}}
p_bo7_lit = pile{{"raw umber", 2}, {"red earth", 1.6}, {"yellow ochre", 1.2}, {"lead white", 1.6}}
p_bo7_sh  = pile{{"red earth", 1.8}, {"raw umber", 1.8}, {"bone black", 2}, {"yellow ochre", 0.3}}
p_bint   = pile{{"raw umber", 2}, {"bone black", 1.8}, {"red earth", 0.8}}
p_rrfl4  = pile{{"red earth", 1.6}, {"lead white", 0.7}, {"yellow ochre", 0.5}}
p_grfl4  = pile{{"yellow ochre", 1.8}, {"lead white", 0.8}, {"green earth", 0.6}}
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + C + B

-- the bowl, warm earthenware, and the rim that stays clear of the fruit
rf = rimb - fruit
work(wall, {pile=p_bo7_mid, hand="broad", coverage=2.6, fill=true, load=1.0, pressure={1.0,1.0},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
work(wall * ellipse(448, 546, 120, 90):soften(48), {pile=p_bo7_lit, tool="filbert 16",
     length={40,90}, coverage=1.5, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(714, 588, 140, 104):soften(50), {pile=p_bo7_sh, tool="filbert 16",
     length={40,90}, coverage=1.8, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(rf, {pile=p_bo7_mid, tool="filbert 12", length={30,64}, coverage=2.6, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(444, 508, 84, 48):soften(26), {pile=p_bo7_lit, tool="filbert 12",
     length={30,64}, coverage=1.3, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(722, 516, 92, 54):soften(30), {pile=p_bo7_sh, tool="filbert 12",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(inner * -rim_o:grow(-6), {pile=p_bint, tool="filbert 16", length={40,90}, coverage=1.8,
     fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, clip=true})
blend(bowl:soften(5), {angle=0.4})

-- the cores, warm instead of blue, and the reflected light quieter
core = function(m, sh, refl, cx, cy, r)
  work(m * ellipse(cx+0.58*r, cy+0.54*r, 0.70*r, 0.66*r):soften(0.16*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.72*r, 0.20*r):soften(0.10*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=1.2, fill=true, load=1.0,
       pressure={0.9,0.7}, dips={1,1.0,0.0}, angle=1.5, clip=true})
end
core(A, p_rsh3, p_rrfl4, 500, 486, 86)  core(C, p_rsh3, p_rrfl4, 690, 496, 78)
core(D, p_rsh3, p_rrfl4, 300, 655, 100)
core(B, p_gsh5, p_grfl4, 600, 448, 94)  core(E, p_gsh5, p_grfl4, 470, 712, 82)

-- the beaded highlights: cover them, and lay a soft light patch instead
gloss = function(m, lit, hi, cx, cy, r, glint)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.32*r, 0.26*r):soften(0.09*r) * m, {pile=lit, tool="filbert 12",
       length={24,50}, coverage=2.2, fill=true, load=1.0, pressure={0.9,0.7},
       dips={1,1.0,0.0}, clip=true})
  work(ellipse(hx, hy, 0.21*r, 0.16*r):soften(0.08*r) * m, {pile=hi, tool="filbert 10",
       length={16,32}, coverage=0.9, fill=false, load=1.0, pressure={0.7,0.5},
       dips={1,1.0,0.0}, clip=true})
  if glint then
    fb = brush{kind="round", width=5, point=1}
    fb:load(hi, 0.7)
    fb:touch(hx+0.02*r, hy-0.03*r, {pressure=0.45, drag={2,-1.5}})
  end
end
gloss(A, p_rlit5, p_rhi2, 500, 486, 86, true)  gloss(C, p_rlit5, p_rhi2, 690, 496, 78, false)
gloss(D, p_rlit5, p_rhi2, 300, 655, 100, true)
gloss(B, p_glit5, p_ghi3, 600, 448, 94, true)  gloss(E, p_glit5, p_ghi3, 470, 712, 82, false)
print("warmth", wait(0))

--@ chunk 262
p_rim_mid = pile{{"raw umber", 2.4}, {"red earth", 1.4}, {"bone black", 1.6}, {"yellow ochre", 0.3}}
p_bo8_lit = pile{{"raw umber", 2.4}, {"red earth", 1.8}, {"yellow ochre", 1}, {"lead white", 1}}
p_rrfl5  = pile{{"red earth", 1.4}, {"raw umber", 0.6}, {"lead white", 0.3}}
p_grfl5  = pile{{"green earth", 1.2}, {"yellow ochre", 1}, {"lead white", 0.4}}
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + C + B
rf = rimb - fruit

work(rf, {pile=p_rim_mid, tool="filbert 12", length={30,64}, coverage=2.0, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(444, 508, 84, 48):soften(26), {pile=p_bo8_lit, tool="filbert 12",
     length={30,64}, coverage=1.2, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(wall * ellipse(448, 546, 118, 88):soften(46), {pile=p_bo8_lit, tool="filbert 16",
     length={40,90}, coverage=1.0, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
blend(bowl:soften(5), {angle=0.4})

-- quieter bounce light along the bottoms
qf = function(m, p, cx, cy, r)
  work(m * ellipse(cx+0.04*r, cy+0.89*r, 0.72*r, 0.20*r):soften(0.10*r), {pile=p,
       tool="filbert 12", length={30,64}, coverage=1.5, fill=true, load=1.0,
       pressure={0.9,0.7}, dips={1,1.0,0.0}, angle=1.5, clip=true})
end
qf(A, p_rrfl5, 500, 486, 86) qf(C, p_rrfl5, 690, 496, 78) qf(D, p_rrfl5, 300, 655, 100)
qf(B, p_grfl5, 600, 448, 94) qf(E, p_grfl5, 470, 712, 82)

-- the beading: cover it with the fruit's own body colour, then one soft touch
sb = brush{kind="round", width=30, point=0.25, hair=0.9, stiffness=0.35, splay=0.25}
gloss = function(m, base, hi, cx, cy, r, glint)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.34*r, 0.28*r):soften(0.10*r) * m, {pile=base, tool="filbert 14",
       length={24,50}, coverage=3.0, fill=true, load=1.0, pressure={1.0,0.85},
       dips={1,1.0,0.0}, clip=true})
  sb:load(hi, 0.3)
  sb:touch(hx, hy, {pressure=0.3, drag={0.10*r, -0.07*r}, twist=0.2})
  if glint then
    fb = brush{kind="round", width=4, point=1}
    fb:load(hi, 0.75)
    fb:touch(hx+0.03*r, hy-0.04*r, {pressure=0.5, drag={1.5,-1}})
  end
end
gloss(A, p_rbase2, p_rhi2, 500, 486, 86, true)  gloss(C, p_rbase2, p_rhi2, 690, 496, 78, false)
gloss(D, p_rbase2, p_rhi2, 300, 655, 100, true)
gloss(B, p_gb3, p_ghi3, 600, 448, 94, true)  gloss(E, p_gb3, p_ghi3, 470, 712, 82, false)
print("gloss", wait(0))

--@ chunk 263
p_rhi3 = pile{{"lead white", 2}, {"vermilion", 1}, {"red earth", 0.8}, {"yellow ochre", 0.3}}
p_ghi4 = pile{{"lead white", 2.5}, {"green earth", 1.2}, {"chrome yellow", 0.4}, {"yellow ochre", 0.4}}
p_rrfl6 = pile{{"red earth", 1.5}, {"raw umber", 1}, {"lead white", 0.3}, {"vermilion", 0.2}}
p_grfl6 = pile{{"green earth", 1.4}, {"yellow ochre", 0.8}, {"raw umber", 0.5}}
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)

-- cover the old highlight and bounce, then lay them again small and quiet
fixup = function(m, base, lit, hi, refl, cx, cy, r, glint)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.40*r, 0.34*r):soften(0.10*r) * m, {pile=base, tool="filbert 14",
       length={24,50}, coverage=3.0, fill=true, load=1.0, pressure={1.0,0.85},
       dips={1,1.0,0.0}, clip=true})
  work(ellipse(hx, hy, 0.26*r, 0.20*r):soften(0.08*r) * m, {pile=lit, tool="filbert 12",
       length={18,36}, coverage=2.0, fill=true, load=1.0, pressure={0.9,0.7},
       dips={1,1.0,0.0}, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.90*r, 0.76*r, 0.24*r):soften(0.10*r), {pile=base,
       tool="filbert 14", length={24,50}, coverage=2.6, fill=true, load=1.0,
       pressure={1.0,0.85}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.90*r, 0.66*r, 0.16*r):soften(0.07*r), {pile=refl,
       tool="filbert 12", length={18,36}, coverage=1.0, fill=true, load=1.0,
       pressure={0.8,0.6}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  s1 = brush{kind="round", width=17, point=0.3, hair=0.9, stiffness=0.4}
  s1:load(hi, 0.3)
  s1:touch(hx, hy, {pressure=0.32, drag={0.07*r, -0.05*r}})
  if glint then
    s2 = brush{kind="round", width=3.5, point=1}
    s2:load(hi, 0.8)
    s2:touch(hx+0.02*r, hy-0.03*r, {pressure=0.5})
  end
end
fixup(A, p_rbase2, p_rlit5, p_rhi3, p_rrfl6, 500, 486, 86, true)
fixup(C, p_rbase2, p_rlit5, p_rhi3, p_rrfl6, 690, 496, 78, false)
fixup(D, p_rbase2, p_rlit5, p_rhi3, p_rrfl6, 300, 655, 100, true)
fixup(B, p_gb3, p_glit5, p_ghi4, p_grfl6, 600, 448, 94, true)
fixup(E, p_gb3, p_glit5, p_ghi4, p_grfl6, 470, 712, 82, false)
print("gloss 2", wait(0))

--@ chunk 264
p_glit6 = pile{{"green earth", 3}, {"chrome yellow", 1}, {"lead white", 1.2}, {"yellow ochre", 0.4}}
p_rlit6 = pile{{"vermilion", 3}, {"red earth", 2.2}, {"yellow ochre", 0.7}, {"lead white", 0.3}}
p_sheen_r = pile{{"yellow ochre", 2.5}, {"vermilion", 1}, {"red earth", 1}}
p_sheen_g = pile{{"yellow ochre", 2}, {"chrome yellow", 1}, {"green earth", 1}}
p_rf7 = pile{{"red earth", 1.6}, {"raw umber", 0.8}, {"lead white", 0.2}}
p_rf8 = pile{{"green earth", 1.4}, {"yellow ochre", 0.8}, {"raw umber", 0.4}}
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)

calm = function(m, base, lit, sheen, refl, cx, cy, r)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.44*r, 0.36*r):soften(0.12*r) * m, {pile=base, tool="filbert 14",
       length={24,50}, coverage=3.0, fill=true, load=1.0, pressure={1.0,0.85},
       dips={1,1.0,0.0}, clip=true})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.52*r, 0.48*r):soften(0.20*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.4, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.90*r, 0.78*r, 0.26*r):soften(0.11*r), {pile=base,
       tool="filbert 14", length={24,50}, coverage=2.8, fill=true, load=1.0,
       pressure={1.0,0.85}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.90*r, 0.64*r, 0.15*r):soften(0.06*r), {pile=refl,
       tool="filbert 12", length={18,36}, coverage=0.7, fill=true, load=1.0,
       pressure={0.75,0.55}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  s = brush{kind="round", width=15, point=0.35, hair=0.9, stiffness=0.45}
  s:load(sheen, 0.75)
  s:touch(hx, hy, {pressure=0.5, drag={0.06*r, -0.04*r}})
end
calm(A, p_rbase2, p_rlit6, p_sheen_r, p_rf7, 500, 486, 86)
calm(C, p_rbase2, p_rlit6, p_sheen_r, p_rf7, 690, 496, 78)
calm(D, p_rbase2, p_rlit6, p_sheen_r, p_rf7, 300, 655, 100)
calm(B, p_gb3, p_glit6, p_sheen_g, p_rf8, 600, 448, 94)
calm(E, p_gb3, p_glit6, p_sheen_g, p_rf8, 470, 712, 82)
print("calm", wait(0))

--@ chunk 265
p_rlit7 = pile{{"vermilion", 3}, {"red earth", 2.2}, {"yellow ochre", 0.7}, {"lead white", 0.3}}
p_glit7 = pile{{"green earth", 3}, {"chrome yellow", 1}, {"lead white", 1.2}, {"yellow ochre", 0.4}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the sheen, laid as a denser patch of the fruit's own light colour
sheen = function(m, lit, cx, cy, r)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.46*r, 0.38*r):soften(0.12*r) * m, {pile=lit, tool="filbert 14",
       length={24,50}, coverage=2.4, fill=true, load=1.0, pressure={1.0,0.85},
       dips={1,1.0,0.0}, clip=true})
  s = brush{kind="round", width=14, point=0.4, hair=0.9, stiffness=0.5}
  s:load(lit, 1.0)
  s:touch(hx, hy, {pressure=0.55, drag={0.05*r, -0.035*r}})
end
sheen(A, p_rlit7, 500, 486, 86)  sheen(C, p_rlit7, 690, 496, 78)  sheen(D, p_rlit7, 300, 655, 100)
sheen(B, p_glit7, 600, 448, 94)  sheen(E, p_glit7, 470, 712, 82)

-- the cloth: knocked to one value, then its folds laid cleanly on top
work(cloth_r, {pile=p_clm4, hand="broad", coverage=0.8, fill=true, load=1.0, pressure={1.0,0.9},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
zone = function(m, p, a, cov, t) work(m*cloth_r, {pile=p, tool=t or "filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
-- lights: the rolls and ridges that face the window
zone(R({{50,586},{300,552},{560,522},{700,514}}, 60, 30), p_cl_lite, -0.08, 1.4)
zone(R({{62,604},{148,700},{228,792}}, 56, 28), p_cl_lite, 1.15, 1.4)
zone(R({{300,572},{252,650},{202,730}}, 50, 26), p_cl_lite, 1.05, 1.3)
zone(R({{336,742},{398,792}}, 44, 22), p_cl_lite, 0.5, 1.2)
zone(R({{196,778},{380,792},{520,796}}, 40, 22), p_cl_lite, 0.1, 1.2)
zone(R({{788,578},{848,640},{870,700}}, 46, 24), p_cl_lite, 0.75, 1.2)
-- darks: the hollows between them
zone(R({{180,620},{150,700},{128,772}}, 46, 24), p_cl_dk, 1.2, 1.5)
zone(ellipse(14, 802, 110, 58):soften(30), p_cl_dk, 0.3, 1.5)
zone(R({{392,700},{404,780}}, 42, 22), p_cl_dk, 1.4, 1.4)
zone(R({{856,566},{880,652}}, 44, 22), p_cl_dk, 1.5, 1.4)
zone(ellipse(800, 792, 150, 62):soften(32), p_cl_dk, 0.2, 1.5)
zone(R({{240,752},{214,798}}, 34, 18), p_cl_dk, 1.0, 1.3)
print("sheen and cloth", wait(0))

--@ chunk 266
p_cl_lite2 = pile{{"lead white", 5}, {"raw umber", 2.5}, {"yellow ochre", 0.8}, {"red earth", 0.5}}
p_rlit8 = pile{{"vermilion", 2.4}, {"red earth", 2.6}, {"yellow ochre", 0.8}, {"raw umber", 0.8}}
p_glit8 = pile{{"green earth", 3.2}, {"chrome yellow", 0.9}, {"lead white", 0.9}, {"yellow ochre", 0.4}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the glowing sheen patches: back to the fruit's own colour, and no separate highlight
plain = function(m, base, lit, cx, cy, r)
  local hx, hy = cx-0.40*r, cy-0.47*r
  work(ellipse(hx, hy, 0.50*r, 0.42*r):soften(0.14*r) * m, {pile=base, tool="filbert 16",
       length={26,54}, coverage=3.2, fill=true, load=1.0, pressure={1.0,0.85},
       dips={1,1.0,0.0}, clip=true})
  work(m * ellipse(cx-0.32*r, cy-0.31*r, 0.56*r, 0.52*r):soften(0.22*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
end
plain(A, p_rbase2, p_rlit8, 500, 486, 86)  plain(C, p_rbase2, p_rlit8, 690, 496, 78)
plain(D, p_rbase2, p_rlit8, 300, 655, 100)
plain(B, p_gb3, p_glit8, 600, 448, 94)  plain(E, p_gb3, p_glit8, 470, 712, 82)

-- the cloth: the white worms back to a soft grey, the folds laid from that
work(cloth_r, {pile=p_clm4, hand="broad", coverage=1.5, fill=true, load=1.0, pressure={1.0,0.9},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
zone = function(m, p, a, cov) work(m*cloth_r, {pile=p, tool="filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zone(R({{50,586},{300,552},{560,522},{700,514}}, 60, 30), p_cl_lite2, -0.08, 1.2)
zone(R({{62,604},{148,700},{228,792}}, 56, 28), p_cl_lite2, 1.15, 1.2)
zone(R({{300,572},{252,650},{202,730}}, 50, 26), p_cl_lite2, 1.05, 1.1)
zone(R({{336,742},{398,792}}, 44, 22), p_cl_lite2, 0.5, 1.0)
zone(R({{196,778},{380,792},{520,796}}, 40, 22), p_cl_lite2, 0.1, 1.0)
zone(R({{788,578},{848,640},{870,700}}, 46, 24), p_cl_lite2, 0.75, 1.0)
zone(R({{180,620},{150,700},{128,772}}, 46, 24), p_cl_dk, 1.2, 1.4)
zone(ellipse(14, 802, 110, 58):soften(30), p_cl_dk, 0.3, 1.4)
zone(R({{392,700},{404,780}}, 42, 22), p_cl_dk, 1.4, 1.3)
zone(R({{856,566},{880,652}}, 44, 22), p_cl_dk, 1.5, 1.3)
zone(ellipse(800, 792, 150, 62):soften(32), p_cl_dk, 0.2, 1.4)
print("unify", wait(0))

--@ chunk 267
p_cl_lite3 = pile{{"lead white", 4.2}, {"raw umber", 2.8}, {"yellow ochre", 0.8}, {"red earth", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the cloth, where it is not covered by the bowl or the fruit
work(free, {pile=p_clm4, hand="broad", coverage=1.2, fill=true, load=1.0, pressure={1.0,0.9},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
zone = function(m, p, a, cov) work(m*free, {pile=p, tool="filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zone(R({{40,592},{290,556},{430,536}}, 54, 28), p_cl_lite3, -0.08, 1.1)
zone(R({{62,604},{148,700},{228,792}}, 52, 26), p_cl_lite3, 1.15, 1.1)
zone(R({{196,778},{380,792},{520,796}}, 38, 20), p_cl_lite3, 0.1, 1.0)
zone(R({{790,578},{848,640},{870,700}}, 44, 22), p_cl_lite3, 0.75, 1.0)
zone(R({{180,620},{150,700},{128,772}}, 44, 22), p_cl_dk, 1.2, 1.2)
zone(ellipse(14, 802, 108, 56):soften(28), p_cl_dk, 0.3, 1.3)
zone(R({{392,700},{404,780}}, 40, 20), p_cl_dk, 1.4, 1.2)
zone(R({{856,566},{880,652}}, 42, 20), p_cl_dk, 1.5, 1.2)
zone(ellipse(800, 792, 148, 60):soften(30), p_cl_dk, 0.2, 1.3)

-- the bowl
rf = rimb - fruit
work(wall, {pile=p_bo7_mid, hand="broad", coverage=2.6, fill=true, load=1.0, pressure={1.0,1.0},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
work(wall * ellipse(448, 546, 118, 88):soften(46), {pile=p_bo8_lit, tool="filbert 16",
     length={40,90}, coverage=1.1, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(714, 588, 140, 104):soften(50), {pile=p_bo7_sh, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 580 end):soften(20), {pile=p_bo6_dk, tool="filbert 16",
     length={30,64}, coverage=1.3, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
work(rf, {pile=p_rim_mid, tool="filbert 12", length={30,64}, coverage=2.2, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(444, 508, 84, 48):soften(26), {pile=p_bo8_lit, tool="filbert 12",
     length={30,64}, coverage=1.2, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(inner * -(rim_o:grow(-4)), {pile=p_bint, tool="filbert 16", length={40,90}, coverage=2.0,
     fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, clip=true})
blend(bowl:soften(5), {angle=0.4})

-- the five fruit, the way that reads
ball = function(m, base, lit, tr, sh, refl, cx, cy, r)
  work(m, {pile=base, hand="broad", coverage=4.0, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.32*r, cy-0.31*r, 0.56*r, 0.52*r):soften(0.22*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.3, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.22*r, cy+0.24*r, 0.78*r, 0.74*r):soften(0.20*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.56*r, cy+0.52*r, 0.68*r, 0.64*r):soften(0.15*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.90*r, 0.72*r, 0.20*r):soften(0.10*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=0.9, fill=true, load=1.0,
       pressure={0.8,0.6}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  blend(m:soften(4), {angle=0.3})
end
ball(A, p_rbase2, p_rlit8, p_rtr2, p_rsh3, p_rrfl6, 500, 486, 86)
ball(C, p_rbase2, p_rlit8, p_rtr2, p_rsh3, p_rrfl6, 690, 496, 78)
ball(D, p_rbase2, p_rlit8, p_rtr2, p_rsh3, p_rrfl6, 300, 655, 100)
ball(B, p_gb3, p_glit8, p_gtr4, p_gsh5, p_grfl6, 600, 448, 94)
ball(E, p_gb3, p_glit8, p_gtr4, p_gsh5, p_grfl6, 470, 712, 82)
print("group", wait(0))

--@ chunk 268
p_shc = pile{{"raw umber", 2}, {"bone black", 1.6}, {"red earth", 0.5}, {"Prussian blue", 0.2}}
p_cl_lite4 = pile{{"lead white", 4.2}, {"raw umber", 2.8}, {"yellow ochre", 0.8}, {"red earth", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the shadows the fruit throw, away from the light, on the cloth only
zone = function(m, p, a, cov) work(m*free, {pile=p, tool="filbert 16", length={40,90},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zone(R({{340,714},{424,748}}, 52, 24), p_shc, 0.5, 1.8)
zone(R({{352,728},{468,772}}, 88, 36), p_shc, 0.5, 1.4)
zone(R({{508,760},{584,792}}, 46, 22), p_shc, 0.5, 1.8)
zone(R({{520,772},{624,802}}, 80, 34), p_shc, 0.5, 1.4)
zone(R({{566,636},{684,676},{796,710}}, 82, 36), p_shc, 0.5, 1.6)
zone(R({{524,620},{600,636},{664,628}}, 26, 14), p_shc, 0.2, 2.0)
blend((R({{340,714},{424,748}}, 52, 24) + R({{508,760},{584,792}}, 46, 22) +
       R({{524,620},{600,636},{664,628}}, 26, 14))*free:soften(16), {angle=0.5})

-- the folds of the cloth, on the left and along the front
zone(R({{150,606},{198,676},{232,742}}, 40, 20), p_cl_dk, 1.0, 1.4)
zone(ellipse(24, 792, 88, 48):soften(24), p_cl_dk, 0.3, 1.4)
zone(R({{402,716},{430,772}}, 34, 17), p_cl_dk, 1.4, 1.3)
zone(R({{60,632},{112,702},{166,778}}, 44, 22), p_cl_lite4, 1.1, 1.1)
zone(R({{140,772},{280,790}}, 36, 18), p_cl_lite4, 0.15, 1.0)
zone(R({{40,600},{180,568}}, 38, 20), p_cl_lite4, -0.2, 1.0)
print("shadows and folds", wait(0))

--@ chunk 269
p_shc  = pile{{"raw umber", 2}, {"bone black", 1.6}, {"red earth", 0.5}, {"Prussian blue", 0.2}}
p_shd  = pile{{"raw umber", 2.4}, {"bone black", 1.2}, {"red earth", 0.6}, {"Prussian blue", 0.2}}
p_cl_lite4 = pile{{"lead white", 4.2}, {"raw umber", 2.8}, {"yellow ochre", 0.8}, {"red earth", 0.6}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl
R = function(pts, w, s) return ribbon(pts, w):soften(s) end

-- the tight shadow each fruit sits in, right at its base
contact = function(m, cx, cy, r, cov)
  local c = ((m:grow(11):soften(9) - m) * ellipse(cx+0.10*r, cy+0.80*r, 1.05*r, 0.55*r)
             * free)
  work(c, {pile=p_shc, tool="filbert 16", length={30,64}, coverage=cov, fill=true, load=1.0,
           pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=0.4, clip=true})
  return c
end
c1 = contact(D, 300, 655, 100, 2.0)
c2 = contact(E, 470, 712, 82, 2.0)
c3 = contact(A, 500, 486, 86, 1.6)
c4 = contact(B, 600, 448, 94, 1.6)
c5 = contact(C, 690, 496, 78, 1.6)
blend((c1+c2+c5):soften(14), {angle=0.5})
-- and the bowl's, along the foot and away to the right
work(R({{586,632},{700,670},{812,704}}, 76, 32)*free, {pile=p_shd, tool="filbert 20",
     length={50,100}, coverage=1.8, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{520,618},{600,634},{668,626}}, 24, 13)*free, {pile=p_shc, tool="filbert 12",
     length={30,64}, coverage=2.2, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.2, clip=true})

-- the cloth's folds: mid shadows, not soot
zone = function(m, p, a, cov) work(m*free, {pile=p, tool="filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zone(R({{150,606},{198,676},{232,742}}, 40, 20), p_shd, 1.0, 1.6)
zone(ellipse(24, 792, 88, 48):soften(24), p_shd, 0.3, 1.6)
zone(R({{402,716},{430,772}}, 34, 17), p_shd, 1.4, 1.5)
zone(R({{60,632},{112,702},{166,778}}, 44, 22), p_cl_lite4, 1.1, 1.2)
zone(R({{140,772},{280,790}}, 36, 18), p_cl_lite4, 0.15, 1.1)
zone(R({{40,600},{180,568}}, 38, 20), p_cl_lite4, -0.2, 1.1)
zone(R({{800,600},{852,660}}, 40, 20), p_cl_lite4, 0.8, 1.0)
print("contact shadows", wait(0))

--@ chunk 270
p_rsh4 = pile{{"red earth", 1.8}, {"raw umber", 2.2}, {"bone black", 2}, {"yellow ochre", 0.3}}
p_gsh6 = pile{{"green earth", 1.8}, {"raw umber", 1.5}, {"bone black", 2.4}, {"yellow ochre", 0.3}}
p_edge_r= pile{{"yellow ochre", 2.5}, {"vermilion", 0.8}, {"red earth", 0.8}}
p_edge_g= pile{{"yellow ochre", 2}, {"chrome yellow", 0.8}, {"green earth", 0.8}}
p_shc2 = pile{{"raw umber", 1.8}, {"bone black", 1.8}, {"red earth", 0.4}, {"Prussian blue", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl

-- each fruit gets a proper dark side
deep = function(m, sh, cx, cy, r)
  work(m * ellipse(cx+0.50*r, cy+0.46*r, 0.72*r, 0.68*r):soften(0.16*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
end
deep(A, p_rsh4, 500, 486, 86)  deep(C, p_rsh4, 690, 496, 78)  deep(D, p_rsh4, 300, 655, 100)
deep(B, p_gsh6, 600, 448, 94)  deep(E, p_gsh6, 470, 712, 82)

-- a warm edge-light on the lit side of each fruit
edge = function(m, p, cx, cy, r)
  local e = (m:grow(3) - m:shrink(3)) * ellipse(cx-0.62*r, cy-0.62*r, 0.92*r, 0.92*r)
  work(e, {pile=p, tool="filbert 10", length={16,36}, coverage=1.3, fill=true, load=1.0,
           pressure={0.9,0.7}, dips={1,1.0,0.0}, angle=0.6, clip=true})
end
edge(A, p_edge_r, 500, 486, 86)  edge(C, p_edge_r, 690, 496, 78)  edge(D, p_edge_r, 300, 655, 100)
edge(B, p_edge_g, 600, 448, 94)  edge(E, p_edge_g, 470, 712, 82)

-- the contact shadows, wider and darker
contact = function(m, cx, cy, r)
  work((m:grow(17):soften(11) - m) * ellipse(cx+0.12*r, cy+0.82*r, 1.1*r, 0.6*r) * free,
       {pile=p_shc2, tool="filbert 16", length={30,64}, coverage=2.2, fill=true, load=1.0,
        pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=0.4, clip=true})
end
contact(D, 300, 655, 100)  contact(E, 470, 712, 82)
contact(A, 500, 486, 86)   contact(B, 600, 448, 94)  contact(C, 690, 496, 78)
print("deep, edge light, contact", wait(0))

--@ chunk 271
p_rlit8 = pile{{"vermilion", 2.4}, {"red earth", 2.6}, {"yellow ochre", 0.8}, {"raw umber", 0.8}}
p_glit8 = pile{{"green earth", 3.2}, {"chrome yellow", 0.9}, {"lead white", 0.9}, {"yellow ochre", 0.4}}
p_edge_r= pile{{"yellow ochre", 2.5}, {"vermilion", 0.8}, {"red earth", 0.8}}
p_edge_g= pile{{"yellow ochre", 2}, {"chrome yellow", 0.8}, {"green earth", 0.8}}
p_shc2 = pile{{"raw umber", 1.8}, {"bone black", 1.8}, {"red earth", 0.4}, {"Prussian blue", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(4, 45, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(3, 40, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(5, 55, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(4, 50, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 50, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl

-- the hard edge-light lines back to a soft turn of the light
soft = function(m, base, lit, edge, cx, cy, r)
  work(m:grow(4), {pile=base, hand="broad", coverage=1.2, fill=true, load=1.0, pressure={1.0,0.9},
       dips={1,1.0,0.0}, angle=0.3, clip=m:grow(4)})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.64*r, 0.60*r):soften(0.28*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.2, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work((m:grow(2) - m) * ellipse(cx-0.58*r, cy-0.58*r, 0.74*r, 0.74*r), {pile=edge,
       tool="filbert 12", length={18,40}, coverage=0.5, fill=true, load=1.0,
       pressure={0.8,0.6}, dips={1,1.0,0.0}, angle=0.6, clip=true})
end
soft(A, p_rbase2, p_rlit8, p_edge_r, 500, 486, 86)
soft(C, p_rbase2, p_rlit8, p_edge_r, 690, 496, 78)
soft(D, p_rbase2, p_rlit8, p_edge_r, 300, 655, 100)
soft(B, p_gb3, p_glit8, p_edge_g, 600, 448, 94)
soft(E, p_gb3, p_glit8, p_edge_g, 470, 712, 82)

-- the contact shadows: a pool just under each fruit, not a ring round it
fixc = function(m, cx, cy, r)
  work((m:grow(20):soften(12) - m) * ellipse(cx+0.12*r, cy+0.80*r, 1.1*r, 0.62*r) * free,
       {pile=p_clm4, hand="broad", coverage=1.4, fill=true, load=1.0, pressure={1.0,0.9},
        dips={1,1.0,0.0}, angle=0.4, clip=true})
  work(ellipse(cx+0.22*r, cy+0.98*r, 0.62*r, 0.26*r):soften(11) * free, {pile=p_shc2,
       tool="filbert 16", length={30,64}, coverage=2.0, fill=true, load=1.0,
       pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(ellipse(cx+0.34*r, cy+1.06*r, 0.9*r, 0.34*r):soften(16) * free, {pile=p_shc2,
       tool="filbert 20", length={40,90}, coverage=1.3, fill=true, load=1.0,
       pressure={0.85,0.7}, dips={1,1.0,0.0}, angle=0.4, clip=true})
end
fixc(D, 300, 655, 100)  fixc(E, 470, 712, 82)
print("soften", wait(0))

--@ chunk 272
p_rlit9 = pile{{"vermilion", 2.2}, {"red earth", 2.8}, {"yellow ochre", 0.7}, {"raw umber", 1}}
p_rtr3 = pile{{"red earth", 3.2}, {"bone black", 2.4}, {"raw umber", 2.4}, {"vermilion", 0.7}}
p_rsh5 = pile{{"red earth", 1.6}, {"raw umber", 2.4}, {"bone black", 2.2}, {"yellow ochre", 0.3}}
p_glit9= pile{{"green earth", 3.2}, {"chrome yellow", 0.8}, {"lead white", 0.8}, {"yellow ochre", 0.3}}
p_gtr5 = pile{{"green earth", 3.6}, {"bone black", 2.6}, {"copper green", 0.5}, {"raw umber", 1}}
p_gsh7 = pile{{"green earth", 1.6}, {"raw umber", 1.6}, {"bone black", 2.6}, {"yellow ochre", 0.3}}
p_rrf9 = pile{{"red earth", 1.5}, {"raw umber", 0.8}, {"lead white", 0.2}}
p_grf9 = pile{{"green earth", 1.3}, {"yellow ochre", 0.7}, {"raw umber", 0.4}}
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(5, 45, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 45, 41)

ball = function(m, base, lit, tr, sh, refl, cx, cy, r)
  work(m, {pile=base, hand="broad", coverage=2.2, fill=true, load=1.0, pressure={1.0,1.0},
           dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx-0.34*r, cy-0.33*r, 0.62*r, 0.58*r):soften(0.28*r), {pile=lit,
       tool="filbert 16", length={40,90}, coverage=1.4, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.20*r, cy+0.22*r, 0.80*r, 0.76*r):soften(0.22*r), {pile=tr,
       tool="filbert 16", length={40,90}, coverage=1.7, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.52*r, cy+0.48*r, 0.72*r, 0.68*r):soften(0.16*r), {pile=sh,
       tool="filbert 16", length={40,90}, coverage=2.0, fill=true, load=1.0,
       pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(m * ellipse(cx+0.04*r, cy+0.90*r, 0.70*r, 0.20*r):soften(0.10*r), {pile=refl,
       tool="filbert 12", length={30,64}, coverage=0.9, fill=true, load=1.0,
       pressure={0.8,0.6}, dips={1,1.0,0.0}, angle=1.5, clip=true})
  -- the shadow-side edge lets go into the dark behind it
  work((m:grow(7):soften(6) - m) * ellipse(cx+0.62*r, cy+0.58*r, 0.95*r, 0.95*r), {pile=sh,
       tool="filbert 14", length={20,44}, coverage=0.7, fill=true, load=1.0,
       pressure={0.8,0.6}, dips={1,1.0,0.0}, edge="soft", clip=true})
  blend(m:soften(4), {angle=0.3})
end
ball(A, p_rbase2, p_rlit9, p_rtr3, p_rsh5, p_rrf9, 500, 486, 86)
ball(C, p_rbase2, p_rlit9, p_rtr3, p_rsh5, p_rrf9, 690, 496, 78)
ball(D, p_rbase2, p_rlit9, p_rtr3, p_rsh5, p_rrf9, 300, 655, 100)
ball(B, p_gb3, p_glit9, p_gtr5, p_gsh7, p_grf9, 600, 448, 94)
ball(E, p_gb3, p_glit9, p_gtr5, p_gsh7, p_grf9, 470, 712, 82)
print("clean fruit", wait(0))

--@ chunk 273
p_stm  = pile{{"raw umber", 1.6}, {"bone black", 1.4}, {"red earth", 0.9}}
p_gzw  = pile{{"raw umber", 2.5}, {"bone black", 1.2}, {"red earth", 0.5}, medium=0.3}
p_gzc  = pile{{"bone black", 2.5}, {"raw umber", 1.5}, {"Prussian blue", 0.5}, medium=0.35}
p_cl_mid5 = pile{{"lead white", 3.4}, {"raw umber", 3}, {"yellow ochre", 0.7}, {"red earth", 0.6}}
p_cl_lit5 = pile{{"lead white", 5}, {"raw umber", 2.6}, {"yellow ochre", 0.8}, {"red earth", 0.5}}
p_shd2 = pile{{"raw umber", 2.2}, {"bone black", 1.5}, {"red earth", 0.5}, {"Prussian blue", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(5, 45, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 45, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
tline = function(x) return 468 - 0.04*x end

-- the smudges on the cloth back to cloth, then real folds
work((R({{140,596},{192,668},{228,742}}, 46, 24) + R({{396,706},{428,776}}, 40, 20) +
      ellipse(20, 792, 92, 50):soften(26) + R({{852,562},{878,650}}, 44, 22))*free,
     {pile=p_cl_mid5, hand="broad", coverage=1.4, fill=true, load=1.0, pressure={1.0,0.9},
      dips={1,1.0,0.0}, angle=0.5, clip=true})
zone = function(m, p, a, cov) work(m*free, {pile=p, tool="filbert 20", length={50,100},
        coverage=cov, fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=a, clip=true}) end
zone(R({{56,626},{108,700},{162,782}}, 40, 20), p_cl_lit5, 1.1, 1.2)
zone(R({{148,772},{290,790}}, 34, 18), p_cl_lit5, 0.15, 1.1)
zone(R({{34,596},{176,566}}, 36, 18), p_cl_lit5, -0.2, 1.1)
zone(R({{796,596},{850,658}}, 38, 20), p_cl_lit5, 0.8, 1.1)
zone(R({{146,602},{196,672},{232,748}}, 34, 17), p_shd2, 1.0, 1.3)
zone(ellipse(18, 794, 82, 44):soften(22), p_shd2, 0.3, 1.3)
zone(R({{400,714},{430,778}}, 30, 15), p_shd2, 1.4, 1.2)
zone(R({{858,566},{880,652}}, 34, 17), p_shd2, 1.5, 1.2)

-- the fruit sit in a shadow again
fixc = function(m, cx, cy, r)
  work(ellipse(cx+0.20*r, cy+0.97*r, 0.66*r, 0.28*r):soften(12) * free, {pile=p_shd2,
       tool="filbert 16", length={30,64}, coverage=2.0, fill=true, load=1.0,
       pressure={0.9,0.75}, dips={1,1.0,0.0}, angle=0.3, clip=true})
  work(ellipse(cx+0.32*r, cy+1.05*r, 0.95*r, 0.36*r):soften(17) * free, {pile=p_shd2,
       tool="filbert 20", length={40,90}, coverage=1.3, fill=true, load=1.0,
       pressure={0.85,0.7}, dips={1,1.0,0.0}, angle=0.4, clip=true})
end
fixc(D, 300, 655, 100)  fixc(E, 470, 712, 82)
fixc(A, 500, 486, 86)   fixc(B, 600, 448, 94)  fixc(C, 690, 496, 78)

-- the stalks, laid last so nothing covers them
rg = brush{kind="round", width=8, point=0.9}
rg:load(p_stm, 1.0); rg:stroke({{504, 410}, {509, 396}, {515, 382}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{605, 358}, {611, 344}, {618, 330}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{694, 424}, {700, 410}, {707, 396}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{304, 562}, {309, 548}, {316, 534}}, {pressure={1.0,0.55,0.05}})
rg:reload(p_stm, 1.0); rg:stroke({{474, 636}, {479, 622}, {486, 608}}, {pressure={1.0,0.55,0.05}})

-- a warm glaze over the group, a cool veil over the room behind it
work(bowl + fruit, {hand="glaze", pile=p_gzw, coverage=1.0, angle=0.4, clip=true})
work(rect(0, 0, 1000, 476):soften(30) - bowl - fruit, {hand="glaze", pile=p_gzc,
     coverage=0.9, angle=0.9, clip=true})
work(below(tline):soften(10) - cloth_r, {hand="glaze", pile=p_gzc, coverage=0.8,
     angle=0.25, clip=true})
print("finish", wait(0))

--@ chunk 274
p_bo7_mid = pile{{"raw umber", 2.6}, {"red earth", 2.2}, {"yellow ochre", 0.8}, {"bone black", 0.8}}
p_bo8_lit = pile{{"raw umber", 2.4}, {"red earth", 1.8}, {"yellow ochre", 1}, {"lead white", 1}}
p_bo7_sh  = pile{{"red earth", 1.8}, {"raw umber", 1.8}, {"bone black", 2}, {"yellow ochre", 0.3}}
p_bo6_dk  = pile{{"bone black", 3.2}, {"raw umber", 1.4}, {"Prussian blue", 0.5}, {"red earth", 0.3}}
p_bint   = pile{{"raw umber", 2}, {"bone black", 1.8}, {"red earth", 0.8}}
p_shd3   = pile{{"raw umber", 2.2}, {"bone black", 1.6}, {"red earth", 0.5}, {"Prussian blue", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(5, 45, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 45, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl
R = function(pts, w, s) return ribbon(pts, w):soften(s) end
rf = rimb - fruit

work(wall, {pile=p_bo7_mid, hand="broad", coverage=2.2, fill=true, load=1.0, pressure={1.0,1.0},
     dips={1,1.0,0.0}, angle=0.4, clip=true})
work(wall * ellipse(444, 544, 112, 84):soften(44), {pile=p_bo8_lit, tool="filbert 16",
     length={40,90}, coverage=1.4, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(wall * ellipse(716, 590, 138, 102):soften(48), {pile=p_bo7_sh, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(bowl * below(function() return 578 end):soften(20), {pile=p_bo6_dk, tool="filbert 16",
     length={30,64}, coverage=1.4, fill=true, load=1.0, pressure={0.9,0.8},
     dips={1,1.0,0.0}, angle=1.2, clip=true})
work(rf, {pile=p_bo7_mid, tool="filbert 12", length={30,64}, coverage=1.8, fill=true,
     load=1.0, pressure={0.95,0.8}, dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(442, 506, 82, 46):soften(24), {pile=p_bo8_lit, tool="filbert 12",
     length={30,64}, coverage=1.4, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(rf * ellipse(724, 514, 90, 52):soften(28), {pile=p_bo7_sh, tool="filbert 12",
     length={30,64}, coverage=1.5, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
work(inner * -(rim_o:grow(-4)), {pile=p_bint, tool="filbert 16", length={40,90}, coverage=1.6,
     fill=true, load=1.0, pressure={0.9,0.75}, dips={1,1.0,0.0}, clip=true})
blend(bowl:soften(5), {angle=0.4})
-- the bowl sits in its own shadow
work(R({{588,634},{702,672},{812,706}}, 78, 32)*free, {pile=p_shd3, tool="filbert 20",
     length={50,100}, coverage=1.8, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.5, clip=true})
work(R({{520,618},{600,634},{668,626}}, 24, 13)*free, {pile=p_bo6_dk, tool="filbert 12",
     length={30,64}, coverage=2.0, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.2, clip=true})
-- the tall green apple's yellow top back toward green
work(B * ellipse(566, 400, 70, 60):soften(24), {pile=p_gb3, tool="filbert 16",
     length={30,64}, coverage=1.1, fill=true, load=1.0, pressure={0.9,0.7},
     dips={1,1.0,0.0}, clip=true})
print("bowl forward", wait(0))

--@ chunk 275
p_cl_lit6 = pile{{"lead white", 5}, {"raw umber", 2.4}, {"yellow ochre", 0.8}, {"red earth", 0.5}}
p_shd4   = pile{{"raw umber", 2.2}, {"bone black", 1.6}, {"red earth", 0.5}, {"Prussian blue", 0.2}}
p_rimlit = pile{{"raw umber", 1.5}, {"red earth", 0.8}, {"yellow ochre", 0.8}, {"lead white", 0.9}}
p_bo6_dk = pile{{"bone black", 3.2}, {"raw umber", 1.4}, {"Prussian blue", 0.5}, {"red earth", 0.3}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(5, 45, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 45, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl

fl = brush{kind="filbert", width=6}
fl:load(p_cl_lit6, 1.0)
fl:stroke({{68, 636}, {110, 702}, {152, 766}}, {pressure={0.85, 0.8, 0.5}, clip=free})
fl:reload(p_cl_lit6, 1.0)
fl:stroke({{150, 774}, {228, 784}, {300, 792}}, {pressure={0.8, 0.75, 0.45}, clip=free})
fl:reload(p_cl_lit6, 1.0)
fl:stroke({{798, 598}, {826, 630}, {852, 662}}, {pressure={0.85, 0.8, 0.5}, clip=free})
fl:reload(p_cl_lit6, 1.0)
fl:stroke({{36, 596}, {110, 580}, {178, 566}}, {pressure={0.8, 0.75, 0.45}, clip=free})
fd = brush{kind="filbert", width=6}
fd:load(p_shd4, 1.0)
fd:stroke({{148, 606}, {196, 674}, {232, 748}}, {pressure={0.85, 0.8, 0.5}, clip=free})
fd:reload(p_shd4, 1.0)
fd:stroke({{862, 572}, {874, 614}, {880, 654}}, {pressure={0.85, 0.8, 0.5}, clip=free})
fd:reload(p_shd4, 1.0)
fd:stroke({{300, 796}, {350, 780}, {392, 764}}, {pressure={0.8, 0.75, 0.45}, clip=free})
-- a thin bright line along the bowl's near-left rim, and the dark under its foot
rb = brush{kind="filbert", width=5}
rb:load(p_rimlit, 1.0)
rb:stroke({{386, 486}, {400, 506}, {432, 524}, {474, 536}}, {pressure={0.9, 0.85, 0.7, 0.4},
            clip=(rim_o - ellipse(580, 500, 182, 53)) * -fruit})
rb:reload(p_rimlit, 1.0)
rb:stroke({{700, 546}, {740, 528}, {772, 508}}, {pressure={0.8, 0.7, 0.4},
            clip=(rim_o - ellipse(580, 500, 182, 53)) * -fruit})
bb = brush{kind="filbert", width=7}
bb:load(p_bo6_dk, 1.0)
bb:stroke({{532, 624}, {600, 634}, {664, 626}}, {pressure={0.9, 0.85, 0.5}, clip=free})
print("accents", wait(0))

--@ chunk 276
p_bint2 = pile{{"raw umber", 1.6}, {"bone black", 2.4}, {"red earth", 0.5}, {"Prussian blue", 0.3}}
p_bint3 = pile{{"bone black", 3.2}, {"raw umber", 1.2}, {"Prussian blue", 0.5}}
p_rim_mid = pile{{"raw umber", 2.4}, {"red earth", 1.4}, {"bone black", 1.6}, {"yellow ochre", 0.3}}
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
rimb   = rim_o - inner
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
fruit = A + B + C
rf = rimb - fruit

-- the inside of the bowl: in shadow, deepest at the back
work(inner * -fruit, {pile=p_bint2, hand="broad", coverage=2.4, fill=true, load=1.0,
     pressure={1.0,1.0}, dips={1,1.0,0.0}, angle=0.2, clip=true})
work(inner * -fruit * ellipse(580, 470, 172, 44):soften(24), {pile=p_bint3, tool="filbert 16",
     length={30,64}, coverage=1.4, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, clip=true})
-- a little light let into the inside from the near rim
work(inner * -fruit * ellipse(500, 542, 120, 26):soften(16), {pile=p_rim_mid, tool="filbert 12",
     length={24,50}, coverage=0.9, fill=true, load=1.0, pressure={0.85,0.65},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
-- the scratchy bright line on the right of the rim back into the rim
work(rf * ellipse(724, 514, 92, 54):soften(28), {pile=p_rim_mid, tool="filbert 12",
     length={30,64}, coverage=1.6, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
print("interior", wait(0))

--@ chunk 277
p_bo7_sh  = pile{{"red earth", 1.8}, {"raw umber", 1.8}, {"bone black", 2}, {"yellow ochre", 0.3}}
p_bo6_dk  = pile{{"bone black", 3.2}, {"raw umber", 1.4}, {"Prussian blue", 0.5}, {"red earth", 0.3}}
p_bounce = pile{{"red earth", 1.4}, {"lead white", 1.2}, {"yellow ochre", 0.5}}
p_shd5   = pile{{"raw umber", 2.2}, {"bone black", 1.6}, {"red earth", 0.5}, {"Prussian blue", 0.2}}
cloth_r = cloth:roughen(9, 70, 12)
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
D = ellipse(300, 655, 100, 96):roughen(5, 45, 3)
E = ellipse(470, 712, 82, 80):roughen(4, 45, 41)
fruit = A + B + C + D + E
free = cloth_r * -fruit * -bowl

-- the bowl's wall turns down and away, with the cloth's light bouncing on its lower left
work(wall * below(function() return 560 end):soften(18), {pile=p_bo7_sh, tool="filbert 16",
     length={40,90}, coverage=1.7, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.9, clip=true})
work(wall * below(function() return 598 end):soften(16), {pile=p_bo6_dk, tool="filbert 16",
     length={30,64}, coverage=1.3, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=1.3, clip=true})
work(wall * ellipse(450, 590, 90, 34):soften(22), {pile=p_bounce, tool="filbert 12",
     length={24,50}, coverage=0.9, fill=true, load=1.0, pressure={0.85,0.65},
     dips={1,1.0,0.0}, angle=0.7, clip=true})
-- and the front apple sits down into the cloth
work(ellipse(322, 748, 78, 30):soften(14) * free, {pile=p_shd5, tool="filbert 16",
     length={30,64}, coverage=2.0, fill=true, load=1.0, pressure={0.9,0.75},
     dips={1,1.0,0.0}, angle=0.3, clip=true})
print("seated", wait(0))

--@ chunk 278
p_bo7_sh  = pile{{"red earth", 1.8}, {"raw umber", 1.8}, {"bone black", 2}, {"yellow ochre", 0.3}}
p_bounce2= pile{{"red earth", 1.8}, {"lead white", 0.7}, {"yellow ochre", 0.5}}
rim_o  = ellipse(580, 500, 200, 64)
inner  = ellipse(580, 500, 182, 53):roughen(2, 45, 11)
arc = function(x) return 500 + 53*math.sqrt(math.max(0, 1 - ((x-580)/182)^2)) end
nearrim = below(arc) * rim_o
skirt  = poly({{392,506},{396,532},{408,560},{430,583},{462,600},{504,613},{544,621},
               {580,625},{616,621},{656,613},{698,600},{730,583},{752,560},{764,532},{768,506}}, true)
bowl   = (rim_o + skirt):roughen(3, 55, 7)
wall   = bowl - inner
A = (ellipse(500, 486, 86, 83):roughen(5, 40, 21)) - nearrim
C = (ellipse(690, 496, 78, 76):roughen(4, 35, 27)) - nearrim
B = (ellipse(600, 448, 94, 90):roughen(6, 50, 33)) - nearrim
fruit = A + B + C
work(ellipse(450, 590, 96, 40):soften(20) * wall, {pile=p_bo7_sh, tool="filbert 16",
     length={40,90}, coverage=2.2, fill=true, load=1.0, pressure={0.95,0.8},
     dips={1,1.0,0.0}, angle=0.7, clip=true})
work(ellipse(446, 588, 84, 30):soften(18) * wall, {pile=p_bounce2, tool="filbert 20",
     length={50,100}, coverage=0.5, fill=false, load=0.8, pressure={0.7,0.55},
     angle=0.7, clip=true})
print("bounce fixed", wait(0))

--@ chunk 279
print("set", wait(6*60))

--@ chunk 280
-- the palette for the repaint. 16 piles, mixed once, kept.
g_dk = pile{{"bone black", 4}, {"raw umber", 3}}                              -- 1 near black
g_wl = pile{{"raw umber", 4}, {"bone black", 2.4}, {"red earth", 2}}         -- 2 wall dark
g_wm = pile{{"raw umber", 3}, {"red earth", 1.6}, {"lead white", 0.5}}     -- 3 wall mid
g_wt = pile{{"raw umber", 2}, {"yellow ochre", 1.6}, {"lead white", 1.1}}   -- 4 wall light
g_tb = pile{{"raw umber", 3}, {"bone black", 3}, {"red earth", 1.2}}       -- 5 table dark
g_cl = pile{{"lead white", 5}, {"raw umber", 0.8}, {"yellow ochre", 0.4}}   -- 6 cloth light
g_cm = pile{{"lead white", 3}, {"raw umber", 1.2}, {"pale smalt", 0.45}}   -- 7 cloth mid
g_cd = pile{{"lead white", 1.6}, {"raw umber", 1.3}, {"pale smalt", 1.1}}  -- 8 cloth shadow
g_rb = pile{{"vermilion", 3}, {"raw umber", 1.6}, {"red earth", 1.6}}      -- 9  red lay-in
g_rm = pile{{"vermilion", 2.4}, {"red earth", 1.6}, {"yellow ochre", 1.1}} -- 10 red mid
g_rl = pile{{"vermilion", 1.5}, {"red earth", 1.3}, {"yellow ochre", 1.3}, {"lead white", 0.9}} -- 11 red lit
g_gb = pile{{"green earth", 2.6}, {"Prussian blue", 0.55}, {"raw umber", 1.2}}                -- 12 green lay-in
g_gm = pile{{"yellow ochre", 3}, {"green earth", 2}, {"raw umber", 0.7}}                       -- 13 green mid
g_gl = pile{{"yellow ochre", 2}, {"green earth", 1.2}, {"lead white", 1.2}}                   -- 14 green lit
g_bw = pile{{"raw umber", 3}, {"red earth", 2}, {"lead white", 0.8}}                           -- 15 bowl lit
g_bd = pile{{"raw umber", 2}, {"bone black", 1.3}, {"red earth", 1.6}}                        -- 16 bowl dark
for _, n in ipairs{"g_dk","g_wl","g_wm","g_wt","g_tb","g_cl","g_cm","g_cd","g_rb","g_rm","g_rl","g_gb","g_gm","g_gl","g_bw","g_bd"} do print(n, _G[n]) end

--@ chunk 281
-- ---------- the composition, decided on paper before any paint ----------
-- one light: up and to the left
LX, LY = -0.50, -0.866

-- the table's far edge, a gentle slope
function tline(x) return 400 - 0.03 * (x - 500) end
tlinep = {{0, tline(0)}, {1000, tline(1000)}}

-- the cloth: a big shape whose back edge crosses the table, running off
-- the left, right and bottom edges
clothpts = {{-10, 590}, {100, 556}, {220, 534}, {350, 526}, {480, 532},
            {610, 540}, {730, 533}, {860, 546}, {1010, 586}}

-- the bowl: rim ellipse centre/radii, and its depth below the rim
BOWLX, BOWLY, BOWLRX, BOWLRY = 575, 498, 185, 62

-- the five apples
AP = {
  {cx=592, cy=400, r=92,  k="g"},   -- green, back, tallest: breaks the table line
  {cx=505, cy=505, r=82,  k="r"},   -- red, in the bowl, left
  {cx=665, cy=515, r=80,  k="r"},   -- red, in the bowl, right
  {cx=300, cy=655, r=88,  k="r"},   -- red, on the cloth, front left
  {cx=485, cy=712, r=72,  k="g"},   -- green, on the cloth, front
}

function circ(cx, cy, rx, ry, n)
  n = n or 44
  local pts = {}
  for i = 0, n do local a = 2*math.pi*i/n; pts[#pts+1] = {cx+rx*math.cos(a), cy+ry*math.sin(a)} end
  return pts
end
function arc(cx, cy, rx, ry, a0, a1, n)
  n = n or 26
  local pts = {}
  for i = 0, n do local a = a0 + (a1-a0)*i/n; pts[#pts+1] = {cx+rx*math.cos(a), cy+ry*math.sin(a)} end
  return pts
end

-- fold lines on the cloth
FOLDS = {
  {{150, 542}, {112, 630}, {72, 750}},
  {{312, 527}, {272, 650}, {240, 792}},
  {{478, 532}, {440, 660}, {424, 796}},
  {{648, 540}, {628, 660}, {622, 796}},
  {{806, 541}, {820, 650}, {842, 788}},
}

h = pencil("2H")
h:line(tlinep, {pressure=0.28, smooth=false})
h:line(clothpts, {pressure=0.32})
-- bowl: far rim, right side, base, left side  (the near rim is an interior line)
h:line(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, math.pi, 2*math.pi, 24), {pressure=0.3})
h:line(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 0, math.pi, 24), {pressure=0.22})
h:line({{758, 498}, {742, 540}, {700, 578}, {620, 596}, {520, 594}, {452, 574}, {412, 534}, {392, 498}}, {pressure=0.3})
for _, a in ipairs(AP) do h:line(circ(a.cx, a.cy, a.r, a.r*0.97), {pressure=0.32}) end
for _, f in ipairs(FOLDS) do h:line(f, {pressure=0.2}) end
print("drawn")

--@ chunk 282
hb = pencil("HB")
hb:line({{40,40},{300,40}}, {pressure=0.9, smooth=false})
hb:line({{40,80},{300,80}}, {pressure=0.5, smooth=false})
ch = chalk()
ch:line({{40,120},{300,120}}, {pressure=0.7, smooth=false})
print(drawing_guide():area())

--@ chunk 283
-- bury everything under one dark warm ground.  Coverage is not opacity:
-- it takes a lot of layers to kill a strong old colour.
work(everywhere(), {hand="broad", tool="filbert 26", pile=g_dk, length={100,240},
                    coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0},
                    angle=0.35, edge="found"})

--@ chunk 284
work(everywhere(), {hand="broad", tool="filbert 26", pile=g_dk, length={90,220},
                    coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0},
                    angle=-0.7, edge="found"})

--@ chunk 285
work(everywhere(), {hand="broad", tool="filbert 26", pile=g_dk, length={90,220},
                    coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0},
                    angle=0.15, edge="found"})
blend((everywhere():shrink(6)):soften(6), {angle=0.2})
print(drying(500, 500))

--@ chunk 286
hp = pencil("HB")
hp:line(tlinep, {pressure=0.55, smooth=false})
hp:line(clothpts, {pressure=0.6})
hp:line(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, math.pi, 2*math.pi, 24), {pressure=0.55})
hp:line(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 0, math.pi, 24), {pressure=0.3})
hp:line({{758, 498}, {742, 540}, {700, 578}, {620, 596}, {520, 594}, {452, 574}, {412, 534}, {392, 498}}, {pressure=0.55})
for i, a in ipairs(AP) do
  hp:line(circ(a.cx, a.cy, a.r, a.r*0.97), {pressure=0.6})
  hp:line({{a.cx - 0.42*a.r, a.cy - 0.42*a.r}, {a.cx + 0.5*a.r, a.cy + 0.5*a.r}}, {pressure=0.28, smooth=false})
end
for _, f in ipairs(FOLDS) do hp:line(f, {pressure=0.3}) end
print("drawn")

--@ chunk 287
cc = chalk()
cc:line(tlinep, {pressure=0.5, smooth=false})
cc:line(clothpts, {pressure=0.55})
cc:line(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, math.pi, 2*math.pi, 24), {pressure=0.5})
cc:line(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 0, math.pi, 24), {pressure=0.22})
cc:line({{758, 498}, {742, 540}, {700, 578}, {620, 596}, {520, 594}, {452, 574}, {412, 534}, {392, 498}}, {pressure=0.5})
for i, a in ipairs(AP) do
  cc:line(circ(a.cx, a.cy, a.r, a.r*0.97), {pressure=0.55})
  cc:line({{a.cx - 0.5*a.r, a.cy - 0.5*a.r}, {a.cx + 0.58*a.r, a.cy + 0.58*a.r}}, {pressure=0.25, smooth=false})
end
for _, f in ipairs(FOLDS) do cc:line(f, {pressure=0.3}) end
print("drawn")

--@ chunk 288
wall = everywhere() - below(tline)
table_ = below(tline)
print(string.format("wall %.0f  table %.0f  canvas %.0f", wall:area(), table_:area(), everywhere():area()))
-- the wall: a dark warm brown, lighter toward the upper left where the light is
work(wall, {hand="broad", tool="filbert 24", pile=g_wl, length={110, 250},
            coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0},
            angle=0.3, edge="soft"})

--@ chunk 289
-- one soft light, up and to the left, built from stacked soft pools (no spots)
pool1 = ellipse(190, 150, 470, 330):blur(200)
pool2 = ellipse(115, 95, 285, 195):blur(135)
work(wall * pool1, {hand="broad", tool="filbert 22", pile=g_wm, length={120, 260},
                    coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.4})
work(wall * pool2, {hand="broad", tool="filbert 20", pile=g_wt, length={110, 240},
                    coverage=2.0, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.5})
-- deepen the far side and the lower corner, so the light has somewhere to fall from
work(wall * (ellipse(900, 420, 460, 330):blur(220)), {hand="glaze", pile=g_dk, coverage=1.6})
work(wall * (ellipse(620, 430, 330, 200):blur(150)), {hand="glaze", pile=g_dk, coverage=1.2})
-- the table
work(table_, {hand="broad", tool="filbert 24", pile=g_tb, length={110, 250},
              coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.1, edge="soft"})
work(table_ * (ellipse(120, 455, 300, 100):blur(110)), {hand="body", pile=g_wl,
              coverage=1.6, pressure=0.9, fill=true, dips={2, 1.0, 0.2}})
work(table_ * (ellipse(880, 700, 420, 300):blur(200)), {hand="glaze", pile=g_dk, coverage=1.4})
print(drying(500, 300))

--@ chunk 290
-- bury the scaly pool
work(wall, {hand="broad", tool="filbert 24", pile=g_wl, length={110, 250},
            coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=-0.5, edge="soft"})
-- and the table, which the glaze pass also touched
work(table_, {hand="broad", tool="filbert 24", pile=g_tb, length={110, 250},
              coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.1, edge="soft"})

--@ chunk 291
stipple(wall * pool1, {pile=g_wm, width=4, coverage=5, pressure={0.5, 0.85},
                       cluster=0.2, dips={6, 1.0, 0.3}})
stipple(wall * pool2, {pile=g_wm, width=3.5, coverage=5, pressure={0.5, 0.85},
                       cluster=0.2, dips={6, 1.0, 0.3}})

--@ chunk 292
-- unify the light with the same pile, so the specks sink back into the tone
work(wall * pool1, {hand="glaze", tool="filbert 26", pile=g_wm, length={130, 300},
                    coverage=1.8, pressure={0.55, 0.3}, angle=0.5, edge="soft"})
work(wall * pool2, {hand="glaze", tool="filbert 24", pile=g_wm, length={120, 280},
                    coverage=1.6, pressure={0.55, 0.3}, angle=0.4, edge="soft"})

--@ chunk 293
clothpts2 = {{-12, 594}, {100, 556}, {220, 534}, {350, 526}, {480, 532}, {610, 540},
             {730, 533}, {860, 546}, {1012, 588}, {1012, 812}, {-12, 812}}
CLOTH = poly(clothpts2, true):soften(2.5)
print(string.format("cloth area %.0f of %.0f", CLOTH:area(), everywhere():area()))
-- lay the cloth in: a warm mid tone first, opaque, so everything after sits on solid paint
work(CLOTH, {hand="broad", tool="filbert 24", pile=g_cm, length={110, 250},
             coverage=3.5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.15, edge="soft"})

--@ chunk 294
-- one blend to fuse the strokes into a solid surface, over the opaque coat only
blend((CLOTH:shrink(5)):soften(5), {angle=0.15})

--@ chunk 295
function off(pts, dx, dy)
  local r = {}
  for i, p in ipairs(pts) do r[i] = {p[1]+dx, p[2]+dy} end
  return r
end
cpool1 = ellipse(210, 615, 430, 200):blur(175)
cpool2 = ellipse(115, 600, 255, 120):blur(110)
cpool3 = ellipse(450, 605, 250, 125):blur(120)
-- the light on the cloth: soft glazes, so no dabs and no thin film over a dark ground
work(CLOTH * cpool1, {hand="glaze", tool="filbert 26", pile=g_cl, length={130, 300}, coverage=2.2, pressure={0.6, 0.35}, angle=0.2})
work(CLOTH * cpool2, {hand="glaze", tool="filbert 24", pile=g_cl, length={120, 280}, coverage=2.0, pressure={0.6, 0.35}, angle=0.25})
work(CLOTH * cpool3, {hand="glaze", tool="filbert 24", pile=g_cl, length={120, 280}, coverage=1.8, pressure={0.6, 0.35}, angle=0.3})
-- the far side of the cloth falls away into the dark
work(CLOTH * (ellipse(905, 690, 400, 250):blur(190)), {hand="glaze", pile=g_cd, coverage=2.4, pressure={0.6, 0.35}, angle=0.3})
work(CLOTH * (ellipse(420, 815, 620, 120):blur(90)), {hand="glaze", pile=g_cd, coverage=1.8, pressure={0.6, 0.35}, angle=0.1})

--@ chunk 296
dright = ellipse(880, 700, 400, 250):blur(180)
dfront = ellipse(430, 830, 640, 120):blur(80)
cllit  = CLOTH * (-(dright + dfront))
print(string.format("cloth %.0f  lit part %.0f", CLOTH:area(), cllit:area()))
work(cllit, {hand="body", tool="filbert 15", pile=g_cl, length={50, 130},
             coverage=3.5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
blend((cllit:shrink(6)):soften(6), {angle=0.2})

--@ chunk 297
-- bury the hard-edged light, back to a solid mid tone
work(CLOTH, {hand="body", tool="filbert 15", pile=g_cm, length={50, 130},
             coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=-0.3, edge="soft"})
blend((CLOTH:shrink(6)):soften(6), {angle=0.15})

--@ chunk 298
clpool = ellipse(250, 650, 470, 215):blur(240)
work(CLOTH * clpool, {hand="body", tool="filbert 15", pile=g_cl, length={50, 130},
                      coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
-- the far side of the cloth, and the front fall, go into shadow: soft pools, no booleans
work(CLOTH * (ellipse(930, 690, 330, 210):blur(200)), {hand="body", tool="filbert 15", pile=g_cd,
      length={50, 130}, coverage=2.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.3, edge="soft"})
work(CLOTH * (ellipse(430, 845, 560, 105):blur(95)), {hand="body", tool="filbert 15", pile=g_cd,
      length={50, 130}, coverage=2.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.1, edge="soft"})
blend((CLOTH:shrink(6)):soften(6), {angle=0.2})

--@ chunk 299
zero = mask(function() return 0 end)
fsh, fli = zero, zero
for _, f in ipairs(FOLDS) do
  fsh = fsh + ribbon(off(f, 24, 15), {22, 30, 42}):soften(15)
  fli = fli + ribbon(off(f, -22, -13), {16, 22, 32}):soften(13)
end
print(string.format("fold shadow %.0f  fold light %.0f", fsh:area(), fli:area()))
work(CLOTH * fsh, {hand="glaze", tool="filbert 20", pile=g_cd, length={90, 220},
                   coverage=2.6, pressure={0.75, 0.5}, angle=1.35, edge="soft"})
work(CLOTH * fli, {hand="glaze", tool="filbert 18", pile=g_cl, length={80, 200},
                   coverage=2.0, pressure={0.7, 0.45}, angle=1.35, edge="soft"})

--@ chunk 300
-- pull the streaks back to the cloth's own tone, then restore the light
work(CLOTH * (fsh + fli), {hand="glaze", tool="filbert 22", pile=g_cm, length={100, 240},
                           coverage=3.2, pressure={0.8, 0.55}, angle=0.25, edge="soft"})
work(CLOTH * clpool, {hand="glaze", tool="filbert 24", pile=g_cl, length={120, 280},
                      coverage=2.4, pressure={0.75, 0.5}, angle=0.2, edge="soft"})
blend((CLOTH:shrink(6)):soften(6), {angle=0.18})

--@ chunk 301
-- drapery: broad soft troughs with a broad soft ridge on the light side of each
BANDS = {
  {pts = {{40, 570}, {140, 660}, {260, 800}}, w = {90, 120, 150}},
  {pts = {{235, 545}, {330, 665}, {430, 800}}, w = {80, 110, 140}},
  {pts = {{700, 545}, {655, 670}, {610, 800}}, w = {90, 120, 150}},
  {pts = {{905, 560}, {870, 675}, {840, 800}}, w = {80, 110, 140}},
}
btr, brg = zero, zero
for _, b in ipairs(BANDS) do
  btr = btr + ribbon(b.pts, b.w):blur(52)
  brg = brg + ribbon(off(b.pts, -95, -28), {70, 95, 120}):blur(58)
end
work(CLOTH * btr, {hand="glaze", tool="filbert 24", pile=g_cd, length={110, 260},
                   coverage=2.4, pressure={0.7, 0.45}, angle=1.1, edge="soft"})
work(CLOTH * brg, {hand="glaze", tool="filbert 22", pile=g_cl, length={100, 240},
                   coverage=1.9, pressure={0.7, 0.45}, angle=1.1, edge="soft"})

--@ chunk 302
blend((CLOTH:shrink(6)):soften(6), {angle=1.0})

--@ chunk 303
BANDS = {
  {pts = {{-30, 592}, {-5, 682}, {15, 800}}, w = {70, 100, 130}},
  {pts = {{40, 570}, {140, 660}, {260, 800}},   w = {90, 120, 150}},
  {pts = {{235, 545}, {330, 665}, {430, 800}},  w = {80, 110, 140}},
  {pts = {{700, 545}, {655, 670}, {610, 800}},  w = {90, 120, 150}},
  {pts = {{800, 550}, {778, 665}, {750, 800}},  w = {75, 105, 135}},
  {pts = {{930, 565}, {890, 678}, {858, 800}},  w = {80, 110, 140}},
}
btr, brg = zero, zero
for _, b in ipairs(BANDS) do
  btr = btr + ribbon(b.pts, b.w):blur(48)
  brg = brg + ribbon(off(b.pts, -95, -28), {70, 95, 120}):blur(56)
end
work(CLOTH * btr, {hand="body", tool="filbert 18", pile=g_cd, length={45, 120},
                   coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.1, edge="soft"})
work(CLOTH * brg, {hand="body", tool="filbert 16", pile=g_cl, length={40, 110},
                   coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.1, edge="soft"})
blend((CLOTH:shrink(6)):soften(6), {angle=1.0})

--@ chunk 304
-- the light falls off to the right and into the near corner
work(CLOTH * (ellipse(930, 690, 300, 190):blur(170)), {hand="body", tool="filbert 18", pile=g_cd,
      length={45, 120}, coverage=2.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.3, edge="soft"})
work(CLOTH * (ellipse(760, 840, 340, 120):blur(110)), {hand="body", tool="filbert 18", pile=g_cd,
      length={45, 120}, coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})

--@ chunk 305
-- the bowl: an outer silhouette (far rim over the top, then the body down to the foot),
-- a rim ring, the inside seen through the opening, and the outer wall below the rim
outer = {}
for i, p in ipairs(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, math.pi, 2*math.pi, 30)) do outer[i] = p end
local bodypts = {{758, 500}, {748, 538}, {724, 572}, {688, 595}, {636, 607}, {576, 610},
                 {516, 607}, {466, 594}, {428, 571}, {402, 538}, {392, 500}}
for i, p in ipairs(bodypts) do outer[#outer+1] = p end
BOWL = poly(outer, true)
rim_out = ellipse(BOWLX, BOWLY, BOWLRX, BOWLRY)
rim_in  = ellipse(BOWLX, BOWLY, BOWLRX - 17, BOWLRY - 12)
BOWL_BODY = BOWL - rim_out
BOWL_RIM  = rim_out - rim_in
function innery(x) return BOWLY + (BOWLRY - 12) * math.sqrt(math.max(0, 1 - ((x - BOWLX)/(BOWLRX - 17))^2)) end
BOWL_IN = rim_in - below(innery)
print(string.format("bowl %.0f | body %.0f | rim %.0f | inside %.0f", BOWL:area(), BOWL_BODY:area(), BOWL_RIM:area(), BOWL_IN:area()))

--@ chunk 306
-- the inside of the bowl: the far inner wall, bounded below by the near rim's inner edge
inpts = {}
for i, p in ipairs(arc(BOWLX, BOWLY, BOWLRX-17, BOWLRY-12, math.pi, 2*math.pi, 26)) do inpts[i] = p end
for i = 1, 26 do
  local t = 1 - (i - 1) / 26
  inpts[#inpts+1] = {BOWLX + (BOWLRX-17) * math.cos(math.pi * t * 0 + 0) * 0 +
                     (BOWLRX-17) * (1 - 2*t), BOWLY + (BOWLRY-12) * math.sin(math.pi * t)}
end
BOWL_IN = poly(inpts, true)
print(string.format("inside %.0f  (the inner ellipse is %.0f, so about half is right)",
  BOWL_IN:area(), rim_in:area()))
print(string.format("check: BOWL %.0f vs body+rim+inside = %.0f", BOWL:area(),
  BOWL_BODY:area() + BOWL_RIM:area() + BOWL_IN:area()))

--@ chunk 307
-- the bowl, laid in solid: raw umber and red earth and bone black, never orange
work(BOWL, {hand="body", tool="filbert 15", pile=g_bd, length={40, 110},
            coverage=3.5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.4, edge="soft"})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})

--@ chunk 308
-- the inside of the bowl is in shadow; the far inner wall catches a little light
work(BOWL_IN, {hand="body", tool="filbert 12", pile=g_dk, length={30, 80},
               coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN * (ellipse(455, 468, 115, 48):blur(50)), {hand="body", tool="filbert 12", pile=g_bd,
      length={30, 80}, coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
-- the outer wall: light from the upper left, deep shadow at the right
work(BOWL_BODY * (ellipse(468, 552, 145, 85):blur(75)), {hand="body", tool="filbert 13", pile=g_bw,
      length={35, 90}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(735, 570, 125, 95):blur(80)), {hand="body", tool="filbert 13", pile=g_dk,
      length={35, 90}, coverage=2.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
-- the white cloth throws a soft cool light back up under the bowl
work(BOWL_BODY * (ellipse(600, 648, 205, 72):blur(58)), {hand="glaze", pile=g_cm, coverage=2.0})

--@ chunk 309
-- raw umber + red earth + white is a pumpkin, not earthenware. Mute it right down.
g_bw = pile{{"raw umber", 4}, {"red earth", 1.1}, {"bone black", 0.7}, {"lead white", 0.5}}
-- a clean crescent for the inside, no wobbly poly edge
BOWL_IN = mask(function(x, y)
  local u = (x - BOWLX) / (BOWLRX - 17)
  local v = (y - BOWLY) / (BOWLRY - 12)
  if u*u + v*v > 1 then return 0 end
  local edge = BOWLY + (BOWLRY - 12) * math.sqrt(math.max(0, 1 - u*u))
  return smoothstep(edge + 2, edge - 2, y)
end)
print(string.format("inside %.0f", BOWL_IN:area()))
-- the outer wall all over again, in the muted earth
work(BOWL_BODY, {hand="body", tool="filbert 13", pile=g_bw, length={35, 90},
                 coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.4, edge="soft"})
work(BOWL_IN, {hand="body", tool="filbert 12", pile=g_dk, length={30, 80},
               coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})

--@ chunk 310
-- a matte earthenware in a dim room is a grey-brown; cool and dark it so that thin
-- passages grey down instead of glowing orange over the white cloth
g_bw = pile{{"raw umber", 3}, {"bone black", 1.3}, {"red earth", 0.8}, {"pale smalt", 0.45}}
work(BOWL_BODY, {hand="body", tool="filbert 13", pile=g_bw, length={35, 90},
                 coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.4, edge="soft"})
work(BOWL_RIM, {hand="body", tool="filbert 9", pile=g_bw, length={22, 55},
                coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN, {hand="body", tool="filbert 10", pile=g_dk, length={25, 60},
               coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})

--@ chunk 311
-- the wall light is finished with, so that pile becomes the bowl's lit earthenware
g_bl = pile{{"raw umber", 2.4}, {"red earth", 0.7}, {"lead white", 1.0},
            {"bone black", 0.4}, {"pale smalt", 0.3}}
-- the rim: its top surface faces the light, and it falls away to the right
work(BOWL_RIM * (ellipse(450, 460, 140, 62):blur(55)), {hand="body", tool="filbert 9", pile=g_bl,
      length={22, 55}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(600, 540, 150, 34):blur(30)), {hand="body", tool="filbert 9", pile=g_bl,
      length={22, 55}, coverage=2.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(735, 512, 115, 62):blur(58)), {hand="body", tool="filbert 9", pile=g_dk,
      length={22, 55}, coverage=2.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
-- the outer wall
work(BOWL_BODY * (ellipse(462, 550, 152, 86):blur(72)), {hand="body", tool="filbert 12", pile=g_bl,
      length={30, 80}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(742, 572, 122, 92):blur(78)), {hand="body", tool="filbert 12", pile=g_dk,
      length={30, 80}, coverage=2.0, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(600, 652, 200, 65):blur(55)), {hand="glaze", pile=g_cm, coverage=1.8})
-- the far inner wall catches a little
work(BOWL_IN * (ellipse(450, 468, 120, 46):blur(45)), {hand="body", tool="filbert 10", pile=g_bw,
      length={25, 60}, coverage=2.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})

--@ chunk 312
-- the bowl's shadow on the cloth: a pool thrown away from the light, plus a contact,
-- never a ring round the base
free = CLOTH - BOWL:grow(2)
work(free * (ellipse(735, 628, 215, 64):blur(56)), {hand="body", tool="filbert 15", pile=g_cm,
      length={40, 110}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
work(free * (ellipse(648, 604, 110, 27):blur(20)), {hand="body", tool="filbert 12", pile=g_cd,
      length={30, 80}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.15, edge="soft"})
-- helpers for painting a sphere: a lighting mask from the one light, and its silhouette
function ashape(a) return ellipse(a.cx, a.cy, a.r, a.r * 0.96):soften(1.2) end
function litf(a, t0, t1)
  return mask(function(x, y)
    local u = (x - a.cx) / a.r
    local v = (y - a.cy) / (a.r * 0.96)
    local r = math.sqrt(u*u + v*v)
    if r < 1e-6 then return 1 end
    return smoothstep(t0, t1, (u * LX + v * LY) / r)
  end)
end
function hoff(a, f) return ellipse(a.cx + LX*a.r*f, a.cy + LY*a.r*f, a.r*0.34, a.r*0.27) end

--@ chunk 313
-- muted olive greens; yellow ochre and green earth alone go lime
g_gm = pile{{"yellow ochre", 2.6}, {"green earth", 2.2}, {"raw umber", 1.0}, {"lead white", 0.2}}
g_gl = pile{{"yellow ochre", 2}, {"green earth", 1.6}, {"lead white", 1.2}, {"raw umber", 0.5}}

-- one fruit, as a value ladder: every step opaque OVER the one under it, so nothing
-- thin ever sits on the dark and nothing blooms
function fruit(a, lay, mid, lit, hi)
  local S = ashape(a)
  local tool = "filbert " .. math.max(7, math.floor(a.r / 6))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, -0.05, 0.65), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.35, 0.92), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=2.8,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:shrink(3)):soften(3), {angle=0.3})
  -- core shadow at the far side from the light
  work(S * litf(a, -0.95, -0.28), {hand="body", tool=tool, pile=g_dk, length={30, 80}, coverage=1.5,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  -- the white cloth throws a cool light back along the shadow-side edge
  work(S:rim(a.r * 0.15, 9) * litf(a, -0.8, -0.02) * ellipse(a.cx, a.cy + a.r * 0.72, a.r * 0.95, a.r * 0.55),
       {hand="glaze", pile=g_cm, coverage=1.8})
  -- a small warm light where the light strikes
  work(hoff(a, 0.46):blur(a.r * 0.13), {hand="body", tool="filbert 6", pile=hi,
       length={12, 30}, coverage=2.2, pressure=1, fill=true, edge="soft"})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_cl)

--@ chunk 314
-- the wall and table are finished, so those piles become deep cores of each fruit's own hue
g_rc = pile{{"vermilion", 1.0}, {"raw umber", 2.2}, {"red earth", 1.6}, {"bone black", 0.7}}
g_gc = pile{{"green earth", 1.6}, {"Prussian blue", 0.5}, {"raw umber", 2.2}, {"bone black", 0.6}}

function fruit(a, lay, mid, lit, hi, core)
  local S = ashape(a)
  local tool = "filbert " .. math.max(7, math.floor(a.r / 6))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, -0.05, 0.65), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.35, 0.92), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=2.8,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:shrink(3)):soften(3), {angle=0.3})
  -- core shadow: broad and soft, in the fruit's own deep hue, never neutral black
  work(S * litf(a, -1.0, -0.18), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=2.6,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  -- the white cloth throws a cool light back along the shadow-side edge
  work(S:rim(a.r * 0.15, 9) * litf(a, -0.8, -0.02) * ellipse(a.cx, a.cy + a.r * 0.72, a.r * 0.95, a.r * 0.55),
       {hand="glaze", pile=g_cm, coverage=1.8})
  -- a small warm light where the light strikes
  work(hoff(a, 0.46):blur(a.r * 0.14), {hand="body", tool="filbert 6", pile=hi,
       length={12, 30}, coverage=2.0, pressure=1, fill=true, edge="soft"})
end
-- put the green apple's core back in as a deep olive
local a = AP[1]
local S = ashape(a)
work(S * litf(a, -1.0, -0.18), {hand="body", tool="filbert 15", pile=g_gc, length={30, 80},
     coverage=2.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((S:shrink(3)):soften(3), {angle=0.3})

--@ chunk 315
fruit(AP[2], g_rb, g_rm, g_rl, g_cl, g_rc)
fruit(AP[3], g_rb, g_rm, g_rl, g_cl, g_rc)

--@ chunk 316
-- a still-life red is a deep crimson, not vermilion poster colour
g_rb = pile{{"vermilion", 2.2}, {"raw umber", 2}, {"red earth", 2}, {"bone black", 0.5}}
g_rm = pile{{"vermilion", 1.8}, {"red earth", 2.2}, {"yellow ochre", 0.7}, {"raw umber", 0.6}}
g_rl = pile{{"vermilion", 1.2}, {"red earth", 1.4}, {"yellow ochre", 1.2}, {"lead white", 0.7}, {"raw umber", 0.3}}
-- a warm cream with a little red in it: a cream highlight on a red apple reads as a pink blob
g_hl = pile{{"lead white", 4}, {"red earth", 0.7}, {"yellow ochre", 0.5}, {"raw umber", 0.3}}

function fruit(a, lay, mid, lit, hi, core, sd)
  local S = ashape(a)
  local tool = "filbert " .. math.max(7, math.floor(a.r / 6))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge={found=0.3, soft=0.7, period=95, seed=sd}})
  work(S * litf(a, -0.05, 0.65), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.35, 0.92), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=2.8,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:shrink(3)):soften(3), {angle=0.3})
  work(S * litf(a, -1.0, -0.18), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=2.6,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S:rim(a.r * 0.15, 9) * litf(a, -0.8, -0.02) * ellipse(a.cx, a.cy + a.r * 0.72, a.r * 0.95, a.r * 0.55),
       {hand="glaze", pile=g_cm, coverage=1.8})
  work(hoff(a, 0.5):blur(a.r * 0.16), {hand="body", tool="filbert 5", pile=hi,
       length={10, 24}, coverage=1.8, pressure=1, fill=true, edge="soft"})
end
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8)

--@ chunk 317
-- a real value ladder for the reds: dark lay-in, medium, and a clearly lighter light
g_rb = pile{{"vermilion", 1.6}, {"raw umber", 3}, {"red earth", 2.4}, {"bone black", 1.2}}
g_rm = pile{{"vermilion", 2.0}, {"red earth", 2.2}, {"raw umber", 1.0}}
g_rl = pile{{"vermilion", 1.4}, {"red earth", 1.2}, {"yellow ochre", 1.6}, {"lead white", 1.2}, {"raw umber", 0.3}}
g_rc = pile{{"vermilion", 0.8}, {"raw umber", 3}, {"red earth", 1.6}, {"bone black", 1.4}}
-- and a deeper ladder for the green
g_gb = pile{{"green earth", 2.0}, {"Prussian blue", 0.5}, {"raw umber", 2.8}, {"bone black", 1.0}}
g_gc = pile{{"green earth", 1.2}, {"Prussian blue", 0.4}, {"raw umber", 3}, {"bone black", 1.4}}

function fruit(a, lay, mid, lit, hi, core, sd)
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(1.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  -- lay-in with a clean drawn edge, then soften it back: crisp AND lost, not scalloped
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found", clip=true})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:grow(4)):soften(5), {angle=0.3})
  work(S * litf(a, -1.0, -0.15), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=2.8,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:grow(3)):soften(4), {angle=0.3})
  -- the white cloth throws a cool light back along the shadow-side edge
  work(S:rim(a.r * 0.16, 10) * litf(a, -0.75, 0.0) * ellipse(a.cx, a.cy + a.r * 0.7, a.r, a.r * 0.6),
       {hand="glaze", pile=g_cm, coverage=2.0})
  -- the light itself: one clean touch. fill=true over a small mask beads into cottage cheese.
  local b = brush{kind="filbert", width=math.max(6, math.floor(a.r / 8)), point=0.6}
  b:load(hi, 0.75)
  local hx, hy = a.cx + LX*a.r*0.5, a.cy + LY*a.r*0.5
  b:touch(hx, hy, {pressure=0.5, clip=S:grow(3)})
  b:touch(hx + 4, hy + 5, {pressure=0.3, clip=S:grow(3)})
end
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8)

--@ chunk 318
for i = 2, 3 do
  local a = AP[i]
  local S = ellipse(a.cx, a.cy, a.r, a.r*0.95):roughen(2.5, 70, 3):soften(1.5)
  print(string.format("apple %d  shape %.0f | mid(0..0.55) %.0f | lit(0.45..0.95) %.0f | core(-1..-0.15) %.0f",
    i, S:area(), (S*litf(a,0.0,0.55)):area(), (S*litf(a,0.45,0.95)):area(), (S*litf(a,-1.0,-0.15)):area()))
  local m1 = mask(function(x,y)
    local u=(x-a.cx)/a.r local v=(y-a.cy)/(a.r*0.95) local r=math.sqrt(u*u+v*v)
    if r<1e-6 then return 1 end
    return smoothstep(0.0,0.55,(u*LX+v*LY)/r)
  end)
  print(string.format("   light mask alone: centre %.2f  upper-left point %.2f  right point %.2f  lower %.2f",
    m1:at(a.cx, a.cy), m1:at(a.cx - a.r*0.5, a.cy - a.r*0.4), m1:at(a.cx + a.r*0.7, a.cy), m1:at(a.cx, a.cy + a.r*0.7)))
end

--@ chunk 319
-- a colour test in a dark corner that the finishing glaze will cover anyway
-- candidates: a real ladder for the reds, dark to light
c1 = pile{{"vermilion", 1.6}, {"raw umber", 3}, {"red earth", 2.4}, {"bone black", 1.2}}
c2 = pile{{"red earth", 3}, {"vermilion", 1.0}, {"raw umber", 1.2}}
c3 = pile{{"lead white", 3}, {"yellow ochre", 1.8}, {"vermilion", 0.9}, {"red earth", 0.5}}
c4 = pile{{"vermilion", 0.8}, {"raw umber", 3}, {"red earth", 1.6}, {"bone black", 1.4}}
-- and for the green
c5 = pile{{"green earth", 2.0}, {"Prussian blue", 0.5}, {"raw umber", 2.8}, {"bone black", 1.0}}
c6 = pile{{"yellow ochre", 2.4}, {"green earth", 2.0}, {"raw umber", 1.0}, {"lead white", 0.3}}
c7 = pile{{"lead white", 2.6}, {"yellow ochre", 1.6}, {"green earth", 0.8}, {"raw umber", 0.3}}
local sw = {}
for i, p in ipairs{c1, c2, c3, c4, c5, c6, c7} do sw[i] = p end
for i = 1, 7 do
  work(rect(860 + (i-1)*18, 120, 17, 90), {hand="body", tool="filbert 12", pile=sw[i],
       length={20, 50}, coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="found"})
end
print("swatches 1-4 red ladder, 5-7 green ladder")

--@ chunk 320
-- take the lightness from yellow ochre (a light, chromatic earth), not from lead white
d1 = pile{{"yellow ochre", 2.2}, {"vermilion", 1.4}, {"red earth", 0.8}, {"lead white", 0.5}, {"raw umber", 0.2}}
d2 = pile{{"yellow ochre", 2.6}, {"vermilion", 1.0}, {"red earth", 0.6}, {"lead white", 0.6}}
d3 = pile{{"yellow ochre", 2.0}, {"green earth", 1.2}, {"lead white", 0.6}, {"raw umber", 0.2}}
d4 = pile{{"yellow ochre", 2.6}, {"green earth", 0.9}, {"lead white", 1.0}, {"raw umber", 0.2}}
d5 = pile{{"red earth", 2.6}, {"vermilion", 1.2}, {"raw umber", 0.8}, {"yellow ochre", 0.4}}
d6 = pile{{"yellow ochre", 2.2}, {"green earth", 1.8}, {"raw umber", 0.8}, {"red earth", 0.3}}
local sw2 = {d1, d2, d3, d4, d5, d6}
for i = 1, 6 do
  work(rect(866 + (i-1)*20, 232, 19, 88), {hand="body", tool="filbert 12", pile=sw2[i],
       length={20, 50}, coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="found"})
end
print("row 2: 1,2 red lit | 3,4 green lit | 5 red mid | 6 green mid")

--@ chunk 321
g_rb = pile{{"vermilion", 1.6}, {"raw umber", 3}, {"red earth", 2.4}, {"bone black", 1.2}}
g_rm = pile{{"red earth", 2.6}, {"vermilion", 1.2}, {"raw umber", 0.8}, {"yellow ochre", 0.4}}
g_rl = pile{{"yellow ochre", 2.2}, {"vermilion", 1.4}, {"red earth", 0.8}, {"lead white", 0.5}, {"raw umber", 0.2}}
g_rc = pile{{"vermilion", 0.8}, {"raw umber", 3}, {"red earth", 1.6}, {"bone black", 1.4}}
g_hl = pile{{"yellow ochre", 2.6}, {"vermilion", 1.0}, {"red earth", 0.6}, {"lead white", 0.6}}
g_gb = pile{{"green earth", 2.0}, {"Prussian blue", 0.5}, {"raw umber", 2.8}, {"bone black", 1.0}}
g_gm = pile{{"yellow ochre", 2.2}, {"green earth", 1.8}, {"raw umber", 0.8}, {"red earth", 0.3}}
g_gl = pile{{"yellow ochre", 2.0}, {"green earth", 1.2}, {"lead white", 0.6}, {"raw umber", 0.2}}
g_gc = pile{{"green earth", 1.2}, {"Prussian blue", 0.4}, {"raw umber", 3}, {"bone black", 1.4}}
g_hg = pile{{"yellow ochre", 2.6}, {"green earth", 0.9}, {"lead white", 1.0}, {"raw umber", 0.2}}

fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8)

--@ chunk 322
function fruit(a, lay, mid, lit, hi, core, sd)
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  -- no clip: clipping to the shape leaves a sawtooth of dab ends along the contour.
  -- Lay soft, then blend with a generous feather to smooth the contour itself.
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.48, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:grow(10)):soften(18), {angle=0.3})
  work(S * litf(a, -1.0, -0.06), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend((S:grow(8)):soften(14), {angle=0.3})
  -- the reflected light is a thin quiet band hugging the lower edge; a wide one just
  -- washes out the core shadow it is supposed to sit beside
  work(S:rim(a.r * 0.10, 7) * litf(a, -0.7, 0.12) * ellipse(a.cx, a.cy + a.r * 0.85, a.r * 0.8, a.r * 0.5),
       {hand="glaze", pile=g_cm, coverage=1.1})
  stipple(hoff(a, 0.5):blur(a.r * 0.18), {pile=hi, width=4, coverage=3,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8)

--@ chunk 323
function fruit(a, lay, mid, lit, hi, core, sd)
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.48, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  -- blend the CONTOUR only. A blend over the whole fruit averages the modelling away.
  blend(S:rim(16, 9), {angle=0.3})
  work(S * litf(a, -1.0, -0.06), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(14, 8), {angle=0.3})
  work(S:rim(a.r * 0.10, 7) * litf(a, -0.7, 0.12) * ellipse(a.cx, a.cy + a.r * 0.85, a.r * 0.8, a.r * 0.5),
       {hand="glaze", pile=g_cm, coverage=1.1})
  stipple(hoff(a, 0.5):blur(a.r * 0.18), {pile=hi, width=4, coverage=3,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5)

--@ chunk 324
function fruit(a, lay, mid, lit, hi, core, sd)
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.48, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 11), {angle=0.3})
  work(S * litf(a, -1.0, -0.06), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(16, 9), {angle=0.3})
  work(S:rim(a.r * 0.10, 7) * litf(a, -0.7, 0.12) * ellipse(a.cx, a.cy + a.r * 0.85, a.r * 0.8, a.r * 0.5),
       {hand="glaze", pile=g_cm, coverage=1.1})
  stipple(hoff(a, 0.5):blur(a.r * 0.18), {pile=hi, width=4, coverage=3,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8)

--@ chunk 325
-- the two apples' shadows on the cloth, thrown away from the light and joined to
-- the bowl's own shadow so the light reads as one family
for _, a in ipairs{AP[4], AP[5]} do
  work(CLOTH * (ellipse(a.cx + a.r*0.62, a.cy + a.r*0.42, a.r*1.25, a.r*0.55):blur(a.r*0.5)),
       {hand="body", tool="filbert 15", pile=g_cm, length={40, 110}, coverage=2.4, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(CLOTH * (ellipse(a.cx + a.r*0.34, a.cy + a.r*0.72, a.r*0.8, a.r*0.26):blur(a.r*0.16)),
       {hand="body", tool="filbert 12", pile=g_cd, length={30, 80}, coverage=2.6, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17)

--@ chunk 326
-- a fruit over the white cloth needs far more paint than one over a dark ground:
-- at coverage 3.4 the films are only half hiding and the cloth reads straight through
function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=3.4*k, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.48, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 11), {angle=0.3})
  work(S * litf(a, -1.0, -0.06), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(16, 9), {angle=0.3})
  work(S:rim(a.r * 0.10, 7) * litf(a, -0.7, 0.12) * ellipse(a.cx, a.cy + a.r * 0.85, a.r * 0.8, a.r * 0.5),
       {hand="glaze", pile=g_cm, coverage=0.9})
  stipple(hoff(a, 0.5):blur(a.r * 0.18), {pile=hi, width=4, coverage=2.4,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 2.1)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 2.1)

--@ chunk 327
-- Prussian blue in the core reads teal over white cloth; take it out
g_gc = pile{{"green earth", 1.4}, {"Prussian blue", 0.15}, {"raw umber", 3}, {"bone black", 1.2}}
g_gb = pile{{"green earth", 2.0}, {"Prussian blue", 0.2}, {"raw umber", 2.8}, {"bone black", 1.0}}

function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  -- lay-in clipped so the silhouette is clean; the rim blend below softens it again
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found", clip=true})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.48, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(22, 13), {angle=0.3})
  work(S * litf(a, -1.0, -0.06), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(18, 10), {angle=0.3})
  -- a NARROW band of bounced light, low down. Wide and strong, it reads as frost.
  work(S:rim(a.r * 0.07, 5) * litf(a, -0.6, 0.15) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.5})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 1.3)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 1.3)

--@ chunk 328
-- paint out the colour test
work(rect(850, 108, 150, 226), {hand="body", tool="filbert 16", pile=g_dk, length={40, 100},
     coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(rect(838, 96, 165, 240), {hand="glaze", pile=g_wl, coverage=2.2, pressure={0.7, 0.45}})
work(ellipse(930, 430, 420, 300):blur(200), {hand="glaze", pile=g_dk, coverage=1.8})

-- shadows only where cloth is actually seen
allap = BOWL:grow(2)
for _, a in ipairs(AP) do allap = allap + ashape(a):grow(2) end
free2 = CLOTH - allap
-- one shadow family thrown down and to the right, away from the upper-left light
work(free2 * (ellipse(775, 622, 265, 72):blur(62)), {hand="body", tool="filbert 15", pile=g_cd,
     length={40, 110}, coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(free2 * (ellipse(655, 601, 125, 30):blur(20)), {hand="body", tool="filbert 12", pile=g_cd,
     length={30, 80}, coverage=4.5, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
for _, a in ipairs{AP[4], AP[5]} do
  work(free2 * (ellipse(a.cx + a.r*0.70, a.cy + a.r*0.46, a.r*1.30, a.r*0.58):blur(a.r*0.50)),
       {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=3.8, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(free2 * (ellipse(a.cx + a.r*0.30, a.cy + a.r*0.78, a.r*0.78, a.r*0.24):blur(a.r*0.15)),
       {hand="body", tool="filbert 11", pile=g_cd, length={25, 70}, coverage=4.5, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end

--@ chunk 329
-- raw umber is a LIGHT earth, so a "dark" pile built mostly on it is not dark at all.
-- Put the weight on bone black for the darks and on lead white for the lights.
r_lay  = pile{{"bone black", 1.6}, {"raw umber", 2}, {"red earth", 2.4}, {"vermilion", 1.2}}
r_mid  = pile{{"red earth", 2.4}, {"vermilion", 1.4}, {"raw umber", 0.6}, {"bone black", 0.5}}
r_lit  = pile{{"lead white", 2.5}, {"yellow ochre", 2}, {"vermilion", 0.8}, {"red earth", 0.5}}
r_core = pile{{"bone black", 2.4}, {"raw umber", 2}, {"red earth", 1.2}, {"vermilion", 0.4}}
g_lay  = pile{{"bone black", 1.5}, {"green earth", 2.2}, {"raw umber", 1.8}, {"Prussian blue", 0.2}}
g_mid  = pile{{"green earth", 2.2}, {"yellow ochre", 1.4}, {"raw umber", 0.8}, {"bone black", 0.4}}
g_lit  = pile{{"lead white", 2.2}, {"yellow ochre", 1.8}, {"green earth", 0.9}}
g_core = pile{{"bone black", 2.2}, {"green earth", 1.2}, {"raw umber", 1.8}, {"Prussian blue", 0.15}}
local sw = {r_lay, r_mid, r_lit, r_core, g_lay, g_mid, g_lit, g_core}
for i, p in ipairs(sw) do
  work(rect(848 + (i-1)*18, 130, 17, 150), {hand="body", tool="filbert 12", pile=p,
       length={20, 50}, coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="found"})
end
print("red: lay mid lit core | green: lay mid lit core")

--@ chunk 330
g_rb, g_rm, g_rl, g_rc = r_lay, r_mid, r_lit, r_core
g_gb, g_gm, g_gl, g_gc = g_lay, g_mid, g_lit, g_core
g_hl = pile{{"lead white", 3.4}, {"yellow ochre", 1.6}, {"vermilion", 0.5}, {"red earth", 0.3}}
g_hg = pile{{"lead white", 3.0}, {"yellow ochre", 1.8}, {"green earth", 0.5}}

function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found", clip=true})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(22, 13), {angle=0.3})
  work(S * litf(a, -1.0, -0.02), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.4*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(18, 10), {angle=0.3})
  work(S:rim(a.r * 0.07, 5) * litf(a, -0.6, 0.15) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.5})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
-- back to front
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5, 1.15)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.15)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8, 1.15)
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 1.4)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 1.4)

--@ chunk 331
local a = AP[2]
local S = ellipse(a.cx, a.cy, a.r, a.r*0.95):roughen(2.5, 70, 3):soften(2.5)
local core_m = S * litf(a, -1.0, -0.02)
local lit_m  = S * litf(a, 0.45, 0.95)
local mid_m  = S * litf(a, 0.0, 0.55)
print(string.format("areas: S %.0f  mid %.0f  lit %.0f  core %.0f", S:area(), mid_m:area(), lit_m:area(), core_m:area()))
local pts = {{"centre", 0, 0}, {"upper-left", -0.5, -0.5}, {"right", 0.85, 0}, {"lower-right", 0.5, 0.5}, {"lower-left", -0.5, 0.6}}
for _, p in ipairs(pts) do
  local x, y = a.cx + p[2]*a.r, a.cy + p[3]*a.r*0.96
  print(string.format("  %-11s S=%.2f  mid=%.2f  lit=%.2f  core=%.2f",
    p[1], S:at(x, y), mid_m:at(x, y), lit_m:at(x, y), core_m:at(x, y)))
end

--@ chunk 332
-- the core shadow is the FAR side from the light, so it is one MINUS the light mask
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
local a = AP[3]
local S = ellipse(a.cx, a.cy, a.r, a.r*0.95):roughen(2.5, 70, 8):soften(2.5)
local cm = S * corem(a, -1.0, 0.05)
print(string.format("core area %.0f of %.0f", cm:area(), S:area()))
for _, p in ipairs{{"upper-left",-0.5,-0.5},{"centre",0,0},{"right",0.85,0},{"lower-right",0.5,0.5}} do
  print(string.format("  %-11s core=%.2f", p[1], cm:at(a.cx + p[2]*a.r, a.cy + p[3]*a.r*0.96)))
end
work(S, {hand="body", tool="filbert 16", pile=g_rb, length={30, 80}, coverage=5, pressure=1,
     fill=true, dips={1, 1.0, 0.0}, edge="found", clip=true})
work(cm, {hand="body", tool="filbert 16", pile=g_rc, length={30, 80}, coverage=3.4, pressure=1,
     fill=true, dips={1, 1.0, 0.0}, edge="soft"})

--@ chunk 333
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end

function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found", clip=true})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(22, 13), {angle=0.3})
  -- the core shadow lies on the FAR side from the light
  work(S * corem(a, -1.0, 0.05), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.4*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(18, 10), {angle=0.3})
  -- a narrow band of light bounced off the cloth, along the lower shadow edge
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5, 1.15)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.15)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8, 1.15)
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 1.4)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 1.4)

--@ chunk 334
-- bury the test swatches properly this time
work(rect(832, 100, 175, 215), {hand="body", tool="filbert 16", pile=g_dk, length={40, 100},
     coverage=7, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.3, edge="soft"})
work(rect(832, 100, 175, 215), {hand="body", tool="filbert 16", pile=g_dk, length={40, 100},
     coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=-0.6, edge="soft"})
work(rect(818, 86, 195, 240), {hand="glaze", pile=g_wl, coverage=2.6, pressure={0.75, 0.5}})
work(ellipse(940, 420, 430, 300):blur(200), {hand="glaze", pile=g_dk, coverage=2.2})

--@ chunk 335
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  -- firm edge and NO clip: clipping stops every bristle on the line and leaves a dotted
  -- fringe; a soft edge scatters half-covered dabs. A found edge, blended afterwards, holds.
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(26, 18), {angle=0.3})
  work(S * corem(a, -1.0, 0.05), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.4*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(22, 15), {angle=0.3})
  blend(S:rim(13, 9), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.15)

--@ chunk 336
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  -- clean the plate first: earlier soft passes scattered specks a little outside the
  -- silhouette, and nothing laid inside the new firm edge can reach them
  work(S:grow(15):soften(9), {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=6,
       pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(26, 18), {angle=0.3})
  work(S * corem(a, -1.0, 0.05), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.4*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(22, 15), {angle=0.3})
  blend(S:rim(13, 9), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.15)

--@ chunk 337
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  -- clean the plate: a FIRM edge on the grown shape too, so the cleanup ring has a
  -- boundary for the blend to fuse instead of a fringe of half-covered dabs
  work(S:grow(14), {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=5, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(30, 22), {angle=0.3})
  work(S * corem(a, -1.0, 0.05), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.4*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(26, 19), {angle=0.3})
  blend(S:rim(15, 11), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.15)

--@ chunk 338
-- clean the whole bowl region back to cloth and bowl, so the fruit can be laid in
-- on a clean ground and every trace of the earlier halos goes with it
bowlzone = ellipse(575, 462, 248, 208)
work(bowlzone * (CLOTH - BOWL:grow(2)), {hand="body", tool="filbert 15", pile=g_cm,
     length={40, 110}, coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
work(bowlzone * BOWL, {hand="body", tool="filbert 14", pile=g_bw, length={35, 95},
     coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.4, edge="soft"})
blend((bowlzone:shrink(4)):soften(5), {angle=0.35})
-- and model the bowl again from clean paint
work(BOWL_IN, {hand="body", tool="filbert 12", pile=g_dk, length={30, 80}, coverage=3.2,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN * (ellipse(452, 466, 122, 48):blur(46)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(452, 458, 142, 64):blur(55)), {hand="body", tool="filbert 9", pile=g_bl,
     length={22, 55}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(742, 514, 118, 64):blur(60)), {hand="body", tool="filbert 9", pile=g_dk,
     length={22, 55}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(462, 548, 155, 88):blur(72)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=2.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(745, 574, 124, 94):blur(80)), {hand="body", tool="filbert 12", pile=g_dk,
     length={30, 80}, coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(600, 654, 200, 62):blur(52)), {hand="glaze", pile=g_cm, coverage=1.6})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})

--@ chunk 339
-- bury the old fruit completely: two heavy passes at crossing angles over the whole zone
for _, ang in ipairs{0.3, -0.7} do
  work(bowlzone * (CLOTH - BOWL:grow(2)), {hand="body", tool="filbert 16", pile=g_cm,
       length={40, 110}, coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
  work(bowlzone * BOWL, {hand="body", tool="filbert 15", pile=g_bw, length={35, 95},
       coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
blend((BOWL:shrink(3)):soften(3), {angle=0.4})
-- model the bowl from clean paint
work(BOWL_IN, {hand="body", tool="filbert 12", pile=g_dk, length={30, 80}, coverage=4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN * (ellipse(452, 466, 122, 48):blur(46)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(452, 458, 142, 64):blur(55)), {hand="body", tool="filbert 9", pile=g_bl,
     length={22, 55}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(742, 514, 118, 64):blur(60)), {hand="body", tool="filbert 9", pile=g_dk,
     length={22, 55}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(462, 548, 155, 88):blur(72)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=2.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(745, 574, 124, 94):blur(80)), {hand="body", tool="filbert 12", pile=g_dk,
     length={30, 80}, coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})

--@ chunk 340
-- the fruit was laid twelve deep; it needs more than ten to go
for _, ang in ipairs{0.9, 0.1, -0.4} do
  work(bowlzone * (CLOTH - BOWL:grow(2)), {hand="body", tool="filbert 16", pile=g_cm,
       length={40, 110}, coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
  work(bowlzone * BOWL, {hand="body", tool="filbert 15", pile=g_bw, length={35, 95},
       coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
-- rebuild the bowl on a clean ground
work(BOWL_IN, {hand="body", tool="filbert 12", pile=g_dk, length={30, 80}, coverage=6,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM, {hand="body", tool="filbert 9", pile=g_bw, length={22, 55}, coverage=4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY, {hand="body", tool="filbert 12", pile=g_bw, length={30, 80}, coverage=4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})
work(BOWL_IN * (ellipse(452, 464, 124, 48):blur(44)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(452, 458, 142, 64):blur(55)), {hand="body", tool="filbert 9", pile=g_bl,
     length={22, 55}, coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(742, 514, 118, 64):blur(60)), {hand="body", tool="filbert 9", pile=g_dk,
     length={22, 55}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(462, 548, 155, 88):blur(72)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(745, 574, 124, 94):blur(80)), {hand="body", tool="filbert 12", pile=g_dk,
     length={30, 80}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:grow(2)):soften(4), {angle=0.4})

--@ chunk 341
-- a properly dark pile: all that raw umber in the old one kept it at mid value
g_dk2 = pile{{"bone black", 5}, {"raw umber", 1.5}}
-- clean ALL THREE regions of the zone: wall, cloth, bowl. The ghosts were surviving
-- above and left of the bowl because that part of the zone is neither cloth nor bowl.
rest = bowlzone * (-(CLOTH + BOWL:grow(2)))
for _, ang in ipairs{0.4, -0.6} do
  work(rest, {hand="body", tool="filbert 15", pile=g_wl, length={40, 110}, coverage=5, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
  work(rest, {hand="body", tool="filbert 15", pile=g_dk2, length={40, 110}, coverage=3, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
work(bowlzone * BOWL, {hand="body", tool="filbert 14", pile=g_bw, length={35, 95}, coverage=5,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.3, edge="soft"})
work(BOWL_IN, {hand="body", tool="filbert 12", pile=g_dk2, length={30, 80}, coverage=5,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(3)):soften(3), {angle=0.4})
work(BOWL_IN * (ellipse(452, 464, 124, 48):blur(44)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(452, 458, 142, 64):blur(55)), {hand="body", tool="filbert 9", pile=g_bl,
     length={22, 55}, coverage=3.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(742, 514, 118, 64):blur(60)), {hand="body", tool="filbert 9", pile=g_dk2,
     length={22, 55}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(462, 548, 155, 88):blur(72)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(745, 574, 124, 94):blur(80)), {hand="body", tool="filbert 12", pile=g_dk2,
     length={30, 80}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:grow(2)):soften(4), {angle=0.4})

--@ chunk 342
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(24, 17), {angle=0.3})
  work(S * corem(a, -1.0, 0.05), {hand="body", tool=tool, pile=core, length={30, 80}, coverage=3.4*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 14), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5, 1.15)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.15)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8, 1.15)

--@ chunk 343
frontzone = ellipse(390, 690, 238, 142)
-- clean the cloth under the two front apples, then put its light and folds back
for _, ang in ipairs{0.2, -0.8} do
  work(frontzone, {hand="body", tool="filbert 15", pile=g_cm, length={40, 110}, coverage=5,
       pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
work(frontzone * clpool, {hand="body", tool="filbert 15", pile=g_cl, length={40, 110}, coverage=3,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
work(frontzone * btr, {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=2.6,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
work(frontzone * brg, {hand="body", tool="filbert 13", pile=g_cl, length={30, 85}, coverage=2.2,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
blend((frontzone:shrink(5)):soften(6), {angle=0.2})
-- and their shadows again, on clean cloth
allap = BOWL:grow(2)
for _, a in ipairs(AP) do allap = allap + ashape(a):grow(2) end
free2 = CLOTH - allap
for _, a in ipairs{AP[4], AP[5]} do
  work(free2 * (ellipse(a.cx + a.r*0.70, a.cy + a.r*0.46, a.r*1.30, a.r*0.58):blur(a.r*0.50)),
       {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=3.8, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(free2 * (ellipse(a.cx + a.r*0.30, a.cy + a.r*0.78, a.r*0.78, a.r*0.24):blur(a.r*0.15)),
       {hand="body", tool="filbert 11", pile=g_cd, length={25, 70}, coverage=4.5, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end

--@ chunk 344
-- the old fruit lay nearly twenty deep; twenty-four goes over the top of it
for _, ang in ipairs{0.2, -0.8, 0.5, -0.3} do
  work(frontzone, {hand="body", tool="filbert 16", pile=g_cm, length={40, 110}, coverage=6,
       pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
work(frontzone * clpool, {hand="body", tool="filbert 15", pile=g_cl, length={40, 110}, coverage=3.4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
work(frontzone * btr, {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=2.8,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
work(frontzone * brg, {hand="body", tool="filbert 13", pile=g_cl, length={30, 85}, coverage=2.4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
work(frontzone * (ellipse(760, 845, 340, 120):blur(110)), {hand="body", tool="filbert 14", pile=g_cd,
     length={35, 95}, coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})

--@ chunk 345
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k, ct)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(24, 17), {angle=0.3})
  work(S * corem(a, -1.0, ct or 0.05), {hand="body", tool=tool, pile=core, length={30, 80},
           coverage=3.4*k, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 14), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
-- shadows on the cleaned cloth, then the fruit over them
allap = BOWL:grow(2)
for _, a in ipairs(AP) do allap = allap + ashape(a):grow(2) end
free2 = CLOTH - allap
for _, a in ipairs{AP[4], AP[5]} do
  work(free2 * (ellipse(a.cx + a.r*0.70, a.cy + a.r*0.46, a.r*1.30, a.r*0.58):blur(a.r*0.50)),
       {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=3.8, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(free2 * (ellipse(a.cx + a.r*0.30, a.cy + a.r*0.78, a.r*0.78, a.r*0.24):blur(a.r*0.15)),
       {hand="body", tool="filbert 11", pile=g_cd, length={25, 70}, coverage=4.5, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 1.4, -0.25)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 1.4, -0.25)

--@ chunk 346
-- over a white cloth a fruit must be laid THICK or the blend lifts the ground through
-- it and the apple goes pale. Same passes, much more paint, smaller blends.
function fruit(a, lay, mid, lit, hi, core, sd, k, ct)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5*k, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 14), {angle=0.3})
  work(S * corem(a, -1.0, ct or 0.05), {hand="body", tool=tool, pile=core, length={30, 80},
           coverage=3.4*k, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(15, 10), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 2.2, -0.25)

--@ chunk 347
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 2.2, -0.25)

--@ chunk 348
-- deeper greens, and no Prussian blue in the core: over white it goes blue-grey
g_gb = pile{{"bone black", 2.0}, {"green earth", 2.2}, {"raw umber", 1.8}}
g_gm = pile{{"green earth", 2.4}, {"yellow ochre", 1.2}, {"raw umber", 1.0}, {"bone black", 0.6}}
g_gl = pile{{"lead white", 2.2}, {"yellow ochre", 2.0}, {"green earth", 1.0}, {"raw umber", 0.3}}
g_gc = pile{{"bone black", 2.6}, {"green earth", 1.4}, {"raw umber", 1.8}}
-- a core with no flat top: darkest only at the silhouette, rising smoothly to the terminator
function fruit(a, lay, mid, lit, hi, core, sd, k, ct)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5*k, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 14), {angle=0.3})
  work(S * corem(a, -0.9, ct or 0.3), {hand="body", tool=tool, pile=core, length={30, 80},
           coverage=4.0*k, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(15, 10), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 2.2, 0.3)

--@ chunk 349
-- the bowl has to READ: deepen the inside, strengthen the wall, and put a crisp light
-- on the rim's upper-left arc. That one thin line is what describes the ellipse.
work(BOWL_IN, {hand="body", tool="filbert 13", pile=g_dk2, length={30, 80}, coverage=4.5,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN * (ellipse(455, 462, 118, 44):blur(40)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
-- the wall: a definite light on the left, deep on the right
work(BOWL_BODY, {hand="body", tool="filbert 12", pile=g_bw, length={30, 80}, coverage=3,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(468, 546, 150, 86):blur(68)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(748, 576, 122, 92):blur(76)), {hand="body", tool="filbert 12", pile=g_dk2,
     length={30, 80}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM, {hand="body", tool="filbert 9", pile=g_bw, length={22, 55}, coverage=3,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(2)):soften(2), {angle=0.4})
-- a crisp light along the rim, strong at the left and dying away to the right
local rb = brush{kind="filbert", width=5, point=0.7}
rb:load(g_bl, 0.95)
rb:stroke(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 3.34, 5.62, 26),
  {pressure={0.95, 0.15}, ramps={0.06, 0.3}, orient="across", swell={1, 1.15, 0.7}, clip=BOWL:grow(3)})
-- and a thin dark along the rim where it turns away
rb:wipe(0.9); rb:load(g_dk2, 0.9)
rb:stroke(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 5.30, 6.35, 16),
  {pressure={0.25, 0.8}, ramps={0.05, 0.3}, orient="across", clip=BOWL:grow(3)})

--@ chunk 350
-- soften the two hard contact bars under the front apples, then re-lay them wider
allap = BOWL:grow(2)
for _, a in ipairs(AP) do allap = allap + ashape(a):grow(2) end
free2 = CLOTH - allap
for _, a in ipairs{AP[4], AP[5]} do
  work(free2 * (ellipse(a.cx + a.r*0.45, a.cy + a.r*0.62, a.r*1.5, a.r*0.8):blur(a.r*0.5)),
       {hand="body", tool="filbert 14", pile=g_cm, length={35, 95}, coverage=3, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(free2 * (ellipse(a.cx + a.r*0.34, a.cy + a.r*0.80, a.r*0.85, a.r*0.30):blur(a.r*0.28)),
       {hand="body", tool="filbert 12", pile=g_cd, length={30, 80}, coverage=3, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end
-- stems: a small dark dimple and a short brown stalk, the one thing that says "apple"
g_st = pile{{"raw umber", 2}, {"bone black", 1.1}, {"red earth", 1.2}}
sb = brush{kind="filbert", width=4.5, point=0.85}
for i, a in ipairs(AP) do
  local dx, dy = a.cx - a.r*0.04, a.cy - a.r*0.86
  work(ellipse(dx, dy, a.r*0.17, a.r*0.11):soften(2), {hand="body", tool="filbert 7", pile=g_st,
       length={14, 30}, coverage=3, pressure=0.9, fill=true, edge="soft"})
  sb:load(g_st, 0.9)
  sb:stroke({{dx, dy}, {dx + a.r*0.05, dy - a.r*0.14}, {dx + a.r*0.14, dy - a.r*0.26}},
    {pressure={0.85, 0.35}, ramps={0.08, 0.35}, orient="across", swell={1, 1, 0.6}})
end

--@ chunk 351
allap = BOWL:grow(2)
for _, a in ipairs(AP) do allap = allap + ashape(a):grow(6) end
room = everywhere() - allap:grow(8)      -- the room, but never across the still life
-- one soft light from the upper left, laid by stipple so it stays smooth
stipple(room * pool1, {pile=g_wm, width=5, coverage=8, pressure={0.55, 0.9}, cluster=0.25, dips={7, 1.0, 0.3}})
stipple(room * pool2, {pile=g_wt, width=4, coverage=8, pressure={0.55, 0.9}, cluster=0.25, dips={7, 1.0, 0.3}})
-- and the far side falls away, so the light has somewhere to come from
work(room * (ellipse(920, 420, 430, 300):blur(200)), {hand="glaze", pile=g_dk2, coverage=2.2})
work(room * (ellipse(600, 430, 300, 190):blur(150)), {hand="glaze", pile=g_dk2, coverage=1.6})
work(room * (ellipse(80, 760, 300, 200):blur(150)), {hand="glaze", pile=g_dk2, coverage=1.4})

--@ chunk 352
-- bury the stipple: the wall back to its dark, and the cloth's upper band back to cloth
for _, ang in ipairs{0.3, -0.7, 0.9} do
  work(wall, {hand="body", tool="filbert 18", pile=g_wl, length={50, 130}, coverage=6, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
blend((wall:shrink(6)):soften(6), {angle=0.3})
band = CLOTH * rect(-6, 470, 610, 200)
for _, ang in ipairs{0.2, -0.8, 0.5} do
  work(band, {hand="body", tool="filbert 16", pile=g_cm, length={40, 110}, coverage=6, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
work(band * clpool, {hand="body", tool="filbert 15", pile=g_cl, length={40, 110}, coverage=3.4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
work(band * btr, {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=2.8,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
work(band * brg, {hand="body", tool="filbert 13", pile=g_cl, length={30, 85}, coverage=2.4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
blend((band:grow(8)):soften(10), {angle=0.2})

--@ chunk 353
-- the table still held a strip of the stipple, and the band left a hard rectangle
for _, ang in ipairs{0.1, -0.9, 0.6} do
  work(table_, {hand="body", tool="filbert 18", pile=g_tb, length={50, 130}, coverage=6, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
work(table_ * (ellipse(130, 452, 300, 100):blur(110)), {hand="body", tool="filbert 15", pile=g_wl,
     length={40, 110}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(table_ * (ellipse(880, 700, 420, 300):blur(200)), {hand="glaze", pile=g_dk2, coverage=2})
-- blend the whole cloth so the band's rectangle dissolves into it
blend((CLOTH:shrink(6)):soften(6), {angle=0.2})
-- and put the front apple back, its top had cloth painted over it
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 2.2, 0.3)

--@ chunk 354
-- the visible table is only the strip above the cloth; table_ itself runs under everything
tablevis = table_ - CLOTH
print(string.format("visible table %.0f, cloth %.0f", tablevis:area(), CLOTH:area()))
-- the cloth, laid again
for _, ang in ipairs{0.2, -0.8, 0.5} do
  work(CLOTH, {hand="body", tool="filbert 16", pile=g_cm, length={40, 110}, coverage=6, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
work(CLOTH * clpool, {hand="body", tool="filbert 15", pile=g_cl, length={40, 110}, coverage=3.4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.2, edge="soft"})
work(CLOTH * btr, {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=2.8,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
work(CLOTH * brg, {hand="body", tool="filbert 13", pile=g_cl, length={30, 85}, coverage=2.4,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=1.0, edge="soft"})
work(CLOTH * (ellipse(930, 690, 300, 190):blur(170)), {hand="body", tool="filbert 15", pile=g_cd,
     length={40, 110}, coverage=2.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(CLOTH * (ellipse(760, 845, 340, 120):blur(110)), {hand="body", tool="filbert 15", pile=g_cd,
     length={40, 110}, coverage=2.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((CLOTH:shrink(6)):soften(6), {angle=0.2})

--@ chunk 355
-- the group's shadows on the clean cloth
free2 = CLOTH - BOWL:grow(2)
for _, a in ipairs(AP) do free2 = free2 - ashape(a):grow(2) end
work(free2 * (ellipse(775, 622, 265, 72):blur(62)), {hand="body", tool="filbert 15", pile=g_cd,
     length={40, 110}, coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(free2 * (ellipse(655, 601, 125, 30):blur(22)), {hand="body", tool="filbert 12", pile=g_cd,
     length={30, 80}, coverage=4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
for _, a in ipairs{AP[4], AP[5]} do
  work(free2 * (ellipse(a.cx + a.r*0.70, a.cy + a.r*0.46, a.r*1.30, a.r*0.58):blur(a.r*0.50)),
       {hand="body", tool="filbert 14", pile=g_cd, length={35, 95}, coverage=3.8, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(free2 * (ellipse(a.cx + a.r*0.34, a.cy + a.r*0.80, a.r*0.85, a.r*0.30):blur(a.r*0.28)),
       {hand="body", tool="filbert 12", pile=g_cd, length={30, 80}, coverage=3.4, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end
-- the bowl again
for _, ang in ipairs{0.3, -0.6} do
  work(BOWL, {hand="body", tool="filbert 14", pile=g_bw, length={35, 95}, coverage=5, pressure=1,
       fill=true, dips={1, 1.0, 0.0}, angle=ang, edge="soft"})
end
blend((BOWL:shrink(2)):soften(2), {angle=0.4})
work(BOWL_IN, {hand="body", tool="filbert 13", pile=g_dk2, length={30, 80}, coverage=4.5,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN * (ellipse(455, 462, 118, 44):blur(40)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(468, 546, 150, 86):blur(68)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(748, 576, 122, 92):blur(76)), {hand="body", tool="filbert 12", pile=g_dk2,
     length={30, 80}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM, {hand="body", tool="filbert 9", pile=g_bw, length={22, 55}, coverage=3,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(452, 458, 142, 64):blur(55)), {hand="body", tool="filbert 9", pile=g_bl,
     length={22, 55}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(742, 514, 118, 64):blur(60)), {hand="body", tool="filbert 9", pile=g_dk2,
     length={22, 55}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
blend((BOWL:shrink(2)):soften(2), {angle=0.4})
local rb = brush{kind="filbert", width=5, point=0.7}
rb:load(g_bl, 0.95)
rb:stroke(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 3.34, 5.62, 26),
  {pressure={0.95, 0.15}, ramps={0.06, 0.3}, orient="across", swell={1, 1.15, 0.7}, clip=BOWL:grow(3)})
rb:wipe(0.9); rb:load(g_dk2, 0.9)
rb:stroke(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 5.30, 6.35, 16),
  {pressure={0.25, 0.8}, ramps={0.05, 0.3}, orient="across", clip=BOWL:grow(3)})

--@ chunk 356
-- a shadow on a white cloth has to be genuinely dark or it does not read at all
g_sh = pile{{"bone black", 1.3}, {"raw umber", 2}, {"pale smalt", 1}, {"lead white", 0.7}}
free2 = CLOTH - BOWL:grow(2)
for _, a in ipairs(AP) do free2 = free2 - ashape(a):grow(2) end
work(free2 * (ellipse(790, 626, 270, 76):blur(60)), {hand="body", tool="filbert 15", pile=g_sh,
     length={40, 110}, coverage=4.5, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(free2 * (ellipse(662, 602, 128, 30):blur(22)), {hand="body", tool="filbert 12", pile=g_sh,
     length={30, 80}, coverage=5, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
for _, a in ipairs{AP[4], AP[5]} do
  work(free2 * (ellipse(a.cx + a.r*0.70, a.cy + a.r*0.46, a.r*1.30, a.r*0.58):blur(a.r*0.50)),
       {hand="body", tool="filbert 14", pile=g_sh, length={35, 95}, coverage=4, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(free2 * (ellipse(a.cx + a.r*0.34, a.cy + a.r*0.80, a.r*0.85, a.r*0.30):blur(a.r*0.28)),
       {hand="body", tool="filbert 12", pile=g_sh, length={30, 80}, coverage=4.5, pressure=1,
        fill=true, dips={1, 1.0, 0.0}, edge="soft"})
end
-- the bowl with a firm edge, then its light back on top
work(BOWL, {hand="body", tool="filbert 14", pile=g_bw, length={35, 95}, coverage=5, pressure=1,
     fill=true, dips={1, 1.0, 0.0}, edge="found"})
blend((BOWL:shrink(2)):soften(2), {angle=0.4})
work(BOWL_IN, {hand="body", tool="filbert 13", pile=g_dk2, length={30, 80}, coverage=4.5,
     pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_IN * (ellipse(450, 460, 122, 44):blur(38)), {hand="body", tool="filbert 10", pile=g_bw,
     length={25, 60}, coverage=3.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(448, 456, 148, 68):blur(52)), {hand="body", tool="filbert 9", pile=g_bl,
     length={22, 55}, coverage=3.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_RIM * (ellipse(748, 518, 116, 64):blur(58)), {hand="body", tool="filbert 9", pile=g_dk2,
     length={22, 55}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(464, 544, 155, 88):blur(64)), {hand="body", tool="filbert 12", pile=g_bl,
     length={30, 80}, coverage=3.8, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(BOWL_BODY * (ellipse(752, 578, 120, 92):blur(72)), {hand="body", tool="filbert 12", pile=g_dk2,
     length={30, 80}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})

--@ chunk 357
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k, ct)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5*k, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 14), {angle=0.3})
  work(S * corem(a, -0.9, ct or 0.3), {hand="body", tool=tool, pile=core, length={30, 80},
           coverage=4.0*k, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(15, 10), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.6})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5, 1.3, 0.3)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.3, 0.3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8, 1.3, 0.3)
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 2.2, 0.3)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 2.2, 0.3)

--@ chunk 358
g_st = pile{{"raw umber", 2}, {"bone black", 1.1}, {"red earth", 1.2}}
sb = brush{kind="filbert", width=5, point=0.85}
for i, a in ipairs(AP) do
  local dx, dy = a.cx - a.r*0.04, a.cy - a.r*0.86
  work(ellipse(dx, dy, a.r*0.18, a.r*0.12):soften(2.5), {hand="body", tool="filbert 7", pile=g_st,
       length={14, 30}, coverage=3.2, pressure=0.9, fill=true, edge="soft"})
  sb:load(g_st, 0.9)
  sb:stroke({{dx, dy}, {dx + a.r*0.05, dy - a.r*0.15}, {dx + a.r*0.15, dy - a.r*0.30}},
    {pressure={0.85, 0.3}, ramps={0.08, 0.35}, orient="across", swell={1, 1, 0.55}})
end
-- the wall's light, laid with body paint and fused. Stippling a lighter pile over the
-- dark wall at high coverage tears it into orange stripes; never again.
groupm = BOWL:grow(10)
for _, a in ipairs(AP) do groupm = groupm + ashape(a):grow(10) end
wallfree = wall - groupm
work(wallfree * pool1, {hand="body", tool="filbert 17", pile=g_wm, length={45, 120},
     coverage=2.2, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.35, edge="soft"})
work(wallfree * pool2, {hand="body", tool="filbert 15", pile=g_wt, length={40, 110},
     coverage=1.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, angle=0.45, edge="soft"})
blend((wallfree * pool1:grow(30)):soften(30), {angle=0.35})
work(wallfree * (ellipse(940, 400, 400, 280):blur(190)), {hand="glaze", pile=g_dk2, coverage=2.2})

--@ chunk 359
-- smooth out the hard vertical boundary the light left in the upper wall
blend((wallfree * (ellipse(400, 180, 620, 380):blur(150))):grow(20):soften(40), {angle=0.35})
-- the cloth's far side must fall away, or the light has no direction
work(CLOTH * (ellipse(960, 680, 300, 210):blur(160)), {hand="body", tool="filbert 16", pile=g_cd,
     length={40, 110}, coverage=3.4, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(CLOTH * (ellipse(820, 830, 330, 130):blur(120)), {hand="body", tool="filbert 16", pile=g_cd,
     length={40, 110}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
work(CLOTH * (ellipse(1000, 560, 220, 180):blur(120)), {hand="body", tool="filbert 14", pile=g_cd,
     length={35, 95}, coverage=3, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
-- the table strip in shadow, tying the cloth's shadow to the wall's
work(tablevis * (ellipse(900, 440, 340, 130):blur(120)), {hand="glaze", pile=g_dk2, coverage=2.2})

--@ chunk 360
groupm = BOWL
for _, a in ipairs(AP) do groupm = groupm + ashape(a) end
-- a warm glaze over the whole group, so the fruit and the bowl sit in the same air
g_warm = pile{{"raw umber", 2}, {"red earth", 1.2}, {"yellow ochre", 0.7}, {"lead white", 0.4}}
work(groupm:grow(2), {hand="glaze", tool="filbert 26", pile=g_warm, length={120, 280},
     coverage=0.9, pressure={0.5, 0.3}, angle=0.4, edge="soft"})
-- a cool glaze in the room's corners, which is what makes the darks read as air
g_cool = pile{{"bone black", 3}, {"pale smalt", 1.2}}
work(ellipse(960, 400, 380, 300):blur(190), {hand="glaze", pile=g_cool, coverage=1.6})
work(ellipse(120, 780, 330, 220):blur(170), {hand="glaze", pile=g_cool, coverage=1.4})
work(everywhere() * (ellipse(500, 40, 700, 180):blur(140)), {hand="glaze", pile=g_cool, coverage=1.0})
-- crisp accents last: the bowl's rim, its contact on the cloth, and the two near fruit
local rb = brush{kind="filbert", width=5, point=0.75}
rb:load(g_bl, 0.95)
rb:stroke(arc(BOWLX, BOWLY, BOWLRX, BOWLRY, 3.36, 5.55, 24),
  {pressure={0.9, 0.12}, ramps={0.06, 0.3}, orient="across", swell={1, 1.1, 0.7}, clip=BOWL:grow(2)})
rb:wipe(0.92); rb:load(g_dk2, 0.85)
rb:stroke({{418, 560}, {500, 596}, {590, 606}, {676, 596}, {748, 566}},
  {pressure={0.5, 0.8}, ramps={0.1, 0.25}, orient="across", clip=BOWL:grow(2)})
-- a small light on the two fruit nearest the eye
local hb = brush{kind="filbert", width=9, point=0.8}
for _, a in ipairs{AP[4], AP[5]} do
  hb:load(a.k == "g" and g_hg or g_hl, 0.7)
  hb:touch(a.cx + LX*a.r*0.55, a.cy + LY*a.r*0.55, {pressure=0.42, clip=ashape(a)})
end

--@ chunk 361
-- the fruit was sitting at the wall's own value: lift its lights, deepen its cores
for _, a in ipairs(AP) do
  local S = ashape(a)
  work(S * litf(a, 0.40, 0.95), {hand="glaze", pile=a.k == "g" and g_hg or g_hl,
       coverage=1.3, pressure={0.55, 0.3}, angle=0.4, edge="soft"})
  work(S * corem(a, -0.95, -0.10), {hand="glaze", pile=g_cool,
       coverage=1.4, pressure={0.6, 0.3}, angle=0.4, edge="soft"})
end
-- deeper darks in the room, and a brighter note on the cloth under the light
work(ellipse(960, 380, 380, 300):blur(180), {hand="glaze", pile=g_dk2, coverage=2.4})
work(ellipse(560, 440, 280, 170):blur(140), {hand="glaze", pile=g_dk2, coverage=1.5})
work(ellipse(960, 760, 320, 220):blur(160), {hand="glaze", pile=g_dk2, coverage=1.8})
work(CLOTH * (ellipse(215, 630, 260, 130):blur(120)), {hand="body", tool="filbert 16", pile=g_cl,
     length={40, 110}, coverage=2.6, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})

--@ chunk 362
function corem(a, t0, t1) return litf(a, t0, t1):map(function(v) return 1 - v end) end
function fruit(a, lay, mid, lit, hi, core, sd, k, ct)
  k = k or 1
  local S = ellipse(a.cx, a.cy, a.r, a.r * 0.95):roughen(2.5, 70, sd):soften(2.5)
  local tool = "filbert " .. math.max(9, math.floor(a.r / 5))
  work(S, {hand="body", tool=tool, pile=lay, length={30, 80}, coverage=4.5*k, pressure=1,
           fill=true, dips={1, 1.0, 0.0}, edge="found"})
  work(S * litf(a, 0.0, 0.55), {hand="body", tool=tool, pile=mid, length={30, 80}, coverage=3.2*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  work(S * litf(a, 0.45, 0.95), {hand="body", tool=tool, pile=lit, length={30, 80}, coverage=3.0*k,
           pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(20, 14), {angle=0.3})
  work(S * corem(a, -0.9, ct or 0.3), {hand="body", tool=tool, pile=core, length={30, 80},
           coverage=4.0*k, pressure=1, fill=true, dips={1, 1.0, 0.0}, edge="soft"})
  blend(S:rim(15, 10), {angle=0.3})
  work(S:rim(a.r * 0.08, 5) * corem(a, -0.6, 0.2) * ellipse(a.cx, a.cy + a.r * 0.9, a.r * 0.7, a.r * 0.42),
       {hand="glaze", pile=g_cm, coverage=0.5})
  stipple(hoff(a, 0.52):blur(a.r * 0.15), {pile=hi, width=4, coverage=2.6,
         pressure={0.5, 0.8}, cluster=0.3, dips={5, 1.0, 0.3}})
end
fruit(AP[1], g_gb, g_gm, g_gl, g_hg, g_gc, 5, 1.6, 0.3)
fruit(AP[2], g_rb, g_rm, g_rl, g_hl, g_rc, 3, 1.6, 0.3)
fruit(AP[3], g_rb, g_rm, g_rl, g_hl, g_rc, 8, 1.6, 0.3)
fruit(AP[4], g_rb, g_rm, g_rl, g_hl, g_rc, 12, 2.4, 0.3)
fruit(AP[5], g_gb, g_gm, g_gl, g_hg, g_gc, 17, 2.4, 0.3)
g_st = pile{{"raw umber", 2}, {"bone black", 1.1}, {"red earth", 1.2}}
sb = brush{kind="filbert", width=5, point=0.85}
for i, a in ipairs(AP) do
  local dx, dy = a.cx - a.r*0.04, a.cy - a.r*0.86
  work(ellipse(dx, dy, a.r*0.18, a.r*0.12):soften(2.5), {hand="body", tool="filbert 7", pile=g_st,
       length={14, 30}, coverage=3.2, pressure=0.9, fill=true, edge="soft"})
  sb:load(g_st, 0.9)
  sb:stroke({{dx, dy}, {dx + a.r*0.05, dy - a.r*0.15}, {dx + a.r*0.15, dy - a.r*0.30}},
    {pressure={0.85, 0.3}, ramps={0.08, 0.35}, orient="across", swell={1, 1, 0.55}})
end
