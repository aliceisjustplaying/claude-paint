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

--@ chunk 31
print("time check:")
print("crest/figures:", drying(465, 570))
print("firs area:", drying(840, 480))
print("moon sky:", drying(700, 145))
print("oak trunk:", drying(290, 500))
print("ridge:", drying(500, 660))
print("valley:", drying(500, 540))

--@ chunk 32
-- The young crescent moon, upper right
moon_core = pile{{"lead white", 7}, {"yellow ochre", 0.3}, medium=0.12}
moon_halo = pile{{"lead white", 5}, {"smalt", 0.4}, {"yellow ochre", 0.2}, medium=0.55}

-- 1) a breath of radiance around it
m_halo = mask(function(x, y)
  local dx = (x - 700) / 30
  local dy = (y - 145) / 30
  return clamp(1 - (dx*dx + dy*dy), 0, 1)
end)
cov_halo = function(x, y)
  local dx = (x - 700) / 30
  local dy = (y - 145) / 30
  return clamp(1 - (dx*dx + dy*dy), 0, 1) * 0.45
end
stipple(m_halo, {pile=moon_halo, width=2.4, coverage=cov_halo, feather=0.7})

-- 2) earthshine: the whole disc barely present
disc = ellipse(700, 145, 7.5, 7.5):soften(1.2)
stipple(disc, {pile=moon_halo, width=2, coverage=0.4, feather=0.8})

-- 3) the bright crescent: lit belly toward the set sun (down-left), horns up-right
cres = (ellipse(700, 145, 7.5, 7.5) - ellipse(701.8, 143.5, 7.6, 7.6)):soften(0.7)
work(cres, {hand="detail", pile=moon_core, coverage=2.4, fill=true})

-- 4) one small companion star, upper left of the moon
sb7 = brush{kind="round", width=1.6, point=0.8, stiffness=0.5}
sb7:load(moon_core, 0.7)
sb7:touch(636, 102, {pressure=0.5})
print("moon and star set")

--@ chunk 33
-- Two spruces at right, against the pale hills: comb-spruce habit —
-- whorled branches, upper ones ascending, lower drooping, fringed tips.
fir_dark = pile{{"bone black", 2}, {"raw umber", 1}, {"green earth", 1.2}, {"Prussian blue", 0.3}, medium=0.1}
fir_mid  = pile{{"bone black", 1}, {"raw umber", 1.2}, {"green earth", 1}, {"yellow ochre", 0.4}, medium=0.1}
fir_rim  = pile{{"raw umber", 1}, {"yellow ochre", 0.7}, {"lead white", 0.9}, {"green earth", 0.4}, medium=0.12}

fb = brush{kind="rigger", width=2.8, point=1, stiffness=0.5, length=20}
ft = brush{kind="rigger", width=1.6, point=1, stiffness=0.5, length=12}

branches_drawn = {}

function bez(p0, p1, p2, t)
  local u = 1 - t
  return u*u*p0[1] + 2*u*t*p1[1] + t*t*p2[1],
         u*u*p0[2] + 2*u*t*p1[2] + t*t*p2[2]
end

