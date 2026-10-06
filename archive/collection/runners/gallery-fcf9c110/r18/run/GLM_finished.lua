-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
-- Canvas: 40x32 cm, medium-fine linen, warm grey-green oil ground
canvas{size=400, aspect=0.8, linen={16, 12}, seed=11,
  ground={
    {pile={{"lead white", 1}, {"raw umber", 0.18}, {"green earth", 0.1}}, um=95, apply="knife", texture=0.55},
    {pile={{"lead white", 1}, {"raw umber", 0.08}, {"green earth", 0.05}}, um=25, apply="brush"},
  }}
print("grooves set")

--@ chunk 2
-- Palette
sunset   = pile{{"vermilion", 1}, {"chrome yellow", 0.55}, {"lead white", 0.7}}                 -- vermilion & yellow pass to lead
gold     = pile{{"chrome yellow", 1}, {"vermilion", 0.22}, {"lead white", 0.9}}                -- gold & one part of its red to lead
peach    = pile{{"lead white", 2.2}, {"vermilion", 0.55}, {"chrome yellow", 0.3}}              -- lead & two parts red, one yellow
rosepink = pile{{"lead white", 2.2}, {"vermilion", 0.65}}
ice      = pile{{"lead white", 2.0}, {"smalt", 0.75}}
warmpale = pile{{"lead white", 2.0}, {"yellow ochre", 0.32}}
landblue = pile{{"cobalt blue", 1}, {"raw umber", 0.5}, {"lead white", 0.65}}                   -- cobalt & half its umber to lead
shoregrey= pile{{"lead white", 1}, {"yellow ochre", 0.2}, {"bone black", 0.16}}
flesh    = pile{{"lead white", 1.6}, {"vermilion", 0.45}, {"yellow ochre", 0.12}}
mistpink = pile{{"lead white", 1.5}, {"vermilion", 0.3}, {"chrome yellow", 0.18}}
skytop   = pile{{"cobalt blue", 1}, {"raw umber", 0.2}, {"lead white", 0.35}}                   -- cobalt & a fifth of umber to lead
print("piles mixed: 12")

--@ chunk 3
-- Sketch: horizon, headland, spit, boat, path of light, birds
h = pencil("HB")
h:rule({30, 515}, {970, 515}, {pressure=0.28})
h:sketch({{30, 446}, {240, 452}, {460, 452}, {700, 440}, {970, 438}}, {pressure=0.3})
h:sketch({{620, 512}, {700, 516}, {800, 528}, {820, 540}}, {pressure=0.26})
h:sketch({{548, 210}, {600, 166}, {620, 128}, {626, 92}, {612, 48}, {586, 30}}, {pressure=0.3})
h:sketch({{520, 176}, {586, 174}, {640, 190}, {668, 232}, {690, 300}, {700, 372}}, {pressure=0.3})
h:sketch({{660, 436}, {690, 444}, {718, 470}, {724, 500}}, {pressure=0.22})
h:sketch({{150, 660}, {176, 656}, {194, 644}}, {pressure=0.2})
h:sketch({{760, 262}, {772, 256}, {788, 252}}, {pressure=0.2})
print("sketched")

--@ chunk 4
-- Tone: thin warm golden-grey wash over everything
wash = pile{{"vermilion", 0.5}, {"yellow ochre", 0.4}, {"raw umber", 0.35}, {"bone black", 0.16}, medium=0.75}
darkmix = pile{{"bone black", 0.55}, {"raw umber", 1}, {"cobalt blue", 0.7}, medium=0.2}
all = mask(function(x, y) return 1 end)
work(all, {hand="glaze", pile=wash, coverage=1.0, angle=0.05})
print("toned")

--@ chunk 5
wait(6 * 60)

--@ chunk 6
-- Full toned ground: lead-rich warm grey, worked swiftly with the broad hand
tone = pile{{"lead white", 1}, {"yellow ochre", 0.28}, {"raw umber", 0.2}, {"bone black", 0.1}, {"vermilion", 0.07}, medium=0.35}
work(all, {hand="broad", pile=tone, coverage=1.2, angle=0.06, dips={70, 0.9, 0}, edge=1})
print("ground toned")

--@ chunk 7
-- Dense toned ground, covering the orange mess fully
tone = pile{{"lead white", 1}, {"raw umber", 0.5}, {"bone black", 0.22}, {"yellow ochre", 0.25}, {"vermilion", 0.1}, medium=0.3}
work(all, {hand="broad", pile=tone, coverage=2.4, fill=true, angle=0.09, dips={90, 0.95, 0}, edge=1})
print("dense tone on")

--@ chunk 8
tone = pile{{"lead white", 1}, {"raw umber", 0.55}, {"bone black", 0.28}, {"yellow ochre", 0.3}, {"vermilion", 0.12}, medium=0.3}
work(all, {hand="broad", pile=tone, coverage=2.2, fill=true, angle=-0.06, dips={90, 0.95, 0}, edge=1})
print("second dense pass")

--@ chunk 9
blend(all, {coverage=1.0})
print("blended")

--@ chunk 10
wait(6 * 60)
-- Sky palette trials on the spare rect, light pressure so I can compare
sk1 = pile{{"lead white", 1}, {"cobalt blue", 2.5}, {"vermilion", 2}, {"chrome yellow", 2}}
sk2 = pile{{"lead white", 1.5}, {"cobalt blue", 2}, {"vermilion", 1}, {"chrome yellow", 1}}
sk3 = pile{{"lead white", 1}, {"smalt", 2}, {"vermilion", 0.5}}
sk4 = pile{{"lead white", 1.5}, {"yellow ochre", 1}, {"vermilion", 0.4}}
trial = rect(0, 1266, 900, 180)
work(trial, {hand="detail", pile=sk1, coverage=1.6, edge=0.4, ruler=true, clip=true})

--@ chunk 11
-- test if masks work off-canvas; measure canvas bounds
e = mask(function(x, y) return 1 end)
print("probe")

--@ chunk 12
-- probe canvas bounds
probe = rect(0, 1300, 100, 10)
print("area:", probe:area())
r2 = rect(0, 1250, 900, 25)
work(r2, {hand="detail", pile=sk4, coverage=1.5, edge=0.2, clip=true, ruler=true})
print("painted 1250")

--@ chunk 13
-- fill the table of sky trials into the spare band, then look
trials = {sk1, sk2, sk3, sk4}
names = {"sk1","sk2","sk3","sk4"}
for i, m in ipairs(trials) do
  local x0 = 0 + (i-1) * 250
  local band = rect(x0, 1252, 248, 30)
  work(band, {hand="detail", pile=m, coverage=2.0, edge=0.2, clip=true, ruler=true})
end
print("trials painted")

--@ chunk 14
-- coordinate probe: X at (300,300) and (150,1050)
hp = pencil("2B")
hp:line({{280, 280}, {320, 320}}, {pressure=0.7, smooth=false})
hp:line({{130, 1030}, {170, 1070}}, {pressure=0.7, smooth=false})
print("probe lines drawn")

--@ chunk 15
wait(15 * 60)
print(drying(500, 300))
print(drying(500, 900))

--@ chunk 16
-- Sky band 1: top of sky, cool warm-grey blue
skyA = pile{{"lead white", 1}, {"cobalt blue", 2.5}, {"vermilion", 2}, {"chrome yellow", 2}}
top = below(function(x) return 190 + 30 * math.sin(x / 150) + 14 * math.sin(x / 57 + 2) end)
work(top, {hand="broad", pile=skyA, coverage=1.6, fill=true, angle=0.04, dips={70, 0.95, 0}, edge=0.5})
print("band 1")

--@ chunk 17
-- Band 2: cool grey-mauve upper-middle sky
skyB = pile{{"lead white", 2.2}, {"cobalt blue", 1.3}, {"vermilion", 0.8}, {"chrome yellow", 0.5}}
b2 = below(function(x) return 300 + 40 * math.sin(x / 130 + 1) + 18 * math.sin(x / 47) end)
work(b2, {hand="broad", pile=skyB, coverage=1.7, fill=true, angle=0.05, dips={70, 0.95, 0}, edge=0.6})
print("band 2")

--@ chunk 18
-- Bands 3+4: warm grey to pale gold, laid fast and generously
skyC = pile{{"lead white", 2.6}, {"cobalt blue", 0.5}, {"vermilion", 0.9}, {"chrome yellow", 1.1}}
b3 = below(function(x) return 395 + 35 * math.sin(x / 110 + 3) + 16 * math.sin(x / 41 + 1) end)
work(b3, {hand="broad", pile=skyC, coverage=2.2, fill=true, angle=0.05, dips={80, 0.95, 0}, edge=0.6})
print("band 3")

--@ chunk 19
-- Dark slate at the top of the sky
slate = pile{{"lead white", 1.2}, {"cobalt blue", 1.6}, {"bone black", 0.2}, {"raw umber", 0.25}}
topband = above(function(x) return 185 + 30 * math.sin(x / 150) + 14 * math.sin(x / 57 + 2) end)
work(topband, {hand="broad", pile=slate, coverage=2.4, fill=true, angle=0.04, dips={70, 0.95, 0}, edge=0.7})
print("slate top")

--@ chunk 20
-- Darker, greyer slate for the top band
slate2 = pile{{"lead white", 1}, {"cobalt blue", 0.9}, {"bone black", 0.45}, {"raw umber", 0.5}, {"vermilion", 0.15}}
topband = above(function(x) return 185 + 30 * math.sin(x / 150) + 14 * math.sin(x / 57 + 2) end)
work(topband, {hand="broad", pile=slate2, coverage=2.6, fill=true, angle=0.04, dips={70, 0.95, 0}, edge=0.7})
print("slate v2")

--@ chunk 21
-- Sky rebuild: three solid fields, dark to light, then blend
slate3 = pile{{"lead white", 1}, {"cobalt blue", 0.9}, {"bone black", 0.45}, {"raw umber", 0.55}, {"vermilion", 0.15}}
mauve  = pile{{"lead white", 1.6}, {"cobalt blue", 0.5}, {"vermilion", 0.5}, {"chrome yellow", 0.4}, {"raw umber", 0.15}}
goldpale = pile{{"lead white", 2.2}, {"chrome yellow", 0.55}, {"vermilion", 0.2}}

t1 = above(function(x) return 205 + 30 * math.sin(x / 150) + 14 * math.sin(x / 57 + 2) end)
work(t1, {hand="broad", pile=slate3, coverage=3.0, fill=true, angle=0.05, dips={70, 0.95, 0}, edge=1})

t2 = below(function(x) return 355 + 35 * math.sin(x / 130 + 1) + 16 * math.sin(x / 47) end)
work(t2, {hand="broad", pile=mauve, coverage=2.6, fill=true, angle=0.05, dips={70, 0.95, 0}, edge=1})

t3 = below(function(x) return 530 + 30 * math.sin(x / 110 + 3) + 14 * math.sin(x / 43 + 2) end)
work(t3, {hand="broad", pile=goldpale, coverage=2.6, fill=true, angle=0.05, dips={70, 0.95, 0}, edge=1})

print("three fields down")

--@ chunk 22
-- Full-cover repaint of the three sky fields
slate3 = pile{{"lead white", 1}, {"cobalt blue", 0.9}, {"bone black", 0.45}, {"raw umber", 0.55}, {"vermilion", 0.15}}
mauve2 = pile{{"lead white", 1.2}, {"cobalt blue", 0.7}, {"vermilion", 0.35}, {"raw umber", 0.3}, {"chrome yellow", 0.2}}
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}

t1 = above(function(x) return 205 + 30 * math.sin(x / 150) + 14 * math.sin(x / 57 + 2) end)
work(t1, {hand="broad", pile=slate3, coverage=3.2, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=1})

t2 = below(function(x) return 365 + 35 * math.sin(x / 130 + 1) + 16 * math.sin(x / 47) end)
work(t2, {hand="broad", pile=mauve2, coverage=3.0, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=1})

t3 = below(function(x) return 545 + 30 * math.sin(x / 110 + 3) + 14 * math.sin(x / 43 + 2) end)
work(t3, {hand="broad", pile=gold3, coverage=2.8, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=1})

print("fields repainted")

--@ chunk 23
-- Fuse the sky while it is open
skyzone = below(function(x) return 560 + 20 * math.sin(x / 90) end)
blend(skyzone, {coverage=1.0})
print("sky blended")

--@ chunk 24
-- Cloud bank shaping: cool knit on top edge, violet shadow under the left
coolknit = pile{{"lead white", 1.4}, {"cobalt blue", 0.5}, {"vermilion", 0.25}, {"raw umber", 0.2}}
undersh  = pile{{"lead white", 1}, {"cobalt blue", 0.4}, {"vermilion", 0.3}, {"bone black", 0.15}, {"raw umber", 0.3}}

c150 = function(x) return 150 + 25 * math.sin(x / 120 + 1) end
c268 = function(x) return 268 + 22 * math.sin(x / 95 + 2) end
c292 = function(x) return 292 + 25 * math.sin(x / 105 + 3) end
c378 = function(x) return 378 + 20 * math.sin(x / 85) end

kz = above(c150) * below(c268) * mask(function(x) return x < 600 and 1 or 0.4 end)
work(kz, {hand="scumble", pile=coolknit, coverage=1.2, angle=0.05, edge=1})

uz = above(c292) * below(c378) * mask(function(x) return x < 520 and 1 or 0.3 end)
work(uz, {hand="scumble", pile=undersh, coverage=1.1, angle=0.05, edge=1})

blend(kz, {coverage=0.6})
blend(uz, {coverage=0.6})
print("bank shaped")

--@ chunk 25
wait(14 * 60)
print(drying(500, 300))

--@ chunk 26
-- Headland: low treeline right, thin far shore left
treedark = pile{{"lead white", 0.5}, {"cobalt blue", 0.6}, {"green earth", 0.35}, {"raw umber", 0.6}, {"bone black", 0.5}}

tl = function(x)
  return 448 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2)
end
hz = function(x) return 512 + 4 * math.sin(x / 40) end

headR = below(tl) * above(hz) * mask(function(x) return x > 370 and 1 or 0 end)
shoreL = below(function(x) return 498 + 8 * math.sin(x / 30) end) * above(hz) * mask(function(x) return x <= 420 and 1 or 0 end)

work(headR, {hand="body", pile=treedark, coverage=2.6, fill=true, angle=0.02, pressure={0.6, 0.85}, edge=0.5})
work(shoreL, {hand="body", pile=treedark, coverage=2.0, fill=true, angle=0.02, pressure={0.5, 0.7}, edge=0.6})
print("headland in")

--@ chunk 27
-- Deepen treeline, add taller irregular clumps
treedark2 = pile{{"lead white", 0.4}, {"cobalt blue", 0.55}, {"green earth", 0.3}, {"raw umber", 0.6}, {"bone black", 0.65}}

tl = function(x)
  return 448 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2)
end
hz = function(x) return 512 + 4 * math.sin(x / 40) end

clumps = ellipse(497, 424, 30, 36) + ellipse(638, 418, 46, 44) + ellipse(798, 426, 36, 28) + ellipse(938, 420, 33, 38)
band = below(tl) * above(hz) * mask(function(x) return x > 370 and 1 or 0 end)
work(band + clumps, {hand="body", pile=treedark2, coverage=2.0, fill=true, angle=0.02, pressure={0.6, 0.85}, edge=0.45})

shoreL = below(function(x) return 500 + 6 * math.sin(x / 30) end) * above(hz) * mask(function(x) return x <= 420 and 1 or 0 end)
work(shoreL, {hand="body", pile=treedark2, coverage=1.6, fill=true, angle=0.02, pressure={0.5, 0.7}, edge=0.6})
print("treeline deepened")

--@ chunk 28
-- Water: three fields over the old cream, blended
treedark2 = pile{{"lead white", 0.4}, {"cobalt blue", 0.55}, {"green earth", 0.3}, {"raw umber", 0.6}, {"bone black", 0.65}}
glowwater = pile{{"lead white", 1.2}, {"chrome yellow", 0.4}, {"vermilion", 0.2}, {"cobalt blue", 0.3}, {"raw umber", 0.3}}
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
waterdeep = pile{{"lead white", 0.7}, {"cobalt blue", 0.9}, {"raw umber", 0.6}, {"bone black", 0.55}, {"green earth", 0.35}, {"vermilion", 0.08}}

-- fill treeline holes
holes = ellipse(662, 452, 26, 20) + ellipse(938, 448, 22, 18) + ellipse(940, 462, 18, 14)
work(holes, {hand="detail", pile=treedark2, coverage=2.2, fill=true, edge=0.4, clip=true})

