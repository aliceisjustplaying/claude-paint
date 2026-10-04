-- easel session "legacy_tiny": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 392
knollg = function(X, Z)
  return 3.2*math.exp(-(((X + 6)/42)^2 + ((Z - 78)/34)^2)) + 0.6*math.sin(X/23 + Z/31) * math.min(1, Z/40)
end
w = world{horizon=HZ, eye=6, fov=50, sun={azimuth=-118, elevation=40}, ground=knollg}
sk = w:sky{haze=2.0, uneven={0.35, 30000, 7}}
cl = w:clouds{sky=sk, cell=3, {kind="cumulus", x=4600, z=14000, base=1250, width=3000, height=1400, seed=31}}
skym = above(function(x) return HZ + 14 end) * rect(0, 0, 400, 1000)
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "red earth"}
work(skym, {hand="broad", color=cl, angle=0, coverage=2, medium=0.3, pal=skypal})
print(w:scale_at(357), w:height(430, 480, 15))

--@ chunk 3 · clock 0
air = haze{visibility=22000, height=900, mist={60, 4, 900, 7}}
rs = w:ranges{near=9000, far=26000, count=2, seed=19, heights={160, 620}, kinds={"dome", "saddle"}}
for i = #rs, 1, -1 do
  local l = rs[i]
  work(l:mask() * above(function(x) return HZ + 3 end) * rect(600, 0, 400, 1000), {hand="body", length={20, 60}, coverage=2, angle=0.04,
    color=function(x, y) return mix("#44566a", sk:airlight(x), 0.8*l:haze(air, x, y)) end, medium=0.3})
end
for i, l in ipairs(rs) do print(i, l:crest(200), l:crest(600), l:crest(900)) end

--@ chunk 4 · clock 0
wait(24*60)
fieldcells = worley{seed=9, period=70}
groundcol = function(x, y)
  local p = w:to_ground(x, y)
  local X, Z = 0, 4000
  if p then X, Z = p[1], p[3] end
  local _, _, edge, r = fieldcells:at(X * 0.55, Z * 1.25)
  local c = mix("#6f8a3c", "#a89452", r)
  return mix(c, sk:airlight(x), math.min(0.85, w:aerial(Z) * 0.95))
end
land = below(function(x) return HZ + 3 end):roughen(0.8, 10) * rect(0, 380, 500, 120)
work(land, {hand="body", length={18, 60}, coverage=2, medium=0.2, angle=0.03, color=groundcol})
hazem = mask(function(x, y) return (y > HZ - 30 and y < HZ) and 1 or 0 end):blur(1.5) * rect(0, 0, 300, 1000)
glaze(hazem, {color="#c6cbd2", coats=0.55, pigment="semi"})
stipple(hazem, {width=1.5, color="#b9c0c9", coverage=1, pressure={0.35, 0.7}, dips={20, 0.3, 0.7}, aim=false, medium=0.6, fade=1})

--@ chunk 5 · clock 1440
OX, OY = 430, 486
oak = tree{habit="oak", x=OX, y=OY, height=150, seed=17, years=10}
oakleaves = oak:foliage{sun={-0.75, -0.6, 0.3}, seed=17, clump=0.04, spray=2}
local s = w:spot(OX, OY)
w = w:proxy(s, body.ellipsoid(s:p(-1.5, 10, 0), s:size(5.2, 3.8, 4.6)):rough(0.5, 3, 4))
w = w:proxy(s, body.block(s:p(0, 3, 0), s:size(0.5, 3, 0.5), s:m(0.2)))
v = w:view()
local sa = w:shadow_angle(OX, OY + 2) or 0
local sh = (v:shadows() * v:land()):roughen(4, 10, 3, 1):soften(1.5)
work(sh * rect(300, 400, 300, 200), {hand="hatch", tool="round 1.3", length={2.5, 6}, coverage=2, angle=-1.5,
  color_over={shift={-0.06, -0.004, -0.017}}})
local big = nil
for _, l in ipairs(oak.limbs) do
  if #l.pts >= 2 and l.w[1] > 4 then local r = ribbon(l.pts, l.w); big = big and (big + r) or r end
end
if big then work(big, {hand="body", tool="filbert 3", color="#27241f", angle=math.pi / 2, length={6, 18}, coverage=2, medium=0.12, clip=big}) end
work(oakleaves:mask() * rect(330, 300, 200, 200), {hand="detail", tool="round 1.4", length={2, 4}, coverage=1, color="#3d4a2a", angle=0.3})
work(oakleaves:gaps(6) * rect(330, 300, 200, 200), {hand="detail", tool="round 1.4", length={2, 4}, coverage=1, color="#c6cbd2", angle=0.3, pal=skypal})

--@ chunk 6 · clock 1440
local function stone(c, r, s, yaw, roll)
  return body.ellipsoid(c, r):turn(c, yaw or 0, 0, roll or 0):rough(0.16 * math.max(r[1], r[2]), 0.7 * math.max(r[1], r[2]), s)
end
dol = stone({230, 484, 0}, {17, 44, 17}, 1, 0.3, 0.14):cut({230, 470, 0}, {0.1, -0.97, 0.2}, 11, 3)
cap = stone({318, 470, -6}, {60, 20, 30}, 5, 0.15, -0.06)
df = form{ {dol + cap, dist=0.3}, light={from={0.8, -0.5}, front=-0.35, ambient=0.3, penumbra=0.1} }
stonesil = df:silhouette{parts={1}, soft=0.4}:roughen(0.9, 16, 12, 0.6)
work(stonesil, {hand="body", tool="flat 5", color=function(x, y) return mix("#2a2823", "#3a362f", smoothstep(0, 0.9, df:value(x, y))) end,
  angle=df:field("across"), length={10, 30}, coverage=2, load=1, medium=0.12, clip=stonesil})
local rim = stonesil * df:lit{parts={1}, soft=0.25}
print(df:part(318, 470), df:part(900, 100))
stipple(rim, {width=1.6, color="#7d6c52", coverage=1.5, pressure={0.35, 0.7}, dips={18, 0.4, 0.6}, medium=0.3, clip=stonesil})
work(df:edges{concave=true} * stonesil, {hand="detail", tool="round 0.8", color="#2e2b27", angle=df:field("edge"), coverage=1})

--@ chunk 7 · clock 1440
local sp = w:spot(709, 492)
local w2 = w:proxy(sp, body.ellipsoid(sp:p(0, 1.6, 0), sp:size(0.75, 1.6, 0.75)))
vs = w2:view()
glaze(vs:cast_shadow{soft=2.2} * -v:visible("bodies"), {color="#6a7090", coats=0.32, view=vs})
glaze(vs:contact_shadow{reach=0.18}, {color="#4a4a58", coats=0.32, view=vs})
region = rect(560, 560, 300, 120)
tufts = sward{region=region, horizon=HZ, near=H, height=36, flowers=0.0, seed=4, thin=0.3, patch=0.6, patch_size=110,
  wind={lean=0.22, gust=0.3, period=140, seed=2}}
local g = brush("rigger", 0.7)
g:reload("#48463a", 0.7)
local n = 0
for i, t in ipairs(tufts) do
  if i <= 12 then
    for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={0.4, 0.0}, ramps={0.05, 0.7}}); n = n + 1 end
  end
end
print(#tufts, n)

--@ chunk 8 · clock 1440
dry(); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief(0.14)
