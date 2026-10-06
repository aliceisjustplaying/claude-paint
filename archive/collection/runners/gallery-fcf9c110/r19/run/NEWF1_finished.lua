-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=740, aspect=1.5, linen={18,16}, seed=27, ground={
 {pile={{"red earth",2},{"yellow ochre",3},{"lead white",3}},um=65,apply="knife",texture=0.13},
 {pile={{"lead white",12},{"yellow ochre",1},{"raw umber",0.3}},um=42,apply="knife",texture=0.08},
 {pile={{"lead white",22},{"yellow ochre",0.55},{"bone black",0.14}},um=30,apply="brush"}}}
-- A measured, faint skeleton, not a complete cartoon.
h=pencil("2H")
h:sketch({{0,379},{103,352},{179,359},{275,320},{357,344},{457,303},{556,278},{624,312},{714,294},{819,334},{1000,347}},{pressure=0.19,passes=2,wander=0.5})
h:sketch({{0,542},{89,528},{166,555},{231,544},{312,565},{394,591},{474,580},{548,539},{592,511},{628,514},{686,501},{770,474},{862,462},{939,473},{1000,458}},{pressure=0.28,passes=2,wander=0.6})
h:sketch({{240,563},{226,529},{227,477},{215,421},{225,371},{214,321},{219,281},{239,246},{247,214}},{pressure=0.25,passes=2,wander=0.6})
h:sketch({{220,387},{179,353},{160,310},{117,292},{87,259},{47,249}},{pressure=0.21,passes=2})
h:sketch({{224,329},{269,316},{301,278},{359,268},{393,237},{438,221}},{pressure=0.19,passes=2})
h:sketch({{339,667},{402,620},{485,589},{550,550},{592,516}},{pressure=0.16,passes=2})
h:rule({690,395},{690,367},{pressure=0.18})
fix()
skyCool=pile{{"lead white",14},{"smalt",3.3},{"bone black",0.42},{"red earth",0.13},medium=0.19}
skyMid=pile{{"lead white",18},{"pale smalt",2.6},{"bone black",0.22},{"red earth",0.28},medium=0.21}
skyPearl=pile{{"lead white",22},{"yellow ochre",0.7},{"vermilion",0.17},{"pale smalt",0.7},medium=0.2}
work(rect(0,0,1000,424),{hand="broad",pile=skyMid,coverage=2.6,load=0.64,angle=0,edge="soft",fill=true,length={110,230},seed=100})
work(poly({{0,0},{1000,0},{1000,154},{804,173},{637,190},{452,188},{226,204},{0,192}}):soften(58),{hand="broad",pile=skyCool,coverage=1.7,load=0.52,angle=0.03,edge="lost",length={140,260},seed=101})
work(poly({{0,320},{193,293},{388,282},{546,274},{727,285},{1000,280},{1000,436},{0,436}}):soften(58),{hand="broad",pile=skyPearl,coverage=2,load=0.55,angle=0,edge="lost",length={130,240},seed=102})
blend(rect(0,0,1000,430),{angle=0,coverage=0.5,seed=103})
print(W,H)

--@ chunk 2
wait(36*60)
upperBlue=pile{{"lead white",7},{"smalt",8},{"cobalt blue",1.7},{"bone black",0.55},{"red earth",0.18},medium=0.26}
work(poly({{0,0},{1000,0},{1000,144},{869,155},{713,159},{532,184},{340,192},{190,210},{0,201}}):soften(65),{hand="broad",pile=upperBlue,tool="filbert 28",coverage=2.8,load=0.68,dips={2,0.68,0.9},length={125,240},angle=0,clip=true,fill=true,seed=112})
blend(rect(0,0,1000,250),{angle=0,coverage=0.6,seed=113})
warmDawn=pile{{"lead white",17},{"yellow ochre",1.55},{"vermilion",0.33},{"red earth",0.15},medium=0.25}
work(poly({{0,371},{146,341},{324,341},{476,334},{620,337},{751,327},{870,340},{1000,334},{1000,441},{0,441}}):soften(48),{hand="broad",pile=warmDawn,coverage=2.1,load=0.57,length={85,175},angle=0.02,clip=true,fill=true,seed=114})
blend(rect(0,282,1000,162):soften(10),{angle=0,coverage=0.45,seed=115})

--@ chunk 3
print(drying(500,145),drying(500,275))
bridgeSky=pile{{"lead white",16},{"pale smalt",5},{"cobalt blue",0.95},{"bone black",0.32},{"red earth",0.19},medium=0.27}
work(poly({{0,141},{162,153},{350,151},{529,128},{700,127},{863,116},{1000,117},{1000,314},{833,325},{685,305},{531,315},{372,310},{179,332},{0,329}}):soften(52),{hand="body",tool="filbert 18",pile=bridgeSky,coverage=2.4,load=0.55,length={42,90},angle=0,edge="lost",clip=true,seed=121})
blend(rect(0,78,1000,275):soften(18),{tool="badger 56",angle=math.pi/2,length={130,240},coverage=1.4,clip=true,seed=122})
-- A quieter pearly veil immediately over the horizon.
pearlAir=pile{{"lead white",22},{"pale smalt",1.2},{"yellow ochre",0.6},{"vermilion",0.12},{"bone black",0.10},medium=0.3}
work(poly({{0,277},{160,264},{365,270},{532,257},{674,267},{827,250},{1000,251},{1000,361},{858,364},{650,356},{468,369},{274,359},{0,376}}):soften(35),{hand="broad",pile=pearlAir,coverage=1.9,load=0.52,length={60,155},angle=0,clip=true,seed=123})
blend(rect(0,230,1000,166):soften(10),{tool="badger 48",angle=1.57,coverage=0.9,length={75,130},seed=124})

--@ chunk 4
wait(2*24*60)
quietBlue=pile{{"lead white",14},{"smalt",5},{"cobalt blue",0.85},{"bone black",0.26},{"red earth",0.15},medium=0.28}
quietPearl=pile{{"lead white",22},{"pale smalt",3},{"cobalt blue",0.33},{"bone black",0.16},{"red earth",0.20},medium=0.28}
stipple(rect(-20,93,1040,167):soften(60),{pile=quietBlue,width=5,coverage=2.5,pressure={0.2,0.5},dips={8,0.55,0.9},feather=1,clip=true,cluster=0.06,seed=131})
stipple(poly({{0,204},{252,207},{453,194},{661,194},{862,181},{1000,180},{1000,327},{721,317},{504,334},{249,340},{0,339}}):soften(48),{pile=quietPearl,width=5,coverage=2.4,pressure={0.18,0.47},dips={8,0.5,0.9},feather=1,clip=true,cluster=0.04,seed=132})
blend(rect(0,50,1000,332),{angle=0,coverage=0.7,tool="badger 36",length={110,220},seed=133})

--@ chunk 5
wait(3*24*60)
stratus=pile{{"lead white",18},{"pale smalt",3},{"cobalt blue",0.46},{"bone black",0.25},{"red earth",0.16},medium=0.25}
cloudDeck=outline{{-30,196},{98,190},{194,199},{306,177},{419,186},{521,173},{666,174},{776,163},{887,156},{1030,166},{1034,270},{916,272},{822,281},{687,272},{577,290},{432,288},{324,300},{167,294},{-30,305},char="soft",amount=0.55,lobe=28,edge=14,seed=140}:mask()
work(cloudDeck,{hand="detail",tool="filbert 6",pile=stratus,coverage=5,load=0.65,dips={2,0.65,0.94},length={13,28},angle=0,clip=true,fill=true,seed=141})
lowHaze=pile{{"lead white",24},{"pale smalt",1.4},{"yellow ochre",0.60},{"vermilion",0.12},{"bone black",0.12},medium=0.29}
work(poly({{-20,296},{210,281},{457,275},{632,283},{861,261},{1020,257},{1020,371},{829,365},{623,369},{413,378},{187,372},{-20,388}},true):soften(20),{hand="detail",tool="filbert 6",pile=lowHaze,coverage=4.6,load=0.6,dips={3,0.6,0.94},length={14,30},angle=0,clip=true,fill=true,seed=142})

--@ chunk 6
transitionBlue=pile{{"lead white",12},{"smalt",4},{"cobalt blue",0.9},{"bone black",0.29},{"red earth",0.14},medium=0.3}
work(poly({{-20,78},{206,95},{428,84},{641,79},{845,71},{1020,73},{1020,187},{864,176},{722,197},{529,201},{351,213},{166,226},{-20,223}},true):soften(30),{hand="detail",tool="filbert 6",pile=transitionBlue,coverage=3.8,load=0.57,dips={3,0.57,0.94},length={18,36},angle=0,clip=true,seed=151})
lose(cloudDeck,{pile=transitionBlue,where=function(x,y) if y<230 then return 0.92 else return 0 end end,tool="filbert 9",reach={22,26},load=0.18,pressure={0.24,0.015},every=0.65,angle=1.57,seed=152})
-- Long, very low relief streaks of vapour, separately drawn.
cloudShade=pile{{"lead white",10},{"smalt",3},{"cobalt blue",0.35},{"bone black",0.4},{"red earth",0.26},medium=0.36}
local c1=poly({{-12,109},{78,103},{168,107},{239,100},{312,105},{356,111},{279,118},{198,117},{107,121},{-12,118}},true):soften(5)
local c2=poly({{539,64},{607,58},{684,59},{757,55},{843,62},{932,59},{1010,67},{1010,77},{886,73},{785,78},{678,72},{594,75}},true):soften(6)
local c3=poly({{372,232},{446,228},{555,231},{622,226},{700,224},{780,229},{724,236},{600,235},{527,240},{431,238}},true):soften(6)
work(c1+c2,{hand="detail",tool="filbert 5",pile=cloudShade,coverage=1.5,load=0.28,length={35,85},angle=0,clip=true,seed=153})
work(c3,{hand="detail",tool="filbert 4",pile=skyMid,coverage=1.7,load=0.3,length={25,65},angle=0,clip=true,seed=154})

--@ chunk 7
wait(3*24*60)
unifiedSky=pile{{"lead white",6.5},{"smalt",7},{"cobalt blue",1.4},{"bone black",0.43},{"red earth",0.18},medium=0.27}
work(rect(0,0,1000,435),{hand="detail",tool="filbert 7",pile=unifiedSky,coverage=5,load=0.65,dips={3,0.65,0.96},length={18,38},angle=0,clip=true,fill=true,seed=161})
luminousAir=pile{{"lead white",24},{"yellow ochre",0.86},{"vermilion",0.17},{"pale smalt",0.85},medium=0.28}
-- Small tip touches, increasing steadily toward the horizon; a painter's graduated veil.
stipple(rect(0,0,1000,438),{pile=luminousAir,width=1.5,coverage=function(x,y) return 5.5*smoothstep(65,421,y) end,pressure={0.20,0.43},dips={12,0.56,0.95},feather=1,clip=true,cluster=0.0,seed=162})

--@ chunk 8
stipple(rect(580,178,420,115),{pile=luminousAir,width=6,coverage=2.3,pressure={0.65,0.9},dips={4,0.85,0.96},feather=0.5,clip=true,cluster=0,seed=171})
blend(rect(580,178,420,115),{tool="badger 32",angle=0,length={65,115},coverage=1.2,seed=172})
print(drying(720,220))

--@ chunk 9
wait(2*24*60)
-- Re-state the sky in five connected, wet horizontal passages.
sa=pile{{"lead white",7},{"smalt",7},{"cobalt blue",1.5},{"bone black",0.44},{"red earth",0.19},medium=0.32}
sb=pile{{"lead white",10},{"smalt",5},{"cobalt blue",1.0},{"bone black",0.33},{"red earth",0.18},medium=0.32}
sc=pile{{"lead white",15},{"pale smalt",4},{"cobalt blue",0.6},{"bone black",0.22},{"red earth",0.20},medium=0.32}
sd=pile{{"lead white",22},{"pale smalt",1.4},{"yellow ochre",0.5},{"vermilion",0.10},{"bone black",0.1},medium=0.32}
se=pile{{"lead white",20},{"yellow ochre",1.30},{"vermilion",0.24},{"pale smalt",0.65},medium=0.32}
local passages={{m=rect(0,0,1000,140),p=sa},{m=rect(0,97,1000,142):soften(22),p=sb},{m=rect(0,193,1000,125):soften(24),p=sc},{m=rect(0,271,1000,127):soften(25),p=sd},{m=rect(0,359,1000,90):soften(21),p=se}}
for i,a in ipairs(passages) do
 work(a.m,{hand="body",tool={kind="filbert",width=19,stiffness=0.35},pile=a.p,coverage=5,load=0.85,dips={2,0.85,0.97},pressure={0.6,0.85},length={38,72},angle=0,clip=true,seed=180+i})
 blend(a.m:grow(15),{tool="badger 45",angle=0.08,pressure={0.55,0.8},length={90,180},coverage=1.2,clip=true,seed=190+i})
end

--@ chunk 10
-- Restrained, individually drawn strata; the upper sky remains mostly open.
cloudLavender=pile{{"lead white",12},{"smalt",3.6},{"bone black",0.42},{"red earth",0.28},{"cobalt blue",0.23},medium=0.48}
local vapour=poly({{419,205},{474,201},{524,205},{609,199},{682,196},{740,201},{821,196},{887,201},{945,199},{980,204},{885,209},{777,210},{676,206},{553,211},{457,211}},true):soften(4)
work(vapour,{hand="detail",tool="filbert 4",pile=cloudLavender,coverage=1.4,load=0.26,length={28,70},angle=0,clip=true,seed=201})
local upperWisp=poly({{34,60},{97,56},{155,62},{207,59},{257,65},{281,70},{206,69},{147,73},{76,67}},true):soften(5)
work(upperWisp,{hand="detail",tool="filbert 4",pile=sb,coverage=1.4,load=0.25,length={22,58},angle=0,clip=true,seed=202})
moonPearl=pile{{"lead white",18},{"pale smalt",0.85},{"yellow ochre",0.30},medium=0.26}
work(ellipse(690,91,7.5,7.5):soften(0.6),{hand="detail",tool="round 2",pile=moonPearl,coverage=3.8,load=0.48,length={3,6},clip=true,fill=true,seed=203})
-- Water is a single horizontal quiet, to be interrupted by the shores.
lakeBase=pile{{"lead white",10},{"smalt",4},{"cobalt blue",0.7},{"bone black",0.44},{"raw umber",0.26},{"yellow ochre",0.18},medium=0.25}
work(rect(0,386,1000,218),{hand="body",tool="filbert 18",pile=lakeBase,coverage=4,load=0.8,dips={3,0.8,0.93},pressure={0.6,0.85},length={50,100},angle=0,clip=true,fill=true,seed=204})
blend(rect(0,386,1000,218),{angle=0,tool="badger 40",coverage=0.9,length={120,210},seed=205})

--@ chunk 11
wait(36*60)
farRock=pile{{"lead white",9},{"smalt",4},{"cobalt blue",0.75},{"bone black",0.43},{"raw umber",0.19},{"red earth",0.08},medium=0.28}
farMount=outline{{-15,367},{56,351},{99,330},{143,306,"c"},{178,320},{228,340},{272,326},{307,302,"c"},{355,328},{401,310},{432,291,"c"},{483,314},{525,296},{556,278,"c"},{581,288},{620,314},{666,304},{707,290,"c"},{748,308},{797,334},{841,317,"c"},{888,335},{941,326},{1018,351},{1018,425},{-15,425},char="firm",amount=0.28,edge=1.5,seed=210}:mask()
work(farMount,{hand="body",tool="filbert 8",pile=farRock,coverage=4,load=0.65,dips={3,0.65,0.93},pressure={0.48,0.7},length={22,48},angle=0.15,clip=true,fill=true,seed=211})
-- Facets on the farther slopes, kept at very low contrast.
farFace=pile{{"lead white",13},{"smalt",3},{"cobalt blue",0.45},{"bone black",0.27},{"raw umber",0.14},medium=0.34}
local facets=poly({{144,309},{168,324},{211,355},{251,383},{149,371},{172,348}})+poly({{433,294},{452,306},{482,317},{524,380},{453,375},{468,339}})+poly({{557,281},{580,294},{620,317},{658,370},{605,367},{578,334}})+poly({{707,294},{736,309},{775,334},{802,375},{737,372},{725,337}})
work(facets:soften(5)*farMount,{hand="detail",tool="filbert 5",pile=farFace,coverage=2.7,load=0.46,length={13,34},angle=0.95,clip=true,seed=212})
nearRock=pile{{"lead white",5.5},{"smalt",4.5},{"cobalt blue",0.55},{"bone black",0.65},{"raw umber",0.5},{"green earth",0.35},medium=0.3}
nearMount=outline{{-15,407},{40,387},{97,367},{151,373},{192,380},{239,366},{274,355},{318,368},{340,382},{385,367},{421,350},{464,366},{493,385},{533,369},{554,355},{590,365},{626,379},{665,359},{706,341},{758,350},{798,361},{844,354},{886,349},{953,367},{1014,389},{1014,442},{-15,442},char="soft",amount=0.4,lobe=24,edge=2.5,seed=214}:mask()
work(nearMount,{hand="body",tool="filbert 8",pile=nearRock,coverage=3.5,load=0.6,length={24,55},angle=0.25,clip=true,fill=true,seed=215})
print(drying(500,360))

--@ chunk 12
mistLight=pile{{"lead white",21},{"pale smalt",1.6},{"yellow ochre",0.72},{"vermilion",0.08},{"bone black",0.08},medium=0.38}
work(rect(0,372,1000,128),{hand="body",tool={kind="filbert",width=16,stiffness=0.25},pile=mistLight,coverage=3.6,load_at=function(x,y) return 0.72*math.exp(-((y-432)/29)^2) end,dips={2,0.7,0.94},pressure={0.55,0.8},length={45,100},angle=0,clip=true,seed=221})
blend(rect(0,380,1000,115),{tool="badger 42",coverage=0.95,length={100,190},angle=0,pressure={0.4,0.65},seed=222})
-- A warm reach of water opens immediately below the gap in the mountains.
waterLight=pile{{"lead white",17},{"pale smalt",1.5},{"yellow ochre",0.75},{"bone black",0.16},medium=0.35}
local reach=poly({{280,450},{382,443},{484,447},{589,439},{700,447},{805,441},{901,450},{913,470},{810,471},{747,488},{647,490},{588,505},{499,501},{407,483},{306,484}},true):soften(16)
work(reach,{hand="body",tool="filbert 12",pile=waterLight,coverage=2.6,load=0.65,dips={3,0.65,0.95},pressure={0.55,0.8},length={55,120},angle=0,clip=true,seed=223})
blend(rect(240,435,730,96),{angle=0,tool="badger 36",coverage=0.7,seed=224})
foreEarth=pile{{"raw umber",5},{"bone black",2.4},{"smalt",2.4},{"red earth",0.8},{"green earth",1},{"lead white",0.6},medium=0.24}
shoreLine=outline{{-10,545},{54,534},{87,528},{126,537},{166,555},{207,548},{232,545},{272,554},{313,566},{355,579},{395,591},{434,588},{475,580},{509,559},{548,539},{580,521},{592,511,"c"},{629,514},{675,503},{715,491},{770,474},{814,468},{862,462},{909,466},{949,474},{986,466},{1010,458},open=true,char="broken",amount=0.5,edge=0.8,seed=225}
foreground=shoreLine:below(H+10)
work(foreground,{hand="body",tool="filbert 14",pile=foreEarth,coverage=4.3,load=0.8,dips={3,0.8,0.95},pressure={0.6,0.9},length={32,76},angle=0.24,clip=true,fill=true,seed=226})