-- water fields
hz = function(x) return 512 + 4 * math.sin(x / 40) end
wg = below(function(x) return 610 + 15 * math.sin(x / 70) end) * above(hz)
work(wg, {hand="broad", pile=glowwater, coverage=2.6, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

wm = below(function(x) return 980 + 25 * math.sin(x / 80) end) * above(hz)
work(wm, {hand="broad", pile=watermid, coverage=2.6, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

wd = below(function(x) return 1260 end)
work(wd, {hand="broad", pile=waterdeep, coverage=2.6, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

print("water in")

--@ chunk 29
-- Water fields (correct masks: water lies below the horizon curve)
glowwater = pile{{"lead white", 1.2}, {"chrome yellow", 0.4}, {"vermilion", 0.2}, {"cobalt blue", 0.3}, {"raw umber", 0.3}}
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
waterdeep = pile{{"lead white", 0.7}, {"cobalt blue", 0.9}, {"raw umber", 0.6}, {"bone black", 0.55}, {"green earth", 0.35}, {"vermilion", 0.08}}

hz = function(x) return 512 + 4 * math.sin(x / 40) end
wg = below(hz) * above(function(x) return 615 + 15 * math.sin(x / 70) end)
work(wg, {hand="broad", pile=glowwater, coverage=2.6, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

wm = below(hz) * above(function(x) return 985 + 25 * math.sin(x / 80) end)
work(wm, {hand="broad", pile=watermid, coverage=2.6, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

wd = below(function(x) return 985 + 25 * math.sin(x / 80) end)
work(wd, {hand="broad", pile=waterdeep, coverage=2.6, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

print("water fields on")

--@ chunk 30
-- Blend the water, close the rims, deepen the bottom
waterzone = below(function(x) return 512 + 4 * math.sin(x / 40) end)
blend(waterzone, {coverage=1.0})

waterdeep = pile{{"lead white", 0.7}, {"cobalt blue", 0.9}, {"raw umber", 0.6}, {"bone black", 0.55}, {"green earth", 0.35}, {"vermilion", 0.08}}
rims = rect(0, 514, 14, 740) + rect(986, 514, 14, 740) + rect(0, 1226, 1000, 26)
work(rims, {hand="body", pile=waterdeep, coverage=2.4, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})
print("water blended, rims closed")

--@ chunk 31
-- Light path on the water, wet into wet
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}

core = ribbon({{640, 516}, {700, 640}, {660, 830}, {600, 1000}, {560, 1160}, {540, 1250}}, {26, 44, 70, 100, 130, 145})
work(core, {hand="body", pile=bright, coverage=2.0, fill=true, angle=function(x, y) return 0.15 * math.sin(y / 60) end, pressure={0.6, 0.85}, edge=1})

wide = ribbon({{640, 516}, {710, 650}, {670, 850}, {610, 1030}, {575, 1180}, {560, 1250}}, {60, 110, 190, 260, 320, 350})
work(wide, {hand="broad", pile=flare, coverage=0.9, fill=false, angle=0.02, pressure={0.5, 0.7}, edge=1})

print("light path laid")

--@ chunk 32
-- Soften and break the light path
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}

wideR = ribbon({{640, 516}, {710, 650}, {670, 850}, {610, 1030}, {575, 1180}, {560, 1250}}, {70, 120, 200, 270, 330, 360})
coreR = ribbon({{640, 516}, {700, 640}, {660, 830}, {600, 1000}, {560, 1160}, {540, 1250}}, {26, 44, 70, 100, 130, 145})

blend(wideR, {coverage=1.0})
ring = wideR - coreR
work(ring, {hand="scumble", pile=watermid, coverage=0.9, angle=0.0, edge=1})
print("path softened and broken")

--@ chunk 33
hullpile = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
hull = body_of{spine = {{352, 912}, {398, 916}, {442, 910}, {474, 902}}, widths = 26, char="firm"}
work(hull:mask(), {hand="body", pile=hullpile, coverage=2.2, fill=true, angle=0.02, pressure={0.6, 0.8}, edge=0.5})
print("hull ok")

--@ chunk 34
-- Yacht: hull from a ribbon, cabin, furled sail, rigger mast
hullpile = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
warmside = pile{{"lead white", 1.2}, {"vermilion", 0.35}, {"chrome yellow", 0.2}, {"raw umber", 0.15}}

hull = ribbon({{350, 912}, {398, 917}, {442, 911}, {476, 901}}, {22, 30, 24, 7})
work(hull, {hand="body", pile=hullpile, coverage=2.4, fill=true, angle=0.02, pressure={0.6, 0.8}, edge=0.4})

cabin = ribbon({{400, 894}, {446, 891}}, 15)
work(cabin, {hand="body", pile=hullpile, coverage=1.8, fill=true, angle=0.02, edge=0.5})

sail = ribbon({{424, 886}, {427, 800}, {428, 745}}, 7)
work(sail, {hand="body", pile=hullpile, coverage=1.6, fill=true, angle=0.02, edge=0.5})

mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hullpile, 0.95)
mast = outline{{420, 904}, {424, 830}, {427, 745}}
mast:paint(mb, {pressure=0.85, every=2})

hs = hull:rim(5, 2) * mask(function(x, y) return x > 424 and 1 or 0 end)
work(hs, {hand="scumble", pile=warmside, coverage=0.8, angle=0.02, edge=1})

print("yacht in")

--@ chunk 35
-- Fix the mast area: cover the fat column, redraw thin mast, faint warm rim
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
hullpile = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}

col = rect(408, 738, 36, 152)
work(col, {hand="body", pile=watermid, coverage=2.2, fill=true, angle=0.02, pressure={0.6, 0.8}, edge=1})

mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hullpile, 0.95)
mast = outline{{421, 892}, {424, 830}, {427, 748}}
mast:paint(mb, {pressure=0.5, every=2})

nub = ellipse(425, 812, 5, 14)
work(nub, {hand="detail", pile=hullpile, coverage=1.6, fill=true, edge=0.3, clip=true})

rim = hull:rim(4, 1.5) * mask(function(x, y) return x > 428 and 1 or 0 end)
stipple(rim, {pile=warmpale2, width=3, coverage=0.5, pressure={0.2, 0.4}, feather=1})

print("mast fixed")

--@ chunk 36
-- Clean the bow, firm the mast, whisper the warm rim
hullpile = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}

hull = ribbon({{350, 912}, {398, 917}, {442, 911}, {476, 901}}, {22, 30, 24, 7})
bowm = hull * mask(function(x, y) return x > 424 and 1 or 0 end)
work(bowm, {hand="body", pile=hullpile, coverage=2.2, fill=true, angle=0.02, pressure={0.65, 0.85}, edge=0.3, clip=true})

mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hullpile, 0.95)
mast = outline{{421, 890}, {425, 820}, {427, 748}}
mast:paint(mb, {pressure=0.55, every=2})

flecks = hull:rim(3, 1) * mask(function(x, y) return x > 432 and 1 or 0 end)
stipple(flecks, {pile=warmpale2, width=2.5, coverage=0.15, pressure={0.15, 0.3}, feather=1})

-- quiet the mottled halo beside the mast
halo = rect(404, 760, 46, 130)
blend(halo, {coverage=0.8})

print("bow cleaned")

--@ chunk 37
hullpile = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
probe = rect(430, 880, 40, 40)
work(probe, {hand="body", pile=hullpile, coverage=2.2, fill=true, angle=0.02, pressure={0.65, 0.85}, edge=0.3, clip=true})
print("probe painted")

--@ chunk 38
wait(3 * 60)
print(drying(455, 905))

--@ chunk 39
wait(3 * 60)
print(drying(455, 905))

--@ chunk 40
wait(24 * 60)
print(drying(455, 905))
print(drying(500, 700))

--@ chunk 41
-- Bow over tacky paint: solid dark, two passes
hullpile = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
hullpile2 = pile{{"bone black", 0.7}, {"raw umber", 0.8}, {"cobalt blue", 0.3}, {"lead white", 0.3}}

hull = ribbon({{350, 912}, {398, 917}, {442, 911}, {476, 901}}, {22, 30, 24, 7})
bowm = hull * mask(function(x, y) return x > 420 and 1 or 0 end)
work(bowm, {hand="body", pile=hullpile, coverage=2.6, fill=true, angle=0.02, pressure={0.75, 0.95}, edge=0.25, clip=true})
work(bowm, {hand="detail", pile=hullpile2, coverage=1.6, fill=true, angle=0.02, edge=0.2, clip=true})
print("bow covered")

--@ chunk 42
-- Stray pink flecks + foreground spit
hullpile2 = pile{{"bone black", 0.7}, {"raw umber", 0.8}, {"cobalt blue", 0.3}, {"lead white", 0.3}}
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.45}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}

strays = ellipse(474, 887, 9, 7) + ellipse(418, 922, 12, 8) + ellipse(408, 916, 8, 6) + ellipse(441, 926, 6, 4)
work(strays, {hand="detail", pile=hullpile2, coverage=2.0, fill=true, edge=0.3, clip=true})

spit = below(function(x) return 1130 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end)
work(spit, {hand="broad", pile=spitdark, coverage=2.6, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=0.7})
print("spit in")

--@ chunk 43
-- Grasses and reeds on the bank edge
grassdark = pile{{"bone black", 0.5}, {"raw umber", 0.65}, {"cobalt blue", 0.2}, {"green earth", 0.25}, {"lead white", 0.2}}
g = brush{kind="rigger", width=2.2, point=1, stiffness=0.45}
g:load(grassdark, 0.9)

spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
xs = uneven(26, 30, 990, 0.8, 0.35, 7)
for i, x in ipairs(xs) do
  local hgt = rand(16, 48)
  if x > 640 and x < 900 then hgt = rand(34, 62) end
  local lean = rand(-14, 26)
  local base = spit_edge(x) + 4
  g:stroke({{x, base}, {x + lean * 0.4, base - hgt * 0.6}, {x + lean, base - hgt}},
    {pressure={0.65, 0.03}, ramps={0.12, 0.45}})
end
-- three taller reeds near the light
for _, x in ipairs({688, 742, 812}) do
  local base = spit_edge(x) + 4
  local hgt = rand(70, 92)
  local lean = rand(-6, 14)
  g:stroke({{x, base}, {x + lean * 0.5, base - hgt * 0.55}, {x + lean, base - hgt}},
    {pressure={0.6, 0.02}, ramps={0.1, 0.5}})
end
print("grasses in:", #xs + 3)

--@ chunk 44
grassdark = pile{{"bone black", 0.5}, {"raw umber", 0.65}, {"cobalt blue", 0.2}, {"green earth", 0.25}, {"lead white", 0.2}}
g = brush{kind="rigger", width=2.2, point=1, stiffness=0.45}
print("fullness after load:", g:fullness())
g:load(grassdark, 0.9)
print("fullness after load:", g:fullness())
g:stroke({{500, 1100}, {505, 1080}, {508, 1060}}, {pressure={0.65, 0.03}, ramps={0.12, 0.45}})
print("mark width:", g:mark_width(0.65))

--@ chunk 45
-- Grass clumps along the bank edge
grassdark = pile{{"bone black", 0.35}, {"raw umber", 0.55}, {"green earth", 0.45}, {"cobalt blue", 0.15}, {"lead white", 0.15}}
g = brush{kind="round", width=5, point=1, stiffness=0.5}

spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
xs = uneven(9, 40, 980, 0.7, 0.4, 11)
for _, x in ipairs(xs) do
  if g:fullness() < 0.3 then g:wipe(0.5); g:load(grassdark, 0.9) end
  local nb = math.random(2, 3)
  for k = 1, nb do
    local bx = x + rand(-7, 7)
    local hgt = rand(18, 46)
    if bx > 620 and bx < 900 then hgt = rand(30, 60) end
    local lean = rand(-16, 30)
    local base = spit_edge(bx) + 5
    g:stroke({{bx, base}, {bx + lean * 0.35, base - hgt * 0.55}, {bx + lean, base - hgt}},
      {pressure={0.8, 0.06}, ramps={0.1, 0.5}})
  end
end
print("grass clumps in")

--@ chunk 46
-- Thicker grass blades and reeds
grassdark = pile{{"bone black", 0.35}, {"raw umber", 0.55}, {"green earth", 0.45}, {"cobalt blue", 0.15}, {"lead white", 0.15}}
g = brush{kind="round", width=11, point=1, stiffness=0.5}

spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
xs = uneven(8, 50, 970, 0.7, 0.4, 13)
for _, x in ipairs(xs) do
  if g:fullness() < 0.3 then g:wipe(0.5); g:load(grassdark, 0.95) end
  local nb = math.random(2, 4)
  for k = 1, nb do
    local bx = x + rand(-8, 8)
    local hgt = rand(20, 48)
    if bx > 600 and bx < 900 then hgt = rand(36, 68) end
    local lean = rand(-18, 34)
    local base = spit_edge(bx) + 6
    g:stroke({{bx, base}, {bx + lean * 0.35, base - hgt * 0.55}, {bx + lean, base - hgt}},
      {pressure={0.85, 0.05}, ramps={0.08, 0.55}})
  end
end
-- reeds
for _, x in ipairs({672, 738, 806, 590}) do
  local base = spit_edge(x) + 6
  local hgt = rand(72, 96)
  local lean = rand(-8, 16)
  g:stroke({{x, base}, {x + lean * 0.5, base - hgt * 0.55}, {x + lean, base - hgt}},
    {pressure={0.85, 0.04}, ramps={0.08, 0.6}})
end
print("blades and reeds in")

--@ chunk 47
-- Firm the spit edge, knock back floaters, finish boat strays
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.45}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
hullpile2 = pile{{"bone black", 0.7}, {"raw umber", 0.8}, {"cobalt blue", 0.3}, {"lead white", 0.3}}

spit = below(function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end)
work(spit, {hand="body", pile=spitdark, coverage=1.8, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=0.15})

floaters = ellipse(172, 950, 22, 8) + ellipse(30, 1100, 30, 12) + ellipse(20, 980, 16, 30)
work(floaters, {hand="body", pile=watermid, coverage=1.6, fill=true, angle=0.02, edge=1})

strays2 = ellipse(468, 886, 14, 9) + ellipse(478, 896, 10, 7) + ellipse(452, 891, 9, 6)
work(strays2, {hand="detail", pile=hullpile2, coverage=2.2, fill=true, edge=0.25, clip=true})
print("cleanup done")

--@ chunk 48
-- Reed-bed tongue, mooring post, boat reflection, bank surface
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.45}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}
hullpile  = pile{{"bone black", 0.6}, {"raw umber", 0.7}, {"cobalt blue", 0.35}, {"lead white", 0.35}, {"vermilion", 0.06}}
postdark  = pile{{"bone black", 0.55}, {"raw umber", 0.75}, {"cobalt blue", 0.25}, {"lead white", 0.2}}

-- tongue of reeds reaching into the water around x 500-640
tongue = poly({{500, 1128}, {560, 1102}, {620, 1088}, {665, 1096}, {640, 1130}, {560, 1160}, {500, 1170}}, true)
work(tongue, {hand="body", pile=spitdark, coverage=2.4, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=0.4})

-- mooring post at the bank edge, raking slightly
post = ribbon({{596, 1010}, {590, 952}}, 6)
work(post, {hand="body", pile=postdark, coverage=2.2, fill=true, angle=0.02, edge=0.3, clip=true})

-- boat reflection: soft broken smear below the hull
refl = ribbon({{398, 948}, {404, 992}, {408, 1032}}, {22, 30, 40})
work(refl, {hand="scumble", pile=hullpile, coverage=0.8, angle=0.02, edge=1})

-- bank surface: subtle variation
spitwarm = pile{{"lead white", 1}, {"yellow ochre", 0.3}, {"raw umber", 0.5}, {"green earth", 0.2}}
patches = ellipse(180, 1210, 90, 30) + ellipse(420, 1180, 70, 24) + ellipse(700, 1140, 80, 26) + ellipse(880, 1090, 70, 24)
work(patches, {hand="scumble", pile=spitwarm, coverage=0.5, angle=0.05, edge=1})
print("tongue, post, reflection in")

--@ chunk 49
-- Cover cream blobs, calm the reflection, crisp the post, birds
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.45}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
postdark  = pile{{"bone black", 0.55}, {"raw umber", 0.75}, {"cobalt blue", 0.25}, {"lead white", 0.2}}

patches = ellipse(180, 1210, 100, 34) + ellipse(420, 1180, 80, 28) + ellipse(700, 1140, 90, 30) + ellipse(880, 1090, 80, 28)
work(patches, {hand="body", pile=spitdark, coverage=2.6, fill=true, angle=0.05, pressure={0.7, 0.9}, edge=1})

-- calm the boat's reflection: blend, then break with water
halo = rect(370, 930, 80, 120)
blend(halo, {coverage=0.9})
refl2 = ribbon({{398, 946}, {404, 985}, {408, 1020}}, {16, 22, 28})
work(refl2, {hand="scumble", pile=watermid, coverage=0.5, angle=0.02, edge=1})

-- crisper post
pb = brush{kind="round", width=8, point=1, stiffness=0.55}
pb:load(postdark, 0.95)
pb:stroke({{594, 1008}, {591, 978}, {589, 950}}, {pressure={0.9, 0.35}, ramps={0.08, 0.4}})

-- birds in the glow gap right of the trees
bird = brush{kind="round", width=6, point=1, stiffness=0.5}
birdpile = pile{{"bone black", 0.6}, {"raw umber", 0.5}, {"cobalt blue", 0.3}, {"lead white", 0.2}}
bird:load(birdpile, 0.95)
birds = {{788, 352}, {826, 366}, {862, 344}, {842, 384}}
for _, p in ipairs(birds) do
  local s = rand(0.8, 1.3)
  bird:stroke({{p[1] - 7 * s, p[2] - 3 * s}, {p[1], p[2]}, {p[1] + 7 * s, p[2] - 4 * s}},
    {pressure={0.55, 0.06}, ramps={0.15, 0.4}})
end
print("tidied, birds in")

--@ chunk 50
-- Decisive cover of pale patches; water over the reflection smear
spitvery = pile{{"raw umber", 1}, {"bone black", 0.5}, {"green earth", 0.3}, {"cobalt blue", 0.25}, {"lead white", 0.3}}
watermid  = pile{{"lead white", 0.9}, {"cobalt blue", 1.0}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.3}, {"vermilion", 0.1}}
postdark  = pile{{"bone black", 0.55}, {"raw umber", 0.75}, {"cobalt blue", 0.25}, {"lead white", 0.2}}

patches = ellipse(180, 1210, 96, 30) + ellipse(420, 1180, 76, 25) + ellipse(700, 1140, 86, 26) + ellipse(880, 1090, 76, 25)
work(patches, {hand="broad", pile=spitvery, coverage=2.2, fill=true, angle=0.05, pressure={0.8, 0.95}, edge=1})
work(patches, {hand="scumble", pile=spitvery, coverage=1.6, angle=0.05, edge=1})

rpale = rect(380, 940, 60, 100)
work(rpale, {hand="body", pile=watermid, coverage=2.0, fill=true, angle=0.02, pressure={0.7, 0.85}, edge=1})

-- one quiet dark hint of reflection
rb = brush{kind="round", width=6, point=1, stiffness=0.5}
rb:load(postdark, 0.9)
rb:stroke({{399, 948}, {403, 982}, {406, 1012}}, {pressure={0.5, 0.1}, ramps={0.15, 0.5}})
print("patches settled")

--@ chunk 51
-- Reflection area unified with the darker water; post firm
waterdeep = pile{{"lead white", 0.7}, {"cobalt blue", 0.9}, {"raw umber", 0.6}, {"bone black", 0.55}, {"green earth", 0.35}, {"vermilion", 0.08}}
postdark  = pile{{"bone black", 0.55}, {"raw umber", 0.75}, {"cobalt blue", 0.25}, {"lead white", 0.2}}

work(rect(378, 936, 64, 108), {hand="body", pile=waterdeep, coverage=2.2, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1})

pb = brush{kind="round", width=14, point=1, stiffness=0.55}
pb:load(postdark, 0.95)
pb:stroke({{594, 1012}, {591, 980}, {589, 948}}, {pressure={0.9, 0.4}, ramps={0.08, 0.4}})
print("reflection and post")

--@ chunk 52
-- Veils to knit the boat zone into the water
veil = pile{{"bone black", 0.25}, {"cobalt blue", 0.4}, {"raw umber", 0.4}, {"green earth", 0.2}, medium=0.65}

veilrect = rect(372, 930, 76, 118)
work(veilrect, {hand="body", pile=veil, coverage=0.9, angle=0.02, edge=1})

veilmast = rect(404, 742, 42, 170)
work(veilmast, {hand="body", pile=veil, coverage=0.8, angle=0.02, edge=1})

veilstreak = rect(280, 888, 90, 46)
work(veilstreak, {hand="body", pile=veil, coverage=0.7, angle=0.02, edge=1})

blend(veilrect, {coverage=0.7})
blend(veilmast, {coverage=0.7})
print("veils laid")

--@ chunk 53
print("sky:", drying(500, 200))
print("treeline:", drying(500, 450))
print("far water:", drying(800, 640))
print("mid water:", drying(800, 750))
print("bank:", drying(600, 1150))

--@ chunk 54
-- Finishing lights (on tacky/dry ground: they grab and stay broken)
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}
flarep = pile{{"lead white", 2.6}, {"chrome yellow", 0.6}, {"vermilion", 0.25}}

pathzone = ribbon({{640, 516}, {700, 640}, {660, 830}, {610, 1000}, {575, 1160}, {560, 1250}}, {50, 90, 150, 210, 260, 290})
stipple(pathzone, {pile=bright, width=3.5, coverage=0.3, pressure={0.25, 0.55}, drag={5, 0}, feather=1})

tltop = function(x) return 452 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2) end
tlrim = below(tltop) * above(function(x) return tltop(x) - 20 end) * mask(function(x) return x > 540 and 1 or 0 end)
stipple(tlrim, {pile=warmpale2, width=2.5, coverage=0.22, pressure={0.15, 0.4}, feather=1})

work(rect(570, 430, 260, 80), {hand="scumble", pile=flarep, coverage=0.55, angle=0.02, edge=1})

tips = ribbon({{672, 950}, {738, 962}, {806, 955}}, 26)
stipple(tips, {pile=warmpale2, width=2.5, coverage=0.12, pressure={0.15, 0.35}, feather=1})
print("lights in")

--@ chunk 55
-- Erase the flare letters: warm glow over sky part, tree dark over tree part
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}
treedark2 = pile{{"lead white", 0.4}, {"cobalt blue", 0.55}, {"green earth", 0.3}, {"raw umber", 0.6}, {"bone black", 0.65}}

skyfix = rect(565, 425, 270, 24)
work(skyfix, {hand="body", pile=gold3, coverage=2.2, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1})
blend(skyfix, {coverage=0.8})

treefix = rect(560, 443, 280, 74)
tltop = function(x) return 452 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2) end
work(below(tltop) * treefix, {hand="body", pile=treedark2, coverage=2.4, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=0.3, clip=true})
print("flare fixed")

--@ chunk 56
-- Heavy dark over the smothered treeline stretch
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
tltop = function(x) return 452 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2) end
hz = function(x) return 512 + 4 * math.sin(x / 40) end
zone = below(tltop) * above(hz) * rect(555, 400, 300, 130)
work(zone, {hand="body", pile=treedark3, coverage=3.0, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.25, clip=true})
work(zone, {hand="scumble", pile=treedark3, coverage=2.0, angle=0.02, edge=0.3})
print("treeline restored")

--@ chunk 57
-- Break the smooth domes into foliage; tie the stretch to the rest
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}

-- new clumps to vary the silhouette
clumps2 = ellipse(585, 470, 34, 26) + ellipse(660, 458, 30, 30) + ellipse(742, 472, 36, 24) + ellipse(828, 462, 30, 28)
work(clumps2, {hand="body", pile=treedark3, coverage=2.6, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.35, clip=true})

-- flecks to roughen the top edges
tltop = function(x) return 452 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2) end
edgeband = below(function(x) return tltop(x) + 16 end) * above(function(x) return tltop(x) - 12 end) * mask(function(x) return x > 555 and x < 870 and 1 or 0 end)
stipple(edgeband, {pile=treedark3, width=4, coverage=0.9, pressure={0.3, 0.6}, feather=1})

