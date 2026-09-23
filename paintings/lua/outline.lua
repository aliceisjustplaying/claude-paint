-- easel study "outline": drawn outlines (outline{} and body_of{}).
--   easel run paintings/lua/outline.lua [--width 3200]
-- Top: one rock from the same seven rough points in three hands (firm,
-- searching, broken). Below: a tree line (soft, open), sheep from five spine
-- points and four legs, a standing figure from a skeleton, bracken whose
-- pinnae are three points each. Every shape is filled inside its own mask
-- and its contour painted with a pointed brush along the hand's strokes.

--@ chunk 1
canvas{style="friedrich", aspect=1.4, seed=5}

--@ chunk 2
-- a pale sheet above, a sky strip and a meadow below
work(rect(0, 0, 1000, 300), {hand="broad", color=function(x, y) return mix("#d3d3cb", "#dcd3bb", y/300) end, angle=0, coverage=3.5, medium=0.25})
work(rect(0, 300, 1000, 120), {hand="broad", color=function(x, y) return mix("#b9c2c4", "#e0d6b8", (y - 300)/120) end, angle=0, coverage=3.5, medium=0.25})
work(rect(0, 420, 1000, 300), {hand="broad", color=function(x, y) return mix("#a39f78", "#7f7b57", (y - 420)/290) end, angle=0.05, coverage=3.5, medium=0.2})
wait(24*60*20)
dry()

--@ chunk 3
-- the same seven rough points, three hands: "c" marks a corner
local R = {{-132, 84, "c"}, {-120, 8}, {-78, -58, "c"}, {-8, -70}, {58, -92, "c"}, {128, -12, "c"}, {140, 86, "c"}}
local function rock(cx, cy, ch, seed)
  local p = {}
  for i, q in ipairs(R) do p[i] = {cx + q[1], cy + q[2], q[3]} end
  return outline{pts=p, char=ch, seed=seed}
end
local line = brush{kind="round", width=4}
rocks = {}
for i, ch in ipairs({"firm", "searching", "broken"}) do
  local o = rock(175 + (i-1)*325, 165, ch, 11)
  rocks[i] = o
  print(ch, o)
  local m = o:mask()
  work(m, {hand="body", color=function(x, y) return mix("#a8a190", "#5e5a50", (y - 90)/170) end, angle=0.3, length={8, 25}, coverage=3, clip=m})
  -- a lit rim just inside the top edge: the mask minus its inset
  local rim = (m - o:inset(7):mask()) * rect(0, 0, 1000, 130):blur(25)
  work(rim, {hand="detail", color="#c4bba6", angle=0.2, length={4, 12}, coverage=2, clip=rim})
  line:load("#2e2b26", 0.9)
  o:paint(line, {pressure=1, dip={"#2e2b26", 0.8}, every=4})
end

--@ chunk 4
-- a tree line: soft lobes along an open line, the wood below it
local tl = outline{{-10, 372}, {140, 352}, {300, 362}, {470, 334}, {640, 356}, {820, 342}, {1010, 360}, open=true, char="soft", lobe=24, seed=4}
print(tl)
local wood = tl:below(440) - rect(0, 432, 1000, 300)
work(wood, {hand="body", color=function(x, y) return mix("#4f5646", "#3c4236", (y - 330)/100) end, angle=0.4, length={6, 16}, coverage=3, clip=wood, tool="round 5"})
work(rect(0, 425, 1000, 16), {hand="broad", color="#8e8c68", angle=0, coverage=2.5})

--@ chunk 5
-- sheep: five spine points (rump, back, shoulder, poll, nose) and four legs
local function sheep(x, y, s, d, graze, seed)
  local function P(u, v) return {x + d*u*s, y + v*s} end
  local head = graze and {P(12, -4), P(14.5, 0)} or {P(12, -10), P(15, -9)}
  local spine, widths = {P(-13, -7), P(-4, -8), P(6, -8), head[1], head[2]}, {8*s, 10*s, 8.5*s, 3.6*s, 2.4*s}
  local o = body_of{spine=spine, widths=widths,
    limbs={{P(-10, -5), P(-10.5, 2), widths={1.3*s, 0.9*s}}, {P(-7, -5), P(-7.3, 2.2), widths={1.3*s, 0.9*s}},
           {P(4, -5), P(4.4, 1.8), widths={1.2*s, 0.9*s}}, {P(7, -5), P(7.6, 2), widths={1.2*s, 0.9*s}}},
    char="soft", seed=seed}
  -- the fleece: the same spine without the head, a little fuller
  local fleece = body_of{spine={spine[1], spine[2], spine[3]}, widths={8.4*s, 10.4*s, 9*s}, char="soft", seed=seed + 50}:mask() * o:mask()
  local m = o:mask()
  local dark = m - fleece
  work(dark, {hand="body", color="#3d3830", angle=1.5, length={3, 8}, coverage=3, tool="round 1.5", clip=dark})
  work(fleece, {hand="body", color=function(px, py) return mix("#dcd6c2", "#8d8878", (py - y + 12*s)/(9*s)) end, angle=0, length={3, 8}, coverage=3, tool="round 2.5", clip=fleece})
  local lb = brush{kind="round", width=1.6}
  lb:load("#3a362e", 0.9)
  o:paint(lb, {pressure=0.7, dip={"#3a362e", 0.8}, every=3})
  return o
