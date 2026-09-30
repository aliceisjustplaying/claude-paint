-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=950, aspect=1.36, linen={17,14}, seed=28, ground={{pile={{"yellow ochre",2},{"red earth",1},{"lead white",1}},um=95,apply="knife",texture=0.22},{pile={{"lead white",3},{"yellow ochre",1}},um=30,apply="brush"}}}

--@ chunk 2
pSky = pile{{"smalt",3},{"cobalt blue",2},{"lead white",3},{"red earth",0.5},{"yellow ochre",0.35}}; sky = above(function(x) return 418 + 7*math.sin(x/150) + 0.014*(x-500) end); work(sky,{hand="broad",pile=pSky,angle=0.18,coverage=1.4,fill=true})

--@ chunk 3
wait(1440); pTop = pile{{"smalt",3},{"cobalt blue",2},{"red earth",0.8},{"bone black",0.2},{"lead white",0.7},medium=0.15}; topmask=above(function(x) return 260 end):soften(105); work(topmask,{hand="glaze",pile=pTop}); pMid=pile{{"smalt",2},{"cobalt blue",2},{"red earth",0.45},{"lead white",2},{"yellow ochre",0.4}}; midsky=rect(0,180,1000,240):soften(95)*sky; work(midsky,{hand="broad",pile=pMid,angle=0.06,coverage=1.4,fill=true}); pGlow=pile{{"lead white",4},{"yellow ochre",1.2},{"vermilion",0.4},{"red earth",0.22},{"pale smalt",0.4}}; glow=rect(0,335,1000,95):soften(55)*sky; work(glow,{hand="glaze",pile=pGlow})

--@ chunk 4
hz = function(x) return 418 + 7*math.sin(x/150) + 0.014*(x-500) end; pSea = pile{{"cobalt blue",3},{"smalt",2},{"raw umber",1},{"lead white",1.3},{"red earth",0.5}}; sea=below(hz); work(sea,{hand="broad",pile=pSea,angle=0.08,coverage=2.3,fill=true});

--@ chunk 5
wait(1440); pLand=pile{{"raw umber",2.5},{"cobalt blue",1.5},{"smalt",1},{"red earth",0.8},{"yellow ochre",0.35},{"lead white",0.4}}; isleL=poly({{0,391},{68,386},{126,382},{184,388},{237,379},{285,385},{328,393},{370,401},{411,406},{438,418},{444,438},{0,438}},true); isleR=poly({{872,416},{899,408},{921,400},{943,393},{961,383},{980,373},{1000,370},{1000,438},{860,438}},true); work(isleL+isleR,{hand="body",pile=pLand,angle=-0.08,coverage=2.1,fill=true})

--@ chunk 6
pMoonGlow=pile{{"lead white",2},{"yellow ochre",1},{"vermilion",0.3},{"pale smalt",0.5},medium=0.36}; halo=ellipse(720,220,66,55):soften(38)*sky; work(halo,{hand="glaze",pile=pMoonGlow}); pMoon=pile{{"lead white",6},{"yellow ochre",1.1},{"vermilion",0.16}}; disk=ellipse(720,220,30,30); work(disk,{hand="detail",pile=pMoon,coverage=1.6,fill=true,clip=true})

--@ chunk 7
wait(1440); pDeep=pile{{"cobalt blue",2.5},{"smalt",2},{"raw umber",1},{"bone black",0.25},{"lead white",0.5},medium=0.18}; deep=rect(0,560,1000,175):soften(115)*sea; work(deep,{hand="glaze",pile=pDeep}); pBank=pile{{"raw umber",2.2},{"cobalt blue",1.7},{"red earth",1},{"yellow ochre",0.8},{"bone black",0.25},{"lead white",0.6}}; bankL=poly({{0,592},{48,581},{96,586},{150,598},{206,613},{262,628},{319,642},{374,663},{424,684},{473,710},{525,735},{0,735}},true); bankR=poly({{1000,596},{961,590},{923,594},{881,605},{841,621},{804,642},{771,666},{744,698},{727,735},{1000,735}},true); work(bankL+bankR,{hand="body",pile=pBank,angle=0.2,coverage=1.9,fill=true})

