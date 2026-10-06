-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=950, aspect=1.4, linen={18, 16}, seed=11,
  ground={
    {pile={{"red earth", 3}, {"yellow ochre", 1}, {"lead white", 1}}, um=90, apply="knife", texture=0.35},
    {pile={{"lead white", 12}, {"yellow ochre", 0.6}}, um=45, apply="brush"}}}
print("W", W, "H", H)

--@ chunk 2
-- Pencil underdrawing: the scaffolding of the whole picture
h = pencil("2H")

-- Far range (with snow peak near x 620-700)
h:sketch({{0, 404}, {90, 397}, {180, 389}, {260, 395}, {340, 385}, {430, 393}, {520, 381}, {580, 366}, {640, 350}, {690, 368}, {760, 385}, {860, 379}, {1000, 393}}, {pressure=0.3})
-- Second range
h:sketch({{0, 427}, {120, 417}, {240, 429}, {360, 413}, {480, 425}, {600, 415}, {740, 427}, {880, 417}, {1000, 429}}, {pressure=0.3})
-- Third, nearest range
h:sketch({{0, 455}, {150, 441}, {300, 457}, {450, 439}, {620, 453}, {800, 443}, {1000, 457}}, {pressure=0.3})
-- Left middle hill falling to the valley gap
h:sketch({{0, 466}, {80, 451}, {170, 459}, {260, 475}, {340, 502}, {400, 528}}, {pressure=0.3})
-- Right middle hill falling to the valley gap
h:sketch({{1000, 460}, {915, 446}, {830, 462}, {755, 486}, {695, 514}, {640, 534}, {590, 546}}, {pressure=0.3})
-- Foreground ridge crest
h:sketch({{0, 664}, {110, 640}, {220, 620}, {330, 600}, {430, 585}, {520, 583}, {610, 590}, {700, 596}, {790, 602}, {880, 618}, {1000, 646}}, {pressure=0.35})

-- Old oak: trunk spine and main limbs
h:sketch({{288, 652}, {290, 570}, {282, 480}, {294, 402}}, {pressure=0.35})   -- trunk
h:sketch({{294, 402}, {262, 340}, {214, 292}, {160, 252}, {128, 232}}, {pressure=0.3})  -- limb up-left
h:sketch({{294, 402}, {350, 352}, {408, 306}, {452, 262}}, {pressure=0.3})   -- limb up-right
h:sketch({{292, 400}, {306, 330}, {312, 262}, {322, 200}}, {pressure=0.3})   -- dead leader
h:sketch({{288, 470}, {240, 452}, {192, 448}, {150, 458}}, {pressure=0.3})   -- lower limb left
h:sketch({{296, 440}, {352, 420}, {410, 410}, {462, 414}}, {pressure=0.3})   -- lower limb right

-- Two firs at right
h:line({{806, 614}, {806, 348}}, {pressure=0.3})
h:line({{868, 632}, {868, 424}}, {pressure=0.3})

-- Two figures on the crest
h:sketch({{448, 588}, {448, 556}}, {pressure=0.4})
h:sketch({{482, 586}, {482, 552}}, {pressure=0.4})

-- Moon position
h:sketch({{692, 148}, {705, 140}, {718, 148}}, {pressure=0.3})
-- Distant spire in the valley gap
h:line({{560, 492}, {560, 468}}, {pressure=0.4})
print("drawn")

--@ chunk 3
-- Very thin underpainting veils
veil_sky = pile{{"lead white", 3}, {"smalt", 1}, {"vermilion", 0.15}, {"raw umber", 0.2}, medium=0.45}
veil_land = pile{{"lead white", 2}, {"raw umber", 1}, {"yellow ochre", 0.5}, medium=0.4}

sky_zone = rect(0, 0, 1000, 470):soften(30)
land_zone = rect(0, 420, 1000, 294):soften(25)

work(sky_zone, {hand="glaze", pile=veil_sky, angle=0, coverage=1.1})
work(land_zone, {hand="glaze", pile=veil_land, angle=0.15, coverage=1.1})
print("underpainting laid")

--@ chunk 4
-- Even out the veils while wet, then blend
work(sky_zone, {hand="glaze", pile=veil_sky, angle=0, coverage=2.5, fill=true})
work(land_zone, {hand="glaze", pile=veil_land, angle=0.15, coverage=2.2, fill=true})
blend(sky_zone, {angle=0})
blend(land_zone, {angle=0.2})
print("veils evened")

--@ chunk 5
-- isolate the failure
p_test = pile{{"lead white", 5}, {"cobalt blue", 1.2}, medium=0.2}
print(p_test)
m_t = mask(function(x, y) return clamp((215 - y) / 130, 0, 1) end)
print("mask at (10,10):", m_t:at(10, 10))
print("mask at (10,400):", m_t:at(10, 400))

--@ chunk 6
-- try positional curve
m_patch = rect(10, 690, 40, 20)
work(m_patch, {hand="body", pile=p_test, angle=0, coverage=1, curve={4, 2}})
print("positional curve ok")