function fir(bx, by, ty, hw, lean, spacing)
  -- leader
  fb:reload(fir_dark, 0.9)
  fb:stroke({{bx, by}, {bx + lean * 0.5, (by + ty) / 2}, {bx + lean, ty}},
    {pressure={0.8, 0.12}, ramps={0.05, 0.25}, orient="along"})
  local levels = math.floor((by - ty) / spacing)
  for i = 0, levels do
    local t = i / levels
    local y = by - t * (by - ty) + rand(-1.5, 1.5)
    local wmax = hw * (1 - t) ^ 0.85 + 1.5
    for side = -1, 1, 2 do
      local w = wmax * rand(60, 102) / 100
      if w > 2.5 then
        local droop = w * (0.40 * (1 - t) + 0.05)
        local sx = bx + lean * t + rand(-1, 1)
        local p0 = {sx, y}
        local p1 = {sx + side * w * 0.5, y + droop * 0.3}
        local p2 = {sx + side * w, y + droop}
        fb:reload(fir_dark, rand(55, 85) / 100)
        fb:stroke({p0, p1, p2}, {pressure={0.75, 0.05}, ramps={0.06, 0.6}, orient="along"})
        branches_drawn[#branches_drawn + 1] = {p0, p1, p2, t, side, w, droop}
      end
    end
  end
  -- the tip's young shoots, ascending
  for j = 1, 6 do
    local ang = -math.pi / 2 + rand(-50, 50) / 100
    local len = rand(30, 55) / 10
    ft:reload(fir_dark, 0.8)
    ft:stroke({{bx + lean, ty + 2}, {bx + lean + len * math.cos(ang), ty + 2 + len * math.sin(ang)}},
      {pressure={0.5, 0.05}, ramps={0.08, 0.7}, orient="along"})
  end
end

fir(806, 612, 352, 17, 3, 5.6)
fir(868, 630, 410, 13, -2, 5.2)

-- hanging comb fringe under the outer arms of the lower branches
for _, br in ipairs(branches_drawn) do
  local p0, p1, p2, t, side = br[1], br[2], br[3], br[4], br[5]
  if t < 0.55 and br[6] > 6 then
    local nfr = math.random(2, 4)
    for k = 1, nfr do
      local tt = rand(45, 96) / 100
      local px, py = bez(p0, p1, p2, tt)
      local hl = rand(25, 70) / 10 * (1 - t * 0.6)
      ft:reload(fir_dark, rand(50, 80) / 100)
      ft:stroke({{px, py}, {px + rand(-8, 8) / 10, py + hl}},
        {pressure={0.55, 0.05}, ramps={0.08, 0.7}, orient="along"})
    end
  end
end
print("fir frames:", #branches_drawn, "branches")

--@ chunk 34
-- Knit the needle mass into each fir: dense at the leader, thinning outward
fir1_sil = poly({{791,612},{787,590},{794,572},{788,552},{796,534},{791,514},{798,496},{793,476},{800,458},{796,438},{802,420},{799,400},{804,382},{803,368},{806,352},{809,370},{812,388},{810,406},{815,424},{812,444},{818,462},{814,482},{820,500},{815,522},{822,542},{817,562},{823,580},{818,600},{821,612}}, true):soften(2)
fir2_sil = poly({{856,630},{853,612},{859,598},{854,582},{860,566},{856,548},{862,534},{858,518},{863,504},{860,488},{864,474},{861,458},{865,444},{863,430},{867,416},{868,410},{871,424},{869,440},{874,454},{871,470},{876,486},{872,502},{878,518},{874,534},{880,550},{875,566},{881,584},{877,602},{880,618},{878,630}}, true):soften(2)

cov1 = function(x, y)
  local lx = 806 + 3 * (612 - y) / 260
  return clamp(1.7 - math.abs(x - lx) / 12, 0.3, 1.7)
end
cov2 = function(x, y)
  local lx = 868 - 2 * (630 - y) / 220
  return clamp(1.7 - math.abs(x - lx) / 9.5, 0.3, 1.7)
end

stipple(fir1_sil, {pile=fir_dark, width=2.2, coverage=cov1, feather=0.6, cluster={0.35, 6}})
stipple(fir2_sil, {pile=fir_dark, width=2.2, coverage=cov2, feather=0.6, cluster={0.35, 6}})
-- a breath of warmer mid-tone deep inside, where the afterglow models them
stipple(fir1_sil, {pile=fir_mid, width=2.4, coverage=function(x, y) return cov1(x, y) * 0.3 end, feather=0.7, cluster={0.4, 7}})
stipple(fir2_sil, {pile=fir_mid, width=2.4, coverage=function(x, y) return cov2(x, y) * 0.3 end, feather=0.7, cluster={0.4, 7}})
print("needle mass knitted")

--@ chunk 35
-- Restate the branch plates over the mass: stepped spruce silhouette,
-- then a faint warm rim on the afterglow side.
function plates(bx, by, ty, hw, lean, spacing)
  local levels = math.floor((by - ty) / spacing)
  for i = 0, levels do
    local t = i / levels
    local y = by - t * (by - ty) + rand(-1.5, 1.5)
    local wmax = (hw * (1 - t) ^ 0.85 + 1.5) * 1.12
    for side = -1, 1, 2 do
      if math.random() < 0.8 then
        local w = wmax * rand(60, 100) / 100
        if w > 3 then
          local droop = w * (0.34 * (1 - t) + 0.05)
          local sx = bx + lean * t + rand(-1, 1)
          fb:reload(fir_dark, rand(45, 70) / 100)
          fb:stroke({{sx, y}, {sx + side * w * 0.5, y + droop * 0.3}, {sx + side * w, y + droop}},
            {pressure={0.5, 0.04}, ramps={0.06, 0.65}, orient="along"})
        end
      end
    end
  end
end
plates(806, 612, 352, 17, 3, 7)
plates(868, 630, 410, 13, -2, 6.5)

-- warm rim: sparse touches along the left (afterglow) side of plates
function rimfir(bx, by, ty, hw, lean, spacing)
  local levels = math.floor((by - ty) / spacing)
  for i = 0, levels, 2 do
    local t = i / levels
    local y = by - t * (by - ty) + rand(-1.5, 1.5)
    local w = (hw * (1 - t) ^ 0.85 + 1.5) * rand(75, 105) / 100
    if w > 3.5 then
      local ex = bx + lean * t - w * 0.92
      local ey = y + w * 0.3 * (0.34 * (1 - t) + 0.05) / 0.4
      ft:reload(fir_rim, rand(25, 45) / 100)
      ft:stroke({{ex + 3, ey - 1}, {ex - 1.5, ey + 0.5}},
        {pressure={0.4, 0.05}, ramps={0.1, 0.7}, orient="along"})
    end
  end
  -- and a breath up the leader's left edge
  ft:reload(fir_rim, 0.35)
  ft:stroke({{bx - 1, by - 8}, {bx + lean * 0.5 - 1, (by + ty) / 2}, {bx + lean - 0.5, ty + 6}},
    {pressure={0.35, 0.05}, ramps={0.08, 0.5}, orient="along"})
end
rimfir(806, 612, 352, 17, 3, 9)
rimfir(868, 630, 410, 13, -2, 8.5)
print("plates and rims done")

--@ chunk 36
-- Two figures on the crest, seen from behind, contemplating the moon
fig_dark = pile{{"bone black", 2}, {"raw umber", 1.2}, {"smalt", 0.2}, medium=0.08}
fig_rim  = pile{{"raw umber", 1}, {"yellow ochre", 0.6}, {"lead white", 1.2}, medium=0.1}

-- A: the man, upright, long coat and broad-brimmed hat, a walking stick
figA = poly({{446.5,546},{449.5,542},{456,542},{459,546},{457.5,547.5},{457,552},{458.5,556.5},{460,561},{461,570},{462,589},{455,589},{454.5,581},{452.5,581},{452,589},{444,589},{445.5,572},{446.5,562},{447.5,557},{448,552},{448.3,549}}, true)
-- B: the woman beside him, hooded cloak, inclined gently toward him
figB = poly({{472.5,550},{476.5,547.5},{480.5,549},{483,552.5},{484.5,557},{486,563},{487.5,572},{489,589},{469,589},{470,575},{470.5,566},{471.5,559},{472,553}}, true)

work(figA, {hand="detail", pile=fig_dark, coverage=2.6, fill=true})
work(figB, {hand="detail", pile=fig_dark, coverage=2.6, fill=true})

fg = brush{kind="rigger", width=1.5, point=0.9, stiffness=0.55, length=12}
-- hat brim, a little wider than the crown
fg:load(fig_dark, 0.8)
fg:stroke({{445,546.5},{452.5,547},{460,546.5}}, {pressure={0.45,0.4}, ramps={0.15,0.3}, orient="along"})
-- his stick, planted ahead of him
fg:reload(fig_dark, 0.7)
fg:stroke({{460.5,563},{463,578},{464.5,591}}, {pressure={0.4,0.15}, ramps={0.1,0.5}, orient="along"})
-- the warm afterglow catching their left contours
fg:reload(fig_rim, 0.4)
fg:stroke({{446.6,545.5},{448.2,549}}, {pressure={0.35,0.1}, ramps={0.15,0.5}, orient="along"})   -- hat edge
fg:reload(fig_rim, 0.35)
fg:stroke({{448,553},{446.8,560},{445.8,568}}, {pressure={0.35,0.08}, ramps={0.15,0.55}, orient="along"}) -- shoulder/coat
fg:reload(fig_rim, 0.3)
fg:stroke({{472.6,550.5},{471.8,557},{470.8,565}}, {pressure={0.32,0.08}, ramps={0.15,0.55}, orient="along"}) -- her hood and shoulder
-- feet anchored: two small shadows on the crest
fg:reload(fig_dark, 0.6)
fg:stroke({{443.5,589.5},{453,590},{462.5,589.5}}, {pressure={0.35,0.2}, ramps={0.2,0.4}, orient="along"})
fg:stroke({{469,589.5},{479,590},{489,589.5}}, {pressure={0.35,0.2}, ramps={0.2,0.4}, orient="along"})
print("figures standing")

--@ chunk 37
-- Repair the figures: press dark back in with touches, no dragging
fb2 = brush{kind="round", width=2.2, point=0.6, stiffness=0.6}
-- A's head and hat
fb2:load(fig_dark, 0.85)
fb2:touch(452.5, 548.5, {pressure=0.65})
fb2:touch(451.5, 551.5, {pressure=0.6})
fb2:touch(450.5, 544.5, {pressure=0.55})
fb2:touch(454, 544, {pressure=0.55})
-- his shoulders and the specks down the coat
fb2:touch(449.5, 557, {pressure=0.55})
fb2:touch(457.5, 558, {pressure=0.55})
fb2:touch(450, 566, {pressure=0.5})
fb2:touch(456, 572, {pressure=0.5})
fb2:touch(449, 578, {pressure=0.5})
fb2:touch(458, 583, {pressure=0.5})
-- B's hood top and shoulder
fb2:touch(476.5, 549, {pressure=0.6})
fb2:touch(479.5, 550.5, {pressure=0.55})
fb2:touch(473.5, 556, {pressure=0.55})
fb2:touch(482, 558, {pressure=0.5})
fb2:touch(474, 568, {pressure=0.5})
fb2:touch(481, 574, {pressure=0.5})
fb2:touch(476, 582, {pressure=0.5})
-- restate the hat crown and brim, thin, in the fresh dark
fg:reload(fig_dark, 0.75)
fg:stroke({{449.5,543.5},{452.5,542.8},{455.5,543.5}}, {pressure={0.4,0.35}, ramps={0.15,0.3}, orient="along"})
fg:stroke({{446,546.8},{452.5,547.2},{459,546.8}}, {pressure={0.35,0.3}, ramps={0.15,0.3}, orient="along"})
print("figures repaired")

--@ chunk 38
-- Press the head zones solid with clipped detail work
headA = poly({{446,547},{448.5,542.5},{456.5,542.5},{459,547},{457.8,549},{457.5,553},{448.3,553},{448,549}}, true)
hoodB = poly({{472.5,550.5},{475,547.8},{480.5,549.2},{482.5,552.5},{483.5,556},{471.8,556},{472,552.5}}, true)
work(headA, {hand="detail", pile=fig_dark, coverage=3.2, fill=true})
work(hoodB, {hand="detail", pile=fig_dark, coverage=3.2, fill=true})
print("heads solid")

--@ chunk 39
-- 1) Mute the path: it should be a worn trace, not a yellow road
path_zone = ribbon({{512, 588}, {540, 622}, {585, 660}, {640, 700}, {668, 714}}, {9, 11, 14, 18, 22}):soften(8)
work(path_zone, {hand="scumble", pile=ridge_dark, angle=1.1, coverage=1.9, fill=true, edge=0.5})
-- a faint worn thread remains, dry-brushed pale along its upper course
worn = pile{{"raw umber", 2}, {"yellow ochre", 0.8}, {"lead white", 0.8}, medium=0.1}
wb = brush("filbert", 5)
wb:load(worn, 0.35)
wb:stroke({{515, 591}, {538, 618}, {562, 642}}, {pressure={0.3, 0.15}, ramps={0.2, 0.5}, orient="along"})

-- 2) Knock back the yellow flecks inside the firs
cov1k = function(x, y)
  local lx = 806 + 3 * (612 - y) / 260
  return clamp(0.9 - math.abs(x - lx) / 7, 0, 0.9)