--@ chunk 8
wait(1440); pCloud=pile{{"smalt",2},{"cobalt blue",1},{"lead white",2.2},{"red earth",0.45},{"raw umber",0.22}}; cloud1=poly({{575,268},{598,252},{633,247},{662,249},{690,257},{719,256},{751,261},{778,267},{806,272},{788,281},{758,275},{729,280},{697,272},{667,277},{634,271},{606,278}},true):soften(9); cloud2=poly({{560,296},{588,289},{615,291},{638,298},{620,302},{590,301}},true):soften(8); work(cloud1+cloud2,{hand="body",pile=pCloud,angle=0.18,coverage=1.2,fill=true,edge="soft"})

--@ chunk 9
wait(1440); pReflect=pile{{"lead white",4},{"yellow ochre",1.15},{"vermilion",0.16},{"smalt",0.55},{"raw umber",0.2}}; rb=brush{kind="filbert",width=6,stiffness=0.5,point=0.12}; rb:load(pReflect,0.72); rb:stroke({{701,432},{716,431},{727,433}}, {pressure={0.44,0.26},ramps={0.08,0.2}}); rb:stroke({{711,443},{725,441},{739,442}}, {pressure={0.36,0.24}}); rb:stroke({{691,455},{708,454},{722,456}}, {pressure={0.48,0.27}}); rb:stroke({{719,467},{735,465},{749,467}}, {pressure={0.4,0.2}}); rb:reload(pReflect,0.72); rb:stroke({{682,479},{700,478},{717,480},{729,479}}, {pressure={0.46,0.28}}); rb:stroke({{702,492},{720,491},{742,493}}, {pressure={0.43,0.23}}); rb:stroke({{674,506},{693,505},{711,507}}, {pressure={0.42,0.26}}); rb:stroke({{724,517},{744,516},{758,518}}, {pressure={0.4,0.25}}); rb:reload(pReflect,0.74); rb:stroke({{690,531},{713,529},{732,531},{749,530}}, {pressure={0.48,0.28}}); rb:stroke({{661,546},{680,545},{698,547}}, {pressure={0.46,0.3}}); rb:stroke({{707,556},{732,555},{753,557}}, {pressure={0.45,0.27}}); rb:stroke({{675,570},{700,568},{721,570},{736,568}}, {pressure={0.5,0.29}}); rb:reload(pReflect,0.76); rb:stroke({{647,585},{671,583},{693,585}}, {pressure={0.5,0.3}}); rb:stroke({{704,594},{732,592},{759,594},{778,592}}, {pressure={0.48,0.3}}); rb:stroke({{667,612},{699,610},{725,612},{749,610}}, {pressure={0.5,0.32}}); rb:stroke({{627,630},{659,628},{686,630}}, {pressure={0.5,0.28}})

--@ chunk 10
pBar=pile{{"lead white",1.8},{"yellow ochre",1.2},{"raw umber",0.8},{"cobalt blue",0.65},{"red earth",0.3}}; bar=poly({{407,617},{426,610},{449,606},{472,609},{497,614},{518,618},{543,621},{565,626},{586,633},{566,639},{544,634},{518,630},{492,625},{468,624},{444,620},{423,621}},true):soften(3); work(bar,{hand="body",pile=pBar,angle=-0.1,coverage=1.4,fill=true})

