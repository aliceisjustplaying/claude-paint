-- easel session "easel3_free": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel3_free.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820", aspect=1.4, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0

HZ = math.floor(H * 0.60)
dn = noise{seed=5, octaves=4, period=14}
ground = function(X, Z)
  local knoll = 1.1 * math.exp(-((X + 3) / 9)^2) * math.exp(-((Z - 17) / 7)^2)
  local shore = 3.2 * smoothstep(16, 34, Z + 0.25 * X)       -- the bank falls away, sooner on the right
  return knoll - 0.03 * Z - shore + 0.25 * dn(X, Z) * (1 - smoothstep(25, 45, Z))
end
w = world{horizon=HZ, eye=1.7, fov=46, sun={azimuth=18, elevation=-4}, ground=ground,
  water={level=-3.9, ripple={0.02, 1.2, 0.25, 7}}, visibility=30000}
v = w:view()
for _, y in ipairs({HZ+10, HZ+20, HZ+30, HZ+45, HZ+60, HZ+80, HZ+120, HZ+160, H-5}) do
  local s = {}
  for _, x in ipairs({50, 200, 350, 500, 650, 800, 950}) do
    local a = v:at(x, y); s[#s+1] = a.what:sub(1,1) .. (a.at and string.format("%.0f", a.at[3]) or "")
  end
  print(y, table.concat(s, " "))
end
local sp = w:spot(400, 500); print(sp, w:height(sp.x, sp.y, 2.2))

--@ chunk 3 · clock 0

local sc = w:sun_canvas(); print(sc[1], sc[2])
SUNX = 640
skyband = noise{seed=9, octaves=4, period=260, stretch={0.0, 5}}
skycol = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local dx = (x - SUNX) / 520
  local glow = math.exp(-dx * dx)                  -- afterglow over where the sun went down
  local tt = clamp(t + 0.10 * glow * t * t + 0.02 * skyband(x, y), 0, 1)
  return gradient({{0, "#3c4768"}, {0.22, "#51628a"}, {0.45, "#7f93ad"}, {0.66, "#b4bdb4"},
                   {0.80, "#dcd6a8"}, {0.92, "#ecca86"}, {1, "#e7ad6c"}}, tt)
end
skym = above(function(x) return HZ + 6 end)
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "chrome yellow", "red earth", "vermilion", "raw umber"}
work(skym, {hand="broad", color=skycol, angle=0, coverage=4.5, medium=0.3, pal=skypal})
blend(skym, {angle=0})

--@ chunk 4 · clock 0

blend(skym, {angle=0, length={60, 160}, coverage=3})

--@ chunk 5 · clock 0

wait(180)
stipple(skym, {width=2.6, color=skycol, coverage=3.2, pressure={0.4, 0.8}, dips={20, 0.35, 0.6}, medium=0.55, pal=skypal})

--@ chunk 6 · clock 180

wait(24*60)
seaband = noise{seed=13, octaves=5, period=90, stretch={0.0, 14}}
seacol = function(x, y)
  local d = math.max(y - HZ, 0)
  local refl = skycol(x, HZ - d * 2.2 - 10)
  local dark = mix("#344063", "#232a3c", smoothstep(0, 110, d))
  local k = 0.40 - 0.22 * smoothstep(0, 110, d) + 0.16 * seaband(x, y)
  local dx = (x - SUNX) / 200
  k = k + 0.22 * math.exp(-dx * dx) * (1 - smoothstep(0, 90, d)) * (0.6 + 0.4 * seaband(x * 1.7, y))
  return mix(dark, refl, clamp(k, 0, 1))
end
seam = v:water():grow(3) * below(function(x) return HZ end)
work(seam, {hand="broad", color=seacol, angle=0, angle_jitter=0.004, length={60, 180}, coverage=4.5, medium=0.28, clip=seam})
blend(seam, {angle=0, length={60, 180}, clip=seam})

