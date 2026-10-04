-- easel session "easel3_green": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel3_green.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 392
knollg = function(X, Z)
  return 3.2*math.exp(-(((X + 6)/42)^2 + ((Z - 78)/34)^2)) + 0.6*math.sin(X/23 + Z/31) * math.min(1, Z/40)
end
w = world{horizon=HZ, eye=6, fov=50, sun={azimuth=-118, elevation=40}, ground=knollg}
for _, p in ipairs({{430, 470}, {430, 520}, {200, 600}, {800, 560}, {500, 420}}) do
  print(w:spot(p[1], p[2]))
end
print(w:project(-6, 3.2, 78))

--@ chunk 3 · clock 0
sk = w:sky{haze=2.0, uneven={0.35, 30000, 7}}
cl = w:clouds{sky=sk, cell=3,
  {kind="cumulus", x=4600, z=14000, base=1250, width=3000, height=1400, seed=31},
  {kind="cumulus", x=7400, z=19000, base=1300, width=2600, height=900, seed=8},
  {kind="cumulus", x=-4300, z=16000, base=1350, width=2000, height=700, seed=12},
  {kind="bank", x0=-30000, x1=30000, z=42000, depth=9000, base=800, top=1800, seed=5},
  {kind="stratus", base=4200, thick=300, cover=0.12, seed=3, wind={-0.1, 2.5}, breaks={12000, 0.25}}}
skym = above(function(x) return HZ + 14 end)
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "red earth"}
work(skym, {hand="broad", color=cl, angle=0, coverage=4.5, medium=0.3, pal=skypal, angle_jitter=0.05})
blend(skym, {angle=0})