-- unify tone across the whole right treeline
unif = below(tltop) * above(function(x) return 512 + 4 * math.sin(x / 40) end) * mask(function(x) return x > 540 and 1 or 0 end)
work(unif, {hand="scumble", pile=treedark3, coverage=0.8, angle=0.02, edge=0.5})
print("treeline re-carved")

--@ chunk 58
-- Lose the hard cream top edge into the sky; break the dark edge with glow flecks
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
tltop = function(x) return 452 + 24 * math.sin(x / 23 + 1) + 16 * math.sin(x / 61) + 9 * math.sin(x / 13 + 2) end

-- soften the straight cream top edge upward into the glow
aboveband = below(function(x) return 425 end) * above(function(x) return 395 end) * mask(function(x) return x > 560 and x < 865 and 1 or 0 end)
work(aboveband, {hand="scumble", pile=gold3, coverage=0.7, angle=0.02, edge=1})

-- glow flecks eating into the dark top edge
gapband = below(function(x) return tltop(x) + 18 end) * above(function(x) return tltop(x) - 8 end) * mask(function(x) return x > 555 and x < 875 and 1 or 0 end)
stipple(gapband, {pile=gold3, width=3, coverage=0.5, pressure={0.2, 0.5}, feather=1})

-- more dark flecks above, to ruffle the silhouette
ruff = below(function(x) return tltop(x) - 2 end) * above(function(x) return tltop(x) - 22 end) * mask(function(x) return x > 555 and x < 875 and 1 or 0 end)
stipple(ruff, {pile=treedark3, width=4, coverage=0.8, pressure={0.25, 0.55}, feather=1})
print("edges broken")

--@ chunk 59
-- Rebuild the right treeline decisively
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}
hz = function(x) return 512 + 4 * math.sin(x / 40) end
stretch = rect(548, 388, 350, 140) * above(hz)

base = below(function(x) return 470 end) * stretch
work(base, {hand="body", pile=treedark3, coverage=3.2, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.15, clip=true})
work(base, {hand="scumble", pile=treedark3, coverage=2.0, angle=0.02, edge=0.2})

clumps3 = ellipse(596, 444, 42, 30) + ellipse(668, 436, 48, 42) + ellipse(748, 444, 40, 26) + ellipse(836, 438, 46, 38) + ellipse(886, 446, 28, 24)
work(clumps3 * stretch, {hand="body", pile=treedark3, coverage=3.0, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.2, clip=true})
work(clumps3 * stretch, {hand="scumble", pile=treedark3, coverage=1.8, angle=0.02, edge=0.25})

-- sparse glow flecks along the new silhouette
tltop = function(x) return 452 + 20 * math.sin(x / 31 + 2) + 14 * math.sin(x / 67) end
gapband = below(function(x) return tltop(x) + 14 end) * above(function(x) return tltop(x) - 6 end) * stretch
stipple(gapband, {pile=gold3, width=3, coverage=0.35, pressure={0.2, 0.45}, feather=1})
print("right treeline rebuilt")

--@ chunk 60
-- Give the right treeline its texture back
treedark2 = pile{{"lead white", 0.4}, {"cobalt blue", 0.55}, {"green earth", 0.3}, {"raw umber", 0.6}, {"bone black", 0.65}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}
hz = function(x) return 512 + 4 * math.sin(x / 40) end
stretch = rect(548, 388, 350, 140) * above(hz)

work(stretch, {hand="scumble", pile=treedark2, coverage=1.3, angle=0.02, edge=0.4})
speck = below(function(x) return 500 end) * stretch
stipple(speck, {pile=treedark3, width=4, coverage=0.7, pressure={0.25, 0.5}, feather=1})

tltop = function(x) return 452 + 20 * math.sin(x / 31 + 2) + 14 * math.sin(x / 67) end
topruff = below(function(x) return tltop(x) + 10 end) * above(function(x) return tltop(x) - 10 end) * stretch
stipple(topruff, {pile=treedark3, width=4, coverage=1.0, pressure={0.3, 0.6}, feather=1})

canopy = below(function(x) return tltop(x) + 26 end) * above(function(x) return tltop(x) - 2 end) * stretch
stipple(canopy, {pile=gold3, width=3, coverage=0.12, pressure={0.15, 0.4}, feather=1})
print("texture back")

--@ chunk 61
wait(2 * 24 * 60)
print("sky:", drying(500, 200))
print("treeline:", drying(500, 450))
print("far water:", drying(800, 640))
print("mid water:", drying(800, 750))
print("bank:", drying(600, 1150))
print("boat:", drying(420, 910))

--@ chunk 62
-- Sky glow re-carves the slab; spires remain
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}

hz = function(x) return 512 + 4 * math.sin(x / 40) end
glowtop = function(x) return 452 + 8 * math.sin(x / 57) end
glowzone = below(glowtop) * above(function(x) return 400 end) * mask(function(x) return x > 548 and 1 or 0 end)
work(glowzone, {hand="body", pile=gold3, coverage=2.6, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.15, clip=true})
blend(glowzone, {coverage=0.8})

spires = ellipse(628, 442, 13, 24) + ellipse(776, 438, 11, 28) + ellipse(862, 446, 13, 22) + ellipse(700, 448, 10, 16)
work(spires, {hand="detail", pile=treedark3, coverage=2.2, fill=true, angle=0.02, edge=0.2, clip=true})

ruff = below(function(x) return glowtop(x) + 6 end) * above(function(x) return glowtop(x) - 8 end) * mask(function(x) return x > 548 and 1 or 0 end)
stipple(ruff, {pile=treedark3, width=3.5, coverage=0.7, pressure={0.25, 0.5}, feather=1})
print("glow recarved")

--@ chunk 63
-- Glow ABOVE the treeline top (y < 452), i.e. above(glowtop) * below(400-line)
gold3  = pile{{"lead white", 3}, {"chrome yellow", 0.6}, {"vermilion", 0.2}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}

glowtop = function(x) return 452 + 8 * math.sin(x / 57) end
glowzone = above(glowtop) * below(function(x) return 402 end) * mask(function(x) return x > 548 and 1 or 0 end)
work(glowzone, {hand="body", pile=gold3, coverage=2.6, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.15, clip=true})
blend(glowzone, {coverage=0.8})

ruff = above(function(x) return glowtop(x) + 8 end) * below(function(x) return glowtop(x) - 6 end) * mask(function(x) return x > 548 and 1 or 0 end)
stipple(ruff, {pile=treedark3, width=3.5, coverage=0.7, pressure={0.25, 0.5}, feather=1})
print("glow recarved for real")

--@ chunk 64
-- Clean sky over the upper zone; then re-fleck the tree edge
skyfixp = pile{{"lead white", 2.6}, {"chrome yellow", 0.5}, {"vermilion", 0.22}, {"raw umber", 0.1}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}

fillz = rect(548, 355, 355, 92)
work(fillz, {hand="body", pile=skyfixp, coverage=3.0, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.2, clip=true})
work(fillz, {hand="scumble", pile=skyfixp, coverage=1.6, angle=0.02, edge=0.25})
blend(fillz, {coverage=0.9})

-- re-fleck the treeline top against the clean sky
glowtop = function(x) return 452 + 10 * math.sin(x / 47 + 1) + 6 * math.sin(x / 23) end
ruff = above(function(x) return glowtop(x) + 10 end) * below(function(x) return glowtop(x) - 8 end) * mask(function(x) return x > 548 and 1 or 0 end)
stipple(ruff, {pile=treedark3, width=4, coverage=0.8, pressure={0.25, 0.55}, feather=1})
print("sky clean")

--@ chunk 65
-- Soft edges for the sky patch; marry the tone
skyfixp2 = pile{{"lead white", 2.2}, {"chrome yellow", 0.42}, {"vermilion", 0.2}, {"raw umber", 0.25}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}

soft = rect(548, 350, 355, 100):soften(14)
work(soft, {hand="body", pile=skyfixp2, coverage=2.0, fill=true, angle=0.02, pressure={0.7, 0.85}, edge=1})
work(soft, {hand="scumble", pile=skyfixp2, coverage=0.9, angle=0.02, edge=1})

glowtop = function(x) return 452 + 10 * math.sin(x / 47 + 1) + 6 * math.sin(x / 23) end
ruff = above(function(x) return glowtop(x) + 10 end) * below(function(x) return glowtop(x) - 8 end) * mask(function(x) return x > 548 and 1 or 0 end)
stipple(ruff, {pile=treedark3, width=4, coverage=0.85, pressure={0.25, 0.55}, feather=1})
print("sky married")

--@ chunk 66
-- Wisps in the glow; bridge the left seam
wisp = pile{{"lead white", 1.6}, {"vermilion", 0.25}, {"chrome yellow", 0.2}, {"raw umber", 0.2}, medium=0.35}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}

wispz = rect(570, 366, 320, 70)
work(wispz, {hand="scumble", pile=wisp, coverage=0.5, angle=0.02, edge=1})

bridge = ellipse(560, 428, 30, 42) + ellipse(545, 470, 26, 30)
work(bridge, {hand="body", pile=treedark3, coverage=2.4, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.25, clip=true})

notch = ellipse(516, 410, 22, 18)
work(notch, {hand="detail", pile=treedark3, coverage=2.0, fill=true, edge=0.25, clip=true})
print("wisps and bridge")
print("boat dryness:", drying(420, 910), drying(600, 1150))

--@ chunk 67
-- Rims and streak calming (dry zones)
waterdeep = pile{{"lead white", 0.7}, {"cobalt blue", 0.9}, {"raw umber", 0.6}, {"bone black", 0.55}, {"green earth", 0.35}, {"vermilion", 0.08}}

rims = rect(0, 516, 12, 620) + rect(988, 516, 12, 620) + rect(0, 1120, 30, 132) + rect(970, 1120, 30, 132)
work(rims, {hand="body", pile=waterdeep, coverage=2.2, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})

streakz = rect(0, 850, 260, 100)
work(streakz, {hand="scumble", pile=waterdeep, coverage=0.7, angle=0.02, edge=1})
print("rims and streak")

--@ chunk 68
-- Cover the dark speckle swarm with matched water
watermid2 = pile{{"lead white", 1.05}, {"cobalt blue", 0.85}, {"raw umber", 0.35}, {"bone black", 0.28}, {"green earth", 0.25}, {"vermilion", 0.08}}
z = rect(0, 828, 270, 140)
work(z, {hand="body", pile=watermid2, coverage=2.4, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1, clip=true})
work(z, {hand="scumble", pile=watermid2, coverage=1.2, angle=0.02, edge=1})
blend(z, {coverage=0.8})
print("swarm covered")
print("boat:", drying(420, 910), "bank:", drying(600, 1150))

--@ chunk 69
-- Darkening glaze over the pale patch, soft edges
veil2 = pile{{"cobalt blue", 0.7}, {"bone black", 0.25}, {"raw umber", 0.45}, {"green earth", 0.2}, medium=0.6}
soft = rect(4, 838, 258, 118):soften(22)
work(soft, {hand="body", pile=veil2, coverage=0.9, angle=0.02, edge=1, clip=true})
work(soft, {hand="scumble", pile=veil2, coverage=0.5, angle=0.02, edge=1})
blend(soft, {coverage=0.7})
print("glazed")

--@ chunk 70
-- Matched water over the pale patch; clean slate behind the boat
watermid3 = pile{{"lead white", 1.0}, {"cobalt blue", 0.95}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.28}, {"vermilion", 0.08}}

soft = rect(4, 838, 258, 118):soften(22)
work(soft, {hand="body", pile=watermid3, coverage=2.8, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1, clip=true})
blend(soft, {coverage=0.8})

slate = rect(300, 718, 225, 290)
work(slate, {hand="body", pile=watermid3, coverage=2.8, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1, clip=true})
work(slate, {hand="scumble", pile=watermid3, coverage=1.2, angle=0.02, edge=1})
blend(slate, {coverage=0.8})
print("clean slate")

--@ chunk 71
-- Marry patch edges into horizontal streaks
watermid3 = pile{{"lead white", 1.0}, {"cobalt blue", 0.95}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.28}, {"vermilion", 0.08}}
darker = pile{{"lead white", 0.7}, {"cobalt blue", 1.05}, {"raw umber", 0.55}, {"bone black", 0.45}, {"green earth", 0.3}, {"vermilion", 0.08}}
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.45}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}

for _, band in ipairs({rect(0, 826, 266, 30), rect(0, 940, 266, 30)}) do
  work(band, {hand="scumble", pile=watermid3, coverage=0.8, angle=0.0, edge=1})
  work(band, {hand="scumble", pile=darker, coverage=0.5, angle=0.0, edge=1})
  blend(band, {coverage=0.8})
end
for _, band in ipairs({rect(296, 712, 235, 26), rect(296, 996, 235, 26), rect(292, 760, 24, 260), rect(517, 760, 20, 260)}) do
  work(band, {hand="scumble", pile=watermid3, coverage=0.7, angle=0.0, edge=1})
  work(band, {hand="scumble", pile=darker, coverage=0.4, angle=0.0, edge=1})
  blend(band, {coverage=0.7})
end

-- bank top patch where the slate spilled
bp = ellipse(400, 1005, 60, 18) + ellipse(430, 1012, 40, 14)
work(bp, {hand="body", pile=spitdark, coverage=2.2, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.4, clip=true})
print("edges married")

--@ chunk 72
-- Re-grade the lower water; re-cut the light path
deep2 = pile{{"lead white", 0.75}, {"cobalt blue", 0.95}, {"raw umber", 0.6}, {"bone black", 0.5}, {"green earth", 0.35}, {"vermilion", 0.1}}
deep3 = pile{{"lead white", 0.6}, {"cobalt blue", 1.0}, {"raw umber", 0.7}, {"bone black", 0.6}, {"green earth", 0.4}, {"vermilion", 0.08}}
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}
waterdeep = pile{{"lead white", 0.7}, {"cobalt blue", 0.9}, {"raw umber", 0.6}, {"bone black", 0.55}, {"green earth", 0.35}, {"vermilion", 0.08}}

spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
notbank = mask(function(x, y) return y < spit_edge(x) and 1 or 0 end)

bandA = below(function(x) return 875 + 18 * math.sin(x / 80) end) * above(function(x) return 1075 + 15 * math.sin(x / 70) end) * notbank
work(bandA, {hand="body", pile=deep2, coverage=2.4, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1, clip=true})
blend(bandA, {coverage=0.7})

bandB2 = below(function(x) return 1260 end) * above(function(x) return 1075 + 15 * math.sin(x / 70) end)
work(bandB2, {hand="body", pile=deep3, coverage=2.4, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1, clip=true})
work(bandB2, {hand="scumble", pile=deep3, coverage=1.0, angle=0.02, edge=1})
blend(bandB2, {coverage=0.7})

lowpath = ribbon({{612, 1000}, {572, 1130}, {505, 1250}}, {150, 215, 260})
work(lowpath, {hand="body", pile=bright, coverage=1.6, fill=true, angle=0.02, pressure={0.6, 0.8}, edge=1})
wide2 = ribbon({{620, 1000}, {580, 1140}, {520, 1250}}, {230, 320, 380})
work(wide2, {hand="broad", pile=flare, coverage=0.8, fill=false, angle=0.02, pressure={0.5, 0.7}, edge=1})

rimb = rect(0, 1218, 1000, 34)
work(rimb, {hand="scumble", pile=waterdeep, coverage=0.8, angle=0.02, edge=1})
blend(rimb, {coverage=0.7})
print("lower water re-graded")

--@ chunk 73
-- Streaky water: alternating horizontal bands across the width
deep2 = pile{{"lead white", 0.75}, {"cobalt blue", 0.95}, {"raw umber", 0.6}, {"bone black", 0.5}, {"green earth", 0.35}, {"vermilion", 0.1}}
deep3 = pile{{"lead white", 0.6}, {"cobalt blue", 1.0}, {"raw umber", 0.7}, {"bone black", 0.6}, {"green earth", 0.4}, {"vermilion", 0.08}}
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}
spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
notbank = mask(function(x, y) return y < spit_edge(x) and 1 or 0 end)

bands = {
  {690, 745, flare, 0.45},
  {775, 825, deep2, 0.5},
  {845, 905, deep2, 0.55},
  {925, 985, flare, 0.3},
  {1010, 1075, deep3, 0.55},
  {1100, 1180, deep3, 0.6},
}
for _, bd in ipairs(bands) do
  local m = below(function(x) return bd[2] + 12 * math.sin(x / 64 + bd[1]) end)
          * above(function(x) return bd[1] - 12 * math.sin(x / 71) end)
          * notbank
  work(m, {hand="scumble", pile=bd[3], coverage=bd[4], angle=0.0, edge=1})
  blend(m, {coverage=0.6})
end
print("streaks in")

--@ chunk 74
-- Registered streak bands (broad hand)
deep2 = pile{{"lead white", 0.75}, {"cobalt blue", 0.95}, {"raw umber", 0.6}, {"bone black", 0.5}, {"green earth", 0.35}, {"vermilion", 0.1}}
deep3 = pile{{"lead white", 0.6}, {"cobalt blue", 1.0}, {"raw umber", 0.7}, {"bone black", 0.6}, {"green earth", 0.4}, {"vermilion", 0.08}}
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}
spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
notbank = mask(function(x, y) return y < spit_edge(x) and 1 or 0 end)

bands = {
  {695, 748, flare, 0.5},
  {778, 828, deep2, 0.55},
  {848, 908, deep2, 0.6},
  {928, 988, flare, 0.35},
  {1012, 1078, deep3, 0.6},
  {1105, 1185, deep3, 0.65},
}
for _, bd in ipairs(bands) do
  local m = below(function(x) return bd[2] + 10 * math.sin(x / 64 + bd[1]) end)
          * above(function(x) return bd[1] - 10 * math.sin(x / 71) end)
          * notbank
  work(m, {hand="broad", pile=bd[3], coverage=bd[4], angle=0.0, pressure={0.55, 0.75}, edge=1})
  blend(m, {coverage=0.7})
end
print("streaks laid")

--@ chunk 75
-- Registered streak bands, masks the right way round
deep2 = pile{{"lead white", 0.75}, {"cobalt blue", 0.95}, {"raw umber", 0.6}, {"bone black", 0.5}, {"green earth", 0.35}, {"vermilion", 0.1}}
deep3 = pile{{"lead white", 0.6}, {"cobalt blue", 1.0}, {"raw umber", 0.7}, {"bone black", 0.6}, {"green earth", 0.4}, {"vermilion", 0.08}}
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}
spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
notbank = mask(function(x, y) return y < spit_edge(x) and 1 or 0 end)

bands = {
  {695, 748, flare, 0.5},
  {778, 828, deep2, 0.55},
  {848, 908, deep2, 0.6},
  {928, 988, flare, 0.35},
  {1012, 1078, deep3, 0.6},
  {1105, 1185, deep3, 0.65},
}
for _, bd in ipairs(bands) do
  local m = below(function(x) return bd[1] - 10 * math.sin(x / 71) end)
          * above(function(x) return bd[2] + 10 * math.sin(x / 64 + bd[1]) end)
          * notbank
  work(m, {hand="broad", pile=bd[3], coverage=bd[4], angle=0.0, pressure={0.55, 0.75}, edge=1})
  blend(m, {coverage=0.7})
end
print("streaks laid")

--@ chunk 76
-- Rebuild the yacht, crisp, on the clean water
hullc = pile{{"bone black", 0.55}, {"raw umber", 0.65}, {"cobalt blue", 0.3}, {"lead white", 0.3}}
warmfleck = pile{{"lead white", 2.0}, {"vermilion", 0.3}, {"chrome yellow", 0.15}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}

hull = ribbon({{350, 912}, {398, 917}, {442, 911}, {476, 901}}, {22, 30, 24, 7})
work(hull, {hand="body", pile=hullc, coverage=2.6, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.25, clip=true})
work(hull, {hand="detail", pile=hullc, coverage=1.5, fill=true, angle=0.02, edge=0.2, clip=true})

cabin = ribbon({{402, 893}, {446, 890}}, 15)
work(cabin, {hand="body", pile=hullc, coverage=1.8, fill=true, angle=0.02, edge=0.3, clip=true})

mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hullc, 0.95)
mast = outline{{421, 893}, {424, 820}, {427, 742}}
mast:paint(mb, {pressure=0.5, every=2})

nub = ellipse(425, 808, 5, 13)
work(nub, {hand="detail", pile=hullc, coverage=1.8, fill=true, edge=0.25, clip=true})

stipple(hull:rim(3, 1) * mask(function(x, y) return x > 430 and 1 or 0 end),
  {pile=warmpale2, width=2.5, coverage=0.12, pressure={0.15, 0.3}, feather=1})

