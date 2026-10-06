-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box hopper

--@ chunk 1
canvas{size=1080, aspect=10/7, linen={16,14}, seed=43, ground={{pile={{'lead white',18},{'yellow ochre',0.55},{'red earth',0.12}}, um=95, apply='knife', texture=0.12},{pile={{'lead white',20},{'yellow ochre',0.3}},um=42,apply='brush'}}}
-- Draw the street and the facade before committing to color.
graphite=pencil('HB')
graphite:rule({250,46},{1000,2},{pressure=0.25})
graphite:rule({250,506},{1000,579},{pressure=0.3})
graphite:rule({0,553},{1000,657},{pressure=0.28})
graphite:line({{244,53},{351,34},{351,521}}, {pressure=0.25})
graphite:line({{378,211},{816,224},{816,515},{378,487},{378,211}}, {pressure=0.3})
graphite:line({{847,225},{951,229},{951,550},{847,539},{847,225}}, {pressure=0.3})
graphite:line({{373,147},{818,158},{818,199},{373,188},{373,147}}, {pressure=0.3})
graphite:rule({570,218},{570,501},{pressure=0.2})
graphite:sketch({{674,344},{670,360},{675,374},{694,379},{700,406},{673,422}}, {pressure=0.4,passes=2,wander=1.2})
graphite:sketch({{677,380},{660,403},{644,417},{670,424},{684,434},{724,450},{721,490}}, {pressure=0.35,passes=2,wander=1.4})
graphite:rule({600,420},{751,427},{pressure=0.25})
fix()
-- The first palette: cool city air, oxidized brick, and cream stucco.
air=pile{{'lead white',10},{'cerulean blue',2},{'cobalt blue',0.65},{'yellow ochre',0.1},medium=0.03}
brickShade=pile{{'red earth',4},{'ultramarine blue',2},{'yellow ochre',0.7},{'lead white',1.7},{'bone black',0.35},medium=0.04}
sideShade=pile{{'yellow ochre',2.5},{'red earth',1},{'ultramarine blue',1.8},{'lead white',3.3},{'bone black',0.3},medium=0.03}
wallSun=pile{{'lead white',11},{'yellow ochre',2.3},{'pale cadmium',0.5},{'red earth',0.18},medium=0.02}
walkBase=pile{{'lead white',6},{'yellow ochre',1.5},{'bone black',0.65},{'ultramarine blue',0.45},{'red earth',0.3},medium=0.02}
roadBase=pile{{'lead white',3.3},{'bone black',1.1},{'ultramarine blue',0.8},{'red earth',0.5},medium=0.03}
function area(m,p,a,c) work(m,{hand='broad',pile=p,coverage=c or 2.8,angle=a or 0,clip=true,fill=true,pressure={0.62,0.82},dips={2,0.9,0.85},mix_jitter=0.035}) end
area(everywhere(),air,0,2.1)
area(poly{{0,73},{73,58},{233,104},{246,505},{0,536}},brickShade,0,3.1)
area(poly{{241,54},{351,34},{353,522},{247,510}},sideShade,1.57,3)
area(poly{{351,34},{1000,0},{1000,581},{350,521}},wallSun,0.02,3.2)
area(poly{{0,537},{245,508},{350,521},{1000,580},{1000,657},{0,563}},walkBase,0.08,3)
area(poly{{0,563},{1000,657},{1000,700},{0,700}},roadBase,0.13,3)
print('drawn and laid in')

--@ chunk 2
wait(2*24*60)
-- Bring the lean fields up to quieter, opaque body color.
function plane(m,p,a,c) work(m,{hand='body',tool='flat 16',pile=p,coverage=c or 3,angle=a or 0,clip=true,fill=true,length={35,95},pressure={0.72,0.9},dips={1,0.97,0.92},mix_jitter=0.025,angle_jitter=0.08,curve={0.15,0.1}}) end
wallBody=pile{{'lead white',14},{'yellow ochre',2.4},{'pale cadmium',0.8},{'red earth',0.17}}
brickBody=pile{{'red earth',4},{'ultramarine blue',2.5},{'bone black',0.8},{'lead white',0.85},{'yellow ochre',0.45}}
sideBody=pile{{'lead white',2.1},{'yellow ochre',2.1},{'ultramarine blue',2},{'red earth',0.8},{'bone black',0.6}}
plane(poly{{351,34},{1000,0},{1000,581},{350,521}},wallBody,0.035,3.2)
plane(poly{{0,73},{73,58},{233,104},{246,505},{0,536}},brickBody,0.025,3.1)
plane(poly{{241,54},{351,34},{353,522},{247,510}},sideBody,1.55,3)
shadowWalk=pile{{'lead white',2.7},{'ultramarine blue',2.3},{'bone black',1},{'red earth',0.5},{'yellow ochre',0.35}}
plane(poly{{0,536},{246,508},{348,521},{624,622},{1000,658},{1000,682},{0,594}},shadowWalk,0.1,3.3)
-- Deep openings, painted as holes before the interior is described.
opening=pile{{'bone black',3},{'ultramarine blue',2},{'viridian',1.4},{'yellow ochre',0.5}}
plane(poly{{368,201},{828,214},{828,530},{368,497}},opening,1.57,3.7)
plane(poly{{839,215},{958,219},{958,554},{839,544}},opening,1.57,3.6)
signDark=pile{{'red earth',2.7},{'bone black',1.7},{'ultramarine blue',0.8},{'yellow ochre',0.25}}
plane(poly{{370,142},{824,154},{824,198},{370,187}},signDark,0.025,3.4)
-- The window's quiet interior: shade at the left, a wedge of sun behind the sitter.
roomShade=pile{{'viridian',2},{'ultramarine blue',1.4},{'yellow ochre',1.4},{'bone black',1},{'lead white',1.4}}
roomLight=pile{{'lead white',6},{'yellow ochre',2.4},{'pale cadmium',0.7},{'red earth',0.25},{'bone black',0.12}}
roomFloor=pile{{'yellow ochre',3},{'red earth',1.4},{'lead white',1.7},{'bone black',0.8},{'ultramarine blue',0.6}}
plane(poly{{380,216},{814,228},{814,513},{380,485}},roomShade,1.57,3.3)
plane(poly{{583,223},{814,229},{814,476},{667,466},{583,436}},roomLight,0.025,3)
plane(poly{{380,456},{602,470},{814,476},{814,513},{380,485}},roomFloor,0.08,3)
print('second session: masses and openings')

