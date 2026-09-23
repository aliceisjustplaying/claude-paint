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