-- broken reflection: short horizontal dark dashes below the hull
reflz = rect(360, 936, 110, 80)
stipple(reflz, {pile=hullc, width=4, coverage=0.25, pressure={0.2, 0.45}, drag={9, 0}, feather=1})
print("yacht rebuilt")

--@ chunk 77
-- Darken the hull; light catches the sheer
hulld = pile{{"bone black", 0.7}, {"raw umber", 0.7}, {"cobalt blue", 0.25}, {"lead white", 0.2}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}

hull = ribbon({{350, 912}, {398, 917}, {442, 911}, {476, 901}}, {22, 30, 24, 7})
work(hull, {hand="body", pile=hulld, coverage=2.0, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.25, clip=true})

sheer = below(function(x) return 910 - 0.03 * x end) * above(function(x) return 886 - 0.03 * x end)
       * mask(function(x) return x > 352 and x < 478 and 1 or 0 end)
stipple(sheer, {pile=warmpale2, width=2.5, coverage=0.22, pressure={0.2, 0.4}, drag={6, 0}, feather=1})
print("hull darkened")

--@ chunk 78
-- Broad glow field on the water; dark left rim; boat reflection
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}
deep3 = pile{{"lead white", 0.6}, {"cobalt blue", 1.0}, {"raw umber", 0.7}, {"bone black", 0.6}, {"green earth", 0.4}, {"vermilion", 0.08}}
hullc = pile{{"bone black", 0.55}, {"raw umber", 0.65}, {"cobalt blue", 0.3}, {"lead white", 0.3}}

-- glow field: soft feathered pale wash around the path, edges feathered out
glowz = rect(280, 730, 290, 300):soften(30)
work(glowz, {hand="broad", pile=flare, coverage=0.55, angle=0.0, pressure={0.45, 0.65}, edge=1})
blend(glowz, {coverage=0.7})

-- left rim dark
work(rect(0, 516, 16, 630), {hand="body", pile=deep3, coverage=2.2, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})
blend(rect(0, 516, 30, 630), {coverage=0.7})

-- boat reflection, a touch stronger
stipple(rect(355, 934, 120, 92), {pile=hullc, width=4.5, coverage=0.3, pressure={0.25, 0.5}, drag={10, 0}, feather=1})
print("glow field, rim, reflection")

--@ chunk 79
-- Hull dark again, mast and cabin crisp
hulld = pile{{"bone black", 0.75}, {"raw umber", 0.7}, {"cobalt blue", 0.25}, {"lead white", 0.15}}
hull = ribbon({{350, 912}, {398, 917}, {442, 911}, {476, 901}}, {22, 30, 24, 7})
work(hull, {hand="body", pile=hulld, coverage=2.4, fill=true, angle=0.02, pressure={0.85, 0.95}, edge=0.25, clip=true})
work(hull, {hand="detail", pile=hulld, coverage=1.4, fill=true, angle=0.02, edge=0.2, clip=true})

cabin = ribbon({{402, 893}, {446, 890}}, 15)
work(cabin, {hand="body", pile=hulld, coverage=1.6, fill=true, angle=0.02, edge=0.3, clip=true})

mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hulld, 0.95)
mast = outline{{421, 892}, {424, 820}, {427, 744}}
mast:paint(mb, {pressure=0.55, every=2})
nub = ellipse(425, 808, 5, 13)
work(nub, {hand="detail", pile=hulld, coverage=1.6, fill=true, edge=0.25, clip=true})
print("boat dark and crisp")

--@ chunk 80
-- Proper hull silhouette, no balloon
hulld = pile{{"bone black", 0.75}, {"raw umber", 0.7}, {"cobalt blue", 0.25}, {"lead white", 0.15}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}
watermid3 = pile{{"lead white", 1.0}, {"cobalt blue", 0.95}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.28}, {"vermilion", 0.08}}

-- knock back the old lens and bump with water, then draw the hull poly
oldboat = rect(330, 878, 170, 56)
work(oldboat, {hand="body", pile=watermid3, coverage=2.4, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1, clip=true})
blend(oldboat, {coverage=0.7})

hullpoly = poly({{352, 898}, {395, 893}, {440, 889}, {478, 883}, {477, 916}, {432, 922}, {390, 924}, {354, 918}}, true)
work(hullpoly, {hand="body", pile=hulld, coverage=2.6, fill=true, angle=0.02, pressure={0.85, 0.95}, edge=0.2, clip=true})
work(hullpoly, {hand="detail", pile=hulld, coverage=1.5, fill=true, angle=0.02, edge=0.2, clip=true})

-- warm sheer line
sheer = below(function(x) return 897 - 0.032 * x end) * above(function(x) return 887 - 0.032 * x end)
       * mask(function(x) return x > 354 and x < 476 and 1 or 0 end)
stipple(sheer, {pile=warmpale2, width=2.2, coverage=0.3, pressure={0.2, 0.4}, drag={6, 0}, feather=1})

-- smaller knot on the mast
mb = brush{kind="rigger", width=3, point=1, stiffness=0.5}
mb:load(hulld, 0.95)
mb:stroke({{424, 800}, {425, 790}}, {pressure=0.6, ramps={0.2, 0.2}})

-- reflection dashes
stipple(rect(360, 934, 110, 76), {pile=hulld, width=4, coverage=0.3, pressure={0.25, 0.5}, drag={9, 0}, feather=1})
print("hull redrawn")

--@ chunk 81
-- Feather cream edge; mast knot off; left sliver; sparkles
skyfixp = pile{{"lead white", 2.6}, {"chrome yellow", 0.5}, {"vermilion", 0.22}, {"raw umber", 0.1}}
watermid3 = pile{{"lead white", 1.0}, {"cobalt blue", 0.95}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.28}, {"vermilion", 0.08}}
deep3 = pile{{"lead white", 0.6}, {"cobalt blue", 1.0}, {"raw umber", 0.7}, {"bone black", 0.6}, {"green earth", 0.4}, {"vermilion", 0.08}}
hulld = pile{{"bone black", 0.75}, {"raw umber", 0.7}, {"cobalt blue", 0.25}, {"lead white", 0.15}}
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}

-- feather the cream cloud bank's right edge into the gold sky
fe = rect(868, 235, 90, 200):soften(30)
work(fe, {hand="scumble", pile=skyfixp, coverage=0.8, angle=0.02, edge=1})
work(rect(880, 250, 70, 170), {hand="scumble", pile=skyfixp, coverage=1.2, angle=0.02, edge=1})
blend(fe, {coverage=0.8})

-- mast knot off
knot = ellipse(424, 808, 10, 18)
work(knot, {hand="detail", pile=watermid3, coverage=2.2, fill=true, edge=0.3, clip=true})
mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hulld, 0.95)
mast = outline{{421, 892}, {424, 820}, {427, 744}}
mast:paint(mb, {pressure=0.55, every=2})

-- left sliver
work(rect(0, 516, 22, 630), {hand="body", pile=deep3, coverage=2.2, fill=true, angle=0.02, pressure={0.7, 0.9}, edge=1})
blend(rect(0, 516, 40, 630), {coverage=0.7})

-- sparkles down the path core
sparkz = ribbon({{640, 516}, {700, 640}, {660, 830}, {610, 1000}, {560, 1250}}, {26, 44, 70, 100, 130})
stipple(sparkz, {pile=bright, width=3.5, coverage=0.35, pressure={0.3, 0.6}, drag={7, 0}, feather=1})
print("final touches")

--@ chunk 82
-- Return the slab to the amber sky
goldsky = pile{{"lead white", 1.5}, {"chrome yellow", 0.7}, {"vermilion", 0.25}, {"raw umber", 0.3}, {"cobalt blue", 0.1}}
cloudtone = pile{{"lead white", 2.0}, {"chrome yellow", 0.3}, {"vermilion", 0.15}, {"raw umber", 0.2}}

z = rect(845, 218, 115, 230)
work(z, {hand="body", pile=goldsky, coverage=2.8, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.5, clip=true})
blend(z, {coverage=0.8})

cz = rect(830, 240, 130, 180)
work(cz, {hand="scumble", pile=cloudtone, coverage=0.6, angle=0.02, edge=1})
print("slab to gold")

--@ chunk 83
-- One cream glow mass, soft edges, then leave it alone
skyfixp = pile{{"lead white", 2.6}, {"chrome yellow", 0.5}, {"vermilion", 0.22}, {"raw umber", 0.1}}
softcream = rect(838, 250, 145, 240):soften(24)
work(softcream, {hand="body", pile=skyfixp, coverage=2.4, fill=true, angle=0.02, pressure={0.7, 0.85}, edge=1, clip=true})
work(softcream, {hand="scumble", pile=skyfixp, coverage=1.0, angle=0.02, edge=1})
blend(softcream, {coverage=0.8})
print("glow unified")

--@ chunk 84
-- Haze the cream mass into the sky
cloudtone = pile{{"lead white", 2.0}, {"chrome yellow", 0.3}, {"vermilion", 0.15}, {"raw umber", 0.2}}
amber2 = pile{{"lead white", 1.0}, {"chrome yellow", 0.5}, {"vermilion", 0.25}, {"raw umber", 0.55}, {"cobalt blue", 0.1}}

topz = rect(460, 222, 540, 50):soften(18)
work(topz, {hand="scumble", pile=amber2, coverage=0.7, angle=0.02, edge=1})
blend(topz, {coverage=0.8})

leftz = rect(450, 255, 45, 215):soften(14)
work(leftz, {hand="scumble", pile=amber2, coverage=0.8, angle=0.02, edge=1})
blend(leftz, {coverage=0.8})

work(rect(455, 250, 80, 60), {hand="scumble", pile=amber2, coverage=0.9, angle=0.02, edge=1})
print("hazed")

--@ chunk 85
-- Amber sky above, tall dark headland below, burning rim between
amber2 = pile{{"lead white", 1.0}, {"chrome yellow", 0.5}, {"vermilion", 0.25}, {"raw umber", 0.55}, {"cobalt blue", 0.1}}
cloudtone = pile{{"lead white", 2.0}, {"chrome yellow", 0.3}, {"vermilion", 0.15}, {"raw umber", 0.2}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}

ambez = rect(450, 215, 550, 140)
work(ambez, {hand="body", pile=amber2, coverage=2.8, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=1, clip=true})
blend(ambez, {coverage=0.8})
work(rect(460, 250, 530, 90), {hand="scumble", pile=cloudtone, coverage=0.5, angle=0.02, edge=1})

hz = function(x) return 512 + 4 * math.sin(x / 40) end
headz = below(function(x) return 348 + 12 * math.sin(x / 61 + 2) end) * above(hz) * rect(440, 330, 560, 220)
work(headz, {hand="body", pile=treedark3, coverage=3.0, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.2, clip=true})
work(headz, {hand="scumble", pile=treedark3, coverage=1.6, angle=0.02, edge=0.25})

rimz = below(function(x) return 360 + 12 * math.sin(x / 61 + 2) end) * above(function(x) return 336 + 12 * math.sin(x / 61 + 2) end) * rect(450, 320, 550, 60)
stipple(rimz, {pile=bright, width=3, coverage=0.5, pressure={0.25, 0.55}, feather=1})
print("headland resolved")

--@ chunk 86
-- Marry the three hard boundaries
amber2 = pile{{"lead white", 1.0}, {"chrome yellow", 0.5}, {"vermilion", 0.25}, {"raw umber", 0.55}, {"cobalt blue", 0.1}}
cloudtone = pile{{"lead white", 2.0}, {"chrome yellow", 0.3}, {"vermilion", 0.15}, {"raw umber", 0.2}}
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
skymid2 = pile{{"lead white", 1.3}, {"cobalt blue", 0.8}, {"raw umber", 0.35}, {"vermilion", 0.2}}
hz = function(x) return 512 + 4 * math.sin(x / 40) end

ambLz = rect(420, 218, 70, 135):soften(16)
work(ambLz, {hand="scumble", pile=cloudtone, coverage=0.9, angle=0.02, edge=1})
work(ambLz, {hand="scumble", pile=amber2, coverage=0.7, angle=0.02, edge=1})
blend(ambLz, {coverage=0.9})

topz2 = rect(450, 193, 550, 46):soften(14)
work(topz2, {hand="scumble", pile=skymid2, coverage=0.6, angle=0.02, edge=1})
blend(topz2, {coverage=0.9})

headl = below(function(x) return 448 + 18 * math.sin(x / 31 + 2) + 10 * math.sin(x / 67) end)
       * above(hz) * rect(300, 330, 190, 220)
work(headl, {hand="body", pile=treedark3, coverage=2.2, fill=true, angle=0.02, pressure={0.75, 0.9}, edge=0.4, clip=true})

tope = below(function(x) return 358 + 12 * math.sin(x / 61 + 2) end) * above(function(x) return 340 + 12 * math.sin(x / 61 + 2) end) * rect(440, 330, 560, 40)
stipple(tope, {pile=treedark3, width=4, coverage=0.9, pressure={0.3, 0.6}, feather=1})
print("married")

--@ chunk 87
-- Final sky: one smooth dusk gradient over everything above the headland
skA = pile{{"lead white", 1}, {"cobalt blue", 1.1}, {"raw umber", 0.5}, {"vermilion", 0.25}}
skB = pile{{"lead white", 1.4}, {"cobalt blue", 0.8}, {"raw umber", 0.4}, {"vermilion", 0.22}}
skC = pile{{"lead white", 1.3}, {"chrome yellow", 0.55}, {"vermilion", 0.22}, {"raw umber", 0.45}}
skD = pile{{"lead white", 2.2}, {"chrome yellow", 0.42}, {"vermilion", 0.2}, {"raw umber", 0.25}}
hz = function(x) return 512 + 4 * math.sin(x / 40) end

b1 = below(function(x) return 165 + 10 * math.sin(x / 80) end)
work(b1, {hand="broad", pile=skA, coverage=2.6, fill=true, angle=0.03, pressure={0.7, 0.9}, edge=1})
blend(b1, {coverage=0.8})

b2 = below(function(x) return 285 + 10 * math.sin(x / 75) end) * above(function(x) return 125 + 10 * math.sin(x / 80) end)
work(b2, {hand="broad", pile=skB, coverage=2.4, fill=true, angle=0.03, pressure={0.7, 0.9}, edge=1})
blend(b2, {coverage=0.8})

b3 = below(function(x) return 365 + 10 * math.sin(x / 70) end) * above(function(x) return 245 + 10 * math.sin(x / 75) end)
work(b3, {hand="broad", pile=skC, coverage=2.4, fill=true, angle=0.03, pressure={0.7, 0.9}, edge=1})
blend(b3, {coverage=0.8})

b4 = below(function(x) return 352 + 8 * math.sin(x / 70) end) * above(function(x) return 300 + 8 * math.sin(x / 70) end)
work(b4, {hand="broad", pile=skD, coverage=2.2, fill=true, angle=0.03, pressure={0.7, 0.9}, edge=1})
blend(b4, {coverage=0.8})

b5 = mask(function(x, y)
  if y > 444 or y < 352 then return 0 end
  if x > 440 and y > 348 then return 0 end
  return 1
end)
work(b5, {hand="broad", pile=skD, coverage=2.2, fill=true, angle=0.03, pressure={0.7, 0.9}, edge=1})
blend(b5, {coverage=0.8})

bigblend = above(function(x) return 345 end)
blend(bigblend, {coverage=1.0})
print("final sky laid")

--@ chunk 88
-- Re-establish treeline and headland over the veil
treedark3 = pile{{"lead white", 0.3}, {"cobalt blue", 0.5}, {"green earth", 0.3}, {"raw umber", 0.65}, {"bone black", 0.75}}
hz = function(x) return 512 + 4 * math.sin(x / 40) end

tlband = below(function(x) return 452 + 20 * math.sin(x / 31 + 2) + 12 * math.sin(x / 67) + 8 * math.sin(x / 13) end) * above(hz)
work(tlband, {hand="body", pile=treedark3, coverage=2.8, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.35, clip=true})
work(tlband, {hand="scumble", pile=treedark3, coverage=1.6, angle=0.02, edge=0.4})

headz = below(function(x) return 356 + 14 * math.sin(x / 61 + 2) end) * above(hz) * rect(440, 330, 560, 220)
work(headz, {hand="body", pile=treedark3, coverage=2.8, fill=true, angle=0.02, pressure={0.8, 0.95}, edge=0.2, clip=true})
work(headz, {hand="scumble", pile=treedark3, coverage=1.4, angle=0.02, edge=0.25})

tope = below(function(x) return 362 + 14 * math.sin(x / 61 + 2) end) * above(function(x) return 344 + 14 * math.sin(x / 61 + 2) end) * rect(440, 330, 560, 40)
stipple(tope, {pile=treedark3, width=4, coverage=1.0, pressure={0.3, 0.6}, feather=1})
print("treeline and headland back")

--@ chunk 89
-- Bank, path, boat over the veil
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.5}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}
flare  = pile{{"lead white", 1.6}, {"chrome yellow", 0.35}, {"vermilion", 0.2}, {"cobalt blue", 0.12}}
hulld = pile{{"bone black", 0.75}, {"raw umber", 0.7}, {"cobalt blue", 0.25}, {"lead white", 0.15}}

spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
bank = below(spit_edge)
work(bank, {hand="body", pile=spitdark, coverage=2.4, fill=true, angle=0.05, pressure={0.75, 0.9}, edge=0.5, clip=true})
work(bank, {hand="scumble", pile=spitdark, coverage=1.2, angle=0.05, edge=0.6})

lowpath = ribbon({{640, 516}, {700, 640}, {660, 830}, {610, 1000}, {560, 1250}}, {30, 60, 110, 180, 230})
work(lowpath, {hand="body", pile=bright, coverage=1.6, fill=true, angle=0.02, pressure={0.6, 0.8}, edge=1})
wide2 = ribbon({{640, 516}, {705, 650}, {665, 850}, {605, 1040}, {565, 1250}}, {70, 130, 220, 300, 360})
work(wide2, {hand="broad", pile=flare, coverage=0.7, fill=false, angle=0.02, pressure={0.5, 0.7}, edge=1})

hull = poly({{352, 898}, {395, 893}, {440, 889}, {478, 883}, {477, 916}, {432, 922}, {390, 924}, {354, 918}}, true)
work(hull, {hand="body", pile=hulld, coverage=2.6, fill=true, angle=0.02, pressure={0.85, 0.95}, edge=0.2, clip=true})
work(hull, {hand="detail", pile=hulld, coverage=1.4, fill=true, angle=0.02, edge=0.2, clip=true})
mb = brush{kind="rigger", width=3.5, point=1, stiffness=0.5}
mb:load(hulld, 0.95)
mast = outline{{421, 892}, {424, 820}, {427, 744}}
mast:paint(mb, {pressure=0.55, every=2})
stipple(rect(360, 934, 110, 76), {pile=hulld, width=4, coverage=0.3, pressure={0.25, 0.5}, drag={9, 0}, feather=1})
print("bank, path, boat re-stated")

--@ chunk 90
-- Path dies into the bank; gold band veil; sparkles
spitdark  = pile{{"raw umber", 1}, {"bone black", 0.5}, {"green earth", 0.35}, {"cobalt blue", 0.2}, {"lead white", 0.25}}
amber2 = pile{{"lead white", 1.0}, {"chrome yellow", 0.5}, {"vermilion", 0.25}, {"raw umber", 0.55}, {"cobalt blue", 0.1}}
cloudtone = pile{{"lead white", 2.0}, {"chrome yellow", 0.3}, {"vermilion", 0.15}, {"raw umber", 0.2}}
bright = pile{{"lead white", 2.2}, {"chrome yellow", 0.5}, {"vermilion", 0.25}}

spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
bank = below(spit_edge)
lowpath = ribbon({{640, 516}, {700, 640}, {660, 830}, {610, 1000}, {560, 1250}}, {30, 60, 110, 180, 230})
wide2 = ribbon({{640, 516}, {705, 650}, {665, 850}, {605, 1040}, {565, 1250}}, {70, 130, 220, 300, 360})
work(bank * (lowpath + wide2), {hand="body", pile=spitdark, coverage=2.4, fill=true, angle=0.05, pressure={0.75, 0.9}, edge=0.4, clip=true})

goldz = rect(300, 150, 700, 120)
work(goldz, {hand="scumble", pile=amber2, coverage=1.1, angle=0.02, edge=1})
work(goldz, {hand="scumble", pile=cloudtone, coverage=0.4, angle=0.02, edge=1})
blend(goldz, {coverage=0.8})

