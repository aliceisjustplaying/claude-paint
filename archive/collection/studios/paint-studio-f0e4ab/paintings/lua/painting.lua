-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=900, aspect=1.25, linen={16,14}, seed=37, ground={{pile={{"red earth",2},{"yellow ochre",1},{"raw umber",1}},um=90,apply="knife",texture=0.38},{pile={{"lead white",5},{"yellow ochre",0.35},{"raw umber",0.12}},um=45,apply="brush"}}}
print(W,H)

--@ chunk 2
h=pencil("2H")
h:sketch({{0,445},{80,435},{145,450},{205,428},{270,442},{330,415},{385,432},{445,420},{505,440},{575,426},{640,438},{700,419},{765,433},{835,410},{900,425},{1000,408}},{pressure=0.32,wander=1.4})
h:sketch({{0,690},{90,666},{165,650},{235,657},{300,640},{370,648},{430,624},{475,600},{520,580},{555,566},{580,560},{605,552},{630,540},{648,520},{669,507},{690,516},{714,530},{750,532},{790,553},{840,566},{890,552},{930,530},{970,505},{1000,490}},{pressure=0.42,wander=1.2})
h:sketch({{0,760},{100,735},{170,708},{245,693},{310,695},{360,710},{405,730}},{pressure=0.3,wander=1})
h:sketch({{302,455},{320,438},{343,444},{360,419},{385,428},{409,410},{429,433},{451,430},{470,448},{470,465}},{pressure=0.32,wander=1})
h:sketch({{376,432},{379,402},{385,402},{388,433},{376,432}},{pressure=0.4,wander=0.3})
h:sketch({{625,541},{630,516},{638,502},{648,494},{657,495},{664,508},{663,530}},{pressure=0.45,wander=0.4})
h:line({{490,435},{490,720}},{pressure=0.18})
h:line({{0,478},{1000,448}},{pressure=0.12})

--@ chunk 3
skyline={{0,440},{95,432},{175,443},{250,430},{330,438},{410,420},{500,438},{585,426},{670,437},{760,419},{850,430},{920,414},{1000,422}}
skyM=above(skyline)
p_sky=pile{{"pale smalt",4},{"lead white",2.4},{"cobalt blue",0.9},{"raw umber",0.35}}
work(skyM,{hand="broad",pile=p_sky,coverage=2.0,fill=true,angle=0.08,angle_jitter=0.2,seed=11})

--@ chunk 4
p_upper=pile{{"cobalt blue",1.4},{"pale smalt",2.5},{"raw umber",0.65},{"lead white",1.2},medium=0.16}
work(rect(0,0,1000,330):soften(65),{hand="glaze",pile=p_upper,seed=14})
p_glow=pile{{"lead white",3},{"yellow ochre",0.55},{"red earth",0.25},{"pale smalt",0.8},medium=0.08}
work(rect(0,340,1000,120):soften(34),{hand="glaze",pile=p_glow,seed=15})
moonGlow=ellipse(218,180,70,70):soften(35)
p_moon=pile{{"lead white",4},{"yellow ochre",0.35},{"pale smalt",0.35}}
work(moonGlow,{hand="body",pile=p_moon,coverage=1.4,fill=true,seed=16})
farM=poly({{0,447},{78,427},{155,441},{221,400},{264,416},{317,389},{365,407},{409,377},{455,398},{502,391},{556,414},{623,389},{681,405},{730,372},{774,389},{824,366},{865,402},{918,384},{1000,419},{1000,510},{0,510}},true)
p_far=pile{{"pale smalt",2},{"cobalt blue",1.3},{"raw umber",0.6},{"lead white",0.7}}
work(farM,{hand="broad",pile=p_far,coverage=2.1,fill=true,angle=-0.18,seed=17})

