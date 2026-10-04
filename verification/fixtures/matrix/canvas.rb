eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
base='canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}},seed=1}'
cases={'default-seed'=>base.sub(',seed=1',''),'seed1'=>base,'seed2'=>base.sub('seed=1','seed=2')}
[50,5000,49,5001].each{|v|cases["size#{v}"]=base.sub('size=100',"size=#{v}")}
[1,2,0.2,0.19,5.01].each{|v|cases["aspect#{v}".tr('.','_')]=base.sub('aspect=5',"aspect=#{v}")}
['4','60','{4,60}','{12,16}','3','61'].each_with_index{|v,i|cases["linen#{i}"]=base.sub('linen=12',"linen=#{v}")}
%w[roller brush].each{|v|cases[v]=base.sub('knife',v)}
[5,400,4,401].each{|v|cases["ground#{v}"]=base.sub('um=40',"um=#{v}")}
cases['coatorder1']=base.sub('ground={{pile={{"lead white",1}},um=40,apply="knife"}}','ground={{pile={{"lead white",1}},um=40,apply="knife"},{pile={{"red earth",1}},um=40,apply="brush"}}')
cases['coatorder2']=cases['coatorder1'].sub('lead white','TEMP').sub('red earth','lead white').sub('TEMP','red earth')
cases.each do |n,src|
 ok(n,'open',n);r=cli(n,'do',src+';print(W,H);p=pile{{"red earth",1}};b=brush("round",4);b:load(p);b:touch(100,80)')
 data={source:src,reply:r,status:cli(n,'status')};data[:look]=look(n,n,'--size','400') if r[:exit]==0;rec(n,data);ok(n,'close')
end
n='material';ok(n,'open',n);ok(n,'do',base)
[0,0.5,0.95,-0.01,0.96].each_with_index{|v,i|rec("medium#{v}",{reply:cli(n,'do',"p#{i}=pile{{'red earth',1},medium=#{v}};print(p#{i});b=brush('round',8);b:load(p#{i});b:touch(#{100+i*150},80)")})}
rec('medium-palette',{look:look(n,'medium-palette','--palette'),canvas:look(n,'medium-strokes')})
rec('duplicate-pile',{reply:cli(n,'do','pA=pile{{"red earth",1},{"red earth",2}};pB=pile{{"red earth",3}};print(pA,pB)'),look:look(n,'duplicate-palette','--palette')})
p=hash(shot(n,'replacement-before'));rec('replace',{reply:cli(n,'do',base.sub('seed=1','seed=123')),reseed:cli(n,'do','math.randomseed(123);print(math.random());print(type(resize),type(clear),type(undo),type(layer))'),png_equal:p==hash(shot(n,'replacement-after'))})
rec('container',{reply:cli(n,'do','p=pile{{"red earth",1}};b=brush("round",4);m=rect(100,40,80,80);o=outline{pts={{100,40},{200,40},{150,120}}};r=rag{width=20};f=form{{body.ellipsoid({400,100,0},{20,20,10})}}'),globals:cli(n,'globals'),reuse:cli(n,'do','b:load(p);b:touch(100,80);r:blot(100,80);print(m:area(),o:area(),f:parts_mask{1}:area())')})
ok(n,'close')
n='precanvas';ok(n,'open',n);%w[pile rect rag brush].zip(['p=pile{{"red earth",1}}','m=rect(10,10,10,10)','r=rag{width=20}','b=brush("round",4);b:touch(10,10)']).each{|id,src|rec('pre-'+id,{reply:cli(n,'do',src)})};rec('unknown-image',{reply:cli(n,'do',base.sub('size=100','image="a.png",size=100'))});ok(n,'close')
