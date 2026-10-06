-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=900, aspect=1.35, linen={22,19}, seed=184, ground={{pile={{"red earth",2},{"yellow ochre",3}},um=85,apply="knife",texture=0.32},{pile={{"lead white",8},{"yellow ochre",1},{"raw umber",0.25}},um=48,apply="brush"}}}
inkdark=pile{{"raw umber",3},{"Prussian blue",2},{"bone black",1}}
rock=pile{{"raw umber",3},{"red earth",1},{"Prussian blue",1},{"bone black",0.5}}
skybase=pile{{"lead white",7},{"pale smalt",2},{"yellow ochre",0.35}}
skycool=pile{{"lead white",4},{"smalt",2},{"Prussian blue",0.22}}
skywarm=pile{{"lead white",7},{"yellow ochre",1.3},{"red earth",0.18}}
work(rect(0,0,1000,470),{hand="broad",pile=skybase,angle=0.03,coverage=1.25,fill=true,edge="soft",seed=21})

--@ chunk 2
wait(1300)
skyveil=pile{{"pale smalt",2},{"smalt",0.8},{"raw umber",0.35},{"lead white",2},medium=0.16}
skyshadow=pile{{"smalt",2},{"raw umber",0.6},{"lead white",2.2}}
cloudglow=pile{{"lead white",5},{"yellow ochre",0.8},{"red earth",0.12}}
work(above(function(x) return 284+15*math.sin(x/108)+9*math.sin(x/47) end),{hand="broad",pile=skyveil,angle=0.06,coverage=0.95,fill=true,edge="lost",seed=44})
work(ellipse(540,379,300,68),{hand="broad",pile=cloudglow,angle=0.02,coverage=0.58,fill=false,edge="soft",seed=45})
cloudA=poly({{0,155},{104,120},{221,137},{304,184},{414,194},{482,217},{406,234},{315,211},{230,196},{122,182},{36,201}}):soften(9)
cloudB=poly({{734,176},{806,148},{903,161},{1000,136},{1000,211},{921,215},{842,202},{778,225},{713,210}}):soften(8)
work(cloudA+cloudB,{hand="body",pile=skyshadow,angle=0.08,coverage=1.1,fill=true,edge="lost",seed=46})
work(ellipse(492,345,197,20)+ellipse(580,325,140,12),{hand="scumble",pile=cloudglow,angle=0.03,coverage=0.6,edge="soft",seed=47})

--@ chunk 3
wait(1100)
far=pile{{"smalt",2.2},{"pale smalt",0.8},{"lead white",3.2},{"raw umber",0.28},{"red earth",0.12}}
farLight=pile{{"lead white",4},{"pale smalt",1.2},{"yellow ochre",0.22}}
farRidge=poly({{0,392},{0,367},{62,356},{126,369},{178,338},{223,350},{278,316},{314,327},{348,291},{374,312},{421,352},{459,340},{506,367},{553,352},{601,366},{647,329},{681,343},{733,306},{766,322},{812,283},{845,309},{885,291},{924,325},{964,305},{1000,318},{1000,449},{0,449}}):roughen(1.3,48,184):soften(1.5)
work(farRidge,{hand="body",pile=far,angle=0.85,coverage=1.35,fill=true,edge="soft",seed=51})
work(poly({{0,386},{61,359},{120,370},{178,339},{216,351},{279,318},{313,329},{345,295},{372,316},{414,353},{446,345},{475,365},{407,364},{367,343},{333,346},{300,341},{275,350},{240,355},{205,351},{174,359},{143,375},{99,372},{51,384}}),{hand="detail",pile=farLight,angle=0.1,coverage=0.75,edge="soft",seed=52})
work(ellipse(528,407,228,24),{hand="glaze",pile=cloudglow,angle=0.02,seed=53})

--@ chunk 4
wait(1250)
farveil=pile{{"smalt",1.2},{"pale smalt",0.5},{"lead white",2.4},{"raw umber",0.16},medium=0.28}
work(farRidge*above(function(x) return 322+8*math.sin(x/140) end),{hand="glaze",pile=farveil,angle=0.18,seed=61})
midmount=pile{{"raw umber",2},{"smalt",1.1},{"Prussian blue",0.12},{"lead white",1.25},{"red earth",0.32}}
leftPeak=poly({{0,317},{43,293},{82,283},{120,246},{155,258},{185,276},{222,284},{256,308},{295,328},{328,344},{365,360},{402,380},{431,407},{466,440},{0,467}}):roughen(1.8,35,61):soften(1)
rightPeak=poly({{1000,297},{968,278},{933,256},{900,244},{866,242},{834,221},{808,239},{783,263},{752,280},{722,307},{696,335},{676,363},{652,388},{625,412},{596,433},{576,444},{1000,475}}):roughen(1.7,39,62):soften(1)
work(leftPeak+rightPeak,{hand="body",pile=midmount,angle=0.72,coverage=1.45,fill=true,edge="soft",seed=63})
midlight=pile{{"lead white",2.3},{"pale smalt",1.1},{"yellow ochre",0.2},{"raw umber",0.22}}
work(poly({{112,250},{127,246},{151,262},{173,275},{198,285},{227,296},{252,309},{276,325},{299,338},{279,333},{254,320},{231,313},{208,302},{183,291},{160,278},{139,265}}),{hand="hatch",pile=midlight,angle=0.8,coverage=0.48,edge="soft",seed=64})
work(poly({{834,223},{850,236},{875,244},{904,250},{930,260},{954,276},{975,292},{956,286},{932,271},{910,264},{887,257},{866,251},{849,243}}),{hand="hatch",pile=midlight,angle=2.25,coverage=0.42,edge="soft",seed=65})