--@ chunk 7
-- Sky base: four interlocking zones, wet into wet
sky_top  = pile{{"lead white", 5}, {"cobalt blue", 1.2}, {"smalt", 0.8}, {"bone black", 0.25}, medium=0.2}
sky_mid  = pile{{"lead white", 6}, {"cobalt blue", 0.5}, {"smalt", 0.4}, {"raw umber", 0.15}, medium=0.2}
sky_low  = pile{{"lead white", 6}, {"yellow ochre", 0.5}, {"vermilion", 0.15}, {"smalt", 0.25}, medium=0.2}
glow     = pile{{"lead white", 5}, {"yellow ochre", 1}, {"chrome yellow", 0.5}, {"vermilion", 0.3}, medium=0.15}

m_top = mask(function(x, y) return clamp((215 - y) / 130, 0, 1) end)
m_mid = mask(function(x, y) return clamp(1 - math.abs(y - 250) / 115, 0, 1) end)
m_low = mask(function(x, y) return clamp(1 - math.abs(y - 385) / 105, 0, 1) end)
m_glow = mask(function(x, y)
  local dx = (x - 380) / 330
  local dy = (y - 425) / 150
  return clamp(1 - (dx*dx + dy*dy), 0, 1)
end)

work(m_top, {hand="body", pile=sky_top, angle=0.02, coverage=2.2, fill=true, curve={4, 2}})
work(m_mid, {hand="body", pile=sky_mid, angle=0, coverage=2.2, fill=true, curve={4, 2}})
work(m_low, {hand="body", pile=sky_low, angle=-0.02, coverage=2.2, fill=true, curve={3, 2}})
work(m_glow, {hand="body", pile=glow, angle=0, coverage=2.4, fill=true})
print("sky zones laid")

--@ chunk 8
-- Fuse the sky while everything is open
sky_all = rect(0, 0, 1000, 480):soften(20)
blend(sky_all, {angle=0})
print("sky blended")

--@ chunk 9
-- Stippled veils to tune the sky (coverage as functions)
deep_top = pile{{"lead white", 4}, {"cobalt blue", 1.4}, {"smalt", 0.7}, {"bone black", 0.35}, medium=0.35}
pearl    = pile{{"lead white", 7}, {"smalt", 0.3}, {"raw umber", 0.2}, medium=0.4}
warm_low = pile{{"lead white", 6}, {"yellow ochre", 0.8}, {"vermilion", 0.35}, medium=0.4}

m_deepen = mask(function(x, y) return clamp((150 - y) / 150, 0, 1) end)
m_calm   = mask(function(x, y) return clamp((390 - y) / 110, 0, 1) * clamp((y - 240) / 60, 0, 1) end)
m_warm   = mask(function(x, y)
  local dx = (x - 380) / 420
  local dy = (y - 445) / 70
  return clamp(1 - (dx*dx + dy*dy), 0, 1)
end)

cov_deepen = function(x, y) return clamp((150 - y) / 150, 0, 1) * 1.2 end
cov_calm   = function(x, y) return clamp((390 - y) / 110, 0, 1) * clamp((y - 240) / 60, 0, 1) * 1.1 end
cov_warm   = function(x, y)
  local dx = (x - 380) / 420
  local dy = (y - 445) / 70
  return clamp(1 - (dx*dx + dy*dy), 0, 1) * 1.3
end

stipple(m_deepen, {pile=deep_top, width=3, coverage=cov_deepen, feather=0.5})
stipple(m_calm,   {pile=pearl,    width=3, coverage=cov_calm, feather=0.5})
stipple(m_warm,   {pile=warm_low, width=3, coverage=cov_warm, feather=0.6})
print("veils stippled")

--@ chunk 10
-- Drag the glow into a horizontal band while still open
m_glowzone = rect(60, 260, 700, 220):soften(30)
blend(m_glowzone, {angle=0})
print("glow dragged out")

--@ chunk 11
-- Stratus bands of dusk
cloud_dark = pile{{"lead white", 3}, {"raw umber", 1}, {"smalt", 0.8}, {"bone black", 0.5}, medium=0.25}
cloud_lit  = pile{{"lead white", 6}, {"yellow ochre", 0.5}, {"vermilion", 0.4}, medium=0.25}

-- low right band, long and thin, swallowing the glow's right side
b1 = ribbon({{1005, 414}, {920, 407}, {830, 415}, {740, 409}, {650, 417}, {560, 413}, {475, 421}}, {18, 16, 19, 15, 17, 13, 9}):soften(7)
-- middle band, higher, thinner
b2 = ribbon({{75, 318}, {220, 309}, {360, 317}, {500, 311}, {645, 319}, {770, 313}}, {10, 13, 11, 14, 11, 8}):soften(8)
-- two small fragments
b3 = ribbon({{620, 362}, {700, 358}, {790, 363}}, {7, 9, 6}):soften(6)
b4 = ribbon({{180, 368}, {255, 364}, {330, 369}}, {6, 8, 5}):soften(6)

