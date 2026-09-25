-- easel session "r9a3_winter": a painting replayed chunk by chunk.
--   easel run paintings/lua/r9a3_winter.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=29}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 468
-- the drawing: horizon ruled, the mound, oak trunk, the cross, the walker
MOUND = {{-10,588},{60,572},{140,556},{220,546},{300,540},{370,546},{430,560},{500,582},{570,598},{650,606},{760,610},{880,606},{1010,600}}
local h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
h:sketch(MOUND, {pressure=0.3})
h:sketch({{318,548},{314,470},{322,390},{312,300},{330,220}}, {pressure=0.3})
local b = pencil("HB")
b:line({{596,604},{598,530},{600,488}}, {pressure={0.4,0.5,0.4}})
b:line({{582,512},{616,510}}, {pressure=0.45})
b:sketch({{700,622},{702,600},{704,580},{703,566}}, {pressure=0.35})

--@ chunk 3 · clock 0
-- the evening sky: gray-violet above, a lemon-rose glow low where the sun went down (left)
bands = noise{seed=41, octaves=4, period=260, stretch={0.03, 6}}
skycol = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local c = gradient({{0, "#58647c"}, {0.35, "#7c8497"}, {0.62, "#a8a3a8"}, {0.82, "#cdb9aa"}, {1, "#e4d3a8"}}, t)
  -- the glow sits left of center and fades to mauve on the right
  local glow = math.exp(-((x - 260) / 420)^2) * smoothstep(0.55, 1, t)
  local right = mix(c, "#b6a6ad", 0.55 * smoothstep(0.6, 1, t))
  c = mix(right, c, glow)
  -- long cloud banks lying across, darker gray-violet
  local bk = smoothstep(0.15, 0.55, bands(x, y)) * (0.6 - 0.4 * t)
  return mix(c, "#6a6878", bk)
end
skym = above(function(x) return HZ + 14 end)
work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.02 * bands(x * 0.3, y) end,
  length={120, 320}, coverage=4.5, medium=0.3})
-- blend later

--@ chunk 4 · clock 0
-- a second lay-in wet into the first: shorter sweeps, fuller loads, to close the gaps
work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.03 * bands(x * 0.3, y + 40) end,
  length={60, 170}, coverage=3.5, medium=0.3, load=1})
blend(skym, {angle=0.01})

--@ chunk 5 · clock 0
wait(24*60)
-- the snow in one wet lay-in, the mound modeled in it: its face toward the glow light and warm,
-- the plain in its lee cooler toward the crest, the far slope and the near drifts blue-violet
mound = function(x)
  local pts = MOUND
  for i = 1, #pts - 1 do
    local a, b = pts[i], pts[i + 1]
    if x >= a[1] and x <= b[1] then local t = (x - a[1]) / (b[1] - a[1]); return lerp(a[2], b[2], smoothstep(0, 1, t)) end
  end
  return 600
end
drift = noise{seed=52, octaves=4, period=140, stretch={0.05, 3}}
crestn = noise{seed=53, octaves=3, period=40}
snowcol = function(x, y)
  local d = clamp((y - HZ) / (714 - HZ), 0, 1)
  local far = mix("#cbc4c6", "#dcd2b6", math.exp(-((x - 260) / 380)^2) * 0.7)
  local c = mix(far, "#b9b8c4", smoothstep(0.05, 0.6, d))
  local m = mound(x) + 1.5 * crestn(x, 0)
  local onm = smoothstep(m - 1.5, m + 1.5, y)          -- 1 on the mound
  local west = 1 - smoothstep(400, 560, x)             -- the part of the mound facing the glow
  -- lee: the plain just behind the crest, cooler toward it
  local lee = (1 - onm) * smoothstep(HZ + 12, m, y) * (1 - smoothstep(560, 700, x))
  c = mix(c, "#b6b5c3", 0.8 * lee)
  -- face: light and warm under the crest, fading down the slope
  local face = onm * west * (1 - smoothstep(m + 4, m + 80, y))
  c = mix(c, mix("#e8ddca", "#dcd4cb", smoothstep(m, m + 60, y)), 0.9 * face)
  -- the slope turning away on the right of the mound
  local away = onm * smoothstep(400, 520, x) * (1 - smoothstep(600, 700, x)) * (1 - smoothstep(m + 6, m + 90, y))
  c = mix(c, "#b0b1c2", 0.8 * away)
  -- drifts: soft blue-violet shadows, stronger near
  c = mix(c, "#a4a7bb", (0.2 + 0.4 * d) * smoothstep(0.1, 0.6, drift(x, y)))
  return mix(c, "#9a9aab", 0.45 * smoothstep(0.6, 1, d))
end
snowm = below(function(x) return HZ - 4 end)
work(snowm, {hand="body", color=snowcol, angle=function(x, y) return 0.04 * drift(x, y) end,
  length={25, 80}, coverage=3.5, medium=0.2})
blend(snowm, {angle=0.02})

--@ chunk 6 · clock 1440
wait(24*60)
-- the far wood on the plain, right of center: a low dark band, hazed by the evening
WOODTOP = {{548,471},{566,462},{590,457},{612,452},{640,449},{668,445},{700,447},{730,441},{760,444},{792,438},{826,442},{860,436},{900,440},{940,437},{975,442},{1010,440}}
wood = fir_wood{skyline=outline{pts=WOODTOP, open=true, char="soft", lobe=6, seed=61}, foot=474,
  depth=2, count=46, recede=0.6, air=0.5, seed=62}
print(wood)
air = "#9d9aa6"
for r = 2, 1, -1 do
  local hz, s, nd = wood:haze(r), wood:scale(r), wood:needles(r)
  local base = mix("#3a4046", air, 0.35 + 0.35 * hz)
  work(nd, {hand="hatch", tool="round 0.9", length={1.5, 4}, coverage=2.6, clip=nd, angle=1.57, angle_jitter=0.4, color=base})
  local rb = brush("rigger", 0.5)
  for n, f in ipairs(wood:trees(r)) do
    if n % 6 == 1 then rb:reload(mix("#34373a", air, 0.4 + 0.3 * hz), 0.8) end
    rb:stroke(f.leader.pts, {pressure={0.8, 0.1}, ramps={0.02, 0.4}})
  end
end

--@ chunk 7 · clock 2880
-- snow dragged across the far trees' feet: a few short horizontal pulls, not a band
local feet = rect(545, 471, 470, 6):roughen(1.5, 14, 72, 1)
work(feet, {hand="body", tool="round 1.6", coverage=1.2, angle=function(x, y) return 0.03 * drift(x, y) end, length={6, 22}, hug=false,
  color="#bdb9c2", medium=0.2})