--@ chunk 13
wait(3*24*60)
veilGrey=pile{{"lead white",13},{"smalt",3},{"cobalt blue",0.30},{"bone black",0.21},{"raw umber",0.15},{"yellow ochre",0.36},medium=0.32}
fogBand=outline{{-15,411},{82,414},{192,409},{311,420},{428,414},{539,421},{653,411},{755,410},{875,401},{1015,406},{1015,467},{895,468},{756,471},{655,468},{522,479},{392,474},{252,468},{95,473},{-15,472},char="soft",amount=0.35,lobe=44,edge=8,seed=230}:mask()
work(fogBand,{hand="body",tool="filbert 7",pile=veilGrey,coverage=5,load=0.8,dips={2,0.8,0.97},pressure={0.65,0.9},length={20,45},angle=0,clip=true,fill=true,seed=231})
lose(fogBand,{pile=veilGrey,where=function(x,y) return y<438 and 0.7 or 0.4 end,tool="filbert 8",reach={12,14},load=0.23,pressure={0.32,0.05},every=0.8,angle=1.5,seed=232})
-- A long, low far promontory. Its lower edge is already sinking into mist.
farLand=pile{{"lead white",4},{"raw umber",1.2},{"smalt",3},{"bone black",0.6},{"green earth",1.2},medium=0.3}
peninsula=poly({{615,445},{650,438},{683,434},{708,429},{742,428},{765,431},{797,431},{835,434},{870,432},{903,437},{960,438},{1008,448},{1008,464},{948,456},{892,457},{827,450},{760,452},{715,447},{664,449}},true):soften(1.5)
work(peninsula,{hand="detail",tool="filbert 4",pile=farLand,coverage=3.5,load=0.6,length={12,25},angle=0,clip=true,fill=true,seed=233})
-- Quiet, narrow mist strata at the shore, not broad cloud billows.
work(ribbon({{15,453},{197,452},{335,457},{491,455},{624,449},{755,450},{903,448},{995,452}},{7,8,6,9,6,7,7,5}):soften(3),{hand="detail",tool="filbert 3",pile=waterLight,coverage=3,load=0.58,pressure={0.6,0.85},length={30,65},angle=0,clip=true,seed=234})

--@ chunk 14
woodDark=pile{{"bone black",4.5},{"raw umber",3.2},{"smalt",1.2},{"red earth",0.5},{"green earth",0.4},medium=0.23}
oakBody=body_of{spine={{238,563},{225,526},{227,477},{215,422},{225,370},{214,323},{220,280},{239,247},{245,214},{237,183},{257,147},{275,137},{282,111}},widths={43,32,27,25,22,19,14,11,8,6,4,2,0.55},limbs={
 {{234,551},{207,559},{184,570},{161,574},widths={22,13,7,1}},
 {{240,550},{263,567},{285,574},{304,587},widths={21,12,6,1}},
 {{226,391},{181,353},{161,311},{122,293},{87,259},{44,250},{17,222},widths={16,13,10,7,4.8,2.5,0.6}},
 {{219,433},{176,419},{145,392},{115,384},{84,402},widths={10,8,5,2.8,0.6}},
 {{220,328},{269,316},{301,278},{359,268},{394,237},{438,221},{467,193},widths={14,12,10,7.8,5.5,2.8,0.55}},
 {{239,247},{289,226},{316,187},{351,175},{383,140},widths={8,6,4.2,2.5,0.6}},
 {{221,280},{187,268},{166,239},{128,218},{119,193},{86,176},widths={9,7,5,3.8,2,0.6}},
 {{245,214},{280,183},{292,150},{321,136},{334,100},widths={5.5,4,2.6,1.5,0.5}}
 },blend=0.9,char="firm"}
work(oakBody:mask(),{hand="detail",tool="round 3.5",pile=woodDark,coverage=5.5,load=0.76,dips={3,0.76,0.95},pressure={0.6,0.85},length={9,19},angle=1.35,clip=true,fill=true,seed=250})
-- Secondary limbs are drawn individually from their parents, each tapering.
limb=brush{kind="round",width=6,point=1,stiffness=0.6}
function branch(pts,pr,width)
 local b=brush{kind="round",width=width or 6,point=1,stiffness=0.55}
 b:load(woodDark,0.82)
 b:stroke(pts,{pressure={pr or 0.72,0.01},ramps={0.01,0.35},swell={1,0.75,0.45},shake=0.12})
end
branch({{176,345},{154,348},{128,332},{110,306},{86,299}},0.9,7)
branch({{159,312},{167,287},{149,264},{143,232},{159,211}},0.82,6)
branch({{125,294},{111,268},{114,242},{103,220},{77,210}},0.72,5)
branch({{86,258},{73,232},{57,217},{52,191},{30,177}},0.75,4.5)
branch({{187,268},{190,241},{178,222},{185,191},{170,173}},0.8,5)
branch({{129,219},{109,218},{87,201},{63,195},{50,177}},0.65,4)
branch({{239,185},{213,172},{205,146},{187,136},{178,112}},0.8,5)
branch({{256,149},{245,127},{255,105},{248,81},{261,60}},0.7,4)
branch({{277,138},{307,119},{319,94},{313,75},{327,58}},0.67,4)
branch({{292,150},{282,121},{292,96},{283,77},{289,60}},0.68,3.7)
branch({{321,136},{353,126},{368,105},{396,93}},0.63,3.5)
branch({{315,188},{332,156},{332,137},{343,118}},0.75,4)
branch({{353,175},{371,178},{395,160},{415,161},{435,142}},0.72,4)
branch({{299,282},{295,250},{312,235},{313,215},{304,204}},0.85,5)
branch({{344,270},{367,291},{391,288},{414,263},{447,254}},0.86,6)
branch({{394,237},{396,210},{419,197},{423,176},{437,158}},0.75,4)
branch({{438,222},{467,220},{482,204},{503,206},{516,195}},0.66,3.5)
branch({{268,316},{296,321},{311,342},{343,354},{368,346}},0.7,5)
branch({{144,393},{146,370},{129,356},{129,341},{119,334}},0.66,4)
branch({{173,421},{164,443},{139,451},{125,448},{102,462}},0.72,4.5)
branch({{226,474},{251,453},{270,450},{278,432}},0.78,5)
branch({{220,420},{245,402},{274,393},{292,371}},0.76,5)

--@ chunk 15
wait(3*24*60)
work(oakBody:mask()*rect(190,356,82,219),{hand="detail",tool="round 3",pile=woodDark,coverage=5,load=0.76,dips={2,0.76,0.96},pressure={0.65,0.85},length={10,20},clip=true,seed=260})
function twig(pts,width,pr)
 local b=brush{kind="rigger",width=width or 1.65,point=1,stiffness=0.52}
 b:load(woodDark,0.75)
 b:stroke(pts,{pressure={pr or 0.68,0.01},ramps={0.01,0.3},shake=0.09,swell={1,0.9,0.6}})
end
-- The fine crown: forks, elbows, and a few hanging shoots, all attached.
twig({{17,222},{13,208},{0,201}},2,0.72)
twig({{18,222},{33,220},{40,205}},1.5)
twig({{31,178},{17,166},{9,145}},1.8)
twig({{52,191},{62,170},{57,157},{66,142}},1.9)
twig({{58,216},{37,209},{25,194}},1.4)
twig({{44,250},{39,264},{19,274},{9,271}},1.8)
twig({{64,239},{81,223},{79,208}},1.5)
twig({{50,177},{38,165},{32,151}},1.6)
twig({{63,195},{80,184},{78,160}},1.9)
twig({{86,176},{102,170},{105,149},{99,137}},2.2)
twig({{102,170},{119,161},{129,139}},1.45)
twig({{119,193},{130,180},{126,161},{139,149}},1.8)
twig({{109,218},{111,196},{102,180}},1.3)
twig({{77,210},{67,214},{59,207}},1.3)
twig({{87,201},{93,186},{87,175}},1.2)
twig({{86,299},{65,292},{56,277},{40,277}},1.7)
twig({{109,306},{113,284},{101,271}},1.5)
twig({{128,332},{108,338},{90,332},{81,315}},1.8)
twig({{153,348},{150,369},{136,381}},1.9)
twig({{143,232},{149,211},{141,197}},1.8)
twig({{149,264},{128,256},{121,244}},1.5)
twig({{159,211},{167,192},{159,183},{162,164}},1.6)
twig({{166,239},{179,242},{199,224},{202,208}},1.8)
twig({{185,191},{200,181},{203,163}},1.5)
twig({{170,173},{163,154},{149,150},{142,137}},1.6)
twig({{178,112},{163,106},{159,91}},1.9)
twig({{187,136},{200,125},{197,106}},1.7)
twig({{205,146},{216,135},{217,116},{229,103}},1.9)
twig({{237,183},{223,194},{207,194},{201,188}},1.7)
twig({{245,127},{231,119},{230,101},{219,85}},1.6)
twig({{255,105},{270,90},{272,75}},1.65)
twig({{248,81},{235,70},{232,49},{217,42}},1.8)
twig({{261,60},{266,44},{256,29},{261,15}},1.9)
twig({{266,44},{281,38},{286,24}},1.4)
twig({{282,111},{282,100},{269,94}},1.5)
twig({{313,75},{303,64},{305,45},{294,32}},1.8)
twig({{327,58},{342,53},{345,35},{355,30}},1.8)
twig({{327,58},{322,39},{328,23}},1.4)
twig({{292,96},{307,86},{315,71}},1.5)
twig({{289,60},{284,48},{271,43}},1.6)
twig({{283,77},{268,66},{262,52}},1.4)
twig({{334,100},{349,94},{357,77},{372,70}},1.8)
twig({{343,118},{340,98},{334,82}},1.6)
twig({{368,105},{363,88},{369,71}},1.6)
twig({{396,93},{409,84},{413,68},{427,64}},1.6)
twig({{383,140},{395,127},{414,126},{424,112}},1.8)
twig({{371,178},{378,196},{394,205},{410,200}},1.8)
twig({{395,160},{389,144},{399,131}},1.4)
twig({{415,161},{429,168},{447,155},{459,151}},1.8)
twig({{435,142},{444,125},{457,122},{465,112}},1.8)
twig({{437,158},{433,142},{424,132}},1.5)
twig({{423,176},{412,162},{413,148}},1.4)
twig({{419,197},{442,187},{450,172}},1.7)
twig({{467,193},{485,180},{486,162},{499,154}},2.0)
twig({{467,220},{474,237},{494,245},{513,235}},1.7)
twig({{482,204},{479,186},{468,180}},1.4)
twig({{503,206},{524,213},{537,206}},1.6)
twig({{516,195},{521,182},{538,179}},1.5)
twig({{447,254},{468,258},{486,246}},1.8)
twig({{414,263},{414,245},{430,239}},1.5)
twig({{391,288},{405,302},{424,301},{436,310}},1.8)
twig({{367,291},{361,311},{367,330},{383,337}},1.6)
twig({{313,215},{330,208},{338,191}},1.7)
twig({{312,235},{332,239},{349,226}},1.7)
twig({{304,204},{299,191},{308,180}},1.5)
twig({{295,250},{280,240},{275,221}},1.6)
twig({{311,342},{300,354},{302,366}},1.5)
twig({{343,354},{357,369},{376,371},{383,383}},1.6)
twig({{368,346},{382,331},{402,330}},1.5)
twig({{292,371},{306,364},{313,349}},1.6)
twig({{274,393},{281,408},{301,416},{316,411}},1.4)
twig({{278,432},{295,420},{297,406}},1.6)
twig({{84,402},{63,409},{51,401}},1.6)
twig({{115,384},{95,374},{91,359},{74,349}},1.7)
twig({{129,341},{111,347},{101,342}},1.3)
twig({{125,448},{111,431},{93,428}},1.5)
twig({{102,462},{88,473},{72,470}},1.5)
twig({{164,443},{176,455},{179,475},{191,480}},1.4)
-- The newest tips and small side buds are finer than the shoots carrying them.
twig({{17,166},{21,152},{18,142}},0.95)
twig({{105,149},{117,149},{123,141}},0.9)
twig({{129,139},{131,125},{125,119}},0.9)
twig({{203,163},{215,157},{219,143}},0.9)
twig({{159,91},{148,86},{141,90}},1.0)
twig({{217,42},{210,31},{213,22}},0.95)
twig({{256,29},{247,25},{242,14}},1.0)
twig({{294,32},{292,17},{298,11}},0.95)
twig({{345,35},{337,28},{336,15}},0.9)
twig({{372,70},{386,67},{391,55}},1.0)
twig({{413,68},{404,58},{405,49}},0.9)
twig({{457,122},{457,105},{466,97}},0.95)
twig({{499,154},{513,150},{518,135}},0.95)
twig({{538,179},{544,171},{555,171}},0.9)
twig({{513,235},{521,231},{525,219}},0.9)
twig({{383,383},{398,383},{405,377}},0.9)

--@ chunk 16
barkGrey=pile{{"raw umber",3},{"bone black",1.8},{"smalt",1.2},{"lead white",1.65},{"yellow ochre",0.45},{"red earth",0.22},medium=0.3}
barkWarm=pile{{"raw umber",3},{"bone black",1.1},{"lead white",1.0},{"yellow ochre",0.65},{"red earth",0.3},medium=0.32}
local ribs=ribbon({{226,557},{214,525},{216,478},{204,422},{214,373},{207,325},{215,284}},{10,8,7,6,4,3,2})+ribbon({{222,386},{180,350},{160,308},{121,290},{87,256}},{3,4,3,2,1})+ribbon({{222,324},{267,312},{300,274},{356,264},{392,235}},{3.5,3.5,3,2,1})
work(ribs*oakBody:mask(),{hand="detail",tool="round 1.8",pile=barkGrey,coverage=2.8,load=0.44,pressure={0.5,0.72},length={4,13},angle=1.35,clip=true,seed=270})
local planes=poly({{231,542},{238,567},{217,560},{205,566},{211,548},{219,530}})+poly({{224,506},{235,517},{234,540},{226,529},{224,520}})+poly({{213,405},{223,420},{217,443},{214,432},{209,424}})+poly({{215,349},{225,365},{215,378},{213,366}})
work(planes*oakBody:mask(),{hand="detail",tool="round 2",pile=barkWarm,coverage=2.4,load=0.45,length={4,9},angle=1.4,clip=true,seed=271})
barkPen=brush{kind="rigger",width=1.25,point=1}
function fissure(pts,pr)
 barkPen:reload(woodDark,0.65)
 barkPen:stroke(pts,{pressure={pr or 0.72,0.08},ramps={0.02,0.2},shake=0.2})
end
fissure({{220,550},{217,537},{221,523},{219,510}},0.9)
fissure({{223,516},{225,502},{221,487},{223,473}},0.85)
fissure({{214,483},{211,467},{212,452}},0.7)
fissure({{214,439},{209,426},{211,413},{208,403}},0.75)
fissure({{219,421},{220,408},{217,396},{220,381}},0.9)
fissure({{212,373},{209,359},{213,348}},0.7)
fissure({{216,331},{211,317},{213,302}},0.75)
fissure({{225,556},{232,559},{237,567}},0.8)
fissure({{202,560},{193,568},{175,572}},0.7)
fissure({{180,350},{172,341},{170,329},{161,318}},0.65)
fissure({{284,300},{299,285},{307,279},{320,276}},0.7)
-- Bark knots have lips, not black polka dots.
work(ellipse(225,487,3.1,7.4),{hand="detail",pile=woodDark,tool="round 1.3",coverage=3,load=0.5,clip=true,seed=272})
local lip=brush{kind="round",width=1.3,point=1}
lip:load(barkGrey,0.52); lip:stroke({{223,495},{220,490},{220,484},{224,479}},{pressure={0.65,0.05}})
lip:reload(barkGrey,0.5); lip:stroke({{217,430},{219,425},{220,419},{227,416}},{pressure={0.5,0.03}})
-- Ground passages around the stone path.
groundOlive=pile{{"raw umber",3},{"green earth",4},{"yellow ochre",0.9},{"bone black",1},{"smalt",0.6},{"lead white",0.7},medium=0.24}
groundRusset=pile{{"raw umber",3},{"yellow ochre",1.6},{"red earth",0.65},{"bone black",0.9},{"lead white",0.65},medium=0.26}
local mossBeds=poly({{4,551},{74,546},{138,566},{191,584},{170,633},{53,641},{0,616}},true)+poly({{279,581},{335,582},{376,606},{383,644},{271,667},{232,638}},true)+poly({{663,523},{735,500},{812,500},{843,535},{821,588},{732,582}},true)+poly({{886,487},{1000,479},{1000,604},{934,587},{873,550}},true)
work(mossBeds*foreground,{hand="scumble",tool="filbert 7",pile=groundOlive,coverage=1.6,load=0.38,length={13,26},pressure={0.38,0.6},angle=0.3,clip=true,seed=273})
work(poly({{507,578},{564,547},{655,556},{730,595},{850,625},{983,642},{1000,667},{490,667}},true)*foreground,{hand="body",tool="filbert 8",pile=groundRusset,coverage=2.2,load=0.47,pressure={0.5,0.7},length={23,47},angle=0.28,clip=true,seed=274})
pathEarth=pile{{"raw umber",2.4},{"yellow ochre",1.2},{"smalt",1.2},{"lead white",2.1},{"bone black",0.55},medium=0.23}
path=ribbon({{417,675},{451,629},{491,589},{545,548},{593,515}},{129,85,46,21,8})*foreground
work(path,{hand="body",tool="filbert 8",pile=pathEarth,coverage=4.6,load=0.72,pressure={0.55,0.8},length={21,48},angle=-0.8,clip=true,fill=true,seed=275})

--@ chunk 17
stoneDark=pile{{"raw umber",2.8},{"bone black",1.6},{"smalt",1.4},{"lead white",1.6},{"green earth",0.5},medium=0.23}
stoneMid=pile{{"raw umber",1.9},{"bone black",0.9},{"smalt",1.6},{"lead white",3},{"yellow ochre",0.28},medium=0.24}
stoneLight=pile{{"lead white",5},{"smalt",1.3},{"raw umber",1.1},{"bone black",0.53},{"yellow ochre",0.3},medium=0.24}
rockLeft=poly({{0,574},{55,561},{95,569},{147,594},{159,616},{129,639},{54,658},{0,651}}):roughen(1.1,8,281)
rockRoot=poly({{159,568},{195,547},{228,551},{263,573},{279,596},{253,618},{187,610},{157,591}}):roughen(0.8,7,282)
rockSmall=poly({{275,590},{301,578},{342,591},{370,611},{360,640},{318,649},{282,628}}):roughen(0.8,6,283)
rockRight=poly({{705,571},{756,541},{812,551},{856,582},{843,619},{798,645},{741,626},{714,609}}):roughen(1.1,9,284)
rockRidge=poly({{786,489},{840,468},{876,459},{916,465},{955,484},{931,511},{843,524},{799,511}}):roughen(0.9,8,285)
rockNear=poly({{624,517},{661,501},{691,506},{704,520},{678,546},{638,547}}):roughen(0.7,6,286)
rocks=(rockLeft+rockRoot+rockSmall+rockRight+rockRidge+rockNear)*foreground-oakBody:mask()
work(rocks,{hand="body",tool="filbert 7",pile=stoneDark,coverage=4.2,load=0.78,pressure={0.55,0.85},length={18,39},angle=0.8,clip=true,fill=true,seed=287})
rockTops=(poly({{0,574},{55,561},{95,569},{147,594},{124,611},{63,608},{22,599},{0,605}})+poly({{164,567},{195,547},{228,551},{263,573},{267,583},{226,591},{191,582}})+poly({{276,590},{301,578},{342,591},{361,605},{323,612},{288,606}})+poly({{705,571},{756,541},{812,551},{849,578},{804,599},{758,583}})+poly({{786,489},{840,468},{876,459},{916,465},{955,484},{899,487},{849,502},{812,501}})+poly({{624,517},{661,501},{691,506},{696,516},{664,526},{639,523}}))*rocks
work(rockTops,{hand="body",tool="filbert 5",pile=stoneMid,coverage=4,load=0.7,pressure={0.55,0.85},length={16,34},angle=-0.2,clip=true,fill=true,seed=288})
local litFaces=(poly({{63,608},{124,611},{147,594},{157,616},{125,635},{74,643}})+poly({{226,591},{267,583},{277,596},{251,612},{222,607}})+poly({{323,612},{361,605},{361,630},{324,643}})+poly({{804,599},{849,578},{839,617},{799,640},{787,621}})+poly({{849,502},{899,487},{946,486},{927,507},{866,520}})+poly({{664,526},{696,516},{678,541},{647,542}}))*rocks
work(litFaces,{hand="detail",tool="filbert 4",pile=stoneLight,coverage=2.8,load=0.52,length={11,22},angle=1.1,clip=true,seed=289})
-- Five independent, perspective-diminished slabs in the old shore path.
slabs=poly({{581,523},{593,512},{613,516},{612,523},{591,529}})+poly({{551,546},{563,536},{589,527},{594,536},{567,548},{554,555}})+poly({{516,573},{539,555},{567,550},{555,568},{534,579}})+poly({{468,610},{490,589},{529,576},{537,587},{510,604},{491,619}})+poly({{405,662},{444,624},{483,613},{491,631},{461,652},{450,667},{402,667}})
slabs=slabs*foreground
work(slabs,{hand="detail",tool="filbert 4",pile=stoneLight,coverage=4.3,load=0.66,pressure={0.6,0.84},length={11,25},angle=-0.25,clip=true,fill=true,seed=290})