--@ chunk 5
p_veil=pile{{"cobalt blue",1.5},{"pale smalt",2.8},{"raw umber",0.45},{"lead white",1},medium=0.24}
work(rect(128,78,190,208):soften(48),{hand="glaze",pile=p_veil,seed=21})
moonCore=ellipse(218,180,25,25):soften(7)
p_core=pile{{"lead white",3.2},{"pale smalt",0.7},{"yellow ochre",0.12}}
work(moonCore,{hand="detail",pile=p_core,coverage=2.4,fill=true,clip=true,seed=22})
lakeM=below({{0,505},{150,500},{310,504},{465,500},{620,504},{790,495},{1000,492}})
p_water=pile{{"pale smalt",3.2},{"lead white",2.5},{"cobalt blue",0.7},{"raw umber",0.3}}
work(lakeM,{hand="broad",pile=p_water,coverage=2.0,fill=true,angle=0.03,angle_jitter=0.14,seed=23})

--@ chunk 6
p_isle=pile{{"raw umber",1.6},{"cobalt blue",1},{"green earth",1.1},{"pale smalt",0.65},{"red earth",0.35}}
isleA=poly({{42,484},{58,472},{79,476},{95,461},{117,467},{134,456},{151,468},{173,462},{192,475},{213,481},{217,492},{40,495}},true)
isleB=poly({{277,486},{291,471},{309,474},{326,459},{344,465},{362,447},{380,455},{398,441},{418,450},{435,446},{451,462},{473,458},{487,478},{487,496},{278,498}},true)
isleC=poly({{704,489},{724,478},{743,482},{758,466},{780,475},{802,458},{826,472},{850,466},{875,484},{884,496},{704,500}},true)
work(isleA,{hand="body",pile=p_isle,coverage=2.0,fill=true,edge="soft",seed=31})
work(isleB,{hand="body",pile=p_isle,coverage=2.4,fill=true,edge="soft",seed=32})
work(isleC,{hand="body",pile=p_isle,coverage=1.8,fill=true,edge="soft",seed=33})
p_church=pile{{"raw umber",1.5},{"bone black",0.55},{"cobalt blue",0.7}}
chapel=poly({{372,455},{373,439},{381,439},{381,425},{385,418},{389,425},{389,439},{398,439},{398,432},{422,432},{438,442},{440,455}},true)
work(chapel,{hand="detail",pile=p_church,coverage=2.4,fill=true,clip=true,seed=34})

--@ chunk 7
print(wait(7*60))

--@ chunk 8
shore=poly({{0,682},{54,674},{116,660},{177,650},{240,651},{302,644},{355,648},{402,634},{445,619},{477,603},{506,584},{535,570},{563,555},{587,544},{610,533},{625,518},{639,506},{648,503},{659,508},{673,516},{692,523},{715,529},{740,535},{765,542},{792,550},{818,552},{844,549},{869,539},{893,525},{918,508},{945,493},{972,480},{1000,474},{1000,800},{0,800}},true)
p_shore=pile{{"raw umber",2.2},{"bone black",1.15},{"cobalt blue",1.05},{"red earth",0.35},{"green earth",0.45}}
work(shore,{hand="body",pile=p_shore,coverage=3.0,fill=true,clip=true,angle=-0.28,seed=41})

--@ chunk 9
print(wait(24*60))

--@ chunk 10
p_bark=pile{{"raw umber",1.2},{"bone black",0.7},{"cobalt blue",0.7},{"red earth",0.25}}
bt=brush{kind="rigger",width=7,point=0.94,stiffness=0.72}
bb=brush{kind="rigger",width=4,point=0.96,stiffness=0.68}
bf=brush{kind="rigger",width=2,point=0.96,stiffness=0.6}
local function draw(b,pts,p1,p2)
 b:reload(p_bark,0.9)
 b:stroke(pts,{pressure={p1,p2},ramps={0.025,0.12},orient="along",shake=0.3})