--@ chunk 3
wait(3*24*60)
-- Dry-over-dry reconstruction of the shop interior.
plane(poly{{380,216},{814,228},{814,478},{380,457}},roomShade,1.52,3.3)
interiorOchre=pile{{'lead white',7},{'yellow ochre',2.6},{'pale cadmium',0.6},{'red earth',0.25},{'bone black',0.16}}
plane(poly{{641,224},{814,228},{814,477},{588,462}},interiorOchre,0.03,3.5)
plane(poly{{380,456},{605,469},{814,477},{814,513},{380,485}},roomFloor,0.08,3.2)
plane(poly{{372,145},{822,157},{822,195},{372,184}},signDark,0.028,3.3)
frameGreen=pile{{'viridian',2.6},{'bone black',2},{'yellow ochre',1.6},{'ultramarine blue',0.65},{'lead white',0.65}}
frameLight=pile{{'viridian',1.2},{'yellow ochre',1.1},{'lead white',3.1},{'bone black',0.7}}
function detail(m,p,a,c) work(m,{hand='detail',tool='flat 5',pile=p,angle=a or 0,coverage=c or 3.2,clip=true,fill=true,pressure={0.7,0.95},dips={1,0.94,0.94},length={6,18},angle_jitter=0.035,curve={0.08,0.05},mix_jitter=0.025}) end
-- Four separately drawn sides of the window and its one heavy mullion.
detail(poly{{368,202},{828,215},{819,225},{378,213}},frameGreen,0.025,3.2)
detail(poly{{368,202},{380,213},{380,487},{368,497}},frameGreen,1.57)
detail(poly{{815,222},{828,215},{828,530},{815,516}},frameGreen,1.57)
detail(poly{{379,484},{815,513},{828,530},{368,497}},frameGreen,0.08)
detail(poly{{565,219},{575,222},{575,501},{565,499}},frameGreen,1.57,3.4)
detail(poly{{574,225},{577,225},{577,498},{574,498}},frameLight,1.57,2.6)
-- Door recess and heavy rails.
plane(poly{{849,227},{950,231},{950,547},{849,537}},frameGreen,1.57,3.1)
plane(poly{{860,240},{938,242},{938,493},{860,486}},opening,1.57,3.1)
detail(poly{{858,270},{941,273},{941,283},{858,280}},frameGreen,0.03)
detail(poly{{858,487},{941,494},{941,537},{858,529}},frameGreen,0.07)
-- Sunlit ledge, its front, and the plinth.
sillTop=pile{{'lead white',9},{'yellow ochre',1.35},{'pale cadmium',0.55}}
sillFace=pile{{'lead white',5},{'yellow ochre',2.1},{'red earth',0.5},{'bone black',0.25}}
plinth=pile{{'lead white',3.9},{'red earth',1.5},{'yellow ochre',1.4},{'ultramarine blue',0.45},{'bone black',0.25}}
detail(poly{{363,496},{829,529},{839,534},{359,505}},sillTop,0.06,3.2)
detail(poly{{359,505},{839,534},{839,548},{359,518}},sillFace,0.065,3.4)
plane(poly{{350,520},{841,551},{841,565},{1000,579},{1000,590},{350,535}},plinth,0.05,3.4)
detail(poly{{847,539},{958,551},{966,558},{840,546}},sillTop,0.08)
-- Two cropped upper windows, each drawn independently.
upperTrim=pile{{'lead white',5},{'yellow ochre',1.2},{'bone black',0.55},{'ultramarine blue',0.45}}
upperGlass=pile{{'ultramarine blue',1.7},{'cerulean blue',1.2},{'bone black',1.3},{'lead white',1.5},{'yellow ochre',0.45}}
plane(poly{{442,26},{552,22},{552,117},{442,112}},upperTrim,1.55,3)
plane(poly{{453,29},{542,27},{542,105},{453,102}},opening,1.57,3.1)
plane(poly{{717,12},{831,8},{831,119},{717,114}},upperTrim,1.57,3)
plane(poly{{728,16},{820,13},{820,107},{728,105}},opening,1.57,3.1)
plane(poly{{460,31},{537,29},{537,62},{460,61}},upperGlass,0,3)
plane(poly{{735,18},{815,16},{815,62},{735,61}},upperGlass,0,3)
print('interior, joinery and upstairs')

--@ chunk 4
wait(2*24*60)
-- The building continues above the picture; the upstairs openings are cropped.
plane(poly{{351,0},{1000,0},{1000,41},{351,48}},wallBody,0.02,3.3)
plane(poly{{241,0},{351,0},{351,45},{241,63}},sideBody,1.57,3.1)
plane(poly{{442,0},{552,0},{552,117},{442,112}},upperTrim,1.57,3)
plane(poly{{453,0},{542,0},{542,105},{453,102}},opening,1.57,3.1)
plane(poly{{717,0},{831,0},{831,119},{717,114}},upperTrim,1.57,3)
plane(poly{{728,0},{820,0},{820,107},{728,105}},opening,1.57,3.1)
-- A final quiet coat over the sun wedge, now the underpaint is dry.
plane(poly{{641,224},{814,228},{814,477},{588,462}},interiorOchre,0.04,3.5)
plane(poly{{849,227},{950,231},{950,547},{849,537}},frameGreen,1.57,3.2)
plane(poly{{860,240},{938,242},{938,493},{860,486}},opening,1.57,3.3)
-- Shade below the shallow lintel and each protruding stone ledge.
stoneShade=pile{{'lead white',2},{'yellow ochre',1.3},{'ultramarine blue',0.8},{'bone black',0.7},{'red earth',0.35}}
detail(poly{{355,120},{1000,135},{1000,145},{355,129}},sillTop,0.03,3)
detail(poly{{355,129},{1000,145},{1000,150},{355,134}},stoneShade,0.03,3)
detail(poly{{365,190},{830,202},{834,213},{365,202}},stoneShade,0.03,3)
detail(poly{{361,517},{837,547},{837,553},{361,523}},stoneShade,0.06,3)
-- Brick building across the short alley: banded masonry and recesses.
brickBand=pile{{'red earth',3},{'ultramarine blue',1.4},{'bone black',0.65},{'lead white',1.6},{'yellow ochre',0.35}}
brickNear=pile{{'red earth',3.5},{'yellow ochre',1},{'ultramarine blue',1.7},{'bone black',0.5},{'lead white',0.55}}
plane(poly{{0,90},{73,77},{232,116},{232,129},{72,91},{0,104}},brickBand,0,2.9)
plane(poly{{0,279},{237,294},{238,311},{0,298}},brickBand,0.045,2.9)
plane(poly{{0,479},{243,469},{246,505},{0,533}},brickNear,0,2.9)
farDark=pile{{'bone black',2},{'ultramarine blue',1.5},{'red earth',1.2},{'lead white',0.4}}
-- None of these little recesses is copied from another.
plane(poly{{19,143},{60,141},{60,235},{19,235}},farDark,1.57,3.3)
plane(poly{{104,149},{150,155},{152,242},{105,240}},farDark,1.57,3.3)
plane(poly{{177,165},{214,173},{216,247},{178,244}},farDark,1.57,3.3)
plane(poly{{23,329},{63,327},{63,420},{23,423}},farDark,1.57,3.3)
plane(poly{{110,330},{156,333},{157,426},{111,429}},farDark,1.57,3.3)
plane(poly{{183,334},{218,339},{220,428},{184,431}},farDark,1.57,3.3)
plane(poly{{114,444},{158,443},{160,502},{115,507}},farDark,1.57,3.3)
-- One narrow side-wall opening, high above the pavement.
plane(poly{{273,139},{318,134},{319,271},{273,271}},stoneShade,1.57,3.1)
plane(poly{{280,145},{311,142},{312,263},{280,263}},farDark,1.57,3.3)
print('body coat set; street now has depth')