--@ chunk 5
wait(1500)
lakebase=pile{{"lead white",4},{"smalt",1.5},{"pale smalt",0.8},{"yellow ochre",0.22},{"Prussian blue",0.05}}
lakedeep=pile{{"smalt",2},{"raw umber",0.35},{"lead white",1.2}}
lakeglint=pile{{"lead white",6},{"yellow ochre",0.7},{"red earth",0.1}}
watermask=poly({{446,438},{519,435},{568,440},{592,460},{622,485},{654,507},{696,529},{730,558},{767,592},{792,632},{815,680},{847,741},{211,741},{244,701},{280,661},{315,623},{348,585},{376,548},{400,514},{423,480}}):soften(2)
work(watermask,{hand="broad",pile=lakebase,angle=0.025,coverage=1.2,fill=true,edge="soft",seed=71})
work(watermask,{hand="hatch",pile=lakedeep,angle=0.01,coverage=0.7,length={7,19},edge="soft",seed=72})
lightwater=poly({{519,441},{545,441},{550,459},{557,478},{564,503},{566,522},{577,548},{590,573},{602,604},{618,633},{611,648},{594,621},{582,591},{569,561},{557,533},{548,507},{541,482},{531,460}}):soften(3)
work(lightwater,{hand="hatch",pile=lakeglint,angle=0.015,coverage=0.75,length={5,13},edge="soft",seed=73})
work(ellipse(529,445,45,8),{hand="scumble",pile=lakeglint,angle=0.02,coverage=0.8,edge="lost",seed=74})

--@ chunk 6
wait(2600)
waterwash=pile{{"lead white",6},{"smalt",1.6},{"pale smalt",0.8},{"yellow ochre",0.18},{"raw umber",0.14}}
work(watermask,{hand="broad",pile=waterwash,angle=0.018,coverage=1.35,fill=true,edge="soft",seed=81})
shore=pile{{"raw umber",2.1},{"green earth",1.1},{"Prussian blue",0.22},{"bone black",0.6},{"red earth",0.25}}
leftshore=poly({{0,454},{41,452},{82,462},{121,469},{158,480},{196,493},{230,512},{266,527},{296,548},{326,567},{350,587},{376,606},{367,628},{341,646},{324,675},{306,706},{290,741},{0,741}}):roughen(2.6,31,91):soften(1)
rightshore=poly({{1000,463},{968,475},{931,486},{899,500},{871,512},{845,531},{822,548},{806,570},{793,592},{791,615},{807,646},{824,681},{846,741},{1000,741}}):roughen(2.5,28,92):soften(1)
work(leftshore+rightshore,{hand="body",pile=shore,angle=0.48,coverage=1.65,fill=true,edge="firm",seed=93})

--@ chunk 7
wait(1700)
midveil=pile{{"smalt",1.25},{"pale smalt",0.45},{"lead white",1.7},{"raw umber",0.12},medium=0.3}
work(leftPeak+rightPeak,{hand="glaze",pile=midveil,angle=0.11,seed=101})
work((watermask*below(function(x) return 518+10*math.sin(x/130) end)):soften(16),{hand="body",pile=lakedeep,angle=0.015,coverage=0.82,fill=true,edge="soft",seed=102})
skyglaze=pile{{"pale smalt",1.2},{"smalt",0.4},{"lead white",1.7},{"raw umber",0.11},medium=0.34}
work(rect(0,0,1000,438),{hand="glaze",pile=skyglaze,angle=0.03,seed=103})
cloudwarm=pile{{"lead white",4},{"yellow ochre",0.68},{"red earth",0.1}}
work(poly({{48,197},{104,182},{170,181},{228,192},{292,195},{349,212},{402,219},{373,226},{313,216},{261,208},{202,201},{146,199},{90,206},{48,214}}):soften(12),{hand="scumble",pile=cloudwarm,angle=0.07,coverage=0.68,edge="lost",seed=104})
work(poly({{729,204},{777,187},{825,178},{875,178},{921,180},{966,169},{1000,172},{1000,192},{951,193},{902,197},{850,196},{805,202},{765,213}}):soften(10),{hand="scumble",pile=cloudwarm,angle=0.04,coverage=0.56,edge="lost",seed=105})
work(poly({{236,274},{294,262},{352,267},{407,279},{455,283},{425,290},{373,286},{322,279},{280,281}}):soften(14),{hand="scumble",pile=skywarm,angle=0.02,coverage=0.52,edge="lost",seed=106})
work(poly({{627,261},{682,251},{723,253},{776,242},{835,250},{802,261},{755,265},{716,266},{675,271}}):soften(13),{hand="scumble",pile=skywarm,angle=0.02,coverage=0.48,edge="lost",seed=107})

--@ chunk 8
blend(rect(0,0,1000,436),{angle=0.14})
blend(watermask*rect(0,463,1000,565),{angle=0.01})

--@ chunk 9
wait(1200)
far2=pile{{"smalt",1.7},{"cobalt blue",0.35},{"lead white",2.6},{"raw umber",0.25},{"pale smalt",0.45}}
backridge=poly({{302,430},{344,412},{384,415},{414,395},{448,403},{480,382},{516,397},{548,373},{581,389},{612,361},{645,386},{680,373},{718,402},{751,381},{786,407},{817,390},{850,415},{884,397},{918,418},{1000,408},{1000,451},{302,451}}):roughen(1.1,45,121):soften(1)
work(backridge,{hand="body",pile=far2,angle=0.85,coverage=1.4,fill=true,edge="soft",seed=122})
mountainblue=pile{{"cobalt blue",2},{"smalt",2.6},{"raw umber",0.85},{"lead white",1.15},{"bone black",0.22},{"red earth",0.12}}
leftPeak2=poly({{0,376},{40,356},{79,344},{119,331},{159,317},{197,316},{230,298},{263,270},{283,258},{301,276},{323,295},{346,309},{370,327},{395,347},{416,370},{438,388},{461,409},{479,430},{476,458},{0,462}}):roughen(1.6,39,123):soften(0.8)
rightPeak2=poly({{1000,371},{968,354},{940,333},{911,317},{883,297},{859,276},{841,284},{823,305},{801,317},{780,337},{756,348},{737,371},{711,387},{686,406},{655,418},{623,432},{594,446},{574,458},{1000,462}}):roughen(1.5,41,124):soften(0.8)
work(leftPeak2+rightPeak2,{hand="body",pile=mountainblue,angle=0.7,coverage=1.55,fill=true,edge="firm",seed=125})
mountfacet=pile{{"lead white",1.4},{"pale smalt",1.4},{"cobalt blue",0.25},{"raw umber",0.18}}
work(poly({{264,271},{282,260},{298,278},{321,297},{343,311},{369,330},{392,350},{414,372},{430,391},{417,386},{397,366},{378,350},{354,334},{335,319},{316,304},{296,288},{281,276}}),{hand="body",pile=mountfacet,angle=0.68,coverage=0.68,edge="soft",seed=126})
work(poly({{841,285},{857,277},{878,296},{900,312},{923,330},{950,349},{974,366},{958,364},{935,348},{913,335},{893,321},{873,306},{855,294}}),{hand="body",pile=mountfacet,angle=2.35,coverage=0.56,edge="soft",seed=127})