end
print("sheep", sheep(110, 560, 3.2, 1, false, 21))
print("sheep", sheep(270, 600, 3.0, -1, true, 22))
print("sheep", sheep(385, 548, 2.2, 1, true, 23))

--@ chunk 6
-- a standing figure in a long coat, from behind: a spine, legs, arms, a head
local x, y = 540, 680
local fig = body_of{spine={{x, y-152}, {x, y-132}, {x+1, y-96}, {x+3, y-52}}, widths={9, 30, 29, 38},
  limbs={{{x-7, y-60}, {x-7, y-28}, {x-8, y-2}, widths={10, 8, 7}}, {{x+8, y-60}, {x+9, y-28}, {x+11, y-2}, widths={10, 8, 7}},
         {{x-14, y-134}, {x-19, y-106}, {x-15, y-80}, widths={9, 8, 6}}, {{x+14, y-134}, {x+18, y-106}, {x+16, y-82}, widths={9, 8, 6}},
         {{x, y-163}, {x+1, y-164}, widths={16, 16}}},
  blend=0.5, char="firm", seed=31}
print("figure", fig)
local fm = fig:mask()
work(fm, {hand="body", color=function(px, py) return mix("#3b3630", "#2a2622", (py - y + 160)/160) end, angle=1.5, length={4, 12}, coverage=3, tool="round 3", clip=fm})
-- light catching the left edge of the coat: a band along the contour, inside only, on the left
local lit = fig:band(4, 0.5) * fm * rect(0, 0, x - 4, 714)
work(lit, {hand="detail", color="#5d554a", angle=1.5, length={3, 8}, coverage=2, clip=lit})
local fb = brush{kind="round", width=2.2}
fb:load("#1f1c19", 0.9)
fig:paint(fb, {pressure=0.75, dip={"#1f1c19", 0.8}, every=3})
-- its shadow on the grass: the drawn contour (a plain point list) laid
-- flat, every 8th point, redrawn as a soft outline and glazed
local p = {}
for i, q in ipairs(fig:path()) do
  if i % 8 == 0 then p[#p+1] = {q[1] + (y - q[2])*1.1, y + 1 + (y - q[2])*0.07} end
end
local shadow = outline{pts=p, char="soft", corners=false, amount=0.5, seed=32}:mask() - fm
glaze(shadow, {color="#4a4a30", coats=0.45})

--@ chunk 7
-- bracken: a rachis of three points; each pinna three points, drawn as a
-- small body whose soft lobes are its pinnules
local function frond(x, y, len, ang, bend, seed)
  local tip = {x + len*math.cos(ang), y + len*math.sin(ang)}
  local mid = {x + 0.5*len*math.cos(ang - bend), y + 0.5*len*math.sin(ang - bend)}
  local r = outline{{x, y}, mid, tip, open=true, char="firm", seed=seed}
  local pin = nil
  local n = 13
  for i = 2, n do
    local t = i / (n + 1)
    local px, py, tx, ty, nx, ny = r:at(t)
    for side = -1, 1, 2 do
      if rand() < 0.92 then
        local pl = len * 0.34 * (1 - t)^0.8 * (0.75 + 0.5*rand())
        local fl = 0.25 + 0.35*rand()
        local e = {px + side*nx*pl + tx*pl*fl, py + side*ny*pl + ty*pl*fl + 0.3*pl*rand()}
        local m = {px + side*nx*pl*0.5 + tx*pl*0.12, py + side*ny*pl*0.5 + ty*pl*0.12 - 0.06*pl}
        local w = math.max(2.2, pl*0.3)
        local b = body_of{spine={{px, py}, m, e}, widths={w*0.7, w, w*0.25}, char="soft", lobe=w*0.55, edge=0, seed=seed*100 + i*2 + side}
        local bm = b:mask()
        pin = pin and (pin + bm) or bm
      end
    end
  end
  work(pin, {hand="detail", color=function(px, py) return mix("#9a5a2c", "#6e3d1f", (py - y + len)/len) end, angle=ang, length={2, 6}, coverage=3, tool="round 2", clip=pin})
  local rb = brush{kind="round", width=2.6}
  rb:load("#5a3218", 0.9)
  r:paint(rb, {pressure=0.8})
end
frond(720, 700, 200, -1.25, 0.25, 7)
frond(805, 705, 170, -1.8, -0.3, 8)
frond(900, 705, 150, -1.45, 0.5, 9)