end
cov2k = function(x, y)
  local lx = 868 - 2 * (630 - y) / 220
  return clamp(0.9 - math.abs(x - lx) / 6, 0, 0.9)
end
stipple(fir1_sil, {pile=fir_dark, width=2.2, coverage=cov1k, feather=0.6, cluster={0.3, 6}})
stipple(fir2_sil, {pile=fir_dark, width=2.2, coverage=cov2k, feather=0.6, cluster={0.3, 6}})

-- 3) Quiet the moon's halo speckle with the surrounding sky tone
sky_hush = pile{{"lead white", 6}, {"cobalt blue", 0.7}, {"smalt", 0.5}, medium=0.5}
m_hush = (rect(590, 85, 190, 130):soften(28) - ellipse(700, 145, 11, 11):grow(3)):soften(3)
cov_hush = function(x, y)
  local dx = (x - 700) / 85
  local dy = (y - 145) / 60
  return clamp(1 - (dx*dx + dy*dy), 0, 1) * 1.1
end
stipple(m_hush, {pile=sky_hush, width=3, coverage=cov_hush, feather=0.6})
print("path muted, firs quieted, halo hushed")

--@ chunk 40
-- Veil the moon's quarter of sky with a thin continuous film to quiet the grain
sky_veil = pile{{"lead white", 6}, {"cobalt blue", 0.5}, {"smalt", 0.4}, {"raw umber", 0.1}, medium=0.45}
m_veil = (rect(585, 78, 205, 145):soften(30) - ellipse(700, 145, 10.5, 10.5):grow(5)):soften(2)
work(m_veil, {hand="glaze", pile=sky_veil, angle=0.05, coverage=2.2, fill=true})
print("sky veiled")

