eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
n='extra';ok(n,'open',n);base=SETUP.split(';p=')[0];ok(n,'do',base);rec('local-palette',{empty:cli(n,'look','--palette'),local:cli(n,'do','local p=pile{{"red earth",1}}'),after_local:cli(n,'look','--palette'),global:cli(n,'do','p=pile{{"red earth",1}}'),after_global:look(n,'global-palette','--palette')})
ok(n,'do','b=brush("round",12);b:load(p);b:touch(20,20);b:touch(120,20);b:touch(20,120);m=rect(300,20,200,60);work(m,{pile=p,hand="detail",coverage=.3,angle=0,clip=true});work(rect(600,20,60,150),{pile=p,hand="detail",coverage=.3,angle=math.pi/2,clip=true})');rec('coordinates',{look:look(n,'coordinates','--grid','20')})
f=ROOT+'/capture.lua';File.write(f,'b:touch(800,150);'+SLOW+'filecaptured=true');c=cpu(n);t=Thread.new{cli(n,'do','-f',f,'--look')};sleep 0.2;active=t.alive?;advanced=cpu(n)!=c;File.write(f,'filecaptured=false');rec('file-post-look',{pending:active,cpu_advanced:advanced,reply:t.value,check:cli(n,'do','assert(filecaptured)')})
f=ROOT+'/replay.lua';File.write(f,base);c=cpu(n);a=req(n,'do',SLOW+'liveonly=true');obs=observe(n,a,c);replay=cli(n,'run',f,'--width','64','--out',ROOT+'/replayed.png');reply=a.read;a.close;rec('live-versus-replay',{observation:obs,replay:replay,live:reply,check:cli(n,'do','assert(liveonly)')})
ok(n,'close')
%w[wet dry].each do |n|
 ok(n,'open',n);ok(n,'do',base+';p=pile{{"red earth",1}};q=pile{{"cobalt blue",1}};b=brush("flat",30);b:load(p);b:stroke({{200,80},{800,80}})');before=look(n,n+'-before');ok(n,'do','wait(10080)') if n=='dry';reply=cli(n,'do','b:load(q);b:stroke({{500,20},{500,150}})');after=look(n,n+'-after');rec('overlap-'+n,{before:before,reply:reply,after:after});ok(n,'close')
end
unused=ROOT+'/never-opened';FileUtils.mkdir_p(unused);rec('not-invoked',{before:Dir.children(unused),prepared_arguments:['open','never'],after:Dir.children(unused),invocation_made:false})