--@ chunk 5
print(drying(710,300),drying(420,300),drying(490,70),drying(900,400))
print(wait(4*24*60))
print(drying(710,300),drying(490,70))
-- Reloaded small brushwork, rather than more broad swirls.
work(poly{{641,224},{814,228},{814,477},{588,462}},{hand='body',tool={kind='filbert',width=12,stiffness=0.45},pile=interiorOchre,coverage=4.7,angle=0.035,clip=true,fill=true,length={18,48},pressure={0.74,0.94},dips={1,1,0.98},mix_jitter=0.025})
-- Baseboard and the long wall shelf in shadow.
woodDark=pile{{'yellow ochre',2},{'red earth',1.4},{'bone black',1.5},{'ultramarine blue',0.7},{'lead white',0.55}}
woodMid=pile{{'yellow ochre',3},{'red earth',0.8},{'lead white',1.5},{'bone black',0.45}}
detail(poly{{381,446},{563,456},{563,462},{381,451}},woodDark,0.05,3.3)
detail(poly{{583,458},{813,474},{813,480},{582,464}},woodDark,0.06,3.3)
-- Low shelving, and a handful of folded shirts.
plane(poly{{394,413},{531,420},{531,470},{394,462}},woodDark,0.045,3)
detail(poly{{390,410},{537,417},{537,424},{390,417}},woodMid,0.045,3.2)
clothGrey=pile{{'lead white',3.8},{'cerulean blue',0.7},{'bone black',0.4},{'yellow ochre',0.5}}
detail(poly{{409,396},{454,398},{454,409},{409,407}},clothGrey,0,3.2)
detail(poly{{414,390},{448,392},{448,397},{414,396}},upperTrim,0,3)
detail(poly{{476,402},{519,403},{518,416},{476,414}},clothGrey,0,3.2)
-- Hanging garment silhouettes, each drawn separately.
coatInk=pile{{'bone black',2.1},{'ultramarine blue',1.1},{'viridian',0.7},{'red earth',0.5},{'lead white',0.35}}
coatBrown=pile{{'red earth',1.5},{'yellow ochre',1.2},{'bone black',1.1},{'ultramarine blue',0.85},{'lead white',0.9}}
coat1=outline{{418,280,'c'},{431,273},{442,277},{454,289,'c'},{458,332},{451,333,'c'},{446,308},{445,386,'c'},{413,383,'c'},{415,307},{407,332,'c'},{399,329,'c'},{404,289,'c'},char='firm',amount=0.28,seed=71}
coat2=outline{{475,285,'c'},{488,281},{500,287},{510,302,'c'},{511,341},{504,340,'c'},{498,317},{499,397,'c'},{467,395,'c'},{470,312},{463,340,'c'},{456,336,'c'},{464,296,'c'},char='firm',amount=0.3,seed=78}
work(coat1:mask(),{hand='body',tool='filbert 5',pile=coatInk,coverage=3.6,angle=1.57,clip=true,fill=true,dips={1,0.95,0.9}})
work(coat2:mask(),{hand='body',tool='filbert 5',pile=coatBrown,coverage=3.6,angle=1.57,clip=true,fill=true,dips={1,0.95,0.9}})
print('garments and interior furniture blocking')

--@ chunk 6
wait(7*24*60)
function line(p,w,pts,pressure)
 local b=brush{kind='flat',width=w,stiffness=0.6,ragged=0.12}
 b:load(p,0.96)
 b:stroke(pts,{pressure={pressure or 0.82,pressure or 0.82},ramps={0.025,0.06},shake=0.15,orient='across'})
end
function smallshape(m,p,a,c)
 work(m,{hand='detail',tool={kind='filbert',width=3.4,stiffness=0.55},pile=p,angle=a or 1.57,coverage=c or 3.3,clip=true,fill=true,pressure={0.7,0.92},dips={1,0.96,0.93},mix_jitter=0.025})
end
chairDark=pile{{'bone black',1.5},{'viridian',1.3},{'yellow ochre',1.6},{'ultramarine blue',0.6},{'lead white',0.4}}
chairLit=pile{{'yellow ochre',2.5},{'lead white',2.1},{'viridian',0.55},{'bone black',0.5}}
-- The chair behind the sitter, angled into the room.
line(chairDark,4.2,{{727,368},{737,436},{744,499}},0.93)
line(chairDark,4,{{744,371},{751,440},{761,500}},0.9)
line(chairDark,5,{{727,369},{743,371}},0.9)
line(chairDark,4,{{731,398},{748,400}},0.85)
line(chairDark,4,{{733,418},{749,420}},0.85)
smallshape(poly{{681,429},{746,434},{753,442},{681,438}},chairDark,0.08,3.3)
line(chairDark,4,{{688,438},{681,493}},0.9)
line(chairLit,1.8,{{730,372},{739,433}},0.83)
line(chairLit,2.3,{{697,432},{745,436}},0.8)
-- An empty chair in the shaded pane.
line(chairDark,4,{{481,382},{483,445},{486,483}},0.9)
line(chairDark,3.6,{{500,385},{503,448},{508,484}},0.9)
line(chairDark,4,{{481,383},{500,385}},0.9)
line(chairDark,3.4,{{483,405},{501,408}},0.9)
smallshape(poly{{456,439},{503,443},{510,449},{460,446}},chairDark,0.08,3.3)
line(chairDark,3.7,{{461,445},{456,480}},0.9)
line(chairLit,1.7,{{486,386},{487,430}},0.8)
-- First silhouette of the woman: a blue skirt and oxide-red short-sleeved blouse.
skirt=pile{{'ultramarine blue',2.2},{'bone black',1.5},{'lead white',1},{'red earth',0.4}}
blouse=pile{{'red earth',3.7},{'deep cadmium',1.2},{'lead white',1.2},{'ultramarine blue',0.25}}
skin=pile{{'lead white',5},{'yellow ochre',1.2},{'red earth',0.55},{'deep cadmium',0.25}}
skinShade=pile{{'yellow ochre',1.5},{'red earth',1},{'lead white',1.4},{'ultramarine blue',0.2},{'bone black',0.15}}
hair=pile{{'bone black',2.5},{'burnt sienna',1.3},{'yellow ochre',0.65},{'ultramarine blue',0.35}}
skirtShape=outline{{679,419,'c'},{703,420},{711,435},{688,443},{670,450},{660,465,'c'},{642,462,'c'},{640,447},{650,436},{670,427},char='firm',amount=0.18,seed=85}
smallshape(skirtShape:mask(),skirt,0.18,3.7)
-- Calves and the two low, sensible shoes.
smallshape(poly{{645,458},{654,460},{651,483},{642,487},{638,484}},skinShade,1.57,3.3)
smallshape(poly{{657,461},{664,456},{669,484},{662,490},{655,486}},skin,1.4,3.3)
smallshape(outline{{638,482,'c'},{648,483},{650,490},{644,494,'c'},{621,495,'c'},{620,491},{630,488},char='firm',amount=0.15,seed=21}:mask(),hair,0,3.5)
smallshape(outline{{658,484,'c'},{667,484},{674,493},{673,498,'c'},{645,497,'c'},{642,494},{651,489},char='firm',amount=0.2,seed=29}:mask(),hair,0,3.5)
blouseShape=outline{{677,363,'c'},{689,362},{699,369},{708,385},{712,406},{705,424,'c'},{677,426,'c'},{672,406},{664,397,'c'},{654,400,'c'},{655,383},{667,370},char='firm',amount=0.25,seed=89}
smallshape(blouseShape:mask(),blouse,1.35,3.8)
-- Neck, crown and the lowered profile.
smallshape(poly{{681,353},{691,352},{692,367},{685,373},{678,365}},skinShade,1.57,3.6)
headHair=outline{{676,325},{686,321},{696,326},{701,336},{699,352},{692,360},{680,356},{674,344},char='firm',amount=0.15,seed=91}
smallshape(headHair:mask(),hair,1.1,3.7)
face=outline{{678,329,'c'},{686,330},{694,337},{693,353},{684,362},{675,357,'c'},{671,353},{671,347,'c'},{666,344,'c'},{671,337},{673,331},char='firm',amount=0.12,seed=95}
smallshape(face:mask(),skin,1.0,3.8)
-- Bare forearms: the near elbow turns toward the folded paper.
smallshape(outline{{697,384,'c'},{705,385},{714,403},{714,410,'c'},{690,417},{664,416,'c'},{660,413},{662,409},{686,410},{703,404},{698,395},char='firm',amount=0.15,seed=97}:mask(),skin,0.3,3.7)
smallshape(outline{{655,397,'c'},{666,400},{658,411},{643,418},{637,415,'c'},{638,411},{649,405},char='firm',amount=0.2,seed=100}:mask(),skinShade,0.3,3.5)
print('sitter blocked in')