work(b1, {hand="body", pile=cloud_dark, angle=0.01, coverage=1.8, fill=true, edge=0.4})
work(b2, {hand="body", pile=cloud_dark, angle=0, coverage=1.5, fill=true, edge=0.5})
work(b3, {hand="body", pile=cloud_dark, angle=0, coverage=1.4, fill=true, edge=0.6})
work(b4, {hand="body", pile=cloud_dark, angle=0, coverage=1.4, fill=true, edge=0.6})
print("cloud bodies in")

--@ chunk 12
-- Fuse each band along its length
blend(b1, {angle=0})
blend(b2, {angle=0})
blend(b3, {angle=0})
blend(b4, {angle=0})

-- Warm lit rims under the two lower bands, on the glow side
rim1 = ribbon({{820, 428}, {740, 423}, {650, 430}, {560, 426}, {480, 432}}, {5, 6, 5, 6, 4}):soften(5)
rim2 = ribbon({{610, 424}, {540, 420}, {470, 425}}, {4, 5, 4}):soften(4)
-- a few small lit streaks above the glow
streak1 = ribbon({{300, 396}, {370, 392}, {440, 396}}, {4, 5, 3}):soften(5)
streak2 = ribbon({{210, 384}, {265, 381}, {320, 385}}, {3, 4, 2}):soften(4)

work(rim1, {hand="body", pile=cloud_lit, angle=0, coverage=1.5, fill=true, edge=0.6})
work(rim2, {hand="body", pile=cloud_lit, angle=0, coverage=1.4, fill=true, edge=0.6})
work(streak1, {hand="body", pile=cloud_lit, angle=0, coverage=1.3, fill=true, edge=0.7})
work(streak2, {hand="body", pile=cloud_lit, angle=0, coverage=1.3, fill=true, edge=0.7})
blend(rim1, {angle=0})
blend(streak1, {angle=0})
print("rims lit")

--@ chunk 13
print(wait(22 * 60))

--@ chunk 14
print("sky top:", drying(500, 80))
print("sky mid:", drying(500, 260))
print("glow:", drying(380, 420))
print("land veil:", drying(500, 620))

--@ chunk 15
-- The three ranges, far to near
r1_pile = pile{{"lead white", 4}, {"smalt", 1}, {"cobalt blue", 0.6}, {"raw umber", 0.2}, medium=0.25}
r2_pile = pile{{"lead white", 3}, {"smalt", 1.2}, {"cobalt blue", 0.8}, {"raw umber", 0.3}, medium=0.25}
r3_pile = pile{{"lead white", 2}, {"smalt", 1}, {"Prussian blue", 0.35}, {"raw umber", 0.6}, {"yellow ochre", 0.3}, medium=0.25}

crest1 = {{0,404},{90,397},{180,389},{260,395},{340,385},{430,393},{520,381},{580,366},{640,350},{690,368},{760,385},{860,379},{1000,393}}
crest2 = {{0,427},{120,417},{240,429},{360,413},{480,425},{600,415},{740,427},{880,417},{1000,429}}
crest3 = {{0,455},{150,441},{300,457},{450,439},{620,453},{800,443},{1000,457}}

range1 = below(crest1):soften(2)
range2 = below(crest2):soften(1.8)
range3 = below(crest3):soften(1.5)

work(range1, {hand="body", pile=r1_pile, angle=0, coverage=1.7, fill=true, edge=0.3})
work(range2, {hand="body", pile=r2_pile, angle=0, coverage=1.7, fill=true, edge=0.3})
work(range3, {hand="body", pile=r3_pile, angle=0, coverage=1.8, fill=true, edge=0.25})
print("ranges in")

--@ chunk 16
-- Snow on the far peak: warm-lit left flank, cool right flank
snow_lit = pile{{"lead white", 5}, {"yellow ochre", 0.4}, {"vermilion", 0.2}, medium=0.1}
snow_sh  = pile{{"lead white", 5}, {"smalt", 0.8}, {"cobalt blue", 0.4}, medium=0.1}

sb = brush("filbert", 5)
-- left flank of the apex, catching the afterglow: strokes along the crest slope
sb:load(snow_lit, 0.9)
sb:stroke({{640, 351}, {622, 357}, {604, 362}}, {pressure={0.7, 0.4}, ramps={0.15, 0.4}, orient="along"})
sb:load(snow_lit, 0.8)
sb:stroke({{636, 353}, {618, 360}, {597, 366}}, {pressure={0.6, 0.35}, ramps={0.15, 0.4}, orient="along"})
sb:load(snow_lit, 0.7)
sb:stroke({{628, 357}, {612, 364}, {590, 368}}, {pressure={0.55, 0.3}, ramps={0.15, 0.45}, orient="along"})
-- right flank, cooler
sb:reload(snow_sh, 0.9)
sb:stroke({{644, 352}, {662, 358}, {684, 366}}, {pressure={0.7, 0.4}, ramps={0.15, 0.4}, orient="along"})
sb:reload(snow_sh, 0.8)
sb:stroke({{648, 356}, {668, 362}, {690, 369}}, {pressure={0.6, 0.35}, ramps={0.15, 0.4}, orient="along"})
-- a small secondary snow patch on the shoulder at x~585
sb:reload(snow_lit, 0.6)
sb:stroke({{586, 366}, {576, 369}, {566, 373}}, {pressure={0.5, 0.3}, ramps={0.2, 0.5}, orient="along"})
print("snow set")

