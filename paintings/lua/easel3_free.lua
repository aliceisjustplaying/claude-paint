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
gclip = seaclip - (oakA:mask() + oakB:mask()):grow(1.5) - stonesil:grow(1) - below(function(x) return moundtop(x) - 2 end) * mask(function(x, y) return (x > 120 and x < 540) and 1 or 0 end)


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

--@ chunk 18 · clock 45631.58203125

MX, MY, MR = 772, 158, 10.5
-- lit side faces the sun, down and to the left
local ang = math.atan(430 - MY, SUNX - MX)
local ox, oy = -math.cos(ang) * MR * 0.62, -math.sin(ang) * MR * 0.62
local disc = ellipse(MX, MY, MR, MR)
crescent = (disc - ellipse(MX + ox, MY + oy, MR * 1.03, MR * 1.03)):soften(0.35)
glaze(ellipse(MX, MY, 60, 60):blur(25), {color="#c7c3ae", coats=0.10, pigment="semi"})
stipple(crescent, {width=1.4, color="#efe7c6", coverage=5, pressure={0.5, 0.9}, dips={30, 0.6, 0.3}, aim=false, medium=0.2, clip=crescent, fade=0})
-- earthshine, barely there
stipple(disc - crescent:grow(0.6), {width=1.2, color="#5f6b8a", coverage=1.0, pressure={0.3, 0.6}, aim=false, medium=0.4, clip=disc, fade=0.8})
-- the evening star
local st = brush("round", 1.6); st:load("#f3efdc", 0.9); st:touch(583, 296, {pressure=0.75}); st:touch(583.3, 296.2, {pressure=0.5})
-- two small sails on the horizon
local sb = brush("round", 1.2)
local function sail(x, h, lean)
  local m = poly({{x, HZ + 1.5}, {x - 0.35 * h, HZ + 1.5}, {x - 0.05 * h + lean, HZ - h}, {x + 0.12 * h, HZ - 0.3 * h}}, false):soften(0.3)
  work(m, {hand="detail", tool="round 1", color="#3b3a44", angle=math.pi / 2, length={2, 5}, coverage=3, clip=m})
  sb:reload("#2f2f38", 0.6); sb:stroke({{x - 0.45 * h, HZ + 1.8}, {x + 0.2 * h, HZ + 1.8}}, {pressure={0.4, 0.4}})
end
sail(836, 15, 1.5); sail(905, 9, 0.8)

--@ chunk 19 · clock 45631.58203125

local star = ellipse(604, 232, 1.5, 1.5)
stipple(star:grow(0.5), {width=1.3, color="#fbf8ea", coverage=6, pressure={0.6, 0.9}, aim=false, medium=0.15, fade=0, clip=star:grow(0.6)})
local fleck = ellipse(942, 316, 5, 3)
stipple(fleck, {width=1.6, color=skycol, coverage=3, pressure={0.4, 0.8}, medium=0.4, pal=skypal, clip=fleck:grow(1)})

--@ chunk 20 · clock 45631.58203125

