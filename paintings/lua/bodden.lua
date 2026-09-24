-- easel session "bodden": a painting replayed chunk by chunk.
--   easel run paintings/lua/bodden.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.45, seed=23, hand=true}; print(W, H); print(pal)

--@ chunk 2 · clock 0
-- the drawing: a level horizon, the far shore and its town, the near bank
HZ = 432
G = function(x, c, s) return math.exp(-((x - c) / s)^2) end
sitting{hours=3}
-- far shore profile (land top), a low tongue with the town at the middle
SHORE = {{-5,433},{40,431},{95,428},{150,426},{200,424},{236,423},{300,421},{360,420},{420,419},{470,418},
         {530,418},{590,419},{650,420},{700,421},{760,423},{830,425},{900,427},{960,429},{1005,430}}
-- the town, drawn as a silhouette line (left to right)
TOWN = {{366,421},{372,415},{380,412},{390,412},{396,411},{401,406},{406,411},{412,410},{440,409},{446,403},
        {452,409},{470,408},{472,403},{488,403},{488,370},{491,369},{491,363},{500,330},{509,363},{509,369},
        {512,370},{512,403},{540,403},{546,407},{560,408},{562,398},{576,398},{576,389},{578,380},{580,389},
        {580,398},{596,398},{600,406},{606,407},{612,401},{618,407},{632,409},{640,410},{652,412},{664,417},{674,420}}
MILL = {x=236, base=423, top=404}
BANK = {}
for x = -10, 1010, 30 do
  BANK[#BANK+1] = {x, 626 - 34*G(x, 90, 190) - 26*G(x, 930, 150) + 6*math.sin(x/47)}
end
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.3})
h:sketch(SHORE, {pressure=0.3})
h:sketch(BANK, {pressure=0.3})
b = pencil("3B")
b:line(TOWN, {pressure={0.55, 0.65, 0.6}, smooth=false})
b:rule({500, 334}, {500, 424}, {pressure=0.35})   -- the axis through the tower
b:line({{MILL.x-4, MILL.base}, {MILL.x-3, MILL.top}, {MILL.x+3, MILL.top}, {MILL.x+4, MILL.base}}, {pressure=0.55, smooth=false})
b:line({{MILL.x-14, MILL.top-12}, {MILL.x+14, MILL.top+12}}, {pressure=0.5})
b:line({{MILL.x+13, MILL.top-11}, {MILL.x-13, MILL.top+11}}, {pressure=0.5})
MILL2 = {x=812, base=425, top=412}
b:line({{MILL2.x-3, MILL2.base}, {MILL2.x-2, MILL2.top}, {MILL2.x+2, MILL2.top}, {MILL2.x+3, MILL2.base}}, {pressure=0.5, smooth=false})
b:line({{MILL2.x-9, MILL2.top-6}, {MILL2.x+9, MILL2.top+6}}, {pressure=0.45})
b:line({{MILL2.x+6, MILL2.top-9}, {MILL2.x-6, MILL2.top+9}}, {pressure=0.45})
b:line(BANK, {pressure={0.5, 0.6, 0.5}})
local t = timesheet(); print(string.format("sitting %.0f min", t.sitting))

--@ chunk 3 · clock 1.8615186177194118
-- a thin brown lay-in of the values, like a sepia wash: the bank darkest, the water darker toward me
sepia = pal:only{"raw umber", "bone black", "yellow ochre", "lead white"}
wob = noise{seed=4, period=90, octaves=3}
bankcurve = function(x)
  return 626 - 34*G(x, 90, 190) - 26*G(x, 930, 150) + 6*math.sin(x/47)
end
bankM = below(function(x) return bankcurve(x) - 3 end):roughen(2, 14, 5, 1.5)
work(bankM, {hand="broad", tool="flat 10", color="#4a3a2a", angle=function(x, y) return 0.06*wob(x, y) end,
  length={30, 90}, coverage=2.4, medium=0.7, load=0.5, pal=sepia, hug=false})
blend(bankM, {angle=0})
local t = timesheet(); print(string.format("sitting %.0f min", t.sitting))