--@ chunk 17
-- Middle-ground hills flanking the valley gap
hill_pile  = pile{{"lead white", 1}, {"Prussian blue", 0.5}, {"smalt", 0.8}, {"raw umber", 1}, {"yellow ochre", 0.4}, medium=0.2}
hill_dark  = pile{{"raw umber", 1}, {"Prussian blue", 0.4}, {"bone black", 0.3}, {"green earth", 0.5}, medium=0.15}

hillL = below({{0,466},{80,451},{170,459},{260,475},{340,502},{400,528}}) * rect(0, 430, 415, 240)
hillR = below({{1000,460},{915,446},{830,462},{755,486},{695,514},{640,534},{590,546}}) * rect(575, 425, 425, 260)
hillL = hillL:soften(1.5)
hillR = hillR:soften(1.5)

work(hillL, {hand="body", pile=hill_pile, angle=0.1, coverage=1.8, fill=true, edge=0.35})
work(hillR, {hand="body", pile=hill_pile, angle=-0.08, coverage=1.8, fill=true, edge=0.35})

-- wooded texture, denser toward each hill's crest
stipple(hillL, {pile=hill_dark, width=2.6, coverage=function(x, y) return clamp(1.6 - (y - 445) / 60, 0, 1.6) end, feather=0.4, cluster=0.4})
stipple(hillR, {pile=hill_dark, width=2.6, coverage=function(x, y) return clamp(1.6 - (y - 440) / 65, 0, 1.6) end, feather=0.4, cluster=0.4})
print("hills in")

--@ chunk 18
-- Valley mist, valley floor light, water gleam, and the sunset point above the far crest
mist = pile{{"lead white", 5}, {"smalt", 0.6}, medium=0.5}
mist_warm = pile{{"lead white", 5}, {"yellow ochre", 0.5}, {"vermilion", 0.2}, medium=0.5}
gleam = pile{{"lead white", 6}, {"yellow ochre", 0.7}, {"vermilion", 0.25}, medium=0.3}
crestlight = pile{{"lead white", 6}, {"yellow ochre", 0.8}, {"vermilion", 0.3}, medium=0.4}

m_mistband = mask(function(x, y) return clamp(1 - math.abs(y - 510) / 75, 0, 1) end)
cov_mist = function(x, y)
  local band = clamp(1 - math.abs(y - 512) / 80, 0, 1)
  local gap = clamp(1 - math.abs(x - 500) / 260, 0, 1)
  return band * (0.9 + 1.6 * gap)
end
stipple(m_mistband, {pile=mist, width=3, coverage=cov_mist, feather=0.6})

-- pale valley floor in the gap
m_floor = mask(function(x, y)
  local dx = (x - 505) / 190
  local dy = (y - 545) / 45
  return clamp(1 - (dx*dx + dy*dy), 0, 1)
end)
cov_floor = function(x, y)
  local dx = (x - 505) / 190
  local dy = (y - 545) / 45
  return clamp(1 - (dx*dx + dy*dy), 0, 1) * 1.5
end
stipple(m_floor, {pile=mist_warm, width=3, coverage=cov_floor, feather=0.6})

-- the river bend catching the afterglow: a few horizontal strokes
gb = brush("filbert", 6)
gb:load(gleam, 0.85)
gb:stroke({{452, 524}, {500, 521}, {548, 524}}, {pressure={0.6, 0.5}, ramps={0.2, 0.5}, orient="along"})
gb:load(gleam, 0.7)
gb:stroke({{478, 534}, {524, 532}, {566, 535}}, {pressure={0.55, 0.4}, ramps={0.2, 0.5}, orient="along"})
gb:load(gleam, 0.6)
gb:stroke({{505, 544}, {540, 543}, {572, 545}}, {pressure={0.5, 0.35}, ramps={0.25, 0.5}, orient="along"})

-- the light gathering just above the far crest where the sun went down
m_set = mask(function(x, y)
  local dx = (x - 385) / 110
  local dy = (y - 396) / 22
  return clamp(1 - (dx*dx + dy*dy), 0, 1)
end)
cov_set = function(x, y)
  local dx = (x - 385) / 110
  local dy = (y - 396) / 22
  return clamp(1 - (dx*dx + dy*dy), 0, 1) * 1.6
end
stipple(m_set, {pile=crestlight, width=2.6, coverage=cov_set, feather=0.7})
print("mist and light in")