end
draw(bt,{{926,518},{919,491},{912,462},{914,435},{906,408},{911,381},{902,356},{908,329},{905,307}},0.92,0.14)
draw(bb,{{917,472},{895,451},{879,430},{866,411},{851,400}},0.78,0.14)
draw(bb,{{895,451},{880,446},{861,443},{844,434}},0.68,0.13)
draw(bf,{{870,429},{854,419},{838,416}},0.62,0.12)
draw(bb,{{911,439},{932,425},{949,405},{968,393}},0.75,0.12)
draw(bf,{{947,408},{955,389},{972,378},{987,371}},0.64,0.1)
draw(bf,{{956,399},{972,399},{984,390}},0.55,0.1)
draw(bb,{{910,405},{889,389},{875,366},{862,344},{844,334}},0.73,0.12)
draw(bf,{{875,367},{857,361},{842,347},{832,330}},0.54,0.1)
draw(bf,{{862,345},{866,326},{857,312}},0.5,0.08)
draw(bb,{{909,379},{931,364},{950,343},{963,324}},0.67,0.1)
draw(bf,{{948,347},{964,346},{979,335}},0.55,0.08)
draw(bf,{{952,341},{954,321},{968,305}},0.5,0.08)
draw(bb,{{905,355},{886,340},{875,319},{866,303}},0.63,0.1)
draw(bf,{{878,324},{856,319},{842,305}},0.5,0.08)
draw(bf,{{913,340},{929,322},{941,301}},0.52,0.08)
draw(bf,{{883,393},{867,388},{852,374}},0.52,0.08)
draw(bf,{{934,425},{951,431},{970,425}},0.5,0.08)
draw(bf,{{885,450},{878,467},{863,479}},0.5,0.08)

--@ chunk 11
print(wait(8*60))

--@ chunk 12
work(chapel:grow(10),{hand="detail",pile=p_isle,coverage=3.2,fill=true,clip=true,seed=51})
chapelNave=poly({{394,438},{403,432},{412,433},{425,439},{435,448},{435,452},{394,452}})
chapelTower=poly({{382,451},{382,430},{387,430},{387,420},{393,410},{399,421},{399,430},{404,430},{404,451}})
work(chapelNave,{hand="detail",pile=p_church,coverage=3.4,fill=true,clip=true,seed=52})
work(chapelTower,{hand="detail",pile=p_church,coverage=3.5,fill=true,clip=true,seed=53})

--@ chunk 13
p_rockmid=pile{{"raw umber",1.25},{"red earth",0.45},{"cobalt blue",0.5},{"lead white",0.75}}
p_rocklight=pile{{"lead white",1.4},{"yellow ochre",0.5},{"red earth",0.35},{"cobalt blue",0.25}}
faceA=poly({{541,601},{553,581},{572,566},{590,551},{611,541},{630,521},{646,511},{653,518},{647,538},{638,552},{619,560},{606,578},{588,589},{575,607},{554,618}})
faceB=poly({{662,520},{682,526},{707,533},{732,541},{748,550},{751,560},{738,566},{719,556},{701,554},{685,545},{672,537}})
faceC=poly({{808,551},{832,548},{857,540},{881,526},{904,510},{930,495},{951,491},{947,509},{926,525},{901,540},{878,551},{850,563},{828,564}})
faceD=poly({{84,676},{133,660},{182,650},{230,653},{259,666},{243,681},{211,683},{179,676},{150,688},{113,694}})
work(faceA,{hand="scumble",pile=p_rockmid,coverage=1.7,clip=true,angle=-0.58,seed=61})
work(faceB,{hand="scumble",pile=p_rockmid,coverage=1.2,clip=true,angle=0.18,seed=62})
work(faceC,{hand="scumble",pile=p_rockmid,coverage=1.15,clip=true,angle=-0.32,seed=63})
work(faceD,{hand="scumble",pile=p_rockmid,coverage=0.9,clip=true,angle=0.14,seed=64})

--@ chunk 14
blend(faceA:grow(14),{angle=-0.6})
blend(faceB:grow(12),{angle=0.15})
blend(faceC:grow(12),{angle=-0.25})
blend(faceD:grow(12),{angle=0.2})
isleSpur=poly({{264,497},{277,475},{295,463},{317,455},{335,440},{355,431},{375,416},{393,410},{409,418},{428,423},{447,439},{466,450},{486,467},{500,487},{502,500},{264,500}},true)
work(isleSpur,{hand="body",pile=p_isle,coverage=2.3,fill=true,clip=true,seed=71})

