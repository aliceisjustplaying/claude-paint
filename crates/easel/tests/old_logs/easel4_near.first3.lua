-- easel session "easel4_near": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_near.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.3, seed=23}; print(H); print(pal)

--@ chunk 2 · clock 0
-- the place: a snowy clearing at the foot of a spruce wood, late afternoon in
-- early winter; a low sun from the left, a little behind me
HZ = 300
w = world{horizon=HZ, eye=1.6, fov=52, sun={azimuth=-112, elevation=11}, visibility=6000,
  ground=function(X, Z) return 0.18*math.sin(X/2.3 + Z/3.1) + 0.12*math.sin(X/5.7 - Z/1.7) + 0.02*Z end}
-- the erratic: a granite boulder left by the ice, broad and low, turned a little
bs = w:spot_at(-0.5, 6.4)
boulder = body.ellipsoid(bs:p(0, 0.3, 0), bs:size(1.45, 0.82, 1.05))
  :turn(bs:p(0, 0, 0), 0.35, 0.08, -0.07)
  :rough(bs:m(0.12), bs:m(1.3), 3)
  :cut(bs:p(-0.2, 0.95, 0.1), {-0.25, -1, 0.35}, 10, bs:m(0.08))
  :cut(bs:p(0.75, 0.5, 0.3), {0.9, -0.35, 0.3}, 11, bs:m(0.06))
  :rough(bs:m(0.025), bs:m(0.25), 5, true)
w, stone = w:place(bs, boulder)
-- a small second stone half sunk at its right foot
ss = w:spot_at(0.95, 5.5)
w, stone2 = w:place(ss, body.ellipsoid(ss:p(0, 0.05, 0), ss:size(0.34, 0.2, 0.28)):rough(ss:m(0.04), ss:m(0.4), 8))
-- the young spruce behind-right of the stone
ts = w:spot_at(1.55, 7.6)
print("boulder", bs, "spruce", ts, "tall 3.2m =", w:height(ts.x, ts.y, 3.2))
v = w:view()
show(v:bodies_mask{stone, stone2}, {label="stone"})
show(ts.x, ts.y, "spruce foot")

--@ chunk 3 · clock 0
-- the drawing, first pass: a hard pencil, light and searching
h = pencil("2H")
-- the boulder's contour, by eye off the grid
BOULDER = {{186,492},{188,432},{214,388},{262,352},{330,330},{410,318},{498,316},{538,334},{572,378},{596,430},{604,494}}
h:sketch(BOULDER, {pressure=0.3})
h:sketch({{612,550},{626,516},{668,504},{712,506},{738,526},{742,556}}, {pressure=0.28})
-- the young spruce: its axis leaning a hair, the spread of its tiers
h:sketch({{709,484},{711,360},{713,200},{716,56}}, {pressure=0.3})
h:sketch({{716,56},{672,200},{640,330},{612,470}}, {pressure=0.22})
h:sketch({{716,56},{756,200},{790,330},{812,470}}, {pressure=0.22})
-- the wood's edge: tall spruces on the left falling away toward the clearing
WOODTOP = {{0,-10},{120,-6},{250,8},{330,30},{410,64},{480,104},{540,150},{585,200},{612,250},{640,300}}
h:sketch(WOODTOP, {pressure=0.25})
h:sketch({{0,352},{200,350},{400,348},{640,340}}, {pressure=0.2})
-- the far wood line across the clearing, and the horizon behind it
h:sketch({{640,298},{760,292},{880,296},{1000,290}}, {pressure=0.22})
-- the birch stump, near, lower left
STUMP = {{96,700},{100,610},{92,568},{112,556},{128,574},{140,548},{150,600},{154,700}}
h:sketch(STUMP, {pressure=0.3})