--@ chunk 19
-- Valley floor: a soft pale wedge between the hills, hiding the cut walls
floor_pile = pile{{"lead white", 4}, {"smalt", 0.5}, {"yellow ochre", 0.4}, {"raw umber", 0.2}, medium=0.35}
wedge = poly({{430, 468}, {560, 468}, {700, 660}, {310, 660}}, true):soften(16)
work(wedge, {hand="body", pile=floor_pile, angle=0, coverage=2.2, fill=true, edge=0.55})
-- re-veil with mist so it sits back
stipple(m_mistband, {pile=mist, width=3, coverage=cov_mist, feather=0.6})
-- the river again, an S of quiet light
gb:load(gleam, 0.85)
gb:stroke({{556, 502}, {530, 508}, {508, 515}}, {pressure={0.5, 0.4}, ramps={0.2, 0.5}, orient="along"})
gb:load(gleam, 0.8)
gb:stroke({{510, 518}, {480, 524}, {455, 531}}, {pressure={0.55, 0.4}, ramps={0.2, 0.5}, orient="along"})
gb:load(gleam, 0.7)
gb:stroke({{470, 538}, {500, 542}, {530, 546}}, {pressure={0.5, 0.4}, ramps={0.2, 0.5}, orient="along"})
gb:load(gleam, 0.6)
gb:stroke({{492, 555}, {524, 558}, {556, 561}}, {pressure={0.45, 0.35}, ramps={0.25, 0.5}, orient="along"})
print("floor rebuilt")

--@ chunk 20
-- Cool the wedge top, then wooded spurs narrowing the valley
m_wedgetop = (wedge * rect(0, 455, 1000, 90)):soften(10)
work(m_wedgetop, {hand="glaze", pile=mist, angle=0, coverage=1.4, fill=true})

spurL = poly({{330, 505}, {395, 522}, {445, 548}, {478, 578}, {455, 600}, {390, 578}, {335, 545}}, true):soften(8)
spurR = poly({{600, 540}, {650, 560}, {672, 592}, {640, 612}, {585, 585}, {560, 560}}, true):soften(8)
work(spurL, {hand="body", pile=hill_pile, angle=0.35, coverage=1.7, fill=true, edge=0.5})
work(spurR, {hand="body", pile=hill_pile, angle=-0.3, coverage=1.7, fill=true, edge=0.5})
stipple(spurL, {pile=hill_dark, width=2.4, coverage=0.9, feather=0.4, cluster=0.4})
stipple(spurR, {pile=hill_dark, width=2.4, coverage=0.9, feather=0.4, cluster=0.4})
print("spurs in")

--@ chunk 21
-- Valley refinements: cool shadow, tree-tops through mist, river, spire
shade_pile = pile{{"lead white", 1.5}, {"smalt", 0.9}, {"raw umber", 0.7}, {"Prussian blue", 0.3}, medium=0.3}
m_vshade = mask(function(x, y)
  local dx = (x - 450) / 120
  local dy = (y - 566) / 40
  return clamp(1 - (dx*dx + dy*dy), 0, 1)
end)
cov_vshade = function(x, y)
  local dx = (x - 450) / 120
  local dy = (y - 566) / 40
  return clamp(1 - (dx*dx + dy*dy), 0, 1) * 1.2
end
stipple(m_vshade, {pile=shade_pile, width=3, coverage=cov_vshade, feather=0.6})

-- dark tree-tops poking through the mist's upper edge
m_treetops = ribbon({{400, 478}, {460, 470}, {520, 476}, {580, 468}, {630, 474}}, 10):soften(6)
stipple(m_treetops, {pile=hill_dark, width=2.2, coverage=0.8, feather=0.5, cluster=0.6})

-- the river again, brighter and on top
gleam2 = pile{{"lead white", 8}, {"yellow ochre", 0.6}, {"vermilion", 0.2}, medium=0.2}
gb2 = brush("filbert", 4.5)
gb2:load(gleam2, 0.9)
gb2:stroke({{548, 540}, {522, 546}, {500, 552}}, {pressure={0.5, 0.4}, ramps={0.2, 0.5}, orient="along"})
gb2:load(gleam2, 0.85)
gb2:stroke({{505, 558}, {532, 562}, {558, 565}}, {pressure={0.5, 0.4}, ramps={0.2, 0.5}, orient="along"})
gb2:load(gleam2, 0.7)
gb2:stroke({{478, 572}, {508, 576}, {536, 579}}, {pressure={0.45, 0.35}, ramps={0.25, 0.5}, orient="along"})

-- the little church spire rising from the mist
spire_dark = pile{{"raw umber", 2}, {"bone black", 1}, {"smalt", 0.5}, medium=0.1}
sp = brush("rigger", 2.2)
sp:load(spire_dark, 0.9)
sp:stroke({{562, 508}, {562, 492}}, {pressure={0.7, 0.5}, ramps={0.1, 0.3}, orient="along"})  -- tower
sp:stroke({{561, 493}, {562.5, 483}}, {pressure={0.6, 0.2}, ramps={0.1, 0.5}, orient="along"}) -- spire
sp:stroke({{556, 510}, {568, 510}}, {pressure={0.5, 0.4}, ramps={0.2, 0.3}, orient="along"})  -- roofline
print("valley detailed")