--@ chunk 7
wait(10*24*60)
-- The small table under the woman's hands.
tableWood=pile{{'yellow ochre',2.8},{'red earth',1},{'bone black',1.1},{'lead white',1},{'ultramarine blue',0.3}}
tableTop=pile{{'lead white',3.8},{'yellow ochre',2.3},{'red earth',0.45},{'bone black',0.3}}
line(tableWood,4.5,{{604,423},{598,486}},0.94)
line(tableWood,4.7,{{704,431},{711,501}},0.94)
line(woodDark,3,{{719,426},{721,480}},0.92)
smallshape(poly{{598,413},{722,421},{707,427},{593,419}},tableTop,0.05,3.8)
smallshape(poly{{593,419},{707,427},{707,433},{593,424}},tableWood,0.06,3.5)
smallshape(poly{{707,427},{722,421},{722,427},{707,433}},woodDark,1.57,3.3)
line(sillTop,1.4,{{597,417},{685,423}},0.82)
paper=pile{{'lead white',7},{'yellow ochre',0.4},{'bone black',0.24},{'cerulean blue',0.1}}
smallshape(poly{{608,408},{649,410},{656,417},{613,416}},paper,0.1,4)
smallshape(poly{{613,416},{656,417},{655,420},{613,419}},clothGrey,0.1,3.3)
-- Body coats over the dry face and arm: a plain cheek, not a miniature portrait.
smallshape(face:mask(),skin,1.0,4.3)
smallshape(poly{{700,389},{706,391},{710,405},{704,410},{687,414},{665,414},{663,411},{687,411},{705,404}},skin,0.25,4.2)
-- The light strikes the right side of the skirt and sleeve.
skirtLit=pile{{'ultramarine blue',1.5},{'cobalt blue',0.5},{'lead white',2},{'bone black',0.6},{'red earth',0.18}}
blouseLit=pile{{'red earth',2.4},{'deep cadmium',1.1},{'lead white',2.1},{'yellow ochre',0.4}}
blouseShade=pile{{'red earth',2.6},{'ultramarine blue',0.55},{'bone black',0.5},{'yellow ochre',0.8},{'lead white',0.4}}
smallshape(poly{{690,367},{698,371},{707,385},{699,386},{693,379},{687,375}},blouseLit,1.1,3.5)
smallshape(poly{{666,376},{672,371},{680,378},{676,403},{670,409},{665,396},{655,399},{657,387}},blouseShade,1.35,3.4)
smallshape(poly{{682,429},{700,431},{685,440},{665,446},{651,458},{648,452},{656,441}},skirtLit,-0.4,3.5)
-- Hand-painted sign, eight separately drawn letters.
signLetter=pile{{'lead white',6.2},{'yellow ochre',0.8},{'pale cadmium',0.3}}
line(signLetter,3.7,{{439,157},{434,154},{418,154},{412,159},{411,171},{417,178},{435,179},{441,176}},0.94)
line(signLetter,3.8,{{460,155},{460,179},{487,180}},0.94)
line(signLetter,3.7,{{532,157},{506,157},{506,180},{533,181}},0.94)
line(signLetter,3.5,{{507,168},{527,169}},0.92)
line(signLetter,3.8,{{551,181},{565,158},{570,158},{582,183}},0.94)
line(signLetter,3.4,{{557,173},{577,174}},0.92)
line(signLetter,3.7,{{598,182},{598,159},{602,159},{628,183},{629,160}},0.94)
line(signLetter,3.7,{{673,161},{646,160},{646,183},{673,184}},0.94)
line(signLetter,3.5,{{647,172},{668,172}},0.92)
line(signLetter,3.7,{{691,184},{691,161},{710,162},{718,167},{716,172},{693,173}},0.94)
line(signLetter,3.8,{{706,173},{721,185}},0.94)
line(signLetter,3.7,{{769,165},{763,162},{746,162},{740,168},{745,173},{764,177},{770,182},{765,186},{744,185},{738,182}},0.94)
print('a table, paper, and the shop name')

--@ chunk 8
wait(14*24*60)
-- Calm the roadway and the band of pavement that catches the sun.
roadBody=pile{{'lead white',3.8},{'bone black',1.3},{'ultramarine blue',0.7},{'red earth',0.35},{'yellow ochre',0.25}}
pavementSun=pile{{'lead white',8},{'yellow ochre',1.5},{'bone black',0.55},{'ultramarine blue',0.25},{'red earth',0.18}}
plane(poly{{0,576},{1000,669},{1000,700},{0,700}},roadBody,0.08,3.6)
plane(poly{{356,536},{1000,591},{1000,652},{633,618}},pavementSun,0.1,3.5)
-- Upper glass, darkness, and curtains: the first swirls are covered, not preserved as detail.
upperNight=pile{{'bone black',2.4},{'ultramarine blue',1.3},{'yellow ochre',0.8},{'viridian',0.55},{'lead white',0.45}}
plane(poly{{453,0},{542,0},{542,105},{453,102}},upperNight,1.57,3.7)
plane(poly{{728,0},{820,0},{820,107},{728,105}},upperNight,1.57,3.7)
plane(poly{{860,240},{938,242},{938,493},{860,486}},upperNight,1.57,3.5)
plane(poly{{280,145},{311,142},{312,263},{280,263}},farDark,1.57,3.7)
-- The back building's stone caps and window sills.
farStone=pile{{'red earth',1.1},{'lead white',2.7},{'bone black',0.7},{'ultramarine blue',0.55},{'yellow ochre',0.5}}
detail(poly{{16,136},{63,134},{64,143},{16,145}},farStone,0,3)
detail(poly{{101,142},{153,148},{154,156},{101,150}},farStone,0.06,3)
detail(poly{{174,158},{216,166},{216,173},{174,166}},farStone,0.15,3)
detail(poly{{16,235},{64,235},{64,243},{16,244}},farStone,0,3)
detail(poly{{101,240},{155,243},{155,251},{101,248}},farStone,0.03,3)
detail(poly{{174,244},{220,247},{220,254},{174,252}},farStone,0.05,3)
detail(poly{{19,420},{67,417},{67,425},{19,429}},farStone,-0.03,3)
detail(poly{{106,429},{162,425},{162,433},{106,437}},farStone,-0.03,3)
detail(poly{{180,432},{224,428},{224,436},{180,441}},farStone,-0.035,3)
-- Light mortar is intermittent, kept subordinate to the building as a whole.
mortar=pile{{'red earth',2.2},{'ultramarine blue',1.1},{'lead white',1.8},{'bone black',0.5},{'yellow ochre',0.5}}
line(mortar,0.8,{{2,115},{78,113},{123,121}},0.72)
line(mortar,0.7,{{162,135},{228,147}},0.75)
line(mortar,0.9,{{4,265},{86,267}},0.72)
line(mortar,0.7,{{102,270},{181,276},{229,281}},0.75)
line(mortar,0.9,{{1,319},{86,319}},0.72)
line(mortar,0.8,{{67,452},{111,450}},0.72)
line(mortar,0.9,{{171,456},{232,451}},0.75)
line(mortar,0.65,{{80,115},{80,132}},0.7)
line(mortar,0.65,{{174,137},{174,154}},0.7)
line(mortar,0.65,{{94,269},{94,282}},0.7)
-- Shop door's inset lower panel.
smallshape(poly{{868,504},{932,510},{932,532},{868,526}},frameGreen,1.57,3.5)
line(frameLight,1.2,{{865,497},{935,503},{935,536}},0.8)
print('architecture and foreground brought forward')