--@ chunk 11
pRipple=pile{{"cobalt blue",2.2},{"smalt",1.2},{"raw umber",0.5},{"lead white",0.8},{"yellow ochre",0.18}}; wb=brush{kind="filbert",width=8,stiffness=0.58,point=0.08}; wb:load(pRipple,0.56); wb:stroke({{647,449},{666,447},{680,448}}, {pressure={0.44,0.3}}); wb:stroke({{753,461},{775,460},{789,462}}, {pressure={0.42,0.3}}); wb:stroke({{656,485},{672,484},{688,485}}, {pressure={0.48,0.27}}); wb:stroke({{744,499},{765,498},{780,500},{795,499}}, {pressure={0.45,0.3}}); wb:stroke({{636,519},{658,517},{676,519}}, {pressure={0.46,0.29}}); wb:reload(pRipple,0.55); wb:stroke({{756,536},{773,534},{794,536}}, {pressure={0.46,0.3}}); wb:stroke({{649,554},{670,552},{688,554}}, {pressure={0.5,0.31}}); wb:stroke({{766,578},{788,576},{806,578}}, {pressure={0.5,0.32}}); wb:stroke({{630,597},{653,595},{674,597},{688,596}}, {pressure={0.5,0.32}}); wb:stroke({{750,615},{779,613},{802,615}}, {pressure={0.5,0.31}}); wb:stroke({{605,646},{628,644},{649,646}}, {pressure={0.48,0.32}}); wb:stroke({{716,651},{746,649},{767,651}}, {pressure={0.48,0.3}})

--@ chunk 12
wait(1440); pBarShade=pile{{"smalt",1.1},{"cobalt blue",0.65},{"raw umber",1},{"lead white",1.2},{"red earth",0.22}}; barshade=poly({{420,622},{441,620},{462,622},{482,626},{504,627},{526,631},{549,634},{573,636},{563,641},{538,637},{516,633},{493,630},{471,628},{449,626},{430,628}},true):soften(2); work(barshade,{hand="detail",pile=pBarShade,coverage=1.2,fill=true});

--@ chunk 13
pFigure=pile{{"raw umber",2},{"cobalt blue",1.2},{"red earth",0.9},{"bone black",0.35},{"yellow ochre",0.3}}; hood=ellipse(517,550,5.2,6.5); cloak=poly({{513,558},{516,556},{521,557},{525,560},{529,566},{530,576},{526,584},{526,590},{530,594},{528,602},{523,602},{520,592},{516,602},{511,603},{512,595},{509,589},{508,580},{509,570}},true); figure=hood+cloak; work(figure,{hand="detail",pile=pFigure,coverage=2.4,fill=true}); pCoat=pile{{"red earth",1.7},{"raw umber",0.9},{"yellow ochre",0.65},{"vermilion",0.22},{"cobalt blue",0.35},{"lead white",0.25}}; coatlight=poly({{520,561},{524,561},{527,566},{526,574},{524,580},{521,585},{518,582},{519,574}},true); work(coatlight,{hand="detail",pile=pCoat,coverage=1.7,fill=true});

--@ chunk 14
pArm=pile{{"raw umber",2},{"red earth",0.8},{"cobalt blue",0.55},{"bone black",0.25}}; ab=brush{kind="round",width=3.4,point=0.75,stiffness=0.55}; ab:load(pArm,0.72); ab:stroke({{526,566},{530,569},{533,574}}, {pressure={0.6,0.34},ramps={0.05,0.2}}); pLantern=pile{{"vermilion",1},{"yellow ochre",1.5},{"lead white",0.9}}; lamp=ellipse(535,576.5,2.4,2.8); work(lamp,{hand="detail",pile=pLantern,coverage=2.1,fill=true}); pFace=pile{{"lead white",0.7},{"yellow ochre",0.9},{"red earth",0.5}}; fb=brush{kind="round",width=2.5,point=0.8}; fb:load(pFace,0.34); fb:touch(521,551,{pressure=0.42,angle=0.2});