--@ chunk 15
btL=brush{kind="rigger",width=5.2,point=0.95,stiffness=0.72}
bbL=brush{kind="rigger",width=3.1,point=0.96,stiffness=0.66}
bfL=brush{kind="rigger",width=1.7,point=0.96,stiffness=0.56}
local function twig(b,pts,p1,p2)
 b:reload(p_bark,0.85)
 b:stroke(pts,{pressure={p1,p2},ramps={0.02,0.12},orient="along",shake=0.3})
end
twig(btL,{{145,667},{150,639},{145,611},{154,583},{147,554},{158,528},{153,501},{166,478}},0.9,0.12)
twig(bbL,{{149,619},{127,596},{112,574},{92,561},{76,557}},0.75,0.12)
twig(bfL,{{111,574},{105,552},{91,536}},0.55,0.08)
twig(bfL,{{126,596},{111,594},{96,583}},0.52,0.08)
twig(bbL,{{147,590},{172,572},{186,551},{205,538}},0.66,0.1)
twig(bfL,{{186,551},{184,532},{194,520}},0.52,0.08)
twig(bbL,{{151,557},{130,538},{120,516},{122,496}},0.65,0.1)
twig(bfL,{{129,537},{109,528},{98,510}},0.5,0.08)
twig(bbL,{{158,529},{181,510},{191,490},{207,477}},0.58,0.08)
twig(bfL,{{190,492},{185,475},{191,459}},0.46,0.08)
twig(bbL,{{154,515},{139,490},{141,469},{152,448}},0.52,0.08)
twig(bfL,{{143,472},{128,459},{124,445}},0.44,0.07)
twig(bfL,{{166,481},{181,466},{184,448}},0.46,0.07)
p_figure=pile{{"raw umber",1.2},{"bone black",0.9},{"cobalt blue",0.35}}
cloak=poly({{644,503},{646,492},{647,486},{651,483},{655,486},{657,493},{662,502},{660,505},{645,505}})
work(cloak,{hand="detail",pile=p_figure,coverage=3.4,fill=true,clip=true,seed=81})
head=ellipse(652,480,3.2,3.6)
work(head,{hand="detail",pile=p_figure,coverage=3.2,fill=true,clip=true,seed=82})
bp=brush{kind="rigger",width=2.1,point=0.92,stiffness=0.7}
bp:load(p_figure,0.9)
bp:stroke({{648,487},{643,489},{640,495}},{pressure={0.75,0.18},ramps={0.03,0.08},orient="along"})
bp:reload(p_figure,0.7)
bp:stroke({{649,501},{648,507}},{pressure={0.72,0.1},ramps={0.02,0.16},orient="along"})
bp:reload(p_figure,0.7)
bp:stroke({{656,501},{658,506}},{pressure={0.68,0.1},ramps={0.02,0.16},orient="along"})

--@ chunk 16
for _,m in ipairs({faceA,faceB,faceC,faceD}) do
 work(m,{hand="body",pile=p_shore,coverage=3.2,fill=true,clip=true,seed=93})
end

--@ chunk 17
work(faceA:grow(14),{hand="body",pile=p_shore,coverage=3.4,fill=true,clip=true,seed=94})

--@ chunk 18
work(faceB:grow(12),{hand="body",pile=p_shore,coverage=3.2,fill=true,clip=true,seed=95})
work(faceC:grow(12),{hand="body",pile=p_shore,coverage=3.0,fill=true,clip=true,seed=96})
work(faceD:grow(12),{hand="body",pile=p_shore,coverage=3.0,fill=true,clip=true,seed=97})

--@ chunk 19
print(wait(12*60))

--@ chunk 20
p_reflect=pile{{"lead white",2.8},{"pale smalt",1.1},{"yellow ochre",0.18},{"cobalt blue",0.18}}
br=brush{kind="filbert",width=5.2,point=0.72,stiffness=0.56}
local function glint(pts,a,z,w)
 br:reload(p_reflect,0.72)
 br:stroke(pts,{pressure={a,z},ramps={0.05,0.2},orient="across",shake=0.3})