--@ chunk 10
wait(1350)
rock=pile{{"raw umber",3},{"red earth",1},{"Prussian blue",1},{"bone black",0.5}}
foreground=poly({{291,741},{314,710},{341,685},{356,657},{381,637},{400,614},{425,597},{448,589},{468,592},{488,601},{514,610},{540,619},{565,634},{589,654},{612,675},{634,700},{655,727},{666,741}}):roughen(2.2,31,131):soften(1)
work(foreground,{hand="body",pile=rock,angle=0.62,coverage=1.65,fill=true,edge="firm",seed=132})
rockplane=pile{{"raw umber",2},{"red earth",1.2},{"yellow ochre",0.35},{"Prussian blue",0.28},{"lead white",0.16}}
work(poly({{358,689},{381,650},{414,619},{444,603},{467,607},{451,619},{423,627},{404,645},{389,666},{376,690},{354,710}}):soften(2),{hand="body",pile=rockplane,angle=0.75,coverage=0.9,edge="soft",seed=133})
work(poly({{467,612},{486,608},{513,619},{541,629},{565,646},{585,663},{608,690},{628,720},{638,741},{609,741},{597,709},{575,681},{551,659},{525,645},{499,632},{478,627}}):soften(3),{hand="body",pile=rockplane,angle=0.82,coverage=0.8,edge="soft",seed=134})
work(poly({{309,728},{328,701},{345,692},{362,690},{342,717},{328,741},{295,741}}):soften(2),{hand="body",pile=inkdark,angle=0.85,coverage=0.75,edge="lost",seed=135})

--@ chunk 11
treebark=pile{{"raw umber",2.4},{"bone black",1},{"red earth",0.55},{"Prussian blue",0.25}}
main=brush{kind="rigger",width=8.5,point=0.88,stiffness=0.85,lay=0.72}
main:load(treebark,0.88)
main:stroke({{908,555},{912,514},{903,470},{909,429},{899,386},{905,343},{899,299}},{pressure={0.88,0.14},ramps={0.04,0.16},orient="across"})
branch=brush{kind="rigger",width=5.3,point=0.82,stiffness=0.86,lay=0.72}
branch:load(treebark,1)
branch:stroke({{905,503},{881,493},{859,490},{836,476}},{pressure={0.78,0.1},ramps={0.05,0.22},orient="across"})
branch:stroke({{904,484},{927,477},{952,460},{975,456}},{pressure={0.72,0.08},ramps={0.05,0.22},orient="across"})
branch:reload(treebark,0.95)
branch:stroke({{904,464},{880,452},{858,431},{839,414}},{pressure={0.74,0.08},ramps={0.04,0.22},orient="across"})
branch:stroke({{908,439},{929,431},{952,412},{971,398}},{pressure={0.7,0.08},ramps={0.04,0.2},orient="across"})
branch:reload(treebark,0.95)
branch:stroke({{901,416},{882,407},{866,386},{855,370}},{pressure={0.69,0.07},ramps={0.05,0.2},orient="across"})
branch:stroke({{902,391},{923,383},{945,365},{957,346}},{pressure={0.65,0.07},ramps={0.05,0.2},orient="across"})
branch:reload(treebark,0.9)
branch:stroke({{903,365},{882,356},{868,338},{858,324}},{pressure={0.6,0.06},ramps={0.05,0.2},orient="across"})
branch:stroke({{904,344},{920,337},{934,321},{942,308}},{pressure={0.55,0.05},ramps={0.05,0.2},orient="across"})
branch:stroke({{900,323},{886,315},{879,302}},{pressure={0.43,0.04},ramps={0.06,0.2},orient="across"})