--@ chunk 22
-- The foreground ridge: the great dark repoussoir
ridge_pile = pile{{"raw umber", 3}, {"bone black", 1.5}, {"yellow ochre", 0.5}, {"lead white", 0.3}, medium=0.15}
crest_pts = {{0,664},{110,640},{220,620},{330,600},{430,585},{520,583},{610,590},{700,596},{790,602},{880,618},{1000,646}}
ridge = below(crest_pts):soften(1.2)
work(ridge, {hand="body", pile=ridge_pile, angle=0.12, coverage=2.8, fill=true, edge=0.15})
print("ridge mass in")

--@ chunk 23
-- Deepen the ridge, darken the corners, lay in a faint path
ridge_dark = pile{{"raw umber", 2}, {"bone black", 2}, {"Prussian blue", 0.2}, medium=0.12}
work(ridge, {hand="scumble", pile=ridge_dark, angle=0.1, coverage=2.2, fill=true, edge=0.1})

m_corners = mask(function(x, y)
  local v = clamp((y - 600) / 114, 0, 1)
  local c = clamp((math.abs(x - 500) - 250) / 250, 0, 1)
  return v * (0.4 + 0.6 * c)
end)
cov_corners = function(x, y)
  local v = clamp((y - 600) / 114, 0, 1)
  local c = clamp((math.abs(x - 500) - 250) / 250, 0, 1)
  return v * (0.4 + 0.6 * c) * 1.6
end
stipple(m_corners, {pile=ridge_dark, width=4, coverage=cov_corners, feather=0.5})

-- the faint path down from the crest
path_pile = pile{{"raw umber", 2}, {"yellow ochre", 1}, {"lead white", 1}, medium=0.15}
path = ribbon({{512, 588}, {540, 622}, {585, 660}, {640, 700}, {668, 714}}, {7, 9, 12, 16, 20}):soften(6)
work(path, {hand="scumble", pile=path_pile, angle=1.1, coverage=1.4, fill=false, edge=0.7})
print("ridge deepened")

--@ chunk 24
print(wait(7 * 60))
print("ridge:", drying(500, 650))
print("sky:", drying(500, 100))

--@ chunk 25
-- Crest rim light, grass tufts, rocks
straw = pile{{"yellow ochre", 2}, {"lead white", 2}, {"raw umber", 0.5}, medium=0.1}
grass_p = pile{{"green earth", 1}, {"yellow ochre", 0.6}, {"raw umber", 0.8}, medium=0.1}
rock_l = pile{{"lead white", 2}, {"raw umber", 1}, {"yellow ochre", 0.5}, medium=0.1}

-- broken straw rim along the crest
crest_band = ribbon(crest_pts, 3.5):soften(2.5)
stipple(crest_band, {pile=straw, width=2, coverage=0.85, feather=0.7, cluster=0.5})

-- grass tufts along and just below the crest, each its own little stroke
gb3 = brush{kind="round", width=1.8, point=0.7, stiffness=0.6}
local cresty = {{0,664},{110,640},{220,620},{330,600},{430,585},{520,583},{610,590},{700,596},{790,602},{880,618},{1000,646}}
local function yat(x)
  for i = 1, #cresty - 1 do
    local a, b = cresty[i], cresty[i + 1]
    if x >= a[1] and x <= b[1] then
      return a[2] + (b[2] - a[2]) * (x - a[1]) / (b[1] - a[1])
    end
  end
  return 650
end
for i = 1, 46 do
  local x = rand(15, 985)
  local y0 = yat(x) + rand(-1, 3)
  local lean = rand(-25, 25) / 10
  local hgt = rand(3, 8)
  if i % 3 == 0 then gb3:reload(straw, rand(35, 70) / 100) else gb3:reload(grass_p, rand(40, 80) / 100) end
  gb3:stroke({{x, y0}, {x + lean, y0 - hgt}}, {pressure={0.5, 0.15}, ramps={0.08, 0.55}, orient="along"})
end

-- a few rock outcrops catching light on their tops
rb = brush("filbert", 5)
rb:load(rock_l, 0.8)
rb:stroke({{255, 634}, {276, 630}}, {pressure={0.5, 0.4}, ramps={0.2, 0.4}, orient="along"})
rb:stroke({{300, 622}, {318, 619}}, {pressure={0.5, 0.4}, ramps={0.2, 0.4}, orient="along"})
rb:load(rock_l, 0.7)
rb:stroke({{760, 628}, {782, 625}}, {pressure={0.5, 0.4}, ramps={0.2, 0.4}, orient="along"})
rb:stroke({{806, 636}, {826, 633}}, {pressure={0.45, 0.35}, ramps={0.2, 0.4}, orient="along"})
print("ridge dressed")

