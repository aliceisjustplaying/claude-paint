require_relative 'driver'
fresh('native','m=rect(350,50,100,60);work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});last=drying(400,80);for i=1,200 do wait(5);if drying(400,80)~=last then print("transition",i*5,drying(400,80),wait(0));break end end');cli('close');$opened=false
fresh('cross','m=rect(350,50,100,60);work(m,{pile=p,hand="detail",coverage=3,clip=true,seed=1});wait(500);r=rag{width=2,seed=1};print("before",drying(400,80),wait(0));');run('r:wipe(m,{passes=20,refold=0.01});print("after",drying(400,80),wait(0),r.fold,r.load)');shot('cross');cli('close')