--@ chunk 18
wait(4*24*60)
-- The traveller, seen only from behind: small enough to belong to the place.
coatDark=pile{{"bone black",3},{"raw umber",2},{"smalt",2},{"red earth",0.4},{"lead white",0.18},medium=0.23}
coatLit=pile{{"raw umber",2},{"bone black",1},{"smalt",1.4},{"lead white",0.7},{"red earth",0.25},medium=0.27}
traveller=outline{{589,484},{594,483},{597,488},{597,498},{601,508,"c"},{594,510},{589,509},{586,507,"c"},{588,496},char="firm",amount=0.2,edge=0.2,seed=300}:mask()
work(traveller,{hand="detail",tool="round 1.4",pile=coatDark,coverage=5,load=0.72,pressure={0.6,0.8},length={4,9},clip=true,fill=true,seed=301})
work(ellipse(591.8,480.3,2.3,3.2),{hand="detail",tool="round 1",pile=coatDark,coverage=4,load=0.6,clip=true,fill=true,seed=302})
local figurePen=brush{kind="round",width=1.3,point=1}
figurePen:load(coatDark,0.7);figurePen:stroke({{587.9,481},{591,478.8},{596,479.7}},{pressure={0.7,0.3},ramps={0.02,0.15}})
figurePen:reload(coatDark,0.7);figurePen:stroke({{590,506},{589.7,512.5},{592,514}},{pressure={0.8,0.4}})
figurePen:reload(coatDark,0.7);figurePen:stroke({{596,507},{596.8,512},{599,514}},{pressure={0.8,0.4}})
figurePen:reload(coatLit,0.5);figurePen:stroke({{594,486},{595.3,493},{594.2,500},{597,506}},{pressure={0.5,0.04},ramps={0.08,0.3}})
figurePen:reload(coatLit,0.45);figurePen:stroke({{590.5,489},{590,495},{591.6,505}},{pressure={0.38,0.01}})
local staff=brush{kind="rigger",width=0.8,point=1}
staff:load(woodDark,0.6);staff:stroke({{601,492},{603.5,504},{606,518}},{pressure={0.8,0.3},ramps={0.02,0.05}})
figurePen:reload(coatDark,0.6);figurePen:stroke({{596,490},{598,496},{601.5,496.8}},{pressure={0.7,0.1}})
-- The unroofed chapel on the opposite promontory, with a broken bell tower.
ruinStone=pile{{"lead white",4.5},{"raw umber",1.0},{"smalt",2},{"bone black",0.6},{"yellow ochre",0.18},medium=0.3}
ruinShade=pile{{"lead white",2.6},{"raw umber",1.2},{"smalt",2},{"bone black",0.85},medium=0.3}
chapel=poly({{707,429},{708,414},{712,409},{718,415},{721,411},{724,401},{728,405},{730,414},{734,413},{735,395},{738,394},{738,389},{741,392},{743,390},{745,395},{747,396},{746,429}})
work(chapel,{hand="detail",tool="round 1.5",pile=ruinStone,coverage=4.2,load=0.63,pressure={0.6,0.8},length={4,9},clip=true,fill=true,seed=303})
work(poly({{741,395},{747,396},{746,429},{741,429}})+poly({{725,405},{729,411},{729,428},{725,428}}),{hand="detail",tool="round 1.2",pile=ruinShade,coverage=3.8,load=0.6,length={4,8},clip=true,seed=304})
work(poly({{738.4,409},{740.1,404},{742.2,409},{742.2,419},{738.2,419}})+poly({{711,428},{711,421},{713.2,418},{715.2,421},{715.2,428}})+poly({{720,428},{720,419},{722,416.5},{724,419},{724,428}}),{hand="detail",tool="round 0.8",pile=ruinShade,coverage=4,load=0.62,length={2,4},clip=true,fill=true,seed=305})
local masonry=brush{kind="round",width=0.65,point=1}
masonry:load(stoneLight,0.44);masonry:stroke({{736.2,396.4},{735.8,412},{735.8,426}},{pressure={0.6,0.16}})
masonry:reload(stoneLight,0.45);masonry:stroke({{708.5,415},{712,411.5},{716.5,415.6}},{pressure={0.55,0.05}})
masonry:reload(stoneLight,0.42);masonry:stroke({{722.5,408},{724.3,403},{727.8,406}},{pressure={0.5,0.02}})
masonry:reload(ruinShade,0.48);masonry:stroke({{736,421.8},{740.5,422.3}},{pressure={0.5,0.15}})
masonry:reload(ruinShade,0.48);masonry:stroke({{736,402},{740,402.2}},{pressure={0.5,0.12}})
-- Thin air across the foundations only.
work(ribbon({{699,429},{716,430},{737,431},{756,430}},{4,5,5,3}):soften(1),{hand="detail",tool="filbert 2",pile=veilGrey,coverage=1.9,load=0.36,length={7,16},angle=0,clip=true,seed=306})

--@ chunk 19
mistEven=pile{{"lead white",17},{"pale smalt",2.4},{"bone black",0.19},{"raw umber",0.13},{"yellow ochre",0.4},medium=0.36}
cleanFog=outline{{-12,391},{100,392},{203,387},{300,399},{431,395},{552,401},{651,393},{780,391},{894,384},{1012,389},{1012,486},{-12,486},char="soft",amount=0.22,lobe=54,edge=18,seed=311}:mask()-oakBody:mask()-foreground-chapel
work(cleanFog,{hand="body",tool="filbert 7",pile=mistEven,coverage=5.6,load=0.82,dips={2,0.82,0.98},pressure={0.65,0.88},length={19,38},angle=0,clip=true,fill=true,seed=312})
-- Keep the opposite bank as a thin physical interval below the ruin.
work(peninsula*rect(620,436,390,26)-chapel,{hand="detail",tool="filbert 3",pile=farLand,coverage=2.7,load=0.55,length={12,27},angle=0,clip=true,seed=313})
-- Some near twigs regain their dark continuity; others are deliberately lost in the light.
twig({{84,402},{63,409},{51,401}},1.6)
twig({{125,448},{111,431},{93,428}},1.5)
twig({{102,462},{88,473},{72,470}},1.5)
twig({{164,443},{176,455},{179,475},{191,480}},1.4)
twig({{274,393},{281,408},{301,416},{316,411}},1.4)
twig({{278,432},{295,420},{297,406}},1.6)
-- Spruces farther along the bank, separate drawings rather than repeated emblems.
sprucePaint=pile{{"raw umber",2.2},{"bone black",1.5},{"smalt",2.7},{"green earth",3},{"lead white",1.35},{"yellow ochre",0.25},medium=0.24}
spruce1=poly({{946,358},{944,373},{939,381},{943,379},{934,390},{940,390},{929,400},{936,401},{924,413},{933,412},{919,425},{932,427},{916,439},{933,441},{920,451},{939,457},{941,466},{946,467},{949,457},{967,461},{959,453},{975,453},{961,440},{974,438},{961,425},{971,423},{957,412},{968,411},{954,399},{962,398},{951,387},{956,387},{950,380},{948,369}}):roughen(0.5,3,314)
spruce2=poly({{991,380},{987,395},{984,404},{988,403},{979,415},{986,413},{975,425},{981,426},{972,439},{983,437},{970,453},{982,452},{974,463},{989,465},{990,474},{993,474},{995,466},{1004,468},{1007,458},{1001,447},{1011,448},{1000,434},{1008,435},{996,420},{1006,422},{996,408},{1001,408},{994,395}}):roughen(0.55,3.5,315)
work((spruce1+spruce2)-rockRidge,{hand="detail",tool="round 2",pile=sprucePaint,coverage=4.6,load=0.76,pressure={0.6,0.85},length={6,13},angle=0.1,clip=true,fill=true,seed=316})

--@ chunk 20
wait(5*24*60)
work(foreground:rim(9,1)*rect(0,440,1000,170)-rocks-path-oakBody:mask(),{hand="detail",tool="round 3",pile=foreEarth,coverage=5,load=0.8,pressure={0.65,0.9},length={8,17},clip=true,fill=true,seed=320})
work((spruce1+spruce2)-rockRidge,{hand="detail",tool="round 2",pile=sprucePaint,coverage=5.5,load=0.82,pressure={0.7,0.9},length={5,11},clip=true,fill=true,seed=321})
soilGrey=pile{{"raw umber",3},{"bone black",1.05},{"smalt",1.0},{"green earth",2.2},{"lead white",1.1},{"yellow ochre",0.65},medium=0.24}
soilDeep=pile{{"raw umber",3.2},{"bone black",1.6},{"green earth",1.2},{"smalt",0.9},{"lead white",0.25},medium=0.21}
soil=foreground-rocks-path-oakBody:mask()
stipple(soil,{pile=soilGrey,width=2.7,coverage=2.4,pressure={0.6,0.9},dips={7,0.74,0.93},cluster={0.35,17},clip=true,seed=322})
stipple(soil,{pile=soilDeep,width=4.2,coverage=1.05,pressure={0.55,0.85},dips={6,0.73,0.94},cluster={0.5,20},clip=true,seed=323})
pathShade=pile{{"raw umber",2.8},{"bone black",0.8},{"smalt",0.8},{"lead white",1.2},{"yellow ochre",0.4},medium=0.22}
work(path-slabs,{hand="hatch",tool="round 2.4",pile=pathShade,coverage=2.5,load=0.5,pressure={0.45,0.7},length={5,12},angle=0.16,clip=true,seed=324})
work(traveller,{hand="detail",tool="round 1.4",pile=coatDark,coverage=5,load=0.73,pressure={0.65,0.9},length={4,8},clip=true,fill=true,seed=325})
work(ellipse(591.8,480.3,2.3,3.2),{hand="detail",tool="round 1",pile=coatDark,coverage=4.5,load=0.64,clip=true,fill=true,seed=326})
local fp=brush{kind="round",width=1.3,point=1}
fp:load(coatDark,0.7);fp:stroke({{587.9,481},{591,478.8},{596,479.7}},{pressure={0.7,0.3},ramps={0.02,0.15}})
fp:reload(coatLit,0.6);fp:stroke({{594,486},{595.3,493},{594.2,500},{597,506}},{pressure={0.63,0.04}})
fp:reload(coatDark,0.65);fp:stroke({{596,490},{598,496},{601.5,496.8}},{pressure={0.75,0.1}})
local st=brush{kind="rigger",width=0.8,point=1}
st:load(woodDark,0.65);st:stroke({{601,492},{603.5,504},{606,518}},{pressure={0.8,0.3},ramps={0.02,0.05}})

--@ chunk 21
rockDust=pile{{"raw umber",2.1},{"smalt",1.5},{"lead white",2.7},{"bone black",0.8},{"yellow ochre",0.25},{"green earth",0.3},medium=0.23}
rockSpeck=pile{{"raw umber",2.4},{"bone black",1.1},{"smalt",1.2},{"lead white",0.9},medium=0.2}
work(rocks,{hand="scumble",tool="filbert 3",pile=rockDust,coverage=1.5,load=0.38,pressure={0.48,0.68},length={5,12},angle=0.13,clip=true,seed=331})
stipple(rocks,{pile=rockSpeck,width=1.4,coverage=0.85,pressure={0.55,0.85},dips={9,0.63,0.94},cluster={0.4,10},clip=true,seed=332})
work(rockTops,{hand="hatch",tool="round 1.4",pile=stoneLight,coverage=1.9,load=0.39,pressure={0.5,0.7},length={3,8},angle=-0.12,clip=true,seed=333})
crackPaint=pile{{"raw umber",3.2},{"bone black",2.8},{"smalt",1},{"lead white",0.3},medium=0.23}
function crack(pts,w,pr)
 local b=brush{kind="rigger",width=w or 1.25,point=1}
 b:load(crackPaint,0.68)
 b:stroke(pts,{pressure={pr or 0.76,0.07},ramps={0.02,0.2},shake=0.12})
end
crack({{42,574},{51,584},{48,592},{61,605},{65,623},{61,642}},1.65)
crack({{49,591},{79,584},{97,594},{124,607},{136,625}},1.3)
crack({{81,611},{94,620},{91,635}},0.95)
crack({{4,620},{25,624},{35,616},{57,620}},1.2)
crack({{116,573},{111,588},{98,593}},0.9)
crack({{174,569},{188,565},{202,580},{224,586}},1.45)
crack({{245,576},{237,587},{242,602},{234,613}},1.2)
crack({{294,582},{307,594},{324,598},{323,616},{331,637}},1.5)
crack({{324,598},{339,590},{350,598}},0.8)
crack({{729,563},{749,562},{768,574},{775,586}},1.2)
crack({{774,553},{798,559},{807,580},{800,596},{811,614},{804,629}},1.8)
crack({{807,580},{824,579},{842,586}},1.2)
crack({{752,586},{758,606},{752,617}},1.05)
crack({{847,472},{869,479},{876,489},{869,504},{865,515}},1.5)
crack({{876,489},{893,482},{915,486},{924,498}},1.15)
crack({{915,468},{912,482}},0.9)
crack({{646,511},{665,515},{672,527},{669,538}},1.15)
crack({{421,649},{437,647},{453,639},{464,643},{480,632}},1.4)
crack({{474,606},{490,606},{502,593},{520,590}},1.05)
crack({{530,568},{542,569},{548,558}},0.9)
crack({{560,544},{568,540},{579,541}},0.7)
-- Ice on the upper ledges is sparse, broken, and much smaller than the rock planes.
frost=pile{{"lead white",9},{"pale smalt",1.2},{"bone black",0.13},{"yellow ochre",0.2},medium=0.24}
local edges=ribbon({{1,574},{54,562},{92,570},{117,582}},{2.2,2.6,1.8,1})+ribbon({{160,568},{194,549},{217,552}},{1.5,2.6,1.2})+ribbon({{277,590},{300,580},{331,588}},{1.5,2,1})+ribbon({{708,572},{756,542},{807,552},{835,569}},{1.4,2.2,1.8,0.8})+ribbon({{790,489},{840,469},{875,461},{911,466}},{1,1.8,2.1,0.8})
work(edges*rocks,{hand="detail",tool="round 1",pile=frost,coverage=1.6,load=0.2,pressure={0.45,0.68},length={3,7},clip=true,seed=334})
stipple(rockTops+slabs,{pile=frost,width=1.1,coverage=0.37,pressure={0.5,0.8},dips={9,0.54,0.94},cluster={0.5,9},clip=true,seed=335})

--@ chunk 22
-- Contact shade seats the stones in earth; their edges cease to float.
contacts=(ribbon({{0,650},{52,659},{124,640},{155,619}},{5,5,4,2})+ribbon({{158,594},{190,612},{251,619},{278,598}},{2,5,4,2})+ribbon({{282,627},{318,650},{359,641}},{3,5,3})+ribbon({{714,610},{741,628},{797,646},{842,620}},{3,4,5,2})+ribbon({{800,512},{842,525},{914,516},{932,510}},{2,4,3,1}))*(foreground-rocks-oakBody:mask())
work(contacts,{hand="detail",tool="round 2.3",pile=soilDeep,coverage=3.6,load=0.63,pressure={0.55,0.8},length={5,12},angle=0,clip=true,seed=341})
grassDull=pile{{"yellow ochre",1.7},{"raw umber",3},{"bone black",0.9},{"smalt",0.8},{"lead white",1.5},{"red earth",0.22},medium=0.24}
grassLight=pile{{"lead white",3},{"yellow ochre",1.8},{"raw umber",2},{"bone black",0.6},{"smalt",0.8},medium=0.24}
local tufts={
 {20,610,13,14,7},{55,641,16,21,10},{96,626,12,10,7},{135,654,18,20,9},{176,626,19,17,9},{194,615,13,12,6},{263,637,20,20,9},{293,665,24,25,10},{354,658,17,17,8},
 {388,649,13,13,6},{425,617,11,10,6},{469,608,10,10,4},{532,636,24,24,10},{552,611,18,17,7},{577,575,12,13,6},{614,557,10,10,5},{654,580,22,19,8},{690,608,18,21,9},{737,660,25,28,11},{868,640,26,24,10},{887,610,15,17,8},{941,658,27,27,11},{984,607,16,19,9},
 {729,494,8,12,5},{760,486,8,12,5},{812,482,9,12,5},{849,480,8,12,5},{902,481,11,15,6},{977,481,13,16,6},{323,593,9,9,4},{109,572,8,9,4}}
local grass=brush{kind="rigger",width=1.05,point=1,stiffness=0.56}
local foot=brush{kind="round",width=5,point=0.65}
for k,t in ipairs(tufts) do
 foot:reload(soilDeep,0.65);foot:touch(t[1],t[2],{pressure=0.7,drag={1.8,0}})
 for j=1,t[4] do
  local bx=rand(t[1]-t[5],t[1]+t[5]);local by=rand(t[2]-1.4,t[2]+1.2)
  local ey=rand(t[2]-t[3],t[2]-t[3]*0.25)
  local ex=rand(t[1]-t[5]-t[3]*0.30,t[1]+t[5]+t[3]*0.25)
  local mx=rand(math.min(bx,ex),math.max(bx,ex))
  local my=rand(ey+(by-ey)*0.35,ey+(by-ey)*0.65)
  if j%3==0 then grass:reload(grassLight,0.65) else grass:reload(grassDull,0.64) end
  grass:stroke({{bx,by},{mx,my},{ex,ey}},{pressure={rand(0.46,0.78),0.005},ramps={0.02,0.3},shake=0.08})
 end
end
-- Last year's wiry heather is not the same form as the grass.
local heath=brush{kind="rigger",width=1.05,point=1}
heath:load(grassDull,0.7);heath:stroke({{875,611},{882,599},{877,587},{884,572}},{pressure={0.8,0.02}})
heath:reload(grassDull,0.7);heath:stroke({{881,599},{867,590},{861,579}},{pressure={0.6,0.01}})
heath:reload(grassDull,0.7);heath:stroke({{877,587},{891,585},{899,573}},{pressure={0.55,0.01}})
heath:reload(grassDull,0.7);heath:stroke({{184,631},{182,611},{169,601},{165,590}},{pressure={0.7,0.01}})
heath:reload(grassDull,0.7);heath:stroke({{182,611},{192,598},{188,586}},{pressure={0.58,0.01}})
-- Needle-scale, not brush-sized, variation inside the two spruce crowns.
needleLight=pile{{"smalt",2.6},{"green earth",3},{"raw umber",1.5},{"bone black",0.8},{"lead white",2},{"yellow ochre",0.2},medium=0.23}
stipple((spruce1+spruce2)-rockRidge,{pile=needleLight,width=1.3,coverage=1.5,pressure={0.48,0.8},dips={8,0.6,0.93},cluster={0.45,6},clip=true,seed=342})