--@ chunk 26
-- Old oak: trunk and main limbs as one skeleton silhouette
bark_dark = pile{{"bone black", 2}, {"raw umber", 1.5}, {"smalt", 0.2}, medium=0.1}
oak = body_of{
  spine = {{288, 652}, {290, 570}, {282, 480}, {294, 402}},
  widths = {24, 19, 15, 13},
  limbs = {
    {pts = {{294, 402}, {262, 340}, {214, 292}, {160, 252}, {128, 232}}, widths = {10, 7.5, 5.5, 3.5, 2}},
    {pts = {{294, 402}, {350, 352}, {408, 306}, {452, 262}}, widths = {10, 7, 4.5, 2.5}},
    {pts = {{292, 400}, {306, 330}, {312, 262}, {322, 200}}, widths = {7, 5, 3, 1.8}},
    {pts = {{288, 470}, {240, 452}, {192, 448}, {150, 458}}, widths = {7, 5, 3.5, 2}},
    {pts = {{296, 440}, {352, 420}, {410, 410}, {462, 414}}, widths = {6, 4.5, 3, 2}},
  },
  blend = 0.8, char = "firm", amount = 0.5, seed = 4}
oak_m = oak:mask()
work(oak_m, {hand="body", pile=bark_dark, angle=-1.35, coverage=2.4, fill=true, edge=0.15})
print("oak frame painted")

--@ chunk 27
-- Secondary branches of the oak, each drawn as its own tapering stroke
rb1 = brush{kind="rigger", width=3.6, point=1, stiffness=0.55, length=22}
local branches = {
  -- off limb A (up-left)
  {{{214,292},{190,250},{168,222},{150,200}}, 3.4},
  {{{160,252},{135,225},{112,210}}, 2.8},
  {{{128,232},{100,212},{78,198},{64,186}}, 2.6},
  {{{262,340},{230,330},{205,322}}, 2.6},
  -- off limb B (up-right)
  {{{350,352},{385,318},{420,295},{445,280}}, 3.2},
  {{{408,306},{445,285},{478,272},{500,262}}, 2.6},
  {{{452,262},{478,240},{505,225},{520,212}}, 2.4},
  {{{408,306},{440,310},{468,312}}, 2.2},
  -- off limb D (lower left)
  {{{240,452},{210,430},{185,420}}, 2.8},
  {{{150,458},{122,462},{100,470}}, 2.4},
  {{{192,448},{165,432},{145,425}}, 2.2},
  -- off limb E (lower right)
  {{{352,420},{385,400},{412,392}}, 2.6},
  {{{462,414},{492,418},{518,424}}, 2.2},
  {{{410,410},{438,398},{460,390}}, 2.0},
  -- dead leader's great side branches (dark for now, bleached later)
  {{{306,330},{330,300},{348,278}}, 2.8},
  {{{312,262},{295,235},{288,212}}, 2.4},
  {{{322,200},{330,178},{334,160}}, 2.0},
  {{{312,262},{335,240},{352,225}}, 2.2},
  -- broken snag on the trunk, right side
  {{{292,520},{314,506}}, 3.0},
  -- epicormic shoots
  {{{330,368},{336,344}}, 1.8},
  {{{286,540},{278,518}}, 1.8},
}
for _, br in ipairs(branches) do
  local pts, w = br[1], br[2]
  rb1:reload(bark_dark, 0.85)
  rb1:stroke(pts, {pressure={0.85 * w / 3.6, 0.06}, ramps={0.06, 0.7}, orient="along"})
