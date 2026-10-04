-- easel session "easel3_near": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel3_near.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.3, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0

HZ = 470
skyn = noise{seed=5, octaves=4, period=300, warp={120, 40}}
sky = function(x, y)
  local t = clamp(y / 520 + 0.06 * skyn(x, y), 0, 1)
  return gradient({{0, "#7f8d9c"}, {0.35, "#a9b1b5"}, {0.72, "#d9d3bf"}, {1, "#e8dcbc"}}, t)
end
skym = above(function(x) return 600 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}})
blend(skym, {angle=0})