--@ chunk 23
wait(4*24*60)
-- In the light coming from beyond the lake the stone fronts must remain in shadow.
fronts=rocks-rockTops
work(fronts,{hand="detail",tool="filbert 4",pile=stoneDark,coverage=4.6,load=0.7,dips={3,0.7,0.94},pressure={0.58,0.82},length={9,22},angle=1.2,clip=true,fill=true,seed=351})
wait(2*24*60)
turnFaces=(poly({{63,608},{124,611},{147,594},{157,616},{125,635},{74,643}})+poly({{226,591},{267,583},{277,596},{251,612},{222,607}})+poly({{323,612},{361,605},{361,630},{324,643}})+poly({{804,599},{849,578},{839,617},{799,640},{787,621}})+poly({{849,502},{899,487},{946,486},{927,507},{866,520}})+poly({{664,526},{696,516},{678,541},{647,542}}))*fronts
work(turnFaces,{hand="detail",tool="filbert 3",pile=stoneMid,coverage=3.1,load=0.52,pressure={0.53,0.75},length={7,16},angle=1.1,clip=true,seed=352})
work(fronts,{hand="hatch",tool="round 1.4",pile=rockDust,coverage=0.9,load=0.26,pressure={0.4,0.65},length={3,7},angle=1.25,clip=true,seed=353})
crack({{65,609},{65,623},{61,642}},1.35)
crack({{91,615},{94,620},{91,635}},0.95)
crack({{243,590},{242,602},{234,613}},1.2)
crack({{323,612},{326,622},{331,637}},1.25)
crack({{800,596},{811,614},{804,629}},1.55)
crack({{752,586},{758,606},{752,617}},1.05)
crack({{876,489},{869,504},{865,515}},1.5)
crack({{672,527},{669,538}},0.9)
-- A cool, oil-rich veil in the high fog, brushed across dry paint.
coolVapour=pile{{"lead white",6},{"pale smalt",5},{"cobalt blue",0.55},{"bone black",0.25},{"raw umber",0.14},medium=0.55}
local upperFog=rect(0,373,1000,86)*cleanFog-chapel
work(upperFog,{hand="glaze",tool={kind="filbert",width=18,stiffness=0.2},pile=coolVapour,coverage=3.1,load_at=function(x,y) return 0.69*math.exp(-((y-413)/32)^2) end,pressure={0.55,0.8},dips={2,0.7,0.95},length={65,135},angle=0,clip=true,seed=354})
blend(upperFog,{tool="badger 35",coverage=0.9,length={90,180},angle=0,pressure={0.4,0.7},seed=355})

--@ chunk 24
-- Drag only the open veil across its lower boundary; protect near forms.
local protect=oakBody:mask()+chapel+traveller+ellipse(591.8,480.3,3.1,4)+(spruce1+spruce2)+foreground
blend(rect(0,427,1000,80)-protect,{tool="badger 44",angle=math.pi/2,pressure={0.55,0.8},length={40,74},coverage=2.2,seed=361})
-- Open slots between the whorls make these crowns branching systems, not triangles.
local gaps1=poly({{937,389},{945,389},{944,395},{932,399}})+poly({{936,402},{946,403},{945,409},{926,414}})+poly({{933,415},{944,417},{942,424},{920,429}})+poly({{935,428},{944,432},{941,438},{919,442}})+poly({{931,443},{944,447},{942,453},{922,455}})+poly({{949,392},{959,398},{952,402},{948,399}})+poly({{950,405},{966,413},{956,419},{948,415}})+poly({{949,426},{968,435},{956,441},{947,438}})+poly({{947,444},{971,453},{955,457},{947,454}})
local gaps2=poly({{984,410},{991,409},{990,417},{979,423}})+poly({{981,430},{991,428},{990,437},{975,443}})+poly({{984,450},{991,449},{990,457},{975,462}})+poly({{994,412},{1003,420},{995,424},{992,420}})+poly({{994,433},{1006,441},{996,447},{992,442}})
spruceOpen=(spruce1-gaps1)+(spruce2-gaps2)
work(spruceOpen-rockRidge,{hand="detail",tool="round 1.4",pile=sprucePaint,coverage=5.8,load=0.76,pressure={0.65,0.85},length={4,8},clip=true,fill=true,seed=362})
local pinePen=brush{kind="rigger",width=1.6,point=1}
pinePen:load(woodDark,0.75);pinePen:stroke({{944,467},{943,442},{946,416},{946,383},{946,358}},{pressure={0.85,0.01},ramps={0.01,0.35}})
pinePen:reload(woodDark,0.7);pinePen:stroke({{991,474},{991,445},{992,416},{991,380}},{pressure={0.7,0.01},ramps={0.01,0.35}})
-- A little broken light follows particular branch platforms, not the entire outline.
local platforms=ribbon({{943,388},{939,390},{934,392}},1.5)+ribbon({{946,409},{953,411},{964,415}},1.5)+ribbon({{943,427},{933,431},{925,434}},1.8)+ribbon({{946,441},{953,445},{967,451}},1.8)+ribbon({{991,425},{984,428},{976,433}},1.3)+ribbon({{993,451},{1000,455}},1.2)
work(platforms*spruceOpen,{hand="detail",tool="round 0.8",pile=needleLight,coverage=2.7,load=0.38,pressure={0.5,0.7},length={2,5},angle=0.8,clip=true,seed=363})
stipple(spruceOpen-rockRidge,{pile=needleLight,width=0.85,coverage=0.8,pressure={0.45,0.7},dips={9,0.55,0.93},cluster={0.4,6},clip=true,seed=364})
-- A lean umber-smalt film quiets the granular stone shadows.
stoneVeil=pile{{"raw umber",3},{"smalt",2},{"bone black",2},{"lead white",0.5},medium=0.64}
work(fronts,{hand="glaze",tool="filbert 6",pile=stoneVeil,coverage=2.5,load=0.42,pressure={0.45,0.7},length={18,40},angle=1.25,clip=true,seed=365})

--@ chunk 25
wait(30*24*60)
print(drying(785,618),drying(800,411))
quietShadow=pile{{"raw umber",2.5},{"smalt",1.8},{"bone black",1.2},{"lead white",2.2},{"green earth",0.4},medium=0.24}
quietTurn=pile{{"raw umber",2.1},{"smalt",1.7},{"bone black",0.85},{"lead white",3.1},{"yellow ochre",0.15},medium=0.24}
work(fronts,{hand="detail",tool="filbert 3.2",pile=quietShadow,coverage=6.2,load=0.95,dips={1,0.95,0.98},pressure={0.72,0.92},length={8,18},angle=1.18,clip=true,fill=true,seed=371})
work(turnFaces,{hand="detail",tool="filbert 2.8",pile=quietTurn,coverage=5,load=0.9,dips={1,0.9,0.98},pressure={0.7,0.9},length={7,14},angle=1.05,clip=true,fill=true,seed=372})
-- Detached flakes of the same rock, with their own small, irregular shapes.
shale=poly({{216,649},{230,642},{247,646},{253,655},{241,663},{224,659}})+poly({{493,636},{503,627},{519,630},{528,640},{512,648},{495,643}})+poly({{921,574},{932,564},{943,568},{947,575},{934,581}})+poly({{608,615},{615,610},{626,613},{626,621},{615,624}})+poly({{82,659},{90,653},{104,657},{101,665},{89,667}})+poly({{668,654},{676,649},{688,654},{688,661},{673,665}})+poly({{847,658},{857,649},{866,653},{864,662},{851,666}})
work(shale*foreground,{hand="detail",tool="round 2",pile=stoneDark,coverage=5,load=0.76,pressure={0.65,0.86},length={4,9},clip=true,fill=true,seed=373})
local flakes=poly({{216,649},{230,642},{247,646},{244,651},{227,653}})+poly({{493,636},{503,627},{519,630},{516,635},{502,638}})+poly({{921,574},{932,564},{943,568},{935,573}})+poly({{608,615},{615,610},{626,613},{622,617}})+poly({{82,659},{90,653},{104,657},{94,660}})+poly({{668,654},{676,649},{688,654},{678,657}})+poly({{847,658},{857,649},{866,653},{857,657}})
work(flakes,{hand="detail",tool="round 1.5",pile=stoneMid,coverage=3.5,load=0.55,pressure={0.55,0.76},length={3,6},clip=true,seed=374})
-- Smaller grey pebbles are laid as distinct tip marks, not a dotted outline.
local pebble=brush{kind="round",width=2.7,point=0.6}
for i=1,72 do
 local x=rand(6,994);local y=rand(542,666)
 if soil:at(x,y)>0.7 then
  pebble:reload(quietTurn,0.55)
  pebble:touch(x,y,{pressure=rand(0.35,0.68),drag={rand(0.8,2.6),rand(-0.3,0.3)},twist=rand(-0.4,0.4)})
 end
end

--@ chunk 26
waterGrey=pile{{"lead white",9},{"pale smalt",4},{"cobalt blue",0.45},{"bone black",0.18},{"raw umber",0.13},medium=0.35}
function waterLine(pts,w,pr,p)
 local b=brush{kind="round",width=w,point=0.8,stiffness=0.28}
 b:load(p or waterGrey,0.44)
 b:stroke(pts,{pressure={pr,0.01},ramps={0.08,0.3},shake=0.07})
end
waterLine({{14,469},{81,467.8},{152,468.2}},2.6,0.45)
waterLine({{261,476},{349,476.5},{423,474.9}},2.3,0.42)
waterLine({{397,482},{505,481.4},{573,482.1}},2.0,0.39)
waterLine({{660,465.4},{715,464.1},{755,464.9},{803,463.8}},2.5,0.43)
waterLine({{464,459.5},{501,460.2},{562,458.8}},2.2,0.4)
waterLine({{358,454},{411,453.6},{444,454.3}},1.8,0.36)
waterLine({{759,473},{791,472.6},{841,473.4}},1.5,0.36)
waterLine({{838,456},{882,455.3},{914,456.1}},1.5,0.37)
waterLine({{64,505},{111,504.5},{156,506}},2.0,0.42,waterLight)
waterLine({{276,517},{314,515.8},{352,516.5}},1.8,0.38,waterLight)
waterLine({{381,539},{424,538.4},{458,539.5}},1.6,0.35,waterLight)
-- The suggestion of the tower in disturbed water is a new, broken drawing.
reflectionTone=pile{{"lead white",7},{"smalt",3},{"bone black",0.38},{"raw umber",0.3},medium=0.42}
work(ribbon({{739,447},{736.8,449.1},{740.2,450.6},{736.4,453.2},{738.5,455.1}},{3.3,2.5,4.2,1.9,0.5}),{hand="detail",tool="round 1.2",pile=reflectionTone,coverage=1.9,load=0.27,pressure={0.4,0.6},length={3,6},angle=0,clip=true,seed=381})
waterLine({{723,450.5},{731,450.2},{742,450.7},{753,450.1}},1.4,0.3,waterLight)
-- The near wood is not dissolved by the atmosphere.
branch({{220,420},{245,402},{274,393},{292,371}},0.76,5)
branch({{226,474},{251,453},{270,450},{278,432}},0.78,5)
branch({{173,421},{164,443},{139,451},{125,448},{102,462}},0.72,4.5)
twig({{164,443},{176,455},{179,475},{191,480}},1.4)
twig({{125,448},{111,431},{93,428}},1.5)
twig({{274,393},{281,408},{301,416},{316,411}},1.4)
twig({{278,432},{295,420},{297,406}},1.6)
twig({{292,371},{306,364},{313,349}},1.6)
barkDust=pile{{"raw umber",3.2},{"bone black",2.5},{"smalt",1.5},{"lead white",0.95},{"red earth",0.2},medium=0.25}
stipple(oakBody:mask()*rect(191,307,78,261),{pile=barkDust,width=1.1,coverage=1.2,pressure={0.52,0.84},dips={6,0.7,0.94},cluster={0.35,8},clip=true,seed=382})
-- Select fissures run through ridges and turn around old branch collars.
fissure({{232,553},{226,540},{230,527},{228,515}},0.9)
fissure({{218,552},{217,537},{221,523},{219,510}},0.85)
fissure({{222,512},{225,502},{221,487},{223,473}},0.9)
fissure({{214,474},{211,460},{213,446}},0.82)
fissure({{218,428},{221,419},{223,409},{219,400}},0.8)
fissure({{221,393},{218,387},{216,380},{219,373}},0.84)
fissure({{213,369},{210,359},{214,349}},0.8)
fissure({{213,336},{210,324},{213,311}},0.73)
fissure({{222,326},{226,322},{239,319}},0.7)
fissure({{184,567},{195,561},{213,557}},0.7)
fissure({{253,565},{265,572},{282,574}},0.7)

--@ chunk 27
wait(8*24*60)
-- Keep bark illumination on a face, not across the whole silhouette.
work(oakBody:mask()*rect(0,295,550,300),{hand="detail",tool="round 3",pile=woodDark,coverage=6,load=0.9,dips={1,0.9,0.98},pressure={0.72,0.92},length={8,17},angle=1.3,clip=true,fill=true,seed=391})
wait(3*24*60)
woodFace=pile{{"raw umber",3},{"bone black",2},{"smalt",1.4},{"lead white",0.95},{"yellow ochre",0.2},medium=0.26}
woodLit=(ribbon({{226,557},{216,524},{219,478},{207,424},{216,373},{209,327}},{10,7,6,5,4,2.5})+poly({{190,564},{210,554},{226,550},{230,565},{212,561},{198,569}})+ribbon({{223,386},{180,350},{160,308}},{2,2.7,1.8})+ribbon({{223,324},{267,313},{296,279}},{2.1,1.5,1}))*oakBody:mask()
work(woodLit,{hand="detail",tool="round 1.7",pile=woodFace,coverage=4.6,load=0.73,pressure={0.63,0.82},length={4,9},angle=1.4,clip=true,fill=true,seed=392})
stipple(woodLit,{pile=barkDust,width=0.85,coverage=0.55,pressure={0.52,0.76},dips={6,0.65,0.94},cluster={0.3,6},clip=true,seed=393})
fissure({{218,552},{217,537},{221,523},{219,510}},0.85)
fissure({{218,500},{220,489},{218,477},{219,465}},0.8)
fissure({{210,437},{209,426},{211,413},{208,403}},0.75)
fissure({{217,381},{211,374},{210,360},{214,349}},0.8)
fissure({{227,557},{231,563},{237,571}},0.8)
fissure({{197,562},{184,566},{170,572}},0.7)
-- Tie the ruin to its spit of land, leaving the footing half veiled.
local foundation=poly({{691,437},{708,429},{725,430},{742,429},{761,433},{775,437},{760,441},{717,440},{698,441}},true)
work(foundation-chapel,{hand="detail",tool="filbert 2.5",pile=farLand,coverage=3.5,load=0.58,pressure={0.55,0.76},length={7,16},angle=0,clip=true,fill=true,seed=394})
work(ribbon({{690,441},{716,442},{740,441.5},{773,440.5}},{3,4,3,1.6}):soften(1.2),{hand="detail",tool="filbert 2",pile=mistEven,coverage=1.7,load=0.22,pressure={0.4,0.6},length={7,18},angle=0,clip=true,seed=395})

--@ chunk 28
wait(4*24*60)
frontGrain=pile{{"raw umber",2.5},{"smalt",1.8},{"bone black",1.2},{"lead white",2.7},{"green earth",0.4},medium=0.23}
stipple(fronts,{pile=frontGrain,width=0.9,coverage=0.85,pressure={0.55,0.82},dips={4,0.78,0.96},cluster={0.25,7},clip=true,seed=401})
-- Fine mineral seams turn over the forms, instead of outlining their boundaries.
crack({{118,587},{113,601},{103,611},{99,625},{103,635}},1.0,0.64)
crack({{28,581},{43,587},{48,602},{38,611}},0.9,0.64)
crack({{170,574},{181,578},{192,575},{211,582},{217,594}},1.0,0.7)
crack({{250,581},{248,593},{256,604}},0.9,0.6)
crack({{301,593},{314,601},{313,618},{304,626}},1.0,0.64)
crack({{734,560},{747,568},{766,569}},1.0,0.65)
crack({{774,554},{781,560},{777,568},{796,576},{799,586},{790,608},{797,624}},1.1,0.65)
crack({{796,576},{814,572},{827,580},{823,595}},0.9,0.65)
crack({{899,468},{886,476},{881,490},{888,506}},1.0,0.7)
crack({{644,511},{651,518},{655,533}},0.8,0.6)
crack({{419,650},{436,647},{449,653}},0.85,0.6)
-- Bark grain is confined to the face catching the sky.
work(woodLit,{hand="hatch",tool="round 0.85",pile=woodDark,coverage=1.5,load=0.38,pressure={0.45,0.68},length={2,5},angle=1.4,clip=true,seed=402})
fissure({{223,543},{221,536},{224,528}},0.65)
fissure({{217,525},{215,518},{219,509}},0.6)
fissure({{219,491},{216,483},{217,476}},0.66)
fissure({{210,426},{214,419},{213,413}},0.6)
fissure({{211,379},{214,374},{212,367}},0.64)
-- Uneven bristle-tip fringe on the conifers, especially the hanging comb branches.
stipple(spruceOpen:rim(2.4,0.4)-rockRidge,{pile=sprucePaint,width=1.05,coverage=1.7,pressure={0.48,0.75},dips={5,0.73,0.94},cluster={0.4,5},clip=false,feather=0.6,seed=403})
local needles=brush{kind="rigger",width=0.85,point=1}
local tips={{{932,397},{930,403},{927,406}},{{940,397},{938,403},{936,405}},{{950,397},{952,403},{952,406}},{{935,415},{934,421},{930,425}},{{928,431},{927,437},{924,440}},{{957,429},{959,435},{960,441}},{{967,445},{966,451},{968,454}},{{936,448},{934,454},{931,458}},{{984,420},{982,426},{981,428}},{{997,428},{998,434},{1001,436}},{{982,445},{980,451},{978,454}},{{995,453},{997,459},{998,462}}}
for _,pts in ipairs(tips) do
 needles:reload(sprucePaint,0.65)
 needles:stroke(pts,{pressure={0.65,0.01},ramps={0.01,0.3},shake=0.08})
end
-- A few dry seedheads, scarcely lighter than the grass bearing them.
local seedTip=brush{kind="round",width=0.7,point=0.55}
for _,pt in ipairs({{54,627},{136,639},{178,610},{294,646},{552,596},{654,564},{735,638},{867,619},{943,637},{979,472}}) do
 seedTip:reload(grassDull,0.6)
 seedTip:touch(pt[1],pt[2],{pressure=0.65,drag={1.2,-1.3}})
end

--@ chunk 29
-- A final, dry-ground shadow statement, lean enough to leave the mineral grain.
wait(6*24*60)
mineralShadow=pile{{"raw umber",3.2},{"bone black",2.0},{"smalt",2.3},{"lead white",0.48},{"red earth",0.18},medium=0.22}
mineralTurn=pile{{"raw umber",2.7},{"bone black",1.4},{"smalt",2.4},{"lead white",1.1},{"yellow ochre",0.15},medium=0.24}
work(fronts,{hand="detail",tool={kind="filbert",width=3.4,stiffness=0.5},pile=mineralShadow,coverage=4.8,load=0.91,dips={2,0.91,0.98},pressure={0.68,0.9},length={8,20},angle=1.25,clip=true,fill=true,seed=410})
wait(5*24*60)
work(turnFaces,{hand="detail",tool="filbert 2.7",pile=mineralTurn,coverage=4.3,load=0.8,dips={2,0.8,0.98},pressure={0.65,0.85},length={6,16},angle=1.05,clip=true,fill=true,seed=411})

