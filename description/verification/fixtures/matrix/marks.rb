require_relative "driver"
B='b=brush("flat",12);b:load(p,0.8);'
PATH='{{200,80},{500,120},{800,80}}'
case_image('blank','','print(wait(0))')
# All tool override fields through the public constructor.
{'length'=>40,'stiffness'=>0.1,'hair'=>0.2,'run'=>0.3,'lay'=>0.1,'pickup'=>0.1,'push'=>0.1,'splay'=>1.5,'ragged'=>0.8,'point'=>0.9,'bristles'=>15}.each{|k,v|case_image('tool-'+k,'',"b=brush{kind=\"round\",width=12,#{k}=#{v}};b:load(p);b:stroke(#{PATH});print(b,b:fullness())")}
case_image('tool-base','','b=brush{kind="round",width=12};b:load(p);b:stroke('+PATH+');print(b,b:fullness())')
{'default'=>'','explicit'=>'pressure={0.8,0.8},orient="across",ramps={0.08,0.15},shake=1','pressure'=>'pressure={0.1,0.9}','ramps'=>'ramps={0.05,0.4}','swell'=>'swell={1,1.3,0.8}','shake'=>'shake=0','along'=>'orient="along"','angle'=>'orient=0.8'}.each{|k,o|case_image('stroke-'+k,B,"b:stroke(#{PATH},{#{o}});print(b:fullness())")}
{'default'=>'','explicit'=>'pressure=0.6,drag={0,0},twist=0,angle=0','pressure'=>'pressure=0.2','drag'=>'drag={40,0}','twist'=>'twist=1','angle'=>'angle=0.8'}.each{|k,o|case_image('touch-'+k,B,"b:touch(400,80,{#{o}});print(b:fullness())")}
%w[stroke touch].each{|k|['','clip=m'].each_with_index{|o,i|case_image("clip-#{k}-#{i}",'b=brush("flat",100);b:load(p);m=rect(300,40,50,80);',k=='stroke' ? "b:stroke({{200,80},{450,80}},{#{o}})" : "b:touch(320,80,{#{o}})")}}
fresh('retained',B+'r=rag{width=20};alias=b;copy=brush(b)');run('print(b,b:fullness(),r.load,r.soaked,r.damp,r.fold);b:stroke({{100,50},{800,50}});print(b:fullness())');shot('retained-first');run('b:stroke({{100,100},{800,100}});print(b:fullness())');shot('retained-second');run('print(r.load,r.soaked,r.damp,r.fold);b:wipe(1);print(b:fullness(),r.load,r.soaked,r.damp,r.fold)');shot('retained-wipe');run('b:stroke({{100,150},{800,150}});print(b:fullness(),copy:fullness())');shot('retained-empty');run('b:load(p);print(b:fullness());b:wipe();print(b:fullness());b:load(p);print(b:fullness());b:reload(q);print(b:fullness())');run('b.width=20');run('b:load("image.png")');run('b:load(p,0.8,0.2)');run('b:load(pile{{"red earth",1},medium=0.2});print(b:fullness())')
['fresh','reload','wipe85'].each{|k|case_image('reload-'+k,'b=brush("round",12);', (k=='fresh' ? '' : 'b:load(p);')+(k=='reload' ? 'b:reload(q);' : k=='wipe85' ? 'b:wipe(0.85);b:load(q);' : 'b:load(q);')+'b:stroke({{100,80},{800,80}});print(b:fullness())')}
[false,true].each{|stripe|case_image('pickup-'+stripe.to_s,'b=brush("round",12);b:load(q);', (stripe ? 'work(rect(350,20,100,140),{pile=p,hand="detail",coverage=2,clip=true,seed=3});' : '')+'b:stroke({{200,80},{800,80}},{shake=0});b:touch(850,120);print(b:fullness())')}
fresh('points','b=brush{kind="round",width=12,point=0.9};b:load(p)');run('print(b:mark_width(0.1),b:mark_width(0.9),b:pressure_for(1));b:stroke({{200,80},{800,80}},{pressure={0.1,0.9},shake=0,ramps={0,0}})');shot('point-taper')
# Pencil one-factor controls; equal fresh point and canvas.
H='h=pencil("HB");'
{'default'=>'','explicit'=>'pressure=0.5','profile'=>'pressure={0.1,0.9,0}','smooth'=>'smooth=true','corners'=>'smooth=false','ruler'=>'ruler=true','tremor0'=>'tremor=0','tremor10'=>'tremor=10','seed2'=>'seed=2'}.each{|k,o|case_image('line-'+k,H,"print(h:line(#{PATH},{#{o}}));print(h.worn,h:width())")}
[1,3,12,0,99].each{|n|case_image('sketch-'+n.to_s,H,"print(h:sketch(#{PATH},{passes=#{n},pressure=0.3,wander=20,tremor=0,seed=1}))")}
{'default'=>'','explicit'=>'pressure=0.3,passes=3','wander0'=>'wander=0','tremor0'=>'tremor=0','tremor10'=>'tremor=10'}.each{|k,o|case_image('sketch-'+k,H,"print(h:sketch(#{PATH},{#{o}}))")}
{'default'=>'','explicit'=>'pressure=0.45','angle'=>'angle=0.5','spacing'=>'spacing=20','length'=>'length=20','pressure'=>'pressure=0.1','seed'=>'seed=2'}.each{|k,o|case_image('hatch-'+k,H,"print(h:hatch(m,{#{o}}));print(h.worn)")}
%w[9H HB 9B chalk].each{|g|case_image('lead-'+g,g=='chalk' ? 'h=chalk();' : "h=pencil(\"#{g}\");",'print(h:line({{100,80},{800,80}},{pressure=0.5,ruler=true}));print(h.worn,h:width())')}
# Explicit drawing/erasing/sealing order, comparison targets kept separate.
DRAW='h=pencil("HB");h:rule({100,80},{800,80},{pressure=0.8});'
[-1,0,0.9,1,2].each{|n|case_image('erase-'+n.to_s,DRAW,"erase(rect(300,20,200,130),{strength=#{n}});print(drawing_guide():area())")}
[4,20].each{|w|case_image('erase-path-'+w.to_s,DRAW,"erase({{300,80},{500,80}},{width=#{w}});print(drawing_guide():area())");case_image('erase-mask-'+w.to_s,DRAW,"erase(m,{width=#{w}});print(drawing_guide():area())")}
case_image('erase-default',DRAW,'erase(m);print(drawing_guide():area())')
case_image('erase-point',DRAW,'erase({{400,80}});print(drawing_guide():area())')
fresh('fix',DRAW);shot('fix-before');run('fix(rect(100,20,300,130));erase(everywhere());print(drawing_guide():at(200,80),drawing_guide():at(600,80))');shot('fix-after');run('h:rule({100,80},{800,80},{pressure=0.8});print(drawing_guide():area());erase(everywhere());print(drawing_guide():area())');shot('fix-new-erased')
case_image('wet-block',H,'work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});h:rule({100,80},{800,80});print(drawing_guide():at(200,80),drawing_guide():at(400,80),drying(400,80))')
# Rag bounds and cloth state. Wet patch always confirmed by query.
WET='work(rect(200,20,500,150),{pile=p,hand="detail",coverage=2,clip=true,seed=1});r=rag{width=40,seed=1};print(drying(400,80));'
[-1,0,0.5,1,2].each{|n|case_image('rag-pressure-'+n.to_s,WET,"r:wipe({{300,80},{600,80}},{pressure=#{n},seed=1});print(r.load,r.soaked,r.damp,wait(0))")}
{'path'=>'r:wipe({{300,80},{600,80}},{seed=1})','profile'=>'r:wipe({{300,80},{600,80}},{pressure={0.1,0.9,0},seed=1})','blot'=>'r:blot(400,80,{seed=1})','one'=>'r:wipe({{400,80}},{seed=1})','short'=>'r:wipe({{400,80},{400.0001,80}},{seed=1})','mask'=>'r:wipe(rect(300,40,80,80),{angle=0,passes=2,seed=1})','angle'=>'r:wipe(rect(300,40,80,80),{angle=1,passes=2,seed=1})','refold'=>'r:dip(1);r:wipe(rect(300,40,280,80),{passes=3,refold=0.01,seed=1})'}.each{|k,c|case_image('rag-'+k,WET,c+';print(r.load,r.soaked,r.damp,r.fold,wait(0))')}
fresh('rag-state',WET+'alias=r;fresh=rag{width=40,seed=1};');run('r:dip(1);r:blot(400,80);print(r.load,alias.load,fresh.load,r.soaked,r.damp,r.fold);for i=1,6 do r:refold();print(r.load,r.soaked,r.damp,r.fold) end');run('r:dip(0.5);print(r.load,r.soaked,r.damp);r:dip(0);print(r.load,r.soaked,r.damp);wait(3);print(r.damp);wait(30);print(r.damp)');run('r:wipe(rect(0,0,0,0));r:blot(-100,-100);print(r.load,r.soaked)')
['r:wipe({})','r:wipe({{0/0,1}})','r:blot(0/0,1)','r:wipe(m,{wat=1})','r:wipe({{1,1}},{passes=1})','r:wipe({{1,1}},{refold=0.5})','r:wipe(m,{pressure={0.1,0.9}})','r:wipe(m,{passes=1.5})','r:dip(0/0)','r:wipe(m,{angle=0/0})','r:wipe(m,{refold=0/0})'].each{|c|run(c)}
cli('close');$opened=false
