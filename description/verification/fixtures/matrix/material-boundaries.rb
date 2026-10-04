require_relative 'driver'
[0,0.5,0.9].each{|medium|fresh("oil-#{medium}","p=pile{{\"red earth\",1},medium=#{medium}};work(rect(350,50,100,60),{pile=p,hand=\"detail\",coverage=1,clip=true,seed=1});");run('print(p.medium,drying(400,80),wait(0));last=drying(400,80);for i=1,2000 do wait(5);local stage=drying(400,80);if stage~=last then print(stage,wait(0));last=stage end;if stage=="dry" then break end end')}
# A setting layer is sampled explicitly before the rag contact.
fresh('rag-setting','m=rect(350,50,100,60);work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});wait(960);r=rag{width=40,seed=1};r:dip(1);print(drying(400,80));');run('r:wipe({{360,80},{440,80}},{pressure=1,seed=1});print(r.load,r.soaked,drying(400,80))')
# Empty session error and final effect refusal are separate from CLI finish which ages first.
cli('close');cli('open','empty');run('brush("round",4)');['work(rect(0,0,10,10),{pile=7})','wait(0)','drying(0,0)','rag()','everywhere()'].each{|s|run(s)};cli('close');$opened=false
fresh('effect-stage','work(m,{pile=p,hand="detail",coverage=2,clip=true,seed=1});print(drying(400,80));');run('varnish()');run('wait(43200);print(drying(400,80))');run('varnish()');cli('close')