end
print("secondaries drawn:", #branches)

--@ chunk 28
-- The twig net: fine shoots at the crown edge, dead spikes on the stag head
dead_pale = pile{{"lead white", 1.5}, {"raw umber", 0.8}, {"bone black", 0.25}, {"smalt", 0.15}, medium=0.1}
rb2 = brush{kind="rigger", width=1.7, point=1, stiffness=0.5, length=14}

local cx0, cy0 = 300, 430  -- tree center for "outward"
local clusters = {
  {150,200,false},{112,210,false},{64,186,false},{205,322,false},
  {445,280,false},{500,262,false},{520,212,false},{468,312,false},
  {185,420,false},{100,470,false},{145,425,false},
  {412,392,false},{518,424,false},{460,390,false},
  {348,278,true},{288,212,true},{334,160,true},{352,225,true},
  {306,330,true},{312,262,true},
  -- interior laterals to thicken the net a little
  {262,340,false},{214,292,false},{160,252,false},{350,352,false},
  {408,306,false},{240,452,false},{352,420,false},{410,410,false},
}
math.randomseed(811)
for _, c in ipairs(clusters) do
  local cx, cy, dead = c[1], c[2], c[3]
  local out = math.atan(cy - cy0, cx - cx0)
  local n = dead and math.random(2, 3) or math.random(3, 5)
  rb2:reload(dead and dead_pale or bark_dark, 0.9)
  for j = 1, n do
    local ang = out + (math.random() - 0.5) * 1.7 - 0.15
    local len = dead and (3 + math.random() * 4) or (4 + math.random() * 5.5)
    local tx = cx + len * math.cos(ang)
    local ty = cy + len * math.sin(ang)
    rb2:stroke({{cx, cy}, {tx, ty}}, {pressure={0.55, 0.05}, ramps={0.08, 0.7}, orient="along"})
    if math.random() < 0.45 then
      local ang2 = ang + (math.random() - 0.5) * 1.1
      local len2 = 2.5 + math.random() * 3.5
      rb2:stroke({{tx, ty}, {tx + len2 * math.cos(ang2), ty + len2 * math.sin(ang2)}}, {pressure={0.4, 0.04}, ramps={0.08, 0.7}, orient="along"})
    end
  end
end
print("twig net done")

--@ chunk 29
-- Denser net: twigs along every branch + fuller clusters at the tips
cx0, cy0 = 300, 430
rb3 = brush{kind="rigger", width=1.5, point=1, stiffness=0.5, length=12}
paths = {
  {{214,292},{190,250},{168,222},{150,200}},
  {{160,252},{135,225},{112,210}},
  {{128,232},{100,212},{78,198},{64,186}},
  {{262,340},{230,330},{205,322}},
  {{350,352},{385,318},{420,295},{445,280}},
  {{408,306},{445,285},{478,272},{500,262}},
  {{452,262},{478,240},{505,225},{520,212}},
  {{408,306},{440,310},{468,312}},
  {{240,452},{210,430},{185,420}},
  {{150,458},{122,462},{100,470}},
  {{192,448},{165,432},{145,425}},
  {{352,420},{385,400},{412,392}},
  {{462,414},{492,418},{518,424}},
  {{410,410},{438,398},{460,390}},
  {{294,402},{262,340},{214,292},{160,252},{128,232}},
  {{294,402},{350,352},{408,306},{452,262}},
}
math.randomseed(417)
for _, p in ipairs(paths) do
  for i = 1, #p - 1 do
    local a, b = p[i], p[i + 1]
    local dir = math.atan(b[2] - a[2], b[1] - a[1])
    local nseg = math.random(1, 2)
    for s = 1, nseg do
      local t = math.random()
      local bx = a[1] + (b[1] - a[1]) * t
      local by = a[2] + (b[2] - a[2]) * t
      rb3:reload(bark_dark, 0.85)
      local ntw = math.random(1, 2)
      for k = 1, ntw do
        local side = (math.random() < 0.5) and 1 or -1
        local ang = dir + side * (0.5 + math.random() * 0.5)
        local len = 2.5 + math.random() * 4
        local tx, ty = bx + len * math.cos(ang), by + len * math.sin(ang)
        rb3:stroke({{bx, by}, {tx, ty}}, {pressure={0.5, 0.05}, ramps={0.08, 0.7}, orient="along"})
        if math.random() < 0.3 then
          local ang2 = ang + (math.random() - 0.5) * 0.9
          local l2 = 2 + math.random() * 2.5
          rb3:stroke({{tx, ty}, {tx + l2 * math.cos(ang2), ty + l2 * math.sin(ang2)}}, {pressure={0.35, 0.04}, ramps={0.08, 0.7}, orient="along"})
        end
      end
    end
  end
end
-- fuller tufts right at the crown-edge tips
tips = {{150,200},{112,210},{64,186},{205,322},{445,280},{500,262},{520,212},{468,312},{185,420},{100,470},{145,425},{412,392},{518,424},{460,390},{128,232},{452,262}}
for _, c in ipairs(tips) do
  local out = math.atan(c[2] - cy0, c[1] - cx0)
  rb3:reload(bark_dark, 0.9)
  for j = 1, math.random(4, 6) do
    local ang = out + (math.random() - 0.5) * 1.5 - 0.12
    local len = 3 + math.random() * 4.5
    rb3:stroke({{c[1], c[2]}, {c[1] + len * math.cos(ang), c[2] + len * math.sin(ang)}}, {pressure={0.5, 0.04}, ramps={0.08, 0.75}, orient="along"})
  end
end
print("dense net done")

--@ chunk 30
-- Bleach the dead leader and snag; warm rim from the afterglow
rim = pile{{"raw umber", 1}, {"lead white", 1.2}, {"yellow ochre", 0.4}, medium=0.08}
rb4 = brush{kind="rigger", width=2.2, point=0.8, stiffness=0.55, length=16}
local deadwood = {
  {{292,400},{306,330},{312,262},{322,200}},
  {{306,330},{330,300},{348,278}},
  {{312,262},{295,235},{288,212}},
  {{322,200},{330,178},{334,160}},
  {{312,262},{335,240},{352,225}},
  {{292,520},{314,506}},
}
for _, p in ipairs(deadwood) do
  rb4:reload(dead_pale, 0.55)
  rb4:stroke(p, {pressure={0.5, 0.3}, ramps={0.1, 0.4}, orient="along"})
end
-- warm rim on the trunk's right edge and limb undersides
rb5 = brush{kind="rigger", width=2, point=0.7, stiffness=0.55, length=18}
local rims = {
  {{300,645},{303,565},{296,485},{302,415}},
  {{300,408},{356,358},{410,312},{450,268}},   -- limb B underside
  {{288,410},{258,350},{210,300},{160,260}},   -- limb A underside
  {{300,446},{354,426},{410,416},{460,420}},   -- limb E underside
}
for _, p in ipairs(rims) do
  rb5:reload(rim, 0.5)
  rb5:stroke(p, {pressure={0.4, 0.25}, ramps={0.12, 0.4}, orient="along"})
end
print("deadwood and rim done")

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
