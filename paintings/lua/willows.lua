-- easel session "willows": a painting replayed chunk by chunk.
--   easel run paintings/lua/willows.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=23, hand=true}; print(H); print(pal); local t=timesheet(); print(t.hours, t.sitting)

--@ chunk 2 · clock 0
HZ = 452
BANK = {{-10,583},{80,580},{170,577},{260,574},{350,571},{420,568},{470,566},{520,566},{580,568},{660,571},{740,575},{830,578},{920,581},{1010,584}}
-- the trunk, drawn as its outline: a squat, swollen, leaning bole, a cleft on the right
TRUNK = {{436,620},{447,604},{458,586},{461,566},{455,546},{447,528},{445,508},{436,494},{428,478},{434,462},{452,454},{474,446},{497,451},{517,441},{540,447},{557,457},{569,472},{562,488},{550,499},{544,512},{549,530},{547,552},{554,574},{566,598},{584,618}}
KNOBS = {{438,470,7,-2.45,0.3},{452,456,9,-2.1,0.28},{474,446,9,-1.85,0.25},{498,448,10,-1.6,0.3},{520,442,10,-1.38,0.26},{542,450,9,-1.1,0.28},{560,464,7,-0.8,0.32}}
RODS = {}
local function rod(x0, y0, a, len, w, bend, depth)
  local pts = {{x0, y0}}
  local n = 5
  local ax = a
  for s = 1, n do
    ax = ax + bend / n + randn(0, 0.025)
    -- the outer rods arch over: gravity pulls their tips
    if math.abs(a + 1.57) > 0.55 then ax = ax + 0.05 * (a < -1.57 and -1 or 1) * s / n end
    local px, py = pts[#pts][1], pts[#pts][2]
    pts[#pts + 1] = {px + math.cos(ax) * len / n, py + math.sin(ax) * len / n}
  end
  RODS[#RODS + 1] = {pts = pts, len = len, w = w, depth = depth}
  return pts
end
for k, kn in ipairs(KNOBS) do
  for i = 1, kn[3] do
    local a = kn[4] + randn(0, kn[5])
    local len = rand(110, 320) * (1 - 0.3 * math.abs(a + 1.57))
    if rand() < 0.12 then len = len * 0.4 end
    local x0, y0 = kn[1] + rand(-5, 5), kn[2] + rand(-4, 4)
    local w = clamp(len / 90, 1.0, 3.2) * rand(0.8, 1.2)
    local pts = rod(x0, y0, a, len, w, randn(0, 0.14), 0)
    -- an older rod forks: one or two side rods from its lower half
    if len > 200 and rand() < 0.35 then
      for f = 1, (rand() < 0.5 and 1 or 2) do
        local j = math.random(2, 3)
        local p = pts[j]
        rod(p[1], p[2], a + (rand() < 0.5 and -1 or 1) * rand(0.2, 0.45), len * rand(0.35, 0.6), w * 0.6, randn(0, 0.1), 1)
      end
    end
  end
end

-- the far shore: a low wood on the left, a row of small pollards on the right, lost in mist at the end
FAR = {{-10,452},{40,450},{62,442},{90,436},{120,433},{150,436},{180,431},{214,434},{246,438},{280,444},{310,449},{360,451},{460,452},{600,451},{700,450},{800,451},{900,452},{1010,452}}
FARWILL = {}
local xs = uneven(11, 612, 905, 0.6, 0.4, 5)
for i, x in ipairs(xs) do FARWILL[i] = {x, 451, rand(7, 14) * (1 - 0.3 * (x - 612) / 300)} end
print(#RODS, #FARWILL)
-- the drawing: a searching 2H pass, then 3B for what I'm sure of
local h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.3})
h:sketch(BANK, {pressure=0.3})
h:sketch(TRUNK, {pressure=0.3})
h:sketch(FAR, {pressure=0.25})
local b = pencil("3B")
b:line(TRUNK, {pressure={0.55, 0.7, 0.65, 0.5}, smooth=true})
b:line(BANK, {pressure={0.45, 0.55, 0.5}})
for i, r in ipairs(RODS) do
  if r.depth == 0 and i % 2 == 1 then b:line(r.pts, {pressure={0.5, 0.2}}) else h:line(r.pts, {pressure={0.4, 0.15}}) end
end
for _, f in ipairs(FARWILL) do h:line({{f[1], f[2]}, {f[1] + randn(0, 1), f[2] - f[3]}}, {pressure=0.35}) end
-- the cleft in the bole
b:line({{516,452},{520,480},{514,510},{518,540},{512,566}}, {pressure={0.4, 0.65, 0.4}})
fix()

--@ chunk 3 · clock 4.672002384904772
umber = pal:only{"raw umber", "bone black", "lead white", "yellow ochre"}
bankM = below(BANK):roughen(1.2, 30, 3, 0.8)
trunkM = poly(TRUNK, true)
-- the sepia lay-in: thin raw umber, in values only, two washes crossing a little
work(bankM, {hand="glaze", color="#5b4633", medium=0.6, coverage=1.4, angle=0.02, length={80, 220}, pal=umber, edge="found"})
work(bankM * below(function(x) return 610 + 10 * math.sin(x / 90) end), {hand="glaze", color="#4d3b2b", medium=0.6, coverage=1.4, angle=-0.06, length={60, 200}, pal=umber, edge="found"})
work(trunkM, {hand="glaze", tool="filbert 7", color="#3d2f23", medium=0.6, coverage=1.4, angle=1.52, length={15, 50}, pal=umber, edge={found=0.5, soft=0.5, period=30}})
work(trunkM, {hand="glaze", tool="filbert 5", color="#3d2f23", medium=0.6, coverage=1.4, angle=1.3, length={15, 40}, pal=umber, edge={found=0.5, soft=0.5, period=30}})