-- foam and wet sand along the water edge, broken
local fb, wb = brush("rigger", 0.8), brush("round", 1.4)
local x = 505
local n = 0
while x < 1000 do
  local len = rand(8, 40)
  local x2 = math.min(x + len, 1000)
  local pts = {}
  for k = 0, 4 do local xx = lerp(x, x2, k / 4); pts[#pts+1] = {xx, shoreY(xx) - 1.5 + randn(0, 0.3)} end
  if math.random() < 0.7 then
    if n % 3 == 0 then fb:reload(mix("#a9a79a", "#7d7f84", rand()), 0.6) end
    fb:stroke(pts, {pressure={0.2, 0.55}, ramps={0.3, 0.4}, clip=seaclip:grow(2)})
  end
  local wpts = {}
  for k, p in ipairs(pts) do wpts[k] = {p[1], p[2] + 2.2} end
  if n % 3 == 1 then wb:reload("#3a3a34", 0.6) end
  wb:stroke(wpts, {pressure={0.3, 0.6}, ramps={0.3, 0.4}})
  x = x2 + rand(2, 25); n = n + 1
end
-- darker toward the edges and the bottom
local edge = mask(function(x, y)
  local b = smoothstep(560, 714, y)
  local l = (1 - smoothstep(0, 260, x)) * smoothstep(480, 714, y)
  local r = smoothstep(820, 1000, x) * smoothstep(540, 714, y) * 0.7
  return clamp(b + 0.6 * l + r, 0, 1)
end)
glaze(edge, {color="#1c1914", coats=0.28, pigment="transparent"})
local top = mask(function(x, y) return (1 - smoothstep(0, 120, y)) * 0.8 end)
glaze(top, {color="#2c3350", coats=0.12, pigment="transparent"})

--@ chunk 21 · clock 45631.58203125

glaze(landm * mask(function(x, y) return 0.5 + 0.5 * smoothstep(520, 640, y) end), {color="#26241c", coats=0.16, pigment="transparent"})

--@ chunk 22 · clock 45631.58203125

local pt = sward{region=pathm:grow(4), horizon=HZ, near=H, height=26, flowers=0.0, seed=12, thin=0.8, patch=0.7, patch_size=40,
  wind={lean=0.22, gust=0.3, period=140, seed=2}}
local g = brush("rigger", 0.7)
for i, t in ipairs(pt) do
  local near = smoothstep(540, 714, t.y)
  if i % 3 == 1 then g:reload(mix(({"#34322a", "#3d3a2f", "#2b2a23"})[1 + (i // 3) % 3], "#1e1c17", 0.5 * near), 0.7) end
  local p = clamp(0.2 + 0.55 * t.scale, 0.15, 0.9)
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}}) end
end

--@ chunk 23 · clock 45631.58203125

local function stone(c, r, s, yaw, cut)
  local b = body.ellipsoid(c, r):turn(c, yaw, 0.1, 0):rough(0.14 * r[1], 0.8 * r[1], s)
  if cut then b = b:cut({c[1], c[2] - 0.6 * r[2], c[3]}, {0.2, -1, 0.35}, s * 3, 3) end
  return b
end
fg = form{ {stone({128, 652, 0}, {44, 22, 30}, 71, 0.3, true) + stone({190, 672, 10}, {17, 10, 12}, 72, -0.4, false)}, {stone({905, 668, 0}, {26, 13, 18}, 73, 0.2, true)},
  light={from={0.4, -1}, front=-0.1, ambient=0.3, penumbra=0.1} }
local clipg = above(function(x) return 700 + 3 * gn(x, 9) end)
fgsil = fg:silhouette{parts={1, 2}, soft=0.4}:roughen(0.8, 12, 5, 0.5) * mask(function(x, y) local fy = (x < 400) and 664 or 674; return 1 - smoothstep(fy - 2, fy + 4, y) end)
work(fgsil, {hand="body", tool="filbert 3", color=function(x, y) return mix("#1e1d1a", "#3a3833", smoothstep(0.1, 0.9, fg:value(x, y))) end,
  angle=fg:field("across"), length={6, 18}, coverage=4.5, medium=0.12, clip=fgsil})
stipple(fgsil * fg:mask(function(s) return clamp((s.shade.sky or 0) * 1.5 - 0.5, 0, 1) end), {width=1.6, color="#56565a", coverage=1.8,
  pressure={0.35, 0.7}, medium=0.3, clip=fgsil})

--@ chunk 24 · clock 45631.58203125

local ft = sward{region=landm * below(function(x) return 628 end), horizon=HZ, near=H, height=58, flowers=0.0, seed=33, thin=0.78, patch=0.5, patch_size=90,
  wind={lean=0.25, gust=0.35, period=120, seed=5}}
local g = brush("rigger", 0.9)
for i, t in ipairs(ft) do
  if i % 3 == 1 then g:reload(({"#1d1b17", "#23211c", "#2b2922", "#191815"})[1 + (i // 3) % 4], 0.75) end
  local p = clamp(0.25 + 0.6 * t.scale, 0.2, 0.95)
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}}) end
end
-- dry stalks and seed heads catching the last of the sky
local sk2, hd = brush("rigger", 0.7), brush("round", 1.3)
local xs = uneven(26, 20, 980, 0.7, 0.5, 6)
local n = 0
for i, x0 in ipairs(xs) do
  local y0 = rand(640, 712)
  local h = rand(45, 110) * (0.6 + 0.4 * (y0 - 600) / 114)
  local lean = randn(0.12, 0.12)
  local top = {x0 + lean * h + randn(0, 3), y0 - h}
  local mid = {x0 + lean * h * 0.45 + randn(0, 2), y0 - h * 0.55}
  sk2:reload(mix("#6a634f", "#4a4538", rand()), 0.7)
  sk2:stroke({{x0, y0}, mid, top}, {pressure={0.75, 0.25}, ramps={0.05, 0.3}, shake=0.5})
  local kind = i % 3
  if kind == 0 then           -- an umbel: spokes and a flat head
    for s = 1, 7 do
      local a = -math.pi / 2 + (s - 4) * 0.28 + randn(0, 0.06)
      local L = rand(6, 10)
      sk2:stroke({top, {top[1] + math.cos(a) * L, top[2] + math.sin(a) * L}}, {pressure={0.35, 0.15}})
      hd:reload("#6a624c", 0.5); hd:touch(top[1] + math.cos(a) * L, top[2] + math.sin(a) * L - 0.5, {pressure=0.35})
    end
  elseif kind == 1 then       -- a thistle or knapweed knob
    hd:reload("#2a2622", 0.8); hd:touch(top[1], top[2], {pressure=0.8}); hd:touch(top[1] + 0.6, top[2] - 1.2, {pressure=0.6})
    hd:reload("#5b4a4a", 0.4); hd:touch(top[1], top[2] - 2.2, {pressure=0.4})
  else                        -- a grass panicle
    for s = 1, 5 do
      local py = top[2] + s * 3.2
      local px = top[1] - lean * s * 3
      sk2:stroke({{px, py}, {px + (s % 2 == 0 and 5 or -5), py - 3}}, {pressure={0.3, 0.05}})
    end
  end
  n = n + 1
end
print(#ft, n)

--@ chunk 25 · clock 45631.58203125

-- contact shadow under the capstone
local under = df:mask(function(s) return clamp(s.n[2] * 2.2 - 0.5, 0, 1) end) * mask(function(x, y) return 1 - smoothstep(438, 450, y) end)
glaze(under * stonesil, {color="#141311", coats=0.25, pigment="transparent"})
-- fissures
local fb = brush("rigger", 0.55)
local cracks = {{200, 425, 0.35, 26}, {262, 404, 0.1, 40}, {345, 398, -0.05, 34}, {395, 404, 0.25, 22}, {430, 415, 1.3, 14},
  {236, 462, 1.45, 30}, {298, 452, 1.6, 20}, {312, 478, 1.2, 16}, {404, 462, 1.75, 24}, {176, 503, 0.2, 12}}
for i, c in ipairs(cracks) do
  local x, y, a, L = c[1], c[2], c[3], c[4]
  local pts = {{x, y}}
  for k = 1, 4 do a = a + randn(0, 0.35); x = x + math.cos(a) * L / 4; y = y + math.sin(a) * L / 4; pts[#pts+1] = {x, y} end
  if i % 3 == 1 then fb:reload("#141311", 0.7) end
  fb:stroke(pts, {pressure={0.6, 0.1}, ramps={0.1, 0.5}, clip=stonesil:shrink(1)})
end
-- lichen on the faces that look up at the sky
local up = stonesil:shrink(1.5) * df:mask(function(s) return clamp((s.shade.sky or 0) * 1.6 - 0.7, 0, 1) end)
local ln = noise{seed=61, octaves=3, period=7}
stipple(up, {width=1.1, color=function(x, y) return ln(x, y) > 0.2 and "#555749" or "#574e3b" end,
  coverage=function(x, y) return 0.9 * smoothstep(0.35, 0.75, ln:at01(x, y)) end, pressure={0.3, 0.6}, aim=false, medium=0.25, fade=0, clip=up})
-- grass over the stone feet
local feet = mask(function(x, y) local m = moundtop(x); return (x > 150 and x < 490 and y > m - 4 and y < 520) and 1 or 0 end)
local ft = sward{region=feet, horizon=HZ, near=H, height=16, flowers=0.0, seed=44, thin=0.2, wind={lean=0.22, gust=0.3, period=140, seed=2}}
local g = brush("rigger", 0.55)
for i, t in ipairs(ft) do
  if i % 3 == 1 then g:reload(({"#3a382c", "#474331", "#2e2d25", "#57513c"})[1 + (i // 3) % 4], 0.7) end
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={clamp(0.2 + 0.5 * t.scale, 0.15, 0.7), 0.0}, ramps={0.05, 0.7}}) end
end
print(#ft)

--@ chunk 26 · clock 45631.58203125

local cn = noise{seed=71, octaves=4, period=40, stretch={0.0, 6}}
local function streak(x0, x1, y, th, tilt, seed)
  local pts, ws = {}, {}
  local n = 14
  for k = 0, n do
    local t = k / n
    local x = lerp(x0, x1, t)
    pts[#pts+1] = {x, y + tilt * (x - x0) + 3 * cn(x, y + seed)}
    ws[#ws+1] = th * math.sin(math.pi * t)^0.6 * (0.4 + 1.0 * cn:at01(x * 2, seed))
  end
  return ribbon(pts, ws):roughen(2, 14, seed, 3):blur(1.5)
end
local front = (oakA:mask() + oakB:mask()):grow(1) + stonesil:grow(1)
bars = (streak(520, 1000, 350, 7, -0.012, 1) + streak(640, 1010, 372, 4.5, -0.004, 2) + streak(-10, 300, 386, 4, 0.006, 3)) - front
stipple(bars, {width=2.2, color=function(x, y) return mix(skycol(x, y), "#7d7084", 0.35) end, coverage=function(x, y) return 2.4 * bars:at(x, y) end,
  pressure={0.4, 0.8}, dips={20, 0.35, 0.6}, medium=0.5, pal=skypal, clip=bars})

--@ chunk 27 · clock 45631.58203125

blend(bars:grow(2), {angle=0, length={15, 40}, coverage=2, clip=bars:grow(3)})

--@ chunk 28 · clock 45631.58203125

-- the roots flare into the turf: a low swelling of bark, then grass over the spikes
local g = brush("rigger", 0.6)
local function tuft_band(cx, cy, rx, ry, h, seed)
  local reg = ellipse(cx, cy, rx, ry)
  local ts = sward{region=reg, horizon=HZ, near=H, height=h, flowers=0.0, seed=seed, thin=0.0, wind={lean=0.22, gust=0.3, period=140, seed=2}}
  for i, t in ipairs(ts) do
    if i % 3 == 1 then g:reload(({"#34322a", "#403c30", "#2a2923", "#4c4736"})[1 + (i // 3) % 4], 0.7) end
    for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={clamp(0.2 + 0.5 * t.scale, 0.15, 0.8), 0.0}, ramps={0.05, 0.7}}) end
  end
  return #ts
end
print(tuft_band(118, 522, 40, 9, 30, 91), tuft_band(118, 524, 34, 6, 24, 96), tuft_band(508, 516, 30, 5, 18, 92), tuft_band(128, 667, 60, 6, 40, 93), tuft_band(190, 674, 26, 5, 34, 94), tuft_band(905, 676, 36, 6, 40, 95))
-- heather: stiff little sprigs over the patches
local hs = brush("rigger", 0.5)
local n = 0
for i = 1, 1400 do
  local x, y = rand(0, 1000), rand(528, 714)
  if heath:at(x, y) > 0.5 then
    local s = 0.6 + 1.6 * (y - 520) / 194
    if n % 10 == 0 then hs:reload(mix(({"#46353a", "#3b2f2c", "#524047"})[1 + n % 3], "#1f1a19", 0.5 * smoothstep(560, 714, y)), 0.7) end
    local a = -math.pi / 2 + randn(0.15, 0.35)
    local L = rand(3, 7) * s
    local tip = {x + math.cos(a) * L, y + math.sin(a) * L}
    hs:stroke({{x, y}, tip}, {pressure={0.5, 0.1}})
    hs:stroke({{x + math.cos(a) * L * 0.5, y + math.sin(a) * L * 0.5}, {tip[1] + L * 0.3, tip[2] + L * 0.2}}, {pressure={0.35, 0.05}})
    n = n + 1
  end
end
print(n)

--@ chunk 29 · clock 45631.58203125

glaze(fgsil:grow(1), {color="#1d1b17", coats=0.3, pigment="transparent"})

--@ chunk 30 · clock 45631.58203125

local sb = brush("round", 1.0)
sb:load("#1c1b1a", 0.8)
sb:stroke({{FX + 8.2, FY - 25}, {FX + 10.6, FY - 12}, {FX + 12.8, FY + 0.5}}, {pressure={0.55, 0.45}, ramps={0.05, 0.1}})

--@ chunk 31 · clock 45631.58203125

dry(); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