--@ chunk 41
-- Rebuild the sky in the damaged quadrant, working into the open veil
sky_fix_top = pile{{"lead white", 6}, {"cobalt blue", 0.8}, {"smalt", 0.5}, {"bone black", 0.12}, medium=0.25}
sky_fix_mid = pile{{"lead white", 6.5}, {"cobalt blue", 0.4}, {"smalt", 0.3}, {"raw umber", 0.12}, medium=0.25}

zone_top = rect(548, 52, 260, 115):soften(28)
zone_mid = rect(548, 128, 260, 108):soften(28)

work(zone_top, {hand="body", pile=sky_fix_top, angle=0.02, coverage=2.3, fill=true, curve={4, 2}, edge=0.55})
work(zone_mid, {hand="body", pile=sky_fix_mid, angle=-0.01, coverage=2.3, fill=true, curve={3, 2}, edge=0.55})
blend(rect(548, 52, 260, 184):soften(22), {angle=0})
print("quadrant rebuilt")

--@ chunk 42
-- Repaint the whole upper sky band, full width, in the original day-1 tones
sky_top2 = pile{{"lead white", 5}, {"cobalt blue", 1.2}, {"smalt", 0.8}, {"bone black", 0.25}, medium=0.2}
sky_mid2 = pile{{"lead white", 6}, {"cobalt blue", 0.5}, {"smalt", 0.4}, {"raw umber", 0.15}, medium=0.2}