--@ chunk 12
pinebody=pile{{"green earth",2},{"raw umber",1.15},{"Prussian blue",0.3},{"yellow ochre",0.45},{"bone black",0.7}}
fol1=poly({{826,470},{836,467},{844,469},{850,463},{860,467},{870,468},{881,475},{892,477},{900,482},{891,484},{880,481},{868,478},{857,475},{847,475},{838,479},{830,477}})
fol2=poly({{922,469},{932,466},{939,458},{949,457},{958,447},{969,445},{981,447},{973,453},{968,461},{960,463},{954,472},{943,475},{933,477},{926,474}})
fol3=poly({{833,414},{840,407},{849,407},{856,401},{866,403},{874,408},{882,414},{889,416},{881,420},{869,416},{858,414},{850,415},{841,419}})
fol4=poly({{920,428},{930,423},{937,414},{948,410},{957,400},{969,397},{977,400},{970,407},{966,417},{957,420},{950,429},{938,433},{927,433}})
fol5=poly({{848,363},{854,355},{861,353},{867,345},{876,346},{882,352},{889,359},{896,364},{886,366},{877,363},{867,361},{859,365}})
fol6=poly({{918,366},{928,359},{935,350},{944,346},{951,339},{958,341},{953,350},{951,359},{943,364},{935,370},{926,371}})
fol7=poly({{871,325},{877,318},{883,313},{889,305},{897,307},{901,313},{908,318},{915,323},{905,325},{897,322},{889,321},{881,327}})
work(fol1+fol2+fol3+fol4+fol5+fol6+fol7,{hand="body",pile=pinebody,angle=0.12,coverage=1.15,fill=true,edge="firm",seed=141})
pineedge=pile{{"green earth",2},{"yellow ochre",0.55},{"raw umber",0.7},{"lead white",0.18},{"Prussian blue",0.18}}
needle=brush{kind="rigger",width=2.2,point=0.92,stiffness=0.55,lay=0.62}
needle:load(pineedge,0.72)
needle:stroke({{843,472},{836,461}},{pressure={0.62,0.03},ramps={0.06,0.18}})
needle:stroke({{855,472},{850,459}},{pressure={0.56,0.02},ramps={0.08,0.2}})
needle:stroke({{866,477},{863,464}},{pressure={0.54,0.02},ramps={0.08,0.2}})
needle:stroke({{876,479},{878,466}},{pressure={0.55,0.02},ramps={0.08,0.2}})
needle:stroke({{950,466},{946,452}},{pressure={0.56,0.02},ramps={0.08,0.2}})
needle:stroke({{959,461},{960,447}},{pressure={0.58,0.02},ramps={0.08,0.2}})
needle:stroke({{969,455},{976,444}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:reload(pineedge,0.7)
needle:stroke({{847,414},{838,402}},{pressure={0.55,0.02},ramps={0.08,0.2}})
needle:stroke({{858,412},{853,397}},{pressure={0.54,0.02},ramps={0.08,0.2}})
needle:stroke({{870,414},{870,401}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{940,428},{938,415}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{950,422},{951,409}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{961,414},{969,402}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:reload(pineedge,0.68)
needle:stroke({{862,359},{853,345}},{pressure={0.51,0.02},ramps={0.08,0.2}})
needle:stroke({{873,360},{869,345}},{pressure={0.49,0.02},ramps={0.08,0.2}})
needle:stroke({{884,362},{887,348}},{pressure={0.52,0.02},ramps={0.08,0.2}})
needle:stroke({{936,361},{932,349}},{pressure={0.46,0.02},ramps={0.08,0.2}})
needle:stroke({{946,358},{951,343}},{pressure={0.49,0.02},ramps={0.08,0.2}})
needle:stroke({{891,319},{885,306}},{pressure={0.45,0.02},ramps={0.08,0.2}})
needle:stroke({{900,321},{902,306}},{pressure={0.45,0.02},ramps={0.08,0.2}})

--@ chunk 13
figurecoat=pile{{"raw umber",2},{"bone black",1.3},{"Prussian blue",0.42},{"red earth",0.2}}
hood=ellipse(473.2,523,8.2,10.2)
work(hood,{hand="body",pile=figurecoat,angle=0.1,coverage=1.3,fill=true,edge="firm",seed=151})
work(poly({{466,520},{467,513},{471,509},{478,509},{483,514},{484,520},{480,522},{475,518},{470,523}}),{hand="body",pile=inkdark,angle=0.1,coverage=1.1,fill=true,edge="firm",seed=152})
cloak=poly({{471,533},{464,537},{458,544},{455,556},{458,565},{457,577},{452,588},{453,593},{462,597},{475,596},{484,594},{490,591},{487,583},{489,573},{486,560},{486,549},{481,539},{477,534}}):roughen(0.7,13,153):soften(0.5)
work(cloak,{hand="body",pile=figurecoat,angle=0.25,coverage=1.5,fill=true,edge="firm",seed=154})
work(poly({{459,542},{466,537},{470,540},{467,554},{467,568},{461,582},{456,587},{458,570},{456,557}}),{hand="body",pile=inkdark,angle=0.1,coverage=0.8,edge="soft",seed=155})
coatlight=pile{{"raw umber",1.3},{"red earth",0.5},{"smalt",0.45},{"lead white",0.22}}
work(poly({{478,542},{483,550},{484,563},{481,576},{484,588},{477,592},{477,578},{480,565}}),{hand="body",pile=coatlight,angle=0.18,coverage=0.7,edge="soft",seed=156})
boots=pile{{"bone black",1.5},{"raw umber",1.5},{"Prussian blue",0.35}}
work(poly({{455,589},{463,590},{462,600},{453,600}})+poly({{476,590},{485,589},{489,598},{480,600},{476,596}}),{hand="body",pile=boots,angle=0.04,coverage=1,fill=true,seed=157})
staff=brush{kind="rigger",width=3.1,point=0.93,stiffness=0.78,lay=0.74}
staff:load(treebark,0.76)
staff:stroke({{486,554},{497,568},{504,585},{509,607}},{pressure={0.65,0.18},ramps={0.08,0.14},orient="across"})
staff:stroke({{483,555},{488,558}},{pressure={0.5,0.2},ramps={0.08,0.18}})

--@ chunk 14
work(cloak,{hand="body",pile=figurecoat,angle=0.22,coverage=1.6,fill=true,edge="firm",seed=161})
work(poly({{470,540},{477,540},{480,548},{479,568},{480,580},{477,586},{474,574},{475,556}}),{hand="body",pile=figurecoat,angle=0.2,coverage=0.8,edge="soft",seed=162})
figureseam=pile{{"smalt",0.7},{"raw umber",0.85},{"lead white",0.25},{"red earth",0.22}}
seam=brush{kind="rigger",width=2.1,point=0.92,stiffness=0.6,lay=0.6}
seam:load(figureseam,0.52)
seam:stroke({{461,545},{459,556},{461,570},{459,581}},{pressure={0.42,0.03},ramps={0.07,0.2}})
seam:stroke({{476,543},{481,555},{480,572}},{pressure={0.28,0.02},ramps={0.1,0.24}})

--@ chunk 15
twig=brush{kind="rigger",width=2.8,point=0.9,stiffness=0.68,lay=0.64}
twig:load(treebark,0.86)
twig:stroke({{884,494},{870,485},{851,482}},{pressure={0.47,0.04},ramps={0.08,0.21}})
twig:stroke({{933,475},{945,478},{962,470}},{pressure={0.46,0.03},ramps={0.08,0.21}})
twig:stroke({{874,450},{861,443},{846,434}},{pressure={0.44,0.03},ramps={0.08,0.22}})
twig:stroke({{938,426},{953,421},{965,410}},{pressure={0.43,0.03},ramps={0.08,0.22}})
twig:stroke({{882,406},{874,393},{864,381}},{pressure={0.41,0.03},ramps={0.08,0.22}})
twig:stroke({{935,379},{948,370},{961,357}},{pressure={0.4,0.03},ramps={0.08,0.22}})
twig:stroke({{880,356},{871,343},{866,331}},{pressure={0.38,0.03},ramps={0.08,0.22}})
twig:stroke({{920,338},{931,329},{937,318}},{pressure={0.36,0.03},ramps={0.08,0.22}})
twig:stroke({{898,328},{894,317},{892,305}},{pressure={0.32,0.03},ramps={0.08,0.22}})
needle:reload(pineedge,0.7)
needle:stroke({{834,475},{821,470}},{pressure={0.55,0.02},ramps={0.08,0.2}})
needle:stroke({{840,472},{836,486}},{pressure={0.52,0.02},ramps={0.08,0.2}})
needle:stroke({{850,472},{845,487}},{pressure={0.52,0.02},ramps={0.08,0.2}})
needle:stroke({{864,476},{860,489}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{878,478},{880,489}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{959,464},{966,477}},{pressure={0.51,0.02},ramps={0.08,0.2}})
needle:stroke({{970,455},{980,458}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{846,412},{834,407}},{pressure={0.5,0.02},ramps={0.08,0.2}})
needle:stroke({{854,410},{852,424}},{pressure={0.48,0.02},ramps={0.08,0.2}})
needle:stroke({{867,413},{865,423}},{pressure={0.47,0.02},ramps={0.08,0.2}})
needle:reload(pineedge,0.65)
needle:stroke({{941,426},{942,438}},{pressure={0.47,0.02},ramps={0.08,0.2}})
needle:stroke({{952,418},{962,422}},{pressure={0.47,0.02},ramps={0.08,0.2}})
needle:stroke({{960,411},{970,400}},{pressure={0.48,0.02},ramps={0.08,0.2}})
needle:stroke({{867,359},{856,356}},{pressure={0.46,0.02},ramps={0.08,0.2}})
needle:stroke({{876,360},{880,373}},{pressure={0.44,0.02},ramps={0.08,0.2}})
needle:stroke({{890,362},{898,369}},{pressure={0.42,0.02},ramps={0.08,0.2}})
needle:stroke({{933,359},{923,364}},{pressure={0.44,0.02},ramps={0.08,0.2}})
needle:stroke({{945,357},{951,370}},{pressure={0.43,0.02},ramps={0.08,0.2}})
needle:stroke({{954,348},{962,342}},{pressure={0.42,0.02},ramps={0.08,0.2}})
needle:reload(pineedge,0.6)
needle:stroke({{883,322},{874,316}},{pressure={0.4,0.02},ramps={0.08,0.2}})
needle:stroke({{890,318},{885,329}},{pressure={0.39,0.02},ramps={0.08,0.2}})
needle:stroke({{901,321},{910,328}},{pressure={0.41,0.02},ramps={0.08,0.2}})
needle:stroke({{905,319},{911,311}},{pressure={0.39,0.02},ramps={0.08,0.2}})

--@ chunk 16
wait(420)
waterglaze=pile{{"lead white",4},{"smalt",2.1},{"pale smalt",1},{"yellow ochre",0.15},medium=0.18}
work(watermask*rect(0,485,1000,100),{hand="glaze",pile=waterglaze,angle=0,coverage=0.32,clip=true,seed=170})
rippleCool=pile{{"smalt",2},{"raw umber",0.35},{"lead white",1.3},{"Prussian blue",0.06}}
ripples=brush{kind="filbert",width=3.8,stiffness=0.72,lay=0.68,ragged=0.12}
ripples:load(rippleCool,0.75)
ripples:stroke({{475,478},{489,477},{504,479}},{pressure={0.52,0.14},ramps={0.06,0.2}})
ripples:stroke({{539,486},{555,484},{574,485}},{pressure={0.48,0.08},ramps={0.06,0.2}})
ripples:stroke({{470,496},{489,494},{504,495}},{pressure={0.47,0.07},ramps={0.06,0.2}})
ripples:stroke({{539,503},{558,505},{580,502}},{pressure={0.49,0.08},ramps={0.06,0.2}})
ripples:stroke({{459,520},{479,517},{498,519}},{pressure={0.46,0.06},ramps={0.07,0.2}})
ripples:stroke({{548,524},{567,520},{592,521}},{pressure={0.42,0.06},ramps={0.07,0.2}})
ripples:stroke({{442,542},{464,539},{482,541}},{pressure={0.43,0.06},ramps={0.07,0.2}})
ripples:stroke({{551,546},{578,543},{604,544}},{pressure={0.41,0.05},ramps={0.07,0.2}})
rippleLight=pile{{"lead white",3},{"yellow ochre",0.7},{"red earth",0.12},{"smalt",0.12}}
glint=brush{kind="filbert",width=4.6,stiffness=0.68,lay=0.64,ragged=0.1}
glint:load(rippleLight,0.7)
glint:stroke({{509,469},{521,468},{534,469}},{pressure={0.56,0.08},ramps={0.08,0.22}})
glint:stroke({{500,478},{515,477},{540,479}},{pressure={0.62,0.1},ramps={0.06,0.2}})
glint:stroke({{504,489},{518,488},{531,489}},{pressure={0.55,0.07},ramps={0.08,0.22}})
glint:stroke({{489,499},{510,497},{533,498}},{pressure={0.59,0.08},ramps={0.07,0.2}})
glint:stroke({{495,510},{514,509},{539,510}},{pressure={0.63,0.09},ramps={0.06,0.2}})
glint:stroke({{480,522},{498,520},{521,522}},{pressure={0.57,0.08},ramps={0.07,0.2}})
glint:stroke({{484,535},{505,533},{533,535}},{pressure={0.58,0.09},ramps={0.07,0.2}})
glint:stroke({{469,548},{490,546},{514,547}},{pressure={0.55,0.07},ramps={0.08,0.22}})
glint:stroke({{466,561},{482,559},{504,560}},{pressure={0.52,0.07},ramps={0.08,0.22}})

--@ chunk 17
work(foreground,{hand="body",pile=rock,angle=0.7,coverage=1.35,fill=true,edge="firm",seed=181})
work(poly({{358,689},{381,650},{414,619},{444,603},{467,607},{451,619},{423,627},{404,645},{389,666},{376,690},{354,710}}):soften(2),{hand="body",pile=rockplane,angle=0.7,coverage=0.45,edge="soft",seed=182})
work(poly({{471,533},{464,537},{458,544},{455,556},{458,565},{457,577},{452,588},{453,593},{462,597},{475,596},{484,594},{490,591},{487,583},{489,573},{486,560},{486,549},{481,539},{477,534}}):roughen(0.7,13,183):soften(0.5),{hand="body",pile=figurecoat,angle=0.25,coverage=1.45,fill=true,edge="firm",seed=184})
work(hood,{hand="body",pile=figurecoat,angle=0.1,coverage=1.2,fill=true,edge="firm",seed=185})
work(poly({{466,520},{467,513},{471,509},{478,509},{483,514},{484,520},{480,522},{475,518},{470,523}}),{hand="body",pile=inkdark,angle=0.1,coverage=1.0,fill=true,edge="firm",seed=186})
work(poly({{455,589},{463,590},{462,600},{453,600}})+poly({{476,590},{485,589},{489,598},{480,600},{476,596}}),{hand="body",pile=boots,angle=0.04,coverage=0.9,fill=true,seed=187})
staff:reload(treebark,0.68)
staff:stroke({{486,554},{497,568},{504,585},{509,607}},{pressure={0.65,0.18},ramps={0.08,0.14},orient="across"})

--@ chunk 18
staffband=ribbon({{486,554},{497,568},{504,585},{509,607}},{4.4,3.4,2.8,1.8})
nonwater=foreground+cloak+hood+staffband
waterveil=pile{{"lead white",3.2},{"smalt",2.1},{"pale smalt",1.2},{"yellow ochre",0.12},medium=0.3}
waterrepair=watermask*rect(0,476,1000,112)-nonwater
work(waterrepair:soften(3),{hand="glaze",pile=waterveil,angle=0,coverage=0.62,clip=true,seed=191})

--@ chunk 19
figuredeep=pile{{"bone black",1.6},{"raw umber",1.4},{"Prussian blue",0.3}}
work(cloak+hood,{hand="body",pile=figuredeep,angle=0.32,coverage=4.8,fill=true,clip=true,edge="firm",seed=201})
work(poly({{455,589},{463,590},{462,600},{453,600}})+poly({{476,590},{485,589},{489,598},{480,600},{476,596}}),{hand="body",pile=figuredeep,angle=0.02,coverage=2,fill=true,clip=true,seed=202})
staff:reload(treebark,0.72)
staff:stroke({{486,554},{497,568},{504,585},{509,607}},{pressure={0.66,0.14},ramps={0.06,0.18},orient="across"})

--@ chunk 20
wait(310)
foliageall=fol1+fol2+fol3+fol4+fol5+fol6+fol7
pineglaze=pile{{"green earth",2.1},{"raw umber",0.9},{"yellow ochre",0.35},{"Prussian blue",0.18},medium=0.24}
work(foliageall:soften(2),{hand="glaze",pile=pineglaze,angle=0.24,coverage=0.58,seed=211})
pineLit=pile{{"green earth",2},{"yellow ochre",0.55},{"raw umber",0.9},{"Prussian blue",0.14},{"lead white",0.12}}
work(foliageall:grow(1.4):soften(1.4),{hand="scumble",pile=pineLit,angle=0.15,angle_jitter=0.6,coverage=0.48,pressure={0.28,0.58},edge="soft",seed=212})

--@ chunk 21
wait(500)
rightRepair=rightPeak2*rect(908,292,92,174)
work(rightRepair,{hand="body",pile=mountainblue,angle=0.7,coverage=2.2,fill=true,clip=true,edge="firm",seed=221})
waterFix=watermask*rect(710,438,176,96)-rightshore-foreground
work(waterFix,{hand="body",pile=waterwash,angle=0.02,coverage=1.8,fill=true,clip=true,edge="soft",seed=222})
shoreFix=rightshore*rect(838,458,162,95)
work(shoreFix,{hand="body",pile=shore,angle=0.48,coverage=2.1,fill=true,clip=true,edge="firm",seed=223})

--@ chunk 22
for _,p in ipairs({{720,420},{720,440},{720,460},{760,430},{760,450},{760,470},{800,430},{800,450},{800,470},{840,450},{840,470},{880,470},{950,400},{950,440}}) do print(p[1],p[2],string.format("mount %.2f water %.2f shore %.2f",rightPeak2:at(p[1],p[2]),watermask:at(p[1],p[2]),rightshore:at(p[1],p[2]))) end

--@ chunk 23
rightRepair=rightPeak2*rect(680,374,320,94)
work(rightRepair,{hand="body",pile=mountainblue,angle=0.7,coverage=2.25,fill=true,clip=true,edge="soft",seed=231})
skyRepair=(rect(910,285,90,80)-rightPeak2)
work(skyRepair,{hand="body",pile=skybase,angle=0.03,coverage=2,fill=true,clip=true,edge="firm",seed=232})
farBank=pile{{"lead white",5},{"yellow ochre",0.7},{"red earth",0.12},{"pale smalt",0.18},{"raw umber",0.28}}
bankStripe=poly({{693,450},{732,453},{770,459},{810,466},{846,473},{881,479},{914,482},{932,479},{928,488},{906,494},{878,492},{842,486},{807,480},{771,472},{736,464},{704,459}}):soften(3)
work(bankStripe,{hand="body",pile=farBank,angle=0.02,coverage=1.55,fill=true,clip=true,edge="soft",seed=233})
work(foliageall,{hand="body",pile=pinebody,angle=0.15,coverage=1.55,fill=true,clip=true,edge="firm",seed=234})

--@ chunk 24
pineDark=pile{{"raw umber",3},{"bone black",1.1},{"green earth",1.1},{"Prussian blue",0.25},{"yellow ochre",0.22}}
work(foliageall,{hand="body",pile=pineDark,angle=0.15,coverage=2.6,fill=true,clip=true,edge="firm",seed=241})
work((rightPeak2*rect(660,368,340,104)),{hand="scumble",pile=mountfacet,angle=0.72,angle_jitter=0.24,coverage=0.42,clip=true,edge="soft",seed=242})
work(rightPeak2*rect(650,370,350,100),{hand="glaze",pile=midveil,angle=0.12,coverage=0.32,clip=true,seed=243})
main=brush{kind="rigger",width=6.8,point=0.88,stiffness=0.85,lay=0.72}
main:load(treebark,0.86)
main:stroke({{908,555},{912,514},{903,470},{909,429},{899,386},{905,343},{899,299}},{pressure={0.88,0.14},ramps={0.04,0.16},orient="across"})
branch=brush{kind="rigger",width=4.2,point=0.82,stiffness=0.86,lay=0.72}
branch:load(treebark,0.9)
branch:stroke({{905,503},{881,493},{859,490},{836,476}},{pressure={0.78,0.1},ramps={0.05,0.22},orient="across"})
branch:stroke({{904,484},{927,477},{952,460},{975,456}},{pressure={0.72,0.08},ramps={0.05,0.22},orient="across"})
branch:stroke({{904,464},{880,452},{858,431},{839,414}},{pressure={0.74,0.08},ramps={0.04,0.22},orient="across"})
branch:stroke({{908,439},{929,431},{952,412},{971,398}},{pressure={0.7,0.08},ramps={0.04,0.2},orient="across"})
branch:stroke({{901,416},{882,407},{866,386},{855,370}},{pressure={0.69,0.07},ramps={0.05,0.2},orient="across"})
branch:stroke({{902,391},{923,383},{945,365},{957,346}},{pressure={0.65,0.07},ramps={0.05,0.2},orient="across"})
branch:stroke({{903,365},{882,356},{868,338},{858,324}},{pressure={0.6,0.06},ramps={0.05,0.2},orient="across"})
branch:stroke({{904,344},{920,337},{934,321},{942,308}},{pressure={0.55,0.05},ramps={0.05,0.2},orient="across"})

--@ chunk 25
mountFix=rightPeak2*rect(600,370,400,103)
work(mountFix,{hand="body",pile=mountainblue,angle=0.7,coverage=3,fill=true,clip=true,edge="soft",seed=251})
work(foliageall,{hand="body",pile=pineDark,angle=0.15,coverage=2.3,fill=true,clip=true,edge="firm",seed=252})
main:reload(treebark,0.84)
main:stroke({{908,555},{912,514},{903,470},{909,429},{899,386},{905,343},{899,299}},{pressure={0.88,0.14},ramps={0.04,0.16},orient="across"})
branch:reload(treebark,0.88)
branch:stroke({{905,503},{881,493},{859,490},{836,476}},{pressure={0.78,0.1},ramps={0.05,0.22},orient="across"})
branch:stroke({{904,484},{927,477},{952,460},{975,456}},{pressure={0.72,0.08},ramps={0.05,0.22},orient="across"})
branch:stroke({{904,464},{880,452},{858,431},{839,414}},{pressure={0.74,0.08},ramps={0.04,0.22},orient="across"})
branch:stroke({{908,439},{929,431},{952,412},{971,398}},{pressure={0.7,0.08},ramps={0.04,0.2},orient="across"})
branch:stroke({{901,416},{882,407},{866,386},{855,370}},{pressure={0.69,0.07},ramps={0.05,0.2},orient="across"})
branch:stroke({{902,391},{923,383},{945,365},{957,346}},{pressure={0.65,0.07},ramps={0.05,0.2},orient="across"})
branch:stroke({{903,365},{882,356},{868,338},{858,324}},{pressure={0.6,0.06},ramps={0.05,0.2},orient="across"})
branch:stroke({{904,344},{920,337},{934,321},{942,308}},{pressure={0.55,0.05},ramps={0.05,0.2},orient="across"})

--@ chunk 26
pineSolid=pile{{"raw umber",4},{"bone black",2},{"green earth",1},{"Prussian blue",0.22}}
work(foliageall,{hand="body",pile=pineSolid,angle=0.15,coverage=2.4,fill=true,clip=true,edge="firm",seed=261})

--@ chunk 27
treeSolid=pile{{"raw umber",4},{"bone black",3},{"Prussian blue",0.45}}
main=brush{kind="rigger",width=9,point=0.9,stiffness=0.86,lay=0.8}
main:load(treeSolid,0.95)
main:stroke({{908,555},{912,515},{905,477},{908,438},{901,398},{905,359},{900,323},{899,299}},{pressure={0.92,0.14},ramps={0.035,0.16},orient="across"})
branch=brush{kind="rigger",width=5.2,point=0.84,stiffness=0.86,lay=0.76}
branch:load(treeSolid,0.9)
branch:stroke({{906,516},{888,505},{870,498},{850,489},{836,477}},{pressure={0.78,0.08},ramps={0.06,0.2}})
branch:stroke({{907,503},{926,495},{946,481},{968,468},{982,461}},{pressure={0.76,0.08},ramps={0.06,0.2}})
branch:stroke({{906,477},{885,465},{865,447},{846,431},{832,421}},{pressure={0.75,0.07},ramps={0.06,0.21}})
branch:stroke({{906,452},{926,443},{946,428},{964,412},{980,402}},{pressure={0.72,0.07},ramps={0.06,0.21}})
branch:stroke({{904,427},{884,414},{869,397},{855,380},{846,371}},{pressure={0.7,0.06},ramps={0.06,0.21}})
branch:stroke({{903,401},{922,393},{942,377},{956,363},{965,352}},{pressure={0.67,0.05},ramps={0.06,0.21}})
branch:stroke({{902,374},{884,364},{869,350},{856,334},{848,326}},{pressure={0.64,0.05},ramps={0.06,0.21}})
branch:stroke({{901,348},{918,341},{933,329},{945,316},{951,306}},{pressure={0.59,0.04},ramps={0.06,0.21}})
branch:stroke({{900,324},{887,316},{879,305}},{pressure={0.5,0.03},ramps={0.08,0.21}})

--@ chunk 28
trunkForm=ribbon({{908,555},{912,514},{905,477},{908,438},{901,398},{905,359},{900,323},{899,299}},{9,8,7,6,5,4,3,1})
branchForm=ribbon({{906,516},{888,505},{870,498},{850,489},{836,477}},{5.5,4.8,3.5,2.4,0.8})+ribbon({{907,503},{926,495},{946,481},{968,468},{982,461}},{5.4,4.5,3.2,2.1,0.7})+ribbon({{906,477},{885,465},{865,447},{846,431},{832,421}},{5.2,4.4,3.2,2.1,0.7})+ribbon({{906,452},{926,443},{946,428},{964,412},{980,402}},{5,4.2,3,2,0.7})+ribbon({{904,427},{884,414},{869,397},{855,380},{846,371}},{4.7,4,3,1.8,0.6})+ribbon({{903,401},{922,393},{942,377},{956,363},{965,352}},{4.5,3.8,2.8,1.7,0.6})+ribbon({{902,374},{884,364},{869,350},{856,334},{848,326}},{4.2,3.5,2.6,1.5,0.6})+ribbon({{901,348},{918,341},{933,329},{945,316},{951,306}},{3.8,3.2,2.3,1.3,0.5})+ribbon({{900,324},{887,316},{879,305}},{3.4,2.2,0.6})
work(trunkForm+branchForm,{hand="body",pile=treeSolid,angle=0.2,coverage=2,fill=true,clip=true,edge="firm",seed=271})

--@ chunk 29
pineLit=pile{{"green earth",2},{"yellow ochre",0.55},{"raw umber",0.9},{"Prussian blue",0.14},{"lead white",0.12}}
needle=brush{kind="rigger",width=1.8,point=0.94,stiffness=0.58,lay=0.55}
needle:load(pineLit,0.7)
needle:stroke({{859,327},{850,319}},{pressure={0.48,0.03},ramps={0.08,0.22}})
needle:stroke({{872,325},{867,313}},{pressure={0.46,0.02},ramps={0.08,0.22}})
needle:stroke({{887,324},{890,312}},{pressure={0.46,0.02},ramps={0.08,0.22}})
needle:stroke({{901,326},{912,317}},{pressure={0.44,0.02},ramps={0.08,0.22}})
needle:stroke({{928,362},{923,348}},{pressure={0.48,0.03},ramps={0.08,0.22}})
needle:stroke({{940,357},{942,343}},{pressure={0.47,0.02},ramps={0.08,0.22}})
needle:stroke({{953,360},{961,349}},{pressure={0.44,0.02},ramps={0.08,0.22}})
needle:stroke({{962,369},{974,366}},{pressure={0.43,0.02},ramps={0.08,0.22}})
needle:stroke({{843,414},{833,405}},{pressure={0.46,0.02},ramps={0.08,0.22}})
needle:stroke({{854,414},{851,400}},{pressure={0.45,0.02},ramps={0.08,0.22}})
needle:stroke({{869,415},{871,403}},{pressure={0.43,0.02},ramps={0.08,0.22}})
needle:stroke({{881,419},{891,412}},{pressure={0.42,0.02},ramps={0.08,0.22}})
needle:stroke({{938,428},{934,416}},{pressure={0.45,0.02},ramps={0.08,0.22}})
needle:stroke({{950,422},{955,410}},{pressure={0.43,0.02},ramps={0.08,0.22}})
needle:stroke({{963,414},{976,408}},{pressure={0.42,0.02},ramps={0.08,0.22}})
needle:stroke({{824,472},{816,463}},{pressure={0.47,0.02},ramps={0.08,0.22}})
needle:stroke({{839,475},{834,462}},{pressure={0.44,0.02},ramps={0.08,0.22}})
needle:stroke({{854,479},{855,465}},{pressure={0.45,0.02},ramps={0.08,0.22}})
needle:stroke({{870,480},{877,470}},{pressure={0.42,0.02},ramps={0.08,0.22}})
needle:stroke({{892,488},{902,480}},{pressure={0.4,0.02},ramps={0.08,0.22}})
needle:stroke({{956,474},{964,464}},{pressure={0.45,0.02},ramps={0.08,0.22}})
needle:stroke({{970,465},{983,463}},{pressure={0.42,0.02},ramps={0.08,0.22}})
needle:stroke({{946,485},{950,477}},{pressure={0.4,0.02},ramps={0.08,0.22}})
needle:stroke({{880,361},{875,348}},{pressure={0.43,0.02},ramps={0.08,0.22}})
needle:stroke({{896,364},{904,356}},{pressure={0.4,0.02},ramps={0.08,0.22}})
needle:stroke({{866,451},{857,443}},{pressure={0.43,0.02},ramps={0.08,0.22}})
needle:stroke({{883,461},{888,450}},{pressure={0.42,0.02},ramps={0.08,0.22}})

--@ chunk 30
needleLight=pile{{"green earth",1.6},{"yellow ochre",0.9},{"raw umber",0.45},{"lead white",0.55},{"Prussian blue",0.12}}
stipple(foliageall,{pile=needleLight,width=2.2,coverage=0.42,pressure={0.2,0.57},cluster=0.55,clip=true,seed=301})

--@ chunk 31
wait(120)

--@ chunk 32
for _,q in ipairs({{600,560},{650,574},{700,586},{665,610},{325,565},{368,590},{705,622},{630,650},{730,650},{590,585}}) do print(q[1],q[2],string.format('water %.2f foreground %.2f shore %.2f',watermask:at(q[1],q[2]),foreground:at(q[1],q[2]),rightshore:at(q[1],q[2]))) end

--@ chunk 33
nearGleam=pile{{"lead white",2.2},{"pale smalt",1.1},{"smalt",0.25},{"yellow ochre",0.08}}
waterline=brush{kind="filbert",width=3.3,stiffness=0.72,lay=0.66,ragged=0.24}
waterline:load(nearGleam,0.56)
waterline:stroke({{402,577},{411,576},{421,577}},{pressure={0.38,0.05},ramps={0.08,0.24},orient="across"})
waterline:stroke({{558,571},{570,570},{581,571}},{pressure={0.42,0.04},ramps={0.08,0.23},orient="across"})
waterline:stroke({{625,579},{637,578},{646,579}},{pressure={0.34,0.035},ramps={0.08,0.22},orient="across"})
waterline:reload(nearGleam,0.48)
waterline:stroke({{680,595},{694,594},{708,595}},{pressure={0.38,0.04},ramps={0.08,0.23},orient="across"})
waterline:stroke({{643,619},{656,618},{669,619}},{pressure={0.36,0.035},ramps={0.08,0.24},orient="across"})
waterline:stroke({{716,652},{731,651},{744,652}},{pressure={0.3,0.025},ramps={0.09,0.23},orient="across"})

--@ chunk 34
wait(90)