--@ chunk 9
wait(12*24*60)
-- Restore warmth and volume to the figure before small facial marks.
fleshMiddle=pile{{'lead white',3.6},{'yellow ochre',1.35},{'red earth',0.65},{'deep cadmium',0.1}}
fleshLight=pile{{'lead white',6},{'yellow ochre',0.8},{'red earth',0.3}}
fleshShadow=pile{{'lead white',1.7},{'yellow ochre',1.4},{'red earth',0.8},{'ultramarine blue',0.25},{'bone black',0.1}}
smallshape(face:mask(),fleshMiddle,1.0,4.1)
smallshape(poly{{699,386},{705,387},{713,403},{713,409},{690,417},{665,415},{662,412},{664,409},{686,410},{704,404}},fleshMiddle,0.2,4)
smallshape(poly{{656,399},{664,401},{657,410},{643,416},{639,414},{649,406}},fleshShadow,0.2,3.7)
smallshape(poly{{681,357},{691,357},{691,366},{685,373},{678,365}},fleshShadow,1.57,3.5)
-- Clear glass and slack curtains in the upstairs rooms.
glassSlate=pile{{'lead white',2},{'cerulean blue',1.5},{'cobalt blue',1.5},{'bone black',0.9},{'yellow ochre',0.25}}
curtain=pile{{'lead white',4.6},{'yellow ochre',0.7},{'bone black',0.45},{'cobalt blue',0.2}}
plane(poly{{483,0},{537,0},{537,55},{483,55}},glassSlate,1.52,3.1)
plane(poly{{734,0},{791,0},{790,56},{734,55}},glassSlate,1.52,3.1)
smallshape(outline{{458,-4,'c'},{483,-4,'c'},{484,37},{481,70},{479,94,'c'},{472,96},{459,91,'c'},char='firm',amount=0.25,seed=109}:mask(),curtain,1.57,3.6)
smallshape(outline{{797,-4,'c'},{817,-4,'c'},{816,49},{814,94,'c'},{807,99},{800,95,'c'},{800,58},char='firm',amount=0.23,seed=110}:mask(),curtain,1.57,3.6)
-- Curbstone through the shadow, separate planes for its top and its face.
curbFace=pile{{'lead white',2},{'bone black',1.25},{'ultramarine blue',1.3},{'red earth',0.35},{'yellow ochre',0.3}}
curbTop=pile{{'lead white',3.8},{'ultramarine blue',1.1},{'bone black',0.7},{'yellow ochre',0.35}}
roadShadow=pile{{'lead white',2.4},{'ultramarine blue',1.55},{'bone black',1.15},{'red earth',0.32}}
plane(poly{{0,578},{1000,669},{1000,683},{0,596}},roadShadow,0.09,3.2)
detail(poly{{0,564},{1000,657},{1000,669},{0,578}},curbFace,0.09,3.6)
detail(poly{{0,560},{1000,652},{1000,657},{0,564}},curbTop,0.09,3.5)
-- The recession around the green door and its threshold.
recess=pile{{'bone black',2},{'yellow ochre',1},{'ultramarine blue',0.9},{'red earth',0.5}}
detail(poly{{839,218},{848,226},{848,538},{839,544}},recess,1.57,3.5)
detail(poly{{950,231},{958,220},{958,554},{950,547}},stoneShade,1.57,3.4)
smallshape(poly{{848,540},{955,552},{965,560},{842,549}},stoneShade,0.09,3.6)
-- Cover an unintended rounded highlight; draw the panel one straight side at a time later.
smallshape(poly{{858,488},{939,495},{939,540},{858,532}},frameGreen,1.57,3.9)
-- A dim reflection from the far wall in the door glass.
doorReflection=pile{{'lead white',1.6},{'cerulean blue',1},{'bone black',1.1},{'viridian',0.5},{'yellow ochre',0.25}}
plane(poly{{864,288},{883,288},{879,473},{866,469}},doorReflection,1.57,2.8)
print('warm figure, upstairs curtains, curb and door depth')

--@ chunk 10
wait(7*24*60)
-- The sitter, reduced to a handful of solid planes.
smallshape(poly{{686,336},{692,340},{692,351},{685,360},{678,357},{684,352},{684,345}},fleshShadow,1.2,3.6)
smallshape(poly{{674,335},{681,334},{685,340},{682,348},{675,351},{671,347},{671,343}},fleshLight,1.0,3.5)
smallshape(poly{{681,360},{687,359},{689,366},{684,370},{681,366}},fleshMiddle,1.57,3.3)
hairBrown=pile{{'burnt sienna',2},{'yellow ochre',1},{'bone black',1.2},{'lead white',0.75}}
smallshape(poly{{692,327},{697,332},{700,342},{698,352},{692,358},{691,350},{694,341}},hairBrown,1.57,3.5)
feature=pile{{'yellow ochre',1.2},{'red earth',1},{'bone black',0.65},{'lead white',0.4}}
line(feature,0.9,{{673,340},{679,341}},0.8)
line(feature,0.7,{{673,343},{676,344}},0.86)
line(feature,0.8,{{672,352},{677,353}},0.86)
line(fleshLight,1,{{676,357},{681,358}},0.8)
smallshape(ellipse(690,346,1.9,3.4),fleshMiddle,1.57,3)
line(hairBrown,1.5,{{679,325},{686,325},{691,329}},0.86)
line(hair,1.2,{{676,329},{674,333}},0.88)
-- Oxide red folds, with a little warm light across the chest.
smallshape(poly{{682,376},{688,381},{693,397},{690,406},{680,402},{678,389}},blouseLit,1.25,3.5)
smallshape(poly{{675,367},{683,372},{680,380},{674,373}},blouseShade,1.2,3.3)
smallshape(poly{{691,367},{698,372},{695,379},{688,375}},blouseLit,1.2,3.3)
line(blouseShade,1.3,{{683,387},{686,397},{690,403}},0.86)
line(blouseLit,1.4,{{670,381},{667,391}},0.87)
smallshape(ellipse(685,389,0.7,0.8),sillFace,0,2.8)
smallshape(ellipse(687,400,0.65,0.75),sillFace,0,2.8)
line(fleshLight,1.8,{{702,391},{708,402}},0.88)
line(fleshLight,1.7,{{669,412},{688,414},{705,407}},0.87)
line(fleshShadow,1,{{687,417},{705,412}},0.9)
line(fleshLight,1.3,{{641,414},{646,412}},0.88)
line(fleshShadow,0.65,{{663,411},{667,413}},0.86)
line(fleshShadow,0.65,{{664,414},{668,415}},0.86)
-- Skirt pleats and the two ankles catch much less light.
line(skirt,2.2,{{674,438},{655,451},{651,461}},0.89)
line(skirtLit,1.9,{{692,435},{675,442},{662,450}},0.88)
line(skirt,1.5,{{702,437},{684,443}},0.9)
line(fleshLight,2.2,{{660,470},{664,484}},0.84)
line(fleshShadow,1.8,{{644,470},{642,483}},0.84)
line(hairBrown,1.8,{{626,491},{638,490}},0.85)
line(hairBrown,1.5,{{650,494},{665,493}},0.85)
-- Quiet shadows where furniture and shoes meet the floor.
smallshape(poly{{618,496},{644,497},{650,500},{620,500},{613,498}},woodDark,0.05,3.1)
smallshape(poly{{646,498},{675,499},{685,501},{754,504},{747,507},{665,503},{641,501}},woodDark,0.08,2.6)
-- A municipal hydrant gives scale to the empty pavement, entirely in shade.
hydrant=pile{{'bone black',1.8},{'viridian',1},{'yellow ochre',1.6},{'ultramarine blue',0.55},{'lead white',0.7}}
hydrantLight=pile{{'lead white',2.6},{'yellow ochre',1.8},{'viridian',0.7},{'bone black',0.85},{'ultramarine blue',0.2}}
smallshape(poly{{189,550},{204,552},{197,558},{148,563},{143,559},{174,551}},curbFace,0.1,3)
hydrantBody=outline{{191,523,'c'},{196,516},{201,518},{205,524,'c'},{204,545},{209,548,'c'},{209,554,'c'},{186,552,'c'},{187,547},{191,544},char='firm',amount=0.3,seed=123}
smallshape(hydrantBody:mask(),hydrant,1.57,3.7)
smallshape(ellipse(197,523,10,4),hydrant,0,3.5)
smallshape(ellipse(187,536,5,6.5),hydrant,1.57,3.5)
smallshape(ellipse(207,537,4.7,6),hydrant,1.57,3.5)
line(hydrantLight,2.6,{{198,526},{199,545}},0.84)
line(hydrantLight,2.1,{{190,522},{201,523}},0.84)
line(hydrantLight,1.6,{{187,532},{189,538}},0.86)
print('the human planes; a hydrant in shade')