--@ chunk 15
pNear=pile{{"lead white",3.1},{"yellow ochre",0.9},{"smalt",0.45},{"red earth",0.22},{"raw umber",0.18}}; nb=brush{kind="filbert",width=8,stiffness=0.48,point=0.12}; nb:load(pNear,0.68); nb:stroke({{688,645},{706,643},{725,645}}, {pressure={0.45,0.28}}); nb:stroke({{715,657},{737,655},{753,658}}, {pressure={0.48,0.3}}); nb:stroke({{676,672},{693,670},{716,672},{730,670}}, {pressure={0.48,0.29}}); nb:reload(pNear,0.65); nb:stroke({{696,687},{718,685},{741,687}}, {pressure={0.5,0.3}}); nb:stroke({{665,702},{685,700},{704,703}}, {pressure={0.48,0.28}}); lb=brush{kind="filbert",width=4,point=0.4}; lb:load(pLantern,0.28); lb:stroke({{537,607},{540,615},{538,619}}, {pressure={0.32,0.14}}); lb:stroke({{544,611},{546,615}}, {pressure={0.22,0.1}})

--@ chunk 16
pEdgeWet=pile{{"lead white",1.15},{"yellow ochre",0.55},{"smalt",0.8},{"raw umber",0.35},{"green earth",0.3}}; eb=brush{kind="filbert",width=7,stiffness=0.55,point=0.15}; eb:load(pEdgeWet,0.56); eb:stroke({{8,596},{22,591},{37,588}}, {pressure={0.36,0.18}}); eb:stroke({{61,588},{76,587},{90,590}}, {pressure={0.34,0.18}}); eb:stroke({{111,593},{125,597},{137,600}}, {pressure={0.33,0.18}}); eb:reload(pEdgeWet,0.55); eb:stroke({{162,605},{178,610},{191,613}}, {pressure={0.35,0.2}}); eb:stroke({{216,620},{236,626},{250,631}}, {pressure={0.34,0.18}}); eb:stroke({{280,638},{299,644},{314,652}}, {pressure={0.36,0.2}}); eb:stroke({{997,595},{979,592},{964,592}}, {pressure={0.36,0.18}}); eb:stroke({{944,594},{928,596},{911,600}}, {pressure={0.34,0.18}}); eb:reload(pEdgeWet,0.56); eb:stroke({{888,604},{872,610},{858,616}}, {pressure={0.35,0.19}}); eb:stroke({{832,627},{814,637},{801,645}}, {pressure={0.34,0.18}}); eb:stroke({{791,649},{778,656},{767,664}}, {pressure={0.35,0.2}});

--@ chunk 17
pGlintCool=pile{{"lead white",0.9},{"pale smalt",1.1},{"smalt",0.65},{"yellow ochre",0.16},{"raw umber",0.16}}; cb=brush{kind="filbert",width=6,stiffness=0.5,point=0.12}; cb:load(pGlintCool,0.44); cb:stroke({{450,460},{464,458},{480,459}}, {pressure={0.34,0.18}}); cb:stroke({{516,478},{530,476},{544,478}}, {pressure={0.34,0.19}}); cb:stroke({{435,502},{450,501},{466,502}}, {pressure={0.34,0.18}}); cb:stroke({{557,526},{573,524},{588,526}}, {pressure={0.35,0.18}}); cb:reload(pGlintCool,0.43); cb:stroke({{484,542},{498,540},{515,542}}, {pressure={0.35,0.19}}); cb:stroke({{589,569},{607,567},{620,569}}, {pressure={0.34,0.18}}); cb:stroke({{600,594},{618,592},{632,593}}, {pressure={0.35,0.18}}); cb:stroke({{831,472},{847,470},{862,471}}, {pressure={0.35,0.18}}); cb:stroke({{800,508},{818,506},{834,508}}, {pressure={0.35,0.18}}); cb:reload(pGlintCool,0.42); cb:stroke({{849,543},{865,541},{879,543}}, {pressure={0.34,0.17}}); cb:stroke({{805,573},{825,571},{841,572}}, {pressure={0.35,0.18}})

--@ chunk 18
wait(3*24*60); print(drying(720,220), drying(518,550), drying(713,645))