--@ chunk 30
-- The rock is broken bedding, not a row of even-sided blocks.
wait(5*24*60)
mineralBed=pile{{"raw umber",2.8},{"smalt",2.4},{"bone black",1.35},{"lead white",1.95},{"yellow ochre",0.16},medium=0.24}
mineralPale=pile{{"lead white",3.8},{"raw umber",2.25},{"smalt",2},{"bone black",0.82},{"yellow ochre",0.2},medium=0.27}
local brokenPlanes=poly({{710,577},{724,580},{733,591},{733,607},{747,620},{770,627},{773,633},{743,621},{722,602}}):roughen(0.8,4,422)+poly({{756,584},{769,589},{776,599},{783,612},{781,624},{791,637},{779,634},{766,615},{766,604}}):roughen(0.7,4,423)+poly({{801,602},{819,593},{826,597},{822,611},{813,621},{802,630},{799,623}}):roughen(0.9,5,424)+poly({{805,504},{819,511},{836,511},{856,518},{857,522},{829,518},{809,512}}):roughen(0.6,5,425)+poly({{880,497},{890,491},{899,490},{903,502},{920,507},{912,512},{900,506},{889,511}}):roughen(0.8,5,426)+poly({{24,603},{35,607},{39,629},{57,639},{66,647},{55,647},{30,635},{24,620}}):roughen(0.8,5,427)+poly({{78,612},{91,615},{91,627},{106,635},{110,644},{96,642},{87,636},{82,624}}):roughen(0.7,5,428)+poly({{178,587},{191,592},{198,603},{214,608},{213,612},{186,605},{180,599}}):roughen(0.6,4,429)+poly({{286,609},{294,612},{295,628},{308,638},{317,646},{303,642},{289,629}}):roughen(0.6,4,430)+poly({{639,525},{647,529},{649,541},{659,545},{649,546},{639,539}}):roughen(0.5,4,431)
work(brokenPlanes*fronts,{hand="detail",tool="round 1.8",pile=mineralBed,coverage=3.6,load=0.61,pressure={0.5,0.8},length={3,9},angle=1.1,clip=true,seed=432})
-- Small uneven splittings on horizontal planes, some disappearing under frost.
local topBeds=poly({{710,568},{732,558},{747,557},{750,561},{737,570},{722,575}})+poly({{770,553},{790,558},{799,565},{803,575},{796,577},{785,566},{767,562}})+poly({{753,575},{766,579},{781,580},{797,589},{784,590},{769,585},{757,586}})+poly({{828,483},{850,478},{873,479},{880,484},{862,485},{845,489},{829,488}})+poly({{886,465},{905,468},{916,474},{899,475},{889,472}})+poly({{37,572},{59,569},{79,576},{83,583},{71,582},{52,578},{40,581}})+poly({{93,591},{111,596},{127,594},{130,599},{111,603},{97,599}})+poly({{172,570},{187,560},{202,561},{210,566},{195,569},{183,575}})+poly({{282,591},{295,584},{302,588},{312,597},{305,600},{298,594}})
work(topBeds:roughen(0.9,5,434)*rockTops,{hand="detail",tool="filbert 2",pile=mineralBed,coverage=2.6,load=0.46,pressure={0.46,0.72},length={3,9},angle=-0.25,clip=true,seed=435})
work(fronts,{hand="hatch",tool={kind="round",width=0.8,point=0.3},pile=mineralBed,coverage=0.45,load=0.22,pressure={0.4,0.7},length={2.5,7},angle=1.14,clip=true,seed=436})
-- Mineral flecks are close in value, clustered, and finer than the fractures.
stipple(fronts,{pile=mineralShadow,width=0.72,coverage=0.5,pressure={0.4,0.74},dips={5,0.5,0.97},cluster={0.65,5},clip=true,seed=437})
stipple(rockTops,{pile=mineralPale,width=0.8,coverage=0.45,pressure={0.44,0.8},dips={5,0.58,0.96},cluster={0.55,7},clip=true,seed=438})

--@ chunk 31
-- Restrain the new mineral planes with a common transparent dark.
wait(8*24*60)
rockShadeGlaze=pile{{"raw umber",3.2},{"bone black",2.1},{"smalt",2.2},{"red earth",0.1},medium=0.5}
work(fronts,{hand="glaze",tool={kind="filbert",width=5.5,stiffness=0.27},pile=rockShadeGlaze,coverage=1.65,load=0.28,dips={3,0.28,0.96},pressure={0.36,0.6},length={18,36},angle=1.15,clip=true,seed=445})
-- Loam occupies unequal pools. The evenly scattered old flecking remains only in islands.
loam=pile{{"raw umber",4},{"bone black",2.0},{"smalt",1.3},{"green earth",1.1},{"red earth",0.25},{"lead white",0.12},medium=0.29}
loamPools=poly({{0,556},{42,551},{94,562},{135,582},{163,604},{162,631},{133,649},{44,654},{0,637}},true):soften(7)+poly({{173,586},{222,582},{255,597},{276,624},{325,632},{365,656},{377,669},{199,672},{166,647}},true):soften(6)+poly({{657,535},{694,512},{733,512},{750,535},{722,548},{718,577},{700,606},{703,637},{679,657},{629,650},{611,625},{632,600},{633,566}},true):soften(9)+poly({{798,518},{850,524},{890,517},{928,517},{961,535},{1008,529},{1010,612},{984,646},{949,654},{931,616},{896,605},{873,581},{848,560},{796,548}},true):soften(9)+poly({{740,622},{771,635},{822,629},{852,641},{885,633},{911,645},{921,672},{715,672}},true):soften(6)
work(loamPools*soil-shale,{hand="body",tool="filbert 5.5",pile=loam,coverage=1.9,load=0.53,dips={4,0.53,0.96},pressure={0.44,0.7},length={11,30},angle=0.2,clip=true,seed=446})

--@ chunk 32
-- Follow the growth of the wood with drawn, interrupted ridges, not a filled ribbon.
barkMidFinal=pile{{"raw umber",3.8},{"bone black",2.0},{"smalt",1.8},{"lead white",0.72},{"yellow ochre",0.25},{"red earth",0.12},medium=0.3}
barkSkyFinal=pile{{"raw umber",3.2},{"bone black",1.6},{"smalt",2.1},{"lead white",1.3},{"yellow ochre",0.15},medium=0.29}
barkCreviceFinal=pile{{"bone black",4},{"raw umber",2.5},{"smalt",1.5},medium=0.26}
function woodmark(pts,p,w,pr,load)
 local b=brush{kind="rigger",width=w or 1.4,point=1,stiffness=0.5}
 b:load(p,load or 0.68)
 b:stroke(pts,{pressure={pr or 0.64,0.04},ramps={0.025,0.24},shake=0.11,clip=oakBody:mask()})
end
local grain={
 {{223,565},{217,558},{215,547},{211,539}},
 {{212,550},{207,542},{209,533},{208,526}},
 {{221,548},{217,537},{220,528},{218,518}},
 {{216,529},{213,520},{214,510},{215,502}},
 {{222,517},{220,508},{223,501},{224,490}},
 {{216,508},{214,499},{216,490},{214,480}},
 {{224,490},{226,482},{223,474},{224,464}},
 {{213,486},{211,477},{213,467},{211,459}},
 {{217,471},{215,463},{216,456},{214,449}},
 {{212,457},{212,445},{208,438},{209,429}},
 {{217,442},{215,431},{217,425},{219,417}},
 {{209,430},{207,421},{210,414},{208,406}},
 {{211,414},{214,406},{212,398},{215,391}},
 {{219,401},{217,394},{216,384},{219,376}},
 {{212,390},{210,381},{210,373},{212,366}},
 {{219,371},{215,364},{216,355},{214,349}},
 {{211,360},{209,351},{210,343},{208,335}},
 {{215,340},{211,331},{212,323},{213,317}},
 {{211,320},{209,310},{211,301},{212,293}},
 {{213,306},{215,297},{214,290},{217,281}},
 {{196,563},{206,558},{214,555},{220,553}},
 {{174,572},{185,568},{195,565},{199,561}},
 {{240,559},{250,561},{261,567},{270,570}},
 {{217,382},{206,376},{196,370},{187,361}},
 {{183,357},{177,347},{173,339},{167,327}},
 {{164,323},{161,314},{155,309},{146,304}},
 {{223,325},{237,322},{249,320},{261,316}},
 {{265,316},{275,310},{283,301},{290,291}},
 {{294,286},{297,279},{307,276},{319,274}},
 {{237,270},{242,260},{244,251},{254,245}},
 {{224,302},{225,291},{228,281},{233,274}},
 {{180,265},{173,256},{171,246},{165,239}},
 {{244,208},{247,199},{257,194},{267,188}},
 {{240,166},{245,158},{251,150},{258,145}}
}
for i,pts in ipairs(grain) do woodmark(pts,barkMidFinal,1.3+(i%4)*0.23,0.64,0.72) end
-- Light catches particular lips. None of these lines follows the whole tree edge.
woodmark({{210,545},{208,537},{209,531}},barkSkyFinal,0.95,0.62,0.63)
woodmark({{215,528},{213,520},{214,514}},barkSkyFinal,1.1,0.58,0.65)
woodmark({{214,491},{212,484},{214,478}},barkSkyFinal,0.85,0.64,0.65)
woodmark({{209,429},{208,421},{210,416}},barkSkyFinal,1.0,0.56,0.63)
woodmark({{213,386},{210,379},{211,373}},barkSkyFinal,0.85,0.54,0.64)
woodmark({{212,345},{210,339},{211,333}},barkSkyFinal,0.85,0.56,0.62)
woodmark({{218,324},{224,320},{234,318}},barkSkyFinal,0.8,0.62,0.65)
woodmark({{176,347},{171,338},{169,332}},barkSkyFinal,0.7,0.6,0.63)
woodmark({{293,286},{299,278},{307,274}},barkSkyFinal,0.75,0.64,0.62)
woodmark({{190,567},{201,561},{210,559}},barkSkyFinal,1.0,0.68,0.66)
-- Shadow fissures divide the ridges and wrap round a knot.
woodmark({{224,552},{226,542},{222,535},{224,526}},barkCreviceFinal,1.6,0.76,0.7)
woodmark({{219,524},{220,515},{218,508}},barkCreviceFinal,1.2,0.74,0.69)
woodmark({{223,498},{220,493},{221,484},{225,479}},barkCreviceFinal,1.5,0.74,0.7)
woodmark({{216,480},{215,470},{216,462}},barkCreviceFinal,1.2,0.7,0.7)
woodmark({{215,446},{212,438},{214,431}},barkCreviceFinal,1.35,0.73,0.68)
woodmark({{219,416},{224,409},{221,402},{218,396}},barkCreviceFinal,1.1,0.7,0.68)
woodmark({{215,370},{213,360},{216,353}},barkCreviceFinal,1.3,0.7,0.69)
woodmark({{220,331},{218,325},{224,321},{234,320}},barkCreviceFinal,1.2,0.7,0.7)

--@ chunk 33
-- The two spruces keep their drawn axes, but acquire hanging, unequal combs of needles.
spruceCoreFinal=pile{{"raw umber",2.5},{"bone black",2.4},{"smalt",2.8},{"green earth",2.3},{"lead white",0.55},medium=0.27}
spruceTipsFinal=pile{{"raw umber",2.4},{"bone black",1.5},{"smalt",3},{"green earth",2.8},{"lead white",1.15},{"yellow ochre",0.12},medium=0.28}
needleMassA=poly({{946,361},{944,375},{941,381},{940,388},{943,386},{942,395},{946,391},{950,396},{950,390},{953,393},{950,385},{949,378}})+poly({{946,388},{939,390},{935,394},{932,399},{937,398},{934,405},{939,402},{939,407},{942,402},{945,403},{947,400},{952,404},{954,410},{956,406},{960,410},{958,404},{955,399},{950,396}})+poly({{944,406},{937,407},{931,411},{926,416},{930,416},{925,423},{930,421},{931,427},{934,421},{938,419},{938,424},{941,421},{944,423},{945,417}})+poly({{948,406},{953,408},{959,412},{966,415},{968,421},{963,419},{962,425},{958,421},{957,427},{953,422},{948,419}})+poly({{943,421},{936,423},{929,426},{924,432},{917,437},{923,437},{923,444},{928,440},{930,445},{933,438},{936,441},{938,436},{942,437},{945,431}})+poly({{948,422},{957,426},{963,431},{971,435},{974,441},{970,439},{970,445},{967,442},{965,447},{962,444},{961,449},{958,443},{953,442},{948,436}})+poly({{943,440},{934,442},{929,447},{922,451},{918,456},{922,456},{923,460},{926,456},{929,461},{932,457},{937,460},{940,456},{944,459},{946,450}})+poly({{948,440},{955,443},{962,449},{970,452},{974,458},{969,456},{968,461},{964,459},{962,463},{958,459},{956,464},{952,460},{948,460},{946,454}})
needleMassB=poly({{991,383},{990,398},{986,403},{985,410},{988,408},{987,414},{991,410},{994,415},{994,409},{997,411},{995,404},{994,398}})+poly({{991,411},{985,414},{981,419},{977,426},{981,425},{979,432},{984,428},{985,433},{988,429},{991,432},{993,425}})+poly({{994,413},{999,418},{1005,422},{1008,429},{1003,426},{1004,433},{1000,430},{999,435},{996,430},{993,425}})+poly({{990,428},{984,433},{979,438},{974,444},{977,445},{974,451},{979,448},{980,453},{983,449},{987,454},{992,448}})+poly({{994,432},{1000,436},{1008,441},{1010,449},{1004,447},{1006,454},{1001,451},{999,455},{995,451},{992,444}})+poly({{990,450},{983,453},{978,458},{974,464},{979,463},{978,468},{982,466},{985,469},{989,466},{993,467},{995,461}})+poly({{995,452},{1000,456},{1007,460},{1010,467},{1004,465},{1004,470},{1000,468},{997,470},{993,465}})
needleMassFinal=(needleMassA+needleMassB):roughen(0.4,2,452)-rockRidge
work(needleMassFinal,{hand="detail",tool="round 1.35",pile=spruceCoreFinal,coverage=4.8,load=0.76,dips={3,0.76,0.97},pressure={0.62,0.88},length={3,7},angle=1.35,clip=true,fill=true,seed=453})
wait(4*24*60)
stipple(needleMassFinal,{pile=spruceTipsFinal,width=0.9,coverage=0.9,pressure={0.38,0.75},dips={5,0.53,0.95},cluster={0.6,4},drag={1.1,1.25},clip=true,seed=454})
stipple(needleMassFinal:rim(1.4,0.35),{pile=spruceCoreFinal,width=0.9,coverage=1.4,pressure={0.45,0.76},dips={5,0.63,0.97},drag={1.0,1.4},cluster={0.35,3},feather=0.5,clip=false,seed=455})

--@ chunk 34
-- A fine dry film compresses the overly vigorous side-plane contrasts without losing them.
wait(8*24*60)
print(drying(767,614),drying(865,509))
mineralRepose=pile{{"raw umber",3},{"bone black",2},{"smalt",2.2},{"lead white",0.48},{"green earth",0.25},medium=0.24}
mineralReposeTurn=pile{{"raw umber",2.7},{"bone black",1.5},{"smalt",2.4},{"lead white",0.95},{"yellow ochre",0.13},medium=0.25}
stipple(fronts,{pile=mineralRepose,width=1.1,coverage=1.7,pressure={0.58,0.86},dips={4,0.72,0.98},cluster={0.15,5},clip=true,seed=462})
stipple(turnFaces,{pile=mineralReposeTurn,width=1.05,coverage=1.3,pressure={0.55,0.82},dips={4,0.68,0.98},cluster={0.25,5},clip=true,seed=463})
-- Some frost only catches the broken upper surface, and doesn't make an outline.
stipple(rockTops,{pile=mineralPale,width=1.0,coverage=0.75,pressure={0.45,0.73},dips={5,0.57,0.97},cluster={0.6,7},clip=true,seed=464})
-- Narrow angular splittings: separately drawn crevices, chips and lips.
local faults=poly({{773,553},{779,558},{781,560},{778,565},{779,568},{795,575},{799,578},{800,585},{798,589},{797,580},{794,577},{777,570},{775.8,567.2},{779,561.3},{777.5,559.4}})+poly({{795,577},{811,573},{815,573},{827,579},{824,584},{823,582},{825,579.8},{814,574.2},{808,575.5}})+poly({{899,468},{893,472},{887,477},{883,486},{882.2,493},{884,489},{885,482},{889,477},{894,475}})+poly({{30,581},{39,584},{44,588},{48,598},{49,603},{46.5,603},{45.5,598},{42.5,590},{37,585}})+poly({{307,594},{315,600},{316,606},{313,617},{311,621},{312.5,615},{313.5,606},{312.5,601.5}})
work(faults*rocks,{hand="detail",tool="round 0.65",pile=mineralShadow,coverage=4.8,load=0.63,pressure={0.62,0.85},length={1.5,4},clip=true,fill=true,seed=465})
local chips=poly({{708,573},{714,574},{720,578},{716,582},{712,579}})+poly({{767,584},{775,588},{778,594},{774,594},{770,589}})+poly({{790,598},{799,598},{801,603},{796,605}})+poly({{838,501},{850,503},{849,507},{843,506}})+poly({{88,610},{94,612},{99,617},{93,615}})+poly({{284,606},{290,606},{292,611},{287,610}})
work(chips*rocks,{hand="detail",tool="round 0.9",pile=mineralPale,coverage=3.2,load=0.55,pressure={0.5,0.76},length={2,4},clip=true,seed=466})

--@ chunk 35
-- Small ground passages link, rather than outline, the bases of the stones.
wait(5*24*60)
soilFineFinal=pile{{"raw umber",3.7},{"bone black",1.7},{"smalt",1.5},{"green earth",1.8},{"lead white",0.32},{"yellow ochre",0.2},medium=0.27}
soilLightFinal=pile{{"raw umber",3.4},{"bone black",1.25},{"smalt",1.6},{"green earth",1.4},{"lead white",0.7},{"yellow ochre",0.3},medium=0.28}
local gritBeds=poly({{10,625},{43,628},{79,646},{104,650},{133,643},{153,647},{156,665},{6,668}},true):soften(3)+poly({{183,610},{211,618},{248,620},{270,637},{292,645},{328,653},{347,668},{175,668}},true):soften(3)+poly({{632,573},{658,585},{686,598},{700,619},{692,645},{672,652},{640,639},{623,609}},true):soften(4)+poly({{715,616},{739,627},{778,645},{803,648},{838,626},{866,630},{885,654},{883,668},{719,669}},true):soften(4)+poly({{800,508},{835,524},{877,525},{913,517},{954,521},{978,542},{977,560},{944,555},{904,550},{864,543},{826,544},{802,531}},true):soften(4)
work(gritBeds*soil-shale,{hand="hatch",tool={kind="round",width=1.35,point=0.5},pile=soilFineFinal,coverage=1.45,load=0.43,pressure={0.4,0.73},length={2.5,7},angle=0.18,clip=true,seed=472})
stipple(gritBeds*soil-shale,{pile=soilLightFinal,width=0.95,coverage=0.9,pressure={0.44,0.77},dips={7,0.55,0.97},cluster={0.7,7},drag={1.1,0.15},clip=true,seed=473})
-- Damp earth nibbles a few bottom edges; the geometry disappears in small runs.
local footEarth=poly({{713,606},{720,611},{724,620},{733,621},{744,628},{752,631},{748,634},{738,630},{725,625},{719,618}})+poly({{789,640},{796,640},{800,638},{806,639},{813,634},{819,635},{816,642},{804,647},{795,648}})+poly({{846,516},{857,517},{863,519},{877,515},{883,518},{892,514},{902,515},{906,521},{884,525},{855,525}})+poly({{273,619},{280,623},{285,632},{294,634},{298,640},{289,641},{280,633}})+poly({{176,603},{186,605},{190,610},{205,610},{206,616},{195,616},{183,612}})+poly({{16,648},{25,649},{32,647},{45,654},{60,653},{65,658},{36,661},{12,655}})
work(footEarth:roughen(0.7,3,474)*foreground,{hand="detail",tool="round 1.1",pile=loam,coverage=3.6,load=0.63,pressure={0.55,0.8},length={2,5},clip=true,seed=475})
-- Only a few stems are restated, darker than the old pale straw tufts.
sedgeFinal=pile{{"raw umber",3.2},{"yellow ochre",0.9},{"bone black",1.0},{"smalt",1.0},{"lead white",0.6},{"red earth",0.15},medium=0.3}
local sedge=brush{kind="rigger",width=1.0,point=1,stiffness=0.55}
local stems={
 {{723,633},{719,625},{721,616}},{{724,634},{729,623},{735,619}},{{722,632},{714,628},{710,621}},{{722,633},{720,619},{715,614}},
 {{805,654},{808,645},{806,636}},{{806,654},{818,646},{821,638}},{{806,653},{800,645},{795,642}},{{808,653},{814,640},{815,633}},
 {{871,534},{865,529},{863,522}},{{871,535},{879,528},{881,521}},{{874,535},{873,526},{874,518}},
 {{300,655},{296,642},{289,634}},{{302,655},{311,644},{319,639}},{{301,654},{300,641},{302,635}},
 {{173,631},{169,620},{165,615}},{{175,631},{179,619},{184,614}},{{174,630},{174,615},{170,608}},
 {{56,654},{51,644},{46,641}},{{56,653},{61,641},{60,634}},{{58,654},{69,646},{73,639}}
}
for _,pts in ipairs(stems) do
 sedge:reload(sedgeFinal,0.57)
 sedge:stroke(pts,{pressure={0.63,0.008},ramps={0.02,0.3},shake=0.1})