--@ chunk 11
-- Fuse only selected wet joins in the face and forearm; keep the profile and eye sharp.
print(drying(683,345))
blend(poly{{679,333},{689,338},{689,352},{681,357},{677,354},{681,347}},{tool='badger 5',coverage=0.85,angle=1.15,length={5,10},pressure={0.18,0.3},clip=true})
blend(poly{{668,409},{687,410},{706,402},{711,408},{690,417},{668,416}},{tool='badger 4',coverage=0.8,angle=0.2,length={5,10},pressure={0.16,0.3},clip=true})
blend(poly{{680,378},{690,381},{695,404},{680,405}},{tool='badger 4',coverage=0.55,angle=1.25,length={4,8},pressure={0.16,0.27},clip=true})
-- Keep straight joinery straight: each sash rail is its own short stroke.
sash=pile{{'lead white',4.4},{'yellow ochre',1.2},{'bone black',0.8},{'ultramarine blue',0.15}}
line(sash,3,{{453,57},{541,60}},0.93)
line(sash,2.7,{{498,0},{498,57}},0.94)
line(sash,2.7,{{498,61},{498,103}},0.94)
line(sash,3.1,{{728,58},{820,61}},0.94)
line(sash,2.8,{{774,0},{774,58}},0.94)
line(sash,2.8,{{774,62},{774,106}},0.94)
line(sillTop,2.1,{{440,111},{551,117}},0.9)
line(sillTop,2.1,{{716,114},{831,120}},0.9)
-- The transom, small building number, and handle.
line(frameGreen,5,{{858,270},{940,273}},0.94)
line(frameLight,1.8,{{859,230},{859,270}},0.87)
line(frameLight,1.5,{{849,229},{849,534}},0.83)
line(frameLight,1.3,{{867,501},{932,507}},0.87)
line(frameLight,1.2,{{932,507},{932,530}},0.87)
line(frameLight,1.3,{{867,501},{867,524}},0.87)
line(frameLight,1.2,{{868,525},{932,532}},0.87)
-- Each digit is drawn independently in the old glass.
line(signLetter,1.3,{{878,251},{882,249},{887,251},{887,255},{879,262},{888,263}},0.88)
line(signLetter,1.3,{{895,251},{901,250},{904,253},{900,256},{904,260},{902,264},{895,263}},0.88)
line(signLetter,1.3,{{921,251},{913,251},{913,257},{919,257},{922,260},{920,265},{913,264}},0.88)
brassShade=pile{{'yellow ochre',2.3},{'bone black',0.8},{'red earth',0.6},{'lead white',0.4}}
brass=pile{{'yellow ochre',2},{'pale cadmium',1.2},{'lead white',1.6},{'bone black',0.3}}
smallshape(ellipse(925,407,3.3,9),brassShade,1.57,3.4)
line(brass,2.6,{{928,398},{928,414}},0.9)
line(sillTop,1.1,{{929,399},{929,407}},0.87)
-- A square little opening-hours card, with its marks left unreadable.
smallshape(poly{{889,345},{918,347},{918,367},{889,365}},clothGrey,0.05,3.3)
line(upperNight,0.7,{{894,353},{914,354}},0.82)
line(upperNight,0.65,{{894,357},{912,358}},0.8)
line(upperNight,0.6,{{898,361},{909,362}},0.8)
print('fused figure planes and completed shop joinery')

--@ chunk 12
wait(8*24*60)
-- Blue air above the shaded tenement.
skyBlue=pile{{'lead white',5.8},{'cerulean blue',3},{'cobalt blue',0.9},{'yellow ochre',0.06}}
plane(poly{{0,0},{241,0},{241,98},{232,104},{73,58},{0,73}},skyBlue,0.01,3.5)
-- Bevel of the corner and quiet, brushed variations in the cream stone.
cornerStone=pile{{'lead white',6.5},{'yellow ochre',1.5},{'red earth',0.16},{'bone black',0.2}}
plane(poly{{351,0},{361,0},{361,523},{351,521}},cornerStone,1.57,3.1)
wallQuiet=pile{{'lead white',9},{'yellow ochre',1.65},{'red earth',0.13},{'bone black',0.09}}
work(poly{{575,0},{665,0},{692,129},{575,127}}:soften(10),{hand='scumble',pile=wallQuiet,tool='filbert 12',angle=1.54,coverage=0.42,length={14,32},clip=true,load=0.24,pressure={0.16,0.34},dips={3,0.24,0.9}})
work(poly{{969,159},{1000,154},{1000,546},{972,541}}:soften(8),{hand='scumble',pile=wallQuiet,tool='filbert 10',angle=1.56,coverage=0.44,length={12,25},clip=true,load=0.24,pressure={0.16,0.34},dips={3,0.24,0.9}})
-- Little upper panes, reflecting the sky from a much darker building.
farGlass=pile{{'lead white',1.3},{'cerulean blue',0.9},{'ultramarine blue',0.8},{'bone black',0.9},{'red earth',0.4}}
smallshape(poly{{25,152},{53,150},{53,185},{25,186}},farGlass,1.57,3.4)
smallshape(poly{{112,161},{145,165},{146,195},{112,193}},farGlass,1.57,3.4)
smallshape(poly{{184,181},{209,187},{210,214},{184,211}},farGlass,1.57,3.4)
smallshape(poly{{28,336},{57,334},{57,369},{28,371}},farGlass,1.57,3.4)
smallshape(poly{{117,338},{150,341},{151,375},{117,374}},farGlass,1.57,3.4)
smallshape(poly{{189,345},{213,348},{214,379},{189,379}},farGlass,1.57,3.4)
-- A dark slit, not a white outline, between the two buildings.
smallshape(poly{{234,114},{240,97},{244,503},{242,506}},upperNight,1.57,3.1)
-- Masonry joints at the foot of the shop.
jointSun=pile{{'lead white',2.3},{'yellow ochre',1.1},{'bone black',0.7},{'ultramarine blue',0.3},{'red earth',0.2}}
line(jointSun,0.85,{{405,523},{405,537}},0.85)
line(jointSun,0.85,{{587,537},{587,550}},0.85)
line(jointSun,0.85,{{779,550},{779,563}},0.85)
line(jointSun,0.8,{{978,579},{978,587}},0.85)
-- Sidewalk seams converge quietly into the empty foreground.
line(jointSun,0.95,{{528,551},{608,616}},0.8)
line(jointSun,0.9,{{805,578},{881,640}},0.8)
line(curbFace,0.9,{{185,523},{147,572}},0.8)
line(curbFace,0.95,{{344,523},{348,591}},0.8)
line(upperNight,0.85,{{218,583},{218,597}},0.82)
line(upperNight,0.85,{{580,615},{579,632}},0.82)
line(upperNight,0.8,{{859,642},{857,655}},0.82)
-- A drain grate at the curb; only a few bars are visible in its darkness.
smallshape(poly{{85,587},{153,594},{152,601},{84,594}},upperNight,0.1,3.2)
line(curbTop,1.2,{{91,589},{90,594}},0.85)
line(curbTop,1.1,{{104,591},{103,596}},0.85)
line(curbTop,1.2,{{117,592},{116,597}},0.85)
line(curbTop,1.1,{{130,594},{129,599}},0.85)
line(curbTop,1.1,{{144,595},{142,600}},0.85)
print('quiet stone, blue sky and sidewalk seams')

