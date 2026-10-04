require_relative 'driver'
# Exact real harness Lua through all three CLI input forms.
code='canvas{size=50,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; b=brush("round",4);b:load(p,0.8);b:touch(100,60);m=rect(50,40,200,100);work(m,{pile=p,hand="detail",coverage=.1}); h=pencil("HB");h:line({{20,30},{200,30}});r=rag{width=20};r:dip(.5);r:blot(100,60); print(wait(0));f=form{{body.ellipsoid({400,100,0},{20,20,10})}};print(f:parts_mask{1}:area())'
%w[inline file stdin].each{|mode|cli('open','channel-'+mode);if mode=='inline';run(code);elsif mode=='file';file=File.join(ROOT,'source.lua');File.write(file,code);cli('do','-f',file);else;o,e,s=Open3.capture3(BIN,'do','-',stdin_data:code);$results<<{mode:mode,args:['do','-',code],out:o,err:e,exit:s.exitstatus};end;shot('channel-'+mode);cli('close')}
%w[found soft lost].each{|edge|case_image('edge-'+edge,'',"work(m,{pile=p,edge=\"#{edge}\",seed=7})")}
# Passage preferences: override is empty-mask so it can consume seeds/time but no image; default passage explicit seed.
['default','override'].each{|k|case_image('preference-'+k,'', (k=='override' ? 'work(rect(0,0,0,0),{pile=p,hand="glaze",pressure={0.2,0.2},seed=7});' : '')+'work(m,{pile=p,seed=7})')}
# Matched material, oil and layer thickness at the same simulated elapsed time.
[[0,1],[0.5,1],[0,3]].each{|medium,cov|fresh("material-#{medium}-#{cov}","p=pile{{\"red earth\",1},medium=#{medium}};work(rect(350,50,100,60),{pile=p,hand=\"detail\",coverage=#{cov},clip=true,seed=1});");run('print(wait(0),drying(400,80));for _,n in ipairs{120,120,240,480,960,1920,3840} do print(wait(n),drying(400,80)) end')}
# Exact confirmed-stage operation matrix. Clock changes from mixing/construction are captured.
[0,960,1920,43200].each{|minutes|fresh("workability-#{minutes}",'m=rect(350,50,100,60);work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});'+"wait(#{minutes});print(drying(400,80),wait(0));");shot("stage-#{minutes}-before");run('blend(m);print(drying(400,80));b=brush("round",8);b:load(q);b:stroke({{300,80},{500,80}},{shake=0});');shot("stage-#{minutes}-after")}
fresh('native-aging','m=rect(350,50,100,60);work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});wait(850);r=rag{width=2,seed=1};print(drying(400,80),wait(0));');run('r:wipe(m,{passes=20,refold=0.01});print(drying(400,80),wait(0),r.fold,r.load)');shot('native-aging')
fresh('clock-with-work');run('r=rag{width=20};r:dip(1);print(wait(0),r.damp);local sum=0;for i=1,300000000 do sum=sum+i end;print(wait(0),r.damp);assert(sum>0)')
cli('close')