end
-- Wisps and small leaf litter, not another even stipple all over the foreground.
local leaf=brush{kind="round",width=2.0,point=0.65}
for _,pt in ipairs({{228,628},{246,635},{318,660},{326,655},{651,597},{671,611},{676,624},{731,644},{753,653},{847,639},{865,650},{888,637},{902,532},{927,540},{941,543},{79,647},{108,663}}) do
 leaf:reload(soilLightFinal,0.52)
 leaf:touch(pt[1],pt[2],{pressure=0.6,drag={2.1,-0.15},twist=0.2})
end

--@ chunk 36
-- Unify the ground's last small deposits. There is no new large passage.
wait(4*24*60)
stipple(soil-shale,{pile=loam,width=1.35,coverage=0.8,pressure={0.48,0.79},dips={5,0.58,0.97},cluster={0.4,8},clip=true,seed=481})
-- Fine oak short-shoots emerge from particular tips and nodes.
twigFinalPaint=pile{{"raw umber",2.3},{"bone black",2.7},{"smalt",1.4},{"red earth",0.22},medium=0.28}
function finalTwig(pts,w)
 local b=brush{kind="rigger",width=w or 0.8,point=1,stiffness=0.53}
 b:load(twigFinalPaint,0.6)
 b:stroke(pts,{pressure={0.66,0.005},ramps={0.01,0.32},shake=0.08})
end
finalTwig({{211,30},{205,29},{202,24}},0.8)
finalTwig({{209,29},{208,21},{211,17}},0.65)
finalTwig({{247,25},{246,19},{249,15}},0.72)
finalTwig({{253,28},{257,23},{257,18}},0.7)
finalTwig({{297,13},{302,10},{307,11}},0.65)
finalTwig({{292,23},{287,19},{285,12}},0.7)
finalTwig({{336,21},{332,17},{333,11}},0.67)
finalTwig({{344,42},{351,39},{354,32}},0.7)
finalTwig({{391,58},{398,57},{400,52}},0.7)
finalTwig({{371,74},{377,79},{378,84}},0.7)
finalTwig({{412,69},{418,71},{424,67}},0.72)
finalTwig({{458,111},{450,107},{447,100}},0.8)
finalTwig({{465,99},{473,97},{477,91}},0.72)
finalTwig({{513,150},{511,143},{514,140}},0.73)
finalTwig({{538,179},{546,182},{551,180}},0.7)
finalTwig({{522,228},{529,229},{534,225}},0.75)
finalTwig({{161,97},{169,95},{173,88}},0.75)
finalTwig({{143,89},{139,82},{133,79}},0.65)
finalTwig({{128,132},{120,131},{116,127}},0.75)
finalTwig({{101,145},{94,147},{90,143}},0.75)
finalTwig({{63,148},{69,148},{72,143}},0.67)
finalTwig({{17,159},{12,157},{10,151}},0.72)
finalTwig({{195,112},{202,107},{205,100}},0.7)
finalTwig({{220,114},{225,114},{230,109}},0.73)
finalTwig({{247,105},{240,102},{237,96}},0.7)
finalTwig({{345,126},{350,120},{354,118}},0.73)
finalTwig({{437,142},{445,143},{450,139}},0.7)
finalTwig({{332,207},{341,208},{347,202}},0.75)
finalTwig({{198,222},{204,223},{209,219}},0.73)
finalTwig({{169,177},{173,174},{174,168}},0.65)
finalTwig({{80,315},{85,311},{86,306}},0.67)
finalTwig({{106,338},{101,333},{97,327}},0.75)
finalTwig({{380,370},{388,366},{392,360}},0.7)
finalTwig({{296,420},{304,422},{308,418}},0.65)
finalTwig({{109,432},{105,423},{108,418}},0.7)
-- Two tiny dormant-bud shoots, part of the existing oak, remain below the open crown.
finalTwig({{228,396},{237,390},{237,383},{243,380}},1.0)
finalTwig({{237,388},{244,387},{248,382}},0.65)
finalTwig({{225,462},{233,464},{239,461},{241,455}},0.9)
-- Stout terminal buds occupy the very ends of selected short shoots.
local bud=brush{kind="round",width=0.57,point=0.8}
for _,pt in ipairs({{202,24},{257,18},{285,12},{333,11},{400,52},{424,67},{447,100},{477,91},{551,180},{534,225},{173,88},{116,127},{205,100},{347,202},{248,382}}) do
 bud:reload(twigFinalPaint,0.45)
 bud:touch(pt[1],pt[2],{pressure=0.62,drag={0.45,-0.65}})
end

--@ chunk 37
-- Remove only the conspicuous vein-like scumble in the air at the mountain feet.
mistRestFinal=pile{{"lead white",17},{"smalt",2.1},{"pale smalt",3},{"bone black",0.27},{"raw umber",0.16},{"yellow ochre",0.3},medium=0.27}
local twigReserve=ribbon({{115,384},{95,374},{91,359},{74,349}},{3.4,2.8,2.3,1.7})+ribbon({{144,393},{146,370},{129,356},{129,341},{119,334}},{4.5,3.6,2.8,2,1.4})+ribbon({{125,448},{111,431},{93,428}},{2.5,2.2,1.8})+ribbon({{164,443},{176,455},{179,475},{191,480}},2.4)+ribbon({{278,432},{295,420},{297,406}},{3.2,2.6,2.1})+ribbon({{274,393},{281,408},{301,416},{316,411}},{2.9,2.5,2,1.4})+ribbon({{292,371},{306,364},{313,349}},{3.0,2.4,1.6})+ribbon({{343,354},{357,369},{376,371},{383,383}},{3.1,2.8,2.5,1.9})+ribbon({{383,383},{398,383},{405,377}},{1.9,1.7,1.3})+ribbon({{228,396},{237,390},{237,383},{243,380}},1.6)+ribbon({{237,388},{244,387},{248,382}},1.3)
local mistReserve=oakBody:mask()+twigReserve+chapel+traveller+spruce1+spruce2+needleMassFinal+foreground
local mistRepair=rect(0,378,1000,60):soften(7)-mistReserve
stipple(mistRepair,{pile=mistRestFinal,width=1.25,coverage=function(x,y) return 5.0*smoothstep(380,391,y)*(1-smoothstep(412,435,y)) end,pressure={0.6,0.87},dips={3,0.79,0.98},cluster=0.1,feather=0.8,clip=true,seed=491})

--@ chunk 38
-- A cool intermediate veil dissolves the newly found upper boundary of the mist.
fogBlueFinal=pile{{"lead white",9},{"smalt",4},{"pale smalt",1.4},{"cobalt blue",0.35},{"bone black",0.45},{"raw umber",0.27},{"yellow ochre",0.18},medium=0.28}
local reserve=oakBody:mask()+chapel+traveller+spruce1+spruce2+needleMassFinal+foreground
reserve=reserve+ribbon({{115,384},{95,374},{91,359},{74,349}},{3.4,2.8,2.3,1.8})+ribbon({{144,393},{146,370},{129,356},{129,341},{119,334}},3.4)+ribbon({{274,393},{281,408},{301,416},{316,411}},{3,2.7,2.2,1.8})+ribbon({{278,432},{295,420},{297,406}},2.7)+ribbon({{343,354},{357,369},{376,371},{383,383}},2.8)+ribbon({{383,383},{398,383},{405,377}},1.9)+ribbon({{292,371},{306,364},{313,349}},2.7)+ribbon({{228,396},{237,390},{237,383},{243,380}},1.7)+ribbon({{237,388},{244,387},{248,382}},1.4)
local veil=rect(0,348,1000,79)-reserve
stipple(veil,{pile=fogBlueFinal,width=1.3,coverage=function(x,y) return 4.0*smoothstep(349,377,y)*(1-smoothstep(383,423,y)) end,pressure={0.56,0.82},dips={3,0.75,0.98},cluster=0.1,feather=0.8,clip=true,seed=501})

--@ chunk 39
-- The small touches now have a sound color, but their grain needs to fuse into air.
-- Work this entire narrow veil quickly, wet, and then cross it with the clean badger.
wait(5*24*60)
fogProtectFinal=oakBody:mask()+chapel+traveller+spruce1+spruce2+needleMassFinal+foreground
fogProtectFinal=fogProtectFinal+ribbon({{115,384},{95,374},{91,359},{74,349}},3.5)+ribbon({{144,393},{146,370},{129,356},{129,341},{119,334}},3.8)+ribbon({{220,420},{245,402},{274,393},{292,371}},{7,6,4.8,2.3})+ribbon({{274,393},{281,408},{301,416},{316,411}},2.7)+ribbon({{278,432},{295,420},{297,406}},2.7)+ribbon({{343,354},{357,369},{376,371},{383,383}},2.8)+ribbon({{383,383},{398,383},{405,377}},1.9)+ribbon({{292,371},{306,364},{313,349}},2.7)+ribbon({{228,396},{237,390},{237,383},{243,380}},1.8)+ribbon({{237,388},{244,387},{248,382}},1.4)+ribbon({{125,448},{111,431},{93,428}},2.5)
local fogBlueArea=rect(0,349,1000,86)-fogProtectFinal
work(fogBlueArea,{hand="body",tool={kind="filbert",width=20,stiffness=0.25},pile=fogBlueFinal,coverage=3.5,load_at=function(x,y) return 0.77*math.exp(-((y-381)/26)^2) end,dips={2,0.78,0.98},pressure={0.56,0.82},length={56,92},angle=0,clip=true,seed=510})
local fogPaleArea=rect(0,382,1000,85)-fogProtectFinal
work(fogPaleArea,{hand="body",tool={kind="filbert",width=20,stiffness=0.25},pile=mistRestFinal,coverage=3.1,load_at=function(x,y) return 0.68*math.exp(-((y-433)/27)^2) end,dips={2,0.72,0.98},pressure={0.55,0.8},length={56,92},angle=0,clip=true,seed=511})
blend(rect(0,349,1000,123)-fogProtectFinal,{tool="badger 48",angle=math.pi/2,pressure={0.57,0.81},length={43,72},coverage=2.3,clip=true,seed=512})
blend(rect(0,355,1000,112)-fogProtectFinal,{tool="badger 48",angle=0.07,pressure={0.45,0.7},length={90,150},coverage=0.8,clip=true,seed=513})
-- Rejoin the near branch where earlier veils accidentally broke its continuity.
branch({{220,420},{245,402},{274,393},{292,371}},0.76,5)
twig({{274,393},{281,408},{301,416},{316,411}},1.4)
twig({{292,371},{306,364},{313,349}},1.6)

--@ chunk 40
-- Return the mountain contours through the upper veil; no horizontal cut remains.
horizonCreamFinal=pile{{"lead white",17},{"pale smalt",1.2},{"yellow ochre",0.55},{"vermilion",0.08},{"bone black",0.17},medium=0.28}
local horizonRestore=(-farMount):times(function(x,y) return smoothstep(325,341,y)*(1-smoothstep(370,385,y)) end)-fogProtectFinal
work(horizonRestore,{hand="detail",tool={kind="filbert",width=12,stiffness=0.3},pile=horizonCreamFinal,coverage=3.6,load=0.8,dips={2,0.8,0.98},pressure={0.58,0.82},length={30,60},angle=0,clip=true,seed=520})
local farRestore=farMount:times(function(x,y) return smoothstep(329,345,y)*(1-smoothstep(350,398,y)) end)-fogProtectFinal
work(farRestore,{hand="detail",tool={kind="filbert",width=12,stiffness=0.3},pile=farRock,coverage=3.4,load=0.73,dips={2,0.73,0.98},pressure={0.56,0.8},length={30,60},angle=0,clip=true,seed=521})
local nearRestore=nearMount:times(function(x,y) return 1-smoothstep(348,407,y) end)-fogProtectFinal
work(nearRestore,{hand="detail",tool={kind="filbert",width=11,stiffness=0.3},pile=nearRock,coverage=3.3,load=0.72,dips={2,0.72,0.98},pressure={0.55,0.8},length={28,53},angle=0.06,clip=true,seed=522})
blend(rect(0,329,1000,80)-fogProtectFinal,{tool="badger 32",angle=0,pressure={0.4,0.6},length={66,115},coverage=0.7,clip=true,seed=523})
-- The far chapel has a footing again, but the spit is scarcely more solid than the air.
footingAirFinal=pile{{"lead white",8},{"smalt",3.2},{"raw umber",0.65},{"bone black",0.42},{"green earth",0.6},medium=0.35}
local distantFoot=poly({{692,439},{706,433},{719,431},{736,432},{746,432},{760,437},{772,440},{756,442},{740,443},{718,442},{698,443}},true):soften(1.5)
work(distantFoot-chapel,{hand="detail",tool="filbert 2.3",pile=footingAirFinal,coverage=3.4,load=0.55,pressure={0.48,0.72},length={6,13},angle=0,clip=true,seed=524})

--@ chunk 41
-- The air is now quiet. Every near shoot must still have a continuous route to the trunk.
wait(6*24*60)
branch({{176,345},{154,348},{128,332},{110,306},{86,299}},0.9,7)
branch({{268,316},{296,321},{311,342},{343,354},{368,346}},0.7,5)
branch({{144,393},{146,370},{129,356},{129,341},{119,334}},0.66,4)
branch({{173,421},{164,443},{139,451},{125,448},{102,462}},0.72,4.5)
branch({{226,474},{251,453},{270,450},{278,432}},0.78,5)
branch({{220,420},{245,402},{274,393},{292,371}},0.76,5)
twig({{128,332},{108,338},{90,332},{81,315}},1.8)
twig({{153,348},{150,369},{136,381}},1.9)
twig({{84,402},{63,409},{51,401}},1.6)
twig({{115,384},{95,374},{91,359},{74,349}},1.7)
twig({{129,341},{111,347},{101,342}},1.3)
twig({{125,448},{111,431},{93,428}},1.5)
twig({{102,462},{88,473},{72,470}},1.5)
twig({{164,443},{176,455},{179,475},{191,480}},1.4)
twig({{367,291},{361,311},{367,330},{383,337}},1.6)
twig({{311,342},{300,354},{302,366}},1.5)
twig({{343,354},{357,369},{376,371},{383,383}},1.6)
twig({{368,346},{382,331},{402,330}},1.5)
twig({{292,371},{306,364},{313,349}},1.6)
twig({{274,393},{281,408},{301,416},{316,411}},1.4)
twig({{278,432},{295,420},{297,406}},1.6)
twig({{383,383},{398,383},{405,377}},0.9)
finalTwig({{380,370},{388,366},{392,360}},0.7)
finalTwig({{296,420},{304,422},{308,418}},0.65)
finalTwig({{109,432},{105,423},{108,418}},0.7)
finalTwig({{106,338},{101,333},{97,327}},0.75)
-- A last broken bark light at the lower branch collars.
woodmark({{232,413},{241,405},{250,402}},barkMidFinal,0.85,0.55,0.59)
woodmark({{239,462},{249,455},{263,452}},barkMidFinal,0.75,0.55,0.58)
woodmark({{170,429},{166,439},{154,447}},barkMidFinal,0.75,0.56,0.57)

--@ chunk 42
-- The last found lower edges of the wooded range are allowed to disappear into one soft, drawn veil.
footMistFinal=pile{{"lead white",12},{"pale smalt",3.5},{"smalt",2},{"bone black",0.33},{"raw umber",0.21},{"yellow ochre",0.2},medium=0.32}
local airEdge=poly({{-15,390},{94,387},{193,388},{306,393},{412,390},{530,397},{656,389},{771,393},{894,386},{1015,390},{1015,429},{901,427},{781,435},{660,433},{551,438},{427,430},{300,434},{170,426},{59,431},{-15,425}},true):soften(18)
local frontReserve=fogProtectFinal+ellipse(741,440,55,9)+ribbon({{173,421},{164,443},{139,451},{125,448},{102,462}},5.3)+ribbon({{226,474},{251,453},{270,450},{278,432}},5.7)+ribbon({{84,402},{63,409},{51,401}},2.6)+ribbon({{164,443},{176,455},{179,475},{191,480}},2.7)
work(airEdge-frontReserve,{hand="body",tool={kind="filbert",width=18,stiffness=0.25},pile=footMistFinal,coverage=2.7,load=0.42,dips={2,0.42,0.98},pressure={0.42,0.68},length={47,82},angle=0,clip=true,seed=530})
blend(airEdge-frontReserve,{tool="badger 38",angle=0.1,pressure={0.42,0.63},length={76,133},coverage=1.0,clip=true,seed=531})

--@ chunk 43
-- Close the last small break in the lower oak bough, across the fully set mist.
wait(3*24*60)
branch({{145,392},{115,384},{84,402}},0.78,4.5)
twig({{84,402},{63,409},{51,401}},1.6)
branch({{173,421},{164,443},{139,451},{125,448},{102,462}},0.72,4.5)
branch({{226,474},{251,453},{270,450},{278,432}},0.78,5)
twig({{125,448},{111,431},{93,428}},1.5)
twig({{164,443},{176,455},{179,475},{191,480}},1.4)
twig({{278,432},{295,420},{297,406}},1.6)

--@ chunk 44
wait(10*24*60)
stoneWeatherBase=pile{{"lead white",3.8},{"raw umber",2.3},{"smalt",2},{"bone black",0.7},{"yellow ochre",0.32},medium=0.2}
stoneWeatherCool=pile{{"lead white",2.5},{"raw umber",2.7},{"smalt",2.5},{"bone black",0.85},{"green earth",0.35},medium=0.22}
stoneWeatherLight=pile{{"lead white",5.5},{"raw umber",2.0},{"smalt",1.6},{"bone black",0.48},{"yellow ochre",0.36},medium=0.2}
-- Quiet the disproportionately dark drawing on this broad, near stone first.
local nearTop=rockRight*rockTops
work(nearTop,{hand="body",tool={kind="filbert",width=4.5,stiffness=0.35},pile=stoneWeatherBase,coverage=4.3,load=0.88,dips={2,0.88,0.98},pressure={0.64,0.86},length={10,21},angle=-0.21,clip=true,fill=true,seed=541})
wait(4*24*60)
-- A dipping bed on the right and a higher, weather-worn shoulder on the left.
local sinkingBed=poly({{777,547},{802,552},{814,558},{824,566},{847,577},{829,586},{803,597},{791,586},{784,582},{784,569},{770,557}},true):soften(4)*nearTop
work(sinkingBed,{hand="body",tool={kind="filbert",width=4,stiffness=0.35},pile=stoneWeatherCool,coverage=1.5,load=0.47,dips={3,0.47,0.95},pressure={0.38,0.61},length={10,20},angle=0.31,clip=true,seed=542})
local raisedShoulder=poly({{710,570},{731,559},{752,548},{769,548},{775,558},{766,566},{753,568},{742,575},{726,576}},true):soften(4)*nearTop
work(raisedShoulder,{hand="body",tool={kind="filbert",width=4,stiffness=0.3},pile=stoneWeatherLight,coverage=1.5,load=0.48,dips={3,0.48,0.95},pressure={0.38,0.61},length={8,19},angle=-0.19,clip=true,seed=543})
print(drying(773,565))