end
glint({{213,505},{219,506},{225,505}},0.6,0.25)
glint({{205,512},{213,513},{224,512},{231,513}},0.64,0.2)
glint({{198,520},{208,519},{220,521},{234,520}},0.58,0.18)
glint({{202,529},{214,530},{226,529},{242,530}},0.67,0.18)
glint({{193,539},{206,540},{219,539},{237,540},{247,539}},0.62,0.15)
glint({{197,550},{212,551},{229,550},{248,551}},0.65,0.15)
glint({{184,561},{201,562},{221,561},{243,562},{255,561}},0.62,0.14)
glint({{190,574},{207,575},{225,574},{252,575}},0.65,0.14)
glint({{180,586},{198,587},{219,586},{245,587},{263,586}},0.62,0.14)
glint({{187,599},{207,600},{229,599},{258,600}},0.68,0.15)
glint({{194,613},{212,614},{234,613},{257,614}},0.62,0.14)
glint({{204,627},{220,628},{241,627},{253,628}},0.58,0.12)

--@ chunk 21
blend(ellipse(218,568,48,66),{angle=0})

--@ chunk 22
blend(ellipse(220,564,72,78),{angle=0.2})

--@ chunk 23
p_ripple=pile{{"pale smalt",2.1},{"cobalt blue",0.42},{"raw umber",0.22},{"lead white",0.35}}
rr=brush{kind="filbert",width=3.1,point=0.72,stiffness=0.58}
local function ripple(pts,a)
 rr:reload(p_ripple,0.64)
 rr:stroke(pts,{pressure={a,0.12},ramps={0.04,0.18},orient="across",shake=0.2})
end
ripple({{175,513},{192,512},{207,513},{222,512},{242,513}},0.5)
ripple({{232,520},{250,521},{272,520},{286,521}},0.44)
ripple({{145,530},{163,531},{181,530},{197,531}},0.48)
ripple({{242,537},{260,536},{278,538},{304,537}},0.45)
ripple({{168,547},{184,548},{200,547},{216,548}},0.47)
ripple({{221,558},{239,557},{257,559},{282,558}},0.45)
ripple({{151,568},{171,569},{190,568},{211,569}},0.45)
ripple({{240,578},{261,579},{282,578},{302,579}},0.47)
ripple({{188,590},{207,589},{226,591},{248,590}},0.46)
ripple({{148,603},{168,604},{190,603},{207,604}},0.44)
ripple({{232,612},{248,611},{269,613},{288,612}},0.44)
ripple({{173,626},{192,627},{212,626},{230,627}},0.42)
ripple({{307,535},{330,536},{354,535},{375,536}},0.4)
ripple({{348,570},{370,569},{392,571},{415,570}},0.43)
ripple({{310,599},{330,600},{348,599}},0.4)
ripple({{431,550},{457,551},{481,550},{505,551}},0.42)
ripple({{454,586},{477,585},{500,587}},0.4)

--@ chunk 24
work(lakeM,{hand="scumble",pile=p_ripple,coverage=0.18,fill=false,clip=true,angle=0.03,angle_jitter=0.18,seed=126})

--@ chunk 25
waterArea=lakeM-shore
work(shore,{hand="body",pile=p_shore,coverage=3.4,fill=true,clip=true,seed=131})
work(waterArea,{hand="body",pile=p_water,coverage=3.2,fill=true,clip=true,angle=0.02,seed=132})

--@ chunk 26
work(waterArea,{hand="broad",pile=p_water,coverage=2.5,fill=true,clip=true,angle=0.03,seed=141})

--@ chunk 27
bg=brush{kind="rigger",width=3.4,point=0.95,stiffness=0.6}
local function lightdash(pts,press)
 bg:reload(p_reflect,0.8)
 bg:stroke(pts,{pressure={press,0.08},ramps={0.04,0.14},orient="across",shake=0.25})
end
lightdash({{214,507},{219,507},{225,506}},0.64)
lightdash({{198,518},{205,517},{211,518}},0.78)
lightdash({{225,528},{232,529},{243,528}},0.72)
lightdash({{189,544},{196,545},{207,544}},0.62)
lightdash({{217,557},{226,556},{238,557}},0.8)
lightdash({{202,579},{211,580},{224,579}},0.74)
lightdash({{233,596},{244,597},{255,596}},0.67)
lightdash({{197,611},{208,612},{220,611}},0.8)
lightdash({{229,628},{239,627},{250,628}},0.65)