--@ chunk 13
-- The clothes are on hangers, not disembodied figures.
metal=pile{{'lead white',1.8},{'bone black',0.9},{'cerulean blue',0.6},{'yellow ochre',0.4}}
line(woodDark,2.4,{{395,266},{527,272}},0.92)
line(metal,1,{{398,265},{523,271}},0.85)
line(metal,1,{{428,270},{430,264},{434,263},{436,266},{433,273}},0.86)
line(metal,1.05,{{415,282},{432,273}},0.87)
line(metal,1,{{432,273},{446,282}},0.87)
line(metal,1.05,{{416,282},{446,283}},0.87)
line(metal,1,{{483,277},{485,272},{489,272},{490,275},{487,280}},0.86)
line(metal,1,{{470,286},{487,280}},0.86)
line(metal,1,{{487,280},{503,289}},0.86)
line(metal,1.05,{{470,287},{503,289}},0.87)
coatBlueLight=pile{{'ultramarine blue',1.5},{'bone black',0.95},{'lead white',1.5},{'cerulean blue',0.35},{'red earth',0.18}}
coatBlueShadow=pile{{'ultramarine blue',1.5},{'bone black',1.8},{'yellow ochre',0.35}}
coatWarmLight=pile{{'red earth',1.8},{'yellow ochre',1.2},{'bone black',0.7},{'lead white',1.5},{'ultramarine blue',0.45}}
smallshape(poly{{417,283},{427,277},{432,301},{425,310}},coatBlueLight,1.3,3.1)
smallshape(poly{{435,279},{442,284},{436,302},{432,306}},coatBlueLight,1.55,3.1)
smallshape(poly{{440,309},{444,312},{443,381},{436,381}},coatBlueLight,1.57,2.8)
line(coatBlueShadow,1.5,{{430,307},{432,379}},0.87)
line(coatBlueLight,1.8,{{412,293},{406,324}},0.86)
line(coatBlueShadow,1.5,{{448,294},{453,324}},0.86)
smallshape(ellipse(432,322,0.75,0.8),metal,0,2.7)
smallshape(ellipse(433,342,0.7,0.85),metal,0,2.7)
smallshape(ellipse(433,362,0.75,0.8),metal,0,2.7)
smallshape(poly{{475,289},{484,283},{488,307},{480,316}},coatWarmLight,1.4,3.1)
smallshape(poly{{493,287},{499,292},{491,309},{488,312}},coatWarmLight,1.4,3.1)
smallshape(poly{{489,317},{495,316},{495,394},{487,393}},coatWarmLight,1.57,2.9)
line(woodDark,1.3,{{483,315},{482,390}},0.88)
line(coatWarmLight,1.6,{{505,304},{506,334}},0.86)
line(woodDark,1.4,{{469,311},{462,335}},0.86)
-- Cabinet fronts and the creases in folded shirts.
line(coatBrown,1.8,{{441,425},{441,462}},0.9)
line(coatBrown,1.6,{{487,427},{487,465}},0.9)
line(woodMid,1.2,{{403,420},{403,457}},0.85)
line(metal,1.3,{{427,439},{430,439}},0.89)
line(metal,1.3,{{472,442},{476,442}},0.89)
line(clothGrey,0.8,{{414,401},{451,403}},0.85)
line(upperTrim,0.9,{{476,410},{515,412}},0.85)
line(paper,0.9,{{418,392},{446,394}},0.85)
-- The window's bevels and a few reflections, kept off the figure.
frameBevel=pile{{'viridian',1.5},{'yellow ochre',1.4},{'lead white',2},{'bone black',0.85}}
line(frameBevel,2.0,{{378,205},{597,211}},0.86)
line(frameBevel,1.9,{{599,211},{816,218}},0.86)
line(opening,1.4,{{378,215},{558,220}},0.9)
line(opening,1.4,{{578,224},{814,231}},0.9)
line(frameBevel,1.7,{{822,237},{822,506}},0.86)
line(opening,1.8,{{373,226},{373,480}},0.87)
line(frameBevel,1.6,{{387,490},{559,501}},0.87)
line(frameBevel,1.6,{{584,503},{813,519}},0.87)
line(frameBevel,1.4,{{574,235},{574,490}},0.86)
reflectionCool=pile{{'lead white',2.8},{'cerulean blue',1.2},{'bone black',0.65},{'viridian',0.35},{'yellow ochre',0.2},medium=0.38}
work(poly{{384,225},{496,228},{473,244},{384,246}}:soften(2),{hand='glaze',tool='filbert 10',pile=reflectionCool,coverage=0.65,angle=0.03,length={30,70},clip=true,load=0.35,pressure={0.16,0.26},dips={2,0.35,0.92}})
reflectionWarm=pile{{'lead white',6},{'cerulean blue',0.6},{'yellow ochre',0.45},{'bone black',0.18},medium=0.3}
work(poly{{800,237},{810,238},{810,304},{803,315}}:soften(1.4),{hand='glaze',tool='filbert 4',pile=reflectionWarm,coverage=0.8,angle=1.57,length={14,28},clip=true,load=0.38,pressure={0.16,0.3},dips={2,0.38,0.92}})
line(metal,0.95,{{385,273},{385,344}},0.83)
line(reflectionWarm,0.85,{{805,352},{805,420}},0.86)
-- The sign's thin board edges catch light, not the entire wall.
line(sillFace,1.2,{{373,145},{612,151}},0.9)
line(sillFace,1.1,{{616,152},{821,157}},0.9)
line(recess,1.4,{{374,185},{823,197}},0.87)
-- Erase the last extraneous glint by painting the door's broad right stile.
smallshape(poly{{940,486},{949,487},{949,542},{940,541}},frameGreen,1.57,3.8)
print('hangers, cloth folds, and glass')

