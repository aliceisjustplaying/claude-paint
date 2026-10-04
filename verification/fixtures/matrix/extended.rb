require_relative 'driver'
# Full defaults at two physical canvas widths: omitted versus explicit millimeter conversions.
[100,400].each{|size|['default','explicit'].each{|v|
 cli('close') if $opened;cli('open',"physical-#{size}-#{v}");$opened=true;mm=size/1000.0
 run("canvas{size=#{size},aspect=5,linen=12,ground={{pile={{\"lead white\",1}},um=40,apply=\"knife\"}}};h=pencil();m=rect(300,40,200,100)")
 run(v=='default' ? 'h:line({{100,20},{800,20}},{seed=1});h:sketch({{100,60},{800,60}},{seed=1});h:hatch(m,{seed=1});erase({{400,20},{450,20}})' : "h:line({{100,20},{800,20}},{seed=1,tremor=#{0.15/mm}});h:sketch({{100,60},{800,60}},{seed=1,wander=#{2/mm},tremor=#{0.15/mm}});h:hatch(m,{seed=1,spacing=#{[1.5/mm,2.5].max},length=#{12/mm}});erase({{400,20},{450,20}},{width=#{4/mm}})")
 shot("physical-#{size}-#{v}")}}
# Cloth stages and fresh-over-set film, identical fresh rag each run.
[0,960,43200].each{|wait|case_image("rag-stage-#{wait}",'work(m,{pile=p,hand="detail",coverage=2,clip=true,seed=1});'+"wait(#{wait});r=rag{width=80,seed=1};r:dip(1);print(drying(400,80));",'r:wipe({{330,80},{480,80}},{pressure=1,seed=1});print(r.load,r.soaked,wait(0))')}
fresh('layer-lift','work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});wait(43200);print(drying(400,80));');shot('layer-set');run('work(m,{pile=q,hand="detail",coverage=2,clip=true,seed=1});print(drying(400,80));');shot('layer-fresh');run('r=rag{width=80,seed=1};r:dip(1);r:wipe(m,{passes=20,refold=0.01,pressure=1,seed=1});print(r.load,r.soaked,r.fold,wait(0))');shot('layer-lifted')
# Independent cloth clean/loaded, repeatability, refold pattern, clamp/default options.
WET='work(m,{pile=p,hand="detail",coverage=2,clip=true,seed=1});r=rag{width=40,seed=1};r:dip(1);'
{'default'=>'','explicit'=>'pressure=0.5,passes=1','two'=>'passes=2','twenty'=>'passes=20','foldminus'=>'passes=3,refold=-1','foldzero'=>'passes=3,refold=0','foldone'=>'passes=3,refold=1','foldtwo'=>'passes=3,refold=2','seed2'=>'seed=2'}.each{|k,o|case_image('rag-variant-'+k,WET,"r:wipe(m,{#{o}});print(r.load,r.soaked,r.damp,r.fold)")}
['fresh','loaded','refold'].each{|k|case_image('rag-face-'+k,WET,(k=='fresh' ? '' : 'r:wipe({{330,60},{450,60}},{seed=1});')+(k=='refold' ? 'r:refold();r:dip(1);' : '')+'r:blot(450,110,{seed=1});print(r.load,r.soaked,r.damp,r.fold)')}
fresh('clock-ops','r=rag{width=20};r:dip(1);h=pencil();');run('print("before",r.damp,wait(0));h:line({{100,80},{800,80}});print("line",r.damp,wait(0));erase(everywhere());print("erase",r.damp,wait(0));fix();print("fix",r.damp,wait(0));h:sharpen();print("sharpen",r.damp,wait(0))')
# Repeat seeds and retained defaults after unrelated customized tools/operations.
case_image('line-repeat','h=pencil("HB");','h:line({{200,80},{500,120},{800,80}},{seed=2})')
case_image('defaults-retained','custom=brush{kind="round",width=4,stiffness=0.1};old=pencil("9B");r=rag{width=20,seed=8};h=pencil("HB");','h:line({{200,80},{500,120},{800,80}},{seed=2})')
# Palette with wide pigment spread, record real reload/remix through dampness.
fresh('ledger-wide','b=brush("round",4);r=rag{width=20};');run('names={"lead white","smalt","pale smalt","yellow ochre","red earth","vermilion","raw umber","bone black","cobalt blue","chrome yellow","Prussian blue","green earth","Rinmann\'s green","copper green"};for i,n in ipairs(names) do _G["p"..i]=pile{{n,1}} end;for i,n in ipairs(names) do _G["p"..(i+14)]=pile{{n,1},{"lead white",1}} end;r:dip(1);print("before",r.damp);b:load(p28);print("newest",r.damp);r:dip(1);b:load(p1);print("oldest",r.damp,type(p1))')
cli('close')