sparkz = ribbon({{640, 516}, {700, 640}, {660, 830}, {610, 1000}}, {26, 50, 100, 170})
stipple(sparkz, {pile=bright, width=3.5, coverage=0.4, pressure={0.3, 0.6}, drag={7, 0}, feather=1})
print("path dies into bank, gold veiled")

--@ chunk 91
-- Blend the gold band's edges out into the mist
softm = rect(270, 95, 730, 200):soften(36)
blend(softm, {coverage=1.2})
blend(softm, {coverage=1.0})
print("gold blended out")

--@ chunk 92
-- Last small things
watermid3 = pile{{"lead white", 1.0}, {"cobalt blue", 0.95}, {"raw umber", 0.45}, {"bone black", 0.35}, {"green earth", 0.28}, {"vermilion", 0.08}}
hulld = pile{{"bone black", 0.75}, {"raw umber", 0.7}, {"cobalt blue", 0.25}, {"lead white", 0.15}}
warmpale2 = pile{{"lead white", 2.4}, {"vermilion", 0.12}, {"chrome yellow", 0.1}}

knotz = ellipse(424, 806, 8, 15)
work(knotz, {hand="detail", pile=watermid3, coverage=2.4, fill=true, edge=0.3, clip=true})
mb = brush{kind="rigger", width=3, point=1, stiffness=0.5}
mb:load(hulld, 0.95)
mast = outline{{421, 890}, {424, 822}, {427, 746}}
mast:paint(mb, {pressure=0.5, every=2})

hull = poly({{352, 898}, {395, 893}, {440, 889}, {478, 883}, {477, 916}, {432, 922}, {390, 924}, {354, 918}}, true)
sheer = hull * mask(function(x, y) return y < 906 and 1 or 0 end)
stipple(sheer, {pile=warmpale2, width=2.2, coverage=0.25, pressure={0.2, 0.4}, drag={6, 0}, feather=1})

blend(rect(270, 175, 260, 60), {coverage=1.0})
print("last touches")

--@ chunk 93
-- A few reeds against the light at the bank edge
grassdark = pile{{"bone black", 0.35}, {"raw umber", 0.55}, {"green earth", 0.45}, {"cobalt blue", 0.15}, {"lead white", 0.15}}
g = brush{kind="round", width=9, point=1, stiffness=0.5}
spit_edge = function(x) return 1150 + 26 * math.sin(x / 90 + 1) + 14 * math.sin(x / 38 + 2) - 0.28 * x end
xs = uneven(7, 430, 900, 0.7, 0.4, 21)
for _, x in ipairs(xs) do
  if g:fullness() < 0.3 then g:wipe(0.5); g:load(grassdark, 0.95) end
  local hgt = rand(26, 52)
  if x > 560 and x < 760 then hgt = rand(38, 64) end
  local lean = rand(-12, 24)
  local base = spit_edge(x) + 5
  g:stroke({{x, base}, {x + lean * 0.4, base - hgt * 0.55}, {x + lean, base - hgt}},
    {pressure={0.85, 0.05}, ramps={0.08, 0.55}})
end
print("reeds struck")

--@ chunk 94

-- hinge strokes along the amber cloud: come up from below with dark, and
-- soften the left edge of the amber with strokes that cross the seam
p_deep = pile{{"bone black", 2}, {"raw umber", 2}, {"cobalt blue", 1}, medium=0.55}
print("mixed deep cool dark")
-- work a soft band where the cloud meets the grey: nothing new, just reseating the seam
m = rect(600, 0, 480, 380)
work(m, {hand="scumble", pile=p_deep, angle=0.15, coverage=0.5, load=0.35,
         edge={lost=0.7}, broken=0.5, seed=41})
blend(m:soften(30), {angle=0.1})

--@ chunk 95

-- taller cloud puffs, warm crest on the lit shoulders
p_crest = pile{{"lead white", 3}, {"yellow ochre", 1.3}, medium=0.5}
p_crest2 = pile{{"lead white", 5}, {"yellow ochre", 1}, {"vermilion", 0.5}, medium=0.5}
n = noise{seed=17, octaves=4, period=180, warp={150, 40}}
-- an upper crest: soft strokes climbing above the old band
mm = mask(function(x, y)
  local z = 250 + 45 * math.sin((x - 620) / 190) - 28 * n:at01(x * 1.7, y)
  return (x > 560 and y < z + 40 and y > z - 95) and 1 or 0
end)
work(mm:soften(25), {hand="body", pile=p_crest, angle=0.5, coverage=1.1,
       edge={soft=0.5, lost=0.5}, seed=52})
work(mm:soften(25), {hand="body", pile=p_crest2, angle=0.6, coverage=0.55,
       edge={lost=0.8}, seed=53})
blend(mm:soften(40), {angle=0.4})

--@ chunk 96
print(wait(260))

--@ chunk 97

-- rescue: the upper right sky was turned to sludge. repaint it as sky.
p_sky = pile{{"lead white", 4}, {"cobalt blue", 1.1}, {"raw umber", 0.45}, medium=0.35}
p_sky2 = pile{{"lead white", 3.5}, {"cobalt blue", 1.3}, medium=0.3}
m = rect(540, 0, 500, 180)
work(m, {hand="broad", pile=p_sky, angle=0.05, coverage=1.6, edge={lost=0.6, soft=0.4}, seed=61})
work(m, {hand="broad", pile=p_sky2, angle=0.1, coverage=0.8, edge={lost=0.9}, seed=62})
blend(rect(520, 0, 520, 200):soften(60), {angle=0.05})

--@ chunk 98

p_sky3 = pile{{"lead white", 5}, {"cobalt blue", 1.2}, {"raw umber", 0.5}, medium=0.3}
m = rect(470, 0, 530, 200)
work(m, {hand="broad", pile=p_sky3, angle=-0.06, coverage=2.2, seed=63})
blend(rect(430, 0, 570, 240):soften(80), {angle=0.0})

--@ chunk 99

-- rescue: the upper sky was turned to sludge by the dark scumble
p_sky = pile{{"lead white", 4}, {"cobalt blue", 1.1}, {"raw umber", 0.45}, medium=0.35}
p_sky2 = pile{{"lead white", 3.5}, {"cobalt blue", 1.3}, medium=0.3}
m = rect(540, 0, 500, 180)
work(m, {hand="broad", pile=p_sky, angle=0.05, coverage=1.6, edge={lost=0.6, soft=0.4}, seed=61})
work(m, {hand="broad", pile=p_sky2, angle=0.1, coverage=0.8, edge={lost=0.9}, seed=62})
blend(rect(520, 0, 520, 200):soften(60), {angle=0.05})

--@ chunk 100

-- Mist rising off the water along the base of the headland, and veiling
-- the far shore where the shadow-water meets it. A pale scent, mostly at the seam.
p_mist = pile{{"lead white", 3}, {"pale smalt", 1.6}, {"yellow ochre", 0.5}, {"bone black", 0.15}, medium=0.6}
n = noise{seed=23, octaves=3, period=140, persistence=0.55}
m = mask(function(x, y)
  local d = y - 385                       -- 0 at the water/shore seam
  local mist = math.exp(-((y - 400) ^ 2) / (2 * 55 ^ 2))
  local drifting = n:at01(x * 2.1, y * 1.3) * 0.5
  return clamp(mist * (0.55 + drifting), 0, 1)
end)
m = m * rect(0, 300, 1000, 190)
work(m, {hand="scumble", pile=p_mist, angle=0.0, coverage=0.7,
         edge={lost=0.9}, broken=0.4, load=0.3, seed=71})
print("mist laid")

--@ chunk 101

-- soften the mist into the seam, then a second fainter pass higher to
-- suggest the mist standing off the water
blend(rect(0, 330, 1000, 160):soften(40), {angle=0.0, seed=72})
p_mist2 = pile{{"lead white", 4}, {"pale smalt", 1.2}, medium=0.75}
mm = mask(function(x, y)
  local band = math.exp(-((y - 430) ^ 2) / (2 * 38 ^ 2))
  return clamp(band * 0.5, 0, 1) * (x > 120 and 1 or (x / 120))
end)
work(mm, {hand="glaze", pile=p_mist2, coverage=0.55, edge={lost=0.95}, seed=73})
print("mist softened and veiled")

--@ chunk 102

-- knock the chalk band back: glaze the local greys over its brightest core,
-- left side water-grey, right side headland-grey, and lift the top of it
p_watgrey = pile{{"lead white", 2.4}, {"cobalt blue", 0.9}, {"raw umber", 0.5}, {"red earth", 0.2}, medium=0.7}
p_softdark = pile{{"raw umber", 1}, {"cobalt blue", 0.8}, {"lead white", 1.4}, medium=0.6}
-- the core of the stripe
m = rect(0, 330, 1000, 150)
work(m, {hand="glaze", pile=p_watgrey, coverage=0.85, edge={lost=0.9}, seed=74})
-- over the dark headland: hazard grey so it stops reading as white chalk
mh = rect(340, 320, 660, 120)
work(mh, {hand="glaze", pile=p_softdark, coverage=0.75, edge={lost=0.9}, seed=75})
print("knocked back")

--@ chunk 103

-- blend the seam zone so the band loses its hard edges and runs into both
-- water and headland
blend(rect(0, 300, 1000, 200):soften(50), {angle=0.02, seed=76})
print(wait(120))

--@ chunk 104

-- restate the far shore through the mist: dark shapes rising wet-in-wet
p_shore = pile{{"bone black", 0.7}, {"raw umber", 0.9}, {"cobalt blue", 0.5}, {"green earth", 0.35}, {"lead white", 0.25}}
n2 = noise{seed=29, octaves=4, period=90, persistence=0.5}
top = function(x)
  local trees = 415 - 26 * math.exp(-((x - 105) ^ 2) / 1200) - 15 * math.exp(-((x - 205) ^ 2) / 2200)
       - 12 * math.exp(-((x - 320) ^ 2) / 3200)
  return trees - (6 + 6 * n2:at01(x * 2.4, 0.3))
end
msh = mask(function(x, y) return (y > top(x) - 4 and y < 445) and 1 or 0 end)
msh = msh * rect(-10, 330, 560, 160)
work(msh, {hand="hatch", pile=p_shore, angle=-0.5, coverage=1.6, edge={found=0.4, soft=0.3, lost=0.3}, clip=true, seed=81})
pines = mask(function(x, y)
  local v = 0
  if y > 370 and y < 420 then v = v + math.exp(-((x - 118) ^ 2) / 260) * 0.9 end
  if y > 380 and y < 420 then v = v + math.exp(-((x - 200) ^ 2) / 300) end
  if y > 388 and y < 420 then v = v + math.exp(-((x - 320) ^ 2) / 500) end
  return clamp(v, 0, 1)
end)
work(pines * rect(0, 350, 560, 80), {hand="detail", pile=p_shore, angle=-0.9, coverage=1.8, edge={found=0.4, soft=0.4}, clip=true, seed=82})
print("far shore restated")

--@ chunk 105

-- now soften the shore into the mist: touches of the mist tone along its
-- foot and on the lit windows between the darks, then a blend on the top edge
p_m = pile{{"lead white", 3}, {"pale smalt", 1.4}, {"yellow ochre", 0.4}, medium=0.75}
mfoot = rect(0, 395, 560, 60)
work(mfoot, {hand="scumble", pile=p_m, angle=0.02, coverage=0.5, load=0.25, edge={lost=1}, seed=83})
-- a warm gap in the trees where the light comes through
p_warmgap = pile{{"lead white", 2.5}, {"yellow ochre", 1.2}, {"vermilion", 0.3}, medium=0.5}
gw = mask(function(x, y)
  return (x > 240 and x < 300 and y > 380 and y < 443) and smoothstep(300, 250, x) * smoothstep(380, 402, y) or 0
end)
work(gw, {hand="detail", pile=p_warmgap, coverage=0.9, edge={found=0.3, lost=0.7}, clip=true, seed=84})
print("shore softened into mist")

--@ chunk 106

-- unify: a smooth grey bank through the rubble, the colour of the far
-- shore at this hour, laid broadly so it reads as one plane
p_bank = pile{{"raw umber", 1.1}, {"bone black", 0.5}, {"cobalt blue", 0.55}, {"lead white", 1.9}, {"yellow ochre", 0.15}, medium=0.5}
mb = rect(0, 372, 640, 84):soften(18)
work(mb, {hand="broad", pile=p_bank, angle=0.02, coverage=2.4, ruler=true, edge={lost=0.4, soft=0.4}, seed=91})
blend(rect(0, 365, 660, 100):soften(30), {angle=0.02, seed=92})
print("bank unified")

--@ chunk 107

-- a few dark clumps on the bank: low trees, drawn not stencilled
p_clump = pile{{"bone black", 0.6}, {"raw umber", 0.85}, {"green earth", 0.4}, {"cobalt blue", 0.35}, medium=0.4}
b = brush{kind="round", width=5, point=0.8, stiffness=0.55}
b:load(p_clump, 0.85)
-- tree masses along the bank: cypress-like darks, a few
local specs = {
  {140, 372, 16, 34}, {166, 375, 12, 26}, {295, 380, 18, 30}, {318, 383, 12, 20},
  {452, 382, 14, 24}, {470, 380, 10, 28}, {565, 383, 13, 22}, {610, 385, 9, 16}
}
for _, s in ipairs(specs) do
  local x, topW, h = s[1], s[3], s[4]
  b:stroke({{x, 400 - 2}, {x + rand(-2, 2), 400 - h * 0.45}, {x + rand(-3, 4), 400 - h - 2}},
    {pressure={0.7, 0.25}, ramps={0.15, 0.4}})
end
print("clumps struck")

--@ chunk 108

-- rest the shore. Move to the amber cloud: knock down the stepped left edge
-- with broad soft strokes of the amber itself, extending the lobe left and
-- dissolving the outline into the blue-grey sky.
p_amber = pile{{"lead white", 2.6}, {"yellow ochre", 1.5}, {"raw umber", 0.4}, {"vermilion", 0.25}, medium=0.55}
p_amberpale = pile{{"lead white", 3.5}, {"yellow ochre", 1.2}, medium=0.65}
-- a soft wedge spreading left from the blob, following the cloud bank slope
wedge = mask(function(x, y)
  if x < 170 or x > 720 then return 0 end
  local centre = 205 + 0.36 * (x - 170)          -- slope of the cloud bank
  local halfw = 34 + 0.30 * (x - 170)
  local d = math.abs(y - centre)
  local v = smoothstep(halfw + 26, halfw - 6, d)
  -- ragged lower fringe like the lit cloud edge
  v = v * clamp(0.75 + 0.5 * math.sin(x / 17 + y / 9), 0, 1)
  return clamp(v, 0, 1)
end)
work(wedge, {hand="scumble", pile=p_amber, angle=0.34, coverage=1.0, load=0.5,
             edge={lost=0.8, soft=0.4}, seed=101})
work(wedge, {hand="scumble", pile=p_amberpale, angle=0.30, coverage=0.5, load=0.35,
             edge={lost=0.95}, seed=102})
print("amber wedge laid")

--@ chunk 109

-- blend the wedge and generously over the old stepped edge so the seam
-- dissolves; also blend its right end into the bright blob
blend(rect(140, 130, 700, 150):soften(35), {angle=0.31, seed=103})
print(wait(45))

--@ chunk 110

-- knock the pale cascade back: dusk grey over its middle and lower parts,
-- the local sky tone, so only a faint warm haze stays near the tree tops
p_dusk = pile{{"lead white", 2.6}, {"cobalt blue", 1.15}, {"raw umber", 0.75}, {"red earth", 0.15}, medium=0.6}
-- the cascade zone below the treetops, both sides
mc = mask(function(x, y) return (y > 210 and y < 420 and x > 240 and x < 640) and 1 or 0 end)
work(mc, {hand="broad", pile=p_dusk, angle=1.2, coverage=1.5, edge={lost=0.8}, seed=111})
work(mc, {hand="broad", pile=p_dusk, angle=1.0, coverage=1.0, edge={lost=0.9}, seed=112})
print("cascade knocked back")

--@ chunk 111

blend(rect(230, 190, 430, 250):soften(45), {angle=1.15, seed=113})
blend(rect(230, 190, 430, 250):soften(45), {angle=1.2, seed=114})
print(wait(20))

--@ chunk 112

-- full cover of the cascade with the dusk grey, opaque, extending up
-- through the amber to undo the smear completely; dry now, so a sit is needed
m = rect(250, 150, 380, 290)
work(m, {hand="broad", pile=p_dusk, angle=1.35, coverage=2.4, ruler=true, edge={lost=0.6}, seed=121})
work(m, {hand="broad", pile=p_dusk, angle=1.2, coverage=2.0, ruler=true, edge={lost=0.7}, seed=122})
blend(rect(240, 140, 400, 310):soften(40), {angle=1.25, seed=123})
print("cascade covered")

--@ chunk 113

p_warmfog = pile{{"lead white", 3.0}, {"yellow ochre", 1.6}, {"raw umber", 0.35}, {"red earth", 0.10}, {"bone black", 0.10}, medium=0.55}
m = rect(240, 140, 390, 300)
work(m, {hand="broad", pile=p_warmfog, angle=0.18, coverage=2.0, ruler=true, edge={lost=0.5}, seed=131})
work(m, {hand="broad", pile=p_warmfog, angle=0.10, coverage=1.2, ruler=true, edge={lost=0.7}, seed=132})
blend(rect(230, 130, 410, 320):soften(35), {angle=0.15, seed=133})
print("warm base laid")

--@ chunk 114

-- the lit core of the cloud: a brighter, warmer lobe where the light strikes,
-- just left of centre, soft-edged, wet into the warm fog
p_gold = pile{{"lead white", 4.5}, {"yellow ochre", 2.2}, {"vermilion", 0.25}, medium=0.4}
core = mask(function(x, y)
  if x < 290 or x > 570 then return 0 end
  local cy = 205 + 0.05 * (x - 290)
  local rx, ry = 135, 78
  local d = ((x - 420) / rx) ^ 2 + ((y - cy) / ry) ^ 2
  return clamp(smoothstep(1.15, 0.35, d) * (0.7 + 0.3 * math.sin(x / 23)), 0, 1)
end)
work(core, {hand="scumble", pile=p_gold, angle=0.15, coverage=1.1, load=0.45,
            edge={lost=0.9, soft=0.3}, seed=141})
blend(core:soften(25), {angle=0.12, seed=142})
-- a crest above: paler and cooler as the cloud tops catch the sky
p_crest = pile{{"lead white", 5}, {"yellow ochre", 1.0}, {"red earth", 0.15}, {"pale smalt", 0.5}, medium=0.5}
crest = mask(function(x, y)
  if x < 310 or x > 660 then return 0 end
  local base = 168 - 0.06 * (x - 310)
  local d = math.abs(y - base) + 14 * math.sin(x / 19 + 1)
  return clamp(smoothstep(46, 6, d) * smoothstep(660, 560, x), 0, 1)
end)
work(crest, {hand="scumble", pile=p_crest, angle=0.02, coverage=0.75, load=0.4,
             edge={lost=0.95, soft=0.4}, seed=143})
print("gold core and crest laid")

--@ chunk 115

blend(rect(280, 100, 390, 130):soften(38), {angle=0.1, seed=144})
-- an under-shadow on the right of the cloud where it turns away: warm grey-gold,
-- deeper than the fog, dragged over the right shoulder
p_shade = pile{{"lead white", 1.6}, {"raw umber", 1.1}, {"yellow ochre", 0.9}, {"bone black", 0.25}, medium=0.5}
shade = mask(function(x, y)
  if x < 500 or x > 700 then return 0 end
  return clamp(smoothstep(1.25, 0.45, ((x - 620) / 110) ^ 2 + ((y - 245) / 85) ^ 2) * 0.8, 0, 1)
end)
work(shade, {hand="scumble", pile=p_shade, angle=-0.2, coverage=0.8, load=0.4,
             edge={lost=0.9, soft=0.3}, seed=145})
blend(shade:soften(30), {angle=-0.15, seed=146})
print("cloud shaded")

--@ chunk 116

-- repay the treeline: one dark wood, cut with torn profile, drawn as tree
-- strokes so it reads as forest, wet into the warm haze above it
p_wood = pile{{"bone black", 0.75}, {"raw umber", 0.9}, {"green earth", 0.5}, {"cobalt blue", 0.4}, {"lead white", 0.22}, medium=0.45}
n3 = noise{seed=37, octaves=4, period=70, persistence=0.55}
topw = function(x)
  local base = 382 - 8 * math.sin((x - 100) / 260) - 20 * math.exp(-((x - 430) ^ 2) / 18000)
       - 14 * math.exp(-((x - 700) ^ 2) / 12000) - 26 * math.exp(-((x - 900) ^ 2) / 9000)
  return base + 8 * n3:at01(x * 2.8, 0.15) - 10
