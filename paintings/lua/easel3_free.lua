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