--@ chunk 28
print(wait(7*60))

--@ chunk 29
broot=brush{kind="rigger",width=5.4,point=0.94,stiffness=0.75}
broot:load(p_bark,0.78)
broot:stroke({{922,482},{925,494},{929,507},{935,518}},{pressure={0.72,0.12},ramps={0.03,0.12},orient="along",shake=0.22})
p_fir=pile{{"raw umber",1},{"green earth",0.65},{"cobalt blue",0.4},{"bone black",0.35}}
bf2=brush{kind="rigger",width=2.2,point=0.96,stiffness=0.62}
local function firline(pts,sz)
 bf2:reload(p_fir,0.85)
 bf2:stroke(pts,{pressure={sz,0.1},ramps={0.02,0.12},orient="along",shake=0.18})
end
firline({{393,451},{394,414}},0.72)
firline({{393,423},{387,428}},0.62)
firline({{393,427},{401,431}},0.68)
firline({{394,430},{383,434}},0.62)
firline({{394,434},{405,438}},0.67)
firline({{394,438},{382,442}},0.61)
firline({{394,442},{403,446}},0.64)
firline({{394,446},{386,449}},0.58)
fline=brush{kind="rigger",width=1.5,point=0.95,stiffness=0.58}
fline:load(p_fir,0.75)
fline:stroke({{410,435},{411,418}},{pressure={0.63,0.08},ramps={0.03,0.1},orient="along"})
fline:reload(p_fir,0.7)
fline:stroke({{411,421},{407,426}},{pressure={0.52,0.06},ramps={0.02,0.1},orient="along"})
fline:reload(p_fir,0.7)
fline:stroke({{411,425},{416,429}},{pressure={0.58,0.06},ramps={0.02,0.1},orient="along"})
fline:reload(p_fir,0.7)
fline:stroke({{411,430},{406,433}},{pressure={0.48,0.06},ramps={0.02,0.1},orient="along"})
fline:reload(p_fir,0.7)
fline:stroke({{411,432},{415,435}},{pressure={0.5,0.06},ramps={0.02,0.1},orient="along"})

--@ chunk 30
p_litwater=pile{{"lead white",4.2},{"pale smalt",1.1},{"yellow ochre",0.22}}
bl=brush{kind="rigger",width=3.1,point=0.96,stiffness=0.58}
local function glimmer(pts,pres)
 bl:reload(p_litwater,0.78)
 bl:stroke(pts,{pressure={pres,0.06},ramps={0.03,0.12},orient="across",shake=0.2})
end
glimmer({{211,509},{219,510},{229,509}},0.77)
glimmer({{194,526},{202,525},{212,526}},0.64)
glimmer({{223,550},{232,551},{244,550}},0.72)
glimmer({{199,576},{209,577},{219,576}},0.7)
glimmer({{230,599},{240,598},{253,599}},0.75)
glimmer({{193,622},{204,623},{215,622}},0.66)
p_figedge=pile{{"lead white",0.7},{"yellow ochre",0.24},{"pale smalt",0.2},{"raw umber",0.2}}
fl=brush{kind="rigger",width=1.5,point=0.98,stiffness=0.5}
fl:load(p_figedge,0.55)
fl:stroke({{649,486},{649,491},{650,495}},{pressure={0.42,0.05},ramps={0.02,0.12},orient="along"})

--@ chunk 31
print(drying(220,570), drying(380,430), drying(900,430))

--@ chunk 32
print(wait(4*24*60))

--@ chunk 33
print(drying(220,570), drying(380,430), drying(900,430), drying(650,494))