band_top = mask(function(x, y) return clamp((185 - y) / 150, 0, 1) end)
band_mid = mask(function(x, y) return clamp(1 - math.abs(y - 165) / 95, 0, 1) end)

work(band_top, {hand="body", pile=sky_top2, angle=0.02, coverage=2.3, fill=true, curve={4, 2}})
work(band_mid, {hand="body", pile=sky_mid2, angle=0, coverage=2.3, fill=true, curve={4, 2}})
blend(rect(0, 0, 1000, 265):soften(18), {angle=0})
print("upper sky repainted full width")

--@ chunk 43
-- Disperse the drag trails around the crown while the band is open
blend(rect(35, 145, 460, 140):soften(22), {angle=0})
print("trails dispersed")

--@ chunk 44
print(wait(180))
print("sky at moon:", drying(700, 145))
print("sky in crown zone:", drying(200, 250))
print("limb A:", drying(200, 290))
print("leader:", drying(315, 240))

--@ chunk 45
print(wait(360))
print("sky at moon:", drying(700, 145))
print("sky in crown zone:", drying(200, 250))
print("limb A:", drying(200, 290))
print("leader:", drying(315, 240))

--@ chunk 46
-- Restate the crown: upper limbs, dead leader, twig net, bleached deadwood
rb1 = brush{kind="rigger", width=3.6, point=1, stiffness=0.55, length=22}
local upper_limbs = {
  {{{214,292},{190,250},{168,222},{150,200}}, 3.4},
  {{{160,252},{135,225},{112,210}}, 2.8},
  {{{128,232},{100,212},{78,198},{64,186}}, 2.6},
  {{{262,340},{230,330},{205,322}}, 2.6},
  {{{350,352},{385,318},{420,295},{445,280}}, 3.2},
  {{{408,306},{445,285},{478,272},{500,262}}, 2.6},
  {{{452,262},{478,240},{505,225},{520,212}}, 2.4},
  {{{306,330},{312,262},{322,200}}, 4.0},
  {{{306,330},{330,300},{348,278}}, 2.8},
  {{{312,262},{295,235},{288,212}}, 2.4},
  {{{322,200},{330,178},{334,160}}, 2.0},
  {{{312,262},{335,240},{352,225}}, 2.2},
}
for _, br in ipairs(upper_limbs) do
  local pts, w = br[1], br[2]
  rb1:reload(bark_dark, 0.85)
  rb1:stroke(pts, {pressure={0.85 * w / 3.6, 0.06}, ramps={0.06, 0.7}, orient="along"})