--@ chunk 45
wait(8*24*60)
local nearTop=rockRight*rockTops
-- The little bristle starts were too separate. Re-wet this one plane quickly,
-- and knit the shoulder and dipping bed while the whole film is still open.
work(nearTop,{hand="body",tool={kind="filbert",width=12,stiffness=0.25},pile=stoneWeatherBase,coverage=4.7,load=0.92,dips={1,0.92,0.99},pressure={0.7,0.9},length={26,46},angle=-0.17,clip=true,fill=true,seed=550})
local lowerBed=poly({{776,547},{805,554},{825,568},{849,578},{803,598},{790,584},{782,570},{769,558}},true):soften(7)*nearTop
work(lowerBed,{hand="body",tool={kind="filbert",width=10,stiffness=0.22},pile=stoneWeatherCool,coverage=1.15,load=0.45,dips={2,0.45,0.97},pressure={0.44,0.67},length={23,37},angle=0.1,clip=true,seed=551})
local crown=poly({{712,569},{750,548},{768,549},{775,559},{753,567},{741,574},{727,575}},true):soften(6)*nearTop
work(crown,{hand="body",tool={kind="filbert",width=10,stiffness=0.22},pile=stoneWeatherLight,coverage=1.15,load=0.44,dips={2,0.44,0.97},pressure={0.44,0.66},length={20,34},angle=-0.12,clip=true,seed=552})
blend(nearTop,{tool={kind="badger",width=22,stiffness=0.16},angle=-0.15,pressure={0.44,0.7},length={38,62},coverage=2.3,clip=true,seed=553})
print(drying(773,565))

--@ chunk 46
wait(6*24*60)
stoneWeatherDust=pile{{"lead white",3.15},{"raw umber",2.5},{"smalt",2.1},{"bone black",0.8},{"yellow ochre",0.25},medium=0.23}
stoneWeatherSpark=pile{{"lead white",4.7},{"raw umber",2.15},{"smalt",2},{"bone black",0.63},{"yellow ochre",0.23},medium=0.22}
stoneWeatherFault=pile{{"lead white",1.55},{"raw umber",2.7},{"smalt",2.2},{"bone black",1.2},medium=0.26}
stoneWeatherShadow=pile{{"raw umber",3.1},{"bone black",2.1},{"smalt",2.4},{"lead white",0.23},{"green earth",0.25},medium=0.24}
local nearTop=rockRight*rockTops
-- Mineral grains sit in unequal beds, not in a pattern across the whole stone.
local dustBeds=poly({{724,569},{737,557},{757,552},{765,554},{758,560},{750,563},{752,568},{742,572},{734,571}})+poly({{777,558},{789,556},{802,561},{806,567},{798,570},{796,576},{784,570}})+poly({{772,579},{785,578},{796,582},{810,585},{825,582},{834,581},{827,588},{804,594},{797,590},{785,588}})
stipple(dustBeds:roughen(0.7,3,561)*nearTop,{pile=stoneWeatherDust,width=0.76,coverage=1.05,pressure={0.4,0.74},dips={6,0.58,0.96},cluster={0.5,4},drag={0.6,-0.18},clip=true,seed=562})
stipple(nearTop,{pile=stoneWeatherSpark,width=0.6,coverage=0.35,pressure={0.37,0.69},dips={8,0.47,0.96},cluster={0.65,5},clip=true,seed=563})
local pen=brush{kind="rigger",width=0.85,point=1,stiffness=0.4}
local faults={
 {{776,550},{780,557},{777,564},{785,569}},
 {{784,569},{793,573},{798,579},{797,590}},
 {{794,574},{809,571},{819,574},{823,580}},
 {{742,561},{738,566},{748,571},{760,573}},
 {{800,596},{797,605},{801,613},{801,620}}
}
for _,pts in ipairs(faults) do
 pen:reload(stoneWeatherFault,0.6)
 pen:stroke(pts,{pressure={0.7,0.015},ramps={0.015,0.28},shake=0.06,clip=rockRight})
end
-- A crumbled edge, with dark undercuts and one modestly illuminated split.
local undercut=poly({{722,581},{736,586},{742,592},{751,595},{770,607},{776,608},{777,613},{767,611},{750,601},{739,599},{736,593},{724,588}}):roughen(0.65,3,565)*fronts*rockRight
work(undercut,{hand="detail",tool="round 1.4",pile=stoneWeatherShadow,coverage=3.1,load=0.61,pressure={0.5,0.76},length={3,7},angle=0.34,clip=true,seed=566})
local split=poly({{790,607},{797,599},{802,600},{805,608},{810,613},{804,617},{801,613},{797,613},{796,607}}):roughen(0.45,2,568)*rockRight
work(split,{hand="detail",tool="round 1.1",pile=mineralBed,coverage=2.6,load=0.45,pressure={0.47,0.68},length={2,5},clip=true,seed=569})
local lip=brush{kind="round",width=0.65,point=1}
lip:load(stoneWeatherSpark,0.4);lip:stroke({{778.2,551.5},{781.4,557.2},{778.9,564.1}},{pressure={0.64,0.015},shake=0.06,clip=nearTop})
lip:reload(stoneWeatherSpark,0.37);lip:stroke({{803,586},{815,584.1},{822,582.6}},{pressure={0.62,0.01},shake=0.1,clip=nearTop})

--@ chunk 47
wait(7*24*60)
stoneLeftAir=pile{{"lead white",4.1},{"raw umber",2.6},{"smalt",1.8},{"bone black",0.72},{"yellow ochre",0.56},{"red earth",0.08},medium=0.21}
stoneLeftShade=pile{{"lead white",2.8},{"raw umber",2.8},{"smalt",1.9},{"bone black",0.88},{"yellow ochre",0.4},medium=0.24}
stoneRidgeAir=pile{{"lead white",3.9},{"raw umber",2.3},{"smalt",2.8},{"bone black",0.78},{"yellow ochre",0.15},medium=0.23}
-- Each stone is a new drawing and a separate wet passage.
local leftTop=rockLeft*rockTops
work(leftTop,{hand="body",tool={kind="filbert",width=12,stiffness=0.25},pile=stoneLeftAir,coverage=4.8,load=0.92,dips={1,0.92,0.99},pressure={0.69,0.9},length={25,43},angle=0.08,clip=true,fill=true,seed=580})
local leftLow=poly({{-7,588},{12,590},{30,583},{47,589},{58,595},{88,602},{109,600},{130,593},{148,595},{129,610},{84,611},{51,606},{23,600},{-7,606}},true):soften(6)*leftTop
work(leftLow,{hand="body",tool={kind="filbert",width=10,stiffness=0.24},pile=stoneLeftShade,coverage=1.2,load=0.48,dips={2,0.48,0.97},pressure={0.42,0.65},length={22,37},angle=0.11,clip=true,seed=581})
local leftHigh=poly({{14,576},{37,567},{56,565},{80,570},{94,579},{77,582},{63,579},{49,583},{29,582}},true):soften(6)*leftTop
work(leftHigh,{hand="body",tool={kind="filbert",width=10,stiffness=0.23},pile=stoneWeatherLight,coverage=1.1,load=0.41,dips={2,0.41,0.97},pressure={0.42,0.64},length={20,34},angle=0,clip=true,seed=582})
blend(leftTop,{tool={kind="badger",width=22,stiffness=0.17},angle=0.13,pressure={0.44,0.7},length={35,59},coverage=2.2,clip=true,seed=583})
local ridgeTop=rockRidge*rockTops
work(ridgeTop,{hand="body",tool={kind="filbert",width=12,stiffness=0.25},pile=stoneRidgeAir,coverage=4.8,load=0.92,dips={1,0.92,0.99},pressure={0.69,0.9},length={25,43},angle=-0.1,clip=true,fill=true,seed=584})
local ridgeLow=poly({{790,486},{816,481},{836,487},{849,487},{860,494},{866,497},{847,502},{812,501},{798,495}},true):soften(6)*ridgeTop
work(ridgeLow,{hand="body",tool={kind="filbert",width=10,stiffness=0.22},pile=stoneWeatherCool,coverage=1.15,load=0.44,dips={2,0.44,0.97},pressure={0.44,0.66},length={23,37},angle=-0.15,clip=true,seed=585})
local ridgeHigh=poly({{849,469},{875,463},{902,467},{919,472},{934,479},{919,482},{899,479},{882,482},{868,477},{851,477}},true):soften(7)*ridgeTop
work(ridgeHigh,{hand="body",tool={kind="filbert",width=10,stiffness=0.22},pile=stoneWeatherLight,coverage=1.15,load=0.4,dips={2,0.4,0.97},pressure={0.43,0.66},length={23,37},angle=0.03,clip=true,seed=586})
blend(ridgeTop,{tool={kind="badger",width=22,stiffness=0.16},angle=-0.1,pressure={0.44,0.7},length={38,62},coverage=2.2,clip=true,seed=587})

--@ chunk 48
wait(6*24*60)
local mainTops=(rockLeft+rockRidge)*rockTops
local beds=poly({{15,578},{29,574},{49,569},{68,571},{73,576},{57,579},{44,578},{37,583},{25,581}})+poly({{67,588},{82,584},{98,589},{107,590},{127,594},{126,598},{112,601},{101,596},{86,596},{74,592}})+poly({{809,485},{829,476},{844,477},{854,481},{850,486},{831,489},{819,489}})+poly({{861,472},{875,466},{886,470},{888,475},{878,477},{869,475}})+poly({{899,478},{913,475},{931,480},{939,484},{926,486},{912,484},{905,483}})
stipple(beds:roughen(0.7,3,591)*mainTops,{pile=stoneWeatherDust,width=0.76,coverage=1.0,pressure={0.42,0.74},dips={6,0.57,0.96},cluster={0.52,4},drag={0.7,-0.12},clip=true,seed=592})
stipple(mainTops,{pile=stoneWeatherSpark,width=0.62,coverage=0.32,pressure={0.38,0.69},dips={8,0.46,0.97},cluster={0.6,5},clip=true,seed=593})
-- Straight bedding faults with elbows, thinning into the stone instead of black grooves.
local seams=poly({{35.9,578},{43.6,582},{44.4,587},{49.2,592},{49.6,599},{48.7,599.6},{48.4,592.4},{43.5,587.5},{42.8,582.5},{35.6,578.7}})+poly({{43.4,582.1},{57,580.4},{67,582},{74.2,586.3},{87,587},{88,587.6},{74,587},{66.5,582.8},{57.1,581.1},{43.4,582.8}})+poly({{112.7,589},{109,594.9},{108,599.7},{108.5,603},{107.6,603.5},{107.3,599.4},{108.2,594.6},{112.2,588.6}})+poly({{899.4,468.2},{893.6,473.2},{886.5,475.8},{886.8,482.1},{881.3,488.6},{880.6,489.1},{885.9,481.8},{885.7,475.4},{893.2,472.5},{899,467.8}})+poly({{885.8,481.8},{874.4,480.6},{865,483.3},{854,482.8},{851,483.1},{854.2,483.4},{865,483.9},{874.6,481.3},{886.1,482.5}})
work(seams*mainTops,{hand="detail",tool={kind="round",width=0.45,point=0.6},pile=stoneWeatherFault,coverage=4.4,load=0.64,dips={3,0.64,0.97},pressure={0.6,0.86},length={1,3},clip=true,fill=true,seed=594})
-- Small plane scratches, restricted to patches of exposed bedding.
work(beds*mainTops,{hand="hatch",tool={kind="round",width=0.65,point=0.4},pile=stoneWeatherSpark,coverage=0.36,load=0.31,pressure={0.4,0.66},length={1.6,4.3},angle=0.04,clip=true,seed=595})

--@ chunk 49
wait(6*24*60)
stoneFrontBody=pile{{"raw umber",3.1},{"bone black",1.85},{"smalt",2.4},{"lead white",0.68},{"green earth",0.25},medium=0.24}
stoneFrontTurn=pile{{"raw umber",2.7},{"bone black",1.28},{"smalt",2.8},{"lead white",1.3},{"yellow ochre",0.1},medium=0.24}
stoneFrontFoot=pile{{"raw umber",3},{"bone black",2.35},{"smalt",2.2},{"lead white",0.25},medium=0.26}
local nearFace=rockRight*fronts
work(nearFace,{hand="body",tool={kind="filbert",width=11,stiffness=0.27},pile=stoneFrontBody,coverage=4.7,load=0.91,dips={1,0.91,0.99},pressure={0.69,0.89},length={23,39},angle=0.48,clip=true,fill=true,seed=601})
local nearTurn=poly({{804,599},{848,579},{839,616},{799,639},{787,621}}):soften(1.7)*nearFace
work(nearTurn,{hand="body",tool={kind="filbert",width=8,stiffness=0.24},pile=stoneFrontTurn,coverage=2.4,load=0.65,dips={2,0.65,0.98},pressure={0.53,0.77},length={18,30},angle=-0.9,clip=true,seed=602})
blend(nearTurn:grow(3)*nearFace,{tool={kind="badger",width=15,stiffness=0.17},angle=-0.8,pressure={0.43,0.68},length={25,40},coverage=1.5,clip=true,seed=603})
local foot=poly({{714,600},{734,611},{758,617},{772,619},{782,630},{798,641},{786,640},{742,627},{717,610}},true):soften(4)*nearFace
work(foot,{hand="body",tool={kind="filbert",width=8,stiffness=0.23},pile=stoneFrontFoot,coverage=1.5,load=0.45,dips={2,0.45,0.97},pressure={0.45,0.68},length={18,28},angle=0.3,clip=true,seed=604})
blend(foot:grow(4)*nearFace,{tool={kind="badger",width=16,stiffness=0.16},angle=0.33,pressure={0.43,0.65},length={25,39},coverage=1.5,clip=true,seed=605})
print(drying(757,609))

--@ chunk 50
wait(7*24*60)
stoneFaceGrain=pile{{"raw umber",3},{"bone black",1.65},{"smalt",2.5},{"lead white",0.9},{"yellow ochre",0.08},medium=0.26}
local nearFace=rockRight*fronts
-- A broken ledge interrupts the single flat front; the lower faces remain dark.
local flake=outline{{718,583,"c"},{730,588},{737,598,"c"},{750,605},{761,604,"c"},{772,611},{778,620,"c"},{785,631,"c"},{776,625},{770,618,"c"},{757,615},{746,612},{736,608,"c"},{731,602},{723,599,"c"},char="broken",amount=0.42,edge=0.3,seed=611}:mask()*nearFace
work(flake,{hand="detail",tool="round 1.6",pile=stoneFaceGrain,coverage=3.0,load=0.62,dips={3,0.62,0.97},pressure={0.5,0.78},length={3,7},angle=0.38,clip=true,seed=612})
local cavities=poly({{725,598},{735,603},{740,610},{751,613},{768,620},{780,625},{783,629},{774,627},{764,624},{750,618},{737,614},{732,607},{725,603}})+poly({{820,592},{829,588},{829,595},{823,600},{819,608},{812,612},{815,605},{820,600}})+poly({{742,615},{753,622},{761,623},{766,629},{760,628},{750,625},{742,619}})
work(cavities:roughen(0.5,2.5,614)*nearFace,{hand="detail",tool="round 1.1",pile=stoneFrontFoot,coverage=3.8,load=0.62,dips={3,0.62,0.97},pressure={0.52,0.8},length={2,5},clip=true,seed=615})
local splits=poly({{798.1,596},{796.6,605.8},{803.5,616.6},{798.3,627.7},{797.5,628.6},{802.4,616.8},{795.7,606},{797.4,595.8}})+poly({{747.7,591},{749.8,602},{755,605.5},{756.5,613},{755.6,613.2},{754.3,606},{749.1,602.5},{747.2,591}})+poly({{812.9,610},{821,612},{823.8,607.3},{828.3,605.2},{824.6,608},{821.6,612.8},{813.6,610.7}})
work(splits*nearFace,{hand="detail",tool="round 0.5",pile=stoneFrontFoot,coverage=4.0,load=0.65,pressure={0.57,0.84},length={1,3},clip=true,fill=true,seed=616})
stipple(nearFace,{pile=stoneFaceGrain,width=0.75,coverage=0.65,pressure={0.4,0.72},dips={7,0.48,0.97},cluster={0.65,4},drag={0.6,0.24},clip=true,seed=617})
-- Worn corner flakes are grey, not the chalk-white triangles of the former film.
local corners=poly({{710,573},{716,576},{719,580},{715,582},{712,578}})+poly({{764,583},{773,587},{776,592},{772,591},{769,588}})+poly({{796,598},{800,597},{805,601},{799,603}})+poly({{840,583},{844,581},{844,586},{841,590}})
work(corners*rockRight,{hand="detail",tool="round 0.8",pile=stoneWeatherDust,coverage=3.4,load=0.56,pressure={0.5,0.76},length={2,4},clip=true,seed=618})

--@ chunk 51
wait(7*24*60)
stoneFarFace=pile{{"raw umber",2.6},{"bone black",1.55},{"smalt",3},{"lead white",0.96},{"green earth",0.2},medium=0.25}
local leftFace=rockLeft*fronts
work(leftFace,{hand="body",tool={kind="filbert",width=11,stiffness=0.27},pile=stoneFrontBody,coverage=4.6,load=0.91,dips={1,0.91,0.99},pressure={0.68,0.88},length={23,40},angle=0.28,clip=true,fill=true,seed=621})
local leftTurn=poly({{63,608},{124,611},{147,595},{157,616},{125,635},{74,643}}):soften(1.8)*leftFace
work(leftTurn,{hand="body",tool={kind="filbert",width=8,stiffness=0.24},pile=stoneFrontTurn,coverage=2.3,load=0.64,dips={2,0.64,0.98},pressure={0.54,0.77},length={18,31},angle=-0.5,clip=true,seed=622})
blend(leftTurn:grow(3)*leftFace,{tool={kind="badger",width=16,stiffness=0.17},angle=-0.5,pressure={0.43,0.68},length={25,40},coverage=1.5,clip=true,seed=623})
local leftFoot=poly({{-7,634},{19,640},{28,638},{49,643},{70,645},{125,627},{155,616},{129,640},{56,658},{-7,653}},true):soften(4)*leftFace
work(leftFoot,{hand="body",tool={kind="filbert",width=9,stiffness=0.24},pile=stoneFrontFoot,coverage=1.5,load=0.43,dips={2,0.43,0.97},pressure={0.44,0.67},length={18,29},angle=0.01,clip=true,seed=624})
blend(leftFoot:grow(4)*leftFace,{tool={kind="badger",width=17,stiffness=0.17},angle=0.04,pressure={0.43,0.65},length={25,40},coverage=1.5,clip=true,seed=625})
-- The farther rock is cooler and receives a lower-contrast statement.
local ridgeFace=rockRidge*fronts
work(ridgeFace,{hand="body",tool={kind="filbert",width=11,stiffness=0.26},pile=stoneFarFace,coverage=4.6,load=0.91,dips={1,0.91,0.99},pressure={0.69,0.88},length={23,39},angle=0.23,clip=true,fill=true,seed=626})
local ridgeTurn=poly({{849,502},{899,487},{946,486},{927,506},{866,520}}):soften(1.8)*ridgeFace
work(ridgeTurn,{hand="body",tool={kind="filbert",width=8,stiffness=0.24},pile=stoneFrontTurn,coverage=2.0,load=0.58,dips={2,0.58,0.98},pressure={0.53,0.76},length={18,30},angle=-0.15,clip=true,seed=627})
blend(ridgeTurn:grow(3)*ridgeFace,{tool={kind="badger",width=16,stiffness=0.17},angle=-0.15,pressure={0.43,0.67},length={25,40},coverage=1.5,clip=true,seed=628})
local ridgeFoot=poly({{799,502},{818,507},{843,516},{864,517},{904,510},{934,502},{930,512},{846,525},{800,513}},true):soften(4)*ridgeFace
work(ridgeFoot,{hand="body",tool={kind="filbert",width=8,stiffness=0.23},pile=stoneFrontBody,coverage=1.4,load=0.43,dips={2,0.43,0.97},pressure={0.44,0.66},length={18,29},angle=0.12,clip=true,seed=629})
blend(ridgeFoot:grow(3)*ridgeFace,{tool={kind="badger",width=16,stiffness=0.16},angle=0.12,pressure={0.43,0.65},length={25,40},coverage=1.4,clip=true,seed=630})