--@ chunk 14
wait(14*24*60)
-- Paint away the unsuccessful, lacy films at the tops of the two panes.
plane(poly{{381,222},{555,227},{555,259},{381,255}},roomShade,0.03,4)
plane(poly{{786,233},{813,234},{813,331},{793,328}},interiorOchre,1.57,4.1)
-- Glass is now a quiet, very small cool plane and two isolated glints.
glassQuiet=pile{{'lead white',1.6},{'cerulean blue',0.8},{'viridian',1.15},{'yellow ochre',1.15},{'bone black',0.65},{'ultramarine blue',0.5}}
smallshape(poly{{384,229},{454,231},{437,238},{384,235}}:soften(1.5),glassQuiet,0.03,2.4)
-- Newspaper, with no legible headline.
newsInk=pile{{'lead white',1.8},{'bone black',0.8},{'ultramarine blue',0.3},{'yellow ochre',0.3}}
line(newsInk,0.5,{{616,412},{628,413}},0.75)
line(newsInk,0.45,{{632,413},{643,414}},0.75)
line(newsInk,0.45,{{618,414},{630,415}},0.75)
line(newsInk,0.4,{{634,415},{648,416}},0.75)
line(clothGrey,0.65,{{633,410},{638,417}},0.8)
-- Softer, partly lost contour on the dark garments.
lose(coat1:mask(),{pile=roomShade,tool='filbert 2',where={found=0.72,soft=0.28,period=31,seed=29},reach={0.8,1.25},load=0.16,pressure={0.28,0.07},every=2.2})
lose(coat2:mask(),{pile=roomShade,tool='filbert 2',where={found=0.76,soft=0.24,period=27,seed=35},reach={0.8,1.2},load=0.16,pressure={0.27,0.07},every=2.4})
line(chairLit,1.4,{{459,442},{502,446}},0.85)
line(chairLit,1.2,{{746,382},{751,423}},0.85)
line(chairLit,1.2,{{754,451},{758,480}},0.85)
-- Frame bars in the shaded tenement, each drawn to its own perspective.
farFrame=pile{{'lead white',1.1},{'yellow ochre',1},{'red earth',0.7},{'bone black',1.2},{'ultramarine blue',0.8}}
line(farFrame,1.6,{{39,151},{39,232}},0.89)
line(farFrame,2,{{23,188},{58,188}},0.9)
line(farFrame,1.6,{{128,163},{129,239}},0.9)
line(farFrame,1.9,{{108,197},{150,199}},0.9)
line(farFrame,1.5,{{197,184},{198,243}},0.9)
line(farFrame,1.9,{{181,216},{214,219}},0.9)
line(farFrame,1.6,{{42,335},{42,418}},0.9)
line(farFrame,1.9,{{26,373},{61,371}},0.9)
line(farFrame,1.6,{{134,340},{135,423}},0.9)
line(farFrame,2,{{114,377},{155,378}},0.9)
line(farFrame,1.4,{{201,347},{202,427}},0.9)
line(farFrame,1.8,{{187,382},{217,383}},0.9)
-- The little side sash has a single slate upper pane.
smallshape(poly{{284,147},{307,145},{308,200},{284,201}},farGlass,1.57,3.3)
line(farFrame,2.6,{{280,205},{312,206}},0.91)
line(farFrame,1.7,{{296,147},{296,261}},0.89)
line(stoneShade,1.6,{{273,272},{319,272}},0.87)
print('quiet reflections and final small structural details')

--@ chunk 15
wait(10*24*60)
-- Four small lower panes upstairs: leave the sash intact but calm the accidental pale scratches.
paneShade=pile{{'bone black',2},{'ultramarine blue',1.5},{'yellow ochre',0.8},{'viridian',0.45},{'lead white',0.55},medium=0.04}
local pane1=poly{{487,64},{495,64},{495,101},{487,101}}
local pane2=poly{{501,64},{537,65},{537,101},{501,100}}
local pane3=poly{{735,65},{771,65},{771,103},{735,102}}
local pane4=poly{{778,66},{796,66},{796,104},{778,104}}
for _,m in ipairs{pane1,pane2,pane3,pane4} do
 work(m,{hand='body',tool={kind='filbert',width=5,stiffness=0.35},pile=paneShade,coverage=4.5,length={8,20},angle=1.57,clip=true,fill=true,pressure={0.78,0.95},dips={1,1,0.98},mix_jitter=0.02})
end
-- A small warm correction to the ankle and a gentler cheek plane.
line(fleshMiddle,1.6,{{661,473},{664,483}},0.9)
work(poly{{675,335},{681,335},{683,342},{678,350},{674,348}}:soften(0.7),{hand='scumble',tool='filbert 2',pile=fleshMiddle,coverage=0.6,angle=1.1,clip=true,pressure={0.35,0.5},load=0.45,dips={2,0.45,0.92}})
-- Resettle the glass glints after the body corrections. Only three fragments.
line(reflectionWarm,0.7,{{807,260},{807,292}},0.9)
line(metal,0.7,{{386,394},{386,425}},0.86)
line(frameBevel,1.1,{{571,233},{571,276}},0.88)
-- Broken lights across the outer sill retain the trace of the hand.
line(sillTop,1.5,{{365,500},{508,509}},0.9)
line(sillTop,1.4,{{541,511},{676,520}},0.9)
line(sillTop,1.4,{{704,522},{825,531}},0.9)
print('final restrained corrections')

--@ chunk 16
-- The late scumble broke too brightly on the cheek. Restore one calm, full-bodied flesh plane.
faceBaseFinal=pile{{'lead white',4.2},{'yellow ochre',1.6},{'red earth',0.65},medium=0.035}
work(face:mask(),{hand='detail',tool={kind='filbert',width=2.8,stiffness=0.32},pile=faceBaseFinal,coverage=5,angle=1.1,clip=true,fill=true,length={4,9},pressure={0.82,0.96},dips={1,1,0.98},mix_jitter=0.015})
print('quiet cheek restored')

--@ chunk 17
wait(7*24*60)
faceShadowFinal=pile{{'lead white',2.3},{'yellow ochre',1.5},{'red earth',0.65},{'ultramarine blue',0.12},{'bone black',0.08}}
faceLightFinal=pile{{'lead white',4.3},{'yellow ochre',0.85},{'red earth',0.3}}
local shadowFace=poly{{686,333},{692,338},{692,352},{685,359},{678,357},{685,351},{686,342}}:soften(0.45)*face:mask()
local lightFace=poly{{674,335},{681,334},{685,339},{683,348},{677,352},{673,348},{671,344},{673,339}}:soften(0.45)*face:mask()
for _,pass in ipairs{{shadowFace,faceShadowFinal},{lightFace,faceLightFinal}} do
 work(pass[1],{hand='detail',tool={kind='filbert',width=2.4,stiffness=0.32},pile=pass[2],coverage=4.4,angle=1.1,clip=true,fill=true,length={4,8},pressure={0.79,0.95},dips={1,1,0.98},mix_jitter=0.015})
end
-- Features are just short, warm marks, with the gaze turned down and away.
line(feature,0.65,{{674,339},{677,340}},0.85)
line(feature,0.55,{{673.8,342},{676.4,342.7}},0.85)
lip=pile{{'red earth',1.8},{'lead white',0.9},{'yellow ochre',0.5},{'bone black',0.15}}
line(lip,0.55,{{673,353},{677,353.5}},0.86)
line(faceShadowFinal,0.6,{{669,346},{672,347}},0.83)
line(faceLightFinal,0.85,{{675,357},{680,358}},0.83)
line(faceBaseFinal,1.8,{{691,343},{690.5,347}},0.88)
print('the last three flesh planes')