--@ chunk 4 · clock 24.21120547875762
-- the sky, first layer: six piles mixed by knife, laid in bands top down, wet into wet, a shade dull
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth", "vermilion", "chrome yellow"}
shoreline = function(x) return 433 - 13*G(x, 500, 330) end
skyM = above(function(x) return shoreline(x) + 6 end)
PILES = {{0, "#525e78"}, {100, "#66748e"}, {185, "#8592a3"}, {250, "#a9aea9"}, {305, "#c6c2a6"}, {355, "#d9cb9e"}, {402, "#e3c28e"}}
rag = noise{seed=12, period=70, octaves=3}
local dome = function(x) return 26 * G(x, 505, 260) end    -- the glow stands higher over the town
for i, p in ipairs(PILES) do
  local top = (i == 1) and -20 or (PILES[i-1][1] + p[1]) / 2
  local bot = (i == #PILES) and 480 or (p[1] + PILES[i+1][1]) / 2
  local lift = (i >= 5) and 1 or 0.4
  local band = mask(function(x, y)
    local yy = y + lift*dome(x) + 9*rag(x, y)
    return smoothstep(top - 14, top + 6, yy) * (1 - smoothstep(bot - 6, bot + 14, yy))
  end) * skyM
  work(band, {hand="broad", color=p[2], angle=function(x, y) return 0.025*rag(x*0.5, y) end,
    length={70, 200}, coverage=4.2, medium=0.3, pal=skypal, hug=false})
end
blend(skyM, {angle=0, length={80, 220}})
local t = timesheet(); print(string.format("sitting %.0f min", t.sitting))

--@ chunk 5 · clock 73.17522525414824
-- the water, first layer: the sky's piles again, more transparent and darker toward me, level strokes
waterpal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "bone black", "red earth"}
WPILES = {{436, "#d2bf94"}, {450, "#c0b69c"}, {470, "#a9aaa3"}, {500, "#8f98a0"}, {540, "#788392"}, {585, "#646f82"}, {635, "#535c6e"}}
waterTop = function(x) return shoreline(x) + 1 end
waterM = mask(function(x, y) return smoothstep(waterTop(x) - 2, waterTop(x) + 2, y) * (1 - smoothstep(bankcurve(x) + 4, bankcurve(x) + 12, y)) end)
ripple = noise{seed=21, period=120, octaves=3, stretch={0, 6}}
for i, p in ipairs(WPILES) do
  local top = (i == 1) and 400 or (WPILES[i-1][1] + p[1]) / 2
  local bot = (i == #WPILES) and 700 or (p[1] + WPILES[i+1][1]) / 2
  local soft = 3 + 0.12 * (p[1] - 430)
  local band = mask(function(x, y)
    local yy = y + 0.25 * soft * ripple(x, y)
    return smoothstep(top - soft, top + soft * 0.4, yy) * (1 - smoothstep(bot - soft * 0.4, bot + soft, yy))
  end) * waterM
  work(band, {hand="broad", color=p[2], angle=0, angle_jitter=0.004, length={200, 500}, coverage=4.0, medium=0.3, pal=waterpal, hug=false, curve={0.02, 0}, cross=0, drift={0, 1}})
end
blend(waterM, {angle=0, angle_jitter=0.002, length={200, 500}, curve={0.01, 0}, cross=0, drift={0, 1}})
local t = timesheet(); print(string.format("sitting %.0f min", t.sitting))

--@ chunk 6 · clock 85.99917685613036
-- next day: the sky's second layer, stippled, the glow zone first. Each touch from the pile of its band
-- (the painter alternates two piles where bands meet), dirtied with what is already there.
rest(16)
sitting{hours=5}
pileAt = function(x, y)
  local yy = y + G(x, 505, 260) * 26 * (y > 300 and 1 or 0.4) + 9 * rag(x, y)
  for i = 1, #PILES - 1 do
    local b = (PILES[i][1] + PILES[i+1][1]) / 2
    if yy < b + 14 then
      return mix(PILES[i][2], PILES[i+1][2], smoothstep(b - 14, b + 14, yy))
    end
  end
  return color(PILES[#PILES][2])
end
glowZone = skyM * mask(function(x, y) return smoothstep(215, 260, y) end)
stipple(glowZone, {width=4, coverage=function(x, y) return 1.5 * smoothstep(215, 265, y) end, fade=1,
  color_over=function(x, y, under) return shift(mix(under, pileAt(x, y), 0.45), 0.01, 0, 0.002) end,
  pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.5, pal=skypal})
local t = timesheet(); print(string.format("sitting %.0f min, touches %d", t.sitting, t.touches))

--@ chunk 7 · clock 1334.1368956528604
-- another day: the upper sky stippled the same way, overlapping the glow zone's top
rest(16)
sitting{hours=5}
upperZone = skyM * mask(function(x, y) return 1 - smoothstep(245, 290, y) end)
stipple(upperZone, {width=4.5, coverage=function(x, y) return 1.35 * (1 - smoothstep(240, 292, y)) end, fade=1,
  color_over=function(x, y, under) return shift(mix(under, pileAt(x, y), 0.45), 0.008, 0, 0) end,
  pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.5, pal=skypal})
local t = timesheet(); print(string.format("sitting %.0f min, touches %d", t.sitting, t.touches))

--@ chunk 8 · clock 2583.2638859711587
-- next day: three strata, dull violet-gray bodies lit from below by the set sun, laid in body color
-- over the dry stipple and blended while wet, their undersides touched in warm
rest(16)
sitting{hours=3}
cloudpal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth", "vermilion", "bone black"}
local function strandM(x0, x1, y, tilt, w)
  local pts, ws, n = {}, {}, math.max(4, math.floor((x1 - x0) / 25))
  for k = 0, n do
    local t = k / n
    pts[#pts+1] = {x0 + (x1 - x0) * t, y - tilt * t + randn(0, 0.8)}
    ws[#ws+1] = w * (0.25 + 0.75 * math.sin(math.pi * t)^0.6) * rand(0.7, 1.15)
  end
  return ribbon(pts, ws)
end
local function cloud(list, tilt, seed)
  local m
  for _, s in ipairs(list) do local r = strandM(s[1], s[2], s[3], tilt, s[4]); m = m and (m + r) or r end
  return m:roughen(1.2, 18, seed, 1.2)
end
cA = cloud({{60, 330, 78, 5}, {150, 520, 86, 7}, {230, 610, 92, 6}, {310, 470, 99, 4}, {520, 700, 88, 3}, {95, 260, 83, 4}, {400, 560, 95, 5}}, 5, 31)
cB = cloud({{560, 860, 168, 5}, {640, 990, 174, 6}, {720, 900, 179, 3}, {880, 1010, 166, 3}}, -4, 32)
local cn = noise{seed=41, period=60}
local function veil(m, body, warm, amt, seed)
  local soft = m:blur(5)
  local und = function(x, y) return clamp(1.6 * (soft:at(x, y) - soft:at(x, y + 5)), 0, 1) end
  work(soft, {hand="broad", angle=-0.005, length={40, 120}, coverage=2.6, medium=0.3, pal=cloudpal, hug=false, seed=seed,
    color_over=function(x, y, under)
      local c = mix(body, warm, und(x, y))
      return mix(under, c, amt * (0.55 + 0.45 * soft:at(x, y)) * (0.8 + 0.4 * cn:at01(x, y)))
    end})
  blend(m:grow(4):blur(3), {angle=0, coverage=1.4, seed=seed + 1})
end
veil(cA, "#7e7a8c", "#c4a09a", 0.75, 51)
veil(cB, "#8c8a98", "#c9aa9a", 0.55, 53)
local t = timesheet(); print(string.format("sitting %.0f min", t.sitting))

--@ chunk 9 · clock 3545.729750197381
-- the far shore and the town against the glow: dark and cool at the top, lost into the mist on the water
farpal = pal:only{"lead white", "pale smalt", "cobalt blue", "raw umber", "bone black", "red earth", "yellow ochre"}
local base = HZ + 2
local shoreP = {}
for _, p in ipairs(SHORE) do shoreP[#shoreP+1] = {p[1], p[2]} end
shoreP[#shoreP+1] = {1005, base}; shoreP[#shoreP+1] = {-5, base}
landM = poly(shoreP)
TOWN2 = {}
for _, p in ipairs(TOWN) do
  if p[1] < 486 or p[1] > 514 then TOWN2[#TOWN2+1] = p
  elseif p[1] == 488 and p[2] == 403 then
    for _, q in ipairs({{491,403},{491,372},{490,371},{490,367},{493,366},{493,362},{500,329},{507,362},{507,366},{510,367},{510,371},{509,372},{509,403}}) do TOWN2[#TOWN2+1] = q end
  end
end
local townP = {}
for _, p in ipairs(TOWN2) do townP[#townP+1] = {p[1], p[2]} end
townP[#townP+1] = {TOWN2[#TOWN2][1], base}; townP[#townP+1] = {TOWN2[1][1], base}
townM = poly(townP)
-- low groves on the shore, drawn by a soft hand, none alike
GROVES = {
  outline{pts={{112,428},{118,421},{128,418},{141,419},{150,416},{162,420},{168,427}}, open=true, char="soft", lobe=5, seed=61},
  outline{pts={{300,423},{306,416},{318,414},{331,415},{338,419},{352,418},{360,421}}, open=true, char="soft", lobe=4, seed=62},
  outline{pts={{672,422},{680,416},{694,413},{708,416},{716,414},{730,418},{742,424}}, open=true, char="soft", lobe=5, seed=63},
  outline{pts={{880,428},{888,423},{899,422},{906,425}}, open=true, char="soft", lobe=3, seed=64},
}
groveM = nil
for _, o in ipairs(GROVES) do local m = o:below(base); groveM = groveM and (groveM + m) or m end
local farcol = function(x, y)
  local t = smoothstep(398, base, y)
  return mix(mix("#4d4f60", "#5a5a68", smoothstep(330, 400, y)), "#8f8c8f", t * t)
end
-- the land sliver and groves: short level strokes of a small filbert
work(landM + groveM, {hand="detail", tool="filbert 3", angle=0, angle_jitter=0.15, length={4, 14}, coverage=4, medium=0.25, pal=farpal,
  color=farcol, edge={found=0.3, soft=0.5, lost=0.2, period=30, seed=65}})
-- the town: roofs level, the tower upright, found against the light
work(townM, {hand="detail", tool="round 1.2", angle=function(x, y) return (x > 488 and x < 512) and 1.57 or 0.05 end,
  length={3, 10}, coverage=3.6, medium=0.2, pal=farpal, color=farcol, edge={found=0.8, soft=0.2, period=25, seed=66}})
blend(landM + groveM + townM, {angle=0, coverage=0.8, clip=true})
local t = timesheet(); print(string.format("sitting %.0f min", t.sitting))