--@ chunk 52
wait(7*24*60)
local leftFace=rockLeft*fronts
local leftFlake=outline{{1,611,"c"},{13,607},{24,613,"c"},{31,621},{46,626},{51,634,"c"},{65,643,"c"},{53,641},{43,637},{30,633,"c"},{24,628},{21,619,"c"},{9,617},char="broken",amount=0.48,edge=0.3,seed=641}:mask()*leftFace
work(leftFlake,{hand="detail",tool="round 1.6",pile=stoneFaceGrain,coverage=3.1,load=0.61,dips={3,0.61,0.97},pressure={0.5,0.78},length={3,7},angle=0.3,clip=true,seed=642})
local leftRecess=poly({{2,627},{20,631},{27,640},{44,645},{50,651},{38,649},{25,642},{20,636},{3,632}})+poly({{112,616},{120,613},{126,615},{125,620},{114,623},{106,631},{91,636},{82,643},{78,641},{89,634},{104,626},{109,620}})
work(leftRecess:roughen(0.5,2.5,643)*leftFace,{hand="detail",tool="round 1.1",pile=stoneFrontFoot,coverage=3.5,load=0.6,dips={3,0.6,0.97},pressure={0.51,0.79},length={2,5},clip=true,seed=644})
local leftSplits=poly({{49.1,606},{47.8,615},{51.2,623.8},{45.7,635.8},{45.1,637},{50.3,623.7},{47,615.1},{48.5,605.8}})+poly({{100.7,612.4},{97.9,621.1},{103,628.5},{98.9,637.2},{97.6,639},{102.1,628.5},{97,621.3},{100,612.3}})+poly({{26.5,614},{25,619},{31.2,625.5},{30.9,633.7},{30.3,635},{30.5,625.7},{24.4,619.2},{26,613.8}})
work(leftSplits*leftFace,{hand="detail",tool="round 0.5",pile=stoneFrontFoot,coverage=4.0,load=0.64,pressure={0.56,0.83},length={1,3},clip=true,fill=true,seed=645})
stipple(leftFace,{pile=stoneFaceGrain,width=0.77,coverage=0.64,pressure={0.4,0.73},dips={7,0.48,0.97},cluster={0.62,4},drag={0.65,0.15},clip=true,seed=646})
local ridgeFace=rockRidge*fronts
local ridgeLedge=outline{{804,500,"c"},{821,505},{835,505,"c"},{843,511},{854,511},{861,516,"c"},{851,517},{837,513},{832,509,"c"},{819,509},{811,505},char="broken",amount=0.42,edge=0.3,seed=647}:mask()*ridgeFace
work(ridgeLedge,{hand="detail",tool="round 1.3",pile=stoneFrontTurn,coverage=2.8,load=0.57,pressure={0.5,0.76},length={3,7},angle=0.08,clip=true,seed=648})
local ridgeBreaks=poly({{821,506},{834,508},{843,513},{856,514},{859,518},{846,517},{834,511},{824,510}})+poly({{903,493},{912,491},{913,497},{907,502},{901,505},{896,504},{903,500}})
work(ridgeBreaks:roughen(0.4,2.5,649)*ridgeFace,{hand="detail",tool="round 1",pile=stoneFrontBody,coverage=3.1,load=0.57,pressure={0.5,0.78},length={2,5},clip=true,seed=650})
local ridgeSplits=poly({{881,490.5},{883.6,497.8},{878.8,507},{880,515},{879.5,516},{878,507},{882.6,497.6},{880.4,490.5}})+poly({{826,502},{827.4,506.3},{824,512.9},{824.5,515.2},{823.9,515.6},{823.4,512.8},{826.6,506.1},{825.6,502}})
work(ridgeSplits*ridgeFace,{hand="detail",tool="round 0.45",pile=stoneFrontBody,coverage=4.0,load=0.61,pressure={0.56,0.82},length={1,3},clip=true,fill=true,seed=651})
stipple(ridgeFace,{pile=stoneFrontTurn,width=0.73,coverage=0.56,pressure={0.4,0.7},dips={8,0.45,0.97},cluster={0.66,4},drag={0.7,0.08},clip=true,seed=652})

--@ chunk 53
wait(7*24*60)
-- Earth covers short, unequal runs of the lower edges, seating the stones again.
local settlingEarth=poly({{711,607},{718,613},{721,619},{728,621},{738,627},{741,632},{732,630},{723,625},{716,619}})+poly({{782,640},{790,641},{796,638},{803,639},{812,633},{818,633},{814,639},{801,646},{791,648},{782,645}})+poly({{0,646},{13,648},{23,645},{34,649},{45,650},{53,655},{63,653},{67,658},{42,661},{13,657},{0,653}})+poly({{120,636},{130,633},{134,629},{142,627},{146,625},{142,632},{132,639},{122,643}})+poly({{806,513},{817,515},{825,516},{835,520},{847,520},{856,519},{863,522},{870,519},{881,519},{886,522},{873,527},{847,528},{823,523},{809,519}})
work(settlingEarth:roughen(0.55,2.8,661)*foreground,{hand="detail",tool="round 1.25",pile=loam,coverage=3.4,load=0.6,dips={3,0.6,0.97},pressure={0.51,0.78},length={2,5},angle=0.13,clip=true,seed=662})
-- Leave the chip lips, but compress their conspicuous lower pale triangles.
local chippedShade=poly({{711.5,576},{715,579},{718,580},{715,582},{712,579}})+poly({{770,587},{774,590},{776,592},{772,591}})+poly({{797,600},{801,599.2},{804,601.4},{800,603}})+poly({{842,584.4},{844,583.1},{844,586.5},{841.4,589.6}})
work(chippedShade*rockRight,{hand="detail",tool="round 0.8",pile=stoneFaceGrain,coverage=4.0,load=0.7,pressure={0.56,0.81},length={1.5,3.5},clip=true,seed=663})
-- A very few short grasses cross stone and loam at the joins; no dotted edging.
local sedge=brush{kind="rigger",width=0.8,point=1,stiffness=0.5}
local stems={
 {{724,631},{719,626},{718,620}},{{725,632},{729,624},{733,622}},
 {{797,651},{795,645},{798,638}},{{798,651},{805,646},{809,641}},
 {{52,660},{48,654},{48,648}},{{52,660},{58,654},{58,646}},
 {{131,644},{132,638},{136,634}},{{132,644},{127,640},{125,635}},
 {{849,530},{847,524},{844,521}},{{850,530},{856,524},{859,521}}
}
for _,pts in ipairs(stems) do
 sedge:reload(sedgeFinal,0.53)
 sedge:stroke(pts,{pressure={0.6,0.005},ramps={0.02,0.34},shake=0.07})
end
print(wait(0))

--@ chunk 54
wait(8*24*60)
pathStoneBody=pile{{"lead white",4.7},{"smalt",1.8},{"raw umber",1.8},{"bone black",0.48},{"yellow ochre",0.25},medium=0.21}
pathStoneTurn=pile{{"lead white",3.3},{"smalt",2.1},{"raw umber",2.3},{"bone black",0.7},{"yellow ochre",0.25},medium=0.22}
pathStoneCrown=pile{{"lead white",5.3},{"pale smalt",1.6},{"raw umber",1.6},{"bone black",0.44},{"yellow ochre",0.23},medium=0.22}
-- The closest flagstone is painted as a shallow rise, not a frost-covered flat cutout.
pathFlagNear=poly({{405,662},{444,624},{483,613},{491,631},{461,652},{450,667},{402,667}})*slabs
work(pathFlagNear,{hand="body",tool={kind="filbert",width=11,stiffness=0.24},pile=pathStoneBody,coverage=4.6,load=0.92,dips={1,0.92,0.99},pressure={0.69,0.88},length={23,39},angle=-0.32,clip=true,fill=true,seed=672})
local turn=poly({{464,619},{482,614},{489,631},{474,640},{458,653},{449,667},{433,669},{438,652},{450,641}},true):soften(4)*pathFlagNear
work(turn,{hand="body",tool={kind="filbert",width=8,stiffness=0.23},pile=pathStoneTurn,coverage=1.25,load=0.46,dips={2,0.46,0.97},pressure={0.44,0.66},length={20,32},angle=-0.36,clip=true,seed=673})
local crown=poly({{408,659},{427,639},{445,627},{461,623},{466,627},{450,640},{433,650},{418,667},{404,669}},true):soften(4)*pathFlagNear
work(crown,{hand="body",tool={kind="filbert",width=8,stiffness=0.23},pile=pathStoneCrown,coverage=1.2,load=0.43,dips={2,0.43,0.97},pressure={0.44,0.65},length={19,31},angle=-0.34,clip=true,seed=674})
blend(pathFlagNear,{tool={kind="badger",width=18,stiffness=0.17},angle=-0.25,pressure={0.44,0.68},length={28,43},coverage=2.2,clip=true,seed=675})
print(drying(454,641))

--@ chunk 55
pathFlagMiddle=poly({{468,610},{490,589},{529,576},{537,587},{510,604},{491,619}})*slabs
pathFlagUpper=poly({{516,573},{539,555},{567,550},{555,568},{534,579}})*slabs
-- These two planes have their own rise and broken dip toward the verge.
work(pathFlagMiddle,{hand="body",tool={kind="filbert",width=9,stiffness=0.24},pile=pathStoneBody,coverage=4.5,load=0.92,dips={1,0.92,0.99},pressure={0.69,0.87},length={19,31},angle=-0.23,clip=true,fill=true,seed=680})
local midDip=poly({{514,582},{530,578},{535,587},{520,596},{509,603},{491,618},{486,614},{500,595}},true):soften(3.5)*pathFlagMiddle
work(midDip,{hand="body",tool={kind="filbert",width=7,stiffness=0.23},pile=pathStoneTurn,coverage=1.15,load=0.42,dips={2,0.42,0.97},pressure={0.44,0.65},length={17,27},angle=-0.15,clip=true,seed=681})
local midRise=poly({{471,609},{490,591},{507,584},{519,582},{519,586},{503,594},{489,603},{480,612}},true):soften(3)*pathFlagMiddle
work(midRise,{hand="body",tool={kind="filbert",width=7,stiffness=0.23},pile=pathStoneCrown,coverage=1.1,load=0.42,dips={2,0.42,0.97},pressure={0.44,0.65},length={17,26},angle=-0.25,clip=true,seed=682})
blend(pathFlagMiddle,{tool={kind="badger",width=16,stiffness=0.16},angle=-0.24,pressure={0.44,0.68},length={23,35},coverage=2.2,clip=true,seed=683})
work(pathFlagUpper,{hand="body",tool={kind="filbert",width=7,stiffness=0.23},pile=pathStoneBody,coverage=4.4,load=0.9,dips={1,0.9,0.99},pressure={0.67,0.87},length={15,26},angle=-0.14,clip=true,fill=true,seed=684})
local upperDip=poly({{551,553},{565,551},{555,567},{546,572},{534,578},{530,575},{541,564}},true):soften(2.5)*pathFlagUpper
work(upperDip,{hand="body",tool={kind="filbert",width=6,stiffness=0.22},pile=pathStoneTurn,coverage=1.0,load=0.4,dips={2,0.4,0.97},pressure={0.44,0.64},length={13,22},angle=-0.14,clip=true,seed=685})
blend(pathFlagUpper,{tool={kind="badger",width=13,stiffness=0.16},angle=-0.12,pressure={0.43,0.67},length={18,30},coverage=2.0,clip=true,seed=686})
print(wait(0))

--@ chunk 56
wait(6*24*60)
pathMineralDust=pile{{"lead white",3.8},{"raw umber",2.0},{"smalt",1.9},{"bone black",0.58},{"yellow ochre",0.24},medium=0.23}
pathMineralPale=pile{{"lead white",5.0},{"raw umber",1.6},{"smalt",1.9},{"bone black",0.45},{"yellow ochre",0.18},medium=0.22}
pathBedding=pile{{"lead white",2.5},{"raw umber",2.6},{"smalt",1.8},{"bone black",0.82},{"yellow ochre",0.16},medium=0.26}
pathFinishedFlags=pathFlagNear+pathFlagMiddle+pathFlagUpper
-- Unequal patches of exposed grain, held close to the stone's body value.
local grainBeds=poly({{424,645},{435,633},{450,628},{459,628},{453,635},{439,640},{437,646},{427,652}})+poly({{440,655},{451,646},{461,641},{473,635},{475,638},{465,646},{459,649},{448,660},{442,665}})+poly({{477,605},{490,592},{501,588},{514,585},{517,588},{503,593},{497,601},{485,610}})+poly({{503,605},{509,599},{524,590},{530,589},{524,595},{515,600},{508,608}})+poly({{531,568},{541,558},{554,555},{559,555},{553,562},{542,566},{538,572}})
stipple(grainBeds:roughen(0.6,2.8,691)*pathFinishedFlags,{pile=pathMineralDust,width=0.66,coverage=0.92,pressure={0.4,0.72},dips={6,0.53,0.97},cluster={0.63,4.5},drag={0.55,-0.23},clip=true,seed=692})
stipple(pathFinishedFlags,{pile=pathMineralPale,width=0.5,coverage=0.37,pressure={0.38,0.68},dips={8,0.45,0.97},cluster={0.66,4},clip=true,seed=693})
local seam=brush{kind="rigger",width=0.85,point=1,stiffness=0.42}
seam:load(pathBedding,0.62)
seam:stroke({{418,651},{432,648},{444,641},{452,642},{467,634}},{pressure={0.72,0.012},ramps={0.02,0.3},shake=0.07,clip=pathFlagNear})
seam:reload(pathBedding,0.52)
seam:stroke({{444,641},{442,649},{446,654}},{pressure={0.62,0.01},ramps={0.02,0.34},shake=0.06,clip=pathFlagNear})
seam:reload(pathBedding,0.59)
seam:stroke({{478,604},{492,603},{500,595},{508,595},{522,588}},{pressure={0.67,0.01},ramps={0.02,0.34},shake=0.06,clip=pathFlagMiddle})
seam:reload(pathBedding,0.55)
seam:stroke({{526,570},{536,569},{541,563},{550,560}},{pressure={0.65,0.009},ramps={0.02,0.36},shake=0.06,clip=pathFlagUpper})
-- Scattered shallow bedding scratches, finer than the seams.
work(grainBeds*pathFinishedFlags,{hand="hatch",tool={kind="round",width=0.6,point=0.5},pile=pathMineralPale,coverage=0.25,load=0.28,pressure={0.4,0.63},length={1.5,3.8},angle=-0.17,clip=true,seed=694})

--@ chunk 57
pathSiltFinal=pile{{"raw umber",2.6},{"lead white",1.9},{"yellow ochre",0.9},{"smalt",1.3},{"bone black",0.58},medium=0.24}
pathFrostFinal=pile{{"lead white",6.5},{"pale smalt",1.6},{"raw umber",1.2},{"bone black",0.27},{"yellow ochre",0.18},medium=0.23}
-- Soil occupies a few notches and low corners instead of describing a complete border.
local siltLaps=poly({{418,643},{424,641},{426,644},{422,647},{423,650},{416,655},{411,655}})+poly({{477,613},{484,611},{488,616},{485,618},{481,616}})+poly({{477,599},{482,597},{484,600},{480,603},{481,605},{475,608},{471,608}})+poly({{514,572},{521,568},{524,570},{522,573},{525,575},{534,577},{533,580},{521,580}})
work(siltLaps:roughen(0.45,2,701)*foreground,{hand="detail",tool="round 1.1",pile=pathSiltFinal,coverage=3.5,load=0.63,dips={3,0.63,0.98},pressure={0.51,0.78},length={2,4.5},angle=-0.23,clip=true,seed=702})
local vergeBites=poly({{478,639},{480,643},{472,647},{471,651},{464,654},{458,658},{453,660},{455,655},{463,650},{466,646},{473,643}})+poly({{532,590},{535,592},{528,596},{525,599},{517,602},{514,607},{509,608},{511,603},{518,599},{519,596},{524,594}})
work(vergeBites:roughen(0.5,2.4,703)*foreground,{hand="detail",tool="round 1.1",pile=pathSiltFinal,coverage=3.4,load=0.62,dips={3,0.62,0.98},pressure={0.5,0.77},length={2,5},angle=-0.3,clip=true,seed=704})
local crumbledEarth=poly({{473,644},{479,639},{482,639},{481,645},{473,650},{471,654},{463,656},{465,652},{469,651}})+poly({{523,599},{529,595},{530,597},{524,601},{521,605},{516,607},{516,604}})
work(crumbledEarth:roughen(0.4,1.7,705)*foreground,{hand="detail",tool="round 0.8",pile=loam,coverage=3.0,load=0.58,dips={3,0.58,0.98},pressure={0.5,0.77},length={1.5,3.5},clip=true,seed=706})
-- Only the exposed high runs retain a trace of frost.
local ice=brush{kind="round",width=1.05,point=0.85,stiffness=0.5}
ice:load(pathFrostFinal,0.26);ice:stroke({{428,641},{435,635},{440,631}},{pressure={0.59,0.02},ramps={0.06,0.35},shake=0.07,clip=pathFlagNear})
ice:reload(pathFrostFinal,0.24);ice:stroke({{449,626},{457,625},{460,623}},{pressure={0.55,0.015},ramps={0.03,0.3},shake=0.07,clip=pathFlagNear})
ice:reload(pathFrostFinal,0.25);ice:stroke({{416,658},{421,653}},{pressure={0.52,0.01},ramps={0.03,0.3},shake=0.07,clip=pathFlagNear})
ice:reload(pathFrostFinal,0.26);ice:stroke({{489,592},{495,589},{502,587}},{pressure={0.55,0.01},ramps={0.03,0.34},shake=0.07,clip=pathFlagMiddle})
ice:reload(pathFrostFinal,0.23);ice:stroke({{508,585},{514,583},{518,583}},{pressure={0.53,0.01},ramps={0.03,0.35},shake=0.07,clip=pathFlagMiddle})
ice:reload(pathFrostFinal,0.24);ice:stroke({{537,557},{544,555},{548,555}},{pressure={0.5,0.01},ramps={0.03,0.35},shake=0.07,clip=pathFlagUpper})
-- Two wiry blades cross the worn joins; they are rooted in the existing verge.
local wiry=brush{kind="rigger",width=0.75,point=1,stiffness=0.5}
wiry:load(grassDull,0.52);wiry:stroke({{460,657},{454,650},{451,641}},{pressure={0.56,0.008},ramps={0.02,0.36},shake=0.06})
wiry:reload(grassDull,0.5);wiry:stroke({{460,657},{459,649},{462,645}},{pressure={0.49,0.005},ramps={0.02,0.38},shake=0.06})
wiry:reload(grassDull,0.5);wiry:stroke({{520,605},{517,598},{516,593}},{pressure={0.49,0.005},ramps={0.02,0.35},shake=0.05})
print(wait(0))

--@ chunk (finishing)
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)