--@ chunk 34
p_chapelDark=pile{{"bone black",1.1},{"raw umber",1.3},{"cobalt blue",0.35}}
p_chapelLight=pile{{"lead white",1.7},{"yellow ochre",0.38},{"raw umber",0.28},{"pale smalt",0.2}}
chapelSil=poly({{368,426},{377,422},{377,418},{380,412},{383,418},{383,422},{393,426},{393,434},{368,434}})
work(chapelSil,{hand="detail",pile=p_chapelDark,coverage=2.0,fill=true,clip=true,seed=204})
chapelWall=poly({{371,427},{378,424},{378,420},{380,416},{382,420},{382,425},{389,428},{389,432},{371,432}})
work(chapelWall,{hand="detail",pile=p_chapelLight,coverage=1.8,fill=true,clip=true,seed=205})
chapelShade=poly({{382,425},{389,428},{389,432},{382,431}})
work(chapelShade,{hand="detail",pile=p_chapelDark,coverage=1.2,fill=true,clip=true,seed=206})
chapelCross=brush{kind="rigger",width=1.15,point=0.96,stiffness=0.6}
chapelCross:load(p_chapelLight,0.62)
chapelCross:stroke({{380,412},{380,408}},{pressure={0.35,0.08},ramps={0.05,0.15},shake=0.15})
chapelCross:reload(p_chapelLight,0.52)
chapelCross:stroke({{378,410},{382,410}},{pressure={0.22,0.06},ramps={0.06,0.16},shake=0.18})

--@ chunk 35
p_glintfinal=pile{{"lead white",2.9},{"pale smalt",0.85},{"yellow ochre",0.32},{"cobalt blue",0.08}}
p_ripplefinal=pile{{"pale smalt",1.7},{"cobalt blue",0.38},{"raw umber",0.18},{"lead white",0.28},medium=0.12}
bw=brush{kind="filbert",width=4.2,point=0.78,stiffness=0.54}
local function waterstroke(p,pts,a,z)
 bw:reload(p,0.72)
 bw:stroke(pts,{pressure={a,z},ramps={0.06,0.19},orient="across",shake=0.24})
end
waterstroke(p_glintfinal,{{197,519},{203,518},{209,519}},0.56,0.16)
waterstroke(p_glintfinal,{{229,535},{239,534},{247,535}},0.64,0.12)
waterstroke(p_glintfinal,{{190,558},{199,559},{211,558}},0.67,0.12)
waterstroke(p_glintfinal,{{230,581},{242,580},{253,581}},0.59,0.1)
waterstroke(p_glintfinal,{{185,605},{194,604},{203,605}},0.63,0.12)
waterstroke(p_glintfinal,{{239,630},{251,631},{261,630}},0.58,0.1)
waterstroke(p_ripplefinal,{{127,544},{141,543},{155,544}},0.48,0.1)
waterstroke(p_ripplefinal,{{276,559},{288,560},{302,559}},0.47,0.08)
waterstroke(p_ripplefinal,{{333,587},{346,586},{358,587}},0.43,0.08)
waterstroke(p_ripplefinal,{{270,619},{283,618},{296,619}},0.42,0.08)

--@ chunk 36
p_waterquiet=pile{{"pale smalt",1.6},{"cobalt blue",0.38},{"lead white",0.55},{"raw umber",0.16},medium=0.1}
p_watersheen=pile{{"lead white",2.2},{"pale smalt",1.2},{"yellow ochre",0.12},{"cobalt blue",0.12},medium=0.05}
wq=brush{kind="filbert",width=3.2,point=0.82,stiffness=0.52}
local function ripplequiet(p,pts,press)
 wq:reload(p,0.48)
 wq:stroke(pts,{pressure={press,0.1},ramps={0.06,0.22},orient="across",shake=0.22})
end
ripplequiet(p_waterquiet,{{76,520},{84,519},{94,520},{104,519}},0.34)
ripplequiet(p_waterquiet,{{302,514},{313,515},{323,514}},0.31)
ripplequiet(p_watersheen,{{345,529},{353,528},{361,529},{369,528}},0.28)
ripplequiet(p_waterquiet,{{448,518},{457,519},{469,518}},0.29)
ripplequiet(p_watersheen,{{390,547},{399,548},{407,547}},0.25)
ripplequiet(p_waterquiet,{{328,579},{337,578},{347,579},{356,578}},0.3)
ripplequiet(p_waterquiet,{{421,601},{431,602},{443,601}},0.27)
ripplequiet(p_watersheen,{{373,616},{380,615},{387,616}},0.25)