end
mwood = mask(function(x, y) return (y > topw(x) and y < 430) and 1 or 0 end)
mwood = mwood * rect(0, 300, 1000, 145)
work(mwood, {hand="hatch", pile=p_wood, angle=-1.05, coverage=2.0, edge={found=0.45, soft=0.35, lost=0.2}, clip=true, seed=151})
work(mwood, {hand="hatch", pile=p_wood, angle=-0.9, coverage=1.2, edge={found=0.35, soft=0.45, lost=0.2}, clip=true, seed=152})
print("treeline repaid")

--@ chunk 117

-- the dark bank mass under the far water: restate its plane with broad
-- wet strokes, cooler than the wood (water reflects open sky behind it)
p_bankd = pile{{"bone black", 0.6}, {"raw umber", 0.8}, {"cobalt blue", 0.55}, {"lead white", 0.3}, medium=0.5}
mb = rect(0, 418, 1000, 44):soften(12)
work(mb, {hand="broad", pile=p_bankd, angle=0.01, coverage=1.3, edge={soft=0.5, lost=0.5}, seed=161})
-- deep shadow line where shadow-water meets the bank
p_deep = pile{{"bone black", 1.1}, {"raw umber", 0.8}, {"cobalt blue", 0.5}, medium=0.4}
mdeep = ribbon({{0, 432}, {250, 436}, {480, 434}, {700, 431}, {1000, 428}}, 16)
work(mdeep:soften(6), {hand="body", pile=p_deep, angle=0.01, coverage=1.4, edge={lost=0.6}, seed=162})
print("bank and edge deepened")

--@ chunk 118
print(wait(150)); local x,y = 500, 360; print(drying(x, y))

--@ chunk 119
print(wait(480)); print("shore: ", drying(300, 400), " haze: ", drying(450, 300))

--@ chunk 120
print(wait(900)); print("shore: ", drying(300, 400), " haze: ", drying(450, 300), " water: ", drying(300, 600))

--@ chunk 121
print(wait(2400)); print("shore: ", drying(300, 400), " haze: ", drying(450, 300))

--@ chunk 122
print(wait(7200)); print("shore: ", drying(300, 400), " haze: ", drying(450, 300))

--@ chunk 123

-- the wood, laid dry on dry: stiffer pile, less medium, in two passes
p_wood = pile{{"bone black", 0.85}, {"raw umber", 0.85}, {"green earth", 0.55}, {"cobalt blue", 0.45}, {"lead white", 0.2}, medium=0.25}
n3 = noise{seed=37, octaves=4, period=70, persistence=0.55}
topw = function(x)
  local base = 382 - 8 * math.sin((x - 100) / 260) - 20 * math.exp(-((x - 430) ^ 2) / 18000)
       - 14 * math.exp(-((x - 700) ^ 2) / 12000) - 26 * math.exp(-((x - 900) ^ 2) / 9000)
  return base + 8 * n3:at01(x * 2.8, 0.15) - 10
end
mwood = mask(function(x, y) return (y > topw(x) and y < 430) and 1 or 0 end)
mwood = mwood * rect(0, 300, 1000, 145)
work(mwood, {hand="hatch", pile=p_wood, angle=-1.05, coverage=2.2, edge={found=0.45, soft=0.35, lost=0.2}, clip=true, seed=171})
work(mwood, {hand="hatch", pile=p_wood, angle=-0.9, coverage=1.6, edge={found=0.35, soft=0.45, lost=0.2}, clip=true, seed=172})
print("wood laid on dry")

--@ chunk 124

-- the misty underbank: quiet grey planes through the strip between
-- the wood and the far water, left side lighter (mist pooling)
p_under = pile{{"lead white", 2.4}, {"raw umber", 1.1}, {"cobalt blue", 0.75}, {"red earth", 0.2}, medium=0.55}
p_underpale = pile{{"lead white", 3.6}, {"raw umber", 0.8}, {"cobalt blue", 0.7}, {"yellow ochre", 0.2}, medium=0.7}
mu = mask(function(x, y) return (y > 424 and y < 462) and 1 or 0 end)
work(mu, {hand="broad", pile=p_under, angle=0.01, coverage=1.7, edge={soft=0.5, lost=0.5}, seed=181})
work(mu, {hand="broad", pile=p_underpale, angle=0.02, coverage=0.8, edge={soft=0.5, lost=0.6}, seed=182})
blend(mu:soften(12), {angle=0.02, seed=183})
-- deepen its foot where it meets the shadow water
p_footd = pile{{"raw umber", 1.0}, {"bone black", 0.5}, {"cobalt blue", 0.65}, {"lead white", 0.5}, medium=0.45}
mfd = ribbon({{0, 452}, {220, 456}, {440, 452}, {640, 448}, {1000, 444}}, 20)
work(mfd:soften(8), {hand="body", pile=p_footd, angle=0.01, coverage=1.3, edge={lost=0.6}, seed=184})
print("underbank quieted")

--@ chunk 125

-- repeat a smooth cool grey over the strip so the brick pattern vanishes
p_flat1 = pile{{"lead white", 2.0}, {"raw umber", 1.25}, {"cobalt blue", 1.05}, {"red earth", 0.25}, {"bone black", 0.3}, medium=0.6}
m_strip = ribbon({{0, 444}, {180, 447}, {400, 445}, {640, 441}, {1000, 438}}, 30)
work(m_strip, {hand="broad", pile=p_flat1, angle=0.01, coverage=1.9, ruler=true, edge={soft=0.4, lost=0.6}, seed=191})
work(m_strip, {hand="broad", pile=p_flat1, angle=0.02, coverage=1.2, ruler=true, edge={soft=0.5, lost=0.5}, seed=192})
blend(m_strip:soften(14), {angle=0.01, seed=193})
print("strip smoothed")

--@ chunk 126

-- warm the strip: a glaze of dun over the grey, left end lightest (mist)
p_dun = pile{{"lead white", 3.2}, {"yellow ochre", 1.5}, {"raw umber", 0.7}, {"red earth", 0.25}, medium=0.75}
strip = rect(0, 426, 1000, 40)
work(strip, {hand="glaze", pile=p_dun, coverage=0.7, angle=0.01, edge={lost=0.8}, seed=201})
work(strip, {hand="glaze", pile=p_dun, coverage=0.5, angle=0.01, edge={lost=0.9}, seed=202})
-- lighten the far left end: mist pooling against the headland
p_pool = pile{{"lead white", 4.5}, {"yellow ochre", 0.6}, {"raw umber", 0.4}, {"pale smalt", 0.6}, medium=0.65}
mpool = mask(function(x, y) return (x < 240 and y > 420 and y < 462) and clamp(smoothstep(240, 60, x), 0, 1) or 0 end)
work(mpool, {hand="glaze", pile=p_pool, coverage=1.2, edge={lost=0.7}, seed=203})
print("strip warmed")

--@ chunk 127

-- misty far water: pale grey-blue, shallow gradation down from the strip
p_fw = pile{{"lead white", 2.9}, {"pale smalt", 1.0}, {"raw umber", 0.55}, {"yellow ochre", 0.15}, medium=0.75}
fw = rect(0, 452, 1000, 34)
work(fw, {hand="broad", pile=p_fw, angle=0.01, coverage=0.75, edge={lost=0.9, soft=0.4}, load=0.4, seed=211})
work(fw, {hand="glaze", pile=p_fw, coverage=0.4, edge={lost=0.95}, seed=212})
blend(fw:soften(16), {angle=0.0, seed=213})
print("far water pale")

--@ chunk 128

-- restate the shore-edge trees on top: drawn clumps again, ON the settled
-- warm strip, dark grey-green, small and clear
p_c2 = pile{{"bone black", 0.7}, {"raw umber", 1.0}, {"green earth", 0.5}, {"lead white", 0.3}, {"yellow ochre", 0.2}, medium=0.3}
b2 = brush{kind="round", width=5, point=0.75, stiffness=0.6}
b2:load(p_c2, 0.9)
local specs = {
  {95, 396, 30}, {118, 402, 24}, {142, 398, 27}, {168, 404, 20},
  {262, 400, 22}, {288, 396, 26}, {300, 405, 16},
  {330, 399, 24}, {352, 403, 18}, {376, 398, 22},
  {415, 401, 18}, {432, 405, 14}, {452, 399, 20}, {476, 404, 15}, {494, 400, 22},
  {520, 402, 17}, {538, 406, 13}, {556, 400, 19}, {578, 404, 14}, {596, 401, 18},
  {626, 403, 16}, {648, 399, 20}, 
  {688, 402, 15}, {702, 405, 12}, {718, 400, 17}, {738, 404, 13}, {756, 401, 16},
  {788, 403, 14}, {804, 406, 11}, {822, 402, 15}, {844, 405, 12}, {862, 401, 17}, {884, 404, 13},
  {912, 403, 15}, {932, 400, 18}, {950, 405, 13}, {968, 402, 16}, {986, 406, 11}
}
for _, s in ipairs(specs) do
  local x, h = s[1], s[3]
  local sw = rand(2, 5)
  b2:stroke({{x + rand(-2, 2), 420 + rand(-2, 2)}, {x + rand(-3, 3), 420 - h * 0.55}, {x + rand(-4, 4), 420 - h}},
    {pressure={0.6, 0.2}, ramps={0.2, 0.5}})
end
print("shore clumps struck")

--@ chunk 129

-- clumps of trees at the left, where the headland meets the mist:
-- a descending line of dark trees standing IN the pale water margin
p_c3 = pile{{"bone black", 0.65}, {"raw umber", 0.95}, {"green earth", 0.5}, {"lead white", 0.3}, medium=0.3}
b3 = brush{kind="round", width=4.5, point=0.8, stiffness=0.6}
b3:load(p_c3, 0.92)
-- base rises from water at left toward the bank at right
local specs = {}
local xs = uneven(10, 20, 220, 0.6, 0.35, 55)
for _, x in ipairs(xs) do
  local base = 424 - 0.055 * x + rand(-2, 3)
  local h = rand(12, 30) + (x < 90 and 8 or 0)
  table.insert(specs, {x, base, h})
end
-- one taller tree for a mark
table.insert(specs, {150, 400, 34})
for _, s in ipairs(specs) do
  local x, base, h = s[1], s[2], s[3]
  b3:stroke({{x, base}, {x + rand(-3, 3), base - h * 0.6}, {x + rand(-4, 4), base - h}},
    {pressure={0.65, 0.2}, ramps={0.2, 0.5}})
end
print("left clumps struck")

--@ chunk 130

p_light = pile{{"lead white", 5}, {"yellow ochre", 1.2}, medium=0.5}
p_hull = pile{{"bone black", 0.9}, {"raw umber", 0.8}, {"vermilion", 0.3}, {"lead white", 0.55}, medium=0.45}
hull = poly({{-2, 960}, {100, 942}, {200, 946}, {400, 938}, {590, 946}, {670, 958}, {660, 976}, {520, 968}, {380, 972}, {240, 968}, {140, 970}, {60, 976}, {0, 968}}, true)
work(hull, {hand="hatch", pile=p_hull, angle=0.5, coverage=1.4, edge={found=0.5, soft=0.5}, clip=true, seed=231})
sheer = ribbon({{10, 952}, {150, 944}, {320, 948}, {520, 941}, {640, 949}}, 9)
work(sheer, {hand="body", pile=p_light, angle=0.1, coverage=1.3, edge={soft=0.4, lost=0.6}, clip=true, seed=232})
print("hull restated")

--@ chunk 131

-- cool the light-path: grey-violet water color mixed from the water tubes,
-- laid over the whole path zone, heavier on its cold west flank
p_violet = pile{{"lead white", 2.2}, {"cobalt blue", 1.05}, {"raw umber", 0.85}, {"red earth", 0.3}, {"bone black", 0.35}, medium=0.6}
zone = rect(460, 430, 400, 520)
work(zone, {hand="broad", pile=p_violet, angle=0.05, coverage=1.5, edge={lost=0.8}, seed=241})
work(zone, {hand="broad", pile=p_violet, angle=0.02, coverage=1.0, edge={lost=0.9}, seed=242})
blend(zone:soften(30), {angle=0.05, seed=243})
print("path cooled")

--@ chunk 132

-- restate the main reflection: a warm amber column again, narrower (about
-- 70 wide), exact strokes down the cool zone, plus a few upstream nicks
p_g2 = pile{{"lead white", 3.2}, {"chrome yellow", 1.5}, {"vermilion", 0.6}, medium=0.45}
col = ribbon({{640, 440}, {628, 500}, {614, 560}, {604, 570}, {596, 560}, {588, 640}, {580, 700}, {576, 750}, {570, 810}, {562, 880}, {558, 920}, {566, 915}, {572, 840}, {578, 760}, {586, 690}, {594, 620}, {602, 550}, {616, 490}}, 26)
work(col, {hand="body", pile=p_g2, angle=1.35, coverage=1.2, edge={lost=0.7, soft=0.3}, seed=251})
work(col, {hand="body", pile=p_g2, angle=1.3, coverage=0.7, edge={lost=0.8}, seed=252})
print("column restated")

--@ chunk 133

-- downstream darks to counterweight: two dark washes left of the boat
p_dl = pile{{"cobalt blue", 0.95}, {"raw umber", 0.9}, {"bone black", 0.55}, {"lead white", 0.5}, {"vermilion", 0.1}, medium=0.5}
md1 = mask(function(x, y) return (y > 640 and y < 800 and x > 320 and x < 520) and smoothstep(520, 380, x) * 0.9 or 0 end)
work(md1, {hand="broad", pile=p_dl, angle=1.4, coverage=1.2, edge={lost=0.8}, seed=261})
md2 = mask(function(x, y) return (y > 800 and y < 900 and x > 200 and x < 460) and smoothstep(460, 340, x) * 0.7 or 0 end)
work(md2, {hand="broad", pile=p_dl, angle=1.4, coverage=0.9, edge={lost=0.9}, seed=262})
print("downstream darks laid")

--@ chunk 134

-- float cool water colour over the feather streaks, then blend the whole
-- zone: they must dissolve into everything around them
p_vw = pile{{"lead white", 2.2}, {"cobalt blue", 1.1}, {"raw umber", 0.8}, {"red earth", 0.2}, medium=0.6}
mz = rect(180, 460, 340, 310)
work(mz, {hand="broad", pile=p_vw, angle=1.45, coverage=1.5, edge={lost=0.85}, seed=271})
work(mz, {hand="broad", pile=p_vw, angle=1.5, coverage=1.0, edge={lost=0.9}, seed=272})
blend(mz:soften(40), {angle=1.45, seed=273})
blend(mz:soften(40), {angle=1.5, seed=274})
print("streaks floated and blended")

--@ chunk 135

-- plane the whole mid-water: one grey passage, mixed horizontally,
-- covering the streak curtain, feathered into everything
p_plane = pile{{"lead white", 2.0}, {"cobalt blue", 0.9}, {"raw umber", 0.75}, {"red earth", 0.25}, {"bone black", 0.3}, medium=0.65}
plane = mask(function(x, y) return (y > 465 and y < 880 and x > 150 and x < 780) and 1 or 0 end)
work(plane, {hand="broad", pile=p_plane, angle=0.02, coverage=1.5, ruler=true, edge={soft=0.6, lost=0.4}, seed=281})
work(plane, {hand="broad", pile=p_plane, angle=0.01, coverage=1.2, ruler=true, edge={soft=0.6, lost=0.4}, seed=282})
blend(plane:soften(30), {angle=0.02, seed=283})
print("plane laid")

--@ chunk 136

blend(plane:soften(40), {angle=0.0, seed=284})
blend(plane:soften(50), {angle=0.01, seed=285})
print(wait(20))

--@ chunk 137

-- the vertical passing warm reflection of the bright western sky剂的
p_refl = pile{{"lead white", 3.0}, {"yellow ochre", 1.2}, {"vermilion", 0.35}, medium=0.55}
rb = mask(function(x, y) return (y > 430 and y < 880) and 1 or 0 end)
rr = ribbon({{560, 440}, {568, 520}, {576, 600}, {570, 680}, {562, 680}, {584, 680}, {590, 700}, {584, 780}, {576, 780}, {608, 770}, {614, 840}, {606, 840}, {602, 872}}, 20)
work(rr * rb, {hand="body", pile=p_refl, angle=1.45, coverage=1.1, edge={lost=0.75, soft=0.3}, seed=291})
work(rr * rb, {hand="body", pile=p_refl, angle=1.5, coverage=0.7, edge={lost=0.85}, seed=292})
print("reflection laid")

--@ chunk 138

