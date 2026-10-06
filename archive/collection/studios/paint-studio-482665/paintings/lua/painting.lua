-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=1000, aspect=1.25, linen={14,13}, seed=11,
 ground={
  {pile={{"red earth",2},{"yellow ochre",2},{"lead white",3}}, um=110, apply="knife", texture=0.25},
  {pile={{"lead white",8},{"yellow ochre",0.8},{"raw umber",0.1}}, um=55, apply="brush"}
 }}
print(W, H)

--@ chunk 2
-- Underdrawing in graphite: the plan of the picture, ruled lightly like Friedrich's.
h = pencil("2H")

-- far mountains right of the sun
R4 = {{545,471},{600,467},{660,458},{705,446},{742,433},{772,426},{796,437},{822,424},{850,410},{872,404},{898,414},{925,432},{955,447},{1000,455}}
-- long low far shore
R3 = {{0,487},{70,482},{150,485},{240,479},{330,483},{410,478},{490,482},{545,475},{620,479},{700,476},{790,472},{880,477},{1000,469}}
-- wooded hills standing out of the fog
RA = {{150,528},{190,508},{230,492},{262,485},{295,486},{322,497},{338,520}}
RB = {{450,526},{488,509},{526,500},{560,497},{592,503},{625,514},{665,528}}
-- the near hill's crest, falling away to the right
CREST = {{0,545},{60,548},{130,552},{200,550},{270,546},{330,544},{372,541},{412,543},{455,551},{510,566},{580,588},{660,612},{760,636},{860,655},{1000,672}}

h:sketch(R4, {pressure=0.2, passes=2, wander=1.0})
h:sketch(R3, {pressure=0.2, passes=2, wander=1.0})
h:sketch(RA, {pressure=0.22, passes=2, wander=1.0})
h:sketch(RB, {pressure=0.22, passes=2, wander=1.0})
h:sketch(CREST, {pressure=0.3, passes=2, wander=1.2})

-- the rock the wanderer stands on
h:sketch({{352,546},{360,538},{380,534},{404,536},{420,543},{426,552}}, {pressure=0.28, passes=2, wander=0.8})

-- the oak: trunk and the great limbs
OAK = {
  trunk = {{172,634},{176,590},{184,540},{192,490},{198,452}},
  tw = {58,46,40,36,32},
  limbs = {
    {pts={{190,468},{160,420},{122,365},{96,300},{90,235},{102,170},{90,110},{64,55},{48,8}}, w={26,22,17,13,10,8,6,4,3}},
    {pts={{200,450},{214,395},{232,335},{230,275},{246,212},{242,150},{262,92},{258,30}}, w={24,19,15,12,9,7,5,3}},
    {pts={{206,458},{250,432},{300,410},{352,386},{404,352},{452,324},{498,300},{544,284}}, w={20,16,13,10,8,6,4,3}},
    {pts={{176,505},{128,520},{84,514},{44,528},{0,524}}, w={20,16,12,9,6}},
  }}
local L, Rr = {}, {}
for i, p in ipairs(OAK.trunk) do
  L[#L+1] = {p[1] - OAK.tw[i]/2, p[2]}
  Rr[#Rr+1] = {p[1] + OAK.tw[i]/2, p[2]}
end
h:sketch(L, {pressure=0.25, passes=2, wander=1.0})
h:sketch(Rr, {pressure=0.25, passes=2, wander=1.0})
for _, lb in ipairs(OAK.limbs) do
  h:sketch(lb.pts, {pressure=0.22, passes=2, wander=1.2})
end

-- the wanderer: a man seen from behind, on the rock
h:line({{392,483},{396,486},{397,490},{395,494},{401,497},{403,508},{402,520},{405,541}}, {pressure=0.4})
h:line({{392,483},{388,486},{387,490},{389,494},{384,497},{381,508},{381,521},{379,541}}, {pressure=0.4})
h:line({{374,479},{376,510},{377,542}}, {pressure=0.35})

-- spruces on the right crest
h:sketch({{960,666},{958,540},{955,420}}, {pressure=0.25, passes=2, wander=1.0})
h:sketch({{906,658},{906,580},{908,500}}, {pressure=0.25, passes=2, wander=1.0})
h:sketch({{994,668},{992,560},{994,455}}, {pressure=0.25, passes=2, wander=1.0})
print("drawn")
