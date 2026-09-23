-- easel session "pencil": a painting replayed chunk by chunk.
--   easel run paintings/lua/pencil.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich_early", size=440, aspect=1.4, seed=11}

--@ chunk 2 · clock 0

-- first pass: a hard pencil, light and searching
HZ = 432
h2 = pencil("2H")
h2:rule({0, HZ}, {1000, HZ}, {pressure=0.3})                 -- the sea horizon, against a ruler
h2:sketch({{0, 470}, {160, 462}, {300, 476}, {352, 490}}, {pressure=0.3})   -- the near ground, to the rock
h2:sketch({{620, 505}, {700, 494}, {1000, 505}}, {pressure=0.3})
ROCK = {{340, 560}, {352, 505}, {385, 468}, {430, 452}, {470, 440}, {530, 446}, {575, 470}, {610, 512}, {640, 560}}
h2:sketch(ROCK, {pressure=0.3})
TRUNK = {{770, 585}, {768, 520}, {776, 460}, {770, 400}, {782, 330}}
h2:sketch(TRUNK, {pressure=0.3})
h2:sketch({{775, 470}, {720, 430}, {690, 380}}, {pressure=0.25})
h2:sketch({{772, 420}, {830, 380}, {860, 330}}, {pressure=0.25})

--@ chunk 3 · clock 0

-- second pass: a soft pencil, firmer, choosing among the searching lines
b = pencil("2B")
b:line({{0, 471}, {160, 463}, {300, 477}, {350, 492}}, {pressure={0.5, 0.65, 0.4}})
b:line({{622, 506}, {700, 495}, {860, 499}, {1000, 504}}, {pressure={0.4, 0.6, 0.5}})
-- the rock: the top came out too high (corrected next)
WRONG = {{343, 556}, {354, 500}, {392, 452}, {445, 424}, {505, 414}, {560, 428}, {602, 468}, {630, 515}, {641, 556}}
b:line(WRONG, {pressure={0.55, 0.7, 0.6, 0.5}})
-- the tree: both sides of the trunk and the two main limbs
b:line({{764, 588}, {762, 520}, {769, 462}, {764, 402}, {776, 332}}, {pressure={0.7, 0.6, 0.35}})
b:line({{779, 588}, {776, 522}, {783, 466}, {778, 404}, {786, 335}}, {pressure={0.7, 0.55, 0.3}})
b:line({{770, 468}, {738, 440}, {712, 410}, {692, 378}}, {pressure={0.55, 0.2}})
b:line({{779, 420}, {812, 398}, {838, 366}, {858, 330}}, {pressure={0.5, 0.2}})
b:line({{778, 380}, {800, 352}, {806, 320}}, {pressure={0.45, 0.15}})

--@ chunk 4 · clock 0

-- the rock top was too high: lift it with the kneaded eraser (twice over) and redraw
local top = {}
for i = 2, #WRONG - 1 do top[#top + 1] = WRONG[i] end
erase(top, {strength=0.9, width=9})
erase(top, {strength=0.6, width=12})
ROCK = {{343, 556}, {350, 512}, {368, 488}, {392, 470}, {432, 454}, {468, 443}, {503, 447}, {532, 444}, {566, 468}, {596, 494}, {616, 520}, {634, 556}}
b:line(ROCK, {pressure={0.55, 0.7, 0.6, 0.5}, smooth=false})
-- a ledge and cracks, the shadow side hatched
b:line({{392, 470}, {420, 492}, {470, 500}, {520, 494}, {566, 468}}, {pressure={0.2, 0.45, 0.3}})
b:line({{470, 500}, {478, 528}, {472, 556}}, {pressure={0.4, 0.15}})
b:hatch(poly({{520, 494}, {566, 468}, {596, 494}, {616, 520}, {634, 556}, {500, 556}}), {angle=-1.1, pressure=0.35})
-- twigs, with a harder pencil
hb = pencil("HB")
for _, t in ipairs({{{692, 378}, {676, 356}, {672, 338}}, {{712, 410}, {690, 402}, {668, 404}},
                    {{838, 366}, {866, 356}, {884, 342}}, {{806, 320}, {798, 298}}, {{858, 330}, {864, 306}}}) do
  hb:line(t, {pressure={0.4, 0.05}})
end

--@ chunk 5 · clock 0

fix()   -- fixative: the drawing is bound before the paint goes on
LEFT, RIGHT = rect(0, 0, 500, H), rect(500, 0, 500, H)
GROUND = {{0, 471}, {160, 463}, {300, 477}, {350, 492}, {622, 506}, {700, 495}, {860, 499}, {1000, 504}}
local rockpoly = {}
for _, p in ipairs(ROCK) do rockpoly[#rockpoly + 1] = p end
rockpoly[#rockpoly + 1] = {634, 562}
rockpoly[#rockpoly + 1] = {343, 562}
rock = poly(rockpoly)
sky = above(function(x) return HZ end)
sea = below(function(x) return HZ end) - below(GROUND)
land = below(GROUND) - rock
tree = ribbon({{771, 590}, {769, 520}, {776, 462}, {771, 402}, {781, 332}}, {15, 13, 11, 9, 6})
PAINT = {
  {sky,  function(x, y) return gradient({{0, "#7d8ea6"}, {0.7, "#b9c0c4"}, {1, "#d9d2bd"}}, y / HZ) end},
  {sea,  "#5d6e78"},
  {land, "#7a7552"},
  {rock, "#8d8a80"},
}
print("ok")

--@ chunk 6 · clock 0

-- left half: thin paint, as in the early works: translucent, oily, lean loads; the drawing shows through
for i, pc in ipairs(PAINT) do
  work(pc[1] * LEFT, {hand=(i == 1) and "broad" or "body", color=pc[2], medium=0.6, load=0.3,
    coverage=2.2, angle=0.04, paint={0.3, 0.3}, clip=LEFT})
end

--@ chunk 7 · clock 0
blend(LEFT, {angle=0.04, clip=LEFT})

--@ chunk 8 · clock 0

-- right half: body color, loaded and opaque; it hides the drawing
for i, pc in ipairs(PAINT) do
  work(pc[1] * RIGHT, {hand=(i == 1) and "broad" or "body", color=pc[2], medium=0.15, load=0.9,
    coverage=4, angle=0.04, clip=RIGHT})
end
blend(RIGHT * sky, {angle=0.02, clip=RIGHT})

--@ chunk 9 · clock 0

dry()   -- the body color is thick: let it dry before painting over it
-- the tree, over its drawing: the trunk along the drawn line, in two loads ...
local tr = brush("round", 7)
tr:load("#3b342c", 1)
tr:stroke({{771, 592}, {769, 520}, {776, 462}}, {pressure={0.85, 0.7}, ramps={0.02, 0.1}})
tr:reload("#3b342c", 1)
tr:stroke({{776, 470}, {771, 402}, {781, 332}}, {pressure={0.7, 0.3}, ramps={0.05, 0.5}})
-- ... and the limbs painted into the firm lines of the hidden drawing
local firm = drawing_mask():band(0.55, 1, 0.1):grow(1.2) * rect(660, 290, 240, 190) * RIGHT
print(string.format("firm drawing in the crown: %.0f sq units", firm:area()))
work(firm, {hand="detail", tool="round 2", color="#3b342c", coverage=3, length={4, 10},
  angle=function(x, y) return (x < 772) and 0.9 or -0.9 end})
dry()
