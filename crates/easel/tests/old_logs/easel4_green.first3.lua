-- easel session "easel4_green": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_green.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}; print(W,H); print(pal)

--@ chunk 2 · clock 0
HZ = 296
w = world{horizon=HZ, eye=28, fov=52, sun={azimuth=-118, elevation=36},
  ground=function(X, Z) return 1.5*math.sin(X/70+0.4)*math.min(1, Z/300) + 6*math.sin(Z/900 + X/1300) end}
BROW = {{-10,474},{90,466},{190,470},{300,482},{400,494},{520,512},{640,534},{760,552},{880,566},{1010,574}}
RIVER = {{1010,420},{900,398},{780,372},{700,352},{640,338},{600,326},{560,316},{530,308},{505,302}}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
h:sketch(BROW, {pressure=0.3})
h:sketch(RIVER, {pressure=0.25})
-- oak: trunk and crown envelope
h:sketch({{205,482},{210,420},{206,360},{214,300}}, {pressure=0.3})
h:sketch({{228,482},{226,420},{230,360},{226,300}}, {pressure=0.3})
h:sketch({{120,330},{90,260},{110,180},{170,120},{240,98},{320,130},{352,210},{330,300},{280,340}}, {pressure=0.25})
-- boulder
h:sketch({{378,508},{384,478},{410,458},{446,452},{470,466},{482,496},{476,516}}, {pressure=0.3})
-- seated figure, back to us
h:sketch({{494,512},{492,494},{496,478},{500,470},{506,478},{510,496},{512,512}}, {pressure=0.3})
-- village and spire
h:sketch({{640,322},{660,318},{700,317},{740,319},{770,322}}, {pressure=0.25})
h:rule({708, 318}, {708, 282}, {pressure=0.3})

--@ chunk 3 · clock 0
b2 = pencil("2B")
b2:line(BROW, {pressure={0.5, 0.6, 0.5}})
b2:line({{120,330},{90,260},{110,180},{170,120},{240,98},{320,130},{352,210},{330,300},{280,340}}, {pressure=0.45})
b2:line({{205,482},{210,420},{206,360},{214,300}}, {pressure=0.5})
b2:line({{228,482},{226,420},{230,360},{226,300}}, {pressure=0.5})
b2:line({{378,508},{384,478},{410,458},{446,452},{470,466},{482,496},{476,516}}, {pressure=0.55, smooth=false})
b2:line({{494,512},{492,494},{496,478},{500,470},{506,478},{510,496},{512,512}}, {pressure=0.5})
b2:line(RIVER, {pressure=0.35})
b2:rule({0, HZ}, {1000, HZ}, {pressure=0.35})
show(BROW, {label="brow"})