--@ chunk 7 · clock 1620

shoreline = function(x) local y = HZ; while y < H - 1 and v:at(x, y).what ~= "ground" do y = y + 1 end; return y end
SH = {}; for x = 0, 990, 10 do SH[#SH+1] = {x, shoreline(x)} end
SH[#SH+1] = {1000, SH[#SH][2] + 1.5}
shoreY = function(x) local i = clamp(math.floor(x / 10) + 1, 1, #SH - 1); local a, b = SH[i], SH[i+1]; return lerp(a[2], b[2], (x - a[1]) / 10) end
landm = below(SH):roughen(2.5, 22, 4, 1)
gn = noise{seed=21, octaves=5, period=70}
sandn = noise{seed=31, octaves=4, period=60, stretch={0.0, 3}}
landcol = function(x, y)
  local d = y - shoreY(x)
  local c = mix("#4a4535", "#2c2a22", smoothstep(0, 160, d))
  c = mix(c, "#5a5238", 0.25 * gn:at01(x, y))
  local sw = 4 + 14 * sandn:at01(x, 0) * smoothstep(380, 760, x)
  local sand = (1 - smoothstep(0, sw, d)) * smoothstep(380, 640, x)
  return mix(c, mix("#8c8468", "#6f6a58", sandn:at01(x, y)), 0.75 * sand)
end
work(landm, {hand="body", color=landcol, angle=function(x, y) return -0.06 + 0.12 * gn(x, y) end, length={20, 60}, coverage=3.5, medium=0.18})

--@ chunk 8 · clock 1620

wait(24*60)
moundtop = function(x) return 510 - 22 * math.exp(-((x - 318) / 120)^2) - 4 * math.exp(-((x - 250) / 40)^2) + 3 * gn(x, 0) end
moundm = below(moundtop):roughen(1.2, 10, 6, 1.2) * mask(function(x, y) return (1 - smoothstep(505, 528, y)) * smoothstep(130, 190, x) * (1 - smoothstep(470, 530, x)) end)
work(moundm, {hand="body", color=function(x, y) return mix("#4d4836", "#3b382b", smoothstep(488, 525, y)) end,
  angle=function(x, y) return 0.25 * math.sin((x - 318) / 60) end, length={10, 30}, coverage=3.2, medium=0.18, hug=false})

--@ chunk 9 · clock 3060

local function stone(c, r, s, yaw, roll, cuts)
  local b = body.ellipsoid(c, r):turn(c, yaw or 0, 0, roll or 0):rough(0.16 * math.max(r[1], r[2]), 0.7 * math.max(r[1], r[2]), s):rough(0.05 * math.max(r[1], r[2]), 0.25 * math.max(r[1], r[2]), s + 7)
  for i, k in ipairs(cuts or {}) do
    b = b:cut({c[1] + k[1] * r[1], c[2] + k[2] * r[2], c[3] + k[3] * r[3]}, k, s * 10 + i, 3)
  end
  return b
end
u1 = stone({230, 484, 0}, {17, 44, 17}, 1, 0.3, 0.14, {{-0.9, -0.3, 0.3}, {0.1, -0.97, 0.2}, {0.8, 0.3, 0.5}})
u2 = stone({302, 478, 8}, {24, 42, 21}, 2, -0.3, -0.05, {{0.85, 0.1, 0.5}, {0.05, -0.98, 0.2}, {-0.8, 0.2, 0.5}})
u3 = stone({404, 480, -4}, {22, 42, 20}, 3, 0.4, 0.14, {{0.9, -0.4, 0.2}, {-0.1, -0.97, 0.2}, {-0.85, 0.1, 0.4}})
cap = stone({318, 416, -6}, {132, 25, 48}, 5, 0.15, -0.06, {{0.1, 0.9, 0.1}, {0.95, -0.2, 0.3}, {-0.7, -0.7, 0.2}, {0.1, -0.95, 0.3}})
f1 = stone({180, 506, 25}, {20, 12, 14}, 6, 0.5, 0, {{0, -0.9, 0.4}})
f2 = stone({462, 510, 18}, {15, 10, 12}, 8, -0.4, 0.2, {{0.3, -0.9, 0.3}})
f3 = stone({526, 528, 30}, {8, 5, 7}, 9, 0.2, 0, {})
dol = u1 + u2 + u3 + cap
df = form{ {dol, dist=0.3}, {f1 + f2 + f3, dist=0.25},
  light={from={0.8, -0.5}, front=-0.35, ambient=0.3, penumbra=0.1} }
stn = noise{seed=41, octaves=4, period=26}
stonecol = function(x, y, lo, hi)
  local c = mix(lo, hi, smoothstep(0.0, 0.9, df:value(x, y)))
  return mix(c, "#5a5344", 0.2 * stn:at01(x, y))
end
ground_clip = above(function(x) return moundtop(x) + 6 end):soften(2) + mask(function(x, y) return y < 500 and 1 or 0 end)
stonesil = df:silhouette{parts={1, 2}, soft=0.4}:roughen(0.9, 16, 12, 0.6) * ground_clip
work(stonesil, {hand="body", tool="flat 5", color=function(x, y) return stonecol(x, y, "#2a2823", "#3a362f") end,
  angle=df:field("across"), length={10, 30}, coverage=5, load=1, medium=0.12, clip=stonesil})
blend(stonesil, {angle=df:field("across"), length={8, 20}, clip=stonesil})

--@ chunk 10 · clock 3060

wait(90)
local toplit = stonesil * df:mask(function(s) return clamp((s.shade.sky or 0) * 1.4 - 0.35, 0, 1) end):soften(1.5)
stipple(toplit, {width=1.8, color=function(x, y) return stonecol(x, y, "#4a4944", "#6c6a64") end, coverage=2.2,
  pressure={0.35, 0.7}, dips={18, 0.4, 0.6}, medium=0.3, clip=stonesil})
local rim = stonesil * df:lit{parts={1, 2}, soft=0.25}
stipple(rim, {width=1.6, color=function(x, y) return stonecol(x, y, "#4d463a", "#7d6c52") end, coverage=2.0,
  pressure={0.35, 0.7}, dips={18, 0.4, 0.6}, medium=0.3, clip=stonesil})

--@ chunk 11 · clock 3150
oakA = tree{habit="oak", x=118, y=518, height=370, seed=22}
oakB = tree{habit="dead_oak", x=508, y=512, height=270, seed=8}

behind = -stonesil:grow(0.5)
barkn = noise{seed=51, octaves=3, period=9}
local function paint_tree(t, dark, mid)
  -- trunk and big limbs as their true shapes
  local big = nil
  for _, l in ipairs(t.limbs) do
    if #l.pts >= 2 and l.w[1] > 4 then
      local r = ribbon(l.pts, l.w); big = big and (big + r) or r
    end
  end
  big = big:roughen(0.8, 6, 3, 0.5)
  work(big * behind, {hand="body", tool="filbert 3", color=function(x, y) return mix(dark, mid, 0.5 * barkn:at01(x, y)) end,
    angle=math.pi / 2, angle_jitter=0.25, length={6, 18}, coverage=4, medium=0.12, clip=big * behind})
  local b1, b2, b3, b4 = brush("round", 5), brush("round", 2.4), brush("round", 1.2), brush("rigger", 0.6)
  local n = 0
  for i, l in ipairs(t.limbs) do
    if #l.pts >= 2 and l.w[1] <= 4 then
      local w0 = l.w[1]
      local b = (w0 > 2.2) and b1 or (w0 > 1.0 and b2 or (w0 > 0.45 and b3 or b4))
      if b:fullness() < 0.35 or i % 7 == 0 then b:reload(w0 < 0.6 and mid or dark, 0.85) end
      local p0 = clamp(b:pressure_for(w0), 0.08, 1)
      b:stroke(l.pts, {pressure={p0, 0.02}, ramps={0.03, 0.6}, shake=0.4, clip=behind})
      n = n + 1
    end
  end
  return n
end
print(paint_tree(oakA, "#27241f", "#3d3830"), paint_tree(oakB, "#2c2924", "#433e36"))

--@ chunk 12 · clock 3150

local tw = brush("rigger", 0.5)
local n = 0
for ti, t in ipairs({oakA, oakB}) do
  for i, l in ipairs(t.limbs) do
    if l.order >= 2 and #l.pts >= 2 and l.w[#l.w] < 1.0 then
      local k = (ti == 1) and 3 or 2
      for j = 1, k do
        local p = l.pts[math.max(1, #l.pts - math.random(0, math.min(3, #l.pts - 1)))]
        local q = l.pts[#l.pts]
        local a = math.atan(q[2] - l.pts[1][2], q[1] - l.pts[1][1]) + randn(0, 0.7)
        local len = rand(5, 14) * (ti == 1 and 1 or 0.8)
        local mid = {p[1] + math.cos(a) * len * 0.5 + randn(0, 1), p[2] + math.sin(a) * len * 0.5 + randn(0, 1)}
        local a2 = a + randn(0, 0.6)
        local tip = {mid[1] + math.cos(a2) * len * 0.5, mid[2] + math.sin(a2) * len * 0.5}
        if n % 12 == 0 then tw:reload("#35312b", 0.8) end
        tw:stroke({p, mid, tip}, {pressure={0.55, 0.0}, ramps={0.05, 0.7}, clip=behind})
        n = n + 1
      end
    end
  end
end
print(n)

--@ chunk 13 · clock 3150

seaclip = seam - landm:grow(1)
glaze(seaclip * mask(function(x, y) return smoothstep(HZ - 10, HZ + 120, y) end), {color="#2a3350", coats=0.16, pigment="transparent"})

--@ chunk 14 · clock 3150
gclip = seaclip - (oakA:mask() + oakB:mask()):grow(1.5) - stonesil:grow(1)


local lt, dk = brush("rigger", 0.9), brush("round", 1.6)
local y = HZ + 1.5
local n = 0
while y < 575 do
  local d = y - HZ
  local cnt = math.floor(rand(2, 5) + d * 0.03)
  for j = 1, cnt do
    local x = rand(-40, 1040)
    local len = rand(6, 45) * (0.5 + d / 90) * (math.random() < 0.15 and 2.2 or 1)
    local glow = math.exp(-((x - SUNX) / 230)^2)
    local yy = y + randn(0, 0.4)
    if math.random() < 0.45 + 0.3 * glow then
      local c = mix(seacol(x, yy), skycol(x, HZ - d * 2.2 - 10), 0.45 + 0.3 * glow)
      if n % 4 == 0 then lt:reload(c, rand(0.3, 0.6)) end
      lt:stroke({{x, yy}, {x + len * 0.5, yy + randn(0, 0.3)}, {x + len, yy}}, {pressure={0.1, 0.35 + 0.3 * d / 100}, ramps={0.4, 0.4}, clip=gclip})
    else
      if n % 4 == 1 then dk:reload(mix(seacol(x, yy), "#1b2030", 0.45), 0.6) end
      dk:stroke({{x, yy}, {x + len * 0.5, yy + randn(0, 0.3)}, {x + len, yy}}, {pressure={0.1, 0.3 + 0.4 * d / 100}, ramps={0.4, 0.4}, clip=gclip})
    end
    n = n + 1
  end
  y = y + 1.2 + d * 0.055 + rand(0, 1)
end
print(n)

--@ chunk 15 · clock 3150

print(clock())
wait(120)
pathpts = {{820, 714}, {735, 668}, {690, 628}, {640, 596}, {590, 566}, {545, 546}, {512, 532}}
pathm = ribbon(pathpts, {34, 26, 20, 15, 11, 7, 4}):roughen(2.5, 14, 8, 2.5) * landm
work(pathm, {hand="body", tool="filbert 3", color=function(x, y) return mix("#5b5644", "#4a4638", smoothstep(530, 714, y) * 0.5 + 0.4 * gn:at01(x * 3, y * 3)) end,
  angle=function(x, y) return -0.6 + 0.35 * gn(x * 2, y * 2) end, length={5, 16}, coverage=2.6, medium=0.18, clip=pathm})
heath = mask(function(x, y) return (gn:at01(x * 1.3, y * 1.9) > 0.6 and y > 540) and 1 or 0 end):roughen(4, 16, 11, 5) * landm - pathm:grow(3)
stipple(heath, {width=2.2, color=function(x, y) return mix("#3f3431", "#302827", smoothstep(530, 714, y)) end, coverage=1.8,
  pressure={0.4, 0.8}, dips={16, 0.35, 0.6}, medium=0.2, fade=1})

--@ chunk 16 · clock 45631.58203125

local region = (landm * below(function(x) return moundtop(x) - 2 end) + landm * mask(function(x, y) return (x < 150 or x > 500) and 1 or 0 end)) - pathm:shrink(2)
tufts = sward{region=region, horizon=HZ, near=H, height=36, flowers=0.0, seed=4, thin=0.3, patch=0.6, patch_size=110,
  wind={lean=0.22, gust=0.3, period=140, seed=2}}
local g = brush("rigger", 0.7)
local cols = {"#48463a", "#565240", "#393a2e", "#625c45", "#302f26", "#4f4a3a"}
local n = 0
for i, t in ipairs(tufts) do
  if i % 3 == 1 then
    local c = cols[1 + (i // 3) % #cols]
    local near = smoothstep(520, 714, t.y)
    g:reload(mix(c, "#24231d", 0.45 * near), 0.7)
  end
  local p = clamp(0.2 + 0.55 * t.scale, 0.15, 0.9)
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}}); n = n + 1 end
end
print(#tufts, n)

--@ chunk 17 · clock 45631.58203125

FX, FY = 452, 512           -- where his feet are
local s = 1.0
local coat = poly({{FX - 5.5, FY - 44}, {FX - 7.2, FY - 36}, {FX - 8.4, FY - 22}, {FX - 9.6, FY - 12}, {FX - 1, FY - 11},
  {FX + 8.8, FY - 12.5}, {FX + 8.0, FY - 24}, {FX + 7.0, FY - 37}, {FX + 5.2, FY - 44}, {FX, FY - 45.5}}, true)
local head = ellipse(FX + 0.3, FY - 49.5, 3.7, 4.3)
local cap = ellipse(FX + 0.3, FY - 52.2, 4.0, 2.4)
local legs = ribbon({{FX - 3.2, FY - 12}, {FX - 3.6, FY}}, 3.0) + ribbon({{FX + 3.0, FY - 12}, {FX + 3.4, FY - 0.5}}, 3.0)
figure = (coat + head + cap + legs):soften(0.4)
work(figure, {hand="detail", tool="round 2", color=function(x, y) return mix("#23252b", "#1b1c20", smoothstep(FY - 50, FY, y)) end,
  angle=math.pi / 2, length={3, 8}, coverage=4, medium=0.12, clip=figure})
work(head - cap, {hand="detail", tool="round 1.2", color="#3a302b", angle=math.pi / 2, length={2, 4}, coverage=3, clip=head - cap})
-- the afterglow catching his right shoulder and cap edge
local rim = figure * mask(function(x, y) return (x > FX + 3.5 and y < FY - 20) and 1 or 0 end)
work(rim, {hand="detail", tool="round 1", color="#57493d", angle=math.pi / 2, length={3, 7}, coverage=1.5, clip=figure})