-- float dry pale strokes into the column: zigzag em
p_amb = pile{{"lead white", 2.5}, {"yellow ochre", 2.0}, {"vermilion", 0.4}, medium=0.5}
colsx = uneven(19, 490, 640, 0.8, 0.5, 9)
for i, x in ipairs(colsx) do
  local base = 428 + (i - 1) * (860 - 440) / (#colsx - 1) + rand(-14, 14)
  local w = rand(7, 13)
  local b_a = brush{kind="filbert", width=w, point=0.2, stiffness=0.45}
  b_a:load(p_amb, rand(0.5, 0.75))
  b_a:stroke({{x, base}, {x + rand(-8, 8), base - 11 + rand(-9, 9)},
              {x + rand(-14, 14), base + rand(2, 12)}},
    {pressure={0.62, 0.45}, ramps={0.25, 0.3}, orient=1.44 + rand(-0.12, 0.12)})
end
print("column chips laid")

--@ chunk 139

-- deepen the water colour overall, two glazes over the whole water so the
-- palette settles back to rich violets and greys
p_bath = pile{{"cobalt blue", 1.1}, {"raw umber", 0.9}, {"bone black", 0.45}, {"lead white", 1.1}, {"vermilion", 0.08}, medium=0.85}
mwater = rect(0, 440, 1000, 480)
work(mwater, {hand="glaze", pile=p_bath, coverage=0.75, edge={lost=0.9}, angle=0.02, seed=301})
work(mwater, {hand="glaze", pile=p_bath, coverage=0.55, edge={lost=0.95}, angle=0.02, seed=302})
print("water bathed")

--@ chunk 140

-- the boat must return as the picture’s human note: a dark little hull
-- beside the light-path, riding on the plane, small but Buoyant
p_boat = pile{{"bone black", 1.0}, {"raw umber", 0.7}, {"vermilion", 0.25}, {"lead white", 0.3}, medium=0.4}
hull2 = poly({{690, 655}, {760, 650}, {820, 658}, {830, 668}, {790, 676}, {700, 670}, {685, 662}}, true)
work(hull2, {hand="hatch", pile=p_boat, angle=0.35, coverage=1.5, edge={found=0.4, soft=0.4}, clip=true, seed=311})
-- a tiny red flag at its mast: a stroke of vermilion, then the mast itself
p_mast = pile{{"bone black", 1.2}, {"raw umber", 0.7}, medium=0.4}
mb = brush{kind="round", width=1.6, point=1, stiffness=0.7}
mb:load(p_mast, 0.9)
mb:stroke({{742, 655}, {738, 575}}, {pressure={0.5, 0.35}, ramps={0.2, 0.3}})
p_flag = pile{{"vermilion", 1.4}, {"bone black", 0.4}, {"lead white", 0.25}, medium=0.35}
mf = brush{kind="round", width=3.2, point=0.6, stiffness=0.55}
mf:load(p_flag, 0.85)
mf:stroke({{738, 580}, {748, 586}, {744, 592}}, {pressure={0.55, 0.2}, ramps={0.2, 0.6}})
print("boat returned")

--@ chunk 141

-- hull repaint, filling straight through the oval: bow left, stern right
p_h3 = pile{{"bone black", 0.95}, {"raw umber", 0.75}, {"vermilion", 0.3}, {"lead white", 0.2}, medium=0.4}
h3 = poly({{676, 662}, {700, 652}, {740, 647}, {780, 648}, {818, 660}, {836, 671}, {830, 681}, {780, 683}, {720, 680}, {690, 674}, {672, 668}}, true)
work(h3, {hand="hatch", pile=p_h3, angle=0.4, coverage=2.0, edge={found=0.35, soft=0.4}, clip=true, seed=321})
work(h3, {hand="body", pile=p_h3, angle=0.3, coverage=1.2, edge={found=0.35, soft=0.5}, clip=true, seed=322})
-- tiny red flag
p_flag = pile{{"vermilion", 1.5}, {"bone black", 0.3}, {"lead white", 0.2}, medium=0.35}
mf = brush{kind="round", width=2.6, point=0.7, stiffness=0.55}
mf:load(p_flag, 0.9)
mf:stroke({{741, 562}, {750, 570}, {745, 580}}, {pressure={0.5, 0.15}, ramps={0.2, 0.6}})
print("hull solid")

--@ chunk 142

-- floats out to the dry mud spit by the shore; a grey cape over the hull
p_rh = pile{{"cobalt blue", 1.0}, {"raw umber", 0.95}, {"bone black", 0.5}, {"lead white", 1.2}, medium=0.5}
rh = ellipse(748, 670, 105, 28)
work(rh, {hand="broad", pile=p_rh, angle=0.01, coverage=1.6, edge={lost=0.7, soft=0.4}, seed=331})
blend(rh:soften(16), {angle=0.01, seed=332})
-- then cut the dark hull back out of that grey, crisp at its top line:
h4 = poly({{686, 668}, {710, 659}, {744, 655}, {776, 654}, {806, 660}, {822, 668}, {816, 676}, {780, 679}, {724, 678}, {694, 674}, {678, 671}}, true)
p_h4 = pile{{"bone black", 1.05}, {"raw umber", 0.8}, {"vermilion", 0.3}, {"lead white", 0.2}, medium=0.35}
work(h4, {hand="hatch", pile=p_h4, angle=0.4, coverage=2.2, edge={found=0.4, soft=0.4}, clip=true, seed=333})
work(h4, {hand="detail", pile=p_h4, angle=0.35, coverage=1.3, edge={found=0.4, soft=0.4}, clip=true, seed=334})
print("hull relearned")

--@ chunk 143

-- soften the halo round the hull into the water: cool scumble around it
p_halo = pile{{"lead white", 1.6}, {"cobalt blue", 0.55}, {"raw umber", 0.45}, medium=0.7}
mh = ellipse(750, 668, 150, 45) - poly({{666, 690}, {848, 690}, {848, 640}, {666, 640}}, true)
work(mh, {hand="scumble", pile=p_halo, angle=0.02, coverage=0.9, load=0.3, edge={lost=1}, seed=341})
-- reflection of hull and flag in the water: a broken dark streak under it
p_ref = pile{{"bone black", 0.8}, {"cobalt blue", 0.7}, {"raw umber", 0.6}, {"lead white", 0.3}, medium=0.5}
refl = ribbon({{736, 682}, {742, 700}, {748, 720}, {744, 700}, {738, 686}}, 7)
work(refl, {hand="body", pile=p_ref, angle=1.5, coverage=0.9, edge={lost=0.8}, seed=342})
print("halo and reflection")

--@ chunk 144

-- wipe the white lagoon: cool grey wash over the pale ring and blend it away
p_lag = pile{{"lead white", 1.1}, {"cobalt blue", 0.85}, {"raw umber", 0.7}, {"bone black", 0.25}, medium=0.7}
ml = ellipse(750, 668, 140, 42)
work(ml, {hand="broad", pile=p_lag, angle=0.02, coverage=1.6, ruler=true, edge={lost=0.75, soft=0.3}, seed=351})
work(ml, {hand="broad", pile=p_lag, angle=0.01, coverage=1.2, ruler=true, edge={lost=0.8, soft=0.4}, seed=352})
-- blend BUT only outside the boat: two半 halves, staying clear of the dark
blend(ellipse(750, 668, 140, 42) - poly({{660, 650}, {860, 640}, {860, 610}, {660, 610}}, true):soften(10), {angle=0.01, seed=353})
print("lagoon wiped")

--@ chunk 145

-- full grey bath over the boat zone, generous margins, opaque enough to
-- bury every trace; the boat then gets restated freshly on top
p_b2 = pile{{"lead white", 1.4}, {"cobalt blue", 1.0}, {"raw umber", 0.8}, {"red earth", 0.15}, {"bone black", 0.35}, medium=0.65}
zone2 = rect(600, 570, 360, 190)
work(zone2, {hand="broad", pile=p_b2, angle=0.02, coverage=2.2, ruler=true, edge={lost=0.7, soft=0.3}, seed=361})
work(zone2, {hand="broad", pile=p_b2, angle=0.01, coverage=1.6, ruler=true, edge={lost=0.8, soft=0.3}, seed=362})
blend(zone2:soften(30), {angle=0.02, seed=363})
blend(zone2:soften(30), {angle=0.0, seed=364})
print("bath laid")

--@ chunk 146

blend(zone2:soften(40), {angle=0.02, seed=365})
-- little wake ripples behind the bath for the water to read wet
p_wk = pile{{"lead white", 1.3}, {"cobalt blue", 0.5}, {"raw umber", 0.4}, medium=0.75}
wkl = ribbon({{590, 730}, {640, 733}, {700, 731}}, 5)
work(wkl, {hand="body", pile=p_wk, angle=0.0, coverage=0.7, load=0.3, edge={lost=0.9}, seed=371})
print(wait(15))

--@ chunk 147

-- the boat at anchor said once and whole: one sitting, no repairs after
p_hull = pile{{"bone black", 1.0}, {"raw umber", 0.85}, {"vermilion", 0.35}, {"lead white", 0.25}, medium=0.4}
h5 = poly({{696, 636}, {724, 626}, {766, 622}, {800, 630}, {820, 642}, {812, 652}, {776, 654}, {728, 651}, {700, 645}, {690, 640}}, true)
work(h5, {hand="hatch", pile=p_hull, angle=0.35, coverage=2.0, edge={found=0.4, soft=0.4}, clip=true, seed=401})
work(h5, {hand="body", pile=p_hull, angle=0.3, coverage=1.0, edge={found=0.4, soft=0.5}, clip=true, seed=402})
-- mast: one dark nick
bm = brush{kind="round", width=1.4, point=1, stiffness=0.7}
p_mast = pile{{"bone black", 1.3}, {"raw umber", 0.7}, medium=0.4}
bm:load(p_mast, 0.85)
bm:stroke({{742, 626}, {740, 568}}, {pressure={0.45, 0.3}, ramps={0.2, 0.35}})
-- flag: one vermilion lick
bf = brush{kind="rigger", width=2.0, point=1, stiffness=0.5}
p_flag = pile{{"vermilion", 1.5}, {"bone black", 0.35}, {"lead white", 0.2}, medium=0.3}
bf:load(p_flag, 0.85)
bf:stroke({{741, 574}, {750, 580}, {746, 588}}, {pressure={0.5, 0.12}, ramps={0.15, 0.55}})
print("boat final")

--@ chunk 148

-- anchor chain glinting beneath the boat: 3 faint pale strokes
bp = brush{kind="rigger", width=1.2, point=1, stiffness=0.6}
p_chain = pile{{"lead white", 1.6}, {"yellow ochre", 0.5}, medium=0.7}
bp:load(p_chain, 0.5)
bp:stroke({{744, 652}, {746, 664}, {742, 672}}, {pressure={0.4, 0.1}, ramps={0.3, 0.5}})
bp:wipe(0.6); bp:load(p_chain, 0.35)
bp:stroke({{754, 650}, {758, 660}, {754, 666}}, {pressure={0.3, 0.05}, ramps={0.3, 0.6}})
print("chain glints")

--@ chunk 149

-- the hull reads too brown and blobby; set it in shadow-grey first
p_knock = pile{{"lead white", 1.5}, {"cobalt blue", 0.7}, {"raw umber", 0.8}, {"bone black", 0.4}, medium=0.6}
kn = ellipse(752, 638, 92, 26)
work(kn, {hand="scumble", pile=p_knock, angle=0.0, coverage=1.2, load=0.45, edge={lost=0.7, soft=0.3}, seed=411})
-- then the true waterline shadow beneath: a grounded dark with a wave breaking over it
p_dk = pile{{"bone black", 1.15}, {"raw umber", 0.8}, {"cobalt blue", 0.55}, medium=0.45}
wl = ribbon({{690, 648}, {720, 650}, {760, 651}, {790, 649}, {815, 645}}, 18)
work(wl:soften(6), {hand="body", pile=p_dk, angle=0.05, coverage=1.5, edge={soft=0.5, lost=0.5}, seed=412})
print("hull knocked in shadow")

--@ chunk 150

-- open grey over the entire spot, everything this time; then wait for it
-- to SET (not dry) before the boat stroke so it takes cleanly
p_gb = pile{{"lead white", 1.3}, {"cobalt blue", 0.75}, {"raw umber", 0.65}, {"red earth", 0.2}, {"bone black", 0.35}, medium=0.6}
gb = rect(620, 580, 320, 140)
work(gb, {hand="broad", pile=p_gb, angle=0.02, coverage=2.0, ruler=true, edge={lost=0.7, soft=0.3}, seed=421})
work(gb, {hand="broad", pile=p_gb, angle=0.01, coverage=1.5, ruler=true, edge={lost=0.8, soft=0.3}, seed=422})
blend(gb:soften(25), {angle=0.02, seed=423})
blend(gb:soften(35), {angle=0.01, seed=424})
print("grey bath generous")

--@ chunk 151
print(wait(210)); print("bath: ", drying(760, 650))

--@ chunk 152
print(wait(420)); print("bath: ", drying(760, 650))

--@ chunk 153

-- the boat in one clean statement on set ground, no repairs
p_hull = pile{{"bone black", 1.05}, {"raw umber", 0.85}, {"vermilion", 0.3}, {"lead white", 0.25}, medium=0.42}
h6 = poly({{700, 632}, {728, 622}, {768, 618}, {800, 626}, {818, 638}, {808, 646}, {772, 648}, {730, 646}, {706, 641}, {696, 636}}, true)
work(h6, {hand="hatch", pile=p_hull, angle=0.35, coverage=2.2, edge={found=0.4, soft=0.35}, clip=true, seed=431})
work(h6, {hand="body", pile=p_hull, angle=0.3, coverage=1.2, edge={found=0.4, soft=0.45}, clip=true, seed=432})
-- green reflection under the hull: water weed shadow, low note of the light
p_wd = pile{{"green earth", 0.55}, {"cobalt blue", 0.5}, {"bone black", 0.45}, {"lead white", 0.4}, medium=0.55}
wd = ribbon({{712, 650}, {742, 652}, {770, 651}, {794, 649}, {806, 652}}, 23)
work(wd:soften(7), {hand="body", pile=p_wd, angle=0.02, coverage=1.2, edge={lost=0.6}, seed=433})
print("boat final, on set ground")

--@ chunk 154

-- The final bath: big, cool, opaque, generous; every ghost buried
p_b3 = pile{{"lead white", 1.2}, {"cobalt blue", 1.05}, {"raw umber", 0.85}, {"red earth", 0.22}, {"bone black", 0.4}, medium=0.55}
big = rect(580, 550, 420, 230)
work(big, {hand="broad", pile=p_b3, angle=0.02, coverage=2.5, ruler=true, edge={lost=0.6, soft=0.3}, seed=441})
work(big, {hand="broad", pile=p_b3, angle=0.01, coverage=1.8, ruler=true, edge={lost=0.7, soft=0.25}, seed=442})
blend(big:soften(28), {angle=0.02, seed=443})
blend(big:soften(28), {angle=0.0, seed=444})
blend(big:soften(35), {angle=0.01, seed=445})
print("final bath")

--@ chunk 155
print(wait(600)); print("bath: ", drying(790, 665))

--@ chunk 156
print(wait(2400)); print("bath: ", drying(790, 665))

--@ chunk 157
print(wait(4800)); print("bath: ", drying(790, 665))

--@ chunk 158

-- THE boat, one pass, on dry ground: a dark moored yacht at rest,
-- hull + deck + mast + furled sail, drawn as a shape by hand
p_hull = pile{{"bone black", 1.1}, {"raw umber", 0.85}, {"cobalt blue", 0.35}, {"vermilion", 0.3}, {"lead white", 0.25}, medium=0.42}
h7 = poly({{702, 634}, {730, 624}, {770, 620}, {800, 628}, {818, 640}, {806, 650}, {772, 652}, {732, 650}, {708, 645}, {698, 639}}, true)
work(h6, {hand="hatch", pile=p_hull, angle=0.35, coverage=2.4, edge={found=0.4, soft=0.35}, clip=true, seed=451})
work(h6, {hand="body", pile=p_hull, angle=0.3, coverage=1.4, edge={found=0.4, soft=0.45}, clip=true, seed=452})
print("boat, one pass")

--@ chunk 159

-- the boat comes out: a final generous bath over the whole zone
p_b4 = pile{{"lead white", 1.2}, {"cobalt blue", 1.05}, {"raw umber", 0.9}, {"red earth", 0.2}, {"bone black", 0.45}, medium=0.55}
big2 = rect(570, 545, 450, 250)
work(big2, {hand="broad", pile=p_b3, angle=0.02, coverage=2.4, ruler=true, edge={lost=0.6, soft=0.3}, seed=461})
work(big2, {hand="broad", pile=p_b3, angle=0.01, coverage=1.7, ruler=true, edge={lost=0.7, soft=0.25}, seed=462})
blend(big2:soften(30), {angle=0.02, seed=463})
blend(big2:soften(30), {angle=0.0, seed=464})
print("boat out")

--@ chunk 160

-- re-knit the right water: the local cool grey over the ghost and brown
p_knit = pile{{"lead white", 1.0}, {"cobalt blue", 1.1}, {"raw umber", 0.85}, {"bone black", 0.3}, medium=0.7}
kk = rect(430, 430, 570, 300)
work(kk, {hand="glaze", pile=p_knit, coverage=0.85, edge={lost=0.9}, angle=0.02, seed=471})
work(kk, {hand="glaze", pile=p_knit, coverage=0.6, edge={lost=0.95}, angle=0.02, seed=472})
print("right water knitting")

--@ chunk 161

-- the mast of the moored yacht, standing at the bank edge: one dark line,
-- and its faint reflection, the last human note in the picture
bm = brush{kind="round", width=1.6, point=1, stiffness=0.7}
p_m = pile{{"bone black", 1.2}, {"raw umber", 0.7}, {"cobalt blue", 0.3}, medium=0.4}
bm:load(p_m, 0.9)
bm:stroke({{448, 770}, {446, 668}}, {pressure={0.5, 0.32}, ramps={0.2, 0.3}})
-- the furled sail: a short pale wedge along it
bs = brush{kind="round", width=6.5, point=0.3, stiffness=0.55}
p_sail = pile{{"lead white", 2.4}, {"raw umber", 0.55}, {"pale smalt", 0.5}, medium=0.55}
bs:load(p_sail, 0.6)
bs:stroke({{448, 748}, {452, 716}}, {pressure={0.6, 0.4}, ramps={0.3, 0.4}})
print("mast and furled sail")

--@ chunk 162

-- the near spit: soften its hard pale crest into the water with a damp glaze
p_spitsoft = pile{{"lead white", 1.6}, {"pale smalt", 0.8}, {"raw umber", 0.6}, medium=0.8}
sp = ribbon({{60, 776}, {220, 772}, {380, 774}, {520, 780}}, 34)
work(sp, {hand="glaze", pile=p_spitsoft, coverage=0.5, edge={lost=0.95}, angle=0.01, seed=481})
-- birds: a loose skein rising from the shore, dark nicks against the sky
bb = brush{kind="rigger", width=1.4, point=1, stiffness=0.6}
p_bird = pile{{"bone black", 1.0}, {"raw umber", 0.6}, medium=0.5}
local birds = {{330, 305, 9}, {352, 292, 11}, {376, 300, 8}, {402, 284, 10}, {430, 296, 7}}
for _, b in ipairs(birds) do
  local x, y, s = b[1], b[2], b[3]
  bb:load(p_bird, 0.6)
  bb:stroke({{x, y}, {x - s, y - s * 0.55}}, {pressure={0.55, 0.12}, ramps={0.25, 0.5}})
  bb:stroke({{x, y}, {x + s * 0.9, y - s * 0.5}}, {pressure={0.5, 0.1}, ramps={0.25, 0.5}})
end
-- two lower, closer to the water
local low = {{700, 384, 12}, {724, 392, 9}}
for _, b in ipairs(low) do
  local x, y, s = b[1], b[2], b[3]
  bb:stroke({{x, y}, {x - s, y - s * 0.5}}, {pressure={0.5, 0.1}, ramps={0.25, 0.5}})
  bb:stroke({{x, y}, {x + s * 0.85, y - s * 0.45}}, {pressure={0.45, 0.08}, ramps={0.25, 0.5}})
end
print("birds and spit")

--@ chunk 163

-- varnish: a unifying glaze of warm evening light over the whole canvas,
-- thinnest over the sky, tying the pale and dark together at last
p_tone = pile{{"lead white", 0.8}, {"yellow ochre", 0.55}, {"vermilion", 0.12}, {"bone black", 0.1}, medium=0.92}
ev = everywhere()
work(ev, {hand="glaze", pile=p_tone, coverage=0.30, angle=0.05, edge=1, seed=491})
print("evening glaze on")

--@ chunk 164

-- the marks are wet: continue the film to continuity
work(ev, {hand="glaze", pile=p_tone, coverage=0.55, angle=0.05, edge=1, seed=492})
work(ev, {hand="glaze", pile=p_tone, coverage=0.45, angle=0.02, edge=1, seed=493})
print("film continued")

--@ chunk 165

-- test: fuse the marks in one small patch by blending
blend(rect(60, 560, 200, 140):soften(20), {angle=0.02, seed=501})
print("test blend")

--@ chunk 166

-- repay the dark treeline over the marked texture: dark body, firm
p_tl = pile{{"bone black", 1.0}, {"raw umber", 0.85}, {"green earth", 0.5}, {"cobalt blue", 0.4}, {"lead white", 0.25}, medium=0.45}
m1 = rect(0, 300, 1000, 105)
work(m1, {hand="broad", pile=p_tl, angle=-0.02, coverage=2.6, ruler=true, edge={found=0.3, soft=0.5}, seed=511})
work(m1, {hand="body", pile=p_tl, angle=-0.04, coverage=1.4, edge={found=0.35, soft=0.45}, seed=512})
-- its torn top edge: slight rise and fall, painted on
mtop = mask(function(x, y)
  local t = 372 - 14 * math.exp(-((x - 430) ^ 2) / 20000) - 10 * math.exp(-((x - 700) ^ 2) / 14000)
  return (y > t and y < 412) and 1 or 0
end)
work(mtop, {hand="hatch", pile=p_tl, angle=-0.9, coverage=1.8, edge={found=0.4, soft=0.4}, clip=true, seed=513})
print("treeline repaid")

--@ chunk 167

-- fill the valleys: stipple the same dark into the gaps between marks,
-- then a body pass, so the treeline evens out
m1 = rect(0, 300, 1000, 110)
stipple(m1, {pile=p_tl, width=9, coverage=1.6, pressure={0.8, 0.95}, seed=521})
work(m1, {hand="body", pile=p_tl, angle=-0.03, coverage=1.6, fill=true, edge={found=0.3, soft=0.5}, seed=522})
print("treeline filled")

--@ chunk 168

-- sky repair, zone by zone, filling between the marks
-- upper left cool violet-blue
p_ul = pile{{"lead white", 3.2}, {"cobalt blue", 1.5}, {"raw umber", 0.6}, {"pale smalt", 0.4}, medium=0.45}
m_ul = rect(0, 0, 240, 260)
work(m_ul, {hand="broad", pile=p_ul, angle=0.02, coverage=2.0, ruler=true, edge={lost=0.6, soft=0.3}, seed=531})
stipple(m_ul, {pile=p_ul, width=10, coverage=1.2, pressure={0.75, 0.9}, seed=532})
work(m_ul, {hand="body", pile=p_ul, angle=0.02, coverage=1.4, fill=true, edge={lost=0.7}, seed=533})
-- upper right blue-grey
p_ur = pile{{"lead white", 3.6}, {"cobalt blue", 1.35}, {"raw umber", 0.55}, medium=0.45}
m_ur = rect(620, 0, 380, 270)
work(m_ur, {hand="broad", pile=p_ur, angle=0.02, coverage=2.0, ruler=true, edge={lost=0.6, soft=0.3}, seed=541})
stipple(m_ur, {pile=p_ur, width=10, coverage=1.2, pressure={0.75, 0.9}, seed=542})
work(m_ur, {hand="body", pile=p_ur, angle=0.02, coverage=1.4, fill=true, edge={lost=0.7}, seed=543})
print("upper corners repaired")

--@ chunk 169

-- the amber band across, right of the cut: warm fog, filled
p_amb = pile{{"lead white", 2.6}, {"yellow ochre", 1.7}, {"raw umber", 0.5}, {"vermilion", 0.25}, medium=0.5}
m_a = rect(600, 150, 400, 125)
work(m_a, {hand="broad", pile=p_amb, angle=0.05, coverage=1.8, ruler=true, edge={lost=0.5, soft=0.3}, seed=551})
stipple(m_a, {pile=p_amb, width=10, coverage=1.2, pressure={0.75, 0.9}, seed=552})
work(m_a, {hand="body", pile=p_amb, angle=0.05, coverage=1.3, fill=true, edge={lost=0.6}, seed=553})
-- the gold core of the cloud, rebuilt
p_gold = pile{{"lead white", 4.5}, {"yellow ochre", 2.2}, {"vermilion", 0.25}, medium=0.45}
core = mask(function(x, y)
  if x < 290 or x > 660 then return 0 end
  local cy = 200 + 0.05 * (x - 290)
  local rx, ry = 160, 85
  local d = ((x - 440) / rx) ^ 2 + ((y - cy) / ry) ^ 2
  return clamp(smoothstep(1.2, 0.3, d) * (0.7 + 0.3 * math.sin(x / 23)), 0, 1)
end)
work(core, {hand="scumble", pile=p_gold, angle=0.15, coverage=1.3, load=0.5, edge={lost=0.85, soft=0.3}, seed=561})
stipple(core, {pile=p_gold, width=7, coverage=0.9, pressure={0.6, 0.8}, seed=562})
print("amber rebuilt")

--@ chunk 170

-- fuse the lobed core and melt its edges into the sky
blend(core:soften(45), {angle=0.15, seed=571})
blend(core:soften(45), {angle=0.05, seed=572})
-- sky-tone veil over the core rim so the boundary dissolves
p_veil = pile{{"lead white", 3.0}, {"cobalt blue", 1.0}, {"yellow ochre", 0.5}, {"raw umber", 0.4}, medium=0.6}
rim = core:rim(34, 24)
work(rim, {hand="scumble", pile=p_veil, angle=0.1, coverage=0.85, load=0.4, edge={lost=0.9, soft=0.3}, seed=573})
print("core fused")

--@ chunk 171

-- one generous fusion over the whole core and rim: the badger crosses
-- everything wet there and evens the film
blend(rect(240, 85, 480, 200):soften(40), {angle=0.08, seed=581})
blend(rect(240, 85, 480, 200):soften(40), {angle=0.02, seed=582})
blend(rect(240, 85, 480, 200):soften(40), {angle=0.12, seed=583})
print(wait(10))

--@ chunk 172

-- the right seam: transitional strokes straddling blue and amber
p_trans = pile{{"lead white", 2.8}, {"yellow ochre", 0.9}, {"cobalt blue", 0.7}, {"raw umber", 0.35}, medium=0.6}
m_s = mask(function(x, y) return (x > 560 and x < 720 and y > 60 and y < 250) and 1 or 0 end)
work(m_s, {hand="scumble", pile=p_trans, angle=1.5, coverage=1.1, load=0.45, edge={lost=0.85, soft=0.3}, seed=591})
blend(m_s:soften(30), {angle=1.55, seed=592})
print("seam knit")

--@ chunk 173

blend(rect(520, 30, 260, 200):soften(50), {angle=1.55, seed=601})
blend(rect(520, 30, 260, 200):soften(50), {angle=1.5, seed=602})
print(wait(8))

--@ chunk 174

-- small soft core rebuild with fill
p_gold = pile{{"lead white", 4.2}, {"yellow ochre", 2.0}, {"vermilion", 0.3}, medium=0.5}
core2 = mask(function(x, y)
  if x < 280 or x > 640 then return 0 end
  local cy = 195 + 0.04 * (x - 280)
  local rx, ry = 140, 72
  local d = ((x - 450) / rx) ^ 2 + ((y - cy) / ry) ^ 2
  local wob = 0.72 + 0.28 * math.sin(x / 26 + y / 40)
  return clamp(smoothstep(1.1, 0.25, d) * wob, 0, 1)
end)
work(core2, {hand="body", pile=p_gold, angle=0.15, coverage=1.5, fill=true, edge={lost=0.8, soft=0.3}, seed=611})
stipple(core2, {pile=p_gold, width=8, coverage=1.1, pressure={0.65, 0.85}, seed=612})
print("core restated")

--@ chunk 175

-- silver haze over the whole upper sky: pale cool grey with a whisper of
-- ochre, laid thin in crossing passes, then one blend
p_haze = pile{{"lead white", 4.0}, {"cobalt blue", 0.6}, {"yellow ochre", 0.5}, {"raw umber", 0.35}, {"bone black", 0.12}, medium=0.85}
sky = rect(0, 0, 1000, 300)
work(sky, {hand="glaze", pile=p_haze, coverage=0.5, angle=0.03, edge=1, seed=621})
work(sky, {hand="glaze", pile=p_haze, coverage=0.4, angle=0.01, edge=1, seed=622})
blend(sky, {angle=0.02, seed=623})
print("haze on")

--@ chunk 176

-- strengthen the light-path: a soft amber column back into the water
p_l2 = pile{{"lead white", 2.6}, {"yellow ochre", 1.6}, {"vermilion", 0.4}, medium=0.55}
lp = ribbon({{600, 320}, {590, 400}, {580, 480}, {572, 540}, {566, 590}, {560, 640}, {556, 690}, {552, 730}}, 30)
work(lp, {hand="body", pile=p_l2, angle=1.45, coverage=1.1, edge={lost=0.75, soft=0.3}, seed=631})
work(lp, {hand="body", pile=p_l2, angle=1.4, coverage=0.6, edge={lost=0.85}, seed=632})
-- bank darks: deepen the foreground mass at the right and the left wedge
p_bank = pile{{"bone black", 0.8}, {"raw umber", 0.95}, {"green earth", 0.35}, {"lead white", 0.3}, {"yellow ochre", 0.3}, medium=0.5}
mbr = mask(function(x, y)
  return (y > 930 - 0.5 * (x - 250) and x > 250) and 1 or 0
end)
work(mbr, {hand="broad", pile=p_bank, angle=-0.4, coverage=2.0, ruler=true, edge={soft=0.5, lost=0.3}, seed=641})
p_bv = pile{{"cobalt blue", 0.7}, {"raw umber", 0.85}, {"bone black", 0.5}, {"lead white", 0.45}, {"vermilion", 0.1}, medium=0.55}
mbl = mask(function(x, y) return (x < 200 and y > 800) and 1 or 0 end)
work(mbl, {hand="broad", pile=p_bv, angle=0.03, coverage=1.8, ruler=true, edge={soft=0.5, lost=0.3}, seed=642})
print("path and bank strengthened")

--@ chunk 177

-- the bank went ochre-lit; deepen it to near-dark with the evening shadow
p_bdk = pile{{"bone black", 1.05}, {"raw umber", 0.9}, {"green earth", 0.4}, {"cobalt blue", 0.3}, {"lead white", 0.25}, medium=0.45}
mbr = mask(function(x, y)
  return (y > 930 - 0.5 * (x - 250) and x > 250) and 1 or 0
end)
work(mbr, {hand="broad", pile=p_bdk, angle=-0.42, coverage=2.2, ruler=true, edge={soft=0.5, lost=0.3}, seed=651})
work(mbr, {hand="body", pile=p_bdk, angle=-0.45, coverage=1.5, fill=true, edge={soft=0.4, lost=0.4}, seed=652})
print("bank deepened")

--@ chunk 178

-- bank texture back over the dark: grass strokes and highlights, sparse
p_g = pile{{"lead white", 1.2}, {"yellow ochre", 1.6}, {"green earth", 0.8}, {"raw umber", 0.5}, medium=0.5}
gd = brush{kind="rigger", width=1.6, point=1, stiffness=0.5}
xs = uneven(26, 300, 990, 0.65, 0.4, 77)
for _, x in ipairs(xs) do
  local yb = 930 - 0.5 * (x - 250) + rand(4, 42)
  local h = rand(8, 22)
  if gd:fullness() < 0.25 then gd:load(p_g, 0.8) end
  gd:stroke({{x, yb}, {x + rand(-3, 6), yb - h}}, {pressure={0.55, 0.08}, ramps={0.2, 0.5}})
end
-- grass shadow side, darker strokes
p_gd = pile{{"bone black", 0.7}, {"raw umber", 1.0}, {"green earth", 0.5}, medium=0.45}
gg = brush{kind="rigger", width=1.8, point=1, stiffness=0.5}
xs2 = uneven(20, 320, 980, 0.6, 0.4, 78)
for _, x in ipairs(xs2) do
  local yb = 930 - 0.5 * (x - 250) + rand(10, 50)
  local h = rand(7, 18)
  if gg:fullness() < 0.25 then gg:load(p_gd, 0.85) end
  gg:stroke({{x, yb}, {x + rand(-4, 4), yb - h}}, {pressure={0.5, 0.08}, ramps={0.2, 0.5}})
end
print("grass struck")

--@ chunk 179

-- warm the water top-left with the reflection of the bank (its shadowed
-- side catches a little of the last light) - a soft olive wash
p_ol = pile{{"yellow ochre", 1.0}, {"green earth", 0.7}, {"raw umber", 0.7}, {"lead white", 1.6}, medium=0.75}
mo = rect(0, 350, 420, 180)
work(mo, {hand="glaze", pile=p_ol, coverage=0.45, edge=1, angle=0.01, seed=661})
print("olive reflection")

--@ chunk 180

-- dark ground under the warm dashes on the bank: a glaze of umber-green
-- shadow to settle the flicker, then re-lift grass tips
p_dg = pile{{"bone black", 0.8}, {"raw umber", 1.0}, {"green earth", 0.6}, {"cobalt blue", 0.3}, medium=0.7}
mdg = mask(function(x, y)
  return (y > 930 - 0.5 * (x - 250) and x > 250 and y < 1010) and 1 or 0
end)
work(mdg, {hand="glaze", pile=p_dg, coverage=0.55, edge=1, angle=-0.4, seed=671})
work(mdg, {hand="glaze", pile=p_dg, coverage=0.4, edge=1, angle=-0.42, seed=672})
print("bank ground settled")

--@ chunk 181

-- restate the grass: short firm strokes of dark grass silhouette against
-- the warm water margin, then a few lit tips
p_g2 = pile{{"bone black", 0.85}, {"raw umber", 1.05}, {"green earth", 0.55}, medium=0.4}
b4 = brush{kind="rigger", width=1.9, point=1, stiffness=0.6}
b4:load(p_g2, 0.9)
local specs2 = {{430, 742, 26}, {452, 750, 20}, {476, 745, 30}, {498, 752, 18}, {522, 748, 24},
  {548, 755, 16}, {574, 750, 22}, {598, 756, 15}, {622, 752, 19}, {648, 758, 14},
  {676, 755, 17}, {704, 760, 13}, {732, 757, 16}, {760, 762, 12}, {788, 759, 15},
  {818, 763, 11}, {846, 760, 14}, {874, 765, 10}, {902, 762, 13}, {930, 766, 11},
  {958, 764, 12}}
for _, s in ipairs(specs2) do
  local x, yb, h = s[1], s[2], s[3]
  if b4:fullness() < 0.25 then b4:load(p_g2, 0.9) end
  b4:stroke({{x, yb}, {x + rand(-4, 6), yb - h * 0.6}, {x + rand(-5, 7), yb - h}},
    {pressure={0.6, 0.1}, ramps={0.2, 0.5}})
end
-- lit tips: a few short ochre strokes at their tops
b5 = brush{kind="rigger", width=1.4, point=1, stiffness=0.55}
p_lit = pile{{"lead white", 2.0}, {"yellow ochre", 1.8}, medium=0.55}
b5:load(p_lit, 0.7)
for _, s in ipairs({{476, 715}, {522, 724}, {598, 734}, {676, 738}, {760, 747}, {874, 755}}) do
  local x, y = s[1], s[2]
  b5:stroke({{x, y + 7}, {x + rand(-3, 4), y}}, {pressure={0.4, 0.05}, ramps={0.3, 0.6}})
end
print("margin grass struck")

--@ chunk 182

-- tree silhouettes on the treeline top edge
p_t = pile{{"bone black", 0.9}, {"raw umber", 0.85}, {"green earth", 0.5}, medium=0.4}
b6 = brush{kind="round", width=6, point=0.7, stiffness=0.6}
b6:load(p_t, 0.9)
local tops = {{320, 248, 26}, {348, 252, 18}, {378, 246, 30}, {408, 250, 20},
  {560, 246, 24}, {584, 250, 16}, {610, 245, 28}, {636, 249, 19},
  {790, 244, 26}, {816, 248, 18}, {844, 243, 30}, {872, 247, 20},
  {905, 245, 24}, {930, 249, 16}}
for _, s in ipairs(tops) do
  local x, yb, h = s[1], s[2], s[3]
  b6:stroke({{x, yb + 6}, {x + rand(-3, 4), yb - h * 0.5}, {x + rand(-4, 5), yb - h}},
    {pressure={0.75, 0.25}, ramps={0.15, 0.45}})
end
-- birds in the warm sky
bb = brush{kind="rigger", width=1.3, point=1, stiffness=0.6}
p_bird = pile{{"bone black", 1.0}, {"raw umber", 0.6}, medium=0.45}
local birds = {{760, 150, 10}, {782, 138, 12}, {806, 148, 9}, {830, 132, 11}, {852, 142, 8}}
for _, b in ipairs(birds) do
  local x, y, s = b[1], b[2], b[3]
  bb:load(p_bird, 0.65)
  bb:stroke({{x, y}, {x - s, y - s * 0.55}}, {pressure={0.55, 0.12}, ramps={0.25, 0.5}})
  bb:stroke({{x, y}, {x + s * 0.9, y - s * 0.5}}, {pressure={0.5, 0.1}, ramps={0.25, 0.5}})
end
print("tufts and birds")

--@ chunk 183

-- the tufts came out as floating dark spikes on the mist, not trees:
-- pull each tuft down into the treeline so they grow out of its top edge
p_t = pile{{"bone black", 0.9}, {"raw umber", 0.85}, {"green earth", 0.5}, medium=0.4}
b7 = brush{kind="round", width=7, point=0.6, stiffness=0.6}
b7:load(p_t, 0.92)
local tops2 = {{320, 252, 26}, {350, 256, 18}, {380, 250, 30}, {410, 254, 20},
  {560, 250, 24}, {586, 254, 16}, {612, 249, 28}, {638, 253, 19},
  {792, 248, 26}, {818, 252, 18}, {846, 247, 30}, {874, 251, 20},
  {906, 249, 24}, {932, 253, 16}}
for _, s in ipairs(tops2) do
  local x, yb, h = s[1], s[2], s[3]
  b7:stroke({{x, yb + 14}, {x + rand(-3, 4), yb + 2}, {x + rand(-4, 5), yb - h + 6}},
    {pressure={0.9, 0.3}, ramps={0.1, 0.5}})
end
print("tufts rooted")

--@ chunk 184

-- tufts at the true edge: base y=312, rising to 272-292
p_t = pile{{"bone black", 0.95}, {"raw umber", 0.9}, {"green earth", 0.5}, medium=0.4}
b8 = brush{kind="round", width=8, point=0.55, stiffness=0.6}
b8:load(p_t, 0.95)
local tops3 = {{322, 314, 40}, {352, 316, 28}, {384, 312, 46}, {412, 315, 32},
  {562, 313, 38}, {588, 316, 26}, {614, 312, 44}, {640, 315, 30},
  {794, 312, 40}, {820, 315, 28}, {848, 311, 46}, {876, 314, 32},
  {908, 313, 38}, {934, 316, 26}}
for _, s in ipairs(tops3) do
  local x, yb, h = s[1], s[2], s[3]
  b8:stroke({{x, yb}, {x + rand(-4, 5), yb - h * 0.55}, {x + rand(-5, 6), yb - h}},
    {pressure={0.95, 0.35}, ramps={0.08, 0.5}})
end
print("tufts v2")

--@ chunk 185

-- swallow the old floating spikes: pale mist tone scumbled over them,
-- soft edges, two zones
p_m2 = pile{{"lead white", 4.0}, {"cobalt blue", 0.55}, {"yellow ochre", 0.55}, {"raw umber", 0.3}, {"bone black", 0.1}, medium=0.8}
z1 = mask(function(x, y) return (x > 290 and x < 720 and y > 175 and y < 262) and 1 or 0 end)
z2 = mask(function(x, y) return (x > 760 and x < 960 and y > 165 and y < 258) and 1 or 0 end)
work(z1, {hand="body", pile=p_m2, angle=0.03, coverage=1.15, fill=true, edge={lost=0.9, soft=0.3}, seed=681})
work(z2, {hand="body", pile=p_m2, angle=0.03, coverage=1.15, fill=true, edge={lost=0.9, soft=0.3}, seed=682})
print("spikes swallowed")

--@ chunk 186

-- the mist band, whole width, one coherent wash; no internal boundaries
band = rect(0, 148, 1000, 122)
work(band, {hand="broad", pile=p_m2, angle=0.02, coverage=2.2, ruler=true, edge={soft=0.5, lost=0.4}, seed=691})
work(band, {hand="broad", pile=p_m2, angle=0.01, coverage=1.6, ruler=true, edge={soft=0.5, lost=0.4}, seed=692})
work(band, {hand="body", pile=p_m2, angle=0.02, coverage=1.2, fill=true, edge={soft=0.4, lost=0.5}, seed=693})
blend(band:soften(24), {angle=0.02, seed=694})
blend(band:soften(24), {angle=0.0, seed=695})
print("band washed")

--@ chunk 187

-- heal the smear on the left treeline
p_tl = pile{{"bone black", 1.0}, {"raw umber", 0.85}, {"green earth", 0.5}, {"cobalt blue", 0.4}, {"lead white", 0.25}, medium=0.45}
sm = rect(60, 296, 260, 52)
work(sm, {hand="glaze", pile=p_tl, coverage=0.6, edge=1, angle=-0.02, seed=701})
work(sm, {hand="glaze", pile=p_tl, coverage=0.45, edge=1, angle=-0.01, seed=702})
-- birds over the water, low
bb = brush{kind="rigger", width=1.3, point=1, stiffness=0.6}
p_bird = pile{{"bone black", 1.0}, {"raw umber", 0.6}, medium=0.45}
local birds = {{420, 490, 9}, {446, 478, 11}, {472, 488, 8}}
for _, b in ipairs(birds) do
  local x, y, s = b[1], b[2], b[3]
  bb:load(p_bird, 0.65)
  bb:stroke({{x, y}, {x - s, y - s * 0.55}}, {pressure={0.5, 0.1}, ramps={0.25, 0.5}})
  bb:stroke({{x, y}, {x + s * 0.9, y - s * 0.5}}, {pressure={0.45, 0.08}, ramps={0.25, 0.5}})
end
print("smear healed, birds over water")

--@ chunk 188

-- dart upper half: band cream, body+fill
p_m3 = pile{{"lead white", 4.0}, {"cobalt blue", 0.5}, {"yellow ochre", 0.55}, {"raw umber", 0.3}, {"bone black", 0.1}, medium=0.75}
up = rect(50, 232, 300, 74)
work(up, {hand="body", pile=p_m3, angle=0.02, coverage=1.6, fill=true, edge={lost=0.85, soft=0.3}, seed=711})
-- dart lower half: treeline dark, body+fill
p_tl2 = pile{{"bone black", 1.0}, {"raw umber", 0.9}, {"green earth", 0.5}, {"cobalt blue", 0.4}, {"lead white", 0.25}, medium=0.45}
dn = rect(50, 298, 300, 36)
work(dn, {hand="body", pile=p_tl2, angle=-0.02, coverage=1.6, fill=true, edge={lost=0.7, soft=0.3}, seed=712})
print("dart covered")

--@ chunk 189

-- cool the left treeline to match the right
p_tc = pile{{"bone black", 0.95}, {"raw umber", 0.7}, {"green earth", 0.5}, {"cobalt blue", 0.5}, {"lead white", 0.25}, medium=0.55}
lt = rect(0, 296, 330, 120)
work(lt, {hand="glaze", pile=p_tc, coverage=0.5, edge=1, angle=-0.01, seed=721})
work(lt, {hand="glaze", pile=p_tc, coverage=0.4, edge=1, angle=-0.02, seed=722})
print("left cooled")

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