end

-- twig net, upper crown only
rb2 = brush{kind="rigger", width=1.7, point=1, stiffness=0.5, length=14}
local cx0, cy0 = 300, 430
local clusters = {
  {150,200,false},{112,210,false},{64,186,false},
  {445,280,false},{500,262,false},{520,212,false},
  {348,278,true},{288,212,true},{334,160,true},{352,225,true},
  {214,292,false},{160,252,false},
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

-- bleach the upper deadwood again
rb4 = brush{kind="rigger", width=2.2, point=0.8, stiffness=0.55, length=16}
local deadwood_up = {
  {{306,330},{312,262},{322,200}},
  {{306,330},{330,300},{348,278}},
  {{312,262},{295,235},{288,212}},
  {{322,200},{330,178},{334,160}},
  {{312,262},{335,240},{352,225}},
}
for _, p in ipairs(deadwood_up) do
  rb4:reload(dead_pale, 0.55)
  rb4:stroke(p, {pressure={0.5, 0.3}, ramps={0.1, 0.4}, orient="along"})
end

-- the crescent and its star, restated crisp on the setting sky
cres2 = (ellipse(700, 145, 7.5, 7.5) - ellipse(701.8, 143.5, 7.6, 7.6)):soften(0.7)
work(cres2, {hand="detail", pile=moon_core, coverage=2.4, fill=true})
sb7:load(moon_core, 0.7)
sb7:touch(636, 102, {pressure=0.5})
print("crown restated, moon reset")

--@ chunk 47
deadwood_up = {
  {{306,330},{312,262},{322,200}},
  {{306,330},{330,300},{348,278}},
  {{312,262},{295,235},{288,212}},
  {{322,200},{330,178},{334,160}},
  {{312,262},{335,240},{352,225}},
}
local cores = {
  {{{262,340},{214,292},{160,252},{128,232}}, 4.2},
  {{{214,292},{190,250},{168,222},{150,200}}, 3.6},
  {{{160,252},{135,225},{112,210}}, 3.0},
  {{{128,232},{100,212},{78,198},{64,186}}, 2.8},
  {{{352,420},{352,352},{408,306},{452,262}}, 4.0},
  {{{350,352},{385,318},{420,295},{445,280}}, 3.4},
  {{{408,306},{445,285},{478,272},{500,262}}, 2.8},
  {{{452,262},{478,240},{505,225},{520,212}}, 2.5},
  {{{300,395},{306,330},{312,262},{322,200}}, 4.4},
}
for _, br in ipairs(cores) do
  local pts, w = br[1], br[2]
  rb1:reload(bark_dark, 0.95)
  rb1:stroke(pts, {pressure={math.min(0.95 * w / 3.6, 1), 0.08}, ramps={0.05, 0.65}, orient="along"})
end

math.randomseed(1331)
local clusters2 = {
  {150,200,false},{112,210,false},{64,186,false},{205,322,false},{262,340,false},
  {445,280,false},{500,262,false},{520,212,false},{468,312,false},{408,306,false},
  {348,278,true},{288,212,true},{334,160,true},{352,225,true},{306,330,true},
  {214,292,false},{160,252,false},{350,352,false},{128,232,false},{452,262,false},
}
for _, c in ipairs(clusters2) do
  local cx, cy, dead = c[1], c[2], c[3]
  local out = math.atan(cy - cy0, cx - cx0)
  local n = dead and math.random(2, 3) or math.random(3, 6)
  rb2:reload(dead and dead_pale or bark_dark, 0.9)
  for j = 1, n do
    local ang = out + (math.random() - 0.5) * 1.8 - 0.15
    local len = dead and (3 + math.random() * 4) or (4 + math.random() * 6)
    local tx = cx + len * math.cos(ang)
    local ty = cy + len * math.sin(ang)
    rb2:stroke({{cx, cy}, {tx, ty}}, {pressure={0.55, 0.05}, ramps={0.08, 0.7}, orient="along"})
    if math.random() < 0.5 then
      local ang2 = ang + (math.random() - 0.5) * 1.1
      local len2 = 2.5 + math.random() * 3.5
      rb2:stroke({{tx, ty}, {tx + len2 * math.cos(ang2), ty + len2 * math.sin(ang2)}}, {pressure={0.4, 0.04}, ramps={0.08, 0.7}, orient="along"})
    end
  end
end
for _, p in ipairs(deadwood_up) do
  rb4:reload(dead_pale, 0.5)
  rb4:stroke(p, {pressure={0.45, 0.25}, ramps={0.1, 0.45}, orient="along"})
end
print("crown strengthened")

--@ chunk 48
print(wait(420))
print("crown sky:", drying(200, 250))
print("halo at limb A:", drying(170, 245))
print("moon:", drying(700, 145))
print("leader halo:", drying(345, 235))

--@ chunk 49
-- Quiet the smear halos: pearl-sky stipple over the smudged zones,
-- sparing the branch corridors so limbs and twigs keep their dark.
pearl_sky = pile{{"lead white", 6}, {"cobalt blue", 0.35}, {"smalt", 0.3}, {"raw umber", 0.12}, {"yellow ochre", 0.12}, medium=0.3}

local P1 = {{262,340},{214,292},{160,252},{128,232}}
local P2 = {{214,292},{190,250},{168,222},{150,200}}
local P3 = {{160,252},{135,225},{112,210}}
local P4 = {{128,232},{100,212},{78,198},{64,186}}
local P5 = {{294,402},{350,352},{408,306},{452,262}}
local P6 = {{350,352},{385,318},{420,295},{445,280}}
local P7 = {{408,306},{445,285},{478,272},{500,262}}
local P8 = {{452,262},{478,240},{505,225},{520,212}}
local P9 = {{300,395},{306,330},{312,262},{322,200}}
local S1 = {{306,330},{330,300},{348,278}}
local S2 = {{312,262},{295,235},{288,212}}
local S3 = {{322,200},{330,178},{334,160}}
local S4 = {{312,262},{335,240},{352,225}}

corridor = ribbon(P1,7)+ribbon(P2,7)+ribbon(P3,7)+ribbon(P4,7)
  + ribbon(P5,7)+ribbon(P6,7)+ribbon(P7,7)+ribbon(P8,7)
  + ribbon(P9,6.5)+ribbon(S1,5)+ribbon(S2,5)+ribbon(S3,5)+ribbon(S4,5)
corridor = corridor:soften(2)

Z1 = poly({{55,205},{140,190},{235,225},{330,265},{335,315},{240,305},{140,275},{60,250}}, true)
Z2 = poly({{330,255},{400,215},{490,190},{540,200},{520,250},{450,285},{360,300},{320,285}}, true)
Z3 = poly({{285,175},{325,150},{355,185},{365,250},{350,320},{315,330},{290,260}}, true)
halo_zone = ((Z1 + Z2 + Z3):soften(14) - corridor):soften(1.5)

stipple(halo_zone, {pile=pearl_sky, width=2.4, coverage=0.85, feather=0.6, cluster={0.25, 8}})
print("halos quieted, first pass")

--@ chunk 50
-- Final touches on the figures and the star
fb2:load(fig_dark, 0.85)
fb2:touch(452, 543.5, {pressure=0.6})    -- his hat crown
fb2:touch(450, 545.5, {pressure=0.55})
fb2:touch(477, 548.3, {pressure=0.55})   -- her hood point
fb2:touch(451, 562, {pressure=0.5})      -- fleck on his back
fb2:touch(479, 564, {pressure=0.5})      -- fleck on her cloak
-- the star, restated a little stronger
sb7:load(moon_core, 0.9)
sb7:touch(640, 100, {pressure=0.6})
sb7:touch(640, 100, {pressure=0.4})
print("final touches done")

--@ chunk 51
-- Sink the bottom corners a touch to hold the eye
cornerL = ellipse(0, 720, 320, 130):soften(45)
cornerR = ellipse(1000, 720, 320, 130):soften(45)
corners = cornerL + cornerR
work(corners, {hand="scumble", pile=ridge_dark, angle=0.15, coverage=1.3, fill=true, edge=0.7})
print("corners sunk")

--@ chunk 52
-- Dissolve the scalloped lobe edges into a gradual corner sink
cornerL2 = ellipse(0, 724, 390, 155):soften(55)
cornerR2 = ellipse(1000, 724, 390, 155):soften(55)
cov_c = function(x, y)
  return 1.1
end
stipple(cornerL2 + cornerR2, {pile=ridge_dark, width=3, coverage=1.1, feather=0.65, cluster={0.3, 8}})
print("corners softened into gradient")

--@ chunk 53
-- A breath of afterglow on their left contours, pressed on dry mist
rb6 = brush{kind="round", width=1.5, point=0.7, stiffness=0.6}
rb6:load(fig_rim, 0.5)
rb6:touch(447.2, 547, {pressure=0.3})   -- his hat brim left
rb6:touch(447.8, 553.5, {pressure=0.28}) -- his temple/shoulder left
rb6:touch(446.8, 561, {pressure=0.26})   -- his coat left
rb6:touch(472.2, 550.5, {pressure=0.28}) -- her hood left
rb6:touch(471, 558, {pressure=0.26})     -- her shoulder left
print("rims breathed on")

--@ chunk 54
print("corridor zone:", drying(200, 250), drying(420, 240))
print("mist by spruces:", drying(880, 540), drying(840, 500))
print("path scratch:", drying(540, 620))
print("leader tip:", drying(330, 158))
print("ridge:", drying(500, 660))
print("sky top:", drying(500, 80))
print("moon:", drying(700, 145))

--@ chunk 55
-- Knit the crown-zone speckle into one quiet haze stratum.
-- Mid pearl-gray, a shade deeper than the white dots, so they stay as its lights.
haze = pile{{"lead white", 4.5}, {"cobalt blue", 0.45}, {"smalt", 0.5}, {"raw umber", 0.3}, {"yellow ochre", 0.12}, medium=0.35}

m_band = mask(function(x, y)
  local vert = clamp(1 - math.abs(y - 252) / 98, 0, 1)
  local horiz = clamp(1 - math.abs(x - 290) / 350, 0, 1)
  return vert * horiz
end)
m_haze = (m_band:soften(16) - corridor:grow(1)):soften(1.5)

cov_haze = function(x, y) return m_haze:at(x, y) * 1.9 end
stipple(m_haze, {pile=haze, width=2.6, coverage=cov_haze, feather=0.55, cluster={0.15, 9}, dips={80, 0.65, 0.4}})
print("haze stratum knitted")

--@ chunk 56
-- Knit the sparse mist flecks over the right hill into a thin continuous veil.
mist_mid = pile{{"lead white", 4}, {"smalt", 0.7}, {"green earth", 0.3}, {"raw umber", 0.2}, medium=0.4}

m_mistfix = (rect(742, 452, 258, 208):soften(24) - fir1_sil:grow(2.5) - fir2_sil:grow(2.5)):soften(1.5)
cov_mf = function(x, y) return m_mistfix:at(x, y) * 1.25 end
stipple(m_mistfix, {pile=mist_mid, width=2.6, coverage=cov_mf, feather=0.6, cluster={0.2, 8}, dips={80, 0.65, 0.4}})
print("right-hill mist knitted")

--@ chunk 57
-- Break the path scratch into a worn trace: pressed ridge-dark touches, gaps left.
pb = brush{kind="round", width=3, point=0.5, stiffness=0.6}
pb:load(ridge_dark, 0.8)
pb:touch(520, 590, {pressure=0.55})
pb:touch(524, 600, {pressure=0.6})
pb:touch(530, 613, {pressure=0.55})
pb:touch(535, 626, {pressure=0.6})
pb:touch(540, 638, {pressure=0.55})
pb:touch(543, 645, {pressure=0.5})
-- soften its edges with two half-pressure touches beside the line
pb:touch(527, 606, {pressure=0.35})
pb:touch(538, 632, {pressure=0.35})

-- Dissolve the pale smear round the dead leader's tip: sky-tone touches beside the twig.
sky_tip = pile{{"lead white", 6}, {"cobalt blue", 0.5}, {"smalt", 0.4}, {"raw umber", 0.15}, medium=0.3}
tb = brush{kind="round", width=2.2, point=0.6, stiffness=0.6}
tb:load(sky_tip, 0.75)
tb:touch(326, 150, {pressure=0.5})
tb:touch(323, 157, {pressure=0.5})
tb:touch(328, 145, {pressure=0.45})
tb:touch(339, 149, {pressure=0.5})
tb:touch(341, 156, {pressure=0.45})
tb:touch(337, 143, {pressure=0.4})
tb:touch(324, 163, {pressure=0.45})
tb:touch(340, 163, {pressure=0.4})
print("path broken, leader tip quieted")
